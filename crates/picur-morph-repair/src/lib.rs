//! Text repair.
//!
//! Every rule in chapter 10 is licensed the same way, and it is worth stating once: a rewrite is
//! applied only when the original is **rejected** and the result is **accepted**. `REP-ARCH-002`
//! puts it as "the `c` form is oracle-accepted", `REP-ARCH-003` as "token rejected with the old
//! sequence and accepted after replacing it". The checker is that oracle, so a repair can never
//! invent a word, and a letter pattern that happens to occur in a correct word is left alone:
//! `vadság` keeps its `ds` because `vadság` is already accepted.
//!
//! That is what makes the rules safe to state as plain sequence rewrites. The morphology does the
//! deciding.
//!
//! See `docs/grammar/10-text-repair.md`.

mod accent;
mod codepage;
mod confusion;
mod glyph;
mod sevenbit;
mod slip;
mod spacing;
mod suffix;
mod tables;
mod word;

pub use picur_morph_ortho::encoding;

pub use accent::{accent_alternatives, is_accent_stripped, restore_accents};
pub use confusion::{OCR_CONFUSIONS, confuses};
pub use glyph::{plain_letters, unescaped};
pub use tables::tabulated;
pub use word::{expand_compatibility, repair_word, repair_word_accented};

use picur_morph_ortho::case::{capitalise, lower, lower_initial, starts_upper};
use picur_morph_ortho::{letters, tokens};
use picur_morph_spell::{Status, analyses, check};

use spacing::spacing_fixes;

/// How aggressively to repair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Mode {
    /// Repair damage only: encoding, spacing, obvious noise.
    Conservative,
    /// Also normalise historical spelling to current orthography.
    ///
    /// `REP-ARCH-009` makes this a mode rather than a default, because historical spelling is not
    /// an error and a scholarly edition must be able to keep it.
    Modernise,
}

/// One proposed repair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fix {
    /// Byte offset of the token in the input.
    pub at: usize,
    /// The token as written.
    pub from: Box<str>,
    /// The proposed replacement.
    pub to: Box<str>,
    /// The rule that licensed it.
    pub rule: &'static str,
}

/// Repairs a whole text, returning one fix per token that has one.
///
/// The fixes are in text order and never overlap, so they can be spliced into the text directly;
/// [`apply`] does exactly that.
///
/// The text must already be in NFC (`WS-NORM-001`); see `check_text` for why this does not compose
/// it and what happens if the caller does not.
///
/// Rules: `REP-ARCH-009`
#[must_use]
pub fn repair(text: &str, mode: Mode) -> Vec<Fix> {
    let words = tokens(text);
    // Spacing is decided across token boundaries, so it is found first and takes precedence over
    // a per-token fix of either of its halves.
    let mut out = spacing_fixes(text, &words);
    out.extend(address_fixes(text));
    out.extend(flying_accent_fixes(text));
    out.extend(run_fixes(text));
    out.extend(space_character_fixes(text));
    out.extend(letter_spaced_fixes(text, &words));
    out.extend(codepage::western_fixes(&words));
    out.extend(codepage::best_fit_fixes(text, &words));
    // `REP-DIAC-001`: whether accents may be restored is a property of the segment, so it is
    // decided once here rather than per token.
    let stripped = mode == Mode::Modernise && is_accent_stripped(text);
    let accented = !stripped && accent::carries_accents(text);
    let mut prev_end = 0usize;
    for (i, &(at, word)) in words.iter().enumerate() {
        let gap = &text[prev_end..at];
        prev_end = at + word.len();
        let found = word::repair_word_in(word, mode, accented)
            .or_else(|| {
                (alone_in_sentence(text, &words, i) && ungoverned_reading(word))
                    .then(|| word::slipped_in_context(word, accented || !word.is_ascii()))
                    .flatten()
                    .map(|w| (w, "REP-KEY-007"))
            })
            .or_else(|| {
                stripped
                    .then(|| restore_accents(word).map(|w| (w, "REP-DIAC-003")))
                    .flatten()
            })
            .or_else(|| {
                (mode == Mode::Modernise && at > 0)
                    .then(|| capitalised_common(gap, word).map(|w| (w, "REP-SPELL-013")))
                    .flatten()
            });
        if let Some((to, rule)) = found {
            out.push(Fix {
                at,
                from: word.into(),
                to: to.into_boxed_str(),
                rule,
            });
        }
    }
    out.sort_by_key(|f| f.at);
    let mut end = 0usize;
    out.retain(|f| {
        let keep = f.at >= end;
        if keep {
            end = f.at + f.from.len();
        }
        keep
    });
    out
}

