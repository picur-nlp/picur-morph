//! Personal pronouns.
//!
//! The oblique cases of a personal pronoun are not built on the pronoun at all: `PND-PERS-003` puts
//! a possessive-type personal suffix on a **stem of the case suffix**, so `nekem` is `nek-` plus
//! `-em` and the pronoun `én` never appears. Twelve such stems exist and nothing derives them from
//! the case suffixes they belong to, so they are stated; everything after the stem is
//! `PND-PERS-004` and is computed.
//!
//! See `docs/grammar/04-pronouns-numerals-determiners.md`.

use std::sync::LazyLock;

use picur_morph_lexicon::{Role, pairs, rows};

use crate::features::named;

/// The person and number a row names in its two cells from `at`.
fn cell(row: &[Box<str>], at: usize) -> (Person, Number) {
    (
        named(&Person::ALL, &row[at]),
        named(&Number::ALL, &row[at + 1]),
    )
}

/// `PND-PERS-003`: the oblique stems, one per case.
static STEMS: LazyLock<Vec<(Case, &'static str)>> = LazyLock::new(|| {
    rows(Role::PronounStem)
        .iter()
        .map(|r| (named(&Case::ALL, &r[0]), r[1].as_ref()))
        .collect()
});

/// `PND-PERS-004`: the oblique forms the suffix rule does not reach.
static IRREGULAR: LazyLock<Vec<(Case, Person, Number, &'static str)>> = LazyLock::new(|| {
    rows(Role::PronounIrregular)
        .iter()
        .map(|r| {
            let (p, n) = cell(r, 1);
            (named(&Case::ALL, &r[0]), p, n, r[3].as_ref())
        })
        .collect()
});

/// `PND-PERS-001`, `PND-PERS-002`: the nominatives and the suppletive accusatives.
fn person_forms(role: Role) -> Vec<(Person, Number, &'static str)> {
    rows(role)
        .iter()
        .map(|r| {
            let (p, n) = cell(r, 0);
            (p, n, r[2].as_ref())
        })
        .collect()
}

static NOMINATIVE: LazyLock<Vec<(Person, Number, &'static str)>> =
    LazyLock::new(|| person_forms(Role::PronounNominative));
static ACCUSATIVE: LazyLock<Vec<(Person, Number, &'static str)>> =
    LazyLock::new(|| person_forms(Role::PronounAccusative));

use picur_morph_ortho::{Harmony, harmony_of, letters};

use crate::features::{Case, Number, Person};
use crate::suffix::{Arch, Seg};

/// Whether `lemma` is a personal pronoun and nothing else: `mi` is also the interrogative,
/// which inflects freely (`valamiket`), so it is not one.
#[must_use]
pub fn is_personal(lemma: &str) -> bool {
    lemma != "mi" && NOMINATIVE.iter().any(|(_, _, l)| *l == lemma)
}

/// The personal suffix for a stem, by the stem's ending and harmony.
///
/// The two shapes are the whole of `PND-PERS-004`: after a consonant the suffix carries its own
/// vowel, after a long vowel it does not and the third singular is zero. Front rounded stems take
/// no `ö` here, which the archiphonemes give for free because the preceding vowel is unrounded.
///
/// Rules: `PND-PERS-004`
pub(crate) fn personal_suffix(vowel_final: bool, person: Person, number: Number) -> &'static [Seg] {
    use Arch::{A, U};
    use Seg::{Lit, Var};
    match (vowel_final, person, number) {
        (false, Person::First, Number::Sing) => &[Var(A), Lit("m")],
        (false, Person::Second, Number::Sing) => &[Var(A), Lit("d")],
        (false, Person::Third, Number::Sing) => &[Var(A)],
        (false, Person::First, Number::Plur) => &[Var(U), Lit("nk")],
        (false, Person::Second, Number::Plur) => &[Var(A), Lit("t"), Var(Arch::O), Lit("k")],
        (false, Person::Third, Number::Plur) => &[Var(U), Lit("k")],
        (true, Person::First, Number::Sing) => &[Lit("m")],
        (true, Person::Second, Number::Sing) => &[Lit("d")],
        (true, Person::Third, Number::Sing) => &[],
        (true, Person::First, Number::Plur) => &[Lit("nk")],
        (true, Person::Second, Number::Plur) => &[Lit("t"), Var(Arch::O), Lit("k")],
        (true, Person::Third, Number::Plur) => &[Lit("j"), Var(U), Lit("k")],
    }
}

/// Realises a segment list against a harmony class, as [`crate::suffix`] does for a real suffix.
pub(crate) fn realise(segs: &[Seg], h: Harmony) -> String {
    let mut out = String::new();
    let mut h = h;
    for seg in segs {
        match *seg {
            Seg::Lit(s) => out.push_str(s),
            Seg::Var(a) => {
                out.push_str(a.realise(h));
                h = a.rounding_after(h);
            }
            Seg::AssimV => out.push('v'),
            Seg::AssimJ => out.push('j'),
        }
    }
    out
}

/// The oblique personal pronoun for a case, person and number.
///
/// `None` when the case has no pronominal form (`PND-PERS-005`).
///
/// Rules: `PND-PERS-003`, `PND-PERS-004`
#[must_use]
pub fn oblique(case: Case, person: Person, number: Number) -> Option<String> {
    if let Some((_, _, _, form)) = IRREGULAR
        .iter()
        .find(|(c, p, n, _)| *c == case && *p == person && *n == number)
    {
        return Some((*form).to_owned());
    }
    let stem = STEMS.iter().find(|(c, _)| *c == case).map(|(_, s)| *s)?;
    let vowel_final = letters(stem).last().is_some_and(|s| s.letter.is_vowel());
    let tail = realise(
        personal_suffix(vowel_final, person, number),
        harmony_of(stem),
    );
    Some(format!("{stem}{tail}"))
}

/// One recognised personal pronoun form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PronounForm {
    /// The case it expresses.
    pub case: Case,
    /// Person.
    pub person: Person,
    /// Number.
    pub number: Number,
}

