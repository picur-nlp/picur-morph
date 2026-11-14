//! Spacing around punctuation marks.
//!
//! Rules: `PUN-SPACE-001`, `PUN-SPACE-002`, `PUN-SPACE-005`, `PUN-SPACE-006`, `PUN-SPACE-007`

use crate::text::{Ch, at, back_spaces, is, is_space, skip_spaces, span};
use crate::{Diagnostic, Protected, Severity};

/// Marks that attach to what precedes them, so no space is wanted before one.
///
/// Used as the right-hand exception to "one space after a mark": `stb.,` and `?!` keep the marks
/// together. Rules: `PUN-SPACE-001`
pub(crate) fn attaches_left(c: char) -> bool {
    matches!(
        c,
        '.' | ',' | ';' | ':' | '!' | '?' | ')' | ']' | '”' | '«' | '…' | '%' | '‰' | '-'
    )
}

/// Whether the mark at `j` attaches to what precedes it, so no space belongs before it.
///
/// Context decides for two of them. A hyphen attaches only as a suffix hyphen (`”-ban`); followed
/// by a space it is a dash, and `„Gyere ide!” - mondta` must keep its space for `PUN-DASH-019` to
/// turn it into a gondolatjel. An ellipsis may be spaced from what precedes it, because
/// `PUN-ELL-003` writes an omitted sentence beginning that way.
fn attaches_at(chars: &[Ch], j: usize) -> bool {
    match at(chars, j) {
        Some('-') => is(chars, j + 1, char::is_lowercase),
        Some('…') | None => false,
        Some(c) => attaches_left(c),
    }
}

/// Brackets and quotation marks that open an enclosure.
///
/// An enclosure opened directly inside another is written solid (`(„szó”)`). A dash is not an
/// enclosure: `PUN-DASH-012` gives the gondolatjel a space on both sides whatever follows it, so
/// `– „Szia”` keeps its space.
pub(crate) fn opener(c: char) -> bool {
    matches!(c, '(' | '[' | '„' | '»')
}

/// Marks that want a space on their left.
fn wants_space_left(c: char) -> bool {
    matches!(c, '(' | '[' | '„' | '»' | '–' | '—' | '§' | '×')
}

/// Whether a space is wanted before and after `c`, from the `PUN-SPACE-001` attachment table.
fn spec(c: char) -> Option<(bool, bool)> {
    Some(match c {
        '.' | ',' | ';' | ':' | '?' | '!' | ')' | ']' | '”' | '«' => (false, true),
        '(' | '[' | '„' | '»' => (true, false),
        _ => return None,
    })
}

/// Reports the spacing errors of `chars`.
pub(crate) fn check(
    chars: &[Ch],
    wanted: &[Option<char>],
    protected: &Protected,
    out: &mut Vec<Diagnostic>,
) {
    for i in 0..chars.len() {
        mark(chars, wanted, protected, i, out);
        suffix_hyphen(chars, protected, i, out);
    }
    runs(chars, protected, out);
}

