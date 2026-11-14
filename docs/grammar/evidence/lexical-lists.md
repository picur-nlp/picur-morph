# Closed lexical lists: evidence and induction

Workstream C. Produced 2026-09-16 from the oracles and corpora listed under Licences.
This file reports what the evidence decided; it proposes rule edits but does not make them.
The lists themselves are working data under `local/lists/` (gitignored), not library data.

## Method

Error-driven induction with cross-resource voting. For every list:

1. **Diagnostic forms.** Each membership question is turned into a pair of surface forms,
   one predicted by membership, one by non-membership: antiharmonic `X-nak` vs `X-nek`;
   lowering `X-ak/-at` vs `X-ok/-ot`; epenthetic `bokrok` vs `bokorok`; shortening `nyarak`
   vs `nyárak`; v-stem `tavak` vs `tók`; CC past `mondottam` vs `mondtam`; and so on.
2. **Candidate enumeration.** Every lemma of the right part of speech that satisfies the
   *necessary shape conditions the rule itself states* is a candidate, not only lemmas
   already flagged in a lexicon. Compounds and derived stems are separated out where the
   rule assigns them by rule rather than by list (`NOM-LOW-006`, `NOM-POSS-007`).
3. **Votes.** Per candidate: morphdb flag, our own `*.ron` trait, hunmorph-foma acceptance
   of each diagnostic form, hunspell acceptance, corpus counts of each diagnostic form,
   and a boolean agree/disagree signal from UniMorph and kaikki.
4. **Decision.** The corpus decides alone when one variant holds at least 95 per cent of at
   least 25 tokens (`high`); otherwise unanimous non-corpus oracles decide (`medium`);
   otherwise the item goes to the review queue. Three-way lists (CC past, geminate verbs,
   settlement cases) use 90/10 and report `both` as a class, not as a failure.
5. **Tie-breaks.** 120 queries to the official spelling checker, cached with timestamps
   under `local/cache/suggest/`, used only where the corpus was silent or contradicted a
   source. Results in `local/lists/suggest-tiebreaks.tsv`.

### Why the corpus is the primary signal

[`crates/picur-morph-lexicon/data/frequency.ron`](../../../crates/picur-morph-lexicon/data/frequency.ron) only holds the top 250k types as
half-octave buckets, so most diagnostic forms fall below its cut-off. The counts here come
from **Webcorpus2FrequencyList** (MIT, Zenodo 10.5281/zenodo.17508385): 36,936,726
form+lemma+emMorph-tag rows over a 9e9-token corpus, indexed into
`local/cache/webcorpus2.sqlite` by [`tools/webcorpus.py`](../../../tools/webcorpus.py). Because every row carries a
lemma and a POS, a diagnostic form is counted **only under the intended lemma**, which
removes the homograph problem that makes raw form counts useless here: `északak` is counted
under `észak`, not under `ész`; `falok` under the verb `fal`, not under the noun.

### Tooling

[`tools/`](../../../tools/): `oracles.py` (loaders and the licence split), `webcorpus.py` (corpus
index), `listgen.py` (form building, voting, output), `suggest.py` (rate-limited, cached
MTA client), and one `gen_*.py` per list family. Every list is reproducible by re-running
its generator.

## Lists produced

All under `local/lists/`. Columns: `lemma, decision, evidence (votes), confidence,
diagnostics (forms and counts), n_pos, n_neg`, plus per-list extras.

