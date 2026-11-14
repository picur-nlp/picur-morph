//! Grades the engine against the goldens extracted from the rulebook.
//!
//! For each `input -> output` pair this tries every mechanism the engine offers and reports
//! whether any of them reproduces the output: generation over every feature stack, analysis for a
//! lemma, text repair, punctuation lint, the writing-system transforms, hyphenation and the
//! inventories. It measures "can the engine produce this form at all", not "does it pick the right
//! features", because the goldens carry no feature annotation.
//!
//! A golden whose input the lexicon does not list is generated on a bare entry, so a form needing
//! a lexical trait fails there. The gap between the two counts this prints is how much of the
//! language is lexical rather than computable.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use picur_morph_core::{
    Case, Definite, DerId, Mood, Morph, Number, Person, Stacks, Tense, Verdict, analyse, article,
    cardinal, forms, generate, generate_compound, ordinal, present_stem,
};
use picur_morph_lexicon::{Entry, Pos, lexicon};

use crate::Result;

const CASES: [Case; 20] = [
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
    Case::Distributivus,
    Case::Sociativus,
];

const PERSONS: [(Person, Number); 6] = [
    (Person::First, Number::Sing),
    (Person::Second, Number::Sing),
    (Person::Third, Number::Sing),
    (Person::First, Number::Plur),
    (Person::Second, Number::Plur),
    (Person::Third, Number::Plur),
];

/// Every feature stack the engine can generate.
pub(crate) fn stacks() -> Vec<Vec<Morph>> {
    let mut out = vec![
        vec![Morph::Plural],
        vec![Morph::Comparative],
        vec![Morph::Anaphoric],
        vec![Morph::Infinitive],
        vec![Morph::PresentParticiple],
        vec![Morph::PastParticiple],
        vec![Morph::Converb],
        vec![Morph::FutureParticiple],
        // `DER-ADJ-019` writes `-hAtÓ` as the potential plus the present participle, which is
        // the only route for an irregular verb: `ehető` is built on the potential stem `ehet`.
        vec![Morph::Potential, Morph::PresentParticiple],
        // `VRB-PART-004`: `-hAtAtlAn` is the privative on the potential stem.
        vec![Morph::Potential, Morph::Derive(DerId::Talan)],
        vec![Morph::Associative],
        vec![Morph::Collective],
        vec![Morph::Multiplicative],
        vec![Morph::Multiplicative, Morph::Derive(DerId::I)],
        vec![Morph::Multiplicative, Morph::Derive(DerId::Os)],
        vec![Morph::Plural, Morph::Anaphoric],
        vec![Morph::Anaphoric, Morph::Plural],
        vec![Morph::Derive(DerId::Oz), Morph::Derive(DerId::Ik)],
        vec![Morph::Comparative, Morph::Derive(DerId::Ik)],
    ];
    for id in DerId::ALL {
        out.push(vec![Morph::Derive(id)]);
        out.push(vec![Morph::Derive(id), Morph::Plural]);
        for c in CASES {
            out.push(vec![Morph::Derive(id), Morph::Case(c)]);
        }
    }
    for c in CASES {
        out.push(vec![Morph::Case(c)]);
        out.push(vec![Morph::Plural, Morph::Case(c)]);
        out.push(vec![Morph::Comparative, Morph::Case(c)]);
        out.push(vec![
            Morph::Comparative,
            Morph::Derive(DerId::Ik),
            Morph::Case(c),
        ]);
        out.push(vec![Morph::Associative, Morph::Case(c)]);
        out.push(vec![Morph::Anaphoric, Morph::Case(c)]);
        out.push(vec![Morph::Anaphoric, Morph::Plural, Morph::Case(c)]);
    }
    verbal_stacks(&mut out);
    possessive_stacks(&mut out);
    out
}

/// Every cell of the verbal paradigm.
fn verbal_cells() -> Vec<Morph> {
    let mut out = Vec::new();
    for (person, number) in PERSONS {
        for (mood, tense, definite) in [
            (Mood::Ind, Tense::Pres, Definite::Ind),
            (Mood::Ind, Tense::Pres, Definite::Def),
            (Mood::Ind, Tense::Past, Definite::Ind),
            (Mood::Ind, Tense::Past, Definite::Def),
            (Mood::Cnd, Tense::Pres, Definite::Ind),
            (Mood::Cnd, Tense::Pres, Definite::Def),
            // `VRB-COND-004`: the past conditional is a cell of its own, written as two words.
            (Mood::Cnd, Tense::Past, Definite::Ind),
            (Mood::Cnd, Tense::Past, Definite::Def),
            (Mood::Imp, Tense::Pres, Definite::Ind),
            (Mood::Imp, Tense::Pres, Definite::Def),
            (Mood::Ind, Tense::Pres, Definite::Two),
            (Mood::Ind, Tense::Past, Definite::Two),
            (Mood::Cnd, Tense::Pres, Definite::Two),
            (Mood::Imp, Tense::Pres, Definite::Two),
        ] {
            out.push(Morph::Verbal {
                mood,
                tense,
                person,
                number,
                definite,
            });
        }
    }
    out
}

/// The verbal cells, bare and under the stem-forming slots that precede them.
///
/// `VRB-MODEL-001`: derivation, the causative and the potential all precede mood and tense, so
/// `elolvastathatnánk` is the causative, the potential and the conditional in that order.
fn verbal_stacks(out: &mut Vec<Vec<Morph>>) {
    for cell in verbal_cells() {
        out.push(vec![cell]);
        out.push(vec![Morph::Potential, cell]);
        out.push(vec![
            Morph::Derive(DerId::Causative),
            Morph::Potential,
            cell,
        ]);
        for id in [
            DerId::Causative,
            DerId::Ogat,
            DerId::It,
            DerId::Oz,
            DerId::OdikV,
        ] {
            out.push(vec![Morph::Derive(id), cell]);
        }
    }
}

