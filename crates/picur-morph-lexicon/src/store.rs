//! The committed lexicon, compiled into the binary.
//!
//! Data is embedded with `include_str!` and parsed once on first use. Lookup is a sorted `Vec` with
//! `partition_point`, not a hash map: it keeps the data compact, gives ordered iteration for the
//! stem index that analysis needs, and stays `wasm` friendly.

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::{Entry, Pos, Trait, TraitSet};

/// Converted source files, one per part of speech present in the committed data.
///
/// A file the converter writes but this list omits is silently invisible at runtime, so a test
/// pins the two lists together.
const SOURCES: &[(&str, &str)] = &[
    ("noun", include_str!("../data/noun.ron")),
    ("adjective", include_str!("../data/adjective.ron")),
    ("verb", include_str!("../data/verb.ron")),
    ("adverb", include_str!("../data/adverb.ron")),
    ("postposition", include_str!("../data/postposition.ron")),
    ("preverb", include_str!("../data/preverb.ron")),
    ("interjection", include_str!("../data/interjection.ron")),
    ("conjunction", include_str!("../data/conjunction.ron")),
];

/// Hand-curated entries, applied on top of [`SOURCES`].
///
/// The rows here **replace** every converted row of their lemma and part of speech, because the
/// specification arbitrates the data: where the rulebook states a lexical property the conversion
/// did not carry, this is where it is stated. Two rows of one lemma and part of speech are two
/// lexemes, which only an abbreviation and the word it shares its letters with may be (`máj.`
/// 'May' beside `máj` 'liver'). Every row needs a rule reference in the file.
const SUPPLEMENT: &str = include_str!("../data/supplement.ron");

/// Reads `name:value` from one RON row, without pulling in a parser.
///
/// The rows are machine-written by `cargo xtask morphdb` in a fixed shape, so a hand-rolled reader
/// is both smaller and faster than a general deserializer, and it cannot silently accept a row the
/// generator would not have produced.
fn field<'a>(row: &'a str, name: &str) -> Option<&'a str> {
    let key = format!("{name}:");
    let i = row.find(&key)? + key.len();
    let rest = &row[i..];
    let end = if let Some(inner) = rest.strip_prefix('"') {
        inner.find('"')? + 2
    } else if rest.starts_with('[') {
        rest.find(']')? + 1
    } else {
        // A bare value ends at the next separator. Both must be checked: a row with no trailing
        // field ends `pos:Noun),`, so cutting only at the comma would yield `Noun)`.
        rest.find([',', ')']).unwrap_or(rest.len())
    };
    Some(&rest[..end])
}

fn unquote(s: &str) -> &str {
    s.trim_matches('"')
}

fn parse_pos(s: &str) -> Option<Pos> {
    Pos::ALL.into_iter().find(|p| format!("{p:?}") == s)
}

fn parse_trait(s: &str) -> Option<Trait> {
    Trait::ALL.into_iter().find(|t| format!("{t:?}") == s)
}

fn parse_row(row: &str) -> Option<Entry> {
    let lemma = unquote(field(row, "lemma")?);
    let pos = parse_pos(field(row, "pos")?)?;
    let parts: Box<[u32]> = field(row, "parts")
        .map(|p| {
            p.trim_matches(['[', ']'])
                .split(',')
                .filter_map(|n| n.trim().parse().ok())
                .collect()
        })
        .unwrap_or_default();
    let traits = field(row, "traits").map_or(TraitSet::EMPTY, |t| {
        t.trim_matches(['[', ']'])
            .split(',')
            .filter_map(|n| parse_trait(n.trim()))
            .collect::<TraitSet>()
    });
    let alt = field(row, "alt").map(|a| unquote(a).into());
    let last_sound = field(row, "last_sound").map(|a| unquote(a).into());
    Some(Entry {
        lemma: lemma.into(),
        parts,
        pos,
        traits,
        alt,
        last_sound,
    })
}

/// Entries sorted by lemma, then part of speech.
#[derive(Debug)]
pub struct Lexicon {
    entries: Vec<Entry>,
}

impl Lexicon {
    /// Every entry for `lemma`, in part-of-speech order. Empty when the lemma is unknown.
    pub fn get(&self, lemma: &str) -> &[Entry] {
        let start = self.entries.partition_point(|e| e.lemma.as_ref() < lemma);
        let end = self.entries.partition_point(|e| e.lemma.as_ref() <= lemma);
        &self.entries[start..end]
    }

    /// The first entry for `lemma` with this part of speech.
    pub fn get_pos(&self, lemma: &str, pos: Pos) -> Option<&Entry> {
        self.get(lemma).iter().find(|e| e.pos == pos)
    }

    /// Whether `lemma` is listed with this part of speech.
    pub fn has_pos(&self, lemma: &str, pos: Pos) -> bool {
        self.get_pos(lemma, pos).is_some()
    }

