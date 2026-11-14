//! Converts morphdb.hu into the committed lexicon, once, offline.
//!
//! morphdb is CC-BY 2.5 and is the only oracle that feeds committed data; the attribution is in
//! `NOTICE`. See `docs/design/06-reproducibility.md` for the flag vocabulary and the rules applied
//! on top of the vote.
//!
//! Scope: the files that carry inflectional information. Pronouns, numerals and determiners are
//! deliberately **not** read from here; they are closed inventories that belong in code with a
//! stated member count, not in data (design bucket A). `ner.lexicon` is skipped as well: 244k named
//! entities would dominate the crate without teaching the engine anything about morphology.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use encoding_rs::ISO_8859_2;
use picur_morph_core::{Case, Definite, Mood, Morph, Number, Person, Tense, generate};
use picur_morph_lexicon::{Entry, Pos, Trait, TraitSet, frequency};

use crate::Result;

/// Source files, the part of speech their entries take, and whether the file is written loosely.
///
/// "Loosely" covers two habits of the small closed-class files: a leading `#` marks an entry rather
/// than a comment, and the lemma may be separated by a plain space instead of a tab.
/// `advpron.lexicon` does both, on different rows (`ahogy` with a tab, `így` with a space).
const FILES: &[(&str, Option<Pos>, bool)] = &[
    ("peeta.lexicon", None, false),
    ("missing.lexicon", None, false),
    ("proper_noun.lexicon", Some(Pos::ProperNoun), false),
    ("proper_noun_geo.lexicon", Some(Pos::ProperNoun), false),
    ("adv.lexicon", Some(Pos::Adverb), false),
    ("advpron.lexicon", Some(Pos::Adverb), true),
    ("conj.lexicon", Some(Pos::Conjunction), true),
    ("prev.lexicon", Some(Pos::Preverb), false),
    ("postp+case.lexicon", Some(Pos::Postposition), false),
    ("acro.lexicon", Some(Pos::Noun), false),
    ("ono.lexicon", Some(Pos::Interjection), false),
    ("sentint.lexicon", Some(Pos::Interjection), false),
    ("greeting.lexicon", Some(Pos::Interjection), false),
];

/// morphdb's tag for a part of speech we keep.
fn pos_of(tag: &str) -> Option<Pos> {
    match tag {
        // `ACRO` shares Noun deliberately: an acronym inflects as a noun.
        "NOUN" | "ACRO" => Some(Pos::Noun),
        "ADJ" => Some(Pos::Adjective),
        // `missing.lexicon` writes the three `-tszik` verbs with morphdb's own lowercase tag for a
        // verb with no definite conjugation. Unmapped, it drops `látszik`, `hallatszik` and
        // `tetszik`, of which `tetszik` alone is 711,197 tokens.
        "VERB" | "verb_indef" | "verb_def" => Some(Pos::Verb),
        "ADV" | "A_ADV" | "S_ADV" | "T_ADV" | "C_ADV" | "M_ADV" => Some(Pos::Adverb),
        "PREV" => Some(Pos::Preverb),
        "POSTP" => Some(Pos::Postposition),
        "ONO" | "SENTINT" | "GREETING" => Some(Pos::Interjection),
        _ => None,
    }
}

/// Flags voting for a trait, flags voting against, the trait on a win, the trait on a loss.
type Rule = (
    &'static [&'static str],
    &'static [&'static str],
    Option<Trait>,
    Option<Trait>,
);

/// See `docs/design/06-reproducibility.md` for the provenance of this table.
const RULES: &[Rule] = &[
    (&["back"], &["front"], Some(Trait::Back), Some(Trait::Front)),
    (&["low"], &["non_low"], Some(Trait::Low), None),
    (
        &["bareT"],
        &["non_bareT"],
        Some(Trait::BareAccusative),
        Some(Trait::LinkedAccusative),
    ),
    (
        &["poss_A", "poss_sg_A"],
        &["poss_jA", "poss_sg_jA"],
        Some(Trait::PossessiveA),
        Some(Trait::PossessiveJa),
    ),
    (
        &["epenthetic"],
        &["non_epenthetic"],
        Some(Trait::Epenthetic),
        None,
    ),
    (
        &["shortening"],
        &["non_shortening"],
        Some(Trait::Shortening),
        None,
    ),
    (&["v_stem"], &["non_v_stem"], Some(Trait::VStem), None),
    (&["oa_stem"], &["non_oa_stem"], Some(Trait::OaStem), None),
    (&["u_stem"], &["non_u_stem"], Some(Trait::UStem), None),
    (
        &["sz-d_stem"],
        &["non_sz-d_stem"],
        Some(Trait::SzDStem),
        None,
    ),
    (
        &["sz-d-v_stem"],
        &["non_sz-d-v_stem"],
        Some(Trait::SzDVStem),
        None,
    ),
    (&["sing"], &[], Some(Trait::SingularOnly), None),
    (&["non_poss"], &[], Some(Trait::NoPossessive), None),
    (&["trans"], &[], Some(Trait::Transitive), None),
    (&["preverb"], &[], Some(Trait::Preverb), None),
    (&["t_final_past"], &[], Some(Trait::PastFinalT), None),
    (&["Vtt_final_past"], &[], Some(Trait::PastFinalVtt), None),
    (&["t_nonfinal_past"], &[], Some(Trait::PastNonfinalT), None),
    (
        &["Vtt_nonfinal_past"],
        &[],
        Some(Trait::PastNonfinalVtt),
        None,
    ),
];

/// One parsed source line.
#[derive(Debug)]
struct Line {
    flags: Vec<String>,
    parts: Vec<String>,
    /// Only `fkp` contributed this line, so only its `aff_*` flags may be trusted.
    fkp_only: bool,
}

/// Strips morphdb's line terminator from a token: `NOUN,;` and `NOUN,` both mean `NOUN`.
///
/// This is the defect that dropped `stressz`, whose whole line is `stressz<TAB>NOUN,;`.
fn bare(tok: &str) -> &str {
    tok.trim_end_matches([',', ';', ' '])
}

/// Whether a lemma is spelled as a word: letters, with a hyphen, space or apostrophe inside.
///
/// morphdb carries rows that are not words, which the engine can do nothing with: punctuation
/// (`,`), codes (`A4`, `3D`), HTML entities (`&Aring;ngström`), soft hyphens, placeholders
/// (`hosszúcs?r?`) and underscored phrases (`Amerikai_Szamoa`).
fn word_like(lemma: &str) -> bool {
    let mut chars = lemma.chars();
    chars.next().is_some_and(char::is_alphabetic)
        && lemma.chars().last().is_some_and(char::is_alphabetic)
        && lemma
            .chars()
            .all(|c| c.is_alphabetic() || matches!(c, '-' | ' ' | '\''))
}

/// Parses one line into a key and its flags.
fn parse(line: &str, forced: Option<Pos>, loose: bool) -> Option<((Pos, String), Line)> {
    let line = match line.strip_prefix('#') {
        Some(rest) if loose => rest,
        Some(_) => return None,
        None => line,
    };
    // Several files separate the lemma with a tab on most rows and a plain space on the rest
    // (`így` and `hogyan` in `advpron.lexicon`, `látszik` and `tetszik` in `missing.lexicon`). The
    // fallback is unconditional because it only fires on a row with no tab at all.
    // It is deliberately not tied to `loose`: that flag strips a leading `#`, and a file with a
    // forced category would turn each of its comment banners into an entry.
    let split = line.split_once('\t').or_else(|| line.split_once(' '));
    let (lemma, rest) = split?;
    // `postp+case.lexicon` writes a leading `, ` on every row, marking the slot a personal suffix
    // fills; it is not part of the lemma.
    let lemma = lemma.trim().trim_start_matches(", ").trim();
    if !word_like(lemma) {
        return None;
    }

    let toks: Vec<&str> = rest.split_whitespace().collect();
    let pos_at = toks.iter().position(|t| pos_of(bare(t)).is_some());
    let pos = match (pos_at, forced) {
        (Some(i), _) => pos_of(bare(toks[i]))?,
        (None, Some(p)) => p,
        (None, None) => return None,
    };

    // Flags are the bare tokens after the POS, up to the first `KEY:` section.
    let mut flags = Vec::new();
    let mut parts = Vec::new();
    let mut section: Option<&str> = None;
    let mut src = Vec::new();
    for tok in toks.iter().skip(pos_at.map_or(0, |i| i + 1)) {
        if tok.ends_with(':') {
            section = Some(tok.trim_end_matches(':'));
            continue;
        }
        match section {
            None => {
                let f = bare(tok);
                if !f.is_empty() && f != "," {
                    flags.push(f.to_owned());
                }
            }
            // The row terminator `,;` lands in whichever section is open, so it is filtered here
            // as in the flag list; counted as a source it defeats `fkp_only`.
            Some("SRC") => {
                let s = bare(tok);
                if !s.is_empty() && s != "," {
                    src.push(s.to_owned());
                }
            }
            Some("STR") if parts.is_empty() && tok.contains('@') => {
                parts = bare(tok)
                    .split('@')
                    .map(str::to_owned)
                    .filter(|p| !p.is_empty())
                    .collect();
            }
            _ => {}
        }
    }

    let fkp_only = src.len() == 1 && src[0] == "fkp";
    Some((
        (pos, lemma.to_owned()),
        Line {
            flags,
            parts,
            fkp_only,
        },
    ))
}

/// Resolves the traits of one lemma from all its source lines.
///
/// Returns the traits and, when the flags contradict each other, the names of the rules that tied.
/// A tie is **recorded**, never silently resolved: `orr` carries both `bareT` and `non_bareT`, and
/// letting the vote fall to neither is what made the engine generate the forbidden `orrt`.
fn votes(lines: &[Line], pos: Pos) -> (TraitSet, Vec<&'static str>) {
    let mut set = TraitSet::EMPTY;
    let mut ties = Vec::new();
    for (fors, againsts, on_win, on_loss) in RULES {
        let mut win = false;
        let mut loss = false;
        for line in lines {
            // An adjective line sourced from `fkp` alone describes its adverb suffix, so only its
            // `aff_*` flags may be kept; the rest yields `régiak` and `szép` = back.
            if line.fkp_only && pos == Pos::Adjective {
                continue;
            }
            let has = |set: &[&str]| set.iter().any(|f| line.flags.iter().any(|g| g == f));
            // Votes are counted independently so a same-line X / non_X pair is visible as a tie.
            if has(fors) {
                win = true;
            }
            if has(againsts) {
                loss = true;
            }
        }
        if win && loss {
            ties.push(fors[0]);
        }
        // A tie means different things for the two shapes of rule, so it is resolved differently.
        //
        // When `on_loss` is `Some` the flags are a two-way choice (`back`/`front`,
        // `bareT`/`non_bareT`), and a row carrying both is not asserting either: it is recording
        // that the word vacillates. `advent NOUN back front` is such a row, and the corpus agrees
        // (`adventben` 3,184 : `adventban` 63). Abstaining leaves the computed harmony in force,
        // which is the right answer; picking the `for` side yields `adventban` and `szépak`.
        //
        // When `on_loss` is `None` the pair is presence against absence of an alternation, and the
        // marked member wins: abstaining would drop genuine optional alternations
        // such as `mű`. Either way the tie is reported, never swallowed.
        if win && loss && on_loss.is_some() {
            continue;
        }
        if win {
            if let Some(t) = on_win {
                set = set.with(*t);
            }
        } else if loss && let Some(t) = on_loss {
            set = set.with(*t);
        }
    }
    (set, ties)
}