/// The six cells of a personal paradigm, in person-then-number order.
pub const CELLS: [(Person, Number); 6] = [
    (Person::First, Number::Sing),
    (Person::Second, Number::Sing),
    (Person::Third, Number::Sing),
    (Person::First, Number::Plur),
    (Person::Second, Number::Plur),
    (Person::Third, Number::Plur),
];

/// Every personal pronoun form, generated from the rules rather than listed.
fn table() -> Vec<(String, PronounForm)> {
    let mut out = Vec::new();
    let cells = CELLS;
    for &(person, number, form) in NOMINATIVE.iter() {
        out.push((
            form.to_owned(),
            PronounForm {
                case: Case::Nom,
                person,
                number,
            },
        ));
    }
    for &(person, number, form) in ACCUSATIVE.iter() {
        out.push((
            form.to_owned(),
            PronounForm {
                case: Case::Acc,
                person,
                number,
            },
        ));
    }
    for &(case, _) in STEMS.iter() {
        for (person, number) in cells {
            if let Some(form) = oblique(case, person, number) {
                out.push((
                    form,
                    PronounForm {
                        case,
                        person,
                        number,
                    },
                ));
            }
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

static TABLE: LazyLock<Vec<(String, PronounForm)>> = LazyLock::new(table);

/// The personal pronoun reading of `word`, if it has one.
///
/// Rules: `PND-PERS-001` .. `PND-PERS-004`
#[must_use]
pub fn nominative(person: Person, number: Number) -> &'static str {
    NOMINATIVE
        .iter()
        .find(|(p, n, _)| *p == person && *n == number)
        .map_or("", |(_, _, form)| *form)
}

/// The citation form of a reflexive pronoun stem.
///
/// `PND-REFL-001` states the reflexive as the stem `mag-` plus a possessive suffix, so the stem is
/// not itself a word and cannot be the lemma of `magam` or `maguk`. The dictionary form is the
/// third person singular, exactly as the personal pronoun's is its nominative.
///
/// Rules: `PND-REFL-001`
#[must_use]
pub fn reflexive_citation(stem: &str) -> Option<&'static str> {
    match stem {
        "mag" => Some("maga"),
        "önmag" => Some("önmaga"),
        _ => None,
    }
}

/// One emphatic doubled form, by what the prefix was written before.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emphatic {
    /// A case-stem pronoun form: `énnekem`, `őrá`.
    Pronoun(PronounForm),
    /// A personal-suffixed postposition: `énmiattam`, `őközöttük`.
    Postposition(crate::postposition::PostpositionForm),
}

