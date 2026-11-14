# Corpus evidence for the usage-variant questions

Measured 2026-09-16. Covers every `usage-variant` entry in [`../open-questions.md`](../open-questions.md) (30) and the
`source-conflict` entries a corpus can arbitrate (10). Nothing in the chapters or in
`open-questions.md` is edited here: each section ends with a proposed field-level edit for the
rule owner to apply.

## Method

### Data

| store | what it is | role |
| --- | --- | --- |
| Webcorpus 2 frequency list | 36,936,726 `form` + `lemma` + emMorph `xpostag` + `freq` rows over a 9e9-token corpus. Rácz, `Webcorpus2FrequencyList`, MIT, Zenodo 10.5281/zenodo.17508385. | the ratio and the homograph control |
| project rate index | per-source word rates over the project corpus (not distributed), 26 sources, absolute count = rate x scanned chars | the register gradient |
| project cleaned corpus | the project corpus as cleaned parquet (not distributed), 97 sources, 55 GB | multiword and hyphenated patterns, sentence contexts |

The project corpus in this table is a 26-source collection the project does not distribute. Figures from it are a dated record and are not reproducible from this repository; [`tools/ratios.py`](../../../tools/ratios.py) and [`tools/kwic.py`](../../../tools/kwic.py) now run over the four public sources in [WORKSPACE.md](../../../WORKSPACE.md), which reproduces the permissive web, official and literary figures only.

Webcorpus 2 supplies the headline ratio wherever both variants are single orthographic words,
because it has no frequency cut-off and carries a morphological tag per row. The project corpus
supplies the register split, which Webcorpus 2 lacks. The cleaned parquet is used only where
both other stores fail: the rate index splits hyphenated tokens, and Webcorpus 2 contains **zero**
digit-initial hyphenated forms (measured), so date, unit and acronym suffixes exist only there.

### Cut-off

The project rate index keeps each source's top 400,000 types. A form absent from a source means
"below that source's cut-off", never "unattested". Absence claims in this document rest on
Webcorpus 2 or on a `kwic.py` scan, never on the rate index.

### Reliability floor and CI

A ratio is quoted only when every competing variant reaches n >= 20 in the bucket quoted.
Intervals are Wilson 95% score intervals on the share of the first-named form. Buckets below the
floor are reported as counts with the verdict `insufficient evidence`.

### Homograph control

Three instruments, in order of preference:

1. **emMorph tag filter** (`wc2.py --tag --lemma`). `orrt` is 158 tokens of lemma `orr` in total
   but only **2** carry `[/N][Acc]`; the rest are mis-tagged or a different lemma. Every
   Webcorpus 2 figure below states the filter used.
2. **Context filter** (`kwic.py --near`), where the tag set cannot separate the readings.
3. **Sample inspection** of KWIC contexts, where the competing string belongs to a different
   lemma entirely. Flagged per section.

Where the tag set demonstrably mis-analyses a form (`teremtem` under lemma `terem`, `iva` under
lemma `i`), the section says so and the verdict is weakened accordingly.

### Licence status of figures

Publishable sources: `hplt`, `gazette`, `hq`, `openwebmath`, `parlamint`, `literature`, `karoli`,
`tenders`. Webcorpus 2 derived counts are publishable (MIT) with the Zenodo citation. Every
other project source is share-alike, non-commercial or train-only.

**Figures marked `[NP]` are non-publishable**: they aggregate at least one restricted source and
exist as internal evidence only. Group tables marked `[NP]` include `hunsum`, `subtitles`, `wiki`,
`njt` and the academic PDF sources. Webcorpus 2 rows and permissive-only aggregates are
publishable.

### Reproduction

```sh
# from the repository root, environment as in WORKSPACE.md
# ratio + register gradient in one call
python tools/ratios.py --wc2 --tag Pst mondtak mondottak --by-source

# tag inventory before choosing a filter
python tools/wc2.py orrt orrot --raw

# hyphenated and multiword patterns (one pass per batch of forms)
python tools/kwic.py --regex --jobs 20 --sources all \
  '(?<![\w-])\d{1,2}-i(?![\w-])' '(?<![\w-])\d{1,2}-ei(?![\w-])'
```

The count index builds itself on first use (`local/cache/counts.sqlite`). KWIC results
cache under `local/cache/kwic/`. Tool documentation: [`tools/README.md`](../../../tools/README.md).

---

# usage-variant

## OQ-04-010 - article before foreign names in `h`, `y`

Rules: `PND-ART-002`. Forms: article + capitalised `Hu-` name, article + capitalised `Y-` name.
Instrument: `kwic.py --regex` over `small,hunsum,subtitles,hq`. Homograph control: the pattern
requires a capital letter followed by lower-case, so common words are excluded; readings were
checked in 6 contexts per pattern.

| bucket | `az Hu-` | `a Hu-` | `az` share | `az Y-` | `a Y-` | `az` share |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| spoken | 24 | 4,816 | | 193 | 8,257 | |
| news `[NP]` | 69 | 35,539 | | 919 | 19,941 | |
| wiki `[NP]` | 29 | 10,030 | | 623 | 8,252 | |
| official | 1 | 4,782 | | 85 | 108 | |
| literary | 37 | 1,014 | | 89 | 132 | |
| web | 364 | 37,447 | | 1,556 | 41,216 | |
| **all `[NP]`** | **529** | **94,695** | 0.0056 [0.0051, 0.0060] | **3,501** | **78,525** | 0.0427 [0.0413, 0.0441] |
| permissive only | 402 | 40,320 | 0.0099 | 1,715 | 41,455 | 0.0397 |

Contexts: `az Hu-` is almost entirely pre-reform literature (`az Hugó`, `az Humprey`,
`az Huszain aga`) plus the acronym-like `az Hungexpo`. `az Y-` is not archaic: it tracks names
pronounced with an initial vowel (`az Ybl`, `az Yves`), which is exactly what the rule predicts.

**Verdict: A preferred** (`a` before `h`-initial foreign names, 99.4%), **register-conditioned
only in that the `az` variant is archaic**; `y`-initial names remain genuinely split (4.3% `az`)
by pronunciation.

Proposed edit - `PND-ART-002`, **Examples**:
old: `` ... `az Yves Saint Laurent` (`[ív]`), `az Ypres`; `az Huelva` (`[uelva]`) ~ `a Huelva` (spelling pronunciation). ``
new: `` ... `az Yves Saint Laurent` (`[ív]`), `az Ypres`; `a Huelva`, `a Hugo` (spelling pronunciation, 99.4% of article tokens before `h`-initial foreign names; `az Hugó` is pre-reform only). ``

## OQ-01-003 - letter-by-letter reading of `W Q X Y`

Rules: `WS-ALPH-012`. Forms: `duplavé` vs `dupla vé`. Instruments: Webcorpus 2 (solid form),
`kwic.py` (spaced form, which no word list keeps).

| form | Webcorpus 2 | cleaned corpus `[NP]` |
| --- | ---: | ---: |
| `duplavé` | 82 | 79 |
| `dupla vé` | - (not a token) | 12 |
| `duplavét` | 8 | 4 |
| `dupla vét` | - | 1 |

n = 91 for the spaced/solid contrast, above the floor; but no suffixed acronym ending in a
`W`-name was found in either store, which is what the question actually needs.

**Verdict: insufficient evidence** for the acronym reading. Settled as a side result: the letter
name is written solid (`duplavé`) about 7:1 over `dupla vé`.

Proposed edit - `WS-ALPH-012`, **Examples**: append
`` ; the `w` name is written solid, `duplavé` (82) vs `dupla vé` (12) ``.
No change to the rule Statement: the acronym reading is still unmeasured.

## OQ-02-005 - harmony of letter-read acronyms after a back letter name

Rules: `PH-G2P-010`. Forms: acronym + `-val`/`-vel`. Instrument: `kwic.py --regex` over
`parlamint,wiki,gazette,njt,tenders,wikitalk,kocsmafal,hunsum,literature,bloghu,gyakorikerdesek`
(hyphenated tokens exist in no word list). Homograph control: the pattern anchors on the exact
acronym plus the exact suffix.

| acronym | back `-val` | front `-vel` | back share | expected from the final letter name |
| --- | ---: | ---: | ---: | --- |
| `BKV` `[NP]` | 500 | 17 | 0.967 [0.948, 0.979] | `vé` -> front |
| `MÁV` `[NP]` | 392 | 0 | 1.000 | word reading -> back |
| `OTP` `[NP]` | 1 | 225 | 0.004 [0.001, 0.025] | `pé` -> front |
| `MTV` `[NP]` | 0 | 129 | 0.000 | `vé` -> front |

The permissive subsets are thin (`BKV` 8/1, `OTP` 15/0), so all four rows are internal evidence.

`BKV` and `MTV` both end in the letter name `vé` and behave oppositely. The final-letter-name rule
therefore does not predict the outcome: `békávé` takes back harmony because it contains a back
vowel (`á`) with only neutral vowels after it, while `emtévé` has none. `OTP` (`ótépé`) contains
a back `ó` and still takes front, so even the dominant-vowel account fails on it.

**Verdict: register-conditioned = no; item-lexicalised = yes.** Harmony of letter-read acronyms is
not derivable from the final letter name and must be lexical per acronym.

