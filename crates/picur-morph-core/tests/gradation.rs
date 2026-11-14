//! Comparative and superlative.

use picur_morph_core::{Morph, Stacks, generate};
use picur_morph_lexicon::{Pos, lexicon};

fn form(lemma: &str, stack: &[Morph]) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Adjective)
        .unwrap_or_else(|| panic!("{lemma} missing"));
    generate(e, stack).unwrap_or_else(|x| panic!("{lemma}: {x}"))
}

#[test]
fn regular_comparatives() {
    // NOM-ADJ-004: the linking vowel is low whatever the stem's own lowering value.
    for (base, want) in [
        ("piros", "pirosabb"),
        ("gazdag", "gazdagabb"),
        ("boldog", "boldogabb"),
        ("fiatal", "fiatalabb"),
        ("idős", "idősebb"),
        ("mély", "mélyebb"),
    ] {
        assert_eq!(form(base, &[Morph::Comparative]), want, "{base}");
    }
}

#[test]
fn stem_traits_derive_most_of_the_irregular_table() {
    assert_eq!(form("nehéz", &[Morph::Comparative]), "nehezebb");
    assert_eq!(form("bátor", &[Morph::Comparative]), "bátrabb");
    assert_eq!(form("bő", &[Morph::Comparative]), "bővebb");
}

#[test]
fn genuinely_irregular_comparatives() {
    // NOM-ADJ-005, the part no rule reaches.
    for (base, want) in [
        ("jó", "jobb"),
        ("kicsi", "kisebb"),
        ("szép", "szebb"),
        ("nagy", "nagyobb"),
        ("hosszú", "hosszabb"),
        ("könnyű", "könnyebb"),
        ("ifjú", "ifjabb"),
    ] {
        assert_eq!(form(base, &[Morph::Comparative]), want, "{base}");
    }
}

#[test]
fn the_superlative_is_a_prefix_on_the_comparative() {
    // NOM-ADJ-006, written solid.
    assert_eq!(form("nagy", &[Morph::Superlative]), "legnagyobb");
    assert_eq!(form("jó", &[Morph::Superlative]), "legjobb");
    assert_eq!(form("szép", &[Morph::Superlative]), "legszebb");
    assert_eq!(form("kicsi", &[Morph::Superlative]), "legkisebb");
    assert_eq!(form("hosszú", &[Morph::Superlative]), "leghosszabb");
    assert_eq!(form("jó", &[Morph::Excessive]), "legeslegjobb");
    assert_eq!(form("szép", &[Morph::Excessive]), "legeslegszebb");
}

#[test]
fn a_comparative_stem_is_lowering() {
    // NOM-ADJ-007: `nagyobbak`, not `*nagyobbok`.
    assert_eq!(
        form("nagy", &[Morph::Comparative, Morph::Plural]),
        "nagyobbak"
    );
    assert_eq!(form("jó", &[Morph::Comparative, Morph::Plural]), "jobbak");
    assert_eq!(
        form("kicsi", &[Morph::Comparative, Morph::Plural]),
        "kisebbek"
    );
}

#[test]
fn ordinals_are_recognised_and_inflect() {
    // PND-ORD-001, PND-ORD-002: generated rather than listed, so the set has no arbitrary edge.
    use picur_morph_core::{Case, Verdict, analyse};
    let stacks = Stacks::from(vec![
        vec![],
        vec![Morph::Plural],
        vec![Morph::Case(Case::Acc)],
        vec![Morph::Case(Case::Ine)],
    ]);
    for word in [
        "első",
        "második",
        "harmadik",
        "ötödik",
        "tizedik",
        "huszadik",
        "századik",
        "ezredik",
        "hányadik",
        "harmadikat",
        "ötödiket",
        "harmadikban",
    ] {
        assert!(
            matches!(analyse(word, &stacks), Verdict::Known(_)),
            "{word} not analysed"
        );
    }
}

