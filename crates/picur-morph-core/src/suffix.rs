//! Archiphonemes and the suffix inventory.
//!
//! A suffix is stored once, as a template over archiphonemes, and its surface allomorphs are
//! computed. `-ban` and `-ben` never appear in this source: the inessive is `-bAn`, and harmony
//! decides. That is what keeps a closed inventory closed instead of growing an allomorph list.
//!
//! See `docs/design/01-core-model.md` and `docs/grammar/02-phonology.md` (`PH-HARM-002`).

use compact_str::CompactString;

use picur_morph_ortho::Harmony;

use crate::features::Case;

/// A realised suffix, kept on the stack.
///
/// Every suffix in the inventory fits: realising one is done once per attach, and the attach is
/// the checker's innermost loop.
pub type Tail = CompactString;

/// A harmonic vowel slot, realised from the stem's harmony class.
///
/// `i`, `í` and `é` in an invariant suffix are **not** archiphonemes: `-ig`, `-ért` and `-ként` are
/// literal and never alternate.
///
/// Rules: `PH-HARM-002`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Arch {
    /// `a` / `e` / `e`.
    A,
    /// `á` / `é` / `é`.
    AAcute,
    /// `o` / `e` / `ö`. The only archiphoneme that distinguishes front rounded.
    O,
    /// `ó` / `ő` / `ő`.
    OAcute,
    /// `u` / `ü` / `ü`.
    U,
    /// `ú` / `ű` / `ű`.
    UAcute,
}

impl Arch {
    /// The surface vowel for this harmony class.
    ///
    /// Rules: `PH-HARM-002`, `PH-HARM-005`
    #[must_use]
    #[expect(
        clippy::match_same_arms,
        reason = "A -> e for any front class and O -> e for plain front are different rules \
                  that happen to share an output; merging them would hide PH-HARM-005"
    )]
    pub const fn realise(self, h: Harmony) -> &'static str {
        match (self, h) {
            (Self::A, Harmony::Back) => "a",
            (Self::A, _) => "e",
            (Self::AAcute, Harmony::Back) => "á",
            (Self::AAcute, _) => "é",
            (Self::O, Harmony::Back) => "o",
            (Self::O, Harmony::Front) => "e",
            (Self::O, Harmony::FrontRounded) => "ö",
            (Self::OAcute, Harmony::Back) => "ó",
            (Self::OAcute, _) => "ő",
            (Self::U, Harmony::Back) => "u",
            (Self::U, _) => "ü",
            (Self::UAcute, Harmony::Back) => "ú",
            (Self::UAcute, _) => "ű",
        }
    }

    /// The harmony class in force after this vowel has been emitted.
    ///
    /// Backness is fixed by the stem, but rounding is read off the **last** vowel, and a suffix's
    /// own earlier vowel becomes that last vowel for the slots behind it. `-AtOk` on `ütött` gives
    /// `ütöttetek`, not `*ütöttetök`: the `e` of the first slot unrounds the second.
    ///
    /// Rules: `PH-HARM-005`
    #[must_use]
    pub fn rounding_after(self, h: Harmony) -> Harmony {
        // Read off the vowel this actually realised as, not off the archiphoneme. `O` is rounded
        // only where the harmony is: under `Front` it comes out `e`, so the slot behind it is
        // unrounded too (`mennetek`).
        rounding_after_text(self.realise(h), h)
    }
}

/// The harmony class in force after a literal segment.
///
/// A literal vowel counts for rounding exactly as a realised archiphoneme does: the `i` of the
/// front definite `-itOk` is unrounded, so `főz` gives `főzitek` and not `*főzitök`.
///
/// Rules: `PH-HARM-005`
fn rounding_after_text(text: &str, h: Harmony) -> Harmony {
    if h == Harmony::Back {
        return Harmony::Back;
    }
    match text.chars().rfind(|c| "aáeéiíoóöőuúüű".contains(*c)) {
        Some(c) if "öőüű".contains(c) => Harmony::FrontRounded,
        Some(_) => Harmony::Front,
        None => h,
    }
}

