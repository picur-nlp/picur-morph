//! Verb inflection.
//!
//! See `docs/grammar/05-verbal.md`.

use std::sync::LazyLock;

use picur_morph_lexicon::{Entry, Pos, Role, Trait, lexicon, words};
use picur_morph_ortho::{Harmony, Letter, letters, syllable_count};

use crate::memo::Memo;

use crate::features::{Definite, Number, Person};
use crate::stem::{Coda, coda};
use crate::suffix::{Arch, Seg, Suffix, SuffixClass};

/// The present stem of a verb lemma.
///
/// morphdb lemmatises `-ik` verbs with the ending attached (`dolgozik`), but the suffixes attach to
/// the bare stem, so it is removed here.
///
/// Rules: `VRB-IK-001`
#[must_use]
pub fn present_stem(lemma: &str) -> &str {
    match lemma.strip_suffix("ik") {
        Some(rest) if rest.chars().count() >= 2 => rest,
        _ => lemma,
    }
}

/// The verb a preverb is written in front of, where the lemma is such a form.
///
/// `ORT-TOG-026`: a preverb is written onto the verb and does not change how it inflects, so
/// whatever the lexicon records about the base holds of the prefixed form too. Both halves must be
/// listed, and the prefix must be a preverb rather than any string that happens to split, which is
/// what stops an ordinary verb coming apart at a coincidence. The longest preverb wins, so
/// `belenéz` is `bele` and `néz` rather than `be` and `lenéz`. A verb takes one preverb, so a
/// base that is itself prefixed is no base (`elébeáll` is not `elé` on `beáll`) unless the
/// preverb is one the lexicon marks as standing before another (`újraeloszt`).
#[must_use]
pub fn preverb_base(lemma: &str) -> Option<&str> {
    // The answer depends only on the lemma, and scanning every boundary against the lexicon sits
    // under each verb form the checker builds. Only the split point is memoised, so the borrow
    // still comes from the caller's string.
    static MEMO: LazyLock<Memo<Option<usize>>> = LazyLock::new(Memo::default);
    MEMO.get_or_compute(lemma, || preverb_split(lemma))
        .map(|at| &lemma[at..])
}

/// The byte the base starts at, which [`preverb_base`] memoises.
fn preverb_split(lemma: &str) -> Option<usize> {
    let lx = lexicon();
    lemma.char_indices().rev().map(|(at, _)| at).find(|&at| {
        at > 0
            && lx.has_pos(&lemma[..at], Pos::Preverb)
            && lx.has_pos(&lemma[at..], Pos::Verb)
            && (stacking(&lemma[..at]) || !prefixed(&lemma[at..]))
    })
}

/// Whether `preverb` may stand in front of a verb that already carries one (`Trait::Stacking`).
#[must_use]
pub fn stacking(preverb: &str) -> bool {
    lexicon()
        .get_pos(preverb, Pos::Preverb)
        .is_some_and(|e| e.has(Trait::Stacking))
}

/// Whether the verb `lemma` already carries a preverb.
///
/// The lexicon records this on its own verbs (`Trait::Preverb`), which is what keeps `felejt` from
/// counting as `fel` on `ejt`; an unlisted verb is prefixed if it splits.
pub fn prefixed(lemma: &str) -> bool {
    lexicon()
        .get_pos(lemma, Pos::Verb)
        .map_or_else(|| preverb_split(lemma).is_some(), |e| e.has(Trait::Preverb))
}

/// Whether the verb carries `flag`, on its own entry or on the base behind a preverb.
fn carries(entry: &Entry, flag: Trait) -> bool {
    entry.has(flag)
        || preverb_base(&entry.lemma)
            .and_then(|base| lexicon().get_pos(base, Pos::Verb))
            .is_some_and(|e| e.has(flag))
}

/// The unstable vowels an sz-d verb's `d` or `z` stem is written with (`VRB-STEM-012`).
///
/// The rule states them: "stem vowel `o/e/ö/u/ü` + `d`", and the `z` stem beside it. Which one a
/// given verb takes is not predictable (`alud` against `nyugod`, `haragud` against `gyarapod`),
/// so the stem is not computed but *recognised*, by asking the lexicon.
const SZ_D_ENDINGS: &[&str] = &["od", "ed", "öd", "ud", "üd", "oz", "ez", "öz"];

