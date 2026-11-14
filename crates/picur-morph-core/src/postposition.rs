//! Postpositions with a personal suffix.
//!
//! `PND-POSTP-005` prints a paradigm of thirty postpositions, but it calls the suffixes `rule` and
//! only the licence and two stems `lexical`. That is exactly right: the suffixes are the
//! `PND-PERS-004` set that `pronoun` already computes, so `szerintem` is `szerint` plus the same
//! `-em` that makes `nekem`. What has to be stated is which postpositions take personal forms at
//! all, since `óta`, `múlva` and every oblique-governing postposition do not (`*együttem`).
//!
//! See `docs/grammar/04-pronouns-numerals-determiners.md`.

use std::sync::LazyLock;

use picur_morph_lexicon::{Role, rows};
use picur_morph_ortho::{harmony_of, letters};

use crate::features::{Number, Person};
use crate::pronoun::{personal_suffix, realise};
use crate::suffix::{Arch, Seg};

/// One recognised personal-suffixed postposition form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PostpositionForm {
    /// The postposition it belongs to.
    pub lemma: &'static str,
    /// Person of the complement.
    pub person: Person,
    /// Number of the complement.
    pub number: Number,
}

/// The personal form of `postposition`, or `None` if it licenses none.
///
/// Rules: `PND-POSTP-005`, `PND-PERS-004`
#[must_use]
pub fn personal(postposition: &str, person: Person, number: Number) -> Option<String> {
    let row = rows(Role::PostpositionPersonal)
        .iter()
        .find(|r| r[0].as_ref() == postposition)?;
    Some(build(row.get(1)?, person, number))
}

/// The form built on one stem.
fn build(stem: &str, person: Person, number: Number) -> String {
    let vowel_final = letters(stem).last().is_some_and(|s| s.letter.is_vowel());
    let tail = realise(
        personal_suffix(vowel_final, person, number),
        harmony_of(stem),
    );
    format!("{stem}{tail}")
}

/// Every personal-suffixed postposition form, computed rather than listed.
fn table() -> Vec<(String, PostpositionForm)> {
    let cells = crate::pronoun::CELLS;
    let mut out = Vec::new();
    // PND-POSTP-005's exception: after a long vowel the third singular is zero, and a `-je`/`-ja`
    // form is a colloquial variant. The rule says to generate the zero form, so this is
    // recognition only: `eléje`, `feléje`, `melléje`, `rája`.
    for row in rows(Role::PostpositionPersonal) {
        let (lemma, stems) = (row[0].as_ref(), &row[1..]);
        for stem in stems {
            let stem: &str = stem;
            if !letters(stem).last().is_some_and(|s| s.letter.is_vowel()) {
                continue;
            }
            let tail = realise(&[Seg::Lit("j"), Seg::Var(Arch::A)], harmony_of(stem));
            out.push((
                format!("{stem}{tail}"),
                PostpositionForm {
                    lemma,
                    person: Person::Third,
                    number: Number::Sing,
                },
            ));
        }
    }
    for row in rows(Role::PostpositionPersonal) {
        let (lemma, stems) = (row[0].as_ref(), &row[1..]);
        for stem in stems {
            let stem: &str = stem;
            for (person, number) in cells {
                out.push((
                    build(stem, person, number),
                    PostpositionForm {
                        lemma,
                        person,
                        number,
                    },
                ));
            }
        }
    }
    out
}

static TABLE: LazyLock<Vec<(String, PostpositionForm)>> = LazyLock::new(table);

/// Recognises a personal-suffixed postposition.
///
/// Rules: `PND-POSTP-005`
#[must_use]
pub fn analyse(surface: &str) -> Option<PostpositionForm> {
    TABLE
        .iter()
        .find(|(form, _)| form == surface)
        .map(|(_, f)| *f)
}

