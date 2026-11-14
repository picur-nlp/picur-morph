//! Dates and times.
//!
//! Most of the family needs syntax: whether the year takes a period depends on what it is
//! grammatically related to (`PUN-DATEF-005`), and whether a day number needs its possessive
//! depends on a following postposition (`PUN-DATEF-009`). What the character stream settles is the
//! spacing of a written-out date and the link inside a range.
//!
//! Rules: `PUN-DATEF-001`, `PUN-DATEF-003`

use picur_morph_lexicon::{Role, lists, words};

use crate::text::{Ch, at, insert_before};
use crate::{Diagnostic, Protected, Severity};

/// Whether `word` names a month, in any of its written forms.
///
/// An inflected month (`januárban`) still cues a date, so a prefix match on the full names is what
/// the caller wants; the abbreviations must match exactly, because `jan.` is already complete.
pub(crate) fn is_month(word: &str) -> bool {
    lists(Role::MonthAbbreviation, word)
        || words(Role::Month)
            .iter()
            .any(|m| word.starts_with(m.as_ref()))
}

/// Reports the date formatting errors of `chars`.
pub(crate) fn check(chars: &[Ch], protected: &Protected, out: &mut Vec<Diagnostic>) {
    for i in 0..chars.len() {
        year_period(chars, protected, i, out);
    }
}

/// `2024.január` -> `2024. január`: a space follows the period of each date element.
///
/// Anchored on a four-digit year, which is what separates a date from a clock time (`10.35`), a
/// version number (`1.2.3`) and an ordinal (`3.osztály`, which `PUN-NUMF-009` owns).
///
/// Rules: `PUN-DATEF-001`, `PUN-DATEF-003`
fn year_period(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if chars[i].c != '.' || i < 4 {
        return;
    }
    if !chars[i - 4..i].iter().all(|c| c.c.is_ascii_digit()) {
        return;
    }
    // Exactly four digits: a longer run is not a year.
    if i >= 5 && chars[i - 5].c.is_ascii_digit() {
        return;
    }
    // Only a written-out month is certain. `2024.01.05` would need every element respaced, and
    // inserting one space would leave `2024. 01.05`, which is worse than either form.
    if !at(chars, i + 1).is_some_and(char::is_alphabetic) {
        return;
    }
    push(
        out,
        protected,
        insert_before(chars, i + 1),
        "PUN-DATEF-001",
        "a space follows the period of each date element",
        " ",
    );
}

/// Pushes a diagnostic unless it falls in a protected region.
fn push(
    out: &mut Vec<Diagnostic>,
    protected: &Protected,
    span: core::ops::Range<usize>,
    rule: &'static str,
    message: &'static str,
    fix: &str,
) {
    if protected.covers(&span) {
        return;
    }
    out.push(Diagnostic {
        span,
        rule,
        message,
        severity: Severity::Error,
        fix: Some(fix.into()),
    });
}

#[cfg(test)]
mod tests {
    use crate::normalise;

    #[test]
    fn a_date_element_period_is_followed_by_a_space() {
        // PUN-DATEF-001.
        assert_eq!(normalise("2024.január 5."), "2024. január 5.");
        assert_eq!(
            normalise("Született 1990.május 3."),
            "Született 1990. május 3."
        );
    }

    #[test]
    fn a_correct_date_is_untouched() {
        for kept in [
            "2024. január 5.",
            "2014. február",
            "február 28.",
            "febr. 28-án",
            "1990. május 3.",
            "az 1990-es évek",
            // A fully numeric date needs every element respaced, so it is left alone.
            "2024.01.05",
        ] {
            assert_eq!(normalise(kept), kept, "{kept}");
        }
    }

    #[test]
    fn times_and_version_numbers_are_not_dates() {
        // A four-digit anchor is what keeps these out.
        for kept in ["10.35-kor", "a v1.3 verzió", "az 1.2.3 pont", "3.osztály"] {
            assert_eq!(normalise(kept), kept, "{kept}");
        }
    }

    #[test]
    fn month_names_are_recognised_inflected() {
        use super::is_month;
        assert!(is_month("január"));
        assert!(is_month("januárban"));
        assert!(is_month("jan."));
        assert!(!is_month("jan"));
        assert!(!is_month("janus"));
    }
}
