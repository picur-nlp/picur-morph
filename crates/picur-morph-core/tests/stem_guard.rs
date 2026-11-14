//! The search filter must never reject a form the engine can actually generate.
//!
//! `analysis` skips a lexicon entry whose stem guard is not a prefix of the surface, which is what
//! makes the search fast enough to be a spellchecker. If the guard is ever wrong the word is not
//! merely slow to find, it is silently unfindable, so the invariant is checked directly rather
//! than inferred from a corpus.

use picur_morph_core::{
    Case, Definite, Mood, Morph, Number, Person, Stacks, Tense, Verdict, analyse_with, generate,
};
use picur_morph_lexicon::{Pos, lexicon};

fn stacks_list() -> Vec<Vec<Morph>> {
    let mut out: Vec<Vec<Morph>> = vec![vec![], vec![Morph::Plural]];
    for c in [
        Case::Acc,
        Case::Ine,
        Case::Ins,
        Case::All,
        Case::Dat,
        Case::Sup,
    ] {
        out.push(vec![Morph::Case(c)]);
        out.push(vec![Morph::Plural, Morph::Case(c)]);
    }
    for person in [Person::First, Person::Third] {
        for number in [Number::Sing, Number::Plur] {
            out.push(vec![Morph::Possessive { person, number }]);
        }
    }
    out
}

#[test]
fn every_generated_form_is_found_again() {
    let stacks = stacks();
    let mut checked = 0usize;
    let mut missed: Vec<String> = Vec::new();
    for (i, entry) in lexicon().iter().enumerate() {
        if !i.is_multiple_of(97) {
            continue;
        }
        for stack in stacks.iter() {
            let Ok(form) = generate(entry, stack) else {
                continue;
            };
            checked += 1;
            if !matches!(analyse_with(&form, &stacks), Verdict::Known(_)) {
                missed.push(format!("{} -> {form}", entry.lemma));
            }
        }
    }
    assert!(checked > 5_000, "sample too small: {checked}");
    assert!(
        missed.is_empty(),
        "{} of {checked} generated forms could not be analysed, e.g. {:?}",
        missed.len(),
        &missed[..missed.len().min(10)]
    );
}

/// Every finite cell, plus the nonfinites.
fn verbal_stacks_list() -> Vec<Vec<Morph>> {
    let mut out = Vec::new();
    for person in [Person::First, Person::Second, Person::Third] {
        for number in [Number::Sing, Number::Plur] {
            for mood in [Mood::Ind, Mood::Cnd, Mood::Imp] {
                for tense in [Tense::Pres, Tense::Past] {
                    if tense == Tense::Past && mood != Mood::Ind {
                        continue;
                    }
                    for definite in [Definite::Ind, Definite::Def, Definite::Two] {
                        out.push(vec![Morph::Verbal {
                            mood,
                            tense,
                            person,
                            number,
                            definite,
                        }]);
                    }
                }
            }
        }
    }
    for m in [
        Morph::Infinitive,
        Morph::PresentParticiple,
        Morph::PastParticiple,
        Morph::Converb,
    ] {
        out.push(vec![m]);
        out.push(vec![Morph::Potential, m]);
    }
    out
}

#[test]
fn every_generated_verb_form_is_found_again() {
    // Verbal cells outnumber nominal ones and use three stem systems (present, bound, suppletive).
    let stacks = verbal_stacks();
    let mut checked = 0usize;
    let mut missed: Vec<String> = Vec::new();
    for (i, entry) in lexicon().iter().enumerate() {
        if entry.pos != Pos::Verb || !i.is_multiple_of(89) {
            continue;
        }
        for stack in stacks.iter() {
            let Ok(form) = generate(entry, stack) else {
                continue;
            };
            checked += 1;
            if !matches!(analyse_with(&form, &stacks), Verdict::Known(_)) {
                missed.push(format!("{} -> {form}", entry.lemma));
            }
        }
    }
    assert!(checked > 5_000, "sample too small: {checked}");
    assert!(
        missed.is_empty(),
        "{} of {checked} generated verb forms could not be analysed, e.g. {:?}",
        missed.len(),
        &missed[..missed.len().min(10)]
    );
}

#[test]
fn irregular_and_ik_verbs_survive_the_guard() {
    let stacks = stacks();
    for lemma in ["eszik", "alszik", "közeledik", "dolgozik", "van", "megy"] {
        let Some(entry) = lexicon().get_pos(lemma, Pos::Verb) else {
            continue;
        };
        let form = generate(entry, &[]).expect("bare form");
        assert!(
            matches!(analyse_with(&form, &stacks), Verdict::Known(_)),
            "{lemma} -> {form} not analysed"
        );
    }
}

#[test]
fn an_szt_verb_is_reachable_from_its_imperative() {
    // VRB-IMP-004 rewrites the `szt`/`st` cluster, so the index key must stop before it: keyed on `osz`, `oszt` is unreachable from `ossza`.
    let stacks = verbal_stacks();
    for form in [
        "ossza",
        "osszon",
        "osszuk",
        "válassz",
        "válassza",
        "fess",
        "fesse",
    ] {
        assert!(
            matches!(analyse_with(form, &stacks), Verdict::Known(_)),
            "{form} not analysed"
        );
    }
}

fn stacks() -> Stacks {
    stacks_list().into()
}

fn verbal_stacks() -> Stacks {
    verbal_stacks_list().into()
}
