//! Word-level repairs: sequence rewrites, encoding noise and the guards that keep them off code.

use std::sync::LazyLock;

use picur_morph_lexicon::{frequency, lexicon};
use picur_morph_ortho::case::capitalise;
use picur_morph_ortho::letters;
use picur_morph_spell::{Status, check};

use crate::accent::{ACCENTED, restore_acronym};
use crate::confusion::{confusion_candidates, replace_nth, undoubled_candidates};
use crate::glyph::repair_glyphs;
use crate::sevenbit::seven_bit;
use crate::slip::case_slip;
use crate::spacing::fusion;
use crate::suffix::resuffixed;
use crate::tables::{Rewrite, char_chars, char_strs, pairs, rewrites};
use crate::{Mode, unvouched, vouched};

static ARCHAIC: LazyLock<Vec<Rewrite>> = LazyLock::new(|| rewrites("Archaic"));
static ASSIMILATION: LazyLock<Vec<Rewrite>> = LazyLock::new(|| rewrites("Assimilation"));
static LOOKALIKES: LazyLock<Vec<Rewrite>> = LazyLock::new(|| rewrites("Lookalike"));
static COMPATIBILITY: LazyLock<Vec<(char, &'static str)>> =
    LazyLock::new(|| char_strs("Compatibility"));
static LONG_DIGRAPHS: LazyLock<Vec<(&'static str, &'static str)>> =
    LazyLock::new(|| pairs("LongDigraph"));
static TELEGRAM: LazyLock<Vec<(&'static str, &'static str)>> = LazyLock::new(|| pairs("Telegram"));
static HUNGARIAN_DIGITS: LazyLock<Vec<(char, char)>> =
    LazyLock::new(|| char_chars("HungarianDigit"));

/// A number typed on the Hungarian layout where a digit key gave a letter (`2ö22`): every
/// other character is a digit, so the token is a number and needs no lexicon.
///
/// Rules: `REP-KEY-003`
fn layout_digits(word: &str) -> Option<String> {
    if !word
        .chars()
        .any(|c| HUNGARIAN_DIGITS.iter().any(|(k, _)| *k == c))
    {
        return None;
    }
    let digits: String = word
        .chars()
        .map(|c| {
            HUNGARIAN_DIGITS
                .iter()
                .find(|(k, _)| *k == c)
                .map_or(c, |(_, d)| *d)
        })
        .collect();
    (digits.chars().all(|c| c.is_ascii_digit()) && digits.len() > 1).then_some(digits)
}

/// Letters a token needs before a typing slip is repaired in it; a shorter one is any word.
const MIN_SLIP_LETTERS: usize = 4;

/// The cost of a slip candidate, in the rule's units: a duplicated or dropped duplicate letter is
/// the cheapest slip, a swap of neighbours costs more.
const DUPLICATE_COST: u32 = 8;
const TRANSPOSITION_COST: u32 = 10;
/// A candidate wins outright when the next one costs this much more; otherwise frequency decides.
const CLEAR_MARGIN: u32 = 5;
/// How common a word a slip may land on: a rare word reached by one edit is a coincidence, and in
/// a foreign text every rejected token is one edit from some rare Hungarian word (`keep`, `kepe`).
const MIN_SLIP_FREQUENCY: u8 = 10;

/// An isolated typing slip repaired (`mnidig`, `visza`): the token is no word, and among the
/// swaps of two neighbouring letters, the doubled letters and the dropped doublings that make
/// one, the cheapest wins by a clear margin or the commonest does.
///
/// A real-word slip is left to [`slipped_in_context`], as the rule's conditions say; a name is guarded
/// by the caller, and the caller asks only in a text that carries its accents, because a slip in
/// a foreign or an accent-stripped text is not the likeliest reading of a rejected token.
///
/// Rules: `REP-KEY-007`
fn slipped(word: &str) -> Option<String> {
    if !unvouched(word) {
        return None;
    }
    slip_of(word)
}

/// A real-word slip whose only reading the context rules out, repaired as a non-word one
/// (`REP-KEY-007`): the context is what makes it a non-word.
pub(crate) fn slipped_in_context(word: &str, accented_text: bool) -> Option<String> {
    slip_of(word).filter(|found| has_hungarian_evidence(word, found, accented_text))
}

/// The slip candidate of [`slipped`], without asking whether the token is a word.
fn slip_of(word: &str) -> Option<String> {
    if letters(word).count() < MIN_SLIP_LETTERS {
        return None;
    }
    let spans: Vec<_> = letters(word).collect();
    let mut found: Vec<(u32, String)> = Vec::new();
    let mut offer = |cost: u32, candidate: String| {
        if candidate != word
            && frequency(&candidate) >= MIN_SLIP_FREQUENCY
            && vouched(&candidate)
            && !found.iter().any(|(_, c)| *c == candidate)
        {
            found.push((cost, candidate));
        }
    };
    // A multi-character letter doubles as its long spelling (`sz` to `ssz`), which the digraph
    // candidates write in both directions.
    for candidate in doubled_digraph_candidates(word) {
        offer(DUPLICATE_COST, candidate);
    }
    for (i, span) in spans.iter().enumerate() {
        let before = &word[..span.at];
        let after = &word[span.at + span.text.len()..];
        if span.text.chars().count() == 1 {
            offer(
                DUPLICATE_COST,
                format!("{before}{}{}{after}", span.text, span.text),
            );
            if i > 0 && spans[i - 1].letter == span.letter {
                offer(DUPLICATE_COST, format!("{before}{after}"));
            }
        }
        if let Some(next) = spans.get(i + 1)
            && next.letter != span.letter
        {
            offer(
                TRANSPOSITION_COST,
                format!(
                    "{before}{}{}{}",
                    next.text,
                    span.text,
                    &word[next.at + next.text.len()..]
                ),
            );
        }
    }
    found.sort_by_key(|(cost, _)| *cost);
    match found.as_slice() {
        [] => None,
        [(_, only)] => Some(only.clone()),
        [(c1, first), (c2, second), ..] => {
            if c2 - c1 >= CLEAR_MARGIN {
                return Some(first.clone());
            }
            match frequency(first).cmp(&frequency(second)) {
                std::cmp::Ordering::Greater => Some(first.clone()),
                std::cmp::Ordering::Less => Some(second.clone()),
                std::cmp::Ordering::Equal => None,
            }
        }
    }
}

/// Fewest telegram sequences a token needs to be decoded on its own evidence; one is any
/// doubled vowel, two in one word is the convention.
const MIN_TELEGRAM_SEQUENCES: usize = 2;

/// Telegram-style doubled vowels decoded (`aarviiztueueroeoe`), when the token is ASCII, is no
/// word as written and the decoding is one.
///
/// A real doubled vowel (`kooperáció`, `vákuum`) is a word as written and is never touched; a
/// foreign name (`Goethe`, `Mueller`) is guarded by the caller. A token with one sequence is
/// decoded only where the text carries its accents, which the caller checks.
///
/// Rules: `REP-ENC-014`
fn telegram(word: &str, accented_text: bool) -> Option<String> {
    let sequences: usize = TELEGRAM
        .iter()
        .map(|(seq, _)| word.matches(seq).count())
        .sum();
    if !word.is_ascii()
        || sequences == 0
        || (!accented_text && sequences < MIN_TELEGRAM_SEQUENCES)
        || !unvouched(word)
    {
        return None;
    }
    let mut decoded = word.to_owned();
    for (seq, letter) in TELEGRAM.iter() {
        decoded = decoded.replace(seq, letter);
    }
    (decoded != word && check(&decoded) == Status::Ok).then_some(decoded)
}

/// `text` with every compatibility code point expanded.
///
/// Rules: `WS-CHAR-010`
#[must_use]
pub fn expand_compatibility(text: &str) -> String {
    if !text
        .chars()
        .any(|c| COMPATIBILITY.iter().any(|(k, _)| *k == c))
    {
        return text.to_owned();
    }
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match COMPATIBILITY.iter().find(|(k, _)| *k == c) {
            Some((_, to)) => out.push_str(to),
            None => out.push(c),
        }
    }
    out
}

