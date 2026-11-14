//! Characters that stand for a Hungarian letter without being one, and the code pages behind them.
//!
//! Every mechanism here is keyed on a character the Hungarian alphabet does not have, so a token
//! spelled with letters alone never reaches any of them. What the glyph was meant to be is then
//! decided by the checker, never by the glyph.

use picur_morph_lexicon::frequency;
use picur_morph_ortho::encoding::{demojibake, recode};
use picur_morph_ortho::norm::{compose, unstack};
use picur_morph_ortho::{Letter, letters};

use crate::word::expand_compatibility;
use crate::{is_hungarian_spelling, vouched_as_written};

use std::sync::LazyLock;

use crate::tables::{char_chars, char_strs};

static LIGATURE_SLOTS: LazyLock<Vec<(char, &'static str)>> =
    LazyLock::new(|| char_strs("LigatureSlot"));
static EARLY_PRINT: LazyLock<Vec<(char, &'static str)>> = LazyLock::new(|| char_strs("EarlyPrint"));
static SPACING_ACCENTS: LazyLock<Vec<(char, char)>> = LazyLock::new(|| char_chars("SpacingAccent"));
static FONT_SHIFT: LazyLock<Vec<(char, char)>> = LazyLock::new(|| char_chars("FontShift"));
static DOUBLE_ACUTE: LazyLock<Vec<(char, char)>> = LazyLock::new(|| char_chars("DoubleAcute"));
static CWI2: LazyLock<Vec<(char, char)>> = LazyLock::new(|| char_chars("Cwi2"));
static HOMOGLYPHS: LazyLock<Vec<(char, char)>> = LazyLock::new(|| char_chars("Homoglyph"));
static US_LAYOUT: LazyLock<Vec<(char, char)>> = LazyLock::new(|| char_chars("UsLayout"));
/// How a mechanism settles on one of its proposals.
#[derive(Clone, Copy)]
enum Choice {
    /// The first proposal the checker accepts; the proposals are ordered by likelihood.
    First,
    /// The only proposal the checker accepts, ranked by frequency where several are and abstaining
    /// on a tie: what the character stood for is genuinely unknown, so a guess corrupts the word.
    Unique,
}

/// One character-level repair: what the token could have been, and the rule that says so.
struct Mechanism {
    /// The forms the token could have been written as. Nothing here consults the checker.
    propose: fn(&str) -> Vec<String>,
    /// How the proposals are narrowed to one.
    choice: Choice,
    /// The rule the mechanism implements.
    rule: &'static str,
}

const fn m(propose: fn(&str) -> Vec<String>, choice: Choice, rule: &'static str) -> Mechanism {
    Mechanism {
        propose,
        choice,
        rule,
    }
}

/// The mechanisms, in the order the chapter runs them.
///
/// Order is evidence, not taste. `REP-FONT-003`'s dotless `ı` stands for `ő` where
/// `REP-FONT-011`'s stands for `i`, and both land on a word the checker accepts (`kerülő` and
/// `kerüli`), so the font shift is asked first exactly as the two rules' Interactions say. The
/// code pages come last because they are the widest hypothesis.
///
/// Rules: `REP-ENC-015`, `REP-QUAL-011`
const MECHANISMS: &[Mechanism] = &[
    m(compatibility, Choice::First, "WS-CHAR-010"),
    m(math_letters, Choice::First, "REP-FONT-007"),
    m(escapes, Choice::First, "REP-ENC-013"),
    m(us_layout, Choice::First, "REP-KEY-003"),
    m(ligature_slots, Choice::First, "REP-FONT-005"),
    m(stacked_marks, Choice::First, "REP-FONT-002"),
    m(soft_hyphen, Choice::First, "REP-FONT-008"),
    m(replacement_hyphen, Choice::First, "REP-ENC-012"),
    m(line_break_hyphen, Choice::First, "REP-SPACE-004"),
    m(early_print, Choice::First, "REP-FONT-012"),
    m(dead_keys, Choice::First, "REP-KEY-006"),
    m(font_shift, Choice::First, "REP-FONT-003"),
    m(double_acute, Choice::First, "REP-FONT-001"),
    m(cwi2, Choice::First, "REP-ENC-009"),
    m(homoglyphs, Choice::First, "REP-FONT-011"),
    m(utf8_as_page, Choice::First, "WS-ENC-006"),
    m(page_as_page, Choice::First, "REP-ENC-008"),
    m(lost_letter, Choice::Unique, "REP-ENC-012"),
];

/// Fewest characters a token must have before a glyph of it is read as a letter.
///
/// A single character is a word of Hungarian, because the alphabet's letters have names, so a
/// mis-decoded byte standing alone (`š`) would be accepted as one (`Á`).
const MIN_WORD_CHARS: usize = 2;

/// Fewest characters a token must have before a whole code page is hypothesised for it.
///
/// A page pair rewrites every character, so a short token reaches a word by accident (`šs` as
/// `Ás`). Characters and not letters, because a mis-decoded word is mostly not letters at all
/// (`szĂłtĂˇr`).
const MIN_PAGE_CHARS: usize = 3;

/// The repair of a token's character-level damage, if the checker accepts one.
///
/// The lexicon decides both ends. Two gates carry every mechanism below. The token must be spelled with something the alphabet
/// does not have, which is what damage leaves behind and a correct word never has; and the lexicon
/// must not vouch for it as written, so nothing that is already a word is rewritten. A proposal is
/// kept on the same test: acceptance alone would not do at either end, because the checker reads
/// any hyphenated pair as a compound, `edz-dze` and `Piros-fehér` alike.
///
/// This runs before the code guard of `REP-DIAC-009`. Misdecoded UTF-8 *writes* the internal
/// capitals the guard keys on (`használatához` read as ISO-8859-2 is `hasznĂĄlatĂĄhoz`), and an
/// all-capital token with a substitute glyph in it is a damaged word rather than an acronym.
///
/// Rules: `REP-ENC-015`, `REP-QUAL-011`
pub(crate) fn repair_glyphs(word: &str) -> Option<(String, &'static str)> {
    if is_hungarian_spelling(word)
        || word.chars().count() < MIN_WORD_CHARS
        || vouched_as_written(unquoted(word))
    {
        return None;
    }
    for mech in MECHANISMS {
        let mut taken: Vec<String> = Vec::new();
        for candidate in (mech.propose)(word) {
            if candidate != word
                && vouched_as_written(unquoted(&candidate))
                && !taken.contains(&candidate)
            {
                taken.push(candidate);
            }
        }
        let found = match mech.choice {
            Choice::First => taken.into_iter().next(),
            Choice::Unique => unique(taken),
        };
        if let Some(found) = found {
            return Some((found, mech.rule));
        }
    }
    None
}

/// `word` without the quotation marks, dash and ellipsis a mis-decoded token carries with it
/// (`REP-ENC-002`: `â€žIgenâ€` is `„Igen”`), which the checker does not read.
fn unquoted(word: &str) -> &str {
    word.trim_matches([
        '\u{201E}', '\u{201D}', '\u{00BB}', '\u{00AB}', '\u{2026}', '\u{2013}',
    ])
}

/// The commonest of several accepted proposals, or nothing when the top two are equally attested.
///
/// Rules: `REP-DIAC-008`
fn unique(mut taken: Vec<String>) -> Option<String> {
    // The frequency table holds case-folded forms, so a capitalised candidate is weighed by its
    // lowercase form.
    let weight = |c: &str| frequency(c).max(frequency(&picur_morph_ortho::case::lower_initial(c)));
    taken.sort_by_key(|c| std::cmp::Reverse(weight(c)));
    if let [a, b, ..] = taken.as_slice()
        && weight(a) == weight(b)
    {
        return None;
    }
    taken.into_iter().next()
}

/// `text` with every character of `table` expanded.
fn expand(text: &str, table: &[(char, &str)]) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match table.iter().find(|(k, _)| *k == c) {
            Some((_, to)) => out.push_str(to),
            None => out.push(c),
        }
    }
    out
}