/// Whether the `i`th of `words` is the only word of its sentence.
fn alone_in_sentence(text: &str, words: &[(usize, &str)], i: usize) -> bool {
    let ends = |gap: &str| gap.contains(['.', '!', '?', '\n']);
    let (at, word) = words[i];
    let before = i
        .checked_sub(1)
        .is_none_or(|p| ends(&text[words[p].0 + words[p].1.len()..at]));
    let after = words
        .get(i + 1)
        .is_none_or(|&(next, _)| ends(&text[at + word.len()..next]));
    before && after
}

/// Whether every reading of `word` is a personal infinitive, which needs a governing impersonal
/// predicate (`VRB-INF-002`, `VRB-INF-003`): standing alone as its sentence it has none, so the
/// token is a non-word there and `REP-KEY-007` may repair it (`köszönnöm` for `köszönöm`).
fn ungoverned_reading(word: &str) -> bool {
    let readings = analyses(word);
    !readings.is_empty()
        && readings.iter().all(|r| {
            matches!(
                r.stack.as_slice(),
                [
                    picur_morph_core::Morph::Infinitive,
                    picur_morph_core::Morph::Possessive { .. }
                ]
            )
        })
}

/// Flying accents decoded: a mark typed after the vowel it belongs to (`a'rvi'ztu"ro"`),
/// `'` the acute, `"` the double acute and `:` the diaeresis (`REP-ENC-014`).
///
/// The convention is a block's, so the text must show at least two marks, and every decoded run
/// has to be a word the checker accepts.
fn flying_accent_fixes(text: &str) -> Vec<Fix> {
    const MIN_MARKS: usize = 2;
    let decode = |run: &str| -> Option<String> {
        let mut out = String::with_capacity(run.len());
        let mut marks = 0usize;
        for c in run.chars() {
            let last = out.chars().next_back();
            let accented = match (last, c) {
                (Some(v), '\'') => flying(v, "áéíóú"),
                (Some(v), '"') => double_acute(v),
                (Some(v), ':') => diaeresis(v),
                _ => None,
            };
            match accented {
                Some(a) => {
                    out.pop();
                    out.push(a);
                    marks += 1;
                }
                None => out.push(c),
            }
        }
        (marks > 0).then_some(out)
    };
    let total: usize = text.matches(['\'', '"', ':']).count();
    if total < MIN_MARKS {
        return Vec::new();
    }
    let mut out = Vec::new();
    for (at, run) in whitespace_runs(text) {
        let Some(decoded) = decode(run) else { continue };
        if decoded != run && check(&decoded) == Status::Ok {
            out.push(Fix {
                at,
                from: run.into(),
                to: decoded.into_boxed_str(),
                rule: "REP-ENC-014",
            });
        }
    }
    out
}

/// The acute of a vowel, from the five accented letters in vowel order.
fn flying(vowel: char, accented: &str) -> Option<char> {
    let i = "aeiou".find(vowel.to_ascii_lowercase())?;
    let letter = accented.chars().nth(i)?;
    Some(if vowel.is_uppercase() {
        letter.to_uppercase().next()?
    } else {
        letter.to_lowercase().next()?
    })
}

fn double_acute(vowel: char) -> Option<char> {
    match vowel {
        'o' => Some('ő'),
        'u' => Some('ű'),
        'O' => Some('Ő'),
        'U' => Some('Ű'),
        _ => None,
    }
}

fn diaeresis(vowel: char) -> Option<char> {
    match vowel {
        'o' => Some('ö'),
        'u' => Some('ü'),
        'O' => Some('Ö'),
        'U' => Some('Ü'),
        _ => None,
    }
}

