//! Tokens that a corpus contains and a dictionary does not.
//!
//! Scraped text is not a corpus of words. It carries URL slugs, runs of punctuation and fragments
//! of markup, and the checker meets all of them. These are the shapes that have actually broken it.

use std::time::{Duration, Instant};

use picur_morph_spell::check;

/// A hyphenated token must not cost more than a word.
///
/// Reading a hyphenated compound analyses both sides of every seam, and those sides are analysed
/// at every seam again, so the cost grew exponentially with the number of hyphens: four took
/// 0.21s, eight took 3.15s, twelve did not finish, and on a spawned thread the recursion overflowed
/// the stack and aborted the process. A crawl of 10,000 Hungarian pages hit it.
#[test]
fn a_long_hyphen_run_is_answered_at_once() {
    // The first call loads the lexicon, which must stay outside the timed calls.
    let _ = check("ház");
    for n in [8usize, 40, 200] {
        let word = std::iter::repeat_n("ha", n).collect::<Vec<_>>().join("-");
        let start = Instant::now();
        let _ = check(&word);
        let took = start.elapsed();
        assert!(
            took < Duration::from_millis(500),
            "{n} hyphens took {took:?}"
        );
    }
}

/// Styled Unicode must not recurse forever.
///
/// The mathematical alphanumerics social media uses for bold and italic text are uppercase with no
/// lowercase mapping, so decapitalising them returned the word unchanged and the analyser reached
/// the same string again. A crawl of 10,000 Hungarian pages aborted the process on one of these.
#[test]
fn styled_unicode_is_answered_at_once() {
    let _ = check("ház");
    for word in [
        "\u{1D5D9}\u{1D5F6}\u{1D5FF}\u{1D600}\u{1D5FD}",
        "\u{1D400}\u{1D401}\u{1D402}",
        "\u{1D5D4}",
    ] {
        let start = Instant::now();
        let _ = check(word);
        let took = start.elapsed();
        assert!(took < Duration::from_millis(500), "{word:?} took {took:?}");
    }
}

/// The shapes a tokeniser leaves behind, none of which may panic.
#[test]
fn punctuation_and_fragments_do_not_panic() {
    for word in [
        "-",
        "--",
        "---------",
        "a-",
        "-a",
        "a--b",
        "....",
        "!!!",
        "a.b.c.d.e",
        "1-2-3-4-5-6-7-8",
        "http://example.hu/a-b-c-d-e-f-g",
        "ház-",
        "-ház",
        "e-e-e-e-e-e-e-e-e-e",
        "A-B-C-D-E-F-G",
        "\u{1D5D9}\u{1D5F6}\u{1D5FF}\u{1D600}\u{1D5FD}",
        "\u{2160}",
        "\u{01C5}",
    ] {
        let _ = check(word);
    }
}