/// The corpus-measured third-person possessive allomorph of each lemma that has one.
///
/// `NOM-POSS-006` says the choice between `-ja/-je` and `-a/-e` is lexical with no learnable
/// default, so it has to come from somewhere; this is the induced list, whose diagnostics are the
/// two possessive counts. It decides 2,288 lemmas, of which morphdb leaves 1,649 unmarked, and
/// overrides morphdb on the 16 where they disagree because every one of those is decisive:
/// `vége` 987,510 against `végje` 9, `farmerja` 551 against `farmera` 0.
///
/// Absent file means an empty map, which leaves the conversion exactly as it was.
///
/// Rules: `NOM-POSS-006`
fn possessive_list(dir: &Path) -> Result<BTreeMap<String, (Trait, bool)>> {
    let mut out = possessive_allomorph_list(dir)?;
    // The exceptions list fills in where the corpus had too little to say.
    for (lemma, t) in possessive_exceptions(dir)? {
        out.entry(lemma).or_insert((t, false));
    }
    Ok(out)
}

/// Reads the measured third-person possessive allomorph.
///
/// Rules: `NOM-POSS-006`
fn possessive_allomorph_list(dir: &Path) -> Result<BTreeMap<String, (Trait, bool)>> {
    let text = read_list(dir, "possessive-allomorph.tsv")?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let t = match *f.get(1)? {
                "ja" => Trait::PossessiveJa,
                "a" => Trait::PossessiveA,
                _ => return None,
            };
            Some(((*f.first()?).to_owned(), (t, f.get(2) == Some(&"yes"))))
        })
        .collect())
}

/// Reads the older hand-built exceptions list.
///
/// Rules: `NOM-POSS-006`
fn possessive_exceptions(dir: &Path) -> Result<BTreeMap<String, Trait>> {
    let text = read_list(dir, "poss3-ja-exceptions.tsv")?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let (lemma, decision, actual) = (*f.first()?, *f.get(1)?, *f.get(9)?);
            if decision != "member" && decision != "non-member" {
                return None;
            }
            let t = if actual == "ja" {
                Trait::PossessiveJa
            } else {
                Trait::PossessiveA
            };
            Some((lemma.to_owned(), t))
        })
        .collect())
}

/// One measured list, or an error naming the file; the conversion does not run without its
/// measurements.
fn read_list(dir: &Path, name: &str) -> Result<String> {
    let path = dir.join(name);
    fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()).into())
}

/// Lowering decisions per lexeme for the plural, accusative and possessive cells.
type LoweringCells = BTreeMap<(String, Pos), (Cell, Cell, Cell)>;

/// Every measured lexical value the converter reads from `local/lists/`.
///
/// Grouped so the conversion loop states one step, and so a new measurement has one place to be
/// added. See `docs/design/06-reproducibility.md` for the sources and counts.
struct Measured {
    possessives: BTreeMap<String, (Trait, bool)>,
    accusatives: BTreeMap<String, (Trait, bool)>,
    past_marker: BTreeMap<String, Vec<Trait>>,
    cluster_past: BTreeMap<String, Trait>,
    heavy: BTreeMap<String, Trait>,
    infinitive_linked: BTreeSet<String>,
    infinitive_other: BTreeSet<String>,
    lowering_cells: LoweringCells,
    present_syncope: BTreeMap<String, SyncopeCells>,
    antiharmonic: BTreeSet<(String, Pos)>,
    alternation: BTreeMap<(String, Pos), Alternation>,
    possessive_alt: BTreeMap<String, (PossessiveAxis, PossessiveAxis)>,
    possessed_plural: BTreeMap<String, Trait>,
    ustem: BTreeMap<(String, Pos), (Cell, Cell)>,
}

impl Measured {
    fn read(dir: &Path) -> Result<Self> {
        Ok(Self {
            possessives: possessive_list(dir)?,
            accusatives: accusative_list(dir)?,
            past_marker: past_list(dir)?,
            cluster_past: cluster_past_list(dir)?,
            heavy: heavy_list(dir)?,
            infinitive_linked: infinitive_list(dir, "linked")?,
            infinitive_other: infinitive_list(dir, "other")?,
            lowering_cells: lowering_cell_list(dir)?,
            present_syncope: present_syncope_list(dir)?,
            antiharmonic: antiharmonic_list(dir)?,
            alternation: alternation_list(dir)?,
            possessive_alt: possessive_alt_list(dir)?,
            possessed_plural: possessed_plural_list(dir)?,
            ustem: ustem_list(dir)?,
        })
    }

    /// Applies the measured lowering cells.
    ///
    /// Rules: `NOM-LOW-001`, `NOM-PL-001`, `NOM-ACC-005`
    fn lowering_only(&self, mut traits: TraitSet, lemma: &str, pos: Pos) -> TraitSet {
        if let Some(&(plural, accusative, possessive)) =
            self.lowering_cells.get(&(lemma.to_owned(), pos))
        {
            let plain = |c| c == Cell::Full;
            let low = |c| c == Cell::Syncopated;
            // `Low` is added only where a cell measured low, and dropped only where both measured
            // plain. An unmeasured cell says nothing either way (`cirok` is measured in the
            // accusative alone and keeps `cirkok`).
            if low(plural) || low(accusative) || low(possessive) {
                traits = traits.with(Trait::Low);
            }
            // `NOM-ACC-005` has a lowering stem take the vowel even in a bare class, and this axis
            // measures the accusative form itself rather than the class, so it settles the
            // question: `néhányat` 86,124 : `néhányt` 162 `[WC2]` against a `bareT` flag.
            if low(accusative) {
                traits = traits.without(Trait::BareAccusative);
            }
            if !(low(plural) || low(accusative) || low(possessive))
                && plain(plural)
                && plain(accusative)
            {
                traits = traits
                    .without(Trait::Low)
                    .without(Trait::LowPluralExempt)
                    .without(Trait::LowAccusativeExempt)
                    .without(Trait::LowPossessiveExempt);
            }
            if traits.has(Trait::Low) {
                if plain(plural) {
                    traits = traits.with(Trait::LowPluralExempt);
                }
                if plain(accusative) {
                    traits = traits.with(Trait::LowAccusativeExempt);
                }
                if plain(possessive) {
                    traits = traits.with(Trait::LowPossessiveExempt);
                }
            }
            if [plural, accusative, possessive].contains(&Cell::Both) {
                traits = traits.with(Trait::AlternationOptional);
            }
        }
        traits
    }

    /// Applies the axes a later axis's candidates must be built on top of.
    ///
    /// The candidates of any axis have to come from the traits the earlier axes have already
    /// settled, or the question asked of the corpus is the wrong one: `ajtó` still carried
    /// morphdb's `OaStem` when its possessed plural was measured, so the candidates were
    /// `ajtavai` against `ajtójai` when the forms are `ajtajai` and `ajtói`.
    fn settled(&self, traits: TraitSet, lemma: &str, pos: Pos) -> TraitSet {
        let traits = self.alternation_only(traits, lemma, pos);
        self.possessive_alt_only(traits, lemma)
    }

    /// Applies the measured u-stem cells.
    ///
    /// `NOM-STEM-012` drops a final `ú`/`ű`, and the cells come apart: `ifjak` beats `ifjúak`
    /// 28623 to 82 while `ifjút` beats `ifjat` 6097 to 660, so the plural takes the alternant and
    /// the accusative the free stem. That pair is `UStem` plus `FreeStemAccSup`, which morphdb
    /// already carries for `borjú` and for neither of `ifjú` and `varjú`.
    ///
    /// Rules: `NOM-STEM-012`, `NOM-ACC-009`
    fn ustem_only(&self, mut traits: TraitSet, lemma: &str, pos: Pos) -> TraitSet {
        let Some(&(plural, accusative)) = self.ustem.get(&(lemma.to_owned(), pos)) else {
            return traits;
        };
        let drops = |c| matches!(c, Cell::Syncopated | Cell::Both);
        if drops(plural) {
            traits = traits.with(Trait::UStem);
            if accusative == Cell::Full {
                traits = traits.with(Trait::FreeStemAccSup);
            }
        } else if plural == Cell::Full && accusative == Cell::Full {
            // Both cells keep the vowel, so the lexeme is not of this class at all. A v-stem is
            // left alone: its own alternation supplies the drop and is measured elsewhere.
            if !traits.has(Trait::VStem) {
                traits = traits.without(Trait::UStem);
            }
        }
        traits
    }

    /// Applies the `ajtó` possessive measurement alone.
    fn possessive_alt_only(&self, traits: TraitSet, lemma: &str) -> TraitSet {
        let Some(&(singular, plural)) = self.possessive_alt.get(lemma) else {
            return traits;
        };
        let cell = |traits: TraitSet, axis, t| match axis {
            PossessiveAxis::Plain => traits.without(t),
            PossessiveAxis::Alt => traits.with(t),
            PossessiveAxis::Both => traits.with(t).with(Trait::AlternationOptional),
            PossessiveAxis::Unmeasured => traits,
        };
        let traits = cell(traits, singular, Trait::PossessiveAlt);
        cell(traits, plural, Trait::PossessedPluralAlt)
    }

    /// Applies the stem-alternation measurement alone.
    ///
    /// Separate because the possessive candidates are emitted from its result: for the `ajtó`
    /// class morphdb's own v-stem marking is the error, so the plain candidate has to be built
    /// without it, while a genuine v-stem must keep it or the plain candidate for `ló` comes out
    /// `lója` instead of `lova`.
    fn alternation_only(&self, mut traits: TraitSet, lemma: &str, pos: Pos) -> TraitSet {
        // NOM-STEM-008/010/012: the marking is kept only where the corpus shows the alternation.
        if let Some(&d) = self.alternation.get(&(lemma.to_owned(), pos)) {
            if d.none {
                for t in ALTERNATIONS {
                    traits = traits.without(t);
                }
                traits = traits
                    .without(Trait::FreeStemAccSup)
                    .without(Trait::FreeStemPlural);
            } else {
                if d.free_plural {
                    traits = traits.with(Trait::FreeStemPlural);
                }
                if d.free_accusative {
                    traits = traits.with(Trait::FreeStemAccSup);
                }
            }
            if d.optional {
                traits = traits.with(Trait::AlternationOptional);
            }
        }
        traits
    }

