# Hungarian grammar rulebook

Implementation specification for `picur-morph`. Every behaviour of the library traces to a rule ID here; tests reference rule IDs.

## Scope

The project has two objectives: generating and ranking Hungarian text for training data, and
repairing damaged text. Everything here serves one of them.

Out of scope: clause-level syntax (chapter 09, cut to the four rules other chapters reference), the comma,
semicolon, list, heading and address families of chapter 08, and collation. Syntax was measured
before it was cut: the detectors built against it fired on 0.01% to 0.05% of 63,202 `NerKor`
sentences, which is no discriminative power for ranking, and the rest of the chapter needs a parse.

In scope, in rough order of weight: the writing system and orthography that decide whether a word is
correct (chapters 01, 07), the morphology that decides whether a form exists (02 to 06), the
character- and token-level typography of chapter 08, and the repair and damage detection of chapter
10.

## Principles
- Logic over data: a phenomenon is encoded as a rule when the conditions are known; lexical data only for what is demonstrably lexical (closed classes, idiosyncrasies), with the class size and source stated
- Normative where a norm exists (AkH12 for spelling and punctuation), descriptive consensus elsewhere (SMNy, Törkenczy, Siptár-Törkenczy), statistical tendencies stated with numbers
- Variation is data, not error: where the standard allows or usage shows variants, the rule lists all variants and the preferred one
- No verbatim copying of copyrighted text; rules are paraphrased, examples are short word forms
- English text, Hungarian technical term in parentheses at first use, Hungarian examples in backticks

## Chapters
| File | ID prefix | Scope | Rules | Status |
|---|---|---|---|---|
| [01-writing-system.md](01-writing-system.md) | `WS` | alphabet, letters, digraphs, doubled digraphs, letter case, character set, Unicode normalisation, legacy encodings | 76 | verified, consolidated 2026-09-15 |
| [02-phonology.md](02-phonology.md) | `PH` | vowel and consonant classes, vowel harmony (incl. vacillation, antiharmony, compounds), length alternations, assimilations visible in spelling or allomorphy, pronunciation of spellings (native and foreign) for suffixing | 112 | verified, consolidated 2026-09-15 |
| [03-nominal.md](03-nominal.md) | `NOM` | nouns and adjectives: stem classes, number, possession, possessed plural, anaphoric `-é`, associative `-ék`, all cases, linking vowels, lowering, comparison, adverb formation | 119 | verified, consolidated 2026-09-15 |
| [04-pronouns-numerals-determiners.md](04-pronouns-numerals-determiners.md) | `PND` | personal, demonstrative, interrogative, relative, reflexive, reciprocal, indefinite and negative pronouns, postpositions with personal suffixes, articles `a`/`az`, numerals (cardinal, ordinal, fractions, distributive, multiplicative, spelled out), dates, times, measures | 119 | verified, consolidated 2026-09-15 |
| [05-verbal.md](05-verbal.md) | `VRB` | verb stem classes, `-ik` verbs, conjugation (indefinite, definite, `-lak`), tenses, moods, infinitive (personal), participles, adverbial participles, potential, causative, irregular and suppletive verbs, verbal prefixes | 122 | verified, consolidated 2026-09-15 |
| [06-derivation-compounding.md](06-derivation-compounding.md) | `DER` | derivational suffixes and their class effects, compound structure, compound-driven harmony and allomorphy, verbal prefix compounds | 147 | verified, consolidated 2026-09-15 |
| [07-orthography.md](07-orthography.md) | `ORT` | AkH12 spelling principles, long and short letters, `j`/`ly`, suffix attachment (names, foreign words, abbreviations, acronyms, numerals, symbols), writing together or apart, the 6:3 rule, hyphen use, capitalisation, hyphenation (elválasztás) | 309 | verified, consolidated 2026-09-15 |
| [08-punctuation-typography.md](08-punctuation-typography.md) | `PUN` | sentence punctuation, quotation marks, dashes and hyphens, parentheses, abbreviation periods, number, date and time formats, spacing | 133 | verified, consolidated 2026-09-15 |
| [09-syntax.md](09-syntax.md) | `SYN` | the four syntax rules other chapters reference: object definiteness, government, agreement | 4 | verified, consolidated 2026-09-15 |
| [10-text-repair.md](10-text-repair.md) | `REP` | character encoding damage (Latin-2, `õ`/`û`, mojibake), font substitutions, spacing damage (letterspacing, broken syllables), typical spelling errors, keyboard and OCR confusions, diacritic restoration | 116 | verified, consolidated 2026-09-15 |
| [glossary.md](glossary.md) | - | Hungarian term, English term, UD / UniMorph feature mapping, rule areas | 363 terms | - |
| [sources.md](sources.md) | - | source register: key, bibliographic entry, URL, access, licence, chapters | 108 sources | - |

