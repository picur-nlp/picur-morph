//! UTF-8 read as a single-byte code page, and back (`WS-ENC-006`).

use std::sync::LazyLock;

use encoding_rs::{Encoding, ISO_8859_2, MACINTOSH, WINDOWS_1250, WINDOWS_1252};
use oem_cp::code_table::{DECODING_TABLE_CP437, DECODING_TABLE_CP850, DECODING_TABLE_CP852};

/// How a page converts between bytes and characters.
enum Table {
    /// A page the WHATWG Encoding Standard covers.
    Whatwg(&'static Encoding),
    /// A DOS page given by its upper half; the lower half is ASCII.
    Dos(&'static [char; 128]),
    /// ISO-8859-1, whose bytes are the first 256 code points. The WHATWG label maps to
    /// Windows-1252, which writes U+2018 where this writes the C1 control U+0091.
    Latin1,
}

/// A code page Hungarian UTF-8 is commonly mis-decoded as (`WS-ENC-006`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    /// The commonest, since it is the Western European default.
    Windows1252,
    /// The Hungarian Windows code page.
    Windows1250,
    /// The Hungarian Unix code page.
    Iso88592,
    /// The Hungarian DOS code page, which shows a UTF-8 lead byte as box drawing.
    Cp852,
    /// The classic Mac OS page, which shows a UTF-8 lead byte as a mathematical symbol.
    MacRoman,
    /// The US DOS page, which shows CP852 `ű` as `√` (`REP-ENC-008`).
    Cp437,
    /// The Western European DOS page, which shares CP852's `Á Í Ó Ú` but not `ő ű`.
    Cp850,
    /// ISO-8859-1 proper, which keeps the C1 controls of UTF-8 continuation bytes (`REP-FONT-010`).
    Latin1,
}

impl Page {
    /// Every page this module handles, commonest first.
    pub const ALL: [Self; 8] = [
        Self::Windows1252,
        Self::Windows1250,
        Self::Iso88592,
        Self::Cp852,
        Self::MacRoman,
        Self::Cp437,
        Self::Cp850,
        Self::Latin1,
    ];

    fn table(self) -> Table {
        match self {
            Self::Windows1252 => Table::Whatwg(WINDOWS_1252),
            Self::Windows1250 => Table::Whatwg(WINDOWS_1250),
            Self::Iso88592 => Table::Whatwg(ISO_8859_2),
            Self::Cp852 => Table::Dos(&DECODING_TABLE_CP852),
            Self::MacRoman => Table::Whatwg(MACINTOSH),
            Self::Cp437 => Table::Dos(&DECODING_TABLE_CP437),
            Self::Cp850 => Table::Dos(&DECODING_TABLE_CP850),
            Self::Latin1 => Table::Latin1,
        }
    }

    /// `bytes` read as this page.
    fn decode(self, bytes: &[u8]) -> String {
        match self.table() {
            Table::Whatwg(enc) => enc.decode(bytes).0.into_owned(),
            Table::Dos(table) => bytes
                .iter()
                .map(|b| match b.checked_sub(0x80) {
                    Some(i) => table[usize::from(i)],
                    None => char::from(*b),
                })
                .collect(),
            Table::Latin1 => bytes.iter().copied().map(char::from).collect(),
        }
    }

