//! The definite article, chosen by the pronunciation of what follows it.
//!
//! `PND-ART-001` states the rule in one line: `az` before a word pronounced with an initial vowel,
//! `a` before one pronounced with an initial consonant. Everything here is the consequence of
//! "pronounced", because the written form is often not the spoken one: a number is read as its
//! spelled-out name, a letter as its letter name, an acronym as either.
//!
//! Where the reading is lexical the answer is [`None`], following `acronym`: this module does not
//! invent a pronunciation it does not have.
//!
//! See `docs/grammar/04-pronouns-numerals-determiners.md`.

/// A definite article allomorph.
///
/// The article never inflects and never agrees, so this is the whole paradigm (`PND-ART-001`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Article {
    /// Before a consonant-initial pronunciation.
    A,
    /// Before a vowel-initial pronunciation.
    Az,
}

impl Article {
    /// The written form.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A => "a",
            Self::Az => "az",
        }
    }

    /// The article for a pronunciation that begins with `c`.
    #[must_use]
    pub fn of_onset(c: char) -> Self {
        if is_vowel(c) { Self::Az } else { Self::A }
    }
}

/// Whether `c` is a Hungarian vowel letter, in either case.
fn is_vowel(c: char) -> bool {
    "aáeéiíoóöőuúüűAÁEÉIÍOÓÖŐUÚÜŰ".contains(c)
}

/// The article before the spoken form `reading`.
///
/// The escape hatch for everything this module refuses to decide: a caller that knows how an
/// acronym or a foreign word is pronounced gets its answer here.
///
/// Rules: `PND-ART-001`, `PND-ART-002`
#[must_use]
pub fn of_reading(reading: &str) -> Option<Article> {
    reading.chars().next().map(Article::of_onset)
}

/// Letters whose Hungarian name begins with a vowel, so they take `az`.
///
/// The `PND-ART-005` table, stored as the exceptions rather than as 44 names: every other letter
/// is named with its own sound plus `é` or `á` and so takes `a`.
const VOWEL_NAMED: &[&str] = &["f", "l", "ly", "m", "n", "ny", "r", "s", "sz", "x", "y"];

/// The article before a letter cited as a symbol, by the letter's Hungarian name.
///
/// Rules: `PND-ART-005`
#[must_use]
pub fn of_letter(letter: &str) -> Option<Article> {
    let lower = letter.to_lowercase();
    if lower.chars().count() == 1 && is_vowel(lower.chars().next()?) {
        return Some(Article::Az);
    }
    if !is_letter(&lower) {
        return None;
    }
    Some(if VOWEL_NAMED.contains(&lower.as_str()) {
        Article::Az
    } else {
        Article::A
    })
}

/// Every letter of the Hungarian alphabet, plus the four borrowed ones.
const LETTERS: &[&str] = &[
    "dzs", "cs", "dz", "gy", "ly", "ny", "sz", "ty", "zs", "a", "á", "b", "c", "d", "e", "é", "f",
    "g", "h", "i", "í", "j", "k", "l", "m", "n", "o", "ó", "ö", "ő", "p", "q", "r", "s", "t", "u",
    "ú", "ü", "ű", "v", "w", "x", "y", "z",
];

/// Whether `s` is exactly one Hungarian letter, counting the digraphs and `dzs`.
fn is_letter(s: &str) -> bool {
    LETTERS.contains(&s)
}

/// The article before an integer written in digits.
///
/// `PND-ART-003` derives it from the spelled-out form without building it: the first word is
/// `egy`, `öt` or `ezer` exactly in the cases below. `n mod 3 == 1` is what picks out the lengths
/// where a leading `1` is read `egy`- or `ezer` rather than `tizen-` or `száz-`.
///
/// Rules: `PND-ART-003`
#[must_use]
pub fn of_digits(digits: &str) -> Option<Article> {
    let d: String = digits.chars().filter(char::is_ascii_digit).collect();
    let d = d.trim_start_matches('0');
    if d.is_empty() || digits.starts_with(['-', '\u{2212}']) {
        return Some(Article::A);
    }
    let n = d.len();
    let first = d.as_bytes()[0];
    Some(if first == b'5' || (first == b'1' && n % 3 == 1) {
        Article::Az
    } else {
        Article::A
    })
}

