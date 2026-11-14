//! Present indicative paradigms from the committed lexicon.

#![expect(
    clippy::expect_used,
    reason = "a golden that cannot find its lemma must fail loudly"
)]

use picur_morph_core::{Definite, Mood, Morph, Number, Person, Tense, generate};
use picur_morph_lexicon::{Pos, lexicon};

fn pres(lemma: &str, person: Person, number: Number) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Verb)
        .unwrap_or_else(|| panic!("{lemma} missing from the lexicon"));
    generate(
        e,
        &[Morph::Verbal {
            mood: Mood::Ind,
            tense: Tense::Pres,
            person,
            number,
            definite: Definite::Ind,
        }],
    )
    .expect("generates")
}

#[test]
fn light_stem_paradigm() {
    // VRB-PRES-001.
    assert_eq!(pres("vár", Person::First, Number::Sing), "várok");
    assert_eq!(pres("vár", Person::Second, Number::Sing), "vársz");
    assert_eq!(pres("vár", Person::Third, Number::Sing), "vár");
    assert_eq!(pres("vár", Person::First, Number::Plur), "várunk");
    assert_eq!(pres("vár", Person::Second, Number::Plur), "vártok");
    assert_eq!(pres("vár", Person::Third, Number::Plur), "várnak");
}

#[test]
fn front_and_rounded_paradigms() {
    assert_eq!(pres("kér", Person::First, Number::Sing), "kérek");
    assert_eq!(pres("kér", Person::Second, Number::Plur), "kértek");
    assert_eq!(pres("kér", Person::Third, Number::Plur), "kérnek");
    assert_eq!(pres("tör", Person::First, Number::Sing), "török");
    assert_eq!(pres("tör", Person::First, Number::Plur), "törünk");
    assert_eq!(pres("tör", Person::Second, Number::Plur), "törtök");
}

#[test]
fn heavy_stems_take_the_linking_vowel() {
    // VRB-LINK-002 (a): tart ends in two different consonants.
    assert_eq!(pres("tart", Person::Second, Number::Sing), "tartasz");
    assert_eq!(pres("tart", Person::Second, Number::Plur), "tartotok");
    assert_eq!(pres("tart", Person::Third, Number::Plur), "tartanak");
    assert_eq!(pres("fest", Person::Second, Number::Sing), "festesz");
    assert_eq!(pres("fest", Person::Third, Number::Plur), "festenek");
}

#[test]
fn it_final_stems_are_heavy() {
    // VRB-LINK-002 (b).
    assert_eq!(pres("tanít", Person::Third, Number::Plur), "tanítanak");
    assert_eq!(pres("tanít", Person::Second, Number::Sing), "tanítasz");
}

#[test]
fn sibilant_stems_take_ol() {
    // VRB-LINK-003.
    assert_eq!(pres("mos", Person::Second, Number::Sing), "mosol");
    assert_eq!(pres("néz", Person::Second, Number::Sing), "nézel");
    assert_eq!(pres("főz", Person::Second, Number::Sing), "főzöl");
}

#[test]
fn light_stems_keep_bare_sz() {
    assert_eq!(pres("ad", Person::Second, Number::Sing), "adsz");
    assert_eq!(pres("lát", Person::Second, Number::Sing), "látsz");
    assert_eq!(pres("fut", Person::Second, Number::Sing), "futsz");
}

#[test]
fn ik_verbs_keep_their_third_singular() {
    // VRB-IK-001: the stem is `dolgoz-` but the 3sg is the lemma itself.
    assert_eq!(pres("dolgozik", Person::Third, Number::Sing), "dolgozik");
    assert_eq!(pres("dolgozik", Person::First, Number::Plur), "dolgozunk");
    assert_eq!(pres("dolgozik", Person::Third, Number::Plur), "dolgoznak");
}

fn past(lemma: &str, person: Person, number: Number) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Verb)
        .unwrap_or_else(|| panic!("{lemma} missing"));
    generate(
        e,
        &[Morph::Verbal {
            mood: Mood::Ind,
            tense: Tense::Past,
            person,
            number,
            definite: Definite::Ind,
        }],
    )
    .expect("generates")
}

fn def(lemma: &str, person: Person, number: Number) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Verb)
        .unwrap_or_else(|| panic!("{lemma} missing"));
    generate(
        e,
        &[Morph::Verbal {
            mood: Mood::Ind,
            tense: Tense::Pres,
            person,
            number,
            definite: Definite::Def,
        }],
    )
    .expect("generates")
}

#[test]
fn past_after_a_coronal_sonorant() {
    // VRB-PAST-001.
    assert_eq!(past("vár", Person::Third, Number::Sing), "várt");
    assert_eq!(past("vár", Person::First, Number::Sing), "vártam");
    assert_eq!(past("vár", Person::Second, Number::Sing), "vártál");
    assert_eq!(past("vár", Person::Third, Number::Plur), "vártak");
}

/// `VRB-PAST-005`: a long `dz` takes `-Ott-` in every cell (`edzettem` 8,407 : `edztem` 24
/// [WC2]); the short `dz` of `-(Ó)dz-` does not (`lopództam`).
#[test]
fn past_after_a_long_dz() {
    assert_eq!(past("edz", Person::First, Number::Sing), "edzettem");
    assert_eq!(past("edz", Person::Third, Number::Plur), "edzettek");
    assert_eq!(past("pedz", Person::First, Number::Sing), "pedzettem");
    assert_eq!(past("lopódzik", Person::First, Number::Sing), "lopództam");
}

#[test]
fn past_with_the_long_marker() {
    // VRB-PAST-005.
    assert_eq!(past("ad", Person::Third, Number::Sing), "adott");
    assert_eq!(past("ad", Person::First, Number::Sing), "adtam");
    assert_eq!(past("néz", Person::Third, Number::Sing), "nézett");
    assert_eq!(past("néz", Person::First, Number::Sing), "néztem");
    assert_eq!(past("mos", Person::Third, Number::Sing), "mosott");
    assert_eq!(past("mos", Person::First, Number::Sing), "mostam");
}

#[test]
fn t_final_cluster_stems_take_the_long_marker() {
    // `lát` keeps the short marker, but not because its vowel is long: `fűt` and `vét` are long
    // too and take `-Ott-`. It is lexical, and the full table is in `t_final_past_classes`.
    assert_eq!(past("lát", Person::First, Number::Sing), "láttam");
    assert_eq!(past("tart", Person::Third, Number::Sing), "tartott");
    assert_eq!(past("tart", Person::First, Number::Sing), "tartottam");
}

#[test]
fn present_definite_splits_on_backness() {
    // VRB-PRES-002.
    assert_eq!(def("vár", Person::Third, Number::Sing), "várja");
    assert_eq!(def("vár", Person::First, Number::Plur), "várjuk");
    assert_eq!(def("vár", Person::Third, Number::Plur), "várják");
    assert_eq!(def("kér", Person::Third, Number::Sing), "kéri");
    assert_eq!(def("kér", Person::Third, Number::Plur), "kérik");
    assert_eq!(def("tör", Person::Third, Number::Sing), "töri");
    assert_eq!(def("tör", Person::First, Number::Sing), "töröm");
    assert_eq!(def("vár", Person::First, Number::Sing), "várom");
}