    /// The bytes `text` would be, written in this page, or `None` if the page cannot write it.
    fn encode(self, text: &str) -> Option<Vec<u8>> {
        match self.table() {
            Table::Whatwg(enc) => {
                let (bytes, _, unmappable) = enc.encode(text);
                (!unmappable).then(|| bytes.into_owned())
            }
            Table::Dos(table) => text
                .chars()
                .map(|c| match u8::try_from(u32::from(c)) {
                    Ok(b) if b < 0x80 => Some(b),
                    _ => table
                        .iter()
                        .position(|t| *t == c)
                        .and_then(|i| u8::try_from(i).ok())
                        .map(|i| 0x80 + i),
                })
                .collect(),
            Table::Latin1 => text
                .chars()
                .map(|c| u8::try_from(u32::from(c)).ok())
                .collect(),
        }
    }
}

/// The lead characters a mis-decoded UTF-8 sequence starts with (`WS-ENC-006`).
///
/// A Hungarian letter is two UTF-8 bytes beginning C3 or C5, which every one of these pages shows
/// as one of these letters. Their absence means the text is not this kind of damage.
const LEADS: [char; 10] = ['Ã', 'Å', 'Ă', 'Ĺ', '├', '┼', '√', '≈', 'â', 'Â'];

/// Whether `c` is a character a mis-decoded Hungarian letter starts with.
#[must_use]
pub fn is_lead(c: char) -> bool {
    LEADS.contains(&c)
}

/// The second bytes of the UTF-8 Hungarian letters, `á` being C3 A1 and `ő` C5 91.
const TRAILS: [u8; 18] = [
    0x81, 0x89, 0x8D, 0x90, 0x91, 0x93, 0x96, 0x9A, 0x9C, 0xA1, 0xA9, 0xAD, 0xB0, 0xB1, 0xB3, 0xB6,
    0xBA, 0xBC,
];

/// Every character a code page shows one of the [`TRAILS`] as, sorted.
static CONTINUATIONS: LazyLock<Vec<char>> = LazyLock::new(|| {
    let mut out: Vec<char> = Page::ALL
        .iter()
        .flat_map(|p| TRAILS.iter().map(move |b| p.decode(&[*b]).chars().next()))
        .flatten()
        .collect();
    out.sort_unstable();
    out.dedup();
    out
});

/// Whether `c` can be the second character of a mis-decoded Hungarian letter.
///
/// Only meaningful after [`is_lead`]: on its own the set holds ordinary punctuation such as the
/// quotation marks and the en dash, which is exactly why a tokeniser must ask about the pair.
#[must_use]
pub fn is_continuation(c: char) -> bool {
    CONTINUATIONS.binary_search(&c).is_ok()
}

/// The mojibake a code page produces from UTF-8 Hungarian text (`WS-ENC-006`).
///
/// `előtt` read as Windows-1252 is `elÅ‘tt`. This is the damaging direction, for building the
/// broken side of a repair test set; [`demojibake`] is the inverse.
#[must_use]
pub fn mojibake(text: &str, page: Page) -> String {
    page.decode(text.as_bytes())
}

/// Re-decodes text that was written as UTF-8 and read as a single-byte code page (`WS-ENC-006`).
///
/// The candidates come back commonest page first. A page that cannot represent every character
/// of the input did not produce it, which is what keeps undamaged text and genuinely foreign
/// words out; but several pages often can, and they disagree (`hĹąvĂ¶s` reverses to `hűvös`
/// under ISO-8859-2 and to `hŹvÜs` under Windows-1250), so the caller decides by lexicon.
#[must_use]
pub fn demojibake(text: &str) -> Vec<String> {
    if !text.chars().any(|c| LEADS.contains(&c)) {
        return Vec::new();
    }
    // `REP-ENC-002`: a strict decoder drops the byte 0x9D, so a closing quote loses its last
    // byte and the sequence ends on the lead and `€`; the byte is put back before re-decoding.
    let restored: String;
    let text = if text.ends_with("\u{00E2}\u{20AC}") {
        restored = format!("{text}\u{009D}");
        &restored
    } else {
        text
    };
    let mut out = Vec::new();
    for page in Page::ALL {
        let Some(bytes) = page.encode(text) else {
            continue;
        };
        let Ok(decoded) = core::str::from_utf8(&bytes) else {
            continue;
        };
        keep(&mut out, text, decoded);
    }
    out
}

/// Re-decodes single-byte text written in one code page and read as another (`REP-ENC-008`).
///
/// The damage is a pair of tables rather than one: CP852 bytes read as Windows-1250 show `k‚pvisel‹`
/// for `képviselő`. The inverse encodes the text with the page that read it and decodes with the
/// page that wrote it, so every ordered pair of [`Page::ALL`] is a candidate and the caller decides
/// by lexicon, as in [`demojibake`].
#[must_use]
pub fn recode(text: &str) -> Vec<String> {
    if text.is_ascii() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for read_as in Page::ALL {
        let Some(bytes) = read_as.encode(text) else {
            continue;
        };
        for written_in in Page::ALL {
            if written_in == read_as {
                continue;
            }
            keep(&mut out, text, &written_in.decode(&bytes));
        }
    }
    out
}

/// Re-decodes Western single-byte text that was read as a Central European page (`REP-ENC-007`).
///
/// Latin-1 `ñ` and Latin-2 `ń` are the same byte, so `España` read as ISO-8859-2 is `Espańa`.
/// The inverse encodes with the page that read the text and decodes with Windows-1252. A
/// candidate keeps every letter a letter: a byte that is a letter in one page and a sign in the
/// other (`š` against `¹`) did not come from Western text.
#[must_use]
pub fn western(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for read_as in [Page::Iso88592, Page::Windows1250] {
        let Some(bytes) = read_as.encode(text) else {
            continue;
        };
        let decoded = Page::Windows1252.decode(&bytes);
        let letters_kept = text
            .chars()
            .zip(decoded.chars())
            .all(|(a, b)| a == b || (a.is_alphabetic() && b.is_alphabetic()));
        if decoded != text && letters_kept && !out.contains(&decoded) {
            out.push(decoded);
        }
    }
    out
}

/// The Hungarian letters a page cannot write, each with the letter a best-fit conversion writes
/// for it (`REP-ENC-011`): its base letter, the accent dropped.
///
/// Windows-1252 has 14 of the 18 accented letters, so `ő ű Ő Ű` come out `o u O U`.
#[must_use]
pub fn best_fit(page: Page) -> Vec<(char, char)> {
    "áéíóöőúüűÁÉÍÓÖŐÚÜŰ"
        .chars()
        .filter(|c| page.encode(&c.to_string()).is_none())
        .filter_map(|c| Some((c, *crate::norm::accent_family(c).first()?)))
        .collect()
}

/// Most letters [`unfit`] expands, which bounds its output at 2^n.
const MAX_UNFIT: usize = 8;

/// The spellings a best-fit conversion into `page` could have produced `text` from
/// (`REP-ENC-011`), `text` itself excluded.
///
/// Only the letters the page lost are candidates, so after Windows-1252 an `o` may have been `ő`
/// and nothing else, because `ó` and `ö` survived the conversion.
#[must_use]
pub fn unfit(text: &str, page: Page) -> Vec<String> {
    let lost = best_fit(page);
    let slots: Vec<(usize, char, char)> = text
        .char_indices()
        .filter_map(|(i, c)| {
            lost.iter()
                .find(|(_, base)| *base == c)
                .map(|(l, _)| (i, c, *l))
        })
        .take(MAX_UNFIT)
        .collect();
    (1..1usize << slots.len())
        .map(|mask| {
            let mut out = text.to_owned();
            for (k, (at, base, letter)) in slots.iter().enumerate().rev() {
                if mask & (1 << k) != 0 {
                    out.replace_range(*at..at + base.len_utf8(), &letter.to_string());
                }
            }
            out
        })
        .collect()
}

/// Keeps a candidate that is a change and carries a Hungarian accent or a Hungarian
/// punctuation mark (the quotes, the dash and the ellipsis of `REP-ENC-002`).
fn keep(out: &mut Vec<String>, text: &str, candidate: &str) {
    if candidate != text
        && candidate
            .chars()
            .any(|c| is_accented(c) || is_hungarian_mark(c))
        && !out.iter().any(|c| c == candidate)
    {
        out.push(candidate.to_owned());
    }
}

/// Whether the character is a typographic mark Hungarian text uses.
fn is_hungarian_mark(c: char) -> bool {
    matches!(
        c,
        '\u{201E}' | '\u{201D}' | '\u{00BB}' | '\u{00AB}' | '\u{2013}' | '\u{2026}'
    )
}

/// Whether the character is an accented Hungarian letter.
fn is_accented(c: char) -> bool {
    "áéíóöőúüűÁÉÍÓÖŐÚÜŰ".contains(c)
}

#[cfg(test)]
mod tests {
    use super::{Page, demojibake, mojibake, recode, unfit, western};

