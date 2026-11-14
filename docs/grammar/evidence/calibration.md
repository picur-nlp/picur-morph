# Threshold and weight calibration

Date: 2026-09-16. Workstream F. Addresses the `statistical-estimate` entries of
[`docs/grammar/open-questions.md`](../open-questions.md) (23 entries): 11 in chapter 10 (text repair), 5 in
chapter 02 (harmony and pronunciation), 1 in chapter 01, and 6 spread over chapters
03, 04, 08 and 09. Nine are settled with a new measured value, three are confirmed or
withdrawn, and fourteen are reported uncalibrated with the missing asset named (the
counts overlap: one entry can be part-settled).

This file is evidence and a proposal. No chapter, `glossary.md` or `open-questions.md`
was edited. Every rule change below is a proposal, given as `rule ID | field | old -> new`.

## Method

### The decision, then the cost of each error type

A threshold is only meaningful next to the decision it serves. For every item below
the write-up states, in order: what the threshold decides, what a false positive
costs, what a false negative costs, and the target precision that follows.

For a source-of-truth library, precision dominates recall in exactly one direction:
the library's output is consumed as fact, so a wrong repair is a corrupted record
while a missed repair is merely an unrepaired one that a later pass or a human can
still catch. The standing target is therefore **precision >= 0.995**, and the
operating point for each threshold is picked at the lowest threshold that clears it.
Three rules take a different target and say so: REP-DIAC-008 `balanced` (a stated
policy level below `conservative`), REP-SPACE-001 (precision is bounded at 0.9966 by
the negative set itself; see that section) and REP-DIAC-001 (a segment *label*, not a
rewrite, so a false negative is the expensive one).

### The evaluation sets

Real damaged/clean pairs are preferred. Where the corpus has none, damage is injected
into clean held-out text with the noise model the rulebook itself specifies
(`REP-NOISE-002 .. 005`), whose reference implementation is
the project's damage model (`PT-DAMAGE`) - so the generator and the detector
are tested against each other, which is what `REP-NOISE-011` asks for.

| set | size | provenance |
| --- | ---: | --- |
| `deaccent` pairs | 7,642 paragraph pairs, 565,493 aligned tokens | the project's damaged/clean pair file (not distributed); gold is real corpus text, damaged side verified to be an exact accent strip of gold (7,642 of 7,642) |
| `mojibake` pairs | 3,080 | same file; UTF-8 as cp1252, Latin-2 as Latin-1, HTML entities, font shift |
| `pdf` pairs | 2,821 (1,213 with letterspacing, 1,227 with syllable fragmentation) | same file |
| `ocr` pairs | 2,984 | same file |
| `typo` pairs | 3,073 | same file |
| gold negatives | 19,757 paragraphs, 19,038 distinct, 8.71 M letters, 1.42 M tokens | the gold sides of all of the above |
| injected 7-bit | 4,000 segments per convention | flying accent, 123 notation, ISO 646-HU applied to gold segments of >= 30 tokens |
| Webcorpus 2 | 36,936,726 rows, 9e9 tokens | Racz, MIT, Zenodo 10.5281/zenodo.17508385 |
| project corpus | 24,183,772 documents, 139.1 Gchar; 9,644,687 documents / 77.9 Gchar Hungarian | the project corpus (not distributed), 2,430 parquet parts, 43 sources, 14 registers, full scan |
| Hayes stem data | 8,915 curated / 10,974 uncurated stems with nak/nek counts | `local/research-lit/HungarianFromGoogleRevised.txt`, Latin-2 |

Registers in the pair file: `legal`, `informal`, `educational`, `encyclopedic`, `web`,
`news`, `narrative`, `dialogue`. Lanes: `sft` 13,811, `cpt` 3,994, `eval` 1,952.
Cross-validation folds are `sha1(id) mod 5`, so a fold never splits a document.

### The word-acceptance oracle

The rulebook's `oracle` is the morphological analyser. No analyser binary is available
in this repo, so acceptance is approximated by the project's own 250k form list
([`crates/picur-morph-lexicon/data/frequency.ron`](../../../crates/picur-morph-lexicon/data/frequency.ron), 200,229 forms with freq > 0).
Webcorpus 2 attestation at freq >= 5 (7,809,629 forms) was tried first and **is not
usable as an acceptance oracle**: over a 9e9-token web corpus every syllable fragment
(`ren`, `de`, `le`, `tei`) is attested, so it rejects nothing. That failure is itself
reported below, because it bounds what `REP-SPACE-003`'s run-evidence clause can be
calibrated against without an analyser.

### Reproduction

The evaluation sets and the corpus scanned here are not distributed, so the figures in this file
are a dated record and are not reproducible from this repository. Every random draw used seed
`20260916`.

---

## 1. REP-QUAL-001 - Hungarian letter and accent baselines

**Decides**: the reference values every accent-loss test in `DIAC` compares against.
Not a decision threshold itself; an error here propagates into `REP-DIAC-001` and
`REP-DIAC-002`, so it is measured first.

**Evaluation set**: two independent measurements, deliberately not pooled.

- **Corpus-wide**: a full scan of the project corpus (not distributed) restricted to
  `language = 'hu'` - **9,644,687 documents, 77.92 Gchar, 63.42 G letters, 10.40 G word
  tokens**, 43 sources, 14 registers. This is the headline.
- **Pair gold**: the 19,038 distinct gold paragraphs of the evaluation sets used
  everywhere else in this file - 10,560,239 characters, 8,713,604 letters, 1,417,558
  tokens. This is the cross-check, and it matters because every precision figure below is
  measured against *this* text.

No damage; this is a description, not a classification, so there is no precision to
report.

| quantity | rulebook | corpus-wide | pair gold | `web` register alone |
| --- | --- | ---: | ---: | ---: |
| accented letters / letters | 11.8% (`[ATT]`) | **11.755%** | 11.90% | 11.94% |
| accented letters / characters | 6.59% (`[LY20]`) | **9.568%** | 9.82% | 9.79% |
| tokens with >= 1 accent | 43.16 / 35.75 / 48% | **47.769%** | 49.69% | 49.02% |

The two measurements agree to within 0.7 points on every axis once the register mix is
accounted for: the pair gold is crawl-weighted, and the `web` register alone reproduces
it. The systematic gap is the 2.17 M non-web Hungarian documents, whose accent rates are
lower. `[ATT]`'s 11.8% of letters is confirmed on 63 G letters. **`[LY20]`'s 6.59% of
characters is 31% low** and `[NS15]`'s 43.16% of tokens is 10% low against this corpus.

Per-family distribution, percent of family, corpus-wide against `[NS15 Table 1]`:

| letter | NS15 | corpus-wide | pair gold | delta vs NS15 |
| --- | ---: | ---: | ---: | ---: |
| a / á | 70.33 / 29.66 | 70.62 / 29.38 | 71.46 / 28.54 | +0.29 |
| e / é | 73.40 / 26.59 | 74.14 / 25.86 | 74.17 / 25.83 | +0.74 |
| i / í | 86.04 / 13.95 | 87.61 / 12.39 | 86.34 / 13.66 | +1.57 |
| o / ó / ö / ő | 55.41 / 14.65 / 15.82 / 14.10 | 57.37 / 14.87 / 14.87 / 12.89 | 56.47 / 14.42 / 15.46 / 13.65 | +1.96 max |
| u / ú / ü / ű | 46.96 / 12.72 / 29.98 / 10.32 | 51.72 / 12.84 / 26.50 / 8.94 | 45.96 / 14.49 / 29.17 / 10.39 | +4.76 max |

NS15 is confirmed within 2 points for every letter except the `u` family, where this
corpus has 4.8 points more plain `u` and 3.5 points fewer `ü`. Both double-acute letters
run about 1.2 points below NS15 corpus-wide (`ő` 12.89 vs 14.10, `ű` 8.94 vs 10.32);
part of that is register mix and part is real unrepaired damage still in the corpus
(sections 11.2 and 11.3).

Derived baselines: `ö/(ö+ő)` = **0.5358** corpus-wide, 0.531 on the pair gold - the
rulebook's 0.53 is confirmed twice. `í/(i+í)` = **0.1239** corpus-wide, 0.1366 on the
pair gold - the rulebook cites 0.14 `[NS15]` and 0.13 `[ATT]`, and the corpus sits at
`[ATT]`'s value.

### The register spread is larger than the error the baselines are used to detect

This is the part that matters for `REP-DIAC-002`, which compares a document's family
share against a single fixed number:

| register | docs | acc/token | `ö/(ö+ő)` | `ő` % of o-family | `ű` % of u-family |
| --- | ---: | ---: | ---: | ---: | ---: |
| legal | 32,078 | 51.82% | 0.5340 | 14.07 | 12.64 |
| **administrative** | 14,599 | 51.56% | **0.3916** | **21.68** | **17.37** |
| procedural | 3,854 | 49.66% | 0.4921 | 15.83 | 11.42 |
| web | 7,476,217 | 49.02% | 0.5328 | 13.11 | 9.01 |
| news | 1,407,208 | 46.96% | 0.5153 | 13.52 | 8.32 |
| academic | 166,090 | 45.10% | 0.5450 | 12.21 | 8.63 |
| encyclopedic | 336,539 | 44.19% | 0.5525 | 11.64 | 9.21 |
| **narrative** | 7,945 | 43.56% | **0.6025** | 10.87 | 7.52 |
| informal | 79,700 | 40.57% | 0.5685 | 10.59 | 9.47 |
| **dialogue** | 103,258 | **37.20%** | 0.5993 | 9.58 | 6.78 |

`ö/(ö+ő)` runs from **0.39 (administrative) to 0.60 (narrative)** against a fixed
baseline of 0.53, and the accented-token share from 37.2% to 51.8% against a fixed 48%.
Two consequences, both of which the chapter should record:

- **`dialogue` is the low-accent register.** Subtitle and parliamentary text at 37.2% of
  tokens will read as partially stripped to a detector tuned on 43-48%.
- **`administrative` is the high-`ő` register**, at `ö/(ö+ő)` = 0.39. `REP-DIAC-002`'s
  class C test asks for `share(ö) > 0.40`; administrative Hungarian sits *below* that line
  while undamaged, which is the wrong side of the test.

**Proposed edits**

| rule | field | old -> new |
| --- | --- | --- |
| `REP-QUAL-001` | Output, accented tokens | `43.16% [NS15]; 35.75% of words and 6.59% of characters [LY20]; frequency-weighted estimate from [ATT]: 48% of tokens, 11.8% of letters (approximate, half-octave buckets)` -> `47.77% of tokens, 11.76% of letters, 9.57% of characters, measured on 9,644,687 Hungarian documents (63.4 G letters) of the project corpus; [ATT]'s 11.8% of letters is confirmed, [LY20]'s 6.59% of characters is 31% low and [NS15]'s 43.16% of tokens 10% low against it` |
| `REP-QUAL-001` | Output, per family | keep the NS15 table; add `confirmed corpus-wide within 2 points for every letter except the u family, where this corpus has 4.8 points more plain u and 3.5 points fewer u-umlaut (63.4 G letters)` |
| `REP-QUAL-001` | Conditions | add: `these are corpus-wide values; the register spread is wider than the damage they are used to detect - accented-token share runs 37.2% (dialogue) to 51.8% (legal), and o-umlaut/(o-umlaut+o-double-acute) runs 0.39 (administrative) to 0.60 (narrative). Compare a document against its own source distribution where one is known.` |
| `REP-DIAC-002` | class C Detection | `baseline ö/(ö+ő) = 0.53` -> `0.536 corpus-wide, but 0.39 in administrative and 0.60 in narrative text; the share(ö) > 0.40 clause puts undamaged administrative Hungarian on the wrong side of the test` |
| `REP-DIAC-002` | class E Detection | `baseline 0.14 [NS15], 0.13 [ATT]` -> `0.124 (measured corpus-wide, 63.4 G letters)` |

---

## 2. REP-DIAC-004 and REP-DIAC-008 - diacritic restoration ranking and abstention

**Decides**: whether an accent-stripped token is rewritten, and to what.
A false positive rewrites a correct word into a different word (`meg` -> `még`), which
is a semantic corruption of the record. A false negative leaves the token unaccented,
which is visible and repairable. Target precision **>= 0.995**.

### The model measured

`REP-DIAC-003` + `REP-DIAC-004` **minus the context term**: candidates are the
accent-only respellings of the token attested in Webcorpus 2 at freq >= 2; the prior is
the Webcorpus 2 form frequency; the decision is arg max prior. This is the unigram
floor - it measures exactly how much of the work the prior alone does, and it is a
lower bound on the full rule, which adds a sentence language model.

Confidence was computed under both readings of `REP-DIAC-008`'s "posterior margin":
`margin_set` = p(best) over the normalised candidate set, and `margin_pair` =
p(best)/(p(best)+p(second)), the literal reading. The two curves agree to within
0.003 precision at every operating point, so the choice of reading is not load-bearing;
the tables below give `margin_set`.

### Ambiguity profile

565,493 aligned tokens from 7,642 paragraph pairs. The `REP-DIAC-009` guards
(URL, e-mail, path, domain) were applied and excluded.

| candidates per token | tokens | share |
| ---: | ---: | ---: |
| 0 (out of lexicon) | 2,674 | 0.47% |
| 1 (unambiguous) | 45,073 | 7.97% |
| 2 | 230,427 | 40.75% |
| 3 | 80,105 | 14.17% |
| 4 | 102,645 | 18.15% |
| 5 | 38,025 | 6.72% |
| 6 | 23,092 | 4.08% |
| 7 | 16,058 | 2.84% |
| >= 8 | 27,394 | 4.84% |

