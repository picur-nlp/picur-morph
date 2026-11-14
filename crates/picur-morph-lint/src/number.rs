//! Number, unit and symbol formatting.
//!
//! Rules: `PUN-NUMF-002`, `PUN-NUMF-005`, `PUN-NUMF-006`, `PUN-NUMF-015`, `PUN-NUMF-019`

use picur_morph_lexicon::{Role, lists, words};

use crate::text::{Ch, at, back_spaces, insert_before, is, skip_spaces, span};
use crate::{Diagnostic, Protected, Severity};

/// Reports the number formatting errors of `chars`.
pub(crate) fn check(chars: &[Ch], protected: &Protected, out: &mut Vec<Diagnostic>) {
    for i in 0..chars.len() {
        percent(chars, protected, i, out);
        degree(chars, protected, i, out);
        section(chars, protected, i, out);
        times(chars, protected, i, out);
        unit_space(chars, protected, i, out);
        currency_order(chars, protected, i, out);
        exponent(chars, protected, i, out);
    }
    grouping(chars, protected, out);
}

/// Pushes a diagnostic unless it falls in a protected region.
fn push(
    out: &mut Vec<Diagnostic>,
    protected: &Protected,
    span: core::ops::Range<usize>,
    rule: &'static str,
    message: &'static str,
    severity: Severity,
    fix: &str,
) {
    if protected.covers(&span) {
        return;
    }
    out.push(Diagnostic {
        span,
        rule,
        message,
        severity,
        fix: Some(fix.into()),
    });
}

/// `48 %` -> `48%`: the percent sign attaches to its number.
///
/// Rules: `PUN-NUMF-005`
fn percent(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if !matches!(chars[i].c, '%' | '‰') {
        return;
    }
    let before = back_spaces(chars, i);
    if before == i || before == 0 || !is(chars, before - 1, |c| c.is_ascii_digit()) {
        return;
    }
    push(
        out,
        protected,
        span(chars, before, i),
        "PUN-NUMF-005",
        "the percent sign attaches to the number",
        Severity::Error,
        "",
    );
}

/// The degree sign: attached for angles, spaced before a temperature unit.
///
/// Rules: `PUN-NUMF-006`
fn degree(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    // The masculine ordinal indicator is a look-alike that the online AkH12 text itself contains.
    if chars[i].c == 'º' {
        push(
            out,
            protected,
            span(chars, i, i + 1),
            "PUN-NUMF-006",
            "U+00BA is a look-alike of the degree sign U+00B0",
            Severity::Error,
            "°",
        );
        return;
    }
    if chars[i].c != '°' {
        return;
    }
    if is(chars, i + 1, |c| c == 'C' || c == 'F') && i > 0 && chars[i - 1].c.is_ascii_digit() {
        push(
            out,
            protected,
            insert_before(chars, i),
            "PUN-NUMF-006",
            "a temperature unit is separated from the number by a space",
            Severity::Error,
            " ",
        );
        return;
    }
    if i > 0 && matches!(chars[i - 1].c, 'C' | 'F') {
        let before = back_spaces(chars, i - 1);
        if before > 0 && is(chars, before - 1, |c| c.is_ascii_digit()) {
            let unit = chars[i - 1].c;
            push(
                out,
                protected,
                span(chars, before, i + 1),
                "PUN-NUMF-006",
                "the degree sign precedes the temperature unit",
                Severity::Error,
                &format!(" °{unit}"),
            );
        }
    }
}

/// `12.§` -> `12. §`: the section sign is spaced from its ordinal.
///
/// Rules: `PUN-NUMF-015`
fn section(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if chars[i].c != '§' || i == 0 || chars[i - 1].c != '.' {
        return;
    }
    if !(i >= 2 && chars[i - 2].c.is_ascii_digit()) {
        return;
    }
    push(
        out,
        protected,
        insert_before(chars, i),
        "PUN-NUMF-015",
        "the section sign follows the ordinal number with a space",
        Severity::Error,
        " ",
    );
}