/// The possessive paradigm, alone and under what follows it.
fn possessive_stacks(out: &mut Vec<Vec<Morph>>) {
    for (person, number) in PERSONS {
        let poss = Morph::Possessive { person, number };
        out.push(vec![poss]);
        out.push(vec![Morph::PossessedPlural, poss]);
        // `NOM-ASSOC-001`: `anyámék`; `NOM-ANP-001`: `házamé`, `házaiméiból`.
        out.push(vec![poss, Morph::Associative]);
        out.push(vec![poss, Morph::Anaphoric]);
        out.push(vec![Morph::PossessedPlural, poss, Morph::Anaphoric]);
        for c in CASES {
            out.push(vec![poss, Morph::Case(c)]);
            out.push(vec![Morph::PossessedPlural, poss, Morph::Case(c)]);
            out.push(vec![poss, Morph::Anaphoric, Morph::Case(c)]);
            out.push(vec![
                Morph::PossessedPlural,
                poss,
                Morph::Anaphoric,
                Morph::Plural,
                Morph::Case(c),
            ]);
        }
    }
}

/// Rules whose suffixes the design never generates.
///
/// `DER-MODEL-012`: only a `P` suffix derives a new word; `R` and `L` describe words that exist,
/// and `P-dom` needs a domain feature the lexicon does not carry. Their goldens are reported apart
/// from real misses. Read from the `DER-MODEL-016` master table; a rule appears only when every
/// suffix it lists is non-generating.
///
/// `PH-LEN-011` is here because every Latinate suffix it names is `P-dom` or `L` and the rule's
/// own lexical input is the stored derivative spelling; `ORT-LEN-022` because every pair in it is
/// one of its listed exceptions (`fúl` -> `fullad`), whose spelling it declares lexical.
const NOT_GENERATED: &[&str] = &[
    "PH-LEN-011",
    "ORT-LEN-022",
    "DER-ADJ-017",
    "DER-ADJ-018",
    "DER-ADJ-024",
    "DER-ADV-004",
    "DER-DIM-005",
    "DER-DIM-006",
    "DER-NOUN-003",
    "DER-NOUN-009",
    "DER-NOUN-010",
    "DER-NOUN-011",
    "DER-NOUN-012",
    "DER-NOUN-013",
    "DER-NOUN-014",
    "DER-NOUN-015",
    "DER-NOUN-016",
    "DER-NOUN-017",
    "DER-NOUN-018",
    "DER-NOUN-019",
    "DER-NOUN-020",
    "DER-NOUN-021",
    "DER-NOUN-022",
    "DER-NOUN-023",
    "DER-NOUN-025",
    "DER-VERB-003",
    "DER-VERB-006",
    "DER-VERB-007",
    "DER-VERB-009",
    "DER-VERB-010",
    "DER-VERB-011",
    "DER-VERB-012",
    "DER-VERB-016",
    "DER-VERB-017",
    "DER-VERB-018",
    "DER-VERB-019",
    "DER-VERB-022",
];

/// Rules whose rewrites the repair module does not perform.
///
/// The repair fixes damage, never a grammatical choice (`docs/design/04-lint-repair.md`), so the
/// register and case corrections of `REP-SPELL-008` and `REP-SPELL-009` are reported, not applied.
/// The `REP-NOISE` rules and `REP-DIAC-002` state how damage is produced for training pairs, the
/// reverse of a repair.
const NOT_REPAIRED: &[&str] = &[
    "REP-SPELL-008",
    "REP-SPELL-009",
    "REP-DIAC-002",
    "REP-DIAC-006",
    "REP-NOISE-002",
    "REP-NOISE-003",
    "REP-NOISE-004",
    "REP-NOISE-005",
    "REP-NOISE-006",
    "REP-NOISE-007",
    "REP-NOISE-008",
    "REP-NOISE-009",
    "REP-NOISE-010",
    "REP-SPACE-002",
    "REP-SPACE-010",
    "REP-OCR-004",
    "REP-OCR-007",
    "REP-KEY-006",
];

/// One golden row.
struct Golden {
    rule: String,
    chapter: String,
    input: String,
    output: String,
    /// An accepted variant, which the engine must accept but need not generate.
    variant: bool,
}

/// One starred form, read from `rejects.ron`.
struct Reject {
    rule: String,
    chapter: String,
    form: String,
    /// The unstarred form a starred one competes with.
    rival: Option<String>,
}

/// The string field `name` of a RON row.
fn ron_field(line: &str, name: &str) -> Option<String> {
    let i = line.find(&format!("{name}:\""))? + name.len() + 2;
    let rest = &line[i..];
    let end = unescaped_end(rest)?;
    Some(unescape(&rest[..end]))
}

fn parse_rejects(text: &str) -> Vec<Reject> {
    text.lines()
        .map(str::trim)
        .filter(|l| l.starts_with("(rule:"))
        .filter_map(|l| {
            Some(Reject {
                rule: ron_field(l, "rule")?,
                chapter: ron_field(l, "chapter")?,
                form: ron_field(l, "form")?,
                rival: ron_field(l, "rival"),
            })
        })
        .collect()
}

fn parse_goldens(text: &str) -> Vec<Golden> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with("(rule:") {
            continue;
        }
        let field = |name: &str| ron_field(line, name);
        if let (Some(rule), Some(chapter), Some(input), Some(output)) = (
            field("rule"),
            field("chapter"),
            field("input"),
            field("output"),
        ) {
            out.push(Golden {
                rule,
                chapter,
                input,
                output,
                variant: line.contains(&format!("note:Some({:?})", crate::goldens::VARIANT)),
            });
        }
    }
    out
}

/// The byte offset of the closing quote of a RON string, skipping escaped characters.
fn unescaped_end(rest: &str) -> Option<usize> {
    let mut escaped = false;
    for (i, c) in rest.char_indices() {
        match c {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            '"' => return Some(i),
            _ => {}
        }
    }
    None
}

/// A RON string's escapes turned back into characters: `\u{301}`, `\"`, `\\`.
fn unescape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('u') => {
                let hex: String = chars.by_ref().skip(1).take_while(|c| *c != '}').collect();
                if let Some(c) = u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                    out.push(c);
                }
            }
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some(other) => out.push(other),
            None => {}
        }
    }
    out
}

/// Outcome of trying to reproduce one golden.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Outcome {
    /// Reproduced from the committed lexicon entry.
    Known,
    /// A derivative the lexicon lists, which an unproductive suffix does not generate.
    Listed,
    /// Reproduced, but only by guessing traits the lexicon does not record.
    Guessed,
    /// Not reproduced.
    Missed,
}

/// Why a golden is counted apart from the scored set.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Excluded {
    /// `NOT_GENERATED`.
    Refused,
    /// The rule declares its output lexical.
    Lexical,
    /// A phrase written as one compound: a choice of spelling, not a form the engine produces.
    Phrase,
    /// `NOT_REPAIRED`.
    Unrepaired,
}

