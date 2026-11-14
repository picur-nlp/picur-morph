//! Measures whether the document score separates good Hungarian from damaged Hungarian.
//!
//! The positives are real edited paragraphs. The negatives are the same paragraphs put through the
//! damage the rulebook inventories: code pages read wrong (`WS-ENC-006`), accents stripped
//! (`REP-DIAC-001`) and the shape confusions of scanning (`REP-OCR-001`). Damaging known-good text
//! is the only way to get a negative set with a known label, and `REP-NOISE-008` says the noise
//! generator shares the repair's inventory, which is what this does.
//!
//! What it reports is separation, not accuracy: the score of a class is only useful if it lands
//! somewhere the other classes do not. A threshold is chosen after the fact, at the point that
//! misclassifies the fewest paragraphs, and the error there is the honest headline.

use std::fs;
use std::path::Path;

use picur_morph_rank::{MIN_TOKENS, quality};
use picur_morph_repair::OCR_CONFUSIONS;
use picur_morph_repair::encoding::{Page, mojibake};

use crate::Result;

/// Paragraphs of at least [`MIN_TOKENS`] words, built by joining consecutive lines.
fn paragraphs(text: &str, want: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if !buf.is_empty() {
            buf.push(' ');
        }
        buf.push_str(line.trim());
        if buf.split_whitespace().count() >= MIN_TOKENS.max(40) {
            out.push(std::mem::take(&mut buf));
            if out.len() == want {
                return out;
            }
        }
    }
    out
}

/// The accent-loss half of the OCR inventory, applied to every letter it names.
fn strip_accents(text: &str) -> String {
    text.chars()
        .map(|c| {
            let s = c.to_string();
            OCR_CONFUSIONS
                .iter()
                .find(|(from, to)| *from == s && to.len() == 1 && to.is_ascii())
                .and_then(|(_, to)| to.chars().next())
                .unwrap_or(c)
        })
        .collect()
}

/// Every second letter that has a shape confusion, swapped for it.
fn ocr_damage(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut n = 0usize;
    let mut rest = text;
    while let Some(c) = rest.chars().next() {
        let s = c.to_string();
        let hit = OCR_CONFUSIONS.iter().find(|(from, _)| *from == s);
        match hit {
            Some((_, to)) if n.is_multiple_of(2) => out.push_str(to),
            _ => out.push(c),
        }
        if hit.is_some() {
            n += 1;
        }
        rest = &rest[c.len_utf8()..];
    }
    out
}

/// A named way of damaging a paragraph.
type Damage = (&'static str, fn(&str) -> String);

/// One labelled class of paragraphs.
struct Class {
    name: &'static str,
    scores: Vec<f64>,
}

impl Class {
    fn mean(&self) -> f64 {
        if self.scores.is_empty() {
            return 0.0;
        }
        #[expect(
            clippy::cast_precision_loss,
            reason = "counts are far below the f64 mantissa"
        )]
        let n = self.scores.len() as f64;
        self.scores.iter().sum::<f64>() / n
    }

    fn quantile(&self, q: f64) -> f64 {
        if self.scores.is_empty() {
            return 0.0;
        }
        let mut v = self.scores.clone();
        v.sort_by(f64::total_cmp);
        #[expect(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "an index into a vector of scores"
        )]
        let i = ((v.len() - 1) as f64 * q).round() as usize;
        v[i]
    }
}

/// Scores each file as one document and prints the parts, worst first.
///
/// The product-facing view: what a caller building a training set sees per document.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn score_files(paths: &[&str]) -> Result {
    let mut rows: Vec<(f64, String, picur_morph_rank::Quality)> = Vec::new();
    for p in paths {
        let text = fs::read_to_string(p)?;
        let q = quality(&text);
        let name = Path::new(p)
            .file_stem()
            .map_or_else(|| (*p).to_string(), |s| s.to_string_lossy().into_owned());
        rows.push((q.score(), name, q));
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));
    println!(
        "{:<34} {:>6} {:>7} {:>6} {:>6} {:>6} {:>5} {:>6}",
        "document", "score", "tokens", "ok", "wrong", "unk", "strip", "moji"
    );
    for (score, name, q) in &rows {
        println!(
            "{:<34} {score:>6.3} {:>7} {:>6.3} {:>6.3} {:>6.3} {:>5} {:>6.3}",
            name, q.tokens, q.accepted, q.rejected, q.unknown, q.accent_stripped, q.mojibake
        );
    }
    Ok(())
}

/// Reads `path` as text and reports how the score separates clean from damaged.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn report(path: &Path, want: usize) -> Result {
    let text = fs::read_to_string(path)?;
    let clean = paragraphs(&text, want);
    if clean.is_empty() {
        return Err(format!("no paragraphs of {MIN_TOKENS}+ words in {}", path.display()).into());
    }
    println!("{} paragraphs from {}", clean.len(), path.display());

    let damaged: [Damage; 3] = [
        ("accents stripped", |t| strip_accents(t)),
        ("mojibake (cp1252)", |t| mojibake(t, Page::Windows1252)),
        ("OCR shapes", |t| ocr_damage(t)),
    ];

    let mut texts: Vec<Vec<String>> = vec![clean.clone()];
    let mut classes = vec![Class {
        name: "clean",
        scores: clean.iter().map(|p| quality(p).score()).collect(),
    }];
    for (name, damage) in damaged {
        let variants: Vec<String> = clean.iter().map(|p| damage(p)).collect();
        classes.push(Class {
            name,
            scores: variants.iter().map(|p| quality(p).score()).collect(),
        });
        texts.push(variants);
    }

    println!(
        "\n{:<20} {:>7} {:>7} {:>7} {:>7}",
        "class", "mean", "p10", "p50", "p90"
    );
    for c in &classes {
        println!(
            "{:<20} {:>7.3} {:>7.3} {:>7.3} {:>7.3}",
            c.name,
            c.mean(),
            c.quantile(0.10),
            c.quantile(0.50),
            c.quantile(0.90)
        );
    }

    let positives = &classes[0].scores;
    let negatives: Vec<f64> = classes[1..].iter().flat_map(|c| c.scores.clone()).collect();
    let mut best = (0.0f64, usize::MAX);
    for step in 0..=100 {
        let t = f64::from(step) / 100.0;
        let missed = positives.iter().filter(|s| **s < t).count()
            + negatives.iter().filter(|s| **s >= t).count();
        if missed < best.1 {
            best = (t, missed);
        }
    }
    let total = positives.len() + negatives.len();
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let rate = best.1 as f64 * 100.0 / total as f64;
    println!(
        "\nbest threshold {:.2}: {} of {total} paragraphs on the wrong side ({rate:.2}%)",
        best.0, best.1
    );
    // A separation figure needs its exceptions named, to show whether the remainder is a defect,
    // an artefact of the damage, or genuinely borderline text.
    for (ci, c) in classes.iter().enumerate() {
        for (i, s) in c.scores.iter().enumerate() {
            let wrong = if ci == 0 { *s < best.0 } else { *s >= best.0 };
            if !wrong {
                continue;
            }
            let q = quality(&texts[ci][i]);
            println!(
                "\n  [{}] score {s:.3} tokens {} accepted {:.3} rejected {:.3} unknown {:.3} \
                 stripped {} mojibake {:.3}",
                c.name, q.tokens, q.accepted, q.rejected, q.unknown, q.accent_stripped, q.mojibake
            );
            println!("    {}", &texts[ci][i][..texts[ci][i].len().min(300)]);
        }
    }
    Ok(())
}
