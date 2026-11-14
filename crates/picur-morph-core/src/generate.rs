//! Generation: the single source of truth.
//!
//! Analysis is search over this, verified by regenerating and comparing, so every morphological
//! claim the library makes is made here once.
//!
//! See `docs/design/02-generation.md`.

use picur_morph_lexicon::{Entry, Pos, Role, Trait, TraitSet, lists, pairs, words};
use picur_morph_ortho::case::{lower_initial, starts_upper};
use picur_morph_ortho::{Harmony, Letter, harmony_of, letters};

use crate::alternation::alternant;
use crate::derivation::DerId;
use crate::features::{Case, Definite, Mood, Morph, Number, Person, Tense};
use crate::stem::{Coda, coda, linking_vowel};
use crate::suffix::{Seg, Suffix, case_suffix};

/// Why generation could not produce a form.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum GenError {
    /// The feature stack is not yet supported by the engine.
    Unsupported(&'static str),
    /// The paradigm has no such cell, so no form exists to generate.
    ///
    /// Distinct from [`GenError::Unsupported`]: this is a fact about the language, not a gap in the
    /// implementation, and it stays an error however complete the engine becomes.
    NoSuchCell(&'static str),
}

impl std::fmt::Display for GenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported(what) => write!(f, "not yet implemented: {what}"),
            Self::NoSuchCell(what) => write!(f, "no such cell in the paradigm: {what}"),
        }
    }
}

impl std::error::Error for GenError {}

/// The harmony class of a stem, letting a lexical trait override what the spelling computes.
///
/// Only antiharmonic stems carry a trait, because only they contradict `harmony_of`.
///
/// Rules: `PH-HARM-011`
#[must_use]
pub fn harmony(stem: &str, entry: &Entry) -> Harmony {
    if entry.has(Trait::Back) {
        Harmony::Back
    } else if entry.has(Trait::FrontRounded) {
        Harmony::FrontRounded
    } else if entry.has(Trait::Front) {
        Harmony::Front
    } else {
        let domain =
            opaque_front(stem).unwrap_or_else(|| last_member(harmony_base(stem, entry), entry));
        // PH-HARM-019: the relational `-i` is transparent, so an `-i` adjective takes its base's
        // backness and its own rounding: `athéniak`, `párizsiak`, `maiak`; every two-neutral `-i`
        // adjective the corpus attests suffixed is back.
        if entry.pos == Pos::Adjective
            && let Some(base) = domain.strip_suffix('i')
            && harmony_of(base) == Harmony::Back
        {
            Harmony::Back
        } else {
            harmony_of(domain)
        }
    }
}

/// Whether a lemma is the potential participle of `DER-ADJ-019`, by its ending.
///
/// Rules: `DER-ADJ-019`
fn potential_participle(lemma: &str) -> bool {
    lemma.ends_with("ható") || lemma.ends_with("hető")
}

/// The opaque front suffix a stem ends in, which is then the whole harmony domain.
///
/// `DER-MODEL-008`'s `F!` class is invariant and opaque: a suffix after it is front whatever the
/// base does, so `válasz` plus `-képp` plus the superessive is `válaszképpen`. The members of the
/// class that are semi-words already form their own domain as compound members.
///
/// Rules: `DER-MODEL-008`, `NOM-CASE-020`
fn opaque_front(stem: &str) -> Option<&str> {
    words(Role::OpaqueFrontEnding)
        .iter()
        .find(|e| stem.ends_with(e.as_ref()))
        .map(|e| {
            let at = stem.len() - e.as_ref().len();
            &stem[at..]
        })
}

/// The lexical trait that pins a harmony class, for a host whose class is already decided.
///
/// Rules: `PH-HARM-001`
const fn harmony_trait(h: Harmony) -> Trait {
    match h {
        Harmony::Back => Trait::Back,
        Harmony::Front => Trait::Front,
        Harmony::FrontRounded => Trait::FrontRounded,
    }
}

/// Whether a lemma is written as an acronym: two or more letters, every one a capital.
///
/// Rules: `ORT-ABBR-009`
pub(crate) fn is_acronym(lemma: &str) -> bool {
    let mut alpha = lemma.chars().filter(|c| c.is_alphabetic()).peekable();
    let mut seen = 0usize;
    for c in &mut alpha {
        if !c.is_uppercase() {
            return false;
        }
        seen += 1;
    }
    seen > 1
}

/// The string the computed harmony class is read off.
///
/// Normally the stem as it stands, so that a suffix already attached can carry the rounding on
/// (`PH-HARM-005`). The exception is a bare alternant that lost a vowel: the drop can take away
/// the very vowel that fixes the class, and an alternation never changes it. `titok` binds to
/// `titk`, whose only remaining vowel is a neutral `i`, which read alone would make the class
/// front and the forms `titkek` and `titket`; `bokr` keeps its `o` and shows nothing. The u-stem
/// drop does the same: `ifjú` binds to `ifj`, leaving only the neutral `i`, for `ifjak`.
///
/// The test is length: an alternant is shorter than its lemma, and anything with a suffix on it is
/// longer. A verb's lemma is read without its `-ik`, whose neutral vowel is no part of the stem
/// (`fürdik` gives `fürdöm`).
///
/// Rules: `PH-HARM-001`, `NOM-STEM-006`, `NOM-STEM-012`
fn harmony_base<'a>(stem: &'a str, entry: &'a Entry) -> &'a str {
    let drops_a_vowel = entry.has(Trait::Epenthetic) || entry.has(Trait::UStem);
    let lemma = if entry.pos == Pos::Verb {
        crate::verbal::present_stem(&entry.lemma)
    } else {
        &entry.lemma
    };
    if drops_a_vowel && stem.len() < lemma.len() {
        lemma
    } else {
        stem
    }
}

/// The part of the stem a suffix harmonises with.
///
/// `DER-COMP-008`: each member of a compound is its own harmony domain and the suffix agrees with
/// the last member only. Reading the whole word instead gets `aprópénz` wrong, because its last
/// non-neutral vowel is the `ó` of `apró` while the suffix follows the `é` of `pénz`:
/// `aprópénzek` 346 against `aprópénzok` 0 `[WC2]`.
///
/// Rules: `DER-COMP-008`
fn last_member<'a>(stem: &'a str, entry: &Entry) -> &'a str {
    let Some(&seam) = entry.parts.last() else {
        return stem;
    };
    let seam = seam as usize;
    // The stem may already carry a suffix, and it may be a bound alternant shorter than the lemma,
    // so the offset is only usable while it still lies inside the string on a boundary.
    if seam < stem.len() && stem.is_char_boundary(seam) {
        &stem[seam..]
    } else {
        stem
    }
}

/// `stem` with a final short `o`/`ö` lengthened.
///
/// `ORT-FOR-010`: a word-final short `o` or `ö` is a foreign ending pronounced long, and it
/// writes long before every suffix (`Oslóban`, `oslói`, `malmői`).
pub(crate) fn lengthen_final_o(stem: &str) -> String {
    let mut out = stem.to_owned();
    match out.pop() {
        Some('o') => out.push('ó'),
        Some('ö') => out.push('ő'),
        Some(c) => out.push(c),
        None => {}
    }
    out
}

/// Lengthens a final `a`, `e`, `o` or `ö` to `á`, `é`, `ó` or `ő`.
///
/// `NOM-LENG-005` adds `o` and `ö` to `NOM-LENG-001`'s pair for foreign spellings, and needs no
/// condition beyond the letter: `PH-SYL-013` says no native content stem ends in a short `o` or
/// `ö` at all, so a stem that does is a loan. Measured: `chicagóban` 10641 against `chicagoban`
/// 1810, `torontóban` 8000 against 639, `oslóban` 4150 against 614, and the corpus lemmatises all
/// three on the short form `[WC2]`.
///
/// Rules: `NOM-LENG-001`, `NOM-LENG-005`
pub fn lengthen_final(stem: &str) -> String {
    let mut out = stem.to_owned();
    lengthen_final_in_place(&mut out);
    out
}

/// [`lengthen_final`] on a buffer the caller owns.
fn lengthen_final_in_place(stem: &mut String) {
    for (short, long) in [('a', 'á'), ('e', 'é'), ('o', 'ó'), ('ö', 'ő')] {
        if stem.ends_with(short) {
            stem.pop();
            stem.push(long);
            break;
        }
    }
}

/// The final letter of a stem, if it is a consonant, with its byte offset.
///
/// Returns the offset rather than the length so the caller can slice safely: summing span lengths
/// drifts from the real position whenever a word contains a character outside the alphabet.
///
/// `PH-G2P-005`: a word-final `y` that forms no digraph is [i], so it hosts no assimilation
/// (`Adyval`, `whiskyvel`, not `*Adyyal`). Where the spelling is `ly`, `ny`, `ty` or `gy` the
/// letter parser has already read the digraph, which is the lexical half of that rule.
///
/// Rules: `PH-G2P-005`
fn final_consonant(stem: &str) -> Option<(Letter, usize)> {
    let span = letters(stem).last()?;
    if span.at + span.text.len() != stem.len() || span.letter == Letter::Y {
        return None;
    }
    span.letter.is_consonant().then_some((span.letter, span.at))
}

/// The consonant a name-final pre-1832 letter group is pronounced as, if any.
///
/// `PH-G2P-004` splits the table: the groups of `ArchaicNameFinal` hold for any name, and the ones
/// the table marks ambiguous need the name flagged, so that `Madách` gives `Madáchcsá` and
/// `Zürich` keeps `Zürichhel`.
///
/// Rules: `PH-G2P-004`, `ORT-FOR-013`
fn archaic_final(stem: &str, marked_spelling: bool) -> Option<&'static str> {
    let roles = [Role::ArchaicNameFinal]
        .into_iter()
        .chain(marked_spelling.then_some(Role::ArchaicNameFinalMarked));
    roles
        .flat_map(|role| pairs(role).iter())
        .find(|(written, _)| stem.ends_with(written.as_ref()))
        .map(|(_, sound)| sound.as_ref())
}

/// Joins stem, linking vowel and realised suffix, applying seam phonology, and reports where the
/// written suffix begins.
///
/// The main effect is `-vAl`/`-vÁ` assimilation: after a vowel the `v` stays (`hajóval`), and after
/// a consonant it becomes that consonant, written with simplified doubling. For a digraph that means
/// repeating only its first character, so `ész` gives `ésszel` and not `*észszel`.
///
/// The returned offset is not always `stem.len()`: at an assimilating seam the doubled consonant is
/// shared, and only the suffix's half of it is written after a digit's hyphen (`húsz` inflects as
/// `hússzal`, but the digit takes `20-szal`).
///
/// Rules: `NOM-CASE-003`, `WS-DIG-002`, `PND-NUMINF-010`
fn join_at(mut stem: String, entry: &Entry, suffix: &Suffix, tail: &str) -> (String, usize) {
    let long_final = doubled_final(&stem);
    if matches!(suffix.template.first(), Some(Seg::AssimV))
        && let Some((letter, at)) = final_consonant(&stem)
    {
        // NOM-STEM-004: where the norm writes the final `h` as silent the stem is vowel-final for
        // these two suffixes, so the `v` stays: `csehvel`, `pléhvé`.
        if letter == Letter::H && entry.has(Trait::SilentH) {
            return (format!("{stem}{tail}"), stem.len());
        }
        // The stem already writes the geminate, so the assimilated `v` would be a third letter:
        // `orr` + `-vAl` is `orral`, not `*orrral`.
        if long_final == Some(letter) {
            let rest = tail.strip_prefix('v').unwrap_or(tail);
            let at = stem.len() - letter.as_str().len();
            return (format!("{stem}{rest}"), at);
        }
        // PH-G2P-007: `x` is [ksz], so the `v` assimilates to `sz` and the `x` stays: `fixszel`.
        if letter == Letter::X {
            let rest = tail.strip_prefix('v').unwrap_or(tail);
            return (format!("{stem}sz{rest}"), stem.len());
        }
        // ORT-FOR-013, PH-G2P-004, PH-G2P-008: where a word's last sound is not the letter that
        // writes it, the `v` assimilates to the sound and the spelling stays. The sound comes from
        // the entry where the lexicon records one (`hertzcel`, `Wellsszel`) and from the archaic
        // table otherwise (`Kossuthtal`, `Babitscsal`, `Madáchcsá`).
        if let Some(sound) = entry
            .last_sound
            .as_deref()
            .filter(|_| !entry.has(Trait::Spoken))
            .or_else(|| {
                starts_upper(&stem)
                    .then(|| archaic_final(&stem, entry.has(Trait::ArchaicSpelling)))
                    .flatten()
            })
        {
            let rest = tail.strip_prefix('v').unwrap_or(tail);
            return (format!("{stem}{sound}{rest}"), stem.len());
        }
        let text = letter.as_str();
        let first = text.chars().next().map(String::from).unwrap_or_default();
        let trunk = &stem[..at];
        let rest = tail.strip_prefix('v').unwrap_or(tail);
        return (
            format!("{trunk}{first}{text}{rest}"),
            trunk.len() + first.len(),
        );
    }
    // VRB-PRES-003: the definite `j` fuses with a stem-final sibilant, `olvas` + `-jUk` giving
    // `olvassuk`. After any other consonant the `j` is simply written (`látja`).
    if matches!(suffix.template.first(), Some(Seg::AssimJ))
        && let Some(fused) = crate::imperative::sibilant_j(&stem)
    {
        let rest = tail.strip_prefix('j').unwrap_or(tail);
        return (format!("{fused}{rest}"), stem.len());
    }
    // WS-DIG-002: two adjacent copies of a multi-character letter are written with the first
    // character doubled, so `húsz` + `-szOr` is `hússzor` and not `*húszszor`. The AssimV branch
    // above reaches the same spelling for `-vAl`, but by assimilation rather than by identity.
    if let Some((letter, at)) = final_consonant(&stem)
        && letter.as_str().chars().count() > 1
        && tail.starts_with(letter.as_str())
        && let Some(first) = letter.as_str().chars().next()
    {
        let trunk = &stem[..at];
        return (
            format!("{trunk}{first}{tail}"),
            trunk.len() + first.len_utf8(),
        );
    }
    // ORT-ASSIM-017: three identical consonant letters from suffixation are written as two. The
    // rule counts letters, not characters, so it also covers `fütty` + `-tyel` -> `füttyel`.
    let tail = match long_final {
        Some(l) => tail.strip_prefix(l.as_str()).unwrap_or(tail),
        None => tail,
    };
    let at = stem.len();
    stem.push_str(tail);
    (stem, at)
}

