//! End-to-end check of harmony plus the suffix inventory against real Hungarian forms.
//!
//! These are whole-word goldens: stem, case, and the suffix the language actually uses. They
//! exercise `picur-morph-ortho`'s `harmony_of` and `picur-morph-core`'s archiphoneme templates
//! together, which is the first point where a wrong rounding rule or a wrong template becomes
//! visible as a wrong Hungarian word.

#![expect(
    clippy::expect_used,
    reason = "a golden that cannot build its input must fail loudly"
)]

use picur_morph_core::{Case, case_suffix};
use picur_morph_ortho::{Harmony, harmony_of};

/// The suffix `stem` takes for `case`, by computed harmony.
fn suffix(stem: &str, case: Case) -> String {
    case_suffix(case)
        .expect("case has a suffix")
        .realise(harmony_of(stem))
}

#[test]
fn back_stems_take_back_suffixes() {
    assert_eq!(harmony_of("ház"), Harmony::Back);
    assert_eq!(suffix("ház", Case::Ine), "ban");
    assert_eq!(suffix("ház", Case::All), "hoz");
    assert_eq!(suffix("ház", Case::Ela), "ból");
    assert_eq!(suffix("ház", Case::Ade), "nál");
    assert_eq!(suffix("ház", Case::Dat), "nak");
}

#[test]
fn front_unrounded_stems_take_e_suffixes() {
    assert_eq!(harmony_of("kert"), Harmony::Front);
    assert_eq!(suffix("kert", Case::Ine), "ben");
    assert_eq!(suffix("kert", Case::All), "hez");
    assert_eq!(suffix("kert", Case::Ela), "ből");
    assert_eq!(suffix("kert", Case::Ade), "nél");
}

#[test]
fn front_rounded_stems_round_the_o_archiphoneme() {
    // PH-HARM-005: only `O` distinguishes rounded, so `-hOz` is the visible case while
    // `-bAn` and `-bOl` stay the same as plain front.
    assert_eq!(harmony_of("tök"), Harmony::FrontRounded);
    assert_eq!(suffix("tök", Case::All), "höz");
    assert_eq!(suffix("tök", Case::Ine), "ben");
    assert_eq!(suffix("tök", Case::Ela), "ből");

    assert_eq!(harmony_of("könyv"), Harmony::FrontRounded);
    assert_eq!(suffix("könyv", Case::All), "höz");
}

#[test]
fn rounding_follows_the_last_vowel_not_the_last_non_neutral_one() {
    // Trap: `döntés` has `ö` then `é`; the last vowel is `é`, so the word is plain front and the allative is `-hez`. Reading rounding off the last non-neutral vowel gives `döntéshöz`.
    assert_eq!(harmony_of("döntés"), Harmony::Front);
    assert_eq!(suffix("döntés", Case::All), "hez");

    assert_eq!(harmony_of("bővítés"), Harmony::Front);
    assert_eq!(suffix("bővítés", Case::All), "hez");

    // PH-HARM-004's own examples, which illustrate the front/back domain and NOT rounding.
    assert_eq!(harmony_of("fűszer"), Harmony::Front);
    assert_eq!(suffix("fűszer", Case::All), "hez");
    assert_eq!(harmony_of("römi"), Harmony::Front);
    assert_eq!(suffix("römi", Case::All), "hez");
}

#[test]
fn all_neutral_stems_are_front() {
    // PH-HARM-001: i, í, é are neutral; a word with only neutral vowels is front.
    assert_eq!(harmony_of("kép"), Harmony::Front);
    assert_eq!(suffix("kép", Case::Ine), "ben");
    assert_eq!(suffix("víz", Case::Ine), "ben");
}

#[test]
fn antiharmonic_stems_are_why_the_harmony_trait_exists() {
    // `híd` has only a neutral vowel, so the computed class is Front and `-ben`/`-nak` are wrong (`hídban`, `hídnak`); `Trait::Back` overrides it. This pins the computed default.
    assert_eq!(harmony_of("híd"), Harmony::Front);
    assert_eq!(suffix("híd", Case::Ine), "ben");
}

#[test]
fn invariant_suffixes_ignore_harmony() {
    for stem in ["ház", "kert", "tök"] {
        assert_eq!(suffix(stem, Case::Ter), "ig");
        assert_eq!(suffix(stem, Case::Cau), "ért");
        assert_eq!(suffix(stem, Case::Formalis), "ként");
        assert_eq!(suffix(stem, Case::Tem), "kor");
    }
}

#[test]
fn nominative_has_no_suffix() {
    assert!(case_suffix(Case::Nom).is_none());
}

#[test]
fn a_compound_harmonises_with_its_last_member_not_its_whole_lemma() {
    // DER-COMP-008: the last member is the harmony domain; `díj` is antiharmonic, so `nyugdíj` computes Back whole but Front from `díj`. The redundancy check must ask about the domain, else `nyugdíjet`/`nyugdíjbe`.
    for word in ["nyugdíjat", "nyugdíjba", "nyugdíjak", "díjat"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn a_cell_measured_as_taking_both_links_accepts_both() {
    // NOM-LOW-001's marking is per lexeme, and `öröm`'s accusative takes either: `örömet` 138,399
    // beside `örömöt` 77,324 [WC2]. Generation takes the winner and both are accepted.
    for word in ["örömet", "örömöt", "örömök", "örömek"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn an_inflected_stem_harmonises_on_itself() {
    // An antiharmonic marking says the lemma's spelling misleads; the suffix vowel does not, so the marking must not travel: `másképp` is front, not `másképpon`.
    for word in ["másképp", "másképpen", "mindenképp", "mindenképpen"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    // The seam does travel, because it is where the word is put together: `akik` harmonises on
    // `ki`, and `hidak` computes its own back class from the suffix vowel.
    for word in ["akiknek", "hidakat", "embereket", "házanként"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}
