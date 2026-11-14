//! Projecting a reading into Universal Dependencies notation.
//!
//! A translation, not a second analysis. Everything here is read off the reading's own category
//! and feature stack; nothing consults the surface, and no feature is emitted that the stack does
//! not carry. Where UD has no home for something the rulebook distinguishes, the feature is left
//! out rather than approximated, which is the same decision [`Case::ud`](crate::features::Case::ud) already records.
//!
//! The point of the projection is to be *comparable*, so that agreement with a hand-annotated
//! corpus can be measured. It is deliberately not tuned to any particular treebank: where this
//! disagrees with an annotation, the disagreement is a finding to be judged, not a defect to be
//! patched here.

use picur_morph_lexicon::{Pos, Trait, lexicon};

use crate::analysis::Analysis;
use crate::features::{Definite, Mood, Morph, Number, Person, Tense};

/// A reading in UD notation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ud {
    /// The universal part of speech tag.
    pub upos: &'static str,
    /// Features, as `(name, value)` in UD's own alphabetical order.
    pub feats: Vec<(&'static str, &'static str)>,
}

impl Ud {
    /// The features as UD writes them: `Case=Ine|Number=Sing`, or `_` when there are none.
    #[must_use]
    pub fn feats(&self) -> String {
        if self.feats.is_empty() {
            return "_".to_owned();
        }
        self.feats
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("|")
    }
}

/// The universal tag for a lexical category.
///
/// The one irreducible table here: UD's tag names are a fixed inventory and cannot be derived from
/// anything. Everything else in this module is computed from the stack.
#[expect(
    clippy::match_same_arms,
    reason = "a preverb standing alone is tagged ADV like an adverb, but it is a category of its \
              own in the rulebook and merging the arms would hide that"
)]
const fn upos(pos: Pos) -> &'static str {
    match pos {
        Pos::Noun => "NOUN",
        Pos::ProperNoun => "PROPN",
        Pos::Adjective => "ADJ",
        Pos::Verb => "VERB",
        Pos::Numeral => "NUM",
        Pos::Pronoun => "PRON",
        Pos::Postposition => "ADP",
        Pos::Adverb => "ADV",
        Pos::Determiner => "DET",
        Pos::Conjunction => "CCONJ",
        Pos::Preverb => "ADV",
        Pos::Interjection => "INTJ",
        _ => "X",
    }
}

/// Whether a category inflects as a nominal, and so carries `Number` and `Case`.
const fn is_nominal(pos: Pos) -> bool {
    matches!(
        pos,
        Pos::Noun | Pos::ProperNoun | Pos::Adjective | Pos::Numeral | Pos::Pronoun
    )
}

const fn person(p: Person) -> &'static str {
    match p {
        Person::First => "1",
        Person::Second => "2",
        Person::Third => "3",
    }
}

const fn number(n: Number) -> &'static str {
    match n {
        Number::Sing => "Sing",
        Number::Plur => "Plur",
    }
}

/// The verb form a non-finite morph names, if it names one.
const fn verb_form(morph: Morph) -> Option<(&'static str, Option<&'static str>)> {
    match morph {
        Morph::Infinitive => Some(("Inf", None)),
        Morph::PresentParticiple => Some(("Part", Some("Pres"))),
        Morph::PastParticiple => Some(("Part", Some("Past"))),
        Morph::FutureParticiple => Some(("Part", Some("Fut"))),
        Morph::Converb => Some(("Conv", None)),
        _ => None,
    }
}

/// The degree a grading morph names, if it names one.
const fn degree(morph: Morph) -> Option<&'static str> {
    match morph {
        Morph::Comparative => Some("Cmp"),
        Morph::Superlative => Some("Sup"),
        Morph::Excessive => Some("Abs"),
        _ => None,
    }
}

/// Whether the reading's lexeme is an article.
fn is_article(reading: &Analysis) -> bool {
    lexicon()
        .get_pos(&reading.lemma, reading.pos)
        .is_some_and(|e| e.has(Trait::Article))
}

/// The features a finite verb cell carries.
fn finite_feats(morph: Morph) -> Vec<(&'static str, &'static str)> {
    let Morph::Verbal {
        mood,
        tense,
        person: p,
        number: n,
        definite,
    } = morph
    else {
        return Vec::new();
    };
    vec![
        (
            "Definite",
            match definite {
                Definite::Def => "Def",
                _ => "Ind",
            },
        ),
        (
            "Mood",
            match mood {
                Mood::Ind => "Ind",
                Mood::Cnd => "Cnd",
                Mood::Imp => "Imp",
            },
        ),
        ("Number", number(n)),
        ("Person", person(p)),
        (
            "Tense",
            match tense {
                Tense::Pres => "Pres",
                Tense::Past => "Past",
            },
        ),
        ("VerbForm", "Fin"),
    ]
}