/// The consonant a stem ends a written geminate in, if it does.
///
/// [`letters`] resolves a simplified doubled digraph, so `fütty` ends in two `ty` letters and this
/// reports `ty` rather than `y`.
///
/// Rules: `WS-DIG-002`, `ORT-ASSIM-017`
pub(crate) fn doubled_final(stem: &str) -> Option<Letter> {
    let (mut prev, mut last) = (None, None);
    for span in letters(stem) {
        prev = last;
        last = Some(span.letter);
    }
    let (a, b) = (prev?, last?);
    (a == b && a.is_consonant()).then_some(a)
}

/// Attaches one suffix to a base form.
///
/// Order matters and is not arbitrary: the linking vowel depends on the stem's final cluster, and
/// harmony is read off the stem **plus** the link, so both precede archiphoneme resolution.
///
/// Rules: `NOM-MODEL-004`, `NOM-LOW-001`, `PH-HARM-002`
#[must_use]
pub fn attach(base: &str, entry: &Entry, suffix: &Suffix, lowering: bool) -> String {
    attach_parts(base, entry, suffix, lowering).0
}

/// `ez` and `az` before a consonant-initial suffix: the `z` assimilates and the suffix's first
/// consonant is written double.
///
/// `ez` + `-bAn` is `ebben`, `az` + `-hOz` is `ahhoz`. Three suffix shapes are exempt and each for
/// its own reason: a realised linking vowel makes the suffix vowel-initial (`ezek`, `ezen`), the
/// accusative `-t` simply does not trigger it (`ezt`, not `*ett`), and `-vAl` is already handled by
/// the general `v` assimilation, which arrives at `ezzel` from the other direction.
///
/// Rules: `PND-DEM-002`, `PND-DEM-003`, `PND-DEM-004`
fn demonstrative_z(
    base: &str,
    entry: &Entry,
    suffix: &Suffix,
    tail: &str,
) -> Option<(String, usize)> {
    if !entry.has(Trait::DemonstrativeZ) || suffix.bare_coda {
        return None;
    }
    if matches!(suffix.template.first(), Some(Seg::AssimV)) {
        return None;
    }
    let trunk = base.strip_suffix('z')?;
    let first = letters(tail).next()?;
    if !first.letter.is_consonant() || first.at != 0 {
        return None;
    }
    // A digraph doubles by repeating only its first character, as everywhere else.
    let text = first.letter.as_str();
    let lead = text.chars().next()?;
    let at = trunk.len() + lead.len_utf8();
    Some((format!("{trunk}{lead}{tail}"), at))
}

/// Attaches, and also reports where the written suffix begins.
///
/// `PND-NUMINF-010` writes exactly this slice after a digit's hyphen, which is why it is not simply
/// the form minus the lemma: a stem change is not written (`ezer` inflects as `ezret`, but the digit
/// takes `1000-et`), and an assimilated consonant is shared between the two.
///
/// Rules: `NOM-MODEL-004`, `PND-NUMINF-010`
#[must_use]
pub fn attach_parts(base: &str, entry: &Entry, suffix: &Suffix, lowering: bool) -> (String, usize) {
    attach_owned(with_room(base), entry, suffix, lowering)
}

/// [`attach_parts`] on a buffer the caller gives up, which the suffix is written into.
///
/// Attaching a suffix is the checker's innermost loop, and building the form in the buffer the
/// stem arrived in is what keeps a stack of suffixes to one allocation.
pub(crate) fn attach_owned(
    mut stem: String,
    entry: &Entry,
    suffix: &Suffix,
    lowering: bool,
) -> (String, usize) {
    let link = linking_vowel(&stem, entry, suffix, lowering);
    let h = harmony(&stem, entry);
    let tail = suffix.realise_with_link(h, link);
    if let Some(out) = demonstrative_z(&stem, entry, suffix, &tail) {
        return out;
    }
    if let Some(out) = name_hyphen(&stem, entry, suffix, &tail) {
        return out;
    }
    if suffix.lengthens && link.is_none() && !keeps_short_e(&stem) {
        lengthen_final_in_place(&mut stem);
    }
    join_at(stem, entry, suffix, &tail)
}

/// Whether a stem is one syllable ending in short `e`, which keeps the vowel short (`PH-LEN-003`).
///
/// No native content word has that shape: the monosyllables in `e` are the function words (`de`,
/// `ne`, `le`), so a noun of the shape is a loan taken with their prosody, and `Che` gives `Chet`
/// where the two-syllable `Ocse` gives `Ocsét`.
///
/// Rules: `PH-LEN-003`
fn keeps_short_e(stem: &str) -> bool {
    stem.ends_with('e') && letters(stem).filter(|s| s.letter.is_vowel()).count() == 1
}

/// A proper name ending in a doubled consonant letter, before a suffix starting with the same
/// letter, keeps its base form recoverable behind a hyphen (`ORT-NAME-012`).
///
/// `Kiss` plus `-vel` is `Kiss-sel` and `Bonn` plus `-nak` is `Bonn-nak`, where a common noun
/// simplifies the three letters to two (`ORT-ASSIM-017`: `toll` plus `-val` is `tollal`). The
/// lexicon carries no proper-name category, so the capital initial is the name.
///
/// Rules: `ORT-NAME-012`, `ORT-FOR-013`
fn name_hyphen(stem: &str, entry: &Entry, suffix: &Suffix, tail: &str) -> Option<(String, usize)> {
    if !starts_upper(&entry.lemma) {
        return None;
    }
    let letter = doubled_final(stem)?;
    let spoken = if matches!(suffix.template.first(), Some(Seg::AssimV)) {
        format!(
            "{}{}",
            letter.as_str(),
            tail.strip_prefix('v').unwrap_or(tail)
        )
    } else {
        tail.to_owned()
    };
    if !spoken.starts_with(letter.as_str()) {
        return None;
    }
    let at = stem.len() + 1;
    Some((format!("{stem}-{spoken}"), at))
}

/// A foreign spelling suffixed by its pronunciation, after a hyphen (`ORT-FOR-012`).
///
/// The stack is generated on a spoken stem and what that stem gained is written after the
/// spelling and a hyphen: `kembridzs` plus `-bAn` is `kembridzsben`, so `Cambridge` plus `-bAn`
/// is `Cambridge-ben`. A derivative is lowercase, as the rule writes `cambridge-i` and
/// `voltaire-es`.
///
/// The suffix consumes three facts of the pronunciation and no more: the harmony class, whether
/// the last sound is a vowel, and which consonant it is otherwise. So the spoken stem is either
/// the full pronunciation in `alt`, or built from those facts alone: the harmony marking the entry
/// carries (its letters otherwise), and `alt` as a single consonant letter or absent for a vowel.
/// `League` with `Front` and `alt` `g` gives `League-ben`, `League-et`, `League-gel`; `Chelsea`
/// with `Front` and no `alt` gives `Chelsea-ben`, `Chelsea-t`, `Chelsea-vel`. Attested forms show
/// exactly those facts, so a corpus can fill them without anyone inventing a pronunciation.
///
/// A derivation reports no seam, and one that lengthens the spoken final vowel (`gőte` to
/// `gőtés`) is told apart by the lengthened stem, so `Goethe` gives `goethés` (`ORT-FOR-010`).
fn spoken_form(entry: &Entry, stack: &[Morph]) -> Result<(String, Option<usize>), GenError> {
    let probe = spoken_probe(entry).ok_or(GenError::Unsupported(
        "the pronunciation facts a foreign name's suffix follows",
    ))?;
    let (form, seam) = generate_seam(&probe, stack)?;
    let long_stem = lengthen_final(&probe.lemma);
    let tail = match seam {
        Some(at) => &form[at..],
        None => form
            .strip_prefix(probe.lemma.as_ref())
            .or_else(|| form.strip_prefix(long_stem.as_str()))
            .ok_or(GenError::Unsupported(
                "a form that rewrites the spoken stem",
            ))?,
    };
    if tail.is_empty() {
        return Ok((entry.lemma.to_string(), None));
    }
    let name = if matches!(stack.first(), Some(Morph::Derive(_))) {
        lower_initial(&entry.lemma)
    } else {
        entry.lemma.to_string()
    };
    // ORT-FOR-012: the hyphen marks a final letter that is silent or spells another sound. A name
    // whose last letter is the vowel it is pronounced with takes the suffix solid, lengthened as
    // the pronunciation was (`Goethe` [gőte] gives `Goethével`).
    let same_vowel = entry.alt.is_some()
        && name
            .chars()
            .last()
            .zip(probe.lemma.chars().last())
            .is_some_and(|(a, b)| a == b && "aeoö".contains(a));
    if same_vowel {
        let lengthened = !form.starts_with(probe.lemma.as_ref());
        let base = if lengthened {
            lengthen_final(&name)
        } else {
            name
        };
        let at = base.len();
        return Ok((format!("{base}{tail}"), Some(at)));
    }
    let at = name.len() + 1;
    Ok((format!("{name}-{tail}"), Some(at)))
}

/// The stem a spoken entry's suffix is computed on: its pronunciation in `alt`, or a probe
/// carrying only the harmony class and the last sound.
///
/// `None` when the entry records neither: a name marked `Spoken` with no facts says only that
/// its suffix follows a pronunciation nobody has written down, and that is a state worth keeping.
/// The bare name is known, a hyphenated suffix is accepted under any reading, and nothing is
/// spelled by the letters the mark says not to trust.
fn spoken_probe(entry: &Entry) -> Option<Entry> {
    if let Some(alt) = entry.alt.as_deref() {
        let mut probe = Entry::new(alt, entry.pos);
        probe.traits = entry.traits.without(Trait::Spoken);
        return Some(probe);
    }
    if !entry
        .traits
        .has_any([Trait::Back, Trait::Front, Trait::FrontRounded])
    {
        return None;
    }
    let last = entry
        .last_sound
        .as_deref()
        .and_then(|s| letters(s).next())
        .map(|span| span.letter);
    let h = harmony(&entry.lemma, entry);
    let mut probe = crate::acronym::probe(h, last);
    probe.pos = entry.pos;
    Some(probe)
}

/// Whether a capitalised lemma ends in one of the letter groups `ORT-FOR-012` names as not read
/// the Hungarian way: French `eau`, `ieu`, `eux`, `aux`, `oux`, `ault`, English `ow`, `gh`, `ee`,
/// `oo`.
///
/// The rule makes the decision lexical, and this covers only the part of it that the letters
/// settle. A silent final `e` is the other part, and it is not in the spelling: `Goethe` says its
/// `e` and `Cambridge` does not. Such a name is suffixed by its letters until a lexical fact says
/// otherwise; the groups here are refused rather than suffixed, because `Glasgowban` is a wrong
/// answer where no answer is the honest one.
fn unusual_final(lemma: &str) -> bool {
    const GROUPS: [&str; 10] = [
        "eau", "ieu", "eux", "aux", "oux", "ault", "ow", "gh", "ee", "oo",
    ];
    GROUPS.iter().any(|g| lemma.ends_with(g))
}

/// `text` in a buffer with room for a suffix.
pub(crate) fn with_room(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    out.push_str(text);
    out
}

/// Whether a feature has any place in this category's paradigm.
///
/// Nominal inflection is for nominals (`NOM-MODEL-001`) and verbal inflection for verbs
/// (`VRB-MODEL-001`). The categories that have neither, such as conjunctions and determiners, are
/// invariant and inflect for nothing.
fn applies_to(morph: Morph, pos: Pos) -> bool {
    match morph {
        Morph::Case(_)
        | Morph::Plural
        | Morph::Possessive { .. }
        | Morph::PossessedPlural
        | Morph::Anaphoric
        | Morph::Associative => pos.is_nominal(),
        // PND-NUMINF-005 puts the collective on numerals and quantity words; the multiplicative is
        // a numeral suffix throughout.
        Morph::Collective => matches!(
            pos,
            Pos::Numeral | Pos::Pronoun | Pos::Determiner | Pos::Adjective
        ),
        // The quantifier table of `PND-QUANT-001` lists the two together (`sokan`/`sokszor`,
        // `néhányan`/`néhányszor`), and our categories put those words with the adjectives, so the
        // two suffixes take the same inputs.
        Morph::Multiplicative => matches!(
            pos,
            Pos::Numeral | Pos::Pronoun | Pos::Determiner | Pos::Adjective
        ),
        Morph::Derive(_) => true,
        Morph::Comparative | Morph::Superlative | Morph::Excessive => {
            matches!(pos, Pos::Adjective | Pos::Adverb)
        }
        Morph::Potential
        | Morph::Infinitive
        | Morph::PresentParticiple
        | Morph::PastParticiple
        | Morph::Converb
        | Morph::FutureParticiple
        | Morph::Verbal { .. } => pos == Pos::Verb,
    }
}

/// The lemmas whose irregular comparative is `form`.
///
/// Analysis needs the reverse direction: `szép` gives `szebb`, so no prefix of `szebb` resembles
/// the lemma and the stem index cannot find it. Several lemmas may share one comparative, as
/// `kicsi`, `kis` and `kicsiny` all give `kisebb`.
///
/// Rules: `NOM-ADJ-005`
#[must_use]
pub fn comparative_bases(form: &str) -> Vec<&'static str> {
    pairs(Role::IrregularComparative)
        .iter()
        .filter(|(_, c)| c.as_ref() == form)
        .map(|(w, _)| w.as_ref())
        .collect()
}

/// The collective or multiplicative of the numeral 2, which uses a stem of its own.
///
/// `PND-NUMINF-003` gives three stems chosen by what follows: `két` before the multiplicative,
/// `kett-` before a vowel-initial derivational suffix, `kettő` elsewhere. Nothing derives one from
/// another, so the choice is stated.
///
/// Rules: `PND-NUMINF-003`
fn numeral_two(entry: &Entry, morph: Morph, lowering: bool) -> Option<String> {
    if entry.lemma.as_ref() != "kettő" {
        return None;
    }
    let (stem, suffix) = if matches!(morph, Morph::Multiplicative) {
        ("két", multiplicative())
    } else {
        ("kett", collective())
    };
    Some(attach(stem, entry, &suffix, lowering))
}

