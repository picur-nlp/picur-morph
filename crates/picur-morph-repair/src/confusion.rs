//! Character confusions: the OCR and lookalike substitutions a rewrite may reverse.

use std::sync::LazyLock;

use picur_morph_ortho::letters;

use crate::tables::pairs;

/// Spellings that are confused for one another, tried in the direction written.
///
/// Each family is a rule: `REP-SPELL-001` for `j` against `ly`, `REP-SPELL-002` and
/// `REP-SPELL-003` for vowel length, `REP-OCR-001` for the Latin-script shape confusions that OCR
/// produces. They share one mechanism because they share one decision procedure: toggle at a single
/// position and keep the result the checker accepts.
static CONFUSIONS: LazyLock<Vec<(&'static str, &'static str)>> =
    LazyLock::new(|| pairs("Confusion"));

/// What OCR reads a letter as (`REP-OCR-001`).
pub static OCR_CONFUSIONS: LazyLock<Vec<(&'static str, &'static str)>> =
    LazyLock::new(|| pairs("OcrConfusion"));

/// Whether OCR of `from` can produce `to` (`REP-OCR-001`).
#[must_use]
pub fn confuses(from: &str, to: &str) -> bool {
    OCR_CONFUSIONS.iter().any(|&(a, b)| a == from && b == to)
}

/// Replaces the `n`th occurrence of `from` with `to`.
///
/// One position at a time: replacing every occurrence would change letters the writer got right,
/// and the whole method depends on proposing the smallest edit the checker will accept.
pub(crate) fn replace_nth(word: &str, from: &str, to: &str, n: usize) -> Option<String> {
    let at = word.match_indices(from).nth(n)?.0;
    Some(format!("{}{to}{}", &word[..at], &word[at + from.len()..]))
}

/// Candidate corrections from the confusion sets and from consonant doubling.
///
/// Rules: `REP-SPELL-001`, `REP-SPELL-002`, `REP-SPELL-003`, `REP-SPELL-004`, `REP-OCR-001`
pub(crate) fn confusion_candidates(word: &str) -> Vec<(String, &'static str)> {
    let mut out = Vec::new();
    for &(from, to) in CONFUSIONS.iter() {
        for n in 0..word.matches(from).count() {
            if let Some(c) = replace_nth(word, from, to, n) {
                out.push((c, rule_for(from)));
            }
        }
    }
    // REP-SPELL-004 and REP-SPELL-005: a consonant written single where it should be long.
    // `bizotság` for `bizottság`, `visza` for `vissza`.
    //
    // Only this direction. The rule's examples are all single-for-long, and the reverse produced
    // the one wrong repair observed on real text: `orrt` is ill formed (`NOM-ACC-006` forbids it),
    // but undoubling gave the accepted `ort`, which is a different word. Removing a letter the
    // writer put there needs better evidence than the checker alone provides.
    for span in letters(word) {
        if !span.letter.is_consonant() {
            continue;
        }
        let text = span.letter.as_str();
        let end = span.at + span.text.len();
        let doubled = match text.chars().next() {
            Some(first) if text.chars().count() > 1 => format!("{first}{text}"),
            _ => format!("{text}{text}"),
        };
        out.push((
            format!("{}{doubled}{}", &word[..span.at], &word[end..]),
            if text.chars().count() > 1 {
                "REP-SPELL-005"
            } else {
                "REP-SPELL-004"
            },
        ));
    }
    out
}

/// Words differing from `word` by one consonant letter written double instead of single.
///
/// `REP-SPELL-004`: the inverse of the doubling in [`confusion_candidates`], one position at a
/// time. The caller decides, and it needs more evidence than the doubling direction does, because
/// this one takes away a letter the writer wrote.
pub(crate) fn undoubled_candidates(word: &str) -> Vec<String> {
    let mut out = Vec::new();
    for span in letters(word) {
        if !span.letter.is_consonant() {
            continue;
        }
        let text = span.letter.as_str();
        let end = span.at + span.text.len();
        // A single-character letter is doubled by repetition, a digraph by repeating its first
        // character, so what comes away is one character either way (`WS-DIG-002`).
        let Some(first) = text.chars().next() else {
            continue;
        };
        let doubled = if text.chars().count() > 1 {
            format!("{first}{text}")
        } else {
            format!("{text}{text}")
        };
        if word[span.at..].starts_with(&doubled) {
            out.push(format!(
                "{}{text}{}",
                &word[..span.at],
                &word[span.at + doubled.len()..]
            ));
        } else if span.at >= first.len_utf8() && word[..end].ends_with(&doubled) {
            out.push(format!(
                "{}{text}{}",
                &word[..span.at - first.len_utf8()],
                &word[end..]
            ));
        }
    }
    out
}

/// The rule a confusion family belongs to, for reporting.
fn rule_for(from: &str) -> &'static str {
    match from {
        "f" => "REP-OCR-006",
        "j" | "ly" => "REP-SPELL-001",
        "rn" | "m" | "cl" | "1" | "0" | "5" => "REP-OCR-001",
        "ó" | "o" | "ő" | "ö" => "REP-SPELL-003",
        _ => "REP-SPELL-002",
    }
}