impl Excluded {
    fn label(self) -> &'static str {
        match self {
            Self::Refused => "refused by DER-MODEL-012 (unproductive suffixes)",
            Self::Lexical => "declared lexical (stored, not derivable)",
            Self::Phrase => "phrase written solid (spelling choice, not generated)",
            Self::Unrepaired => "grammar choice or damage synthesis (not repaired)",
        }
    }
}

/// The engine's mechanisms, with the stack inventory built once.
struct Engine {
    stacks: Vec<Vec<Morph>>,
    bare: Stacks,
}

impl Engine {
    fn new() -> Self {
        let stacks = stacks();
        let bare = Stacks::from(stacks.clone());
        Self { stacks, bare }
    }

    /// Tries every mechanism that could satisfy one golden.
    ///
    /// Chapter 08 states typography and chapter 10 transformations, but chapter 01 mixes them with
    /// morphology, so each is tried rather than assumed from the chapter. A variant only has to
    /// be a form the checker accepts; a word an unproductive suffix made is reproduced by being
    /// listed, which is how `DER-MODEL-012` says such words exist.
    fn outcome_of(&self, c: &Golden) -> Outcome {
        let first = match c.chapter.as_str() {
            "08" => normalises(&c.input, &c.output),
            "10" => repairs(&c.rule, &c.input, &c.output),
            _ => self.generates(&c.input, &c.output, c.variant),
        };
        if first != Outcome::Missed {
            return first;
        }
        if c.variant && matches!(analyse(&c.output, &self.bare), Verdict::Known(_)) {
            return Outcome::Known;
        }
        let derivational =
            c.chapter == "06" || c.rule.starts_with("PH-ZERO") || c.rule.starts_with("ORT-LEN");
        if derivational
            && !c.output.contains(char::is_whitespace)
            && !lexicon().get(&c.output).is_empty()
        {
            return Outcome::Listed;
        }
        let mut others = vec![
            repairs(&c.rule, &c.input, &c.output),
            normalises(&c.input, &c.output),
            self.lemmatises(&c.input, &c.output),
            stems(&c.input, &c.output),
            articles(&c.input, &c.output),
            ordinals(&c.input, &c.output),
            streets(&c.input, &c.output),
        ];
        // Case mapping applied everywhere would pass an acronym golden for the wrong reason.
        if c.chapter == "01" {
            others.push(respells(&c.input, &c.output));
        }
        if c.input.contains('+') {
            others.push(rejoins(&c.input, &c.output));
        }
        if c.input.contains(['+', '#']) && c.output.contains('-') {
            others.push(hyphenates(&c.input, &c.output));
        }
        if c.input.contains('#') {
            others.push(writes_solid(&c.input, &c.output));
        }
        others.push(tabulates(&c.rule, &c.input, &c.output));
        others
            .into_iter()
            .find(|o| *o != Outcome::Missed)
            .unwrap_or(Outcome::Missed)
    }

    /// Whether some stack generates `want` from `input`.
    ///
    /// A multi-word input is a name or a phrase whose last word carries the suffix
    /// (`Kiss János` -> `Kiss Jánosné`), and a hyphenated coordinate pair inflects both halves
    /// (`VRB-IMP-004`: `lót-fut` -> `lósson-fusson`).
    fn generates(&self, input: &str, want: &str, variant: bool) -> Outcome {
        let outcome = self.reproduces(input, want, variant);
        if outcome != Outcome::Missed {
            return outcome;
        }
        if let Some((head, last)) = input.rsplit_once(' ')
            && let Some(rest) = want.strip_prefix(head).and_then(|w| w.strip_prefix(' '))
        {
            return self.generates(last, rest, variant);
        }
        if let Some((a, b)) = input.split_once('-')
            && !a.is_empty()
            && !b.is_empty()
            && let Some((wa, wb)) = want.split_once('-')
        {
            let left = self.reproduces(a, wa, variant);
            let right = self.reproduces(b, wb, variant);
            if left != Outcome::Missed && right != Outcome::Missed {
                return left.max_lexical(right);
            }
        }
        Outcome::Missed
    }

    /// Tries the committed lexicon entry first, then a guess, and reports which was needed.
    ///
    /// A preferred form has to be the one generation answers with; a variant only has to be among
    /// the forms the engine vouches for.
    fn reproduces(&self, input: &str, want: &str, variant: bool) -> Outcome {
        let input = lemma_of(input);
        let lx = lexicon();
        // A compound is productive, so the lexicon lists its members and not the whole: `vashíd`
        // has no entry, and its paradigm is composed from `híd` (`DER-COMP-008`).
        if lx.get(input).is_empty()
            && self
                .stacks
                .iter()
                .any(|s| generate_compound(input, s).as_deref() == Ok(want))
        {
            return Outcome::Known;
        }
        if self.generates_from_entries(input, want, variant) {
            return Outcome::Known;
        }
        // A rule may pair two inflected forms rather than a lemma and a form: `NOM-POSS-008`
        // writes `ajtaja -> ajtajuk`. Analysing the input recovers the lemma, and the pair is then
        // an ordinary generation question, so a golden of this shape fails if either direction is
        // wrong.
        if let Verdict::Known(readings) = analyse(input, &self.bare)
            && readings
                .iter()
                .any(|r| self.generates_from_entries(&r.lemma, want, variant))
        {
            return Outcome::Known;
        }
        // A semi-word is a compound member rather than a suffix (`DER-COMP-017`), so it is
        // generated by compounding and no feature stack reaches it: `kutyaszerű`, `utcabeli`.
        for (semi, _) in picur_morph_core::compound::semi_words() {
            if picur_morph_core::compound::with_semi_word(input, semi).as_deref() == Some(want) {
                return if lx.get(input).is_empty() {
                    Outcome::Guessed
                } else {
                    Outcome::Known
                };
            }
        }
        // A bare entry isolates how much is pure logic. Several categories are tried because a
        // paradigm belongs to a category: a comparative needs an adjective and a verbal stack a
        // verb.
        for pos in [Pos::Noun, Pos::Adjective, Pos::Verb] {
            let bare = Entry::new(input, pos);
            if self.stacks.iter().any(|s| {
                if variant {
                    forms(&bare, s).iter().any(|f| f == want)
                } else {
                    generate(&bare, s).as_deref() == Ok(want)
                }
            }) {
                return Outcome::Guessed;
            }
        }
        Outcome::Missed
    }

