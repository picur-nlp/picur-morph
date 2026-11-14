//! Spelling numbers out in words.
//!
//! Almost all of chapter 04 is algorithm rather than lexicon: the base numerals are a closed list
//! of twenty-odd words, and everything else is composition. The three things that are *not*
//! obvious, and which this module exists to get right, are the choice between `két` and `kettő`
//! (`PND-CARD-004`), where `egy` appears before a scale word (`PND-CARD-005`), and the hyphen rule
//! above 2000 (`PND-CARD-006`).
//!
//! See `docs/grammar/04-pronouns-numerals-determiners.md`.

use std::sync::LazyLock;

use picur_morph_lexicon::{Entry, Pos, Role, Trait, TraitSet, lexicon, lists, rows, words};

use crate::features::{Case, Morph};
use crate::generate::{GenError, generate, generate_seam};

/// One spelled token, with the three shapes a numeral word can take.
#[derive(Clone, Copy, Debug)]
struct Tok {
    /// Standing alone or ending the number: `kettő`.
    indep: &'static str,
    /// Followed by another word: `két`.
    attr: &'static str,
    /// As the last element of an ordinal: `kettedik`.
    ord: &'static str,
}

/// A word that is the same in every position.
const fn plain(text: &'static str, ord: &'static str) -> Tok {
    Tok {
        indep: text,
        attr: text,
        ord,
    }
}

/// A row of the inventory as a token: independent, attributive, ordinal.
fn tok_of(row: &'static [Box<str>]) -> Tok {
    Tok {
        indep: &row[0],
        attr: &row[1],
        ord: &row[2],
    }
}

/// `PND-CARD-001`, `PND-ORD-001`: the units; only `kettő` distinguishes independent from
/// attributive.
static UNITS: LazyLock<Vec<Tok>> =
    LazyLock::new(|| rows(Role::NumeralUnit).iter().map(|r| tok_of(r)).collect());
/// `PND-CARD-001`: the tens, indexed by the tens digit.
static TENS: LazyLock<Vec<Tok>> =
    LazyLock::new(|| rows(Role::NumeralTen).iter().map(|r| tok_of(r)).collect());
/// `PND-CARD-002`: 11-19 and 21-29 use a bound stem with a short vowel, not the free `tíz`/`húsz`.
static TIZEN: LazyLock<&'static str> = LazyLock::new(|| &words(Role::NumeralBoundTen)[0]);
static HUSZON: LazyLock<&'static str> = LazyLock::new(|| &words(Role::NumeralBoundTen)[1]);
static SZAZ: LazyLock<Tok> = LazyLock::new(|| tok_of(&rows(Role::NumeralHundred)[0]));
/// `PND-CARD-001`: the scale words, descending. Long scale, so `billió` is 10^12.
static SCALES: LazyLock<Vec<(u64, Tok)>> = LazyLock::new(|| {
    rows(Role::NumeralScale)
        .iter()
        .filter_map(|r| Some((r[0].parse().ok()?, plain(&r[1], &r[2]))))
        .collect()
});

/// Above this the three-digit groups are separated by hyphens.
///
/// Rules: `PND-CARD-006`
const HYPHEN_ABOVE: u64 = 2000;

/// The tokens spelling a group of 1..=999, most significant first.
///
/// Rules: `PND-CARD-002`, `PND-CARD-003`, `PND-CARD-005`
#[expect(
    clippy::cast_possible_truncation,
    reason = "each index is a single decimal digit, 0..=9"
)]
fn group_tokens(g: u64) -> Vec<Tok> {
    let mut out = Vec::with_capacity(4);
    let (h, t, u) = (g / 100, (g / 10) % 10, g % 10);
    if h > 0 {
        // PND-CARD-005: a hundreds digit of 1 is silent. The corpus does show `ezeregyszáz`
        // outnumbering `ezerszáz`, but the rule states the bare form, so that is what is generated.
        if h > 1 {
            out.push(UNITS[h as usize]);
        }
        out.push(*SZAZ);
    }
    match (t, u) {
        // PND-CARD-002: the bound stems appear only when a unit follows.
        (1, 1..=9) => out.push(plain(*TIZEN, "")),
        (2, 1..=9) => out.push(plain(*HUSZON, "")),
        (1..=9, _) => out.push(TENS[t as usize]),
        _ => {}
    }
    if u > 0 {
        out.push(UNITS[u as usize]);
    }
    out
}

/// Joins a group's tokens, using the independent and ordinal shapes only on the very last one.
fn render(tokens: &[Tok], last: bool, ordinal: bool) -> String {
    let mut out = String::new();
    for (i, t) in tokens.iter().enumerate() {
        let final_token = last && i + 1 == tokens.len();
        out.push_str(match (final_token, ordinal) {
            (true, true) => t.ord,
            (true, false) => t.indep,
            (false, _) => t.attr,
        });
    }
    out
}

/// The last numeral word of `n`, and everything written before it.
///
/// A composed numeral inflects only on its last element, so this is what inflection needs: the
/// inert prefix, and the lemma to look up. `47 563` splits as `negyvenhétezer-ötszázhatvan` plus
/// `három`.
///
/// Rules: `PND-NUMINF-002`
#[must_use]
pub fn split_last(n: u64) -> (String, &'static str) {
    let whole = cardinal(n);
    let last = last_token(n).indep;
    let cut = whole.len() - last.len();
    (whole[..cut].to_owned(), last)
}

