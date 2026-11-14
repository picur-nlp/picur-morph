# Reproducibility

Every committed data file regenerates by a committed command from inputs that live outside
the repository, under the gitignored `local/`; [WORKSPACE.md](../../WORKSPACE.md) lists each input
and where to obtain it. The commands
refuse to run without their inputs rather than produce a different file. Nothing derived from a
non-permissive oracle is committed.

The committed data is what these inputs produced at the time of the commit. No network at library
runtime or in `cargo test`.

## The regeneration graph

| committed artefact | from | command |
|---|---|---|
| `lexicon/data/{noun,adjective,verb,adverb,interjection,postposition,conjunction,preverb}.ron` | morphdb.hu lexicon files, `local/lists/*.tsv` | `cargo xtask morphdb <morphdb-in-dir>` |
| `local/lists/*.tsv` (not committed) | Webcorpus 2, oracles, the converter's candidates | [`tools/`](../../tools/README.md) |
| `lexicon/data/supplement.ron` | the rulebook | hand-written, each row citing its rule |
| `lexicon/data/inventory.ron`, `repair/data/glyphs.ron` | the rulebook | hand-written, each list citing its rule |
| `lexicon/data/frequency.ron`, `local/cache/counts.json` | the pinned corpus shards under `local/corpora/` | `cargo xtask frequency [<corpus-dir>]` |
| [`docs/grammar/goldens.ron`](../grammar/goldens.ron), `goldens-review.ron`, `rejects.ron`, `accepts.ron` | the rulebook's `Output` and `Examples` fields | `cargo xtask goldens` |
| rule coverage report | code and rulebook | `cargo xtask coverage` |
| rule ledger, one status per rule | goldens, starred and example forms | `cargo xtask ledger [status]` |
| rule provenance check | rulebook | `cargo xtask lint-rules` |
| golden reproduction | goldens and the engine | `cargo xtask eval [chapter]` |
| generation against analysis | the lexicon and the checker's search space | `cargo xtask roundtrip [<n>] [<class>]` |
| UD agreement | a directory of CoNLL-U files (UD Szeged, NerKor) | `cargo xtask gold <dir>` |
| acceptance over a word list | a `form<TAB>count` list | `cargo xtask accept <list>` |


## The morphdb conversion

Reads the morphdb files that carry inflectional information: `peeta.lexicon` and `missing.lexicon`
for nouns, adjectives and verbs, plus the closed-class files for adverbs, conjunctions, preverbs,
postpositions, interjections, acronyms and proper nouns. `ner.lexicon` is skipped: 244k named
entities would dominate the crate without teaching the engine anything. Pronouns, numerals and
determiners are not read either; they are closed inventories stated in code with their source.

`postp+case.lexicon` also lists the personal forms of the case stems and postpositions as lemmas
(`nekem`, `értem`, `hozzátok`, `őket`); a row that is a cell of a paradigm the engine computes, or
a pronoun, is skipped, because a listed copy shadows the paradigm with a wrong lemma.

The accusative of a cluster stem that the corpus attests both ways without a tenfold margin
(`NOM-ACC-009`: `csimpánzt` 485 against `csimpánzot` 81, `reneszánszt` 588 against 617) is
written bare with `AlternationOptional`, so the analyser accepts both; the undecided rows of the
raw list supply them, the verified list the decided ones.

A row is `lemma<TAB>POS flag... SRC: ... STR: a@b;`, encoded ISO-8859-2. Rows are grouped by lemma
and part of speech and every row votes; a flag in the "for" column argues for the trait, one in
"against" for its opposite.