#[test]
fn stated_stem_exceptions_keep_the_free_stem() {
    // NOM-STEM-010: `szó`, `jó` and `szú` alternate in the plural but not in the accusative or
    // superessive. Corpus: `szót` 534,651 : `szavat` 1,408.
    use picur_morph_core::Case;
    use picur_morph_lexicon::Pos;
    let f = |lemma: &str, pos: Pos, stack: &[Morph]| {
        let e = lexicon()
            .get_pos(lemma, pos)
            .unwrap_or_else(|| panic!("{lemma} missing"));
        generate(e, stack).unwrap_or_else(|x| panic!("{lemma}: {x}"))
    };
    assert_eq!(f("szó", Pos::Noun, &[Morph::Case(Case::Acc)]), "szót");
    assert_eq!(f("szó", Pos::Noun, &[Morph::Case(Case::Sup)]), "szón");
    assert_eq!(f("szó", Pos::Noun, &[Morph::Plural]), "szavak");
    assert_eq!(f("jó", Pos::Adjective, &[Morph::Case(Case::Acc)]), "jót");
    assert_eq!(f("tó", Pos::Noun, &[Morph::Case(Case::Acc)]), "tavat");
    assert_eq!(f("kő", Pos::Noun, &[Morph::Case(Case::Sup)]), "kövön");
    // NOM-STEM-010 names these as non-v-stems of the same shape.
    assert_eq!(f("só", Pos::Noun, &[Morph::Case(Case::Acc)]), "sót");
    assert_eq!(f("só", Pos::Noun, &[Morph::Plural]), "sók");
    assert_eq!(f("nő", Pos::Noun, &[Morph::Plural]), "nők");
}

#[test]
fn derivational_suffixes_from_the_master_table() {
    // DER-MODEL-016, the productive rows.
    use picur_morph_core::DerId;
    use picur_morph_lexicon::Pos;
    let d = |lemma: &str, pos: Pos, id: DerId| {
        let e = lexicon()
            .get_pos(lemma, pos)
            .unwrap_or_else(|| panic!("{lemma} missing"));
        generate(e, &[Morph::Derive(id)]).ok()
    };
    assert_eq!(
        d("szép", Pos::Adjective, DerId::Sag).as_deref(),
        Some("szépség")
    );
    assert_eq!(d("olvas", Pos::Verb, DerId::As).as_deref(), Some("olvasás"));
    assert_eq!(d("olvas", Pos::Verb, DerId::O).as_deref(), Some("olvasó"));
    assert_eq!(
        d("olvas", Pos::Verb, DerId::Hato).as_deref(),
        Some("olvasható")
    );
    assert_eq!(d("néz", Pos::Verb, DerId::Hato).as_deref(), Some("nézhető"));
    assert_eq!(d("gond", Pos::Noun, DerId::Os).as_deref(), Some("gondos"));
    // DER-ADJ-009 splits the privative three ways by what the base does. A consonant-final base
    // that neither lowers nor is epenthetic takes `-tAlAn` on the free stem; `gondatlan` is the
    // lexical half of a doublet with a meaning split ('careless' against 'carefree'), so it is a
    // stored word and this is the regular formation.
    assert_eq!(
        d("gond", Pos::Noun, DerId::Talan).as_deref(),
        Some("gondtalan")
    );
    assert_eq!(
        d("láb", Pos::Noun, DerId::Talan).as_deref(),
        Some("lábatlan")
    );
    assert_eq!(
        d("olvas", Pos::Verb, DerId::Talan).as_deref(),
        Some("olvasatlan")
    );
    assert_eq!(d("gond", Pos::Noun, DerId::Oz).as_deref(), Some("gondoz"));
    assert_eq!(d("ház", Pos::Noun, DerId::Nyi).as_deref(), Some("háznyi"));
    assert_eq!(
        d("szép", Pos::Adjective, DerId::It).as_deref(),
        Some("szépít")
    );
    // DER-ADJ-008: the `(j)` appears after a vowel, and the stem lengthens first.
    assert_eq!(d("láb", Pos::Noun, DerId::U).as_deref(), Some("lábú"));
    assert_eq!(d("szem", Pos::Noun, DerId::U).as_deref(), Some("szemű"));
    assert_eq!(d("alma", Pos::Noun, DerId::U).as_deref(), Some("almájú"));
    assert_eq!(d("hajó", Pos::Noun, DerId::U).as_deref(), Some("hajójú"));
    // DER-DIM-001 restricts the base to `i` or to a polysyllable in a non-stop consonant: its
    // rule 6 gives a monosyllable `-(O)cskA` only, and the exceptions (`lányka`, `tálka`) are
    // stored words rather than formations.
    assert_eq!(d("madár", Pos::Noun, DerId::Ka).as_deref(), Some("madárka"));
    assert_eq!(d("lány", Pos::Noun, DerId::Ka), None);
    assert_eq!(d("szív", Pos::Noun, DerId::Ka), None);
    assert_eq!(d("gond", Pos::Noun, DerId::Ka), None);
}

