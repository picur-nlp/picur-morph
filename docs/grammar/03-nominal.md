# 03 Nominal inflection (`NOM`)

Scope: inflection of nouns (főnév) and adjectives (melléknév): the morphotactic template, stem alternants and stem classes, linking vowels (kötőhangzó) and lowering (nyitótő), final low vowel lengthening (tővégi nyúlás), plural, possession (birtokos személyragozás), possessed plural (többes birtokjel), anaphoric possessive `-é` (birtokjel), associative plural `-ék`, the accusative, all case and case-like suffixes, comparison, adverb formation from adjectives, proper-name specifics, and reference paradigms. Pronoun and numeral paradigms, articles and postpositions with personal suffixes are in `PND`; derivational suffixes and compounds in `DER`; the spelling of suffix attachment (hyphens after names, abbreviations, acronyms, numerals, symbols, foreign silent letters, the doubled-letter and three-letter rules as orthography) in `ORT`; vowel harmony, the consonant and vowel classes, and pronunciation of foreign spellings in `PH`; agreement and case government in `SYN`. Clitics (`-e` question particle, `is`, `-s` 'also') are out of scope.

## Area index
| Area | Topic |
|---|---|
| `MODEL` | morphotactic template, feature bundles, licit combinations, suffix classes |
| `STEM` | stem alternants and stem classes (regular, a/e-final, epenthetic, shortening, v-stems, u/ú and ú-deleting stems, possessive-only alternants, kinship, `száj`, other irregulars) |
| `LOW` | lowering: the lowered linking vowel, predictable and lexical portions |
| `LINK` | linking vowel presence and quality per suffix |
| `LENG` | final `a`/`e` lengthening, lengthening and non-lengthening suffixes |
| `PL` | plural `-k` |
| `POSS` | possessive suffixes for a singular possessed noun, `-a`/`-ja` choice |
| `PPL` | possessed plural `-i-` |
| `ANP` | anaphoric possessive `-é`, `-éi` |
| `ASSOC` | associative plural `-ék` |
| `ACC` | accusative `-t` |
| `CASE` | all other case and case-like suffixes |
| `ADJ` | adjective inflection: plural, comparison, `-ik` forms, adverbs |
| `PROP` | proper-name specifics |
| `PARA` | reference paradigms |

## Conventions used in this chapter
- Suffix templates use the archiphonemes of the README. Additional stem-side symbols: `V̄` = lengthened stem-final vowel (`a`->`á`, `e`->`é`).
- Harmony class of a stem (`B`, `F`, `R`) is the output of `PH-HARM`; this chapter consumes it as stem property `harmony` and never recomputes it. For suffixes with three-way `O` (`o`/`e`/`ö`) the `R` variant `ö` is selected when the stem harmony is `R`; `F` selects `e`; `B` selects `o`.
- Stem property names consumed from the lexicon: `pos` (noun, adjective, proper noun), `harmony`, `lowering` (bool), `epenthetic` (bool, with bound alternant), `shortening` (bool, with bound alternant), `v_stem` (with bound alternant), `poss3` (`a` | `ja` | `both`, with optional per-meaning split), `acc_bare` (bool, for CC-final and geminate stems, `NOM-ACC`), `poss_alt` (possessive-only alternant), `loc_t` (locative `-t/-tt` licence), `place_case` (`in` | `on` for place names).
- "Consonant-final" and "vowel-final" are judged on the pronounced stem end (`PH` pronunciation of spellings); for native spelling the last letter decides, digraphs count as one consonant, `h` counts as a vowel only where `PH` marks it silent.
- Corpus buckets quoted as `[ATT n]` are the log-frequency bucket of a word in the project frequency list [`crates/picur-morph-lexicon/data/frequency.ron`](../../crates/picur-morph-lexicon/data/frequency.ron) (top 250k permissive corpus) (higher is more frequent, `-` unattested); they are attestation evidence, not norms.
- UD values are from the UD Hungarian feature documentation [UD-hu]; UniMorph values from the UniMorph Hungarian data [UM-hun].

---

## MODEL: morphotactic template and feature model

### NOM-MODEL-001 Slot order of nominal inflection
- **Statement**: An inflected noun or adjective is a linear sequence of at most seven slots after the stem; every slot is optional, but the order is fixed.
- **Conditions**:
  - Slot 0 stem: free or bound stem alternant (`NOM-STEM-001`), including any derivational material (`DER`).
  - Slot 1 general possessive marker (birtokviszonyjel) `-(j)A`: present in 3sg possessed forms, 3pl possessed forms (as the `-(j)U-` part), and in every possessed plural form after a consonant-final stem.
  - Slot 2 number of the possessed/head noun: plural `-k` (non-possessed) or possessed plural `-i` (after slot 1 or after a vowel-final stem).
  - Slot 3 possessor person/number (birtokos személyrag): `-m`, `-d`, zero, `-nk`, `-tOk`, `-k`.
  - Slot 4 anaphoric possessive `-é` or associative `-ék`.
  - Slot 5 plural of the anaphoric possessive `-i` (only after `-é`).
  - Slot 6 case suffix (at most one; zero = nominative).
- **Output**: `stem + (j)A + i + person + é + i + case`; maximal example `ház+a+i+m+é+i+ból` -> `házaiméiból` 'from those of my houses'.
- **Examples**: `ház+ak+at` -> `házakat`; `kert+je+i+nk+ben` -> `kertjeinkben`; `ház+am+é` -> `házamé`; `ház+é+i+t` -> `házéit`; `Péter+ék+nek` -> `Péteréknek`; `barát+om+ék` -> `barátomék`.
- **Exceptions**: slot 4 `-é` may iterate marginally (`karjaiéié`) [SMNy3 p. 671]; implementations generate one `-é` only and accept iteration in analysis.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-MODEL-002` (feature bundle), `NOM-MODEL-003` (co-occurrence), `NOM-STEM-001` (stem selection by the first suffix), `NOM-LENG-001` (lengthening at each boundary).
- **Sources**: [SMNy3 Kiefer §10.3.5 (56)-(57) pp. 511-512]; [SMNy3 Rebrus §13.1.3 pp. 668-671]; [KVF98 pp. 192-193].
- **Validation**: `goldens` (paradigm tables `NOM-PARA`), `oracle:UniMorph`, `oracle:hunmorph-foma`.
- **Confidence**: `consensus` (segmentation of `-(j)A` as a separate general possessive marker is analytic choice of SMNy3; surface strings are uncontested).

### NOM-MODEL-002 Feature bundle and UD/UniMorph alignment
- **Statement**: Each inflected nominal form is described by the feature bundle `Number`, `Number[psor]`, `Person[psor]`, `Number[psed]`, `Case`, plus the non-UD flag `Assoc` for `-ék`.
- **Conditions**: applies to every form generated by `NOM-MODEL-001`.
- **Output**:
  | Slot filler | UD feature | UniMorph |
  |---|---|---|
  | none (sg head) | `Number=Sing` | `SG` |
  | `-k`, or `-i` of possessed plural | `Number=Plur` | `PL` |
  | person suffix | `Person[psor]=1/2/3`, `Number[psor]=Sing/Plur` | `PSS1S` `PSS2S` `PSS3S` `PSS1P` `PSS2P` `PSS3P` |
  | `-é` | `Number[psed]=Sing` | `LGSPEC1` |
  | `-éi` | `Number[psed]=Plur` | `LGSPEC2` |
  | `-ék` | no dedicated UD value; emit `Number=Plur` plus `Assoc=Yes` (project-internal) | - |
  | case suffix | `Case=` value in `NOM-CASE-001` | see `NOM-CASE-001` |
- **Examples**: `házaim` -> `Number=Plur|Number[psor]=Sing|Person[psor]=1|Case=Nom`; `háza` -> `Number=Sing|Number[psor]=Sing|Person[psor]=3`; `házuk` -> `Number[psor]=Plur|Person[psor]=3`; `házáéi` -> `Number=Sing|Number[psor]=Sing|Person[psor]=3|Number[psed]=Plur`; `házaké` -> `Number=Plur|Number[psed]=Sing`.
- **Exceptions**: in UD, the `-nAk` suffix of a possessor phrase is `Case=Gen`, elsewhere `Case=Dat` (same string, `NOM-CASE-002`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-MODEL-001`, `NOM-CASE-001`, `NOM-ASSOC-001`.
- **Sources**: [UD-hu features Number[psed], Number[psor], Case]; [UM-hun].
- **Validation**: `oracle:UniMorph`, `goldens`.
- **Confidence**: `consensus` for UD possessor features; `contested` for `-ék` (no UD value documented).

### NOM-MODEL-003 Co-occurrence restrictions
- **Statement**: Slot fillers combine freely within the template except for the listed restrictions.
- **Conditions**: per item, as listed in Output.
- **Output**:
  - Plural `-k` and any possessive slot (1-3) exclude each other; plurality of a possessed noun is expressed only by `-i` (`*házakom`, `házaim`).
  - A case suffix is always last; nothing inflectional follows it (`*házbanom`, `*ház+at+om`, `*házban-é`); `-é` cannot follow a case suffix (`házét`, `*házaté`). The string `házatom` is no example: it is also the compound `ház|atom`, which `DER-COMP-023` returns as a reading of its own.
  - `-é` cannot follow bare slot 1 without a person suffix other than 3sg zero; the 3sg form is `-já-é` (`vadjáé`), `*vadjué` [SMNy3 Kiefer p. 511].
  - `-ék` follows a bare stem or a possessed singular (`anyámék`, `barátomék`); after a possessed plural it is marginal (`??barátaimék`) [SMNy3 Kiefer p. 501]; `-ék` is not followed by `-k`.
  - `-é` may follow `-ék` (`Péteréké`); `-ék` does not follow `-é`.
  - The case-like suffixes `-(O)nként`, `-(O)stUl`, `-képp(en)`, `-(O)ntA` attach only to an unpossessed singular stem (`*házakonként`, `*házamostul`, `*házéstul`) [SMNy3 Rebrus (5) p. 669]; `-kor` takes no `-é`/`-ék`, but follows a possessive (`érkezésekor` 10,741, `születésekor` 17,703 [WC2]) and a plural: the plural of an occasion or a recurring time point reads 'on each occasion of' (`alkalmakkor` 26,471, `ünnepekkor` 5,085, `ünnepnapokkor` 51, `késő esti órákkor` 98 [WC2]).
  - `-ként` and essive `-Ul` attach to plural and possessed forms (`házakként`, `házamként`, `segítségül`, `támaszomul`) but are degraded after `-é` (`*házéul`; `*házéként`: 0 tokens - every `-éként` type in Webcorpus 2 is an `-é`-final stem such as `kupéként`, `kávéként`, `attaséként`, or `*egyéként` 895, which is `egyébként` 2,061,271 with its `b` dropped, not the anaphoric `egy+é+ként`; `*máséként` 6, `mindenkiéként` 0, so the restriction stands). The restriction after **proper names is removed**: `-ként` on a given name is regular (`Lajosként` 116, `Jánosként` 525, `Péterként` 425, `Máriaként` 342, `Istvánként` 296, `Annaként` 121 [WC2]), contra SMNy3 Kiefer p. 495; `??Lajosokul` is untested [SMNy3 Kiefer p. 495].
  - `-kor` attaches only to nouns and numerals denoting time points or events, in the singular or, for a recurring one, the plural (`éjfélkor`, `ötkor`, `karácsonykor`, `ünnepekkor`; `*asztalkor`). The restriction is semantic, so it binds the stem and not the plural.
- **Examples**: see Conditions.
- **Exceptions**: SMNy3 lists `házéként` as well-formed in one table and `*házéként` in the text of another chapter; the corpus arbitrates for the starred reading (`OQ-03-003` resolved).
- **Predictability**: `rule`
- **Lexical input**: stem semantic flag `time_noun` for `-kor`.
- **Interactions**: `NOM-CASE-008`, `NOM-CASE-009`, `NOM-CASE-017`..`NOM-CASE-021`, `NOM-ANP-001`, `NOM-ASSOC-001`.
- **Sources**: [SMNy3 Kiefer §10.3.1 pp. 493-495, §10.3.4 p. 511]; [SMNy3 Rebrus §13.1.3 (5) p. 669]; [WC2]; [ATT evidence/corpus-variants.md 2026-09-16].
- **Validation**: `goldens` (negative examples), `oracle:hunmorph-foma`, `attestation` (every `-éként` type in Webcorpus 2 inspected; six common given names + `-ként`, all above the floor).
- **Confidence**: `consensus`; `measured` for `-é` + `-ként` (unattested, restriction kept) and for `-ként` after a given name (regular, restriction removed).

### NOM-MODEL-004 Suffix morphophonological classes
- **Statement**: Each nominal suffix belongs to one of three classes that decide stem alternant selection and lengthening: synthetic (class I, selects the bound stem), analytic (class II, selects the free stem, lengthens `a`/`e`), and super-analytic (class III, free stem, no lengthening).
- **Conditions**:
  | Class | Suffixes (inflectional and case-like) |
  |---|---|
  | I synthetic | plural `-(O)k`; accusative `-(O)t`; superessive `-(O)n`; all possessive suffixes `-(O)m -(O)d -(j)A -(U)nk -(O)tOk -(j)Uk` and `-i-`; distributive `-(O)nként`; sociative `-(O)stUl`; adjective comparative `-(A)bb`; adverbial `-(A)n`; distributive-temporal `-(O)ntA` |
  | II analytic | `-nAk`, `-vAl`, `-ért`, `-vÁ`, `-ig`, `-bAn`, `-bA`, `-bÓl`, `-rA`, `-rÓl`, `-nÁl`, `-hOz`, `-tÓl`, essive `-Ul`, `-é`, `-ék` |
  | III super-analytic | `-ként`, `-kor`, `-képp(en)`, multiplicative `-szOr` (`PND`); derivational `-ság/-ség`, `-i`, `-szerű`, `-féle` (`DER`) |
- **Output**: class I -> bound alternant where the stem has one (`bokr-`, `lov-`, `madar-`, `falv-`); class II and III -> free stem (`bokor-`, `ló-`, `madár-`); classes I and II lengthen final `a`/`e`; class III does not.
- **Examples**: `bokor`: `bokrok`, `bokrot`, `bokron`, `bokra` (I) vs `bokorban`, `bokorig`, `bokoré` (II) vs `bokorként` (III); `alma`: `almák`, `almában` (I, II) vs `almaként` (III).
- **Exceptions**: stems of individual classes may be unstable before the accusative, the superessive and 3sg possessive (`NOM-STEM-005`, `NOM-STEM-007`, `NOM-STEM-011`); v-stems keep the free stem before some class I suffixes (`szót`, `szón`, `NOM-STEM-010`); shortening stems do not shorten before the superessive (`NOM-STEM-007`).
- **Predictability**: `rule`
- **Lexical input**: `none` (class is a suffix property)
- **Interactions**: `NOM-STEM-*`, `NOM-LENG-001`, `NOM-LENG-002`.
- **Sources**: [SMNy3 Rebrus §13.3.1 (33) p. 694, §13.3.2.5 pp. 716-718, §13.4.1 (113) p. 743, §13.5.1 (135)-(136) pp. 757-759, §13.6.1 (187)-(189) pp. 787-788]; [T04 §4.1.2 pp. 145-152].
- **Validation**: `goldens`, `oracle:hunmorph-foma`.
- **Confidence**: `consensus`.

---

## STEM: stem alternants and stem classes

### NOM-STEM-001 Stem alternant selection
- **Statement**: A nominal lexeme has one free stem and at most three bound alternants (synthetic, possessive-3, possessive-only); the first suffix after the stem selects the alternant by its class.
- **Conditions**:
  - Free stem: citation form; used word-finally and before class II and III suffixes.
  - Synthetic alternant: used before class I suffixes (`NOM-MODEL-004`) for epenthetic, shortening, v-stem, u/ú->v and ú-deleting stems.
  - Possessive-3 alternant: used only before `-(j)A` / `-(j)U-` (ajtó type, kinship type).
  - Possessive-only alternant: used before all possessive suffixes (`száj` type, `fiú` type).
- **Output**: the alternant; subsequent suffixes attach to the output of the previous one, which is itself an inflected (non-alternating, lowering) stem (`NOM-LOW-002`).
- **Examples**: `ló`: free `ló-` (`lóban`), synthetic `lov-` (`lovak`); `ajtó`: possessive-3 `ajtaj-` (`ajtaja`), elsewhere `ajtó-` (`ajtóm`); `száj`: possessive-only `szá-` before `-m -d -nk -tok` (`szám`, `szánk`).
- **Exceptions**: listed per class in `NOM-STEM-005`..`NOM-STEM-017`.
- **Predictability**: `rule` (selection), `lexical` (membership of each class)
- **Lexical input**: stem alternants per class.
- **Interactions**: `NOM-MODEL-004`, `NOM-LOW-005`.
- **Sources**: [SMNy3 Rebrus §13.1 pp. 659-661, §13.6.5 pp. 816-818]; [KVF98 pp. 193-194].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-STEM-002 Regular (invariant) stems
- **Statement**: A stem without any alternation lexical flag uses the same string before every suffix; only final `a`/`e` lengthening (`NOM-LENG-001`) applies.
- **Conditions**: no `epenthetic`, `shortening`, `v_stem`, `poss_alt` flag.
- **Output**: free stem everywhere.
- **Examples**: `kalap` -> `kalapok`, `kalapot`, `kalapon`, `kalapja`, `kalapban`; `hajó` -> `hajók`, `hajót`, `hajón`, `hajója`; `kert` -> `kertek`, `kertet`, `kertje`.
- **Exceptions**: `-`
- **Predictability**: `rule` (default class; all new loans and coinages belong here [T04 p. 161])
- **Lexical input**: absence of alternation flags.
- **Interactions**: `NOM-LENG-001`, `NOM-LINK-*`.
- **Sources**: [T04 §4.1.3 p. 161]; [SMNy3 Rebrus §13.3.1 p. 695].
- **Validation**: `goldens`, `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-STEM-003 Final a/e stems
- **Statement**: A stem ending in short `a` or `e` alternates with `á`/`é` before class I and II suffixes; this is the only fully productive stem alternation and is specified in `NOM-LENG-001`.
- **Conditions**: stem ends in letter `a` or `e` pronounced as a vowel; not an all-caps acronym (`ORT-ABBR-015`).
- **Output**: see `NOM-LENG-001`.
- **Examples**: `alma` -> `almát`, `almák`; `kefe` -> `keféje`, `kefének`; `alma` -> `almaként`.
- **Exceptions**: see `NOM-LENG-003`.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LENG-001`..`NOM-LENG-004`.
- **Sources**: [AkH12 §29]; [SMNy3 Rebrus §13.6.1 pp. 786-791].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `normative`.

### NOM-STEM-004 Stems with final h
- **Statement**: A stem written with final `h` counts as consonant-final when `h` is pronounced; when the norm allows a silent final `h`, both vowel-final and consonant-final suffixation are admitted before `-vAl`/`-vÁ`; before vowel-initial suffixes the `h` is always pronounced and the stem is consonant-final.
- **Conditions**: stem-final letter `h`; `PH-CONS-005` pronunciation flag `h_silent` (`always` | `variable` | `never`).
- **Output**:
  - Vowel-initial suffixes and linking vowels: consonant-final behaviour (`csehek`, `méhet`, `dühöt`, `juhot`, `céhet`, `oláhot`, `dohot`).
  - Accusative: linking vowel required, since `h` does not form a licit coda with `t` (`NOM-ACC-004`).
  - `-vAl`/`-vÁ`: `h_silent=always` -> `-val/-vel` unassimilated (`csehvel`, `pléhvé`); variable (`céh`, `düh`, `juh`, `méh`, `rüh` [AkH12 §74]) -> both (`dühvel` ~ `dühhel`, `méhvé` ~ `méhhé`); never -> assimilated `hh` (`dohhal`, `Allahhal`, `pechhel`).
  - Other consonant-initial suffixes: no change in spelling (`csehnek`, `méhben`).
- **Examples**: see Output.
- **Exceptions**: foreign `ch` and `h` finals follow pronunciation (`ORT-FOR-011`, `PH-G2P-008`) (`Bachhal`).
- **Predictability**: `lexical` (h-adding class of 8 items listed in [SMNy3 p. 789]: `oláh`, `cseh`, `éh`, `céh`, `pléh`, `méh`, `juh`, `düh`; plus `rüh` [AkH12 §74]; only `cseh` and `pléh` are written as vowel-final by the norm [AkH12 §41])
- **Lexical input**: `h_silent`.
- **Interactions**: `NOM-CASE-004`, `NOM-ACC-004`, `PH-CONS-005` (pronunciation of h), `ORT-ASSIM-005`.
- **Sources**: [AkH12 §41, §74, §82 e)]; [SMNy3 Rebrus p. 789].
- **Validation**: `goldens`, `attestation` (`méhet` [ATT 3], `dühöt` [ATT 6], `juhot` [ATT 5], `dühvel` [ATT 4]).
- **Confidence**: `normative` for spelling doublets; `consensus` for the rest.

### NOM-STEM-005 Epenthetic (vowel-zero) stems
- **Statement**: In a closed class of stems the vowel of the last syllable is absent before class I suffixes and present elsewhere.
- **Conditions**:
  - Lexical flag `epenthetic`; bound alternant = free stem minus last vowel.
  - Necessary shape (not sufficient): at least two syllables, ending in `V(V)CVC` with a single consonant on each side of the unstable vowel; if the preceding vowel is long it is `á` or `é` (exceptions `ólom`, `sólyom`); flanking consonants not identical; if both obstruents, same voicing; the unstable vowel is short mid `o`/`e`/`ö` by harmony, except seven irregular stems (`vacak`, `kazal`, `ajak`, `bajusz` among nouns).
  - Triggering suffixes: all class I suffixes (plural, accusative, superessive, all possessive suffixes, `-(O)nként`, `-(O)stUl`, comparative, adverbial `-An`) and vowel-initial derivational suffixes (`DER`).
  - Non-triggering: class II and III (`bokorban`, `bokorig`, `bokoré`, `bokorként`).
- **Output**: bound alternant before triggering suffixes.
- **Examples**: `bokor` -> `bokrok`, `bokrot`, `bokron`, `bokra`, `bokrom`, `bokrunk`, `bokrotok`, `bokruk`, `bokrai`; `tükör` -> `tükrök`, `tükröt`, `tükrön`, `tükre`; `retek` -> `retkek`; `öböl` -> `öblök`; `torony` -> `tornyok`, `tornyot`; `bagoly` -> `baglyok`; `majom` -> `majmok`; `terem` -> `termek`; `ikrek` (`iker`); `szobor` -> `szobrok`.
- **Exceptions**:
  - Near-identical non-epenthetic stems: `perem` -> `peremek`; `tábor` -> `táborok`; `szurony` -> `szuronyok`; `csődör` -> `csődörök`; `motor` -> `motorok` (`%motrok` only in group varieties) [T04 (4); SMNy3 p. 694].
  - Metathesis stems (5): `kehely` -> `kelyhek`, `pehely` -> `pelyhek`, `teher` -> `terhek`, `?moholy` -> `molyhok`, `?boholy` -> `bolyhok`.
  - Instability before accusative, superessive and 3sg possessive: some items also show free-stem forms (`bajuszt` ~ `bajszot`, `öbölt` ~ `öblöt`, `sarokja`, `fodoron`, `bátort`, `retekje`; marginal `?háromon`, `?ezeren`) [T04 p. 148; SMNy3 p. 693]; conditions idiosyncratic, variation per item.
  - Place names: epenthetic only before derivational `-i` (`Eger` -> `egri`, but `Egert`, `Egerben`, `Egeren`) [SMNy3 p. 693].
  - Derived nouns in `-alom/-elem` and `-dalom/-delem` are epenthetic and lowering (`hatalom` -> `hatalmak`, `szerelem` -> `szerelmek`, `birodalom` -> `birodalma`).
- **Predictability**: `lexical` (closed; 104 monomorphemic noun stems in SMNy3 counts: 51 with `o`, 31 with `e`, 22 with `ö`; about 250 noun stems including complex ones [T04 fn. 1]; plus `-alom/-elem` derivatives)
- **Lexical input**: `epenthetic`, bound alternant.
- **Interactions**: `NOM-MODEL-004`; `PH-ZERO-001`, `PH-ZERO-002`, `PH-ZERO-005`; `NOM-LOW-005` (epenthetic stems may be lowering or not: `marok` -> `markom` vs `farok` -> `farkam`); `NOM-POSS-009`; `NOM-ACC-008`.
- **Sources**: [SMNy3 Rebrus §13.3.1 (33)-(43) pp. 693-699]; [T04 §4.1.1 pp. 139-144, fn. 1]; [KVF98 p. 194].
- **Validation**: `goldens`, `attestation` (`bokrok` [ATT 9], `tornyok` [ATT 7], `hatalmak` [ATT 10], `bajuszt` [ATT 3], `bajszot` `-`).
- **Confidence**: `consensus`.

### NOM-STEM-006 Epenthetic stem inventory (common members)
- **Statement**: The implementation lexicon lists epenthetic stems individually; the following common members serve as goldens.
- **Conditions**: lexical.
- **Output**:
  | Unstable vowel | Stems (plural shown for doubtful ones) |
  |---|---|
  | `o` | `bokor`, `sarok` (`sarkok` ~ `sarkak`, meaning-dependent), `titok`, `fodor`, `csokor`, `dolog`, `gyomor`, `majom`, `malom`, `halom`, `álom`, `ólom`, `sólyom`, `torony`, `torok`, `farok`, `marok`, `bagoly`, `kapocs`, `cukor`, `szobor`, `haszon`, `vászon` (`vásznak`), `jászol` (`jászlak`); adjective `bátor` |
  | `e` | `retek`, `fészek`, `kebel`, `féreg`, `szeder`, `eper`, `cseber`, `verem`, `terem`, `telek`, `lepel`, `iker`, `meder`; numeral `ezer` (`PND`) |
  | `ö` | `öböl`, `ököl`, `gödör`, `tükör`, `vödör`, `kölyök`, `köröm` |
  | irregular vowel | `kazal` (`kazlak`), `ajak` (`ajkak`), `vacak` (`vackok`), `bajusz` (`bajszok`) |
- **Examples**: `dolgok` [ATT 17], `titkok` [ATT 10], `férgek` [ATT 17], `telkek` [ATT 9], `körmök` [ATT 9], `ajkak` [ATT 8].
- **Exceptions**: `-`
- **Predictability**: `lexical` (see `NOM-STEM-005` for class size)
- **Lexical input**: `epenthetic`.
- **Interactions**: `NOM-STEM-005`.
- **Sources**: [SMNy3 Rebrus (33) p. 694]; [T04 (2)-(4) pp. 140-143]; [KVF98 p. 194].
- **Validation**: `attestation`, `oracle:hunmorph-foma`.
- **Confidence**: `consensus`.

### NOM-STEM-007 Shortening stems (stem-internal)
- **Statement**: In a closed class of stems the last stem vowel (`á`, `é`, `í`, `ú`, `ű`) is short (`a`, `e`, `i`, `u`, `ü`) when a short low vowel follows across a single consonant, and before possessive `-Unk`/`-Uk`.
- **Conditions**:
  - Lexical flag `shortening`; only the last stem vowel shortens; no other long vowel in the stem; never `ó`/`ő`; the stem ends in one consonant (`lélek` is the only item that also drops its last vowel).
  - Triggers: plural `-ak/-ek`, accusative `-at/-et`, possessive `-am -ad -a -unk -atok -uk` and `-ai-`, `-anként`, `-astul`, derivational suffixes with low vowel (`DER`).
  - Non-triggers: superessive (`nyáron`, `kézen`, `hídon`, `úton`, `tűzön`), class II and III suffixes (`nyárig`, `nyárban`, `nyáré`, `madárként`).
- **Output**: shortened bound alternant; the stem is always lowering (`NOM-LOW-005`).
- **Examples**: `nyár` -> `nyarak`, `nyarat`, `nyaram`, `nyara`, `nyarunk`, `nyaruk`, `nyáron`; `kéz` -> `kezek`, `kezet`, `keze`, `kézen`, `kézzel`; `víz` -> `vizek`, `vizet`, `vize`, `vízen`; `út` -> `utak`, `utat`, `utam`, `úton`; `tűz` -> `tüzek`, `tüzet`, `tüze`, `tűzön`; `lélek` -> `lelkek`, `lelket`, `lelke`, `lelken`.
- **Exceptions**:
  - Unstable 3sg possessive and 3pl/possessed plural with long vowel and `-ja`: `híd` -> `hídja`, `hídjuk`, `hídjai`; `kút` -> `kútja`; `lúd` -> `lúdja`; `rúd` -> `rúdja`; `út` -> `útja`, `útjuk`, `útjai` (`?utai`, archaic and biblical: `útjai` 28,629 : `utai` 241 [WC2]; AkH12 §27 names `útja`, `útjuk` alone) [SMNy3 p. 743; AkH12 §27 b); UM-hun]; but `nyula`, `vize`.
  - Instability before the accusative: `ér` 'brook' -> `ért`, `ész` -> `észt` alongside `eszet` [SMNy3 p. 743].
  - Homophones split: `dél` 'noon/south' -> `delet` vs `délt` (`dél` as compass point in some uses); `szél` 'wind' -> `szelek` vs `szél` 'edge' -> `szélek`; `felünk` vs `félünk` (SMNy3 pair; `fél` 'half' and 'party' both shorten in `fele`, `felek`); `nyár` 'summer' -> `nyarak` vs `nyár` 'poplar' -> `nyárak`, `nyárja` [SMNy3 p. 742; Tab21 (4)].
  - Superessive `vizen` is attested next to standard `vízen` ([ATT 4] vs [ATT 11]), but the superessive is a non-trigger by this rule's own conditions and the web corpus shows the short form only in accent-stripped text (`uton` 17,480 : `úton` 1,275,847, `kezen` 514 : `kézen` 39,345 [WC2]), so the bound alternant is not generated or accepted before it.
