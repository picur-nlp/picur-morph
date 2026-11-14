# Productivity and tagset evidence

Date: 2026-09-16. Workstream G+E. Settles the `theory-productivity` and `tagset-mapping`
entries of [`docs/grammar/open-questions.md`](../open-questions.md), plus three productivity questions named in the
brief that sit in other categories (`OQ-03-010`, and the lowering and `-AtlAn` defaults).

This file is evidence and a proposal. No chapter, `glossary.md` or `open-questions.md` was
edited. Every rule edit below is a proposal, given as `rule ID | field | old -> new`.

## Method

### Part 1: the Tolerance Principle

Yang's Tolerance Principle: a rule stated over a candidate set of `N` items is learnable as a
rule (i.e. it generalises to new items) only if the number of exceptions `e` satisfies

    e <= theta_N = floor(N / ln N)

Above that threshold the learner stores the items individually and the process is not a
productive default. The arithmetic is trivial; the work is defining `N` and `e` so that they
are not question-begging:

- `N` is the set of items that *meet the structural description* of the rule, counted as
  types, not tokens. It must be defined independently of the outcome.
- `e` is the set of items in `N` that *take a different outcome*. Non-attestation is only
  admitted as evidence of exceptionality where the candidate is frequent enough that its
  absence from an 8.57-billion-token corpus is informative; where that is the case it is
  stated explicitly, because absence is weaker evidence than a contrary attestation.
- Where `|e - theta_N| / theta_N < 10%` the result is reported as **inconclusive**.

### Data

| source | what | size |
| --- | --- | --- |
| `local/research-racz/Webcorpus2FrequencyList/frequencies.parquet` | `form`, `lemma`, `xpostag` (emMorph), `freq`, `hunspell` | 36,936,726 rows; corpus 8,570,080,167 tokens |
| [`crates/picur-morph-lexicon/data/noun.ron`](../../../crates/picur-morph-lexicon/data/noun.ron) | codified noun lemmas + traits | 55,459 |
| [`crates/picur-morph-lexicon/data/verb.ron`](../../../crates/picur-morph-lexicon/data/verb.ron) | codified verb lemmas + traits | 19,703 |
| [`crates/picur-morph-lexicon/data/adjective.ron`](../../../crates/picur-morph-lexicon/data/adjective.ron) | codified adjective lemmas + traits | 16,395 |
| `local/lists/*.tsv` | prior antiharmonic / lowering / non-lowering decisions | 188 / 3,365 / 2,964 |

The emMorph `xpostag` column is what makes this tractable: it disambiguates homographs and
labels the exact morphemes at issue. The tags used below are `[_Mod/V]` (potential `-hAt`),
`[EssFor:ként]` / `[EssFor:képp]` / `[EssFor:képpen]`, `[_Com:stUl/Adv]`, `[_Distr:nként/Adv]`,
`[Fam.Pl]` (associative `-ék`), `[Prs.1Sg>2]` (the `-lak/-lek` cell; the tag uses U+203A),
`[/N][Poss.3Sg][Nom]` and `[/N][Acc]`.

"New / not codified" is operationalised two ways, both stated per test: (a) the lemma is
absent from our own 55,459-entry `noun.ron`; (b) the corpus `hunspell` flag is false. Where a
loan test is run, (a) is combined with a foreign-orthography filter
(`w x q ch ph th ck sh oo ee ea`) and a filter removing compounds whose head is a codified
lowering noun, so that the head's lexical lowering is not counted as the loan's behaviour.

### Method validation

Run on one uncontroversially productive and one uncontroversially listed process before any
open case was touched.

**Productive control: potential `-hAt` on verbs.** Every source treats this as fully
productive. `N` = verb lemmas in `verb.ron` attested in Webcorpus2 above a frequency floor;
`e` = those with zero `[_Mod/V]` form in the corpus.

| floor | N | e | theta_N | verdict |
| ---: | ---: | ---: | ---: | --- |
| 100 | 12,103 | 283 | 1,287 | productive |
| 1,000 | 6,708 | 51 | 761 | productive |
| 10,000 | 2,217 | 4 | 287 | productive |

`e` is far under threshold at every floor and falls as the floor rises, which is what a real
productive process looks like under an attestation proxy. The residue at the 10,000 floor is
`alapszik`, `fénylik`, `rémlik`, `vetkez` - semantically non-agentive or defective, i.e. the
known lexical exceptions. **The method behaves correctly on a productive process.**

**Listed control: lowering of codified consonant-final nouns.** Hungarian lowering is the
textbook case of lexically arbitrary class membership. `N` = consonant-final noun lemmas in
`noun.ron` (38,885); `e` = those with the other value of the `Low` trait.

| rule | N | e | theta_N | verdict |
| --- | ---: | ---: | ---: | --- |
| "every C-final noun lowers" | 38,885 | 34,296 | 3,679 | listed (e = 9.3x theta) |
| "no C-final noun lowers" | 38,885 | 4,589 | 3,679 | listed (e = 1.25x theta) |

Both directions fail, which is the correct diagnosis: over the whole codified vocabulary
lowering is neither the rule nor tolerably exceptional, so membership is stored per item.
**The method behaves correctly on a listed process.** Note the asymmetry - the non-lowering
direction misses by only 25% - which is the lead into the loans test in Q-X1 below.

## Part 1: productivity

### OQ-02-008 - `-beli`, `-fajta`, `-mentes`, `-szerte`: suffixes or compound members

Rule `PH-LEN-002`. The question is decided by whether a stem-final short `a`/`e` lengthens
before them. If they are ordinary synthetic suffixes it must lengthen; if they are compound
members or super-analytic it must not.

Candidate set `N` = corpus form types in which one of the four elements is immediately
preceded by a stem-final `a` or `e` (the only environment where the two hypotheses differ).
`e` = types showing the lengthened `a`/`e`. Forms whose stem ends in an underlying long vowel
(`kávé-`, `tévé-`, `klisé-`, `Bábel-`) are not in `N`; they were the entire apparent
"lengthened" residue of a naive count and are excluded.

Minimal pairs, corpus token counts:

| short (no lengthening) | tokens | lengthened | tokens |
| --- | ---: | --- | ---: |
| `korabeli` | 211,974 | `korábeli` | 3 |
| `mesebeli` | 34,063 | `mesébeli` | 5 |
| `szakmabeli` | 11,811 | `szakmábeli` | 0 |
| `kutyafajta` | 5,869 | `kutyáfajta` | 0 |
| `rozsdamentes` | 74,442 | `rozsdámentes` | 0 |
| `hibamentes` | 9,060 | `hibámentes` | 0 |
| `problémamentes` | 27,997 | - | - |
| `megyeszerte` | 2,640 | `megyészerte` | 0 |
| `utcaszerte` | 49 | `utcászerte` | 0 |
| `ligaszerte` | 79 | `ligászerte` | 0 |

Aggregate type counts in the `a`/`e`-final environment: `-beli` 1,957 types, `-fajta` 3,975,
`-mentes` 4,216, `-szerte` 154; total `N` = 10,302. `e` = 2 types with any lengthened
attestation (`korábeli` 3 tokens, `mesébeli` 5 tokens, both at ratios of 1:70,000 and
1:6,800 against, i.e. typos). `theta_10302 = floor(10302 / 9.240) = 1114`.

**Verdict: productive - non-lengthening, categorically.** `e = 2` vs `theta = 1114`.
The four elements pattern with `PH-LEN-002`, not with ordinary suffixes. Whether they are
called suffixes or compound members is then a labelling choice with no phonological
consequence; the phonology is settled.

Proposed edits:

- `PH-LEN-002` | Statement | add `-beli`, `-fajta`, `-mentes`, `-szerte` to the enumerated
  set: `... adjectival -i, -izmus, -ista, -beli, -fajta, -mentes, -szerte, marginally Latinate ...`
- `PH-LEN-002` | Exceptions | remove the clause `compound-like elements (-fajta, -szerte,
  -mentes, -beli) behave as compound members (no lengthening) (DER)` -> they are now in the
  Statement list, so the exception is redundant.
- `PH-LEN-002` | Predictability | `lexical (closed list of about 12 suffixes; -ként is the
  only inflectional one [SMNy3 p. 788])` -> `lexical (closed list of about 16 suffixes; -ként
  is the only inflectional one [SMNy3 p. 788])`
- `PH-LEN-002` | Validation | append: `corpus: no lengthening in 10,302 a/e-final types
  before -beli/-fajta/-mentes/-szerte (2 typo-level counterexamples; korabeli 211,974 vs
  korábeli 3)`
- `PH-LEN-002` | Confidence | `consensus (the norm gives examples only)` -> `consensus
  (corpus-confirmed for the four compound-like elements)`
- `DER-ADJ-012` (`-beli`) | Conditions | add `non-lengthening (PH-LEN-002): korabeli,
  mesebeli, szakmabeli` if not already implied.

### OQ-04-007 - `-i` on `honnan?` postpositions and on `helyetti`