/// `text` with every character mapped through `table`.
fn map(text: &str, table: &[(char, char)]) -> String {
    text.chars()
        .map(|c| table.iter().find(|(k, _)| *k == c).map_or(c, |(_, to)| *to))
        .collect()
}

/// The Unicode compatibility code points, which are a fact rather than a judgement.
///
/// Rules: `WS-CHAR-010`
fn compatibility(word: &str) -> Vec<String> {
    vec![expand_compatibility(word)]
}

/// Rules: `REP-FONT-005`
fn ligature_slots(word: &str) -> Vec<String> {
    vec![expand(word, &LIGATURE_SLOTS)]
}

/// Rules: `REP-FONT-002`
fn stacked_marks(word: &str) -> Vec<String> {
    vec![unstack(word), compose(word)]
}

/// Rules: `REP-FONT-012`
fn early_print(word: &str) -> Vec<String> {
    let mut umlauted = String::with_capacity(word.len());
    for c in word.chars() {
        match (umlauted.chars().next_back(), c) {
            (Some(base), '\u{0364}') => {
                umlauted.pop();
                umlauted.push(match base {
                    'o' => 'ö',
                    'u' => 'ü',
                    'a' => 'ä',
                    other => other,
                });
            }
            _ => umlauted.push(c),
        }
    }
    vec![expand(&umlauted, &EARLY_PRINT), expand(word, &EARLY_PRINT)]
}

