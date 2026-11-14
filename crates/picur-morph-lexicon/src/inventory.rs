//! The closed word classes the rules name, read from `data/inventory.ron`.
//!
//! A rule that says "these words" is stated in the data file and nowhere else; the engine asks
//! for a list by its role.

use std::sync::LazyLock;

use crate::entry::Pos;

const INVENTORY: &str = include_str!("../data/inventory.ron");

/// What a list is for. The names are the `role:` values of the data file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    /// Kinship nouns truncating before the third person possessive.
    KinshipTruncating,
    /// Stems whose final `h` the norm may write as silent.
    SilentHStem,
    /// The postpositions that license the adjectival `-i`.
    PostpositionAdjective,
    /// Prefixes of the pronoun series.
    SeriesPrefix,
    /// The interrogatives the series prefixes to.
    InterrogativeBase,
    /// The identity and emphatic prefixes of the demonstratives (`ugyan-`, `csak-`, `épp-`).
    DemonstrativePrefix,
    /// The demonstratives those prefixes take.
    DemonstrativeBase,
    /// The universal prefix of the demonstratives (`mind-`).
    UniversalPrefix,
    /// The demonstratives the universal prefix takes beside its listed compounds.
    UniversalDemonstrativeBase,
    /// Short words admitted as compound members.
    ShortMember,
    /// Short words admitted as compound heads.
    ShortHead,
    /// Bound first members of compounds.
    BoundMember,
    /// Semi-words closing a compound, with their category.
    SemiWord,
    /// Monosyllabic verbs taking the causative `-tAt`.
    CausativeTat,
    /// Verbs with a suppletive causative.
    IrregularCausative,
    /// The sz-d verbs with a `v` stem.
    VStemVerb,
    /// The sz-d verbs whose potential and converb are also built on the short stem.
    ShortStemVerb,
    /// The long-vowel `t` stems whose imperative is written `ss`.
    ImperativeSs,
    /// Archaic name-final letter groups and the consonant they are pronounced as.
    ArchaicNameFinal,
    /// The same, for the groups only a name flagged `ArchaicSpelling` reads this way.
    ArchaicNameFinalMarked,
    /// Adjectives with a suppletive comparative.
    IrregularComparative,
    /// Adjectives whose regular comparative stands beside the listed irregular one.
    ComparativeDoublet,
    /// Words `leg-` attaches to directly.
    NonComparativeSuperlative,
    /// Adjectives deleting their final vowel before `-An`.
    VowelDeletingAdjective,
    /// Possessed settlement-name endings.
    PossessivePlaceEnding,
    /// The ordinals not built on a numeral stem.
    SuppletiveOrdinal,
    /// The demonstrative adjectives that truncate before a suffix.
    TruncatingDemonstrative,
    /// Ordinal-like words beside the numeric ordinals.
    OrdinalExtra,
    /// Fraction stems beside the numeric ones.
    FractionExtra,
    /// Symbols and the words they expand to.
    SymbolExpansion,
    /// Derivational tails written after an acronym's hyphen.
    AcronymDerivationalTail,
    /// The weekdays.
    Weekday,
    /// The months.
    Month,
    /// The months' written abbreviations.
    MonthAbbreviation,
    /// The lowercase generics of public place names (`ORT-NAME-030`).
    PublicPlace,
    /// Words that settle a hyphen between numbers as a range.
    RangeCue,
    /// Accent-free function words that mark Hungarian text.
    AccentSignature,
    /// Unit symbols a number is spaced from.
    UnitSymbol,
    /// Currency units that follow the amount.
    CurrencyUnit,
    /// The personal pronouns' nominatives: person, number, form.
    PronounNominative,
    /// The personal pronouns' accusatives: person, number, form.
    PronounAccusative,
    /// The oblique pronoun stems: case, stem.
    PronounStem,
    /// The oblique pronoun forms the suffix rule does not reach: case, person, number, form.
    PronounIrregular,
    /// The `-rA` forms of the demonstrative adverbs.
    DemonstrativeSublative,
    /// The independent possessive pronouns: lemma, then its forms.
    PossessivePronoun,
    /// The postpositions taking person suffixes: postposition, then its stems.
    PostpositionPersonal,
    /// The suppletive verbs, one row per verb.
    IrregularVerb,
    /// A lexeme with listed possessive forms: lemma, then the six person forms and the
    /// possessed plural.
    IrregularPossessive,
    /// The units: independent, attributive, ordinal.
    NumeralUnit,
    /// The tens by digit: independent, attributive, ordinal.
    NumeralTen,
    /// The bound stems of the teens and twenties.
    NumeralBoundTen,
    /// The hundred: independent, attributive, ordinal.
    NumeralHundred,
    /// The scale words: value, word, ordinal.
    NumeralScale,
    /// Suffix endings that are an invariant front harmony domain of their own.
    OpaqueFrontEnding,
    /// The decimal prefixes of the units (`kilo-`, `milli-`).
    UnitPrefix,
    /// The spoken names of the units a decimal prefix attaches to.
    UnitName,
    /// The Hungarian names of the letters.
    LetterName,
}