fn cond(lemma: &str, person: Person, number: Number) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Verb)
        .unwrap_or_else(|| panic!("{lemma} missing"));
    generate(
        e,
        &[Morph::Verbal {
            mood: Mood::Cnd,
            tense: Tense::Pres,
            person,
            number,
            definite: Definite::Ind,
        }],
    )
    .expect("generates")
}

fn pot(lemma: &str, person: Person, number: Number) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Verb)
        .unwrap_or_else(|| panic!("{lemma} missing"));
    generate(
        e,
        &[
            Morph::Potential,
            Morph::Verbal {
                mood: Mood::Ind,
                tense: Tense::Pres,
                person,
                number,
                definite: Definite::Ind,
            },
        ],
    )
    .expect("generates")
}

#[test]
fn conditional_paradigm() {
    // VRB-COND-001: the 1sg -nék is invariant across all three harmony classes.
    assert_eq!(cond("vár", Person::First, Number::Sing), "várnék");
    assert_eq!(cond("kér", Person::First, Number::Sing), "kérnék");
    assert_eq!(cond("tör", Person::First, Number::Sing), "törnék");
    assert_eq!(cond("vár", Person::Second, Number::Sing), "várnál");
    assert_eq!(cond("vár", Person::Third, Number::Sing), "várna");
    assert_eq!(cond("kér", Person::Third, Number::Sing), "kérne");
    assert_eq!(cond("vár", Person::First, Number::Plur), "várnánk");
    assert_eq!(cond("kér", Person::First, Number::Plur), "kérnénk");
}

#[test]
fn conditional_after_a_heavy_stem() {
    assert_eq!(cond("tart", Person::Third, Number::Sing), "tartana");
    assert_eq!(cond("fest", Person::Third, Number::Sing), "festene");
}

#[test]
fn potential_builds_a_new_stem() {
    // VRB-POT-001: the h is never assimilated in spelling.
    assert_eq!(pot("ad", Person::Third, Number::Sing), "adhat");
    assert_eq!(pot("néz", Person::Third, Number::Sing), "nézhet");
    assert_eq!(pot("vár", Person::First, Number::Sing), "várhatok");
    assert_eq!(pot("kér", Person::First, Number::Plur), "kérhetünk");
    assert_eq!(pot("tör", Person::Third, Number::Sing), "törhet");
}

#[test]
fn infinitive_takes_the_linking_vowel_only_when_heavy() {
    // VRB-INF-001: várni, adni, but tartani and tanítani.
    let inf = |lemma: &str| {
        let e = lexicon()
            .get_pos(lemma, Pos::Verb)
            .unwrap_or_else(|| panic!("{lemma} missing"));
        generate(e, &[Morph::Infinitive]).expect("generates")
    };
    assert_eq!(inf("vár"), "várni");
    assert_eq!(inf("kér"), "kérni");
    assert_eq!(inf("ad"), "adni");
    assert_eq!(inf("lát"), "látni");
    assert_eq!(inf("tart"), "tartani");
    assert_eq!(inf("fest"), "festeni");
    assert_eq!(inf("tanít"), "tanítani");
    assert_eq!(inf("dolgozik"), "dolgozni");
}

fn imp(lemma: &str, person: Person, number: Number) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Verb)
        .unwrap_or_else(|| panic!("{lemma} missing"));
    generate(
        e,
        &[Morph::Verbal {
            mood: Mood::Imp,
            tense: Tense::Pres,
            person,
            number,
            definite: Definite::Ind,
        }],
    )
    .expect("generates")
}

#[test]
fn imperative_paradigm_with_plain_j() {
    // VRB-IMP-001, VRB-IMP-002.
    assert_eq!(imp("vár", Person::Second, Number::Sing), "várj");
    assert_eq!(imp("vár", Person::Third, Number::Sing), "várjon");
    assert_eq!(imp("vár", Person::First, Number::Sing), "várjak");
    assert_eq!(imp("vár", Person::First, Number::Plur), "várjunk");
    assert_eq!(imp("vár", Person::Third, Number::Plur), "várjanak");
    assert_eq!(imp("küld", Person::Third, Number::Sing), "küldjön");
}

#[test]
fn imperative_sibilant_assimilation() {
    // VRB-IMP-003: the marker is written as a doubled letter.
    assert_eq!(imp("olvas", Person::Second, Number::Sing), "olvass");
    assert_eq!(imp("olvas", Person::Third, Number::Sing), "olvasson");
    assert_eq!(imp("néz", Person::Third, Number::Sing), "nézzen");
    assert_eq!(imp("hoz", Person::Second, Number::Sing), "hozz");
}

#[test]
fn imperative_t_stems() {
    // VRB-IMP-004: short vowel + t gives ss. VRB-IMP-005: consonant + t gives ts.
    assert_eq!(imp("köt", Person::Third, Number::Sing), "kössön");
    assert_eq!(imp("fut", Person::Third, Number::Sing), "fusson");
    assert_eq!(imp("lát", Person::Third, Number::Sing), "lásson");
    assert_eq!(imp("tanít", Person::Third, Number::Sing), "tanítson");
    assert_eq!(imp("tart", Person::Third, Number::Sing), "tartson");
}

#[test]
fn imperative_st_and_szt_stems_drop_the_t() {
    // VRB-IMP-006.
    assert_eq!(imp("fest", Person::Third, Number::Sing), "fessen");
    assert_eq!(imp("oszt", Person::Third, Number::Sing), "osszon");
}

#[test]
fn sz_d_v_verbs_switch_stems() {
    // VRB-STEM-012: `alsz-` in the present, `alud-` everywhere else. morphdb has no entry for
    // these at all, so they come from the curated supplement.
    assert_eq!(pres("alszik", Person::Third, Number::Sing), "alszik");
    assert_eq!(pres("alszik", Person::First, Number::Plur), "alszunk");
    assert_eq!(pres("alszik", Person::Third, Number::Plur), "alszanak");

    assert_eq!(past("alszik", Person::First, Number::Sing), "aludtam");
    assert_eq!(imp("alszik", Person::Second, Number::Sing), "aludj");
    assert_eq!(imp("alszik", Person::Third, Number::Plur), "aludjanak");
    assert_eq!(cond("alszik", Person::First, Number::Sing), "aludnék");

    let inf = |lemma: &str| {
        let e = lexicon().get_pos(lemma, Pos::Verb).expect("lemma");
        generate(e, &[Morph::Infinitive]).expect("generates")
    };
    assert_eq!(inf("alszik"), "aludni");
    assert_eq!(inf("fekszik"), "feküdni");
    assert_eq!(imp("fekszik", Person::Third, Number::Sing), "feküdjön");
    assert_eq!(past("nyugszik", Person::Third, Number::Sing), "nyugodott");
}

#[test]
fn the_imperative_of_an_sz_d_verb_is_never_sibilant_assimilated() {
    // `*alusszanak` has 0 corpus hits against `aludjanak` 2,351: the imperative of this class uses
    // the `d` stem only, so the sibilant assimilation of VRB-IMP-003 must not apply to `alsz-`.
    let got = imp("alszik", Person::Third, Number::Plur);
    assert_eq!(got, "aludjanak");
    assert!(
        !got.contains("ssz"),
        "sibilant assimilation leaked in: {got}"
    );
}

fn nonfinite(lemma: &str, m: Morph) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Verb)
        .unwrap_or_else(|| panic!("{lemma} missing"));
    generate(e, &[m]).expect("generates")
}