    /// Whether some entry of `lemma` generates `want`, or, for a variant, vouches for it.
    fn generates_from_entries(&self, lemma: &str, want: &str, variant: bool) -> bool {
        lexicon().get(lemma).iter().any(|entry| {
            self.stacks.iter().any(|s| {
                if variant {
                    forms(entry, s).iter().any(|f| f == want)
                } else {
                    generate(entry, s).as_deref() == Ok(want)
                }
            })
        })
    }

    /// Whether analysing `input` yields `want` as a lemma (`VRB-PREV-004`: `elmentünk` -> `elmegy`).
    fn lemmatises(&self, input: &str, want: &str) -> Outcome {
        match analyse(input, &self.bare) {
            Verdict::Known(readings) if readings.iter().any(|r| &*r.lemma == want) => {
                Outcome::Known
            }
            _ => Outcome::Missed,
        }
    }
}

impl Outcome {
    /// The weaker of two reproductions: a guess on either half makes the pair a guess.
    fn max_lexical(self, other: Self) -> Self {
        if self == Self::Guessed || other == Self::Guessed {
            Self::Guessed
        } else {
            Self::Known
        }
    }
}

/// Whether the repair rules turn `input` into `want`.
///
/// Chapter 10 states transformations rather than paradigm cells, so it is scored by running the
/// repair. A one-token input goes through the word repair; a longer one through the text repair,
/// which is where the context rules live. A `+` in the input is a seam (`fold+bol`).
fn repairs(rule: &str, input: &str, want: &str) -> Outcome {
    use picur_morph_repair::{
        Mode, accent_alternatives, apply, encoding, plain_letters, repair, repair_word_accented,
        restore_text, tabulated, unescaped,
    };
    let input = input.replace('+', "");
    let got = apply(&input, &repair(&input, Mode::Modernise));
    if got == want {
        return Outcome::Known;
    }
    // A golden is one token with no context, so it is read as a token of accented Hungarian
    // text, which is what the chapter's examples are.
    if repair_word_accented(&input, Mode::Modernise).is_some_and(|(got, _)| got == want) {
        return Outcome::Known;
    }
    // Accent restoration is licensed by the segment and not by the token (`REP-DIAC-001`), so a
    // golden has to say so itself. A `REP-DIAC` or `REP-ENC-011` golden is exactly that statement.
    if (rule.starts_with("REP-DIAC") || rule == "REP-ENC-011") && restore_text(&input) == want {
        return Outcome::Known;
    }
    // `REP-DIAC-003` states a candidate set, every member of which is an alternative the repair
    // offers and none of which it applies to a word that is already one.
    if rule == "REP-DIAC-003" && accent_alternatives(&input).iter().any(|c| c == want) {
        return Outcome::Known;
    }
    // A rule's substitution table is stated as data, and a row of it is reproduced by the data;
    // a code-page or an escape decoding is reproduced by the decoder, which a token too short
    // for the repair's own floors still answers.
    if tabulated(rule, &input, want)
        || encoding::demojibake(&input).iter().any(|c| c == want)
        || encoding::recode(&input).iter().any(|c| c == want)
        || encoding::western(&input).iter().any(|c| c == want)
        || encoding::unfit(&input, encoding::Page::Windows1252)
            .iter()
            .any(|c| c == want)
        || plain_letters(&input) == want
        || unescaped(&input).as_deref() == Some(want)
    {
        return Outcome::Known;
    }
    Outcome::Missed
}

/// Whether `want` is the public place name the geographic name `input` enters (`ORT-NAME-030`).
fn streets(input: &str, want: &str) -> Outcome {
    let found = want.rsplit_once(' ').is_some_and(|(_, generic)| {
        picur_morph_core::derivation::street_names(input, generic)
            .iter()
            .any(|s| s == want)
    });
    if found {
        Outcome::Known
    } else {
        Outcome::Missed
    }
}

/// Whether the punctuation lint turns `input` into `want`.
///
/// Chapter 08 states typography rather than paradigm cells, so a golden there is a rewrite.
/// `punct` measures the same lint's silence on correct text; this measures what it fixes.
fn normalises(input: &str, want: &str) -> Outcome {
    // A golden states what its rule's fix gives; a tendency is a fix `normalise` leaves to the
    // caller, so it is applied here too.
    let every_fix = picur_morph_lint::apply(input, &picur_morph_lint::check(input));
    if picur_morph_lint::normalise(input) == want || every_fix == want {
        Outcome::Known
    } else {
        Outcome::Missed
    }
}

/// Whether a writing-system text transform turns `input` into `want`.
///
/// Chapter 01 states operations on text: case mapping (`WS-CASE-004`), an initial capital
/// (`WS-CASE-001`), the repair of a mixed-case multi-character letter (`WS-CASE-007`, whose two
/// readings are both valid), the 7-bit accent notations (`WS-NORM-006`) and the code-page damage
/// tables (`WS-ENC-006`).
fn respells(input: &str, want: &str) -> Outcome {
    use picur_morph_ortho::case;
    use picur_morph_repair::encoding::{Page, mojibake};
    let mut direct = vec![
        case::upper(input),
        case::lower(input),
        case::capitalise(input),
        picur_morph_ortho::norm::expand_accents(input),
        picur_morph_ortho::norm::compose(&input.replace('+', "")),
    ];
    // `WS-ENC-006` tabulates the damage, not its repair, so the golden runs that way round.
    direct.extend(Page::ALL.map(|p| mojibake(input, p)));
    if direct.iter().any(|g| g == want)
        || case::repair(input).as_deref() == Some(want)
        || case::repair_alternative(input).as_deref() == Some(want)
    {
        Outcome::Known
    } else {
        Outcome::Missed
    }
}

/// Whether rejoining a line-end split reaches `want` (`WS-DIG-012`).
///
/// The rule is binary and undecidable without morphology, so it offers both spellings; the golden
/// is reproduced when either is the answer.
fn rejoins(input: &str, want: &str) -> Outcome {
    let Some((left, right)) = input.split_once('+') else {
        return Outcome::Missed;
    };
    if picur_morph_ortho::rejoin(left, right)
        .iter()
        .any(|c| c == want)
    {
        Outcome::Known
    } else {
        Outcome::Missed
    }
}

