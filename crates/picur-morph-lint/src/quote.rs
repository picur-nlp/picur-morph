//! Quotation marks.
//!
//! Rules: `PUN-QUOT-001`, `PUN-QUOT-002`, `PUN-QUOT-015`

use crate::text::{Ch, at, span};
use crate::{Diagnostic, Protected, Severity};

/// Whether the mark at `i` stands where a quotation opens.
///
/// Nothing but the left neighbour decides this: a mark after a space, an opening bracket or
/// another opening quote begins a quotation, and a mark after a word or a closing mark ends one.
pub(crate) fn opens(chars: &[Ch], i: usize) -> bool {
    match at(chars, i.wrapping_sub(1)).filter(|_| i > 0) {
        None => true,
        // A colon introduces a quotation (`mondta:„Jó”`), while a letter, a digit or a sentence
        // mark is the end of the quoted text.
        Some(c) => {
            c.is_whitespace() || matches!(c, '(' | '[' | '„' | '»' | '–' | '—' | '/' | ':' | ';')
        }
    }
}

/// Whether the text uses the straight single quote as a quotation mark rather than an apostrophe.
///
/// `hál'` is an apostrophe and the closing half of `'szó'` is a mark; only the text as a whole
/// tells them apart. A shortened year is excluded because `'90` stands exactly where a quotation
/// would open and would otherwise disable the apostrophe rules for the whole text.
pub(crate) fn single_quotes_are_marks(chars: &[Ch]) -> bool {
    (0..chars.len()).any(|i| {
        chars[i].c == '\''
            && opens(chars, i)
            && !(at(chars, i + 1).is_some_and(|c| c.is_ascii_digit())
                && at(chars, i + 2).is_some_and(|c| c.is_ascii_digit())
                && !at(chars, i + 3).is_some_and(|c| c.is_ascii_digit()))
    })
}

/// The character each quotation mark ought to be, by nesting depth.
///
/// Direction follows the depth, never the neighbouring character. A mark decided by its left
/// neighbour oscillates: `)»` reads as a closer, becomes `«`, and then reads as an opener again.
/// `PUN-QUOT-002` states the pairs by depth, and counting marks is what implements that.
pub(crate) fn wanted(chars: &[Ch]) -> Vec<Option<char>> {
    let mut out = vec![None; chars.len()];
    let mut doubles = 0usize;
    let mut guillemets = 0usize;
    for i in 0..chars.len() {
        let want = match chars[i].c {
            // A straight double after a digit in a coordinate is a double prime, not a mark.
            '"' | '\u{201c}' | '\u{201d}' | '„' => {
                if crate::apostrophe::is_prime(chars, i) {
                    continue;
                }
                doubles += 1;
                if doubles % 2 == 1 { '„' } else { '”' }
            }
            '«' | '»' => {
                guillemets += 1;
                if guillemets % 2 == 1 { '»' } else { '«' }
            }
            _ => continue,
        };
        if want != chars[i].c {
            out[i] = Some(want);
        }
    }
    out
}

/// The quotation marks OCR and typewriters spell with two characters: `,,` opening, `''`
/// closing and `>>` `<<` for the guillemets (`PUN-QUOT-015`, `REP-OCR-003`).
fn two_character_marks(chars: &[Ch], protected: &Protected, out: &mut Vec<Diagnostic>) {
    for i in 0..chars.len().saturating_sub(1) {
        let pair = (chars[i].c, chars[i + 1].c);
        let want = match pair {
            (',', ',') if opens(chars, i) => "\u{201E}",
            ('\'', '\'') if !opens(chars, i) => "\u{201D}",
            ('>', '>') => "\u{00BB}",
            ('<', '<') => "\u{00AB}",
            _ => continue,
        };
        let at_span = span(chars, i, i + 2);
        if protected.covers(&at_span) {
            continue;
        }
        out.push(Diagnostic {
            span: at_span,
            rule: "PUN-QUOT-015",
            message: "Hungarian quotation marks are the low-high pair and inward guillemets",
            severity: Severity::Error,
            fix: Some(want.into()),
        });
    }
}

/// Reports the quotation mark errors of `chars`.
pub(crate) fn check(
    chars: &[Ch],
    wanted: &[Option<char>],
    protected: &Protected,
    out: &mut Vec<Diagnostic>,
) {
    two_character_marks(chars, protected, out);
    for (i, want) in wanted.iter().enumerate() {
        let Some(want) = want else { continue };
        let at_span = span(chars, i, i + 1);
        if protected.covers(&at_span) {
            continue;
        }
        out.push(Diagnostic {
            span: at_span,
            rule: "PUN-QUOT-015",
            message: "Hungarian quotation marks are the low-high pair and inward guillemets",
            severity: Severity::Error,
            fix: Some(want.to_string()),
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::normalise;

    #[test]
    fn straight_quotes_become_the_hungarian_pair() {
        // PUN-QUOT-015.
        assert_eq!(normalise(r#"Ezt mondta: "Jó"."#), "Ezt mondta: „Jó”.");
        assert_eq!(normalise(r#"A "szó" jelentése."#), "A „szó” jelentése.");
    }

    #[test]
    fn english_curly_quotes_become_the_hungarian_pair() {
        assert_eq!(
            normalise("Ezt mondta: \u{201c}Jó\u{201d}."),
            "Ezt mondta: „Jó”."
        );
    }

    #[test]
    fn reversed_guillemets_are_turned_inward() {
        // PUN-QUOT-002: the inner pair points inward in Hungarian.
        assert_eq!(normalise("„Ezt «mondta» ő”"), "„Ezt »mondta« ő”");
    }

    #[test]
    fn correct_quotations_are_untouched() {
        for kept in [
            "Ezt mondta: „Jó”.",
            "„Ezt »mondta« ő”",
            "A „Magyar szólások”-ban.",
        ] {
            assert_eq!(normalise(kept), kept, "{kept}");
        }
    }

    #[test]
    fn a_double_prime_after_a_digit_is_not_a_quotation_mark() {
        // PUN-APOS-006 owns it; converting it to `”` would be damage.
        assert_eq!(normalise("47° 13' 5\""), "47° 13′ 5″");
    }

    #[test]
    fn normalising_also_fixes_the_spacing_it_reveals() {
        // The spacing rules cannot see a `"`; they can see the `„` it becomes.
        assert_eq!(normalise(r#"Ezt mondta:"Jó"."#), "Ezt mondta: „Jó”.");
    }
}