/// Rejects a stack this entry has no paradigm for, before any of it is built.
///
/// Two checks, each from its own rule. A paradigm belongs to a category (`NOM-MODEL-001`,
/// `VRB-MODEL-001`): without this a conjunction takes a case suffix, and `s` plus the inessive
/// gives `sben`, which then lets `házsben` parse as a compound. And `ORT-ABBR-009` attaches a
/// suffix to an abbreviation, a symbol or a bare letter only after a hyphen, so those have no
/// inflected forms at all: the letter `h` was producing `hhez`.
/// The entries that take no suffix directly, whatever the stack.
fn closed_paradigm(entry: &Entry, stack: &[Morph]) -> Result<(), GenError> {
    if stack.is_empty() {
        return Ok(());
    }
    // `NOM-ADJ-005`: a suppletive comparative is already graded, so it is not graded again:
    // `több` has `legtöbb`, through `sok`, and no `*többebb`.
    if stack.iter().any(|m| {
        matches!(
            m,
            Morph::Comparative | Morph::Superlative | Morph::Excessive
        )
    }) && !comparative_bases(&entry.lemma).is_empty()
    {
        return Err(GenError::NoSuchCell(
            "a suppletive comparative is not graded again",
        ));
    }
    // `ORT-ABBR-009`: an acronym writes its suffix after a hyphen, so no suffix attaches to it
    // directly and `MÁVnál` is not a form of `MÁV` at all. The shape decides it and no lexical
    // input is needed: a lemma written wholly in capitals is an acronym. `crate::acronym` owns the
    // hyphenated form, which is where the pronunciation-dependent allomorph lives.
    if is_acronym(&entry.lemma) {
        return Err(GenError::Unsupported(
            "an acronym takes its suffix after a hyphen",
        ));
    }
    // `PND-PERS-005`: a personal pronoun has the local cases by suppletion (`crate::pronoun`) and
    // nothing else: `*énig`, `*énként`, `*énné`.
    if entry.pos == Pos::Pronoun && crate::pronoun::is_personal(&entry.lemma) {
        return Err(GenError::NoSuchCell(
            "a personal pronoun inflects only by suppletion",
        ));
    }
    Ok(())
}

/// Refuses a suffix on an attributive-only form.
///
/// `NOM-ADJ-002`: `kis` stands before its head and nowhere else, so the predicative `kicsi`
/// carries the paradigm (`kicsik`, `a kicsit`). The gradation is shared, so a graded stack is
/// admitted and inflects as `NOM-STEM-017` has it (`kisebbeket`).
///
/// Rules: `NOM-ADJ-002`
fn attributive_only(entry: &Entry, stack: &[Morph]) -> Result<(), GenError> {
    if entry.has(Trait::AttributiveOnly)
        && !matches!(
            stack.first(),
            None | Some(Morph::Comparative | Morph::Superlative | Morph::Excessive)
        )
    {
        return Err(GenError::NoSuchCell("this form is attributive only"));
    }
    Ok(())
}

/// Whether `-é` or `-ék` is among `before`, which `-kor` may not follow.
///
/// Rules: `NOM-MODEL-003`
fn follows_anaphoric(before: &[Morph]) -> bool {
    before
        .iter()
        .any(|m| matches!(m, Morph::Anaphoric | Morph::Associative))
}

/// Refuses a derivational suffix after the derivations `before` it.
///
/// `DER-ADJ-006`: a name's `-i` adjective takes `-(O)s` only where it is the name itself, an
/// `i`-final one (`petőfies`; `*kossuthias`). `DER-VERB-012`: `-(O)skOdik` is a noun's, and an
/// adjective takes `-kOdik` directly (`okoskodik`, not `*okosaskodik`).
fn derivation_chain(entry: &Entry, before: &[Morph], id: DerId) -> Result<(), GenError> {
    match id {
        DerId::Os
            if before.last() == Some(&Morph::Derive(DerId::I))
                && starts_upper(&entry.lemma)
                && !entry.lemma.ends_with(['i', 'y']) =>
        {
            Err(GenError::NoSuchCell(
                "a consonant-final name's -i adjective takes no -(O)s",
            ))
        }
        DerId::Kodik if before == [Morph::Derive(DerId::Os)] && entry.pos == Pos::Adjective => {
            Err(GenError::NoSuchCell("an adjective takes -kOdik directly"))
        }
        _ => Ok(()),
    }
}

fn admissible(entry: &Entry, stack: &[Morph]) -> Result<(), GenError> {
    closed_paradigm(entry, stack)?;
    attributive_only(entry, stack)?;
    // VRB-INF-002: the infinitive takes possessive-like person suffixes, which is the one place a
    // verb carries a possessive (`tudnom`, `lennie`).
    let personal_infinitive = matches!(
        stack,
        [Morph::Infinitive, Morph::Possessive { .. }] if entry.pos == Pos::Verb
    );
    // `PND-DEM-001`, `PND-INT-001`: a temporal in `-kor` takes the sublative (`akkorra`,
    // `mikorra`, `amikorra`), the one case an adverb takes.
    if entry.pos == Pos::Adverb && stack == [Morph::Case(Case::Sbl)] && entry.lemma.ends_with("kor")
    {
        return Ok(());
    }
    // `DER-MODEL-003`: a derivational suffix assigns the output's category, so the rest of the
    // stack is judged against that and not against the lemma's: `vágyik` plus `-Ó` is a noun, so
    // `vágyók` is its plural although a verb has none.
    let mut pos = entry.pos;
    // `NOM-CASE-001`'s case-like suffixes (`-nként`, `-stul`, `-képp`, `-nta`) close a bare
    // singular; `-kor` follows a plural and a possessive (`ünnepekkor`, `érkezésekor`) but no
    // `-é`/`-ék`.
    let mut numbered = false;
    let mut possessed = false;
    for (i, morph) in stack.iter().enumerate() {
        if personal_infinitive {
            continue;
        }
        match morph {
            // `PND-DEM-005`, `PND-REFL-004`: a quantity pronoun has no plural (`*annyik`) and the
            // reciprocal no possessive (`*egymásunk`); the lexicon states which.
            Morph::Plural if pos == Pos::Pronoun && entry.has(Trait::SingularOnly) => {
                return Err(GenError::NoSuchCell("this pronoun has no plural"));
            }
            Morph::Possessive { .. } if pos == Pos::Pronoun && entry.has(Trait::NoPossessive) => {
                return Err(GenError::NoSuchCell("this pronoun has no possessive"));
            }
            Morph::Plural | Morph::Anaphoric | Morph::Associative => numbered = true,
            Morph::Possessive { .. } | Morph::PossessedPlural => possessed = true,
            Morph::Case(
                Case::Sociativus
                | Case::Distributivus
                | Case::Modalis
                | Case::DistributivusTemporalis,
            ) if numbered || possessed => {
                return Err(GenError::NoSuchCell(
                    "a case-like suffix closes an unpossessed singular",
                ));
            }
            Morph::Case(Case::Tem) if follows_anaphoric(&stack[..i]) => {
                return Err(GenError::NoSuchCell("the temporal follows no -é or -ék"));
            }
            // `NOM-CASE-009`: `-ként` and `-Ul` are degraded after `-é` (`*házéként`, `*házéul`).
            Morph::Case(Case::Formalis | Case::Ess) if stack[..i].contains(&Morph::Anaphoric) => {
                return Err(GenError::NoSuchCell("no essive after the anaphoric"));
            }
            // `DER-ADV-003`: the essive adverb of an adjective is formed on privatives and
            // language names; a participle takes `-An` (`*mérhetőül`, `*olvashatóul`). A potential
            // participle is one whatever category it is read as, so it is refused for a noun
            // reading too, where `NOM-CASE-009`'s essive otherwise lets it through.
            Morph::Case(Case::Ess)
                if potential_participle(&entry.lemma)
                    || (pos == Pos::Adjective
                        && (stack[..i].iter().any(|m| {
                            !matches!(
                                m,
                                Morph::Comparative | Morph::Superlative | Morph::Excessive
                            )
                        }) || crate::compound::participle_shaped(&entry.lemma))) =>
            {
                return Err(GenError::NoSuchCell("no essive on a participle"));
            }
            // `PND-DEM-002`: the demonstrative's terminative is the lexicalised `eddig`/`addig`.
            Morph::Case(Case::Ter) if entry.has(Trait::DemonstrativeZ) => {
                return Err(GenError::NoSuchCell(
                    "the demonstrative's terminative is listed",
                ));
            }
            // `VRB-MODEL-003`: `-hAt` takes no converb (`*olvashatva`).
            Morph::Converb if i > 0 && stack[i - 1] == Morph::Potential => {
                return Err(GenError::NoSuchCell("the potential has no converb"));
            }
            // `PND-INDF-004`: among adjectives only a quantifier counts people or times.
            Morph::Collective | Morph::Multiplicative
                if pos == Pos::Adjective && !entry.has(Trait::Quantity) =>
            {
                return Err(GenError::NoSuchCell(
                    "only a quantifier adjective takes the collective or the multiplicative",
                ));
            }
            // `VRB-PART-003`: the future participle has no potential stem.
            Morph::FutureParticiple if i > 0 && stack[i - 1] == Morph::Potential => {
                return Err(GenError::NoSuchCell(
                    "the future participle has no potential stem",
                ));
            }
            _ => {}
        }
        if let Morph::Derive(id) = *morph {
            derivation_chain(entry, &stack[..i], id)?;
            pos = id.record().output;
            continue;
        }
        if !applies_to(*morph, pos) {
            return Err(GenError::NoSuchCell(
                "feature does not apply to this category",
            ));
        }
        // `DER-ADJ-022`: a participle is an adjective and a converb an adverb from here on.
        if matches!(
            morph,
            Morph::PresentParticiple | Morph::PastParticiple | Morph::FutureParticiple
        ) {
            pos = Pos::Adjective;
        } else if *morph == Morph::Converb {
            pos = Pos::Adverb;
        }
    }
    lexical_cells(entry, stack)
}

/// The cells only a marked lexeme has.
///
/// Rules: `ORT-ABBR-009`, `NOM-CASE-016`
fn lexical_cells(entry: &Entry, stack: &[Morph]) -> Result<(), GenError> {
    if !stack.is_empty() && entry.has(Trait::Abbreviation) {
        return Err(GenError::NoSuchCell(
            "an abbreviation suffixes only after a hyphen",
        ));
    }
    // NOM-CASE-016: the locative is not productive; only the towns the lexicon marks take it.
    if stack.contains(&Morph::Case(Case::Loc)) && !entry.has(Trait::Locative) {
        return Err(GenError::NoSuchCell("the locative is lexical"));
    }
    Ok(())
}

/// The person suffix of a possessed plural, whose third singular is zero.
///
/// Rules: `NOM-PPL-001`
fn possessed_plural_person(
    entry: &Entry,
    base: &str,
    person: Person,
    number: Number,
    lowering: bool,
) -> String {
    crate::possessive::possessive_after_i(person, number)
        .map_or_else(|| base.to_owned(), |s| attach(base, entry, &s, lowering))
}

/// The stem a suffix attaches to: the lemma while nothing has been added, the form itself after.
fn stem_so_far(entry: &Entry, form: &str, at_lemma: bool) -> String {
    if at_lemma {
        entry.lemma.to_string()
    } else {
        form.to_owned()
    }
}

/// The entry a verbal slot inflects: the verb the stack has derived, else the lemma's own.
///
/// `DER-MODEL-010`: a derived verb is a word of its own, so `nézeget` and `írat` conjugate on
/// their own stem and past class rather than on `néz` and `ír`.
fn verb_base<'a>(entry: &'a Entry, derived: Option<&'a Entry>) -> &'a Entry {
    derived.filter(|d| d.pos == Pos::Verb).unwrap_or(entry)
}

/// Builds the potential stem.
///
/// `VRB-STEM-012` puts it on the `d` stem (`aludhat`, not `*alszhat`) and `VRB-IRR-*` on the
/// suppletive one (`lehet`, not `*vanhat`).
fn potential_step(entry: &Entry, verb_stem: Option<&str>, lowering: bool) -> String {
    let base = verb_stem.map_or_else(
        || {
            crate::irregular::irregular(&entry.lemma).map_or_else(
                || crate::verbal::unlinked_stem(entry),
                |i| i.potential.to_owned(),
            )
        },
        str::to_owned,
    );
    attach(&base, entry, &crate::verbal::potential(), lowering)
}

/// Derives, taking the current form as the stem once inflection has begun.
///
/// Rules: `DER-MODEL-003`
fn derive_step(
    entry: &Entry,
    derived: Option<&Entry>,
    form: &str,
    at_lemma: bool,
    pos: Pos,
    id: crate::derivation::DerId,
) -> Result<Entry, GenError> {
    let base = if at_lemma {
        entry.clone()
    } else if let Some(d) = derived {
        // The stack has already produced a word of its own, with its own category and its own
        // stem properties (`DER-MODEL-010`), which the lemma's traits do not carry: a participle
        // takes `NOM-ADJ-010`'s link (`kezdődően`) although its verb would not.
        let mut e = d.clone();
        e.lemma = form.into();
        e
    } else {
        let mut e = Entry::new(form, pos);
        e.traits = entry.traits;
        e
    };
    crate::derivation::derived(&base, crate::derivation::record_for(id, &base))
}

/// Builds a comparative, and prefixes it for the superlative and the excessive.
///
/// Rules: `NOM-ADJ-004`, `NOM-ADJ-005`, `NOM-ADJ-006`
fn gradation_step(
    entry: &Entry,
    form: &str,
    at_lemma: bool,
    morph: Morph,
    lowering: bool,
) -> String {
    let stem = if at_lemma { entry.lemma.as_ref() } else { form };
    // DER-PREF-004: `leg-` attaches to a comparative, and lexically to a closed list of
    // ordinal-like words that have no comparative at all. For those the prefix goes straight on
    // the base, so `legelső` is `leg` + `első` and not `leg` + a comparative of it.
    if matches!(morph, Morph::Superlative | Morph::Excessive)
        && lists(Role::NonComparativeSuperlative, stem)
    {
        return match morph {
            Morph::Excessive => format!("legesleg{stem}"),
            _ => format!("leg{stem}"),
        };
    }
    let base = pairs(Role::IrregularComparative)
        .iter()
        .find(|(w, _)| w.as_ref() == stem)
        .map_or_else(
            || {
                let s = comparative();
                let b = if at_lemma {
                    alternant(entry, &s).into_owned()
                } else {
                    form.to_owned()
                };
                attach(&b, entry, &s, lowering)
            },
            |(_, c)| c.to_string(),
        );
    match morph {
        Morph::Superlative => format!("leg{base}"),
        Morph::Excessive => format!("legesleg{base}"),
        _ => base,
    }
}