## Status
- Rules: 1257 (IDs unique, all 11 fields present and in order, cross-references resolve, every `[KEY]` citation resolves to sources.md; `cargo xtask lint-rules` checks the provenance of every `Lexical input`)
- Open questions: 155 tracked in [open-questions.md](open-questions.md): 65 open, 85 resolved, 5 retired
- Sources: 108 keys in [sources.md](sources.md)
- 1016 rules consume lexical input. Resources the implementation must provide, with the rules that define them:
  - Nominal stem record: `lowering` (`NOM-LOW`), `epenthetic` + bound alternant (`PH-ZERO-001`..`005`), `shortening` and lexical length (`PH-LEN-007`..`009`, `PH-CONS-004`), `v_stem` (`PH-VSTEM-002`..`006`), `h_silent` (`PH-CONS-005`, `NOM-STEM-004`), `poss3`/`poss_pl` (`NOM-POSS-006`, `NOM-PPL-001`), `acc_bare` (`NOM-ACC-009`), `time_noun`, `loc_t`, `place_case`, `human`, `adv_ul` (`NOM-CASE-016`, `NOM-CASE-017`, `NOM-PROP-006`, `NOM-ASSOC-001`, `NOM-ADJ-011`)
  - Harmony: stem class and vacillation `p_back_V`/`p_back_C` (`PH-HARM-013`, `PH-HARM-015`), suffix transparency and opacity (`PH-HARM-019`, `PH-HARM-020`, `DER-MODEL-008`)
  - Verb record: `ik` (`VRB-IK`), heavy `H` and stem slots `SQ SJ SD SP SA SV` (`VRB-MODEL-004`, `VRB-MODEL-006`), `epenthetic` (`PH-ZERO-003`), `ad_suffixal`, `cc_past`, geminate type, `defective` (`VRB-STEM-005`, `VRB-PAST-007`, `VRB-PAST-004`, `VRB-STEM-009`), sz-d(-v) and irregular tables (`VRB-STEM-012`, `VRB-IRR`), causative flags (`VRB-CAUS-001`), `transitive`/`stative` (`VRB-PART-004`, `DER-ADJ-010`)
  - Valency: government frames (`SYN-GOV-001`)
  - Derivational suffix record: linking vowel type, LVL, lengthening, truncation, harmony, `semi_word`, productivity (`DER-MODEL`); semantic noun and adjective features (`DER-MODEL-012`, `DER-ADJ-013`, `DER-VERB-008`); blocking and lexicalised derivatives (`DER-LEX`)
  - Compounds and prefixes: member boundaries and member type (`DER-COMP-026`, `WS-DIG-007`), verbal prefix inventory (`VRB-PREV-001`, `ORT-TOG-026`, `ORT-63-004`), bound-element and semi-word lists (`ORT-TOG-038`, `DER-COMP-017`)
  - Pronunciation: letter names (`WS-ALPH-006`, `PH-G2P-010`), acronym `reading` and `pron` (`PH-G2P-011`, `ORT-ABBR-014`), pronunciation of archaic and foreign spellings (`ORT-NAME-003`, `PH-G2P-009`, `WS-LET-004`), spoken forms for article choice (`PND-ART`)
  - Closed lists: abbreviations with period flag and expansion (`ORT-ABBR`, `PUN-ABBR`, sentence segmentation (out of scope)), postpositions with class and personal forms (`PND-POSTP`), pronoun series and numeral tables (`PND-INDF-001`, `PND-CARD`, `PND-ORD`, `PND-NUMINF`), month, weekday and time-adverbial tables (`PND-DATE-001`, `PND-DATE-002`), currency list (`PUN-NUMF-008`), variant and doublet links (`ORT-SUFX-006`, `NOM-STEM-018`, `VRB-IK-007`)
  - Names: proper-name flag and class, name lexicon, official status and title spans (`ORT-NAME`, `ORT-CAP`, `NOM-PROP-002`)
  - Repair: restoration lexicon and analyser, keyboard and OCR confusion tables, AkH11-to-AkH12 change list (`REP-DIAC`, `REP-KEY`, `REP-OCR-007`, `REP-SPELL-010`); corpus frequency list `ATT` for attestation