/// The emphatic doubled form `word` is, if it is one.
///
/// `PND-PERS-007` writes the nominative pronoun before the case-stem form (`PND-PERS-003`) or the
/// personal postposition (`PND-POSTP-005`) of the same person as one word. Nothing assimilates or
/// harmonises across the boundary, so the host is the free form unchanged; the third person
/// prefixes with `ő` in both numbers and `ők` prefixes nothing; the nominative and the accusative
/// have no such form.
///
/// Rules: `PND-PERS-007`
#[must_use]
pub fn emphatic(word: &str) -> Option<Emphatic> {
    for (person, number) in CELLS {
        let carrier = if person == Person::Third {
            Number::Sing
        } else {
            number
        };
        let Some(rest) = word.strip_prefix(nominative(person, carrier)) else {
            continue;
        };
        if let Some(p) = analyse(rest)
            && (p.person, p.number) == (person, number)
            && !matches!(p.case, Case::Nom | Case::Acc)
        {
            return Some(Emphatic::Pronoun(p));
        }
        if let Some(f) = crate::postposition::analyse(rest)
            && (f.person, f.number) == (person, number)
        {
            return Some(Emphatic::Postposition(f));
        }
    }
    None
}

/// Recognises a personal pronoun form.
#[must_use]
pub fn analyse(word: &str) -> Option<PronounForm> {
    let t = &*TABLE;
    let i = t.partition_point(|(f, _)| f.as_str() < word);
    t.get(i).filter(|(f, _)| f == word).map(|(_, p)| *p)
}

/// The demonstrative adverb `surface` is the `-rA` form of, if it is one (`PND-DEM-001`).
#[must_use]
pub fn demonstrative_adverb(surface: &str) -> Option<&'static str> {
    pairs(Role::DemonstrativeSublative)
        .iter()
        .find(|(_, form)| form.as_ref() == surface)
        .map(|(lemma, _)| lemma.as_ref())
}