    /// Applies the measurements to the flag-derived traits of one lemma.
    ///
    /// A measurement replaces whatever the flags said on its own axis: the flags have no default to
    /// fall back on and the measurement does. morphdb also keeps both sides of a tie, which left
    /// `fogad` carrying `PastFinalT` and `PastFinalVtt` at once.
    fn apply(&self, mut traits: TraitSet, lemma: &str, pos: Pos, seams: &[u32]) -> TraitSet {
        // NOM-ACC-005, NOM-ACC-009.
        if pos == Pos::Noun
            && let Some(&(t, both)) = self.accusatives.get(lemma)
        {
            traits = traits
                .without(Trait::BareAccusative)
                .without(Trait::LinkedAccusative)
                .with(t);
            if both {
                traits = traits.with(Trait::AlternationOptional);
            }
        }
        // NOM-POSS-006: the list is keyed by lemma, and a verb takes no possessive.
        if pos != Pos::Verb
            && let Some(&(t, other)) = self.possessives.get(lemma)
        {
            traits = traits
                .without(Trait::PossessiveA)
                .without(Trait::PossessiveJa)
                .with(t);
            if other {
                traits = traits.with(Trait::PossessiveOptional);
            }
        }
        // NOM-LOW-001: the marking is per lexeme and the cells come apart, so each exemption is
        // measured on its own cell: `olaj` gives `olajat` and `olajok`.
        traits = self.lowering_only(traits, lemma, pos);
        // NOM-PPL-001: the possessed plural has its own allomorph choice, written only where it
        // disagrees with the singular so the common case stays derived.
        if let Some(&t) = self.possessed_plural.get(lemma) {
            // Written only when generation would not get there by itself. Comparing against the
            // singular's allomorph is wrong, as the cell has its own default: after a
            // vowel that default is the bare `-i`, so `ajtó` needs the trait although its singular
            // takes `-ja` too.
            let bare = traits
                .without(Trait::PossessedPluralA)
                .without(Trait::PossessedPluralJa);
            traits = if possessed_plural_form(lemma, pos, bare)
                == possessed_plural_form(lemma, pos, bare.with(t))
            {
                bare
            } else {
                bare.with(t)
            };
        }
        // NOM-STEM-013: the `ajtó` possessive is its own axis, because it is a possessive-only
        // alternation and the cells the list above measures say nothing about it.
        traits = self.possessive_alt_only(traits, lemma);
        traits = self.alternation_only(traits, lemma, pos);
        // NOM-STEM-012: the u-stem drop is the plural's cell for this class, so it is measured
        // per cell like the others.
        traits = self.ustem_only(traits, lemma, pos);
        // PH-HARM-011: the antiharmonic class is listed because the spelling cannot show it.
        if self.antiharmonic.contains(&(lemma.to_owned(), pos)) {
            traits = traits.without(Trait::Front).with(Trait::Back);
        }
        if pos != Pos::Verb {
            return drop_computed_harmony(drop_contradictions(traits), lemma, seams);
        }
        // VRB-STEM-007: both present cells of the epenthetic syncope are measured, and they
        // disagree for a third of the class.
        if let Some(&cells) = self.present_syncope.get(lemma) {
            // The trait marking a syncope the rule already computes would restate it, so it is
            // only written where the computed answer is the other one.
            let computed = lemma.ends_with(['g', 'z', 'm']);
            for (cell, full, sync) in [
                (
                    cells.definite,
                    Trait::DefiniteFullStem,
                    Trait::DefiniteSyncope,
                ),
                (cells.first, Trait::PresentFullStem, Trait::PresentSyncope),
                (cells.past, Trait::PastFullStem, Trait::PastSyncope),
            ] {
                match cell {
                    Cell::Full => traits = traits.with(full),
                    Cell::Syncopated if !computed => traits = traits.with(sync),
                    Cell::Syncopated | Cell::Both | Cell::Unmeasured => {}
                }
            }
            if cells.other_attested {
                traits = traits.with(Trait::AlternationOptional);
            }
        }
        // VRB-LINK-002/VRB-INF-001: the infinitive is its own cell of the heaviness axis, and the
        // coda computation is wrong for a small class: `edzeni` 53,767 : `edzni` 10 [WC2].
        if self.infinitive_linked.contains(lemma) {
            traits = traits.with(Trait::HeavyStem);
        }
        if self.infinitive_other.contains(lemma) {
            traits = traits.with(Trait::AlternationOptional);
        }
        // VRB-LINK-002: heaviness is measured, and light is the computed default; a stem
        // measured both ways keeps both (VRB-LINK-010).
        if let Some(&t) = self.heavy.get(lemma) {
            traits = traits.with(t);
        }
        // VRB-PAST-001: the cluster stems decide only the non-final cell.
        if let Some(&t) = self.cluster_past.get(lemma) {
            traits = traits
                .without(Trait::PastNonfinalT)
                .without(Trait::PastNonfinalVtt)
                .with(t);
        }
        // VRB-PAST-004, VRB-PAST-008, on both axes.
        if let Some(ts) = self.past_marker.get(lemma) {
            traits = traits
                .without(Trait::PastFinalT)
                .without(Trait::PastFinalVtt)
                .without(Trait::PastNonfinalT)
                .without(Trait::PastNonfinalVtt);
            for &t in ts {
                traits = traits.with(t);
            }
        }
        // A verb's harmony domain is its present stem: `fürdik` computes front from its `-ik`
        // and front-rounded from `fürd`, which is the class its suffixes take (`fürdöm`).
        let stem = picur_morph_core::present_stem(lemma);
        drop_computed_harmony(drop_contradictions(traits), stem, seams)
    }
}

/// Drops a harmony trait that merely restates what the spelling computes.
///
/// `Trait::Back` and `Trait::Front` mean "harmony **against** the spelling"; the computed class is
/// the default and needs no mark. morphdb has one `back` boolean and emits a trait either way, and
/// a bare `Front` on a front-rounded spelling is not redundant but destructive: it downgrades the
/// rounding (`csütörtökön`, `tükrön`). Of the 327 nouns marked this way, 156 of the attested ones
/// take the rounded link and two do not, one of those a verb homograph (`pörköltek`).
///
/// Rules: `PH-HARM-004`, `PH-HARM-011`
fn drop_computed_harmony(traits: TraitSet, lemma: &str, seams: &[u32]) -> TraitSet {
    use picur_morph_ortho::Harmony;
    // `DER-COMP-008` makes the last member the harmony domain, so the engine computes the class
    // from it and this check asks the same question: `nyugdíj` computes Back as a whole and Front
    // from `díj`, so its `Back` is not redundant.
    let domain = seams
        .last()
        .and_then(|&at| lemma.get(at as usize..))
        .filter(|m| !m.is_empty())
        .unwrap_or(lemma);
    let computed = picur_morph_ortho::harmony_of(domain);
    let front = matches!(computed, Harmony::Front | Harmony::FrontRounded);
    if front && (traits.has(Trait::Front) || traits.has(Trait::FrontRounded)) {
        return traits.without(Trait::Front).without(Trait::FrontRounded);
    }
    if computed == Harmony::Back && traits.has(Trait::Back) {
        return traits.without(Trait::Back);
    }
    traits
}

/// Removes any pair of traits that state opposite things.
///
/// morphdb keeps both sides of a tie, which is not a lexical value but an admission that its flags
/// disagree. Carrying both lets whichever the generator tests first win silently: `mond` had
/// `PastFinalT` and `PastFinalVtt` together and produced `mondt`. Dropping the pair hands the cell
/// back to the rules, which is what an unmarked verb gets and what `mond` needs (`mondott`).
fn drop_contradictions(mut traits: TraitSet) -> TraitSet {
    // PH-HARM-011: a stem flagged both ways is one the norm suffixes in both harmonies
    // (`norvégok` ~ `norvégek`), so the tie is kept as a value of its own.
    if traits.has(Trait::Back) && traits.has(Trait::Front) {
        traits = traits
            .without(Trait::Back)
            .without(Trait::Front)
            .with(Trait::HarmonyOptional);
    }
    for (a, b) in [
        (Trait::PastFinalT, Trait::PastFinalVtt),
        (Trait::PastNonfinalT, Trait::PastNonfinalVtt),
        (Trait::BareAccusative, Trait::LinkedAccusative),
        (Trait::PossessiveA, Trait::PossessiveJa),
    ] {
        if traits.has(a) && traits.has(b) {
            traits = traits.without(a).without(b);
        }
    }
    traits
}

/// What the corpus says about a marked stem alternation, cell by cell.
///
/// The traits are marked per lexeme but the alternation is nearly always cell-restricted, so one
/// verdict for the lexeme cannot express what the corpus shows. `odú` pluralises `odúk` and takes
/// `odút`, but its possessive really is `odva`; `szél` is two lexemes under one spelling and only
/// the possessive separates them, `széle` 37,411 beside `szele` 16,866 `[WC2]`. Each cell therefore
/// gets its own flag, and `optional` records a cell where both alternants are attested in quantity
/// rather than one winning.
///
/// Rules: `NOM-STEM-008`, `NOM-STEM-010`, `NOM-STEM-012`
#[derive(Clone, Copy, Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "one independent flag per measured cell, which is the point"
)]
struct Alternation {
    /// No cell alternates: the marking is simply wrong and comes off.
    none: bool,
    /// The plural takes the free stem.
    free_plural: bool,
    /// The accusative and superessive take the free stem.
    free_accusative: bool,
    /// Some cell admits either alternant.
    optional: bool,
}

/// Reads the measured stem alternations.
fn alternation_list(dir: &Path) -> Result<BTreeMap<(String, Pos), Alternation>> {
    let text = read_list(dir, "stem-alternation.tsv")?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let lemma = *f.first()?;
            let pos = match *f.get(1)? {
                "Noun" => Pos::Noun,
                "Adjective" => Pos::Adjective,
                _ => return None,
            };
            let (plural, accusative, possessive) = (*f.get(2)?, *f.get(3)?, *f.get(4)?);
            let cells = [plural, accusative, possessive];
            let decision = Alternation {
                // Only when no cell shows the alternation at all. An unmeasured cell does not
                // count as evidence either way, so `-` neither confirms nor denies.
                none: cells.iter().all(|c| *c == "free" || *c == "-") && cells.contains(&"free"),
                free_plural: plural == "free",
                free_accusative: accusative == "free",
                optional: cells.contains(&"both"),
            };
            if !decision.none
                && !decision.free_plural
                && !decision.free_accusative
                && !decision.optional
            {
                return None;
            }
            Some(((lemma.to_owned(), pos), decision))
        })
        .collect())
}