/// The token that ends the spelled-out number.
fn last_token(n: u64) -> Tok {
    if n == 0 {
        return UNITS[0];
    }
    let rest = n % 1000;
    if rest > 0 {
        return *group_tokens(rest).last().unwrap_or(&UNITS[0]);
    }
    // Otherwise the smallest scale word present does. The group digit is read as `n / scale % 1000`
    // rather than `n % (scale * 1000)`, which would overflow at `trillió`.
    for &(scale, word) in SCALES.iter().rev() {
        if !(n / scale).is_multiple_of(1000) {
            return word;
        }
    }
    UNITS[0]
}

/// Spells `n` out and inflects it, which a composed numeral does on its last element only.
///
/// The inflection itself is the ordinary nominal machinery: the last word is looked up as a
/// [`Pos::Numeral`] entry and run through [`generate`], so `negyvenhétezer-ötszázhatvanhármat`
/// needs no numeral-specific suffix rule at all.
///
/// Rules: `PND-NUMINF-001`, `PND-NUMINF-002`
pub fn inflect(n: u64, stack: &[Morph]) -> Result<String, GenError> {
    let (prefix, last) = split_last(n);
    let entry = lexicon()
        .get_pos(last, Pos::Numeral)
        .ok_or(GenError::Unsupported("numeral missing from the lexicon"))?;
    Ok(format!("{prefix}{}", generate(entry, stack)?))
}

/// A number left in digits, with its case suffix after a hyphen: `5` and `Acc` give `5-öt`.
///
/// The suffix is not looked up from a table. `PND-NUMINF-011` prints one, but `PND-NUMINF-010`
/// states the rule it tabulates: the written suffix is what the last spoken word takes, minus that
/// word. So this inflects the last word and writes the suffix half of the seam, which is why
/// `1000-et` comes out right although `ezer` inflects as `ezret`, and `20-szal` although `húsz`
/// inflects as `hússzal`.
///
/// Rules: `PND-NUMINF-010`, `PND-NUMINF-011`
pub fn digits(n: u64, case: Case) -> Result<String, GenError> {
    let (_, last) = split_last(n);
    let entry = lexicon()
        .get_pos(last, Pos::Numeral)
        .ok_or(GenError::Unsupported("numeral missing from the lexicon"))?;
    written(n, entry, &[Morph::Case(case)])
}

/// A day of the month in digits, with its suffix after a hyphen: `1-jén`, `5-én`, `20-án`.
///
/// The spoken word is the day noun, which is the third person possessive of the ordinal:
/// `elseje`, `másodika`, `ötödike`. So this is `PND-NUMINF-010` again with a two-step stack, and
/// the seam is what makes `1-jén` come out of a stem that changes to `else` on the way.
///
/// Rules: `PND-DATE-005`, `PND-NUMINF-010`, `PND-ORD-002`
pub fn digits_day(n: u64, case: Case) -> Result<String, GenError> {
    let word = ordinal(n);
    // The lexicon's own entry where it has one: `első` alternates before the possessive and the
    // generated ordinal carries no such trait, which would give `elsője` for `elseje`.
    let entry = lexicon()
        .get(&word)
        .first()
        .or_else(|| ordinal_entry(&word))
        .ok_or(GenError::Unsupported("ordinal missing from the lexicon"))?;
    let stack = [
        Morph::Possessive {
            person: crate::features::Person::Third,
            number: crate::features::Number::Sing,
        },
        Morph::Case(case),
    ];
    written(n, entry, &stack)
}

/// `n` in digits with the written half of `entry`'s suffixes after a hyphen.
fn written(n: u64, entry: &Entry, stack: &[Morph]) -> Result<String, GenError> {
    let (form, seam) = generate_seam(entry, stack)?;
    let Some(at) = seam else {
        // Nothing was attached to the stem, so nothing is written: the nominative is zero-marked
        // and the digits stand alone with no hyphen.
        return Ok(n.to_string());
    };
    Ok(format!("{n}-{}", &form[at..]))
}

/// Whether `base` is a number in digits carrying `tail` as its written suffix.
///
/// Rules: `PND-NUMINF-010`
#[must_use]
pub fn licenses_number(base: &str, tail: &str) -> bool {
    // A group separator may be a space of some width, which a token boundary would have cut, or a
    // period in the older notation.
    let plain_digits: String = base
        .chars()
        .filter(|c| !c.is_whitespace() && !matches!(c, '\u{a0}' | '\u{202f}' | '.'))
        .collect();
    plain_digits
        .parse::<u64>()
        .is_ok_and(|n| licenses_digits(n, tail))
}