/// The value of a Roman numeral, or `None` if `s` is not one.
fn roman(s: &str) -> Option<u32> {
    let value = |c: char| match c {
        'I' => Some(1),
        'V' => Some(5),
        'X' => Some(10),
        'L' => Some(50),
        'C' => Some(100),
        'D' => Some(500),
        'M' => Some(1000),
        _ => None,
    };
    if s.is_empty() {
        return None;
    }
    let digits: Option<Vec<u32>> = s.chars().map(value).collect();
    let digits = digits?;
    let mut total = 0;
    for (i, &v) in digits.iter().enumerate() {
        if digits[i + 1..].iter().any(|&w| w > v) {
            total -= i32::try_from(v).ok()?;
        } else {
            total += i32::try_from(v).ok()?;
        }
    }
    u32::try_from(total).ok()
}

/// Characters that carry no pronunciation, so the article looks past them.
///
/// Rules: `PND-ART-008`
fn silent(c: char) -> bool {
    c.is_whitespace()
        || matches!(
            c,
            '„' | '”'
                | '»'
                | '«'
                | '"'
                | '\''
                | '’'
                | '('
                | '['
                | '{'
                | '–'
                | '—'
                | '-'
                | '*'
                | '_'
        )
}

/// The definite article before the text `next`.
///
/// Returns `None` when the choice needs a pronunciation that no rule fixes: an acronym that could
/// be read out or as a word (`PND-ART-006`), an abbreviation standing for a word we do not have
/// the expansion of (`PND-ART-007`), or a foreign spelling (`PND-ART-002`).
///
/// Rules: `PND-ART-001`, `PND-ART-003`, `PND-ART-004`, `PND-ART-005`, `PND-ART-008`
#[must_use]
pub fn article(next: &str) -> Option<Article> {
    // A sign before a number is pronounced (`mínusz`, `plusz`), unlike the dash that introduces a
    // line of dialogue. Only the digit after it tells them apart, so this is decided first.
    let mut signed = next.trim_start().chars();
    if signed
        .next()
        .is_some_and(|c| matches!(c, '-' | '\u{2212}' | '+'))
        && signed.next().is_some_and(|c| c.is_ascii_digit())
    {
        return Some(Article::A);
    }
    let next = next.trim_start_matches(silent);
    let token: &str = next.split_whitespace().next()?;
    let bare = token.trim_end_matches(|c: char| !c.is_alphanumeric());

    if bare.starts_with(|c: char| c.is_ascii_digit()) {
        // Only the leading run of digits is one number word. `1.2.3` is read `egy pont kettő`,
        // `1848-49` is `ezernyolcszáz...`, and a grouped number's first group already decides,
        // because grouping is by three and so the group's length matches the whole modulo three.
        let head: String = bare.chars().take_while(char::is_ascii_digit).collect();
        // An ordinal reads the same way as its cardinal for this purpose: `első` and `egy` both
        // give `az`, `második` and `kettő` both give `a` (`PND-ART-004`).
        return of_digits(&head);
    }
    if token.ends_with('.')
        && let Some(v) = roman(bare)
    {
        return of_digits(&v.to_string());
    }
    // A structured reference is read part by part and the first part decides: `a III-147. cikk`
    // begins `a harmadik`. Which reading the numeral has does not matter, because a Roman
    // numeral's cardinal and ordinal take the same article.
    if let Some((head, rest)) = bare.split_once('-')
        && let Some(v) = roman(head)
        && rest.starts_with(|c: char| c.is_ascii_digit())
    {
        return of_digits(&v.to_string());
    }
    if is_letter(&bare.to_lowercase()) {
        return of_letter(bare);
    }
    // An all-capital string is an acronym; where its shape fixes the reading (`PH-G2P-011`) the
    // article follows the spoken form (`az OTP`, `a MÁV`), and otherwise the reading is lexical.
    if bare.chars().count() > 1 && bare.chars().all(|c| c.is_uppercase() || c.is_ascii_digit()) {
        return crate::acronym::spoken(bare)
            .and_then(|s| s.chars().next())
            .map(Article::of_onset);
    }
    bare.chars().next().map(Article::of_onset)
}

#[cfg(test)]
mod tests {
    use super::{Article, article, of_digits, of_letter, of_reading};

