//! The demonstrative paradigm, which is one assimilation rule rather than a table.

use picur_morph_core::{Case, Morph, generate};
use picur_morph_lexicon::{Pos, lexicon};

fn form(lemma: &str, stack: &[Morph]) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Pronoun)
        .unwrap_or_else(|| panic!("{lemma} missing"));
    generate(e, stack).unwrap_or_else(|x| panic!("{lemma}: {x}"))
}

fn case(lemma: &str, c: Case) -> String {
    form(lemma, &[Morph::Case(c)])
}

#[test]
fn z_assimilates_to_a_consonant_initial_suffix() {
    // PND-DEM-002.
    assert_eq!(case("ez", Case::Ine), "ebben");
    assert_eq!(case("az", Case::Ine), "abban");
    assert_eq!(case("ez", Case::Sbl), "erre");
    assert_eq!(case("az", Case::Sbl), "arra");
    assert_eq!(case("ez", Case::All), "ehhez");
    assert_eq!(case("az", Case::All), "ahhoz");
    assert_eq!(case("ez", Case::Dat), "ennek");
    assert_eq!(case("az", Case::Dat), "annak");
    assert_eq!(case("ez", Case::Abl), "ettől");
    assert_eq!(case("az", Case::Ela), "abból");
    assert_eq!(case("ez", Case::Del), "erről");
    assert_eq!(case("az", Case::Ade), "annál");
}

#[test]
fn the_exempt_suffixes_do_not_assimilate() {
    // PND-DEM-003: the accusative, and anything the linking vowel makes vowel-initial.
    assert_eq!(case("ez", Case::Acc), "ezt");
    assert_eq!(case("az", Case::Acc), "azt");
    assert_eq!(case("ez", Case::Sup), "ezen");
    assert_eq!(case("az", Case::Sup), "azon");
    assert_eq!(case("ez", Case::Cau), "ezért");
    assert_eq!(case("az", Case::Cau), "azért");
    assert_eq!(form("ez", &[Morph::Plural]), "ezek");
    assert_eq!(form("az", &[Morph::Plural]), "azok");
}

#[test]
fn the_instrumental_comes_out_of_the_general_v_assimilation() {
    // PND-DEM-004: `ezzel` is reached by assimilating the suffix's `v` to the stem's `z`, which is
    // the same rule that gives `házzal`. The variant `evvel` is not generated.
    assert_eq!(case("ez", Case::Ins), "ezzel");
    assert_eq!(case("az", Case::Ins), "azzal");
    assert_eq!(case("ez", Case::Tra), "ezzé");
    assert_eq!(case("az", Case::Tra), "azzá");
}

#[test]
fn the_rule_carries_to_every_demonstrative_in_z() {
    // PND-DEM-002's own examples.
    assert_eq!(case("amaz", Case::Dat), "amannak");
    assert_eq!(case("mindez", Case::Abl), "mindettől");
    assert_eq!(case("ugyanaz", Case::All), "ugyanahhoz");
    assert_eq!(case("ugyanaz", Case::Ine), "ugyanabban");
    assert_eq!(case("mindez", Case::Ine), "mindebben");
    assert_eq!(case("mindez", Case::Ins), "mindezzel");
    assert_eq!(case("ugyanaz", Case::Ins), "ugyanazzal");
}

#[test]
fn a_plural_demonstrative_inflects_regularly() {
    // PND-DEM-003: the plural stem is an ordinary one, so nothing assimilates after it.
    assert_eq!(
        form("ez", &[Morph::Plural, Morph::Case(Case::Acc)]),
        "ezeket"
    );
    assert_eq!(
        form("az", &[Morph::Plural, Morph::Case(Case::Acc)]),
        "azokat"
    );
    assert_eq!(
        form("ez", &[Morph::Plural, Morph::Case(Case::Dat)]),
        "ezeknek"
    );
    assert_eq!(
        form("az", &[Morph::Plural, Morph::Case(Case::Ine)]),
        "azokban"
    );
    assert_eq!(
        form("az", &[Morph::Plural, Morph::Case(Case::Ins)]),
        "azokkal"
    );
}

#[test]
fn the_reflexive_lemma_is_its_third_singular() {
    // PND-REFL-001 builds the reflexive on the stem `mag-`, which is not a word, so the stem
    // cannot be the lemma of `magam` or `maguk`.
    for (word, want) in [
        ("magam", "maga"),
        ("magunk", "maga"),
        ("maguk", "maga"),
        ("magukat", "maga"),
        ("önmagam", "önmaga"),
    ] {
        let readings = picur_morph_spell::analyses(word);
        assert!(
            readings.iter().any(|a| a.lemma.as_ref() == want),
            "{word} has no {want} reading: {:?}",
            readings.iter().map(|a| &a.lemma).collect::<Vec<_>>()
        );
    }
}

