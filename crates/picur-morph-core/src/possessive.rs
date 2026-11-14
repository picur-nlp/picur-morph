//! Possessive suffixes.
//!
//! The shape depends on the possessor, on whether the stem ends in a vowel, and for third person
//! on whether the lexeme takes `-a`/`-e` or `-ja`/`-je`. That last choice is partly categorical and
//! partly lexical, and `-ja` is **not** the general consonant-final default: it is 10.7% of
//! measured usage, so the fallback is `-a`.
//!
//! See `docs/grammar/03-nominal.md` (`NOM-POSS-001` onward).

use picur_morph_lexicon::{Entry, Pos, Role, Trait, rows, words};
use picur_morph_ortho::{Letter, letters};

use crate::features::{Number, Person};
use crate::stem::{Coda, coda};
use crate::suffix::{Arch, Seg, Suffix, SuffixClass};

/// Consonants after which the third person possessive is categorically `-a`/`-e`.
///
/// The sibilants and affricates of `NOM-POSS-005`'s own list, and `j`, which the suffix's own `j`
/// could not be told from. The palatals of that list are in [`palatal`] instead.
///
/// Rules: `NOM-POSS-005`
const fn categorical_a(l: Letter) -> bool {
    matches!(
        l,
        Letter::S
            | Letter::Sz
            | Letter::Z
            | Letter::Zs
            | Letter::C
            | Letter::Cs
            | Letter::Dzs
            | Letter::J
    )
}

/// The palatals, after which a suffix-initial `j` is written but not pronounced.
///
/// `PH-ASSIM-007` states the assimilation and names its own members: `anya` and `atya` on their
/// bound stems, and `nagy` + `-ja`. So the `-a` default of `NOM-POSS-005` holds here only until a
/// lexical class says otherwise (`nagyja` 9,120 : `nagyai` 33, `csínyje` 31 : `csínya` 4 `[WC2]`).
///
/// Rules: `PH-ASSIM-007`, `NOM-POSS-005`, `NOM-POSS-006`
const fn palatal(l: Letter) -> bool {
    matches!(l, Letter::Ly | Letter::Gy | Letter::Ny | Letter::Ty)
}

/// The kinship nouns of `NOM-STEM-014`, which truncate before the third person possessive
/// (`apja`, `atyja`, `bátyjuk`), and every compound they close (`nagyapja`, `édesanyja`).
#[must_use]
pub fn kinship_truncates(entry: &Entry) -> bool {
    entry.pos == Pos::Noun
        && words(Role::KinshipTruncating)
            .iter()
            .any(|k| entry.lemma.ends_with(k.as_ref()))
}

/// Whether the third person possessive takes the `j` form.
///
/// After a vowel it always does (`hajója`). After a consonant the categorical class of
/// `NOM-POSS-005` forbids it, a lexical trait may force it either way, and the residual default is
/// the `-a` form.
///
/// Rules: `NOM-POSS-004`, `NOM-POSS-005`, `NOM-POSS-006`, `NOM-POSS-007`
#[must_use]
pub fn takes_j(stem: &str, entry: &Entry) -> bool {
    // NOM-STEM-014 states `anyja` and `bátyja`, which the categorical class would forbid: the
    // truncated kinship stems end in `ny` and `ty` and still take `-jA`.
    if kinship_truncates(entry) {
        return true;
    }
    match coda(stem) {
        Coda::Vowel => true,
        Coda::Single(c) | Coda::Geminate(c) | Coda::Cluster(c) => {
            if stem.ends_with("ság") || stem.ends_with("ség") {
                return false;
            }
            if !palatal(c) && categorical_a(c) {
                return false;
            }
            entry.has(Trait::PossessiveJa)
        }
    }
}