/// Whether `word` carries a bigram a layout slip writes and Hungarian never does
/// (`REP-KEY-004`): `gy ny ty ly` come out `gz nz tz lz`, `sz zs` come out `sy ys`.
fn layout_signature(word: &str) -> bool {
    ["gz", "nz", "tz", "lz", "sy", "ys"]
        .iter()
        .any(|b| word.contains(b))
}

/// `y` and `z` exchanged, which is the whole of a QWERTZ/QWERTY layout slip (`REP-KEY-004`).
fn swap_y_z(word: &str) -> String {
    word.chars()
        .map(|c| match c {
            'y' => 'z',
            'z' => 'y',
            'Y' => 'Z',
            'Z' => 'Y',
            other => other,
        })
        .collect()
}

/// Words differing from `word` by one multi-character consonant's length (`REP-SPELL-005`).
///
/// Both directions, because both are errors: writing the long consonant single (`menyi` for
/// `mennyi`) and doubling one that is short (`annya` for `anya`). One position at a time, so a
/// word carrying the sequence twice is not rewritten in both places at once.
///
/// Nothing here decides; the caller keeps a candidate only where the lexicon vouches for it, which
/// is what leaves `kulcscsomó` and `vadság` alone.
///
/// That licensing also puts one of the rule's examples out of reach. `menyi` is a word: `meny`
/// plus the relational `-i`, vouched for and never a candidate. The rule separates it from
/// `mennyi` by attestation, 0 against 17, and rewriting a word the lexicon vouches for on a
/// frequency alone is not a repair this crate makes.
fn doubled_digraph_candidates(word: &str) -> Vec<String> {
    let mut out = Vec::new();
    for &(short, long) in LONG_DIGRAPHS.iter() {
        for n in 0..word.matches(long).count() {
            if let Some(c) = replace_nth(word, long, short, n) {
                out.push(c);
            }
        }
        // A `dzs` inside the word is not a `dz`, and the longer letter was tried first.
        let singles = word.matches(short).count() - word.matches(long).count();
        for n in 0..singles {
            if let Some(c) = replace_nth(word, short, long, n) {
                out.push(c);
            }
        }
    }
    out
}

