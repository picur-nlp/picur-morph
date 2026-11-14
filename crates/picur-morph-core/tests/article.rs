//! `PND-ART-003` names its own validation: an exhaustive property test against the spell-out
//! generator. The digit rule is a shortcut that avoids building the number's name, so the only
//! honest check is that it agrees with the name for every number it claims to cover.

use picur_morph_core::article::{Article, of_digits};
use picur_morph_core::numeral::{cardinal, ordinal};

/// The article the spelled-out form actually licenses.
fn from_spelling(word: &str) -> Option<Article> {
    word.chars().next().map(Article::of_onset)
}

#[test]
fn the_digit_rule_agrees_with_the_cardinal_spelling() {
    for n in 0..100_000u64 {
        let want = from_spelling(&cardinal(n));
        let got = of_digits(&n.to_string());
        assert_eq!(got, want, "{n} is {}", cardinal(n));
    }
}

#[test]
fn the_digit_rule_agrees_at_every_magnitude() {
    for exp in 0..15 {
        let base = 10u64.pow(exp);
        for k in [1, 2, 3, 4, 5, 6, 7, 8, 9] {
            for delta in [0, 1, 23, 456] {
                let n = base * k + delta;
                let want = from_spelling(&cardinal(n));
                let got = of_digits(&n.to_string());
                assert_eq!(got, want, "{n} is {}", cardinal(n));
            }
        }
    }
}

#[test]
fn the_digit_rule_agrees_with_the_ordinal_spelling() {
    // PND-ART-004: the ordinal never disagrees with the cardinal, `első` and `második` included.
    for n in 1..50_000u64 {
        let want = from_spelling(&ordinal(n));
        let got = of_digits(&n.to_string());
        assert_eq!(got, want, "{n} is {}", ordinal(n));
    }
}

#[test]
fn grouping_separators_are_not_pronounced() {
    for n in [1_000u64, 5_000, 55_000, 180_000, 1_200_000, 12_000] {
        let plain = of_digits(&n.to_string());
        let spaced = of_digits(&group(n, '\u{a0}'));
        let dotted = of_digits(&group(n, '.'));
        assert_eq!(plain, spaced, "{n}");
        assert_eq!(plain, dotted, "{n}");
    }
}

/// Writes `n` with `sep` between groups of three digits.
fn group(n: u64, sep: char) -> String {
    let d = n.to_string();
    let mut out = String::new();
    for (i, c) in d.chars().enumerate() {
        if i > 0 && (d.len() - i).is_multiple_of(3) {
            out.push(sep);
        }
        out.push(c);
    }
    out
}