Rule `PND-POSTP-010`. The chapter marks `?helyetti [ATT 1]`, `?előli [ATT 1]`,
`?közüli [ATT 0]`, `?mögüli [ATT -]` on the strength of the 250k-type project list. The
8.57-billion-token corpus contradicts that entirely.

Candidate set `N` = the 31 non-lative (`hol?` and `honnan?`) postpositions that
`PND-POSTP-010` already lists as accepting `-i`. `e` = those with no usable attestation
(threshold 50 tokens, which is 6 orders of magnitude below the frequent members and well
below anything that could be called current usage).

`honnan?` subgroup and `helyetti`, corpus token counts:

| form | tokens | chapter's mark |
| --- | ---: | --- |
| `felőli` | 54,799 | accepted |
| `alóli` | 38,865 | accepted |
| `helyetti` | 3,281 | `?helyetti [ATT 1]` |
| `előli` | 2,597 | `?előli [ATT 1]` |
| `közüli` | 1,240 | `?közüli [ATT 0]` |
| `mögüli` | 393 | `?mögüli [ATT -]` |
| `mellőli` | 236 | not listed |

`hol?` subgroup, for scale: `szerinti` 2,047,068, `közötti` 1,849,441, `elleni` 1,176,812,
`nélküli` 1,089,479, `alatti` 868,936, `utáni` 851,022, `előtti` 818,818, `iránti` 651,105,
`feletti` 519,819, `kívüli` 470,226, `belüli` 447,748, `körüli` 448,410, `melletti` 309,002,
`közti` 280,571, `szembeni` 264,248, `túli` 227,228, `általi` 212,779, `miatti` 185,708,
`menti` 146,089, `fölötti` 94,105, `felüli` 82,799, `közbeni` 51,068, `aluli` 49,426,
`mögötti` 91,148.

`N = 31`, `e = 0`, `theta_31 = floor(31 / 3.434) = 9`.

**Verdict: productive.** Every non-lative postposition in the set licenses `-i`. The
`honnan?` forms are 2 to 4 orders of magnitude rarer than the `hol?` forms, which is a
frequency fact about the postpositions themselves, not an acceptability fact about `-i`.

The reject list is also confirmed: `alái`, `elei`, `mögéi`, `mellei`, `közéi`, `feléi`,
`ótai`, `múlvai`, `áti`, `fogvai`, `kezdvei`, `képesti` = 0 tokens each; `gyanánti` 12,
`együtti` 135, `végetti` 115 - all three at noise level against six-figure base frequencies.
The lative (`hová?`) restriction is exceptionless.

Proposed edits:

- `PND-POSTP-010` | Output, accept list | `rare ?helyetti [ATT 1], ?előli [ATT 1], ?közüli
  [ATT 0], ?mögüli [ATT -]` -> `helyetti [W2 3281], előli [W2 2597], közüli [W2 1240],
  mögüli [W2 393], mellőli [W2 236]` (drop all four `?` marks; add `mellőli`, which the
  chapter omits)
- `PND-POSTP-010` | Exceptions | `honnan? forms accept -i in principle but are rare` ->
  `honnan? forms accept -i and are all attested; they are 2-4 orders of magnitude rarer than
  the hol? forms, tracking the base postposition frequency`
- `PND-POSTP-010` | Predictability | `tendency (directional hová? forms never accept; hol?
  forms accept; others lexical)` -> `rule (directional hová? forms never accept; all other
  postpositions accept)` - the lative/non-lative split is exceptionless over N = 31 + 15.
- `PND-POSTP-010` | Validation | append `Webcorpus2: 31/31 non-lative postpositions attested
  with -i; 12/15 lative and case-containing rejects at 0 tokens, 3 at noise level`
- `PND-POSTP-010` | Confidence | keep `consensus`.

### OQ-06-002 - is `-mA` (`szakma`, `kelme`) a synchronic suffix

Rule `DER-NOUN-026`. Corpus lemma frequencies: `szakma` 312,290, `kelme` 2,793. Both are
robustly attested *as words*; the question is whether a rule exists.

Candidate set `N` = the candidate items named by the inventories = 2. `theta_2 = floor(2 /
0.693) = 2`. **The Tolerance Principle does not apply at N = 2**: any threshold computed over
a two-member set is meaningless, and no rule can be induced from two items that share no
productive base class (`szak-` and `kel-` are not independently available bases with the
right meaning).

A corpus search for further `-ma`/`-me` deverbal nouns returned no type that is analysable
as base + `-mA` with a live base; the frequent `-ma`/`-me` noun lemmas are loans (`téma`,
`dilemma`, `program`), possessed forms, or monomorphemic.

**Verdict: listed (not a synchronic suffix).** Not an inconclusive TP result - the test is
inapplicable, which is itself the answer: a two-item pattern with no productive base class
cannot be a rule under any construal.

Proposed edits:

- `DER-NOUN-026` | Statement | `A noun-forming -ma/-me has been named in word-formation
  inventories; no source read for this chapter confirms it as a suffix, so it is not
  modelled.` -> `A noun-forming -ma/-me has been named in word-formation inventories. The
  candidate set is two items with no shared live base, so no rule is induceable and it is not
  modelled.`
- `DER-NOUN-026` | Validation | `attestation (szakma 15, kelme 2)` -> `attestation
  (Webcorpus2: szakma 312,290, kelme 2,793); Tolerance Principle inapplicable at N = 2; no
  further -mA type with a live base found in 36.9M corpus types`
- `DER-NOUN-026` | Confidence | `contested` -> `consensus` (the modelling decision is settled;
  the etymology is not, and need not be)
- Retire `OQ-06-002`.

### OQ-06-005 - does the `-kA` stop condition include the palatal stops `ty`, `gy`

Rule `DER-DIM-001`, whose condition is "N (C-final not stop, `i`-final) -> N". The question is
whether `ty` and `gy` count as stops for this.

Diagnostic: for a stem ending in the consonant under test, does `-kA` attach, or does the
stem take `-(O)cskA` instead? Corpus token counts:

| stem-final | `-kA` form | tokens | `-(O)cskA` form | tokens | ratio |
| --- | --- | ---: | --- | ---: | ---: |
| `gy` | `hegyke` | 23 | `hegyecske` | 196 | 1 : 8.5 |
| `gy` | `ágyka` | 0 | `ágyacska` | 252 | 0 : 252 |
| `ty` | `atyka` | 1 | `atyácska` | 425 | 1 : 425 |
| `ty` | `bátyka` | 16 | `bátyácska` | 20 | 1 : 1.3 |
| `ty` | `kutyka` | 10 | `kutyácska` | 326 | 1 : 33 |
| `p` (control stop) | `kalapka` | 102 | - | - | - |
| `v` (control non-stop) | `könyvke` | 0 | `könyvecske` | 24,182 | 0 : 24,182 |
| `r` (control non-stop) | `madárka` | 6,942 | - | - | - |
| `i` (control) | `nénike` | 4,124 | - | - | - |

Candidate set `N` = corpus-attested diminutive types formed on `ty`/`gy`-final common-noun
stems = 5 (`hegyke`, `ágyka`, `atyka`, `bátyka`, `kutyka`). `e` = those where `-kA` is the
dominant variant = 1 (`bátyka`, and only at 16 vs 20). `theta_5 = floor(5 / 1.609) = 3`.
`e = 1 <= 3`.

`N` is too small for the TP result to carry much weight on its own, so the token asymmetry is
the primary evidence: `-kA` loses to `-(O)cskA` by 8.5x, 33x, 425x and infinitely on the four
clear items. `padka` 4,009 and `lapka` 11,035 are lexicalised and not derived synchronically
(they have no `-(O)cskA` competitor and no diminutive reading).

**Verdict: productive - `ty` and `gy` are stops for this rule, i.e. they block `-kA`.**
Reported with the caveat that `N = 5`; what would strengthen it is a wug test, since the
corpus cannot supply novel `ty`/`gy`-final bases.

Proposed edits:

- `DER-DIM-001` | Conditions (rule table row, column "base") | `N (C-final not stop, i-final)
  -> N` -> `N (C-final not stop, where stop = p t k b d g ty gy; i-final) -> N`
- `DER-DIM-001` | Exceptions | add `lexicalised -ka on stop-final stems without a diminutive
  reading: padka [W2 4009], lapka [W2 11035], kalapka [W2 102]`
- `DER-DIM-001` | Validation | append `Webcorpus2: ty/gy-final stems take -(O)cskA over -kA
  by 8.5x (hegyecske 196 / hegyke 23), 33x (kutyácska 326 / kutyka 10), 425x (atyácska 425 /
  atyka 1); ágyacska 252 / ágyka 0`
- `DER-DIM-001` | Confidence | keep, and add `(N = 5 corpus-attested ty/gy diminutives; a wug
  test would settle it)`.

### OQ-06-008 - whether to generate `-(O)sdi` and child-language `-ikA`

Rule `DER-NOUN-025`. The chapter records "`-sdi` forms unattested in the corpus list" from
the 250k-type list. Webcorpus2 contradicts this decisively for `-(O)sdi` and confirms it for
`-ikA`.