**91.6% of accent-stripped tokens are ambiguous under accent-only respelling.** This is
the number `REP-DIAC-003` needs and did not have: the analyser-filtered candidate set
is not a near-deterministic map, it is genuinely a ranking problem for nine tokens in ten.

Top-1 accuracy, no threshold: **97.91%** overall; **99.75%** on the 45,073 unambiguous
tokens; **98.07%** on the 517,746 ambiguous ones.

### Margin sweep

279,754 of 565,493 tokens (49.5%) need a change. A rewrite is a true positive when the
output equals gold (with `REP-DIAC-010` doublets counted as correct).

| margin | tp | fp | fn | precision | recall | F1 | token accuracy | rewrite rate |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0.00 | 270,378 | 5,781 | 6,070 | 0.9791 | 0.9780 | 0.9786 | 0.9791 | 0.488 |
| 0.50 | 270,097 | 5,503 | 6,567 | 0.9800 | 0.9763 | 0.9781 | 0.9787 | 0.487 |
| **0.60** | 268,207 | 4,181 | 9,297 | **0.9847** | **0.9665** | 0.9755 | 0.9762 | 0.482 |
| 0.70 | 265,696 | 2,993 | 12,482 | 0.9889 | 0.9551 | 0.9717 | 0.9727 | 0.475 |
| 0.75 | 264,243 | 2,500 | 14,189 | 0.9906 | 0.9490 | 0.9694 | 0.9705 | 0.472 |
| 0.80 | 262,872 | 2,180 | 15,791 | 0.9918 | 0.9433 | 0.9669 | 0.9683 | 0.469 |
| 0.85 | 260,184 | 1,661 | 18,762 | 0.9937 | 0.9327 | 0.9622 | 0.9639 | 0.463 |
| 0.90 | 257,017 | 1,319 | 22,130 | 0.9949 | 0.9207 | 0.9564 | 0.9586 | 0.457 |
| **0.95** | 248,380 | 870 | 31,006 | **0.9965** | **0.8890** | 0.9397 | 0.9437 | 0.441 |
| 0.98 | 218,738 | 550 | 60,833 | 0.9975 | 0.7824 | 0.8770 | 0.8915 | 0.388 |
| 0.99 | 165,951 | 398 | 113,702 | 0.9976 | 0.5934 | 0.7442 | 0.7983 | 0.294 |

The rulebook's `conservative` threshold of 0.9 measures **0.99489**, which misses the
0.995 target. The lowest swept point that clears it is **0.95**, at precision 0.99651
and recall 0.88910. The `balanced` threshold of 0.6 measures 0.9847, which is the
right shape for a level below `conservative` and is confirmed.

### Cross-validation (5-fold, `sha1(id) mod 5`)

| margin | precision mean | precision sd | recall mean | recall sd | tokens per fold |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 0.00 | 0.97908 | 0.00105 | 0.97806 | 0.00125 | 106,213 - 116,529 |
| 0.60 | 0.98465 | 0.00067 | 0.96654 | 0.00179 | " |
| 0.90 | 0.99489 | 0.00024 | 0.92079 | 0.00267 | " |
| 0.95 | 0.99651 | 0.00024 | 0.88910 | 0.00305 | " |

The fold-to-fold spread at 0.95 is +-0.00024 precision, so the gap between 0.9 and 0.95
(0.0016) is about seven standard deviations wide: the threshold move is not noise.

### Per register

| register | tokens | P (t=0) | R (t=0) | acc (t=0) | P (t=0.9) | R (t=0.9) |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| legal | 93,369 | 0.9910 | 0.9900 | 0.9887 | 0.9984 | 0.9652 |
| news | 60,524 | 0.9811 | 0.9825 | 0.9822 | 0.9963 | 0.9258 |
| educational | 62,968 | 0.9806 | 0.9799 | 0.9802 | 0.9963 | 0.9234 |
| informal | 116,742 | 0.9800 | 0.9720 | 0.9792 | 0.9954 | 0.9121 |
| web | 79,850 | 0.9767 | 0.9817 | 0.9801 | 0.9949 | 0.9187 |
| encyclopedic | 60,974 | 0.9757 | 0.9777 | 0.9758 | 0.9937 | 0.9123 |
| dialogue | 23,030 | 0.9697 | 0.9636 | 0.9749 | 0.9935 | 0.8765 |
| narrative | 68,036 | 0.9611 | 0.9597 | 0.9651 | 0.9856 | 0.8703 |

The spread is 3.0 points of unthresholded precision between `legal` (0.9910) and
`narrative` (0.9611). `narrative` is the hard register: literary text has the widest
form vocabulary and the most free-variant spellings, and it is the only register that
still misses 0.995 at margin 0.9 (0.9856). A per-register threshold is not proposed -
the policy levels are a user-facing contract - but 0.95 brings `narrative` to target
where 0.9 does not.

### Residual error classes

11,832 wrong tokens at margin 0:

| class | count | share of errors |
| --- | ---: | ---: |
| ambiguous, prior picked the wrong candidate | 10,006 | 84.6% |
| out of lexicon, token kept unaccented | 1,712 | 14.5% |
| single candidate but wrong (lexicon gap) | 114 | 1.0% |

Top confusions (damaged -> gold, predicted):

| damaged | gold | predicted | n | class |
| --- | --- | --- | ---: | --- |
| meg | még | meg | 1,454 | function word, needs context (`REP-DIAC-004` names it) |
| es | es | és | 250 | gold itself is unaccented; not a model error |
| fel | fél | fel | 173 | homograph, needs context |
| ot | őt | öt | 123 | homograph |
| elo | élő | elő | 123 | homograph |
| hat | hat | hát | 103 | homograph |
| el | él | el | 88 | homograph |
| akar | akar | akár | 86 | homograph |
| szinten | szinten | szintén | 59 | homograph |
| teruleten | területen | területén | 56 | inflectional (`REP-DIAC-006`) |
| ugy | ügy | úgy | 51 | homograph |
| vettek | vették | vettek | 51 | inflectional, definiteness |
| cimet | címét | címet | 46 | inflectional, possessive |

`meg` / `még` alone is 12.3% of all errors and 14.5% of the ambiguous-class errors. It
is a single lexical pair that no unigram prior can ever get right and that a bigram
context model gets right most of the time. That is the strongest measured argument for
`REP-DIAC-004`'s context term, and it quantifies what the term has to buy: at best it
removes about 85% of the residual error, taking top-1 from 97.91% toward 99.7%.

A second class is visible in row 2: `es` -> gold `es` 250 times means the *gold* text
is itself partially unaccented in places. Measured precision is therefore a slight
under-estimate, by at most a few tenths of a point.

### Evidence size, and why the lane split does not apply

565,493 tokens, 7,642 paragraphs, 8 registers, 5 folds. Not provisional. Per register
the smallest cell is `dialogue` at 23,030 tokens, also not provisional.

All pairs were used, not only the `eval` lane. Nothing was fitted on them: the candidate
set and the prior come entirely from Webcorpus 2, an external corpus, and the only free
parameter is the margin, which is swept and reported rather than chosen by fitting. So
every pair is held out with respect to the model, and restricting to the `eval` lane
would only shrink the sample. `diac_eval.py --lane eval` reproduces the run on the 1,952
held-out pairs for anyone who wants the stricter reading.

**Proposed edits**

| rule | field | old -> new |
| --- | --- | --- |
| `REP-DIAC-008` | Conditions, policies | `conservative (margin >= 0.9, oracle-accepted candidates only)` -> `conservative (margin >= 0.95, oracle-accepted candidates only)` |
| `REP-DIAC-008` | Conditions, policies | `balanced (>= 0.6)` -> unchanged; add the measured figures |
| `REP-DIAC-008` | Conditions | add: `margin is p(best) over the normalised candidate set; the pairwise reading p(best)/(p(best)+p(second)) gives the same operating points within 0.003 precision` |
| `REP-DIAC-008` | Validation | `precision/recall per policy on held-out pairs` -> `measured on 565,493 tokens of 7,642 accent-stripped pairs: conservative 0.9965 / 0.8890, balanced 0.9847 / 0.9665, aggressive 0.9791 / 0.9780 (frequency prior only, no context model)` |
| `REP-DIAC-008` | Confidence | `consensus` -> `measured` |
| `REP-DIAC-003` | Output | add: `91.6% of accent-stripped tokens have more than one attested accent-only respelling (565,493 tokens); the analyser filter reduces the candidate count but does not usually reduce it to one` |
| `REP-DIAC-004` | Output, Confidence | add: `the frequency prior alone reaches 97.91% top-1 on 565,493 held-out tokens; 84.6% of the residual error is ambiguous tokens the prior ranks wrongly, of which meg/meg-acute alone is 14.5%, so the context term is what the remaining accuracy depends on` |

---

## 3. REP-DIAC-001 - detecting accent-stripped text

**Decides**: a segment label (`stripped` / `accented` / `abstain`), not a rewrite. A
false positive sends clean text into the restorer, which then has its own guards and
margins; a false negative silently skips a damaged segment entirely. So here the false
negative is the expensive one and the target is maximum recall at precision 1.

**Evaluation set**: 7,642 stripped paragraphs (positives) against 19,757 gold paragraphs
(negatives), same registers.

### The accent-density clause is not the binding one

| quantity | measured |
| --- | ---: |
| accented-letter share, clean segments, median | 0.11828 |
| accented-letter share, clean segments, **minimum over 19,757** | **0.006623** |
| accented-letter share, stripped segments, **maximum over 7,642** | **0.000000** |

The classes are perfectly separated. Sweeping the cut from 0.0005 to 0.05 changes
nothing at all: precision is 1.00000 and recall is identical at every value. The
rulebook's 0.5% therefore cannot be wrong, but it sits only 1.33x below the lowest
clean segment seen, which is thin headroom for text this rule will meet in the wild
(formula-heavy or heavily-quoted Hungarian). 0.002 keeps the same measured behaviour
with 3.3x headroom and is proposed for that reason alone.

### The function-word clauses cost half the recall for nothing

Accent share fixed at < 0.005. `sig` is the count of distinct signature function words;
`rare` the token count of {`es`, `mar`, `igy`, `utan`, `ugy`, `tobb`, `kozott`}.

| sig >= | rare >= | tp | fp | precision | recall |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 0 | 7,642 | 0 | 1.00000 | 1.00000 |
| 2 | 0 | 7,617 | 0 | 1.00000 | 0.99673 |
| **3** | **1** | 6,463 | 0 | **1.00000** | **0.84572** |
| 3 | 2 | 4,655 | 0 | 1.00000 | 0.60913 |
| 4 | 2 | 4,399 | 0 | 1.00000 | 0.57563 |
| **5** | **2** | 4,057 | 0 | **1.00000** | **0.53088** |
| 6 | 2 | 3,561 | 0 | 1.00000 | 0.46598 |
| 8 | 2 | 2,443 | 0 | 1.00000 | 0.31968 |

The rulebook's combination (`sig >= 5`, `rare >= 2`) **misses 47% of stripped
Hungarian** and buys no precision at all on Hungarian negatives. Moving to
`sig >= 3, rare >= 1` raises recall from 0.531 to 0.846 with precision still exactly
1.00000 on 19,757 clean segments.

**Honest limit on that sweep.** The signature clauses exist to keep English, Dutch and
Latin out, and this evaluation has no foreign negatives: the project corpus (not distributed)
carries an empty `lang` column for all 441,718 sampled documents, and the pair file is
Hungarian throughout. So what is measured is that the clauses are *useless against
Hungarian negatives*, not that they are useless in general.

### The third clause does the work, and it is the language-independent one

The rule's third clause - ">= 60% of tokens rejected as written become accepted after the
best restoration" - was measured separately. It is the only clause that is not a
Hungarian word list: it asks whether the segment's *unknown* tokens turn into Hungarian
words when accents are put back, which stripped English or Latin cannot do. Oracle: the
curated 250k list; restoration: the section-2 model.

| quantity | stripped (n = 7,642) | clean (n = 18,337) |
| --- | ---: | ---: |
| restoration-acceptance ratio, median | 0.8235 | 0.0000 |
| p05 / p95 | 0.5714 | 0.1250 |
| p01 / p99 | 0.4516 | 0.3333 |
| maximum on clean | | 1.0000 |
| rejected tokens per segment, median | 24 | 5 |

Sweeping the clause on its own, with a floor on how many rejected tokens the ratio may be
computed from:

| clause >= | min rejected tokens | tp | fp | precision | recall |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 0.40 | 5 | 7,611 | 32 | 0.99581 | 0.99594 |
| **0.50** | **5** | 7,506 | 6 | **0.99920** | **0.98220** |
| 0.50 | 3 | 7,506 | 20 | 0.99734 | 0.98220 |
| 0.50 | 1 | 7,506 | 103 | 0.98646 | 0.98220 |
| **0.60** | **5** | 7,108 | 5 | **0.99930** | **0.93012** |
| 0.60 | 1 | 7,108 | 41 | 0.99426 | 0.93012 |
| 0.70 | 5 | 6,094 | 0 | 1.00000 | 0.79744 |
| 0.80 | 5 | 4,425 | 0 | 1.00000 | 0.57904 |

**The third clause alone, at 0.50 with a floor of 5 rejected tokens, reaches precision
0.99920 and recall 0.98220** - against recall 0.53088 for the rule's full conjunction.
The clean maximum of 1.0000 is entirely an artefact of short segments: a clean paragraph
with one rejected token that happens to restore gives a ratio of 1.0, which is why the
floor matters (it takes false positives from 103 to 6 at no recall cost at all).