#[test]
fn the_independent_possessive_pronouns_are_a_closed_table() {
    // PND-REFL-005 states them: suppletive in the first and second person and `ő` + `-é` in the
    // third, with `-i-` before the person suffix for several possessed. Nothing in the lexicon
    // reaches them, and the rule sets the doublet policy: generate the long form, accept both.
    for word in [
        "enyém", "enyéim", "tiéd", "tied", "tieid", "övé", "övéi", "miénk", "mienk", "tiétek",
        "övék", "magáé",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    // PND-PERS-005 has them behave as nouns once formed, so the ordinary cases apply.
    for word in ["enyémet", "övének", "miénkben"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    assert!(
        picur_morph_spell::analyses("tied")
            .iter()
            .any(|a| a.lemma.as_ref() == "tiéd")
    );
}

#[test]
fn a_demonstrative_and_a_postposition_are_written_together() {
    // PND-DEM-009 states the form as a rule, not as its own 30-item list: `a`/`e` before a
    // consonant-initial postposition and `az`/`ez` before a vowel-initial one.
    for word in [
        "amiatt",
        "emiatt",
        "emellett",
        "ezelőtt",
        "azóta",
        "anélkül",
        "eszerint",
        "afelé",
        "ahelyett",
        "ezután",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    for word in ["azmiatt", "emellettt"] {
        assert_ne!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn a_relative_pronoun_harmonises_on_its_interrogative() {
    // PND-REL-001's paradigm. `aki` is `a` plus `ki`, so DER-COMP-008 makes `ki` the harmony
    // domain and the suffixes are front despite the back `a`. The seam has to travel onto the
    // inflected stem or `akik` harmonises on the whole string: `akiknak` for `akiknek`.
    for word in [
        "akinek", "akiknek", "akiket", "akik", "akit", "aminek", "amiknek", "amelyek",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn a_prefixed_pronoun_series_is_computed_not_enumerated() {
    // PND-INDF-001: seven prefixes over seventeen bases, each through its whole paradigm; a rule, not a list.
    for word in [
        "valamilyen",
        "akármilyen",
        "semmilyen",
        "bárhány",
        "valamekkora",
        "akármeddig",
        "néhol",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    // The base is analysed on its own, so its paradigm and its harmony come for free: `valakinek`
    // is `vala` plus the dative of `ki`, and `ki` is front whatever the prefix's back `a` suggests.
    for word in ["valakinek", "bárkivel", "akárkinek", "valamiket"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    assert!(
        picur_morph_spell::analyses("valamilyen")
            .iter()
            .any(|a| a.lemma.as_ref() == "valamilyen")
    );
}

#[test]
fn the_ilyen_accusative_truncates() {
    // PND-DEM-008 states it as a rule over every lemma ending in `ilyen`/`olyan`, which covers
    // `milyen`, `amilyen` and the prefixed forms without naming them, and names the truncated
    // form as the one to generate: `ilyet` and `olyat` beat `ilyent` and `olyant` about 23 to 1.
    for word in [
        "ilyet",
        "olyat",
        "milyet",
        "amilyet",
        "bármilyet",
        "ugyanolyat",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    // The untruncated accusative is the attested variant, and the other cases use the full stem.
    for word in ["ilyent", "olyant", "ilyenek", "olyannak", "ilyenben"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn the_relative_series_is_the_same_cross_product() {
    // PND-REL-001 builds on the same PND-INT-001 bases with `a-`.
    for word in ["ahol", "amikor", "ahány", "ameddig", "amekkora", "amilyen"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn the_interrogative_inventory_is_complete() {
    // PND-INT-001's inventory, and PND-INDF-001 builds every prefixed series on it, so a missing
    // base takes its whole series with it. morphdb has none of `hová`, `meddig` or `miféle`.
    for word in [
        "ki", "mi", "mely", "melyik", "milyen", "miféle", "mekkora", "mennyi", "hány", "hol",
        "hová", "honnan", "mikor", "hogyan", "merre", "meddig",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    for word in ["valahová", "bárhová", "ameddig", "valamiféle", "semmiféle"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn a_sentence_initial_capital_keeps_both_readings() {
    // A capitalised token may have a capitalised homograph, so the lowercased readings are merged
    // rather than tried only when nothing else was found: `Ebben` parses as the abbreviation `Eb`
    // plus the inessive, which hid the `ez` it almost always is.
    let readings = picur_morph_spell::analyses("Ebben");
    for want in ["Eb", "ez"] {
        assert!(
            readings.iter().any(|a| a.lemma.as_ref() == want),
            "Ebben has no {want} reading: {:?}",
            readings.iter().map(|a| &a.lemma).collect::<Vec<_>>()
        );
    }
    // The reverse is never tried, because a lowercase proper name really is an error.
    assert_ne!(
        picur_morph_spell::check("európa"),
        picur_morph_spell::Status::Ok
    );
}