#[test]
fn participles_and_converb() {
    // VRB-PART-001, VRB-PART-004, VRB-ADVP-001.
    assert_eq!(nonfinite("vár", Morph::PresentParticiple), "váró");
    assert_eq!(nonfinite("kér", Morph::PresentParticiple), "kérő");
    assert_eq!(nonfinite("dolgozik", Morph::PresentParticiple), "dolgozó");

    assert_eq!(nonfinite("vár", Morph::PastParticiple), "várt");
    assert_eq!(nonfinite("ad", Morph::PastParticiple), "adott");

    assert_eq!(nonfinite("olvas", Morph::Converb), "olvasva");
    assert_eq!(nonfinite("ül", Morph::Converb), "ülve");
    assert_eq!(nonfinite("mond", Morph::Converb), "mondva");
    assert_eq!(nonfinite("dob", Morph::Converb), "dobva");
    assert_eq!(nonfinite("tanít", Morph::Converb), "tanítva");
}

/// One finite cell of `lemma`, from the committed lexicon entry.
fn cell(lemma: &str, mood: Mood, tense: Tense, p: Person, n: Number, d: Definite) -> String {
    let e = lexicon()
        .get_pos(lemma, Pos::Verb)
        .unwrap_or_else(|| panic!("{lemma} missing from the lexicon"));
    generate(
        e,
        &[Morph::Verbal {
            mood,
            tense,
            person: p,
            number: n,
            definite: d,
        }],
    )
    .expect("generates")
}

/// The six cells of one tense, in person-number order.
fn row(lemma: &str, mood: Mood, tense: Tense, d: Definite) -> Vec<String> {
    [
        (Person::First, Number::Sing),
        (Person::Second, Number::Sing),
        (Person::Third, Number::Sing),
        (Person::First, Number::Plur),
        (Person::Second, Number::Plur),
        (Person::Third, Number::Plur),
    ]
    .into_iter()
    .map(|(p, n)| cell(lemma, mood, tense, p, n, d))
    .collect()
}

#[test]
fn past_definite_paradigm() {
    // VRB-PAST-011: the definite table, whose first singular coincides with the indefinite.
    assert_eq!(
        row("vár", Mood::Ind, Tense::Past, Definite::Def),
        ["vártam", "vártad", "várta", "vártuk", "vártátok", "várták"]
    );
    assert_eq!(
        row("kér", Mood::Ind, Tense::Past, Definite::Def),
        ["kértem", "kérted", "kérte", "kértük", "kértétek", "kérték"]
    );
    assert_eq!(
        cell(
            "vár",
            Mood::Ind,
            Tense::Past,
            Person::First,
            Number::Plur,
            Definite::Ind
        ),
        "vártunk"
    );
}

#[test]
fn past_rounding_does_not_spread_past_an_unrounded_vowel() {
    // PH-HARM-005: -AtOk on `ütött` is `ütöttetek`. The `e` of the first slot unrounds the second,
    // so reading harmony once off the stem gives the wrong `*ütöttetök`.
    assert_eq!(
        row("üt", Mood::Ind, Tense::Past, Definite::Ind),
        [
            "ütöttem",
            "ütöttél",
            "ütött",
            "ütöttünk",
            "ütöttetek",
            "ütöttek"
        ]
    );
    assert_eq!(
        row("üt", Mood::Ind, Tense::Past, Definite::Def),
        [
            "ütöttem",
            "ütötted",
            "ütötte",
            "ütöttük",
            "ütöttétek",
            "ütötték"
        ]
    );
    assert_eq!(
        row("üt", Mood::Cnd, Tense::Pres, Definite::Ind),
        ["ütnék", "ütnél", "ütne", "ütnénk", "ütnétek", "ütnének"]
    );
}

#[test]
fn conditional_definite_paradigm() {
    // VRB-COND-001: the indefinite third plural has its own long vowel, `várnának`.
    assert_eq!(
        row("vár", Mood::Cnd, Tense::Pres, Definite::Ind),
        [
            "várnék",
            "várnál",
            "várna",
            "várnánk",
            "várnátok",
            "várnának"
        ]
    );
    // VRB-COND-002: 1pl and 2pl coincide with the indefinite; the other four differ.
    assert_eq!(
        row("vár", Mood::Cnd, Tense::Pres, Definite::Def),
        ["várnám", "várnád", "várná", "várnánk", "várnátok", "várnák"]
    );
    assert_eq!(
        row("kér", Mood::Cnd, Tense::Pres, Definite::Def),
        ["kérném", "kérnéd", "kérné", "kérnénk", "kérnétek", "kérnék"]
    );
}

#[test]
fn lak_across_the_moods() {
    // VRB-PRES-005, VRB-LINK-008: light stems take -lAk, heavy stems -AlAk, and the past,
    // conditional and imperative all place it after a vowel.
    let lak = |lemma: &str, mood: Mood, tense: Tense| {
        cell(
            lemma,
            mood,
            tense,
            Person::First,
            Number::Sing,
            Definite::Two,
        )
    };
    assert_eq!(lak("vár", Mood::Ind, Tense::Pres), "várlak");
    assert_eq!(lak("kér", Mood::Ind, Tense::Pres), "kérlek");
    assert_eq!(lak("lát", Mood::Ind, Tense::Pres), "látlak");
    assert_eq!(lak("szeret", Mood::Ind, Tense::Pres), "szeretlek");
    assert_eq!(lak("fest", Mood::Ind, Tense::Pres), "festelek");
    assert_eq!(lak("tanít", Mood::Ind, Tense::Pres), "tanítalak");

    assert_eq!(lak("vár", Mood::Ind, Tense::Past), "vártalak");
    assert_eq!(lak("vár", Mood::Cnd, Tense::Pres), "várnálak");
    assert_eq!(lak("vár", Mood::Imp, Tense::Pres), "várjalak");
    assert_eq!(lak("kér", Mood::Imp, Tense::Pres), "kérjelek");
}

#[test]
fn lak_has_no_cell_outside_the_first_singular() {
    let e = lexicon().get_pos("vár", Pos::Verb).expect("vár");
    let got = generate(
        e,
        &[Morph::Verbal {
            mood: Mood::Ind,
            tense: Tense::Pres,
            person: Person::Third,
            number: Number::Sing,
            definite: Definite::Two,
        }],
    );
    assert!(got.is_err(), "generated {got:?} for a cell that has none");
}

#[test]
fn three_identical_letters_become_two() {
    // ORT-ASSIM-017: hall + -lak is hallak.
    assert_eq!(
        cell(
            "hall",
            Mood::Ind,
            Tense::Pres,
            Person::First,
            Number::Sing,
            Definite::Two
        ),
        "hallak"
    );
}