/// `20x30` -> `20 × 30`: the multiplication sign in dimensions.
///
/// A tendency: the letter `x` is the accepted plain-text substitute, so this is an offer, not a
/// correction. Rules: `PUN-NUMF-019`
fn times(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if !matches!(chars[i].c, 'x' | '×') {
        return;
    }
    let before = back_spaces(chars, i);
    let after = skip_spaces(chars, i + 1);
    if before == 0
        || !is(chars, before - 1, |c| c.is_ascii_digit())
        || !is(chars, after, |c| c.is_ascii_digit())
    {
        return;
    }
    if chars[i].c == '×' && before + 1 == i && after == i + 2 {
        return;
    }
    push(
        out,
        protected,
        span(chars, before, after),
        "PUN-NUMF-019",
        "dimensions take a spaced multiplication sign",
        Severity::Tendency,
        " × ",
    );
}

/// The run of letters, with `/` between letters, starting at `i`.
fn symbol_run(chars: &[Ch], i: usize) -> (usize, String) {
    let mut j = i;
    let mut text = String::new();
    while let Some(c) = at(chars, j) {
        let joins = c.is_alphabetic()
            || c == 'Ω'
            || (c == '/' && is(chars, j + 1, char::is_alphabetic) && !text.is_empty());
        if !joins {
            break;
        }
        text.push(c);
        j += 1;
    }
    (j, text)
}

/// `5kg` -> `5 kg`: a unit symbol is separated from its number.
///
/// Rules: `PUN-NUMF-007`
fn unit_space(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if !chars[i].c.is_ascii_digit() || !is(chars, i + 1, char::is_alphabetic) {
        return;
    }
    let (end, symbol) = symbol_run(chars, i + 1);
    // The run must be the whole symbol: `5kg` is a unit, `5km-es` is one too, `5kilo` is not.
    let followed_by_letter = is(chars, end, char::is_alphanumeric);
    if followed_by_letter || !lists(Role::UnitSymbol, &symbol) {
        return;
    }
    push(
        out,
        protected,
        insert_before(chars, i + 1),
        "PUN-NUMF-007",
        "a unit symbol is separated from the number by a space",
        Severity::Error,
        " ",
    );
}

/// `Ft2890` -> `2890 Ft`: the amount comes first, then the currency.
///
/// Rules: `PUN-NUMF-008`
fn currency_order(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if !chars[i].c.is_alphabetic() || (i > 0 && chars[i - 1].c.is_alphanumeric()) {
        return;
    }
    let (end, symbol) = symbol_run(chars, i);
    if !lists(Role::CurrencyUnit, &symbol) {
        return;
    }
    let digits_from = skip_spaces(chars, end);
    let mut digits_to = digits_from;
    while is(chars, digits_to, |c| {
        c.is_ascii_digit() || c == ' ' && is(chars, digits_to + 1, |d| d.is_ascii_digit())
    }) {
        digits_to += 1;
    }
    if digits_to == digits_from || is(chars, digits_to, char::is_alphanumeric) {
        return;
    }
    let amount: String = chars[digits_from..digits_to].iter().map(|c| c.c).collect();
    push(
        out,
        protected,
        span(chars, i, digits_to),
        "PUN-NUMF-008",
        "the amount precedes the currency unit",
        Severity::Error,
        &format!("{amount} {symbol}"),
    );
}

/// `10 m2` -> `10 m²`: an area or volume unit takes a superscript exponent.
///
/// Rules: `PUN-ABBR-011`
fn exponent(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if !matches!(chars[i].c, '2' | '3') || !is(chars, i.wrapping_sub(1), char::is_alphabetic) {
        return;
    }
    let mut start = i;
    while start > 0 && chars[start - 1].c.is_alphabetic() {
        start -= 1;
    }
    let symbol: String = chars[start..i].iter().map(|c| c.c).collect();
    // The symbol stands on its own, after a number or not: `10 m2`, `m2`; a word's tail is not
    // one (`km2` inside `tkm2`).
    let alone = start == 0 || !chars[start - 1].c.is_alphanumeric();
    if !alone
        || !words(Role::UnitSymbol)
            .iter()
            .any(|u| u.as_ref() == symbol && u.ends_with('m'))
        || is(chars, i + 1, char::is_alphanumeric)
    {
        return;
    }
    let sup = if chars[i].c == '2' { "²" } else { "³" };
    push(
        out,
        protected,
        span(chars, i, i + 1),
        "PUN-ABBR-011",
        "an area or volume unit takes a superscript exponent",
        Severity::Tendency,
        sup,
    );
}