/// Projects `reading` into UD notation.
///
/// The tag follows the reading's category, except that a non-finite verb form keeps its verbal tag
/// and says so through `VerbForm`, which is what UD does: a participle is a `VERB`, not an `ADJ`.
#[must_use]
pub fn project(reading: &Analysis) -> Ud {
    let mut feats: Vec<(&'static str, &'static str)> = Vec::new();
    let mut finite = false;
    let mut nonfinite = false;

    for morph in &reading.stack {
        if let Some((form, tense)) = verb_form(*morph) {
            nonfinite = true;
            feats.push(("VerbForm", form));
            if let Some(t) = tense {
                feats.push(("Tense", t));
            }
        }
        if let Some(d) = degree(*morph) {
            feats.push(("Degree", d));
        }
        match morph {
            // Both write `Number=Plur`, and they are different facts that happen to share a
            // notation: the plural of the word, and the plural of the thing possessed. UD has one
            // feature for the noun's own number and does not distinguish which put it there.
            Morph::Plural | Morph::PossessedPlural => feats.push(("Number", "Plur")),
            Morph::Possessive {
                person: p,
                number: n,
            } => {
                feats.push(("Number[psor]", number(*n)));
                feats.push(("Person[psor]", person(*p)));
            }
            Morph::Case(c) => {
                if let Some(u) = c.ud() {
                    feats.push(("Case", u.as_str()));
                }
            }
            Morph::Verbal { .. } => {
                finite = true;
                feats.extend(finite_feats(*morph));
            }
            _ => {}
        }
    }

    // The unmarked member of a paradigm is still a value UD writes: no plural marking is singular,
    // no case marking is nominative, an ungraded adjective is positive.
    let has = |feats: &[(&str, &str)], name: &str| feats.iter().any(|(k, _)| *k == name);
    if is_nominal(reading.pos) && !finite && !nonfinite {
        if !has(&feats, "Number") {
            feats.push(("Number", "Sing"));
        }
        if !has(&feats, "Case") {
            feats.push(("Case", "Nom"));
        }
    }
    // An article says so, and says which: `PronType=Art` with `Definite`. The lexicon marks the
    // two that are articles, because the category does not tell them from the other determiners.
    if reading.pos == Pos::Determiner && is_article(reading) {
        feats.push(("PronType", "Art"));
        feats.push(("Definite", "Def"));
    }
    if reading.pos == Pos::Adjective && !has(&feats, "Degree") {
        feats.push(("Degree", "Pos"));
    }
    // Every verb is active: the rulebook has no passive paradigm, so the feature has one value
    // wherever it applies at all. It applies to the infinitive as much as to a finite cell.
    if finite || has(&feats, "VerbForm") && !has(&feats, "Voice") {
        feats.push(("Voice", "Act"));
    }

    // UD orders features alphabetically, comparing case-insensitively.
    feats.sort_by_key(|(name, _)| name.to_lowercase());
    feats.dedup();

    Ud {
        upos: upos(reading.pos),
        feats,
    }
}

#[cfg(test)]
mod tests {
    use super::project;
    use crate::analysis::Analysis;
    use crate::features::{Case, Morph, Number, Person};
    use picur_morph_lexicon::Pos;

    fn reading(lemma: &str, pos: Pos, stack: Vec<Morph>) -> Analysis {
        Analysis {
            lemma: lemma.into(),
            pos,
            stack,
        }
    }

    #[test]
    fn a_bare_noun_is_singular_nominative() {
        let ud = project(&reading("ház", Pos::Noun, Vec::new()));
        assert_eq!(ud.upos, "NOUN");
        assert_eq!(ud.feats(), "Case=Nom|Number=Sing");
    }

    #[test]
    fn a_possessed_and_cased_noun_writes_every_slot() {
        let ud = project(&reading(
            "ház",
            Pos::Noun,
            vec![
                Morph::PossessedPlural,
                Morph::Possessive {
                    person: Person::First,
                    number: Number::Sing,
                },
                Morph::Case(Case::Ine),
            ],
        ));
        assert_eq!(
            ud.feats(),
            "Case=Ine|Number=Plur|Number[psor]=Sing|Person[psor]=1"
        );
    }

    #[test]
    fn a_finite_verb_is_not_given_nominal_features() {
        let ud = project(&reading(
            "olvas",
            Pos::Verb,
            vec![Morph::Verbal {
                mood: crate::features::Mood::Ind,
                tense: crate::features::Tense::Past,
                person: Person::Third,
                number: Number::Sing,
                definite: crate::features::Definite::Def,
            }],
        ));
        assert_eq!(ud.upos, "VERB");
        assert_eq!(
            ud.feats(),
            "Definite=Def|Mood=Ind|Number=Sing|Person=3|Tense=Past|VerbForm=Fin|Voice=Act"
        );
    }

    /// A participle stays a verb and says so, which is UD's own treatment.
    #[test]
    fn a_participle_keeps_its_verbal_tag() {
        let ud = project(&reading("olvas", Pos::Verb, vec![Morph::PastParticiple]));
        assert_eq!(ud.upos, "VERB");
        assert_eq!(ud.feats(), "Tense=Past|VerbForm=Part|Voice=Act");
    }
}
