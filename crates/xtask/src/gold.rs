//! Measures the analyser against gold lemma annotation.
//!
//! Reads CoNLL-U (`.conllu`) and CoNLL-U Plus (`.conllup`) and asks, for each word token, whether
//! any reading the checker finds carries the annotated lemma. That is a stricter question than
//! `accept`: a word can be accepted on a reading whose lemma is wrong.
//!
//! ```text
//! cargo run -p xtask -- gold local/research-impl/ud_hungarian-szeged
//! cargo run -p xtask -- gold local/research-impl/nytud_nerkor/data
//! ```
//!
//! Both corpora are study-only (`CC BY-NC-SA 3.0` and `CC BY-SA 4.0`), so they live under `local/`
//! and nothing derived from them is committed beyond the aggregate figures.
//!
//! The annotation is not a ground truth to be matched blindly. Where it disagrees with the
//! rulebook the disagreement is reported, because it is as likely to be an annotation error as
//! ours: the report lists the commonest disagreements so each can be judged on its own.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use crate::Result;

/// Which columns of a row carry the form and the lemma, and whether column 0 is an ID.
#[derive(Clone, Copy)]
struct Layout {
    form: usize,
    lemma: usize,
    /// A second lemma column, where the file declares one. Reported separately: the two follow
    /// different conventions and neither is a ground truth.
    alt: Option<usize>,
    /// Whether the first column is a token ID, which is what marks a multiword range or an empty
    /// node. CoNLL-U always has one; a CoNLL-U Plus header may not declare one at all, and `NerKor`
    /// does not, where column 0 is a form that may contain `-` or `.`.
    has_id: bool,
    /// The universal tag, where the file carries one.
    upos: Option<usize>,
    /// The feature bundle, where the file carries one.
    feats: Option<usize>,
}

/// CoNLL-U: `ID FORM LEMMA UPOS ...`.
const CONLLU: Layout = Layout {
    form: 1,
    lemma: 2,
    alt: None,
    has_id: true,
    upos: Some(3),
    feats: Some(5),
};

/// Reads the layout from a CoNLL-U Plus `global.columns` header.
///
/// `LEMMA` is the primary. `NerKor` also ships `EMMORPH:LEMMA`, and the two are different
/// conventions rather than one being more dictionary-like than the other: emMorph keeps the
/// derivation of `uniós` and `közötti`, and reduces `első` to `egy` and `jól` to `jó`, which are
/// citation forms of their own under the Universal Dependencies guidelines the `LEMMA` column
/// follows. Both are measured, because the difference is a convention and not an error either way.
fn layout_from_header(line: &str) -> Option<Layout> {
    let (_, declared) = line.split_once('=')?;
    let names: Vec<&str> = declared.split_whitespace().collect();
    let form = names.iter().position(|n| *n == "FORM")?;
    let at = |want: &str| names.iter().position(|n| *n == want);
    let lemma = at("LEMMA").or_else(|| at("EMMORPH:LEMMA"))?;
    Some(Layout {
        form,
        lemma,
        alt: at("EMMORPH:LEMMA").filter(|i| *i != lemma),
        has_id: names.first() == Some(&"ID"),
        upos: at("UPOS"),
        feats: at("FEATS"),
    })
}

/// What the evaluation found.
#[derive(Default)]
struct Tally {
    tokens: u64,
    analysed: u64,
    agreed: u64,
    /// Agreement with the second lemma column, and how many tokens had one.
    alt_agreed: u64,
    alt_tokens: u64,
    /// Tokens the file annotates with a tag, and how many we matched.
    upos_tokens: u64,
    upos_agreed: u64,
    /// Tokens the file annotates with features, and how many we matched exactly.
    feats_tokens: u64,
    feats_agreed: u64,
    /// Tokens where one reading matched lemma, tag and features at once.
    whole_tokens: u64,
    whole_agreed: u64,
    /// Readings offered in total, against which the agreement above must be read: matching on
    /// *any* reading rewards ambiguity, so the count of readings is part of the result.
    readings: u64,
    /// Tokens offering exactly one reading, and how many of those carried the whole annotation.
    single_tokens: u64,
    single_agreed: u64,
    /// Disagreements, by (gold lemma, the lemmas we offered).
    misses: BTreeMap<(String, String), u64>,
    /// Tag disagreements, by (gold tag, the tags we offered).
    upos_misses: BTreeMap<(String, String), u64>,
    /// Feature disagreements, by (gold features, the bundle we offered).
    feats_misses: BTreeMap<(String, String), u64>,
    /// Tokens with no reading at all, by form.
    unanalysed: BTreeMap<String, u64>,
}

/// One reading of a form, in the shape the annotation is written in.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Reading {
    lemma: String,
    upos: &'static str,
    feats: String,
}

/// The distinct values a set of readings offered, for a disagreement line.
fn joined(values: impl Iterator<Item = String>) -> String {
    let mut out: Vec<String> = values.collect();
    out.sort();
    out.dedup();
    // Separated by a slash: the readings themselves contain `|`, which is UD's own separator.
    out.join(" / ")
}