/// Whether this cell is exempt from the lexeme's lowering marking.
///
/// `NOM-LOW-001` marks lowering per lexeme, and the plural and the accusative come apart for a
/// measurable class: `olaj` gives `olajat` and `olajok`. Each exemption is measured on its own
/// cell, so neither is inferred from the other.
///
/// Rules: `NOM-LOW-001`, `NOM-PL-001`, `NOM-ACC-005`
/// Only while the stem is still the lemma: `NOM-LOW-002` makes an inflected stem lowering whatever
/// the lexeme is marked, so the exemption cannot survive an earlier suffix or a derivation.
fn cell_exempt(entry: &Entry, morph: Morph) -> bool {
    match morph {
        Morph::Plural => entry.has(Trait::LowPluralExempt),
        Morph::Case(Case::Acc) => entry.has(Trait::LowAccusativeExempt),
        Morph::Possessive { .. } | Morph::PossessedPlural => entry.has(Trait::LowPossessiveExempt),
        _ => false,
    }
}

/// Generates a form of a compound that the lexicon does not list as a whole.
///
/// `DER-COMP-008` and `DER-COMP-009`: a compound inflects exactly as its head member does, so
/// `vashíd` gives `vashidat` and `vashidak` because `híd` gives `hidat` and `hidak`, and
/// `virágbokor` gives `virágbokrok` because `bokor` drops its epenthetic vowel. Compounding is
/// productive, so the lexicon cannot list the results and generation has to compose them.
///
/// Analysis already reads such a form by splitting it and analysing the head; this is the same
/// step in the other direction, which is what lets a caller ask a novel compound for its paradigm.
///
/// The members are tried longest-prefix first, and the first head the lexicon knows decides.
///
/// Rules: `DER-COMP-008`, `DER-COMP-009`
pub fn generate_compound(lemma: &str, stack: &[Morph]) -> Result<String, GenError> {
    let mut best: Option<(usize, String)> = None;
    for (members, at) in crate::compound::member_prefixes(lemma) {
        if members.is_empty() || at >= lemma.len() {
            continue;
        }
        let (prefix, head) = (&lemma[..at], &lemma[at..]);
        let after_preverb = crate::compound::preverb_member(&members).is_some();
        for entry in picur_morph_lexicon::lexicon().get(head) {
            if !crate::compound::head_allows(entry.pos, after_preverb)
                || !crate::compound::numeral_head_allows(entry.pos, &members)
            {
                continue;
            }
            if let Ok(form) = generate(entry, stack)
                && best.as_ref().is_none_or(|(len, _)| at > *len)
            {
                // VRB-PREV-005: the imperative puts the preverb after the verb, written apart
                // (`egyem meg`, `fedd meg`).
                let imperative = matches!(
                    stack.first(),
                    Some(Morph::Verbal {
                        mood: Mood::Imp,
                        ..
                    })
                );
                let written = if after_preverb && imperative {
                    format!("{form} {prefix}")
                } else {
                    format!("{prefix}{form}")
                };
                best = Some((at, written));
            }
        }
    }
    best.map(|(_, f)| f)
        .ok_or(GenError::Unsupported("a compound with no listed head"))
}

/// Generates a surface form from a lemma and an ordered feature stack.
///
/// Rules: `NOM-MODEL-003`
pub fn generate(entry: &Entry, stack: &[Morph]) -> Result<String, GenError> {
    generate_seam(entry, stack).map(|(form, _)| form)
}

/// Generates a form and reports where the lemma's stem ends in it.
///
/// The seam is the offset the first suffix was written at, which is what `PND-NUMINF-010` needs:
/// the written half of a suffix after a hyphen is everything past that offset, so `első` plus the
/// third person and the superessive gives `jén` and `1-jén`. It is `None` when the stack's first
/// step rebuilds the form instead of attaching to it, as a derivation or a participle does.
///
/// Rules: `NOM-MODEL-003`, `PND-NUMINF-010`
pub fn generate_seam(entry: &Entry, stack: &[Morph]) -> Result<(String, Option<usize>), GenError> {
    generate_guided(entry, stack, None)
}

/// Generates a form that is meant to equal `target`, giving up as soon as it cannot.
///
/// Analysis regenerates every candidate stack and compares, and almost every candidate fails;
/// this is the same generation, stopped at the first step after which the form can no longer
/// become `target`. A step only appends to the form or rewrites its seam, so once the form minus
/// its last two letters is not a prefix of `target`, no later step recovers. The result, when
/// there is one, is exactly what [`generate`] returns.
pub fn generate_toward(entry: &Entry, stack: &[Morph], target: &str) -> Result<String, GenError> {
    generate_guided(entry, stack, Some(target)).map(|(form, _)| form)
}

/// Bytes at the end of a form a later step may still rewrite: two letters, three bytes each at
/// most, for the seam changes a suffix makes (`oszt` gives `ossz`, `alma` gives `almák`).
const SEAM_SLACK: usize = 6;

/// Whether `form` can still become `target` by steps that only extend it or rewrite its seam.
fn heads_toward(form: &str, target: &str) -> bool {
    let mut keep = form.len().saturating_sub(SEAM_SLACK);
    while !form.is_char_boundary(keep) {
        keep -= 1;
    }
    target.starts_with(&form[..keep])
}

/// [`generate_seam`], checked against `target` after every step when one is given.
#[expect(
    clippy::too_many_lines,
    reason = "one arm per slot of the feature stack; splitting it would hide the order the slots \
              apply in, which is the model `VRB-MODEL-001` and `NOM-MODEL-001` state"
)]
fn generate_guided(
    entry: &Entry,
    stack: &[Morph],
    target: Option<&str>,
) -> Result<(String, Option<usize>), GenError> {
    if !stack.is_empty() && entry.has(Trait::Spoken) {
        return spoken_form(entry, stack);
    }
    // ORT-ABBR-009: an acronym whose reading its shape fixes takes its case after a hyphen; a
    // lexical pronunciation (`AIDS` read as a foreign word) was answered above.
    if let [Morph::Case(case)] = stack
        && is_acronym(&entry.lemma)
        && let Some(form) = crate::acronym::inflected(&entry.lemma, *case)
    {
        return Ok((form, Some(entry.lemma.len() + 1)));
    }
    admissible(entry, stack)?;
    // `ORT-FOR-012`: the suffix of such a name follows its pronunciation, which no letter shows.
    // `PH-G2P-004` exempts the archaic Hungarian spellings, whose value the table gives: the
    // `gh` of `Balogh` is [g] where the `gh` of `Edinburgh` is not read the Hungarian way.
    if !stack.is_empty()
        && starts_upper(&entry.lemma)
        && !entry.has(Trait::ArchaicSpelling)
        && unusual_final(&entry.lemma)
    {
        return Err(GenError::Unsupported(
            "a foreign name's suffix follows its pronunciation",
        ));
    }
    let mut form = with_room(&entry.lemma);
    // Fixed by the first suffix attached to the lemma, held as the stem it was written after so
    // that a later step which rebuilds the form instead of appending to it invalidates itself.
    let mut seam: Option<String> = None;
    // NOM-LOW-002: an inflected stem is lowering, so the flag can only turn on as we go.
    // NOM-LOW-001 plus NOM-STEM-008: a v-stem is lowering as a class, so this is computed rather
    // than marked: every attested v-stem noun takes the low link (`lovat`, `követ`, `tavat`,
    // `füvet`, `falvak` 71,593 : `falvok` 3 [WC2]), whatever morphdb marks on it.
    let mut lowering = entry.has(Trait::Low) || entry.has(Trait::VStem);
    // Alternation applies to the lemma, not to an already-suffixed form: only the first suffix
    // sees the bound alternant, and after that the stem is the inflected string.
    let mut at_lemma = true;
    let mut pos = entry.pos;
    // `DER-MODEL-010` assigns the output's stem properties rather than inheriting them, so once
    // the stack has derived, the rest of it is judged against the derived word and not the lemma:
    // `kezdődő` plus `-An` is `kezdődően` by `NOM-ADJ-010`, which the verb `kezdődik` would refuse.
    let mut derived: Option<Entry> = None;
    let mut verb_stem: Option<String> = None;
    let mut after_possessed_plural = false;
    let mut after_infinitive = false;
    // NOM-STEM-017: an inflected form is a new stem, non-alternating and lowering, so the
    // lemma's accusative class does not reach it (`embert` but `embereket`).
    //
    // The harmony traits do not travel either. An antiharmonic marking says that the lemma's
    // spelling misleads, and the inflected form carries the suffix's own vowel, which does
    // not: `másképp` computes front from its `é` (`másképpen`).
    //
    // The seams do travel: they are where the word is put together, and `DER-COMP-008` makes
    // the last member the harmony domain (`akik` harmonises on `ki`: `akiknek`).
    //
    // Built once: nothing that reads a host's lemma survives in this trait set, so the host
    // need not follow the form from suffix to suffix.
    let mut inflected: Option<Entry> = None;
    // NOM-ANP-002, NOM-ASSOC-002: the anaphoric `-é` and the associative `-ék` carry no harmony
    // of their own, so a suffix after them agrees with the stem through their neutral vowels.
    // The class is frozen where they attached.
    let mut frozen: Option<Harmony> = None;
    for (i, morph) in stack.iter().enumerate() {
        if i > 0
            && let Some(target) = target
            && !heads_toward(&form, target)
        {
            return Err(GenError::NoSuchCell("the form has left the target"));
        }
        let suffix = match *morph {
            Morph::Case(Case::Nom) => continue,
            // NOM-CASE-020: `-képpen` is one invariant suffix, so its `-en` does not harmonise.
            Morph::Case(Case::Sup) if i > 0 && stack[i - 1] == Morph::Case(Case::Modalis) => {
                crate::suffix::modalis_superessive()
            }
            Morph::Case(c) => {
                let Some(s) = case_suffix(c) else { continue };
                s
            }
            Morph::Derive(id) => {
                // PND-NUMINF-005: `-szOrOs` is `-szOr` with `-Os` on it (`hatszoros`), the one
                // place the adverb takes it.
                let after_multiplicative = i > 0 && stack[i - 1] == Morph::Multiplicative;
                let base_pos = if after_multiplicative && id == crate::derivation::DerId::Os {
                    Pos::Noun
                } else {
                    pos
                };
                let out = derive_step(entry, derived.as_ref(), &form, at_lemma, base_pos, id)?;
                lowering = out.has(Trait::Low);
                pos = out.pos;
                if seam.is_none() && at_lemma && out.lemma.starts_with(form.as_str()) {
                    seam = Some(form.clone());
                }
                form = out.lemma.to_string();
                derived = Some(out);
                at_lemma = false;
                continue;
            }
            Morph::Collective | Morph::Multiplicative => {
                // PND-NUMINF-005: `-szor` and `-An` make an adverb, which `-i` then builds on
                // (`hatszori`).
                pos = Pos::Adverb;
                if let Some(built) = numeral_two(entry, *morph, lowering) {
                    if seam.is_none() && at_lemma && built.starts_with(form.as_str()) {
                        seam = Some(form.clone());
                    }
                    form = built;
                    at_lemma = false;
                    continue;
                }
                if matches!(morph, Morph::Collective) {
                    collective()
                } else {
                    multiplicative()
                }
            }
            // NOM-ANP-001: the anaphoric `-é` pluralises with the possessed plural's `-i`, not
            // with `-k`: `házéi`, `házéit`, `házaiméiból`.
            Morph::Plural if i > 0 && stack[i - 1] == Morph::Anaphoric => anaphoric_plural(),
            Morph::Plural => plural(),
            Morph::Comparative | Morph::Superlative | Morph::Excessive => {
                form = gradation_step(entry, &form, at_lemma, *morph, lowering);
                at_lemma = false;
                // NOM-ADJ-007: a comparative stem is lowering, so `nagyobb` pluralises `nagyobbak`.
                lowering = true;
                continue;
            }
            Morph::Anaphoric => anaphoric(),
            Morph::Associative => associative(),
            Morph::PossessedPlural
                if at_lemma
                    && let Some(built) =
                        crate::possessive::irregular_possessed_plural(&entry.lemma) =>
            {
                form.clear();
                form.push_str(built);
                at_lemma = false;
                after_possessed_plural = true;
                lowering = true;
                continue;
            }
            Morph::PossessedPlural => {
                // The allomorph is chosen against the alternant, not the lemma: `műv` ends in a
                // consonant and so takes `-ei`, while the lemma `mű` is vowel-final and took the
                // bare `-i`, giving `műi` for `művei`.
                let base = (at_lemma && entry.has(Trait::PossessedPluralAlt))
                    .then(|| crate::alternation::possessive_alt(&entry.lemma))
                    .flatten()
                    .or_else(|| bound_for_vowel(entry, at_lemma))
                    .unwrap_or_else(|| stem_so_far(entry, &form, at_lemma));
                after_possessed_plural = true;
                crate::possessive::possessed_plural(&base, entry)
            }
            // VRB-INF-002: every cell is built on the infinitive, which is why the irregular
            // infinitives need no separate statement: `lenni` gives `lennem` and `lennie`.
            Morph::Possessive { person, number } if after_infinitive => {
                form = personal_infinitive(&form, entry, person, number);
                at_lemma = false;
                continue;
            }
            // `NOM-POSS-009`: the irregular form is the inalienable sense (`fia`), and the regular
            // `-jA` form is the other (`fiúja`), so an entry asking for `-jA` is not irregular.
            Morph::Possessive { person, number }
                if at_lemma
                    && !entry.has(Trait::PossessiveJa)
                    && let Some(built) =
                        crate::possessive::irregular(&entry.lemma, person, number) =>
            {
                form.clear();
                form.push_str(built);
                at_lemma = false;
                lowering = true;
                continue;
            }
            Morph::Possessive { person, number } => {
                // NOM-STEM-013: the `ajtó` type alternates before the third person possessive, and
                // the alternant is what the `-a`/`-ja` choice is made against: `idej` ends in a
                // consonant and takes `-e`, while the lemma `idő` would take `-je`.
                let base = possessive_alt_stem(entry, person, at_lemma)
                    .or_else(|| bound_for_vowel(entry, at_lemma))
                    .unwrap_or_else(|| stem_so_far(entry, &form, at_lemma));
                // NOM-PPL-001: after the possessed plural the person suffixes are a reduced set
                // with a zero third singular, so `adatai` is already "his data".
                if after_possessed_plural {
                    form = possessed_plural_person(entry, &base, person, number, lowering);
                    at_lemma = false;
                    lowering = true;
                    continue;
                }
                // NOM-STEM-017: a graded or inflected form is a new stem, so the lemma's
                // possessive class does not reach it: `jó` has `java`, its comparative `jobbja`.
                // A derived word answers for itself (`DER-MODEL-010`).
                let plain;
                let host = if at_lemma {
                    entry
                } else if let Some(d) = derived.as_ref() {
                    d
                } else {
                    plain = Entry::new(base.as_str(), pos);
                    &plain
                };
                // NOM-ADJ-007: the comparative `-bb` is a suffix, and a nominalised comparative
                // takes `-jA` on it whatever its base took (`java` but `jobbja`, `szebbje`).
                let graded = i > 0
                    && matches!(
                        stack[i - 1],
                        Morph::Comparative | Morph::Superlative | Morph::Excessive
                    );
                let graded_host;
                let host = if graded {
                    graded_host = Entry {
                        traits: host.traits.with(Trait::PossessiveJa),
                        ..host.clone()
                    };
                    &graded_host
                } else {
                    host
                };
                crate::possessive::possessive(person, number, &base, host)
            }
            Morph::Potential => {
                form = potential_step(
                    verb_base(entry, derived.as_ref()),
                    verb_stem.as_deref(),
                    lowering,
                );
                verb_stem = Some(form.clone());
                at_lemma = false;
                continue;
            }
            Morph::PresentParticiple
            | Morph::PastParticiple
            | Morph::Converb
            | Morph::FutureParticiple
            | Morph::Infinitive
            | Morph::Verbal { .. } => {
                form = verbal_form(
                    verb_base(entry, derived.as_ref()),
                    *morph,
                    verb_stem.as_deref(),
                    lowering,
                )?;
                at_lemma = false;
                pos = match morph {
                    Morph::PresentParticiple | Morph::PastParticiple | Morph::FutureParticiple => {
                        Pos::Adjective
                    }
                    Morph::Converb => Pos::Adverb,
                    _ => pos,
                };
                if pos != entry.pos {
                    let mut out = Entry::new(form.as_str(), pos);
                    // NOM-ADJ-010 lists the `-ó/-ő` participles with the `-an/-en` link, beside
                    // the bare `-n` of `drágán` and `olcsón`: `kezdődően`, `bántóan`. The link
                    // is the adverb's alone: the plural is the bare `-k` that `NOM-PL-003`
                    // prefers, `értendők` and `kezdődők`.
                    let adverb_next =
                        stack.get(i + 1) == Some(&Morph::Derive(crate::derivation::DerId::An));
                    if pos == Pos::Adjective && adverb_next && form.ends_with(['ó', 'ő']) {
                        out.traits = out.traits.with(Trait::LinkedPlural);
                    }
                    derived = Some(out);
                }
                after_infinitive = matches!(morph, Morph::Infinitive);
                continue;
            }
        };
        let mut suffix = suffix;
        if is_possessive_alternant(entry, *morph, at_lemma) {
            suffix.lengthens = false;
        }
        let (base, free_stem) = if at_lemma {
            stem_for(entry, *morph, &suffix)
        } else {
            (std::mem::take(&mut form), false)
        };
        // NOM-CASE-016: `-t` after a vowel, `-Ott` after a consonant (`Pécsett`, `Győrött`).
        if *morph == Morph::Case(Case::Loc) && !matches!(coda(&base), Coda::Vowel) {
            suffix = crate::suffix::locative_linked();
        }
        // The free stem is not the alternant, so the v-stem lowering that belongs to the alternant
        // does not apply to it: `mag` gives `magvak` but `magot`, not `magat`.
        // NOM-LOW-001 is marked per lexeme but the cells come apart: `olaj` gives `olajat` and
        // `olajok`, so the plural is exempt while the accusative is not.
        // The truncated `ily-`/`oly-` takes the low link, which is what `PND-DEM-008`'s `ilyet`
        // and `olyat` are; the harmony of the truncated stem supplies the vowel's quality.
        let low_here = if at_lemma
            && *morph == Morph::Case(Case::Acc)
            && truncating_demonstrative(&entry.lemma).is_some()
        {
            true
        } else if at_lemma && cell_exempt(entry, *morph) {
            false
        } else if free_stem {
            entry.has(Trait::Low)
        } else {
            lowering
        };
        if !at_lemma && derived.is_none() && inflected.is_none() {
            let mut e = Entry::new(base.as_str(), pos);
            e.parts.clone_from(&entry.parts);
            e.traits = TraitSet::from_iter_of(std::iter::once(Trait::Low));
            inflected = Some(e);
        }
        let host = (derived.is_none())
            .then_some(inflected.as_ref())
            .flatten()
            .or(derived.as_ref())
            .unwrap_or(entry);
        let frozen_host = frozen.map(|h| {
            let mut e = Entry::new(base.as_str(), pos);
            e.traits = TraitSet::from_iter_of([Trait::Low, harmony_trait(h)]);
            e
        });
        let host = frozen_host.as_ref().unwrap_or(host);
        let transparent =
            matches!(morph, Morph::Anaphoric | Morph::Associative).then(|| harmony(&base, host));
        let at;
        (form, at) = attach_owned(base, host, &suffix, low_here);
        frozen = frozen.or(transparent);
        if seam.is_none() && at_lemma {
            seam = Some(form[..at].to_owned());
        }
        at_lemma = false;
        // NOM-LOW-002: an inflected stem is lowering, the possessed plural, the anaphoric `-é`
        // and the associative `-ék` included. `adatai` takes `adataimat`, not `adataimot`,
        // although `adat` itself is not a lowering stem; `Szabóék` takes `Szabóékat`.
        if matches!(
            morph,
            Morph::Plural
                | Morph::Possessive { .. }
                | Morph::PossessedPlural
                | Morph::Anaphoric
                | Morph::Associative
        ) {
            lowering = true;
        }
    }
    let seam = seam
        .filter(|stem| form.starts_with(stem.as_str()))
        .map(|stem| stem.len());
    Ok((form, seam))
}