impl Role {
    const ALL: [Self; 57] = [
        Self::KinshipTruncating,
        Self::SilentHStem,
        Self::PostpositionAdjective,
        Self::SeriesPrefix,
        Self::InterrogativeBase,
        Self::DemonstrativePrefix,
        Self::DemonstrativeBase,
        Self::UniversalPrefix,
        Self::UniversalDemonstrativeBase,
        Self::ShortMember,
        Self::ShortHead,
        Self::BoundMember,
        Self::SemiWord,
        Self::CausativeTat,
        Self::IrregularCausative,
        Self::VStemVerb,
        Self::ShortStemVerb,
        Self::ImperativeSs,
        Self::ArchaicNameFinal,
        Self::ArchaicNameFinalMarked,
        Self::IrregularComparative,
        Self::ComparativeDoublet,
        Self::NonComparativeSuperlative,
        Self::VowelDeletingAdjective,
        Self::PossessivePlaceEnding,
        Self::SuppletiveOrdinal,
        Self::TruncatingDemonstrative,
        Self::OrdinalExtra,
        Self::FractionExtra,
        Self::SymbolExpansion,
        Self::AcronymDerivationalTail,
        Self::Weekday,
        Self::Month,
        Self::MonthAbbreviation,
        Self::PublicPlace,
        Self::RangeCue,
        Self::AccentSignature,
        Self::UnitSymbol,
        Self::CurrencyUnit,
        Self::PronounNominative,
        Self::PronounAccusative,
        Self::PronounStem,
        Self::PronounIrregular,
        Self::DemonstrativeSublative,
        Self::PossessivePronoun,
        Self::PostpositionPersonal,
        Self::IrregularVerb,
        Self::IrregularPossessive,
        Self::NumeralUnit,
        Self::NumeralTen,
        Self::NumeralBoundTen,
        Self::NumeralHundred,
        Self::NumeralScale,
        Self::OpaqueFrontEnding,
        Self::UnitPrefix,
        Self::UnitName,
        Self::LetterName,
    ];

    fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| format!("{r:?}") == name)
    }
}

/// One list of the data file.
#[derive(Debug, Default)]
struct List {
    words: Vec<Box<str>>,
    pairs: Vec<(Box<str>, Box<str>)>,
    entries: Vec<(Box<str>, Pos)>,
    rows: Vec<Box<[Box<str>]>>,
    rules: Vec<Box<str>>,
}

/// The quoted strings of a bracketed list, in order, with `\u{XXXX}` escapes resolved.
fn quoted(inner: &str) -> Vec<Box<str>> {
    inner
        .split('"')
        .skip(1)
        .step_by(2)
        .map(|s| unescaped(s).into_boxed_str())
        .collect()
}