/// One mark, with both of its sides decided together.
///
/// A single diagnostic covers the space run before the mark, the mark, and the space run after it,
/// because the two sides are often both wrong (`Szia ,Péter`) and two overlapping diagnostics
/// would lose one of the fixes.
fn mark(
    chars: &[Ch],
    wanted: &[Option<char>],
    protected: &Protected,
    i: usize,
    out: &mut Vec<Diagnostic>,
) {
    let c = chars[i].c;
    let Some((mut want_before, mut want_after)) = spec(c) else {
        return;
    };
    let before = back_spaces(chars, i);
    let after = skip_spaces(chars, i + 1);
    let prev = if before == 0 {
        None
    } else {
        at(chars, before - 1)
    };
    let next = at(chars, after);
    let mut judge_before = prev.is_some_and(|p| p != '\n');
    let mut judge_after = next.is_some_and(|n| n != '\n');

    match c {
        // Periods carry times, decimals, abbreviations and ordinals. Only the one pattern that
        // PUN-SPACE-006 names is safe to flag: a lowercase letter, a period, a capital.
        '.' => {
            if crate::ellipsis::in_run(chars, i) {
                return;
            }
            judge_after &= at(chars, i + 1).is_some_and(char::is_uppercase)
                && i > 0
                && chars[i - 1].c.is_lowercase();
            judge_before &= prev.is_some_and(|p| p.is_alphanumeric() || attaches_left(p));
        }
        // A comma between digits is a decimal separator and a colon between them is a clock time
        // or a score, all of which the table exempts.
        ',' | ':' => {
            if prev.is_some_and(|p| p.is_ascii_digit()) && next.is_some_and(|n| n.is_ascii_digit())
            {
                return;
            }
        }
        // `írásjel(ek)`: a parenthesis opened inside a word is spaced as part of the word.
        '(' | '[' => {
            if at(chars, i.wrapping_sub(1)).is_some_and(char::is_alphanumeric) && i > 0 {
                judge_before = false;
            }
        }
        ')' | ']' if intra_word(chars, i) => return,
        // A guillemet pointing the wrong way is fixed by PUN-QUOT-002 first; its spacing follows
        // the direction it should have.
        // A mark that PUN-QUOT-015 is about to re-point takes the spacing of the character
        // it will become, so it is left to the next pass.
        _ if wanted[i].is_some() => return,
        _ => {}
    }

    if judge_after && want_after && attaches_at(chars, after) {
        want_after = false;
    }
    if judge_before && want_before && prev.is_some_and(opener) {
        want_before = false;
    }
    // Only a letter, a digit or another mark establishes what the spacing should be. A mark that
    // has its own entry in the table judges the gap on its left itself, so this one defers: both
    // reporting it inserted two spaces between `)` and `„`.
    judge_after &= next.is_some_and(|n| {
        n != '…'
            && spec(n).is_none()
            && (n.is_alphanumeric() || attaches_left(n) || wants_space_left(n))
    });
    judge_before &=
        prev.is_some_and(|p| p.is_alphanumeric() || attaches_left(p) || wants_space_left(p));

    let has_before = before < i;
    let has_after = after > i + 1;
    let bad_before = judge_before && has_before != want_before;
    let bad_after = judge_after && has_after != want_after;
    // PUN-SPACE-002: a correct side with more than one space is still wrong.
    let long_before = judge_before && want_before && before + 1 < i;
    let long_after = judge_after && want_after && after > i + 2;
    if !(bad_before || bad_after || long_before || long_after) {
        return;
    }

    let keep_before = if judge_before {
        want_before
    } else {
        has_before
    };
    let keep_after = if judge_after { want_after } else { has_after };
    let mut fix = String::new();
    if keep_before {
        fix.push(' ');
    }
    fix.push(c);
    if keep_after {
        fix.push(' ');
    }
    let at_span = span(chars, before, after);
    if protected.covers(&at_span) {
        return;
    }
    out.push(Diagnostic {
        span: at_span,
        rule: "PUN-SPACE-006",
        message: "punctuation mark is spaced wrongly",
        severity: Severity::Error,
        fix: Some(fix),
    });
}

/// Whether the parenthesis closing at `i` was opened inside a word (`Hara(n)g`).
fn intra_word(chars: &[Ch], i: usize) -> bool {
    let open = if chars[i].c == ')' { '(' } else { '[' };
    let close = chars[i].c;
    let mut depth = 0i32;
    for j in (0..i).rev() {
        let c = chars[j].c;
        if c == close {
            depth += 1;
        } else if c == open {
            if depth == 0 {
                return j > 0 && chars[j - 1].c.is_alphanumeric();
            }
            depth -= 1;
        }
    }
    false
}

/// `2024 -ben`: a suffix hyphen never takes a space before it.
///
/// Restricted to a hyphen followed by a lowercase letter, which is what a suffix looks like. A
/// hyphen followed by a space is a suspended compound (`bel- és`) or a dash, and a hyphen followed
/// by a digit is a range or a negative number.
///
/// Rules: `PUN-SPACE-007`
fn suffix_hyphen(chars: &[Ch], protected: &Protected, i: usize, out: &mut Vec<Diagnostic>) {
    if chars[i].c != '-' || !is(chars, i + 1, char::is_lowercase) {
        return;
    }
    let before = back_spaces(chars, i);
    if before == i || before == 0 {
        return;
    }
    if !is(chars, before - 1, |c| {
        c.is_alphanumeric() || c == '%' || c == ')'
    }) {
        return;
    }
    let at_span = span(chars, before, i);
    if protected.covers(&at_span) {
        return;
    }
    out.push(Diagnostic {
        span: at_span,
        rule: "PUN-SPACE-007",
        message: "no space separates a suffix hyphen from its base",
        severity: Severity::Error,
        fix: Some(String::new()),
    });
}

/// Runs of spaces between words, where no mark decided the spacing.
///
/// Rules: `PUN-SPACE-002`
fn runs(chars: &[Ch], protected: &Protected, out: &mut Vec<Diagnostic>) {
    let mut i = 0;
    while i < chars.len() {
        if !is_space(chars[i].c) {
            i += 1;
            continue;
        }
        let end = skip_spaces(chars, i);
        let indented = i == 0 || chars[i - 1].c == '\n';
        if end - i > 1 && !indented && at(chars, end).is_some_and(|c| c != '\n') {
            let at_span = span(chars, i, end);
            if !protected.covers(&at_span) {
                out.push(Diagnostic {
                    span: at_span,
                    rule: "PUN-SPACE-002",
                    message: "words are separated by exactly one space",
                    severity: Severity::Error,
                    fix: Some(" ".into()),
                });
            }
        }
        i = end;
    }
}