Proposed edit - `PH-G2P-010`, **Statement**:
old: `` ... vowel length does not change the pronounced end or harmony, which the name determines for suffixes. ``
new: `` ... vowel length does not change the pronounced end or harmony; the final letter name determines the suffix by default, but harmony is lexical for acronyms whose spelled-out form mixes back and neutral vowels (`BKV-val` 500 : 17, `MÁV-val` 392 : 0 against `OTP-vel` 225 : 1, `MTV-vel` 129 : 0). ``

Proposed edit - `ORT-ABBR-018`, **Examples**: change `` `OTP-t` [ó-té-pé] `` to
`` `OTP-t`, `OTP-vel` [ó-té-pé, front]; `BKV-val` [bé-ká-vé, back - lexical] ``.

## OQ-02-007 - bare vs linking-vowel accusative after clusters and geminates

Rules: `PH-SYL-008`, `PH-SYL-009`. Instrument: Webcorpus 2 filtered to `[Acc]` on the named lemma.
Homograph control: without the tag filter `orrt` shows 158 tokens; with `[/N][Acc]` and lemma
`orr` it shows 2.

| item | linking vowel | bare | LV share | cluster |
| --- | ---: | ---: | ---: | --- |
| `orr` | `orrot` 8,930 | `orrt` 2 | 0.9998 [0.9992, 0.9999] | geminate `rr` |
| `fajansz` | `fajanszot` 134 | `fajanszt` 9 | 0.937 [0.885, 0.967] | `nsz` |
| `bross` | `brossot` 32 | `brosst` 508 | 0.059 [0.042, 0.082] | geminate `ss` |
| `briliáns` | `briliánsot` 0 | `briliánst` 199 | 0.000 [0.000, 0.019] | `ns` |

All four rows are Webcorpus 2, publishable. Project-corpus register split for `orr` `[NP]`:
`orrot` spoken 414 / news 956 / literary 131 / web 2,835 against `orrt` spoken 19 / literary 21 /
technical 3 - the `orrt` tokens are nineteenth-century literature (Jókai) and subtitle noise, not
current usage.

**Verdict: item-lexical, no general preference.** The geminate `rr` of `orr` takes the linking
vowel categorically; the geminate `ss` of `bross` and the `ns` of `briliáns` take bare `-t`
categorically; `fajansz` takes the linking vowel. The chapters' "lexically marked" formulation is
correct, and the per-item values above are the marks.

Proposed edit - `PH-SYL-008`, **Examples**:
old: `` see table; also `ENSZ-t` (spelled acronym /ɛns/). ``
new: `` see table; also `ENSZ-t` (spelled acronym /ɛns/); measured shares: `briliánst` 199 : `briliánsot` 0, `fajanszot` 134 : `fajanszt` 9. ``

Proposed edit - `PH-SYL-009`, **Examples**:
old: `` `idill`->`idillt` [ATT 2]; `finn`->`finnt` [ATT 1]; `plüss`->`plüsst`; `dzsessz`->`dzsesszt` [ATT 0]; ``
new: `` `idill`->`idillt` [ATT 2]; `finn`->`finnt` [ATT 1]; `plüss`->`plüsst`; `bross`->`brosst` 508 : `brossot` 32; `dzsessz`->`dzsesszt` [ATT 0]; ``

Proposed edit - `NOM-ACC-006`, **Examples**:
old: `` `orrot` [ATT 5] vs `orrt` `-`; ``
new: `` `orrot` 8,930 vs `orrt` 2 (Webcorpus 2, `[/N][Acc]`); ``
This also settles `OQ-03-012` (`engineering-choice`): do not generate `orrt`; **do** generate
`brosst`, which is the majority form.

## OQ-03-002 - `mag`: `magok ~ magvak`, `magja ~ magva`

Rules: `NOM-STEM-011`, `NOM-STEM-016`. Instrument: Webcorpus 2 `[Pl]` / `[Poss.3Sg]` on lemma
`mag`, plus the project register split and KWIC contexts from `karoli`.

| contrast | A | B | A share |
| --- | ---: | ---: | ---: |
| `magok` : `magvak` `[Pl]` | 42,603 | 32,449 | 0.568 [0.564, 0.571] |
| `magja` : `magva` `[Poss.3Sg]` | 33,860 | 7,314 | 0.822 [0.819, 0.826] |

Register `[NP]`, project corpus:

| bucket | `magok` | `magvak` | `magok` share | `magja` | `magva` | `magja` share |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| spoken | 579 | 180 | 0.763 | 563 | 62 | 0.901 |
| news | 960 | 976 | 0.496 | 1,760 | 114 | 0.939 |
| wiki | 1,237 | 628 | 0.663 | 1,287 | 251 | 0.837 |
| official | 198 | 627 | 0.240 | 252 | 16 | 0.940 |
| technical | 14,921 | 5,167 | 0.743 | 6,320 | 3,308 | 0.657 |
| literary | 3,317 | 169 | 0.952 | 176 | 505 | 0.258 |
| web | 15,279 | 14,104 | 0.520 | 10,652 | 2,858 | 0.788 |

Contexts: `magva` in `karoli` is the biblical 'seed, offspring' (`Izráelnek, az ő szolgájának
magva`); in technical prose it is the 'kernel, core' sense.

**Verdict: register-conditioned with a sense split.** `magvak` is the agricultural and official
plural (official 76% `magvak`) while `magok` is the general one (literary 95% `magok`). `magva` is
the literary and 'essence/kernel' 3sg (literary 74% `magva`) while `magja` is the default
elsewhere.

Proposed edit - `NOM-STEM-016`, **Statement**: append
`` ; `mag` has both alternants with a usage split: plural `magok` (general, 57%) ~ `magvak` (botanical and official, 76% of official-register tokens), 3sg possessive `magja` (general, 82%) ~ `magva` (literary and 'kernel, essence', 74% of literary tokens). ``

Proposed edit - `NOM-POSS-009`, **Examples** table: add the row
`` `mag` | `magja` 33,860 (default) | `magva` 7,314 ('kernel, essence'; literary) ``.

## OQ-03-008 - accusative and plural linking vowel of `-ik` forms

Rules: `NOM-ADJ-009`. Instrument: Webcorpus 2 `[Acc]` / `[Pl]`.

| contrast | A | B | A share |
| --- | ---: | ---: | ---: |
| `nagyobbikat` : `nagyobbikot` | 1,218 | 3 | 0.998 [0.993, 0.999] |
| `kisebbiket` : `kisebbikot` | 1,213 | 0 | 1.000 [0.997, 1.000] |
| all `[Acc]` in `-ikat` : `-ikot` | 1,483,254 | 63,355 | 0.959 |
| all `[Acc]` in `-iket` | 2,746,511 | | |

Project corpus `[NP]` agrees: `nagyobbikat` spoken 36 / news 46 / technical 99 / web 301 against
`nagyobbikot` 0 in every bucket.

**Verdict: A preferred, near-categorical.** `-ik` takes `-at`/`-et`, never `-ot`. The `?` on
`nagyobbikot` should become an exclusion.

Proposed edit - `NOM-ADJ-009`, **Examples**:
old: `` `nagyobbik` [ATT 10], `kisebbik` [ATT 10], `jobbik` [ATT 15]. ``
new: `` `nagyobbik` -> `nagyobbikat` 1,218 (not `*nagyobbikot`, 3), `kisebbik` -> `kisebbiket` 1,213 (`*kisebbikot` 0), `jobbik` [ATT 15]; `-ik` takes `-at`/`-et`, never `-ot`. ``

## OQ-04-003 - `ezerszáz` vs `ezeregyszáz`

Rules: `PND-CARD-005`. Instrument: Webcorpus 2 (whole-word forms).

| contrast | A | B | A share |
| --- | ---: | ---: | ---: |
| `ezerszáz` : `ezeregyszáz` | 694 | 1,304 | 0.347 [0.327, 0.368] |
| `ezerkilencszáz` : `ezerkilencegyszáz` | 415 | 0 | 1.000 [0.991, 1.000] |
| `egymillióezer` : `egymillióegyezer` | 0 | 0 | - |

Register `[NP]`: `ezerszáz` literary 20 / news 63 / spoken 2 against `ezeregyszáz` spoken 24 /
news 182 / web 389 / technical 31.

**Verdict: B preferred** (`ezeregyszáz`, 65%) **but both current**; the `egymillió-ezer` contrast
is **insufficient evidence** (0 : 0). Note that `egy` appears only where it disambiguates the
hundreds digit: `ezerkilencszáz` never takes it.

Proposed edit - `PND-CARD-005`, **Statement**:
old: `` ... `egyszáz`, `egyezer` are used where unambiguity matters (money, legal texts). ``
new: `` ... `egyszáz`, `egyezer` are used where unambiguity matters (money, legal texts); inside a larger numeral the hundreds digit takes `egy` about 2:1 (`ezeregyszáz` 1,304 : `ezerszáz` 694), while a non-1 hundreds digit never does (`ezerkilencszáz` 415 : `ezerkilencegyszáz` 0). ``

## OQ-04-004 - plural linking vowel of back-harmony ordinals

Rules: `PND-ORD-002`. Instrument: Webcorpus 2 `[Pl]`.

