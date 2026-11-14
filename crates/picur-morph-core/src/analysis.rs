//! Analysis: search over generation, verified by regenerating and comparing.
//!
//! No morphological claim is made here. A candidate is accepted only when
//! [`generate`](fn@crate::generate) reproduces the input exactly, which is what makes analysis correct
//! by construction and keeps a bug fix in generation fixing both directions at once.
//!
//! See `docs/design/03-analysis.md`.

use std::sync::LazyLock;

use picur_morph_lexicon::{Entry, Pos, Role, Trait, TraitSet, lexicon, lists, words};
use picur_morph_ortho::case::{decapitalised, lower_initial, starts_upper, upper_initial};
use picur_morph_ortho::norm::skeleton;
use picur_morph_ortho::{Harmony, letters, pick2, pick3};

use crate::compound::Position;
use crate::features::{Case, Definite, Mood, Morph, Number, Person, Tense};
use crate::generate::{generate, generate_toward};

/// One accepted reading.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Analysis {
    /// The lemma this form belongs to.
    pub lemma: Box<str>,
    /// Its part of speech.
    pub pos: Pos,
    /// The feature stack that regenerates the surface form.
    pub stack: Vec<Morph>,
}

/// What analysis concluded about a surface form.
///
/// Three outcomes, not two. An affix-file spellchecker must answer "in the dictionary or not", so
/// an unlisted word and a misspelling are the same event to it. In Hungarian, where productive
/// suffixation means no dictionary is ever complete, separating them is the single biggest quality
/// difference available.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Verdict {
    /// Analysed. The form is correct and these are its readings.
    Known(Vec<Analysis>),
    /// The stem is not in the lexicon, but nothing about the form is ill formed.
    ///
    /// Neologisms and unlisted proper names land here. This is **not** an error, and it is what
    /// makes the "never add a word to silence a false rejection" discipline affordable.
    WellFormed,
    /// No legal parse.
    IllFormed,
}

/// The longest prefix of a lemma that no alternation can change.
///
/// `bokor` is epenthetic so its stable prefix is `bok`; `ló` is a v-stem so it is `l`. The prefix
/// must never end inside a digraph, or a seam gemination makes the lookup miss: truncating
/// `gyümölcs` to `gyümölc` stops it matching `gyümölccsel`.
///
/// Rules: `NOM-STEM-006`, `NOM-STEM-008`, `NOM-STEM-010`, `NOM-STEM-012`
#[must_use]
pub fn stable_prefix(entry: &Entry) -> &str {
    let alternating = entry.traits.has_any([
        Trait::Epenthetic,
        Trait::Shortening,
        Trait::VStem,
        Trait::OaStem,
        Trait::UStem,
    ]);
    let lemma: &str = &entry.lemma;
    // VRB-STEM-008: an epenthetic `-ik` verb regains its vowel inside the final cluster
    // (`ugrik`, `ugorhat`), so the key stops before the cluster's last consonant.
    if entry.pos == Pos::Verb && crate::verbal::full_stem(entry).is_some() {
        let stem = crate::verbal::present_stem(lemma);
        if let Some(last) = letters(stem).last() {
            return &lemma[..last.at];
        }
    }
    // VRB-IMP-004: a verb in `szt` or `st` rewrites that cluster in the imperative, so neither
    // letter is stable: `oszt` gives `ossz` and `fest` gives `fess`. The key has to stop before the
    // cluster or the index cannot reach the form from its own surface. `stem_guard` makes the same
    // cut for the same reason.
    if entry.pos == Pos::Verb && (lemma.ends_with("szt") || lemma.ends_with("st")) {
        let n = letters(lemma).count();
        return letters(lemma)
            .nth(n.saturating_sub(2))
            .map_or(lemma, |span| &lemma[..span.at]);
    }
    if alternating {
        // Cut at the earliest vowel any of these alternations touches. That is the last vowel for
        // one alternation, but two vowels change when the epenthetic drop and the shortening
        // compose: `lélek` gives `lelk-`, so the key has to stop before the `é` as well or no form
        // of the word can be reached from its own surface.
        let depth: usize =
            usize::from(entry.has(Trait::Epenthetic) && entry.has(Trait::Shortening));
        let vowels: Vec<usize> = letters(lemma)
            .filter(|s| s.letter.is_vowel())
            .map(|s| s.at)
            .collect();
        let back = vowels.len().saturating_sub(1).saturating_sub(depth);
        let cut = vowels.get(back).copied().unwrap_or(lemma.len());
        // NOM-STEM-005: a metathesis stem moves the `h` before its dropped vowel behind the final
        // consonant (`teher`, `terhek`), so the key stops before the `h` as well.
        if entry.has(Trait::Epenthetic)
            && let Some(before) = letters(lemma).take_while(|s| s.at < cut).last()
            && before.letter == picur_morph_ortho::Letter::H
        {
            return &lemma[..before.at];
        }
        return &lemma[..cut];
    }
    // A verb inflects on its present stem, so the `-ik` must come off before the key is cut or
    // the key reaches past the stem: `ívik` keyed on `ívi`, which no form of the verb begins with.
    // `stem_guard` cuts the same way.
    let base: &str = if entry.pos == Pos::Verb {
        crate::verbal::present_stem(lemma)
    } else {
        lemma
    };
    // A stem with no alternation can still be rewritten at the seam: `-vAl` turns `lány` into
    // `lánnyal`, so the final letter is not stable either. The key is therefore the first few
    // letters *excluding the last*, which is short enough to survive gemination and long enough
    // to keep the buckets small.
    // At least one letter, or a one-letter lemma keys on the empty string and is dropped from the
    // index altogether, the definite article `a` first of all.
    let n = letters(base).count();
    let keep = KEY_LETTERS.min(n.saturating_sub(1)).max(1);
    let cut = letters(base).nth(keep).map_or(base.len(), |s| s.at);
    &base[..cut]
}

/// The second key of a lemma that is one multi-character letter, if it is one.
///
/// Such a lemma keys on its whole letter, and a seam gemination rewrites that letter by repeating
/// its first character (`zs` gives `zzsel`), so the surface never begins with the key; its first
/// character is keyed as well.
fn geminable_key(key: &str) -> Option<&str> {
    let mut spans = letters(key);
    let first = spans.next()?;
    if spans.next().is_some() || first.at != 0 || first.text.chars().count() < 2 {
        return None;
    }
    let c = first.text.chars().next()?;
    Some(&key[..c.len_utf8()])
}

/// The same entry with the other third-person possessive allomorph, when the lexicon records
/// neither or marks both with [`Trait::PossessiveOptional`].
///
/// `NOM-POSS-006` is explicit that for consonant-final stems outside the categorical list the
/// choice between `-ja/-je` and `-a/-e` is lexical and has **no learnable default**, and that a
/// prediction is a ranking hint "rather than a generator". Analysis therefore accepts both: `kertje`
/// outnumbers `kerte` 34,693 to 2,065 `[WC2]`, and an entry carrying no value must not make one of
/// them an error.
///
/// Rules: `NOM-POSS-006`
fn possessive_variant(entry: &Entry) -> Option<Entry> {
    // Every noun the corpus settles carries one of the two, so the open case is the one
    // measured as taking both: `utóda` 23,241 beside `utódja` 38,421 `[WC2]`.
    if entry.has(Trait::PossessiveA) || entry.has(Trait::PossessiveJa) {
        if !entry.has(Trait::PossessiveOptional) {
            return None;
        }
        let mut e = entry.clone();
        e.traits = if entry.has(Trait::PossessiveJa) {
            e.traits
                .without(Trait::PossessiveJa)
                .with(Trait::PossessiveA)
        } else {
            e.traits
                .without(Trait::PossessiveA)
                .with(Trait::PossessiveJa)
        };
        return Some(e);
    }
    let mut e = entry.clone();
    e.traits = e.traits.with(Trait::PossessiveJa);
    Some(e)
}

/// The same entry with its stem alternation removed, where the corpus shows the alternation is
/// optional.
///
/// `Trait::AlternationOptional` marks the stems that inflect both ways. Generation has to pick one
/// and picks the alternant, so this is what makes the other form reachable: `magvak` comes from the
/// entry and `magok` from this variant.
///
/// Rules: `NOM-STEM-008`, `NOM-STEM-010`, `NOM-STEM-012`
fn free_stem_variant(entry: &Entry) -> Option<Entry> {
    if !entry.has(Trait::AlternationOptional) {
        return None;
    }
    let mut e = entry.clone();
    for t in [
        Trait::Epenthetic,
        Trait::Shortening,
        Trait::VStem,
        Trait::OaStem,
        Trait::UStem,
        Trait::PossessiveAlt,
        Trait::PossessedPluralAlt,
        // The possessed plural allomorph was measured on the alternant (`ajtajai`); the free
        // stem takes the shape's own (`ajtói`).
        Trait::PossessedPluralJa,
        Trait::PossessedPluralA,
        // A cell measured as taking both the low and the plain link: `NOM-LOW-001`'s marking is
        // per lexeme, and where the corpus shows both forms neither is an error.
        Trait::LowPluralExempt,
        Trait::LowAccusativeExempt,
    ] {
        e.traits = e.traits.without(t);
    }
    Some(e)
}

/// The same noun on its free stem before the third person possessive, where `PH-ZERO-002`
/// attests the unalternated form beside the alternant (`sarokja` beside `sarka`).
///
/// Rules: `PH-ZERO-002`, `NOM-POSS-009`
fn free_possessive_variant(entry: &Entry) -> Option<Entry> {
    if !entry.has(Trait::FreeStemPossessive) {
        return None;
    }
    let mut e = entry.clone();
    for t in [
        Trait::Epenthetic,
        Trait::Shortening,
        Trait::VStem,
        Trait::OaStem,
        Trait::UStem,
        Trait::PossessiveA,
    ] {
        e.traits = e.traits.without(t);
    }
    // The free stem takes `-jA` (`sarokja`, `retekje`), whatever the alternant takes.
    e.traits = e.traits.with(Trait::PossessiveJa);
    Some(e)
}

/// The same verb with the other choice in both present-syncope cells.
///
/// `VRB-STEM-007`'s cells are measured, and where the losing form is attested in its own right the
/// entry carries [`Trait::AlternationOptional`]: `képezi` outnumbers `képzi` 201,559 to 11,758 and
/// both are Hungarian `[WC2]`. Generation takes the winner, so this is what makes the other
/// reachable.
///
/// Rules: `VRB-STEM-007`
fn syncope_variants(entry: &Entry) -> Vec<Entry> {
    if entry.pos != Pos::Verb
        || !entry.has(Trait::AlternationOptional)
        || !entry.has(Trait::Epenthetic)
    {
        return Vec::new();
    }
    let bare = TraitSet::from_iter_of(entry.traits.iter().filter(|t| {
        !matches!(
            t,
            Trait::DefiniteFullStem
                | Trait::PresentFullStem
                | Trait::DefiniteSyncope
                | Trait::PresentSyncope
        )
    }));
    // Both directions, because the winner differs by cell and a stack only ever reaches one.
    let mut full = entry.clone();
    full.traits = bare
        .with(Trait::DefiniteFullStem)
        .with(Trait::PresentFullStem);
    let mut sync = entry.clone();
    sync.traits = bare
        .with(Trait::DefiniteSyncope)
        .with(Trait::PresentSyncope);
    vec![full, sync]
}

/// The same entry with the lowering exemptions added, where a cell is measured as taking both.
///
/// [`free_stem_variant`] removes them, which reaches the low form of a lexeme marked exempt. This
/// is the other direction: `öröm` is marked lowering and its accusative takes both, `örömet`
/// 138,399 beside `örömöt` 77,324 `[WC2]`, so the plain link has to be reachable too.
///
/// Rules: `NOM-LOW-001`
fn plain_link_variant(entry: &Entry) -> Option<Entry> {
    if !entry.has(Trait::AlternationOptional) || !entry.has(Trait::Low) {
        return None;
    }
    let mut e = entry.clone();
    e.traits = e
        .traits
        .with(Trait::LowPluralExempt)
        .with(Trait::LowAccusativeExempt)
        .with(Trait::LowPossessiveExempt);
    Some(e)
}

/// The same verb with the other past marker in both cells.
///
/// `VRB-PAST-001`'s two cells are measured, and where the losing form is attested in its own right
/// the entry carries [`Trait::AlternationOptional`]: `mondtam` 457,946 beside `mondottam` 10,973
/// `[WC2]`. Generation takes the winner, so this is what makes the other reachable.
///
/// Rules: `VRB-PAST-001`
fn past_variants(entry: &Entry) -> Vec<Entry> {
    if entry.pos != Pos::Verb || !entry.has(Trait::AlternationOptional) {
        return Vec::new();
    }
    let bare = TraitSet::from_iter_of(entry.traits.iter().filter(|t| {
        !matches!(
            t,
            Trait::PastFinalT | Trait::PastFinalVtt | Trait::PastNonfinalT | Trait::PastNonfinalVtt
        )
    }));
    let mut t = entry.clone();
    t.traits = bare.with(Trait::PastFinalT).with(Trait::PastNonfinalT);
    let mut ott = entry.clone();
    ott.traits = bare.with(Trait::PastFinalVtt).with(Trait::PastNonfinalVtt);
    vec![t, ott]
}

/// The entries to try besides `entry` itself, each with whether it only applies to a stack that
/// reaches the third person possessive.
///
/// Both variants exist because the lexicon records a single value where the language admits two,
/// so generation must choose and analysis must not.
/// The cells a variant entry stands in for.
#[derive(Clone, Copy)]
enum Scope {
    Every,
    Possessed,
    /// `VRB-STEM-012`: the archaic `alusz-` stem is a present indicative stem only; the
    /// imperative uses the `d` stem (`aludjanak`, `*alusszanak`).
    PresentIndicative,
    /// `VRB-STEM-012`: the short stem carries the potential and the converb alone
    /// (`alhat ~ aludhat`, `alva ~ aludva`), never the finite cells (`*alt`, `*alni`).
    PotentialOrConverb,
}

impl Scope {
    fn admits(self, stack: &[Morph]) -> bool {
        match self {
            Self::Every => true,
            Self::Possessed => has_possessive(stack),
            Self::PresentIndicative => stack.iter().all(|m| {
                matches!(
                    m,
                    Morph::Verbal {
                        mood: Mood::Ind,
                        tense: Tense::Pres,
                        ..
                    }
                )
            }),
            Self::PotentialOrConverb => {
                matches!(stack.first(), Some(Morph::Potential | Morph::Converb))
            }
        }
    }
}

