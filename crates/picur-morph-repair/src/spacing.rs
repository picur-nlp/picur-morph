//! Spacing repairs: words a space has broken apart (`REP-SPACE`).

use picur_morph_lexicon::{Pos, frequency, lexicon};
use picur_morph_ortho::letters;

use crate::{Fix, unvouched, vouched};

/// Fewest letters either side of a space repair must have.
///
/// A single letter is an accepted word here, because the alphabet's letters have names that are
/// written and inflected, and they carry real corpus frequency: `t` 25, `l` 23. That makes every
/// long word splittable one letter from its end (`borja t`), which the rule's oracle condition
/// cannot see on its own.
const MIN_PART_LETTERS: usize = 2;

/// Whether a piece is long enough to be a word of a space repair rather than a letter name.
///
/// It must also be spelled with letters alone: a digit, a symbol or a hyphen puts the joined
/// string under the designation and hyphen clauses, which accept `Fallout4-ig` and `Az` before
/// `iowa-i` without either being a word.
fn substantial(word: &str) -> bool {
    letters(word).count() >= MIN_PART_LETTERS && word.chars().all(char::is_alphabetic)
}

/// Rejoins a word split across a space (`REP-SPACE-009`).
///
/// The oracle does all the deciding, so no word can be invented: a join is proposed only where the
/// pieces do not stand on their own and the whole does. That is also why no list of function words
/// appears here although the rule names one. A name is never one half of a split word, so `Elvis`
/// in `Elvis Presley-nek` counts as vouched for and the two are not joined into a longer name.
///
/// `REP-SPACE-009` covers splits and fusions alike, and they are decided by different evidence.
/// A split asks for the *whole* to be a word, which cannot invent one; a fusion asks for a cut,
/// and a cut licensed by the oracle alone falls on correct words the checker wrongly rejects
/// (`zászló ja`, `arat tat`). The fusion lives in [`fusion`], where the rule's own two further
/// conditions supply the evidence: the cut falls after a function word, and the fused string is
/// one the corpus has never seen.
///
/// Rules: `REP-SPACE-009`
pub(crate) fn spacing_fixes(text: &str, words: &[(usize, &str)]) -> Vec<Fix> {
    /// Most pieces a word is reassembled from: a line of hyphenation breaks a word in two, a
    /// column of them in a few (`REP-SPACE-003`: `ren de le tei`).
    const MAX_PIECES: usize = 4;
    let mut out: Vec<Fix> = Vec::new();
    let spaced = |j: usize| text.get(words[j].0 + words[j].1.len()..words[j + 1].0) == Some(" ");
    let mut i = 0usize;
    while i < words.len() {
        let mut found: Option<(usize, usize)> = None;
        // The shortest run of pieces around this one that is a word, where the pieces do not all
        // stand on their own. Compounding is productive, so `túlszabályozás` before
        // `szükségtelen` is read as one compound and accepted, and `vizsgálatl` before `és` as
        // `vizsgálat`, `lé` and `-s`; that acceptance only restates the pieces. The whole must be
        // a listed word or a form of one.
        'runs: for len in 2..=MAX_PIECES {
            for start in i.saturating_sub(len - 1)..=i {
                let end = start + len - 1;
                if end >= words.len() || start > i {
                    continue;
                }
                let run = &words[start..=end];
                if !run.iter().all(|(_, w)| substantial(w))
                    || !(start..end).all(spaced)
                    || run.iter().all(|(_, w)| !unvouched(w))
                {
                    continue;
                }
                let whole: String = run.iter().map(|(_, w)| *w).collect();
                if vouched(&whole) {
                    found = Some((start, end));
                    break 'runs;
                }
            }
        }
        if let Some((start, end)) = found {
            let whole: String = words[start..=end].iter().map(|(_, w)| *w).collect();
            out.push(Fix {
                at: words[start].0,
                from: text[words[start].0..words[end].0 + words[end].1.len()].into(),
                to: whole.into_boxed_str(),
                rule: if end - start == 1 {
                    "REP-SPACE-009"
                } else {
                    "REP-SPACE-003"
                },
            });
            i = end + 1;
        } else {
            i += 1;
        }
    }
    out
}

