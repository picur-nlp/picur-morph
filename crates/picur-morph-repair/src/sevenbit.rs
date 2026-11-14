//! The seven-bit conventions that wrote Hungarian before Unicode (`REP-ENC-014`).

use picur_morph_spell::{Status, check};

use crate::unvouched;

use std::sync::LazyLock;

use crate::tables::char_rows;

/// The seven-bit convention: a vowel, a digit and the letter they name (`REP-KEY-003`).
static DIGITS: LazyLock<Vec<Vec<char>>> = LazyLock::new(|| char_rows("SevenBitDigit"));
/// The token decoded from the digit notation, if the checker accepts the result (`REP-ENC-014`).
///
/// The digit notation is the one convention a token carries its own evidence for: a digit between
/// letters inside a word is not Hungarian and not English either, so the rate clause the rule
/// states for a block is already met by the token.
///
/// The other two are not here. The flying accent (`a'`, `o:`, `o"`) writes its mark with
/// characters no token contains, so it has to be decoded before tokenisation, and
/// `norm::expand_accents` is where it lives. The telegram doubling (`aa`, `oeoe`) is a real
/// spelling in `kooperáció` and `vákuum`, and acceptance does not separate it: measured over
/// 27,000 words of English prose it turned `been` into `bén`, `too` into `tó` and `soon` into
/// `són` 41 times, which is what the rule's block-level rate clause is for.
///
/// Rules: `REP-ENC-014`, `WS-NORM-006`
pub(crate) fn seven_bit(word: &str) -> Option<(String, &'static str)> {
    if !unvouched(word) {
        return None;
    }
    let decoded = digits(word);
    (decoded != word && check(&decoded) == Status::Ok).then_some((decoded, "REP-ENC-014"))
}

/// `word` with every vowel-and-digit pair read as the letter it names.
fn digits(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    let mut out = String::with_capacity(word.len());
    let mut i = 0;
    while i < chars.len() {
        if let Some(letter) = chars.get(i + 1).and_then(|mark| accented(chars[i], *mark)) {
            out.push(letter);
            i += 2;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

/// The letter a vowel and a digit name, in the case of the vowel.
fn accented(vowel: char, mark: char) -> Option<char> {
    let lower = vowel.to_lowercase().next()?;
    let letter = DIGITS
        .iter()
        .find(|r| r[0] == lower && r[1] == mark)
        .map(|r| r[2])?;
    if vowel.is_uppercase() {
        letter.to_uppercase().next()
    } else {
        Some(letter)
    }
}

#[cfg(test)]
mod tests {
    use super::seven_bit;

    fn decoded(word: &str) -> Option<String> {
        seven_bit(word).map(|(w, _)| w)
    }

    /// `REP-ENC-014`: the rule's own example of the digit notation, and the pangram.
    #[test]
    fn rep_enc_014_every_notation_decodes() {
        for (written, want) in [
            ("a1rvi1ztu3ro3", "árvíztűrő"),
            ("tu2ko2rfu1ro1ge1p", "tükörfúrógép"),
            ("Peto3fi", "Petőfi"),
        ] {
            assert_eq!(decoded(written).as_deref(), Some(want), "{written}");
        }
    }

    /// A digit that is not a notation, and the doubled vowels the telegram style would take.
    #[test]
    fn rep_enc_014_a_real_spelling_is_not_a_notation() {
        for word in ["kooperáció", "vákuum", "Aachen", "x1", "A4", "2020-ban"] {
            assert_eq!(decoded(word), None, "{word}");
        }
    }
}