/// The demonstrative written together with a nominative-governing postposition.
///
/// `PND-DEM-009` states the form as a rule: `a`/`e` before a consonant-initial postposition and
/// `az`/`ez` before a vowel-initial one, so `amiatt` and `ezelőtt` are computed from the
/// postposition rather than listed. The rule's own list is 30 items long and is not the class.
///
/// Returns the compound itself, which is the lemma: these are adverbs and correlatives, not case
/// forms of the postposition.
///
/// The rule notes `ezóta` as marginal, so the form is accepted where the rulebook prefers `azóta`.
///
/// Rules: `PND-DEM-009`, `ORT-TOG-029`
#[must_use]
pub fn demonstrative(surface: &str) -> Option<String> {
    let lower = surface.to_lowercase();
    for prefix in ["az", "ez", "a", "e"] {
        let Some(rest) = lower.strip_prefix(prefix) else {
            continue;
        };
        if rest.is_empty() {
            continue;
        }
        let vowel_initial = picur_morph_ortho::letters(rest)
            .next()
            .is_some_and(|s| s.letter.is_vowel());
        // The long demonstrative goes before a vowel, the short one before a consonant.
        if vowel_initial != (prefix.len() == 2) {
            continue;
        }
        let governs_nominative =
            picur_morph_lexicon::lexicon().has_pos(rest, picur_morph_lexicon::Pos::Postposition);
        if governs_nominative {
            return Some(lower);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{analyse, personal};
    use crate::features::{Number, Person};

    #[test]
    fn the_colloquial_third_singular_is_recognised() {
        // PND-POSTP-005: after a long vowel the third singular is zero, and a `-je`/`-ja` form is
        // a colloquial variant. The rule says to generate the zero form, so these are recognition
        // only.
        for word in ["eléje", "feléje", "melléje", "mögéje"] {
            assert!(analyse(word).is_some(), "{word}");
        }
        assert_eq!(
            personal("elé", Person::Third, Number::Sing).as_deref(),
            Some("elé")
        );
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "the whole PND-POSTP-005 table, as a table"
    )]
    fn the_paradigm_matches_the_rulebook_table() {
        for (lemma, want) in [
            (
                "szerint",
                [
                    "szerintem",
                    "szerinted",
                    "szerinte",
                    "szerintünk",
                    "szerintetek",
                    "szerintük",
                ],
            ),
            (
                "által",
                [
                    "általam",
                    "általad",
                    "általa",
                    "általunk",
                    "általatok",
                    "általuk",
                ],
            ),
            (
                "után",
                ["utánam", "utánad", "utána", "utánunk", "utánatok", "utánuk"],
            ),
            (
                "alatt",
                [
                    "alattam",
                    "alattad",
                    "alatta",
                    "alattunk",
                    "alattatok",
                    "alattuk",
                ],
            ),
            (
                "fölött",
                [
                    "fölöttem",
                    "fölötted",
                    "fölötte",
                    "fölöttünk",
                    "fölöttetek",
                    "fölöttük",
                ],
            ),
            (
                "előtt",
                [
                    "előttem",
                    "előtted",
                    "előtte",
                    "előttünk",
                    "előttetek",
                    "előttük",
                ],
            ),
            (
                "mögött",
                [
                    "mögöttem",
                    "mögötted",
                    "mögötte",
                    "mögöttünk",
                    "mögöttetek",
                    "mögöttük",
                ],
            ),
            (
                "mellett",
                [
                    "mellettem",
                    "melletted",
                    "mellette",
                    "mellettünk",
                    "mellettetek",
                    "mellettük",
                ],
            ),
            (
                "nélkül",
                [
                    "nélkülem",
                    "nélküled",
                    "nélküle",
                    "nélkülünk",
                    "nélkületek",
                    "nélkülük",
                ],
            ),
            (
                "ellen",
                [
                    "ellenem",
                    "ellened",
                    "ellene",
                    "ellenünk",
                    "ellenetek",
                    "ellenük",
                ],
            ),
            (
                "iránt",
                [
                    "irántam",
                    "irántad",
                    "iránta",
                    "irántunk",
                    "irántatok",
                    "irántuk",
                ],
            ),
            (
                "miatt",
                [
                    "miattam",
                    "miattad",
                    "miatta",
                    "miattunk",
                    "miattatok",
                    "miattuk",
                ],
            ),
            (
                "helyett",
                [
                    "helyettem",
                    "helyetted",
                    "helyette",
                    "helyettünk",
                    "helyettetek",
                    "helyettük",
                ],
            ),
            (
                "elől",
                ["előlem", "előled", "előle", "előlünk", "előletek", "előlük"],
            ),
            (
                "alól",
                ["alólam", "alólad", "alóla", "alólunk", "alólatok", "alóluk"],
            ),
            (
                "mellől",
                [
                    "mellőlem",
                    "mellőled",
                    "mellőle",
                    "mellőlünk",
                    "mellőletek",
                    "mellőlük",
                ],
            ),
            (
                "felől",
                [
                    "felőlem",
                    "felőled",
                    "felőle",
                    "felőlünk",
                    "felőletek",
                    "felőlük",
                ],
            ),
            (
                "mögül",
                [
                    "mögülem",
                    "mögüled",
                    "mögüle",
                    "mögülünk",
                    "mögületek",
                    "mögülük",
                ],
            ),
            (
                "közül",
                [
                    "közülem",
                    "közüled",
                    "közüle",
                    "közülünk",
                    "közületek",
                    "közülük",
                ],
            ),
        ] {
            let cells = [
                (Person::First, Number::Sing),
                (Person::Second, Number::Sing),
                (Person::Third, Number::Sing),
                (Person::First, Number::Plur),
                (Person::Second, Number::Plur),
                (Person::Third, Number::Plur),
            ];
            for (i, (person, number)) in cells.into_iter().enumerate() {
                assert_eq!(
                    personal(lemma, person, number).as_deref(),
                    Some(want[i]),
                    "{lemma} cell {i}"
                );
            }
        }
    }

    #[test]
    fn a_vowel_final_postposition_has_a_zero_third_singular() {
        // PND-PERS-004: after a long vowel the suffix carries no vowel and 3sg is the bare stem.
        for (lemma, want) in [
            ("alá", ["alám", "alád", "alá", "alánk", "alátok", "alájuk"]),
            (
                "felé",
                ["felém", "feléd", "felé", "felénk", "felétek", "feléjük"],
            ),
            (
                "mögé",
                ["mögém", "mögéd", "mögé", "mögénk", "mögétek", "mögéjük"],
            ),
            (
                "köré",
                ["körém", "köréd", "köré", "körénk", "körétek", "köréjük"],
            ),
        ] {
            let cells = [
                (Person::First, Number::Sing),
                (Person::Second, Number::Sing),
                (Person::Third, Number::Sing),
                (Person::First, Number::Plur),
                (Person::Second, Number::Plur),
                (Person::Third, Number::Plur),
            ];
            for (i, (person, number)) in cells.into_iter().enumerate() {
                assert_eq!(
                    personal(lemma, person, number).as_deref(),
                    Some(want[i]),
                    "{lemma} cell {i}"
                );
            }
        }
    }

    #[test]
    fn the_two_lexical_stems_are_used() {
        // `körül` builds on `körülött-` and `között` prefers the short `közt-`.
        assert_eq!(
            personal("körül", Person::First, Number::Sing).as_deref(),
            Some("körülöttem")
        );
        assert_eq!(
            personal("között", Person::Third, Number::Plur).as_deref(),
            Some("köztük")
        );
        assert!(analyse("közöttük").is_some());
        assert!(analyse("közöttem").is_some());
    }

    #[test]
    fn forms_are_recognised_back() {
        for form in [
            "szerintem",
            "általa",
            "köztük",
            "miattuk",
            "nélküled",
            "körülötte",
            "utánam",
        ] {
            assert!(analyse(form).is_some(), "{form}");
        }
    }

    #[test]
    fn an_unlicensed_postposition_has_no_personal_form() {
        // PND-POSTP-005 exceptions: `óta`, `múlva` and the oblique-governing ones.
        for lemma in ["óta", "múlva", "közben", "során", "együtt", "végett"] {
            assert_eq!(
                personal(lemma, Person::First, Number::Sing),
                None,
                "{lemma}"
            );
        }
    }
}