/// A compound's stem class, taken from its head.
///
/// `DER-COMP-007` inflects a compound on its last member only, so its stem class is the head's by
/// construction, not a property the compound carries of its own. 24,284 compounds have a head that
/// is itself an entry and only 187 disagree with it, which is the measure of how much this is a
/// rule and how much the flags are noise: `jármű` came out with a `Shortening` its own head `mű`
/// does not have, so `járművek` was ill formed and `jármüvek` was the form generated.
///
/// Rules: `DER-COMP-007`, `DER-COMP-008`
fn head_alternation(
    flags: &BTreeMap<(Pos, &str), TraitSet>,
    lemma: &str,
    pos: Pos,
    lines: &[Line],
    traits: TraitSet,
) -> TraitSet {
    if !matches!(pos, Pos::Noun | Pos::Adjective) {
        return traits;
    }
    let parts = lines
        .iter()
        .find(|l| !l.parts.is_empty())
        .map(|l| seams(lemma, &l.parts));
    let Some(&seam) = parts.as_ref().and_then(|p| p.last()) else {
        return traits;
    };
    let head = match lemma.get(seam as usize..) {
        Some(h) if !h.is_empty() => h,
        _ => return traits,
    };
    let Some(&theirs) = flags.get(&(pos, head)) else {
        return traits;
    };
    let mut out = traits;
    for t in ALTERNATIONS {
        out = if theirs.has(t) {
            out.with(t)
        } else {
            out.without(t)
        };
    }
    out
}

/// Reads the measured possessed-plural allomorph.
///
/// Rules: `NOM-PPL-001`, `NOM-POSS-006`
fn possessed_plural_list(dir: &Path) -> Result<BTreeMap<String, Trait>> {
    let text = read_list(dir, "possessed-plural.tsv")?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let t = match *f.get(1)? {
                "ja" => Trait::PossessedPluralJa,
                "a" => Trait::PossessedPluralA,
                _ => return None,
            };
            Some(((*f.first()?).to_owned(), t))
        })
        .collect())
}

/// What the corpus says about the `ajtó` possessive alternation.
#[derive(Clone, Copy)]
enum PossessiveAxis {
    /// The alternation is real: `ajtaja`, `ideje`.
    Alt,
    /// It is not: `sója`, `szőlője`.
    Plain,
    /// Both are attested in quantity: `ajtajai` 2,388 : `ajtói` 2,802 `[WC2]`.
    Both,
    /// Too little evidence; the flags keep whatever they say.
    Unmeasured,
}

/// Reads the measured `ajtó` possessive alternation.
///
/// Rules: `NOM-STEM-013`
fn possessive_alt_list(dir: &Path) -> Result<BTreeMap<String, (PossessiveAxis, PossessiveAxis)>> {
    let text = read_list(dir, "possessive-alt.tsv")?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let axis = |s: &str| match s {
                "alt" => Some(PossessiveAxis::Alt),
                "plain" => Some(PossessiveAxis::Plain),
                "both" => Some(PossessiveAxis::Both),
                _ => None,
            };
            let singular = axis(f.get(1)?).unwrap_or(PossessiveAxis::Unmeasured);
            let plural = axis(f.get(2)?).unwrap_or(PossessiveAxis::Unmeasured);
            Some(((*f.first()?).to_owned(), (singular, plural)))
        })
        .collect())
}

/// The traits that make a stem alternate before a vowel-initial suffix.
const ALTERNATIONS: [Trait; 5] = [
    Trait::VStem,
    Trait::OaStem,
    Trait::UStem,
    Trait::Shortening,
    Trait::Epenthetic,
];

/// The corpus-measured antiharmonic stems: back harmony against a front or neutral spelling.
///
/// `PH-HARM-011` says the class has to be listed, because nothing in the spelling shows it:
/// `izzik` takes `izzanak` and `bír` takes `bírnak`. The induced list agrees across morphdb, the
/// corpus, foma, `hunspell`, `UniMorph` and `kaikki` on its high-confidence members, which is
/// stronger evidence than the frequency tie-break in `resolve_harmony`. A reviewed-but-undecided
/// row is
/// left to the rules.
fn antiharmonic_list(dir: &Path) -> Result<BTreeSet<(String, Pos)>> {
    let text = read_list(dir, "antiharmonic.tsv")?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let lemma = *f.first()?;
            if *f.get(1)? != "member" {
                return None;
            }
            let pos = match *f.get(7)? {
                "ADJ" => Pos::Adjective,
                "NOUN" => Pos::Noun,
                "VERB" => Pos::Verb,
                _ => return None,
            };
            Some((lemma.to_owned(), pos))
        })
        .collect())
}

/// The corpus-measured past marker of the consonant-cluster verbs, on the non-final axis.
///
/// `VRB-PAST-001` calls the cluster stems lexical before a person suffix, and they are: `mondtam`
/// beats `mondottam` 536,884 to 12,219 while `kezdtem` loses to `kezdettem`. The list decides only
/// this axis, since its diagnostics are the `-tAm`/`-tAk` forms; the word-final cell stays with the
/// rules.
fn cluster_past_list(dir: &Path) -> Result<BTreeMap<String, Trait>> {
    let text = read_list(dir, "cc-past-verbs.tsv")?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let lemma = *f.first()?;
            let vowel: u64 = f.get(5)?.parse().unwrap_or(0);
            let bare: u64 = f.get(6)?.parse().unwrap_or(0);
            let t = match *f.get(1)? {
                "none" => Trait::PastNonfinalT,
                "vowel" => Trait::PastNonfinalVtt,
                // A doublet still has to generate one form, so the corpus majority decides.
                "both" if bare != vowel => {
                    if bare > vowel {
                        Trait::PastNonfinalT
                    } else {
                        Trait::PastNonfinalVtt
                    }
                }
                _ => return None,
            };
            Some((lemma.to_owned(), t))
        })
        .collect())
}

/// The epenthetic verbs that keep their full stem in the present.
///
/// `VRB-STEM-007` calls the syncope obligatory after `g`, `z` and `m`, and for 21 of the 199
/// attested verbs the corpus says otherwise: `képezi` beats `képzi` 201,559 to 11,758 while the
/// same verb's `-Ás` derivative is `képzés` by 664,766 to 78. The split is by cell, so the trait
/// What the corpus says about one cell of the epenthetic present syncope.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    /// The full stem wins: `képezi`, `énekelek`.
    Full,
    /// The syncopated stem wins: `közli`, `érzek`.
    Syncopated,
    /// Both are attested in their own right, so neither is an error.
    Both,
    /// Too little evidence, or the two forms coincide. The computed rule decides.
    Unmeasured,
}

/// The measured verdict for both present cells of one epenthetic verb.
#[derive(Clone, Copy)]
struct SyncopeCells {
    /// Before the definite `-i`.
    definite: Cell,
    /// Before the vowel-initial indefinite endings.
    first: Cell,
    /// Before the past marker. Mostly the full stem, unlike the two present cells: `közölt`
    /// 61,583 against no `közlött` at all, beside `közli` 120,588 `[WC2]`.
    past: Cell,
    /// Whether the losing form of either cell is attested in its own right, which makes it
    /// something to accept rather than something to flag.
    other_attested: bool,
}

/// Reads the measured epenthetic present syncope, per cell.
///
/// Rules: `VRB-STEM-007`
fn present_syncope_list(dir: &Path) -> Result<BTreeMap<String, SyncopeCells>> {
    let text = read_list(dir, "present-syncope.tsv")?;
    let cell = |s: &&str| match *s {
        "full" => Cell::Full,
        "syncopated" => Cell::Syncopated,
        _ => Cell::Unmeasured,
    };
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            Some((
                (*f.first()?).to_owned(),
                SyncopeCells {
                    definite: cell(f.get(1)?),
                    first: cell(f.get(3)?),
                    past: cell(f.get(5)?),
                    other_attested: [*f.get(2)?, *f.get(4)?, *f.get(6)?].contains(&"yes"),
                },
            ))
        })
        .collect())
}

/// The corpus-measured heavy geminate verb stems.
///
/// `VRB-LINK-002` says the geminates go both ways and nothing in the shape decides, so heaviness
/// is measured: `hallani` 270,582 against `hallni` 165 makes `hall` heavy, while `állni` 310,223
/// against `állani` 5,035 makes `áll` light. Light is what the shape computes, so only the heavy
/// ones are marked.
fn heavy_list(dir: &Path) -> Result<BTreeMap<String, Trait>> {
    let text = read_list(dir, "geminate-verbs.tsv")?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let lemma = *f.first()?;
            // VRB-LINK-010: a stem measured with both forms keeps both (`függenek` ~ `függnek`).
            let trait_ = match *f.get(9)? {
                "vowel" => Trait::HeavyStem,
                "both" => Trait::LinkOptional,
                _ => return None,
            };
            Some((lemma.to_owned(), trait_))
        })
        .collect())
}

/// Reads the measured lowering cells: whether the plural and the accusative are exempt, and
/// whether either cell shows both forms.
///
/// Rules: `NOM-LOW-001`, `NOM-PL-001`, `NOM-ACC-005`
fn lowering_cell_list(dir: &Path) -> Result<LoweringCells> {
    let text = read_list(dir, "lowering-cells.tsv")?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let pos = match *f.get(1)? {
                "Noun" => Pos::Noun,
                "Adjective" => Pos::Adjective,
                _ => return None,
            };
            // `Cell` is shared with the syncope axis, where `Full` is the unaltered form and
            // `Syncopated` the altered one. Here `Full` is the plain link and `Syncopated` the low.
            let cell = |s: &&str| match *s {
                "plain" => Cell::Full,
                "low" => Cell::Syncopated,
                "both" => Cell::Both,
                _ => Cell::Unmeasured,
            };
            Some((
                ((*f.first()?).to_owned(), pos),
                (cell(f.get(2)?), cell(f.get(3)?), cell(f.get(4)?)),
            ))
        })
        .collect())
}

/// Reads the measured u-stem cells.
///
/// `NOM-STEM-012` drops a final `ú`/`ű` before a suffix, and the cells come apart: `ifjak` beats
/// `ifjúak` 28623 to 82 while `ifjút` beats `ifjat` 6097 to 660, so the drop is the plural's and
/// the accusative keeps the free stem. morphdb has the pair for `borjú` and neither trait for
/// `ifjú` or `varjú`, which is what the measurement supplies.
///
/// Rules: `NOM-STEM-012`, `NOM-ACC-009`
fn ustem_list(dir: &Path) -> Result<BTreeMap<(String, Pos), (Cell, Cell)>> {
    let text = read_list(dir, "ustem.tsv")?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let pos = match *f.get(1)? {
                "Noun" => Pos::Noun,
                "Adjective" => Pos::Adjective,
                _ => return None,
            };
            // `Full` is the form that keeps the vowel, `Syncopated` the one that drops it.
            let cell = |s: &&str| match *s {
                "keep" => Cell::Full,
                "drop" => Cell::Syncopated,
                "both" => Cell::Both,
                _ => Cell::Unmeasured,
            };
            Some((
                ((*f.first()?).to_owned(), pos),
                (cell(f.get(2)?), cell(f.get(3)?)),
            ))
        })
        .collect())
}

