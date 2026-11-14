//! A listed stem carrying a mis-written case suffix (`REP-SPELL-006`, `REP-SPELL-012`).

use picur_morph_core::{Case, Morph, generate};
use picur_morph_lexicon::{Trait, lexicon};
use picur_morph_ortho::case::starts_upper;
use picur_morph_ortho::letters;

use crate::{is_all_caps, vouched};

/// Most letters a written case suffix has, with its assimilated consonant.
const MAX_SUFFIX_LETTERS: usize = 6;

/// The form a listed stem's case gives, where the token writes the same case wrongly.
///
/// A name, an acronym or a foreign word takes its suffix by its pronunciation, and the errors are
/// what the seam can be written as instead: the assimilated or doubled consonant written once
/// (`Bachal`, `Grimmel`, `pech-el`), or a hyphen where none belongs or none where one does
/// (`Glasgowban`, `Goethe-vel`). The lexicon supplies the stem, the generator writes the cell, and
/// the token has to differ from it in exactly one of those two ways; nothing else is a match, so
/// `Tarzan` is not `Tarban`. An unlisted name cannot be regenerated and is never touched, and a
/// second candidate makes the rule abstain.
///
/// Rules: `REP-SPELL-006`, `REP-SPELL-012`, `REP-SPELL-016`, `ORT-FOR-012`
pub(crate) fn resuffixed(word: &str) -> Option<(String, &'static str)> {
    if !(starts_upper(word) || is_all_caps(word) || word.contains('-')) || vouched(word) {
        return None;
    }
    let mut found: Vec<(String, &'static str)> = Vec::new();
    let spans: Vec<_> = letters(word).collect();
    for (k, span) in spans.iter().enumerate().skip(1) {
        if spans.len() - k > MAX_SUFFIX_LETTERS {
            continue;
        }
        let stem = word[..span.at].trim_end_matches('-');
        let tail = &word[span.at..];
        if stem.is_empty() || tail.is_empty() || tail.contains('-') {
            continue;
        }
        for entry in lexicon().get(stem) {
            // A name's hyphen is put in or taken out only on a pronunciation the lexicon records
            // (`ORT-FOR-012`), because a stem without one may end in a silent letter the writer
            // knew about (`Seattle-ben`). The hyphen after a doubled final consonant is a rule of
            // the letters (`ORT-NAME-012`: `Grimm-mel`), and a common noun has no silent letters.
            let pronounced = entry.has(Trait::Spoken)
                || entry.alt.is_some()
                || !starts_upper(stem)
                || doubled_final(stem);
            for case in Case::ALL {
                let Ok(form) = generate(entry, &[Morph::Case(case)]) else {
                    continue;
                };
                if form == word || !same_cell(word, stem, tail, &form) || !vouched(&form) {
                    continue;
                }
                if word.contains('-') != form.contains('-') && !pronounced {
                    continue;
                }
                let rule = match case {
                    Case::Ins | Case::Tra => "REP-SPELL-006",
                    _ => "REP-SPELL-012",
                };
                if !found.iter().any(|(f, _)| *f == form) {
                    found.push((form, rule));
                }
            }
        }
    }
    match found.as_slice() {
        [only] => Some(only.clone()),
        _ => None,
    }
}

/// Whether `stem` ends in a doubled consonant letter (`Grimm`, `Baross`).
fn doubled_final(stem: &str) -> bool {
    let spans: Vec<_> = letters(stem).collect();
    matches!(spans.as_slice(), [.., a, b] if a.letter == b.letter && b.letter.is_consonant())
}

/// Whether `word`, read as `stem` plus `tail`, is `form` with one seam error.
///
/// Either the written tail is the generated one less its first consonant letter, which is the
/// assimilated or doubled consonant written once (`Bach` + `al` against `Bachhal`), or the two
/// differ only by a hyphen and the length of the stem's final vowel (`Goethe-vel` against
/// `Goethével`, `Glasgowban` against `Glasgow-ban`).
fn same_cell(word: &str, stem: &str, tail: &str, form: &str) -> bool {
    let plain = |s: &str| s.replace('-', "");
    if plain(word) == plain(form) {
        return true;
    }
    let lengthened = format!("{}{tail}", picur_morph_core::lengthen_final(stem));
    if plain(&lengthened) == plain(form) {
        return true;
    }
    let Some(generated_tail) = form.strip_prefix(stem).map(|t| t.trim_start_matches('-')) else {
        return false;
    };
    let mut written = letters(generated_tail);
    let Some(first) = written.next().filter(|s| s.letter.is_consonant()) else {
        return false;
    };
    let rest = &generated_tail[first.text.len()..];
    if rest == tail {
        return true;
    }
    // After an acronym's or an abbreviation's hyphen the assimilated consonant follows the
    // pronunciation, which the writer may have missed (`AIDS-cel` against `AIDS-szel`).
    word.contains('-')
        && letters(tail)
            .next()
            .is_some_and(|w| w.letter.is_consonant() && &tail[w.text.len()..] == rest)
}