/// Whether hyphenating at the seam the golden names produces the break it shows.
///
/// `ORT-SPLIT-020` writes its inputs with the seam in them, `+` for a stem-suffix junction and
/// `#` for a compound boundary: the break is licensed by the morphology and the letters may not
/// merge across it, so `ház` plus `sor` is `ház-sor` and never `há-zsor`. A word has more break
/// points than the one the golden shows, so the test is that the golden's break is among them.
fn hyphenates(input: &str, want: &str) -> Outcome {
    let Some(seam) = input.find(['+', '#']) else {
        return Outcome::Missed;
    };
    let word: String = input.chars().filter(|c| !matches!(c, '+' | '#')).collect();
    let Some((left, _)) = want.split_once('-') else {
        return Outcome::Missed;
    };
    let mut prefix = String::new();
    for p in &picur_morph_ortho::hyphenate_at(&word, &[seam]) {
        prefix.push_str(p);
        if prefix == left {
            return Outcome::Known;
        }
    }
    Outcome::Missed
}

/// Whether a compound written with its seams is solid by the 6:3 rule (`ORT-63-001`).
///
/// A `#` separates members and a `+` a preverb or suffix inside one. A compound of two members, or
/// of six syllables or fewer, is written solid whatever its length; the hyphen of a longer one
/// falls at its main seam, which `hyphenates` checks.
fn writes_solid(input: &str, want: &str) -> Outcome {
    let members = input.split('#').count();
    let solid: String = input.chars().filter(|c| !matches!(c, '+' | '#')).collect();
    let short = picur_morph_ortho::syllable_count(&solid) <= 6;
    if (members <= 2 || short) && solid == want {
        Outcome::Known
    } else {
        Outcome::Missed
    }
}

/// Whether the golden states a row of an inventory the engine holds.
///
/// `REP-OCR-001`'s output says print `ó` can be scanned as `o` and equally as `ö`, so no function
/// answers it. The vowel-length rules do the same for the short-long pairs, which is the
/// alphabet's own pairing and lives in `Letter`.
fn tabulates(rule: &str, input: &str, want: &str) -> Outcome {
    if rule == "REP-OCR-001" && picur_morph_repair::confuses(input, want) {
        return Outcome::Known;
    }
    if pairs_by_length(input, want) {
        return Outcome::Known;
    }
    Outcome::Missed
}

/// Whether the two strings are one letter and its length partner (`WS-ALPH-003`).
fn pairs_by_length(input: &str, want: &str) -> bool {
    let letter = |s: &str| {
        let mut it = picur_morph_ortho::letters(s);
        let first = it.next()?;
        it.next().is_none().then_some(first.letter)
    };
    let (Some(a), Some(b)) = (letter(input), letter(want)) else {
        return false;
    };
    a != b && a.short() == b.short() && a.long() == b.long()
}

/// Whether `want` is the `-ik`-less present stem of the verb `input` (`VRB-STEM-001`).
fn stems(input: &str, want: &str) -> Outcome {
    let is_verb = lexicon().get(input).iter().any(|e| e.pos == Pos::Verb);
    if is_verb && present_stem(input) == want {
        Outcome::Known
    } else {
        Outcome::Missed
    }
}

/// Whether `want` is `input` with its definite article (`ORT-ABBR-001`: `MÁV` -> `a MÁV`).
fn articles(input: &str, want: &str) -> Outcome {
    match article(input) {
        Some(a) if want == format!("{} {input}", a.as_str()) => Outcome::Known,
        _ => Outcome::Missed,
    }
}

/// Whether `want` is the ordinal of the cardinal `input` (`DER-ADJ-025`: `hat` -> `hatodik`),
/// or its ten (`nyolc` -> `nyolcvan`).
fn ordinals(input: &str, want: &str) -> Outcome {
    let small = 0..=100;
    let round = (200..=1000).step_by(100);
    let mut all = small.chain(round).chain([2000, 1_000_000]);
    match all.find(|&n| cardinal(n) == input) {
        Some(n) if ordinal(n) == want || (n < 10 && cardinal(n * 10) == want) => Outcome::Known,
        _ => Outcome::Missed,
    }
}

/// The lemma a golden's input names.
///
/// A rule sometimes writes the input as a morpheme string to show the seam: `NOM-CASE-005` gives
/// `orr+ra -> orra`. The part before the first `+` is the stem.
fn lemma_of(input: &str) -> &str {
    input.split('+').next().unwrap_or(input)
}

/// A golden stated as a phrase that the output writes as one word.
fn phrase_to_solid(c: &Golden) -> bool {
    c.input.contains(char::is_whitespace) && !c.output.contains(char::is_whitespace)
}

/// Runs the evaluation and prints a report.
/// How one rule's goldens and starred forms fared.
#[derive(Clone, Copy, Default)]
pub(crate) struct RuleScore {
    /// Scored goldens: reproduced, total.
    pub goldens: (usize, usize),
    /// Goldens set aside: reproduced anyway, total.
    pub excluded: (usize, usize),
    /// Starred forms: rejected, total.
    pub rejects: (usize, usize),
    /// Example forms: accepted, total.
    pub accepts: (usize, usize),
    /// Example texts: unflagged by the lint, total.
    pub silences: (usize, usize),
}

/// Everything one evaluation run found.
struct Scored<'a> {
    hit: usize,
    known: usize,
    listed: usize,
    in_lexicon: usize,
    variants: (usize, usize),
    excluded: BTreeMap<Excluded, (usize, usize)>,
    by_rule: BTreeMap<&'a str, (usize, usize)>,
    by_chapter: BTreeMap<&'a str, (usize, usize)>,
    misses: Vec<&'a Golden>,
    rejects: usize,
    /// Starred forms the engine accepts in the reading the rule stars.
    accepted: Vec<Starred<'a>>,
    /// Starred forms the engine accepts only in a reading the rule does not govern.
    elsewhere: Vec<&'a Reject>,
    /// Example forms tried, and the ones the checker did not accept.
    examples: usize,
    unaccepted: Vec<&'a Reject>,
    /// Example texts tried, and the ones the lint flagged.
    texts: usize,
    flagged: Vec<&'a Reject>,
    per_rule: BTreeMap<String, RuleScore>,
}

/// A starred form the engine accepts against its rule.
#[derive(Clone, Copy)]
struct Starred<'a> {
    reject: &'a Reject,
    why: Against,
}