| file | rows screened | members | other classes | rule(s) | OQ |
| --- | --- | --- | --- | --- | --- |
| `antiharmonic.tsv` | 2,213 shape candidates | 79 rows / 72 lemmas (46 V, 24 N, 9 A) | 23 review, 92 non-member | `PH-HARM-008`, `VRB-STEM-013` | `OQ-02-003` |
| `neutral-final-loan-harmony.tsv` | 162 | 8 back | 51 variable, 103 front | `PH-HARM-013`, `PH-HARM-019` | `OQ-02-002`, `OQ-02-015` |
| `lowering-nouns.tsv` | 4,812 | 155 lowering (139 noun-only) | 23 variable, 83 review, 2,821 non-lowering | `NOM-LOW-007`, `NOM-LOW-008` | - |
| `nonlowering-adjectives.tsv` | 1,597 | 184 non-lowering | 3 variable, 381 lowering, 392 review | `NOM-LOW-004` | `OQ-03-004` |
| `epenthetic-nouns.tsv` | 8,768 | 1,106 (230 root, 146 `-alom/-elem`, 729 compound) | 11 review | `PH-ZERO-001`, `NOM-STEM-006` | - |
| `epenthetic-verbs.tsv` | 3,357 | 327 (300 root) | 31 review | `PH-ZERO-003`, `VRB-STEM-007` | - |
| `shortening-stems.tsv` | 8,983 | 497 (141 root: 35 monosyllabic, 106 polysyllabic) | 9 review | `NOM-STEM-007/008` | `OQ-03-001` |
| `v-stems.tsv` | 139 | 26 | 36 review | `PH-VSTEM-001..005` | `OQ-02-012` |
| `sz-d-verbs.tsv` | 158 | 52 (13 with attested v-stem) | 1 review | `VRB-STEM-012` | `OQ-05-013` |
| `geminate-verbs.tsv` | 47 | 9 vowel, 15 none, 11 both | 12 review | `VRB-STEM-006` | - |
| `t-final-monosyllabic-verbs.tsv` | 16 | 14 `-Ott`, 1 `-t` (`lát`), 1 both (`hat`) | - | `VRB-STEM-003`, `VRB-PAST-006` | - |
| `cc-past-verbs.tsv` | 410 | 340 vowel, 59 none, 11 both | - | `VRB-PAST-007` | `OQ-05-003` |
| `bare-accusative.tsv` | 10,611 | 705 bare-`t` | 855 review | `NOM-ACC-007/009` | `OQ-03-005` |
| `poss3-ja-exceptions.tsv` | 2,373 monomorphemic | 726 `-ja`, 1,562 `-a` | 85 variable | `NOM-POSS-006/007` | `OQ-03-010` |
| `settlement-local-cases.tsv` | 2,819 | 1,381 surface, 1,018 interior | 420 both | `NOM-PROP-006` | `OQ-03-013` |
| `derivational-inventories.tsv` | 1,185 | per suffix, see below | - | `DER-NOUN`, `DER-ADJ` | `OQ-06-006` |
| `sag-linking-vowel.tsv` | 779 | 1 linked, 2 both | 776 bare | `DER-NOUN-002` | `OQ-06-001` |
| `shortened-i-adjectives.tsv` | 17 probed | 6 shortened, 5 both | 6 full | `DER-ADJ-003` | `OQ-06-007` |
| `epenthetic-z-derivation.tsv` | 28 | 26 drop, 2 keep | - | `DER-VERB-002` | `OQ-06-009` |
| `causative-selection.tsv` | 153 | 119 `-At`, 19 `-tAt`, 15 both | - | `VRB-CAUS-001` | `OQ-05-012` |
| `non-lengthening-suffixes.tsv` | 18 probed | 8 non-lengthening, 3 lengthening | 2 both, 5 unattested | `ORT-SUFX-002` | `OQ-07-001` |
| `determiner-like-adjectives.tsv` | 12 | 6 pluralise, 6 do not | - | `SYN-NUMAGR-003` | `OQ-09-011` |
| `initial-ty-types.tsv` | 200 | see below | - | `PH-CONS` | `OQ-02-013` |
| `digraph-junctions.tsv` | 8 sequences | inconclusive | - | `WS-DIG-006` | `OQ-01-009` |
| `fused-hogy-class.tsv` | 14 | not decided | - | `PUN-COMMA-009` | `OQ-08-003` |
| `suggest-tiebreaks.tsv` | 58 pairs | - | - | many | many |
| `review-queue.tsv` | 1,255 | - | ranked | - | - |

### Composition notes

**Antiharmonic (72 lemmas).** The induced set reproduces the classic inventory.

- nouns: `csík csín cél derék díj fi fing gyík héj híd kín nyíl nyír pír szíj sík síp sír
  zsír íj ír`, plus the loans `spejz` and `design`
- adjectives: `derék híg hív nyílt sík víg ír írt rítt`
- verbs, roots: `bír bíz bízik dívik fingik hív hízik ing irt izzik nyit nyí nyílik nyír
  ring rí siklik szid szít szív sír tilt vív ír ívik`
- verbs, `-ít` derivatives with a back or truncated base: `bénít cifrít csitít fitít hígít
  ifjít indít izzít némít pirít piszkít ritkít simít tisztít vidít`
- verbs, sound-symbolic `-ít`: `rikít sikít sipít sivít virít visít`

`ín` was settled by the spelling checker (`ínnak` legal, `ínnek` unknown) against a thin
corpus. `férfi` stays in review by design: `férfinak` 70,490 vs `férfinek` 17,325 and both
are legal, which is exactly the vacillation `PH-HARM-009` already describes.

**Loan stems with a neutral final vowel (162, MIT source).** Disjoint from the class above:
these have a back vowel earlier in the stem. Only 8 are back-dominant, 51 vacillate. This is
the first item-level figure the project has for `PH-HARM-013` / `PH-HARM-019`.

**Lowering nouns (155).** All of the `NOM-LOW-008` goldens come out: `ház fal hal had vad
fog ág agy ágy vágy tárgy gyár vár ár tál szál sál lyuk hold vas máj száj nyáj haj talp
váll toll ujj gally árny szárny sarj zár tár nyak díj szíj íj héj láz oldal`, the front set
`fül föld szög ügy völgy tölgy hölgy öv tőgy törzs szörny fürj ölyv könny`, and the
accusative-diagnosed `tej fej hely méz szenny`. The class size (139 noun-only lemmas) is
close to Tab21's 119, which is reassuring given the different lexicon base.

