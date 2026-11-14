//! Text normalisations of the writing system (`WS-NORM`).

/// The combining marks that compose onto a Hungarian vowel (`WS-NORM-002`).
const ACUTE: char = '\u{0301}';
const DIAERESIS: char = '\u{0308}';
const DOUBLE_ACUTE: char = '\u{030B}';

/// The precomposed letter for a base and a combining mark.
///
/// `WS-NORM-002` enumerates what composes to a Hungarian letter: `a e i o u` with an acute, `o u`
/// with a diaeresis, and `o u` with a double acute, uppercase alike. The dotless `ı` is not in the
/// rule's list because Unicode has no precomposed form for it, but LaTeX sets `í` that way and PDF
/// extraction hands it over as such, so it composes to the dotted letter here.
fn composed(base: char, mark: char) -> Option<char> {
    Some(match (base, mark) {
        ('a', ACUTE) => 'á',
        ('e', ACUTE) => 'é',
        ('i' | '\u{0131}', ACUTE) => 'í',
        ('o', ACUTE) => 'ó',
        ('u', ACUTE) => 'ú',
        ('o', DIAERESIS) => 'ö',
        ('u', DIAERESIS) => 'ü',
        ('o', DOUBLE_ACUTE) => 'ő',
        ('u', DOUBLE_ACUTE) => 'ű',
        ('A', ACUTE) => 'Á',
        ('E', ACUTE) => 'É',
        ('I', ACUTE) => 'Í',
        ('O', ACUTE) => 'Ó',
        ('U', ACUTE) => 'Ú',
        ('O', DIAERESIS) => 'Ö',
        ('U', DIAERESIS) => 'Ü',
        ('O', DOUBLE_ACUTE) => 'Ő',
        ('U', DOUBLE_ACUTE) => 'Ű',
        _ => return None,
    })
}

/// Composes decomposed Hungarian letters (`WS-NORM-001`, `WS-NORM-002`).
///
/// Every rule of the rulebook assumes NFC, and text does not always arrive that way: PDF
/// extraction, macOS file names and some input methods hand over a base letter followed by its
/// combining mark. Left alone, every word carrying one is rejected, because `é` written as `e`
/// plus U+0301 is not the letter the lexicon holds.
///
/// Only the Hungarian compositions are made, which is what `WS-NORM-002` lists; a mark that does
/// not compose to a Hungarian letter is left as it stands for `WS-NORM-004` to judge.
#[must_use]
pub fn compose(text: &str) -> String {
    if !text
        .chars()
        .any(|c| matches!(c, ACUTE | DIAERESIS | DOUBLE_ACUTE))
    {
        return text.to_owned();
    }
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match (out.chars().next_back(), c) {
            (Some(base), mark) if matches!(mark, ACUTE | DIAERESIS | DOUBLE_ACUTE) => {
                if let Some(done) = composed(base, mark) {
                    out.pop();
                    out.push(done);
                } else {
                    out.push(mark);
                }
            }
            _ => out.push(c),
        }
    }
    out
}

/// Whether `c` is a combining mark, or the spacing double acute a dead key leaves behind.
#[must_use]
pub fn is_mark(c: char) -> bool {
    matches!(c, '\u{0300}'..='\u{036F}' | '\u{02DD}')
}

/// The `ő` or `ű` a letter stacked with a further mark renders as (`REP-FONT-002`).
///
/// Whatever accent the letter already carries, a mark on top of an `o` or a `u` is a font's way
/// of drawing the double acute, and a mark on top of `ő` or `ű` is a stray one.
fn stacked(base: char, mark: char) -> Option<char> {
    if !matches!(mark, ACUTE | DIAERESIS | DOUBLE_ACUTE | '\u{02DD}') {
        return None;
    }
    Some(match base {
        'o' | 'ó' | 'ö' | 'ő' | 'ô' | 'õ' | 'ò' | 'ǒ' | 'ŏ' => 'ő',
        'u' | 'ú' | 'ü' | 'ű' | 'û' | 'ù' | 'ǔ' | 'ŭ' | 'ǘ' | 'ǜ' => 'ű',
        'O' | 'Ó' | 'Ö' | 'Ő' | 'Ô' | 'Õ' | 'Ò' | 'Ǒ' | 'Ŏ' => 'Ő',
        'U' | 'Ú' | 'Ü' | 'Ű' | 'Û' | 'Ù' | 'Ǔ' | 'Ŭ' | 'Ǘ' | 'Ǜ' => 'Ű',
        _ => return None,
    })
}