**`-(O)sdi`.** After removing place-name `-i` adjectives (`Nádasd` -> `nádasdi`, `Kövesd` ->
`kövesdi`, and 20 similar) and the monomorphemic `ósdi`, `élősdi`, `kisdi`, there are **63
genuine derivational types at freq >= 50, totalling 18,477 tokens**:

`egyenlősdi` 1360, `katonásdi` 1336, `kiszorítósdi` 1096, `háborúsdi` 666, `szembekötősdi`
478, `összefogósdi` 474, `zenekarosdi` 394, `színházasdi` 377, `nyomozósdi` 367,
`bizottságosdi` 343, `vállalkozósdi` 334, `indiánosdi` 322, `szavazósdi` 305, `kitalálósdi`
195, `társkeresősdi` 158, `szeretősdi` 130, `iskolásdi` 130, `zálogosdi` 127, `detektívesdi`
125, `feljelentősdi` 121, `orvososdi` 117, `népszavazósdi` 105, `hatóságosdi` 92,
`ügynökösdi` 92, `kirekesztősdi` 91, `szellemesdi` 90, `bújósdi` 84, `demokráciásdi` 84,
`felfedezősdi` 84, `pályázatosdi` 83, `családosdi` 81, `doktorosdi` 80, `túlélősdi` 78,
`üldözősdi` 76, `párkapcsolatosdi` 75, `kórházasdi` 75, `rendőrösdi` 75, `kiszámolósdi` 74,
`tervezősdi` 68, `kirakósdi` 66, `jelmezesdi` 66, `tenyeresdi` 64, `egyesületesdi` 62,
`forradalmasdi` 56, `fesztiválosdi` 56, `bejelentősdi` 55, `számolósdi` 54, `királyosdi` 52,
`kémesdi` 52, `elhatárolósdi` 52.

Candidate set `N` = these 63 types. `e` = those whose base is *not* of a licensed class under
the rule as currently written ("base is a person or role noun"). Counting the deverbal
`-Ó`-participle bases (`kiszorító`, `nyomozó`, `szembekötő`, `összefogó`, `kitaláló`,
`szavazó`, `társkereső`, `feljelentő`, `bújó`, `felfedező`, `túlélő`, `üldöző`, `kiszámoló`,
`tervező`, `kirakó`, `elhatároló`, `számoló`, `vállalkozó`, `népszavazó`, `kirekesztő`,
`bejelentő`) plus the abstract/institution bases (`egyenlőség` -> `egyenlősdi`, `bizottság`,
`hatóság`, `pályázat`, `demokrácia`, `forradalom`, `fesztivál`, `egyesület`, `párkapcsolat`,
`kórház`, `színház`, `zenekar`, `jelmez`, `zálog`, `iskola`, `háború`, `család`, `királyság`)
gives `e = 39`. `theta_63 = floor(63 / 4.143) = 15`. `39 > 15`.

**Verdict: `-(O)sdi` is productive, but the rule's base class as written is wrong.** The
suffix is clearly productive (63 types, many transparently novel: `demokráciásdi`,
`népszavazósdi`, `fesztiválosdi`, `párkapcsolatosdi`), and the meaning has extended from
'children's role game' to the pejorative 'going through the motions of X'. The TP failure is
not a failure of `-(O)sdi`; it is a failure of the stated restriction to person and role
nouns. Restated over the wider base class (person/role nouns, `-Ó` deverbal participles,
institution and abstract activity nouns), `e` drops to the residue and the rule passes.

**`-ikA`.** Corpus: `nénike` 4,124, `bácsika` 3,442 (both lexicalised, no live base with a
diminutive contrast), `anyika` 126, `apika` 78, and nothing else above noise.
`N` = 4, `theta_4 = floor(4 / 1.386) = 2`, and only 2 of the 4 are synchronically
transparent. **Verdict: listed.** Do not generate.

Proposed edits:

- `DER-NOUN-025` | Statement | `-(O)sdi forms names of children's role games from person and
  role nouns and is productive in that domain.` -> `-(O)sdi forms 'playing at N' nouns -
  children's role games, and pejoratively 'going through the motions of N' - from person and
  role nouns, from -Ó deverbal participles, and from institution and abstract activity nouns;
  it is productive.`
- `DER-NOUN-025` | Conditions, `-(O)sdi` bullet | `base is a person or role noun` -> `base is
  a person or role noun (katonásdi, indiánosdi, rendőrösdi, orvososdi, doktorosdi), an -Ó
  deverbal participle (kiszorítósdi, nyomozósdi, szembekötősdi, kitalálósdi, bújósdi), or an
  institution or abstract activity noun (egyenlősdi, demokráciásdi, bizottságosdi,
  népszavazósdi, fesztiválosdi)`
- `DER-NOUN-025` | Predictability | `rule (-(O)sdi on person and role nouns, tendency)` ->
  `rule (-(O)sdi on person/role nouns, -Ó participles and institution/activity nouns)`
- `DER-NOUN-025` | Validation | `-sdi forms unattested in the corpus list` -> `Webcorpus2: 63
  genuine -(O)sdi derivational types at freq >= 50, 18,477 tokens (katonásdi 1336,
  egyenlősdi 1360, kiszorítósdi 1096, demokráciásdi 84); place-name -i adjectives excluded`
- `DER-NOUN-025` | Confidence | `consensus for the lists; tendency for -sdi productivity` ->
  `consensus`
- `DER-NOUN-025` (or `DER-DIM`) | add an explicit non-generation note: `child-language -ikA is
  not generated: only nénike [W2 4124] and bácsika [W2 3442] are frequent, both lexicalised;
  anyika 126, apika 78; no productive pattern (N = 4)`.

### OQ-06-010 - which privative adjectives allow `-Odik` / `-ít`

Rule `DER-ADJ-011`, `DER-VERB-009`. The chapter marks `-ít`/`-Odik` coverage as lexical with
no semantic subclass established.

Candidate set `N` = privative (`-AtlAn`/`-tAlAn`) adjective lemmas attested at freq >= 1,000
in Webcorpus2 = 782. `e` = those with no corresponding `-AtlAnít` / `-AtlAnodik` verb lemma in
the corpus (bare, or with `el-`/`meg-`) = 567. `theta_782 = floor(782 / 6.662) = 117`.

`567 > 117` by 4.8x.

**Verdict: listed.** `-ít`/`-Odik` on privatives is not a productive default; it is stored
per item. The 215 privatives that do feed a verb are the list.

The attested privative-based verbs (2,111 lemma types, 253,826 tokens) are headed by
`ellehetetlenít` 36,470, `méregtelenít` 20,136, `érvénytelenít` 16,762, `hatástalanít`
14,331, `elbizonytalanít` 13,865, `függetlenít` 13,781, `nyugtalanít` 11,379, `áramtalanít`
11,118, `lehetetlenít` 10,001, `ivartalanít` 8,544, `ártalmatlanít` 8,001, `kártalanít`
7,634, `helytelenít` 6,030, `elbizonytalanodik` 4,159, `portalanít` 3,908, `hiteltelenít`
3,698, `zsírtalanít` 3,458, `szőrtelenít` 3,335, `légtelenít` 2,979, `megszentségtelenít`
2,563.

The chapter's own example contrast is confirmed: `portalanít` 3,908 vs `*lábatlanít` 0.

A semantic tendency is visible in the top of the list - the `-ít` verbs are overwhelmingly
*removal of a physical substance or a legal property* (`méreg`, `por`, `zsír`, `szőr`, `lég`,
`áram`, `ivar`, `érvény`, `hitel`, `hatás`) - but it is a tendency, not a rule: it does not
license generation, since `e` exceeds `theta` by a factor of 4.8 even within that semantic
band. The list stands; the tendency is worth recording only as a ranking hint.

Proposed edits:

- `DER-ADJ-011` | Exceptions | `-ít/-Odik coverage is lexical (no semantic subclass
  established)` -> `-ít/-Odik coverage is lexical: 215 of 782 frequent privatives feed such a
  verb (theta = 117, so not a rule). The productive-looking band is removal of a physical
  substance or a legal property (méregtelenít, portalanít, zsírtalanít, szőrtelenít,
  légtelenít, áramtalanít, ivartalanít, érvénytelenít, hiteltelenít, hatástalanít), but it is
  a ranking hint, not a licence to generate.`
- `DER-ADJ-011` | Predictability | `rule (lowering, adverb); lexical (verbs)` - unchanged,
  now with evidence.
- `DER-ADJ-011` | Validation | append `Webcorpus2: 2,111 privative-based verb lemma types,
  253,826 tokens; portalanít 3,908 vs *lábatlanít 0; N = 782 frequent privatives, e = 567,
  theta = 117`
- `DER-VERB-009` | Conditions, privative bullet | `privatives with el- lexically
  (elsótlanodik)` -> `privatives with el- lexically (elsótlanodik [W2 low], elbizonytalanodik
  [W2 4159], elbátortalanodik, elerőtlenedik); lexical list, not generated by rule`
