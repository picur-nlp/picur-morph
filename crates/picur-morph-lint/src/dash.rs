//! Hyphens, en dashes and the em dash.
//!
//! Rules: `PUN-DASH-012`, `PUN-DASH-014`, `PUN-DASH-015`, `PUN-DASH-019`

use picur_morph_lexicon::{Role, lists};

use crate::date::is_month;
use crate::text::{Ch, at, back_spaces, is, is_space, skip_spaces, span};
use crate::{Diagnostic, Protected, Severity};

/// Reports the dash errors of `chars`.
pub(crate) fn check(chars: &[Ch], protected: &Protected, out: &mut Vec<Diagnostic>) {
    for i in 0..chars.len() {
        em_dash(chars, protected, i, out);
        hyphen_as_dash(chars, protected, i, out);
        gondolatjel(chars, protected, i, out);
    }
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

/// The word ending just before position `i`, lowercased.
fn word_before(chars: &[Ch], i: usize) -> String {
    let end = back_spaces(chars, i);
    let mut start = end;
    while start > 0 && (chars[start - 1].c.is_alphanumeric() || chars[start - 1].c == '.') {
        start -= 1;
    }
    chars[start..end]
        .iter()
        .flat_map(|c| c.c.to_lowercase())
        .collect()
}

/// The em dash is not part of the `AkH12` system.
///
/// Rules: `PUN-DASH-015`
fn em_dash(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if chars[i].c != '—' {
        return;
    }
    let numeric = i > 0
        && chars[i - 1].c.is_ascii_digit()
        && at(chars, i + 1).is_some_and(|c| c.is_ascii_digit());
    let (from, to, fix) = if numeric {
        (i, i + 1, "–")
    } else {
        (back_spaces(chars, i), skip_spaces(chars, i + 1), " – ")
    };
    push(
        out,
        protected,
        span(chars, from, to),
        "PUN-DASH-015",
        "Hungarian uses the en dash where other traditions use the em dash",
        Severity::Tendency,
        fix,
    );
}

/// A hyphen-minus standing in a dash function.
///
/// The cases are `PUN-DASH-019`'s ordered list. Each one needs a positive cue, because the default
/// reading of a hyphen between numbers is approximation (`5-6 darab`), which is correct as written.
///
/// Rules: `PUN-DASH-019`
fn hyphen_as_dash(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if chars[i].c != '-' {
        return;
    }
    let line_start = (i == 0 || chars[i - 1].c == '\n') && is(chars, i + 1, is_space);
    if line_start {
        push(
            out,
            protected,
            span(chars, i, i + 1),
            "PUN-DASH-019",
            "a dialogue or list dash is the en dash",
            Severity::Tendency,
            "–",
        );
        return;
    }

    let before = back_spaces(chars, i);
    let after = skip_spaces(chars, i + 1);
    let spaced = before < i && after > i + 1;
    let prev = if before == 0 {
        None
    } else {
        at(chars, before - 1)
    };
    let next = at(chars, after);

    // A hyphen spaced on both sides is a gondolatjel. Between digits it is more likely subtraction,
    // which `PUN-DASH-016` leaves alone.
    if spaced {
        let arithmetic =
            prev.is_some_and(|c| c.is_ascii_digit()) && next.is_some_and(|c| c.is_ascii_digit());
        if !arithmetic && prev.is_some() && next.is_some() {
            push(
                out,
                protected,
                span(chars, before, after),
                "PUN-DASH-019",
                "a spaced dash is the en dash",
                Severity::Error,
                " – ",
            );
        }
        return;
    }

    // An unspaced hyphen between numbers is a range only on evidence: a year pair, or a page or
    // house number announced by its cue word.
    if !(prev.is_some_and(|c| c.is_ascii_digit()) && next.is_some_and(|c| c.is_ascii_digit())) {
        return;
    }
    let left = digits_before(chars, i);
    let right = digits_after(chars, i);
    // An ISO date is zero-padded, which a shortened year never is: `1848-49` against `2024-01-05`.
    // A third hyphen-joined group is the day, but a suffix after one (`1848-49-es`) is not.
    let tail = i + 1 + right.len();
    let iso = right.starts_with('0')
        || (at(chars, tail) == Some('-') && is(chars, tail + 1, |c| c.is_ascii_digit()));
    let year_pair = !iso
        && left.len() == 4
        && (left.starts_with('1') || left.starts_with("20"))
        && (2..=4).contains(&right.len());
    // A page cue follows its number (`45-47. oldalon`) and a street cue precedes it
    // (`Petőfi utca 12-14.`), so both sides count.
    // PUN-DATEF-007: days within one month are linked by an en dash, and the month says so.
    let dated = is_month(&word_before(chars, i - left.len()));
    let cued = !iso
        && (dated
            || time_range(chars, i)
            || lists(Role::RangeCue, &word_before(chars, i - left.len()))
            || lists(Role::RangeCue, &word_after(chars, tail)));
    if !(year_pair || cued) {
        return;
    }
    push(
        out,
        protected,
        span(chars, i, i + 1),
        "PUN-DASH-019",
        "a from-to range takes the en dash",
        Severity::Error,
        "–",
    );
}

/// Whether the hyphen at `i` links two clock times (`10.00-12.00`).
///
/// Rules: `PUN-DATEF-013`
fn time_range(chars: &[Ch], i: usize) -> bool {
    let clock_before = i >= 4
        && chars[i - 2..i].iter().all(|c| c.c.is_ascii_digit())
        && matches!(chars[i - 3].c, '.' | ':')
        && chars[i - 4].c.is_ascii_digit();
    let hour = usize::from(is(chars, i + 2, |c| c.is_ascii_digit()));
    let clock_after = is(chars, i + 1, |c| c.is_ascii_digit())
        && at(chars, i + 2 + hour).is_some_and(|c| matches!(c, '.' | ':'))
        && is(chars, i + 3 + hour, |c| c.is_ascii_digit())
        && is(chars, i + 4 + hour, |c| c.is_ascii_digit());
    clock_before && clock_after
}

/// The word starting just after position `i`, lowercased, skipping an ordinal period.
fn word_after(chars: &[Ch], i: usize) -> String {
    let mut start = i;
    if at(chars, start) == Some('.') {
        start += 1;
    }
    let start = skip_spaces(chars, start);
    let mut end = start;
    while at(chars, end).is_some_and(|c| c.is_alphanumeric() || c == '.') {
        end += 1;
    }
    chars[start..end.min(chars.len())]
        .iter()
        .flat_map(|c| c.c.to_lowercase())
        .collect()
}

/// The run of digits ending just before `i`.
fn digits_before(chars: &[Ch], i: usize) -> String {
    let mut start = i;
    while start > 0 && chars[start - 1].c.is_ascii_digit() {
        start -= 1;
    }
    chars[start..i].iter().map(|c| c.c).collect()
}

/// The run of digits starting just after `i`.
fn digits_after(chars: &[Ch], i: usize) -> String {
    let mut end = i + 1;
    while is(chars, end, |c| c.is_ascii_digit()) {
        end += 1;
    }
    chars[i + 1..end].iter().map(|c| c.c).collect()
}

/// An en dash spaced on one side only.
///
/// The unspaced en dash is the nagykötőjel (`magyar–angol`) and the spaced one is the gondolatjel;
/// one space is neither. The gondolatjel reading is offered because a stray space is far more
/// common in running text than a nagykötőjel that lost one, but the rulebook does not decide it.
///
/// Rules: `PUN-DASH-012`, `PUN-DASH-014`
fn gondolatjel(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if chars[i].c != '–' {
        return;
    }
    let before = back_spaces(chars, i);
    let after = skip_spaces(chars, i + 1);
    let (has_before, has_after) = (before < i, after > i + 1);
    if has_before == has_after {
        return;
    }
    // Nothing on one side means a line edge, not a missing space: a dialogue turn opens its line
    // with the dash (`PUN-QUOT-012`).
    if before == 0 || chars[before - 1].c == '\n' || at(chars, after).is_none_or(|c| c == '\n') {
        return;
    }
    // An enclosure attaches to its contents, so `„– szó”` is not a dash missing a space on the
    // left. The bracket's own rule owns that side.
    if !has_before && at(chars, i.wrapping_sub(1)).is_some_and(crate::space::opener) {
        return;
    }
    // PUN-SPACE-001: the gondolatjel takes no space between it and a following `,` `;` `:` `.`,
    // and an enclosure closes against it.
    if !has_after && at(chars, i + 1).is_some_and(crate::space::attaches_left) {
        return;
    }
    push(
        out,
        protected,
        span(chars, before, after),
        "PUN-DASH-012",
        "the gondolatjel takes a space on both sides",
        Severity::Tendency,
        " – ",
    );
}

#[cfg(test)]
mod tests {
    use crate::{apply, check, normalise};

    fn fixed(text: &str) -> String {
        apply(text, &check(text))
    }

    #[test]
    fn a_spaced_hyphen_becomes_a_gondolatjel() {
        // PUN-DASH-019 case 2.
        assert_eq!(normalise("jön - ha ráér"), "jön – ha ráér");
        assert_eq!(
            normalise("A csapat - meglepetésre - nyert."),
            "A csapat – meglepetésre – nyert."
        );
    }

    #[test]
    fn dialogue_turns_open_their_lines() {
        for kept in [
            "– Hova a pokolba?\n– Maklárra.",
            "– Nagyon vártalak már – fogadta a barátját. – Sok a teendőnk.\n– Jó.",
            "– Gyere ide – kiáltott rá –, és segíts már egy kicsit!",
        ] {
            assert!(check(kept).is_empty(), "{kept}");
        }
    }

    #[test]
    fn year_pairs_take_the_en_dash() {
        // PUN-DASH-019 case 3.
        assert_eq!(normalise("1990-2000 között"), "1990–2000 között");
        assert_eq!(normalise("az 1848-49-es évek"), "az 1848–49-es évek");
        assert_eq!(normalise("a 2013-14. tanév"), "a 2013–14. tanév");
    }

    #[test]
    fn an_iso_date_is_not_a_year_range() {
        // PUN-DATEF-004 owns it; an ISO date is zero-padded and a shortened year never is.
        for kept in ["2024-01-05", "a 2024-01 havi", "1990-01-01"] {
            assert_eq!(normalise(kept), kept, "{kept}");
        }
    }

    #[test]
    fn approximation_keeps_its_hyphen() {
        // PUN-DASH-004: a hyphen between numbers means 'five or six' and is correct as written.
        for kept in ["5-6 darab", "2-2 alma", "20-30 fő"] {
            assert_eq!(normalise(kept), kept, "{kept}");
        }
    }

    #[test]
    fn a_cue_word_makes_a_number_pair_a_range() {
        // PUN-DASH-019 cases 4 and 5.
        assert_eq!(normalise("a 45-47. oldalon"), "a 45–47. oldalon");
        assert_eq!(normalise("Petőfi utca 12-14."), "Petőfi utca 12–14.");
    }

    #[test]
    fn a_month_makes_a_day_pair_a_range() {
        // PUN-DATEF-007: days within one month are linked by an en dash.
        assert_eq!(normalise("2014. április 28-30."), "2014. április 28–30.");
        assert_eq!(normalise("márc. 15-20. között"), "márc. 15–20. között");
    }

    #[test]
    fn clock_times_are_linked_by_an_en_dash() {
        // PUN-DATEF-013.
        assert_eq!(normalise("10.00-12.00 óráig"), "10.00–12.00 óráig");
        assert_eq!(normalise("10:00-12:00"), "10:00–12:00");
    }

    #[test]
    fn arithmetic_is_left_alone() {
        // PUN-DASH-016 owns the minus sign.
        assert_eq!(normalise("5 - 3 = 2"), "5 - 3 = 2");
    }

    #[test]
    fn suspended_compounds_keep_their_hyphens() {
        // PUN-DASH-002.
        for kept in [
            "bel- és külföldi",
            "tej-, hús- és halipar",
            "öt- és hatéves",
        ] {
            assert_eq!(normalise(kept), kept, "{kept}");
        }
    }

    #[test]
    fn the_em_dash_is_offered_the_en_dash() {
        // PUN-DASH-015 is a tendency: the em dash is a legacy variant, not an error in a source.
        assert_eq!(
            fixed("A csapat — meglepetésre — nyert."),
            "A csapat – meglepetésre – nyert."
        );
        assert_eq!(fixed("1990—2000"), "1990–2000");
        assert_eq!(normalise("A csapat — nyert."), "A csapat — nyert.");
    }

    #[test]
    fn a_nagykotojel_keeps_its_solid_spacing() {
        // PUN-DASH-005, -007: unspaced between the members it links.
        for kept in [
            "angol–magyar szótár",
            "Budapest–Győr",
            "1990–1994",
            "a 45–47. oldalon",
            "az 1848–49-es évek",
        ] {
            assert_eq!(normalise(kept), kept, "{kept}");
        }
    }

    #[test]
    fn a_correct_gondolatjel_is_untouched() {
        assert_eq!(
            normalise("A csapat – meglepetésre – nyert."),
            "A csapat – meglepetésre – nyert."
        );
        assert_eq!(
            normalise("Nem biztos – csak valószínű –, hogy késik."),
            "Nem biztos – csak valószínű –, hogy késik."
        );
    }

    #[test]
    fn a_half_spaced_en_dash_is_flagged() {
        // PUN-DASH-012: one space is neither the nagykötőjel nor the gondolatjel.
        assert_eq!(fixed("jön –ha ráér"), "jön – ha ráér");
        assert_eq!(fixed("jön– ha ráér"), "jön – ha ráér");
    }
}