#[test]
fn collective_and_multiplicative() {
    // PND-NUMINF-005 and PND-NUMINF-006, their Output lines in full.
    use picur_morph_lexicon::Pos;
    let f = |lemma: &str, pos: Pos, m: Morph| {
        let e = lexicon()
            .get_pos(lemma, pos)
            .unwrap_or_else(|| panic!("{lemma} missing"));
        generate(e, &[m]).unwrap_or_else(|x| panic!("{lemma}: {x}"))
    };
    for (lemma, want) in [
        ("három", "hárman"),
        ("öt", "öten"),
        ("hét", "heten"),
        ("tíz", "tízen"),
        ("húsz", "húszan"),
        ("ezer", "ezren"),
        ("hat", "hatan"),
        ("millió", "millióan"),
        ("száz", "százan"),
        ("kettő", "ketten"),
    ] {
        assert_eq!(f(lemma, Pos::Numeral, Morph::Collective), want, "{lemma}");
    }
    assert_eq!(f("sok", Pos::Adjective, Morph::Collective), "sokan");
    assert_eq!(f("kevés", Pos::Adjective, Morph::Collective), "kevesen");
    assert_eq!(f("több", Pos::Adjective, Morph::Collective), "többen");
    assert_eq!(f("mennyi", Pos::Pronoun, Morph::Collective), "mennyien");

    for (lemma, want) in [
        ("egy", "egyszer"),
        ("kettő", "kétszer"),
        ("három", "háromszor"),
        ("négy", "négyszer"),
        ("öt", "ötször"),
        ("hat", "hatszor"),
        ("hét", "hétszer"),
        ("tíz", "tízszer"),
        // WS-DIG-002: two adjacent `sz` letters are written `ssz`.
        ("húsz", "hússzor"),
        ("száz", "százszor"),
        ("ezer", "ezerszer"),
        ("millió", "milliószor"),
    ] {
        assert_eq!(
            f(lemma, Pos::Numeral, Morph::Multiplicative),
            want,
            "{lemma}"
        );
    }
}

#[test]
fn the_numeral_two_has_three_stems() {
    // PND-NUMINF-003: `két` before the multiplicative, `kett-` before a vowel-initial derivational
    // suffix, `kettő` elsewhere.
    use picur_morph_core::{Case, DerId};
    use picur_morph_lexicon::Pos;
    let e = lexicon().get_pos("kettő", Pos::Numeral).expect("kettő");
    assert_eq!(
        generate(e, &[Morph::Multiplicative]).expect("mult"),
        "kétszer"
    );
    assert_eq!(generate(e, &[Morph::Collective]).expect("coll"), "ketten");
    assert_eq!(
        generate(e, &[Morph::Derive(DerId::Os)]).expect("-Os"),
        "kettes"
    );
    assert_eq!(
        generate(e, &[Morph::Case(Case::Acc)]).expect("acc"),
        "kettőt"
    );
}

#[test]
fn a_high_vowel_final_adjective_takes_a_low_link_before_the_plural() {
    // NOM-PL-003. The link is low whatever the stem's own lowering value: `hosszúak` 22,897
    // against `hosszúok` 1, `budapestiek` 21,705 against `budapestik` 6 [WC2].
    use picur_morph_core::Case;
    use picur_morph_lexicon::Pos;
    let f = |lemma: &str, pos: Pos, stack: &[Morph]| {
        let e = lexicon()
            .get_pos(lemma, pos)
            .unwrap_or_else(|| panic!("{lemma} missing"));
        generate(e, stack).unwrap_or_else(|x| panic!("{lemma}: {x}"))
    };
    for lemma in ["budapesti", "házi", "hosszú"] {
        assert!(
            f(lemma, Pos::Adjective, &[Morph::Plural]).ends_with("ak")
                || f(lemma, Pos::Adjective, &[Morph::Plural]).ends_with("ek"),
            "{lemma} -> {}",
            f(lemma, Pos::Adjective, &[Morph::Plural])
        );
    }
    assert_eq!(
        f("budapesti", Pos::Adjective, &[Morph::Plural]),
        "budapestiek"
    );
    assert_eq!(f("hosszú", Pos::Adjective, &[Morph::Plural]), "hosszúak");
    assert_eq!(f("házi", Pos::Adjective, &[Morph::Plural]), "háziak");
    // NOM-PL-003 names `férfi` as the noun that follows the rule.
    assert_eq!(f("férfi", Pos::Noun, &[Morph::Plural]), "férfiak");

    assert_eq!(
        f("budapesti", Pos::Adjective, &[Morph::Case(Case::Acc)]),
        "budapestit"
    );
    assert_eq!(f("férfi", Pos::Noun, &[Morph::Case(Case::Acc)]), "férfit");
    assert_eq!(
        f("hosszú", Pos::Adjective, &[Morph::Case(Case::Acc)]),
        "hosszút"
    );
    assert_eq!(f("hajó", Pos::Noun, &[Morph::Plural]), "hajók");
    assert_eq!(f("alma", Pos::Noun, &[Morph::Plural]), "almák");
}