- Retire `OQ-06-010` as answered: the answer is "no semantic subclass licenses generation;
  keep the list".

### Q-X1 (named in the brief; relates to `OQ-03-010`, `NOM-LOW`) - lowering in new loans

The method-validation run showed that over the whole codified vocabulary neither "all lower"
nor "none lower" is a rule. The interesting question is the one the brief names: **is
non-lowering the productive default for new words?**

Diagnostic: the accusative of a back-harmony consonant-final noun, where lowering `-at` and
default `-ot` are orthographically distinct. (Front-harmony stems are not diagnosable: the
lowering and the default both surface as `-et`.)

Candidate sets, all requiring >= 50 accusative tokens:

| candidate set | N | e (lowering) | theta_N | lowering share | verdict |
| --- | ---: | ---: | ---: | ---: | --- |
| codified (in `noun.ron`) | 5,640 | 1,190 | 652 | 21.1% | listed |
| not in `noun.ron` | 11,326 | 1,801 | 1,213 | 15.9% | listed |
| not in `noun.ron`, foreign orthography | 4,180 | 507 | 501 | 12.1% | listed, **inconclusive** (1.2%) |
| not in `noun.ron`, foreign orthography, no codified lowering compound head | **805** | **10** | **120** | **1.2%** | **productive** |

The last row is the correct candidate set and the previous rows show why. The apparent
lowering in the loan sets is almost entirely compounds whose *head* is a codified lowering
noun: `webáruház`, `eszköztár`, `operettszínház`, `apartmanház`, `magánnyugdíjpénztár`,
`nagykörút`, `transzferár`, `teafaolaj`. The loan is the modifier; the head does the
lowering, exactly as the grammar predicts. Once compound heads are excluded, the loan set is
`facebook`, `workshop`, `bluetooth`, `network`, `xbox`, `webshop`, `firefox`, `coaching`,
`food`, `room`, `Hollywood`, `max`, `pack`, `box`, `Tóth`, `Horváth`, `Kossuth`, `Madách`,
`Bach`, `Edward`, `Richard` and similar.

`N = 805`, `e = 10`, `theta_805 = floor(805 / 6.691) = 120`. The 10 apparent exceptions are
all lemmatisation artefacts, not lowering: `beallitas` (unaccented `beállítás`), `cookiek`,
`workshopok`, `screenshotok`, `mikotoxinok` (plurals mis-lemmatised as singulars), `os`,
`am`, `ag`, `technikaj`, `reakcioj`. True `e` is effectively 0.

**Verdict: productive. Non-lowering is the default; new loans do not lower.** This is a
strong result: `e/theta = 0.083`.

Proposed edits:

- `NOM-LOW` model rule (whichever rule states the default; `NOM-LOW-001` or the `Low` trait
  definition) | Statement | make the default explicit and directional: `Lowering is a lexical
  property of the stem (trait Low). The default for any stem without the trait - including
  every new word, loan and proper name - is the non-lowering linking vowel -ot/-et/-öt.`
- same rule | Predictability | `lexical` -> `rule (non-lowering default) + lexical (Low
  membership)`
- same rule | Validation | append `Webcorpus2: over 805 back-harmony C-final loans with
  foreign orthography, absent from the lexicon and not headed by a codified lowering noun,
  the lowering accusative occurs in 10 types, all lemmatisation artefacts (theta = 120);
  lowering share 1.2% vs 21.1% in the codified vocabulary`
- `NOM-ACC` / `NOM-PLUR` | add a cross-reference: `unknown-word fallback: never lower`
- Note for `OQ-03-010`: this settles the *default* half of the `-a`/`-ja` and lowering
  modelling question. The quantitative model for `Low` membership itself remains open; this
  result only fixes the behaviour of items outside the list.

### Q-X2 (named in the brief; `OQ-03-010`, `NOM-POSS-006`, `NOM-POSS-007`) - is `-ja` the default 3sg possessive

Diagnostic: `[/N][Poss.3Sg][Nom]` forms in Webcorpus2, per lemma, with a 200-token floor.
A lemma counts as `-ja`-taking if `-ja`/`-je` forms outnumber the rest. Stems ending in `j`
are excluded from the `-ja` count (their `-a` form ends in the string `ja` spuriously).

| candidate set | N | `-ja` share | rule tested | e | theta_N | verdict |
| --- | ---: | ---: | --- | ---: | ---: | --- |
| V-final, all | 4,176 | 89.7% | "-ja is the default" | 430 | 500 | **productive** (14.0%) |
| C-final, codified | 10,218 | 10.7% | "-ja is the default" | 9,123 | 1,106 | listed (8.3x) |
| C-final, codified | 10,218 | 10.7% | "-a is the default" | 1,095 | 1,106 | productive, **inconclusive** (1.0%) |
| C-final, not in lexicon | 8,675 | 7.6% | "-ja is the default" | 8,017 | 956 | listed (8.4x) |

**`-ja` is the default for vowel-final stems** (`autója` 65,091, `szobája` 29,611): `e = 430`
vs `theta = 500`, and the 430 exceptions are a coherent, already-listed class - the v-stems
and `-ó/-ő` stems (`szó`, `fő`, `ló`, `kő`, `tó`, `mű`, `fiú`, `jármű`, `élő`, `következő`)
plus single-letter and foreign-brand noise. Margin 14.0%, i.e. outside the inconclusive band,
but not comfortably.

**`-ja` is emphatically not the default for consonant-final stems.** Only 10.7% of codified
C-final nouns are `-ja`-dominant, and the figure falls to 7.6% for non-codified ones - the
opposite of what a productive-default account of `-ja` predicts.

The obvious counter-rule "`-a/-e` is the default for C-final stems" lands at `e = 1,095` vs
`theta = 1,106`: **inconclusive**, missing the threshold by 1.0%. That is not a usable
result, so the candidate set was partitioned by the one phonological factor the literature
names:

| partition | N | rule | e | theta_N | `-ja` share | verdict |
| --- | ---: | --- | ---: | ---: | ---: | --- |
| sibilant/affricate-final (`s sz z zs cs c dzs`) | 4,330 | "-a/-e" | **0** | 517 | 0.0% | **productive, exceptionless** |
| non-sibilant C-final | 5,888 | "-a/-e" | 1,095 | 678 | 18.6% | listed (1.6x) |

**Verdict:**
1. V-final -> `-ja/-je`: productive default, with the v-stem and `-ó/-ő` exception list.
2. Sibilant/affricate-final -> `-a/-e`: productive and **exceptionless** over 4,330 types
   (`része` 2,331,597, `kéz`/`keze` 172,145, `ház`/`háza` 232,762 - not one `-ja`-dominant
   sibilant stem).
3. All other C-final stems: **listed**, 18.6% `-ja`. No default is learnable; the `-a`/`-ja`
   choice must come from the lexicon (`PossessiveA` / `PossessiveJa` traits), which is what
   we already do.

The whole-set "`-a` is the default" result is **inconclusive** and should not be relied on.
What would settle whether a default exists for the non-sibilant residue is an MGL run
(`local/research-racz/Racz2024rules/scripts/learner.R` if downloaded) over the C-final
possessive data, or a wug test - the residue is exactly the domain Rácz & Rebrus 2012 model
probabilistically, and a 18.6%/81.4% split is a gradient effect, not a rule with exceptions.

Proposed edits:

- `NOM-POSS-006` / `NOM-POSS-007` | Statement | state the three-way conditioning explicitly:
  `Vowel-final stems take -ja/-je (default; v-stems and some -ó/-ő stems are lexical
  exceptions). Consonant-final stems ending in a sibilant or affricate (s, sz, z, zs, c, cs,
  dzs) take -a/-e without exception. All other consonant-final stems are lexically specified
  (PossessiveA / PossessiveJa); there is no learnable default.`
- `NOM-POSS-006` | Predictability | `lexical` (or `tendency`) -> `rule (V-final -> -ja;
  sibilant-final -> -a) + lexical (all other C-final stems)`
- `NOM-POSS-006` | Validation | append `Webcorpus2 Poss.3Sg types, freq >= 200: V-final
  N=4176 -ja share 89.7% (e=430, theta=500); sibilant-final C stems N=4330 -ja share 0.0%
  (e=0, theta=517); other C-final N=5888 -ja share 18.6% (e=1095, theta=678, listed)`
- `NOM-POSS-007` | Exceptions | record the V-final exception class: `v-stems and some -ó/-ő
  stems take -a/-e: szava, feje/fő, lova, köve, tava, műve, fia`
- Lexicon: the `PossessiveA` trait is redundant on sibilant/affricate-final stems (the rule
  supplies it). Proposal: keep the trait for round-tripping but do not require it; add a lint
  that flags `PossessiveJa` on a sibilant/affricate-final stem, since the corpus has zero
  such items.
