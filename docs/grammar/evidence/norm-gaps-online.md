# Norm-gap evidence found online

Date: 2026-09-16. Companion to [`norm-gaps.md`](norm-gaps.md), which worked offline against the cached advisory
archive. This pass took the 18 questions that archive could not settle and searched for primary
legal texts, open-access academic studies and corpus evidence. Seventeen are resolved, one advanced,
none remain open. One further question from the offline pass's "partially advanced" list
(`OQ-01-002`) fell out of the same corpus work and is resolved here too, so 18 in total. The last
two were settled by native-speaker judgements given in session on 2026-09-16, each of which turned
out to generalise further than the example that prompted it.

Method: one question at a time, no subagents. Web search and fetch for the documentary questions;
[`tools/kwic.py`](../../../tools/kwic.py) over the project corpus (not distributed; see [corpus-variants.md](corpus-variants.md#data)) for the usage questions, since the Webcorpus2 frequency
list is lowercased and strips apostrophes and so cannot answer them. Downloaded study texts are
kept as local oracles under `local/research-akh/` and `local/research-lit/`, gitignored.

Two sources carry most of this pass:

- `KESZLER2016`, by a member of the MTA Magyar Nyelvi Bizottság, contains the AkH11 -> AkH12
  change list (pp. 95-100, 190 entries) and the AkH10/AkH11 rule changes (p. 83). It replaces the
  Wikipedia summary the rulebook had been leaning on for dictionary-level claims.
- `IHM2006`, retrieved from the Nemzeti Jogszabálytár, is a legal text and therefore outside
  copyright; the spelling alphabet can be used directly rather than via a CC BY-SA encyclopaedia.

## Resolved

### OQ-01-011 spelling alphabet (`WS-ALPH-013`)
The decree's 1. melléklet 1.2 was retrieved and is in force. All 26 entries match the encoded
table exactly, including `Q Kvelle`, `W dupla-Vilmos`, `X ikszes`, `Y ipszilon` and the two-variant
entries `A Aladár ~ Antal`, `M Mátyás ~ Mihály`.
Decision: confidence `consensus` -> `normative` for the domain the decree regulates; cite `IHM2006`
directly and drop the dependency on `HUWIKI-BETUZ`.

### OQ-01-012 no-break space after one-letter words (`WS-CHAR-008`)
The typesetting literature states only that line-final `a`, `és` and the like are "generally"
(`általában`) carried to the next line, and presents it as page-layout aesthetics. AkH12 is silent.
Decision: typographic habit, not a norm. Never flag its absence.

### OQ-01-008 dating of simplified doubling and `dz`/`dzs` (`WS-DIG-010`, `WS-LET-002`)
AkH10 (1954) made two rule changes: simplified doubling was extended to the ends of suffixed proper
names (`Aranynyal` -> `Arannyal`, `Keménynyel` -> `Keménnyel`), and the general digraph-doubling
rule was extended to `dz` and `dzs` (`fogódzzam` -> `fogóddzam`, `edzzük` -> `eddzük`). AkH11 (1984)
made `dz` and `dzs` indivisible in hyphenation (`mad-zag`, `hod-zsa` -> `ma-dzag`, `ho-dzsa`),
which is when they became full members of the alphabet.
Evidence: `[KESZLER2016 p. 83]`.
Decision: both dates now rest on an orthography-committee member's study. The proper-name case also
closes the unverified exception `WS-DIG-010` carried.

### OQ-10-007 when simplified doubling became general (`REP-ARCH-001`, `REP-ARCH-004`)
Simonyi Zsigmond's 1903 school orthography (`Az új helyesírás`), rejected by the Academy and
published through the education ministry, spread through schools, the press and book publishing;
the Academy adopted its reforms almost in full in 1922.
Evidence: `[LACZKO24 pp. 12-13]`, with `[KESZLER2016 p. 83]` for the 1954 proper-name extension.
Decision: the dating no longer rests on Wikipedia. A pre-1954 text can legitimately show simplified
common words beside unsimplified suffixed proper names, which the repair rule now states.

### OQ-08-010 suffixing foreign names with an apostrophe (`PUN-APOS-004`)
The apostrophe plays no part. Suffixation is decided by the final letter, by the ordinary
`[AkH12 §217]` test, and the corpus splits exactly along that line:

| name ends in | direct | hyphenated | direct share |
|---|---|---|---|
| pronounced `l`, `n`, `r` | 4,726 | 596 | 88.8% |
| silent or unusual (`e`, `ey`, `y`, `ie`) | 764 | 1,098 | 41.0% |

Direct: `O’Nealtől`, `O’Connorról`, `O’Nealt`. Hyphenated: `O’Toole-nak`, `O’Malley-vel`,
`O’Kelley-t`, `O’Brian-re`. Overall 15,295 direct against 2,153 hyphenated.
Decision: never add a hyphen because a name contains an apostrophe; apply `§217` to the final letter.

### OQ-01-005 archaic `ly`, `ty` in surnames for sorting (`WS-COLL-008`)
AkH12 §15 decomposes the two-character letters of archaic-spelled surnames without restricting the
set, and orders by the written characters rather than the sound value.
Decision: `Thököly` -> `l y`, `Vörösmarty` -> `t y`, like `cz`, `th`, `ew`. The rule's "contested"
marking is withdrawn.

### OQ-01-007 stroke letters (`WS-COLL-009`)
§15's own example sorts `Møsstrand` under `o`, so a stroke counts as a `mellékjel` for this rule.
The `hu_HU.UTF-8` collation agrees: `Đakovo` sorts with `D`, `Łódź` with `L`, `Ørsted` with `O`.
Decision: `ł -> l`, `đ -> d`, `ø -> o`, `ħ -> h`, `ŧ -> t`.

### OQ-01-001 vowel quality in letter names before a suffix (`WS-ALPH-009`)
AkH12 §288 a) words the parallel acronym rule as "írásban nem nyújtjuk az `a`, `e`, `o`-t", which
presupposes that the lengthening is real and merely unwritten.
Decision: a TTS or phonetic generator lengthens (`a` -> [aː], `e` -> [eː], `o` -> [oː]); a spelling
checker must not. This is the third of the three options the question listed.

### OQ-07-005 accuracy of the AkH11 -> AkH12 change list (`ORT-TOG-047`)
Checked against the editor's own list. Of 37 spot-checked items, 34 match verbatim. The three
absent ones are `együttérez`, `papíralapú` and `orvos szakértő`; they carry no norm-change claim.
`papíralapú` is nevertheless solid under the noun + `-ú/-ű` main rule, since `alapú` is written
apart only when it names a shape (`négyzet alapú`).
Decision: cite `KESZLER2016` as primary; keep the three as flagged exceptions.

### OQ-07-023 four dictionary-level claims (`ORT-NUM-017`, `ORT-NUM-021`, `ORT-63-003`, `ORT-CAP-018`, `ORT-CAP-023`, `ORT-CAP-026`)
All four confirmed verbatim in the editor's change list:

| AkH11 | AkH12 |
|---|---|
| `másodiki v. 2-i` | `másodiki v. 2-i v. másodikai v. 2-ai` |
| `három nullás v. 3:0-s` | `három nullás v. 3:0-s v. 3-0-s` |
| `vendéglátó-ipari` | `vendéglátóipari` |
| `aquinói v. Aquinói Szent Tamás` | `Aquinói Szent Tamás` |

### OQ-09-004 possessive agreement with `önök`/`maguk` (`SYN-AGR-020`)
Both agreements are correct and the choice is the speaker's: `Maguknak mi a véleménye?` ~
`... véleményük?`, and so `az önök autója` ~ `az önök autójuk`. `Önök` is the more polite pronoun,
which is a style difference, not an agreement one.
Evidence: `[ENYELV 2016-06-23]`. This answer was in the cached archive; the offline pass simply
queried the wrong terms.
Decision: never flag either. The rule's `akiknek a` exception generalises to the polite pronouns.

### OQ-09-014 sourcing the officialese pattern list (`SYN-LINT-034`)
The advisers name the class (`funkcióigés szerkezet`, `terpeszkedő kifejezés`) and list its members
(`átadásra kerül`, `előadásra kerül`, `bemutatásra kerül`, `elhatározásra jut`, `kezdetét veszi`,
`döntést hoz`, `felmentést nyer`, `elbírálást nyer`, `elintézést nyer`, `megállapodást köt`). Across
22 answers they treat these as a property of official and media style and recommend the finite verb
(`bejegyzésre került` -> `bejegyezték`), without calling them errors.
Evidence: `[ENYELV 2013-08-15]`, `[ENYELV 2024-07-08]`, `[ENYELV 2022-09-03]`, `[ENYELV 2023-04-26]`.
Decision: the pattern list and the `style` severity are now sourced rather than asserted. Severity
must never be raised to `error`.

### OQ-09-016 definiteness with ordinal `hányadik` (`SYN-DEF-008`)
`hányadik` patterns with `melyik`, not with `hány`. Of 1,813 corpus hits of `hányadik` + accusative
noun, every classifiable context in a 40-context sample takes the definite conjugation and none the
indefinite: `hányadik helyet taposta ki`, `hányadik rubrikát fogyasztja el`, `hányadik jóvátételt
hajtják be`, `hányadik értéket fejezzük ki`, `hányadik mérkőzést döntötte el`.
Decision: `hányadik` joins the `-ik` group of `SYN-DEF-008` and is `Def`; it does not belong with the
indefinite interrogatives of `SYN-DEF-003 (d), (f)`. This follows Bartos's generalisation: `hányadik X`
is selective and so a DP, while `hány X` is quantificational and not.

### OQ-01-002 possessive of consonant-final letter names (`WS-ALPH-007`)
Letter names take `-je`, not `-e`: `a hold l-je`, `az ajánl l-je`, `a -bAn n-je`, `a sure s-je`,
`a school sz-je`, `melyeknek sz-je`. The apparent `-e` hits are all something else, the interrogative
clitic (`jobbikos-e`) or dates (`január l-e`).
Decision: `l-je`, `n-je`, `s-je`, `sz-je`, `r-je`, `m-je`, `f-je`, `x-je`. This was on the offline
pass's "partially advanced" list rather than the no-evidence list.

### OQ-04-002, OQ-07-017 suffixing decimals (`PND-FRAC-007`, `ORT-NUM-002`)
Not a norm gap at all: it is a reading-dependent rule, and both spellings are correct. The two
readings separate cleanly in the corpus because no Hungarian digit name ends in `d`, so a
`-del`/`-dal` suffix can only come from a denominator (`tizeddel`, `századdal`).

| reading | last spoken word | suffixes | hits |
|---|---|---|---|
| digit-name (`három egész öt`) | the final digit | `-tel`/`-tal` 3,036, `-vel`/`-val` 1,424 | 4,460 (66.5%) |
| denominator (`négy tized`) | `tized`, `század`, `ezred` | `-del`/`-dal` | 2,245 (33.5%) |

Within the digit-name reading the suffix tracks the final digit as predicted: 89.7% of `-vel`/`-val`
follow a final `2` (`kettővel`). `-del`/`-dal` is spread across all final digits, as a
digit-independent suffix must be.
Decision: `1,2-vel` and `1,2-del` are both correct and encode different readings; a lint must never
flag either. The existing R2 default for generation is confirmed as the majority reading.

## Advanced, still open

- **OQ-09-003 non-referential `az`** (`SYN-DEF-009`): Bartos's open-access precursor supplies the
  analysis - `hogy`-clause objects are `{DP, CP}` chains, the expletive `azt` lexicalises the chain
  head, and the matrix verb agrees with whatever fills that head, so an extracted non-DP gives the
  indefinite (`Kit szeretnél, hogy megverjek?` against `Pétert szeretném, hogy megverd.`). His
  generalisation (25) is that the verb takes the definite paradigm iff its object's category is DP.
  The p. 655 ex. 175 examples themselves are not in the 2000 article, so the question stands.
  Evidence: `[BARTOS2000 pp. 161-163]`.


## Settled by native-speaker judgement

### OQ-04-011 suffix of a written-but-unread time (`PND-DATE-011`)
Native-speaker judgement (2026-09-16): `10:00` is read either `tíz nulla-nulla` or `tíz óra`, both
ending in a back `-a` word, so it is always `10:00-ra`; and `tíz harminc` -> `-ra` is the
anti-example showing the suffix simply follows the last read word.

A corpus check afterwards corrected one thing I had over-claimed and confirmed the rest. Counts need
a homograph filter, since sports scores (`50:30-ra vertük`) and durations (`4:30-re` of a skating
programme) share the notation, so treat these as indicative:

| form | `-ra` | `-re` |
|---|---|---|
| `HH:00` | 1,187 | 387 |
| `HH:30` | 2,343 | 22 |

Two findings. First, my draft said a written `:00` does not license a bare-hour reading. It does:
the `-re` hits are genuine clock times (`7:00-re jártam dolgozni`, `04:00-re oda is értem`,
`10:00-re van itthon`), i.e. writers do read `7:00` as bare `hét` and suffix `hétre`. So the
original question's premise was not false, and `HH:00-re` is attested at roughly a quarter of hits.
It is the minority and judged incorrect, so it warrants a `warning`, not an `error`, and never
silent acceptance in generation.

Second, and more useful: the fraction reading never surfaces in the suffix. `HH:30-re` has 22 hits
against 2,343, and several of those are durations rather than clock times. So a digit-written time is
suffixed from its digit reading (`tíz harminc`), not from the fraction system (`fél tizenegy`), even
though the fraction system is the normal way to say the time aloud. My draft claim that
`10:30-ra` ~ `10:30-re` are co-equal variants is withdrawn.
Decision: generate `-ra` for `HH:00`; suffix every digit-written time from its digit reading; flag
`HH:00-re` as a warning only. Default generation reading `óra` (`10 órára`).


### Correction: `jó irányba halad` is not free variation (`SYN-LINT-010`)
This file's first draft cited `jó irányba` ~ `jó irányban halad` as an archive-attested case of free
variation supporting the rule's guards. That was wrong: it took an adviser's hedge for a descriptive
fact. A native-speaker judgement (2026-09-16) is that `jó irányba halad` is the correct form, and the
corpus agrees by a wide margin:

| phrase | `-bA` | `-bAn` | `-bA` share |
|---|---|---|---|
| `jó irány... halad` | 5,508 | 1,092 | 83.5% |
| `rossz irány... halad` | 1,644 | 211 | 88.6% |
| `irány... megy` | 5,080 | 303 | 94.4% |
| `irány... tart` | 2,765 | 51 | 98.2% |
| `irány... halad` (all modifiers) | 17,980 | 8,442 | 68.0% |

The last row is diluted because it mixes two readings, and the contexts show which: `-bAn` collects
the axis cases (`ellentétes irányban haladt`, `déli irányban megy az útvonal`, `egyenes irányban tart
hazafelé`, `minden irányban`), while an evaluative modifier (`jó`, `rossz`, `helyes`) gives the goal
reading and `-bA`. So `irány` is not a free-variation head at all: **the modifier, not the verb,
selects the reading**, and the two forms are not interchangeable for a given reading.
`SYN-LINT-010` now carries this as a guard, because the rule's reverse clause (flagging `Ine` with a
motion verb) would otherwise fire on the perfectly correct `ellentétes irányban haladt`.