/// The `d` or `z` stem of an sz-d verb, recovered from its doublet (`VRB-STEM-012`).
///
/// The class is closed and lexical, but it needs no list here, because the rule says the doublet
/// "is a full regular verb": `öregszik` alternates with `öregedik`, and that verb is in the
/// lexicon. So the stem is found by trying the endings the rule names and keeping the one whose
/// `-ik` verb is listed. A prefixed member inherits from its base the same way, `megharagszik`
/// from `haragszik`.
///
/// Built on the `sz` stem instead, the `d`-stem paradigm comes out as `öregszett` and `öregszeni`
/// for `öregedett` and `öregedni`.
#[must_use]
pub fn sz_d_stem(lemma: &str) -> Option<String> {
    // Up to fifteen lexicon lookups sit under every verb form the checker builds; the answer
    // depends only on the lemma and the verbs are finite.
    static MEMO: LazyLock<Memo<Option<Box<str>>>> = LazyLock::new(Memo::default);
    MEMO.get_or_compute(lemma, || sz_d_stem_uncached(lemma).map(Into::into))
        .map(Into::into)
}

/// The lookup [`sz_d_stem`] memoises.
fn sz_d_stem_uncached(lemma: &str) -> Option<String> {
    let root = lemma
        .strip_suffix("szik")
        .or_else(|| lemma.strip_suffix("sz"))?;
    let lx = lexicon();
    if let Some(stem) = SZ_D_ENDINGS
        .iter()
        .map(|v| format!("{root}{v}"))
        .find(|stem| lx.has_pos(&format!("{stem}ik"), Pos::Verb))
    {
        return Some(stem);
    }
    let base = preverb_base(lemma)?;
    let alt = lx
        .get_pos(base, Pos::Verb)
        .and_then(|e| e.alt.as_deref())
        .map(ToOwned::to_owned)
        .or_else(|| sz_d_stem(base))?;
    Some(format!("{}{alt}", &lemma[..lemma.len() - base.len()]))
}

/// The stem alternant a verb inflects on outside the present, where it has one.
///
/// The lexicon's own alternant where it records one, else the `d`/`z` stem of an sz-d verb
/// (`VRB-STEM-012`). `None` for a verb whose paradigm is built on its present stem throughout.
#[must_use]
pub fn alternant_stem(entry: &Entry) -> Option<String> {
    entry
        .alt
        .as_deref()
        .map(ToOwned::to_owned)
        .or_else(|| sz_d_stem(&entry.lemma))
}

/// The stem a verb uses outside the present indicative.
///
/// Rules: `VRB-STEM-012`
#[must_use]
pub fn bound_stem(entry: &Entry) -> String {
    alternant_stem(entry).unwrap_or_else(|| present_stem(&entry.lemma).to_owned())
}

/// The full stem of an epenthetic `-ik` verb whose lemma shows the syncopated one.
///
/// `VRB-STEM-008`: `ugrik`, `fürdik` and `romlik` are cited with the cluster, and the vowel
/// returns before an ending that neither begins with a vowel nor takes a linking one: `ugorhat`,
/// `ugorjon`, `ugorva`, `fürödhet`, `romoljon`. The vowel is the harmony's mid vowel. `None` for
/// any other verb.
///
/// Rules: `VRB-STEM-008`
#[must_use]
pub fn full_stem(entry: &Entry) -> Option<String> {
    if !entry.has(Trait::Epenthetic) || !entry.lemma.ends_with("ik") {
        return None;
    }
    let stem = present_stem(&entry.lemma);
    let spans: Vec<_> = letters(stem).collect();
    let [.., a, b] = spans.as_slice() else {
        return None;
    };
    if !a.letter.is_consonant() || !b.letter.is_consonant() {
        return None;
    }
    let vowel = match crate::generate::harmony(stem, entry) {
        Harmony::Back => "o",
        Harmony::FrontRounded => "ö",
        Harmony::Front => "e",
    };
    Some(format!("{}{vowel}{}", &stem[..b.at], &stem[b.at..]))
}

/// The stem a consonant-initial ending without a linking vowel attaches to.
///
/// Rules: `VRB-STEM-008`, `VRB-STEM-012`
#[must_use]
pub fn unlinked_stem(entry: &Entry) -> String {
    full_stem(entry).unwrap_or_else(|| bound_stem(entry))
}

/// The `v` stem of an sz-d verb, which the present participle and `-Ás` are built on.
///
/// `VRB-STEM-012` restricts it to the members with an attested `v` participle, and
/// `DER-MODEL-006` names `-Ó` and `-Ás` as the suffixes that take it: `alszik` gives `alvó` and
/// `alvás`, not `*aludó` and `*alszás`.
#[must_use]
pub fn v_stem(lemma: &str) -> Option<String> {
    let base = words(Role::VStemVerb)
        .iter()
        .find(|v| lemma.ends_with(v.as_ref()))?;
    let prefix = &lemma[..lemma.len() - base.len()];
    let root = base.strip_suffix("szik")?;
    Some(format!("{prefix}{root}v"))
}