/// Whether `tail` is the written suffix of the number `n` under some reading of it.
///
/// Both readings of a suffixed number are offered, because the digits do not say which it is:
/// `5-én` is the fifth day of a month and `5-en` is five people.
///
/// Rules: `PND-NUMINF-010`, `PND-DATE-005`
#[must_use]
pub fn licenses_digits(n: u64, tail: &str) -> bool {
    use crate::derivation::DerId;
    let carries = |w: Result<String, GenError>| {
        w.ok()
            .and_then(|w| w.split_once('-').map(|(_, t)| t.to_owned()))
            .is_some_and(|t| t == tail)
    };
    if carries(digits_day(n, Case::Nom)) {
        return true;
    }
    if crate::acronym::ALL_CASES
        .iter()
        .any(|&c| carries(digits(n, c)) || carries(digits_day(n, c)))
    {
        return true;
    }
    let derived = [
        vec![Morph::Derive(DerId::Os)],
        vec![Morph::Derive(DerId::Os), Morph::Plural],
        vec![Morph::Derive(DerId::I)],
        vec![Morph::Derive(DerId::Nyi)],
    ];
    let inflected = crate::acronym::ALL_CASES
        .iter()
        .map(|&c| vec![Morph::Derive(DerId::Os), Morph::Case(c)]);
    derived
        .into_iter()
        .chain(inflected)
        .any(|stack| carries(digits_stack(n, &stack)))
}

/// A number in digits under any stack, its suffix after a hyphen.
fn digits_stack(n: u64, stack: &[Morph]) -> Result<String, GenError> {
    let (_, last) = split_last(n);
    let entry = lexicon()
        .get_pos(last, Pos::Numeral)
        .ok_or(GenError::Unsupported("numeral missing from the lexicon"))?;
    written(n, entry, stack)
}

/// Whether `word` is a Roman numeral in the standard subtractive form (`PUN-NUMF-010`).
///
/// `MMXXIV` and `IV` are, `IIII` and `VX` are not. The form is the rule's, not a table: at most
/// three of a repeating letter, and a smaller letter before a larger one only in the six
/// subtractive pairs.
#[must_use]
pub fn is_roman(word: &str) -> bool {
    let mut rest = word;
    let mut take = |options: &[&str]| {
        if let Some(o) = options.iter().find(|o| rest.starts_with(*o)) {
            rest = &rest[o.len()..];
        }
    };
    take(&["MMM", "MM", "M"]);
    take(&["CM", "CD", "DCCC", "DCC", "DC", "D", "CCC", "CC", "C"]);
    take(&["XC", "XL", "LXXX", "LXX", "LX", "L", "XXX", "XX", "X"]);
    take(&["IX", "IV", "VIII", "VII", "VI", "V", "III", "II", "I"]);
    !word.is_empty() && rest.is_empty()
}

/// What one spelled numeral token contributes.
#[derive(Clone, Copy)]
enum Piece {
    /// A unit or a ten, which adds: `öt`, `ötven`, and the bound `tizen`, `huszon`.
    Add(u64),
    /// `száz`, which multiplies what stands before it.
    Hundred,
    /// A scale word, which closes the group it multiplies: `ezer`, `millió`.
    Scale(u64),
}