| contrast | A | B | A share |
| --- | ---: | ---: | ---: |
| `harmadikak` : `harmadikok` | 1,576 | 3,200 | 0.330 [0.317, 0.343] |
| `hatodikak` : `hatodikok` | 610 | 434 | 0.584 [0.554, 0.614] |
| `ötödikek` : `ötödikök` | 1,635 | 0 | 1.000 [0.998, 1.000] |
| all `[Pl]` `-ikak` : `-ikok` | 8,682 | 40,655 | 0.176 |
| all `[Pl]` `-ikek` : `-ikök` | 13,803 | 209 | 0.985 |

**Verdict: both current, item-split.** Back ordinals take `-ikok` 82% of the time at pattern level
but per item the two are within a factor of 2 (`hatodikak` even leads). Front ordinals take
`-ikek` categorically. The earlier "ATT 0 vs 2" is superseded.

Proposed edit - `PND-ORD-002`, **Statement**: append
`` ; the back-harmony plural varies between `-ikak` and `-ikok` with no clear default (`harmadikok` 3,200 : `harmadikak` 1,576, but `hatodikak` 610 : `hatodikok` 434); the front plural is always `-ikek` (`ötödikek` 1,635 : `ötödikök` 0). ``

## OQ-04-005 - `15-i` vs `15-ei`

Rules: `PND-DATE-006`. Instrument: `kwic.py --regex` over `small,hunsum,subtitles,hq`; the forms
are hyphenated and exist in neither word list (Webcorpus 2 has 0 digit-initial hyphenated forms).

| bucket | `N-i` | `N-ei` | `N-i` share | `15-i` | `15-ei` |
| --- | ---: | ---: | ---: | ---: | ---: |
| spoken | 1,358 | 1,222 | 0.526 | 38 | 127 |
| news `[NP]` | 53,724 | 7,640 | 0.876 | 3,821 | 1,020 |
| wiki `[NP]` | 17,347 | 4,002 | 0.813 | 811 | 353 |
| official | 81,370 | 6,188 | 0.929 | 3,719 | 988 |
| technical `[NP]` | 1,003 | 52 | 0.951 | 46 | 2 |
| literary | 921 | 13 | 0.986 | 40 | 0 |
| web | 68,751 | 8,491 | 0.890 | - | - |
| **all `[NP]`** | **225,174** | **27,671** | **0.891 [0.889, 0.892]** | **8,540** | **2,509** |
| permissive only | 130,848 | 13,557 | 0.906 | 2,773 | 752 |

The `-ai` and `-jei` variants add 18,081 and 14,818 tokens; `-jei` is concentrated in `gazette`
and `njt`, i.e. it is the official spelling for the 1st of the month.

**Verdict: A preferred** (`15-i`, 89% overall, 93% in official prose) **and
register-conditioned**: the gradient runs literary 99% > technical 95% > official 93% > web 89% >
news 88% > wiki 81% > spoken 53%. Spoken transcription is the only register where `15-ei` is at
parity, which is expected because it transcribes the read form.

Proposed edit - `PND-DATE-006`, **Statement**:
old: `` The `-i` adjective of a day ('of the Nth') is written `N-i` or `N-ei`/`N-ai` (with the possessive), for 1 also `1-ji`, `1-jei`; ... ``
new: `` The `-i` adjective of a day ('of the Nth') is written `N-i` (the default, 89% of tokens and 93% in official prose) or `N-ei`/`N-ai` (with the possessive), for 1 also `1-ji`, `1-jei`; ... ``

Proposed edit - `PND-DATE-006`, **Confidence**: raise to `measured` for the `N-i` default.

## OQ-04-008 - `bele` vs `belé` as the 3sg illative pronoun

Rules: `PND-PERS-003`. Instrument: Webcorpus 2 with the tag filter `[/N|Pro][Ill][3Sg]`, which is
the homograph control: the same strings are also verbal prefixes (`[/Prev]`), and the prefix
reading is 20x more frequent for `bele`.

| reading | `bele` | `belé` | `bele` share |
| --- | ---: | ---: | ---: |
| pronoun `[Ill][3Sg]` | 67,815 | 16,465 | 0.805 [0.802, 0.807] |
| verbal prefix `[/Prev]` | 1,658,556 | 21,419 | 0.987 [0.987, 0.987] |

Project corpus `[NP]`, both readings pooled (the rate index has no tags): spoken 105,998 : 5,485
(0.951), news 83,241 : 1,097 (0.987), official 5,471 : 13 (0.998), literary 10,921 : 3,507 (0.757),
web 419,627 : 10,581 (0.975). `belé` peaks in literary text.

**Verdict: A preferred** (`bele`, 80.5% as a pronoun) **with a literary gradient** for `belé`.
`belé` is not archaic - one pronoun token in five - so both stay generated, with `bele` as the
formal default.

Proposed edit - `PND-PERS-003`, **Examples**: append
`` ; 3sg illative `bele` is the default (67,815 : `belé` 16,465 as a pronoun), `belé` is literary and emphatic. ``

## OQ-05-004 - `mondotok ~ mondtok`, `mondalak ~ mondlak`, `küldelek ~ küldlek`, `?mondsz`

Rules: `VRB-LINK-008`, `VRB-LINK-010`. Instrument: Webcorpus 2 with slot tag filters on lemma
`mond` / `küld`. Cross-checked against Rácz (2026) `Racz2026mondasz` (MIT), which draws the same
four slots from the same list.

| slot | with linking vowel | without | V share |
| --- | ---: | ---: | ---: |
| 2Sg `mond` | `mondasz` 39,605 | `mondsz` 407 | 0.990 [0.989, 0.991] |
| 2Sg `küld` | `küldesz` 9,413 | `küldsz` 38 | 0.996 [0.994, 0.997] |
| 3Pl `mond` | `mondanak` 113,365 | `mondnak` 412 | 0.996 [0.996, 0.997] |
| 1Sg>2 `mond` | `mondalak` 740 | `mondlak` 26 | 0.966 [0.951, 0.977] |
| 1Sg>2 `küld` | `küldelek` 1,871 | `küldlek` 68 | 0.965 [0.956, 0.972] |
| **2Pl `mond`** | **`mondotok` 1,909** | **`mondtok` 2,410** | **0.442 [0.427, 0.457]** |

Register for the one variable slot `[NP]`:

| bucket | `mondtok` | `mondotok` | `mondtok` share |
| --- | ---: | ---: | ---: |
| spoken | 762 | 279 | 0.732 |
| wiki | 40 | 8 | 0.833 |
| technical | 2 | 11 | 0.154 |
| literary | 13 | 50 | 0.206 |
| web | 505 | 448 | 0.530 |

**Verdict: 2Sg, 3Pl and 1Sg>2 - A preferred (linking vowel), near-categorical.
2Pl - register-conditioned**: `mondtok` in speech (73%), `mondotok` in literary and technical prose
(79-85%), parity on the web. `?mondsz` is a genuine 1.0% minority, not ungrammatical.

Proposed edit - `VRB-LINK-008`, **Statement**:
old: `` ... after cluster stems the vowel is preferred but may be omitted colloquially. ``
new: `` ... after cluster stems the vowel is near-obligatory (`küldelek` 1,871 : `küldlek` 68, `mondalak` 740 : `mondlak` 26). ``

Proposed edit - `VRB-LINK-010`, **Examples**:
old: `` `mond` -> `mondotok ~ mondtok` ``
new: `` `mond` -> `mondotok ~ mondtok` (44 : 56 overall; `mondtok` 73% in speech, `mondotok` 79% in literary prose) ``

Proposed edit - `VRB-LINK-003`, **Examples**: append `` ; `mondsz` is attested at 1.0% of the slot (407 : `mondasz` 39,605) and should be flagged, not rejected. ``

## OQ-05-006 - syncopated + linking vowel vs full stem for epenthetic `-ik` verbs

Rules: `VRB-STEM-008`. Instrument: Webcorpus 2 `[Inf]` per lemma.

| lemma | syncopated | full stem | syncopated share |
| --- | ---: | ---: | ---: |
| `ugrik` | `ugrani` 32,345 | `ugorni` 1,040 | 0.969 [0.967, 0.971] |
| `fürdik` | `fürdeni` 25,400 | `fürödni` 13,578 | 0.652 [0.647, 0.656] |
| `söpör` | `söpreni` 30 | `söpörni` 7,361 | 0.004 [0.003, 0.006] |
| `vándorol` | `vándorlani` 13 | `vándorolni` 6,191 | 0.002 [0.001, 0.004] |
| `cselekszik` | `cselekdni` 56 | `cselekedni` 51,155 | 0.001 |
| `emlékezik` | `emlékezeni` 2 | `emlékezni` 66,844 | - |

Register `[NP]` for the one variable item (`fürdeni` : `fürödni`): spoken 1,012 : 849, news 991 :
802, technical 309 : 501, literary 117 : 215, web 4,085 : 3,921. The syncopated form leads in
speech and news, the full stem in literary and technical prose - a weak gradient, all buckets
within 0.4-0.6.

**Verdict: item-lexical.** `ugrik` takes the syncopated infinitive categorically, `söpör`,
`vándorol`, `cselekszik` the full stem categorically, and only `fürdik` is genuinely variable
(65:35, with a weak register gradient).

