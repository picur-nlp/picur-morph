//! Letter case: mapping, folding, and the non-standard mixed forms (`WS-CASE`).
//!
//! Hungarian case mapping is Unicode simple case mapping with no locale tailoring
//! (`WS-CASE-004`), so the interesting work is the multi-character letters: an initial capital
//! capitalises the first character only (`Gy`, not `GY`), and the mixed forms (`gY`, `GYőr`) are
//! spelling errors with a computable repair (`WS-CASE-007`).

use crate::letter::letters;

/// The case pattern of one letter's characters.
///
/// Rules: WS-CASE-003, WS-CASE-007.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LetterCase {
    /// `gy`, `a`.
    Lower,
    /// `Gy`: first character uppercase, the rest lowercase. Only for multi-character letters.
    Initial,
    /// `GY`, `A`.
    Upper,
    /// `gY`, `dZs`: never standard spelling.
    NonStandard,
}

/// The case pattern of the characters of one letter.
///
/// Rules: WS-CASE-003, WS-CASE-007.
#[must_use]
pub fn letter_case(text: &str) -> LetterCase {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return LetterCase::Lower;
    };
    let rest: Vec<char> = chars.collect();
    let rest_lower = rest.iter().all(|c| !c.is_uppercase());
    let rest_upper = rest.iter().all(|c| !c.is_lowercase());
    match (first.is_uppercase(), rest.is_empty()) {
        (false, true) => LetterCase::Lower,
        (true, true) => LetterCase::Upper,
        (false, false) if rest_lower => LetterCase::Lower,
        (true, false) if rest_upper => LetterCase::Upper,
        (true, false) if rest_lower => LetterCase::Initial,
        _ => LetterCase::NonStandard,
    }
}

/// Uppercases every character (`WS-CASE-004`).
///
/// Correct for an all-caps rendering of a whole word; for an initial capital use [`capitalise`],
/// which leaves `Gy` as `Gy`.
#[must_use]
pub fn upper(s: &str) -> String {
    s.to_uppercase()
}

/// Lowercases every character (`WS-CASE-004`).
#[must_use]
pub fn lower(s: &str) -> String {
    s.to_lowercase()
}

/// Uppercases the first character only, lowercasing the rest (`WS-CASE-001`).
///
/// A multi-character letter keeps its remaining characters lowercase: `gyöngy` -> `Gyöngy`.
#[must_use]
pub fn capitalise(s: &str) -> String {
    let lowered = s.to_lowercase();
    let mut out = String::with_capacity(lowered.len());
    let mut chars = lowered.chars();
    if let Some(first) = chars.next() {
        out.extend(first.to_uppercase());
        out.push_str(chars.as_str());
    }
    out
}

/// Whether `c` is a letter of some Latin-script alphabet, Hungarian or not.
///
/// `ORT-FOR-005`: a Latin-script proper name keeps its original spelling, diacritics the Hungarian
/// alphabet does not have included, so `Čapek` and `Gdańsk` are correctly written as they stand.
#[must_use]
pub fn is_latin_letter(c: char) -> bool {
    c.is_ascii_alphabetic()
        || matches!(c, '\u{C0}'..='\u{24F}' | '\u{1E00}'..='\u{1EFF}') && c.is_alphabetic()
}

/// Whether `s` begins with an uppercase character.
#[must_use]
pub fn starts_upper(s: &str) -> bool {
    s.chars().next().is_some_and(char::is_uppercase)
}

/// `s` with its first character lowercased and the rest untouched.
#[must_use]
pub fn lower_initial(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_lowercase().collect::<String>() + chars.as_str()
    })
}

/// `s` with its first character uppercased and the rest untouched, or `None` if the first
/// character is not lowercase.
#[must_use]
pub fn upper_initial(s: &str) -> Option<String> {
    let mut chars = s.chars();
    let first = chars.next()?;
    first
        .is_lowercase()
        .then(|| first.to_uppercase().collect::<String>() + chars.as_str())
}

/// `s` with its first character lowercased, or `None` if that changes nothing.
///
/// A character can be uppercase and have no lowercase at all, as the mathematical alphanumerics
/// used for bold text on social media are, so an uppercase initial is not enough: the result must
/// differ, or a caller retrying on it would retry on the same string.
#[must_use]
pub fn decapitalised(s: &str) -> Option<String> {
    if !starts_upper(s) {
        return None;
    }
    let lower = lower_initial(s);
    (lower != s).then_some(lower)
}

/// Case-folds for matching, so that `gy Gy GY` all compare equal (`WS-CASE-005`).
#[must_use]
pub fn fold(s: &str) -> String {
    s.to_lowercase()
}

/// Whether the word contains a letter whose case pattern is not standard spelling.
///
/// Flags the `gY` forms anywhere, an all-caps multi-character letter in an otherwise lowercase
/// word (`SZekszárd`, `GYőr`), and an initial-capital one inside an all-caps word (`MAGyAR`).
///
/// A member-initial capital inside a solid compound is standard (`WS-CASE-007`) but needs
/// compound structure, which this crate does not have; such a word is flagged here and the
/// caller with morphology must override.
///
/// Rules: WS-CASE-007.
#[must_use]
pub fn is_mixed_case(word: &str) -> bool {
    mixed_case(word).is_some()
}