/// One segment of a suffix template.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seg {
    /// Literal text, never alternating.
    Lit(&'static str),
    /// A harmonic vowel.
    Var(Arch),
    /// The `v` of `-vAl` and `-vÁ`, which assimilates to a preceding consonant.
    ///
    /// Rules: `NOM-CASE-003`
    AssimV,
    /// The `j` of the definite suffixes, which assimilates to a preceding sibilant.
    ///
    /// Unlike [`Seg::AssimV`] it assimilates only to `s`, `sz`, `z` and `dz`; after anything else,
    /// `j` is written as it stands (`látja`, `tanítja`).
    ///
    /// Rules: `VRB-PRES-003`, `VRB-PRES-004`
    AssimJ,
}

/// How a suffix attaches.
///
/// Rules: `NOM-MODEL-004`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuffixClass {
    /// Takes an optional linking vowel after a consonant: `-(O)t`, `-(O)n`, `-(O)nként`.
    One,
    /// Attaches directly and still lengthens a final `a`/`e`: `-nAk`, `-bAn`, `-hOz`.
    Two,
    /// Invariant, and does not lengthen: `-ként`, `-kor`, `-képp`.
    Three,
}

/// A suffix: its template, how it attaches, and whether it lengthens a final `a`/`e`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each flag is an independent property `NOM-CASE-001` states per suffix, and the \
              constructors below are where that table is read; a packed flag set would hide which \
              of them a given suffix sets"
)]
pub struct Suffix {
    /// Segments, in order.
    pub template: &'static [Seg],
    /// Attachment class.
    pub class: SuffixClass,
    /// Whether a stem-final `a`/`e` lengthens to `á`/`é` before it.
    pub lengthens: bool,
    /// Whether a lowering stem makes its linking vowel low.
    ///
    /// The superessive is class [`SuffixClass::One`] but **not** lowerable, which is the whole
    /// reason `ház` gives `házak` and `házat` but `házon`. One boolean, not a per-stem exception.
    ///
    /// Rules: `NOM-LOW-001`
    pub lowerable: bool,
    /// Whether the bare-coda class of `NOM-ACC-003` applies.
    ///
    /// Only the accusative has it: `dal` gives `dalt` because `lt` is a licit coda, but the plural
    /// of the same stem is `dalok`, with the link, because `lk` is not. Every other class I suffix
    /// takes the linking vowel after any consonant.
    ///
    /// Rules: `NOM-ACC-003`
    pub bare_coda: bool,
    /// Whether a shortening stem keeps its long vowel before this suffix.
    ///
    /// The superessive is the one class I suffix that does: `út` gives `utat` and `utak` but
    /// `úton`, and `kéz` gives `kezet` and `kezek` but `kézen`. Corpus, free against bound:
    /// `úton` 1,275,847 : `uton` 17,480; `kézen` 39,345 : `kezen` 514; `nyáron` 605,018 :
    /// `nyaron` 5,328; `téren` 516,508 : `teren` 7,745 `[WC2]`. Only shortening is affected; the
    /// epenthetic, v- and oa-stems all take the bound alternant here (`bokron`, `lovon`, `tavon`).
    ///
    /// Rules: `NOM-STEM-006`
    pub free_stem: bool,
    /// Whether a vowel-final gradable stem still takes a low linking vowel before this suffix.
    ///
    /// Only the plural does. `NOM-PL-003` is stated for the plural alone, and the comparative goes
    /// the other way on the same stems: `hosszú` gives `hosszúak` but `hosszabb`, and `egyszerű`
    /// gives `egyszerűek` but `egyszerűbb`. Gating on `lowerable` instead caught the comparative
    /// and produced `egyszerűebb` and `gyakoriabb`.
    ///
    /// Rules: `NOM-PL-003`
    pub high_vowel_link: bool,
}

