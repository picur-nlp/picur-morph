//! Hungarian punctuation and typography diagnostics.
//!
//! Covers the families of `docs/grammar/08-punctuation-typography.md` that are decidable from the
//! character stream alone: spacing around marks, number and unit formatting, and the ellipsis.
//! The comma and clause families need a parse and are out of scope.
//!
//! Every function here is a pure function over text.

mod apostrophe;
mod article;
mod dash;
mod date;
mod ellipsis;
mod number;
mod protect;
mod quote;
mod space;
mod text;

use core::ops::Range;

pub use protect::Protected;

/// How far the rulebook commits to a diagnostic.
///
/// Mirrors the `Confidence` field of the rule entries, collapsed to what a consumer must decide:
/// whether to apply a fix unattended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Severity {
    /// `AkH` states the form. Applying the fix unattended is safe.
    Error,
    /// Typographic practice rather than a rule, or a rule whose trigger cannot be decided from
    /// the character stream. Offer it, do not apply it silently.
    Tendency,
}

/// One punctuation or typography finding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// Byte range in the input that the finding covers.
    pub span: Range<usize>,
    /// The rule identifier, as in `docs/grammar/08-punctuation-typography.md`.
    pub rule: &'static str,
    /// What is wrong.
    pub message: &'static str,
    /// Severity of the finding.
    pub severity: Severity,
    /// Replacement for `span`, or `None` when the rule detects an error but cannot choose a
    /// correction. `..` is the standing example: the rulebook says to ask.
    pub fix: Option<String>,
}

/// Every punctuation and typography finding in `text`, ordered by position.
///
/// Findings never overlap: where two rules reach the same characters the earlier and longer span
/// wins, so [`apply`] can splice them in one pass.
pub fn check(text: &str) -> Vec<Diagnostic> {
    let chars = text::scan(text);
    let protected = Protected::of(text);
    let mut out = Vec::new();
    let wanted = quote::wanted(&chars);
    space::check(&chars, &wanted, &protected, &mut out);
    number::check(&chars, &protected, &mut out);
    ellipsis::check(&chars, &protected, &mut out);
    dash::check(&chars, &protected, &mut out);
    date::check(&chars, &protected, &mut out);
    quote::check(&chars, &wanted, &protected, &mut out);
    apostrophe::check(&chars, &protected, &mut out);
    article::check(text, &chars, &protected, &mut out);
    out.sort_by(|a, b| {
        a.span
            .start
            .cmp(&b.span.start)
            .then(b.span.end.cmp(&a.span.end))
    });
    let mut end = 0;
    out.retain(|d| {
        let keep = d.span.start >= end;
        if keep {
            end = d.span.end;
        }
        keep
    });
    out
}

/// Applies the fixes of `diagnostics` to `text`, ignoring findings that carry none.
///
/// The diagnostics must come from [`check`] on the same text, which guarantees they are sorted and
/// disjoint.
pub fn apply(text: &str, diagnostics: &[Diagnostic]) -> String {
    let mut out = String::with_capacity(text.len());
    let mut at = 0;
    for d in diagnostics {
        let Some(fix) = &d.fix else { continue };
        if d.span.start < at {
            continue;
        }
        out.push_str(&text[at..d.span.start]);
        out.push_str(fix);
        at = d.span.end;
    }
    out.push_str(&text[at..]);
    out
}

/// Applies every fix that [`Severity::Error`] licenses, leaving tendencies alone.
///
/// Runs to a fixed point, because one rule can expose another: the spacing rules cannot see a
/// straight `"`, only the `„` that `PUN-QUOT-015` turns it into.
pub fn normalise(text: &str) -> String {
    let mut out = text.to_owned();
    for _ in 0..PASSES {
        let errors: Vec<_> = check(&out)
            .into_iter()
            .filter(|d| d.severity == Severity::Error)
            .collect();
        if errors.is_empty() {
            break;
        }
        let next = apply(&out, &errors);
        if next == out {
            break;
        }
        out = next;
    }
    out
}

/// The cap on how many times [`normalise`] re-runs the rules.
///
/// One rule exposes another: a quotation mark is normalised, which reveals its spacing, which
/// reveals a run of spaces. Diagnostics within a pass are disjoint, so a run of adjacent marks
/// settles one link per pass and the chain is as long as that run. The loop stops as soon as a
/// pass changes nothing, so this only bounds pathological input.
pub const PASSES: usize = 8;

#[cfg(test)]
mod tests {
    use super::{Severity, apply, check, normalise};

    #[test]
    fn correct_text_is_untouched() {
        for text in [
            "A macska felmászott a fára.",
            "Hát… nem tudom.",
            "A 15. §-ban olvasható.",
            "Ez 48%-os eredmény.",
            "A víz 100 °C-on forr.",
            "Írj a nev@pelda.hu címre!",
            "Nézd meg a www.mta.hu oldalon.",
            "A vonat 10.35-kor indul.",
            "Az érték 3,16 volt.",
            "bel- és külpolitika",
            "A mérkőzés 6 : 3 lett.",
        ] {
            assert_eq!(check(text), Vec::new(), "{text}");
        }
    }

    #[test]
    fn fixes_are_disjoint_and_splice_cleanly() {
        let text = "Szia ,Péter !Hogy vagy ?";
        let out = apply(text, &check(text));
        assert_eq!(out, "Szia, Péter! Hogy vagy?");
    }

    #[test]
    fn normalise_leaves_tendencies_alone() {
        // PUN-NUMF-019 is a tendency: `x` for the multiplication sign is a plain-text substitute.
        assert_eq!(normalise("A lap 20x30 cm."), "A lap 20x30 cm.");
        let d = check("A lap 20x30 cm.");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].severity, Severity::Tendency);
        assert_eq!(apply("A lap 20x30 cm.", &d), "A lap 20 × 30 cm.");
    }

    #[test]
    fn spans_are_char_boundaries() {
        // Accented letters are multibyte; a span that split one would panic in `apply`.
        let text = "Előtte ,utána !És tovább ?";
        for d in check(text) {
            assert!(text.is_char_boundary(d.span.start), "{d:?}");
            assert!(text.is_char_boundary(d.span.end), "{d:?}");
        }
        assert_eq!(apply(text, &check(text)), "Előtte, utána! És tovább?");
    }
}