/// The spelled tokens, built once.
fn pieces() -> &'static [(&'static str, Piece)] {
    static PIECES: std::sync::LazyLock<Vec<(&'static str, Piece)>> =
        std::sync::LazyLock::new(piece_table);
    &PIECES
}

/// The spelled tokens, longest first so `ötven` is not read as `öt`.
fn piece_table() -> Vec<(&'static str, Piece)> {
    let mut out: Vec<(&'static str, Piece)> = Vec::new();
    // From zero: `nulla` is a numeral standing alone, and it adds nothing inside a composition.
    for (i, t) in UNITS.iter().enumerate() {
        let v = i as u64;
        // Both shapes of `kettő`: `tizenkettő` uses the independent one and `kétezer` the
        // attributive, and which is right is settled by regenerating.
        out.push((t.indep, Piece::Add(v)));
        if t.attr != t.indep {
            out.push((t.attr, Piece::Add(v)));
        }
    }
    for (i, t) in TENS.iter().enumerate().skip(1) {
        let v = i as u64 * 10;
        out.push((t.indep, Piece::Add(v)));
    }
    out.push((*TIZEN, Piece::Add(10)));
    out.push((*HUSZON, Piece::Add(20)));
    out.push((SZAZ.indep, Piece::Hundred));
    for &(scale, t) in SCALES.iter() {
        out.push((t.indep, Piece::Scale(scale)));
    }
    out.sort_by_key(|(s, _)| std::cmp::Reverse(s.len()));
    out
}

/// A spelled-out cardinal read back to its value.
///
/// The tokeniser is deliberately liberal - longest match, no grammar - because the answer is then
/// verified by spelling the value out again. A wrong split cannot survive that, so this needs no
/// second copy of `PND-CARD`'s composition rules, only their inverse arithmetic. Both spellings
/// above 2000 verify: the hyphenated amount and the solid year (`AkH12` §291 a)), which a token
/// alone cannot tell apart.
///
/// Rules: `PND-CARD-001`, `PND-CARD-002`, `PND-CARD-005`, `PND-CARD-006`, `ORT-NUM-010`
#[must_use]
pub fn parse(word: &str) -> Option<u64> {
    let n = accumulate(word)?;
    let word = normalised(word);
    (normalised(&cardinal(n)) == word || normalised(&year(n)) == word).then_some(n)
}

/// A numeral spelling with the choices acceptance does not make.
///
/// `PND-CARD-005` makes the `egy` before `száz` and before a scale word silent, though the corpus
/// writes `ezeregyszáz` more often than `ezerszáz` and generation writes `egymillió`;
/// `PND-CARD-004` ends a numeral with `kettő` unless it stands attributively, where `tizenkét` is
/// as correct as `tizenkettő`.
///
/// Rules: `PND-CARD-004`, `PND-CARD-005`
fn normalised(word: &str) -> String {
    let mut out = word.replace(UNITS[2].indep, UNITS[2].attr);
    for bare in std::iter::once(SZAZ.indep).chain(SCALES.iter().map(|(_, t)| t.indep)) {
        out = out.replace(&format!("{}{bare}", UNITS[1].indep), bare);
    }
    out
}

/// The offsets where a leading run of spelled numeral tokens has a boundary, 0 excluded.
///
/// `PND-NUMINF-002` inflects a composed numeral on its last element only, so analysis has to try
/// each boundary as the seam between the inert prefix and the element that carries the suffix.
/// Every boundary is offered, not only the last: the run stops early when the final element is
/// inflected beyond recognition (`huszonhárman`), and it runs past the seam when the suffix
/// happens to spell a numeral too (`harmincötöt`, whose `öt` matches twice).
///
/// Empty when the word does not begin with a numeral token, which is the common case and costs
/// one table scan.
#[must_use]
pub fn token_bounds(word: &str) -> Vec<usize> {
    let table = pieces();
    let mut at = 0usize;
    let mut out = Vec::new();
    while at < word.len() {
        if word[at..].starts_with('-') {
            at += 1;
            continue;
        }
        let Some((text, _)) = table.iter().find(|(t, _)| word[at..].starts_with(t)) else {
            break;
        };
        if at > 0 {
            out.push(at);
        }
        at += text.len();
    }
    if at > 0 && at < word.len() {
        out.push(at);
    }
    out
}

/// Sums the tokens of `word`, or `None` if it is not wholly made of them.
fn accumulate(word: &str) -> Option<u64> {
    let table = pieces();
    let mut rest = word;
    let mut total: u64 = 0;
    let mut group: u64 = 0;
    while !rest.is_empty() {
        // PND-CARD-006 writes a hyphen between the groups above 2000.
        if let Some(r) = rest.strip_prefix('-') {
            rest = r;
            continue;
        }
        let (text, piece) = table.iter().find(|(t, _)| rest.starts_with(t))?;
        match piece {
            Piece::Add(v) => group = group.checked_add(*v)?,
            Piece::Hundred => group = group.max(1).checked_mul(100)?,
            Piece::Scale(m) => {
                total = total.checked_add(group.max(1).checked_mul(*m)?)?;
                group = 0;
            }
        }
        rest = &rest[text.len()..];
    }
    total.checked_add(group)
}

/// Spells `n` out, as a cardinal or as an ordinal.
///
/// Rules: `PND-CARD-003`, `PND-CARD-006`, `PND-ORD-001`
fn spell(n: u64, ordinal: bool) -> String {
    // PND-ORD-001: 1 and 2 are suppletive standing alone, but regular as the last element of a
    // compound, which is why this is a check on the whole number and not on the token.
    if ordinal {
        match n {
            1 => return "első".to_owned(),
            2 => return "második".to_owned(),
            _ => {}
        }
    }
    if n == 0 {
        return if ordinal { "nulladik" } else { "nulla" }.to_owned();
    }

    let mut chunks: Vec<String> = Vec::new();
    let mut rest = n;
    for &(scale, word) in SCALES.iter() {
        let g = rest / scale;
        if g == 0 {
            continue;
        }
        rest %= scale;
        let last = rest == 0;
        let mut chunk = String::new();
        // PND-CARD-005: `ezer` stands bare for the value 1, `millió` and above take `egy`.
        if g > 1 || scale >= 1_000_000 {
            chunk.push_str(&render(&group_tokens(g), false, false));
        }
        chunk.push_str(match (last, ordinal) {
            (true, true) => word.ord,
            _ => word.indep,
        });
        chunks.push(chunk);
    }
    if rest > 0 {
        chunks.push(render(&group_tokens(rest), true, ordinal));
    }

    // PND-CARD-006: one word up to 2000, hyphens between the groups above it.
    let sep = if n > HYPHEN_ABOVE { "-" } else { "" };
    chunks.join(sep)
}

/// The cardinal numeral spelling `n`: `2001` is `kétezer-egy`.
///
/// Rules: `PND-CARD-003`, `PND-CARD-006`
#[must_use]
pub fn cardinal(n: u64) -> String {
    spell(n, false)
}

/// The ordinal numeral spelling `n`: `4605` is `négyezer-hatszázötödik`.
///
/// Rules: `PND-ORD-001`
#[must_use]
pub fn ordinal(n: u64) -> String {
    spell(n, true)
}

/// Ordinal lemmas, as entries ready to inflect.
///
/// Generated from [`ordinal`] rather than listed: the ordinals are unbounded, so a checker that
/// stores them stores an arbitrary prefix of an infinite set. The range below covers what running
/// text uses, and the scales beyond it.
///
/// `PND-ORD-002` makes an ordinal inflect as an `-ik` adjective, which is a lowering stem
/// (`harmadikat`, `ötödiket`).
///
/// Rules: `PND-ORD-001`, `PND-ORD-002`
fn ordinal_entries() -> Vec<Entry> {
    let mut lemmas: Vec<String> = (0..=200).map(ordinal).collect();
    for n in [
        300u64, 400, 500, 600, 700, 800, 900, 1000, 2000, 10_000, 100_000, 1_000_000,
    ] {
        lemmas.push(ordinal(n));
    }
    lemmas.extend(words(Role::OrdinalExtra).iter().map(ToString::to_string));
    lemmas.sort();
    lemmas.dedup();
    lemmas.iter().map(|l| ordinal_shaped(l)).collect()
}

/// The iteratives, `-szOr` on the fraction stem: `másodszor`, `harmadszor`, `sokadszor`,
/// `hányadszor`; `először` is a lexicon entry.
///
/// Rules: `PND-ORD-005`
fn iterative_entries() -> Vec<Entry> {
    let mut lemmas: Vec<String> = fractions()
        .iter()
        .map(|e| {
            let h = crate::generate::harmony(&e.lemma, e);
            format!(
                "{}{}",
                e.lemma,
                picur_morph_ortho::pick3(h, "szor", "szer", "ször")
            )
        })
        .collect();
    lemmas.push("másodszor".to_owned());
    lemmas.sort();
    lemmas.dedup();
    lemmas
        .into_iter()
        .map(|l| Entry::new(l, Pos::Adverb))
        .collect()
}

/// The iterative adverbs, built once.
fn iteratives() -> &'static [Entry] {
    static ITERATIVES: std::sync::LazyLock<Vec<Entry>> =
        std::sync::LazyLock::new(iterative_entries);
    &ITERATIVES
}

/// The ordinal lemmas, built once.
pub fn ordinals() -> &'static [Entry] {
    static ORDINALS: std::sync::LazyLock<Vec<Entry>> = std::sync::LazyLock::new(ordinal_entries);
    &ORDINALS
}