This changes the shape of the proposal. Rather than relaxing the signature clauses on
their own - which cannot be validated without foreign negatives - the restoration
clause should become the primary test with the floor added, and the signature clauses
demoted to a cheap pre-filter at `sig >= 2` (recall 0.99673, precision 1.00000 on
Hungarian negatives). The conjunction of `sig >= 2` and `clause >= 0.50, min 5` is what
is proposed, and its foreign-language behaviour rests on the restoration clause, which is
the right place for it.

### Segment-size floor

The `>= 30 alphabetic tokens` floor never fires on this data: 0.0% of the stripped
paragraphs are shorter than 30 tokens, and raising the floor to 50 tokens drops 23% of
positives from the set without changing precision. The floor is untested here and is
left alone.

**Proposed edits**

| rule | field | old -> new |
| --- | --- | --- |
| `REP-DIAC-001` | Conditions, Stripped when | `accented-letter share among letters < 0.5%` -> `< 0.2%` |
| `REP-DIAC-001` | Conditions, Stripped when | `>= 5 tokens from the accent-free Hungarian function-word signature` -> `>= 2 distinct tokens from ...` (a pre-filter; the restoration clause is the test) |
| `REP-DIAC-001` | Conditions, Stripped when | `the stripped tokens es, mar, igy, utan, ugy, tobb, kozott (rejected or rare as written) together occur at least twice` -> drop the clause; it costs 38 points of recall and adds no precision |
| `REP-DIAC-001` | Conditions, Stripped when | `>= 60% of tokens rejected as written become accepted after the best restoration` -> `>= 50%, computed only when at least 5 tokens are rejected as written` |
| `REP-DIAC-001` | Baselines | `43.16% of tokens ... 6.59% of characters ... 48% of word tokens (approximate)` -> the measured figures of section 1 |
| `REP-DIAC-001` | Validation | add: `measured on 7,642 stripped / 19,757 clean paragraphs. Current conjunction: precision 1.00000, recall 0.53088. Proposed (sig >= 2 and restoration clause >= 0.50 over >= 5 rejected tokens): precision 0.99920, recall 0.98220. No foreign negative set was available, so the exclusion of English, Dutch and Latin rests on the restoration clause by argument, not by measurement.` |

---

## 4. REP-DIAC-002 - loss-class detection

**Decides**: which letters are candidates for restoration. A wrong class either lets the
restorer rewrite letters it should not touch (class C asserted on clean text would
license `ö` -> `ő` everywhere) or withholds candidates it needs. Target precision
**>= 0.995**.