- **Predictability**: `lexical` (closed class; inventory `NOM-STEM-008`)
- **Lexical input**: `shortening`, bound alternant.
- **Interactions**: `NOM-LOW-005`, `NOM-POSS-010`, `NOM-ACC-008`, `PH-LEN-007`.
- **Sources**: [SMNy3 Rebrus §13.4.1 (113) pp. 741-744]; [KVF98 p. 194]; [UM-hun].
- **Validation**: `goldens`, `attestation` (`kezet` [ATT 12], `vizet` [ATT 17], `nyarat` [ATT 6], `hídja` [ATT 8]).
- **Confidence**: `consensus`.

### NOM-STEM-008 Shortening stem inventory
- **Statement**: The shortening class is closed; its members are listed.
- **Conditions**: lexical.
- **Output**:
  | Alternation | Monosyllabic | Polysyllabic |
  |---|---|---|
  | `á`~`a` | `nyár`, `sár` | `agár`, `bogár`, `kanál`, `kosár`, `madár`, `mocsár`, `mozsár`, `pohár`, `sudár`, `sugár`, `szamár`, `parázs`, `darázs`, `fonál` (`gúnár` is **not** a member: `gunarak` rejected by the spelling checker, `gúnárok` 158) |
  | `é`~`e` | `tél`, `ég`, `jég`, `lég`, `légy`, `bél`, `dél`, `fél`, `nyél`, `szél`, `szén`, `dér`, `ér`, `tér`, `ész`, `mész`, `hét`, `hév`, `név`, `kéz`, `réz` | corpus-verified: `egér`, `fedél`, `födél`, `fenék`, `gyökér`, `kenyér`, `kerék`, `kötél`, `levél`, `szekér`, `tehén`, `tenyér`, `veréb`, `cserép`, `fazék`, `szemét`, `nehéz`, `derék`, `elég`, `közép`, `egyéb`, `miegyéb`, `nyolcadfél`, `hitfél`, `raktér`, `altér`, `beltér`, `kültér`; `lélek` (with vowel drop); adjectives `nehéz`, `kevés`, `derék`, `elevén` |
  | `í`~`i` | `víz`, `híd`, `nyíl`, `ín` | - |
  | `ú`~`u` | `út`, `kút`, `lúd`, `nyúl`, `rúd`, `úr` | - |
  | `ű`~`ü` | `tűz`, `fűz`, `szűz` | - |
- **Examples**: `poharat` [ATT 8], `kanalat` [ATT 5], `kenyeret` [ATT 12], `levelet` [ATT 15], `egeret` [ATT 8], `gyökeret` [ATT 8], `kötelet` [ATT 8], `tehenet` [ATT 6], `nyilat` [ATT 5], `urat` [ATT 13], `nyulat` [ATT 6].
- **Exceptions**: SMNy3 gives 21 monosyllabic `é` items plus "about 20" further items. The polysyllabic `é ~ e` column was induced over 8,983 shape candidates and corpus-verified (`OQ-03-001` resolved), and `?gúnár` is dropped. SMNy3 also lists `szűgy` in the `ű` row; the dictionary spelling is `szügy` with a short vowel throughout (`szügye` in UM-hun), so no written alternation arises and the item is not listed.
- **Predictability**: `lexical` (**141 roots** measured, of which 35 monosyllabic and 106 polysyllabic, inside a 497-member list that also covers derived and compound stems; SMNy3 (113) gives 17 `á`, 21+about 20 `é`, 4 `í`, 6 `ú`, 4 `ű`, all of which are confirmed)
- **Lexical input**: `shortening`; the list is `shortening-stems.tsv`, 497 members (141 roots), in `local/lists/shortening-stems.tsv` - working data, not yet read by the build.
- **Interactions**: `NOM-STEM-007`.
- **Sources**: [SMNy3 Rebrus (113) p. 743]; [KVF98 p. 194]; [ATT]; [WC2]; [MTA-HELYES]; [hunmorph-foma]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `attestation`, `oracle:hunmorph-foma`; 8,983 shape candidates screened, 9 in review. Oracle agreement on corpus-decided rows: morphdb 5057/5072, our `.ron` 5056/5072, foma 4902/4903, hunspell 4698/4700, UniMorph 1638/1639, kaikki 2074/2075.
- **Confidence**: `consensus` for the SMNy3 items; `consensus` (upgraded from `tendency`) for the polysyllabic list, which is now corpus-verified rather than reconstructed.

### NOM-STEM-009 Non-systematic vowel shortening (out of inflection)
- **Statement**: Shortening before derivational suffixes (`ír` -> `irodalom`, `négy` -> `negyed`, `tíz` -> `tizen-`, `húsz` -> `huszon-`, Latinate `filozofál`) is not an inflectional stem class and is handled in `DER` and `PND`.
- **Conditions**: derivational or numeral contexts.
- **Output**: no effect on inflection: `négyet`, `négyek` (no shortening).
- **Examples**: `négy` -> `négyet`, but `negyed`.
- **Exceptions**: `-`
- **Predictability**: `lexical`
- **Lexical input**: `none` in `NOM`.
- **Interactions**: `DER-*`, `PND-NUMINF-004`, `PH-LEN-010`.
- **Sources**: [SMNy3 Rebrus §13.4.1 pp. 741-744].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-STEM-010 Monosyllabic v-stems
- **Statement**: A closed class of monosyllabic stems ending in a long (almost always rounded) vowel add `v` before class I suffixes; shortening v-stems also shorten the vowel (`ló` -> `lov-`, `kő` -> `köv-`), vowel-changing v-stems also change `ó` to `a` (`tó` -> `tav-`); `lé` -> `lev-` is the only front item of the vowel-changing type [SMNy3 p. 783].
- **Conditions**:
  | Type | Stems | Bound alternant |
  |---|---|---|
  | non-shortening | `mű` (noun), `bő` (adj) | `műv-`, `bőv-` |
  | shortening | `ló`, `kő`, `cső`, `tő`, `fű`, `nyű`, `?szú` | `lov-`, `köv-`, `csöv-`, `töv-`, `füv-`, `nyüv-`, `szuv-` |
  | vowel-changing | `tó`, `hó`, `szó`, `?jó` (noun 'goods'), `lé` ('juice, broth', `é`->`e`) | `tav-`, `hav-`, `szav-`, `jav-`, `lev-` |
  - Triggers: plural, accusative, superessive, all possessive suffixes, `-(A)nként`, `-(A)stUl`, vowel-initial derivational suffixes.
  - Non-triggers: class II and III (`lóban`, `lóval`, `lóig`, `lóé`, `lóként`, `kővel`, `műben`).
- **Output**: bound alternant; all nominal v-stems are lowering (`lovak`, `kövek`, `művek`).
- **Examples**: `ló` -> `lovak`, `lovat`, `lovon`, `lova`, `lovam`, `lovunk`, `lovatok`, `lovuk`, `lovai`; `tó` -> `tavak`, `tavat`, `tavon`, `tava`; `kő` -> `kövek`, `követ`, `kövön`, `köve`, `követek` (2pl), `kövük`; `mű` -> `művek`, `művet`, `művön`, `műve`; `fű` -> `füvek`, `füvet`, `füvön`; `cső` -> `csövek`; `lé` -> `levek`, `levet`, `leve`.
- **Exceptions**:
  - `szó` keeps the free stem in accusative and superessive: `szót`, `szón` (marginal `?szavat`, `?szavon`: `szót` 534,648 : `szavat` 1,408, `szón` 4,588 : `szavon` 57 [WC2]), but `szavak` ~ `szók` (normative doublet, `szavak` preferred: [ATT 15] vs [ATT 3]), `szava` [SMNy3 (136); AkH12 §30].
  - `tó`: accusative `tavat` ~ `tót` (normative doublet [AkH12 §30]; `tót` is also the ethnonym 'Slovak').
  - `jó`: adjective is not a v-stem (`jók`, `jót`, `jón`, `jobb`); noun 'goods, property' has `javak`, `javai`; accusative `jót` for both [SMNy3 p. 758].
  - `hó`: 'snow' accusative `havat` ~ `hót`, both standard, including in compounds (`műhavat` ~ `műhót`); `havat` is the ordinary form (`havat` [ATT 8], `hót` [ATT 0]) and `hót` is accepted, with a literary use noted [ENYELV 2022-11-01], [ENYELV 2021-05-18] (`OQ-03-018` resolved 2026-09-16; the meaning split SMNy3 p. 758 implies is not confirmed by the advisory service); 'month' (archaic) `hava`, `havonta`, and `hó` elsewhere.
  - `lé`: 'juice' `levek`, `levet` ~ `lét` (normative doublet [AkH12 §30]), superessive `lén` ~ `?leven`; 'money' (slang) is not a v-stem (`lém`, `léd`, `lét`).
  - `szú`: only `szuvas` shortens reliably; `szúja`, `szút`, `szún` otherwise.
  - `mű` also occurs with free-stem inflection in technical/prefixoid uses (`műk`, `műje`), listed as a second lexeme in [UM-hun]; the standard noun is the v-stem.
  - Instrumental of `mű` is `művel` (`mű+vel`), homographic with the verb `művel`.
  - Non-v-stems of the same shape: `nő` (`nők`), `só` (`sók`), `fő` 'head' (`fők`, `főt`), `hű` adj (`hűek`; noun `hívek` 'followers' lexicalised) [SMNy3 (138)].
- **Predictability**: `lexical` (closed; about 14 nominal items; the shape is necessary, not sufficient)
- **Lexical input**: `v_stem`, bound alternant.
- **Interactions**: `NOM-LOW-005`, `NOM-POSS-010`, `NOM-CASE-011` (superessive), `NOM-STEM-018`, `PH-VSTEM-001`, `PH-VSTEM-002`.
- **Sources**: [SMNy3 Rebrus §13.5.1 (135)-(139) pp. 755-760, (181) p. 781]; [AkH12 §30]; [KVF98 p. 194]; [T04 fn. 18 p. 149]; [UM-hun].
- **Validation**: `goldens`, `attestation` (`lovat` [ATT 10], `tavat` [ATT 9], `követ` [ATT 15], `füvet` [ATT 9], `csövek` [ATT 10], `lovon` [ATT 7], `tavon` [ATT 8], `szón` [ATT 3]).
- **Confidence**: `consensus`.

### NOM-STEM-011 Bisyllabic u/ú/ű -> v stems
- **Statement**: In a small unstable class of bisyllabic stems ending in a rounded high vowel, the final vowel is replaced by `v` before the plural `-k` and the possessed plural `-i` (`PH-VSTEM-004`); the personal possessives, the accusative and the superessive keep the vowel stem, except the third singular `-A` of `tetű` and `odú` 'hollow'.
- **Conditions**:
  | Stem | v-forms | non-v forms |
  |---|---|---|
  | `tetű` | `tetvek` ~ `tetűk`, `tetve` ~ `tetűje`, `tetvei` ~ `tetűi` | `tetűm`, `tetűt`, `tetűn` |
  | `falu` | `falvak` ~ `faluk`, `falvai` ~ `falui` | `falum`, `falud`, `falunk`, `faluja`, `falujuk`, `falut`, `falun` |
  | `daru` | `darvak` 'cranes (birds)' | `daruk` 'cranes (machines)', `darut`, `darun` |
  | `odú` | `odvak`, `odva` 'its hollow' | `odúk`, `odúja` 'its lair', `odút`, `odún` |
  | `hamu` | `hamvak`, `hamvai` (lexicalised 'ashes, remains') | `hamut`, `hamuja` |
- **Output**: per table; preferred variant for `falu`: plural `falvak` over `faluk` (71,591 : 4,267 [WC2]); singular possessives on the vowel stem, `falum` 3,585 : `?falvam` 17 [WC2]; 3sg only `faluja` [ATT 4] and 3pl `falujuk` (SMNy3 (142) marks both `(!)` as non-v forms); `falva` [ATT 3] survives only as the place-name element `-falva` (`Istvánfalva`, `ORT-NAME-026`) and must not be generated as the common-noun 3sg; possessed plural `falvai` ~ `falui` (6,153 : 92 [WC2]); `faluk` ~ `falvak` is a normative doublet [AkH12 §30].
- **Examples**: see Conditions.
- **Exceptions**: `mag` -> `magvak` ~ `magok`, `magva` ~ `magja` is a consonant-final stem with `v`-insertion; treated as lexical alternants, both current with a register and sense split measured 2026-09-16 (`NOM-STEM-016`; `magok` 42,603 : `magvak` 32,449, `magja` 33,860 : `magva` 7,314 [WC2]).
- **Predictability**: `lexical` (5 stems)
- **Lexical input**: class membership (`v_stem` with u-deletion) and the per-meaning split; the per-suffix licence follows from the statement.
- **Interactions**: `NOM-STEM-010`.
- **Sources**: [SMNy3 Rebrus §13.5.1.4 (142) pp. 762-763]; [AkH12 §30]; [KVF98 p. 194]; [ATT].
- **Validation**: `attestation`, `goldens`.
- **Confidence**: `consensus` for the class; `tendency` for variant preference.

### NOM-STEM-012 ú-deleting stems
- **Statement**: In bisyllabic stems ending in consonant + palatal/`j` + `ú`, the final `ú` may be absent before class I suffixes; the superessive always keeps it; for `borjú` the split is by paradigm cell rather than by item - the accusative keeps the `ú` (`borjút` 4,195 : `borjat` 636, 87%) while the plural drops it (`borjak` 3,171 : `borjúk` 361, 90%) [WC2].
- **Conditions**:
  | Stem | ú-less forms | full forms |
  |---|---|---|
  | `borjú` | `borjak`, `borjat`, `borjam`, `borja` 'calf of (a cow)', `borjuk` (3pl) | `borjúk`, `borjút`, `borjúm`, `borjúja` 'calf owned by', `borjún` |
  | `varjú` | `varjak`, `varjat` | `varjúk`, `varjút`, `varjún` |
  | `ifjú` (noun) | `ifjak`, `ifjai` | `ifjút` |
  | `gyapjú` | `gyapja` 'wool of (a sheep)' | `gyapjúja` 'wool of (a sweater)' |
  | `sarjú` | `sarja` | `sarjút` |
  | `fattyú` | `fattyak` | `fattyút` |
- **Output**: plural ú-less forms are preferred (`borjak` [ATT 5] vs `borjúk` `-`; `varjak` [ATT 6]; `ifjak` [ATT 9]); accusative `borjat` ~ `borjút` and plural `borjak` ~ `borjúk`, `varjak` ~ `varjúk` are normative doublets [AkH12 §30; SMNy3 (148)]; measured [WC2]: `borjút` 4,195 : `borjat` 636 and `borjak` 3,171 : `borjúk` 361, all four legal [MTA-HELYES]; `borjúk` survives essentially only in literary register (literary 123 : 32, against technical 1 : 2,495 and official 0 : 219); superessive always full (`borjún`).
- **Examples**: see Conditions.
- **Exceptions**: adjective `ifjú` has comparative `ifjabb`, adverb `ifjan` (`NOM-ADJ-008`).
- **Predictability**: `lexical` (6 stems)
- **Lexical input**: bound alternant, per-meaning split for possessives.
- **Interactions**: `NOM-ADJ-008`, `NOM-POSS-010`.
- **Sources**: [SMNy3 Rebrus §13.5.1.6 (148)-(149) p. 766]; [AkH12 §30]; [T04 fn. 12 p. 145]; [WC2]; [MTA-HELYES]; [ATT evidence/corpus-variants.md 2026-09-16]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `attestation` (`[Acc]` / `[Pl]` filters on lemma `borjú`, plus a register split).
- **Confidence**: `consensus`; `measured` for `borjú`, cell-split not item-split.

### NOM-STEM-013 Possessive-only vowel change (ajtó type)
- **Statement**: In a closed set of stems ending in `ó`/`ő`, the final vowel becomes `a`/`e` + `j` before 3sg and 3pl possessive `-(j)A`/`-(j)U-` (and the possessed plural built on it); elsewhere the stem is invariant.
- **Conditions**:
  - Lexical flag `poss_alt=aj/ej`; not before `-m -d -nk -tOk` (`ajtóm`, `időm`) and not before other suffixes.
  - Members `ó`: `ajtó`, `bimbó`, `csikó`, `disznó`, `hintó`, `korsó`, `kunyhó`, `tinó`, `zászló`, `zsidó`; `ő`: `idő`, `első`, `erdő`, `erő`, `esztendő`, `külső`, `belső`, `mező`, `szőlő`, `szülő`, `tető`, `tüdő`, `velő`, `vő` (SMNy3 (180): 10 `ó` + 13 `ő` items; `belső` added by attestation `belseje` [ATT 9]). Not members: `csípő` (`csípője` [ATT 4], `csípeje` `-`), `vessző` (`vesszője`), and the vast majority of `ó`/`ő` nouns (`hajója`, `cipője`).
- **Output**: per stem preference (variant `~`):
  | Stem | 3sg | 3pl | possessed plural |
  |---|---|---|---|
  | `ajtó` | `ajtaja` (pref.) ~ `ajtója` | `ajtajuk` ~ `ajtójuk` | `ajtói` 2,802 ~ `ajtajai` 2,388 - both current at 54 : 46 [WC2], both legal [MTA-HELYES]; SMNy3 lists only `tinói`-type regular forms and archaic `disznai` |
  | `idő` | `ideje` (only) | `idejük` | `idői` |
  | `erő` | `ereje` | `erejük` | `erői` |
  | `első`, `külső`, `belső` | `elseje` (date: `május elseje`), `külseje`, `belseje` | `külsejük` | - |
  | `esztendő` | `esztendeje` | `esztendejük` | - |
  | `erdő`, `mező` | `erdeje` ~ `erdője`, `mezeje` ~ `mezője` | `erdejük` ~ `erdőjük` | `erdei` ~ `erdői` |
  | `tüdő` | `tüdeje` | `tüdejük` | - |
  | `vő` | `veje` | `vejük` | - |
  | `zászló` | `zászlaja` ~ `zászlója` | - | - |
  | `tető` | `teteje` 'top of' vs `tetője` 'roof of' | - | - |
  | `szülő` | `szülője` (pref.) | `szülőjük` | `szülei` 'his parents' (lexicalised) |
  | `disznó`, `hintó`, `csikó`, `korsó`, `kunyhó`, `tinó`, `bimbó`, `zsidó` | `-aja` ~ `-ója`, rare |  |  |
- **Examples**: `ajtói` 2,802 ~ `ajtajai` 2,388 (both current) [WC2]; `ideje` [ATT 19] vs `idője` `-`; `ereje` [ATT 15] vs `erője` `-`; `erői` [ATT 9]; `külseje` [ATT 8], `belseje` [ATT 9]; `ajtaja` [ATT 9] vs `ajtója` `-`; `zászlaja` [ATT 7] ~ `zászlója` [ATT 4]; `mezeje` [ATT 3] ~ `mezője` [ATT 5]; `teteje` [ATT 10].
- **Exceptions**: before `-jai` the old forms `disznaim`, `erdeim` are archaic [SMNy3 p. 781]; the alternation never occurs before derivational `-i` except lexicalised `erdei`, `mezei`, `esztendei`.
- **Predictability**: `lexical` (24 stems; the class is unstable, the `-ója/-ője` forms spread; `zászlója` ~ `zászlaja`, `erdője` ~ `erdeje` are normative doublets [AkH12 §30])
- **Lexical input**: `poss_alt`, per-item preference.
- **Interactions**: `NOM-POSS-004`, `NOM-PPL-002`.
- **Sources**: [SMNy3 Rebrus §13.5.3.3 (180) pp. 780-781]; [UM-hun `ajtó`]; [ATT]; [WC2]; [MTA-HELYES]; [ATT evidence/corpus-variants.md 2026-09-16].
- **Validation**: `attestation`, `goldens`.
- **Confidence**: `consensus` for class; `tendency` for preferences; `measured` for `ajtói` ~ `ajtajai` (both standard, no preference).

### NOM-STEM-014 Kinship stems with possessive-only truncation
- **Statement**: A few kinship nouns use a truncated alternant before 3sg and 3pl possessive `-jA`/`-jU-` and a lengthened or extended alternant before other possessive suffixes.
- **Conditions**: per item, as listed in Output.
- **Output**:
  | Stem | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | possessed pl 1sg |
  |---|---|---|---|---|---|---|---|
  | `apa` | `apám` | `apád` | `apja` | `apánk` | `apátok` | `apjuk` | `apáim` |
  | `anya` | `anyám` | `anyád` | `anyja` | `anyánk` | `anyátok` | `anyjuk` | `anyáim` |
  | `bátya` | `bátyám` | `bátyád` | `bátyja` | `bátyánk` | `bátyátok` | `bátyjuk` | `bátyáim` |
  | `néne` | `néném` | `nénéd` | `nénje` (archaic) ~ `nénéje` | `nénénk` | `nénétek` | `nénjük` | `nénéim` |
  | `öcs` (bound) | `öcsém` | `öcséd` | `öccse` | `öcsénk` | `öcsétek` | `öccsük` | `öcséim` |
  | `fiú` 'son' | `fiam` | `fiad` | `fia` | `fiunk` | `fiatok` | `fiuk` | `fiaim` |
- **Examples**: `apja` [ATT 15], `anyja` [ATT 16], `anyjuk` [ATT 7], `apjuk` [ATT 7], `bátyja` [ATT 10], `öccse` [ATT 9], `öcsém` [ATT 8], `fia` [ATT 17], `fiam` [ATT 13], `fiai` [ATT 12].
- **Exceptions**:
  - `fiú`: non-possessed plural `fiúk` 'boys' [ATT 14] vs `fiak` 'sons' [ATT 4]; `fiúja` 'her boyfriend' [ATT 0] vs `fia` 'her son'.
  - Non-kinship `apa`/`anya` compounds follow the head (`nagyapja`, `nagyanyja`, `anyósa` regular).
  - `öcs` has no free form; citation `öcs` is lexicographic.
  - `húg` is regular (`húgom`, `húga`).
- **Predictability**: `lexical` (6 stems)
- **Lexical input**: `poss_alt` table.
- **Interactions**: `NOM-POSS-001`, `NOM-ASSOC-001` (`anyámék`).
- **Sources**: [emMorph stemalt.rul kinship and `öcs`/`báty` rules, study only]; [RST24 fn. 35 (`fi-a-m`)]; [ATT].
- **Validation**: `attestation`, `oracle:hunmorph-foma`.
- **Confidence**: `consensus`; `tendency` for `néne`.

### NOM-STEM-015 The száj type
- **Statement**: `száj` uses the alternant `szá-` before the consonant-initial possessive suffixes `-m`, `-d`, `-nk`, `-tok` and the regular stem elsewhere.
- **Conditions**: lexical item `száj` (also in compounds `-száj`).
- **Output**: `szám`, `szád`, `szája`, `szánk`, `szátok`, `szájuk`; possessed plural `szájaim` etc.; non-possessive `szájak`, `szájat`, `szájon`, `szájba`.
- **Examples**: `szája` [ATT 11], `szájába` [ATT 9], `szánkba` [ATT 2], `szájukba` [ATT 3]; `?szájam` is colloquial against the standard `szám` (1,488 : 15,453 [WC2]).
- **Exceptions**: `szám` and `szánk` are homographs of `szám` 'number' and `szánk` 'sledge'; analysis must return both.
- **Predictability**: `lexical` (1 stem)
- **Lexical input**: `poss_alt`.
- **Interactions**: `NOM-POSS-001`.
- **Sources**: [emMorph stemalt.rul special alternations, study only]; [ATT].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### NOM-STEM-016 Other irregular nominal stems
- **Statement**: The following items have idiosyncratic alternants not covered by any class and are listed individually.
- **Conditions**: per item, as listed in Output.
- **Output**:
  - `férfi` (and `-fi` in `hadfi`): vowel-final but lowering: plural `férfiak`, possessed plural `férfiai`; accusative `férfit`; harmony vacillates before C-initial suffixes (`férfinak` ~ `férfinek`) [T04 p. 157; RST12 p. 495].
  - `mag`: both alternants are current, with a usage split [WC2; ATT evidence/corpus-variants.md 2026-09-16]. Plural `magok` 42,603 (general, 57%) ~ `magvak` 32,449 (botanical and official; `magvak` is 76% of official-register tokens, while `magok` is 95% of literary tokens); 3sg possessive `magja` 33,860 (general, 82%) ~ `magva` 7,314 (literary and 'kernel, essence'; `magva` is 74% of literary tokens). The two evidence files differ in reading: [`evidence/lexical-lists.md`](evidence/lexical-lists.md) calls it a plain doublet at its granularity, [`evidence/corpus-variants.md`](evidence/corpus-variants.md) finds a register and sense split on the larger register-stratified base; both readings are recorded, the register split being the better-powered one. `magvas`.
  - `olaj`: accusative `olajat` [ATT 13] (lowered) but plural `olajok`; `olajt` [ATT 1] marginal.
  - `derék` (NN antiharmonic): `derekak`, `derekat`, `derekán`; C-initial suffixes vacillate (`deréknak` ~ `deréknek`) [RST12 p. 496].
  - `ő`-based `övé` belongs to `PND`.
  - `tél`, `nyár` adverbial `telente`, `nyaranta` (`NOM-CASE-021`).
- **Examples**: `férfiak` [ATT 18], `férfit` [ATT 15], `olajat` [ATT 13].
- **Exceptions**: `-`
- **Predictability**: `lexical`
- **Lexical input**: per-item alternants.
- **Interactions**: `NOM-LOW-005`, `NOM-PL-003`, `PH-HARM`.
- **Sources**: [T04 §4.1.3 p. 157]; [RST12 pp. 495-497]; [emMorph stemalt.rul `olaj`, study only]; [ATT]; [WC2]; [MTA-HELYES]; [ATT evidence/corpus-variants.md 2026-09-16]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `attestation`, `goldens`; `mag` measured on tag-filtered `[Pl]` / `[Poss.3Sg]` counts plus a seven-bucket register split.
- **Confidence**: `consensus`; `measured` for `mag`, with two readings recorded (plain doublet at type granularity; register- and sense-conditioned on the register-stratified base).