/// Reads the measured infinitive linking vowel.
///
/// `which` is `linked` for the verbs that take it, or `other` for those whose losing form is
/// attested in its own right.
///
/// Rules: `VRB-LINK-002`, `VRB-INF-001`
fn infinitive_list(dir: &Path, which: &str) -> Result<BTreeSet<String>> {
    let text = read_list(dir, "infinitive-link.tsv")?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let keep = if which == "linked" {
                *f.get(1)? == "linked"
            } else {
                *f.get(2)? == "yes"
            };
            let lemma = (*f.first()?).to_owned();
            keep.then_some(lemma)
        })
        .collect())
}

/// The corpus-measured past marker of the verbs whose past is lexical.
///
/// `VRB-PAST-004` calls the geminate past "lexical (closed)" and `VRB-PAST-008` makes the
/// `-ad`/`-ed` split turn on an `ad_suffixal` flag, so neither can be computed. Each list gives a
/// decision per axis, because the two come apart: `függ` is `függött` word-finally but `függtem`
/// before a person suffix, which the measurement reproduces independently of the rulebook saying
/// so. A fluctuating or unattested verb is left unmarked and falls back to the rules.
fn past_list(dir: &Path) -> Result<BTreeMap<String, Vec<Trait>>> {
    let mut out: BTreeMap<String, Vec<Trait>> = BTreeMap::new();
    for file in ["geminate-past.tsv", "ad-past.tsv", "past-marker.tsv"] {
        let text = read_list(dir, file)?;
        for line in text.lines().skip(1) {
            let f: Vec<&str> = line.split('\t').collect();
            let (Some(lemma), Some(final_), Some(nonfinal)) = (f.first(), f.get(1), f.get(2))
            else {
                continue;
            };
            let mut ts = Vec::new();
            match *final_ {
                "t" => ts.push(Trait::PastFinalT),
                "ott" => ts.push(Trait::PastFinalVtt),
                _ => {}
            }
            match *nonfinal {
                "t" => ts.push(Trait::PastNonfinalT),
                "ott" => ts.push(Trait::PastNonfinalVtt),
                _ => {}
            }
            // The generic list carries whether the losing form of either cell is attested in its
            // own right: `mondtam` 457,946 beside `mondottam` 10,973 [WC2].
            if f.get(3) == Some(&"yes") {
                ts.push(Trait::AlternationOptional);
            }
            if !ts.is_empty() {
                out.insert((*lemma).to_owned(), ts);
            }
        }
    }
    Ok(out)
}

/// The corpus-measured accusative of each noun that has one.
///
/// `NOM-ACC-009` says a stem in sonorant + sibilant "may be lexically marked for bare `-t`", and
/// `NOM-ACC-005`'s rule that a lowering stem takes the vowel has exceptions, so the accusative of a
/// given noun is not always computable: `pénz` gives `pénzt` and `oldal` gives `oldalt` although
/// both would take the vowel by rule.
///
/// The list is produced by `local/tools/verify_accusative.py` from the two candidate forms this
/// crate generates, counted with **both** a tag and a lemma filter. That matters: an earlier list
/// built from raw counts made `áll` a bare-accusative noun on the strength of `állt` 1,107,516,
/// which is the verb. Filtered by lemma and tag, the noun has no attested accusative at all.
///
/// Only decisions with a tenfold margin are kept, which is 28,659 of 34,864. `NOM-ACC-009`
/// names the cluster stems that vary (`csimpánzt` ~ `csimpánzot`), and those are read from the
/// undecided rows of the raw list: a cluster stem whose two forms both clear the floor without a
/// tenfold margin has both.
///
/// Rules: `NOM-ACC-005`, `NOM-ACC-009`
fn accusative_list(dir: &Path) -> Result<BTreeMap<String, (Trait, bool)>> {
    let text = read_list(dir, "bare-accusative-verified.tsv")?;
    let mut out: BTreeMap<String, (Trait, bool)> = text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let t = match *f.get(1)? {
                "bare" => Trait::BareAccusative,
                "linked" => Trait::LinkedAccusative,
                _ => return None,
            };
            Some(((*f.first()?).to_owned(), (t, false)))
        })
        .collect();
    let raw = read_list(dir, "bare-accusative.tsv")?;
    for line in raw.lines().skip(1) {
        let f: Vec<&str> = line.split('\t').collect();
        if f.get(1) != Some(&"review") || f.get(8) != Some(&"cluster") {
            continue;
        }
        let (Some(bare), Some(linked)) = (
            f.get(5).and_then(|n| n.parse::<u64>().ok()),
            f.get(6).and_then(|n| n.parse::<u64>().ok()),
        ) else {
            continue;
        };
        if bare < MIN_ATTESTED
            || linked < MIN_ATTESTED
            || bare >= 10 * linked
            || linked >= 10 * bare
        {
            continue;
        }
        // Stated as bare with the linked form accepted beside it: the analyser reads the linked
        // variant of a bare stem, and the linked form is what the default gives anyway.
        if let Some(lemma) = f.first() {
            out.entry((*lemma).to_owned())
                .or_insert((Trait::BareAccusative, true));
        }
    }
    Ok(out)
}

/// The attestation floor of a measured decision.
const MIN_ATTESTED: u64 = 20;

/// Picks the harmony a back/front tie should resolve to, from corpus frequency.
///
/// A row carrying both `back` and `front` is recording that the word vacillates, not asserting
/// either, so the flags alone cannot decide. The corpus can, and it decides per word: `havernak`
/// beats `havernek` 3,260 to 161 while `szépek` beats `szépak` 87,441 to 1. Both rows look
/// identical in morphdb.
///
/// The dative is the probe because it is class II and takes no linking vowel or stem alternation,
/// so the two candidates are plain concatenations and the comparison is not confounded.
///
/// `None` when neither form is attested, which leaves the computed harmony in force.
///
/// Rules: `PH-HARM-011`
fn resolve_harmony(lemma: &str) -> Option<Trait> {
    let back = frequency(&format!("{lemma}nak"));
    let front = frequency(&format!("{lemma}nek"));
    if back == 0 && front == 0 {
        return None;
    }
    // A bucket is a half-octave, so one step apart is already a clear margin.
    match back.cmp(&front) {
        std::cmp::Ordering::Greater => Some(Trait::Back),
        std::cmp::Ordering::Less => Some(Trait::Front),
        std::cmp::Ordering::Equal => None,
    }
}

/// Picks the accusative trait that reproduces a golden for this lemma, if one exists.
///
/// Rules: `NOM-ACC-006`
fn resolve_accusative(
    lemma: &str,
    pos: Pos,
    traits: TraitSet,
    goldens: &BTreeMap<String, Vec<String>>,
) -> Option<TraitSet> {
    let wanted = goldens.get(lemma)?;
    let base = traits
        .iter()
        .filter(|t| !matches!(t, Trait::BareAccusative | Trait::LinkedAccusative));
    let base: TraitSet = base.collect();
    for candidate in [Trait::LinkedAccusative, Trait::BareAccusative] {
        let t = base.with(candidate);
        let entry = Entry {
            lemma: lemma.into(),
            parts: Box::default(),
            pos,
            traits: t,
            alt: None,
            last_sound: None,
        };
        if let Ok(form) = generate(&entry, &[Morph::Case(Case::Acc)])
            && wanted.iter().any(|w| w == &form)
        {
            return Some(t);
        }
    }
    None
}