/// The short stem of an sz-d verb, its `sz` stem without the `sz`.
///
/// `VRB-STEM-012` builds the potential and the converb of the listed members on it beside the
/// `d` stem: `alhat ~ aludhat`, `fekhet ~ feküdhet`, `nyughat`, `alva ~ aludva`. A prefixed
/// member is matched by ending, as for the `v` stem.
///
/// Rules: `VRB-STEM-012`
#[must_use]
pub fn short_stem(lemma: &str) -> Option<String> {
    let base = words(Role::ShortStemVerb)
        .iter()
        .find(|v| lemma.ends_with(v.as_ref()))?;
    let prefix = &lemma[..lemma.len() - base.len()];
    let root = base.strip_suffix("szik")?;
    Some(format!("{prefix}{root}"))
}

/// Whether a stem is heavy, which is what makes the self-lowering suffixes take a linking vowel.
///
/// Computable part of `VRB-LINK-002`: a stem is heavy when it ends in two **different** consonants,
/// in `-ít`, or in a long `dz` (`long_affricate`). The rest of the rule is lexical and goes both
/// ways, which is why it cannot be derived: `hall` is heavy but `áll` is not, though both end in a
/// geminate `ll`.
///
/// Rules: `VRB-LINK-002`
#[must_use]
pub fn heavy(stem: &str) -> bool {
    if stem.ends_with("ít") || long_affricate(stem) {
        return true;
    }
    matches!(coda(stem), Coda::Cluster(_))
}

/// Whether a monosyllabic stem ends in vowel + `dz`, which is pronounced long and so patterns as a
/// geminate: `edzeni`, `edzettem`, `pedzettem`.
///
/// The polysyllabic `-dz` stems carry the reflexive suffix `-(Ó)dz-`, whose `dz` is short:
/// `lopództam`, `lopódzni`.
///
/// Rules: `VRB-LINK-002`, `VRB-PAST-005`
#[must_use]
fn long_affricate(stem: &str) -> bool {
    matches!(coda(stem), Coda::Single(Letter::Dz)) && syllable_count(stem) == 1
}

/// As [`heavy`], but consulting the lexical value a geminate stem carries.
///
/// `VRB-LINK-002` makes the geminates lexical in both directions, so the shape alone cannot decide
/// between `hallani` and `állni`. A stem measured both ways (`VRB-LINK-010`) is exactly as heavy as
/// its flag says, which is how `mondotok` and `mondtok` are both reached.
///
/// Rules: `VRB-LINK-002`, `VRB-LINK-010`
#[must_use]
pub fn heavy_entry(stem: &str, entry: &Entry) -> bool {
    if entry.has(Trait::LinkOptional) {
        return entry.has(Trait::HeavyStem);
    }
    entry.has(Trait::HeavyStem) || heavy(stem)
}

/// Whether a stem ends in a sibilant, which takes the `-Ol` second person singular.
///
/// Rules: `VRB-LINK-003`
#[must_use]
fn sibilant_final(stem: &str) -> bool {
    letters(stem).last().is_some_and(|s| {
        matches!(
            s.letter,
            Letter::S | Letter::Sz | Letter::Z | Letter::Dz | Letter::Zs
        )
    })
}

const fn one(template: &'static [Seg]) -> Suffix {
    Suffix {
        template,
        class: SuffixClass::One,
        lengthens: false,
        lowerable: false,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: false,
    }
}

const fn two(template: &'static [Seg]) -> Suffix {
    Suffix {
        template,
        class: SuffixClass::Two,
        lengthens: false,
        lowerable: false,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: false,
    }
}