    #[test]
    fn rep_enc_007_the_reverse_table_comes_from_the_pages() {
        assert_eq!(western("Espańa"), ["España"]);
        assert_eq!(western("Săo"), ["São"]);
        assert_eq!(western("Leoš"), Vec::<String>::new());
        assert_eq!(western("ház"), Vec::<String>::new());
    }

    #[test]
    fn rep_enc_011_windows_1252_loses_the_double_acute_only() {
        assert_eq!(unfit("u", Page::Windows1252), ["ű"]);
        assert!(unfit("elotte", Page::Windows1252).contains(&"előtte".to_owned()));
        assert_eq!(unfit("ház", Page::Windows1252), Vec::<String>::new());
    }

    #[test]
    fn rep_enc_008_cp852_read_as_cp437() {
        assert!(recode("t\u{221A}z").contains(&"tűz".to_owned()));
    }

    #[test]
    fn rep_enc_004_the_cp852_column_of_table_t_enc_2() {
        for (letter, damaged) in [
            ("á", "├í"),
            ("é", "├ę"),
            ("ó", "├│"),
            ("ö", "├Â"),
            ("ő", "┼Ĺ"),
            ("ú", "├║"),
            ("ü", "├╝"),
            ("ű", "┼▒"),
            ("Ő", "┼É"),
        ] {
            assert_eq!(mojibake(letter, Page::Cp852), damaged, "{letter}");
            assert!(
                demojibake(damaged).contains(&letter.to_owned()),
                "{damaged}"
            );
        }
    }

