//! The anaphoric `-é`, the associative `-ék` and what the rulebook writes after them.
//!
//! `NOM-ANP-001`, `NOM-ANP-002`, `NOM-ASSOC-001`, `NOM-ASSOC-002`, `NOM-LOW-002`.

#![expect(
    clippy::expect_used,
    reason = "a golden that cannot find its lemma must fail loudly"
)]

use picur_morph_core::{Case, Morph, Number, Person, Stacks, Verdict, analyse, generate};
use picur_morph_lexicon::{Entry, Pos, lexicon};

fn form(lemma: &str, stack: &[Morph]) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Noun)
        .unwrap_or_else(|| panic!("{lemma} missing from the lexicon"));
    generate(e, stack).expect("generates")
}

fn named(lemma: &str, stack: &[Morph]) -> String {
    generate(&Entry::new(lemma, Pos::Noun), stack).expect("generates")
}

/// The stacks these rules write, which is what the checker has to search for them.
fn stacks() -> Stacks {
    let mut all: Vec<Vec<Morph>> = Vec::new();
    for base in [
        vec![],
        vec![Morph::Plural],
        vec![POSS1],
        vec![Morph::Possessive {
            person: Person::Third,
            number: Number::Sing,
        }],
        vec![Morph::PossessedPlural, POSS1],
        vec![Morph::Associative],
        vec![POSS1, Morph::Associative],
    ] {
        all.push(base.clone());
        for tail in [
            vec![Morph::Anaphoric],
            vec![Morph::Anaphoric, Morph::Plural],
        ] {
            let stack = [base.as_slice(), tail.as_slice()].concat();
            all.push(stack.clone());
            for c in [Case::Acc, Case::Dat, Case::Ela] {
                all.push([stack.as_slice(), &[Morph::Case(c)]].concat());
            }
        }
    }
    Stacks::new(all)
}

fn known(word: &str) -> bool {
    matches!(analyse(word, &stacks()), Verdict::Known(_))
}

const POSS1: Morph = Morph::Possessive {
    person: Person::First,
    number: Number::Sing,
};

#[test]
fn the_anaphoric_pluralises_with_i() {
    // NOM-ANP-002: plurality of the possessed items is `-é` plus `-i`, never `-k`.
    assert_eq!(form("ház", &[Morph::Anaphoric, Morph::Plural]), "házéi");
    assert_eq!(
        form("ház", &[Morph::Plural, Morph::Anaphoric, Morph::Plural]),
        "házakéi"
    );
}

#[test]
fn harmony_after_the_anaphoric_follows_the_stem() {
    // NOM-ANP-002: the neutral `é` and `i` are transparent, so the case agrees with `ház`.
    for (stack, want) in [
        (vec![Morph::Anaphoric, Morph::Case(Case::Dat)], "házénak"),
        (
            vec![Morph::Anaphoric, Morph::Plural, Morph::Case(Case::Dat)],
            "házéinak",
        ),
        (
            vec![Morph::Anaphoric, Morph::Plural, Morph::Case(Case::All)],
            "házéihoz",
        ),
        (
            vec![
                Morph::PossessedPlural,
                POSS1,
                Morph::Anaphoric,
                Morph::Plural,
                Morph::Case(Case::Ela),
            ],
            "házaiméiból",
        ),
    ] {
        assert_eq!(form("ház", &stack), want);
    }
    assert_eq!(
        form("kert", &[Morph::Anaphoric, Morph::Case(Case::Ade)]),
        "kerténél"
    );
}

#[test]
fn the_associative_is_transparent_and_lowering() {
    // NOM-ASSOC-002, NOM-LOW-002: `é` is transparent to the stem's backness and the `-ék` stem
    // is lowering, so the accusative is `-at` after a back name.
    assert_eq!(named("Szabó", &[Morph::Associative]), "Szabóék");
    assert_eq!(
        named("Szabó", &[Morph::Associative, Morph::Case(Case::Acc)]),
        "Szabóékat"
    );
    assert_eq!(
        named("Tomi", &[Morph::Associative, Morph::Case(Case::Dat)]),
        "Tomiéknak"
    );
    assert_eq!(
        named("Péter", &[Morph::Associative, Morph::Case(Case::Dat)]),
        "Péteréknek"
    );
}

#[test]
fn the_checker_reads_both_back() {
    for word in [
        "Szabóék",
        "Péterék",
        "szomszédék",
        "orvosék",
        "Péteréké",
        "házéi",
        "házakéi",
        "házáéi",
        "házaiméiból",
        "öcsémék",
    ] {
        assert!(known(word), "{word}");
    }
}
