//! How the checker scales across cores.
//!
//! Ranking a corpus is embarrassingly parallel: a verdict is a pure function of one token, so the
//! only thing that can serialise it is the memo.
//!
//! The memo is shared behind a lock, and that is the measured choice rather than the obvious one.
//! A per-thread cache, which takes no lock at all, came out slower at 1, 4 and 16 threads alike:
//! threads checking the same corpus miss on the same words, so sharing the answers is worth more
//! than the lock costs. Scaling is around 6x on 16 threads of a 24-core machine, and what caps it
//! is not the lock.
//!
//! One thread count per process, selected by `THREADS`. A cache that outlives the run would make
//! every configuration after the first a stream of hits, so the configurations cannot share a
//! process and still be comparable.
//!
//! Run with `CORPUS=<file> THREADS=<n> cargo test --release -p picur-morph-spell --test scaling --
//! --ignored --nocapture`.

use std::time::Instant;

use picur_morph_spell::{Status, check};

/// Checks `words` on `threads` threads and returns the rate and the accepted count.
fn run(words: &[&str], threads: usize) -> (f64, usize) {
    let chunk = words.len().div_ceil(threads);
    let start = Instant::now();
    let ok: usize = std::thread::scope(|s| {
        let handles: Vec<_> = words
            .chunks(chunk)
            .map(|part| s.spawn(move || part.iter().filter(|w| check(w) == Status::Ok).count()))
            .collect();
        handles.into_iter().filter_map(|h| h.join().ok()).sum()
    });
    let secs = start.elapsed().as_secs_f64();
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let rate = words.len() as f64 / secs;
    (rate, ok)
}

#[test]
#[ignore = "benchmark, needs CORPUS"]
fn scales_across_threads() {
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

    let threads: usize = std::env::var("THREADS")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(1);
    let (rate, ok) = run(&words, threads);
    #[expect(clippy::print_stdout, reason = "benchmark report")]
    {
        println!("{threads:>3} threads {rate:>9.0} words/s  {ok} accepted");
    }
}