**Evaluation set**: 13,531 gold segments of > 400 characters, each damaged with the
exact map the rulebook gives for classes B, C, D and E, and each also kept clean as its
own negative. Injection, not real data: no labelled partially-stripped corpus exists
here (this is `OQ-10-006`'s "labelled sample from forum/SMS corpora", still missing).

### The family-count floor is the binding clause, and it is much more binding than it looks

The rule says "computed over >= 200 letters of the affected families" and "detection
uses surviving-letter ratios only when their counts are >= 20". Measured family density
per letter of running text, and the text length each floor implies:

| family | density per letter | letters for 20 | letters for 200 |
| --- | ---: | ---: | ---: |
| {ö, ő} | 0.01950 | 1,026 | 10,257 |
| {ü, ű} | 0.00686 | 2,913 | 29,133 |
| {a, á} | 0.12500 | 160 | 1,600 |
| {e, é} | 0.13660 | 146 | 1,464 |
| {i, í} | 0.04762 | 420 | 4,200 |

So classes B and C need about **10,000 letters (1,600 words)** of text before their
`>= 200` condition can be evaluated, and the `ű` sub-test needs **29,000 letters
(4,500 words)**. On the 13,531 test segments the `>= 20` floor alone leaves only 1,059
segments (7.8%) decidable for {ö, ő} and 37 (0.3%) for {ü, ű}. **The `share(ű) < 0.02`
clause of class B is effectively never evaluable** and should be stated as optional.

### Class E is the precision failure

`share(í) < 0.02`, negatives are the same segments undamaged:

| family floor | t(í) | tp | fp | precision | recall on decidable | decidable |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 0.02 | 13,531 | 1,121 | 0.9235 | 1.000 | 13,531 |
| 10 | 0.02 | 13,322 | 1,056 | 0.9266 | 1.000 | 13,322 |
| **20** | **0.02** | 9,291 | 452 | **0.9536** | 1.000 | 9,291 |
| 50 | 0.01 | 439 | 2 | **0.9955** | 1.000 | 439 |
| 50 | 0.02 | 439 | 5 | 0.9887 | 1.000 | 439 |

At the rulebook's own settings (floor 20, threshold 0.02) **452 of the 9,291 decidable
clean segments - 4.9% - are labelled class E**, which is the 0.9536 precision. The cause is in section 1's table: `í` is only 13.66%
of its family and the family is thin, so a 400-character clean paragraph routinely
contains zero `í`. Only a floor of 50 {i, í} - about 1,050 letters, 165 words - and a
threshold of 0.01 reach 0.9955. Note 439 decidable segments is under 300 per fold, so
the 0.9955 figure is **provisional**.

### Classes B, C, D

| class | condition | floor | tp | fp | precision | decidable of 13,531 |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| B | share(ő) < 0.02, share(ö) > 0.90 | 10 | 2,073 | 23 | 0.9890 | 2,073 |
| B | " | 20 | 79 | 1 | 0.9875 | 79 |
| C | share(ő) < 0.02, share(ö) > 0.40 | 10 | 7,234 | 23 | 0.9968 | 7,234 |
| C | " | 20 | 1,059 | 1 | 0.9991 | 1,059 |
| D | share(á) < 0.03 and share(é) < 0.03 | 20 | 13,526 | 0 | **1.0000** | 13,527 |
| D | share(á) < 0.05 and share(é) < 0.05 | 20 | 13,526 | 0 | 1.0000 | 13,527 |

Class D at the rulebook's 0.03 is exact: precision 1.0000, recall 0.9996, on 13,527
decidable segments. **Confirmed, no change.** It is the only class whose families are
dense enough to decide on a paragraph.

Class C at floor 20 reaches 0.9991 and clears target; the `share(ö) > 0.40` clause,
swept over 0.40 / 0.70 / 0.90, changes nothing at all, because class C damage drives
`share(ö)` to exactly 1.0. It is redundant given `share(ő) < 0.02` and is retained only
as a guard against class B being read as C.

Class B at floor 20 reaches 0.9875 on 79 decidable segments - under 300, **provisional**,
and below target. Class B is the hardest case because `ő` -> `o` leaves the `ö`/`ő`
family smaller as well as skewed, so the single surviving ratio carries all the weight.

**Proposed edits**

| rule | field | old -> new |
| --- | --- | --- |
| `REP-DIAC-002` | Conditions | `computed over >= 200 letters of the affected families` -> `computed over >= 200 letters of the affected families, which for {o-umlaut, o-double-acute} means about 10,000 letters (1,600 words) of running text and for {u-umlaut, u-double-acute} about 29,000 (4,500 words): classes B and C are document-level tests, never segment-level` |
| `REP-DIAC-002` | class B Detection | `share(ő) < 0.02 and share(ű) < 0.02` -> `share(ő) < 0.02; the share(ű) test is evaluable only above 29,000 letters and is optional` |
| `REP-DIAC-002` | class E Detection | `share(í) < 0.02` -> `share(í) < 0.01 with at least 50 letters of the {i, í} family (about 165 words); at the previous 0.02 with a floor of 20 the measured precision is 0.954: 452 of 9,291 decidable clean segments are labelled class E` |
| `REP-DIAC-002` | Exceptions | `detection uses surviving-letter ratios only when their counts are >= 20` -> `... only when their counts are >= 50 for the {i, í} and {o-umlaut, o-double-acute} families` |
| `REP-DIAC-002` | Validation | add the measured confusion figures: `D 1.0000 / 0.9996 on 13,527 segments; C 0.9991 on 1,059; B 0.9875 on 79 (provisional); E 0.9955 on 439 at the proposed floor (provisional)` |
| `REP-DIAC-002` | Confidence | `tendency` -> `tendency` (class D `measured`) |

---

## 5. REP-SPACE-001 - letterspacing guards (OQ-10-011)

**The open question, exactly**: "Effect on join precision of adding `ó` and `ő` to the
one-letter-word guard set (`a á e é s`)."

**Decides**: whether a run of single spaced letters is joined into one word. A false
positive fuses real one-letter words (`és a s e` -> `ésase`); a false negative leaves a
letterspaced heading unjoined and unsearchable. Target precision >= 0.995.

**Evaluation set**: 1,213 `pdf` pairs whose damage patterns include `letters` (2,373
letterspaced runs) as positives, against all 19,757 gold paragraphs as negatives. The
letterspacing generator is `damage.letter_spaced`, which is what `REP-NOISE-004`
describes. Oracle: the 250k list.

The evidence clause is the rulebook's: join if the joined form is oracle-accepted, or
the run is all capitals, or the run length >= N; guard A overrides.

| guard set | min run length | tp | fp | fn | precision | recall | F1 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| **a á e é s** | **3** | 2,372 | 8 | 1 | **0.99664** | **0.99958** | **0.99811** |
| a á e é s | 4 | 2,365 | 8 | 8 | 0.99663 | 0.99663 | 0.99663 |
| a á e é s | 5 | 2,313 | 8 | 60 | 0.99655 | 0.97472 | 0.98551 |
| a á e é s | 6 | 2,253 | 8 | 120 | 0.99646 | 0.94943 | 0.97238 |
| **a á e é s ó ő** | **3** | 2,371 | 8 | 2 | **0.99664** | **0.99916** | 0.99790 |
| a á e é s ó ő | 4 | 2,364 | 8 | 9 | 0.99663 | 0.99621 | 0.99642 |
| a á e é s ó ő | 5 | 2,312 | 8 | 61 | 0.99655 | 0.97429 | 0.98530 |
| a á e é s ó ő | 6 | 2,252 | 8 | 121 | 0.99646 | 0.94901 | 0.97216 |

**Answer to OQ-10-011: adding `ó` and `ő` has no effect on join precision and a small
negative effect on recall.** It protects zero additional clean runs (0 of the 8 false
positives are covered by either guard set) and it loses one further true run per 2,373.
The precision column is identical to five decimal places. The extension should be
reverted.

The false-positive floor of 8 is the same 8 runs under every setting, and they are what
the whole guard exists for. Their inventory over 19,757 clean paragraphs:

| clean run | n | what it is |
| --- | ---: | --- |
| `abcde` | 4 | alphabet listing |
| `abc` | 2 | alphabet listing |
| `abcd` | 1 | alphabet listing |
| `árveréstkitűzi` | 1 | a genuine letterspaced heading already in gold |

Not one of the 8 is a run of Hungarian one-letter words. **Guard A does not fire on real
text at all**: the case it was written for (`és a s e nem`) does not occur in 19,757
paragraphs of eight registers. It should be kept, because its cost is one run in 2,373
and the damage it prevents is severe when it does occur, but the rulebook should stop
presenting it as a load-bearing clause.

The run-length clause is a different matter: at length >= 4 (the current
`PC-KB SPACED_RUN` value) recall is 0.99663; at >= 3 it is 0.99958, with precision
unchanged at 0.9966. The extra length was buying nothing.

**Caveat on the 8 negatives.** Precision here is bounded by a negative base of 8 runs,
so the precision *differences* between settings are measured on essentially no data and
are reported as "identical" rather than as a ranking. The recall differences rest on
2,373 runs and are solid. The conclusion for `OQ-10-011` rests on the recall side and on
the demonstration that neither guard set covers any observed clean run.

**Proposed edits**

| rule | field | old -> new |
| --- | --- | --- |
| `REP-SPACE-001` | Conditions, Guard A | `a á e é s ó ő (articles, conjunction s, interjections á é ó, pronoun ő)` -> `a á e é s (articles, conjunction s, interjection á é); ó and ő are not included: they protect no observed clean run and cost recall` |
| `REP-SPACE-001` | Conditions, Evidence | `or the run length >= 4` -> `or the run length >= 3` |
| `REP-SPACE-001` | Sources | `one-letter set a á e é s, extended here with ó ő` -> drop the extension clause |
| `REP-SPACE-001` | Validation | add: `measured on 2,373 injected runs against 19,757 clean paragraphs: precision 0.99664, recall 0.99958 with guard {a á e é s} and length >= 3; guard A fires on 0 of the 8 clean runs found` |
| `REP-QUAL-006` | Output | `spaced_runs = runs of >= 4 single-letter tokens` -> `>= 3 single-letter tokens` |

---

## 6. REP-SPACE-003 - fragment-rejoin document trigger

**Decides**: whether a document's text layer is treated as syllable-fragmented at all.
This is the gate in front of a dynamic programme that fuses adjacent tokens, and the
rule itself records that "an audit found 84% of joins in triggered documents were such
merges when evidence was not required". A false positive therefore routes clean prose
into a word-fusing pass; a false negative leaves a wrecked PDF layer unrepaired. Target
precision >= 0.995.

**Evaluation set**: 1,227 `pdf` pairs whose patterns include `syllables`, against 19,757
gold paragraphs; and the same aggregated into pseudo-documents of >= 300 words by
concatenating consecutive same-source paragraphs (127 damaged, 1,978 clean).

Feature: `short_word_share` = lowercase alphabetic tokens of length <= 3 over all
alphabetic tokens.

### Distribution

| granularity | clean median | clean p95 | clean p99 | clean max | damaged min | damaged p05 | damaged median |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| paragraph (n = 19,757 / 1,227) | 0.2344 | 0.3333 | 0.3800 | 0.5769 | 0.2174 | 0.5000 | 0.7391 |
| document >= 300w (n = 1,978 / 127) | 0.2388 | 0.2877 | 0.3045 | **0.3333** | **0.5992** | 0.6731 | 0.7428 |

At document granularity - which is what the rule specifies - the two classes are
separated by a clear gap from 0.333 to 0.599, and **the rulebook's trigger of 0.30 sits
below the clean maximum.**

### Sweep, document granularity

| trigger | tp | fp | precision | recall | F1 |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 0.25 | 127 | 718 | 0.1503 | 1.0000 | 0.2613 |
| **0.30** | 127 | 32 | **0.7987** | 1.0000 | 0.8881 |
| **0.35** | 127 | 0 | **1.0000** | **1.0000** | **1.0000** |
| 0.40 | 127 | 0 | 1.0000 | 1.0000 | 1.0000 |
| 0.50 | 127 | 0 | 1.0000 | 1.0000 | 1.0000 |

**The trigger of 0.30 has a measured precision of 0.799: one clean document in
sixty-two is sent into the fragment-rejoin pass.** 0.35 is exact on both sides and is
the proposed value; anything in [0.34, 0.59] is equivalent on this data, and 0.35 is
chosen as the low end of that band so that lightly fragmented layers are still caught.

At paragraph granularity 0.30 is far worse (precision 0.299, 2,857 false positives of
19,757) and the equivalent operating point is 0.45 (precision 0.9958). That is worth
recording because implementations that apply the trigger to a block rather than a
document will need the higher value.

The rule's own prose figures are also off: "healthy Hungarian about 0.20, damaged layers
above 0.40". Measured healthy is 0.239 at both granularities and damaged is 0.743
(minimum 0.599 at document level).

**Evidence size**: 1,978 clean documents is ample; **127 damaged documents is under 300
and the damaged side is provisional**, though the decisive number - that clean documents
reach 0.333 - rests on the 1,978.

### The run-evidence clause could not be calibrated

`FRAG_RUN_SHARE 0.25` and `FRAG_RUN_MIN_BAD 2` ask for a run in which at least 2 tokens,
and at least 25%, are lowercase, of length <= 4, and **rejected by the oracle**. With
the 250k list as the oracle the clause fires on only 4% of damaged sentences, and its
precision never exceeds 0.83 at any swept setting (9,905 damaged runs against 109,180
clean runs). Two reasons, both real:

1. Hungarian syllables are largely themselves words (`a`, `az`, `el`, `le`, `de`, `meg`,
   `ki`, `be`, `fel`), so a lexicon-based oracle accepts most fragments. Only a
   morphological analyser that can say "this is not a possible standalone word form in
   this position" can reject them.
2. The damage generator spaces a *stretch* of sentences, so undamaged sentences inside a
   damaged document dilute any per-sentence measure.

This clause is therefore **not calibrated**. It needs the analyser as the oracle, and a
pair set with per-sentence damage labels.

**Proposed edits**

| rule | field | old -> new |
| --- | --- | --- |
| `REP-SPACE-003` | Conditions, Document trigger | `share of lowercase alphabetic tokens of length <= 3 among all alphabetic tokens >= 0.30 (healthy Hungarian about 0.20, damaged layers above 0.40)` -> `... >= 0.35 measured over a whole document of at least 300 words (healthy Hungarian 0.239 median, 0.333 maximum over 1,978 documents; fragmented layers 0.743 median, 0.599 minimum over 127). Applied per block instead of per document the equivalent value is 0.45.` |
| `REP-SPACE-003` | Validation | add: `at 0.35, precision 1.0000 and recall 1.0000 on 127 fragmented and 1,978 clean documents; at the previous 0.30, precision 0.799` |
| `REP-SPACE-003` | Confidence | `tendency` -> `tendency` (trigger `measured`, run-evidence clause still unmeasured) |
| `REP-QUAL-006` | Output | `short_word_share = ... (healthy about 0.20; trigger 0.30; damaged layers > 0.40)` -> `(healthy 0.24; trigger 0.35 per document; damaged layers > 0.55)` |

---

## 7. REP-ENC-016 - plausibility score and accept margin (OQ-10-002)

**Decides**: whether a decoding hypothesis is applied to a run. A false positive
re-encodes text through the wrong code page and destroys it irrecoverably; a false
negative leaves mojibake in place, which is visible and repairable later. This is the
most precision-critical threshold in the chapter. Target precision >= 0.995.

**Evaluation set**: 3,080 `mojibake` pairs (positives), 19,757 gold paragraphs
(negatives). The score is implemented exactly as `REP-ENC-016` states it, including the
fixed ordering constraint (reject any candidate whose accepted-word share decreases).
Hypotheses: UTF-8 as cp1252, UTF-8 as Latin-1, Latin-2 as Latin-1, Latin-2 as cp1252,
font shift, HTML entities - the inverses of the damage this corpus carries. A true
positive requires **exact recovery of the whole paragraph**, which is a strict measure.

### What the margin actually protects against

| quantity | value |
| --- | ---: |
| gain on damaged text, median | 1.426 |
| gain on damaged text, p05 | 0.062 |
| gain on clean text, **maximum over 19,757 segments** | **0.00107** |
| share of damaged pairs where some hypothesis recovers gold exactly | 0.9526 |

On clean text almost no hypothesis is even admissible (strict byte re-encoding fails),
and where one is, the best gain ever observed is 0.001. **The accept margin is not
protecting clean text; it is arbitrating between competing wrong hypotheses on damaged
text.** That is a useful correction to how the rule reads.

### Sweep

`fp wrong decode` = a hypothesis was applied and the result is not gold;
`fp clean text touched` = a clean segment was rewritten.

| margin | tp exact | fp wrong | fp clean | fn | precision | recall | F1 |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0.00 | 2,934 | 124 | 1 | 146 | 0.9591 | 0.9526 | 0.9559 |
| 0.10 | 2,648 | 94 | 0 | 432 | 0.9657 | 0.8597 | 0.9097 |
| **0.25** | 1,940 | 16 | 0 | 1,140 | **0.9918** | 0.6299 | 0.7705 |
| 0.40 | 1,676 | 5 | 0 | 1,404 | 0.9970 | 0.5442 | 0.7041 |
| **0.50** | 1,617 | 0 | 0 | 1,463 | **1.0000** | 0.5250 | 0.6885 |
| 0.75 | 1,576 | 0 | 0 | 1,504 | 1.0000 | 0.5117 | 0.6770 |
| 1.00 | 1,566 | 0 | 0 | 1,514 | 1.0000 | 0.5084 | 0.6741 |
| 1.50 | 1,484 | 0 | 0 | 1,596 | 1.0000 | 0.4818 | 0.6503 |
| 2.00 | 658 | 0 | 0 | 2,422 | 1.0000 | 0.2136 | 0.3521 |

The rulebook's 0.25 measures **0.9918**, below target. Raising the margin to 0.50 is the
obvious fix - it is the first swept point at 1.0000 - but it costs recall, from 0.630 to
0.525. The next two subsections show it is also the wrong fix. Note that recall here is
against exact paragraph recovery: the pair generator composes up to three patterns per
pair, so many "misses" are pairs no single hypothesis can invert, and the 0.9526 ceiling
row bounds what is reachable.

### Per hypothesis, at margin 0.25

| hypothesis | tp | fp | precision |
| --- | ---: | ---: | ---: |
| entities | 837 | 0 | 1.0000 |
| utf8_as_cp1252 | 580 | 0 | 1.0000 |
| utf8_as_latin1 | 202 | 0 | 1.0000 |
| latin2_as_latin1 | 164 | 1 | 0.9939 |
| font_shift | 157 | 0 | 1.0000 |
| **latin2_as_cp1252** | **0** | **15** | **0.0000** |

All but one of the false positives at 0.25 come from a single hypothesis. Inspecting
them changes the diagnosis: `latin2_as_cp1252` is not choosing the wrong code page. It
gets the Hungarian letters right (`zûrbõl` -> `zűrből`, `hétfõ` -> `hétfő`) and fails
exact match only on the typographic quotation marks around them, which cp1252 encodes
into the C1 byte range (`„` = 0x84, `”` = 0x94) where ISO-8859-2 has control characters.
The output is correct Hungarian with two C1 controls embedded in it.

### The C1 penalty is the real defect, and it is a weight, not a threshold

`REP-ENC-016` already penalises C1 controls, at `- 5.0 * c1_control_rate`. But the term
is a *rate*: two stray controls in a 150-character run cost 0.067, which cannot outweigh
a letter gain of 0.33. Hungarian prose never contains U+0080..U+009F, so their presence
is not a matter of degree - a candidate that produces one is wrong by construction.
Making it an admissibility veto instead of a soft penalty:

| margin | soft penalty (current) | | | hard C1 veto | | |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| | precision | recall | F1 | precision | recall | F1 |
| **0.00** | 0.95914 | 0.95260 | 0.95586 | **0.99864** | **0.95260** | **0.97507** |
| **0.10** | 0.96572 | 0.85974 | 0.90965 | **0.99887** | **0.85974** | 0.92410 |
| 0.25 | 0.99182 | 0.62987 | 0.77045 | 0.99948 | 0.62987 | 0.77275 |
| 0.40 | 0.99703 | 0.54416 | 0.70405 | 1.00000 | 0.54416 | 0.70479 |
| 0.50 | 1.00000 | 0.52500 | 0.68852 | 1.00000 | 0.52500 | 0.68852 |
| 1.00 | 1.00000 | 0.50844 | 0.67413 | 1.00000 | 0.50844 | 0.67413 |

**With the veto, the margin can be lowered rather than raised.** At margin 0.10 the
veto gives precision 0.99887 and recall 0.85974, against 0.99182 / 0.62987 at the
current setting and 1.00000 / 0.52500 at the margin-only fix of 0.50. That is the same
precision target met with **64% more recall than raising the margin**, and it is reached
by correcting a weight the rule already has rather than by trading recall away.

The proposed operating point is therefore **C1 veto + margin 0.10**: precision 0.99887,
recall 0.85974, on 3,080 damaged and 19,757 clean segments. Margin 0.00 with the veto
has the best F1 (0.97507) and still clears the 0.995 target at 0.99864; 0.10 is chosen
over it only because a strictly positive margin is a safer default for hypothesis
families this evaluation does not contain.

### The remaining weights could not be refit

`REP-ENC-016`'s Exceptions field says "weights are initial values to be tuned on
REP-NOISE-002 pairs". They cannot be tuned on this data, and the reason is the
maximum-gain-on-clean-text figure above: the score is never asked to make a close call.
Admissibility (strict byte re-encoding) plus the accepted-word ordering constraint
already separate the classes; the five numeric weights only have to preserve the sign of
a gap that averages 1.43 against a noise floor of 0.001. A weight refit needs pairs where
two *admissible* hypotheses both raise the accepted-word share - mixed-encoding documents
of the `REP-ENC-015` kind - and the pair file has none.

**Proposed edits**

| rule | field | old -> new |
| --- | --- | --- |
| `REP-ENC-016` | Output, score | `- 5.0 * c1_control_rate` -> move the term out of the score: `a candidate containing any character in U+0080..U+009F is inadmissible` |
| `REP-ENC-016` | Output, acceptance | `Accept a candidate when score(candidate) - score(input) >= 0.25` -> `>= 0.10`, given the C1 veto above |
| `REP-ENC-016` | Exceptions | `weights are initial values to be tuned on REP-NOISE-002 pairs` -> `the C1 term is corrected to a veto (see Validation). The remaining four weights are not identifiable on single-damage pairs: admissibility and the ordering constraint separate the classes on their own (median gain 1.43, maximum gain over 19,757 clean segments 0.001). Refitting them needs mixed-encoding documents where two admissible hypotheses both raise the accepted-word share.` |
| `REP-ENC-016` | Validation | add: `measured on 3,080 mojibake pairs against 19,757 clean segments. With the C1 veto: precision 0.99887 / recall 0.85974 at margin 0.10, 0.99864 / 0.95260 at margin 0.00. Without it: 0.96572 / 0.85974 at 0.10, 0.99182 / 0.62987 at 0.25, 1.00000 / 0.52500 at 0.50. Recall is against exact whole-paragraph recovery and is capped at 0.9526 by composed damage.` |
| `REP-ENC-016` | Confidence | `tendency` -> `measured` (C1 veto and margin), `tendency` (remaining weights) |
| `REP-ENC-015` | Conditions, admissibility | `A hypothesis is admissible for a run only if its byte re-encoding is total ... and the decode is strict` -> add `and the decoded run contains no C1 control (U+0080..U+009F)`; a strict ISO-8859-2 decode accepts C1 bytes as controls, so strictness alone does not exclude them, and this is the whole of the residual false-positive mass for `{ISO-8859-2 as windows-1252}` (15 of 16 wrong decodes at margin 0.25 in 3,080 pairs) |

---

## 8. REP-ENC-014 - seven-bit Hungarian conventions (OQ-10-002, second half)

**Decides**: whether a block is decoded from a 7-bit convention. A false positive
rewrites apostrophes, inch marks and emoticons into accented letters throughout a clean
document; a false negative leaves `a'rvi'ztu"ro"` unreadable. Target precision >= 0.995.

**Evaluation set**: injection. 4,000 gold segments of >= 30 tokens per convention,
damaged with the flying-accent, 123-notation and ISO 646-HU tables the rule names, and
the same 4,000 segments clean as negatives. The negatives carry the real apostrophe,
quotation-mark and bracket traffic of eight registers, which is the confusion the rule's
Exceptions field is about.

Marked-token rate = marked tokens / word tokens. The rulebook's "at least one per 20
words" is a rate of 0.05.

### Flying accent

| quantity | value |
| --- | ---: |
| marked rate, damaged, median | 0.4324 |
| marked rate, damaged, p05 | 0.3131 |
| marked rate, clean, p95 | 0.0099 |
| marked rate, clean, **maximum** | **0.0638** |

| rate >= | acceptance >= | tp | fp | precision | recall |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 0.02 | 0.8 | 3,941 | 60 | 0.98500 | 0.98525 |
| **0.05** | **0.8** | 3,941 | 4 | **0.99899** | 0.98525 |
| **0.10** | **0.8** | 3,941 | 0 | **1.00000** | **0.98525** |
| 0.10 | 0.5 | 4,000 | 0 | 1.00000 | 1.00000 |
| 0.20 | 0.8 | 3,941 | 0 | 1.00000 | 0.98525 |
| 0.30 | 0.8 | 3,819 | 0 | 1.00000 | 0.95475 |

**The clean maximum (0.0638) is above the rulebook's threshold (0.05).** One clean
Hungarian segment in a thousand carries enough apostrophes and inch marks to clear
"one per 20 words". At one per 10 words the false positives go to zero with no recall
cost at all, because real flying-accent text runs at 0.31 and above.

### 123 notation

Clean maximum 0.0263, damaged p05 0.2651. Rate >= 0.05 gives precision 1.00000 and
recall 1.00000 (3,997 positives). The rule currently says only "at a consistent rate",
which is not a number; 0.05 is the measured value and 0.10 is equally exact.

### ISO 646-HU

Clean maximum 0.0513, damaged p05 0.1981. Rate >= 0.05 gives precision 0.99975
(1 false positive of 4,000); rate >= 0.10 gives 1.00000 at recall 0.99975. The rule
currently gives no rate at all for this convention.

### The acceptance clause

`decoded words oracle-accepted at >= 0.8` behaves identically on both classes
(positives p05 0.8409 median 0.9375; negatives median 0.9375), because the decode is a
no-op on clean text. It costs 1.5% recall and moves precision by less than 0.001 at
every rate setting. It is cheap insurance against a corpus this evaluation does not
contain (foreign text with heavy apostrophe use) and is kept, but the rulebook should
stop presenting it as the discriminating clause - the rate is.

**Proposed edits**

| rule | field | old -> new |
| --- | --- | --- |
| `REP-ENC-014` | Conditions, Flying accent | `at a rate of at least one per 20 words` -> `at a rate of at least one per 10 words` |
| `REP-ENC-014` | Conditions, Digit notation | `at a consistent rate` -> `at a rate of at least one per 20 words` |
| `REP-ENC-014` | Conditions, ISO 646-HU | add `at a rate of at least one per 10 words` |
| `REP-ENC-014` | Confidence | `tendency (detection thresholds proposed here, unmeasured)` -> `tendency (detection thresholds measured by injection on 4,000 segments per convention: precision 1.00000 at the proposed rates, recall 0.985 with the 0.8 acceptance clause and 1.000 without it)` |

---

## 9. PH-HARM-012 and PH-HARM-014 - harmony vacillation priors (OQ-02-001, OQ-02-002, OQ-02-015)

**Decides**: the per-class prior for back vs front suffix selection on stems whose last
vowel is neutral, and whether the consonant-context effects of `PH-HARM-014` are enabled
by default. A wrong prior generates the wrong dative for a whole class of stems.

**Evaluation set**: `local/research-lit/HungarianFromGoogleRevised.txt` (Latin-2, 8,915
rows, the hand-curated list) and `HungarianFromGoogle.txt` (10,974 rows, uncurated).
Classification is mechanical from the grapheme column: back `a á o ó u ú`, neutral
`i í e é`, front rounded `ö ő ü ű`. Stems containing a front rounded vowel are excluded:
1,616 of 10,974 (uncurated), 1,241 of 8,915 (curated).

### Provenance, first

Every figure currently in `PH-HARM-012` and `PH-HARM-014` was reproduced exactly, and the
two rules turn out to rest on different files and different statistics:

- `PH-HARM-012`'s table is the **uncurated** file, the Hayes-Londe 97% rule, and its
  "backness index" column is the **unweighted type mean** of the -nak fraction.
  Reproduced to three decimals for all 13 rows, and the stem counts match.
- `PH-HARM-014`'s figures are the **curated** file, and its "% take the predicted front
  value" is the share of zone stems whose -nak fraction is below 0.5. Reproduced:
  CC 168 stems 0.940 (rulebook 168 / 0.938); bilabial 41 / 0.878 (41 / 0.880); sibilant
  138 / 0.652 (136 / 0.651); coronal sonorant 183 / 0.667 (182 / 0.642).

The arithmetic is sound. Two substantive things are still wrong.

### Per-class -nak rates, uncurated file (the better-powered sample)

| class | stems | tokens | token -nak | 95% Wilson | type -nak | median |
| --- | ---: | ---: | ---: | --- | ---: | ---: |
| B | 6,197 | 4,726,354 | 0.9997 | 0.9997-0.9997 | 0.9993 | 1.0000 |
| Bi | 472 | 305,738 | 0.9986 | 0.9985-0.9987 | 0.9885 | 1.0000 |
| Bí | 53 | 7,466 | 0.9618 | 0.9572-0.9659 | 0.9803 | 1.0000 |
| Bé | 117 | 51,512 | 0.9750 | 0.9737-0.9763 | 0.8492 | 1.0000 |
| Be | 116 | 48,415 | 0.0589 | 0.0568-0.0610 | 0.1045 | 0.0041 |
| BNi | 32 | 11,589 | 0.0305 | 0.0276-0.0338 | 0.2094 | 0.0020 |
| BNí | 8 | 4,876 | 0.6993 | 0.6863-0.7121 | 0.3576 | 0.2625 |
| BNé | 16 | 2,398 | 0.6343 | 0.6148-0.6533 | 0.3161 | 0.0022 |
| BNe | 23 | 6,929 | 0.0004 | 0.0001-0.0013 | 0.0002 | 0.0000 |
| BNN (all) | 79 | 25,792 | 0.2050 | 0.2001-0.2100 | 0.1851 | 0.0000 |
| N | 296 | 666,188 | 0.0341 | 0.0337-0.0345 | 0.0784 | 0.0000 |
| NN | 2,028 | 2,736,941 | 0.0060 | 0.0059-0.0061 | 0.0014 | 0.0000 |

**Every class except `NN` and `B` rests on fewer than 300 stems and is provisional.** The
two-neutral classes are the worst: `BNí` is 8 stems, `BNé` 16, `BNe` 23, `BNi` 32. The
Wilson intervals are on token counts and are therefore far narrower than the real
uncertainty, which is dominated by a handful of items; `BNí` = 0.70 must not be read as a
stable prior. The type and token columns diverge badly exactly where the classes are
smallest (`Bé` 0.975 token vs 0.849 type; `BNi` 0.031 vs 0.209; `BNé` 0.634 vs 0.316),
because token weighting is carried by a few high-frequency stems.

### The consonant effects (OQ-02-001, OQ-02-002): sibilant and coronal sonorant are not there

Zone stems (`BN`, `BNN`, `N`), share whose dominant value is front, curated file:

| stem-final consonant | stems | front-dominant share |
| --- | ---: | ---: |
| CC cluster (geminates included) | 168 | 0.940 |
| bilabial p b m | 41 | 0.878 |
| **other single C, no constraint claimed** | **231** | **0.671** |
| coronal sonorant n ny l r | 183 | 0.667 |
| sibilant sz z s zs c cs dzs | 138 | 0.652 |
| vowel-final | 106 | 0.208 |
| all zone stems | 699 | 0.608 |

**The sibilant (0.652) and coronal-sonorant (0.667) classes are indistinguishable from
stems ending in any other single consonant (0.671).** The apparent effect in the rulebook
is an artefact of the pooled baseline, which mixes in 106 vowel-final zone stems at 0.208
and so reads much lower than the correct comparison class. The CC effect (0.940) and the
bilabial effect (0.878) are real and large against 0.671; bilabial rests on 41 stems and
is provisional.

`PH-HARM-014`'s stated baseline is also mislabelled: "40.3% front" for 692 zone stems is
39.2% **back** for 699 zone stems, i.e. 60.8% front. This is the single largest
correction in the chapter-02 material, because the sign of the baseline is what the
consonant effects are measured against.

Within-class contrast, curated file, confirming Forró (2013) as the rulebook reports it:

| class | bucket | stems | token -nak | type -nak |
| --- | --- | ---: | ---: | ---: |
| Be | single C | 69 | 0.1291 | 0.1681 |
| Be | CC | 44 | 0.0032 | 0.0095 |
| BNN | single C | 49 | 0.1905 | 0.1836 |
| BNN | CC | 9 | 0.0000 | 0.0000 |
| Bi | single C | 98 | 0.9975 | 0.9713 |
| Bi | CC | 9 | 0.9998 | 0.9956 |

The CC effect is strong in disyllabic `e`-final stems and absent at ceiling in the
high-vowel classes. `PH-HARM-014`'s default of "enable CC only" is confirmed.

**Proposed edits**

| rule | field | old -> new |
| --- | --- | --- |
| `PH-HARM-014` | Output, zone baseline | `baseline for all 692 zone stems: 40.3% front` -> `baseline for all 699 zone stems (BN, BNN, N): 39.2% back / 60.8% front` |
| `PH-HARM-014` | Output, sibilant and coronal sonorant | annotate `64.2%` and `65.1%` as `not distinguishable from the 67.1% of zone stems ending in any other single consonant (231 stems); the apparent effect is baseline contamination by 106 vowel-final zone stems` |
| `PH-HARM-014` | Output, default configuration | `enable CC only` -> unchanged, now measured: CC 0.940 and bilabial 0.878 against 0.671 |
| `PH-HARM-012` | Output table, `BNé` backness index | `0.421` (12 stems) -> `0.316` (16 stems, uncurated) or `0.172` (12 stems, curated); 0.421 could not be reproduced from either file |
| `PH-HARM-012` | Output table, `BNN` backness index | `0.206` -> `0.185` (79 stems, uncurated) |
| `PH-HARM-012` | Output table, height-count claim | `BNé 0.421 > BNí 0.358 > BNi 0.223` -> `BNí 0.358 > BNé 0.316 > BNi 0.209`; the non-monotonicity survives with a different ordering, and on 8/16/32 stems it is not evidence of anything - demote the within-BNN height ordering to `not established` |
| `PH-HARM-012` | Output table | add a token-weighted -nak column beside the type-weighted backness index; they diverge by more than 0.2 in `Bé`, `BNi`, `BNé` and `N` |
| `PH-HARM-012` | Output table, `NN` row total | `939 (as printed) / 933` -> `933`, with `exactly two neutral vowels; a further 1,095 all-neutral stems have 3-4 neutral vowels and are not in this row (778 with 3, 317 with 4)` |
| `PH-HARM-012` | Output table, `R` row total | `698` -> `1,517` stems whose last non-neutral vowel is front rounded (uncurated file); 698 could not be reproduced |
| `PH-HARM-022` | Output, foreign `p_back` | `no published weight; estimate from corpus` -> `direction confirmed, magnitude not measurable: only 15 zone stems in the Hayes list end in a Greek-Latin piece (-méter, -gén, -lízis, -tézis, -gráf, -lógia), type-mean -nak 0.023 against 0.118 for the other 2,380 zone + N/NN stems; 0 stems begin with bio-, hidro- or hetero-. Mark provisional; do not hard-code a weight.` |
| `PH-HARM-013` | weight table | no change proposed: re-fitting the 13 MaxEnt weights is a separate exercise and this measurement does not constrain them individually |

---

## 10. WS-DIG-008 - doubled-digraph heuristic precision (OQ-01-010)

**The open question, exactly**: "Precision of the doubled-digraph heuristics (`nny`, `ssz`,
`ggy`, `zsz`, `ccs`) measured on 100-type samples only. Full-type-list measurement with
hunmorph-foma."