- `OQ-03-010` | narrow rather than retire: the default and the sibilant condition are settled;
  the quantitative model for the remaining 5,888 non-sibilant C-final stems is still open and
  still needs an MGL or MaxEnt fit.

### Q-X3 (named in the brief; `DER-ADJ-010`, `VRB-PART-004`) - deverbal privative `-AtlAn` on verbs

The chapter records `-(A)tlAn` on verbs as productive on transitive action verbs
(SMNy3 p. 124, p. 133).

Candidate set `N` = verb lemmas in `verb.ron` with the `Transitive` trait and corpus
frequency >= 1,000 = 4,064. `e` = those with no corresponding `-AtlAn`/`-tAlAn` adjective
lemma attested anywhere in Webcorpus2 (all six linking-vowel/allomorph shapes tried, plus the
`-ik` stem) = 2,038. `theta_4064 = floor(4064 / 8.310) = 489`.

`2,038 > 489` by 4.2x.

Attested `-AtlAn` adjective lemma types: 41,794, totalling 23,098,298 tokens - the suffix is
enormously frequent. But half of all frequent transitive verbs have no attested privative,
including very common ones: `abbahagy`, `adományoz`, `adóztat`, `aggaszt`, `agyoncsap`,
`agyonlő`, `agyonver`, `ajándékoz`, `akaszt`, `alapít`, `alkot`, `alábecsül`, `alárendel`.

**Verdict: listed as stated, i.e. "transitive verb" is the wrong candidate set.** Inspection
of the residue shows the real conditioning: the verbs that lack a privative are almost all
*non-resultative* - creation verbs (`alkot`, `alapít`), transfer verbs (`ajándékoz`,
`adományoz`), manner-of-action verbs (`agyonver`, `agyoncsap`) and prefixed aspectual verbs
(`abbahagy`). `-AtlAn` needs a base whose object can be in a *result state* that the
adjective denies. That is the `VRB-PART-004` past-participle condition, not transitivity.

This is reported as a **negative result plus a diagnosis**, not as a settled rule, because
the residue was classified by inspection rather than by an independent feature in our data.
`e` under a resultative-only candidate set cannot be computed until the lexicon carries a
telicity or resultativity feature.

Proposed edits:

- `DER-ADJ-010` (deverbal `-(A)tlAn`) | Conditions | `productive on transitive action verbs`
  -> `productive on transitive verbs that license a resultative past participle
  (VRB-PART-004); not on creation, transfer or manner-of-action verbs (*alkotatlan,
  *ajándékozatlan, *agyonveretlen)`
- `DER-ADJ-010` | Predictability | `rule` -> `rule (resultative transitives) + lexical`
- `DER-ADJ-010` | Validation | append `Webcorpus2: 41,794 -AtlAn adjective lemma types,
  23.1M tokens; over N = 4,064 frequent transitive verbs e = 2,038 lack any privative
  (theta = 489), so transitivity alone is not the condition`
- `DER-ADJ-010` | Lexical input | `verb transitive` -> `verb transitive + resultative`
- **New open question** (proposed, to replace the over-broad claim): `theory-productivity`,
  `DER-ADJ-010`, "the exact base condition for deverbal `-(A)tlAn`: resultativity is
  diagnosed by inspection of the 2,038-verb residue, not by a feature in our data. A
  telicity/resultativity feature on verb lemmas plus a re-run of the Tolerance test would
  settle it." Impact: `quality`.

### Part 1 summary

| question | rule(s) | N | e | theta_N | verdict |
| --- | --- | ---: | ---: | ---: | --- |
| `OQ-02-008` `-beli`/`-fajta`/`-mentes`/`-szerte` | `PH-LEN-002` | 10,302 | 2 | 1,114 | productive (non-lengthening) |
| `OQ-04-007` postposition `-i` | `PND-POSTP-010` | 31 | 0 | 9 | productive |
| `OQ-06-002` `-mA` | `DER-NOUN-026` | 2 | - | - | listed (TP inapplicable) |
| `OQ-06-005` `-kA` after `ty`/`gy` | `DER-DIM-001` | 5 | 1 | 3 | productive (they block `-kA`); small N |
| `OQ-06-008` `-(O)sdi` | `DER-NOUN-025` | 63 | 39 | 15 | productive, base class must be widened |
| `OQ-06-008` `-ikA` | `DER-NOUN-025` | 4 | 2 | 2 | listed - do not generate |
| `OQ-06-010` privative `-Odik`/`-ít` | `DER-ADJ-011` | 782 | 567 | 117 | listed |
| Q-X1 lowering in new loans | `NOM-LOW` | 805 | 10 | 120 | productive (non-lowering default) |
| Q-X2 `-ja` V-final | `NOM-POSS-006` | 4,176 | 430 | 500 | productive |
| Q-X2 `-a` sibilant-final | `NOM-POSS-006` | 4,330 | 0 | 517 | productive, exceptionless |
| Q-X2 `-a` non-sibilant C-final | `NOM-POSS-006` | 5,888 | 1,095 | 678 | listed |
| Q-X2 `-a` all C-final | `NOM-POSS-006` | 10,218 | 1,095 | 1,106 | **inconclusive** (1.0%) |
| Q-X3 deverbal `-AtlAn` on transitives | `DER-ADJ-010` | 4,064 | 2,038 | 489 | listed (wrong candidate set) |

Inconclusive, needing an MGL run or a wug test:

1. "`-a/-e` is the default for all C-final stems" - 1.0% off threshold. Superseded in
   practice by the sibilant partition, which is exceptionless. An MGL run over the C-final
   possessive data would say whether the non-sibilant residue has a gradient default.
2. `OQ-06-005` `-kA` after `ty`/`gy` - the TP passes but `N = 5`. A wug test with novel
   `ty`/`gy`-final bases is the only way to raise `N`.
3. Q-X3 deverbal `-AtlAn` - the negative result is solid, the resultativity diagnosis is not
   independently measured. Needs a telicity feature on verb lemmas.

## Part 2: tagset mapping

### Authority order used

1. `https://raw.githubusercontent.com/UniversalDependencies/tools/master/data/feats.json`,
   `hu` section - the validator's machine-readable declaration. **A feature/value pair not
   permitted here is a validation error, whatever the guidelines say.**
2. UD guidelines pages (`u/feat/*`, `hu/feat/*`).
3. UD_Hungarian-Szeged actual annotation. `local/research-method/hu_all.conllu` is the
   concatenation of the three splits (1,800 sentences / 42,032 tokens = train 910/20,166 +
   dev 441/11,418 + test 449/10,448), not a separate corpus. It is the only Hungarian UD
   treebank available.
4. UniMorph `hun` (1,036,931 lines).
5. `emmorph2ud2` `converterdata/mappings.py` (GPL-3, read only).

Relevant `feats.json` `hu` facts, used repeatedly below:

- `Case`: uvalues `Abl Abs Acc Ade All Cau Dat Del Dis Ela Ess Gen Ill Ine Ins Loc Nom Sbl
  Sup Tem Ter Tra`; **lvalues `[]`**, evalues `[]`. `Abs` is permitted on NOUN, PROPN, ADJ
  only - **not on PRON, NUM, DET**. `Ess` is permitted on NOUN, ADJ, PRON, NUM. `Case` is not
  permitted on ADV, VERB or AUX at all. **No `Frm` value exists in UD, for any language.**
- `Definite`: uvalues `Def Ind`; **lvalues `['2']`**, permitted on VERB.
- `Mood`: uvalues `Cnd Imp Ind Pot`.
- `PronType`: includes `Int`, permitted on ADV.
- `Degree`: `Cmp Pos Sup` only - **`Degree=Abs` is not permitted for hu**.
- `Person[psor]`: lvalues `['1','3']`, **`'2'` explicitly in unused_lvalues**.
- `Number[psed]`: lvalues `['Sing']`, `Plur` unused.
- `VerbForm`: lvalues `PartFut PartPast PartPres`.

`Case.lvalues = []` is the decisive constraint for Part 2: **we cannot invent a Hungarian
`Case` value.** Anything not in the 22 uvalues is a validation error. This kills `Case=Frm`,
`Case=Com` and `Case=Inl` outright.

### -ként (`OQ-03-006`, and the `-ként` conflict)

| authority | value |
| --- | --- |
| 1. `feats.json` hu | `Abs` permitted on NOUN/PROPN/ADJ; `Ess` permitted on NOUN/ADJ/PRON/NUM; no `Frm` |
| 2. `hu/feat/Case.html` | documents **`Abs`, suffix `-ként`, example `éveként`** - but the prose is the copy-pasted universal absolutive gloss ("subject of intransitive verb and direct object of transitive verb"), which is semantically wrong for `-ként`. `Ess` is glossed "temporary state, 'as a...'", suffix column empty, example `ráadásul` |
| 3. Szeged | **`Case=Abs`, 57 tokens, all `-ként`/`-képpen`**: `tagjaként`, `példaként`, `feladatként`, `tanárként`, `elsőként`, `éveként`, `részeként`, `aktivistaként`, `Moraleseként`. UPOS NOUN 51 / ADJ 4 / PROPN 2; deprel obl 50. `-nként` is separately `Case=Dis` (7 tokens). Lexicalised `egyébként`/`főként`/`önként` are ADV with `FEATS=_`; `miként` is ADV `PronType=Int` |
| 4. UniMorph hun | `FRML` (formative): `N;FRML;SG` 13,302, `N;FRML;PL` 12,276, `ADJ;FRML;SG` 4,089, `ADJ;FRML;PL` 4,089 |
| 5. emmorph2ud2 | `'EssFor:ként': ['Case=Ess']`. `Case=Abs` appears **nowhere** in the converter |