/// The case pattern the single-character letters of `word` agree on, and the offending letter.
///
/// `Some(all_caps)` when the word has a non-standard letter case; `all_caps` is the pattern the
/// repair should follow.
fn mixed_case(word: &str) -> Option<bool> {
    let spans: Vec<_> = letters(word).collect();
    let singles: Vec<_> = spans
        .iter()
        .filter(|s| s.text.chars().count() == 1)
        .collect();
    if singles.is_empty() {
        // Nothing unambiguous to read the pattern off: a bare `GY` is a letter name, not a slip.
        return None;
    }
    // The single-character letters are unambiguous, so they carry the word's intended pattern.
    let all_caps = singles
        .iter()
        .all(|s| letter_case(s.text) == LetterCase::Upper);
    let offending = spans.iter().enumerate().any(|(i, s)| {
        let multi = s.text.chars().count() > 1;
        match letter_case(s.text) {
            LetterCase::NonStandard => true,
            LetterCase::Upper if multi => !all_caps,
            LetterCase::Initial => i > 0 || all_caps,
            _ => false,
        }
    });
    offending.then_some(all_caps)
}

/// Repairs the letter case of a word whose multi-character letters are written non-standardly.
///
/// `GYőr` -> `Győr`, `SZekszárd` -> `Szekszárd`, `MAGyAR` -> `MAGYAR`, `cSók` -> `csók`. The
/// pattern is taken from the single-character letters, which cannot be ambiguous; when they are
/// lowercase the word keeps an initial capital if it had one. [`repair_alternative`] gives the
/// other reading of an ambiguous case.
///
/// `None` when the word is already standard.
///
/// Rules: WS-CASE-007.
#[must_use]
pub fn repair(word: &str) -> Option<String> {
    let all_caps = mixed_case(word)?;
    if all_caps {
        return Some(upper(word));
    }
    let had_initial = word.chars().next().is_some_and(char::is_uppercase);
    Some(if had_initial {
        capitalise(word)
    } else {
        lower(word)
    })
}

/// The all-caps reading of a word that [`repair`] read as an initial capital.
///
/// `GYőr` is a slip for either `Győr` or `GYŐR` and the word alone does not say which
/// (`WS-CASE-007`: "repair to the surrounding pattern").
#[must_use]
pub fn repair_alternative(word: &str) -> Option<String> {
    let all_caps = mixed_case(word)?;
    (!all_caps && word.chars().next().is_some_and(char::is_uppercase)).then(|| upper(word))
}

#[cfg(test)]
mod tests {
    use super::{
        LetterCase, capitalise, is_mixed_case, letter_case, lower, repair, repair_alternative,
        upper,
    };

    #[test]
    fn ws_case_004_mapping_is_per_character_and_locale_independent() {
        assert_eq!(upper("őrület"), "ŐRÜLET");
        assert_eq!(lower("GYŐR"), "győr");
        // Never the Turkish mapping.
        assert_eq!(upper("igen"), "IGEN");
        assert_eq!(lower("IGEN"), "igen");
    }

    #[test]
    fn ws_case_001_an_initial_capital_touches_only_the_first_character() {
        assert_eq!(capitalise("gyöngy"), "Gyöngy");
        assert_eq!(capitalise("szeged"), "Szeged");
        assert_eq!(capitalise("GYŐR"), "Győr");
    }

    #[test]
    fn ws_case_003_letter_case_patterns() {
        assert_eq!(letter_case("gy"), LetterCase::Lower);
        assert_eq!(letter_case("Gy"), LetterCase::Initial);
        assert_eq!(letter_case("GY"), LetterCase::Upper);
        assert_eq!(letter_case("gY"), LetterCase::NonStandard);
        assert_eq!(letter_case("a"), LetterCase::Lower);
        assert_eq!(letter_case("A"), LetterCase::Upper);
    }

    #[test]
    fn ws_case_007_mixed_case_is_detected_and_repaired() {
        assert!(is_mixed_case("GYőr"));
        assert_eq!(repair("GYőr").as_deref(), Some("Győr"));
        assert_eq!(repair_alternative("GYőr").as_deref(), Some("GYŐR"));
        assert_eq!(repair("SZekszárd").as_deref(), Some("Szekszárd"));
        assert_eq!(repair("MAGyAR").as_deref(), Some("MAGYAR"));
        assert_eq!(repair("cSók").as_deref(), Some("csók"));
    }

    #[test]
    fn ws_case_007_standard_spellings_are_left_alone() {
        for w in [
            "Győr",
            "GYŐR",
            "győr",
            "Szekszárd",
            "MAGYAR",
            "csók",
            "Gyöngy",
        ] {
            assert!(!is_mixed_case(w), "{w} flagged");
            assert_eq!(repair(w), None, "{w} repaired");
        }
    }
}
