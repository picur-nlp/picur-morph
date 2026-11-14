//! Whole-word goldens driven by the committed lexicon rather than hand-set traits.
//!
//! These are the forms a user would actually ask for, generated from the lemma alone. They are the
//! end-to-end check that the lexicon, the alternation rules and the suffix inventory agree.

#![expect(
    clippy::expect_used,
    reason = "a golden that cannot find its lemma must fail loudly"
)]

use picur_morph_core::{Case, Morph, Number, Person, generate};
use picur_morph_lexicon::{Pos, lexicon};

fn form(lemma: &str, stack: &[Morph]) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Noun)
        .unwrap_or_else(|| panic!("{lemma} missing from the lexicon"));
    generate(e, stack).expect("generates")
}

const ACC: &[Morph] = &[Morph::Case(Case::Acc)];
const PL: &[Morph] = &[Morph::Plural];

#[test]
fn the_orr_case() {
    // NOM-ACC-006 forbids `orrt` (corpus 8,930 : 2). morphdb carries both `bareT` and
    // `non_bareT` for this lemma, so the tie is resolved against the rulebook at conversion time.
    assert_eq!(form("orr", ACC), "orrot");
}

#[test]
fn lowering_contrast_from_real_entries() {
    assert_eq!(form("ház", ACC), "házat");
    assert_eq!(form("gáz", ACC), "gázt");
    assert_eq!(form("ház", PL), "házak");
    assert_eq!(form("gáz", PL), "gázok");
}

#[test]
fn alternating_stems_from_real_entries() {
    assert_eq!(form("bokor", ACC), "bokrot");
    assert_eq!(form("bokor", PL), "bokrok");
    assert_eq!(form("kéz", ACC), "kezet");
    assert_eq!(form("kéz", PL), "kezek");
}

#[test]
fn bare_and_linked_accusatives_from_real_entries() {
    assert_eq!(form("dal", ACC), "dalt");
    assert_eq!(form("bot", ACC), "botot");
    assert_eq!(form("lány", ACC), "lányt");
}

#[test]
fn instrumental_assimilation_from_real_entries() {
    assert_eq!(form("ház", &[Morph::Case(Case::Ins)]), "házzal");
    assert_eq!(form("lány", &[Morph::Case(Case::Ins)]), "lánnyal");
}

#[test]
fn case_paradigm_from_a_real_entry() {
    assert_eq!(form("ház", &[Morph::Case(Case::Ine)]), "házban");
    assert_eq!(form("ház", &[Morph::Case(Case::All)]), "házhoz");
    assert_eq!(form("ház", &[Morph::Case(Case::Sup)]), "házon");
    assert_eq!(
        form("ház", &[Morph::Plural, Morph::Case(Case::Ine)]),
        "házakban"
    );
}

#[test]
fn possessive_paradigm_after_a_vowel() {
    use picur_morph_core::{Number, Person};
    let p = |person, number| form("hajó", &[Morph::Possessive { person, number }]);
    assert_eq!(p(Person::First, Number::Sing), "hajóm");
    assert_eq!(p(Person::Second, Number::Sing), "hajód");
    assert_eq!(p(Person::Third, Number::Sing), "hajója");
    assert_eq!(p(Person::First, Number::Plur), "hajónk");
    assert_eq!(p(Person::Second, Number::Plur), "hajótok");
    assert_eq!(p(Person::Third, Number::Plur), "hajójuk");
}

#[test]
fn possessive_linking_vowel_follows_lowering() {
    use picur_morph_core::{Number, Person};
    let p = |lemma: &str, person, number| form(lemma, &[Morph::Possessive { person, number }]);
    // NOM-POSS-002: ház is lowering, pad is not.
    assert_eq!(p("ház", Person::First, Number::Sing), "házam");
    assert_eq!(p("pad", Person::First, Number::Sing), "padom");
    assert_eq!(p("ház", Person::Second, Number::Plur), "házatok");
    // NOM-POSS-003: 1pl ignores lowering.
    assert_eq!(p("ház", Person::First, Number::Plur), "házunk");
    assert_eq!(p("kert", Person::First, Number::Plur), "kertünk");
}

