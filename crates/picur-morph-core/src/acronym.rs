//! Suffixes on acronyms and abbreviations, written after a hyphen.
//!
//! `ORT-ABBR-014` attaches the suffix in the form the acronym's **pronunciation** requires, and
//! `ORT-ABBR-018` says plainly that the pronunciation is lexical: an acronym may be read out by
//! letter names or as a word, and where the spelled-out form mixes back and neutral vowels the
//! harmony cannot be derived at all. The rule's own measurement makes the point: `BKV-val` beats
//! `BKV-vel` 500 to 17 although the last letter name `vé` is front.
//!
//! So this module does not decide the reading. It reports which written suffixes some licensed
//! reading would produce, which is what a checker needs: enough to accept `MÁV-val`, `BKV-val` and
//! `ENSZ-szel` without inventing a pronunciation we do not have, and still enough to reject a tail
//! no reading could give.
//!
//! See `docs/grammar/07-orthography.md`.

use picur_morph_lexicon::{Entry, Pos, Role, Trait, TraitSet, lists, pairs};
use picur_morph_ortho::{Harmony, Letter, letters};

use crate::features::{Case, Morph};
use crate::generate::attach_parts;
use crate::suffix::case_suffix;

/// A stem that carries a harmony class and the given final letter, for realising a suffix against.
pub(crate) fn probe(h: Harmony, final_letter: Option<Letter>) -> Entry {
    let mut lemma = String::from("a");
    if let Some(l) = final_letter {
        lemma.push_str(l.as_str());
    }
    let mut e = Entry::new(lemma, Pos::Noun);
    e.traits = TraitSet::from_iter_of([match h {
        Harmony::Back => Trait::Back,
        Harmony::Front => Trait::Front,
        Harmony::FrontRounded => Trait::FrontRounded,
    }]);
    e
}

/// Every written suffix some licensed reading of an acronym ending in `final_letter` could take for
/// this case.
///
/// Two axes, both genuinely open: the harmony class, because the reading is lexical, and whether
/// the seam assimilates, because that depends on the last *spoken* sound rather than the last
/// written letter. `ENSZ` read as a word ends in `sz` and gives `ENSZ-szel`; `BKV` read out ends in
/// the vowel of `vé` and gives `BKV-val`.
///
/// Rules: `ORT-ABBR-014`, `ORT-ABBR-018`
#[must_use]
pub fn written_suffixes(final_letter: Option<Letter>, case: Case) -> Vec<String> {
    let Some(suffix) = case_suffix(case) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for h in [Harmony::Back, Harmony::Front, Harmony::FrontRounded] {
        // A vowel-final probe blocks assimilation, a consonant-final one allows it.
        for tail in [None, final_letter] {
            let e = probe(h, tail);
            let (form, at) = attach_parts(&e.lemma, &e, &suffix, false);
            let written = form[at..].to_owned();
            if !written.is_empty() && !out.contains(&written) {
                out.push(written);
            }
        }
    }
    out
}

/// Whether `tail` is a suffix a reading of the acronym `base` licenses.
///
/// An acronym read as a word has one spoken form and one set of suffixes (`ENSZ-szel`, not
/// `ENSZ-vel`). One read out by letter names mixes back and neutral vowels, and `ORT-ABBR-018`
/// leaves its harmony lexical, so both classes stay licensed (`BKV-val` is the usual form and
/// `BKV-vel` is not an error).
///
/// Rules: `ORT-ABBR-014`, `ORT-ABBR-018`, `PH-G2P-011`
#[must_use]
pub fn licenses(base: &str, tail: &str) -> bool {
    if lists(Role::AcronymDerivationalTail, tail) {
        return true;
    }
    if reading(base) == Some(Reading::Word) {
        return ALL_CASES
            .iter()
            .any(|&c| written_suffix(base, c).as_deref() == Some(tail));
    }
    let final_letter = letters(base).last().map(|s| s.letter);
    ALL_CASES
        .iter()
        .any(|&c| written_suffixes(final_letter, c).iter().any(|s| s == tail))
}

/// How an acronym is read out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reading {
    /// As a word (`MÁV`, `NATO`, `ENSZ`).
    Word,
    /// Letter by letter (`MTA`, `OTP`, `KSH`).
    Letters,
}