/// Whether a token is a word rather than punctuation or a number.
fn is_word(form: &str) -> bool {
    form.chars().any(char::is_alphabetic) && !form.contains(['.', '/', '@'])
}

/// Evaluates one file into `t`, memoising by form.
///
/// Running text repeats its words heavily and a reading is a pure function of the token, so the
/// cache is what makes a million-token corpus practical. `check` memoises inside the crate but
/// `analyses` does not, because it hands back owned readings.
fn file(path: &Path, t: &mut Tally, seen: &mut HashMap<String, Vec<Reading>>) -> Result {
    let text = fs::read_to_string(path)?;
    let mut layout = if path.extension().is_some_and(|e| e == "conllup") {
        None
    } else {
        Some(CONLLU)
    };
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("# global.columns") {
            layout = layout_from_header(rest);
            continue;
        }
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let Some(layout) = layout else { continue };
        let cols: Vec<&str> = line.split('\t').collect();
        let (Some(form), Some(gold)) = (cols.get(layout.form), cols.get(layout.lemma)) else {
            continue;
        };
        let range = layout.has_id && cols.first().is_some_and(|id| id.contains(['-', '.']));
        if range || *gold == "_" {
            continue;
        }
        if !is_word(form) {
            continue;
        }
        t.tokens += 1;
        // Kept as written, compared case-folded: a lowercased stored lemma makes the report read
        // `Megyei -> megye`, which looks like a casing bug rather than the `-i` disagreement.
        let readings = seen.entry((*form).to_owned()).or_insert_with(|| {
            let mut out: Vec<Reading> = picur_morph_spell::analyses(form)
                .iter()
                .map(|a| {
                    let ud = picur_morph_core::project(a);
                    Reading {
                        lemma: a.lemma.to_string(),
                        upos: ud.upos,
                        feats: ud.feats(),
                    }
                })
                .collect();
            out.sort();
            out.dedup();
            out
        });
        if readings.is_empty() {
            *t.unanalysed.entry((*form).to_owned()).or_default() += 1;
            continue;
        }
        t.analysed += 1;
        let agrees = |want: &str| {
            let want = want.to_lowercase();
            readings.iter().any(|r| r.lemma.to_lowercase() == want)
        };
        let gold_upos = layout.upos.and_then(|i| cols.get(i)).filter(|u| **u != "_");
        let gold_feats = layout.feats.and_then(|i| cols.get(i));
        columns(t, readings, gold, gold_upos.copied(), gold_feats.copied());
        if let Some(alt) = layout.alt.and_then(|i| cols.get(i)).filter(|a| **a != "_") {
            t.alt_tokens += 1;
            if agrees(alt) {
                t.alt_agreed += 1;
            }
        }
        let gold_lower = gold.to_lowercase();
        if readings
            .iter()
            .any(|r| r.lemma.to_lowercase() == gold_lower)
        {
            t.agreed += 1;
        } else {
            let offered = joined(readings.iter().map(|r| r.lemma.clone()));
            *t.misses.entry(((*gold).to_owned(), offered)).or_default() += 1;
        }
    }
    Ok(())
}

/// Scores the tag and feature columns of one token, and whether a single reading carries all of it.
fn columns(
    t: &mut Tally,
    readings: &[Reading],
    gold: &str,
    gold_upos: Option<&str>,
    gold_feats: Option<&str>,
) {
    if let Some(want) = gold_upos {
        t.upos_tokens += 1;
        if readings.iter().any(|r| r.upos == want) {
            t.upos_agreed += 1;
        } else {
            let offered = joined(readings.iter().map(|r| r.upos.to_owned()));
            *t.upos_misses.entry((want.to_owned(), offered)).or_default() += 1;
        }
    }
    if let Some(want) = gold_feats {
        t.feats_tokens += 1;
        if readings.iter().any(|r| r.feats == want) {
            t.feats_agreed += 1;
        } else {
            let offered = joined(readings.iter().map(|r| r.feats.clone()));
            *t.feats_misses
                .entry((want.to_owned(), offered))
                .or_default() += 1;
        }
    }
    // The question a caller building an annotated corpus actually has: does one reading carry the
    // whole annotation, rather than three columns each matched by a different reading.
    if let (Some(u), Some(f)) = (gold_upos, gold_feats) {
        t.whole_tokens += 1;
        let want = gold.to_lowercase();
        let whole = |r: &Reading| r.lemma.to_lowercase() == want && r.upos == u && r.feats == f;
        if readings.iter().any(whole) {
            t.whole_agreed += 1;
        }
        // Agreement where there was nothing to choose between. The difference between this and
        // the figure above is how much of the agreement is owed to offering several readings.
        if readings.len() == 1 {
            t.single_tokens += 1;
            if readings.iter().any(whole) {
                t.single_agreed += 1;
            }
        }
    }
    t.readings += readings.len() as u64;
}

