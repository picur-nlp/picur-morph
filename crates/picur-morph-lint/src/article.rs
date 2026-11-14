//! The definite article against the word it precedes.
//!
//! The two directions are not equally safe. The article `a` is never anything else, so `a alma` is
//! simply wrong. The string `az` is also the demonstrative pronoun 'that', which is a full noun
//! phrase and takes no following head: in `csak az kell` the rule would ask for `a` and be wrong.
//! Telling them apart needs syntax, so that direction is offered rather than applied.
//!
//! Rules: `PND-ART-001`, `PND-ART-003`, `PND-ART-004`, `PND-ART-005`, `PND-ART-008`

use picur_morph_core::article::{Article, article};

use crate::text::{Ch, span};
use crate::{Diagnostic, Protected, Severity};

/// Reports the article errors of `chars`.
pub(crate) fn check(text: &str, chars: &[Ch], protected: &Protected, out: &mut Vec<Diagnostic>) {
    for (start, end) in tokens(chars) {
        let word = chars[start..end].iter().map(|c| c.c).collect::<String>();
        let written = match word.to_lowercase().as_str() {
            "a" => Article::A,
            "az" => Article::Az,
            _ => continue,
        };
        // The article is a word of its own. `a1` is one token, a label, not `a` before a number.
        if chars.get(end).is_some_and(|c| c.c.is_alphanumeric())
            || (start > 0 && chars[start - 1].c.is_alphanumeric())
        {
            continue;
        }
        // Everything after the article decides, punctuation included: `PND-ART-008`.
        let rest = &text[chars[end - 1].end..];
        let Some(want) = article(rest) else { continue };
        if want == written {
            continue;
        }
        let at_span = span(chars, start, end);
        if protected.covers(&at_span) {
            continue;
        }
        let mut fix = want.as_str().to_owned();
        if word.starts_with(char::is_uppercase) {
            let mut rest = fix.chars();
            fix = rest
                .next()
                .map(|c| c.to_uppercase().to_string() + rest.as_str())
                .unwrap_or_default();
        }
        out.push(Diagnostic {
            span: at_span,
            rule: "PND-ART-001",
            message: "the definite article follows the pronunciation of the next word",
            severity: if written == Article::A {
                Severity::Error
            } else {
                Severity::Tendency
            },
            fix: Some(fix),
        });
    }
}

/// Runs of letters, as character position ranges.
fn tokens(chars: &[Ch]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if !chars[i].c.is_alphabetic() {
            i += 1;
            continue;
        }
        let start = i;
        while i < chars.len() && chars[i].c.is_alphabetic() {
            i += 1;
        }
        out.push((start, i));
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::{Severity, apply, check, normalise};

    #[test]
    fn a_before_a_vowel_is_corrected() {
        // PND-ART-001. This direction is unambiguous: `a` is only ever the article.
        assert_eq!(normalise("Leesett a alma."), "Leesett az alma.");
        assert_eq!(normalise("A ember jött."), "Az ember jött.");
        assert_eq!(normalise("a őz"), "az őz");
    }

    #[test]
    fn a_before_a_number_follows_the_spelled_form() {
        // PND-ART-003: `egy`, `öt` and `ezer` begin with a vowel.
        assert_eq!(normalise("a 1848-as évek"), "az 1848-as évek");
        assert_eq!(normalise("a 5 forint"), "az 5 forint");
        assert_eq!(normalise("a 1 fő"), "az 1 fő");
        for kept in ["a 2024-es év", "a 100 forint", "a 15 fő", "a 0 érték"] {
            assert_eq!(normalise(kept), kept, "{kept}");
        }
    }

    #[test]
    fn a_before_a_letter_follows_its_name() {
        // PND-ART-005.
        assert_eq!(normalise("a f) pont"), "az f) pont");
        assert_eq!(normalise("a m betű"), "az m betű");
        assert_eq!(normalise("a x tengely"), "az x tengely");
        for kept in ["a b) pont", "a h hang", "a z tengely"] {
            assert_eq!(normalise(kept), kept, "{kept}");
        }
    }

    #[test]
    fn punctuation_between_is_transparent() {
        // PND-ART-008.
        assert_eq!(normalise("a „alma”"), "az „alma”");
        assert_eq!(normalise("a (ember)"), "az (ember)");
    }

    #[test]
    fn the_demonstrative_az_is_only_offered() {
        // `az` is also the pronoun 'that', which takes no following head. Applying a fix here
        // would rewrite `csak az kell` to `csak a kell`.
        let text = "csak az kell";
        assert_eq!(normalise(text), text);
        let d = check(text);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].severity, Severity::Tendency);
        assert_eq!(apply(text, &d), "csak a kell");
    }

    #[test]
    fn an_article_glued_to_a_token_is_not_an_article() {
        // `a1` and `az2` are labels; splitting them into an article and a number would rewrite
        // identifiers.
        for kept in ["a1", "az2", "x a1 y", "A4-es lap"] {
            assert_eq!(check(kept), Vec::new(), "{kept}");
        }
    }

    #[test]
    fn an_acronym_takes_the_article_of_its_reading() {
        // PND-ART-006, PH-G2P-011: `MTA` is read out and begins with `em`, `NATO` is a word.
        for kept in ["az MTA", "a NATO", "az ELTE", "a MÁV", "az OTP"] {
            assert_eq!(check(kept), Vec::new(), "{kept}");
        }
        for flagged in ["a MTA", "az NATO", "a ELTE"] {
            assert!(!check(flagged).is_empty(), "{flagged}");
        }
    }

    #[test]
    fn correct_articles_are_untouched() {
        for kept in [
            "A macska felmászott a fára.",
            "Az alma leesett.",
            "az én házam",
            "a ház",
            "Az ember a fa alatt ült.",
        ] {
            assert_eq!(normalise(kept), kept, "{kept}");
        }
    }
}
