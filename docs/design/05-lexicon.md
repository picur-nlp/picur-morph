# Lexicon

Holds what cannot be computed, and nothing else. Every entry should be defensible as "no rule
predicts this".

## Schema

```rust
pub struct Entry {
    pub lemma: Box<str>,
    pub parts: Box<[u32]>,            // compound seam offsets; empty for a simplex
    pub pos: Pos,
    pub traits: TraitSet,             // bitset over the 56 traits
    pub alt: Option<Box<str>>,        // pronunciation of a name spelled foreign
    pub last_sound: Option<Box<str>>, // or only its last sound
}
```

Committed as RON per part of speech under [`crates/picur-morph-lexicon/data/`](../../crates/picur-morph-lexicon/data/), embedded with
`include_str!` and parsed lazily on first use.

| file | entries | from |
|---|---|---|
| `noun.ron` | 85,328 | morphdb |
| `verb.ron` | 19,719 | morphdb |
| `adjective.ron` | 16,488 | morphdb |
| `adverb.ron` | 1,833 | morphdb |
| `interjection.ron` | 603 | morphdb |
| `postposition.ron` | 287 | morphdb, less the personal forms the engine's paradigms compute |
| `conjunction.ron` | 260 | morphdb |
| `preverb.ron` | 132 | morphdb |
| `supplement.ron` | 510 | hand-written from the rulebook |
| `inventory.ron` | 57 lists | hand-written from the rulebook: the closed word classes and paradigm tables the rules name |
| `frequency.ron` | 250,000 forms | corpus counts, half-octave buckets |

Logic lives in the engine and vocabulary in the data: a rule that says "these words" is a list in
`inventory.ron` under a `Role` (`KinshipTruncating`, `SeriesPrefix`, `IrregularComparative`,
`PostpositionPersonal`, `IrregularVerb`, the numeral words), read through `words`, `pairs`,
`entries` or `rows`, and no Rust source names a Hungarian word outside its tests. The repair crate
reads its substitution tables the same way from its own [`crates/picur-morph-repair/data/glyphs.ron`](../../crates/picur-morph-repair/data/glyphs.ron) through the shared
`Tables` parser.

A supplement row **replaces** the converted row with the same lemma and part of speech. It holds
what the rulebook states and the conversion cannot carry: the sz-d(-v) verbs with their `d` stem,
the irregular and suppletive verbs, the pronoun stems, closed-class adverbs and postpositions, the
towns that take the `-Vt` locative, the names whose pronunciation `ORT-FOR-012` needs, and the
lexical marks a rule names (`április` back, `újra` and `tovább` stacking).

Lookup is a sorted `Vec` with `partition_point` for the range of a lemma: compact, ordered for the
stem index, no allocation at startup beyond the parse. Three passes run at load: a compound with no
traits of its own inherits its head's (`DER-COMP-008`), a verb family that attests both past
classes marks them optional, and the stems `NOM-STEM-004` names get `SilentH`.

## Traits

A trait is admissible only if nothing about the written or spoken form predicts it. The test
before adding one: can `harmony_of`, `coda` or an existing alternation already tell me this?

