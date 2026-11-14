//! Hungarian letters: the spelling unit of `WS-LET-001`.
//!
//! A letter is one, two or three characters (`WS-ALPH-004`). [`letters`] segments a string into
//! letter tokens, resolving the `dzs` trigraph, the eight two-character letters and simplified
//! doubled digraphs (`WS-DIG-001`, `WS-DIG-002`).

use smallvec::SmallVec;

/// One of the 44 letters of the extended Hungarian alphabet, in alphabetical order.
///
/// Case is not part of a letter's identity; [`letters`] records the original characters, with
/// their original case, in [`Span::text`].
///
/// Rules: WS-ALPH-001, WS-ALPH-002, WS-ALPH-003, WS-ALPH-004.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Letter {
    /// `a`.
    A,
    /// `á`.
    AAcute,
    /// `b`.
    B,
    /// `c`.
    C,
    /// `cs`.
    Cs,
    /// `d`.
    D,
    /// `dz`.
    Dz,
    /// `dzs`.
    Dzs,
    /// `e`.
    E,
    /// `é`.
    EAcute,
    /// `f`.
    F,
    /// `g`.
    G,
    /// `gy`.
    Gy,
    /// `h`.
    H,
    /// `i`.
    I,
    /// `í`.
    IAcute,
    /// `j`.
    J,
    /// `k`.
    K,
    /// `l`.
    L,
    /// `ly`.
    Ly,
    /// `m`.
    M,
    /// `n`.
    N,
    /// `ny`.
    Ny,
    /// `o`.
    O,
    /// `ó`.
    OAcute,
    /// `ö`.
    OUmlaut,
    /// `ő`.
    ODblAcute,
    /// `p`.
    P,
    /// `q`.
    Q,
    /// `r`.
    R,
    /// `s`.
    S,
    /// `sz`.
    Sz,
    /// `t`.
    T,
    /// `ty`.
    Ty,
    /// `u`.
    U,
    /// `ú`.
    UAcute,
    /// `ü`.
    UUmlaut,
    /// `ű`.
    UDblAcute,
    /// `v`.
    V,
    /// `w`.
    W,
    /// `x`.
    X,
    /// `y`.
    Y,
    /// `z`.
    Z,
    /// `zs`.
    Zs,
}

impl Letter {
    /// Every letter, in alphabetical order (`WS-ALPH-002`).
    pub const ALL: [Self; 44] = [
        Self::A,
        Self::AAcute,
        Self::B,
        Self::C,
        Self::Cs,
        Self::D,
        Self::Dz,
        Self::Dzs,
        Self::E,
        Self::EAcute,
        Self::F,
        Self::G,
        Self::Gy,
        Self::H,
        Self::I,
        Self::IAcute,
        Self::J,
        Self::K,
        Self::L,
        Self::Ly,
        Self::M,
        Self::N,
        Self::Ny,
        Self::O,
        Self::OAcute,
        Self::OUmlaut,
        Self::ODblAcute,
        Self::P,
        Self::Q,
        Self::R,
        Self::S,
        Self::Sz,
        Self::T,
        Self::Ty,
        Self::U,
        Self::UAcute,
        Self::UUmlaut,
        Self::UDblAcute,
        Self::V,
        Self::W,
        Self::X,
        Self::Y,
        Self::Z,
        Self::Zs,
    ];