/// The combining mark a spacing accent stands for.
fn combining(c: char) -> Option<char> {
    SPACING_ACCENTS
        .iter()
        .find(|(k, _)| *k == c)
        .map(|(_, to)| *to)
}

/// Dead-key residue composed with the vowel it belongs to (`REP-KEY-006`).
///
/// A mark before the vowel (`˝ot`) and a mark after it (`o˝sz`) are the same slip, and nothing in
/// the character stream says which, so both are proposed. A mark that composes to no Hungarian
/// letter leaves the third proposal, the token without it, which is what the rule asks for.
fn dead_keys(word: &str) -> Vec<String> {
    if !word.chars().any(|c| combining(c).is_some()) {
        return Vec::new();
    }
    let mut forward = String::with_capacity(word.len());
    let mut backward = String::with_capacity(word.len());
    let mut dropped = String::with_capacity(word.len());
    let mut pending: Option<char> = None;
    for c in word.chars() {
        if let Some(mark) = combining(c) {
            pending = Some(mark);
            backward.push(mark);
            continue;
        }
        forward.push(c);
        if let Some(mark) = pending.take() {
            forward.push(mark);
        }
        backward.push(c);
        dropped.push(c);
    }
    vec![compose(&forward), compose(&backward), dropped]
}

/// Rules: `REP-FONT-003`
fn font_shift(word: &str) -> Vec<String> {
    if !word.contains('\u{0131}') && !word.contains('\u{0130}') {
        return Vec::new();
    }
    vec![map(word, &FONT_SHIFT)]
}

/// Rules: `REP-FONT-001`, `REP-ENC-006`
fn double_acute(word: &str) -> Vec<String> {
    vec![map(word, &DOUBLE_ACUTE)]
}

/// Rules: `REP-ENC-009`
fn cwi2(word: &str) -> Vec<String> {
    vec![map(word, &CWI2)]
}

/// Most look-alikes a word may carry before it counts as written in the other script.
const MAX_HOMOGLYPHS: usize = 3;

/// Rules: `REP-FONT-011`
fn homoglyphs(word: &str) -> Vec<String> {
    let n = word
        .chars()
        .filter(|c| HOMOGLYPHS.iter().any(|(k, _)| k == c))
        .count();
    if n == 0 || n > MAX_HOMOGLYPHS {
        return Vec::new();
    }
    vec![map(word, &HOMOGLYPHS)]
}