/// The three lists of one tally of the form sets.
struct Tally<'a> {
    per_rule: BTreeMap<String, RuleScore>,
    accepted: Vec<Starred<'a>>,
    elsewhere: Vec<&'a Reject>,
    unaccepted: Vec<&'a Reject>,
    flagged: Vec<&'a Reject>,
    /// Starred forms, example forms and example texts tried.
    tried: (usize, usize, usize),
}

/// The derivational suffixes a rule governs: the records it heads, or for a rule that heads none,
/// those whose head rule's heading names a suffix its own heading names (`DER-ADJ-002 Relational
/// -i` restricts the `-i` of `DER-ADJ-001 Relational -i`).
fn governed(rule: &str, specs: &BTreeMap<String, crate::coverage::Spec>) -> Vec<DerId> {
    let own: Vec<DerId> = DerId::ALL
        .into_iter()
        .filter(|d| d.record().id == rule)
        .collect();
    if !own.is_empty() {
        return own;
    }
    let Some(spec) = specs.get(rule) else {
        return Vec::new();
    };
    DerId::ALL
        .into_iter()
        .filter(|d| {
            specs
                .get(d.record().id)
                .is_some_and(|h| h.suffixes.iter().any(|s| spec.suffixes.contains(s)))
        })
        .collect()
}

/// Why a starred form counts against its rule.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Against {
    /// A reading goes through the suffix the rule governs.
    Through,
    /// The lexicon lists the starred string as a word of its own, so lexicon and rule disagree.
    Listed,
    /// A reading carries no feature stack, so what forms it is unknown.
    Opaque,
    /// A reading fills the cell of the rival form the text sets against the star.
    Rival,
    /// The rule names no suffix and the text no rival, so the string itself was tested.
    Unscoped,
}

/// How the engine stands to a starred form it accepts: against the rule, or only in readings the
/// rule does not govern (`None`).
///
/// A derived word is also reported as its own lemma (`DER-MODEL-001`); that bare reading belongs
/// to the derivation beside it, so only a lexicon entry makes the string listed.
///
/// Without a suffix, a rival scopes the star: `*lót` beside `lovat` counts only when a reading of
/// `lót` has the lemma and the features of a reading of `lovat`. A rival only clears inflections
/// of listed words: a derivation or an unlisted bare reading still counts against the rule.
fn against(form: &str, suffixes: &[DerId], rival: Option<&str>) -> Option<Against> {
    if suffixes.is_empty() {
        let cells = rival.map(picur_morph_spell::analyses).unwrap_or_default();
        if cells.is_empty() {
            return Some(Against::Unscoped);
        }
        let readings = picur_morph_spell::analyses(form);
        if readings.iter().any(|a| {
            cells
                .iter()
                .any(|c| c.lemma == a.lemma && c.stack == a.stack)
        }) {
            return Some(Against::Rival);
        }
        if readings
            .iter()
            .any(|a| a.stack.iter().any(|m| matches!(m, Morph::Derive(_))))
        {
            return Some(Against::Unscoped);
        }
        let listed = !lexicon().get(form).is_empty();
        return readings
            .iter()
            .any(|a| a.stack.is_empty() && !listed)
            .then_some(Against::Opaque);
    }
    let readings = picur_morph_spell::analyses(form);
    let through = readings.iter().any(|a| {
        a.stack
            .iter()
            .any(|m| matches!(m, Morph::Derive(d) if suffixes.contains(d)))
    });
    if through {
        return Some(Against::Through);
    }
    let listed = !lexicon().get(form).is_empty();
    let formed = readings.iter().any(|a| !a.stack.is_empty());
    let bare = |a: &&picur_morph_core::Analysis| {
        a.stack.is_empty() && (listed || !formed || *a.lemma != *form)
    };
    match readings.iter().find(bare) {
        Some(a) if *a.lemma == *form && listed => Some(Against::Listed),
        Some(_) => Some(Against::Opaque),
        None if readings.is_empty() => Some(Against::Opaque),
        None => None,
    }
}

/// Tallies the starred forms, the example forms and the example texts of `chapter` per rule,
/// returning the ones the engine got wrong.
///
/// A star forbids a reading, not a spelling: `*autói` is no `-i` adjective of `autó`, yet it is
/// the possessed plural. Where the rule's suffix is known, only an accepted reading through that
/// suffix is a miss; elsewhere the form itself must be rejected.
fn tally<'a>(
    rejects: &'a [Reject],
    accepts: &'a [Reject],
    silences: &'a [Reject],
    specs: &BTreeMap<String, crate::coverage::Spec>,
    chapter: Option<&str>,
) -> Tally<'a> {
    let of = |rows: &'a [Reject]| -> Vec<&'a Reject> {
        rows.iter()
            .filter(|r| chapter.is_none_or(|ch| r.chapter == ch))
            .collect()
    };
    let (rejects, accepts, silences) = (of(rejects), of(accepts), of(silences));
    let mut per_rule: BTreeMap<String, RuleScore> = BTreeMap::new();
    let mut accepted = Vec::new();
    let mut elsewhere = Vec::new();
    for r in &rejects {
        let slot = &mut per_rule.entry(r.rule.clone()).or_default().rejects;
        slot.1 += 1;
        if picur_morph_spell::check(&r.form) != picur_morph_spell::Status::Ok {
            slot.0 += 1;
            continue;
        }
        if let Some(why) = against(&r.form, &governed(&r.rule, specs), r.rival.as_deref()) {
            accepted.push(Starred { reject: r, why });
        } else {
            slot.0 += 1;
            elsewhere.push(*r);
        }
    }
    let mut unaccepted = Vec::new();
    for r in &accepts {
        let slot = &mut per_rule.entry(r.rule.clone()).or_default().accepts;
        slot.1 += 1;
        if stands(r) {
            slot.0 += 1;
        } else {
            unaccepted.push(*r);
        }
    }
    let mut flagged = Vec::new();
    for r in &silences {
        let slot = &mut per_rule.entry(r.rule.clone()).or_default().silences;
        slot.1 += 1;
        if picur_morph_lint::check(&r.form).is_empty() {
            slot.0 += 1;
        } else {
            flagged.push(*r);
        }
    }
    Tally {
        per_rule,
        accepted,
        elsewhere,
        unaccepted,
        flagged,
        tried: (rejects.len(), accepts.len(), silences.len()),
    }
}