fn variants(entry: &Entry) -> Vec<(Entry, Scope)> {
    let mut out = Vec::new();
    if let Some(v) = possessive_variant(entry) {
        out.push((v, Scope::Possessed));
    }
    if let Some(v) = free_possessive_variant(entry) {
        out.push((v, Scope::Possessed));
    }
    if let Some(v) = free_stem_variant(entry) {
        out.push((v, Scope::Every));
    }
    out.extend(
        syncope_variants(entry)
            .into_iter()
            .map(|v| (v, Scope::Every)),
    );
    out.extend(past_variants(entry).into_iter().map(|v| (v, Scope::Every)));
    if let Some(v) = linked_plural_variant(entry) {
        out.push((v, Scope::Every));
    }
    if let Some(v) = plain_link_variant(entry) {
        out.push((v, Scope::Every));
    }
    if let Some(v) = harmony_variant(entry) {
        out.push((v, Scope::Every));
    }
    if let Some(v) = link_variant(entry) {
        out.push((v, Scope::Every));
    }
    if let Some(v) = long_present_variant(entry) {
        out.push((v, Scope::PresentIndicative));
    }
    if let Some(v) = short_stem_variant(entry) {
        out.push((v, Scope::PotentialOrConverb));
    }
    if let Some(v) = long_v_stem_variant(entry) {
        out.push((v, Scope::Every));
    }
    if let Some(v) = low_possessive_variant(entry) {
        out.push((v, Scope::Every));
    }
    if let Some(v) = bound_cells_variant(entry) {
        out.push((v, Scope::Every));
    }
    if let Some(v) = linked_accusative_variant(entry) {
        out.push((v, Scope::Every));
    }
    // `DER-MODEL-009`: the harmonic suffix's alternant is chosen independently of the allomorph,
    // so a lexeme the norm suffixes both ways has both in every cell a variant stands in for:
    // `haverja`, `haverje`, `havera`, `havere`. Only a lexeme marked `HarmonyOptional` pays for
    // it, and `harmony_variant` is already the flip of the entry itself.
    if entry.has(Trait::HarmonyOptional) {
        let crossed: Vec<(Entry, Scope)> = out
            .iter()
            .filter_map(|(v, s)| harmony_variant(v).map(|h| (h, *s)))
            .collect();
        out.extend(crossed);
    }
    if let Some(v) = silent_h_variant(entry) {
        out.push((v, Scope::Every));
    }
    out
}

/// The same stem with its final `h` pronounced, so `-vAl`/`-vÁ` assimilates to it.
///
/// `NOM-STEM-004` admits both spellings for the stems whose final `h` the norm may write as
/// silent; generation writes the unassimilated one, so this is what makes the other reachable:
/// `dühhel` beside `dühvel`, `méhhé` beside `méhvé`.
///
/// Rules: `NOM-STEM-004`
fn silent_h_variant(entry: &Entry) -> Option<Entry> {
    if !entry.has(Trait::SilentH) {
        return None;
    }
    let mut e = entry.clone();
    e.traits = e.traits.without(Trait::SilentH);
    Some(e)
}

/// A stem measured free in the accusative or the plural, read on its bound stem there.
///
/// `NOM-ACC-001`, `NOM-STEM-012`: `borjút` is generated and `borjat` accepted, `borjúk` and
/// `borjak` likewise (`AkH12` §30).
///
/// `NOM-ACC-008` has the bound alternant take the linking vowel even where the final consonant
/// would permit a bare `-t`, so the bare accusative and its low exemption belong to the free stem
/// and come off with it: `bajuszt` is generated and `bajszot` accepted, `észt` and `eszet`.
///
/// Rules: `NOM-ACC-001`, `NOM-ACC-008`, `NOM-STEM-012`
fn bound_cells_variant(entry: &Entry) -> Option<Entry> {
    if !entry.has(Trait::AlternationOptional)
        || !entry
            .traits
            .has_any([Trait::FreeStemAccSup, Trait::FreeStemPlural])
    {
        return None;
    }
    let mut e = entry.clone();
    e.traits = e
        .traits
        .without(Trait::FreeStemAccSup)
        .without(Trait::FreeStemPlural)
        .without(Trait::BareAccusative)
        .without(Trait::LowAccusativeExempt);
    Some(e)
}

/// A geminate-final stem whose bare accusative is measured beside the linked one.
///
/// `NOM-ACC-006`: `brosst` is generated and `brossot` accepted.
///
/// Rules: `NOM-ACC-006`
fn linked_accusative_variant(entry: &Entry) -> Option<Entry> {
    if !entry.has(Trait::AlternationOptional) || !entry.has(Trait::BareAccusative) {
        return None;
    }
    let mut e = entry.clone();
    e.traits = e
        .traits
        .without(Trait::BareAccusative)
        .with(Trait::LinkedAccusative);
    Some(e)
}

/// An sz-d verb on its long present stem, the `d` stem with `sz` in place of the `d`.
///
/// `VRB-STEM-012`: `alszanak` is preferred and `alusznak` accepted; the same stem gives
/// `alusztok` and `aluszom`, and `feküsznek` beside `fekszenek`.
///
/// Rules: `VRB-STEM-012`, `VRB-LINK-004`
fn long_present_variant(entry: &Entry) -> Option<Entry> {
    if entry.pos != Pos::Verb || !entry.traits.has_any([Trait::SzDStem, Trait::SzDVStem]) {
        return None;
    }
    let d_stem = crate::verbal::alternant_stem(entry)?;
    let root = d_stem.strip_suffix('d')?;
    let mut e = entry.clone();
    e.lemma = format!("{root}szik").into();
    e.alt = None;
    e.traits = e.traits.without(Trait::SzDStem).without(Trait::SzDVStem);
    Some(e)
}

/// A v-stem read without its vowel shortening.
///
/// `PH-VSTEM-001` lists the stems that add `v` without changing the vowel, and the corpus shows
/// the split running by paradigm cell rather than by speaker: `fővök` 22 : `fövök` 9 but `fövő`
/// 909 : `fővő` 671 `[WC2]`, with no standard preference over the 2,118 v-alternant tokens. So the
/// shortening is exactly as optional as the alternation flag says, and the long alternant is read
/// beside the generated short one.
///
/// Rules: `PH-VSTEM-001`, `PH-VSTEM-002`
fn long_v_stem_variant(entry: &Entry) -> Option<Entry> {
    if !entry.has(Trait::VStem)
        || !entry.has(Trait::Shortening)
        || !entry.has(Trait::AlternationOptional)
    {
        return None;
    }
    let mut e = entry.clone();
    e.traits = e.traits.without(Trait::Shortening);
    Some(e)
}

/// An sz-d verb on its short stem, the `sz` stem without the `sz`.
///
/// `VRB-STEM-012`: the potential and the converb are built on it beside the `d` stem
/// (`alhat ~ aludhat`, `alva ~ aludva`, `fekhet`, `nyughat`). Generation gives the `d` stem
/// throughout, so this is what makes the short alternant reachable.
///
/// Rules: `VRB-STEM-012`
fn short_stem_variant(entry: &Entry) -> Option<Entry> {
    if entry.pos != Pos::Verb {
        return None;
    }
    let short = crate::verbal::short_stem(&entry.lemma)?;
    let mut e = entry.clone();
    e.alt = Some(short.into());
    e.traits = e.traits.without(Trait::SzDStem).without(Trait::SzDVStem);
    Some(e)
}

/// A noun measured low in the possessive alone, read with the low link there.
///
/// `NOM-LOW-005`: `farkom` and `farkam` are both attested and the plural and accusative are
/// plain (`farkok`, `farkot`).
///
/// Rules: `NOM-LOW-001`, `NOM-LOW-005`
fn low_possessive_variant(entry: &Entry) -> Option<Entry> {
    if entry.pos != Pos::Noun
        || !entry.has(Trait::AlternationOptional)
        || !entry.has(Trait::Epenthetic)
        || entry.has(Trait::Low)
    {
        return None;
    }
    let mut e = entry.clone();
    e.traits = e
        .traits
        .with(Trait::Low)
        .with(Trait::LowPluralExempt)
        .with(Trait::LowAccusativeExempt);
    Some(e)
}

/// The same stem in the other harmony, for a lexeme the norm suffixes both ways.
///
/// `PH-HARM-011`: `norvég` and `szlovén` take back and front suffixes alike; generation takes
/// the marked or computed class and this makes the other reachable.
///
/// Rules: `PH-HARM-011`
fn harmony_variant(entry: &Entry) -> Option<Entry> {
    if !entry.has(Trait::HarmonyOptional) {
        return None;
    }
    let mut e = entry.clone();
    let marked_front = entry.has(Trait::Front) || entry.has(Trait::FrontRounded);
    let front = marked_front
        || (!entry.has(Trait::Back)
            && crate::generate::harmony(&entry.lemma, entry) != Harmony::Back);
    e.traits = if front {
        e.traits
            .without(Trait::Front)
            .without(Trait::FrontRounded)
            .with(Trait::Back)
    } else {
        e.traits.without(Trait::Back).with(Trait::Front)
    };
    Some(e)
}

/// The same verb with the other linking-vowel choice, for a stem measured both ways.
///
/// `VRB-LINK-010`, `VRB-STEM-006`: `függenek` beside `függnek`, `állsz` beside `állasz`.
///
/// Rules: `VRB-LINK-010`, `VRB-STEM-006`
fn link_variant(entry: &Entry) -> Option<Entry> {
    if !entry.has(Trait::LinkOptional) {
        return None;
    }
    let mut e = entry.clone();
    e.traits = if entry.has(Trait::HeavyStem) {
        e.traits.without(Trait::HeavyStem)
    } else {
        e.traits.with(Trait::HeavyStem)
    };
    Some(e)
}

/// The same adjective with the optional plural link, for an `ó`/`ő`-final stem.
///
/// `NOM-PL-003` lists the plural of these as `-k` ~ `-ak/-ek`, both correct: `bántók` ~ `bántóak`,
/// `elérhetők` ~ `elérhetőek`. Generation takes the preferred bare `-k`, so this is what makes the
/// other reachable. Computed from the lemma's shape, not stored.
///
/// Rules: `NOM-PL-003`
fn linked_plural_variant(entry: &Entry) -> Option<Entry> {
    if entry.pos != Pos::Adjective
        || entry.has(Trait::LinkedPlural)
        || !entry.lemma.ends_with(['ó', 'ő'])
    {
        return None;
    }
    let mut e = entry.clone();
    e.traits = e.traits.with(Trait::LinkedPlural);
    Some(e)
}

/// The lemma to report for a reading of `entry`.
///
/// Almost always the entry's own lemma. The reflexive pronoun is the exception: it inflects on a
/// stem that is not a word, so `magam` belongs to `maga`.
fn lemma_of(entry: &Entry, stack: &[Morph]) -> Box<str> {
    if entry.pos == Pos::Pronoun
        && stack.iter().any(|m| matches!(m, Morph::Possessive { .. }))
        && let Some(citation) = crate::pronoun::reflexive_citation(&entry.lemma)
    {
        return citation.into();
    }
    entry.lemma.clone()
}

/// Whether a stack reaches the third person possessive, where the allomorph is open.
fn has_possessive(stack: &[Morph]) -> bool {
    stack.iter().any(|m| {
        matches!(
            m,
            Morph::Possessive {
                person: Person::Third,
                ..
            }
        )
    })
}

/// Whether any alternation can rewrite this stem's interior.
fn alternates(entry: &Entry) -> bool {
    entry.traits.has_any([
        Trait::Epenthetic,
        Trait::Shortening,
        Trait::VStem,
        Trait::OaStem,
        Trait::UStem,
    ])
}

/// The longest prefix of a lemma that must survive into any form it generates.
///
/// Distinct from [`stable_prefix`], which is capped short so the index keeps usable buckets. This
/// is the full guarantee and is used to reject candidates: the `ház` bucket holds 131 entries, of
/// which only `ház` itself shares enough of its spelling with `házban` to have produced it. Testing
/// it turns a bucket scan into a handful of generate calls.
fn stem_guard(entry: &Entry) -> &str {
    // A suppletive verb shares nothing with its own lemma: `eszik` generates `ettem`. It is reached
    // through the extra index keys instead, so it can offer no guarantee here.
    if crate::irregular::irregular(&entry.lemma).is_some() {
        return "";
    }
    if alternates(entry) {
        return stable_prefix(entry);
    }
    // An sz-d verb has two stems that share almost nothing: `alszik` inflects on `alud-`.
    if entry.alt.is_some() {
        return "";
    }
    // A verb inflects on its stem, not its lemma, so the `-ik` must come off first or the guard
    // rejects every form of the verb: `közeledik` generates `közeledünk`.
    let base: &str = if entry.pos == Pos::Verb {
        crate::verbal::present_stem(&entry.lemma)
    } else {
        &entry.lemma
    };
    // Usually only the final letter is rewritten at the seam, by assimilation or gemination. The
    // exception is `VRB-IMP-006`: an `st` or `szt` verb drops the `t` and doubles the sibilant, so
    // `választ` gives `válassz` and two letters change.
    let keep = if base.ends_with("szt") || base.ends_with("st") {
        2
    } else {
        1
    };
    let n = letters(base).count();
    letters(base)
        .nth(n.saturating_sub(keep))
        .map_or(base, |s| &base[..s.at])
}

/// Whether `entry` could plausibly have produced `surface`, by spelling alone.
///
/// The prefix guard answers this for almost every entry. The exception is the handful whose guard
/// is empty because they alternate in their own first letter (`ész` giving `eszet`, `út` giving
/// `utat`); for those the first letter still has to match once vowel length is set aside, which is
/// what keeps that bucket from being tried in full against every word in the text.
fn plausible(entry: &Entry, surface: &str) -> bool {
    // A suppletive verb's stems share nothing with its lemma, not even the first letter: `van`
    // gives `lennem`. Its stems are what the index keyed, so the surface is already vouched for.
    if crate::irregular::irregular(&entry.lemma).is_some() {
        return true;
    }
    let guard = stem_guard(entry);
    if !guard.is_empty() {
        if surface.starts_with(guard) {
            return true;
        }
        // `ORT-NAME-022` makes the adjective derived from a one-word name lowercase, so `szerbiai`
        // shares no prefix with `Szerbia` as written. Only the initial may differ, and the
        // regenerate check still decides: nothing regenerates `szerbia` from `Szerbia`.
        return guard.chars().next().is_some_and(char::is_uppercase)
            && surface.chars().next().is_some_and(char::is_lowercase)
            && lower_initial(surface).starts_with(&lower_initial(guard));
    }
    let (Some(a), Some(b)) = (
        letters(&entry.lemma).next().map(|s| s.letter),
        letters(surface).next().map(|s| s.letter),
    ) else {
        return true;
    };
    // A shortening or lengthening alternation changes the vowel's length, never its quality.
    a == b || a.short() == Some(b) || b.short() == Some(a)
}

