//! The one tokeniser: what every crate above this one means by a word.

use crate::encoding::{is_continuation, is_lead};
use crate::norm::is_mark;

/// Symbols that belong to the token they end, because a suffix may follow them.
///
/// `PH-G2P-013`: `80%-os` and `°C-on` are single tokens, and splitting them left the bare `os`
/// and `on` to be checked as words.
const SYMBOLS: &str = "%‰§°²³€$";

/// Characters that join the parts of an e-mail or web address.
const ADDRESS: [char; 4] = ['@', '.', '/', ':'];

/// Word tokens of `text` with their byte offsets.
///
/// A token is a run of alphanumerics, the suffixable symbols and internal hyphens, so `2020-ban` and
/// `80%-os` are one token each and their suffixes are never checked bare. A token with no letter
/// at all is dropped.
///
/// Code-page damage is kept whole: `előtt` read as Windows-1252 is `elÅ‘tt`, and the `‘` is part
/// of the word because it follows a lead character (`WS-ENC-006`). The same `‘` after an ordinary
/// letter is a quotation mark and ends the token, so a mis-decoded word reaches the checker as
/// one token while `‘ház’` still yields `ház`.
///
/// A combining mark belongs to the letter before it, so a mark NFC could not compose
/// (`WS-NORM-002`) reaches the repair inside its word rather than cutting the word in two.
///
/// An address is not a word: a run joined by `@ . / :` between alphanumerics
/// (`info@picur.hu`, `www.picur.hu`, `picur.hu/docs`) is one token and is dropped whole, so its
/// fragments are never judged as Hungarian.
#[must_use]
pub fn tokens(text: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    let mut prev = ' ';
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        let joins = |x: char| x.is_alphanumeric() || ADDRESS.contains(&x);
        let next_joins = chars.peek().is_some_and(|(_, n)| joins(*n));
        // `REP-ENC-011`: a byte a strict decoder dropped shows as `?` or U+FFFD between the
        // letters of a word (`Gy?r`), and belongs to the word.
        let lost_byte = matches!(c, '?' | '\u{FFFD}')
            && start.is_some()
            && prev.is_alphabetic()
            && chars.peek().is_some_and(|(_, n)| n.is_alphabetic());
        let wordy = c.is_alphanumeric()
            || SYMBOLS.contains(c)
            || is_lead(c)
            || (is_continuation(c) && is_lead(prev))
            || (is_mark(c) && start.is_some())
            || lost_byte
            || (c == '-' && start.is_some())
            || (ADDRESS.contains(&c) && joins(prev) && next_joins && start.is_some());
        match (wordy, start) {
            (true, None) => start = Some(i),
            (false, Some(s)) => {
                out.push((s, text[s..i].trim_matches('-')));
                start = None;
            }
            _ => {}
        }
        prev = c;
    }
    if let Some(s) = start {
        out.push((s, text[s..].trim_matches('-')));
    }
    out.retain(|(_, w)| w.chars().any(char::is_alphabetic) && !w.contains(ADDRESS));
    out
}

#[cfg(test)]
mod tests {
    use super::tokens;

    fn words(text: &str) -> Vec<&str> {
        tokens(text).into_iter().map(|(_, w)| w).collect()
    }

    #[test]
    fn offsets_index_the_input() {
        let text = "A ház nagy.";
        for (at, w) in tokens(text) {
            assert_eq!(&text[at..at + w.len()], w);
        }
        assert_eq!(words(text), ["A", "ház", "nagy"]);
    }

    #[test]
    fn suffixed_numbers_and_symbols_stay_whole() {
        assert_eq!(
            words("2020-ban 80%-os °C-on"),
            ["2020-ban", "80%-os", "°C-on"]
        );
        assert_eq!(words("42 + 7"), Vec::<&str>::new());
    }

    #[test]
    fn ws_enc_006_mojibake_is_one_token() {
        assert_eq!(
            words("elÅ‘tt, K√©rem hĹąvĂ¶s"),
            ["elÅ‘tt", "K√©rem", "hĹąvĂ¶s"]
        );
    }

    #[test]
    fn a_quotation_mark_after_a_letter_ends_the_token() {
        assert_eq!(
            words("‘ház’ „kert” ház–kert"),
            ["ház", "kert", "ház", "kert"]
        );
    }
}