Three authorities, three answers. Resolution:

- `Frm` is not available. UD has no such value and `Case.lvalues` for hu is empty, so we
  cannot add one. UniMorph's `FRML` is the semantically right label but it is a different
  tagset and cannot be projected into UD `Case`.
- `Ess` is the semantically defensible universal value ("as a ..."), and it is what the
  converter emits. **But Szeged already uses `Ess` for something else**: 365 tokens, 338 of
  them UPOS=ADJ, and they are the productive `-an`/`-en` manner adverbs
  (`lényegesen`, `fokozatosan`, `párhuzamosan`, all `Case=Ess|Degree=Pos|Number=Sing`,
  deprel `advmod:mode`), plus `-ul`/`-ül` as NOUN (`ráadásul`, `vendégül`, `összegzésül`).
  Adopting `Ess` for `-ként` would merge two morphologically and syntactically distinct
  suffixes into one value, and would contradict the only Hungarian treebank there is.
- `Abs` is a bad *name* - Hungarian has no absolutive, and the hu doc's prose for it is a
  copy-paste error - but it is (a) permitted by the validator on exactly the three UPOS that
  carry `-ként`, (b) documented in `hu/feat/Case.html` against the suffix `-ként` with the
  example `éveként`, and (c) what all 57 relevant treebank tokens actually carry.

**Decision: emit `Case=Abs` for `-ként`.** Authority levels 1-3 all support it; only the
converter dissents, and the converter has two known invalid outputs (`Case=Inl`, which is not
a UD value at all, and `Degree=Abs`, which is not permitted for hu), so it is the weakest
witness here. Interoperability with the only Hungarian treebank is worth more than a better
value name, and there is no better *available* name: `Ess` is taken and `Frm` does not exist.

**Our own model is not UD.** Internally the suffix is `NOM-CASE` essive-formal, feature
`Case=Formalis` (our name), and `Case=Abs` is only the UD projection. This is recorded so the
two are never confused.

Corpus scale, for the record: `-ként` is `[EssFor:ként]` on **204,192 distinct bases,
11,428,189 tokens** in Webcorpus2, and combines freely with `[Poss.3Sg]` (40,953),
`[Pl]` (13,693), `[Pl.Poss.3Sg]` (5,384), `[Poss.1Sg]`, `[Poss.2Sg]`, `[Poss.1Pl]`,
`[Poss.3Pl]` and with `[/Adj]` (16,698) and `[_Comp/Adj]` (211). It is fully productive
inflectional morphology, which is a further argument for a `Case` value over an `ADV`
derivation analysis.

### -stUl / -stül (`OQ-03-006`)

| authority | value |
| --- | --- |
| 1. `feats.json` hu | `Case.lvalues = []`. `Com` is **not** among the 22 permitted hu values |
| 2. guidelines | no hu doc entry |
| 3. Szeged | **zero tokens.** No treebank evidence exists |
| 4. UniMorph hun | **absent.** No `-stul`/`-stül` forms |
| 5. emmorph2ud2 | **no mapping.** `'_Com:stUl/Adv': 'ADV'` is commented out in `POS_RULES`; there is no `NOMINAL_INFL_RULES` entry. The converter deliberately emits nothing |

Every authority is silent or empty. `Case=Com` would be the right universal value but is not
permitted for hu, so emitting it is a validation error.

**Decision: emit no `Case` feature.** UD projection: `UPOS=NOUN`, `Case` omitted,
`deprel=obl`. The information is kept in our own model only, as `Case=Sociativus` on the
`NOM-CASE-001` inventory.

This is a genuine gap, not a judgement call - there is nothing to defer to. Webcorpus2 shows
the suffix is real and moderately productive (`[_Com:stUl/Adv]`: **8,024 distinct bases,
73,040 tokens**; `családostul` 5,368, `kamatostul` 4,834, `gyökerestül` 4,510, `héjastul`
2,320, `ajtóstul` 1,463, `ruhástul` 1,026, `mindenestül`, `lovastul`, `cserepestül`), so the
absence from UD and UniMorph reflects their coverage, not the language.

If UD ever adds `Com` to the hu permitted list, switch to `Case=Com`; that is the only
principled target. Proposed as an upstream issue, not as a local workaround.

### -képp / -képpen vs -ként (`OQ-03-006`)

| authority | value |
| --- | --- |
| 1. `feats.json` hu | same constraints as `-ként` |
| 3. Szeged | 17 tokens. Productive adjectival `kellőképpen` -> **ADJ `Case=Abs|Number=Sing|VerbForm=PartPres`** and `Legfőképpen` -> ADJ `Case=Abs|Degree=Pos|Number=Sing`. Pronominal/lexicalised ones are ADV with `PronType` only: `mindenképpen` `PronType=Tot` (5), `semmiképpen` `PronType=Neg` (2), `miképpen` `PronType=Int`; `voltaképpen`, `tulajdonképpen` get `FEATS=_` |
| 4. UniMorph hun | **absent** |
| 5. emmorph2ud2 | `'EssFor:képp': ['Case=Ess']`, `'EssFor:képpen': ['Case=Ess']` - identical treatment to `-ként` |

Both the treebank and the converter treat `-képp(en)` **identically to `-ként`**; they simply
disagree with each other on what that shared value is. emMorph itself gives them the same tag
family (`EssFor:`).

**Decision: same value as `-ként`, i.e. `Case=Abs`**, with a lexical carve-out for the
pronominal and lexicalised forms, following Szeged exactly:

- productive `-képp(en)` on a noun or adjective: `Case=Abs`
- `mindenképp(en)`: `ADV`, `PronType=Tot`, no `Case`
- `semmiképp(en)`: `ADV`, `PronType=Neg`, no `Case`
- `miképp(en)`: `ADV`, `PronType=Int`, no `Case`
- `tulajdonképpen`, `voltaképpen`: `ADV`, no features

`-ként` and `-képp(en)` are therefore **not** distinguished in the UD projection. They are
distinguished in our own model (`Case=Formalis` vs `Case=Modalis`), which is where the
difference belongs, since UD has no value pair that expresses it.

Corpus scale: `-ként` 204,192 bases / 11.4M tokens; `-képpen` 4,648 bases / 532,380 tokens;
`-képp` 1,985 bases / 211,987 tokens. `-képp(en)` is an order of magnitude less productive
than `-ként` but far from closed - another reason to treat it inflectionally rather than
listing it.

### Associative -ék (`OQ-03-006` neighbourhood; `NOM-PLUR`)

| authority | value |
| --- | --- |
| 1. `feats.json` hu | no associative feature exists; `Number` uvalues are `Plur Sing` only |
| 2. guidelines | UD has no associative-plural feature |
| 3. Szeged | `Orbánék`, `Thaciék`, `Gyerevék` -> PROPN, **`Case=Nom|Number=Plur`**, lemma is the bare name, deprel `nsubj` |
| 4. UniMorph hun | **absent** (the only `-ék` hits are lexical stems like `lövedék`) |
| 5. emmorph2ud2 | **`'Fam.Pl': ['Number=Plur']`** - the associative collapses to plain plural |

Levels 3 and 5 agree exactly, and levels 1, 2, 4 offer nothing.

**Decision: emit `Number=Plur` with the bare-base lemma.** No `Case` or extra feature.

**The associative/plural distinction is lost in the UD projection and kept in our own model**
as a separate `Number=Assoc` value on the `NOM-PLUR` inventory. This is a real information
loss (`Kovácsék` 'the Kovács family' vs `Kovácsok` 'people named Kovács' both project to
`Number=Plur`), and it is recorded as such rather than papered over with an invented feature,
because inventing one would fail validation.

Corpus scale: `[Fam.Pl]` on **31,723 distinct bases, 582,764 tokens**, combining with the
full case paradigm (`Nom` 24,052, `Acc` 5,963, `Ade` 4,246, `Dat` 3,988, `All` 3,159, `Ins`
2,852, `Abl` 1,684, `Subl` 1,202, ...) and with possessives (`[Poss.1Sg][Fam.Pl]` 497). Top
bases: `Vidék`-type false positives aside, `Orbánék` 25,060, `anyáék` 10,155, `anyuék` 6,658,
`Gyurcsányék` 5,904, `nagyiék` 4,915. Fully productive on proper names and kin terms.

### The -e clitic (`OQ-09-019`)