/// How many leading letters key a non-alternating stem.
const KEY_LETTERS: usize = 3;

/// The index key for a stem that is not a lemma, by the same rule as [`stable_prefix`].
fn key_of(stem: &str) -> &str {
    let n = letters(stem).count();
    let keep = KEY_LETTERS.min(n.saturating_sub(1)).max(1);
    let cut = letters(stem).nth(keep).map_or(stem.len(), |s| s.at);
    &stem[..cut]
}

/// Stable prefixes, sorted, pointing into the lexicon's entry order.
struct Index {
    keys: Vec<(Box<str>, u32)>,
}

impl Index {
    fn build() -> Self {
        let lx = lexicon();
        let mut keys: Vec<(Box<str>, u32)> = Vec::with_capacity(lx.len() + 64);
        for (i, e) in lx.iter().enumerate() {
            let Ok(idx) = u32::try_from(i) else { continue };
            // The empty key is kept, not dropped. Seven stems alternate in their very first
            // letter, so nothing of them survives into every form: `ész` gives `eszet` and `út`
            // gives `utat`, and the empty key is their only way into the index.
            keys.push((stable_prefix(e).into(), idx));
            // ORT-NAME-022 makes the adjective derived from a one-word name lowercase, so
            // `szerbiai` has to reach the capitalised `Szerbia` whose key it does not share. The
            // bare lowercase name stays wrong regardless: nothing regenerates `szerbia` from
            // `Szerbia`, which is what makes this safe to key both ways.
            let key = stable_prefix(e);
            if key.chars().next().is_some_and(char::is_uppercase) {
                let lower: String = key
                    .chars()
                    .next()
                    .map(|c| c.to_lowercase().collect::<String>() + &key[c.len_utf8()..])
                    .unwrap_or_default();
                keys.push((lower.into(), idx));
            }
            if let Some(k) = geminable_key(key) {
                keys.push((k.into(), idx));
            }
            if let Some(irr) = crate::irregular::irregular(&e.lemma) {
                for stem in irr.stems() {
                    let k = key_of(stem);
                    if !k.is_empty() {
                        keys.push((k.into(), idx));
                    }
                }
            }
        }
        keys.sort();
        keys.dedup();
        Self { keys }
    }

    /// Entry indices whose stable prefix is exactly `p`.
    fn get(&self, p: &str) -> impl Iterator<Item = usize> {
        let start = self.keys.partition_point(|(k, _)| k.as_ref() < p);
        let end = self.keys.partition_point(|(k, _)| k.as_ref() <= p);
        self.keys[start..end].iter().map(|(_, i)| *i as usize)
    }
}

static INDEX: LazyLock<Index> = LazyLock::new(Index::build);

/// Entries by the skeleton of their lemma, for the accent-insensitive search.
static SKELETONS: LazyLock<std::collections::HashMap<Box<str>, Vec<u32>>> = LazyLock::new(|| {
    let mut out: std::collections::HashMap<Box<str>, Vec<u32>> = std::collections::HashMap::new();
    for (i, e) in lexicon().iter().enumerate() {
        if let Ok(i) = u32::try_from(i) {
            out.entry(skeleton(&e.lemma).into()).or_default().push(i);
        }
    }
    out
});

/// Whether some accepted form differs from `surface` only in the accents of its vowels.
///
/// A dropped or wrong accent is the commonest damage Hungarian text suffers (`REP-DIAC`), and a
/// word that is such a change away from an accepted word is that word misspelled, not a word
/// nobody listed: `hosszu`, `ugy`, `kivülről`, `útat`. The test is the analyser's own,
/// regenerate and compare, with the comparison made on skeletons: every prefix of the surface
/// names the lemmas that share its skeleton, and a stack whose ending the surface could carry up
/// to accents is generated on each. A coined compound is not a lemma, so `ónvezető` does not make
/// `önvezető` a misspelling.
#[must_use]
pub fn accent_confusable(surface: &str, stacks: &Stacks) -> bool {
    let lx = lexicon();
    let target = skeleton(surface);
    let mut cuts: Vec<usize> = letters(surface).map(|s| s.at).collect();
    cuts.push(surface.len());
    let matching: Vec<&Vec<Morph>> = stacks.matching_loose(surface).collect();
    for &cut in cuts.iter().rev().take(MAX_ACCENT_CUTS) {
        let Some(ids) = SKELETONS.get(skeleton(&surface[..cut]).as_str()) else {
            continue;
        };
        for &i in ids {
            let Some(entry) = lx.at(i as usize) else {
                continue;
            };
            if entry.lemma.len() > surface.len() + MAX_STEM_SHRINK {
                continue;
            }
            for stack in &matching {
                if let Ok(form) = generate(entry, stack)
                    && form != surface
                    && skeleton(&form) == target
                {
                    return true;
                }
            }
        }
    }
    false
}

/// Prefixes of the surface tried as a stem by [`accent_confusable`], longest first.
const MAX_ACCENT_CUTS: usize = 8;

/// Whether `surface` has any reading, without building the readings.
///
/// A checker asks this of every token and needs an answer in microseconds, so it differs from
/// [`analyse`] in three ways that matter at that scale: an exact lemma match short-circuits, the
/// search stops at the first success instead of collecting every reading, and entries that cannot
/// possibly produce a form this short are skipped before generation is attempted.
pub fn is_known(surface: &str, stacks: &Stacks) -> bool {
    known(surface, stacks, true)
}

/// Whether `surface` is a listed word or a form of one, coined compounds excluded.
///
/// What [`is_known`] accepts beyond this is a compound the rules allow but no entry lists, which
/// is a possible word rather than a vouched one; a text repair may land only on the latter.
#[must_use]
pub fn is_vouched(surface: &str, stacks: &Stacks) -> bool {
    known(surface, stacks, false)
}

/// [`is_known`], with the coined-compound path under `coined`.
fn known(surface: &str, stacks: &Stacks, coined: bool) -> bool {
    // The closed computed paradigms `analyse` answers before the lexicon: the personal pronouns
    // (`PND-PERS-003`), the possessive pronouns (`PND-REFL-005`), the demonstrative adverbs and
    // postpositions (`PND-DEM-001`, `PND-DEM-009`) and the personal postpositions
    // (`PND-POSTP-005`).
    if crate::pronoun::analyse(surface).is_some()
        || crate::pronoun::possessive_pronoun(surface).is_some()
        || crate::postposition::demonstrative(surface).is_some()
        || crate::pronoun::demonstrative_adverb(surface).is_some()
        || crate::postposition::analyse(surface).is_some()
        || crate::pronoun::emphatic(surface).is_some()
    {
        return true;
    }
    let lx = lexicon();
    if !lx.get(surface).is_empty() {
        return true;
    }
    if simplex_hit(surface, stacks) {
        return true;
    }
    if ordinal_hit(surface, stacks) {
        return true;
    }
    if superlative_hit(surface, stacks) {
        return true;
    }
    if !irregular_comparative_readings(surface, stacks).is_empty() {
        return true;
    }
    if lengthened_demonstrative(surface).is_some_and(|long| !lx.get(&long).is_empty()) {
        return true;
    }
    if !spelled_numeral_readings(surface, stacks).is_empty() {
        return true;
    }
    if !coined {
        return false;
    }
    crate::compound::member_prefixes(surface)
        .into_iter()
        .filter(|(members, _)| !members.is_empty())
        .any(|(members, at)| {
            let after_preverb = crate::compound::preverb_member(&members).is_some();
            let head = &surface[at..];
            lx.get(head).iter().any(|e| {
                ((crate::compound::head_allows(e.pos, after_preverb)
                    || crate::compound::reflexive_head(e.pos, &members))
                    || (after_preverb && crate::compound::participle_shaped(&e.lemma)))
                    && crate::compound::numeral_head_allows(e.pos, &members)
                    && crate::compound::admissible_head(&e.lemma, &members)
            }) || simplex_hit_headed(head, stacks, Position::Head(&members))
        })
}

/// The readings of a surface on a computed numeral, an ordinal or a fraction.
fn ordinal_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    let mut cuts: Vec<usize> = letters(surface).map(|s| s.at).collect();
    cuts.push(surface.len());
    let mut out = Vec::new();
    for &cut in cuts.iter().rev() {
        let Some(entry) = crate::numeral::numeral_entry(&surface[..cut]) else {
            continue;
        };
        for stack in stacks.matching(surface) {
            if generate(&entry, stack).as_deref() == Ok(surface) {
                out.push(Analysis {
                    lemma: entry.lemma.clone(),
                    pos: entry.pos,
                    stack: stack.clone(),
                });
            }
        }
    }
    out
}

/// The readings of a surface built on the name of a letter.
///
/// `PND-ART-005` states the names as a closed table, and a name is a noun that inflects
/// (`az ipszilont`, `két em`). They are not lexicon entries because the inventory is the
/// definition, and several would shadow a listed word (`es`, `el`, `em`).
///
/// A name that is a listed word with its accents stripped (`es` for `és`, `el` for `él`, `en`
/// for `én`, `té` for `te`) is not read: in running text that spelling is the damage
/// `REP-DIAC-001` detects, and the restoration must stay free to repair it.
///
/// Rules: `PND-ART-005`, `REP-DIAC-001`
fn letter_name_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    let mut cuts: Vec<usize> = letters(surface).map(|s| s.at).collect();
    cuts.push(surface.len());
    let mut out = Vec::new();
    for &cut in cuts.iter().rev() {
        if !lists(Role::LetterName, &surface[..cut]) || accent_confusable(&surface[..cut], stacks) {
            continue;
        }
        let entry = Entry::new(&surface[..cut], Pos::Noun);
        for stack in stacks.matching(surface) {
            if generate(&entry, stack).as_deref() == Ok(surface) {
                out.push(Analysis {
                    lemma: entry.lemma.clone(),
                    pos: entry.pos,
                    stack: stack.clone(),
                });
            }
        }
    }
    out
}

/// The reading of a surface built on an irregular comparative.
///
/// `szép` gives `szebb`, so no prefix of `szebbek` resembles the lemma and the stem index cannot
/// reach it. The reverse table can, and the reading is then verified by regenerating as usual.
///
/// Rules: `NOM-ADJ-005`, `NOM-ADJ-007`
fn irregular_comparative_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    let lx = lexicon();
    let mut out: Vec<Analysis> = Vec::new();
    let mut cuts: Vec<usize> = letters(surface).map(|s| s.at).collect();
    cuts.push(surface.len());
    // Every reading, not the first: `szebben` is both the inessive of `szebb` and the manner
    // adverb of `szép`'s comparative.
    for &cut in cuts.iter().rev() {
        for base in crate::generate::comparative_bases(&surface[..cut]) {
            for entry in lx.get(base) {
                for stack in stacks.matching(surface) {
                    if generate(entry, stack).as_deref() == Ok(surface) {
                        let a = Analysis {
                            lemma: entry.lemma.clone(),
                            pos: entry.pos,
                            stack: stack.clone(),
                        };
                        if !out.contains(&a) {
                            out.push(a);
                        }
                    }
                }
            }
        }
    }
    out
}

/// The superlative prefixes, longest first.
///
/// `NOM-ADJ-006` writes the superlative as `leg-` on the comparative, so the surface does not begin
/// with the stem at all and the stem index cannot reach it. Stripping the prefix and requiring the
/// remainder to be a comparative is what makes `legnagyobb` analysable.
const SUPERLATIVE_PREFIXES: [(&str, Morph); 2] =
    [("legesleg", Morph::Excessive), ("leg", Morph::Superlative)];

/// Whether `surface` is a superlative of something the lexicon has.
///
/// Rules: `NOM-ADJ-006`
fn superlative_hit(surface: &str, stacks: &Stacks) -> bool {
    !superlative_readings(surface, stacks).is_empty()
}

/// Every superlative reading of `surface`.
///
/// Every reading, not the first: `legjobban` is the manner adverb and the superessive alike.
///
/// Rules: `NOM-ADJ-006`
fn superlative_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    let mut out: Vec<Analysis> = Vec::new();
    let mut add = |a: Analysis| {
        if !out.contains(&a) {
            out.push(a);
        }
    };
    for (prefix, morph) in SUPERLATIVE_PREFIXES {
        if surface.strip_prefix(prefix).is_none() {
            continue;
        }
        let rest_of = surface.strip_prefix(prefix).unwrap_or_default();
        // DER-PREF-004: on the closed list the prefix goes straight on the base, with no
        // comparative in between, so `legelső` is reached from `első` and a bare stack.
        if lists(Role::NonComparativeSuperlative, rest_of) {
            let lx = lexicon();
            for entry in lx.get(rest_of) {
                if generate(entry, &[morph]).as_deref() == Ok(surface) {
                    add(Analysis {
                        lemma: entry.lemma.clone(),
                        pos: entry.pos,
                        stack: vec![morph],
                    });
                }
            }
        }
        // `DER-PREF-004` says the prefix attaches to the comparative, so the check is that the
        // prefix plus the regenerated comparative is the surface. Regenerating the superlative
        // instead misses a base that has one of its own: `felső` is on the rule's closed list, so
        // `[Superlative]` gives the real but different `legfelső`, and `legfelsőbb` is `leg` plus
        // the comparative `felsőbb`.
        for a in superlative_of_comparative(surface, prefix, rest_of, morph, stacks) {
            add(a);
        }
        // The prefix is only a superlative if what follows is a comparative, so the search is for
        // that reading rather than for any reading at all: `legenda` must not become a superlative.
        let graded: Stacks = stacks
            .iter()
            .filter(|s| s.first() == Some(&Morph::Comparative))
            .map(|s| {
                let mut v = s.clone();
                v[0] = morph;
                v
            })
            .collect::<Vec<_>>()
            .into();
        if graded.is_empty() {
            continue;
        }
        let lx = lexicon();
        let index = &*INDEX;
        let rest = rest_of;
        // An irregular comparative shares nothing with its lemma (`szép` gives `szebb`), so those
        // are reached through the reverse table rather than the index.
        let mut cuts: Vec<usize> = letters(rest).map(|s| s.at).collect();
        cuts.push(rest.len());
        // Over prefixes, not the whole remainder only: `legszebben` carries a suffix after the
        // comparative, so the reverse table has to be asked about `szebb` rather than `szebben`.
        for &cut in cuts.iter().rev() {
            for base in crate::generate::comparative_bases(&rest[..cut]) {
                for entry in lx.get(base) {
                    for stack in graded.matching(surface) {
                        if forms(entry, stack).iter().any(|f| f == surface) {
                            add(Analysis {
                                lemma: entry.lemma.clone(),
                                pos: entry.pos,
                                stack: stack.clone(),
                            });
                        }
                    }
                }
            }
        }
        for &cut in cuts.iter().rev() {
            for i in index.get(&rest[..cut]) {
                let Some(entry) = lx.at(i) else { continue };
                for stack in graded.matching(surface) {
                    if forms(entry, stack).iter().any(|f| f == surface) {
                        add(Analysis {
                            lemma: entry.lemma.clone(),
                            pos: entry.pos,
                            stack: stack.clone(),
                        });
                    }
                }
            }
        }
    }
    out
}