#[test]
fn third_person_a_versus_ja() {
    use picur_morph_core::{Number, Person};
    let p = |lemma: &str| {
        form(
            lemma,
            &[Morph::Possessive {
                person: Person::Third,
                number: Number::Sing,
            }],
        )
    };
    // NOM-POSS-005: categorical -a after sibilants and palatals.
    assert_eq!(p("sas"), "sasa");
    assert_eq!(p("rész"), "része");
    assert_eq!(p("kulcs"), "kulcsa");
    assert_eq!(p("olaj"), "olaja");
}

#[test]
fn comparative_anaphoric_and_associative() {
    // NOM-ADJ-004, NOM-ANP-001, NOM-ASSOC-001. All three lengthen a final a/e.
    assert_eq!(form("ház", &[Morph::Anaphoric]), "házé");
    assert_eq!(form("ház", &[Morph::Plural, Morph::Anaphoric]), "házaké");
    assert_eq!(form("szomszéd", &[Morph::Associative]), "szomszédék");
}

#[test]
fn possessed_plural_repeats_the_third_singular_choice() {
    use picur_morph_core::{Number, Person};
    let pp = |lemma: &str, person, number| {
        form(
            lemma,
            &[Morph::PossessedPlural, Morph::Possessive { person, number }],
        )
    };
    assert_eq!(pp("ház", Person::First, Number::Sing), "házaim");
    assert_eq!(pp("ház", Person::Second, Number::Sing), "házaid");
    assert_eq!(pp("hajó", Person::First, Number::Sing), "hajóim");
    assert_eq!(pp("alma", Person::First, Number::Sing), "almáim");
}

#[test]
fn a_shortening_stem_keeps_its_long_vowel_before_the_superessive() {
    // NOM-STEM-006. The superessive is the one class I suffix that takes the free stem, so the
    // same lemma gives `utat` and `utak` but `úton`. Corpus, free against bound: `úton`
    // 1,275,847 : `uton` 17,480; `kézen` 39,345 : `kezen` 514; `nyáron` 605,018 : `nyaron` 5,328
    // [WC2].
    for (lemma, want) in [
        ("út", "úton"),
        ("hét", "héten"),
        ("tér", "téren"),
        ("kéz", "kézen"),
        ("víz", "vízen"),
        ("tél", "télen"),
        ("nyár", "nyáron"),
    ] {
        assert_eq!(form(lemma, &[Morph::Case(Case::Sup)]), want, "{lemma}");
    }
}

#[test]
fn the_other_alternations_still_bind_before_the_superessive() {
    // Only shortening is exempt. The epenthetic, v- and oa-stems take the bound alternant here.
    for (lemma, want) in [("bokor", "bokron"), ("ló", "lovon"), ("tó", "tavon")] {
        assert_eq!(form(lemma, &[Morph::Case(Case::Sup)]), want, "{lemma}");
    }
}

