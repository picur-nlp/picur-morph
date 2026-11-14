//! The silent-h stems, the invariant `-képpen` and the listed possessive paradigms.
//!
//! `NOM-STEM-004`, `NOM-STEM-014`, `NOM-STEM-015`, `NOM-CASE-003`, `NOM-CASE-004`,
//! `NOM-CASE-020`, `NOM-ACC-008`.

#![expect(
    clippy::expect_used,
    reason = "a golden that cannot find its lemma must fail loudly"
)]

use picur_morph_core::{Case, Morph, Number, Person, Stacks, Verdict, analyse, generate};
use picur_morph_lexicon::{Pos, lexicon};

fn form(lemma: &str, stack: &[Morph]) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Noun)
        .unwrap_or_else(|| panic!("{lemma} missing from the lexicon"));
    generate(e, stack).expect("generates")
}

/// Every nominal stack these rules need, which is what the checker searches for them.
fn stacks() -> Stacks {
    let mut all: Vec<Vec<Morph>> = vec![vec![], vec![Morph::Plural]];
    for c in [
        Case::Acc,
        Case::Ins,
        Case::Tra,
        Case::Dat,
        Case::Ill,
        Case::Sup,
    ] {
        all.push(vec![Morph::Case(c)]);
        all.push(vec![Morph::Plural, Morph::Case(c)]);
    }
    all.push(vec![Morph::Case(Case::Modalis)]);
    all.push(vec![Morph::Case(Case::Modalis), Morph::Case(Case::Sup)]);
    for person in [Person::First, Person::Second, Person::Third] {
        for number in [Number::Sing, Number::Plur] {
            let poss = Morph::Possessive { person, number };
            all.push(vec![poss]);
            all.push(vec![Morph::PossessedPlural, poss]);
            all.push(vec![poss, Morph::Case(Case::Ill)]);
        }
    }
    Stacks::new(all)
}

fn known(word: &str) -> bool {
    matches!(analyse(word, &stacks()), Verdict::Known(_))
}

#[test]
fn a_silent_final_h_keeps_the_v() {
    // NOM-STEM-004, NOM-CASE-003, NOM-CASE-004: the h-adding stems are vowel-final before
    // `-vAl` and `-vÁ`; the corpus agrees (`dühvel` 5,724 : `dühhel` 456).
    assert_eq!(form("cseh", &[Morph::Case(Case::Ins)]), "csehvel");
    assert_eq!(form("pléh", &[Morph::Case(Case::Tra)]), "pléhvé");
    assert_eq!(form("düh", &[Morph::Case(Case::Ins)]), "dühvel");
    assert_eq!(form("doh", &[Morph::Case(Case::Ins)]), "dohhal");
    for word in ["dühhel", "méhhé", "csehvel", "pléhvé", "méhet"] {
        assert!(known(word), "{word}");
    }
}

#[test]
fn keppen_is_invariant() {
    // NOM-CASE-020: `-képp` ~ `-képpen` is one invariant suffix, so a back stem does not
    // harmonise its `-en`.
    let stack = [Morph::Case(Case::Modalis), Morph::Case(Case::Sup)];
    assert_eq!(form("példa", &stack), "példaképpen");
    // NOM-LENG-002: and it does not lengthen the stem's final `a`.
    assert_eq!(form("óra", &stack), "óraképpen");
}

#[test]
fn the_listed_possessive_paradigms() {
    // NOM-STEM-014: `öcs` has no free form; NOM-STEM-015: `száj` uses `szá-` before the
    // consonant-initial suffixes.
    let cells = [
        (Person::First, Number::Sing),
        (Person::Second, Number::Sing),
        (Person::Third, Number::Sing),
        (Person::First, Number::Plur),
        (Person::Second, Number::Plur),
        (Person::Third, Number::Plur),
    ];
    for (lemma, want) in [
        (
            "öcs",
            ["öcsém", "öcséd", "öccse", "öcsénk", "öcsétek", "öccsük"],
        ),
        (
            "száj",
            ["szám", "szád", "szája", "szánk", "szátok", "szájuk"],
        ),
    ] {
        for (i, (person, number)) in cells.into_iter().enumerate() {
            assert_eq!(
                form(lemma, &[Morph::Possessive { person, number }]),
                want[i],
                "{lemma} cell {i}"
            );
        }
    }
    assert_eq!(
        form(
            "öcs",
            &[
                Morph::PossessedPlural,
                Morph::Possessive {
                    person: Person::First,
                    number: Number::Sing,
                },
            ]
        ),
        "öcséim"
    );
    assert!(known("szájba"));
    assert!(known("szánkba"));
}

#[test]
fn the_bound_alternant_takes_the_linked_accusative() {
    // NOM-ACC-008: the free stem takes the bare `-t` after the sibilant and the bound alternant
    // the linking vowel, and both are current (`bajuszt` 4,257 : `bajszot` 958).
    assert_eq!(form("bajusz", &[Morph::Case(Case::Acc)]), "bajuszt");
    assert_eq!(form("ész", &[Morph::Case(Case::Acc)]), "észt");
    for word in ["bajszot", "eszet", "öbölt", "öblöt"] {
        assert!(known(word), "{word}");
    }
}