/// The readings of `surface` as `prefix` plus a comparative of something.
///
/// `DER-PREF-004` attaches `leg-` to the comparative, so this regenerates the comparative and
/// checks that the prefix plus it is the surface. It reaches a base whose own superlative is
/// listed: `felső` gives `legfelső` for `[Superlative]`, while `legfelsőbb` is `leg` plus
/// `felsőbb`.
///
/// Rules: `DER-PREF-004`, `NOM-ADJ-006`
fn superlative_of_comparative(
    surface: &str,
    prefix: &str,
    rest: &str,
    morph: Morph,
    stacks: &Stacks,
) -> Vec<Analysis> {
    let mut out = Vec::new();
    let Verdict::Known(readings) = analyse_with(rest, stacks) else {
        return out;
    };
    if format!("{prefix}{rest}") != surface {
        return out;
    }
    for r in readings {
        if r.stack.first() != Some(&Morph::Comparative) {
            continue;
        }
        let lx = lexicon();
        // Every form of the cell, not generation's one answer, so a variant spelling of the
        // comparative is read under `leg-` as it is without it.
        for entry in lx.get(&r.lemma) {
            if forms(entry, &r.stack).iter().any(|f| f == rest) {
                let mut stack = r.stack.clone();
                stack[0] = morph;
                let a = Analysis {
                    lemma: entry.lemma.clone(),
                    pos: entry.pos,
                    stack,
                };
                if !out.contains(&a) {
                    out.push(a);
                }
            }
        }
    }
    out
}

/// Whether an ordinal lemma plus a suffix explains the surface.
///
/// The ordinals are generated rather than stored, so they are not in the lexicon and are searched
/// separately. `PND-NUMINF-002` puts the suffix on the last element, which for an ordinal is the
/// whole word.
///
/// Rules: `PND-ORD-001`, `PND-ORD-002`, `PND-FRAC-001`, `PND-FRAC-003`
fn ordinal_hit(surface: &str, stacks: &Stacks) -> bool {
    let mut cuts: Vec<usize> = letters(surface).map(|s| s.at).collect();
    cuts.push(surface.len());
    cuts.iter().rev().any(|&cut| {
        crate::numeral::numeral_entry(&surface[..cut]).is_some_and(|e| {
            stacks
                .iter()
                .any(|s| generate(&e, s).as_deref() == Ok(surface))
        })
    })
}

/// First-match search over the stem index, with no allocation.
fn simplex_hit(surface: &str, stacks: &Stacks) -> bool {
    simplex_hit_headed(surface, stacks, Position::Whole)
}

/// As [`simplex_hit`], but when the surface is a compound's head the entry must be admissible as
/// one: the right category, and long enough to be a member at all.
fn simplex_hit_headed(surface: &str, stacks: &Stacks, at: Position<'_>) -> bool {
    let lx = lexicon();
    let index = &*INDEX;
    let mut cuts: Vec<usize> = letters(surface).map(|s| s.at).collect();
    cuts.push(surface.len());
    for &cut in cuts.iter().rev() {
        let prefix = &surface[..cut];
        for i in index.get(prefix) {
            let Some(entry) = lx.at(i) else { continue };
            // A suffix only ever adds material, so a lemma longer than the surface cannot reach it.
            // Alternation can shorten a stem, but by at most one vowel.
            if entry.lemma.len() > surface.len() + MAX_STEM_SHRINK || !plausible(entry, surface) {
                continue;
            }
            if let Position::Head(members) = at
                && ((!crate::compound::head_allows(
                    entry.pos,
                    crate::compound::preverb_member(members).is_some(),
                ) && !crate::compound::reflexive_head(entry.pos, members))
                    || !crate::compound::numeral_head_allows(entry.pos, members)
                    || !crate::compound::admissible_head(&entry.lemma, members))
            {
                continue;
            }
            let alts = variants(entry);
            for stack in stacks.matching(surface) {
                if generate(entry, stack).as_deref() == Ok(surface) {
                    return true;
                }
                if alts.iter().any(|(v, scope)| {
                    scope.admits(stack) && generate(v, stack).as_deref() == Ok(surface)
                }) {
                    return true;
                }
            }
        }
    }
    false
}

/// Bytes a stem alternation may remove from a lemma.
const MAX_STEM_SHRINK: usize = 2;

/// The feature stacks an analysis searches, indexed by the ending each one can write.
///
/// Regenerating every stack against every candidate stem is the checker's whole cost, and most
/// stacks cannot end the token at hand: a stack ending in `-bAn` is irrelevant to a token ending
/// in `t`. The last segment of a suffix is the last segment of the word, since a linking vowel
/// goes in front of it, lengthening applies to the stem and both assimilations act at the seam.
/// That segment realises as one of at most three strings, so the stacks are grouped by those
/// strings once and a token asks only for its own ending. A stack whose last morph fixes no
/// ending (a bare stem, a derivation, a verb cell) is open and is tried on every token, because a
/// wrong rejection here would be a wrong verdict.
#[derive(Clone, Debug)]
pub struct Stacks {
    all: Vec<Vec<Morph>>,
    open: Vec<usize>,
    by_ending: std::collections::HashMap<Box<str>, Vec<usize>>,
    lengths: Vec<usize>,
    suffixes: std::collections::HashSet<Box<str>>,
}

impl Stacks {
    /// Indexes `all` by the ending each stack can write.
    #[must_use]
    pub fn new(all: Vec<Vec<Morph>>) -> Self {
        let mut open = Vec::new();
        let mut by_ending: std::collections::HashMap<Box<str>, Vec<usize>> =
            std::collections::HashMap::new();
        let mut suffixes = std::collections::HashSet::new();
        for (i, stack) in all.iter().enumerate() {
            match endings(stack) {
                None => open.push(i),
                Some(keys) => {
                    for key in keys {
                        by_ending.entry(key.into()).or_default().push(i);
                    }
                }
            }
            for suffix in final_suffixes(stack).unwrap_or_default() {
                for h in [Harmony::Back, Harmony::Front, Harmony::FrontRounded] {
                    suffixes.insert(suffix.realise(h).into_boxed_str());
                    suffixes.insert(
                        suffix
                            .realise_with_link(h, None)
                            .into_string()
                            .into_boxed_str(),
                    );
                }
            }
        }
        let mut lengths: Vec<usize> = by_ending.keys().map(|k| k.len()).collect();
        lengths.sort_unstable();
        lengths.dedup();
        Self {
            all,
            open,
            by_ending,
            lengths,
            suffixes,
        }
    }

    /// The stacks whose ending `surface` can carry up to the accents of the ending, the open
    /// ones included.
    pub fn matching_loose<'a>(
        &'a self,
        surface: &'a str,
    ) -> impl Iterator<Item = &'a Vec<Morph>> + 'a {
        let target = skeleton(surface);
        let keyed = self.lengths.iter().flat_map(move |&n| {
            let target = target.clone();
            self.by_ending
                .iter()
                .filter(move |(key, _)| {
                    key.len() == n
                        && target
                            .len()
                            .checked_sub(skeleton(key).len())
                            .is_some_and(|at| target[at..] == skeleton(key))
                })
                .flat_map(|(_, ids)| ids)
        });
        self.open.iter().chain(keyed).map(move |&i| &self.all[i])
    }

    /// Every stack in the space, in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &Vec<Morph>> {
        self.all.iter()
    }

    /// Whether `tail` is the written form of some stack's last suffix under some harmony.
    #[must_use]
    pub fn writes_suffix(&self, tail: &str) -> bool {
        self.suffixes.contains(tail)
    }

    /// The stacks whose ending `surface` can carry, the open ones included.
    pub fn matching<'a>(&'a self, surface: &'a str) -> impl Iterator<Item = &'a Vec<Morph>> + 'a {
        let keyed = self.lengths.iter().flat_map(move |&n| {
            surface
                .len()
                .checked_sub(n)
                .filter(|&at| surface.is_char_boundary(at))
                .and_then(|at| self.by_ending.get(&surface[at..]))
                .into_iter()
                .flatten()
        });
        self.open.iter().chain(keyed).map(move |&i| &self.all[i])
    }
}

impl std::ops::Deref for Stacks {
    type Target = [Vec<Morph>];

    fn deref(&self) -> &Self::Target {
        &self.all
    }
}

impl From<Vec<Vec<Morph>>> for Stacks {
    fn from(all: Vec<Vec<Morph>>) -> Self {
        Self::new(all)
    }
}

/// The strings the last suffix of `stack` can end a form with, or `None` when it fixes none.
///
/// A suffix whose shape depends on the stem is asked on probe stems of each shape it distinguishes
/// (a vowel-final, a sibilant-final and a plain consonant-final one), and the endings are the
/// union. What is left open is only a cell with no suffix at all: the bare present third singular
/// and the bare imperative second singular.
fn endings(stack: &[Morph]) -> Option<Vec<&'static str>> {
    // NOM-PPL-001: the third singular after the possessed plural is zero, so the surface ends the
    // way the possessed plural does (`tagjai`), and that is the ending to key.
    if let [
        ..,
        Morph::PossessedPlural,
        Morph::Possessive {
            person: Person::Third,
            number: Number::Sing,
        },
    ] = stack
    {
        return endings(&stack[..stack.len() - 1]);
    }
    let suffixes = final_suffixes(stack)?;
    let mut out: Vec<&'static str> = Vec::new();
    for suffix in suffixes {
        match suffix.template.last()? {
            crate::suffix::Seg::Lit(text) => out.push(text),
            crate::suffix::Seg::Var(a) => {
                out.extend(
                    [Harmony::Back, Harmony::Front, Harmony::FrontRounded]
                        .into_iter()
                        .map(|h| a.realise(h)),
                );
            }
            _ => return None,
        }
    }
    // `VRB-IRR-001`: a suppletive verb may write a present cell whole, so the surface ends in the
    // listed form and not in any suffix (`vagy`). Keyed on the form itself.
    if let [
        Morph::Verbal {
            mood: Mood::Ind,
            tense: Tense::Pres,
            person,
            number,
            definite: Definite::Ind,
        },
    ] = *stack
    {
        let cell = Person::ALL.iter().position(|p| *p == person).unwrap_or(0)
            + usize::from(number == Number::Plur) * 3;
        out.extend(crate::irregular::present_forms(cell));
    }
    out.sort_unstable();
    out.dedup();
    Some(out)
}

/// Every suffix the last morph of `stack` can be written with, or `None` when it fixes none.
fn final_suffixes(stack: &[Morph]) -> Option<Vec<crate::suffix::Suffix>> {
    use crate::verbal;
    let nouns = [Entry::new("alma", Pos::Noun), Entry::new("ház", Pos::Noun)];
    // A sibilant, a plain consonant, a vowel and an `-ik` stem: between them every shape a
    // person suffix is chosen by.
    let verbs = [
        Entry::new("olvas", Pos::Verb),
        Entry::new("tanul", Pos::Verb),
        Entry::new("lő", Pos::Verb),
        Entry::new("alszik", Pos::Verb),
    ];
    Some(match *stack.last()? {
        Morph::Case(c) => vec![crate::suffix::case_suffix(c)?],
        // `NOM-ANP-002`: the anaphoric `-é` pluralises with `-i`, so a stack ending that way is
        // keyed on `-i` and not on the plural `-k`.
        Morph::Plural if stack.len() >= 2 && stack[stack.len() - 2] == Morph::Anaphoric => {
            vec![crate::generate::anaphoric_plural()]
        }
        Morph::Plural => vec![crate::generate::plural()],
        // `leg-` is a prefix, so the superlative and the excessive end as the comparative does.
        Morph::Comparative | Morph::Superlative | Morph::Excessive => {
            vec![crate::generate::comparative()]
        }
        Morph::Associative => vec![crate::generate::associative()],
        Morph::Anaphoric => vec![crate::generate::anaphoric()],
        Morph::Collective => vec![crate::generate::collective()],
        Morph::Multiplicative => vec![crate::generate::multiplicative()],
        Morph::Derive(id) => vec![id.record().suffix()],
        Morph::PossessedPlural => nouns
            .iter()
            .map(|e| crate::possessive::possessed_plural(&e.lemma, e))
            .collect(),
        Morph::Possessive { person, number } => nouns
            .iter()
            .map(|e| crate::possessive::possessive(person, number, &e.lemma, e))
            .collect(),
        Morph::Infinitive => verbs
            .iter()
            .map(|e| verbal::infinitive(&e.lemma, e))
            .collect(),
        Morph::Potential => vec![verbal::potential()],
        Morph::PresentParticiple => vec![verbal::present_participle()],
        Morph::PastParticiple => verbs
            .iter()
            .map(|e| verbal::past_participle(&e.lemma, e))
            .collect(),
        Morph::FutureParticiple => vec![verbal::future_participle()],
        // VRB-ADVP-002: the formal `-vÁn` is read beside `-vA`, so its ending is keyed too.
        Morph::Converb => vec![verbal::converb(), verbal::FORMAL_CONVERB],
        Morph::Verbal {
            mood,
            tense,
            person,
            number,
            definite,
        } => match (mood, tense, definite) {
            (Mood::Ind, Tense::Pres, Definite::Two) => verbs
                .iter()
                .map(|e| verbal::present_lak(&e.lemma, e))
                .collect(),
            (Mood::Ind, Tense::Pres, Definite::Ind) => verbs
                .iter()
                .map(|e| verbal::present_indefinite(person, number, &e.lemma, e))
                .collect::<Option<Vec<_>>>()?,
            (Mood::Ind, Tense::Pres, Definite::Def) => [true, false]
                .into_iter()
                .map(|back| verbal::present_definite(person, number, back))
                .collect(),
            (Mood::Ind, Tense::Past, _) => match verbal::past_person(person, number, definite) {
                Some(s) => vec![s],
                None => verbs
                    .iter()
                    .map(|e| verbal::past_final(&e.lemma, e))
                    .collect(),
            },
            // The past conditional is written apart, so no token ends the way it does; keying it
            // on its own word keeps it out of every lookup.
            (Mood::Cnd, Tense::Past, _) => vec![crate::suffix::VOLNA],
            (Mood::Cnd, Tense::Pres, _) => {
                vec![verbal::conditional_person(person, number, definite)]
            }
            // VRB-IK-005: the formal `-jék` and `-jAm` are read beside the neutral `-jOn` and
            // `-jAk`, so their endings are keyed too; the first singular's is the definite's.
            (Mood::Imp, _, _) => {
                let mut out = vec![verbal::imperative_person(person, number, definite)?];
                match (person, number, definite) {
                    (Person::Third, Number::Sing, Definite::Ind) => {
                        out.push(verbal::IK_FORMAL_THIRD);
                    }
                    (Person::First, Number::Sing, Definite::Ind) => {
                        out.extend(verbal::imperative_person(person, number, Definite::Def));
                    }
                    _ => {}
                }
                out
            }
        },
    })
}