/// The other ways the same long multi-character consonant is written (`REP-SPELL-005`).
///
/// Two spellings of a letter that is already long, so neither changes the word's length: the full
/// doubling a compound seam requires and a simplified spelling drops (`jeggyűrű` for `jegygyűrű`),
/// and the doubling of the second character instead of the first (`edzze` for `eddze`, which
/// `REP-ARCH-007` dates to before 1954).
///
/// Being the same letter is why these need no attestation, unlike a length change: `kulcscsomó`
/// and `kulccsomó` are both unattested and only one of them is the word.
///
/// Rules: `REP-SPELL-005`, `REP-ARCH-007`, `ORT-ASSIM-016`
fn respelled_digraph_candidates(word: &str) -> Vec<String> {
    let mut out = Vec::new();
    for &(short, long) in LONG_DIGRAPHS.iter() {
        for n in 0..word.matches(long).count() {
            if let Some(c) = replace_nth(word, long, &format!("{short}{short}"), n) {
                out.push(c);
            }
        }
        let Some(last) = short.chars().next_back() else {
            continue;
        };
        let wrong = format!("{short}{last}");
        for n in 0..word.matches(wrong.as_str()).count() {
            if let Some(c) = replace_nth(word, &wrong, long, n) {
                out.push(c);
            }
        }
    }
    out
}

/// Whether a token is a name, which the archaic rules leave alone.
///
/// `REP-ARCH-002` and `REP-ARCH-003` both except names: `Móricz`, `Babits` and `Kováts` keep their
/// spelling. Capitalisation is the only signal available without a name lexicon, so a capitalised
/// token is not modernised. That is the conservative direction: it declines a repair rather than
/// rewriting somebody's name.
fn looks_like_a_name(word: &str) -> bool {
    word.chars().next().is_some_and(char::is_uppercase)
}