/// Every form of the table, with the lemma it belongs to, sorted for lookup.
///
/// Built once. `PND-PERS-005` has these behave as nouns once formed, so the case endings are the
/// ordinary ones and are generated rather than listed.
fn possessive_table() -> Vec<(String, &'static str)> {
    let mut out = Vec::new();
    for row in rows(Role::PossessivePronoun) {
        let lemma: &'static str = &row[0];
        for base in &row[1..] {
            let base: &str = base;
            out.push((base.to_owned(), lemma));
            let entry = picur_morph_lexicon::Entry::new(base, picur_morph_lexicon::Pos::Noun);
            for case in crate::features::Case::ALL {
                if let Ok(form) =
                    crate::generate::generate(&entry, &[crate::features::Morph::Case(case)])
                {
                    out.push((form, lemma));
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

static POSSESSIVE_TABLE: LazyLock<Vec<(String, &'static str)>> = LazyLock::new(possessive_table);

/// The lemma of an independent possessive pronoun, bare or with a case suffix.
///
/// Rules: `PND-REFL-005`, `PND-PERS-005`
#[must_use]
pub fn possessive_pronoun(surface: &str) -> Option<&'static str> {
    let t = &*POSSESSIVE_TABLE;
    let hit = |w: &str| {
        let i = t.partition_point(|(f, _)| f.as_str() < w);
        t.get(i).filter(|(f, _)| f == w).map(|(_, l)| *l)
    };
    hit(surface).or_else(|| hit(&surface.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::{analyse, oblique};
    use crate::features::{Case, Number, Person};

    fn row(case: Case) -> Vec<String> {
        [
            (Person::First, Number::Sing),
            (Person::Second, Number::Sing),
            (Person::Third, Number::Sing),
            (Person::First, Number::Plur),
            (Person::Second, Number::Plur),
            (Person::Third, Number::Plur),
        ]
        .into_iter()
        .map(|(p, n)| oblique(case, p, n).unwrap_or_default())
        .collect()
    }

    #[test]
    fn the_whole_table_of_pers_003() {
        assert_eq!(
            row(Case::Dat),
            ["nekem", "neked", "neki", "nekünk", "nektek", "nekik"]
        );
        assert_eq!(
            row(Case::Ins),
            ["velem", "veled", "vele", "velünk", "veletek", "velük"]
        );
        assert_eq!(
            row(Case::Cau),
            ["értem", "érted", "érte", "értünk", "értetek", "értük"]
        );
        assert_eq!(
            row(Case::Ine),
            ["bennem", "benned", "benne", "bennünk", "bennetek", "bennük"]
        );
        assert_eq!(
            row(Case::Ela),
            [
                "belőlem",
                "belőled",
                "belőle",
                "belőlünk",
                "belőletek",
                "belőlük"
            ]
        );
        assert_eq!(
            row(Case::Sup),
            ["rajtam", "rajtad", "rajta", "rajtunk", "rajtatok", "rajtuk"]
        );
        assert_eq!(
            row(Case::Sbl),
            ["rám", "rád", "rá", "ránk", "rátok", "rájuk"]
        );
        assert_eq!(
            row(Case::Del),
            ["rólam", "rólad", "róla", "rólunk", "rólatok", "róluk"]
        );
        assert_eq!(
            row(Case::Ade),
            ["nálam", "nálad", "nála", "nálunk", "nálatok", "náluk"]
        );
        assert_eq!(
            row(Case::All),
            [
                "hozzám",
                "hozzád",
                "hozzá",
                "hozzánk",
                "hozzátok",
                "hozzájuk"
            ]
        );
        assert_eq!(
            row(Case::Abl),
            ["tőlem", "tőled", "tőle", "tőlünk", "tőletek", "tőlük"]
        );
    }

    #[test]
    fn a_front_rounded_stem_takes_no_o_umlaut_in_the_suffix() {
        // PND-PERS-004: `tőletek`, not `*tőletök`, although `től-` is front rounded.
        assert_eq!(
            oblique(Case::Abl, Person::Second, Number::Plur).as_deref(),
            Some("tőletek")
        );
        assert_eq!(
            oblique(Case::Ela, Person::Second, Number::Plur).as_deref(),
            Some("belőletek")
        );
    }

    #[test]
    fn a_vowel_final_stem_has_a_zero_third_singular() {
        assert_eq!(
            oblique(Case::Sbl, Person::Third, Number::Sing).as_deref(),
            Some("rá")
        );
        assert_eq!(
            oblique(Case::All, Person::Third, Number::Sing).as_deref(),
            Some("hozzá")
        );
    }

    #[test]
    fn the_three_irregular_datives_are_stated() {
        // PND-PERS-004 names them: the rule would give `*neke`, `*nekük` and `*nekentek`.
        assert_eq!(
            oblique(Case::Dat, Person::Third, Number::Sing).as_deref(),
            Some("neki")
        );
        assert_eq!(
            oblique(Case::Dat, Person::Third, Number::Plur).as_deref(),
            Some("nekik")
        );
        assert_eq!(
            oblique(Case::Dat, Person::Second, Number::Plur).as_deref(),
            Some("nektek")
        );
    }

    #[test]
    fn forms_are_recognised_back() {
        for (word, case) in [
            ("nekem", Case::Dat),
            ("veled", Case::Ins),
            ("benned", Case::Ine),
            ("rád", Case::Sbl),
            ("tőlük", Case::Abl),
            ("hozzánk", Case::All),
            ("én", Case::Nom),
            ("engem", Case::Acc),
            ("őket", Case::Acc),
        ] {
            assert_eq!(analyse(word).map(|p| p.case), Some(case), "{word}");
        }
        assert!(analyse("ház").is_none());
    }

    #[test]
    fn a_case_without_a_pronominal_form_has_none() {
        // PND-PERS-005.
        assert!(oblique(Case::Ter, Person::First, Number::Sing).is_none());
        assert!(oblique(Case::Tem, Person::First, Number::Sing).is_none());
    }
}