/// `text` with every `\u{XXXX}` replaced by its character.
fn unescaped(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(i) = rest.find("\\u{") {
        out.push_str(&rest[..i]);
        let after = &rest[i + 3..];
        let Some(end) = after.find('}') else {
            out.push_str(&rest[i..]);
            return out;
        };
        if let Some(c) = u32::from_str_radix(&after[..end], 16)
            .ok()
            .and_then(char::from_u32)
        {
            out.push(c);
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

/// The lists of one data file, by role name.
///
/// The lexicon's own file is read through [`Role`]; another crate's file of the same shape is
/// read through this, with its own names.
#[derive(Debug)]
pub struct Tables {
    lists: Vec<(Box<str>, List)>,
}

impl Tables {
    /// Parses a data file of `(role:Name, words|pairs|entries|rows:[...])` rows.
    #[must_use]
    pub fn parse(text: &str) -> Self {
        let mut lists: Vec<(Box<str>, List)> = text
            .lines()
            .map(str::trim)
            .filter(|l| l.starts_with("(role:"))
            .filter_map(|row| {
                let name = row.split("role:").nth(1)?.split([',', ')']).next()?.trim();
                Some((Box::from(name), parse_list(row)))
            })
            .collect();
        lists.sort_by(|a, b| a.0.cmp(&b.0));
        Self { lists }
    }

    fn list(&self, role: &str) -> &List {
        let i = self.lists.partition_point(|(r, _)| r.as_ref() < role);
        self.lists
            .get(i)
            .filter(|(r, _)| r.as_ref() == role)
            .map_or_else(|| panic!("no list for {role}"), |(_, l)| l)
    }

    /// The words of a plain list.
    #[must_use]
    pub fn words(&self, role: &str) -> &[Box<str>] {
        &self.list(role).words
    }

    /// The pairs of a list.
    #[must_use]
    pub fn pairs(&self, role: &str) -> &[(Box<str>, Box<str>)] {
        &self.list(role).pairs
    }

    /// The rows of a table.
    #[must_use]
    pub fn rows(&self, role: &str) -> &[Box<[Box<str>]>] {
        &self.list(role).rows
    }

    /// The rules a list cites in its `rules:` field.
    #[must_use]
    pub fn rules(&self, role: &str) -> &[Box<str>] {
        &self.list(role).rules
    }

    /// The role names the file states, sorted.
    #[must_use]
    pub fn roles(&self) -> Vec<&str> {
        self.lists.iter().map(|(r, _)| r.as_ref()).collect()
    }
}

/// The bracketed value of `key` in a row.
fn bracketed<'a>(row: &'a str, key: &str) -> Option<&'a str> {
    let i = row.find(key)? + key.len();
    let rest = row[i..].strip_prefix('[')?;
    Some(&rest[..rest.rfind(']')?])
}

/// The list a row states, whichever shape it has.
fn parse_list(row: &str) -> List {
    let mut list = List::default();
    if let Some(i) = row.find("rules:\"") {
        let rest = &row[i + 7..];
        if let Some(end) = rest.find('"') {
            list.rules = rest[..end].split_whitespace().map(Box::from).collect();
        }
    }
    if let Some(inner) = bracketed(row, "words:") {
        list.words = quoted(inner);
    } else if let Some(inner) = bracketed(row, "pairs:") {
        list.pairs = quoted(inner)
            .chunks(2)
            .filter_map(|c| Some((c.first()?.clone(), c.get(1)?.clone())))
            .collect();
    } else if let Some(inner) = bracketed(row, "rows:") {
        list.rows = inner
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split("],[")
            .map(|r| quoted(r).into_boxed_slice())
            .collect();
    } else if let Some(inner) = bracketed(row, "entries:") {
        list.entries = inner
            .split("(word:")
            .skip(1)
            .filter_map(|e| {
                let word = quoted(e).into_iter().next()?;
                let pos = e.split("pos:").nth(1)?.split([')', ',']).next()?.trim();
                Some((
                    word,
                    Pos::ALL.into_iter().find(|p| format!("{p:?}") == pos)?,
                ))
            })
            .collect();
    }
    list
}

static TABLES: LazyLock<Tables> = LazyLock::new(|| {
    let t = Tables::parse(INVENTORY);
    for name in t.roles() {
        assert!(
            Role::parse(name).is_some(),
            "inventory.ron names no role {name}"
        );
    }
    t
});

/// The lists by role, in `Role::ALL` order, resolved once.
static BY_ROLE: LazyLock<Vec<&'static List>> = LazyLock::new(|| {
    Role::ALL
        .iter()
        .map(|r| TABLES.list(&format!("{r:?}")))
        .collect()
});

fn list(role: Role) -> &'static List {
    let at = Role::ALL
        .iter()
        .position(|r| *r == role)
        .unwrap_or_else(|| unreachable!("every role is in ALL"));
    BY_ROLE[at]
}

/// The words of a plain list.
#[must_use]
pub fn words(role: Role) -> &'static [Box<str>] {
    &list(role).words
}

/// Whether `word` is in the list.
#[must_use]
pub fn lists(role: Role, word: &str) -> bool {
    words(role).iter().any(|w| w.as_ref() == word)
}

/// The pairs of a word and its suppletive form.
#[must_use]
pub fn pairs(role: Role) -> &'static [(Box<str>, Box<str>)] {
    &list(role).pairs
}

/// The rows of a table, each a sequence of cells in the order the file documents.
#[must_use]
pub fn rows(role: Role) -> &'static [Box<[Box<str>]>] {
    &list(role).rows
}

/// The words of a list with their category.
#[must_use]
pub fn entries(role: Role) -> &'static [(Box<str>, Pos)] {
    &list(role).entries
}

#[cfg(test)]
mod tests {
    use super::{Role, entries, lists, pairs, rows, words};
    use crate::entry::Pos;

    #[test]
    fn every_role_is_stated_once() {
        for role in Role::ALL {
            let l = super::list(role);
            assert!(
                !l.words.is_empty()
                    || !l.pairs.is_empty()
                    || !l.entries.is_empty()
                    || !l.rows.is_empty(),
                "{role:?}"
            );
        }
    }

    #[test]
    fn the_three_shapes_read() {
        assert!(lists(Role::KinshipTruncating, "atya"));
        assert_eq!(words(Role::VowelDeletingAdjective).len(), 4);
        assert!(
            pairs(Role::IrregularComparative)
                .iter()
                .any(|(w, c)| (w.as_ref(), c.as_ref()) == ("szép", "szebb"))
        );
        assert!(entries(Role::SemiWord).contains(&("szerte".into(), Pos::Adverb)));
        assert_eq!(rows(Role::IrregularVerb)[0].len(), 15);
        assert_eq!(rows(Role::NumeralUnit).len(), 10);
    }
}