/// Tokens that no repair rule may touch.
///
/// `REP-DIAC-009` lists them, and the reason is that a repair here is damage rather than
/// correction: an address, an identifier or an acronym is not misspelled Hungarian. The name clause
/// is handled separately by [`looks_like_a_name`], which is stricter than the rule allows and
/// declines more than it must.
///
/// Rules: `REP-DIAC-009`
pub(crate) fn is_guarded(word: &str) -> bool {
    if word.contains("://")
        || word.starts_with("www.")
        || word.contains('@')
        || word.contains('/')
        || word.contains('\\')
    {
        return true;
    }
    // Code: an underscore or internal capitals. A digit is *not* guarded here, because
    // `REP-OCR-001` repairs exactly the digit-for-letter confusions OCR produces (`1ehet` for
    // `lehet`); the digit clause of `REP-DIAC-009` belongs to restoration and lives there.
    if word.contains('_') {
        return true;
    }
    if word
        .chars()
        .skip(1)
        .any(|c| c.is_uppercase() && word.chars().next().is_some_and(char::is_lowercase))
    {
        return true;
    }
    if word.starts_with('#') || word.starts_with('@') {
        return true;
    }
    // An all-caps token is an acronym, not a word with lost accents.
    let letters_only = word.chars().filter(|c| c.is_alphabetic()).count();
    if letters_only > 1
        && word
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(char::is_uppercase)
    {
        return true;
    }
    false
}

/// The repair for one token, if any rule licenses one.
///
/// Rules: `REP-ARCH-002` .. `REP-ARCH-005`
#[must_use]
pub fn repair_word(word: &str, mode: Mode) -> Option<(String, &'static str)> {
    repair_word_in(word, mode, false)
}

/// [`repair_word`] for a token taken from a text that is known to carry its accents.
#[must_use]
pub fn repair_word_accented(word: &str, mode: Mode) -> Option<(String, &'static str)> {
    repair_word_in(word, mode, true)
}

/// [`repair_word`] for a token from a text whose accents the caller has judged.
///
/// The token-level evidence gate declines to put an accent into an all-ASCII token because the
/// token alone could be any language. A text that carries its accents is that evidence: an
/// unaccented token in it is `REP-SPELL-002`'s `hosszu`, not an English word.
pub(crate) fn repair_word_in(
    word: &str,
    mode: Mode,
    accented_text: bool,
) -> Option<(String, &'static str)> {
    if let Some(found) = repair_glyphs(word) {
        return Some(found);
    }
    if let Some(found) = layout_digits(word) {
        return Some((found, "REP-KEY-003"));
    }
    // `REP-FONT-007`: a styled letter is the letter, whatever the token spells.
    let plain = crate::glyph::plain_letters(word);
    if plain != word {
        return Some((plain, "REP-FONT-007"));
    }
    if let Some(found) = case_slip(word) {
        return Some(found);
    }
    // `REP-DIAC-012`: an all-capital token is guarded as an acronym below, which is right, and the
    // one repair an acronym has is the accents the capitals lost.
    if mode == Mode::Modernise
        && let Some(found) = restore_acronym(word)
    {
        return Some((found, "REP-DIAC-012"));
    }
    if is_guarded(word) {
        return None;
    }
    if let Some(found) = seven_bit(word) {
        return Some(found);
    }
    if check(word) != Status::Ok {
        for w in LOOKALIKES.iter() {
            if !word.contains(w.from) {
                continue;
            }
            let candidate = word.replace(w.from, w.to);
            if candidate != word && check(&candidate) == Status::Ok {
                return Some((candidate, w.rule));
            }
        }
    }
    // `REP-KEY-004`: the QWERTZ/QWERTY confusion swaps `y` and `z` and nothing else, which in
    // Hungarian is conspicuous because `gy ny ty ly` come out `gz nz tz lz` and `sz` comes out
    // `sy`. A wrong layout is damage rather than a choice, so it is repaired in every mode. The
    // rule's exceptions need no statement here: `egzotikus` and `egzakt` are accepted as written,
    // so they are never candidates. A capitalised word the lexicon does not list is a name
    // (`Gipsy`, `Banzai`), and a slip that lands on a name or a coinage (`Nzelvtan-a`) is no
    // repair. An unlisted lowercase word is a candidate only when it shows the rule's own
    // signature, one of the bigrams the swap writes; a rejected one needs no further evidence.
    if check(word) == Status::Wrong || (unvouched(word) && layout_signature(word)) {
        let swapped = swap_y_z(word);
        if swapped != word && vouched(&swapped) {
            return Some((swapped, "REP-KEY-004"));
        }
    }
    if let Some(found) = telegram(word, accented_text) {
        return Some((found, "REP-ENC-014"));
    }
    if mode == Mode::Modernise
        && let Some(found) = modernised(word, accented_text)
    {
        return Some(found);
    }
    // `REP-OCR-002` puts a lost space last of the token-level stages, after the spellings: `visza`
    // is `vissza` before it is `visz a`.
    if let Some(found) = fusion(word) {
        return Some(found);
    }
    if let Some(found) = slipped(word)
        && has_hungarian_evidence(word, &found, accented_text)
    {
        return Some((found, "REP-KEY-007"));
    }
    None
}