#[test]
fn the_ajto_type_alternates_only_in_the_third_person_possessive() {
    // NOM-STEM-013 confines the `ó`/`ő` to `a`/`e` plus `j` change to 3sg and 3pl; v-stem marking would spread it
    // to every cell (`idevet` for `időt`).
    for (lemma, acc, plural, first, third, third_pl) in [
        ("idő", "időt", "idők", "időm", "ideje", "idejük"),
        ("erő", "erőt", "erők", "erőm", "ereje", "erejük"),
        ("ajtó", "ajtót", "ajtók", "ajtóm", "ajtaja", "ajtajuk"),
        ("tető", "tetőt", "tetők", "tetőm", "teteje", "tetejük"),
        ("vő", "vőt", "vők", "vőm", "veje", "vejük"),
        ("első", "elsőt", "elsők", "elsőm", "elseje", "elsejük"),
        ("tüdő", "tüdőt", "tüdők", "tüdőm", "tüdeje", "tüdejük"),
        (
            "zászló",
            "zászlót",
            "zászlók",
            "zászlóm",
            "zászlaja",
            "zászlajuk",
        ),
    ] {
        assert_eq!(form(lemma, &[Morph::Case(Case::Acc)]), acc, "{lemma} acc");
        assert_eq!(form(lemma, &[Morph::Plural]), plural, "{lemma} plural");
        assert_eq!(
            form(
                lemma,
                &[Morph::Possessive {
                    person: Person::First,
                    number: Number::Sing
                }]
            ),
            first,
            "{lemma} 1sg"
        );
        assert_eq!(
            form(
                lemma,
                &[Morph::Possessive {
                    person: Person::Third,
                    number: Number::Sing
                }]
            ),
            third,
            "{lemma} 3sg"
        );
        assert_eq!(
            form(
                lemma,
                &[Morph::Possessive {
                    person: Person::Third,
                    number: Number::Plur
                }]
            ),
            third_pl,
            "{lemma} 3pl"
        );
    }
}

#[test]
fn a_real_v_stem_still_alternates_everywhere() {
    // The `ajtó` type must not be confused with the v-stems, which alternate in every class I cell.
    for (lemma, acc, plural, third) in [
        ("kő", "követ", "kövek", "köve"),
        ("tó", "tavat", "tavak", "tava"),
        ("ló", "lovat", "lovak", "lova"),
        ("fű", "füvet", "füvek", "füve"),
        ("hó", "havat", "havak", "hava"),
        // NOM-STEM-012: the `v` replaces the final vowel here rather than following it. `daru`
        // and `falu` keep the free stem in the accusative, which is measured rather than assumed:
        // `darut` 2,710 against `darvat` 124, `falut` 42,879 against `falvat` 279 [WC2].
        // The possessive allomorph is measured too, and it takes the lemma with it: `-ja` is
        // consonant-initial so no v-stem alternant is selected. `faluja` is 7,589 against no
        // `falva` of this lemma at all, the `falva` tokens being the verb `fal` [WC2].
        ("falu", "falut", "falvak", "faluja"),
        ("tetű", "tetvet", "tetvek", "tetve"),
    ] {
        assert_eq!(form(lemma, &[Morph::Case(Case::Acc)]), acc, "{lemma} acc");
        assert_eq!(form(lemma, &[Morph::Plural]), plural, "{lemma} plural");
        assert_eq!(
            form(
                lemma,
                &[Morph::Possessive {
                    person: Person::Third,
                    number: Number::Sing
                }]
            ),
            third,
            "{lemma} 3sg"
        );
    }
}

/// `PH-VSTEM-004`: the high-vowel-to-v stems keep the vowel stem before the personal
/// possessives and take `v` before the plurals (`falum` 3,585 : `falvam` 17, `falvai` 6,153 :
/// `falui` 92 [WC2]).
#[test]
fn a_high_vowel_v_stem_alternates_only_before_the_plurals() {
    let poss = |person, number| Morph::Possessive { person, number };
    for (lemma, first, first_pl, possessed_pl) in [
        ("falu", "falum", "falunk", "falvai"),
        ("daru", "darum", "darunk", "darvai"),
        ("tetű", "tetűm", "tetűnk", "tetvei"),
    ] {
        assert_eq!(form(lemma, &[poss(Person::First, Number::Sing)]), first);
        assert_eq!(form(lemma, &[poss(Person::First, Number::Plur)]), first_pl);
        assert_eq!(
            form(
                lemma,
                &[Morph::PossessedPlural, poss(Person::Third, Number::Sing)]
            ),
            possessed_pl
        );
    }
}