/// Hungarian keys typed under the US layout (`t[r` for `tőr`, `h'z` for `ház`).
///
/// Rules: `REP-KEY-003`
fn us_layout(word: &str) -> Vec<String> {
    // A hyphen inside a token is a hyphen (`egy-két`), not the key under `ü`.
    if word.contains('-') {
        return Vec::new();
    }
    let shifted = us_layout_text(word);
    if shifted == word {
        return Vec::new();
    }
    vec![shifted]
}

/// `text` with every US-layout key read as the Hungarian key in its place.
pub(crate) fn us_layout_text(text: &str) -> String {
    map(text, &US_LAYOUT)
}

/// The mathematical alphanumeric symbols read as the letters they style (`REP-FONT-007`).
///
/// The block is thirteen alphabets of fifty-two letters and five rows of ten digits, so the
/// plain letter is arithmetic on the code point rather than a table.
#[must_use]
pub fn plain_letters(text: &str) -> String {
    text.chars()
        .map(|c| {
            let cp = u32::from(c);
            match cp {
                0x1D400..=0x1D6A3 => {
                    let i = (cp - 0x1D400) % 52;
                    char::from_u32(if i < 26 { 0x41 + i } else { 0x61 + i - 26 }).unwrap_or(c)
                }
                0x1D7CE..=0x1D7FF => char::from_u32(0x30 + (cp - 0x1D7CE) % 10).unwrap_or(c),
                _ => c,
            }
        })
        .collect()
}

/// Rules: `REP-FONT-007`
fn math_letters(word: &str) -> Vec<String> {
    let plain = plain_letters(word);
    if plain == word {
        return Vec::new();
    }
    vec![plain]
}

/// Transfer encodings left in the text: quoted-printable `=C3=A9`, URL `%C5%91`, the JSON or
/// Java escape `\u0151` and the TeX accents `\H{o}`, `\'a` (`REP-ENC-013`).
#[must_use]
pub fn unescaped(word: &str) -> Option<String> {
    let mut bytes: Vec<u8> = Vec::with_capacity(word.len());
    let mut rest = word;
    let mut changed = false;
    while !rest.is_empty() {
        let mut taken = false;
        for lead in ["=", "%"] {
            if let Some(after) = rest.strip_prefix(lead)
                && let Some(hex) = after.get(..2)
                && let Ok(b) = u8::from_str_radix(hex, 16)
            {
                bytes.push(b);
                rest = &after[2..];
                changed = true;
                taken = true;
                break;
            }
        }
        if taken {
            continue;
        }
        if let Some(after) = rest.strip_prefix("\\u")
            && let Some(hex) = after.get(..4)
            && let Some(c) = u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)
        {
            let mut buf = [0u8; 4];
            bytes.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            rest = &after[4..];
            changed = true;
            continue;
        }
        if let Some((letter, len)) = tex_group(rest) {
            bytes.extend_from_slice(letter.as_bytes());
            rest = &rest[len..];
            changed = true;
            continue;
        }
        let c = rest.chars().next()?;
        let mut buf = [0u8; 4];
        bytes.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
        rest = &rest[c.len_utf8()..];
    }
    if !changed {
        return None;
    }
    String::from_utf8(bytes).ok().map(|t| compose(&t))
}

/// The combining mark a TeX accent command writes, as the plain TeX format defines it.
fn tex_mark(command: char) -> Option<char> {
    Some(match command {
        '\'' => '\u{0301}',
        '"' => '\u{0308}',
        '`' => '\u{0300}',
        '^' => '\u{0302}',
        '~' => '\u{0303}',
        '=' => '\u{0304}',
        '.' => '\u{0307}',
        'H' => '\u{030B}',
        'v' => '\u{030C}',
        'u' => '\u{0306}',
        'r' => '\u{030A}',
        'c' => '\u{0327}',
        'k' => '\u{0328}',
        _ => return None,
    })
}

/// A TeX accent at the start of `text`, optionally braced as a whole (`{\'a}`): the base letter
/// with its mark, and the bytes it spans.
fn tex_group(text: &str) -> Option<(String, usize)> {
    if let Some(inner) = text.strip_prefix('{')
        && let Some((letter, len)) = tex_accent(inner)
        && inner[len..].starts_with('}')
    {
        return Some((letter, len + 2));
    }
    tex_accent(text)
}