**Decides**: step 8 of `WS-DIG-008`, the fall-through that reads `ccs`, `nny`, `ssz` as
long letters when no morphological analysis is available. A false positive treats a
compound boundary as one long letter, which corrupts syllabification, hyphenation and
collation; a false negative splits a real long letter.

**Evaluation set**: the **full type list** of Webcorpus 2 (36,936,726 rows, no sampling),
units being distinct lowercased forms with summed frequency >= 5. The automatic (a)/(b)
classifier was validated by hand on 50 random types per digraph, 400 types total:
**397/400 agreement (0.9925)**, and all three disagreements are false boundaries, so the
precisions below are if anything slightly pessimistic.

### Dictionary-valid types (the figure to quote)

| digraph | types | tokens | precision, type-weighted | 95% Wilson | precision, token-weighted | rulebook |
| --- | ---: | ---: | ---: | --- | ---: | --- |
| tty | 1,780 | 418,967 | 1.0000 | 0.9978-1.0000 | 1.0000 | (not given) |
| lly | 812 | 2,040,072 | 0.9988 | 0.9931-0.9998 | 1.0000 | (not given) |
| ccs | 1,222 | 1,459,955 | 0.9894 | 0.9819-0.9938 | 0.9997 | ~94% (134 types) |
| ddz | 53 | 6,694 | 0.9811 | 0.9006-0.9967 | 0.9987 | (not given) |
| nny | 5,135 | 17,965,700 | 0.9646 | 0.9591-0.9693 | 0.9941 | 96% (97 sampled) |
| ssz | 23,273 | 40,709,187 | 0.9538 | 0.9510-0.9564 | 0.9913 | 91% (97 sampled) |
| **ggy** | 1,724 | 2,667,904 | **0.4455** | 0.4222-0.4690 | 0.2523 | 194 of 283 |
| **zsz** | 604 | 401,381 | **0.0132** | 0.0067-0.0259 | 0.0011 | 137 vs 2 types |

