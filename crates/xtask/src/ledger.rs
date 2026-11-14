//! One line per rule: what the engine does about it.
//!
//! Coverage counts citations; the ledger counts behaviour. A rule is covered when its goldens
//! reproduce and its starred forms are rejected, whether or not any code cites it, and the rules
//! with nothing to measure are listed so that they can be given goldens or a reason.

use std::collections::BTreeMap;
use std::path::Path;

use crate::Result;
use crate::eval::RuleScore;

/// What the ledger says about one rule.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Status {
    /// Every golden reproduced, every starred form rejected, every example form accepted.
    Covered,
    /// Some golden missed or some starred form accepted.
    Partial,
    /// Only goldens set aside (lexical, refused), all reproduced anyway.
    Aside,
    /// Only goldens set aside, some not reproduced.
    AsideMissed,
    /// Nothing to measure: no golden, no starred form.
    Unmeasured,
}

impl Status {
    const fn label(self) -> &'static str {
        match self {
            Self::Covered => "covered",
            Self::Partial => "partial",
            Self::Aside => "aside",
            Self::AsideMissed => "aside-missed",
            Self::Unmeasured => "unmeasured",
        }
    }
}

fn status(s: RuleScore) -> Status {
    let measured = s.goldens.1 + s.rejects.1 + s.accepts.1 + s.silences.1;
    if measured > 0 {
        if s.goldens.0 == s.goldens.1
            && s.rejects.0 == s.rejects.1
            && s.accepts.0 == s.accepts.1
            && s.silences.0 == s.silences.1
        {
            Status::Covered
        } else {
            Status::Partial
        }
    } else if s.excluded.1 > 0 {
        if s.excluded.0 == s.excluded.1 {
            Status::Aside
        } else {
            Status::AsideMissed
        }
    } else {
        Status::Unmeasured
    }
}

/// Prints the ledger, every rule or one status.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn report(grammar: &Path, code: &Path, only: Option<&str>) -> Result {
    let specs = crate::coverage::specs(grammar)?;
    let claimed = crate::coverage::claimed(code)?;
    let scores = crate::eval::rule_scores(&grammar.join("goldens.ron"))?;
    let mut counts: BTreeMap<(String, Status), usize> = BTreeMap::new();
    let mut totals: BTreeMap<Status, usize> = BTreeMap::new();
    for spec in &specs {
        let s = scores.get(&spec.id).copied().unwrap_or_default();
        let st = status(s);
        *counts.entry((spec.chapter.clone(), st)).or_default() += 1;
        *totals.entry(st).or_default() += 1;
        if only.is_none_or(|o| o == st.label()) {
            println!(
                "{:<16} {} {:<9} {:<13} cited={} goldens={}/{} aside={}/{} starred={}/{} examples={}/{} texts={}/{}",
                spec.id,
                spec.chapter,
                spec.predictability,
                st.label(),
                u8::from(claimed.contains(&spec.id)),
                s.goldens.0,
                s.goldens.1,
                s.excluded.0,
                s.excluded.1,
                s.rejects.0,
                s.rejects.1,
                s.accepts.0,
                s.accepts.1,
                s.silences.0,
                s.silences.1,
            );
        }
    }
    println!();
    for (st, n) in &totals {
        println!("{:<13} {n}", st.label());
    }
    println!("\nby chapter (covered / partial / aside / aside-missed / unmeasured)");
    let chapters: Vec<String> = {
        let mut v: Vec<String> = specs.iter().map(|s| s.chapter.clone()).collect();
        v.sort();
        v.dedup();
        v
    };
    for ch in chapters {
        let n = |st: Status| counts.get(&(ch.clone(), st)).copied().unwrap_or(0);
        println!(
            "  {ch}  {:>4} {:>4} {:>4} {:>4} {:>4}",
            n(Status::Covered),
            n(Status::Partial),
            n(Status::Aside),
            n(Status::AsideMissed),
            n(Status::Unmeasured)
        );
    }
    Ok(())
}