/// Whether `word` is a fraction stem, an ordinal without its `-ik` (`harmad`, `huszonötöd`).
#[must_use]
pub fn is_fraction(word: &str) -> bool {
    exact(fractions(), word).is_some()
}

/// The ordinal entry whose lemma is exactly `word`, if there is one.
#[must_use]
pub fn ordinal_entry(word: &str) -> Option<&'static Entry> {
    exact(ordinals(), word)
}

/// The fraction denominators: each ordinal without its `-ik`, a noun with the `-a`/`-e`
/// possessive (`harmad`, `harmada`, `negyede`).
///
/// `PND-FRAC-001` names `ketted` and `hányad` beside the numeric ones; `fél` is a lexicon entry
/// and `egyed` is not a fraction.
///
/// Rules: `PND-FRAC-001`, `PND-FRAC-003`
fn fraction_entries() -> Vec<Entry> {
    let mut lemmas: Vec<String> = ordinals()
        .iter()
        .filter(|e| !lists(Role::SuppletiveOrdinal, &e.lemma))
        .filter_map(|e| e.lemma.strip_suffix("ik").map(str::to_owned))
        .collect();
    lemmas.extend(words(Role::FractionExtra).iter().map(ToString::to_string));
    lemmas.sort();
    lemmas.dedup();
    lemmas
        .into_iter()
        .map(|l| {
            let mut e = Entry::new(l, Pos::Noun);
            e.traits = TraitSet::from_iter_of([Trait::LinkedAccusative, Trait::PossessiveA]);
            e
        })
        .collect()
}

/// The fraction lemmas, built once.
pub fn fractions() -> &'static [Entry] {
    static FRACTIONS: std::sync::LazyLock<Vec<Entry>> = std::sync::LazyLock::new(fraction_entries);
    &FRACTIONS
}

/// The number `word` spells as an ordinal, if it is one.
///
/// `PND-ORD-001` puts the ordinal shape on the numeral's last element only, so an ordinal reads
/// back by putting that element's independent shape in its place and reading the cardinal:
/// `ezeregyedik` is `ezeregy` and `hatmilliomodik` is `hatmillió`. The value is confirmed by
/// respelling it, exactly as [`parse`] confirms a cardinal.
///
/// Rules: `PND-ORD-001`, `PND-NUMINF-002`
#[must_use]
pub fn parse_ordinal(word: &str) -> Option<u64> {
    let cells = UNITS
        .iter()
        .chain(TENS.iter())
        .chain(std::iter::once(&*SZAZ))
        .map(|t| (t.ord, t.indep))
        .chain(SCALES.iter().map(|(_, t)| (t.ord, t.indep)));
    for (ord, indep) in cells {
        if ord.is_empty() {
            continue;
        }
        let Some(rest) = word.strip_suffix(ord) else {
            continue;
        };
        let Some(n) = parse(&format!("{rest}{indep}")) else {
            continue;
        };
        // `PND-CARD-005`: the `egy` generation writes before a scale word is silent, and the
        // ordinal stem hides it from the cardinal normalisation (`milliomodik`, `egymilliomodik`).
        let spelled = ordinal(n);
        let bare = spelled.strip_prefix(UNITS[1].indep).unwrap_or(&spelled);
        if normalised(&spelled) == normalised(word) || normalised(bare) == normalised(word) {
            return Some(n);
        }
    }
    None
}

