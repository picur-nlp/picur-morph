//! Stem shape: what a suffix sees when it attaches.

use picur_morph_lexicon::{Entry, Pos, Trait};
use picur_morph_ortho::{Letter, bare_t_class, letters};

use crate::suffix::{Arch, Suffix, SuffixClass};

/// The shape of a stem's final consonants, in **letters** rather than characters.
///
/// A digraph is one letter, so `varázs` ends in a single `zs` and `stressz` ends in a geminate
/// `sz`, not in a four-character cluster.
///
/// Rules: `NOM-ACC-002`, `NOM-ACC-003`, `NOM-ACC-006`, `NOM-ACC-009`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coda {
    /// Ends in a vowel.
    Vowel,
    /// Exactly one final consonant.
    Single(Letter),
    /// Two identical final consonants (`orr`, `stressz`, `toll`).
    Geminate(Letter),
    /// Two or more different final consonants; carries the last one.
    Cluster(Letter),
}

/// Classifies the end of a word.
///
/// Rules: `NOM-ACC-002`
#[must_use]
pub fn coda(stem: &str) -> Coda {
    let (mut prev, mut last) = (None, None);
    for span in letters(stem) {
        prev = last;
        last = Some(span.letter);
    }
    let Some(last) = last else {
        return Coda::Vowel;
    };
    if last.is_vowel() {
        return Coda::Vowel;
    }
    match prev {
        Some(prev) if prev == last => Coda::Geminate(last),
        Some(prev) if prev.is_consonant() => Coda::Cluster(last),
        _ => Coda::Single(last),
    }
}

/// Whether a linking vowel intervenes before `suffix`.
///
/// The computed default comes from the coda; a lexeme may override it in **either** direction,
/// which is how `orr` gives `orrot` while `stressz` gives `stresszt` despite identical shapes.
///
/// Whether `NOM-PL-003` applies: a vowel-final stem that still takes a linking vowel.
///
/// A vowel-final stem normally takes none, but an adjective ending in a non-low vowel takes a low
/// one before the plural: `budapesti` gives `budapestiek` (21,705 against `budapestik` 6 `[WC2]`)
/// and `hosszú` gives `hosszúak` (22,897 against `hosszúok` 1).
///
/// The accusative is excluded by its bare-coda flag, which no other class I suffix carries: the
/// accusative of the same words is `budapestit` and `férfit`, with no link at all.
///
/// Rules: `NOM-PL-003`
fn high_vowel_plural(stem: &str, entry: &Entry, suffix: &Suffix) -> bool {
    if !suffix.high_vowel_link || !gradable_or_marked(entry) {
        return false;
    }
    // `NOM-PL-003` makes the link obligatory after the vowels `high_vowel_final` names and
    // *optional* after `ó`/`ő`: `bántók` ~ `bántóak`, `olcsók` ~ `olcsóak`. Generation takes the
    // preferred `-k`, so the linked form is reached through the marked variant.
    high_vowel_final(stem) || (entry.has(Trait::LinkedPlural) && mid_rounded_final(stem))
}

/// Whether the stem ends in `ó` or `ő`, the vowels `NOM-PL-003` leaves optional.
fn mid_rounded_final(stem: &str) -> bool {
    letters(stem)
        .last()
        .is_some_and(|s| matches!(s.letter, Letter::OAcute | Letter::ODblAcute))
}

/// Whether the stem ends in one of the non-low vowels `NOM-PL-003` names.
fn high_vowel_final(stem: &str) -> bool {
    letters(stem).last().is_some_and(|s| {
        matches!(
            s.letter,
            Letter::I
                | Letter::IAcute
                | Letter::U
                | Letter::UAcute
                | Letter::UUmlaut
                | Letter::UDblAcute
        )
    })
}

/// Whether `NOM-PL-003` reaches this entry: an adjective, or a noun the rule names.
fn gradable_or_marked(entry: &Entry) -> bool {
    // `férfi` is the noun the rule lists beside the adjectives (`férfiak`, 633,484 against
    // `férfik` 285 `[WC2]`), and it is reached by its own trait rather than by its category.
    entry.pos == Pos::Adjective || entry.has(Trait::LinkedPlural)
}

/// Rules: `NOM-ACC-003`, `NOM-ACC-005`, `NOM-ACC-006`, `NOM-ACC-009`
#[must_use]
pub fn needs_link(stem: &str, entry: &Entry, suffix: &Suffix, lowering: bool) -> bool {
    if coda(stem) == Coda::Vowel {
        return high_vowel_plural(stem, entry, suffix);
    }
    // Only the accusative has a bare-coda class: `dal` gives `dalt` but `dalok`.
    if !suffix.bare_coda {
        return true;
    }
    if entry.has(Trait::BareAccusative) {
        return false;
    }
    if entry.has(Trait::LinkedAccusative) {
        return true;
    }
    match coda(stem) {
        Coda::Vowel => false,
        // NOM-ACC-005: a lowering stem takes the vowel even in a bare class.
        Coda::Single(c) | Coda::Geminate(c) => lowering || !bare_t_class(c),
        // NOM-ACC-009: clusters take the vowel by default.
        Coda::Cluster(_) => true,
    }
}