### NOM-STEM-017 Inflected forms as stems
- **Statement**: Any inflected form ending in a slot-1..5 suffix is a new stem for the next suffix; such stems are non-alternating and lowering.
- **Conditions**: stem = output of `-k`, `-i`, a person suffix, `-é`, `-ék`, or comparative `-bb`.
- **Output**: no further stem alternation; lowered linking vowel before accusative and superessive does not apply (superessive never lowers), before accusative does.
- **Examples**: `gáz` -> `gázok` -> `gázokat`; `bűn` -> `bűnöm` -> `bűnömet`; `ház` -> `házam` -> `házamat`; `kert` -> `kertjeim` -> `kertjeimet`; `nagy` -> `nagyobb` -> `nagyobbat`; `Péterék` -> `Péteréket`; `házaim` -> `házaimon`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LOW-002`, `NOM-ACC-010`.
- **Sources**: [T04 §4.1.3 (17)-(19) pp. 158-161].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-STEM-018 Normative stem doublets
- **Statement**: For a listed set of alternating stems the norm accepts both the alternating and the non-alternating suffixed form as correct; generation emits the preferred variant, analysis and spell-checking accept both.
- **Conditions**: stem listed below; suffix in the class where the doublet is licensed.
- **Output**:
  | Stem | Doublet | Preferred (attestation) |
  |---|---|---|
  | `borjú` | `borjak` ~ `borjúk`; `borjat` ~ `borjút` | `borjak` [ATT 5] vs `-`; `borjút` [ATT 3] vs `-` |
  | `varjú` | `varjak` ~ `varjúk` | `varjak` [ATT 6] |
  | `zászló` | `zászlaja` ~ `zászlója` | `zászlaja` [ATT 7] vs [ATT 4] |
  | `erdő` | `erdeje` ~ `erdője` | `erdeje` [ATT 3] |
  | `szó` | `szavak` ~ `szók` | `szavak` [ATT 15] vs [ATT 3] |
  | `falu` | `falvak` ~ `faluk` | `falvak` [ATT 11] vs [ATT 3] |
  | `lé` 'juice' | `levet` ~ `lét` | `levet` [ATT 8] (`lét` [ATT 14] is mostly `lét` 'existence') |
  | `tó` | `tavat` ~ `tót` | `tavat` [ATT 9] (`tót` [ATT 9] is mostly the ethnonym) |
- **Examples**: see Output.
- **Exceptions**: the doublets do not extend to other members of the same class (`lovat` only, `*lót` as the accusative of `ló`; `faluja` only, `*falva` as the possessive of `falu`; both strings are words elsewhere, the verb `lót` and the converb of `fal`, and `-falva` in place names).
- **Predictability**: `lexical` (8 stems named by AkH12 with `stb.`; further doublets per stem rule `NOM-STEM-010`..`NOM-STEM-013`)
- **Lexical input**: per-stem variant list with preference.
- **Interactions**: `NOM-STEM-010`..`NOM-STEM-013`, `NOM-ACC-001`, `PH-VSTEM-002`, `PH-VSTEM-005`.
- **Sources**: [AkH12 §30]; [SMNy3 Rebrus (136) p. 759, (148) p. 766, (180) p. 781]; [ATT].
- **Validation**: `attestation`, `goldens` (both variants accepted).
- **Confidence**: `normative`.

---

## LOW: lowering

### NOM-LOW-001 Definition of lowering and lowerable suffixes
- **Statement**: Lowering is a lexical property of the stem (trait `lowering`). After a stem carrying it, the linking vowel of a lowerable suffix is low `A` (`a`/`e`) instead of mid `O` (`o`/`e`/`ö`), and a lowered linking vowel never undergoes rounding harmony. **The default for any stem without the trait - including every new word, loan and proper name - is the non-lowering linking vowel `-ot`/`-et`/`-öt`**; this is the unknown-word fallback for `NOM-ACC` and `NOM-PL` (`never lower`).
- **Conditions**:
  - Lowerable suffixes (conformist): plural `-(O)k`, accusative `-(O)t`, possessive 1sg `-(O)m`, 2sg `-(O)d`, 2pl `-(O)tOk`, distributive `-(O)nként`, sociative `-(O)stUl`, adverbial `-(O)n` of adjectives, distributive-temporal `-(O)ntA`; derivational `-(O)s`, diminutive `-(O)cskA`, verbalisers `-(O)z`, `-(O)l` (`DER`).
  - Not lowerable: superessive `-(O)n` (always mid: `házon`, `fülön`, `méz` -> `mézen`, `szűz` -> `szűzön`); stable-vowel suffixes (`-ért`, `-ig`, `-é`, `-ék`, `-Ul`).
  - Always low regardless of stem (self-lowering): comparative `-(A)bb` (`NOM-ADJ-004`), privative `-tlAn` (`DER`); verbal self-lowering suffixes in `VRB`.
  - Always high: possessive 1pl `-(U)nk`, 3pl `-(U)k` (`NOM-POSS-003`, `NOM-POSS-008`).
- **Output**:
  | Stem harmony | non-lowering | lowering |
  |---|---|---|
  | `B` | `o` (`gáz-ok`) | `a` (`ház-ak`) |
  | `R` | `ö` (`bűz-ök`, `rög-öt`) | `e` (`szüz-ek`, `szög-et`) |
  | `F` | `e` (`gyep-ek`) | `e` (`kép-ek`) (quality identical in standard; visible only through `NOM-LOW-003`) |
- **Examples**: `ház` -> `házak`, `házat`, `házam`, `házad`, `házatok`, `házanként`; `gáz` -> `gázok`, `gázt`, `gázom`, `gázotok`; `fül` -> `fülek`, `fület`, `fülem`, `fületek`, but `fülön`; `kő` -> `kövek`, `követek`, but `kövön`.
- **Exceptions**: `-`
- **Predictability**: `rule` (non-lowering default) + `lexical` (membership of the lowering class, `NOM-LOW-007`)
- **Lexical input**: stem property `lowering` (three-valued, `NOM-LOW-007`).
- **Interactions**: `NOM-LOW-002`..`NOM-LOW-008`; `NOM-LINK-002`; `NOM-ACC-005`, `NOM-PL-002` (unknown-word fallback: never lower); `PH-HARM` (rounding harmony).
- **Sources**: [T04 §4.1.3 (12)-(20) pp. 153-162]; [RST24 §3 (3) pp. 144-145]; [SMNy3 Kiefer p. 503; Rebrus p. 800]; [UM-hun `fül`, `kő`]; [WC2]; [ATT evidence/productivity-tagset.md 2026-09-16].
- **Validation**: `goldens`, `oracle:UniMorph`, `attestation`; over 805 back-harmony consonant-final loans with foreign orthography, absent from the lexicon and not headed by a codified lowering noun, the lowering accusative occurs in 10 types, all lemmatisation artefacts (Tolerance `theta` = 120, so `e/theta` = 0.083): lowering share 1.2% against 21.1% in the codified vocabulary. Earlier candidate sets that still showed apparent lowering in loans were contaminated by compounds whose **head** is a codified lowering noun (`webáruház`, `eszköztár`, `nagykörút`), where the head does the lowering exactly as the grammar predicts.
- **Confidence**: `consensus` (RST24 reanalyses lowering as paradigm classes; surface predictions identical); the non-lowering default for new words is `measured`.

### NOM-LOW-002 Inflected stems are lowering
- **Statement**: Every stem ending in an inflectional suffix (plural, possessive, possessed plural, anaphoric `-é`, associative `-ék`, comparative) is lowering.
- **Conditions**: stem output of `NOM-STEM-017`.
- **Output**: lowered linking vowel before a following lowerable suffix (in nominal inflection only the accusative can follow).
- **Examples**: `gázok` -> `gázokat` (`*gázokot`); `bűnöm` -> `bűnömet` (`*bűnömöt`); `tökök` -> `tököket`; `szobrom` -> `szobromat`; `Péterék` -> `Péteréket`; `jobb` -> `jobbat`.
- **Exceptions**: derivational suffixes follow `NOM-LOW-006`, not this rule (`hatod` -> `hatodot` 'sixth-ACC').
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-ACC-010`, `NOM-STEM-017`.
- **Sources**: [T04 (19a) p. 161 and pp. 158-160]; [RST24 fn. 12].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-LOW-003 Detecting lowering in front unrounded stems
- **Statement**: For `F` stems lowering does not change the linking vowel quality but still forces a linking vowel before the accusative where a bare `-t` would otherwise be licit.
- **Conditions**: stem harmony `F`; final consonant in the licit set of `NOM-ACC-003`.
- **Output**: lowering -> `-et`; non-lowering -> `-t`.
- **Examples**: `tej` -> `tejet` vs `bej` -> `bejt`; `mész` -> `meszet` vs `rész` -> `részt`; `méz` -> `mézet` vs `rizs` -> `rizst`; `hely` -> `helyet`; `fej` -> `fejet`; `szén` -> `szenet` vs `fény` -> `fényt`; `tehén` -> `tehenet`.
- **Exceptions**: for `F` stems ending in a consonant that requires a linking vowel anyway (`kép` -> `képet`), lowering status is not recoverable from standard forms; the lexicon may leave it `undetermined` and generation is unaffected [Tab21 §3.3.2].
- **Predictability**: `lexical` (property), `rule` (effect)
- **Lexical input**: `lowering` (may be `undetermined` for `F` stems).
- **Interactions**: `NOM-ACC-005`.
- **Sources**: [T04 (13) p. 154, fn. 31 p. 158]; [Tab21 §3.3.2 pp. 17-18].
- **Validation**: `attestation` (`tejet` [ATT 13] vs `tejt` `-`; `mézet` [ATT 11]; `helyet` [ATT 18]; `rizst` [ATT 10]).
- **Confidence**: `consensus`.

### NOM-LOW-004 Adjectives are lowering by default
- **Statement**: Adjectives are lowering unless listed as exceptions; the listed exceptions share properties that make exceptions predictable in part.
- **Conditions**:
  - `pos=adjective` (including participles in `-(V)tt`/`-t` used adjectivally, and derived adjectives in `-s`, `-ú/-ű`, `-i`, `-ékony`, `-beli`, `-szerű`, `-nemű`).
  - Non-lowering exceptions (back consonant-final adjectives): `nagy` (`nagyok`, and uniquely `nagyobb`), `agg` (`aggok`) [T04 fn. 37], and, by attestation only (no source lists them), a group ending in `g`/`d`/`k`/`l` that are frequently used as nouns: `fiatal` (`fiatalok`, `fiatalt`), `gazdag` (`gazdagok`, `gazdagot`), `boldog` (`boldogok` ~ `?boldogak`), `vastag` (`vastagok`), `szabad` (`szabadok`, `szabadot`), `vak` (`vakok`, `vakot`), `bolond` (`bolondok`), `gonosz` (`gonoszok` ~ `gonoszak`).
  - Privative `-(V)tlAn/-talan/-telen` adjectives are non-lowering (`tanulatlanok`, `tanulatlant`).
  - Ethnonyms and language names used as nouns are non-lowering (`magyarok`, `angolok`, `oroszok`, `törökök`).
  - Nominalised adjectives with a lexicalised noun sense are non-lowering in that sense (`vörösök` 'communists' vs `vörösek` 'red ones'; `komikusok` 'comedians' vs `komikusak`; `szárnyasok` 'poultry' vs `szárnyasak` 'winged'; `piros` 'hearts (cards)' `pirosok`, `pirost` vs adjective `pirosak`, `pirosat`).
- **Output**: lowered linking vowel before plural, accusative, adverbial `-n`: `pirosak`, `pirosat`, `pirosan`; front rounded adjectives show `-ek`/`-et`/`-en` (`bölcsek`, `dühösek`, `erősek`, `öregek`, `szűkek`) except nominalised ethnonyms (`törökök`).
- **Examples**: `gyorsak` [ATT 8], `magasak` [ATT 9], `rosszak` [ATT 9], `újak` [ATT 9], `finomak` [ATT 7], `vékonyak` [ATT 4], `komolyak` [ATT 4], `igazak` [ATT 10], `hamisak` [ATT 5], `fáradtak` [ATT 7], `halottak` [ATT 11], `zártak` [ATT 10]; `nagyok` [ATT 12] vs `nagyak` `-`; `fiatalok` [ATT 18] vs `fiatalak` `-`; `gazdagok` [ATT 11]; `vakok` [ATT 10]; `szabadok` [ATT 7]; `vastagok` [ATT 3]; `boldogok` [ATT 11] vs `?boldogak` [ATT 1] (`boldogok` 90,431 : `boldogak` 3,534 [WC2]); `gonoszok` [ATT 7] ~ `gonoszak` [ATT 4].
- **Exceptions**: the non-lowering list is open-ended in the literature; SMNy3/T04 name only `nagy`, `agg`; emMorph uses the heuristic "C-final adjectives lowering except final `g`, `k`, `l`, `n`" [emMorph stemalt.rul, study only], which the attestation above supports for `g` and `l` but not as a rule (`halkak`, `finomak`). Implementation: default lowering; exceptions stored with class size - **184 non-lowering adjectives** measured over 1,597 candidates (`OQ-03-004` resolved). The **final-`g`/`d`/`k`/`l` phonological predictor is not supported**: `halk`, `finom`, `vastag` and `gazdag` behave differently from each other. The class is overwhelmingly semantic - about 120 of the 184 are ethnonyms and language names (`magyar angol orosz török görög holland horvát lengyel ...`), confirming the generalisation below - plus the sources' `nagy` and `agg` and the attested `fiatal vak szabad bolond józan hajadon bitang pazar`. `boldog` comes out `variable`, matching the rule's `boldogok ~ ?boldogak`.
- **Predictability**: `rule` (default lowering) + `lexical` (the 184-item exception list), with a **semantic** generalisation over that list - ethnonyms, language names and nominalised adjectives - and no phonological one.
- **Lexical input**: `lowering=false` override for adjectives; sense-level `pos`; the list is `nonlowering-adjectives.tsv`, 184 non-lowering (3 variable, 381 lowering, 392 in review), in `local/lists/nonlowering-adjectives.tsv` - working data, not yet read by the build.
- **Interactions**: `NOM-ADJ-001`, `NOM-ADJ-010`, `NOM-ACC-005`.
- **Sources**: [T04 (19b) p. 161, fn. 37, (14)-(15) pp. 155-156]; [RST24 fn. 12, fn. 14]; [Tab21 p. 17]; [emMorph, study only]; [UM-hun `piros`]; [ATT]; [WC2]; [hunmorph-foma]; [kaikki]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `attestation`, `oracle:hunmorph-foma`; oracle agreement on corpus-decided rows: morphdb 484/537, our `.ron` 422/537, foma 46/52, hunspell 65/66, UniMorph 240/256, kaikki 277/287.
- **Confidence**: `consensus` for the default; `measured` for the exception list (184 items); the phonological predictor is **withdrawn**.

### NOM-LOW-005 Stem classes that are always lowering
- **Statement**: Nominal v-stems, shortening stems, bisyllabic v-alternating and ú-deleting stems are lowering; epenthetic stems are unconstrained.
- **Conditions**: flags `v_stem`, `shortening`, `NOM-STEM-011`, `NOM-STEM-012` membership.
- **Output**: `lowering=true`.
- **Examples**: `ló` -> `lovak`; `kő` -> `kövek`; `madár` -> `madarak`; `kéz` -> `kezek`; `tűz` -> `tüzek`; `falu` -> `falvak`; `borjú` -> `borjak`; epenthetic: `marok` -> `markom` (non-lowering) vs `farok` -> `farkam` (lowering); `bokor` -> `bokrok` (non-lowering) vs `hatalom` -> `hatalmak` (lowering).
- **Exceptions**: vowel-final nouns are otherwise non-lowering except `férfi`, `-fi` (`NOM-STEM-016`) and `fiú` 'son' possessives (`fiam`).
- **Predictability**: `rule`
- **Lexical input**: stem class flags.
- **Interactions**: `NOM-STEM-005`, `NOM-STEM-007`, `NOM-STEM-010`.
- **Sources**: [T04 p. 161, fn. 38, p. 157]; [SMNy3 Rebrus p. 760, p. 800].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-LOW-006 Derivational suffixes and lowering
- **Statement**: A stem ending in a derivational suffix is lowering iff the suffix forms adjectives, with two lexical exceptions.
- **Conditions**:
  - Adjective-forming -> lowering: `-(V)s` (`sósak`, `sósat`), `-ú/-ű`, `-jú/-jű` (`nagy lábúak`), `-i` (`városiak`), `-beli` (`falubeliek`), `-szerű` (`újszerűek`), `-nemű` (`egyneműek`), `-ékony` (`tanulékonyak`), `-(V)tt`/`-t` participle (`írottat`, `fáradtak`), `-Ó` participle (optional linking, `NOM-PL-003`).
  - Adjective-forming but non-lowering: privative `-(V)tlAn`, `-talan/-telen` (`tanulatlanok`).
  - Not adjective-forming but lowering: `-van/-ven` numerals (`nyolcvanat`), `-alom/-elem` (`hatalmat`).
  - Noun-forming -> non-lowering: `-ság/-ség` (`barátságok`), `-ás/-és` (`írások`, `írást`), noun `-(V)s` (`harcosok`), fraction `-Od` (`hatodot`), `-ász/-ész` (`halászok`), `-da/-de`, `-at/-et` (`javaslatok`), `-mány/-mény` (`eredmények`).
- **Output**: lowering value for the derived stem.
- **Examples**: `só` -> `sós` -> `sósak`; `harc` -> `harcos` (noun) -> `harcosok`; `ír` -> `írott` -> `írottak`; `ír` -> `írás` -> `írások`.
- **Exceptions**: see Conditions.
- **Predictability**: `rule`
- **Lexical input**: derivational suffix identity (`DER`).
- **Interactions**: `DER-*`, `NOM-PL-003`.
- **Sources**: [T04 pp. 160-161, fn. 36]; [SMNy3 Kiefer pp. 502-503]; [emMorph stemalt.rul lowering rules, study only].
- **Validation**: `goldens`, `oracle:hunmorph-foma`.
- **Confidence**: `consensus`.

### NOM-LOW-007 Lexical lowering of monomorphemic nouns
- **Statement**: Among consonant-final monomorphemic nouns lowering is a lexical property of a closed class; new nouns and loans are non-lowering.
- **Conditions**: `pos=noun`, consonant-final, not covered by `NOM-LOW-005`.
- **Output**: lexicon value, **three-valued** true / false / variable; default false. The variable value must name the paradigm cell it applies to, because the corpus separates the plural cell from the accusative cell for a small group (`öröm`: `örömök` 19,728 but `örömet` 138,399 : `örömöt` 77,324; likewise `kor`, `viszony`, `tulajdon`, `vagyon`, `panasz`, `rokon`, `kölcsön`, `szektor`, `fázis`, `bál`, `humor`, `labor`, `áll`, `ász` - about 25 nouns). A two-valued flag generates either a wrong plural or a wrong accusative for every one of them.
- **Examples**: see `NOM-LOW-008`.
- **Exceptions**: phonological correlates are weak: all nouns in `-rj` and `-rny` are lowering (`fürjet`, `sarjat`, `árnyat`, `szárnyat`, `szörnyet`), `-rzs` too (`törzset`); counterexamples to cluster markedness: `akt` -> `aktot`, `copf` -> `copfot`, `taps` -> `tapsot`, `barack` -> `barackot`, `üdv` -> `üdvöt` vs `kedv` -> `kedvet`, `zsold` -> `zsoldot` vs `hold` -> `holdat`, `föld` -> `földet`; all CCC-final nouns non-lowering (`mumpszot`, `dunsztot`, `karsztot`) [T04 pp. 161-162].
- **Predictability**: `lexical`; class size: 119 lowering vs 1599 non-lowering (6.9%) among consonant-final monomorphemic nouns without variation in Papp (1969), plus a small variable group [Tab21 Table 9]. Measured independently over 4,812 candidates: **155 lowering (139 noun-only lemmas), 23 variable, 2,821 non-lowering**, 83 in review; the 139 noun-only figure is close to Tab21's 119 on a different lexicon base, which is reassuring.
- **Lexical input**: `lowering` (three-valued, with the cell named for `variable`); the list is `lowering-nouns.tsv`, 155 members plus 23 variable, in `local/lists/lowering-nouns.tsv` - working data, not yet read by the build.
- **Interactions**: `NOM-POSS-007` (lowering predicts `-a`).
- **Sources**: [T04 §4.1.3 pp. 160-162]; [Tab21 §3.2 Table 9 p. 14]; [SMNy3 Rebrus (213) p. 801]; [WC2]; [morphdb]; [hunmorph-foma]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `attestation`, `oracle:hunmorph-foma`; oracle agreement on corpus-decided rows: morphdb 2610/2667, our `.ron` 2610/2667, foma 2330/2332, hunspell 2273/2278, UniMorph 1377/1404, kaikki 1588/1612.
- **Confidence**: `consensus`; class size and the three-valued flag `measured`.

### NOM-LOW-008 Common lowering nouns (goldens)
- **Statement**: The following frequent lowering and minimal-pair non-lowering nouns are goldens for the lexicon.
- **Conditions**: lexical.
- **Output**:
  | Harmony | Lowering (plural) | Non-lowering (plural) |
  |---|---|---|
  | `B` | `ház` `házak`, `fal` `falak`, `hal` `halak`, `had` `hadak`, `vad` `vadak`, `fog` `fogak`, `ág` `ágak`, `agy` `agyak`, `ágy` `ágyak`, `vágy` `vágyak`, `tárgy` `tárgyak`, `gyár` `gyárak`, `vár` `várak`, `ár` `árak`, `tál` `tálak`, `szál` `szálak`, `sál` `sálak`, `lyuk` `lyukak`, `hold` `holdak`, `vas` `vasak`, `máj` `májak`, `száj` `szájak`, `nyáj` `nyájak`, `haj` `hajak`, `talp` `talpak`, `váll` `vállak`, `toll` `tollak`, `ujj` `ujjak`, `gally` `gallyak`, `árny` `árnyak`, `szárny` `szárnyak`, `sarj` `sarjak`, `zár` `zárak`, `tár` `tárak`, `nyak` `nyakak`, `díj` `díjak`, `szíj` `szíjak`, `íj` `íjak`, `héj` `héjak`, `száz` `százak`, `láz` `lázak`, `oldal` `oldalak` | `gáz` `gázok`, `dal` `dalok`, `bál` `bálok`, `lap` `lapok`, `zsák` `zsákok`, `nap` `napok`, `hang` `hangok`, `csont` `csontok`, `pad` `padok`, `kár` `károk`, `pár` `párok`, `bor` `borok`, `sor` `sorok`, `kor` `korok`, `kar` `karok`, `baj` `bajok`, `olaj` `olajok`, `sír` `sírok`, `cél` `célok`, `hús` `húsok`, `húr` `húrok`, `csók` `csókok`, `ok` `okok`, `fok` `fokok`, `hab` `habok`, `rab` `rabok`, `tag` `tagok`, `kín` `kínok` |
  | `R` | `fül` `fülek`, `föld` `földek`, `szög` `szögek`, `ügy` `ügyek`, `völgy` `völgyek`, `tölgy` `tölgyek`, `hölgy` `hölgyek`, `öv` `övek`, `tőgy` `tőgyek`, `törzs` `törzsek`, `szörny` `szörnyek`, `fürj` `fürjek`, `ölyv` `ölyvek`, `könny` `könnyek`, `szűz` `szüzek`, `tűz` `tüzek` | `bőr` `bőrök`, `sör` `sörök`, `köd` `ködök`, `tök` `tökök`, `bűn` `bűnök`, `bűz` `bűzök`, `gőz` `gőzök`, `hős` `hősök`, `kör` `körök`, `gyöngy` `gyöngyök`, `öröm` `örömök`, `kürt` `kürtök`, `üst` `üstök`, `fürt` `fürtök`, `rög` `rögök`, `gyümölcs` `gyümölcsök` |
  | `F` (acc. test) | `tej` `tejet`, `fej` `fejet`, `hely` `helyet`, `méz` `mézet`, `mész` `meszet`, `szem` `szemet` (undetectable), `mell` `mellet`, `szenny` `szennyet` | `bej` `bejt`, `rész` `részt`, `rizs` `rizst`, `fény` `fényt`, `vér` `vért`, `kés` `kést` |
- **Examples**: all plurals above attested except `fürjek`, `ölyvek`, `tőgyek`, `nyakak` (`-`), which are standard by [T04] and dictionaries; `kár` is non-lowering (`károk`, `kárt`, `kára`) [Tab21 (4)], `*kárak`.
- **Exceptions**: lowering nouns with bare accusative: `oldal` (`oldalak`, `oldalt`; confirmed hard - `*oldalat`, `oldalt` 272,452 : `oldalat` 1,066 [WC2], rejected by the spelling checker), `szakáll` (`szakállak`, `szakállt` 2,773 ~ `szakállat` 2,988, a genuine doublet, both legal); **`báj` is no longer a member** (`bájat` 127 : `bájt` 10, both legal) (`NOM-ACC-007`).
- **Predictability**: `lexical`
- **Lexical input**: `lowering`; every item in the table above is confirmed by the induction of `NOM-LOW-007`, so the table stands as goldens unchanged.
- **Interactions**: `NOM-LOW-007`, `NOM-ACC-007`.
- **Sources**: [T04 (13), (17), fn. 23 pp. 154-158]; [SMNy3 Kiefer p. 506]; [ATT]; [WC2]; [MTA-HELYES]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `attestation`, `oracle:hunmorph-foma`; every listed golden re-derived from the corpus 2026-09-16.
- **Confidence**: `consensus`; goldens `measured`.

---

## LINK: linking vowels

### NOM-LINK-001 Linking vowel types of nominal suffixes
- **Statement**: Every vowel-initial nominal suffix is classified by the behaviour of its initial vowel: unstable Type A (present after any consonant, absent after vowels), unstable Type B (present only if the consonant cannot be syllabified), or stable (always present).
- **Conditions**:
  | Suffix | Template | Type | Quality |
  |---|---|---|---|
  | plural | `-(O)k` | A | `O`, lowerable |
  | accusative | `-(O)t` | B | `O`, lowerable |
  | superessive | `-(O)n` | A | `O`, never lowered |
  | possessive 1sg | `-(O)m` | A | `O`, lowerable |
  | possessive 2sg | `-(O)d` | A | `O`, lowerable |
  | possessive 1pl | `-(U)nk` | A | `U` |
  | possessive 2pl | `-(O)tOk` | A | `O`, lowerable (second vowel harmonic `O`) |
  | possessive 3sg | `-(j)A` | own allomorphy (`NOM-POSS-004`) | `A` |
  | possessive 3pl | `-(j)Uk` | own allomorphy | `U` |
  | possessed plural | `-(jA)i-` | stable after the `-(j)A` part | - |
  | distributive | `-(O)nként` | A | `O`, lowerable |
  | sociative | `-(O)stUl` | A | `O`, lowerable |
  | distributive-temporal | `-(O)ntA` | A | `O`, lowerable |
  | comparative | `-(A)bb` | A | always `A` |
  | adverbial | `-(A)n` / `-(O)n` | A | lowerable |
  | stable-vowel suffixes | `-ért`, `-ig`, `-é`, `-ék`, `-Ul`, `-i` (DER) | stable | fixed |
- **Output**: see `NOM-LINK-002`..`NOM-LINK-005`.
- **Examples**: `lány` -> `lányok`, `lányon`, `lányom`, but `lányt`; `holló` -> `hollók`, `hollón`, `hollóm`, `hollót`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LOW-001`, `NOM-ACC-*`, `PH-HARM`.
- **Sources**: [T04 §4.1.2.2 (5)-(8) pp. 146-152]; [SMNy3 Rebrus §13.1.1, (2) pp. 661-666]; [RST24 (3) p. 145].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-LINK-002 Quality of the mid linking vowel
- **Statement**: A non-lowered `O` linking vowel is `o` after `B` stems, `e` after `F` stems, `ö` after `R` stems; lowering replaces it by `a`/`e` (`NOM-LOW-001`).
- **Conditions**: stem harmony from `PH-HARM`; for vacillating stems (`B` + neutral vowels) the stem's per-suffix-class harmony value applies.
- **Output**: per table in `NOM-LOW-001`.
- **Examples**: `bolt` -> `boltok`; `kert` -> `kertek`; `fürt` -> `fürtök`; `gyümölcs` -> `gyümölcsök`, `gyümölcsöm`, `gyümölcsötök`; `fotel` -> `fotelek` ~ `?fotelok`; `haver` -> `haverok`, `havernak` (back throughout); `partner` -> `partnerek`, `partnernek` (front throughout).
- **Exceptions**: RST24 p. 159 reports a linking vowel whose harmony differs from that of C-initial suffixes (`haver-o-k` but `haver-nak/nek`; `partner-e-k` but `partner-nak/nek`); the minority C-initial forms are marginal (`?havernek` 161 : `havernak` 3,259; `?partnernak` 188 : `partnernek` 19,802 [WC2]), so these stems keep one harmony; lexicon stores harmony per suffix type (`linking` vs `suffix`) for `B`+`e` stems.
- **Predictability**: `rule` given harmony; `lexical` for vacillating loans
- **Lexical input**: `harmony` (optionally split `harmony_link`, `harmony_sfx`).
- **Interactions**: `PH-HARM`, `NOM-LOW-001`.
- **Sources**: [SMNy3 Kiefer p. 503]; [RST24 p. 159]; [F13]; [ATT `partnerek` 14, `partnerok` `-`, `fotelek` 3].
- **Validation**: `attestation`, `goldens`.
- **Confidence**: `consensus`; `tendency` for vacillating stems.

### NOM-LINK-003 No linking vowel after vowel-final stems
- **Statement**: Type A and B linking vowels are absent after a vowel-final stem.
- **Conditions**: stem ends in a vowel (after lengthening where applicable).
- **Output**: suffix consonant attaches directly.
- **Examples**: `hajó` -> `hajók`, `hajót`, `hajón`, `hajóm`, `hajód`, `hajónk`, `hajótok`, `hajónként`, `hajóstul`; `kocsi` -> `kocsik`, `kocsim`, `kocsink`, `kocsitok`; `teknő` -> `teknőtök`; `alma` -> `almák`, `almánk`, `almátok`; `kapu` -> `kapunk`.
- **Exceptions**: lowering vowel-final adjectives and `férfi` take a linking vowel before plural and adverbial `-n` (`NOM-PL-003`, `NOM-ADJ-010`); `fiú` 'son' possessives (`fiam`, `fiad`, `fiatok`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-PL-003`, `NOM-STEM-014`.
- **Sources**: [T04 (5), (7a) pp. 146-147]; [SMNy3 Rebrus p. 799].
- **Validation**: `goldens`, `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-LINK-004 Linking vowel after consonant-final stems (Type A)
- **Statement**: After a consonant-final stem the Type A linking vowel is always present, independent of the final consonant or cluster.
- **Conditions**: stem ends in a consonant; suffix of Type A.
- **Output**: `O`/`A`/`U` per `NOM-LINK-001`, `NOM-LINK-002`, `NOM-LOW-001`.
- **Examples**: `dal` -> `dalok` (although `dalt`); `bor` -> `borok`; `lány` -> `lányom`; `pénz` -> `pénzek`, `pénzen`, `pénzem`; `fájl` -> `fájlok`, `fájlom`; `stressz` -> `stresszek`, `stresszen`; `kert` -> `kertek`, `kerten`, `kertem`, `kertünk`, `kertetek`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-ACC-002` (Type B contrast).
- **Sources**: [T04 (5)-(6) pp. 146-147].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-LINK-005 Second vowel of -(O)tOk and -(O)stUl
- **Statement**: The suffix-internal vowel of `-tOk` and `-stUl` follows stem harmony; after a lowered linking vowel the `R` rounding of `-tOk` is lost.
- **Conditions**: 2pl possessive `-(O)tOk`; sociative `-(O)stUl`.
- **Output**:
  - `B`: `-otok`/`-atok`, `-ostul`/`-astul`; `F`: `-etek`, `-estül`; `R` non-lowering: `-ötök`, `-östül`; `R` lowering: `-etek`, `-estül`.
  - After vowel-final stem: `-tok`/`-tek`/`-tök`, `-stul`/`-stül`.
  - Possessed plural 2pl: after `-i-` always `-tok` (`B`) or `-tek` (`F`, `R`); no `-tök` (`gyümölcseitek`, `teknőitek`, `köveitek`).