Both advisory answers diverge from usage here: the 2012 one preferred `-bAn` with figurative `halad`
on the strength of the Kéziszótár, and the 2026 one called both correct, while usage runs about 5:1
the other way. Advisory register judgements need corpus checking before they are encoded.

### OQ-09-020 register label behind `SYN-LINT-010`
This was recorded as a register judgement resting on prescriptive tradition with no open normative
locator, and this file's first draft floated `-n` dropping from `-bAn` as an account that would have
argued for softening the severity. The minimal pair settles it the other way: `táskába` is 'into the
bag' and `táskában` is 'in the bag', a semantic case contrast rather than two renderings of one
form. With a static verb only the locative is grammatical, so `a táskámba van` is a grammatical
error, not a tolerated colloquialism, and `error` is the right severity for the `Ill` + `van`/`lakik`
pattern. `-n` dropping still explains why the error arises in speech; it does not license the written
form.
Decision: the severity follows from case semantics, not from prescriptive tradition, so the question
is answered rather than merely deferred. The separate `jó irányba halad` claim is withdrawn in the subsection above.

## Reproduction

```
# documentary
https://njt.jog.gov.hu/jogszabaly/2006-6-20-0P                      # OQ-01-011
local/research-akh/keszler2016.txt                                  # OQ-01-008, OQ-07-005, OQ-07-023
local/research-akh/keszler2016-akh12-changes.txt                    # the change list alone
local/research-akh/laczko2024-helyesirasi-tartomanyok.txt           # OQ-10-007
local/research-lit/bartos2000-alanyi-targyas.txt                    # OQ-09-003

# corpus
python tools/kwic.py --sources big --regex --contexts 6 \
  "\bO['’][A-Z][a-z]+(nak|nek|val|vel|ra|re|ban|ben|t|ot|et|nál|nél|ról|ről|tól|től|hoz|hez|ig|é|ék|ja|je)\b" \
  "\bO['’][A-Z][a-z]+-(nak|nek|val|vel|ra|re|ban|ben|t|ot|et|nál|nél|ról|ről|tól|től|hoz|hez|ig|é|ék|ja|je)\b"
python tools/kwic.py --sources big --regex --contexts 5 "\b[lnrsm]-je\b" "\b[lnrsm]-e\b"

# collation
printf 'Dakovo\nĐakovo\nLodz\nŁódź\nOslo\nØrsted\n' | LC_ALL=hu_HU.UTF-8 sort   # OQ-01-007

# archive (offline tool, better query terms than the first pass used)
python tools/enyelv.py "magázás" "egyeztetés"     # OQ-09-004
python tools/enyelv.py "terpeszkedő"              # OQ-09-014
```