    /// Every entry, in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter()
    }

    /// The entry at a position in the sorted order, as handed out by the stem index.
    #[must_use]
    pub fn at(&self, i: usize) -> Option<&Entry> {
        self.entries.get(i)
    }

    /// Number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the lexicon is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Every parsable row of a source file.
fn rows(text: &str) -> impl Iterator<Item = Entry> {
    text.lines().filter_map(|l| {
        let l = l.trim();
        l.starts_with("(lemma:").then(|| parse_row(l)).flatten()
    })
}

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let supplement: Vec<Entry> = rows(SUPPLEMENT).collect();
    let replaced: std::collections::HashSet<(&str, Pos)> = supplement
        .iter()
        .map(|e| (e.lemma.as_ref(), e.pos))
        .collect();
    let mut entries: Vec<Entry> = SOURCES
        .iter()
        .flat_map(|(_, text)| rows(text))
        .filter(|e| !replaced.contains(&(e.lemma.as_ref(), e.pos)))
        .chain(supplement.iter().cloned())
        .collect();
    entries.sort_by(|a, b| a.lemma.cmp(&b.lemma).then(a.pos.cmp(&b.pos)));
    let mut lx = Lexicon { entries };
    inherit_heads(&mut lx);
    pool_past_classes(&mut lx);
    mark_silent_h(&mut lx);
    lx
});

/// Marks the stems whose final `h` the norm may write as silent.
///
/// `NOM-STEM-004` names nine such stems and `DER-COMP-008` makes the last member of a compound the
/// domain of every stem rule, so a compound closed by one of them is marked too (`szarvasméh`).
fn mark_silent_h(lx: &mut Lexicon) {
    let silent = crate::inventory::words(crate::Role::SilentHStem);
    for e in &mut lx.entries {
        if e.lemma.ends_with('h') && silent.iter().any(|w| e.lemma.ends_with(w.as_ref())) {
            e.traits = e.traits.with(Trait::SilentH);
        }
    }
}

/// Marks a verb's past marker optional where its preverb family attests both.
///
/// `ORT-TOG-026` makes a prefixed verb inflect as its base, so the family is one paradigm and the
/// converted rows are independent measurements of it. Where they disagree on the word-final past
/// marker, `tagad` with `-t` against `letagad`, `megtagad`, `eltagad` and `kitagad` with `-Ott`,
/// both are attested and neither row can be called the wrong one, which is exactly what
/// `Trait::AlternationOptional` records for a single verb (`VRB-PAST-001`, `VRB-PAST-008`).
fn pool_past_classes(lx: &mut Lexicon) {
    let class = |e: &Entry| (e.has(Trait::PastFinalT), e.has(Trait::PastFinalVtt));
    let mut families: HashMap<Box<str>, Vec<usize>> = HashMap::new();
    for (i, e) in lx.entries.iter().enumerate() {
        if e.pos != Pos::Verb {
            continue;
        }
        let base = match e.parts.first() {
            Some(&at) if e.has(Trait::Preverb) => e.lemma.get(at as usize..),
            None => Some(e.lemma.as_ref()),
            Some(_) => None,
        };
        if let Some(base) = base {
            families.entry(base.into()).or_default().push(i);
        }
    }
    for members in families.into_values() {
        let (t, ott) = members
            .iter()
            .map(|&i| class(&lx.entries[i]))
            .fold((false, false), |(t, ott), (a, b)| (t || a, ott || b));
        if t && ott {
            for i in members {
                let e = &mut lx.entries[i];
                e.traits = e.traits.with(Trait::AlternationOptional);
            }
        }
    }
}

/// Gives a compound the stem properties of its last member where it records none of its own.
///
/// `DER-COMP-008` makes the last member the domain of every stem rule, and `DER-COMP-009` has
/// the compound inflect as that member does: `szekrényajtó` alternates like `ajtó`
/// (`szekrényajtaja`) and `virágbokor` drops its vowel like `bokor` (`virágbokrok`). The converted
/// lexicon records this for some compounds and leaves 7,922 nouns unmarked, so the member's
/// marking is copied where the compound has none, alternant included.
fn inherit_heads(lx: &mut Lexicon) {
    let inherited: Vec<(usize, TraitSet, Option<Box<str>>)> = lx
        .entries
        .iter()
        .enumerate()
        .filter(|(_, e)| e.traits.is_empty() && e.alt.is_none())
        .filter_map(|(i, e)| {
            let at = *e.parts.last()? as usize;
            let head = lx.get_pos(e.lemma.get(at..)?, e.pos)?;
            (!head.traits.is_empty() || head.alt.is_some())
                .then(|| (i, head.traits, head.alt.clone()))
        })
        .collect();
    for (i, traits, alt) in inherited {
        lx.entries[i].traits = traits;
        lx.entries[i].alt = alt;
    }
}