/// The computed numeral entry whose lemma is exactly `word`: an ordinal or a fraction.
///
/// The ordinals are unbounded, so one the sampled table does not list is read back from its
/// spelling and built on the spot (`PND-ORD-001`).
#[must_use]
pub fn numeral_entry(word: &str) -> Option<Entry> {
    if let Some(e) = ordinal_entry(word)
        .or_else(|| exact(fractions(), word))
        .or_else(|| exact(iteratives(), word))
    {
        return Some(e.clone());
    }
    parse_ordinal(word).map(|_| ordinal_shaped(word))
}

/// An ordinal word as an entry ready to inflect.
///
/// `PND-ORD-002` makes an ordinal inflect as an `-ik` adjective, which is a lowering stem with a
/// plain plural (`harmadikat`, `harmadikok`).
///
/// Rules: `PND-ORD-002`
fn ordinal_shaped(word: &str) -> Entry {
    let mut e = Entry::new(word, Pos::Adjective);
    e.traits = TraitSet::from_iter_of([Trait::Low, Trait::LowPluralExempt]);
    e
}

fn exact<'a>(sorted: &'a [Entry], word: &str) -> Option<&'a Entry> {
    let i = sorted.partition_point(|e| e.lemma.as_ref() < word);
    sorted.get(i).filter(|e| e.lemma.as_ref() == word)
}

/// A year, which is written as one word whatever its size.
///
/// `PND-CARD-006` makes this the one exception to the hyphen rule: `kétezertizennégy májusában`
/// is a year, `kétezer-tizennégy forint` a quantity.
///
/// Rules: `PND-CARD-006`
#[must_use]
pub fn year(n: u64) -> String {
    cardinal(n).replace('-', "")
}

#[cfg(test)]
mod tests {
    use super::{cardinal, digits, digits_day, inflect, ordinal, parse, split_last, year};
    use crate::features::{Case, Morph};

    #[test]
    fn units_and_teens() {
        // PND-CARD-001, PND-CARD-002.
        assert_eq!(cardinal(0), "nulla");
        assert_eq!(cardinal(1), "egy");
        assert_eq!(cardinal(2), "kettő");
        assert_eq!(cardinal(7), "hét");
        assert_eq!(cardinal(10), "tíz");
        assert_eq!(cardinal(11), "tizenegy");
        assert_eq!(cardinal(12), "tizenkettő");
        assert_eq!(cardinal(14), "tizennégy");
        assert_eq!(cardinal(19), "tizenkilenc");
        assert_eq!(cardinal(20), "húsz");
        assert_eq!(cardinal(21), "huszonegy");
        assert_eq!(cardinal(29), "huszonkilenc");
        assert_eq!(cardinal(31), "harmincegy");
        assert_eq!(cardinal(42), "negyvenkettő");
        assert_eq!(cardinal(99), "kilencvenkilenc");
    }

    #[test]
    fn hundreds_drop_a_leading_one() {
        // PND-CARD-005.
        assert_eq!(cardinal(100), "száz");
        assert_eq!(cardinal(101), "százegy");
        assert_eq!(cardinal(200), "kétszáz");
        assert_eq!(cardinal(896), "nyolcszázkilencvenhat");
    }

    #[test]
    fn composition_examples_from_the_rulebook() {
        // PND-CARD-003, every worked example in the rule.
        assert_eq!(cardinal(1772), "ezerhétszázhetvenkettő");
        assert_eq!(cardinal(2000), "kétezer");
        assert_eq!(cardinal(2001), "kétezer-egy");
        assert_eq!(cardinal(3016), "háromezer-tizenhat");
        assert_eq!(cardinal(47_563), "negyvenhétezer-ötszázhatvanhárom");
        assert_eq!(cardinal(53_000), "ötvenháromezer");
        assert_eq!(cardinal(1_200_000), "egymillió-kétszázezer");
        assert_eq!(
            cardinal(7_490_530),
            "hétmillió-négyszázkilencvenezer-ötszázharminc"
        );
        assert_eq!(cardinal(62_000_000), "hatvankétmillió");
        assert_eq!(cardinal(2_000_500), "kétmillió-ötszáz");
    }

    #[test]
    fn the_hyphen_rule_turns_on_above_2000() {
        // PND-CARD-006: 2000 itself is still one word.
        assert_eq!(cardinal(1999), "ezerkilencszázkilencvenkilenc");
        assert_eq!(cardinal(2000), "kétezer");
        assert!(!cardinal(2000).contains('-'));
        assert!(cardinal(2001).contains('-'));
        assert_eq!(cardinal(200_000), "kétszázezer");
        assert!(!cardinal(11_000).contains('-'));
        assert_eq!(cardinal(11_028), "tizenegyezer-huszonnyolc");
        assert_eq!(
            cardinal(3_540_238),
            "hárommillió-ötszáznegyvenezer-kétszázharmincnyolc"
        );
    }