/// Seam offsets, if the parts really concatenate to the lemma.
fn seams(lemma: &str, parts: &[String]) -> Vec<u32> {
    if parts.len() < 2 || parts.concat() != lemma {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut at = 0usize;
    for p in &parts[..parts.len() - 1] {
        at += p.len();
        if let Ok(v) = u32::try_from(at) {
            out.push(v);
        }
    }
    out
}

/// Entries morphdb lists that are inflected forms of another lemma, not lemmas.
///
/// A homograph is not a reason to drop one: `ment` really is both the past of `megy` and the verb
/// "saves". These are the forms with no reading of their own, where keeping the entry shadowed the
/// correct analysis instead of adding to it. `volna` is the counterfactual conditional of `van`
/// (`VRB-IRR-001`), and while it was a lemma nothing could read it as one.
/// `fia` is the third person possessive of `fiú`, which morphdb marks `NoPossessive` because it
/// cannot state the `fi` stem; with the paradigm generated it is the reading, and the entry only
/// shadowed it. `mentes` is an adjective alone: Webcorpus 2 has no noun form of it but `mentest`,
/// which is the unaccented `mentést`, and the noun entry headed every `-mentes` compound as a
/// non-lowering noun (`*hibamentest`, `DER-ADJ-016`).
const NOT_A_LEMMA: &[(Pos, &str)] = &[
    (Pos::Verb, "volna"),
    (Pos::Noun, "fia"),
    (Pos::Noun, "mentes"),
];

/// The long present lemma of every sz-d verb beside its short one: `aluszik` for `alszik`.
///
/// `VRB-STEM-012` reads `alusz-` as a present indicative stem of `alszik`, not as a verb of its
/// own, so a morphdb row under the long lemma is folded onto the short one, preverb and all
/// (`átaluszik` is `átalszik`); listed apart it would conjugate the long stem through the
/// imperative (`*alusszanak`).
fn long_sz_d_lemmas() -> Vec<(String, String)> {
    picur_morph_lexicon::lexicon()
        .iter()
        .filter(|e| e.pos == Pos::Verb && e.traits.has_any([Trait::SzDStem, Trait::SzDVStem]))
        .filter_map(|e| {
            let d = picur_morph_core::verbal::alternant_stem(e)?;
            let root = d.strip_suffix('d')?;
            Some((format!("{root}szik"), e.lemma.to_string()))
        })
        .collect()
}

/// Folds a verb row under a long sz-d lemma onto the short one, seams included.
fn fold_sz_d(key: &mut (Pos, String), parsed: &mut Line, long: &[(String, String)]) {
    if key.0 != Pos::Verb {
        return;
    }
    let Some(short) = short_sz_d_lemma(&key.1, long) else {
        return;
    };
    if short == key.1 {
        return;
    }
    // The seams follow the lemma: the last part is the verb itself.
    if let Some(last) = parsed.parts.last_mut()
        && let Some(prefix) = key.1.len().checked_sub(last.len())
        && short.is_char_boundary(prefix)
    {
        short[prefix..].clone_into(last);
    }
    key.1 = short;
}

/// The short lemma a long sz-d lemma folds onto, if `lemma` is one.
fn short_sz_d_lemma(lemma: &str, long: &[(String, String)]) -> Option<String> {
    long.iter().find_map(|(l, short)| {
        let prefix = lemma.strip_suffix(l.as_str())?;
        let preverb = prefix.is_empty()
            || picur_morph_lexicon::lexicon()
                .get(prefix)
                .iter()
                .any(|p| p.pos == Pos::Preverb);
        preverb.then(|| format!("{prefix}{short}"))
    })
}

/// Whether a postposition row is a cell of a paradigm the engine computes.
///
/// `postp+case.lexicon` lists the personal forms of the case stems and postpositions as lemmas
/// of their own (`nekem`, `értem`, `hozzátok`, `őket`). `PND-PERS-003` and `PND-POSTP-005` make
/// those cells of `én`, `ért`, `hozzá` and `ők`, and a listed copy only shadows the paradigm with
/// a wrong lemma.
fn paradigm_form(key: &(Pos, String)) -> bool {
    use picur_morph_core::{postposition, pronoun};
    let (pos, lemma) = key;
    *pos == Pos::Postposition
        && (pronoun::analyse(lemma).is_some()
            || pronoun::CELLS
                .iter()
                .any(|(p, n)| pronoun::nominative(*p, *n) == lemma)
            || postposition::analyse(lemma).is_some_and(|f| f.lemma != lemma))
}

/// Converts `dir` into RON under the lexicon crate.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn convert(dir: &Path) -> Result {
    let mut groups: BTreeMap<(Pos, String), Vec<Line>> = BTreeMap::new();
    let long_sz_d = long_sz_d_lemmas();
    let mut read = 0usize;
    for (file, forced, loose) in FILES {
        let path = dir.join(file);
        let Ok(bytes) = fs::read(&path) else {
            println!("  skipped (absent): {file}");
            continue;
        };
        let (text, _, _) = ISO_8859_2.decode(&bytes);
        let mut n = 0usize;
        for line in text.lines() {
            if let Some((mut key, mut parsed)) = parse(line, *forced, *loose) {
                if NOT_A_LEMMA.contains(&(key.0, key.1.as_str())) || paradigm_form(&key) {
                    continue;
                }
                fold_sz_d(&mut key, &mut parsed, &long_sz_d);
                groups.entry(key).or_default().push(parsed);
                n += 1;
            }
        }
        println!("  {file:<26} {n:>7}");
        read += n;
    }
    if groups.is_empty() {
        return Err(format!("no entries parsed from {}", dir.display()).into());
    }

    // Contradictory flags are resolved against the rulebook, not by a blanket preference. The
    // specification is what says `orr` gives `orrot` and forbids `orrt`; morphdb carries both
    // `bareT` and `non_bareT` for it and cannot settle the question itself.
    let goldens = crate::eval::goldens_by_input(Path::new("docs/grammar/goldens.ron"))?;

    let mut all_ties = 0usize;
    let mut resolved = 0usize;
    let mut by_pos: BTreeMap<Pos, Vec<Entry>> = BTreeMap::new();
    let measured = Measured::read(Path::new("local/lists"))?;
    let mut out = Candidates::default();
    // The flag-derived traits of every lemma, so a compound can be given its head's stem class
    // before anything else looks at it.
    let flags: BTreeMap<(Pos, &str), TraitSet> = groups
        .iter()
        .map(|((pos, lemma), lines)| ((*pos, lemma.as_str()), votes(lines, *pos).0))
        .collect();
    for ((pos, lemma), lines) in &groups {
        let (mut traits, ties) = votes(lines, *pos);
        let parts = lines
            .iter()
            .find(|l| !l.parts.is_empty())
            .map(|l| seams(lemma, &l.parts))
            .unwrap_or_default();
        traits = head_alternation(&flags, lemma, *pos, lines, traits);
        // The same cleanup the entry gets, before the candidates are read off these traits. A
        // redundant harmony flag that contradicts the computed class is dropped from the entry but
        // would still steer the candidate: `tükör` is measured as `tükret` against `tükört`,
        // when the form it has is `tükröt`.
        traits = drop_computed_harmony(drop_contradictions(traits), lemma, &parts);
        // Emitted from the flags, before the measurements are applied. Measuring off the committed
        // lexicon instead is a loop: a lexeme the last measurement corrected no longer carries the
        // trait, so it produces no candidate, so the next measurement has nothing to say about it
        // and the flags win again (`fiú` generated `fik` for `fiúk`).
        out.push(lemma, *pos, traits, &measured);
        traits = measured.apply(traits, lemma, *pos, &parts);
        all_ties += usize::from(!ties.is_empty());
        // PH-HARM-011: a harmony tie is a stem the norm suffixes both ways; the corpus picks the
        // form generation prefers where it can, and the other stays acceptable.
        if ties.contains(&"back") {
            traits = traits.with(Trait::HarmonyOptional);
            if let Some(h) = resolve_harmony(lemma) {
                traits = traits.with(h);
                resolved += 1;
            }
        }
        if ties.contains(&"bareT")
            && let Some(fixed) = resolve_accusative(lemma, *pos, traits, &goldens)
        {
            traits = fixed;
            resolved += 1;
        }
        by_pos.entry(*pos).or_default().push(Entry {
            lemma: lemma.as_str().into(),
            parts: parts.into_boxed_slice(),
            pos: *pos,
            traits,
            alt: None,
            last_sound: None,
        });
    }

    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../picur-morph-lexicon/data");
    fs::create_dir_all(&out_dir)?;
    let mut total = 0usize;
    for (pos, entries) in &by_pos {
        let name = format!("{}.ron", format!("{pos:?}").to_lowercase());
        let mut ron = String::from("// Generated by `cargo xtask morphdb`. Do not edit.\n[\n");
        for e in entries {
            ron.push_str("    ");
            ron.push_str(&ron_entry(e));
            ron.push_str(",\n");
        }
        ron.push_str("]\n");
        fs::write(out_dir.join(&name), ron)?;
        println!("  -> {name:<20} {:>7} entries", entries.len());
        total += entries.len();
    }
    out.write()?;
    println!("\nread {read} lines, wrote {total} entries");
    println!("{all_ties} with contradictory flags, {resolved} resolved against the rulebook");
    Ok(())
}

/// The third person possessive with and without the `ajtó` alternation, as a measurement row.
///
/// A separate axis from [`alternation_candidates`], because `NOM-STEM-013` is a possessive-only
/// alternation and the plural and accusative say nothing about it: `ajtó` gives `ajtaja` beside a
/// perfectly free `ajtók` and `ajtót`. Measuring only those two cells and concluding "no
/// alternation" is what lost `ajtaja`, `erdeje` and `teteje`.
///
/// Rules: `NOM-STEM-013`
fn possessive_candidates(lemma: &str, pos: Pos, traits: TraitSet) -> String {
    if pos != Pos::Noun || !lemma.ends_with(['ó', 'ő']) {
        return String::new();
    }
    // Only this axis differs between the two candidates. Stripping the other alternations as well
    // makes the plain candidate wrong for a genuine v-stem: `ló` is measured as `laja` against
    // `lója` when its form is `lova`.
    let plain = TraitSet::from_iter_of(
        traits
            .iter()
            .filter(|t| !matches!(t, Trait::PossessiveAlt | Trait::PossessedPluralAlt)),
    );
    let mut bound = Entry::new(lemma, pos);
    bound.traits = plain.with(Trait::PossessiveAlt);
    let mut free = Entry::new(lemma, pos);
    free.traits = plain;
    let poss = [Morph::Possessive {
        person: Person::Third,
        number: Number::Sing,
    }];
    // The possessed plural is its own cell: `szülő` has the plain singular `szülője` beside the
    // alternating `szülei`, which is why one verdict for the lexeme cannot serve both.
    let ppl = [
        Morph::PossessedPlural,
        Morph::Possessive {
            person: Person::Third,
            number: Number::Sing,
        },
    ];
    let form = |e: &Entry, st: &[Morph]| generate(e, st).unwrap_or_default();
    // The stem question is asked independently of the allomorph, because the two axes interact:
    // the alternating possessed plural is `ajtajai` with the `j` and `szülei` without it. Both
    // shapes go out and the measurement takes whichever is attested, so the answer is "does the
    // stem alternate in this cell" and not "which allomorph does it take".
    let mut alt_ppl = Entry::new(lemma, pos);
    alt_ppl.traits = plain.with(Trait::PossessedPluralAlt);
    let mut alt_ppl_j = Entry::new(lemma, pos);
    alt_ppl_j.traits = plain
        .with(Trait::PossessedPluralAlt)
        .with(Trait::PossessedPluralJa);
    let (b, f) = (form(&bound, &poss), form(&free, &poss));
    let (bp, bpj, fp) = (
        form(&alt_ppl, &ppl),
        form(&alt_ppl_j, &ppl),
        form(&free, &ppl),
    );
    if (b.is_empty() || b == f) && (bp.is_empty() || (bp == fp && bpj == fp)) {
        return String::new();
    }
    format!("{lemma}\t{pos:?}\t{b}\t{f}\t{bp}\t{bpj}\t{fp}\n")
}

/// The low and non-low plural and accusative of a noun, as a measurement row.
///
/// `NOM-LOW-001` marks lowering per lexeme and the two cells come apart: `olaj` gives `olajat` and
/// `olajok`. Only a noun already marked lowering is asked about, because the question is which of
/// its cells the marking reaches.
///
/// Rules: `NOM-LOW-001`, `NOM-PL-001`, `NOM-ACC-005`
fn lowering_candidates(lemma: &str, pos: Pos, traits: TraitSet) -> String {
    if !matches!(pos, Pos::Noun | Pos::Adjective) {
        return String::new();
    }
    // Asked of every noun and adjective, not only of the ones already marked: a measurement that
    // can narrow a marking but never discover one leaves morphdb's omissions in place. `néhány`
    // is unmarked in morphdb and would otherwise leave `néhányat` ill formed.
    let mut low = Entry::new(lemma, pos);
    low.traits = traits
        .with(Trait::Low)
        .without(Trait::LowPluralExempt)
        .without(Trait::LowAccusativeExempt)
        .without(Trait::LowPossessiveExempt);
    let mut plain = Entry::new(lemma, pos);
    plain.traits = traits
        .without(Trait::Low)
        .without(Trait::LowPluralExempt)
        .without(Trait::VStem);
    let g = |e: &Entry, st: &[Morph]| generate(e, st).unwrap_or_default();
    let pl = [Morph::Plural];
    let acc = [Morph::Case(Case::Acc)];
    // The possessive is the third cell, and it parts company with the others: `pár` takes the low
    // `párat` and the plain `párom`.
    let poss = [Morph::Possessive {
        person: Person::First,
        number: Number::Sing,
    }];
    let (lpl, ppl) = (g(&low, &pl), g(&plain, &pl));
    let (lacc, pacc) = (g(&low, &acc), g(&plain, &acc));
    let (lp, pp) = (g(&low, &poss), g(&plain, &poss));
    if lpl == ppl && lacc == pacc && lp == pp {
        return String::new();
    }
    format!("{lemma}\t{pos:?}\t{lpl}\t{ppl}\t{lacc}\t{pacc}\t{lp}\t{pp}\n")
}

