//! Vowel harmony as a computed property, never stored (`docs/design/01-core-model.md`).

use crate::letter::{Letter, vowel_char};

/// The harmony class of a Hungarian word form, which determines which suffix allomorph attaches.
///
/// Rules: PH-HARM-001, PH-HARM-002, PH-HARM-003, PH-HARM-004, PH-HARM-005.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Harmony {
    /// Back harmony: suffixes take `a`, `á`, `o`, `ó`, `u`, `ú`.
    Back,
    /// Front unrounded harmony: suffixes take `e`, `é`.
    Front,
    /// Front rounded harmony: suffixes take `ö`, `ő`, `ü`, `ű` where `PH-HARM-005` licenses
    /// rounding.
    FrontRounded,
}

/// Whether `letter` is a neutral vowel (`i`, `í`, `é`), excluded from the backness decision.
const fn is_neutral(letter: Letter) -> bool {
    matches!(letter, Letter::I | Letter::IAcute | Letter::EAcute)
}

/// Whether `letter` is a front rounded vowel (`ö`, `ő`, `ü`, `ű`).
const fn is_rounded(letter: Letter) -> bool {
    matches!(
        letter,
        Letter::OUmlaut | Letter::ODblAcute | Letter::UUmlaut | Letter::UDblAcute
    )
}

/// Whether `letter` is a back vowel (`a`, `á`, `o`, `ó`, `u`, `ú`).
const fn is_back(letter: Letter) -> bool {
    matches!(
        letter,
        Letter::A | Letter::AAcute | Letter::O | Letter::OAcute | Letter::U | Letter::UAcute
    )
}

/// The harmony class of `word`.
///
/// Backness follows the last non-neutral vowel; `i`, `í`, `é` are excluded from that decision, and
/// a word with only neutral vowels is `Front` (`PH-HARM-001`, `PH-HARM-003`, `PH-HARM-004`).
///
/// Rounding follows the *last* vowel of the word, not the last non-neutral one (`PH-HARM-005`):
/// `döntés` ends in `é`, so it is `Front`, not `FrontRounded`, even though its only other vowel is
/// `ö`; likewise `bővítés`. Deriving rounding from the last non-neutral vowel instead produces the
/// wrong `*döntéshöz` / `*bővítésök` instead of `döntéshez` / `bővítések`.
///
/// Rules: PH-HARM-001, PH-HARM-003, PH-HARM-004, PH-HARM-005.
#[must_use]
pub fn harmony_of(word: &str) -> Harmony {
    // Reads vowels off the characters, not segmented letters: called on every suffix attach.
    let mut last = None;
    let mut last_non_neutral = None;
    let mut trailing_neutral = 0usize;
    for (at, c) in word.char_indices().rev() {
        let Some(letter) = vowel_char(c) else {
            continue;
        };
        last = last.or(Some(letter));
        if !is_neutral(letter) {
            // The rightmost non-neutral vowel decides backness, and the rightmost vowel of all is
            // already in hand, so nothing to the left can change either answer.
            last_non_neutral = Some(letter);
            break;
        }
        // PH-HARM-019: the `í` of `-ít` never changes harmony, so it is not a neutral syllable
        // that could tip the count: `bizonyíték` is `bizonyít` plus `-ék`, one neutral, back.
        if !(c == 'í' && word[at + c.len_utf8()..].starts_with('t')) {
            trailing_neutral += 1;
        }
    }

    // PH-HARM-012: one neutral vowel after a back one leaves the stem back (`papírban`), two
    // turn it front by default (`analízisben`, `Assisiben`, `Újvidéken`); the lexicon marks the
    // stems that go the other way.
    if last_non_neutral.is_some_and(is_back) && trailing_neutral < 2 {
        return Harmony::Back;
    }

    match last {
        Some(letter) if is_rounded(letter) => Harmony::FrontRounded,
        _ => Harmony::Front,
    }
}

/// Picks the back or front suffix allomorph for `h` (`PH-HARM-002`).
///
/// `FrontRounded` picks `front`: archiphonemes with only two surface forms (`Á`, `Ú`, and lowered
/// `A`) do not distinguish rounding.
///
/// Rules: PH-HARM-002.
#[must_use]
pub fn pick2<'a>(h: Harmony, back: &'a str, front: &'a str) -> &'a str {
    match h {
        Harmony::Back => back,
        Harmony::Front | Harmony::FrontRounded => front,
    }
}