/// The plural suffix `-(O)k`.
///
/// Rules: `NOM-PL-001`
#[must_use]
pub fn plural() -> Suffix {
    use crate::suffix::{Arch, SuffixClass};
    Suffix {
        template: &[Seg::Var(Arch::O), Seg::Lit("k")],
        class: SuffixClass::One,
        lengthens: true,
        lowerable: true,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: true,
    }
}

/// The comparative `-(A)bb`.
///
/// Self-lowering: the linking vowel is low whatever the stem does, so `nagy` gives `nagyabb` by
/// the default rule and the mid-vowel exceptions are lexical (`NOM-ADJ-005`).
///
/// Rules: `NOM-ADJ-004`
#[must_use]
pub fn comparative() -> Suffix {
    use crate::suffix::{Arch, SuffixClass};
    Suffix {
        template: &[Seg::Var(Arch::A), Seg::Lit("bb")],
        class: SuffixClass::One,
        lengthens: true,
        lowerable: true,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: false,
    }
}

/// The anaphoric possessive `-é`.
///
/// Invariant for harmony, but it lengthens a final `a`/`e`: `fa` gives `fáé`.
///
/// Rules: `NOM-ANP-001`
#[must_use]
pub fn anaphoric() -> Suffix {
    use crate::suffix::SuffixClass;
    Suffix {
        template: &[Seg::Lit("é")],
        class: SuffixClass::Two,
        lengthens: true,
        lowerable: false,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: false,
    }
}

/// The collective `-An`, 'as N people'.
///
/// Two-way only: `PND-NUMINF-005` says it has no `o`/`ö` allomorph, so `öt` gives `öten` and not
/// `*ötön`. The vowel belongs to the suffix and appears after a vowel-final stem too, without
/// lengthening: `millióan`, `ennyien`.
///
/// Rules: `PND-NUMINF-005`
#[must_use]
pub fn collective() -> Suffix {
    use crate::suffix::{Arch, SuffixClass};
    Suffix {
        template: &[Seg::Var(Arch::A), Seg::Lit("n")],
        class: SuffixClass::Two,
        lengthens: false,
        lowerable: false,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: false,
    }
}

/// The multiplicative `-szOr`, 'N times'.
///
/// Rules: `PND-NUMINF-006`
#[must_use]
pub fn multiplicative() -> Suffix {
    use crate::suffix::{Arch, SuffixClass};
    Suffix {
        template: &[Seg::Lit("sz"), Seg::Var(Arch::O), Seg::Lit("r")],
        class: SuffixClass::Two,
        lengthens: false,
        lowerable: false,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: false,
    }
}

/// The plural of the anaphoric `-é`, which is the possessed plural's `-i`.
///
/// Rules: `NOM-ANP-001`
#[must_use]
pub fn anaphoric_plural() -> Suffix {
    use crate::suffix::SuffixClass;
    Suffix {
        template: &[Seg::Lit("i")],
        class: SuffixClass::Two,
        lengthens: false,
        lowerable: false,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: false,
    }
}

/// The associative plural `-ék`.
///
/// Rules: `NOM-ASSOC-001`
#[must_use]
pub fn associative() -> Suffix {
    use crate::suffix::SuffixClass;
    Suffix {
        template: &[Seg::Lit("ék")],
        class: SuffixClass::Two,
        lengthens: true,
        lowerable: false,
        bare_coda: false,
        free_stem: false,
        high_vowel_link: false,
    }
}

#[cfg(test)]
mod tests {
    use picur_morph_lexicon::{Entry, Pos, Trait, TraitSet};

    use super::{generate, harmony};
    use crate::features::{Case, Morph};
    use picur_morph_ortho::Harmony;

    fn noun(lemma: &str, traits: &[Trait]) -> Entry {
        let mut e = Entry::new(lemma, Pos::Noun);
        e.traits = TraitSet::from_iter_of(traits.iter().copied());
        e
    }

    fn form(lemma: &str, traits: &[Trait], stack: &[Morph]) -> String {
        generate(&noun(lemma, traits), stack).expect("generates")
    }

    #[test]
    fn back_stem_case_paradigm() {
        let t = &[Trait::Low];
        assert_eq!(form("ház", t, &[Morph::Case(Case::Acc)]), "házat");
        assert_eq!(form("ház", t, &[Morph::Case(Case::Ine)]), "házban");
        assert_eq!(form("ház", t, &[Morph::Case(Case::All)]), "házhoz");
        assert_eq!(form("ház", t, &[Morph::Case(Case::Ela)]), "házból");
        assert_eq!(form("ház", t, &[Morph::Case(Case::Ade)]), "háznál");
        assert_eq!(form("ház", t, &[Morph::Case(Case::Dat)]), "háznak");
        assert_eq!(form("ház", t, &[Morph::Case(Case::Ter)]), "házig");
        assert_eq!(form("ház", t, &[Morph::Case(Case::Nom)]), "ház");
    }

    #[test]
    fn lowering_versus_non_lowering_minimal_pair() {
        assert_eq!(
            form("ház", &[Trait::Low], &[Morph::Case(Case::Acc)]),
            "házat"
        );
        assert_eq!(form("gáz", &[], &[Morph::Case(Case::Acc)]), "gázt");
        assert_eq!(form("ház", &[Trait::Low], &[Morph::Plural]), "házak");
        assert_eq!(form("gáz", &[], &[Morph::Plural]), "gázok");
    }

    #[test]
    fn superessive_keeps_the_mid_link_on_a_lowering_stem() {
        assert_eq!(
            form("ház", &[Trait::Low], &[Morph::Case(Case::Sup)]),
            "házon"
        );
        assert_eq!(
            form("fül", &[Trait::Low], &[Morph::Case(Case::Sup)]),
            "fülön"
        );
        assert_eq!(form("fül", &[Trait::Low], &[Morph::Plural]), "fülek");
    }

    #[test]
    fn bare_accusative_after_a_coronal_sonorant() {
        assert_eq!(form("dal", &[], &[Morph::Case(Case::Acc)]), "dalt");
        assert_eq!(form("bor", &[], &[Morph::Case(Case::Acc)]), "bort");
        assert_eq!(form("lány", &[], &[Morph::Case(Case::Acc)]), "lányt");
        assert_eq!(form("rizs", &[], &[Morph::Case(Case::Acc)]), "rizst");
    }

    #[test]
    fn linking_vowel_after_an_obstruent() {
        assert_eq!(form("bot", &[], &[Morph::Case(Case::Acc)]), "botot");
        assert_eq!(form("rab", &[], &[Morph::Case(Case::Acc)]), "rabot");
        assert_eq!(form("nyom", &[], &[Morph::Case(Case::Acc)]), "nyomot");
    }

    #[test]
    fn orr_needs_its_lexical_override() {
        // NOM-ACC-006 forbids `orrt`; the trait is what prevents the computed default.
        assert_eq!(
            form("orr", &[Trait::LinkedAccusative], &[Morph::Case(Case::Acc)]),
            "orrot"
        );
        assert_eq!(form("stressz", &[], &[Morph::Case(Case::Acc)]), "stresszt");
    }

    #[test]
    fn instrumental_v_assimilates_and_writes_simplified() {
        // NOM-CASE-003: after a vowel the v stays.
        assert_eq!(form("hajó", &[], &[Morph::Case(Case::Ins)]), "hajóval");
        assert_eq!(form("ház", &[], &[Morph::Case(Case::Ins)]), "házzal");
        assert_eq!(form("kert", &[], &[Morph::Case(Case::Ins)]), "kerttel");
        assert_eq!(form("ész", &[], &[Morph::Case(Case::Ins)]), "ésszel");
        assert_eq!(form("lány", &[], &[Morph::Case(Case::Ins)]), "lánnyal");
    }

    #[test]
    fn final_vowel_lengthens_before_a_lengthening_suffix() {
        assert_eq!(form("alma", &[], &[Morph::Case(Case::Acc)]), "almát");
        assert_eq!(form("alma", &[], &[Morph::Case(Case::Dat)]), "almának");
        assert_eq!(form("alma", &[], &[Morph::Plural]), "almák");
        assert_eq!(
            form("alma", &[], &[Morph::Case(Case::Formalis)]),
            "almaként"
        );
    }

    #[test]
    fn antiharmonic_stems_use_the_trait_not_the_spelling() {
        // hid is spelled with a neutral vowel only, so harmony_of says Front; the trait fixes it.
        assert_eq!(harmony("híd", &noun("híd", &[])), Harmony::Front);
        assert_eq!(harmony("híd", &noun("híd", &[Trait::Back])), Harmony::Back);
        assert_eq!(
            form("híd", &[Trait::Back], &[Morph::Case(Case::Ine)]),
            "hídban"
        );
        assert_eq!(
            form("híd", &[Trait::Back], &[Morph::Case(Case::Dat)]),
            "hídnak"
        );
    }

