//! Measures how well the suggester ranks the correction a rule asks for.
//!
//! Chapter 10's goldens pair a wrong spelling with the right one, which is exactly a suggestion
//! question: given the wrong form, does [`picur_morph_spell::suggest`] offer the right one, and
//! does it offer it first? Accuracy at one is the figure that matters to a user, because a
//! correction buried sixth is a correction nobody takes.
//!
//! ```text
//! cargo run --release -p xtask -- suggest
//! ```

use std::fs;
use std::path::Path;

use picur_morph_spell::{Status, check, suggest};

use crate::Result;

/// One (wrong, right) pair from the goldens.
struct Pair {
    rule: String,
    wrong: String,
    right: String,
}

/// The chapter-10 goldens that are a spelling correction.
///
/// A pair only counts when the input really is wrong and the output really is right: several rules
/// pair two legal forms to show a preference, and a suggester is not asked about those.
fn pairs(text: &str) -> Vec<Pair> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with("(rule:") || !line.contains("chapter:\"10\"") {
            continue;
        }
        let field = |name: &str| -> Option<String> {
            let i = line.find(&format!("{name}:\""))? + name.len() + 2;
            let rest = &line[i..];
            Some(rest[..rest.find('"')?].to_owned())
        };
        let (Some(rule), Some(wrong), Some(right)) =
            (field("rule"), field("input"), field("output"))
        else {
            continue;
        };
        // A single word on each side, or the question is a text repair rather than a suggestion.
        if wrong.split_whitespace().count() != 1 || right.split_whitespace().count() != 1 {
            continue;
        }
        if check(&wrong) == Status::Ok || check(&right) != Status::Ok {
            continue;
        }
        out.push(Pair { rule, wrong, right });
    }
    out
}

/// Reads the goldens and reports suggestion accuracy.
///
/// `limit` caps how many pairs are asked, because a suggestion costs seconds: `suggest` builds
/// some seven hundred candidates and each one whose stem is real pays a full failing search.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn report(goldens: &Path, limit: Option<usize>) -> Result {
    let text = fs::read_to_string(goldens)?;
    let mut pairs = pairs(&text);
    if let Some(n) = limit {
        pairs.truncate(n);
    }
    let pairs = pairs;
    if pairs.is_empty() {
        return Err("no single-word correction pairs in chapter 10".into());
    }
    let mut first = 0usize;
    let mut anywhere = 0usize;
    let mut missed: Vec<(&str, &str, &str, Vec<String>)> = Vec::new();
    for p in &pairs {
        let offered = suggest(&p.wrong);
        let at = offered.iter().position(|s| *s == p.right);
        match at {
            Some(0) => {
                first += 1;
                anywhere += 1;
            }
            Some(_) => {
                anywhere += 1;
                missed.push((&p.rule, &p.wrong, &p.right, offered));
            }
            None => missed.push((&p.rule, &p.wrong, &p.right, offered)),
        }
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let pct = |n: usize| n as f64 * 100.0 / pairs.len() as f64;
    println!("{} correction pairs from chapter 10", pairs.len());
    println!("  offered first   {first:>4}  {:>6.2}%", pct(first));
    println!("  offered at all  {anywhere:>4}  {:>6.2}%", pct(anywhere));
    // Recall and ranking are different failures with different fixes, so the ranking figure is
    // reported over the pairs the suggester actually reached. An archaic spelling like
    // `philosophia` is four edits from `filozófia` and belongs to `repair_word`, not here.
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    if anywhere > 0 {
        println!(
            "  first of those  {first:>4}  {:>6.2}%   ranking alone",
            first as f64 * 100.0 / anywhere as f64
        );
    }
    println!("\nnot offered first");
    for (rule, wrong, right, offered) in missed.iter().take(25) {
        println!("  {rule:<18} {wrong} -> {right}  got {offered:?}");
    }
    Ok(())
}