| for | against | trait on win | trait on loss |
|---|---|---|---|
| `back` | `front` | `Back` | `Front` |
| `low` | `non_low` | `Low` | - |
| `bareT` | `non_bareT` | `BareAccusative` | `LinkedAccusative` |
| `poss_A`, `poss_sg_A`, `poss_optional` | `poss_jA`, `poss_sg_jA`, `poss_optional` | `PossessiveA` | `PossessiveJa` |
| `epenthetic` | `non_epenthetic` | `Epenthetic` | - |
| `shortening` | `non_shortening` | `Shortening` | - |
| `v_stem` | `non_v_stem` | `VStem` | - |
| `oa_stem` | `non_oa_stem` | `OaStem` | - |
| `u_stem` | `non_u_stem` | `UStem` | - |
| `sz-d_stem` | `non_sz-d_stem` | `SzDStem` | - |
| `sz-d-v_stem` | `non_sz-d-v_stem` | `SzDVStem` | - |
| `sing`, `non_poss`, `trans`, `preverb` | - | `SingularOnly`, `NoPossessive`, `Transitive`, `Preverb` | - |
| `t_final_past`, `Vtt_final_past`, `t_nonfinal_past`, `Vtt_nonfinal_past` | - | the four `Past*` traits | - |

Rules the converter applies on top of the vote:

- An adjective row sourced from `fkp` alone describes its adverb suffix, so only its `aff_*` flags
  are read; its `back`/`low`/`bareT` would give `régiak`.
- A same-line `X` / `non_X` pair for a marked alternation votes **for** the alternation, which is
  how an optional alternation such as `mű` ~ `művek` is kept.
- A contradictory pair that survives the vote is dropped and the cell handed back to the rules;
  16 such ties are resolved against the rulebook's own examples instead. A harmony tie is kept as
  `HarmonyOptional`, the stems the norm suffixes both ways, with the corpus's commoner form
  generated where the frequency table shows one.
- A computed harmony trait is dropped where `harmony_of` already gives it, so only antiharmonic
  stems carry one.
- A lemma that is not spelled as a word (punctuation, codes, HTML entities, soft hyphens,
  placeholders) is not an entry.
- `STR:` seams become `parts`; `|` derivation marks are dropped.

Known limits of the source, recorded so they are not rediscovered as engine bugs: two readings
of one lemma with the same part of speech merge, so the numeral `pár` 'a few' lends its lowering
to the noun 'pair' (`párak` is accepted beside `párok`); and stray whole-word rows such as `házok`
are entries because the source lists them.

## Measured lexical values

The properties the rulebook calls lexical are measured, and the converter reads the measurements
from `local/lists/`; it refuses to run if any list is missing, naming the file, rather than write a
lexicon with the traits unmarked:

| property | rule | list |
|---|---|---|
| third-person possessive `-ja/-je` against `-a/-e` | `NOM-POSS-006` | `poss3-ja-exceptions.tsv`, `possessive-allomorph.tsv` |
| accusative, bare against linked | `NOM-ACC-005`, `NOM-ACC-009` | `bare-accusative-verified.tsv`, `bare-accusative.tsv` |
| antiharmonic stems | `PH-HARM-011` | `antiharmonic.tsv` |
| infinitive linking vowel | `VRB-LINK-002`, `VRB-INF-001` | `infinitive-link.tsv` |
| geminate stem heaviness | `VRB-LINK-002` | `geminate-verbs.tsv` |
| past marker of the geminates | `VRB-PAST-004` | `geminate-past.tsv` |
| past marker of the `-ad`/`-ed` verbs | `VRB-PAST-008` | `ad-past.tsv` |
| past marker of the cluster verbs | `VRB-PAST-001` | `cc-past-verbs.tsv`, `past-marker.tsv` |
| stem alternation, per cell | `NOM-STEM-008`, `-010`, `-012` | `stem-alternation.tsv`, `ustem.tsv` |
| epenthetic syncope, per cell | `VRB-STEM-007` | `present-syncope.tsv` |
| lowering, per cell | `NOM-LOW-001` | `lowering-cells.tsv` |
| possessed plural allomorph | `NOM-PPL-001` | `possessed-plural.tsv` |
| possessive alternant stems | `NOM-STEM-013` | `possessive-alt.tsv` |