Proposed edit - `VRB-STEM-008`, **Examples**:
old: `` `fürdik` -> `fürdeni ~ fürödni`, ``
new: `` `fürdik` -> `fürdeni` 25,400 ~ `fürödni` 13,578 (the only item in the class that genuinely varies; `ugrani` 32,345 : `ugorni` 1,040, `söpörni` 7,361 : `söpreni` 30), ``

## OQ-05-010 - `lettem volna` vs `voltam volna`

Rules: `VRB-COND-004`. Instrument: `kwic.py --regex` over `small,hunsum,subtitles,hq` (a
two-word pattern that no word list holds).

| bucket | `lettem volna` | `voltam volna` | | `lett volna` | `volt volna` |
| --- | ---: | ---: | --- | ---: | ---: |
| spoken `[NP]` | 5,663 | 6 | | 56,479 | 5 |
| news `[NP]` | 1,518 | 1 | | 64,417 | 10 |
| wiki `[NP]` | 276 | 2 | | 17,015 | 7 |
| official | - | - | | 3,245 | 0 |
| literary | 500 | 31 | | 10,039 | 423 |
| web | 2,263 | 2 | | 90,738 | 85 |
| **all `[NP]`** | **10,297** | **44** | 0.9957 [0.9943, 0.9968] | **245,186** | **535** |
| permissive only | 2,819 | 33 | 0.9884 | 107,525 | 508 |

79% of all `volt volna` tokens and 70% of all `voltam volna` tokens sit in `literature`.

**Verdict: A preferred, near-categorical, with the minority variant register-conditioned**:
`voltam/volt volna` is a literary archaism at 0.4% overall and 4% inside `literature`.

Proposed edit - `VRB-COND-004`, **Examples**: append
`` ; the copula uses `lett volna`, not `volt volna` (245,186 : 535, and 79% of the `volt volna` tokens are in nineteenth-century literature). ``

## OQ-05-011 - are plural present sz-d-v forms archaic or merely less frequent?

Rules: `VRB-STEM-012`. Instrument: Webcorpus 2 `Prs.NDef.3Pl` per lemma, plus the project
register split.

| lemma | `-usz-` form | `-sz-` form | `-usz-` share |
| --- | ---: | ---: | ---: |
| `alszik` | `alusznak` 2,311 | `alszanak` 33,580 | 0.064 [0.062, 0.067] |
| `fekszik` | `feküsznek` 1,781 | `fekszenek` 20,974 | 0.078 [0.075, 0.082] |
| `haragszik` | `haragusznak` 354 | `haragszanak` 3,340 | 0.096 [0.087, 0.106] |
| `nyugszik` | `nyugosznak` 887 | `nyugszanak` 12,611 | 0.066 [0.062, 0.070] |

Register `[NP]`, `alusznak` : `alszanak` and `feküsznek` : `fekszenek`:

| bucket | `alusznak` | `alszanak` | share | `feküsznek` | `fekszenek` | share |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| spoken | 38 | 3,516 | 0.011 | 0 | 906 | 0.000 |
| news | 0 | 2,168 | 0.000 | 0 | 1,655 | 0.000 |
| wiki | 3 | 315 | 0.009 | 40 | 1,350 | 0.029 |
| technical | 522 | 1,240 | 0.296 | 1,575 | 5,238 | 0.231 |
| literary | 495 | 305 | 0.619 | 362 | 686 | 0.345 |
| web | 789 | 10,288 | 0.071 | 346 | 7,787 | 0.042 |

**Verdict: register-conditioned - archaic, not merely less frequent.** The gradient is steep and
monotone: literary 34-62% > technical 23-30% > web 4-7% > spoken 0-1% > news 0%. A form that is
absent from news and speech but carries a third of literary tokens is an archaism.

Proposed edit - `VRB-STEM-012`, **Examples**:
old: `` `alszotok ~ alusztok`, `alszanak ~ alusznak`, ``
new: `` `alszotok ~ alusztok`, `alszanak ~ %alusznak` (6.4% overall; 62% of literary tokens but 0% of news and 1% of spoken - archaic), ``

Proposed edit - `VRB-STEM-012`, **Confidence**: add the register label `archaic` to the `-usz-`
plural alternants and mark them `measured`.

## OQ-06-003 - harmony of case suffixes after `-né` on back stems

Rules: `DER-NOUN-024`. Instrument: Webcorpus 2, 13 back-stem `-né` lemmas, all case forms.

| lemma | `-nak` | `-nek` | `-val` | `-vel` | `-ról` | `-ről` |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `lászlóné` | 1,153 | 13 | 322 | 3 | 45 | 2 |
| `istvánné` | 982 | 2 | 2,064 | 0 | 33 | 0 |
| `jánosné` | 736 | 6 | 242 | 6 | 36 | 0 |
| `sándorné` | 668 | 15 | 173 | 4 | 45 | 0 |
| `lajosné` | 375 | 7 | 141 | 1 | 14 | 1 |
| `zoltánné` | 360 | 0 | 109 | 1 | 15 | 0 |
| `gáborné` | 315 | 2 | 140 | 6 | 11 | 0 |
| `andrásné` | 261 | 1 | 74 | 1 | 15 | 3 |
| `pálné` | 257 | 0 | 100 | 1 | 28 | 0 |
| `károlyné` | 239 | 0 | 98 | 1 | 9 | 1 |
| `antalné` | 136 | 3 | 36 | 0 | 5 | 0 |
| `tamásné` | 95 | 4 | 44 | 0 | 2 | 0 |
| `szabóné` | 55 | 0 | 48 | 1 | 3 | 0 |
| **total** | **5,632** | **53** | **3,591** | **25** | **261** | **7** |

Back share across all three cases: 9,484 : 85 = 0.9911 [0.9890, 0.9928]. Publishable.

**Verdict: A preferred, near-categorical.** `-né` is harmonically transparent: back stems keep
back suffixes (`Szabónénak`). The chapter's claim was an assumption; it is now measured.

Proposed edit - `DER-NOUN-024`, **Statement**:
old: `` ... has no linking vowel and is a transparent neutral vowel for harmony. ``
new: `` ... has no linking vowel and is a transparent neutral vowel for harmony (back stems take back suffixes in 99.1% of 9,569 case tokens over 13 lemmas: `Szabónénak`, `Istvánnéval`). ``

Proposed edit - `DER-NOUN-024`, **Confidence**: `measured`.

## OQ-07-016 - spoken forms of signs with two readings

Rules: `ORT-ABBR-010`, `ORT-ABBR-018`. Instrument: `kwic.py --regex` over `small,hunsum` and
`small,hunsum,subtitles,hq`. These are hyphenated tokens, absent from both word lists.

| sign | full-name reading | short-name reading | full-name share |
| --- | ---: | ---: | ---: |
| `kg` | `kg-ot`/`kg-os` 19,315 + `kg-mal` 867 | `kg-t`/`kg-s` 234 | 0.988 [0.986, 0.989] |
| `dkg` | `dkg-ot`/`dkg-os` 238 | `dkg-t` 6 | 0.975 [0.947, 0.989] |
| `cm` | `cm-es` 9,294, `cm-t` 1,990, `cm-rel` 1,309 | `cm-terrel` 0 | - |

All figures `[NP]`; the permissive subsets are `kg` 11,432 : 66 and `cm-es` 1,920.

| acronym | front | back | front share |
| --- | ---: | ---: | ---: |
| `CNN-nek` : `CNN-nak` `[NP]` | 1,738 | 1 | 0.9994 [0.9967, 0.9999] |
| `CNN-nel` : `CNN-nal` `[NP]` | 53 | 0 | 1.000 |
| `CNN-hez` : `CNN-hoz` `[NP]` | 28 | 0 | 1.000 |

**Verdict: A preferred for both.** `kg`/`dkg` take the full-name reading (`kilogrammot`,
`dekagrammot`) at 98-99%; `CNN` takes front harmony regardless of which letter-name inventory the
reader uses, so the Hungarian/English distinction has no orthographic consequence.

Proposed edit - `ORT-ABBR-010`, **Examples** table: add rows
`` `kg` -> `kg-ot`, `kg-os`, `kg-mal` (kilogramm reading, 98.8%; `kg-t` 234 = kiló reading) ``
and `` `dkg` -> `dkg-ot`, `dkg-os` (97.5%) ``.

Proposed edit - `ORT-ABBR-018`, **Statement**: append
`` ; a foreign acronym read either way takes the same suffix where both inventories end in a front name (`CNN-nek` 1,738 : `CNN-nak` 1), so the choice is orthographically inert for `CNN`. ``

Proposed edit - `PND-MEAS-003`, **Examples**: append
`` ; in edited text the full-name reading dominates: `kg-ot` : `kg-t` = 19,315 : 191. ``

## OQ-09-001 - conjugation with coordinated objects of mixed definiteness

Rules: `SYN-DEF-016`. Instrument attempted: `kwic.py --regex` for coordinated object patterns.
No probe separates the two conjugations without a parser: the 3sg past definite and indefinite
forms that disambiguate (`meglátta` / `meglátott`) are not adjacent to the coordination in enough
cases, and the regexes that do fire cannot confirm that both conjuncts are objects.