- **Examples**: `ház` -> `házatok`; `bot` -> `bototok`; `kert` -> `kertetek`; `gyümölcs` -> `gyümölcsötök`; `fül` -> `fületek`; `kő` -> `követek`; `teknő` -> `teknőtök`; `család` -> `családostul`; `cipő` -> `cipőstül`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-HARM`, `NOM-PPL-004`.
- **Sources**: [T04 (18) fn. 32 p. 159]; [RST24 (15) p. 160]; [UM-hun `kő`, `gyümölcs`, `teknő`]; [AkH12 §35].
- **Validation**: `oracle:UniMorph`, `goldens`.
- **Confidence**: `consensus`.

---

## LENG: final a/e lengthening

### NOM-LENG-001 Lengthening of stem-final a and e
- **Statement**: A stem-final short `a` or `e` becomes `á` or `é` before every class I and class II suffix.
- **Conditions**:
  - Stem (or suffix string forming the current stem) ends in `a`/`e`.
  - Next suffix is not super-analytic (`NOM-LENG-002`).
  - Applies to native words, loans, nonce words, blends and lowercase word-read common acronyms (`áfát`, `stukák`, `Alitaliánál`, `ORT-ABBR-016`), but never all-capitals proper acronyms, whether read as a word or letter by letter (`FIFA-nak`, `NATO-t`, `ELTE-n`; `NOM-LENG-003`), and all personal and place names (`Tompát`, `Ráckevén`).
- **Output**: `a` -> `á`, `e` -> `é`.
- **Examples**: `alma` -> `almák`, `almát`, `almán`, `almám`, `almája`, `almái`, `almával`, `almába`, `almáig`, `almáért`, `almáé`, `almául`, `almánként`, `almástul`; `kefe` -> `kefék`, `kefét`, `kefém`, `keféje`, `kefének`, `keféé`, `keféig`, `kefében`; `drága` -> `drágább`; `Varga` -> `Vargáék`, `Vargáé`.
- **Exceptions**: `NOM-LENG-002`, `NOM-LENG-003`.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-STEM-003`; applies again to suffix-final `-jA` (`NOM-LENG-004`).
- **Sources**: [AkH12 §29]; [SMNy3 Rebrus §13.6.1 (187)-(188) pp. 786-788]; [KVF98 p. 193].
- **Validation**: `goldens`, `attestation` (`almával` [ATT 7]).
- **Confidence**: `normative`.

### NOM-LENG-002 Non-lengthening suffixes
- **Statement**: Final `a`/`e` stays short before the super-analytic suffixes.
- **Conditions**: next suffix in the closed list:
  - Inflectional/case-like: `-ként`, `-kor`, `-képp`/`-képpen`, multiplicative `-szor/-szer/-ször` (`PND`).
  - Derivational (`DER`): `-ság/-ség`, adjectival `-i`, `-szerű`, `-féle`, `-ista`, `-izmus`, `-(i)kus`/`-ológia` type Latinate endings.
  - Before possessed plural `-i`: the `A` of `-(j)A` stays short (`lapjai`, `házai`, `kertjei`) (`NOM-PPL-003`); compare stem-final `a` before `-i` which lengthens (`almái`).
- **Output**: no lengthening.
- **Examples**: `alma` -> `almaként`; `óra` -> `órakor`, `óraként`, `óraképpen`; `kefe` -> `kefeként`, `kefeképp`; `buta` -> `butaság`; `Buda` -> `budai`; `apa` -> `apai`; `kerge` -> `kergeség`; `fa` -> `faszerű`, `faféle`.
- **Exceptions**: acronyms never lengthen irrespective of suffix (`NOM-LENG-003`).
- **Predictability**: `lexical` (closed list of suffixes; AkH12 gives only examples, the list is from SMNy3 and KVF98)
- **Lexical input**: suffix property `lengthening=false`.
- **Interactions**: `NOM-MODEL-004`, `NOM-CASE-008`, `NOM-CASE-017`, `NOM-CASE-020`.
- **Sources**: [AkH12 §29 (examples `apai`, `kergeség`)]; [SMNy3 Rebrus (188)-(189) pp. 788-789]; [KVF98 p. 193].
- **Validation**: `goldens`.
- **Confidence**: `consensus` (the norm does not enumerate the list).

### NOM-LENG-003 Stems exempt from lengthening
- **Statement**: Lengthening does not apply in writing to all-capitals proper acronyms, to monosyllabic function words and to monosyllabic `Ce`-shaped names borrowed with function-word prosody.
- **Conditions**:
  - All-capitals proper-noun acronym, whether read letter by letter (`MTA`) or as a word (`ELTE`, `ETO`, `FIFA`): no written lengthening, hyphenated suffix (`ORT-ABBR-014`, `ORT-ABBR-015`); blends not in all capitals and lowercase common acronyms lengthen (`Hungexpónál`, `áfát`).
  - Function words suffixed marginally keep the short vowel (`de` -> `det`, `dek`).
  - Monosyllabic loan names in `-e` (`Che`, `Ce`): `Chet` preferred over `?Chét`; bisyllabic loans lengthen obligatorily (`Ocsét`).
- **Output**: short vowel retained.
- **Examples**: `MTA-t`, `MTA-nak`, `ELTE-re`, `ELTE-n`, `ETO-hoz`; word-like blends lengthen (`Alitaliánál`, `stukák`, `Hungexpónál`).
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: stem flag `acronym_letterwise`, `function_word`.
- **Interactions**: `ORT-ABBR-015`, `ORT-ABBR-016`, `PH-LEN-003`, `NOM-PROP-004`.
- **Sources**: [AkH12 §29, §288 a)-b)]; [SMNy3 Rebrus pp. 790-791].
- **Validation**: `goldens`.
- **Confidence**: `normative` for acronyms; `consensus` for function words and `Ce` names.

### NOM-LENG-004 Lengthening of suffix-final A
- **Statement**: The final `A` of 3sg possessive `-(j)A` lengthens before a following class I or II suffix except possessed plural `-i`; it stays short before class III suffixes.
- **Conditions**: stem = 3sg possessed form.
- **Output**: `-já-`/`-jé-`, `-á-`/`-é-`.
- **Examples**: `lapja` -> `lapját`, `lapjában`, `lapjához`, `lapjáé`; `kertje` -> `kertjét`, `kertjében`; `háza` -> `házát`, `házán`, `házáé`; but `lapjai`, `kertjei`, `házai`, and before super-analytic `-ként`: `tagjaként` [ATT 12], `részeként` [ATT 15], `*tagjáként` [SMNy3 (188); `PH-LEN-002`].
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-POSS-011`, `NOM-PPL-003`.
- **Sources**: [SMNy3 Rebrus §13.6.3.1 (207) pp. 797-798, §13.6.3.4 pp. 802-804]; [RST12 (10c) p. 501].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-LENG-005 Foreign-spelled names and words ending in o or ö
- **Statement**: In foreign-spelled words and names whose final written `a`, `e`, `o`, `ö` is pronounced as a lengthening vowel, the suffixed form is spelled with the long letter; this is a spelling rule consuming the pronunciation.
- **Conditions**: stem written with foreign orthography; final letter `a`/`e`/`o`/`ö` pronounced; suffix not in `NOM-LENG-002` and not `-i`, `-szerű`.
- **Output**: `a`->`á`, `e`->`é`, `o`->`ó`, `ö`->`ő` in the suffixed form.
- **Examples**: `Coca-Cola` -> `Coca-Colát`; `Goethe` -> `Goethének`; `Cicero` -> `Cicerót`; `Oslo` -> `Oslóban`; `Malmö` -> `Malmőből`; but `goethei`, `Coca-Cola-szerű`.
- **Exceptions**: silent final letters trigger a hyphen and no lengthening (`Montreux-ig`), `ORT-FOR-012`.
- **Predictability**: `rule`
- **Lexical input**: pronunciation of final letter (`PH`).
- **Interactions**: `ORT-FOR-010`, `ORT-FOR-012`, `PH-LEN-004`, `NOM-PROP-004`.
- **Sources**: [AkH12 §216 a), §217 a)].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

---

## PL: plural

### NOM-PL-001 Plural after vowel-final stems
- **Statement**: The plural of a vowel-final noun or adjective is stem + `-k`, with final `a`/`e` lengthening.
- **Conditions**: vowel-final stem, not in `NOM-PL-003`, not a v-stem or u/ú->v, ú-deleting stem.
- **Output**: `-k`.
- **Examples**: `hajó` -> `hajók`; `kocsi` -> `kocsik`; `teknő` -> `teknők`; `kapu` -> `kapuk`; `alma` -> `almák`; `kefe` -> `kefék`; `fekete` -> `feketék`; `drága` -> `drágák`; `kávé` -> `kávék`; `burzsoá` -> `burzsoák`; `kicsi` -> `kicsik`.
- **Exceptions**: v-stems (`lovak`), `NOM-STEM-011`, `NOM-STEM-012`, `férfi` -> `férfiak`.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LENG-001`, `NOM-STEM-010`.
- **Sources**: [SMNy3 Kiefer p. 501]; [T04 p. 155].
- **Validation**: `goldens`, `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-PL-002 Plural after consonant-final stems
- **Statement**: The plural of a consonant-final stem is bound/free stem + linking vowel + `k`; the linking vowel is `O` or lowered `A`.
- **Conditions**: consonant-final synthetic stem alternant.
- **Output**: `-ok/-ek/-ök` (non-lowering), `-ak/-ek` (lowering).
- **Examples**: `bolt` -> `boltok`; `kert` -> `kertek`; `fürt` -> `fürtök`; `ház` -> `házak`; `fül` -> `fülek`; `bokor` -> `bokrok`; `madár` -> `madarak`; `ló` -> `lovak`; `toll` -> `tollak`; `orr` -> `orrok`; `stressz` -> `stresszek`; `fájl` -> `fájlok`; `piros` -> `pirosak`; `szép` -> `szépek`.
- **Exceptions**: `-`
- **Predictability**: `rule` given `lowering`
- **Lexical input**: `lowering`, stem alternants.
- **Interactions**: `NOM-LOW-*`, `NOM-STEM-*`.
- **Sources**: [SMNy3 Kiefer pp. 501-503]; [T04 (17) p. 158].
- **Validation**: `goldens`, `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-PL-003 Linking vowel in plurals of vowel-final lowering adjectives
- **Statement**: Vowel-final adjectives ending in a non-low vowel take a low linking vowel before plural `-k`: obligatorily after derived `-i`, `-si`, `-beli`, `-ú/-ű`, `-szerű`, `-nemű` and after monomorphemic `ú`/`ű`-final adjectives; optionally after `ó`/`ő`-final adjectives; never after `a`/`e`/`á`/`é`.
- **Conditions**: `pos=adjective`, vowel-final.
- **Output**:
  | Stem end | Plural | Preference |
  |---|---|---|
  | derivational `-i`, `-si`, `-beli` | `-ak/-ek` | obligatory: `városiak`, `pestiek`, `falusiak`, `falubeliek`, `maiak`, `régiek` |
  | `-ú/-ű`, `-jú/-jű`, `-szerű`, `-nemű` | `-ak/-ek` | obligatory: `nagy lábúak`, `újszerűek`, `egyneműek` |
  | monomorphemic `ú`/`ű` | `-ak/-ek` | obligatory: `hosszúak`, `szomorúak`, `keserűek`, `hűek` |
  | participle `-ó/-ő` | `-k` ~ `-ak/-ek` | both: `bántók` ~ `bántóak`, `sértők` ~ `sértőek` |
  | monomorphemic `ó`/`ő` | `-k` ~ `-ak/-ek` | `-k` preferred: `olcsók` [ATT 5] ~ `olcsóak` [ATT 1]; `aprók` [ATT 5] |
  | monomorphemic `i` | `-k` | `kicsik` [ATT 12] |
  | `a`, `e`, `á`, `é` | `-k` | `feketék`, `drágák`, `ordenárék`, `burzsoák` |
- **Examples**: `hosszúak` [ATT 8] vs `hosszúk` `-`; `szomorúak` [ATT 5] vs `szomorúk` `-`; `városiak` [ATT 3] vs `városik` `-`.
- **Exceptions**: nouns never take the linking vowel (`hajók`, `kapuk`), except `férfi` -> `férfiak`; the same vowel-final adjectives take bare accusative and superessive (`hosszút`, `városit`, `városin`) [T04 (15) p. 156].
- **Predictability**: `rule` for obligatory and exclusion cells; `tendency` for `ó/ő`
- **Lexical input**: derivational structure, `pos`.
- **Interactions**: `NOM-ADJ-001`, `NOM-ADJ-010` (adverbial `-an` behaves in parallel: `szomorúan`, `bántóan`).
- **Sources**: [T04 (14)-(16) pp. 154-157]; [SMNy3 Kiefer (40) pp. 502-503]; [RST24 fn. 35 p. 158]; [ATT].
- **Validation**: `attestation`, `goldens`.
- **Confidence**: `consensus`; `tendency` for preference in `ó/ő`.

