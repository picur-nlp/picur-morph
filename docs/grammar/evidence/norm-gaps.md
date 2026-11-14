# Norm-gap evidence from the language advisory archive

Date: 2026-09-16. Method: offline search of the cached e-nyelv.hu advisory archive (43,125 answered
questions with date and URL, `local/cache/enyelv/qa.jsonl`, see [WORKSPACE.md](../../../WORKSPACE.md); search
tool [`tools/enyelv.py`](../../../tools/enyelv.py)). Queries were AND-term searches over headword, question and answer.

Status of this evidence: the advisory service is run by language professionals who cite AkH and the
reference works, but its answers are **not normative**. They are precedent: they show how the norm is
applied where its text is silent or self-contradictory. Rules resolved from this source are marked
`consensus` with the advisory locator, never `normative`. Answers are paraphrased, never quoted at
length; each carries its URL and date so the claim can be rechecked.

Source key: `ENYELV` (see sources.md).

## Resolved

### OQ-08-002 sports score colon (`PUN-SEMI-012`, `PUN-NUMF-021`)
The advisory summary states outright that there is no rule for writing sports results, that the colon
is the most norm-conform choice and is written spaced (`2 : 1`), that their own recommendation is a
spaced hyphen to avoid a range reading, and that the sports press writes an unspaced en dash (`2-1`).
Evidence: e-nyelv 2019-07-19 (summary of five earlier answers), 2021-11-16.
Decision: AkH12 §240 f)'s spaced `6 : 3` is the norm-conform form; the unspaced rendering in the
online edition is a rendering artefact, not a second norm. Record the press variant as a usage note.

### OQ-08-015 ratio and map-scale colon (`PUN-SEMI-012`)
The norm does not cover map scales; this belongs to subject-field orthographies. Attested practice in
atlases: `1 : 3000`, and with grouping for large numbers `1 : 3 000 000`.
Evidence: e-nyelv 2020-11-11, 2024-01-10.
Decision: spaced colon, with the digit grouping of `PUN-NUMF`; confidence `consensus`, not normative.

### OQ-08-014 weekday abbreviations (`PUN-ABBR-013`)
Several forms are acceptable (`H`/`Hé`, `K`/`Ke`, `Sz`/`Sze`, `Cs`/`Csüt`, `P`/`Pé`, `Sz`/`Szo`,
`V`/`Va`/`Vas`); the requirement is consistency within a document, and these carry no final period.
Evidence: e-nyelv 2026-01-20.
Decision: replace the single `csüt.` example with the variant set plus the consistency and no-period
conditions.

### OQ-08-008 abbreviation followed by a colon (`PUN-ABBR-007`)
`pl.:` is acceptable. The pattern occurs in 1,142 advisory answers, i.e. the professional advisers
themselves write it routinely.
Decision: acceptable; confidence `tendency` (usage evidence, no explicit ruling).

### OQ-08-009 telephone numbers (`PUN-NUMF-017`)
No normative format exists. Grouping is for memorability. The suffix follows the reading of the last
block: `450-8200-as`, because that block is read as a four-digit number.
Evidence: e-nyelv 2009-05-21, 2018-06-28.
Decision: state the suffix-by-reading rule; mark formats as convention, not norm.

### OQ-08-016 operator spacing (`PUN-DASH-016`)
The equals sign takes a space on both sides (the advisers cite AkH §275). The `2+3` form in the same
paragraph is an example, not a prescription, so unspaced arithmetic operators are not an error.
Evidence: e-nyelv 2022-07-14, 2025-10-16.
Decision: `=` spaced; `+`, `-` etc. unspaced in simple arithmetic; both readings documented.

### OQ-08-018 spaced en dash (`PUN-DASH-008`)
Between numbers and dates the en dash is unspaced (`2013-2014`); in complex cases it may stand with
spaces (the advisers cite AkH §264 c), e.g. between full dates with periods.
Evidence: e-nyelv 2026-06-21.
Decision: confirms the optional reading the chapter adopted; keep, and add the complex-case allowance.

### OQ-08-005 closing marks with parentheses (`PUN-PAREN-008`)
Editorial practice does not put two closing marks at a sentence end; a parenthetical inserted into a
sentence begins lowercase and keeps its own closing mark except a period.
Evidence: e-nyelv 2022-01-27, 2019-09-19.
Decision: resolves the §240 c), d) vs §253 conflict for our purposes: prefer one closing mark;
document the AkH example as an inconsistency in the source.

### OQ-07-015 foreign name particles (`ORT-CAP-019`)
`Van Gogh` as a headword or sentence-initially, `van Gogh` in running text, `Vincent van Gogh` with
the given name; suffixed and derived forms `Goghhal`, `goghi`. The advisers cite AkH §214.
Evidence: e-nyelv 2022-06-04, 2024-04-24.
Decision: encode the headword-vs-running-text distinction; it is a real conditioning factor the
chapter lacked.

### OQ-07-025 the 6:3 rule and proper names (`ORT-63-007`)
The mozgószabály is not applied to proper-name plus common-noun compounds (`Szent Margit-kolostor-
romok` keeps both hyphens) nor to multi-word foreign expressions (`via ferrata túra` stays apart).
Evidence: e-nyelv 2015-06-12, 2020-11-06.
Decision: the chapter's inference is confirmed at advisory level; upgrade from inference to
`consensus` with these locators.

### OQ-03-018 `hó` accusative (`NOM-STEM-016`)
Both `hót` and `havat` are correct, including in compounds (`műhót` ~ `műhavat`). The advisers state
no meaning split; a literary `hót` is acceptable.
Evidence: e-nyelv 2022-11-01, 2021-05-18.
Decision: drop the unsourced meaning-split claim attributed to SMNy3 p. 758; record two standard
variants and let the corpus ratio order them.