    #[test]
    fn plural_then_case_stacks_in_order() {
        // NOM-MODEL-003: plural is inside case.
        assert_eq!(
            form(
                "ház",
                &[Trait::Low],
                &[Morph::Plural, Morph::Case(Case::Ine)]
            ),
            "házakban"
        );
        assert_eq!(
            form("gáz", &[], &[Morph::Plural, Morph::Case(Case::All)]),
            "gázokhoz"
        );
    }

    #[test]
    fn front_rounded_paradigm() {
        assert_eq!(form("tök", &[], &[Morph::Case(Case::All)]), "tökhöz");
        assert_eq!(form("tök", &[], &[Morph::Case(Case::Acc)]), "tököt");
        assert_eq!(form("tök", &[], &[Morph::Plural]), "tökök");
    }
}

#[cfg(test)]
mod alternating_paradigms {
    use picur_morph_lexicon::{Entry, Pos, Trait, TraitSet};

    use super::generate;
    use crate::features::{Case, Morph};

    fn form(lemma: &str, traits: &[Trait], stack: &[Morph]) -> String {
        let mut e = Entry::new(lemma, Pos::Noun);
        e.traits = TraitSet::from_iter_of(traits.iter().copied());
        generate(&e, stack).expect("generates")
    }

    const ACC: &[Morph] = &[Morph::Case(Case::Acc)];
    const PL: &[Morph] = &[Morph::Plural];
    const INE: &[Morph] = &[Morph::Case(Case::Ine)];

    #[test]
    fn epenthetic_paradigm() {
        // NOM-STEM-006: bokor ~ bokr-. The cluster created by epenthesis is what forces the link.
        let t = &[Trait::Epenthetic];
        assert_eq!(form("bokor", t, ACC), "bokrot");
        assert_eq!(form("bokor", t, PL), "bokrok");
        assert_eq!(form("bokor", t, INE), "bokorban");
        assert_eq!(form("majom", t, ACC), "majmot");
        assert_eq!(form("majom", t, PL), "majmok");
    }

    #[test]
    fn epenthetic_lowering_paradigm() {
        let t = &[Trait::Epenthetic, Trait::Low];
        assert_eq!(form("retek", t, PL), "retkek");
        assert_eq!(form("retek", t, ACC), "retket");
    }

    #[test]
    fn shortening_paradigm() {
        // NOM-STEM-008: kez ~ kez-, and the class II form keeps the long vowel.
        let t = &[Trait::Shortening, Trait::Low];
        assert_eq!(form("kéz", t, ACC), "kezet");
        assert_eq!(form("kéz", t, PL), "kezek");
        assert_eq!(form("kéz", t, INE), "kézben");
        assert_eq!(form("nyár", t, PL), "nyarak");
        assert_eq!(form("nyár", t, ACC), "nyarat");
    }

    #[test]
    fn v_stem_paradigm() {
        // NOM-STEM-010: lo ~ lov-. The alternant ends in a consonant, so the link appears.
        let t = &[Trait::VStem, Trait::Shortening, Trait::Low];
        assert_eq!(form("ló", t, ACC), "lovat");
        assert_eq!(form("ló", t, PL), "lovak");
        assert_eq!(form("ló", t, INE), "lóban");
        assert_eq!(form("kő", t, ACC), "követ");
        assert_eq!(form("kő", t, PL), "kövek");
    }

    #[test]
    fn vowel_changing_v_stem_paradigm() {
        let t = &[Trait::OaStem, Trait::Low];
        assert_eq!(form("tó", t, ACC), "tavat");
        assert_eq!(form("tó", t, PL), "tavak");
        assert_eq!(form("tó", t, INE), "tóban");
        assert_eq!(form("szó", t, PL), "szavak");
    }

    #[test]
    fn non_shortening_v_stem_paradigm() {
        // `mu` is lowering: the real forms are muvet/muvek, not *muvot/*muvok. Without Low the
        // engine computes FrontRounded from the final u and produces the mid link.
        let t = &[Trait::VStem, Trait::Low];
        assert_eq!(form("mű", t, ACC), "művet");
        assert_eq!(form("mű", t, PL), "művek");
    }
}

/// The position of a person and number in a six-cell paradigm row.
const fn cell_index(person: Person, number: Number) -> usize {
    let row = match person {
        Person::First => 0,
        Person::Second => 1,
        Person::Third => 2,
    };
    row + if matches!(number, Number::Plur) { 3 } else { 0 }
}

/// Generates one cell of a suppletive verb.
///
/// Every branch substitutes a stem and hands off to the regular machinery; nothing is spelled out
/// cell by cell. `jöjjetek` comes out of the ordinary imperative endings, and `tesszük` out of the
/// ordinary j-assimilation.
///
/// Rules: `VRB-IRR-001` .. `VRB-IRR-010`, `VRB-PAST-010`
#[expect(
    clippy::too_many_lines,
    reason = "one arm per cell of the suppletive paradigm"
)]
fn irregular_form(
    entry: &Entry,
    irr: &crate::irregular::Irregular,
    morph: Morph,
    lowering: bool,
) -> Result<String, GenError> {
    if let Morph::Verbal { definite, .. } = morph
        && definite != Definite::Ind
        && !irr.definite
    {
        return Err(GenError::NoSuchCell(
            "definite cell of an intransitive verb",
        ));
    }
    match morph {
        Morph::PresentParticiple => Ok(attach(
            irr.participle,
            entry,
            &crate::verbal::present_participle(),
            lowering,
        )),
        // VRB-PAST-010: the past participle is the third singular indefinite past.
        Morph::PastParticiple => Ok(irr.past_3sg.to_owned()),
        Morph::FutureParticiple if !irr.future_participle.is_empty() => {
            Ok(irr.future_participle.to_owned())
        }
        Morph::FutureParticiple => Err(GenError::NoSuchCell("this verb has no future participle")),
        // VRB-ADVP-001: `tesz`, `vesz`, `visz` and `megy` build the converb on a stem of their
        // own, `téve`, `véve`, `víve`, `menve`, not on the bound stem that gives `venni`.
        Morph::Converb => Ok(attach(
            irr.converb_stem(),
            entry,
            &crate::verbal::converb(),
            lowering,
        )),
        Morph::Infinitive => Ok(attach(
            irr.bound,
            entry,
            &crate::verbal::infinitive(irr.bound, entry),
            lowering,
        )),
        // `VRB-COND-004`: the past conditional is the past indicative plus `volna`, which holds
        // for a suppletive verb as much as for any other (`mentem volna`).
        Morph::Verbal {
            mood: Mood::Cnd,
            tense: Tense::Past,
            person,
            number,
            definite,
        } => irregular_past_conditional(entry, irr, lowering, person, number, definite),
        Morph::Verbal {
            mood: Mood::Cnd,
            person,
            number,
            definite,
            ..
        } => {
            let stem = irr.conditional_stem();
            let marked = attach(
                stem,
                entry,
                &crate::verbal::conditional_marker(stem, entry),
                lowering,
            );
            let p = crate::verbal::conditional_person(person, number, definite);
            Ok(attach(&marked, entry, &p, lowering))
        }
        Morph::Verbal {
            mood: Mood::Imp,
            person,
            number,
            definite,
            ..
        } => {
            if (person, number, definite) == (Person::Second, Number::Sing, Definite::Ind) {
                return Ok(irr.imperative_2sg.to_owned());
            }
            // VRB-IMP-007: `tedd`, `vidd`, `hidd`, none of them derivable from the fused stem.
            if (person, number, definite) == (Person::Second, Number::Sing, Definite::Def) {
                return Ok(irr.imperative_2sg_def.to_owned());
            }
            Ok(
                match crate::verbal::imperative_person(person, number, definite) {
                    Some(p) => attach(irr.imperative, entry, &p, lowering),
                    None => irr.imperative.to_owned(),
                },
            )
        }
        Morph::Verbal {
            tense: Tense::Past,
            person,
            number,
            definite,
            ..
        } => match crate::verbal::past_person(person, number, definite) {
            None => Ok(irr.past_3sg.to_owned()),
            Some(p) => Ok(attach(irr.past, entry, &p, lowering)),
        },
        Morph::Verbal {
            person,
            number,
            definite,
            ..
        } => {
            // VRB-IRR-001, 003, 004: `van`, `megy` and `jön` are suppletive inside the present too.
            if let Some(forms) = irr.present_forms {
                if definite == Definite::Ind {
                    return Ok(forms[cell_index(person, number)].to_owned());
                }
                return Err(GenError::NoSuchCell("present cell of a suppletive verb"));
            }
            Ok(finite(
                entry,
                irr.present,
                Tense::Pres,
                person,
                number,
                definite,
                lowering,
            ))
        }
        _ => Err(GenError::Unsupported("this form of a suppletive verb")),
    }
}

/// An imperative form (`VRB-IMP-002` .. `VRB-IMP-007`).
fn imperative_form(
    entry: &Entry,
    bound: &str,
    lowering: bool,
    person: Person,
    number: Number,
    definite: Definite,
) -> String {
    // VRB-STEM-008: the imperative `-j` takes no linking vowel, so an epenthetic `-ik` verb
    // shows its vowel (`ugorjon`); a stem an earlier slot built is used as it is.
    let full;
    let bound = if bound == crate::verbal::bound_stem(entry)
        && let Some(f) = crate::verbal::full_stem(entry)
    {
        full = f;
        full.as_str()
    } else {
        bound
    };
    // VRB-IMP-007: the definite second singular is the short form, which the rulebook lists first
    // everywhere; the long `-jAd` variant is a correct form too, and analysis builds it.
    if (person, number, definite) == (Person::Second, Number::Sing, Definite::Def) {
        return crate::imperative::short_definite_2sg(bound);
    }
    // VRB-IMP-002..006: the marker fuses with the stem, so the fused form is built first and the
    // person ending attaches to it.
    let imp = crate::imperative::imperative_stem(bound);
    match crate::verbal::imperative_person(person, number, definite) {
        Some(p) => attach(&imp, entry, &p, lowering),
        None => imp,
    }
}

/// The past conditional of an irregular verb (`VRB-COND-004`).
///
/// The copula is the exception the rule names: its past conditional is built on `lesz`, not on
/// `volt`. `lett volna` 245,186 against `volt volna` 535, and `lettem volna` 10,297 against
/// `voltam volna` 44, which is a literary archaism rather than a free alternant
/// [ATT evidence/corpus-variants.md 2026-09-16].
fn irregular_past_conditional(
    entry: &Entry,
    irr: &crate::irregular::Irregular,
    lowering: bool,
    person: Person,
    number: Number,
    definite: Definite,
) -> Result<String, GenError> {
    let ind = Morph::Verbal {
        mood: Mood::Ind,
        tense: Tense::Past,
        person,
        number,
        definite,
    };
    let built = (&*entry.lemma == "van")
        .then(|| {
            picur_morph_lexicon::lexicon()
                .get("lesz")
                .iter()
                .find_map(|e| generate(e, &[ind]).ok())
        })
        .flatten();
    let past = match built {
        Some(f) => f,
        None => irregular_form(entry, irr, ind, lowering)?,
    };
    Ok(format!("{past} volna"))
}

/// The past conditional: the past indicative plus the invariant `volna` (`VRB-COND-004`).
fn past_conditional(
    entry: &Entry,
    derived: Option<&str>,
    lowering: bool,
    person: Person,
    number: Number,
    definite: Definite,
) -> Result<String, GenError> {
    let past = verbal_form(
        entry,
        Morph::Verbal {
            mood: Mood::Ind,
            tense: Tense::Past,
            person,
            number,
            definite,
        },
        derived,
        lowering,
    )?;
    Ok(format!("{past} volna"))
}

/// The non-finite forms, which all attach one suffix to the bound stem.
///
/// `VRB-PART-001`: the present participle uses the derivational stem, which for an sz-d verb is
/// the `v` stem where the verb has one (`alvó`) and the `d` stem otherwise (`emlékező`).
/// `VRB-STEM-007`: the vowel-initial `-Ó` and `-AndÓ` take an epenthetic verb's syncopated stem
/// (`forgó`, `végzendő`), while `-ni` and `-vA` meet the full one (`forogni`, `forogva`). A
/// stem an earlier slot built (`örökölhet`) has settled its alternations and is used as it is.
fn nonfinite_form(
    entry: &Entry,
    morph: Morph,
    bound: &str,
    derived: bool,
    lowering: bool,
) -> String {
    let syncopated = || {
        (!derived && entry.has(Trait::Epenthetic))
            .then(|| crate::alternation::syncopated(&entry.lemma))
            .flatten()
    };
    let (stem, suffix) = match morph {
        Morph::PresentParticiple => {
            let stem = (!derived)
                .then(|| crate::verbal::v_stem(&entry.lemma))
                .flatten()
                .or_else(syncopated);
            return attach(
                stem.as_deref().unwrap_or(bound),
                entry,
                &crate::verbal::present_participle(),
                lowering,
            );
        }
        Morph::FutureParticiple => {
            let stem = syncopated();
            return attach(
                stem.as_deref().unwrap_or(bound),
                entry,
                &crate::verbal::future_participle(),
                lowering,
            );
        }
        Morph::PastParticiple => (bound, crate::verbal::past_participle(bound, entry)),
        // VRB-STEM-008: `-vA` takes no linking vowel, so the full stem returns (`ugorva`).
        Morph::Converb => {
            let stem = (!derived)
                .then(|| crate::verbal::full_stem(entry))
                .flatten();
            return attach(
                stem.as_deref().unwrap_or(bound),
                entry,
                &crate::verbal::converb(),
                lowering,
            );
        }
        _ => (bound, crate::verbal::infinitive(bound, entry)),
    };
    attach(stem, entry, &suffix, lowering)
}

/// Generates a verbal form. Split out of [`generate`] to keep either function readable.
///
/// Rules: `VRB-PRES-001`, `VRB-PRES-002`, `VRB-PAST-001`, `VRB-COND-001`, `VRB-POT-001`,
/// `VRB-INF-001`
/// The verb a derived stem conjugates as, which is a word of its own.
///
/// `DER-MODEL-010`: the stem has resolved every alternation, its past marker is its own business
/// (`maradhat` ends in `t` and takes `-ott` by `VRB-PAST-006`), and its `-ik` membership does not
/// travel, because `VRB-IK-002` holds of an `-ik` verb and `aludhat` is not one: the 1sg
/// indefinite is `aludhatok` and `aludhatom` stays the definite.
///
/// Rules: `DER-MODEL-010`, `VRB-IK-002`, `VRB-PAST-006`
fn derived_host(entry: &Entry, stem: &str) -> Entry {
    let mut e = entry.clone();
    e.lemma = stem.into();
    e.traits = e
        .traits
        .without(Trait::PastFinalT)
        .without(Trait::PastFinalVtt)
        .without(Trait::PastNonfinalT)
        .without(Trait::PastNonfinalVtt);
    e
}

