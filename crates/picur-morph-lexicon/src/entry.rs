//! Lexicon entries.

use serde::{Deserialize, Serialize};

use crate::TraitSet;

/// Part of speech.
///
/// The inventory covers the language, not the source files the converter happens to read.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Pos {
    /// Common noun.
    Noun,
    /// Proper noun.
    ProperNoun,
    /// Adjective.
    Adjective,
    /// Verb.
    Verb,
    /// Numeral.
    Numeral,
    /// Pronoun.
    Pronoun,
    /// Postposition.
    Postposition,
    /// Adverb.
    Adverb,
    /// Determiner or article.
    Determiner,
    /// Conjunction.
    Conjunction,
    /// Verbal prefix, standing alone.
    Preverb,
    /// Interjection.
    Interjection,
}

impl Pos {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Noun,
        Self::ProperNoun,
        Self::Adjective,
        Self::Verb,
        Self::Numeral,
        Self::Pronoun,
        Self::Postposition,
        Self::Adverb,
        Self::Determiner,
        Self::Conjunction,
        Self::Preverb,
        Self::Interjection,
    ];

    /// Whether this part of speech inflects for case and number.
    #[must_use]
    pub const fn is_nominal(self) -> bool {
        matches!(
            self,
            Self::Noun | Self::ProperNoun | Self::Adjective | Self::Numeral | Self::Pronoun
        )
    }
}

/// One lexeme.
///
/// `parts` holds compound seam *offsets* into `lemma`, not owned copies of the members, whose
/// bytes the lemma already holds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    /// Citation form.
    pub lemma: Box<str>,
    /// Byte offsets of compound seams within `lemma`, ascending, never 0 or `lemma.len()`.
    #[serde(default, skip_serializing_if = "<[u32]>::is_empty")]
    pub parts: Box<[u32]>,
    /// Part of speech.
    pub pos: Pos,
    /// Lexical properties no rule predicts.
    #[serde(default, skip_serializing_if = "TraitSet::is_empty")]
    pub traits: TraitSet,
    /// A bound stem that no rule derives from the lemma.
    ///
    /// Only the sz-d(-v) verbs need this: `alszik` has the present stem `alsz-` in the lemma but a
    /// `d` stem `alud-` whose vowel is not predictable (`alud-` against `nyugod-`, both back). See
    /// `VRB-STEM-012`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<Box<str>>,
    /// The last spoken sound of a foreign spelling, as a consonant letter, where the pronunciation
    /// itself is not recorded.
    ///
    /// Only a [`Trait::Spoken`](crate::Trait::Spoken) entry reads it, and only when `alt` is
    /// absent: `League` with `g` takes `League-gel`, and absence means the word ends in a vowel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_sound: Option<Box<str>>,
}

impl Entry {
    /// A simplex entry with no traits.
    pub fn new(lemma: impl Into<Box<str>>, pos: Pos) -> Self {
        Self {
            lemma: lemma.into(),
            parts: Box::default(),
            pos,
            traits: TraitSet::EMPTY,
            alt: None,
            last_sound: None,
        }
    }

    /// Whether this trait is set.
    #[must_use]
    pub const fn has(&self, t: crate::Trait) -> bool {
        self.traits.has(t)
    }

    /// Whether the lemma is a compound.
    #[must_use]
    pub const fn is_compound(&self) -> bool {
        !self.parts.is_empty()
    }

    /// The compound members, or the whole lemma if simplex.
    pub fn members(&self) -> impl Iterator<Item = &str> {
        let mut prev = 0usize;
        self.parts
            .iter()
            .map(|&o| o as usize)
            .chain(std::iter::once(self.lemma.len()))
            .map(move |end| {
                let s = &self.lemma[prev..end];
                prev = end;
                s
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{Entry, Pos};
    use crate::{Trait, TraitSet};

    #[test]
    fn simplex_yields_one_member() {
        let e = Entry::new("ház", Pos::Noun);
        assert!(!e.is_compound());
        assert_eq!(e.members().collect::<Vec<_>>(), vec!["ház"]);
    }

    #[test]
    fn compound_members_split_on_offsets() {
        let e = Entry {
            lemma: "abbahagy".into(),
            parts: vec![4].into_boxed_slice(),
            pos: Pos::Verb,
            traits: TraitSet::from_iter_of([Trait::Preverb]),
            alt: None,
            last_sound: None,
        };
        assert!(e.is_compound());
        assert_eq!(e.members().collect::<Vec<_>>(), vec!["abba", "hagy"]);
        assert!(e.has(Trait::Preverb));
    }

    #[test]
    fn multibyte_offsets_split_correctly() {
        // "gyümölcslé" = "gyümölcs" + "lé"; "gyümölcs" is 10 bytes (ü, ö are 2 each).
        let lemma = "gyümölcslé";
        let seam = lemma.find("lé").expect("seam");
        let e = Entry {
            lemma: lemma.into(),
            parts: vec![u32::try_from(seam).expect("fits")].into_boxed_slice(),
            pos: Pos::Noun,
            traits: TraitSet::EMPTY,
            alt: None,
            last_sound: None,
        };
        assert_eq!(e.members().collect::<Vec<_>>(), vec!["gyümölcs", "lé"]);
    }

    #[test]
    fn nominal_classification() {
        assert!(Pos::Noun.is_nominal());
        assert!(!Pos::Verb.is_nominal());
        assert_eq!(Pos::ALL.len(), 12);
    }
}
