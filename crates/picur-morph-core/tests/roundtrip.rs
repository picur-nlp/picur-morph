//! The property that keeps the two directions honest: everything the engine generates must
//! analyse back to the stack that produced it.

use picur_morph_core::{Case, Morph, Number, Person, Stacks, Verdict, analyse_with, generate};
use picur_morph_lexicon::{Pos, lexicon};

/// The search space the round-trip uses.
fn stacks_list() -> Vec<Vec<Morph>> {
    let cases = [
        Case::Acc,
        Case::Dat,
        Case::Ins,
        Case::Ine,
        Case::Ill,
        Case::Ela,
        Case::Sup,
        Case::Sbl,
        Case::Del,
        Case::Ade,
        Case::All,
        Case::Abl,
    ];
    let mut out = vec![vec![Morph::Plural]];
    for c in cases {
        out.push(vec![Morph::Case(c)]);
        out.push(vec![Morph::Plural, Morph::Case(c)]);
    }
    for person in [Person::First, Person::Second, Person::Third] {
        for number in [Number::Sing, Number::Plur] {
            out.push(vec![Morph::Possessive { person, number }]);
        }
    }
    out
}

#[test]
fn generated_nominal_forms_analyse_back() {
    let lx = lexicon();
    let stacks = stacks();
    let lemmas = [
        "ház", "gáz", "bokor", "kéz", "dal", "bot", "lány", "alma", "orr", "kert",
    ];
    let mut checked = 0usize;
    for lemma in lemmas {
        let entry = lx
            .get_pos(lemma, Pos::Noun)
            .unwrap_or_else(|| panic!("{lemma} missing"));
        for stack in stacks.iter() {
            let Ok(surface) = generate(entry, stack) else {
                continue;
            };
            match analyse_with(&surface, &stacks) {
                Verdict::Known(readings) => {
                    assert!(
                        readings.iter().any(|r| r.lemma.as_ref() == lemma),
                        "{surface} did not analyse back to {lemma}: {readings:?}"
                    );
                    checked += 1;
                }
                other => panic!("{surface} (from {lemma}) analysed as {other:?}"),
            }
        }
    }
    assert!(checked > 200, "only {checked} round-trips checked");
}

#[test]
fn an_unlisted_but_well_formed_word_is_not_an_error() {
    assert_eq!(
        analyse_with("sztrimingelhetetlenség", &stacks()),
        Verdict::WellFormed
    );
}

#[test]
fn a_non_hungarian_shape_is_ill_formed() {
    assert_eq!(analyse_with("xyz123", &stacks()), Verdict::IllFormed);
    assert_eq!(analyse_with("", &stacks()), Verdict::IllFormed);
}

#[test]
fn a_known_stem_with_an_impossible_ending_is_ill_formed() {
    // `orrt` is the form NOM-ACC-006 forbids; nothing regenerates it, and the stem is known.
    assert_eq!(analyse_with("orrt", &stacks()), Verdict::IllFormed);
}

fn stacks() -> Stacks {
    stacks_list().into()
}