impl Suffix {
    /// Realises the template against a harmony class.
    ///
    /// [`Seg::AssimV`] is emitted as `v`; the caller applies assimilation at the seam, because it
    /// needs the preceding consonant to do so.
    ///
    /// Rules: `PH-HARM-002`
    #[must_use]
    pub fn realise(&self, h: Harmony) -> String {
        self.realise_with_link(h, self.linking_slot()).into_string()
    }

    /// The archiphoneme of the optional linking vowel, for a class I suffix.
    ///
    /// In a class I template the **leading** `Var` is that optional vowel: `-(O)t` is
    /// `[Var(O), Lit("t")]`. It is part of the template rather than a separate field, so realising
    /// has to know whether to keep it.
    ///
    /// Rules: `NOM-MODEL-004`
    #[must_use]
    pub fn linking_slot(&self) -> Option<Arch> {
        match (self.class, self.template.first()) {
            (SuffixClass::One, Some(Seg::Var(a))) => Some(*a),
            _ => None,
        }
    }

    /// Realises the template, substituting `link` for the optional linking vowel.
    ///
    /// `None` drops the slot entirely (`dal` + `-(O)t` -> `dalt`), and `Some(a)` realises `a` there,
    /// which is how a lowering stem gets `a` where the template says `O` (`ház` -> `házat`).
    ///
    /// Rules: `PH-HARM-002`, `NOM-LOW-001`
    #[must_use]
    pub fn realise_with_link(&self, h: Harmony, link: Option<Arch>) -> Tail {
        let mut out = Tail::default();
        let mut h = h;
        let mut segs = self.template.iter();
        if self.linking_slot().is_some() {
            segs.next();
            if let Some(a) = link {
                out.push_str(a.realise(h));
                h = a.rounding_after(h);
            }
        }
        for seg in segs {
            match *seg {
                Seg::Lit(s) => {
                    out.push_str(s);
                    h = rounding_after_text(s, h);
                }
                Seg::Var(a) => {
                    out.push_str(a.realise(h));
                    h = a.rounding_after(h);
                }
                Seg::AssimV => out.push('v'),
                Seg::AssimJ => out.push('j'),
            }
        }
        out
    }

    /// Whether the template begins with a vowel, which decides the stem alternant.
    #[must_use]
    pub fn starts_with_vowel(&self) -> bool {
        matches!(self.template.first(), Some(Seg::Var(_)))
    }
}

/// Builds a class II suffix that lengthens.
/// The locative after a consonant, `-Ott`: `Győrött`, `Pécsett` (`NOM-CASE-016`).
///
/// [`case_suffix`] carries the bare `-t` of the vowel-final form; which one a stem takes is
/// decided where the stem is known.
#[must_use]
pub const fn locative_linked() -> Suffix {
    one(&[Seg::Var(Arch::O), Seg::Lit("tt")], false)
}

const fn two(template: &'static [Seg]) -> Suffix {
    Suffix {
        template,
        class: SuffixClass::Two,
        lengthens: true,
        lowerable: false,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: false,
    }
}

/// Builds an invariant class III suffix.
/// `volna`, the past conditional's separate word (`VRB-COND-004`), as the ending it leaves a form
/// with: none, since it is written apart.
pub const VOLNA: Suffix = three(&[Seg::Lit("volna")]);

const fn three(template: &'static [Seg]) -> Suffix {
    Suffix {
        template,
        class: SuffixClass::Three,
        lengthens: false,
        lowerable: false,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: false,
    }
}

/// Builds a class I suffix with an optional linking vowel.
const fn one(template: &'static [Seg], lowerable: bool) -> Suffix {
    Suffix {
        template,
        class: SuffixClass::One,
        lengthens: true,
        lowerable,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: false,
    }
}