    #[test]
    fn rep_enc_008_cp852_read_as_windows_1250() {
        assert!(recode("k\u{201a}pvisel\u{2039}").contains(&"képviselő".to_owned()));
        assert!(recode("\u{df}ll").contains(&"áll".to_owned()));
    }

    #[test]
    fn rep_enc_008_clean_text_is_not_recoded() {
        for word in ["előtt", "ház", "árvíztűrő", "Ångström"] {
            assert!(
                !recode(word).contains(&word.to_owned()),
                "{word} -> {:?}",
                recode(word)
            );
        }
    }

    #[test]
    fn ws_enc_006_the_windows_1252_table() {
        for (letter, damaged) in [
            ("á", "Ã¡"),
            ("é", "Ã©"),
            ("ó", "Ã³"),
            ("ö", "Ã¶"),
            ("ő", "Å‘"),
            ("ú", "Ãº"),
            ("ü", "Ã¼"),
            ("ű", "Å±"),
        ] {
            assert_eq!(mojibake(letter, Page::Windows1252), damaged, "{letter}");
        }
    }

    #[test]
    fn ws_enc_006_the_mac_roman_table() {
        assert_eq!(mojibake("Kérem", Page::MacRoman), "K√©rem");
        assert_eq!(mojibake("ő", Page::MacRoman), "≈ë");
    }

    #[test]
    fn ws_enc_006_damage_round_trips_through_every_page() {
        for page in Page::ALL {
            for word in ["előtt", "hűvös", "árvíztűrő", "működés"] {
                let damaged = mojibake(word, page);
                assert_ne!(damaged, word);
                assert!(
                    demojibake(&damaged).contains(&word.to_owned()),
                    "{word} {page:?} -> {damaged} gave {:?}",
                    demojibake(&damaged)
                );
            }
        }
    }

    #[test]
    fn ws_enc_006_every_hungarian_letter_is_a_lead_and_a_continuation() {
        for page in super::Page::ALL {
            for letter in "áéíóöőúüűÁÉÍÓÖŐÚÜŰ".chars() {
                let damaged = mojibake(&letter.to_string(), page);
                let mut cs = damaged.chars();
                let (lead, trail) = (cs.next().unwrap(), cs.next().unwrap());
                assert!(super::is_lead(lead), "{letter} {page:?} {damaged}");
                assert!(super::is_continuation(trail), "{letter} {page:?} {damaged}");
            }
        }
    }

    #[test]
    fn ws_enc_006_undamaged_text_is_left_alone() {
        for word in ["előtt", "ház", "Ångström", "Åland"] {
            assert!(demojibake(word).is_empty(), "{word}");
        }
    }
}