### NOM-PL-004 Plural of stems with alternants
- **Statement**: The plural selects the synthetic alternant of epenthetic, shortening, v-stem, u/ú->v and ú-deleting stems.
- **Conditions**: stem flags per `NOM-STEM-005`..`NOM-STEM-012`.
- **Output**: bound alternant + lowered or non-lowered `-Ok`.
- **Examples**: `tükör` -> `tükrök`; `lélek` -> `lelkek`; `kéz` -> `kezek`; `víz` -> `vizek`; `híd` -> `hidak`; `út` -> `utak`; `tűz` -> `tüzek`; `tó` -> `tavak`; `kő` -> `kövek`; `mű` -> `művek`; `falu` -> `falvak` ~ `faluk`; `borjú` -> `borjak`; `kehely` -> `kelyhek`.
- **Exceptions**: see stem rules.
- **Predictability**: `lexical`
- **Lexical input**: stem alternants.
- **Interactions**: `NOM-STEM-*`.
- **Sources**: [SMNy3 Rebrus §13.3-13.5]; [KVF98 p. 194].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-PL-005 Plural in the template
- **Statement**: Non-possessive plural `-k` occupies slot 2 and is followed only by `-é`, `-éi` and case suffixes; possessed plurals use `-i` (`NOM-PPL-001`).
- **Conditions**: plural form.
- **Output**: `házak` + `-é` -> `házaké`; + case -> `házakat`, `házakban`, `házakkal`, `házakká`, `házakként`.
- **Examples**: `városokban`; `székekben`; `házakéi`.
- **Exceptions**: case-like `-(O)nként`, `-(O)stUl`, `-képp(en)` do not follow the plural; `-kor` does only for a recurring event (`ünnepekkor`, `NOM-MODEL-003`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-MODEL-001`, `NOM-ACC-010`, `NOM-CASE-005` (three-letter reduction does not arise: `-kkal` has two `k`).
- **Sources**: [KVF98 pp. 192-193]; [SMNy3 Kiefer §10.3.5].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

---

## POSS: possessive suffixes (singular possessed)

### NOM-POSS-001 Possessive paradigm for a singular possessed noun
- **Statement**: A singular possessed noun takes one of six person/number suffixes; their shape depends on stem-final segment, lowering and harmony.
- **Conditions**: possessor person/number; stem alternant per `NOM-STEM-001`.
- **Output**:
  | Possessor | after vowel | after C, non-lowering `B`/`F`/`R` | after C, lowering `B`/`F`,`R` |
  |---|---|---|---|
  | 1sg | `-m` | `-om`/`-em`/`-öm` | `-am`/`-em` |
  | 2sg | `-d` | `-od`/`-ed`/`-öd` | `-ad`/`-ed` |
  | 3sg | `-ja`/`-je` | `-a`/`-e` or `-ja`/`-je` (`NOM-POSS-004`..`NOM-POSS-007`) | usually `-a`/`-e` |
  | 1pl | `-nk` | `-unk`/`-ünk` | `-unk`/`-ünk` |
  | 2pl | `-tok`/`-tek`/`-tök` | `-otok`/`-etek`/`-ötök` | `-atok`/`-etek` |
  | 3pl | `-juk`/`-jük` | `-uk`/`-ük` or `-juk`/`-jük` (`NOM-POSS-008`) | usually `-uk`/`-ük` |
- **Examples**: `hajó`: `hajóm`, `hajód`, `hajója`, `hajónk`, `hajótok`, `hajójuk`; `kalap`: `kalapom`, `kalapod`, `kalapja`, `kalapunk`, `kalapotok`, `kalapjuk`; `kert`: `kertem`, `kerted`, `kertje`, `kertünk`, `kertetek`, `kertjük`; `gyümölcs`: `gyümölcsöm`, `gyümölcsöd`, `gyümölcse`, `gyümölcsünk`, `gyümölcsötök`, `gyümölcsük`; `ház`: `házam`, `házad`, `háza`, `házunk`, `házatok`, `házuk`; `fül`: `fülem`, `füled`, `füle`, `fülünk`, `fületek`, `fülük`.
- **Exceptions**: stem-specific alternants (`NOM-STEM-013`..`NOM-STEM-015`); `-ja` after `-i`-final stems is regular (`kocsija`, `kocsijuk`).
- **Predictability**: `rule` except the 3rd person `j` choice
- **Lexical input**: `lowering`, `poss3`, stem alternants.
- **Interactions**: `NOM-LENG-001` (`almám`, `kefém`), `NOM-LINK-005`, `NOM-POSS-002`..`NOM-POSS-011`.
- **Sources**: [SMNy3 Kiefer §10.3.3 (41)-(49) pp. 503-507]; [SMNy3 Rebrus (209), (213) pp. 798-801]; [UM-hun].
- **Validation**: `oracle:UniMorph`, `goldens`.
- **Confidence**: `consensus`.

### NOM-POSS-002 Linking vowel of 1sg, 2sg, 2pl possessive
- **Statement**: The linking vowel of `-(O)m`, `-(O)d`, `-(O)tOk` is mid `O` after non-lowering and `A` after lowering stems (possessive suffixes are lowerable).
- **Conditions**: consonant-final stem alternant.
- **Output**: per `NOM-LOW-001`.
- **Examples**: `pad` -> `padom`; `ház` -> `házam`; `ős` -> `ősöm`; `tűz` -> `tüzem`; `bűn` -> `bűnöm`; `szög` -> `szögem`; `ló` -> `lovam`, `lovatok`; `toll` -> `tollam`; `orr` -> `orrom`; `könny` -> `könnyem`, `könnyetek`; `lélek` -> `lelkem`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `lowering`
- **Interactions**: `NOM-LOW-001`, `NOM-LINK-005`.
- **Sources**: [SMNy3 Kiefer (49) p. 507, (47) p. 506]; [T04 p. 158].
- **Validation**: `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-POSS-003 1pl possessive -(U)nk
- **Statement**: The 1pl possessive is `-nk` after vowels and `-unk`/`-ünk` after consonants, irrespective of lowering.
- **Conditions**: 1pl possessor.
- **Output**: `-nk`; `-unk` (`B`), `-ünk` (`F`, `R`).
- **Examples**: `fánk` (homograph of `fánk` 'doughnut'), `almánk`, `kocsink`; `botunk`, `házunk`, `lovunk`, `nyarunk`, `bokrunk`; `kertünk`, `fülünk`, `kövünk`, `lelkünk`.
- **Exceptions**: before `-unk` shortening stems shorten (`utunk`, `hidunk`, `vizünk`) though `u`/`ü` is not low [SMNy3 p. 743].
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-STEM-007`.
- **Sources**: [T04 p. 152]; [SMNy3 Rebrus p. 743, (213) p. 801]; [UM-hun `híd`].
- **Validation**: `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-POSS-004 3sg -ja/-je after vowels
- **Statement**: After a vowel-final stem the 3sg possessive is always `-ja` (`B`) or `-je` (`F`, `R`), with lengthening of final `a`/`e`.
- **Conditions**: vowel-final possessive alternant.
- **Output**: `-ja`/`-je`.
- **Examples**: `hajója`, `kapuja`, `kocsija`, `almája`, `keféje`, `teknője`, `kávéja`, `autója`, `hobbija`, `szerzője`, `vezetője`.
- **Exceptions**: stems with possessive alternants: `ajtaja`, `ideje` (`NOM-STEM-013`); v-stems (`lova`, `köve`, `műve`); `fia`, `apja` (`NOM-STEM-014`); `borja` (`NOM-STEM-012`); `tetve` ~ `tetűje` (`NOM-STEM-011`); but `faluja` (`NOM-STEM-011`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LENG-001`.
- **Sources**: [SMNy3 Kiefer p. 509 rule 1]; [Tab21 Table 7].
- **Validation**: `oracle:UniMorph`, `attestation` (`autója` [ATT 11], `munkája` [ATT 16]).
- **Confidence**: `consensus`.

### NOM-POSS-005 3sg -a/-e after consonants: categorical conditions
- **Statement**: After a consonant-final common noun the 3sg possessive is categorically `-a`/`-e` when the stem ends in a sibilant/affricate or palatal, or in the suffix `-ság/-ség`.
- **Conditions**:
  - Final consonant in `s sz z zs c cs dzs` (sibilants and affricates, including geminates `ssz`, `ccs`) or `j ly gy ny ty` (palatals, including geminates).
  - Stem ends in derivational `-ság/-ség`.
  - `pos` is common noun or adjective used as noun (not proper name, not nonce).
- **Output**: `-a`/`-e`.
- **Examples**: `sas` -> `sasa`; `rész` -> `része`; `kulcs` -> `kulcsa`; `kincs` -> `kincse`; `perec` -> `perece`; `olaj` -> `olaja`; `király` -> `királya`; `bárány` -> `báránya`; `hegy` -> `hegye`; `pötty` -> `pöttye`; `gyümölcs` -> `gyümölcse`; `stressz` -> `stressze`; `könny` -> `könnye`; `ellenség` -> `ellensége`; `királyság` -> `királysága`.
- **Exceptions**: `újság` -> `újságja` ~ `újsága` (lexicalised, not transparent `-ság`); proper names and nonce words do not obey the phonological condition (`Koncz` -> `Konczja`, `Fónagy` -> `Fónagyja`, `Kemény` -> `Keménye` ~ `Keményje`; nonce `bács` -> `bácsja`) (`NOM-PROP-003`).
- **Predictability**: `rule` (sibilant: coefficient -11.07; palatal: -9.25 in [Tab21], effectively categorical)
- **Lexical input**: `pos`, proper-name flag.
- **Interactions**: `NOM-POSS-006`, `NOM-PROP-003`.
- **Sources**: [SMNy3 Kiefer p. 510 rules 2, 4]; [SMNy3 Rebrus p. 801, pp. 805-806]; [Tab21 Tables 7, 17]; [ATT `újsága` 1, `újságja` 2].
- **Validation**: `attestation`, `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-POSS-006 3sg after consonants: tendency model
- **Statement**: Vowel-final stems take `-ja`/`-je` by default (v-stems and some `-ó`/`-ő` stems are lexical exceptions). Consonant-final stems ending in a sibilant or affricate (`s sz z zs c cs dzs`) take `-a`/`-e` **without exception**. For all other consonant-final stems outside `NOM-POSS-005` the choice is lexical and **there is no learnable default**; in the absence of a lexical value, predict with the logistic model of [Tab21 Table 17] and emit `-ja` iff p >= 0.5, treating the output as a ranking hint rather than a generator.
- **Conditions**: consonant-final monomorphemic common noun; no `poss3` in lexicon.
- **Output**: `p(-ja) = 1 / (1 + exp(-z))`, `z` = sum of:
  | Predictor | Value | Coefficient |
  |---|---|---|
  | intercept (plosive, alveolar, back, mid vowel, short, singleton, monosyllabic, non-lowering) | - | +3.53 |
  | stem class | lowering | -3.71 |
  | stem class | variable lowering | -2.76 |
  | final C manner | fricative (`f v h`) | -1.03 |
  | final C manner | sibilant | -11.07 |
  | final C manner | nasal (`m n ny`) | -2.07 |
  | final C manner | approximant (`l r j`) | -4.06 |
  | final C place | labial (`p b m f v`) | -2.22 |
  | final C place | palatal | -9.25 |
  | final C place | velar (`k g`) | -3.54 |
  | harmony | front | -1.98 |
  | harmony | vacillating | +2.25 |
  | last vowel height | high (`i í u ú ü ű`) | +1.85 |
  | last vowel height | low (`a á e`) | +0.77 |
  | last vowel length | long | +1.30 |
  | coda | geminate | +2.43 |
  | coda | cluster | -0.08 (n.s.) |
  | syllables | polysyllabic | +0.79 |
- **Examples** (model vs actual): `bot` z=3.53, p=.97 -> `botja` (actual `botja`); `kalap` z=3.53-2.22+0.77+0.79=2.87, p=.95 -> `kalapja` (actual); `szék` z=3.53-3.54-1.98+1.30=-0.69, p=.33 -> `széke` (actual); `pár` z=3.53-4.06+0.77+1.30=1.54, p=.82 -> `párja` (actual); `ablak` z=3.53-3.54+0.77+0.79=1.55, p=.83 -> `*ablakja` (actual `ablaka`: lexical override needed).
- **Exceptions**: model accuracy is limited and now measured - the [Tab21 Table 17] model reproduces the corpus choice for **68.1% of 2,373 monomorphemic consonant-final nouns** (731 exceptions, enumerated in the list below), with 85 further stems genuinely variable. That is a usable prior but not a generator, so the rule's own "every frequent noun must carry a lexical `poss3`" is confirmed. Rácz & Rebrus (2012), on a web corpus including complex stems, rank place as labial > alveolar > velar for `-ja`, differing from [Tab21] (labial < alveolar) because of comparatives in `-bb` [Tab21 p. 18].
- **Predictability**: `rule` (V-final -> `-ja`; sibilant/affricate-final -> `-a`) + `lexical` (all other C-final stems). Background counts: 2433 monomorphemic noun-only C-final stems, Papp 1969: 59% `-ja` among non-lowering (943/1599), 13% among lowering (15/119) [Tab21 Table 9]; SMNy3 count: non-lowering stems 1107 exceptions to `-ja` out of 2468, lowering stems 26 exceptions to `-a` out of 222 [SMNy3 (213)].
- **Lexical input**: `poss3` (preferred), otherwise the phonological predictors; the enumerated exception set is `poss3-ja-exceptions.tsv`, 2,373 monomorphemic stems (726 `-ja`, 1,562 `-a`, 85 variable), in `local/lists/poss3-ja-exceptions.tsv` - read by `cargo xtask morphdb`. The `PossessiveA` trait is redundant on sibilant/affricate-final stems, where the rule supplies it; keep it for round-tripping but do not require it, and lint `PossessiveJa` on a sibilant/affricate-final stem, since the corpus has zero such items.
- **Interactions**: `NOM-POSS-005`, `NOM-POSS-007`.
- **Sources**: [Tab21 §3.3 Tables 16-17 pp. 32-33]; [SMNy3 Kiefer p. 510]; [SMNy3 Rebrus (213) p. 801]; [WC2]; [ATT evidence/productivity-tagset.md 2026-09-16]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `oracle:hunmorph-foma` (coverage of `poss3`), `attestation`. Webcorpus 2 `[/N][Poss.3Sg][Nom]` types at freq >= 200, Tolerance Principle per candidate set: V-final N = 4,176, `-ja` share 89.7%, e = 430 vs theta = 500 -> **productive**; sibilant/affricate-final C stems N = 4,330, `-ja` share 0.0%, e = 0 vs theta = 517 -> **productive and exceptionless**; other C-final N = 5,888, `-ja` share 18.6%, e = 1,095 vs theta = 678 -> **listed**. The whole-set reading "`-a` is the default for all C-final stems" lands at e = 1,095 vs theta = 1,106, i.e. **inconclusive** by 1.0%, and must not be relied on.
- **Confidence**: `measured` for the V-final and sibilant defaults; `tendency` for the Tab21 model over the non-sibilant residue, which is a gradient effect (18.6% / 81.4%) rather than a rule with exceptions and still needs an MGL or MaxEnt fit.

### NOM-POSS-007 3sg: stem-class and derivational effects
- **Statement**: Stem class and final derivational material override the tendency model as listed.
- **Conditions**: per item, as listed in Output.
- **Output**:
  | Class | Choice | Examples |
  |---|---|---|
  | v-stems, shortening stems, u/ú->v, ú-deleting | `-a`/`-e` | `lova`, `köve`, `műve`, `tava`, `nyara`, `keze`, `vize`, `tüze`, `lelke`, `tetve`, `borja`; but `faluja`, and long-vowel `-ja` forms in the next row |
  | shortening stems with long vowel kept | `-ja` | `hídja`, `kútja`, `lúdja`, `rúdja`, `útja` (`NOM-STEM-007`) |
  | lowering stems (lexical lowering nouns, v-stems and shortening stems counted together) | `-a`/`-e` in 88% (196/222) [SMNy3 (213)] | `háza`, `hala`, `fala`, `tolla`, `foga`, `gyára`; exceptions `holdja`, `vadja` (~ archaic `vada`), `hídja` |
  | epenthetic | `-a`/`-e` | `bokra`, `tükre`, `terme`, `torka`, `farka`, `hatalma`; variants `sarokja` |
  | final CC or geminate (non-sibilant, non-palatal) | `-ja`/`-je`, few exceptions | `kertje`, `csontja`, `galambja`, `gondja`, `rendje`, `pártja`, `sakkja`, `cseppje`, `címzettje`; exceptions `teste`, `cikke`, `kedve`, `nyelve`, `tette` (and palatal-final `völgye` by `NOM-POSS-005`) |
  | `-ság/-ség` | `-a`/`-e` | `NOM-POSS-005` |
  | agentive/Latin `-tor`, `-ter`, `-er`, `-um` | `-a`/`-e` (analogical tendency) | `szenátora`, `doktora`, `minisztere`, `slágere`, `múzeuma`; but `motorja` (non-agentive `-or`) |
  | `-ás/-és`, `-at/-et`, `-zat/-zet`, `-alom/-elem`, `-nok/-nök` | `-a`/`-e` | `lakása`, `kérdése`, `javaslata`, `szervezete`, `birodalma`, `hivatalnoka` |
  | noun `-(V)s`, `-ász/-ész`, `-mány/-mény`, `-vány/-vény` | `-a`/`-e` by `NOM-POSS-005` | `orvosa`, `halásza`, `eredménye` |
  | `-ár/-ér` agentives | `-a`/`-e` | `titkára`, `tanára` |
  | new loans, nonce words | `-ja`/`-je` (productive default) | `szputnyikja`, `röntgenje`, `filmje`, `bankja`, `parkja`, `sportja` |
- **Examples**: `szomszédja` [ATT 8] vs `szomszéda` `-`; `főnöke` [ATT 11] vs `főnökje` `-`; `vonata` [ATT 1] vs `vonatja` `-`; `asztala` [ATT 6]; `ablaka` [ATT 8]; `tanára` [ATT 12]; `motorja` [ATT 10].
- **Exceptions**: the vowel-final default `-ja`/`-je` has a coherent, already-listed exception class - the v-stems and some `-ó`/`-ő` stems take `-a`/`-e`: `szava`, `feje`/`fő`, `lova`, `köve`, `tava`, `műve`, `fia`, and the `-ó`/`-ő` participial stems (`élő`, `következő`) and `jármű` pattern with them. Diachronic change runs only from `-a` to `-ja` (`vada` -> `vadja`, `kerte` -> `kertje`); child language over-uses `-ja` (`asztalja`, `székje`) [SMNy3 Kiefer p. 510].
- **Predictability**: `tendency` (class-level shares as given), `rule` for v/shortening classes
- **Lexical input**: stem class, derivational suffix identity.
- **Interactions**: `NOM-POSS-006`, `DER-*`.
- **Sources**: [SMNy3 Kiefer p. 510 rules 3-5]; [SMNy3 Rebrus (213) p. 801, p. 806]; [Tab21 Table 9]; [ATT]; [WC2]; [ATT evidence/productivity-tagset.md 2026-09-16].
- **Validation**: `attestation`; the sibilant/affricate condition is exceptionless over 4,330 corpus types (`része` 2,331,597, `keze` 172,145, `háza` 232,762 - not one `-ja`-dominant sibilant stem).
- **Confidence**: `consensus` for classes; `tendency` for derivational groups; `measured` for the V-final exception class and the sibilant condition.

### NOM-POSS-008 3pl -uk/-juk follows the 3sg choice
- **Statement**: The 3pl possessive uses `-juk/-jük` exactly when the 3sg uses `-ja/-je`, and `-uk/-ük` when the 3sg uses `-a/-e`.
- **Conditions**: 3pl possessor, singular possessed.
- **Output**: `-uk/-ük` or `-juk/-jük`; `U` never lowers or rounds by `O` rules.
- **Examples**: `lapja` -> `lapjuk`; `hala` -> `haluk`; `lova` -> `lovuk`; `nyara` -> `nyaruk`; `bokra` -> `bokruk`; `kertje` -> `kertjük`; `füle` -> `fülük`; `hajója` -> `hajójuk`; `hídja` -> `hídjuk`; `anyja` -> `anyjuk`; `ajtaja` -> `ajtajuk`; `barátja` -> `barátjuk`.
- **Exceptions**: divergences are sporadic and unsystematic [SMNy3 Kiefer p. 509]; implement per-lemma override `poss3pl`.
- **Predictability**: `rule` with lexical overrides
- **Lexical input**: `poss3`, optional `poss3pl`.
- **Interactions**: `NOM-POSS-005`..`NOM-POSS-007`.
- **Sources**: [SMNy3 Kiefer p. 509]; [SMNy3 Rebrus pp. 799-800]; [ATT `barátjuk` 5, `barátuk` `-`].
- **Validation**: `oracle:UniMorph`, `attestation`.
- **Confidence**: `consensus`.

### NOM-POSS-009 Doublets with meaning split
- **Statement**: Some stems have both 3sg variants with a lexicalised meaning difference (typically inalienable vs alienable possession); both are generated and tagged by sense.
- **Conditions**: lexicon `poss3=both` with sense mapping.
- **Output**:
  | Stem | Variant 1 | Variant 2 |
  |---|---|---|
  | `fiú` | `fia` 'son of' | `fiúja` 'boyfriend of' |
  | `borjú` | `borja` 'calf of (cow)' | `borjúja` 'calf owned by' |
  | `gyapjú` | `gyapja` 'wool of (sheep)' | `gyapjúja` 'wool of (garment)' |
  | `odú` | `odva` 'hollow of (tree)' | `odúja` 'nest/lair of' |
  | `tető` | `teteje` 'top of' | `tetője` 'roof of' |
  | `sarok` | `sarka` 'heel/corner of' | `sarokja` (colloquial variant) |
  | `nyár` | `nyara` 'summer of' | `nyárja` 'poplar of' (distinct lexeme) |
  | `mag` | `magja` 33,860 (default) | `magva` 7,314 ('kernel, essence'; literary) |
  | `néne` | `nénje` 2,383 (modern; the only form generated) | `nénéje` 12 (marginal, rejected by the spelling checker) |
- **Examples**: see table.
- **Exceptions**: `-`
- **Predictability**: `lexical`
- **Lexical input**: sense-level `poss3`.
- **Interactions**: `NOM-STEM-011`..`NOM-STEM-014`.
- **Sources**: [SMNy3 Rebrus pp. 763, 766, 781]; [Tab21 fn. 5 (Kiefer 1985)]; [WC2]; [MTA-HELYES]; [ATT evidence/corpus-variants.md 2026-09-16]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `goldens`, `attestation` (`[/N][Poss.3Sg]` per lemma).
- **Confidence**: `consensus`; `measured` for `mag` and `néne`. `nénje` is 99.5% of 2,395 tokens, so `nénéje` is recorded but not generated - the oracles' ignorance of it was correct behaviour.

### NOM-POSS-010 Stem alternants in possessive forms
- **Statement**: All possessive suffixes are synthetic and select the bound alternant, with the class-specific exceptions for 3sg/3pl `-ja` forms.
- **Conditions**: flags `epenthetic`, `shortening`, `v_stem`, u/ú->v, ú-deleting, `poss_alt`.
- **Output**: bound alternant before `-(O)m`, `-(O)d`, `-(U)nk`, `-(O)tOk`, and before `-(j)A`, `-(j)Uk` when these surface as `-a`, `-uk`; before `-ja`, `-juk` the free stem is kept (`hídja`, `kútja`, `faluja`, `borjúja`, `sarokja`). The high-vowel-to-v stems keep the vowel stem before every personal possessive (`falum`, `PH-VSTEM-004`).
- **Examples**: `bokrom`, `bokra`; `madaram`, `madara`; `hidam`, `hídja`; `lovam`, `lova`; `falum`, `faluja`; `borjam` ~ `borjúm`.
- **Exceptions**: `-`
- **Predictability**: `rule` (given alternants)
- **Lexical input**: stem alternants.
- **Interactions**: `NOM-STEM-*`.
- **Sources**: [SMNy3 Rebrus p. 758 (j-less forms shorten, j-forms do not), p. 802]; [UM-hun `híd`, `út`].
- **Validation**: `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-POSS-011 Further suffixation of possessed singular forms
- **Statement**: A singular possessed form is a lowering stem: final `a`/`e` of 3sg lengthens (`NOM-LENG-004`), and accusative after `-m`, `-d`, `-nk`, `-tOk`, `-k` takes `-at/-et`.
- **Conditions**: possessed singular + following suffix.
- **Output**: 1sg `házamat`, `kertemet`, `gyümölcsömet`; 2sg `házadat`, `kertedet`; 3sg `házát`, `kertjét`; 1pl `házunkat`, `kertünket`; 2pl `házatokat`, `kerteteket`; 3pl `házukat`, `kertjüket`; case: `házamban`, `házában`, `kertjükből`.
- **Examples**: `bűnömet`; `lovukat`; `almámat`.
- **Exceptions**: generation emits the suffixed form as canonical.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: -
- **Sources**: [T04 p. 158]; [KVF98 pp. 193, 195].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

---

## PPL: possessed plural

### NOM-PPL-001 Structure of possessed plural forms
- **Statement**: A plural possessed noun is `stem + (j)A + i + person` after consonant-final stems and `stem + i + person` after vowel-final stems; person suffixes after `-i` are `-m`, `-d`, zero, `-nk`, `-tok/-tek`, `-k`.
- **Conditions**: plural possessed; stem as for 3sg possessive.
- **Output**:
  | Possessor | `ház` | `kert` | `hajó` | `alma` | `kocsi` |
  |---|---|---|---|---|---|
  | 1sg | `házaim` | `kertjeim` | `hajóim` | `almáim` | `kocsijaim` |
  | 2sg | `házaid` | `kertjeid` | `hajóid` | `almáid` | `kocsijaid` |
  | 3sg | `házai` | `kertjei` | `hajói` | `almái` | `kocsijai` |
  | 1pl | `házaink` | `kertjeink` | `hajóink` | `almáink` | `kocsijaink` |
  | 2pl | `házaitok` | `kertjeitek` | `hajóitok` | `almáitok` | `kocsijaitok` |
  | 3pl | `házaik` | `kertjeik` | `hajóik` | `almáik` | `kocsijaik` |
- **Examples**: `barátaim`, `lapjaim`, `gyümölcseim`, `füleim`, `lovaim`, `köveim`, `bokraim`, `teknőim`, `kapuim`.
- **Exceptions**: `NOM-PPL-002`.
- **Predictability**: `rule`
- **Lexical input**: `poss3` choice.
- **Interactions**: `NOM-POSS-005`..`NOM-POSS-007`, `NOM-LENG-004`.
- **Sources**: [SMNy3 Kiefer (39b), (46), (51) pp. 503-509]; [SMNy3 Rebrus (216)-(218) pp. 802-803]; [KVF98 p. 193]; [UM-hun].
- **Validation**: `oracle:UniMorph`, `goldens`.
- **Confidence**: `consensus`.

### NOM-PPL-002 Choice of -ai/-jai/-ei/-jei and vowel-final stems
- **Statement**: After consonant-final stems the `-(j)A-` part repeats the 3sg choice; after vowel-final stems it is absent except after stem-final `i`, where `-jai/-jei` is obligatory.
- **Conditions**: stem end.
- **Output**:
  - C-final, 3sg `-a/-e` -> `-ai-/-ei-`; 3sg `-ja/-je` -> `-jai-/-jei-`.
  - V-final (not `i`) -> `-i-` (`hajói`, `kapui`, `teknői`, `kávéi`, `almái`, `keféi`).
  - `i`-final -> `-jai-/-jei-` (`kocsijai`, `zoknijai`, `hobbijai`; `*zoknii`).
  - Stems with possessive alternants follow their 3sg alternant or the plain vowel stem as listed in `NOM-STEM-013` (`ajtajai` ~ `ajtói`, `idői`, `erői`).
- **Examples**: `lapjai`, `halai`, `kertjei`, `székei`, `hídjai`, `útjai` (`?utai`), `falvai`.
- **Exceptions**: sporadic divergence from 3sg: `barátja` -> `barátai` (`barátjai` `-`), `korlátja` -> `korlátai` [SMNy3 Rebrus p. 798; attestation `barátai` 12, `korlátai` 10].
- **Predictability**: `rule` with lexical overrides (`poss_pl`)
- **Lexical input**: `poss3`, optional `poss_pl`.
- **Interactions**: `NOM-POSS-008`.
- **Sources**: [SMNy3 Rebrus §13.6.3.1 p. 798]; [UM-hun `kocsi`, `út`]; [ATT].
- **Validation**: `attestation`, `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-PPL-003 Lengthening in possessed plurals
- **Statement**: Stem-final `a`/`e` lengthens before `-i-`; the `A` of `-(j)A-` does not.
- **Conditions**: possessed plural.
- **Output**: `almái`, `keféi`; `házai`, `lapjai` (`*lapjái`).
- **Examples**: `kefe` -> `keféim`; `ház` -> `házaim`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LENG-002`, `NOM-LENG-004`.
- **Sources**: [SMNy3 Rebrus §13.6.3.1 (207) p. 797, (187) p. 787].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-PPL-004 Harmony of person suffixes after -i-
- **Statement**: After `-i-` the 2pl suffix is `-tok` for `B` stems and `-tek` for `F` and `R` stems (no `-tök`); 1pl `-nk`, 3pl `-k`.
- **Conditions**: possessed plural 2pl.
- **Output**: `házaitok`, `hajóitok`, `kertjeitek`, `gyümölcseitek`, `köveitek`, `teknőitek`.
- **Examples**: see Output.
- **Exceptions**: vacillating stems follow the harmony of their `-(j)A` variant, since front `-je` is opaque (`foteljeinek` 14 ~ `foteljainak` 5, `*foteljeinak` 0 [WC2]) [RST12 (10c)]; a back stem stays back (`haverjaitok` 61 [WC2]).
- **Predictability**: `rule`
- **Lexical input**: `harmony`
- **Interactions**: `PH-HARM`, `NOM-LINK-005`.
- **Sources**: [UM-hun `teknő`, `kő`, `gyümölcs`]; [RST12 (10c) p. 501].
- **Validation**: `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-PPL-005 Case suffixes after possessed plurals
- **Statement**: Possessed plural forms take case suffixes as lowering stems; after `-i` (3sg) the accusative and superessive attach without linking vowel.
- **Conditions**: possessed plural + case.
- **Output**: `házaimat`, `házaidat`, `házait`, `házainkat`, `házaitokat`, `házaikat`; `házaimon`, `házain`, `házainkon`; `házaimban`, `házaival`.
- **Examples**: `kertjeimet`, `kertjeit`, `kertjeiteket`, `kertjeiket`.
- **Exceptions**: the `-i` of 3sg possessed plural does not lengthen before accusative or superessive (`házait`, `házain`, `*házaít`) [SMNy3 Rebrus p. 804].
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-ACC-010`, `NOM-CASE-011`.
- **Sources**: [SMNy3 Rebrus (219) p. 804]; [UM-hun].
- **Validation**: `attestation` (`házait` [ATT 4], `házaikat` [ATT 3]), `oracle:UniMorph`.
- **Confidence**: `consensus`.

---

## ANP: anaphoric possessive -é

### NOM-ANP-001 Formation of -é
- **Statement**: The anaphoric possessive `-é` ('the one belonging to X') attaches to an unsuffixed, plural, or possessed nominal form; it is invariant for harmony, lengthens a preceding `a`/`e`, and selects the free stem.
- **Conditions**: slot 4; preceding slot not a case suffix.
- **Output**: `-é`.
- **Examples**: `ház` -> `házé`; `házak` -> `házaké`; `házam` -> `házamé`; `háza` -> `házáé`; `házaim` -> `házaimé`; `fa` -> `fáé`; `kefe` -> `keféé`; `kávé` -> `kávéé`; `bokor` -> `bokoré`; `nyár` -> `nyáré`; `ló` -> `lóé`; `Petőfi` -> `Petőfié`; `Kovács` -> `Kovácsé`; `Péterék` -> `Péteréké`.
- **Exceptions**: `ő` -> `övé` and pronominal possessives belong to `PND`.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-MODEL-003`, `NOM-LENG-001`, `NOM-ANP-002`.
- **Sources**: [SMNy3 Kiefer §10.3.4 (55) p. 511]; [SMNy3 Rebrus (187) p. 787, (33) p. 694]; [UD-hu Number[psed]].
- **Validation**: `goldens`, `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-ANP-002 Plural -éi and case after -é
- **Statement**: Plurality of the possessed items is `-é` + `-i`; both `-é` and `-éi` take case suffixes, and suffix harmony after them is decided by the stem (the neutral `é`/`i` are transparent).
- **Conditions**: stem + `-é` (+ `-i`) + case.
- **Output**:
  - Accusative: `-t` directly (`házét`, `házéit`).
  - Harmonic case suffixes use the stem harmony: `házénak`, `házéban`, `házéihoz`; `kerténél`; after stems ending in neutral vowels harmony may vacillate (`Sáriénak` ~ `?Sáriének`).
  - `-é` + `-ként`, `-Ul` degraded (`NOM-MODEL-003`).
- **Examples**: `Nagyénak`; `házaiméiból`; `Kolumbuszéi`.
- **Exceptions**: `-`
- **Predictability**: `rule`; `tendency` for neutral-vowel stems
- **Lexical input**: `harmony`
- **Interactions**: `PH-HARM`, `NOM-ASSOC-002`.
- **Sources**: [RST12 §5.1 (8)-(9) pp. 500-501]; [SMNy3 Kiefer p. 511]; [UD-hu Number[psed]].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

---

## ASSOC: associative plural -ék

### NOM-ASSOC-001 Formation of -ék
- **Statement**: The associative plural `-ék` ('X and his/her family, group') attaches to personal names and person-denoting common nouns, bare or possessed singular; it is invariant, analytic and lengthens final `a`/`e`.
- **Conditions**: head denotes a person; slot 4; not after plural `-k`; marginal after possessed plural.
- **Output**: `-ék`.
- **Examples**: `Szabó` -> `Szabóék`; `Erzsi` -> `Erzsiék`; `Péter` -> `Péterék`; `Varga` -> `Vargáék`; `Kovács` -> `Kovácsék`; `főnök` -> `főnökék`; `szomszéd` -> `szomszédék`; `orvos` -> `orvosék`; `anya` -> `anyámék`; `barát` -> `barátomék`; `öcsém` -> `öcsémék`; attested `Péterék` [ATT 1].
- **Exceptions**: `??barátaimék`, `?öcséimék` [SMNy3 Kiefer p. 501]; `-ék` on non-person nouns is ungrammatical (`*házék`); the condition is semantic and the engine has no person class, so it does not enforce it.
- **Predictability**: `rule` (with semantic condition)
- **Lexical input**: `human` semantic flag.
- **Interactions**: `NOM-MODEL-002` (`Assoc=Yes`), `NOM-ASSOC-002`, `NOM-CASE-001`.
- **Sources**: [SMNy3 Kiefer §10.3.2 p. 501]; [SMNy3 Rebrus (188) p. 788]; [UD-hu UD_Hungarian-Szeged annotation]; [UD-feats]; [em2ud]; [WC2]; [ATT evidence/productivity-tagset.md 2026-09-16].
- **Validation**: `goldens`; UD projection validated against [UD-feats] (`Number` uvalues are `Plur Sing` only; UD has no associative feature).
- **Confidence**: `consensus`; the UD projection is `measured`. **UD projection**: emit `Number=Plur` with the bare-base lemma and no extra feature (Szeged: `Orbánék`, `Thaciék`, `Gyerevék` -> PROPN, `Case=Nom|Number=Plur`, lemma the bare name, `deprel=nsubj`; [em2ud] `'Fam.Pl': ['Number=Plur']` agrees). Our own model keeps a separate `Number=Assoc` value. **The associative/plural distinction is lost in the UD projection** (`Kovácsék` 'the Kovács family' and `Kovácsok` 'people named Kovács' both project to `Number=Plur`), and that loss is recorded rather than papered over with an invented feature, which would fail validation. Corpus scale: `[Fam.Pl]` on 31,723 distinct bases, 582,764 tokens, combining with the full case paradigm and with possessives - fully productive on proper names and kin terms.

### NOM-ASSOC-002 Suffixes after -ék
- **Statement**: An `-ék` form takes `-é` and all case suffixes; the accusative is `-et`; harmonic suffixes follow the stem harmony through transparent `é`.
- **Conditions**: stem ends in `-ék`.
- **Output**: `Péteréket`, `Péteréknek`, `Péterékhez`, `Péteréké`; back stems: `Szabóékat` (`é` transparent to back harmony, lowered `a`), `Szabóéknak`, `Nagyékhoz`.
- **Examples**: `Nagyéknak` [RST12]; `Tomiéknak` ~ `?Tomiéknek`.
- **Exceptions**: stems ending in neutral vowels vacillate (`Tomiéknak` ~ `?Tomiéknek`).
- **Predictability**: `rule`; `tendency` for neutral-vowel names
- **Lexical input**: `harmony`
- **Interactions**: `NOM-ANP-002`, `NOM-ACC-010`.
- **Sources**: [RST12 (9) p. 500]; [SMNy3 Kiefer p. 501 (`Szabóékat`, `orvosékat`)].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

---

## ACC: accusative

### NOM-ACC-001 Accusative after vowel-final stems
- **Statement**: After a vowel-final stem the accusative is `-t`, with final `a`/`e` lengthening.
- **Conditions**: vowel-final free stem that is not a v-stem requiring `-at/-et`.
- **Output**: `-t`.
- **Examples**: `hajó` -> `hajót`; `kocsi` -> `kocsit`; `kapu` -> `kaput`; `teknő` -> `teknőt`; `menü` -> `menüt`; `sí` -> `sít`; `alma` -> `almát`; `kefe` -> `kefét`; `kávé` -> `kávét`; `hosszú` -> `hosszút`; `városi` -> `városit`; `falu` -> `falut`; `borjú` -> `borjút` ~ `borjat`; `szó` -> `szót`.
- **Exceptions**: v-stems take the bound alternant (`lovat`, `követ`, `művet`, `havat`), except `szó` (`szót`), `jó` (`jót`), `szú` (`szút`), and the normative doublets `tó` (`tavat` ~ `tót`), `lé` (`levet` ~ `lét`) [AkH12 §30]; ú-deleting stems vary (`borjút` ~ `borjat`, `NOM-STEM-012`).
- **Predictability**: `rule`
- **Lexical input**: `v_stem`
- **Interactions**: `NOM-STEM-010`, `NOM-LENG-001`.
- **Sources**: [AkH12 §30, §36]; [T04 (7a) p. 147]; [SMNy3 Rebrus (136) p. 759, (148) p. 766].
- **Validation**: `goldens`, `oracle:UniMorph`.
- **Confidence**: `normative` (single `t`); `consensus`.

### NOM-ACC-002 Accusative after consonants: principle
- **Statement**: After a consonant-final stem the accusative `-t` attaches directly iff `C+t` is a licit coda for that stem and the stem is not lowering or epenthetic; otherwise a linking vowel appears.
- **Conditions**: consonant-final stem; evaluate `NOM-ACC-003`..`NOM-ACC-009` in order: lowering/epenthetic/alternant -> vowel; single C -> coda class; geminate -> class of its short counterpart with lexical exceptions; cluster -> lexical `acc_bare`.
- **Output**: `-t` or `-Ot`/`-At`.
- **Examples**: `ón` -> `ónt`; `nyom` -> `nyomot`; `hal` -> `halat` (lowering); `dal` -> `dalt`.
- **Exceptions**: listed in the following rules.
- **Predictability**: `rule` with lexical flags
- **Lexical input**: `lowering`, `epenthetic`, `acc_bare`.
- **Interactions**: `NOM-LOW-001`, `NOM-STEM-*`.
- **Sources**: [T04 §4.1.2.2 (7)-(8) pp. 147-149, §3.2.4.3 pp. 69-74].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `consensus`.

### NOM-ACC-003 Single final consonant: coda classes
- **Statement**: After a single final consonant (non-lowering, non-epenthetic stem) the accusative is bare after `j`/`ly`, `l`, `r`, `n`, `ny`, `s`, `sz`, `z`, `zs`, and takes a linking vowel after all other consonants.
- **Conditions**: stem ends in exactly one consonant letter (digraph = one).
- **Output**:
  | Final C | Accusative | Examples |
  |---|---|---|
  | `j`, `ly` | `-t` | `baj` -> `bajt`, `báj` -> `bájt`, `bej` -> `bejt`, `akadály` -> `akadályt` |
  | `l` | `-t` | `dal` -> `dalt`, `hivatal` -> `hivatalt`, `bál` -> `bált` |
  | `r` | `-t` | `sör` -> `sört`, `bor` -> `bort`, `kör` -> `kört`, `vér` -> `vért` |
  | `n` | `-t` | `ón` -> `ónt`, `szín` -> `színt`, `kín` -> `kínt` |
  | `ny` | `-t` | `lány` -> `lányt`, `fény` -> `fényt` |
  | `s` | `-t` | `rés` -> `rést`, `kés` -> `kést`, `hús` -> `húst`, `sas` -> `sast`, `kas` -> `kast` |
  | `sz` | `-t` | `kosz` -> `koszt`, `rész` -> `részt` |
  | `z` | `-t` | `gőz` -> `gőzt`, `gáz` -> `gázt`, `bűz` -> `bűzt` |
  | `zs` | `-t` | `varázs` -> `varázst`, `rizs` -> `rizst` |
  | `p b t d k g ty gy` | `-Ot` | `rab` -> `rabot`, `bot` -> `botot`, `pad` -> `padot`, `tök` -> `tököt`, `rög` -> `rögöt`, `nap` -> `napot`, `hegy` -> `hegyet` |
  | `f v h` | `-Ot` | `zsiráf` -> `zsiráfot`, `ív` -> `ívet`, `doh` -> `dohot` |
  | `m` | `-Ot` | `nyom` -> `nyomot`, `szem` -> `szemet` |
  | `c cs dzs` | `-Ot` | `perec` -> `perecet`, `rács` -> `rácsot`, `bridzs` -> `bridzset` |
  | `x` (= `ks`) | `-Ot` (cluster) | `fix` -> `fixet`, `szex` -> `szexet` |
- **Examples**: `hivatalt` [ATT 10], `lányt` [ATT 13], `rést` [ATT 8], `gőzt` [ATT 6], `rizst` [ATT 10], `bajt` [ATT 10].
- **Exceptions**: `t`-final stems always take a vowel even though `tt` is a licit geminate coda (`bot` -> `botot`, `rét` -> `rétet`, `öt` -> `ötöt`); lowering stems in the bare classes take a vowel (`NOM-ACC-005`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-ACC-005`, `NOM-LINK-002`.
- **Sources**: [T04 (7b)-(7c) p. 147, p. 149]; [ATT].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `consensus`.

### NOM-ACC-004 Final h and foreign finals
- **Statement**: A pronounced final `h` (including `ch`) requires a linking vowel; for foreign spellings the pronounced final consonant decides by `NOM-ACC-003`.
- **Conditions**: pronunciation from `PH`.
- **Output**: `dohot`, `juhot`, `méhet`, `csehet`, `pechet`; `Bach` -> `Bachot`; `show` (`[ʃoː]`) -> `show-t` (`ORT-FOR-012`).
- **Examples**: `dühöt` [ATT 6], `juhot` [ATT 5], `méhet` [ATT 3].
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: pronunciation.
- **Interactions**: `NOM-STEM-004`, `ORT-FOR-011`, `ORT-FOR-012`, `PH-CONS-005`.
- **Sources**: [T04 (7c) p. 147]; [AkH12 §217 a)].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-ACC-005 Lowering forces the linking vowel
- **Statement**: A lowering stem takes `-at/-et` even when its final consonant belongs to the bare class.
- **Conditions**: `lowering=true`; consonant-final.
- **Output**: `-at/-et`.
- **Examples**: `hal` -> `halat` (vs `dal` -> `dalt`); `fal` -> `falat` (vs `hivatal` -> `hivatalt`); `ár` -> `árat` (vs `sör` -> `sört`); `vár` -> `várat` (vs `vér` -> `vért`); `haj` -> `hajat` (vs `baj` -> `bajt`); `has` -> `hasat` (vs `kas` -> `kast`); `mész` -> `meszet` (vs `kosz` -> `koszt`); `méz` -> `mézet` (vs `gőz` -> `gőzt`); `darázs` -> `darazsat` (vs `varázs` -> `varázst`); `tehén` -> `tehenet` (vs `ón` -> `ónt`); `hány` -> `hányat` (vs `lány` -> `lányt`); `tej` -> `tejet`; `piros` (adj) -> `pirosat` vs `piros` (noun) -> `pirost`; `fiatal` (non-lowering adj) -> `fiatalt`.
- **Exceptions**: lowering stems with bare accusative (`NOM-ACC-007`).
- **Predictability**: `rule`
- **Lexical input**: `lowering`
- **Interactions**: `NOM-LOW-001`, `NOM-LOW-003`, `NOM-LOW-004`.
- **Sources**: [T04 (12)-(13) pp. 153-154, p. 71]; [ATT `halat` 12, `falat` 11, `hajat` 11, `vasat` 9, `mézet` 11, `pirosat` 5, `pirost` `-`, `fiatalt` 9].
- **Validation**: `attestation`, `goldens`.
- **Confidence**: `consensus`.

### NOM-ACC-006 Geminate-final stems
- **Statement**: Stems ending in a geminate whose short counterpart is in the bare class (`ss`, `ssz`, `zz`, `nn`, `nny`, `ll`, `rr`, `jj`) take bare `-t` when non-lowering; in native nouns such geminates are mostly lowering or lexically vowel-taking; `tt` and obstruent geminates take the vowel.
- **Conditions**: final geminate letter pair.
- **Output**:
  | Group | Accusative | Examples |
  |---|---|---|
  | loans and words with written double pronounced short | `-t` | `stressz` -> `stresszt`, `passz` -> `passzt`, `futball` -> `futballt`, `hall` -> `hallt`, `finn` -> `finnt`, `jazz` -> `jazzt`, `dzsessz` -> `dzsesszt`, `plüss` -> `plüsst`, `idill` -> `idillt`, `bross` -> `brosst` ~ `brossot` |
  | native lowering geminate stems | `-at/-et` | `toll` -> `tollat`, `ujj` -> `ujjat`, `gally` -> `gallyat`, `mell` -> `mellet`, `szenny` -> `szennyet`, `könny` -> `könnyet` |
  | native non-lowering with lexical vowel | `-Ot` | `orr` -> `orrot`; `genny` -> `gennyet` (`?gennyt`) |
  | `tt`, `kk`, `pp`, `bb`, `dd`, `gg`, `ggy`, `tty`, `cc`, `ccs` | `-Ot` | `sakk` -> `sakkot`, `csepp` -> `cseppet`, `címzett` -> `címzettet`, `pötty` -> `pöttyöt`, `meggy` -> `meggyet` |
- **Examples**: `orrot` 8,930 vs `*orrt` 2 [WC2, `[/N][Acc]` on lemma `orr`]; `gennyet` 853 vs `?gennyt` 50 [WC2], past the tenfold margin of a doublet; `bross` -> `brosst` 508 vs `brossot` 32 [WC2]; `könnyet` [ATT 6] vs `könnyt` `-`; `stresszt` [ATT 11] vs `stresszet` `-`; `passzt` [ATT 5]; `futballt` [ATT 6]; `jazzt` [ATT 4]; `tollat` [ATT 9]; `ujjat` [ATT 4].
- **Exceptions**: [T04] lists `orrt` ~ `orrot` and `bross-t` ~ `bross-ot` as variable; measured on 8.57e9 tokens the two go opposite ways and each is near-categorical. **Do not generate `*orrt`** (2 tokens against `orrot` 8,930, and those 2 are nineteenth-century literature and subtitle noise); **do generate `brosst`**, which is the majority form (508 : 32). This settles `OQ-03-012`.
- **Predictability**: `rule` for loans and obstruent geminates; `lexical` for native sonorant geminates (flag `acc_bare`)
- **Lexical input**: `lowering`, `acc_bare`, loan flag.
- **Interactions**: `NOM-ACC-003`, `NOM-ACC-005`.
- **Sources**: [T04 §3.2.4.3 p. 73 fn. 58, §4.1.2.2 p. 148]; [ATT]; [WC2]; [ATT evidence/corpus-variants.md 2026-09-16].
- **Validation**: `attestation` (tag-filtered per-lemma counts; homograph control matters here - unfiltered `orrt` shows 158 tokens, only 2 of which carry `[/N][Acc]` under lemma `orr`).
- **Confidence**: `consensus`; `measured` for `orr` and `bross`, which are near-categorical in opposite directions.

### NOM-ACC-007 Lowering stems with bare accusative
- **Statement**: A few lowering stems take bare `-t` despite lowering.
- **Conditions**: lexical.
- **Output**: measured against a corpus-verified lowering diagnosis rather than a flag, the class is small - **15 lowering members** plus the compounds of `oldal`: `oldal` -> `oldalt` (pl. `oldalak`; confirmed hard, `*oldalat` rejected by the spelling checker) and its nine compounds `címoldal domboldal hegyoldal hátoldal partoldal töltésoldal árnyoldal éroldal ügyféloldal`; `leltár` and `vagyonleltár`; `sudár`; `sivár`; and `avas hamvas kormos kásás köpcös gőgös meglehetős parkettás szorgalmatos túlkoros őshonos` in the wider bare-`t` set. `szakáll` -> `szakállt` 2,773 ~ `szakállat` 2,988 is an explicit doublet, both legal. **`báj` is not a member** (`bájat` 127 : `bájt` 10, both legal): the earlier claim rested on a thin `[ATT]` figure.
- **Examples**: `oldalt` [ATT 14], `*oldalat` rejected [MTA-HELYES]; `szakállt` 2,773 ~ `szakállat` 2,988 [WC2]; `bájat` 127 : `bájt` 10 [WC2].
- **Exceptions**: Papp (1975)'s full list is still not available to the project, but the class is now enumerated from the corpus instead (`OQ-03-005` resolved).
- **Predictability**: `lexical` (closed list, 15 lowering members)
- **Lexical input**: `acc_bare=true` on lowering stems; from `bare-accusative.tsv` (705 bare-`t` items over 10,611 candidates, of which 15 are lowering; 855 in review), `local/lists/bare-accusative.tsv` - read by `cargo xtask morphdb`.
- **Interactions**: `NOM-ACC-005`, `NOM-ACC-009`.
- **Sources**: [T04 fn. 23 p. 154]; [ATT]; [WC2]; [MTA-HELYES]; [hunmorph-foma]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `attestation`, `oracle:hunmorph-foma`.
- **Confidence**: `consensus`; membership `measured`.

### NOM-ACC-008 Epenthetic and alternating stems
- **Statement**: Epenthetic, shortening and v-stems take the accusative on the bound alternant with a linking vowel even where the final consonant would permit bare `-t`; a few epenthetic stems also allow the free stem with bare `-t`.
- **Conditions**: stem flags.
- **Output**: bound alternant + `-Ot`/`-At`; variants per item.
- **Examples**: `haszon` -> `hasznot`; `torony` -> `tornyot`; `öböl` -> `öblöt` ~ `öbölt`; `ökör` -> `ökröt`; `bagoly` -> `baglyot`; `bajusz` -> `bajuszt` ~ `bajszot`; `bátor` -> `bátrat` (adj, lowering); `kanál` -> `kanalat`; `szél` -> `szelet`; `ér` -> `eret` ~ `ért` 'brook'; `ész` -> `eszet` ~ `észt`.
- **Exceptions**: preference for `bajusz`: `bajuszt` [ATT 3], `bajszot` `-`.
- **Predictability**: `lexical` (variants)
- **Lexical input**: stem alternants, `acc_free_stem` variant flag.
- **Interactions**: `NOM-STEM-005`, `NOM-STEM-007`, `NOM-STEM-010`.
- **Sources**: [T04 (8) p. 148]; [SMNy3 Rebrus p. 693, p. 743]; [ATT `hasznot` 11, `tornyot` 8, `öblöt` 4].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### NOM-ACC-009 Cluster-final stems: default, lexical bare, variation
- **Statement**: After a final two-consonant cluster the accusative takes a linking vowel by default; stems ending in sonorant + (`s sz z zs n l r j`) may be lexically marked for bare `-t`; `r` + sibilant always takes the vowel; some stems vary.
- **Conditions**: final CC (not geminate).
- **Output**:
  | Cluster | bare `-t` | `-Ot` | both |
  |---|---|---|---|
  | `ns` (`[nʃ]`) | `protestánst`, `briliánst` | `revansot` | `szegmenst` ~ `szegmenset` |
  | `nsz` | `nonszenszt` | `sanszot`, `fajanszot` | `reneszánszt` ~ `reneszánszot` |
  | `jz` | `csuszpájzt` | `rajzot` | - |
  | `nz` | `pénzt` | `bronzot` | `csimpánzt` ~ `csimpánzot` |
  | `jzs` | - | `pajzsot` | - |
  | `rn`, `jn`, `jl`, `rl` etc. | `konszernt`, `kombájnt`, `fájlt` | - | - |
  | `rs`, `rsz`, `rz` | never | `borsot`, `merszet`, `borzot` | - |
  | obstruent-initial clusters | never | `tapsot`, `szexet`, `aktot`, `barackot`, `copfot` | - |
  | sonorant + non-coda-forming C | never | `farmot`, `galambot` | - |
  | stop-final clusters | never | `kertet`, `csontot`, `gondot`, `hangot` | - |
  | lowering clusters | never | `szárnyat`, `fürjet`, `törzset`, `holdat`, `földet` | - |
- **Examples**: `pénzt` [ATT 18] vs `pénzet` `-`; `bronzot` [ATT 5]; `rajzot` [ATT 8]; `borsot` [ATT 9]; `fájlt` [ATT 10] vs `fájlot` `-`; `tapsot` [ATT 5]; measured [WC2, tagged on the lemma]: `protestánst` 341 : `*protestánsot` 1, `briliánst` 199 : `briliánsot` 0, `nonszenszt` 69 : `nonszenszot` 0, `szegmenst` 6,828 ~ `szegmenset` 1,696, `reneszánszt` 588 ~ `reneszánszot` 616, `fajanszot` 134 : `?fajanszt` 9, `pajzsot` 7,629 : `*pajzst` 22. A doublet is both forms above 20 tokens within a tenfold margin, the measure the lexicon conversion applies; T04's `?briliánsot`, `?fajanszt` and `*pajzst` fail it.
- **Exceptions**: consonant-final stems with three-consonant clusters always take the vowel (`karsztot`, `dunsztot`, `mumpszot`).
- **Predictability**: `lexical` for sonorant+sibilant/sonorant clusters (default vowel unless `acc_bare`); `rule` for others
- **Lexical input**: `acc_bare`, `acc_variants`, supplied by `bare-accusative.tsv` - 705 bare-`t` items and an 855-item variation list over 10,611 candidates, in `local/lists/bare-accusative.tsv` (read by `cargo xtask morphdb`). This list **replaces** the sparse and inconsistent `BareAccusative` / `LinkedAccusative` traits, which disagree with the corpus on 22% of rows (19 and 6 on nouns, 262 and 1,027 on adjectives).
- **Interactions**: `NOM-ACC-005`, `NOM-ACC-007`.
- **Sources**: [T04 (26) p. 73, pp. 69-74, p. 162]; [ATT]; [WC2]; [hunmorph-foma]; [magyarispell]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `attestation`, `oracle:hunmorph-foma`; oracle agreement on corpus-decided rows: morphdb 619/681, our `.ron` 2481/3181, foma 1753/1761, hunspell 1940/1946, UniMorph 925/948, kaikki 1220/1240.
- **Confidence**: `consensus`; `tendency` for variable items. The `-s`-final adjective accusative (`teljest` 372 ~ `teljeset` 2,372 and about 400 similar items) is the one large residue and needs a register-stratified count; it belongs here, not under `NOM-ACC-007`.

### NOM-ACC-010 Accusative after inflected stems
- **Statement**: After plural, possessive, `-é`, `-ék` and comparative forms the accusative follows `NOM-ACC-001`..`NOM-ACC-003` on a lowering stem.
- **Conditions**: stem is an inflected form.
- **Output**:
  - ends in `-k` (plural, 1pl, 3pl, possessed pl 1pl/3pl): `-at/-et` (`házakat`, `kertünket`, `házukat`, `házaikat`).
  - ends in `-m`, `-d`: `-at/-et` (`házamat`, `kertedet`, `bűnömet`).
  - ends in `-tOk`: `-at/-et` (`házatokat`, `kerteteket`, `gyümölcsötöket`).
  - ends in vowel (3sg `-(j)A` lengthened, `-i`, `-é`): `-t` (`házát`, `kertjét`, `házait`, `házét`, `házéit`).
  - ends in `-ék`: `-et` after front, `-at` after back stems through transparent `é` (`Péteréket`, `Szabóékat`).
  - comparative `-bb`: `-at/-et` (`nagyobbat`, `szebbet`).
- **Examples**: `gyümölcsöket` (UM), `tököket`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LOW-002`, `NOM-POSS-011`, `NOM-PPL-005`.
- **Sources**: [T04 pp. 158-161]; [SMNy3 Kiefer p. 501]; [UM-hun].
- **Validation**: `oracle:UniMorph`, `goldens`.
- **Confidence**: `consensus`.

---

## CASE: case and case-like suffixes

### NOM-CASE-001 Case suffix inventory
- **Statement**: Nominal case marking uses the following suffixes; each has a fixed template, class (`NOM-MODEL-004`), lengthening behaviour and feature value.
- **Conditions**: slot 6.
- **Output**:
  | Name (Hungarian) | Template | Allomorphs | Class | Lengthens | UD `Case` | UniMorph | Rule |
  |---|---|---|---|---|---|---|---|
  | nominative (alanyeset) | zero | - | - | - | `Nom` | `NOM` | - |
  | accusative (tárgyeset) | `-(O)t` | `-t -ot -at -et -öt` | I | yes | `Acc` | `ACC` | `NOM-ACC-*` |
  | dative (részes eset) | `-nAk` | `-nak -nek` | II | yes | `Dat` (`Gen` for possessor) | `DAT` | `NOM-CASE-002` |
  | instrumental-comitative (eszközhatározó) | `-vAl` | `-val -vel`, `-CCal/-CCel` | II | yes | `Ins` | `INS` | `NOM-CASE-003` |
  | causal-final (okhatározó) | `-ért` | `-ért` | II | yes | `Cau` | `PRP` | `NOM-CASE-006` |
  | translative-factive (eredményhatározó) | `-vÁ` | `-vá -vé`, `-CCá/-CCé` | II | yes | `Tra` | `TRANS` | `NOM-CASE-004` |
  | terminative (határeset) | `-ig` | `-ig` | II | yes | `Ter` | `TERM` | `NOM-CASE-007` |
  | essive-formal (-ként); our model `Case=Formalis` | `-ként` | `-ként` | III | no | `Abs` (decided 2026-09-16: permitted by [UD-feats] on exactly NOUN/PROPN/ADJ, documented in [UD-hu] `hu/feat/Case.html` against the suffix `-ként` with the example `éveként`, and carried by all 57 relevant Szeged tokens; [em2ud] dissents with `Case=Ess` and is the weakest witness) | `FRML` | `NOM-CASE-008` |
  | essive (-ul/-ül) | `-Ul` | `-ul -ül` | II | yes | `Ess` | - | `NOM-CASE-009` |
  | inessive | `-bAn` | `-ban -ben` | II | yes | `Ine` | `IN+ESS` | `NOM-CASE-010` |
  | illative | `-bA` | `-ba -be` | II | yes | `Ill` | `IN+ALL` | `NOM-CASE-010` |
  | elative | `-bÓl` | `-ból -ből` | II | yes | `Ela` | `IN+ABL` | `NOM-CASE-010` |
  | superessive | `-(O)n` | `-n -on -en -ön` | I | yes | `Sup` | `ON+ESS` | `NOM-CASE-011` |
  | sublative | `-rA` | `-ra -re` | II | yes | `Sbl` | `ON+ALL` | `NOM-CASE-012` |
  | delative | `-rÓl` | `-ról -ről` | II | yes | `Del` | `ON+ABL` | `NOM-CASE-012` |
  | adessive | `-nÁl` | `-nál -nél` | II | yes | `Ade` | `AT+ESS` | `NOM-CASE-013` |
  | allative | `-hOz` | `-hoz -hez -höz` | II | yes | `All` | `AT+ALL` | `NOM-CASE-013` |
  | ablative | `-tÓl` | `-tól -től` | II | yes | `Abl` | `AT+ABL` | `NOM-CASE-013` |
  | temporal | `-kor` | `-kor` | III | no | `Tem` | - | `NOM-CASE-017` |
  | distributive; our model `Case=Distributivus` | `-(O)nként` | `-nként -onként -anként -enként -önként` | I | yes | `Dis` (7 Szeged tokens, separate from `-ként`) | - | `NOM-CASE-018` |
  | sociative; our model `Case=Sociativus` | `-(O)stUl` | `-stul -stül -ostul -astul -estül -östül` | I | yes | **no UD `Case`** - a recorded decision, not an omission: `Com` is not among the 22 permitted hu values, so emitting it is a validation error; UD projection is `UPOS=NOUN`, `Case` omitted, `deprel=obl` | - | `NOM-CASE-019` |
  | formal; our model `Case=Modalis` | `-képp`, `-képpen` | invariant | III | no | `Abs`, the same value as `-ként` (the two are not distinguished in the UD projection), for productive `-képp(en)` on a noun or adjective; the pronominal and lexicalised forms are ADV with `PronType` and no `Case`: `mindenképp(en)` `PronType=Tot`, `semmiképp(en)` `PronType=Neg`, `miképp(en)` `PronType=Int`, `tulajdonképpen` and `voltaképpen` no features | - | `NOM-CASE-020` |
  | distributive-temporal | `-(O)ntA` | `-nta -nte -onta -ente -önte` | I | yes | - | - | `NOM-CASE-021` |
  | locative | `-t`, `-Ott` | `-t -ott -ett -ött` | lexical | - | `Loc` | - | `NOM-CASE-016` |
- **Examples**: `ház`: `házat`, `háznak`, `házzal`, `házért`, `házzá`, `házig`, `házként`, `házul`, `házban`, `házba`, `házból`, `házon`, `házra`, `házról`, `háznál`, `házhoz`, `háztól`.
- **Exceptions**: SMNy3 counts 17 inflectional case suffixes plus zero nominative and treats `-kor`, `-nként`, `-stUl`, `-képp(en)`, `-ntA`, locative `-t`, adverbial `-An`, multiplicative `-szOr` as derivational; KVF98 lists 27 including modal `-lAg` and modal-essive `-An` [SMNy3 p. 668; KVF98 pp. 191-192]. The library generates all rows above as nominal inflection with the listed restrictions.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-MODEL-003`, `NOM-MODEL-004`, `NOM-CASE-014`.
- **Sources**: [SMNy3 Kiefer §10.3.1 (22) p. 494; Rebrus (6) p. 670]; [KVF98 pp. 191-193, pp. 227-236]; [UD-hu Case, UD_Hungarian-Szeged annotation]; [UD-feats] `hu` section - `Case.lvalues = []`, so **no Hungarian-specific `Case` value may be emitted**, which is the constraint that drives all four decisions above and kills `Case=Frm`, `Case=Com` and `Case=Inl` outright; [UM-hun]; [em2ud] (read only; it emits `Case=Inl`, not a UD value in any language, and `Degree=Abs`, not permitted for hu, so it is filtered if used as an oracle); [AkH12 §34, §35, §38]; [ATT evidence/productivity-tagset.md 2026-09-16].
- **Validation**: `oracle:UniMorph`, `goldens`, UD validator conformance against [UD-feats].
- **Confidence**: `consensus` for forms; `measured` for the UD projection (`OQ-03-006` retired); `contested` for case vs derivational status (irrelevant to generation). Two conflicts are documented rather than hidden: `Case=Abs` is a bad *name* - Hungarian has no absolutive and the `hu/feat/Case.html` prose for it is a copy-paste of the universal absolutive gloss - but `Ess` is already taken in Szeged by the `-an`/`-en` manner adverbs (365 tokens, `advmod:mode`) and `Frm` does not exist, so interoperability with the only Hungarian treebank wins; and `-stUl` has no UD home at all, which is a coverage gap in UD, not a judgement call (Webcorpus 2 has 8,024 distinct bases / 73,040 tokens). Both are worth reporting upstream.

### NOM-CASE-002 Dative -nAk
- **Statement**: The dative is `-nak` (`B`) / `-nek` (`F`, `R`) on the free stem.
- **Conditions**: any nominal form not ending in a case suffix.
- **Output**: `-nak`/`-nek`; before it final `a`/`e` lengthen.
- **Examples**: `háznak`, `kertnek`, `gyümölcsnek`, `almának`, `kefének`, `lónak`, `bokornak`, `madárnak`, `tükörnek`, `házaknak`, `házamnak`, `házáénak`; `Mariann` -> `Mariann-nak` (name, `ORT-NAME-012`), common `-nn` + `-nak` -> three letters reduced (`NOM-CASE-005`).
- **Exceptions**: in possessor function the same form is annotated `Case=Gen` in UD (`Péternek a háza`).
- **Predictability**: `rule`
- **Lexical input**: `harmony`
- **Interactions**: -
- **Sources**: [KVF98 p. 192]; [SMNy3 Kiefer p. 493]; [UD-hu Case].
- **Validation**: `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-CASE-003 Instrumental -vAl and v-assimilation
- **Statement**: The instrumental is `-val/-vel` after vowel-final stems; after a consonant-final stem its `v` is replaced by a copy of the final consonant, written as a doubled letter.
- **Conditions**: free stem end (pronounced).
- **Output**:
  - Vowel-final (including silent `h`): `-val/-vel` (`almával`, `kefével`, `hajóval`, `lóval`, `kővel`, `művel`, `csehvel`).
  - Single consonant letter: consonant doubled + `-al/-el` (`bottal`, `kalappal`, `ablakkal`, `házzal`, `kézzel`, `madárral`, `bokorral`, `kerttel`, `pénzzel`, `fájllal`).
  - Digraph consonant: first letter doubled (`cs` -> `ccs`, `sz` -> `ssz`, `zs` -> `zzs`, `gy` -> `ggy`, `ly` -> `lly`, `ny` -> `nny`, `ty` -> `tty`, `dz` -> `ddz`, `dzs` -> `ddzs`): `gyümölccsel`, `rozzsal`, `néggyel`, `királlyal`, `arannyal`, `kosszal`, `briddzsel`.
  - Final long consonant (geminate or doubled digraph): no additional letter (`NOM-CASE-005`): `tollal`, `orral`, `könnyel`, `meggyel`, `stresszel`, `füttyel`.
  - Final cluster: last consonant doubled (`hanggal`, `komppal`, `minddel`, `kulccsal`, `szárnnyal`, `gyönggyel`).
  - Final `v`: `vv` written (`szívvel`, `kedvvel`, `hévvel`, `könyvvel`).
  - Final `h` pronounced: `hh` (`dohhal`, `pechhel`); optional-silent `h`: both (`dühvel` ~ `dühhel`, `NOM-STEM-004`).
  - Final `x` (`[ks]`): `xsz` (`fixszel`, `Marxszal`); `ch` (`[x]` or `[tʃ]`) by pronunciation (`Bachhal`, `Madáchcsal`) (`ORT-NAME-011`, `ORT-FOR-011`).
- **Examples**: see Output.
- **Exceptions**: proper names, numerals, abbreviations, acronyms: hyphenation and letter treatment by `ORT-NAME-011`, `ORT-NAME-012`, `ORT-ABBR-009`, `ORT-ABBR-014`, `ORT-NUM-002` (`Kiss-sel`, `4-gyel`, `DNS-sel`); the phonological output is identical.
- **Predictability**: `rule`
- **Lexical input**: pronunciation of final segment (`PH`).
- **Interactions**: `NOM-CASE-004`, `NOM-CASE-005`, `ORT-ASSIM-001`..`ORT-ASSIM-004`, `PH-ASSIM-002`.
- **Sources**: [AkH12 §41, §82 a)-f), §92, §93, §160, §216 b)]; [ATT `tollal` 8, `orral` 5, `kulccsal` 7, `szárnnyal` 4, `szívvel` 13, `dühvel` 4].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `normative`.

### NOM-CASE-004 Translative -vÁ
- **Statement**: The translative is `-vá/-vé` after vowels and assimilates exactly like the instrumental after consonants.
- **Conditions**: as `NOM-CASE-003`.
- **Output**: `-vá/-vé`, `-CCá/-CCé`.
- **Examples**: `almává`, `kővé`, `pléhvé`; `házzá`, `kertté`, `szénné`, `vízzé`, `királlyá`, `özveggyé`; `tollá`, `jobbá`, `könnyé`, `rosszá`; `füstté`, `bolonddá`, `bölccsé`, `szárnnyá`; `névvé`, `savvá`, `könyvvé`; `méhvé` ~ `méhhé`.
- **Exceptions**: homographs with `-é`: `könny+vé` = `könny+é` = `könnyé`; `stressz+vé` = `stresszé`; analysis returns both.
- **Predictability**: `rule`
- **Lexical input**: pronunciation.
- **Interactions**: `NOM-CASE-003`, `NOM-ANP-001`.
- **Sources**: [AkH12 §41, §82]; [UM-hun `könny`].
- **Validation**: `goldens`, `oracle:UniMorph`.
- **Confidence**: `normative`.

### NOM-CASE-005 Three identical consonant letters
- **Statement**: When suffixation places three identical consonant letters in a row in a common word, only two are written; the pronounced consonant stays long.
- **Conditions**: stem ends in a doubled letter (or doubled digraph) and the suffix begins with the same consonant, or `v`-assimilation copies it; not proper names and not compounds (hyphen, `ORT-NAME-012`, `ORT-FOR-013`).
- **Output**: two letters.
- **Examples**: `orr+ra` -> `orra`; `orr+ról` -> `orról`; `toll+val` -> `tollal`; `vádlott+tól` -> `vádlottól`; `fütty+vel` -> `füttyel`; `könny+vé` -> `könnyé`; `nagyobb+ban` -> `nagyobban`; `stressz+vel` -> `stresszel`; `hall+lak` (verb, `VRB`).
- **Exceptions**: proper names keep all letters with a hyphen (`Széll-lel`, `Mariann-nal`, `Tallinn-nak`, `Bükk-kel`) [AkH12 §93, §160 c)]; AkH11 simplified given names (`Mariannal`) [AkH11 §94].
- **Predictability**: `rule`
- **Lexical input**: proper-name flag.
- **Interactions**: `NOM-PROP-002`, `ORT-ASSIM-017`, `PH-ASSIM-012`.
- **Sources**: [AkH12 §93, §160 c)]; [UM-hun `orr` sublative `orra`].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### NOM-CASE-006 Causal-final -ért
- **Statement**: `-ért` is invariant, stable-vowel, analytic and lengthening.
- **Conditions**: any nominal form.
- **Output**: `-ért`.
- **Examples**: `házért`, `almáért`, `lóért`, `bokorért`, `nyárért`, `forintért`, `házakért`, `fiamért`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-MODEL-004`.
- **Sources**: [T04 (20) p. 162]; [SMNy3 Rebrus (33) p. 694].
- **Validation**: `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-CASE-007 Terminative -ig
- **Statement**: `-ig` is invariant, short `i`, analytic and lengthening; stem-final `i` is not merged (`ii` written).
- **Conditions**: any nominal form.
- **Output**: `-ig`.
- **Examples**: `házig`, `almáig`, `kocsiig`, `zokniig`, `bokorig`, `nyárig`, `lóig`, `holnapig`, `keddig`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-MODEL-004`.
- **Sources**: [AkH12 §35]; [SMNy3 Rebrus p. 798 (`zokniig`)].
- **Validation**: `oracle:UniMorph`.
- **Confidence**: `normative`.

### NOM-CASE-008 Essive-formal -ként
- **Statement**: `-ként` ('as') is invariant, super-analytic and non-lengthening.
- **Conditions**: stem, plural or possessed form; not after `-é` (`NOM-MODEL-003`).
- **Output**: `-ként`.
- **Examples**: `tolmácsként`, `almaként`, `kefeként`, `óraként`, `bokorként`, `madárként`, `lóként`, `házakként`, `házamként`.
- **Exceptions**: `másként` (pronoun, `PND`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LENG-002`.
- **Sources**: [KVF98 p. 193, p. 227]; [SMNy3 Rebrus (188) p. 788].
- **Validation**: `oracle:UniMorph`.
- **Confidence**: `consensus`.

### NOM-CASE-009 Essive -Ul
- **Statement**: The essive `-ul/-ül` ('as, for') is harmonic, short-vowelled, analytic and lengthening; it is lexically restricted in productive use.
- **Conditions**: mainly with nouns in predicative-complement function of a few verbs (`választ`, `alkalmaz`, `hív`, `ad`, `vesz`); fixed forms common.
- **Output**: `-ul` (`B`), `-ül` (`F`, `R`); stem-final `u` is not merged (`bantuul`).
- **Examples**: `feleségül`, `segítségül`, `tanúul`, `például`, `ráadásul`, `franciául`; language adverbs `magyarul`, `németül` (`NOM-ADJ-011`).
- **Exceptions**: degraded after proper names and plurals (`?Lajosul`, `??Lajosokul`) and after `-é` (`*házéul`).
- **Predictability**: `rule` (form), `lexical` (usage)
- **Lexical input**: `none`
- **Interactions**: `NOM-ADJ-011`, `NOM-MODEL-003`.
- **Sources**: [AkH12 §35]; [SMNy3 Kiefer p. 495, p. 498]; [SMNy3 Rebrus (187) p. 787].
- **Validation**: `attestation` (`feleségül` [ATT 12], `segítségül` [ATT 10]).
- **Confidence**: `consensus`.

### NOM-CASE-010 Interior local cases -bAn, -bA, -bÓl
- **Statement**: Inessive `-ban/-ben`, illative `-ba/-be`, elative `-ból/-ből` are harmonic two-way suffixes on the free stem, lengthening; `-ból/-ből` always has long `ó/ő` in writing.
- **Conditions**: any nominal form.
- **Output**: per harmony.
- **Examples**: `házban`, `házba`, `házból`; `kertben`, `kertbe`, `kertből`; `gyümölcsben`, `gyümölcsbe`, `gyümölcsből`; `almában`, `bokorban`, `lóban`, `kézben`, `vízben`; `nagyobb+ban` -> `nagyobban`.
- **Exceptions**: `-bÓl`'s `ó/ő` is pronounced short in colloquial speech [SMNy3 Rebrus p. 780] but always written long [AkH12 §34].
- **Predictability**: `rule`
- **Lexical input**: `harmony`
- **Interactions**: `NOM-CASE-014`, `NOM-PROP-006` (place names).
- **Sources**: [AkH12 §34]; [KVF98 p. 192]; [SMNy3 Rebrus p. 780].
- **Validation**: `oracle:UniMorph`.
- **Confidence**: `normative`.

### NOM-CASE-011 Superessive -(O)n
- **Statement**: The superessive is `-n` after vowels and `-on/-en/-ön` after consonants; it is synthetic (selects epenthetic and v-stem alternants) but never lowered and does not trigger stem-internal shortening.
- **Conditions**:
  - Vowel-final free stem -> `-n` (`almán`, `kefén`, `hajón`, `kocsin`, `kapun`, `teknőn`, `falun`, `borjún`, `szón`).
  - Consonant-final -> `O` by harmony (`B` `-on`, `F` `-en`, `R` `-ön`), including lowering stems (`házon`, `fülön`, `szögön`, `könnyön`, `tűzön`, `szűzön`).
  - Epenthetic -> bound alternant (`bokron`, `tükrön`, `lelken`, `kelyhen`); variant `fodoron`; marginal `?háromon`.
  - v-stems -> bound alternant (`lovon`, `tavon`, `havon`, `kövön`, `füvön`, `művön`, `csövön`); not `szó` (`szón`), `jó` (`jón`), `lé` (`lén` ~ `?leven`), `szú` (`szún`).
  - Shortening stems -> free long stem (`nyáron`, `madáron`, `kézen`, `vízen` ~ `vizen`, `hídon`, `úton`, `tűzön`, `lelken`).
  - After inflected stems: `házakon`, `házamon`, `házán`, `házain`.
- **Output**: see Conditions.
- **Examples**: `lovon` [ATT 7] vs `lón` `-`; `tavon` [ATT 8]; `kövön` [ATT 7]; `hídon` [ATT 10]; `nyáron` [ATT 16]; `kézen` [ATT 10].
- **Exceptions**: `vizen` variant (`NOM-STEM-007`); adjectives: superessive is `-on/-en/-ön` (`piroson`), distinct from adverbial `-an/-en` (`pirosan`) (`NOM-ADJ-010`).
- **Predictability**: `rule`
- **Lexical input**: stem alternants.
- **Interactions**: `NOM-STEM-005`, `NOM-STEM-007`, `NOM-STEM-010`, `NOM-LOW-001`.
- **Sources**: [T04 fn. 30 p. 158, (20) p. 162]; [SMNy3 Rebrus (33) p. 694, (113) p. 743, (136) p. 759, p. 760]; [UM-hun `fül`, `könny`, `kő`, `mű`].
- **Validation**: `oracle:UniMorph`, `attestation`.
- **Confidence**: `consensus`.

### NOM-CASE-012 Surface local cases -rA, -rÓl
- **Statement**: Sublative `-ra/-re` and delative `-ról/-ről` are harmonic two-way analytic lengthening suffixes; `-ról/-ről` is written long.
- **Conditions**: any nominal form.
- **Output**: per harmony; three-`r` reduction after `rr` (`orra`, `orról`).
- **Examples**: `házra`, `házról`; `kertre`, `kertről`; `almára`, `lóra`, `bokorra`, `madárra`, `kézre`; `orr` -> `orra`, `orról`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `harmony`
- **Interactions**: `NOM-CASE-005`, `NOM-CASE-014`.
- **Sources**: [AkH12 §34, §93]; [UM-hun `orr`].
- **Validation**: `oracle:UniMorph`.
- **Confidence**: `normative`.

### NOM-CASE-013 Exterior local cases -nÁl, -hOz, -tÓl
- **Statement**: Adessive `-nál/-nél` and ablative `-tól/-től` are two-way; allative `-hoz/-hez/-höz` is three-way by rounding harmony; all are analytic and lengthening.
- **Conditions**: any nominal form.
- **Output**: `-hoz` (`B`), `-hez` (`F`), `-höz` (`R`, including stems whose last vowel is `ö ő ü ű`, whether lowering or not).
- **Examples**: `háznál`, `házhoz`, `háztól`; `kertnél`, `kerthez`, `kerttől`; `gyümölcsnél`, `gyümölcshöz`, `gyümölcstől`; `fülhöz`, `kőhöz`, `tűzhöz`, `könnyhöz`, `teknőhöz`; `keféhez`, `stresszhez`; three-`t` reduction: `vádlott+tól` -> `vádlottól`.
- **Exceptions**: `-hOz` after `-é`, `-ék`, `-i` follows the stem-level harmony choice made before the neutral suffix; front rounded stems + `-é`: `kőéhez` (front unrounded, since `é` is the last vowel and rounding harmony is local) (`PH-HARM`). After the possessive `-é` the allative is always `-hez`: at pattern level `-éhez` 4,735,027 : `-éhöz` 300 [WC2], and the 15 most frequent `-éhöz` types are archaic or dialectal spellings of ordinary possessives (`föltételéhöz`, `szivéhöz`, `nevéhöz`), not rounding harmony after `-é`. `-höz` after `-é` is not standard Hungarian. Item-level evidence for `kőéhez` itself remains insufficient (5 tokens against `kőéhöz` 0).
- **Predictability**: `rule`
- **Lexical input**: `harmony`
- **Interactions**: `NOM-CASE-014`.
- **Sources**: [AkH12 §34, §93]; [SMNy3 Rebrus (6) p. 670 (`tökhöz`)]; [UM-hun `fül`, `könny`, `kő`]; [WC2]; [ATT evidence/corpus-variants.md 2026-09-16].
- **Validation**: `oracle:UniMorph`, `attestation` (pattern-level `-éhez` : `-éhöz`).
- **Confidence**: `consensus`; `measured` at pattern level for rounding after `-é` (`OQ-03-011` resolved at pattern level, item level still insufficient).

### NOM-CASE-014 Harmony selection for case suffixes
- **Statement**: Two-way case suffixes (`A`, `Á`, `Ó`, `U`) take the back variant after `B` stems and the front variant after `F` and `R` stems; three-way `O` suffixes (`-hOz`, linking `O`) distinguish `R`; `-ért`, `-ig`, `-ként`, `-kor`, `-képp(en)` are invariant.
- **Conditions**: harmony value of the immediately preceding string computed by `PH-HARM` (antiharmonic, vacillating stems, neutral suffixes).
- **Output**: allomorph choice.
- **Examples**: `híd` (antiharmonic) -> `hídnak`, `hídhoz`; `cél` -> `célnak`; `derék` -> `deréknak` ~ `deréknek`; `férfi` -> `férfinak` ~ `férfinek`; `fotel` -> `fotelnek` ~ `fotelnak`; `haver` -> `havernak` (back; `?havernek` 161 : 3,259 [WC2] is colloquial); `agilis` -> `agilisnak` ~ `agilisnek`; `alibi` -> `alibinek`.
- **Exceptions**: invariant back `-kor` after front stems (`éjfélkor`, `hétkor`) [KVF98 pp. 422-423].
- **Predictability**: `rule` given `PH-HARM` output
- **Lexical input**: `harmony`
- **Interactions**: `PH-HARM`.
- **Sources**: [RST12 §3-5 pp. 493-505]; [KVF98 pp. 419-424].
- **Validation**: `oracle:UniMorph`, `attestation`.
- **Confidence**: `consensus`.

### NOM-CASE-015 Case suffixes after possessive and plural stems
- **Statement**: Case suffixes attach to plural, possessed, `-é` and `-ék` forms without further stem change; `a`/`e` of 3sg `-(j)A` lengthens.
- **Conditions**: slot 6 after slots 1-5.
- **Output**: `házakban`, `házamban`, `házadban`, `házában`, `házunkban`, `házatokban`, `házukban`, `házaimban`, `házaiban`, `kertjében`, `kertjeinkben`, `házéban`, `Péteréknél`.
- **Examples**: `almájával`, `kertjükből`, `lovaikhoz`.
- **Exceptions**: `-vAl` after 1sg/2sg `-m`/`-d`: assimilated doubling (`házammal`, `házaddal`, `házunkkal`, `házatokkal`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LENG-004`, `NOM-CASE-003`.
- **Sources**: [KVF98 p. 193]; [SMNy3 Rebrus p. 803].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-CASE-016 Locative -t/-tt (lexical)
- **Statement**: The locative (`-t` after vowels, `-ott/-ett/-ött` after consonants; `-t` also directly after some consonants) survives only with a closed set of Hungarian place names and in lexicalised adverbs; it is not productive.
- **Conditions**: lexical flag `loc_t` with the stored form.
- **Output**: stored forms; the productive inessive/superessive alternative is always available and more frequent.
  | Place | Locative | Productive alternative |
  |---|---|---|
  | `Pécs` | `Pécsett` | `Pécsen` |
  | `Győr` | `Győrött` | `Győrben` |
  | `Vác` | `Vácott` | `Vácon` |
  | `Kaposvár` | `Kaposvárott` | `Kaposváron` |
  | `Kolozsvár` | `Kolozsvárt` ~ `Kolozsvárott` | `Kolozsváron` |
  | `Székesfehérvár` | `Székesfehérvárt` | `Székesfehérváron` |
  | `Vásárhely` | `Vásárhelyt` | `Vásárhelyen` |
  | lexicalised | `helyütt`, `másutt`, `mindenütt`, `itt`, `ott` (`PND`) | - |
- **Examples**: `Pécsett` [ATT 11] vs `Pécsen` [ATT 8]; `Győrött` [ATT 6] vs `Győrben` [ATT 10]; `Kaposvárott` [ATT 2] vs `Kaposváron` [ATT 8].
- **Exceptions**: single `t` after vowel, double after consonant is normative [AkH12 §38].
- **Predictability**: `lexical` (closed list of about 10 place names)
- **Lexical input**: `loc_t`.
- **Interactions**: `NOM-PROP-006`.
- **Sources**: [AkH12 §38]; [KVF98 p. 192]; [UD-hu Case Loc `helyütt`]; [ATT].
- **Validation**: `attestation`.
- **Confidence**: `normative` (spelling), `consensus` (list).

### NOM-CASE-017 Temporal -kor
- **Statement**: `-kor` ('at (time)') is invariant back, short-vowelled, super-analytic and non-lengthening; it attaches to time-denoting nouns, clock-time numerals and a few pronouns.
- **Conditions**: `time_noun` (holidays, parts of day, events: `karácsony`, `húsvét`, `éjfél`, `dél`, `szürkület`, `születés`), numerals (`ötkor`, `fél hatkor`), `óra` (`tíz órakor`); after a plural only for a recurring event (`ünnepekkor`), not after `-é`/`-ék` (`NOM-MODEL-003`).
- **Output**: `-kor`.
- **Examples**: `karácsonykor`, `húsvétkor`, `éjfélkor`, `hétkor`; `hatkor`, `órakor`, `10:35-kor` (`ORT-NUM-002`).
- **Exceptions**: lexicalised pronominal forms `akkor`, `ekkor`, `máskor`, `olyankor`, `mikor` (`PND`).
- **Predictability**: `rule` (form), `lexical` (licensing nouns)
- **Lexical input**: `time_noun`.
- **Interactions**: `NOM-LENG-002`, `NOM-CASE-014`.
- **Sources**: [AkH12 §35]; [SMNy3 Kiefer p. 493]; [KVF98 p. 192]; [UD-hu Case Tem].
- **Validation**: `attestation` (`karácsonykor` [ATT 11], `éjfélkor` [ATT 8], `hétkor` [ATT 6]).
- **Confidence**: `consensus`.

### NOM-CASE-018 Distributive -(O)nként
- **Statement**: `-nként` ('per, by') attaches with a Type A lowerable linking vowel to unpossessed singular nouns, selects bound alternants, and lengthens final `a`/`e`.
- **Conditions**: unpossessed singular noun (`*házakonként`); numerals (`PND`).
- **Output**: `-nként` after vowels (`fánként`, `falunként`, `óránként`, `hétfőnként`); `-onként/-enként/-önként` non-lowering (`városonként`, `esetenként`, `időnként`); `-anként/-enként` lowering (`házanként`, `fejenként`, `kanalanként`, `tavanként`, `helyenként`, epenthetic lowering `alkalom` -> `alkalmanként`).
- **Examples**: `fejenként` [ATT 9], `óránként` [ATT 11], `percenként` [ATT 10], `alkalmanként` [ATT 11], `helyenként` [ATT 12], `keddenként` [ATT 4], `egyenként` [ATT 13].
- **Exceptions**: `-`
- **Predictability**: `rule` (given `lowering` and alternants)
- **Lexical input**: `lowering`, stem alternants.
- **Interactions**: `NOM-LOW-001`, `NOM-STEM-005`.
- **Sources**: [T04 p. 152]; [SMNy3 Rebrus (33) p. 694 (`bokronként`), (113) p. 743 (`kanalanként`), (135) p. 757 (`művenként`)]; [RST24 (3)]; [UD-hu Case Dis].
- **Validation**: `attestation`, `goldens`.
- **Confidence**: `consensus`.

### NOM-CASE-019 Sociative -(O)stUl
- **Statement**: `-stul/-stül` ('together with') attaches with a Type A lowerable linking vowel to unpossessed singular nouns denoting accompaniments; short `u/ü`; lengthening.
- **Conditions**: unpossessed singular; semantic restriction to accompanying entities (family, clothing, containers).
- **Output**: `-stul/-stül` after vowels (`kutyástul`, `cipőstül`); `-ostul/-estül/-östül` non-lowering (`családostul`, `gyerekestül`, `dobostul`); `-astul/-estül` lowering (`házastul`, `bokrostul` is non-lowering epenthetic).
- **Examples**: `családostul` [ATT 3], `ruhástul`, `csőstül` (idiom `csőstül`).
- **Exceptions**: `-`
- **Predictability**: `rule` (form), `lexical` (usage)
- **Lexical input**: `lowering`
- **Interactions**: `NOM-LINK-005`, `NOM-MODEL-003`.
- **Sources**: [AkH12 §35]; [T04 p. 152]; [SMNy3 Kiefer p. 494].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-CASE-020 Formal -képp(en)
- **Statement**: `-képp` ~ `-képpen` ('by way of') is invariant, written with `pp`, super-analytic and non-lengthening; restricted to singular unpossessed nouns and pronouns.
- **Conditions**: lexicalised use (`például` is essive, not formal).
- **Output**: `-képp` ~ `-képpen` (free variants).
- **Examples**: `példaképpen`, `ajándékképpen`, `tulajdonképpen`, `voltaképpen`, `másképp` ~ `másképpen`, `ekképp`.
- **Exceptions**: `-`
- **Predictability**: `rule` (form), `lexical` (usage)
- **Lexical input**: `none`
- **Interactions**: `NOM-LENG-002`.
- **Sources**: [AkH12 §37]; [KVF98 p. 192]; [SMNy3 Rebrus (188) p. 788].
- **Validation**: `attestation` (`tulajdonképpen` [ATT 16], `másképp` [ATT 14]).
- **Confidence**: `normative`.

### NOM-CASE-021 Distributive-temporal -(O)ntA (lexical)
- **Statement**: `-nta/-nte/-onta/-ente/-önte` ('every (time unit)') is restricted to a closed set of time nouns.
- **Conditions**: lexical.
- **Output**: `naponta`, `hetente`, `havonta` (from `hó` 'month', v-stem), `évente`, `reggelente`, `éjjelente` 9,857, `éjente` 181, `nyaranta`, `telente`, `hajnalonta` 975, `hónaponta` 421, `vasárnaponta` 276, `hétköznaponta` 114, `hétvégente` 6,710 [WC2]; `#koronta` is unattested (0 [WC2]) and not generated.
- **Examples**: `naponta` [ATT 17], `hetente` [ATT 14], `havonta` [ATT 15], `évente` [ATT 18], `reggelente` [ATT 8], `nyaranta` [ATT 7].
- **Exceptions**: productive equivalent `-nként` (`óránként`).
- **Predictability**: `lexical` (about 10 items)
- **Lexical input**: stored forms.
- **Interactions**: `NOM-CASE-018`.
- **Sources**: [KVF98 p. 192]; [T04 p. 152]; [ATT].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### NOM-CASE-022 Restricted and archaic case-like suffixes
- **Statement**: The following suffixes appear in reference grammars as cases but are handled outside productive nominal inflection.
- **Conditions**: per item, as listed in Output.
- **Output**:
  - Multiplicative `-szor/-szer/-ször` (`hatszor`, `hétszer`, `ötször`, `sokszor`): numerals and quantifiers, `PND`.
  - Modal-essive `-An` on numerals (`ketten`, `hárman`, `heten`): `PND`; on adjectives: `NOM-ADJ-010`.
  - Modal `-lag/-leg` (`gyakorlatilag`, `viszonylag`): `NOM-ADJ-012`, `DER`.
  - `-szerte` (`országszerte`, `városszerte`): `DER`.
  - Archaic `-nOtt` / `-nÓl` in postpositions and pronouns (`PND`).
- **Examples**: see Conditions.
- **Exceptions**: `-`
- **Predictability**: `lexical`
- **Lexical input**: `none`
- **Interactions**: `PND-NUMINF-005`, `PND-NUMINF-006`, `DER-*`.
- **Sources**: [KVF98 p. 192]; [SMNy3 Kiefer p. 492].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

---

## ADJ: adjective inflection

### NOM-ADJ-001 Adjectives take nominal inflection when heading a noun phrase or as predicates
- **Statement**: An attributive adjective is uninflected; an adjective used as predicate agrees in number (`-k`), and an adjective heading an elliptical noun phrase takes all nominal suffixes, with adjective lowering.
- **Conditions**: syntactic position (`SYN-AGR`).
- **Output**: attributive `szép házak`; predicative `A házak szépek`; headless `a pirosat`, `a szépeknek`, `a nagyobbal`.
- **Examples**: `piros` -> `pirosak`, `pirosat`, `pirosnak`, `pirossal`, `piroson`; `hosszú` -> `hosszúak`, `hosszút`, `hosszúval`.
- **Exceptions**: possessive suffixes on adjectives occur only in nominalised uses (`a legjobbja`) and follow noun rules.
- **Predictability**: `rule`
- **Lexical input**: `pos`
- **Interactions**: `NOM-LOW-004`, `NOM-PL-003`, `SYN-AGR`.
- **Sources**: [T04 pp. 155-157]; [KVF98 p. 192].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-ADJ-002 Attributive-only and predicative-only adjective forms
- **Statement**: A few adjectives have distinct attributive and predicative/headless stems.
- **Conditions**: lexical pairs.
- **Output**: `kis` (attributive only) vs `kicsi`/`kicsiny` (all positions): `kis ház`, `A ház kicsi`, `a kicsit`; comparative of both `kisebb`; numeral `két`/`kettő` (`PND`).
- **Examples**: `kicsik` [ATT 12], `*kisek`.
- **Exceptions**: `-`
- **Predictability**: `lexical` (1 pair in `NOM`: `kis`/`kicsi`; `két`/`kettő` in `PND`)
- **Lexical input**: `attr_only`.
- **Interactions**: -
- **Sources**: [SMNy3 Rebrus p. 660].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### NOM-ADJ-003 Plural of adjectives
- **Statement**: Adjective plurals follow `NOM-PL-002` with default lowering and `NOM-PL-003` for vowel-final adjectives.
- **Conditions**: `pos=adjective`.
- **Output**: `pirosak`, `szépek`, `bölcsek`, `nagyok`, `fiatalok`, `feketék`, `hosszúak`, `olcsók` ~ `olcsóak`, `városiak`.
- **Examples**: see `NOM-LOW-004`, `NOM-PL-003`.
- **Exceptions**: `NOM-LOW-004`.
- **Predictability**: `rule` with lexical exceptions
- **Lexical input**: `lowering`
- **Interactions**: `NOM-LOW-004`, `NOM-PL-003`.
- **Sources**: [T04 (14), (19b)].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### NOM-ADJ-004 Comparative -bb / -(A)bb
- **Statement**: The comparative is `-bb` after vowel-final stems (with lengthening) and `-abb/-ebb` after consonant-final stems; its linking vowel is always low regardless of the stem's lowering status, except `nagy` -> `nagyobb`.
- **Conditions**: gradable adjective or adverb stem.
- **Output**: `-bb`; `-abb` (`B`), `-ebb` (`F`, `R`); bound alternants of epenthetic and shortening stems.
- **Examples**: `drága` -> `drágább`; `fekete` -> `feketébb`; `olcsó` -> `olcsóbb`; `szomorú` -> `szomorúbb`; `egyszerű` -> `egyszerűbb`; `gyakori` -> `gyakoribb`; `piros` -> `pirosabb`; `új` -> `újabb`; `fiatal` -> `fiatalabb`; `gazdag` -> `gazdagabb`; `boldog` -> `boldogabb`; `vastag` -> `vastagabb`; `idős` -> `idősebb`; `mély` -> `mélyebb`; `bátor` -> `bátrabb`; `nehéz` -> `nehezebb`; `kevés` -> `kevesebb`; `nagy` -> `nagyobb`.
- **Exceptions**: `NOM-ADJ-005`.
- **Predictability**: `rule`
- **Lexical input**: stem alternants.
- **Interactions**: `NOM-LOW-001` (self-lowering suffix), `NOM-LENG-001`.
- **Sources**: [RST24 (3), fn. 14 p. 144]; [T04 p. 157]; [AkH12 §38]; [ATT `fiatalabb` 14, `gazdagabb` 10, `nagyobb` 22].
- **Validation**: `attestation`, `goldens`.
- **Confidence**: `consensus`.

### NOM-ADJ-005 Irregular comparatives
- **Statement**: The following adjectives and adverbs have irregular comparative stems.
- **Conditions**: lexical.
- **Output**:
  | Positive | Comparative | Note |
  |---|---|---|
  | `jó` | `jobb` | suppletive-like shortening |
  | `sok` | `több` | suppletive |
  | `kicsi`, `kis`, `kicsiny` | `kisebb` | stem `kis-` |
  | `szép` | `szebb` | vowel shortening, single `b` stem `sze-` |
  | `nagy` | `nagyobb` | mid linking vowel (unique) |
  | `könnyű` | `könnyebb` | final vowel deleted |
  | `hosszú` | `hosszabb` | final vowel deleted |
  | `lassú` | `lassabb` ~ `lassúbb` | `lassabb` preferred (60,918 : 11,354 [WC2]); the only doublet of the table (`hosszúbb` 19, `könnyűbb` 90, `ifjúbb` 4) |
  | `ifjú` | `ifjabb` | final vowel deleted |
  | `bő` | `bővebb` | v-stem |
  | `nehéz`, `kevés` | `nehezebb`, `kevesebb` | shortening stems |
  | `bátor` | `bátrabb` | epenthetic |
  | adverb `későn` / adj `késő` | `később`, `későbbi` | - |
  | adverb `rég`, adj `régi` | `régebben`, `régebbi` | built on `rég` |
  | adverb `hamar` | `hamarabb` | - |
  | adverb `szívesen` | `szívesebben` | - |
  | - | `inkább` | lexicalised comparative 'rather' |
- **Examples**: `jobb` [ATT 21], `több` [ATT 25], `kisebb` [ATT 20], `szebb` [ATT 13] vs `szépebb` `-`, `könnyebb` [ATT 16] vs `könnyűbb` `-`, `hosszabb` [ATT 17] vs `hosszúbb` `-`, `bővebb` [ATT 13], `ifjabb` [ATT 9].
- **Exceptions**: `hű` has two current comparatives, `hűbb` 641 ~ `hívebb` 547 (54 : 46 of 1,192 tokens) [WC2]; both are generated, `hűbb` slightly preferred as the general form and `hívebb` leading 2:1 in literary and academic prose. The earlier "neither attested" note is superseded. Homograph control matters: `hívebb` is tagged under the etymological doublet lemma `hív`, so filtering on lemma `hű` alone returns 0 and would produce a false categorical verdict.
- **Predictability**: `lexical` (about 12 items)
- **Lexical input**: stored comparative stem.
- **Interactions**: `NOM-ADJ-008`, `NOM-ADJ-006`.
- **Sources**: [T04 fn. 12 p. 145]; [emMorph stemalt.rul `szép`, study only]; [ATT]; [WC2]; [MTA-HELYES]; [ATT evidence/corpus-variants.md 2026-09-16]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `attestation` (`[/Adj][_Comp/Adj]` with lemma control).
- **Confidence**: `consensus`; `measured` and register-conditioned for `hű` (`OQ-03-007` resolved).

### NOM-ADJ-006 Superlative leg- and excessive legesleg-
- **Statement**: The superlative is `leg-` + comparative; the excessive is `legesleg-` + comparative; both are written solid.
- **Conditions**: comparative form.
- **Output**: `legnagyobb`, `legjobb`, `legszebb`, `legkisebb`, `leghosszabb`, `legeslegjobb`, `legeslegszebb`.
- **Examples**: `legjobb` [ATT 21], `legnagyobb` [ATT 22], `legeslegjobb` [ATT 2].
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-ADJ-004`, `NOM-ADJ-009`.
- **Sources**: [AkH12 §38 (comparative `-bb`)]; [ATT].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### NOM-ADJ-007 Inflection of comparatives and superlatives
- **Statement**: Comparative and superlative stems are lowering, consonant-final stems; `-bb` + a suffix beginning with `b` reduces three `b` letters to two.
- **Conditions**: comparative stem + suffix.
- **Output**: plural `-bbak/-bbek`; accusative `-bbat/-bbet`; `-bban/-bben` (`nagyobban`); `-bbal/-bbel` (`jobbal`, `szebbel`).
- **Examples**: `nagyobbak` [ATT 10], `nagyobbat` [ATT 8], `kisebbek` [ATT 9], `kisebbeket` [ATT 4], `jobban` (adverb, `NOM-ADJ-013`).
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LOW-002`, `NOM-CASE-005`.
- **Sources**: [AkH12 §93 (`nagyobban`)]; [ATT].
- **Validation**: `attestation`.
- **Confidence**: `normative`.

### NOM-ADJ-008 Final-vowel deletion in ú/ű adjectives
- **Statement**: Four adjectives delete their final vowel before comparative `-(A)bb` and adverbial `-(A)n`: `könnyű`, `hosszú`, `lassú`, `ifjú`; other `ú/ű` adjectives keep it.
- **Conditions**: lexical.
- **Output**: `könnyebb`, `könnyen`; `hosszabb`, `hosszan`; `lassabb`, `lassan`; `ifjabb`, `ifjan`; but `szomorúbb`, `szomorúan`; `keserűbb`, `keserűen`.
- **Examples**: `könnyen` [ATT 19] vs `könnyűen` `-`; `lassan` [ATT 18] vs `lassúan` `-`; `hosszan` [ATT 14].
- **Exceptions**: plural and case keep the vowel (`hosszúak`, `könnyűek`, `hosszút`).
- **Predictability**: `lexical` (4 items)
- **Lexical input**: truncated stem for `-bb`, `-An`.
- **Interactions**: `NOM-ADJ-005`, `NOM-ADJ-010`, `NOM-STEM-012`.
- **Sources**: [T04 fn. 12 p. 145]; [ATT].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### NOM-ADJ-009 Partitive -ik on comparatives
- **Statement**: A comparative (or superlative) selecting one of two or more is suffixed with `-ik`, used as a headless noun phrase or attributively with a definite article.
- **Conditions**: comparative stem.
- **Output**: `nagyobbik`, `kisebbik`, `jobbik`, `idősebbik`, `szebbik`, `hosszabbik`; colloquial superlative `?legnagyobbik`; inflected like a lowering noun: `a kisebbiket`, `a nagyobbikat`, `a nagyobbikba`.
- **Examples**: `nagyobbik` -> `nagyobbikat` 1,218 (not `*nagyobbikot`, 3), `kisebbik` -> `kisebbiket` 1,213 (`*kisebbikot` 0), `jobbik` [ATT 15]; `-ik` takes `-at`/`-et`, never `-ot` [WC2; pattern level `-ikat` 1,483,254 : `-ikot` 63,355, `-iket` 2,746,511].
- **Exceptions**: the proper noun `Jobbik` (party) is a non-lowering name (`Jobbikot` [ATT 5]).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-ADJ-004`, `NOM-PROP-001`.
- **Sources**: [emMorph stemalt.rul `+ik[KIEM]`, study only]; [ATT]; [WC2]; [ATT evidence/corpus-variants.md 2026-09-16].
- **Validation**: `attestation` (`[Acc]` / `[Pl]` per lemma and at pattern level).
- **Confidence**: `consensus` for forms; `measured` and near-categorical for the accusative linking vowel - the earlier `?` on `nagyobbikot` becomes an exclusion (`OQ-03-008` resolved).

### NOM-ADJ-010 Adverbs in -(A)n / -(O)n
- **Statement**: Manner adverbs are formed from adjectives by `-n` after vowels and a lowerable linking vowel + `n` after consonants; the linking vowel follows the adjective's lowering status.
- **Conditions**: adjective not in `NOM-ADJ-011`.
- **Output**:
  - Vowel-final: `-n` with lengthening (`drágán`, `feketén`, `sárgán`, `olcsón`); `ú/ű`, derived `-i`, `-ó/-ő` participles: `-an/-en` (`szomorúan`, `keserűen`, `hűen`, `bántóan`, `kiválóan`, `borzasztóan`, `hasonlóan`, `emberien`).
  - Consonant-final lowering: `-an/-en` (`pirosan`, `gyorsan`, `halkan`, `konkrétan`, `szépen`, `erősen`, `bölcsen`, `hidegen`).
  - Consonant-final non-lowering: `-on` (`nagyon` 'very', `vakon`, `fiatalon`, `gazdagon`, `szabadon`, `vastagon`).
- **Examples**: `gazdagon` [ATT 10] vs `gazdagan` `-`; `szabadon` [ATT 17]; `vastagon` [ATT 9]; `halkan` [ATT 11]; `konkrétan` [ATT 13] vs `konkréton` `-`; `olcsón` [ATT 11] vs `olcsóan` `-`.
- **Exceptions**: `boldog` (non-lowering plural `boldogok`) forms the manner adverb `boldogan` [ATT 12] (`*boldogon` in that reading; the string is the superessive of `boldog` 'happy person'); `jó` -> `jól`; `kevés` -> `kevéssé`; `kicsi` -> `kicsit`, `kissé`; `nagy` 'very' `nagyon` vs manner 'greatly' `nagyon`.
- **Predictability**: `rule` given `lowering`, with lexical exceptions
- **Lexical input**: `lowering`, stored exceptions.
- **Interactions**: `NOM-LOW-004`, `NOM-ADJ-008`, `NOM-CASE-011` (distinct from superessive).
- **Sources**: [T04 (14)-(15) pp. 155-156]; [RST24 (3), p. 159 (`konkrétan`)]; [ATT].
- **Validation**: `attestation`.
- **Confidence**: `consensus`; `tendency` for non-lowering exceptions.

### NOM-ADJ-011 Adverbs in -Ul/-ül
- **Statement**: Adjectives in privative `-(V)tlAn/-talan/-telen`, language/ethnic adjectives and a small lexical set form adverbs with `-ul/-ül` instead of `-An`.
- **Conditions**:
  - Privative adjectives: `váratlanul`, `szerencsétlenül`, `ügyetlenül`, `tudatlanul`, `gondatlanul`, `feltétlenül`, `végtelenül`, `öntudatlanul`, `meztelenül` (`meztelen` pattern).
  - Language names: `magyarul`, `németül`, `angolul`, `franciául`, `latinul`.
  - Lexical: `rossz` -> `rosszul`, `gonosz` -> `gonoszul`, `idegen` -> `idegenül`, `egyedül`, `bolond` -> `bolondul`, `jó` -> `jól` (`-l` form).
- **Output**: `-ul` (`B`), `-ül` (`F`, `R`).
- **Examples**: `váratlanul` [ATT 13] vs `váratlanan` `-`; `szerencsétlenül` [ATT 8]; `rosszul` [ATT 16]; `gonoszul` [ATT 4] vs `gonoszan` `-`; `idegenül` [ATT 4]; `magyarul` [ATT 15].
- **Exceptions**: `-`
- **Predictability**: `rule` for privative and language classes; `lexical` for the rest (about 6 items)
- **Lexical input**: `adv_ul` flag.
- **Interactions**: `NOM-CASE-009`, `DER-*`.
- **Sources**: [AkH12 §35 (`magyarul`)]; [KVF98 p. 192]; [ATT].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### NOM-ADJ-012 Adverbs in -lAg
- **Statement**: `-lag/-leg` forms adverbs mainly from adjectives in `-i` and from participles; it is invariant two-way, non-lengthening on `-i` stems.
- **Conditions**: derived `-i` adjectives, some participles and nouns.
- **Output**: `gyakorlatilag`, `elméletileg`, `eredetileg`, `valószínűleg`, `viszonylag`, `aránylag`, `hallgatólag`, `visszamenőleg`, `táviratilag`.
- **Examples**: `valószínűleg` [ATT 19], `gyakorlatilag` [ATT 17], `viszonylag` [ATT 17].
- **Exceptions**: productivity and base selection belong to `DER`.
- **Predictability**: `lexical`
- **Lexical input**: stored adverbs.
- **Interactions**: `DER-ADV`.
- **Sources**: [SMNy3 Kiefer p. 492]; [KVF98 p. 192].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### NOM-ADJ-013 Comparative adverbs
- **Statement**: Comparative adverbs are comparative stem + `-an/-en`; superlative adverbs add `leg-`.
- **Conditions**: gradable adverb.
- **Output**: `jobban`, `szebben`, `gyorsabban`, `lassabban`, `legjobban`, `leginkább`.
- **Examples**: `jobban` [ATT 20], `szebben` [ATT 7], `legjobban` [ATT 16].
- **Exceptions**: `inkább`, `hamarabb`, `később` are adverbial without `-an`.
- **Predictability**: `rule` with lexical exceptions
- **Lexical input**: `none`
- **Interactions**: `NOM-ADJ-004`, `NOM-ADJ-010`.
- **Sources**: [ATT].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

---

## PROP: proper names

### NOM-PROP-001 Proper names inflect like common nouns
- **Statement**: Personal and place names take the full nominal paradigm; harmony and consonant-dependent allomorphy follow pronunciation; `a`/`e` lengthening applies.
- **Conditions**: `pos=proper noun`.
- **Output**: `Tompát`, `Tompával`, `Ráckevén`, `Petőfit`, `Petőfinek`, `Kovácsot`, `Kovácsnak`, `Goethének` (`NOM-LENG-005`), `Bushnak` by pronunciation (`PH-G2P-008`).
- **Examples**: `Tompát`, `Ráckevéig` [AkH12 §29].
- **Exceptions**: `NOM-PROP-002`..`NOM-PROP-004`.
- **Predictability**: `rule`
- **Lexical input**: pronunciation, `harmony`.
- **Interactions**: `ORT-NAME-009`, `ORT-FOR-016`, `PH-G2P`.
- **Sources**: [AkH12 §29, §211, §215-217].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### NOM-PROP-002 No three-letter reduction in names
- **Statement**: Names ending in a doubled consonant letter keep it before a suffix beginning with the same consonant (with hyphen, `ORT-NAME-012`); the phonological form is unchanged.
- **Conditions**: proper name; stem-final doubled letter; suffix starts with the same letter or `v`-assimilation.
- **Output**: `Széll-lel`, `Kiss-sel`, `Papp-pal`, `Mariann-nal`, `Bonn-nal`, `Tallinn-nak`, `Scott-tal`, `Hermann-nál`, `Tarr-ról`.
- **Examples**: see Output.
- **Exceptions**: AkH11 simplified Hungarian given names (`Mariannal`); AkH12 does not.
- **Predictability**: `rule`
- **Lexical input**: proper-name flag.
- **Interactions**: `NOM-CASE-005`, `ORT-NAME-012`, `ORT-FOR-013`.
- **Sources**: [AkH12 §93, §160 c), §217 c)]; [AkH11 §94].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### NOM-PROP-003 Possessive -ja preference in names
- **Statement**: Proper names do not obey the sibilant/palatal `-a` rule and prefer `-ja/-je`.
- **Conditions**: proper name in possessed use.
- **Output**: `Konczja`, `Bandicsja`, `Fónagyja`, `Keményje`.
- **Examples**: see Output.
- **Exceptions**: names homonymous with common nouns may follow the common noun in figurative use; unverified by corpus (`NOM-OPEN-9`).
- **Predictability**: `tendency`
- **Lexical input**: proper-name flag.
- **Interactions**: `NOM-POSS-005`.
- **Sources**: [SMNy3 Kiefer p. 510 rule 2]; [SMNy3 Rebrus p. 806].
- **Validation**: `goldens`.
- **Confidence**: `tendency`.

### NOM-PROP-004 Stem alternations in names
- **Statement**: Place names homonymous with or derived from epenthetic stems keep the free stem before inflectional suffixes and alternate only before derivational `-i`; personal names are non-alternating and non-lowering by default.
- **Conditions**: proper name with epenthetic-shaped final syllable.
- **Output**: `Eger` -> `Egert`, `Egerben`, `Egeren`, `Egerem`; derivative `egri`; `Sándor` -> `Sándort`, `Sándorok`.
- **Examples**: `Egert` [ATT 2] vs `Egret` `-`; `egri` [ATT 14].
- **Exceptions**: surnames identical to shortening or v-stem nouns (`Madár`, `Kő`, `Víz`): alternation status not documented in sources (`NOM-OPEN-9`); generate non-alternating forms.
- **Predictability**: `tendency`
- **Lexical input**: proper-name flag.
- **Interactions**: `NOM-STEM-005`, `DER-ADJ`.
- **Sources**: [SMNy3 Rebrus p. 693]; [T04 p. 161 (new items non-lowering)]; [ATT].
- **Validation**: `attestation`.
- **Confidence**: `consensus` for place names; `contested` for surnames.

### NOM-PROP-005 Associative -ék and names
- **Statement**: Family and given names take `-ék` productively (`NOM-ASSOC-001`); the married-name suffix `-né` belongs to `DER`.
- **Conditions**: personal names.
- **Output**: `Kovácsék`, `Szabóék`, `Vargáék`, `Péterék`, `Erzsiék`.
- **Examples**: see Output.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-ASSOC-001`, `DER-NOUN-024`.
- **Sources**: [SMNy3 Kiefer p. 501].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### NOM-PROP-006 Place names: inner vs surface local cases
- **Statement**: Each place name selects either the interior (`-bAn -bA -bÓl`) or surface (`-(O)n -rA -rÓl`) local case series; the choice is lexical with tendencies.
- **Conditions**: `place_case` stored per name.
- **Output**:
  - Countries and foreign places: interior (`Németországban`, `Bécsben`, `Párizsban`), except `Magyarország` -> `Magyarországon` and some islands/regions (`Cipruson`, `Krétán`, `Máltán`, `Izlandon`, `Balkánon`; but `Szicíliában` [ATT 4] vs `Szicílián` [ATT -]).
  - Hungarian settlements: majority surface (`Budapesten`, `Szegeden`, `Pécsen`, `Miskolcon`, `Kecskeméten`, `Nyíregyházán`, `Szombathelyen`, `Kaposváron`, `Budán`, `Pesten`); a minority interior (`Debrecenben`, `Győrben`, `Egerben`, `Sopronban`, `Veszprémben`, `Esztergomban`, `Tihanyban`, `Komáromban`); KVF98 lists `Badacsony`, `Debrecen`, `Eger`, `Esztergom`, `Győr`, `Marcali`, `Sopron`, `Tihany` as interior and about 30 major towns (`Szolnok`, `Kőszeg`, `Keszthely`, `Szigetvár`, `Hódmezővásárhely`) as surface; the ending-based tendency is now measured over **2,819 settlement names** (1,381 surface, 1,018 interior, 420 both) [WC2]. Surface dominates for `-vár` (39 of 42), `-egy` (19/20), `-kút` (15/15), `-föld` (13/15), `-erdő` (12/14), `-hely` and `-ely` (21/26) and `-ács` (13/16); interior dominates for `-ány` (35/46), `-ény` (28/31), `-ony` (23/24), `-ország` (21/22) and for the foreign endings `-ton`, `-burg`, `-tán`.
- **Examples**: `Budapesten` [ATT 18], `Debrecenben` [ATT 12], `Magyarországon` [ATT 21], `Győrben` [ATT 10], `Szegeden` [ATT 12], `Sopronban` [ATT 10].
- **Exceptions**: locative `-t/-tt` doublets (`NOM-CASE-016`).
- **Predictability**: `lexical` with a measured ending-based `tendency` (2,819 names)
- **Lexical input**: `place_case`, from `settlement-local-cases.tsv`, 2,819 rows, in `local/lists/settlement-local-cases.tsv` - working data, not yet read by the build.
- **Interactions**: `NOM-CASE-010`, `NOM-CASE-011`, `NOM-CASE-016`.
- **Sources**: [KVF98 pp. 240-241]; [ATT]; [WC2]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `attestation` (90/10 three-way decision, `both` reported as a class).
- **Confidence**: `tendency`, now quantified (`OQ-03-013` resolved).

### NOM-PROP-007 Instrumental and translative with names
- **Statement**: With names the `v` of `-vAl/-vÁ` assimilates to the pronounced final consonant; archaic and foreign final letters are kept and the pronounced consonant is added.
- **Conditions**: proper name, consonant-final pronunciation.
- **Output**: `Ádámmal`, `Bálinttal`, `Kodállyal`, `Szabolccsal`, `Arannyal`; `Kossuthtal`, `Véghgel`, `Móriczcal`, `Babitscsal`, `Madáchcsá`, `Marxszal`, `Félixszel`; `Balzackal`, `Bachhal`, `Wellsszel`, `Greenwichcsel`; silent-final foreign names with hyphen: `Rousseau-val`, `Molière-rel`, `guillotine-nal`.
- **Examples**: `Arannyal` [ATT 8], `Ádámmal` [ATT 4].
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: pronunciation of final segment.
- **Interactions**: `NOM-CASE-003`, `ORT-NAME-011`, `ORT-FOR-011`, `ORT-FOR-012`.
- **Sources**: [AkH12 §92, §160 a)-c), §216 b), §217 a)].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

