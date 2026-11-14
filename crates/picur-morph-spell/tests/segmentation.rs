//! How much of a corpus the segmenter can actually cut, and how cleanly.
//!
//! Segmentation is not guaranteed: a reading whose lemma the lexicon does not carry whole has no
//! entry to generate from, and a morph that rewrites the stem cannot be cut out of it. Both are
//! reported rather than hidden, because the share that comes back cut is what a caller building an
//! annotated corpus actually gets.
//!
//! Run with `CORPUS=<file> cargo test --release -p picur-morph-spell --test segmentation --
//! --ignored --nocapture`.

use picur_morph_core::{Role, segment_reading};
use picur_morph_spell::analyses;

#[test]
#[ignore = "measurement, needs CORPUS"]
fn how_much_of_a_corpus_is_cut() {
    let Ok(path) = std::env::var("CORPUS") else {
        return;
    };
    let text = std::fs::read_to_string(path).expect("corpus is readable");
    let words: Vec<&str> = text
        .split_whitespace()
        .map(|t| t.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|t| t.chars().any(char::is_alphabetic))
        .take(20_000)
        .collect();

    let (mut analysable, mut cut, mut clean, mut fused, mut preverb, mut affixes, mut members) =
        (0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    for w in &words {
        let Some(reading) = analyses(w).into_iter().next() else {
            continue;
        };
        analysable += 1;
        let Some(pieces) = segment_reading(&reading) else {
            continue;
        };
        cut += 1;
        let joined: String = pieces.iter().map(|p| p.text.as_ref()).collect();
        assert!(!joined.is_empty(), "{w}: empty segmentation");
        let mut had_fused = false;
        for p in &pieces {
            match &p.role {
                Role::Preverb => preverb += 1,
                Role::Member => members += 1,
                Role::Stem(m) if !m.is_empty() => had_fused = true,
                Role::Affix(_) => affixes += 1,
                _ => {}
            }
        }
        if had_fused {
            fused += 1;
        } else {
            clean += 1;
        }
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let pct = |k: usize, of: usize| k as f64 * 100.0 / of as f64;
    #[expect(clippy::print_stdout, reason = "measurement report")]
    {
        println!("{} tokens, {analysable} with a reading", words.len());
        println!(
            "  segmented        {cut:>6}  {:.1}% of readings",
            pct(cut, analysable)
        );
        println!(
            "    every morph cut  {clean:>6}  {:.1}% of segmented",
            pct(clean, cut)
        );
        println!(
            "    a morph fused    {fused:>6}  {:.1}% of segmented",
            pct(fused, cut)
        );
        println!("  preverbs split   {preverb:>6}");
        println!("  compound seams   {members:>6}");
        println!("  affix pieces     {affixes:>6}");
    }
}