#[test]
fn t_final_past_classes() {
    // VRB-PAST-006: the whole t-final table. The discriminator is polysyllabicity, not vowel
    // length: `fut` and `fűt` behave alike, and `mutat` differs from both.
    let past = |lemma: &str, p: Person, n: Number| {
        cell(lemma, Mood::Ind, Tense::Past, p, n, Definite::Ind)
    };
    let nonfinal = |lemma: &str| past(lemma, Person::First, Number::Sing);
    let final3 = |lemma: &str| past(lemma, Person::Third, Number::Sing);

    for (lemma, want) in [
        ("mutat", "mutattam"),
        ("szeret", "szerettem"),
        ("alkot", "alkottam"),
        ("ugat", "ugattam"),
        ("követ", "követtem"),
    ] {
        assert_eq!(nonfinal(lemma), want);
    }
    for (lemma, want) in [
        ("fut", "futottam"),
        ("köt", "kötöttem"),
        ("üt", "ütöttem"),
        ("fűt", "fűtöttem"),
        ("vét", "vétettem"),
        ("bocsát", "bocsátottam"),
        ("tanít", "tanítottam"),
        ("fest", "festettem"),
        ("tart", "tartottam"),
    ] {
        assert_eq!(nonfinal(lemma), want);
    }
    // `lát` is the one lexical exception, and needs its trait to get there.
    assert_eq!(nonfinal("lát"), "láttam");
    for (lemma, want) in [
        ("mutat", "mutatott"),
        ("fut", "futott"),
        ("lát", "látott"),
        ("szeret", "szeretett"),
    ] {
        assert_eq!(final3(lemma), want);
    }
}

#[test]
fn potential_combines_with_every_mood() {
    // VRB-POT-002, VRB-MODEL-001: -hAt is a slot before mood, so it multiplies with all of them.
    let e = lexicon().get_pos("olvas", Pos::Verb).expect("olvas");
    let pot = |mood, tense, p, n, d| {
        generate(
            e,
            &[
                Morph::Potential,
                Morph::Verbal {
                    mood,
                    tense,
                    person: p,
                    number: n,
                    definite: d,
                },
            ],
        )
        .expect("generates")
    };
    let (s1, s3) = ((Person::First, Number::Sing), (Person::Third, Number::Sing));
    assert_eq!(
        pot(Mood::Ind, Tense::Pres, s1.0, s1.1, Definite::Ind),
        "olvashatok"
    );
    assert_eq!(
        pot(Mood::Ind, Tense::Pres, s3.0, s3.1, Definite::Ind),
        "olvashat"
    );
    assert_eq!(
        pot(Mood::Ind, Tense::Pres, s3.0, s3.1, Definite::Def),
        "olvashatja"
    );
    // The T5 past, which is what makes this `olvashattam` and not `*olvashatottam`.
    assert_eq!(
        pot(Mood::Ind, Tense::Past, s1.0, s1.1, Definite::Ind),
        "olvashattam"
    );
    assert_eq!(
        pot(Mood::Ind, Tense::Past, s3.0, s3.1, Definite::Ind),
        "olvashatott"
    );
    // Potential plus conditional and potential plus imperative: neither is expressible if the
    // potential is a mood value.
    assert_eq!(
        pot(Mood::Cnd, Tense::Pres, s1.0, s1.1, Definite::Ind),
        "olvashatnék"
    );
    assert_eq!(
        pot(Mood::Imp, Tense::Pres, s1.0, s1.1, Definite::Ind),
        "olvashassak"
    );
    assert_eq!(
        pot(Mood::Imp, Tense::Pres, s3.0, s3.1, Definite::Ind),
        "olvashasson"
    );
    assert_eq!(
        generate(e, &[Morph::Potential, Morph::Infinitive]).expect("generates"),
        "olvashatni"
    );
}

#[test]
fn definite_j_assimilates_to_a_sibilant() {
    // VRB-PRES-003: the same assimilation as the imperative, in the definite conjugation.
    assert_eq!(def("olvas", Person::Third, Number::Sing), "olvassa");
    assert_eq!(def("olvas", Person::First, Number::Plur), "olvassuk");
    assert_eq!(def("mos", Person::Third, Number::Plur), "mossák");
    assert_eq!(def("hoz", Person::First, Number::Plur), "hozzuk");
    assert_eq!(def("edz", Person::First, Number::Plur), "eddzük");
    assert_eq!(def("játszik", Person::Third, Number::Sing), "játssza");
    assert_eq!(def("úszik", Person::Third, Number::Sing), "ússza");
    assert_eq!(def("néz", Person::First, Number::Plur), "nézzük");
    assert_eq!(def("néz", Person::Third, Number::Sing), "nézi");
    assert_eq!(def("lát", Person::Third, Number::Sing), "látja");
    assert_eq!(def("tanít", Person::Third, Number::Plur), "tanítják");
}

#[test]
fn a_literal_vowel_unrounds_the_slots_behind_it() {
    // PH-HARM-005: the `i` of the front definite -itOk is unrounded, so a front rounded stem
    // still gives `-itek`.
    assert_eq!(def("főz", Person::Second, Number::Plur), "főzitek");
    assert_eq!(def("néz", Person::Second, Number::Plur), "nézitek");
    assert_eq!(def("főz", Person::First, Number::Plur), "főzzük");
}