/// Builds the superessive, the class I suffix a shortening stem keeps its long vowel before.
const fn superessive(template: &'static [Seg]) -> Suffix {
    Suffix {
        template,
        class: SuffixClass::One,
        lengthens: true,
        lowerable: false,
        bare_coda: false,
        free_stem: true,
        high_vowel_link: false,
    }
}

/// The accusative, the one suffix with a bare-coda class.
const fn accusative(template: &'static [Seg]) -> Suffix {
    Suffix {
        template,
        class: SuffixClass::One,
        lengthens: true,
        lowerable: true,
        bare_coda: true,
        free_stem: false,
        high_vowel_link: false,
    }
}

/// The invariant `-en` of `-képpen`.
///
/// `NOM-CASE-020` states `-képp` ~ `-képpen` as free variants of one invariant suffix, so the
/// superessive written after the modalis does not harmonise with the stem: `példaképpen`,
/// `óraképpen`, `mindenképpen`.
///
/// Rules: `NOM-CASE-020`
#[must_use]
pub fn modalis_superessive() -> Suffix {
    three(&[Seg::Lit("en")])
}

/// The case suffix for a case, or `None` for the zero-marked nominative.
///
/// This is the inventory of `NOM-CASE-001`: a closed list the language actually has, not an
/// override table. Every row states its class and lengthening behaviour there.
///
/// Rules: `NOM-CASE-001`
#[must_use]
pub fn case_suffix(case: Case) -> Option<Suffix> {
    use Arch::{A, AAcute, O, OAcute, U};
    use Seg::{AssimV, Lit, Var};
    Some(match case {
        Case::Nom => return None,
        Case::Acc => accusative(&[Var(O), Lit("t")]),
        Case::Dat => two(&[Lit("n"), Var(A), Lit("k")]),
        Case::Ins => two(&[AssimV, Var(A), Lit("l")]),
        Case::Cau => two(&[Lit("ért")]),
        Case::Tra => two(&[AssimV, Var(AAcute)]),
        Case::Ter => two(&[Lit("ig")]),
        Case::Ess => two(&[Var(U), Lit("l")]),
        Case::Ine => two(&[Lit("b"), Var(A), Lit("n")]),
        Case::Ill => two(&[Lit("b"), Var(A)]),
        Case::Ela => two(&[Lit("b"), Var(OAcute), Lit("l")]),
        Case::Sup => superessive(&[Var(O), Lit("n")]),
        Case::Sbl => two(&[Lit("r"), Var(A)]),
        Case::Del => two(&[Lit("r"), Var(OAcute), Lit("l")]),
        Case::Ade => two(&[Lit("n"), Var(AAcute), Lit("l")]),
        Case::All => two(&[Lit("h"), Var(O), Lit("z")]),
        Case::Abl => two(&[Lit("t"), Var(OAcute), Lit("l")]),
        Case::Tem => three(&[Lit("kor")]),
        Case::Loc => three(&[Lit("t")]),
        Case::Formalis => three(&[Lit("ként")]),
        Case::Distributivus => one(&[Var(O), Lit("nként")], true),
        Case::Sociativus => one(&[Var(O), Lit("st"), Var(U), Lit("l")], true),
        Case::Modalis => three(&[Lit("képp")]),
        Case::DistributivusTemporalis => one(&[Var(O), Lit("nt"), Var(A)], true),
    })
}

#[cfg(test)]
mod tests {
    use picur_morph_ortho::Harmony;

    use super::{Arch, Seg, SuffixClass, case_suffix};
    use crate::features::Case;

    fn realise(c: Case, h: Harmony) -> String {
        case_suffix(c).expect("has a suffix").realise(h)
    }

    #[test]
    fn nominative_is_zero() {
        assert!(case_suffix(Case::Nom).is_none());
    }

    #[test]
    fn inessive_allomorphs_are_computed() {
        // NOM-CASE-010: -ban / -ben, never stored as two strings.
        assert_eq!(realise(Case::Ine, Harmony::Back), "ban");
        assert_eq!(realise(Case::Ine, Harmony::Front), "ben");
        assert_eq!(realise(Case::Ine, Harmony::FrontRounded), "ben");
    }