fn verbal_form(
    entry: &Entry,
    morph: Morph,
    derived: Option<&str>,
    lowering: bool,
) -> Result<String, GenError> {
    let host;
    let entry = match derived {
        Some(stem) => {
            host = derived_host(entry, stem);
            &host
        }
        None => entry,
    };
    let stem = derived.map_or_else(
        || crate::verbal::present_stem(&entry.lemma).to_owned(),
        str::to_owned,
    );
    // VRB-STEM-012: the sz-d(-v) verbs use their `d` stem in the past, conditional, infinitive,
    // imperative and potential, and the `sz` stem only in the present.
    let bound = if derived.is_some() {
        stem.clone()
    } else {
        crate::verbal::bound_stem(entry)
    };
    // VRB-IRR-*: a suppletive verb substitutes its own stem in each slot, then inflects normally.
    // A derived stem is already past that point, so `lehetnék` goes through the regular path.
    if derived.is_none()
        && let Some(irr) = crate::irregular::irregular(&entry.lemma)
    {
        return irregular_form(entry, irr, morph, lowering);
    }
    // VRB-LINK-008: `-lAk` exists only for a first person singular subject.
    if let Morph::Verbal {
        person,
        number,
        definite: Definite::Two,
        ..
    } = morph
        && (person, number) != (Person::First, Number::Sing)
    {
        return Err(GenError::NoSuchCell("-lAk outside the first singular"));
    }
    match morph {
        Morph::PresentParticiple
        | Morph::PastParticiple
        | Morph::Converb
        | Morph::FutureParticiple
        | Morph::Infinitive => Ok(nonfinite_form(
            entry,
            morph,
            &bound,
            derived.is_some(),
            lowering,
        )),
        // `VRB-COND-004`: the past conditional is the past indicative plus the invariant `volna`,
        // written apart. Dropping the tense would answer with the present conditional, which is a
        // wrong answer rather than a missing one.
        Morph::Verbal {
            mood: Mood::Cnd,
            tense: Tense::Past,
            person,
            number,
            definite,
        } => past_conditional(entry, derived, lowering, person, number, definite),
        Morph::Verbal {
            mood: Mood::Cnd,
            person,
            number,
            definite,
            ..
        } => {
            let marked = attach(
                &bound,
                entry,
                &crate::verbal::conditional_marker(&bound, entry),
                lowering,
            );
            Ok(attach(
                &marked,
                entry,
                &crate::verbal::conditional_person(person, number, definite),
                lowering,
            ))
        }
        Morph::Verbal {
            mood: Mood::Imp,
            person,
            number,
            definite,
            ..
        } => Ok(imperative_form(
            entry, &bound, lowering, person, number, definite,
        )),
        Morph::Verbal {
            mood: Mood::Ind,
            tense,
            person,
            number,
            definite,
        } => {
            let base = if tense == Tense::Past { &bound } else { &stem };
            Ok(finite(
                entry, base, tense, person, number, definite, lowering,
            ))
        }
        _ => Err(GenError::Unsupported("this verb form")),
    }
}

/// Inflects an already-built verb stem for tense, person, number and definiteness.
///
/// Shared by the indicative and the potential, whose `-hAt` stem inflects exactly like a plain one.
///
/// Rules: `VRB-PRES-001`, `VRB-PRES-002`, `VRB-PAST-001`
fn finite(
    entry: &Entry,
    stem: &str,
    tense: Tense,
    person: Person,
    number: Number,
    definite: Definite,
    lowering: bool,
) -> String {
    if tense == Tense::Past {
        return past_form(entry, stem, person, number, definite, lowering);
    }
    // VRB-PRES-005: `-lAk` is a cell of its own, outside both conjugation tables.
    if definite == Definite::Two {
        let s = crate::verbal::present_lak(stem, entry);
        return attach(stem, entry, &s, lowering);
    }
    if definite == Definite::Def {
        let back = harmony(stem, entry) == Harmony::Back;
        let s = crate::verbal::present_definite(person, number, back);
        // PH-VSTEM-002: a v-stem verb takes the v alternant before the definite `-i`, but keeps the
        // long-vowel stem before a `j`-initial ending. So `lő` gives `lövi`, `lövöd` and `lövik`
        // beside `lőjük`.
        let base = if entry.has(Trait::VStem)
            && entry.pos == Pos::Verb
            && !matches!(s.template.first(), Some(Seg::AssimJ))
        {
            alternant(entry, &crate::generate::plural()).into_owned()
        } else if let Some(alt) = epenthetic_verb_syncope(entry, &s, SyncopeCell::Definite) {
            alt
        } else {
            stem.to_owned()
        };
        return attach(&base, entry, &s, lowering);
    }
    // VRB-LINK-009: the 1sg and 1pl endings always begin with a vowel whatever the stem's shape, so
    // a v-stem verb attaches them to the v alternant: `lő` gives `lövök` and `lövünk` beside `lősz`
    // and `lőtök`, whose endings are consonant-initial.
    // The ending is chosen against the full stem, because its own shape depends on the coda:
    // `érez` takes the vowel-initial `-El` and so syncopates to `érzel`, while `forog` takes `-sz`
    // and keeps its vowel in `forogsz`.
    let ending = crate::verbal::present_indefinite(person, number, stem, entry);
    let syncopated = ending
        .as_ref()
        .and_then(|s| epenthetic_verb_syncope(entry, s, SyncopeCell::Present));
    let stem: &str = &match syncopated {
        Some(alt) => alt,
        None if entry.has(Trait::VStem) && entry.pos == Pos::Verb && person == Person::First => {
            alternant(entry, &crate::generate::plural()).into_owned()
        }
        None => stem.to_owned(),
    };
    match ending {
        Some(s) => attach(stem, entry, &s, lowering),
        // VRB-PRES-001: third singular is zero-marked, or the bare `-ik` lemma.
        None if crate::verbal::is_ik(entry)
            && stem == crate::verbal::present_stem(&entry.lemma) =>
        {
            entry.lemma.to_string()
        }
        None => stem.to_owned(),
    }
}

/// The stem this slot attaches to, and whether it is the free stem rather than the alternant.
///
/// `NOM-STEM-010` singles out the accusative and the superessive, where `szó`, `jó` and `szú`
/// alternate everywhere else but not here, and `NOM-STEM-013` and `NOM-STEM-014` single out the
/// third person possessive.
///
/// Rules: `NOM-STEM-010`, `NOM-STEM-013`, `NOM-STEM-014`, `PH-VSTEM-004`
fn stem_for(entry: &Entry, morph: Morph, suffix: &Suffix) -> (String, bool) {
    if entry.has(Trait::FreeStemAccSup) && matches!(morph, Morph::Case(Case::Acc | Case::Sup)) {
        return (entry.lemma.to_string(), true);
    }
    if let Morph::Possessive { person, .. } = morph
        && crate::alternation::free_personal_possessive(entry, person)
    {
        return (entry.lemma.to_string(), true);
    }
    if entry.has(Trait::FreeStemPlural) && morph == Morph::Plural {
        return (entry.lemma.to_string(), true);
    }
    if let Morph::Possessive { person, .. } = morph
        && let Some(alt) = possessive_alt_stem(entry, person, true)
    {
        return (alt, false);
    }
    // PND-DEM-008: a lemma ending in `ilyen`/`olyan` truncates before the accusative, and the
    // rule states the truncated form as the one to generate: `ilyet` and `olyat` beat `ilyent`
    // and `olyant` about 23 to 1. The condition is the stem ending, so it is computed.
    if morph == Morph::Case(Case::Acc)
        && let Some(cut) = truncating_demonstrative(&entry.lemma)
    {
        return (cut, false);
    }
    if morph == Morph::PossessedPlural
        && entry.has(Trait::PossessedPluralAlt)
        && let Some(alt) = crate::alternation::possessive_alt(&entry.lemma)
    {
        return (alt, false);
    }
    (alternant(entry, suffix).into_owned(), false)
}

/// The truncated accusative stem of an `ilyen`/`olyan` demonstrative.
///
/// `PND-DEM-008` states it as a rule over every lemma ending in those strings, which covers
/// `milyen`, `amilyen` and the prefixed forms without naming them.
///
/// Rules: `PND-DEM-008`
pub(crate) fn truncating_demonstrative(lemma: &str) -> Option<String> {
    for ending in words(Role::TruncatingDemonstrative) {
        if lemma.ends_with(ending.as_ref()) {
            return Some(lemma[..lemma.len() - 2].to_owned());
        }
    }
    None
}

/// Whether the base a suffix attaches to is a bound alternant of the `ajtó` class.
///
/// Such a stem ends in the `a`/`e` the change produced, which is not a free final vowel, so
/// `NOM-LENG-001` does not lengthen it: `szüle` plus the possessed plural's `-i` is `szülei`.
///
/// Rules: `NOM-STEM-013`, `NOM-LENG-001`
fn is_possessive_alternant(entry: &Entry, morph: Morph, at_lemma: bool) -> bool {
    if !at_lemma {
        return false;
    }
    match morph {
        Morph::Possessive { .. } => entry.has(Trait::PossessiveAlt),
        Morph::PossessedPlural => entry.has(Trait::PossessedPluralAlt),
        _ => false,
    }
}

/// The stem a vowel-initial suffix sees, when the lemma alternates.
///
/// The `-a`/`-ja` choice is made against the stem the suffix actually attaches to, not against the
/// lemma. A v-stem is the case that shows it: `kő` is vowel-final and would take `-je`, but the
/// suffix attaches to `köv`, which is consonant-final and takes `-e` (`köve`, `tava`, `lova`).
///
/// Rules: `NOM-POSS-004`, `NOM-STEM-008`, `NOM-STEM-010`
fn bound_for_vowel(entry: &Entry, at_lemma: bool) -> Option<String> {
    if !at_lemma {
        return None;
    }
    let probe = crate::suffix::case_suffix(Case::Acc)?;
    let bound = alternant(entry, &probe);
    (bound != entry.lemma.as_ref()).then(|| bound.into_owned())
}

/// The `ajtó`-type stem, when this slot is the one that sees it.
///
/// `NOM-STEM-013` confines the alternation to the third person possessive on the bare lemma, so
/// `ideje` and `idejük` alternate while `időm`, `időt` and `idők` do not. `NOM-STEM-014` truncates
/// in the same slot.
///
/// Rules: `NOM-STEM-013`, `NOM-STEM-014`
fn possessive_alt_stem(entry: &Entry, person: Person, at_lemma: bool) -> Option<String> {
    if !at_lemma || person != Person::Third {
        return None;
    }
    if entry.has(Trait::PossessiveAlt) {
        return crate::alternation::possessive_alt(&entry.lemma);
    }
    // NOM-STEM-014: the kinship stems truncate instead, `apa` giving `apja` and `apjuk`.
    if crate::possessive::kinship_truncates(entry) {
        return crate::alternation::syncopated(&entry.lemma);
    }
    None
}

/// Whether an epenthetic verb loses its unstable vowel before `suffix`.
///
/// `VRB-STEM-007` makes syncope obligatory when the consonant after the unstable vowel is `g`, `z`
/// or `m`, and item-specific when it is `l` or `r`: `érez` gives `érzem` categorically while
/// `énekel` keeps its vowel (`énekelek` against `éneklek`). Only the obligatory side is computed
/// here; the `l`/`r` items keep the full stem, which is what they are attested with.
///
/// The corpus refines the rule: 21 of the 199 attested `g`/`z`/`m` verbs keep the full stem in the
/// present although they syncopate elsewhere, and they carry [`Trait::PresentFullStem`] for it.
///
/// The trigger is a vowel-initial ending, which is `VRB-STEM-007`'s list read off its shape: the
/// vowel drops before `-Ok`, `-Unk`, `-Om`, `-Od` and the front definite `-i`, and stays before
/// `-sz`, `-tOk`, `-nAk`, `-ni` and the `j`-initial definite endings (`pótolja`, `töröljük`).
///
/// Rules: `VRB-STEM-007`
fn epenthetic_verb_syncope(entry: &Entry, suffix: &Suffix, cell: SyncopeCell) -> Option<String> {
    if entry.pos != Pos::Verb || !entry.has(Trait::Epenthetic) || !vowel_initial(suffix) {
        return None;
    }
    // The cells are measured and marked apart, because they disagree for a third of the class.
    let (full, sync) = match cell {
        SyncopeCell::Definite => (Trait::DefiniteFullStem, Trait::DefiniteSyncope),
        SyncopeCell::Present => (Trait::PresentFullStem, Trait::PresentSyncope),
        SyncopeCell::Past => (Trait::PastFullStem, Trait::PastSyncope),
    };
    if entry.has(full) {
        return None;
    }
    let computed = matches!(
        letters(&entry.lemma).last().map(|s| s.letter),
        Some(Letter::G | Letter::Z | Letter::M)
    );
    (computed || entry.has(sync))
        .then(|| crate::alternation::syncopated(&entry.lemma))
        .flatten()
}

/// Which cell of the epenthetic present syncope is being generated.
///
/// `VRB-STEM-007` is refined per cell, and the cells disagree: `jegyez` syncopates in the past
/// (`jegyzett`) and `énekel` in the definite present (`énekli`) but not the indefinite.
#[derive(Clone, Copy, PartialEq, Eq)]
enum SyncopeCell {
    /// Before the definite `-i`.
    Definite,
    /// Before a vowel-initial indefinite present ending.
    Present,
    /// Before the past marker.
    Past,
}

/// Whether a suffix begins with a vowel, counting a literal one.
///
/// [`Suffix::starts_with_vowel`] asks only about an archiphoneme, which is what `alternant`'s gate
/// needs; the definite `-i` and `-ik` are literals and still meet the stem as vowels.
fn vowel_initial(suffix: &Suffix) -> bool {
    match suffix.template.first() {
        Some(Seg::Var(_)) => true,
        Some(Seg::Lit(text)) => letters(text).next().is_some_and(|s| s.letter.is_vowel()),
        _ => false,
    }
}