/// The present indicative indefinite ending, or `None` for the zero-marked third singular.
///
/// Rules: `VRB-PRES-001`, `VRB-LINK-003`, `VRB-LINK-004`, `VRB-LINK-005`
#[must_use]
pub fn present_indefinite(
    person: Person,
    number: Number,
    stem: &str,
    entry: &Entry,
) -> Option<Suffix> {
    use Arch::{A, O, U};
    use Seg::{Lit, Var};
    let h = heavy_entry(stem, entry);
    Some(match (person, number) {
        // VRB-IK-002: an `-ik` verb writes `-Om` in the standard, `eszem` and `dolgozom`; the
        // colloquial `-Ok` is read as a variant.
        (Person::First, Number::Sing) if is_ik(entry) => one(&[Var(O), Lit("m")]),
        (Person::First, Number::Sing) => one(&[Var(O), Lit("k")]),
        (Person::Second, Number::Sing) => {
            if sibilant_final(stem) {
                one(&[Var(O), Lit("l")])
            } else if h {
                one(&[Var(A), Lit("sz")])
            } else {
                two(&[Lit("sz")])
            }
        }
        (Person::Third, Number::Sing) => return None,
        (Person::First, Number::Plur) => one(&[Var(U), Lit("nk")]),
        (Person::Second, Number::Plur) => {
            if h {
                one(&[Var(O), Lit("t"), Var(O), Lit("k")])
            } else {
                two(&[Lit("t"), Var(O), Lit("k")])
            }
        }
        (Person::Third, Number::Plur) => {
            if h {
                one(&[Var(A), Lit("n"), Var(A), Lit("k")])
            } else {
                two(&[Lit("n"), Var(A), Lit("k")])
            }
        }
    })
}

/// The present indicative **definite** ending.
///
/// The paradigm splits on backness rather than on the three-way harmony: back stems take the `j`
/// forms throughout (`várja`, `várjátok`, `várják`) while front stems take `-i` forms in the third
/// person and second plural (`kéri`, `kéritek`, `kérik`).
///
/// Rules: `VRB-PRES-002`
#[must_use]
pub fn present_definite(person: Person, number: Number, back: bool) -> Suffix {
    use Arch::{A, AAcute, O, U};
    use Seg::{AssimJ, Lit, Var};
    match (person, number) {
        (Person::First, Number::Sing) => one(&[Var(O), Lit("m")]),
        (Person::Second, Number::Sing) => one(&[Var(O), Lit("d")]),
        (Person::Third, Number::Sing) => {
            if back {
                two(&[AssimJ, Var(A)])
            } else {
                two(&[Lit("i")])
            }
        }
        (Person::First, Number::Plur) => two(&[AssimJ, Var(U), Lit("k")]),
        (Person::Second, Number::Plur) => {
            if back {
                two(&[AssimJ, Var(AAcute), Lit("t"), Var(O), Lit("k")])
            } else {
                two(&[Lit("i"), Lit("t"), Var(O), Lit("k")])
            }
        }
        (Person::Third, Number::Plur) => {
            if back {
                two(&[AssimJ, Var(AAcute), Lit("k")])
            } else {
                two(&[Lit("i"), Lit("k")])
            }
        }
    }
}

/// The formal third-singular imperative of an `-ik` verb, `-jék` (`VRB-IK-005`).
///
/// Analysis reads it beside the neutral `-jOn` generation returns; here so the ending index can
/// key it.
pub const IK_FORMAL_THIRD: Suffix = two(&[Seg::Lit("j"), Seg::Lit("ék")]);

/// The `-lAk` / `-AlAk` ending: first singular subject acting on a second person object.
///
/// Present tense only. The past, conditional and imperative all place it after a vowel-final marker,
/// where the linking vowel is part of the marker's own template and this selection does not apply.
///
/// Rules: `VRB-PRES-005`, `VRB-LINK-008`
#[must_use]
pub fn present_lak(stem: &str, entry: &Entry) -> Suffix {
    use Arch::A;
    use Seg::{Lit, Var};
    // VRB-LINK-008 states the `ll` stems separately from the heavy ones, and the `ll` clause wins:
    // `hall` is heavy everywhere else (`hallani`) but gives `hallak`, not `*hallalak`
    // (776 against 3 `[WC2]`). The three `l` letters then reduce by `ORT-ASSIM-017`.
    if matches!(coda(stem), Coda::Geminate(Letter::L)) {
        return two(&[Lit("l"), Var(A), Lit("k")]);
    }
    if heavy_entry(stem, entry) {
        one(&[Var(A), Lit("l"), Var(A), Lit("k")])
    } else {
        two(&[Lit("l"), Var(A), Lit("k")])
    }
}

/// The `-AlAk` ending as it appears after the past, conditional and imperative markers.
///
/// Rules: `VRB-PAST-011`, `VRB-COND-002`, `VRB-IMP-001`
#[must_use]
fn bound_lak() -> Suffix {
    use Arch::A;
    use Seg::{Lit, Var};
    two(&[Var(A), Lit("l"), Var(A), Lit("k")])
}

