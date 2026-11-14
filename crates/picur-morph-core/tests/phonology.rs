//! Chapter 02 seam phonology: the archaic name spellings and the silent coda `h`.

use picur_morph_core::{Case, Morph, generate};
use picur_morph_lexicon::{Pos, lexicon};

fn case_form(lemma: &str, case: Case) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Noun)
        .unwrap_or_else(|| panic!("{lemma} missing from the lexicon"));
    generate(e, &[Morph::Case(case)]).unwrap_or_else(|_| panic!("{lemma} takes no {case:?}"))
}

#[test]
fn an_archaic_name_assimilates_v_to_its_spoken_consonant() {
    // PH-G2P-004: `ts` is [cs], `ch` is [cs], `cz` and `tz` are [c], `gh` is [g].
    assert_eq!(case_form("Babits", Case::Ins), "Babitscsal");
    assert_eq!(case_form("Madách", Case::Tra), "Madáchcsá");
    assert_eq!(case_form("Móricz", Case::Ins), "Móriczcal");
    assert_eq!(case_form("Rátz", Case::Ins), "Rátzcal");
    assert_eq!(case_form("Végh", Case::Ins), "Véghgel");
    assert_eq!(case_form("Kossuth", Case::Ins), "Kossuthtal");
}

#[test]
fn an_unflagged_name_keeps_the_modern_reading_of_ch() {
    // PH-G2P-004 marks `ch` ambiguous, so only a flagged name reads it as [cs].
    assert_eq!(case_form("Zürich", Case::Ins), "Zürichhel");
}

#[test]
fn a_silent_coda_h_leaves_the_v_of_val_alone() {
    // PH-CONS-005: `csehvel`, `dühvel`, `pléhvé`; `dohhal` is the open default.
    assert_eq!(case_form("cseh", Case::Ins), "csehvel");
    assert_eq!(case_form("düh", Case::Ins), "dühvel");
    assert_eq!(case_form("pléh", Case::Tra), "pléhvé");
    assert_eq!(case_form("pléh", Case::Acc), "pléhet");
    assert_eq!(case_form("doh", Case::Ins), "dohhal");
}

#[test]
fn the_geminate_spelling_is_read_too() {
    // PH-CONS-005 leaves the spelling open for the `céh` group.
    for form in ["csehhel", "dühhel", "méhhé", "pléhhel"] {
        assert_eq!(
            picur_morph_spell::check(form),
            picur_morph_spell::Status::Ok,
            "{form}"
        );
    }
}

#[test]
fn a_word_final_y_is_a_vowel_for_val() {
    // PH-G2P-005: a final `y` that forms no digraph is [i] (`Adyval`, `whiskyvel`); where the
    // spelling is a digraph the consonant assimilates (`Kodállyal`, `Arannyal`).
    assert_eq!(case_form("Ady", Case::Ins), "Adyval");
    assert_eq!(case_form("whisky", Case::Ins), "whiskyvel");
    assert_eq!(case_form("Kodály", Case::Ins), "Kodállyal");
    assert_eq!(case_form("Arany", Case::Ins), "Arannyal");
}

#[test]
fn a_cluster_final_ik_verb_has_no_vowel_alternant() {
    // PH-ZERO-004: `csukl-`, `bűzl-` and `vedl-` are cluster stems, not epenthetic ones, so
    // `*csukolj` is not a form while the paradigm that needs no vowel is.
    assert_ne!(
        picur_morph_spell::check("csukolj"),
        picur_morph_spell::Status::Ok
    );
    for form in ["csuklik", "csuklott", "csuklani", "vedlik", "vedlett"] {
        assert_eq!(
            picur_morph_spell::check(form),
            picur_morph_spell::Status::Ok,
            "{form}"
        );
    }
}

#[test]
fn a_non_shortening_v_stem_is_read_both_ways() {
    // PH-VSTEM-001: `fő` and `nyű` add `v` without shortening in a measurable share of their
    // tokens, with no standard preference, so both alternants are words.
    for form in [
        "fővök", "fővöm", "fővi", "fővő", "nyűvök", "fövök", "fövő", "nyüvök",
    ] {
        assert_eq!(
            picur_morph_spell::check(form),
            picur_morph_spell::Status::Ok,
            "{form}"
        );
    }
}

#[test]
fn the_demonstratives_assimilate_before_a_semi_word() {
    // PH-ASSIM-003 puts `-fajta` and `-féle` beside `-kora`.
    for form in ["affajta", "afféle", "effajta", "efféle", "akkora", "ekkora"] {
        assert_eq!(
            picur_morph_spell::check(form),
            picur_morph_spell::Status::Ok,
            "{form}"
        );
    }
}

#[test]
fn a_foreign_spelling_assimilates_val_to_its_last_sound() {
    // PH-G2P-008: the sound the letters do not show drives the allomorph, the spelling stays.
    assert_eq!(case_form("hertz", Case::Ins), "hertzcel");
    assert_eq!(case_form("Wells", Case::Ins), "Wellsszel");
    assert_eq!(case_form("Greenwich", Case::Ins), "Greenwichcsel");
    assert_eq!(case_form("Balzac", Case::Ins), "Balzackal");
    assert_eq!(case_form("Steinbeck", Case::Ins), "Steinbeckkel");
}