**Variable lowering (23).** The corpus separates the plural cell from the accusative cell
for a small group, exactly the "small variable group" Tab21 reports: `öröm` (`örömök`
19,728 but `örömet` 138,399 vs `örömöt` 77,324), `kor`, `viszony`, `tulajdon`, `vagyon`,
`panasz`, `rokon`, `kölcsön`, `szektor`, `fázis`, `bál`, `humor`, `labor`, `áll`, `ász`.
Our rulebook has no `variable` value for `lowering`; it needs one, or these items generate a
wrong plural or a wrong accusative whichever way they are flagged.

**Non-lowering adjectives (184).** The class is overwhelmingly ethnonyms and language names
(`magyar angol orosz török görög holland horvát lengyel...`, about 120 of 184), confirming
the `NOM-LOW-004` generalisation, plus the sources' `nagy` and `agg` and the attested
`fiatal vak szabad bolond józan hajadon bitang pazar`. `boldog` comes out `variable`
(matching the rule's `boldogok ~ ?boldogak`). The final-`g`/`d`/`k`/`l` phonological
predictor is **not** supported: `halk`, `finom`, `vastag` and `gazdag` behave differently
from each other, and most non-lowering adjectives are semantic (ethnonym or nominalised),
not phonological.

**Epenthetic nouns (1,106).** 230 monomorphemic roots, 146 `-alom/-elem` derivatives, 729
compounds whose head is one of the roots. T04's "about 250 nouns including complex ones"
matches the root plus `-alom/-elem` figure (376); SMNy3's 104 monomorphemic count is lower
than our 230 because we include lower-frequency roots (`bugyor cseber töbör lucsok pocok
vöcsök rücsök`) that the printed list omits.

**Epenthetic verbs (327, 300 roots).** More than T04's ~150 because morphdb does not split
the `-Vg` frequentatives (`csavarog forog morog zörög dörög`), which are genuinely
epenthetic but derived. Filtering those leaves roughly the expected size; the `kind` column
carries the information.

**Shortening stems (141 roots).** All the `NOM-STEM-008` monosyllables are confirmed. The
polysyllabic `é ~ e` column that `OQ-03-001` asks about comes out as `egér fedél födél
fenék gyökér kenyér kerék kötél levél szekér tehén tenyér veréb cserép fazék szemét nehéz
derék elég közép egyéb miegyéb nyolcadfél hitfél raktér altér beltér kültér`, and the
`á ~ a` set as `agár bogár kanál kosár madár mocsár mozsár pohár sudár sugár szamár parázs
darázs fonál`. `gúnár` is **not** a member (`gunarak` rejected by the spelling checker,
`gúnárok` 158), so the rulebook's `?gúnár` can be dropped.

**v-stems (26).** `tó hó szó jó ló lé kő cső tő fű mű nyű` plus the verbs `lő nő sző ró fő
fú`, the disyllabic `falu tetű daru hamu` and the ú-deleting `borjú varjú ifjú fattyú`.
`szú` is **not** a v-stem (`szuvak` rejected, `szúk` 510). `hó` 'month' **is** (`havak`
legal, `hók` rejected). `bő` and `odú` stay in review, both variants being legal and
attested. `pléh` is not a v-stem (`plével` rejected, `pléhvel` 71).

**CC-final past (410).** Decisive. The linking vowel is absent in 59 verbs, and the class is
almost entirely `-ng`: 48 of the 59 (`zeng mereng rajong dong döng kereng kering borong
bolyong lázong lézeng szorong tolong ujjong zsong csüng csöng dühöng dereng terjeng
töpreng őrjöng ...`), with the `-Cd` members `küld kezd hord kérd örvend`. 11 verbs take
both (`cseng ing leng peng reng teng ráng fingik érzik esd`). Every other CC stem (340)
takes the linking vowel. This settles the rule's "default for unlisted `-Cd`, `-ng` stems"
question in the opposite direction for `-ng`: the default there should be **no linking
vowel**, not `-Ott-`.

**Geminate verbs (47).** Reproduces `VRB-STEM-006` item by item: `áll száll` no vowel and
past `-t`; `hall vall kell illik ellik züllik izzik` vowel and `-ott`; `hull mállik` both;
`varr forr` no vowel with a variable past; `függ csügg` variable linking with `-ött` in the
3sg. `mállt` is rejected by the spelling checker, so `mállott` is the only form.

**t-final monosyllables.** T1 and T2 merge in the past: 14 of 16 take `-Ott` in all cells
(`fut hat jut köt nyit süt üt vet tát vét fűt hűt műt szít dűt`), `lát` is the only `-t-`
verb, and `hat` is variable in the corpus but `hattam` is rejected by the checker, so it
belongs with T1 as the rule says.

**Bare accusative.** The lowering-and-bare-`t` class that `NOM-ACC-007` asks about, measured
against a corpus-verified lowering diagnosis rather than a flag, is small: `oldal` and nine
compounds of it (`címoldal domboldal hegyoldal hátoldal partoldal töltésoldal árnyoldal
éroldal ügyféloldal`), `leltár` and `vagyonleltár`, `sudár`, `sivár`, `avas`, `hamvas`,
`kormos`, `kásás`, `köpcös`, `gőgös`, `meglehetős`, `parkettás`, `szorgalmatos`,
`túlkoros`, `őshonos`. `oldal` is confirmed hard: `oldalat` is rejected by the checker.
**`báj` is not a member**: `bájat` 127 vs `bájt` 10, both legal - the rulebook's claim rests
on a thin `ATT` figure. `szakáll` is genuinely variable (`szakállt` 2,773 vs `szakállat`
2,988, both legal).

**3sg possessive.** The `Tab21 Table 17` logistic model reproduces the corpus choice for
**68.1 per cent** of 2,373 monomorphemic consonant-final nouns (731 exceptions), with 85
further stems genuinely variable. That is a usable prior but not a generator: the rulebook's
"every frequent noun must carry a lexical `poss3`" is confirmed, and the exception list is
now enumerated.

**Settlement names (2,819).** Surface case dominates for `-vár` (39 of 42), `-egy` (19/20),
`-kút` (15/15), `-föld` (13/15), `-erdő` (12/14), `-hely`, `-ely` (21/26) and `-ács`
(13/16). Interior case dominates for `-ány` (35/46), `-ény` (28/31), `-ony` (23/24),
`-ország` (21/22), and for the foreign endings `-ton`, `-burg`, `-tán`. 420 names take both.

**Derivational inventories.** Attested lemma counts with at least 100 corpus tokens are far
above Rebrus' figures: `-mÁny` 402 (Rebrus 89), `-vÁny` 223 (78), `-(O)dA` 260 (30), `-nOk`
159 (36), `-(O)nc` 90 (15), `-AtAg` 58 (24), `-lAg` 86 (46). Rebrus is evidently counting
non-compound, non-prefixed dictionary entries; our figures include compounds
(`keresztrejtvény`, `főmérnök`). The root/compound split is not computed for this list, so
the comparison is not yet apples to apples.

**`-sÁg` linking vowel.** Over 779 `-s`-final adjective bases, only `friss` takes the linked
form categorically (`frissesség`); `nyers` and `gyors` vary; every other base is bare. The
"15 lexical items" of `OQ-06-001` are not reproducible from the corpus.

**Shortened `-i` adjectives.** `heti havi tavi művi` are confirmed, and `kövi` and `levi`
join them (`kövi` 7,326 vs `kői` 632). `lovi` and `teli` are homograph-contaminated and
should not be counted. `kézi vízi úti tűzi nyári égi` are confirmed unshortened.

**Non-lengthening suffixes.** `-ként -féle -fajta -szerű -beli -kor -szor -izmus` measure as
non-lengthening on `a`/`e`-final bases; `-s`, `-nyi`, `-stUl` lengthen; `-sÁg` and `-z` show
both. The marginal learned suffixes `-izál -ikus -lógia` produce no attested forms on such
bases, so `OQ-07-001` remains open for exactly the items it flags.

**Determiner-like adjectives.** `bizonyos további számos különböző némely egynémely`
pluralise; `mindenféle különféle többféle sokféle egyéb némi` do not (0 to 40 tokens).

**Initial `ty`.** Above 100 tokens the only native Hungarian types are `tyúk` and its
compounds, the interjection `tyű` (1,683), and the place name `Tyukod` / `tyukodi`. `tyüh`
is below the threshold. KVF98's claim holds.

## Oracle agreement

Agreement between each oracle and the corpus decision, on rows the corpus decided
(`local/lists/agreement.json` has the raw pairs).

| list | morphdb | our `.ron` | foma | hunspell | UniMorph | kaikki |
| --- | --- | --- | --- | --- | --- | --- |
| antiharmonic | 67/159 (42%) | 153/159 (96%) | 144/144 (100%) | 144/144 (100%) | 113/113 (100%) | 131/131 (100%) |
| lowering-nouns | 2610/2667 (98%) | 2610/2667 (98%) | 2330/2332 (100%) | 2273/2278 (100%) | 1377/1404 (98%) | 1588/1612 (99%) |
| nonlowering-adjectives | 484/537 (90%) | 422/537 (79%) | 46/52 (88%) | 65/66 (98%) | 240/256 (94%) | 277/287 (97%) |
| epenthetic-nouns | 4918/4953 (99%) | 4918/4953 (99%) | 4224/4230 (100%) | 4241/4256 (100%) | 1423/1423 (100%) | 1851/1852 (100%) |
| epenthetic-verbs | 1713/1767 (97%) | 1713/1767 (97%) | 1595/1598 (100%) | 1564/1567 (100%) | - | - |
| shortening-stems | 5057/5072 (100%) | 5056/5072 (100%) | 4902/4903 (100%) | 4698/4700 (100%) | 1638/1639 (100%) | 2074/2075 (100%) |
| v-stems | 90/97 (93%) | 90/97 (93%) | 66/68 (97%) | 57/60 (95%) | 67/69 (97%) | 73/73 (100%) |
| sz-d-verbs | 121/138 (88%) | 123/138 (89%) | 133/133 (100%) | 134/134 (100%) | - | - |
| bare-accusative | 619/681 (91%) | 2481/3181 (78%) | 1753/1761 (100%) | 1940/1946 (100%) | 925/948 (98%) | 1220/1240 (98%) |
| poss3 | 599/620 (97%) | 646/755 (86%) | - | - | 1364/1380 (99%) | 1514/1527 (99%) |

Three findings matter for the implementation:

- **morphdb's `back` flag is not an antiharmony flag** (42 per cent agreement). morphdb
  carries `back` and `front` on the same lemma often enough (`szép`, `rövid`, `éves`) that
  it cannot be read as a harmony class at all. Our own `Back` trait is far better (96 per
  cent), presumably because it was curated. Any code that derives harmony from morphdb must
  ignore `back` except on all-neutral stems; the induction here does exactly that.
- **foma and hunspell are near-perfect legality filters and near-useless rankers.** They
  agree with the corpus on essentially every row where they take a side, but they abstain
  (both variants legal) on 30 to 60 per cent of rows, which is why they cannot close the
  variable classes.
- **Our `.ron` disagrees with the corpus on bare accusative in 22 per cent of rows.** The
  `BareAccusative` / `LinkedAccusative` traits (19 and 6 on nouns, 262 and 1,027 on
  adjectives) are sparse and inconsistent; `bare-accusative.tsv` should replace them.

CC BY-SA sources (UniMorph, kaikki) and the study-only Rácz datasets were used only as the
boolean agree/disagree columns above and in the per-row `evidence` field. No string from
them is in any list.

## Review queue

`local/lists/review-queue.tsv`, 1,255 items, ranked by `rules x (1 + sqrt(lemma_freq)/100)`,
that is by how many rules depend on the list times how frequent the lemma is. Rows with no
attested diagnostic form at all are excluded: nothing can decide them and generation falls
back to the rule default. Top 50, grouped where the same evidence decides a block:

| # | list | lemma | state | what decides it |
| --- | --- | --- | --- | --- |
| 1 | lowering-nouns | `lát` | review | homograph with the verb; needs a sense-tagged count |
| 2 | lowering-nouns | `áll` | variable | `állak` 309 / `állot` 115: plural lowers, accusative does not |
| 3 | nonlowering-adjectives | `más` | variable | `másak` 2,969 vs `mások` 80,965: noun and adjective senses split |
| 4 | antiharmonic | `férfi` | review | `férfinak` 70,490 / `férfinek` 17,325, both legal (`PH-HARM-009` already says so) |
| 5 | lowering-nouns | `kor` | variable | `korok` 47,397 but `korat` 310 vs `korot` 16 |
| 6 | lowering-nouns | `igaz` | variable | adjective and noun senses split |
| 7 | bare-accusative | `teljes` | review | `teljest` 372 / `teljeset` 2,372: register split |
| 8 | epenthetic-nouns | `lélek` | review | our diagnostic ignores the accompanying shortening (`lelkek`, not `*lélkek`): generator gap, not missing data |
| 9 | lowering-nouns | `öröm` | variable | `örömök` 19,728, `örömet` 138,399: needs a per-cell value |
| 10 | v-stems | `bő` | review | `bővek` 103 / `bők` 76, both legal (`PH-VSTEM-001` speaker variation) |
| 11-18 | bare-accusative | `magas természetes szükséges pár személyes közös képes erős` | review | `-s`-final adjectives: genuine `NOM-ACC-009` variation, needs a register split |
| 19-26 | lowering-nouns | `panasz tulajdon vagyon kölcsön rokon szektor fázis viszony` | variable | plural mid, accusative low: all need a per-cell `lowering` value |
| 27 | poss3 | `szabad` | review | `szabadja` 72 / `szabada` 35: sense split |
| 28-29 | bare-accusative | `számos érdemes` | review | same `-s`-final adjective class as rows 11-18 |
| 30 | t-final | `hat` | both | corpus variable, `hattam` rejected by the checker, so treat as T1 |
| 31-50 | bare-accusative | `általános összes tökéletes bizonyos hall ismert levél sikeres hivatalos rendszeres alkalmas hatékony lehetséges elektronikus hagyományos` | review | the `-s`/`-t` adjective class again: one register-stratified count would close all of them |

The queue collapses into five actual questions:

1. Per-cell `lowering` for about 25 nouns whose plural and accusative differ.
2. The `-s`-final adjective accusative (`teljest ~ teljeset`), about 400 items, one
   register-stratified count away from closure. This is `NOM-ACC-009`, not `NOM-ACC-007`.
3. Sense splits on noun/adjective homographs (`más`, `igaz`, `piros`, `vörös`), which need a
   sense-tagged corpus we do not have.
4. Speaker-variable v-stems (`bő`, `odú`, `fő`, `nyű`), already flagged in `PH-VSTEM`.
5. About 36 `-fű` / `-mű` compounds whose head class is known and which need no separate
   decision.

## Licence notes

| resource | licence | what it contributed |
| --- | --- | --- |
| morphdb.hu | CC BY 2.5 | candidate lemmas, `STR` morpheme splits, class flags; strings copied into our lists, attribution required |
| hunmorph-foma | BSD-2-Clause | acceptance of each diagnostic form (legality filter) |
| hunspell hu_HU | MPL | acceptance of each diagnostic form (legality filter) |
| our `*.ron` | ours | prior flags, candidate lemmas |
| Webcorpus2FrequencyList | MIT | all corpus counts; derived figures publishable, cite Zenodo 10.5281/zenodo.17508385 |
| Racz2027vh `dzsungel.tsv` | MIT | the 162 neutral-final loan stems with back/front counts, copied |
| UniMorph hun | CC BY-SA | boolean agree/disagree only, no strings copied |
| kaikki-hu | CC BY-SA | boolean agree/disagree only, no strings copied |
| Racz2024, RaczRebrus2024, RaczRebrusTorkenczy2018 | all rights reserved | read for cross-checking only; `noun_bag.tsv` turned out to encode harmony from the vowels rather than from behaviour, so it is not an antiharmony oracle and contributed nothing |
| helyesiras.mta.hu | service | 120 legality judgements, cached with timestamps under `local/cache/suggest/` |

None of this file's figures come from the project rate index; Webcorpus2 supplies every count.

## Proposed rule edits

Each row is a proposal, not an applied change. `list` names the file under `local/lists/`.

| OQ | rule | field | old -> new | list, size |
| --- | --- | --- | --- | --- |
| `OQ-02-003` | `PH-HARM-008` | Predictability / Lexical input | "about 60 roots ... full list from Vago (1980) and the lexicon oracle" -> closed list of 72 lemmas (46 verbs, 24 nouns, 9 adjectives), induced and corpus-verified | `antiharmonic.tsv`, 72 |
| `OQ-02-003` | `PH-HARM-008` | Exceptions | add `design` beside `spejz` as a neutral-vowel loan taking back suffixes | `antiharmonic.tsv` |
| `OQ-02-003` | `VRB-STEM-013` | Conditions | "~35 roots" -> the 46 verb lemmas of the list; adds `csitít izzít fingik ívik dívik siklik`, removes none | `antiharmonic.tsv` |
| `OQ-02-002`, `OQ-02-015` | `PH-HARM-013` | Lexical input | add the 162-stem neutral-final loan table with per-item `p_back`; 8 back-dominant, 51 vacillating | `neutral-final-loan-harmony.tsv`, 162 |
| `OQ-03-001` | `NOM-STEM-008` | Output, polysyllabic `é ~ e` row | reconstruction -> the 106 corpus-verified polysyllabic roots; drop `?gúnár` | `shortening-stems.tsv`, 141 roots |
| `OQ-03-001` | `NOM-STEM-008` | Confidence | "`tendency` for the reconstructed polysyllabic list" -> `consensus` | - |
| `OQ-03-004` | `NOM-LOW-004` | Exceptions | "currently 10 listed items" -> 184 items; drop the final-`g`/`d`/`k`/`l` phonological predictor, the class is ethnonyms plus nominalised adjectives | `nonlowering-adjectives.tsv`, 184 |
| `OQ-03-004` | `NOM-LOW-004` | Predictability | `tendency` (phonological) -> `lexical` with a semantic (ethnonym / nominalised) generalisation | - |
| - | `NOM-LOW-007` | Lexical input | `lowering` boolean -> three-valued `true / false / variable`, the variable value naming the cell | `lowering-nouns.tsv`, 155 + 23 variable |
| - | `NOM-LOW-008` | Output | keep as goldens: every listed item is confirmed | `lowering-nouns.tsv` |
| `OQ-03-005` | `NOM-ACC-007` | Output | `báj`, `szakáll`, `oldal` -> `oldal` plus 9 compounds, `leltár`, `sudár`, `sivár` and 12 further items; `báj` moves out (`bájat` 127 vs `bájt` 10), `szakáll` becomes an explicit doublet | `bare-accusative.tsv`, 15 lowering members |
| `OQ-03-005` | `NOM-ACC-009` | Lexical input | add the 705-item bare-`t` list and the 855-item variation list, replacing the `BareAccusative` trait | `bare-accusative.tsv` |
| `OQ-03-010` | `NOM-POSS-006` | Predictability | add the measured accuracy: the Tab21 model reproduces 68.1% of 2,373 monomorphemic C-final stems; 731 lexical exceptions enumerated | `poss3-ja-exceptions.tsv` |
| `OQ-03-013` | `NOM-PROP-006` | Output | example lists -> ending-based tendencies over 2,819 names (surface `-vár -egy -kút -föld -erdő -hely -ely -ács`; interior `-ány -ény -ony -ország` and foreign `-ton -burg -tán`), with a 2,819-row lexicon | `settlement-local-cases.tsv` |
| `OQ-05-003` | `VRB-PAST-007` | Conditions, "default for unlisted" | "`-Ott-` (safer), flagged for review" -> for `-ng` stems the default is **no** linking vowel (48 of the 59 no-vowel verbs are `-ng`); `-Cd` keeps `-Ott-` | `cc-past-verbs.tsv`, 410 |
| `OQ-05-003` | `VRB-PAST-007` | Conditions, "no linking vowel" | add `kérd`, `örvend` and the 48 `-ng` verbs; move `cseng ing leng peng reng teng ráng fingik` to "optional" | `cc-past-verbs.tsv` |
| `OQ-05-003` | `VRB-PAST-007` | Confidence | `contested` (individual items) -> `consensus`, with the corpus counts cited | - |
| `OQ-05-012` | `VRB-CAUS-001` | Conditions | extend from the listed items to the 153 measured monosyllables: 119 `-At`, 19 `-tAt`, 15 both | `causative-selection.tsv` |
| `OQ-05-013` | `VRB-STEM-012` | Lexical input | record that `alszik fekszik nyugszik igyekszik gyanakszik gyarapszik telepszik` are absent from morphdb's `peeta.lexicon` **and** from our `verb.ron`; the 52-item list is the reconciliation | `sz-d-verbs.tsv`, 52 |
| `OQ-05-013` | `VRB-STEM-012` | Conditions, v-stem row | restrict to the 13 lemmas with an attested `v` participle | `sz-d-verbs.tsv` |
| `OQ-02-012` | `PH-VSTEM-002` | Output, `ú ~ u` row | `szú` marginal -> `szú` is **not** a v-stem (`szuvak` rejected, `szúk` 510); `hó` 'month' **is** (`havak` legal, `hók` rejected) | `v-stems.tsv`, 26 |
| `OQ-03-015` | `PH-CONS-005` | Statement | `pléh` variable v-stem -> not a v-stem (`plével` rejected, `pléhvel` 71) | `suggest-tiebreaks.tsv` |
| `OQ-03-017` | `NOM-POSS-009` | Output | `nénje ~ nénéje` -> `nénje` only (`nénéje` rejected, 12 tokens) | `suggest-tiebreaks.tsv` |
| `OQ-05-005` | `VRB-PAST-007` | Output | `edzettem ~ edztem` -> `edzettem` only (`edztem` rejected, 24 tokens) | `suggest-tiebreaks.tsv` |
| `OQ-05-014` | `VRB-STEM-006` | Validation | `mállott ~ mállt` -> `mállott` only; `javallott` 12,955 > `javallt` 3,421; `sugallt` 7,140 > `sugallott` 93 | `geminate-verbs.tsv`, `suggest-tiebreaks.tsv` |
| `OQ-05-006` | `VRB-STEM-008` | Output | per-item shares: `ugrani` 32,345 vs `ugorni` 1,040; `fürdeni` 25,400 vs `fürödni` 13,578; `hangzani` 3,000 vs `hangozni` 564 | `suggest-tiebreaks.tsv` |
| `OQ-05-011` | `VRB-STEM-012` | Output, plural present | `alszanak` 33,580 vs `alusznak` 2,311; `fekszenek` 20,974 vs `feküsznek` 1,781; `haragszanak` 3,340 with `haragusznak` **rejected** -> the `-usz-` plurals are archaic, not merely less frequent | `suggest-tiebreaks.tsv` |
| `OQ-06-001` | `DER-NOUN-002` | Output | "15 lexical linking-vowel `-sÁg` forms" -> only `frissesség` is categorical; `nyers` and `gyors` vary; no further single-`s` base takes it | `sag-linking-vowel.tsv`, 779 bases |
| `OQ-06-006` | `DER-NOUN`, `DER-ADJ` | Lexical input | replace Rebrus' counts with the attested inventories, noting that ours include compounds: `-mÁny` 402, `-vÁny` 223, `-(O)dA` 260, `-nOk` 159, `-(O)nc` 90, `-AtAg` 58, `-lAg` 86 | `derivational-inventories.tsv`, 1,185 |
| `OQ-06-007` | `DER-ADJ-003` | Output | 4 items -> `heti havi tavi művi kövi levi` (6); `kézi vízi úti tűzi nyári égi` confirmed unshortened | `shortened-i-adjectives.tsv` |
| `OQ-06-009` | `DER-VERB-002` | Statement | add the measurement: 26 of 28 epenthetic nouns with an attested `-(O)z` verb drop the vowel (`tükröz dolgoz halmoz értelmez`); only `kebel` and `sátor` keep it | `epenthetic-z-derivation.tsv` |
| `OQ-07-001` | `ORT-SUFX-002` | Output | confirm `-ként -féle -fajta -szerű -beli -kor -szor -izmus` as non-lengthening; `-s -nyi -stUl` lengthen; `-izál -ikus -lógia` remain unmeasurable on `a`/`e`-final bases | `non-lengthening-suffixes.tsv` |
| `OQ-09-011` | `SYN-NUMAGR-003` | Conditions | plural-taking `bizonyos további számos különböző némely egynémely`; never plural `mindenféle különféle többféle sokféle egyéb némi` | `determiner-like-adjectives.tsv` |
| `OQ-02-013` | `PH-CONS` | Exceptions | initial `ty` above 100 tokens is `tyúk` and its compounds, `tyű`, and the place name `Tyukod`; KVF98 confirmed | `initial-ty-types.tsv` |
| `OQ-03-014` | `NOM-STEM-012/013` | Output | `borjút` 4,195 vs `borjat` 636; `ajtói` 2,803 vs `ajtajai` 2,388, all four legal -> both doublets are standard, `borjút` preferred | `suggest-tiebreaks.tsv` |
| `OQ-03-007` | `NOM-ADJ-005` | Output | `hűbb` 645 vs `hívebb` 547, both legal -> keep both, `hűbb` slightly preferred | `suggest-tiebreaks.tsv` |
| `OQ-03-002` | `NOM-STEM-011/016` | Output | `magok` 42,611 vs `magvak` 32,456, both legal -> a real doublet, not a sense-conditioned choice at this granularity | `suggest-tiebreaks.tsv` |
| - | `PH-ZERO-001` | Exceptions | counts: 230 monomorphemic roots plus 146 `-alom/-elem` derivatives plus 729 compounds | `epenthetic-nouns.tsv` |
| - | `PH-ZERO-003` | Predictability | "about 150 stems" -> 300 roots once the `-Vg` frequentatives are counted; 150 is the non-derived subset | `epenthetic-verbs.tsv` |
| - | `VRB-STEM-003` | Output | T1 and T2 merge in the past: 14 of 16 monosyllables take `-Ott` throughout, only `lát` does not | `t-final-monosyllabic-verbs.tsv` |

## What could not be settled

- **`OQ-01-009`** (native `s+z`, `d+zs`, `z+zs`, `l+ly`, `t+ty` junctions). The foma pass
  over frequent noun types cannot isolate a morpheme boundary that falls *inside* the
  digraph string, only that the word is multi-morphemic. `digraph-junctions.tsv` records the
  attempt; the question needs a boundary-aware analyser run, not a type count.
- **`OQ-08-003`** (`anélkül hogy` class). The frequency list is type-based with no bigrams,
  so `X hogy` cannot be counted. `fused-hogy-class.tsv` holds only head-word frequencies.
  Needs the KWIC tool the sibling workstream is building.
- **`OQ-07-012`, `OQ-07-009`, `OQ-02-006`, `OQ-02-014`, `OQ-01-014`, `OQ-07-014`,
  `OQ-10-013`**: all need a name pronunciation register or a transliteration source that is
  not openly licensed. No progress; nothing in the assets substitutes for them.
- **`OQ-09-007`** (valency lexicon), **`OQ-09-008`** (definiteness-transmission class) and
  **`OQ-04-006`** (postposition dative extraction) are syntactic and need a parsed corpus
  rather than a type list. Untouched here.
- **`OQ-03-009`** (surnames homonymous with alternating nouns): the frequency list is
  lowercased, so `Madár` cannot be separated from `madár`. Needs a cased corpus.
- **`OQ-10-009`, `OQ-10-010`, `OQ-10-016`, `OQ-06-017`, `OQ-07-003`, `OQ-07-022`,
  `OQ-06-004`, `OQ-06-014`** are not lexical-induction questions; they need the layout
  sources, the printed SMNy3 table, or the AkH12 dictionary part.

Score against the 37 `lexical-list` entries: **16 answered** with a list and a proposed edit
(`OQ-02-003 OQ-02-012 OQ-02-013 OQ-03-001 OQ-03-004 OQ-03-005 OQ-03-013 OQ-05-003 OQ-05-012
OQ-05-013 OQ-06-001 OQ-06-006 OQ-06-007 OQ-06-009 OQ-07-001 OQ-09-011`), **2 attempted and
still open** (`OQ-01-009 OQ-08-003`), **19 not addressable** from the available assets
(licence-blocked pronunciation and transliteration sources, syntactic questions needing a
parsed corpus, and questions needing the printed SMNy3 table or the AkH12 dictionary part).

Eleven further open questions outside the `lexical-list` category were settled as a
by-product, because the same diagnostics answer them: `OQ-02-002 OQ-02-015` (loan-stem
vacillation figures), `OQ-03-002 OQ-03-007 OQ-03-014` (doublet preferences), `OQ-03-010`
(the `-a`/`-ja` model's measured accuracy), `OQ-03-015 OQ-03-017` (`pléh`, `néne`),
`OQ-05-005 OQ-05-006 OQ-05-011 OQ-05-014` (past and stem variants).