/// Analyses `surface`, trying it as a compound when no simplex reading is found.
///
/// A compound inflects on its last member only (`DER-COMP-007`), so the members before the head are
/// inert text and the head is analysed exactly as a free word would be. That is what lets an
/// unlisted but well-formed compound come back `Known` instead of being rejected, which is the
/// single most common false rejection in a Hungarian spellchecker.
///
/// Rules: `DER-COMP-007`, `DER-COMP-021`
pub fn analyse(surface: &str, stacks: &Stacks) -> Verdict {
    // The closed paradigms the suffix parser cannot reach. Merged with the stem readings rather
    // than returned alone, because a form is often both: `értem` is 'for me' and 'I understand',
    // `hozzátok` is 'to you' and 'you bring it', `ellenem` is also a noun's possessive.
    let mut closed: Vec<Analysis> = Vec::new();
    if let Some(p) = crate::pronoun::analyse(surface) {
        // The lemma of the paradigm is its nominative: `PND-PERS-003` builds the oblique forms on
        // a stem of the case suffix, so nothing in `nekem` or `őket` resembles the pronoun itself.
        closed.push(Analysis {
            lemma: crate::pronoun::nominative(p.person, p.number).into(),
            pos: Pos::Pronoun,
            stack: vec![Morph::Case(p.case)],
        });
    }
    // PND-REFL-005: the independent possessive pronouns are suppletive, so no stem in the lexicon
    // reaches `enyém` or `övé`.
    if let Some(lemma) = crate::pronoun::possessive_pronoun(surface) {
        closed.push(Analysis {
            lemma: lemma.into(),
            pos: Pos::Pronoun,
            stack: Vec::new(),
        });
    }
    // PND-DEM-009: a demonstrative written together with a postposition is an adverb of its own.
    if let Some(lemma) = crate::postposition::demonstrative(surface) {
        closed.push(Analysis {
            lemma: lemma.into(),
            pos: Pos::Adverb,
            stack: Vec::new(),
        });
    }
    // PND-DEM-001: the demonstrative time and extent adverbs take `-rA`, which no adverb stack
    // reaches because the case is not an adverb's to take.
    if let Some(lemma) = crate::pronoun::demonstrative_adverb(surface) {
        closed.push(Analysis {
            lemma: lemma.into(),
            pos: Pos::Adverb,
            stack: vec![Morph::Case(Case::Sbl)],
        });
    }
    // PND-POSTP-005: a personal-suffixed postposition is a closed computed paradigm, not a stem
    // plus a case, so the suffix parser cannot reach it.
    if let Some(p) = crate::postposition::analyse(surface) {
        closed.push(Analysis {
            lemma: p.lemma.into(),
            pos: Pos::Postposition,
            stack: Vec::new(),
        });
    }
    let simplex = analyse_with(surface, stacks);
    // The stem index cannot reach a suppletive comparative or a generated ordinal, so those are
    // searched separately and merged, not used as a fallback: `szebben` is `szép` plus the
    // comparative plus the manner adverb and also the inessive of `szebb`; the first is only
    // reachable through the reverse table.
    let mut extra = closed;
    for a in reverse_table_readings(surface, stacks) {
        if !extra.contains(&a) {
            extra.push(a);
        }
    }
    for a in emphatic_readings(surface) {
        if !extra.contains(&a) {
            extra.push(a);
        }
    }
    // A written hyphen is evidence of a seam, so this path always runs: `A-vitamin` also
    // analyses through the decapitalised `a`, and that reading alone gives the lemma `a-vitamin`.
    if surface.contains('-') {
        for a in hyphenated_readings(surface, stacks) {
            if !extra.contains(&a) {
                extra.push(a);
            }
        }
    }
    // A sentence-initial capital is not a different word, and it may have a capitalised homograph,
    // so the lowercased readings are merged rather than tried only when nothing else was found:
    // `Ebben` parses as the abbreviation `Eb` plus the inessive, which hid the `ez` it almost
    // always is. The reverse is never tried, because a lowercase proper name really is an error.
    if let Some(lower) = decapitalised(surface)
        && let Verdict::Known(readings) = analyse(&lower, stacks)
    {
        for a in readings {
            if !extra.contains(&a) {
                extra.push(a);
            }
        }
    }
    if let Verdict::Known(readings) = simplex {
        let mut out = readings;
        for a in extra {
            if !out.contains(&a) {
                out.push(a);
            }
        }
        return settle(surface, out);
    }
    if !extra.is_empty() {
        return settle(surface, extra);
    }
    let mut out = compound_readings(surface, stacks);
    if out.is_empty() {
        out = hyphenated_readings(surface, stacks);
    }
    if out.is_empty() {
        out = solid_phrase_readings(surface, stacks);
    }
    if out.is_empty() {
        return simplex;
    }
    settle(surface, out)
}

/// Letters each side of a solid phrase needs at least.
const MIN_PHRASE_WORD_LETTERS: usize = 3;

/// The readings of a phrase that is written apart in its base form and solid once derived.
///
/// `AkH12` §127 writes a phrase solid when more than one derivational suffix follows it (`házhoz
/// szállít`, `házhozszállítási`) and §128 when `leg-` makes it a superlative (`harcra kész`,
/// `legharcrakészebb`); one suffix or a bare comparative keeps the gap (`házhoz szállítás`,
/// `szívhez szólóbb`), so neither is read here. The dependent is the case-marked word of the
/// phrase and the head the verb or adjective governing it.
///
/// Rules: `ORT-TOG-036`, `ORT-TOG-037`
fn solid_phrase_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    let mut out: Vec<Analysis> = Vec::new();
    if surface.contains('-') || starts_upper(surface) {
        return out;
    }
    let graded = SUPERLATIVE_PREFIXES
        .iter()
        .find_map(|&(p, m)| surface.strip_prefix(p).map(|rest| (Some(m), rest)));
    for (graded, body) in graded.into_iter().chain([(None, surface)]) {
        let long_enough = |w: &str| letters(w).count() >= MIN_PHRASE_WORD_LETTERS;
        for at in crate::compound::boundaries(body) {
            let (dependent, head) = body.split_at(at);
            if !long_enough(dependent) || !long_enough(head) || !case_marked(dependent) {
                continue;
            }
            let Verdict::Known(heads) = analyse_with(head, stacks) else {
                continue;
            };
            let heads = heads
                .into_iter()
                .filter(|r| matches!(r.pos, Pos::Verb | Pos::Adjective))
                .filter_map(|r| solid_phrase_head(r, graded));
            for r in heads {
                let a = Analysis {
                    lemma: format!("{dependent}{}", r.lemma).into(),
                    ..r
                };
                if !out.contains(&a) {
                    out.push(a);
                }
            }
        }
    }
    out
}

/// The head reading of a solid phrase: a superlative made of its comparative under `graded`, or
/// a reading with two or more lexeme-forming steps otherwise.
fn solid_phrase_head(mut r: Analysis, graded: Option<Morph>) -> Option<Analysis> {
    match graded {
        Some(morph) => {
            let at = r.stack.iter().position(|&m| m == Morph::Comparative)?;
            r.stack[at] = morph;
            Some(r)
        }
        None => (r.stack.iter().filter(|&&m| is_lexeme_forming(m)).count() >= 2).then_some(r),
    }
}

/// Whether `word` is a nominal carrying a case other than the nominative, as the dependent of a
/// phrase does (`házhoz`, `hasznot`).
///
/// Searched over the case stacks alone, because every cut of every unknown word asks it.
fn case_marked(word: &str) -> bool {
    static CASED: LazyLock<Stacks> = LazyLock::new(|| {
        Case::ALL
            .into_iter()
            .filter(|&c| c != Case::Nom)
            .flat_map(|c| [vec![Morph::Case(c)], vec![Morph::Plural, Morph::Case(c)]])
            .collect::<Vec<_>>()
            .into()
    });
    matches!(analyse_with(word, &CASED), Verdict::Known(readings)
        if readings.iter().any(|r| r.pos.is_nominal()))
}

/// The emphatic doubled reading of `surface`, if it has one.
///
/// `PND-PERS-007` writes a nominative pronoun before its own case-stem form or personal
/// postposition, which no suffix stack reaches because neither host is a stem plus a case.
///
/// Rules: `PND-PERS-007`
fn emphatic_readings(surface: &str) -> Vec<Analysis> {
    use crate::pronoun::Emphatic;
    match crate::pronoun::emphatic(surface) {
        Some(Emphatic::Pronoun(p)) => vec![Analysis {
            lemma: crate::pronoun::nominative(p.person, p.number).into(),
            pos: Pos::Pronoun,
            stack: vec![Morph::Case(p.case)],
        }],
        Some(Emphatic::Postposition(f)) => vec![Analysis {
            lemma: f.lemma.into(),
            pos: Pos::Postposition,
            stack: Vec::new(),
        }],
        None => Vec::new(),
    }
}

/// Completes a reading list: adds the derived citation form and orders by morph count.
fn settle(surface: &str, mut out: Vec<Analysis>) -> Verdict {
    for a in derived_citation_readings(surface, &out) {
        out.push(a);
    }
    for a in adverb_base_readings(&out) {
        out.push(a);
    }
    out.sort_by_key(|a| a.stack.len());
    Verdict::Known(out)
}

/// The reading in which a derived word is its own lemma.
///
/// `DER-MODEL-001` makes a derivational suffix lexeme-forming, so `uniós` is a word and its
/// citation form is `uniós`, not `unió` plus `-s`. Both of `NerKor`'s lemma columns record it that
/// way, 500 tokens each for `uniós` and `közötti`, and the base reading is kept beside it because
/// the derivation is still what the form is made of.
///
/// A participle counts too, by `DER-ADJ-022`'s conversion, and the corpus is emphatic about it:
/// tagged as an adjective, `követő` is lemmatised `követő` 1129651 times, `tervezett` 919930 and
/// `készített` 618714, while the same `készített` tagged as a verb is `készít` 232990 `[WC2]`. Both
/// answers are therefore offered and the category tells them apart. A converb is not included,
/// because there the corpus keeps the verb: `hivatkozva` is `hivatkozik` 231063 times.
///
/// Only a wholly lexeme-forming stack qualifies. With an inflection on top the surface is not a
/// citation form: `uniósok` is the plural of `uniós`, and claiming the plural as a lemma would be
/// worse than reporting the base.
///
/// Rules: `DER-MODEL-001`, `DER-ADJ-022`
fn derived_citation_readings(surface: &str, readings: &[Analysis]) -> Vec<Analysis> {
    let mut out: Vec<Analysis> = Vec::new();
    for r in readings {
        if r.stack.is_empty() || !r.stack.iter().copied().all(is_lexeme_forming) {
            continue;
        }
        let a = Analysis {
            lemma: surface.into(),
            pos: derived_pos(r),
            stack: Vec::new(),
        };
        if !readings.contains(&a) && !out.contains(&a) {
            out.push(a);
        }
    }
    out
}

/// The reading in which a manner adverb belongs to the word it is built on.
///
/// `DER-ADV-001` makes an adverb of an adjective and the corpus lemmatises the result on that
/// adjective: `követően` is `követő` 2459008 times, `vonatkozóan` is `vonatkozó` 503772 and
/// `illetően` is `illető` 388803 `[WC2]`. Where the base is a lexicon entry the ordinary path
/// already reaches it, which is how `könnyen` gives `könnyű`; where the base is a participle it is
/// no entry at all, and only regenerating the stack without its last step finds it.
///
/// Rules: `DER-ADV-001`, `NOM-ADJ-010`
fn adverb_base_readings(readings: &[Analysis]) -> Vec<Analysis> {
    let mut out: Vec<Analysis> = Vec::new();
    for r in readings {
        let Some((Morph::Derive(crate::derivation::DerId::An), rest)) = r.stack.split_last() else {
            continue;
        };
        if rest.is_empty() {
            continue;
        }
        for entry in lexicon().get(&r.lemma) {
            let Ok(base) = generate(entry, rest) else {
                continue;
            };
            let a = Analysis {
                lemma: base.into(),
                pos: Pos::Adjective,
                stack: Vec::new(),
            };
            if !readings.contains(&a) && !out.contains(&a) {
                out.push(a);
            }
        }
    }
    out
}

/// The category of what a reading's lexeme-forming steps built: the last step's output, not the
/// base's. `uniós` is an adjective although `unió` is a noun, a participle is an adjective and
/// `tekercselés` a noun although their bases are verbs.
///
/// Rules: `DER-MODEL-003`, `DER-ADJ-022`
fn derived_pos(r: &Analysis) -> Pos {
    r.stack
        .iter()
        .rev()
        .find_map(|&m| match m {
            Morph::Derive(id) => Some(id.record().output),
            m if is_lexeme_forming(m) => Some(Pos::Adjective),
            _ => None,
        })
        .unwrap_or(r.pos)
}