#[test]
fn the_rounded_link_survives_the_lexicon() {
    // PH-HARM-004. `Trait::Front` means front harmony against the spelling, so marking a front-rounded spelling
    // with it is destructive: it downgrades the rounding (`csütörtöken` for `csütörtökön`).
    for (lemma, want) in [
        ("csütörtök", "csütörtökön"),
        ("tükör", "tükrön"),
        ("öröm", "örömön"),
        ("sör", "sörön"),
        ("füzet", "füzeten"),
        ("kert", "kerten"),
    ] {
        assert_eq!(form(lemma, &[Morph::Case(Case::Sup)]), want, "{lemma}");
    }
}

#[test]
fn the_kinship_stems_truncate_before_the_third_person() {
    // NOM-STEM-014: a truncated alternant before `-jA`/`-jU-` and the lengthened stem elsewhere.
    // The truncated stem ends in `ny` or `ty`, which the categorical class of NOM-POSS-005 would
    // refuse `-jA` after, so the rulebook's own table is what settles `anyja` and `bátyja`.
    for (lemma, first, third, first_pl, third_pl) in [
        ("apa", "apám", "apja", "apánk", "apjuk"),
        ("anya", "anyám", "anyja", "anyánk", "anyjuk"),
        ("bátya", "bátyám", "bátyja", "bátyánk", "bátyjuk"),
    ] {
        let poss = |person, number| form(lemma, &[Morph::Possessive { person, number }]);
        assert_eq!(poss(Person::First, Number::Sing), first, "{lemma} 1sg");
        assert_eq!(poss(Person::Third, Number::Sing), third, "{lemma} 3sg");
        assert_eq!(poss(Person::First, Number::Plur), first_pl, "{lemma} 1pl");
        assert_eq!(poss(Person::Third, Number::Plur), third_pl, "{lemma} 3pl");
    }
}

#[test]
fn the_possessed_plural_inflects_and_lowers() {
    // NOM-PPL-001: the possessed plural is a stem like any other and takes case. NOM-LOW-002 makes an inflected stem
    // lowering, so `adat` gives `adataimat` though it is not itself a lowering stem.
    let ppl = Morph::PossessedPlural;
    let first = Morph::Possessive {
        person: Person::First,
        number: Number::Sing,
    };
    for (lemma, plain, accusative, mine, mine_acc) in [
        ("adat", "adatai", "adatait", "adataim", "adataimat"),
        ("ház", "házai", "házait", "házaim", "házaimat"),
        ("könyv", "könyvei", "könyveit", "könyveim", "könyveimet"),
    ] {
        assert_eq!(form(lemma, &[ppl]), plain, "{lemma}");
        assert_eq!(
            form(lemma, &[ppl, Morph::Case(Case::Acc)]),
            accusative,
            "{lemma} acc"
        );
        assert_eq!(form(lemma, &[ppl, first]), mine, "{lemma} 1sg");
        assert_eq!(
            form(lemma, &[ppl, first, Morph::Case(Case::Acc)]),
            mine_acc,
            "{lemma} 1sg acc"
        );
    }
}

#[test]
fn the_epenthetic_drop_and_the_shortening_compose() {
    // `lélek` drops the epenthetic vowel to `lélk` and then shortens the new last vowel; stopping after the drop gives
    // `lélket` and unanalysable forms. Measured alternants: `lelkek` 73,264 : `lélekek` 6, `lelket` 97,886 : `léleket` 15 [WC2].
    assert_eq!(form("lélek", ACC), "lelket");
    assert_eq!(form("lélek", PL), "lelkek");
    assert_eq!(form("lélek", &[Morph::Case(Case::Sup)]), "lelken");
    assert_eq!(form("szentlélek", ACC), "szentlelket");
    assert_eq!(
        form(
            "lélek",
            &[Morph::Possessive {
                person: Person::Third,
                number: Number::Sing,
            }]
        ),
        "lelke"
    );
}