/// The parts of speech that license a cut (`REP-SPACE-009`).
///
/// The rule names the function words a fusion happens around (`a az egy és hogy nem is meg`), and
/// what they have in common is their category rather than their spelling, so the lexicon answers
/// it and no list appears here. These are the closed classes: the article and the determiners, the
/// conjunctions, the pronouns and the postpositions.
const CLOSED: [Pos; 4] = [
    Pos::Determiner,
    Pos::Conjunction,
    Pos::Pronoun,
    Pos::Postposition,
];

/// Fewest letters a fused token must have before it is cut.
///
/// The rule writes six characters, but its own example `hogya` has four letters, so the floor is
/// read as what the two pieces need between them: a function word and a word.
const MIN_FUSED_LETTERS: usize = 4;

/// How common a one-letter piece of a cut must be.
///
/// The alphabet's letters are words, so a cut can always leave one, and [`MIN_PART_LETTERS`] is
/// the floor that keeps the split direction off them. A fusion needs the definite article, which
/// is one letter, so the floor is lifted for a piece commoner than every letter name: the names
/// reach 25 (`t`) and 23 (`l`), the conjunction `s` 26, the article `a` 38 and `az` 35.
const MIN_ONE_LETTER_FREQUENCY: u8 = 30;

/// Whether a piece can stand as a word of a cut.
fn cuttable(piece: &str) -> bool {
    (substantial(piece) || frequency(piece) >= MIN_ONE_LETTER_FREQUENCY) && vouched(piece)
}

/// Whether `word` is a word of a closed class.
///
/// Asked of the analyser, not of the lexicon's listing: a pronoun's case form (`rám`, `nekem`)
/// is a closed-class word that no entry lists, because its paradigm is computed.
fn closed_class(word: &str) -> bool {
    lexicon().get(word).iter().any(|e| CLOSED.contains(&e.pos))
        || picur_morph_spell::analyses(word)
            .iter()
            .any(|a| CLOSED.contains(&a.pos))
}

/// The bucket a form would have if it were every token of the corpus.
///
/// A bucket of the frequency table is `2 log2 p` plus this offset, `p` being the form's share of
/// tokens. The article `a` is 9.2% of the tokens of Webcorpus 2 (786,648,450 of 8,570,080,167)
/// and sits at 37, so the offset is `37 - 2 log2 0.092`, which rounds to 44.
const WHOLE_CORPUS_BUCKET: u8 = 44;

/// The bucket two words written apart would have if they co-occurred by chance: the product of
/// their shares, which on the half-octave scale is a sum less the offset.
fn pair_bucket(left: &str, right: &str) -> u8 {
    (frequency(left) + frequency(right)).saturating_sub(WHOLE_CORPUS_BUCKET)
}

/// Splits a token into the two words a lost space fused (`REP-SPACE-009`, `REP-OCR-002`).
///
/// The oracle is not enough on its own here, because a cut can always be found, so both of the
/// rule's further conditions are load-bearing. The cut falls *after* a function word, which is
/// what the rule's own condition and both of its examples say (a closed-class word on either side
/// cuts `node` into `no de`), and the split pair is at least as frequent as the fused string.
/// With no pair counts to hand the pair is weighed as if its words co-occurred by chance
/// ([`pair_bucket`]), which underestimates a function word's collocations: `hogy a` at 25 beats
/// the fused `hogya` at 5, while the attested `mindenkorra` (8 against 0) and `egyazon` (9
/// against 6) stand. Both halves must be words the lexicon vouches for, and more than one cut
/// qualifying means the rule abstains.
///
/// Rules: `REP-SPACE-009`, `REP-OCR-002`
pub(crate) fn fusion(word: &str) -> Option<(String, &'static str)> {
    if !unvouched(word)
        || letters(word).count() < MIN_FUSED_LETTERS
        || !word.chars().all(char::is_alphabetic)
    {
        return None;
    }
    let fused = frequency(word);
    let mut found: Vec<String> = Vec::new();
    for span in letters(word) {
        if span.at == 0 {
            continue;
        }
        let (left, right) = (&word[..span.at], &word[span.at..]);
        if closed_class(left)
            && cuttable(left)
            && cuttable(right)
            && pair_bucket(left, right) >= fused
        {
            found.push(format!("{left} {right}"));
        }
    }
    match found.as_slice() {
        [only] => Some((only.clone(), "REP-SPACE-009")),
        _ => None,
    }
}