---

## PARA: reference paradigms

### NOM-PARA-001 Noun reference paradigms
- **Statement**: The following forms are goldens; the columns not shown (`-ért`, `-ig`, `-vÁ`, `-bA`, `-bÓl`, `-rA`, `-rÓl`, `-nÁl`, `-tÓl`) are free stem + suffix with the harmony of the `-bAn` and `-hOz` columns and the assimilation of the `-vAl` column.
- **Conditions**: standard written language; variants separated by `~`, preferred first.
- **Output**:

Table A: plural, accusative, cases (singular)
| Stem | Class | PL | ACC | ACC.PL | SUP | INS | DAT | INE | ALL | -ként |
|---|---|---|---|---|---|---|---|---|---|---|
| `ház` | B, lowering | `házak` | `házat` | `házakat` | `házon` | `házzal` | `háznak` | `házban` | `házhoz` | `házként` |
| `kert` | F, CC | `kertek` | `kertet` | `kerteket` | `kerten` | `kerttel` | `kertnek` | `kertben` | `kerthez` | `kertként` |
| `gyümölcs` | R, non-lowering | `gyümölcsök` | `gyümölcsöt` | `gyümölcsöket` | `gyümölcsön` | `gyümölccsel` | `gyümölcsnek` | `gyümölcsben` | `gyümölcshöz` | `gyümölcsként` |
| `fül` | R, lowering | `fülek` | `fület` | `füleket` | `fülön` | `füllel` | `fülnek` | `fülben` | `fülhöz` | `fülként` |
| `alma` | B, a-final | `almák` | `almát` | `almákat` | `almán` | `almával` | `almának` | `almában` | `almához` | `almaként` |
| `kefe` | F, e-final | `kefék` | `kefét` | `keféket` | `kefén` | `kefével` | `kefének` | `kefében` | `keféhez` | `kefeként` |
| `hajó` | B, V-final | `hajók` | `hajót` | `hajókat` | `hajón` | `hajóval` | `hajónak` | `hajóban` | `hajóhoz` | `hajóként` |
| `kocsi` | B, i-final | `kocsik` | `kocsit` | `kocsikat` | `kocsin` | `kocsival` | `kocsinak` | `kocsiban` | `kocsihoz` | `kocsiként` |
| `teknő` | R, V-final | `teknők` | `teknőt` | `teknőket` | `teknőn` | `teknővel` | `teknőnek` | `teknőben` | `teknőhöz` | `teknőként` |
| `ló` | v-stem | `lovak` | `lovat` | `lovakat` | `lovon` | `lóval` | `lónak` | `lóban` | `lóhoz` | `lóként` |
| `tó` | v-stem | `tavak` | `tavat` | `tavakat` | `tavon` | `tóval` | `tónak` | `tóban` | `tóhoz` | `tóként` |
| `kő` | v-stem | `kövek` | `követ` | `köveket` | `kövön` | `kővel` | `kőnek` | `kőben` | `kőhöz` | `kőként` |
| `mű` | v-stem | `művek` | `művet` | `műveket` | `művön` | `művel` | `műnek` | `műben` | `műhöz` | `műként` |
| `bokor` | epenthetic | `bokrok` | `bokrot` | `bokrokat` | `bokron` | `bokorral` | `bokornak` | `bokorban` | `bokorhoz` | `bokorként` |
| `tükör` | epenthetic | `tükrök` | `tükröt` | `tükröket` | `tükrön` | `tükörrel` | `tükörnek` | `tükörben` | `tükörhöz` | `tükörként` |
| `lélek` | epenthetic + shortening | `lelkek` | `lelket` | `lelkeket` | `lelken` | `lélekkel` | `léleknek` | `lélekben` | `lélekhez` | `lélekként` |
| `kéz` | shortening | `kezek` | `kezet` | `kezeket` | `kézen` | `kézzel` | `kéznek` | `kézben` | `kézhez` | `kézként` |
| `madár` | shortening | `madarak` | `madarat` | `madarakat` | `madáron` | `madárral` | `madárnak` | `madárban` | `madárhoz` | `madárként` |
| `víz` | shortening | `vizek` | `vizet` | `vizeket` | `vízen` ~ `vizen` | `vízzel` | `víznek` | `vízben` | `vízhez` | `vízként` |
| `híd` | shortening, antiharmonic | `hidak` | `hidat` | `hidakat` | `hídon` | `híddal` | `hídnak` | `hídban` | `hídhoz` | `hídként` |
| `út` | shortening | `utak` | `utat` | `utakat` | `úton` | `úttal` | `útnak` | `útban` | `úthoz` | `útként` |
| `tűz` | shortening | `tüzek` | `tüzet` | `tüzeket` | `tűzön` | `tűzzel` | `tűznek` | `tűzben` | `tűzhöz` | `tűzként` |
| `kalap` | regular | `kalapok` | `kalapot` | `kalapokat` | `kalapon` | `kalappal` | `kalapnak` | `kalapban` | `kalaphoz` | `kalapként` |
| `ablak` | regular | `ablakok` | `ablakot` | `ablakokat` | `ablakon` | `ablakkal` | `ablaknak` | `ablakban` | `ablakhoz` | `ablakként` |
| `ajtó` | poss. alternant | `ajtók` | `ajtót` | `ajtókat` | `ajtón` | `ajtóval` | `ajtónak` | `ajtóban` | `ajtóhoz` | `ajtóként` |
| `toll` | geminate, lowering | `tollak` | `tollat` | `tollakat` | `tollon` | `tollal` | `tollnak` | `tollban` | `tollhoz` | `tollként` |
| `orr` | geminate, lexical vowel | `orrok` | `orrot` | `orrokat` | `orron` | `orral` | `orrnak` | `orrban` | `orrhoz` | `orrként` |
| `könny` | geminate, lowering | `könnyek` | `könnyet` | `könnyeket` | `könnyön` | `könnyel` | `könnynek` | `könnyben` | `könnyhöz` | `könnyként` |
| `stressz` | loan geminate | `stresszek` | `stresszt` | `stresszeket` | `stresszen` | `stresszel` | `stressznek` | `stresszben` | `stresszhez` | `stresszként` |
| `pénz` | CC, bare acc. | `pénzek` | `pénzt` | `pénzeket` | `pénzen` | `pénzzel` | `pénznek` | `pénzben` | `pénzhez` | `pénzként` |
| `fájl` | CC, bare acc. | `fájlok` | `fájlt` | `fájlokat` | `fájlon` | `fájllal` | `fájlnak` | `fájlban` | `fájlhoz` | `fájlként` |