    /// The canonical lowercase spelling of the letter.
    ///
    /// Rules: WS-ALPH-001, WS-ALPH-002.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A => "a",
            Self::AAcute => "á",
            Self::B => "b",
            Self::C => "c",
            Self::Cs => "cs",
            Self::D => "d",
            Self::Dz => "dz",
            Self::Dzs => "dzs",
            Self::E => "e",
            Self::EAcute => "é",
            Self::F => "f",
            Self::G => "g",
            Self::Gy => "gy",
            Self::H => "h",
            Self::I => "i",
            Self::IAcute => "í",
            Self::J => "j",
            Self::K => "k",
            Self::L => "l",
            Self::Ly => "ly",
            Self::M => "m",
            Self::N => "n",
            Self::Ny => "ny",
            Self::O => "o",
            Self::OAcute => "ó",
            Self::OUmlaut => "ö",
            Self::ODblAcute => "ő",
            Self::P => "p",
            Self::Q => "q",
            Self::R => "r",
            Self::S => "s",
            Self::Sz => "sz",
            Self::T => "t",
            Self::Ty => "ty",
            Self::U => "u",
            Self::UAcute => "ú",
            Self::UUmlaut => "ü",
            Self::UDblAcute => "ű",
            Self::V => "v",
            Self::W => "w",
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
            Self::Zs => "zs",
        }
    }

    /// Whether the letter is one of the 14 vowels (`WS-ALPH-003`).
    #[must_use]
    pub const fn is_vowel(self) -> bool {
        matches!(
            self,
            Self::A
                | Self::AAcute
                | Self::E
                | Self::EAcute
                | Self::I
                | Self::IAcute
                | Self::O
                | Self::OAcute
                | Self::OUmlaut
                | Self::ODblAcute
                | Self::U
                | Self::UAcute
                | Self::UUmlaut
                | Self::UDblAcute
        )
    }

    /// Whether the letter is a consonant (`WS-ALPH-004`).
    #[must_use]
    pub const fn is_consonant(self) -> bool {
        !self.is_vowel()
    }

    /// Whether the vowel is the long member of its short-long pair (`WS-ALPH-003`).
    ///
    /// `false` for consonants.
    #[must_use]
    pub const fn is_long_vowel(self) -> bool {
        matches!(
            self,
            Self::AAcute
                | Self::EAcute
                | Self::IAcute
                | Self::OAcute
                | Self::ODblAcute
                | Self::UAcute
                | Self::UDblAcute
        )
    }

    /// The short member of this vowel's short-long pair; `None` for consonants.
    ///
    /// Rules: WS-ALPH-003.
    #[must_use]
    pub const fn short(self) -> Option<Self> {
        match self {
            Self::A | Self::AAcute => Some(Self::A),
            Self::E | Self::EAcute => Some(Self::E),
            Self::I | Self::IAcute => Some(Self::I),
            Self::O | Self::OAcute => Some(Self::O),
            Self::OUmlaut | Self::ODblAcute => Some(Self::OUmlaut),
            Self::U | Self::UAcute => Some(Self::U),
            Self::UUmlaut | Self::UDblAcute => Some(Self::UUmlaut),
            _ => None,
        }
    }

    /// The long member of this vowel's short-long pair; `None` for consonants.
    ///
    /// Rules: WS-ALPH-003.
    #[must_use]
    pub const fn long(self) -> Option<Self> {
        match self {
            Self::A | Self::AAcute => Some(Self::AAcute),
            Self::E | Self::EAcute => Some(Self::EAcute),
            Self::I | Self::IAcute => Some(Self::IAcute),
            Self::O | Self::OAcute => Some(Self::OAcute),
            Self::OUmlaut | Self::ODblAcute => Some(Self::ODblAcute),
            Self::U | Self::UAcute => Some(Self::UAcute),
            Self::UUmlaut | Self::UDblAcute => Some(Self::UDblAcute),
            _ => None,
        }
    }
}

/// One letter located in source text (`WS-LET-001`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span<'a> {
    /// The letter this span spells.
    pub letter: Letter,
    /// The original characters, exactly as written (1 to 3 characters, any case).
    pub text: &'a str,
    /// Byte offset of `text` within the string passed to [`letters`].
    ///
    /// Present because a caller cannot reconstruct it by summing `text.len()`: characters outside
    /// the alphabet are skipped, so the sum drifts from the real position as soon as a word
    /// contains a hyphen, an apostrophe or a digit.
    pub at: usize,
}

/// Segments `s` into Hungarian letters, longest match first.
///
/// Resolves the `dzs` trigraph, the eight two-character letters, and simplified doubled digraphs:
/// `ssz` -> two `Sz` spans (`"s"`, `"sz"`); `ccs` -> two `Cs`; `ggy` -> two `Gy`; `lly` -> two `Ly`;
/// `nny` -> two `Ny`; `tty` -> two `Ty`; `zzs` -> two `Zs`; `ddz` -> two `Dz`; `ddzs` -> two `Dzs`.
/// Matching is case-insensitive; each span keeps the original characters and case.
///
/// Characters outside the alphabet (digits, punctuation, whitespace) are skipped and produce no
/// span.
///
/// This is stem-level segmentation only: it has no morphological information, so it cannot tell a
/// genuine long `sz` inside a stem from a compound seam that merely looks like one
/// (`hossz-szelvény`, `WS-DIG-010`). Resolving that ambiguity is the caller's job
/// (`docs/design/01-core-model.md`).
///
/// Rules: WS-LET-001, WS-LET-002, WS-DIG-001, WS-DIG-002.
pub fn letters(s: &str) -> impl Iterator<Item = Span<'_>> {
    raw_spans(s).into_iter().map(move |(letter, start, end)| {
        let (start, end) = (start as usize, end as usize);
        Span {
            letter,
            text: &s[start..end],
            at: start,
        }
    })
}