/// The two u-stem candidates of a nominal in `-ú`/`-ű`, as a measurement row.
///
/// `NOM-STEM-012` drops the final vowel of a u-stem before a suffix (`ifjú` giving `ifjak` and
/// `ifjan`), and `NOM-ADJ-005` names four adjectives that do it. morphdb carries the trait for
/// nouns and not for adjectives at all, so the class cannot be read off the flags: `ifjú` and
/// `hosszú` were unmarked, which left `ifjak` ill formed and `hosszan` ungeneratable while
/// `szomorúan` has to stay as it is. Every such nominal is asked, marked or not.
///
/// Rules: `NOM-STEM-012`, `NOM-ADJ-005`
fn ustem_candidates(lemma: &str, pos: Pos, traits: TraitSet) -> String {
    if !matches!(pos, Pos::Noun | Pos::Adjective) || !lemma.ends_with(['ú', 'ű']) {
        return String::new();
    }
    let mut dropping = Entry::new(lemma, pos);
    dropping.traits = traits.with(Trait::UStem);
    let mut keeping = Entry::new(lemma, pos);
    keeping.traits = traits.without(Trait::UStem);
    let g = |e: &Entry, st: &[Morph]| generate(e, st).unwrap_or_default();
    let pl = [Morph::Plural];
    let acc = [Morph::Case(Case::Acc)];
    let (dpl, kpl) = (g(&dropping, &pl), g(&keeping, &pl));
    let (dacc, kacc) = (g(&dropping, &acc), g(&keeping, &acc));
    if dpl == kpl && dacc == kacc {
        return String::new();
    }
    format!("{lemma}\t{pos:?}\t{dpl}\t{kpl}\t{dacc}\t{kacc}\n")
}

/// The four past-marker candidates of a verb, as a measurement row.
///
/// `VRB-PAST-001` chooses between `-t` and `-Ott` by the coda, and the choice is lexical for a
/// large class and splits by cell: `függ` is `függött` word-finally but `függtem` before a person
/// suffix. The existing lists cover the geminates and the `-ad` verbs; every other verb answers
/// the same two questions, which is what settles `aludt` and `feküdt` without naming them.
///
/// Rules: `VRB-PAST-001`, `VRB-PAST-004`, `VRB-STEM-009`
fn past_candidates(lemma: &str, pos: Pos, traits: TraitSet) -> String {
    if pos != Pos::Verb {
        return String::new();
    }
    // `VRB-PAST-001` makes `-Ott` categorical after a stem-final `t`, so the bare-`t` candidate is
    // an impossible form (`tartt`) and the cell has nothing to measure. Asking anyway lets a typo
    // win where the corpus files the real form under another lemma: `figyelmeztetett` is 46,772
    // tokens of `figyelmez`, leaving 27 against 317 for `figyelmeztett` under its own lemma.
    if picur_morph_core::present_stem(lemma).ends_with('t') {
        return String::new();
    }
    let plain = TraitSet::from_iter_of(traits.iter().filter(|t| {
        !matches!(
            t,
            Trait::PastFinalT | Trait::PastFinalVtt | Trait::PastNonfinalT | Trait::PastNonfinalVtt
        )
    }));
    let cell = |person, number| {
        vec![Morph::Verbal {
            mood: Mood::Ind,
            tense: Tense::Past,
            person,
            number,
            definite: Definite::Ind,
        }]
    };
    let third = cell(Person::Third, Number::Sing);
    let first = cell(Person::First, Number::Sing);
    let with = |a, b| {
        let mut e = Entry::new(lemma, pos);
        e.traits = plain.with(a).with(b);
        e
    };
    let t = with(Trait::PastFinalT, Trait::PastNonfinalT);
    let ott = with(Trait::PastFinalVtt, Trait::PastNonfinalVtt);
    let g = |e: &Entry, st: &[Morph]| generate(e, st).unwrap_or_default();
    let (ft, fo) = (g(&t, &third), g(&ott, &third));
    let (nt, no) = (g(&t, &first), g(&ott, &first));
    if (ft.is_empty() || ft == fo) && (nt.is_empty() || nt == no) {
        return String::new();
    }
    format!("{lemma}\t{ft}\t{fo}\t{nt}\t{no}\n")
}

/// The linked and unlinked infinitive of a verb, as a measurement row.
///
/// `VRB-LINK-002`'s heaviness is computed from the coda, and the computation is right for a
/// cluster (`tartani`) and wrong for several single-consonant stems: `edzeni` and `bocsátani` are
/// the forms, not `edzni` and `bocsátni`. The corpus decides, and `véd` takes both.
///
/// Rules: `VRB-LINK-002`, `VRB-INF-001`
fn infinitive_candidates(lemma: &str, pos: Pos, traits: TraitSet) -> String {
    if pos != Pos::Verb {
        return String::new();
    }
    let plain = TraitSet::from_iter_of(traits.iter().filter(|t| *t != Trait::HeavyStem));
    let mut heavy = Entry::new(lemma, pos);
    heavy.traits = plain.with(Trait::HeavyStem);
    let mut light = Entry::new(lemma, pos);
    light.traits = plain;
    let g = |e: &Entry| generate(e, &[Morph::Infinitive]).unwrap_or_default();
    let (linked, bare) = (g(&heavy), g(&light));
    if linked.is_empty() || linked == bare {
        return String::new();
    }
    format!("{lemma}\t{linked}\t{bare}\n")
}

/// The third person possessed plural of one lemma under one trait set.
fn possessed_plural_form(lemma: &str, pos: Pos, traits: TraitSet) -> String {
    let mut e = Entry::new(lemma, pos);
    e.traits = traits;
    generate(
        &e,
        &[
            Morph::PossessedPlural,
            Morph::Possessive {
                person: Person::Third,
                number: Number::Sing,
            },
        ],
    )
    .unwrap_or_default()
}

/// The two third-person possessive allomorphs of a noun, as a measurement row.
///
/// `NOM-POSS-006` calls the choice lexical with no learnable default, and the existing list is an
/// *exceptions* list, so it can only correct a lemma someone already doubted: `utódja` is 38,421
/// against `utóda` 23,241 `[WC2]` and was marked `-a`. Asking every noun settles it, and both are
/// accepted where both are attested.
///
/// Rules: `NOM-POSS-006`
fn possessive_allomorph_candidates(lemma: &str, pos: Pos, traits: TraitSet) -> String {
    if pos != Pos::Noun {
        return String::new();
    }
    let plain = TraitSet::from_iter_of(
        traits
            .iter()
            .filter(|t| !matches!(t, Trait::PossessiveA | Trait::PossessiveJa)),
    );
    let cell = [Morph::Possessive {
        person: Person::Third,
        number: Number::Sing,
    }];
    let form = |t| {
        let mut e = Entry::new(lemma, pos);
        e.traits = plain.with(t);
        generate(&e, &cell).unwrap_or_default()
    };
    let (ja, a) = (form(Trait::PossessiveJa), form(Trait::PossessiveA));
    if ja.is_empty() || ja == a {
        return String::new();
    }
    format!("{lemma}\t{ja}\t{a}\n")
}

/// The two possessed-plural allomorphs of a noun, as a measurement row.
///
/// `NOM-PPL-001` derives the cell from the singular's `-ja`/`-a` choice, and the corpus shows the
/// two cells parting company often: `barátja` is right beside `barátai` 116,592 to `barátjai` 591.
///
/// Rules: `NOM-PPL-001`, `NOM-POSS-006`
fn possessed_plural_candidates(lemma: &str, pos: Pos, traits: TraitSet) -> String {
    if pos != Pos::Noun {
        return String::new();
    }
    // The cell's own traits decide, not the singular's: after a vowel the default is the bare `-i`
    // whatever the singular takes, which is why a vowel-final stem has a question to answer at all
    // (`ajtajai` beside the same class's `szülei`).
    let plain = TraitSet::from_iter_of(
        traits
            .iter()
            .filter(|t| !matches!(t, Trait::PossessedPluralA | Trait::PossessedPluralJa)),
    );
    let mut ja = Entry::new(lemma, pos);
    ja.traits = plain.with(Trait::PossessedPluralJa);
    let mut a = Entry::new(lemma, pos);
    a.traits = plain.with(Trait::PossessedPluralA);
    let cell = [
        Morph::PossessedPlural,
        Morph::Possessive {
            person: Person::Third,
            number: Number::Sing,
        },
    ];
    let g = |e: &Entry| generate(e, &cell).unwrap_or_default();
    let (with_j, without) = (g(&ja), g(&a));
    if with_j.is_empty() || with_j == without {
        return String::new();
    }
    format!("{lemma}\t{with_j}\t{without}\n")
}

/// The syncopated and full present forms of an epenthetic verb, as a measurement row.
///
/// The syncopated candidate is built from the already-dropped stem rather than by asking the
/// engine for it, because the engine's own gate is what the measurement is testing: it only
/// syncopates after `g`, `z` and `m`, so for an `l` verb both candidates came out the same and
/// `közöl` was skipped as carrying no evidence. The corpus has `közli` 120,588 : `közöli` 757.
///
/// Rules: `VRB-STEM-007`
fn present_syncope_candidates(lemma: &str, pos: Pos, traits: TraitSet) -> String {
    if pos != Pos::Verb || !traits.has(Trait::Epenthetic) {
        return String::new();
    }
    let Some(dropped) = picur_morph_core::syncopated(lemma) else {
        return String::new();
    };
    // The harmony class travels with the lexeme, and the dropped vowel may have been the one that
    // fixed it, so it is stated on the synthetic entry rather than recomputed from the short stem.
    let class = match picur_morph_ortho::harmony_of(lemma) {
        picur_morph_ortho::Harmony::Back => Trait::Back,
        picur_morph_ortho::Harmony::FrontRounded => Trait::FrontRounded,
        picur_morph_ortho::Harmony::Front => Trait::Front,
    };
    let plain = TraitSet::from_iter_of(traits.iter().filter(|t| *t != Trait::Epenthetic));
    let mut sync = Entry::new(dropped.as_str(), pos);
    sync.traits = plain.with(class);
    let mut full = Entry::new(lemma, pos);
    full.traits = plain.with(class);
    let cell = |definite, person| {
        vec![Morph::Verbal {
            mood: Mood::Ind,
            tense: Tense::Pres,
            person,
            number: Number::Sing,
            definite,
        }]
    };
    let def = cell(Definite::Def, Person::Third);
    let first = cell(Definite::Ind, Person::First);
    // The past is the third cell, and it disagrees with the other two: `jegyez` syncopates here
    // and keeps its vowel in the infinitive.
    let past = vec![Morph::Verbal {
        mood: Mood::Ind,
        tense: Tense::Past,
        person: Person::Third,
        number: Number::Sing,
        definite: Definite::Ind,
    }];
    let g = |e: &Entry, st: &[Morph]| generate(e, st).unwrap_or_default();
    format!(
        "{lemma}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        g(&sync, &def),
        g(&full, &def),
        g(&sync, &first),
        g(&full, &first),
        g(&sync, &past),
        g(&full, &past),
    )
}