/// Resolves marks stacked on an `o` or `u` to `ő` and `ű` (`REP-FONT-002`).
///
/// NFC composes a mark onto a bare vowel; a mark on a vowel that is already accented stays
/// decomposed, and that is what a font substitution or a PDF extractor leaves where a double acute
/// was: `ó` plus an acute, `ö` plus an acute, `ô` plus a double acute. `ǘ` is the one such stack
/// NFC does compose, so it is read the same way. Other marks are left as they stand.
#[must_use]
pub fn unstack(text: &str) -> String {
    if !text
        .chars()
        .any(|c| is_mark(c) || matches!(c, 'ǘ' | 'ǜ' | 'Ǘ' | 'Ǜ'))
    {
        return text.to_owned();
    }
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match out.chars().next_back().and_then(|base| stacked(base, c)) {
            Some(done) => {
                out.pop();
                out.push(done);
            }
            None => out.push(match c {
                'ǘ' | 'ǜ' => 'ű',
                'Ǘ' | 'Ǜ' => 'Ű',
                other => other,
            }),
        }
    }
    out
}

/// The letters that differ from `c` only in accent, `c` itself first.
///
/// `WS-ALPH-003`: the seven vowel letters come in short and long pairs, and `o` and `u` each
/// head a family of four once the umlaut is counted.
pub(crate) fn accent_family(c: char) -> &'static [char] {
    match c {
        'a' | 'á' => &['a', 'á'],
        'e' | 'é' => &['e', 'é'],
        'i' | 'í' => &['i', 'í'],
        'o' | 'ó' | 'ö' | 'ő' => &['o', 'ó', 'ö', 'ő'],
        'u' | 'ú' | 'ü' | 'ű' => &['u', 'ú', 'ü', 'ű'],
        'A' | 'Á' => &['A', 'Á'],
        'E' | 'É' => &['E', 'É'],
        'I' | 'Í' => &['I', 'Í'],
        'O' | 'Ó' | 'Ö' | 'Ő' => &['O', 'Ó', 'Ö', 'Ő'],
        'U' | 'Ú' | 'Ü' | 'Ű' => &['U', 'Ú', 'Ü', 'Ű'],
        _ => &[],
    }
}

/// `word` with every vowel reduced to the bare letter of its family: `kívülről` and `kivulrol`
/// share the skeleton `kivulrol`.
///
/// Two words with one skeleton differ only in accents, which is the commonest damage Hungarian
/// text suffers (`REP-DIAC`).
#[must_use]
pub fn skeleton(word: &str) -> String {
    word.chars()
        .map(|c| accent_family(c).first().copied().unwrap_or(c))
        .collect()
}