/// Picks the back, front or front-rounded suffix allomorph for `h` (`PH-HARM-002`,
/// `PH-HARM-004`).
///
/// Rules: PH-HARM-002, PH-HARM-004.
#[must_use]
pub fn pick3<'a>(h: Harmony, back: &'a str, front: &'a str, front_rounded: &'a str) -> &'a str {
    match h {
        Harmony::Back => back,
        Harmony::Front => front,
        Harmony::FrontRounded => front_rounded,
    }
}

#[cfg(test)]
mod tests {
    use super::{Harmony, harmony_of, pick2, pick3};

    #[test]
    fn ph_harm_003_back_words() {
        for w in [
            "ablak", "bíró", "glükóz", "csinos", "betyár", "béka", "amőba",
        ] {
            assert_eq!(harmony_of(w), Harmony::Back, "{w}");
        }
    }

    #[test]
    fn ph_harm_004_front_rounded_words() {
        // PH-HARM-004 examples whose actual last vowel is itself rounded, so PH-HARM-005 also
        // gives FrontRounded (see the next test for the cases where it does not).
        for w in ["üst", "szemölcs", "sofőr", "kosztüm"] {
            assert_eq!(harmony_of(w), Harmony::FrontRounded, "{w}");
        }
    }

    #[test]
    fn ph_harm_005_last_vowel_not_last_non_neutral_decides_rounding() {
        // The trap cited in docs/design/01-core-model.md: harmony_of("döntés") must not be
        // FrontRounded even though its non-final vowel is ö.
        assert_eq!(harmony_of("döntés"), Harmony::Front);
        assert_eq!(harmony_of("bővítés"), Harmony::Front);
        // Same trap, PH-HARM-004's own examples: "fűszer" and "römi" are in the front domain
        // (rightmost non-neutral vowel is rounded), but their actual last vowel (e, i) is not, so
        // the O archiphoneme resolves to plain e, not ö (PH-HARM-005).
        assert_eq!(harmony_of("fűszer"), Harmony::Front);
        assert_eq!(harmony_of("römi"), Harmony::Front);
    }

    #[test]
    fn ph_harm_012_two_trailing_neutral_vowels_turn_a_back_stem_front() {
        assert_eq!(harmony_of("papír"), Harmony::Back);
        assert_eq!(harmony_of("kávé"), Harmony::Back);
        assert_eq!(harmony_of("analízis"), Harmony::Front);
        assert_eq!(harmony_of("bizonyíték"), Harmony::Back);
        assert_eq!(harmony_of("boríték"), Harmony::Back);
        assert_eq!(harmony_of("Assisi"), Harmony::Front);
        assert_eq!(harmony_of("Újvidék"), Harmony::Front);
    }

    #[test]
    fn all_neutral_words_are_front() {
        assert_eq!(harmony_of("kicsi"), Harmony::Front);
        assert_eq!(harmony_of("éjjeli"), Harmony::Front);
    }

    #[test]
    fn a_fully_rounded_stem_stays_front_rounded() {
        assert_eq!(harmony_of("gyűrű"), Harmony::FrontRounded);
        assert_eq!(harmony_of("tök"), Harmony::FrontRounded);
    }

    #[test]
    fn no_vowels_defaults_to_front() {
        assert_eq!(harmony_of(""), Harmony::Front);
        assert_eq!(harmony_of("brr"), Harmony::Front);
    }

    #[test]
    fn pick2_ignores_rounding() {
        assert_eq!(pick2(Harmony::Back, "nak", "nek"), "nak");
        assert_eq!(pick2(Harmony::Front, "nak", "nek"), "nek");
        assert_eq!(pick2(Harmony::FrontRounded, "nak", "nek"), "nek");
    }

    #[test]
    fn pick3_distinguishes_rounding() {
        assert_eq!(pick3(Harmony::Back, "hoz", "hez", "höz"), "hoz");
        assert_eq!(pick3(Harmony::Front, "hoz", "hez", "höz"), "hez");
        assert_eq!(pick3(Harmony::FrontRounded, "hoz", "hez", "höz"), "höz");
    }
}
