//! Prints every reading of a word, against the checker's own search space.
//!
//! ```text
//! cargo run -p xtask -- analyse jól végzett volna
//! ```
//!
//! The counterpart to `gold`: that reports which words disagree with an annotation, this says what
//! the analyser actually offered for one of them.

use picur_morph_core::{Role, segment_reading};
use picur_morph_spell::{Status, analyses, check};

/// A morph's name, without its fields, for a one-line segmentation.
fn short(morph: picur_morph_core::Morph) -> String {
    let text = format!("{morph:?}");
    text.split([' ', '(']).next().unwrap_or(&text).to_string()
}

/// The morphs a stem piece has absorbed.
fn describe(fused: &[picur_morph_core::Morph]) -> String {
    fused
        .iter()
        .copied()
        .map(short)
        .collect::<Vec<_>>()
        .join(",")
}

/// Reports the readings of each word in `words`.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn report(words: &[&str]) {
    for word in words {
        let status = match check(word) {
            Status::Ok => "ok",
            Status::Unknown => "unknown",
            Status::Wrong => "wrong",
            _ => "?",
        };
        println!("{word}  [{status}]");
        let readings = analyses(word);
        if readings.is_empty() {
            println!("    no reading");
        }
        for a in readings {
            println!("    {:<24} {:?}  {:?}", a.lemma, a.pos, a.stack);
            if let Some(pieces) = segment_reading(&a) {
                let cut: Vec<String> = pieces
                    .iter()
                    .map(|p| match &p.role {
                        Role::Preverb => format!("{}=", p.text),
                        Role::Member => format!("{}|", p.text),
                        Role::Stem(fused) if fused.is_empty() => p.text.to_string(),
                        Role::Stem(fused) => format!("{}({})", p.text, describe(fused)),
                        Role::Affix(m) => format!("{}:{}", p.text, short(*m)),
                        _ => p.text.to_string(),
                    })
                    .collect();
                println!("      {}", cut.join(" + "));
            }
        }
    }
}