**Verdict: insufficient evidence.** Needs a parsed corpus (UD Szeged or a dependency parse of the
project corpus), not a concordancer.

Proposed edit: none. `SYN-DEF-016` **Validation** should gain `needs:parsed-corpus` so the
question is not re-attempted with string search.

## OQ-09-006 - nearest-conjunct agreement with `sem ... sem` subjects

Rules: `SYN-AGR-007`. Instrument: `kwic.py --regex` over 11 sources.

| pattern | hits |
| --- | ---: |
| `sem X, sem Y nem V-nak/-nek` (plural present/conditional) `[NP]` | 516 |
| `sem X, sem Y nem V-ott/-ett/-ött` (singular past) `[NP]` | 864 |

The two patterns are different tenses, so the counts are not a ratio: the regex cannot hold the
tense constant while varying the number, because the singular and plural past differ only in the
final consonant and the plural present in `-nak` is homographic with the dative.

**Verdict: insufficient evidence.** Plural agreement with `sem ... sem` is clearly attested (516
hits), which supports the `%` marking in the rule rather than a star, but the share is not
measurable this way.

Proposed edit - `SYN-AGR-007`, **Examples**:
old: `` `Sem Péter, sem Anna nem jött el.` ~ `%Sem Péter, sem Anna nem jöttek el.`; ``
new: `` `Sem Péter, sem Anna nem jött el.` ~ `%Sem Péter, sem Anna nem jöttek el.` (the plural pattern is attested 516 times in the project corpus; the share is not measurable without a parser); ``

## OQ-09-012 - zero accusative on 1pl/2pl possessed objects

Rules: `SYN-GOV-009`. Instrument: `kwic.py --regex` with a closed verb list
(`megnéztük|láttuk|elolvastuk|megettük|eladtuk|megvettük`) plus `a` plus a 1pl possessed noun,
which fixes person, number and object status.

| bucket | zero (`... a házunk`) | accusative (`... a házunkat`) | zero share |
| --- | ---: | ---: | ---: |
| spoken `[NP]` | 4 | 27 | |
| news `[NP]` | 6 | 10 | |
| literary | 3 | 0 | |
| web | 5 | 12 | |
| **all `[NP]`** | **19** | **49** | **0.279 [0.187, 0.396]** |
| permissive only | 8 | 12 | |

n = 68 overall, but only six verbs and no single bucket clears the floor for both variants.

**Verdict: insufficient evidence for a rate**, but the pattern is **attested** (19 hits across
`literature`, `bloghu` and web), so the `%` on `%Eladtuk a házunk.` is right and a star would be
wrong.

Proposed edit - `SYN-GOV-009`, **Statement**: no change. **Examples**:
old: `` `%Eladtuk a házunk.`; ``
new: `` `%Eladtuk a házunk.` (attested; 19 hits against 49 suffixed in a closed six-verb probe, too few to quote a rate); ``

## OQ-02-009 - conservative vs innovative final high vowel length

Rules: `PH-SYL-013`. No corpus instrument exists: the contrast is in pronunciation and Hungarian
orthography fixes the length normatively, so spelling counts measure the norm, not the speakers.
Webcorpus 2 spelling variants (`falu` : `falú`, `kapu` : `kapú`) count typos.

**Verdict: insufficient evidence.** Needs the pronunciation survey the question already names.

Proposed edit: none. `PH-SYL-013` **Validation**: add `needs:pronunciation-survey` and record that
a corpus cannot settle it.

## OQ-02-016 - `fővök` vs `fövök`

Rules: `PH-VSTEM-001`, `PH-VSTEM-002`. Instrument: Webcorpus 2, all `v`-alternant forms of lemma
`fő` (verb). The project rate index has neither form: both are below the 400k cut-off in every
source, which is why the question was unanswerable before.

| form pair | non-shortening `fő-` | shortening `fö-` |
| --- | ---: | ---: |
| 1Sg | `fővök` 22 | `fövök` 9 |
| 1Pl | `fővünk` 16 | `fövünk` 17 |
| participle | `fővő` 671 | `fövő` 909 |
| `-és` noun | `fővés` 225 | `fövés` 249 |
| **total** | **934** | **1,184** |

`fő-` share 0.441 [0.420, 0.462], n = 2,118. Publishable.

**Verdict: both current (0.44 : 0.56).** SMNy3's `%` on both alternants is confirmed; there is no
standard preference, and the split runs by paradigm cell rather than by speaker (1Sg favours
`fővök`, the participle favours `fövő`).

Proposed edit - `PH-VSTEM-001`, **Examples**:
old: `` `fő` (verb; non-shortening variant, speaker-dependent beside shortening `fövök`, `PH-VSTEM-002`) ``
new: `` `fő` (verb; non-shortening variant at 44% of 2,118 v-alternant tokens, beside shortening `fövök`, `PH-VSTEM-002`; the split is per cell: `fővök` 22 : `fövök` 9 but `fövő` 909 : `fővő` 671) ``

Proposed edit - both rules, **Confidence**: `measured`, variation retained.

## OQ-03-007 - comparative of `hű`

Rules: `NOM-ADJ-005`. Instrument: Webcorpus 2 `[/Adj][_Comp/Adj]`. Homograph control: `hívebb` is
tagged under lemma `hív`, the etymological doublet, not under `hű`; filtering on lemma `hű` alone
returns 0 for `hívebb` and would have produced a false categorical verdict.

| form | lemma | Nom | Acc |
| --- | --- | ---: | ---: |
| `hűbb` | `hű` | 641 | 4 |
| `hívebb` | `hív` | 547 | 12 |

`hűbb` share 0.541 [0.512, 0.569], n = 1,192. Publishable.

Register `[NP]`, project corpus: technical 171 : 379, literary 43 : 93, wiki 20 : 0 - `hívebb`
leads 2:1 in academic and literary prose, `hűbb` leads on the web.

**Verdict: both current (54 : 46), register-conditioned.** `hűbb` is the general form, `hívebb`
the edited-prose form. Neither is unattested, which supersedes the "neither attested" note.

Proposed edit - `NOM-ADJ-005`, **Examples**: add
`` `hűbb` 641 ~ `hívebb` 547 (both current; `hívebb` leads 2:1 in literary and academic prose) ``.

## OQ-03-011 - harmony after an R stem + `-é`

Rules: `NOM-CASE-013`. Instrument: Webcorpus 2, item level and pattern level.

| contrast | count |
| --- | ---: |
| `kőéhez` | 5 |
| `kőéhöz` | 0 |
| all forms in `-éhez` | 4,735,027 |
| all forms in `-éhöz` | 300 |

The item is below the floor (n = 5), but the pattern is not: `-éhöz` is 0.006% of the allative
after `-é`, and inspection of the 15 most frequent `-éhöz` types shows they are archaic or
dialectal spellings of ordinary possessives (`föltételéhöz`, `szivéhöz`, `nevéhöz`), not R-stem
forms.

**Verdict: A preferred** at pattern level (`-é` + `-hez`, unrounded front); `-höz` after `-é` is
not standard Hungarian. Item-level evidence remains **insufficient**.

Proposed edit - `NOM-CASE-013`, **Examples**: append
`` ; after the possessive `-é` the allative is always `-hez` (`kőéhez`; `-éhez` 4,735,027 : `-éhöz` 300, and the `-öz` tokens are archaic spellings, not rounding harmony). ``

## OQ-03-014 - `borjút ~ borjat`, `ajtói ~ ajtajai`

Rules: `NOM-STEM-012`, `NOM-STEM-013`. Instrument: Webcorpus 2 with `[Acc]` / `[Pl]` filters on
lemma `borjú`, and the unfiltered pair for `ajtó`.

| contrast | A | B | A share |
| --- | ---: | ---: | ---: |
| `borjút` : `borjat` `[Acc]` | 4,195 | 636 | 0.868 [0.859, 0.878] |
| `borjúk` : `borjak` `[Pl]` | 361 | 3,171 | 0.102 [0.093, 0.113] |
| `ajtói` : `ajtajai` | 2,802 | 2,388 | 0.540 [0.526, 0.553] |

Register `[NP]` for the plural: technical 1 : 2,495, official 0 : 219, news 0 : 195, wiki 0 : 160,
literary 123 : 32, spoken 26 : 115.

**Verdict: cell-split, not item-split.** `borjú` keeps the `ú` in the accusative (87% `borjút`)
and drops it in the plural (90% `borjak`); the literary register is the only one that uses
`borjúk`. `ajtói` and `ajtajai` are **both current** at 54 : 46.

Proposed edit - `NOM-STEM-012`, **Statement**:
old: `` ... the superessive always keeps it, the accusative varies. ``
new: `` ... the superessive always keeps it; the accusative keeps it (`borjút` 4,195 : `borjat` 636) while the plural drops it (`borjak` 3,171 : `borjúk` 361). ``

Proposed edit - `NOM-STEM-013`, **Examples**:
old: `` `erői` [ATT 9]; ``
new: `` `erői` [ATT 9]; `ajtói` 2,802 ~ `ajtajai` 2,388 (both current); ``

## OQ-03-017 - 3sg of `néne`