/// Letter tokens as `(letter, start_byte, end_byte)` into the source string.
///
/// Shared with `syllable.rs`, which needs byte offsets to slice and to detect where a
/// simplified doubled digraph was split (`ORT-SPLIT-007`).
pub(crate) fn raw_spans(s: &str) -> Spans {
    MEMO.with_borrow_mut(|memo| {
        let slot = memo_slot(s);
        if let Some((held, spans)) = &memo[slot]
            && held.as_ref() == s
        {
            return spans.clone();
        }
        let spans = raw_spans_seamed(s, &[]);
        memo[slot] = Some((s.into(), spans.clone()));
        spans
    })
}

/// Slots in the segmentation memo.
///
/// Analysing one word segments its candidate stems once per stack it tries, thousands of times
/// for a few dozen distinct strings, so a small table answers nearly every call.
const MEMO_SLOTS: usize = 1 << 10;

/// One memo slot: the string it holds and its segmentation.
type MemoSlot = Option<(Box<str>, Spans)>;

thread_local! {
    /// Segmentations by string, direct-mapped: a slot holds whichever string last hashed to it.
    static MEMO: std::cell::RefCell<Vec<MemoSlot>> =
        std::cell::RefCell::new(vec![None; MEMO_SLOTS]);
}

/// The memo slot of `s`, by FNV-1a over its bytes.
fn memo_slot(s: &str) -> usize {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    #[expect(
        clippy::cast_possible_truncation,
        reason = "an index into a fixed table"
    )]
    let slot = (h as usize) % MEMO_SLOTS;
    slot
}

/// Letter tokens of one word, kept on the stack.
///
/// Segmentation is the innermost operation of the checker: analysing one word re-segments short
/// strings roughly twelve thousand times, so heap traffic here dominates the whole cost. The
/// inline capacity covers a Hungarian word comfortably; longer compounds spill to the heap.
pub(crate) type Spans = SmallVec<[(Letter, Off, Off); INLINE]>;

/// A byte offset within one word. Narrower than `usize` to keep [`Spans`] cheap to return.
pub(crate) type Off = u32;

/// Letters held without allocating. Above the length of nearly every word, below a large frame.
const INLINE: usize = 16;

/// Letter tokens, with a morpheme seam forbidden from falling inside a multi-character letter.
///
/// `ORT-SPLIT-020`: a sequence that looks like a digraph is two letters when a boundary runs
/// through it, so `köz` plus `ség` has a `z` and an `s` rather than a `zs`, and `ház` plus `sor`
/// likewise. `WS-DIG-006` is the same fact seen from the compounding side, where it is what keeps
/// `vízsugár` from splitting as `ví|zsugár`.
///
/// `seams` are byte offsets into `s`; an empty slice is the ordinary reading.
pub(crate) fn raw_spans_seamed(s: &str, seams: &[usize]) -> Spans {
    let mut offsets: SmallVec<[Off; INLINE]> = s.char_indices().map(|(b, _)| off(b)).collect();
    offsets.push(off(s.len()));
    let lower: SmallVec<[char; INLINE]> = s.chars().map(lower_char).collect();

    let intact = |start: Off, end: Off| !seams.iter().any(|&b| off(b) > start && off(b) < end);

    let mut spans = Spans::new();
    let mut i = 0;
    while i < lower.len() {
        // WS-DIG-001 step 1: c0 == c1 and a multi-character letter starts at c1 -> emit its
        // first half here, then let the next iteration emit the full letter.
        if i + 1 < lower.len()
            && lower[i + 1] == lower[i]
            && let Some((letter, len)) = multichar_at(&lower, i + 1)
            && intact(offsets[i + 1], offsets[i + 1 + len])
        {
            spans.push((letter, offsets[i], offsets[i + 1]));
            i += 1;
            continue;
        }
        // WS-DIG-001 steps 2-3: longest multi-character letter.
        if let Some((letter, len)) = multichar_at(&lower, i)
            && intact(offsets[i], offsets[i + len])
        {
            spans.push((letter, offsets[i], offsets[i + len]));
            i += len;
            continue;
        }
        // WS-DIG-001 step 4: single character, if it is a letter.
        if let Some(letter) = single_letter(lower[i]) {
            spans.push((letter, offsets[i], offsets[i + 1]));
        }
        i += 1;
    }
    spans
}

