//! Properties that must hold for every input, not just the curated cases.

use picur_morph_lint::{Severity, apply, check, normalise};

/// Characters that exercise every rule, including the ones that damage each other.
const ALPHABET: &[&str] = &[
    "a", "á", "Ő", "z", "1", "9", " ", "  ", "\t", "\n", ".", "..", "...", ". . .", ",", ";", ":",
    "!", "?", "(", ")", "[", "]", "„", "”", "»", "«", "\"", "'", "’", "´", "…", "-", "–", "%", "°",
    "§", "×", "x", "C", "/", "@", "_", "www.", "12345", "23816",
];

/// Every string of `len` pieces from the alphabet, as a deterministic pseudo-random sample.
fn sample(len: usize, count: usize) -> impl Iterator<Item = String> {
    let n = ALPHABET.len();
    (0..count).map(move |seed| {
        let mut s = String::new();
        let mut x = seed.wrapping_mul(2_654_435_761);
        for _ in 0..len {
            x = x.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            s.push_str(ALPHABET[(x >> 16) % n]);
        }
        s
    })
}

#[test]
fn normalising_is_idempotent() {
    for len in 1..=12 {
        for text in sample(len, 20000) {
            let once = normalise(&text);
            let twice = normalise(&once);
            assert_eq!(once, twice, "not idempotent on {text:?}");
        }
    }
}

#[test]
fn diagnostics_are_sorted_and_disjoint() {
    for len in 1..=12 {
        for text in sample(len, 20000) {
            let mut end = 0;
            for d in check(&text) {
                assert!(d.span.start >= end, "overlapping spans in {text:?}: {d:?}");
                assert!(d.span.end >= d.span.start, "inverted span in {text:?}");
                assert!(d.span.end <= text.len(), "span past the end in {text:?}");
                end = d.span.end;
            }
        }
    }
}

#[test]
fn spans_never_split_a_character() {
    // Hungarian accented letters and every quotation mark are multibyte; a span that cut one would
    // panic in `apply` rather than return a wrong answer.
    for len in 1..=12 {
        for text in sample(len, 20000) {
            for d in check(&text) {
                assert!(text.is_char_boundary(d.span.start), "{text:?} {d:?}");
                assert!(text.is_char_boundary(d.span.end), "{text:?} {d:?}");
            }
        }
    }
}

#[test]
fn applying_every_fix_still_converges() {
    // `normalise` applies only errors; tendencies must not fight them or each other. A run of
    // adjacent marks settles one link per pass, so convergence is bounded by `PASSES`, not by one.
    for len in 1..=12 {
        for text in sample(len, 20000) {
            let mut at = text.clone();
            let mut settled = false;
            for _ in 0..picur_morph_lint::PASSES {
                let next = apply(&at, &check(&at));
                if next == at {
                    settled = true;
                    break;
                }
                at = next;
            }
            assert!(settled, "does not converge from {text:?}, stuck at {at:?}");
        }
    }
}

#[test]
fn letters_and_digits_are_never_lost() {
    // The punctuation rules move, merge or replace marks; none of them may touch the words. The
    // article rule is the one exception, since `a` and `az` are themselves words, so it is applied
    // separately and excluded here.
    for len in 1..=12 {
        for text in sample(len, 20000) {
            let punctuation: Vec<_> = check(&text)
                .into_iter()
                .filter(|d| d.severity == Severity::Error && d.rule != "PND-ART-001")
                .collect();
            let words: String = text.chars().filter(|c| c.is_alphanumeric()).collect();
            let after: String = apply(&text, &punctuation)
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect();
            assert_eq!(words, after, "letters changed in {text:?}");
        }
    }
}

#[test]
fn a_tendency_is_never_applied_by_normalise() {
    for len in 1..=12 {
        for text in sample(len, 10000) {
            let tendencies: Vec<_> = check(&text)
                .into_iter()
                .filter(|d| d.severity == Severity::Tendency)
                .collect();
            if tendencies.iter().all(|d| d.fix.is_none()) {
                continue;
            }
            // The normalised text must still be reachable without the tendency fixes applied.
            let n = normalise(&text);
            assert_eq!(n, normalise(&n), "{text:?}");
        }
    }
}