/// Whether a stem is the T5 class: polysyllabic, ending in a short vowel plus a single `t`.
///
/// This is the only t-final class whose past marker is short before a person suffix (`mutattam`,
/// `szerettem`, `olvashattam`). Every other t-final stem takes `-Ott-`, including the monosyllables
/// (`futottam`), the long-vowel stems (`fűtöttem`, `vétettem`) and `-ít` (`tanítottam`).
///
/// `lát` is the single lexical exception, and carries [`Trait::PastNonfinalT`] for it.
///
/// Rules: `VRB-PAST-006`
fn short_vowel_t_polysyllable(stem: &str) -> bool {
    if stem.ends_with("ít") || syllable_count(stem) < 2 {
        return false;
    }
    let (mut prev, mut last) = (None, None);
    for span in letters(stem) {
        prev = last;
        last = Some(span.letter);
    }
    match (last, prev) {
        (Some(Letter::T), Some(v)) => v.is_vowel() && !v.is_long_vowel(),
        _ => false,
    }
}

/// The past marker in word-final position, which is the third singular indefinite.
///
/// `-tt` after a vowel, `-t` after a coronal sonorant, `-Ott` otherwise, unless the verb carries a
/// measured mark. The geminates are lexical by `VRB-PAST-004` and the `-ad`/`-ed` verbs by
/// `VRB-PAST-008`, and neither is computable from the coda: `áll` gives `állt` while `hall` gives
/// `hallott`, and `marad` gives `maradt` while `fogad` gives `fogadott`.
///
/// Rules: `VRB-PAST-001`, `VRB-PAST-002`, `VRB-PAST-004`, `VRB-PAST-005`, `VRB-PAST-008`
#[must_use]
pub fn past_final(stem: &str, entry: &Entry) -> Suffix {
    use Arch::O;
    use Seg::{Lit, Var};
    // `VRB-STEM-012` states the exception on `alszik` and `fekszik`, and `ORT-TOG-026` carries it
    // to the prefixed forms: `átaludt`, not `*átaludott`.
    if carries(entry, Trait::PastFinalT) {
        return two(&[Lit("t")]);
    }
    if carries(entry, Trait::PastFinalVtt) {
        return one(&[Var(O), Lit("tt")]);
    }
    match coda(stem) {
        Coda::Vowel => two(&[Lit("tt")]),
        Coda::Single(c) if coronal_sonorant(c) => two(&[Lit("t")]),
        _ => one(&[Var(O), Lit("tt")]),
    }
}

/// The past marker before a person suffix.
///
/// `-t-` in general, `-Ott-` for the t-final classes and a long `dz`, `-tt` after a vowel.
///
/// Rules: `VRB-PAST-001`, `VRB-PAST-005`
#[must_use]
pub fn past_nonfinal(stem: &str, entry: &Entry) -> Suffix {
    use Arch::O;
    use Seg::{Lit, Var};
    if matches!(coda(stem), Coda::Vowel) {
        return two(&[Lit("tt")]);
    }
    // The measured mark wins on this axis too, and the two axes come apart: `függ` is `függött`
    // word-finally but `függtem` here.
    if entry.has(Trait::PastNonfinalVtt) {
        return one(&[Var(O), Lit("tt")]);
    }
    if entry.has(Trait::PastNonfinalT) {
        return two(&[Lit("t")]);
    }
    let t_final = matches!(
        coda(stem),
        Coda::Single(Letter::T) | Coda::Cluster(Letter::T)
    );
    // VRB-PAST-006: every t-final stem takes the long marker except the T5 class and `lát`.
    if (t_final && !short_vowel_t_polysyllable(stem)) || long_affricate(stem) {
        return one(&[Var(O), Lit("tt")]);
    }
    // VRB-PAST-007: a consonant cluster takes the linking vowel by default (`zajlottam`,
    // `csuklottam`), measured over 410 roots. The `-ng` stems go the other way (`zengtem`) and
    // the `-Cd` members that drop it (`küldtem`, `kezdtem`) are marked, so both are decided above.
    if matches!(coda(stem), Coda::Cluster(c) if c != Letter::G) {
        return one(&[Var(O), Lit("tt")]);
    }
    two(&[Lit("t")])
}

/// Whether a consonant is a coronal sonorant, after which the word-final past is bare `-t`.
const fn coronal_sonorant(c: Letter) -> bool {
    matches!(
        c,
        Letter::L | Letter::R | Letter::N | Letter::Ny | Letter::J | Letter::Ly
    )
}