#[test]
fn imperative_definite_has_its_own_long_vowel_cells() {
    // VRB-IMP-001: definite 2pl is -jÁtOk and 3pl -jÁk, distinct from the indefinite.
    let imp = |lemma: &str, p: Person, n: Number, d: Definite| {
        cell(lemma, Mood::Imp, Tense::Pres, p, n, d)
    };
    assert_eq!(
        imp("vár", Person::Second, Number::Plur, Definite::Def),
        "várjátok"
    );
    assert_eq!(
        imp("vár", Person::Third, Number::Plur, Definite::Def),
        "várják"
    );
    assert_eq!(
        imp("kér", Person::Second, Number::Plur, Definite::Def),
        "kérjétek"
    );
    assert_eq!(
        imp("kér", Person::Third, Number::Plur, Definite::Def),
        "kérjék"
    );
    assert_eq!(
        imp("vár", Person::Second, Number::Plur, Definite::Ind),
        "várjatok"
    );
    assert_eq!(
        imp("vár", Person::Third, Number::Plur, Definite::Ind),
        "várjanak"
    );
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one assertion per rulebook table cell"
)]
fn suppletive_paradigms() {
    // VRB-IRR-001..010: stem plus the ordinary endings.
    let f = |lemma: &str, mood, tense, p, n, d| cell(lemma, mood, tense, p, n, d);
    let (s1, s3, p1, p2, p3) = (
        (Person::First, Number::Sing),
        (Person::Third, Number::Sing),
        (Person::First, Number::Plur),
        (Person::Second, Number::Plur),
        (Person::Third, Number::Plur),
    );

    // VRB-IRR-001: suppletive inside the present, `vol-` past and conditional, everything else
    // from `lesz`.
    assert_eq!(
        f("van", Mood::Ind, Tense::Pres, s1.0, s1.1, Definite::Ind),
        "vagyok"
    );
    assert_eq!(
        f("van", Mood::Ind, Tense::Pres, p3.0, p3.1, Definite::Ind),
        "vannak"
    );
    assert_eq!(
        f("van", Mood::Ind, Tense::Past, s1.0, s1.1, Definite::Ind),
        "voltam"
    );
    // The conditional is `vol-` too, not the bound stem. `lenne` belongs to `lesz`, 4,241,008
    // against 5,791 for `van`, while `volnék` is 23,934 tokens of `van` and of nothing else
    // `[WC2]`. See `van_conditionalises_on_the_past_stem`.
    assert_eq!(
        f("van", Mood::Cnd, Tense::Pres, s3.0, s3.1, Definite::Ind),
        "volna"
    );
    assert_eq!(
        f("van", Mood::Imp, Tense::Pres, s3.0, s3.1, Definite::Ind),
        "legyen"
    );

    // VRB-IRR-003, VRB-IRR-004.
    assert_eq!(
        f("megy", Mood::Ind, Tense::Pres, p2.0, p2.1, Definite::Ind),
        "mentek"
    );
    assert_eq!(
        f("megy", Mood::Cnd, Tense::Pres, s1.0, s1.1, Definite::Ind),
        "mennék"
    );
    assert_eq!(
        f("megy", Mood::Imp, Tense::Pres, s3.0, s3.1, Definite::Ind),
        "menjen"
    );
    assert_eq!(
        f("jön", Mood::Ind, Tense::Pres, s1.0, s1.1, Definite::Ind),
        "jövök"
    );
    assert_eq!(
        f("jön", Mood::Ind, Tense::Past, p2.0, p2.1, Definite::Ind),
        "jöttetek"
    );
    // Rounding: `jöjjön` keeps it, `jöjjetek` does not.
    assert_eq!(
        f("jön", Mood::Imp, Tense::Pres, s3.0, s3.1, Definite::Ind),
        "jöjjön"
    );
    assert_eq!(
        f("jön", Mood::Imp, Tense::Pres, p2.0, p2.1, Definite::Ind),
        "jöjjetek"
    );

    // VRB-IRR-007: the tesz type, definite included.
    assert_eq!(
        f("tesz", Mood::Ind, Tense::Pres, s1.0, s1.1, Definite::Ind),
        "teszek"
    );
    assert_eq!(
        f("tesz", Mood::Ind, Tense::Pres, p1.0, p1.1, Definite::Def),
        "tesszük"
    );
    assert_eq!(
        f("tesz", Mood::Ind, Tense::Past, s3.0, s3.1, Definite::Ind),
        "tett"
    );
    assert_eq!(
        f("tesz", Mood::Ind, Tense::Past, p3.0, p3.1, Definite::Def),
        "tették"
    );
    assert_eq!(
        f("tesz", Mood::Cnd, Tense::Pres, s3.0, s3.1, Definite::Ind),
        "tenne"
    );
    assert_eq!(
        f("tesz", Mood::Imp, Tense::Pres, s3.0, s3.1, Definite::Def),
        "tegye"
    );

    // VRB-IRR-005, VRB-IRR-006: the past 3sg differs from the past stem here.
    assert_eq!(
        f("eszik", Mood::Ind, Tense::Past, s3.0, s3.1, Definite::Ind),
        "evett"
    );
    assert_eq!(
        f("eszik", Mood::Ind, Tense::Past, s1.0, s1.1, Definite::Ind),
        "ettem"
    );
    assert_eq!(
        f("eszik", Mood::Ind, Tense::Pres, s3.0, s3.1, Definite::Ind),
        "eszik"
    );
    assert_eq!(
        f("eszik", Mood::Ind, Tense::Pres, s3.0, s3.1, Definite::Def),
        "eszi"
    );
    // `iszik` is back although every vowel it is spelled with is neutral.
    assert_eq!(
        f("iszik", Mood::Ind, Tense::Past, s1.0, s1.1, Definite::Ind),
        "ittam"
    );
    assert_eq!(
        f("iszik", Mood::Ind, Tense::Pres, s3.0, s3.1, Definite::Def),
        "issza"
    );
    assert_eq!(
        f("iszik", Mood::Imp, Tense::Pres, p1.0, p1.1, Definite::Ind),
        "igyunk"
    );
}

#[test]
fn suppletive_nonfinites_and_potential() {
    let e = |lemma: &str| lexicon().get_pos(lemma, Pos::Verb).expect(lemma);
    for (lemma, want) in [
        ("van", "lenni"),
        ("megy", "menni"),
        ("jön", "jönni"),
        ("eszik", "enni"),
        ("iszik", "inni"),
        ("tesz", "tenni"),
        ("hisz", "hinni"),
    ] {
        assert_eq!(
            generate(e(lemma), &[Morph::Infinitive]).expect("generates"),
            want
        );
    }
    // VRB-POT-002: `lehet`, not `*vanhat`.
    for (lemma, want) in [
        ("van", "lehet"),
        ("megy", "mehet"),
        ("jön", "jöhet"),
        ("iszik", "ihat"),
        ("tesz", "tehet"),
    ] {
        let got = generate(
            e(lemma),
            &[
                Morph::Potential,
                Morph::Verbal {
                    mood: Mood::Ind,
                    tense: Tense::Pres,
                    person: Person::Third,
                    number: Number::Sing,
                    definite: Definite::Ind,
                },
            ],
        )
        .expect("generates");
        assert_eq!(got, want);
    }
}

#[test]
fn an_intransitive_suppletive_has_no_definite_cells() {
    // VRB-IRR-001: `van` has no definite conjugation, and inventing one would be worse than
    // refusing.
    let e = lexicon().get_pos("van", Pos::Verb).expect("van");
    let got = generate(
        e,
        &[Morph::Verbal {
            mood: Mood::Ind,
            tense: Tense::Pres,
            person: Person::Third,
            number: Number::Sing,
            definite: Definite::Def,
        }],
    );
    assert!(got.is_err(), "generated {got:?} for a cell that has none");
}

#[test]
fn short_definite_second_singular_imperative() {
    // VRB-IMP-007.
    let short = |lemma: &str| {
        cell(
            lemma,
            Mood::Imp,
            Tense::Pres,
            Person::Second,
            Number::Sing,
            Definite::Def,
        )
    };
    assert_eq!(short("vár"), "várd");
    assert_eq!(short("ad"), "add");
    assert_eq!(short("mond"), "mondd");
    assert_eq!(short("olvas"), "olvasd");
    assert_eq!(short("lát"), "lásd");
    assert_eq!(short("mutat"), "mutasd");
    assert_eq!(short("tart"), "tartsd");
    assert_eq!(short("tanít"), "tanítsd");
    assert_eq!(short("fest"), "fesd");
    // The suppletive ones are stated, because `higgy` does not shorten to `*higyd`.
    assert_eq!(short("tesz"), "tedd");
    assert_eq!(short("vesz"), "vedd");
    assert_eq!(short("visz"), "vidd");
    assert_eq!(short("hisz"), "hidd");
    assert_eq!(short("eszik"), "edd");
    assert_eq!(short("iszik"), "idd");
}

#[test]
fn a_v_stem_verb_splits_its_definite_paradigm() {
    // PH-VSTEM-002: the definite `-i` takes the v alternant, a `j`-initial ending keeps the long
    // vowel. `lő` gives `lövi` and `lőjük` in the same paradigm.
    assert_eq!(
        row("lő", Mood::Ind, Tense::Pres, Definite::Def),
        ["lövöm", "lövöd", "lövi", "lőjük", "lövitek", "lövik"]
    );
    assert_eq!(
        row("sző", Mood::Ind, Tense::Pres, Definite::Def),
        ["szövöm", "szövöd", "szövi", "szőjük", "szövitek", "szövik"]
    );
    assert_eq!(
        row("ró", Mood::Ind, Tense::Pres, Definite::Def),
        ["rovom", "rovod", "rója", "rójuk", "rójátok", "róják"]
    );
    assert_eq!(
        row("vár", Mood::Ind, Tense::Pres, Definite::Def),
        ["várom", "várod", "várja", "várjuk", "várjátok", "várják"]
    );
}