    #[test]
    fn allative_is_the_three_way_case() {
        // NOM-CASE-013: -hoz / -hez / -hoz; O is the only archiphoneme with a rounded form.
        assert_eq!(realise(Case::All, Harmony::Back), "hoz");
        assert_eq!(realise(Case::All, Harmony::Front), "hez");
        assert_eq!(realise(Case::All, Harmony::FrontRounded), "höz");
    }

    #[test]
    fn elative_uses_long_o() {
        // NOM-CASE-010: -bol / -bol.
        assert_eq!(realise(Case::Ela, Harmony::Back), "ból");
        assert_eq!(realise(Case::Ela, Harmony::Front), "ből");
    }

    #[test]
    fn adessive_uses_long_a() {
        assert_eq!(realise(Case::Ade, Harmony::Back), "nál");
        assert_eq!(realise(Case::Ade, Harmony::Front), "nél");
    }

    #[test]
    fn essive_uses_u() {
        // NOM-CASE-009: -ul / -ul.
        assert_eq!(realise(Case::Ess, Harmony::Back), "ul");
        assert_eq!(realise(Case::Ess, Harmony::Front), "ül");
    }

    #[test]
    fn invariant_suffixes_do_not_alternate() {
        // -ig, -ert, -kent, -kor are literal: i, i and e are not archiphonemes.
        for h in [Harmony::Back, Harmony::Front, Harmony::FrontRounded] {
            assert_eq!(realise(Case::Ter, h), "ig");
            assert_eq!(realise(Case::Cau, h), "ért");
            assert_eq!(realise(Case::Formalis, h), "ként");
            assert_eq!(realise(Case::Tem, h), "kor");
        }
    }

    #[test]
    fn superessive_is_class_one_but_not_lowerable() {
        // NOM-LOW-001: why haz gives hazak, hazat, but hazon.
        let sup = case_suffix(Case::Sup).expect("has a suffix");
        assert_eq!(sup.class, SuffixClass::One);
        assert!(!sup.lowerable, "superessive must never take a low link");

        let acc = case_suffix(Case::Acc).expect("has a suffix");
        assert_eq!(acc.class, SuffixClass::One);
        assert!(acc.lowerable);
    }

    #[test]
    fn instrumental_carries_the_assimilating_v() {
        let ins = case_suffix(Case::Ins).expect("has a suffix");
        assert_eq!(ins.template.first(), Some(&Seg::AssimV));
        // Realised bare it is `val`/`vel`; the seam applies assimilation.
        assert_eq!(ins.realise(Harmony::Back), "val");
        assert_eq!(ins.realise(Harmony::Front), "vel");
    }

    #[test]
    fn class_three_never_lengthens() {
        for c in [Case::Formalis, Case::Tem, Case::Modalis, Case::Loc] {
            let s = case_suffix(c).expect("has a suffix");
            assert_eq!(s.class, SuffixClass::Three);
            assert!(!s.lengthens, "{c:?} must not lengthen a final a/e");
        }
    }

    #[test]
    fn vowel_initial_suffixes_are_flagged() {
        assert!(case_suffix(Case::Acc).expect("acc").starts_with_vowel());
        assert!(!case_suffix(Case::Dat).expect("dat").starts_with_vowel());
    }

    #[test]
    fn archiphoneme_table_matches_ph_harm_002() {
        assert_eq!(Arch::A.realise(Harmony::Back), "a");
        assert_eq!(Arch::A.realise(Harmony::FrontRounded), "e");
        assert_eq!(Arch::O.realise(Harmony::FrontRounded), "ö");
        assert_eq!(Arch::OAcute.realise(Harmony::Front), "ő");
        assert_eq!(Arch::U.realise(Harmony::Front), "ü");
        assert_eq!(Arch::UAcute.realise(Harmony::Back), "ú");
    }
}