Rules: `NOM-POSS-009`. Instrument: Webcorpus 2 `[/N][Poss.3Sg]` on lemma `néne`.

| form | count |
| --- | ---: |
| `nénje` | 2,383 |
| `nénéje` | 12 |

`nénje` share 0.995 [0.991, 0.997], n = 2,395. Publishable. Project register `[NP]`: `nénje`
literary 479, web 743, technical 462, wiki 189, spoken 111; `nénéje` 0 everywhere.

**Verdict: A preferred, near-categorical.** `nénéje` exists (12 tokens) but is not the modern
form. The oracles' ignorance of `nénéje` was correct behaviour.

Proposed edit - `NOM-POSS-009`, **Examples** table: add
`` `néne` | `nénje` 2,383 (modern) | `nénéje` 12 (marginal) ``.

## OQ-05-005 - `edz` past non-3sg and 2sg

Rules: `VRB-PAST-007`, `VRB-LINK-008`. Instrument: Webcorpus 2, lemma `edz`, tags `Pst` and `2Sg`.

| contrast | A | B | A share |
| --- | ---: | ---: | ---: |
| `edzettem` : `edztem` | 8,406 | 24 | 0.997 [0.996, 0.998] |
| `edzettünk` : `edztünk` | 3,809 | 3 | 0.999 [0.998, 1.000] |
| `edzel` : `edzesz` | 6,942 | 11 | 0.998 [0.997, 0.999] |

**Verdict: A preferred, near-categorical, for both.** The current encoding is correct: `dz` is
heavy for the past linking vowel and counts as a sibilant for the 2sg (`-Ol`).

Proposed edit - `VRB-PAST-007`, **Examples**: append
`` ; `edz` -> `edzettem` 8,406 : `edztem` 24 (heavy). ``

Proposed edit - `VRB-LINK-003`, **Examples**: append
`` ; `edz` -> `edzel` 6,942 : `edzesz` 11 (`dz` sibilant). ``

Proposed edit - both rules, **Confidence**: `measured`.

## OQ-05-007 - non-3sg past of `m`-final epenthetic verbs (`terem`)

Rules: `VRB-PAST-009`. Instrument: Webcorpus 2. **Homograph control fails here**: the list assigns
both `termettem` and `teremtem` to lemma `terem`, and `teremtem` is in fact dominated by the
distinct verb `teremt` ('create') in its 1sg definite present. The tag cannot separate them and no
second analysis is listed, so the string ratio is unusable.

| form | count under lemma `terem` | usable? |
| --- | ---: | --- |
| `termettem` | 272 (`Pst.NDef.1Sg`) | yes - the string has no other reading |
| `termettünk` | 202 | yes |
| `teremtem` | 1,319 (`Pst.NDef.1Sg`) | no - dominated by `teremt` 1Sg definite present |
| `teremtünk` | 90 | no - dominated by `teremt` 1Pl present |

**Verdict: A preferred, with a homograph caveat.** The syncopated-stem past (`termettem`,
`termettünk`) is robustly attested with 474 unambiguous tokens; the full-stem alternative has no
attestation that can be separated from `teremt`. Generate `termettem`, do not generate
`teremtem`.

Proposed edit - `VRB-PAST-009`, **Examples**: append
`` ; `terem` -> `termettem` 272, `termettünk` 202 (the competing string `teremtem` cannot be counted, being the 1sg definite of `teremt`). ``

## OQ-05-009 - adverbial participles of `eszik`, `iszik`, `hisz`

Rules: `VRB-ADVP-001`. Instrument: Webcorpus 2 with the tag filter `[_AdvPtcp/Adv]`, which is the
only way to separate these from massively more frequent homographs.

| form | total tokens | of which `[_AdvPtcp/Adv]` | dominant reading of the string |
| --- | ---: | ---: | --- |
| `éve` | 2,434,990 | **0** | `év` `[Poss.3Sg]` |
| `eve` | 5,359 | 0 | noise, unaccented `éve` |
| `iva` | 1,955 | 149 | proper noun `Iva` (1,512) |
| `ivva` | 8 | 8 | - |
| `híve` | 49,936 | 8 | `hív` `[Poss.3Sg]` 'his follower' |
| `hívén` | 398 | 0 | - |

The 149 `iva` participle tokens are split between lemma `i` (130) and lemma `iva` (19), which is
itself a mis-analysis, so even that figure is soft.

**Verdict: marginal confirmed.** `éve` as an adverbial participle of `eszik` is **unattested**
(0 of 2.4M tokens of the string); `iva` and `híve` reach 149 and 8 tokens, both below or near the
floor once the tagging noise is removed.

Proposed edit - `VRB-ADVP-001`, **Examples**: append
`` ; `eszik` has no current adverbial participle (`éve` 0 tokens tagged as one out of 2.4M), `iszik` -> `%iva` (149) and `hisz` -> `%híve` (8) are marginal. ``

Keep the existing `marginal` marking; the measurement supports it.

## OQ-05-014 - `hangzani ~ hangozni`, `javallott ~ javallt`, `sugallt ~ sugallott`

Rules: `VRB-STEM-006`, `VRB-PAST-007`. Instrument: Webcorpus 2. For the `-All` verbs the 3sg past
is a participle homograph, so the 1sg past is used as the clean probe.

| contrast | A | B | A share |
| --- | ---: | ---: | ---: |
| `hangzani` : `hangozni` `[Inf]` | 3,000 | 564 | 0.842 [0.829, 0.853] |
| `javallottam` : `javalltam` `Pst` | 76 | 14 | 0.844 [0.756, 0.905] |
| `sugalltam` : `sugallottam` `Pst` | 347 | 2 | 0.994 [0.979, 0.998] |
| `sugallt` : `sugallott` `Pst.NDef.3Sg` | 3,665 | 42 | 0.989 [0.985, 0.992] |

Register `[NP]` for `hangzani` : `hangozni`: spoken 1,533 : 136, news 81 : 0, technical 167 : 0,
web 720 : 0 - `hangozni` occurs almost only in transcribed speech.

**Verdict: item-lexical, with opposite values inside the same `-All` class.** `javall` takes the
`-Ott` past (84%), `sugall` takes bare `-t` (99%). `hangzik` takes the syncopated infinitive
(84%), and its full-stem variant is spoken-only.

Proposed edit - `VRB-STEM-006`, **Statement**: append
`` ; the `-All` verbs split lexically: `javallottam` 76 : `javalltam` 14 against `sugalltam` 347 : `sugallottam` 2. ``

Proposed edit - `VRB-STEM-006`, **Examples**: append
`` ; `javall` -> `javallott`; `sugall` -> `sugallt`. ``

## OQ-10-015 - `látsszék ~ lássék`

Rules: `REP-SPELL-007`. Instrument: Webcorpus 2.

| form | count |
| --- | ---: |
| `lássék` | 1,572 |
| `látsszék` | 349 |
| `látszódjék` | 1,579 |

`lássék` share against `látsszék` 0.818 [0.800, 0.835], n = 1,921. Publishable.
Register `[NP]`: `lássék` technical 606, literary 227, web 1,216, news 193, spoken 62;
`látsszék` technical 4, literary 10, wiki 4, web 0.

**Verdict: B preferred** (`lássék`, 82%) **and both current**. `látsszék` is a real minority form,
not a spelling error, so the repair rule must not "fix" it. `látszódjék` is as frequent as
`lássék` and is the neutral modern choice.

Proposed edit - `REP-SPELL-007`, **Exceptions** (add):
`` `látsszék` is a standard minority variant (349 : `lássék` 1,572) and is not repaired; `látszódjék` (1,579) is the neutral alternative. ``

The AkH12 §78 check the question asks for is still outstanding; the corpus decides usage, not
normativity.

---

# source-conflict arbitrated by usage

## OQ-05-015 - are long vowel + `t` stems heavy?

Rules: `VRB-LINK-003`, `VRB-STEM-006`. KVF98 p. 290 against SMNy3 p. 735 and T04.
Instrument: Webcorpus 2 `3Pl` / `Pst` per lemma.

| lemma | with linking vowel | without | V share | verdict |
| --- | ---: | ---: | ---: | --- |
| `vét` | `vétenek` 2,424 | `vétnek` 20 | 0.992 [0.987, 0.995] | heavy |
| `vét` | `vétettem` 4,120 | `véttem` 0 | 1.000 [0.999, 1.000] | heavy |
| `bocsát` | `bocsátanak` 21,461 | `bocsátnak` 50 | 0.998 [0.997, 0.998] | heavy |
| `lát` | `látanak` 4 | `látnak` 139,116 | 0.00003 | **not** heavy |
| `tát` | `tátanak` 1 | `tátnak` 1 | - | below floor |

Side result: `bocsátott` 32,745 : `bocsájtott` 1,923, so the `j`-inserting spelling is a 5.5%
minority.

**Verdict: both sources are partly right; heaviness is lexical.** KVF98 holds for `vét` and
`bocsát`; SMNy3/T04 hold for `lát`, which is categorically light. `tát` remains **insufficient
evidence** (n = 2). The previous "1 vs 0" support is superseded by n = 2,444 and n = 21,511.