#[cfg(test)]
mod tests {
    use crate::{apply, check};

    fn fixed(text: &str) -> String {
        apply(text, &check(text))
    }

    #[test]
    fn a_space_before_an_attaching_mark_is_removed() {
        // PUN-SPACE-005, PUN-SPACE-006.
        assert_eq!(fixed("Miért ?"), "Miért?");
        assert_eq!(fixed("Igen , persze."), "Igen, persze.");
        assert_eq!(fixed("Hogyan ; mikor?"), "Hogyan; mikor?");
        assert_eq!(fixed("Vége ."), "Vége.");
    }

    #[test]
    fn a_missing_space_after_a_mark_is_inserted() {
        assert_eq!(fixed("Szia,Péter"), "Szia, Péter");
        assert_eq!(fixed("Igen!Megyek"), "Igen! Megyek");
        assert_eq!(fixed("Miért?Nem tudom"), "Miért? Nem tudom");
        assert_eq!(fixed("Vége.Új mondat."), "Vége. Új mondat.");
    }

    #[test]
    fn both_sides_of_one_mark_are_fixed_together() {
        // Two overlapping diagnostics would drop one of the two fixes.
        assert_eq!(fixed("Szia ,Péter"), "Szia, Péter");
        assert_eq!(fixed("Igen !Megyek"), "Igen! Megyek");
    }

    #[test]
    fn attaching_marks_stay_together() {
        // PUN-SPACE-001: no space is inserted between a mark and a following attaching mark.
        for kept in ["stb., és", "Mi?! Nem.", "Ezt mondta: „Jó”.", "a 3.-nak"] {
            assert_eq!(fixed(kept), kept, "{kept}");
        }
    }

    #[test]
    fn numeric_punctuation_is_left_alone() {
        // Times, decimals, scores and version numbers all look like spacing errors.
        for kept in [
            "A vonat 10.35-kor indul.",
            "Az érték 3,16 volt.",
            "A mérkőzés 6 : 3 lett.",
            "A v1.3 verzió.",
            "Az 1.2.3 pont.",
        ] {
            assert_eq!(fixed(kept), kept, "{kept}");
        }
    }

    #[test]
    fn addresses_are_left_alone() {
        // PUN-SPACE-003.
        for kept in [
            "Nézd meg a www.mta.hu oldalon.",
            "Írj a nev@pelda.hu címre.",
            "Nyisd meg: http://example.hu/oldal.",
        ] {
            assert_eq!(fixed(kept), kept, "{kept}");
        }
    }

    #[test]
    fn parentheses_are_spaced_outside_and_solid_inside_a_word() {
        assert_eq!(fixed("a szó (zárójel) után"), "a szó (zárójel) után");
        // PUN-SPACE-001: a parenthesis inside a word takes no space on either side.
        assert_eq!(fixed("az írásjel(ek) helye"), "az írásjel(ek) helye");
        assert_eq!(fixed("a Hara(n)g szó"), "a Hara(n)g szó");
        assert_eq!(fixed("a szó ( zárójel )"), "a szó (zárójel)");
    }

    #[test]
    fn quotation_marks_take_their_side() {
        assert_eq!(fixed("Ezt mondta: „ Jó ”"), "Ezt mondta: „Jó”");
    }

    #[test]
    fn runs_of_spaces_collapse() {
        // PUN-SPACE-002.
        assert_eq!(fixed("Vége.  Új mondat."), "Vége. Új mondat.");
        assert_eq!(fixed("két    szó"), "két szó");
        assert_eq!(fixed("Igen  ,  persze"), "Igen, persze");
    }

    #[test]
    fn indentation_and_line_breaks_are_layout() {
        // Collapsing these would reflow the text rather than punctuate it.
        assert_eq!(fixed("    behúzott sor"), "    behúzott sor");
        assert_eq!(fixed("első sor\nmásodik sor"), "első sor\nmásodik sor");
    }

    #[test]
    fn a_suffix_hyphen_loses_its_space() {
        // PUN-SPACE-007.
        assert_eq!(fixed("a 2024 -ben"), "a 2024-ben");
        assert_eq!(fixed("az MTA -t"), "az MTA-t");
        assert_eq!(fixed("bel- és külpolitika"), "bel- és külpolitika");
        assert_eq!(fixed("öt- és hatéves"), "öt- és hatéves");
    }
}