#[test]
fn a_compound_harmonises_with_its_last_member() {
    // DER-COMP-008: each member is its own harmony domain. `aprópénz` has `ó` as its last
    // non-neutral vowel, but the suffix follows the `é` of `pénz`: `aprópénzek` 346 against
    // `aprópénzok` 0 [WC2].
    use picur_morph_core::Case;
    use picur_morph_lexicon::Pos;
    let f = |lemma: &str, stack: &[Morph]| {
        let e = lexicon()
            .get_pos(lemma, Pos::Noun)
            .unwrap_or_else(|| panic!("{lemma} missing"));
        generate(e, stack).unwrap_or_else(|x| panic!("{lemma}: {x}"))
    };
    assert_eq!(f("aprópénz", &[Morph::Plural]), "aprópénzek");
    assert_eq!(f("aprópénz", &[Morph::Case(Case::Ine)]), "aprópénzben");
    assert_eq!(f("zsebpénz", &[Morph::Plural]), "zsebpénzek");
    assert_eq!(f("könyvespolc", &[Morph::Plural]), "könyvespolcok");
    assert_eq!(f("ablakkeret", &[Morph::Plural]), "ablakkeretek");
    assert_eq!(f("gyümölcslé", &[Morph::Plural]), "gyümölcslevek");
    assert_eq!(f("ház", &[Morph::Plural]), "házak");
}

#[test]
fn the_measured_accusative_beats_the_computed_one() {
    // NOM-ACC-005 and NOM-ACC-009 both have lexical exceptions, and the corpus supplies them:
    // pénzt 827,855 against pénzet 148, oldalt 272,453 against oldalat 1,168 [WC2, Acc-tagged].
    use picur_morph_core::Case;
    use picur_morph_lexicon::Pos;
    let acc = |lemma: &str| {
        let e = lexicon()
            .get_pos(lemma, Pos::Noun)
            .unwrap_or_else(|| panic!("{lemma} missing"));
        generate(e, &[Morph::Case(Case::Acc)]).expect("accusative")
    };
    assert_eq!(acc("pénz"), "pénzt");
    assert_eq!(acc("oldal"), "oldalt");
    assert_eq!(acc("antioxidáns"), "antioxidánst");
    assert_eq!(acc("ház"), "házat");
    assert_eq!(acc("gáz"), "gázt");
    assert_eq!(acc("tér"), "teret");
}

#[test]
fn the_possessed_plural_has_its_own_person_set() {
    // NOM-PPL-001: `-m`, `-d`, zero, `-nk`, `-tok/-tek`, `-k`. The ordinary possessive set would
    // give `*adataija` and `*adataijuk`.
    use picur_morph_core::{Number, Person};
    use picur_morph_lexicon::Pos;
    let e = lexicon().get_pos("adat", Pos::Noun).expect("adat");
    let p = |person, number| {
        generate(
            e,
            &[Morph::PossessedPlural, Morph::Possessive { person, number }],
        )
        .expect("possessed plural")
    };
    assert_eq!(
        generate(e, &[Morph::PossessedPlural]).expect("bare"),
        "adatai"
    );
    assert_eq!(p(Person::First, Number::Sing), "adataim");
    assert_eq!(p(Person::Second, Number::Sing), "adataid");
    assert_eq!(p(Person::Third, Number::Sing), "adatai");
    assert_eq!(p(Person::First, Number::Plur), "adataink");
    assert_eq!(p(Person::Third, Number::Plur), "adataik");
    // NOM-PPL-004: the 2pl has no rounded form.
    assert_eq!(p(Person::Second, Number::Plur), "adataitok");
}

