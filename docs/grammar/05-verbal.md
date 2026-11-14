# 05 Verbal morphology

Scope: inflection of Hungarian verbs and the inflection-like verbal forms. Covers the morphotactic template and feature model, verb stem classes and how to classify a verb from its citation form, `-ik` verbs, linking vowels, present, past, conditional and imperative (subjunctive) paradigms in the indefinite, definite and `-lak/-lek` conjugations, the infinitive and personal infinitive, participles, adverbial participles, the potential `-hat/-het`, causative selection with hooks into derivation, full paradigms of irregular and suppletive verbs, and verbal prefixes (igekötők) as far as word formation, spelling and lemmatisation are concerned. Choice of conjugation (definite vs indefinite object), prefix placement, negation and modal constructions are syntax (`SYN-DEF`, word order (out of scope), modal syntax (out of scope)); vowel harmony basics are phonology (`PH-HARM`); derivational suffixes in general are `DER-VERB`; spelling principles are `ORT`.

Corpus evidence in this chapter comes from the project attestation list ([`crates/picur-morph-lexicon/data/frequency.ron`](../../crates/picur-morph-lexicon/data/frequency.ron), 250k types): numbers are frequency buckets (half-octaves above the rarest kept word; higher = more frequent), `-` = not in the list. A missing form is not evidence of ungrammaticality; the list is too small for rare verbs.

## Area index
| Area | Topic |
|---|---|
| `MODEL` | morphotactic template, features, stem slots, suffix classes |
| `STEM` | stem classes, closed lists, classification procedure |
| `IK` | `-ik` verbs |
| `LINK` | linking vowels across the paradigm |
| `PRES` | present indicative |
| `PAST` | past indicative |
| `COND` | conditional, past conditional |
| `IMP` | imperative / subjunctive |
| `INF` | infinitive, personal infinitive |
| `PART` | adjectival participles |
| `ADVP` | adverbial participles |
| `POT` | potential `-hat/-het`, `-hatnék` |
| `CAUS` | causative selection, derivation hooks |
| `IRR` | irregular and suppletive verbs |
| `PREV` | verbal prefixes |
| `PARA` | reference paradigm tables |

## Conventions used in this chapter
- Archiphonemes as in the README: `A` a/e, `Á` á/é, `O` o/e/ö, `Ó` ó/ő, `U` u/ü. A verbal `O` is three-way (o back, e front unrounded, ö front rounded); `A`, `Á`, `U`, `Ó` are two-way. Harmony class of a verb stem: `B`, `F`, `R` (`VRB-MODEL-008`, `VRB-STEM-013`).
- Stem slot names (`VRB-MODEL-006`): `S0` citation stem, `SV` synthetic (before vowel-initial Class I suffixes), `SQ` quasi-analytic (before Class II.A suffixes), `SA` analytic (before Class II.B suffixes), `SP` past stem, `SD` derivational stem (before `-Ó`, `-Ás`, `-AndÓ`), `SJ` imperative stem.
- `H` = heavy stem property (`VRB-LINK-002`): the stem takes the low linking vowel before `-sz`, `-tOk`, `-nAk`, `-ni`, `-nA`, `-lAk`.
- Paradigm order in tables: 1sg, 2sg, 3sg, 1pl, 2pl, 3pl, 1sg>2 (`-lak/-lek`).

## MODEL

### VRB-MODEL-001 Morphotactic template of the finite verb
- **Statement**: A finite verb form is built as `[PREV] + STEM + [DER*] + [POT] + MOOD/TENSE + PERSON/NUMBER/DEFINITENESS`; at most one mood/tense marker occurs, and the person suffix fuses number and object definiteness.
- **Conditions**:
  - `PREV`: verbal prefix, written together only when adjacent and preceding (`VRB-PREV-002`); morphologically inert (`VRB-PREV-005`)
  - `DER`: verb-to-verb derivation (causative `-tAt/-At`, frequentative `-gAt/-OgAt`, medio-passive `-Ódik`, `-kOdik`, `-Ul`, `-ít`, ...), recursive, owned by `DER-VERB`
  - `POT`: `-hAt` (`VRB-POT-001`), at most once, after all derivation
  - `MOOD/TENSE`: zero (present indicative), past `-t/-tt/-Ott`, conditional `-n(A)-`, imperative `-j` with assimilations; mood and tense are mutually exclusive in a single word (no synthetic past conditional or past imperative)
  - `PERSON`: set selected by mood/tense and conjugation (`PRES-001/002`, `PAST-011`, `COND-001/002`, `IMP-001`)
- **Output**: `el+olvas+tat+hat+n+ánk` -> `elolvastathatnánk`; `ír+t+am` -> `írtam`; `vár+j+atok` -> `várjatok`.
- **Examples**: `olvas` -> `olvashatnánk`; `ír` -> `írathattam`; `néz` -> `nézegessétek`
- **Exceptions**: periphrastic categories are not in the word: future `fog` + infinitive, past conditional `volna` (`VRB-COND-004`); `van` and `lesz` have synthetic future/suppletion (`VRB-IRR-001/002`).
- **Predictability**: `rule`
- **Lexical input**: stem class (`VRB-STEM-014`), transitivity (for definite cells)
- **Interactions**: `VRB-MODEL-006` selects the stem form; `VRB-LINK-*` inserts linking vowels; `VRB-PREV-004` lemmatisation.
- **Sources**: [SMNy3 pp. 513-523], [KVF98 pp. 289-315], [morphdb grammar V_*, PRES_*, PAST_*, COND_*]
- **Validation**: `goldens` (PARA tables), `oracle:hunmorph-foma`
- **Confidence**: `consensus`

### VRB-MODEL-002 Paradigm cells and feature bundle
- **Statement**: Every inflected verb form carries a feature bundle aligned to UD; the cell inventory is fixed.
- **Conditions**:
  - finite: `VerbForm=Fin`; `Mood` in {Ind, Cnd, Imp, Pot}. **Potential `-hAt/-hEt` replaces `Ind` and combines with `Cnd` and `Imp`**: indicative potential is bare `Mood=Pot` (never `Ind,Pot`) with `Tense=Pres|Past` kept, conditional potential is `Mood=Cnd,Pot`, imperative/subjunctive potential is `Mood=Imp,Pot`; the lemma is the non-potential stem (`csökkenhet` -> `csökken`, `lehet` -> `lesz`), `-hAt/-hEt` being inflectional rather than a derived lemma. `Tense` in {Pres, Past} (imperative: `Tense=Pres`; conditional: `Tense=Pres`, past conditional is analytic); `Person` 1-3; `Number` Sing/Plur; `Definite` Ind/Def/2
  - `-lak/-lek` cells: `Definite=2|Number=Sing|Person=1` in UD, where `Number` and `Person` are the **subject's**. The object's person and number (2sg or 2pl - the form is identical) is **not representable in UD** and is kept in our own model only, as `Object=2`. The four cells, from UniMorph (2,776 verbs each): present indicative `adlak`/`verlek`/`nézlek` `Definite=2|Mood=Ind|Number=Sing|Person=1|Tense=Pres|VerbForm=Fin|Voice=Act`; past indicative `adtalak`/`vertelek` with `Tense=Past`; present conditional `adnálak`/`vernélek` with `Mood=Cnd`; present subjunctive/imperative `adjalak`/`verjelek` with `Mood=Imp` and no `Tense`. Our rules also generate the potential-stacked cell `adhatlak` -> `Definite=2|Mood=Pot|Number=Sing|Person=1|Tense=Pres|VerbForm=Fin|Voice=Act`, which is in neither the treebank nor UniMorph
  - verbs with no definite forms (intransitive) fill only `Definite=Ind` cells; the feature is still emitted
  - nonfinite: `VerbForm=Inf` (with `Person`/`Number` when personal), `VerbForm=PartPres|PartPast|PartFut` (adjectival participles, POS ADJ or VERB per `SYN`), `VerbForm=Conv` (`-vA`, `-vÁn`, `-tAtAn`)
  - `Voice=Cau` for causative derivatives, `Aspect=Iter` for `-gAt` (optional analysis layer, UD Szeged practice)
- **Output**: cells per transitive verb: present ind 13 (6 indef + 6 def + 1 `-lak`), past 13, conditional 13, imperative 13 = 52 finite cells; the same again with `Pot` = 52; plus infinitive 1 + personal infinitive 6, participles 3 (+ `-hAtÓ`), converbs 2-3.
- **Examples**: `látlak` -> `Definite=2|Mood=Ind|Number=Sing|Person=1|Tense=Pres|VerbForm=Fin|Voice=Act` (our model adds `Object=2`); `olvashatnánk` -> `Mood=Cnd,Pot|Tense=Pres|Person=1|Number=Plur|Definite=Ind` (form shared with Def, `VRB-COND-002`); `olvashat` -> `Mood=Pot|Tense=Pres|...`, not `Mood=Ind,Pot`; `menniük` -> `VerbForm=Inf|Person=3|Number=Plur`
- **Exceptions**: syncretic cells produce one form with several bundles (`VRB-PRES-007`, `VRB-COND-002`, `VRB-PAST-011`); the generator emits all bundles.
- **Predictability**: `rule`
- **Lexical input**: transitivity flag (`trans`)
- **Interactions**: glossary mapping in [glossary.md](glossary.md).
- **Sources**: [UD-hu features, `hu/feat/Mood.html` (multivalues `Cnd,Pot` and `Imp,Pot`), `hu/feat/Definite.html` (value `2`), UD_Hungarian-Szeged annotation], [UD-feats] (`Definite.lvalues = ['2']` permitted on VERB; `Mood` uvalues `Cnd Imp Ind Pot`; `Person[psor]` value `2` is in `unused_lvalues`), [UM-hun] (`LGSPEC3`, always with `1;SG`, carrying no `DEF`/`INDF` - the third value of the definiteness dimension, exactly as `Definite=2` is; and `POT` only as bare `V;POT`), [em2ud] (`'1Sg>2': ['Definite=2','Number=Sing','Person=1']`; `'_Mod/V'` logic: `Pot` replaces `Ind`, combines with `Cnd` and `Imp`), [SMNy3 p. 513], [WC2], [ATT evidence/productivity-tagset.md 2026-09-16]
- **Validation**: `oracle:UniMorph`, `goldens`, UD validator conformance against [UD-feats]
- **Confidence**: `measured` for the UD projection (`OQ-05-001`, `OQ-05-002` retired). **Correctness fix 2026-09-16**: the earlier `Person[obj]=2` was wrong and could not validate - there is no `Person[obj]` feature in UD at all, and `Person[psor]=2` is in `unused_lvalues`. The object feature is now `Object=2` in our own model only, never emitted as UD. Szeged attests `Definite=2` exactly once in 42,032 tokens (`nézlek`), which is why the paradigm comes from UniMorph rather than the treebank; Webcorpus 2 confirms it (`[Prs.1Sg>2]` 9,499 types, `[Pst.1Sg>2]` 3,589, `[Sbjv.1Sg>2]` 2,242, headed by `kérlek` 295,321)

### VRB-MODEL-003 Status of the potential -hat/-het
- **Statement**: `-hAt` is modelled as an inflectional slot (feature `Mood=Pot`), not as a derivational suffix, although it is traditionally listed among suffixes (képzők).
- **Conditions**:
  - justification: fully productive with every verb that has an analytic stem; meaning compositional (possibility, permission); does not change part of speech or argument structure; stacks outside all derivation and inside mood/tense/person; the result inflects like a regular `-at/-et` polysyllabic stem (`VRB-POT-002`)
  - counter-evidence recorded: it feeds a few derivations (`-hatÓ`, `-hatatlan`, `-hatnék`), which the model treats as participle/derivation built on the POT layer
  - causative `-tAt/-At`, frequentative `-gAt`, medio-passive `-Ódik` and `-kOdik` are derivation (`DER-VERB`): lexically restricted, meaning often non-compositional, change argument structure
