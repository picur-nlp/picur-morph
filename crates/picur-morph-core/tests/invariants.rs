//! Invariants that hold of every entry and every feature stack.
//!
//! The seam code slices by byte offset in several places (assimilation, gemination, the
//! demonstrative `z`, the digraph seam), and a wrong offset is a panic rather than a wrong answer.
//! These sweep widely enough to catch one.

use picur_morph_core::{
    Case, DerId, Mood, Morph, Number, Person, Tense, generate, member_prefixes, split,
};
use picur_morph_lexicon::lexicon;

/// A wide slice of the feature model: every case, the derivations, gradation and the verbal cells.
fn every_stack() -> Vec<Vec<Morph>> {
    let cases = [
        Case::Acc,
        Case::Ins,
        Case::Tra,
        Case::Sup,
        Case::All,
        Case::Ine,
        Case::Dat,
        Case::Cau,
        Case::Formalis,
        Case::Sociativus,
    ];
    let mut out: Vec<Vec<Morph>> = vec![
        vec![],
        vec![Morph::Plural],
        vec![Morph::Comparative],
        vec![Morph::Superlative],
        vec![Morph::Excessive],
        vec![Morph::Anaphoric],
        vec![Morph::Associative],
        vec![Morph::Collective],
        vec![Morph::Multiplicative],
        vec![Morph::PossessedPlural],
        vec![Morph::Infinitive],
        vec![Morph::PresentParticiple],
        vec![Morph::PastParticiple],
        vec![Morph::Converb],
        vec![Morph::Potential],
    ];
    for c in cases {
        out.push(vec![Morph::Case(c)]);
        out.push(vec![Morph::Plural, Morph::Case(c)]);
        out.push(vec![Morph::Comparative, Morph::Case(c)]);
    }
    for id in DerId::ALL {
        out.push(vec![Morph::Derive(id)]);
        out.push(vec![Morph::Derive(id), Morph::Plural]);
    }
    for person in [Person::First, Person::Third] {
        for number in [Number::Sing, Number::Plur] {
            out.push(vec![Morph::Possessive { person, number }]);
            out.push(vec![
                Morph::PossessedPlural,
                Morph::Possessive { person, number },
            ]);
            for mood in [Mood::Ind, Mood::Cnd, Mood::Imp] {
                out.push(vec![Morph::Verbal {
                    mood,
                    tense: Tense::Pres,
                    person,
                    number,
                    definite: picur_morph_core::Definite::Def,
                }]);
            }
        }
    }
    out
}

#[test]
fn generation_never_panics_and_never_returns_an_empty_form() {
    let stacks = every_stack();
    let mut produced = 0usize;
    for (i, entry) in lexicon().iter().enumerate() {
        if !i.is_multiple_of(11) {
            continue;
        }
        for stack in &stacks {
            if let Ok(form) = generate(entry, stack) {
                assert!(
                    !form.is_empty(),
                    "{} with {stack:?} produced an empty form",
                    entry.lemma
                );
                produced += 1;
            }
        }
    }
    assert!(produced > 100_000, "sample too small: {produced}");
}

#[test]
fn a_split_accounts_for_every_character_of_the_word() {
    for (i, entry) in lexicon().iter().enumerate() {
        if !i.is_multiple_of(11) {
            continue;
        }
        for candidate in split(&entry.lemma) {
            let joined: String = candidate.members.iter().map(|m| m.text.as_ref()).collect();
            assert_eq!(
                joined,
                entry.lemma.as_ref(),
                "split of {} does not reassemble",
                entry.lemma
            );
        }
    }
}

#[test]
fn member_prefix_offsets_are_character_boundaries() {
    // The offsets are used to slice the word, so a wrong one is a panic.
    for (i, entry) in lexicon().iter().enumerate() {
        if !i.is_multiple_of(11) {
            continue;
        }
        for (members, at) in member_prefixes(&entry.lemma) {
            assert!(
                entry.lemma.is_char_boundary(at),
                "{} offset {at} is not a boundary",
                entry.lemma
            );
            let joined: String = members.iter().map(|m| m.text.as_ref()).collect();
            assert_eq!(joined, entry.lemma[..at], "{} prefix mismatch", entry.lemma);
        }
    }
}