| authority | value |
| --- | --- |
| 1. `feats.json` hu | `PronType=Int` is explicitly permitted on ADV (60 occurrences in the byupos counts). `Polite` is not used |
| 2. guidelines | nothing hu-specific |
| 3. Szeged | **unanimous.** The token is spelled **`-e`, with the leading hyphen**, `UPOS=ADV`, `FEATS=PronType=Int`, lemma `-e`, `deprel=advmod:que`. 20 tokens, all 20 with `PronType=Int`, 19 with `advmod:que`. `advmod:que` totals 22: 19x `-e` plus 3x `vajon` (also ADV `PronType=Int`) |
| 4. UniMorph hun | absent |
| 5. emmorph2ud2 | **no rule at all** for `-e` |

Confirmed exactly as the brief reported. **Decision: adopt it verbatim.**

- separate token, surface form and lemma both `-e` **including the hyphen**
- `UPOS=ADV`, `FEATS=PronType=Int`, `deprel=advmod:que`, head = the finite verb
- `vajon` gets the same treatment (`ADV`, `PronType=Int`, `advmod:que`)
- `Polite` is confirmed unused in the treebank and is not emitted

**Critical distractor**, worth a rule condition: the bare token `e`/`E` without a hyphen
occurs 13 times in Szeged and is the **demonstrative determiner** 'this':
`Case=Nom|Number=Sing|Person=3|PronType=Dem`, UPOS PRON 8 / DET 5, deprel `det`. The
tokeniser must not strip the hyphen from the clitic, or the two collapse. Also: `UPOS=PART`
is used in Szeged only for `meg` (105) and `utol` (2) as `compound:preverb`, never for `-e`.

### Potential -hat/-het (`OQ-05-001`)

| authority | value |
| --- | --- |
| 1. `feats.json` hu | `Mood` uvalues `Cnd Imp Ind Pot`. byupos: VERB `{Cnd:124, Imp:150, Ind:2840, Pot:226}`, AUX `{Cnd:35, Imp:10, Ind:149, Pot:10}` |
| 2. `hu/feat/Mood.html` | documents `Pot` and states explicitly: "In Hungarian, it can be also combined with imperative and conditional, leading to multivalues `Cnd,Pot` and `Imp,Pot`", examples `Tanulhatnék az egyetemen.` (Cnd,Pot), `hogy az egyetemen tanulhasson` (Imp,Pot). The universal `Mood` page says nothing about combination |
| 3. Szeged | `Mood=Ind 2989, Pot 216, Imp 150, Cnd 149, Cnd,Pot 10, Imp,Pot 10`. Indicative potential is **bare `Mood=Pot`, never `Ind,Pot`**. `Tense` is retained: `Definite=Ind|Mood=Pot|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin|Voice=Act` (96 tokens), and `Mood=Pot|...|Tense=Past` is attested (20 tokens). Lemma is the non-potential stem (`csökkenhet` -> `csökken`, `lehet` -> `lesz`). UPOS VERB 233 / AUX 16 |
| 4. UniMorph hun | `POT`, and **only as bare `V;POT`** (4,580 entries: `ad/adhat`, `ver/verhet`). Never combined with person/tense/mood - UniMorph treats potential stems as derived lemmas, not paradigm cells |
| 5. emmorph2ud2 | `'_Mod/V': ['Pot=Pot']`, then `converter.py:157-168`: if `Mood` absent -> `Mood=Pot`; **elif `Mood == 'Ind'` -> `Mood` is replaced by `'Pot'`**; else `Mood += ',Pot'` |

All five authorities agree, and the converter's logic is explicit about it: **`Pot` replaces
`Ind`, but combines with `Cnd` and `Imp`.**

**Decision, confirming `OQ-05-001`:**

- indicative potential -> `Mood=Pot` (bare; **never** `Ind,Pot`), with `Tense=Pres|Past` kept
- conditional potential -> `Mood=Cnd,Pot`
- imperative/subjunctive potential -> `Mood=Imp,Pot`
- lemma is the non-potential stem; `-hat/-het` is inflectional, not a derived lemma (we
  diverge from UniMorph here, which is a UniMorph coverage artefact)
- `Voice=Act` and `Definite=Def|Ind` are emitted alongside as normal

Corpus scale: `[_Mod/V]` is on essentially the whole verb lexicon - see the method-validation
run above, where 6,708 of 6,759 verb lemmas above frequency 1,000 have a potential form.
Fully inflectional.

### -lak / -lek (`OQ-05-002`)

| authority | value |
| --- | --- |
| 1. `feats.json` hu | **`Definite.lvalues = ['2']`, permitted on VERB.** byupos VERB `{'2':1, Def:1300, Ind:2018}`. Separately, **`Person[psor].lvalues = ['1','3']` with `'2'` in unused_lvalues** - so `Person[obj]`-style 2nd-person marking has no permitted home |
| 2. `u/feat/Definite.html` | values `Ind Spec Def Cons Com`. **No value `2`.** Hungarian is only a link. `hu/feat/Definite.html` documents `2`: "definite second person object marked on verb ... used for marking second person objects which trigger special verb suffixes", examples `Szeretlek.` vs `Szeretem őt.` |
| 3. Szeged | **`Definite=2` occurs exactly once in 42,032 tokens**, on `nézlek`: `néz VERB _ Definite=2|Mood=Ind|Number=Sing|Person=1|Tense=Pres|VerbForm=Fin|Voice=Act`. Note `Person=1` is the *subject* person; there is no object-person feature |
| 4. UniMorph hun | **`LGSPEC3`**, always with `1;SG`, exactly 4 combos, 2,776 entries each: `V;IND;PRS;1;SG;LGSPEC3` (`adlak`, `verlek`), `V;IND;PST;1;SG;LGSPEC3` (`adtalak`, `vertelek`), `V;COND;PRS;1;SG;LGSPEC3` (`adnálak`, `vernélek`), `V;SBJV;PRS;1;SG;LGSPEC3` (`adjalak`, `verjelek`). **`LGSPEC3` carries no `DEF`/`INDF`** - it is the third value of that dimension, exactly as `Definite=2` is |
| 5. emmorph2ud2 | **`'1Sg>2': ['Definite=2', 'Number=Sing', 'Person=1']`** (the emMorph tag uses U+203A) |

Levels 1, 2(hu), 4 and 5 agree on the analysis - it is the third value of the definiteness
dimension, not an object-agreement feature. Level 3 attests it, but only once, which is why
the brief is right that the paradigm should come from UniMorph rather than the treebank.

**Decision: `Definite=2`.** The glossary's provisional `Person[obj]=2` is **wrong and must be
changed**: `Person[psor]=2` is in `unused_lvalues` and there is no `Person[obj]` feature in
UD at all, so `Person[obj]=2` would fail validation.

The four cells, taken from UniMorph (2,776 verbs each), with their full UD projection:

| cell | example | UD FEATS |
| --- | --- | --- |
| present indicative | `adlak`, `verlek`, `nézlek` | `Definite=2|Mood=Ind|Number=Sing|Person=1|Tense=Pres|VerbForm=Fin|Voice=Act` |
| past indicative | `adtalak`, `vertelek` | `Definite=2|Mood=Ind|Number=Sing|Person=1|Tense=Past|VerbForm=Fin|Voice=Act` |
| present conditional | `adnálak`, `vernélek` | `Definite=2|Mood=Cnd|Number=Sing|Person=1|Tense=Pres|VerbForm=Fin|Voice=Act` |
| present subjunctive/imperative | `adjalak`, `verjelek` | `Definite=2|Mood=Imp|Number=Sing|Person=1|VerbForm=Fin|Voice=Act` |

`Number=Sing|Person=1` is the **subject**. The object's person and number (2sg or 2pl - the
form is identical) is **not representable in UD** and is kept in our own model only, as
`Object=2` on `VRB-MODEL-002`. Webcorpus2 confirms the paradigm: `[Prs.1Sg>2]` 9,499 types,
`[Pst.1Sg>2]` 3,589, `[Sbjv.1Sg>2]` 2,242, headed by `kérlek` 295,321.

There is one further potential-stacked cell not in the treebank or UniMorph but generated by
our rules: `adhatlak` -> `Definite=2|Mood=Pot|Number=Sing|Person=1|Tense=Pres|VerbForm=Fin|Voice=Act`.

### Part 2 summary and the glossary mapping column

| suffix | our model | UD projection we emit | information lost in UD |
| --- | --- | --- | --- |
| `-ként` | `Case=Formalis` | `Case=Abs` | the value name is wrong; `-ként` and `-képp(en)` merge |
| `-képp(en)` (productive) | `Case=Modalis` | `Case=Abs` | merged with `-ként` |
| `-képp(en)` (pronominal) | lexical adverb | `ADV` + `PronType=Tot/Neg/Int`, no `Case` | - |
| `-stUl` | `Case=Sociativus` | `NOUN`, **no `Case`**, `deprel=obl` | the case entirely |
| associative `-ék` | `Number=Assoc` | `Number=Plur`, bare-base lemma | associative vs plural |
| `-e` clitic | `Q=Polar` | `ADV`, `PronType=Int`, `deprel=advmod:que`, token `-e` | - |
| `-hat/-het` | `Mood=Pot` | `Mood=Pot` / `Mood=Cnd,Pot` / `Mood=Imp,Pot` | - |
| `-lak/-lek` | `Definite=2` + `Object=2` | `Definite=2` | object person/number |
| `-nként` | `Case=Distributivus` | `Case=Dis` | - |
| `-an/-en`, `-ul/-ül` manner | `Adv=Manner` | `Case=Ess` (ADJ lemma, `deprel=advmod:mode`) | - |