Each measurement counts with a part-of-speech tag and a lemma filter, keeps a decision only above
a 20-occurrence floor and a tenfold margin, and leaves an exact tie or an unattested lexeme
unmarked to fall back to the rules. The past marker is measured on two axes, word-final and before
a person suffix, because they come apart (`függött`, `függtem`). Alternations are measured per
cell, because `szó` binds in the plural (`szavak`) and stays free in the accusative (`szót`).

The candidate forms come from this crate's own generation: the converter writes
`local/lists/*-candidates.tsv` whenever `local/lists/` exists, the `measure_*.py` scripts decide
them against Webcorpus 2, and a second conversion reads the decisions. The `-ad`/`-ed` verbs come
from the `ad_verbs` example of `picur-morph-core`, which `measure_past.py` runs itself. The lists
not written by the converter (`antiharmonic.tsv`, `poss3-ja-exceptions.tsv`, `geminate-verbs.tsv`,
`cc-past-verbs.tsv`, `bare-accusative.tsv`) start from the `gen_*.py` generators and
`verify_accusative.py`.

```sh
cargo xtask morphdb local/research-impl/morphdb_morphdb.hu/in   # writes the candidates
for m in tools/measure_*.py; do python "$m"; done
cargo xtask morphdb local/research-impl/morphdb_morphdb.hu/in   # reads the decisions
```

A measured trait is never restated in `supplement.ron`, because a supplement row replaces the
converted row and would silently drop the measurement.

## Frequency

`frequency.ron` holds the 250,000 most frequent written forms with their rate in half-octave
buckets. `cargo xtask frequency` builds it from the `text` column of the pinned Hugging Face shards
under `local/corpora/` (sources and revisions in [WORKSPACE.md](../../WORKSPACE.md#corpora), licences in
[`NOTICE`](../../NOTICE)). A form is a run of word characters, split at `camelCase` and `snake_case`
seams and case-folded; counts are pooled across the sources, which weights each by its scanned
volume; forms spelled with letters only are kept; the 250,000 commonest survive; and each count is
recorded as `floor(2 * log2(count / count of the rarest kept form))`, clamped to a byte. The command
also writes the per-source counts to `local/cache/counts.json` for [`tools/corpus.py`](../../tools/corpus.py).
The command reproduces the committed file byte for byte.

The table is facts about those corpora, which forms occur and how often, and no expressive content
survives in it.

## Comparing with the other analysers

hunspell (`hu_HU`) and hunmorph-foma run locally and answer accept or reject for any form, so
they are the two witnesses in the overgeneration measure and the disagreement sets. Over the
UD Szeged types, the forms only this engine rejects while both accept are the lexicon's gaps
(names, loans) and were the stem classes and stacks it lacked; the forms only this engine accepts
are names and acronyms read by shape and coined compounds. The paradigms of a few hundred random
lemmas, fed to both, give the rate of forms neither vouches for, grouped by feature stack: a
stack both reject in every lemma is a cell the language does not have, and the deepest
possessed-plural anaphoric stacks (`házaiméiból`) are what remains there.

## Local oracles

Gitignored under `local/` (layout in [WORKSPACE.md](../../WORKSPACE.md)), never committed and never shipped:

| oracle | licence | use |
|---|---|---|
| morphdb.hu | CC-BY 2.5 | converted once into the committed RON, attributed in `NOTICE` |
| hunmorph-foma | BSD-3 | agreement checks |
| magyarispell | MPL/GPL/LGPL | study only |
| kaikki, UniMorph hun | CC BY-SA | study only |
| UD Szeged, emMorph | CC BY-NC-SA | evaluation and study only |
| NerKor | CC BY-SA 4.0 | evaluation only |
| Magyar Értelmező Kéziszótár | proprietary | study only |
| Webcorpus 2 frequency list | MIT | attestation counts |
| e-nyelv.hu advisory archive | cached | precedent search |

Only morphdb feeds committed data. Everything else informs decisions that are then restated in the
rulebook's own words with a citation, which is why [`docs/grammar/sources.md`](../grammar/sources.md) carries a locator for
every claim.

