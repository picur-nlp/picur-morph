//! Throughput of the checker on real text.
//!
//! A tracked number, not a micro-benchmark: ranking scraped Hungarian means running the checker
//! over billions of words, and the cost per word is what decides whether that is hours or weeks.
//! It degrades as input quality drops, because an accepted word answers from the fast path while a
//! rejected or unknown one builds readings, and scraped text is full of the latter.
//!
//! Tokens are trimmed of surrounding punctuation, as `picur-morph-rank` does before it calls the
//! checker. That is not a detail: a token still carrying its comma is certainly rejected and pays
//! the full exhaustive search, and on this corpus a fifth of the whitespace tokens carry one. The
//! untrimmed reading spent most of its time on strings the product never submits and reported 749
//! words/s where the same text actually runs at 2,746.
//!
//! Run with `CORPUS=<file> cargo test --release -p picur-morph-spell --test throughput -- --ignored
//! --nocapture`.

use std::time::Instant;

use picur_morph_spell::{Status, check};

#[test]
#[ignore = "benchmark, needs CORPUS"]
fn words_per_second() {
    let Ok(path) = std::env::var("CORPUS") else {
        return;
    };
    let text = std::fs::read_to_string(path).expect("corpus is readable");
    let words: Vec<&str> = text
        .split_whitespace()
        .map(|t| t.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|t| t.chars().any(char::is_alphabetic))
        .take(200_000)
        .collect();
    let start = Instant::now();
    let ok = words.iter().filter(|w| check(w) == Status::Ok).count();
    let secs = start.elapsed().as_secs_f64();
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let rate = words.len() as f64 / secs;
    #[expect(clippy::print_stdout, reason = "benchmark report")]
    {
        println!(
            "{} words in {secs:.1}s = {rate:.0} words/s, {ok} accepted",
            words.len()
        );
    }
}