/// The bound and free plural and accusative of a marked stem, as a measurement row.
///
/// Empty for anything with no marked alternation: there is no pair to ask the corpus about.
fn alternation_candidates(lemma: &str, pos: Pos, traits: TraitSet) -> String {
    if !matches!(pos, Pos::Noun | Pos::Adjective) || !traits.has_any(ALTERNATIONS) {
        return String::new();
    }
    // `FreeStemAccSup` comes off both sides: with it the bound accusative is already the free one
    // and the two columns would be the same string.
    let keep = |t: &Trait| *t != Trait::FreeStemAccSup && *t != Trait::AlternationOptional;
    let mut bound = Entry::new(lemma, pos);
    bound.traits = TraitSet::from_iter_of(traits.iter().filter(keep));
    let mut free = Entry::new(lemma, pos);
    free.traits = TraitSet::from_iter_of(
        traits
            .iter()
            .filter(|t| keep(t) && !ALTERNATIONS.contains(t)),
    );
    let form = |e: &Entry, stack: &[Morph]| generate(e, stack).unwrap_or_default();
    // The third person possessive is the third cell, and the one that shows a homograph: `szél`
    // is two lexemes under one spelling and only this cell separates them, `széle` 37,411 for the
    // edge beside `szele` 16,866 for the wind `[WC2]`.
    let poss = [Morph::Possessive {
        person: picur_morph_core::Person::Third,
        number: picur_morph_core::Number::Sing,
    }];
    format!(
        "{lemma}\t{pos:?}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        form(&bound, &[Morph::Plural]),
        form(&free, &[Morph::Plural]),
        form(&bound, &[Morph::Case(Case::Acc)]),
        form(&free, &[Morph::Case(Case::Acc)]),
        form(&bound, &poss),
        form(&free, &poss),
    )
}

struct Candidates {
    alternation: String,
    possessive: String,
    syncope: String,
    possessed_plural: String,
    infinitive: String,
    lowering: String,
    past: String,
    possessive_allomorph: String,
    ustem: String,
}

impl Default for Candidates {
    fn default() -> Self {
        Self {
            alternation:
                "lemma\tpos\tbound_pl\tfree_pl\tbound_acc\tfree_acc\tbound_poss\tfree_poss\n"
                    .to_owned(),
            possessive: "lemma\tpos\talt\tplain\talt_ppl\talt_ppl_j\tplain_ppl\n".to_owned(),
            syncope: "lemma\tsync_def\tfull_def\tsync_1sg\tfull_1sg\tsync_past\tfull_past\n"
                .to_owned(),
            possessed_plural: "lemma\tja\ta\n".to_owned(),
            infinitive: "lemma\tlinked\tbare\n".to_owned(),
            lowering: "lemma\tpos\tlow_pl\tplain_pl\tlow_acc\tplain_acc\tlow_poss\tplain_poss\n"
                .to_owned(),
            past: "lemma\tfinal_t\tfinal_ott\tnonfinal_t\tnonfinal_ott\n".to_owned(),
            possessive_allomorph: "lemma\tja\ta\n".to_owned(),
            ustem: "lemma\tpos\tdrop_pl\tkeep_pl\tdrop_acc\tkeep_acc\n".to_owned(),
        }
    }
}

impl Candidates {
    /// Emits every axis's row for one lemma.
    ///
    /// Each axis is built from the traits the earlier axes have already settled, or the question
    /// asked of the corpus is the wrong one.
    fn push(&mut self, lemma: &str, pos: Pos, traits: TraitSet, measured: &Measured) {
        let settled = measured.settled(traits, lemma, pos);
        self.alternation
            .push_str(&alternation_candidates(lemma, pos, traits));
        self.syncope
            .push_str(&present_syncope_candidates(lemma, pos, traits));
        self.infinitive
            .push_str(&infinitive_candidates(lemma, pos, traits));
        self.lowering
            .push_str(&lowering_candidates(lemma, pos, traits));
        self.possessive.push_str(&possessive_candidates(
            lemma,
            pos,
            measured.alternation_only(traits, lemma, pos),
        ));
        self.possessed_plural
            .push_str(&possessed_plural_candidates(lemma, pos, settled));
        self.possessive_allomorph
            .push_str(&possessive_allomorph_candidates(lemma, pos, settled));
        self.past.push_str(&past_candidates(lemma, pos, settled));
        self.ustem.push_str(&ustem_candidates(lemma, pos, traits));
    }

    /// Writes the measurement inputs, when the gitignored list directory is present.
    #[expect(clippy::print_stdout, reason = "CLI report")]
    fn write(&self) -> Result {
        let lists = Path::new("local/lists");
        if !lists.is_dir() {
            return Ok(());
        }
        for (name, text) in [
            ("alternation-candidates.tsv", &self.alternation),
            ("possessive-alt-candidates.tsv", &self.possessive),
            ("present-syncope-candidates.tsv", &self.syncope),
            ("possessed-plural-candidates.tsv", &self.possessed_plural),
            ("infinitive-candidates.tsv", &self.infinitive),
            ("lowering-candidates.tsv", &self.lowering),
            ("past-candidates.tsv", &self.past),
            ("ustem-candidates.tsv", &self.ustem),
            (
                "possessive-allomorph-candidates.tsv",
                &self.possessive_allomorph,
            ),
        ] {
            fs::write(lists.join(name), text)?;
            println!("  -> local/lists/{name}");
        }
        Ok(())
    }
}

/// One entry as a RON row.
fn ron_entry(e: &Entry) -> String {
    use std::fmt::Write as _;
    let mut s = format!("(lemma:{:?}", e.lemma);
    if !e.parts.is_empty() {
        let _ = write!(s, ",parts:{:?}", e.parts);
    }
    let _ = write!(s, ",pos:{:?}", e.pos);
    let traits: Vec<String> = e.traits.iter().map(|t| format!("{t:?}")).collect();
    if !traits.is_empty() {
        let _ = write!(s, ",traits:[{}]", traits.join(","));
    }
    s.push(')');
    s
}

#[cfg(test)]
mod tests {
    use picur_morph_lexicon::{Pos, Trait};

    use super::{Line, bare, parse, seams, votes};

    fn line(flags: &[&str]) -> Line {
        Line {
            flags: flags.iter().map(|s| (*s).to_owned()).collect(),
            parts: Vec::new(),
            fkp_only: false,
        }
    }

    #[test]
    fn strips_the_line_terminator() {
        // `stressz` is a whole line `stressz<TAB>NOUN,;`, which must not be dropped.
        assert_eq!(bare("NOUN,;"), "NOUN");
        assert_eq!(bare("NOUN,"), "NOUN");
        assert_eq!(bare("NOUN"), "NOUN");
    }

    #[test]
    fn a_space_separated_row_parses_too() {
        // Several files separate the lemma with a tab on most rows and a space on the rest, and a
        // tab-only split loses those rows outright: 554 of `missing.lexicon`, all 262 of
        // `conj.lexicon`, 86 acronyms and 14 proper nouns.
        let (key, _) = parse("látszik verb_indef SRC: fkp el", None, false).expect("parses");
        assert_eq!(key, (Pos::Verb, "látszik".to_owned()));
        let (key, _) = parse("http    ACRO,;", Some(Pos::Noun), false).expect("parses");
        assert_eq!(key, (Pos::Noun, "http".to_owned()));
        assert!(parse("#################", Some(Pos::Noun), false).is_none());
    }

    #[test]
    fn parses_a_flagless_entry() {
        let (key, l) = parse("stressz\tNOUN,;", None, false).expect("parses");
        assert_eq!(key, (Pos::Noun, "stressz".to_owned()));
        assert_eq!(l.flags, [] as [String; 0]);
    }

    #[test]
    fn parses_flags_and_stops_at_a_section() {
        let (key, l) = parse(
            "orr\tNOUN bareT non_bareT poss_A SRC: el mi ,;",
            None,
            false,
        )
        .expect("ok");
        assert_eq!(key.0, Pos::Noun);
        assert_eq!(l.flags, vec!["bareT", "non_bareT", "poss_A"]);
    }

    #[test]
    fn parses_compound_parts() {
        let (_, l) = parse("óriáscég\tNOUN STR: óriás@cég, óriás@cég,;", None, false).expect("ok");
        assert_eq!(l.parts, vec!["óriás", "cég"]);
    }

    #[test]
    fn parses_a_forced_pos_file() {
        let (key, _) = parse("meg\tPREV,;", Some(Pos::Preverb), false).expect("ok");
        assert_eq!(key, (Pos::Preverb, "meg".to_owned()));
    }

    #[test]
    fn parses_the_two_tab_adverb_format() {
        let (key, _) = parse("abszolúte\tA_ADV\tADV,;", Some(Pos::Adverb), false).expect("ok");
        assert_eq!(key, (Pos::Adverb, "abszolúte".to_owned()));
    }

    #[test]
    fn contradictory_flags_are_reported_as_a_tie() {
        // `orr` carries both bareT and non_bareT. The tie is reported either way.
        let (set, ties) = votes(&[line(&["bareT", "non_bareT"])], Pos::Noun);
        assert_eq!(ties, vec!["bareT"]);
        // A two-way choice abstains, leaving the computed default and the golden resolution to
        // settle it; picking the `for` side yields `adventban` and `szépak`.
        assert!(!set.has(Trait::BareAccusative));
        assert!(!set.has(Trait::LinkedAccusative));
    }

    #[test]
    fn a_tie_over_presence_keeps_the_marked_alternation() {
        // Where the flags are presence against absence rather than a choice, the marked member
        // still wins: abstaining there would drop genuine optional alternations such as `mű`.
        let (set, ties) = votes(&[line(&["epenthetic", "non_epenthetic"])], Pos::Noun);
        assert_eq!(ties, vec!["epenthetic"]);
        assert!(set.has(Trait::Epenthetic));
    }

    #[test]
    fn a_row_terminator_is_not_a_source() {
        // `SRC: fkp ,;` has one source, not two. Counting the terminator defeats `fkp_only`,
        // which would let the adverb-suffix row of `szép` set Back.
        let (_, l) = parse(
            "szép\tADJ aff_ul back front non_round SRC: fkp ,;",
            None,
            false,
        )
        .expect("ok");
        assert!(l.fkp_only);
    }

    #[test]
    fn seam_offsets_only_when_parts_reconstruct_the_lemma() {
        assert_eq!(
            seams("óriáscég", &["óriás".to_owned(), "cég".to_owned()]),
            vec![7]
        );
        // A derivational split that does not concatenate is not a compound seam.
        assert_eq!(
            seams("kutya", &["kutya".to_owned(), "x".to_owned()]),
            [] as [u32; 0]
        );
    }

    #[test]
    fn fkp_only_adjective_lines_are_ignored() {
        let mut l = line(&["back"]);
        l.fkp_only = true;
        let (set, _) = votes(&[l], Pos::Adjective);
        assert!(
            !set.has(Trait::Back),
            "fkp-only ADJ flags produced szép=back"
        );
    }
}