## Evidence
Six evidence files under [evidence/](evidence/), all dated 2026-09-16, hold the measurements behind the `Resolution 2026-09-16` entries in each chapter's verification log. They are evidence, not specification: each proposes field-level rule edits which the chapters then carry.
- [evidence/corpus-variants.md](evidence/corpus-variants.md) - every `usage-variant` question (30) and the `source-conflict` ones a corpus can arbitrate (10), measured on Webcorpus 2 for the ratio and the homograph control, on the project rate index for the register gradient, and on the cleaned parquet for hyphenated and multiword patterns. Ratios are quoted only above a 20-token floor, with Wilson intervals; figures aggregating a restricted source are marked non-publishable.
- [evidence/lexical-lists.md](evidence/lexical-lists.md) - 16 lexical-list questions closed by error-driven induction with cross-resource voting (corpus, morphdb, our `.ron`, foma, hunspell, UniMorph, kaikki, plus 120 spelling-checker tie-breaks). Produces the 27 lists under `local/lists/`, a 1,255-item review queue that collapses into five residual questions, and an oracle-agreement table.
- [evidence/productivity-tagset.md](evidence/productivity-tagset.md) - Yang's Tolerance Principle applied to seven productivity questions, with a productive and a listed control run first to validate the method; plus the UD and UniMorph mapping decisions, settled against the validator's own `feats.json` because `Case.lvalues = []` for hu forbids inventing a value.
- [evidence/calibration.md](evidence/calibration.md) - measured thresholds for the `statistical-estimate` questions, each stated next to the decision it serves and the cost of each error direction (standing target: precision >= 0.995). Nine values changed, three confirmed or withdrawn, fourteen reported uncalibrated with the missing asset named.
- [evidence/norm-gaps.md](evidence/norm-gaps.md) - the `norm-gap` questions worked against the cached e-nyelv.hu advisory archive (43,125 answered questions): 15 resolved, 8 advanced, 17 with no evidence in the archive. Advisory answers are precedent, not norm: rules resolved from them are `consensus` with a dated URL.
- [evidence/norm-gaps-online.md](evidence/norm-gaps-online.md) - the 17 questions the archive could not settle, taken online one at a time: 17 resolved, 1 advanced, none open (plus `OQ-01-002` from the offline pass's advanced list). The last two were settled by native-speaker judgements given in session, each of which generalised further than the example that prompted it. Sources are the decree text from the Nemzeti Jogszabálytár, two open-access studies (the AkH12 editor's own AkH11 -> AkH12 change list, and Bartos's open-access precursor to the SMNy3 chapter) and corpus probes over the project corpus (not distributed), since the Webcorpus2 frequency list is lowercased and strips apostrophes.

The scripts that produce these measurements are in [tools/](../../tools/README.md); the inputs they read are laid out in [WORKSPACE.md](../../WORKSPACE.md).

## Rule format
Each rule is a level-3 heading `### <ID> <short title>` followed by these fields, in this order, omitting none (use `-` when empty):

```markdown
### NOM-ACC-002 Linking vowel before the accusative after a consonant
- **Statement**: one or two precise sentences.
- **Conditions**: bullet list of the exact triggering context (stem class, final segments, following suffix, part of speech).
- **Output**: the resulting form or choice, including all allowed variants and which is preferred.
- **Examples**: `hal` -> `halat`; `dal` -> `dalt`; ...
- **Exceptions**: listed items or sub-rules; for closed lists give size and where the full list comes from.
- **Predictability**: `rule` | `tendency` (with statistics) | `lexical` (closed class, size).
- **Lexical input**: `none` or the lexical property consumed (e.g. stem property `lowering`).
- **Interactions**: related rule IDs and the order of application.
- **Sources**: keys from sources.md with locators, e.g. `[AkH12 §82]`, `[T04 §4.1.2.2]`, `[SMNy3 p. 761]`.
- **Validation**: how the implementation is checked: `goldens`, `attestation` (corpus probe), `oracle:<name>`.
- **Confidence**: `normative` | `consensus` | `tendency` | `contested`, with a note when sources disagree.
```

IDs: `<PREFIX>-<AREA>-<NNN>`, `AREA` a short uppercase mnemonic (`ACC`, `PL`, `HARM`); numbers never reused. Each chapter opens with a scope paragraph and an area index, and ends with `## Open questions`.

## Notation
- `->` derivation or output, `~` free variation, `*` ungrammatical form, `?` marginal form
- `C` consonant letter (digraphs count as one), `V` vowel letter, `#` word boundary, `+` morpheme boundary
- Archiphonemes in suffix templates: `A` = a/e, `Á` = á/é, `O` = o/e/ö, `Ó` = ó/ő, `U` = u/ü, `Ú` = ú/ű; a parenthesised vowel is a linking vowel present only after a consonant-final stem: `-(O)k`
- Harmony classes: `B` back, `F` front unrounded, `R` front rounded; vowel sets as in `PH-HARM`

## Validation assets
- [`crates/picur-morph-lexicon/data/frequency.ron`](../../crates/picur-morph-lexicon/data/frequency.ron): permissive corpus word frequencies (top 250k); attestation evidence
- Local oracles (never committed): listed with their licences in [docs/design/06-reproducibility.md](../design/06-reproducibility.md#local-oracles)