    #[test]
    fn ket_versus_ketto() {
        // PND-CARD-004: `két` before a scale word, `kettő` when the number ends there.
        assert_eq!(cardinal(2), "kettő");
        assert_eq!(cardinal(12), "tizenkettő");
        assert_eq!(cardinal(22), "huszonkettő");
        assert_eq!(cardinal(200), "kétszáz");
        assert_eq!(cardinal(2000), "kétezer");
        assert_eq!(cardinal(2_000_000), "kétmillió");
        assert_eq!(cardinal(202_000), "kétszázkétezer");
        assert_eq!(cardinal(132), "százharminckettő");
    }

    #[test]
    fn egy_before_a_scale_word() {
        // PND-CARD-005: bare `száz` and `ezer`, but `egymillió`.
        assert_eq!(cardinal(1000), "ezer");
        assert_eq!(cardinal(100_000), "százezer");
        assert_eq!(cardinal(1_000_000), "egymillió");
        assert_eq!(cardinal(1_000_000_000), "egymilliárd");
    }

    #[test]
    fn long_scale() {
        // PND-CARD-001: `billió` is 10^12, not the English billion.
        assert_eq!(cardinal(1_000_000_000_000), "egybillió");
        assert_eq!(cardinal(2_000_000_000), "kétmilliárd");
    }

    #[test]
    fn ordinals_from_the_rulebook() {
        // PND-ORD-001, every example in the rule.
        assert_eq!(ordinal(1), "első");
        assert_eq!(ordinal(2), "második");
        assert_eq!(ordinal(3), "harmadik");
        assert_eq!(ordinal(5), "ötödik");
        assert_eq!(ordinal(10), "tizedik");
        assert_eq!(ordinal(11), "tizenegyedik");
        assert_eq!(ordinal(12), "tizenkettedik");
        assert_eq!(ordinal(22), "huszonkettedik");
        assert_eq!(ordinal(31), "harmincegyedik");
        assert_eq!(ordinal(101), "százegyedik");
        assert_eq!(ordinal(102), "százkettedik");
        assert_eq!(ordinal(200), "kétszázadik");
        assert_eq!(ordinal(1001), "ezeregyedik");
        assert_eq!(ordinal(2000), "kétezredik");
        assert_eq!(ordinal(4605), "négyezer-hatszázötödik");
        assert_eq!(ordinal(6_000_000), "hatmilliomodik");
        assert_eq!(ordinal(1000), "ezredik");
        assert_eq!(ordinal(100), "századik");
    }

    #[test]
    fn a_year_is_always_one_word() {
        // PND-CARD-006.
        assert_eq!(year(2014), "kétezertizennégy");
        assert_eq!(cardinal(2014), "kétezer-tizennégy");
        assert_eq!(year(1956), "ezerkilencszázötvenhat");
    }

    #[test]
    fn every_number_below_ten_thousand_spells_without_a_stray_separator() {
        for n in 0..10_000u64 {
            let c = cardinal(n);
            assert!(!c.is_empty(), "{n}");
            assert!(!c.starts_with('-') && !c.ends_with('-'), "{n} -> {c}");
            assert!(!c.contains("--"), "{n} -> {c}");
            let o = ordinal(n);
            assert!(
                o.ends_with("dik") || o == "első" || o == "második",
                "{n} -> {o}"
            );
        }
    }

    #[test]
    fn a_composed_numeral_splits_on_its_last_element() {
        // PND-NUMINF-002.
        assert_eq!(
            split_last(47_563),
            ("negyvenhétezer-ötszázhatvan".to_owned(), "három")
        );
        assert_eq!(split_last(2000), ("két".to_owned(), "ezer"));
        assert_eq!(split_last(200), ("két".to_owned(), "száz"));
        assert_eq!(split_last(200_000), ("kétszáz".to_owned(), "ezer"));
        assert_eq!(split_last(2_000_000), ("két".to_owned(), "millió"));
        assert_eq!(split_last(23), ("huszon".to_owned(), "három"));
        assert_eq!(split_last(7), (String::new(), "hét"));
    }

    #[test]
    fn inflection_lands_on_the_last_element() {
        // PND-NUMINF-001, PND-NUMINF-002: every one of these is the plain NOM machinery applied
        // to the final word, with the rest of the compound inert.
        let acc = &[Morph::Case(Case::Acc)];
        assert_eq!(inflect(3, acc).expect("3"), "hármat");
        assert_eq!(inflect(23, acc).expect("23"), "huszonhármat");
        assert_eq!(inflect(12, acc).expect("12"), "tizenkettőt");
        assert_eq!(inflect(200, acc).expect("200"), "kétszázat");
        assert_eq!(inflect(2000, acc).expect("2000"), "kétezret");
        assert_eq!(
            inflect(47_563, acc).expect("47563"),
            "negyvenhétezer-ötszázhatvanhármat"
        );
        for (n, want) in [
            (0, "nullát"),
            (1, "egyet"),
            (2, "kettőt"),
            (3, "hármat"),
            (4, "négyet"),
            (5, "ötöt"),
            (6, "hatot"),
            (7, "hetet"),
            (8, "nyolcat"),
            (9, "kilencet"),
            (10, "tízet"),
            (20, "húszat"),
            (30, "harmincat"),
            (40, "negyvenet"),
            (100, "százat"),
            (1000, "ezret"),
        ] {
            let got = inflect(n, acc).expect("generates");
            assert_eq!(got, want, "{n}");
        }
    }