/// The past person ending, or `None` for the zero-marked third singular indefinite.
///
/// The two conjugations coincide in the first singular (`vártam`) and diverge everywhere else; the
/// first plural is the pair that carries the load, `vártunk` against `vártuk`.
///
/// Rules: `VRB-PAST-011`
#[must_use]
pub fn past_person(person: Person, number: Number, definite: Definite) -> Option<Suffix> {
    use Arch::{A, AAcute, O, U};
    use Seg::{Lit, Var};
    if definite == Definite::Two {
        return Some(bound_lak());
    }
    let def = definite == Definite::Def;
    Some(match (person, number, def) {
        (Person::First, Number::Sing, _) => two(&[Var(A), Lit("m")]),
        (Person::Second, Number::Sing, false) => two(&[Var(AAcute), Lit("l")]),
        (Person::Second, Number::Sing, true) => two(&[Var(A), Lit("d")]),
        (Person::Third, Number::Sing, false) => return None,
        (Person::Third, Number::Sing, true) => two(&[Var(A)]),
        (Person::First, Number::Plur, false) => two(&[Var(U), Lit("nk")]),
        (Person::First, Number::Plur, true) => two(&[Var(U), Lit("k")]),
        (Person::Second, Number::Plur, false) => two(&[Var(A), Lit("t"), Var(O), Lit("k")]),
        (Person::Second, Number::Plur, true) => two(&[Var(AAcute), Lit("t"), Var(O), Lit("k")]),
        (Person::Third, Number::Plur, false) => two(&[Var(A), Lit("k")]),
        (Person::Third, Number::Plur, true) => two(&[Var(AAcute), Lit("k")]),
    })
}

/// The infinitive `-ni` / `-Ani`.
///
/// Rules: `VRB-INF-001`, `VRB-LINK-006`
#[must_use]
pub fn infinitive(stem: &str, entry: &Entry) -> Suffix {
    use Arch::A;
    use Seg::{Lit, Var};
    if heavy_entry(stem, entry) {
        one(&[Var(A), Lit("ni")])
    } else {
        two(&[Lit("ni")])
    }
}

/// The potential marker `-hAt`, which attaches without a linking vowel.
///
/// Two-way harmony: rounded stems take `-het` (`ülhet`), and the `h` is never assimilated in
/// spelling (`adhat`, `nézhet`).
///
/// Rules: `VRB-POT-001`
#[must_use]
pub fn potential() -> Suffix {
    use Arch::A;
    use Seg::{Lit, Var};
    two(&[Lit("h"), Var(A), Lit("t")])
}

/// The conditional marker `-(A)n-`.
///
/// Rules: `VRB-COND-001`
#[must_use]
pub fn conditional_marker(stem: &str, entry: &Entry) -> Suffix {
    use Arch::A;
    use Seg::{Lit, Var};
    if heavy_entry(stem, entry) {
        one(&[Var(A), Lit("n")])
    } else {
        two(&[Lit("n")])
    }
}

/// The present conditional person ending, after the marker.
///
/// The indefinite first singular `-ék` is invariant: `várnék`, `kérnék` and `törnék` all end the
/// same way, which is why it is a literal rather than an archiphoneme. The definite first and second
/// plural coincide with the indefinite, so the two conjugations are distinguished only in four
/// cells.
///
/// Rules: `VRB-COND-001`, `VRB-COND-002`
#[must_use]
pub fn conditional_person(person: Person, number: Number, definite: Definite) -> Suffix {
    use Arch::{A, AAcute, O};
    use Seg::{Lit, Var};
    if definite == Definite::Two {
        return two(&[Var(AAcute), Lit("l"), Var(A), Lit("k")]);
    }
    let def = definite == Definite::Def;
    match (person, number, def) {
        (Person::First, Number::Sing, false) => two(&[Lit("ék")]),
        (Person::First, Number::Sing, true) => two(&[Var(AAcute), Lit("m")]),
        (Person::Second, Number::Sing, false) => two(&[Var(AAcute), Lit("l")]),
        (Person::Second, Number::Sing, true) => two(&[Var(AAcute), Lit("d")]),
        (Person::Third, Number::Sing, false) => two(&[Var(A)]),
        (Person::Third, Number::Sing, true) => two(&[Var(AAcute)]),
        (Person::First, Number::Plur, _) => two(&[Var(AAcute), Lit("nk")]),
        (Person::Second, Number::Plur, _) => two(&[Var(AAcute), Lit("t"), Var(O), Lit("k")]),
        // `várnának`, not `*várnak`: the indefinite third plural has its own long vowel.
        (Person::Third, Number::Plur, false) => two(&[Var(AAcute), Lit("n"), Var(A), Lit("k")]),
        (Person::Third, Number::Plur, true) => two(&[Var(AAcute), Lit("k")]),
    }
}