#[test]
fn the_future_participle() {
    // VRB-PART-003: `-AndÓ`, 'to be V-ed'.
    let e = |lemma: &str| lexicon().get_pos(lemma, Pos::Verb).expect(lemma);
    for (lemma, want) in [
        ("olvas", "olvasandó"),
        ("vár", "várandó"),
        ("néz", "nézendő"),
        ("fizet", "fizetendő"),
        ("ír", "írandó"),
    ] {
        assert_eq!(
            generate(e(lemma), &[Morph::FutureParticiple])
                .as_deref()
                .ok(),
            Some(want),
            "{lemma}"
        );
    }
    for (lemma, want) in [("tesz", "teendő"), ("van", "leendő"), ("jön", "jövendő")] {
        assert_eq!(
            generate(e(lemma), &[Morph::FutureParticiple])
                .as_deref()
                .ok(),
            Some(want),
            "{lemma}"
        );
    }
    for lemma in ["megy", "eszik"] {
        assert!(
            generate(e(lemma), &[Morph::FutureParticiple]).is_err(),
            "{lemma}"
        );
    }
}

#[test]
fn a_v_stem_verb_takes_the_v_alternant_before_a_vowel_initial_ending() {
    // VRB-LINK-009: the 1sg and 1pl endings always begin with a vowel, so they attach to the v
    // stem; the consonant-initial ones do not.
    assert_eq!(
        row("lő", Mood::Ind, Tense::Pres, Definite::Ind),
        ["lövök", "lősz", "lő", "lövünk", "lőtök", "lőnek"]
    );
    assert_eq!(
        row("ró", Mood::Ind, Tense::Pres, Definite::Ind),
        ["rovok", "rósz", "ró", "rovunk", "rótok", "rónak"]
    );
}

#[test]
fn geminate_verbs_are_heavy_or_light_by_lexical_value() {
    // VRB-LINK-002: nothing in the shape decides, so `hall` and `áll` diverge. Counts are
    // infinitive ratios from the induced list.
    let inf = |lemma: &str| {
        let e = lexicon().get_pos(lemma, Pos::Verb).expect(lemma);
        generate(e, &[Morph::Infinitive]).expect("infinitive")
    };
    // Heavy: hallani 270,582 against hallni 165.
    assert_eq!(inf("hall"), "hallani");
    assert_eq!(inf("vall"), "vallani");
    assert_eq!(inf("kell"), "kelleni");
    assert_eq!(inf("illik"), "illeni");
    assert_eq!(inf("izzik"), "izzani");
    assert_eq!(inf("áll"), "állni");
    assert_eq!(inf("száll"), "szállni");
    assert_eq!(inf("varr"), "varrni");
    assert_eq!(inf("forr"), "forrni");
}

#[test]
fn the_ll_clause_of_lak_beats_heaviness() {
    // VRB-LINK-008 lists the `ll` stems apart from the heavy ones and the `ll` clause wins:
    // `hall` is heavy everywhere else but gives `hallak`, 776 against `hallalak` 3 [WC2].
    let lak = |lemma: &str| {
        let e = lexicon().get_pos(lemma, Pos::Verb).expect(lemma);
        generate(
            e,
            &[Morph::Verbal {
                mood: Mood::Ind,
                tense: Tense::Pres,
                person: Person::First,
                number: Number::Sing,
                definite: Definite::Two,
            }],
        )
        .expect("lak")
    };
    assert_eq!(lak("hall"), "hallak");
    assert_eq!(lak("áll"), "állak");
}

#[test]
fn the_lexical_past_marker_is_measured_on_both_axes() {
    // VRB-PAST-004 and VRB-PAST-008 both say `lexical`: nothing in the coda separates `állt` from
    // `hallott` or `maradt` from `fogadott`. The two axes come apart, which is why there are four
    // traits and not two: `függött` word-finally against `függtem` before a person suffix.
    for (lemma, third, first) in [
        // Geminates, VRB-PAST-004. Corpus: `állt` 1,110,962 : `állott` 63,672; `hallott`
        // 162,272 : `hallt` 1,256 [WC2].
        ("áll", "állt", "álltam"),
        ("száll", "szállt", "szálltam"),
        ("varr", "varrt", "varrtam"),
        ("forr", "forrt", "forrtam"),
        ("hall", "hallott", "hallottam"),
        ("vall", "vallott", "vallottam"),
        ("függ", "függött", "függtem"),
        // `-ad`/`-ed`, VRB-PAST-008. Corpus: `maradt` 1,270,278 : `maradott` 2,558;
        // `fogadott` 176,235 : `fogadt` 647 [WC2].
        ("marad", "maradt", "maradtam"),
        ("fárad", "fáradt", "fáradtam"),
        ("szalad", "szaladt", "szaladtam"),
        ("ébred", "ébredt", "ébredtem"),
        ("fogad", "fogadott", "fogadtam"),
        ("enged", "engedett", "engedtem"),
        ("szenved", "szenvedett", "szenvedtem"),
        ("ad", "adott", "adtam"),
        // Consonant clusters, VRB-PAST-001. Corpus: `mondtam`/`mondtak` 536,884 against
        // `mondottam`/`mondottak` 12,219 [WC2].
        ("mond", "mondott", "mondtam"),
        ("küld", "küldött", "küldtem"),
        ("kezd", "kezdett", "kezdtem"),
        ("tart", "tartott", "tartottam"),
    ] {
        assert_eq!(
            past(lemma, Person::Third, Number::Sing),
            third,
            "{lemma} 3sg"
        );
        assert_eq!(
            past(lemma, Person::First, Number::Sing),
            first,
            "{lemma} 1sg"
        );
    }
}

#[test]
fn the_computed_past_still_holds_where_nothing_is_marked() {
    // The rules must keep deciding the unmarked cases, or the traits paper over a broken default.
    for (lemma, third, first) in [
        ("vár", "várt", "vártam"),
        ("olvas", "olvasott", "olvastam"),
        ("kér", "kért", "kértem"),
        ("fut", "futott", "futottam"),
        ("köt", "kötött", "kötöttem"),
        ("lát", "látott", "láttam"),
        ("tanul", "tanult", "tanultam"),
    ] {
        assert_eq!(
            past(lemma, Person::Third, Number::Sing),
            third,
            "{lemma} 3sg"
        );
        assert_eq!(
            past(lemma, Person::First, Number::Sing),
            first,
            "{lemma} 1sg"
        );
    }
}

#[test]
fn the_antiharmonic_class_takes_back_suffixes() {
    // PH-HARM-011: the spelling computes front, the language takes back. Measured in
    // `local/lists/antiharmonic.tsv`, which agrees across six oracles and the corpus.
    for (lemma, want) in [("ír", "írnak"), ("bír", "bírnak"), ("izzik", "izzanak")] {
        assert_eq!(pres(lemma, Person::Third, Number::Plur), want, "{lemma}");
    }
}