/// The repairs of a spelling the text chose under an older or a careless norm (`Mode::Modernise`).
fn modernised(word: &str, accented_text: bool) -> Option<(String, &'static str)> {
    // A hyphen wrongly present or wrongly absent is a single string change, and unlike the letter
    // rules it applies to exactly the capitalised tokens `looks_like_a_name` declines: an acronym
    // and a foreign name are what `REP-SPELL-012` is about (`MÁV-nál`, `Glasgow-ban`). Nothing is
    // rewritten inside the name itself.
    if check(word) != Status::Ok
        && let [only] = hyphen_candidates(word).as_slice()
    {
        return Some((only.clone(), "REP-SPELL-012"));
    }
    if let Some(found) = resuffixed(word) {
        return Some(found);
    }
    if looks_like_a_name(word) {
        return None;
    }
    // The rules below fire only on a token the lexicon cannot vouch for, which is wider than
    // rejection: an archaic spelling usually has no analysis at all rather than a bad one, and the
    // checker reads `végkép` as a coined compound. A correct word containing the same sequence,
    // like `vadság` or `kulcscsomó`, is vouched for and therefore never touched; every candidate
    // below must be vouched for and commoner than the token.
    if vouched(word) {
        return None;
    }
    for w in ARCHAIC.iter().chain(ASSIMILATION.iter()) {
        if !word.contains(w.from) {
            continue;
        }
        let candidate = word.replace(w.from, w.to);
        if candidate != word && vouched(&candidate) {
            return Some((candidate, w.rule));
        }
    }
    // `REP-SPELL-005`: a long multi-character consonant is written with its first letter doubled,
    // and the errors are writing it single or doubling one that should not be.
    //
    // The repair must also make the word commoner. Acceptance alone is not enough here, because
    // the checker refuses some correct forms and the rule would then "fix" them: `látsszék` is a
    // standard variant that comes back unlisted, and shortening it reaches `látszék`, which the
    // checker accepts and the corpus has never seen. Both stand at zero, so the guard holds.
    for candidate in doubled_digraph_candidates(word) {
        if vouched(&candidate) && frequency(&candidate) > frequency(word) {
            return Some((candidate, "REP-SPELL-005"));
        }
    }
    for candidate in respelled_digraph_candidates(word) {
        if vouched(&candidate) {
            return Some((candidate, "REP-SPELL-005"));
        }
    }
    // `REP-SPELL-004`: a consonant doubled where the word writes one.
    for candidate in undoubled_candidates(word) {
        if has_hungarian_evidence(word, &candidate, accented_text)
            && vouched(&candidate)
            && frequency(&candidate) > frequency(word)
            && frequency(&candidate) >= MIN_UNDOUBLED_FREQUENCY
        {
            return Some((candidate, "REP-SPELL-004"));
        }
    }
    // `REP-SPELL-016`: a three-consonant junction written the wrong way.
    for candidate in triple_consonant_candidates(word) {
        if candidate != word && vouched(&candidate) {
            return Some((candidate, "REP-SPELL-016"));
        }
    }
    // A single toggle from one of the confusion sets, accepted only when it is the unique reading.
    // `REP-SPELL-001` says real-word pairs are to be kept and reported rather than changed, so an
    // ambiguous toggle abstains.
    let mut accepted: Vec<(String, &'static str)> = Vec::new();
    for (candidate, rule) in confusion_candidates(word) {
        if candidate != word
            && has_hungarian_evidence(word, &candidate, accented_text)
            && frequency(&candidate) > frequency(word)
            && vouched(&candidate)
            && !accepted.iter().any(|(c, _)| *c == candidate)
        {
            accepted.push((candidate, rule));
        }
    }
    match accepted.as_slice() {
        [(one, rule)] => Some((one.clone(), rule)),
        many if many.len() > 1 => {
            let mut ranked: Vec<&(String, &'static str)> = many.iter().collect();
            ranked.sort_by_key(|(c, _)| std::cmp::Reverse(frequency(c)));
            let top = frequency(&ranked[0].0);
            if top > frequency(&ranked[1].0) {
                Some((ranked[0].0.clone(), ranked[0].1))
            } else {
                None
            }
        }
        _ => None,
    }
}

// A correction must make the word commoner. The checker accepts a novel compound, which is what it
// is for, but that makes it a poor test for a repair: `fifa` split as `ffi|fa` gives `ffifa`,
// `color` as `col|orr` gives `colorr`, and `jobbikos` as `jobbik|kos` gives `jobbikkos`, all
// accepted and all nonsense. A form rarer than the one it replaces is not what the writer meant.

/// Whether rewriting `word` as `candidate` rests on any evidence that `word` is Hungarian.
///
/// A token written entirely in ASCII could be any language, and toggling a length in one reaches
/// a Hungarian word often enough to be worthless as evidence. Measured over the commonest 20,000
/// Webcorpus 2 types, the length rules turned `to` into `tó`, `all` into `áll`, `art` into `árt`,
/// `real` into `reál`, `in` into `ín`, `it` into `itt` and `for` into `forr`, every one of them
/// English. So a repair that puts a Hungarian accent into an all-ASCII token is declined, and so
/// is any rewrite of an all-ASCII token of three letters or fewer.
///
/// The rules' own examples all survive: they either carry an accent already (`rendör`, `ütö`,
/// `kertböl`) or stay inside ASCII (`bizotság`, `ezel`, `visza`, `rnegye`).
///
/// This is the token-level counterpart of `REP-DIAC-001`, which abstains on a short segment for
/// the same reason: too little of the language to judge. A text that carries its accents supplies
/// the evidence the token lacks, and then only the three-letter floor stands: `to`, `for` and
/// `all` stay as they are even in Hungarian text, `hosszu` and `ujra` do not.
pub(crate) fn has_hungarian_evidence(word: &str, candidate: &str, accented_text: bool) -> bool {
    if word.chars().any(|c| ACCENTED.contains(c)) || !word.is_ascii() {
        return true;
    }
    if candidate.chars().any(|c| ACCENTED.contains(c)) && !accented_text {
        return false;
    }
    word.chars().filter(|c| c.is_alphabetic()).count() > 3
}

/// How common the shorter form must be before a doubled consonant is undoubled.
///
/// `REP-SPELL-004` repairs a consonant doubled where the word writes one (`kissebb` for `kisebb`),
/// and removing a letter the writer put there needs more than acceptance and more than a frequency
/// gain. Measured: `orrt` is ill formed (`NOM-ACC-006`) and undoubling reaches `ort`, a listed word
/// attested 3 against `orrt`'s 0, so both of those tests hold for a repair that is wrong. The
/// shorter form has to be a common word, which `kisebb` is at 20 and `ort` is not.
///
/// The token-level evidence gate carries the rest: English `all` undoubles to the Hungarian `al`
/// at 11, and the three-letter floor of [`has_hungarian_evidence`] is what declines it.
const MIN_UNDOUBLED_FREQUENCY: u8 = 10;

/// The letters a compound member needs, as `DER-COMP` counts them.
const MIN_MEMBER_LETTERS: usize = 3;

/// The forms a token with a three-consonant junction could have been meant as.
///
/// `REP-SPELL-016` names three errors and `ORT-ASSIM-017` the norm behind them. Where suffixation
/// or compounding brings three of the same consonant letter together, a common word writes two
/// and a proper name or compound boundary keeps all three with a hyphen. So the errors are a
/// triple written solid (`sakkkör`), a simplification where the hyphen was required (`sakkör`,
/// `balettáncos`), and a hyphen where the simplification was required (`toll-lal`).
///
/// Rules: `REP-SPELL-016`, `ORT-ASSIM-017`, `ORT-HYPH-001`
fn triple_consonant_candidates(word: &str) -> Vec<String> {
    let chars: Vec<char> = word.chars().collect();
    let offset = |i: usize| chars[..i].iter().map(|c| c.len_utf8()).sum::<usize>();
    let is_consonant = |c: char| c.is_alphabetic() && !"aáeéiíoóöőuúüű".contains(c);
    let mut out = Vec::new();

    if let Some((base, tail)) = word.split_once('-')
        && let Some(last) = base.chars().last()
        && tail.starts_with(last)
        && base.chars().rev().nth(1) == Some(last)
    {
        out.push(format!("{base}{}", &tail[last.len_utf8()..]));
    }

    let lower: Vec<char> = word.chars().flat_map(char::to_lowercase).collect();
    if lower.len() != chars.len() {
        return out;
    }
    for i in 0..chars.len() {
        if i + 2 < chars.len()
            && is_consonant(lower[i])
            && lower[i] == lower[i + 1]
            && lower[i + 1] == lower[i + 2]
        {
            let at = offset(i + 2);
            out.push(format!("{}-{}", &word[..at], &word[at..]));
        }
        // A doubled consonant followed by something else: the third letter was dropped, and both
        // it and the hyphen come back (`sakkör` -> `sakk-kör`, `Széllel` -> `Széll-lel`). What
        // comes back has to be a word in its own right, or any doubled consonant would sprout a
        // hyphen: `orrt` is a misspelling of `orrot`, not `orr-rt`.
        if i + 1 < chars.len()
            && is_consonant(lower[i])
            && lower[i] == lower[i + 1]
            && chars.get(i + 2).is_some_and(|c| *c != chars[i + 1])
        {
            let at = offset(i + 2);
            let restored = format!("{}{}", chars[i + 1], &word[at..]);
            // Both sides must be able to be members, by the compound splitter's three-letter floor, and the restored one must be a word.
            if letters(&restored).count() >= MIN_MEMBER_LETTERS
                && letters(&word[..at]).count() >= MIN_MEMBER_LETTERS
                && check(&restored) == Status::Ok
            {
                out.push(format!("{}-{restored}", &word[..at]));
            }
        }
    }
    out
}

/// The accepted forms of `word` that differ from it only by a hyphen before the suffix.
///
/// `REP-SPELL-012`: a suffix attaches with a hyphen to a digit, an acronym or a foreign name whose
/// final letter is silent, and directly otherwise, so both a missing and a spurious hyphen are
/// errors of the same rule. `1848ban` wants `1848-ban` and `ház-ban` wants `házban`.
///
/// The checker decides, as everywhere in this module: the hyphenated candidate is accepted only
/// when the tail really is a written suffix the base licenses, which is what stops a hyphen being
/// inserted at an arbitrary point. More than one accepted candidate means the rule abstains.
fn hyphen_candidates(word: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if let Some((base, tail)) = word.split_once('-') {
        // A spurious hyphen: the two halves simply join. The tail must be a suffix and the whole a
        // word the lexicon vouches for. A digit range (`1999-2000-es`) is not the rule's error,
        // and a name's hyphen follows its pronunciation (`ORT-FOR-012`: `seattle-i`), which the
        // lexicon does not always carry, so a listed name is left as written.
        if !base.is_empty()
            && !tail.is_empty()
            && !tail.contains(['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
            && !base.ends_with(|c: char| c.is_ascii_digit())
            && lexicon().get(&capitalise(base)).is_empty()
        {
            let joined = format!("{base}{tail}");
            if vouched(&joined) {
                out.push(joined);
            }
        }
        return out;
    }
    for span in letters(word) {
        if span.at == 0 {
            continue;
        }
        let (base, tail) = (&word[..span.at], &word[span.at..]);
        // Only a base that takes a hyphenated suffix at all, or `kints` is "repaired" to
        // `kint-s`: the checker accepts that as an ordinary hyphenated compound, so it cannot be
        // the only guard. `ORT-ABBR-009` and `PND-NUMINF-010` name the hosts, and both are shapes
        // rather than lexical facts.
        if !takes_a_hyphenated_suffix(base) {
            continue;
        }
        let candidate = format!("{base}-{tail}");
        if check(&candidate) == Status::Ok && !out.contains(&candidate) {
            out.push(candidate);
        }
    }
    out
}

/// Whether `base` is the kind of host whose suffix is written after a hyphen.
///
/// A number (`PND-NUMINF-010`) or an acronym (`ORT-ABBR-009`); both are decided by shape.
///
/// Rules: `ORT-ABBR-009`, `PND-NUMINF-010`
fn takes_a_hyphenated_suffix(base: &str) -> bool {
    let alpha: Vec<char> = base.chars().filter(|c| c.is_alphabetic()).collect();
    let digits = base.chars().any(char::is_numeric);
    let all_caps = !alpha.is_empty() && alpha.iter().all(|c| c.is_uppercase());
    (digits && alpha.is_empty()) || (all_caps && alpha.len() > 1)
}

#[cfg(test)]
mod keyboard_tests {
    use super::{Mode, repair_word};

    fn repaired(word: &str) -> Option<String> {
        repair_word(word, Mode::Conservative).map(|(w, _)| w)
    }

    /// `REP-KEY-004`: the rule's own examples.
    #[test]
    fn rep_key_004_a_wrong_layout_swaps_y_and_z() {
        for (typed, want) in [
            ("hogz", "hogy"),
            ("egz", "egy"),
            ("nagz", "nagy"),
            ("yöld", "zöld"),
            ("kösyönöm", "köszönöm"),
        ] {
            assert_eq!(repaired(typed).as_deref(), Some(want), "{typed}");
        }
    }

    /// The oracle keeps the words that really are written with `gz`.
    #[test]
    fn rep_key_004_a_legitimate_gz_is_left_alone() {
        for word in ["egzotikus", "egzakt", "ház", "szöveg"] {
            assert_eq!(repaired(word), None, "{word}");
        }
    }
}

#[cfg(test)]
mod digraph_length_tests {
    use super::{Mode, repair_word};

    fn repaired(word: &str) -> Option<String> {
        repair_word(word, Mode::Modernise).map(|(w, _)| w)
    }

    /// `REP-SPELL-005`: the long consonant written single, and the short one doubled.
    #[test]
    fn rep_spell_005_a_multi_character_consonant_gets_its_length() {
        for (written, want) in [
            ("hoszú", "hosszú"),
            ("visza", "vissza"),
            ("aszony", "asszony"),
            ("annya", "anya"),
        ] {
            assert_eq!(repaired(written).as_deref(), Some(want), "{written}");
        }
    }

    /// A word that really is written that way is accepted, and never a candidate.
    #[test]
    fn rep_spell_005_a_correct_sequence_is_left_alone() {
        for word in ["kulcscsomó", "vadság", "asszony", "mennyi", "anya"] {
            assert_eq!(repaired(word), None, "{word}");
        }
    }
}

#[cfg(test)]
mod encoding_guard_tests {
    use super::{Mode, repair_word};

    fn repaired(word: &str) -> Option<String> {
        repair_word(word, Mode::Modernise).map(|(w, _)| w)
    }

    /// `REP-ENC-003`: misdecoded UTF-8 writes the internal capitals the code guard keys on, so the
    /// code page is read before the guard runs.
    #[test]
    fn rep_enc_003_mojibake_is_repaired_although_it_looks_like_code() {
        for (damaged, want) in [
            ("hasznĂĄlatĂĄhoz", "használatához"),
            ("szĂłtĂˇr", "szótár"),
            ("tĹ±z", "tűz"),
            ("KĂ©rem", "Kérem"),
        ] {
            assert_eq!(repaired(damaged).as_deref(), Some(want), "{damaged}");
        }
    }

    /// What the guard is for is still guarded: none of these carries a code-page signature.
    #[test]
    fn rep_diac_009_code_is_still_left_alone() {
        for word in [
            "some_var",
            "camelCase",
            "http://example.hu/x",
            "user@example.hu",
        ] {
            assert_eq!(repaired(word), None, "{word}");
        }
    }
}