/// The linking vowel before `suffix`, if any.
///
/// Only class I suffixes take one. A lowering stem makes it low, but **only** for a lowerable
/// suffix: the superessive is class I and not lowerable, which is why `ház` gives `házak` and
/// `házat` but `házon`.
///
/// Rules: `NOM-LOW-001`
#[must_use]
pub fn linking_vowel(stem: &str, entry: &Entry, suffix: &Suffix, lowering: bool) -> Option<Arch> {
    if suffix.class != SuffixClass::One {
        return None;
    }
    if !needs_link(stem, entry, suffix, lowering) {
        return None;
    }
    // NOM-PL-003 calls for a low vowel whatever the stem's own lowering value, so `hosszú` gives
    // `hosszúak` although it is not a lowering stem.
    if high_vowel_plural(stem, entry, suffix) {
        return Some(Arch::A);
    }
    // The unlowered vowel is whatever the template declares, not always mid `O`: the 1pl
    // possessive is `-(U)nk`, so `ház` gives `házunk` and not `*házonk`.
    Some(if lowering && suffix.lowerable {
        Arch::A
    } else {
        suffix.linking_slot().unwrap_or(Arch::O)
    })
}

#[cfg(test)]
mod tests {
    use picur_morph_lexicon::{Entry, Pos, Trait, TraitSet};
    use picur_morph_ortho::Letter;

    use super::{Coda, coda, linking_vowel, needs_link};
    use crate::features::Case;
    use crate::suffix::{Arch, case_suffix};

    fn acc() -> crate::suffix::Suffix {
        case_suffix(Case::Acc).expect("acc")
    }

    fn plain(lemma: &str) -> Entry {
        Entry::new(lemma, Pos::Noun)
    }

    fn with(lemma: &str, t: Trait) -> Entry {
        let mut e = Entry::new(lemma, Pos::Noun);
        e.traits = TraitSet::from_iter_of([t]);
        e
    }

    #[test]
    fn coda_counts_letters_not_characters() {
        assert_eq!(coda("ház"), Coda::Single(Letter::Z));
        // zs is one letter: varazs ends in a single consonant, not a cluster.
        assert_eq!(coda("varázs"), Coda::Single(Letter::Zs));
        assert_eq!(coda("alma"), Coda::Vowel);
        assert_eq!(coda("orr"), Coda::Geminate(Letter::R));
        // stressz ends in a geminate sz, not in s+s+z.
        assert_eq!(coda("stressz"), Coda::Geminate(Letter::Sz));
        assert_eq!(coda("kert"), Coda::Cluster(Letter::T));
    }

    #[test]
    fn bare_class_single_consonants_take_no_link() {
        // NOM-ACC-003: haz -> hazat is lowering, but gaz -> gazt would be bare without it.
        assert!(!needs_link("gáz", &plain("gáz"), &acc(), false));
        assert!(!needs_link("dal", &plain("dal"), &acc(), false));
        assert!(!needs_link("lány", &plain("lány"), &acc(), false));
        assert!(!needs_link("rizs", &plain("rizs"), &acc(), false));
    }

    #[test]
    fn obstruents_take_a_link() {
        for s in ["bot", "rab", "nap", "hegy", "perec", "zsiráf", "nyom"] {
            assert!(
                needs_link(s, &plain(s), &acc(), false),
                "{s} must take a link"
            );
        }
    }

    #[test]
    fn clusters_take_a_link_by_default() {
        assert!(needs_link("kert", &plain("kert"), &acc(), false));
    }

    #[test]
    fn lowering_forces_a_link_even_in_a_bare_class() {
        // NOM-ACC-005: haz is lowering and z is a bare-class sibilant, yet hazat has the vowel.
        assert!(needs_link("ház", &plain("ház"), &acc(), true));
    }

    #[test]
    fn orr_and_stressz_are_the_geminate_pair() {
        // Identical shapes: both end in a bare-class geminate, so the computed default is bare.
        assert!(!needs_link("orr", &plain("orr"), &acc(), false));
        assert!(!needs_link("stressz", &plain("stressz"), &acc(), false));
        // NOM-ACC-006 forbids generating `orrt`, so orr carries the lexical override.
        assert!(needs_link(
            "orr",
            &with("orr", Trait::LinkedAccusative),
            &acc(),
            false
        ));
        // stressz keeps the default and stays bare.
        assert!(!needs_link("stressz", &plain("stressz"), &acc(), false));
    }

    #[test]
    fn bare_accusative_trait_overrides_the_other_way() {
        // fajl is cluster-final, so the default is a link; the trait makes it bare (`fajlt`).
        assert!(needs_link("fájl", &plain("fájl"), &acc(), false));
        assert!(!needs_link(
            "fájl",
            &with("fájl", Trait::BareAccusative),
            &acc(),
            false
        ));
    }

    #[test]
    fn vowel_final_stems_never_link() {
        assert!(!needs_link("alma", &plain("alma"), &acc(), false));
        assert!(!needs_link("alma", &plain("alma"), &acc(), true));
    }

    #[test]
    fn lowering_picks_the_low_link_for_a_lowerable_suffix() {
        let acc = case_suffix(Case::Acc).expect("acc");
        assert_eq!(
            linking_vowel("ház", &plain("ház"), &acc, true),
            Some(Arch::A)
        );
        assert_eq!(
            linking_vowel("bot", &plain("bot"), &acc, false),
            Some(Arch::O)
        );
    }

    #[test]
    fn superessive_is_never_lowered() {
        // NOM-LOW-001: hazak, hazat, but hazon. One boolean on the suffix.
        let sup = case_suffix(Case::Sup).expect("sup");
        assert_eq!(
            linking_vowel("ház", &plain("ház"), &sup, true),
            Some(Arch::O),
            "superessive must keep the mid link even on a lowering stem"
        );
    }

    #[test]
    fn class_two_suffixes_never_take_a_link() {
        let ine = case_suffix(Case::Ine).expect("ine");
        assert_eq!(linking_vowel("kert", &plain("kert"), &ine, false), None);
    }
}
