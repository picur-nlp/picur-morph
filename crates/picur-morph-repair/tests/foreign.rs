//! `REP-DIAC-001` must never classify foreign text as accent-stripped Hungarian.
//!
//! The rulebook records this as an honest limit: no foreign negative set was available, so the
//! exclusion of English, Dutch and Latin "rests on the restoration clause by argument, not by
//! measurement". This measures it. English is the hard case, because unaccented text is exactly
//! what stripped Hungarian looks like, and a false positive here is not a missed repair but a
//! corrupted document: the restorer would rewrite English words into Hungarian ones.
//!
//! Run with `FOREIGN=<file>` to point it at a corpus, one paragraph per line.

use picur_morph_repair::is_accent_stripped;

#[test]
fn english_is_not_accent_stripped_hungarian() {
    let Ok(path) = std::env::var("FOREIGN") else {
        return;
    };
    let text = std::fs::read_to_string(path).expect("corpus is readable");
    // Paragraphs, because the detector abstains below 30 tokens by design.
    let mut paragraphs: Vec<String> = Vec::new();
    let mut buf = String::new();
    for line in text.lines() {
        buf.push(' ');
        buf.push_str(line.trim());
        if buf.split_whitespace().count() >= 60 {
            paragraphs.push(std::mem::take(&mut buf));
        }
    }
    assert!(paragraphs.len() >= 10, "need a real sample");
    let flagged: Vec<&String> = paragraphs
        .iter()
        .filter(|p| is_accent_stripped(p))
        .collect();
    assert!(
        flagged.is_empty(),
        "{} of {} English paragraphs read as stripped Hungarian, e.g. {:?}",
        flagged.len(),
        paragraphs.len(),
        flagged.first().map(|p| &p[..p.len().min(120)])
    );
}