/// Scores every golden and starred form of `chapter`, or of every chapter.
fn score<'a>(inputs: &'a Inputs, chapter: Option<&str>) -> Scored<'a> {
    let Inputs {
        all,
        rejects,
        accepts,
        silences,
        pred,
        specs,
    } = inputs;
    let cases: Vec<&Golden> = all
        .iter()
        .filter(|c| chapter.is_none_or(|ch| c.chapter == ch))
        .collect();
    let lexical = |rule: &str| pred.get(rule).is_some_and(|p| p == "lexical");
    let engine = Engine::new();
    let Tally {
        per_rule,
        accepted,
        elsewhere,
        unaccepted,
        flagged,
        tried: (n_rejects, n_accepts, n_silences),
    } = tally(rejects, accepts, silences, specs, chapter);
    let mut s = Scored {
        hit: 0,
        known: 0,
        listed: 0,
        in_lexicon: 0,
        variants: (0, 0),
        excluded: BTreeMap::new(),
        by_rule: BTreeMap::new(),
        by_chapter: BTreeMap::new(),
        misses: Vec::new(),
        rejects: n_rejects,
        accepted,
        elsewhere,
        examples: n_accepts,
        unaccepted,
        texts: n_silences,
        flagged,
        per_rule,
    };
    for c in &cases {
        let class = if NOT_GENERATED.contains(&c.rule.as_str()) {
            Some(Excluded::Refused)
        } else if NOT_REPAIRED.contains(&c.rule.as_str()) {
            Some(Excluded::Unrepaired)
        } else if lexical(&c.rule) {
            Some(Excluded::Lexical)
        } else if phrase_to_solid(c) {
            Some(Excluded::Phrase)
        } else {
            None
        };
        let score = s.per_rule.entry(c.rule.clone()).or_default();
        if let Some(class) = class {
            let slot = s.excluded.entry(class).or_default();
            slot.1 += 1;
            score.excluded.1 += 1;
            // The refused set is not tried: generating with an unproductive suffix is the error.
            if class != Excluded::Refused && engine.outcome_of(c) != Outcome::Missed {
                slot.0 += 1;
                score.excluded.0 += 1;
            }
            continue;
        }
        let outcome = engine.outcome_of(c);
        if c.variant {
            s.variants.1 += 1;
            s.variants.0 += usize::from(outcome != Outcome::Missed);
        }
        let rule = s.by_rule.entry(c.rule.as_str()).or_default();
        let ch = s.by_chapter.entry(c.chapter.as_str()).or_default();
        rule.1 += 1;
        ch.1 += 1;
        score.goldens.1 += 1;
        if !lexicon().get(&c.input).is_empty() {
            s.in_lexicon += 1;
        }
        match outcome {
            Outcome::Known | Outcome::Guessed | Outcome::Listed => {
                s.hit += 1;
                s.known += usize::from(outcome == Outcome::Known);
                s.listed += usize::from(outcome == Outcome::Listed);
                rule.0 += 1;
                ch.0 += 1;
                score.goldens.0 += 1;
            }
            Outcome::Missed => s.misses.push(c),
        }
    }
    s
}

/// Whether the checker lets an example form stand: a word it vouches for, or a capitalised form
/// it finds well formed, which is what a name the lexicon does not list gets (`Putyin`,
/// `Gdańsk`); the rule that shows such a name states its spelling, not its listing.
/// Whether an example form holds: a line-break rule's hyphenated example is a split to check,
/// every other one a word to accept.
fn stands(r: &Reject) -> bool {
    if r.rule.starts_with("ORT-SPLIT") && r.form.contains('-') {
        return picur_morph_spell::is_legal_split(&r.form);
    }
    lets_stand(&r.form)
}

fn lets_stand(form: &str) -> bool {
    use picur_morph_spell::Status;
    if form.contains('\u{2013}') {
        return picur_morph_spell::check_text(form).is_empty();
    }
    match picur_morph_spell::check(form) {
        Status::Ok => true,
        Status::Unknown => form.chars().next().is_some_and(char::is_uppercase),
        _ => false,
    }
}

/// The golden sets and the rulebook columns scoring reads.
struct Inputs {
    all: Vec<Golden>,
    rejects: Vec<Reject>,
    accepts: Vec<Reject>,
    silences: Vec<Reject>,
    /// Each rule's predictability.
    pred: BTreeMap<String, String>,
    /// Each rule's heading suffixes, by rule ID.
    specs: BTreeMap<String, crate::coverage::Spec>,
}

/// Reads the goldens, the starred forms and the rulebook's predictability and interaction columns.
fn inputs(goldens: &Path) -> Result<Inputs> {
    let all = parse_goldens(&fs::read_to_string(goldens)?);
    if all.is_empty() {
        return Err(format!("no goldens in {}", goldens.display()).into());
    }
    let rejects = parse_rejects(&fs::read_to_string(goldens.with_file_name("rejects.ron"))?);
    let accepts = parse_rejects(&fs::read_to_string(goldens.with_file_name("accepts.ron"))?);
    let silences = parse_rejects(&fs::read_to_string(goldens.with_file_name("silences.ron"))?);
    let pred = goldens
        .parent()
        .map(crate::coverage::predictability)
        .transpose()?
        .unwrap_or_default();
    let specs = goldens
        .parent()
        .map(crate::coverage::specs)
        .transpose()?
        .unwrap_or_default()
        .into_iter()
        .map(|s| (s.id.clone(), s))
        .collect();
    Ok(Inputs {
        all,
        rejects,
        accepts,
        silences,
        pred,
        specs,
    })
}

/// Every rule's score over all chapters, for the ledger.
pub(crate) fn rule_scores(goldens: &Path) -> Result<BTreeMap<String, RuleScore>> {
    let i = inputs(goldens)?;
    Ok(score(&i, None).per_rule)
}

pub(crate) fn run(goldens: &Path, chapter: Option<&str>) -> Result {
    let i = inputs(goldens)?;
    let s = score(&i, chapter);
    report(&Report {
        hit: s.hit,
        known: s.known,
        listed: s.listed,
        in_lexicon: s.in_lexicon,
        variants: s.variants,
        excluded: &s.excluded,
        by_rule: &s.by_rule,
        by_chapter: &s.by_chapter,
        misses: &s.misses,
        rejects: s.rejects,
        accepted: &s.accepted,
        elsewhere: &s.elsewhere,
        examples: s.examples,
        unaccepted: &s.unaccepted,
        texts: s.texts,
        flagged: &s.flagged,
        per_chapter: chapter.is_some(),
    });
    Ok(())
}