Proposed edit - `VRB-LINK-003`, **Exceptions** (add):
`` long vowel + `t` stems are heavy lexically, not as a class: `vét` (`vétenek` 2,424 : `vétnek` 20) and `bocsát` (`bocsátanak` 21,461 : `bocsátnak` 50) are heavy, `lát` is light (`látnak` 139,116 : `látanak` 4). ``

Proposed edit - `VRB-STEM-006`, **Lexical input**: add the flag `heavy_long_vowel_t` set true for
`vét`, `bocsát` and false for `lát`.

## OQ-06-011 - does `-An` attach productively to `-i` adjectives?

Rules: `DER-ADV-001`. SMNy3 excludes it; KVF98 lists examples.
Instrument: Webcorpus 2 type count over `xpostag` containing `[_Manner/Adv]`, lemma ending in
`i`, form ending in `-ian`/`-ien`.

| measure | value |
| --- | ---: |
| distinct `-i` lemmas with an `-An` adverb | 4,868 |
| tokens | 282,678 |
| distinct lemmas with freq >= 20 | **400** |
| tokens in those | 270,605 |
| distinct `-i` lemmas with a `-lAg` adverb | 120 |
| `-lAg` tokens | 2,808 |

Top native items: `kíváncsian` 66,536, `forradalmian` 23,999, `mesterien` 18,727, `drámaian`
18,682, `pokolian` 6,622, `istenien` 5,663, `egyedien` 5,013, `művészien` 4,863, `érzékien` 3,711,
`ördögien` 2,727, `gyermekien` 2,588, `költőien` 2,524, `festőien` 1,854, `emberien` 1,675,
`szolgaian` 1,568, `barátian` 1,305. English strings that the pattern also catches (`russian`,
`italian`, `dorian`, `lucian`, `varian`) were excluded by inspection of the top 25; they account
for about 6% of the top-25 tokens and none of the 400-lemma count is dominated by them.

Related finding for `OQ-06-016` and `PH-LEN-002`: `-beli` also takes both suffixes -
`mesebelien` 324, `vérbelien` 57 against `számbelileg` 957, `időbelileg` 925, `térbelileg` 803.

**Verdict: KVF98 is right, SMNy3 is wrong. A preferred: `-An` is productive on `-i` adjectives**
(400 lemmas above the floor, 270,605 tokens), and it is 100x more frequent than `-lAg` on the same
bases. Generation should be enabled.

Proposed edit - `DER-ADV-001`, **Statement**:
old: `` `-An` forms manner adverbs from adjectives of all kinds (root, participial, `-(O)s`, `-(j)Ú`, comparative) except `-i`, `-beli`, `-nyi` adjectives and privatives; ``
new: `` `-An` forms manner adverbs from adjectives of all kinds (root, participial, `-(O)s`, `-(j)Ú`, comparative), including `-i` and `-beli` adjectives (400 `-i` lemmas above 20 tokens, 270,605 tokens: `kíváncsian`, `mesterien`, `drámaian`, `költőien`, `mesebelien`), but not `-nyi` adjectives or privatives; ``

Proposed edit - `DER-ADV-001`, **Predictability**: `rule` with the `-i` restriction removed;
**Confidence**: `measured`.

## OQ-06-016 - SMNy3 internal contradictions

Rules: `DER-ADJ`, `DER-VERB`. Instrument: Webcorpus 2.

| claim in dispute | measurement | reading |
| --- | --- | --- |
| `-lAg` after `-beli` (p. 125 allows, p. 166 excludes) | `számbelileg` 957, `időbelileg` 925, `térbelileg` 803, `szóbelileg` 378, `ténybelileg` 212, `hitbelileg` 84, `pénzbelileg` 80, `írásbelileg` 43 - 8 lemmas above the floor | p. 125 is right |
| passive `-(t)At[ik]` (table 36 excludes, usage has it) | `adatik` 29,912, `mondatik` 1,962, `tartatik` 1,543, `említtetik` 1,096, `hivatik` 192 | usage is right; the suffix is current in legal and archaic register |
| verbal `-Ul` (table 36 vs p. 161) | not measured - the contrast is about epenthetic behaviour, not existence | open |

**Verdict: two of three arbitrated.** `-lAg` after `-beli` is productive; passive `-(t)Atik` is
attested with five lemmas over the floor. The verbal `-Ul` item is **insufficient evidence**.

Proposed edit - `DER-ADJ` `-lAg` rule, **Statement**: remove the `-beli` exclusion and add
`` (`számbelileg` 957, `időbelileg` 925, `térbelileg` 803). ``

Proposed edit - `DER-VERB` passive rule, **Statement**: add the register label
`` archaic/legal, current (`adatik` 29,912, `mondatik` 1,962) `` rather than excluding the suffix.

## OQ-07-013 - Russian transliteration positional rules

Rules: `ORT-FOR-018`. **Not corpus-arbitrable.** A corpus shows which spellings writers use, not
which positional rule CSNMH85 states, and the Hungarian spellings of Russian names in the corpus
are themselves downstream of the same Wikipedia-mediated summary the table rests on.

**Verdict: insufficient evidence.** Needs CSNMH85.

Proposed edit: none. `ORT-FOR-018` **Validation**: add `needs:primary-source` and remove `corpus`
if present.

## OQ-09-018 - indefinite agreement with `egy` + possessed object

Rules: `SYN-DEF-009`. KVF98 silent, Bartos calls it dialectal.
Instrument: `kwic.py --regex` with a closed eight-verb list, `egy`, and a `-ját`/`-jét` possessed
object, which fixes the construction exactly.

| pattern | hits `[NP]` |
| --- | ---: |
| indefinite (`meglátogatott egy barátját`) | 3 |
| definite (`meglátogatta egy barátját`) | 15 |

n = 18, below the floor for the indefinite variant. Contexts confirm both readings are the target
construction (`Temesváron meglátogatta egy barátját`).

**Verdict: insufficient evidence for a rate**, but the indefinite pattern is **attested** in
edited wiki prose, so `%` is the right marking and Bartos's "dialectal" is too strong.

Proposed edit - `SYN-DEF-009`, **Examples**:
old: `` `Meglátogatta egy barátját.` ~ `%Meglátogatott egy barátját.` ``
new: `` `Meglátogatta egy barátját.` (15) ~ `%Meglátogatott egy barátját.` (3; attested in edited prose, too few to quote a rate) ``

## OQ-03-003 - may `-ként` follow `-é`?

Rules: `NOM-MODEL-003`. SMNy3 gives contradictory judgements.
Instrument: Webcorpus 2, every form ending in `-éként`. Homograph control: the pattern catches
`-é`-final **stems** as well, so the 20 most frequent hits were inspected.

| item | count | analysis |
| --- | ---: | --- |
| `egyéként` | 895 | `egy` `[AnP][EssFor:ként]` - anaphoric possessive, not a possessive `-é` on a noun |
| `rituáléként`, `kupéként`, `büféként`, `kávéként`, `attaséként`, `püréként`, `fináléként`, `kanapéként`, `zseléként`, `tévéként` | 104-283 each | `-é`-final stems, not the possessive `-é` |
| `házéként` | **0** | the target |

**Verdict: B preferred - the combination is unattested.** Every `-éként` type in the list is either
an `-é`-final stem or the anaphoric-possessive pronoun `egyé`. The restriction stands.

Proposed edit - `NOM-MODEL-003`, **Conditions** (restriction list): keep the `-ként` after `-é`
restriction and annotate it
`` (`*házéként`: 0 tokens; every `-éként` type in Webcorpus 2 is an `-é`-final stem or the anaphoric `egyéként`). ``

## OQ-03-015 - is `pléh` a variable v-stem?

Rules: `PH-CONS-005`. AkH12 §41/§74 treat it as vowel-final; SMNy3 p. 789 reports speaker
variation.
Instrument: Webcorpus 2.

| form | count | reading |
| --- | ---: | --- |
| `pléhvel` | 71 | consonant-final, no assimilation |
| `plével` | 0 | vowel-final v-stem |
| `pléhet` | 55 | consonant-final accusative |
| `plét` | 6 | vowel-final accusative |

`pléhvel` share 1.000 [0.949, 1.000], n = 71; `pléhet` share 0.902 [0.802, 0.954], n = 61. Both
above the floor.

**Verdict: A preferred - `pléh` behaves as consonant-final in current usage.** The `plé-` stem is
unattested in the instrumental and marginal (6) in the accusative. SMNy3's reported variation is
not visible in a 9e9-token corpus; the AkH12 `pléhvé` form the chapter cites is not the instrumental
and does not conflict.

Proposed edit - `PH-CONS-005`, **Examples**:
old: `` `csehvel`, `pléhvé` [AkH12 §41]; ``
new: `` `csehvel`, `pléhvé` [AkH12 §41] but `pléhvel` 71 : `plével` 0, `pléhet` 55 : `plét` 6 in use; ``

Proposed edit - `PH-CONS-005`, **Statement**: move `pléh` out of the `cseh` class into the `doh`
class for the instrumental, keeping the AkH12 form as a variant.

## OQ-03-016 - is `Lajosként` odd?

Rules: `NOM-MODEL-003`. SMNy3 Kiefer p. 495 marks it odd; UniMorph and native intuition do not.
Instrument: Webcorpus 2, six common given names (the list is case-folded).