#[test]
fn an_epenthetic_verb_syncopates_before_a_vowel_initial_ending() {
    // VRB-STEM-007. The trigger is read off the ending's shape: the unstable vowel drops before
    // `-Ok`, `-Unk`, `-Om` and the front definite `-i`, and stays before `-sz`, `-tOk`, `-nAk` and
    // the `j`-initial definite endings. Syncope is obligatory only when the following consonant is
    // `g`, `z` or `m`; for `l` and `r` the rulebook calls it item-specific and those keep the full
    // stem, which is what they are attested with (`énekelek` against `éneklek`).
    let def = |lemma: &str, person, number| {
        let e = lexicon()
            .get_pos(lemma, Pos::Verb)
            .unwrap_or_else(|| panic!("{lemma} is a verb"));
        generate(
            e,
            &[Morph::Verbal {
                mood: Mood::Ind,
                tense: Tense::Pres,
                person,
                number,
                definite: Definite::Def,
            }],
        )
        .expect("generates")
    };
    for (lemma, first, second, third_pl, definite) in [
        ("érez", "érzek", "érzel", "éreznek", "érzi"),
        ("jelez", "jelzek", "jelzel", "jeleznek", "jelzi"),
        ("végez", "végzek", "végzel", "végeznek", "végzi"),
        ("forog", "forgok", "forogsz", "forognak", "forogja"),
        ("mozog", "mozgok", "mozogsz", "mozognak", "mozogja"),
    ] {
        assert_eq!(
            pres(lemma, Person::First, Number::Sing),
            first,
            "{lemma} 1sg"
        );
        assert_eq!(
            pres(lemma, Person::Second, Number::Sing),
            second,
            "{lemma} 2sg"
        );
        assert_eq!(
            pres(lemma, Person::Third, Number::Plur),
            third_pl,
            "{lemma} 3pl"
        );
        assert_eq!(
            def(lemma, Person::Third, Number::Sing),
            definite,
            "{lemma} 3sg definite"
        );
    }
    // VRB-STEM-007 calls the syncope obligatory after `g`, `z` and `m`, and the corpus refines it
    // per cell, not per lexeme: `képez` keeps the full stem before the definite `-i`, `képezi`
    // 201,559 to `képzi` 11,758, while the indefinite first person syncopates, `képzek` 442 to
    // `képezek` 235 and `képzem` 3,149 to `képezem` 421 [WC2]. Deciding the two cells together lets the definite outvote the other and gives `képezek`.
    for (lemma, first, definite) in [
        ("képez", "képzek", "képezi"),
        ("szegez", "szegezek", "szegezi"),
        ("köröz", "körözök", "körözi"),
    ] {
        assert_eq!(
            pres(lemma, Person::First, Number::Sing),
            first,
            "{lemma} 1sg"
        );
        assert_eq!(
            def(lemma, Person::Third, Number::Sing),
            definite,
            "{lemma} 3sg definite"
        );
    }
    // The `l` items are what `VRB-STEM-007` calls item-specific, and the corpus settles them cell
    // by cell rather than as a class. `énekel` keeps its vowel in the indefinite, `énekelek` 7,957
    // to `éneklek` 1,904, but not before the definite `-i`, `énekli` 27,146 to `énekeli` 595;
    // `töröl` syncopates in both, `törli` 89,954 to `töröli` 351 and `törlök` 1,753 to `törölök`
    // 21 [WC2]. Treating the class as keeping its vowel throughout gives `énekeli` and `törölök`.
    for (lemma, first, definite) in [
        ("énekel", "énekelek", "énekli"),
        ("pótol", "pótolok", "pótolja"),
        ("töröl", "törlök", "törli"),
    ] {
        assert_eq!(
            pres(lemma, Person::First, Number::Sing),
            first,
            "{lemma} 1sg"
        );
        assert_eq!(
            def(lemma, Person::Third, Number::Sing),
            definite,
            "{lemma} 3sg definite"
        );
    }
}

#[test]
fn the_personal_infinitive_matches_the_rulebook_table() {
    // VRB-INF-002. Every cell is built on the infinitive, which is why the irregular infinitives
    // need no separate statement: `lenni` gives `lennem` and `lennie`, `aludni` gives `aludnom`.
    // The harmony comes from the verb and not from the infinitive, whose `-ni` would flatten the
    // rounding and give `ülnem` for `ülnöm`.
    let cells = [
        (Person::First, Number::Sing),
        (Person::Second, Number::Sing),
        (Person::Third, Number::Sing),
        (Person::First, Number::Plur),
        (Person::Second, Number::Plur),
        (Person::Third, Number::Plur),
    ];
    for (lemma, want) in [
        (
            "tanul",
            [
                "tanulnom",
                "tanulnod",
                "tanulnia",
                "tanulnunk",
                "tanulnotok",
                "tanulniuk",
            ],
        ),
        (
            "kér",
            [
                "kérnem",
                "kérned",
                "kérnie",
                "kérnünk",
                "kérnetek",
                "kérniük",
            ],
        ),
        (
            "ül",
            ["ülnöm", "ülnöd", "ülnie", "ülnünk", "ülnötök", "ülniük"],
        ),
        (
            "mond",
            [
                "mondanom",
                "mondanod",
                "mondania",
                "mondanunk",
                "mondanotok",
                "mondaniuk",
            ],
        ),
        (
            "lesz",
            [
                "lennem", "lenned", "lennie", "lennünk", "lennetek", "lenniük",
            ],
        ),
        (
            "megy",
            [
                "mennem", "menned", "mennie", "mennünk", "mennetek", "menniük",
            ],
        ),
        (
            "alszik",
            [
                "aludnom",
                "aludnod",
                "aludnia",
                "aludnunk",
                "aludnotok",
                "aludniuk",
            ],
        ),
    ] {
        let e = lexicon()
            .get_pos(lemma, Pos::Verb)
            .unwrap_or_else(|| panic!("{lemma} is a verb"));
        for (i, (person, number)) in cells.into_iter().enumerate() {
            assert_eq!(
                generate(
                    e,
                    &[Morph::Infinitive, Morph::Possessive { person, number }]
                )
                .as_deref(),
                Ok(want[i]),
                "{lemma} cell {i}"
            );
        }
    }
}

#[test]
fn van_conditionalises_on_the_past_stem() {
    // VRB-IRR-001 gives `van` two conditionals. `lenne` is also the conditional of `lesz`, reachable through that lemma;
    // `volna` is reachable only as a form of `van`.
    let e = lexicon().get_pos("van", Pos::Verb).expect("van is a verb");
    for (person, number, want) in [
        (Person::First, Number::Sing, "volnék"),
        (Person::Second, Number::Sing, "volnál"),
        (Person::Third, Number::Sing, "volna"),
        (Person::First, Number::Plur, "volnánk"),
        (Person::Second, Number::Plur, "volnátok"),
        (Person::Third, Number::Plur, "volnának"),
    ] {
        assert_eq!(
            generate(
                e,
                &[Morph::Verbal {
                    mood: Mood::Cnd,
                    tense: Tense::Pres,
                    person,
                    number,
                    definite: Definite::Ind,
                }]
            )
            .as_deref(),
            Ok(want)
        );
    }
}