/// The imperative person ending, which attaches to the fused imperative stem.
///
/// The second singular indefinite is the bare stem (`várj`, `moss`), with a long variant `-jÁl`
/// that this returns as `None` so the caller emits the stem alone.
///
/// Rules: `VRB-IMP-001`
#[must_use]
pub fn imperative_person(person: Person, number: Number, definite: Definite) -> Option<Suffix> {
    use Arch::{A, AAcute, O, U};
    use Seg::{Lit, Var};
    if definite == Definite::Two {
        return Some(bound_lak());
    }
    let definite = definite == Definite::Def;
    Some(match (person, number, definite) {
        (Person::First, Number::Sing, false) => two(&[Var(A), Lit("k")]),
        (Person::First, Number::Sing, true) => two(&[Var(A), Lit("m")]),
        (Person::Second, Number::Sing, false) => return None,
        (Person::Second, Number::Sing, true) => two(&[Var(A), Lit("d")]),
        (Person::Third, Number::Sing, false) => two(&[Var(O), Lit("n")]),
        (Person::Third, Number::Sing, true) => two(&[Var(A)]),
        (Person::First, Number::Plur, false) => two(&[Var(U), Lit("nk")]),
        (Person::First, Number::Plur, true) => two(&[Var(U), Lit("k")]),
        (Person::Second, Number::Plur, false) => two(&[Var(A), Lit("t"), Var(O), Lit("k")]),
        (Person::Second, Number::Plur, true) => two(&[Var(AAcute), Lit("t"), Var(O), Lit("k")]),
        (Person::Third, Number::Plur, false) => two(&[Var(A), Lit("n"), Var(A), Lit("k")]),
        (Person::Third, Number::Plur, true) => two(&[Var(AAcute), Lit("k")]),
    })
}

/// The present participle `-Ó`.
///
/// Rules: `VRB-PART-001`
#[must_use]
pub fn present_participle() -> Suffix {
    use Arch::OAcute;
    use Seg::Var;
    two(&[Var(OAcute)])
}

/// The past participle `-t` / `-Ott`, which is the word-final past marker.
///
/// Rules: `VRB-PART-004`
#[must_use]
pub fn past_participle(stem: &str, entry: &Entry) -> Suffix {
    past_final(stem, entry)
}

/// The future participle `-AndÓ`, 'to be V-ed'.
///
/// Rules: `VRB-PART-003`
#[must_use]
pub fn future_participle() -> Suffix {
    use Arch::{A, OAcute};
    use Seg::{Lit, Var};
    two(&[Var(A), Lit("nd"), Var(OAcute)])
}

/// The adverbial participle `-vA`, whose `v` is written and never assimilated.
///
/// Rules: `VRB-ADVP-001`
#[must_use]
pub fn converb() -> Suffix {
    use Arch::A;
    use Seg::{Lit, Var};
    two(&[Lit("v"), Var(A)])
}

/// The formal converb `-vÁn` (`VRB-ADVP-002`), an accepted variant of [`converb`].
pub const FORMAL_CONVERB: Suffix = two(&[Seg::Lit("v"), Seg::Var(Arch::AAcute), Seg::Lit("n")]);

/// Whether this entry is an `-ik` verb, whose third singular is `-ik` rather than zero.
///
/// Rules: `VRB-IK-001`
#[must_use]
pub fn is_ik(entry: &Entry) -> bool {
    entry.lemma.ends_with("ik") && present_stem(&entry.lemma) != entry.lemma.as_ref()
}

#[cfg(test)]
mod tests {
    use super::{heavy, present_stem, sibilant_final};

    #[test]
    fn strips_the_ik_ending() {
        assert_eq!(present_stem("dolgozik"), "dolgoz");
        assert_eq!(present_stem("alszik"), "alsz");
        assert_eq!(present_stem("vár"), "vár");
        assert_eq!(present_stem("ik"), "ik");
    }

    #[test]
    fn two_different_final_consonants_are_heavy() {
        // VRB-LINK-002 (a).
        for s in ["tart", "fest", "küld", "mond", "ugr"] {
            assert!(heavy(s), "{s} must be heavy");
        }
    }

    #[test]
    fn it_final_stems_are_heavy() {
        // VRB-LINK-002 (b).
        assert!(heavy("tanít"));
        assert!(heavy("fordít"));
    }