On unfiltered web text the same figures fall (ccs 0.908, nny 0.946, ssz 0.881, lly 0.942,
tty 0.993), and the gap is almost entirely foreign spellings (`hollywood`, `telly`,
`neuilly`, `shaggy`) and unaccented misspellings (`fröccs` written `froccs`), both of
which the dictionary rejects. `lly` is the clearest case: 0.9988 on 812 dictionary types
against 0.9424 on 6,911 raw types, with 313 foreign types accounting for the difference -
the same phenomenon `WS-DIG-006` recorded as "lly foreign 63/200".

**The measurement moves every sampled figure upward**, and by more than the sample
intervals allowed: `ssz` from 91% (97 types) to 95.4% (23,273 types), `nny` from 96% to
96.5%, `ccs` from 94% to 98.9%. The 100-type samples were pessimistic because web noise
was in the denominator.

`ggy` and `zsz` behave in the opposite direction and confirm that they belong in steps 2
and 4 of the rule, not in the step-8 default: `zsz` is `z`+`sz` 98.7% of the time (long
`zs` is written `zzs`), and `ggy` is a boundary 55.5% of the time.

Dominant false-hit families, useful for the rule's lexicon steps:
`nny` - `tizen|nyolc`, `magán|nyomozó`; `ccs` - `lánc|csörgés`, `árbóc|csúcs`,
`piac|csarnok`; `ssz` - `társ|szerkesztő`, `tojás|szám`, `varázs|szer`, `kulcs|szó`
(i.e. `-s`-final first members before `sz`-initial second members); `ggy` - `meg|`
preverb (1,839 types), `leg|` superlative (235), plus `ország|gyűlés`, `üveg|gyöngy`;
`zsz` - `víz|szint`, `gáz|szerelés`, `tűz|szünet`, plus `száz|szor`, `tíz|szer`.

**Proposed edits**

| rule | field | old -> new |
| --- | --- | --- |
| `WS-DIG-008` | step 8 | `correct in about 94% (126 of 134 non-noise types), 96% (93 of 97 sampled), 91% (88 of 97 sampled) of types respectively` -> `correct in 98.9% (ccs, 1,222 types), 96.5% (nny, 5,135 types) and 95.4% (ssz, 23,273 types) of dictionary-valid types on the full Webcorpus 2 type list; token-weighted 99.97%, 99.41%, 99.13%` |
| `WS-DIG-008` | step 8 | add `ddz` 98.1% (53 types), `lly` 99.9% (812 types), `tty` 100% (1,780 types), all token-weighted >= 99.9%; add the caveat that on unfiltered text `lly` falls to 94.2% through foreign `ll`+`y`, so `WS-DIG-009` is consulted first for names |
| `WS-DIG-008` | step 2 (`zsz`) | `(137 vs 2 types)` -> `98.7% of dictionary-valid zsz types are z+sz (596 of 604); the residual is zs+z plus gyöngyház|színű and hegyköz|szentmiklósi, which need the first-member list of step 6` |
| `WS-DIG-008` | step 4 (`g+gy`) | `covers 137 of the 194 g+gy types` -> `55.5% of dictionary-valid ggy types are g|gy boundaries (956 of 1,724), 74.8% of tokens; the meg/leg prefix rule alone covers 2,074 of them` |
| `WS-DIG-008` | step 4, exception | the `meggy` exception must cover derivatives and place names, not only inflections: measured false negatives `meggyes`, `meggypiros`, `meggylekvár`, `meggyfa`, `meggymag`, `Meggyeskovácsi`, `Nyírmeggyes`, `Zalameggyes` |
| `WS-DIG-008` | new sub-rule before step 8 | `a string created at a stem+suffix junction where the lemma ends in the single letter is a boundary (száz+szor, tíz+szer, vigyáz+sz)` - worth 71,192 `zsz` tokens on its own |
| `WS-DIG-008` | step 5 lexicon | extend the `g`-final first-member list with `szöveg`, `ideg`, `üveg`, `tag`, `ország`, `szalag`, `nyereg`, `hang`, `szőnyeg`, `virág`, `csillag`, `tömeg`, `egészség` |
| `WS-DIG-008` | step 7 lexicon | extend the `c`-final list with `árbóc`, `lánc`, `nyolc` |
| `WS-DIG-008` | Sources, Validation | add `[WC2] Webcorpus 2 frequency list, full type list, 2026-09-16`; `oracle:hunmorph-foma (to measure precision)` can be marked done for the six genuine doubled digraphs |
| `WS-DIG-008` | Confidence | `tendency` -> keep overall, but the step-8 long-letter default for `ccs`/`nny`/`ssz`/`ddz`/`lly`/`tty` is `consensus` on dictionary words; `ggy` and `zsz` must never reach it |
| `WS-DIG-006` | statistics note | replace the sample figures: `ccs 8/136` -> `13 of 1,222`; `nny 4/100` -> `182 of 5,135`; `ssz 9/100` -> `1,075 of 23,273`; `ggy 194 of 283` -> `956 of 1,724`; `zsz 137/2` -> `596 of 604`; `lly foreign 63/200` -> `313 foreign of 6,911 raw, 1 of 812 dictionary-valid`; `tty foreign 10/89` -> `42 of 8,039 raw, 0 of 1,780 dictionary-valid` |

---

## 11. Corpus prevalence: OQ-10-001, OQ-10-003, OQ-10-004 and the noise rates

These four questions ask not "what threshold" but "how often", and they are answered by a
full scan of the project corpus (not distributed) rather than by a sweep.

**The corpus, measured rather than assumed**: 2,430 parquet parts, **43 sources** (the 97
directory entries include `*.manifest.json`), **24,183,772 documents**, 139.1 Gchar,
19.39 G words. Of these, **9,644,687 documents / 77.92 Gchar carry `language = 'hu'`**;
the two largest sources by volume (`openwebmath` 48.6 Gchar, `code` 9.4 Gchar) are
English and are excluded from every Hungarian figure. The schema also carries
precomputed per-document `accent`, `dotless`, `letter_spaced`, `pre_reform`,
`replacement_rate`, `pua`, `invisible` and `unanalysable` columns, so the screening below
is a genuine full scan, not a sample: all 2,430 parts completed.

### 11.1 What the cleaning pipeline actually repaired

`repairs` is a JSON object per document. Corpus-wide key counts:

| repair | documents | characters |
| --- | ---: | ---: |
| invisible | 3,053,431 | 34.89 G |
| latin2 | 578,509 | 4.17 G |
| latin2_analysed | 281,199 | 13.41 G |
| entity | 179,841 | 1.81 G |
| ligature | 72,275 | 2.38 G |
| letter_spaced | 64,827 | 6.72 G |
| nfc | 37,733 | 2.43 G |
| control | 25,379 | 2.16 G |
| hyphen_replacement | 21,178 | 2.20 G |
| replacement | 20,587 | 1.20 G |
| fragment_rejoin | 10,760 | 0.44 G |
| private_use | 3,602 | 0.74 G |
| math_alnum | 2,452 | 0.32 G |
| font_shift | 1,490 | 0.13 G |
| doubled_acute | 163 | 46 M |

**`utf8_latin1` fired on zero documents in the entire corpus.** It is declared only for
the PDF sources; the crawls that actually carry the damage (`hplt`, `hq`, `hunsum`,
`wiki`) run only `entity, latin2, latin2_analysed, ligature, invisible, control, nfc`.
So every mojibake figure in 11.2 is unrepaired residue, and the `repairs` column has
nothing to agree or disagree with.

### 11.2 OQ-10-001 - windows-1252 versus windows-1250 (REP-ENC-003, REP-NOISE-002)

**The open question, exactly**: "Relative frequency of UTF-8 misdecoding as windows-1250
vs windows-1252, inferred from the Windows ANSI default."

Signatures were generated from the code page tables rather than hand-listed: lead
`Ã Å Â` plus any cp1252 image of bytes 0x80-0xBF for the 1252 family; lead `Ă Ĺ` plus any
cp1250 image for the 1250 family; the pair must touch a Latin letter. The detector was
validated against the rulebook's own examples and its negative set: `GyÅ'rben`, `KÃ©rem`
match 1252 only; `KĂ©rem`, `Ĺ'sz`, `tĹ±z` match 1250 only; `SÃO PAULO`, `Åland`, `Âge`,
`ROMĂN` match neither. `Ä` (byte C4) decodes identically in both code pages and cannot
discriminate, so it is counted as a third, ambiguous class rather than as 1250 evidence.

**The answer inverts with language, and the Hungarian answer is the one the rulebook
guessed:**

| population | only-1252 docs | only-1250 docs | n | P(1252 \| mojibake) | 95% Wilson |
| --- | ---: | ---: | ---: | ---: | --- |
| all documents | 10,116 | 1,258 | 11,374 | 0.8894 | 0.8835-0.8950 |
| **`language = 'hu'`** | **311** | **710** | **1,021** | **0.3046** | **0.2771-0.3335** |
| non-Hungarian | 9,805 | 548 | 10,353 | 0.9471 | 0.9426-0.9513 |

Hungarian tokens affected: 4,720 (1252) against 9,082 (1250). The direction never flips
under the hit threshold (P(1252) = 0.429 at >= 1 hit, 0.305 at >= 2, 0.261 at >= 3,
0.243 at >= 10), and per source it is the crawls that carry it: `hplt` 262:582,
`hq` 41:493, while `openwebmath` (English) is 9,508:121 and `code` 243:3.

**Proposed value: P(windows-1252 | mojibake) = 0.30 for Hungarian text, 95% CI
[0.28, 0.33]** - i.e. windows-1250 is about 2.3 times more common, which **confirms** the
Windows ANSI-default inference `REP-ENC-003` records as a guess. A naive corpus-wide
reading would have given 0.89 and pointed the opposite way; the English sources carry it,
and in `code` largely as literal escape data that `REP-ENC-002`'s own exception says to
suppress.

**Caveat on prevalence, not on the ratio.** Mojibake reaches only 0.011% of Hungarian
documents (1,021 of 9.64 M). The ratio is well determined; the absolute rate is not a
guide to what an uncleaned source looks like, because the crawl's own upstream has
already fixed an unknown amount.

**An actionable gap found on the way.** The pipeline's `_UTF8_AS_LATIN1` pattern has no
cp1250 branch at all - no `Ă`, no `Ĺ` - so the family that is 70% of Hungarian mojibake
is not detectable by the current cleaner even where it is declared.

### 11.3 OQ-10-003 - do the shifted-font thresholds generalise? (REP-FONT-003)

**The open question, exactly**: "Whether the 0.20 document-share threshold for Latin-2
damage and the shifted-font thresholds generalise (they come from one project: 12,000 web
documents, 125 PDFs)."

Full scan of the `dotless` column over all 24,183,772 documents, plus per-document `ı/İ`,
`ő/Ő`, `ű/Ű` counts and the 8-window spread. 252,558 documents (1.04%) carry any dotless
`i`.