/// Whether a step builds a new lexeme rather than inflecting one.
fn is_lexeme_forming(morph: Morph) -> bool {
    matches!(
        morph,
        Morph::Derive(_)
            | Morph::PresentParticiple
            | Morph::PastParticiple
            | Morph::FutureParticiple
    )
}

/// The readings in which `surface` is a compound, its last member carrying the inflection.
///
/// `DER-COMP-007` makes the non-final members inert, so only the head is parsed for suffixes.
///
/// Rules: `DER-COMP-007`, `DER-COMP-021`
fn compound_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    let mut out: Vec<Analysis> = Vec::new();
    for (members, at) in crate::compound::member_prefixes(surface) {
        if members.is_empty() {
            continue;
        }
        let head = &surface[at..];
        let after_preverb = crate::compound::preverb_member(&members).is_some();
        let prefix: String = members.iter().map(|m| m.text.as_ref()).collect();
        for a in semi_word_readings(surface, &prefix, head, stacks) {
            if !out.contains(&a) {
                out.push(a);
            }
        }
        if let Verdict::Known(readings) = analyse_with(head, stacks) {
            for r in readings {
                // DER-COMP-021: the head's own category is constrained too, or `ház` plus the verb
                // `csen` makes `házcsen` a word. A participle is the exception: it is an adjective
                // by category but a verb form underneath, and the preverb belongs to that verb, so
                // `meglévő` and `elmondott` are words while `megház` is not.
                let participle = matches!(
                    r.stack.first(),
                    Some(
                        Morph::PresentParticiple
                            | Morph::PastParticiple
                            | Morph::FutureParticiple
                            | Morph::Converb
                    )
                ) || (after_preverb
                    && matches!(r.pos, Pos::Adjective | Pos::Noun)
                    && crate::compound::participle_shaped(&r.lemma));
                let head_pos = derived_pos(&r);
                if (!participle
                    && !crate::compound::head_allows(head_pos, after_preverb)
                    && !crate::compound::reflexive_head(head_pos, &members))
                    || !crate::compound::numeral_head_allows(head_pos, &members)
                    || !crate::compound::admissible_head(&r.lemma, &members)
                {
                    continue;
                }
                let a = Analysis {
                    lemma: format!("{prefix}{}", r.lemma).into(),
                    pos: r.pos,
                    stack: r.stack,
                };
                if !out.contains(&a) {
                    out.push(a);
                }
            }
        }
    }
    out
}

/// The readings in which the compound's head is a semi-word.
///
/// `DER-COMP-017` makes a semi-word a member that closes the compound and turns it into an
/// adjective, and `DER-COMP-022` makes that the preferred reading where a suffix reading competes.
/// The whole word is then the lemma, which is also what the corpus records: `értelemszerűen`
/// 101549 times under `értelemszerű`; the competing reading is `kutya|szer` plus the `-ú/-ű`
/// derivation, whose lemma would be the non-word `kutyaszer`.
///
/// `DER-MODEL-010` makes an adjective output lowering. The front semi-words show it in the
/// accusative alone (`hibamenteset`, `NOM-LOW-003`); after the `a`/`e` of `-fajta` and `-féle` it
/// shows nowhere (`NOM-PL-003`).
///
/// Rules: `DER-COMP-017`, `DER-COMP-022`, `DER-MODEL-010`
fn semi_word_readings(surface: &str, prefix: &str, head: &str, stacks: &Stacks) -> Vec<Analysis> {
    let mut out = Vec::new();
    for (semi, pos) in crate::compound::semi_words() {
        let (semi, pos) = (semi.as_ref(), *pos);
        if !head.starts_with(semi) || !crate::compound::semi_word_joins(prefix, semi) {
            continue;
        }
        let lemma = format!("{prefix}{semi}");
        let mut entry = Entry::new(lemma.as_str(), pos);
        // `DER-ADJ-014`'s output inflects as an ordinary adjective, and the `ű` takes the linked
        // manner adverb: `szerűen` beats `szerűn` everywhere in the corpus.
        if pos == Pos::Adjective {
            entry.traits = entry.traits.with(Trait::LinkedPlural).with(Trait::Low);
        }
        for stack in stacks.matching(surface) {
            if generate(&entry, stack).as_deref() != Ok(surface) {
                continue;
            }
            let a = Analysis {
                lemma: lemma.clone().into(),
                pos,
                stack: stack.clone(),
            };
            if !out.contains(&a) {
                out.push(a);
            }
        }
    }
    out
}

/// Whether `prefix` composes with the interrogative `base` at all (`PND-INDF-001`).
///
/// The negative prefix geminates before the `mi-` bases (`semmi`, `semmilyen`, `semmikor`) and is
/// `se` before every other base, `m`-initial ones included (`semekkora`, `semeddig`, `AkH12` §123);
/// `senki` and `mindenki` are listed, and `minden` opens no `m`-initial base (`*mindenmi`).
///
/// Rules: `PND-INDF-001`, `ORT-TOG-032`
fn series_composes(prefix: &str, base: &str) -> bool {
    let mi_initial = base.starts_with("mi");
    match prefix {
        "sem" => mi_initial,
        "se" => !mi_initial && base != "ki",
        "minden" => !base.starts_with('m') && base != "ki",
        _ => true,
    }
}

/// The readings of `ugyan-` or `mind-` on a demonstrative, which inflects as the base does.
///
/// Rules: `PND-DEM-001`, `PND-DEM-002`
fn demonstrative_series_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    let mut out = Vec::new();
    for (prefixes, bases) in [
        (Role::DemonstrativePrefix, Role::DemonstrativeBase),
        (Role::UniversalPrefix, Role::UniversalDemonstrativeBase),
    ] {
        let Some((prefix, rest)) = words(prefixes).iter().find_map(|p| {
            surface
                .strip_prefix(p.as_ref())
                .map(|rest| (p.as_ref(), rest))
        }) else {
            continue;
        };
        let bases = words(bases);
        if !bases.iter().any(|b| rest.starts_with(b.as_ref())) {
            continue;
        }
        let Verdict::Known(readings) = analyse_with(rest, stacks) else {
            continue;
        };
        for r in readings {
            if !bases.contains(&r.lemma) {
                continue;
            }
            let a = Analysis {
                lemma: format!("{prefix}{}", r.lemma).into(),
                pos: r.pos,
                stack: r.stack,
            };
            if !out.contains(&a) {
                out.push(a);
            }
        }
    }
    out
}

/// The readings of a prefixed pronoun, when `surface` is one.
///
/// The base is analysed on its own, so its paradigm and its harmony come for free: `valakinek` is
/// `vala` plus the dative of `ki`, and `ki` is front whatever the back `a` of the prefix suggests.
///
/// Rules: `PND-INDF-001`, `PND-INT-001`
fn series_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    let mut out = Vec::new();
    for prefix in words(Role::SeriesPrefix) {
        let prefix: &str = prefix;
        let Some(rest) = surface.strip_prefix(prefix) else {
            continue;
        };
        // The remainder has to begin with an interrogative before it is worth analysing. Without
        // the prefilter every token paid for up to eight full sub-analyses, which is what an
        // evaluation over a million tokens notices.
        if !words(Role::InterrogativeBase)
            .iter()
            .any(|b| rest.starts_with(b.as_ref()))
        {
            continue;
        }
        let Verdict::Known(readings) = analyse_with(rest, stacks) else {
            continue;
        };
        for r in readings {
            if !lists(Role::InterrogativeBase, &r.lemma) || !series_composes(prefix, &r.lemma) {
                continue;
            }
            let a = Analysis {
                lemma: format!("{prefix}{}", r.lemma).into(),
                pos: r.pos,
                stack: r.stack,
            };
            if !out.contains(&a) {
                out.push(a);
            }
        }
    }
    out
}

/// The readings of a numeral spelled out in words.
///
/// `PND-CARD` composes an unbounded set out of a couple of dozen words, so the composed forms
/// cannot be listed and none of them had a reading: `tizenöt` alone is 182316 tokens [WC2].
/// `PND-NUMINF-002` inflects the last element only, so the last spelled token plus whatever
/// follows it is analysed as a word and the rest is the inert prefix, exactly as generation
/// composes it. `PND-FRAC-003`: a cardinal before a fraction is written solid the same way
/// (`kétharmada`).
///
/// Rules: `PND-CARD-003`, `PND-NUMINF-002`, `PND-FRAC-003`
fn spelled_numeral_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    let mut out: Vec<Analysis> = Vec::new();
    for at in crate::numeral::token_bounds(surface) {
        let (prefix, head) = surface.split_at(at);
        if crate::numeral::parse(prefix).is_some() {
            for r in ordinal_readings(head, stacks) {
                if r.pos == Pos::Noun {
                    out.push(Analysis {
                        lemma: format!("{prefix}{}", r.lemma).into(),
                        pos: Pos::Noun,
                        stack: r.stack,
                    });
                }
            }
        }
        let Verdict::Known(readings) = analyse_with(head, stacks) else {
            continue;
        };
        for r in readings {
            if r.pos != Pos::Numeral {
                continue;
            }
            let lemma = format!("{prefix}{}", r.lemma);
            if crate::numeral::parse(&lemma).is_none() {
                continue;
            }
            let a = Analysis {
                lemma: lemma.into(),
                pos: Pos::Numeral,
                stack: r.stack,
            };
            if !out.contains(&a) {
                out.push(a);
            }
        }
    }
    out
}

/// The bare truncated manner demonstrative: `oly`, `ily`, `mily`, `ugyanoly`, `amily`.
///
/// `PND-DEM-008`'s truncation is not only an accusative stem. The truncated form stands on its own
/// as the formal variant of the long one, and it is a lemma rather than an inflection of it: the
/// corpus has `oly` 681644 times against 6 for `olyan` lemmatised as `oly`. The inventory follows
/// from the long forms, which are already listed, so nothing is added to the lexicon.
///
/// Rules: `PND-DEM-008`
fn truncated_demonstrative_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    let Some(long) = lengthened_demonstrative(surface) else {
        return Vec::new();
    };
    let Verdict::Known(readings) = analyse(&long, stacks) else {
        return Vec::new();
    };
    // Only the bare long form counts. An inflected one would make `oly` the citation form of
    // `olyanok`, which it is not.
    readings
        .into_iter()
        .filter(|r| r.stack.is_empty())
        .map(|r| Analysis {
            lemma: surface.into(),
            pos: r.pos,
            stack: Vec::new(),
        })
        .collect()
}

/// The long form a truncated manner demonstrative belongs to, if `surface` is one.
///
/// The harmony of the restored vowel is fixed by the ending, not computed: `oly` takes `an` and
/// `ily` takes `en`.
fn lengthened_demonstrative(surface: &str) -> Option<String> {
    for (ending, vowel) in [("oly", 'a'), ("ily", 'e')] {
        if surface.ends_with(ending) {
            return Some(format!("{surface}{vowel}n"));
        }
    }
    None
}

/// The readings the stem index cannot reach, so they are searched through their own tables.
///
/// A suppletive comparative, a generated ordinal, a `leg-` superlative and a prefixed pronoun all
/// have a surface that no prefix of the lemma matches.
fn reverse_table_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    let mut out = irregular_comparative_readings(surface, stacks);
    for a in spelled_numeral_readings(surface, stacks) {
        if !out.contains(&a) {
            out.push(a);
        }
    }
    for a in truncated_demonstrative_readings(surface, stacks) {
        if !out.contains(&a) {
            out.push(a);
        }
    }
    for a in series_readings(surface, stacks) {
        if !out.contains(&a) {
            out.push(a);
        }
    }
    for a in demonstrative_series_readings(surface, stacks) {
        if !out.contains(&a) {
            out.push(a);
        }
    }
    for a in ordinal_readings(surface, stacks) {
        if !out.contains(&a) {
            out.push(a);
        }
    }
    for a in letter_name_readings(surface, stacks) {
        if !out.contains(&a) {
            out.push(a);
        }
    }
    for a in superlative_readings(surface, stacks) {
        if !out.contains(&a) {
            out.push(a);
        }
    }
    out
}

/// Most hyphens a token may carry and still be read as a hyphenated compound.
///
/// `ORT-TOG-030` writes the hyphen into a compound of three or more members, and the solid case
/// stops at six members, so five hyphens is already past anything the orthography produces. Beyond
/// it a token is not a word that needs analysing.
const MAX_HYPHENS: usize = 5;

/// The readings of a hyphenated compound.
///
/// `ORT-TOG-024` writes a hyphen into a compound of six syllables or more and `ORT-TOG-030` into
/// one of three or more members, so a hyphenated compound is as productive as a solid one and no
/// word list covers it. The members are analysed separately and the head carries the inflection,
/// exactly as in the solid case.
///
/// Rules: `ORT-TOG-024`, `ORT-TOG-030`, `DER-COMP-007`
fn hyphenated_readings(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    // Each seam analyses both of its sides, and those sides are analysed at every seam again, so
    // the work grows exponentially in the number of hyphens: measured, four hyphens took 0.21s,
    // eight took 3.15s and twelve did not finish, and on a spawned thread the recursion overflowed
    // the stack. Real crawled text carries such tokens by the thousand, as URL slugs and runs of
    // noise, and one of them aborted the process mid-corpus.
    if surface.matches('-').count() > MAX_HYPHENS {
        return Vec::new();
    }
    let mut out: Vec<Analysis> = Vec::new();
    for (at, _) in surface.char_indices().filter(|(_, c)| *c == '-') {
        let (left, right) = (&surface[..at], &surface[at + 1..]);
        if left.is_empty() || right.is_empty() {
            continue;
        }
        // A single letter is a member by its name, which `ORT-HYPH` writes with a hyphen before a
        // common noun: `A-vitamin`, `B-terv`, `x-tengely`. It is not a lexicon entry, so it cannot
        // be asked to analyse, and `A-vitamin` must not be read through the article `a`.
        let letter = letters(left).count() == 1;
        // `ORT-NAME-023` lowercases every member of a hyphenated name inside its `-i` adjective,
        // so a member may be a name written small: `győr` in `győr-moson-soproni` is `Győr`.
        let name = upper_initial(left).is_some_and(|c| !lexicon().get(&c).is_empty());
        // A member keeps the capital a sentence or an institution name gives it: `Észak-atlanti`
        // in `Észak-atlanti Szerződés` (`ORT-NAME-034`) is `észak` before `atlanti`.
        let capitalised = starts_upper(left)
            && matches!(analyse(&lower_initial(left), stacks), Verdict::Known(_));
        // `ORT-ABBR-017` writes a compound member onto an acronym with a hyphen: `PNR-adatok`,
        // `EU-tagállam`. The acronym is not a word the analyser can read, so its shape admits it.
        let acronym = crate::acronym::is_letter_acronym(left);
        if !letter
            && !name
            && !acronym
            && !capitalised
            && !matches!(analyse(left, stacks), Verdict::Known(_))
        {
            continue;
        }
        // `ORT-HYPH-004` writes a semi-word on a proper name with a hyphen, and the name keeps
        // its capital: `Petőfi-szerű`, `Coca-Cola-szerű`.
        for a in semi_word_readings(surface, &format!("{left}-"), right, stacks) {
            if !out.contains(&a) {
                out.push(a);
            }
        }
        if let Verdict::Known(readings) = analyse(right, stacks) {
            for r in readings {
                // No length floor here: `admissible_head` caps the noise of *searching* for a
                // seam, and this seam is written. `egy-egy` has members of two letters each.
                let first = if crate::compound::head_allows(r.pos, false)
                    || reduplicated_preverb(left, &r.lemma)
                {
                    left.to_owned()
                } else if let Some(l) = coordinate_member(left, &r, stacks) {
                    l
                } else {
                    continue;
                };
                // The name's own spelling capitalises every member (`ORT-NAME-023`), so the lemma
                // does too once the head shows it is one: `dél-kínai` is `Dél-Kína`, not
                // `dél-Kína`. A common compound keeps its members as written.
                let lemma = format!("{first}-{}", r.lemma);
                let lemma = if starts_upper(&r.lemma) {
                    lemma
                        .split('-')
                        .map(|m| upper_initial(m).unwrap_or_else(|| m.to_owned()))
                        .collect::<Vec<_>>()
                        .join("-")
                } else {
                    lemma
                };
                let a = Analysis {
                    lemma: lemma.into(),
                    pos: r.pos,
                    stack: r.stack,
                };
                if !out.contains(&a) {
                    out.push(a);
                }
            }
        }
    }
    out
}