### OQ-07-021 single-vowel syllable line breaks (`ORT-SPLIT-004`)
AkH §226 a) is quoted by the advisers as making `haza-i`, `emóci-ó`, `Le-a`, `A-ron` forms "not
recommended in print", while two-syllable names of that shape are separable in principle but not
customarily broken.
Evidence: e-nyelv 2016-03-15, 2014-02-01.
Decision: the §226 a) "not recommended" reading stands; §236 concerns compound-boundary breaks, so
the two paragraphs do not actually conflict.

### OQ-01-013 `ssz` at a compound boundary (`WS-DIG-010`)
`hossz-szelvény`, `össz-szellemiség`: a compound boundary with three identical consonant letters
takes a hyphen and the digraph is written out (the advisers cite AkH §61, §92).
Evidence: e-nyelv 2021-11-01, 2023-02-27.
Decision: supports treating AkH12 §79's `ssz-el` as a typographical slip against §80; keep the
assumption and cite this precedent.

### OQ-09-009 infinitive chains (`SYN-ORDER-012`)
With a modal-type finite verb both orders are standard: the prefix may climb out of the infinitive or
stay on it (`le szeretném kísérni` ~ `szeretném lekísérni`).
Evidence: e-nyelv 2012-03-21.
Decision: record both as standard rather than preferring one.

### OQ-07-018 years in the 6:3 rule (`ORT-NUM-010`)
Years are not normally written out. e-nyelv applies the hyphen grouping to a spelled-out year
(`kétezer-tizenhat`), but AkH12 §291 a) exempts years from it ("kivéve az évszámokat", "de:
kétezertizennégy májusában"). Year ranges take the en dash and the suffix on the second member
(`1848–49-ben`, §264 c), §296).
Evidence: AkH12 §291 a), §264 c), §296; e-nyelv 2025-08-12, 2025-04-01 (not followed).
Decision: a spelled-out year is solid at any size; the norm text outranks the advisory.

## Partially advanced, still open

- **OQ-01-004 collation** (`WS-COLL-017`): the archive confirms hyphens are ignored in sorting and
  lowercase precedes uppercase (AkH §14, §14 d), but says nothing about morphology-aware digraph
  segmentation (`község`). The question stands.
- **OQ-07-008 syllable counting** (`ORT-63-003`): confirmed that `-i` does not count and `-szerű`
  does (AkH §139); nothing found on counting syllables inside foreign-spelled members.
- **OQ-07-004 prefixed participles** (`ORT-TOG-024`): prefixed verbs are written solid "in participial
  use too" (e-nyelv 2026-03-11), which supports treating them as simple words, but the syllable
  criterion itself is still unsourced.
- **OQ-07-010 three-element place names** (`ORT-NAME-024`): the archive shows geographic names and
  public-space names behave differently, without settling the §177 capitals reading.
- **OQ-09-005 relative-clause agreement** (`SYN-AGR-012`): only number agreement with the antecedent
  is covered (`amely`/`amelyek`); person agreement with 1st/2nd person antecedents is not.
- **OQ-06-013 semi-words** (`DER-COMP`, `ORT-63`): `-mentes` forms are treated as ordinary compound
  members in the answers, but `-beli` is not addressed.
- **OQ-10-008 `szöllő`** (`REP-SPELL-003`): named in an answer as a spoken variant that the norm does
  not codify, alongside `erőssen` (e-nyelv 2009-02-08); still not named as nonstandard in AkH itself.
- **OQ-01-002 letter names** (`WS-ALPH-007`): `x`-final common words take suffixes directly
  (`szexszel`, `fixen`), but the possessive of letter names (`l-je` vs `l-e`) is not addressed.

## No evidence in the archive

`OQ-04-002` and `OQ-07-017` (suffixing decimals), `OQ-04-011` (suffix of a written-but-unread time),
`OQ-08-010` (apostrophe in foreign names before a suffix), `OQ-01-001`, `OQ-01-005`, `OQ-01-007`,
`OQ-01-008`, `OQ-01-011`, `OQ-01-012`, `OQ-07-005`, `OQ-07-023`, `OQ-09-003`, `OQ-09-004`,
`OQ-09-014`, `OQ-09-016`, `OQ-09-020`, `OQ-10-007`.

These are either dating questions the archive does not discuss, dictionary-level claims, or syntax
questions outside its scope. Nothing in the archive contradicts the current rules; the questions stay
open with their existing "evidence that would settle it".

## Reproduction

```
python3 tools/enyelv.py "sporteredmény"          # OQ-08-002
python3 tools/enyelv.py "méretarány"             # OQ-08-015
python3 tools/enyelv.py "csüt"                   # OQ-08-014
python3 tools/enyelv.py "telefonszám" "tagol"    # OQ-08-009
python3 tools/enyelv.py "egyenlőségjel"          # OQ-08-016
python3 tools/enyelv.py "nagykötőjel" "szóköz"   # OQ-08-018
python3 tools/enyelv.py "Van Gogh"               # OQ-07-015
python3 tools/enyelv.py "mozgószabály" "tulajdonnév"  # OQ-07-025
python3 tools/enyelv.py "havat"                  # OQ-03-018
python3 tools/enyelv.py "elválasztás" "nem ajánlott"  # OQ-07-021
python3 tools/enyelv.py "ssz-szel"               # OQ-01-013
python3 tools/enyelv.py "főnévi igenév" "szórend"     # OQ-09-009
```
