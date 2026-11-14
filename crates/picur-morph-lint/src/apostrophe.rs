//! The apostrophe, and the primes it is confused with.
//!
//! Rules: `PUN-APOS-001`, `PUN-APOS-003`, `PUN-APOS-005`, `PUN-APOS-006`

use crate::quote::single_quotes_are_marks;
use crate::text::{Ch, at, is, span};
use crate::{Diagnostic, Protected, Severity};

/// Reports the apostrophe errors of `chars`.
pub(crate) fn check(chars: &[Ch], protected: &Protected, out: &mut Vec<Diagnostic>) {
    // `hál'` is an apostrophe, but the closing half of `'szó'` is a quotation mark. Only the text
    // as a whole tells them apart: a straight quote standing where a quotation could open means
    // the text uses the character as a mark, and then none of its occurrences is an apostrophe.
    let quoting = single_quotes_are_marks(chars);
    for i in 0..chars.len() {
        prime(chars, protected, i, out);
        if !quoting {
            apostrophe(chars, protected, i, out);
        }
        suffix(chars, protected, i, out);
    }
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

/// `'` and its look-alikes become `’` where an apostrophe belongs.
///
/// Rules: `PUN-APOS-001`, `PUN-APOS-003`
fn apostrophe(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if !matches!(chars[i].c, '\'' | '´' | '`') {
        return;
    }
    let after_letter = i > 0 && chars[i - 1].c.is_alphabetic();
    // PUN-APOS-003: `’95`. Exactly two digits, so a prime or a foot mark cannot match.
    let shortened_year = is(chars, i + 1, |c| c.is_ascii_digit())
        && is(chars, i + 2, |c| c.is_ascii_digit())
        && !is(chars, i + 3, |c| c.is_ascii_digit())
        && (i == 0 || !chars[i - 1].c.is_alphanumeric());
    if !(after_letter || shortened_year) {
        return;
    }
    push(
        out,
        protected,
        span(chars, i, i + 1),
        "PUN-APOS-001",
        "the apostrophe is U+2019",
        "’",
    );
}

/// Whether the mark at `i` is a prime rather than a quotation mark or an apostrophe.
///
/// A degree sign or an arc minute shortly before it is the anchor. Without one, a straight quote
/// after a digit is just as likely to open a quotation that follows a number.
pub(crate) fn is_prime(chars: &[Ch], i: usize) -> bool {
    if !matches!(
        chars.get(i).map(|c| c.c),
        Some('\'' | '\u{b4}' | '"' | '\u{201d}')
    ) {
        return false;
    }
    if !(i > 0 && chars[i - 1].c.is_ascii_digit()) {
        return false;
    }
    chars[i.saturating_sub(12)..i]
        .iter()
        .any(|c| matches!(c.c, '\u{b0}' | '\u{2032}'))
}

/// Minutes and seconds of arc take the prime and double prime, not quotation characters.
///
/// Restricted to a coordinate, which a degree sign earlier in the run establishes. Without that
/// anchor a straight quote after a digit is just as likely to be an inch mark in a quoted width or
/// a quotation that happens to follow a number.
///
/// Rules: `PUN-APOS-006`
fn prime(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if !is_prime(chars, i) {
        return;
    }
    let want = match chars[i].c {
        '\'' | '´' => '′',
        _ => '″',
    };
    push(
        out,
        protected,
        span(chars, i, i + 1),
        "PUN-APOS-006",
        "arc minutes and seconds take the prime and double prime",
        &want.to_string(),
    );
}

/// `2020'ban` and `CD'k`: a suffix never attaches with an apostrophe.
///
/// Only the two cases the character stream settles. After a digit or an all-capital acronym the
/// hyphen is required, and nothing else can be meant. After an ordinary word the apostrophe may
/// belong to a foreign name (`O’Neill`, `McDonald’s`), which `PUN-APOS-004` keeps, so those are
/// left alone.
///
/// Rules: `PUN-APOS-005`
fn suffix(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if !matches!(chars[i].c, '\'' | '’' | '´') || i == 0 {
        return;
    }
    if !is(chars, i + 1, char::is_lowercase) {
        return;
    }
    let prev = chars[i - 1].c;
    let base_ok =
        prev.is_ascii_digit() || (prev.is_uppercase() && i >= 2 && chars[i - 2].c.is_uppercase());
    if !base_ok {
        return;
    }
    // A run of lowercase letters is a suffix; anything else is a word and this is not the rule.
    let mut j = i + 1;
    while is(chars, j, char::is_lowercase) {
        j += 1;
    }
    if at(chars, j).is_some_and(char::is_alphanumeric) {
        return;
    }
    push(
        out,
        protected,
        span(chars, i, i + 1),
        "PUN-APOS-005",
        "a suffix attaches with a hyphen, never an apostrophe",
        "-",
    );
}

#[cfg(test)]
mod tests {
    use crate::normalise;

    #[test]
    fn the_apostrophe_glyph_is_normalised() {
        // PUN-APOS-001.
        assert_eq!(normalise("hál' istennek"), "hál’ istennek");
        assert_eq!(normalise("hál´ istennek"), "hál’ istennek");
        assert_eq!(normalise("O'Neill"), "O’Neill");
        assert_eq!(normalise("d'Artagnan"), "d’Artagnan");
    }

    #[test]
    fn shortened_years_take_the_apostrophe() {
        // PUN-APOS-003.
        assert_eq!(normalise("a '90-es évek"), "a ’90-es évek");
        assert_eq!(normalise("az '56-os forradalom"), "az ’56-os forradalom");
    }

    #[test]
    fn a_suffix_after_a_number_or_acronym_takes_a_hyphen() {
        // PUN-APOS-005.
        assert_eq!(normalise("2020'ban"), "2020-ban");
        assert_eq!(normalise("CD'k"), "CD-k");
        assert_eq!(normalise("MTA’t"), "MTA-t");
    }

    #[test]
    fn a_foreign_names_own_apostrophe_is_kept() {
        // PUN-APOS-004: the name's spelling wins over the suffix rule.
        for kept in ["O’Neill", "d’Artagnan", "McDonald’s"] {
            assert_eq!(normalise(kept), kept, "{kept}");
        }
    }

    #[test]
    fn coordinates_take_primes() {
        // PUN-APOS-006.
        assert_eq!(normalise("47° 13' 5\""), "47° 13′ 5″");
        assert_eq!(normalise("47° 13′ 5″"), "47° 13′ 5″");
    }

    #[test]
    fn a_text_that_quotes_with_single_marks_keeps_them() {
        // Converting only the closing half would leave `'szó’`, which is worse than either reading.
        assert_eq!(normalise("A 'szó' jelentése."), "A 'szó' jelentése.");
    }
}