Table B: possessives and anaphoric possessive
| Stem | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl | PPL 1sg | PPL 3sg | PPL 2pl | `-é` |
|---|---|---|---|---|---|---|---|---|---|---|
| `ház` | `házam` | `házad` | `háza` | `házunk` | `házatok` | `házuk` | `házaim` | `házai` | `házaitok` | `házé` |
| `kert` | `kertem` | `kerted` | `kertje` | `kertünk` | `kertetek` | `kertjük` | `kertjeim` | `kertjei` | `kertjeitek` | `kerté` |
| `gyümölcs` | `gyümölcsöm` | `gyümölcsöd` | `gyümölcse` | `gyümölcsünk` | `gyümölcsötök` | `gyümölcsük` | `gyümölcseim` | `gyümölcsei` | `gyümölcseitek` | `gyümölcsé` |
| `fül` | `fülem` | `füled` | `füle` | `fülünk` | `fületek` | `fülük` | `füleim` | `fülei` | `füleitek` | `fülé` |
| `alma` | `almám` | `almád` | `almája` | `almánk` | `almátok` | `almájuk` | `almáim` | `almái` | `almáitok` | `almáé` |
| `kefe` | `kefém` | `keféd` | `keféje` | `kefénk` | `kefétek` | `keféjük` | `keféim` | `keféi` | `keféitek` | `keféé` |
| `hajó` | `hajóm` | `hajód` | `hajója` | `hajónk` | `hajótok` | `hajójuk` | `hajóim` | `hajói` | `hajóitok` | `hajóé` |
| `kocsi` | `kocsim` | `kocsid` | `kocsija` | `kocsink` | `kocsitok` | `kocsijuk` | `kocsijaim` | `kocsijai` | `kocsijaitok` | `kocsié` |
| `teknő` | `teknőm` | `teknőd` | `teknője` | `teknőnk` | `teknőtök` | `teknőjük` | `teknőim` | `teknői` | `teknőitek` | `teknőé` |
| `ló` | `lovam` | `lovad` | `lova` | `lovunk` | `lovatok` | `lovuk` | `lovaim` | `lovai` | `lovaitok` | `lóé` |
| `tó` | `tavam` | `tavad` | `tava` | `tavunk` | `tavatok` | `tavuk` | `tavaim` | `tavai` | `tavaitok` | `tóé` |
| `kő` | `kövem` | `köved` | `köve` | `kövünk` | `követek` | `kövük` | `köveim` | `kövei` | `köveitek` | `kőé` |
| `mű` | `művem` | `műved` | `műve` | `művünk` | `művetek` | `művük` | `műveim` | `művei` | `műveitek` | `műé` |
| `bokor` | `bokrom` | `bokrod` | `bokra` | `bokrunk` | `bokrotok` | `bokruk` | `bokraim` | `bokrai` | `bokraitok` | `bokoré` |
| `tükör` | `tükröm` | `tükröd` | `tükre` | `tükrünk` | `tükrötök` | `tükrük` | `tükreim` | `tükrei` | `tükreitek` | `tüköré` |
| `lélek` | `lelkem` | `lelked` | `lelke` | `lelkünk` | `lelketek` | `lelkük` | `lelkeim` | `lelkei` | `lelkeitek` | `léleké` |
| `kéz` | `kezem` | `kezed` | `keze` | `kezünk` | `kezetek` | `kezük` | `kezeim` | `kezei` | `kezeitek` | `kézé` |
| `madár` | `madaram` | `madarad` | `madara` | `madarunk` | `madaratok` | `madaruk` | `madaraim` | `madarai` | `madaraitok` | `madáré` |
| `víz` | `vizem` | `vized` | `vize` | `vizünk` | `vizetek` | `vizük` | `vizeim` | `vizei` | `vizeitek` | `vízé` |
| `híd` | `hidam` | `hidad` | `hídja` | `hidunk` | `hidatok` | `hídjuk` | `hídjaim` | `hídjai` | `hídjaitok` | `hídé` |
| `út` | `utam` | `utad` | `útja` | `utunk` | `utatok` | `útjuk` | `útjaim` (`?utaim`) | `útjai` (`?utai`) | `útjaitok` (`?utaitok`) | `úté` |
| `tűz` | `tüzem` | `tüzed` | `tüze` | `tüzünk` | `tüzetek` | `tüzük` | `tüzeim` | `tüzei` | `tüzeitek` | `tűzé` |
| `kalap` | `kalapom` | `kalapod` | `kalapja` | `kalapunk` | `kalapotok` | `kalapjuk` | `kalapjaim` | `kalapjai` | `kalapjaitok` | `kalapé` |
| `ablak` | `ablakom` | `ablakod` | `ablaka` | `ablakunk` | `ablakotok` | `ablakuk` | `ablakaim` | `ablakai` | `ablakaitok` | `ablaké` |
| `ajtó` | `ajtóm` | `ajtód` | `ajtaja` ~ `ajtója` | `ajtónk` | `ajtótok` | `ajtajuk` ~ `ajtójuk` | `ajtóim` ~ `ajtajaim` | `ajtói` ~ `ajtajai` | `ajtóitok` ~ `ajtajaitok` | `ajtóé` |
| `toll` | `tollam` | `tollad` | `tolla` | `tollunk` | `tollatok` | `tolluk` | `tollaim` | `tollai` | `tollaitok` | `tollé` |
| `orr` | `orrom` | `orrod` | `orra` | `orrunk` | `orrotok` | `orruk` | `orraim` | `orrai` | `orraitok` | `orré` |
| `könny` | `könnyem` | `könnyed` | `könnye` | `könnyünk` | `könnyetek` | `könnyük` | `könnyeim` | `könnyei` | `könnyeitek` | `könnyé` |
| `stressz` | `stresszem` | `stresszed` | `stressze` | `stresszünk` | `stresszetek` | `stresszük` | `stresszeim` | `stresszei` | `stresszeitek` | `stresszé` |
| `pénz` | `pénzem` | `pénzed` | `pénze` | `pénzünk` | `pénzetek` | `pénzük` | `pénzeim` | `pénzei` | `pénzeitek` | `pénzé` |
| `fájl` | `fájlom` | `fájlod` | `fájlja` | `fájlunk` | `fájlotok` | `fájljuk` | `fájljaim` | `fájljai` | `fájljaitok` | `fájlé` |

