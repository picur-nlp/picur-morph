//! Measures the lint's false-positive rate over real edited Hungarian.
//!
//! `punct` uses the rulebook's own sentences, which are short, normative and few. This reads a
//! plain-text corpus, one sentence per line, and reports what the lint changes there. Published
//! prose is not error-free, so a finding is not automatically a false positive; but at the rates
//! a grammar lint must hit, every rule that fires often on edited text is wrong until shown
//! otherwise, and the samples say which.
//!
//! Syntax rules are the reason this exists: a punctuation rule can be judged from the rulebook,
//! while an agreement rule only shows its precision against running text.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use picur_morph_lint::{Severity, check};

use crate::Result;

/// Reads `path` as one sentence per line and reports the findings by rule.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn report(path: &Path, only: Option<&str>) -> Result {
    let text = fs::read_to_string(path)?;
    let lines: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|l| l.chars().filter(|c| c.is_alphabetic()).count() >= 10)
        .collect();

    let mut by_rule: BTreeMap<&str, (usize, Vec<String>)> = BTreeMap::new();
    let mut flagged = 0usize;
    for line in &lines {
        let found: Vec<_> = check(line)
            .into_iter()
            .filter(|d| d.severity == Severity::Error)
            .filter(|d| only.is_none_or(|r| d.rule == r))
            .collect();
        if found.is_empty() {
            continue;
        }
        flagged += 1;
        for d in found {
            let entry = by_rule.entry(d.rule).or_default();
            entry.0 += 1;
            if entry.1.len() < 8 {
                let fix = d.fix.as_deref().unwrap_or("-");
                entry
                    .1
                    .push(format!("{} [{}] {line}", &line[d.span.clone()], fix));
            }
        }
    }

    let total = lines.len();
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let rate = if total == 0 {
        0.0
    } else {
        flagged as f64 * 100.0 / total as f64
    };
    println!("{flagged}/{total} sentences flagged ({rate:.3}%)");

    let mut rows: Vec<_> = by_rule.into_iter().collect();
    rows.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(b.0)));
    for (rule, (n, samples)) in rows {
        println!("\n{rule:<16} {n}");
        for s in samples {
            println!("    {s}");
        }
    }
    Ok(())
}
