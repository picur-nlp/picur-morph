//! Reports which rules the code claims to implement.
//!
//! Traceability is the property that makes this project reviewable: a reader who doubts a form
//! should be able to follow its rule ID from the code to the evidence that settled it. This command
//! measures whether that chain exists.
//!
//! Code declares coverage in a doc comment: `/// Rules: NOM-ACC-003, NOM-ACC-009`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::Result;

/// A rule as the rulebook declares it.
pub(crate) struct Spec {
    pub id: String,
    pub chapter: String,
    pub predictability: String,
    /// The suffixes its heading names in archiphoneme notation (`-sÁg`, `-(O)gAt`).
    pub suffixes: Vec<String>,
}

fn is_rule_id(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    parts.len() == 3
        && (2..=4).contains(&parts[0].len())
        && parts[0].chars().all(|c| c.is_ascii_uppercase())
        && !parts[2].is_empty()
        && parts[2].chars().all(|c| c.is_ascii_digit())
}

/// The primary backticked token of a field, e.g. `rule` from ``- **Predictability**: `rule` (...)``.
pub(crate) fn primary_token(v: &str) -> String {
    v.split('`')
        .nth(1)
        .unwrap_or_else(|| v.split_whitespace().next().unwrap_or("?"))
        .to_owned()
}

pub(crate) fn specs(dir: &Path) -> Result<Vec<Spec>> {
    let mut files: Vec<_> = fs::read_dir(dir)?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|e| e == "md")
                && p.file_stem()
                    .and_then(|s| s.to_str())
                    .is_some_and(|s| s.starts_with(|c: char| c.is_ascii_digit()))
        })
        .collect();
    files.sort();

    let mut out = Vec::new();
    for path in &files {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let chapter = stem.get(..2).unwrap_or(stem).to_owned();
        let text = fs::read_to_string(path)?;
        for block in text.split("\n### ").skip(1) {
            let Some(id) = block.split_whitespace().next() else {
                continue;
            };
            if !is_rule_id(id) {
                continue;
            }
            let pred = block
                .find("- **Predictability**")
                .and_then(|i| {
                    let rest = &block[i..];
                    let end = rest.find('\n').unwrap_or(rest.len());
                    rest[..end]
                        .find(':')
                        .map(|c| primary_token(&rest[c + 1..end]))
                })
                .unwrap_or_else(|| "?".to_owned());
            let heading = block.lines().next().unwrap_or_default();
            let suffixes = heading
                .split_whitespace()
                .map(|w| w.trim_end_matches([':', ',', ';']))
                .filter(|w| w.len() > 1 && w.starts_with('-'))
                .map(str::to_owned)
                .collect();
            out.push(Spec {
                id: id.to_owned(),
                chapter: chapter.clone(),
                predictability: pred,
                suffixes,
            });
        }
    }
    Ok(out)
}

/// Each rule's declared predictability, by rule ID.
///
/// A rule that declares itself `lexical` states that its output is stored rather than derived, so
/// a generator cannot reproduce its cases and counting them with the rest hides which failures are
/// the engine's.
pub(crate) fn predictability(dir: &Path) -> Result<BTreeMap<String, String>> {
    Ok(specs(dir)?
        .into_iter()
        .map(|s| (s.id, s.predictability))
        .collect())
}

/// Every rule ID claimed by a `Rules:` doc comment under `root`.
pub(crate) fn claimed(root: &Path) -> Result<BTreeSet<String>> {
    let mut out = BTreeSet::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for e in entries.filter_map(std::result::Result::ok) {
            let p = e.path();
            if p.is_dir() {
                if p.file_name().is_some_and(|n| n == "target") {
                    continue;
                }
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                for line in fs::read_to_string(&p)?.lines() {
                    let Some(i) = line.find("Rules:") else {
                        continue;
                    };
                    for tok in line[i + 6..].split([',', ' ', '`', ';', '.']) {
                        let tok = tok.trim();
                        if is_rule_id(tok) {
                            out.insert(tok.to_owned());
                        }
                    }
                }
            }
        }
    }
    Ok(out)
}

/// Prints the coverage report.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn report(grammar: &Path, code: &Path) -> Result {
    let specs = specs(grammar)?;
    if specs.is_empty() {
        return Err(format!("no rules found in {}", grammar.display()).into());
    }
    let claimed = claimed(code)?;
    let known: BTreeSet<&str> = specs.iter().map(|s| s.id.as_str()).collect();

    let unknown: Vec<&String> = claimed
        .iter()
        .filter(|c| !known.contains(c.as_str()))
        .collect();

    let mut by_chapter: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    let mut by_pred: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for s in &specs {
        let hit = usize::from(claimed.contains(&s.id));
        let c = by_chapter.entry(s.chapter.as_str()).or_default();
        c.0 += hit;
        c.1 += 1;
        let p = by_pred.entry(s.predictability.as_str()).or_default();
        p.0 += hit;
        p.1 += 1;
    }

    let done: usize = by_chapter.values().map(|(d, _)| d).sum();
    println!("rule coverage: {done}/{} implemented", specs.len());
    println!("\nby chapter");
    for (ch, (d, t)) in &by_chapter {
        println!(
            "  {ch}  {d:>4}/{t:<4} {:>5.1}%",
            100.0 * f64::from(u32::try_from(*d).unwrap_or(0))
                / f64::from(u32::try_from(*t).unwrap_or(1))
        );
    }
    println!("\nby predictability");
    for (p, (d, t)) in &by_pred {
        println!("  {p:<12} {d:>4}/{t:<4}");
    }

    if !unknown.is_empty() {
        println!(
            "\ncode cites {} rule ID(s) that do not exist:",
            unknown.len()
        );
        for u in unknown.iter().take(20) {
            println!("  {u}");
        }
        return Err(format!("{} unknown rule ID(s) cited in code", unknown.len()).into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{is_rule_id, primary_token};

    #[test]
    fn recognises_ids() {
        assert!(is_rule_id("NOM-ACC-003"));
        assert!(!is_rule_id("NOM-ACC-"));
        assert!(!is_rule_id("nom-acc-003"));
    }

    #[test]
    fn reads_the_primary_token() {
        assert_eq!(primary_token(" `rule` (given stem)"), "rule");
        assert_eq!(primary_token(" `lexical`"), "lexical");
    }
}
