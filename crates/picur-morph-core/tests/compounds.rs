//! Compound-aware analysis: the false-rejection case a word list cannot cover.

use picur_morph_core::{
    Case, Definite, Mood, Morph, Number, Person, Stacks, Tense, Verdict, analyse, split,
};
use picur_morph_lexicon::lexicon;

/// The search space a checker would use for a nominal.
fn stacks_list() -> Vec<Vec<Morph>> {
    let cases = [
        Case::Acc,
        Case::Ine,
        Case::Dat,
        Case::Ins,
        Case::Sup,
        Case::All,
        Case::Ela,
        Case::Sbl,
    ];
    let mut out: Vec<Vec<Morph>> = vec![vec![], vec![Morph::Plural]];
    for c in cases {
        out.push(vec![Morph::Case(c)]);
        out.push(vec![Morph::Plural, Morph::Case(c)]);
    }
    out
}

fn known(word: &str) -> bool {
    matches!(analyse(word, &stacks()), Verdict::Known(_))
}

#[test]
fn unlisted_compounds_are_accepted() {
    // DER-COMP-021: compounds are productive, so these are ordinary words no word list contains.
    for word in ["vízbicikli", "macskaeledel", "almalé", "autókulcs"] {
        assert!(
            lexicon().get(word).is_empty(),
            "{word} is listed, so it does not test the compound path"
        );
        assert!(known(word), "{word} rejected");
    }
}

#[test]
fn an_unlisted_compound_inflects_on_its_head() {
    // DER-COMP-007: the members before the head are inert, so the head carries the suffix and the
    // head's own stem alternation still applies.
    for word in ["kutyaházat", "hóemberek", "asztallapon", "kertkapuval"] {
        assert!(lexicon().get(word).is_empty(), "{word} is listed");
        assert!(known(word), "{word} rejected");
    }
    assert!(known("vízsugarat"));
}

#[test]
fn the_head_lemma_is_reported_with_its_members() {
    let v = analyse("kutyaházat", &stacks());
    let Verdict::Known(readings) = v else {
        panic!("kutyaházat rejected");
    };
    assert!(
        readings.iter().any(|r| r.lemma.as_ref() == "kutyaház"),
        "{readings:?}"
    );
}

#[test]
fn harmony_errors_inside_a_compound_are_still_caught() {
    for word in ["almafaban", "asztallapen", "kutyaházot", "hóemberök"] {
        assert!(!known(word), "{word} wrongly accepted");
    }
}

#[test]
fn splitting_and_analysis_agree_on_what_is_a_compound() {
    for word in ["asztal", "rózsa", "kenyér"] {
        assert!(split(word).iter().all(|s| !s.is_compound()), "{word}");
        assert!(known(word));
    }
}

#[test]
fn a_suppletive_verb_is_recognised_through_its_own_stems() {
    // The stem index keys on the lemma, and no prefix of `ettem` resembles `eszik`, so without
    // the suppletive stems in the index these generate but never analyse.
    let stacks = Stacks::from(
        [(Person::First, Number::Sing), (Person::Third, Number::Sing)]
            .into_iter()
            .flat_map(|(person, number)| {
                [Definite::Ind, Definite::Def].into_iter().map(move |d| {
                    vec![Morph::Verbal {
                        mood: Mood::Ind,
                        tense: Tense::Past,
                        person,
                        number,
                        definite: d,
                    }]
                })
            })
            .collect::<Vec<Vec<Morph>>>(),
    );
    for word in ["ettem", "voltam", "mentem", "jöttem", "tettem", "ittam"] {
        assert!(
            matches!(analyse(word, &stacks), Verdict::Known(_)),
            "{word} not analysed"
        );
    }
}

