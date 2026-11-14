//! The ellipsis.
//!
//! Rules: `PUN-ELL-001`, `PUN-ELL-007`

use crate::text::{Ch, at, is, is_space, span};
use crate::{Diagnostic, Protected, Severity};

/// Reports the ellipsis errors of `chars`.
pub(crate) fn check(chars: &[Ch], protected: &Protected, out: &mut Vec<Diagnostic>) {
    let mut i = 0;
    while i < chars.len() {
        if chars[i].c != '.' {
            i += 1;
            continue;
        }
        let Some((end, periods)) = run(chars, i) else {
            i += 1;
            continue;
        };
        // PUN-ELL-002: an ellipsis after an interrupted word attaches to that word. It keeps a
        // space before it only when it marks an omitted sentence beginning (PUN-ELL-003), which is
        // what a letter directly after the run signals: `Vége. ...s akkor`.
        let start = if at(chars, end).is_some_and(char::is_alphabetic) {
            i
        } else {
            let back = crate::text::back_spaces(chars, i);
            if back > 0 && chars[back - 1].c.is_alphanumeric() {
                back
            } else {
                i
            }
        };
        let at_span = span(chars, start, end);
        if !protected.covers(&at_span) {
            out.push(diagnostic(at_span, periods));
        }
        i = end;
    }
}

/// Whether the period at `i` belongs to a period run, so the ellipsis rules own it.
///
/// The spacing rules must not claim such a period: `Hát . . .` would otherwise be reported as a
/// space before an attaching mark, and the two findings would overlap.
pub(crate) fn in_run(chars: &[Ch], i: usize) -> bool {
    if chars.get(i).map(|c| c.c) != Some('.') {
        return false;
    }
    let back = |j: usize| at(chars, j) == Some('.');
    run(chars, i).is_some()
        || (i > 0 && back(i - 1))
        || (i > 1 && is(chars, i - 1, is_space) && back(i - 2))
}

/// The extent of a period run starting at `i`, allowing single spaces between the periods.
///
/// Returns the end position and the number of periods, or `None` for a lone period.
///
/// Spaced periods are `PUN-ELL-007`: `. . .` is one ellipsis written wrongly, not three sentences.
/// A run is only read as spaced when every gap is a single space, so an abbreviation period
/// followed by a new sentence starting with a period cannot be swallowed.
fn run(chars: &[Ch], i: usize) -> Option<(usize, usize)> {
    let mut end = i + 1;
    let mut periods = 1;
    loop {
        if at(chars, end) == Some('.') {
            end += 1;
            periods += 1;
        } else if is(chars, end, is_space) && at(chars, end + 1) == Some('.') {
            end += 2;
            periods += 1;
        } else {
            break;
        }
    }
    (periods > 1).then_some((end, periods))
}

/// The finding for a run of `periods` periods.
fn diagnostic(span: core::ops::Range<usize>, periods: usize) -> Diagnostic {
    // Two periods are neither a sentence end nor an ellipsis, and the rulebook says to ask rather
    // than guess between `.` and `…`.
    if periods == 2 {
        return Diagnostic {
            span,
            rule: "PUN-ELL-001",
            message: "two periods are neither a sentence end nor an ellipsis",
            severity: Severity::Tendency,
            fix: None,
        };
    }
    Diagnostic {
        span,
        rule: "PUN-ELL-001",
        message: "an ellipsis is the single character U+2026",
        severity: Severity::Error,
        fix: Some("…".into()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Severity, apply, check};

    fn fixed(text: &str) -> String {
        apply(text, &check(text))
    }

    #[test]
    fn three_periods_become_the_ellipsis_character() {
        // PUN-ELL-001.
        assert_eq!(fixed("Hát... nem tudom."), "Hát… nem tudom.");
        assert_eq!(fixed("És aztán...?"), "És aztán…?");
    }

    #[test]
    fn four_or_more_periods_become_one_ellipsis() {
        assert_eq!(fixed("Hát...."), "Hát…");
        assert_eq!(fixed("Hát......"), "Hát…");
    }

    #[test]
    fn spaced_periods_become_the_ellipsis_character() {
        // PUN-ELL-007.
        assert_eq!(fixed("Hát . . . nem."), "Hát… nem.");
    }

    #[test]
    fn two_periods_are_flagged_without_a_fix() {
        // The rulebook says to ask: `.` and `…` are both plausible and the choice changes meaning.
        let d = check("Hát.. nem.");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].fix, None);
        assert_eq!(d[0].severity, Severity::Tendency);
        assert_eq!(fixed("Hát.. nem."), "Hát.. nem.");
    }

    #[test]
    fn ordinary_periods_are_left_alone() {
        for kept in [
            "Egy mondat. Másik mondat.",
            "Az 1.2.3 pont.",
            "A v1.3 verzió.",
            "Nézd a www.mta.hu oldalt.",
            "Kb. 20 fő.",
        ] {
            assert_eq!(fixed(kept), kept, "{kept}");
        }
    }

    #[test]
    fn an_existing_ellipsis_is_left_alone() {
        assert_eq!(fixed("Hát… nem tudom."), "Hát… nem tudom.");
    }
}