/// The repairs that need the characters the tokeniser leaves out of a word: a Hungarian key
/// typed under the US layout (`t[r`, `h'z`) and a transfer encoding (`=C3=A9s`, `\u0151sz`).
///
/// A run is repaired when its decoding is a word the lexicon vouches for and the run itself is
/// not.
///
/// Rules: `REP-KEY-003`, `REP-ENC-013`
fn run_fixes(text: &str) -> Vec<Fix> {
    let mut out = Vec::new();
    for (at, run) in whitespace_runs(text) {
        let run = run.trim_end_matches(['.', ',', ';', ':', '!', '?']);
        // A run of letters is a token the word repair saw; a hyphenated pair of words is two
        // words (`egy-két`); a run needs two letters before a key in it is read as a letter.
        if run.chars().all(char::is_alphanumeric)
            || run.chars().filter(|c| c.is_alphabetic()).count() < 2
            || run.contains('-')
            || vouched(run)
        {
            continue;
        }
        let found = unescaped(run)
            .filter(|w| w != run && vouched(w))
            .map(|w| (w, "REP-ENC-013"))
            .or_else(|| {
                let shifted = glyph::us_layout_text(run);
                (shifted != run && vouched(&shifted)).then_some((shifted, "REP-KEY-003"))
            });
        if let Some((to, rule)) = found {
            out.push(Fix {
                at,
                from: run.into(),
                to: to.into_boxed_str(),
                rule,
            });
        }
    }
    out
}

/// A no-break space between digits written as a plain space, and a line separator as a line
/// feed (`REP-SPACE-006`).
fn space_character_fixes(text: &str) -> Vec<Fix> {
    let mut out = Vec::new();
    for (at, c) in text.char_indices() {
        let before = text[..at].chars().next_back();
        let after = text[at + c.len_utf8()..].chars().next();
        let to = match c {
            '\u{00A0}'
                if before.is_some_and(|b| b.is_ascii_digit())
                    && after.is_some_and(|a| a.is_ascii_digit()) =>
            {
                " "
            }
            '\u{2028}' => "\n",
            _ => continue,
        };
        out.push(Fix {
            at,
            from: c.to_string().into_boxed_str(),
            to: to.into(),
            rule: "REP-SPACE-006",
        });
    }
    out
}

/// The whitespace-separated runs of `text` with their byte offsets.
fn whitespace_runs(text: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut at = 0usize;
    for run in text.split(char::is_whitespace) {
        if !run.is_empty() {
            out.push((at, run));
        }
        at += run.len() + 1;
    }
    out.retain(|(a, r)| text.get(*a..a + r.len()) == Some(*r));
    out
}