/// Whether `left` repeats the preverb of the prefixed verb `lemma`: `meg-megáll`, `ki-kinéz`,
/// `vissza-visszatér`.
///
/// Rules: `DER-PREF-003`
fn reduplicated_preverb(left: &str, lemma: &str) -> bool {
    crate::verbal::preverb_base(lemma).is_some_and(|base| lemma.strip_suffix(base) == Some(left))
}

/// The lemma of `left` as the first member of a coordinative compound with `head`.
///
/// `AkH12 §101 b)` joins two words of the same role and the same suffix with a hyphen:
/// `sütnek-főznek`, `rohantak-szaladtak`, `kínos-fájdalmas`. So `left` must carry a suffix and
/// have a reading of the head's category with the head's own stack; a pair of bare words is a
/// listed compound or none (`süt-főz`, not `apró-nagy`).
///
/// Rules: `DER-COMP-015`
fn coordinate_member(left: &str, head: &Analysis, stacks: &Stacks) -> Option<String> {
    let Verdict::Known(readings) = analyse(left, stacks) else {
        return None;
    };
    readings
        .into_iter()
        .find(|l| l.pos == head.pos && l.stack == head.stack && *l.lemma != *left)
        .map(|l| l.lemma.into())
}

/// The colloquial `-Ok` of an `-ik` verb's first singular, the variant of the `-Om` generation
/// returns (`VRB-IK-002`): `dolgozok` beside `dolgozom`.
fn colloquial_ik_1sg(entry: &Entry, stack: &[Morph]) -> Option<String> {
    if !matches!(
        stack,
        [Morph::Verbal {
            mood: Mood::Ind,
            tense: Tense::Pres,
            person: Person::First,
            number: Number::Sing,
            definite: Definite::Ind,
        }]
    ) || !crate::verbal::is_ik(entry)
    {
        return None;
    }
    let standard = generate(entry, stack).ok()?;
    Some(format!("{}k", standard.strip_suffix('m')?))
}

/// The bare `-t` locative after a consonant, the variant of the `-Ott` generation returns
/// (`NOM-CASE-016`): `Kolozsvárt` beside `Kolozsvárott`, `Vásárhelyt`.
fn bare_locative(entry: &Entry, stack: &[Morph]) -> Option<String> {
    if stack != [Morph::Case(Case::Loc)] || !entry.has(Trait::Locative) {
        return None;
    }
    let linked = generate(entry, stack).ok()?;
    linked
        .strip_suffix("tt")
        .filter(|s| s.len() > entry.lemma.len())
        .map(|_| format!("{}t", entry.lemma))
}

/// The untruncated accusative of an `ilyen`/`olyan` demonstrative, the variant of the truncated
/// form generation returns.
///
/// `PND-DEM-008` generates `ilyet` and `olyat`, which beat `ilyent` and `olyant` about 23 to 1,
/// and names the full-stem forms as the attested variant. The full stem ends in `n` and takes
/// the bare `-t`.
fn full_demonstrative_accusative(entry: &Entry, stack: &[Morph]) -> Option<String> {
    if stack != [Morph::Case(Case::Acc)] {
        return None;
    }
    crate::generate::truncating_demonstrative(&entry.lemma)?;
    Some(format!("{}t", entry.lemma))
}

/// The linked plural of a participle, the variant of the bare `-k` generation returns.
///
/// `NOM-PL-003` gives an `ó`/`ő`-final adjective both `-k` and `-Ak`, and a participle is such an
/// adjective (`DER-ADJ-022`): `értendők` beside `értendőek`, `kezdődők` beside `kezdődőek`. The
/// lexicon's own adjectives reach the linked form through a variant entry; a participle is built
/// inside the stack, so its variant is built from the generated form.
fn linked_participle_plural(entry: &Entry, stack: &[Morph]) -> Option<String> {
    let at = stack.iter().position(|m| {
        matches!(
            m,
            Morph::PresentParticiple | Morph::PastParticiple | Morph::FutureParticiple
        )
    })?;
    if stack.get(at + 1) != Some(&Morph::Plural) {
        return None;
    }
    let participle = generate(entry, &stack[..=at]).ok()?;
    if !participle.ends_with(['ó', 'ő']) {
        return None;
    }
    let whole = generate(entry, stack).ok()?;
    let rest = whole.strip_prefix(participle.as_str())?.strip_prefix('k')?;
    let link = pick2(crate::generate::harmony(&participle, entry), "ak", "ek");
    Some(format!("{participle}{link}{rest}"))
}

/// The formal imperative of an `-ik` verb, the variant of the form generation returns.
///
/// `VRB-IK-005`: the traditional `-ik` imperative has `-jék` in the third singular and `-jAm` in
/// the first, beside the neutral `-jOn` and `-jAk` that generation produces. The formal cell is
/// the neutral one with its ending exchanged, so `uralkodjék` and `egyem` are read without a
/// second paradigm.
/// `VRB-ADVP-002`: the formal `-vÁn` beside the converb's `-vA`, on the same stem
/// (`mondván`, `lévén`).
fn formal_converb(entry: &Entry, stack: &[Morph]) -> Option<String> {
    if stack != [Morph::Converb] {
        return None;
    }
    let plain = generate(entry, stack).ok()?;
    let (stem, low) = plain
        .strip_suffix("va")
        .map(|s| (s, "ván"))
        .or_else(|| plain.strip_suffix("ve").map(|s| (s, "vén")))?;
    Some(format!("{stem}{low}"))
}

fn formal_ik_imperative(entry: &Entry, stack: &[Morph]) -> Option<String> {
    let [
        Morph::Verbal {
            mood: Mood::Imp,
            person,
            number: Number::Sing,
            definite: Definite::Ind,
            ..
        },
    ] = stack
    else {
        return None;
    };
    if !crate::verbal::is_ik(entry) {
        return None;
    }
    let neutral = generate(entry, stack).ok()?;
    match person {
        Person::Third => {
            let cut = ["on", "en", "ön"]
                .iter()
                .find_map(|e| neutral.strip_suffix(e))?;
            Some(format!("{cut}ék"))
        }
        Person::First => Some(format!("{}m", neutral.strip_suffix('k')?)),
        Person::Second => None,
    }
}

/// The `-i` adjective on a v-stem's bound alternant, where the lexicon lists it: `tavi` beside
/// `tói`, `havi` beside `hói` (`PH-LEN-002` names `tavi`, `havi`, `művi` as lexicalised).
///
/// A name's derivative is its head member's (`ORT-NAME-022`), so the listed adjective replaces
/// the head's in it: `Huron-tó` gives `Huron-tavi` beside `Huron-tói`, as `AkH12` §175 writes.
///
/// Rules: `ORT-NAME-022`, `PH-LEN-002`
fn listed_bound_stem_adjective(entry: &Entry, stack: &[Morph]) -> Option<String> {
    let der = &crate::derivation::I;
    if stack != [Morph::Derive(crate::derivation::DerId::I)] {
        return None;
    }
    let lemma: &str = &entry.lemma;
    let head_at = lemma.rfind([' ', '-']).map_or(0, |i| i + 1);
    let lx = lexicon();
    let head = if head_at == 0 {
        entry
    } else {
        lx.get(&lower_initial(&lemma[head_at..]))
            .iter()
            .find(|e| e.pos == Pos::Noun)?
    };
    let bound = crate::alternation::bound_alternant(head, &der.suffix());
    if *bound == *head.lemma {
        return None;
    }
    let listed = format!("{bound}i");
    lx.get_pos(&listed, Pos::Adjective)?;
    let standard = generate(entry, stack).ok()?;
    let own = generate(head, stack).ok()?;
    Some(format!("{}{listed}", standard.strip_suffix(own.as_str())?))
}

/// The long past participle `-Ott` of a verb whose finite past takes `-t`, where the lexicon
/// lists it: `írott` beside `írt` (`VRB-PART-002`).
///
/// Rules: `VRB-PART-002`
fn listed_long_participle(entry: &Entry, stack: &[Morph]) -> Option<String> {
    if stack != [Morph::PastParticiple] || entry.pos != Pos::Verb {
        return None;
    }
    let short = generate(entry, stack).ok()?;
    let stem = crate::verbal::bound_stem(entry);
    let h = crate::generate::harmony(&stem, entry);
    let long = format!("{stem}{}tt", pick3(h, "o", "e", "ö"));
    (long != short && !lexicon().get(&long).is_empty()).then_some(long)
}

/// The infinitive of an epenthetic `-ik` verb on its full stem, the variant of the syncopated
/// one generation returns: `ugorni` beside `ugrani`, `fürödni` beside `fürdeni` (`VRB-LINK-010`).
///
/// Rules: `VRB-LINK-010`, `VRB-STEM-008`
fn full_stem_infinitive(entry: &Entry, stack: &[Morph]) -> Option<String> {
    if stack != [Morph::Infinitive] {
        return None;
    }
    let full = crate::verbal::full_stem(entry)?;
    Some(crate::generate::attach(
        &full,
        entry,
        &crate::verbal::infinitive(&full, entry),
        false,
    ))
}

/// The long second-singular imperative, the variant of the short form generation returns.
///
/// `VRB-IMP-008` gives both second-singular imperative cells two equally correct forms, the short
/// one being the unmarked default in writing: `várj ~ várjál`, `várd ~ várjad`. Generation answers
/// with the short one, and this supplies the long one.
///
/// Both variants are built on the imperative stem, which the third singular shows before its
/// `-On`: `várjon` gives `várj`, so `várjál` and `várjad`. The short indefinite itself is not
/// that stem for the irregular verbs, whose archaic short form lengthens the vowel
/// (`VRB-IMP-009`): `végy` beside `vegyen`, so `vegyél` and not `*végyél`.
///
/// Rules: `VRB-IMP-008`, `VRB-IMP-009`
fn long_imperative_2sg(entry: &Entry, stack: &[Morph]) -> Option<String> {
    let [
        Morph::Verbal {
            mood: Mood::Imp,
            person: Person::Second,
            number: Number::Sing,
            definite,
            ..
        },
    ] = stack
    else {
        return None;
    };
    if *definite == Definite::Two {
        return None;
    }
    let third = generate(
        entry,
        &[Morph::Verbal {
            mood: Mood::Imp,
            tense: Tense::Pres,
            person: Person::Third,
            number: Number::Sing,
            definite: Definite::Ind,
        }],
    )
    .ok()?;
    let spans: Vec<_> = letters(&third).collect();
    let [.., vowel, n] = spans.as_slice() else {
        return None;
    };
    if n.text != "n" || !vowel.letter.is_vowel() {
        return None;
    }
    let stem = &third[..vowel.at];
    // The imperative stem decides, under the lemma's marks: `ír` is all-neutral but antiharmonic,
    // so it takes `-ál`, and the suppletive `legy-` of `van` is front whatever the lemma says.
    let h = crate::generate::harmony(stem, entry);
    let ending = if *definite == Definite::Def {
        pick2(h, "ad", "ed")
    } else {
        pick2(h, "ál", "él")
    };
    Some(format!("{stem}{ending}"))
}

/// The regular comparative of an adjective whose irregular one has a doublet, with the rest of
/// the stack on it: `lassúbb` and `leglassúbbat` beside the generated `lassabb` and
/// `leglassabbat`.
///
/// The comparative is an inflected stem and so lowering (`NOM-LOW-002`), which is all the rest of
/// the stack needs to know of it.
///
/// Rules: `NOM-ADJ-005`, `NOM-ADJ-006`, `NOM-LOW-002`
fn regular_comparative(entry: &Entry, stack: &[Morph]) -> Option<String> {
    let (first, rest) = stack.split_first()?;
    let prefix = match first {
        Morph::Comparative => "",
        Morph::Superlative => "leg",
        Morph::Excessive => "legesleg",
        _ => return None,
    };
    if !lists(Role::ComparativeDoublet, &entry.lemma) || generate(entry, stack).is_err() {
        return None;
    }
    let lowering = entry.has(Trait::Low);
    let mut comparative = Entry::new(
        crate::generate::attach(
            &entry.lemma,
            entry,
            &crate::generate::comparative(),
            lowering,
        ),
        entry.pos,
    );
    comparative.traits = TraitSet::from_iter_of([Trait::Low]);
    Some(format!("{prefix}{}", generate(&comparative, rest).ok()?))
}

/// A producer of one further correct form of a cell.
type VariantForm = fn(&Entry, &[Morph]) -> Option<String>;