/// The reading an acronym's shape fixes.
///
/// `PH-G2P-011`: an acronym is read as a word when its letters spell one a Hungarian mouth can
/// say, and letter by letter otherwise. The test is phonotactic: a vowel, an onset of at most one
/// consonant or a loan onset (`tr`, `pl`, `st`), and a well-formed final cluster. `MTA` fails on
/// its onset, `OTP` on its coda and `KSH` on its vowel; `MÁV`, `ELTE` and `ENSZ` pass.
///
/// `None` for anything that is not a letter acronym.
///
/// Rules: `PH-G2P-011`, `ORT-ABBR-018`
#[must_use]
pub fn reading(word: &str) -> Option<Reading> {
    if !is_letter_acronym(word) {
        return None;
    }
    let lower = word.to_lowercase();
    let spans: Vec<Letter> = letters(&lower).map(|s| s.letter).collect();
    let Some(first_vowel) = spans.iter().position(|l| l.is_vowel()) else {
        return Some(Reading::Letters);
    };
    let onset = &spans[..first_vowel];
    let onset_ok = match onset {
        [] | [_] => true,
        [a, b] => {
            matches!(b.as_str(), "l" | "r") && !matches!(a.as_str(), "l" | "r" | "n" | "m")
                || matches!(a.as_str(), "s" | "sz") && matches!(b.as_str(), "p" | "t" | "k")
        }
        _ => false,
    };
    if onset_ok && picur_morph_ortho::well_formed_final_cluster(&lower) {
        Some(Reading::Word)
    } else {
        Some(Reading::Letters)
    }
}

/// The name of a letter, as it is said when an acronym is read out (`WS-ALPH-001`).
fn letter_name(letter: Letter) -> &'static str {
    match letter.as_str() {
        "a" | "á" => "á",
        "b" => "bé",
        "c" => "cé",
        "cs" => "csé",
        "d" => "dé",
        "dz" => "dzé",
        "dzs" => "dzsé",
        "e" | "é" => "é",
        "f" => "ef",
        "g" => "gé",
        "gy" => "gyé",
        "h" => "há",
        "i" | "í" => "í",
        "j" => "jé",
        "k" => "ká",
        "l" => "el",
        "ly" => "elipszilon",
        "m" => "em",
        "n" => "en",
        "ny" => "eny",
        "o" | "ó" => "ó",
        "ö" | "ő" => "ő",
        "p" => "pé",
        "q" => "kú",
        "r" => "er",
        "s" => "es",
        "sz" => "esz",
        "t" => "té",
        "ty" => "tyé",
        "u" | "ú" => "ú",
        "ü" | "ű" => "ű",
        "v" => "vé",
        "w" => "duplavé",
        "x" => "iksz",
        "y" => "ipszilon",
        "z" => "zé",
        "zs" => "zsé",
        _ => "",
    }
}

/// The spoken form of an acronym whose reading its shape fixes: the word itself, or its letter
/// names in a row (`MTA` is `emtéá`).
#[must_use]
pub fn spoken(word: &str) -> Option<String> {
    match reading(word)? {
        Reading::Word => Some(word.to_lowercase()),
        Reading::Letters => Some(
            letters(&word.to_lowercase())
                .map(|s| letter_name(s.letter))
                .collect(),
        ),
    }
}

/// The written suffix of `case` after the acronym `word`, by its spoken form.
///
/// The harmony is the spoken form's (`BKV-val`: `békávé` is back), and the seam follows its last
/// sound: a letter name ending in a vowel takes the bare suffix (`MTA-val`), a word-read acronym
/// assimilates like any stem (`ENSZ-szel`).
///
/// Rules: `ORT-ABBR-014`, `PH-G2P-011`
#[must_use]
pub fn written_suffix(word: &str, case: Case) -> Option<String> {
    let spoken = spoken(word)?;
    let suffix = case_suffix(case)?;
    let h = picur_morph_ortho::harmony_of(&spoken);
    let last = letters(&spoken).last().map(|s| s.letter)?;
    let e = probe(h, last.is_consonant().then_some(last));
    let (form, at) = attach_parts(&e.lemma, &e, &suffix, false);
    let written = form[at..].to_owned();
    (!written.is_empty()).then_some(written)
}

/// The acronym `word` in `case`, written with its hyphen (`ORT-ABBR-009`).
///
/// Rules: `ORT-ABBR-009`, `ORT-ABBR-014`
#[must_use]
pub fn inflected(word: &str, case: Case) -> Option<String> {
    Some(format!("{word}-{}", written_suffix(word, case)?))
}

