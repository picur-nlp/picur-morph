//! Measures the punctuation lint's false-positive rate against the rulebook's own examples.
//!
//! Every back-ticked Hungarian sentence in the rulebook is normatively punctuated, so
//! `picur_morph_lint::normalise` must leave it alone. Anything it changes is a false positive, and
//! the report names the rule that fired so the cause is not guessed at.
//!
//! This is the counterpart of `eval`: goldens measure what the lint gets right, this measures what
//! it breaks. A lint is only as good as its silence on correct text.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::Result;

/// Whether a back-ticked span is a Hungarian sentence rather than a form, a gloss or markup.
///
/// A sentence has several words and ends in a sentence mark. Requiring both keeps out the word
/// forms that fill most of the rulebook's back-ticks, and the feature labels and rule IDs with them.
fn is_sentence(s: &str) -> bool {
    let t = s.trim();
    if t.split_whitespace().count() < 3 || t.chars().count() > 200 {
        return false;
    }
    if !t.ends_with(['.', '!', '?', '…', '”', '"']) {
        return false;
    }
    // A marked example is deliberately wrong, so the lint is right to change it.
    if t.starts_with(['*', '%', '?', '#', '|']) || t.contains("->") || t.contains('~') {
        return false;
    }
    if t.contains(['=', '|', '<', '>']) {
        return false;
    }
    // The rulebook writes `...` for its own elisions and abbreviated patterns, and lists the marks
    // bare when stating an inventory. Neither is Hungarian prose, and counting them as false
    // positives would measure the extractor rather than the lint.
    if t.contains("...") || t.starts_with('(') {
        return false;
    }
    if t.chars().filter(|c| c.is_alphabetic()).count() < 10 {
        return false;
    }
    t.starts_with(|c: char| c.is_uppercase()) || t.starts_with(['„', '–'])
}

/// Back-ticked spans of one line, in order.
fn ticked(line: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(a) = rest.find('`') {
        let after = &rest[a + 1..];
        let Some(b) = after.find('`') else { break };
        out.push(&after[..b]);
        rest = &after[b + 1..];
    }
    out
}

/// Reads every chapter under `dir` and reports what the lint changes.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn report(dir: &Path) -> Result {
    let mut files: Vec<_> = fs::read_dir(dir)?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "md"))
        .collect();
    files.sort();

    let mut sentences = Vec::new();
    for path in &files {
        for line in fs::read_to_string(path)?.lines() {
            for s in ticked(line) {
                if is_sentence(s) {
                    sentences.push(s.to_owned());
                }
            }
        }
    }
    sentences.sort();
    sentences.dedup();

    let mut changed: Vec<(String, String, Vec<&'static str>)> = Vec::new();
    for s in &sentences {
        let out = picur_morph_lint::normalise(s);
        if out != *s {
            let rules = picur_morph_lint::check(s)
                .into_iter()
                .filter(|d| d.severity == picur_morph_lint::Severity::Error)
                .map(|d| d.rule)
                .collect();
            changed.push((s.clone(), out, rules));
        }
    }

    let total = sentences.len();
    let bad = changed.len();
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let rate = if total == 0 {
        0.0
    } else {
        bad as f64 * 100.0 / total as f64
    };
    println!("{bad}/{total} rulebook sentences changed ({rate:.2}% false positives)");

    let mut by_rule: BTreeMap<&str, usize> = BTreeMap::new();
    for (_, _, rules) in &changed {
        for r in rules {
            *by_rule.entry(r).or_default() += 1;
        }
    }
    if !by_rule.is_empty() {
        println!("\nby rule");
        let mut rows: Vec<_> = by_rule.into_iter().collect();
        rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        for (rule, n) in rows {
            println!("  {rule:<16} {n}");
        }
    }
    println!();
    for (before, after, rules) in changed.iter().take(40) {
        println!("  {rules:?}\n    {before}\n    {after}");
    }
    Ok(())
}
