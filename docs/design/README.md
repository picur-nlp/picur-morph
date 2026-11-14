# Implementation design

How the code is shaped. The rulebook in [`docs/grammar/`](../grammar/) is the specification and stays
authoritative; these files say how it is implemented and where the line between logic and data runs.

| file | content |
|---|---|
| [`01-core-model.md`](01-core-model.md) | the types: `Letter`, `Harmony`, `Suffix`, `Entry`, `Morph`, `Analysis` |
| [`02-generation.md`](02-generation.md) | the suffixation engine, the single source of truth |
| [`03-analysis.md`](03-analysis.md) | analysis as search over generation, the checker's verdict, the guesser |
| [`04-lint-repair.md`](04-lint-repair.md) | text repair, typography diagnostics, document quality |
| [`05-lexicon.md`](05-lexicon.md) | entry schema, the traits and how each gets its value |
| [`06-reproducibility.md`](06-reproducibility.md) | what is committed and how each artefact regenerates |
| [`07-dependencies.md`](07-dependencies.md) | dependency policy, the dependency graph, CI |

## The one architectural commitment

**Generation is the single source of truth. Analysis is search plus verification.**

A candidate analysis is accepted only when regenerating it reproduces the input surface exactly.
Nothing about morphology is encoded twice, so analysis cannot drift from generation, and a fix to a
phonological rule propagates to both directions at once. The rulebook's `Output` fields describe
generation, and there is no separate parsing grammar.

The cost is search breadth, bounded by a stem index over stable prefixes (the part of a lemma no
alternation changes) and by an index of the search space keyed on the ending each feature stack can
write.

## Crates

```
picur-morph            facade: re-exports every crate below and the main entry points; the `picur-morph` binary (`cli` feature, default)
  picur-morph-spell    three-way verdict per word, readings, guesses, suggestions, text checking
  picur-morph-repair   damage repair: encoding, OCR, layout, keyboard, archaic spelling
  picur-morph-lint     punctuation and typography diagnostics
  picur-morph-rank     document quality score over the checker and the damage detectors
  picur-morph-core     generation, analysis, derivation, compounds, numerals, pronouns, UD projection
  picur-morph-lexicon  entries, traits, frequency; data compiled in
  picur-morph-ortho    letters, harmony, syllables, phonology, case, tokens, encodings

xtask                  conversion, goldens, coverage, ledger and evaluation; not published
```

`ortho` depends on nothing in the workspace and is pure functions over text. `lexicon` depends on
`ortho`; `core` on both; `spell` on `core`; `repair` and `rank` on `spell`; `lint` on `core`. No
cycles, and no crate depends on `rank`.

```
text
  -> tokens            picur-morph-ortho::tokens
  -> check / analyses  picur-morph-spell          per token: Ok, Unknown or Wrong, and the readings
  -> repair            picur-morph-repair         fixes licensed by the checker
  -> lint              picur-morph-lint           typography, from the character stream
  -> quality           picur-morph-rank           one score per document

lemma + Vec<Morph>
  -> generate          picur-morph-core
  -> surface
```

## Where the line runs

Measured over the 1,257 rules by their `Lexical input` field, 839 consume lexical input of some
kind. The implementation keeps that input to a minimum, in this order of preference:

1. **Derive it.** If a property follows from the written or spoken shape of the word, compute it.
   Harmony class, linking-vowel choice, `-vAl` assimilation, `-t` after a sibilant, hyphenation
   points, suffixation of a foreign name by its final letter, the hyphen of a name before a
   same-letter suffix: all computed, never stored.
2. **Otherwise make it a lexeme trait.** Irreducible idiosyncrasy attaches to the entry, not to the
   rule. `bokor` is `Epenthetic` because nothing predicts it. The 56 traits are listed in
   [`05-lexicon.md`](05-lexicon.md), each with the rule that needs it.
3. **An explicit inventory only when the language itself is a closed list.** The alphabet, the
   pronoun paradigm, the case-suffix inventory, the preverbs, the bound first members of
   `DER-PREF-005`. These are definitions with a stated source, not overrides.
4. **Never a per-word override on a rule.** If a rule needs a list of exceptions to work, the rule
   is wrong; find the conditioning factor. Where a rule names a lexical exception class, the
   lexicon marks the members and the rule states the default.

### Unsourced lists

117 rules name a list in `Lexical input` without saying where it comes from, against a budget of
154; `cargo xtask lint-rules` fails if the count rises. Each such reference is resolved, before the
code that would consume it is written, to exactly one of: derived (delete the list, state the
derivation), trait (fold into the lexicon schema, say how the trait is induced), or closed
inventory (keep, with provenance and a fixed member count).

## Traceability

Every implemented rule is reachable from its ID. Doc comments carry the ID:

```rust
/// Accusative `-t` attaches bare after a sibilant, with a linking vowel otherwise.
/// Rules: NOM-ACC-004, NOM-ACC-009
```

`cargo xtask coverage` reports which rule IDs appear in the code, split by `Predictability`.

## Status

| measure | value | how |
|---|---|---|
| goldens reproduced, scored set | 2,151 of 2,166 (99.3%) | `cargo xtask eval` |
| goldens reproduced, every row | 2,771 of 3,148 (88.0%) | same |
| starred forms rejected | 196 of 242 (81.0%) | same |
| example forms accepted | 13,258 of 13,494 (98.3%) | same |
| example texts the lint leaves alone | 145 of 145 | same |
| rules covered by behaviour / partial / set aside / unmeasured | 846 / 132 / 32 / 247 of 1,257 | `cargo xtask ledger` |
| rule IDs cited in code (traceability, not coverage) | 330 of 1,257 | `cargo xtask coverage` |
| UD Szeged, tokens analysed | 97.1% | `cargo xtask gold <conllu dir>` |
| UD Szeged, lemma agrees | 96.5% of the analysed | same |
| UD Szeged, lemma, tag and features all agree | 70.3% | same |
| readings per analysed token | 1.76 | same |
| generation analyses back | 100% of 170k forms over the 50 commonest lemmas of each open class, less pure homographs | `cargo xtask roundtrip 50` |
| checker throughput | about 4,300 words/s on running text, 5,155 stacks | [`crates/picur-morph-spell/tests/throughput.rs`](../../crates/picur-morph-spell/tests/throughput.rs) |
| tests | 693 | `cargo test` |
| paradigm forms both hunspell and hunmorph-foma reject | 9% of 513k forms over 332 random lemmas | `picur-morph paradigm`, see [06](06-reproducibility.md) |

Goldens by chapter:

| chapter | reproduced |
|---|---|
| 01 writing system | 36 of 36 |
| 02 phonology | 285 of 285 |
| 03 nominal | 446 of 446 |
| 04 pronouns, numerals, determiners | 3 of 3 |
| 05 verbal | 492 of 492 |
| 06 derivation and compounding | 374 of 374 |
| 07 orthography | 203 of 203 |
| 08 punctuation and typography | 16 of 16 |
| 09 syntax | no goldens; its example sentences pass the lint |
| 10 text repair | 296 of 311 |

Rules by chapter in the ledger (covered / partial / set aside / unmeasured): 01 18/1/0/57, 02
97/12/0/3, 03 107/9/0/3, 04 74/22/0/23, 05 105/6/0/11, 06 102/16/0/29, 07 207/54/0/48, 08
80/3/2/48, 09 4/0/0/0, 10 52/9/30/25. An example form the rulebook marks as unattested
(`` `pénzet` `-` ``) is not extracted, because the rule cites it as what the language does not do. A partial rule is one with a starred form the checker
accepts or an example form it does not; the remaining starred acceptances are words under another
reading or semantic classes no rule computes, listed per chapter in the agents' and the eval's
reports. An unmeasured rule has no form, starred form or text to measure, which is a gap in the
rulebook's examples rather than in the engine.

`cargo xtask eval` scores a golden by every mechanism the engine has (generation over every stack,
analysis for a lemma, repair, lint, the writing-system transforms) and sets four groups aside with
their counts: suffixes `DER-MODEL-012` refuses to coin, rules whose output is declared lexical,
phrases written solid, and the grammar choices the repair leaves alone; the rate over every row
is printed beside the scored one. The rulebook's starred forms ([`docs/grammar/rejects.ron`](../grammar/rejects.ron)) are
scored the other way round: each must come back from the checker as not accepted, and the ones
accepted are listed. About a quarter of those are words under another reading (`lest` is the
accusative of `les`, `meglepőn` a superessive) and the rest are forbidden by a semantic class the
rule names and no rule computes (material nouns take no `-i`, stative verbs no `-gAt`). Every doublet the rulebook
states is generated in its preferred form and accepted in the other (`ugrani`, `ugorni`). What
remains unreproduced is a repair that would need context (`Mannal` is also `Man` with `-nal`), a
foreign spelling, or a name the lexicon does not describe. The analyser does not disambiguate between readings and does not try to: a token's
readings are all returned, and a consumer that needs one picks by context. That includes the
closed paradigms: `értem` is returned as the pronoun's causal, the present of `ért` and the past
of `ér` together, where an earlier version answered with the first paradigm that matched.

## Non-goals

No FFI, no runtime loading of foreign formats, no network at runtime or in tests. Data is compiled
in. Output is deterministic. Nothing derived from a non-permissive oracle is committed. Hungarian is
the only language, and there is no abstraction layer for a second one.