/// The person suffix that follows the possessed plural `-i-`.
///
/// `NOM-PPL-001` states the set: `-m`, `-d`, zero, `-nk`, `-tok/-tek`, `-k`. It is not the ordinary
/// possessive set, which would give `*adataija` for `adatai` and `*adataijuk` for `adataik`.
/// `NOM-PPL-004` adds that the 2pl has no rounded form, which follows anyway from the `i` the
/// suffix attaches to.
///
/// `None` is the zero third singular.
///
/// Rules: `NOM-PPL-001`, `NOM-PPL-004`
#[must_use]
pub fn possessive_after_i(person: Person, number: Number) -> Option<Suffix> {
    use crate::suffix::{Arch, Seg::Lit, Seg::Var};
    Some(match (person, number) {
        (Person::First, Number::Sing) => class_two(&[Lit("m")]),
        (Person::Second, Number::Sing) => class_two(&[Lit("d")]),
        (Person::Third, Number::Sing) => return None,
        (Person::First, Number::Plur) => class_two(&[Lit("nk")]),
        (Person::Second, Number::Plur) => class_two(&[Lit("t"), Var(Arch::O), Lit("k")]),
        (Person::Third, Number::Plur) => class_two(&[Lit("k")]),
    })
}

const fn class_two(template: &'static [Seg]) -> Suffix {
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

const fn class_one(template: &'static [Seg], lowerable: bool) -> Suffix {
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

/// The possessive forms of a lexeme whose stem no rule reaches.
///
/// `fiú` has the bound stem `fi`, which takes the consonant-stem allomorphs throughout: `fi` plus
/// `-am`, `-ad`, `-a`, `-unk`, `-atok`, `-uk`, `-ai` is the whole paradigm, exactly as `ház`
/// inflects. The suffixation machinery cannot get there, because it reads the link off the coda
/// and `fi` ends in a vowel, so the forms are stated. morphdb hides the lexeme behind a phantom
/// lemma `fia` marked as taking no possessive, which left `fiam` unanalysable at 198037 `[WC2]`.
///
/// `None` for every other lexeme, which is the ordinary path.
///
/// Rules: `NOM-POSS-002`, `NOM-POSS-003`, `NOM-PPL-001`
#[must_use]
pub fn irregular(lemma: &str, person: Person, number: Number) -> Option<&'static str> {
    let row = listed_possessive(lemma)?;
    let at = match (person, number) {
        (Person::First, Number::Sing) => 1,
        (Person::Second, Number::Sing) => 2,
        (Person::Third, Number::Sing) => 3,
        (Person::First, Number::Plur) => 4,
        (Person::Second, Number::Plur) => 5,
        (Person::Third, Number::Plur) => 6,
    };
    row.get(at).map(AsRef::as_ref)
}

/// The inventory row of a lexeme with listed possessive forms: the lemma, the six person forms
/// and the possessed plural.
fn listed_possessive(lemma: &str) -> Option<&'static [Box<str>]> {
    rows(Role::IrregularPossessive)
        .iter()
        .find(|r| r[0].as_ref() == lemma)
        .map(AsRef::as_ref)
}

/// The possessed plural of a lexeme whose stem no rule reaches.
///
/// Rules: `NOM-PPL-001`
#[must_use]
pub fn irregular_possessed_plural(lemma: &str) -> Option<&'static str> {
    listed_possessive(lemma)?.get(7).map(AsRef::as_ref)
}

/// The possessive suffix for this possessor, given the stem it attaches to.
///
/// Rules: `NOM-POSS-001`, `NOM-POSS-002`, `NOM-POSS-003`, `NOM-POSS-008`
#[must_use]
pub fn possessive(person: Person, number: Number, stem: &str, entry: &Entry) -> Suffix {
    use Arch::{A, O, U};
    use Seg::{Lit, Var};
    match (person, number) {
        // NOM-POSS-002: lowerable, so `ház` gives `házam` while `pad` gives `padom`.
        (Person::First, Number::Sing) => class_one(&[Var(O), Lit("m")], true),
        (Person::Second, Number::Sing) => class_one(&[Var(O), Lit("d")], true),
        (Person::Second, Number::Plur) => class_one(&[Var(O), Lit("t"), Var(O), Lit("k")], true),
        // NOM-POSS-003: `-unk`/`-ünk` irrespective of lowering, so this one is not lowerable.
        (Person::First, Number::Plur) => class_one(&[Var(U), Lit("nk")], false),
        (Person::Third, Number::Sing) => {
            if takes_j(stem, entry) {
                class_two(&[Lit("j"), Var(A)])
            } else {
                class_two(&[Var(A)])
            }
        }
        (Person::Third, Number::Plur) => {
            if takes_j(stem, entry) {
                class_two(&[Lit("j"), Var(U), Lit("k")])
            } else {
                class_two(&[Var(U), Lit("k")])
            }
        }
    }
}