/// Letters spaced out by a layout or an OCR of letter-spacing (`semmi m á s !`), rejoined
/// where three or more single letters in a row make a word (`REP-SPACE-001`).
fn letter_spaced_fixes(text: &str, words: &[(usize, &str)]) -> Vec<Fix> {
    const MIN_LETTERS: usize = 3;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < words.len() {
        let mut j = i;
        while j + 1 < words.len()
            && words[j].1.chars().count() == 1
            && words[j + 1].1.chars().count() == 1
            && text.get(words[j].0 + words[j].1.len()..words[j + 1].0) == Some(" ")
        {
            j += 1;
        }
        if j - i + 1 >= MIN_LETTERS {
            let joined: String = words[i..=j].iter().map(|(_, w)| *w).collect();
            let word = if is_all_caps(&joined) {
                lower(&joined)
            } else {
                joined.clone()
            };
            // Four or more spaced letters are never prose, so the run is joined whatever it
            // spells; three must spell a word.
            if j - i + 1 > MIN_LETTERS || check(&word) == Status::Ok {
                out.push(Fix {
                    at: words[i].0,
                    from: text[words[i].0..words[j].0 + words[j].1.len()].into(),
                    to: joined.into_boxed_str(),
                    rule: "REP-SPACE-001",
                });
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }
    out
}

/// Restores the accents of every token of a text the caller knows to be stripped, where the
/// restoration is unique (`REP-DIAC-003`, `REP-DIAC-012`).
///
/// [`repair`] decides for itself whether a segment is stripped (`REP-DIAC-001`) and needs a
/// segment's worth of evidence to do so; a caller that already knows, a sentence at a time,
/// uses this.
#[must_use]
pub fn restore_text(text: &str) -> String {
    let fixes: Vec<Fix> = tokens(text)
        .into_iter()
        .filter_map(|(at, word)| {
            let restored = if is_all_caps(word) {
                accent::restore_acronym(word)
            } else {
                restore_accents(word)
            }?;
            Some(Fix {
                at,
                from: word.into(),
                to: restored.into_boxed_str(),
                rule: "REP-DIAC-003",
            })
        })
        .collect();
    apply(text, &fixes)
}

/// An e-mail address whose `@` came out as the key's base letter `v` (`pistavfreemail.hu`).
///
/// Addresses are not word tokens, so the text is scanned for them directly: a run of address
/// characters ending in a listed mail domain, with a `v` where the `@` belongs and no `@` at all.
///
/// Rules: `REP-KEY-005`
fn address_fixes(text: &str) -> Vec<Fix> {
    let mut out = Vec::new();
    let domains = tables::words("MailDomain");
    for (at, run) in text
        .split(|c: char| c.is_whitespace())
        .scan(0usize, |pos, w| {
            let start = text[*pos..].find(w).map(|i| *pos + i)?;
            *pos = start + w.len();
            Some((start, w))
        })
    {
        let run = run.trim_end_matches(['.', ',', ';', ':', ')', '!', '?']);
        if run.contains('@') || run.contains('/') {
            continue;
        }
        let Some(domain) = domains.iter().find(|d| run.ends_with(*d)) else {
            continue;
        };
        let local = &run[..run.len() - domain.len()];
        let Some(local) = local.strip_suffix('v') else {
            continue;
        };
        let plausible = !local.is_empty()
            && local
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
        if plausible {
            out.push(Fix {
                at,
                from: run.into(),
                to: format!("{local}@{domain}").into_boxed_str(),
                rule: "REP-KEY-005",
            });
        }
    }
    out
}

/// The weekdays and months, which `PND-DATE-002` and `PND-DATE-003` write in lower case.
fn calendar() -> impl Iterator<Item = &'static str> {
    picur_morph_lexicon::words(picur_morph_lexicon::Role::Weekday)
        .iter()
        .chain(picur_morph_lexicon::words(picur_morph_lexicon::Role::Month))
        .map(AsRef::as_ref)
}

/// A common word capitalised inside a sentence, in its lower case (`REP-SPELL-013`).
///
/// The rule's lexical input is which capitalised words are names, and the lexicon answers it only
/// for the classes no name shares: the essive of a nationality adjective (`Magyarul`) and a
/// weekday or month (`Hétfőn`). The `-i` adjective of a place is not one, because surnames and
/// institution names are written with it (`Károlyi`, `Pest Megyei Bíróság`), and a surname that is
/// also a common word (`Nagy`, `Fekete`) stays for the same reason. `gap` is the text before the
/// token; a sentence or a quotation starts after it, so a capital there is not a slip.
///
/// Rules: `REP-SPELL-013`, `ORT-CAP-002`, `PND-DATE-002`
fn capitalised_common(gap: &str, word: &str) -> Option<String> {
    if !starts_upper(word)
        || is_all_caps(word)
        || gap.contains(['.', '!', '?', ':', '"', '\u{201e}'])
    {
        return None;
    }
    let lowered = picur_morph_ortho::case::lower_initial(word);
    let shaped = lowered.ends_with(['l']) && lowered.ends_with("ul") | lowered.ends_with("ül")
        || calendar().any(|d| lowered.starts_with(d));
    if !shaped || !vouched(&lowered) {
        return None;
    }
    let common = analyses(&lowered).iter().any(|a| {
        let lemma: &str = &a.lemma;
        calendar().any(|d| d == lemma)
            || (a.pos == picur_morph_lexicon::Pos::Adjective
                && a.stack.first()
                    == Some(&picur_morph_core::Morph::Case(picur_morph_core::Case::Ess)))
    });
    common.then_some(lowered)
}

/// Whether the checker cannot vouch for `word` as written.
///
/// Rejection is too strong a test for a repair that has no evidence beyond the checker: a word the
/// lexicon does not list comes back [`Status::Unknown`] whether it is a fragment (`nyörű`), a
/// slip (`yöld`) or a name. A capitalised word the checker cannot place is a name, which is what
/// the checker means by unvouched, and no repair improves on a name.
pub(crate) fn unvouched(word: &str) -> bool {
    match check(word) {
        Status::Ok => false,
        Status::Unknown => !starts_upper(word),
        _ => true,
    }
}

/// Whether the checker accepts `word` as a listed word or a form of one.
///
/// Acceptance alone is not that: a hyphenated capitalised base is accepted as a possible foreign
/// name (`Nzelvtan-a`) and a coined compound by the compounding rules (`vizsgálatlés`), and a
/// repair that lands on either has only moved the damage. A reading whose lemma the lexicon lists
/// is the evidence a repair needs.
pub(crate) fn vouched(word: &str) -> bool {
    picur_morph_spell::vouches(word)
}

/// Whether the lexicon vouches for a token as written, reading an all-capital one as a word.
///
/// `ORT-ABBR-011` makes any all-capital token an acronym, and the checker accepts one whatever its
/// letters are, which says nothing about whether the letters are right: `ÁLLAT` and `ÅLLAT` are
/// both accepted and only one is a word. `ORT-CAP-002` keeps the accents on capitals, so an
/// all-capital token is judged by the word it spells, tried lowercase and capitalised.
///
/// Rules: `ORT-ABBR-011`, `ORT-CAP-002`, `REP-DIAC-012`
pub(crate) fn vouched_as_written(word: &str) -> bool {
    if !is_all_caps(word) {
        return vouched(word) || (starts_upper(word) && vouched(&lower_initial(word)));
    }
    let spelled = lower(word);
    vouched(&spelled) || vouched(&capitalise(&spelled))
}

/// Whether every letter of `word` is a capital and there is more than one.
pub(crate) fn is_all_caps(word: &str) -> bool {
    let mut letters = word.chars().filter(|c| c.is_alphabetic());
    letters.clone().count() > 1 && letters.all(char::is_uppercase)
}

/// Applies the fixes of [`repair`] to the text.
#[must_use]
pub fn apply(text: &str, fixes: &[Fix]) -> String {
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0usize;
    for f in fixes {
        out.push_str(&text[cursor..f.at]);
        out.push_str(&f.to);
        cursor = f.at + f.from.len();
    }
    out.push_str(&text[cursor..]);
    out
}

/// Whether a word is spelled only with alphabet letters.
#[must_use]
pub fn is_hungarian_spelling(word: &str) -> bool {
    !word.is_empty()
        && word
            .chars()
            .all(|c| c.is_ascii_alphabetic() || accent::ACCENTED.contains(c))
        && letters(word).map(|s| s.text.len()).sum::<usize>() == word.len()
}

#[cfg(test)]
mod tests {
    use super::{Fix, Mode, apply, is_accent_stripped, repair, repair_word};

    fn fixed(word: &str) -> Option<String> {
        repair_word(word, Mode::Modernise).map(|(w, _)| w)
    }

    #[test]
    fn unsimplified_doubled_digraphs() {
        for (old, new) in [
            ("hoszszú", "hosszú"),
            ("veszszük", "vesszük"),
            ("tavaszszal", "tavasszal"),
            ("aszszony", "asszony"),
            ("profeszszionális", "professzionális"),
        ] {
            assert_eq!(fixed(old).as_deref(), Some(new), "{old}");
        }
    }

    #[test]
    fn a_compound_boundary_is_not_a_doubled_digraph() {
        // REP-ARCH-004: `kulcscsomó` is modern standard and must survive. It does so because the
        // checker accepts it, so the rule never fires.
        for kept in ["kulcscsomó", "jegygyűrű"] {
            assert_eq!(fixed(kept), None, "{kept} was rewritten");
        }
    }

    #[test]
    fn pre_reform_cz() {
        // REP-ARCH-002.
        assert_eq!(fixed("czukor").as_deref(), Some("cukor"));
        assert_eq!(fixed("czél").as_deref(), Some("cél"));
    }

    #[test]
    fn names_keep_their_spelling() {
        // REP-ARCH-002, REP-ARCH-003: `Móricz`, `Babits`, `Kováts` are not damage.
        for name in ["Móricz", "Babits", "Kováts", "Rátz", "Czuczor"] {
            assert_eq!(fixed(name), None, "{name} was rewritten");
        }
    }

    #[test]
    fn rep_spell_012_a_hyphen_before_the_suffix() {
        let fix = |w| repair_word(w, Mode::Modernise);
        assert_eq!(
            fix("1848ban"),
            Some(("1848-ban".to_owned(), "REP-SPELL-012"))
        );
        assert_eq!(fix("MÁVnál"), Some(("MÁV-nál".to_owned(), "REP-SPELL-012")));
        assert_eq!(fix("ház-ban"), Some(("házban".to_owned(), "REP-SPELL-012")));
        // A host that takes no hyphenated suffix is left to the letter rules, or `kints` would
        // come back as `kint-s` rather than `kincs`.
        assert_eq!(fix("kints"), Some(("kincs".to_owned(), "REP-ARCH-003")));
    }

    #[test]
    fn protestant_ts_and_tz() {
        // REP-ARCH-003.
        assert_eq!(fixed("kints").as_deref(), Some("kincs"));
        assert_eq!(fixed("ketske").as_deref(), Some("kecske"));
        assert_eq!(fixed("tzifra").as_deref(), Some("cifra"));
    }

    #[test]
    fn a_legitimate_ts_across_a_boundary_is_kept() {
        // REP-ARCH-003 names these: the sequence is real morphology, not old spelling.
        for kept in ["vadság", "kétszer", "látszik", "fáradtság"] {
            assert_eq!(fixed(kept), None, "{kept} was rewritten");
        }
    }

    #[test]
    fn phonetic_j_assimilation_spellings() {
        // REP-ARCH-005: the ipszilonista spellings become morphological ones.
        for (old, new) in [
            ("láttya", "látja"),
            ("aggya", "adja"),
            ("kerttye", "kertje"),
            ("bánnya", "bánja"),
        ] {
            assert_eq!(fixed(old).as_deref(), Some(new), "{old}");
        }
    }

    #[test]
    fn conservative_mode_leaves_history_alone() {
        // REP-ARCH-009: historical spelling is not an error.
        assert_eq!(repair_word("hoszszú", Mode::Conservative), None);
    }

    #[test]
    fn a_correct_text_is_untouched() {
        let text = "A hosszú asszony kulcscsomót vesz.";
        assert_eq!(repair(text, Mode::Modernise), [] as [Fix; 0]);
    }

    #[test]
    fn fixes_apply_in_place() {
        let text = "A hoszszú aszszony.";
        let fixes = repair(text, Mode::Modernise);
        assert_eq!(apply(text, &fixes), "A hosszú asszony.");
    }

    #[test]
    fn accents_are_restored() {
        // REP-DIAC-003, REP-DIAC-004: candidates generated by class, ranked by frequency.
        use super::restore_accents;
        for (stripped, want) in [
            ("vasar", "vásár"),
            ("es", "és"),
            ("szep", "szép"),
            ("tobb", "több"),
            ("ejjel", "éjjel"),
            ("kivul", "kívül"),
            ("tukorfurogep", "tükörfúrógép"),
        ] {
            assert_eq!(
                restore_accents(stripped).as_deref(),
                Some(want),
                "{stripped}"
            );
        }
    }

    #[test]
    fn accent_restoration_abstains_on_a_real_ambiguity() {
        // REP-DIAC-008. `arvizturo` restores two ways and both are well formed compounds:
        // `árvíztűrő` 'flood-resisting', which is the word of the pangram, and `árvíztúró`, which
        // is `árvíz` plus `túró` and parses although nobody writes it. The corpus separates them,
        // `árvíztűrő` 246 against no `árvíztúró` at all [WC2], but the frequency table is per
        // lemma and cannot see a compound, so the two tie and the rule abstains rather than guess.
        use super::restore_accents;
        use picur_morph_spell::{Status, check};
        assert_eq!(restore_accents("arvizturo"), None);
        for word in ["árvíztűrő", "árvíztúró"] {
            assert_eq!(check(word), Status::Ok, "{word}");
        }
    }

    #[test]
    fn a_token_that_is_already_a_word_is_left_alone() {
        // REP-DIAC-009: `mar` and `kert` are words. Restoring them would be damage, not repair.
        //
        // The set grows with the checker's lexicon: `varos` (`var` + `-Os`, 'scabby') and `meny` (+ relational
        // `-i`) are in it. Each entry costs a restoration a reader would want, which is correct: the
        // accent-stripped input is ambiguous and this layer has no context to break the tie.
        use super::restore_accents;
        for kept in ["mar", "kert", "haza", "ver", "varosban", "menyi"] {
            assert_eq!(restore_accents(kept), None, "{kept} was rewritten");
        }
    }

    #[test]
    fn restoration_declines_a_capitalised_token() {
        // A name may legitimately lack accents, and guessing them is worse than leaving them.
        use super::restore_accents;
        assert_eq!(restore_accents("Varosban"), None);
    }

    #[test]
    fn vowel_length_errors() {
        // REP-SPELL-002, REP-SPELL-003, REP-DIAC-011: the rules' own examples.
        for (wrong, right) in [
            ("mindíg", "mindig"),
            ("gyógyúl", "gyógyul"),
            ("végűl", "végül"),
            ("vízet", "vizet"),
            ("kapú", "kapu"),
            ("rendör", "rendőr"),
            ("öszinte", "őszinte"),
            ("elöre", "előre"),
            ("kertböl", "kertből"),
            ("ütö", "ütő"),
            ("elött", "előtt"),
            ("kerülö", "kerülő"),
        ] {
            assert_eq!(fixed(wrong).as_deref(), Some(right), "{wrong}");
        }
    }

    #[test]
    fn consonant_length_errors() {
        // REP-SPELL-004 and REP-SPELL-005, including the doubled digraphs.
        for (wrong, right) in [
            ("bizotság", "bizottság"),
            ("ezel", "ezzel"),
            ("hoszú", "hosszú"),
            ("visza", "vissza"),
            ("aszony", "asszony"),
        ] {
            assert_eq!(fixed(wrong).as_deref(), Some(right), "{wrong}");
        }
    }

    #[test]
    fn ocr_shape_confusions() {
        // REP-OCR-001.
        assert_eq!(fixed("rnegye").as_deref(), Some("megye"));
        assert_eq!(fixed("1ehet").as_deref(), Some("lehet"));
    }

    #[test]
    fn an_ill_formed_word_with_no_licensed_repair_abstains() {
        // `orrt` is forbidden by NOM-ACC-006, but no repair rule reaches `orrot`, and undoubling
        // would give `ort`, a different word. Abstaining is the only honest answer.
        assert_eq!(fixed("orrt"), None);
    }

    #[test]
    fn a_letter_does_not_inflect_without_a_hyphen() {
        // ORT-ABBR-009: `h` + the allative gives `hhez`, which must not be offered as a repair.
        assert_eq!(
            picur_morph_spell::check("hhez"),
            picur_morph_spell::Status::Wrong
        );
    }

    #[test]
    fn guarded_tokens_are_never_repaired() {
        // REP-DIAC-009: a repair here would be damage, not correction.
        for kept in [
            "felhasznalo@profinter.hu",
            "www.valamiceg.hu",
            "http://example.hu/oldal",
            "USA",
            "NATO",
            "HTML",
            "camelCase",
            "snake_case",
            "x1",
            "#hashtag",
            "path/to/file",
        ] {
            assert_eq!(fixed(kept), None, "{kept} was rewritten");
        }
        // The guard must not swallow ordinary words. `vasar` is accent restoration, which is
        // licensed by the segment rather than by the token (`REP-DIAC-001`), so the word-level
        // call abstains and the restoration itself still answers.
        assert!(super::restore_accents("vasar").is_some());
        assert!(fixed("rendör").is_some());
    }

    #[test]
    fn rep_diac_001_english_is_never_restored_as_hungarian() {
        // `to`, `for`, `all`, `art` and `real` all restore to real Hungarian words, so a
        // word-level restoration rewrites English. The segment decides.
        let english = "The report is for all of the real art in the city and we have to \
                       send it to the star of the show by the end of the day at the office \
                       so that the team can see it";
        assert!(!is_accent_stripped(english));
        assert_eq!(repair(english, Mode::Modernise), [] as [Fix; 0]);
        // The word alone is never restored either, whatever it could become.
        for w in ["to", "for", "all", "art", "real", "as", "at"] {
            assert_eq!(repair_word(w, Mode::Modernise), None, "{w}");
        }
    }

    #[test]
    fn rep_diac_001_a_stripped_hungarian_paragraph_is_restored() {
        let stripped = "Tegnap a varosban voltam es lattam, hogy mar nyitva van a vasar. \
                        Nagyon sok ember jott ossze, mert igy szoktak minden evben. \
                        A gyerekek is ott futkostak a teren, ahol a zenekar jatszott, \
                        es a szuloek csak neztek oket, amikor mar kezdett esteledni.";
        assert!(is_accent_stripped(stripped));
        let fixes = repair(stripped, Mode::Modernise);
        assert!(
            fixes.iter().any(|f| &*f.to == "és"),
            "no restoration among {:?}",
            fixes.iter().map(|f| (&f.from, &f.to)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn rep_diac_001_a_short_segment_abstains() {
        // The rule's own example: two tokens are not enough to decide.
        assert!(!is_accent_stripped("arvizturo tukorfurogep"));
    }

    #[test]
    fn rep_spell_016_three_identical_consonants() {
        assert_eq!(fixed("sakkkör").as_deref(), Some("sakk-kör"));
        assert_eq!(fixed("sakkör").as_deref(), Some("sakk-kör"));
        assert_eq!(fixed("balettáncos").as_deref(), Some("balett-táncos"));
        assert_eq!(fixed("toll-lal").as_deref(), Some("tollal"));
    }

    #[test]
    fn rep_spell_016_leaves_an_ordinary_doubled_consonant_alone() {
        // `orrt` is a misspelling of `orrot`, not a compound wanting a hyphen.
        for w in ["orrt", "tollal", "orra", "hallak"] {
            assert_ne!(
                fixed(w).as_deref(),
                Some(&*format!("{w}-")),
                "{w} sprouted a hyphen"
            );
        }
        assert_eq!(fixed("tollal"), None);
    }

    #[test]
    fn rep_spell_007_imperative_j_assimilation() {
        for (wrong, right) in [
            ("higyj", "higgy"),
            ("tanísd", "tanítsd"),
            ("fessd", "fesd"),
            ("mosjon", "mosson"),
            ("hozjon", "hozzon"),
            ("látsunk", "lássunk"),
        ] {
            assert_eq!(fixed(wrong).as_deref(), Some(right), "{wrong}");
        }
    }

    #[test]
    fn rep_spell_007_keeps_the_standard_minority_variant() {
        // The rule's own exception: `látsszék` is a current variant of `lássék`, not an error,
        // and the `metsz`/`látszik`/`tetszik` class allows `tssz` beside `ss`.
        for kept in ["látsszék", "tetszik", "látszik", "metsz"] {
            assert_eq!(fixed(kept), None, "{kept} was rewritten");
        }
    }
}

#[cfg(test)]
mod spacing_tests {
    use super::{Fix, Mode, apply, repair};

    fn repaired(text: &str) -> String {
        apply(text, &repair(text, Mode::Conservative))
    }

    #[test]
    fn rep_space_009_a_word_split_across_a_space_is_rejoined() {
        assert_eq!(repaired("a gyö nyörű kert"), "a gyönyörű kert");
        assert_eq!(repaired("a kor mány döntött"), "a kormány döntött");
    }

    /// A lost space around a function word is put back, and nothing else is cut.
    #[test]
    fn rep_space_009_a_fused_word_is_cut_at_the_function_word() {
        assert_eq!(repaired("ezértnem jött"), "ezért nem jött");
        assert_eq!(repaired("azegész napot"), "az egész napot");
    }

    /// The oracle decides, so text that is already right is left alone.
    #[test]
    fn rep_space_009_correct_spacing_is_untouched() {
        for text in [
            "nagyon jó volt",
            "a ház előtt állt",
            "nemcsak a kert",
            "egyelőre nem tudom",
        ] {
            assert_eq!(repaired(text), text, "{text}");
        }
    }

    /// A name is unlisted, not wrong, and `REP-SPACE-009` excepts it.
    #[test]
    fn rep_space_009_an_unfamiliar_name_is_not_taken_apart() {
        for text in ["Kovácsné Szabó", "Reykjavíkban jártam"] {
            assert_eq!(repaired(text), text, "{text}");
        }
    }

    /// Two names in a row are not one split word, however the second is suffixed.
    #[test]
    fn rep_space_009_two_names_are_not_joined() {
        for text in ["Elvis Presley-nek", "Lil Wayne-nek", "Elvis Presleynek"] {
            assert_eq!(repaired(text), text, "{text}");
        }
    }

    /// `REP-KEY-004` repairs a slip, not a name the lexicon does not list.
    #[test]
    fn rep_key_004_an_unlisted_name_is_not_a_layout_slip() {
        for text in ["Gipsy Kings", "Bonanza", "Banzai", "Eyed"] {
            assert_eq!(repaired(text), text, "{text}");
        }
        assert_eq!(repaired("nzelvtan"), "nyelvtan");
    }

    /// `REP-FONT-002`: a mark stacked on an accented letter is a double acute.
    #[test]
    fn rep_font_002_stacked_marks_are_repaired_inside_the_word() {
        assert_eq!(repaired("az erdô\u{30B}ben"), "az erdőben");
        assert_eq!(repaired("a kó\u{301} mellett"), "a kő mellett");
        assert_eq!(repaired("mǘködik"), "működik");
    }

    /// The fixes of a text never overlap, so a consumer may splice them without `apply`.
    #[test]
    fn fixes_are_disjoint_and_ordered() {
        let text = "Gyö nyörû volt, elõdöntõ lesz";
        let fixes = repair(text, Mode::Modernise);
        assert_ne!(fixes, [] as [Fix; 0]);
        let mut end = 0;
        for f in &fixes {
            assert!(f.at >= end, "{f:?}");
            end = f.at + f.from.len();
        }
    }
}