/// Whether `word` is written as a letter acronym: two to six letters, every one a capital.
///
/// `ORT-ABBR-011` writes proper-noun and scientific letter acronyms in all capitals, digraphs
/// included, and which strings are acronyms is its lexical input. The shape alone is therefore
/// what a checker can know, and it is enough to keep `KSH` and `DNS` from being called wrong
/// Hungarian for having no vowel.
#[must_use]
pub fn is_letter_acronym(word: &str) -> bool {
    let n = letters(word).count();
    (2..=6).contains(&n)
        && letters(word).map(|s| s.text.len()).sum::<usize>() == word.len()
        && word.chars().all(char::is_uppercase)
}

/// Every consonant letter, for probing a suffix against a stem whose last sound is unknown.
const CONSONANTS: [&str; 30] = [
    "b", "c", "cs", "d", "dz", "dzs", "f", "g", "gy", "h", "j", "k", "l", "ly", "m", "n", "ny",
    "p", "q", "r", "s", "sz", "t", "ty", "v", "w", "x", "y", "z", "zs",
];

/// Every written nominal suffix a stem of unknown pronunciation could take.
///
/// `ORT-FOR-012` writes the suffix after a hyphen when a foreign name ends in a letter that is
/// not pronounced, and the suffix then follows the *spoken* ending, which no spelling shows:
/// `Chilcote-ot` is back-harmonic and consonant-final in speech. So a hyphenated tail on a name
/// is licensed by any harmony class and any final sound, which is the acronym probe over every
/// consonant and the vowel-final case.
fn foreign_tails() -> &'static std::collections::HashSet<String> {
    static TAILS: std::sync::LazyLock<std::collections::HashSet<String>> =
        std::sync::LazyLock::new(|| {
            let finals = std::iter::once(None).chain(
                CONSONANTS
                    .iter()
                    .map(|c| letters(c).next().map(|s| s.letter)),
            );
            let mut out = std::collections::HashSet::new();
            for final_letter in finals {
                for case in ALL_CASES {
                    out.extend(written_suffixes(final_letter, case));
                }
                out.extend(nominal_tails(final_letter));
            }
            out
        });
    &TAILS
}

/// Whether `tail` is a written suffix some pronunciation of a foreign name licenses.
///
/// Rules: `ORT-FOR-012`
#[must_use]
pub fn licenses_foreign(tail: &str) -> bool {
    foreign_tails().contains(tail) || lists(Role::AcronymDerivationalTail, tail)
}

/// Whether `tail` is a written suffix of the word `base` ends with a symbol for.
///
/// Rules: `PH-G2P-013`, `PND-FRAC-006`
#[must_use]
pub fn licenses_expansion(base: &str, tail: &str) -> bool {
    let Some((_, word)) = pairs(Role::SymbolExpansion)
        .iter()
        .find(|(sym, _)| base.ends_with(sym.as_ref()))
    else {
        return false;
    };
    let Some(entry) = picur_morph_lexicon::lexicon().get(word).first() else {
        return false;
    };
    let poss = Morph::Possessive {
        person: crate::features::Person::Third,
        number: crate::features::Number::Sing,
    };
    let cases = ALL_CASES.iter().flat_map(|&c| {
        [
            vec![Morph::Case(c)],
            vec![poss, Morph::Case(c)],
            vec![Morph::Plural, Morph::Case(c)],
        ]
    });
    let others = [
        vec![Morph::Plural],
        vec![poss],
        vec![Morph::Derive(crate::derivation::DerId::Os)],
        vec![Morph::Derive(crate::derivation::DerId::Nyi)],
        vec![Morph::Derive(crate::derivation::DerId::I)],
    ];
    cases.chain(others).any(|stack| {
        crate::generate::generate_seam(entry, &stack).is_ok_and(|(form, seam)| match seam {
            Some(at) => &form[at..] == tail,
            // A derivation rebuilds the form rather than attaching to it, so there is no seam to
            // read; it is concatenative, so the tail is what follows the expansion itself.
            None => form.strip_prefix(word.as_ref()) == Some(tail),
        })
    })
}

/// Cases a suffixed acronym may take.
pub(crate) const ALL_CASES: [Case; 18] = [
    Case::Acc,
    Case::Dat,
    Case::Ins,
    Case::Cau,
    Case::Tra,
    Case::Ter,
    Case::Ess,
    Case::Ine,
    Case::Ill,
    Case::Ela,
    Case::Sup,
    Case::Sbl,
    Case::Del,
    Case::Ade,
    Case::All,
    Case::Abl,
    Case::Tem,
    Case::Formalis,
];