/// The vowel `c` writes, if it writes one.
///
/// Every Hungarian vowel is written with a single character: all nine multi-character letters
/// (`cs dz dzs gy ly ny sz ty zs`) are consonants. So a vowel can be read straight off a
/// character, and code that only wants the vowels need not segment the word at all.
pub(crate) fn vowel_char(c: char) -> Option<Letter> {
    Some(match lower_char(c) {
        'a' => Letter::A,
        'á' => Letter::AAcute,
        'e' => Letter::E,
        'é' => Letter::EAcute,
        'i' => Letter::I,
        'í' => Letter::IAcute,
        'o' => Letter::O,
        'ó' => Letter::OAcute,
        'ö' => Letter::OUmlaut,
        'ő' => Letter::ODblAcute,
        'u' => Letter::U,
        'ú' => Letter::UAcute,
        'ü' => Letter::UUmlaut,
        'ű' => Letter::UDblAcute,
        _ => return None,
    })
}

/// A byte position as an [`Off`].
///
/// Saturates rather than wrapping, so a string beyond the range degrades to a missed letter
/// boundary instead of a wrong one. No Hungarian word comes near it.
fn off(b: usize) -> Off {
    Off::try_from(b).unwrap_or(Off::MAX)
}

/// The Unicode lowercase of `c`, or `c` itself if lowercasing is not 1:1.
///
/// The alphabet is answered without touching the Unicode conversion tables, whose lookup is
/// otherwise the single most expensive step of segmentation. Anything else still goes through
/// `to_lowercase`, so a letter outside Hungarian is cased exactly as before.
fn lower_char(c: char) -> char {
    match c {
        'a'..='z' | '\u{E0}'..='\u{FF}' => c,
        'A'..='Z' => c.to_ascii_lowercase(),
        'Á' => 'á',
        'É' => 'é',
        'Í' => 'í',
        'Ó' => 'ó',
        'Ö' => 'ö',
        'Ő' => 'ő',
        'Ú' => 'ú',
        'Ü' => 'ü',
        'Ű' => 'ű',
        _ => c.to_lowercase().next().unwrap_or(c),
    }
}

/// The two- or three-character letter starting at `lower[i]`, longest match first.
fn multichar_at(lower: &[char], i: usize) -> Option<(Letter, usize)> {
    if lower.get(i..i + 3) == Some(&['d', 'z', 's']) {
        return Some((Letter::Dzs, 3));
    }
    match (lower.get(i).copied(), lower.get(i + 1).copied()) {
        (Some('c'), Some('s')) => Some((Letter::Cs, 2)),
        (Some('d'), Some('z')) => Some((Letter::Dz, 2)),
        (Some('g'), Some('y')) => Some((Letter::Gy, 2)),
        (Some('l'), Some('y')) => Some((Letter::Ly, 2)),
        (Some('n'), Some('y')) => Some((Letter::Ny, 2)),
        (Some('s'), Some('z')) => Some((Letter::Sz, 2)),
        (Some('t'), Some('y')) => Some((Letter::Ty, 2)),
        (Some('z'), Some('s')) => Some((Letter::Zs, 2)),
        _ => None,
    }
}