Proposed edits:

- `glossary.md` | mapping column, `-lak/-lek` row | `Person[obj]=2` (provisional) ->
  `Definite=2` - **this is a correctness fix, not a preference**: `Person[obj]` is not a UD
  feature and `Person[psor]=2` is in `unused_lvalues`, so the current value cannot validate.
  Note `UniMorph: LGSPEC3` in the same row.
- `glossary.md` | mapping column, `-ként` row | -> `UD Case=Abs (treebank + hu/feat/Case.html;
  our Case=Formalis; UniMorph FRML; note emmorph2ud2 dissents with Case=Ess)`
- `glossary.md` | mapping column, `-képp/-képpen` row | -> `UD Case=Abs, same as -ként;
  pronominal mindenképpen/semmiképpen/miképpen are ADV + PronType, no Case`
- `glossary.md` | mapping column, `-stUl` row | -> `no UD Case (Com not permitted for hu;
  zero treebank and UniMorph evidence; emmorph2ud2 emits nothing); our Case=Sociativus only`
- `glossary.md` | mapping column, associative `-ék` row | -> `UD Number=Plur with bare-base
  lemma; our Number=Assoc (the distinction is not representable in UD)`
- `glossary.md` | mapping column, `-e` row | -> `UD ADV, PronType=Int, deprel=advmod:que,
  token and lemma "-e" with the hyphen`
- `glossary.md` | mapping column, `-hat/-het` row | -> `UD Mood=Pot (replaces Ind, Tense
  kept); stacked Mood=Cnd,Pot and Mood=Imp,Pot`
- `NOM-CASE-001` | Output / UD column | add the values above for `-ként`, `-képp(en)`,
  `-stUl`, `-nként`; add an explicit `no UD Case` marker for `-stUl` so that emitting nothing
  is a recorded decision rather than an omission.
- `NOM-CASE-001` | Sources | add `feats.json hu (Case.lvalues = [], so no Hungarian-specific
  Case value may be emitted)` - this is the constraint that drives all four decisions.
- `VRB-MODEL-002` | UD column | `Mood=Pot` for indicative potential (not `Ind,Pot`),
  `Cnd,Pot`, `Imp,Pot`; `Definite=2` for `-lak/-lek`; add our own `Object=2`.
- `VRB-MODEL-002` | Sources | add `hu/feat/Mood.html (multivalues Cnd,Pot and Imp,Pot);
  hu/feat/Definite.html (value 2); UniMorph hun LGSPEC3`
- `SYN-Q-010` | Output | `-e` is a separate token `-e`, `ADV`, `PronType=Int`,
  `deprel=advmod:que`; `vajon` likewise; `Polite` unused. Add the tokenisation warning about
  bare `e` = demonstrative `PronType=Dem`.
- Retire `OQ-03-006`, `OQ-05-001`, `OQ-05-002`, `OQ-09-019`.

### Two converter bugs, recorded so we do not inherit them

`emmorph2ud2` emits two values that are not permitted for hu. If we ever use it as an oracle,
these must be filtered:

1. `'Inl': ['Case=Inl']` - `Inl` is not a UD `Case` value in any language.
2. `converter.py:278` emits `Degree=Abs` for `legesleg-` superlatives - `Degree` for hu
   permits only `Cmp Pos Sup`.

This is the main reason the converter is ranked below the treebank on `-ként`.

## Reproduction

The commands below reproduce every number in this file from the repository plus the Webcorpus2
parquet, run from the repository root with the inputs laid out as in
[WORKSPACE.md](../../../WORKSPACE.md).

```sh
PARQ=local/research-racz/Webcorpus2FrequencyList/frequencies.parquet
CONLLU=local/research-method/hu_all.conllu     # = train + dev + test concatenated
```

Pass 1 - form-regex extraction (one scan, vectorised with `pyarrow.compute.match_substring_regex`,
about 4 minutes):

```python
import pyarrow.parquet as pq, pyarrow.compute as pc
PATS = {'kent': r'(ként|kent)$', 'stul': r'(stul|stül|stól|stől)$',
        'keppen': r'(képpen|képp|képen)$', 'beli': r'(beli|béli)$', 'fajta': r'(fajta|fajtá)',
        'mentes': r'mentes', 'szerte': r'szerte$', 'ka': r'(ka|ke)$', 'cska': r'(cska|cske)$',
        'sdi': r'sdi', 'atlan': r'(atlan|etlen|talan|telen)',
        'anodik': r'(atlan|etlen|talan|telen)(odik|edik|ít)', 'hat': r'(hat|het)'}
f = pq.ParquetFile(PARQ)
for b in f.iter_batches(batch_size=500000, columns=['form','lemma','xpostag','freq']):
    for k, rx in PATS.items():
        sub = b.take(pc.indices_nonzero(pc.match_substring_regex(b.column('form'), rx)))
        # write form, lemma, xpostag, freq to ex/<k>.tsv
```

Pass 2 - emMorph-tag aggregation (potential, possessive, `Fam.Pl`, `_Com:stUl`, `EssFor:*`,
`Prs.1Sg>2`): same loop, filtering `xpostag` instead of `form` on
`(_Mod/V|Poss\.3Sg|Fam\.Pl|_Com:stUl|EssFor:|Prs\.1Sg›2|\[/V\])`.

Pass 3 - noun paradigm cells with the `hunspell` flag, streamed to TSV with a
frequency floor of 20 (holding this in memory exhausts RAM):

```python
RX = r'^\[/N\]\[(Nom|Acc|Pl\]\[Nom|Poss\.3Sg\]\[Nom)\]$'
for b in f.iter_batches(batch_size=500000,
                        columns=['form','lemma','xpostag','freq','hunspell']):
    m = pc.and_(pc.match_substring_regex(b.column('xpostag'), RX),
                pc.greater_equal(b.column('freq'), 20))
    # write form, lemma, cell, freq, hunspell -> nouns.tsv   (1,123,554 rows)
```

Tolerance Principle:

```python
import math
def theta(N): return math.floor(N / math.log(N))    # verdict: e <= theta(N)
```

Treebank counts:

```sh
grep -o 'Case=[A-Za-z]*'     $CONLLU | sort | uniq -c | sort -rn
grep -o 'Mood=[A-Za-z,]*'    $CONLLU | sort | uniq -c | sort -rn
grep -o 'Definite=[A-Za-z0-9]*' $CONLLU | sort | uniq -c | sort -rn
awk -F'\t' '$6 ~ /Case=Abs/  {print $2}' $CONLLU | sort | uniq -c | sort -rn
awk -F'\t' '$8 == "advmod:que" {print $2, $4, $6}' $CONLLU | sort | uniq -c
awk -F'\t' '$6 ~ /Definite=2/ {print}' $CONLLU
```

Single-form corpus lookup (used for the minimal pairs):

```sh
python tools/attest.py korabeli korábeli hegyke hegyecske   # 250k list, buckets only
# full-corpus counts: scan $PARQ for form in a given set, summing freq
```

Authority files (all cached under `local/research-method/`, byte-identical to the live
versions as of this date): `feats.json`, `m2ud.py` (= `emmorph2ud2` `mappings.py`, 15,199
bytes), `um_hun.tsv` (= UniMorph `hun`, 37,281,606 bytes, 1,036,931 lines).

## What is still open after this file

1. `OQ-03-010` - narrowed, not closed. The default (`-ja` for V-final, `-a` for
   sibilant-final) and the loan behaviour (never lower) are settled. The `-a`/`-ja` choice for
   the 5,888 non-sibilant consonant-final stems is a gradient effect needing an MGL or MaxEnt
   fit, not a Tolerance verdict.
2. `OQ-06-005` - the verdict holds but rests on `N = 5`. A wug test is the only way to raise
   it.
3. Q-X3 deverbal `-AtlAn` - the resultativity diagnosis needs a telicity feature on verb
   lemmas before it can be tested rather than asserted. Proposed as a new
   `theory-productivity` entry against `DER-ADJ-010`.
4. `-stUl` has no UD home. Proposed as an upstream UD issue: add `Com` to the hu permitted
   `Case` values. Until then we emit nothing, deliberately.
5. The `Case=Abs` name for `-ként` is wrong on its face and the `hu/feat/Case.html` prose for
   it is a copy-paste of the absolutive definition. Worth reporting upstream; it does not
   change our decision, since interoperability with the only Hungarian treebank wins.