/// A TeX accent command at the start of `text` (`\'a`, `\"o`, `\H{o}`, `\'{\i}`).
///
/// A command named by a letter takes a braced argument, since `\Ho` is a macro of its own.
fn tex_accent(text: &str) -> Option<(String, usize)> {
    let after = text.strip_prefix('\\')?;
    let command = after.chars().next()?;
    let mark = tex_mark(command)?;
    let arg = &after[command.len_utf8()..];
    let (body, len) = match arg.strip_prefix('{') {
        Some(braced) => {
            let close = braced.find('}')?;
            (&braced[..close], close + 2)
        }
        None if command.is_ascii_alphabetic() => return None,
        None => {
            let n = if arg.starts_with("\\i") {
                2
            } else {
                arg.chars().next()?.len_utf8()
            };
            (&arg[..n], n)
        }
    };
    let base = if body == "\\i" {
        'i'
    } else {
        let mut cs = body.chars();
        let c = cs.next().filter(char::is_ascii_alphabetic)?;
        if cs.next().is_some() {
            return None;
        }
        c
    };
    Some((format!("{base}{mark}"), 1 + command.len_utf8() + len))
}

/// Rules: `REP-ENC-013`
fn escapes(word: &str) -> Vec<String> {
    unescaped(word).into_iter().collect()
}

/// Rules: `WS-ENC-006`, `REP-ENC-002`, `REP-ENC-003`, `REP-ENC-004`
fn utf8_as_page(word: &str) -> Vec<String> {
    if word.chars().count() < MIN_PAGE_CHARS {
        return Vec::new();
    }
    demojibake(word)
}

/// Rules: `REP-ENC-008`, `REP-ENC-010`
fn page_as_page(word: &str) -> Vec<String> {
    if word.chars().count() < MIN_PAGE_CHARS {
        return Vec::new();
    }
    recode(word)
}

/// The characters a break inside a word is written with.
///
/// `REP-FONT-008`'s soft hyphen marks a permitted break; `REP-ENC-012`'s replacement character is
/// what a decoder left where it could not read the hyphen at one.
const SOFT_HYPHEN: char = '\u{00AD}';
const REPLACEMENT: char = '\u{FFFD}';

/// Rules: `REP-FONT-008`
fn soft_hyphen(word: &str) -> Vec<String> {
    rejoin(word, SOFT_HYPHEN)
}

/// Rules: `REP-ENC-012`
fn replacement_hyphen(word: &str) -> Vec<String> {
    rejoin(word, REPLACEMENT)
}

/// A line-end hyphen inside a long multi-character letter (`REP-SPACE-004`, `REP-ARCH-007`).
///
/// Only that junction. A hyphen that is simply spurious is `REP-SPELL-012`, which decides it from
/// the host rather than from the break, so the plain join is offered here only where the break
/// itself is the evidence: `kulcs-csomó` writes its seam in full, `asz-szony` simplifies it.
fn line_break_hyphen(word: &str) -> Vec<String> {
    let Some((left, right)) = broken(word, '-') else {
        return Vec::new();
    };
    let mut out = simplified(left, right);
    if !out.is_empty() {
        out.push(format!("{left}{right}"));
    }
    out
}

/// The two halves of a word broken once at `marker`.
fn broken(word: &str, marker: char) -> Option<(&str, &str)> {
    let (left, right) = word.split_once(marker)?;
    (!left.is_empty() && !right.is_empty() && !right.contains(marker)).then_some((left, right))
}

/// The forms a word broken at `marker` could have been (`REP-SPACE-004`).
///
/// Deleting the marker is the plain join. Where the break falls inside a long multi-character
/// letter, which is written out in full on both lines, the simplified spelling has to be restored
/// instead: `asz` and `szony` are one `ssz`. Both are proposed and the checker chooses, which is
/// what keeps `kulcs` and `csomó` a compound seam.
fn rejoin(word: &str, marker: char) -> Vec<String> {
    let Some((left, right)) = broken(word, marker) else {
        return Vec::new();
    };
    let mut out = vec![format!("{left}{right}")];
    out.extend(simplified(left, right));
    out
}