/// The plural and possessive tails, which take the same hyphen treatment.
#[must_use]
pub fn licenses_nominal(base: &str, tail: &str) -> bool {
    if licenses(base, tail) {
        return true;
    }
    // Read out by letter names the acronym ends in a vowel (`ká-ká-vé`), read as a word in its
    // last letter, so both are tried, as [`written_suffixes`] does for the cases: `KKV-k`.
    let final_letter = letters(base).last().map(|s| s.letter);
    [None, final_letter]
        .iter()
        .any(|f| nominal_tails(*f).iter().any(|t| t == tail))
}

/// The written plural and third-singular possessive after a stem with this final letter, under
/// every harmony class.
fn nominal_tails(final_letter: Option<Letter>) -> Vec<String> {
    let mut out = Vec::new();
    for h in [Harmony::Back, Harmony::Front, Harmony::FrontRounded] {
        let e = probe(h, final_letter);
        for stack in [
            vec![Morph::Plural],
            vec![Morph::Possessive {
                person: crate::features::Person::Third,
                number: crate::features::Number::Sing,
            }],
            vec![Morph::Anaphoric],
        ] {
            // Read off the seam rather than the lemma: a vowel-final probe lengthens before the
            // plural (`a` + `-k` is `ák`), so the lemma is no longer a prefix of the form.
            if let Ok((form, Some(at))) = crate::generate::generate_seam(&e, &stack)
                && !form[at..].is_empty()
                && !out.iter().any(|x| x == &form[at..])
            {
                out.push(form[at..].to_owned());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{licenses, licenses_expansion, licenses_nominal};

    #[test]
    fn symbols_are_suffixed_by_their_expansion() {
        // PH-G2P-013 and PND-FRAC-006 output lines.
        for (base, tail) in [
            ("15%", "kal"),
            ("80%", "os"),
            ("%", "ot"),
            ("%", "nak"),
            ("%", "ban"),
            ("%", "nyi"),
            ("°C", "on"),
            ("°C", "kal"),
            ("°C", "os"),
            ("§", "ban"),
            ("m²", "es"),
            ("Ft", "ot"),
            ("Ft", "tal"),
        ] {
            assert!(licenses_expansion(base, tail), "{base}-{tail}");
        }
        // The harmony of the expansion, not of the glyph: `százalék` is back.
        assert!(!licenses_expansion("80%", "es"));
        assert!(!licenses_expansion("15%", "kel"));
    }

    #[test]
    fn the_rulebook_examples_are_licensed() {
        // ORT-ABBR-014 and ORT-ABBR-018 output lines.
        for (base, tail) in [
            ("MÁV", "ot"),
            ("MÁV", "val"),
            ("BKV", "val"),
            ("BKV", "vel"),
            ("OTP", "vel"),
            ("OTP", "t"),
            ("ENSZ", "szel"),
            ("DNS", "sel"),
            ("BKK", "nál"),
            ("KSH", "hoz"),
            ("NATO", "hoz"),
            ("MTA", "tól"),
            ("MTA", "nak"),
            ("USA", "ban"),
            ("ELTE", "re"),
            ("CNN", "nek"),
            ("FTC", "vel"),
            ("ORFK", "val"),
            ("SZDSZ", "t"),
        ] {
            assert!(licenses(base, tail), "{base}-{tail} rejected");
        }
    }

    #[test]
    fn derivational_tails_are_licensed() {
        for (base, tail) in [
            ("tbc", "s"),
            ("MAFC", "os"),
            ("TDK", "s"),
            ("ENSZ", "beli"),
            ("USA", "beli"),
        ] {
            assert!(licenses(base, tail), "{base}-{tail} rejected");
        }
    }

    #[test]
    fn a_tail_no_reading_could_give_is_refused() {
        for tail in ["xyz", "qqq", "bam", "zzz"] {
            assert!(!licenses("MÁV", tail), "MÁV-{tail} wrongly accepted");
        }
    }

    #[test]
    fn plural_and_possessive_tails() {
        assert!(licenses_nominal("MÁV", "ok"));
        assert!(licenses_nominal("ENSZ", "ek"));
        assert!(!licenses_nominal("MÁV", "qqq"));
    }
}
