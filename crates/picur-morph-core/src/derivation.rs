//! Derivation.
//!
//! `DER-MODEL-002` stores every derivational suffix as one record whose fields decide attachment
//! and surface form, and `DER-MODEL-016` calls that table normative data. This module is that
//! record type plus the entries the rulebook licenses for free generation.
//!
//! The generation boundary is `DER-MODEL-012` and is deliberate rather than a coverage gap: only
//! [`Prod::P`] suffixes may derive new words. `R` and `L` suffixes describe words that already
//! exist, so generating with them would invent vocabulary, which is exactly what a source of truth
//! must not do. They may still be applied where the result is a listed word, which is how
//! `titkár` is read as `titok` plus `-Ár` without `asztalár` ever being produced.
//!
//! See `docs/grammar/06-derivation-compounding.md`.

use picur_morph_lexicon::{Entry, Pos, Role, Trait, TraitSet, lexicon, lists, pairs, words};
use picur_morph_ortho::case::{lower_initial, starts_upper};
use picur_morph_ortho::{Harmony, Letter, letters};

use crate::alternation::alternant;
use crate::generate::{GenError, attach};
use crate::suffix::{Arch, Seg, Suffix, SuffixClass};

/// How a derivational suffix takes a linking vowel.
///
/// Rules: `DER-MODEL-004`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Link {
    /// Consonant-initial, never takes a vowel: `-sÁg`, `-i`, `-nyi`.
    None,
    /// Plural-type: the linking vowel is the one the same stem takes in the plural, so a lowering
    /// stem gets the low vowel. `-(O)s`, `-(O)z`, `-(O)l`.
    Plural,
    /// Fixed low `a`/`e` whatever the stem does: `-(A)lOm`, `-(A)dék`.
    Low,
    /// Fixed mid, with the vowel set named by the suffix: `-(O)gAt`, `-(O)mÁny`.
    Mid,
    /// The vowel belongs to the suffix and appears after every consonant: `-Ás`, `-Ó`, `-ít`.
    Own,
}

/// Whether a suffix may derive new words.
///
/// Rules: `DER-MODEL-012`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prod {
    /// Productive: any base meeting the input conditions.
    P,
    /// Productive only inside a stated semantic domain.
    PDom,
    /// Unproductive over a closed base class; the rule describes existing words.
    R,
    /// Unproductive with no rule; the derivatives are lexicon entries.
    L,
}

/// One derivational suffix, as the record of `DER-MODEL-002`.
#[derive(Clone, Copy, Debug)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "one field per column of `DER-MODEL-016`, which the rulebook calls normative data; \
              grouping them would stop the record reading as the table it mirrors"
)]
pub struct Der {
    /// Rule ID of the suffix.
    pub id: &'static str,
    /// Archiphoneme template.
    pub template: &'static [Seg],
    /// Base categories the suffix accepts.
    pub input: &'static [Pos],
    /// Category of the derivative.
    pub output: Pos,
    /// Linking vowel type.
    pub link: Link,
    /// Whether a stem-final `a`/`e` lengthens before it.
    pub lvl: bool,
    /// Whether the derived stem is itself lowering.
    pub out_lowering: bool,
    /// Generation policy.
    pub prod: Prod,
    /// Whether a `j` appears between a vowel-final stem and the suffix.
    ///
    /// The `(j)` of `-(j)Ú`: `láb` gives `lábú` but `alma` gives `almájú`.
    pub j_after_vowel: bool,
    /// A further condition on the stem that the input categories cannot express.
    pub stem_filter: Option<fn(&str) -> bool>,
    /// A condition on the base lexeme rather than on its spelling.
    ///
    /// This is how a `P-dom` suffix states a domain the lexicon carries as a trait, which
    /// `DER-MODEL-012` licenses: generation is free inside the domain and refused outside it.
    pub entry_filter: Option<fn(&Entry) -> bool>,
    /// Which stem the suffix attaches to, from `DER-MODEL-006`'s stem column.
    pub stem: Stem,
    /// Whether the suffix deletes a stem-final vowel, `DER-MODEL-007`'s `trunc` column.
    ///
    /// Only `-ít` has it among the productive suffixes: `barna` gives `barnít`, `fekete`
    /// `feketít` and `lassú` `lassít`.
    pub truncates_vowel: bool,
}

/// The stem a derivational suffix attaches to, as `DER-MODEL-006` classes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stem {
    /// The citation form: `-sÁg`, `-kA`, `-nyi`, `-lAg`.
    Free,
    /// Every alternation the stem has, which is what a vowel meeting the stem selects anyway.
    Bound,
    /// The epenthetic and v-stem alternants, but not the standalone shortening.
    ///
    /// The rule's `bound(ep)`, which only `-i` takes. Measured: `irodalmi` 495060 and `lelki`
    /// 752791 take the alternant, as does the v-stem in `havi` 813669 and `tavi` 16188, while
    /// `kézi` beats `kezi` 212594 to 662, so the shortening of `kéz` is not applied here.
    BoundEp,
    /// Shortening and the v-stem, with the epenthetic vowel kept: the rule's `bound(-ep)`, which
    /// is why `bátor` gives `bátorít` and not `bátrít`.
    BoundNoEp,
}

impl Der {
    /// The attachment record this suffix uses, in the shared [`Suffix`] form.
    ///
    /// The mapping is the whole point of `DER-MODEL-004`: a derivational suffix attaches by the
    /// same machinery as an inflectional one, so `-(O)s` behaves like the plural and needs no
    /// separate code path.
    ///
    /// Rules: `DER-MODEL-004`, `DER-MODEL-005`
    #[must_use]
    pub const fn suffix(&self) -> Suffix {
        let (class, lowerable) = match self.link {
            // A leading `Var` is only read as a linking slot in class I, so a suffix whose vowel is
            // its own is class II and emits it unconditionally.
            Link::None | Link::Own => (SuffixClass::Two, false),
            Link::Plural => (SuffixClass::One, true),
            Link::Low | Link::Mid => (SuffixClass::One, false),
        };
        Suffix {
            template: self.template,
            class,
            lengthens: self.lvl,
            lowerable,
            bare_coda: false,
            free_stem: false,
            // `Link::Plural` means "links like the plural", and that includes `NOM-PL-003`: a
            // gradable stem in a high vowel keeps the link. It is what separates `szomorúan` and
            // `emberien` from `olcsón` and `drágán` (`NOM-ADJ-010`).
            high_vowel_link: matches!(self.link, Link::Plural),
        }
    }
}

/// `DER-NOUN-001`: `-sÁg` on adjectives, fully productive.
pub const SAG: Der = Der {
    id: "DER-NOUN-001",
    template: &[Seg::Lit("s"), Seg::Var(Arch::AAcute), Seg::Lit("g")],
    input: &[Pos::Adjective],
    output: Pos::Noun,
    link: Link::None,
    // Non-lengthening, which is what makes it `lustaság` and not `*lustáság`.
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: Some(abstract_base),
    entry_filter: None,
    stem: Stem::Free,
    truncates_vowel: false,
};

/// Whether a noun is the occupational or functional person noun `-sÁg` needs.
///
/// Rules: `DER-NOUN-003`
fn occupation(entry: &Entry) -> bool {
    entry.has(Trait::Occupation)
}

/// `DER-NOUN-003`: `-sÁg` on a noun, productive over the occupational and functional person
/// nouns and nowhere else (`menedzserség`, `brókerség`, `vezérség`, not `*székség`).
///
/// The same suffix as [`SAG`] with the other input category, which is why the record repeats
/// rather than widening `SAG`: the adjective base is `P` and the noun base `P-dom`, and the
/// domain is a trait the lexicon carries, not a shape.
pub const SAG_N: Der = Der {
    id: "DER-NOUN-003",
    template: &[Seg::Lit("s"), Seg::Var(Arch::AAcute), Seg::Lit("g")],
    input: &[Pos::Noun],
    output: Pos::Noun,
    link: Link::None,
    lvl: false,
    out_lowering: false,
    prod: Prod::PDom,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: Some(occupation),
    stem: Stem::Free,
    truncates_vowel: false,
};