| dotless per word | documents |
| --- | ---: |
| zero | 23,930,186 |
| < 1e-4 | 6,680 |
| 1e-4 .. 1e-3 | 135,933 |
| 1e-3 .. 5e-3 | 75,343 |
| 5e-3 .. 0.02 | 10,472 |
| 0.02 .. 0.05 | 10,346 |
| 0.05 .. 0.067 | 5,746 |
| **0.067 .. 0.085 (the rulebook's damaged band)** | **3,098** |
| 0.085 .. 0.15 | 2,380 |
| >= 0.15 | 3,588 |

**It is not bimodal.** There is no empty gap: the distribution is a smooth heavy tail from
1e-4 out to 0.2, the rulebook's damaged band holds 3,098 documents while 21,808 sit above
0.02, and the 1e-4 to 5e-3 region (212 k documents) is not damage at all but one or two
quoted Turkish names in healthy Hungarian.

Threshold sweep, population = documents with dotless >= 5, cells are documents flagged:

| min dotless | `ű` share <= | flagged | + spread >= 0.5 | + spread and `ő` >= 100 |
| ---: | ---: | ---: | ---: | ---: |
| 5 | 0.01 | 9,310 | 6,059 | 717 |
| 5 | 0.02 | 10,341 | 6,831 | 1,384 |
| 5 | 0.05 | 14,958 | 10,063 | 4,286 |
| 25 | 0.02 | 4,573 | 4,338 | 1,347 |
| 50 | 0.02 | 3,356 | 3,259 | 1,331 |
| **100** | **0.02** | 2,559 | 2,526 | **1,317** |
| 100 | 0.01 | 1,844 | 1,811 | 700 |
| 100 | 0.05 | 5,341 | 5,308 | 3,815 |
| 200 | 0.02 | 1,995 | 1,984 | 1,233 |

**The dotless-count threshold does almost no work.** Moving it from 100 down to 5, with
everything else held, adds 67 documents (1,317 -> 1,384). Moving the `ű`-share cap from
0.02 to 0.05 adds 2,498. The `ő >= 100` requirement and the `ű`-share cap carry the rule.

**Hand inspection, 20 documents drawn with seed 0.916** from the exact rulebook set
(dotless >= 100, `ő` >= 100, `ű` share <= 0.02, spread >= 0.5; 1,317 documents):
**20 of 20 are genuinely font-shifted Hungarian; 0 Turkish, 0 Azeri, 0 other.** They read
`fıbb` = `főbb`, `ülésérıl` = `üléséről`, `Képviselı` = `Képviselő`, `mőködı` = `működő`,
`közgyőlés` = `közgyűlés`, `Nınap` = `Nőnap` - exactly the `ı` -> `ő`, `ő` -> `ű` map.

A second sample of 20 from the **loosened** band (dotless 5-99, `ő` requirement dropped)
is **7 of 20 genuine**; the other 13 are Turkish localisation files and a Wiktionary
article on Izmir, and **all 13 have `ő` count 0**, so the `ő >= 100` clause removes every
one of them.

**Answer: the thresholds generalise for precision, on 17,000 times more documents than
the original 125, but two of the rule's clauses do not.**

- Precision 20/20 on 1,317 documents; the Turkish and Azeri negative set is excluded
  entirely. **Generalises.**
- `ő >= 100` and `ű` share <= 0.02 are load-bearing: dropping the first takes precision
  from 20/20 to 7/20; raising the second to 0.05 admits 2,498 unverified documents.
- `dotless >= 100` is **not** a useful threshold here. Lowering it to 25 costs nothing in
  precision and gains 30 documents; lowering it to 5 gains 67. It can safely drop to 25.
- **The residual-rate test does not generalise.** The rulebook says damaged documents
  measure 0.067-0.085 dotless per word and clean ones <= 0.00064. On this corpus the
  damaged population spreads 0.028 to 0.21 with no gap, and all 1,490 documents the
  pipeline repaired land at **exactly 0.0**, because the map is total. The band as stated
  is inert.

**Two findings the rule does not currently anticipate.**

First, **the 1,317 unrepaired shifted documents are not the ones the pipeline fixed.**
`font_shift` fired on 1,490 documents, all in PDF sources (`real` 448, `real-d` 426,
`szte-dokt` 244, `szte-acta` 206, `bme-omikk` 81, ...). The 1,317 that still meet the
fingerprint are entirely in the crawls (`hplt` 1,312, `hq` 5), almost all from
docplayer.hu, a document-hosting site whose text layer comes from the same TAMOP-era PDFs.
So the corpus holds roughly 1,490 repaired plus 1,317 unrepaired shifted documents, and
the unrepaired half is unreachable by the current per-source repair declaration.

Second, **those crawl documents are mixed, and the rule's gate would mis-route them.**
The docplayer.hu page chrome ("2 évvel ezelőtt", "Látták: Átírás") is clean and carries
real `ő`, while the document body is shifted. The `ű`-share test still passes, because
the chrome is small against a body with 100-500 shifted `ő`, so `REP-FONT-003`'s
three-outcome gate would choose "fully shifted" and turn `ezelőtt` into `ezelűtt`. The
crawl carrier needs a per-region or per-word decision, not the document-wide translate.

### 11.4 OQ-10-004 - CWI-2 prevalence (REP-ENC-009, REP-FONT-001)

**The open question, exactly**: "CWI-2 prevalence in current corpora, and whether `ô` for
`ő` instead comes from typed substitution."

Documents with at least 2 word tokens carrying a substitute glyph, with the CWI-2 tells
of `REP-ENC-009`: CP437 box drawing U+2500-U+257F (>= 5) and the CWI-2 capital images
`º ÿ ì Å ò ù` (>= 5).

| glyph | documents | substitute tokens | box tell | caps tell | both | already latin2-repaired |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| ô | 155,865 | 5,415,839 | 265 | 5,402 | 104 | 105,539 |
| û | 39,539 | 1,009,251 | 205 | 2,741 | - | 28,339 |
| õ | 66,452 | 711,991 | 209 | 2,946 | - | 48,286 |
| any | 188,167 | 5,579,060 | 414 | 6,276 | - | 121,814 |

Restricted to `language = 'hu'`: 129,870 `ô` documents, 5,186,152 substitute tokens, box
tell 32, capitals tell 1,992, **both tells 13**. The capitals tell is heavily
contaminated - `Å` is Scandinavian, `ò ù` Italian and French, `ì` Italian - so only the
box-drawing tell and the conjunction are trustworthy. The 13 Hungarian documents with
both tells are `real` 9, `szte-acta` 2, `elte-edit` 1, `real-d` 1: all scanned academic
PDFs, which is where DOS-era material would in fact survive.

**Answer, as a number: of 129,870 Hungarian documents carrying `ô`, 0.025% (32) carry the
clean CP437 tell, 0.010% (13) carry both, and at the most generous reading 1.5% (2,011)
carry either - and that 1.5% is an upper bound inflated by Scandinavian and Romance
capitals. CWI-2 accounts for at most 1.5% and realistically well under 0.1% of
`ô`-for-`ő` in this corpus. The remaining 98.5%+ is typed substitution, Latin-1 or
Latin-2 font fallback, or PDF font issues in otherwise clean modern text.**

Supporting evidence: **104,554 of the 129,870 (80.5%) already had `latin2` fire on them**,
i.e. they carried `õ`/`û` damage that the Latin-2 repair recognised, with `ô` left behind
because the Latin-2 map does not cover it. `REP-ENC-009` should therefore keep its mapping
table and be weighted near zero as a *prior*; the per-word decision of `REP-FONT-001` is
what does the work.

**Honest limit on the date argument.** `date` is the crawl timestamp for `hplt` and `hq`,
which is 93% of the population - the sampled docplayer.hu documents are dated 2017-2018
and contain 2008-2012 municipal minutes. Only the PDF sources have real authoring dates
(`szte-acta` median 2001, `real` median 2013). So the DOS-era argument rests on the 13
both-tell academic documents, not on the date column.

### 11.5 Accent loss in the wild (OQ-10-006, REP-NOISE-005 class weights)

Full scan of the `accent` column against the corpus baseline 0.09568, over the 9,644,687
Hungarian documents of at least 200 characters.

| register | documents | < 0.5% of baseline | < 5% | < 20% | < 50% |
| --- | ---: | ---: | ---: | ---: | ---: |
| web | 7,476,217 | 2,300 (0.031%) | 2,965 | 3,905 | 11,227 |
| news | 1,407,208 | 4 | 7 | 51 | 3,763 (0.267%) |
| encyclopedic | 336,539 | 0 | 3 | 459 | 19,505 (5.80%) |
| academic | 166,090 | 6 | 13 | 101 | 1,277 |
| dialogue | 103,258 | 0 | 0 | 15 | 112 |
| informal | 79,700 | 5 | 6 | 37 | 560 |
| legal | 32,078 | 1 | 4 | 17 | 73 |
| lexicographic | 8,891 | 0 | 0 | 6 | 391 (4.40%) |
| narrative | 7,945 | 1 | 1 | 1 | 8 |
| **all Hungarian** | **9,644,687** | **2,320 (0.024%)** | **3,003 (0.031%)** | **4,603 (0.048%)** | **37,068 (0.384%)** |

**Fully accent-stripped Hungarian is rare: 0.024% of Hungarian documents, 2,320 of
9.64 M, and 2,289 of those 2,320 (98.7%) are in the `hplt` web crawl.** The `< 50%` column
is not accent loss but register variation, which is why `encyclopedic` reads 5.8% there
(list-heavy stubs) and `lexicographic` 4.4%. An independent token-level histogram over the
same documents brackets the same figures and agrees at the fully-stripped end.

`REP-DIAC-002` classes at the rulebook's own thresholds, over the 1,067,174 Hungarian
documents long enough to have 200 letters per family:

| class | documents | share of eligible | top sources |
| --- | ---: | ---: | --- |
| B | 2,616 | 0.245% | `hplt` 744, `real` 678, `subtitles` 381, `szte-acta` 225 |
| **C** | **3** | 0.0003% | - |
| **D** | **4** | 0.0004% | - |
| E | 2,246 | 0.210% | `hplt` 535, `bme-omikk` 508, `szte-dokt` 369, `real` 208 |

**Classes C and D are effectively absent (3 and 4 documents of 1.07 M). They must be
recorded as unmeasured, not as measured-zero**: n = 3 and n = 4 cannot distinguish "does
not happen" from "the detector does not fire". This matters because section 4 showed class
D is the one class whose detector is exact by injection - so the detector works and the
phenomenon is simply not in this corpus, which is a statement about the corpus.

**Class E fires mostly on causes it is not the repair for.** It concentrates in
`bme-omikk` (508 of 6,701 eligible, 7.6%) and `szte-dokt` (369 of 7,136, 5.2%), which are
scanned theses - that is a scanner losing the `í` acute, an OCR signature, not the
ANSI-keyboard cause `REP-KEY-005` gives. In `literature` (125 of 1,870, 6.7%) it is
pre-reform orthography, not loss at all. Combined with section 4's finding that the class
E detector has precision 0.954 at its current settings, the class as specified is the
weakest part of `REP-DIAC-002`.

**Proposed edits**

| rule | field | old -> new |
| --- | --- | --- |
| `REP-ENC-003` | Conditions / weight | add `P(windows-1252 | mojibake) = 0.30, 95% CI [0.28, 0.33] for Hungarian text, i.e. windows-1250 is 2.3x more common (1,021 Hungarian documents of 9.64 M). The Windows ANSI-default inference is confirmed. A corpus-wide figure that does not exclude English reads 0.89 and points the other way.` |
| `REP-NOISE-002` | Conditions, `utf8_as` weights | `{windows-1252: 0.35, windows-1250: 0.25, ...}` -> swap the two leading weights to `{windows-1250: 0.35, windows-1252: 0.25, ...}`, or restate them as 0.30 / 0.70 of the 1252+1250 mass per the measurement |
| `REP-FONT-003` | Conditions, fingerprint | `dotless i/I count >= 100 in the document` -> `>= 25`; lowering it costs no precision (20/20 hand-checked at either value) and the `o-double-acute >= 100` and `u-double-acute share <= 0.02` clauses are what carry the rule |
| `REP-FONT-003` | Output, residual rate | `damaged documents measured 0.067-0.085 dotless per word, clean ones <= 0.00064; a repaired document must fall to the clean band` -> `the damaged population spreads 0.028-0.21 with no gap on a 24.2 M document corpus, and a repaired document falls to exactly 0.0 because the map is total; use "dotless = 0 after repair" as the test, not a band` |
| `REP-FONT-003` | Sources / Confidence | `tendency (thresholds from one corpus of 125 documents)` -> `measured: precision 20/20 by hand on the 1,317 documents of a 24,183,772-document corpus that meet the fingerprint, Turkish and Azeri excluded entirely` |
| `REP-FONT-003` | Output, mixed font | add: `a crawl carrier (document-hosting sites re-serving TAMOP-era PDFs) mixes clean page chrome with a shifted body; the u-double-acute test still passes because the chrome is small, so the whole-document map would damage it. Route crawl-sourced documents to the per-region branch even when all fingerprint conditions hold.` |
| `REP-ENC-009` | Confidence / weight | add: `CWI-2 accounts for at most 1.5% and realistically under 0.1% of o-circumflex-for-o-double-acute in a 9.64 M document Hungarian corpus (32 documents with the CP437 tell, 13 with both tells, of 129,870 carrying the glyph). Keep the mapping table; weight the hypothesis near zero as a prior. 80.5% of the carriers already had the Latin-2 repair fire on them, so REP-FONT-001's per-word decision is what does the work.` |
| `REP-NOISE-005` | Conditions, class weights | add: `measured prevalence in the project corpus, over 1,067,174 documents long enough to classify: class B 0.245%, class E 0.210%, classes C and D 3 and 4 documents respectively (unmeasured, not zero). Fully stripped text is 0.024% of Hungarian documents. The generator's weights are a training-mix decision and need not match these, but the gap should be deliberate.` |
| `REP-DIAC-002` | class E Cause | `Hungarian layout on ANSI boards` -> add `in this corpus the class fires mostly on scanned theses (bme-omikk 7.6%, szte-dokt 5.2%), which is OCR acute loss, and on pre-reform literary orthography (6.7% of literature), neither of which the class-E repair is the right repair for` |
| `REP-QUAL-005` | Output, `dotless_rate` | `shifted-font documents 0.067-0.085, clean <= 0.00064` -> `shifted-font documents 0.028-0.21 (no gap); 1.04% of all documents carry some dotless i, almost all of it one or two quoted Turkish names; clean after repair is exactly 0` |


---

## Summary table

`P` is the measured precision at the proposed value on the stated evidence.
Rows marked `confirmed` change no value; they record that the existing number was
measured and survived.

| rule ID | field | old value | new value | evidence size | measured P |
| --- | --- | --- | --- | ---: | ---: |
| `REP-DIAC-008` | conservative margin | 0.9 | **0.95** | 565,493 tokens | 0.9965 |
| `REP-DIAC-008` | balanced margin | 0.6 | 0.6 (confirmed) | 565,493 tokens | 0.9847 |
| `REP-DIAC-001` | accent share cut | < 0.5% | **< 0.2%** | 7,642 / 19,757 segments | 1.0000 |
| `REP-DIAC-001` | signature words | >= 5 | **>= 2** (pre-filter only) | 7,642 / 19,757 segments | 1.0000 |
| `REP-DIAC-001` | rare stripped tokens | >= 2 | **dropped** | 7,642 / 19,757 segments | 1.0000 |
| `REP-DIAC-001` | restoration-acceptance clause | >= 60% | **>= 50%, over >= 5 rejected tokens** | 7,642 / 18,337 segments | 0.9992 |
| `REP-DIAC-002` | class E threshold | share(i-acute) < 0.02 | **< 0.01** | 13,531 segments | 0.9955 |
| `REP-DIAC-002` | class C o-umlaut baseline | 0.53 | **0.536, range 0.39-0.60 by register** | 9,644,687 documents | n/a |
| `REP-DIAC-002` | class E i-acute baseline | 0.14 / 0.13 | **0.124** | 63.42 G letters | n/a |
| `REP-DIAC-002` | family count floor | >= 20 | **>= 50** (i and o families) | 13,531 segments | 0.9955 |
| `REP-DIAC-002` | class D threshold | share(a-acute) < 0.03 | 0.03 (confirmed) | 13,527 segments | 1.0000 |
| `REP-DIAC-002` | class B u-double-acute test | share < 0.02 | **optional** (never evaluable) | 37 decidable of 13,531 | n/a |
| `REP-QUAL-001` | accented token baseline | 43.16 / 35.75 / 48% | **47.77%** | 10.40 G tokens | n/a |
| `REP-QUAL-001` | accented character baseline | 6.59% | **9.57%** | 77.92 Gchar | n/a |
| `REP-QUAL-001` | accented letter baseline | 11.8% | 11.76% (confirmed) | 63.42 G letters | n/a |
| `REP-QUAL-001` | per-family table | NS15 Table 1 | confirmed within 2 pt (u family 4.8) | 63.42 G letters | n/a |
| `REP-SPACE-001` | guard A one-letter set | a á e é s ó ő | **a á e é s** | 2,373 runs / 19,757 segments | 0.9966 |
| `REP-SPACE-001` | run-length evidence | >= 4 | **>= 3** | 2,373 runs | 0.9966 |
| `REP-QUAL-006` | spaced_runs length | >= 4 | **>= 3** | 2,373 runs | 0.9966 |
| `REP-SPACE-003` | document trigger | 0.30 | **0.35** (per document) | 127 / 1,978 documents | 1.0000 |
| `REP-SPACE-003` | healthy short-word share | about 0.20 | **0.239** | 1,978 documents | n/a |
| `REP-SPACE-003` | damaged short-word share | > 0.40 | **> 0.55** | 127 documents | n/a |
| `REP-QUAL-006` | short_word_share band | 0.20 / 0.30 / 0.40 | **0.24 / 0.35 / 0.55** | 2,105 documents | n/a |
| `REP-ENC-016` | C1 control term | -5.0 * rate | **admissibility veto** | 3,080 / 19,757 segments | 0.9989 |
| `REP-ENC-016` | accept margin | 0.25 | **0.10** (with the veto) | 3,080 / 19,757 segments | 0.9989 |
| `REP-ENC-014` | flying accent rate | 1 per 20 words | **1 per 10 words** | 4,000 / 4,000 segments | 1.0000 |
| `REP-ENC-014` | digit notation rate | "consistent" | **1 per 20 words** | 3,997 / 4,000 segments | 1.0000 |
| `REP-ENC-014` | ISO 646-HU rate | (none) | **1 per 10 words** | 4,000 / 4,000 segments | 1.0000 |
| `REP-ENC-014` | decoded acceptance | >= 0.8 | 0.8 (confirmed, non-binding) | 12,000 segments | 1.0000 |
| `PH-HARM-014` | zone-stem baseline | 40.3% front | **39.2% back / 60.8% front** | 699 stems | n/a |
| `PH-HARM-014` | sibilant effect | 65.1% front | **not distinguishable from 67.1%** | 138 vs 231 stems | n/a |
| `PH-HARM-014` | coronal sonorant effect | 64.2% front | **not distinguishable from 67.1%** | 183 vs 231 stems | n/a |
| `PH-HARM-014` | CC effect, default on | 93.8% front | 94.0% (confirmed) | 168 stems | n/a |
| `PH-HARM-012` | BNé backness index | 0.421 | **0.316** | 16 stems (provisional) | n/a |
| `PH-HARM-012` | BNN backness index | 0.206 | **0.185** | 79 stems (provisional) | n/a |
| `PH-HARM-012` | NN row total | 939 / 933 | **933** (2 vowels only) | 2,028 stems | n/a |
| `PH-HARM-012` | R row total | 698 | **1,517** | 10,974 stems | n/a |
| `PH-HARM-022` | foreign p_back | "estimate from corpus" | **provisional, 15 stems** | 15 stems (provisional) | n/a |
| `WS-DIG-008` | ccs step-8 precision | ~94% (134 types) | **98.9%** | 1,222 types | 0.9894 |
| `WS-DIG-008` | nny step-8 precision | 96% (97 types) | **96.5%** | 5,135 types | 0.9646 |
| `WS-DIG-008` | ssz step-8 precision | 91% (97 types) | **95.4%** | 23,273 types | 0.9538 |
| `WS-DIG-008` | lly / tty / ddz | (not given) | **99.9 / 100 / 98.1%** | 812 / 1,780 / 53 types | as stated |
| `WS-DIG-008` | zsz boundary share | 137 vs 2 types | **98.7% (596 of 604)** | 604 types | 0.9868 |
| `WS-DIG-008` | ggy boundary share | 194 of 283 | **55.5% (956 of 1,724)** | 1,724 types | 0.5545 |
| `REP-ENC-003` | P(1252 \| mojibake), Hungarian | (unstated, inferred) | **0.30, CI [0.28, 0.33]** | 1,021 documents | n/a |
| `REP-NOISE-002` | utf8_as code page weights | 1252 0.35 / 1250 0.25 | **1250 0.35 / 1252 0.25** | 1,021 documents | n/a |
| `REP-FONT-003` | dotless count fingerprint | >= 100 | **>= 25** | 24,183,772 documents | 20/20 by hand |
| `REP-FONT-003` | residual dotless band | 0.067-0.085 vs <= 0.00064 | **0.028-0.21 vs exactly 0** | 24,183,772 documents | n/a |
| `REP-QUAL-005` | dotless_rate band | 0.067-0.085 / <= 0.00064 | **0.028-0.21 / 0** | 24,183,772 documents | n/a |
| `REP-ENC-009` | CWI-2 prior | (unstated) | **<= 1.5%, realistically < 0.1%** | 129,870 documents | n/a |
| `REP-NOISE-005` | measured class prevalence | (none) | **B 0.245%, E 0.210%, C and D unmeasured** | 1,067,174 documents | n/a |
| `REP-QUAL-001` | register spread note | (none) | **acc/token 37.2-51.8%, o-share 0.39-0.60** | 9,644,687 documents | n/a |


---

## Not calibrated, and what each one needs

Of the 23 `statistical-estimate` entries, the following could not be settled with the
data available here. Each says what is missing, not merely that it is missing.

### Chapter 10

**`OQ-10-005` - per-pair OCR confusion rates (T-OCR-1, `REP-OCR-*`).** The corpus has
2,984 `ocr` pairs, but they were generated by `damage.misread` from a **hand-written
28-entry confusion list at a flat 12% rate**. Measuring the rulebook's costs against them
would recover the generator's own list, not the truth - the circularity is total, and any
number produced that way would be worse than the current honest "ranked estimates
anchored on JiM counts". Needs: a published Hungarian OCR confusion matrix, or the
8,593-type analysis of L22/M24, or a same-edition parallel scanned/clean corpus. Nothing
in the project corpus is a scan with its own printed source.

**`OQ-10-017` - whether the L22 `é` -> `e` count of 581 is OCR error or edition
difference.** Same missing asset: a same-edition parallel corpus. Untouched.

**`REP-SPACE-003` run-evidence clause (`FRAG_RUN_SHARE` 0.25, `FRAG_RUN_MIN_BAD` 2,
`FRAG_MEAN_LEN` 4.5).** Measured and rejected as uncalibratable here, for two reasons
given in section 6: an attestation lexicon cannot reject Hungarian syllable fragments
because most of them are words, and the damage generator spaces a stretch of sentences
so per-sentence measures are diluted. Needs: the morphological analyser as the oracle,
plus pairs carrying per-sentence damage spans.

**`REP-ENC-016` numeric weights (2.0 / 1.0 / 3.0 / 5.0 / 5.0).** The three terms other
than the C1 one are not identifiable on single-damage pairs: admissibility and the
accepted-word ordering constraint separate the classes before the weights are consulted
(median gain 1.43 against a clean-text noise floor of 0.001). Needs: mixed-encoding
documents of the `REP-ENC-015` kind, where two admissible hypotheses both raise the
accepted-word share. The pair file contains none, because `damage.SINGLE` forbids
composing encoding faults.

**`REP-DIAC-001` foreign-language negatives.** Section 3 relocates the work of the rule
onto the restoration-acceptance clause, which is language-independent by construction, so
the proposal no longer *depends* on the signature clauses holding foreign text out. But
that argument is an argument, not a measurement: the `lang` column of
the project corpus (not distributed) is empty for all 441,718 sampled documents and the pair file is
Hungarian throughout, so no foreign segment was ever scored. Needs: an English / Dutch /
Latin / Indonesian negative set of at least 2,000 segments, run through the same sweep.
Expected result is that the restoration clause sits near zero on all four; if it does not,
the signature pre-filter must go back up rather than down.

**`OQ-10-006` real partially-accented text.** The class *detectors* were calibrated by
**injection**, not on labelled real data, because no labelled partially-stripped corpus
exists here; the class-B figure rests on 79 decidable segments. Section 11.5 supplies the
missing half - real prevalence over 1,067,174 documents - but prevalence is not a
detector evaluation: a document the detector calls class B was never independently
labelled class B. **Classes C and D are the sharp gap: 3 and 4 documents respectively of
1.07 M, which cannot distinguish "does not occur" from "detector does not fire", so both
must be recorded as unmeasured rather than as measured-zero.** Needs: the forum/SMS sample
the open question already asks for, with per-document class labels, which would also let
the class-B and class-E detectors be scored against something other than their own
injection.

**`OQ-10-012`, `OQ-10-014`, `OQ-10-011` residue.** `OQ-10-011` is answered (section 5),
but its precision side rests on a negative base of 8 runs; a larger technical-text
negative set (formulas, variable listings) would make the guard's value measurable rather
than merely bounded.

### Chapter 02 and elsewhere

**`OQ-02-002` suffix-type weights (`V` vs `C` suffixes).** Both Hayes Google files carry
the dative `-nak`/`-nek` only, so there is nothing in them to fit a suffix-type contrast
against. Needs: a suffix inventory over Webcorpus 2, which is a separate extraction job
(the parquet has emMorph tags, so it is feasible, but it is not a threshold sweep).

**`OQ-02-004` and `OQ-02-015` item-level figures for `-né` and stacked neutral suffixes.**
The Hayes files contain bare stems only; no suffixed forms. The Webcorpus 2 parquet could
supply per-item counts for `Szabónénak` / `Szabónének` and for `-i`+`-é`, `-i`+`-ék`, but
that is an item-level attestation task for the `attest.py` / `wc2.py` pipeline, not a
calibration sweep, and it was not run here.

**`OQ-02-010` foreign pronunciation heuristic accuracy (`PH-G2P-009`).** Needs an openly
licensed Hungarian respelling lexicon to evaluate against. None is present in
`local/research-lit` or `local/lists`. Untouched.

**`OQ-03-010` quantitative `-a`/`-ja` model.** Partially settled by workstream G
([`docs/grammar/evidence/productivity-tagset.md`](productivity-tagset.md) Q-X2), which fixes the default and the
sibilant condition. The full quantitative model over derived and adjectival stems was not
attempted here and still needs either Rácz and Rebrus 2012's data or a fit on Webcorpus 2.

**`OQ-09-015` segmentation-heuristic thresholds (`SYN-SEG-002`, `004`, `005`, `012`).**
Needs a sentence-segmented annotated corpus with abbreviation-class labels. The project corpus
has no sentence-boundary gold; `benchmark-a2.jsonl` is 7 lines. Untouched.

**`OQ-04-001`, `OQ-04-013`, `OQ-08-012`, `OQ-08-013`, `OQ-09-002`, `OQ-09-010`,
`OQ-09-013` - register-stratified frequencies.** These need a register-labelled *sentence*
corpus with syntactic annotation (antecedent typing for `ami`/`amely`, definiteness of
objects, address-line context for `Kedves X,`). the project corpus (not distributed) has a source
column that maps to registers, so raw string counts per register are obtainable, but none
of these questions can be answered by string counting alone - `ami` with a noun antecedent
is not a regex. A dependency-parsed register-stratified corpus is the missing asset for
all seven.

**`REP-FONT-003` mixed-font share.** Section 11.3 measures how many documents each
threshold combination flags, and hand-checks the fully-shifted set. It does not measure
how many of the 2,498 documents in the 0.02-0.05 `ű`-share band are genuinely mixed rather
than clean. That needs per-region inspection of the PDF font resources, which the crawl
carrier does not preserve.

### The honest total, against the 23 entries

| status | count | entries |
| --- | ---: | --- |
| calibrated, value changed | 6 | `OQ-10-001`, `OQ-10-002`, `OQ-10-003`, `OQ-10-004`, `OQ-10-011`, `OQ-01-010` |
| calibrated, value confirmed or the claim withdrawn | 3 | `OQ-02-001` (consonant effects: CC confirmed, sibilant and coronal sonorant withdrawn), `OQ-02-015` (item figures supplied where the file has them), `OQ-10-006` (prevalence supplied, detectors calibrated by injection) |
| not calibrated, asset named | 14 | `OQ-02-002`, `OQ-02-004`, `OQ-02-010`, `OQ-03-010`, `OQ-04-001`, `OQ-04-013`, `OQ-08-012`, `OQ-08-013`, `OQ-09-002`, `OQ-09-010`, `OQ-09-013`, `OQ-09-015`, `OQ-10-005`, `OQ-10-017` |

Six of the fourteen uncalibrated entries (`OQ-04-001`, `OQ-04-013`, `OQ-09-002`,
`OQ-09-010`, `OQ-09-013`, and `OQ-08-012`) need the same single asset: a
register-stratified, dependency-parsed Hungarian corpus. That is the highest-value missing
input in the whole register, and it would close more than a quarter of this category on
its own.

