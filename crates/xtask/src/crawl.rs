//! Scores real crawled documents, to see what the ranker does with text nobody curated.
//!
//! Every other measurement of the score uses damage this project generated: known-good text put
//! through a rewrite whose label is therefore known. That answers whether the score notices the
//! damage we thought of, and nothing about what a crawl actually contains. This reads documents as
//! they came off the web and asks the only question that matters for ranking a training set: is
//! the text the score ranks lowest actually the text worth dropping.
//!
//! There is no ground truth here, so nothing is asserted. What it prints is the distribution, the
//! agreement with the crawl's own filter verdict, and the documents at both ends by name, because
//! a ranking is judged by reading its extremes.

use std::fs;
use std::path::Path;

use picur_morph_rank::{Quality, quality};

use crate::Result;

/// One crawled document, as much of it as this needs.
struct Doc {
    url: String,
    /// The crawl's own verdict, an independent opinion rather than a label.
    filter: String,
    text: String,
}

/// The value of a string field, for the flat one-line-per-document JSON the crawl ships.
///
/// Deliberately not a JSON parser: the fields wanted are strings at the top level of an object
/// written on one line, and pulling three of them out does not justify a dependency.
fn field(line: &str, key: &str) -> String {
    let pat = format!("\"{key}\":\"");
    let Some(start) = line.find(&pat).map(|i| i + pat.len()) else {
        return String::new();
    };
    let mut out = String::new();
    let mut chars = line[start..].chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => break,
            '\\' => match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('u') => {
                    let hex: String = chars.by_ref().take(4).collect();
                    if let Ok(n) = u32::from_str_radix(&hex, 16)
                        && let Some(c) = char::from_u32(n)
                    {
                        out.push(c);
                    }
                }
                Some(other) => out.push(other),
                None => break,
            },
            other => out.push(other),
        }
    }
    out
}

/// Scores every document in a crawl dump and reports what the ranking looks like.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn report(path: &Path, show: usize) -> Result {
    let body = fs::read_to_string(path)?;
    let docs: Vec<Doc> = body
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| Doc {
            url: field(l, "u"),
            filter: field(l, "filter"),
            text: field(l, "text"),
        })
        .collect();
    if docs.is_empty() {
        return Err(format!("no documents in {}", path.display()).into());
    }

    // The checker is a pure function behind a shared memo, so the documents split across threads.
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    let chunk = docs.len().div_ceil(threads);
    let mut scored: Vec<(f64, Quality, &Doc)> = std::thread::scope(|s| {
        let handles: Vec<_> = docs
            .chunks(chunk)
            .map(|part| {
                s.spawn(move || {
                    part.iter()
                        .map(|d| {
                            let q = quality(&d.text);
                            (q.score(), q, d)
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .filter_map(|h| h.join().ok())
            .flatten()
            .collect()
    });
    scored.sort_by(|a, b| a.0.total_cmp(&b.0));

    println!("{} documents from {}", scored.len(), path.display());
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let n = scored.len() as f64;

    println!("\ndecile   score");
    for d in 0..=10 {
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "an index into the sorted scores"
        )]
        let i = ((n - 1.0) * f64::from(d) / 10.0) as usize;
        println!("{:>4}%   {:.3}", d * 10, scored[i].0);
    }

    verdict_table(&docs, &scored);
    extremes(&scored, show);
    disagreements(&scored, show);
    Ok(())
}

/// How the score lines up with the crawl's own filter verdict.
///
/// Not a label: it is a different tool's opinion, and where the two disagree the documents
/// themselves decide which was right.
#[expect(clippy::print_stdout, reason = "CLI report")]
fn verdict_table(docs: &[Doc], scored: &[(f64, Quality, &Doc)]) {
    println!("\ncrawl verdict      docs   mean score   below 0.5");
    let mut verdicts: Vec<&str> = docs.iter().map(|d| d.filter.as_str()).collect();
    verdicts.sort_unstable();
    verdicts.dedup();
    for v in verdicts {
        let rows: Vec<&(f64, Quality, &Doc)> =
            scored.iter().filter(|(_, _, d)| d.filter == v).collect();
        if rows.is_empty() {
            continue;
        }
        #[expect(
            clippy::cast_precision_loss,
            reason = "counts are far below the f64 mantissa"
        )]
        let k = rows.len() as f64;
        let mean = rows.iter().map(|(s, ..)| s).sum::<f64>() / k;
        #[expect(
            clippy::cast_precision_loss,
            reason = "counts are far below the f64 mantissa"
        )]
        let low = rows.iter().filter(|(s, ..)| *s < 0.5).count() as f64;
        println!(
            "{v:<16} {:>6}   {mean:>10.3}   {:>8.1}%",
            rows.len(),
            low * 100.0 / k
        );
    }
}

/// Documents the crawl kept and this scored low.
///
/// Where two tools built on different principles disagree, one of them is wrong, and which one is
/// not decidable from the scores. Printing them is the only way to find out whether the text is
/// genuinely poor, genuinely fine, or something neither tool models.
#[expect(clippy::print_stdout, reason = "CLI report")]
fn disagreements(scored: &[(f64, Quality, &Doc)], show: usize) {
    println!("\n=== crawl kept it, we scored it below 0.5 ===");
    let rows = scored
        .iter()
        .filter(|(s, q, d)| *s < 0.5 && d.filter == "keep" && q.tokens >= 30)
        .take(show);
    for (score, q, d) in rows {
        let text: String = d
            .text
            .split_whitespace()
            .take(30)
            .collect::<Vec<_>>()
            .join(" ");
        println!(
            "\n  {score:.3}  tokens {:<6} ok {:.2} wrong {:.2} unk {:.2}\n  {}\n  {}",
            q.tokens,
            q.accepted,
            q.rejected,
            q.unknown,
            &d.url[..d.url.len().min(100)],
            &text[..text.len().min(300)]
        );
    }
}

/// The documents at both ends, by name, because a ranking is judged by reading its extremes.
#[expect(clippy::print_stdout, reason = "CLI report")]
fn extremes(scored: &[(f64, Quality, &Doc)], show: usize) {
    for (label, rows) in [
        ("lowest", scored.iter().take(show).collect::<Vec<_>>()),
        (
            "highest",
            scored.iter().rev().take(show).collect::<Vec<_>>(),
        ),
    ] {
        println!("\n=== {label} {show} ===");
        for (score, q, d) in rows {
            let text: String = d
                .text
                .split_whitespace()
                .take(22)
                .collect::<Vec<_>>()
                .join(" ");
            println!(
                "\n  {score:.3}  tokens {:<6} ok {:.2} wrong {:.2} unk {:.2}  [{}]\n  {}\n  {}",
                q.tokens,
                q.accepted,
                q.rejected,
                q.unknown,
                d.filter,
                &d.url[..d.url.len().min(100)],
                &text[..text.len().min(220)]
            );
        }
    }
}