/// `DER-NOUN-006`: `-Ás` on verbs, the most productive deverbal suffix.
pub const AS: Der = Der {
    id: "DER-NOUN-006",
    template: &[Seg::Var(Arch::AAcute), Seg::Lit("s")],
    input: &[Pos::Verb],
    output: Pos::Noun,
    link: Link::Own,
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// `DER-ADJ-005`: `-(O)s` on nouns, taking the plural's linking vowel.
pub const OS: Der = Der {
    id: "DER-ADJ-005",
    template: &[Seg::Var(Arch::O), Seg::Lit("s")],
    // The conditions name an adjective base explicitly, low-linking: `vöröses`, `szegedies`.
    input: &[Pos::Noun, Pos::ProperNoun, Pos::Numeral, Pos::Adjective],
    output: Pos::Adjective,
    link: Link::Plural,
    lvl: true,
    out_lowering: true,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// `DER-VERB-015`: the medio-passive `-Ódik` on a transitive verb's syncopated stem
/// (`íródik`, `ismétlődik`, `mosódik`); the output inflects as an `-ik` verb.
pub const ODIK_V: Der = Der {
    id: "DER-VERB-015",
    template: &[Seg::Var(Arch::OAcute), Seg::Lit("dik")],
    input: &[Pos::Verb],
    output: Pos::Verb,
    link: Link::None,
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::BoundNoEp,
    truncates_vowel: false,
};

/// `DER-ADJ-001`: relational `-i`, invariant and transparent to harmony.
pub const I: Der = Der {
    id: "DER-ADJ-001",
    template: &[Seg::Lit("i")],
    input: &[Pos::Noun, Pos::ProperNoun, Pos::Adverb, Pos::Postposition],
    output: Pos::Adjective,
    link: Link::None,
    // Non-lengthening: `Buda` gives `budai`, not `*budái`.
    lvl: false,
    out_lowering: true,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: Some(relational_base),
    entry_filter: None,
    // `DER-MODEL-006`: the epenthetic vowel stays (`ároki`, `toronyi`) except in the `-Alom`/`-Elem`
    // nouns, which `derived` syncopates (`hatalmi`); shortening and the v-stem are lexical.
    stem: Stem::Free,
    truncates_vowel: false,
};

/// `DER-ADV-001`: `-An`, the default manner adverb, on adjectives.
///
/// `NOM-ADJ-010` owns the surface: `-n` after a vowel and a lowerable linking vowel plus `n` after
/// a consonant, the vowel following the adjective's own lowering value. So `olcsó` gives `olcsón`,
/// `halk` gives `halkan` and `gazdag` gives `gazdagon`. The `ú`/`ű` adjectives that truncate
/// (`hosszan`, `könnyen`) do so lexically by `DER-ADV-002` and are lexicon entries.
pub const AN: Der = Der {
    id: "DER-ADV-001",
    template: &[Seg::Var(Arch::O), Seg::Lit("n")],
    input: &[Pos::Adjective],
    output: Pos::Adverb,
    link: Link::Plural,
    lvl: true,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// Whether a stem is in the domain `DER-ADV-004` states for `-lAg`: an `-i` adjective or a present
/// participle (`gyakorlatilag`, `hallgatólag`, `serkentőleg`).
///
/// The category is checked separately, so this is only the shape. `politikai` and `szakmai` end in
/// `ai`, so excluding that ending to keep a noun's possessed plural out excluded them too.
fn lag_base(stem: &str) -> bool {
    stem.ends_with('i') || participial(stem)
}

/// `DER-ADJ-002`: `-i` takes no `i`-final common noun (`*zoknii`), no `-ció` noun
/// (`*privatizációi`) and no acronym (`*PM-i`). An `i`-final name writes one `i` (`tamási`), which
/// `derived` answers before the filter.
fn relational_base(stem: &str) -> bool {
    !stem.ends_with('i') && !stem.ends_with("ció") && !crate::generate::is_acronym(stem)
}

/// `-lAg`, the adverb of an `-i` adjective or a present participle. `DER-ADV-004`.
///
/// `P-dom`, and `DER-MODEL-012` excludes a `P-dom` suffix from generation because the domain is
/// normally a semantic feature the lexicon does not carry. This one's domain is stated as "`-i`
/// adjectives of disciplines" and participles, and both are in the form, so it is computed by
/// `lag_base` rather than listed. That is what makes `gazdaságilag` and `hallgatólag` analysable
/// without inventing vocabulary from an arbitrary base.
pub const LAG: Der = Der {
    id: "DER-ADV-004",
    template: &[Seg::Lit("l"), Seg::Var(Arch::A), Seg::Lit("g")],
    input: &[Pos::Adjective],
    output: Pos::Adverb,
    link: Link::None,
    lvl: true,
    out_lowering: false,
    prod: Prod::PDom,
    j_after_vowel: false,
    stem_filter: Some(lag_base),
    entry_filter: None,
    stem: Stem::Free,
    truncates_vowel: false,
};

/// A derivational suffix, as a value small enough to sit in a feature stack.
///
/// [`Morph`](crate::features::Morph) has to stay `Copy` and serialisable, so the stack carries this
/// identifier rather than the record itself.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[non_exhaustive]
pub enum DerId {
    /// `-sÁg` on an adjective. `DER-NOUN-001`.
    Sag,
    /// `-sÁg` on an occupational noun. `DER-NOUN-003`.
    SagN,
    /// `-Ás` on a verb. `DER-NOUN-006`.
    As,
    /// `-Ó` on a verb. `DER-NOUN-008`.
    O,
    /// `-(O)s` on a noun. `DER-ADJ-005`.
    Os,
    /// Relational `-i`. `DER-ADJ-001`.
    I,
    /// Privative `-tAlAn`. `DER-ADJ-009`.
    Talan,
    /// `-(j)Ú` possessive adjective. `DER-ADJ-008`.
    U,
    /// `-(O)z` on a noun. `DER-VERB-001`.
    Oz,
    /// `-ít` on an adjective. `DER-VERB-008`.
    It,
    /// Diminutive `-kA`. `DER-DIM-001`.
    Ka,
    /// Diminutive `-(O)cskA`. `DER-DIM-001`.
    Ocska,
    /// `-hAtÓ`, the potential participle. `DER-ADJ-019`.
    Hato,
    /// `-nyi`, 'as much as one N'. `DER-ADJ-013`.
    Nyi,
    /// `-An`, the default manner adverb. `DER-ADV-001`.
    An,
    /// `-lAg` on an `-i` adjective. `DER-ADV-004`.
    Lag,
    /// `-(O)gAt`, the frequentative and diminutive verb. `DER-VERB-013`.
    Ogat,
    /// The causative `-tAt` / `-At` (`VRB-CAUS-001`).
    Causative,
    /// `-ik` on a comparative, the one of two (`NOM-ADJ-009`).
    Ik,
    /// Person noun `-Ár` (`DER-NOUN-019`), unproductive.
    Ar,
    /// Occupation noun `-Ász` (`DER-NOUN-013`), unproductive.
    Asz,
    /// Denominal verb `-(O)l` (`DER-VERB-003`), unproductive.
    Ol,
    /// `-(A)ll`, 'consider too ADJ' (`DER-VERB-011`), unproductive.
    All,
    /// Inchoative `-Odik`, 'become ADJ' (`DER-VERB-009`), unproductive outside its domain.
    Odik,
    /// Distributive-temporal `-(O)ntA` (`DER-ADV-008`), unproductive.
    Nta,
    /// `-né`, the wife's name (`DER-NOUN-024`).
    Ne,
    /// Medio-passive `-Ódik` on a transitive verb (`DER-VERB-015`).
    OdikV,
    /// Durative `-(O)g` of a sound-imitating root (`DER-VERB-018`).
    Og,
    /// Momentaneous `-An` of a sound-imitating root (`DER-VERB-018`).
    Momentaneous,
    /// `-Ós` on a present participle (`DER-ADJ-017`).
    OsHabitual,
    /// Passive `-tAtik` / `-Atik` on a transitive verb (`DER-VERB-020`).
    Passive,
    /// `-kOdik`, 'behave ADJ', on an `-(O)s` or privative adjective (`DER-VERB-012`).
    Kodik,
}

impl DerId {
    /// The record this identifier names.
    #[must_use]
    pub const fn record(self) -> &'static Der {
        match self {
            Self::Sag => &SAG,
            Self::SagN => &SAG_N,
            Self::As => &AS,
            Self::O => &O,
            Self::Os => &OS,
            Self::I => &I,
            Self::Talan => &TALAN,
            Self::U => &U,
            Self::Oz => &OZ,
            Self::It => &IT,
            Self::Ka => &KA,
            Self::Ocska => &OCSKA,
            Self::Hato => &HATO,
            Self::Nyi => &NYI,
            Self::An => &AN,
            Self::Lag => &LAG,
            Self::Ogat => &OGAT,
            Self::Causative => &TAT,
            Self::Ik => &IK,
            Self::Ar => &AR,
            Self::Asz => &ASZ,
            Self::Ol => &OL,
            Self::All => &ALL,
            Self::Odik => &ODIK,
            Self::Nta => &NTA,
            Self::Ne => &NE,
            Self::OdikV => &ODIK_V,
            Self::Og => &OG,
            Self::Momentaneous => &AN_MOM,
            Self::OsHabitual => &OS_HABITUAL,
            Self::Passive => &TATIK,
            Self::Kodik => &KODIK,
        }
    }

    /// Every identifier, in declaration order.
    pub const ALL: [Self; 32] = [
        Self::Sag,
        Self::SagN,
        Self::As,
        Self::O,
        Self::Os,
        Self::I,
        Self::Talan,
        Self::U,
        Self::Oz,
        Self::It,
        Self::Ka,
        Self::Ocska,
        Self::Hato,
        Self::Nyi,
        Self::An,
        Self::Lag,
        Self::Ogat,
        Self::Causative,
        Self::Ik,
        Self::Ar,
        Self::Asz,
        Self::Ol,
        Self::All,
        Self::Odik,
        Self::Nta,
        Self::Ne,
        Self::OdikV,
        Self::Og,
        Self::Momentaneous,
        Self::OsHabitual,
        Self::Passive,
        Self::Kodik,
    ];
}

/// `DER-NOUN-019`: person noun `-Ár`, on the bound stem, truncating a final vowel: `titkár`,
/// `csatár`. Unproductive, so it only reproduces a listed word.
pub const AR: Der = Der {
    id: "DER-NOUN-019",
    template: &[Seg::Var(Arch::AAcute), Seg::Lit("r")],
    input: &[Pos::Noun, Pos::Verb],
    output: Pos::Noun,
    link: Link::Own,
    lvl: false,
    out_lowering: false,
    prod: Prod::R,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: true,
};

/// `DER-NOUN-013`: occupation noun `-Ász`, on the bound stem, truncating a final vowel:
/// `lovász`, `fodrász`, `kertész`. Unproductive.
pub const ASZ: Der = Der {
    id: "DER-NOUN-013",
    template: &[Seg::Var(Arch::AAcute), Seg::Lit("sz")],
    input: &[Pos::Noun],
    output: Pos::Noun,
    link: Link::Own,
    lvl: false,
    out_lowering: false,
    prod: Prod::R,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: true,
};

/// `DER-VERB-003`: denominal `-(O)l`, the `-(O)z` pattern's twin: `kezel`, `titkol`, `énekel`.
/// Unproductive; each verb is listed.
pub const OL: Der = Der {
    id: "DER-VERB-003",
    template: &[Seg::Var(Arch::O), Seg::Lit("l")],
    input: &[Pos::Noun],
    output: Pos::Verb,
    link: Link::Plural,
    lvl: true,
    out_lowering: false,
    prod: Prod::R,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// `DER-VERB-011`: `-(A)ll` on quantity adjectives and, lexically, a noun: `sokall`, `sarkall`,
/// `drágáll`. Unproductive.
pub const ALL: Der = Der {
    id: "DER-VERB-011",
    template: &[Seg::Var(Arch::A), Seg::Lit("ll")],
    input: &[Pos::Adjective, Pos::Noun],
    output: Pos::Verb,
    link: Link::Low,
    lvl: true,
    out_lowering: false,
    prod: Prod::R,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// `DER-VERB-009`: inchoative `-Odik` on the stem that keeps its epenthetic vowel, truncating a
/// final one: `bátorodik`, `feketedik`, `sötétedik`. Productive only on a semantic class, so it
/// reproduces listed verbs.
pub const ODIK: Der = Der {
    id: "DER-VERB-009",
    template: &[Seg::Var(Arch::O), Seg::Lit("dik")],
    input: &[Pos::Adjective, Pos::Noun],
    output: Pos::Verb,
    link: Link::Mid,
    lvl: false,
    out_lowering: false,
    prod: Prod::R,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::BoundNoEp,
    truncates_vowel: true,
};

/// `DER-ADV-008`: distributive-temporal `-(O)ntA` on the bound stem: `hetente`, `naponta`.
/// Unproductive; `havonta` keeps a mid vowel lexically.
pub const NTA: Der = Der {
    id: "DER-ADV-008",
    template: &[Seg::Var(Arch::O), Seg::Lit("nt"), Seg::Var(Arch::A)],
    input: &[Pos::Noun],
    output: Pos::Adverb,
    link: Link::Mid,
    lvl: false,
    out_lowering: false,
    prod: Prod::R,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// Whether the stem is a comparative, which is the only base `-ik` takes.
fn comparative_shaped(stem: &str) -> bool {
    stem.ends_with("bb")
}

/// `NOM-ADJ-009`: `-ik` on a comparative, picking one of two: `nagyobbik`, `kisebbik`.
///
/// Inflects like a lowering noun, which the rule measures as near-categorical: `nagyobbikat`
/// 1,218 against `nagyobbikot` 3.
pub const IK: Der = Der {
    id: "NOM-ADJ-009",
    template: &[Seg::Lit("ik")],
    input: &[Pos::Adjective],
    output: Pos::Adjective,
    link: Link::None,
    lvl: false,
    out_lowering: true,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: Some(comparative_shaped),
    entry_filter: None,
    stem: Stem::Free,
    truncates_vowel: false,
};

/// `DER-NOUN-008`: `-Ó` on verbs, the agent, instrument and place suffix.
pub const O: Der = Der {
    id: "DER-NOUN-008",
    template: &[Seg::Var(Arch::OAcute)],
    input: &[Pos::Verb],
    output: Pos::Noun,
    link: Link::Own,
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// `DER-ADJ-009`: privative `-tAlAn` after a consonant, `-tlAn` after a vowel.
///
/// The two allomorphs are one record here because the linking vowel does the work: class I with a
/// leading low slot gives `gondtalan` after a consonant and `sótlan` after a vowel.
pub const TALAN: Der = Der {
    id: "DER-ADJ-009",
    template: &[
        Seg::Var(Arch::A),
        Seg::Lit("tl"),
        Seg::Var(Arch::A),
        Seg::Lit("n"),
    ],
    input: &[Pos::Noun, Pos::Adjective, Pos::Verb],
    output: Pos::Adjective,
    link: Link::Low,
    lvl: true,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// `DER-ADJ-009`: the privative on a consonant-final base that neither lowers nor is epenthetic.
///
/// The rule splits the privative three ways by what the base does: a vowel-final base takes
/// `-tlAn` with lowering, a lowering or epenthetic base takes `-AtlAn` on the bound stem
/// (`lábatlan`, `álmatlan`), and everything else takes `-tAlAn` on the free stem with no linking
/// vowel at all (`víztelen`, `szívtelen`, `gondtalan`). Only the third needed a record of its own:
/// it is the one case where the stem is free, so the shortening of `víz` must not happen.
///
/// The four lexical `-AtlAn` forms on such bases (`gondatlan`, `pártatlan`, `pontatlan`) are
/// stored words, and the doublet `gondatlan` 'careless' beside `gondtalan` 'carefree' is why they
/// have to be: the regular formation is the one generated here, and both are current
/// (`gondtalan` 29,317, `gondatlan` 25,878 `[WC2]`).
///
/// The split is real but it is not the lowering trait, which is a measured residue rather than a
/// rule this can apply. `víz`, `szív` and `könyv` all lower in the accusative (`vizet`, `szívet`,
/// `könyvet`) and all take the free stem anyway: `víztelen` 345 against `vizetlen` 30,
/// `szívtelen` 5,975 against `szívetlen` 7, `könyvtelen` 20 against `könyvetlen` 3, while the
/// lowering bases the rule groups them against go the other way by the same margins
/// (`lábatlan` 6,715 : 8, `fogatlan` 5,593 : 12, `fületlen` 716 : 8) `[WC2]`. Closing that needs a
/// measured lexical axis, as the other stem classes have.
pub const TALAN_FREE: Der = Der {
    id: "DER-ADJ-009",
    template: &[
        Seg::Lit("t"),
        Seg::Var(Arch::A),
        Seg::Lit("l"),
        Seg::Var(Arch::A),
        Seg::Lit("n"),
    ],
    input: &[Pos::Noun, Pos::Adjective, Pos::Verb],
    output: Pos::Adjective,
    link: Link::None,
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Free,
    truncates_vowel: false,
};

/// A `g`-final stem is not attested with the frequentative, and the grammar model excludes it.
fn not_g_final(stem: &str) -> bool {
    !stem.ends_with('g')
}

/// `DER-VERB-014`: `-(O)gAt` skips the reflexive and medial verbs, which their suffix shows:
/// `-Ódik`, `-kOdik`, `-kÓdik`, `-kOzik`, `-kÓzik`, `-Odik` (`*gazdagodgat`, `*törülközget`).
fn frequentative_base(stem: &str) -> bool {
    not_g_final(stem) && !medial(stem)
}

/// Whether a verb stem ends in a reflexive or medial suffix.
///
/// A suffix follows a base, so a monosyllable is a root (`szed`, `fed`), and a stem that is a
/// listed noun plus `-(O)z` is denominal, its `k` the noun's (`emlék` -> `emlékez`, not
/// `ölel` -> `ölelkez`).
///
/// Rules: `DER-VERB-014`
fn medial(stem: &str) -> bool {
    const MEDIAL: [&str; 15] = [
        "ód", "őd", "kod", "ked", "köd", "kód", "kőd", "koz", "kez", "köz", "kóz", "kőz", "od",
        "ed", "öd",
    ];
    MEDIAL.iter().any(|e| stem.ends_with(e)) && !monosyllabic(stem) && !denominal_z(stem)
}

/// Whether a verb stem is a listed noun plus `-(O)z` (`DER-VERB-001`).
fn denominal_z(stem: &str) -> bool {
    stem.strip_suffix('z')
        .and_then(|s| s.strip_suffix(['o', 'e', 'ö']))
        .is_some_and(|noun| lexicon().get_pos(noun, Pos::Noun).is_some())
}

/// `DER-VERB-008`: `-ít` takes no `-i` adjective (`*művészisít`) and no privative
/// (`*lábatlanít`, `DER-ADJ-011`).
fn factitive_base(stem: &str) -> bool {
    !stem.ends_with('i') && !privative_shaped(stem)
}

/// Whether a stem ends in the privative `-tAlAn`.
fn privative_shaped(stem: &str) -> bool {
    ["talan", "telen", "tlan", "tlen"]
        .iter()
        .any(|e| stem.ends_with(e))
}

/// `DER-NOUN-004`: `-sÁg` takes no `-nyi` adjective (`*kanálnyiság`); `DER-NOUN-002`: after a
/// geminate `ss` the linked form is listed (`frissesség`), and the bare one would simplify to
/// `*frisség`.
fn abstract_base(stem: &str) -> bool {
    !stem.ends_with("nyi") && !stem.ends_with("ss")
}

/// `DER-DIM-004`: no diminutive on an abstract noun, which its own suffix shows (`*unalomka`,
/// `*boldogságocska`).
fn abstract_shaped(stem: &str) -> bool {
    ["ság", "ség", "alom", "elem", "ás", "és"]
        .iter()
        .any(|e| stem.ends_with(e))
}

/// `DER-DIM-001`: `-(O)cskA` yields to `-kA` after `i` (`*csokicska`).
fn ocska_base(stem: &str) -> bool {
    !stem.ends_with('i') && !abstract_shaped(stem)
}

/// `DER-VERB-013`: `-(O)gAt` on a monosyllabic stem, which takes the mid linking vowel.
///
/// `adogat`, `eszeget`, `főzöget`, `kötöget`, `ütöget`, `lépeget`.
pub const OGAT: Der = Der {
    id: "DER-VERB-013",
    template: &[
        Seg::Var(Arch::O),
        Seg::Lit("g"),
        Seg::Var(Arch::A),
        Seg::Lit("t"),
    ],
    input: &[Pos::Verb],
    output: Pos::Verb,
    link: Link::Mid,
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: Some(frequentative_base),
    entry_filter: None,
    stem: Stem::BoundNoEp,
    truncates_vowel: false,
};

/// Whether a stem is the present participle of a verb, the base `-Ós` takes.
///
/// `DER-ADJ-017`'s event reading ('involving V-ing') is productive from activity verbs. The
/// lexicon marks no verb stative, so every verb passes: a listed verb's participle (`ballagó`,
/// `túrázó`, `lövöldöző`), and one carrying a frequentative or durative suffix whose verb is
/// itself derived (`nézegető`, `üldögélő`; `DER-VERB-013`, `DER-VERB-017`). The participle of a
/// potential verb is not a verb's own (`*olvashatós`).
///
/// Rules: `DER-ADJ-017`
fn verbal_participle(stem: &str) -> bool {
    const ASPECTUAL: [&str; 20] = [
        "gat", "get", "göt", "gál", "gél", "kál", "kél", "dos", "des", "dös", "dal", "del", "dál",
        "dél", "ároz", "éroz", "ong", "eng", "öng", "ész",
    ];
    stem.strip_suffix(['ó', 'ő']).is_some_and(|root| {
        ASPECTUAL.iter().any(|e| root.ends_with(e))
            || [root.to_owned(), format!("{root}ik")]
                .iter()
                .any(|l| lexicon().get_pos(l, Pos::Verb).is_some())
    })
}

/// `DER-ADJ-017`: `-Ós`, the `-(O)s` adjective of the present participle.
pub const OS_HABITUAL: Der = Der {
    id: "DER-ADJ-017",
    template: &[Seg::Var(Arch::O), Seg::Lit("s")],
    input: &[Pos::Noun, Pos::Adjective],
    output: Pos::Adjective,
    link: Link::Plural,
    lvl: true,
    out_lowering: true,
    prod: Prod::PDom,
    j_after_vowel: false,
    stem_filter: Some(verbal_participle),
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// Whether a root is the sound-imitating one `DER-VERB-018` forms its verbs from.
///
/// The rule's domain is the semantic flag `onomatopoeic`, which the lexicon does not carry, so
/// the computable domain is the shape the rule itself describes together with the category: a
/// monosyllabic interjection whose final consonant is written long (`bumm`, `zutty`, `kopp`,
/// `csatt`, `puff`). The proxy is not exact, because the exclamations of the same shape (`hess`,
/// `kuss`, `sicc`) sit inside it and imitate nothing.
///
/// Rules: `DER-VERB-018`
fn onomatopoeic_root(stem: &str) -> bool {
    monosyllabic(stem) && crate::generate::doubled_final(stem).is_some()
}

/// `DER-VERB-018`: the durative `-(O)g` of a sound-imitating root (`bummog`, `zuttyog`).
pub const OG: Der = Der {
    id: "DER-VERB-018",
    template: &[Seg::Var(Arch::O), Seg::Lit("g")],
    input: &[Pos::Interjection],
    output: Pos::Verb,
    link: Link::Mid,
    lvl: false,
    out_lowering: false,
    prod: Prod::PDom,
    j_after_vowel: false,
    stem_filter: Some(onomatopoeic_root),
    entry_filter: None,
    stem: Stem::Free,
    truncates_vowel: false,
};

/// `DER-VERB-018`: the momentaneous `-An` of a sound-imitating root (`bumman`, `zuttyan`).
///
/// The rule geminates the root's final consonant before it; the roots the lexicon lists are the
/// geminate ones already (`kopp` beside the `kop` of `kopog`), so nothing is doubled here.
pub const AN_MOM: Der = Der {
    id: "DER-VERB-018",
    template: &[Seg::Var(Arch::A), Seg::Lit("n")],
    input: &[Pos::Interjection],
    output: Pos::Verb,
    link: Link::Low,
    lvl: false,
    out_lowering: false,
    prod: Prod::PDom,
    j_after_vowel: false,
    stem_filter: Some(onomatopoeic_root),
    entry_filter: None,
    stem: Stem::Free,
    truncates_vowel: false,
};

/// `VRB-CAUS-001`: the causative after a polysyllabic stem, an `-ít` stem, or a monosyllable
/// ending in a vowel plus `t`. The stem's own final `t` and the suffix's meet as a written double
/// (`fut` -> `futtat`, `tanít` -> `taníttat`), which is plain concatenation and needs no rule.
pub const TAT: Der = Der {
    id: "VRB-CAUS-001",
    template: &[Seg::Lit("t"), Seg::Var(Arch::A), Seg::Lit("t")],
    input: &[Pos::Verb],
    output: Pos::Verb,
    link: Link::None,
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::BoundNoEp,
    truncates_vowel: false,
};

/// `VRB-CAUS-001`: the causative after any other monosyllable, which is the default for them.
pub const AT: Der = Der {
    id: "VRB-CAUS-001",
    template: &[Seg::Var(Arch::A), Seg::Lit("t")],
    input: &[Pos::Verb],
    output: Pos::Verb,
    link: Link::None,
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::BoundNoEp,
    truncates_vowel: false,
};

/// `DER-VERB-020`: the passive on the stem that takes the causative `-tAt`, an `-ik` verb
/// (`foglaltatik`, `kinyittatik`, `taníttatik`).
pub const TATIK: Der = Der {
    id: "DER-VERB-020",
    template: &[Seg::Lit("t"), Seg::Var(Arch::A), Seg::Lit("tik")],
    ..TAT
};

/// `DER-VERB-020`: the passive on the stem that takes the causative `-At` (`adatik`, `oltatik`).
pub const ATIK: Der = Der {
    id: "DER-VERB-020",
    template: &[Seg::Var(Arch::A), Seg::Lit("tik")],
    ..AT
};

/// The passive allomorph of a base, which is its causative's (`VRB-CAUS-001`).
fn passive_record(stem: &str) -> &'static Der {
    if std::ptr::eq(causative_record(stem), &TAT) {
        &TATIK
    } else {
        &ATIK
    }
}

/// `DER-VERB-012`: the adjective bases of `-kOdik`, the `-(O)s` adjectives, whose `s` follows a
/// vowel, and the privatives (`okoskodik`, `petőfieskedik`, `nyugtalankodik`; not `gyors`).
fn kodik_base(stem: &str) -> bool {
    let tail: Vec<Letter> = letters(stem).map(|s| s.letter).collect();
    matches!(tail.as_slice(), [.., v, Letter::S] if v.is_vowel()) || privative_shaped(stem)
}

/// `DER-VERB-012`: `-kOdik` on the free stem of an adjective, an `-ik` verb. A noun's
/// `-(O)skOdik` is `-(O)s` plus this suffix (`katonáskodik`, `mérnökösködik`).
pub const KODIK: Der = Der {
    id: "DER-VERB-012",
    template: &[Seg::Lit("k"), Seg::Var(Arch::O), Seg::Lit("dik")],
    input: &[Pos::Adjective],
    output: Pos::Verb,
    link: Link::None,
    lvl: false,
    out_lowering: false,
    prod: Prod::PDom,
    j_after_vowel: false,
    stem_filter: Some(kodik_base),
    entry_filter: None,
    stem: Stem::Free,
    truncates_vowel: false,
};

/// The stem a deverbal suffix attaches to, where it is not the lemma's own.
///
/// `DER-MODEL-006` puts `-Ás` and `-Ó` on the bound stem, which for a verb is whichever of three
/// stems the verb has: the `v` stem of an sz-d verb that has one (`alv` -> `alvás`), the stem a
/// suppletive verb already builds its participle on (`ev` -> `evés`), or the alternant the rest of
/// the paradigm uses (`emlékez` -> `emlékezés`, `söpr` -> `söprés`).
///
/// `None` where the verb has no alternant at all, which leaves the ordinary path to attach the
/// suffix to the lemma.
fn deverbal_stem(entry: &Entry) -> Option<String> {
    if let Some(v) = crate::verbal::v_stem(&entry.lemma) {
        return Some(v);
    }
    if let Some(p) = crate::irregular::irregular(&entry.lemma)
        .map(|i| i.participle)
        .filter(|p| !p.is_empty())
    {
        return Some(p.to_owned());
    }
    crate::verbal::alternant_stem(entry)
}

/// Which causative allomorph a stem takes (`VRB-CAUS-001`).
///
/// Computed, not listed: a polysyllable takes `-tAt`, and so does a monosyllable whose final `t`
/// follows a vowel (`fut`, `köt`, `lát`). Every other monosyllable takes `-At`, including one whose
/// `t` follows a consonant (`fest`, `tart`, `önt`).
///
/// The rule also names a closed minority of monosyllables that take `-tAt` against this default,
/// mainly `l`-final (`ül` -> `ültet`, `él` -> `éltet`). They are not here, so their causative comes
/// out with the default suffix and is refused rather than listed. Nineteen of the hundred and
/// fifty-three monosyllables measured for `OQ-05-012` fall that way.
fn causative_record(stem: &str) -> &'static Der {
    if picur_morph_ortho::syllable_count(stem) > 1 {
        return &TAT;
    }
    if lists(Role::CausativeTat, stem) {
        return &TAT;
    }
    let (mut prev, mut last) = (None, None);
    for span in letters(stem) {
        prev = last;
        last = Some(span.letter);
    }
    match (prev, last) {
        (Some(v), Some(Letter::T)) if v.is_vowel() => &TAT,
        _ => &AT,
    }
}

/// `DER-VERB-013`: `-gAt` on a polysyllabic stem, which takes no linking vowel.
///
/// `olvasgat`, `sétálgat`, `beszélget`, and the epenthetic verbs, which keep their vowel and so
/// count as polysyllabic: `hímezget`, `udvarolgat`, `söpörget`.
///
/// The one attested form this misses is `jövöget` (111 `[WC2]`): `jön` is a v-stem verb but is not
/// marked as one, its finite forms coming from the irregular table instead, and giving it the trait
/// to win one derivative would put `jönni` at risk.
pub const GAT: Der = Der {
    id: "DER-VERB-013",
    template: &[Seg::Lit("g"), Seg::Var(Arch::A), Seg::Lit("t")],
    input: &[Pos::Verb],
    output: Pos::Verb,
    link: Link::None,
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: Some(frequentative_base),
    entry_filter: None,
    stem: Stem::BoundNoEp,
    truncates_vowel: false,
};

/// Whether the stem has exactly one vowel, which is what selects the linking vowel of the
/// frequentative (`DER-VERB-013`).
fn monosyllabic(stem: &str) -> bool {
    picur_morph_ortho::letters(stem)
        .filter(|s| s.letter.is_vowel())
        .count()
        == 1
}

/// The record a suffix uses on this particular base.
///
/// Almost every suffix has one record. The privative is the exception `DER-ADJ-009` describes: its
/// allomorph is selected by what the base does, not by what the suffix is. A verb base keeps
/// `-AtlAn` whatever else it does, which is a different suffix wearing the same shape: `olvasatlan`
/// 10,233 against `olvastalan` 3, `hallatlan` 15,690 against `halltalan` 0 `[WC2]`.
///
/// Rules: `DER-ADJ-009`, `DER-MODEL-006`
#[must_use]
pub fn record_for(id: DerId, base: &Entry) -> &'static Der {
    if matches!(id, DerId::Causative | DerId::Passive) {
        let stem = if base.pos == Pos::Verb {
            crate::verbal::present_stem(&base.lemma)
        } else {
            &base.lemma
        };
        return if id == DerId::Causative {
            causative_record(stem)
        } else {
            passive_record(stem)
        };
    }
    // The count is on the stem the suffix attaches to, not on the lemma: `iszik` is written with
    // two vowels but its stem is `isz`, and it takes the linking vowel (`iszogat`). A preverb is
    // not part of that stem (`kinyitogat`).
    if id == DerId::Ogat
        && !monosyllabic(if base.pos == Pos::Verb {
            let lemma: &str = &base.lemma;
            crate::verbal::present_stem(crate::verbal::preverb_base(lemma).unwrap_or(lemma))
        } else {
            &base.lemma
        })
    {
        return &GAT;
    }
    if id == DerId::Talan
        && matches!(base.pos, Pos::Noun | Pos::Adjective)
        && !base.has(Trait::Low)
        && !base.has(Trait::Epenthetic)
        && base
            .lemma
            .chars()
            .last()
            .is_some_and(|c| !"aáeéiíoóöőuúüű".contains(c))
    {
        return &TALAN_FREE;
    }
    id.record()
}

/// `DER-ADJ-008`: `-(j)Ú` 'having N'.
pub const U: Der = Der {
    id: "DER-ADJ-008",
    template: &[Seg::Var(Arch::UAcute)],
    input: &[Pos::Noun],
    output: Pos::Adjective,
    link: Link::Own,
    lvl: false,
    out_lowering: true,
    prod: Prod::P,
    j_after_vowel: true,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// `DER-VERB-001`: `-(O)z` on nouns and adjectives.
pub const OZ: Der = Der {
    id: "DER-VERB-001",
    template: &[Seg::Var(Arch::O), Seg::Lit("z")],
    input: &[Pos::Noun],
    output: Pos::Verb,
    link: Link::Plural,
    lvl: true,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// `DER-VERB-008`: `-ít`, invariant and opaque to harmony.
pub const IT: Der = Der {
    id: "DER-VERB-008",
    template: &[Seg::Lit("ít")],
    input: &[Pos::Adjective],
    output: Pos::Verb,
    link: Link::None,
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: Some(factitive_base),
    entry_filter: None,
    stem: Stem::BoundNoEp,
    truncates_vowel: true,
};

/// Whether `-kA` may attach: a stem ending in `i`, or a polysyllabic one ending in a consonant
/// that is not a stop.
///
/// `DER-DIM-001`'s rule 6 gives a monosyllabic consonant-final base `-(O)cskA` only (`szívecske`,
/// `lábacska`, not `*szívke` or `*lábka`); the monosyllables with an established `-kA` (`lányka`,
/// `tálka`) are entries, which the rule calls exceptions.
///
/// Rules: `DER-DIM-001`
fn diminutive_base(stem: &str) -> bool {
    use picur_morph_ortho::Letter;
    if abstract_shaped(stem) {
        return false;
    }
    let Some(last) = picur_morph_ortho::letters(stem).last().map(|s| s.letter) else {
        return false;
    };
    if last == Letter::I {
        return true;
    }
    !monosyllabic(stem)
        && last.is_consonant()
        && !matches!(
            last,
            Letter::P
                | Letter::B
                | Letter::T
                | Letter::D
                | Letter::K
                | Letter::G
                | Letter::Ty
                | Letter::Gy
        )
}

/// `DER-DIM-001`: diminutive `-kA`.
///
/// Lengthening is "not applicable" in the master table rather than `n`: `DER-MODEL-005` notes that
/// `-kA` never follows a stem in `a`/`e` at all, so the question does not arise.
pub const KA: Der = Der {
    id: "DER-DIM-001",
    template: &[Seg::Lit("k"), Seg::Var(Arch::A)],
    input: &[Pos::Noun, Pos::ProperNoun, Pos::Adjective],
    output: Pos::Noun,
    link: Link::None,
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: Some(diminutive_base),
    entry_filter: None,
    stem: Stem::Free,
    truncates_vowel: false,
};

/// `DER-DIM-001`: diminutive `-(O)cskA`, the second of the rule's two suffixes.
///
/// Plural-type link and the bound stem, so the alternations the plural shows appear here too:
/// `bokrocska`, `kezecske`, `levelecske`. After a vowel there is no link and the stem lengthens,
/// which is what makes `fa` give `fácska`.
///
/// Unlike `-kA` it has no restriction on the stem's final consonant: `-kA` is limited to the
/// coronal and velar stops that `diminutive_base` lists, while `-(O)cskA` has its own vowel to
/// stand between.
pub const OCSKA: Der = Der {
    id: "DER-DIM-001",
    template: &[Seg::Var(Arch::O), Seg::Lit("csk"), Seg::Var(Arch::A)],
    input: &[Pos::Noun, Pos::ProperNoun, Pos::Adjective],
    output: Pos::Noun,
    link: Link::Plural,
    lvl: true,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: Some(ocska_base),
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// `DER-ADJ-019`: `-hAtÓ`, which is the potential plus the present participle.
pub const HATO: Der = Der {
    id: "DER-ADJ-019",
    template: &[
        Seg::Lit("h"),
        Seg::Var(Arch::A),
        Seg::Lit("t"),
        Seg::Var(Arch::OAcute),
    ],
    input: &[Pos::Verb],
    output: Pos::Adjective,
    link: Link::None,
    lvl: false,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Bound,
    truncates_vowel: false,
};

/// `DER-ADJ-013`: `-nyi`, 'as much as one N'.
///
/// Invariant for harmony but lengthening: `DER-MODEL-005` lists `-nyi` among the lengthening
/// suffixes, so `deka` gives `dekányi`.
pub const NYI: Der = Der {
    id: "DER-ADJ-013",
    template: &[Seg::Lit("nyi")],
    input: &[Pos::Noun, Pos::Numeral],
    output: Pos::Adjective,
    link: Link::None,
    lvl: true,
    out_lowering: true,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: None,
    entry_filter: None,
    stem: Stem::Free,
    truncates_vowel: false,
};

/// `DER-NOUN-024`: `-né` names the wife by the husband's name: `Kovácsné`, `Katonáné`. The
/// rule's input is a person noun, which the lexicon does not mark, so the computable subset is a
/// name. The derivative keeps the base's harmony (`Szabónénak`) and takes the `-ja` possessive.
pub const NE: Der = Der {
    id: "DER-NOUN-024",
    template: &[Seg::Lit("né")],
    input: &[Pos::Noun, Pos::ProperNoun],
    output: Pos::Noun,
    link: Link::None,
    lvl: true,
    out_lowering: false,
    prod: Prod::P,
    j_after_vowel: false,
    stem_filter: Some(starts_upper),
    entry_filter: None,
    stem: Stem::Free,
    truncates_vowel: false,
};

/// Every suffix this module can generate with.
pub const PRODUCTIVE: &[Der] = &[
    SAG,
    SAG_N,
    AS,
    O,
    OS,
    I,
    TALAN,
    U,
    NYI,
    OZ,
    IT,
    KA,
    HATO,
    LAG,
    NE,
    OG,
    AN_MOM,
    OS_HABITUAL,
];

/// The base a derivational suffix attaches to.
///
/// A verb contributes its stem rather than its lemma, so the `-ik` of `ugrik` does not survive into
/// `ugrás`.
///
/// Rules: `DER-MODEL-006`
fn base(entry: &Entry, suffix: &Suffix, stem: Stem) -> String {
    // PND-NUMINF-003: the numeral 2 uses `kett-` before a vowel-initial derivational suffix, so
    // `-(O)s` gives `kettes` rather than `*kettős`.
    if entry.lemma.as_ref() == "kettő" && suffix.starts_with_vowel() {
        return "kett".to_owned();
    }
    if entry.pos == Pos::Verb {
        let present = crate::verbal::present_stem(&entry.lemma);
        // The stem alternation machinery keys off the lemma, so it is applied to the stem here by
        // way of a temporary entry carrying the same traits.
        let mut bare = Entry::new(present, Pos::Verb);
        bare.traits = entry.traits;
        return alternant(&bare, suffix).into_owned();
    }
    match stem {
        // The rule states the stem per suffix, so a literal vowel that meets the stem selects the
        // alternant even though `alternant`'s own gate cannot see it in the template.
        Stem::BoundEp => {
            let mut s = *suffix;
            s.free_stem = true;
            crate::alternation::bound_alternant(entry, &s).into_owned()
        }
        Stem::BoundNoEp => {
            let mut without = entry.clone();
            without.traits = without.traits.without(Trait::Epenthetic);
            crate::alternation::bound_alternant(&without, suffix).into_owned()
        }
        Stem::Free | Stem::Bound => alternant(entry, suffix).into_owned(),
    }
}

/// Whether `lemma` is a settlement name whose last element is a possessed noun.
///
/// Rules: `ORT-NAME-026`
fn possessive_place_name(lemma: &str) -> bool {
    lemma.chars().next().is_some_and(char::is_uppercase)
        && words(Role::PossessivePlaceEnding)
            .iter()
            .any(|e| lemma.ends_with(e.as_ref()))
}

/// The stem a suffix attaches to, after the stem choice and the vowel adjustments.
fn derivation_stem(entry: &Entry, der: &Der, suffix: &Suffix) -> String {
    let stem = base(entry, suffix, der.stem);
    // `ORT-FOR-010`: a final `o`/`ö` is a foreign ending pronounced long, so it lengthens before
    // the non-lengthening `-i` as before any suffix: `oslói`, `malmői`. `DER-MODEL-006`: the
    // `-Alom`/`-Elem` nouns drop their epenthetic vowel before it (`hatalmi`, `fegyelmi`).
    let stem = if der.id == I.id {
        let alom =
            entry.has(Trait::Epenthetic) && (stem.ends_with("alom") || stem.ends_with("elem"));
        alom.then(|| crate::alternation::syncopated(&stem))
            .flatten()
            .unwrap_or_else(|| crate::generate::lengthen_final_o(&stem))
    } else {
        stem
    };
    // `DER-MODEL-007`: the suffix may delete a stem-final vowel before attaching.
    let deletes_before_adverb =
        der.id == AN.id && lists(Role::VowelDeletingAdjective, &entry.lemma);
    if der.truncates_vowel
        || deletes_before_adverb
        || (der.id == I.id && possessive_place_name(&entry.lemma))
    {
        truncated(&stem)
    } else {
        stem
    }
}

/// `word` with the vowel letter `pick` chooses replaced by its partner `swap` gives.
fn with_vowel(
    word: &str,
    pick: impl Fn(&[picur_morph_ortho::Span<'_>]) -> Option<usize>,
    swap: fn(Letter) -> Option<Letter>,
) -> Option<String> {
    let spans: Vec<_> = letters(word).collect();
    let s = spans[pick(&spans)?];
    let to = swap(s.letter)?;
    Some(format!(
        "{}{}{}",
        &word[..s.at],
        to.as_str(),
        &word[s.at + s.text.len()..]
    ))
}

/// Whether `form` is `stack` on a listed word that it extends by at most four letters, the
/// word's final vowel taken short too since a lengthening suffix writes it long (`drága` ->
/// `drágán`).
fn formed_by(form: &str, stack: &[crate::Morph]) -> bool {
    let cuts = letters(form).map(|s| s.at).collect::<Vec<_>>();
    cuts.iter().rev().take(4).any(|&at| {
        let prefix = &form[..at];
        let short = with_vowel(
            prefix,
            |s| {
                s.len()
                    .checked_sub(1)
                    .filter(|&i| s[i].letter.is_long_vowel())
            },
            Letter::short,
        );
        std::iter::once(prefix.to_owned()).chain(short).any(|base| {
            lexicon()
                .get(&base)
                .iter()
                .any(|e| crate::generate::generate(e, stack).is_ok_and(|f| f == form))
        })
    })
}

/// Whether an adjective or adverb is an `-An`, `-Ul`, `-képp`, `-stUl` or `-ntA` adverb, after
/// which no suffix follows (`*szépenes`, `*angolului`), however the lexicon lists it. A
/// postposition is its own word (`belül` gives `belüli`).
///
/// Rules: `DER-ADV-010`
fn terminal_adverb(lemma: &str) -> bool {
    use crate::{Case, Morph};
    let suffix = match lemma.chars().last() {
        Some('n') => Morph::Derive(DerId::An),
        Some('l') if lemma.ends_with("stul") || lemma.ends_with("stül") => {
            Morph::Case(Case::Sociativus)
        }
        Some('l') => Morph::Case(Case::Ess),
        Some('p') => Morph::Case(Case::Modalis),
        Some('a' | 'e') if lemma.ends_with("nta") || lemma.ends_with("nte") => {
            Morph::Case(Case::DistributivusTemporalis)
        }
        _ => return false,
    };
    !lexicon().has_pos(lemma, Pos::Postposition) && formed_by(lemma, &[suffix])
}

/// Whether an `-i` adjective is built on a deverbal `-Ás` noun (`pihenési`, `tanulási`), which
/// takes no `-(O)s` (`*pihenésies`).
///
/// Rules: `DER-ADJ-006`
fn deverbal_relational(stem: &str) -> bool {
    stem.strip_suffix('i')
        .filter(|n| n.ends_with("ás") || n.ends_with("és"))
        .is_some_and(|n| formed_by(n, &[crate::Morph::Derive(DerId::As)]))
}

/// Whether an `-itás` noun is the quality noun of a listed adjective, which blocks its `-(O)s`
/// adjective (`stabilitás` has `stabil`, so `*stabilitásos`; `prioritásos` has no rival).
///
/// The adjective is the root as written, with its last vowel long, or with `-is` or `-ív`:
/// `stabil`, `aktív`, `reális`, `intenzív`.
///
/// Rules: `DER-ADJ-006`
fn blocked_quality_noun(lemma: &str) -> bool {
    let Some(root) = lemma.strip_suffix("itás") else {
        return false;
    };
    let long = with_vowel(
        root,
        |s| s.iter().rposition(|l| l.letter.is_vowel()),
        Letter::long,
    )
    .unwrap_or_else(|| root.to_owned());
    [
        root.to_owned(),
        format!("{root}is"),
        format!("{long}is"),
        long.clone(),
        format!("{root}ív"),
    ]
    .iter()
    .any(|a| lexicon().has_pos(a, Pos::Adjective))
}

/// The bases a suffix refuses beyond its category and shape filter.
fn refused(entry: &Entry, der: &Der, stem: &str) -> Result<(), GenError> {
    // `PND-POSTP-010`: the `-i` licence is per postposition, so a lative one (`alá`, `mögé`) and
    // one that is a converb or a case form (`fogva`, `óta`, `át`) take none. A postposition is
    // often an adverb as well (`át`, `együtt`, `innen`), and that reading is the same word, so it
    // is licensed the same way; a homograph noun derives its own `-i` by `DER-ADJ-001` (`szám`
    // gives the ethnonym `számi`) and is not this rule's business.
    if der.id == I.id
        && matches!(entry.pos, Pos::Postposition | Pos::Adverb)
        && !lists(Role::PostpositionAdjective, &entry.lemma)
        && picur_morph_lexicon::lexicon().has_pos(&entry.lemma, Pos::Postposition)
    {
        return Err(GenError::NoSuchCell("this postposition takes no -i"));
    }
    // `DER-VERB-015`, `DER-VERB-020`: the medio-passive and the passive are made of a transitive
    // verb.
    if (der.id == ODIK_V.id || der.id == TATIK.id) && !entry.has(Trait::Transitive) {
        return Err(GenError::NoSuchCell(
            "the medio-passive and the passive need a transitive verb",
        ));
    }
    if der.id != AN.id
        && matches!(entry.pos, Pos::Adjective | Pos::Adverb)
        && terminal_adverb(&entry.lemma)
    {
        return Err(GenError::NoSuchCell("this adverb takes no further suffix"));
    }
    if der.id == OS.id
        && ((entry.pos == Pos::Adjective && deverbal_relational(stem))
            || (entry.pos == Pos::Noun && blocked_quality_noun(&entry.lemma)))
    {
        return Err(GenError::NoSuchCell("this base takes no -(O)s"));
    }
    // `DER-ADJ-010`: on a verb the privative needs a transitive base (`*ugrálatlan`,
    // `*repületlen`) and attaches to the stem, never to the participle (`*megtanultatlan`).
    if der.id == TALAN.id
        && ((entry.pos == Pos::Verb && !entry.has(Trait::Transitive))
            || (entry.pos == Pos::Adjective && stem.ends_with('t')))
    {
        return Err(GenError::NoSuchCell(
            "the privative needs a transitive verb or a non-participial base",
        ));
    }
    Ok(())
}

/// `DER-ADV-001`: an `ő`-final adjective and an `ó`-final participle take the linked `-An`
/// (`meglepően` 132,308 : `meglepőn` 145, `kiválóan` 279,837 : `kiválón` 342 [WC2]), while a
/// root `ó` adjective takes bare `-n` (`olcsón` 116,524 : `olcsóan` 43 [WC2]).
fn participial_adverb(stem: &str) -> Option<String> {
    if stem.ends_with('ő') {
        return Some(format!("{stem}en"));
    }
    (stem.ends_with('ó') && participial(stem)).then(|| format!("{stem}an"))
}

/// Whether an `ó`/`ő`-final adjective is a present participle: its stem is a verb, with or without
/// `-ik`, or a potential stem (`kiváló`, `látható`, `serkentő`; not `olcsó`).
fn participial(stem: &str) -> bool {
    let Some(root) = stem.strip_suffix(['ó', 'ő']) else {
        return false;
    };
    root.ends_with("hat")
        || root.ends_with("het")
        || [root.to_owned(), format!("{root}ik")]
            .iter()
            .any(|l| lexicon().get_pos(l, Pos::Verb).is_some())
}

/// `stem` without a final vowel, which `DER-MODEL-007`'s `trunc` column calls for.
fn truncated(stem: &str) -> String {
    let mut out = stem.to_owned();
    if picur_morph_ortho::letters(&out)
        .last()
        .is_some_and(|s| s.letter.is_vowel())
    {
        out.pop();
    }
    out
}

/// Derives a new word, when the suffix is one the rulebook licenses for generation.
///
/// Rules: `DER-MODEL-012`
pub fn derive(entry: &Entry, der: &Der) -> Result<String, GenError> {
    derived(entry, der).map(|e| e.lemma.into_string())
}

/// The `-i` adjective of a simplex name lowercases (`ORT-NAME-021`): `Oslo` gives `oslói`.
fn lowercase_name_derivative(form: &str, entry: &Entry, der: &Der) -> String {
    if der.output == Pos::Adjective && starts_upper(&entry.lemma) {
        lower_initial(form)
    } else {
        form.to_owned()
    }
}

/// The `-i` derivative of a name of more than one element, with each element's case decided.
///
/// `DER-ADJ-004`: a name whose last word is itself a name keeps every capital and takes the suffix
/// after a hyphen (`New York-i`, `Arany János-i`), unless it already carries one (`Buen
/// Tiempo-foki`). `ORT-NAME-023` and `ORT-NAME-027`: a name whose head element is a proper name
/// lowercases throughout (`győr-moson-soproni`, `budapest–bécsi`). Otherwise the head is a common
/// noun and `ORT-NAME-022`, `ORT-NAME-024`, `ORT-NAME-028` and `ORT-NAME-029` decide each element
/// on its own, a common word lowercasing and a name keeping its capital (`belső-János-dűlői`,
/// `Volga–Don-csatornai`, `római birodalmi`). What an element is settles the rest: a run of words
/// ending in a name is one name and keeps every capital (`Szent Anna-tói`, `Trinidad és Tobago
/// köztársasági`); members joined by an en dash share their case (`zala–somogyi`); and a
/// hyphenated word is what its last member is (`abaúj-hegyközi` against `Dél-Dunántúl`).
///
/// The head's own derivative is built on its listed entry where it has one, so its stem
/// alternation applies (`birodalmi`). `None` for a simplex lemma or a non-adjectival suffix.
///
/// Rules: `DER-ADJ-004`, `ORT-NAME-022`, `ORT-NAME-023`, `ORT-NAME-024`, `ORT-NAME-027`,
/// `ORT-NAME-028`, `ORT-NAME-029`
fn compound_name_derivative(entry: &Entry, der: &Der) -> Option<String> {
    let lemma: &str = &entry.lemma;
    if der.output != Pos::Adjective || !starts_upper(lemma) || !lemma.contains([' ', '-', EN_DASH])
    {
        return None;
    }
    let words: Vec<&str> = lemma.split(' ').collect();
    let (&head_word, specifier) = words.split_last()?;
    let members = name_members(head_word);
    let (&(head, head_sep), inner) = members.split_last()?;
    let head_form = head_derivative(head, entry, der)?;
    // `DER-ADJ-004`: the suffix after a hyphen, the name untouched.
    if !specifier.is_empty() && !is_common_word(head) && !lemma.contains('-') {
        let tail = head_form.strip_prefix(&lower_initial(head))?;
        return (!tail.is_empty()).then(|| format!("{lemma}-{tail}"));
    }
    let mut out = String::new();
    // A capitalised head member of a hyphenated name is a proper name whatever else it spells
    // (`Kál-Kápolna`); a simplex head word is judged by the lexicon, because a state name
    // capitalises its common nouns too (`Római Birodalom`).
    let head_is_name = if inner.is_empty() {
        !is_common_word(head)
    } else {
        starts_upper(head)
    };
    if head_is_name {
        for w in specifier {
            out.push_str(&lower_initial(w));
            out.push(' ');
        }
        for (m, sep) in inner {
            out.extend(*sep);
            out.push_str(&lower_initial(m));
        }
    } else {
        // The words before a hyphenated head form one phrase with the members hyphenated to it
        // (`Szent Anna-tó`, `Buen Tiempo-fok`); without such words each member stands alone.
        let phrase_end = if specifier.is_empty() {
            None
        } else {
            inner.last().map(|(m, _)| *m).or(specifier.last().copied())
        };
        let name_unit = phrase_end.is_some_and(|w| starts_upper(w) && !word_is_common(w));
        for w in specifier {
            if name_unit || !word_is_common(w) {
                out.push_str(w);
            } else {
                for (m, sep) in name_members(w) {
                    out.extend(sep);
                    out.push_str(&lower_initial(m));
                }
            }
            out.push(' ');
        }
        if name_unit {
            for (m, sep) in inner {
                out.extend(*sep);
                out.push_str(m);
            }
        } else {
            out.push_str(&cased_members(inner));
        }
    }
    out.extend(head_sep);
    out.push_str(&head_form);
    Some(out)
}

/// A name of several elements with a semi-word after its head, each element cased as in its `-i`
/// derivative: `AkH12` §179 writes the `-i` and the `-beli` derivative alike (`Ipoly Eurorégió`
/// gives `Ipoly eurorégióbeli` as it gives `Ipoly eurorégiói`).
///
/// `None` where the `-i` derivative does not end in the head's own (`New York-i`), which leaves
/// the semi-word to its hyphen after a name.
///
/// Rules: `ORT-NAME-022`, `ORT-NAME-028`, `DER-COMP-017`
pub(crate) fn name_with_semi_word(lemma: &str, semi: &str) -> Option<String> {
    let entry = Entry::new(lemma, Pos::Noun);
    let cased = compound_name_derivative(&entry, &I)?;
    let head_word = lemma.rsplit(' ').next()?;
    let (head, _) = *name_members(head_word).last()?;
    let own = head_derivative(head, &entry, &I)?;
    let before = cased.strip_suffix(own.as_str())?;
    Some(format!("{before}{}{semi}", lower_initial(head)))
}

/// A geographic name of `ORT-NAME-022`'s shape, a capitalised word hyphenated to a lowercase
/// common noun, split into the two.
fn geographic_parts(name: &str) -> Option<(&str, &str)> {
    let (first, noun) = name.split_once('-')?;
    (starts_upper(first)
        && !first.contains(' ')
        && !noun.contains(['-', ' '])
        && !starts_upper(noun)
        && is_common_word(noun))
    .then_some((first, noun))
}

/// The forms a geographic name takes inside a public place name ending in `generic`: written
/// solid, bare or as its `-i` derivative (`ORT-NAME-030`).
///
/// `AkH12` §181 replaces the hyphen of a geographic name with a geographic common noun by solid
/// writing once it enters a street name: `Csörsz-árok` gives `Csörszárok utca`, `Ferenc-hegy`
/// gives `Ferenchegyi út`. The derivative is the head noun's, on its listed entry, and the first
/// word of a public place name is capitalised. Empty when `generic` is no public place generic or
/// the name is not of that shape.
///
/// Rules: `ORT-NAME-030`, `ORT-NAME-022`
#[must_use]
pub fn street_names(geographic: &str, generic: &str) -> Vec<String> {
    if !lists(Role::PublicPlace, generic) {
        return Vec::new();
    }
    let Some((first, noun)) = geographic_parts(geographic) else {
        return Vec::new();
    };
    let mut out = vec![format!("{first}{noun} {generic}")];
    let bare = Entry::new(noun, Pos::Noun);
    if let Some(adjective) = head_derivative(noun, &bare, &I) {
        out.push(format!("{first}{adjective} {generic}"));
    }
    out
}

/// Whether `word` before the public place generic `generic` is a geographic name written solid as
/// `ORT-NAME-030` asks, bare or derived: a capitalised name the lexicon lists, then a lowercase
/// word the `accepts` oracle takes (`Csörszárok`, `Ferenchegyi`).
///
/// Rules: `ORT-NAME-030`
pub fn solid_in_street_name(word: &str, generic: &str, accepts: impl Fn(&str) -> bool) -> bool {
    lists(Role::PublicPlace, generic)
        && letters(word).skip(1).any(|span| {
            let (first, rest) = word.split_at(span.at);
            starts_upper(first)
                && !starts_upper(rest)
                && lexicon().get(first).iter().any(|e| e.pos == Pos::Noun)
                && letters(rest).count() > 1
                && accepts(rest)
        })
}

/// Whether `word` is a hyphenated geographic name that a following public place generic requires
/// solid (`ORT-NAME-030`): `Csörsz-árok` before `utca`.
///
/// Rules: `ORT-NAME-030`
#[must_use]
pub fn hyphenated_in_street_name(word: &str, generic: &str) -> bool {
    lists(Role::PublicPlace, generic) && geographic_parts(word).is_some()
}

const EN_DASH: char = '\u{2013}';

/// The members of a hyphenated or en-dashed word, each with the separator before it.
fn name_members(word: &str) -> Vec<(&str, Option<char>)> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut sep = None;
    for (i, c) in word.char_indices() {
        if c == '-' || c == EN_DASH {
            out.push((&word[start..i], sep));
            sep = Some(c);
            start = i + c.len_utf8();
        }
    }
    out.push((&word[start..], sep));
    out
}

/// The non-head members of the head word, each cased on its own: a hyphenated member by what it
/// is, an en-dashed group by any member of it being common.
fn cased_members(inner: &[(&str, Option<char>)]) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < inner.len() {
        let mut end = i + 1;
        while end < inner.len() && inner[end].1 == Some(EN_DASH) {
            end += 1;
        }
        let group = &inner[i..end];
        let lower = group.iter().any(|(m, _)| is_common_word(m));
        for (m, sep) in group {
            out.extend(*sep);
            if lower {
                out.push_str(&lower_initial(m));
            } else {
                out.push_str(m);
            }
        }
        i = end;
    }
    out
}

/// The derivative of the head element alone, lowercase, on its listed entry where it has one.
fn head_derivative(head: &str, entry: &Entry, der: &Der) -> Option<String> {
    let lx = lexicon();
    let lower = lower_initial(head);
    let listed = lx
        .get(&lower)
        .iter()
        .chain(lx.get(head))
        .find(|e| der.input.contains(&e.pos))
        .cloned();
    let base = listed.unwrap_or_else(|| {
        let mut e = Entry::new(head, entry.pos);
        e.traits = entry.traits;
        e
    });
    derived(&base, der).ok().map(|e| lower_initial(&e.lemma))
}

/// Whether a space-separated word of a name is a common word: a simplex one by the lexicon, a
/// hyphenated one by its last member (`Abaúj-Hegyközi` is, `Dél-Dunántúl` is not).
fn word_is_common(word: &str) -> bool {
    name_members(word)
        .last()
        .is_some_and(|(m, _)| is_common_word(m))
}

/// Whether `text` names a common word, which the lexicon answers by listing it in lower case.
///
/// A proper name is listed capitalised and nothing else, so the lowercased form finds nothing:
/// `arany` and `sólyom` are entries, `csepel` and `balaton` are not.
///
/// Rules: `ORT-NAME-022`, `ORT-NAME-028`
fn is_common_word(text: &str) -> bool {
    // An element that is itself the `-i` adjective of a name is a common word, not a name:
    // `Kijevi Nagyfejedelemség` gives `kijevi nagyfejedelemségi` and `Gyergyói-medence`
    // `gyergyói-medencei`. So is a compound of common words (`városkörnyék`). A lowercased name
    // that happens to be well formed is neither, which is why the test is a reading and not a
    // shape.
    static COMMON: std::sync::LazyLock<crate::analysis::Stacks> = std::sync::LazyLock::new(|| {
        let mut stacks = vec![vec![]];
        stacks.extend(DerId::ALL.map(|id| vec![crate::Morph::Derive(id)]));
        crate::analysis::Stacks::from(stacks)
    });
    let lower = lower_initial(text);
    !lexicon().get(&lower).is_empty()
        || matches!(
            crate::analysis::analyse(&lower, &COMMON),
            crate::analysis::Verdict::Known(_)
        )
}

/// Whether the suffix wrote no harmonic vowel, so the base's harmony passes through it.
///
/// `DER-MODEL-009`: a harmonic suffix fixes harmony once it has picked an alternant, and after
/// that nothing vacillates (`agilisság` gives `agilisságnak` only). `DER-MODEL-008`'s `N` class
/// picks nothing, so it is transparent and the base's vacillation survives (`karibinak ~
/// karibinek`), and a type `O` suffix after a vowel-final stem writes no linking vowel either, so
/// it picks nothing either (`martini` gives `martinizza ~ martinizi`).
///
/// Rules: `DER-MODEL-008`, `DER-MODEL-009`
fn fixes_no_harmony(der: &Der, stem: &str, form: &str) -> bool {
    if !der.template.iter().any(|s| matches!(s, Seg::Var(_))) {
        return true;
    }
    let tail = form.get(stem.len()..).unwrap_or_default();
    !letters(tail).any(|s| s.letter.is_vowel())
}

/// The stem with the `(j)` of `-(j)Ú` in place.
///
/// The `j` appears only after a vowel, exactly as in the possessive, and the stem lengthens first,
/// so `alma` gives `almájú` rather than `*almajú`: once the `j` is in place the stem no longer
/// ends in a vowel and `attach` would not lengthen it. `DER-ADJ-008` puts `-jú/-jű` on the
/// possessive stem, so the `ajtó` class's possessive-only vowel change applies and supplies the
/// `j` itself (`idő` gives `idejű`, `tető` `tetejű`), and that alternant does not lengthen.
///
/// Rules: `DER-ADJ-008`, `NOM-STEM-013`
fn linking_j(entry: &Entry, der: &Der, stem: String) -> String {
    if !der.j_after_vowel {
        return stem;
    }
    if let Some(alt) = entry
        .has(Trait::PossessiveAlt)
        .then(|| crate::alternation::possessive_alt(&entry.lemma))
        .flatten()
    {
        return format!("{alt}j");
    }
    if letters(&stem).last().is_some_and(|s| s.letter.is_vowel()) {
        return format!("{}j", crate::generate::lengthen_final(&stem));
    }
    stem
}

/// `traits` plus the base's harmony traits, which a transparent suffix passes on.
///
/// Rules: `DER-MODEL-008`, `DER-MODEL-009`
fn carried_harmony(entry: &Entry, traits: TraitSet) -> TraitSet {
    [
        Trait::Back,
        Trait::Front,
        Trait::FrontRounded,
        Trait::HarmonyOptional,
    ]
    .into_iter()
    .filter(|&t| entry.has(t))
    .fold(traits, TraitSet::with)
}

/// Derives, and returns the result as an entry so it can be inflected or derived again.
///
/// The output's own stem properties are assigned by the suffix, not inherited: `DER-MODEL-010`
/// makes the lowering value a fixed property of the derivational suffix.
///
/// Rules: `DER-MODEL-003`, `DER-MODEL-010`, `DER-MODEL-012`
pub fn derived(entry: &Entry, der: &Der) -> Result<Entry, GenError> {
    // `DER-MODEL-012` lets only a productive suffix derive a new word, because a `P-dom` one needs
    // a domain feature the lexicon does not carry. A `P-dom` suffix whose domain *is* computable
    // from the stem carries a `stem_filter` for it, and then the objection does not apply: `-lAg`
    // is restricted to `-i` adjectives, so it reaches `gazdaságilag` without reaching `házlag`.
    // An unproductive suffix describes words that exist, so it may reproduce a listed word and
    // nothing else: `titok` plus `-Ár` is `titkár` because the lexicon says so, and `asztal` plus
    // `-Ár` is nothing.
    let domain_is_stated =
        der.prod == Prod::PDom && (der.stem_filter.is_some() || der.entry_filter.is_some());
    let listed_only = der.prod != Prod::P && !domain_is_stated;
    if !der.input.contains(&entry.pos) {
        return Err(GenError::NoSuchCell(
            "base category is not in the suffix's input",
        ));
    }
    if let Some(f) = der.entry_filter
        && !f(entry)
    {
        return Err(GenError::NoSuchCell(
            "base lexeme is outside the suffix's domain",
        ));
    }
    // `DER-MODEL-006`: the deverbal `-Ás` and `-Ó` take the `v` stem where the verb has one, the
    // same stem the present participle uses: `alszik` gives `alvás` and `eszik` `evés`.
    if (der.id == AS.id || der.id == O.id)
        && entry.pos == Pos::Verb
        && let Some(v) = deverbal_stem(entry)
    {
        let suffix = der.suffix();
        let form = attach(&v, entry, &suffix, entry.has(Trait::Low));
        return Ok(Entry::new(form, der.output));
    }
    // `VRB-CAUS-002`: an irregular verb's causative is listed, because its stem is not derivable,
    // and its passive is built on it (`vitetik`, `etetik`).
    let passive = der.id == TATIK.id;
    if (der.id == TAT.id || passive)
        && let Some((_, form)) = pairs(Role::IrregularCausative)
            .iter()
            .find(|(lemma, _)| *lemma == entry.lemma)
    {
        refused(entry, der, &entry.lemma)?;
        let form = if passive {
            format!("{form}ik")
        } else {
            form.to_string()
        };
        return Ok(Entry::new(form, der.output));
    }
    let suffix = der.suffix();
    if let Some(form) = compound_name_derivative(entry, der) {
        return Ok(Entry::new(form, der.output));
    }
    let stem = derivation_stem(entry, der, &suffix);
    // `ORT-NAME-020`: a name ending in `i` writes one `i` in its `-i` derivative, `Helsinki`
    // giving `helsinki` and `Tamási` `tamási`.
    let name_in_i = der.id == I.id && starts_upper(&entry.lemma) && stem.ends_with('i');
    if !name_in_i
        && let Some(f) = der.stem_filter
        && !f(&stem)
    {
        return Err(GenError::NoSuchCell(
            "stem does not meet the suffix's condition",
        ));
    }
    refused(entry, der, &stem)?;
    if der.id == AN.id
        && !listed_only
        && let Some(form) = participial_adverb(&stem)
    {
        return Ok(Entry::new(form, der.output));
    }
    let stem = linking_j(entry, der, stem);
    // `DER-ADJ-005` puts every adjective base in its lowering class whatever the adjective itself
    // does, so `vörös` gives `vöröses` and `szegedi` `szegedies`.
    let lowering =
        entry.has(Trait::Low) || (entry.pos == Pos::Adjective && der.link == Link::Plural);
    let form = if name_in_i {
        stem.clone()
    } else {
        attach(&stem, entry, &suffix, lowering)
    };
    let transparent = fixes_no_harmony(der, &stem, &form);
    let form = lowercase_name_derivative(&form, entry, der);
    if listed_only {
        return lexicon()
            .get_pos(&form, der.output)
            .cloned()
            .ok_or(GenError::Unsupported(
                "suffix is not productive, so it may only reproduce a listed word",
            ));
    }
    let mut out = Entry::new(form, der.output);
    if der.out_lowering {
        out.traits = TraitSet::from_iter_of([Trait::Low]);
    }
    if transparent {
        out.traits = carried_harmony(entry, out.traits);
    }
    if der.id == NE.id {
        out.traits = out.traits.with(Trait::PossessiveJa);
        if crate::generate::harmony(&entry.lemma, entry) == Harmony::Back {
            out.traits = out.traits.with(Trait::Back);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod unproductive_tests {
    use picur_morph_lexicon::{Pos, lexicon};

    use super::{AR, ASZ, OL, derived};

    /// `DER-MODEL-012`: an unproductive suffix reproduces a listed word and coins nothing.
    #[test]
    fn an_unproductive_suffix_reproduces_only_listed_words() {
        let lx = lexicon();
        let titok = lx.get_pos("titok", Pos::Noun).expect("listed");
        assert_eq!(
            derived(titok, &AR).map(|e| e.lemma).as_deref(),
            Ok("titkár")
        );
        let lo = lx.get_pos("ló", Pos::Noun).expect("listed");
        assert_eq!(derived(lo, &ASZ).map(|e| e.lemma).as_deref(), Ok("lovász"));
        let kez = lx.get_pos("kéz", Pos::Noun).expect("listed");
        assert_eq!(derived(kez, &OL).map(|e| e.lemma).as_deref(), Ok("kezel"));
        let asztal = lx.get_pos("asztal", Pos::Noun).expect("listed");
        assert!(derived(asztal, &AR).is_err());
        assert!(derived(asztal, &ASZ).is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::{AS, I, OS, PRODUCTIVE, Prod, SAG, derive, derived};
    use picur_morph_lexicon::{Entry, Pos, Trait, TraitSet, lexicon};

    fn word(lemma: &str, pos: Pos, traits: &[Trait]) -> Entry {
        let mut e = Entry::new(lemma, pos);
        e.traits = TraitSet::from_iter_of(traits.iter().copied());
        e
    }

    #[test]
    fn sag_on_adjectives() {
        // DER-NOUN-001, the rule's own examples.
        for (base, want) in [
            ("jó", "jóság"),
            ("szép", "szépség"),
            ("hosszú", "hosszúság"),
            ("széles", "szélesség"),
            ("beteg", "betegség"),
            ("nehéz", "nehézség"),
            ("bátor", "bátorság"),
            ("hasonló", "hasonlóság"),
            ("figyelmes", "figyelmesség"),
            ("elszigetelt", "elszigeteltség"),
        ] {
            let e = word(base, Pos::Adjective, &[]);
            assert_eq!(derive(&e, &SAG).expect(base), want, "{base}");
        }
    }

    #[test]
    fn sag_does_not_lengthen_a_final_a_or_e() {
        // DER-MODEL-005: `-sÁg` is in the non-lengthening class.
        assert_eq!(
            derive(&word("lusta", Pos::Adjective, &[]), &SAG).expect("lusta"),
            "lustaság"
        );
        assert_eq!(
            derive(&word("hülye", Pos::Adjective, &[]), &SAG).expect("hülye"),
            "hülyeség"
        );
    }

    #[test]
    fn as_on_verbs() {
        // DER-NOUN-006. The suffix carries its own vowel, so it appears after every consonant.
        for (base, want) in [
            ("olvas", "olvasás"),
            ("beszélget", "beszélgetés"),
            ("klónoz", "klónozás"),
            ("megold", "megoldás"),
        ] {
            let e = word(base, Pos::Verb, &[]);
            assert_eq!(derive(&e, &AS).expect(base), want, "{base}");
        }
        assert_eq!(
            derive(&word("ugrik", Pos::Verb, &[]), &AS).expect("ugrik"),
            "ugrás"
        );
    }

    #[test]
    fn as_selects_the_epenthetic_stem() {
        // DER-MODEL-006: `mozog` derives on `mozg-`, not on the lemma.
        assert_eq!(
            derive(&word("mozog", Pos::Verb, &[Trait::Epenthetic]), &AS).expect("mozog"),
            "mozgás"
        );
        assert_eq!(
            derive(&word("érez", Pos::Verb, &[Trait::Epenthetic]), &AS).expect("érez"),
            "érzés"
        );
    }

    #[test]
    fn os_takes_the_plural_linking_vowel() {
        // DER-MODEL-004, type `O`: a lowering stem gets the low vowel, others the mid one.
        assert_eq!(
            derive(&word("ház", Pos::Noun, &[Trait::Low]), &OS).expect("ház"),
            "házas"
        );
        assert_eq!(
            derive(&word("gáz", Pos::Noun, &[]), &OS).expect("gáz"),
            "gázos"
        );
        assert_eq!(
            derive(&word("alma", Pos::Noun, &[]), &OS).expect("alma"),
            "almás"
        );
        assert_eq!(derive(&word("só", Pos::Noun, &[]), &OS).expect("só"), "sós");
    }

    #[test]
    fn relational_i_is_invariant_and_does_not_lengthen() {
        // DER-ADJ-001, DER-MODEL-005.
        assert_eq!(
            derive(&word("ház", Pos::Noun, &[Trait::Low]), &I).expect("ház"),
            "házi"
        );
        // ORT-NAME-022 case (a): the adjective derived from a one-word name is lowercase, and the
        // rulebook's own examples are `Buda` -> `budai`, `Budapest` -> `budapesti`.
        assert_eq!(
            derive(&word("Buda", Pos::ProperNoun, &[]), &I).expect("Buda"),
            "budai"
        );
        // ORT-NAME-022 case (b): a multiword name keeps its capitals and the suffix follows a
        // hyphen, which is `DER-ADJ-004`'s own output line.
        assert_eq!(
            derive(&word("New York", Pos::ProperNoun, &[]), &I).expect("New York"),
            "New York-i"
        );
        // A name whose last member is a geographic common noun lowercases the first part only
        // when that part is a common word: `arany` is listed, `Csepel` is not.
        assert_eq!(
            derive(&word("Arany-patak", Pos::ProperNoun, &[]), &I).expect("Arany-patak"),
            "arany-pataki"
        );
        assert_eq!(
            derive(&word("Csepel-sziget", Pos::ProperNoun, &[]), &I).expect("Csepel-sziget"),
            "Csepel-szigeti"
        );
        // ORT-NAME-023: a name all through, so every member lowercases.
        assert_eq!(
            derive(&word("Dél-Kína", Pos::ProperNoun, &[]), &I).expect("Dél-Kína"),
            "dél-kínai"
        );
    }

    #[test]
    fn a_derivative_can_be_inflected_again() {
        // DER-MODEL-003: derivation precedes inflection, so the output is an ordinary stem.
        use crate::features::{Case, Morph};
        use crate::generate::generate;
        let e = word("szép", Pos::Adjective, &[]);
        let n = derived(&e, &SAG).expect("szépség");
        assert_eq!(n.lemma.as_ref(), "szépség");
        assert_eq!(n.pos, Pos::Noun);
        // `-sÁg` outputs a non-lowering stem, so the plural takes the mid vowel.
        assert_eq!(generate(&n, &[Morph::Plural]).expect("plural"), "szépségek");
        assert_eq!(
            generate(&n, &[Morph::Case(Case::Ine)]).expect("inessive"),
            "szépségben"
        );
    }

    #[test]
    fn the_input_category_is_checked() {
        let v = word("olvas", Pos::Verb, &[]);
        assert!(derive(&v, &SAG).is_err());
        let a = word("szép", Pos::Adjective, &[]);
        assert!(derive(&a, &AS).is_err());
    }

    #[test]
    fn only_productive_suffixes_generate() {
        // DER-MODEL-012: only a productive suffix may derive a new word. A `P-dom` one qualifies
        // exactly when its domain is computable from the stem instead of being a semantic feature
        // the lexicon would have to carry, which is what its `stem_filter` states.
        assert!(PRODUCTIVE.iter().all(|d| d.prod == Prod::P
            || (d.prod == Prod::PDom && (d.stem_filter.is_some() || d.entry_filter.is_some()))));
        // And the domain has to bite, or the exemption is a way round the policy.
        for d in PRODUCTIVE.iter().filter(|d| d.prod == Prod::PDom) {
            let haz = Entry::new("ház", Pos::Noun);
            let refuses = d.stem_filter.is_some_and(|f| !f("ház"))
                || d.entry_filter.is_some_and(|f| !f(&haz));
            assert!(refuses, "{} accepts any base", d.id);
        }
    }

    #[test]
    fn sag_on_a_noun_needs_the_occupational_domain() {
        // DER-NOUN-003: `P-dom` over a trait the lexicon carries, so the rule's own default
        // applies to every other noun.
        use super::{SAG_N, Trait};
        let lx = lexicon();
        let menedzser = lx.get_pos("menedzser", Pos::Noun).expect("listed");
        assert!(menedzser.has(Trait::Occupation));
        assert_eq!(derive(menedzser, &SAG_N).as_deref(), Ok("menedzserség"));
        let szek = lx.get_pos("szék", Pos::Noun).expect("listed");
        assert!(derive(szek, &SAG_N).is_err());
    }

    #[test]
    fn a_transparent_suffix_passes_the_base_harmony_on() {
        // DER-MODEL-009: `-i` picks no alternant, so a vacillating base goes on vacillating;
        // `-sÁg` picks one, so the derivative is settled.
        use super::Trait;
        let lx = lexicon();
        let karib = lx.get_pos("karib", Pos::Noun).expect("listed");
        assert!(karib.has(Trait::HarmonyOptional));
        let karibi = derived(karib, &I).expect("karibi");
        assert!(karibi.has(Trait::HarmonyOptional));
        let agilis = lx.get_pos("agilis", Pos::Adjective).expect("listed");
        assert!(
            !derived(agilis, &SAG)
                .expect("agilisség")
                .has(Trait::HarmonyOptional)
        );
    }

    #[test]
    fn derives_from_the_committed_lexicon() {
        let lx = lexicon();
        let szep = lx.get_pos("szép", Pos::Adjective).expect("szép");
        assert_eq!(derive(szep, &SAG).expect("szépség"), "szépség");
        let olvas = lx.get_pos("olvas", Pos::Verb).expect("olvas");
        assert_eq!(derive(olvas, &AS).expect("olvasás"), "olvasás");
    }
}

#[cfg(test)]
mod causative_tests {
    use picur_morph_lexicon::{Pos, lexicon};

    use crate::features::Morph;
    use crate::generate::generate;

    use super::DerId;

    /// The causative of a listed verb.
    fn causative(stem: &str) -> String {
        let lx = lexicon();
        let entry = lx.get_pos(stem, Pos::Verb).expect("a listed verb");
        generate(entry, &[Morph::Derive(DerId::Causative)]).expect("generates")
    }

    /// Every example the rule states, in the order it states them.
    #[test]
    fn vrb_caus_001_selects_the_allomorph_by_stem_shape() {
        for (stem, want) in [
            ("olvas", "olvastat"),
            ("dolgozik", "dolgoztat"),
            ("énekel", "énekeltet"),
            ("tanít", "taníttat"),
            ("épít", "építtet"),
            ("arat", "arattat"),
            ("fut", "futtat"),
            ("köt", "köttet"),
            ("lát", "láttat"),
            ("nyit", "nyittat"),
            ("tart", "tartat"),
            ("fest", "festet"),
            ("önt", "öntet"),
            ("olt", "oltat"),
            ("ír", "írat"),
            ("vár", "várat"),
            ("kér", "kéret"),
            ("mos", "mosat"),
            ("hoz", "hozat"),
            ("vág", "vágat"),
        ] {
            let lx = lexicon();
            let entry = lx.get_pos(stem, Pos::Verb).expect("a listed verb");
            let got = generate(entry, &[Morph::Derive(DerId::Causative)]).expect("generates");
            assert_eq!(got, want, "{stem}");
        }
    }

    /// The closed minority the shape rule gets wrong, which the rule names as lexical.
    #[test]
    fn vrb_caus_001_the_lexical_tat_monosyllables() {
        for (stem, want) in [
            ("ül", "ültet"),
            ("él", "éltet"),
            ("nyom", "nyomtat"),
            ("foly", "folytat"),
            ("un", "untat"),
            ("lép", "léptet"),
            ("kel", "keltet"),
            ("von", "vontat"),
        ] {
            assert_eq!(causative(stem), want, "{stem}");
        }
    }

    /// `VRB-CAUS-002`: built on a lexical stem that nothing in the verb predicts.
    #[test]
    fn vrb_caus_002_irregular_verbs_have_listed_causatives() {
        for (stem, want) in [
            ("eszik", "etet"),
            ("iszik", "itat"),
            ("tesz", "tetet"),
            ("vesz", "vetet"),
            ("visz", "vitet"),
            ("alszik", "altat"),
            ("fekszik", "fektet"),
            ("nyugszik", "nyugtat"),
            ("lő", "lövet"),
            // Regular by the shape rule, so they need no entry in the table.
            ("lát", "láttat"),
            ("hall", "hallat"),
        ] {
            assert_eq!(causative(stem), want, "{stem}");
        }
    }
}

#[cfg(test)]
mod deverbal_stem_tests {
    use picur_morph_lexicon::{Pos, lexicon};

    use crate::features::Morph;
    use crate::generate::generate;

    use super::DerId;

    /// `DER-MODEL-006`: `-Ás` takes the bound stem, whichever of the three the verb has.
    #[test]
    fn der_model_006_the_deverbal_noun_uses_the_bound_stem() {
        for (lemma, want) in [
            ("alszik", "alvás"),
            ("igyekszik", "igyekvés"),
            ("eszik", "evés"),
            ("tesz", "tevés"),
            ("emlékszik", "emlékezés"),
            ("söpör", "söprés"),
            ("olvas", "olvasás"),
            ("dolgozik", "dolgozás"),
        ] {
            let lx = lexicon();
            let entry = lx.get_pos(lemma, Pos::Verb).expect("a listed verb");
            let got = generate(entry, &[Morph::Derive(DerId::As)]).expect("generates");
            assert_eq!(got, want, "{lemma}");
        }
    }
}

#[cfg(test)]
mod derivation_logic_tests {
    use picur_morph_lexicon::{Pos, lexicon};

    use crate::features::Morph;
    use crate::generate::generate;

    use super::DerId;

    fn form(lemma: &str, pos: Pos, stack: &[Morph]) -> Option<String> {
        let lx = lexicon();
        generate(lx.get_pos(lemma, pos).expect("a listed word"), stack).ok()
    }

    /// `DER-VERB-020`: the passive is the causative plus `-ik`, on transitive verbs only.
    #[test]
    fn der_verb_020_the_passive_follows_the_causative() {
        let passive = [Morph::Derive(DerId::Passive)];
        for (lemma, want) in [
            ("ad", "adatik"),
            ("olt", "oltatik"),
            ("kinyit", "kinyittatik"),
            ("foglal", "foglaltatik"),
        ] {
            assert_eq!(form(lemma, Pos::Verb, &passive).as_deref(), Some(want));
        }
        assert_eq!(form("alszik", Pos::Verb, &passive), None);
    }

    /// `DER-VERB-012`: `-kOdik` on an `-(O)s` or privative adjective, and a noun's `-(O)skOdik`.
    #[test]
    fn der_verb_012_kodik_takes_os_and_privative_adjectives() {
        let kodik = [Morph::Derive(DerId::Kodik)];
        assert_eq!(
            form("nyugtalan", Pos::Adjective, &kodik).as_deref(),
            Some("nyugtalankodik")
        );
        assert_eq!(form("gyors", Pos::Adjective, &kodik), None);
        let skodik = [Morph::Derive(DerId::Os), Morph::Derive(DerId::Kodik)];
        assert_eq!(
            form("katona", Pos::Noun, &skodik).as_deref(),
            Some("katonáskodik")
        );
        assert_eq!(form("okos", Pos::Adjective, &skodik), None);
    }

    /// `DER-ADV-004`: `-lAg` on a present participle.
    #[test]
    fn der_adv_004_lag_takes_a_present_participle() {
        let lag = [Morph::PresentParticiple, Morph::Derive(DerId::Lag)];
        assert_eq!(
            form("hallgat", Pos::Verb, &lag).as_deref(),
            Some("hallgatólag")
        );
    }

    /// `DER-ADJ-006`, `DER-ADV-010`: the base's own derivation refuses `-(O)s`.
    #[test]
    fn der_adj_006_the_base_derivation_decides_os() {
        let os = [Morph::Derive(DerId::Os)];
        assert_eq!(form("pihenési", Pos::Adjective, &os), None);
        assert_eq!(form("stabilitás", Pos::Noun, &os), None);
        assert_eq!(form("szépen", Pos::Adjective, &os), None);
        assert!(form("tanulás", Pos::Noun, &os).is_some());
        let ias = [Morph::Derive(DerId::I), Morph::Derive(DerId::Os)];
        assert_eq!(form("Kossuth", Pos::Noun, &ias), None);
        assert_eq!(form("Petőfi", Pos::Noun, &ias).as_deref(), Some("petőfies"));
    }
}
