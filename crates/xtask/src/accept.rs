//! Measures what the checker accepts over a frequency-ranked word list.
//!
//! The headline quality number, and the one that moves when a stem class is wrong. Input is a
//! `form<TAB>frequency` list, most frequent first, such as the commonest types of a corpus:
//!
//! ```text
//! sqlite3 local/cache/webcorpus2.sqlite \
//!   "select form, sum(freq) f from f group by form order by f desc limit 20000;" \
//!   -separator '\t' > top20k.tsv
//! cargo run -p xtask -- accept top20k.tsv
//! ```
//!
//! A word-frequency corpus is normally case-folded, and a lowercase proper name or acronym is a
//! real spelling error in Hungarian, so rejecting `európa` and `mti` is correct rather than a miss.
//! Those are counted separately instead of being folded into either side, because counting them as
//! misses understates the checker and accepting them would be wrong.

use std::fs;
use std::path::Path;

use picur_morph_core::{Verdict, analyse};
use picur_morph_lexicon::lexicon;
use picur_morph_repair::{Mode, repair_word, restore_accents};
use picur_morph_spell::{Status, check};

use crate::Result;

/// Whether `word` is correct once its case is restored.
///
/// Two restorations, because a case-folded corpus flattens both: a proper name loses its initial
/// capital (`európa`) and an acronym loses all of them (`mti`, `elte`).
fn correct_when_cased(word: &str) -> bool {
    let mut chars = word.chars();
    let title = chars.next().map_or_else(String::new, |f| {
        f.to_uppercase().collect::<String>() + chars.as_str()
    });
    check(&title) == Status::Ok || check(&word.to_uppercase()) == Status::Ok
}

/// What the measurement found.
#[derive(Default)]
struct Tally {
    types: u64,
    ok: u64,
    unknown: u64,
    wrong: u64,
    /// Rejected, but correct once the case is restored: a case-folded name or acronym.
    case_only: u64,
    weight: f64,
    weight_ok: f64,
    weight_case_only: f64,
    /// Unknown, but correct once the case is restored: a case-folded name we did not flag.
    unknown_cased: u64,
    /// Rejected and not a case-folded name, most frequent first.
    rejected: Vec<(String, u64)>,
    /// Accepted, but only because it splits into listed members.
    novel_compounds: Vec<(String, u64)>,
    /// Neither accepted nor rejected: well formed, but the stem is not in the lexicon.
    unknown_forms: Vec<(String, u64)>,
}

/// Whether the word is accepted without any listed lemma behind it.
///
/// Hungarian compounding is productive, so an unlisted compound is a word the checker should
/// accept. But the admissibility test is a three-letter floor plus lexicon membership, which also
/// admits `col` plus `orr` for `colorr` and `ffi` plus `fa` for `ffifa`. The share of accepted
/// types that rest on this path alone is the exposure of the headline figure: they are accepted
/// without the lexicon ever having seen the word or a lemma it inflects.
///
/// The test is the lemma, not the presence of a compound split: `kellett` and `esetében` also
/// split into listed members, but they are accepted as inflections of `kell` and `eset`, which is
/// a different thing entirely.
fn has_no_listed_lemma(word: &str, stacks: &picur_morph_core::Stacks) -> bool {
    let lower = word.to_lowercase();
    if !lexicon().get(word).is_empty() || !lexicon().get(&lower).is_empty() {
        return false;
    }
    match analyse(&lower, stacks) {
        Verdict::Known(readings) => readings.iter().all(|r| lexicon().get(&r.lemma).is_empty()),
        _ => false,
    }
}

/// Prints the headline acceptance rates.
#[expect(clippy::print_stdout, reason = "CLI report")]
fn report_rates(t: &Tally, path: &Path) {
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let pct = |n: u64| n as f64 * 100.0 / t.types as f64;
    let wpct = |w: f64| w * 100.0 / t.weight;
    println!("{} types from {}", t.types, path.display());
    println!(
        "  accepted          {:>6}  {:>6.2}%   weighted {:>6.2}%",
        t.ok,
        pct(t.ok),
        wpct(t.weight_ok)
    );
    println!(
        "  unknown           {:>6}  {:>6.2}%   not flagged, no analysis",
        t.unknown,
        pct(t.unknown)
    );
    println!(
        "    case-folded     {:>6}  {:>6.2}%   name or acronym, not flagged either",
        t.unknown_cased,
        pct(t.unknown_cased)
    );
    println!("  rejected          {:>6}  {:>6.2}%", t.wrong, pct(t.wrong));
    println!(
        "    case-folded     {:>6}  {:>6.2}%   name or acronym, a correct rejection",
        t.case_only,
        pct(t.case_only)
    );
    println!(
        "  accepted or case  {:>6}  {:>6.2}%   weighted {:>6.2}%",
        t.ok + t.case_only,
        pct(t.ok + t.case_only),
        wpct(t.weight_ok + t.weight_case_only)
    );
}

/// Reports the share of accepted types with no lexicon entry behind them.
#[expect(clippy::print_stdout, reason = "CLI report")]
fn report_exposure(accepted: u64, unlisted: &[(String, u64)]) {
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let share = if accepted == 0 {
        0.0
    } else {
        unlisted.len() as f64 * 100.0 / accepted as f64
    };
    println!(
        "    no listed lemma {:>6}  {:>6.2}%   of the accepted, no lexicon entry behind it",
        unlisted.len(),
        share
    );
    println!("\ncommonest types accepted with no listed lemma behind them");
    for (form, freq) in unlisted.iter().take(25) {
        println!("  {freq:>10}  {form}");
    }
}

