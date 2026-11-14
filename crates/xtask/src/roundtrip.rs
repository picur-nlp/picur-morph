//! The two-direction property over the checker's own search space: every form the engine
//! generates for a lemma must analyse back to that lemma under the stack that produced it.
//!
//! ```text
//! cargo xtask roundtrip [<lemmas per class>] [<pos>]
//! ```
//!
//! Lemmas are sampled by frequency so the forms are the ones running text has. A failure is
//! classed by what analysis did return: nothing, the lemma under another stack only, or another
//! lemma only.

use std::collections::BTreeMap;

use picur_morph_core::{Morph, Verdict, analyse, forms};
use picur_morph_lexicon::{Entry, Pos, frequency, lexicon};
use picur_morph_spell::stacks;

/// What came back for a generated form that did not round-trip.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Miss {
    /// No reading at all.
    Unread,
    /// The lemma, but never under the generating stack.
    OtherStack,
    /// Only other lemmas.
    OtherLemma,
}

/// The classes the audit samples.
const CLASSES: [Pos; 4] = [Pos::Noun, Pos::Adjective, Pos::Verb, Pos::Adverb];

/// Parses the arguments and runs the audit.
pub(crate) fn run(per_class: Option<&str>, only: Option<&str>) -> crate::Result {
    let n = per_class.map_or(Ok(40), str::parse)?;
    let only = only.map(pos).transpose()?;
    report(n, only);
    Ok(())
}

/// A class by its command-line name.
fn pos(name: &str) -> crate::Result<Pos> {
    CLASSES
        .into_iter()
        .find(|p| format!("{p:?}").eq_ignore_ascii_case(name))
        .ok_or_else(|| format!("unknown class {name}; one of {CLASSES:?}").into())
}

/// The `n` commonest lemmas of `pos`, ties broken alphabetically.
fn sample(pos: Pos, n: usize) -> Vec<&'static Entry> {
    let mut entries: Vec<&Entry> = lexicon().iter().filter(|e| e.pos == pos).collect();
    entries.sort_by(|a, b| {
        frequency(&b.lemma)
            .cmp(&frequency(&a.lemma))
            .then_with(|| a.lemma.cmp(&b.lemma))
    });
    entries.truncate(n);
    entries
}

/// Classifies the verdict for `surface` generated from `entry` under `stack`.
fn judge(entry: &Entry, stack: &[Morph], surface: &str) -> Option<Miss> {
    let Verdict::Known(readings) = analyse(surface, stacks()) else {
        return Some(Miss::Unread);
    };
    if readings
        .iter()
        .any(|r| r.lemma == entry.lemma && r.pos == entry.pos && r.stack == stack)
    {
        return None;
    }
    if readings.iter().any(|r| r.lemma == entry.lemma) {
        return Some(Miss::OtherStack);
    }
    Some(Miss::OtherLemma)
}

/// Runs the audit and prints the rates, then the misses grouped by stack.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn report(per_class: usize, only: Option<Pos>) {
    let classes: Vec<Pos> = only.map_or_else(|| CLASSES.to_vec(), |p| vec![p]);
    let mut by_stack: BTreeMap<(Miss, String), Vec<String>> = BTreeMap::new();
    for pos in classes {
        let mut generated = 0usize;
        let mut misses = [0usize; 3];
        for entry in sample(pos, per_class) {
            for stack in stacks().iter() {
                for surface in forms(entry, stack) {
                    generated += 1;
                    if let Some(miss) = judge(entry, stack, &surface) {
                        misses[miss as usize] += 1;
                        by_stack
                            .entry((miss, format!("{stack:?}")))
                            .or_default()
                            .push(format!("{surface} <{}>", entry.lemma));
                    }
                }
            }
        }
        let total: usize = misses.iter().sum();
        let ok = generated - total;
        #[expect(
            clippy::cast_precision_loss,
            reason = "counts far below the f64 mantissa"
        )]
        let rate = 100.0 * ok as f64 / generated.max(1) as f64;
        println!(
            "{pos:?}: {ok} of {generated} round-trip ({rate:.2}%); unread {}, other stack {}, other lemma {}",
            misses[0], misses[1], misses[2]
        );
    }
    for ((miss, stack), examples) in &by_stack {
        println!("\n{miss:?} {stack} ({})", examples.len());
        for e in examples.iter().take(6) {
            println!("    {e}");
        }
    }
}