/// The committed lexicon.
pub fn lexicon() -> &'static Lexicon {
    &LEXICON
}

/// Word frequencies, as half-octave buckets rather than counts.
///
/// A bucket is enough for every use the library has: ranking candidate corrections and restorations
/// needs an order, not a count, and a bucket keeps the file small and the numbers stable across
/// corpus revisions. Frequencies are facts about a corpus, not adapted material, so they are
/// committed where the lexicon itself could not be.
static FREQUENCY: LazyLock<Vec<(Box<str>, u8)>> = LazyLock::new(|| {
    let text = include_str!("../data/frequency.ron");
    let mut out: Vec<(Box<str>, u8)> = text
        .lines()
        .filter_map(|l| {
            let l = l.trim().trim_end_matches(',');
            let inner = l.strip_prefix('(')?.strip_suffix(')')?;
            let (w, n) = inner.rsplit_once(',')?;
            Some((w.trim().trim_matches('"').into(), n.trim().parse().ok()?))
        })
        .collect();
    out.sort();
    out
});

/// The frequency bucket of a written form, or 0 when it is not in the list.
///
/// Higher is commoner.
#[must_use]
pub fn frequency(word: &str) -> u8 {
    let f = &*FREQUENCY;
    let i = f.partition_point(|(w, _)| w.as_ref() < word);
    f.get(i)
        .filter(|(w, _)| w.as_ref() == word)
        .map_or(0, |(_, n)| *n)
}

#[cfg(test)]
mod tests {
    use super::lexicon;
    use crate::{Pos, Trait};

    #[test]
    fn the_supplement_has_no_duplicate_rows() {
        // A second row of one lexeme makes it two, so a duplicate splits a curated trait from its
        // lexeme without any error: a second `valaki` row with no traits gave `valakinak` beside
        // `valakinek`. Only an abbreviation may share its letters with a word.
        let mut seen: Vec<(String, Pos, bool)> = super::rows(super::SUPPLEMENT)
            .map(|e| (e.lemma.to_string(), e.pos, e.has(Trait::Abbreviation)))
            .collect();
        let before = seen.len();
        seen.sort();
        seen.dedup();
        assert_eq!(
            before,
            seen.len(),
            "supplement.ron has {} duplicate lemma and part-of-speech rows",
            before - seen.len()
        );
    }

    #[test]
    fn loads_a_substantial_lexicon() {
        let lx = lexicon();
        assert!(lx.len() > 100_000, "only {} entries", lx.len());
    }

    #[test]
    fn known_stems_carry_their_traits() {
        let lx = lexicon();
        let haz = lx.get_pos("ház", Pos::Noun).expect("ház");
        assert!(haz.has(Trait::Low));
        let gaz = lx.get_pos("gáz", Pos::Noun).expect("gáz");
        assert!(!gaz.has(Trait::Low));
        let bokor = lx.get_pos("bokor", Pos::Noun).expect("bokor");
        assert!(bokor.has(Trait::Epenthetic));
    }

    #[test]
    fn flagless_entries_survive_parsing() {
        // `stressz` is `stressz<TAB>NOUN,;` in morphdb: a row with no flags at all.
        assert!(lexicon().get_pos("stressz", Pos::Noun).is_some());
    }

    #[test]
    fn lookup_is_sorted_and_exact() {
        let lx = lexicon();
        assert!(lx.get("ház").iter().all(|e| e.lemma.as_ref() == "ház"));
        assert_eq!(lx.get("nonexistent-lemma-xyz"), []);
    }

    #[test]
    fn every_converted_file_is_loaded() {
        // A file written by `xtask morphdb` but missing from SOURCES is invisible at runtime, with no error.
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/data");
        let loaded: Vec<&str> = super::SOURCES.iter().map(|(n, _)| *n).collect();
        for entry in std::fs::read_dir(dir).expect("data dir") {
            let path = entry.expect("entry").path();
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if path.extension().is_none_or(|e| e != "ron") {
                continue;
            }
            // `supplement`, `frequency` and `inventory` are loaded by their own paths.
            if ["supplement", "frequency", "inventory"].contains(&stem) {
                continue;
            }
            assert!(
                loaded.contains(&stem),
                "{stem}.ron is written but never loaded"
            );
        }
    }

    #[test]
    fn covers_more_than_three_parts_of_speech() {
        let lx = lexicon();
        for pos in [Pos::Noun, Pos::Verb, Pos::Adverb, Pos::Postposition] {
            assert!(
                lx.iter().any(|e| e.pos == pos),
                "no entries for {pos:?}; the POS inventory must cover the language"
            );
        }
    }
}