/// Every `.conllu` and `.conllup` file under `root`, or `root` itself.
fn files(root: &Path, out: &mut Vec<std::path::PathBuf>) -> Result {
    if root.is_file() {
        out.push(root.to_owned());
        return Ok(());
    }
    let mut entries: Vec<_> = fs::read_dir(root)?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            files(&path, out)?;
        } else if path
            .extension()
            .is_some_and(|e| e == "conllu" || e == "conllup")
        {
            out.push(path);
        }
    }
    Ok(())
}

/// The tag, feature and whole-annotation lines of the summary.
#[expect(clippy::print_stdout, reason = "CLI report")]
fn column_summary(t: &Tally) {
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let share = |n: u64, of: u64| {
        if of == 0 {
            0.0
        } else {
            n as f64 * 100.0 / of as f64
        }
    };
    if t.upos_tokens > 0 {
        println!(
            "  tag agrees      {:>8}  {:>6.2}%   of the {} tokens carrying one",
            t.upos_agreed,
            share(t.upos_agreed, t.upos_tokens),
            t.upos_tokens
        );
    }
    if t.feats_tokens > 0 {
        println!(
            "  features agree  {:>8}  {:>6.2}%   of the {} tokens carrying them",
            t.feats_agreed,
            share(t.feats_agreed, t.feats_tokens),
            t.feats_tokens
        );
    }
    if t.whole_tokens > 0 {
        println!(
            "  all three       {:>8}  {:>6.2}%   some reading carrying the whole annotation",
            t.whole_agreed,
            share(t.whole_agreed, t.whole_tokens)
        );
        println!(
            "  unambiguous     {:>8}  {:>6.2}%   of the {} tokens with only one reading",
            t.single_agreed,
            share(t.single_agreed, t.single_tokens),
            t.single_tokens
        );
        #[expect(
            clippy::cast_precision_loss,
            reason = "counts are far below the f64 mantissa"
        )]
        let per = t.readings as f64 / t.analysed.max(1) as f64;
        println!("  readings offered            {per:>6.2}    per analysed token");
    }
}

/// The commonest tag and feature disagreements, for judging one by one.
#[expect(clippy::print_stdout, reason = "CLI report")]
fn disagreement_tables(
    upos: BTreeMap<(String, String), u64>,
    feats: BTreeMap<(String, String), u64>,
) {
    for (title, table) in [("tag", upos), ("feature", feats)] {
        let mut rows: Vec<_> = table.into_iter().collect();
        rows.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
        println!("\ncommonest {title} disagreements (gold -> ours)");
        for ((gold, ours), n) in rows.iter().take(20) {
            println!("  {n:>6}  {gold} -> {ours}");
        }
    }
}

/// Reads the corpus at `root` and reports lemma agreement.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn report(root: &Path) -> Result {
    let mut paths = Vec::new();
    files(root, &mut paths)?;
    if paths.is_empty() {
        return Err(format!("no conllu or conllup files under {}", root.display()).into());
    }
    let mut t = Tally::default();
    let mut seen = HashMap::new();
    for path in &paths {
        file(path, &mut t, &mut seen)?;
    }
    if t.tokens == 0 {
        return Err("no word tokens found".into());
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let pct = |n: u64| n as f64 * 100.0 / t.tokens as f64;
    println!(
        "{} word tokens from {} files under {}",
        t.tokens,
        paths.len(),
        root.display()
    );
    println!(
        "  analysed        {:>8}  {:>6.2}%",
        t.analysed,
        pct(t.analysed)
    );
    println!(
        "  lemma agrees    {:>8}  {:>6.2}%   of all tokens",
        t.agreed,
        pct(t.agreed)
    );
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let of_analysed = if t.analysed == 0 {
        0.0
    } else {
        t.agreed as f64 * 100.0 / t.analysed as f64
    };
    println!("                            {of_analysed:>6.2}%   of the analysed");
    column_summary(&t);

    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    if t.alt_tokens > 0 {
        println!(
            "  emMorph agrees  {:>8}  {:>6.2}%   of the {} analysed tokens carrying it",
            t.alt_agreed,
            t.alt_agreed as f64 * 100.0 / t.alt_tokens as f64,
            t.alt_tokens
        );
    }

    let mut misses: Vec<_> = t.misses.into_iter().collect();
    misses.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
    println!("\ncommonest lemma disagreements (gold -> ours)");
    for ((gold, ours), n) in misses.iter().take(30) {
        println!("  {n:>6}  {gold} -> {ours}");
    }

    disagreement_tables(t.upos_misses, t.feats_misses);

    let mut unknown: Vec<_> = t.unanalysed.into_iter().collect();
    unknown.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
    println!("\ncommonest tokens with no reading");
    for (form, n) in unknown.iter().take(30) {
        println!("  {n:>6}  {form}");
    }
    Ok(())
}