/// The personal infinitive of `VRB-INF-002`, built on the infinitive itself.
///
/// The first and second persons replace the infinitive's final `i` (`tanulni` gives `tanulnom`),
/// the third persons keep it (`tanulnia`, `tanulniuk`). Building on the infinitive rather than on
/// the stem is what makes the irregular infinitives fall out: `lenni` gives `lennem` and `lennie`,
/// `aludni` gives `aludnom`.
///
/// Rules: `VRB-INF-002`
fn personal_infinitive(infinitive: &str, entry: &Entry, person: Person, number: Number) -> String {
    use crate::suffix::{Arch, Seg::Lit, Seg::Var};
    // The harmony comes from the stem the infinitive was built on, not from the infinitive, whose
    // neutral `-ni` would flatten the rounding (`ülnöm`), and not from the lemma, which a
    // suppletive stem contradicts (`VRB-IRR-001`: `van` gives `lennem`, not `*lennom`).
    let h = harmony(infinitive.strip_suffix("ni").unwrap_or(infinitive), entry);
    let (keeps_i, segs): (bool, &[Seg]) = match (person, number) {
        (Person::First, Number::Sing) => (false, &[Var(Arch::O), Lit("m")]),
        (Person::Second, Number::Sing) => (false, &[Var(Arch::O), Lit("d")]),
        (Person::Third, Number::Sing) => (true, &[Var(Arch::A)]),
        (Person::First, Number::Plur) => (false, &[Var(Arch::U), Lit("nk")]),
        (Person::Second, Number::Plur) => {
            (false, &[Var(Arch::O), Lit("t"), Var(Arch::O), Lit("k")])
        }
        (Person::Third, Number::Plur) => (true, &[Var(Arch::U), Lit("k")]),
    };
    let base = if keeps_i {
        infinitive
    } else {
        infinitive.strip_suffix('i').unwrap_or(infinitive)
    };
    format!("{base}{}", crate::pronoun::realise(segs, h))
}

/// The past indicative indefinite of `stem`.
///
/// Rules: `VRB-PAST-001`, `VRB-PAST-011`
fn past_form(
    entry: &Entry,
    stem: &str,
    person: Person,
    number: Number,
    definite: Definite,
    lowering: bool,
) -> String {
    match crate::verbal::past_person(person, number, definite) {
        None => {
            let marker = crate::verbal::past_final(stem, entry);
            let base = past_stem(entry, stem, &marker);
            attach(&base, entry, &marker, lowering)
        }
        Some(p) => {
            let marker = crate::verbal::past_nonfinal(stem, entry);
            let base = past_stem(entry, stem, &marker);
            let past = attach(&base, entry, &marker, lowering);
            attach(&past, entry, &p, lowering)
        }
    }
}

/// The stem the past marker attaches to, syncopated where this cell is measured to syncopate.
///
/// The past is a cell of `VRB-STEM-007` like the two present ones, and it disagrees with them:
/// `jegyez` gives `jegyzett` 13,079 against `jegyezett` 206 `[WC2]` while keeping its vowel in the
/// infinitive.
///
/// Rules: `VRB-STEM-007`, `VRB-PAST-001`
fn past_stem(entry: &Entry, stem: &str, marker: &Suffix) -> String {
    epenthetic_verb_syncope(entry, marker, SyncopeCell::Past).unwrap_or_else(|| stem.to_owned())
}

#[cfg(test)]
mod compound_generation {
    use super::generate_compound;
    use crate::features::{Case, Morph};

    #[test]
    fn der_comp_008_a_compound_inflects_as_its_head() {
        for (lemma, stack, want) in [
            ("vashíd", vec![Morph::Case(Case::Acc)], "vashidat"),
            ("vashíd", vec![Morph::Plural], "vashidak"),
            ("balkéz", vec![Morph::Plural], "balkezek"),
            ("virágbokor", vec![Morph::Plural], "virágbokrok"),
        ] {
            assert_eq!(
                generate_compound(lemma, &stack).as_deref(),
                Ok(want),
                "{lemma}"
            );
        }
    }

    #[test]
    fn der_comp_008_a_compound_with_no_listed_head_is_refused() {
        assert!(generate_compound("qqqxxx", &[Morph::Plural]).is_err());
    }
}

#[cfg(test)]
mod frequentative {
    use super::generate;
    use crate::derivation::DerId;
    use crate::features::Morph;
    use picur_morph_lexicon::lexicon;

    fn der(lemma: &str) -> Vec<String> {
        lexicon()
            .get(lemma)
            .iter()
            .filter_map(|e| generate(e, &[Morph::Derive(DerId::Ogat)]).ok())
            .collect()
    }

    #[test]
    fn der_verb_013_a_monosyllabic_stem_takes_the_linking_vowel() {
        for (lemma, want) in [
            ("ad", "adogat"),
            ("vált", "váltogat"),
            ("tép", "tépeget"),
            ("főz", "főzöget"),
            ("köt", "kötöget"),
            ("üt", "ütöget"),
            ("lép", "lépeget"),
            // The stem decides, not the lemma: `iszik` is written with two vowels, `isz` has one.
            ("iszik", "iszogat"),
            ("eszik", "eszeget"),
            ("tesz", "teszeget"),
            ("nő", "növöget"),
        ] {
            assert!(
                der(lemma).iter().any(|g| g == want),
                "{lemma} gave {:?}",
                der(lemma)
            );
        }
    }

    #[test]
    fn der_verb_013_a_polysyllabic_stem_takes_none() {
        for (lemma, want) in [
            ("olvas", "olvasgat"),
            ("sétál", "sétálgat"),
            ("beszél", "beszélget"),
            ("söpör", "söpörget"),
            ("hímez", "hímezget"),
            ("udvarol", "udvarolgat"),
        ] {
            assert!(
                der(lemma).iter().any(|g| g == want),
                "{lemma} gave {:?}",
                der(lemma)
            );
        }
    }
}

#[cfg(test)]
mod past_conditional {
    use super::generate;
    use crate::features::{Definite, Mood, Morph, Number, Person, Tense};
    use picur_morph_lexicon::lexicon;

    fn cond_past(lemma: &str, person: Person, number: Number, definite: Definite) -> Vec<String> {
        lexicon()
            .get(lemma)
            .iter()
            .filter_map(|e| {
                generate(
                    e,
                    &[Morph::Verbal {
                        mood: Mood::Cnd,
                        tense: Tense::Past,
                        person,
                        number,
                        definite,
                    }],
                )
                .ok()
            })
            .collect()
    }

    #[test]
    fn vrb_cond_004_the_past_conditional_is_the_past_plus_volna() {
        for (lemma, person, number, definite, want) in [
            (
                "kell",
                Person::Third,
                Number::Sing,
                Definite::Ind,
                "kellett volna",
            ),
            (
                "lát",
                Person::First,
                Number::Plur,
                Definite::Def,
                "láttuk volna",
            ),
            (
                "vár",
                Person::First,
                Number::Sing,
                Definite::Ind,
                "vártam volna",
            ),
            (
                "vár",
                Person::Second,
                Number::Sing,
                Definite::Def,
                "vártad volna",
            ),
            (
                "megy",
                Person::First,
                Number::Sing,
                Definite::Ind,
                "mentem volna",
            ),
            (
                "eszik",
                Person::Third,
                Number::Sing,
                Definite::Ind,
                "evett volna",
            ),
        ] {
            assert!(
                cond_past(lemma, person, number, definite)
                    .iter()
                    .any(|g| g == want),
                "{lemma} gave {:?}",
                cond_past(lemma, person, number, definite)
            );
        }
    }

    #[test]
    fn vrb_cond_004_the_copula_builds_on_lesz() {
        // `lett volna` 245,186 against `volt volna` 535: the `volt` form is a literary archaism.
        assert!(
            cond_past("van", Person::First, Number::Sing, Definite::Ind)
                .iter()
                .any(|g| g == "lettem volna")
        );
        assert!(
            cond_past("van", Person::Third, Number::Sing, Definite::Ind)
                .iter()
                .any(|g| g == "lett volna")
        );
    }
}

#[cfg(test)]
mod name_tests {
    use picur_morph_lexicon::{Entry, Pos};

    use super::generate;
    use crate::features::{Case, Morph};

    fn form(lemma: &str, case: Case) -> Result<String, super::GenError> {
        generate(&Entry::new(lemma, Pos::Noun), &[Morph::Case(case)])
    }

    #[test]
    fn ort_name_012_a_doubled_final_takes_a_hyphen_on_a_name_only() {
        assert_eq!(form("Neumann", Case::Ins).as_deref(), Ok("Neumann-nal"));
        assert_eq!(form("Bonn", Case::Dat).as_deref(), Ok("Bonn-nak"));
        assert_eq!(form("Kiss", Case::Ins).as_deref(), Ok("Kiss-sel"));
        assert_eq!(form("Papp", Case::Tra).as_deref(), Ok("Papp-pá"));
        assert_eq!(form("Bükk", Case::Ins).as_deref(), Ok("Bükk-kel"));
        assert_eq!(form("Kiss", Case::Dat).as_deref(), Ok("Kissnek"));
        assert_eq!(form("Neumann", Case::Ine).as_deref(), Ok("Neumannban"));
        assert_eq!(form("toll", Case::Ins).as_deref(), Ok("tollal"));
    }

    #[test]
    fn ort_for_012_a_spoken_stem_suffixes_the_spelling_after_a_hyphen() {
        let lx = picur_morph_lexicon::lexicon();
        let f = |lemma: &str, stack: &[Morph]| {
            generate(lx.get_pos(lemma, Pos::Noun).expect("listed"), stack).expect("generates")
        };
        assert_eq!(f("Cambridge", &[Morph::Case(Case::Ine)]), "Cambridge-ben");
        assert_eq!(
            f("Shakespeare", &[Morph::Case(Case::Ins)]),
            "Shakespeare-rel"
        );
        assert_eq!(f("Glasgow", &[Morph::Case(Case::Ins)]), "Glasgow-val");
        assert_eq!(f("Rousseau", &[Morph::Case(Case::Dat)]), "Rousseau-nak");
        assert_eq!(f("Loire", &[Morph::Case(Case::Sup)]), "Loire-on");
        assert_eq!(f("Peugeot", &[Morph::Case(Case::Acc)]), "Peugeot-t");
        assert_eq!(f("Montreux", &[Morph::Case(Case::Ter)]), "Montreux-ig");
        assert_eq!(f("Voltaire", &[Morph::Anaphoric]), "Voltaire-é");
        assert_eq!(f("guillotine", &[Morph::Case(Case::Acc)]), "guillotine-t");
        assert_eq!(
            f("Cambridge", &[Morph::Derive(crate::derivation::DerId::I)]),
            "cambridge-i"
        );
        assert_eq!(f("Cambridge", &[]), "Cambridge");
    }

    #[test]
    fn ort_for_012_the_three_observable_facts_suffix_a_name_without_a_pronunciation() {
        use picur_morph_lexicon::Trait;
        let name = |lemma: &str, harmony: Trait, last: Option<&str>| {
            let mut e = Entry::new(lemma, Pos::Noun);
            e.traits = e.traits.with(Trait::Spoken).with(harmony);
            e.last_sound = last.map(Into::into);
            e
        };
        let f = |e: &Entry, stack: &[Morph]| generate(e, stack).expect("generates");
        let league = name("League", Trait::Front, Some("g"));
        assert_eq!(f(&league, &[Morph::Case(Case::Ine)]), "League-ben");
        assert_eq!(f(&league, &[Morph::Case(Case::Acc)]), "League-et");
        assert_eq!(f(&league, &[Morph::Case(Case::Ins)]), "League-gel");
        let college = name("College", Trait::Back, Some("dzs"));
        assert_eq!(f(&college, &[Morph::Case(Case::Acc)]), "College-ot");
        assert_eq!(f(&college, &[Morph::Case(Case::All)]), "College-hoz");
        let chelsea = name("Chelsea", Trait::Front, None);
        assert_eq!(f(&chelsea, &[Morph::Case(Case::Acc)]), "Chelsea-t");
        assert_eq!(f(&chelsea, &[Morph::Case(Case::Ins)]), "Chelsea-vel");
        let apple = name("Apple", Trait::FrontRounded, None);
        assert_eq!(f(&apple, &[Morph::Case(Case::All)]), "Apple-höz");
        assert_eq!(f(&apple, &[Morph::Case(Case::Ine)]), "Apple-ben");
        // Marked as spoken with no facts at all: the bare name stands, every suffix is refused.
        let mut cleveland = Entry::new("Cleveland", Pos::Noun);
        cleveland.traits = cleveland.traits.with(Trait::Spoken);
        assert_eq!(f(&cleveland, &[]), "Cleveland");
        assert!(generate(&cleveland, &[Morph::Case(Case::Ine)]).is_err());
    }

    #[test]
    fn ort_for_012_a_name_ending_in_a_listed_letter_group_is_refused() {
        for name in ["Braintree", "Moscow", "Bordeaux", "Renault"] {
            assert!(form(name, Case::Ine).is_err(), "{name}");
        }
        // A silent `e` is a fact about the name, not about its letters: `Goethe` says its `e`
        // (`ORT-FOR-010`), so no shape rule may refuse it, and `Cambridge` pays for that.
        for (name, want) in [
            ("Imre", "Imrében"),
            ("Newton", "Newtonban"),
            ("Goethe", "Goethében"),
            ("Győr", "Győrben"),
        ] {
            assert_eq!(form(name, Case::Ine).as_deref(), Ok(want), "{name}");
        }
    }
}

#[cfg(test)]
mod locative_tests {
    use picur_morph_lexicon::{Pos, lexicon};

    use super::{generate, generate_seam};
    use crate::derivation::DerId;
    use crate::features::{Case, Morph};

    #[test]
    fn nom_case_016_the_locative_is_t_after_a_vowel_and_ott_after_a_consonant() {
        let lx = lexicon();
        for (town, want) in [
            ("Pécs", "Pécsett"),
            ("Győr", "Győrött"),
            ("Vác", "Vácott"),
            ("Kaposvár", "Kaposvárott"),
        ] {
            let e = lx.get_pos(town, Pos::Noun).expect("listed");
            assert_eq!(generate(e, &[Morph::Case(Case::Loc)]).as_deref(), Ok(want));
        }
        let budapest = lx.get_pos("Budapest", Pos::Noun).expect("listed");
        assert!(generate(budapest, &[Morph::Case(Case::Loc)]).is_err());
    }

    #[test]
    fn a_concatenative_derivation_keeps_its_seam() {
        let lx = lexicon();
        let e = lx.get_pos("ház", Pos::Noun).expect("listed");
        let (form, seam) = generate_seam(e, &[Morph::Derive(DerId::Os)]).expect("generates");
        assert_eq!(form, "házas");
        assert_eq!(seam, Some("ház".len()));
    }
}