    #[test]
    fn numerals_take_ordinary_cases() {
        assert_eq!(
            inflect(1956, &[Morph::Case(Case::Ine)]).expect("1956"),
            "ezerkilencszázötvenhatban"
        );
        assert_eq!(
            inflect(3, &[Morph::Case(Case::Ins)]).expect("3"),
            "hárommal"
        );
        assert_eq!(inflect(1, &[Morph::Case(Case::Ins)]).expect("1"), "eggyel");
        assert_eq!(
            inflect(1000, &[Morph::Case(Case::Ins)]).expect("1000"),
            "ezerrel"
        );
    }

    #[test]
    fn digit_suffixes_from_the_rulebook() {
        // PND-NUMINF-011, every example the rule prints.
        let d = |n, c| digits(n, c).expect("generates");
        for (n, want) in [
            (1, "1-et"),
            (2, "2-t"),
            (3, "3-at"),
            (5, "5-öt"),
            (6, "6-ot"),
            (7, "7-et"),
            (8, "8-at"),
            (20, "20-at"),
            (40, "40-et"),
            (100, "100-at"),
            (1000, "1000-et"),
        ] {
            assert_eq!(d(n, Case::Acc), want, "accusative of {n}");
        }
        for (n, want) in [
            (1, "1-gyel"),
            (4, "4-gyel"),
            (5, "5-tel"),
            (6, "6-tal"),
            (8, "8-cal"),
            (9, "9-cel"),
            (10, "10-zel"),
            (20, "20-szal"),
            (30, "30-cal"),
            (40, "40-nel"),
            (60, "60-nal"),
            (100, "100-zal"),
            (1000, "1000-rel"),
        ] {
            assert_eq!(d(n, Case::Ins), want, "instrumental of {n}");
        }
        assert_eq!(d(2, Case::All), "2-höz");
        assert_eq!(d(5, Case::All), "5-höz");
        assert_eq!(d(3, Case::All), "3-hoz");
        assert_eq!(d(7, Case::All), "7-hez");
        assert_eq!(d(2000, Case::Ine), "2000-ben");
        assert_eq!(d(1848, Case::Ine), "1848-ban");
        assert_eq!(d(1956, Case::Ine), "1956-ban");
        assert_eq!(d(2010, Case::Ine), "2010-ben");
        assert_eq!(d(2020, Case::Ine), "2020-ban");
        // A stem change is never written after the hyphen.
        assert_eq!(d(0, Case::Acc), "0-t");
        assert_eq!(d(0, Case::Ins), "0-val");
    }

    #[test]
    fn a_spelled_numeral_reads_back_to_its_value() {
        for (word, want) in [
            ("tizenöt", 15u64),
            ("tizenkettő", 12),
            // PND-CARD-004's attributive shape, as correct as the independent one.
            ("tizenkét", 12),
            ("huszonhárom", 23),
            ("harmincöt", 35),
            ("százhuszonöt", 125),
            ("egymillió", 1_000_000),
            ("kétezer-tizennégy", 2014),
            ("negyvenhétezer-ötszázhatvanhárom", 47563),
            ("ezerkilencszáznegyvennégy", 1944),
            // PND-CARD-005 writes the silent `egy`, and the corpus writes it either way.
            ("ezerszáz", 1100),
            ("ezeregyszáz", 1100),
        ] {
            assert_eq!(parse(word), Some(want), "{word}");
        }
        for word in ["ötlet", "százalék", "tizedes", "hatalom", "hetes", "ház"] {
            assert_eq!(parse(word), None, "{word}");
        }
        for n in [
            0u64, 1, 2, 7, 11, 20, 21, 99, 100, 101, 999, 1000, 2000, 123_456,
        ] {
            assert_eq!(parse(&cardinal(n)), Some(n), "{n}");
        }
    }

    #[test]
    fn days_of_the_month_take_the_ordinal_possessive() {
        // PND-DATE-005: the day noun is the third person possessive of the ordinal, so the
        // written tail is that word's suffix half. `1-jén` is the case the seam exists for:
        // `első` becomes `else` before the possessive and the change is not written.
        let d = |n, c| digits_day(n, c).expect("generates");
        for (n, want) in [
            (1, "1-jén"),
            (2, "2-án"),
            (3, "3-án"),
            (4, "4-én"),
            (5, "5-én"),
            (6, "6-án"),
            (20, "20-án"),
            (31, "31-én"),
        ] {
            assert_eq!(d(n, Case::Sup), want, "superessive of day {n}");
        }
        assert_eq!(d(1, Case::Abl), "1-jétől");
        assert_eq!(d(15, Case::Ter), "15-éig");
    }

    #[test]
    fn invariant_suffixes_need_no_harmony() {
        // PND-NUMINF-011: -kor, -ig and -ért are the same after every number.
        assert_eq!(digits(5, Case::Tem).expect("5"), "5-kor");
        assert_eq!(digits(10, Case::Ter).expect("10"), "10-ig");
        assert_eq!(digits(100, Case::Cau).expect("100"), "100-ért");
        assert_eq!(digits(3, Case::Formalis).expect("3"), "3-ként");
    }

    #[test]
    fn the_nominative_takes_no_hyphen() {
        assert_eq!(digits(42, Case::Nom).expect("42"), "42");
    }
}