/// Lists the rejections, split by whether the repair layer explains them.
///
/// A rejection is worse than an unknown, because it tells a user that a correct word is wrong.
/// But most rejections are right: the corpus carries markup residue (`amp`, `title`, `lt`),
/// English, and Hungarian with its accents stripped (`igy`, `ugy`). A rejection a restoration
/// explains is a correct one with a correction attached, so it is counted apart; what is left is
/// the work queue. Without the split the bucket read as a defect list when it is mostly the
/// checker doing its job.
///
/// The three buckets are not interchangeable. A repair `repair_word` returns is licensed by the
/// token itself. A restoration is not: `REP-DIAC-001` licenses it from the segment, because `to`,
/// `for` and `all` restore to real Hungarian words and a lone token cannot say whether it is
/// Hungarian at all. Those are reported as what a restoration *would* propose, not as corrections
/// this layer would make, which is why they are counted apart instead of being added to the first
/// bucket.
#[expect(clippy::print_stdout, reason = "CLI report")]
fn report_rejections(rejected: &[(String, u64)]) {
    let mut repaired: Vec<(&String, u64, String)> = Vec::new();
    let mut restorable: Vec<(&String, u64, String)> = Vec::new();
    let mut unexplained: Vec<(&String, u64)> = Vec::new();
    for (form, freq) in rejected {
        if let Some((fixed, _)) = repair_word(form, Mode::Modernise) {
            repaired.push((form, *freq, fixed));
        } else if let Some(fixed) = restore_accents(form) {
            restorable.push((form, *freq, fixed));
        } else {
            unexplained.push((form, *freq));
        }
    }
    println!(
        "\n  repaired on the token   {:>6}   a misspelling we also correct",
        repaired.len()
    );
    println!(
        "  a restoration would     {:>6}   needs the segment (REP-DIAC-001), not a correction",
        restorable.len()
    );
    println!(
        "  unexplained             {:>6}   the work queue",
        unexplained.len()
    );
    println!("\ncommonest rejections repaired on the token alone");
    for (form, freq, fixed) in repaired.iter().take(15) {
        println!("  {freq:>10}  {form} -> {fixed}");
    }
    println!("\ncommonest rejections a restoration would rewrite, licensed only by the segment");
    for (form, freq, fixed) in restorable.iter().take(15) {
        println!("  {freq:>10}  {form} -> {fixed}");
    }
    println!("\ncommonest rejections with no repair");
    for (form, freq) in unexplained.iter().take(40) {
        println!("  {freq:>10}  {form}");
    }
}

/// Reads the list at `path` and reports the acceptance rates.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn report(path: &Path) -> Result {
    let text = fs::read_to_string(path)?;
    let shapes = picur_morph_core::Stacks::from(crate::eval::stacks());
    let mut t = Tally::default();
    for line in text.lines() {
        let mut fields = line.split('\t');
        let Some(form) = fields.next().filter(|f| !f.is_empty()) else {
            continue;
        };
        let freq: f64 = fields
            .next()
            .ok_or_else(|| format!("no count on line {line:?}"))?
            .parse()?;
        t.types += 1;
        t.weight += freq;
        match check(form) {
            Status::Ok => {
                t.ok += 1;
                t.weight_ok += freq;
                if has_no_listed_lemma(form, &shapes) {
                    #[expect(
                        clippy::cast_possible_truncation,
                        clippy::cast_sign_loss,
                        reason = "a corpus frequency is a small positive integer"
                    )]
                    t.novel_compounds.push((form.to_owned(), freq as u64));
                }
            }
            Status::Wrong => {
                t.wrong += 1;
                if correct_when_cased(form) {
                    t.case_only += 1;
                    t.weight_case_only += freq;
                } else {
                    #[expect(
                        clippy::cast_possible_truncation,
                        clippy::cast_sign_loss,
                        reason = "a corpus frequency is a small positive integer"
                    )]
                    t.rejected.push((form.to_owned(), freq as u64));
                }
            }
            _ => {
                t.unknown += 1;
                // A case-folded name lands here rather than in the rejected bucket, because an
                // unlisted stem is well formed. It is counted apart from the rejected ones and
                // *not* folded into the headline: we did not flag it, so it is a miss in the
                // other direction rather than a correct rejection. Reporting it is what shows
                // that the unknown bucket is mostly names and not missing vocabulary.
                if correct_when_cased(form) {
                    t.unknown_cased += 1;
                } else {
                    #[expect(
                        clippy::cast_possible_truncation,
                        clippy::cast_sign_loss,
                        reason = "a corpus frequency is a small positive integer"
                    )]
                    t.unknown_forms.push((form.to_owned(), freq as u64));
                }
            }
        }
    }
    if t.types == 0 {
        return Err(format!("no rows in {}", path.display()).into());
    }

    report_rates(&t, path);
    report_exposure(t.ok, &t.novel_compounds);
    report_rejections(&t.rejected);
    // The unknown bucket is the other work queue: a word we neither accept nor flag, because the
    // form is well made but the stem is unlisted. A neologism or an unlisted name belongs here; a
    // common Hungarian word does not.
    println!("\ncommonest forms with no listed stem");
    for (form, freq) in t.unknown_forms.iter().take(40) {
        println!("  {freq:>10}  {form}");
    }
    println!("\ncommonest rejections that case does not explain");
    for (form, freq) in t.rejected.iter().take(700) {
        println!("  {freq:>10}  {form}");
    }
    Ok(())
}