/// Rewrites 7-bit accent notations as accented letters (`WS-NORM-006`).
///
/// `a'` -> `á`, `o:` -> `ö`, `o"` -> `ő`, `o''` -> `ő`, and the same for the other vowels:
/// `a'rvi'ztu"ro" tu:ko:rfu'ro'ge'p` -> `árvíztűrő tükörfúrógép`.
///
/// The notation is a whole-text convention, and the conventions conflict: where this one reads
/// `o"` as `ő`, the SZTE convention writes `o~` for `ö` and leaves `o"` for `ő`. A caller decides
/// from the whole text whether one notation is in consistent use, and validates the result
/// against the lexicon, before applying this.
#[must_use]
pub fn expand_accents(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < chars.len() {
        let next = chars.get(i + 1).copied();
        // `o''` is the two-apostrophe spelling of the double acute, so it must be read before the
        // single apostrophe turns `o'` into `ó`.
        let doubled = next == Some('\'') && chars.get(i + 2) == Some(&'\'');
        let mark = if doubled { Some('"') } else { next };
        if let Some(mark) = mark
            && let Some(accented) = accent(chars[i], mark)
        {
            out.push(accented);
            i += if doubled { 3 } else { 2 };
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

/// The accented letter a vowel plus an ASCII mark stands for (`WS-NORM-006`).
fn accent(vowel: char, mark: char) -> Option<char> {
    let upper = vowel.is_uppercase();
    let low = vowel.to_lowercase().next()?;
    let accented = match (low, mark) {
        ('a', '\'') => 'á',
        ('e', '\'') => 'é',
        ('i', '\'') => 'í',
        ('o', '\'') => 'ó',
        ('u', '\'') => 'ú',
        ('o', ':') => 'ö',
        ('u', ':') => 'ü',
        ('o', '"') => 'ő',
        ('u', '"') => 'ű',
        _ => return None,
    };
    if upper {
        accented.to_uppercase().next()
    } else {
        Some(accented)
    }
}

#[cfg(test)]
mod tests {
    use super::{compose, expand_accents, skeleton, unstack};

    #[test]
    fn a_skeleton_strips_every_accent() {
        assert_eq!(skeleton("kívülről"), "kivulrol");
        assert_eq!(skeleton("Őrség"), "Orseg");
        assert_eq!(skeleton("brr"), "brr");
    }

    #[test]
    fn rep_font_002_stacked_marks_render_the_double_acute() {
        assert_eq!(unstack("ko\u{301}\u{301}"), "kő");
        assert_eq!(unstack("kó\u{301}"), "kő");
        assert_eq!(unstack("elö\u{301}"), "elő");
        assert_eq!(unstack("kő\u{301}"), "kő");
        assert_eq!(unstack("erdô\u{30B}ben"), "erdőben");
        assert_eq!(unstack("mǘködik"), "működik");
        assert_eq!(unstack("o\u{2DD}r"), "őr");
        assert_eq!(unstack("Ö\u{301}r"), "Őr");
        assert_eq!(unstack("x\u{302}"), "x\u{302}");
    }

    #[test]
    fn ws_norm_001_decomposed_input_composes() {
        assert_eq!(compose("e\u{301}s"), "és");
        assert_eq!(
            compose("sza\u{301}mı\u{301}to\u{301}ge\u{301}pes"),
            "számítógépes"
        );
        assert_eq!(compose("to\u{308}ro\u{30B}"), "törő");
        assert_eq!(compose("A\u{301}RVI\u{301}Z"), "ÁRVÍZ");
    }

    #[test]
    fn ws_norm_002_a_mark_that_is_not_hungarian_is_left_alone() {
        // `n` plus a tilde is not a Hungarian letter, and `WS-NORM-004` judges it, not this.
        assert_eq!(compose("n\u{303}"), "n\u{303}");
        assert_eq!(compose("ház"), "ház");
    }

    #[test]
    fn ws_norm_006_every_notation_in_the_table() {
        for (input, want) in [
            ("a'", "á"),
            ("e'", "é"),
            ("i'", "í"),
            ("o'", "ó"),
            ("u'", "ú"),
            ("o:", "ö"),
            ("u:", "ü"),
            ("o\"", "ő"),
            ("u\"", "ű"),
            ("o''", "ő"),
        ] {
            assert_eq!(expand_accents(input), want, "{input}");
        }
    }

    #[test]
    fn ws_norm_006_the_pangram_round_trips() {
        assert_eq!(
            expand_accents("a'rvi'ztu\"ro\" tu:ko:rfu'ro'ge'p"),
            "árvíztűrő tükörfúrógép"
        );
    }

    #[test]
    fn ws_norm_006_case_is_kept_and_other_text_is_untouched() {
        assert_eq!(expand_accents("A'rvi'z"), "Árvíz");
        assert_eq!(expand_accents("Peto\"fi"), "Petőfi");
        assert_eq!(expand_accents("ház"), "ház");
        assert_eq!(expand_accents("Kovács'"), "Kovács'");
    }
}