#[test]
fn the_relative_series_harmonises_with_its_second_member() {
    // DER-COMP-008 again: `aki` is `a` plus `ki`, so the suffix follows `ki` and gives `akinek`
    // rather than `*akinak`.
    use picur_morph_core::Case;
    use picur_morph_lexicon::Pos;
    let f = |lemma: &str, c: Case| {
        let e = lexicon()
            .get_pos(lemma, Pos::Pronoun)
            .unwrap_or_else(|| panic!("{lemma} missing"));
        generate(e, &[Morph::Case(c)]).unwrap_or_else(|x| panic!("{lemma}: {x}"))
    };
    assert_eq!(f("aki", Case::Dat), "akinek");
    assert_eq!(f("aki", Case::Acc), "akit");
    assert_eq!(f("ami", Case::Dat), "aminek");
    assert_eq!(f("amely", Case::Acc), "amelyet");
}

#[test]
fn a_vowel_final_stem_takes_no_link_before_the_comparative() {
    // NOM-PL-003 gives a vowel-final gradable stem a low link before the plural only; the comparative goes the other way:
    // `hosszúak` but `hosszabb`, `egyszerűek` but `egyszerűbb`. Gating on `lowerable` would give `egyszerűebb`, `gyakoriabb`.
    for (lemma, comparative, plural) in [
        ("egyszerű", "egyszerűbb", "egyszerűek"),
        ("szomorú", "szomorúbb", "szomorúak"),
        ("gyakori", "gyakoribb", "gyakoriak"),
        ("olcsó", "olcsóbb", "olcsók"),
        ("drága", "drágább", "drágák"),
    ] {
        let e = lexicon()
            .get_pos(lemma, Pos::Adjective)
            .unwrap_or_else(|| panic!("{lemma} is an adjective"));
        assert_eq!(
            generate(e, &[Morph::Comparative]).as_deref(),
            Ok(comparative),
            "{lemma} comparative"
        );
        assert_eq!(
            generate(e, &[Morph::Plural]).as_deref(),
            Ok(plural),
            "{lemma} plural"
        );
    }
}

#[test]
fn leg_attaches_to_the_closed_list_without_a_comparative() {
    // DER-PREF-004 names these as the rule's own lexical input: `leg-` is productive on a
    // comparative and lexical on a handful of ordinal-like words that have no comparative at all.
    for (word, lemma) in [
        ("legelső", "első"),
        ("legutolsó", "utolsó"),
        ("legfelső", "felső"),
        ("legalsó", "alsó"),
    ] {
        let readings = picur_morph_spell::analyses(word);
        assert!(
            readings.iter().any(|a| a.lemma.as_ref() == lemma),
            "{word} has no {lemma} reading: {:?}",
            readings.iter().map(|a| &a.lemma).collect::<Vec<_>>()
        );
    }
    let big = picur_morph_spell::analyses("legnagyobb");
    assert!(big.iter().any(|a| a.lemma.as_ref() == "nagy"));
    assert!(
        picur_morph_spell::analyses("legenda")
            .iter()
            .all(|a| a.lemma.as_ref() == "legenda")
    );
}