#[test]
fn a_preverb_takes_an_inflected_verb() {
    // DER-COMP-022: the head is a finite form, not a lemma, so the constraint has to be the head's
    // category rather than a lemma lookup.
    let stacks = Stacks::from(vec![vec![Morph::Verbal {
        mood: Mood::Ind,
        tense: Tense::Past,
        person: Person::First,
        number: Number::Sing,
        definite: Definite::Ind,
    }]]);
    assert!(matches!(analyse("megettem", &stacks), Verdict::Known(_)));
}

#[test]
fn a_two_letter_preverb_is_a_member() {
    // The two-letter preverbs are a closed class and must pass `MIN_MEMBER_LETTERS`: `bej` is a noun and `ön` no suffix, so `bejön` parses only as `be` + `jön`.
    let stacks = Stacks::from(vec![vec![Morph::Verbal {
        mood: Mood::Ind,
        tense: Tense::Pres,
        person: Person::Third,
        number: Number::Sing,
        definite: Definite::Ind,
    }]]);
    for word in [
        "bejön", "elvisz", "kijön", "elmegy", "bemegy", "lemegy", "rájön", "átjön",
    ] {
        assert!(
            matches!(analyse(word, &stacks), Verdict::Known(_)),
            "{word} not analysed"
        );
    }
}

#[test]
fn a_preverb_does_not_open_a_nominal_compound() {
    // A preverb modifies a verb only; the `after_preverb` guard on nominal heads keeps `be` + `ház` from parsing.
    let stacks = Stacks::from(vec![vec![], vec![Morph::Plural]]);
    for word in [
        "elalma",
        "beház",
        "elkutya",
        "beablak",
        "bealma",
        "elablakok",
    ] {
        assert!(
            !matches!(analyse(word, &stacks), Verdict::Known(_)),
            "{word} wrongly analysed"
        );
    }
}

#[test]
fn a_hyphenated_compound_analyses_like_a_solid_one() {
    // ORT-TOG-024 writes a hyphen into a compound of six syllables or more and ORT-TOG-030 into
    // one of three or more members, so a hyphenated compound is as productive as a solid one and
    // no word list covers it. The head carries the inflection.
    for word in ["élelmiszer-ipar", "élelmiszer-ipari", "dél-afrikai"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    assert!(
        picur_morph_spell::analyses("élelmiszer-ipari")
            .iter()
            .any(|a| a.lemma.as_ref() == "élelmiszer-ipar")
    );
}

#[test]
fn a_tokenised_off_clitic_is_a_word() {
    // A tokeniser that splits SYN-Q-002's clitic leaves it as a token of its own, which is how UD
    // writes it: `-e`, tagged an interrogative adverb with itself as the lemma.
    assert!(
        picur_morph_spell::analyses("-e")
            .iter()
            .any(|a| a.lemma.as_ref() == "-e")
    );
}

fn stacks() -> Stacks {
    stacks_list().into()
}

#[test]
fn a_verb_takes_one_preverb() {
    // A preverb never stacks on an already prefixed verb, so `elé` on `beáll` is no word; the
    // lexicon marks the two that do (`újra`, `tovább`).
    let stacks = Stacks::from(vec![
        vec![Morph::Verbal {
            mood: Mood::Ind,
            tense: Tense::Past,
            person: Person::Third,
            number: Number::Sing,
            definite: Definite::Ind,
        }],
        vec![],
    ]);
    for word in ["elébeállt", "megbeáll", "kibelép"] {
        assert!(
            !matches!(analyse(word, &stacks), Verdict::Known(_)),
            "{word} analysed"
        );
    }
    assert!(known("továbbfeldolgoz"));
    assert!(known("újraeloszt"));
}

#[test]
fn a_name_is_not_the_head_of_a_coined_compound() {
    // ORT-NAME-023: a common member before a name takes a hyphen, so a solid reading through a
    // capitalised head is not a word: `oldalrolis` is not `oldal` before `Roli` with `-s`.
    for word in ["oldalrolis", "blogolás", "torinói"] {
        assert!(!known(word), "{word} analysed");
    }
}