/// The forms a cell has beside the one generation returns.
///
/// Generation answers with one string per cell; these are the other correct spellings of the same
/// cell, each computed from the entry rather than stored.
const VARIANT_FORMS: &[VariantForm] = &[
    formal_converb,
    full_stem_infinitive,
    listed_long_participle,
    listed_bound_stem_adjective,
    long_imperative_2sg,
    formal_ik_imperative,
    linked_participle_plural,
    full_demonstrative_accusative,
    colloquial_ik_1sg,
    bare_locative,
    regular_comparative,
];

/// Every form the engine vouches for in one cell: the one generation returns and the variants
/// analysis accepts beside it, in that order.
///
/// Rules: `VRB-IMP-008`, `VRB-IK-002`, `VRB-IK-005`, `NOM-PL-003`, `NOM-CASE-016`
#[must_use]
pub fn forms(entry: &Entry, stack: &[Morph]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut push = |form: Option<String>| {
        if let Some(f) = form
            && !out.contains(&f)
        {
            out.push(f);
        }
    };
    push(generate(entry, stack).ok());
    for f in VARIANT_FORMS {
        push(f(entry, stack));
    }
    for (v, scope) in variants(entry) {
        if scope.admits(stack) {
            push(generate(&v, stack).ok());
        }
    }
    out
}

/// Analyses `surface` against `stacks`, returning every reading that regenerates it.
///
/// Simplex only: see [`analyse`] for the compound-aware entry point.
///
/// `stacks` is the caller's search space; the engine does not invent feature combinations.
pub fn analyse_with(surface: &str, stacks: &Stacks) -> Verdict {
    let lx = lexicon();
    let index = &*INDEX;
    let mut out: Vec<Analysis> = Vec::new();
    let mut seen_stem = false;

    let mut cuts: Vec<usize> = letters(surface).map(|s| s.at).collect();
    cuts.push(surface.len());
    for &cut in cuts.iter().rev() {
        let prefix = &surface[..cut];
        for i in index.get(prefix) {
            let Some(entry) = lx.at(i) else {
                continue;
            };
            if !plausible(entry, surface) {
                continue;
            }
            seen_stem = true;
            let alts = variants(entry);
            for stack in stacks.matching(surface) {
                let hit = generate_toward(entry, stack, surface).as_deref() == Ok(surface)
                    || VARIANT_FORMS
                        .iter()
                        .any(|f| f(entry, stack).as_deref() == Some(surface))
                    || alts.iter().any(|(v, scope)| {
                        scope.admits(stack)
                            && generate_toward(v, stack, surface).as_deref() == Ok(surface)
                    });
                if hit {
                    let a = Analysis {
                        lemma: lemma_of(entry, stack),
                        pos: entry.pos,
                        stack: stack.clone(),
                    };
                    if !out.contains(&a) {
                        out.push(a);
                    }
                }
            }
        }
    }

    if !out.is_empty() {
        out.sort_by_key(|a| a.stack.len());
        return Verdict::Known(out);
    }
    let _ = seen_stem;
    if !looks_hungarian(surface) {
        return Verdict::IllFormed;
    }
    // A listed lemma followed by a string the stacks write as a suffix, which no stack produces
    // on that lemma, is that lemma with the wrong suffix shape: `házben` is `ház` with the front
    // inessive, `orrt` is `orr` with the bare accusative `NOM-ACC-006` forbids. What is not a
    // suffix leaves the word merely unlisted: `kakapó` is not `kaka` with anything. A lemma of
    // fewer than three letters is a letter name (`mém` is not a misspelling of `mé`), and only a
    // lemma of an inflecting category counts: an adverb takes a case by its own lights
    // (`innentől`), and the checker has no paradigm to hold it to.
    //
    // A capitalised surface is exempt: an unlisted name is not a misspelling of whatever it
    // begins with (`Dale` as `dal`, `Peter` as `Pete`, `Janet` as `Jan`: sixty name tokens in
    // twenty thousand).
    let known_stem_with_suffix_shape = !starts_upper(surface)
        && letters(surface).skip(MIN_GUESSED_STEM).any(|s| {
            lx.get(&surface[..s.at]).iter().any(|e| {
                matches!(
                    e.pos,
                    Pos::Noun | Pos::Adjective | Pos::Verb | Pos::Numeral | Pos::Pronoun
                )
            }) && stacks.writes_suffix(&surface[s.at..])
        });
    if known_stem_with_suffix_shape {
        Verdict::IllFormed
    } else {
        Verdict::WellFormed
    }
}

/// The fewest letters a guessed stem may have.
const MIN_GUESSED_STEM: usize = 3;

/// Readings of a surface whose stem no lexicon lists, from the suffix rules alone.
///
/// A source of truth does not vouch for `Loder`, and [`analyse`] rightly leaves `Lodernek`
/// unknown. But the suffix is not in doubt: every stack the search space holds is tried on every
/// prefix that could be a stem, longest first, and a prefix that regenerates the surface under the
/// ordinary rules is offered as its lemma. Every such cut is returned, fewest morphs first and
/// longest stem next, because the stem is exactly what nothing here knows: `Lodernek` is `Loder`
/// plus the dative and `Lodern` plus the plural alike, and a caller building an annotation or a
/// segmentation seed chooses, knowing it came from here.
///
/// A capitalised surface is a name and is tried as a noun; a lowercase one as a noun, an
/// adjective and a verb. A stem must end the way a word may (`PH-SYL-007`), so `Lillianra` is
/// never `Lillianr` plus a possessive, and a verb stem must end in a consonant (`PH-SYL-011`), so
/// `hugának` is never a first singular. A hyphenated name (`Chilcote-ot`) is tried under every
/// pronunciation the suffix admits (`ORT-FOR-012`), the way the checker accepts it.
#[must_use]
pub fn guess(surface: &str, stacks: &Stacks) -> Vec<Analysis> {
    if let Some((left, tail)) = surface.split_once('-')
        && starts_upper(left)
        && !tail.is_empty()
    {
        return guess_hyphenated(surface, left, tail, stacks);
    }
    if !looks_hungarian(surface) {
        return Vec::new();
    }
    let categories: &[Pos] = if starts_upper(surface) {
        &[Pos::Noun]
    } else {
        &[Pos::Noun, Pos::Adjective, Pos::Verb]
    };
    let cuts: Vec<usize> = letters(surface)
        .map(|s| s.at)
        .filter(|&at| at > 0 && letters(&surface[..at]).count() >= MIN_GUESSED_STEM)
        .collect();
    let mut out = Vec::new();
    for &cut in cuts.iter().rev() {
        let stem = &surface[..cut];
        let last = letters(stem).last().map(|s| s.letter);
        if !letters(stem).any(|s| s.letter.is_vowel())
            || !picur_morph_ortho::well_formed_final_cluster(stem)
        {
            continue;
        }
        for &pos in categories {
            if pos == Pos::Verb && last.is_some_and(picur_morph_ortho::Letter::is_vowel) {
                continue;
            }
            let entry = Entry::new(stem, pos);
            for stack in stacks.matching(surface) {
                if !stack.is_empty() && generate(&entry, stack).as_deref() == Ok(surface) {
                    out.push(Analysis {
                        lemma: stem.into(),
                        pos,
                        stack: stack.clone(),
                    });
                }
            }
        }
    }
    out.sort_by_key(|a| (a.stack.len(), std::cmp::Reverse(a.lemma.len())));
    out
}

/// [`guess`] for a name carrying its suffix after a hyphen.
///
/// The suffix consumes three facts of the name's pronunciation, so those are enumerated: each
/// harmony class, and the last sound as a vowel or as a consonant. Where the tail begins with a
/// consonant that is the one it assimilated to (`Vole-lal`); otherwise any consonant serves,
/// since the linking vowel is all a consonant-final stem shows (`Chilcote-ot`).
fn guess_hyphenated(surface: &str, left: &str, tail: &str, stacks: &Stacks) -> Vec<Analysis> {
    let consonant = letters(tail)
        .next()
        .filter(|s| s.letter.is_consonant())
        .map_or("b", |s| s.letter.as_str());
    let sounds: [Option<&str>; 2] = [None, Some(consonant)];
    let mut out: Vec<Analysis> = Vec::new();
    for h in [Trait::Back, Trait::Front, Trait::FrontRounded] {
        for last in sounds {
            let mut entry = Entry::new(left, Pos::Noun);
            entry.traits = entry.traits.with(Trait::Spoken).with(h);
            entry.last_sound = last.map(Into::into);
            for stack in stacks.matching(surface) {
                if stack.is_empty() || generate(&entry, stack).as_deref() != Ok(surface) {
                    continue;
                }
                let a = Analysis {
                    lemma: left.into(),
                    pos: Pos::Noun,
                    stack: stack.clone(),
                };
                if !out.contains(&a) {
                    out.push(a);
                }
            }
        }
    }
    out.sort_by_key(|a| a.stack.len());
    out
}

/// Whether a string could be a Hungarian word: every character is an alphabet letter, and it has
/// at least one vowel.
fn looks_hungarian(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut n = 0usize;
    let mut vowels = 0usize;
    for span in letters(s) {
        n += span.text.len();
        if span.letter.is_vowel() {
            vowels += 1;
        }
    }
    // No Hungarian word opens on a doubled consonant (`PH-SYL-003`): `hhez` is the letter `h`
    // suffixed without its hyphen (`ORT-ABBR-009`). None opens on `y` either, which is a letter
    // of the extended alphabet only (`WS-ALPH-002`) and respelled `j` in every loan (`jacht`,
    // `jóga`): `yöld` is `zöld` typed on the wrong layout (`REP-KEY-004`). A capitalised word may
    // be a foreign name, and a lowercase `-i` adjective may be derived from one (`ORT-NAME-022`:
    // `yogyakartai`), so neither is held to it.
    let mut first = letters(s).take(2);
    let opening = !starts_upper(s)
        && match (first.next(), first.next()) {
            (Some(a), Some(b)) => {
                (a.letter == b.letter && !a.letter.is_vowel())
                    || (a.text == "y" && !s.ends_with('i'))
            }
            (Some(a), None) => a.text == "y",
            _ => false,
        };
    n == s.len() && vowels > 0 && !opening
}

#[cfg(test)]
mod tests {
    use picur_morph_lexicon::{Entry, Pos, Trait, TraitSet};

    use super::{
        Definite, Mood, Morph, Number, Person, Stacks, Tense, Verdict, analyse, looks_hungarian,
        stable_prefix,
    };

    fn e(lemma: &str, traits: &[Trait]) -> Entry {
        let mut x = Entry::new(lemma, Pos::Noun);
        x.traits = TraitSet::from_iter_of(traits.iter().copied());
        x
    }

    #[test]
    fn a_guess_reads_the_suffix_off_an_unlisted_stem() {
        use super::{Analysis, Stacks, guess};
        use crate::features::Case;
        let stacks = Stacks::from(vec![
            vec![],
            vec![Morph::Case(Case::Dat)],
            vec![Morph::Case(Case::Acc)],
            vec![Morph::Plural, Morph::Case(Case::Ine)],
            vec![Morph::Verbal {
                mood: Mood::Ind,
                tense: Tense::Past,
                person: Person::Third,
                number: Number::Plur,
                definite: Definite::Ind,
            }],
        ]);
        let g = guess("Lodernek", &stacks);
        assert!(
            g.iter()
                .any(|a| &*a.lemma == "Loder" && a.stack == [Morph::Case(Case::Dat)]),
            "{g:?}"
        );
        let g = guess("Chilcote-ot", &stacks);
        assert!(
            g.iter()
                .any(|a| &*a.lemma == "Chilcote" && a.stack == [Morph::Case(Case::Acc)]),
            "{g:?}"
        );
        let g = guess("sztrimingeltek", &stacks);
        assert!(
            g.iter()
                .any(|a| &*a.lemma == "sztrimingel" && a.pos == Pos::Verb),
            "{g:?}"
        );
        assert_eq!(guess("Loder", &stacks), [] as [Analysis; 0]);
    }

    #[test]
    fn non_alternating_stems_are_their_own_prefix() {
        // The final letter is excluded because a seam can rewrite it (`lány` -> `lánnyal`).
        assert_eq!(stable_prefix(&e("ház", &[])), "há");
        assert_eq!(stable_prefix(&e("lány", &[])), "lá");
        assert_eq!(stable_prefix(&e("gyümölcs", &[])), "gyüm");
    }

    #[test]
    fn alternating_stems_cut_at_the_last_vowel() {
        assert_eq!(stable_prefix(&e("bokor", &[Trait::Epenthetic])), "bok");
        assert_eq!(stable_prefix(&e("kéz", &[Trait::Shortening])), "k");
        assert_eq!(stable_prefix(&e("ló", &[Trait::VStem])), "l");
    }

    #[test]
    fn prefixes_never_end_inside_a_digraph() {
        let entry = e("gyümölcs", &[Trait::Epenthetic]);
        let p = stable_prefix(&entry);
        assert!(
            "gyümölcs".starts_with(p) && !p.ends_with('c'),
            "cut inside a digraph: {p}"
        );
    }

    #[test]
    fn recognises_hungarian_shapes() {
        assert!(looks_hungarian("házak"));
        assert!(looks_hungarian("sztrimingelhetetlenség"));
        assert!(!looks_hungarian("xyz123"));
        assert!(!looks_hungarian(""));
        assert!(!looks_hungarian("bcd"));
    }

    #[test]
    fn vrb_imp_008_the_long_second_singular_is_a_correct_form() {
        // Both second-singular cells have a short and a long form, equally correct. Only the
        // short one is generated, so the long one is reached through the variant.
        for (form, lemma, definite) in [
            ("adjál", "ad", Definite::Ind),
            ("nézzél", "néz", Definite::Ind),
            ("mossál", "mos", Definite::Ind),
            ("tanítsál", "tanít", Definite::Ind),
            ("írjál", "ír", Definite::Ind),
            ("menjél", "megy", Definite::Ind),
            ("higgyél", "hisz", Definite::Ind),
            ("várjad", "vár", Definite::Def),
            ("nézzed", "néz", Definite::Def),
        ] {
            let stacks = Stacks::from(vec![vec![Morph::Verbal {
                mood: Mood::Imp,
                tense: Tense::Pres,
                person: Person::Second,
                number: Number::Sing,
                definite,
            }]]);
            let Verdict::Known(readings) = analyse(form, &stacks) else {
                panic!("{form} not analysed");
            };
            assert!(
                readings.iter().any(|r| &*r.lemma == lemma),
                "{form} gave {readings:?}"
            );
        }
    }
}