/// What one evaluation run found.
struct Report<'a> {
    hit: usize,
    known: usize,
    listed: usize,
    in_lexicon: usize,
    variants: (usize, usize),
    excluded: &'a BTreeMap<Excluded, (usize, usize)>,
    by_rule: &'a BTreeMap<&'a str, (usize, usize)>,
    by_chapter: &'a BTreeMap<&'a str, (usize, usize)>,
    misses: &'a [&'a Golden],
    /// Starred forms tried, the ones accepted against their rule, and the ones accepted only in a
    /// reading the rule does not govern.
    rejects: usize,
    accepted: &'a [Starred<'a>],
    elsewhere: &'a [&'a Reject],
    examples: usize,
    unaccepted: &'a [&'a Reject],
    texts: usize,
    flagged: &'a [&'a Reject],
    per_chapter: bool,
}

/// Prints the report.
#[expect(clippy::print_stdout, reason = "CLI report")]
fn report(r: &Report<'_>) {
    let Report {
        hit,
        known,
        listed,
        in_lexicon,
        variants,
        excluded,
        by_rule,
        by_chapter,
        misses,
        rejects,
        accepted,
        elsewhere,
        examples,
        unaccepted,
        texts,
        flagged,
        per_chapter,
    } = *r;
    let total = by_rule.values().map(|(_, t)| t).sum::<usize>();
    let (ex_hit, ex_total) = excluded
        .values()
        .fold((0, 0), |(h, t), (eh, et)| (h + eh, t + et));
    println!("reproduced {hit}/{total} ({})", percent(hit, total));
    println!(
        "  every row, excluded ones included {}/{} ({})",
        hit + ex_hit,
        total + ex_total,
        percent(hit + ex_hit, total + ex_total)
    );
    let rejected = rejects - accepted.len();
    println!(
        "kept out {rejected}/{rejects} starred forms ({})",
        percent(rejected, rejects)
    );
    println!(
        "  {} of them accepted only in a reading the rule does not govern",
        elsewhere.len()
    );
    let ok = examples - unaccepted.len();
    println!(
        "accepted {ok}/{examples} example forms ({})",
        percent(ok, examples)
    );
    let quiet = texts - flagged.len();
    println!(
        "unflagged {quiet}/{texts} example texts ({})",
        percent(quiet, texts)
    );
    println!("  from the lexicon entry   {known}");
    println!("  from a bare stem         {}", hit - known - listed);
    println!("  listed derivatives       {listed}");
    println!("  inputs present in lexicon {in_lexicon}/{total}");
    println!(
        "  accepted variants         {}/{} (the rest are preferred forms generation must give)",
        variants.0, variants.1
    );
    if !excluded.is_empty() {
        println!("\nexcluded from the score");
        for (class, (h, t)) in excluded {
            println!("  {t:>4} {}, {h} reproduced anyway", class.label());
        }
    }

    if !per_chapter {
        println!("\nby chapter");
        for (ch, (h, t)) in by_chapter {
            println!("  {ch} {h:>5}/{t:<5} {}", percent(*h, *t));
        }
    }

    let mut worst: Vec<(&&str, &(usize, usize))> =
        by_rule.iter().filter(|(_, (h, t))| h < t).collect();
    worst.sort_by_key(|(_, (h, t))| h.wrapping_sub(*t));
    println!("\nrules with unreproduced cases (worst first)");
    for (rule, (h, t)) in worst.iter().take(15) {
        println!("  {rule:<18} {h}/{t}");
    }

    // A chapter run is an audit, so it lists every miss; the full run stays a summary.
    let shown = if per_chapter { misses.len() } else { 20 };
    println!("\nmisses");
    for c in misses.iter().take(shown) {
        let tag = if c.variant { " (variant)" } else { "" };
        println!("  {:<18} {} -> {}{tag}", c.rule, c.input, c.output);
    }
    starred(accepted, elsewhere, per_chapter);
    println!("\nexample forms not accepted");
    let shown = if per_chapter { unaccepted.len() } else { 20 };
    for r in unaccepted.iter().take(shown) {
        println!("  {:<18} {}", r.rule, r.form);
    }
    println!("\nexample texts the lint flags");
    let shown = if per_chapter { flagged.len() } else { 20 };
    for r in flagged.iter().take(shown) {
        println!("  {:<18} {}", r.rule, r.form);
    }
}

/// Prints the starred forms accepted against their rule, by why, and on a chapter run the ones
/// accepted only in another reading.
#[expect(clippy::print_stdout, reason = "CLI report")]
fn starred(accepted: &[Starred<'_>], elsewhere: &[&Reject], per_chapter: bool) {
    let shown = if per_chapter { accepted.len() } else { 20 };
    for (why, title) in [
        (Against::Through, "starred readings the engine forms"),
        (Against::Listed, "starred forms the lexicon lists as words"),
        (
            Against::Opaque,
            "starred forms accepted with no feature stack",
        ),
        (Against::Rival, "starred forms read in their rival's cell"),
        (
            Against::Unscoped,
            "starred forms accepted, with neither a suffix nor a rival to scope them",
        ),
    ] {
        println!("\n{title}");
        for s in accepted.iter().filter(|s| s.why == why).take(shown) {
            println!(
                "  {:<18} *{:<16} {}",
                s.reject.rule,
                s.reject.form,
                readings(&s.reject.form)
            );
        }
    }
    if per_chapter {
        println!("\nstarred forms accepted in another reading");
        for r in elsewhere {
            println!("  {:<18} *{:<16} {}", r.rule, r.form, readings(&r.form));
        }
    }
}

/// The engine's readings of `form`, each as its lemma and feature stack.
fn readings(form: &str) -> String {
    picur_morph_spell::analyses(form)
        .iter()
        .map(|a| format!("{}{:?}", a.lemma, a.stack))
        .collect::<Vec<_>>()
        .join(" | ")
}

#[expect(clippy::cast_precision_loss, reason = "report percentages")]
fn percent(h: usize, t: usize) -> String {
    if t == 0 {
        return "-".to_owned();
    }
    format!("{:.1}%", 100.0 * h as f64 / t as f64)
}

/// Goldens indexed by input form, for cross-checking generated data against the specification.
pub(crate) fn goldens_by_input(path: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for c in parse_goldens(&fs::read_to_string(path)?) {
        out.entry(c.input).or_default().push(c.output);
    }
    Ok(out)
}