| group | traits | why irreducible |
|---|---|---|
| harmony | `Back`, `Front`, `FrontRounded`, `HarmonyOptional` | only stems that contradict their spelling (`híd`, `április`), and the mixed stems the norm suffixes both ways (`norvég`) |
| lowering | `Low`, `LowPluralExempt`, `LowPossessiveExempt`, `LowAccusativeExempt`, `LinkedPlural` | `ház` -> `házak` against `gáz` -> `gázok`; the exemptions are measured per cell |
| accusative | `BareAccusative`, `LinkedAccusative` | override the computed coda default in either direction (`orrot`, `stresszt`) |
| possessive | `PossessiveA`, `PossessiveJa`, `PossessiveAlt`, `PossessedPluralA`, `PossessedPluralJa`, `PossessedPluralAlt`, `PossessiveOptional` | `-ja` is not the consonant-final default, so both directions are marked; the alternant stems of `NOM-STEM-013` |
| nominal stem | `Epenthetic`, `Shortening`, `VStem`, `OaStem`, `UStem`, `FreeStemAccSup`, `FreeStemPlural`, `FreeStemPossessive`, `AlternationOptional` | `bokor` ~ `bokr-`, `ló` ~ `lov-`; which cells alternate is measured per lexeme; `PH-ZERO-002`'s third unstable cell (`sarka`, `sarokja`) is read by the analyser and not generated |
| verb stem | `SzDStem`, `SzDVStem`, `DefiniteSyncope`, `DefiniteFullStem`, `PastSyncope`, `PastFullStem`, `PresentSyncope`, `PresentFullStem`, `HeavyStem`, `LinkOptional` | `alszik` ~ `aludt`; `VRB-STEM-007`'s syncope is lexical per cell; `függenek` ~ `függnek` is measured both ways |
| past marker | `PastFinalT`, `PastFinalVtt`, `PastNonfinalT`, `PastNonfinalVtt` | `maradt` against `fogadott`, measured on two axes |
| syntax and class | `Transitive`, `SingularOnly`, `NoPossessive`, `Quantity`, `Preverb`, `Stacking`, `Abbreviation`, `Article`, `DemonstrativeZ`, `Occupation`, `AttributiveOnly` | selectional or categorial, not phonological; `SingularOnly` and `NoPossessive` are enforced on pronouns only, because morphdb's `sing` marks tendencies (`anya` carries it, `anyák` is a word) |
| names | `Spoken`, `Locative`, `ArchaicSpelling` | a pronunciation the spelling does not show; the ten towns of `NOM-CASE-016`; the pre-1832 letter groups of `PH-G2P-004`, whose value is lexical because the same letters occur in modern words (`Madách` [madács] beside `Zürich`) |
| coda | `SilentH` | `PH-CONS-005`'s `cseh` type: the coda `h` is not pronounced, so `-vAl` keeps its `v` (`csehvel`, `dühvel`), and the geminate spelling the norm also allows is read through an analysis variant |

Defaults are chosen so that the unmarked case is the productive one, which is what makes the
guesser correct by construction: `NOM-LOW-001` states that every new word, loan and proper name is
non-lowering and takes `-ot`/`-et`/`-öt`.

## How a trait gets its value

Never by hand. In order of authority:

1. **Corpus attestation.** Tag- and lemma-filtered counts over a local frequency list, with a
   20-occurrence floor and a tenfold margin where a decision is kept. This is what settles `orrot`
   against `orrt`, and the filter is the whole method: `állt` is the verb, not an accusative of
   `áll`.
2. **Cross-resource voting.** The morphdb conversion, with flags voting for and against each trait;
   a contradictory pair is dropped and handed back to the rules, and 16 ties are resolved against
   the rulebook's own examples.
3. **Error-driven induction.** Generate, compare against the corpus, and let the residue propose a
   trait.

The measured lists live under the gitignored `local/lists/` and the converter reads them. They are
evidence, not runtime data: they derive and validate the traits and are never loaded by the
library. [`06-reproducibility.md`](06-reproducibility.md) names each list and the rule it serves.

## Unsourced list references

117 rules name a list in `Lexical input` without saying where it comes from, against a budget of
154 that `cargo xtask lint-rules` enforces:

| chapter | count | | chapter | count |
|---|---|---|---|---|
| 01 writing-system | 3 | | 06 derivation | 14 |
| 02 phonology | 3 | | 07 orthography | 18 |
| 03 nominal | 3 | | 08 punctuation | 19 |
| 04 pronouns/numerals | 16 | | 10 text-repair | 17 |
| 05 verbal | 24 | | | |

Each is resolved, before the code that would consume it is written, to exactly one of: a
mis-labelled closed inventory (state the member count and the source), a lexeme trait (fold into
the schema, state the induction), a genuinely open list (a named source and an induction method,
never a hand-edited file), or something derivable (delete the list, match the condition).

## Provenance and licence

morphdb.hu (CC-BY 2.5) is the only oracle that feeds committed lexicon data, converted once,
offline, with attribution in `NOTICE`. Every other resource (hunmorph-foma, magyarispell, kaikki,
UniMorph, UD Szeged, emMorph, the Kéziszótár) is read as a local oracle and never copied. The
frequency table holds counts, which are facts about a corpus rather than adapted material, and its
sources are listed in `NOTICE`.