#[test]
fn the_plural_link_is_optional_after_o_and_oe() {
    // NOM-PL-003 lists the plural of an `ó`/`ő`-final adjective as `-k` ~ `-ak/-ek`, both correct.
    // Generation takes the preferred bare `-k`; the linked form is reached as a variant.
    for word in [
        "elérhetők",
        "elérhetőek",
        "olcsók",
        "olcsóak",
        "bántók",
        "bántóak",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    assert_eq!(
        picur_morph_spell::check("hosszúak"),
        picur_morph_spell::Status::Ok
    );
    assert_eq!(
        picur_morph_spell::check("hajók"),
        picur_morph_spell::Status::Ok
    );
    assert_ne!(
        picur_morph_spell::check("hajóak"),
        picur_morph_spell::Status::Ok
    );
}

#[test]
fn the_lag_adverb_is_reached_through_its_computed_domain() {
    // DER-ADV-004 states `-lAg` as P-dom, on `-i` adjectives of disciplines. DER-MODEL-012 keeps a
    // P-dom suffix out of generation because the domain is normally a semantic feature, but the
    // `-i` half of this one is in the spelling, so it is computed and the objection lapses.
    for word in [
        "gazdaságilag",
        "jogilag",
        "politikailag",
        "szakmailag",
        "lelkileg",
        "nemzetközileg",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    // The domain is what stops it inventing vocabulary: `ház` is no `-i` adjective.
    assert_ne!(
        picur_morph_spell::check("házlag"),
        picur_morph_spell::Status::Ok
    );
}

#[test]
fn a_derived_word_inflects_as_its_output_category() {
    // DER-MODEL-003: the suffix assigns the output category, so the stack is judged against that, not the lemma's:
    // `vágyik` + `-Ó` is a noun with plural `vágyók`.
    for word in ["vágyók", "fellépők", "elérhetők", "olvasók", "vágyót"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn the_ju_adjective_is_built_on_the_possessive_stem() {
    // DER-ADJ-008 puts `-jú/-jű` on the possessive stem, so NOM-STEM-013's possessive-only vowel change supplies the `j`;
    // lengthening the lemma instead gives `időjű`.
    for word in ["idejű", "tetejű", "erejű", "ajtajú"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    for word in ["almájú", "házú"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn a_manner_adverb_can_be_built_on_a_participle() {
    // NOM-ADJ-010 lists `-ó/-ő` participles with the `-an/-en` link beside bare `-n` (`drágán`, `olcsón`). Category and stem
    // properties must travel with the stack (DER-MODEL-010): `kezdődő` + `-An` is `kezdődően`, not `kezdődőn`.
    for word in [
        "kezdődően",
        "bántóan",
        "kiválóan",
        "hasonlóan",
        "vonatkozóan",
        "következően",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    for word in ["drágán", "olcsón", "nagyon", "szépen", "gyorsan"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn a_graded_manner_adverb_is_the_adverb_of_the_comparative() {
    // `NOM-ADJ-010` on a `NOM-ADJ-004` stem, not a lemma of its own. Suppletive ones need the reverse-table readings merged
    // with the simplex ones, and the table queried for prefixes.
    for (word, lemma) in [
        ("gyorsabban", "gyors"),
        ("szebben", "szép"),
        ("jobban", "jó"),
        ("korábban", "kora"),
    ] {
        let readings = picur_morph_spell::analyses(word);
        assert!(
            readings.iter().any(|a| a.lemma.as_ref() == lemma
                && a.stack.first() == Some(&picur_morph_core::Morph::Comparative)),
            "{word} has no comparative reading of {lemma}: {:?}",
            readings.iter().map(|a| &a.lemma).collect::<Vec<_>>()
        );
    }
    for word in ["legszebben", "legjobban", "legnagyobb"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn a_derived_noun_takes_a_possessive_and_the_relational_i() {
    // The deverbal `-Ás` stacks constantly in running text; only noun-forming suffixes stack this way, since the search
    // space is paid on every token that is not found.
    for word in [
        "létrehozása",
        "azonosítása",
        "létrehozásában",
        "barátsága",
        "foglalkoztatási",
        "fejlesztési",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn a_graded_stem_takes_the_other_suffixes_too() {
    // NOM-STEM-017 makes the comparative a stem like any other, so the multiplicative, the
    // relational `-i` and the collective all follow it. NOM-ADJ-005 lists `régebbi` and
    // `későbbi` as built on the adverb stem, which is why the adjective `régi` does not derive it.
    for word in [
        "többször",
        "legtöbbször",
        "régebbi",
        "későbbi",
        "legrégebbi",
        "többen",
        "régebben",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn a_name_derivative_is_lowercase_and_still_reaches_its_name() {
    // ORT-NAME-022 case (a): the adjective derived from a one-word or solid name is lowercase with
    // the suffix attached directly, `Budapest` giving `budapesti` and `Szerbia` giving `szerbiai`.
    // Such a form shares no written prefix with its lemma, so the index keys a capitalised stem
    // both ways and the prefix guard allows a differing initial.
    for word in ["szerbiai", "budapesti", "magyarországi", "bécsi"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    assert!(
        picur_morph_spell::analyses("szerbiai")
            .iter()
            .any(|a| a.lemma.as_ref() == "Szerbia")
    );
    // The bare lowercase name stays wrong, which is what makes keying both ways safe: nothing
    // regenerates `szerbia` from `Szerbia`.
    for word in ["szerbia", "európa"] {
        assert_ne!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn leg_attaches_to_the_comparative_even_where_the_base_has_its_own_superlative() {
    // DER-PREF-004 attaches the prefix to the comparative, so the check is that the prefix plus
    // the regenerated comparative is the surface. Regenerating the superlative instead misses a
    // base whose own superlative is on the rule's closed list: `felső` gives the real but
    // different `legfelső`, and `legfelsőbb` is `leg` plus `felsőbb`.
    for word in [
        "legfelsőbb",
        "legfelső",
        "legelső",
        "legnagyobb",
        "legszebben",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    assert!(
        picur_morph_spell::analyses("legenda")
            .iter()
            .all(|a| a.lemma.as_ref() == "legenda")
    );
}