    fn a(next: &str) -> Option<&'static str> {
        article(next).map(Article::as_str)
    }

    #[test]
    fn ordinary_words_go_by_their_initial_letter() {
        // PND-ART-001.
        assert_eq!(a("ház"), Some("a"));
        assert_eq!(a("alma"), Some("az"));
        assert_eq!(a("ember"), Some("az"));
        assert_eq!(a("őz"), Some("az"));
        assert_eq!(a("üveg"), Some("az"));
        assert_eq!(a("macska felmászott"), Some("a"));
    }

    #[test]
    fn digits_follow_the_spelled_out_number() {
        // PND-ART-003, with the rulebook's own example list.
        for (n, want) in [
            ("1", "az"),
            ("5", "az"),
            ("50", "az"),
            ("500", "az"),
            ("1848", "az"),
            ("1956", "az"),
            ("5000", "az"),
            ("55000", "az"),
            ("1200000", "az"),
            ("2", "a"),
            ("10", "a"),
            ("15", "a"),
            ("100", "a"),
            ("150", "a"),
            ("12000", "a"),
            ("180000", "a"),
            ("2024", "a"),
            ("0", "a"),
        ] {
            assert_eq!(a(n), Some(want), "{n}");
        }
        assert_eq!(a("-5"), Some("a"));
    }

    #[test]
    fn grouped_digits_read_the_same() {
        // PUN-NUMF-002 grouping is not pronounced.
        assert_eq!(of_digits("55 000"), Some(Article::Az));
        assert_eq!(of_digits("1 200 000"), Some(Article::Az));
        assert_eq!(of_digits("180 000"), Some(Article::A));
    }

    #[test]
    fn suffixed_and_ordinal_digits_keep_the_cardinal_reading() {
        // PND-ART-004: `első` and `egy` agree, `második` and `kettő` agree.
        assert_eq!(a("1848-as"), Some("az"));
        assert_eq!(a("1956-os"), Some("az"));
        assert_eq!(a("2024-es"), Some("a"));
        assert_eq!(a("5."), Some("az"));
        assert_eq!(a("2."), Some("a"));
        assert_eq!(a("12-es"), Some("a"));
    }

    #[test]
    fn a_structured_reference_is_read_from_its_first_part() {
        // PND-ART-001: `III-147.` begins `harmadik`, so the article is `a`, not the `az` the
        // letter `I` would ask for.
        assert_eq!(a("III-147. cikk"), Some("a"));
        assert_eq!(a("III-419. cikkben"), Some("a"));
        assert_eq!(a("I-5. cikk"), Some("az"));
        assert_eq!(a("X-2. cikk"), Some("a"));
    }

    #[test]
    fn roman_ordinals_read_as_their_value() {
        // PND-ART-004.
        assert_eq!(a("V."), Some("az"));
        assert_eq!(a("IV."), Some("a"));
        assert_eq!(a("XX."), Some("a"));
        assert_eq!(a("I."), Some("az"));
        assert_eq!(a("L."), Some("az"));
    }

    #[test]
    fn letters_follow_their_hungarian_names() {
        // PND-ART-005, with the rulebook's own examples.
        assert_eq!(a("f)"), Some("az"));
        assert_eq!(a("m"), Some("az"));
        assert_eq!(a("x"), Some("az"));
        assert_eq!(a("y"), Some("az"));
        assert_eq!(a("s"), Some("az"));
        assert_eq!(a("A"), Some("az"));
        assert_eq!(a("b)"), Some("a"));
        assert_eq!(a("c)"), Some("a"));
        assert_eq!(a("h"), Some("a"));
        assert_eq!(a("k"), Some("a"));
        assert_eq!(a("z"), Some("a"));
        assert_eq!(of_letter("ly"), Some(Article::Az));
        assert_eq!(of_letter("zs"), Some(Article::A));
        assert_eq!(of_letter("cs"), Some(Article::A));
    }

    #[test]
    fn punctuation_before_the_word_is_transparent() {
        // PND-ART-008.
        assert_eq!(a("„alma”"), Some("az"));
        assert_eq!(a("(ember)"), Some("az"));
        assert_eq!(a("  ház"), Some("a"));
        assert_eq!(a("„ház”"), Some("a"));
    }

    #[test]
    fn an_acronym_follows_its_reading() {
        // PND-ART-006, PH-G2P-011: `NATO` is read as a word and `MTA` letter by letter, which
        // the shape decides.
        assert_eq!(a("MTA"), Some("az"));
        assert_eq!(a("NATO"), Some("a"));
        assert_eq!(a("ELTE"), Some("az"));
        assert_eq!(a("OTP"), Some("az"));
        assert_eq!(a("MÁV"), Some("a"));
        assert_eq!(a("KSH"), Some("a"));
        assert_eq!(of_reading("emtéá"), Some(Article::Az));
        assert_eq!(of_reading("nátó"), Some(Article::A));
    }

    #[test]
    fn nothing_is_not_a_word() {
        assert_eq!(a(""), None);
        assert_eq!(a("   "), None);
    }
}