#[test]
fn a_minority_present_form_is_accepted_though_it_is_not_generated() {
    // Generation takes the cell's winner, but a losing form with real attestation is not an error:
    // `képezi` outnumbers `képzi` 201,559 to 11,758 and both are Hungarian [WC2]. The measurement
    // records the winner and whether the other is attested, and analysis takes either.
    for word in [
        "képezi",
        "képzi",
        "képzek",
        "képezek",
        "énekli",
        "énekelek",
        "éneklek",
        "közli",
        "törli",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn the_infinitive_link_is_measured_where_the_coda_computes_it_wrong() {
    // VRB-LINK-002 computes heaviness from the coda, which is right for a cluster and wrong for a
    // small class of single-consonant stems: `edzeni` 53,767 : `edzni` 10, `bocsátani` 44,997 :
    // `bocsátni` 69, `védeni` 112,745 : `védni` 358 [WC2]. 77 verbs of 10,330 attested.
    let inf = |lemma: &str| {
        let e = lexicon()
            .get_pos(lemma, Pos::Verb)
            .unwrap_or_else(|| panic!("{lemma} is a verb"));
        generate(e, &[Morph::Infinitive]).expect("generates")
    };
    assert_eq!(inf("edz"), "edzeni");
    assert_eq!(inf("bocsát"), "bocsátani");
    assert_eq!(inf("véd"), "védeni");
    assert_eq!(inf("tart"), "tartani");
    assert_eq!(inf("nyit"), "nyitni");
}

#[test]
fn the_sz_d_past_has_a_closed_bare_t_pair() {
    // VRB-STEM-009 names `aludt` and `feküdt` as the exception: every other member of the class
    // takes `-Ott`, which is also what the `d`-final coda computes.
    for (lemma, want) in [("alszik", "aludt"), ("fekszik", "feküdt")] {
        let e = lexicon()
            .get_pos(lemma, Pos::Verb)
            .unwrap_or_else(|| panic!("{lemma} is a verb"));
        assert_eq!(
            generate(
                e,
                &[Morph::Verbal {
                    mood: Mood::Ind,
                    tense: Tense::Past,
                    person: Person::Third,
                    number: Number::Sing,
                    definite: Definite::Ind,
                }]
            )
            .as_deref(),
            Ok(want)
        );
    }
    for word in ["nyugodott", "cselekedett", "esküdött"] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn the_past_marker_is_measured_for_every_verb() {
    // VRB-PAST-001 chooses by the coda and the choice is lexical for a large class, splitting by cell: `mond` is `mondott`
    // word-finally but `mondtam` before a person suffix; 10,438 verbs are measured on both cells. The losing form is accepted
    // where attested: `mondottam` 10,973 [WC2].
    for word in [
        "mondott",
        "mondtam",
        "mondottam",
        "tartott",
        "fogadott",
        "függött",
        "függtem",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
    // `-Ott` is categorical after a stem-final `t`, so that cell is never measured and the bare
    // `-t` form it would have produced is not a word.
    assert_ne!(
        picur_morph_spell::check("tartt"),
        picur_morph_spell::Status::Ok
    );
}

#[test]
fn the_sz_verbs_build_the_converb_on_their_own_stem() {
    // VRB-ADVP-001 lists them: `téve`, `véve`, `víve`, `menve`. The converb must not use the bound stem that gives
    // `venni` and `tenni` (`venve` is wrong).
    for (lemma, want) in [
        ("tesz", "téve"),
        ("vesz", "véve"),
        ("visz", "víve"),
        ("megy", "menve"),
    ] {
        let e = lexicon()
            .get_pos(lemma, Pos::Verb)
            .unwrap_or_else(|| panic!("{lemma} is a verb"));
        assert_eq!(generate(e, &[Morph::Converb]).as_deref(), Ok(want));
        assert!(generate(e, &[Morph::Infinitive]).is_ok());
    }
}

#[test]
fn the_tszik_verbs_come_from_the_source() {
    // morphdb has them in `missing.lexicon` with a lowercase tag for a verb with no definite conjugation, on rows that
    // separate the lemma with a space rather than a tab; `tetszik` alone is 711,197 tokens.
    for word in [
        "tetszik",
        "tetszett",
        "látszik",
        "látszott",
        "látszanak",
        "hallatszik",
    ] {
        assert_eq!(
            picur_morph_spell::check(word),
            picur_morph_spell::Status::Ok,
            "{word}"
        );
    }
}

#[test]
fn the_past_is_its_own_syncope_cell() {
    // VRB-STEM-007 is refined per cell, and the past disagrees with the two present ones: `jegyez`
    // gives `jegyzett` 13,079 : `jegyezett` 206 while keeping its vowel in the infinitive, and
    // `közöl` is the reverse, `közli` 120,588 beside `közölt` 61,583 and no `közlött` at all [WC2].
    // 250 of the 687 attested verbs syncopate here against 365 that do not.
    let past = |lemma: &str| {
        let e = lexicon()
            .get_pos(lemma, Pos::Verb)
            .unwrap_or_else(|| panic!("{lemma} is a verb"));
        generate(
            e,
            &[Morph::Verbal {
                mood: Mood::Ind,
                tense: Tense::Past,
                person: Person::Third,
                number: Number::Sing,
                definite: Definite::Ind,
            }],
        )
        .expect("generates")
    };
    assert_eq!(past("jegyez"), "jegyzett");
    assert_eq!(past("közöl"), "közölt");
    assert_eq!(past("énekel"), "énekelt");
    assert_ne!(
        picur_morph_spell::check("közlött"),
        picur_morph_spell::Status::Ok
    );
}

#[test]
fn the_short_stem_carries_the_potential_and_the_converb() {
    // VRB-STEM-012: `alhat ~ aludhat`, `alva ~ aludva`, `fekhet`, `nyughat`. The short stem is
    // confined to those two cells, so the finite ones stay on the `d` stem.
    for form in ["alhat", "fekhet", "nyughat", "alva", "fekve"] {
        assert_eq!(
            picur_morph_spell::check(form),
            picur_morph_spell::Status::Ok,
            "{form}"
        );
    }
    // `aljon` is left out: it is the superessive of the noun `alj`.
    for form in ["alni", "alusszanak"] {
        assert_ne!(
            picur_morph_spell::check(form),
            picur_morph_spell::Status::Ok,
            "{form}"
        );
    }
    // The class is closed: `haragszik` keeps its `d` stem throughout.
    assert_ne!(
        picur_morph_spell::check("haraghat"),
        picur_morph_spell::Status::Ok
    );
}

#[test]
fn a_derived_stem_is_no_longer_an_ik_verb() {
    // VRB-IK-002 holds of an `-ik` verb, and the potential of one is not: `aludhatok` 790,
    // `dolgozhatok` 7,003 against the definite `dolgozhatom` 1,429 [WC2].
    let pot = |lemma: &str, definite| {
        let e = lexicon().get_pos(lemma, Pos::Verb).expect("lemma");
        generate(
            e,
            &[
                Morph::Potential,
                Morph::Verbal {
                    mood: Mood::Ind,
                    tense: Tense::Pres,
                    person: Person::First,
                    number: Number::Sing,
                    definite,
                },
            ],
        )
        .expect("generates")
    };
    assert_eq!(pot("dolgozik", Definite::Ind), "dolgozhatok");
    assert_eq!(pot("dolgozik", Definite::Def), "dolgozhatom");
    assert_eq!(pot("alszik", Definite::Ind), "aludhatok");
}

#[test]
fn a_suppletive_present_form_is_read_back() {
    // VRB-IRR-001: `vagy` is the whole 2sg cell, so no suffix ends the surface.
    for form in ["vagy", "vagyok", "vagyunk", "mész", "megy", "jössz"] {
        assert_eq!(
            picur_morph_spell::check(form),
            picur_morph_spell::Status::Ok,
            "{form}"
        );
    }
    let readings = picur_morph_spell::analyses("vagy");
    assert!(
        readings.iter().any(|a| a.lemma.as_ref() == "van"),
        "no `van` reading: {readings:?}"
    );
}