/// The simplified spelling of a long multi-character letter written out on both sides of a break.
fn simplified(left: &str, right: &str) -> Vec<String> {
    let Some(span) = letters(left).last() else {
        return Vec::new();
    };
    let digraph = span.letter.as_str();
    if digraph.chars().count() < 2 {
        return Vec::new();
    }
    match (right.strip_prefix(digraph), digraph.chars().next()) {
        (Some(rest), Some(first)) => {
            vec![format!("{}{first}{digraph}{rest}", &left[..span.at])]
        }
        _ => Vec::new(),
    }
}

/// A letter lost to a decoder, restored where exactly one letter answers (`REP-ENC-012`).
///
/// The replacement character stands for bytes the decoder could not read, and in a Hungarian word
/// that is one letter. Every letter of the alphabet is tried; the rule allows the repair only when
/// one of them makes the word a word, so an ambiguous position keeps the character.
fn lost_letter(word: &str) -> Vec<String> {
    // `REP-ENC-011`: a strict decoder writes the lost byte as `?` as often as as U+FFFD.
    let lost = if word.matches(REPLACEMENT).count() == 1 {
        REPLACEMENT
    } else if word.matches('?').count() == 1 && !word.ends_with('?') {
        '?'
    } else {
        return Vec::new();
    };
    let word = &word.replace(lost, &REPLACEMENT.to_string());
    let initial = word.starts_with(REPLACEMENT);
    Letter::ALL
        .iter()
        .flat_map(|l| {
            let lower = l.as_str().to_owned();
            let upper = initial.then(|| lower.to_uppercase());
            [Some(lower), upper]
        })
        .flatten()
        .map(|letter| word.replace(REPLACEMENT, &letter))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::repair_glyphs;

    fn repaired(word: &str) -> Option<String> {
        repair_glyphs(word).map(|(w, _)| w)
    }

    /// `REP-FONT-001`, `REP-ENC-006`: a mark on an `o` or `u` that is not the double acute.
    #[test]
    fn rep_font_001_substitute_glyphs_become_the_double_acute() {
        for (damaged, want) in [
            ("elsô", "első"),
            ("elôtt", "előtt"),
            ("mûködik", "működik"),
            ("fũtés", "fűtés"),
            ("elõdöntõ", "elődöntő"),
            ("KÖVETKEZÔ", "KÖVETKEZŐ"),
            ("KÕSZEG", "KŐSZEG"),
        ] {
            assert_eq!(repaired(damaged).as_deref(), Some(want), "{damaged}");
        }
    }

    /// `REP-ENC-009`: CWI-2 read as CP437 shows the accented capitals as Latin-1 letters.
    #[test]
    fn rep_enc_009_the_cwi2_capitals_are_restored() {
        for (damaged, want) in [("ÅLLAT", "ÁLLAT"), ("ÒRA", "ÓRA"), ("Ùj", "Új")] {
            assert_eq!(repaired(damaged).as_deref(), Some(want), "{damaged}");
        }
    }

    /// A foreign word the repair would have to invent a Hungarian word for is left alone.
    #[test]
    fn rep_font_001_a_foreign_letter_is_not_a_substitute() {
        for word in ["hôtel", "Tõnu", "Guimarães", "flûte", "Ångström", "São"] {
            assert_eq!(repaired(word), None, "{word}");
        }
    }

    /// `REP-ENC-008`: one code page's bytes read with another's table.
    #[test]
    fn rep_enc_008_a_code_page_pair_is_inverted() {
        assert_eq!(
            repaired("k\u{201a}pvisel\u{2039}").as_deref(),
            Some("képviselő")
        );
        assert_eq!(repaired("K\u{251c}\u{119}rem").as_deref(), Some("Kérem"));
    }

    /// `REP-ENC-012`: the replacement character as a lost hyphen and as a lost letter.
    #[test]
    fn rep_enc_012_a_replacement_character_is_a_hyphen_or_a_letter() {
        for (damaged, want) in [
            ("vis\u{FFFD}sza", "vissza"),
            ("kavic\u{FFFD}csal", "kaviccsal"),
            ("v\u{FFFD}ros", "város"),
        ] {
            assert_eq!(repaired(damaged).as_deref(), Some(want), "{damaged}");
        }
    }

    /// `REP-FONT-008`: a soft hyphen is a break position, and the word is one word.
    #[test]
    fn rep_font_008_a_soft_hyphen_is_deleted() {
        for (damaged, want) in [
            ("szö\u{AD}veg", "szöveg"),
            ("vissza\u{AD}tér", "visszatér"),
            ("asz\u{AD}szony", "asszony"),
        ] {
            assert_eq!(repaired(damaged).as_deref(), Some(want), "{damaged}");
        }
    }

    /// `REP-SPACE-004`: a break inside a long multi-character letter restores the simplification.
    #[test]
    fn rep_space_004_a_broken_digraph_is_simplified() {
        for (damaged, want) in [
            ("edz-dze", "eddze"),
            ("bridzs-dzsel", "briddzsel"),
            ("asz-szony", "asszony"),
            ("kulcs-csomó", "kulcscsomó"),
        ] {
            assert_eq!(repaired(damaged).as_deref(), Some(want), "{damaged}");
        }
    }

    /// `REP-FONT-012`, `REP-FONT-005`: early print and a PDF producer's ligature slots.
    #[test]
    fn rep_font_012_early_print_glyphs_become_letters() {
        for (damaged, want) in [
            ("\u{17F}zép", "szép"),
            ("e\u{17F}ztendő", "esztendő"),
            ("ko\u{364}nyv", "könyv"),
            ("speci\u{F001}káció", "specifikáció"),
        ] {
            assert_eq!(repaired(damaged).as_deref(), Some(want), "{damaged}");
        }
    }

    /// `REP-KEY-006`: the mark a dead key left, on either side of its vowel.
    #[test]
    fn rep_key_006_dead_key_residue_composes() {
        for (damaged, want) in [
            ("\u{2DD}ot", "őt"),
            ("k\u{A8}onyv", "könyv"),
            ("\u{B4}allat", "állat"),
            ("o\u{2DD}sz", "ősz"),
        ] {
            assert_eq!(repaired(damaged).as_deref(), Some(want), "{damaged}");
        }
    }

    /// `REP-ENC-013`: TeX accent commands in their braced and bare forms.
    #[test]
    fn rep_enc_013_tex_accents_are_decoded() {
        for (damaged, want) in [
            ("\\H{o}r", "őr"),
            ("k\\\"onyv", "könyv"),
            ("{\\'a}ll", "áll"),
            ("v\\'{\\i}z", "víz"),
        ] {
            assert_eq!(
                super::unescaped(damaged).as_deref(),
                Some(want),
                "{damaged}"
            );
        }
        assert_eq!(super::unescaped("\\Hor"), None);
    }

    /// `REP-FONT-003`: the shifted font, inverted on the token that shows the dotless letter.
    #[test]
    fn rep_font_003_the_shifted_font_is_inverted() {
        assert_eq!(repaired("kerülı").as_deref(), Some("kerülő"));
    }

    /// `REP-FONT-011`: a single look-alike of another script.
    #[test]
    fn rep_font_011_a_homoglyph_becomes_its_latin_letter() {
        assert_eq!(repaired("Мagyarország").as_deref(), Some("Magyarország"));
        assert_eq!(repaired("mıért").as_deref(), Some("miért"));
    }

    /// Nothing here touches a word spelled with the alphabet, nor a token too short to judge.
    #[test]
    fn a_word_of_letters_is_never_a_glyph_repair() {
        for word in [
            "hoszszú",
            "kerülö",
            "rnegye",
            "ház",
            "árvíztűrő",
            "NATO",
            "š",
            "šs",
            "õ",
        ] {
            assert_eq!(repaired(word), None, "{word}");
        }
    }
}
