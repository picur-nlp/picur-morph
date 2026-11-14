//! Text-level code-page repairs whose evidence is spread over several tokens (`REP-ENC-007`,
//! `REP-ENC-011`).

use picur_morph_lexicon::frequency;
use picur_morph_ortho::encoding::{Page, unfit, western};
use picur_morph_spell::{Status, check};

use crate::{Fix, is_hungarian_spelling, unvouched, vouched};

/// Whether `c` is `ő`, `ű` or a capital of them, the letters only Hungarian writes and the
/// Latin-2 images of Western `õ` and `û`.
fn is_double_acute(c: char) -> bool {
    matches!(c, 'ő' | 'ű' | 'Ő' | 'Ű')
}

/// The Western reading of a token the checker rejects, and whether it changes a letter other
/// than `ő` and `ű`.
fn inverse(word: &str) -> Option<(String, bool)> {
    if is_hungarian_spelling(word) && !word.chars().any(is_double_acute) {
        return None;
    }
    if word.is_ascii() || check(word) == Status::Ok {
        return None;
    }
    let candidate = western(word).into_iter().next()?;
    let foreign = word
        .chars()
        .zip(candidate.chars())
        .any(|(a, b)| a != b && !is_double_acute(a));
    Some((candidate, foreign))
}

/// Western words read as a Central European page, restored (`REP-ENC-007`).
///
/// A run of neighbouring rejected tokens that each have a Western reading is restored when it
/// mixes the scripts of two languages: `ő` or `ű`, which only Hungarian writes, beside an image
/// letter Hungarian never writes (`crčme brűlée`, `ń` and `ő` in one token). Either alone is a
/// spelling of some language: a rejected word with `ő` is far likelier a Hungarian word the
/// lexicon lacks (`leértékelődéséből`) than Western damage, and `Dvořák` or the Polish-spelled
/// `Espańa` are Czech and Polish as written. A word the checker accepts is never touched.
pub(crate) fn western_fixes(words: &[(usize, &str)]) -> Vec<Fix> {
    let read: Vec<Option<(String, bool)>> = words.iter().map(|(_, w)| inverse(w)).collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < words.len() {
        if read[i].is_none() {
            i += 1;
            continue;
        }
        let mut j = i;
        while read.get(j + 1).is_some_and(Option::is_some) {
            j += 1;
        }
        let run = i..=j;
        let hungarian = run.clone().any(|k| words[k].1.chars().any(is_double_acute));
        let foreign = run
            .clone()
            .any(|k| read[k].as_ref().is_some_and(|(_, f)| *f));
        if hungarian && foreign {
            for k in run {
                if let Some((to, _)) = &read[k] {
                    out.push(Fix {
                        at: words[k].0,
                        from: words[k].1.into(),
                        to: to.clone().into_boxed_str(),
                        rule: "REP-ENC-007",
                    });
                }
            }
        }
        i = j + 1;
    }
    out
}

/// Fewest letters of the `o` and `u` families a text needs before the absence of `ő` and `ű` in
/// it is evidence (`REP-ENC-011`).
const MIN_OU_LETTERS: usize = 200;

/// Whether `text` lost exactly the letters a best-fit conversion into Windows-1252 drops: the
/// other accents present, `ő` and `ű` absent over enough `o` and `u` letters (`REP-ENC-011`).
fn best_fitted(text: &str) -> bool {
    let mut ou = 0usize;
    for c in text.chars() {
        if is_double_acute(c) {
            return false;
        }
        if "oóöuúüOÓÖUÚÜ".contains(c) {
            ou += 1;
        }
    }
    ou >= MIN_OU_LETTERS && crate::accent::carries_accents(text)
}

/// Rejected tokens of a best-fitted text given back the `ő` and `ű` the conversion wrote as `o`
/// and `u` (`REP-ENC-011`).
///
/// The candidates are the lost letters only, so `elotte` may become `előtte` and never `élőtte`;
/// the commonest accepted one wins, and a tie abstains as `REP-DIAC-008` asks.
pub(crate) fn best_fit_fixes(text: &str, words: &[(usize, &str)]) -> Vec<Fix> {
    if !best_fitted(text) {
        return Vec::new();
    }
    words
        .iter()
        .filter(|(_, w)| unvouched(w))
        .filter_map(|&(at, w)| {
            let mut found: Vec<(u8, String)> = unfit(w, Page::Windows1252)
                .into_iter()
                .filter(|c| vouched(c))
                .map(|c| (frequency(&c), c))
                .collect();
            found.sort_by_key(|(f, _)| std::cmp::Reverse(*f));
            let to = match found.as_slice() {
                [(a, _), (b, _), ..] if a == b => return None,
                [(_, first), ..] => first.clone(),
                [] => return None,
            };
            Some(Fix {
                at,
                from: w.into(),
                to: to.into_boxed_str(),
                rule: "REP-ENC-011",
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::western_fixes;
    use picur_morph_ortho::tokens;

    fn restored(text: &str) -> Vec<String> {
        western_fixes(&tokens(text))
            .into_iter()
            .map(|f| f.to.into_string())
            .collect()
    }

    #[test]
    fn rep_enc_007_a_western_phrase_read_as_latin_2_is_restored() {
        assert_eq!(restored("crčme brűlée"), ["crème", "brûlée"]);
        assert_eq!(restored("egy Espańőla"), ["Españõla"]);
    }

    #[test]
    fn rep_enc_007_central_european_words_are_left_alone() {
        for text in [
            "Dvořák őrült",
            "Jiří Bělohlávek",
            "Espańa",
            "flűte",
            "Kőszeg és Győr",
        ] {
            assert!(restored(text).is_empty(), "{text}");
        }
    }
}