/// Thousands grouping for integers of five or more digits.
///
/// A tendency, because the rule's exclusions (identifier, account number, serial number) are not
/// decidable from the character stream. What is decidable is excluded here: a digit run touching a
/// letter or any of `.,:/-+#` is part of a larger construct, and a phone cue suppresses the whole
/// line. Rules: `PUN-NUMF-002`
fn grouping(chars: &[Ch], protected: &Protected, out: &mut Vec<Diagnostic>) {
    let mut i = 0;
    while i < chars.len() {
        if !chars[i].c.is_ascii_digit() {
            i += 1;
            continue;
        }
        let start = i;
        while is(chars, i, |c| c.is_ascii_digit()) {
            i += 1;
        }
        let len = i - start;
        if len < 5 {
            continue;
        }
        let adjacent =
            |c: char| c.is_alphanumeric() || matches!(c, '.' | ',' | ':' | '/' | '-' | '+' | '#');
        if (start > 0 && at(chars, start - 1).is_some_and(adjacent))
            || at(chars, i).is_some_and(adjacent)
        {
            continue;
        }
        let digits: String = chars[start..i].iter().map(|c| c.c).collect();
        let mut grouped = String::new();
        for (n, c) in digits.chars().enumerate() {
            if n > 0 && (len - n) % 3 == 0 {
                grouped.push('\u{a0}');
            }
            grouped.push(c);
        }
        push(
            out,
            protected,
            span(chars, start, i),
            "PUN-NUMF-002",
            "numbers of five or more digits are grouped in threes",
            Severity::Tendency,
            &grouped,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{apply, check, normalise};

    fn fixed(text: &str) -> String {
        apply(text, &check(text))
    }

    #[test]
    fn the_percent_sign_attaches() {
        // PUN-NUMF-005.
        assert_eq!(fixed("Ez 48 % volt."), "Ez 48% volt.");
        assert_eq!(fixed("Ez 12,5 %-os."), "Ez 12,5%-os.");
        assert_eq!(fixed("Ez 3 ‰."), "Ez 3‰.");
        assert_eq!(fixed("Ez 48%-os."), "Ez 48%-os.");
    }

    #[test]
    fn degrees_take_their_places() {
        // PUN-NUMF-006: attached for angles, spaced before a temperature unit.
        assert_eq!(fixed("A víz 100°C-on forr."), "A víz 100 °C-on forr.");
        assert_eq!(fixed("A víz 100 C°-on forr."), "A víz 100 °C-on forr.");
        assert_eq!(fixed("A szög 45°-os."), "A szög 45°-os.");
        assert_eq!(fixed("A víz 100 °C-on forr."), "A víz 100 °C-on forr.");
    }

    #[test]
    fn the_ordinal_indicator_is_normalised() {
        // U+00BA appears in the online AkH12 text itself.
        assert_eq!(fixed("A szög 45º."), "A szög 45°.");
    }

    #[test]
    fn the_section_sign_is_spaced() {
        // PUN-NUMF-015.
        assert_eq!(fixed("A 15.§-ban."), "A 15. §-ban.");
        assert_eq!(fixed("A 15. §-ban."), "A 15. §-ban.");
    }

    #[test]
    fn dimensions_are_offered_the_multiplication_sign() {
        // PUN-NUMF-019 is a tendency, so `normalise` declines it.
        assert_eq!(fixed("A lap 20x30 cm."), "A lap 20 × 30 cm.");
        assert_eq!(normalise("A lap 20x30 cm."), "A lap 20x30 cm.");
        assert_eq!(fixed("A lap 20 × 30 cm."), "A lap 20 × 30 cm.");
    }

    #[test]
    fn long_numbers_are_offered_grouping() {
        // PUN-NUMF-002, a tendency: the exclusions are not all decidable here.
        assert_eq!(fixed("Ez 23816 forint."), "Ez 23\u{a0}816 forint.");
        assert_eq!(fixed("Ez 5602164 volt."), "Ez 5\u{a0}602\u{a0}164 volt.");
        assert_eq!(normalise("Ez 23816 forint."), "Ez 23816 forint.");
    }

    #[test]
    fn short_numbers_and_identifiers_keep_their_digits() {
        for kept in [
            "A 2500 forint.",
            "Az 1978. évben.",
            "Irányítószám: 1117.",
            "A +36 1 234 5678 szám.",
            "A 06-30/123-4567 szám.",
            "Az A-12345 azonosító.",
            "A 20 611,413 érték.",
        ] {
            assert_eq!(fixed(kept), kept, "{kept}");
        }
    }
}