| form | count |
| --- | ---: |
| `jánosként` | 525 |
| `péterként` | 425 |
| `máriaként` | 342 |
| `istvánként` | 296 |
| `annaként` | 121 |
| `lajosként` | 116 |
| **total** | **1,825** |

Every one clears the floor. The project rate index agrees `[NP]`: `Lajosként` 104 entity tokens
(news 23, technical 25, wiki 13, web 43).

**Verdict: B preferred - `-ként` on given names is regular.** SMNy3's `?` is not supported by
usage.

Proposed edit - `NOM-MODEL-003`, **Conditions**: remove any restriction on `-ként` after a given
name and annotate
`` (`Lajosként` 116, `Jánosként` 525, `Péterként` 425 - regular, contra SMNy3 Kiefer p. 495). ``

## OQ-04-012 - does the possessive of `húsz` keep the long vowel?

Rules: `PND-NUMINF`. UniMorph + AkH12 §27 a) give long; KVF98 p. 266 gives short.
Instrument: Webcorpus 2, all `[/Num][Poss...]` forms of lemma `húsz`, excluding the `-Ad` fraction
derivative (`huszad-`), which shortens by a separate rule.

| form | long | short |
| --- | ---: | ---: |
| 1Pl Nom | `húszunk` 7 | `huszunk` 1 |
| 3Pl Nom | `húszuk` 3 | `huszuk` 2 |
| 3Pl Acc | `húszukat` 3 | `huszukat` 0 |
| 3Pl Dat | `húszuknak` 2 | `huszuknak` 0 |
| **subtotal** | **15** | **3** |
| wider paradigm (`húszán` 44, `húsza` 29, `húszával` 18, `húszunkat` 16) | 107 | 0 |

The exact pair is at the floor (n = 18, long share 0.833 [0.586, 0.948]); the wider `[/Num][Poss]`
paradigm is 107 : 0 long.

**Verdict: A preferred (long `húsz-`)**, at the floor for the `-uk` cell but categorical in the
rest of the possessive paradigm. AkH12/UniMorph are right; KVF98's `huszuknak` has 0 attestations.

Proposed edit - `PND-NUMINF`, **Examples**: add
`` `húsz` -> `húszuk`, `húszunk`, `húszuknak` (long; the wider possessive paradigm is 107 : 0 long, `huszuknak` 0) ``.

## OQ-09-017 - KVF98 star judgements on word order

Rules: `SYN-Q-007`, `SYN-ORDER-009`. **Not measurable with a concordancer.** A preposed universal
before a wh-phrase and neutral order under proposing verbs are configurational; a regex cannot
identify either without a parse, and the negative judgement being tested requires counting
*absences*.

**Verdict: insufficient evidence.** Needs a parsed corpus.

Proposed edit: none. `SYN-Q-007` and `SYN-ORDER-009` **Validation**: add `needs:parsed-corpus`.

## OQ-04-001 / OQ-09-013 - `ami` vs `amely` (measured as a sanity check)

Rules: `PND-REL-004`, `SYN-CLAUSE-005`, `SYN-LINT-025`. These are `statistical-estimate`
questions, not `usage-variant`, and were measured to validate the toolkit against a known gradient.

| bucket | `ami` | `amely` | `ami` share |
| --- | ---: | ---: | ---: |
| spoken `[NP]` | 485,571 | 67,928 | 0.877 |
| news `[NP]` | 703,402 | 579,109 | 0.548 |
| wiki `[NP]` | 226,252 | 253,044 | 0.472 |
| official | 29,494 | 280,919 | 0.095 |
| technical `[NP]` | 1,228,976 | 1,976,991 | 0.383 |
| literary | 32,865 | 33,926 | 0.492 |
| web | 3,138,654 | 3,011,468 | 0.510 |
| Webcorpus 2 (no register) | 11,886,421 | 8,071,922 | 0.596 |

The gradient is monotone and matches the prescriptive description: spoken 88% `ami`, official 9.5%.
This is a token count over both antecedent types, so it calibrates the lint only as an upper
bound; antecedent typing still needs a parse.

**Verdict: register-conditioned**, gradient as tabulated. The word-list "25/25 buckets are equal"
note that the question records is an artefact of bucketing, not a finding.

---

# Summary

| OQ | category | verdict | confidence |
| --- | --- | --- | --- |
| `OQ-04-010` | usage-variant | A preferred (`a Hugo`, 99.4%); `y` names split by pronunciation | high |
| `OQ-01-003` | usage-variant | insufficient evidence (acronym reading); `duplavé` solid 7:1 | low |
| `OQ-02-005` | usage-variant | item-lexicalised, not predictable from the final letter name | medium |
| `OQ-02-007` | usage-variant | item-lexical; per-item shares given | high |
| `OQ-03-002` | usage-variant | register-conditioned + sense split | high |
| `OQ-03-008` | usage-variant | A preferred (`-ikat`/`-iket`), near-categorical | high |
| `OQ-04-003` | usage-variant | B preferred (`ezeregyszáz` 65%), both current | high |
| `OQ-04-004` | usage-variant | both current, item-split; front always `-ikek` | high |
| `OQ-04-005` | usage-variant | A preferred (`15-i` 89%), register-conditioned | high |
| `OQ-04-008` | usage-variant | A preferred (`bele` 80.5%), literary gradient for `belé` | high |
| `OQ-05-004` | usage-variant | linking vowel categorical except 2Pl, which is register-conditioned | high |
| `OQ-05-006` | usage-variant | item-lexical; only `fürdik` varies | high |
| `OQ-05-010` | usage-variant | A preferred, near-categorical; `volt volna` literary | high |
| `OQ-05-011` | usage-variant | register-conditioned: archaic, not merely less frequent | high |
| `OQ-06-003` | usage-variant | A preferred (back, 99.1%); `-né` transparent | high |
| `OQ-07-016` | usage-variant | A preferred: full-name reading for `kg`; `CNN` front | medium |
| `OQ-09-001` | usage-variant | insufficient evidence (needs a parser) | - |
| `OQ-09-006` | usage-variant | insufficient evidence; plural attested 516x | low |
| `OQ-09-012` | usage-variant | insufficient evidence for a rate; zero form attested | low |
| `OQ-02-009` | usage-variant | insufficient evidence (not corpus-arbitrable) | - |
| `OQ-02-016` | usage-variant | both current (0.44 : 0.56), split by paradigm cell | high |
| `OQ-03-007` | usage-variant | both current (54 : 46), register-conditioned | high |
| `OQ-03-011` | usage-variant | A preferred at pattern level; item insufficient | medium |
| `OQ-03-014` | usage-variant | cell-split (`borjút` / `borjak`); `ajtói ~ ajtajai` both current | high |
| `OQ-03-017` | usage-variant | A preferred (`nénje` 99.5%) | high |
| `OQ-05-005` | usage-variant | A preferred both (`edzettem`, `edzel`) | high |
| `OQ-05-007` | usage-variant | A preferred (`termettem`) with a homograph caveat | medium |
| `OQ-05-009` | usage-variant | marginal confirmed; `éve` participle unattested | medium |
| `OQ-05-014` | usage-variant | item-lexical; `javallott` vs `sugallt` opposite | high |
| `OQ-10-015` | usage-variant | B preferred (`lássék` 82%), `látsszék` standard minority | high |
| `OQ-05-015` | source-conflict | both sources partly right; heaviness is lexical | high |
| `OQ-06-011` | source-conflict | KVF98 right, SMNy3 wrong; `-An` productive on `-i` | high |
| `OQ-06-016` | source-conflict | 2 of 3 arbitrated (`-lAg` after `-beli`, passive `-(t)Atik`) | medium |
| `OQ-07-013` | source-conflict | insufficient evidence (not corpus-arbitrable) | - |
| `OQ-09-018` | source-conflict | insufficient evidence for a rate; pattern attested | low |
| `OQ-03-003` | source-conflict | B preferred: `-ként` after `-é` unattested | high |
| `OQ-03-015` | source-conflict | A preferred: `pléh` consonant-final in use | high |
| `OQ-03-016` | source-conflict | B preferred: `Lajosként` regular, SMNy3 not supported | high |
| `OQ-04-012` | source-conflict | A preferred (long `húsz-`); AkH12/UniMorph right | medium |
| `OQ-09-017` | source-conflict | insufficient evidence (needs a parser) | - |

Counts by primary verdict, 40 questions measured: **A preferred** 14, **B preferred** 4,
**both current** 3, **register-conditioned** 3, **item-lexical** 6, **marginal confirmed** 1
(`OQ-05-009`), **partly arbitrated** 1 (`OQ-06-016`), **insufficient evidence** 8 - of which 4 are
not corpus-arbitrable at all (`OQ-02-009`, `OQ-07-013`, `OQ-09-001`, `OQ-09-017`).

Several questions carry a secondary verdict: `OQ-04-005`, `OQ-04-008`, `OQ-05-004`, `OQ-05-010`,
`OQ-03-007` and `OQ-03-002` are all register-conditioned on top of their primary verdict, and the
register gradients are tabulated in their sections.

Confidence key: `high` = every quoted bucket above the floor with a tag-controlled count;
`medium` = above the floor but with a homograph or licence caveat; `low` = attestation only,
no quotable rate.