    /// `VRB-LINK-002` (b): a monosyllable in vowel + `dz` ends in a long affricate; the `-(Ó)dz-`
    /// of a polysyllable is short.
    #[test]
    fn a_long_dz_is_heavy() {
        assert!(heavy("edz"));
        assert!(heavy("pedz"));
        assert!(!heavy("lopódz"));
    }

    #[test]
    fn single_consonants_are_light() {
        for s in ["vár", "kér", "ad", "lát", "fut", "tör", "mos"] {
            assert!(!heavy(s), "{s} must be light");
        }
    }

    #[test]
    fn geminates_are_lexical_and_go_both_ways() {
        // VRB-LINK-002 (d): `hall` is heavy, `áll` is not, and nothing in the shape says which.
        // The computed answer is "light" for both; the lexicon must override `hall`.
        assert!(!heavy("hall"));
        assert!(!heavy("áll"));
    }

    #[test]
    fn sibilant_final_stems_take_the_ol_ending() {
        for s in ["mos", "néz", "főz", "olvas"] {
            assert!(sibilant_final(s), "{s}");
        }
        assert!(!sibilant_final("vár"));
    }
}

#[cfg(test)]
mod v_stem_tests {
    use picur_morph_lexicon::lexicon;

    use super::v_stem;
    use crate::features::Morph;
    use crate::generate::generate;

    #[test]
    fn vrb_stem_012_the_v_stem_is_computed_from_the_lemma() {
        assert_eq!(v_stem("alszik").as_deref(), Some("alv"));
        assert_eq!(v_stem("fekszik").as_deref(), Some("fekv"));
        assert_eq!(v_stem("törekszik").as_deref(), Some("törekv"));
        assert_eq!(v_stem("átalszik").as_deref(), Some("átalv"));
        assert_eq!(v_stem("emlékszik"), None);
        assert_eq!(v_stem("olvas"), None);
    }

    /// `DER-MODEL-006`: `-Ó` and `-Ás` take the `v` stem, the `d` stem gives no word.
    #[test]
    fn vrb_part_001_the_participle_is_built_on_the_v_stem() {
        for (lemma, want) in [
            ("alszik", "alvó"),
            ("fekszik", "fekvő"),
            ("nyugszik", "nyugvó"),
            ("törekszik", "törekvő"),
            ("növekszik", "növekvő"),
            ("emlékszik", "emlékező"),
        ] {
            let lx = lexicon();
            let entry = lx
                .get_pos(lemma, picur_morph_lexicon::Pos::Verb)
                .expect("a listed verb");
            let got = generate(entry, &[Morph::PresentParticiple]).expect("generates");
            assert_eq!(got, want, "{lemma}");
        }
    }
}

#[cfg(test)]
mod sz_d_stem_tests {
    use picur_morph_lexicon::{Pos, lexicon};

    use super::sz_d_stem;
    use crate::features::{Definite, Mood, Morph, Number, Person, Tense};
    use crate::generate::generate;

    /// `VRB-STEM-012`: everything but the present is built on the `d` or `z` stem.
    #[test]
    fn vrb_stem_012_the_d_stem_is_recovered_from_the_doublet() {
        for (lemma, past, infinitive) in [
            ("öregszik", "öregedett", "öregedni"),
            // ORT-TOG-026: a preverb carries the base's past-t exception.
            ("átalszik", "átaludt", "átaludni"),
            ("telepszik", "telepedett", "telepedni"),
            ("mosakszik", "mosakodott", "mosakodni"),
            ("veszekszik", "veszekedett", "veszekedni"),
            ("megharagszik", "megharagudott", "megharagudni"),
            ("emlékszik", "emlékezett", "emlékezni"),
            ("alszik", "aludt", "aludni"),
        ] {
            let lx = lexicon();
            let entry = lx.get_pos(lemma, Pos::Verb).expect("a listed verb");
            let got = generate(
                entry,
                &[Morph::Verbal {
                    mood: Mood::Ind,
                    tense: Tense::Past,
                    person: Person::Third,
                    number: Number::Sing,
                    definite: Definite::Ind,
                }],
            )
            .expect("generates");
            assert_eq!(got, past, "{lemma} past");
            let got = generate(entry, &[Morph::Infinitive]).expect("generates");
            assert_eq!(got, infinitive, "{lemma} infinitive");
        }
    }

    /// A verb that merely ends in `sz` is not of the class.
    #[test]
    fn vrb_stem_012_an_ordinary_sz_verb_is_left_alone() {
        for lemma in ["játszik", "tetszik", "úszik"] {
            assert_eq!(sz_d_stem(lemma), None, "{lemma}");
        }
    }
}