/// The possessed-plural marker `-(j)Ai-` / `-i-`.
///
/// After a consonant it repeats the third singular `-a`/`-ja` choice (`házai`, `kertjei`); after a
/// vowel it is bare `-i` (`hajói`, `almái`), except after stem-final `i`, where the `j` returns to
/// break the hiatus (`kocsijai`, never `*kocsii`).
///
/// Rules: `NOM-PPL-001`, `NOM-PPL-002`
#[must_use]
pub fn possessed_plural(stem: &str, entry: &Entry) -> Suffix {
    use Arch::A;
    use Seg::{Lit, Var};
    let i_final = stem.ends_with('i') || stem.ends_with('í');
    // NOM-PPL-001: the cell follows the singular's allomorph unless the corpus says it does not,
    // which for this cell it often does: `barátja` beside `barátai`.
    let j = if entry.has(Trait::PossessedPluralA) {
        false
    } else if entry.has(Trait::PossessedPluralJa) {
        true
    } else {
        takes_j(stem, entry)
    };
    match coda(stem) {
        // After a vowel the default is the bare `-i`, whatever the singular takes: `hajója` but
        // `hajói`. Only a measured `-jai` overrides it, which is what `ajtajai` needs beside the
        // same class's `szülei`.
        Coda::Vowel if !i_final => {
            if entry.has(Trait::PossessedPluralJa) {
                class_two(&[Lit("j"), Var(A), Lit("i")])
            } else {
                class_two(&[Lit("i")])
            }
        }
        Coda::Vowel => class_two(&[Lit("j"), Var(A), Lit("i")]),
        _ if j => class_two(&[Lit("j"), Var(A), Lit("i")]),
        _ => class_two(&[Var(A), Lit("i")]),
    }
}

/// Whether a stem is vowel-final, which decides several possessive shapes.
#[must_use]
pub fn ends_in_vowel(stem: &str) -> bool {
    letters(stem).last().is_some_and(|s| s.letter.is_vowel())
}

#[cfg(test)]
mod tests {
    use picur_morph_lexicon::{Entry, Pos, Trait, TraitSet};

    use super::takes_j;

    fn e(lemma: &str, traits: &[Trait]) -> Entry {
        let mut x = Entry::new(lemma, Pos::Noun);
        x.traits = TraitSet::from_iter_of(traits.iter().copied());
        x
    }

    #[test]
    fn vowel_final_stems_always_take_j() {
        assert!(takes_j("hajó", &e("hajó", &[])));
        assert!(takes_j("alma", &e("alma", &[])));
    }

    #[test]
    fn sibilants_are_categorically_a() {
        // NOM-POSS-005: sasa, része, kulcsa, olaja, perece, whatever the entry says.
        for s in ["sas", "rész", "kulcs", "olaj", "perec"] {
            assert!(!takes_j(s, &e(s, &[Trait::PossessiveJa])), "{s} must be -a");
        }
    }

    #[test]
    fn a_palatal_takes_a_by_default_and_ja_when_marked() {
        // PH-ASSIM-007: the `j` is written and not pronounced, so the lexical class decides
        // (`királya`, `nagyja` 9,120 : `nagyai` 33 [WC2]).
        assert!(!takes_j("király", &e("király", &[])));
        assert!(takes_j("nagy", &e("nagy", &[Trait::PossessiveJa])));
    }

    #[test]
    fn sag_seg_derivatives_are_a() {
        assert!(!takes_j("szépség", &e("szépség", &[])));
        assert!(!takes_j("igazság", &e("igazság", &[])));
    }

    #[test]
    fn the_lexical_trait_decides_the_rest() {
        assert!(takes_j("kalap", &e("kalap", &[Trait::PossessiveJa])));
        assert!(!takes_j("ablak", &e("ablak", &[Trait::PossessiveA])));
    }

    #[test]
    fn the_default_is_a_not_ja() {
        // `-ja` is 10.7% of measured usage, so an unmarked consonant-final stem takes `-a`.
        assert!(!takes_j("xyzp", &e("xyzp", &[])));
    }
}