- **Examples**: the tables.
- **Exceptions**: `könnyé`, `stresszé` are homographs of the translative (`NOM-CASE-004`); `orra` is both 3sg possessive and sublative (`NOM-CASE-012`); `művel` is also a verb.
- **Predictability**: `rule` given the lexical properties in column Class
- **Lexical input**: properties per stem as in `NOM-STEM-*`, `NOM-LOW-*`, `NOM-POSS-*`.
- **Interactions**: all `NOM` rules.
- **Sources**: [UM-hun] (all 31 stems cross-checked; `mű` second free-stem lexeme ignored); [SMNy3 Rebrus tables (33), (113), (135), (136), (180), (213)]; [ATT].
- **Validation**: `goldens`, `oracle:UniMorph`, `oracle:hunmorph-foma`.
- **Confidence**: `consensus`.

### NOM-PARA-002 Adjective reference paradigms
- **Statement**: The following adjective forms are goldens.
- **Conditions**: standard written language.
- **Output**:
  | Stem | PL | ACC | ACC.PL | DAT | INS | SUP | Comparative | Comp. PL | Superlative | Excessive | `-ik` | Adverb | Comp. adverb |
  |---|---|---|---|---|---|---|---|---|---|---|---|---|---|
  | `szép` | `szépek` | `szépet` | `szépeket` | `szépnek` | `széppel` | `szépen` | `szebb` | `szebbek` | `legszebb` | `legeslegszebb` | `szebbik` | `szépen` | `szebben` |
  | `piros` | `pirosak` | `pirosat` | `pirosakat` | `pirosnak` | `pirossal` | `piroson` | `pirosabb` | `pirosabbak` | `legpirosabb` | `legeslegpirosabb` | `pirosabbik` | `pirosan` | `pirosabban` |
  | `hosszú` | `hosszúak` | `hosszút` | `hosszúakat` | `hosszúnak` | `hosszúval` | `hosszún` | `hosszabb` | `hosszabbak` | `leghosszabb` | `legesleghosszabb` | `hosszabbik` | `hosszan` | `hosszabban` |
- **Examples**: the table.
- **Exceptions**: `szépen` is both superessive and adverb (front stem: lowered and mid linking vowel coincide).
- **Predictability**: `rule` with `NOM-ADJ-005`, `NOM-ADJ-008` lexical stems
- **Lexical input**: `lowering`, comparative stem.
- **Interactions**: `NOM-ADJ-*`.
- **Sources**: [T04 (14)-(15)]; [ATT `szépek` 10, `szebbek` 5, `hosszúak` 8, `pirosak` 3, `pirosat` 5].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `consensus`.

---

## Open questions
- `NOM-OPEN-1`: closed inventory of polysyllabic `é`~`e` shortening stems (SMNy3 gives "+20" without listing); verify the reconstructed list in `NOM-STEM-008` against hunmorph-foma and a dictionary.
- `NOM-OPEN-2`: `mag` (`magok` ~ `magvak`, `magja` ~ `magva`): sense conditions and preference.
- `NOM-OPEN-3`: acceptability of `-ként` after `-é` (`házéként`): SMNy3 gives contradictory judgments.
- `NOM-OPEN-4`: full list of non-lowering consonant-final adjectives (`nagy`, `agg`, `fiatal`, `gazdag`, `boldog`, `vastag`, `szabad`, `vak`, `bolond`, `gonosz`...) and whether a phonological (final `g`/`d`/`k`/`l`) or semantic (noun use) predictor holds; needs a lexicon-wide count.
- `NOM-OPEN-5`: complete list of lowering stems with bare accusative (Papp 1975 not accessible).
- `NOM-OPEN-6`: UD Hungarian `Case` value for `-ként` (documentation example under `Abs`) and for `-stUl`, `-képp(en)`, `-ntA` (no documented value); confirm against UD_Hungarian-Szeged treebank data.
- `NOM-OPEN-7`: comparative of `hű` (`hűbb` vs `hívebb`).
- `NOM-OPEN-8`: accusative and plural linking vowel of `-ik` forms (`nagyobbikat` vs `?nagyobbikot`): no corpus evidence for common-noun use.
- `NOM-OPEN-9`: inflection of surnames homonymous with alternating common nouns (`Madár`, `Kő`, `Víz`, `Bokor`) and the scope of the `-ja` preference in names; no source found.
- `NOM-OPEN-10`: quantitative model for `-a`/`-ja` including derived and adjectival stems (Rácz & Rebrus 2012 data not openly available).
- `NOM-OPEN-11`: harmony of case suffixes after `R` stem + `-é` (`kőéhez` vs `?kőéhöz`).
- `NOM-OPEN-12`: `orrt` and `brosst` variants reported by T04 are unattested in the project corpus; decide whether to generate them as dispreferred variants.
- `NOM-OPEN-13`: surface vs interior local case of Hungarian settlement names: KVF98 gives only example lists; the ending-based tendencies in `NOM-PROP-006` need a lexicon-wide count.
- `NOM-OPEN-14`: preferred accusative of `borjú` (`borjút` attested, `borjat` normative doublet, unattested) and possessed plural of `ajtó` (`ajtói` ~ `ajtajai`, equal low attestation).

## Sources used
All keys are in [sources.md](sources.md). Reliability notes for this chapter:
- `Papp69` via `Tab21`, `Papp75` via `T04`: not accessed.
- `UM-hun`: oracle only.

## Terms
See [glossary.md](glossary.md).