#[test]
fn an_optional_alternation_generates_the_alternant_and_accepts_both() {
    // `mag` pluralises both ways in quantity, `magok` 42,607 : `magvak` 32,449 [WC2], so neither
    // may be the only reachable form. Generation keeps the alternant and analysis takes either.
    assert_eq!(form("mag", PL), "magvak");
    for word in ["magvak", "magok"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn the_corpus_arbitrates_where_the_rulebook_over_lists_a_bound_form() {
    // These three contradict a rulebook golden, and the corpus is the arbiter where the rule says
    // the class is lexical. NOM-ACC-008 lists `ész` -> `eszet`, but `eszet` is 1,277 tokens of a
    // different lemma `esz` while `ész` takes `észt` 19,382 [WC2]. PH-VSTEM-004 lists `odú` ->
    // `odva`, 396 against `odúja` 585, and `odvas` is a lexeme of its own.
    assert_eq!(form("ész", ACC), "észt");
    assert_eq!(form("odú", ACC), "odút");
    assert_eq!(form("odú", PL), "odúk");
    // The same measurement is what makes the commonest forms of `fiú` reachable at all:
    // `fiúk` 262,813 and `fiút` 84,246 against `fik` 623 and no `fit` of this lemma [WC2].
    assert_eq!(form("fiú", PL), "fiúk");
    assert_eq!(form("fiú", ACC), "fiút");
}

#[test]
fn the_ajto_possessive_is_measured_rather_than_listed() {
    // NOM-STEM-013 is a possessive-only alternation, so the plural and accusative say nothing about it and it has its
    // own measured axis; it corrects one rulebook member: `szőlője` 2,828 : `szőleje` 445 [WC2].
    let poss = &[Morph::Possessive {
        person: Person::Third,
        number: Number::Sing,
    }];
    assert_eq!(form("ajtó", poss), "ajtaja");
    assert_eq!(form("idő", poss), "ideje");
    assert_eq!(form("tető", poss), "teteje");
    assert_eq!(form("szőlő", poss), "szőlője");
    assert_eq!(form("só", poss), "sója");
    // A genuine v-stem keeps its own possessive: the plain candidate for `ló` has to be `lova`,
    // not `lója`, or the measurement asks the corpus the wrong question.
    assert_eq!(form("ló", poss), "lova");
    assert_eq!(form("kő", poss), "köve");
}

#[test]
fn an_epenthetic_drop_cannot_change_the_harmony_class() {
    // `titok` binds to `titk`, whose only vowel is a neutral `i`; reading the class off the alternant makes every form
    // front (`titkek`, `titket`). `bokor` does not show it because `bokr` keeps its `o`.
    assert_eq!(form("titok", ACC), "titkot");
    assert_eq!(form("titok", PL), "titkok");
    assert_eq!(form("piszok", ACC), "piszkot");
    assert_eq!(form("cirok", PL), "cirkok");
    assert_eq!(form("bokor", ACC), "bokrot");
    assert_eq!(form("retek", PL), "retkek");
}

#[test]
fn a_compound_takes_its_head_stem_class() {
    // DER-COMP-007 inflects a compound on its last member, so its stem class is the head's: `jármű` has no
    // `Shortening` because `mű` has none (`járművek`, not `jármüvek`).
    assert_eq!(form("jármű", PL), "járművek");
    assert_eq!(form("jármű", ACC), "járművet");
    assert_eq!(form("mű", PL), "művek");
}

#[test]
fn each_cell_of_an_alternation_is_measured_on_its_own() {
    // One verdict per lexeme cannot express what the corpus shows, so each cell carries its own
    // flag. `odú` pluralises `odúk` 2,668 and takes `odút` 2,571, but its possessive is `odva`;
    // `szél` is two lexemes under one spelling and only the possessive separates them,
    // `széle` 37,411 beside `szele` 16,866 [WC2], so both are accepted.
    let poss = &[Morph::Possessive {
        person: Person::Third,
        number: Number::Sing,
    }];
    assert_eq!(form("odú", PL), "odúk");
    assert_eq!(form("odú", ACC), "odút");
    // `odúja` 585 : `odva` 396 [WC2], so the measured winner is generated and both are accepted.
    assert_eq!(form("odú", poss), "odúja");
    assert_eq!(
        picur_morph_spell::check("odva"),
        picur_morph_spell::Status::Ok
    );
    assert_eq!(form("fiú", PL), "fiúk");
    assert_eq!(form("fiú", ACC), "fiút");
    for word in ["széle", "szele", "szélén", "szelet", "szélek"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    assert_eq!(form("tükör", ACC), "tükröt");
    assert_eq!(form("tükör", PL), "tükrök");
    assert_eq!(form("tükör", &[Morph::Case(Case::Sup)]), "tükrön");
}

#[test]
fn the_possessed_plural_has_its_own_allomorph_choice() {
    // NOM-PPL-001 derives the cell from the singular's `-ja`/`-a`, and the two part company often:
    // `barátja` is the right singular while the plural is `barátai` 116,592 to `barátjai` 591.
    // `kert` keeps the `j` in both, `kertjei` 2,424 to `kertei` 20 [WC2].
    let ppl = &[
        Morph::PossessedPlural,
        Morph::Possessive {
            person: Person::Third,
            number: Number::Sing,
        },
    ];
    assert_eq!(form("barát", ppl), "barátai");
    assert_eq!(form("kert", ppl), "kertjei");
    assert_eq!(form("állam", ppl), "államai");
    let sg = &[Morph::Possessive {
        person: Person::Third,
        number: Number::Sing,
    }];
    assert_eq!(form("barát", sg), "barátja");
    assert_eq!(form("kert", sg), "kertje");
}

#[test]
fn lowering_is_measured_per_cell_and_per_category() {
    // NOM-LOW-001 marks lowering per lexeme and the cells come apart: `olaj` takes `olajat`
    // 156,574 : `olajot` 2,551 but `olajok` 43,083 : `olajak` 3,789 [WC2].
    assert_eq!(form("olaj", ACC), "olajat");
    assert_eq!(form("olaj", PL), "olajok");
    // And per category, or a homograph votes in the wrong paradigm: `piros` is primarily an
    // adjective, `pirosak` 5,860 and `pirosat` 13,876 against no plain form at all. A noun-only
    // query scored those at zero and made the whole lexeme look non-lowering. Its noun reading's
    // `pirosok` is 114 tokens, below the floor an exemption needs, so it is not generated.
    for word in ["pirosak", "pirosat"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    assert_eq!(form("ház", PL), "házak");
    assert_eq!(form("ház", ACC), "házat");
    assert_eq!(form("fül", PL), "fülek");
}

#[test]
fn the_possessed_plural_allomorph_is_chosen_against_the_alternant() {
    // `műv` ends in a consonant and takes `-ei`; treating it as the vowel-final lemma `mű` gives `műi` for `művei`.
    let ppl = &[
        Morph::PossessedPlural,
        Morph::Possessive {
            person: Person::Third,
            number: Number::Sing,
        },
    ];
    assert_eq!(form("mű", ppl), "művei");
    assert_eq!(form("ló", ppl), "lovai");
    assert_eq!(form("hajó", ppl), "hajói");
}

#[test]
fn the_ajto_stem_change_is_the_vowel_alone_and_the_j_is_the_allomorph() {
    // NOM-STEM-013 writes the class as "`a`/`e` plus `j`", but the `j` is the ordinary `-jA`
    // allomorph after a vowel. Putting it in the stem forced exceptions: the stem `ajtaj` cannot
    // produce `szülei`, which is this same change plus the possessed plural's bare `-i`.
    //
    // Both cells are measured, and they come apart: `ajtó` alternates in the singular and takes
    // both in the possessed plural, `ajtajai` 2,388 ~ `ajtói` 2,802, while `szülő` is the reverse,
    // plain `szülője` 23,889 beside `szülei` 179,772 [WC2]. That last is what NOM-STEM-013 records
    // as lexicalised, and no exception is needed for it.
    let poss = &[Morph::Possessive {
        person: Person::Third,
        number: Number::Sing,
    }];
    assert_eq!(form("ajtó", poss), "ajtaja");
    assert_eq!(form("idő", poss), "ideje");
    assert_eq!(form("szülő", poss), "szülője");
    for word in [
        "ajtaja",
        "ajtói",
        "ajtajai",
        "ideje",
        "idői",
        "szülője",
        "szülei",
        "szüleim",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    assert!(
        !picur_morph_spell::analyses("szüléi")
            .iter()
            .any(|a| a.lemma.as_ref() == "szülő")
    );
}

#[test]
fn lowering_is_discovered_and_not_only_narrowed() {
    // A measurement that can narrow a marking but never add one leaves morphdb's omissions in
    // place, so every noun and adjective is asked. `néhány` was unmarked: `néhányat` is 86,124
    // against `néhányt` 162 [WC2], and NOM-ACC-005 has a lowering stem take the vowel even in a
    // bare class, so the measured accusative also settles a contradicting `bareT` flag.
    //
    // Our categories are coarser than the corpus tagset, and the closed-class quantifiers are why
    // that matters: `néhányat` and `párat` are tagged Num while our entries are nouns.
    for word in ["néhányat", "párat", "sokat", "keveset", "eleget"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    // An unmeasured cell says nothing either way: reading it as plain added `Low` to `cirok` off
    // the back of its accusative alone and gave `cirkak`.
    assert_eq!(form("cirok", PL), "cirkok");
    assert_eq!(form("orr", ACC), "orrot");
    assert_eq!(form("gáz", ACC), "gázt");
}

#[test]
fn lowering_has_three_measured_cells() {
    // The possessive parts company with the other two: `pár` takes the low `párat` and the plain
    // `párom` 91,800 : `páram` 43 [WC2].
    let poss = &[Morph::Possessive {
        person: Person::First,
        number: Number::Sing,
    }];
    assert_eq!(form("pár", poss), "párom");
    assert_eq!(form("pár", ACC), "párat");
    assert_eq!(form("pár", PL), "párok");
    assert_eq!(form("ház", poss), "házam");
    assert_eq!(form("ház", ACC), "házat");
    assert_eq!(form("ház", PL), "házak");
}

#[test]
fn the_possessive_allomorph_is_measured_for_every_noun() {
    // NOM-POSS-006 calls the choice lexical with no learnable default. Both are accepted because both are attested:
    // `utódja` 38,421 : `utóda` 23,241 [WC2]; `kerte` 97 and `házja` are not.
    for word in ["utódja", "utóda", "kertje", "háza"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    for word in ["kerte", "házja"] {
        assert_ne!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn an_inflected_form_is_a_new_stem() {
    // NOM-STEM-017: non-alternating and lowering, so the lemma's accusative class must not reach it: `ember` is bare
    // `embert`, but its plural accusative is `embereket`, not `emberekt`.
    let pl_acc = &[Morph::Plural, Morph::Case(Case::Acc)];
    assert_eq!(form("ember", pl_acc), "embereket");
    assert_eq!(form("ember", ACC), "embert");
    assert_eq!(form("ház", pl_acc), "házakat");
    assert_eq!(form("kert", pl_acc), "kerteket");
    // The harmony traits travel, because an antiharmonic lemma stays antiharmonic.
    assert_eq!(form("híd", pl_acc), "hidakat");
}

#[test]
fn a_quantifier_takes_the_multiplicative() {
    // The quantifier table lists the collective and the multiplicative together, and our
    // categories put those words with the adjectives, so the two take the same inputs.
    for word in ["sokszor", "néhányszor", "sokan", "néhányan"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}