- **Output**: analysis `mehet` -> lemma `megy` + `Mood=Pot`; not lemma `mehet`. Exception: `lehet` as impersonal modal predicate keeps lemma `lehet` in syntactic analysis (modal syntax (out of scope)), with morphological analysis `van/lesz` + `Pot` available.
- **Examples**: `olvashat`, `olvashatott`, `olvashatna`, `olvashasson`, `olvashatom`
- **Exceptions**: `-hat` does not combine with the infinitive in the standard (`?olvashatni` archaic) nor with participle `-vA` (`*olvashatva`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-POT-001..003`, `VRB-MODEL-001`.
- **Sources**: [KVF98 pp. 314-315] (calls it inflectional), [morphdb grammar AFF_HAT: "modal -hat/-het is here treated as inflection"], [UD-hu features Mood=Pot], [SMNy3 p. 513]
- **Validation**: `oracle:hunmorph-foma`
- **Confidence**: `consensus` (traditional school grammar lists it as képző; decision here follows KVF98, morphdb and UD)

### VRB-MODEL-004 Lemma and citation form
- **Statement**: The lemma of a verb is its present indicative indefinite 3sg form, including `-ik` and any verbal prefix written together with it.
- **Conditions**:
  - `-ik` verbs: lemma ends in `-ik` (`dolgozik`, `alszik`)
  - verbs with an `-ik`/zero doublet store both as variants of one lemma, preferred one first (`hull ~ hullik`, `zöldell ~ zöldellik`, `VRB-IK-007`)
  - `sz/d` doublets that are both finite 3sg forms are variants of one lemma (`dicsekszik ~ dicsekedik`, `igyekszik ~ igyekezik`)
  - defective verbs without 3sg indefinite: lemma is the 3sg definite (`kétli`, `sínyli`) per morphdb practice; alternative lemma `kétell` recorded as variant
  - prefix verbs: `megold`, `elmegy`; separated prefix is re-attached for lemmatisation (`VRB-PREV-004`)
- **Output**: `vagyok` -> `van`; `lennék` -> `lesz` (UD Szeged lemmatises the conditional so; `volnék` -> `van`); `mentem` -> `megy`; `eszem` -> `eszik`.
- **Examples**: `ettünk` -> `eszik`; `aludtak` -> `alszik`; `kétlem` -> `kétel`
- **Exceptions**: `van` and `lesz` share forms (`lenne`, `legyen`, `lenni`); ambiguity resolved by SYN, default lemma `van` for copular uses (`VRB-IRR-001`).
- **Predictability**: `rule`
- **Lexical input**: lemma, `ik` flag, variant list
- **Interactions**: `VRB-PREV-004`, `VRB-IK-007`.
- **Sources**: [SMNy3 p. 514], [morphdb grammar V_IK_BASIC_FILTER], [KVF98 p. 289]
- **Validation**: `oracle:emMorph`, `oracle:hunmorph-foma`
- **Confidence**: `normative` (dictionary practice), `consensus`

### VRB-MODEL-005 Conjugation selection interface
- **Statement**: Morphology generates both conjugations for transitive verbs; the choice between indefinite, definite and `-lak/-lek` is made by syntax from the object.
- **Conditions**:
  - definite: object definite (article `a/az`, proper name, possessed noun, 3rd person pronoun, demonstrative, subordinate clause object) -> `SYN-DEF`
  - `-lak/-lek`: subject 1sg and object 2nd person (`téged`, `titeket`, `önt` excluded: formal `Ön` is 3rd person, definite)
  - 1st/2nd person objects otherwise take the indefinite (`lát engem`, `látnak minket`)
  - intransitive verbs have only indefinite forms
- **Output**: cell choice, not form choice.
- **Examples**: `látok egy házat`, `látom a házat`, `látlak`, `látunk titeket`
- **Exceptions**: `-lak/-lek` has no plural-subject counterpart; `kétli`, `sínyli` definite-only defectives (`VRB-STEM-009`).
- **Predictability**: `rule`
- **Lexical input**: transitivity flag
- **Interactions**: `SYN-DEF`.
- **Sources**: [SMNy3 p. 514], [KVF98 pp. 289-291]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-MODEL-006 Stem-form slots
- **Statement**: Every verb is represented by a set of stem forms, and each suffix selects one slot; regular verbs have one form in all slots, irregular and alternating verbs fill slots differently.
- **Conditions**: slot assignment of suffixes:
  - `SV` (synthetic, Class I): present indef 1sg `-Ok/-Om`, 1pl `-Unk`; present def 1sg `-Om`, 2sg `-Od`, front 3sg/2pl/3pl `-i/-itek/-ik`; 3sg `-ik`; past 3sg indef `-Ott` (and past participle `-Ott`)
  - `SQ` (quasi-analytic, Class II.A): 2sg `-sz/-Asz/-Ol`, 2pl `-tOk`, 3pl `-nAk`, infinitive `-ni`, conditional `-nA-`, `-lAk`, past non-3sg `-t-`
  - `SA` (analytic, Class II.B): definite back 3sg/1pl/2pl/3pl `-jA/-jUk/-jÁtOk/-jÁk`, front 1pl `-jük`, imperative `-j`, potential `-hAt`, converb `-vA/-vÁn`, frequentative `-gAt` on polysyllables
  - `SD` (derivational): `-Ó`, `-Ás`, `-AndÓ`, `-Ódik`, `-At` causative on monosyllables
  - `SP`: overrides for the past (`vol-`, `men-`, `jö-`, `te-`, `ve-`, `vi-`, `hi-`, `le-`, `e-`, `i-`, `alud-`, ...)
  - `SJ`: overrides for the imperative (`legy-`, `tegy-`, `jöj-`, `menj-` ...)
- **Output**: regular `vár`: all slots `vár`. Epenthetic `forog`: `SV` `forg`, `SQ/SA` `forog`. v-stem `lő`: `SV/SD` `löv`, others `lő`. `alszik`: `SV` `alsz`, `SQ` `alsz ~ alusz` (present) and `alud` (conditional, infinitive), `SA` `alud` (imperative) `~ al` (potential), `SD` `alv`, `SP` `alud`.
- **Examples**: `forg+ok`, `forog+sz`, `forog+j+on`, `forg+ó`; `löv+ök`, `lő+sz`, `lő+j+ön`, `löv+ő`
- **Exceptions**: past suffix class is split by context (`VRB-PAST-001`): word-final 3sg indef behaves as Class I, non-final as II.A.
- **Predictability**: `rule` (slot map); slot contents `lexical` for irregular classes
- **Lexical input**: stem-form table per irregular or alternating verb
- **Interactions**: all paradigm rules consume slots; `VRB-STEM-*` define slot contents per class.
- **Sources**: [SMNy3 pp. 718-740 (Rebrus 13.3.3-13.3.5, suffix classes I, II.A, II.B)], [SMNy3 p. 761 (sz-d-v stems)], [morphdb grammar V_STEM_VARIANT_FACTORY: analytic, synthetic_inflex, synthetic_derivation]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-MODEL-007 Suffix classes
- **Statement**: Verbal suffixes fall into three attachment classes that determine vowel-zero behaviour of the stem and the linking vowel.
- **Conditions**:
  - Class I (synthetic, vowel-initial): trigger syncope of unstable stem vowels (`forg-ok`, `érz-em`), never take a linking vowel because they begin with a vowel (Type A behaviour, `VRB-LINK-009`)
  - Class II.A (quasi-analytic, consonant-initial `-sz`, `-tOk`, `-nAk`, `-ni`, `-nA`, `-lAk`): take a low linking vowel after heavy stems (`VRB-LINK-002`); do not trigger syncope in non-`-ik` epenthetic verbs; vary in `-ik` epenthetic verbs (`ugranak ~ ugornak`)
  - Class II.B (analytic, `-j`, `-jA` family, `-hAt`, `-vA`, `-gAt`): never trigger syncope, never take a linking vowel; attach to the free stem as if to a word
  - past `-t/-tt/-Ott`: special (`VRB-PAST-001`)
- **Output**: class membership table above (`VRB-MODEL-006`).
- **Examples**: `énekl-ő` (I), `énekel-nek` (II.A), `énekel-jen` (II.B); `ugr-ik`, `ugr-anak ~ ugor-nak`, `ugor-jon`
- **Exceptions**: 2sg `-sz` belongs to II.A but has sibilant allomorph `-Ol` (`VRB-LINK-003`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-STEM-007/008`, `VRB-LINK-*`.
- **Sources**: [SMNy3 pp. 718, 729-740], [T04 §4.1.2.2, §4.1.3 (18) pp. 158-159]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-MODEL-008 Vowel harmony of verbal suffixes
- **Statement**: Verbal suffix vowels harmonise with the stem per `PH-HARM`; verbal suffixes realise `O` three-way, `A/Á/U/Ó` two-way, and some suffixes are invariant.
- **Conditions**:
  - harmony class from the last non-neutral vowel; all-neutral stems (i í e é) front unless antiharmonic (`VRB-STEM-013`); derived stems take harmony from the last suffix (`-gat/-get`, `-tat/-tet`, `-ít` keeps the base harmony: `tanít` B, `segít` F)
  - `O` (1sg `-Ok`, def `-Om/-Od`, 2pl `-tOk`, 2sg `-Ol`, past 3sg `-Ott`, linking `-OtOk`): o / e / ö
  - `A` (3pl `-nAk`, `-lAk`, conditional `-nA`, inf linking `-Ani`, imperative 3pl `-jAnAk`, past person suffixes after `-t-`): a / e only, never ö (`ütötték`, `üssetek`, `lőjenek`)
  - imperative 3sg `-jOn`: o / e / ö (`üssön`, `lőjön`)
  - invariant: `-ni`, `-i`, `-itek`, `-ik` (def), `-nék` (1sg cond indef), `-ék` (archaic imperative 3sg)
- **Output**: `ül` -> `ülök`, `ültök`, `ülnek`, `ültetek`, `üljön`, `üljenek`.
- **Examples**: `tör` -> `töröm`, `töri`, `törjük`; `fut` -> `futok`, `futnak`; `hív` -> `hívok` (antiharmonic)
- **Exceptions**: mixed `B+N` verb stems: harmony from the back vowel (`szeret` front, `harap` back, `bírál` back, `ajándékoz` back, `csinál` back); loan verbs follow the verbaliser (`csekkol` back, `stíröl` front, `RST24`).
- **Predictability**: `rule`; antiharmonic roots `lexical` (closed, `VRB-STEM-013`)
- **Lexical input**: harmony override flag
- **Interactions**: `PH-HARM`.
- **Sources**: [SMNy3 p. 516], [KVF98 pp. 419-424], [morphdb grammar V_BACK_FRONT_HARMONY_FILTER, V_ROUNDING_HARMONY_FILTER], [RST12 §5]
- **Validation**: `goldens`, `oracle:hunmorph-foma`
- **Confidence**: `consensus`

## STEM

### VRB-STEM-001 Citation stem extraction
- **Statement**: The citation stem `S0` is the lemma minus `-ik`; all shape tests are made on `S0` with digraphs counted as one consonant.
- **Conditions**:
  - strip `-ik` only if the lemma is an `-ik` verb (`VRB-IK-001`); `ik`-final non-`-ik` verbs do not exist among native roots except that `-ik` is the verb suffix itself
  - letters: digraphs `cs dz dzs gy ly ny sz ty zs` are one consonant; a doubled letter (`ll`, `rr`, `tt`, `dd`, `ssz`, `ggy`) is one geminate consonant
  - syllable count = number of vowel letters in `S0`
- **Output**: `dolgozik` -> `dolgoz`; `fekszik` -> `feksz`; `metsz` -> `metsz` (final `t+sz` cluster); `varr` -> `varr` (geminate `rr`).
- **Examples**: `játszik` -> `játsz` (CC `t+sz`); `hagy` -> `hagy` (single `gy`); `edz` -> `edz` (single digraph `dz`)
- **Exceptions**: sz-d-v and suppletive verbs have `S0` but their slots are lexical (`VRB-STEM-011/012`).
- **Predictability**: `rule`
- **Lexical input**: `ik` flag
- **Interactions**: `WS-DIG-001`, `WS-DIG-005` for digraph segmentation; `VRB-STEM-014`.
- **Sources**: [SMNy3 p. 514], [morphdb grammar V_IK_BASIC_FILTER]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-STEM-002 Final-segment shape classes
- **Statement**: Regular verbs are classified by the final segment(s) of `S0`; the class drives 2sg, past, imperative and linking-vowel allomorphy.
- **Conditions**: classes (first match in this order):
  - `V`: vowel-final: only v-stem verbs (`VRB-STEM-010`) and suppletive vowel stems
  - `SIB`: final `s`, `sz`, `z`, `dz` (single or after vowel; includes `ssz`, `zz`): 2sg `-Ol`, j-assimilation
  - `T`: final `t` (subclasses `VRB-STEM-003`)
  - `CC`: two different final consonants (clusters ending in a sibilant such as `-tsz`, `-ksz`, `-lsz` carry both CC and SIB; clusters ending in `t` carry CC and T)
  - `GEM`: geminate sonorant `ll`, `rr`; geminate obstruent `gg`, `dd`
  - `SON`: single `l`, `r`, `n`, `ny`, `j`, `ly`
  - `OBS`: any other single consonant: `b`, `d`, `g`, `k`, `p`, `f`, `v`, `h`, `m`, `gy`, `ty`, `c`, `cs`, `zs`, `dzs`
- **Output**: class label set, e.g. `olvas` {SIB}; `játsz` {SIB, CC}; `tart` {T, CC}; `hív` {OBS}; `fúj` {SON}.
- **Examples**: `mos` SIB; `néz` SIB; `edz` SIB; `ad` OBS; `nyom` OBS; `kér` SON; `hány` SON; `rajong` CC; `függ` GEM; `áll` GEM; `köt` T
- **Exceptions**: `-ad/-ed` stems (`VRB-STEM-005`), epenthetic (`VRB-STEM-007/008`) and lexical classes override the defaults.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-LINK-002`, `VRB-PAST-*`, `VRB-IMP-*`.
- **Sources**: [KVF98 pp. 290-295], [SMNy3 pp. 515-518, 523], [T04 §4.1.2.2 (9)], [jepst hvcul.pl ending classes], [magyarispell ige_alanyi.aff context classes]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-STEM-003 t-final subclasses
- **Statement**: t-final verbs split into six subclasses that differ in past, imperative and linking vowel; two of them are closed lists.
- **Conditions**: subclasses of `S0` ending in `t`:
  - `T1` monosyllabic short vowel + `t`: closed list of 8: `fut`, `hat`, `jut`, `köt`, `nyit`, `süt`, `üt`, `vet`
  - `T2` monosyllabic long vowel + `t`: closed list of 8: `lát`, `lót` (only in `lót-fut`), `tát`, `vét`, `fűt`, `hűt`, `műt`, `szít`
  - `T3` polysyllabic long vowel + `t` not `-ít`: `bocsát` (with prefixed `megbocsát`, `elbocsát`, `kibocsát`, `lebocsát`...); only root
  - `T4` `-ít` verbs (`VRB-STEM-004`), open class
  - `T5` polysyllabic short vowel + `t`: open class; mostly `-At/-Et` (`-gAt`, `-tAt`, `-At` causative, `-hAt`), plus roots `szeret`, `követ`, `ugat`, `matat`, `faggat`, `alkot`
  - `T6` consonant + `t`: `C` = `s`/`sz` (`fest`, `oszt`, `választ`, `ereszt`, `ébreszt`) = `T6s`; other `C` (`tart`, `hajt`, `önt`, `ölt`, `ért`, `ment`, `sért`, `teremt`) = `T6c`
- **Output**: behaviour matrix:

| Class | 2sg | 2pl | 3pl / inf | past 3sg | past 1sg | imperative |
|---|---|---|---|---|---|---|
| T1 `köt` | `kötsz` | `köttök` | `kötnek`, `kötni` | `kötött` | `kötöttem` | `köss` |
| T2 `lát` | `látsz` | `láttok` | `látnak`, `látni` | `látott` | `láttam` | `láss` |
| T2 `fűt` | `fűtesz` | `fűtötök` | `fűtenek`, `fűteni` | `fűtött` | `fűtöttem` | `fűts` |
| T3 `bocsát` | `bocsátasz` | `bocsátotok` | `bocsátanak`, `bocsátani` | `bocsátott` | `bocsátottam` | `bocsáss` |
| T4 `tanít` | `tanítasz` | `tanítotok` | `tanítanak`, `tanítani` | `tanított` | `tanítottam` | `taníts` |
| T5 `mutat` | `mutatsz` | `mutattok` | `mutatnak`, `mutatni` | `mutatott` | `mutattam` | `mutass` |
| T6s `fest` | `festesz` | `festetek` | `festenek`, `festeni` | `festett` | `festettem` | `fess` |
| T6c `tart` | `tartasz` | `tartotok` | `tartanak`, `tartani` | `tartott` | `tartottam` | `tarts` |

- **Examples**: `vet` -> `vetettem`, `vess`; `tát` -> `tátasz`, `tátottam`, `táts`; `szeret` -> `szerettem`, `szeress`; `választ` -> `választottam`, `válassz`
- **Exceptions**: **T1 and T2 merge in the past**: 14 of the 16 monosyllables take `-Ott` in all cells (`fut hat jut köt nyit süt üt vet tát vét fűt hűt műt szít`, plus `dűt`), and `lát` is the only `-t-` verb [WC2]. `lát` (and `lót`) behaves like T1 in linking and imperative and like T5 in the past (`láttam`); the rest of T2 behaves like T4. `hat` is variable in the corpus but `hattam` is rejected by the spelling checker, so it belongs with T1 as stated. Within T5 `alkot` is regular (`alkottam`). Heaviness within T2 is lexical, not a class property (`VRB-LINK-003`): `vét` and `bocsát` are heavy, `lát` is light.
- **Predictability**: `lexical` (T1 8 items, T2 8 items, T3 1 root) + `rule` (T4, T5, T6)
- **Lexical input**: none beyond the closed lists (shape decides once lists are known)
- **Interactions**: `VRB-LINK-002`, `VRB-PAST-006`, `VRB-IMP-004..006`.
- **Sources**: [SMNy3 pp. 517-518, 523], [T04 §4.1.4.6.2 (60)-(63) pp. 206-209], [AkH12 §76-78], [KVF98 pp. 290, 295], [Deckstar _past.ts group A list of 8 monosyllables], [Wikt conj tables `vét`, `tát`, `üt`], [WC2], [MTA-HELYES], [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: `goldens`, `attestation` (all 16 t-final monosyllables probed, `t-final-monosyllabic-verbs.tsv` in `local/lists/t-final-monosyllabic-verbs.tsv` - working data, not yet read by the build; `láttam` 17, `látottam` -, `kötöttem` 6, `köttem` -, `mutattam` 7, `tanítottam` 7)
- **Confidence**: `consensus`; the T1/T2 past merger `measured`

### VRB-STEM-004 -ít verbs
- **Statement**: Verbs ending in `-ít` form an open class with fixed behaviour: heavy for linking vowels, past `-Ott` in all persons, imperative `-ts`.
- **Conditions**:
  - `S0` ends in `ít` and has two or more syllables, or is a derived monosyllabic-looking stem with truncation (`szít` is a root in T2, `ít` alone is not a verb)
  - harmony from the base before `-ít` (truncated vowel-final adjectives keep their harmony: `tisztít` B from `tiszta`, `simít` B from `sima`, `hűsít` R, `segít` F)
- **Output**: `tanít`: `tanítasz`, `tanítotok`, `tanítanak`, `tanítani`, `tanítana`, `tanítalak`, `tanított`, `tanítottam`, `taníts`, `tanítsd`, `tanítja`.
- **Examples**: `segít` -> `segítesz`, `segítettem`, `segíts`; `épít` -> `építenek`, `építettük`; `javít` -> `javítani`, `javíts`
- **Exceptions**: none known in the standard; colloquial `tanítsuk` for indicative `tanítjuk` is stigmatised (`VRB-IMP-012`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-STEM-003` (T4), `CAUS` (`-ít` is also a derivational suffix, `DER-VERB`).
- **Sources**: [T04 §4.1.3 p. 159], [KVF98 pp. 290, 295], [AkH12 §76]
- **Validation**: `goldens`, `attestation` (`tanítanak` 8, `tanítani` 12, `tanítottam` 7, `taníts` 5)
- **Confidence**: `consensus`

### VRB-STEM-005 -ad/-ed stems: suffixal vs root
- **Statement**: Polysyllabic verbs ending in `-ad/-ed` split into a class where `-Ad` is (historically) a suffix, which takes the past `-t` in all persons, and a root class taking `-Ott` in 3sg.
- **Conditions**:
  - suffixal class (intransitive, inchoative 'become X'): `akad`, `apad`, `dagad`, `fakad`, `fárad`, `halad`, `hasad`, `hervad`, `lankad`, `lohad`, `marad`, `pirkad`, `sorvad`, `szakad`, `szalad`, `szunnyad`, `tapad`, `virrad`, `fonnyad`, `ébred`, `dermed`, `ered`, `ernyed`, `mered`, `reked`, `reped`; open to productive `-Ad` formations
  - root or transitive class: `fogad`, `tagad`, `enged`, `szenved`, `feled`, `vigad`, `senyved`; monosyllables `ad`, `fed` are ordinary OBS (`VRB-PAST-005`)
  - fluctuating, often with meaning split: `támad` (`támadt` 'arose' ~ `támadott` 'attacked'), `ragad` (`ragadt` 'got stuck' ~ `ragadott` 'grabbed'), `téved` (`tévedt ~ tévedett`)
  - diagnostic tendency: intransitive change-of-state verb -> suffixal; transitive -> root
- **Output**: suffixal: `marad` -> `maradt`, `maradtam`, `maradtak`; root: `fogad` -> `fogadott`, `fogadtam`.
- **Examples**: `szalad` -> `szaladt`; `ébred` -> `ébredt`; `szenved` -> `szenvedett`; `enged` -> `engedett`
- **Exceptions**: past participle of suffixal class is `-t` too (`fáradt`, `szakadt`, `repedt`), often lexicalised adjectives.
- **Predictability**: `lexical` (suffixal class ~30 common members, open via `-Ad` derivation) with `tendency` (transitivity)
- **Lexical input**: flag `ad_suffixal`
- **Interactions**: `VRB-PAST-008`.
- **Sources**: [T04 fn. 19 p. 150], [SMNy3 p. 518 (mondottam-mondtam, ragadott-ragadt, támadott-támadt)], [morphdb grammar V_PAST_TENSE_FILTER -ad/-ed], [jepst hvcul.pl -AD/-ED list]
- **Validation**: `attestation` (`maradt` 19 vs `maradott` 3; `fáradt` 12 vs `fáradott` -; `ébredt` 9 vs `ébredett` -; `szenvedett` 15 vs `szenvedt` -; `fogadott` 13 vs `fogadt` -; `támadt` 12 ~ `támadott` 10; `ragadt` 9 ~ `ragadott` 8; `tévedett` 8 ~ `tévedt` 6)
- **Confidence**: `consensus` (lists), `contested` for fluctuating items

### VRB-STEM-006 Geminate stems (-ll, -rr, -gg, -dd, -zz)
- **Statement**: Verbs ending in a geminate form a small closed set whose linking vowel and past behaviour are lexical per item.
- **Conditions**: full lists (monosyllabic roots):
  - `ll`, no linking vowel, past `-t` (`-ott` archaic/adjectival): `áll`, `száll`
  - `ll`, linking vowel, past `-ott`: `hall`, `vall`, `kell`, `illik`, `ellik`, `züllik`
  - `ll`, both: `hull ~ hullik` (`hullanak ~ hullnak`, `hullott ~ hullt`), `mállik`, `pállik`; **`mállott` is the only past form of `mállik`** (`mállt` is rejected by the spelling checker [MTA-HELYES])
  - `rr`: `varr`, `forr`: no linking vowel (`varrni`, `forrnak`), past `varrt ~ varrott`, `forrt ~ forrott` (`-t` preferred)
  - `gg`: `függ`, `csügg`: linking vowel optional (`függnek ~ függenek`), past 3sg `függött`, non-3sg `-t-` (`függtek`)
  - `dd`: `fedd`: linking vowel (`feddeni`), past `feddett`, `feddtek`
  - `zz`: `izzik`: linking vowel (`izzanak`), past `izzott`, `izzottam`
  - the `-All` verbs split **lexically** in the past, with opposite values inside the same class: `javall` takes the `-Ott` past (`javallottam` 76 : `javalltam` 14, 84%; `javallott` 12,955 > `javallt` 3,421) while `sugall` takes bare `-t` (`sugalltam` 347 : `sugallottam` 2, 99%; `sugallt` 3,665 : `sugallott` 42) [WC2]. For the `-All` verbs the 3sg past is a participle homograph, so the 1sg past is the clean probe.
  - polysyllabic `-ll` verbs (`-All` derivatives `szégyell`, `sokall`, `kevesell`, `rosszall`, `nagyoll`, `kicsinyell`, `drágáll`, `furcsáll`, `zöldell`, `sárgáll`, `kékell`, `fehérl(ik)`, `torkoll(ik)`, `sarkall`, `nyilall`, `átall`; also `sugall`, `javall`, `restell ~ röstell`, `rühell`, `lövell`, `rivall`, `fuvall`, `hajnallik`, `pitymallik`, `szögellik`): linking vowel optional, no-vowel preferred (`szégyellni ~ szégyelleni`, `rosszallnám ~ rosszallanám`); past `-t ~ -ott/-ett` (`zöldellt ~ zöldellett`); some have syncopated doublets before vowel-initial suffixes (`kevesellem ~ keveslem`, `kicsinyelli ~ kicsinyli`, `kékellik ~ kéklik`)
- **Output**: see lists; `-lAk` after `ll`: triple `l` reduced (`hall+lak` -> `hallak`, `VRB-PRES-005`).
- **Examples**: `áll` -> `állsz`, `állnak`, `állni`, `állt`, `álltam`; `hall` -> `hallasz`, `hallanak`, `hallani`, `hallott`, `hallottam`; `varr` -> `varrsz`, `varrnak`, `varrt`; `javall` -> `javallott`; `sugall` -> `sugallt`
- **Exceptions**: `állasz` is a normatively accepted doublet of `állsz`, rare in writing (`állsz` 10, `állasz` 0) [AkH12 §44]; `állana` is marginal and not listed by AkH12; archaic or dialectal monosyllables `hallik`, `dall`, `köll` (for `kell`) are not generated by default.
- **Predictability**: `lexical` (closed: 11 `ll` monosyllables, 2 `rr`, 2 `gg`, 1 `dd`, 1 `zz`; polysyllabic `-ll` ~50 entries incl. prefixed in the project lexicon). Measured over 47 geminate-verb candidates: 9 take the vowel, 15 take none, 11 take both, 12 in review - item by item this reproduces the lists above.
- **Lexical input**: per-item flags `H` (heavy), past type, and `heavy_long_vowel_t` (true for `vét`, `bocsát`; false for `lát`, `VRB-LINK-003`); the geminate list is `geminate-verbs.tsv` in `local/lists/geminate-verbs.tsv` - read by `cargo xtask morphdb`
- **Interactions**: `VRB-LINK-002`, `VRB-PAST-004`, `VRB-IRR-019/020`, `ORT-LEN-023`, `ORT-LEN-024`, `DER-VERB-011`.
- **Sources**: [AkH12 §38, §44, §64 (`mállottak`), §65], [T04 §4.1.3 p. 159 (áll-ni vs hall-ani)], [KVF98 p. 295], [morphdb grammar ll-finals], [jepst hvcul.pl -LL list], [WC2], [MTA-HELYES], [ATT evidence/corpus-variants.md 2026-09-16], [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: `attestation`, with full-corpus figures where they exist [WC2]: `javallottam` 76 : `javalltam` 14; `sugalltam` 347 : `sugallottam` 2; `sugallt` 3,665 : `sugallott` 42; `javallott` 12,955 : `javallt` 3,421; `mállt` rejected [MTA-HELYES]. Earlier 250k-list figures (`sugallt` 4, `javallott` 5) are superseded. Also: `hallottam` 14, `halltam` -; `vallott` 11, `vallt` -; `állt` 19, `állott` 12; `hullott` 10, `hullt` 5; `hullanak` 6, `hullnak` 3; `varrt` 5, `varrott` 4; `függnek` 8, `függenek` 8; `függtek` 3, `függöttek` -; `szégyellni` 4, `szégyelleni` -; lexicon scan of `verb.ron` for monosyllabic geminate-final roots
- **Confidence**: `consensus`; `-rr` past variants `tendency`; `measured` and item-lexical for the `-All` past (`OQ-05-014` resolved)

### VRB-STEM-007 Epenthetic (unstable-vowel) stems without -ik
- **Statement**: A closed class of polysyllabic verbs of shape `...VC{o,e,ö}C` loses the last stem vowel before Class I suffixes and keeps it elsewhere.
- **Conditions**:
  - necessary shape: two or more syllables, final `V C1 O C2` with `O` a short mid vowel harmonising (o/e/ö; the only exception is `őriz` with `i`); `C2` almost always `g`, `z`, `l` (the verbalisers `-Og`, `-Oz`, `-Ol`), or one of the 7 roots with `r`/`m` listed by SMNy3 (`terem`, `tipor`, `seper`, `söpör`, `kotor`, `gyötör`, `sodor`; `seper ~ söpör` are variants of one lexeme), plus `pödör` (`pödri`, flagged epenthetic in the project lexicon)
  - syncope must not create a geminate (`dalol`, `gágog`, `zokog`, `mezez` are not epenthetic) and the preceding long vowel is usually `á`/`é` (apparent exceptions are derived: `kínoz`, `hímez`, `túloz`, `őröl`, `pótol`, `jósol`, and `őriz`)
  - shape is not sufficient: `ragyog`, `dalol`, `tanul`, `ugat` are not epenthetic; membership is lexical
  - vowel drops before: 1sg `-Ok`, 1pl `-Unk`, def `-Om`, `-Od`, front def `-i`, `-itek`, `-ik`, past 3sg `-Ott` (when `C2` is not a coronal sonorant, `VRB-PAST-009`), `-Ó`, `-Ás`, `-At` causative, `-AtlAn`
  - vowel stays before: `-sz`, `-tOk`, `-nAk`, `-ni`, `-nA`, `-lAk`, `-jA` (back), `-jük`, imperative `-j`, `-hAt`, `-vA`, past non-3sg `-t-`, past 3sg after `l`/`r`
  - syncope before Class I is obligatory with `C2` = `g`, `z`, `m`; with `C2` = `l`, `r` it is optional and item-specific (`énekelek ~ éneklek`, `ünnepelünk ~ ünneplünk`, `rablom ~ rabolom`), lexicalised derivatives always syncopated (`éneklő`, `rabló`, `törlés`, `képző`, `jegyzet`)
- **Output**: `forog` `SV` = `forg`, `SQ/SA` = `forog`; `érez` `SV` = `érz`.
- **Examples**: `forog` -> `forgok`, `forogsz`, `forog`, `forgunk`, `forogtok`, `forognak`, `forgott`, `forogtam`, `forogjon`, `forgó`; `érez` -> `érzek`, `érzel`, `érzi`, `érezzük`, `érzett`, `éreztem`, `érezni`, `érezzen`; `söpör` -> `söpri`, `söpörjük`, `söpört`
- **Exceptions**: metathesis stems are nominal only; `hangzik`, `lélegzik` are `-ik` (`VRB-STEM-008`).
- **Predictability**: `lexical` (closed class: ~150 roots [T04 fn. 1]; 513 non-prefixed verb entries flagged `epenthetic` in morphdb peeta incl. `-Ol/-Og/-Oz` derivatives); `rule` for which suffixes trigger syncope
- **Lexical input**: flag `epenthetic`, optional flag `syncope_optional` for `-l/-r` items
- **Interactions**: `VRB-MODEL-007`, `VRB-PAST-009`, `VRB-PRES-006`, `PH-ZERO-003`.
- **Sources**: [SMNy3 pp. 718-723 (13.3.3), p. 720 (root list), (82) no geminate], [T04 §4.1.1, §4.1.4.4 (47), (51)], [KVF98 p. 295], [morphdb grammar V_STEM_VARIANT_FACTORY epenthetic], [jepst hvcul.pl vowel-deleting list]
- **Validation**: `attestation` (`törli` 10 vs `törölik` -; `töröljük` 8 vs `törljük` -; `közli` 12; `pótolja` 9 vs `pótlja` -; `érzem` 16 vs `érezem` 0; `énekelek` 3 vs `éneklek` 0; `mozgok` 3; `mozgott` 10); `oracle:hunmorph-foma`
- **Confidence**: `consensus` (class), `tendency` (optional syncope of `-l/-r` items)

Frequent members (non-exhaustive, for goldens): `-Og`: `forog`, `mozog`, `inog`, `csorog`, `morog`, `dörög`, `zörög`, `csörög`, `pörög`, `hörög`, `sajog`, `mosolyog`, `didereg`, `kesereg`, `pezseg`, `rezeg`, `közeleg`, `tévelyeg`, `füstölög`, `hömpölyög`, `könyörög`, `társalog`, `csavarog`, `szivárog`, `párolog`, `ácsorog`, `bolyog`, `dübörög`, `vánszorog`, `tántorog`. `-Oz/-iz`: `érez`, `szerez`, `végez`, `céloz`, `kínoz`, `túloz`, `sebez`, `viszonoz`, `őriz`, `elemez`, `ellenez`, `címez`, `bélyegez`, `képez`, `jegyez`, `szoroz`. `-Ol/-El/-Öl`: `énekel`, `érdemel`, `ismétel`, `igényel`, `irigyel`, `ünnepel`, `térdepel`, `becsmérel`, `ingerel`, `érdekel`, `bérel`, `csépel`, `perel`, `közöl`, `töröl`, `őröl`, `üdvözöl`, `dögöl`, `pótol`, `rabol`, `javasol`, `pazarol`, `gyakorol`, `kóborol`, `vásárol`, `vándorol`, `birtokol`, `panaszol`, `varázsol`, `gyalogol`, `lovagol`. Roots: `terem`, `tipor`, `seper ~ söpör`, `kotor`, `gyötör`, `sodor`, `pödör`.

### VRB-STEM-008 Epenthetic -ik verbs (parallel stems)
- **Statement**: `-ik` verbs with an unstable vowel have a syncopated stem used before `-ik` and Class I suffixes and in the past, a full stem used before Class II.B suffixes, and both before Class II.A suffixes.
- **Conditions**:
  - members (closed): `ugrik` (`ugor-`), `romlik` (`romol-`), `bomlik` (`bomol-`), `omlik` (`omol-`), `ömlik` (`ömöl-`), `hajlik` (`hajol-`, distinct from the non-`-ik` verb `hajol`), `oszlik` (`oszol-`), `fürdik` (`füröd-`), `hangzik` (`hangoz-`), `vérzik` (`vérez-`), `hiányzik` (`hiányoz-`), `dohányzik` (`dohányoz-`), `lélegzik` (`lélegez-`), `burjánzik` (`burjánoz-`), `virágzik` (`virágoz-`), `zuhanyzik` (`zuhanyoz-`), `rajzik` (`rajoz-`); sz-d verbs are separate (`VRB-STEM-012`)
  - `SV` = syncopated: `ugr-om`, `ugr-unk`, `ugr-ó`, `ugr-ás`
  - past: 3sg syncopated + `-Ott` regardless of final consonant (`ugrott`, `romlott`, `fürdött`, `hiányzott`); non-3sg (quasi-analytic, both variants exist) split by `C2`: `r`/`l` members prefer syncopated + `-Ott-` (`ugrottam ~ ugortam`, `romlottak ~ romoltak`, `hajlottak`), `d`/`z` members prefer full stem + `-t-` (`fürödtem ~ fürdöttem`, `hiányoztam ~ hiányzottam`, `dohányoztam`, `virágoztak`)
  - `SQ`: syncopated + low linking vowel (preferred) ~ full stem without linking vowel: `ugrasz ~ ugorsz`, `ugrotok ~ ugortok`, `ugranak ~ ugornak`, `ugrani ~ ugorni`, `ugrana ~ ugorna`
  - `SA` = full: `ugorjon`, `ugorhat`, `ugorva`, `romoljon`, `fürödjön`, `hangozzék`
- **Output**: `ugrik` paradigm `VRB-PARA-001` (`ugrik`).
- **Examples**: `romlik` -> `romlanak ~ romolnak`, `romlott`, `romoljon`; `fürdik` -> `fürdeni` 25,400 ~ `fürödni` 13,578 (the only item in the class that genuinely varies; `ugrani` 32,345 : `ugorni` 1,040, `söpörni` 7,361 : `söpreni` 30 [WC2]), `fürdött`, `fürödjön`; `hiányzik` -> `hiányzott`, `hiányozni`, `hiányozzon`
- **Exceptions**: the `SQ` choice is **item-lexical, and mostly categorical** [WC2, `[Inf]` per lemma]: `ugrik` takes the syncopated infinitive categorically (`ugrani` 32,345 : `ugorni` 1,040, 97%), `söpör`, `vándorol`, `cselekszik` and `emlékezik` take the full stem categorically (`söpörni` 7,361 : `söpreni` 30; `vándorolni` 6,191 : `vándorlani` 13; `cselekedni` 51,155 : `cselekdni` 56; `emlékezni` 66,844 : `emlékezeni` 2), and `hangzik` takes the syncopated form at 84% (`hangzani` 3,000 : `hangozni` 564), its full-stem variant being essentially spoken-only. **Only `fürdik` is genuinely variable** (`fürdeni` 25,400 : `fürödni` 13,578, 65 : 35), with a weak register gradient - syncopated leading in speech and news, full stem in literary and technical prose, all buckets within 0.4-0.6. For `vérzik`, `hiányzik`, `dohányzik`, `lélegzik` the full-stem forms remain preferred.
- **Predictability**: `lexical` (closed, ~20 roots); choice of `SQ` variant `lexical` per item, measured, rather than a class-level `tendency`
- **Lexical input**: syncopated and full stem, `SQ` preference with the measured per-item share
- **Interactions**: `VRB-IK-*`, `VRB-PAST-009`, `VRB-LINK-010`.
- **Sources**: [SMNy3 pp. 729-740 (13.3.5; p. 729: word-final past always syncopated, II.A incl. non-final past fluctuates)], [T04 §4.1.4.4 (47) and fn. 74], [morphdb grammar epenthetic ik_basic], [jepst hvcul.pl ugrik, romlik, fürdik, hangzik], [WC2], [ATT evidence/corpus-variants.md 2026-09-16], [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: `attestation`, now with full-corpus per-item shares [WC2]: `ugrani` 32,345 : `ugorni` 1,040; `fürdeni` 25,400 : `fürödni` 13,578; `hangzani` 3,000 : `hangozni` 564; `söpörni` 7,361 : `söpreni` 30; `vándorolni` 6,191 : `vándorlani` 13; `cselekedni` 51,155 : `cselekdni` 56; `emlékezni` 66,844 : `emlékezeni` 2. Earlier 250k-list figures (`ugrani` 8, `hangzani` 1) are superseded. `oracle:hunmorph-foma`
- **Confidence**: `consensus` (paradigm shape); `measured` (variant preference, item-lexical; `OQ-05-006` resolved)

### VRB-STEM-009 Defective stems with ill-formed final clusters
- **Statement**: Verbs whose bound stem ends in a cluster that cannot close a word lack (or replace) the forms that would expose the bare stem before an analytic suffix.
- **Conditions**:
  - members: `-ik` verbs in consonant + `l`: `vedlik`, `bűzlik`, `sejlik`, `porlik`, `ízlik`, `áramlik`, `vonaglik`, `csuklik`, `siklik`, `hanyatlik`, `csillámlik`, `szüremlik`, `özönlik`; consonant + `z`: `habzik`, `porzik`, `bagzik`, `fajzik`, `csapzik`; non-`-ik`: `kétl-` (`kétli`), `sínyl-` (`sínyli`, also `sínylik`), `metsz` (not defective in practice, `metssz`, `metszhet` fine)
  - missing cells: imperative (`*vedlj`), `-hAt` (`*vedlhet`), converb `-vA`, 3sg def back `-jA`; 1pl def `-jük` (`*kétljük`); for `kétli`, `sínyli` also 3sg indefinite (`*kétl`)
  - speaker variants fill gaps with an epenthetic full stem: `?csukoljon`, `?sikoljon`, `?ízeljen`, `?foszolhat`; some never (`sejlik`, `porlik`)
  - forms with vowel-initial or II.A suffixes exist and take the low linking vowel: `vedlenek`, `bűzlenek`, `siklanak`, `kétlem`, `kétled`, `kétlik`, `kételkedik` (derivation)
  - past: `-Ott` in all persons (`vedlett`, `vedlettem`, `csuklottam`)
- **Output**: generator marks missing cells as defective (no form) unless variant flag given.
- **Examples**: `bűzlik` -> `bűzlök ~ bűzlöm`, `bűzlesz`, `bűzlött`, `bűzlene`, imperative none; `kétli` -> `kétlem`, `kétled`, `kétli`, `kétlitek`, `kétlik`, past cells are supplied only by the variant lemma `kétell` where speakers use it
- **Exceptions**: `játszik`, `látszik`, `tetszik`, `hallatszik`, `metsz` have ill-formed `tsz` codas but sibilant assimilation makes all forms available (`játsszon`, `játszhat`, `játszva`).
- **Predictability**: `lexical` (closed, ~30 roots) with `rule` (which cells are missing follows from suffix class)
- **Lexical input**: flag `defective`, optional variant stem
- **Interactions**: `VRB-MODEL-007`, `VRB-LINK-002` (always heavy), `PH-ZERO-004`, `PH-SYL-011`.
- **Sources**: [SMNy3 pp. 732-737 (98)-(104)], [T04 fn. 21 p. 151], [morphdb grammar V_QUASI_ANALYTIC_FILTER porlik, vonaglik, sínyli]
- **Validation**: `oracle:hunmorph-foma`, `attestation` (`bűzlik` 4, `ízlik` 6, `kétli` 0, `sínyli` 0)
- **Confidence**: `consensus` (defectiveness), `contested` for speaker-variant fillers

### VRB-STEM-010 v-stem verbs
- **Statement**: The only vowel-final verb roots are a closed set of monosyllables with a long vowel; they insert `v` before Class I and derivational vowel-initial suffixes, most with shortening of the vowel.
- **Conditions**: full list (standard): `lő`, `nő`, `sző`, `ró`, `fő`, `nyű`, `rí`; dialectal or only in derivatives: `fú` (`fúvó`, `fúvós`), `hí`, `szí`, `nyí`
  - shortening in `SV/SD`: `lő` -> `löv`, `nő` -> `növ`, `sző` -> `szöv`, `ró` -> `rov`, `fő` -> `föv ~ főv`, `nyű` -> `nyüv ~ nyűv`; no shortening: `rí` -> `rív`
  - `SQ`, `SA`, `SP`: the vowel stem: `lősz`, `lőtök`, `lőnek`, `lőni`, `lőne`, `lőlek`, `lője`, `lőjük`, `lőjön`, `lőhet`, `lőve`, past `lőtt`
  - definite: back `ró`: `rovom`, `rovod`, `rója`, `rójuk`, `rójátok`, `róják`; front: `lövöm`, `lövöd`, `lövi`, `lőjük`, `lövitek`, `lövik`
  - harmony: `lő nő sző fő nyű` R; `ró` B; `rí` B (antiharmonic: `rínak`, `ríttam`, `ríva`)
  - spelling: never doubled `j` or `sz` (`lője`, `lősz`), past always `tt` (`rótt`), short definite imperative `lődd`, `ródd`, `sződd`, `nyűdd`
- **Output**: see `VRB-IRR-018` for full paradigms.
- **Examples**: `lő` -> `lövök`, `lősz`, `lő`, `lövünk`, `lőtök`, `lőnek`, `lőtt`, `lőjön`, `lövő`, `lövés`; `ró` -> `rovok`, `rósz`, `rója`, `rótt`, `rovás`
- **Exceptions**: participle and noun forms with shortening but not in paradigm: `növendék`, `fövetlen`, `rovátka`; `nő` intransitive has no definite forms; `növekszik` is a separate sz-d verb.
- **Predictability**: `lexical` (closed: 7 standard verbs); shape rule: every vowel-final verb root is a v-stem
- **Lexical input**: v-stem table
- **Interactions**: `VRB-PAST-002`, `VRB-IMP-002`, `VRB-IRR-018`, `PH-VSTEM-002`, `ORT-LEN-025`.
- **Sources**: [SMNy3 pp. 514, 517, 755-765 (13.5)], [AkH12 §66], [KVF98 p. 291], [Wikt conj `fő`, `nyű`, `rí`], [morphdb peeta v_stem flags]
- **Validation**: `attestation` (`lövök` 2, `lősz` 1, `lövi` 6, `lőjük` 2, `lőtt` 13, `lövő` 7, `rója` 5, `rótt` 7, `szövi` 5, `szőtt` 8, `nőtt` 17)
- **Confidence**: `normative` (spelling AkH §66), `consensus`

### VRB-STEM-011 Vowel-stem suppletive verbs (tesz type)
- **Statement**: `tesz`, `vesz`, `visz`, `hisz`, `lesz`, `eszik`, `iszik` share a stem-form pattern: `sz` stem in the present, bare vowel stem in the past and before `-hAt`, `n`-geminating vowel stem in conditional and infinitive, `v` stem before `-Ó/-Ás`, `gy` stem in the imperative.
- **Conditions**: slot table:

| Verb | present `SV/SQ` | past `SP` | 3sg past indef | cond/inf | `-hAt` | `-Ó`, `-Ás` | imperative `SJ` | short def imp 2sg |
|---|---|---|---|---|---|---|---|---|
| `tesz` | `tesz` | `te` + `tt` | `tett` | `tenn-`, `tenni` | `tehet` | `tevő`, `tevés` | `tegy-` | `tedd` |
| `vesz` | `vesz` | `ve` + `tt` | `vett` | `venn-`, `venni` | `vehet` | `vevő`, `vevés` (rare; `vétel`) | `vegy-` | `vedd` |
| `visz` | `visz` | `vi` + `tt` | `vitt` | `vinn-`, `vinni` | `vihet` | `vivő`, (`vitel`) | `vigy-` | `vidd` |
| `hisz` | `hisz` | `hi` + `tt` | `hitt` | `hinn-`, `hinni` | `hihet` | `hívő ~ hivő`, (`hit`) | `higgy-` | `hidd` |
| `lesz` | `lesz` | `le` + `tt` | `lett` | `lenn-`, `lenni` | `lehet` | `levő ~ lévő` | `legy-` | none |
| `eszik` | `esz` | `e` + `tt` | `evett` | `enn-`, `enni` | `ehet` | `evő`, `evés` | `egy-` | `edd` |
| `iszik` | `isz` | `i` + `tt` | `ivott` | `inn-`, `inni` | `ihat` | `ivó`, `ivás` | `igy-` | `idd` |

  - harmony: all front unrounded except `iszik` B (antiharmonic)
  - 2sg indefinite `-Ol` (sibilant rule): `teszel`, `iszol`
  - definite: `j`-initial suffixes assimilate (`isszuk`, `tesszük`); front verbs only 1pl `-jük` shows it
  - `eszik`, `iszik` past 3sg indefinite uses the `v` stem + `-Ott` (`evett`, `ivott`); all other past cells `ett-`, `itt-`
  - imperative 2sg indef long form only for `tegyél`, `vegyél`, `vigyél`, `egyél`, `igyál`, `higgyél ~ higgy`, `legyél ~ légy`; archaic short `tégy`, `végy`, `vígy`, `égy`
- **Output**: full paradigms `VRB-IRR-002`, `VRB-IRR-005..010`.
- **Examples**: `vesz` -> `veszek`, `veszel`, `vesz`, `veszünk`, `vesztek`, `vesznek`, `vettem`, `vennék`, `vegyek`, `venni`, `vehet`, `vevő`
- **Exceptions**: `hisz` imperative has long `ggy` (`higgyen`) [AkH12 §81]; `lesz` has no definite forms; `eszik`/`iszik` are `-ik` verbs (`VRB-IK-002`).
- **Predictability**: `lexical` (closed, 7 verbs)
- **Lexical input**: slot table
- **Interactions**: `VRB-IRR-*`, `VRB-CAUS-002`, `PH-VSTEM-006`, `PH-ASSIM-008`.
- **Sources**: [SMNy3 p. 761], [AkH12 §42, §68, §81], [KVF98 pp. 289-290, 295, 310-311], [morphdb grammar irregular verbs tesz, vesz, eszik, iszik, hisz, visz, van]
- **Validation**: `attestation` (`evett` 10, `ettem` 10, `ivott` 10, `ittam` 8, `tegyél` 11, `tégy` 9, `higgy` 7, `higgyél` -), `goldens`
- **Confidence**: `normative` (spelling), `consensus`

### VRB-STEM-012 Polysyllabic sz-d(-v) and sz-z verbs
- **Statement**: A closed set of `-szik` verbs has a present `sz` stem and a `d` (or `z`) stem with an unstable vowel elsewhere; a subset also has a `v` stem before `-Ó` and `-Ás`.
- **Conditions**:
  - `sz` stem (`SV`, present `SQ`): `alsz-`, `feksz-`, `nyugsz-`, `haragsz-`, `esküsz-`, `alkusz-`, `cseleksz-`, `dicseksz-`, ...
  - `d` stem (`SP`, conditional, infinitive, imperative, `-hAt`, `-vA`, `-lAk`): stem vowel `o/e/ö/u/ü` + `d`: `alud-`, `feküd-`, `nyugod-`, `haragud-`, `esküd-`, `alkud-`, `cseleked-`, `dicseked-`
  - `z` stem: `igyekez-`, `emlékez-`, `gyülekez-`
  - `v` stem (`SD`), restricted to the **13 lemmas with an attested `v` participle**: `alv-`, `fekv-`, `nyugv-`, `haragv-`, `esküv-`, `cselekv-`, `dicsekv-`, `törekv-`, `igyekv-`, `gyanakv-`, `növekv-` and the two remaining members of the measured list (others use the `d/z` stem: `emlékező`, `öregedő`)
  - no finite `d/z` doublet (sz-d-v proper, 6): `alszik`, `fekszik`, `nyugszik`, `haragszik`, `esküszik`, `alkuszik`
  - with finite `-dik/-zik` doublet (sz-d, sz-z; doublet is a full regular verb): `cselekszik ~ cselekedik`, `dicsekszik ~ dicsekedik`, `törekszik ~ törekedik`, `igyekszik ~ igyekezik`, `emlékszik ~ emlékezik`, `gyülekszik ~ gyülekezik`, `növekszik ~ növekedik`, `gyarapszik ~ gyarapodik`, `gazdagszik ~ gazdagodik`, `melegszik ~ melegedik`, `öregszik ~ öregedik`, `betegszik ~ betegedik`, `telepszik ~ telepedik`, `ülepszik ~ ülepedik`, `mosakszik ~ mosakodik`, `verekszik ~ verekedik`, `veszekszik ~ veszekedik`, `vetekszik ~ vetekedik`, `gyanakszik ~ gyanakodik`, `furakszik ~ furakodik`, `tolakszik ~ tolakodik`, `dulakszik ~ dulakodik`, `marakszik ~ marakodik`, `tanakszik ~ tanakodik`, `elégszik ~ elégedik`, `menekszik ~ menekedik` (archaic)
  - `*alusszanak` is **not a form**: 0 hits against `alszanak` 26,072, `alusznak` 2,025 and `aludjanak` 2,351 `[ATT 2026-09-16]`. The imperative uses the `d` stem only, so a sibilant-assimilated imperative of this class does not exist; generation must not produce one.
  - present plural and 2pl/3pl: `sz` stem + linking vowel (the current form) ~ `sz` stem with inserted stem vowel (**archaic**, not merely less frequent): `alszanak` 33,580 : `%alusznak` 2,311 (6.4%), `fekszenek` 20,974 : `%feküsznek` 1,781 (7.8%), `haragszanak` 3,340 : `%haragusznak` 354 (9.6%, and `haragusznak` is rejected by the spelling checker), `nyugszanak` 12,611 : `%nyugosznak` 887 (6.6%) [WC2; MTA-HELYES]; `alszotok ~ alusztok`. The register gradient is steep and monotone - literary 34-62% > technical 23-30% > web 4-7% > spoken 0-1% > news 0% - and a form that is absent from news and speech while carrying a third of literary tokens is an archaism, not a low-frequency variant. For verbs with doublets, the doublet supplies the alternative (`dicsekszenek ~ dicsekednek`)
  - `esküszik`, `alkuszik`: `sz` preceded by vowel, no linking vowel: `esküsztök`, `esküsznek`, `alkusznak`
  - past 3sg indef: `aludt`, `feküdt` (`-t`, closed exception pair); all others `-Ott` (`nyugodott`, `haragudott`, `esküdött`, `alkudott`, `cselekedett`, `igyekezett`, `emlékezett`)
  - `-hAt`: `alhat ~ aludhat`, `fekhet ~ feküdhet`, `nyugodhat ~ nyughat`; others `d/z` stem (`haragudhat`, `emlékezhet`)
  - imperative: `aludjon`, `feküdjön`, `nyugodjon`, `haragudjon`, `esküdjön`, `cselekedjen`, `igyekezzen`, `emlékezzen`
  - converb: `alva ~ aludva`, `fekve ~ feküdve` (short `al-`, `fek-` + `-vA` preferred); others `d/z` stem (`nyugodva`, `haragudva`, `esküdve`)
- **Output**: full paradigms `VRB-IRR-011..017`.
- **Examples**: `alszik` -> `alszom`, `alszol`, `alszik`, `alszunk`, `alszotok ~ alusztok`, `alszanak ~ alusznak`, `aludtam`, `aludt`, `aludnék`, `aludjon`, `aludni`, `alhat`, `alvó`, `alva`
- **Exceptions**: `megismerszik` has no `d` stem (defective: `*megismeredhet`); `hazudik` is not in this class (regular d-final `-ik` verb). Lexicon gap recorded: `alszik fekszik nyugszik igyekszik gyanakszik gyarapszik telepszik` are absent from morphdb's `peeta` lexicon **and** from our own `verb.ron`; the 52-item list below is the reconciliation.
- **Predictability**: `lexical` (closed: **52 members measured** over 158 candidates, 13 of them with an attested `v` stem; traditionally 6 sz-d-v proper plus ~25 sz-d/sz-z with doublets)
- **Lexical input**: `sz`, `d/z`, `v` stems; doublet link; past-`t` flag for `alszik`, `fekszik`; from `sz-d-verbs.tsv` (52 members, 1 in review) in `local/lists/sz-d-verbs.tsv` - working data, not yet read by the build
- **Interactions**: `VRB-IK-*`, `VRB-PAST-010`, `VRB-IRR-011..017`, `PH-VSTEM-006`.
- **Sources**: [SMNy3 pp. 723-724 (85)-(86), p. 761, p. 764 (144) (`alhat ~ aludhat`, `fekhet ~ feküdhet`, `nyughat ~ nyugodhat`, `alva ~ aludva`, `fekve ~ feküdve`, `nyugodva`)], [KVF98 p. 295], [T04 fn. 19 p. 150], [AkH12 §44 (öregedik - öregszik, igyekezik - igyekszik)], [morphdb grammar sz-d_stem, sz-d-v_stem; peeta lexicon flags], [WC2], [MTA-HELYES], [hunmorph-foma], [ATT evidence/corpus-variants.md 2026-09-16], [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: `attestation`, with full-corpus `Prs.NDef.3Pl` figures per lemma [WC2] and a six-bucket register split; 158 candidates screened, oracle agreement on corpus-decided rows morphdb 121/138, our `.ron` 123/138, foma 133/133, hunspell 134/134. Earlier 250k-list figures (`alszanak` 8 vs `alusznak` 2) are superseded. Also `aludt` 10; `feküdt` 12; `haragudott` 5 vs `haragudt` -; `nyugodott` 6; `esküdött` 5; `alvó` 10; `alhat` 1 vs `aludhat` 0; `alva` 3 vs `aludva` -
- **Confidence**: `consensus`; the `-usz-` plural alternants are `measured` and carry the register label **archaic** (`OQ-05-011`, `OQ-05-013` resolved)

### VRB-STEM-013 Harmony-marked verb roots
- **Statement**: Verb roots containing only neutral vowels (`i í e é`) are front by default; a closed list of roots is back (antiharmonic), and derived `-ít` stems keep the harmony of their (possibly truncated) base.
- **Conditions**:
  - antiharmonic (back) roots, closed list: `bír`, `bíz(ik)`, `dívik`, `fing(ik)`, `hív`, `hízik`, `ing` (in `inog`), `ír`, `irt`, `iszik`, `ívik`, `izzik`, `nyí`, `nyílik`, `nyír`, `nyit`, `rí`, `ring`, `sír`, `siklik`, `szid`, `szít`, `szív`, `tilt`, `vív`, `virít`, `visít`, `sivít`, `sikít`, `sipít`, `rikít`, `indít`
  - back by truncated base: `tisztít` (`tiszta`), `simít` (`sima`), `ritkít` (`ritka`), `pirít` (`piros`), `vidít` (`vidám`), `cifrít` (`cifra`), `piszkít` (`piszok`), `hígít` (`híg`), `ifjít` (`ifjú`), `bénít` (`béna`), `némít` (`néma`), `csitít` (`csitt`), `izzít` (`izzik`)
  - front neutral roots (default, examples): `csíp`, `hint`, `hisz`, `illik`, `int`, `ízlik`, `visz`, `vesz`, `ér`, `kér`, `ítél`, `pisil`
  - mixed back + neutral stems are back (`bírál`, `csinál`, `ajándékoz`)
- **Output**: `hív` -> `hívok`, `hívnak`, `hívja`; `visz` -> `viszek`, `visznek`, `viszi`.
- **Examples**: `nyit` -> `nyitok`, `nyitottam`, `nyissa`; `szid` -> `szidja`, `szidott`; `sír` -> `sírnak`; `int` -> `intenek`
- **Exceptions**: none within verbs beyond the list; nominal antiharmony list is in `PH-HARM`.
- **Predictability**: `lexical` (closed, ~35 roots)
- **Lexical input**: harmony override `back`
- **Interactions**: `VRB-MODEL-008`, `PH-HARM`.
- **Sources**: [morphdb grammar V_BACK_FRONT_HARMONY_FILTER comment "exceptions: many: ír, vív, sír, irt, fing"; peeta lexicon `back` flags], [KVF98 p. 421], [Wikt conj `rí`]
- **Validation**: `oracle:hunmorph-foma`, `goldens`; lexicon scan of `verb.ron` `Back` flag on all-neutral verbs (2026-09-15)
- **Confidence**: `consensus`

### VRB-STEM-014 Classification procedure
- **Statement**: A verb's inflection class is determined by lexical overrides first and by shape tests second, in a fixed order.
- **Conditions**: decision order:
  1. suppletive / irregular table (`van`, `lesz`, `megy`, `jön`, tesz type `VRB-STEM-011`, sz-d-v and sz-d/sz-z `VRB-STEM-012`, `lát`, `kell`) -> lexical slots
  2. vowel-final -> v-stem (`VRB-STEM-010`)
  3. lexical flags: `epenthetic` (`VRB-STEM-007/008`), `defective` (`VRB-STEM-009`), `ad_suffixal` (`VRB-STEM-005`), geminate per-item (`VRB-STEM-006`), harmony override (`VRB-STEM-013`), `ik`
  4. shape: `t`-final subclass (`VRB-STEM-003`, closed lists T1/T2/T3 checked by string), `-ít`, CC, SIB, SON, OBS (`VRB-STEM-002`)
  5. heavy property `H` (`VRB-LINK-002`) and past type (`VRB-PAST-*`) computed from 3-4
  6. CC past non-3sg type: lexical flag if listed (`VRB-PAST-007`), else default by cluster
- **Output**: class record `{harmony, ik, slots, H, past_type, imp_type, sib, defective}`.
- **Examples**: `fest` -> {F, -, regular, H, past `Ott` all, imp `ss` drop t, sib no}; `forog` -> {B, -, epenthetic g, not H, past 3sg `forgott` other `forogt-`, imp `j`}; `lát` -> {B, lexical T2 exception}
- **Exceptions**: unknown new verbs (loans with `-Ol`, `-Oz`, `-izál`, `-ál`) are regular non-epenthetic; new `-ik` verbs are formed with `-zik` (`szörfözik`, `teniszezik`), new non-`-ik` verbs with `-Ol` (`bloggol ~ blogol`) [T04 §3.4.3].
- **Predictability**: `rule` over `lexical` inputs
- **Lexical input**: all flags above
- **Interactions**: consumed by every paradigm rule.
- **Sources**: [SMNy3 pp. 514-523, 740 (112)], [T04 §3.4.3, §4.1.4.4 (51)], [morphdb grammar V_* filter chain], [jepst hvcul.pl type detection]
- **Validation**: `goldens` (PARA), `oracle:hunmorph-foma` on the lexicon
- **Confidence**: `consensus`

## IK

### VRB-IK-001 3sg present indefinite -ik
- **Statement**: `-ik` verbs mark the present indicative indefinite 3sg with `-ik`; all other verbs have a zero 3sg.
- **Conditions**:
  - lexical property of the verb (`ik` flag); not predictable from meaning, only partly from shape
  - obligatory `-ik` when the bound stem ends in a cluster that cannot close a word (`vedlik`, `bűzlik`, `játszik`, `ugrik`) [T04 fn. 21]
  - productive verbalisers: `-zik` forms new `-ik` verbs from nouns (`szörfözik`, `teniszezik`), `-Odik/-kOdik` medio-passives, `-Ózik`
  - `-ik` is invariant (no harmony)
  - `-ik` is attached to `SV` (syncopated stem for epenthetic `-ik` verbs)
- **Output**: `dolgoz` -> `dolgozik`; `ugr` -> `ugrik`; `esz` -> `eszik`.
- **Examples**: `lakik`, `alszik`, `tetszik`, `történik`, `születik`, `érkezik`, `utazik`
- **Exceptions**: the `-ik` disappears in every other cell, including derived forms (`dolgozhat`, `dolgozó`); archaic `-hatik` (`különbözhetik`) is not generated by default.
- **Predictability**: `lexical` (open but lexically marked class; derivational sources `rule`)
- **Lexical input**: `ik` flag
- **Interactions**: `VRB-MODEL-004`, `VRB-IK-002..007`.
- **Sources**: [KVF98 pp. 289-290], [SMNy3 pp. 514, 729-730], [T04 fn. 21 p. 151], [morphdb grammar PRES_INDIC_INDEF_SG_3]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-IK-002 1sg present indefinite of -ik verbs: -m ~ -k
- **Statement**: `-ik` verbs take the definite-looking `-Om` in the 1sg present indefinite in the standard; the regular `-Ok` is the colloquial variant.
- **Conditions**:
  - `-ik` verb, present indicative indefinite 1sg
  - normative / formal written: `-Om` (`eszem`, `dolgozom`, `alszom`, `emlékszem`)
  - colloquial, widespread in speech and informal writing: `-Ok` (`eszek`, `dolgozok`, `alszok`)
  - `-Om` of an `-ik` verb is homographous with the definite 1sg of transitive `-ik` verbs (`eszem` indef and def)
- **Output**: default generation `-Om`; `-Ok` generated as variant with `Style=Coll`; analyser accepts both.
- **Examples**: `lakik` -> `lakom ~ lakok`; `iszik` -> `iszom ~ iszok`; `foglalkozik` -> `foglalkozom ~ foglalkozok`; `emlékszik` -> `emlékszem ~ emlékszek`
- **Exceptions**: verbs whose `-ik` membership is weak take `-Ok` as the normal form: `hazudik` -> `hazudok` (`hazudom` is only definite); `-ik`/zero doublets follow the zero variant (`hull` -> `hullok`); sz-d verbs with doublets: `dicsekszem ~ dicsekedem`.
- **Predictability**: `tendency` (corpus share of `-Om` among the pairs tested: `dolgozom` 12 vs `dolgozok` 5; `lakom` 8 vs 2; `iszom` 8 vs 3; `alszom` 8 vs 0; `emlékszem` 14 vs 0; `foglalkozom` 10 vs 2; `utazom` 6 vs 2; `érdeklődöm` 5 vs 3; buckets are logarithmic, so `-Om` dominates by roughly one to two orders of magnitude in written text)
- **Lexical input**: `ik` flag; `ik_weak` flag for `hazudik` type
- **Interactions**: `VRB-PRES-001`, `VRB-IK-004`.
- **Sources**: [KVF98 p. 290 (Kontra 1995)], [morphdb grammar PRES_INDIC_INDEF_SG_1 USAGE: informal], [jepst hvcul.pl hazudik]
- **Validation**: `attestation`
- **Confidence**: `normative` (`-Om`), `tendency` (usage)

### VRB-IK-003 Traditional ikes forms in other present cells
- **Statement**: The historical ikes conjugation had distinct 2sg `-Ol` and 3sg/2sg forms beyond the sibilant rule; these are archaic and generated only on request.
- **Conditions**:
  - 2sg `-Ol` after non-sibilant `-ik` stems in older literary texts: `lakol` (for `laksz`), `törődöl` (for `törődsz`)
  - modern standard: `-ik` verbs use the general 2sg rule (`laksz`, `dolgozol` by sibilant rule, `ugrasz`)
- **Output**: default modern forms; archaic flag `Style=Arch` for `lakol` type.
- **Examples**: `lakik` -> `laksz` (standard), `lakol` (archaic)
- **Exceptions**: sibilant-final `-ik` verbs have `-Ol` anyway (`eszel`, `iszol`, `alszol`, `játszol`).
- **Predictability**: `lexical` (archaic register)
- **Lexical input**: `ik` flag
- **Interactions**: `VRB-LINK-003`.
- **Sources**: [HUWIKI-IKES (traditional paradigm `lakom, lakol, lakik`; `-l` as ikes 2sg marker)], [KVF98 p. 289 (modern `szöksz`, `iszol`, `eszel`)]
- **Validation**: `oracle:hunmorph-foma` (archaic tags); `attestation` weak (`lakol` 0 is ambiguous with `lakol` 'atone', `törődöl` -)
- **Confidence**: `consensus`

### VRB-IK-004 Conditional of -ik verbs: -nám, -nék
- **Statement**: Traditionally `-ik` verbs have 1sg conditional indefinite `-nÁm` and 3sg `-nék`; today the general `-nék` / `-nA` forms are used almost exclusively, the traditional ones survive in formal literary style.
- **Conditions**:
  - `-ik` verb, conditional present indefinite
  - modern: 1sg `-nék`, 3sg `-nA` (`utaznék`, `utazna`)
  - traditional: 1sg `-nÁm`, 3sg `-nék` (`utaznám`, `utaznék`); traditional 3sg coincides with modern 1sg
- **Output**: default modern; traditional as `Style=Arch/Formal` variant.
- **Examples**: `eszik` -> `ennék` (1sg), `enne` (3sg); traditional `enném`, `ennék`; `esik` -> `esnék`, `esne` ~ traditional `esném`, `esnék`
- **Exceptions**: the traditional forms survive only in very formal literary style and some dialects [KVF98 pp. 309-310]; KVF98 (673) uses modern `esne`.
- **Predictability**: `tendency` (`ennék` 2 vs `enném` -; `dolgoznék` 2 vs `dolgoznám` -; `utaznék` 0 vs `utaznám` -)
- **Lexical input**: `ik` flag
- **Interactions**: `VRB-COND-001`.
- **Sources**: [KVF98 pp. 309-310], [SMNy3 p. 521]
- **Validation**: `attestation`
- **Confidence**: `consensus`

### VRB-IK-005 Imperative of -ik verbs: -jék, 1sg -jAm
- **Statement**: Traditional `-ik` imperative has 3sg `-jék` and 1sg `-jAm`; modern usage has `-jOn` and `-jAk`, the 3sg `-jék` remains common in formal and official style.
- **Conditions**:
  - `-ik` verb, imperative indefinite 3sg: `-jOn` (neutral) ~ `-jék` (formal, official, elevated)
  - 1sg: `-jAk` (modern) ~ `-jAm` (archaic, `hadd egyem`)
  - the `j` assimilates as in `VRB-IMP-002..006`
  - `-jék` in fixed formal formulae: passive `-tatik/-tetik` verbs, themselves `-ik` verbs (`adassék`, `áldassék`, `vétessék`); lexicalised `tessék` (`tetszik`); non-`-ik` `legyék` (`van`, archaic)
- **Output**: default `-jOn`; `-jék` as `Style=Formal` variant.
- **Examples**: `történik` -> `történjen ~ történjék`; `eszik` -> `egyen ~ egyék`; `iszik` -> `igyon ~ igyék`; `dolgozik` -> `dolgozzon ~ dolgozzék`
- **Exceptions**: `tetszik` -> `tessék` is a lexicalised interjection/politeness form (`tessék` 12 vs `tessen` 0).
- **Predictability**: `tendency` (`történjék` 7 vs `történjen` 11; `egyék` 4 vs `egyen` 11; `igyék` 2 vs `igyon` 9; `lakjék` 1 vs `lakjon` 3)
- **Lexical input**: `ik` flag
- **Interactions**: `VRB-IMP-010`.
- **Sources**: [morphdb grammar SUBJ-IMP_INDEF_SG_3 +ék USAGE: formal], [KVF98 p. 311]
- **Validation**: `attestation`
- **Confidence**: `tendency`

### VRB-IK-006 -ik verbs and definiteness
- **Statement**: Being an `-ik` verb does not block the definite conjugation; transitive `-ik` verbs have a full regular definite paradigm, intransitive ones (most `-ik` verbs) have none.
- **Conditions**:
  - transitive `-ik` verbs: `eszik`, `iszik`, `játszik`, `alszik` (`alussza az igazak álmát`, fixed), `hazudik` (`hazudja`); prefixed transitive derivatives are usually non-`-ik` (`feldolgoz`)
  - definite forms are identical to those of non-`-ik` verbs (`eszem`, `eszed`, `eszi`, `esszük`, `eszitek`, `eszik`)
  - 3pl definite `eszik` is homographous with 3sg indefinite `eszik`
- **Output**: definite cells generated iff trans.
- **Examples**: `iszik` -> `iszom`, `iszod`, `issza`, `isszuk`, `isszátok`, `isszák`, `iszlak`; `játszik` -> `játssza`, `játsszuk`
- **Exceptions**: medio-passive `-Ódik`/`-kOdik` verbs are intransitive.
- **Predictability**: `rule` (given transitivity)
- **Lexical input**: `trans`
- **Interactions**: `VRB-PRES-002`, `VRB-PRES-007`.
- **Sources**: [KVF98 p. 290], [SMNy3 p. 516]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-IK-007 -ik / zero doublets
- **Statement**: A set of verbs is used both with and without `-ik`; both lemmas are variants of one lexeme and share all non-3sg forms.
- **Conditions**:
  - list (common): `hull ~ hullik`, `zöldell ~ zöldellik`, `sárgáll ~ sárgállik`, `kékell ~ kékellik ~ kéklik`, `feketéll ~ feketéllik`, `fehérl ~ fehérlik`, `torkoll ~ torkollik`, `máll ~ mállik`, `hazudik ~ hazud` (dialectal)
  - meaning-split pairs are separate lemmas (`bíz` 'entrust' / `bízik` 'trust', `hajol` / `hajlik`, `vérez` / `vérzik`, `ötöl` / `ötlik`)
- **Output**: lemma variants recorded; generation of 3sg gives preferred variant first.
- **Examples**: `hull ~ hullik` -> 3sg `hull ~ hullik`, 3pl `hullnak ~ hullanak` (same for both)
- **Exceptions**: `bíz` and `bízik` share present plural forms (`bíznak`) and are disambiguated by syntax.
- **Predictability**: `lexical` (closed, ~20 pairs)
- **Lexical input**: variant list
- **Interactions**: `VRB-MODEL-004`, `VRB-STEM-006`.
- **Sources**: [AkH12 §64-65], [SMNy3 p. 730 (hajol/hajlik, vérez/vérzik, ötöl/ötlik)], [morphdb grammar V_IK_BASIC_FILTER ik_basic/zero_basic]
- **Validation**: `attestation` (`hull` 9, `hullik` 8)
- **Confidence**: `consensus`

## LINK

### VRB-LINK-001 Linking vowel qualities and suffix inventory
- **Statement**: Verbal suffixes use two linking-vowel qualities: mid `O` (three-way) in the 2pl and past, low `A` (two-way) in the self-lowering suffixes; Class I suffixes begin with their own vowel.
- **Conditions**: inventory:

| Suffix | without LV | with LV | LV quality |
|---|---|---|---|
| 2sg indef | `-sz` | `-Asz` | A |
| 2sg indef after sibilant | - | `-Ol` | O (own vowel) |
| 2pl indef | `-tOk` | `-OtOk` | O |
| 3pl indef | `-nAk` | `-AnAk` | A |
| infinitive | `-ni` | `-Ani` | A |
| conditional | `-nA-`, `-nÁ-`, `-nék` | `-AnA-`, `-AnÁ-`, `-Anék` | A |
| 1sg>2 | `-lAk` | `-AlAk` | A |
| past | `-t-` | `-Ott(-)` | O |
| 1sg, 1pl, def 1sg/2sg | - | `-Ok/-Om`, `-Unk`, `-Om`, `-Od` | own vowel |

  - `A` after front rounded stems is `e` (`küldenek`, `fűtenek`, `küldelek`), `O` is `ö` (`küldötök`, `fűtötök`, `küldött`)
- **Output**: selection rules `VRB-LINK-002..010`.
- **Examples**: `tart` -> `tartasz`, `tartotok`, `tartanak`, `tartani`, `tartana`, `tartalak`, `tartott`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-MODEL-007`, `VRB-MODEL-008`.
- **Sources**: [T04 §4.1.3 (18) pp. 158-159], [SMNy3 pp. 515-516, 519-521], [KVF98 p. 290]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-LINK-002 Heavy stems (H)
- **Statement**: The self-lowering suffixes `-sz`, `-tOk`, `-nAk`, `-ni`, `-nA`, `-lAk` take a linking vowel exactly when the stem is heavy (`H`); heaviness follows from shape with closed-list exceptions.
- **Conditions**: `H` is true iff one of:
  - (a) `S0` ends in two different consonants (digraph = one): `-rt`, `-lt`, `-nt`, `-jt`, `-mt`, `-st`, `-szt`, `-nd`, `-ld`, `-rd`, `-zd`, `-ng`, `-nz`, `-mz`, `-tsz`, `-ksz`, `-psz`, `-gsz`, `-lsz`, `-Cl` (defective), `-Cz` (defective), `-gr`, `-ml` (epenthetic `-ik` syncopated stems)
  - (b) `S0` ends in `-ít` (T4), or is a monosyllable ending in vowel + `dz`, which is pronounced long (`edz`, `pedz`: `edzeni`, `pedzeni`)
  - (c) `S0` is T2 other than `lát`, `lót` (`tát`, `vét`, `fűt`, `hűt`, `műt`, `szít`) or T3 (`bocsát`)
  - (d) listed geminates: `hall`, `vall`, `kell`, `illik`, `ellik`, `züllik`, `fedd`, `izzik`; optional: `hull`, `mállik`, `pállik`, `függ`, `csügg`, polysyllabic `-All` (`VRB-STEM-006`)
  - not heavy: `áll`, `száll`, `varr`, `forr`; every single consonant; `lát`; polysyllabic `-ódzik/-ődzik` (`csókolódznak`)
  - sz-d-v present stems: `alsz-`, `feksz-`, `nyugsz-`, `haragsz-` heavy (`alszanak`), `esküsz-`, `alkusz-` not heavy (V + sz)
  - irregular verbs specify `SQ` directly (`VRB-IRR-*`)
- **Output**: boolean `H` (with `optional` value for the variation set, `VRB-LINK-010`).
- **Examples**: `fest` H -> `festeni`; `ad` not H -> `adni`; `tanít` H -> `tanítani`; `lát` not H -> `látni`; `fűt` H -> `fűteni`; `hall` H -> `hallani`; `áll` not H -> `állni`; `varr` not H -> `varrni`; `edz` H -> `edzeni`
- **Exceptions**: the 2sg of sibilant stems uses `-Ol` regardless of `H` (`VRB-LINK-003`).
- **Predictability**: `rule` (a-c) + `lexical` (d: closed, 9 obligatory + ~25 optional items)
- **Lexical input**: geminate flags
- **Interactions**: `VRB-LINK-003..010`, `VRB-STEM-003`, `VRB-STEM-006`.
- **Sources**: [T04 §4.1.3 (18) pp. 158-159], [KVF98 p. 290], [SMNy3 p. 734 (102)], [morphdb grammar V_QUASI_ANALYTIC_FILTER], [magyarispell ige_alanyi.aff eloG, eloL, eloD, eloTJT2, eloZ contexts]
- **Validation**: `attestation` (`festeni` 9; `tanítani` 12; `fűteni` 6; `látni` 17; `hallani` 14; `állni` 15; `varrni` 6, `varrani` -; `edzeni` 9; `mondani` 17; `küldeni` 13)
- **Confidence**: `consensus`

### VRB-LINK-003 2sg present indefinite: -sz, -Asz, -Ol
- **Statement**: The 2sg indefinite is `-Ol` after sibilant-final stems, `-Asz` after other heavy stems, and `-sz` elsewhere.
- **Conditions**: ordered:
  1. `S0` (or `SQ`) ends in `s`, `sz`, `z`, `dz` (including `-tsz`, `-ksz`, `-ssz`, `-zz`) -> `-Ol` on `SV` (`olvasol`, `hozol`, `nézel`, `főzöl`, `játszol`, `metszel`, `alszol`, `edzel`, `izzol`)
  2. `H` -> `-Asz` (`tartasz`, `festesz`, `küldesz`, `tanítasz`, `fűtesz`, `hallasz`, `ugrasz`)
  3. else -> `-sz` (`vársz`, `kérsz`, `adsz`, `látsz`, `futsz`, `lősz`, `forogsz`, `állsz`)
- **Output**: see conditions.
- **Examples**: `mos` -> `mosol`; `vesz` -> `veszel`; `űz` -> `űzöl`; `hív` -> `hívsz`; `fon` -> `fonsz`; `mond` -> `mondasz` 39,605, and `?mondsz` is attested at 1.0% of the slot (407) and should be flagged, not rejected [WC2]; `edz` -> `edzel` 6,942 : `?edzesz` 11 (`dz` sibilant, rule 1 confirmed).
- **Exceptions**: suppletive: `megy` -> `mész` (archaic `mégy`), `jön` -> `jössz`, `van` -> `vagy`, `lesz` -> `leszel`; `áll`, `száll` -> `állsz` (`állasz` accepted doublet, rare [AkH12 §44]); `-sz` after `t`/`d` is written unchanged though pronounced `[t͡s(ː)]` (`adsz`, `látsz`) [AkH12 §58]; esküszik -> `esküszöl`. **Long vowel + `t` stems are heavy lexically, not as a class**, which is how the KVF98 p. 290 / SMNy3 p. 735 + T04 conflict resolves - both sources are partly right: `vét` (`vétenek` 2,424 : `vétnek` 20; `vétettem` 4,120 : `véttem` 0) and `bocsát` (`bocsátanak` 21,461 : `bocsátnak` 50) are heavy, `lát` is categorically light (`látnak` 139,116 : `látanak` 4), and `tát` is below the floor (n = 2) [WC2]. Side finding: `bocsátott` 32,745 : `bocsájtott` 1,923, so the `j`-inserting spelling is a 5.5% minority.
- **Predictability**: `rule`
- **Lexical input**: `H`, plus the lexical flag `heavy_long_vowel_t` (true for `vét`, `bocsát`; false for `lát`)
- **Interactions**: `VRB-LINK-002`, `VRB-IK-003`, `VRB-STEM-006`.
- **Sources**: [SMNy3 p. 515, p. 735], [KVF98 pp. 289-290], [AkH12 §44, §58], [morphdb grammar PRES_INDIC_INDEF_SG_2], [magyarispell ige_alanyi.aff E/2], [T04], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: `attestation` (`mondasz` 39,605 : `mondsz` 407; `edzel` 6,942 : `edzesz` 11; `vétenek` 2,424 : `vétnek` 20; `bocsátanak` 21,461 : `bocsátnak` 50; `látnak` 139,116 : `látanak` 4; `tartasz` 8; `állsz` 10, `állasz` 0; `hallasz` 6)
- **Confidence**: `consensus`; `measured` for `edz` and for the heaviness of long vowel + `t` stems (`OQ-05-005`, `OQ-05-015` resolved), which supersedes the earlier "1 vs 0" support with n = 2,444 and n = 21,511

### VRB-LINK-004 2pl present indefinite: -tOk, -OtOk
- **Statement**: The 2pl indefinite is `-OtOk` after heavy stems and `-tOk` elsewhere, with three-way `O`.
- **Conditions**:
  - `H` -> `-OtOk` (`tartotok`, `festetek`, `küldötök`, `tanítotok`, `fűtötök`, `hallotok`, `ugrotok`)
  - else -> `-tOk` (`vártok`, `kértek`, `ültök`, `olvastok`, `köttök`, `láttok`, `lőtök`)
- **Output**: see conditions.
- **Examples**: `ad` -> `adtok`; `ír` -> `írtok`; `üt` -> `üttök`; `alszik` -> `alszotok ~ alusztok`
- **Exceptions**: `megy` -> `mentek`; `jön` -> `jöttök`; `van` -> `vagytok`; `eszik` -> `esztek`; `iszik` -> `isztok`; `tesz` -> `tesztek`. Homographs with the past 3pl: `kértek`, `vettek`, `szerettek`, `mentek` (`VRB-PRES-007`).
- **Predictability**: `rule`
- **Lexical input**: `H`
- **Interactions**: `VRB-LINK-002`.
- **Sources**: [KVF98 p. 290 (tanítotok, segítetek, fűtötök)], [SMNy3 p. 515], [Wikt conj `üt` (üttök), `tát` (tátotok)]
- **Validation**: `attestation` (`tartotok` 3, `tartatok` -; `láttok` 5)
- **Confidence**: `consensus`

### VRB-LINK-005 3pl present indefinite: -nAk, -AnAk
- **Statement**: The 3pl indefinite is `-AnAk` after heavy stems and `-nAk` elsewhere.
- **Conditions**:
  - `H` -> `-AnAk` (`tartanak`, `festenek`, `küldenek`, `tanítanak`, `fűtenek`, `hallanak`)
  - optional `H` -> both (`függnek ~ függenek`, `hullnak ~ hullanak`)
  - else -> `-nAk` (`várnak`, `kérnek`, `ülnek`, `látnak`, `lőnek`, `állnak`, `varrnak`)
- **Output**: see conditions.
- **Examples**: `mond` -> `mondanak`; `ért` -> `értenek`; `ad` -> `adnak`; `fut` -> `futnak`
- **Exceptions**: `megy` -> `mennek`; `jön` -> `jönnek`; `van` -> `vannak`; tesz type -> `tesznek`, `isznak`.
- **Predictability**: `rule`
- **Lexical input**: `H`
- **Interactions**: `VRB-LINK-002`, `VRB-LINK-010`.
- **Sources**: [SMNy3 p. 515], [T04 §4.1.3 (18)], [KVF98 p. 290]
- **Validation**: `attestation` (`mondanak` 12, `mondnak` -; `tartanak` 16; `festenek` 6; `küldenek` 9; `bocsátanak` 8; `állnak` 18; `hallanak` 6, `hallnak` -)
- **Confidence**: `consensus`

### VRB-LINK-006 Infinitive: -ni, -Ani
- **Statement**: The infinitive is `-Ani` after heavy stems and `-ni` elsewhere, attached to `SQ`.
- **Conditions**:
  - `H` -> `-Ani` (`tartani`, `festeni`, `küldeni`, `tanítani`, `fűteni`, `hallani`, `bocsátani`, `ugrani`)
  - else -> `-ni` (`várni`, `kérni`, `ülni`, `látni`, `futni`, `lőni`, `forogni`, `állni`)
  - `-ni` itself is invariant; only the linking vowel harmonises
- **Output**: see conditions.
- **Examples**: `mond` -> `mondani`; `ért` -> `érteni`; `önt` -> `önteni`; `szít` -> `szítani`; `alkot` -> `alkotni`
- **Exceptions**: irregular infinitives: `lenni`, `tenni`, `venni`, `vinni`, `hinni`, `enni`, `inni`, `menni`, `jönni`, `aludni`, `feküdni`, `nyugodni`, `haragudni`, `esküdni` (`VRB-INF-001`); `szégyellni ~ szégyelleni` [AkH12 §44].
- **Predictability**: `rule`
- **Lexical input**: `H`
- **Interactions**: `VRB-INF-001`, `VRB-INF-002`.
- **Sources**: [T04 §4.1.3 (18)], [KVF98 p. 290], [morphdb grammar INF]
- **Validation**: `attestation` (PARA infinitives all attested except rare verbs)
- **Confidence**: `consensus`

### VRB-LINK-007 Conditional: -nA-, -AnA-
- **Statement**: The conditional marker takes a low linking vowel after heavy stems, identically to the infinitive.
- **Conditions**:
  - `H` -> `-AnA-`/`-AnÁ-`/`-Anék` (`tartana`, `festene`, `küldene`, `tanítana`, `fűtene`, `hallana`)
  - else -> `-nA-` (`várna`, `látna`, `lőne`, `állna`, `forogna`)
  - the same `H` value as for the infinitive; an item with `-ni`/`-Ani` variation has the same variation here (`függne ~ függene`, `állna ~ ?állana` marginal, `rosszallnám ~ rosszallanám` [AkH12 §44])
- **Output**: see conditions.
- **Examples**: `mond` -> `mondanék`, `mondanád`; `kezd` -> `kezdene`; `bocsát` -> `bocsátana`
- **Exceptions**: suppletive conditional stems geminate `n` (`venne`, `tenne`, `vinne`, `hinne`, `lenne`, `enne`, `inna`, `menne`, `jönne`); `alszik` -> `aludna`; `kell` -> `kellene ~ kéne` (`VRB-COND-005`).
- **Predictability**: `rule`
- **Lexical input**: `H`
- **Interactions**: `VRB-COND-001..003`.
- **Sources**: [SMNy3 pp. 520-521], [morphdb grammar PRES_COND_* quasi_analytic_V_nonindic], [jepst hvcul.pl IND_COND_*2]
- **Validation**: `attestation` (`mondana` 6, `mondna` -; `tartana` 7, `tartna` -; `küldene` 4; `állna` 10, `állana` 2)
- **Confidence**: `consensus`

### VRB-LINK-008 1sg subject 2nd person object: -lAk, -AlAk
- **Statement**: The `-lAk` suffix takes a low linking vowel after heavy stems; after cluster stems the vowel is near-obligatory (`küldelek` 1,871 : `küldlek` 68, `mondalak` 740 : `mondlak` 26 [WC2]).
- **Conditions**:
  - `H` -> `-AlAk` (`tartalak`, `festelek`, `tanítalak`, `fűtelek`, `küldelek`, `értelek`)
  - after `ll`: `hall+lak` -> `hallak` (three identical letters reduced, `VRB-PRES-005`); `áll` -> `állak`
  - else -> `-lAk` (`várlak`, `kérlek`, `látlak`, `ütlek`, `lőlek`, `kereslek`)
  - cluster stems with `-lAk` allow the vowel-less form more easily than `-nAk`: `mondalak ~ mondlak`, `küldelek ~ küldlek`, `hordalak ~ hordlak` (colloquial); `-t`-final clusters keep the vowel (`tartalak`; `?tartlak` 91 : 5,730 [WC2], above the 1% flag line of `VRB-LINK-003`)
- **Output**: preferred `-AlAk`; vowel-less variant for `-nd`, `-ld`, `-rd`, `-zd` stems as `Style=Coll`.
- **Examples**: `szeret` -> `szeretlek`; `ismer` -> `ismerlek`; `hív` -> `hívlak`; `vesz` -> `veszlek`; `visz` -> `viszlek`
- **Exceptions**: past and conditional `-lAk` always follow a vowel: `vártalak`, `várnálak`, `tanítanálak`; imperative `-jAlAk` (`várjalak`).
- **Predictability**: `rule`; the vowel-less variant is a 3.4% minority, not a free alternant (SMNy3 marks `%mondlak`, `%hordlak`)
- **Lexical input**: `H`
- **Interactions**: `VRB-PRES-005`.
- **Sources**: [SMNy3 p. 735, p. 738 (ajzlak-ajznak, küldlek-?*küldnek, mondlak-*mondnak)], [T04 §4.1.3 (18) (mond-(a)lak, sért-(e)lek, küld-(e)lek)], [morphdb grammar PRES_INDIC_SG_1_OBJ_2], [WC2], [Racz-DS `Racz2026mondasz`], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: `attestation` (`küldelek` 1,871 : `küldlek` 68, 96.5%; `mondalak` 740 : `mondlak` 26, 96.6%; `tartalak` 3, `tartlak` -; `értelek` 3); cross-checked against Rácz (2026), which draws the same slots from the same list
- **Confidence**: `measured` (linking vowel near-categorical); `mondlak` is a real 3.4% minority rather than a contested judgement (`OQ-05-004` resolved for this slot)

### VRB-LINK-009 Vowel-initial person suffixes
- **Statement**: The present 1sg `-Ok/-Om`, 1pl `-Unk`, definite 1sg `-Om`, 2sg `-Od` and front definite `-i/-itek/-ik` always begin with a vowel, regardless of stem shape; they attach to `SV`.
- **Conditions**:
  - after v-stems the `v` stem is used (`lövök`, `rovom`); after epenthetic stems the syncopated stem (`forgok`, `érzem`)
  - `U` of `-Unk` is u/ü; `O` three-way
- **Output**: `vár` -> `várok`, `várunk`, `várom`, `várod`; `kér` -> `kérek`, `kérem`, `kéri`, `kéritek`.
- **Examples**: `tör` -> `török`, `töröm`, `töri`; `lő` -> `lövök`, `lövünk`, `lövöm`, `lövi`
- **Exceptions**: `megy` -> `megyek` (front unrounded despite `gy` stem: regular), `jön` -> `jövök`, `van` -> `vagyok`, `vagyunk`.
- **Predictability**: `rule`
- **Lexical input**: `SV`
- **Interactions**: `VRB-MODEL-006`.
- **Sources**: [T04 §4.1.2.2 Type A pp. 146-147], [SMNy3 p. 515]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-LINK-010 Variation zone
- **Statement**: Linking-vowel presence varies for a defined set of stems; the generator emits both forms with a preference order.
- **Conditions**:
  - `-gg`, `-ll` optional items (`VRB-STEM-006`): both, no preference established (`függnek` 8 = `függenek` 8)
  - polysyllabic `-All`: no vowel preferred (`szégyellni` 4, `szégyelleni` -)
  - epenthetic `-ik` verbs: syncopated + vowel ~ full stem, item preference (`VRB-STEM-008`)
  - `mond` 2sg: `mondasz` 39,605 : `mondsz` 407 (99.0%), so the linking vowel is near-categorical and `mondsz` is a flagged minority rather than a marginal form. `mond` 2pl is the one genuinely variable slot and is **register-conditioned**: `mondotok` 1,909 : `mondtok` 2,410 overall (44 : 56), with `mondtok` at 73% in speech and 83% in wiki, `mondotok` at 79% in literary and 85% in technical prose, and parity on the web [WC2; ATT evidence/corpus-variants.md 2026-09-16]. 3Pl is likewise near-categorical (`mondanak` 113,365 : `mondnak` 412)
  - sz-d-v present plural: `alszanak` preferred ~ `alusznak` (`VRB-STEM-012`)
  - `áll`, `száll`: `állsz ~ állasz` accepted doublet [AkH12 §44], `állna ~ ?állana` marginal (not in AkH12)
- **Output**: ordered variant list per cell.
- **Examples**: `hull` -> `hullanak ~ hullnak`; `ugrik` -> `ugrani ~ ugorni`; `mond` -> `mondotok ~ mondtok` (44 : 56 overall; `mondtok` 73% in speech, `mondotok` 79% in literary prose)
- **Exceptions**: -
- **Predictability**: `tendency` (numbers in the referenced rules); the variation zone is much narrower than it looked - of the slots probed, only `mond` 2Pl, `fürdik` and the `-gg`/`-ll` items genuinely vary
- **Lexical input**: per-item variant flags
- **Interactions**: `VRB-LINK-002..008`.
- **Sources**: [AkH12 §44], [T04 §4.1.3 (18) and fn. 74], [SMNy3 p. 735 (102iii)], [WC2], [Racz-DS `Racz2026mondasz`], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: `attestation` (slot-tag-filtered counts per lemma, plus a five-bucket register split for the 2Pl slot)
- **Confidence**: `tendency`; `measured` for the `mond` slots (`OQ-05-004` resolved: 2Sg, 3Pl and 1Sg>2 near-categorical with the linking vowel, 2Pl register-conditioned)

## PRES

### VRB-PRES-001 Present indicative indefinite endings
- **Statement**: The present indicative indefinite has no tense marker; person/number suffixes attach to the stem slots listed below.
- **Conditions**:

| Cell | Suffix | Slot | Back `vár` | Front `kér` | Rounded `tör` |
|---|---|---|---|---|---|
| 1sg | `-Ok` (`-Om` for `-ik`, `VRB-IK-002`) | `SV` | `várok` | `kérek` | `török` |
| 2sg | `-sz` / `-Asz` / `-Ol` (`VRB-LINK-003`) | `SQ` | `vársz` | `kérsz` | `törsz` |
| 3sg | zero / `-ik` | `S0` / `SV` | `vár` | `kér` | `tör` |
| 1pl | `-Unk` | `SV` | `várunk` | `kérünk` | `törünk` |
| 2pl | `-tOk` / `-OtOk` (`VRB-LINK-004`) | `SQ` | `vártok` | `kértek` | `törtök` |
| 3pl | `-nAk` / `-AnAk` (`VRB-LINK-005`) | `SQ` | `várnak` | `kérnek` | `törnek` |

- **Output**: table.
- **Examples**: `olvas` -> `olvasok`, `olvasol`, `olvas`, `olvasunk`, `olvastok`, `olvasnak`; `fest` -> `festek`, `festesz`, `fest`, `festünk`, `festetek`, `festenek`; `dolgozik` -> `dolgozom`, `dolgozol`, `dolgozik`, `dolgozunk`, `dolgoztok`, `dolgoznak`
- **Exceptions**: suppletive present stems (`VRB-IRR-001..004`).
- **Predictability**: `rule`
- **Lexical input**: `ik`, `H`, slots
- **Interactions**: `VRB-LINK-*`, `VRB-IK-002`.
- **Sources**: [KVF98 p. 289], [SMNy3 pp. 514-515], [morphdb grammar PRES_INDIC_INDEF_*]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-PRES-002 Present indicative definite endings
- **Statement**: The definite present uses `-Om`, `-Od` in the singular 1-2, and a harmony-split set in 3sg and plural: back stems `-jA`, `-jUk`, `-jÁtOk`, `-jÁk`; front stems `-i`, `-jük`, `-itek`, `-ik`.
- **Conditions**:

| Cell | Back `vár` | Front `kér` | Rounded `tör` | Slot |
|---|---|---|---|---|
| 1sg | `várom` | `kérem` | `töröm` | `SV` |
| 2sg | `várod` | `kéred` | `töröd` | `SV` |
| 3sg | `várja` | `kéri` | `töri` | back `SA`, front `SV` |
| 1pl | `várjuk` | `kérjük` | `törjük` | `SA` |
| 2pl | `várjátok` | `kéritek` | `töritek` | back `SA`, front `SV` |
| 3pl | `várják` | `kérik` | `törik` | back `SA`, front `SV` |
| 1sg>2 | `várlak` | `kérlek` | `törlek` | `SQ` (`VRB-LINK-008`) |

  - no linking vowel in any definite cell
  - `j`-initial suffixes undergo `VRB-PRES-003` / `VRB-PRES-004`
- **Output**: table.
- **Examples**: `olvas` -> `olvasom`, `olvasod`, `olvassa`, `olvassuk`, `olvassátok`, `olvassák`, `olvaslak`; `néz` -> `nézem`, `nézed`, `nézi`, `nézzük`, `nézitek`, `nézik`, `nézlek`; `köt` -> `kötöm`, `kötöd`, `köti`, `kötjük`, `kötitek`, `kötik`, `kötlek`
- **Exceptions**: back epenthetic stems use the full stem before `-jA` (`sodorja`), front ones the syncopated stem before `-i` (`söpri`) (`VRB-PRES-006`); v-stems `rója`, `lövi`, `lőjük` (`VRB-STEM-010`).
- **Predictability**: `rule`
- **Lexical input**: harmony, slots
- **Interactions**: `VRB-PRES-003..006`, `VRB-MODEL-005`.
- **Sources**: [KVF98 p. 290], [SMNy3 pp. 516-517], [morphdb grammar PRES_INDIC_DEF_*]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-PRES-003 j-assimilation after s, sz, z, dz
- **Statement**: The `j` of definite suffixes (`-jA`, `-jUk`, `-jÁtOk`, `-jÁk`, `-jük`) assimilates totally to a stem-final `s`, `sz`, `z`, `dz`, and the result is written as a doubled letter.
- **Conditions**:
  - stem ends in `s` -> `ss`; `sz` -> `ssz`; `z` -> `zz`; `dz` -> `ddz`; also after clusters ending in these (`játsz` -> `játssz-`, `metsz` -> `metssz-`)
  - front stems show it only in 1pl `-jük` (`nézzük`, `főzzük`, `tesszük`, `eddzük`)
  - `zs`, `cs`, `c` are not in the set (no verbs affected in practice)
- **Output**: `olvas+ja` -> `olvassa`; `hoz+juk` -> `hozzuk`; `vadász+ják` -> `vadásszák`; `edz+jük` -> `eddzük`.
- **Examples**: `mos` -> `mossa`, `mossuk`, `mossátok`, `mossák`; `iszik` -> `issza`; `visz` -> `visszük`; `játszik` -> `játssza`, `játsszuk`, `játsszátok`, `játsszák`; `úszik` -> `ússza`
- **Exceptions**: in forms without `j` a single letter is written (`játszom`, `metszi`) [AkH12 §79]; indicative `játszunk` (indef) vs imperative `játsszunk` must be kept distinct in writing [AkH12 §80].
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-IMP-003` (same assimilation), `PH-ASSIM-004`, `ORT-ASSIM-011`, doubled digraph spelling `ORT-ASSIM-016`.
- **Sources**: [AkH12 §43, §79], [KVF98 p. 290, §3.4.1.1.6.1 p. 440], [SMNy3 p. 516]
- **Validation**: `attestation` (`olvassa` 16, `hozza` 16, `nézzük` 15, `issza` 6, `visszük` 8, `tesszük` 15, `játssza` 11, `eddzük` -)
- **Confidence**: `normative`

### VRB-PRES-004 j after other consonants
- **Statement**: After all other stem-final consonants the `j` of definite suffixes is written unchanged, even where pronunciation assimilates or fuses.
- **Conditions**:
  - `t`, `d`, `n` + `j`: written `tj`, `dj`, `nj`, pronounced `[c:]`, `[ɟ:]`, `[ɲ:]` (`látja`, `adja`, `fonja`) [AkH12 §57]
  - `gy`, `ny` + `j`: written `gyj`, `nyj` (`hagyja`, `hányja`) [AkH12 §54]
  - `l`, `ll` + `j`: written `lj`, `llj` (`tanulja`, `hallja`) [AkH12 §55]
  - clusters `st`, `szt` + `j`: written `stj`, `sztj` (`festjük`, `választja`)
  - non-standard: applying imperative-style assimilation to indicative definite forms of `t`-final verbs (`lássuk`, `válasszuk`, `fessük` for `látjuk`, `választjuk`, `festjük`) is heavily stigmatised (`VRB-IMP-012`)
- **Output**: concatenation without spelling change.
- **Examples**: `tanít` -> `tanítja`, `tanítjuk`; `mond` -> `mondja`; `szeret` -> `szeretjük`; `hány` -> `hányja`; `tart` -> `tartják`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-IMP-012`, `REP` (substandard spelling repair), `ORT-ASSIM-018`, `PH-ASSIM-007`.
- **Sources**: [AkH12 §54, §55, §57], [KVF98 p. 291 (Kontra 1995)]
- **Validation**: `attestation` (`látja` 17, `látjuk` 15; `választja` 13, `választjuk` 9; `festjük` 1 vs `fessük` 2, ambiguous with imperative)
- **Confidence**: `normative`

### VRB-PRES-005 -lak/-lek in the present
- **Statement**: The 1sg subject with 2nd person object is `-lAk` / `-AlAk` on `SQ`; after stems ending in `ll` the three `l`s are written as two.
- **Conditions**:
  - selection per `VRB-LINK-008`
  - `ll` stems: `hall+lak` -> `hallak`, `áll+lak` -> `állak`; spelling rule `ORT` (three identical consonant letters) [AkH12 §93]
  - suppletive stems: `eszlek`, `iszlak`, `teszlek`, `veszlek`, `viszlek`, `hiszlek`
- **Output**: `lát` -> `látlak`; `tanít` -> `tanítalak`; `hall` -> `hallak`.
- **Examples**: `szeret` -> `szeretlek`; `kér` -> `kérlek`; `vár` -> `várlak`; `ismer` -> `ismerlek`; `fest` -> `festelek`; `lő` -> `lőlek`; `ró` -> `rólak`
- **Exceptions**: intransitive verbs have no such form.
- **Predictability**: `rule`
- **Lexical input**: `H`
- **Interactions**: `VRB-LINK-008`, `VRB-PAST-011`, `VRB-COND-002`, `VRB-IMP-001`, `ORT-ASSIM-017`, `PH-ASSIM-012`.
- **Sources**: [KVF98 pp. 289-291], [AkH12 §93], [morphdb grammar PRES_INDIC_SG_1_OBJ_2]
- **Validation**: `attestation` (`látlak` 6, `kérlek` 14, `szeretlek` 11, `hívlak` 4)
- **Confidence**: `normative`

### VRB-PRES-006 Present stem selection for alternating classes
- **Statement**: Alternating classes realise the present cells from their slots as follows.
- **Conditions**:
  - epenthetic non-`-ik`: 1sg, 1pl, def 1sg/2sg, front def 3sg/2pl/3pl, and the vowel-initial 2sg `-Ol` of sibilant-final stems: syncopated (obligatory for `g`/`z`/`m`, optional for `l`/`r`); 2sg `-sz`, 2pl, 3pl, `-lAk`, back def 3sg/1pl/2pl/3pl, front 1pl def: full
  - epenthetic `-ik`: `-ik`, 1sg, 1pl: syncopated; 2sg, 2pl, 3pl: syncopated + LV ~ full (`VRB-STEM-008`)
  - v-stems: 1sg, 1pl, def 1sg/2sg, front def 3sg/2pl/3pl: `v` stem; all others vowel stem
  - sz-d-v: all present cells `sz` stem; `-lAk` from `d` stem where transitive (`alszik` none)
- **Output**: `érez`: `érzek`, `érzel`, `érez`, `érzünk`, `éreztek`, `éreznek`; def `érzem`, `érzed`, `érzi`, `érezzük`, `érzitek`, `érzik`, `érezlek`.
- **Examples**: `sodor` -> `sodrom ~ sodorom`, `sodrod`, `sodorja`, `sodorjuk`, `sodorjátok`, `sodorják`; `söpör` -> `söpröm`, `söpröd`, `söpri`, `söpörjük`, `söpritek`, `söprik`; `közöl` -> `közlöm`, `közli`, `közöljük`
- **Exceptions**: -
- **Predictability**: `rule` (given class)
- **Lexical input**: slots, `syncope_optional`
- **Interactions**: `VRB-STEM-007/008/010/012`.
- **Sources**: [SMNy3 p. 729 (morogja vs söpri)], [T04 §4.1.4.4 (47)], [KVF98 p. 291]
- **Validation**: `attestation` (`törli` 10, `töröljük` 8, `közli` 12, `közöljük` 9, `pótolja` 9, `sodorja` 7, `érzem` 16, `érezzük` expected)
- **Confidence**: `consensus`

### VRB-PRES-007 Syncretism and homography in the present
- **Statement**: Several present cells coincide with each other or with past cells; the analyser returns all readings.
- **Conditions**:
  - present 2pl indefinite `-tek` = past 3pl indefinite `-tek` for front stems with non-final past `-t-`: `kértek`, `néztek`, `szerettek`, `mentek` (`megy` 2pl present / 3pl past); `vettek` (`vet` 2pl present / `vesz` 3pl past)
  - 3sg indefinite `-ik` = 3pl definite `-ik` for front transitive `-ik` verbs: `eszik`
  - 1sg indefinite `-Om` of `-ik` verbs = 1sg definite: `eszem`, `iszom`, `játszom`
  - conditional 1sg indefinite = 3pl definite for front stems: `kérnék`
  - present definite 1pl = imperative definite 1pl for every stem whose imperative `-j` is written unchanged (all but sibilant- and `t`-final stems): `várjuk`, `kérjük`, `adjuk`, `lőjük`, `rójuk`; also back present def 3sg/2pl/3pl = imperative def (`várja`, `várjátok`, `várják`)
- **Output**: multiple analyses.
- **Examples**: `kértek` -> `kér` Pres Ind 2pl / Past Ind 3pl; `eszik` -> 3sg Ind / 3pl Def; `lőjük` -> Pres Def 1pl / Imp Def 1pl
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `SYN` disambiguation.
- **Sources**: [SMNy3 pp. 519-521 (homonymy avoidance)], [AkH12 §80]
- **Validation**: `goldens`
- **Confidence**: `consensus`

## PAST

### VRB-PAST-001 Past marker allomorphy
- **Statement**: The past marker has the forms `-t`, `-tt` and `-Ott`; its linking vowel depends on the stem-final consonant and on whether a person suffix follows, and after a linking vowel or a vowel the `t` is written double.
- **Conditions**:
  - word-final past (3sg indefinite, and the past participle): Class I behaviour: `-Ott` unless the stem ends in a vowel (`-tt`) or a coronal sonorant (`-t`), per `VRB-PAST-002..010`
  - non-final past (followed by a person suffix): Class II.A behaviour: `-t-` unless the stem class requires `-Ott-` (t-final classes T1, T2 except `lát`, T3, T4, T6; CC stems per `VRB-PAST-007`; geminates per `VRB-STEM-006`)
  - `O` three-way: `adott`, `kérdezett`, `ütött`; the following person suffixes use two-way `A` (`ütöttem`, `ütöttetek`)
  - spelling: `tt` after a vowel, `t` after a consonant [AkH12 §38]
- **Output**: past stem `SP+t` or `SP+Ott` or `SP+tt`, then `VRB-PAST-011` endings.
- **Examples**: `vár` -> `várt`, `vártam`; `ad` -> `adott`, `adtam`; `tart` -> `tartott`, `tartottam`; `lő` -> `lőtt`, `lőttem`
- **Exceptions**: see subclasses.
- **Predictability**: `rule` + `lexical` (CC non-final)
- **Lexical input**: class, CC past flag
- **Interactions**: `VRB-PAST-002..012`, `VRB-PART-002`, `PH-SYL-006`.
- **Sources**: [T04 §4.1.2.2 (9)-(11) pp. 149-152, §4.1.4.4], [SMNy3 pp. 517-519], [KVF98 pp. 294-295], [AkH12 §38]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-PAST-002 Vowel-final stems
- **Statement**: Stems ending in a vowel take `-tt` in all past cells.
- **Conditions**:
  - v-stem verbs (vowel stem, not `v` stem): `lőtt`, `nőtt`, `szőtt`, `főtt`, `rótt`, `rítt`, `nyűtt`
  - suppletive vowel past stems: `tett`, `vett`, `vitt`, `hitt`, `lett`, `jött`; `ett-`, `itt-` (non-3sg of `eszik`, `iszik`)
- **Output**: `lő` -> `lőttem`, `lőttél`, `lőtt`, `lőttünk`, `lőttetek`, `lőttek`.
- **Examples**: `ró` -> `rótt`, `róttak` [AkH12 §66]; `tesz` -> `tettem`; `jön` -> `jöttünk`
- **Exceptions**: `eszik`, `iszik` 3sg indef `evett`, `ivott` (`VRB-PAST-010`).
- **Predictability**: `rule`
- **Lexical input**: `SP`
- **Interactions**: `VRB-STEM-010/011`.
- **Sources**: [SMNy3 p. 517], [AkH12 §38, §66], [T04 fn. 18 p. 149]
- **Validation**: `attestation` (`lőtt` 13, `nőtt` 17, `rótt` 7, `tett` 20, `jött` 19)
- **Confidence**: `normative`

### VRB-PAST-003 Single coronal sonorant stems
- **Statement**: Stems ending in a single `l`, `r`, `n`, `ny`, `j`, `ly` take `-t` in all past cells.
- **Conditions**:
  - final consonant `l r n ny j ly`, not geminate, not an epenthetic `-ik` stem
  - applies also to full stems of non-`-ik` epenthetic verbs in `l`/`r` (`VRB-PAST-009`)
  - applies to polysyllables (`tanul`, `csinál`, `beszél`, `köszön`)
- **Output**: `vár` -> `vártam`, `vártál`, `várt`, `vártunk`, `vártatok`, `vártak`.
- **Examples**: `kér` -> `kért`; `fon` -> `font`; `hány` -> `hányt`; `fúj` -> `fújt`; `folyik` -> `folyt`; `tanul` -> `tanult`; `énekel` -> `énekelt`
- **Exceptions**: none in the standard; several past forms are homographs of other words (`ért` 'arrived' / 'understands', `félt`, `kelt`, `tört`), disambiguated in `SYN`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-PAST-004`, `VRB-PAST-009`.
- **Sources**: [KVF98 p. 295], [SMNy3 p. 517 (65a)], [T04 (9b) p. 149]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-PAST-004 Geminate stems
- **Statement**: Geminate-final verbs follow their per-item type (`VRB-STEM-006`).
- **Conditions**:
  - `áll`, `száll`: `-t` all cells (`állt`, `álltam`); `-ott` variants `állott`, `állottam` archaic, `állott` also adjective 'stale'
  - `hall`, `vall`, `kell`, `illik`, `ellik`, `züllik`, `izzik`, `fedd` (3sg): `-Ott` all cells (`hallott`, `hallottam`, `kellett`, `illett`, `izzott`)
  - `fedd`: `feddett` / non-3sg `-t-` (`feddtek`) [T04 (11c)]
  - `hull(ik)`, `mállik`, `pállik`: `-Ott ~ -t` (`hullott ~ hullt`, `hullottak ~ hulltak`)
  - `varr`, `forr`: `-t` preferred ~ `-ott` (`varrt ~ varrott`, `forrt ~ forrott`)
  - `függ`, `csügg`: 3sg `függött`, non-3sg `-t-` (`függtem`, `függtek`)
  - polysyllabic `-All`: `-t ~ -Ott` (`zöldellt ~ zöldellett`, `kéklett ~ kékellett`) [AkH12 §38]
- **Output**: per list.
- **Examples**: `száll` -> `szállt`, `szálltam`; `hall` -> `hallottuk`; `kell` -> `kellett`
- **Exceptions**: -
- **Predictability**: `lexical` (closed)
- **Lexical input**: geminate type
- **Interactions**: `VRB-STEM-006`.
- **Sources**: [KVF98 p. 295 (hall-ott, hull-ott~t, kell-ett, vall-ott)], [T04 (11)], [AkH12 §38, §64], [morphdb grammar V_PAST_TENSE_FILTER ll-finals, rr]
- **Validation**: `attestation` (`hallottam` 14, `halltam` -; `állt` 19, `állott` 12; `álltam` 10, `állottam` 1; `szállt` 13, `szállott` 6; `hullott` 10, `hullt` 5; `varrt` 5, `varrott` 4; `forrt` 4, `forrott` 2; `függött` 8, `függtek` 3)
- **Confidence**: `consensus`; `tendency` for doublet preference

### VRB-PAST-005 Other single-consonant stems
- **Statement**: Stems ending in a single consonant other than a coronal sonorant or `t` take `-Ott` in the 3sg indefinite and `-t-` in all other past cells.
- **Conditions**:
  - final `b d g k p f v h m s sz z zs gy ty c cs dzs`, single (not cluster, not geminate)
  - a monosyllabic stem in vowel + `dz` is excluded: the `dz` is pronounced long [d͡zː] (`PH-CONS-004`), so the stem patterns as a geminate and takes `-Ott-` in every cell (`edzettem`, `pedzettem`), as it takes the linking vowel of `-Ani` (`edzeni`, `VRB-LINK-002`); the polysyllabic `-(Ó)dz-` stems keep a short `dz` and the `-t-` (`lopództam`)
  - includes monosyllabic `ad`, `fed`, `véd`, `tud`, `szid`, root polysyllabic `-ad/-ed` (`fogad`, `szenved`), `-Og/-Oz` polysyllables (`dolgoz`, `kérdez`), d-final `-ik` verbs (`hazudik`, `csalódik`)
  - sibilant `-ik` verbs: `dolgozik` -> `dolgozott`, `dolgoztam`
- **Output**: `ad` -> `adtam`, `adtál`, `adott`, `adtunk`, `adtatok`, `adtak`; definite `adtam`, `adtad`, `adta`, `adtuk`, `adtátok`, `adták`.
- **Examples**: `hív` -> `hívott`, `hívtam`; `néz` -> `nézett`, `néztem`; `mos` -> `mosott`, `mostam`; `lép` -> `lépett`, `léptem`; `hagy` -> `hagyott`, `hagytam`; `nyom` -> `nyomott`, `nyomtam`; `edz` -> `edzett`, `edzettem`, not `*edztem`; `lopódzik` -> `lopództam`
- **Exceptions**: suffixal `-ad/-ed` (`VRB-PAST-008`); sz-d-v (`VRB-PAST-010`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-PAST-001`.
- **Sources**: [KVF98 p. 295], [SMNy3 p. 517 (65b)], [T04 (10) p. 150]
- **Validation**: `attestation` (`adtam` 12; `edzettem` 8,407 : `edztem` 24, `pedzettem` 66 : `pedztem` 2, `lopództam` 60 : `lopódzottam` 0 [WC2])
- **Confidence**: `consensus`

### VRB-PAST-006 t-final stems
- **Statement**: t-final verbs take `-Ott` in all past cells, except polysyllabic short-vowel `-t` stems and `lát`, which have `-Ott` only in the 3sg indefinite.
- **Conditions**:
  - T1 (`fut hat jut köt nyit süt üt vet`): `-Ott` all (`futottam`, `hatottam`, `vetettem`)
  - T2 `tát vét fűt hűt műt szít`: `-Ott` all (`fűtöttem`, `szítottam`)
  - T2 `lát`: 3sg `látott`, other `lát-t-` (`láttam`, `látta`); `lót`: `lótott`, non-3sg unattested (`lóttam` by analogy with `lát`, unverified; only in `lót-fut`)
  - T3 `bocsát`, T4 `-ít`, T6 C+t: `-Ott` all (`bocsátottam`, `tanítottam`, `festettem`, `tartottam`)
  - T5 polysyllabic short V + t: 3sg `-Ott`, other `-t-` merging into written `tt` (`mutatott`, `mutattam`, `szeretett`, `szerettem`, `alkotott`, `alkottam`, `olvashatott`, `olvashattam`, `várat` -> `váratott`, `várattam`)
- **Output**: see conditions.
- **Examples**: `köt` -> `kötöttem`, `kötött`, `kötötték`; `ugat` -> `ugatott`, `ugattam`; `követ` -> `követett`, `követtem`; `vét` -> `vétettem`; `hűt` -> `hűtöttük`
- **Exceptions**: `lát` (1 item). Deckstar's monosyllable list and T04 (63) agree on `hat`, `vet` being `-Ott-` in all persons, so they are not exceptions in the monosyllabic rule.
- **Predictability**: `rule` + `lexical` (`lát`)
- **Lexical input**: T-subclass
- **Interactions**: `VRB-STEM-003`.
- **Sources**: [T04 §4.1.4.6.2 (60)-(63) pp. 206-209], [SMNy3 pp. 518 (66)-(67)], [KVF98 p. 295 ("monosyllabic verbs ending in -t (except lát)")], [magyarispell ige_alanyi.aff t-final comments]
- **Validation**: `attestation` (`láttam` 17, `látottam` -; `futottam` 7, `futtam` -; `kötöttem` 6; `vetettem` 6; `mutattam` 7; `szerettem` 14; `követtem` 8; `alkottam` 3; `tanítottam` 7; `sütöttem` 7, `süttem` -)
- **Confidence**: `consensus`

### VRB-PAST-007 Consonant-cluster stems
- **Statement**: CC-final stems take `-Ott` in the 3sg indefinite; in the other cells the linking vowel is obligatory after `t`-final and ill-formed clusters, and lexically obligatory, optional or excluded after other clusters.
- **Conditions**:
  - 3sg indef always `-Ott`: `mondott`, `küldött`, `kezdett`, `tartott`, `festett`, `rajongott`
  - non-final, obligatory `-Ott-`: cluster ends in `t` (`tartottam`, `festettem`, `hajtottam`, `öntöttem`, `választottam`, `értettem`); cluster is not a possible word-final coda (`csuklottam`, `játszottam`, `metszettem`, `vedlettem`, `ugrottam`); lexically: `old` (`oldottam`), `száguld` (`száguldottak`; `?száguldtak` 118 : 1,723 [WC2]), `küzd` (`küzdöttem`)
  - non-final, no linking vowel (`-t-`), **59 verbs measured, 48 of them `-ng`**: `zeng mereng rajong dong döng kereng kering borong bolyong lázong lézeng szorong tolong ujjong zsong csüng csöng dühöng dereng terjeng töpreng őrjöng ...` plus the `-Cd` members `küld` (`küldtem`, `küldtek`), `kezd` (`kezdtem`), `hord` (`hordtam`), `kérd`, `örvend`, and `fedd` (`feddtem`), `függ` (`függtem`)
  - non-final, optional, **11 verbs measured**: `cseng`, `ing`, `leng`, `peng`, `reng`, `teng`, `ráng`, `fingik` (`fingtam ~ fingottam`), `érzik`, `esd`, and `mond` (`mondtam` ~ `mondottam`, the vowel form elevated/archaic); also `áld` (`áldtak ~ áldottak`), `told` (`toldottak ~ toldtak`) [T04 (11a), p. 152]
  - default for unlisted stems, **measured over 410 CC-final verbs**: every CC stem outside the two lists above (340 verbs) takes the linking vowel, so `-Ott-` remains the default in general - **but for `-ng` stems the default is the opposite, no linking vowel**, since 48 of the 59 no-vowel verbs are `-ng`; `-Cd` keeps `-Ott-`
- **Output**: per list.
- **Examples**: `mond` -> `mondtam`, `mondtál`, `mondott`, `mondtunk`, `mondtatok`, `mondtak`; def `mondtam`, `mondtad`, `mondta ~ mondotta`, `mondtuk`, `mondtátok`, `mondták ~ mondották`; `old` -> `oldottam`, `oldotta`, `oldották`; `küld` -> `küldtem`, `küldte`, `küldték`
- **Exceptions**: `edz` is heavy for the past linking vowel: `edzettem` 8,406 : `edztem` 24 and `edzettünk` 3,809 : `edztünk` 3 [WC2], `edztem` rejected by the spelling checker, so only `edzettem` is generated. `terem` -> `termettem` 272 (`VRB-PAST-009`).
- **Predictability**: `rule` (t-final, ill-formed; `-ng` default no vowel) + `lexical` (other clusters: 410 CC-final verb roots screened, 340 vowel / 59 none / 11 both)
- **Lexical input**: flag `cc_past` in {`vowel`, `none`, `both`}, from `cc-past-verbs.tsv` (410 rows) in `local/lists/cc-past-verbs.tsv` - read by `cargo xtask morphdb`
- **Interactions**: `VRB-PAST-011`, `VRB-PART-002`.
- **Sources**: [T04 (11) p. 151, §4.1.4.4 (51) p. 194], [KVF98 p. 295 (exceptions küld, mond, zeng, mereng)], [SMNy3 p. 518], [AkH12 §44 (mondta - mondotta)], [morphdb grammar V_PAST_TENSE_FILTER hesitant old, told, áld, küzd, hord], [WC2], [MTA-HELYES], [ATT evidence/lexical-lists.md 2026-09-16], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: `attestation`; 410 CC-final verbs screened with both diagnostic past forms, three-way decision at 90/10 with `both` reported as a class. Also `edzettem` 8,406 : `edztem` 24 [WC2]; `mondtam` 15, `mondottam` 6; `mondták` 15, `mondották` 4; `küldtek` 11, `küldöttek` 8 (ambiguous with participle plural); `küldtem` 7, `küldöttem` -; `kezdtem` 14, `kezdettem` 0; `oldották` 8, `oldták` -; `küzdöttek` 9, `küzdtek` -; `hordták` 6, `hordtam` 3, `hordottam` -; `rajongtam` 1
- **Confidence**: `consensus` (rule part); `consensus` (upgraded from `contested`) for the individual items, which are now enumerated with corpus counts (`OQ-05-003`, `OQ-05-005` resolved). morphdb's blanket "all `-Cd` hesitant" is not supported.

### VRB-PAST-008 -ad/-ed stems
- **Statement**: Suffixal `-Ad` verbs take `-t` in all past cells; root `-ad/-ed` verbs follow `VRB-PAST-005`.
- **Conditions**: class membership `VRB-STEM-005`.
- **Output**: `marad` -> `maradtam`, `maradt`, `maradtak`; `fogad` -> `fogadtam`, `fogadott`, `fogadtak`.
- **Examples**: `szalad` -> `szaladt`; `fárad` -> `fáradt`; `ébred` -> `ébredt`; `szenved` -> `szenvedett`; `enged` -> `engedett`; `tagad` -> `tagadott`
- **Exceptions**: fluctuating `támad`, `ragad`, `téved` (`VRB-STEM-005`); pronunciation of `dt` is `[tt]`, spelling keeps `dt`.
- **Predictability**: `lexical`
- **Lexical input**: `ad_suffixal`
- **Interactions**: `VRB-STEM-005`, `VRB-PART-002`.
- **Sources**: [T04 fn. 19 p. 150], [SMNy3 p. 518]
- **Validation**: `attestation` (see `VRB-STEM-005`)
- **Confidence**: `consensus`

### VRB-PAST-009 Epenthetic stems
- **Statement**: Non-`-ik` epenthetic verbs take a 3sg past on the syncopated stem + `-Ott` when the final consonant is not a coronal sonorant, and on the full stem + `-t` otherwise; the other past cells use the full stem + `-t-`. Epenthetic `-ik` verbs use the syncopated stem + `-Ott` in 3sg; in the other cells `l`/`r` members prefer the syncopated stem + `-Ott-`, `d`/`z` members the full stem + `-t-` (`VRB-STEM-008`).
- **Conditions**:
  - `C2` in {`g`, `z`, `m`}: 3sg `forgott`, `érzett`, `mozgott`, `termett`; other cells `forogtam`, `éreztem`, `mozogtunk`
  - `C2` in {`l`, `r`}: 3sg `énekelt`, `rabolt`, `sodort`, `söpört`, `közölt`; other cells `énekeltem`, `raboltam`, `sodortuk`
  - `-ik` epenthetic: `ugrott`, `ugrottam ~ ugortam`; `romlott`, `romlottak ~ romoltak`; `fürdött`, `fürödtem ~ fürdöttem`; `hiányzott`, `hiányoztam ~ hiányzottam`
- **Output**: see conditions.
- **Examples**: `forog` -> `forogtam`, `forogtál`, `forgott`, `forogtunk`, `forogtatok`, `forogtak`; `érez` -> `éreztem`, `érzett`, `éreztük`; `énekel` -> `énekeltem`, `énekelt`
- **Exceptions**: `terem` non-3sg forms take the syncopated stem: `termettem` 272, `termettünk` 202, robustly attested with 474 unambiguous tokens [WC2]. **The competing string cannot be counted**: the frequency list assigns both `termettem` and `teremtem` to lemma `terem`, but `teremtem` (1,319) is in fact dominated by the distinct verb `teremt` 'create' in its 1sg definite present, and no second analysis is listed, so the string ratio is unusable. Generate `termettem`, do not generate `teremtem` (`OQ-05-007` resolved, with a homograph caveat). `fürdik` and the `-zik` members (`hiányzik`, `dohányzik`, `lélegzik`, `vérzik`, `virágzik`): non-3sg on the full stem is normal (`fürödtem`, `hiányoztam`, `dohányoztam`, `virágoztak`).
- **Predictability**: `rule` (non-`-ik`), `tendency` (`-ik` variants)
- **Lexical input**: `epenthetic`, syncopated and full stems
- **Interactions**: `VRB-STEM-007/008`.
- **Sources**: [KVF98 p. 295 (éreztem/érzett, inogtam/ingott, mosolyogtam/mosolygott)], [T04 §4.1.4.4 (47), (51)], [SMNy3 pp. 724-726, 729], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: `attestation` (`termettem` 272, `termettünk` 202 [WC2]; `érzett` 12, `éreztem` 15, `érzettem` -; `mozgott` 10, `mozogtam` 2, `mozgottam` -; `forgott` 9, `forogtak` 4; `énekelt` 11; `sodorta` 8; `söpört` 6; `ugrott` 12, `ugrottam` 4, `ugortam` -; `romlott` 11)
- **Confidence**: `consensus`; `measured` for `terem`, with a stated homograph caveat

### VRB-PAST-010 sz-d-v and suppletive past stems
- **Statement**: Irregular verbs supply a lexical past stem; the 3sg indefinite may differ from the other cells.
- **Conditions**:

| Verb | 3sg indef | other cells |
|---|---|---|
| `van` | `volt` | `vol-t-` (`voltam`) |
| `lesz` | `lett` | `lett-` |
| `megy` | `ment` | `men-t-` (`mentem`) |
| `jön` | `jött` | `jött-` |
| `tesz`, `vesz`, `visz`, `hisz` | `tett`, `vett`, `vitt`, `hitt` | `tett-`, `vett-`, `vitt-`, `hitt-` |
| `eszik`, `iszik` | `evett`, `ivott` | `ett-`, `itt-` |
| `alszik`, `fekszik` | `aludt`, `feküdt` | `alud-t-`, `feküd-t-` |
| `nyugszik`, `haragszik`, `esküszik`, `alkuszik` | `nyugodott`, `haragudott`, `esküdött`, `alkudott` | `nyugod-t-`, `haragud-t-`, `esküd-t-`, `alkud-t-` |
| sz-d/sz-z with doublet | `cselekedett`, `dicsekedett`, `igyekezett`, `emlékezett`, `öregedett` | `cseleked-t-`, `igyekez-t-` |

- **Output**: table.
- **Examples**: `iszik` -> `ittam`, `ittál`, `ivott`, `ittunk`, `ittatok`, `ittak`; def `ittam`, `ittad`, `itta`, `ittuk`, `ittátok`, `itták`
- **Exceptions**: past participles of `eszik`, `iszik`: `evett`, `ivott` (`ivott ember`), not `ett`; `esküdt` is a noun/adjective ('juror, sworn'), `nyugodt` an adjective ('calm').
- **Predictability**: `lexical` (closed)
- **Lexical input**: `SP` per verb
- **Interactions**: `VRB-IRR-*`.
- **Sources**: [KVF98 p. 295], [T04 fn. 19], [SMNy3 p. 761], [morphdb grammar irregular verbs]
- **Validation**: `attestation` (`evett` 10, `ettek` 9, `evettek` -; `ivott` 10, `ittak` 8, `ivottak` -; `aludt` 10; `haragudott` 5; `esküdött` 5; `nyugodott` 6; `cselekedett` 9)
- **Confidence**: `consensus`

### VRB-PAST-011 Past person endings
- **Statement**: After the past marker, person suffixes use a two-way `A` (never `ö`) and a separate definite set.
- **Conditions**:

| Cell | Indefinite | Definite |
|---|---|---|
| 1sg | `-Am` | `-Am` |
| 2sg | `-Ál` | `-Ad` |
| 3sg | zero (marker word-final: `-Ott`/`-t`/`-tt`) | `-A` |
| 1pl | `-Unk` | `-Uk` |
| 2pl | `-AtOk` (a: `-atok`, e: `-etek`) | `-ÁtOk` (`-átok`, `-étek`) |
| 3pl | `-Ak` | `-Ák` |
| 1sg>2 | `-AlAk` | - |

  - 1sg indefinite and definite coincide (`vártam`)
  - 1pl indefinite `-Unk` vs definite `-Uk` distinct (`vártunk` / `vártuk`)
  - front rounded stems: `ütöttem`, `ütöttél`, `ütött`, `ütöttünk`, `ütöttetek`, `ütöttek`; def `ütötted`, `ütötte`, `ütöttük`, `ütöttétek`, `ütötték`, `ütöttelek`
- **Output**: table.
- **Examples**: `kér` -> `kértem`, `kértél`, `kért`, `kértünk`, `kértetek`, `kértek`; def `kértem`, `kérted`, `kérte`, `kértük`, `kértétek`, `kérték`, `kértelek`
- **Exceptions**: 3sg indefinite forms of `-Ott` classes keep the linking vowel; the other indefinite cells follow `VRB-PAST-001` (`adott` / `adtam`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-PAST-001`, `VRB-COND-004`.
- **Sources**: [SMNy3 pp. 519-520 (71)-(72)], [KVF98 pp. 294-295], [morphdb grammar PAST_INDIC_*]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-PAST-012 Past spelling
- **Statement**: The past marker is written `tt` after a vowel (stem vowel or linking vowel) and `t` after a consonant; assimilated pronunciations are not written.
- **Conditions**:
  - `tt` after vowel: `lőtt`, `adott`, `hallottuk`
  - `t` after consonant, incl. geminate stems: `varrtuk` (geminate pronounced short), `küldte`, `néztük` (devoicing `[st]` not written) [AkH12 §48, §59]
  - `d+t`, `z+t`, `gy+t` voicing assimilation not written (`adtam`, `hagytam`)
  - T5 `t+t` written `tt` (`mutattam`)
- **Output**: orthographic concatenation.
- **Examples**: `folyt`, `kente`, `varrtuk`, `csengetett`, `dobott`, `megrótták` [AkH12 §38]
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT`, `ORT-LEN-017`, `ORT-LEN-021`.
- **Sources**: [AkH12 §38, §48, §59]
- **Validation**: `goldens`
- **Confidence**: `normative`

## COND

### VRB-COND-001 Conditional marker and indefinite endings
- **Statement**: The present conditional consists of `-n-` (with linking vowel `A` after heavy stems) and a conditional person set with long `Á`, except 3sg indefinite `-A` and 1sg indefinite `-ék`.
- **Conditions**:

| Cell | Indefinite | Back `vár` | Front `kér` | Rounded `tör` |
|---|---|---|---|---|
| 1sg | `-nék` (invariant) | `várnék` | `kérnék` | `törnék` |
| 2sg | `-nÁl` | `várnál` | `kérnél` | `törnél` |
| 3sg | `-nA` | `várna` | `kérne` | `törne` |
| 1pl | `-nÁnk` | `várnánk` | `kérnénk` | `törnénk` |
| 2pl | `-nÁtOk` | `várnátok` | `kérnétek` | `törnétek` |
| 3pl | `-nÁnAk` | `várnának` | `kérnének` | `törnének` |

  - heavy stems: `-Anék`, `-AnÁl`, `-AnA`, ... (`tartanék`, `festenének`)
  - front rounded stems use `é`/`e` (no ö): `ülnétek`
- **Output**: table.
- **Examples**: `ad` -> `adnék`, `adnál`, `adna`, `adnánk`, `adnátok`, `adnának`; `tanít` -> `tanítanék`, `tanítana`; `lő` -> `lőnék`, `lőne`
- **Exceptions**: `-ik` traditional forms (`VRB-IK-004`); `van` -> `lennék ~ volnék` (`VRB-IRR-001`).
- **Predictability**: `rule`
- **Lexical input**: `H`
- **Interactions**: `VRB-LINK-007`, `VRB-IK-004`.
- **Sources**: [SMNy3 pp. 520-521 (74)-(75)], [KVF98 p. 309], [morphdb grammar PRES_COND_INDEF_*]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-COND-002 Conditional definite endings and -lAk
- **Statement**: The definite conditional is `-nÁm`, `-nÁd`, `-nÁ`, `-nÁnk`, `-nÁtOk`, `-nÁk`; 1pl and 2pl coincide with the indefinite.
- **Conditions**:

| Cell | Back `vár` | Front `kér` |
|---|---|---|
| 1sg | `várnám` | `kérném` |
| 2sg | `várnád` | `kérnéd` |
| 3sg | `várná` | `kérné` |
| 1pl | `várnánk` | `kérnénk` |
| 2pl | `várnátok` | `kérnétek` |
| 3pl | `várnák` | `kérnék` |
| 1sg>2 | `várnálak` | `kérnélek` |

  - front 3pl definite `kérnék` = 1sg indefinite `kérnék` (syncretism tolerated)
  - archaic 1pl definite `-nÓk` (`várnók`, `kérnők`): generated only with `Style=Arch`
- **Output**: table.
- **Examples**: `lát` -> `látnám`, `látnád`, `látná`, `látnánk`, `látnátok`, `látnák`, `látnálak`; `fest` -> `festeném`, `festené`, `festenék`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: `H`
- **Interactions**: `VRB-PRES-007`.
- **Sources**: [SMNy3 p. 521 (76)-(77), note on várnók/kérnők], [KVF98 p. 309], [morphdb grammar PRES_COND_DEF_PL_1 USAGE: formal]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-COND-003 Conditional stem
- **Statement**: The conditional attaches to `SQ` (same stem and linking vowel as the infinitive); suppletive verbs geminate the `n`.
- **Conditions**:
  - regular: infinitive minus `-i` + person set: `mondani` -> `mondan-` -> `mondana`
  - epenthetic non-`-ik`: full stem (`forogna`, `énekelne`)
  - epenthetic `-ik`: syncopated + LV ~ full (`ugrana ~ ugorna`)
  - v-stems: vowel stem (`lőne`, `róna`)
  - sz-d-v: `d` stem (`aludna`, `feküdne`, `haragudna`), sz-z: `z` stem (`igyekezne`)
  - suppletive: `tenne`, `venne`, `vinne`, `hinne`, `lenne`, `enne`, `inna`, `menne`, `jönne`
- **Output**: see conditions.
- **Examples**: `eszik` -> `ennék`, `ennél`, `enne`, `ennénk`, `ennétek`, `ennének`; def `enném`, `ennéd`, `enné`, `ennénk`, `ennétek`, `ennék`
- **Exceptions**: `kell` -> `kellene ~ kéne` (`VRB-COND-005`).
- **Predictability**: `rule` + `lexical` (irregular stems)
- **Lexical input**: `SQ` conditional stem
- **Interactions**: `VRB-INF-001`.
- **Sources**: [KVF98 p. 310 (vennék, enném, innám)], [SMNy3 p. 761]
- **Validation**: `attestation` (`lennék` 12, `menne` 10, `jönne` 10, `aludna` 1, `enne` 4, `inna` 2)
- **Confidence**: `consensus`

### VRB-COND-004 Past conditional
- **Statement**: The past conditional is the past indicative form (both conjugations, all persons) followed by the invariant particle `volna` as a separate word.
- **Conditions**:
  - `volna` does not inflect; it stays after the verb; neutral order with prefix: `elmentem volna`; with focus or negation the prefix separates: `nem mentem volna el`, `*nem mentem el volna`
  - `van`: `lettem volna` (10,297) is the form; `voltam volna` (44) is a literary archaism at 0.4% overall and 4% inside literature, not a free alternant. `lettem volna` also means 'would have become'
  - the lemma and features belong to the past form; `volna` gets `Mood=Cnd` and is analysed as an auxiliary/particle, the construction `Mood=Cnd|Tense=Past` at phrase level
- **Output**: `vártam volna`, `vártad volna`, `vártalak volna`.
- **Examples**: `megy` -> `mentem volna`; `lát` -> `láttuk volna`; `kell` -> `kellett volna`; the copula uses `lett volna`, not `volt volna` (245,186 : 535, and 79% of the `volt volna` tokens are in nineteenth-century literature) [ATT evidence/corpus-variants.md 2026-09-16].
- **Exceptions**: archaic synthetic-looking `vala` constructions (`vártam vala`) are historical pluperfect, not generated.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: word order (out of scope), modal syntax (out of scope).
- **Sources**: [SMNy3 p. 514], [KVF98 pp. 296, 310], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: `goldens` (phrase level); `attestation` (two-word regex over six registers; these patterns are in no word list)
- **Confidence**: `consensus`; `measured` and near-categorical for the copula, with the minority variant register-conditioned (`OQ-05-010` resolved)

### VRB-COND-005 Contracted and marginal conditional forms
- **Statement**: A few conditional forms have colloquial contractions or archaic doublets.
- **Conditions**:
  - `kellene` -> colloquial `kéne` (widely written informally); `kellett volna` -> `kellett volna` only
  - `volna` as present conditional of `van` (`volnék`, `volnál`, `volna`, `volnánk`, `volnátok`, `volnának`) is standard alongside `lennék` ...; preferred `lennék` in present meaning
  - archaic 1pl definite `-nók/-nők` (`VRB-COND-002`)
- **Output**: variants with `Style=Coll` / `Style=Arch`.
- **Examples**: `kéne`, `kellene`; `lenne ~ volna`
- **Exceptions**: -
- **Predictability**: `tendency` (`kellene` 19, `kéne` 15; `lenne` 21, `volna` 23 includes past-conditional particle uses)
- **Lexical input**: none
- **Interactions**: `VRB-IRR-001`, `VRB-IRR-022`.
- **Sources**: [morphdb grammar kell +kéne USAGE: informal, van +volna], [KVF98 pp. 289, 313]
- **Validation**: `attestation`
- **Confidence**: `consensus`

## IMP

### VRB-IMP-001 Imperative marker and person endings
- **Statement**: The imperative (subjunctive) is formed by the marker `-j` (with assimilations `VRB-IMP-002..006`) on `SJ`, followed by an imperative person set; the marker always requires a vowel-initial ending except in the short 2sg forms.
- **Conditions**:

| Cell | Indefinite | Definite |
|---|---|---|
| 1sg | `-jAk` | `-jAm` |
| 2sg | `-j` ~ `-jÁl` | short `-d` (`VRB-IMP-007`) ~ long `-jAd` |
| 3sg | `-jOn` (`-jék` formal `-ik`) | `-jA` |
| 1pl | `-jUnk` | `-jUk` |
| 2pl | `-jAtOk` (`-jatok`, `-jetek`) | `-jÁtOk` (`-játok`, `-jétek`) |
| 3pl | `-jAnAk` | `-jÁk` |
| 1sg>2 | `-jAlAk` | - |

  - no linking vowel before `-j`; harmony: `A` two-way, `O` three-way in 3sg indef only (`üssön`, `üljön`)
  - back `vár`: `várjak`, `várj ~ várjál`, `várjon`, `várjunk`, `várjatok`, `várjanak`, `várjalak`; def `várjam`, `várd ~ várjad`, `várja`, `várjuk`, `várjátok`, `várják`
  - front `kér`: `kérjek`, `kérj ~ kérjél`, `kérjen`, `kérjünk`, `kérjetek`, `kérjenek`, `kérjelek`; def `kérjem`, `kérd ~ kérjed`, `kérje`, `kérjük`, `kérjétek`, `kérjék`
- **Output**: table.
- **Examples**: `ül` -> `üljek`, `ülj`, `üljön`, `üljünk`, `üljetek`, `üljenek`; `ad` -> `adjak`, `adj`, `adjon`; def `adjam`, `add ~ adjad`, `adja`, `adjuk`, `adjátok`, `adják`
- **Exceptions**: irregular imperative stems (`VRB-IMP-009`); archaic 3sg `-jék` (`VRB-IK-005`, `VRB-IMP-010`).
- **Predictability**: `rule`
- **Lexical input**: `SJ`
- **Interactions**: `VRB-IMP-002..012`.
- **Sources**: [KVF98 pp. 310-311], [SMNy3 p. 522], [AkH12 §42, §44], [morphdb grammar SUBJ-IMP_*]
- **Validation**: `goldens`
- **Confidence**: `normative`

### VRB-IMP-002 Plain -j
- **Statement**: The marker is written `-j` after vowels and after all consonants except `s`, `sz`, `z`, `dz` and `t`; pronounced assimilations are not written.
- **Conditions**:
  - after vowel (v-stems use the vowel stem, `j` single): `lőjön`, `rójon`, `szőjön`, `ríjon`, `nyűjön`; never `lőjjön` [AkH12 §66; morphdb marks `lőjj` substandard orthography]
  - after `d`, `gy`, `n`, `ny`, `l`, `ll`: written `dj`, `gyj`, `nj`, `nyj`, `lj`, `llj` (`adjon`, `hagyjon`, `fonjon`, `hányjon`, `tanuljon`, `álljon`), pronounced as geminates [AkH12 §54, §55, §57]
  - after `b p k g f v h m r j ly ty c cs zs`: `dobjon`, `lopjon`, `rakjon`, `fogjon`, `döfjön`, `hívjon`, `nyomjon`, `várjon`, `fújjon` (j+j written `jj`), `folyjon`
  - epenthetic and sz-d-v verbs use the full / `d` stem: `forogjon`, `ugorjon`, `aludjon`
- **Output**: `SJ + j + ending`.
- **Examples**: `vár` -> `várj`; `fúj` -> `fújj`, `fújjon`; `hagy` -> `hagyj`; `lép` -> `lépj`; `mond` -> `mondj`; `küld` -> `küldjön`
- **Exceptions**: `jön` -> `jöjj-` (`VRB-IMP-009`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-IMP-003..006`, `ORT-ASSIM-021`, `PH-ASSIM-007`.
- **Sources**: [AkH12 §42, §54, §55, §57, §66], [KVF98 §3.4.1.1.6.2 p. 440]
- **Validation**: `attestation` (`adjon` 14, `hagyjon` 8, `lőjön` 2, `nőjön` 5)
- **Confidence**: `normative`

### VRB-IMP-003 Sibilant stems
- **Statement**: After stem-final `s`, `sz`, `z`, `dz` the marker assimilates totally and the result is written with a doubled letter.
- **Conditions**:
  - `s` -> `ss`, `sz` -> `ssz`, `z` -> `zz`, `dz` -> `ddz`, including after clusters (`játsz` -> `játssz-`, `metsz` -> `metssz-`, `alsz-` not used: sz-d-v use `d` stem)
  - applies to all imperative cells; short 2sg indef is the doubled form alone (`moss`, `hozz`, `játssz`, `eddz`)
- **Output**: `olvas` -> `olvassak`, `olvass ~ olvassál`, `olvasson`, `olvassunk`, `olvassatok`, `olvassanak`, `olvassalak`.
- **Examples**: `néz` -> `nézzen`; `mászik` -> `másszon`; `vadászik` -> `vadásszon`; `edz` -> `eddzen`; `tetszik` -> `tetsszen`; `hallatszik` -> `hallatsszon`; `lopódzik` -> `lopóddzon`
- **Exceptions**: suppletive `sz` verbs (`tesz`, `vesz`, `visz`, `hisz`, `lesz`, `eszik`, `iszik`) use `gy` stems (`VRB-IMP-009`); sz-d-v use `d` / `z` stems; `metsz`, `látszik`, `tetszik` may also be written with `ss` for `tssz` (`metssze ~ messe`, `látsszék ~ lássék`, `tetsszem ~ tessem`; short def `metszd ~ mesd`) [AkH12 §32, §80], `játszik`, `hallatszik` only `tssz`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-PRES-003`, `VRB-IMP-012`, `ORT-ASSIM-012`, `ORT-ASSIM-020`, `PH-ASSIM-004`.
- **Sources**: [AkH12 §42, §80], [KVF98 p. 311], [SMNy3 p. 523 (1)]
- **Validation**: `attestation` (`olvass` 11, `hozz` 8, `nézz` 11, `játssz` 8, `játsszon` 8, `mássz` 4, `eddz` 1)
- **Confidence**: `normative`

### VRB-IMP-004 Short vowel + t, and lát, bocsát, lót: ss
- **Statement**: Stems ending in short vowel + `t`, and the long-vowel stems `lát`, `bocsát`, `lót`, replace `t` + `j` by `ss`.
- **Conditions**:
  - `S0` ends in `a e i o u ö ü` + `t` (T1, T5, `-hAt`, `-gAt`, `-tAt`), or `S0` in {`lát`, `bocsát` and its prefixed forms, `lót`}
  - written `ss` in all cells except short definite 2sg (`VRB-IMP-007`)
- **Output**: `köt` -> `kössek`, `köss ~ kössél`, `kössön`, `kössünk`, `kössetek`, `kössenek`; def `kössem`, `kösd ~ kössed`, `kösse`, `kössük`, `kössétek`, `kössék`.
- **Examples**: `fut` -> `fusson`; `nyit` -> `nyissa`; `szeret` -> `szeress`; `mutat` -> `mutasd`; `alkot` -> `alkoss`; `hallgat` -> `hallgassák`; `lát` -> `lássuk`, `lásd`; `bocsát` -> `bocsáss`, `bocsásd`; `olvashat` -> `olvashasson`; `lót-fut` -> `lósson-fusson`
- **Exceptions**: other long vowel + `t` verbs (`VRB-IMP-005`).
- **Predictability**: `rule` + `lexical` (2 long-vowel roots + `lót`)
- **Lexical input**: T-subclass
- **Interactions**: `VRB-STEM-003`, `ORT-ASSIM-014`, `PH-ASSIM-005`.
- **Sources**: [AkH12 §77], [SMNy3 p. 523 (2i)], [KVF98 p. 311]
- **Validation**: `attestation` (`láss` 8, `lásd` 17, `lássuk` 13, `bocsáss` 6, `fuss` 7, `köss` 3, `kösd` 6, `szeress` 6, `mutasd` 10, `alkoss` 5)
- **Confidence**: `normative`

### VRB-IMP-005 Long vowel + t and consonant + t: ts
- **Statement**: Stems ending in a long vowel + `t` (other than `lát`, `bocsát`, `lót`), including all `-ít` verbs, and stems ending in a consonant other than `s`/`sz` + `t`, keep `t` and write the marker as `s`.
- **Conditions**:
  - long vowel + `t`: `-ít`, `tát`, `vét`, `fűt`, `hűt`, `műt`, `szít`
  - consonant + `t`, consonant not `s`/`sz`: `rt`, `lt`, `nt`, `jt`, `mt`: `tart`, `márt`, `olt`, `ölt`, `önt`, `ránt`, `ment`, `hajt`, `ejt`, `rejt`, `teremt`, `ért`, `sért`, `félt`
  - pronounced `[tʃ(:)]`; written `ts` in every cell; short def 2sg `-tsd` (`VRB-IMP-007`)
- **Output**: `tanít` -> `tanítsak`, `taníts ~ tanítsál`, `tanítson`, `tanítsunk`, `tanítsatok`, `tanítsanak`; def `tanítsam`, `tanítsd ~ tanítsad`, `tanítsa`, `tanítsuk`, `tanítsátok`, `tanítsák`.
- **Examples**: `fűt` -> `fűtsön`; `tart` -> `tartson`, `tartsd`; `hajt` -> `hajts`; `önt` -> `öntsd`; `ért` -> `értsd`; `ment` -> `mentsd`; `tát` -> `tátsd`; `vét` -> `vétsen`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: T-subclass
- **Interactions**: `VRB-IMP-004`, `VRB-IMP-006`, `ORT-ASSIM-013`, `PH-ASSIM-005`.
- **Sources**: [AkH12 §76], [SMNy3 p. 523 (2ii)], [KVF98 p. 311], [Wikt conj `tát`, `vét`]
- **Validation**: `attestation` (`tanítson` 4, `taníts` 5, `tanítsd` 4, `tarts` 8, `tartsd` 11, `hajtsd` 6, `öntsd` 9, `értsd` 9, `mentsd` 6, `hűtse` 1)
- **Confidence**: `normative`

### VRB-IMP-006 st and szt stems
- **Statement**: Stems ending in `st` or `szt` drop the `t` in the imperative and the marker assimilates to the sibilant: `ss`, `ssz`.
- **Conditions**:
  - `S0` ends in `st` or `szt`
  - all cells use `-ss-` / `-ssz-`; short def 2sg is truncated stem + `d` (`fesd`, `oszd`)
- **Output**: `fest` -> `fessek`, `fess ~ fessél`, `fessen`, `fessünk`, `fessetek`, `fessenek`; def `fessem`, `fesd ~ fessed`, `fesse`, `fessük`, `fessétek`, `fessék`.
- **Examples**: `oszt` -> `osszon`, `oszd`; `választ` -> `válassz`, `válaszd`; `ereszt` -> `eresszen`, `ereszd`; `füröszt` -> `fürösszétek`; `illeszt` -> `illesszük`; `rekeszt` -> `rekessze`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-IMP-012`, `ORT-ASSIM-015`, `PH-ASSIM-005`.
- **Sources**: [AkH12 §78], [SMNy3 p. 523 (2iii)], [KVF98 p. 311]
- **Validation**: `attestation` (`fess` 4, `fesd` 4, `oszd` 12, `válassz` 12, `válaszd` 12, `ereszd` 4)
- **Confidence**: `normative`

### VRB-IMP-007 Short definite 2sg
- **Statement**: The short definite 2sg is the imperative stem before the marker plus `-d`, with `-dd` after a vowel.
- **Conditions**: by stem type:
  - consonant stems with plain `-j`: stem + `d`: `várd`, `kérd`, `dobd`, `hagyd`, `hívd`, `írd`, `tanuld`
  - `d`-final stems: stem `d` + `d`, both written: `add`, `védd`, `tudd`, `mondd`, `küldd`, `hordd`, `kezdd`, `oldd`, `áldd` [AkH12 §62]
  - vowel stems: `dd`: `lődd`, `ródd`, `sződd`, `nyűdd` [AkH12 §66]; suppletive vowel stems: `tedd`, `vedd`, `vidd`, `edd`, `idd`, `hidd` [AkH12 §81]
  - sibilant stems: stem + `d`: `olvasd`, `nézd`, `hozd`, `mászd`, `edzd`, `játszd`, `metszd ~ mesd` [AkH12 §80]
  - short V + `t` and `lát`/`bocsát`: `t` -> `s` + `d`: `kösd`, `nyisd`, `mutasd`, `lásd`, `bocsásd` [AkH12 §77]
  - long V + `t` / C + `t`: `ts` + `d`: `tanítsd`, `tartsd`, `mentsd` [AkH12 §76]
  - `st`/`szt`: truncated + `d`: `fesd`, `oszd`, `válaszd` [AkH12 §78]
  - sz-d-v: `d` stem + `d`: `esküdd`; `alszik` intransitive
- **Output**: see conditions; long form `-jAd` always available (`VRB-IMP-008`).
- **Examples**: `ad` -> `add ~ adjad`; `lő` -> `lődd ~ lőjed`; `fest` -> `fesd ~ fessed`; `tanít` -> `tanítsd ~ tanítsad`
- **Exceptions**: spelling `dd` after consonant is kept although pronounced short (`mondd`) [AkH12 §59, §62].
- **Predictability**: `rule`
- **Lexical input**: `SJ`
- **Interactions**: `VRB-IMP-002..006`, `ORT`, `ORT-LEN-020`, `PH-ASSIM-006`.
- **Sources**: [AkH12 §38, §62, §66, §76-§81], [morphdb grammar SUBJ-IMP_DEF_SG_2 d_cut_final, dd_cut_final, d_add]
- **Validation**: `attestation` (`add` 14, `mondd` expected, `tedd` 14, `vedd` 13, `hidd` 11, `lődd` 6, `olvasd` 13, `nézd` 15, `kösd` 6, `lásd` 17, `tartsd` 11, `fesd` 4, `oszd` 12)
- **Confidence**: `normative`

### VRB-IMP-008 Long vs short 2sg forms
- **Statement**: Both 2sg imperative cells have a short and a long form that are equally correct; the short form is the unmarked default in writing.
- **Conditions**:
  - indefinite: short `-j`-stem (`várj`, `moss`, `taníts`) ~ long `-jÁl` (`várjál`, `mossál`, `tanítsál`)
  - definite: short `-d` (`VRB-IMP-007`) ~ long `-jAd` (`várjad`, `mossad`, `tanítsad`)
  - long forms are pragmatically softer (request) and more frequent in speech
  - irregular verbs lacking a short indefinite form: `tegyél`, `vegyél`, `vigyél`, `egyél`, `igyál`, `menj ~ menjél` (both), `jöjj ~ jöjjél ~ gyere`; archaic short `tégy`, `végy`, `vígy`, `égy`, `légy` (`légy` is fully current)
- **Output**: default short; long as variant.
- **Examples**: `kér` -> `kérj ~ kérjél`, `kérd ~ kérjed`; `higgy ~ higgyél`, `hidd ~ higgyed`
- **Exceptions**: see irregular list.
- **Predictability**: `rule`; usage `tendency` (`menj` 13 vs `menjél` 2; `legyél` 13 ~ `légy` 14; `tegyél` 11 vs `tégy` 9; `vegyél` 10 vs `végy` 7; `higgy` 7 vs `higgyél` -)
- **Lexical input**: irregular 2sg list
- **Interactions**: `VRB-IMP-009`.
- **Sources**: [AkH12 §44], [SMNy3 p. 522], [KVF98 p. 311]
- **Validation**: `attestation`
- **Confidence**: `normative`

### VRB-IMP-009 Irregular imperative stems
- **Statement**: Suppletive and alternating verbs use lexical imperative stems.
- **Conditions**:

| Verb | `SJ` | 1sg indef | 2sg indef | 3sg | short def 2sg |
|---|---|---|---|---|---|
| `van`, `lesz` | `legy-` | `legyek` | `légy ~ legyél` | `legyen` | - |
| `tesz` | `tegy-` | `tegyek` | `tegyél ~ tégy` | `tegyen` | `tedd` |
| `vesz` | `vegy-` | `vegyek` | `vegyél ~ végy` | `vegyen` | `vedd` |
| `visz` | `vigy-` | `vigyek` | `vigyél ~ vígy` (arch.) | `vigyen` | `vidd` |
| `hisz` | `higgy-` | `higgyek` | `higgy ~ higgyél` | `higgyen` | `hidd` |
| `eszik` | `egy-` | `egyek` | `egyél ~ égy` (arch.) | `egyen ~ egyék` | `edd` |
| `iszik` | `igy-` | `igyak` | `igyál` | `igyon ~ igyék` | `idd` |
| `megy` | `menj-` | `menjek` | `menj ~ menjél` | `menjen` | - |
| `jön` | `jöjj-` | `jöjjek` | `gyere ~ jöjj ~ jöjjél` | `jöjjön` | - |
| `alszik` | `aludj-` | `aludjak` | `aludj ~ aludjál` | `aludjon` | - |
| `fekszik` | `feküdj-` | `feküdjek` | `feküdj ~ feküdjél` | `feküdjön` | - |
| `nyugszik` | `nyugodj-` | `nyugodjak` | `nyugodj` | `nyugodjon` | - |
| `haragszik` | `haragudj-` | `haragudjak` | `haragudj` | `haragudjon` | - |
| `esküszik` | `esküdj-` | `esküdjek` | `esküdj` | `esküdjön` | `esküdd` |
| `igyekszik` | `igyekezz-` | `igyekezzek` | `igyekezz` | `igyekezzen` | - |
| `emlékszik` | `emlékezz-` | `emlékezzek` | `emlékezz` | `emlékezzen` | - |

  - `jön` 2pl indefinite `gyertek ~ jöjjetek`; `gyerünk` 'let's go' is a lexicalised 1pl
  - `gy` of `legy-`, `tegy-` ... is single, of `higgy-` double [AkH12 §81]
- **Output**: table; remaining cells regular on `SJ`.
- **Examples**: `vigyétek`, `igyátok`, `együnk`, `legyetek`, `higgyék`, `menjünk`, `jöjjenek`
- **Exceptions**: -
- **Predictability**: `lexical` (closed, 16 verbs + doublet partners)
- **Lexical input**: `SJ`
- **Interactions**: `VRB-IRR-*`, `ORT-ASSIM-019`, `PH-ASSIM-008`.
- **Sources**: [AkH12 §42, §67, §81], [KVF98 p. 311], [morphdb grammar irregular verbs], [SMNy3 p. 761]
- **Validation**: `attestation` (`legyen` 22, `légy` 14, `gyere` 12, `gyertek` 8, `jöjjetek` 7, `aludjon` 5, `feküdjön` 6, `nyugodjon` 5, `igyekezzen` 4, `emlékezzen` 5)
- **Confidence**: `normative`

### VRB-IMP-010 Archaic and fixed imperative forms
- **Statement**: The 3sg `-jék` ending survives in formal style (all `-ik` verbs, including archaic passive `-tatik/-tetik` formulae) and in lexicalised items; the only common non-`-ik` item is archaic `legyék`.
- **Conditions**:
  - `-ik` verbs: `VRB-IK-005`
  - formulae: `adassék`, `legyék` (archaic), `tessék` (lexicalised politeness/interjection), `áldassék`, `vétessék`
  - archaic 1sg `-jAm` for `-ik` verbs (`hadd egyem`, `hadd aludjam`)
- **Output**: variants with `Style=Arch/Formal`.
- **Examples**: `tetszik` -> `tessék`; `eszik` -> `egyék`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: `ik`
- **Interactions**: `VRB-IK-005`.
- **Sources**: [morphdb grammar SUBJ-IMP_INDEF_SG_3 +ék], [KVF98 pp. 310-311]
- **Validation**: `attestation` (`tessék` 12, `legyék` -)
- **Confidence**: `consensus`

### VRB-IMP-011 Functions
- **Statement**: The imperative paradigm serves both as imperative and as subjunctive; morphology is identical.
- **Conditions**:
  - direct commands (2nd person, prefix after verb: `olvasd el`), polite commands in 3rd person (`üljön le`), 1pl exhortative (`menjünk`), `hadd` + imperative (`hadd menjek`), purpose and complement clauses (`hogy olvasson`), 1st person deliberative questions (`mit csináljak?`)
  - negation with `ne` (`ne menj el`)
- **Output**: feature `Mood=Imp` for all uses; clause-level function in `SYN`.
- **Examples**: `Mondtam, hogy olvassa el.`; `Hadd főzzek.`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: word order (out of scope), negation syntax (out of scope).
- **Sources**: [KVF98 pp. 20-23, 311-312], [SMNy3 p. 522]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-IMP-012 Non-standard mergers of imperative and indicative
- **Statement**: Two non-standard patterns blur imperative and indicative forms; the generator never emits them, the analyser tags them `Typo` / `Style=Substd` with the standard reading.
- **Conditions**:
  - t-final verbs in the indicative definite with imperative assimilation (`lássuk`, `válasszuk`, `fessük`, `tanítsuk` for `látjuk`, `választjuk`, `festjük`, `tanítjuk`), stigmatised as uneducated
  - `-sz`-stem imperatives written without doubling (`játszunk` for `játsszunk`, `ne játszunk`), and hypercorrect doubling in the indicative
  - `higyj`, `lőjj`-type misspellings of irregular imperatives
- **Output**: analysis with standard form suggestion.
- **Examples**: `ne játszunk` -> suggest `ne játsszunk`; `azt lássuk, hogy` (indicative intent) -> `látjuk`
- **Exceptions**: `lássuk` is correct as imperative ('let us see').
- **Predictability**: `rule` (detection needs syntax)
- **Lexical input**: none
- **Interactions**: `REP`, `SYN`.
- **Sources**: [KVF98 p. 291 (Kontra 1995, Váradi and Kontra 1995)], [AkH12 §80], [LP-HU rules (ne játszunk)], [morphdb grammar substandard_orth]
- **Validation**: `goldens`
- **Confidence**: `normative`

## INF

### VRB-INF-001 Infinitive
- **Statement**: The infinitive is `-ni` / `-Ani` on `SQ` (`VRB-LINK-006`); irregular verbs have lexical infinitive stems.
- **Conditions**:
  - irregular: `van`, `lesz` -> `lenni`; `tesz` -> `tenni`; `vesz` -> `venni`; `visz` -> `vinni`; `hisz` -> `hinni`; `eszik` -> `enni`; `iszik` -> `inni`; `megy` -> `menni`; `jön` -> `jönni`; `alszik` -> `aludni`; `fekszik` -> `feküdni`; `nyugszik` -> `nyugodni`; `haragszik` -> `haragudni`; `esküszik` -> `esküdni`; `alkuszik` -> `alkudni`; sz-d/sz-z verbs -> doublet stem (`cselekedni`, `igyekezni`, `emlékezni`)
  - epenthetic non-`-ik`: full stem (`forogni`, `érezni`, `énekelni`)
  - `-ik` verbs: stem without `-ik` (`dolgozni`)
  - `kell` -> `kelleni` (rare)
- **Output**: `VerbForm=Inf`.
- **Examples**: `vár` -> `várni`; `tart` -> `tartani`; `lő` -> `lőni`; `ugrik` -> `ugrani ~ ugorni`
- **Exceptions**: `-hAt` + infinitive (`olvashatni`) is archaic, not generated.
- **Predictability**: `rule` + `lexical`
- **Lexical input**: `SQ`, `H`
- **Interactions**: `VRB-LINK-006`, `VRB-INF-002`.
- **Sources**: [KVF98 p. 204 section 2.1.1.3.2], [morphdb grammar INF, irregular verbs], [SMNy3 p. 761]
- **Validation**: `attestation` (`lenni` 19, `tenni` expected, `enni` 15, `inni` 13, `menni` 16, `jönni` 12, `aludni` 13, `feküdni` 6)
- **Confidence**: `consensus`

### VRB-INF-002 Personal infinitive
- **Statement**: The infinitive takes possessive-like person suffixes; 1sg, 2sg, 1pl, 2pl replace the final `-i`, 3sg and 3pl add `-A` / `-Uk` after it.
- **Conditions**:

| Cell | Suffix | Back `tanul` | Front `kér` | Rounded `ül` | Heavy `mond` |
|---|---|---|---|---|---|
| 1sg | `-nOm` | `tanulnom` | `kérnem` | `ülnöm` | `mondanom` |
| 2sg | `-nOd` | `tanulnod` | `kérned` | `ülnöd` | `mondanod` |
| 3sg | `-niA` | `tanulnia` | `kérnie` | `ülnie` | `mondania` |
| 1pl | `-nUnk` | `tanulnunk` | `kérnünk` | `ülnünk` | `mondanunk` |
| 2pl | `-nOtOk` | `tanulnotok` | `kérnetek` | `ülnötök` | `mondanotok` |
| 3pl | `-niUk` | `tanulniuk` | `kérniük` | `ülniük` | `mondaniuk` |

  - archaic 3pl `-niOk` (`tanulniok`), `Style=Arch`
  - irregular infinitives: `mennem`, `lennie`, `ennünk`, `innotok`, `tenniük`, `aludnom`, `jönnöd`
  - used with impersonal predicates taking a dative subject: `kell`, `lehet`, `szabad`, `muszáj`, `sikerül`, `érdemes`, `tilos` (modal syntax (out of scope))
- **Output**: `VerbForm=Inf|Person=X|Number=Y`.
- **Examples**: `Mennem kell.`; `Nem kell ezt tudnod.`; `Lehet, hogy el kell utazniuk.`
- **Exceptions**: rounded 2pl `-nötök` follows `O` three-way; `A` in 3sg is two-way (`ülnie`).
- **Predictability**: `rule`
- **Lexical input**: infinitive stem
- **Interactions**: modal syntax (out of scope).
- **Sources**: [KVF98 pp. 313-315 (kell + INF-1SG)], [morphdb grammar INF_SG_1..INF_PL_3 (archaic -ok)], [jepst hvcul.pl INF_BACK/FRNT/FRNR]
- **Validation**: `attestation` (`mennem` 9, `mennie` 11, `mennünk` 8, `menniük` 6, `tanulnom` 5, `tanulnia` 8, `tanulnunk` 7, `tanulniuk` 6, `mondanom` 11, `mondaniuk` 4)
- **Confidence**: `consensus`

### VRB-INF-003 Modal constructions interface
- **Statement**: Morphology provides the infinitive forms; the choice between bare and personal infinitive and the case of the subject are syntactic.
- **Conditions**:
  - bare infinitive after finite auxiliaries (`fog`, `akar`, `tud`, `bír`, `szokott`, `kezd`) and generic impersonals (`Itt nem lehet parkolni.`)
  - personal infinitive after impersonals with a specific dative subject (`Péternek mennie kell`)
  - the matrix verb agrees in definiteness with the object of the infinitive (`meg akarom venni a könyvet`)
- **Output**: pointer only.
- **Examples**: `El fogok menni.`; `Meg kell csinálnunk.`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: modal syntax (out of scope), `SYN-DEF`.
- **Sources**: [KVF98 pp. 204-206, 313-315]
- **Validation**: `goldens`
- **Confidence**: `consensus`

## PART

### VRB-PART-001 Present participle -Ó
- **Statement**: The present (imperfective) participle is `-ó` / `-ő` on the derivational stem `SD`.
- **Conditions**:
  - regular: `vár` -> `váró`, `kér` -> `kérő`, `ül` -> `ülő`
  - epenthetic: syncopated (`forgó`, `érző`, `éneklő`, `ugró`)
  - v-stems: `v` stem (`lövő`, `növő`, `szövő`, `rovó`, `fövő`, `rívó`, `nyüvő`)
  - suppletive/sz-d-v: `tevő`, `vevő`, `vivő`, `hívő ~ hivő`, `evő`, `ivó`, `levő ~ lévő`, `való` (van), `menő`, `jövő`, `alvó`, `fekvő`, `nyugvó`, `haragvó`, `esküvő`, `cselekvő`, `dicsekvő`, `törekvő`, `igyekvő`, `gyanakvó`, `növekvő`; sz-z: `emlékező`, `gyülekező`
  - `-ik` dropped (`dolgozó`)
  - vowel-final result lengthens nothing; further suffixes treat it as a vowel-final adjective/noun (`NOM`)
- **Output**: `VerbForm=PartPres`.
- **Examples**: `olvas` -> `olvasó`; `tanít` -> `tanító`; `lát` -> `látó`; `jön` -> `jövő`
- **Exceptions**: `lévő` and `levő` both standard [AkH12 §25]; `hívő` (believer) ~ `hivő`.
- **Predictability**: `rule` + `lexical` (`SD` of irregulars)
- **Lexical input**: `SD`
- **Interactions**: `VRB-PART-005`, `NOM`.
- **Sources**: [morphdb grammar AFF_Ó synthetic_derivation, irregular +tev, +lev, +lév], [AkH12 §25, §66], [SMNy3 p. 761]
- **Validation**: `attestation` (`tevő` 12, `vevő` 17, `evő` 4, `ivó` 6, `alvó` 10, `fekvő` 16, `lévő` 21, `levő` 16, `hívő` 12, `hivő` 2, `lövő` 7)
- **Confidence**: `normative`

### VRB-PART-002 Past participle
- **Statement**: The past (perfective) participle has the same form as the past 3sg indefinite by default; a lexical list takes `-Ott` where the finite past has `-t`.
- **Conditions**:
  - default = past 3sg indefinite (`VRB-PAST-001..010`): `várt`, `kért`, `adott`, `tartott`, `festett`, `lőtt`, `tett`, `evett`, `ivott`, `aludt`
  - lexical `-Ott` participles next to finite `-t`, closed list: `írt ~ írott`, `állt ~ állott` ('stale'), `forrt ~ forrott`, `varrt ~ varrott`, `hullt ~ hullott`
  - stems with 3sg past `-Ott`: the plural participle keeps the linking vowel where the finite non-3sg past lacks it: `küldöttek` vs finite `küldtek`, `adottak` vs `adtak` [T04 p. 152]
  - suffixal `-Ad`: `-t` (`fáradt`, `szakadt`)
  - prefix + participle written together (`elvégzett munka`)
- **Output**: `VerbForm=PartPast`.
- **Examples**: `ír` -> `írt ~ írott`; `küld` -> `küldött`, pl. `küldöttek`; `mos` -> `mosott`; `kitűz` -> `kitűzött`
- **Exceptions**: many participles are lexicalised with separate meaning (`nyugodt`, `esküdt`, `fáradt`, `hivatott`, `tanult`, `kötött`).
- **Predictability**: `rule` + `lexical` (~10 `-Ott` doublets)
- **Lexical input**: participle override
- **Interactions**: `VRB-PAST-*`, `NOM` (lowering: past participles are lowering adjectives, `festettek`).
- **Sources**: [T04 §4.1.2.2 p. 152], [AkH12 §38 (bérelt, felforrt, kitalált; elvégzett, kitűzött, mosott)], [morphdb grammar AFF_TT]
- **Validation**: `attestation` (`írott` 13, `írt` 18; `állott` 12; `forrott` 2)
- **Confidence**: `consensus`

### VRB-PART-003 Future participle -AndÓ
- **Statement**: The future (necessitative) participle is `-andó` / `-endő` on `SD`, meaning 'to be V-ed'; it is formed mainly from transitive verbs.
- **Conditions**:
  - harmony two-way (`-andó`, `-endő`; rounded stems take `-endő`: `küldendő`, `ütendő`)
  - irregular: `teendő`, `veendő`, `leendő`, `jövendő`, `viendő` (rare) [SMNy3 p. 762 (140)]; none for `eszik`, `iszik`, `megy`, `alszik`
  - no linking vowel alternation; `-ik` dropped
- **Output**: `VerbForm=PartFut`.
- **Examples**: `fizet` -> `fizetendő`; `olvas` -> `olvasandó`; `megold` -> `megoldandó`; `elvégez` -> `elvégzendő` (syncopated)
- **Exceptions**: many are lexicalised nouns (`teendő` 'task', `jövendő` 'future', `leendő` 'future (adj)', `mondandó` 'what one has to say').
- **Predictability**: `rule` (form), `tendency` (usage restricted to transitive verbs)
- **Lexical input**: `SD`
- **Interactions**: `VRB-PART-005`.
- **Sources**: [morphdb grammar AFF_ANDÓ, irregular +te AFF_ANDÓ, +le, +viv], [UD-hu VerbForm=PartFut]
- **Validation**: `attestation` (`teendő` 12, `veendő` 7, `leendő` 14, `jövendő` 9, `elvégzendő` 10, `fizetendő` 14, `olvasandó` 6, `megoldandó` 8)
- **Confidence**: `consensus`

### VRB-PART-004 Modal and negative participles
- **Statement**: `-hAtÓ` (modal 'V-able'), `-hAtAtlAn` ('un-V-able') and `-AtlAn` ('un-V-ed') are participle-like derivations on the analytic / derivational stem, owned by `DER-VERB` for their nominal behaviour.
- **Conditions**:
  - `-hAtÓ`: `SA` of `-hAt` (`VRB-POT-001`): `látható`, `ehető`, `iható`, `hihető`, `tehető`, `vehető`
  - `-hAtAtlAn`: `elképzelhetetlen`, `ehetetlen`, `ihatatlan`
  - `-AtlAn/-EtlEn` on `SD` (syncopated): `olvasatlan`, `festetlen`, `romlatlan`; v-stems `fövetlen`
- **Output**: adjectives; UD `VerbForm=Part` not assigned (ADJ).
- **Examples**: `lát` -> `látható`, `láthatatlan`, `látatlan`; `tör` -> `törhetetlen`
- **Exceptions**: `-AtlAn` is productive only on transitive action verbs (passive reading 'not V-ed'); intransitive and stative bases occur only in lexicalised words (`látatlan`, `hallatlan`, `független`, `nyugtalan`) (`DER-ADJ-010`).
- **Predictability**: `rule` (`-hAtÓ`, `-hAtAtlAn`); `rule` restricted to transitive action verbs + `lexical` (`-AtlAn`)
- **Lexical input**: verb `transitive`, `stative` (`-AtlAn`, `DER-ADJ-010`)
- **Interactions**: `DER-VERB`, `VRB-POT-001`.
- **Sources**: [morphdb grammar AFF_HATÓ, AFF_HATATLAN, AFF_ATLAN], [SMNy3 p. 723 (-AtlAn syncope)], [SMNy3 p. 124, p. 133 (12) (-AtlAn productive only on transitive action verbs)], [KVF98 p. 366]
- **Validation**: `attestation` (`látható` 19, `ehető` 9, `iható` 5, `hihető` 7)
- **Confidence**: `consensus`

### VRB-PART-005 Lexicalised participles
- **Statement**: Participles that have become nouns or adjectives with independent meaning are stored as lexemes and also analysed as participles.
- **Conditions**: common list: `vevő` 'customer', `tevő`, `író` 'writer', `olvasó` 'reader', `tanító` 'teacher', `jövő` 'future', `esküvő` 'wedding', `fekvő` 'lying; recumbent', `való` 'real', `lévő`, `menő` 'cool', `hívő` 'believer', `ivó` 'bar', `sütő` 'oven', `nyugodt` 'calm', `fáradt` 'tired', `esküdt` 'juror', `tanult` 'educated', `kötött` 'fixed', `teendő` 'task', `mondandó`
- **Output**: dual analysis: lemma noun/adj + derivational analysis `V+PartPres`.
- **Examples**: `a vevő` (NOUN), `a könyvet vevő ember` (VERB participle)
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexeme entries
- **Interactions**: `NOM`, `SYN`.
- **Sources**: [SMNy3 ch. 10 inflection/derivation boundary pp. 94-98], [morphdb peeta lexicon]
- **Validation**: `oracle:emMorph`
- **Confidence**: `consensus`

## ADVP

### VRB-ADVP-001 Adverbial participle -vA
- **Statement**: The adverbial participle (határozói igenév) is `-va` / `-ve` on the analytic stem.
- **Conditions**:
  - regular: `olvasva`, `ülve`, `állva`, `mondva`, `tanítva`, `festve`
  - epenthetic: full stem (`forogva`, `érezve`, `ugorva`)
  - v-stems: vowel stem (`lőve`, `szőve`, `főve`, `róva`, `ríva`)
  - irregular: `téve`, `véve`, `víve`; `alva ~ aludva`, `fekve ~ feküdve`, `nyugodva`, `menve`, `jőve` (archaic), `jövet` (adverb 'on the way here'), `menet` ('on the way'); `eszik` has **no current adverbial participle** (`éve` is 0 tokens tagged `[_AdvPtcp/Adv]` out of 2,434,990 tokens of the string, which are all `év` `[Poss.3Sg]`), `iszik` -> `%iva` (149) and `hisz` -> `%híve` (8) are marginal [WC2]
  - `v` is written, never assimilated (`dobva`, `lopva`)
- **Output**: `VerbForm=Conv`.
- **Examples**: `ül` -> `ülve`; `tesz` -> `téve`; `vesz` -> `véve` (`figyelembe véve`); `lát` -> `látva`
- **Exceptions**: `-ik` dropped (`dolgozva`); sz-d-v with `d` stem where no `v` form: `haragudva`, `esküdve`.
- **Predictability**: `rule` + `lexical`
- **Lexical input**: `SA`
- **Interactions**: `VRB-ADVP-002`.
- **Sources**: [morphdb grammar AFF_VA, irregular +té, +vé, +al, +fek, +men], [SMNy3 pp. 727-728], [UD-hu VerbForm=Conv], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: `attestation` (`téve` 14, `véve` 16, `alva` 3, `fekve` 9, `menve` 5, `olvasva` 9, `mondva` 9, `ülve` 12); the `eszik`/`iszik`/`hisz` forms measured with the `[_AdvPtcp/Adv]` tag filter, which is the only way to separate them from massively more frequent homographs (`éve` = `év` `[Poss.3Sg]`; `iva` = the proper noun `Iva`, 1,512 of 1,955; `híve` = `hív` `[Poss.3Sg]` 'his follower')
- **Confidence**: `consensus`; the `marginal` marking on `iva` and `híve` is `measured` and confirmed, and `éve` is measured **unattested** as a participle (`OQ-05-009` resolved). The 149 `iva` participle tokens are themselves split between a mis-analysed lemma `i` (130) and lemma `iva` (19), so even that figure is soft.

### VRB-ADVP-002 Adverbial participle -vÁn
- **Statement**: `-ván` / `-vén` is a formal/archaic variant of `-vA`, productive only in formal written style; a few forms are lexicalised conjunction-like items.
- **Conditions**:
  - same stem as `-vA`
  - lexicalised: `lévén` ('being', causal), `mondván` ('saying that'), `tudván`, `menvén`
- **Output**: `VerbForm=Conv|Style=Form`.
- **Examples**: `Beteg lévén nem jött.`; `mondván, hogy`
- **Exceptions**: `lévén` uses `lév-` (not `lé-`).
- **Predictability**: `rule` + `tendency` (`mondván` 13 vs `mondva` 9; `olvasván` 0 vs `olvasva` 9; `lévén` 13)
- **Lexical input**: none
- **Interactions**: `VRB-ADVP-001`.
- **Sources**: [morphdb grammar AFF_VÁN, van +lév AFF_VÁN]
- **Validation**: `attestation`
- **Confidence**: `consensus`

### VRB-ADVP-003 Past stem + -Á + possessive + inessive (jártában type)
- **Statement**: A marginal adverbial construction adds possessive + inessive/elative to the past participle-like stem: `jártában`, `mentében`, `ültében`, meaning 'while V-ing'.
- **Conditions**:
  - past stem (`-t`/`-tt`) + `-Á` (lengthened) + possessive + case: `ment+é+ben` -> `mentében`, `jár+t+á+ban` -> `jártában`, `jártomban`, `jártunkban`
  - restricted to motion/posture verbs (`jár`, `megy`, `jön`, `ül`, `áll`, `fekszik` -> `fektében` [SMNy3 p. 764]) and emotion set phrases (`ijedtében`, `féltében`); `kínjában`, `örömében` are nominal
- **Output**: `VerbForm=Conv` + possessor features, or treated as lexicalised adverbs.
- **Examples**: `mentében`, `jöttében`, `ijedtében`, `álltában`
- **Exceptions**: not productive; generator does not produce them by default.
- **Predictability**: `lexical`
- **Lexical input**: list
- **Interactions**: `NOM` possessive suffixes.
- **Sources**: [SMNy3 p. 764 (`fektében`)], [ATT]
- **Validation**: `attestation` (`mentében` 2, `jártában` 0, `ültében` -)
- **Confidence**: `contested` (status and productivity; see Open questions)

## POT

### VRB-POT-001 Potential -hAt attachment
- **Statement**: The potential `-hat` / `-het` attaches to the analytic stem `SA` without linking vowel; irregular verbs have lexical potential stems.
- **Conditions**:
  - harmony two-way (`-hat`, `-het`; rounded stems `-het`: `ülhet`, `lőhet`)
  - `-ik` dropped (`dolgozhat`); archaic `-hatik` not generated
  - epenthetic: full stem (`foroghat`, `érezhet`, `ugorhat`)
  - v-stems: vowel stem (`lőhet`, `róhat`, `nőhet`, `szőhet`)
  - irregular: `lehet` (`van`, `lesz`), `tehet`, `vehet`, `vihet`, `hihet`, `ehet`, `ihat`, `mehet`, `jöhet`, `alhat ~ aludhat`, `fekhet ~ feküdhet`, `nyugodhat ~ nyughat`, `haragudhat`, `esküdhet`, `cselekedhet`, `igyekezhet`, `emlékezhet`
  - `h` is never assimilated in spelling (`adhat`, `nézhet`)
- **Output**: potential stem = `SA + hAt`, inflected per `VRB-POT-002`.
- **Examples**: `vár` -> `várhat`; `tanít` -> `taníthat`; `fest` -> `festhet`; `megy` -> `mehet`
- **Exceptions**: `kell`, `van` (except `lehet`), `lesz` have no other potential forms; defective CC stems lack it (`VRB-STEM-009`).
- **Predictability**: `rule` + `lexical`
- **Lexical input**: potential stem for irregulars
- **Interactions**: `VRB-MODEL-003`.
- **Sources**: [KVF98 pp. 314-315], [morphdb grammar AFF_HAT], [SMNy3 p. 761]
- **Validation**: `attestation` (`tehet` 14, `vehet` 14, `ehet` 6, `ihat` 3, `vihet` 6, `mehet` 12, `jöhet` 14, `alhat` 1, `aludhat` 0, `lőhet` 0)
- **Confidence**: `consensus`

### VRB-POT-002 Inflection of potential stems
- **Statement**: A `-hAt` stem inflects as a regular polysyllabic short-vowel + `t` stem (class T5) in all moods, with both conjugations.
- **Conditions**:
  - present: `olvashatok`, `olvashatsz`, `olvashat`, `olvashatunk`, `olvashattok`, `olvashatnak`; def `olvashatom`, `olvashatod`, `olvashatja`, `olvashatjuk`, `olvashatjátok`, `olvashatják`, `olvashatlak`
  - past: 3sg `olvashatott`, others `olvashatt-` (`olvashattam`, `olvashatta`)
  - conditional: `olvashatnék`, `olvashatna`, `olvashatnám`
  - imperative (`ss`, `VRB-IMP-004`): `olvashassak`, `olvashass`, `olvashasson`, `olvashassuk`; short def 2sg `olvashasd`
  - `-ik` verbs 1sg indef: `-hatok` (not `-hatom`, which is definite)
- **Output**: see conditions.
- **Examples**: `megy` -> `mehetek`, `mehetsz`, `mehet`, `mehetünk`, `mehettek`, `mehetnek`, `mehettem`, `mehetett`, `mehetne`, `mehessek`, `mehessen`; `van` -> `lehetek`, `lehetett`, `lehetne`, `lehessen`
- **Exceptions**: none; front 2pl `mehettek` = past 3pl `mehettek` (homograph).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-STEM-003` (T5), `VRB-PAST-006`, `VRB-IMP-004`.
- **Sources**: [SMNy3 p. 518 (olvas+hat like -CVt)], [T04 (61) p. 207], [morphdb grammar AFF_HAT OUT: Vtt_final_past t_nonfinal_past quasi_analytic_C]
- **Validation**: `attestation` (`mehetek` 7, `mehetsz` 6, `mehettem` 4, `mehetett` 8, `mehessen` 4, `lehessen` 15, `olvashattam` 2, `dolgozhatok` 3, `dolgozhatom` -)
- **Confidence**: `consensus`

### VRB-POT-003 Desiderative -hatnék
- **Statement**: `-hatnék` / `-hetnék` forms a defective noun 'urge to V', used with possessive suffixes and `van`/`támad`.
- **Conditions**:
  - stem as `VRB-POT-001`; only nominative with possessive: `-hatnékom`, `-hatnékod`, `-hatnékja`, `-hatnékunk`, `-hatnékotok`, `-hatnékjuk`
  - common: `mehetnékem van`, `sírhatnékom támadt`, `nevethetnékem`, `ehetnékem`, `alhatnékom`
- **Output**: NOUN with `Poss` features; derivational analysis `V+hatnék`.
- **Examples**: `mehetnékje van`; `aludhatnékom ~ alhatnékom`
- **Exceptions**: the bare form `mehetnék` is homographous with the potential conditional 1sg (`mehetnék` 'I could go').
- **Predictability**: `rule` (form), `tendency` (usage mostly intransitive verbs)
- **Lexical input**: none
- **Interactions**: `NOM-POSS`.
- **Sources**: [morphdb grammar AFF_HATNEK (defective_inflection poss)]
- **Validation**: `attestation` (`mehetnék` 2; possessed forms not in the list)
- **Confidence**: `consensus`

## CAUS

### VRB-CAUS-001 Causative allomorph selection
- **Statement**: The causative is `-tat` / `-tet` after polysyllabic stems, `-ít` stems and vowel + `t` monosyllables, and `-at` / `-et` after other monosyllabic stems; a lexical minority of monosyllables takes `-tat/-tet`.
- **Conditions**:
  - polysyllabic stem (incl. `-ít`): `-tAt`: `olvastat`, `dolgoztat`, `énekeltet`, `vásároltat`, `taníttat`, `építtet`, `javíttat`
  - monosyllabic ending in vowel + `t`: `-tAt` (written `tt`): `futtat`, `üttet`, `köttet`, `láttat`, `fűttet`, `süttet`
  - monosyllabic ending in consonant + `t`: `-At`: `tartat`, `festet`, `öntet`, `értet`
  - other monosyllables: `-At` (default): `írat`, `várat`, `kéret`, `mosat`, `adat`, `hívat`, `hozat`, `vágat`, `veret`, `járat`
  - lexical `-tAt` monosyllables (closed, mainly `l`-final): `ül` -> `ültet`, `él` -> `éltet`
  - sz-d-v and suppletive: `CAUS-002`
  - harmony two-way; the result is a T5 stem (`VRB-STEM-003`)
- **Output**: `Voice=Cau` derivative lemma (e.g. lemma `olvastat` with analysis `olvas+tat`).
- **Examples**: `ír` -> `írat`; `olvas` -> `olvastat`; `köt` -> `köttet`; `fest` -> `festet`; `ül` -> `ültet`
- **Exceptions**: meaning-split pairs with the lexical causative/factitive (`forgat` 'turn' vs `forogtat`; `ültet` 'seat, plant'); defective or unused causatives are frequent; ownership of productivity details: `DER-VERB`.
- **Predictability**: `rule` + `lexical` (`aff_tat_V`/`aff_tat_C` flag on monosyllables). The monosyllable split is now enumerated rather than exemplified: over **153 measured monosyllables, 119 take `-At`, 19 take `-tAt` and 15 take both** [WC2].
- **Lexical input**: monosyllable causative flag, from `causative-selection.tsv` (153 rows) in `local/lists/causative-selection.tsv` - working data, not yet read by the build
- **Interactions**: `DER-VERB`, `VRB-POT-002` (same T5 inflection), `ORT-LEN-019`.
- **Sources**: [morphdb grammar AFF_TAT (analytic polisyllabic -tat; synthetic monosyllabic -at; aff_tat_C)], [SMNy3 p. 718 (forgat vs forogtat)], [WC2], [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: `attestation` (`várat` 11, `adat` 18 (noun homograph), `fektet` 10, `ültet` 4, `éltet` 4, `építtet` 3, `futtat` 3); 153 monosyllables screened with both diagnostic forms
- **Confidence**: `consensus` (default); the monosyllable exception list is `measured` (`OQ-05-012` resolved)

### VRB-CAUS-002 Causatives of irregular verbs
- **Statement**: Irregular verbs form causatives on a lexical stem.
- **Conditions**: list: `eszik` -> `etet`; `iszik` -> `itat`; `tesz` -> `tetet`; `vesz` -> `vetet`; `visz` -> `vitet`; `hisz` -> `hitet` (in `elhitet`); `alszik` -> `altat`; `fekszik` -> `fektet`; `nyugszik` -> `nyugtat`; `lő` -> `lövet`; `sző` -> `szövet` (verb, rare; homograph of noun `szövet`); `nő` -> `növeszt` (different suffix); `megy`, `jön`, `van` -> none (`járat`, `hozat` are separate); `lát` -> `láttat`; `hall` -> `hallat`
- **Output**: lexical derivative.
- **Examples**: `etetem a macskát`; `elaltat`; `lefektet`; `megnyugtat`
- **Exceptions**: -
- **Predictability**: `lexical` (closed)
- **Lexical input**: causative stem
- **Interactions**: `DER-VERB`.
- **Sources**: [morphdb grammar irregular verbs +te/+ve/+vi/+hi/+e/+i/+al/+fek/+nyug AFF_TAT]
- **Validation**: `attestation` (`etet` 2, `hitet` 11 (in derivatives), `fektet` 10, `nyugtat` 4)
- **Confidence**: `consensus`

### VRB-CAUS-003 Medio-passive and reflexive derivation hooks
- **Statement**: Verb-to-verb derivations that change voice are derivation (`DER-VERB`); this chapter only fixes their inflection class.
- **Conditions**:
  - `-ódik/-ődik` (medio-passive/reflexive on `SD`: `mosódik`, `íródik`, `kötődik`): `-ik` verb, polysyllabic `d`: past `-ott`/-`t-` (`íródott`, `íródtam`), non-heavy
  - `-kodik/-kedik/-ködik` (reflexive: `mosakodik`, `törölközik`, `öltözködik`): `-ik` verb; some with sz-doublet (`mosakszik`, `VRB-STEM-012`)
  - `-ul/-ül` (`javul`, `épül`): single `l`: past `-t` (`javult`)
  - `-Odik` inchoative (`öregedik`, `melegedik`), `-Ad/-Ed` (`VRB-STEM-005`)
  - `-dzik/-ódzik` variants of `-zik` (`csókolódzik ~ csókolózik`) [AkH12 §25b, §44]: non-heavy
  - passive `-tatik/-tetik` (`adatik`, `mondatik`): archaic, `-ik` verbs, 3rd person only in practice
- **Output**: class flags for derived lemmas.
- **Examples**: `mosódik` -> `mosódott`, `mosódnak`; `javul` -> `javultak`; `adatik` -> `adatott`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: derivational analysis
- **Interactions**: `DER-VERB`.
- **Sources**: [morphdb grammar AFF_ÓDIK, AFF_KODIK], [AkH12 §25, §44], [SMNy3 p. 723 (-(k)Odik / -(k)Ozik sz-stems)]
- **Validation**: `oracle:hunmorph-foma`
- **Confidence**: `consensus`

### VRB-CAUS-004 Inflection class of derived verbs
- **Statement**: Derived verbs take the inflection class of their final derivational suffix, independent of the base.
- **Conditions**:

| Suffix | Class | Example forms |
|---|---|---|
| `-gAt`, `-OgAt` | T5 | `olvasgat`, `olvasgattam`, `olvasgatott`, `olvasgass` |
| `-tAt`, `-At` | T5 | `írat`, `írattam`, `íratott`, `írass` |
| `-hAt` | T5 (inflectional) | `VRB-POT-002` |
| `-ít` | T4 | `tanít`, `tanítottam`, `taníts` |
| `-Ol`, `-Al`, `-Ul` | SON single `l` | `tanul`, `tanult`, `tanuljon` |
| `-Oz`, `-Ozik`, `-zik` | SIB `z` | `kérdez`, `kérdezett`, `kérdezz` |
| `-Og`, `-Ong` | OBS `g` / CC `ng` | `csillog`, `csillogott`, `rajong`, `rajongtam` |
| `-An`, `-En` (momentane) | SON `n` | `villan`, `villant`, `villanjon` |
| `-ódik`, `-kodik` | OBS `d`, `-ik` | `íródik`, `íródott`, `íródtam` |
| `-All` | geminate `ll` | `VRB-STEM-006` |
| `-(A)szt`, `-(E)szt` | T6s | `ébreszt`, `ébresztettem`, `ébressz` |

- **Output**: class record.
- **Examples**: `nézeget` -> `nézegetett`, `nézegettem`, `nézegess`; `futkos` -> `futkosol`, `futkossa`
- **Exceptions**: `-Ol/-Oz/-Og` derivatives can be epenthetic (`VRB-STEM-007`).
- **Predictability**: `rule`
- **Lexical input**: derivational segmentation
- **Interactions**: `DER-VERB`, `VRB-STEM-014`.
- **Sources**: [T04 (61) (-gat, -tat as at/et stems)], [SMNy3 p. 518], [morphdb grammar AFF_GAT, AFF_TAT OUT: Vtt_final_past t_nonfinal_past]
- **Validation**: `goldens`
- **Confidence**: `consensus`

## IRR

Table rows: `PI` present indicative indefinite, `PD` present definite, `XI`/`XD` past indefinite/definite, `CI`/`CD` conditional, `II`/`ID` imperative; last column 1sg>2. Nonfinite line order: infinitive; personal infinitive 1sg/3sg/3pl; potential; present participle; past participle; future participle; adverbial participle; other.

### VRB-IRR-001 van
- **Statement**: `van` 'be' is suppletive: present `vagy-`/`van-`, past `vol-`, all other moods and nonfinite forms from `lesz`/`le-`; it has no definite conjugation and fused negative forms in the 3rd person.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `vagyok` | `vagy` | `van` | `vagyunk` | `vagytok` | `vannak` |
| PI negative | `nem vagyok` | `nem vagy` | `nincs ~ nincsen` | `nem vagyunk` | `nem vagytok` | `nincsenek` |
| XI | `voltam` | `voltál` | `volt` | `voltunk` | `voltatok` | `voltak` |
| future | `leszek` | `leszel` | `lesz` | `leszünk` | `lesztek` | `lesznek` |
| CI | `lennék ~ volnék` | `lennél ~ volnál` | `lenne ~ volna` | `lennénk ~ volnánk` | `lennétek ~ volnátok` | `lennének ~ volnának` |
| II | `legyek` | `légy ~ legyél` | `legyen` | `legyünk` | `legyetek` | `legyenek` |

  - nonfinite: `lenni`; `lennem`, `lennie`, `lenniük`; `lehet` (`lehetek`, `lehetett`, `lehetne`, `lehessen`); `való ~ levő ~ lévő`; `volt` (adj. 'former'); `leendő`; `lévén`
  - `sincs ~ sincsen`, `sincsenek` = `sem` + `van`
  - past conditional `lettem volna ~ voltam volna`
  - 3rd person present is omitted with nominal predicates (`Péter orvos`) (`SYN`)
  - archaic: `vagyon` (3sg), `vala`, `valék`, `valának`, `lőn`, `lészen`
- **Output**: table; `nincs` analysed as `van` + `Polarity=Neg`.
- **Examples**: `vagytok`, `voltatok`, `lennétek`, `legyetek`
- **Exceptions**: no `Definite=Def` cells, no `-lAk`.
- **Predictability**: `lexical`
- **Lexical input**: full table
- **Interactions**: -
- **Sources**: [KVF98 pp. 289, 295, 297], [morphdb grammar irregular van], [jepst hvcul.pl van]
- **Validation**: `attestation` (`vagyok` 21, `vagytok` 13, `nincs` 23, `nincsen` 15, `nincsenek` 17, `lennék` 12, `volnék` 8, `légy` 14, `legyél` 13, `vala` 14)
- **Confidence**: `normative`

### VRB-IRR-002 lesz
- **Statement**: `lesz` 'become, will be' has a regular-looking `sz` present and shares its other stems with `van`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `leszek` | `leszel` | `lesz` | `leszünk` | `lesztek` | `lesznek` |
| XI | `lettem` | `lettél` | `lett` | `lettünk` | `lettetek` | `lettek` |
| CI | `lennék` | `lennél` | `lenne` | `lennénk` | `lennétek` | `lennének` |
| II | `legyek` | `légy ~ legyél` | `legyen` | `legyünk` | `legyetek` | `legyenek` |

  - nonfinite: `lenni`; `lennem`, `lennie`, `lenniük`; `lehet`; `levő ~ lévő`; `lett`; `leendő`; -
  - spelling always single `sz` (`leszek`) [AkH12 §68]
- **Output**: table.
- **Examples**: `leszel`, `lettünk`, `lennének`
- **Exceptions**: no definite cells.
- **Predictability**: `lexical`
- **Lexical input**: full table
- **Interactions**: `VRB-IRR-001`, `ORT-LEN-027`.
- **Sources**: [AkH12 §68, §81], [morphdb grammar van +lesz, +le, +len, +legy]
- **Validation**: `attestation` (`leszek` 14, `lesztek` 8, `lettem` 13)
- **Confidence**: `normative`

### VRB-IRR-003 megy
- **Statement**: `megy` 'go' uses `megy-` in the present 1sg, 3sg, 1pl, `mész` in 2sg, `me-` before `-hAt` (`mehet`) and `men-` elsewhere; it is intransitive.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `megyek` | `mész` (arch. `mégy`) | `megy` | `megyünk` | `mentek` | `mennek` |
| XI | `mentem` | `mentél` | `ment` | `mentünk` | `mentetek` | `mentek` |
| CI | `mennék` | `mennél` | `menne` | `mennénk` | `mennétek` | `mennének` |
| II | `menjek` | `menj ~ menjél` | `menjen` | `menjünk` | `menjetek` | `menjenek` |

  - nonfinite: `menni`; `mennem`, `mennie`, `menniük`; `mehet`; `menő`; `ment`; -; `menve ~ menvén`; adverb `menet`
- **Output**: table.
- **Examples**: `elmész`, `bementek`, `kimenjünk`
- **Exceptions**: 2pl present `mentek` = 3pl past `mentek`.
- **Predictability**: `lexical`
- **Lexical input**: full table
- **Interactions**: `VRB-PRES-007`.
- **Sources**: [KVF98 p. 291], [morphdb grammar irregular megy]
- **Validation**: `attestation` (`megyek` 13, `mész` 11, `mégy` 5, `mentek` 14, `mentetek` 1, `menjél` 2, `menvén` 4)
- **Confidence**: `normative`

### VRB-IRR-004 jön
- **Statement**: `jön` 'come' uses `jöv-` before vowels, `jön-` before `n`, `jö-` with doubled consonants elsewhere, and suppletive `gyere`, `gyertek` in the 2nd person imperative.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `jövök` | `jössz` | `jön` | `jövünk` | `jöttök` | `jönnek` |
| XI | `jöttem` | `jöttél` | `jött` | `jöttünk` | `jöttetek` | `jöttek` |
| CI | `jönnék` | `jönnél` | `jönne` | `jönnénk` | `jönnétek` | `jönnének` |
| II | `jöjjek` | `gyere ~ jöjj ~ jöjjél` | `jöjjön` | `jöjjünk` | `gyertek ~ jöjjetek` | `jöjjenek` |

  - nonfinite: `jönni`; `jönnöm`, `jönnie`, `jönniük`; `jöhet`; `jövő`; `jött`; `jövendő`; `jőve` (arch.); adverb `jövet`
  - spelling: short `ö` in all suffixed forms except archaic `jő`, `jőve` [AkH12 §67]; consonants after `jö-` doubled (`jössz`, `jöjj`)
  - non-standard `jösztök`, `jösz`
- **Output**: table.
- **Examples**: `eljövök`, `bejöttök`, `gyertek be`
- **Exceptions**: `gyerünk` 'let's go' lexicalised.
- **Predictability**: `lexical`
- **Lexical input**: full table
- **Interactions**: `VRB-IMP-009`, `ORT-LEN-026`.
- **Sources**: [AkH12 §42, §67], [KVF98 pp. 291, 311], [morphdb grammar irregular jön]
- **Validation**: `attestation` (`jövök` 10, `jössz` 9, `jöttök` 4, `gyere` 12, `jöjj` 10, `jöjjetek` 7, `jövet` 6, `jösztök` -)
- **Confidence**: `normative`

### VRB-IRR-005 eszik
- **Statement**: `eszik` 'eat' is a transitive `-ik` verb of the tesz type (`VRB-STEM-011`) with past 3sg indefinite `evett`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `eszem ~ eszek` | `eszel` | `eszik` | `eszünk` | `esztek` | `esznek` | `eszlek` |
| PD | `eszem` | `eszed` | `eszi` | `esszük` | `eszitek` | `eszik` | |
| XI | `ettem` | `ettél` | `evett` | `ettünk` | `ettetek` | `ettek` | `ettelek` |
| XD | `ettem` | `etted` | `ette` | `ettük` | `ettétek` | `ették` | |
| CI | `ennék` (trad. `enném`) | `ennél` | `enne` (trad. `ennék`) | `ennénk` | `ennétek` | `ennének` | `ennélek` |
| CD | `enném` | `ennéd` | `enné` | `ennénk` | `ennétek` | `ennék` | |
| II | `egyek` | `egyél` | `egyen ~ egyék` | `együnk` | `egyetek` | `egyenek` | `egyelek` |
| ID | `egyem` | `edd ~ egyed` | `egye` | `együk` | `egyétek` | `egyék` | |

  - nonfinite: `enni`; `ennem`, `ennie`, `enniük`; `ehet`; `evő`; `evett`; -; converb marginal; `evés`, `ehető`, causative `etet`
- **Output**: table.
- **Examples**: `megeszed`, `megettük`, `egyétek meg`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: slot table
- **Interactions**: `VRB-STEM-011`, `VRB-IK-*`.
- **Sources**: [KVF98 pp. 289, 295, 310], [AkH12 §81], [morphdb grammar irregular eszik], [jepst hvcul.pl eszik]
- **Validation**: `attestation` (`eszel` 9, `esztek` 2, `esszük` 4, `evett` 10, `ettünk` 8, `egyél` 9, `edd` 6, `egyed` 11, `együnk` 8)
- **Confidence**: `normative`

### VRB-IRR-006 iszik
- **Statement**: `iszik` 'drink' parallels `eszik` with back (antiharmonic) harmony and past 3sg indefinite `ivott`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `iszom ~ iszok` | `iszol` | `iszik` | `iszunk` | `isztok` | `isznak` | `iszlak` |
| PD | `iszom` | `iszod` | `issza` | `isszuk` | `isszátok` | `isszák` | |
| XI | `ittam` | `ittál` | `ivott` | `ittunk` | `ittatok` | `ittak` | `ittalak` |
| XD | `ittam` | `ittad` | `itta` | `ittuk` | `ittátok` | `itták` | |
| CI | `innék` (trad. `innám`) | `innál` | `inna` (trad. `innék`) | `innánk` | `innátok` | `innának` | `innálak` |
| CD | `innám` | `innád` | `inná` | `innánk` | `innátok` | `innák` | |
| II | `igyak` | `igyál` | `igyon ~ igyék` | `igyunk` | `igyatok` | `igyanak` | `igyalak` |
| ID | `igyam` | `idd ~ igyad` | `igya` | `igyuk` | `igyátok` | `igyák` | |

  - nonfinite: `inni`; `innom`, `innia`, `inniuk`; `ihat`; `ivó`; `ivott`; -; `iva` (marginal); `ivás`, `iható`, `ittas` (adj), causative `itat`
- **Output**: table.
- **Examples**: `megisszuk`, `ittatok`, `igyatok`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: slot table
- **Interactions**: `VRB-STEM-011`, `VRB-STEM-013`.
- **Sources**: [KVF98 pp. 295, 310-311], [AkH12 §81], [morphdb grammar irregular iszik], [jepst hvcul.pl iszik]
- **Validation**: `attestation` (`iszol` 6, `isztok` 0, `issza` 6, `isszák` 5, `ivott` 10, `ittak` 8, `igyon` 9, `igyatok` 2, `inna` 2)
- **Confidence**: `normative`

### VRB-IRR-007 tesz
- **Statement**: `tesz` 'put, do' is the model of the tesz type (`VRB-STEM-011`).
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `teszek` | `teszel` | `tesz` | `teszünk` | `tesztek` | `tesznek` | `teszlek` |
| PD | `teszem` | `teszed` | `teszi` | `tesszük` | `teszitek` | `teszik` | |
| XI | `tettem` | `tettél` | `tett` | `tettünk` | `tettetek` | `tettek` | `tettelek` |
| XD | `tettem` | `tetted` | `tette` | `tettük` | `tettétek` | `tették` | |
| CI | `tennék` | `tennél` | `tenne` | `tennénk` | `tennétek` | `tennének` | `tennélek` |
| CD | `tenném` | `tennéd` | `tenné` | `tennénk` | `tennétek` | `tennék` | |
| II | `tegyek` | `tegyél ~ tégy` | `tegyen` | `tegyünk` | `tegyetek` | `tegyenek` | `tegyelek` |
| ID | `tegyem` | `tedd ~ tegyed` | `tegye` | `tegyük` | `tegyétek` | `tegyék` | |

  - nonfinite: `tenni`; `tennem`, `tennie`, `tenniük`; `tehet`; `tevő`; `tett`; `teendő`; `téve`; `tevés`, `tehető`, causative `tetet`
- **Output**: table.
- **Examples**: `leteszed`, `feltették`, `tegyétek le`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: slot table
- **Interactions**: `VRB-STEM-011`.
- **Sources**: [AkH12 §42, §81], [morphdb grammar irregular tesz], [jepst hvcul.pl tesz]
- **Validation**: `attestation` (`tett` 20, `tesszük` 15, `tegyél` 11, `tégy` 9, `tedd` 14, `tennie` 13, `téve` 14, `teendő` 12)
- **Confidence**: `normative`

### VRB-IRR-008 vesz
- **Statement**: `vesz` 'take, buy' inflects exactly like `tesz` with `v` for `t`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `veszek` | `veszel` | `vesz` | `veszünk` | `vesztek` | `vesznek` | `veszlek` |
| PD | `veszem` | `veszed` | `veszi` | `vesszük` | `veszitek` | `veszik` | |
| XI | `vettem` | `vettél` | `vett` | `vettünk` | `vettetek` | `vettek` | `vettelek` |
| XD | `vettem` | `vetted` | `vette` | `vettük` | `vettétek` | `vették` | |
| CI | `vennék` | `vennél` | `venne` | `vennénk` | `vennétek` | `vennének` | `vennélek` |
| CD | `venném` | `vennéd` | `venné` | `vennénk` | `vennétek` | `vennék` | |
| II | `vegyek` | `vegyél ~ végy` | `vegyen` | `vegyünk` | `vegyetek` | `vegyenek` | `vegyelek` |
| ID | `vegyem` | `vedd ~ vegyed` | `vegye` | `vegyük` | `vegyétek` | `vegyék` | |

  - nonfinite: `venni`; `vennem`, `vennie`, `venniük`; `vehet`; `vevő`; `vett`; `veendő`; `véve`; `vétel` (noun), `vehető`, causative `vetet`
- **Output**: table.
- **Examples**: `megveszem`, `elvettük`, `vegyél`
- **Exceptions**: `vettek` = `vet` 2pl present.
- **Predictability**: `lexical`
- **Lexical input**: slot table
- **Interactions**: `VRB-STEM-011`, `VRB-PRES-007`.
- **Sources**: [AkH12 §81], [KVF98 p. 310], [morphdb grammar irregular vesz]
- **Validation**: `attestation` (`vett` 19, `veszi` 18, `vesszük` 14, `vegyél` 10, `végy` 7, `vedd` 13, `veendő` 7, `véve` 16)
- **Confidence**: `normative`

### VRB-IRR-009 visz
- **Statement**: `visz` 'carry' follows the tesz type with vowel `i`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `viszek` | `viszel` | `visz` | `viszünk` | `visztek` | `visznek` | `viszlek` |
| PD | `viszem` | `viszed` | `viszi` | `visszük` | `viszitek` | `viszik` | |
| XI | `vittem` | `vittél` | `vitt` | `vittünk` | `vittetek` | `vittek` | `vittelek` |
| XD | `vittem` | `vitted` | `vitte` | `vittük` | `vittétek` | `vitték` | |
| CI | `vinnék` | `vinnél` | `vinne` | `vinnénk` | `vinnétek` | `vinnének` | `vinnélek` |
| CD | `vinném` | `vinnéd` | `vinné` | `vinnénk` | `vinnétek` | `vinnék` | |
| II | `vigyek` | `vigyél` (arch. `vígy`) | `vigyen` | `vigyünk` | `vigyetek` | `vigyenek` | `vigyelek` |
| ID | `vigyem` | `vidd ~ vigyed` | `vigye` | `vigyük` | `vigyétek` | `vigyék` | |

  - nonfinite: `vinni`; `vinnem`, `vinnie`, `vinniük`; `vihet`; `vivő`; `vitt`; -; `víve`; `vitel` (noun), causative `vitet`
- **Output**: table.
- **Examples**: `elviszed`, `hazavittük`, `vigyétek`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: slot table
- **Interactions**: `VRB-STEM-011`.
- **Sources**: [AkH12 §81], [morphdb grammar irregular visz], [jepst hvcul.pl visz]
- **Validation**: `attestation` (`vitt` 13, `visszük` 8, `vigyél` 7, `vidd` 10, `vivő` 7, `vitték` 13)
- **Confidence**: `normative`

### VRB-IRR-010 hisz
- **Statement**: `hisz` 'believe' follows the tesz type but has a long `ggy` imperative and a short indefinite 2sg imperative `higgy`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `hiszek` | `hiszel` | `hisz` | `hiszünk` | `hisztek` | `hisznek` | `hiszlek` |
| PD | `hiszem` | `hiszed` | `hiszi` | `hisszük` | `hiszitek` | `hiszik` | |
| XI | `hittem` | `hittél` | `hitt` | `hittünk` | `hittetek` | `hittek` | `hittelek` |
| XD | `hittem` | `hitted` | `hitte` | `hittük` | `hittétek` | `hitték` | |
| CI | `hinnék` | `hinnél` | `hinne` | `hinnénk` | `hinnétek` | `hinnének` | `hinnélek` |
| CD | `hinném` | `hinnéd` | `hinné` | `hinnénk` | `hinnétek` | `hinnék` | |
| II | `higgyek` | `higgy ~ higgyél` | `higgyen` | `higgyünk` | `higgyetek` | `higgyenek` | `higgyelek` |
| ID | `higgyem` | `hidd ~ higgyed` | `higgye` | `higgyük` | `higgyétek` | `higgyék` | |

  - nonfinite: `hinni`; `hinnem`, `hinnie`, `hinniük`; `hihet`; `hívő ~ hivő`; `hitt`; -; -; `hihető`, causative `hitet` (in `elhitet`)
  - `hisz` is also a discourse particle ('after all'), not a verb form (`SYN`)
- **Output**: table.
- **Examples**: `elhiszem`, `higgyétek el`, `nem hittük`
- **Exceptions**: non-standard `higyj`, `higyjen` (`VRB-IMP-012`).
- **Predictability**: `lexical`
- **Lexical input**: slot table
- **Interactions**: `VRB-STEM-011`.
- **Sources**: [AkH12 §42, §81], [morphdb grammar irregular hisz (+higyj substandard)]
- **Validation**: `attestation` (`hitt` 11, `hiszi` 12, `hisszük` 10, `higgy` 7, `hidd` 11, `hívő` 12)
- **Confidence**: `normative`

### VRB-IRR-011 alszik
- **Statement**: `alszik` 'sleep' is sz-d-v proper: `alsz-` present, `alud-` past/conditional/imperative/infinitive, `al-`/`alv-` in derivation; past 3sg `aludt`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `alszom ~ alszok` | `alszol` | `alszik` | `alszunk` | `alszotok ~ alusztok` | `alszanak ~ alusznak` |
| XI | `aludtam` | `aludtál` | `aludt` | `aludtunk` | `aludtatok` | `aludtak` |
| CI | `aludnék` | `aludnál` | `aludna` | `aludnánk` | `aludnátok` | `aludnának` |
| II | `aludjak` | `aludj ~ aludjál` | `aludjon` | `aludjunk` | `aludjatok` | `aludjanak` |

  - nonfinite: `aludni`; `aludnom`, `aludnia`, `aludniuk`; `alhat ~ aludhat`; `alvó`; `aludt` (`aludt tej`); -; `alva ~ aludva`; `alvás`, causative `altat`
  - definite forms only in the idiom `alussza az igazak álmát` (`alussza`), not generated by default
- **Output**: table.
- **Examples**: `elalszom`, `elaludtunk`, `aludjatok`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: slot table
- **Interactions**: `VRB-STEM-012`.
- **Sources**: [SMNy3 pp. 723, 761], [T04 fn. 19], [KVF98 p. 295], [morphdb grammar irregular alszik], [jepst hvcul.pl alszik]
- **Validation**: `attestation` (`alszom` 8, `alszok` 0, `alszol` 6, `alszanak` 8, `alusznak` 2, `aludtam` 8, `aludt` 10, `aludni` 13, `aludjon` 5, `alvó` 10, `alva` 3)
- **Confidence**: `consensus`

### VRB-IRR-012 fekszik
- **Statement**: `fekszik` 'lie' parallels `alszik` with front rounded/unrounded split: present `feksz-` (front unrounded suffixes), `feküd-` elsewhere (rounded), past 3sg `feküdt`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `fekszem ~ fekszek` | `fekszel` | `fekszik` | `fekszünk` | `fekszetek ~ feküsztök` | `fekszenek ~ feküsznek` |
| XI | `feküdtem` | `feküdtél` | `feküdt` | `feküdtünk` | `feküdtetek` | `feküdtek` |
| CI | `feküdnék` | `feküdnél` | `feküdne` | `feküdnénk` | `feküdnétek` | `feküdnének` |
| II | `feküdjek` | `feküdj ~ feküdjél` | `feküdjön` | `feküdjünk` | `feküdjetek` | `feküdjenek` |

  - nonfinite: `feküdni`; `feküdnöm`, `feküdnie`, `feküdniük`; `fekhet ~ feküdhet`; `fekvő`; `feküdt`; -; `fekve ~ feküdve`; `fekvés`, causative `fektet`
- **Output**: table.
- **Examples**: `lefekszem`, `lefeküdtek`, `feküdjön le`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: slot table
- **Interactions**: `VRB-STEM-012`.
- **Sources**: [SMNy3 p. 723], [KVF98 p. 295], [morphdb grammar irregular fekszik]
- **Validation**: `attestation` (`fekszem` 5, `fekszenek` 8, `feküsznek` 1, `feküdt` 12, `feküdni` 6, `feküdjön` 6, `fekvő` 16, `fekve` 9)
- **Confidence**: `consensus`

### VRB-IRR-013 nyugszik
- **Statement**: `nyugszik` 'rest, set (sun)' is sz-d-v proper with past 3sg `nyugodott`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `nyugszom` | `nyugszol` | `nyugszik` | `nyugszunk` | `nyugszotok` (arch. `nyugosztok`) | `nyugszanak` (arch. `nyugosznak`) |
| XI | `nyugodtam` | `nyugodtál` | `nyugodott` | `nyugodtunk` | `nyugodtatok` | `nyugodtak` |
| CI | `nyugodnék` | `nyugodnál` | `nyugodna` | `nyugodnánk` | `nyugodnátok` | `nyugodnának` |
| II | `nyugodjak` | `nyugodj ~ nyugodjál` | `nyugodjon` | `nyugodjunk` | `nyugodjatok` | `nyugodjanak` |

  - nonfinite: `nyugodni`; `nyugodnom`, `nyugodnia`, `nyugodniuk`; `nyugodhat ~ nyughat`; `nyugvó`; `nyugodott` (verbal) / `nyugodt` (adj. 'calm'); -; `nyugodva`; causative `nyugtat`
- **Output**: table.
- **Examples**: `megnyugszom`, `megnyugodtunk`, `nyugodjon békében`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: slot table
- **Interactions**: `VRB-STEM-012`.
- **Sources**: [KVF98 p. 295], [morphdb grammar irregular nyugszik]
- **Validation**: `attestation` (`nyugszik` 11, `nyugszanak` 6, `nyugodott` 6, `nyugodtak` 6, `nyugodni` 7, `nyugodjon` 5, `nyugvó` 10)
- **Confidence**: `consensus`

### VRB-IRR-014 esküszik
- **Statement**: `esküszik` 'swear' is sz-d-v proper whose `sz` follows a vowel, so present plural forms take no linking vowel; past 3sg `esküdött`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `esküszöm ~ esküszök` | `esküszöl` | `esküszik` | `esküszünk` | `esküsztök` | `esküsznek` |
| XI | `esküdtem` | `esküdtél` | `esküdött` | `esküdtünk` | `esküdtetek` | `esküdtek` |
| CI | `esküdnék` | `esküdnél` | `esküdne` | `esküdnénk` | `esküdnétek` | `esküdnének` |
| II | `esküdjek` | `esküdj ~ esküdjél` | `esküdjön` | `esküdjünk` | `esküdjetek` | `esküdjenek` |

  - nonfinite: `esküdni`; `esküdnöm`, `esküdnie`, `esküdniük`; `esküdhet`; `esküvő` (noun 'wedding'); `esküdött` / `esküdt` (noun 'juror', adj. 'sworn'); -; `esküdve`
  - `alkuszik` inflects identically with back harmony: `alkuszom`, `alkuszol`, `alkusztok`, `alkusznak`, `alkudtam`, `alkudott`, `alkudna`, `alkudjon`, `alkudni`
- **Output**: table.
- **Examples**: `megesküszöm`, `megesküdtek`, `esküdj meg`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: slot table
- **Interactions**: `VRB-STEM-012`.
- **Sources**: [SMNy3 p. 723], [morphdb peeta esküszik sz-d-v_stem], [jepst hvcul.pl esküszik]
- **Validation**: `attestation` (`esküszöm` 7, `esküsznek` 5, `esküdött` 5, `esküdtek` 5, `esküdni` 2, `esküvő` 12, `alkudni` 3)
- **Confidence**: `consensus`

### VRB-IRR-015 haragszik
- **Statement**: `haragszik` 'be angry' is sz-d-v proper with past 3sg `haragudott`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `haragszom ~ haragszok` | `haragszol` | `haragszik` | `haragszunk` | `haragszotok ~ haragusztok` | `haragszanak ~ haragusznak` |
| XI | `haragudtam` | `haragudtál` | `haragudott` | `haragudtunk` | `haragudtatok` | `haragudtak` |
| CI | `haragudnék` | `haragudnál` | `haragudna` | `haragudnánk` | `haragudnátok` | `haragudnának` |
| II | `haragudjak` | `haragudj ~ haragudjál` | `haragudjon` | `haragudjunk` | `haragudjatok` | `haragudjanak` |

  - nonfinite: `haragudni`; `haragudnom`, `haragudnia`, `haragudniuk`; `haragudhat`; `haragvó`; -; -; `haragudva`
- **Output**: table.
- **Examples**: `megharagszol`, `haragudtak`, `ne haragudj`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: slot table
- **Interactions**: `VRB-STEM-012`.
- **Sources**: [morphdb peeta haragszik sz-d-v_stem], [jepst hvcul.pl haragszik]
- **Validation**: `attestation` (`haragszom` 5, `haragszol` 3, `haragszanak` 1, `haragudott` 5, `haragudt` -, `haragudni` 5, `haragudjon` 4, `haragvó` 1)
- **Confidence**: `consensus`

### VRB-IRR-016 dicsekszik and cselekszik (sz-d with doublet)
- **Statement**: `dicsekszik ~ dicsekedik` and `cselekszik ~ cselekedik` combine a present `sz` stem with a full regular `-dik` doublet that supplies all other forms and present variants.
- **Conditions**: `dicsekszik`:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `dicsekszem ~ dicsekedem` | `dicsekszel ~ dicsekedsz` | `dicsekszik ~ dicsekedik` | `dicsekszünk ~ dicsekedünk` | `dicsekszetek ~ dicsekedtek` | `dicsekszenek ~ dicsekednek` |
| XI | `dicsekedtem` | `dicsekedtél` | `dicsekedett` | `dicsekedtünk` | `dicsekedtetek` | `dicsekedtek` |
| CI | `dicsekednék` | `dicsekednél` | `dicsekedne` | `dicsekednénk` | `dicsekednétek` | `dicsekednének` |
| II | `dicsekedjek` | `dicsekedj ~ dicsekedjél` | `dicsekedjen` | `dicsekedjünk` | `dicsekedjetek` | `dicsekedjenek` |

  - nonfinite: `dicsekedni`; `dicsekednem`, `dicsekednie`, `dicsekedniük`; `dicsekedhet`; `dicsekvő`; -; -; `dicsekedve`; `dicsekvés`
  - `cselekszik` identical pattern: `cselekszem ~ cselekedem`, `cselekszel ~ cselekedsz`, `cselekedtem`, `cselekedett`, `cselekedne`, `cselekedjen`, `cselekedni`, `cselekedhet`, `cselekvő`, `cselekvés`, noun `cselekedet`
  - same pattern for the other doublet verbs of `VRB-STEM-012` (`törekszik`, `növekszik`, `melegszik`, `öregszik`, `telepszik`, `verekszik`, `mosakszik`, ...), with `v` participle only where listed
- **Output**: table.
- **Examples**: `eldicsekszik`, `cselekedtek`, `törekedjünk`
- **Exceptions**: present `-dik` doublet 2pl `dicsekedtek` = past 3pl.
- **Predictability**: `lexical`
- **Lexical input**: doublet link, `v` participle flag
- **Interactions**: `VRB-STEM-012`, `VRB-PRES-007`.
- **Sources**: [SMNy3 pp. 723-724], [KVF98 p. 295 (cselekedtem, cselekedett)], [AkH12 §44 (öregedik - öregszik)], [morphdb peeta dicsekedik/dicsekszik, cselekedik/cselekszik]
- **Validation**: `attestation` (`cselekszik` 10, `cselekszem` 4, `cselekedett` 9, `cselekedni` 10, `cselekvő` 8, `dicsekszik` 3, `dicsekedett` 4, `törekszik` 13, `törekedett` 10)
- **Confidence**: `consensus`

### VRB-IRR-017 igyekszik and emlékszik (sz-z with doublet)
- **Statement**: `igyekszik ~ igyekezik` and `emlékszik ~ emlékezik` have a present `sz` stem and a `z` doublet stem for all other forms.
- **Conditions**: `emlékszik`:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `emlékszem ~ emlékezem` | `emlékszel ~ emlékezel` | `emlékszik ~ emlékezik` | `emlékszünk ~ emlékezünk` | `emlékszetek ~ emlékeztek` | `emlékszenek ~ emlékeznek` |
| XI | `emlékeztem` | `emlékeztél` | `emlékezett` | `emlékeztünk` | `emlékeztetek` | `emlékeztek` |
| CI | `emlékeznék` | `emlékeznél` | `emlékezne` | `emlékeznénk` | `emlékeznétek` | `emlékeznének` |
| II | `emlékezzek` | `emlékezz ~ emlékezzél` | `emlékezzen` | `emlékezzünk` | `emlékezzetek` | `emlékezzenek` |

  - nonfinite: `emlékezni`; `emlékeznem`, `emlékeznie`, `emlékezniük`; `emlékezhet`; `emlékező`; -; -; `emlékezve`; nouns `emlékezés`, `emlékezet`
  - `igyekszik`: `igyekszem ~ igyekezem`, `igyekszel ~ igyekezel`, `igyekszünk`, `igyekszenek ~ igyekeznek`, `igyekeztem`, `igyekezett`, `igyekezne`, `igyekezzen`, `igyekezni`, `igyekezhet`, participle `igyekvő`, noun `igyekezet`
  - `gyülekszik ~ gyülekezik` same pattern (`gyülekező`)
  - present `sz` forms strongly preferred for `emlékszik`, `igyekszik`
- **Output**: table.
- **Examples**: `visszaemlékszem`, `igyekeztünk`, `emlékezzetek`
- **Exceptions**: present `-zik` 2pl `emlékeztek` = past 3pl.
- **Predictability**: `lexical`
- **Lexical input**: doublet link
- **Interactions**: `VRB-STEM-012`.
- **Sources**: [AkH12 §44 (igyekezik - igyekszik)], [SMNy3 p. 723 (86)], [jepst hvcul.pl emlékszik, igyekszik]
- **Validation**: `attestation` (`emlékszem` 14, `emlékezem` 5, `emlékszel` 9, `emlékezel` -, `emlékszik` 12, `emlékezik` 9, `igyekszik` 15, `igyekezik` -, `igyekszem` 11, `igyekezett` 13, `igyekvő` 7)
- **Confidence**: `consensus`

### VRB-IRR-018 v-stem verbs: lő, nő, sző, fő, ró, rí, nyű
- **Statement**: Full paradigms of the seven standard v-stem verbs (`VRB-STEM-010`).
- **Conditions**: `lő` (transitive, R):

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `lövök` | `lősz` | `lő` | `lövünk` | `lőtök` | `lőnek` | `lőlek` |
| PD | `lövöm` | `lövöd` | `lövi` | `lőjük` | `lövitek` | `lövik` | |
| XI | `lőttem` | `lőttél` | `lőtt` | `lőttünk` | `lőttetek` | `lőttek` | `lőttelek` |
| XD | `lőttem` | `lőtted` | `lőtte` | `lőttük` | `lőttétek` | `lőtték` | |
| CI | `lőnék` | `lőnél` | `lőne` | `lőnénk` | `lőnétek` | `lőnének` | `lőnélek` |
| CD | `lőném` | `lőnéd` | `lőné` | `lőnénk` | `lőnétek` | `lőnék` | |
| II | `lőjek` | `lőj ~ lőjél` | `lőjön` | `lőjünk` | `lőjetek` | `lőjenek` | `lőjelek` |
| ID | `lőjem` | `lődd ~ lőjed` | `lője` | `lőjük` | `lőjétek` | `lőjék` | |

  - nonfinite `lő`: `lőni`; `lőnöm`, `lőnie`, `lőniük`; `lőhet`; `lövő`; `lőtt`; -; `lőve`; `lövés`, causative `lövet`
  - `sző` (transitive, R): as `lő`: `szövök`, `szősz`, `sző`, `szövünk`, `szőtök`, `szőnek`, `szőlek`; def `szövöm`, `szövöd`, `szövi`, `szőjük`, `szövitek`, `szövik`; `szőttem`, `szőtt`; `szőnék`, `szőne`; `szőjek`, `szőjön`; def `szőjem`, `sződd ~ szőjed`, `szője`; `szőni`, `szőhet`, `szövő`, `szőtt`, `szőve`, `szövés`
  - `nyű` (transitive, R): `nyüvök ~ nyűvök`, `nyűsz`, `nyű`, `nyüvünk`, `nyűtök`, `nyűnek`, `nyűlek`; def `nyüvöm`, `nyüvöd`, `nyüvi`, `nyűjük`, `nyüvitek`, `nyüvik`; `nyűttem`, `nyűtt`; `nyűnék`; `nyűjek`, `nyűj ~ nyűjél`, `nyűjön`; def `nyűjem`, `nyűdd ~ nyűjed`, `nyűje`; `nyűni`, `nyűhet`, `nyüvő`, `nyűtt`, `nyüvendő`, `nyűve`
  - `nő` (intransitive, R): `növök`, `nősz`, `nő`, `növünk`, `nőtök`, `nőnek`; `nőttem`, `nőtt`, `nőttek`; `nőnék`, `nőne`; `nőjek`, `nőj ~ nőjél`, `nőjön`, `nőjünk`, `nőjetek`, `nőjenek`; `nőni`, `nőhet`, `növő`, `nőtt`, `nőve`, `növés`
  - `fő` (intransitive, R): `fövök ~ fővök`, `fősz`, `fő`, `fövünk ~ fővünk`, `főtök`, `főnek`; `főttem`, `főtt`; `főnék`, `főne`; `főjek`, `főjön`; `főni`, `főhet`, `fövő ~ fővő`, `főtt`, `főve ~ fővén`, `fövés`
  - `ró` (transitive, B): `rovok`, `rósz`, `ró`, `rovunk`, `rótok`, `rónak`, `rólak`; def `rovom`, `rovod`, `rója`, `rójuk`, `rójátok`, `róják`; `róttam`, `róttál`, `rótt`, `róttunk`, `róttatok`, `róttak`; def `rótta`, `rótták`; `rónék`, `róna`, def `rónám`, `róná`; `rójak`, `rój ~ rójál`, `rójon`, `rójunk`, `rójatok`, `rójanak`; def `rójam`, `ródd ~ rójad`, `rója`, `rójuk`, `rójátok`, `róják`; `róni`, `róhat`, `rovó`, `rótt`, `róva`, `rovás`
  - `rí` (intransitive, B antiharmonic): `rívok`, `rísz`, `rí`, `rívunk`, `rítok`, `rínak`; `ríttam`, `ríttál`, `rítt`, `ríttunk`, `ríttatok`, `ríttak`; `rínék`, `rínál`, `rína`, `rínánk`, `rínátok`, `rínának`; `ríjak`, `ríj ~ ríjál`, `ríjon`, `ríjunk`, `ríjatok`, `ríjanak`; `ríni`, `ríhat`, `rívó`, `rítt`, `ríva ~ ríván`, `rívás`, causative `ríkat`
- **Output**: tables and lists.
- **Examples**: `lelőtték`, `beleszőtt`, `felnőttek`, `megrótta`
- **Exceptions**: spelling never doubles `sz`, `t` (present), `j` [AkH12 §66].
- **Predictability**: `lexical` (closed)
- **Lexical input**: v-stem table
- **Interactions**: `VRB-STEM-010`.
- **Sources**: [AkH12 §66], [KVF98 p. 291], [SMNy3 pp. 755-765], [Wikt conj `fő`, `nyű`, `rí`], [morphdb grammar V_STEM_VARIANT_FACTORY shortening]
- **Validation**: `attestation` (`lövök` 2, `lősz` 1, `lőnek` 7, `lövöm` 1, `lövi` 6, `lőjük` 2, `lövik` 5, `lőtt` 13, `lőni` 10, `lövő` 7, `lőve` 3, `lődd` 6, `szövi` 5, `szövik` 2, `szőtt` 8, `rója` 5, `rótt` 7, `rónak` 4, `főtt` 13, `főve` 2, `nőtt` 17, `nőjön` 5); rare forms from [Wikt]
- **Confidence**: `normative`

### VRB-IRR-019 hall and vall
- **Statement**: `hall` 'hear' and `vall` 'confess' are `-ll` geminate stems that are heavy (linking vowel) and take `-ott` in all past cells.
- **Conditions**: `hall`:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `hallok` | `hallasz` | `hall` | `hallunk` | `hallotok` | `hallanak` | `hallak` |
| PD | `hallom` | `hallod` | `hallja` | `halljuk` | `halljátok` | `hallják` | |
| XI | `hallottam` | `hallottál` | `hallott` | `hallottunk` | `hallottatok` | `hallottak` | `hallottalak` |
| XD | `hallottam` | `hallottad` | `hallotta` | `hallottuk` | `hallottátok` | `hallották` | |
| CI | `hallanék` | `hallanál` | `hallana` | `hallanánk` | `hallanátok` | `hallanának` | `hallanálak` |
| CD | `hallanám` | `hallanád` | `hallaná` | `hallanánk` | `hallanátok` | `hallanák` | |
| II | `halljak` | `hallj ~ halljál` | `halljon` | `halljunk` | `halljatok` | `halljanak` | `halljalak` |
| ID | `halljam` | `halld ~ halljad` | `hallja` | `halljuk` | `halljátok` | `hallják` | |

  - nonfinite: `hallani`; `hallanom`, `hallania`, `hallaniuk`; `hallhat`; `halló`; `hallott`; -; `hallva`; `hallás`, causative `hallat`
  - `vall`: `vallasz`, `vallotok`, `vallanak`, `vallak`, `vallott`, `vallottam`, `vallana`, `valljon`, `vallani`, `vallhat`, `valló`, `vallva`
- **Output**: table.
- **Examples**: `meghallottuk`, `bevallanám`, `halljátok`
- **Exceptions**: `hallak` from `hall+lak` with triple-letter reduction [AkH12 §93].
- **Predictability**: `lexical`
- **Lexical input**: geminate type
- **Interactions**: `VRB-STEM-006`.
- **Sources**: [T04 §4.1.3 p. 159 (hall-ani)], [KVF98 p. 295], [AkH12 §93], [jepst hvcul.pl -LL]
- **Validation**: `attestation` (`hallasz` 6, `hallanak` 6, `hallani` 14, `hallottam` 14, `hallott` 13, `hallottuk` 9, `vallott` 11, `vallanak` 7, `vallani` 6)
- **Confidence**: `consensus`

### VRB-IRR-020 áll and száll
- **Statement**: `áll` 'stand' and `száll` 'fly, board' are `-ll` stems that are not heavy and take `-t` in all past cells, with elevated `-ott` past doublets and the accepted 2sg doublet `állasz`.
- **Conditions**: `áll`:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `állok` | `állsz ~ állasz` | `áll` | `állunk` | `álltok` | `állnak` | `állak` |
| PD | `állom` | `állod` | `állja` | `álljuk` | `álljátok` | `állják` | |
| XI | `álltam` | `álltál` | `állt ~ állott` | `álltunk` | `álltatok` | `álltak` | `álltalak` |
| XD | `álltam` | `álltad` | `állta` | `álltuk` | `álltátok` | `állták` | |
| CI | `állnék` | `állnál` | `állna` (marginal `?állana`) | `állnánk` | `állnátok` | `állnának` | `állnálak` |
| CD | `állnám` | `állnád` | `állná` | `állnánk` | `állnátok` | `állnák` | |
| II | `álljak` | `állj ~ álljál` | `álljon` | `álljunk` | `álljatok` | `álljanak` | `álljalak` |
| ID | `álljam` | `álld ~ álljad` | `állja` | `álljuk` | `álljátok` | `állják` | |

  - nonfinite: `állni`; `állnom`, `állnia`, `állniuk`; `állhat`; `álló`; `állt ~ állott` (adj. `állott` 'stale'); -; `állva`; `állás`
  - definite uses: `állja a sarat`, `állom a szavam`
  - `száll`: same pattern; `szállt ~ szállott`, `szállnak`, `szállj`, `szállni`, `szállva`; mostly intransitive
- **Output**: table.
- **Examples**: `megállt`, `felszálltunk`, `álljatok meg`
- **Exceptions**: `állott`, `szállott` archaic/elevated in finite use.
- **Predictability**: `lexical`
- **Lexical input**: geminate type
- **Interactions**: `VRB-STEM-006`.
- **Sources**: [AkH12 §44, §64], [T04 §4.1.3 p. 159 (áll-ni)], [jepst hvcul.pl áll, száll]
- **Validation**: `attestation` (`állsz` 10, `állasz` 0, `állnak` 18, `állni` 15, `állt` 19, `állott` 12, `álltam` 10, `állna` 10, `állana` 2, `szállt` 13, `szállott` 6)
- **Confidence**: `normative`

### VRB-IRR-021 lát
- **Statement**: `lát` 'see' is a long-vowel + `t` stem that behaves like a short-vowel `t` stem: no linking vowel, past `-ott` only in 3sg indefinite, imperative `ss`.
- **Conditions**:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `látok` | `látsz` | `lát` | `látunk` | `láttok` | `látnak` | `látlak` |
| PD | `látom` | `látod` | `látja` | `látjuk` | `látjátok` | `látják` | |
| XI | `láttam` | `láttál` | `látott` | `láttunk` | `láttatok` | `láttak` | `láttalak` |
| XD | `láttam` | `láttad` | `látta` | `láttuk` | `láttátok` | `látták` | |
| CI | `látnék` | `látnál` | `látna` | `látnánk` | `látnátok` | `látnának` | `látnálak` |
| CD | `látnám` | `látnád` | `látná` | `látnánk` | `látnátok` | `látnák` | |
| II | `lássak` | `láss ~ lássál` | `lásson` | `lássunk` | `lássatok` | `lássanak` | `lássalak` |
| ID | `lássam` | `lásd ~ lássad` | `lássa` | `lássuk` | `lássátok` | `lássák` | |

  - nonfinite: `látni`; `látnom`, `látnia`, `látniuk`; `láthat`; `látó`; `látott`; `látandó`; `látva`; `látás`, `látható`, causative `láttat`
- **Output**: table.
- **Examples**: `meglátjuk`, `láttalak`, `lásd be`
- **Exceptions**: the whole verb is the exception to T2 (`VRB-STEM-003`); `lót` in `lót-fut` follows it.
- **Predictability**: `lexical` (1 item)
- **Lexical input**: lexical T-class override
- **Interactions**: `VRB-STEM-003`, `VRB-IMP-004`.
- **Sources**: [SMNy3 p. 518 (lát marked as exception)], [T04 (63b) p. 208], [KVF98 p. 295], [AkH12 §77]
- **Validation**: `attestation` (`látsz` 10, `láttok` 5, `látnak` 12, `látni` 17, `láttam` 17, `látott` 17, `látta` expected, `láss` 8, `lásd` 17, `látlak` 6)
- **Confidence**: `normative`

### VRB-IRR-022 kell
- **Statement**: `kell` 'must, be needed' is used impersonally in the 3sg (modal) and personally with a full indefinite paradigm in the lexical sense 'be needed/wanted'; it has no definite forms.
- **Conditions**:
  - impersonal modal: `kell`, `kellett`, `kellene ~ kéne`, `kellett volna`, `kelljen`, with infinitive (`VRB-INF-002`)
  - personal indefinite:

| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `kellek` | `kellesz` | `kell` | `kellünk` | `kelletek` | `kellenek` |
| XI | `kellettem` | `kellettél` | `kellett` | `kellettünk` | `kellettetek` | `kellettek` |
| CI | `kellenék` | `kellenél` | `kellene ~ kéne` | `kellenénk` | `kellenétek` | `kellenének` |
| II | `kelljek` | `kellj ~ kelljél` | `kelljen` | `kelljünk` | `kelljetek` | `kelljenek` |

  - nonfinite: `kelleni` (rare); -; potential marginal; `kellő` (adj. 'adequate'); -; -; -; `kelletlen` (adj.)
- **Output**: table.
- **Examples**: `Kellesz nekem.`; `Mennem kellett.`; `Nem kéne.`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: defective flag (no Def cells)
- **Interactions**: modal syntax (out of scope), `VRB-COND-005`.
- **Sources**: [KVF98 pp. 313-314], [morphdb grammar kell +kéne], [jepst hvcul.pl kell]
- **Validation**: `attestation` (`kell` 27, `kellett` 21, `kellene` 19, `kéne` 15, `kellenek` 11, `kelleni` 4)
- **Confidence**: `consensus`

## PREV

### VRB-PREV-001 Inventory of verbal prefixes
- **Statement**: Verbal prefixes form a closed core set plus a larger set of adverbial and nominal elements that behave like prefixes with specific verbs.
- **Conditions**:
  - core (AkH12 list, fully productive or broadly combinable): `abba`, `agyon`, `alá`, `át`, `be`, `bele`, `benn`, `egybe`, `el`, `ellen`, `elő`, `előre`, `fel ~ föl`, `félbe`, `félre`, `felül ~ fölül`, `fenn ~ fönn`, `hátra`, `haza`, `helyre`, `hozzá`, `ide`, `keresztül`, `ketté`, `ki`, `körül`, `közbe`, `közre`, `külön`, `le`, `meg`, `mellé`, `neki`, `oda`, `össze`, `rá`, `rajta`, `széjjel`, `szembe`, `szerte`, `szét`, `tele`, `tova`, `tovább`, `tönkre`, `túl`, `újjá`, `újra`, `utána`, `végbe`, `végig`, `vissza` (52 with variants counted once)
  - additional adverbial prefixes (morphdb): `alul`, `által`, `arrébb`, `beljebb`, `bent`, `együtt`, `elé`, `feljebb ~ följebb`, `fölé`, `idébb`, `kijjebb`, `kint ~ kinn`, `körbe`, `közé`, `lejjebb`, `mögé`, `odább ~ odébb`, `összébb`, `után`
  - lexically restricted (verb-specific, morphdb `GOV` lists): `jóvá` (`hagy`, `ír`, `tesz`), `közzé` (`tesz`), `létre` (`hoz`, `jön`), `észre` (`vesz`), `végre` (`hajt`), `véghez` (`visz`), `rendbe`, `karban` (`tart`), `nyilván` (`tart`), `tudomásul` (`vesz`), `számba`, `szemügyre` (`vesz`), `cserben` (`hagy`), `utol` (`ér`), `pórul` (`jár`), `helyt` (`áll`), `ellent` (`mond`), `egyet` (`ért`), `kölcsön` (`ad`, `vesz`, `kér`), `pofon` (`vág`, `üt`), `zokon` (`vesz`), `hoppon` (`marad`), `békén`, `nyugton` (`hagy`, `marad`), `szörnyet` (`hal`), `óva` (`int`), `alább` (`hagy`, `száll`), `jót` (`áll`), `földet` (`ér`), `észhez` (`tér`), `szabadjára` (`enged`)
  - pronominal forms of `hozzá`, `neki`, `rá`, `bele`, `utána`, `rajta` with person suffixes (`hozzám`, `rád`) are not prefixes [AkH12 §120e]
- **Output**: prefix lexicon with class core / adverbial / restricted (+ licensed verbs).
- **Examples**: `megold`, `kinyit`, `hazamegy`, `jóváhagy`, `létrehoz`, `tudomásul vesz`
- **Exceptions**: `fel` / `föl`, `fenn` / `fönn`, `felül` / `fölül` are free spelling variants [AkH12 §25].
- **Predictability**: `lexical` (closed core 52; restricted list ~60 with verb licensing)
- **Lexical input**: prefix lexicon
- **Interactions**: `VRB-PREV-002..006`, `DER-PREF`, `DER-COMP`.
- **Sources**: [AkH12 §120 a) list], [morphdb grammar PREVERB_PLUS list and GOV constraints]
- **Validation**: `oracle:hunmorph-foma`, `oracle:emMorph`
- **Confidence**: `normative` (core), `consensus` (restricted)

### VRB-PREV-002 Writing prefixes together or apart
- **Statement**: A prefix is written together with the verb (or non-finite form, or deverbal noun) only when it immediately precedes it; otherwise it is a separate word.
- **Conditions**:
  - immediately before its verb, infinitive, participle or `-Ás` noun: together (`átad`, `felnyit`, `kiállítani`, `megtartó`, `eldobott`, `szembeszállva`, `megtartás`)
  - after the verb: apart (`pihend ki`, `menjetek előre`, `ültünk le`)
  - another word intervenes: apart (`el ne késs`, `le nem tenné`, `közre kell bocsátani`, `haza szabad menni`); also nominalised `be nem avatkozás`, `meg nem értés`
  - prefix before an auxiliary belongs to the following infinitive: apart (`meg fogom nézni`, `el akar menni`, `meg tudták javítani`, `le szokott utazni`) vs lexical verb together (`megfogom a lepkét`, `megtudták a hírt`, `leszokott`)
  - adverbial reading of the same word: apart (`abba tette` vs `abbahagyta`; `ide jött` 'here, not elsewhere' vs `idejött`); personal-pronoun reading apart (`hozzá ment feleségül`)
  - `megvan` 'exists, is found' together vs `meg van töltve` apart; `elleszek` vs `el leszek fáradva`
- **Output**: tokenisation and spelling decision.
- **Examples**: `bemegy` / `nem megy be` / `be akar menni`; `elolvasta` / `olvasd el`
- **Exceptions**: -
- **Predictability**: `rule` (given syntax)
- **Lexical input**: prefix lexicon
- **Interactions**: -
- **Sources**: [AkH12 §120 a-c, e]
- **Validation**: `goldens`
- **Confidence**: `normative`

### VRB-PREV-003 Repeated and paired prefixes
- **Statement**: A repeated prefix (iterative) is hyphenated and written together with the verb; two different, typically opposite prefixes are hyphenated to each other and written apart from the verb.
- **Conditions**:
  - repetition: `ki-kinéz`, `le-leereszt`, `meg-megállt`, `össze-összevesznek`, `elő-előtűnő`, `vissza-visszatérve`, `vissza-visszatérés`
  - pair: `ki-be járkál`, `le-föl sétál`, `ide-oda tekint`, `oda-vissza utazni`, `előre-hátra pillantva`, `le-föl sétálás`
  - analysis: repeated prefix -> lemma `kinéz` + feature `Aspect=Iter` (analysis layer); pair -> two prefixes coordinated, lemma of verb without prefix
- **Output**: spelling and analysis as above.
- **Examples**: `meg-megáll`, `fel-felnéz`, `ki-be jár`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: prefix lexicon
- **Interactions**: `ORT-TOG-028`, `ORT-HYPH-008`.
- **Sources**: [AkH12 §120 d), §141 c)]
- **Validation**: `goldens`
- **Confidence**: `normative`

### VRB-PREV-004 Lemma convention and analysis of prefixed verbs
- **Statement**: The lemma of a prefixed verb includes the prefix (`megold`); when the prefix is separated, the verb token keeps its bare morphological lemma and the prefixed lemma is assigned at the syntactic level by re-attachment.
- **Conditions**:
  - joined token `megoldotta`: morphological analysis `meg` + `old` + `ott` + `a`; lemma `megold`; `PREV=meg` recorded
  - separated `oldotta meg`: token `oldotta` lemma `old`; token `meg` POS `PART`/`ADV` with relation `compound:prt` (UD); syntactic lemma `megold`
  - ambiguous strings resolved by lexicon (`megint` adverb vs `meg+int` verb; `elég` adjective vs `el+ég`; `behat` vs `be+hat`); the prefixed reading requires the verb to be listed or productively combinable with the prefix
  - lexicalised non-compositional prefix verbs are separate lexemes (`megél`, `felel`, `kitart`, `elvisel`)
- **Output**: lemma and prefix feature.
- **Examples**: `elmentünk` -> `elmegy`; `menjünk el` -> `elmegy`; `felhívtalak` -> `felhív`
- **Exceptions**: `felel`, `kiabál` look prefixed but are simple verbs; `megvan` has its own lemma with the `van` paradigm (`megvagyok`, `megvolt`, `meglesz`).
- **Predictability**: `rule` + `lexical` (ambiguity list)
- **Lexical input**: prefixed lexemes, ambiguity list
- **Interactions**: `VRB-MODEL-004`, word order (out of scope).
- **Sources**: [morphdb grammar PREVERB (LEMMA with prefix)], [UD-hu relation compound:prt], [AkH12 §120 c)]
- **Validation**: `oracle:emMorph`, `goldens`
- **Confidence**: `consensus`

### VRB-PREV-005 Inflectional transparency of prefixes
- **Statement**: Prefixes do not affect any inflectional property of the verb: harmony, stem class, `-ik`, irregularity and conjugation all come from the base verb.
- **Conditions**:
  - harmony from the verb: `elolvas` -> `elolvasom` (back), `meglát` -> `meglátod`; the prefix vowel is ignored
  - irregular paradigms inherited: `elmegy` -> `elmész`, `elmentem`; `bejön` -> `bejössz`, `gyere be`
  - transitivity may change with the prefix (`jár` intr. vs `bejár` tr. 'travel all over'; `dolgozik` vs `feldolgoz` non-`-ik` transitive): stored per prefixed lexeme
  - hyphenation at prefix boundary (`fel-ad`, `meg-öl`, `szét-üt`) [AkH12 §234]
- **Output**: paradigm = base paradigm with prefix concatenated.
- **Examples**: `megeszik` -> `megeszem`, `megettem`, `egyem meg`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: transitivity of prefixed lexeme
- **Interactions**: `VRB-MODEL-001`, `VRB-PREV-004`.
- **Sources**: [SMNy3 p. 514], [AkH12 §234], [morphdb grammar IRREGULAR_VERB preverb LEMMA "+"]
- **Validation**: `goldens`
- **Confidence**: `consensus`

### VRB-PREV-006 Separability and word order interface
- **Statement**: Prefix placement is syntactic; morphology must accept a prefix both attached and as a separate token in any of the positions defined by word order (out of scope).
- **Conditions** (summary for tokenisation, details in word order (out of scope)):
  - neutral declarative: prefix + verb (`Péter elolvasta a könyvet.`)
  - focus, negation, question word, imperative, many adverbs: verb + prefix (`Péter olvasta el.`, `Nem olvasta el.`, `Olvasd el!`)
  - auxiliary (`fog`, `akar`, `tud`, `kell`, `szokott`, `kezd`...): prefix climbs before the auxiliary (`el fogja olvasni`) or stays with the infinitive after it (`fogja elolvasni` under focus)
  - progressive reading: verb + prefix (`éppen ment be`)
- **Output**: pointer.
- **Examples**: `Be kell mennem.`; `Nem mentem be.`; `Menj be!`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [KVF98 pp. 21-23, 263, 312], [SMNy3 p. 514]
- **Validation**: `goldens`
- **Confidence**: `consensus`

## PARA

### VRB-PARA-001 Reference paradigms as goldens
- **Statement**: The tables below are the golden paradigms for one exemplar of every inflection class; every cell must be reproduced exactly by the generator, and variants are listed with the preferred form first.
- **Conditions**:
  - row codes as in `IRR`; nonfinite line: infinitive; personal infinitive 1sg / 3sg / 3pl; potential 3sg; `-Ó`; past participle; `-AndÓ`; `-vA`
  - intransitive verbs list definite rows as `-` (not generated)
  - exemplars and classes: `vár` (B SON), `kér` (F SON), `ül` (R SON), `olvas` (B SIB), `néz` (F SIB), `köt` (R T1), `lát` (B T2 exception, `VRB-IRR-021`), `tanít` (B T4), `fest` (F T6s), `tart` (B T6c), `mond` (B CC optional past), `küld` (R CC no-vowel past), `ad` (B OBS), `szalad` (B suffixal `-ad`), `dolgozik` (B `-ik` SIB), `eszik` (`VRB-IRR-005`), `alszik` (`VRB-IRR-011`), `forog` (B epenthetic), `ugrik` (B epenthetic `-ik`), `hív` (B antiharmonic OBS), `ír` (B antiharmonic SON), `fut` (B T1), `üt` (R T1), `fűt` (R T2 heavy), `lő` (`VRB-IRR-018`), `jön` (`VRB-IRR-004`), `megy` (`VRB-IRR-003`), `van` (`VRB-IRR-001`)
- **Output**: tables.
- **Examples**: see tables
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: class records
- **Interactions**: all rules in this chapter.
- **Sources**: [KVF98 pp. 289-311], [SMNy3 pp. 514-523], [AkH12 §38-§81], [Wikt conj tables], [ATT]
- **Validation**: `goldens`, `attestation` (cells spot-checked in the rules above)
- **Confidence**: `normative` (standard cells), `tendency` (variant order)

#### vár
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `várok` | `vársz` | `vár` | `várunk` | `vártok` | `várnak` | `várlak` |
| PD | `várom` | `várod` | `várja` | `várjuk` | `várjátok` | `várják` | |
| XI | `vártam` | `vártál` | `várt` | `vártunk` | `vártatok` | `vártak` | `vártalak` |
| XD | `vártam` | `vártad` | `várta` | `vártuk` | `vártátok` | `várták` | |
| CI | `várnék` | `várnál` | `várna` | `várnánk` | `várnátok` | `várnának` | `várnálak` |
| CD | `várnám` | `várnád` | `várná` | `várnánk` | `várnátok` | `várnák` | |
| II | `várjak` | `várj ~ várjál` | `várjon` | `várjunk` | `várjatok` | `várjanak` | `várjalak` |
| ID | `várjam` | `várd ~ várjad` | `várja` | `várjuk` | `várjátok` | `várják` | |

Nonfinite: `várni`; `várnom`, `várnia`, `várniuk`; `várhat`; `váró`; `várt`; `várandó`; `várva`

#### kér
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `kérek` | `kérsz` | `kér` | `kérünk` | `kértek` | `kérnek` | `kérlek` |
| PD | `kérem` | `kéred` | `kéri` | `kérjük` | `kéritek` | `kérik` | |
| XI | `kértem` | `kértél` | `kért` | `kértünk` | `kértetek` | `kértek` | `kértelek` |
| XD | `kértem` | `kérted` | `kérte` | `kértük` | `kértétek` | `kérték` | |
| CI | `kérnék` | `kérnél` | `kérne` | `kérnénk` | `kérnétek` | `kérnének` | `kérnélek` |
| CD | `kérném` | `kérnéd` | `kérné` | `kérnénk` | `kérnétek` | `kérnék` | |
| II | `kérjek` | `kérj ~ kérjél` | `kérjen` | `kérjünk` | `kérjetek` | `kérjenek` | `kérjelek` |
| ID | `kérjem` | `kérd ~ kérjed` | `kérje` | `kérjük` | `kérjétek` | `kérjék` | |

Nonfinite: `kérni`; `kérnem`, `kérnie`, `kérniük`; `kérhet`; `kérő`; `kért`; `kérendő`; `kérve`

#### ül
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `ülök` | `ülsz` | `ül` | `ülünk` | `ültök` | `ülnek` | `üllek` |
| PD | `ülöm` | `ülöd` | `üli` | `üljük` | `ülitek` | `ülik` | |
| XI | `ültem` | `ültél` | `ült` | `ültünk` | `ültetek` | `ültek` | `ültelek` |
| XD | `ültem` | `ülted` | `ülte` | `ültük` | `ültétek` | `ülték` | |
| CI | `ülnék` | `ülnél` | `ülne` | `ülnénk` | `ülnétek` | `ülnének` | `ülnélek` |
| CD | `ülném` | `ülnéd` | `ülné` | `ülnénk` | `ülnétek` | `ülnék` | |
| II | `üljek` | `ülj ~ üljél` | `üljön` | `üljünk` | `üljetek` | `üljenek` | `üljelek` |
| ID | `üljem` | `üld ~ üljed` | `ülje` | `üljük` | `üljétek` | `üljék` | |

Nonfinite: `ülni`; `ülnöm`, `ülnie`, `ülniük`; `ülhet`; `ülő`; `ült`; -; `ülve`. Definite rows are marginal (`üli a lovat`, `üli a torát`).

#### olvas
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `olvasok` | `olvasol` | `olvas` | `olvasunk` | `olvastok` | `olvasnak` | `olvaslak` |
| PD | `olvasom` | `olvasod` | `olvassa` | `olvassuk` | `olvassátok` | `olvassák` | |
| XI | `olvastam` | `olvastál` | `olvasott` | `olvastunk` | `olvastatok` | `olvastak` | `olvastalak` |
| XD | `olvastam` | `olvastad` | `olvasta` | `olvastuk` | `olvastátok` | `olvasták` | |
| CI | `olvasnék` | `olvasnál` | `olvasna` | `olvasnánk` | `olvasnátok` | `olvasnának` | `olvasnálak` |
| CD | `olvasnám` | `olvasnád` | `olvasná` | `olvasnánk` | `olvasnátok` | `olvasnák` | |
| II | `olvassak` | `olvass ~ olvassál` | `olvasson` | `olvassunk` | `olvassatok` | `olvassanak` | `olvassalak` |
| ID | `olvassam` | `olvasd ~ olvassad` | `olvassa` | `olvassuk` | `olvassátok` | `olvassák` | |

Nonfinite: `olvasni`; `olvasnom`, `olvasnia`, `olvasniuk`; `olvashat`; `olvasó`; `olvasott`; `olvasandó`; `olvasva`

#### néz
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `nézek` | `nézel` | `néz` | `nézünk` | `néztek` | `néznek` | `nézlek` |
| PD | `nézem` | `nézed` | `nézi` | `nézzük` | `nézitek` | `nézik` | |
| XI | `néztem` | `néztél` | `nézett` | `néztünk` | `néztetek` | `néztek` | `néztelek` |
| XD | `néztem` | `nézted` | `nézte` | `néztük` | `néztétek` | `nézték` | |
| CI | `néznék` | `néznél` | `nézne` | `néznénk` | `néznétek` | `néznének` | `néznélek` |
| CD | `nézném` | `néznéd` | `nézné` | `néznénk` | `néznétek` | `néznék` | |
| II | `nézzek` | `nézz ~ nézzél` | `nézzen` | `nézzünk` | `nézzetek` | `nézzenek` | `nézzelek` |
| ID | `nézzem` | `nézd ~ nézzed` | `nézze` | `nézzük` | `nézzétek` | `nézzék` | |

Nonfinite: `nézni`; `néznem`, `néznie`, `nézniük`; `nézhet`; `néző`; `nézett`; `nézendő`; `nézve`

#### köt
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `kötök` | `kötsz` | `köt` | `kötünk` | `köttök` | `kötnek` | `kötlek` |
| PD | `kötöm` | `kötöd` | `köti` | `kötjük` | `kötitek` | `kötik` | |
| XI | `kötöttem` | `kötöttél` | `kötött` | `kötöttünk` | `kötöttetek` | `kötöttek` | `kötöttelek` |
| XD | `kötöttem` | `kötötted` | `kötötte` | `kötöttük` | `kötöttétek` | `kötötték` | |
| CI | `kötnék` | `kötnél` | `kötne` | `kötnénk` | `kötnétek` | `kötnének` | `kötnélek` |
| CD | `kötném` | `kötnéd` | `kötné` | `kötnénk` | `kötnétek` | `kötnék` | |
| II | `kössek` | `köss ~ kössél` | `kössön` | `kössünk` | `kössetek` | `kössenek` | `kösselek` |
| ID | `kössem` | `kösd ~ kössed` | `kösse` | `kössük` | `kössétek` | `kössék` | |

Nonfinite: `kötni`; `kötnöm`, `kötnie`, `kötniük`; `köthet`; `kötő`; `kötött`; `kötendő`; `kötve`

#### lát
See `VRB-IRR-021`.

#### tanít
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `tanítok` | `tanítasz` | `tanít` | `tanítunk` | `tanítotok` | `tanítanak` | `tanítalak` |
| PD | `tanítom` | `tanítod` | `tanítja` | `tanítjuk` | `tanítjátok` | `tanítják` | |
| XI | `tanítottam` | `tanítottál` | `tanított` | `tanítottunk` | `tanítottatok` | `tanítottak` | `tanítottalak` |
| XD | `tanítottam` | `tanítottad` | `tanította` | `tanítottuk` | `tanítottátok` | `tanították` | |
| CI | `tanítanék` | `tanítanál` | `tanítana` | `tanítanánk` | `tanítanátok` | `tanítanának` | `tanítanálak` |
| CD | `tanítanám` | `tanítanád` | `tanítaná` | `tanítanánk` | `tanítanátok` | `tanítanák` | |
| II | `tanítsak` | `taníts ~ tanítsál` | `tanítson` | `tanítsunk` | `tanítsatok` | `tanítsanak` | `tanítsalak` |
| ID | `tanítsam` | `tanítsd ~ tanítsad` | `tanítsa` | `tanítsuk` | `tanítsátok` | `tanítsák` | |

Nonfinite: `tanítani`; `tanítanom`, `tanítania`, `tanítaniuk`; `taníthat`; `tanító`; `tanított`; `tanítandó`; `tanítva`

#### fest
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `festek` | `festesz` | `fest` | `festünk` | `festetek` | `festenek` | `festelek` |
| PD | `festem` | `fested` | `festi` | `festjük` | `festitek` | `festik` | |
| XI | `festettem` | `festettél` | `festett` | `festettünk` | `festettetek` | `festettek` | `festettelek` |
| XD | `festettem` | `festetted` | `festette` | `festettük` | `festettétek` | `festették` | |
| CI | `festenék` | `festenél` | `festene` | `festenénk` | `festenétek` | `festenének` | `festenélek` |
| CD | `festeném` | `festenéd` | `festené` | `festenénk` | `festenétek` | `festenék` | |
| II | `fessek` | `fess ~ fessél` | `fessen` | `fessünk` | `fessetek` | `fessenek` | `fesselek` |
| ID | `fessem` | `fesd ~ fessed` | `fesse` | `fessük` | `fessétek` | `fessék` | |

Nonfinite: `festeni`; `festenem`, `festenie`, `festeniük`; `festhet`; `festő`; `festett`; `festendő`; `festve`

#### tart
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `tartok` | `tartasz` | `tart` | `tartunk` | `tartotok` | `tartanak` | `tartalak` |
| PD | `tartom` | `tartod` | `tartja` | `tartjuk` | `tartjátok` | `tartják` | |
| XI | `tartottam` | `tartottál` | `tartott` | `tartottunk` | `tartottatok` | `tartottak` | `tartottalak` |
| XD | `tartottam` | `tartottad` | `tartotta` | `tartottuk` | `tartottátok` | `tartották` | |
| CI | `tartanék` | `tartanál` | `tartana` | `tartanánk` | `tartanátok` | `tartanának` | `tartanálak` |
| CD | `tartanám` | `tartanád` | `tartaná` | `tartanánk` | `tartanátok` | `tartanák` | |
| II | `tartsak` | `tarts ~ tartsál` | `tartson` | `tartsunk` | `tartsatok` | `tartsanak` | `tartsalak` |
| ID | `tartsam` | `tartsd ~ tartsad` | `tartsa` | `tartsuk` | `tartsátok` | `tartsák` | |

Nonfinite: `tartani`; `tartanom`, `tartania`, `tartaniuk`; `tarthat`; `tartó`; `tartott`; `tartandó`; `tartva`

#### mond
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `mondok` | `mondasz` | `mond` | `mondunk` | `mondotok ~ mondtok` | `mondanak` | `mondalak ~ mondlak` |
| PD | `mondom` | `mondod` | `mondja` | `mondjuk` | `mondjátok` | `mondják` | |
| XI | `mondtam ~ mondottam` | `mondtál ~ mondottál` | `mondott` | `mondtunk ~ mondottunk` | `mondtatok ~ mondottatok` | `mondtak ~ mondottak` | `mondtalak ~ mondottalak` |
| XD | `mondtam ~ mondottam` | `mondtad ~ mondottad` | `mondta ~ mondotta` | `mondtuk ~ mondottuk` | `mondtátok ~ mondottátok` | `mondták ~ mondották` | |
| CI | `mondanék` | `mondanál` | `mondana` | `mondanánk` | `mondanátok` | `mondanának` | `mondanálak` |
| CD | `mondanám` | `mondanád` | `mondaná` | `mondanánk` | `mondanátok` | `mondanák` | |
| II | `mondjak` | `mondj ~ mondjál` | `mondjon` | `mondjunk` | `mondjatok` | `mondjanak` | `mondjalak` |
| ID | `mondjam` | `mondd ~ mondjad` | `mondja` | `mondjuk` | `mondjátok` | `mondják` | |

Nonfinite: `mondani`; `mondanom`, `mondania`, `mondaniuk`; `mondhat`; `mondó`; `mondott`; `mondandó`; `mondva ~ mondván`

#### küld
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `küldök` | `küldesz` | `küld` | `küldünk` | `küldötök` | `küldenek` | `küldelek ~ küldlek` |
| PD | `küldöm` | `küldöd` | `küldi` | `küldjük` | `külditek` | `küldik` | |
| XI | `küldtem` | `küldtél` | `küldött` | `küldtünk` | `küldtetek` | `küldtek` | `küldtelek` |
| XD | `küldtem` | `küldted` | `küldte` | `küldtük` | `küldtétek` | `küldték` | |
| CI | `küldenék` | `küldenél` | `küldene` | `küldenénk` | `küldenétek` | `küldenének` | `küldenélek` |
| CD | `küldeném` | `küldenéd` | `küldené` | `küldenénk` | `küldenétek` | `küldenék` | |
| II | `küldjek` | `küldj ~ küldjél` | `küldjön` | `küldjünk` | `küldjetek` | `küldjenek` | `küldjelek` |
| ID | `küldjem` | `küldd ~ küldjed` | `küldje` | `küldjük` | `küldjétek` | `küldjék` | |

Nonfinite: `küldeni`; `küldenem`, `küldenie`, `küldeniük`; `küldhet`; `küldő`; `küldött`; `küldendő`; `küldve`

#### ad
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `adok` | `adsz` | `ad` | `adunk` | `adtok` | `adnak` | `adlak` |
| PD | `adom` | `adod` | `adja` | `adjuk` | `adjátok` | `adják` | |
| XI | `adtam` | `adtál` | `adott` | `adtunk` | `adtatok` | `adtak` | `adtalak` |
| XD | `adtam` | `adtad` | `adta` | `adtuk` | `adtátok` | `adták` | |
| CI | `adnék` | `adnál` | `adna` | `adnánk` | `adnátok` | `adnának` | `adnálak` |
| CD | `adnám` | `adnád` | `adná` | `adnánk` | `adnátok` | `adnák` | |
| II | `adjak` | `adj ~ adjál` | `adjon` | `adjunk` | `adjatok` | `adjanak` | `adjalak` |
| ID | `adjam` | `add ~ adjad` | `adja` | `adjuk` | `adjátok` | `adják` | |

Nonfinite: `adni`; `adnom`, `adnia`, `adniuk`; `adhat`; `adó`; `adott`; `adandó`; `adva`

#### szalad
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `szaladok` | `szaladsz` | `szalad` | `szaladunk` | `szaladtok` | `szaladnak` |
| PD | - | - | - | - | - | - |
| XI | `szaladtam` | `szaladtál` | `szaladt` | `szaladtunk` | `szaladtatok` | `szaladtak` |
| CI | `szaladnék` | `szaladnál` | `szaladna` | `szaladnánk` | `szaladnátok` | `szaladnának` |
| II | `szaladjak` | `szaladj ~ szaladjál` | `szaladjon` | `szaladjunk` | `szaladjatok` | `szaladjanak` |

Nonfinite: `szaladni`; `szaladnom`, `szaladnia`, `szaladniuk`; `szaladhat`; `szaladó`; -; -; `szaladva`

#### dolgozik
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `dolgozom ~ dolgozok` | `dolgozol` | `dolgozik` | `dolgozunk` | `dolgoztok` | `dolgoznak` |
| PD | - | - | - | - | - | - |
| XI | `dolgoztam` | `dolgoztál` | `dolgozott` | `dolgoztunk` | `dolgoztatok` | `dolgoztak` |
| CI | `dolgoznék` (trad. `dolgoznám`) | `dolgoznál` | `dolgozna` (trad. `dolgoznék`) | `dolgoznánk` | `dolgoznátok` | `dolgoznának` |
| II | `dolgozzak` | `dolgozz ~ dolgozzál` | `dolgozzon ~ dolgozzék` | `dolgozzunk` | `dolgozzatok` | `dolgozzanak` |

Nonfinite: `dolgozni`; `dolgoznom`, `dolgoznia`, `dolgozniuk`; `dolgozhat`; `dolgozó`; -; -; `dolgozva`

#### eszik
See `VRB-IRR-005`.

#### alszik
See `VRB-IRR-011`.

#### forog
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `forgok` | `forogsz` | `forog` | `forgunk` | `forogtok` | `forognak` |
| PD | - | - | - | - | - | - |
| XI | `forogtam` | `forogtál` | `forgott` | `forogtunk` | `forogtatok` | `forogtak` |
| CI | `forognék` | `forognál` | `forogna` | `forognánk` | `forognátok` | `forognának` |
| II | `forogjak` | `forogj ~ forogjál` | `forogjon` | `forogjunk` | `forogjatok` | `forogjanak` |

Nonfinite: `forogni`; `forognom`, `forognia`, `forogniuk`; `foroghat`; `forgó`; -; -; `forogva`; noun `forgás`

#### ugrik
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `ugrom ~ ugrok` | `ugrasz ~ ugorsz` | `ugrik` | `ugrunk` | `ugrotok ~ ugortok` | `ugranak ~ ugornak` |
| PD | - | - | - | - | - | - |
| XI | `ugrottam ~ ugortam` | `ugrottál ~ ugortál` | `ugrott` | `ugrottunk ~ ugortunk` | `ugrottatok ~ ugortatok` | `ugrottak ~ ugortak` |
| CI | `ugranék ~ ugornék` | `ugranál ~ ugornál` | `ugrana ~ ugorna` | `ugranánk ~ ugornánk` | `ugranátok ~ ugornátok` | `ugranának ~ ugornának` |
| II | `ugorjak` | `ugorj ~ ugorjál` | `ugorjon` | `ugorjunk` | `ugorjatok` | `ugorjanak` |

Nonfinite: `ugrani ~ ugorni`; `ugranom ~ ugornom`, `ugrania ~ ugornia`, `ugraniuk ~ ugorniuk`; `ugorhat`; `ugró`; -; -; `ugorva`; noun `ugrás`

#### hív
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `hívok` | `hívsz` | `hív` | `hívunk` | `hívtok` | `hívnak` | `hívlak` |
| PD | `hívom` | `hívod` | `hívja` | `hívjuk` | `hívjátok` | `hívják` | |
| XI | `hívtam` | `hívtál` | `hívott` | `hívtunk` | `hívtatok` | `hívtak` | `hívtalak` |
| XD | `hívtam` | `hívtad` | `hívta` | `hívtuk` | `hívtátok` | `hívták` | |
| CI | `hívnék` | `hívnál` | `hívna` | `hívnánk` | `hívnátok` | `hívnának` | `hívnálak` |
| CD | `hívnám` | `hívnád` | `hívná` | `hívnánk` | `hívnátok` | `hívnák` | |
| II | `hívjak` | `hívj ~ hívjál` | `hívjon` | `hívjunk` | `hívjatok` | `hívjanak` | `hívjalak` |
| ID | `hívjam` | `hívd ~ hívjad` | `hívja` | `hívjuk` | `hívjátok` | `hívják` | |

Nonfinite: `hívni`; `hívnom`, `hívnia`, `hívniuk`; `hívhat`; `hívó`; `hívott`; `hívandó`; `hívva`

#### ír
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `írok` | `írsz` | `ír` | `írunk` | `írtok` | `írnak` | `írlak` |
| PD | `írom` | `írod` | `írja` | `írjuk` | `írjátok` | `írják` | |
| XI | `írtam` | `írtál` | `írt` | `írtunk` | `írtatok` | `írtak` | `írtalak` |
| XD | `írtam` | `írtad` | `írta` | `írtuk` | `írtátok` | `írták` | |
| CI | `írnék` | `írnál` | `írna` | `írnánk` | `írnátok` | `írnának` | `írnálak` |
| CD | `írnám` | `írnád` | `írná` | `írnánk` | `írnátok` | `írnák` | |
| II | `írjak` | `írj ~ írjál` | `írjon` | `írjunk` | `írjatok` | `írjanak` | `írjalak` |
| ID | `írjam` | `írd ~ írjad` | `írja` | `írjuk` | `írjátok` | `írják` | |

Nonfinite: `írni`; `írnom`, `írnia`, `írniuk`; `írhat`; `író`; `írt ~ írott`; `írandó`; `írva`

#### fut
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| PI | `futok` | `futsz` | `fut` | `futunk` | `futtok` | `futnak` |
| PD | `futom` | `futod` | `futja` | `futjuk` | `futjátok` | `futják` |
| XI | `futottam` | `futottál` | `futott` | `futottunk` | `futottatok` | `futottak` |
| XD | `futottam` | `futottad` | `futotta` | `futottuk` | `futottátok` | `futották` |
| CI | `futnék` | `futnál` | `futna` | `futnánk` | `futnátok` | `futnának` |
| CD | `futnám` | `futnád` | `futná` | `futnánk` | `futnátok` | `futnák` |
| II | `fussak` | `fuss ~ fussál` | `fusson` | `fussunk` | `fussatok` | `fussanak` |
| ID | `fussam` | `fusd ~ fussad` | `fussa` | `fussuk` | `fussátok` | `fussák` |

Nonfinite: `futni`; `futnom`, `futnia`, `futniuk`; `futhat`; `futó`; `futott`; -; `futva`. Definite rows occur with cognate or measure objects (`futja a köröket`, `futja az erejéből`).

#### üt
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `ütök` | `ütsz` | `üt` | `ütünk` | `üttök` | `ütnek` | `ütlek` |
| PD | `ütöm` | `ütöd` | `üti` | `ütjük` | `ütitek` | `ütik` | |
| XI | `ütöttem` | `ütöttél` | `ütött` | `ütöttünk` | `ütöttetek` | `ütöttek` | `ütöttelek` |
| XD | `ütöttem` | `ütötted` | `ütötte` | `ütöttük` | `ütöttétek` | `ütötték` | |
| CI | `ütnék` | `ütnél` | `ütne` | `ütnénk` | `ütnétek` | `ütnének` | `ütnélek` |
| CD | `ütném` | `ütnéd` | `ütné` | `ütnénk` | `ütnétek` | `ütnék` | |
| II | `üssek` | `üss ~ üssél` | `üssön` | `üssünk` | `üssetek` | `üssenek` | `üsselek` |
| ID | `üssem` | `üsd ~ üssed` | `üsse` | `üssük` | `üssétek` | `üssék` | |

Nonfinite: `ütni`; `ütnöm`, `ütnie`, `ütniük`; `üthet`; `ütő`; `ütött`; -; `ütve`

#### fűt
| | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | 1sg>2 |
|---|---|---|---|---|---|---|---|
| PI | `fűtök` | `fűtesz` | `fűt` | `fűtünk` | `fűtötök` | `fűtenek` | `fűtelek` |
| PD | `fűtöm` | `fűtöd` | `fűti` | `fűtjük` | `fűtitek` | `fűtik` | |
| XI | `fűtöttem` | `fűtöttél` | `fűtött` | `fűtöttünk` | `fűtöttetek` | `fűtöttek` | `fűtöttelek` |
| XD | `fűtöttem` | `fűtötted` | `fűtötte` | `fűtöttük` | `fűtöttétek` | `fűtötték` | |
| CI | `fűtenék` | `fűtenél` | `fűtene` | `fűtenénk` | `fűtenétek` | `fűtenének` | `fűtenélek` |
| CD | `fűteném` | `fűtenéd` | `fűtené` | `fűtenénk` | `fűtenétek` | `fűtenék` | |
| II | `fűtsek` | `fűts ~ fűtsél` | `fűtsön` | `fűtsünk` | `fűtsetek` | `fűtsenek` | `fűtselek` |
| ID | `fűtsem` | `fűtsd ~ fűtsed` | `fűtse` | `fűtsük` | `fűtsétek` | `fűtsék` | |

Nonfinite: `fűteni`; `fűtenem`, `fűtenie`, `fűteniük`; `fűthet`; `fűtő`; `fűtött`; `fűtendő`; `fűtve`

#### lő
See `VRB-IRR-018`.

#### jön
See `VRB-IRR-004`.

#### megy
See `VRB-IRR-003`.

#### van
See `VRB-IRR-001`.

## Open questions
1. `Mood=Pot` stacking: UD Szeged uses `Mood=Cnd,Pot` / `Imp,Pot`; confirm that the treebank encodes indicative potential as bare `Mood=Pot` and not `Ind,Pot` before fixing the serializer (`VRB-MODEL-002`).
2. UniMorph encoding of `-lak/-lek` (`Person[obj]=2`): the exact UniMorph hun tag (argument-marking feature) was not verified; mapping in [glossary.md](glossary.md) is provisional.
3. CC-final verbs, non-3sg past (`VRB-PAST-007`): item lists for `hord`, `áld`, `told`, `fing`, `rajong`, `zeng`, `mereng`, `fedd` rest on T04, KVF98 and thin corpus counts; a larger corpus (MNSZ2) study is needed, especially for `-Cd`, `-ng` verbs not listed.
4. Preference between `mondotok ~ mondtok`, `mondalak ~ mondlak`, `küldelek ~ küldlek` and `?mondsz` (`VRB-LINK-008/010`) is unquantified.
5. Resolved: a monosyllable in vowel + `dz` is heavy and takes `-Ott-` in every past cell (`VRB-LINK-002`, `VRB-PAST-005`); 2sg `edzel` follows the sibilant rule (`VRB-LINK-003`).
6. Epenthetic `-ik` verbs: per-item preference between syncopated + linking vowel and full stem (`ugrani ~ ugorni`, `fürdeni ~ fürödni`, `hiányozni`) is only partly quantified (`VRB-STEM-008`).
7. `terem` non-3sg past (`termettem` vs `teremtem`) and other `m`-final epenthetic past forms have no reliable attestation (`VRB-PAST-009`).
8. `-tÁbAn` converb-like forms (`jártában`, `mentében`): whether to model as inflection (`VerbForm=Conv` + possessive) or as lexicalised adverbs (`VRB-ADVP-003`).
9. Adverbial participles of `eszik`, `iszik` (`éve`/`eve`, `iva`) and `hisz`: usage too rare to confirm; currently marked marginal.
10. Past conditional of `van`: relative frequency and meaning split of `lettem volna` vs `voltam volna` (`VRB-COND-004`).
11. Plural present of sz-d-v verbs: whether `alusznak`, `feküsznek`, `haragusznak` should be tagged archaic or merely less frequent (`VRB-STEM-012`).
12. Monosyllabic causative selection (`-At` vs `-tAt`) beyond the listed items needs a lexicon-wide check in `DER-VERB` (`VRB-CAUS-001`).
13. Closed-list completeness: a scan of `verb.ron` (19,703 entries, 2026-09-15) confirmed T1 (8) and T2 (8 incl. `lót`; extra only dialectal `dűt`), v-stem verbs (7; extra `fú`, `nyí`, `sí` non-standard), `-ll` monosyllables (11; extra archaic/dialectal `hallik`, `dall`, `köll`, unclear `kall`, `pall`) and added 4 `-ít` back items to `VRB-STEM-013`; the lexicon lacks `nyű`, `fing`, `alszik`, `fekszik`, `nyugszik`, `gyanakszik`, and its `SzDStem`/`SzDVStem` flags disagree with `VRB-STEM-012` (e.g. `növekszik`, `cselekszik` flagged `SzDVStem`, `alkuszik` not `SzDStem`), to be reconciled in the lexicon.
14. `hangzik` infinitive/conditional preference (`hangzani` 1 vs `hangozni` -) and the `-All` past (`javallott` 5 vs `javallt` 2, `sugallt` 4 vs `sugallott` -) vary item by item; a larger corpus is needed.
15. KVF98 p. 290 states that stems in a long vowel other than `á`, `é` + `t` take linking vowels, which would exclude `vét` and include `bocsát`/`lát` inconsistently; this chapter follows SMNy3 p. 735 and T04 (62) (`vét`, `tát` heavy; only `lát`, `lót` light), with `vétenek` 1 vs `vétnek` - as weak support.

## Sources used
All keys are in [sources.md](sources.md). Reliability notes for this chapter:
- `jepst`, `Deckstar`, `magyarispell`, `LP-HU`, `Wikt`: study only; `Deckstar` has no licence.

## Terms
See [glossary.md](glossary.md).