/// The single-character letter `c`, if `c` is in the alphabet.
fn single_letter(c: char) -> Option<Letter> {
    match c {
        'a' => Some(Letter::A),
        'á' => Some(Letter::AAcute),
        'b' => Some(Letter::B),
        'c' => Some(Letter::C),
        'd' => Some(Letter::D),
        'e' => Some(Letter::E),
        'é' => Some(Letter::EAcute),
        'f' => Some(Letter::F),
        'g' => Some(Letter::G),
        'h' => Some(Letter::H),
        'i' => Some(Letter::I),
        'í' => Some(Letter::IAcute),
        'j' => Some(Letter::J),
        'k' => Some(Letter::K),
        'l' => Some(Letter::L),
        'm' => Some(Letter::M),
        'n' => Some(Letter::N),
        'o' => Some(Letter::O),
        'ó' => Some(Letter::OAcute),
        'ö' => Some(Letter::OUmlaut),
        'ő' => Some(Letter::ODblAcute),
        'p' => Some(Letter::P),
        'q' => Some(Letter::Q),
        'r' => Some(Letter::R),
        's' => Some(Letter::S),
        't' => Some(Letter::T),
        'u' => Some(Letter::U),
        'ú' => Some(Letter::UAcute),
        'ü' => Some(Letter::UUmlaut),
        'ű' => Some(Letter::UDblAcute),
        'v' => Some(Letter::V),
        'w' => Some(Letter::W),
        'x' => Some(Letter::X),
        'y' => Some(Letter::Y),
        'z' => Some(Letter::Z),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{Letter, Span, letters};

    fn spans(s: &str) -> Vec<(Letter, &str)> {
        letters(s).map(|sp| (sp.letter, sp.text)).collect()
    }

    #[test]
    fn all_variants_are_in_declaration_order() {
        // WS-ALPH-001, WS-ALPH-002: ALL must match the extended alphabet order 1:1, or the
        // derived Ord (used for collation elsewhere) drifts from the norm.
        assert_eq!(Letter::ALL.len(), 44);
        for (i, l) in Letter::ALL.into_iter().enumerate() {
            assert_eq!(l as usize, i, "Letter::ALL out of order at {i}");
        }
    }

    #[test]
    fn as_str_round_trips_through_single_letter() {
        for l in Letter::ALL {
            assert_eq!(spans(l.as_str()), vec![(l, l.as_str())]);
        }
    }

    #[test]
    fn ws_let_001_hosszu_is_five_letters_six_characters() {
        let got = spans("hosszú");
        assert_eq!(
            got,
            vec![
                (Letter::H, "h"),
                (Letter::O, "o"),
                (Letter::Sz, "s"),
                (Letter::Sz, "sz"),
                (Letter::UAcute, "ú"),
            ]
        );
        assert_eq!(got.iter().map(|(_, t)| t.len()).sum::<usize>(), 7);
        assert_eq!(got.len(), 5);
    }

    #[test]
    fn ws_let_001_dzsungel_is_six_letters_eight_characters() {
        let got = spans("dzsungel");
        assert_eq!(
            got,
            vec![
                (Letter::Dzs, "dzs"),
                (Letter::U, "u"),
                (Letter::N, "n"),
                (Letter::G, "g"),
                (Letter::E, "e"),
                (Letter::L, "l"),
            ]
        );
        assert_eq!(got.len(), 6);
    }

    #[test]
    fn ws_let_001_jegygyuru_is_seven_letters_via_two_plain_gy() {
        // "jegygyűrű" = j e gy gy ű r ű: this is the WS-DIG-004 full-doubling shape written at a
        // compound seam, but letters() has no morphology, so it just parses two ordinary `gy`
        // digraphs -- which happens to be the documented correct letter count.
        assert_eq!(
            spans("jegygyűrű"),
            vec![
                (Letter::J, "j"),
                (Letter::E, "e"),
                (Letter::Gy, "gy"),
                (Letter::Gy, "gy"),
                (Letter::UDblAcute, "ű"),
                (Letter::R, "r"),
                (Letter::UDblAcute, "ű"),
            ]
        );
    }

    #[test]
    fn ws_dig_001_kedd_does_not_merge_a_plain_single_letter_geminate() {
        assert_eq!(
            spans("kedd"),
            vec![
                (Letter::K, "k"),
                (Letter::E, "e"),
                (Letter::D, "d"),
                (Letter::D, "d"),
            ]
        );
    }

    #[test]
    fn ws_dig_002_meggy_is_e_g_gy_gy() {
        assert_eq!(
            spans("meggy"),
            vec![
                (Letter::M, "m"),
                (Letter::E, "e"),
                (Letter::Gy, "g"),
                (Letter::Gy, "gy"),
            ]
        );
    }

    #[test]
    fn ws_dig_002_asszony_is_a_sz_sz_o_ny() {
        assert_eq!(
            spans("asszony"),
            vec![
                (Letter::A, "a"),
                (Letter::Sz, "s"),
                (Letter::Sz, "sz"),
                (Letter::O, "o"),
                (Letter::Ny, "ny"),
            ]
        );
    }

    #[test]
    fn simplified_doubling_of_every_multichar_letter() {
        assert_eq!(spans("ssz"), vec![(Letter::Sz, "s"), (Letter::Sz, "sz")]);
        assert_eq!(spans("ccs"), vec![(Letter::Cs, "c"), (Letter::Cs, "cs")]);
        assert_eq!(spans("ggy"), vec![(Letter::Gy, "g"), (Letter::Gy, "gy")]);
        assert_eq!(spans("lly"), vec![(Letter::Ly, "l"), (Letter::Ly, "ly")]);
        assert_eq!(spans("nny"), vec![(Letter::Ny, "n"), (Letter::Ny, "ny")]);
        assert_eq!(spans("tty"), vec![(Letter::Ty, "t"), (Letter::Ty, "ty")]);
        assert_eq!(spans("zzs"), vec![(Letter::Zs, "z"), (Letter::Zs, "zs")]);
        assert_eq!(spans("ddz"), vec![(Letter::Dz, "d"), (Letter::Dz, "dz")]);
        assert_eq!(
            spans("ddzs"),
            vec![(Letter::Dzs, "d"), (Letter::Dzs, "dzs")]
        );
    }

    #[test]
    fn ws_dig_005_dz_is_single_length_in_stems() {
        // bodza: stem-internal dz is not doubled (WS-DIG-005).
        assert_eq!(
            spans("bodza"),
            vec![
                (Letter::B, "b"),
                (Letter::O, "o"),
                (Letter::Dz, "dz"),
                (Letter::A, "a"),
            ]
        );
    }

    #[test]
    fn ws_dig_001_case_insensitive_segmentation() {
        assert_eq!(
            spans("SZÉP"),
            vec![(Letter::Sz, "SZ"), (Letter::EAcute, "É"), (Letter::P, "P")]
        );
        assert_eq!(
            spans("Gyöngy"),
            vec![
                (Letter::Gy, "Gy"),
                (Letter::OUmlaut, "ö"),
                (Letter::N, "n"),
                (Letter::Gy, "gy"),
            ]
        );
    }

    #[test]
    fn ws_let_001_non_alphabet_characters_are_skipped() {
        assert_eq!(spans("a-b 1"), vec![(Letter::A, "a"), (Letter::B, "b")]);
    }

    #[test]
    fn ws_dig_001_known_morpheme_boundary_ambiguity_is_not_resolved() {
        // Documented limitation (WS-DIG-001 Exceptions): without morphology, "kilencszer"
        // (kilenc + szer) parses with a spurious cs, exactly as the rule predicts.
        assert_eq!(
            spans("kilencszer"),
            vec![
                (Letter::K, "k"),
                (Letter::I, "i"),
                (Letter::L, "l"),
                (Letter::E, "e"),
                (Letter::N, "n"),
                (Letter::Cs, "cs"),
                (Letter::Z, "z"),
                (Letter::E, "e"),
                (Letter::R, "r"),
            ]
        );
    }

    #[test]
    fn vowel_pairing_short_and_long() {
        assert_eq!(Letter::A.long(), Some(Letter::AAcute));
        assert_eq!(Letter::AAcute.short(), Some(Letter::A));
        assert_eq!(Letter::OUmlaut.long(), Some(Letter::ODblAcute));
        assert_eq!(Letter::ODblAcute.short(), Some(Letter::OUmlaut));
        assert_eq!(Letter::UUmlaut.long(), Some(Letter::UDblAcute));
        assert_eq!(Letter::B.short(), None);
        assert_eq!(Letter::B.long(), None);
    }

    #[test]
    fn vowel_and_consonant_classification() {
        for l in Letter::ALL {
            assert_eq!(l.is_vowel(), !l.is_consonant());
        }
        assert!(Letter::AAcute.is_vowel());
        assert!(Letter::AAcute.is_long_vowel());
        assert!(!Letter::A.is_long_vowel());
        assert!(Letter::Sz.is_consonant());
        assert!(!Letter::Sz.is_long_vowel());
    }

    #[test]
    fn span_debug_and_eq_are_derived() {
        let a = Span {
            letter: Letter::A,
            text: "a",
            at: 0,
        };
        let b = Span {
            letter: Letter::A,
            text: "a",
            at: 0,
        };
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "Span { letter: A, text: \"a\", at: 0 }");
    }
}
