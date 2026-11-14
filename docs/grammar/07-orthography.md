# 07 Orthography (`ORT`)

Scope: the Hungarian spelling norm (helyesírás) as codified by the Academy rulebook, 12th edition (AkH12, 2015), paragraphs 1-238 (letters, spelling principles, stems and suffixes, writing together or apart (egybeírás, különírás), capitalisation (kis és nagy kezdőbetű), proper names, foreign words and transliteration (átírás), hyphenation (elválasztás)) and paragraphs 263-300 where they govern the hyphen (kötőjel), suffixation of digits, symbols, abbreviations (rövidítés) and acronyms (mozaikszó), numerals and dates. Punctuation proper (§239-§262, §266-§269, §271-§274) belongs to `PUN`; letter inventory and digraph segmentation to `WS`; pronunciation-to-suffix mapping (harmony, linking vowels) to `PH` and `NOM`. AkH11 (1984) differences are marked where older texts would be spelled differently.

The chapter encodes decisions as string operations over graphemes (a digraph `cs dz dzs gy ly ny sz ty zs` is one grapheme, see `WS-DIG`) with morpheme boundaries (`+` suffix boundary, `#` compound boundary) supplied by morphology (`DER`, `NOM`, `VRB`). Where the norm decides by meaning, the rule states the exact signal required (lexicon flag, syntactic parse, semantic class).

## Area index
| Area | Topic | AkH12 (primary paragraphs) |
|---|---|---|
| `PRIN` | the four spelling principles, ranking, non-written sound changes, doublets, traditional spellings | §1-§2, §6, §9, §17-§19, §25, §33, §45, §48-§52, §58-§60, §85-§86, §88-§91, §134, §138, §263 b) |
| `LEN` | long and short vowel and consonant letters in stems and suffixes | §4, §6, §9, §20-§24, §26-§28, §31, §34-§40, §46-§47, §59, §62-§74, §87, §206 |
| `JLY` | `j` versus `ly` | §88 |
| `ASSIM` | written assimilations: `-val/-vá`, `az/ez`, verb `j`-suffixes, imperative, doubled digraphs, three identical letters; non-written coalescence | §32, §41-§43, §53-§58, §76-§84, §92-§93 |
| `SUFX` | attaching suffixes to common words: final `a/e` lengthening, suffix doublets | §29-§30, §44, §265 |
| `TOG` | writing together or apart | §94-§138, §141 b-c |
| `63` | the 6:3 syllable rule for multiple compounds | §139-§141 a |
| `HYPH` | hyphen usage | §61, §93, §112-§113, §133, §140, §152, §156, §159-§161, §166-§169, §175-§182, §200, §217, §263-§265, §273, §283, §289, §291, §294 |
| `CAP` | capitalisation | §5, §8, §142-§152, §157, §161, §163-§167, §170-§181, §184-§201, §214, §246, §257-§258, §268, §277 |
| `NAME` | personal, family, animal, object, geographic, astronomical, institution, brand, award names and titles: forms and suffixation | §12, §86, §152-§201 |
| `FOR` | foreign common words and names, transliteration | §13, §202-§222 |
| `ABBR` | abbreviations, symbols, units, acronyms, blends | §275-§289 |
| `NUM` | numerals in digits and letters, ordinals, dates, times | §117, §132, §263 d, i, §265 d, §267, §290-§300 |
| `SPLIT` | hyphenation at line end | §223-§238, §270 |

Conventions used below: `G` = grapheme; `GG` = geminate written form of `G` (for single letters doubled `bb`, for digraphs the simplified doubling `ssz`, `ggy`, `ddzs`); "simple word" (egyszerű szó) = a word that is not a compound, derived and inflected words included; "member" (összetételi tag) = a compound constituent.

## PRIN: spelling principles

### ORT-PRIN-001 Nature of Hungarian writing
- **Statement**: Hungarian spelling is alphabetic, Latin-based and phonemic (letters mostly denote pronounced sounds), and additionally reflects grammar: morphemes are kept recognisable, proper-name types are distinguished, compounds are distinguished from phrases by writing together vs apart, and compounds are divided at line end with regard to member boundaries.
- **Conditions**: all Hungarian text in standard orthography.
- **Output**: design constraint for all `ORT` rules: spelling is a function of (morph sequence, morph spellings, boundary types, proper-name status, lexical flags).
- **Examples**: `kertje` (morph `kert` kept despite [kertye]); `gyorsírás` vs `gyors írás`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-PRIN-002`; `WS` for the letter inventory.
- **Sources**: [AkH12 §1-§2]
- **Validation**: -
- **Confidence**: `normative`

### ORT-PRIN-002 The four principles
- **Statement**: Word forms are spelled by four principles: (1) pronunciation (kiejtés szerinti): each morpheme (stem, derivational suffix, inflectional suffix) is written as it is pronounced in standard speech when uttered alone; (2) morphological or word-analysis (szóelemző): in multi-morpheme forms each morpheme keeps its own spelling even when the whole is pronounced differently; (3) traditional (hagyományos): some spellings follow neither pronunciation nor analysis; (4) simplification (egyszerűsítő): doubled digraphs and three identical consonant letters are simplified.
- **Conditions**: any word form.
- **Output**: see `ORT-PRIN-003` for the operational order.
- **Examples**: pronunciation: `cipő`, `tíz`, `-talan`, `-tól`; analysis: `néztük` [nésztük], `község` [kösség], `szállnak` [szálnak]; tradition: `Kossuth`, `lyuk`, `utca`; simplification: `hosszú` (not *`hoszszú`), `tollal` (not *`tolllal`).
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-PRIN-003`
- **Sources**: [AkH12 §17-§18, §48, §85, §91]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-PRIN-003 Operational order of the principles
- **Statement**: To spell a word form: (a) take each morph's lexical spelling (its pronunciation spelling or its traditional spelling, whichever the lexicon stores); (b) select the allomorph required by phonology and morphology (harmony, linking vowel, lengthened stem `apá-`, imperative stem `fus-`); (c) apply the codified written modifications (`ORT-ASSIM-001` to `ORT-ASSIM-015`, `ORT-ASSIM-019` to `ORT-ASSIM-021`: `v`-assimilation, `az/ez` assimilation, verb `j` assimilation, imperatives); (d) concatenate, leaving every other sound change unwritten (`ORT-PRIN-004` to `ORT-PRIN-007`); (e) apply simplification (`ORT-ASSIM-016`, `ORT-ASSIM-017`) unless blocked (proper names, compound boundaries: `ORT-HYPH-001`, `ORT-NAME-012`); (f) insert hyphens required by `ORT-HYPH-020`.
- **Conditions**: all word forms; the order is derived from the structure of AkH12 (the norm states no ranking explicitly).
- **Output**: spelled form.
- **Examples**: `ház+val`: (c) `v`->`z`: `ház+zal` -> `házzal`; `toll+val`: (c) `toll+lal`, (e) `tollal`; `Széll+val`: (c) `Széll+lel`, (e) blocked -> `Széll-lel`; `fut+j+on`: (b) stem `fus-`, (c) `j`->`s` -> `fusson`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: morph spellings; proper-name flag; compound boundaries
- **Interactions**: all `ORT` areas; `PH-HARM`, `NOM`, `VRB`
- **Sources**: [AkH12 §17, §48, §75, §85, §91-§93]
- **Validation**: goldens
- **Confidence**: `consensus` (ordering inferred from the rule system)

### ORT-PRIN-004 Voicing and nasal place assimilation are not written
- **Statement**: Regressive voicing assimilation between voiceless/voiced pairs (`p-b t-d ty-gy k-g f-v sz-z s-zs c-dz cs-dzs`) and place assimilation of `n` before `p b m` ([m]) and before `gy ty ny` ([ny]) are not written, inside words, across compound boundaries and across words.
- **Conditions**: morph-final and morph-initial consonants in contact.
- **Output**: morphs in their own spelling.
- **Examples**: `ablakban` [ablagban]; `adhat` [athat]; `népdal`; `szívtelen`; `hideg tél`; `szénpor` [szémpor]; `azonban`; `öngyújtó`; `színnyomat`.
- **Exceptions**: `h` devoices but is never voiced; `v` devoices but does not voice (pronunciation facts only).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PH-ASSIM`; reverse direction (pronunciation from spelling) in `PH-G2P`
- **Sources**: [AkH12 §49-§51]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-PRIN-005 Total assimilation and coalescence are not written
- **Statement**: Total assimilations (`t/d/gy + cs` -> [ccs], `t/d + gy` -> [ggy], `t/d/gy + c` -> [cc], `sz/z + s` -> [ss], `sz/z + zs` -> [zzs], `s/zs + sz` -> [ssz], `s/zs + z` -> [zz]) and coalescences (`t d gy + sz` -> [c], `t d gy + s` -> [cs]) are not written; the letters of both morphs stay.
- **Conditions**: morph contact inside words, across compound boundaries, across words.
- **Output**: unchanged morph spellings.
- **Examples**: `átcsap`, `jegycsalás`, `nádcukor`, `község`, `egészség`, `száraz zsömle`; `metsz`, `adsz`, `egyszer`, `költség`, `barátság`, `bolondság`, `nagyság`, `vadság`.
- **Exceptions**: the codified written assimilations `ORT-ASSIM-001` to `ORT-ASSIM-015` and `ORT-ASSIM-019` to `ORT-ASSIM-021` (`-val/-vá`, `az/ez`, verb `j`-suffixes and imperative; AkH12 §52 points to §79-§84).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-ASSIM-018` (`j`-coalescence); `ORT-SPLIT-020` (digraph ambiguity these spellings create)
- **Sources**: [AkH12 §52, §58]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-PRIN-006 Consonant shortening and cluster reduction are not written
- **Statement**: A long consonant next to a different consonant is pronounced short, and the middle member of a three-consonant cluster may be dropped; spelling keeps all letters.
- **Conditions**: long consonant adjacent to a different consonant (inside a word, at a compound boundary, across words); CCC clusters.
- **Output**: full letters.
- **Examples**: `jobbra`, `varrt`, `vidd be`, `otthon`, `visszhang`, `küldd`, `arccal`, `parttalan`, `inggomb`, `hozzánk képest`; `most sincs`, `mindnek`, `csukd be`, `mindnyájan`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-LEN-021`; simplification of identical letters is different (`ORT-ASSIM-017`)
- **Sources**: [AkH12 §59-§60]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-PRIN-007 Closed e and occasional pronunciations are not marked
- **Statement**: Closed [ë] (dialectal) is written `e`; dialectal, archaic or other non-standard forms may be needed in writing for artistic or scholarly purposes (outside the standard).
- **Conditions**: any `e` letter; dialect representation.
- **Output**: `e`; dialect spelling is out of norm (tag as stylistic).
- **Examples**: `ember` [embër], `lehet` [lëhet], `gyerekeket`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `REP` (dialect spellings not to be "repaired" in literary quotes)
- **Sources**: [AkH12 §18, §89]
- **Validation**: -
- **Confidence**: `normative`

### ORT-PRIN-008 Stylistic letter multiplication
- **Statement**: To render unusually long pronunciation, vowel letters may be repeated and consonant letters multiplied.
- **Conditions**: expressive text (interjections, shouting).
- **Output**: allowed non-lexical forms; not errors.
- **Examples**: `Góóól!`, `Neeem.`, `Nnem!`, `Pssszt!`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `REP` must not normalise these by default; tokenisation may map to the lemma (`gól`, `nem`).
- **Sources**: [AkH12 §6, §9]
- **Validation**: -
- **Confidence**: `normative`

### ORT-PRIN-009 Stem doublets both correct
- **Statement**: Words used in two standard forms may be written in either form.
- **Conditions**: word in the doublet list (open; AkH gives examples).
- **Output**: both variants valid (AkH12: "generally" both correct); no preference stated by AkH12 (AkH11 §25 noted `leány`, `reá` as elevated style).
- **Examples**: vowels: `csepeg~csöpög`, `fedél~födél`, `fel~föl`, `seprű~söprű`, `csoda~csuda`, `csorog~csurog`, `dől~dűl`, `hova~hová`, `levő~lévő`, `mienk~miénk`, `tied~tiéd`, `lábas~lábos`, `lány~leány`, `rá~reá`; consonants: `benn~bent`, `fenn~fent`, `kinn~kint`, `lenn~lent`, `gyerek~gyermek`, `foltoz~foldoz`, `bólingat~bólintgat`, `kacsingat~kacsintgat`, `tekinget~tekintget` (AkH12 adds `benn~bent` and the last three pairs; AkH11 §25 also listed `föveny~fövény`, `mért~miért`, dropped in AkH12).
- **Exceptions**: -
- **Predictability**: `lexical` (open list, AkH12 dictionary)
- **Lexical input**: variant links in the lexicon
- **Interactions**: `ORT-SUFX-006` (suffix doublets); corpus: `hova` and `hová` equally frequent (bucket 14/14)
- **Sources**: [AkH12 §25]; [AkH11 §25]; [HUWIKI-AKH12CH]
- **Validation**: attestation
- **Confidence**: `normative`

### ORT-PRIN-010 Obscured morphology is written by pronunciation
- **Statement**: When the derived or compound origin of a word is no longer felt, it is spelled as pronounced, not by its historical morphs.
- **Conditions**: lexicalised opaque words.
- **Output**: pronunciation spelling.
- **Examples**: `kesztyű` (from `kéz`), `lélegzik` (from `lélek`), `aggat`, `faggat`, `szaggat`.
- **Exceptions**: the traditional list `ORT-PRIN-011` keeps etymological spellings instead.
- **Predictability**: `lexical`
- **Lexical input**: lexicon spelling
- **Interactions**: `ORT-PRIN-011`
- **Sources**: [AkH12 §45]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-PRIN-011 Traditional spellings
- **Statement**: A small set of words keeps an etymological or older spelling against pronunciation; archaic family-name letters are also traditional (`ORT-NAME-003`), as is `ly` (`ORT-JLY-001`).
- **Conditions**: listed words and their derivatives.
- **Output**: listed spelling.
- **Examples**: `esd`, `hágcsó`, `hagyján`, `játszik`, `kapzsi`, `metsz`, `mindjárt`, `mindnyájan`, `ósdi`, `pünkösd`, `rögtön`, `szegfű`, `tetszik`, `utca`.
- **Exceptions**: -
- **Predictability**: `lexical` (AkH lists 14 as examples; open)
- **Lexical input**: lexicon spelling
- **Interactions**: `ORT-PRIN-010`
- **Sources**: [AkH12 §85-§86, §88-§90]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-PRIN-012 Codified suffix forms
- **Statement**: Suffixes have one codified written form each (plus harmonic and assimilated allomorphs) despite colloquial variation; only the doublets of `ORT-SUFX-006` are both accepted.
- **Conditions**: all suffixes.
- **Output**: codified form; colloquial variants (e.g. `-bul`, `-rul`, `-tul` for `-ból`, `-ról`, `-tól`) are non-standard.
- **Examples**: `házból` (not `házbul`).
- **Exceptions**: `ORT-SUFX-006`
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-LEN-013`, `ORT-LEN-014`; `REP`
- **Sources**: [AkH12 §33, §44]
- **Validation**: goldens
- **Confidence**: `normative` (the `-bul/-rul/-tul` examples are an illustration not given in AkH12)

### ORT-PRIN-013 Literary deviation
- **Statement**: In literary works stylistic intent may justify writing together, apart or with a hyphen against the general rules, and occasional poetic hyphenated compounds are typical of poetic language.
- **Conditions**: literary genre.
- **Output**: deviating forms tolerated; mark as stylistic, not as errors.
- **Examples**: `bogáncs-szívem`, `köd-gubában`, `rőzse-dalok`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: genre metadata
- **Interactions**: `ORT-TOG`, `ORT-HYPH`
- **Sources**: [AkH12 §138, §263 b)]
- **Validation**: -
- **Confidence**: `normative`

### ORT-PRIN-014 The dictionary as the complement of the rules
- **Statement**: The rule paragraphs do not list all exceptional spellings (writing together/apart, `j/ly`, vowel length, loanword forms); the AkH12 dictionary part decides those, and orthography data must be lexical where the rules defer to it.
- **Conditions**: every rule marked `lexical` in this chapter.
- **Output**: lexicon lookup.
- **Examples**: `bérbeadás`, `dércsípte`, `légiposta`.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: AkH12 dictionary spellings (oracle only; not redistributable)
- **Interactions**: `ORT-TOG-044`, `ORT-JLY-001`
- **Sources**: [AkH12 §88, §134]
- **Validation**: oracle:magyarispell
- **Confidence**: `normative` for together/apart (§134) and `j/ly` (§88); extension to vowel length and loanword forms is inference

## LEN: long and short letters

### ORT-LEN-001 Vowel length is written and distinctive
- **Statement**: Short vowels `a e i o ö u ü` and long vowels `á é í ó ő ú ű` are distinct single letters; length distinguishes words and must never be neutralised in output.
- **Conditions**: every vowel letter.
- **Output**: lexical length preserved; changes only by rule (`ORT-SUFX-001`, `ORT-LEN-006` to `ORT-LEN-009`).
- **Examples**: `akar`/`akár`, `kemény`/`kémény`, `bal`/`bál`, `veréb`/`véreb`, `oda`/`óda`, `irt`/`írt`, `koros`/`kóros`, `öt`/`őt`, `áru`/`árú`, `tüzért`/`tűzért`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: stem spelling
- **Interactions**: `WS` (letters), `REP` (diacritic restoration, `õ`/`û` damage), `ORT-CAP-002` (accents on capitals)
- **Sources**: [AkH12 §4, §6]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-002 Words with long í, ú, ű in all forms
- **Statement**: Many stems keep a long high vowel in every form, including derivatives.
- **Conditions**: stem lexically marked non-alternating.
- **Output**: long vowel throughout.
- **Examples**: `íz`, `ízes`; `szín`, `színész`; `szív`, `szívből`; `hús`, `húsok`; `súly`, `súlyos`; `új`, `újság`, `újra`; `gyűjt`, `gyűjtemény`; `hűs`, `hűsít`; `tű`, `tűk`.
- **Exceptions**: alternating stems `ORT-LEN-006`, `ORT-LEN-007`.
- **Predictability**: `lexical`
- **Lexical input**: stem spelling; alternation class
- **Interactions**: `NOM` shortening stems
- **Sources**: [AkH12 §20-§21]; AkH11 §21 restricts the statement to initial and medial í, ú, ű
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-003 Word-final i is mostly short
- **Statement**: A word-final `i` is short in almost all words, including hypocoristic names; long final `í` occurs in a handful of words.
- **Conditions**: stem-final high front unrounded vowel.
- **Output**: `i`; `í` only in the closed set.
- **Examples**: `kocsi`, `kifli`, `bácsi`, `néni`, `kicsi`, `nini`, `Peti`, `Mari`; long: `sí`, `rí`, `zrí`.
- **Exceptions**: set of long-final words (AkH gives `sí`, `rí`, `zrí` as examples); short monosyllabic function words `ki`, `mi`, `ti` (not in AkH12 §22; inference from the lexicon).
- **Predictability**: `rule` for polysyllables (default short), `lexical` for monosyllables
- **Lexical input**: stem spelling
- **Interactions**: `ORT-LEN-014` (suffix-final `-i` always short)
- **Sources**: [AkH12 §22]; [SMNy3 p. 789]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-004 Final ú, ű in nouns
- **Statement**: Monosyllabic nouns end in long `ú`/`ű`; polysyllabic nouns are split lexically between long and short final `u/ü`; hypocoristic forms end in short `u`.
- **Conditions**: noun ending in a rounded high vowel.
- **Output**: monosyllable: long; polysyllable: lexical; pet name suffix `-u`: short.
- **Examples**: monosyllabic: `bú`, `szú`, `fű`, `mű`, `nyű`, `tű`; long polysyllabic (AkH frequent list, 29 `ú` + 22 `ű` items): `ágyú`, `borjú`, `fiú`, `háború`, `interjú`, `kátyú`, `koszorú`, `vályú`, `varjú`, `betű`, `billentyű`, `fésű`, `gyűrű`, `hegedű`, `kesztyű`, `seprű`, `tetű`; short (30 `u` + 8 `ü` items, incl. `hindu`): `adu`, `alku`, `áru`, `bábu`, `batyu`, `daru`, `falu`, `hamu`, `kapu`, `kenu`, `menü`, `revü`, `eskü`, `ürü`, `zsaru`; pet forms: `anyu`, `apu`, `Icu`, `Pityu`.
- **Exceptions**: nouns in `-tyú/-tyű`, `-attyú/-ettyű` are always long (`ORT-LEN-013`).
- **Predictability**: `rule` (monosyllables, pet forms, `-tyú`), `lexical` (other polysyllables; AkH lists the frequent ones only)
- **Lexical input**: stem spelling
- **Interactions**: `ORT-LEN-005`, `ORT-LEN-009`
- **Sources**: [AkH12 §23]
- **Validation**: goldens; attestation
- **Confidence**: `normative`

### ORT-LEN-005 Final ú, ű in adjectives is long
- **Statement**: Adjectives ending in a rounded high vowel always have long `ú`/`ű`, whether underived or derived with `-ú/-ű/-jú/-jű`.
- **Conditions**: word class adjective; final `u/ü` sound.
- **Output**: `ú`/`ű`.
- **Examples**: `domború`, `homorú`, `hosszú`, `savanyú`, `szomorú`, `gömbölyű`, `gyönyörű`, `keserű`, `könnyű`, `szörnyű`; `négylábú`, `jószívű`.
- **Exceptions**: `hindu` is listed among short-final nouns (§23); its adjectival use with short `u` is a dictionary fact, not stated in §24 (inference: lexical exception).
- **Predictability**: `rule`
- **Lexical input**: part of speech
- **Interactions**: `ORT-LEN-013`, `ORT-LEN-009`
- **Sources**: [AkH12 §24, §34]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-006 Long vowel kept in most forms, shortened in some derivatives
- **Statement**: Some stems with `í ú ű` keep the long vowel in most inflected and derived forms and shorten only in listed derivatives.
- **Conditions**: stem lexically marked; derivative listed.
- **Output**: short vowel in the listed derivatives only.
- **Examples**: `zsír`, `zsíros`, but `zsiradék`; `húsz`, `húszas`, but `huszadik`, `huszad`, `huszon-`; `tíz`, `tízes`, but `tizedik`, `tized`, `tizen-`; `ír`, `írás`, `írat` (verb), but `irat` (noun), `iroda`, `irodalom`; `úszik`, `ússzon`, but `uszály`, `uszoda`, `uszony`; `bújik`, `bújócska`, but `bujkál`, `bujdosó`; `hűl`, but `hüledezik`; `tűr`, but `türelem`, `türtőztet`.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: derivative spellings
- **Interactions**: `DER`; `PND-CARD-002` (`huszon-`, `tizen-`)
- **Sources**: [AkH12 §27 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-007 Alternating í-i, ú-u, ű-ü nouns follow the kéz pattern
- **Statement**: Nouns `híd`, `ín`, `nyíl`, `víz`, `kút`, `lúd`, `nyúl`, `rúd`, `úr`, `út`, `fűz`, `szűz`, `tűz` generally shorten before the same suffixes as `kéz` (-> `kez-`); AkH12 illustrates shortening before accusative, plural, `-s` and 1pl possessive and length before `-ben`, `-re` and `-i`; the full suffix set follows the `kéz` stem class.
- **Conditions**: stem in the shortening class (closed, `NOM` stem class).
- **Output**: `vizet`, `vizek`, `vizes`, `vizünk`; `vízben`, `vízre`, `vízi`.
- **Examples**: `út` -> `utat`, `utak`, `utas`, `utunk`, `útban`, `útra`, `úti`; `tűz` -> `tüzet`, `tüzek`, `tüzes`, `tüzünk`, `tűzben`, `tűzi`.
- **Exceptions**: 3rd-person possessive of `híd`, `kút`, `lúd`, `rúd`, `út` takes `-ja/-juk` and keeps the long vowel: `hídja`, `kútja`, `lúdja`, `rúdja`, `útja`, `útjuk`.
- **Predictability**: `lexical` (class membership), `rule` (suffix conditions shared with `kéz`)
- **Lexical input**: stem property `shortening`
- **Interactions**: `NOM` shortening stems; `ORT-LEN-006`
- **Sources**: [AkH12 §27 b)]; possessive paradigm `NOM-STEM`
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-008 Long stem vowel shortens before learned foreign suffixes
- **Statement**: In a part of common loanwords a long stem-internal vowel alternates with short before foreign-origin suffixes (AkH12 names `-ikus`, `-ista`, `-itás`, `-izál`, `-izmus` etc.; examples also show `-isztika`, `-izáció`), also in further derivatives; some words keep the long vowel (not always consistently).
- **Conditions**: loan stem with long vowel + learned suffix.
- **Output**: short vowel (default), long in the lexical exception set.
- **Examples**: `szláv` -> `szlavista`; `szociális` -> `szocialista`; `hérosz` -> `heroikus`; `passzív` -> `passzivitás`; `hipnózis` -> `hipnotikus`; `szuverén` -> `szuverenitás`; `irónia` -> `ironikus`; `típus` -> `tipikus`; `periódus` -> `periodikus`; `reális` -> `realista`; `kolléga` -> `kollegiális` -> `kollegialitás`; keep long: `akadémia` -> `akadémikus`, `akadémizmus`, `akadémista`; `fókusz` -> `fókuszál` (AkH12 addition); `esztéta` -> `esztétika`, `esztétikus`.
- **Exceptions**: listed keep-long derivatives (lexical, not consistent per AkH12).
- **Predictability**: `lexical` (AkH11 §206: "overwhelming majority" shorten; AkH12 weakened to "a significant part" (§28) and "a part" (§206), and "alternates with short" (§206) instead of "changes to short")
- **Lexical input**: derivative spellings
- **Interactions**: `DER`; `ORT-FOR-006`; AkH12 dictionary changed several (`sámánizmus`, `sátánizmus`, `sarlatánizmus` now long: `ORT-FOR-006`)
- **Sources**: [AkH12 §28, §206]; [AkH11 §28, §206]; [HUWIKI-AKH12CH]
- **Validation**: oracle:magyarispell
- **Confidence**: `normative`

### ORT-LEN-009 Final ú/ű shortens before the verbal suffix -l
- **Statement**: In the closed set of verbs derived with `-l` from `ú/ű`-final stems the vowel is short.
- **Conditions**: stems `ború`, `bosszú`, `csepű`, `derű`, `domború`, `fésű`, `hegedű`, `keserű`, `köpű`, `köszörű`, `lassú`, `szörnyű` + `-l`.
- **Output**: `borul`, `(meg)bosszul`, `csepül`, `derül`, `domborul`, `fésül`, `hegedül`, `(meg)keserül`, `köpül`, `köszörül`, `lassul`, `szörnyülködik`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (12 stems listed)
- **Lexical input**: derivative spellings
- **Interactions**: `DER`
- **Sources**: [AkH12 §31]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-010 dz and dzs are not doubled in stems
- **Statement**: `dz` and `dzs` are written single even when pronounced long inside stems; length arising from suffix assimilation is written (`ddz`, `ddzs`).
- **Conditions**: stem-internal or stem-final `dz`/`dzs`.
- **Output**: single in stems; doubled (simplified) at assimilating suffixes.
- **Examples**: `bodza`, `madzag`, `edz`, `edző`, `pedz`, `pedzi`, `maharadzsa`, `bridzs`; assimilated: `eddzük`, `peddze`, `briddzsel` ('with bridge').
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-ASSIM-001`, `ORT-ASSIM-011`, `ORT-ASSIM-012`, `ORT-SPLIT-011`
- **Sources**: [AkH12 §87]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-011 benn/bent word family
- **Statement**: `benn`, `fenn~fönn`, `kinn`, `lenn` and compounds `bennfentes`, `bennszülött`, `fenntart~fönntart` have `nn`; `benső`, `fenséges`, `bent`, `fent~fönt`, `kint`, `lent`, `bentlakó`, `kintlévőség` have single `n`.
- **Conditions**: members of this family.
- **Output**: as listed; `-t` forms single `n`.
- **Examples**: as in Statement.
- **Exceptions**: -
- **Predictability**: `lexical` (heuristic, not in AkH12: `nn` unless `t` or `s/ség` follows)
- **Lexical input**: spelling
- **Interactions**: `ORT-PRIN-009`
- **Sources**: [AkH12 §46]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-012 Comparative of be, fel, ki, le
- **Statement**: The comparatives of `be`, `fel/föl`, `ki`, `le` are written partly by pronunciation.
- **Conditions**: these four adverbs + `-bb`.
- **Output**: `beljebb`, `feljebb~följebb`, `kijjebb`, `lejjebb`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (4)
- **Lexical input**: spelling
- **Interactions**: -
- **Sources**: [AkH12 §47]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-013 Always-long suffix vowels
- **Statement**: The vowel is always long in: verbalising `-ít`, `-dít`, `-sít`; adjectival `-ú`, `-ű`, `-jú`, `-jű`; noun endings `-tyú`, `-tyű`, `-attyú`, `-ettyű`; elative/delative/ablative `-ból/-ből`, `-ról/-ről`, `-tól/-től`; the `-ól/-ől` of whence-postpositions.
- **Conditions**: these morphemes in any position.
- **Output**: long vowel letter.
- **Examples**: `épít`, `tanító`, `buzdít`, `állandósít`; `egyenáramú`, `bőkezű`, `erős rugójú`, `lapos tetejű`; `fogantyú`, `billentyű`, `dugattyú`, `csengettyű`; `szobából`, `erdőből`, `házról`, `fától`; `alól`, `elől`, `felől`, `mellől`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: morpheme identity
- **Interactions**: `ORT-LEN-014`; `ORT-LEN-017` (`-tyú` consonant)
- **Sources**: [AkH12 §34]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-014 Always-short suffix vowels
- **Statement**: The vowel is always short in: verbalising `-ul/-ül`, `-dul/-dül`; essive-modal `-ul/-ül` and comitative `-stul/-stül`; the `-ul/-ül` of postpositions and adverbs; multiplicative `-szor/-szer/-ször`; temporal `-kor`; terminative `-ig`; any word-final `i` of a suffixed form.
- **Conditions**: these morphemes; `i` as the last letter of a suffixed word.
- **Output**: short vowel letter.
- **Examples**: `gyógyul`, `kerület`, `fordul`, `pezsdül`; `magyarul`, `végül`, `családostul`, `csőstül`; `alul`, `hátul`, `belül`, `felül~fölül`, `keresztül`, `közül`, `mögül`, `nélkül`; `háromszor`, `hússzor`, `hétszer`, `ötször`; `éjfélkor`, `hatkor`; `félig`, `keddig`; `esti`, `kecskeméti`, `házai`, `kertjei`, `kérdi`, `nézi`, `írni`, `öklömnyi`.
- **Exceptions**: `túl` (long).
- **Predictability**: `rule`
- **Lexical input**: morpheme identity
- **Interactions**: `ORT-LEN-013`, `ORT-LEN-003`
- **Sources**: [AkH12 §35]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-015 Accusative -t is single
- **Statement**: The accusative suffix is always one `t`, after vowels and consonants alike.
- **Conditions**: accusative.
- **Output**: `-t` / `-Vt`.
- **Examples**: `hajót`, `könyvet`, `sast`, `sárgát`, `nyolcat`, `valakit`.
- **Exceptions**: - (after `t`/`tt`-final stems a linking vowel intervenes: `hatot`, `vádlottat`, so no letter clash arises).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `NOM-ACC`
- **Sources**: [AkH12 §36]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-016 -képp and -képpen
- **Statement**: The adverbial suffix `-képp`, `-képpen` has `pp`.
- **Conditions**: this suffix.
- **Output**: `pp`.
- **Examples**: `ekképp`, `másképp`, `tulajdonképpen`, `voltaképpen`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-SUFX-002` (non-lengthening)
- **Sources**: [AkH12 §37]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-017 Suffix consonant doubled after vowels, single after consonants
- **Statement**: Several suffixes have a doubled consonant after a vowel (including a linking vowel) and a single one after a consonant: past tense `-t/-tt`; past participle `-t/-tt`; locative `-t/-tt`; the `-t/-tt` ending postpositions; comparative `-b/-bb`; imperative definite 2sg short form `-d/-dd`; the 'seem, consider' suffix `-l/-ll`; endings `-tyú/-tyű` vs `-attyú/-ettyű`.
- **Conditions**: segment immediately before the suffix consonant: vowel -> double; consonant -> single.
- **Output**: see Examples.
- **Examples**: past: `folyt`, `kente`, `varrtuk` vs `csengetett`, `dobott`, `hallottuk`, `megrótták`; participle: `bérelt`, `felforrt`, `kitalált` vs `elvégzett`, `kitűzött`, `mosott`; locative: `Vásárhelyt` vs `Győrött`, `Pécsett`, `Vácott`, `Kolozsvárt~Kolozsvárott`; postpositions: `iránt`, `közt`, `szerint` vs `alatt`, `fölött`, `között`, `mögött`; comparative: `helyesbít`, `idősb`, `kevésbé` vs `helyesebb`, `idősebb`, `jobb`, `szebb`; imperative: `írd`, `nézd` vs `edd`, `idd`, `sződd`; long form always single: `írjad`, `szőjed`; `-l/-ll`: `fehérl(ik)`, `pirosl(ik)` vs `barnálló`, `nagyollom`, `zöldellő`; `kéklett~kékellett`, `keveslem~kevesellem`; but `helyesel` (single `l` after vowel, different suffix); `pattantyú`, `billentyű` vs `dugattyú`, `csengettyű`.
- **Exceptions**: postpositions `át`, `hosszat` (single `t` after vowel).
- **Predictability**: `rule`
- **Lexical input**: whether a linking vowel is present (from `VRB`/`NOM`)
- **Interactions**: `VRB-PAST`, `NOM-CASE-016`, `ORT-LEN-024`, `ORT-LEN-025` (`rótt`), `ORT-SUFX-006`
- **Sources**: [AkH12 §38]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-018 Privative -atlan/-etlen after -ít; -etlen in listed words
- **Statement**: The privative derivative of `-ít` verbs uses `-atlan/-etlen` (one `t`); `-etlen` with single `t` also in `befejezetlen`, `éretlen`, `fedetlen`, `fegyelmezetlen`, `képzetlen`, `sértetlen` and similar.
- **Conditions**: privative suffix after `-ít` or after the listed stems.
- **Output**: single `t`.
- **Examples**: `csorbítatlan`, `hamisítatlan`, `javítatlan`; `sértetlen`.
- **Exceptions**: -
- **Predictability**: `rule` (after `-ít`), `lexical` (other `-etlen` words)
- **Lexical input**: derivative spelling
- **Interactions**: `DER`
- **Sources**: [AkH12 §39]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-019 Causative after t-final verbs: -at/-et vs -tat/-tet
- **Statement**: After a `t`-final verb, the causative is `-at/-et` if the `t` is preceded by a consonant, `-tat/-tet` if preceded by a vowel; same for archaic passive `-atik/-etik` vs `-tatik/-tetik`.
- **Conditions**: verb stem ends in `t`; causative or archaic passive suffix.
- **Output**: C + `t` -> `-at/-et`; V + `t` -> `-tat/-tet` (spelled `tt` + `at`).
- **Examples**: `bontat`, `festet`, `gyűjtet`, `irtat`, `sejteti`; `arattat`, `köttet`, `nyittat`, `süttet`, `taníttatja`; `oltatik`, `gyűjtetik`, `kinyittatik`, `kihirdettetik`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-CAUS`, `PH-HARM`
- **Sources**: [AkH12 §40]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-020 d-final verbs: imperative definite 2sg always dd
- **Statement**: The short imperative definite 2sg of `d`-final verbs is written with `dd` both after vowels (long [d]) and after consonants (short [d]).
- **Conditions**: stem-final `d` + `-d`.
- **Output**: stem + `d`.
- **Examples**: `add`, `fedd (el)`, `szedd`, `tudd`, `védd`; `áldd`, `hordd`, `kezdd`, `küldd`, `mondd`, `oldd`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-PRIN-006`, `VRB-IMP`; three-letter case for the verb `fedd` 'scold': `fedd+d meg` -> `fedd meg` (`ORT-ASSIM-017`, AkH12 §93); `fedd (el)` here is `fed`+`d`
- **Sources**: [AkH12 §62]
- **Validation**: goldens; attestation (`mondd` bucket 13)
- **Confidence**: `normative`

### ORT-LEN-021 Stem-final long consonant letters are kept before consonant-initial suffixes
- **Statement**: A stem's doubled consonant letter is kept before any suffix beginning with a different consonant, regardless of shortened pronunciation; likewise a doubled suffix-initial letter after a different stem-final consonant.
- **Conditions**: stem ends in `GG`; suffix starts with a consonant other than `G`.
- **Output**: `GG` + suffix.
- **Examples**: `jobbra`, `varrt`, `hallgat`, `könnytől`, `meggyből`, `rosszkor`, `tollban`, `Kissnek`; `arccal`, `parttalan`.
- **Exceptions**: same-consonant suffix: `ORT-ASSIM-017`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-PRIN-006`, `ORT-SPLIT-005`
- **Sources**: [AkH12 §59, §226 d)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-022 l-final verbs keep single l
- **Statement**: Verbs ending in single `l` keep one `l` in all suffixed forms; a few derivative verbs have `ll`.
- **Conditions**: stem-final `l`.
- **Output**: single `l`; `ll` in listed derivatives.
- **Examples**: `szól`, `szólás`; `múlik`, `múló`; `legel`, `legelő`; exceptions: `dűl` -> `dülled`, `dülleszt`; `fúl` -> `fullad`, `fullaszt`; `gyúl` -> `gyullad`, `gyullaszt`.
- **Exceptions**: as listed (lexical, "etc.").
- **Predictability**: `rule` with `lexical` exceptions
- **Lexical input**: derivative spelling
- **Interactions**: `ORT-LEN-023`
- **Sources**: [AkH12 §63]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-023 ll-final verbs keep ll
- **Statement**: Verbs ending in `ll` keep `ll` in almost all suffixed forms regardless of pronunciation; a few derivatives have single `l`; some pairs distinguish meaning by `ll` vs `l`.
- **Conditions**: stem-final `ll`.
- **Output**: `ll`; listed derivatives `l`.
- **Examples**: `áll`, `állomás`, `állt`; `mállik`, `málljanak`; `száll`, `szálloda`; `rosszall`, `rosszallhatjuk`; `zöldell(ik)`, `zöldellt~zöldellett`; `szégyell`, `szégyellni`; single: `torkoll(ik)` -> `torkolat`; `sarkall` -> `sarkalatos`; meaning pairs: `torkollik` 'flow into' / `letorkol` 'silence'; `sarkall` 'spur' / `sarkal` 'heel (a shoe)'; `nagyoll` 'think too big' / `nagyol` 'rough out'.
- **Exceptions**: as listed (lexical).
- **Predictability**: `rule` with `lexical` exceptions; meaning pairs need a sense tag
- **Lexical input**: derivative spelling; sense
- **Interactions**: `ORT-LEN-024`
- **Sources**: [AkH12 §64]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-024 ll/l doublets of some ll verbs
- **Statement**: Some `ll` verbs have a longer post-vowel `ll` form and a shorter post-consonant `l` form in certain inflections; both are correct.
- **Conditions**: `-ll` 'seem, consider' verbs (`kevesell`, `kicsinyell`, `kékell` etc.).
- **Output**: both variants.
- **Examples**: `kevesellem~keveslem`, `kicsinyelli~kicsinyli`, `kékell(ik)~kékl(ik)`.
- **Exceptions**: -
- **Predictability**: `rule` (both valid)
- **Lexical input**: membership in the `-ll` derivative class
- **Interactions**: `ORT-LEN-017`, `ORT-SUFX-006`
- **Sources**: [AkH12 §65, §38, §44]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-025 Verbs ending in a long vowel
- **Statement**: For `fő`, `lő`, `nő`, `nyű`, `rí`, `ró`, `sző`: `-sz`, `-tok/-tök` and `-j`/`j`-initial suffixes are written single despite variable pronunciation; past tense and past participle have `tt`; imperative definite 2sg short form has `dd`; in the `v`-extended stem the vowel is usually short.
- **Conditions**: these seven verbs (closed).
- **Output**: `lősz`, `lőtök`, `lője`, `lőjük`, `nőj`, `rósz`, `rótok`, `rója`, `rójuk`; `rótt`, `nőtt`, `nőttek`; `ródd`, `lődd`, `sződd`, `nyűdd`; `fövetlen`, `lövő`, `növendék`, `rovok`.
- **Examples**: as Output; *`lőjjön`, *`nőttök` for `lőjön`, `nőtök`.
- **Exceptions**: -
- **Predictability**: `lexical` (7 verbs), `rule` within
- **Lexical input**: v-stem verb class
- **Interactions**: `VRB` v-stems, `ORT-LEN-017`
- **Sources**: [AkH12 §66]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-026 jön
- **Statement**: Suffixed forms of `jön` have short `ö` (except archaic `jő`, `jődögél`, `jőve`); consonant-initial suffixes on the `jö-` stem are written doubled.
- **Conditions**: verb `jön`.
- **Output**: `jövök`, `jöhet`, `jövedelem`; `jössz`, `jöttél`, `jöjj`, `jöjjön`, `jöjjenek`.
- **Examples**: as Output.
- **Exceptions**: `jő`, `jődögél`, `jőve`.
- **Predictability**: `lexical`
- **Lexical input**: irregular paradigm
- **Interactions**: `VRB` irregular; `ORT-ASSIM-021`
- **Sources**: [AkH12 §67, §42]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-027 lesz: single sz
- **Statement**: All forms of `lesz` are written with one `sz`, including the bare form.
- **Conditions**: verb `lesz`.
- **Output**: `lesz`, `leszek`, `leszel`, `leszünk`, `lesztek`, `lesznek`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: none
- **Interactions**: `ORT-ASSIM-019`
- **Sources**: [AkH12 §68]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-028 egy: single gy except before -val/-vá
- **Statement**: `egy` and its forms and derivatives are written with one `gy` whatever the pronounced length; `gg` appears only in the `-val/-vá` assimilated forms.
- **Conditions**: stem `egy`.
- **Output**: `egy`, `egyet`, `egyik`, `egyes`, `együtt`, `egyed`, `egyetem`, `egység`, `egyre`; `eggyel`, `eggyé`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-ASSIM-001`
- **Sources**: [AkH12 §69]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-029 kis: single s in the comparative family
- **Statement**: Graded forms of `kis` and their derivatives have one `s` despite long [s].
- **Conditions**: `kis` + `-ebb` and derivatives.
- **Output**: `kisebb`, `legkisebbnek`, `kisebbedik`, `kisebbít`, `kisebbség`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [AkH12 §70]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-030 s-final adjectives: single s
- **Statement**: Accusative, `-n` adverbial, plural and comparative forms of `s`-final adjectives are written with one `s` even if pronounced long.
- **Conditions**: adjective ending in `s`.
- **Output**: `erőset`, `magasat`, `erősen`, `magasan`, `erősek`, `erősebb`, `magasabb`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [AkH12 §71]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-031 Personal-suffixed case and postposition forms of pronouns
- **Statement**: Case-suffixed and postpositional forms of personal pronouns (`belőlem`, `előled`, `nálad`, `tőlünk`, `utánunk`) have single `l`/`n`; but the inessive forms `benne`, `bennünk` have `nn` (extension to the whole `benn-` paradigm is inference).
- **Conditions**: personal-suffixed case stems and postpositions.
- **Output**: as listed.
- **Examples**: `belőlem`, `nálad`, `tőlünk`, `utánunk`; `benne`, `bennünk`, `bennem`.
- **Exceptions**: -
- **Predictability**: `lexical` (closed paradigm set)
- **Lexical input**: paradigm
- **Interactions**: `PND-PERS`
- **Sources**: [AkH12 §72]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-032 új
- **Statement**: `új` is written with long `ú` and one `j` in all forms despite frequent [ujj] pronunciation.
- **Conditions**: stem `új`.
- **Output**: `új`, `újat`, `újabb`, `újra`.
- **Examples**: as Output.
- **Exceptions**: - (the unrelated `ujj` 'finger' with `u` + `jj` is a different word; not in AkH12 §73)
- **Predictability**: `lexical`
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [AkH12 §73]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-033 h-final words keep h
- **Statement**: In `céh`, `düh`, `juh`, `méh`, `rüh` (variable pronunciation of `h`) the `h` is written in all forms.
- **Conditions**: these stems (AkH says "characteristically"; open).
- **Output**: `h` retained: `dühnek`, `méhtől`.
- **Examples**: `düh`, `dühnek`, `méh`, `méhtől`.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: stem property `variable_h`
- **Interactions**: `ORT-ASSIM-005`
- **Sources**: [AkH12 §74]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-LEN-034 Consonant length is written and distinctive
- **Statement**: Short and long consonants are distinguished in writing (single vs doubled letter, simplified doubling for digraphs); consonant length distinguishes words and must never be neutralised in output.
- **Conditions**: every consonant letter.
- **Output**: lexical consonant length preserved; changes only by rule (`ORT-LEN-017`, `ORT-ASSIM`).
- **Examples**: `megy`/`meggy`, `orom`/`orrom`, `feje`/`fejje`, `eset`/`esett`, `hal`/`hall`, `hason`/`hasson`, `len`/`lenn`, `kasza`/`kassza`.
- **Exceptions**: `dz`, `dzs` length unmarked in stems (`ORT-LEN-010`); stylistic multiplication (`ORT-PRIN-008`).
- **Predictability**: `rule`
- **Lexical input**: stem spelling
- **Interactions**: `ORT-LEN-001`, `ORT-ASSIM-016`, `WS-DIG`
- **Sources**: [AkH12 §9]
- **Validation**: goldens
- **Confidence**: `normative`

## JLY: j versus ly

### ORT-JLY-001 ly is a traditional letter for [j]; no complete rule exists
- **Statement**: Standard pronunciation has no separate sound for `ly`; it is a traditional spelling of [j]. Which words have `ly` is lexical; the norm gives only guiding tendencies (`ORT-JLY-002` to `ORT-JLY-005`) and defers to the dictionary.
- **Conditions**: every [j] in a stem.
- **Output**: lexicon spelling; suffix-initial and suffix-internal [j] is always `j` (`-ja`, `-juk`, `-j`, `-jú`); stem `ly` is kept unchanged under suffixation (`hely` -> `helyes`, `helyi`, `helyet`).
- **Examples**: `hely`, `helyen`; `folyik`, `folyjon` (stem `ly` + imperative `j`).
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: stem spelling
- **Interactions**: `PH-G2P` (`ly` = [j]); `ORT-ASSIM-001` (`lly` in `-val`); `ORT-SPLIT-003` (`ly` one grapheme); `REP` (j/ly confusion)
- **Sources**: [AkH12 §88]
- **Validation**: oracle:magyarispell
- **Confidence**: `normative` (statement); the suffix-`j` and stem-`ly` retention clauses of Output are inferred from AkH12 examples, not stated in §88

### ORT-JLY-002 Word-initial ly only in lyuk
- **Statement**: Word-initially `ly` occurs only in `lyuk` and its suffixed and derived forms.
- **Conditions**: [j] at the start of a word (a compound member starting with `lyuk` counts: `kulcslyuk`).
- **Output**: `j` everywhere else initially.
- **Examples**: `lyuk`, `lyukban`, `lyukak`, `lyuggat`, `lyukad`, `lyukaszt`, `lyukas`; `kulcslyuk`, `gomblyuk`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [AkH12 §88 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-JLY-003 Word-final [j] in monosyllables and pronouns
- **Statement**: The norm distinguishes monosyllables from polysyllables for word-final [j] and lists monosyllables written with `ly` by example only; in pronouns [j] is written `ly`.
- **Conditions**: monosyllabic stem ending in [j]; pronoun stems.
- **Output**: `ly` in `boly`, `foly(ik)`, `hely` (noun), `mély`, `moly`, `súly`, `süly`, `gally` (AkH12 "for example" list, not closed); pronouns: `ily`, `ilyen`, `mely`, `amely`, `amelyik`, `mily`, `milyen`, `oly`, `olyan`; derivatives (`ilyesmi`, `olyankor`) keep the pronoun spelling; other monosyllables from the lexicon, typically `j` (`baj`, `díj`, `fej`, `háj`, `máj`, `száj`, `tej`, `vaj`).
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (open example list for monosyllables; the `j` default for other monosyllables is an inference, AkH12 states none), `rule` for pronouns
- **Lexical input**: stem spelling
- **Interactions**: -
- **Sources**: [AkH12 §88 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-JLY-004 Word-final [j] in polysyllables
- **Statement**: Most polysyllabic words ending in [j] are spelled with `ly`, but among those ending in [aj] or [ej] `j` is the majority.
- **Conditions**: polysyllabic stem ending in [j].
- **Output**: default `ly`; after `a`/`e` default `j`; lexical exceptions both ways.
- **Examples**: `ly`: `akadály`, `bagoly`, `csekély`, `fogoly`, `harkály`, `kevély`, `kristály`, `mosoly`, `osztály`, `ragály`, `tartály`, `Gergely`, `Ipoly`, `Mihály`; exceptions with `j`: `lakáj`, `muszáj`, `papagáj`; `-aj/-ej` with `j`: `csevej`, `dörej`, `duhaj`, `karaj~karéj`, `óhaj`, `olaj`, `robaj`, `sóhaj`, `talaj`, `tolvaj`, `zörej`, `zsivaj`; exceptions with `ly`: `karvaly`, `pehely`, `tavaly`, `zsindely`.
- **Exceptions**: as listed ("for example" lists; lexicon decides).
- **Predictability**: `tendency` (no counts in the norm; use as fallback for unknown words only)
- **Lexical input**: stem spelling
- **Interactions**: `ORT-JLY-001`
- **Sources**: [AkH12 §88 b)]
- **Validation**: oracle:magyarispell (measure the heuristic on the lexicon before use)
- **Confidence**: `normative` (examples), `tendency` (heuristic)

### ORT-JLY-005 -lya, -lye endings
- **Statement**: Words ending in [ja]/[je] form a characteristic `-lya/-lye` group; a few are spelled `-ja/-je`.
- **Conditions**: stem ending [jA] (not a possessive suffix).
- **Output**: `ly` by default; `j` in exceptions.
- **Examples**: `boglya`, `csigolya`, `gereblye`, `gólya`, `hülye`, `ibolya`, `korcsolya`, `nyavalya`, `pocsolya`, `pólya`, `skatulya`, `zsöllye`; exceptions: `bója`, `cserje`, `héja` (bird), `perje`.
- **Exceptions**: as listed ("for example" list; lexicon decides).
- **Predictability**: `tendency`
- **Lexical input**: stem spelling; morphological analysis must first exclude the possessive `-ja/-je` (`fája`, `kertje` always `j`)
- **Interactions**: `NOM-POSS`
- **Sources**: [AkH12 §88 c)]
- **Validation**: oracle:magyarispell
- **Confidence**: `normative`

### ORT-JLY-006 ly in suffixation and old names
- **Statement**: `ly` behaves as the consonant [j] for all suffix choices (3sg possessive `-a/-e`, linking vowels) and as one grapheme for doubling: `-val/-vá` gives `lly`; `ly` + `ly`-initial suffix does not occur. In archaic family names final `y` after a vowel is [i], not part of `ly` (`Kölcsey`), while `ly` after a vowel in names is [j] (`Kodály`, `Illyés`).
- **Conditions**: `ly`-final stems; names.
- **Output**: `hely` -> `helye`, `hellyel`; `Kodály` -> `Kodállyal`; `gally` -> `gallyal` (three-grapheme simplification).
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`; names need lexical `y` = [i] vs `ly` decision (`ORT-NAME-004`)
- **Lexical input**: name pronunciation
- **Interactions**: `ORT-ASSIM-001`, `ORT-ASSIM-002`, `ORT-NAME-004`
- **Sources**: [AkH12 §82 a)-b), §88, §92, §12]
- **Validation**: goldens
- **Confidence**: `normative`

## ASSIM: written assimilations and simplifications

### ORT-ASSIM-001 -val/-vel and -vá/-vé: v assimilates to a final consonant
- **Statement**: After a vowel-final stem the suffixes are `-val/-vel`, `-vá/-vé`; after a consonant-final stem the `v` becomes identical to the stem's final consonant and the resulting long consonant is written by doubling: a single-letter consonant `X` gives `XX`, a digraph gives the simplified doubling (first letter doubled).
- **Conditions**: instrumental (`-val/-vel`) or translative (`-vá/-vé`); stem ends in exactly one short consonant grapheme `G` (preceded by a vowel).
- **Output**: stem + `G` + `al/el` or `á/é`, with digraph doubling `cs`->`ccs`, `dz`->`ddz`, `dzs`->`ddzs`, `gy`->`ggy`, `ly`->`lly`, `ny`->`nny`, `sz`->`ssz`, `ty`->`tty`, `zs`->`zzs`; harmony by `PH-HARM`.
- **Examples**: `babával`, `kővé`; `bottal`, `cukorral`, `kékkel`, `dobbal`, `emberré`, `jósággal`, `szénnel`; `arannyal`, `rozzsal`, `néggyel`, `királlyá`, `özveggyé`, `tudóssá`, `vízzé`; `bridzs` -> `briddzsel`.
- **Exceptions**: long-consonant stems `ORT-ASSIM-002`; clusters `ORT-ASSIM-003`; `v`-final `ORT-ASSIM-004`; variable `h` `ORT-ASSIM-005`; hyphenated attachments `ORT-ASSIM-007`; personal names with archaic letters `ORT-NAME-011`; foreign letters `ORT-FOR-011`.
- **Predictability**: `rule`
- **Lexical input**: none (final grapheme from segmentation)
- **Interactions**: `NOM-CASE-003`, `NOM-CASE-004`, `ORT-ASSIM-016`, `ORT-SPLIT-006`
- **Sources**: [AkH12 §41, §82 a), §7 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-002 -val/-vá after a long consonant: two letters, not three
- **Statement**: After a stem ending in a written long consonant `GG`, the assimilated `v` would produce three identical letters; only the doubled form is written.
- **Conditions**: common word ending in `GG` (e.g. `ll`, `bb`, `tt`, `lly`, `nny`, `ggy`, `ssz`); instrumental/translative.
- **Output**: stem + `al/el` or `á/é` (no additional consonant letter).
- **Examples**: `jobbal`, `tollal`, `tettel`, `jobbá`, `tollá`, `tetté`, `gallyal`, `könnyel`, `meggyel`, `könnyé`, `rosszá`, `füttyel`.
- **Exceptions**: proper names and title words: hyphen and full letters (`ORT-NAME-012`: `Kiss-sel`, `Papp-pal`); blends ending in `GG`: `Keravill-lal` (`ORT-ABBR-016`).
- **Predictability**: `rule`
- **Lexical input**: proper-name flag
- **Interactions**: `ORT-ASSIM-017`
- **Sources**: [AkH12 §82 b), §93]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-003 -val/-vá after a consonant cluster
- **Statement**: After a stem ending in two (or more) different consonant graphemes, the `v` assimilates to the last one, which is doubled (digraphs simplified).
- **Conditions**: stem ends in `C1 C2`, `C1 != C2`, `C2` not `v`.
- **Output**: stem + `C2` + `al/el` / `á/é`.
- **Examples**: `hanggal`, `komppal`, `minddel`, `gyönggyel`, `kulccsal`, `szárnnyal`, `füstté`, `bolonddá`, `szilánkká`, `bölccsé`, `ronggyá`, `szárnnyá`; `bors` -> `borssal`; `karszt` -> `karszttá`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-PRIN-006` (pronounced short, written double)
- **Sources**: [AkH12 §82 c)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-004 -val/-vá after v: vv always written
- **Statement**: When the stem ends in `v` (after a vowel or a consonant), the suffix `v` is written: `vv`.
- **Conditions**: stem-final `v`.
- **Output**: stem + `val/vel/vá/vé`.
- **Examples**: `hévvel`, `szívvel`, `kedvvel`, `tervvel`, `névvé`, `savvá`, `könyvvé`, `nedvvé`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-ASSIM-006`
- **Sources**: [AkH12 §82 d)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-005 -val/-vá after h-final words
- **Statement**: `h`-final words whose `h` is silent count as vowel-final (`v` kept); words with a pronounced `h` assimilate (`hh`); words with variable `h` (`céh`, `düh`, `juh`, `méh`, `rüh`) allow both.
- **Conditions**: stem-final `h`; lexical class `silent_h` / `pronounced_h` / `variable_h`.
- **Output**: silent: `csehvel`, `pléhvé`; pronounced: `Allahhal`, `dohhal`; variable: `dühvel~dühhel`, `méhvé~méhhé` (AkH12 examples); `céhvel~céhhel`, `juhval~juhhal`, `rühvel~rühhel` (by §74 membership, not exemplified in §82 e)).
- **Examples**: as Output.
- **Exceptions**: foreign `ch` = [h]: `pechhel` (`ORT-FOR-011`).
- **Predictability**: `lexical` (class of `h`)
- **Lexical input**: stem property `h_class`
- **Interactions**: `ORT-LEN-033`; AkH11 grouped `céh`, `cseh`, `düh`, `juh`, `méh`, `pléh`, `rüh` as silent-`h` words before consonant-initial suffixes, giving only the `v` form (`csehvel`, `méhvel`, `juhval`); the `hh` variants of the variable class are AkH12 additions. Corpus: `dühvel` bucket 4, `dühhel` unattested.
- **Sources**: [AkH12 §41, §74, §82 e)]; [AkH11 §75]; [HUWIKI-AKH12CH]
- **Validation**: goldens; attestation
- **Confidence**: `normative`

### ORT-ASSIM-006 Doubling distinguishes word forms
- **Statement**: Omitting the assimilated doubling can yield a different word; the doubled form must be generated for the case suffix and never "corrected" away.
- **Conditions**: homographic pairs.
- **Output**: instrumental/translative with doubling vs verb or possessive form without.
- **Examples**: `szívvel` ('with heart') vs `szível` ('likes'); `tervvel` vs `tervel` ('plans'); `bölccsé` ('into a sage') vs `bölcsé` ('the sage's'); `briddzsel` ('with bridge') vs `bridzsel` ('plays bridge').
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `REP` spelling-error detection must use morphology
- **Sources**: [AkH12 §82 d)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-007 -val/-vá after a hyphen: assimilated form by pronunciation
- **Statement**: When `-val/-vel` or `-vá/-vé` is attached with a hyphen (to digits, symbols, punctuation, abbreviations, acronyms, silent-letter foreign words), the assimilated form is always written, chosen by the pronounced final consonant of the spoken form.
- **Conditions**: hyphen attachment (`ORT-HYPH-020`).
- **Output**: `-` + `C` + `al/el` / `á/é`, where `C` is the final consonant of the pronunciation; after a vowel-final pronunciation `-val/-vel`.
- **Examples**: `4-gyel` (négy), `15%-kal` (százalék), `Bp.-tel` (Budapest), `DNS-sel` (dé-en-es), `Ft-tal` (forint), `K-mal` (kálium), `°C-kal`, `guillotine-nal`, `MTA-val` (em-té-á).
- **Exceptions**: -
- **Predictability**: `rule` given the spoken form
- **Lexical input**: spoken form (`ORT-NUM-002`, `ORT-ABBR-009`, `ORT-FOR-012`)
- **Interactions**: `ORT-NUM-003`, `ORT-ABBR-010`, `ORT-ABBR-014`, `ORT-FOR-012`
- **Sources**: [AkH12 §82 f), §217 a), §265 d), §282, §288 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-008 az, ez before consonant-initial suffixes
- **Statement**: The `z` of the demonstratives `az`, `ez` (also in `amaz`, `emez`, `mindez`, `ugyanaz`) totally assimilates to the first consonant of a following consonant-initial suffix and is written doubled; in the instrumental two forms are correct: `avval/evvel` (`z` assimilated to `v`) and `azzal/ezzel` (suffix `v` assimilated to `z`, as in `ORT-ASSIM-001`); the translative has only `azzá/ezzé`.
- **Conditions**: stem `az`/`ez` (and compounds ending in them) + suffix (case suffix or derivational, e.g. `-beli`) beginning with a consonant, except the accusative `-t`.
- **Output**: `a`/`e` + doubled first consonant of the suffix + rest of suffix.
- **Examples**: `abba`, `ebbe`, `ahhoz`, `ehhez`, `akképp`, `ekképp`, `arra`, `erre`, `akkor`, `ekkor`, `annak`, `abból`, `attól`, `abbeli`, `ebbeli`, `amannak`, `mindettől`, `ugyanahhoz`; instrumental `avval~azzal`, `evvel~ezzel`; translative `azzá`, `ezzé`.
- **Exceptions**: accusative `azt`, `ezt` (no assimilation; not stated in AkH12, from usage); vowel-initial suffixes: `azon`, `ezen`, `azok`, `azért`; terminative `addig`, `eddig` (doubled `d` before vowel-initial `-ig`; lexical, not in AkH12 §83); translative only `azzá`, `ezzé` (AkH11 §84 explicitly; AkH12 §83 silent).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-DEM`, `ORT-SUFX-006`; AkH11 restricted the rule to case suffixes (`abbeli` not listed) and listed `azzá`, `ezzé` as the only translatives; AkH12 lists `avval` first, corpus prefers the other variant: `azzal`/`ezzel` bucket 22, `avval`/`evvel` bucket 8 (generate `azzal`, `ezzel`); `azzá` bucket 9, `avvá` unattested
- **Sources**: [AkH12 §83, §44]; [AkH11 §84]
- **Validation**: goldens; attestation
- **Confidence**: `normative`

### ORT-ASSIM-009 az, ez with -fajta, -féle, -kora
- **Statement**: Words formed from `az/ez` with `-fajta`, `-féle`, `-kora` always have the doubled consonant.
- **Conditions**: these three elements.
- **Output**: `affajta`, `afféle`, `akkora`, `effajta`, `efféle`, `ekkora`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (6)
- **Lexical input**: none
- **Interactions**: `ORT-ASSIM-008`
- **Sources**: [AkH12 §83]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-010 a, e + postposition compounds
- **Statement**: Postpositions form compounds with the short demonstrative forms `a`, `e` (not `az`, `ez`).
- **Conditions**: demonstrative + postposition written together (`ORT-TOG-029`).
- **Output**: `a`/`e` + postposition.
- **Examples**: `afelé`, `afölött`, `amiatt`, `emellett`, `eszerint`, `evégett`, `anélkül`, `ezelőtt`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-TOG-029`
- **Sources**: [AkH12 §84]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-011 Definite j-suffixes after s, sz, z, dz verbs
- **Statement**: In the indicative definite conjugation the `j` of `-ja`, `-juk`, `-jük`, `-játok`, `-ják` becomes identical to a stem-final `s`, `sz`, `z`, `dz` and is written doubled (`ss`, `ssz`, `zz`, `ddz`); `-tsz`-final verbs of the `játszik` and `metsz` type follow the same rule (`tssz`; AkH12 names only these two, `hallatszik`, `látszik`, `tetszik` belong to the type); forms without `j` keep a single consonant.
- **Conditions**: verb stem ends in `s`, `sz`, `z`, `dz`; definite suffix beginning with `j`.
- **Output**: stem with final consonant doubled + suffix vowel part.
- **Examples**: `keressük`, `mossa`, `mossuk`, `olvassák`, `vadásszátok`, `visszük`, `hozzák`, `nézzük`, `végezzük`, `eddzük`, `peddzük`, `játssza`, `játsszuk`, `játsszátok`, `metsszük`; single: `játszom`, `játszod`, `metszi`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-PRES-003`, `ORT-ASSIM-012`; corpus `játsszuk` bucket 5, *`játszuk` unattested
- **Sources**: [AkH12 §43, §79]
- **Validation**: goldens; attestation
- **Confidence**: `normative`

### ORT-ASSIM-012 Imperative of s, sz, z, dz verbs
- **Statement**: The imperative `-j` becomes identical to stem-final `s`, `sz`, `z`, `dz` and is written doubled; also for `hallatszik`, `játszik`, `látszik`, `tetszik`, `metsz` (`tssz`); the short definite 2sg attaches `-d` to the bare stem.
- **Conditions**: verb stem ends in `s`, `sz`, `z`, `dz`; imperative.
- **Output**: doubled final consonant + person suffix; 2sg definite short: stem + `d`.
- **Examples**: `késsen`, `mossam`, `vesszenek`, `vadásszatok`, `hozzad`, `végezzünk`, `eddzétek`, `lopóddz(ál)`, `hallatsszon`, `játsszál`, `látsszam`, `tetsszél`, `metssze`; `nyesd`, `olvasd`, `mászd`, `úszd`, `húzd`, `végezd`, `edzd`, `játszd`, `metszd~mesd`.
- **Exceptions**: `ORT-ASSIM-020` (ss variants of `metsz`, `látszik`, `tetszik`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-IMP`; meaning contrast `játszanak` (indicative) vs `játsszanak` (imperative) must be preserved (homophones [jáccanak])
- **Sources**: [AkH12 §42, §80]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-013 Imperative of t-final verbs after a consonant or long vowel: ts
- **Statement**: If the stem-final `t` is preceded by a consonant other than `s`/`sz`, or by a long vowel (except `lát`, `bocsát`), the `t` is kept and the imperative marker is written `s`: `ts`; also before the short definite 2sg `-d`.
- **Conditions**: stem `...Ct` (C not `s`, `sz`) or `...V:t`, not in `ORT-ASSIM-014`.
- **Output**: stem + `s` + person suffix; 2sg definite short: stem + `sd`.
- **Examples**: `ejtse`, `féltsétek`, `márts`, `fűts`, `szítsa`, `taníts`; `ejtsd`, `mártsd`, `tanítsd`.
- **Exceptions**: `lát`, `bocsát` (`ORT-ASSIM-014`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-IMP`
- **Sources**: [AkH12 §76]
- **Validation**: goldens; attestation (`tanítsd` bucket 4)
- **Confidence**: `normative`

### ORT-ASSIM-014 Imperative of short-vowel + t verbs, lát, bocsát: ss
- **Statement**: After a short vowel + `t`, and in `lát`, `bocsát` (and the `lót` of `lót-fut`), both the stem `t` and the marker are written `s`: `ss`; the short definite 2sg has one `s` + `d`.
- **Conditions**: stem `...Vt` with short `V`, or `lát`, `bocsát`, `lót`.
- **Output**: stem minus `t` + `ss` + person suffix; 2sg definite short: stem minus `t` + `sd`.
- **Examples**: `alkoss`, `hallgassák`, `kössétek`, `nevessen`, `nyissátok`, `bocsássa`, `lássuk`, `lósson-fusson`; `hallgasd`, `kösd`, `nyisd`, `bocsásd`, `lásd`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-IMP`
- **Sources**: [AkH12 §77]
- **Validation**: goldens; attestation (`lássuk` 13)
- **Confidence**: `normative`

### ORT-ASSIM-015 Imperative of st, szt verbs
- **Statement**: `st`- and `szt`-final verbs form the imperative from the `t`-less stem like `s`/`sz` verbs (`ss`, `ssz`); the short definite 2sg adds `-d` to the `t`-less stem.
- **Conditions**: stem ends in `st` or `szt`.
- **Output**: `fess`, `fürösszétek`, `illesszük`, `rekessze`, `válassza`; `fesd`, `füröszd`, `illeszd`, `rekeszd`, `válaszd`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-IMP`; indicative definite of such verbs is regular (`festi`, `választja`)
- **Sources**: [AkH12 §78]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-016 Doubled digraphs are simplified except at compound boundaries
- **Statement**: A long consonant written with a digraph (or `dzs`) is written by repeating only its first letter, in simple words and in suffixed forms, common words and proper names alike; at a compound boundary two identical digraphs are written in full.
- **Conditions**: two adjacent identical digraph graphemes; boundary type: none or `+` -> simplify; `#` -> full.
- **Output**: `ccs ddz ddzs ggy lly nny ssz tty zzs` inside words and at suffix boundaries; `cscs dzdz dzsdzs gygy lyly nyny szsz tyty zszs` at compound boundaries.
- **Examples**: simple: `loccsan`, `hosszú`, `meggy`, `fütty`, `Berettyó`, `Meggyes`; suffixed: `jeggyel`, `mésszel`, `rosszal` ('with bad'), `briddzsel`, `eddzék`, `eljátsszuk`, `Arannyal`, `Kodállyal`; compounds: `kulcscsomó`, `jegygyűrű`, `nagygyűlés`, `fénynyaláb`, `díszszázad`, `észszerű`, `mészszerű`, `gipszszerű`.
- **Exceptions**: at a compound boundary with a third identical grapheme a hyphen is used (`ORT-HYPH-001`: `dzsessz-szerzemény`); in hyphenation the full digraphs reappear (`ORT-SPLIT-007`).
- **Predictability**: `rule`
- **Lexical input**: compound boundary (`DER-COMP`)
- **Interactions**: `ORT-SPLIT-007`; AkH11 treated `-szerű` as a suffix and simplified: `ésszerű`, `mésszerű`; AkH12: `észszerű`, `mészszerű`. Corpus still prefers old `ésszerű` (bucket 12) over `észszerű` (6): accept both in older texts, generate AkH12.
- **Sources**: [AkH12 §7 b), §91-§92, §132]; [AkH11 §94]; [HUWIKI-AKH12CH]
- **Validation**: goldens; attestation
- **Confidence**: `normative`

### ORT-ASSIM-017 Three identical consonant letters from suffixation become two
- **Statement**: When suffixation places a written long consonant `GG` next to a suffix beginning with `G`, only `GG` is written; this simplification applies to common words only, not to proper names (including their lowercase-written parts) and not at compound boundaries, where a hyphen keeps all letters.
- **Conditions**: stem (common word) ends in `GG`; next morph is a suffix (inflectional, including verbal person suffixes such as the definite 2sg `-d` in `fedd+d`, or derivational) starting with `G`.
- **Output**: stem + suffix minus its first grapheme.
- **Examples**: `orr+ra` -> `orra`; the definite imperative of `fedd` is `fedd` (`fedd+d`, not `feddd`); `toll+lal` -> `tollal`; `vádlott+tól` -> `vádlottól`; `fütty+tyel` -> `füttyel`; `könny+nyé` -> `könnyé`; `nagyobb+ban` -> `nagyobban`; `hall+lak` -> `hallak`; `több+be` -> `többe`; `szebb+ből` -> `szebből`.
- **Exceptions**: proper names, including their lowercase-written elements: `Hermann-nál`, `Széll-lel`, `Tarr-ról`, `Wittmann-né`, `Bernadett-től`, `Mariann-nal`, `Bonn-nal`, `Bükk-kel`, `Tallinn-nak`, `Elzett-tel`, `Knorr-ral`, `Blikk-kel`, title `Szerelmes szonett-tel` (`ORT-NAME-012`, `ORT-FOR-013`); compounds keep all letters with a hyphen: `balett-táncos`, `sakk-kör`, `rossz-szívű` ('wicked'), `váll-lövés` (`ORT-HYPH-001`); proper blends: `Keravill-lal` (`ORT-ABBR-016`).
- **Predictability**: `rule`
- **Lexical input**: proper-name flag; morph boundary type
- **Interactions**: AkH11 simplified given names (`Bernadettől`, `Mariannal`, `Marcellal~Marcellel`) and exempted only Hungarian family names, foreign proper names and compounds. `ORT-SPLIT-006` (split as written: `töb-be`, `or-ra`, `hal-lak`).
- **Sources**: [AkH12 §61, §91, §93, §265 a), §288 b)]; [AkH11 §94, §163 c)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-018 j-coalescence and j-assimilation outside the codified cases are not written
- **Statement**: The letters of stem and `j`-initial suffix are kept when the pronunciation totally assimilates or coalesces them: nouns `anya`, `atya` (truncated stem) and `nagy` + possessive `-ja/-juk/-jai`; `gy`/`ny`-final verbs + definite `-ja...` or imperative `-j`; `l`/`ll`-final verbs + definite or imperative `j`; `t`/`d`/`n`-final nouns + possessive `-ja/-je/-juk/-jük/-jai/-jei`; `t`/`d`/`n`-final verbs + indicative definite `j`-suffixes, and `d`/`n`-final verbs + imperative `-j`.
- **Conditions**: listed stem classes and suffixes.
- **Output**: stem + `j...` unchanged.
- **Examples**: `anyja` [annya], `anyjuk`, `atyja`, `nagyjai`; `hagyjátok`, `hagyjuk`, `fogyjon`, `hányja`, `hunyják`; `ítéljük`, `kapálják`, `tanulja`, `állj`, `beszéljetek`; `kertje`, `kürtjei`, `partja`, `botjai`, `kardjuk`, `hídjai`, `vadjai`, `kínjuk`, `turbánja`; `féltjük`, `mártjátok`, `látja`, `tanítják`, `szeretjük`, `hordja`, `adja`, `maradjanak`, `fonjátok`, `kenjük`.
- **Exceptions**: `s sz z dz` stems (`ORT-ASSIM-011`, `ORT-ASSIM-012`); `t`-final imperatives (`ORT-ASSIM-013` to `ORT-ASSIM-015`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: AkH11 also listed `bátya` (`bátyja`, `bátyjuk`) and `nagyja`, `nagyjaik`; spelling unchanged. Corpus: `hagyjuk` 15, *`haggyuk` unattested; `anyja` 16, *`annya` unattested.
- **Sources**: [AkH12 §53-§57]; [AkH11 §54]
- **Validation**: goldens; attestation
- **Confidence**: `normative`

### ORT-ASSIM-019 Imperative of lesz, tesz, vesz, visz, eszik, iszik, hisz
- **Statement**: The imperative marker is `-gy` on the vowel stem of `lesz`, `tesz`, `vesz`, `visz`, `eszik`, `iszik`; `hisz` has `ggy`; their short definite 2sg (except `lesz`) is vowel stem + `dd`.
- **Conditions**: these seven verbs.
- **Output**: `legyek`, `tegyen`, `vegyük`, `vigyétek`, `egyél`, `igyanak`; `higgy`, `higgye`, `higgyük`; `tedd`, `vedd`, `vidd`, `edd`, `idd`, `hidd`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (7 verbs)
- **Lexical input**: irregular verb class
- **Interactions**: `VRB` sz-d-v verbs, `ORT-SUFX-006` (`higgy~higgyél`)
- **Sources**: [AkH12 §42, §81]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ASSIM-020 metsz, látszik, tetszik: tssz or ss in the imperative
- **Statement**: Because of two pronunciations, the imperative forms of `metsz`, `látszik`, `tetszik` may be written with `tssz` or with `ss` (§32). `játszik` and `hallatszik` take `tssz` (`játsszál`, `hallatsszon`, §80); that they have no `ss` variant is an inference from §32 naming only the three verbs, not an AkH12 statement.
- **Conditions**: imperative (with `j`) of these verbs.
- **Output**: `metssze~messe`, `látsszék~lássék`, `tetsszem~tessem`; 2sg definite `metszd~mesd`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (3 verbs with variant)
- **Lexical input**: verb identity
- **Interactions**: `ORT-ASSIM-012`; corpus: `messe` 2, `lássék` 3, `tssz` forms unattested (the `ss` variant dominates usage)
- **Sources**: [AkH12 §32, §80]
- **Validation**: goldens; attestation
- **Confidence**: `normative`; the exclusion of `ss` for `játszik`, `hallatszik` inferred

### ORT-ASSIM-021 Imperative -j elsewhere
- **Statement**: Outside `ORT-ASSIM-012` to `ORT-ASSIM-015` and `ORT-ASSIM-019`, the imperative marker is written `j` (after `jö-` of `jön`: `jj`), including after `gy ny l ll d n`-final stems where pronunciation assimilates or coalesces.
- **Conditions**: imperative; stem-final grapheme not `s sz z dz` and not `t` (all `t`-final verbs fall under `ORT-ASSIM-013` to `ORT-ASSIM-015`).
- **Output**: stem + `j` + person suffix.
- **Examples**: `fújjon`, `lépjen`, `lőjön`, `szőjön`, `folyjon`, `hagyjon`, `adj`, `kenj`, `állj`, `jöjjön`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-ASSIM-018`, `ORT-LEN-026`
- **Sources**: [AkH12 §42, §54-§55, §57, §67]
- **Validation**: goldens
- **Confidence**: `normative`

## SUFX: suffix attachment to common words

### ORT-SUFX-001 Final a and e lengthen before suffixes
- **Statement**: A stem-final short `a` becomes `á` and `e` becomes `é` before (almost) all inflectional suffixes and most derivational suffixes; the process is fully productive (nonce words, loans, names written in Hungarian letters).
- **Conditions**: stem ends in letter `a` or `e`; the following suffix is lengthening (anything not in `ORT-SUFX-002`); stem is not an all-caps acronym (`ORT-ABBR-015`).
- **Output**: `a` -> `á`, `e` -> `é`.
- **Examples**: `apa` -> `apát`, `apátlan`; `kerge` -> `kergét`, `kergével`, `kergék`; `fa` -> `fák`, `fám`, `fája`, `fán`, `fáig`, `fáért`, `fáé`, `fában`, `fástul`, `fácska`, `fás`, `fátlan`, `fájú`; `óra` -> `óránként`; `francia` -> `franciául`; `eke` -> `ekéjük`, `ekéstül`; `Tompa` -> `Tompát`; `Ráckeve` -> `Ráckevén`, `Ráckevéig`; `Varga` -> `Vargáné`, `Vargáék`; verb forms `adna` -> `adnák`, `adnám`; `kóla` -> `kólám`.
- **Exceptions**: `ORT-SUFX-002`; all-caps proper acronyms `ORT-ABBR-015` (`ELTE-re`, `MTA-t`, `MTA-nak`); foreign spellings follow `ORT-FOR-010`; marginally inflected function words keep the short vowel (`de` -> `dek`; SMNy3, not AkH12; `NOM-LENG-003`).
- **Predictability**: `rule`
- **Lexical input**: suffix property `lengthening`
- **Interactions**: `PH-LEN` (low vowel lengthening), `NOM-LENG-001`, `NOM-LENG-003`, `VRB`
- **Sources**: [AkH12 §29, §288 a)-b)]; [SMNy3 pp. 786-788, tables (187)-(188)]
- **Validation**: goldens
- **Confidence**: `normative` (AkH12 §29 examples: `apát`, `apátlan`, `kergét`, `kergével`, `kergék`, `Tompát`, `Tompával`, `Ráckevén`, `Ráckevéig`); productivity claim and the remaining examples `consensus` (SMNy3)

### ORT-SUFX-002 Non-lengthening suffixes
- **Statement**: A stem-final `a/e` lengthens before most suffixes and stays short before a few (AkH12 §29: "csak néhány toldalék előtt marad rövid, a legtöbb toldalék előtt á-ra, illetőleg é-re változik"). Whether a suffix lengthens is a property of the suffix, so the rule is the default plus a suffix flag, not a list of stems. AkH12 names `-i` and `-ság/-ség` by example (§29), and a semi-word or compound member leaves the stem as it is (`campanileszerű`, `Coca-Cola-szerű`, §216 a)). Which further suffixes carry the flag is measured, not normative: on `a`/`e`-final bases `-ként`, `-féle`, `-fajta`, `-szerű`, `-beli`, `-kor`, `-szor`, `-izmus`, `-mentes` and `-szerte` keep the short vowel [WC2], `-képp(en)` and `-ista` likewise; `-s`, `-nyi` and `-stUl` lengthen; `-sÁg` and `-z` show both.
- **Conditions**: stem-final `a/e` + a suffix flagged non-lengthening.
- **Output**: short `a/e` kept.
- **Examples**: `apai`, `budai`, `velencei`, `ráckevei`, `goethei`; `kergeség`, `butaság`, `henyeség`; `campanileszerű`; measured: `faként`, `fajaként`; `faképp`; `órakor`; `faszerű`, `faféle`, `kefeszerű`; `filmpalotabeli`; `dadaizmus`, `dadaista`.
- **Exceptions**: 3sg possessive forms before `-kor`/`-ként` keep their own `á` (`órájakor`, `fájaként`: the lengthening belongs to `-ja`). The marginal learned suffixes `-izál`, `-ikus`, `-lógia` produce **no attested form at all** on an `a`/`e`-final base, so their membership is unmeasurable and they stay marked marginal (`OQ-07-001` remains open for exactly those three).
- **Predictability**: `rule` (lengthening by default, AkH12 §29) + `lexical` (the suffix flag; AkH12 gives `-i` and `-ság` explicitly, the rest is measured)
- **Lexical input**: suffix property `lengthening=false`
- **Interactions**: `ORT-SUFX-001`; `ORT-FOR-010` (foreign names: not before `-i`, `-szerű`)
- **Sources**: [AkH12 §29, §216 a)]; [SMNy3 p. 788 (188) b]; [WC2]; [ATT evidence/lexical-lists.md 2026-09-16]; [ATT evidence/productivity-tagset.md 2026-09-16]
- **Validation**: goldens; attestation (18 suffixes probed on `a`/`e`-final bases: 8 non-lengthening, 3 lengthening, 2 both, 5 unattested; list `non-lengthening-suffixes.tsv` in `local/lists/non-lengthening-suffixes.tsv` - working data, not yet read by the build). Cross-check for the four compound-like elements: no lengthening in 10,302 `a`/`e`-final types before `-beli`/`-fajta`/`-mentes`/`-szerte`, with 2 typo-level counterexamples (`korabeli` 211,974 : `korábeli` 3)
- **Confidence**: `normative` for the principle, `-i`, `-ság/-ség` (§29) and the semi-words (§216 a)); `measured` for `-ként -féle -fajta -szerű -beli -kor -szor -izmus -mentes -szerte`; `contested` only for `-izál`, `-ikus`, `-lógia`, which have no attested forms in the diagnostic environment

### ORT-SUFX-003 Other stem-final vowels
- **Statement**: Stem-final `i`, `u`, `ü` never lengthen; `á é í ó ő ú ű` are already long; Hungarian content words never end in short `o`/`ö` (loans in foreign spelling: `ORT-FOR-010`).
- **Conditions**: vowel-final stems other than `a/e`.
- **Output**: unchanged vowel.
- **Examples**: `macit`, `macija`, `adut`, `aduja`, `ürüt`, `kocsiban`; `hajót`.
- **Exceptions**: interjections `no`, `nono`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-FOR-010`
- **Sources**: [SMNy3 pp. 787-790]
- **Validation**: goldens
- **Confidence**: `consensus`

### ORT-SUFX-004 -i after a stem ending in i
- **Statement**: When the adjectival `-i` is added to a one-word place name (Hungarian or foreign) ending in `i`, only one `i` is written; a name ending in `ii` in its own spelling keeps it (`hawaii`). AkH12 states the rule for place names only; applying it to other `i`-final stems is an inference.
- **Conditions**: stem-final `i` + adjectival `-i`.
- **Output**: single `i`.
- **Examples**: `Tamási` -> `tamási`; `Zamárdi` -> `zamárdi`; `Helsinki` -> `helsinki`; `Pompeji` -> `pompeji`; `Hawaii` -> `hawaii`.
- **Exceptions**: `y`-final foreign names: `ORT-FOR-009`; silent final letter or multiword names take a hyphen: `ploieşti-i`, `Velikije Luki-i` (`ORT-FOR-012`, `ORT-FOR-014`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-NAME-020`
- **Sources**: [AkH12 §173, §215, §217 a)-b)]
- **Validation**: goldens; attestation (`helsinki` 10, *`helsinkii` unattested)
- **Confidence**: `normative`

### ORT-SUFX-005 Suffixes attach directly by default
- **Statement**: Inflectional and derivational suffixes are written solid with the stem; a hyphen is used only in the cases enumerated in `ORT-HYPH-020`.
- **Conditions**: any suffix.
- **Output**: stem+suffix solid.
- **Examples**: `házban`, `Petőfiről`, `cowboynak`, `Csajkovszkijjal`.
- **Exceptions**: `ORT-HYPH-020`
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-HYPH-020`
- **Sources**: [AkH12 §265]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SUFX-006 Suffixation doublets
- **Statement**: The following pairs of suffixed forms are both standard: short and long imperative (`dobj~dobjál`, `kérj~kérjél`, `vágj~vágjál`, `higgy~higgyél`, `írj~írjál`, `moss~mossál`, `nézz~nézzél`, `dolgozz~dolgozzál`); short and long definite imperative 2sg (`dobd~dobjad`, `vágd~vágjad`, `kérd~kérjed`, `hidd~higgyed`, `írd~írjad`, `mosd~mossad`, `nézd~nézzed`, `vájd~vájjad`); linking-vowel variants (`állsz~állasz`, `hullt~hullott`, `rosszallnám~rosszallanám`, `szégyellni~szégyelleni`); `-ll` verbs (`keveslem~kevesellem`, `kicsinyli~kicsinyelli`); stem variants (`öregedik~öregszik`, `igyekezik~igyekszik`, `gondolkodik~gondolkozik`); `-ódzik~-ózik` (`csókolódzik~csókolózik`, `hallgatódzik~hallgatózik`, `lopódzik~lopózik`, `rugdalódzik~rugdalózik`); `mondta~mondotta`; `értem~érettem`, `érted~éretted`; `ezzel~evvel`, `azzal~avval`.
- **Conditions**: listed forms and analogous members of the same classes.
- **Output**: both variants valid.
- **Examples**: as Statement.
- **Exceptions**: -
- **Predictability**: `lexical` (open "for example" list)
- **Lexical input**: variant links
- **Interactions**: `VRB`, `ORT-LEN-024`, `ORT-ASSIM-008`
- **Sources**: [AkH12 §33, §44]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SUFX-007 Stem doublets under suffixation
- **Statement**: Some nominal stems have two living suffixed forms: vowel-final and truncated stems, and `v`-stems with and without `v`.
- **Conditions**: listed and analogous stems (`NOM` classes).
- **Output**: both valid.
- **Examples**: `borjúk~borjak`, `borjút~borjat`, `varjúk~varjak`, `zászlója~zászlaja`, `erdője~erdeje`; `szók~szavak`, `faluk~falvak`, `(gyümölcs)lét~(gyümölcs)levet`, `(halas)tót~(halas)tavat`.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: stem class with variants
- **Interactions**: `NOM` v-stems
- **Sources**: [AkH12 §30]
- **Validation**: goldens
- **Confidence**: `normative`

## TOG: writing together or apart

Decision inputs used throughout: `POS` of both members; boundary morphology (is the syntactic relation marked by a case suffix or possessive?); member complexity (simple vs compound, `DER-COMP`); lexicon flag `lex_compound` (meaning change or custom, from the dictionary); syntactic parse where stated.

### ORT-TOG-001 Default: words apart; three sources of compounds
- **Statement**: Consecutive words are written apart. A sequence is written as one compound (solid or hyphenated) for one of three reasons: (1) the joint meaning differs from the sum of the members; (2) the grammatical relation between the members is formally unmarked; (3) codified writing tradition.
- **Conditions**: any two adjacent words.
- **Output**: apart unless `ORT-TOG-002` to `ORT-TOG-047` apply.
- **Examples**: `szilárd jellem`, `sok lehetőség`, `nagyon ügyes`, `rádiót javít`; (1) `nyitvatartás`; (2) `állattenyésztő`; (3) `bérbeadás`, `elmegy`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: see area preamble
- **Interactions**: all `TOG`, `63`, `HYPH`
- **Sources**: [AkH12 §94]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-002 Meaning change: solid; the same words in literal use: apart
- **Statement**: If the meaning of two words together is more or other than the sum of their meanings, they are written together; in literal, occasional use the same words are written apart.
- **Conditions**: lexicalised compound with specialised meaning.
- **Output**: solid for the lexicalised sense; apart for the compositional phrase.
- **Examples**: `szárazföld` 'mainland' vs `száraz föld`; `tízórai` 'mid-morning snack'; `tűzrőlpattant` 'lively'; `szóbeszéd` 'rumour'; `gyorsírás` 'shorthand' vs `gyors írás`; `jelzőlámpa` vs `(veszélyt) jelző lámpa`; `sokszög` 'polygon' vs `sok szög`; `földönfutó` 'homeless' vs `a földön futó (bogár)`.
- **Exceptions**: -
- **Predictability**: `lexical` (meaning change is a dictionary fact); for ambiguous strings a sense decision is needed
- **Lexical input**: `lex_compound` entry; for generation: intended sense; for checking: word-sense disambiguation or accept both
- **Interactions**: `ORT-TOG-015`, `ORT-TOG-016`, `ORT-TOG-017`
- **Sources**: [AkH12 §95]
- **Validation**: oracle:magyarispell; goldens
- **Confidence**: `normative`

### ORT-TOG-003 Occasional repetition: apart with comma
- **Statement**: Words repeated for emphasis do not form a compound: written apart with a comma between them.
- **Conditions**: identical word forms repeated ad hoc (not a lexicalised reduplication).
- **Output**: `X, X`.
- **Examples**: `fiam, fiam`; `várt, várt`; `gyorsan, gyorsan`.
- **Exceptions**: lexicalised reduplications `ORT-TOG-004`.
- **Predictability**: `lexical` (distinguishing ad hoc repetition from `ORT-TOG-004` needs the meaning shift listed there)
- **Lexical input**: reduplication lexicon
- **Interactions**: comma rules (out of scope)
- **Sources**: [AkH12 §96]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-004 Reduplication with unchanged members: hyphen
- **Statement**: Compounds formed by doubling an unchanged word, carrying a meaning shift (distributive, intensifying, 'rarely', 'almost'), are hyphenated; intensifying repetitions may have more than two members, with a hyphen between every pair; the conjunction `azaz` and a few doubled interjections are solid.
- **Conditions**: `X-X` with shifted meaning.
- **Output**: `X-X`, `X-X-X`; solid for `azaz`, `nana`, `nini`, `nono`, `hehe`, `dádá` type.
- **Examples**: `más-más`, `egy-egy`, `sok-sok`, `ki-ki`, `alig-alig`, `már-már`, `néha-néha`, `messze-messze`, `csupa-csupa-csupa`, `nagyon-nagyon-nagyon`; solid: `azaz`, `nana`, `nini`, `nono`, `hehe`, `dádá`.
- **Exceptions**: solid set: conjunction `azaz` and the `nana`, `nini`, `nono`, `hehe`, `dádá` type (open, small).
- **Predictability**: `rule` (form) / `lexical` (solid set)
- **Lexical input**: list of solid reduplicated particles
- **Interactions**: `ORT-TOG-003`, `ORT-HYPH-008`, `ORT-SPLIT-017`; distributive numeral repetition is also hyphenated (`2-2`, `húsz-húsz`, `ORT-NUM-013`)
- **Sources**: [AkH12 §97, §263 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-005 Same stem with different suffixes, postpositional repetition: apart
- **Statement**: Differently suffixed forms of the same stem, and repetitions joined by a postposition, are written apart, without comma.
- **Conditions**: `X+suffix1 X+suffix2`; `N N postposition`.
- **Output**: apart.
- **Examples**: `háztól házig`, `napról napra`, `szemtől szembe`, `jobbnál jobb`, `szebbnél szebb`, `tudván tudta`, `kérve kéri`, `várva várt`; `fej fej mellett`, `nap nap után`, `ember ember hátán`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none (morphology identifies same lemma)
- **Interactions**: -
- **Sources**: [AkH12 §98]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-006 Modified repetition with a non-independent member: hyphen
- **Statement**: Repetitions in modified form where one member does not exist on its own are hyphenated.
- **Conditions**: member not a free word.
- **Output**: `X-Y`.
- **Examples**: `nőttön-nő`, `réges-régi`, `unos-untalan`.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon
- **Interactions**: `ORT-TOG-013`; AkH11 §98 also listed `örökkön-örökké`, `körös-körül`, `végestelen-végig` here; AkH12 dropped them from the rule text (dictionary: `örökkön örökké`, `ORT-TOG-047`)
- **Sources**: [AkH12 §99]; [AkH11 §98]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-007 leges- is solid
- **Statement**: The hyperbolic prefix `leges-` is written together with the superlative.
- **Conditions**: `leges` + `leg...bb`.
- **Output**: solid.
- **Examples**: `legeslegjobb`, `legeslegnagyobb`, `legeslegszebb`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-SPLIT-015` (`leg-` as member)
- **Sources**: [AkH12 §100]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-008 Coordination: occasional listing vs loose coordinative compounds
- **Statement**: Occasionally juxtaposed coordinate constituents are separated by commas. If coordinated members jointly mean something other than each alone and both members take the suffixes, they form a loose compound written with a hyphen; members identically suffixed that do not exist as a compound unsuffixed are also hyphenated.
- **Conditions**: (a) ad hoc list -> comma; (b) coordinative pair whose joint meaning differs from the members and both members take the suffixes -> hyphen; also identically suffixed pairs whose unsuffixed form is not a compound (`apraja-nagyja`, `keze-lába`) -> hyphen. AkH12 records that the unsuffixed pairs are no compounds ("de nincs: apró-nagy, kéz-láb stb. összetétel"): a fact about the lexicon that licenses the hyphen of the suffixed pair, not a starred spelling.
- **Output**: `X, Y` / `X-Y` (each member inflected).
- **Examples**: (a) `apja, anyja`, `tejet, vajat, kenyeret`; (b) `süt-főz`, `sütnek-főznek`, `sütött-főzött`, `sütni-főzni`, `sütés-főzés`, `ég-föld`, `eget-földet`, `ide-oda`, `idébb-odább`; `apraja-nagyja`, `apraját-nagyját`, `keze-lába`, `kézzel-lábbal`, `oktató-nevelő`, `orrán-száján`, `testi-lelki`.
- **Exceptions**: `ORT-TOG-010`, `ORT-TOG-011`.
- **Predictability**: `lexical` (whether a pair is lexicalised)
- **Lexical input**: coordinative-compound lexicon
- **Interactions**: comma rules (out of scope)
- **Sources**: [AkH12 §101 a)-b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-009 Ad hoc pairs of same-function words: hyphen, restricted
- **Statement**: An occasional unit of two words with the same suffix and function may be shown by a hyphen; this is restricted to two members that are related or opposite in meaning; more members or unrelated words take commas or a conjunction.
- **Conditions**: exactly two coordinated words, same inflection/derivation, synonymous/related/antonymous.
- **Output**: `X-Y`; otherwise `X, Y és Z`.
- **Examples**: `kínos-fájdalmas (búcsú)`, `gazdasági-társadalmi (változások)`, `nyelvi-stilisztikai (elemzés)`, `kérő-esdeklő`, `sírva-nevetve`; not `nyelvtani-stilisztikai-helyesírási`, but `nyelvtani, stilisztikai és helyesírási`.
- **Exceptions**: -
- **Predictability**: `rule` (form), `lexical`/semantic (relatedness)
- **Lexical input**: semantic relatedness judgement (synonym/antonym/co-hyponym); count of members
- **Interactions**: `ORT-TOG-008`
- **Sources**: [AkH12 §101 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-010 Fused coordinative compounds: solid
- **Statement**: Coordinative compounds inflected only on the second member are written solid.
- **Conditions**: only the last member takes suffixes.
- **Output**: solid.
- **Examples**: `rúgkapál`, `rúgkapálnak`, `búbánat`, `búbánatos`, `hadsereg`, `hadseregben`, `hányaveti`, `hányavetiek`.
- **Exceptions**: `ORT-TOG-011`, `ORT-TOG-012`.
- **Predictability**: `lexical`
- **Lexical input**: lexicon
- **Interactions**: -
- **Sources**: [AkH12 §101 c)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-011 Coordinatives that do not inflect or inflect only at the end: hyphen kept
- **Statement**: Coordinative compounds that rarely take suffixes (and then only on the last member) are hyphenated, and the hyphen stays when suffixed.
- **Conditions**: lexicalised coordination of adjectives/nouns of this type; colour sequences.
- **Output**: `X-Y`, `X-Y+suffix`.
- **Examples**: `édes-bús`, `édes-búsan`, `sík-domború`, `sík-domborúra`, `rabló-pandúr`, `rabló-pandúrt`, `szoba-konyha`, `szoba-konyhás`, `sakk-matt`, `sakk-mattot`, `piros-fehér-zöld`, `piros-fehér-zöldet`.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon
- **Interactions**: `ORT-TOG-046`
- **Sources**: [AkH12 §102 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-012 Dual inflection and meaning split of coordinatives
- **Statement**: Some solid coordinatives can be inflected either on the last member (solid) or on both (hyphen); some pairs are hyphenated as an ad hoc description but solid as a lexicalised noun.
- **Conditions**: listed items.
- **Output**: variant spellings following the inflection pattern; POS/meaning-dependent spelling.
- **Examples**: `hírnév`: `hírneve~híre-neve`, `hírneves~híres-neves`; `szántó-vető (emberek)` vs `a szántóvetők` 'farmers'; `csúszó-mászó (kisgyerek)` vs `a csúszómászók` 'reptiles'.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon; POS (adjectival description vs noun)
- **Interactions**: `ORT-TOG-008`, `ORT-TOG-010`
- **Sources**: [AkH12 §102 b)-c)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-013 Twin words (ikerszók)
- **Statement**: Twin words (one or neither member meaningful alone) follow the coordinative rules: if both members take suffixes, or the twin is inherently suffixed, hyphen; if only the end takes suffixes, solid.
- **Conditions**: twin word lexicon entry with inflection pattern.
- **Output**: hyphen / solid.
- **Examples**: hyphen: `irul-pirul`, `irult-pirult`, `izeg-mozog`, `izegnek-mozognak`, `izgő-mozgó`, `dimbes-dombos`, `hébe-hóba`, `ímmel-ámmal`, `tyúkom-búkom`; solid: `csigabiga`, `csigabigák`, `hercehurca`, `limlom`, `limlomos`, `mendemonda`, `terefere`, `tereferél`, `ugribugri`.
- **Exceptions**: `ORT-TOG-014`; AkH12 dictionary moved `sete-suta` -> `setesuta`.
- **Predictability**: `lexical` (inflection pattern is lexical); unlisted twins (inference, not normative): hyphen if both members visibly inflected (`-t`, `-nak` on both), else solid
- **Lexical input**: twin lexicon
- **Interactions**: `ORT-TOG-014`; `csiki-csuki`: not in AkH12 rule text; spelling to be taken from the dictionary (Open questions)
- **Sources**: [AkH12 §95, §103]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-014 Non-inflecting and variable twin words
- **Statement**: Twins that do not or only exceptionally inflect are hyphenated, keeping the hyphen when suffixed; twin proper names likewise. Twins with variable inflection are solid in the base form and follow the inflection pattern otherwise.
- **Conditions**: lexical inflection type.
- **Output**: `csihi-puhi`, `csihi-puhit`, `piff-puff`, `csinn-bumm`, `csinn-bummja`, `Ista-Pista`, `Ista-Pistát`, `Anna-Panna`, `Anna-Pannával`; variable: `gizgaz`, `gizgazok`, `gizgazos~gizes-gazos`; `icipici`, `icipicit~icit-picit`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: twin lexicon with inflection type
- **Interactions**: `ORT-TOG-013`
- **Sources**: [AkH12 §104]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-015 Subject, attributive and quantifier phrases: apart unless meaning change
- **Statement**: Phrases with a subject relation (noun + `-ta/-te` participle), with an adjectival/participial attribute, or with a numeral/quantifier attribute are written apart; if the combination has a changed meaning it is written solid.
- **Conditions**: N + participle `-(t)A` with N as agent; Adj/participle + N; Num/quantifier + N.
- **Output**: apart; solid if `lex_compound`.
- **Examples**: apart: `anyja nevelte (gyermek)`, `árvíz mosta (töltés)`, `vihar okozta (kár)`, `általános iskola`, `érett gyümölcs`, `jobb kéz`, `olvasó lány`, `ezer esztendő`, `öt ujj`, `sok munka`, `huszonegy köbméter`; solid: `botcsinálta`, `eszeveszett`, `lélekszakadva`, `madárlátta`, `nyakatekert`, `ügyefogyott`, `forgószél`, `holtág`, `melegágy`, `söröspohár`, `ezermester`, `hatökör`, `öttusa`, `tizenkétszög`.
- **Exceptions**: customary solid `ORT-TOG-043`; material `ORT-TOG-022`; numerals + derived adjective `ORT-TOG-023`; participles `ORT-TOG-024`; plant and animal names follow the specialist nomenclature (outside AkH).
- **Predictability**: `rule` (apart) + `lexical` (meaning change)
- **Lexical input**: `lex_compound`
- **Interactions**: `ORT-TOG-002`
- **Sources**: [AkH12 §105]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-016 Marked object, adverbial and possessive phrases: apart unless meaning change
- **Statement**: If the relation is marked by a case suffix (object, adverbial) or a possessive marker, the words are written apart; with changed meaning they are written solid (suffix retained inside).
- **Conditions**: first word carries the relational case suffix or the head carries the possessive marker.
- **Output**: apart; solid if `lex_compound`.
- **Examples**: apart: `főtengelyt esztergál`, `könyvet ír`, `virágot szed`, `földre hull`, `jutalomra érdemes`, `készpénzért veszi`, `a ház alapja`, `a lakóház pincéje`; solid: `egyetért`, `helytáll`, `idejétmúlt`, `jótáll`, `semmittevő`, `ágrólszakadt`, `napraforgó`, `semmirekellő`, `tagbaszakadt`, `tökkelütött`, `barátfüle`, `bolondokháza`, `istennyila`, `vásárfia`.
- **Exceptions**: customary solid `ORT-TOG-043`.
- **Predictability**: `rule` + `lexical`
- **Lexical input**: `lex_compound`
- **Interactions**: `ORT-TOG-019` (unmarked counterpart)
- **Sources**: [AkH12 §106]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-017 Frequently co-occurring words fused with new meaning
- **Statement**: Words frequently adjacent that fused into a new meaning (mostly particles, conjunctions, adverbs) are solid; the separate spelling of the same string has its compositional meaning.
- **Conditions**: lexicon entry.
- **Output**: solid for the fused item; apart for the compositional reading.
- **Examples**: `nemsokára`, `sohasem`, `dehogy`, `hogyne`, `kétségkívül`, `csakhogy`, `hanem`, `ugyanis`; contrasts: `mintha` 'as if' vs `mint ha` ('than if'); `egyrészt` (conjunction) vs `egy részt (kapott)`; `dehogyis!` 'no way' vs `de hogy is gondolod?`; AkH12 addition `méghogy`.
- **Exceptions**: -
- **Predictability**: `lexical`; disambiguation signal: `mint ha` occurs after a comparative (`szebb, ha nevet, mint ha sír`); `egy részt` is an object NP (accusative of `rész` with numeral); `de hogy is` is followed by a verb in a question.
- **Lexical input**: fused-word lexicon; local syntax
- **Interactions**: `PND`, `SYN`
- **Sources**: [AkH12 §107]; [HUWIKI-AKH12CH]
- **Validation**: goldens; attestation (`nemsokára` 13, `kétségkívül` 11)
- **Confidence**: `normative`

### ORT-TOG-018 Postpositions
- **Statement**: A postposition is written apart from the preceding noun; an `-i` derivative of a postpositional phrase keeps the separation; lexicalised postpositional compounds with changed meaning are solid.
- **Conditions**: N + postposition (`PND-POSTP` list); `+i`.
- **Output**: apart: `bokor mögött`, `bokor mögötti`; solid: lexicon set.
- **Examples**: apart: `felesége iránt`, `föld alatt`, `föld alatti (üreg)`, `ház melletti`, `idő előtti`, `munka nélküli (jövedelem)`, `szó szerinti (idézet)`, `többek között`; solid: `délelőtt`, `délelőtti`, `délután`, `délutáni`, `földalatti` ('underground railway; illegal'), `hazafelé`, `holnapután`, `holnaputáni`, `mindenekelőtt`, `mindenekfelett`, `munkanélküli` (noun 'unemployed'), `napközben`, `rendszerint`, `visszafelé`.
- **Exceptions**: pronoun + postposition `ORT-TOG-029`; AkH12 `szó végi` (AkH11 `szóvégi`; also visible in the rule texts of both editions).
- **Predictability**: `rule` + `lexical` (solid set; `földalatti`, `munkanélküli` need POS/sense: noun or specialised sense -> solid, attributive literal -> apart)
- **Lexical input**: postposition list; solid lexicon; POS of the derivative
- **Interactions**: `ORT-TOG-036` (more derivation)
- **Sources**: [AkH12 §108]; [HUWIKI-AKH12CH]
- **Validation**: goldens; attestation (`délután` 17, `munkanélküli` 11)
- **Confidence**: `normative`

### ORT-TOG-019 Unmarked grammatical relation: solid
- **Statement**: If a relation that can be marked by a case suffix or a possessive marker (object, adverbial, possessive attribute) appears without it, the members form a compound written solid (subject to `63` and `HYPH`).
- **Conditions**: N/Adj/Adv + head (N, Adj, participle, deverbal noun) with no case suffix on the dependent and no possessive marker on the head, where the phrase would need one.
- **Output**: solid.
- **Examples**: `félig kész` -> `félkész`; `kéményt seprő` -> `kéményseprő`; `a nap sugara` -> `napsugár`; `pályát tévesztett` -> `pályatévesztett`; `vitaminban dús` -> `vitamindús`; `autómentő`, `kincskereső`, `munkavállaló`, `áldozatkész`, `hátúszó`, `szélvédett`, `gépkocsifényszóró`, `iskolaudvar`, `mosógépeladás`.
- **Exceptions**: proper-name or acronym first member: hyphen (`ORT-HYPH-003`, `ORT-HYPH-005`); three identical letters: hyphen (`ORT-HYPH-001`); length: `ORT-63-002`.
- **Predictability**: `rule`
- **Lexical input**: POS and case marking from morphology; dependency (whether a relation exists vs attributive adjective + noun, which is `ORT-TOG-015`)
- **Interactions**: `DER-COMP`
- **Sources**: [AkH12 §109-§110]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-020 Condensed-meaning (jelentéstömörítő) compounds: solid
- **Statement**: Compounds whose member relation can only be paraphrased by a longer structure are solid; this includes comparison compounds (colour, property) and many dish names.
- **Conditions**: noun + noun (or noun + adjective) without an expressible single-suffix relation.
- **Output**: solid (subject to `63`).
- **Examples**: `vámvizsgálat`, `csigalépcső`, `árvízvédelem`, `barázdabillegető`, `emlékkönyv`, `motorcsónak`, `testvérváros`, `villanyvilágítás`, `zsemlegombóc`, `tengeralattjáró-háború`; `cinóbervörös`, `galambősz`, `hófehér`, `hollófekete`, `mustársárga`, `búzavirágkék`, `elefántcsontfehér`, `vattacukor-rózsaszín`; `bableves`, `birkapörkölt`, `sárgaborsó-főzelék`.
- **Exceptions**: -
- **Predictability**: `rule` (noun + noun compounds default solid)
- **Lexical input**: none beyond compound detection
- **Interactions**: `ORT-63-002`
- **Sources**: [AkH12 §111]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-021 Structure-dependent writing: simple + simple solid, any compound member apart
- **Statement**: For material-attribute phrases, numeral + derived-adjective phrases, and (partly) `-ó/-ő` participle + noun phrases, the spelling depends on member structure: two simple words are written together; if either or both members are compounds, apart.
- **Conditions**: phrase types of `ORT-TOG-022`, `ORT-TOG-023`, `ORT-TOG-024`.
- **Output**: simple+simple -> solid; compound+simple, simple+compound, compound+compound -> apart.
- **Examples**: `aranylánc` / `fehérarany lánc` / `arany nyaklánc` / `fehérarany nyaklánc`; `ötméteres` / `tizenöt méteres` / `öt négyzetméteres` / `tizenöt négyzetméteres`; `védőgát` / `partvédő gát` / `védő homokgát` / `partvédő homokgát`.
- **Exceptions**: `ORT-TOG-044` (customary solid `-ó/-ő` + short compound).
- **Predictability**: `rule`
- **Lexical input**: compound status of each member (`DER-COMP`; verbal prefix + stem counts as compound: `összekötő`)
- **Interactions**: `ORT-TOG-022` to `ORT-TOG-024`
- **Sources**: [AkH12 §115]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-022 Material attribute
- **Statement**: A material noun (what the head is made of) is written together with a simple head noun if it is itself simple; apart if either member is a compound.
- **Conditions**: N(material) + N; semantic relation "made of".
- **Output**: per `ORT-TOG-021`.
- **Examples**: `aranygyűrű`, `alumíniumedény`, `faburkolat`, `gyapjúsál`, `kőfal`, `platinatégely`; `acél mérőszalag`, `bőr pénztárca`, `hernyóselyem ing`, `vasbeton gerenda`, `műanyag padlóburkolat`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: semantic class `material` for the first noun and relation "made of" (versus e.g. `aranyár` 'price of gold', which is a plain unmarked compound)
- **Interactions**: `ORT-TOG-021`
- **Sources**: [AkH12 §116]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-023 Numeral + derived adjective
- **Statement**: A simple cardinal (or `sok`, `több`, `fél`) + a simple word derived with `-i`, `-ú/-ű/-jú/-jű`, `-s`, `-nyi` is solid; if either member is compound, apart; POS change to a noun (banknotes) makes it solid; with digits always apart. `-nként` and `-nta/-nte` formations follow the same rule.
- **Conditions**: numeral + derived adjective (or `-nként`, `-nta/-nte` adverb).
- **Output**: solid / apart per member structure; digits: apart.
- **Examples**: solid: `harmincnapi`, `kétéves`, `kilenchavi`, `háromlábú`, `ötágú`, `hétfejű`, `kétzónájú`, `kétpetéjű`, `sokmilliós`, `többoldalas`, `húszfőnyi`, `kéthetenként`, `háromhavonta`, `ötévente`; apart: `hat vegyértékű`, `egy szótagos`, `negyven négyszögöles`, `száz hektoliternyi`, `huszonnégy tagú`, `negyvennyolc órás`, `tizenkét emeletes`, `száztizenhat napi`, `hetvenöt centiméteres`, `tizenöt naponként`, `három hónaponként`; POS change: `ötszázforintos`, `tízezerforintos` (banknotes); digits: `9 havi`, `2 éves`, `20 főnyi`, `116 napi`, `5 évente`, `15 naponként`.
- **Exceptions**: POS change (noun sense) -> solid.
- **Predictability**: `rule`
- **Lexical input**: numeral simplicity: simple = one of `egy két/kettő három négy öt hat hét nyolc kilenc tíz húsz harminc negyven ötven hatvan hetven nyolcvan kilencven száz ezer millió milliárd` or `sok több fél`; all others (`tizen-`, `huszon-`, `ötszáz`, `háromezer`) are compound; head simplicity from `DER-COMP`; noun-sense flag for banknote-type conversions
- **Interactions**: `ORT-TOG-021`, `ORT-NUM-012`; AkH12 dictionary also made noun/adjective + `-ú/-ű` combinations solid in several items (`papíralapú`, `alsófokú`, `elsőfokú`) and split others (`kis méretű`, `nagy fokú`, `magas rendű`, `zárt körű`) (`ORT-TOG-047`)
- **Sources**: [AkH12 §117]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-024 -ó/-ő participle + noun
- **Statement**: An occasional participial attribute is written apart; if participle and noun fused in meaning (a type, not an ongoing process), two simple words are solid; if either member is compound, apart even when fused.
- **Conditions**: V-`ó/ő` + N.
- **Output**: occasional -> apart; fused + simple/simple -> solid; fused + compound member -> apart.
- **Examples**: apart: `eltérő vélemény`, `éneklő gyerek`, `izzó fém`, `(az újoncokat) kiképző tiszt`, `közvetítő javaslat`, `parancsoló hang`, `sajgó térd`; solid: `desztillálótorony`, `izzólámpa`, `kiképzőtiszt` (rank), `védőnő`, `eladólány`, `felvonóhíd`, `ivóvíz`, `költőpénz`, `hálószoba`, `kutatóintézet`, `pihenőnap`, `belépőjegy`, `sétálóutca`; apart with compound member: `szélező körfűrész`, `növényvédő szer`, `rakétaindító állvány`, `földrengésjelző műszer`, `időjárás-előrejelző rendszer`, `ellenőrző leltár`, `előadó körút`, `összekötő híd`.
- **Exceptions**: customary solid `mérőműszer`, `vendéglátóipar` (`ORT-TOG-044`).
- **Predictability**: `lexical` (fusion) + `rule` (structure)
- **Lexical input**: `lex_compound`; compound status of each member; prefixed participles (inference, not stated in §118): count a one-syllable prefix + participle as simple and a prefix of two or more syllables as a member, as §139 does for syllable counting (`ORT-63-004`); this reproduces every §118 example: `kiképzőtiszt` (`ki`) solid vs `összekötő híd` (`össze`), `előadó körút` (`elő`), `ellenőrző leltár` (`ellen`) apart
- **Interactions**: `ORT-TOG-021`, `ORT-TOG-044`
- **Sources**: [AkH12 §118]
- **Validation**: goldens
- **Confidence**: `normative`; prefixed-participle treatment is `consensus` (inference from §139, consistent with all §118 examples; AkH11 §118 defined fusion more narrowly: capability/purpose reading or head as an adverbial of the action)

### ORT-TOG-025 Colour names
- **Statement**: Compound colour names are solid regardless of member structure; an adjective of shade added to a compound colour name is written apart.
- **Conditions**: colour adjective compounds.
- **Output**: solid (hyphen only by `63`); shade adjective + compound colour: apart.
- **Examples**: `halványlila`, `sötétzöld`, `világossárga`, `zöldeskék`, `almásszürke`, `haragoszöld`, `mélykék`, `törtfekete`, `püspöklila`, `búzavirágkék`, `vattacukor-rózsaszín`; apart: `sötét rózsaszín`, `világos narancssárga`, `halvány kékeszöld`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: semantic class `colour`; shade adjectives (`sötét`, `világos`, `halvány`, ...) ; compound status of the colour
- **Interactions**: `ORT-TOG-020`, `ORT-63-002`
- **Sources**: [AkH12 §119]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-026 Verbal prefix directly before its verb: solid
- **Statement**: A verbal prefix standing immediately before the verb, infinitive, participle or deverbal noun it belongs to is written together with it.
- **Conditions**: token sequence `PREFIX VERB-form` where `PREFIX` belongs to that verb.
- **Output**: solid.
- **Examples**: `átad`, `benéz`, `felnyit~fölnyit`, `lebecsül`, `visszaszerez`, `kiállítani`, `megtartó`, `eldobott`, `szembeszállva`, `megtartás`, `szembeszállás`.
- **Exceptions**: `ORT-TOG-027`, `ORT-TOG-028`, `ORT-TOG-030`; AkH12 dictionary moved some adverbial-case prefixes apart (`cserben hagy`, `síkra száll`, `útba igazít`, `véghez visz`, `sorba állás`, `partra szállás`) and others together (`számontart`, `együttérez`, `továbbtanul`) (`ORT-TOG-047`).
- **Predictability**: `rule`
- **Lexical input**: prefix list (AkH12, open): `abba`, `agyon`, `alá`, `át`, `be`, `bele`, `benn`, `egybe`, `el`, `ellen`, `elő`, `előre`, `fel~föl`, `félbe`, `félre`, `felül~fölül`, `fenn~fönn`, `hátra`, `haza`, `helyre`, `hozzá`, `ide`, `keresztül`, `ketté`, `ki`, `körül`, `közbe`, `közre`, `külön`, `le`, `meg`, `mellé`, `neki`, `oda`, `össze`, `rá`, `rajta`, `széjjel`, `szembe`, `szerte`, `szét`, `tele`, `tova`, `tovább`, `tönkre`, `túl`, `újjá`, `újra`, `utána`, `végbe`, `végig`, `vissza`; plus lexicalised adverb+verb units from the dictionary
- **Interactions**: `VRB-PREV`, `SYN` prefix placement; `ORT-63-004` (prefix syllables)
- **Sources**: [AkH12 §120 a)]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-027 Verbal prefix after the verb or separated: apart; attachment ambiguity
- **Statement**: A prefix following its verb, or separated from it by any word, is written apart (also in nominalised `be nem avatkozás`, `meg nem értés`). Before an auxiliary-like verb the prefix is written apart if it belongs to the following infinitive/participle, and together if it belongs to the adjacent verb.
- **Conditions**: prefix not immediately preceding its own verb.
- **Output**: apart.
- **Examples**: `pihend ki (magad)`, `hagyj fel (ezzel)`, `üljünk bele`, `menjetek előre`, `ne sározd össze`, `nem nézve oda`, `(akkor) ültünk le`, `(ti már) mehettek vissza`; `el ne késs`, `le nem tenné`, `újjá is építjük`, `közre kell bocsátani`, `haza szabad menni`, `föl sem véve`; contrasts: `megvan` vs `meg van töltve`; `megfogom a lepkét` vs `meg fogom nézni`; `leszokott (a dohányzásról)` vs `le szokott utazni`; `beleszeretett` vs `bele szeretett volna szólni`; `elleszek (nélküle)` vs `el leszek fáradva`; `megtudták (a hírt)` vs `meg tudták javítani`.
- **Exceptions**: -
- **Predictability**: `rule` given a parse
- **Lexical input**: dependency parse (the auxiliary list and decision procedure are inference, not normative): if the finite verb after the prefix is `fog`, `szokott`, `tud`, `akar`, `kell`, `szabad`, `lehet`, `szeret`, `kezd`, `bír`, `mer`, `óhajt`, `kíván`, `talál` (auxiliary use) followed by an infinitive, or `van`/`lesz` followed by a `-va/-ve` adverbial participle, and the prefix is a valid prefix of that infinitive/participle lemma -> apart; if prefix + adjacent verb is a lexicalised verb and no such complement follows -> solid; otherwise ambiguous (require parse)
- **Interactions**: `SYN` word order; `VRB-PREV`
- **Sources**: [AkH12 §120 b)-c)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-028 Repeated and opposite prefixes
- **Statement**: A repeated prefix is hyphenated to itself and the pair is written together with the verb; two opposite prefixes are hyphenated and written apart from the verb; `összevissza`, `szerteszéjjel`, `szerteszét` are adverbs written apart from the verb.
- **Conditions**: `P-P VERB`; `P1-P2 VERB` with opposite meaning.
- **Output**: `P-PVERB`; `P1-P2 VERB`.
- **Examples**: `ki-kinéz`, `le-leereszt`, `meg-megállt`, `össze-összevesznek`, `elő-előtűnő`, `vissza-visszatérve`, `vissza-visszatérés`; `ki-be járkál`, `le-föl sétál`, `ide-oda tekint`, `oda-vissza utazni`, `előre-hátra pillantva`, `le-föl sétálás`; `összevissza beszél`, `szerteszéjjel szórták`, `szerteszét szórva`, `szerteszét szórás`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: antonymous prefix pairs attested in §120 d) and §141 c): `ki/be`, `le/föl~fel`, `ide/oda`, `oda/vissza`, `előre/hátra`; further pairs (`fel/alá`) are inference
- **Interactions**: `ORT-TOG-046`
- **Sources**: [AkH12 §120 d), §141 c)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-029 Demonstrative a, e, az, ez + postposition
- **Statement**: `a/e/az/ez` + postposition is solid when it forms an adverb, a cataphoric pointer (to a following clause) or part of a conjunction-like construction; apart when the demonstrative emphatically points to a noun (typically repeated: `a ... a N POSTP`).
- **Conditions**: demonstrative + postposition.
- **Output**: solid / apart.
- **Examples**: solid: `ezelőtt`, `ezután`, `eszerint`, `afelől (hogy)`, `azalatt (míg)`, `anélkül (hogy)`, `ahelyett (hogy)`; apart: `a köré a ház köré`, `a szerint kell eljárni, nem e szerint`, `a nélkül a könyv nélkül`, `e miatt nem működik`, `csak az iránt érdeklődik`.
- **Exceptions**: -
- **Predictability**: `rule` given syntax
- **Lexical input**: syntactic signal: followed by an NP with the same postposition, or contrastive focus (`nem e szerint`), or relative clause `amelyik` -> apart; followed by `hogy`/`míg` clause or used adverbially -> solid
- **Interactions**: `ORT-ASSIM-010`, comma rules (out of scope) (comma with `anélkül hogy`)
- **Sources**: [AkH12 §121, §84]
- **Validation**: goldens; attestation (`anélkül` 16, `ahelyett` 13)
- **Confidence**: `normative`

### ORT-TOG-030 Adverb versus verbal prefix; pronominal hozzá, neki, rá
- **Statement**: A word usable as a prefix is written apart when its adverbial (locational, contrastive) role is emphasised; a prefix-shaped pronominal adverb clearly meaning a personal pronoun ('to him/her') is apart; personal-suffixed forms (`hozzám`, `nekünk`, `rád`) are always apart.
- **Conditions**: adverb/prefix homographs before a verb.
- **Output**: apart for adverbial or pronominal reading, solid for prefix reading.
- **Examples**: `abba tette` vs `abbahagyta`; `fenn~fönn maradt (a padláson)` vs `fennmarad (a neve)`; `félre állt (nem középre)` vs `félreállt`; `ide jött (nem a szomszédba)` vs `idejött`; `hozzá ment feleségül` vs `hozzájárul`; `neki megy a levél` vs `nekimegy (a szekrénynek)`; `(csak) rá gondolok`, `rossz rá gondolni` vs `rossz rágondolni` (different stress); `hozzám adták`, `nekünk jött`, `rád gondolok`.
- **Exceptions**: -
- **Predictability**: `lexical`/semantic
- **Lexical input**: semantic reading (contrastive location or personal referent vs aspectual/idiomatic prefix); prosodic stress not available in text; fallback (inference): prefix + verb that is a dictionary lemma -> solid
- **Interactions**: `ORT-TOG-026`
- **Sources**: [AkH12 §120 e)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-031 Emphatic element + pronoun
- **Statement**: Pronouns are written together with a preceding emphatic element: `egy-`, `ön-`, `önnön-`, `ugyan-`, `szint-`, `csak-`, `épp-` (equative sense) and personal pronoun + suffixed pronoun forms; the same words in their free sense are apart.
- **Conditions**: listed element + pronoun form.
- **Output**: solid; apart when the element is a free adverb ('only', 'exactly') or `magam` is an emphatic apposition.
- **Examples**: `egymagam`, `önmagam`, `önmagunk`, `önnönmaga`, `ugyanez`, `ugyanolyan`, `ugyanakkora`, `ugyanott`, `szintolyan`, `szintannyi`, `csakolyan`, `csakannyi`, `éppolyan`, `éppakkora`, `éppannyi`, `szintakkora`, `önnönmagatok`; `énbennem`, `tenálad`, `őtőle`, `mihozzánk`, `tinektek`, `ővelük`, `énmiattam`, `teáltalad`, `őutána`, `mimellettünk`, `tinélkületek`, `őközöttük`; apart: `én magam megyek`, `csak annyi szükséges`, `épp olyan esetben`.
- **Exceptions**: -
- **Predictability**: `rule` with semantic signal
- **Lexical input**: (inference, not normative) equative reading for `csak/épp + olyan/annyi/akkora` ('just as', typically with `mint`) -> solid; restrictive 'only'/'precisely such' -> apart
- **Interactions**: `PND`
- **Sources**: [AkH12 §122]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-032 se, sem, semmi in negative pronouns
- **Statement**: `se-`, `sem-`, `semmi-` as the first element of negative pronouns are solid; `se`, `sem` after a pronoun are apart, except `sohase`, `sohasem`, `sose`, `sosem`.
- **Conditions**: negative pronoun formation vs negative particle.
- **Output**: `semekkora`, `semennyi`, `sehogy`, `sehol`, `sehonnan`, `semeddig`, `semmikor`, `semmilyen`, `semmint`, `semmiféle`; apart: `semmi se`, `senki sem`, `senkinek se`, `semmitől sem`, `semmikor se`, `sehova se`, `sehol sem`, `sehogy sem`; solid: `sohase`, `sohasem`, `sose`, `sosem`.
- **Examples**: as Output.
- **Exceptions**: as listed.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-INDF-003`
- **Sources**: [AkH12 §123]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-033 akár-, bár-, mind-, minden-, né-, vala- pronouns
- **Statement**: Pronouns with these first elements are solid.
- **Conditions**: element + interrogative/pronoun stem.
- **Output**: solid.
- **Examples**: `akármekkora`, `akárhol`, `bármi`, `bármikor`, `mindannyi`, `mindenki`, `mindenhonnan`, `némelyik`, `néha`, `néhol`, `valami`, `valamerre`.
- **Exceptions**: not in §124 (inference): `akár` and `bár` as conjunctions/particles are free words (`akár ma is`).
- **Predictability**: `rule`
- **Lexical input**: POS (pronoun vs conjunction)
- **Interactions**: `PND-INDF-001`
- **Sources**: [AkH12 §124]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-034 Noun used as a qualifier; intensifiers; kind-denoting nouns
- **Statement**: A noun qualifier denoting occupation, age, status or group is written apart from its head, as are intensifying words; a noun qualifier denoting the kind of the head is solid, except breed names of animals.
- **Conditions**: N + N attributive.
- **Output**: occupation/status/age -> apart; intensifier -> apart; kind -> solid; animal breed + species -> apart.
- **Examples**: apart: `ajándék könyv`, `belgyógyász barátunk`, `gyermek király`, `huszár főhadnagy`, `mérnök bátyám`, `professzor asszony`, `rendőr százados`, `tanár úr`, `tanító néni`, `vendég néni`, `körte alakú`; intensifiers: `csuda érdekes`, `jó nagy`, `jó néhányszor`, `kutya hideg`, `nagy néha`, `szép piros`; kind: `csiperkegomba`, `fiúgyerek`, `fűszeráru`, `jonatánalma`, `lakatosmester`, `legényember`, `sasmadár`; breeds: `mangalica sertés`, `racka juh`.
- **Exceptions**: AkH12 dictionary `csoda szép` (was `csodaszép`), `orvos szakértő`; `alak` + `-ú`: `körte alakú` apart.
- **Predictability**: `lexical`/semantic
- **Lexical input**: semantic class of the first noun (`occupation`, `age`, `status`, `kind/subtype`, `breed`)
- **Interactions**: `ORT-TOG-019`, `ORT-NAME-015`
- **Sources**: [AkH12 §125]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-035 Proper name as attribute of a common noun: apart
- **Statement**: A proper name standing as an attribute (appositive/qualifier) of a common noun is written apart; this differs from an unmarked compound (`ORT-HYPH-003`).
- **Conditions**: `Name N` where N is a title, kinship term, profession, or a classifier (`család`, `fiú`, brand + product).
- **Output**: apart.
- **Examples**: `Dezső bácsi`, `Fazekas úr`, `Kovács mérnök`, `Mariska néni`, `a Kalmár fiú`, `a Nagy család`, `Fabulon arckrém`, `Volkswagen gépkocsi`.
- **Exceptions**: dynasty `ház/család/dinasztia` hyphen (`Árpád-ház`, `Habsburg-család`, `ORT-NAME-014`, AkH12 §166); collective nouns after names of groups stay apart (`Benedek család`, `Kaláka együttes`, §164).
- **Predictability**: `rule` with semantic signal
- **Lexical input**: relation type (operationalisation is inference): appositive/classifying (N denotes the named entity itself: `Nagy család` = the family called Nagy) -> apart; N denotes something related to the name (`Petőfi-szobor` = statue of Petőfi) -> hyphen
- **Interactions**: `ORT-HYPH-003`, `ORT-NAME-013`
- **Sources**: [AkH12 §126, §164, §166]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-036 Derivational suffixes on a phrase written apart
- **Statement**: One derivational suffix added to a phrase written apart generally keeps the separation; two or more derivational suffixes make it solid.
- **Conditions**: phrase written apart (e.g. adverbial + verb, postpositional phrase, negated prefix phrase) + derivational suffixes on the last word; count the derivational suffixes on the last word relative to the underived phrase (`egyenlő oldal` + `-ú` + `-ság`; `meg nem támad` + `-ás` + `-i`).
- **Output**: count 1 -> apart; count >= 2 -> solid.
- **Examples**: `házhoz szállít` -> `házhoz szállítás` (1: `-ás`) -> `házhozszállítási` (2: `-ás`, `-i`); `egymás után` -> `egymásutániság` (`-i`, `-ság`); `meg nem támad` -> `megnemtámadási`; `egyenlő oldal` -> `egyenlő oldalú` (1: `-ú`) -> `egyenlőoldalúság` (2: `-ú`, `-ság`).
- **Exceptions**: customary solid spellings with one suffix (`bérbeadás`, `kézhezvétel`, `rendbehozatal`: `ORT-TOG-043`, §135); a postpositional phrase + `-i` stays apart as the one-suffix case (`ORT-TOG-018`).
- **Predictability**: `rule` (the norm says 'generally'; lexical exceptions via `ORT-TOG-043`)
- **Lexical input**: morphological segmentation of derivational suffixes
- **Interactions**: `ORT-TOG-018`, `ORT-TOG-043`, `ORT-TOG-045`
- **Sources**: [AkH12 §127, §135]
- **Validation**: goldens; attestation (`megnemtámadási` 1)
- **Confidence**: `normative`

### ORT-TOG-037 Superlative of a phrase written apart: solid
- **Statement**: The superlative (`leg-...-bb`) of an adjectival phrase written apart is solid; the comparative stays apart.
- **Conditions**: phrase + `leg-` ... `-bb`.
- **Output**: solid superlative.
- **Examples**: `szívhez szóló`, `szívhez szólóbb`, `legszívhezszólóbb`; `legharcrakészebb`, `leghasznothajtóbb`.
- **Exceptions**: paraphrase preferred: `harcra legkészebb`, `leginkább hasznot hajtó`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-TOG-036`
- **Sources**: [AkH12 §128]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-038 Bound first elements
- **Statement**: First elements not used as independent words are always solid with what follows.
- **Conditions**: bound element list (AkH12, open): `al-`, `bel-`, `elő-`, `ét-`, `gyógy-`, `kultúr-`, `kül-`, `pót-` (the restriction of `elő-` to the non-prefix 'fore-' sense is inference).
- **Output**: solid.
- **Examples**: `alelnök`, `belföld`, `előtag`, `külügy`, `pótágy` (AkH12); `gyógyszer`, `kultúrház`, `étvágy` (added).
- **Exceptions**: -
- **Predictability**: `lexical` (list)
- **Lexical input**: bound-element list
- **Interactions**: `ORT-63-003` (members)
- **Sources**: [AkH12 §129]
- **Validation**: goldens; attestation (`alelnök` 12)
- **Confidence**: `normative`

### ORT-TOG-039 -d ordinal attributes
- **Statement**: An ordinal-type numeral with suffix `-d` used as an attribute is solid with the following noun or adjective.
- **Conditions**: `NUM-d` + N/Adj.
- **Output**: solid.
- **Examples**: `hetedízben`, `ötödévben`, `ezredévi`, `másodrendű`, `ötödéves`, `tizenötödrangú`, `harmadfél`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-NUM-015` (fractions in letters)
- **Sources**: [AkH12 §130]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-040 -d numeral + maga
- **Statement**: `-d` numeral attributes are solid with forms of `maga`; long forms are hyphenated per `ORT-63-002` (§131 refers to §139).
- **Conditions**: `NUM-d` + `maga` forms.
- **Output**: solid / hyphen by syllable count.
- **Examples**: `másodmagammal`, `századmagával`, `huszonötödmagunkkal`, `tizenkilenced-magunkkal`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-63-002`
- **Sources**: [AkH12 §131]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-041 -fajta, -féle, -nemű, -rét, -rétű, -szerű with common words
- **Statement**: These second elements (open list) are solid with common words (no simplification of `sz`+`sz`: `ORT-ASSIM-016`); with digits they take a hyphen; in lists a hyphen marks the omitted shared element.
- **Conditions**: common-word first member.
- **Output**: solid; digit + hyphen.
- **Examples**: `másfajta`, `tízféle`, `10-féle`, `többféle`, `egynemű`, `negyedrét`, `sokrétű`, `gipszszerű`, `nagyszerű`; `bükk-, mogyoró- és nyírfafélék`.
- **Exceptions**: proper names and acronyms take `-fajta`, `-féle`, `-forma`, `-szerű` with a hyphen (`Eötvös-féle`, `OTP-szerű`, `ORT-HYPH-004`, §133); three identical consonant letters at the boundary: hyphen (`expressz-szerű`, `ORT-HYPH-001`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-HYPH-004`, `ORT-HYPH-007`, `ORT-SUFX-002`; AkH11 §141 also cited `ésszerű` (traditional simplified spelling, dropped from the AkH12 rule text); AkH11 hyphenated only `-féle`, `-szerű` after names
- **Sources**: [AkH12 §132-§133, §263 c)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-042 Fixed multiword expressions: apart
- **Statement**: Many always co-occurring fixed expressions (idioms, terms) are written apart.
- **Conditions**: lexicalised expression without compound status.
- **Output**: apart.
- **Examples**: `állást foglal`, `szemet szúr`, `véget ér`, `dugába dől`, `lépre csal`, `főnévi igenév`, `virtuális tér`.
- **Exceptions**: meaning-change compounds `ORT-TOG-016`.
- **Predictability**: `lexical`
- **Lexical input**: multiword-expression lexicon (for NLP: treat as MWE, spelled apart)
- **Interactions**: `ORT-TOG-002`
- **Sources**: [AkH12 §137]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-043 Customary solid spellings without meaning change
- **Statement**: By established custom some subject, attributive, marked adverbial and marked possessive combinations are solid without meaning change.
- **Conditions**: dictionary list.
- **Output**: solid.
- **Examples**: `átokverte`, `dércsípte`, `dérlepte`, `molyrágta`, `napsütötte`, `porlepte`, `szúette`; `fiatalkorú`, `haditerv`, `jótett`, `útitárs`; `bérbeadás`, `célravezető`, `életbelépés`, `kézhezvétel`, `lényegbevágó`, `rendbehozatal`; `szeretetreméltóság`; `holdtölte`, `napkelte`, `tojásfehérje`; `légiposta`.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: dictionary
- **Interactions**: `ORT-TOG-044`
- **Sources**: [AkH12 §134-§135]
- **Validation**: oracle:magyarispell
- **Confidence**: `normative`

### ORT-TOG-044 Customary solid -ó/-ő + compound
- **Statement**: Some `-ó/-ő` participle + compound-noun combinations (typically with a two-syllable compound head) are solid by custom, against `ORT-TOG-024`.
- **Conditions**: dictionary list.
- **Output**: solid.
- **Examples**: `bevásárlóközpont`, `forgószínpad`, `javítóműhely`, `mérőműszer`, `mozgókórház`, `vendéglátóipar`.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: dictionary
- **Interactions**: `ORT-TOG-024`, `ORT-PRIN-014`
- **Sources**: [AkH12 §136]
- **Validation**: oracle:magyarispell
- **Confidence**: `normative`

### ORT-TOG-045 Phrase written apart + further member
- **Statement**: When a member attaches to a whole phrase normally written apart, the phrase is written together and the new member attached with a hyphen, regardless of syllable count; the same with a first member attached to a phrase. A paraphrase is preferred when possible.
- **Conditions**: `[A B] # C` or `C # [A B]` where `A B` is a phrase written apart and `C` relates to the whole phrase.
- **Output**: `AB-C`; `C-AB`.
- **Examples**: `hajlított bútor` + `gyár` -> `hajlítottbútor-gyár`; `hideg víz` + `csap` -> `hidegvíz-csap`; `házi feladat` + `készítés` -> `házifeladat-készítés`; `kémia` + `házi feladat` -> `kémia-házifeladat`; `légi fénykép` + `montázs` -> `légifénykép-montázs`; preferred paraphrases: `hőre lágyuló műanyagok feldolgozása` (not `hőrelágyulóműanyag-feldolgozás`), `lőtt vadak árusítása` (not `lőttvad-árusítás`).
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: bracketing (whether `C` modifies the whole phrase: `hidegvíz-csap` 'tap for cold water' vs `hideg vízcsap` 'a cold water tap')
- **Interactions**: `ORT-63-008`
- **Sources**: [AkH12 §141 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-046 Two-member unit + third element
- **Statement**: When two coordinated words act as one unit modifying a third, they are hyphenated and written apart from the third; likewise opposite prefixes before a verb. In specialist texts an en dash may be used instead.
- **Conditions**: `[A B] C`, A and B coordinated.
- **Output**: `A-B C`.
- **Examples**: `élet-halál harc`, `kutya-macska barátság`, `réz-arany ötvözet`, `föl-le sétál`, `ki-be nézeget`; `szövő-fonó ipar` (§263 b)).
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: coordination detection
- **Interactions**: `ORT-TOG-028`, `ORT-HYPH-013`, `PUN-DASH`; AkH11 dictionary `élethalálharc`, `napéjegyenlőség` (AkH12 `élet-halál harc`, `nap-éj egyenlőség`, per HUWIKI-AKH12CH); AkH11 §139 c) had the same rule with `nyomás-térfogat diagram`, `ajak-fog hang`
- **Sources**: [AkH12 §141 c), §264]; [AkH11 §139 c)]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-TOG-047 AkH11 to AkH12 dictionary changes in writing together or apart
- **Statement**: The 2015 dictionary changed the spelling of specific items; texts before 2015 legitimately use the old forms.
- **Conditions**: document date or declared norm AkH11.
- **Output**: AkH12 form for generation; accept AkH11 form for older texts (tag `norm=AkH11`).
- **Examples**: AkH11 apart -> AkH12 solid: `számon tart` -> `számontart`, `együtt érez` -> `együttérez`, `előre gyártott` -> `előregyártott`, `nagyra törő` -> `nagyratörő`, `tenyérbe mászó` -> `tenyérbemászó`, `számon kér` -> `számonkér`, `nyitva tartás` -> `nyitvatartás`, `észre térítés` -> `észretérítés`, `alsó fokú` -> `alsófokú`, `első fokú` -> `elsőfokú`, `éti csiga` -> `éticsiga`, `hájas tészta` -> `hájastészta`, `kemény tojás` -> `keménytojás`, `lágy tojás` -> `lágytojás`, `keresztes pók` -> `keresztespók`, `szabad piac` -> `szabadpiac`, `szabad strand` -> `szabadstrand`, `szabad vers` -> `szabadvers`, `száraz borsó` -> `szárazborsó`, `vegyes úszás` -> `vegyesúszás`, `fekvő beteg` -> `fekvőbeteg`, `járó beteg` -> `járóbeteg`, `görög katolikus` -> `görögkatolikus`, `teljes körűen` -> `teljeskörűen`, `papír alapú` -> `papíralapú`, `on-line` -> `online`, `taj-szám` -> `tajszám`, `sete-suta` -> `setesuta`, `valószínűség-számítás` -> `valószínűségszámítás`, `szárazbab-főzelék` -> `szárazbabfőzelék`; context-dependent: `feketedoboz`, `jóbarát`, `alkotómunka`, `egyidőben`, `gyengénlátó`; AkH11 solid -> AkH12 apart: `cserbenhagy` -> `cserben hagy`, `síkraszáll` -> `síkra száll`, `útbaigazít` -> `útba igazít`, `véghezvisz` -> `véghez visz`, `fejbelövés` -> `fejbe lövés`, `sorbaállás` -> `sorba állás`, `tervbevétel` -> `tervbe vétel`, `vízbefúlás` -> `vízbe fúlás`, `nyakoncsapás` -> `nyakon csapás`, `kisméretű` -> `kis méretű`, `nagyméretű` -> `nagy méretű`, `magasrendű` -> `magas rendű`, `nagyfokú` -> `nagy fokú`, `zártkörű` -> `zárt körű`, `házinyúl` -> `házi nyúl`, `sóskifli` -> `sós kifli`, `csodaszép` -> `csoda szép`, `orvosszakértő` -> `orvos szakértő`, `szóvégi` -> `szó végi`, `élethalálharc` -> `élet-halál harc`, `napéjegyenlőség` -> `nap-éj egyenlőség`, `örökkön-örökké` -> `örökkön örökké`, `partraszállás` -> `partra szállás` (context-dependent).
- **Exceptions**: `együttérez`, `papíralapú` and `orvos szakértő` are absent from the authoritative change list and carry no norm-change claim; `papíralapú` is nevertheless solid under the noun + `-ú/-ű` main rule of AkH12, which this chapter does not encode.
- **Predictability**: `lexical`
- **Lexical input**: norm version per document
- **Interactions**: corpus lags: `zártkörű` (AkH11) bucket 8, `nyitvatartás` 8, `papíralapú` 8, `online` 20
- **Sources**: [KESZLER2016 pp. 95-100 "A megváltozott helyesírású szavak jegyzéke"] (the AkH12 editor's own change list, 190 entries; 34 of the 37 spot-checked items here match it verbatim, the other 3 are the Exceptions above); [HUWIKI-AKH12CH]
- **Validation**: attestation; goldens against the change list (`local/research-akh/keszler2016-akh12-changes.txt`)
- **Confidence**: `normative` (`OQ-07-005` resolved 2026-09-16)

## 63: the 6:3 rule for long compounds

Algorithm summary (normative content in the rules below):

```text
spell_compound(members M1..Mn with binary bracketing, all common words):
  if any boundary needs a hyphen for other reasons (proper name, acronym, letter symbol,
     three identical letters, phrase + member):            -> HYPH rules, then 63-008/63-009
  k = number of counted members (63-004, 63-005)
  if k <= 2:                                              -> solid          (63-001)
  s = syllables(uninflected form, derivational suffixes except -i counted)   (63-003)
  if s <= 6:                                              -> solid          (63-002)
  if lexical exception:                                   -> solid          (63-007)
  else                                                    -> hyphen at main boundary (63-006)
```

### ORT-63-001 Two-member compounds of simple common words are solid at any length
- **Statement**: A subordinative compound of two simple common words is written solid regardless of its syllable count.
- **Conditions**: exactly two members, each a simple (non-compound) common word; one-syllable verbal prefixes do not count as members (`ORT-63-004`); treating derived words as simple members is inferred from the examples (`felezőegyenes`, `teljesítménynövekedés`), not stated.
- **Output**: solid.
- **Examples**: `asztallap`, `felezőegyenes`, `matematikatanítás` (8 syllables), `nitrogénasszimiláció`, `paradicsomsaláta`, `televíziókészülék`, `teljesítménynövekedés`, `befogadóképesség`, `szövegkiegészítés`.
- **Exceptions**: three identical letters: `sakk-kör` (`ORT-HYPH-001`); proper-name, acronym and abbreviation members (`ORT-HYPH-003` to `ORT-HYPH-006`).
- **Predictability**: `rule`
- **Lexical input**: member segmentation (`DER-COMP`)
- **Interactions**: `ORT-63-002`
- **Sources**: [AkH12 §139]; [AkH11 §138] (same rule)
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-63-002 Multiple compounds: solid up to 6 syllables, hyphen above
- **Statement**: Compounds of more than two members are solid up to six syllables; longer ones may be hyphenated at the boundary of the two main members. Overlong but regular compounds can be avoided by a syntactic paraphrase.
- **Conditions**: counted members >= 3; syllables per `ORT-63-003`.
- **Output**: <= 6: solid; >= 7: main-left-main-right with one hyphen.
- **Examples**: solid: `cseppkőbarlang`, `élelmiszeripar`, `gépkocsivezető`, `ivóvízellátás`, `mértékegységrendszer`, `nyersolajmotor`, `rendőrjárőr`, `tapétaszaküzlet`, `tűzoltólaktanya`; hyphen: `csapatzászló-avatás`, `dokumentumfilm-bemutató`, `foszforműtrágya-gyártás`, `könyvritkaság-gyűjtemény`, `munkaerő-nyilvántartás`, `tornász-csapatbajnokság`; paraphrase: `tervezőiroda-vezető` = `a tervezőiroda vezetője`, `gépkocsivezető-tanfolyam` = `gépkocsivezetői tanfolyam`.
- **Exceptions**: `ORT-63-007`.
- **Predictability**: `rule`
- **Lexical input**: members, bracketing
- **Interactions**: `ORT-63-003` to `ORT-63-009`. AkH12 says long compounds "may be" hyphenated (optional wording) while its dictionary hyphenates consistently; AkH11 said they are "mostly" hyphenated. Generate the hyphen; accept the solid form as a tolerated variant only if explicitly configured. Note: `mértékegységrendszer` (mér-ték-egy-ség-rend-szer = 6 syllables, `mérték#egység#rendszer`) is listed solid in AkH12, consistent with the count.
- **Sources**: [AkH12 §139]; [AkH11 §138]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-63-003 Syllable counting
- **Statement**: Count syllables (= vowels, `ORT-SPLIT-001`) of the compound without inflectional suffixes (plural, possessive, case, verb inflection); derivational suffixes are counted, except the adjectival `-i`.
- **Conditions**: compound to be tested by `ORT-63-002`.
- **Output**: integer syllable count.
- **Examples**: `anyagcserezavar(a)` counts `anyagcserezavar` = 6 -> solid; `jogsegélyszolgálat(hoz)` = 6 -> solid; `divatlapszerkesztő` = 6 -> solid, `divatlap-szerkesztőség` = 7 -> hyphen; `adó-visszatérítés` = 7, `adó-visszatérítési` (`-i` ignored) -> hyphen; `híradástechnikai`, `munkaerőpiaci`, `szakközépiskolai` (6 without `-i`) -> solid; `vendéglátóipari` (AkH12) vs `vendéglátó-ipari` (AkH11).
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: morphological analysis separating inflection from derivation; for foreign-spelled members, pronounced vowels (`ORT-SPLIT-012`)
- **Interactions**: AkH11 counted `-i` too (`kerékpár-javítási`); AkH12 changed this. Corpus: `vendéglátóipari` 10, `munkaerőpiaci` 12, `szakközépiskolai` 10 (solid forms dominant). Participle `-ó/-ő`, `-ás/-és`, `-s`, `-ú/-ű`, `-ség` are derivational and counted.
- **Sources**: [AkH12 §139]; [AkH11 §138]; [HUWIKI-AKH12CH]
- **Validation**: goldens; attestation
- **Confidence**: `normative`

### ORT-63-004 Verbal prefixes count as members only with two or more syllables
- **Statement**: A verbal prefix is a separate member for the 6:3 rule only if it has at least two syllables; one-syllable prefixes are part of the member they attach to.
- **Conditions**: prefixed member in a compound.
- **Output**: member count adjusted.
- **Examples**: `adat#vissza#keresés` -> 3 members, 7 syllables -> `adat-visszakeresés`; `elő#adó#művészet` -> `előadó-művészet`; `be+fogadó#képesség` -> 2 members -> `befogadóképesség`; `szöveg#ki+egészítés` -> `szövegkiegészítés`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: prefix syllable count: one-syllable prefixes `át`, `be`, `benn`, `el`, `fel`, `föl`, `fenn`, `fönn`, `ki`, `kinn`, `le`, `meg`, `rá`, `szét`, `túl` (also prefix-like `ott`, `jól`, `jót`, `helyt` as in emMorph; the list is not in AkH12, it is derived from the prefix inventory); all others in the `ORT-TOG-026` list have 2+ syllables
- **Interactions**: `ORT-TOG-026`; AkH11 had the same rule (`kölcsön-visszafizetés`, `állománykiegészítés`)
- **Sources**: [AkH12 §139]; [AkH11 §138]; [emMorph stemalt.rul] (study only)
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-63-005 Foreign first elements as members
- **Statement**: A foreign first element counts as a separate member if it is used independently in Hungarian, or if the second element joined to it is used independently; otherwise the foreign formation is a single member (the last clause is inferred; AkH12 states only the positive cases).
- **Conditions**: learned prefixes/combining forms (`mini`, `anti`, `bio`, `mikro`, `tele`, `foto`, ...).
- **Output**: member status.
- **Examples**: `mini` independent: `miniszoknya`, `miniszoknya-viselet`; `anti` + independent `alkoholista`: `antialkoholista`, `antialkoholista-klub`; `bio` + independent `technológia`: `biotechnológia`, `biotechnológia-ipar`; inferred: `foto#gráfia` (neither part independent) = one member.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: independence of each element (lexicon: is the element or its partner a free word)
- **Interactions**: `ORT-SPLIT-014` (pseudo-compounds in hyphenation); `DER-PREF-006` (member flag). AkH11 counted only foreign elements independently used in Hungarian (`centi-`, `deci-`, `deka-`, `extra-`: `centiméter-beosztású`, `extraprofit-elmélet`); the independent-second-element criterion is new in AkH12.
- **Sources**: [AkH12 §139]; [AkH11 §138]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-63-006 The hyphen goes at the main boundary
- **Statement**: The hyphen splits the compound into its two immediate constituents.
- **Conditions**: compound qualifying for a hyphen by `ORT-63-002`.
- **Output**: one hyphen between the top-level constituents; lower-level boundaries solid.
- **Examples**: `[csapat#zászló]#avatás` -> `csapatzászló-avatás`; `tornász#[csapat#bajnokság]` -> `tornász-csapatbajnokság`; `[munka#erő]#[nyilván#tartás]` -> `munkaerő-nyilvántartás`.
- **Exceptions**: -
- **Predictability**: `rule` given bracketing; `lexical`/semantic for bracketing
- **Lexical input**: binary bracketing; heuristic: prefer the split where both sides are lexicon words and the left side is a lexicalised compound (`munkaerő`, `csapatzászló`); ambiguity must be flagged, not guessed
- **Interactions**: `DER-COMP`
- **Sources**: [AkH12 §139]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-63-007 Exceptions and scope limits
- **Statement**: Some long multiple compounds are solid by lexical exception (AkH12 §139 lists four and ends with "etc."). Scope limits inferred from other paragraphs, not stated in §139: multi-element Hungarian settlement names are solid at any length (§174 b)), spelled-out numerals follow `ORT-NUM-010` (§291 a)), two-member compounds follow `ORT-63-001`. The rule is not applied to proper-name + common-noun compounds (`Szent Margit-kolostor-romok` keeps both hyphens) nor to multi-word foreign expressions (`via ferrata túra` stays apart) `[ENYELV 2015-06-12]`, `[ENYELV 2020-11-06]` (`OQ-07-025` resolved 2026-09-16).
- **Conditions**: lexicon exceptions; entity type.
- **Output**: solid.
- **Examples**: `akasztófáravaló`, `fizetővendéglátás`, `pénzügyminisztérium`, `valószínűségszámítás` (AkH11: `valószínűség-számítás`); proper names: `Szépkenyerűszentmárton`, `Garamszentbenedek`; numerals: `ezerkilencszáznegyvennyolc`.
- **Exceptions**: -
- **Predictability**: `lexical` (open "etc." list)
- **Lexical input**: exception lexicon; named-entity flag
- **Interactions**: `ORT-TOG-047`; emMorph also exempts settlement names
- **Sources**: [AkH12 §139, §174 b)]; [HUWIKI-AKH12CH]
- **Validation**: goldens; attestation (`valószínűségszámítás` 6)
- **Confidence**: `normative` (listed exceptions); `consensus` (scope limits)

### ORT-63-008 Compound already hyphenated for another reason: every further member gets a hyphen
- **Statement**: If the first part of a multiple compound is itself hyphenated because of a proper name, a letter acronym, or three identical consonant letters, a further member is attached with another hyphen (two hyphens); the syllable count is irrelevant (inferred: `sakk-készlet-gyűjtemény` has 6 syllables).
- **Conditions**: left constituent contains a non-6:3 hyphen.
- **Output**: `X-Y-Z`.
- **Examples**: `Kossuth-nóta-éneklés`, `Nobel-díj-átadás`, `Oscar-díj-átadás`, `Achilles-ín-szakadás`, `Munkácsy-kép-restaurálás`, `Afrika-kutató-találkozó`, `Budapest-térkép-vásár`, `Petőfi-szobor-avatás`, `C-vitamin-adagolás`, `tb-járulék-csökkentés`, `sakk-készlet-gyűjtemény`, `tarokk-kártya-játékos`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: reason of the existing hyphen
- **Interactions**: `ORT-HYPH-001`, `ORT-HYPH-003`, `ORT-HYPH-005`, `ORT-63-009`; AkH11 §139 a) excluded two hyphens only in common-noun subordinative compounds, consistent with this rule
- **Sources**: [AkH12 §112, §140, §166]; [AkH11 §139 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-63-009 6:3-hyphenated compound + a further hyphenated member: first hyphen removed
- **Statement**: When a further member that itself requires a hyphen is attached to a word already hyphenated (by the 6:3 rule or by `ORT-TOG-045`), the earlier hyphen is dropped and that part written solid, regardless of syllable count; a paraphrase is usually clearer.
- **Conditions**: left constituent contains a common-word structural hyphen (6:3 or phrase + member), not a `ORT-63-008` hyphen (proper name, acronym, three letters).
- **Output**: `XY-Z`.
- **Examples**: `anyagcsere-vizsgálat` + `kérés` -> `anyagcserevizsgálat-kérés`; `békeszerződés-tervezet` + `kidolgozás` -> `békeszerződéstervezet-kidolgozás`; `kerekasztal-konferencia` + `rendezés` -> `kerekasztalkonferencia-rendezés`; paraphrases: `anyagcsere-vizsgálat kérése`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: reason of the existing hyphen
- **Interactions**: `ORT-63-008`, `ORT-TOG-045`
- **Sources**: [AkH12 §141 a), §263 b)]; [AkH11 §139 a)] (same rule)
- **Validation**: goldens
- **Confidence**: `normative`

## HYPH: the hyphen

The hyphen (kötőjel) is U+002D in running text (U+2010 acceptable in typesetting); the en dash (nagykötőjel) U+2013 is a different sign (`ORT-HYPH-013`, `PUN-DASH`). Line-end hyphenation is `ORT-SPLIT`.

### ORT-HYPH-001 Three identical consonant letters at a compound boundary
- **Statement**: If the first member of a compound ends in a written long consonant `GG` and the second begins with `G`, all letters are kept and the members are joined by a hyphen.
- **Conditions**: `#` boundary; left ends in `GG` (single-letter geminate or simplified digraph geminate), right begins with `G`.
- **Output**: left-right.
- **Examples**: `balett-táncos`, `sakk-kör`, `váll-lövés`, `ott-tartózkodás`, `hossz-szelvény`, `dzsessz-szerzemény`, `expressz-szerű`, `puff-féle`, `rossz-szívű` ('wicked'), `tarokk-kártya`.
- **Exceptions**: two identical digraphs without a third (`jegygyűrű`, `díszszázad`): solid (`ORT-ASSIM-016`).
- **Predictability**: `rule`
- **Lexical input**: compound boundary
- **Interactions**: `ORT-63-008`, `ORT-SPLIT-017`; `-szerű`/`-féle` count as compound members here
- **Sources**: [AkH12 §61, §93, §263 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-002 Three identical consonant letters when suffixing a proper name
- **Statement**: A proper name ending in a doubled consonant letter takes a same-letter-initial suffix (including assimilated `-val/-vá`) with a hyphen, preserving the name; see `ORT-NAME-012` and `ORT-FOR-013`.
- **Conditions**: proper name (any type, Hungarian or foreign, including titles and brand blends, and also lowercase elements of names) ending in `GG` + suffix starting with `G`.
- **Output**: Name-suffix.
- **Examples**: `Széll-lel`, `Kiss-sel`, `Papp-pá`, `Wittmann-né`, `Mariann-nal`, `Bonn-nál`, `Tallinn-nak`, `Scott-tal`, `Elzett-tel`, `Knorr-ral`, `Blikk-kel`, `Keravill-lal`.
- **Exceptions**: common words simplify (`ORT-ASSIM-017`).
- **Predictability**: `rule`
- **Lexical input**: proper-name flag
- **Interactions**: `ORT-NAME-012`, `ORT-FOR-013`, `ORT-ABBR-016`
- **Sources**: [AkH12 §93, §152 c), §160 c), §217 c), §263 a), §265 a), §288 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-003 Proper name + common noun or adjective in an unmarked compound
- **Statement**: A proper name forming an unmarked (subordinative) compound with a common noun or adjective is joined to it by a hyphen; the name keeps its spelling and capitals; for a multiword name the hyphen follows the last word only; two names in relation are joined by an en dash and the common noun by a hyphen; further members add hyphens.
- **Conditions**: `Name # N/Adj` where the relation is compound-forming (possessive, 'about', 'by', 'of'), not attributive (`ORT-TOG-035`).
- **Output**: Name-noun; First Last-noun; Name1–Name2-noun.
- **Examples**: `Afrika-kutató`, `Budapest-térkép`, `Petőfi-szobor`, `Herkules-erejű`, `Ady-vers`, `Balassi-strófa`, `Celsius-fok`, `Kodály-módszer`, `Deák-párt`, `Szemere-kormány`, `Arany János-idézet`, `Bartók Béla-emlékév`, `Szent Benedek-rend`, `Szent István-nap`, `Geiger–Müller-számlálócső`, `Hadrovics–Gáldi-szótár`, `Kazinczy-verseny`, `József Attila-díj`, `Baudelaire-vers`, `Loire-part`, `Shakespeare-dráma`, `Flaubert-mű`, `Itália-szerte`, `Temze-part`.
- **Exceptions**: name as attribute or with an explanatory common noun: apart (`a Nagy család`, `Bodri kutya`, `ORT-TOG-035`); dynasty words `ház`, `család`, `dinasztia` also take the hyphen (`Árpád-ház`, `ORT-NAME-014`); name + noun that became a common noun: solid lowercase (`ádámcsutka`, `ORT-NAME-016`); award/geographic/institution names have their own rules (`ORT-NAME`).
- **Predictability**: `rule` given the relation type
- **Lexical input**: relation type (compound vs attributive), named-entity span
- **Interactions**: `ORT-63-008`, `ORT-NAME-013`, `ORT-NAME-015`, `ORT-CAP`
- **Sources**: [AkH12 §112, §126, §166, §170, §211, §217 d), §263 b), e)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-004 -fajta, -féle, -forma, -szerű after proper names and acronyms
- **Statement**: These elements are joined by a hyphen to proper names (after the last word of a multiword name, and to titles and institution names in unchanged form) and to acronyms; an omitted shared element is marked by a hyphen.
- **Conditions**: proper name or acronym + `-fajta`, `-féle`, `-forma`, `-szerű`.
- **Output**: `Name-féle`.
- **Examples**: `Illyés-fajta`, `Eötvös-féle`, `Wesselényi-féle`, `Szent-Györgyi-féle`, `Kossuth-forma`, `Győr-forma`, `Petőfi-szerű`, `Munkácsy-szerű`, `OTP-szerű`, `Jedlik Ányos-féle`, `Madách Színház-féle`, `Paál László-szerű`, `Szinyei Merse Pál-szerű`, `Tömörkény- és Gárdonyi-szerű`, `Kritika-féle`, `Élet és Tudomány-szerű`, `Coca-Cola-szerű`.
- **Exceptions**: common words: solid (`ORT-TOG-041`), but digits take a hyphen (`10-féle`); foreign common words solid unless silent-final (`campanileszerű`).
- **Predictability**: `rule`
- **Lexical input**: proper-name/acronym flag
- **Interactions**: `ORT-TOG-041`, `ORT-SUFX-002` (no lengthening)
- **Sources**: [AkH12 §132, §133, §169, §200 b), §216 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-005 Acronym or proper-name blend + compound member
- **Statement**: A compound member (noun) is joined with a hyphen to letter acronyms (upper- or lowercase) and to proper-name blends; common-noun blends and letter names spelled out are solid with the member.
- **Conditions**: acronym (`ORT-ABBR-011`) or proper blend (`ORT-ABBR-012`) + noun member.
- **Output**: `ACR-noun`; common blend: solid.
- **Examples**: `EU-csatlakozás`, `KSH-kimutatás`, `UNESCO-jogszabály`, `MTI-hír`, `OTP-kölcsön`, `UEFA-torna`, `TDK-dolgozat`, `URH-adás`, `tb-járulék`, `tv-műsor`, `tv-közvetítés`, `Alitalia-iroda`, `Orex-üzlet`, `PIN-kód` (dictionary, per HUWIKI-AKH12CH); letter or letter-digit symbols likewise: `C-vitamin`, `B12-vitamin` (§294); solid: `radarállomás`, `trafótekercselés`, `tébécészakorvos`, `tévéközvetítés`, `tévéműsor`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: acronym type
- **Interactions**: `ORT-63-008`, `ORT-ABBR-017`; AkH12 extended the hyphen to explanatory members (`PIN kód` -> `PIN-kód`)
- **Sources**: [AkH12 §113, §140, §289, §294]; [HUWIKI-AKH12CH]
- **Validation**: goldens; attestation (`tévéműsor` 4)
- **Confidence**: `normative`

### ORT-HYPH-006 Abbreviation or symbol + full word
- **Statement**: An abbreviation or symbol forming a compound with a full word is joined by a hyphen, the abbreviation keeping its period.
- **Conditions**: abbreviation/unit symbol + noun member.
- **Output**: abbr-noun, abbr.-noun.
- **Examples**: `mm-beosztás`, `fszla.-kivonat`; not in AkH12: `km-kő`, `%-érték`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: abbreviation detection
- **Interactions**: `ORT-ABBR-007`
- **Sources**: [AkH12 §283]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-007 Hyphen for an omitted shared member
- **Statement**: In a sequence of compounds sharing the last (or first) member, only the last (or first) compound is written in full and the omitted member is indicated by a hyphen on the others.
- **Conditions**: coordinated compounds with a common member.
- **Output**: `X- és YZ`; `XY és -Z`.
- **Examples**: `bel- és külkereskedelem`, `tej-, zöldség- és gyümölcsfelhozatal`, `bortermelő és -értékesítő bt.`, `gépgyártó, -szerelő és -javító üzem`, `bükk-, mogyoró- és nyírfafélék`, `Tömörkény- és Gárdonyi-szerű`, `Szilas Menti Lakásépítő és -fenntartó Szövetkezet`, `Ganz Danubius Hajó- és Darugyár`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PUN-DASH-002` (spacing: hyphen attached to the fragment, space after); tokenisation must reconstruct the full compounds
- **Sources**: [AkH12 §132, §133, §193, §240 k), §263 c)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-008 Reduplications, coordinative compounds, twin words
- **Statement**: Hyphens within reduplications, loose coordinative compounds, twin words and repeated or opposite verbal prefixes are governed by the `TOG` rules listed here.
- **Conditions**: see referenced rules.
- **Output**: see referenced rules.
- **Examples**: `egy-egy`, `néha-néha`, `réges-régi`, `süt-főz`, `sírva-nevetve`, `édes-bús`, `szoba-konyhás`, `dimbes-dombos`, `csihi-puhi`, `Anna-Panna`, `gizes-gazos`, `meg-megállt`, `ki-be ugrál`, `vissza-visszatérés`, `le-föl sétálás`, `anyagcserevizsgálat-kérés`, `hidegvíz-csap`, `szövő-fonó ipar`; poetic occasional compounds `bogáncs-szívem`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: see referenced rules
- **Interactions**: `ORT-TOG-004`, `ORT-TOG-006`, `ORT-TOG-008` to `ORT-TOG-014`, `ORT-TOG-028`, `ORT-TOG-045`, `ORT-TOG-046`, `ORT-63-009`, `ORT-PRIN-013` (poetic hyphenation), `ORT-FOR-015` (`lime-likőr`), `ORT-HYPH-003`
- **Sources**: [AkH12 §263 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-009 The interrogative particle -e
- **Statement**: The yes/no question particle `-e` is joined with a hyphen to the word it follows (in practice mostly the finite verb, but any focused constituent: `Jobbra-e vagy balra?`).
- **Conditions**: clitic `e` in (embedded) yes/no questions.
- **Output**: word-e.
- **Examples**: `írjunk-e`, `tudod-e`, `megvan-e még`, `Meghaltál-e?`.
- **Exceptions**: the verb suffix-final `e` (`vinné`, `kérje`) is not the particle.
- **Predictability**: `rule`
- **Lexical input**: clitic identification (interrogative context)
- **Interactions**: `SYN-Q`, `PUN-DASH-003`; tokenizers split `-e` (`spaCy` exceptions)
- **Sources**: [AkH12 §263 h)]; [AkH11 §262 h)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-010 Approximate or alternative numbers; distributive repetition
- **Statement**: Two numbers (in letters or digits) expressing approximation or alternatives are joined by a hyphen; so are distributive repetitions of a numeral. Ranges (from-to) use an en dash (`ORT-HYPH-013`).
- **Conditions**: `NUM1-NUM2` with 'or so' meaning; `NUM-NUM` distributive.
- **Output**: hyphen.
- **Examples**: `egy-két (ember)`, `nyolc-tíz (napra)`, `5-6 (darab)`, `10-12 (éves)`; `2-2 (almát)`, `húsz-húsz (deka)`.
- **Exceptions**: ranges `1–4. osztály` (en dash).
- **Predictability**: `rule` (form); meaning (approximation vs range) needs context: consecutive small numbers with a count noun -> approximation; ordinals/pages/years -> range
- **Lexical input**: semantic reading
- **Interactions**: `ORT-HYPH-013`, `ORT-NUM`; the distributive repetition sub-point is new in AkH12 (absent from AkH11 §262 i))
- **Sources**: [AkH12 §263 i)]; [AkH11 §262 i)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-011 Hyphens inside proper names
- **Statement**: Double family names (hyphen required by civil registry law) and several types of geographic, astronomical and award names contain hyphens by the `NAME` rules.
- **Conditions**: see referenced rules.
- **Output**: see referenced rules.
- **Examples**: `Endrődy-Somogyi`, `Konkoly-Thege`, `Arany-patak`, `Csepel-sziget`, `Vas Mihály-hegy`, `Holt-Tisza`, `Nagy-New York`, `Kál-Kápolna`, `Érd-Ófalu`, `Rohonci-Arany-patak`, `Arany-patak-völgy`, `Magellán-felhő`, `Kossuth-díj`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: see referenced rules
- **Interactions**: `ORT-NAME-007`, `ORT-NAME-022` to `ORT-NAME-025`, `ORT-NAME-031`, `ORT-NAME-036`
- **Sources**: [AkH12 §156, §175-§177, §185, §195 a), §263 f)-g)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-012 Spelled-out numerals above 2000
- **Statement**: Hyphens separate the thousand groups of spelled-out numerals above 2000 when the number continues after the thousands (`ORT-NUM-010`); years are excepted and written solid.
- **Conditions**: see `ORT-NUM-010`.
- **Output**: `kétezer-egy`.
- **Examples**: `ötezer-nyolcszáztíz`, `tizenegyezer-huszonnyolc`, `hárommillió-ötszáznegyvenezer-kétszázharmincnyolc`; solid: `ötvenháromezer`, `hatvankétmillió`.
- **Exceptions**: years: `kétezertizennégy` (§291 a)); up to 2000 solid: `ezerhétszázhetvenkettő`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-NUM-010`
- **Sources**: [AkH12 §263 d), §291 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-013 En dash groups and their suffixes
- **Statement**: The en dash (unspaced) joins names of peoples, languages, subjects in a relation, two or more proper names in a relation, and from-to pairs of words or numbers; suffixes and derivational `-i` attach directly, without hyphen, to the last member (in from-to pairs possibly to both members: `tavasszal–nyáron`, `tavaszi–nyári`), and a derived adjective of a geographic from-to pair is lowercase; digit members still take a hyphen before suffixes; complex members may take a spaced en dash (optional in AkH12; "exceptionally" in AkH11).
- **Conditions**: `X–Y` groups.
- **Output**: `X–Y+suffix`.
- **Examples**: `angol–magyar (szótár)`, `francia–spanyol (határ)`, `német–magyar (szak)`, `biológia–kémia (tanár)`, `olasz–német (mérkőzés)`, `Osztrák–Magyar Monarchia`, `Magyar–Finn Társaság`, `Hadrovics–Gáldi`, `Újpest–Ferencváros`, `Budapest–Bécs`, `1983–1984. évi`, `kelet–nyugat irányban`; suffixed: `az angol–brazilt`, `a magyar–angolban`, `német–franciás`, `a Hadrovics–Gáldiban`, `a Honvéd–Haladáson`, `Budapest–Bécsen át`, `az első–negyedikben`, `tavasszal–nyáron`, `budapest–bécsi`, `kelet–nyugati`, `június–júliusi`; digits: `a 128–9.-en`, `az I–IV.-ben`, `1848–49-ben`, `1914–18-as`; spaced: `2011. április 25. – 2012. május 13.`, `Rácz Endre – Szemere Gyula: ...`, `Szojuz–24 – Szaljut–5 űrkomplexum`.
- **Exceptions**: geographic combinations with a geographic common noun: hyphen before it (`Volga–Don-csatorna`, `Cseh–Morva-dombság`), `-i` on the unchanged form (`Volga–Don-csatornai`) except name associations like `cseh–morva-dombsági` (`ORT-NAME-027`).
- **Predictability**: `rule`
- **Lexical input**: relation reading (hyphen for coordinative unit vs en dash for relation between separate entities)
- **Interactions**: `PUN-DASH` (spacing, character), `ORT-NAME-027`, `ORT-NUM-018`
- **Sources**: [AkH12 §178, §264]; [AkH11 §263]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-014 Suffixes on titles
- **Statement**: Case and plural suffixes attach directly to titles of works (with normal lengthening); if the title ends in a case suffix or in a punctuation mark, the suffix takes a hyphen; compound members and `-beli` always attach with a hyphen to the unchanged title; further suffixing of an already suffixed title should be avoided by paraphrase.
- **Conditions**: title of a work, periodical or programme (`ORT-NAME-040`).
- **Output**: direct / hyphen per condition.
- **Examples**: direct: `az Autóséleté`, `a Kritikában`, `a Geodézia és Kartográfiát`, `a Számadásnak`, `a Nemzeti dalt`, `Arany Toldijában`, `Siralomháznak`; hyphen after final case suffix: `a Légy jó mindhalálig-ot`, `a Szülőföldemen-ben`; after punctuation: `a Szerelem?-nek`, `„Az arany ember”-e`; members: `Magyar Nyelvőr-évfolyam`, `Valóság-beli`, `Orvosi Hetilap-beli`, `Odüsszeia-beli` (no lengthening), `a Kincskereső kisködmön-beli`; paraphrase: `Petőfinek a Szülőföldemen című versében`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: title span; whether the title's last word ends in a case suffix (morphological analysis of the last word)
- **Interactions**: `ORT-HYPH-015`, `ORT-CAP-021`, `ORT-CAP-022`
- **Sources**: [AkH12 §200]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-015 Suffixes after quotation marks and on web addresses
- **Statement**: A suffix added to a title in quotation marks is attached with a hyphen after the closing quotation mark; suffixes on internet addresses are attached with a hyphen.
- **Conditions**: quoted title; domain name or URL.
- **Output**: ”-suffix; domain-suffix.
- **Examples**: `a „Magyar szólások és közmondások”-ban`; `a hvg.hu-n`, `az index.hu-t`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: URL detection; suffix form by pronunciation of the address
- **Interactions**: `PUN-QUOT`, `ORT-HYPH-014`
- **Sources**: [AkH12 §200 a), §273]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-016 Letter-digit designations
- **Statement**: Combinations of letters and digits (vitamins, roads, metro lines, IDs, plates, types) are written in several ways with no substantive difference; individual systems may deviate from general spelling practice. Suffixes after a final digit take a hyphen by `ORT-NUM-002` (`M7-es`); §294 itself says nothing about suffixes.
- **Conditions**: alphanumeric designators.
- **Output**: keep the source form; suffix with hyphen.
- **Examples**: `B12-vitamin`, `123456HA`, `M7-es autópálya`, `M2 metróvonal`, `H1N1`, `ABC-123`, `TU–154`, `Apollo–11`, `Forma–1`, `123456 KL`, `Yamaha XQ125`, `Kovács II. Béla`, `NB I.`, `IV/3-as tábla`, `4/a osztály`.
- **Exceptions**: -
- **Predictability**: `rule` (no normalisation)
- **Lexical input**: none
- **Interactions**: `ORT-NUM-008`, `ORT-NUM-002`, `REP` (do not "repair"); AkH11 §263 d) required the en dash for type designations (`T–34`, `TU–154-gyel`)
- **Sources**: [AkH12 §294]; [AkH11 §263 d)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-017 Hyphenated derivational suffix after multiword names
- **Statement**: Derivational suffixes (`-i`, `-s`, `-es`, `-beli`, `-ista` ...) attach with a hyphen to the last word of personal and geographic names (and titles) written as several separate words; the capitals are kept.
- **Conditions**: name span of two or more space-separated words + derivational suffix.
- **Output**: `First Last-i`.
- **Examples**: `Arany János-i`, `Táncsics Mihály-i`, `Eötvös József-i`, `Csokonai Vitéz Mihály-os`, `Ilosvai Selymes Péter-es`, `Leonardo da Vinci-s`, `Walter Scott-os`, `Anatole France-i`, `Victor Hugó-i`, `New York-i`, `Kuala Lumpur-i`, `Karlovy Vary-i`, `Frankfurt am Main-i`, `Hriszto Botev-i`, `Velikije Luki-i`, `Magyar Nemzet-beli`, `Nagy Medve-beli`.
- **Exceptions**: inflectional suffixes attach directly (`Táncsics Mihálynak`, `New Yorkban`); single-element, solid or hyphenated personal names take `-i`, `-s`, `-ista`, `-izmus` directly and lowercase (`móriczi`, `horváth-pappos`, `ORT-NAME-010`); separately written state, state-part, public place, explanatory and institution names keep separate writing with `-i` (`-beli`) on the last word, no hyphen, capitals per type (`francia köztársasági`, `Baranya megyei`, `Váci utcai`, `Mátra hegységi`, `ORT-NAME-028` to `ORT-NAME-030`, `ORT-NAME-032`, `ORT-NAME-034`).
- **Predictability**: `rule`
- **Lexical input**: name type and span
- **Interactions**: `ORT-NAME-010`, `ORT-FOR-014`
- **Sources**: [AkH12 §152 c), §159, §161, §179-§182, §184, §200 b), §217 b), §222, §265 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-018 Hyphen is kept in derivatives of hyphenated compounds
- **Statement**: Adjectives derived (usually with `-i`, `-beli`, `-s`, `-ú/-ű`) from a hyphenated name + noun compound keep the hyphen; capitalisation follows `ORT-CAP-017`.
- **Conditions**: hyphenated compound + derivational suffix.
- **Output**: hyphen kept.
- **Examples**: `Kossuth-díjas`, `Árpád-kori`, `Jókai-regénybeli`, `Zrínyi-soros`, `Mária-arcú`, `Kodály-módszerű`, `Jászai Mari-díjas`, `Konkoly-Thege-érmes`, `Hadrovics–Gáldi-szótárbeli`, `Balassi-strófás`, `Androméda-ködbeli`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-CAP-017`, `ORT-NAME-015`
- **Sources**: [AkH12 §167, §185, §195 a), §263 e)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-019 Hyphen with digits, symbols, abbreviations, acronyms, silent letters
- **Statement**: Suffixes are hyphenated to digits, punctuation/symbol signs, abbreviations, letter acronyms and to foreign words ending in a silent letter or unusual letter group, always in the form required by the spoken word (harmony, assimilation, linking vowel written out).
- **Conditions**: see `ORT-NUM-002`, `ORT-ABBR-009`, `ORT-ABBR-014`, `ORT-FOR-012`.
- **Output**: X-suffix.
- **Examples**: `4-et`, `12-t`, `19-szer`, `32-en` (= `harmincketten`) vs `32-n` (= `harminckettőn`), `67-es`, `a IV.-be`, `80%-os`, `6%-kal`, `3°-kal`, `a 15. §-ban`, `Bp.-en`, `az MTA-tól`, `100 °C-on`, `16.25-kor`, `guillotine-t`, `guillotine-nal`, `Glasgow-ban`, `Montreux-ig`, `Rousseau-nak`, `voltaire-es`, `ploieşti-i`.
- **Exceptions**: blends and word-read common acronyms: directly (`ORT-ABBR-016`); `Anjouk` (lexical exception to the silent-letter hyphen).
- **Predictability**: `rule`
- **Lexical input**: spoken form
- **Interactions**: `ORT-NUM-002`, `ORT-ABBR-009`, `ORT-ABBR-014`, `ORT-FOR-012`
- **Sources**: [AkH12 §217 a), §265 c)-d), §275, §282, §288 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-HYPH-020 Decision procedure: hyphen before a suffix
- **Statement**: A suffix (inflectional or derivational) attaches directly unless one of the following holds, checked in order: (1) the base is written with digits (cardinal, ordinal with period, date day/year, time), a symbol (`%`, `§`, `°`, `°C`, currency, unit), or an abbreviation -> hyphen; (2) the base is a letter acronym (`ORT-ABBR-011`) -> hyphen; (3) the base is a title ending in a case suffix or punctuation, a quoted title, or a web address -> hyphen; (4) the base is a proper name (or brand blend, or a lowercase name element) ending in `GG` and the suffix starts with `G` -> hyphen; (5) the base is a foreign-spelled word ending in a silent letter or unusual letter group (`ORT-FOR-012`) -> hyphen (except `Anjouk`); (6) the suffix is derivational and the base is a personal or geographic name of several separate words (not a state, state-part, public place, explanatory or institution name, `ORT-HYPH-017`), or `-beli` on a title -> hyphen on the last word; (7) otherwise direct, including en dash groups ending in a word (`ORT-HYPH-013`). For compound members, `ORT-HYPH-001` to `ORT-HYPH-006` and `ORT-FOR-015` apply instead.
- **Conditions**: every suffix attachment.
- **Output**: base-suffix or basesuffix; suffix allomorph always from the pronounced base.
- **Examples**: (1) `2014-ben`, `3.-nak`, `10:35-kor`, `Ft-ot`; (2) `ENSZ-szel`, `tv-t`; (3) `Szülőföldemen-ben`; (4) `Papp-pal`; (5) `Peugeot-t`; (6) `New York-i`; (7) `Petőfiről`, `Stockholmnál`, `Orexszel`, `áfát`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: token type (digits, symbol, abbreviation, acronym type), proper-name flag and span, silent-final flag, spoken form
- **Interactions**: `ORT-SUFX-005`, `ORT-ASSIM-007`, `ORT-ABBR`, `ORT-NUM`, `ORT-FOR`, `ORT-NAME`
- **Sources**: [AkH12 §93, §160 c), §161, §179-§182, §200, §217, §264, §265, §273, §275, §282, §288, §292, §298, §300]
- **Validation**: goldens
- **Confidence**: `normative`

## CAP: capitalisation

### ORT-CAP-001 Common words lowercase, proper names capitalised
- **Statement**: Common words are written with a lowercase initial except in defined positions; proper names always have a capital initial. For some expressions the common/proper status is unclear and the initial may vary.
- **Conditions**: every word not in a capitalising position (`ORT-CAP-007` to `ORT-CAP-013`); the norm names sentence-internal position, dictionary headwords and word lists as typical lowercase contexts.
- **Output**: lowercase for common words (all word classes); capital for every proper name (per its type rules).
- **Examples**: `szeret`, `elnök`, `ország`, `drága`, `három`, `bármennyi`, `itt`, `jóllehet`, `nahát`.
- **Exceptions**: `ORT-CAP-007` to `ORT-CAP-013`.
- **Predictability**: `rule` (given named-entity status)
- **Lexical input**: proper-name flag / NER span and type
- **Interactions**: `ORT-CAP-014`, `WS-CASE`
- **Sources**: [AkH12 §142-§143]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-002 Accents are kept on capital letters
- **Statement**: Capital vowel letters carry the same accents as lowercase ones; capital `I` has no dot, but capital `Í` carries its acute accent.
- **Conditions**: uppercase or titlecase output.
- **Output**: `Ö Ü Á Ó Ő É Ú Ű Í`; `I`.
- **Examples**: `Ödön`, `Ürményi`, `Ázsia`, `Óbuda`, `Őriszentpéter`, `Ilona`, `Izland`, `Írország`, `Írott-kő`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-CASE`, `REP` (accent loss in all-caps sources)
- **Sources**: [AkH12 §5]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-003 Capitalising digraphs
- **Statement**: With an initial capital only the first letter of a digraph or trigraph is uppercase; in all-capitals text and in letter acronyms every letter of the digraph is uppercase.
- **Conditions**: titlecase vs all-caps rendering.
- **Output**: titlecase `Cs Dz Dzs Gy Ly Ny Sz Ty Zs`; all caps `CS DZ DZS GY LY NY SZ TY ZS`.
- **Examples**: `Csoma`, `Dzsingisz`, `Szolnok`, `Zsolt`; `ENSZ`, `GYSEV`, `CSOMAGMEGŐRZŐ`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: grapheme segmentation
- **Interactions**: `WS-CASE` (Unicode titlecase is per code point: `Sz` needs grapheme-aware titlecasing), `ORT-ABBR-011`
- **Sources**: [AkH12 §3, §8, §10, §285 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-004 Lofty or emotive words are lowercase
- **Statement**: Words with elevated or pleasant content are generally lowercase despite their emotional charge.
- **Conditions**: common nouns such as `béke`, `család`, `édesanya`, `élet`, `haza`, `szabadság`, `szerelem`, `tavasz`.
- **Output**: lowercase.
- **Examples**: as Conditions.
- **Exceptions**: official salutations `ORT-CAP-010`; official documents and poetry `ORT-CAP-011`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-CAP-011`
- **Sources**: [AkH12 §143]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-005 Holidays, memorial days, periods, historical events are lowercase
- **Statement**: Names of holidays, commemorative days, periods and historical events are written lowercase (proper names inside them keep their capitals).
- **Conditions**: semantic class holiday/event/period.
- **Output**: lowercase except embedded proper names.
- **Examples**: `a magyar kultúra napja`, `május elseje`, `március tizenötödike`, `nemzeti ünnep`, `a víz világnapja`, `anyák napja`, `farsang`, `hanuka`, `húsvét`, `karácsony`, `mindenszentek`, `nagyböjt`, `ramadán`, `a bécsi kongresszus`, `a francia forradalom`, `a honfoglalás`, `a mohácsi vész`, `az ónodi országgyűlés`, `a szabadságharc`; embedded names: `Szent István-nap` (hyphenated name compound), `Nagyboldogasszony napja`, `Szent Mihály hava`.
- **Exceptions**: events and programmes: `ORT-CAP-025` (capitals acceptable as institution-like names).
- **Predictability**: `rule`
- **Lexical input**: semantic class
- **Interactions**: `ORT-CAP-025`, `ORT-NAME-014` (`Szent István-nap`); NER training data must not tag these as proper names by case
- **Sources**: [AkH12 §144, §164, §166, §191]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-006 Peoples, ethnic groups, languages, language families, religions are lowercase
- **Statement**: Names of peoples, ethnic groups and language families, and words for belonging to a religion or denomination are lowercase. Language names (`magyar`, `orosz`) are lowercase as well (inference: AkH12 §145 does not name languages; they coincide with the ethnonym forms and are lowercase throughout AkH12 examples).
- **Conditions**: ethnonyms, glottonyms, religious adherents.
- **Output**: lowercase.
- **Examples**: `észt`, `kínai`, `magyar`, `orosz`, `szlovák`, `palóc`, `sváb`, `szász`, `székely`, `indián`, `finnugor`, `germán`, `román`, `nyugati szláv`, `urál-altaji`; `buddhista`, `katolikus`, `muszlim`, `református`, `sintó`.
- **Exceptions**: as members of institution and state names (`Magyar Tudományos Akadémia`, `Francia Köztársaság`), which are capitalised by their own rules.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-NAME-028`, `ORT-NAME-034`
- **Sources**: [AkH12 §145, §179, §187]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-007 Capital at the start of a text and of each sentence
- **Statement**: The first word of a text and of each new sentence after a period, question mark or exclamation mark is capitalised; after a question or exclamation mark inserted inside a sentence, the sentence continues in lowercase.
- **Conditions**: sentence-initial position (sentence segmentation from `PUN`/`SYN`).
- **Output**: capital initial (digraph rule `ORT-CAP-003`).
- **Examples**: `Mi volt a diákok vétsége? Hogyan védelmezték magukat?`; inserted: `Most veszem észre, hohó! mind, amit mondtam, hazugság`.
- **Exceptions**: a lowercase abbreviation at sentence start is capitalised (`Kb. húszan leszünk.`, `Csüt. délben érkezünk.`); standardised symbol forms (`Ft`, `m²`) are never altered, not even in all-capitals text (`ORT-ABBR-003`); not changing them at sentence start is inference from that clause. A sentence beginning with a digit keeps the digit (inference; not stated in AkH12).
- **Predictability**: `rule`
- **Lexical input**: sentence boundaries
- **Interactions**: `PUN-END`, `SYN` segmentation, `ORT-CAP-008`
- **Sources**: [AkH12 §146, §251, §277 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-008 Capitalisation after a colon
- **Statement**: After a colon the text continues in lowercase unless a capitalising case applies: (a) the colon closes a common attention-raising sentence followed by several separate sentences (each, including the first, is capitalised); (b) a verbatim quotation introduced by the colon (its first word is generally capitalised, but lowercase when the quoted original has lowercase there); (c) proper names. An interrupted quoting sentence continues in lowercase after the quotation.
- **Conditions**: token after `:`.
- **Output**: lowercase by default (inference from the §146 wording "in certain cases" and the §246 example); capital in cases (a)-(c).
- **Examples**: `Közös a célunk: mindig többet ... akarunk.`; (a) `A nyelvújításnak több oka volt: Nem tudtuk ... Irodalmunknak ... Tiltakozást ...`; (b) `Örök kérdést tesz fel Vörösmarty: „Mi dolgunk a világon?”`; lowercase original: `ekképp jeleníti meg Babits: „már hullnak ...”`; interrupted: `Madách szavait: „Mondottam, ember: ...” ma is gyakran idézzük`.
- **Exceptions**: cited examples after a colon keep their own case: example words lowercase (`összetételek: műhold, számítástechnika`), an example sentence capitalised (`...: Magyarország legnagyobb tava a Balaton.`).
- **Predictability**: `rule` given structure; (a) requires detecting that more than one complete sentence follows before the paragraph end
- **Lexical input**: sentence segmentation after the colon; quotation marks
- **Interactions**: -
- **Sources**: [AkH12 §146, §246, §257 a), §268]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-009 Quotations
- **Statement**: Quotations and their internal parts generally keep the original case; a quotation introduced after a colon usually starts with a capital (`ORT-CAP-008`); a quoting clause after the quotation (joined by a dash) or inserted into it (between dashes) starts lowercase; a quotation woven into one's own sentence starts lowercase when its first word is a common word, even if the original was capitalised.
- **Conditions**: quoted material.
- **Output**: as stated.
- **Examples**: `„Ismerni a jót könnyebb, mint követni” – írta ... Kölcsey.`; `„A tudományos felfogás szerint – írja Kosztolányi – nincs ...”`; woven: `Aranynak a „nem hal meg az, ki milliókra költi ...” gondolata`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: quotation structure
- **Interactions**: `PUN-QUOT`
- **Sources**: [AkH12 §149, §257-§258]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-010 Salutations and forms of address
- **Statement**: In official documents and letters, besides the first word and proper names of a salutation, common nouns in it are customarily capitalised (adjectives, conjunctions not); elsewhere only the first word and proper names must be capitalised, the other words may be lowercase. In the body, nouns, pronouns and pronominal adverbs referring to the addressee are generally lowercase; a capital may express heightened respect, then only the first word of a multiword address is capitalised.
- **Conditions**: salutation line; references to the addressee.
- **Output**: official: `Tisztelt Nagy Úr!`, `Tisztelt Osztályvezető Úr!`, `Mélyen tisztelt Asszonyom és Uram!`; other: `Drága Ilonka néni!`, `Kedves barátom!`, `Édes jó anyám és apám!`; body default: `Légy szíves, barátom`, `önöket is érdeklik`, `neked`; respectful: `Szeretném, Apám, ...`, `az Önöké`, `Veled`, `Bocsássa meg, Igazgató úr`, `Államtitkár asszonyt`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule` (both variants valid in the body)
- **Lexical input**: genre (official letter), addressee reference
- **Interactions**: -
- **Sources**: [AkH12 §147, §254]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-011 Capitalised common nouns in official documents and poetry; verse lines
- **Statement**: Capitalised common nouns are not customary except in official documents (`Alperes`, `Felperes`, `Eladó`, `Vevő`, `Szerző`) and in poetry to heighten mood or draw attention to the concept (`a Szerelem`, `a Végzet`, `a Hatalom`); verse lines may also begin with a capital.
- **Conditions**: genre legal/official, poetry.
- **Output**: capital allowed.
- **Examples**: as Statement.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: genre
- **Interactions**: `ORT-CAP-004`, `ORT-NAME-035`
- **Sources**: [AkH12 §148, §188 d)]
- **Validation**: -
- **Confidence**: `normative`

### ORT-CAP-012 Signs, captions, headings; signature designations
- **Statement**: Direction signs and texts above or below tables, charts and figures begin with a capital; in multiword ones only the first word is capitalised; if the first word is a numeral in digits, the following common word is lowercase. Designations under signatures on documents are lowercase.
- **Conditions**: standalone sign/caption text; signature block.
- **Output**: `Bejárat`, `Pénztár`, `Kettős megállóhely`, `Nyilvános telefon`, `B porta`, `A légfék vázlatos rajza`, `2. metróvonal`, `5. ajtó`, `III. vágány`; under signature: `elnök`, `igazgató`, `a bizottság titkára`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: layout context
- **Interactions**: `PUN` (no period after headings §266)
- **Sources**: [AkH12 §150]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-013 Pre-name words: doktor, idősb, ifjabb, özvegy, gróf
- **Statement**: Common words and their abbreviations typically preceding names (`doktor/dr.`, `idősb/id.`, `ifjabb/ifj.`, `özvegy/özv.`, `gróf/gr.`, noble predicates like `felsővadászi`) are lowercase inside sentences and capitalised only in highlighted positions (signature, letter address, name lists in columns, business cards) and at sentence start.
- **Conditions**: position of the name.
- **Output**: `Ott volt dr. Mészáros Mihály is.`; highlighted: `Dr. Mészáros Mihály`, `Id. Kelemen László`, `Ifj. Jakab Imre`, `Özvegy Lukács Péterné`, `Gróf Széchenyi István`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: layout context
- **Interactions**: `ORT-ABBR-004`
- **Sources**: [AkH12 §151, §165]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-014 Proper names of all types are capitalised; homonymy with common words
- **Statement**: All proper-name types (personal, animal, object, geographic, astronomical, institution, brand, title, award names) have a capital initial, which distinguishes them from homonymous common words.
- **Conditions**: named entity.
- **Output**: capital initial per type rules (`ORT-NAME`).
- **Examples**: `Szőke` (family name) / `szőke`; `Ibolya` / `ibolya`; `Fakó` (horse) / `fakó`; `Mór` (town) / `mór` (people); `Hollófészek` (mountain) / `hollófészek`; `Rák` (constellation) / `rák`; `Kontroll` (company) / `kontroll`; `Parázs` (brand) / `parázs`; `Kortárs` (journal) / `kortárs`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: named-entity status
- **Interactions**: `ORT-NAME`
- **Sources**: [AkH12 §152 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-015 Marking the extent of multiword names
- **Statement**: The span of a multiword name is marked by writing it solid (second element lowercase even if itself a name), hyphenating it, or capitalising each element; linguistic structure (`utca`/`u.` is the last element, `című`/`c.` immediately follows the name) and layout (a title on its own line, not inside a sentence) also mark the extent.
- **Conditions**: multiword proper names.
- **Output**: per type: solid (`Hegyalja`, `Olaszország`, `Nagykőrös`, `Kispista`, `Tóthpál`, `Annamária`), hyphen (`Nemeskéri-Kiss`, `Velencei-tó`, `Észak-Afrika`), all capitals (`Kőrösi Csoma Sándor`, `Budapesti Műszaki és Gazdaságtudományi Egyetem`, `Magyar Nyelvőr`).
- **Examples**: `Kiss János altábornagy utca`; `az Ének a pesti ligetről című vers`; standalone titles: `A város peremén`, `Egri csillagok`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: name type
- **Interactions**: `ORT-NAME`, tokenisation of names
- **Sources**: [AkH12 §152 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-016 Mythological and religious names
- **Statement**: Mythological and religious words used as personal names are capitalised; `isten` is also frequently used as a common noun and is then lowercase (by `ORT-CAP-001`).
- **Conditions**: deity/saint name use.
- **Output**: `Allah`, `Boldogasszony`, `Hadúr`, `Isten`, `Jupiter`, `Ozirisz`, `Szűzanya`, `Zeusz`, `Szent Péter`, `Szűz Mária`; common (illustrative, not AkH12 examples): `a görög istenek`, `hál' istennek`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`/semantic (`Isten` as name vs `isten` as common noun)
- **Lexical input**: sense (monotheistic God as name vs generic deity)
- **Interactions**: -
- **Sources**: [AkH12 §157]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-017 Case of derived adjectives of names: summary
- **Statement**: Adjectives (and `-ista`, `-izmus` nouns) derived from names: (a) one-word, solid or hyphenated family names, one-word astronomical names, one-word or solid geographic names, and foreign one-word or hyphenated names with a hyphen-attached suffix: lowercase, suffix attached directly (or after the hyphen required by `ORT-FOR`); (b) personal and foreign names of several separately written elements: original capitals kept, suffix hyphenated to the last element; (c) state names: written apart, common elements lowercase, proper-name(-valued) elements capital, no hyphen; state subdivisions, institution names, and station/cinema/hotel type names: separate writing kept, common-noun elements lowercase, proper-name(-valued) elements capital; public place names: `-i` added to the unchanged form; (d) hyphenated geographic names: hyphen kept; capital kept only for a proper-name first element before a geographic common noun (`Csepel-szigeti`), common first element lowercase (`arany-pataki`), a proper-name second element lowercases everything (`új-zélandi`, `győr-moson-soproni`); three-element names: common members lowercase, proper-name members capital (`belső-János-dűlői`, `holt-Tisza-bereki`); (e) personal name + common noun compounds, award names and astronomical names with a common-noun last member: capitals and hyphen kept (`Kossuth-díjas`, `Androméda-ködbeli`); multiword astronomical names: `-beli` hyphenated, capitals kept; (f) titles: `-beli` hyphenated to the unchanged title; (g) all-caps letter acronyms keep their form with a hyphen; adjectival derivatives of proper blends are lowercase.
- **Conditions**: derivational suffix `-i`, `-s`, `-beli`, `-ista`, `-izmus`, `-ú/-ű`, `-jú/-jű` on a name.
- **Output**: see (a)-(g).
- **Examples**: (a) `móriczi`, `mikszáthos`, `tóthpáli`, `horváth-pappos`, `vénuszi`, `budapesti`, `magyarországi`, `shakespeare-i`, `bordeaux-i`; (b) `Eötvös József-i`, `Csokonai Vitéz Mihály-os`, `New York-i`, `Leonardo da Vinci-s`; (c) `francia köztársasági`, `San Marino köztársasági`, `ungvári járási`, `Baranya megyei`, `magyar tudományos akadémiai`, `Madách színházi`, `Keleti pályaudvari`, `Váci utcai`, `Táborhegyi úti`; (d) `Csepel-szigeti`, `Szent Anna-tói`, `arany-pataki`, `új-zélandi`, `kelet-közép-európai`, `belső-János-dűlői`; (e) `Kossuth-díjas`, `Árpád-kori`, `Erkel Ferenc-díjas`, `Androméda-ködbeli`, `Nagy Medve-beli`; (f) `Valóság-beli`, `Orvosi Hetilap-beli`; (g) `ENSZ-beli`, `MAFC-os`, `mavados`, `helirbeli`.
- **Exceptions**: details and variants in the referenced NAME rules (`velencei-tói~velencei-tavi`, `-falva` truncation `ORT-NAME-026`).
- **Predictability**: `rule`
- **Lexical input**: name type and structure; proper-name value of each element
- **Interactions**: `ORT-NAME-010`, `ORT-NAME-020` to `ORT-NAME-037`, `ORT-NAME-040`, `ORT-ABBR-014`, `ORT-ABBR-016`, `ORT-HYPH-017`, `ORT-HYPH-018`
- **Sources**: [AkH12 §161, §167, §173-§181, §184-§185, §187, §190, §195 a), §200 b), §217 a)-b), §288]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-018 Epithets of persons
- **Statement**: Occasional qualifying adjectives before a personal name are lowercase; permanent epithets forming part of a historical person's name are capitalised.
- **Conditions**: adjective/noun before a personal name.
- **Output**: occasional: `az ifjú Ady`, `a nagy Napóleon`, `a nyelvújító Kazinczy`, `a szép Csapóné`; fixed: `Könyves Kálmán`, `Nagy Sándor`, `Oroszlánszívű Richárd`, `Rettegett Iván`, `Rőtszakállú Frigyes`, `Szent Erzsébet`; AkH12 dictionary: `Aquinói Szent Tamás`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (fixed epithet list)
- **Lexical input**: historical epithet lexicon
- **Interactions**: `ORT-CAP-013` (pre-name common words stay lowercase); AkH11 dictionary allowed `aquinói v. Aquinói Szent Tamás`, AkH12 dictionary only `Aquinói Szent Tamás` and fixed-epithet `Szent` with abbreviation `Szt.` (AkH11 `szt.`) (dictionary changes, not in the rule text)
- **Sources**: [AkH12 §163]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-019 Particles of foreign names
- **Statement**: Name particles of foreign names (`bin`, `d'`, `de`, `ten`, `van`, `von` ...) are generally lowercase inside the full name and capitalised when the family name stands as a headword or highlighted at the start of a text (title, signature).
- **Conditions**: foreign family name with particle.
- **Output**: `Charles de Gaulle`, `Sir Anthonis van Dyck`, `Walther von Dyck`; headword/initial: `De Gaulle`, `Van Dyck`, `Von Dyck`.
- **Examples**: as Output.
- **Exceptions**: the distinction is headword vs running text: `Van Gogh` as a headword or sentence-initially, `van Gogh` in running text, `Vincent van Gogh` with the given name; suffixed and derived forms `Goghhal`, `goghi` `[ENYELV 2022-06-04]`, `[ENYELV 2024-04-24]` (`OQ-07-015` resolved 2026-09-16). Source-language conventions for names that always capitalise the particle (e.g. Flemish/Belgian `Van` in some names) are not covered by AkH (Open questions).
- **Predictability**: `rule`
- **Lexical input**: position (full name vs surname alone at headword/start)
- **Interactions**: `ORT-FOR-005`
- **Sources**: [AkH12 §214]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-020 Institution names
- **Statement**: In official or widely used multiword names of offices, organisations, institutions, research institutes, foundations, parties, cooperatives, companies every word is capitalised except the conjunction `és` and articles; type words referring back to a named institution are recommended lowercase (internal capitalised use acceptable); `Akadémia`, `Opera`, `Operaház` short forms are capitalised.
- **Conditions**: institution name (`ORT-NAME-034`, `ORT-NAME-035`).
- **Output**: `Honvédelmi Minisztérium`, `Magyar Tudományos Akadémia`, `Országos Széchényi Könyvtár`, `Ganz Danubius Hajó- és Darugyár`, `Bíró Háza Étterem és Panzió`; referring back: `a kutatóintézet~a Kutatóintézet`, `a bizottság~a Bizottság`.
- **Examples**: as Output.
- **Exceptions**: `ORT-NAME-035`, `ORT-NAME-037`.
- **Predictability**: `rule` (casing), `lexical` (name status)
- **Lexical input**: institution name registry/NER
- **Interactions**: `ORT-NAME-034`
- **Sources**: [AkH12 §186-§188]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-021 Permanent titles: periodicals, series, yearbooks
- **Statement**: Titles of newspapers, weeklies, periodicals, book series, yearbooks appearing under the same title and book-format almanacs are capitalised in every word except the conjunction `és`; sentence-like permanent titles capitalise only the first word. Library and documentation standards capitalise only the first element.
- **Conditions**: permanent (serial) title.
- **Output**: `Búvár`, `Népszabadság`, `Nyugat`, `Az Est`, `Magyar Nemzet`, `Nemzeti Sport`, `Nők Lapja`, `Élet és Tudomány`, `Magyar Remekírók`, `Nyelvtudományi Értekezések`; sentence-like: `Ne láss, ne hallj, ne szólj!`.
- **Examples**: as Output.
- **Exceptions**: library cataloguing: first word only.
- **Predictability**: `rule`
- **Lexical input**: title type (serial vs individual)
- **Interactions**: `ORT-CAP-022`, `ORT-HYPH-014`
- **Sources**: [AkH12 §196-§197]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-022 Individual titles of works
- **Statement**: Titles of individual works (poems, books, papers, articles, pictures, sculptures, music, programmes) capitalise the first word and proper names only; in running text they need no quotation marks, though quotes or different type are common.
- **Conditions**: individual title.
- **Output**: `Szózat`, `Napraforgók`, `A kőszívű ember fiai`, `Képes politikai és gazdasági világatlasz`, `Magyar értelmező kéziszótár`, `Tanulmányok Arany János költészetéről`, `Hajóvontatók a Volgán`, `Kis éji zene`, `Jó éjszakát, gyerekek!`, `Önök kérték`, `1916 őszén`, `IX. szimfónia`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: title span and type
- **Interactions**: `ORT-HYPH-014`, `PUN-QUOT`
- **Sources**: [AkH12 §196, §198-§199]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-023 Astronomical names; föld, hold, nap
- **Statement**: Names of stars, constellations, planets, moons are capitalised in every word; `föld`, `hold`, `nap` are capitalised only when used as proper names (astronomical context), which everyday writing does not require.
- **Conditions**: celestial body names.
- **Output**: `Fiastyúk`, `Ikrek`, `Orion`, `Tejút`, `Merkúr`, `Plútó`, `Vénusz`, `Bereniké Haja`, `Dél Keresztje`, `Nagy Medve`; `a Hold távolsága a Földtől`; everyday: `föld körüli utazás`, `a hold szépen világít`, `a nap sugarai`.
- **Examples**: as Output.
- **Exceptions**: AkH11 dictionary forms `Nagymedve` (AkH12 `Nagy Medve`), `Kaszás-csillag` (AkH12 `Kaszáscsillag`). Common-noun last member (`Androméda-köd`): `ORT-NAME-031`.
- **Predictability**: `rule` + semantic (astronomical vs everyday sense)
- **Lexical input**: sense
- **Interactions**: `ORT-NAME-031`
- **Sources**: [AkH12 §184]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-024 Brand names
- **Statement**: Every word of a brand name used as a product name is capitalised; product names not used as brands (material, origin) are lowercase.
- **Conditions**: brand name vs generic designation.
- **Output**: `Doxa`, `Fabulon`, `Persil`, `Trental`, `Rama Gold`, `Toyota Corolla`; generic: `meggylé`, `narancsital`, `szegedi paprika`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (brand registry)
- **Lexical input**: brand flag
- **Interactions**: `ORT-NAME-039`
- **Sources**: [AkH12 §193]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-025 Events, programmes, movements
- **Statement**: Names of events, event series, social and political movements, programmes are recommended lowercase; as institution-like names a capitalised form is acceptable.
- **Conditions**: event names.
- **Output**: default lowercase: `jövőkutatási konferencia`, `nemzetközi orvoskongresszus`, `a magyar nyelv hete`, `országos középiskolai tanulmányi verseny`, `15. nemzetközi könyvfesztivál`; acceptable capitalised: `Budapesti Nemzetközi Vásár`, `Szegedi Szabadtéri Játékok`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule` (both valid)
- **Lexical input**: none
- **Interactions**: `ORT-CAP-005`
- **Sources**: [AkH12 §191]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-026 Proper names that became common words
- **Statement**: A proper name that has become a common word is lowercase, possibly departing from the name's spelling in other respects too, often with Hungarian spelling replacing a foreign one; compounds fused from a name and a common word are solid and lowercase. The change is gradual: capitalised foreign and lowercase Hungarian forms coexist for a time, and the spelling changes only once the common-noun status is widely established.
- **Conditions**: lexicalised common noun from a name.
- **Output**: `havanna`, `pecsovics`, `röntgen`, `ferencjóska`, `háryjános`, `aszpirin`, `konyak`, `priznic`, `szendvics`, `termosz`; compounds `ádámcsutka`, `pálfordulás`, `pullmankocsi`, `wertheimzár`, `dízelmotor`, `makadámút`, `pasztőroltás`; AkH12 dictionary `internet` (lowercase; AkH11 dictionary distinguished `Internet` as institution from `internet` as network).
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon
- **Interactions**: `ORT-NAME-016`, `ORT-FOR-008`
- **Sources**: [AkH12 §166, §201]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-CAP-027 Constancy of proper-name forms and recoverable base forms
- **Statement**: Established proper-name spellings are kept unchanged (traditional family names, brand names), and suffixed or derived forms of names are written so that the base form stays recoverable, using a hyphen where direct attachment would yield the form of a different name or an unrecoverable string.
- **Conditions**: any proper name receiving a suffix or derivational suffix; names with established non-standard spellings.
- **Output**: base form preserved: `Thököly`, `Werbőczy`, `Kalmopyrin`, `Pepsi-Cola`; recoverable: `Kiss-sel` (not `Kissel`, which is the form of `Kis`), `Wittmann-né` (not `Wittmanné`, form of `Wittman`), `New York-i`, `Kuala Lumpur-i` (not `kualalumpuri`).
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule` (principle; the operational cases are `ORT-NAME-012`, `ORT-HYPH-017`, `ORT-FOR` hyphen rules)
- **Lexical input**: proper-name flag; name lexicon for established spellings
- **Interactions**: `ORT-NAME-002`, `ORT-NAME-012`, `ORT-NAME-039`, `ORT-HYPH-017`, `ORT-ASSIM-017`
- **Sources**: [AkH12 §93, §152 c)]
- **Validation**: goldens
- **Confidence**: `normative`

## NAME: proper names

### ORT-NAME-001 Personal names: capitals, registry forms
- **Statement**: Family names and given names are capitalised; the spelling of Hungarian citizens' names is fixed by civil registry law (taking the spelling system into account), so registered forms are authoritative data, not to be normalised.
- **Conditions**: personal names.
- **Output**: registered form, capital initial.
- **Examples**: `Bakos`, `Deli`, `Körmendi`, `Szántó`, `Éva`, `Zsuzsa`, `György`, `Pál`, `Szerencsés András`, `Szeremlei-Szabó Zsolt`.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: name lexicon
- **Interactions**: `REP` must never modernise names (`Kiss`, `Tóth`)
- **Sources**: [AkH12 §153]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-002 Family names: modern and traditional spellings coexist
- **Statement**: Most family names use modern letters, but many living names keep traditional or legally fixed variants that differ from the modern spelling; names deliberately coined by literary authors keep their authors' spelling.
- **Conditions**: family names.
- **Output**: lexical form; variants are distinct names (`Kis` vs `Kiss`, `Tót` vs `Tóth`).
- **Examples**: modern: `Bíró`, `Budai`, `Gál`, `Gombóc`, `Hajdú`, `Kis`, `Kulcsár`, `Miskolci`, `Ördög`, `Pajzs`, `Takács`, `Tót`; traditional: `Biró`, `Buday`, `Eördögh`, `Gaál`, `Gombocz`, `Hajdu`, `Kiss`, `Kultsár`, `Miskolczy`, `Pais`, `Takáts`, `Tóth`; literary: `Baradlay`, `Maszlaczky`, `Keöcherepy`.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: name lexicon
- **Interactions**: `ORT-NAME-003`, `ORT-NAME-012`
- **Sources**: [AkH12 §154]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-003 Archaic letters in family names
- **Statement**: Historical family names keep archaic letters and letter groups; each has a modern sound value used for pronunciation-based decisions (harmony, `-val` assimilation, hyphenation, collation by letters).
- **Conditions**: family names containing archaic graphemes.
- **Output**: keep spelling; map to sound value per table.

  | archaic | sound | examples |
  |---|---|---|
  | `aa` | á | `Gaal` [gál] |
  | `eé` | é | `Veér` [vér] |
  | `eö` | ö or ő | `Eötvös` [ötvös], `Beöthy` [bőti], `Deseö` [dezső] |
  | `ew` | ö or ő | `Thewrewk` [török], `Dessewffy` [dezsőfi] |
  | `oó` | ó | `Soós` [sós] |
  | `y` (after a vowel, or after a consonant letter with which it forms no digraph; after `g l n t` see `ORT-NAME-004`) | i | `Kölcsey` [kölcsei], `Zichy` [zicsi], `Egressy` [egresi], `Thököly` [tököli] |
  | `ch` | cs | `Madách` [madács], `Széchy` [szécsi], `Széchenyi` [szécsényi], `Georch` [görcs], `Cházár` [császár] |
  | `cz` | c | `Czuczor` [cucor], `Móricz` [móric], `Benczúr` [bencúr] |
  | `tz` | c | `Rátz`, `Czetz` [cec] |
  | `s` | zs (in some names) | `Jósika` [józsika], `Deseö` [dezső] |
  | `th` | t | `Csáth` [csát], `Kossuth` [kosut~kosút], `Thököly` [tököli] |
  | `ts` | cs | `Takáts` [takács], `Babits` [babics], `Kováts` |
  | `w` | v | `Wesselényi` [veselényi] |
  | `gh` | g | `Véghgel` (`Végh` [vég]), `Faraghó` [faragó], `Wargha` [varga] |
  | `ff` (before final `y`) | f | `Dessewffy` [dezsőfi], `Pálffy` |
  | `ss` | s; zs in some names | `Kossuth` [kosut], `Egressy` [egresi]; `Dessewffy` [dezsőfi] |
  | `tthy` before a vowel | tty | `Batthyány` [battyányi] |

- **Examples**: see table.
- **Exceptions**: sound values are name-specific (`Kossuth` [kosut]~[kosút]; `s`, `ss` = [zs] only in some names; `eö`, `ew` length; `Széchenyi` [szécsényi] has a vowel value not shown by the letters): store per-name pronunciation.
- **Predictability**: `lexical` (per-name pronunciation), `tendency` (table as default)
- **Lexical input**: name pronunciation
- **Interactions**: `ORT-NAME-011`, `ORT-SPLIT-013`, `PH-G2P`
- **Sources**: [AkH12 §12, §15, §86, §155, §229]
- **Validation**: goldens
- **Confidence**: `normative` (pronunciations given in AkH12 §12, §86, §229); `consensus` (inference) for generalising them into default letter values, esp. `ff`, `ss`, `gh`, `tz` rows, which AkH12 shows only in single names

### ORT-NAME-004 y in names versus ly
- **Statement**: In names, `y` after a vowel or after a consonant letter other than `g`, `l`, `n`, `t` is the vowel [i]. After `g`, `l`, `n`, `t` the string is either the digraph (`gy ly ny ty`) or consonant + [i]; only the name's pronunciation decides (default: digraph, unless the name has other archaic letters). Ambiguous names (`Pally`) require per-name data.
- **Conditions**: `y` in a family name.
- **Output**: [i] vs digraph decision.
- **Examples**: [i]: `Kölcsey`, `Buday`, `Vay`, `Zichy` (`ch`+`y`), `Egressy`, `Ilosvay`; consonant + [i] after `l`/`t`: `Thököly` [tököli], `Vörösmarty`, `Beöthy` [bőti], `Komjáthy`; digraph: `Kodály` [j], `Arany`, `Nagy`; `Batthyány` (`tthy` + vowel = [tty]).
- **Exceptions**: `thy` is read [ty] in `Batthyány` but [ti] in `Beöthy`, `Komjáthy`: lexical.
- **Predictability**: `tendency` + `lexical`
- **Lexical input**: name pronunciation
- **Interactions**: `ORT-NAME-003`, `ORT-JLY-006`, `PH-G2P-005` (same default); emMorph default: `y` after a letter other than `l n t g` is [i]
- **Sources**: [AkH12 §12, §86, §229] (example pronunciations only; the positional default is inference); [emMorph stemalt.rul] (study only)
- **Validation**: goldens
- **Confidence**: `consensus`

### ORT-NAME-005 Historical names: which variant
- **Statement**: Old family names keep their original form; where variants arose, use the form fixed by the family or the prominent historical person that is closest to current writing habits; long-established traditional forms are used even if their bearers wrote otherwise, and must be neither re-archaised nor modernised; without a fixed form, keep harmless archaic features allowing the old pronunciation. Living persons use their registered form.
- **Conditions**: historical family names.
- **Output**: `Beythe`, `Cházár`, `Dessewffy`, `Orczy`, `Sigray`, `Thewrewk`, `Vay`; variants: `Apafi`, `Batsányi`, `Jókai`, `Kazinczy`, `Lorántffy`, `Verseghy`; living: `Dezsőfi`, `Rákóczy`, `Teleky`; traditional: `Bocskai`, `Damjanich`, `Rákóczi`, `Zrínyi`, `Apáczai Csere`, `Tsétsi János` (not `Zrini`, not `Csécsi`); constructed: `Komjáthy`, `Pesthi`, `Werbőczy`, `Wathay`; `Kakas`, `Csombor`, `Csulyak`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: historical name registry
- **Interactions**: `REP`
- **Sources**: [AkH12 §155]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-006 Compound family names are solid
- **Statement**: Family names fused from two names are written as one word with one capital.
- **Conditions**: fused family names (lexical).
- **Output**: `Györgyjakab`, `Kispéter`, `Nagygyörgy`, `Nagysándor`, `Tóthmátyás`, `Vörösbaranyi`, `Kispista`, `Tóthpál`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: name lexicon
- **Interactions**: `ORT-NAME-010` (`tóthpáli`)
- **Sources**: [AkH12 §152 b), §156]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-007 Double family names hyphenated; old epithet names apart
- **Statement**: Double family names (two names in coordination) are joined by a hyphen, as required by civil registry law, each part capitalised; in old family names consisting of an epithet and the name proper no hyphen is written between them.
- **Conditions**: two family names.
- **Output**: `Bajcsy-Zsilinszky`, `Endrődy-Somogyi`, `Horváth-Papp`, `Kerpel-Fronius`, `Konkoly-Thege`, `Nemeskéri-Kiss`, `Szent-Györgyi`; epithet: `Baróti Szabó`, `Csokonai Vitéz`, `Kőrösi Csoma`, `Tótfalusi Kis`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: name lexicon
- **Interactions**: `ORT-HYPH-011`, `ORT-SPLIT-017`
- **Sources**: [AkH12 §152 b), §156, §169, §263 f)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-008 Given names spelled by current pronunciation
- **Statement**: Old and newly borrowed given names are spelled to reflect current standard pronunciation, except traditional `Attila` (`Atilla` also exists); `ch` and `x` are kept in given names and pet names.
- **Conditions**: given names.
- **Output**: `Csilla`, `Jolán`, `Piroska`, `Vilma`, `Árpád`, `Bálint`, `László`, `Zsolt`, `Betti`, `Klaudia`, `Zsanett`, `Artúr`, `Márió`, `Raul`, `Rómeó`; kept: `Ráchel`, `Richárd`, `Alexandra`, `Xénia`, `Félix`, `Lexi`; AkH12 dictionary: `Otília~Ottilia`.
- **Examples**: as Output.
- **Exceptions**: foreigners' given names keep foreign spelling (`ORT-FOR-005`).
- **Predictability**: `rule` + `lexical` (registry of approved given names)
- **Lexical input**: given-name registry
- **Interactions**: `ORT-FOR-007`
- **Sources**: [AkH12 §158, §209]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-009 Suffixes attach directly to Hungarian personal names
- **Statement**: Case suffixes, plural, possessive and derivational suffixes are attached to personal names without hyphen, with normal lengthening and harmony by pronunciation; the name's archaic letters are kept.
- **Conditions**: personal name (one or multiword) + inflection; one-word name + derivation.
- **Output**: direct.
- **Examples**: `Dózsát`, `Kossuthhoz`, `Szemeréről`, `Táncsics Mihálynak`, `Ilosvai Selymes Péternél`, `Vörösmartyé`, `a Hunyadiak`, `jókais`, `madáchi`, `petőfieskedők`.
- **Exceptions**: derivation on names of two or more separately written words takes a hyphen (`Táncsics Mihály-i`, `Ilosvai Selymes Péter-es`; `ORT-NAME-010`, `ORT-HYPH-017`); doubled final letter + same letter `ORT-NAME-012`; `-val/-vá` `ORT-NAME-011`.
- **Predictability**: `rule`
- **Lexical input**: name pronunciation (harmony: `Kölcsey` [i] front neutral -> `Kölcseynek`; `Vörösmarty` -> `Vörösmartynak`)
- **Interactions**: `PH-HARM`, `ORT-SUFX-001`, `NOM-PROP-001`
- **Sources**: [AkH12 §159]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-010 -i, -s, -ista, -izmus from personal names
- **Statement**: One-word, solid compound and hyphenated double family names take `-i`, `-s`, `-ista`, `-izmus` directly and become lowercase (hyphen of double names kept); names of two or more separate words keep their capitals and take the suffix with a hyphen on the last word.
- **Conditions**: personal name + these derivational suffixes.
- **Output**: `móriczi`, `mikszáthos`, `buddhista`, `jozefinizmus`, `tóthpáli`, `kispéteres`, `horváth-pappos`, `schöpf-mereis`, `rippl-rónais`; multiword: `Eötvös József-i`, `Széchenyi István-i`, `Csokonai Vitéz Mihály-os`.
- **Examples**: as Output.
- **Exceptions**: silent-letter foreign names `ORT-FOR-012` (`shakespeare-i`).
- **Predictability**: `rule`
- **Lexical input**: name structure
- **Interactions**: `ORT-CAP-017`, `ORT-HYPH-017`
- **Sources**: [AkH12 §161]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-011 -val/-vá on consonant-final personal names
- **Statement**: (a) If the name ends in a letter for a short consonant, that letter is doubled as in common words (digraphs by simplified doubling). (b) If it ends in an archaic letter (group) or `x`, the final letters stay unchanged and the suffix `v` is written as the modern letter of the pronounced final consonant (a digraph sound written once in full: `ts` + `cs`). (c) If it ends in a doubled consonant letter, `ORT-NAME-012` applies.
- **Conditions**: personal name with a pronounced final consonant + `-val/-vel`, `-vá/-vé`.
- **Output**: (a) `Name` + doubled final letter + `al/el/á/é`; (b) name + letter(s) of final sound + `al/el/á/é`: `th`->`t`, `gh`->`g`, `cz/tz`->`c`, `ts/ch`->`cs`, `x`->`sz`.
- **Examples**: (a) `Ádámmal`, `Bálinttal`, `Kodállyal`, `Szabolccsal`; (b) `Kossuthtal`, `Véghgel`, `Móriczcal`, `Rátzcal`, `Babitscsal`, `Madáchcsá`, `Marxszal`, `Luxszal`, `Alexszel`, `Félixszel`; (c) `Kiss-sel`.
- **Exceptions**: names whose final letter is a vowel in pronunciation (`y` = [i]) keep `v`: `Kölcseyvel`, `Pálffyval` (inference from the general rule).
- **Predictability**: `rule` given pronunciation
- **Lexical input**: pronounced final consonant
- **Interactions**: `ORT-ASSIM-001`, `ORT-FOR-011`, `ORT-SPLIT-013`; corpus `Kossuthtal` attested (bucket 0), `Kossuth-tal`/`Kossuthhal` unattested
- **Sources**: [AkH12 §82 a), c), §160 a)-c)]; [AkH11 §163 b) had no `x` examples in this paragraph]
- **Validation**: goldens; attestation
- **Confidence**: `normative`

### ORT-NAME-012 Names ending in a doubled consonant letter + same-letter suffix: hyphen
- **Statement**: If a proper name ends in a doubled consonant letter and the suffix (including assimilated `-val/-vá`, `-né`) begins with the same letter, the suffix is attached with a hyphen so the base form stays recoverable. In AkH12 this applies to all proper names (family names, given names, place names, brand and company names, titles), even when the affected element of the name is lowercase (`Szerelmes szonett-tel`).
- **Conditions**: proper name ending `GG` (single-letter geminate) + suffix starting with `G`.
- **Output**: Name-suffix.
- **Examples**: `Kiss-sel`, `Kiss-sé`, `Makk-kal`, `Papp-pal`, `Papp-pá`, `Széll-lel`, `Hermann-nál`, `Tarr-ról`, `Wittmann-né`, `Ivett-tel`, `Bernadett-től`, `Mariann-nal`, `Bonn-nal`, `Bükk-kel`, `Tallinn-nak`, `Elzett-tel`, `Knorr-ral`, `Blikk-kel`, `Szerelmes szonett-tel`; other suffixes direct: `Kissnek`, `Papptól`, `Kisséknél`.
- **Exceptions**: suffix starting with a different letter: direct (`Kissnek`); plural `-ok` etc. direct (`Kissek`).
- **Predictability**: `rule`
- **Lexical input**: proper-name flag
- **Interactions**: `ORT-ASSIM-017`; AkH11 simplified given names (`Bernadettel`, `Mariannal`, `Marcellal~Marcellel`) and wrote hyphen only for family names and foreign names; `ORT-SPLIT-013` (line-end split `Kis-sék`, `Pap-pot`); pairs `Kissel` (from `Kis`) vs `Kiss-sel`, `Wittmanné` (from `Wittman`) vs `Wittmann-né`
- **Sources**: [AkH12 §93, §152 c), §160 c), §265 a)]; [AkH11 §94, §163 c)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-013 Plural of personal names
- **Statement**: The plural `-k` attaches directly to the unchanged capitalised name (families, type-plurals).
- **Conditions**: personal name + plural.
- **Output**: `az Árpádok`, `a Kisfaludyak`, `a Kossuthok`, `Petőfik`, `Táncsicsok`, `az Ady Endrék és József Attilák`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-NAME-009`
- **Sources**: [AkH12 §162]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-014 Personal name + noun: apart as attribute, hyphen in compounds, dynasty words
- **Statement**: A personal name (one or more words) as attribute of a title, rank, profession or kinship noun is written apart, and so is a collective noun after a name adopted by a group; a personal name in an unmarked compound with a common noun takes a hyphen (after the last word of a multiword name; names joined by an en dash likewise); a further member added to such a hyphenated compound is also joined by a hyphen; `ház`, `család`, `dinasztia` meaning a ruling house are hyphenated.
- **Conditions**: `PersonalName N`.
- **Output**: apart: `Bánk bán`, `Dobay törzsőrmester`, `Inotai úr`, `István király`, `Kovács államtitkár`, `Noszty fiú`, `Zsófi néni`, `Boldogasszony anyánk`, `Erdélyi Ilona tudományos kutató`, `Szalay Tamás vasdiplomás orvos`, `Nagyboldogasszony napja`, `Szent Mihály hava`; groups: `Benedek család`, `Buddenbrook ház`, `Csák nemzetség`, `Kaláka együttes`, `Nyék törzs`, `Kölcsey Ferenc olvasókör`; compound: `Kossuth-szobor`, `Kossuth-nóta`, `Ady-vers`, `Jókai-villa`, `Mária-lak`, `Nádasdy-kastély`, `Forrai-kórus`, `Lenkey-század`, `Celsius-fok`, `Kodály-módszer`, `Geiger–Müller-számlálócső`, `Arany János-idézet`, `Szent István-nap`; extended: `Achilles-ín-szakadás`, `Oscar-díj-átadás`; dynasty: `Anjou-ház`, `Árpád-ház`, `Jagelló-ház`, `Habsburg-család`, `Karoling-dinasztia`.
- **Examples**: as Output.
- **Exceptions**: `ORT-NAME-015`, `ORT-NAME-016`.
- **Predictability**: `rule` given relation type (see `ORT-TOG-035`)
- **Lexical input**: relation type; dynasty sense of `ház/család` (`Benedek család` 'the Benedek family' vs `Habsburg-család` 'the dynasty')
- **Interactions**: `ORT-HYPH-003`, `ORT-TOG-035`, `ORT-63-008`
- **Sources**: [AkH12 §140, §164, §166]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-015 Separately written name + noun attribute + further noun: no hyphen
- **Statement**: If a separately written structure of personal name + common noun (`Julianus barát`) takes a further common noun, no hyphen is inserted.
- **Conditions**: `[Name N1] N2` with `Name N1` written apart.
- **Output**: all apart.
- **Examples**: `Gül Baba türbe`, `Julianus barát szobor`, `Kászim pasa bástya`, `Szent István vértanú plébániatemplom`, `Jeremiás próféta kolostor`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: structure
- **Interactions**: `ORT-NAME-014`, `ORT-HYPH-018`
- **Sources**: [AkH12 §168]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-016 Name + noun compounds that became common nouns
- **Statement**: A compound of a personal name and a common noun that became a common noun is written solid and lowercase.
- **Conditions**: lexicalised common noun.
- **Output**: `ádámcsutka`, `krisztustövis`, `martinkemence`, `pálfordulás`, `röntgensugár`, `szentjánoskenyér`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon
- **Interactions**: `ORT-CAP-026`
- **Sources**: [AkH12 §166, §201]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-017 Names with -fajta, -féle, -forma, -szerű
- **Statement**: See `ORT-HYPH-004`; multiword names take the element after the last word with a hyphen.
- **Conditions**: personal name + these elements.
- **Output**: `Eötvös-féle`, `Paál László-szerű`.
- **Examples**: `Wesselényi-féle`, `Munkácsy-szerű`, `Szent-Györgyi-féle`, `Szinyei Merse Pál-szerű`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-HYPH-004`
- **Sources**: [AkH12 §169]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-018 Animal names
- **Statement**: Proper names of animals are capitalised; an explanatory common noun follows lowercase, apart, without hyphen.
- **Conditions**: animal proper names.
- **Output**: `Bodri`, `Lassie`, `Neszmélyi Bendegúz`, `Cirmos`, `Ráró`, `Villám`, `Vuk`; `Bodri kutya`, `Ráró lovam`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: named-entity flag
- **Interactions**: -
- **Sources**: [AkH12 §170]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-019 Object names
- **Statement**: Names of national relics, vehicles (ships, old locomotives, spacecraft), weapons, musical instruments, famous gems etc. are capitalised; all AkH12 multiword examples capitalise every word (inference: every word). The paragraph is new in AkH12 (AkH11 has no object-name section).
- **Conditions**: named objects.
- **Output**: `Szent Jobb`, `Szent Korona`, `Kisfaludy Sándor` (ship), `Titanic`, `Santa Maria`, `Nádor`, `Deáki`, `Apollo`, `Luna`, `Szaljut`, `Durandal`, `Excalibur`, `Baba`, `Kövér Berta`, `Lady Blunt`, `Ignác` (bell), `Kohinoor`, `Rózsaszín Párduc`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: named-entity flag
- **Interactions**: -
- **Sources**: [AkH12 §171]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-020 One-word geographic names and their -i derivatives
- **Statement**: One-word geographic names are capitalised; their `-i` derivative is lowercase and solid; if the name ends in `i`, only one `i` is written; other suffixes attach directly with normal lengthening.
- **Conditions**: geographic name consisting of one simple word.
- **Output**: `Európa` -> `európai`; `Ausztria` -> `ausztriai`; `Erdély` -> `erdélyi`; `Mátra` -> `mátrai`; `Duna` -> `dunai`; `Debrecen` -> `debreceni`; `Huszt` -> `huszti`; `Krakkó` -> `krakkói`; `Tamási` -> `tamási`; `Zamárdi` -> `zamárdi`.
- **Examples**: as Output; `Budapesten`, `Szegedre`.
- **Exceptions**: place-name locative `-t/-tt` (`Győrött`, `Pécsett`, `ORT-LEN-017`); stem alternations of place names (`Eger` -> `egri`) are lexical (`NOM`).
- **Predictability**: `rule`
- **Lexical input**: geographic-name flag; morphology of name stems
- **Interactions**: `ORT-SUFX-004`, `ORT-CAP-017`
- **Sources**: [AkH12 §172-§173]
- **Validation**: goldens; attestation (`budapesti` 20, `váci` 14)
- **Confidence**: `normative`

### ORT-NAME-021 Solid multi-element geographic names
- **Statement**: Written solid (capital initial, lowercase `-i` form): country and region names with `-ország`, `-föld`, `-alföld`, `-part`, `-szék` and some multi-element landscape names (listed in the dictionary); multi-element Hungarian settlement names and generally names of settlement parts. Settlement-part names containing a multiword personal name are hyphenated before the generic element and keep capitals in the `-i` form; a public-place name + generic keeps separate writing.
- **Conditions**: listed types.
- **Output**: `Magyarország` -> `magyarországi`; `Franciaország`, `Csecsenföld`, `Palócföld`, `Németalföld`, `Kisalföld`, `Elefántcsontpart`, `Háromszék`, `Dunakanyar`, `Hegyalja`, `Szamoshát`; `Budapest` -> `budapesti`; `Balatonalmádi`, `Vonyarcvashegy`, `Garamszentbenedek`, `Szépkenyerűszentmárton`, `Törökkanizsa`, `Újvidék`, `Fokváros`, `Békásmegyer`, `Ferencváros`, `Alsóbélatelep`, `Dénesmajor`, `Csehipuszta`, `Rétitanya`, `Angyalkert`, `Békeliget`; but `Baross Gábor-telep` -> `Baross Gábor-telepi`, `Biczó István-kert` -> `Biczó István-kerti`, `József Attila-lakótelep` -> `József Attila-lakótelepi`, `Füredi úti lakótelep` -> `Füredi úti lakótelepi`.
- **Examples**: as Output.
- **Exceptions**: the 6:3 rule does not apply (`ORT-63-007`; inferred from solid examples such as `Szépkenyerűszentmárton`, not stated in §139-§141).
- **Predictability**: `lexical` (which landscape names are solid), `rule` (settlements)
- **Lexical input**: official gazetteer (Magyarország Földrajzinév-tára) for Hungarian names
- **Interactions**: `ORT-NAME-026`, `ORT-SPLIT-015`
- **Sources**: [AkH12 §174 a)-b)]
- **Validation**: goldens; attestation (`balatonalmádi` 9)
- **Confidence**: `normative`

### ORT-NAME-022 Geographic common noun after a name element: hyphen
- **Statement**: A geographic name consisting of a (one-word or solid) common word or proper name + a geographic common noun joins the lowercase common noun to the capitalised first part with a hyphen; in the `-i` (`-beli`) derivative the hyphen stays; the initial is lowercase if the first part is a common word, capital if it is a proper name (also a multiword name).
- **Conditions**: first part + geographic common noun (`óceán`, `tó`, `patak`, `hegy`, `völgy`, `domb`, `hágó`, `lapos`, `dűlő`, `sivatag`, `sziget`, `félsziget`, `öntözőcsatorna`, `holtág`, `fennsík`, `dombvidék`, `szőlőhegy`, `halastó`, `hegység`, `tenger`, `medence`, `felvidék`, `havasok`, `fok`, ...).
- **Output**: common-word first part: `Arany-patak` -> `arany-pataki`; `Velencei-tó` -> `velencei-tói~velencei-tavi`; `Sólyom-sziget` -> `sólyom-szigeti`; `Sváb-hegy` -> `sváb-hegyi`; `Fogarasi-havasok` -> `fogarasi-havasokbeli`; `Gyergyói-medence` -> `gyergyói-medencei`; `Kis-kőszikla` -> `kis-kősziklai`; `Holdvilág-árok` -> `holdvilág-ároki`; `Dunántúli-középhegység` -> `dunántúli-középhegységi`; name first part: `János-dűlő` -> `János-dűlői`; `Csepel-sziget` -> `Csepel-szigeti`; `Balaton-felvidék` -> `Balaton-felvidéki`; `Kaszpi-tenger` -> `Kaszpi-tengeri`; `Kab-hegy` -> `Kab-hegyi`; `Huron-tó` -> `Huron-tói~-tavi`; `Szandaváralja-patak` -> `Szandaváralja-pataki`; `Dunazug-hegység` -> `Dunazug-hegységi`; `Szent Anna-tó` -> `Szent Anna-tói~-tavi`; `Vas Mihály-hegy` -> `Vas Mihály-hegyi`; `Buen Tiempo-fok` -> `Buen Tiempo-foki`.
- **Examples**: as Output; also `Zsivány-patak`, `Bakónaki-tó`, `Apáthy-szikla`, `Hűvös-völgy` (terrain) vs `Hűvösvölgy` (district), `Sáros-patak` (stream) vs `Sárospatak` (town).
- **Exceptions**: `-tó` derivative `-tói~-tavi` variants; same form different referent type -> different spelling (lexical by referent type).
- **Predictability**: `rule`
- **Lexical input**: referent type (terrain/water vs settlement); whether the first part is a common word or a proper name (a word with no recognisable common meaning counts as a proper name)
- **Interactions**: `ORT-CAP-017`, `ORT-NAME-030` (street-name conversion)
- **Sources**: [AkH12 §172, §175]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-023 Proper-name second part with a prefix element: hyphen, all lowercase in -i
- **Statement**: If the second part is a (one-word or solid) proper name and the first part a common word (compass direction, `Felső`, `Új`, `Holt`, `Magas`...) or a proper name, the parts are hyphenated, each capitalised; in the `-i` derivative both parts are lowercase and the hyphen stays; likewise settlement + settlement-part names.
- **Conditions**: `X-ProperName`.
- **Output**: `Holt-Tisza` -> `holt-tiszai`; `Dél-Kína` -> `dél-kínai`; `Magas-Tátra` -> `magas-tátrai`; `Öreg-Cselőte` -> `öreg-cselőtei`; `Új-Zéland` -> `új-zélandi`; `Felső-Csallóköz` -> `felső-csallóközi`; `Nyugat-Dunántúl` -> `nyugat-dunántúli`; `Délkelet-Magyarország` -> `délkelet-magyarországi`; `Kál-Kápolna` -> `kál-kápolnai`; `Győr-Moson-Sopron` -> `győr-moson-soproni`; `Bödd-Bakod` -> `bödd-bakodi`; `Csörnöc-Herpenyő` -> `csörnöc-herpenyői`; `Érd-Ófalu` -> `érd-ófalui`; `Győr-Ménfőcsanak` -> `győr-ménfőcsanaki`; `Észak-Afrika`; `Nagy-New York`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: part types
- **Interactions**: `ORT-CAP-017`, `ORT-NAME-024`
- **Sources**: [AkH12 §175 c), §176, §263 g)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-024 Three-element names: common prefix + hyphenated name
- **Statement**: A common-word prefix before a two-element hyphenated geographic name is joined with a hyphen and the whole is capitalised; in the `-i` derivative the hyphens stay, common-word parts are lowercase, proper-name parts keep capitals.
- **Conditions**: `CommonWord-[X-Y]` where `X-Y` is a `ORT-NAME-022`/`ORT-NAME-023` name.
- **Output**: `Rohonci-Arany-patak` -> `rohonci-arany-pataki`; `Alcsi-Holt-Tisza` -> `alcsi-holt-tiszai`; `Új-Dél-Wales` -> `új-dél-walesi`; `Belső-János-dűlő` -> `belső-János-dűlői`; `Nagy-Hideg-hegy` -> `nagy-hideg-hegyi`; `Kelet-Közép-Európa` -> `kelet-közép-európai`; `Holt-Sebes-Körös` -> `holt-sebes-körösi`.
- **Examples**: as Output.
- **Exceptions**: the §177 text says proper-name members keep capitals, but its (a) examples lowercase `Tisza`, `Körös`, `Európa`, `Wales` (lowercase in the embedded name's own derivative, `ORT-NAME-023`) and keep only `János` (capital in `János-dűlői`, `ORT-NAME-022`). Implement the examples: each member keeps the case it has in the `-i` derivative of the embedded two-element name.
- **Predictability**: `rule`
- **Lexical input**: structure of the embedded name
- **Interactions**: `ORT-NAME-022`, `ORT-NAME-023`
- **Sources**: [AkH12 §177 a)]
- **Validation**: goldens
- **Confidence**: `normative` (examples); `contested` (text and examples diverge; see `ORT-NAME-025`)

### ORT-NAME-025 Three-element names: hyphenated name + geographic common noun
- **Statement**: A geographic common noun added after a two-element hyphenated name is attached lowercase with a hyphen; in the `-i` derivative the hyphens stay, common-word parts lowercase, proper-name parts capitalised.
- **Conditions**: `[X-Y]-geographicNoun`.
- **Output**: `Arany-patak-völgy` -> `arany-patak-völgyi`; `Két-bükkfa-nyereg` -> `két-bükkfa-nyeregi`; `Dél-kínai-tenger` -> `dél-kínai-tengeri`; `Holt-Tisza-berek` -> `holt-Tisza-bereki`; `Kelet-európai-síkság` -> `kelet-európai-síksági`; `János-dűlő-oldalas` -> `János-dűlő-oldalasi`; `Zöld-foki-szigetek` -> `zöld-foki-szigeteki`.
- **Examples**: as Output.
- **Exceptions**: `holt-Tisza-bereki` keeps `Tisza` capital, following the §177 text (proper-name members capitalised), while `holt-tiszai` (`ORT-NAME-023`) and `alcsi-holt-tiszai` (`ORT-NAME-024`) lowercase it. A consistent reading (inference): a proper-name member keeps its capital when it is not the member that takes `-i`. See Open questions.
- **Predictability**: `rule`
- **Lexical input**: part types
- **Interactions**: `ORT-NAME-022`, `ORT-NAME-024`
- **Sources**: [AkH12 §177 b)]
- **Validation**: goldens
- **Confidence**: `normative` (examples); `contested` (generalisation of capital retention)

### ORT-NAME-026 -falva, -háza type names drop final a/e before -i
- **Statement**: Names ending in `-falva`, `-földe`, `-halma`, `-háza`, `-telke`, `-völgye` etc. (possessive-form elements) and occasionally other `a/e`-final names drop the final `a/e` in the `-i` derivative; for names that are not well known and whose form is not fixed by tradition, the full form (`-ai/-ei`) may be useful for recovering the base; neither variant is an error.
- **Conditions**: settlement name ending in a possessive-shaped generic.
- **Output**: `Hollóháza` -> `hollóházi`; `Királyhalma` -> `királyhalmi`; `Pannonhalma` -> `pannonhalmi`; less known: `Kerekegyháza` -> `kerekegyházi~kerekegyházai`; `Becsvölgye` -> `becsvölgyi~becsvölgyei`.
- **Examples**: as Output.
- **Exceptions**: for well-known or traditional forms the dropped form is the usual one (`hollóházi porcelán`, `pannonhalmi főapátság`); the full form is not declared wrong.
- **Predictability**: `rule` + `tendency` (both correct for less-known names)
- **Lexical input**: generic element list; name familiarity (not computable: accept both)
- **Interactions**: `ORT-NAME-021`; corpus `hollóházi` 6, `hollóházai` unattested
- **Sources**: [AkH12 §174 c)]
- **Validation**: goldens; attestation
- **Confidence**: `normative`

### ORT-NAME-027 From-to geographic combinations with en dash
- **Statement**: Capitalised geographic names in a from-to relation are joined with an en dash; in the `-i` derivative the dashes stay, all parts are lowercase and `-i` attaches to the last part directly. A geographic common noun is attached lowercase with a hyphen, and then the `-i` goes directly onto the unchanged form (capitals kept). Name associations like `Cseh–Morva-dombság` are written likewise in the base but lowercase in the `-i` form.
- **Conditions**: `Name1–Name2(-noun)`.
- **Output**: `Budapest–Bécs` -> `budapest–bécsi`; `Moszkva–Párizs` -> `moszkva–párizsi`; `Baradla–Domica-barlangrendszer` -> `Baradla–Domica-barlangrendszeri`; `Volga–Don-csatorna` -> `Volga–Don-csatornai`; `Rajna–Majna–Duna-csatorna` -> `Rajna–Majna–Duna-csatornai`; `Cseh–Morva-dombság` -> `cseh–morva-dombsági`; `Zala–Somogyi-határárok` -> `zala–somogyi-határároki`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: whether the elements are names of the connected places (keep capitals with noun) or adjective-like name associations (lowercase)
- **Interactions**: `ORT-HYPH-013`
- **Sources**: [AkH12 §178, §264]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-028 State names
- **Statement**: Current and historical state names (usually containing a word for the form of state) and euroregions: each word separate and capitalised except `és`; in the `-i`/`-beli` derivative separate writing stays, common elements become lowercase, proper-name elements stay capitalised.
- **Conditions**: state names.
- **Output**: `Francia Köztársaság` -> `francia köztársasági`; `Egyesült Királyság` -> `egyesült királysági`; `Ománi Szultánság` -> `ománi szultánsági`; `Amerikai Egyesült Államok` -> `amerikai egyesült államokbeli`; `Dél-afrikai Köztársaság` -> `dél-afrikai köztársasági`; `Római Birodalom` -> `római birodalmi`; `Kijevi Nagyfejedelemség` -> `kijevi nagyfejedelemségi`; `Suriname Köztársaság` -> `Suriname köztársasági`; `San Marino Köztársaság` -> `San Marino köztársasági`; `Trinidad és Tobago Köztársaság` -> `Trinidad és Tobago köztársasági`; `Ipoly Eurorégió` -> `Ipoly eurorégióbeli`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: element types (adjective from a name `francia` counts as common; a bare name `Suriname` counts as proper)
- **Interactions**: `ORT-CAP-017`
- **Sources**: [AkH12 §179]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-029 State subdivisions
- **Statement**: In names of state subdivisions the lowercase generic (`járás`, `kistérség`, `városkörnyék`, `bánság`, `terület`, `megye`, `régió`, `grófság`, `állam`) is written apart; in the `-i`/`-beli` derivative separation stays, common elements lowercase, proper elements capitalised.
- **Conditions**: subdivision names.
- **Output**: `Ungvári járás` -> `ungvári járási`; `Abaúj-Hegyközi kistérség` -> `abaúj-hegyközi kistérségi`; `Mezőtúri városkörnyék` -> `mezőtúri városkörnyéki`; `Szörényi bánság` -> `szörényi bánsági`; `Voronyezsi terület` -> `voronyezsi területi`; `Baranya megye` -> `Baranya megyei`; `Szabolcs-Szatmár-Bereg megye` -> `Szabolcs-Szatmár-Bereg megyei`; `Dél-Dunántúl régió` -> `Dél-Dunántúl régióbeli`; `Kent grófság` -> `Kent grófsági`; `New York állam` -> `New York állami~állambeli`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: element types
- **Interactions**: `ORT-NAME-028`
- **Sources**: [AkH12 §180]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-030 Public place names
- **Statement**: In names of public places the lowercase generic (`utca`, `út`, `tér`, `köz`, `körút`, `lépcső`, `híd` ...) is written apart; `-i` attaches to the unchanged form. `híd` in a possessive relation to a river name is hyphenated. A hyphenated geographic name with a geographic common noun becomes solid when it enters a street name.
- **Conditions**: street/square/bridge names.
- **Output**: `Váci utca` -> `Váci utcai`; `Gutenberg körút` -> `Gutenberg körúti`; `Szabó Ilonka lépcső` -> `Szabó Ilonka lépcsői`; `Tas vezér utca` -> `Tas vezér utcai`; `Március 15. tér` -> `Március 15. téri`; `Táborhegyi út` -> `Táborhegyi úti`; `Szemlőhegy utca` -> `Szemlőhegy utcai`; `Erzsébet híd` -> `Erzsébet hídi`; river: `Tisza-híd`, `a Duna-hidak`; conversion: `Csörsz-árok` -> `Csörszárok utca`; `Ferenc-hegy` -> `Ferenchegyi út`; contrast `Ferenc-hegy` (terrain) vs `Ferenc körút` (street).
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: referent type; whether `híd` is 'bridge of the river' (possessive) or a named bridge
- **Interactions**: `ORT-NAME-022`; the conversion rule is new in AkH12
- **Sources**: [AkH12 §172, §181]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-031 Astronomical names
- **Statement**: Multiword astronomical names capitalise every word; `-i`/`-beli` forms: one-word -> lowercase solid (`vénuszi`), multiword -> capitals kept with hyphen (`Nagy Medve-beli`), `hold` -> `holdbeli`. A final common-noun element (`felhő`, `köd`, `üstökös`, `kráter`) is hyphenated to the name; derivatives keep the capital.
- **Conditions**: celestial names.
- **Output**: `vénuszi`, `holdbeli`, `Nagy Medve-beli`; `Magellán-felhő`, `Androméda-köd`, `Jászol-köd`, `Orion-köd`, `Rák-köd`, `Hale–Bopp-üstökös`, `Kopernikusz-kráter`, `Androméda-ködbeli`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: named-entity type
- **Interactions**: `ORT-CAP-023` (`föld`, `hold`, `nap`); the multiword capitalisation, `-i/-beli` forms and the hyphen rule for celestial common nouns are new in AkH12 (AkH11 §185 only capitalised simple names)
- **Sources**: [AkH12 §184-§185]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-032 Explanatory common nouns and occasional attributes with geographic names
- **Statement**: A common noun added to explain a geographic name (not part of the name) is written apart; in the derivative separation and initials are unchanged. Occasional attributes before the name are lowercase and apart; in the derivative the name follows its own derivative spelling.
- **Conditions**: `Name explanatoryNoun`; `attribute Name`.
- **Output**: `Gyöngyös város` -> `Gyöngyös városi`; `Mátra hegység` -> `Mátra hegységi`; `Fertő tó` -> `Fertő tói~tavi`; `Séd patak` -> `Séd pataki`; `Duna folyam` -> `Duna folyami`; `Krím félsziget` -> `Krím félszigeti`; `Szahara sivatag` -> `Szahara sivatagi`; attributes: `az olasz Alpok` -> `az olasz alpokbeli`; `az erdős Bükk` -> `az erdős bükki`; `a lengyel Magas-Tátra` -> `a lengyel magas-tátrai`; `a második Baku` -> `a második bakui`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (whether the noun is part of the official name: `Fertő tó` vs `Velencei-tó`, gazetteer)
- **Lexical input**: official name form
- **Interactions**: `ORT-NAME-022`, `ORT-NAME-039` (brand + product analogue)
- **Sources**: [AkH12 §182]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-033 Phrase-structured geographic names
- **Statement**: Geographic names with the structure of a marked object, adverbial, possessive or postpositional phrase keep separate writing with only the first word capitalised; `-i` attaches to the unchanged form (possessive-final forms may drop the possessive).
- **Conditions**: phrase-shaped names.
- **Output**: `Keveset érő` -> `Keveset érői`; `Székre járó` -> `Székre járói`; `Tilos alja` -> `Tilos aljai~alji`; `Duna mente` -> `Duna menti`; `Szekszárd vidéke` -> `Szekszárd vidéki`; `Vác környéke` -> `Vác környéki`; `Rétek alatt` -> `Rétek alatti`; `Felső-Marcal melléke` -> `Felső-Marcal melléki`; `Kis-kút feletti`; `Duna–Tisza köze` -> `Duna–Tisza közi`; `Maros–Körös köze` -> `Maros–Körös közi`.
- **Examples**: as Output.
- **Exceptions**: `mente`, `vidéke`, `környéke`, `melléke`, `köze` drop the possessive before `-i`.
- **Predictability**: `rule`
- **Lexical input**: phrase structure
- **Interactions**: `ORT-TOG-018`
- **Sources**: [AkH12 §183]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-034 Institution names and their -i derivatives
- **Statement**: Multiword institution names capitalise every word except `és` and articles; in the `-i`/`-beli` derivative common elements become lowercase and proper-name-valued elements keep capitals; solid institution names exist.
- **Conditions**: institution names.
- **Output**: `Honvédelmi Minisztérium` -> `honvédelmi minisztériumi`; `Magyar Tudományos Akadémia` -> `magyar tudományos akadémiai`; `Madách Színház` -> `Madách színházi`; `Nemzeti Múzeum` -> `nemzeti múzeumi`; `Törökbálint Városi Önkormányzat` -> `Törökbálint városi önkormányzati`; `Géza Király Téri Egészségügyi Szakközépiskola` -> `Géza király téri egészségügyi szakközépiskolai`; `Szilas Menti Lakásépítő és -fenntartó Szövetkezet` -> `Szilas menti lakásépítő és -fenntartó szövetkezeti`; `József Attila Gimnázium` -> `József Attila gimnáziumi`; `Babeş–Bolyai Tudományegyetem` -> `Babeş–Bolyai tudományegyetemi`; `Batthyány Lajos Alapítvány` -> `Batthyány Lajos alapítványi`; `Magyar–Japán Baráti Társaság` -> `magyar–japán baráti társasági`; `Országos Széchényi Könyvtár` -> `országos Széchényi könyvtári`; `Dél-budai Vendéglátó Vállalat` -> `dél-budai vendéglátó vállalati`; `I. Sz. Sebészeti Klinika` -> `I. sz. sebészeti klinikai`; `Bíró Háza Étterem és Panzió` -> `Bíró Háza étterem és panzióbeli`; `Corvin Budapest Filmpalota` -> `Corvin Budapest filmpalotabeli`; solid: `Belügyminisztérium` -> `belügyminisztériumi`; likewise `Pénzügyminisztérium`, `Bélyegmúzeum`.
- **Examples**: as Output. Hungarian names of international bodies follow the same rules: `Egyesült Nemzetek Szervezete`, `Észak-atlanti Szerződés Szervezete`, `Nemzetközi Valutaalap`; unofficial descriptions lowercase: `a francia külügyminisztérium`, `az osztrák légitársaság`.
- **Exceptions**: -
- **Predictability**: `rule` (casing), `lexical` (element status: which words are proper-name-valued)
- **Lexical input**: element status (personal/place names inside, `Bíró Háza` as a name)
- **Interactions**: `ORT-CAP-020`, `ORT-HYPH-017`
- **Sources**: [AkH12 §187, §192]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-035 Informal, shortened and internal-unit institution names
- **Statement**: (a) Institution names identical to informal descriptions are lowercase unless the proper-name character is stressed; (b) a seat name after a comma is part of the full name; moved to the front as an `-i` adjective it is capitalised as part of the name; clearly occasional location adjectives are lowercase; (c) shortened names keep the capitals of the retained elements; (d) type words referring to a named institution are recommended lowercase (capitals from occasional proper-name use are acceptable for internal use); `Akadémia`, `Opera~Operaház` capitalised; subordinate institutions capitalise all words; larger internal units when uniqueness matters; small units and ad hoc committees lowercase.
- **Conditions**: variants of institution names.
- **Output**: (a) `a pécsi nevelőintézet`, `a tolmácsi általános iskola`; (b) `Nemzeti Színház, Szeged`, `Szegedi Nemzeti Színház`; `a budapesti Eötvös Loránd Tudományegyetem`; (c) `a Lombiknál dolgozik`, `a Gondolat jelentette meg`, `a Nemzetiben játsszák`, `az Eötvösön~az Eötvös Lorándon végzett`; (d) `a kutatóintézet kidolgozta~a Kutatóintézet kidolgozta`, `a bizottság véleménye szerint~a Bizottság véleménye szerint`; `Akadémia`, `Opera~Operaház`; `Állam- és Jogtudományi Kar`, `Nyelvtudományi Intézet`, `Járványtani és Mikrobiológiai Tanszék`, `a Bölcsészettudományi Kar gondnoksága`, `a műszaki osztály`, `a felügyelő bizottság`, `koordináló bizottságot alakítottak`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule` + semantic (occasional vs part of name)
- **Lexical input**: official name registry
- **Interactions**: `ORT-CAP-020`, `ORT-CAP-011`
- **Sources**: [AkH12 §148, §188-§189]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-036 Awards and decorations
- **Statement**: (a) Proper name + `díj/érem/emlékérem/...`: hyphen (multiword name: after the last word); `-s` derivative keeps the base; (b) generic with its own attribute after a name: no hyphen, attribute and generic lowercase; (c) generic as organic part in a possessive or attributive structure: all words capitalised; (d) a non-name first part that becomes name-valued: all its words capitalised, generic lowercase, no hyphen; (e) grades and classes lowercase; (f) some compound generic names became names: solid capitalised.
- **Conditions**: award names.
- **Output**: (a) `Kossuth-díj`, `Nobel-díj`, `Erkel Ferenc-díj`, `Forbáth Imre-díj`, `Eötvös Loránd-emlékérem`, `Grammy-díj`, `ICOMOS-díj`, `NOB-érdemérem`, `Kossuth-díjas`, `Erkel Ferenc-díjas`, `Eötvös Loránd-emlékérmes`, `NOB-érdemérmes`; (b) `Gundel művészeti díj`, `Ybl építészeti díj`; (c) `Akadémiai Aranyérem`, `a Francia Köztársaság Becsületrendje`, `Magyar Érdemrend`; (d) `Életfa díj`, `Korona érdemrend`, `Közép-európai Örökség díj`, `Pro Urbe díj`, `Pro Universitate díj`; (e) `a Magyar Érdemrend középkeresztje a csillaggal`; (f) `Citromdíj`, `Sajtódíj`, `Toleranciadíj`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule` given the structure type; `lexical` for (d)/(f)
- **Lexical input**: award name registry
- **Interactions**: `ORT-HYPH-018`, `ORT-63-008` (`Nobel-díj-átadás`)
- **Sources**: [AkH12 §195]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-037 Stations, cinemas, hotels, restaurants, shops, baths, cemeteries, residential parks, rooms
- **Statement**: In names of stations, stops, airports, cinemas, hotels, restaurants, espresso bars, shops, baths, cemeteries, residential parks and rooms not forming a separate institution the proper-name-valued part is capitalised and the explanatory common noun is lowercase and apart (it may be omitted, the rest keeping its form); `-i`/`-beli` derivatives keep capitals and separation. Such facilities may also have full institution names (all words capitalised).
- **Conditions**: facility names used in institution-like (not official institution) form.
- **Output**: `Keleti pályaudvar`, `Katonatelep vasúti megállóhely`, `Corvin mozi`, `Vadszőlő szálloda`, `Kis Rabló étterem`, `Apacuka internetkávézó`, `Vén Diák eszpresszó`, `Kaiser’s szupermarket`, `Korona cipőbolt`, `Lukács fürdő`, `Kerepesi temető`, `Sasadliget lakópark`, `Kossuth rádió`, `Gombocz Zoltán terem`, `Nádor terem`; omitted: `a Keletiből`, `a Lukácsba`, `a Kis Rablóban`, `a Kossuthon`; derivatives: `Keleti pályaudvari`, `Tabán mozibeli`, `Kis Rabló éttermi`; institution form: `Vadszőlő Szálloda`, `Szt. Lukács Gyógyfürdő és Uszoda`, `Fiumei Úti Nemzeti Sírkert`, `Klasszik Rádió`, `Csontváry Terem`, `Platán Panzió`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (official status decides the form)
- **Lexical input**: whether the name is the official institution name
- **Interactions**: `ORT-CAP-020`; hotels, residential parks, rooms added in AkH12
- **Sources**: [AkH12 §190]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-038 Events; international institutions
- **Statement**: Names of events, event series, movements and programmes are recommended lowercase, capitals being acceptable when used as an institution name (`ORT-CAP-025`); Hungarian official names of foreign or international institutions follow `ORT-NAME-034`, unofficial descriptions are lowercase.
- **Conditions**: see referenced rules.
- **Output**: see referenced rules.
- **Examples**: `nemzetközi orvoskongresszus`, `Budapesti Nemzetközi Vásár`; `Egyesült Nemzetek Szervezete`, `a Francia Köztársaság Külügyminisztériuma`; `a lengyel hadsereg`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: official status
- **Interactions**: `ORT-CAP-025`, `ORT-NAME-034`
- **Sources**: [AkH12 §191-§192]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-039 Brand names: suffixing and explanatory nouns
- **Statement**: Every member of a brand name is capitalised (`ORT-CAP-024`); brand names take suffixes by the general rules; an informative type noun after a brand name is written apart and lowercase.
- **Conditions**: brand + suffix / type noun.
- **Output**: `Audi gépkocsi`, `Omnia kávé`, `Panangin tabletta`, `Algoflex Forte fájdalomcsillapító`, `Tomi Kristály mosópor`; suffixes (constructed by the general rules): `Fabulonnal`, `Persilt`; double letters: `Blikk-kel`, `Elzett-tel`, `Knorr-ral`.
- **Examples**: as Output.
- **Exceptions**: brands with hyphen in their own form keep it (`Pepsi-Cola`, `Coca-Colát`).
- **Predictability**: `rule`
- **Lexical input**: brand flag
- **Interactions**: `ORT-CAP-024`, `ORT-NAME-012`, `ORT-FOR-010`
- **Sources**: [AkH12 §93, §152 c), §193-§194]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NAME-040 Titles: types and forms
- **Statement**: Titles are permanent (periodicals, series, yearbooks: `ORT-CAP-021`) or individual (works, articles, programmes: `ORT-CAP-022`); suffixation `ORT-HYPH-014`; titles in running text generally need no quotation marks, though quoting or a distinct typeface for individual titles is widespread; standalone headings take no period (`PUN`).
- **Conditions**: titles.
- **Output**: see referenced rules.
- **Examples**: `Móricznak egyik híres regénye a Rokonok.`; `a Siralomháznak`; `Tanulmányát a Magyar Nyelv jelentette meg.`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: title span and type
- **Interactions**: `ORT-CAP-021`, `ORT-CAP-022`, `ORT-HYPH-014`, heading punctuation (out of scope)
- **Sources**: [AkH12 §196-§200, §266, §273]
- **Validation**: goldens
- **Confidence**: `normative`

## FOR: foreign words and names

### ORT-FOR-001 Loanwords and foreign words: Hungarian spelling once common
- **Statement**: Fully naturalised loanwords are always spelled in Hungarian letters; words whose foreign origin is still felt are "foreign words", spelled in Hungarian letters once they become common (pronunciation rendered by Hungarian letter values) or kept in the source spelling while rare; some Hungarian-spelled words retain one original element.
- **Conditions**: common words from Latin-script languages.
- **Output**: Hungarian spelling for common items; source spelling for rare ones (lexicon decides).
- **Examples**: `antracén`, `digitális`, `diktafon`, `dzsúsz`, `fájl`, `hisztamin`, `izotóp`, `kapucsínó`, `kóla`, `kombájn`, `lézer`, `magnetofon`, `menedzser`, `szelektív`, `televízió`; retained elements: `bonbon`, `futball`, `millió`; AkH12 dictionary variants (not in the rule text): `e-mail~ímél`, `pacemaker~pészméker`, `spray~spré~szpré`, `standard~sztenderd`, `dzsessz~jazz`.
- **Exceptions**: `ORT-FOR-004`.
- **Predictability**: `lexical`
- **Lexical input**: lexicon (naturalisation status is a dictionary fact)
- **Interactions**: `ORT-FOR-002`, `PH-G2P`
- **Sources**: [AkH12 §202 a), §203]; [HUWIKI-AKH12CH]
- **Validation**: oracle:magyarispell; attestation
- **Confidence**: `normative`

### ORT-FOR-002 ch, x, qu in Hungarian-spelled foreign words
- **Statement**: In Hungarian-spelled common foreign words `ch` pronounced [h] is kept; `x` pronounced [ksz] is generally kept (rarely `ksz`); `x` pronounced [gz] is mostly respelled `gz` (sometimes kept); `qu` is respelled `kv` in common words. In given names and geographic names of foreign origin `ch` and `x` are always kept.
- **Conditions**: Hungarian-spelled foreign common words with these letters; given names and geographic names with `ch`, `x`.
- **Output**: `jacht`, `mechanika`, `monarchia`, `pech`, `technika`; `expressz`, `fix`, `oxigén`, `szexuális`, `taxi`, `textil`, but `bokszol`; `egzakt`, `egzotikus`, `egzisztencia`, but `exegézis`, `exogámia`, `hexameter`; `akvárium`, `antikvárium`, `kvartett`, `kvint`; names: `Albrecht`, `Richárd`, `Beatrix`, `Félix`, `Maximilián`, `Lech-mező`, `Luxemburg`, `Mexikó`.
- **Examples**: as Output.
- **Exceptions**: listed lexical exceptions.
- **Predictability**: `rule` (`ch`, `qu`), `tendency` (`x`)
- **Lexical input**: lexicon for `x`
- **Interactions**: `ORT-SPLIT-010`, `ORT-FOR-011` (`fixszel`)
- **Sources**: [AkH12 §204, §209]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-003 Naturalised words from non-Latin scripts: by Hungarian pronunciation
- **Statement**: Established words borrowed from non-Latin-script languages (often via other languages) are spelled by their usual Hungarian pronunciation, regardless of transliteration rules.
- **Conditions**: established loans.
- **Output**: `dzsunka`, `gésa`, `karate`, `láma`, `mecset`, `minaret`, `pagoda`, `szamovár`, `számum`, `szamuráj`, `tájfun`, `tajga`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon
- **Interactions**: `ORT-FOR-017`
- **Sources**: [AkH12 §205, §218]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-004 Foreign spelling of rare words and of foreign phrases
- **Statement**: Some (mostly less frequent) common words from Latin-script languages are written in lowercase with the source-language spelling; multiword foreign expressions, sayings and proverbs keep the source spelling, and a word otherwise naturalised is written in the source spelling inside such an expression.
- **Conditions**: rare foreign word; multiword foreign expression.
- **Output**: `bestseller`, `couchette`, `crescendo`, `kapitälchen`, `lady`, `myocarditis`, `rinascimento`, `señor`, `whisky`, `złoty`; `all right`, `park and ride`, `eppur si muove`, `sine qua non`, `tête-à-tête`; `bacilus` but `Bacillus anthracis`; `dózis` but `dosis effectiva minima`; `professzor` but `professor emeritus`; `státus~státusz` but `status quo`; `szaltó` but `salto mortale`; `technikus` but `terminus technicus`; `tuberkulózis` but `tuberculosis bronchialis`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule` (phrases), `lexical` (single words)
- **Lexical input**: foreign-phrase detection (language ID of the span)
- **Interactions**: `REP` (do not magyarise foreign phrases)
- **Sources**: [AkH12 §212-§213]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-005 Latin-script proper names keep the original spelling, diacritics included
- **Statement**: Proper names from Latin-script languages are used unchanged, including letters with diacritics unknown in Hungarian; diacritics may not be dropped (AkH12); family names with such letters must be written with them in official documents on the bearer's request.
- **Conditions**: foreign proper names.
- **Output**: source spelling.
- **Examples**: `Cervantes`, `Chopin`, `Eminescu`, `Horatius`, `Puccini`, `Rousseau`, `Shakespeare`, `Zweig`, `Aquincum`, `Bologna`, `Buenos Aires`, `Hawaii`, `Karlovy Vary`, `Loire`, `Tallinn`, `Vaasa`, `Wuppertal`; `Čapek`, `François`, `Kästner`, `Krleža`, `Molière`, `Châtelet`, `Gdańsk`, `Liepāja`, `Łódź`, `Mărăşeşti`, `Marañón`, `Njegoš`, `Plzeň`, `Jyväskylä` (not `Jyvaskyla`).
- **Exceptions**: traditional Hungarian forms `ORT-FOR-007`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS` (character set must include these letters; Romanian `ş/ș`, `ţ/ț` comma/cedilla variants: `WS` normalisation), `ORT-CAP-019`, `REP`
- **Sources**: [AkH12 §202 b), §214]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-006 Vowel length in foreign derivatives; AkH12 length changes
- **Statement**: In part of the common foreign words a long stem-internal vowel alternates with a short one before foreign derivational suffixes (`-ikus`, `-ista`, `-itás`, `-izál`, `-izmus`); in some words the long vowel is kept in the derivatives, not always consistently (`ORT-LEN-008`). Separately, the AkH12 dictionary changed the length of several loanword spellings (dictionary data, not in §206).
- **Conditions**: foreign stem + learned suffix (lexical per word); loanwords listed.
- **Output**: alternation: `passzív` -> `passzivitás`, `periódus` -> `periodikus`, `reális` -> `realista`, `realizál`, `realizmus`, `típus` -> `tipikus`, `tipizál`; kept: `akadémia` -> `akadémikus`, `akadémizmus`, `akadémista`, `esztéta` -> `esztétika`, `esztétikum`, `esztétikus`. AkH12 dictionary shortened: `immunis` (was `immúnis`), `irreverzibilis`, `nüansz`, `okarina`, `revans`, `reverzibilis`, `rubeola`; lengthened: `árbóc`, `bédekker`, `biennálé`, `búra`, `fantáziál`, `sámánizmus`, `sarlatánizmus`, `sátánizmus`, `videóklip`, `videófilm`, `videókazetta`, `videofelvétel` -> `videófelvétel`, `videómagnó`, `videóreklám`, `videótelefon` (but `videogenerátor`, `videojel`, `videoprocesszor`, `videotechnika`, `videotéka` unchanged); variants `advent~ádvent`, `hozsanna~hozsánna`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: norm version
- **Interactions**: `ORT-LEN-008`, `ORT-TOG-047`
- **Sources**: [AkH12 §28, §206]; dictionary changes: [HUWIKI-AKH12CH]
- **Validation**: attestation
- **Confidence**: `normative` (alternation); dictionary change list `consensus` (secondary source, verify against the AkH12 dictionary)

### ORT-FOR-007 Hungarian forms of foreign names
- **Statement**: A few prominent historical/literary personal names are used in traditional Hungarian form (fully, or with Hungarian given name and order but original family name); many countries, regions, mountains, waters and cities have Hungarian names or Hungarianised forms; the source-language forms may be used in geography, libraries, documentation, tourism, post and on maps. Traditional forms of names from non-Latin scripts are kept even when deviating from current transliteration, some only in fixed expressions.
- **Conditions**: names with a Hungarian exonym or traditional form.
- **Output**: `Brankovics György`, `Husz János`, `Kálvin János`, `Kolumbusz Kristóf`, `Luther Márton`, `Marx Károly`; `Ausztrália`, `Franciaország`, `Hispánia`, `Itália`, `Svájc`, `Alsó-Ausztria`, `Burgundia`, `Moldva`, `Szicília`, `Alpok`, `Kordillerák`, `Sziklás-hegység`, `Vezúv`, `Csendes-óceán`, `Elba`, `Odera`, `Rajna`, `Szent Lőrinc-folyó`, `Temze`, `Bécs`, `Eperjes`, `Isztambul`, `Koppenhága`, `Krakkó`, `Nagyvárad`, `Nápoly`, `Párizs`, `Szabadka`; source forms: `İstanbul`, `København`, `Kraków`, `Napoli`, `Oradea`, `Paris`, `Prešov`, `Subotica`, `Wien`; traditional: `Anyegin`, `Herkules`, `Krőzus`, `Athén`, `Babilon`, `Kairó`, `Peking`, `Himalája`, `Kréta`, `Néva`; fixed expressions: `valóságos Krőzus` vs `Kroiszosz`; `Potemkin-falvak`, `Patyomkin cirkáló` vs `Potyomkin`; AkH12 dictionary: `Vietnám`, `Konstanca`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: exonym lexicon; register/domain
- **Interactions**: `ORT-FOR-017`
- **Sources**: [AkH12 §207-§210]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-008 Units named after persons
- **Statement**: Unit names derived from proper names use only the basic Latin alphabet (no accents or diacritics), lowercase, sometimes dropping letters.
- **Conditions**: unit names.
- **Output**: `Ampère` -> `amper`; `Joule` -> `joule`; `Henry` -> `henry`; `Newton` -> `newton`; `Pascal` -> `pascal`; `Volta` -> `volt`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: unit lexicon
- **Interactions**: `ORT-CAP-026`, `ORT-ABBR-006`
- **Sources**: [AkH12 §214]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-009 -i derivative of foreign place names ending in i or y
- **Statement**: Foreign place names ending in `i` take `-i` with a single `i` (`helsinki`, `pompeji`), but a name spelled with `ii` keeps it (`hawaii`); a one-word name ending in `y` (letter `y` alone representing [i]) takes `-i` directly (`coventryi`, `vichyi`); if final [i] is written by a letter group (`ey` in AkH12; `ay`, `uy` by inference via `ORT-FOR-012`), the `-i` takes a hyphen (`sydney-i`).
- **Conditions**: foreign place name + `-i`.
- **Output**: as stated.
- **Examples**: `helsinki (olimpia)`, `pompeji (ásatások)`, `hawaii (nyaralás)`, `coventryi`, `vichyi`, `sydney-i`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: whether final [i] is spelled by a single `y` or by a group (`ey`, `ay`, `uy`)
- **Interactions**: `ORT-SUFX-004`, `ORT-FOR-012`; corpus has `sydneyi` (bucket 3): nonstandard
- **Sources**: [AkH12 §215]
- **Validation**: goldens; attestation
- **Confidence**: `normative`

### ORT-FOR-010 Final a, e, o, ö lengthen where pronunciation lengthens
- **Statement**: Suffixes attach directly to foreign-spelled words; a final `a`, `e`, `o`, `ö` (pronounced) is written `á`, `é`, `ó`, `ő` before suffixes and compound members where Hungarian pronunciation lengthens it; not before `-i` and `-szerű` (and other non-lengthening elements); a suffix-hyphenated multiword name also lengthens (`Victor Hugó-i`).
- **Conditions**: foreign-spelled word ending in a pronounced `a/e/o/ö`.
- **Output**: lengthened vowel letter per `ORT-SUFX-001`/`ORT-SUFX-002`.
- **Examples**: `signorina` -> `signorinák`; `Sinaia` -> `Sinaiában`, `sinaiai`; `Coca-Cola` -> `Coca-Colát`, `Coca-Cola-szerű`; `campanile` -> `campanilén`, `campanileszerű`; `Cicero` -> `Cicerót`; `Goethe` -> `Goethének`, `goethés`, `goethei`; `allegro` -> `allegróban`, `allegrós`; `Oslo` -> `Oslóban`, `oslói`; `Malmö` -> `Malmőből`, `malmői`; `Victor Hugó-i`.
- **Exceptions**: final letter silent (French `e` in `Voltaire`): `ORT-FOR-012` (no lengthening of a silent letter); all-caps acronyms (`ORT-ABBR-015`).
- **Predictability**: `rule` given pronunciation (is the final vowel letter pronounced as a/e/o/ö)
- **Lexical input**: pronunciation of the final letter (foreign G2P lexicon)
- **Interactions**: `ORT-SUFX-001`, `ORT-SUFX-002`, `PH-G2P`
- **Sources**: [AkH12 §216 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-011 -val/-vá after foreign final consonant letters: assimilate to the pronounced consonant
- **Statement**: After a foreign-spelled word ending in a non-Hungarian consonant letter or letter group that is pronounced, `-val/-vel`, `-vá/-vé` attach directly and `v` is replaced by the Hungarian letter(s) of the last pronounced consonant, written in full after the unchanged foreign spelling (no simplification against the foreign letters: `Greenwichcsel`, `Wellsszel`).
- **Conditions**: foreign spelling; last letter(s) pronounced; no silent final letter.
- **Output**: word + consonant letters of the final sound + `al/el/á/é`.
- **Examples**: `Balzackal` ([k]), `pechhel` ([h]), `Bachhal` ([h]), `Greenwichcsel` ([cs]), `Steinbeckkel`, `Wellsszel` ([sz]), `hertzcel` ([c]), `fixszel`, `fixszé`, `ortodoxszá`, `Félixszel`; `Beatrixszal~Beatrixszel` (from the §228 hyphenation examples).
- **Exceptions**: silent final letter or unusual group: `ORT-FOR-012` (hyphen); double letter + same consonant: `ORT-FOR-013`.
- **Predictability**: `rule` given pronunciation
- **Lexical input**: pronounced final consonant (foreign pronunciation lexicon; emMorph `pron.lx1` as study reference)
- **Interactions**: `ORT-ASSIM-001`, `ORT-NAME-011`, `PH-HARM` (harmony by pronunciation)
- **Sources**: [AkH12 §216 b), §228]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-012 Silent final letter or unusual final letter group: hyphen
- **Statement**: If a foreign-spelled word ends in a silent letter, or its last pronounced sound is written by a letter group unusual in Hungarian, suffixes are always attached with a hyphen; the suffix form (harmony, linking vowel, `v`-assimilation) follows pronunciation. Derived `-i`, `-s`, `-ista`, `-izmus` forms of one-word or hyphenated names are lowercase (hyphen kept).
- **Conditions**: final letter(s) not pronounced as written in Hungarian reading. AkH12 examples: French final `e`, `t`, `x`, `eau`, `ieu`, `eur`, `ire`, English `ow`, Romanian non-syllabic final `i`; further classes (French `s`, `aux`, `oi(s)`, `ault`, English `gh`, silent `e`) are inference from the same criterion.
- **Output**: word-suffix.
- **Examples**: `guillotine-t`, `guillotine-nal`, `monsieur-nek`, `Glasgow-ban`, `Loire-on`, `Montesquieu-nek`, `Montreux-ig`, `Peugeot-t`, `Rousseau-val`, `Rousseau-nak`, `Voltaire-é`, `Iaşi-sal`, `Molière-rel`, `Dumas-nak` (§270 line-break example); derivatives: `shakespeare-i`, `voltaire-es`, `voltaire-izmus`, `bordeaux-i`, `lille-i`, `ploieşti-i`, `châlons-en-champagne-i`; not in AkH12 (magyarispell data, inference): `Renault-k`, `Georges-zsal`, `Bretagne-nyal`, `Windows-zal`. AkH11 had `châlons-sur-marne-i` and `Iaşi-nál` in the same paragraph.
- **Exceptions**: `Anjouk` (lexicalised).
- **Predictability**: `rule` given pronunciation; the silent/unusual decision is `lexical` per word (needs foreign pronunciation)
- **Lexical input**: flag `hyphen_suffix` (derivable from a pronunciation lexicon: written final letters not all mapped to pronounced Hungarian-letter values)
- **Interactions**: `ORT-HYPH-020`, `ORT-FOR-015`, `ORT-CAP-017`; magyarispell `kotojeles.awk` and `ph:` fields (study only)
- **Sources**: [AkH12 §217 a), §265 c), §270]
- **Validation**: goldens; oracle:magyarispell
- **Confidence**: `normative`

### ORT-FOR-013 Foreign names ending in a double consonant letter + same-letter suffix: hyphen
- **Statement**: A foreign proper name ending in a doubled consonant letter takes a suffix beginning with the same letter (including assimilated `-val/-vá`) with a hyphen.
- **Conditions**: foreign name ending `GG` + suffix starting with `G`.
- **Output**: `Mann-nak`, `Tallinn-nál`, `Scott-tól`, `Scott-tal`, `Falstaff-fal`, `Grimm-mel`, `Bonn-nal`, `Bonn-nál`; by the same rule (not an AkH12 example) `Mann-né`.
- **Examples**: as Output; other suffixes direct (§215): `Scottnak`, `Grimmtől` (inferred forms).
- **Exceptions**: the rule is stated for proper names only; foreign-spelled common words ending in a doubled letter are not addressed by §217 c) (inference: general simplification `ORT-ASSIM-017` applies); Hungarian-spelled `stressz` -> `stresszel` follows `ORT-ASSIM-017`.
- **Predictability**: `rule`
- **Lexical input**: proper-name flag
- **Interactions**: `ORT-NAME-012`, `ORT-HYPH-002`
- **Sources**: [AkH12 §217 c), §265 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-014 Multiword foreign names + derivational suffix
- **Statement**: A foreign proper name of two or more separately written elements takes `-i`, `-s` etc. with a hyphen on the last element, keeping original capitals (lengthening applies).
- **Conditions**: multiword foreign name + derivational suffix.
- **Output**: `Anatole France-i`, `Victor Hugó-i`, `Leonardo da Vinci-s`, `Walter Scott-os`, `Karlovy Vary-i`, `New York-i`, `Kuala Lumpur-i`, `Frankfurt am Main-i`; transliterated: `Hriszto Botev-i`, `Velikije Luki-i`.
- **Examples**: as Output.
- **Exceptions**: inflection attaches directly to the last element (`New Yorkban`, `Victor Hugónak`).
- **Predictability**: `rule`
- **Lexical input**: name span
- **Interactions**: `ORT-HYPH-017`
- **Sources**: [AkH12 §152 c), §217 b), §222, §265 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-015 Compound members after silent-letter foreign words
- **Statement**: A compound member is attached with a hyphen to a foreign common word ending in a silent letter or unusual letter group (new in AkH12: AkH11 §217 covered suffixes only); with proper-name first members the general proper-name compound hyphen applies.
- **Conditions**: foreign word (common) with `hyphen_suffix` flag + compound member.
- **Output**: `couchette-rendelés`, `lime-likőr`, `ragtime-koncert`, `barbecue-szósz` (§263 b); names: `Baudelaire-vers`, `Loire-part`, `Shakespeare-dráma`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: `hyphen_suffix` flag
- **Interactions**: `ORT-FOR-012`, `ORT-HYPH-003`
- **Sources**: [AkH12 §112, §166, §217 d), §263 b)]; [AkH11 §217]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-016 Direct attachment to foreign-spelled words; harmony by pronunciation
- **Statement**: Otherwise suffixes, including derivational `-i`, `-s`, attach directly to foreign-spelled common words and names, with suffix forms chosen by Hungarian pronunciation of the word; lowercase `-i`/`-s` derivatives of one-word names attach directly; Hungarian-spelled foreign words and names follow Hungarian rules.
- **Conditions**: foreign-spelled word without `ORT-FOR-012`/`ORT-FOR-013` conditions.
- **Output**: direct.
- **Examples**: `cowboynak`, `fairül`, `shillingért`, `Amundsenről`, `Stockholmnál`, `Janus Pannoniustól`, `stewardessek`, `Habsburgok`, `barrelnyi`, `horatiusi`, `verdis`, `greenwichi`, `wrocławi`, `toulouse-lautreci`; Hungarian-spelled: `frontálisan`, `koktélt`, `menedzsertől`, `riporttal`, `Milánóig`, `Prágában`, `arabeszkek`, `hobbija`, `Huszé`, `kálvini`, `krómoz`, `löncsöl`, `nikotinos`, `hobbibolt`, `riportalany`, `Itália-szerte`, `Temze-part`.
- **Exceptions**: `ORT-FOR-009` to `ORT-FOR-015`.
- **Predictability**: `rule`
- **Lexical input**: pronunciation for harmony (`Word` [vörd] -> front; magyarispell `magas2` list)
- **Interactions**: `PH-HARM`, `PH-G2P`
- **Sources**: [AkH12 §211, §215]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-017 Transliteration principles
- **Statement**: In literature, press and school publications, names and words from non-Latin-script languages are transliterated into the Hungarian alphabet (`ORT` letters only), directly from the source language where possible, replacing sounds (Chinese) or letters and sounds together (Russian, Arabic, Greek) by language-specific codified correspondences; the result is read with Hungarian letter values. Whether Hungarian transliteration is applied also depends on the established practice for the source language. Established non-regular forms keep their traditional form. In sciences, tourism, post, higher education, popular science and maps, official national or international romanisations may be used; libraries follow their own standards.
- **Conditions**: non-Latin source script.
- **Output**: Hungarian transliteration per language table (`ORT-FOR-018` to `ORT-FOR-023`) or traditional form (`ORT-FOR-007`).
- **Examples**: Greek: `Hérakleitosz`, `Odüsszeusz`, `Periklész`, `Platón`, `Délosz`, `Thébai`, `khitón`, `Evdokía`, `Venizélosz`, `Iráklion`, `Kavála`; Russian: `Gogol`, `Tolsztoj`, `Irtis`, `Kijev`, `Seremetyjevo`, `szputnyik`; Chinese: `Po Csü-ji`, `Szun Jat-szen`, `Hupej`, `Lancsou`, `Sanghaj`, `Szecsuan`; Japanese: `Hokuszai`, `Kuroszava`, `Hirosima`, `Macujama`, `Nagoja`, `ikebana`; Arabic: `Ahmed`, `Dzulfikár`, `Szaladin`, `Báb-el-Mandeb`, `Iszmáilíja`, `Marrákes`, `bektási`, `hidzsra`.
- **Exceptions**: domain-specific romanisation (ISO, pinyin, Hepburn) in the domains listed: accept as variants.
- **Predictability**: `rule` (per-language tables) + `lexical` (traditional forms)
- **Lexical input**: source-script form; domain/register
- **Interactions**: `ORT-FOR-007`, `ORT-FOR-024`
- **Sources**: [AkH12 §202 c), §205, §210, §218-§221]; detailed tables in academic publications [CSNMH85], [KNMH81], [FNH98], [OH04]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-FOR-018 Russian to Hungarian transliteration
- **Statement**: Russian Cyrillic is transliterated letter by letter with positional rules for palatalised `д т н`, iotated vowels and the soft sign; vowel length is not marked; doubled consonants are kept.
- **Conditions**: Russian source.
- **Output**:

  | Cyrillic | Hungarian | positional rule |
  |---|---|---|
  | а | a | |
  | б | b | |
  | в | v | |
  | г | g | `v` in adjective/pronoun endings `-ого`, `-его` |
  | д | d | `gy` before е ё и ю я ь |
  | е | e | `je` word-initially and after a vowel, ь, ъ |
  | ё | jo | `o` after ж ч ш щ; after д т н: `gyo tyo nyo`; after other consonants `jo` (`Szemjonov`) |
  | ж | zs | |
  | з | z | |
  | и | i | after ь: `ji` |
  | й | j | endings `-ий`, `-ый` -> `-ij` |
  | к | k | |
  | л | l | ль -> `l` |
  | м | m | |
  | н | n | `ny` before е ё и ю я ь |
  | о | o | |
  | п | p | |
  | р | r | |
  | с | sz | |
  | т | t | `ty` before е ё и ю я ь |
  | у | u | |
  | ф | f | |
  | х | h | |
  | ц | c | |
  | ч | cs | |
  | ш | s | |
  | щ | scs | |
  | ъ | - | not written; a following е ё ю я is written `je jo ju ja` by their own rows |
  | ы | i | |
  | ь | - | not written; palatalises preceding д т н (`gy ty ny`); before о: `j`; a following е и ё ю я takes `j` by its own row (`je ji jo ju ja`) |
  | э | e | |
  | ю | ju | after д т н: `gyu tyu nyu` |
  | я | ja | after д т н: `gya tya nya` |

- **Examples**: `Толстой` -> `Tolsztoj`; `Чехов` -> `Csehov`; `Достоевский` -> `Dosztojevszkij`; `Чайковский` -> `Csajkovszkij`; `Хрущёв` -> `Hruscsov`; `Семёнов` -> `Szemjonov`; `Путин` -> `Putyin`; `Медведев` -> `Medvegyev`; `Тургенев` -> `Turgenyev`; `Шереметьево` -> `Seremetyjevo`; `Ельцин` -> `Jelcin`; `Наталья` -> `Natalja`; `Ольга` -> `Olga`; `Горький` -> `Gorkij`; `Великие Луки` -> `Velikije Luki`; traditional exceptions: `Lenin`, `Néva`, `Moszkva`.
- **Exceptions**: traditional forms (`ORT-FOR-007`); combinations after ъ and before и after other consonants follow the full tables in [CSNMH85] (Open questions).
- **Predictability**: `rule` + `lexical` (traditional)
- **Lexical input**: Cyrillic source; morphological ending detection for `-ого/-его`
- **Interactions**: `ORT-FOR-024`, `PH-HARM` (`Csajkovszkijjal`)
- **Sources**: [AkH12 §218-§219]; [CSNMH85]; [OH04 pp. 252-253]; [HUWIKI-CYR]; [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens; `needs:primary-source` (CSNMH85). **Not corpus-arbitrable**: a corpus shows which spellings writers use, not which positional rule CSNMH85 states, and the Hungarian spellings of Russian names in the corpus are themselves downstream of the same Wikipedia-mediated summary this table rests on. `corpus` is therefore not a validation route for this rule.
- **Confidence**: `consensus` (table from secondary sources consistent with AkH12 examples; verify against CSNMH85)

### ORT-FOR-019 Other Cyrillic-script languages
- **Statement**: Ukrainian, Belarusian, Bulgarian, Macedonian and Serbian Cyrillic have their own Hungarian tables (e.g. Ukrainian `г` -> `h`, `ґ` -> `g`, `є` -> `je`, `ї` -> `ji`, `и` -> `i`); Serbian names are commonly written in their Latin-script form (`Đorđe Branković`, cited in §207 as the source of `Brankovics György`; the Latin-script practice is not stated in AkH12, inference).
- **Conditions**: non-Russian Cyrillic sources.
- **Output**: per-language table (not reproduced).
- **Examples**: `Hriszto Botev` (Bulgarian `Христо Ботев`).
- **Exceptions**: -
- **Predictability**: `rule` (tables exist) - not encoded here
- **Lexical input**: source language identification
- **Interactions**: `ORT-FOR-018`
- **Sources**: [AkH12 §207, §220, §222]; [CSNMH85]
- **Validation**: -
- **Confidence**: `consensus` (partial; see Open questions)

### ORT-FOR-020 Greek transliteration
- **Statement**: Ancient Greek is transliterated by letters: `η` -> `é`, `ω` -> `ó`, `υ` -> `ü` (in diphthongs `αυ` -> `au`, `ευ` -> `eu`, `ου` -> `u`), `θ` -> `th`, `φ` -> `ph`, `χ` -> `kh`, `ψ` -> `psz`, `ξ` -> `x`, `σ/ς` -> `sz`, `ζ` -> `z`, `γγ` -> `ng`, rough breathing -> `h`, doubled letters kept (`σσ` -> `ssz`), final `-ος` -> `-osz`. Modern Greek is transliterated by modern pronunciation with the stressed vowel written long (`í é á ó ú`).
- **Conditions**: Greek source; period (ancient vs modern) decides the system.
- **Output**: ancient: `Hérakleitosz`, `Odüsszeusz`, `Periklész`, `Platón`, `Délosz`, `Thébai`, `khitón`, `Plutarkhosz`, `Szapphó`, `Arisztotelész`; modern: `Evdokía`, `Venizélosz`, `Iráklion`, `Kavála`.
- **Examples**: as Output.
- **Exceptions**: traditional Latinate forms (`Herkules`, `Athén`, `Kréta`).
- **Predictability**: `rule` (letter mapping) + `lexical` (stress for modern Greek from the source accent mark)
- **Lexical input**: period; modern Greek accent (tonos) position
- **Interactions**: `ORT-FOR-024`
- **Sources**: [AkH12 §219]; [OH04]
- **Validation**: goldens
- **Confidence**: `consensus` (mapping inferred from AkH12 examples and OH; full table to verify)

### ORT-FOR-021 Chinese transcription (Hungarian popular system)
- **Statement**: Chinese is transcribed by sound in the Hungarian popular system; syllables of personal names are joined by hyphens with only the first syllable capitalised; place names are usually written solid. Initial correspondences from pinyin: `b` -> `p`, `d` -> `t`, `g` -> `k`, `j q zh ch` -> `cs`, `z` -> `c`, `r` -> `zs`, `s` -> `sz`, `sh` -> `s`, `x` -> `hsz`, `w` -> `v`, `y` -> `j`; finals: `-ong` -> `-ung`, `-ui` -> `-uj`, `-ai` -> `-aj`, `-ei` -> `-ej`, `-i` after `z c s zh ch sh r` -> `-e`.
- **Conditions**: Mandarin source; general (non-specialist) register.
- **Output**: `Sanghaj` (Shanghai), `Hupej` (Hubei), `Lancsou` (Lanzhou), `Szecsuan` (Sichuan), `Po Csü-ji` (Bai Juyi, traditional), `Szun Jat-szen` (traditional), `Mao Ce-tung` (Mao Zedong).
- **Examples**: as Output.
- **Exceptions**: traditional forms (`Peking`); pinyin accepted in specialist domains.
- **Predictability**: `rule` for initials; finals only partly encoded (Open questions)
- **Lexical input**: pinyin syllabification
- **Interactions**: `ORT-FOR-017`
- **Sources**: [AkH12 §219, §221]; [KNMH81]; [OH04]; [HUWIKI-CHIN]
- **Validation**: goldens
- **Confidence**: `contested` (secondary summary; finals such as `e` -> `ö`/`o`/`e` not verified against the primary tables)

### ORT-FOR-022 Japanese transcription
- **Statement**: Japanese is transcribed by sound with Hungarian letters: (Hepburn) `sh` -> `s`, `ch` -> `cs`, `j` -> `dzs`, `ts` -> `c`, `s` -> `sz`, `y` -> `j`, `w` -> `v`, `z` -> `z`.
- **Conditions**: Japanese source.
- **Output**: `Hirosima`, `Macujama`, `Kuroszava`, `Hokuszai`, `Nagoja`, `ikebana`.
- **Examples**: as Output.
- **Exceptions**: traditional `Tokió`; treatment of long vowels not stated in AkH12 (Open questions).
- **Predictability**: `rule` for consonants
- **Lexical input**: Hepburn or kana source
- **Interactions**: `ORT-FOR-017`
- **Sources**: [AkH12 §219]; [KNMH81]
- **Validation**: goldens
- **Confidence**: `consensus` (consonants), open for vowel length

### ORT-FOR-023 Arabic and other scripts
- **Statement**: Arabic, Persian, Hebrew, Indic and other scripts have codified Hungarian transcription in academic publications; AkH12 gives only examples (Arabic long vowels written long: `Iszmáilíja`, `Marrákes`, `Dzulfikár`; `dzs` for `j`: `hidzsra`; article joined by hyphens in place names: `Báb-el-Mandeb`).
- **Conditions**: these sources.
- **Output**: lexicon of transcribed names; no generative table here.
- **Examples**: as Statement.
- **Exceptions**: -
- **Predictability**: `lexical` (for this library)
- **Lexical input**: transcribed name lexicon
- **Interactions**: `ORT-FOR-017`
- **Sources**: [AkH12 §219-§220]; [KNMH81]; [FNH98]
- **Validation**: goldens
- **Confidence**: `normative` (examples only)

### ORT-FOR-024 Suffixing transliterated names and words
- **Statement**: Transliterated names and words take suffixes by the rules for Hungarian words and names (direct attachment, lengthening, assimilation by the Hungarian letters, hyphen for multiword derivatives).
- **Conditions**: transliterated forms.
- **Output**: `Tobrukig`, `Szöulnál`, `Csehovtól`, `Csajkovszkijjal`, `Plutarkhosszal`, `száriban`, `tatamin`, `Szapphóé`, `kamikazék`, `arisztotelészi`, `kuvaiti`, `tel-avivi`, `Hriszto Botev-i`, `Velikije Luki-i`, `dzsúdózna`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-NAME-009`, `ORT-ASSIM-001`, `ORT-HYPH-017`
- **Sources**: [AkH12 §222]
- **Validation**: goldens
- **Confidence**: `normative`

## ABBR: abbreviations, symbols, acronyms

Terminology: abbreviation (rövidítés) = written short form read as the full word(s) (`u.` = `utca`); symbol/unit sign (jel) = standardised sign read as the full word (`km`, `%`); acronym (mozaikszó) = form read from its letters: letter acronym (betűszó) from initials (`MÁV`, `OTP`, `tv`), blend (szóösszevonás) from word parts (`Mahart`, `trafó`).

### ORT-ABBR-001 Abbreviations are read as the full form; article choice
- **Statement**: Abbreviations and abbreviation-like signs and codes live in writing and are pronounced as the full word; the definite article is `az` before abbreviations beginning with a vowel letter and `a` before a consonant letter (following the full word's initial); acronyms are pronounced from their written form (letter names or as a word) and the article follows that pronunciation.
- **Conditions**: article + abbreviation/acronym.
- **Output**: abbreviation: by full word; acronym: by spoken form.
- **Examples**: `u.` [utca], `km` [kilométer], `Bp.` [budapest]; `az igh.`, `az i. m.`, `a szerk.`, `a f. k.`, `a mb. ea.`; acronyms: `OTP` [ó-té-pé] -> `az OTP`; `MÁV` [máv] -> `a MÁV`; `a Mahart`, `a tb`, `az FTC` [ef-té-cé], `az MTA` [em-té-á], `az Iposz`, `az áfa`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: full form of the abbreviation; spoken form of the acronym (`ORT-ABBR-018`)
- **Interactions**: `PND-ART`; Lightproof `a/az` before letter labels by letter name (study only)
- **Sources**: [AkH12 §276]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-002 Form of abbreviations
- **Statement**: A simple or compound word abbreviates to one letter group; a phrase of separately written words abbreviates to as many period-marked elements as it has words, separated by spaces; some multiword abbreviations are written solid.
- **Conditions**: abbreviation formation.
- **Output**: `ált.`, `gk.`, `rkp.`, `Szfv.`; `a. m.`, `i. sz.`, `Kr. e.`, `s. k.`; solid: `stb.`, `vö.`, `NB.`
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (abbreviation inventory)
- **Lexical input**: abbreviation list (`ORT-ABBR-019`)
- **Interactions**: `PUN-ABBR` (spacing, non-breaking space), tokenisation
- **Sources**: [AkH12 §277 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-003 Letter case of abbreviations
- **Statement**: Abbreviations usually keep the lower/upper case of the abbreviated word(s), but many do not; a lowercase abbreviation at sentence start gets a capital; standardised forms (units, symbols) are never changed, not even in all-capitals text.
- **Conditions**: abbreviation rendering.
- **Output**: `c.`, `m.`, `kb.`, `mfszt.`, `m. v.`, `r. k.`, `D.` (Debrecen), `Mo.`; not following base: `Ft`, `ÉNy`, `ÉKsz.`; sentence start: `Csüt. délben érkezünk.`, `Kb. húszan leszünk.`; all caps: `A KÉSZÍTMÉNY ÁRA 150 Ft`, `ALAPTERÜLET: 100 m²`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (case per abbreviation) + `rule` (sentence start, protected symbols)
- **Lexical input**: abbreviation list with case; symbol flag
- **Interactions**: `ORT-CAP-007`, `WS-CASE` (case conversion must skip protected symbols)
- **Sources**: [AkH12 §277 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-004 Period after abbreviations
- **Statement**: Most abbreviations end in a period.
- **Conditions**: abbreviation not in `ORT-ABBR-005`/`ORT-ABBR-006` classes.
- **Output**: `a.` (alatt, alsó), `dec.`, `é. n.`, `Salg.`, `dr.`, `ifj.`, `özv.`, `u.`
- **Examples**: as Output.
- **Exceptions**: `ORT-ABBR-005`, `ORT-ABBR-006`, acronyms (`ORT-ABBR-013`).
- **Predictability**: `rule` + `lexical`
- **Lexical input**: abbreviation list
- **Interactions**: `PUN-ABBR` (sentence-final abbreviation period not doubled), sentence segmentation
- **Sources**: [AkH12 §267 b), §278]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-005 Abbreviations without period
- **Statement**: No period after: compass points (`D`, `ÉK`), international vehicle codes (`H`), chemical symbols (`S`, `Cl`), physical quantity symbols (`a`, `v`), mathematical notation (`log`, `sin`), abbreviations whose end is a complete compound member (`uő`, `Bföldvár`, `Bpest`, `Szfvár`); in dictionaries and lexicons with masses of abbreviations the period may be dropped (list them separately: `nm`, `rég`, `ol`).
- **Conditions**: listed classes.
- **Output**: no period.
- **Examples**: as Statement; also `db`, `ffi`, `Ft`, `vki`, `vmi` (list §284).
- **Exceptions**: -
- **Predictability**: `rule` (classes) + `lexical`
- **Lexical input**: class of the abbreviation
- **Interactions**: `ORT-ABBR-019`
- **Sources**: [AkH12 §281, §284]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-006 Units of measurement (SI)
- **Statement**: Unit symbols take no period; prefix and unit symbol are written without space or sign between them; a space separates the number from the unit symbol, except the geometric degree sign; some symbols use Greek letters, special signs or exponents.
- **Conditions**: unit symbols.
- **Output**: `cm`, `m`, `km`, `dl`, `l~L`, `dkg~dag`, `kg`, `min`, `GHz`, `kB`, `MB`, `μm`, `kΩ`, `°C`, `47° 13′ 5″`, `m²`, `km²`, `m³`, `cm³`; `100 km`, `1 kg`, `10 kWh`, `120 km/h`, `220 W`, `23 °C`; degree in geometry: `45°`, `90°`.
- **Examples**: as Output.
- **Exceptions**: geometric degree unspaced.
- **Predictability**: `rule`
- **Lexical input**: unit symbol inventory (SI)
- **Interactions**: `PUN-NUMF-007` (spacing, non-breaking space), `ORT-ABBR-010`
- **Sources**: [AkH12 §275, §279]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-007 Abbreviation or symbol in a compound
- **Statement**: See `ORT-HYPH-006`.
- **Conditions**: abbreviation/symbol + full word.
- **Output**: `mm-beosztás`, `fszla.-kivonat`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-HYPH-006`
- **Sources**: [AkH12 §283]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-008 Currencies
- **Statement**: Currency units follow unit rules (except for prefixes); foreign currencies are mostly named in Hungarian (`dollár`, `font`, `frank`, `euró`, `jen`, `korona`, `rubel`); signs are abbreviations (`Kr`, `Fr`; `Ft` in §284) or special signs (`$`, `£`, `€`, `¥`); in finance the ISO three-letter codes are used (`HUF`, `GBP`, `USD`, `AUD`, `EUR`).
- **Conditions**: currency mentions.
- **Output**: `2890 Ft`, `Ft-ot`, `Ft-tal`, `€-ért` (spoken `euróért`), `USD-ben` (spoken [ú-es-dé]); suffixed forms are not AkH12 examples except `Ft-ot`, `Ft-tal` (§282).
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: currency sign -> spoken name
- **Interactions**: `ORT-ABBR-010`
- **Sources**: [AkH12 §280]
- **Validation**: goldens
- **Confidence**: `normative` (the ISO-code suffix examples are `consensus`)

### ORT-ABBR-009 Suffixes on abbreviations and symbols: hyphen, spoken form
- **Statement**: Suffixes are attached to abbreviations and signs with a hyphen, in the form they would take on the spoken full word: harmony, assimilation, linking vowels are all written as for the full word; the abbreviation period is kept before the hyphen.
- **Conditions**: abbreviation/sign + suffix.
- **Output**: `-` + suffix allomorph of the full word.
- **Examples**: `u.-ban` (utcában), `Ft-ot`, `Ft-tal` (forintot, forinttal), `ker.-nek` (kerületnek), `ÉK-en` (északkeleten), `K-ot`, `K-mal` (káliumot, káliummal), `m²-enként`, `m²-es` (négyzetméterenként, -es), `°C-on`, `°C-kal`, `°C-os`, `Bp.-en`, `Bp.-tel`, `10%-os`, `6%-kal`, `a 6. §-ban`, `3°-kal`, `Kft.-nél`.
- **Exceptions**: -
- **Predictability**: `rule` given the full form
- **Lexical input**: full spoken form of each abbreviation/sign (ambiguous readings resolved by context, e.g. `kg` kilogramm/kiló, `sz.` szám/század)
- **Interactions**: `ORT-ASSIM-007`, `ORT-ABBR-010`, `ORT-HYPH-020`, `NOM`
- **Sources**: [AkH12 §82 f), §265 d), §275, §282]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-010 Spoken forms of frequent signs for suffixing
- **Statement**: The suffix allomorph is computed from the spoken full word; the frequent signs map as follows.
- **Conditions**: sign + suffix.
- **Output**:

  | sign | spoken | acc. | instr. | adj. `-s` | allative | inessive |
  |---|---|---|---|---|---|---|
  | `%` | százalék | `%-ot` | `%-kal` | `%-os` | `%-hoz` | `%-ban` |
  | `§` | paragrafus | `§-t` | `§-sal` | `§-os` | `§-hoz` | `§-ban` |
  | `°` | fok | `°-ot` | `°-kal` | `°-os` | `°-hoz` | `°-ban` |
  | `°C` | Celsius-fok | `°C-ot` | `°C-kal` | `°C-os` | `°C-hoz` | `°C-ban` |
  | `Ft` | forint | `Ft-ot` | `Ft-tal` | `Ft-os` | `Ft-hoz` | `Ft-ban` |
  | `€` | euró | `€-t` | `€-val` | `€-s` | `€-hoz` | `€-ban` |
  | `$` | dollár | `$-t` | `$-ral` | `$-os` | `$-hoz` | `$-ban` |
  | `km` | kilométer | `km-t` | `km-rel` | `km-es` | `km-hez` | `km-ben` |
  | `m` | méter | `m-t` | `m-rel` | `m-es` | `m-hez` | `m-ben` |
  | `kg` | kilogramm / kiló | `kg-ot` | `kg-mal` | `kg-os` | `kg-hoz` | `kg-ban` |
  | `kg` (measured) | the **kilogramm reading dominates at 98.8%**: `kg-ot`/`kg-os` 19,315 + `kg-mal` 867 against the kiló reading `kg-t`/`kg-s` 234 | `kg-ot` | `kg-mal` | `kg-os` | - | - |
  | `dkg` | dekagramm / deka; the full-name reading dominates at 97.5% (`dkg-ot`/`dkg-os` 238 : `dkg-t` 6) | `dkg-ot` | - | `dkg-os` | - | - |
  | `cm` | centiméter / centi; `cm-es` 9,294, `cm-t` 1,990, `cm-rel` 1,309, and the short-name instrumental `cm-terrel` has 0 tokens | `cm-t` | `cm-rel` | `cm-es` | `cm-hez` | `cm-ben` |
  | `l` | liter | `l-t` | `l-rel` | `l-es` | `l-hez` | `l-ben` |
  | `m²` | négyzetméter | `m²-t` | `m²-rel` | `m²-es` | `m²-hez` | `m²-ben` |
  | `Bp.` | Budapest | `Bp.-et` | `Bp.-tel` | - | `Bp.-hez` | `Bp.-en` (superessive) |
  | `u.` | utca | `u.-t` | `u.-val` | `u.-i` | `u.-hoz` | `u.-ban` |

- **Examples**: as table (AkH12 attests `%-os`, `%-kal`, `§-ban`, `°-kal`, `°C-on`, `°C-kal`, `°C-os`, `Ft-ot`, `Ft-tal`, `m²-enként`, `m²-es`, `Bp.-en`, `Bp.-tel`, `u.-ban`).
- **Exceptions**: `kg` spoken `kiló` gives `kg-t`, `kg-val`: both readings occur, but in edited text the full-name reading is 98.8% of tokens, so it is the default (`PND-MEAS-003`); the `-i` adjective of `Bp.` is not codified (write `budapesti`); `€-t` [eurót] lengthening does not arise (ó already long); `§-t` from `paragrafust`: a linking-vowel-less accusative after `s` (`NOM-ACC`).
- **Predictability**: `rule` given spoken form; `lexical` for alternative readings
- **Lexical input**: spoken form lexicon for signs
- **Interactions**: `NOM-ACC`, `NOM-CASE-003`, `PH-HARM`
- **Sources**: [AkH12 §275, §279-§282]; [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens; attestation (regex scan over four sources; these are hyphenated tokens, absent from both word lists)
- **Confidence**: `normative` for attested cells; `consensus` for derived cells; `measured` for the default reading of `kg` and `dkg` (`OQ-07-016` resolved)

### ORT-ABBR-011 Letter acronyms: case
- **Statement**: Proper-noun letter acronyms (mostly for multiword institution names), Hungarian or foreign, are written in all capitals (every letter of a digraph too); common-noun letter acronyms are lowercase; in sciences multiword common terms are also abbreviated in capitals, and colloquial capitalised common acronyms are not wrong; letter names of common acronyms may be written out as a word.
- **Conditions**: letter acronym.
- **Output**: `BKK`, `DE`, `KSH`, `ELTE`, `EMKE`, `KMKSZ`, `TESZ`, `UNESCO`, `GYSEV`; lowercase: `tbc`, `tv`, `tb`; scientific: `CB`, `DNS`, `EKG`, `PVC`, `URH`; colloquial: `BÚÉK` (AkH12, was `B. ú. é. k.`), `TDK`, `TKM`; spelled-out letter names: `pévécé`, `tébécé`, `tévé`. AkH11 §283 also listed `áfa`, `tsz`, `téesz` and `NE` (dropped from the AkH12 paragraph).
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (acronym inventory)
- **Lexical input**: acronym list with type
- **Interactions**: `ORT-CAP-003`, `ORT-ABBR-014`
- **Sources**: [AkH12 §8, §285]; [AkH11 §283]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-012 Blends: case
- **Statement**: Proper-noun blends are written like proper names (capital initial, rest lowercase); common-noun blends (and foreign ones borrowed as such) are all lowercase; `közért` (common noun from a proper blend) is lowercase. AkH11 §284 a) also allowed all-caps advertising forms (`MAVAD`, `ALITALIA`); AkH12 dropped this.
- **Conditions**: blend (szóösszevonás).
- **Output**: `Helir`, `Mavad`, `Műpa`, `Mahart`, `Malév`, `Agrocoop`, `Intransmas`, `Röltex`, `Iposz`, `Magyosz` (only `Helir`, `Mavad`, `Műpa` in §286; others from §276, §287-§288 and AkH11); common: `belker`, `gyes`, `kisker`, `radar`, `trafó`, `közért`; (`áfa` is a word-read common letter acronym, `ORT-ABBR-016`, listed as a letter acronym in AkH11 §283 b)); `Trafó` (arts centre) vs `trafó` (transformer).
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: blend inventory
- **Interactions**: `ORT-ABBR-016`
- **Sources**: [AkH12 §286]; [AkH11 §284]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-013 No periods in acronyms
- **Statement**: Acronyms take no period between or after their elements.
- **Conditions**: letter acronyms and blends.
- **Output**: `MTI`, `ENSZ`, `NATO`, `LSD`, `tbc`, `Magyosz`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: acronym type
- **Interactions**: `ORT-ABBR-004`
- **Sources**: [AkH12 §287]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-014 Suffixes on letter acronyms: hyphen, spoken form
- **Statement**: Suffixes are attached to letter acronyms (upper- and lowercase) with a hyphen in the form required by the acronym's pronunciation (letter names spelled out or reading as a word), with harmony, assimilation and linking vowels written; capitalised letter acronyms keep their form in derived adjectives too.
- **Conditions**: letter acronym + suffix.
- **Output**: `a BKK-nál`, `a MÁV-ot`, `az ENSZ-szel`, `tbc-s`, `tv-zik~tévézik`, `DNS-sel`, `ENSZ-beli`, `MAFC-os`, `TDK-s`, `USA-beli`, `ENSZ-ért`, `NATO-hoz`, `az MTA-tól`.
- **Examples**: as Output.
- **Exceptions**: word-read common acronyms written in lowercase (`áfa`, `gyes`): `ORT-ABBR-016`.
- **Predictability**: `rule` given spoken form
- **Lexical input**: spoken form (`ORT-ABBR-018`)
- **Interactions**: `ORT-ASSIM-007`, `ORT-HYPH-020`, `ORT-SPLIT-018`
- **Sources**: [AkH12 §288 a)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-015 No lengthening in all-caps proper acronyms; lengthening in blends
- **Statement**: A final `a`, `e`, `o` of an all-capitals proper-noun letter acronym is not lengthened in writing before suffixes (even though it may be pronounced long); in blends not written in all capitals the final `a`, `e`, `o` is lengthened.
- **Conditions**: acronym-final vowel letter + lengthening suffix.
- **Output**: `az MTA-nak`, `az MTA-t`, `az ELTE-n`, `az ELTE-re`, `az ETO-hoz`; blends: `stukák`, `a Hungexpónál`, `az Alitaliánál`, `áfát`, `áfáról`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: acronym type
- **Interactions**: `ORT-SUFX-001`; spoken form of `MTA` ends in [á] (letter name), so the suffix allomorph is back-harmonic vowel-final (`-nak`, `-t`, `-val`)
- **Sources**: [AkH12 §29, §288 a)-b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-016 Blends and word-read common acronyms: direct attachment
- **Statement**: Suffixes attach directly (no hyphen) to blends and to common-noun acronyms pronounced as words, as well as to spelled-out letter-name forms; a double final letter + same-letter suffix takes a hyphen; adjectives derived from proper blends are lowercase.
- **Conditions**: blend or word-read lowercase acronym.
- **Output**: `Orexszel`, `az Alitaliánál`, `gyesen (van)`, `(a) trafója`, `áfát`, `áfáról`, `tébécés`, `tévézik`; `a Keravill-lal` (AkH11 also `az Elzett-től`); `mavados`, `helirbeli`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: acronym type
- **Interactions**: `ORT-NAME-012`, `ORT-CAP-017`; AkH11 examples `Röltexszel`, `telexszel`
- **Sources**: [AkH12 §288 b)]; [AkH11 §286 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-017 Compound members on acronyms
- **Statement**: A compound member is attached with a hyphen to letter acronyms and proper-noun blends; it is written solid with common-noun blends and letter-name spellings (see `ORT-HYPH-005`).
- **Conditions**: acronym + noun member.
- **Output**: `MTI-hír`, `OTP-kölcsön`, `UEFA-torna`, `tv-közvetítés`, `TDK-dolgozat`, `URH-adás`, `Alitalia-iroda`, `Orex-üzlet`, `EU-csatlakozás`, `KSH-kimutatás`, `UNESCO-jogszabály`, `tb-járulék`, `tv-műsor`; `radarállomás`, `trafótekercselés`, `tébécészakorvos`, `tévéközvetítés`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: acronym type
- **Interactions**: `ORT-HYPH-005`, `ORT-63-008`
- **Sources**: [AkH12 §113, §289]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-ABBR-018 Pronunciation of letter acronyms
- **Statement**: A letter acronym is pronounced either by letter names or as a word (lexical); letter names in Hungarian: `a` [á], `á`, `b` [bé], `c` [cé], `cs` [csé], `d` [dé], `dz` [dzé], `dzs` [dzsé], `e`, `é`, `f` [ef], `g` [gé], `gy` [gyé], `h` [há], `i`, `í`, `j` [jé], `k` [ká], `l` [el], `ly` [elipszilon], `m` [em], `n` [en], `ny` [eny], `o`, `ó`, `ö`, `ő`, `p` [pé], `q` [kú], `r` [er], `s` [es], `sz` [esz], `t` [té], `ty` [tyé], `u`, `ú`, `ü`, `ű`, `v` [vé], `w` [dupla vé], `x` [iksz], `y` [ipszilon], `z` [zé], `zs` [zsé]. Suffix harmony and assimilation come from the last spoken letter name (or the word reading). Foreign acronyms may be read with Hungarian or source letter names (lexical); where both inventories end in a front name the choice is **orthographically inert** (`CNN-nek` 1,738 : `CNN-nak` 1, `CNN-nel` 53 : `CNN-nal` 0, `CNN-hez` 28 : `CNN-hoz` 0), so no reading decision is needed for such acronyms. Harmony is **not derivable from the final letter name** for acronyms whose spelled-out form mixes back and neutral vowels and must be lexical per acronym (`PH-G2P-010`).
- **Conditions**: acronym spoken form needed.
- **Output**: harmony and final segment from the last spoken letter name: back-vowel names `a` [á], `h` [há], `k` [ká], `q` [kú], `o`, `ó`, `u`, `ú` and `y` [ipszilon, last vowel o] -> back suffixes, vowel-final (`MTA-nak`, `USA-ban`, `KSH-hoz`, `ORFK-val`); names in `é` and `e/i` (`b c cs d dz dzs g gy j p t ty v z zs` [..é], `e é i í`) and consonant-final names `f l m n ny r s sz x` [ef el em en eny er es esz iksz] -> front unrounded suffixes (`PSG-vel`, `DNS-sel`, `ENSZ-szel`, `SZDSZ-t`); letter names `ö ő ü ű` -> front rounded (`-höz`, `-t`); word readings take harmony of the word: `NATO` [nátó] `NATO-hoz`, `MÁV` [máv] `MÁV-ot`, `ELTE` [elte] `ELTE-re`, `UNESCO` [uneszkó] `UNESCO-hoz`.
- **Examples**: `DNS-sel` [dé-en-es], `az FTC-vel` [ef-té-cé], `a BKK-nál` [bé-ká-ká], `a KSH-hoz` [ká-es-há], `OTP-t`, `OTP-vel` [ó-té-pé, front: 225 : `OTP-val` 1], `BKV-val` [bé-ká-vé, back - lexical: 500 : `BKV-vel` 17], `tv-t` [té-vé], `tbc-s`.
- **Exceptions**: the letter names of `a` in acronyms is [á]; `ny` name varies ([eny]/[enny]); `w` name varies ([dupla vé]/[vé], written solid `duplavé` 82 : `dupla vé` 12 [WC2]); foreign readings (`CNN` [szí-en-en] vs [cé-en-en], `BBC` [bí-bí-szí]) - for `CNN` the two readings give the same suffix. **The final-letter-name rule does not predict the outcome for mixed-vowel acronyms**: `BKV` and `MTV` both end in `vé` and behave oppositely (`BKV-val` 500 : 17 against `MTV-vel` 129 : 0), and `OTP` (`ótépé`) contains a back `ó` and still takes front (`OTP-vel` 225 : 1), so even the dominant-vowel account fails. Such acronyms carry a lexical harmony value.
- **Predictability**: `rule` (reading type from the shape, `PH-G2P-011`; suffix from the spoken form) + `lexical` (harmony for mixed-vowel letter-read acronyms, where both classes are accepted and the spoken form's own class is generated: `BKV-val`)
- **Lexical input**: acronym reading (letter-by-letter vs word; Hungarian vs source letter names); acronym harmony value
- **Interactions**: `PH-HARM`, `PH-G2P`, `ORT-ABBR-001`, `ORT-ABBR-014`; Lightproof letter-label harmony classes (back: `a h k o q u y`; accusative `-et` after `f m n x`) (study only)
- **Sources**: [AkH12 §276, §288]; [LP-HU] (study only); [WC2]; [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens; attestation (regex scan over 11 sources anchored on the exact acronym plus the exact suffix; these are hyphenated tokens and exist in no word list)
- **Confidence**: `consensus` (letter-name inventory); `measured` for acronym harmony, which is item-lexical rather than derivable (`OQ-02-005`, `OQ-07-016` resolved)

### ORT-ABBR-019 Standard abbreviation list
- **Statement**: AkH12 lists 125 entries (120 distinct forms; `hg.`, `m.`, `min.`, `sz.`, `üi.` twice) of common abbreviations and signs with expansions; they form the base inventory (period, case, expansion) for tokenisation and suffixing. Compared with AkH11 §282 the list was restructured, not merely extended: unit symbols moved to §279-§280, entries such as `a.`, `á. é.`, `bp.-i`, `i. h.`, `i. m.`, `k.`, `kk.`, `megh.`, `ny.`, `p.`, `r.`, `N. N.`, `X. Y.` were dropped, and entries such as `bek.`, `eü.`, `ffi`, `kkt.`, `max.`, `nyrt.`, `Pf.`, `prof.`, `pü.`, `rtg.`, `szla.`, `uő`, `üi.`, `vki`, `vmi`, `zrt.` were added.
- **Conditions**: abbreviation lookup.
- **Output**: entries such as `áll.`, `ált.`, `a. m.`, `a. n.`, `ápr.`, `aug.`, `bek.`, `Bp.`, `br.`, `bt.`, `c.`, `csüt.`, `D`, `db`, `de.`, `dec.`, `DK`, `DNy`, `dr.`, `du.`, `É`, `ÉK`, `em.`, `ÉNy`, `eü.`, `ev.`, `évf.`, `febr.`, `felv.`, `f. é.`, `ffi`, `f. hó`, `ford.`, `fszla.`, `fszt.`, `Ft`, `gimn.`, `gk.`, `gr.`, `h.`, `hg.` (hegység; herceg), `honv.`, `id.`, `i. e.`, `ifj.`, `ig.`, `igh.`, `ill.`, `isk.`, `i. sz.`, `jan.`, `júl.`, `jún.`, `K`, `kb.`, `ker.`, `kft.`, `kkt.`, `köv.`, `Kr. e.`, `Kr. u.`, `krt.`, `l.`, `l.~ld.`, `m.` (magyar; megye), `máj.`, `márc.`, `max.`, `mb.`, `mfszt.`, `min.` (minimum; miniszter), `NB.`, `nov.`, `Ny`, `nyrt.`, `o.~old.`, `okl.`, `okt.`, `olv.`, `ov.`, `özv.`, `Pf.`, `(P. H.)`, `pl.`, `prof.`, `pu.`, `pü.`, `ref.`, `r. k.`, `rt.`, `rtg.`, `s. k.`, `stb.`, `sz.` (szám; század), `szept.`, `szerk.`, `szla.`, `Szt.`, `szül.`, `t.`, `tc.`, `tel.`, `ti.`, `tkp.`, `tvr.`, `u.`, `ua.`, `ui.`, `Ui.`, `úm.`, `ún.`, `uo.`, `uő`, `üi.` (ügyintéző; ügyirat), `v.`, `vki`, `vmi`, `vö.`, `vsz.`, `zrt.`
- **Examples**: as Output.
- **Exceptions**: ambiguous entries need context (`hg.`, `m.`, `min.`, `sz.`, `üi.`, and `l.` lap vs lásd); `Ui.` vs `ui.` distinguished by case.
- **Predictability**: `lexical` (closed list of 125 entries in AkH12 §284; open in usage; spaCy (MIT) has about 590)
- **Lexical input**: abbreviation lexicon
- **Interactions**: `PUN-ABBR`, sentence segmentation
- **Sources**: [AkH12 §284]; [AkH11 §282]
- **Validation**: goldens
- **Confidence**: `normative`

## NUM: numerals, dates, times

### ORT-NUM-001 Letters or digits
- **Statement**: No general rule decides between letters and digits; practice: letters in continuous (especially literary) text for numbers expressible by a short word and for numerals with suffixes, in compounds or with postpositions; digits for longer/larger numbers and for times, amounts of money, measures, statistics; round numbers above three digits may be written as digit + word.
- **Conditions**: numeral rendering.
- **Output**: `öt`, `húsz`, `ezer`, `százezer`, `tízmillió`, `huszonötöt`, `ezernek`, `tízféle`, `hatvan után`, `harmadmagával`, `kéthetenként`; digits: `388 személy`, `65 téglát`, `du. 5 órakor`, `2890 Ft`, `15 méter szövet`; mixed (optional): `107 ezer`, `6 millió` (`ORT-NUM-011`).
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `tendency` (the norm states that no general rule exists; established practice only)
- **Lexical input**: register
- **Interactions**: `PND-CARD-003` (spell-out), `ORT-NUM-010`, `ORT-NUM-011`
- **Sources**: [AkH12 §290]
- **Validation**: -
- **Confidence**: `normative` (as a tendency statement)

### ORT-NUM-002 Suffixes on numbers in digits: hyphen, spoken form
- **Statement**: Suffixes are attached to digits with a hyphen, in the form required by the spoken numeral (its last word decides harmony, linking vowel, assimilation); the linking vowel is written.
- **Conditions**: digit string (cardinal) + suffix.
- **Output**: `-` + allomorph of the spoken last element. Last element classes (units and round elements):

  | ends in | spoken | acc. | instr. | allative | inessive | adj. `-s` | multipl. |
  |---|---|---|---|---|---|---|---|
  | 1 | egy | `-et` | `-gyel` | `-hez` | `-ben` | `-es` | `-szer` |
  | 2 | kettő | `-t` | `-vel` | `-höz` | `-ben` | `-es` | `-szer` (kétszer) |
  | 3 | három | `-at` | `-mal` | `-hoz` | `-ban` | `-as` | `-szor` |
  | 4 | négy | `-et` | `-gyel` | `-hez` | `-ben` | `-es` | `-szer` |
  | 5 | öt | `-öt` | `-tel` | `-höz` | `-ben` | `-ös` | `-ször` |
  | 6 | hat | `-ot` | `-tal` | `-hoz` | `-ban` | `-os` | `-szor` |
  | 7 | hét | `-et` | `-tel` | `-hez` | `-ben` | `-es` | `-szer` |
  | 8 | nyolc | `-at` | `-cal` | `-hoz` | `-ban` | `-as` | `-szor` |
  | 9 | kilenc | `-et` | `-cel` | `-hez` | `-ben` | `-es` | `-szer` |
  | 10 | tíz | `-et` | `-zel` | `-hez` | `-ben` | `-es` | `-szer` |
  | 20 | húsz | `-at` | `-szal` | `-hoz` | `-ban` | `-as` | `-szor` |
  | 30 | harminc | `-at` | `-cal` | `-hoz` | `-ban` | `-as` | `-szor` |
  | 40, 50, 70, 90 | negyven, ötven, hetven, kilencven | `-et` | `-nel` | `-hez` | `-ben` | `-es` | `-szer` |
  | 60, 80 | hatvan, nyolcvan | `-at` | `-nal` | `-hoz` | `-ban` | `-as` | `-szor` |
  | 00 | száz | `-at` | `-zal` | `-hoz` | `-ban` | `-as` | `-szor` |
  | 000 | ezer | `-et` | `-rel` | `-hez` | `-ben` | `-es` | `-szer` |
  | 000 000 | millió | `-t` | `-val` | `-hoz` | `-ban` | `-s` | `-szor` |
  | 000 000 000 | milliárd | `-ot` | `-dal` | `-hoz` | `-ban` | `-os` | `-szor` |
  | 0 (alone) | nulla | `-t` | `-val` | `-hoz` | `-ban` | `-s` | `-szor` |

  The last spoken word decides, i.e. the word of the lowest-order non-zero place (units, tens, `száz`, then the scale word of the lowest non-zero three-digit group): `12` -> kettő, `19` -> kilenc, `1848` -> nyolc, `2000` -> ezer, `1900` -> száz, `2014` -> négy, `110` -> tíz, `1 000 000` -> millió. Table cells other than the §265 d) and §82 f) examples are derived from the spoken numeral's paradigm (`PND-NUMINF-011`).
- **Examples**: `4-et`, `12-t`, `19-szer`, `32-en`, `67-es`, `4-gyel`, `6%-kal`, `16.25-kor`, `1848-ban`, `2014-ben`, `1838-as`, `1514-ben`; derived from the table: `100-at`, `3-mal`, `20-szal`, `5-höz`.
- **Exceptions**: `-szor/-szer` on 2 is `2-szer` (kétszer); collective `-an/-en` vs superessive `ORT-NUM-004`; ordinals `ORT-NUM-006`; dates `ORT-NUM-017`. Decimals have two live readings and therefore two correct suffixations (`1,2-vel` from `kettővel`, `1,2-del` from `tizeddel`); neither is an error, see `PND-FRAC-007` (`OQ-07-017` resolved 2026-09-16).
- **Predictability**: `rule`
- **Lexical input**: spelled-out last element (`PND-NUMINF-002`)
- **Interactions**: `ORT-ASSIM-007`, `ORT-HYPH-020`, `PH-HARM`; Lightproof numeral table (study only) agrees
- **Sources**: [AkH12 §82 f), §265 d), §298]; [AkH11 §265 e)] (same principle); [LP-HU] (study only)
- **Validation**: goldens
- **Confidence**: `normative` (principle and attested forms); `consensus` (table cells)

### ORT-NUM-003 Derivatives and compounds with digits
- **Statement**: Derivational suffixes and suffix-like second members (`-féle`, `-szerű` ...) attach directly to digits with a hyphen (`67-es`, `10-féle`); a derived adjective whose stem is a separate word (`2 éves`, `9 havi`) is written apart after digits (`ORT-NUM-012`).
- **Conditions**: digits + derivational suffix (`-s`, `-szOr`, `-nyi` ...) or `-féle`-type second member attached to the digit itself.
- **Output**: `67-es`, `80%-os`, `1914–18-as`, `10-féle`, `M7-es`; by the same rule (not AkH examples): `20-szoros`, `5-ös (busz)`, `5-nyi`; score adjective `3-0-s` (`ORT-NUM-021`).
- **Examples**: as Output.
- **Exceptions**: `2 éves`-type phrases (`ORT-NUM-012`).
- **Predictability**: `rule`
- **Lexical input**: spoken form
- **Interactions**: `ORT-NUM-002`, `ORT-TOG-041`
- **Sources**: [AkH12 §132, §265 d), §294]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-004 32-en versus 32-n
- **Statement**: The spoken form disambiguates digits with the `-n` suffixes: collective-numeral adverb `-an/-en` ('as a group of') is written with its vowel, superessive after vowel-final `kettő` is `-n`.
- **Conditions**: digits ending in 2 (kettő/két-) + `-n`.
- **Output**: `32-en` (harmincketten), `32-n` (harminckettőn); similarly `2-en` (ketten), `2-n` (kettőn).
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule` given the function
- **Lexical input**: syntactic function (collective subject/quantity 'in a group of' vs locative)
- **Interactions**: `PND-NUMINF-005` collective forms
- **Sources**: [AkH12 §265 d)]; example absent in [AkH11 §265 e)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-005 Grouping digits, decimal comma
- **Statement**: Numbers of five or more digits are grouped in threes from the right with a space (or, as a variant, a period); four-digit numbers are not grouped except when aligned in a column with numbers of five or more digits; the decimal separator is a comma without spaces.
- **Conditions**: digit strings.
- **Output**: `20 611~20.611`, `357 864~357.864`, `5 602 164~5.602.164`, `4 210` (in a column), `3,16`, `37,5`, `20 611,413`, `38,6`, `1,23`; four digits ungrouped: `2500 Ft`, `Pf. 3301`, `Budapest 1810`.
- **Examples**: as Output.
- **Exceptions**: identifiers of five or more digits (document numbers `123456HA`, phone numbers) are shown ungrouped in §294 but AkH12 states no grouping exception for them (inference: do not group identifiers). AkH11 allowed only the space.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PUN-NUMF-002` (thin/no-break space), tokenisation (period-grouped thousands vs ordinal periods)
- **Sources**: [AkH12 §274, §291 b), §293-§294]; [AkH11 §289 b)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-006 Ordinals in digits: period, kept before suffixes
- **Statement**: Arabic or Roman digits used as ordinals take a period; before a hyphenated suffix the period is kept; the suffix is the form of the spoken ordinal (`-adik/-edik/-odik`, `első`, `második`).
- **Conditions**: ordinal digits (not a date day/year: `ORT-NUM-017`).
- **Output**: `1. osztály`, `3. sor`, `1978. évi`, `a 10–12. oldalon`, `IX. kerület`; suffixed: `3.-nak` (harmadiknak), `a 8.-ba` (nyolcadikba), `a 10.-kel` (tizedikkel), `a IV.-be`, `a 128–9.-en`, `az I–IV.-ben`; derived (not AkH examples): spoken ordinal endings end in `k` (instrumental `-kal/-kel`, accusative `-at/-et`), except standalone `1.` (első: `1.-t`, `1.-vel`, `1.-nek`; `11.`, `21.` are `-egyedik`); harmony: `1. 4. 5. 7. 9. 10. 1000.` front (első, negyedik, ötödik, hetedik, kilencedik, tizedik, ezredik), `2. 3. 6. 8. 20. 30. 60. 80. 100.` back (második, but in compounds `-kettedik` is front: `12.-kel`; harmadik, hatodik, nyolcadik, huszadik, harmincadik, hatvanadik, nyolcvanadik, századik), `40. 50. 70. 90.` front (negyvenedik...), `millió` back (milliomodik).
- **Examples**: as Output.
- **Exceptions**: dates `ORT-NUM-017`; house numbers `ORT-NUM-007`. The linking vowel follows the harmony of the whole ordinal, not of the final `i`: `ötödiken` -> `5.-en`, `harmadikon` -> `3.-on`.
- **Predictability**: `rule`
- **Lexical input**: spoken ordinal
- **Interactions**: `PND-ORD`, `ORT-NUM-002`, `ORT-NUM-018`, `PUN-NUMF-009`
- **Sources**: [AkH12 §264 c), §265 d), §267 a), §292]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-007 House numbers
- **Statement**: A house number after a street name takes a period although read as a cardinal; suffixes attach after the period with a hyphen in the cardinal's form; with a letter or slash suffix no period is written.
- **Conditions**: street address.
- **Output**: `a Tutaj u. 4.-ben` (read [négyben]); `a Tutaj u. 4/B-ben~4/b-ben`; `Ménesi út 11–13.`; `Petőfi u. 8. III. 2.`
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: address context
- **Interactions**: `ORT-NUM-006`
- **Sources**: [AkH12 §264 c), §292, §294]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-008 Roman numerals; mixed designations
- **Statement**: Numbers are mostly written in Arabic digits; Roman numerals are used consistently only for ordinals in some traditional cases (rulers) and are otherwise receding, recommended where they distinguish or structure (dates, addresses, table numbers) and where professional usage requires; for volumes, congresses, centuries, districts both are correct; school classes use Arabic digits. Letter-digit combinations: `ORT-HYPH-016`.
- **Conditions**: ordinal rendering.
- **Output**: `IV. Béla`, `II. Rákóczi Ferenc`, `VII. Gergely`, `2013. IX. 12.`, `IV/3-as tábla`, `X. agyideg`, `IV. ágyéki csigolya`; both: `3. kötet~III. kötet`, `16. század~XVI. század`, `XIX. kerület~19. kerület`; school: `2. osztály`, `4. a~4/a osztály`, `10. osztály`; also both: `6. nyelvészkongresszus~VI. nyelvészkongresszus`.
- **Examples**: as Output.
- **Exceptions**: AkH11 §292 listed `XIX. kerület` and `VI. nyelvészkongresszus` among the consistently Roman cases; AkH12 moves them to the both-correct list.
- **Predictability**: `rule` (both valid where stated)
- **Lexical input**: none
- **Interactions**: `ORT-HYPH-016`, `REP` (Roman numeral vs letters `I`, `V`, `X` ambiguity)
- **Sources**: [AkH12 §294]; [AkH11 §292]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-009 Time of day
- **Statement**: In running text the words `óra` and `perc` are often written out after the digits (`10 óra 25 perckor`); when the time is given in digits only, hour and minute are separated by a period or colon without space, and suffixes attach with a hyphen (the suffix form follows the spoken last number: `ORT-NUM-002`, inference from §265 d)). Fractional-hour expressions follow `ORT-NUM-015` (`negyed kilenckor`, `fél tizenkettőig`, `háromnegyed négyre`).
- **Conditions**: clock time.
- **Output**: `10.35~10:35`; `10.35-kor~10:35-kor`, `10.35-ig~10:35-ig`, `10.35-ös~10:35-ös`, `16.25-kor`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: spoken minute value
- **Interactions**: `PUN-DATEF-012`, `ORT-NUM-002`, tokenisation (`10.35` time vs decimal)
- **Sources**: [AkH12 §265 d), §293, §300]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-010 Spelled-out numerals: solid up to 2000, hyphen groups above
- **Statement**: Cardinal and ordinal numerals written in letters are solid up to two thousand inclusive; above that only round thousands and round millions are solid; above 2000, if the number continues after the thousand (scale) word, the numeral is divided into the usual groups of three from the right, with hyphens between groups. Year numbers are exempt and written solid at any size: "Kétezren felül (kivéve az évszámokat)", "de: kétezertizennégy májusában" (§291 a)), so `kétezertizenhat` is the year and `kétezer-tizenhat` the amount. A year range takes the unspaced en dash, its suffix hyphenated after the second member (`1848–49-ben`, `1914–18-as`, §264 c), §296; `ORT-NUM-018`). The e-nyelv advisory reading that hyphenates a spelled-out year (`OQ-07-018`) contradicts §291 a) and is not followed. Whether a spelled-out numeral is a year is semantic, so a checker accepts both spellings above 2000 and generation writes the year solid only when told it is one.
- **Conditions**: spelled-out numeral value `n`.
- **Output**: split `n` into three-digit groups; `n <= 2000` -> solid; `n > 2000` with exactly one non-zero group (`ötvenháromezer`, `hatvankétmillió`) -> solid; otherwise hyphen after every scale word that is followed by further words, i.e. between non-empty groups (`hárommillió-ötszázezer`, `ötmillió-tíz`); years -> solid.
- **Examples**: `tizennégy`, `nyolcszázkilencvenhat`, `ezerhétszázhetvenkettő`, `ötvenháromezer`, `hatvankétmillió`, `nyolcszázkilencvenhatodik`, `hatmilliomodik`; `kétezer-egy`, `háromezer-tizenhat`, `negyvenhétezer-ötszázhatvanhárom`, `hétmillió-négyszázkilencvenezer-ötszázharminc`, `hárommillió-ötszáznegyvenezer-kétszázharmincnyolc`, `ötezer-nyolcszáztíz`, `tizenegyezer-huszonnyolc`, `négyezer-hatszázötödik`; year: `kétezertizennégy májusában`, but amount: `kétezer-tíz forint` (AkH12 dictionary via [HUWIKI-AKH12CH]).
- **Exceptions**: years; AkH11 §289 a) had no year exception (so `kétezer-tizennégy` for years too) and named only cardinals (`tőszámnevek`); AkH12 adds ordinals explicitly.
- **Predictability**: `rule`
- **Lexical input**: whether the numeral denotes a year (semantic)
- **Interactions**: `PND-CARD-006` (same rule), `PND-CARD-003` spell-out (CLDR RBNF lacks the >2000 hyphen rule), `ORT-HYPH-012`, `ORT-63-007`; numerals up to 2000 (`ezerkétszázhuszonöt`) and `kétezer` itself are solid
- **Sources**: [AkH12 §263 d), §264 c), §291 a), §296]; [AkH11 §289 a)]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-011 Digit + word for large round numbers
- **Statement**: The leading part of round numbers of more than three digits may optionally be written in digits followed by the scale word.
- **Conditions**: round thousands/millions/milliárds.
- **Output**: `107 ezer~107 000`, `6 millió~6 000 000`; by extension (not an AkH example): `2,5 milliárd`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule` (optional variant)
- **Lexical input**: none
- **Interactions**: `ORT-NUM-005`
- **Sources**: [AkH12 §290]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-012 Numeral + derived adjective with digits: apart
- **Statement**: With digits, numeral + derived adjective (`-i`, `-s`, `-ú/-ű`, `-nyi`) and `-nként`, `-nta/-nte` forms are always written apart (see `ORT-TOG-023`).
- **Conditions**: digit + derived word.
- **Output**: `9 havi`, `2 éves`, `20 főnyi`, `40 köbcentiméteres`, `116 napi`, `5 évente`, `15 naponként`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-TOG-023`
- **Sources**: [AkH12 §117]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-013 Approximate and distributive numbers
- **Statement**: See `ORT-HYPH-010`.
- **Conditions**: two numerals (letters or digits) expressing approximation, alternative, or distributive repetition.
- **Output**: `egy-két (ember)`, `nyolc-tíz (napra)`, `5-6 (darab)`, `10-12 (éves)`; distributive: `2-2 (almát)`, `húsz-húsz (deka)`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: reading
- **Interactions**: `ORT-HYPH-010`
- **Sources**: [AkH12 §263 i)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-014 Period after the year
- **Statement**: The year number (or year group) takes a period when it is not in a grammatical relation with the following month name or other time word. The norm's examples: month name unsuffixed or with a non-possessive case suffix, `-i` adjectives (`áprilisi`, `évi`, `őszi`), month ranges, and `havában`, `folyamán`, `végén` following a month name (the relation holds with the month, not the year).
- **Conditions**: year (or year group) + following month name or time word that is not in a grammatical relation with the year (no possessive suffix referring to the year, no postposition governing the year).
- **Output**: `1848. március 9.`, `1848. márciusban`, `1848. áprilisi`, `2014. évi`, `1944. őszi`, `1945. március–áprilisban`, `1849. tavaszi–nyári`, `1956. október havában`, `1956. október folyamán`, `1956. október végén`.
- **Examples**: as Output; contrasts `1848 márciusában`, `1944 őszén`, `1956 folyamán`, `1956 októberének végén` (`ORT-NUM-016`).
- **Exceptions**: `ORT-NUM-016`. AkH11 §294 phrased the condition as absence of a possessive relation (`birtokviszony`); AkH12 widens it to any grammatical relation (covering postpositions and subject use, `ORT-NUM-016`).
- **Predictability**: `rule` given morphology of the next word (possessive suffix `-a/-e/-ja/-je` on the time word = relation -> no period)
- **Lexical input**: morphological analysis of the following word
- **Interactions**: `PND-DATE`, `PUN-DATEF-005`, `ORT-NUM-016`, `ORT-NUM-018`
- **Sources**: [AkH12 §267 c), §295-§296]; [AkH11 §294]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-015 Fractions in letters
- **Statement**: A fraction with a cardinal attribute is written apart when it is noun-like, solid when the whole acts as an attribute; clock expressions likewise; decimal fractions in digits use the decimal comma.
- **Conditions**: cardinal + fraction word (`negyed`, `harmad`, `tized`, `fél`...).
- **Output**: noun-like: `egy negyed`, `két harmad`, `harmincnyolc egész egy tized`; attributive: `az ország kétharmad része`, `háromnegyed óra múlva`; clock: `negyed kilenckor`, `fél tizenkettőig`, `háromnegyed négyre`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule` given function
- **Lexical input**: syntactic function (fraction used as a noun vs as an attribute of a following noun)
- **Interactions**: `PND-FRAC`, `ORT-TOG-039`
- **Sources**: [AkH12 §293]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-016 No period after the year
- **Statement**: No period follows a year (group) (a) before a postposition or an adjective derived from a postposition, or when the year is a possessor of the next word; (b) when the year is the subject; (c) the period may also be omitted elsewhere, e.g. on gravestones, memorial plaques, book title pages, and after years in parentheses.
- **Conditions**: listed contexts.
- **Output**: (a) `1848 előtt`, `1848 előtti`, `1849 után`, `1867 és 1896 között`, `1526 augusztusában`, `1848 hősei`, `1849 mártírjai`, `1703–1711 szabadságharca`, `1914 késő őszén`, `1848 első felében`, `1849 tavaszi hónapjai`, `1944/45 nehéz telén`; (b) `1492 fordulópont ...`, `1941–1945 mérhetetlen károkat okozott`; (c) `A mohácsi vész (1526) ...`.
- **Examples**: as Output.
- **Exceptions**: (c) is optional; generation keeps the period in (c) contexts other than parentheses (inference). AkH11 §295 allowed omission only in parentheses; inscriptions and title pages are new in AkH12.
- **Predictability**: `rule` given syntax for (a)-(b); optional for (c)
- **Lexical input**: postposition list; possessive marking on the next word; subject function
- **Interactions**: `ORT-NUM-014`
- **Sources**: [AkH12 §297]; [AkH11 §295]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-017 Suffixes on day and year digits: hyphen, no period
- **Statement**: Suffixes attach to year and day digits with a hyphen and without period; day forms reflect the possessive-marked spoken ordinal (`tizenötödikén`): 3sg possessive `-a/-e` (`-je` for 1), superessive `-án/-én`, etc.; for `1` both `-je`-forms and short forms exist; `-ig`, `-től` have short and full variants; `-i` adjectives have several variants (AkH12 rule text added `1.`, `1-én`, `1-ig`, `1-i`; `2-ai` comes from the AkH12 dictionary, not from §298, and is confirmed by the editor's change list: AkH11 `másodiki v. 2-i` -> AkH12 `másodiki v. 2-i v. másodikai v. 2-ai` [KESZLER2016 pp. 95-100]).
- **Conditions**: date day number or year in digits + suffix.
- **Output**: years: `az 1838-as árvíz`, `1514-ben`, `1848–49-ben`; days: `1848. márc. 15-én`, `július 6-ig~6-áig`, `12-től~12-étől`, `december 25-től január 27-ig`, `február 28-a`, `02. 28-án`; first of month: `1.~1-je`, `1-én~1-jén`, `1-ig~1-jéig`, `1-i~1-ji~1-jei`; second: `2-a`, `2-án`, `2-i~2-ai` (dictionary); other days: `15-i~15-ei` (`PND-DATE-006`; generate `N-i`). Day endings by spoken ordinal harmony: back `-a/-án/-áig/-ától` for 2 (`másodika`), 3, 6, 8, 13, 16, 18, 20, 23, 26, 28, 30; front `-e/-én/-éig/-étől` for 4, 5, 7, 9, 10, 11, 12 (`tizenkettedike`), 14, 15, 17, 19, 21, 22 (`huszonkettedike`), 24, 25, 27, 29, 31; 1: `-je/-jén` (from `elseje`) or, AkH12 variant, `-én`.
- **Examples**: as Output.
- **Exceptions**: AkH11 §296 allowed only `1-je`, `1-jén`, `1-jéig`, `1-ji~1-jei` (no `1.`, `1-én`, `1-ig`, `1-i`) and did not list `2-ai`.
- **Predictability**: `rule` + `lexical` (variants)
- **Lexical input**: spoken ordinal
- **Interactions**: `PND-DATE`, `PND-DATE-006`, `ORT-NUM-006` (ordinal period rule does not apply to dates), `ORT-NUM-019`
- **Sources**: [AkH12 §292, §295, §298]; [AkH11 §296]; [KESZLER2016 pp. 95-100]; [HUWIKI-AKH12CH]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-018 Ranges of years and dates
- **Statement**: With joined ordinal numbers (years, centuries, days) the period is written only after the second number; year pairs may be joined by en dash or slash, with the second year abbreviated; ranges of full dates use a spaced en dash and the first date keeps its period.
- **Conditions**: ranges.
- **Output**: `2013–2014.~2013/2014.~2013–14.~2013/14. évi`, `20–21. század~XX–XXI. század`, `2014. április 28–30.`, `2014. április 11. – 2014. október 10.`, `2014. I. 15. – VIII. 23.`, `1983–1984. évi`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-HYPH-013`, `PUN-DASH`
- **Sources**: [AkH12 §264 c), §296]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-019 Day number followed by a postposition
- **Statement**: If a postposition follows the day number in a date, the number takes either a period or a hyphenated possessive suffix.
- **Conditions**: `day + postposition`.
- **Output**: `2013. december 20. és 31. között~2013. december 20-a és 31-e között`; `2013. december 10. óta~2013. december 10-e óta`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: postposition list
- **Interactions**: `ORT-NUM-017`
- **Sources**: [AkH12 §299]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-020 Full date formats
- **Statement**: The year is in Arabic digits with a period; the month may be written out, abbreviated, or in Roman or Arabic digits (Arabic digits with a period, leading zero allowed); the day is in Arabic digits followed by a period, or with a hyphenated suffix without period.
- **Conditions**: complete dates.
- **Output**: `2014. február 28.`, `2014. II. 28.`, `2014. febr. 28.`, `2014. 02. 28.`, `2014. febr. 28-a`, `2014. 02. 28-án`, `1848. III. 15.`
- **Examples**: as Output.
- **Exceptions**: ISO `2014-02-28` is not an AkH12 form (technical contexts only); the online AkH11 text (§293) noted the undotted hyphenated form `1994-02-05` as spreading, AkH12 §295 omits it. AkH11 did not state the suffixed-day option in this paragraph.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-DATE`, `PUN-DATEF-001`, tokenisation (period after day vs sentence end)
- **Sources**: [AkH12 §267 c), §295]; [AkH11 §293]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-NUM-021 Sports results
- **Statement**: Score adjectives may be written in letters or digits; AkH12 dictionary accepts the hyphenated digit form beside the colon form.
- **Conditions**: score + `-s`.
- **Output**: `három nullás~3:0-s~3 : 0-s~3-0-s`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: none
- **Interactions**: `PUN-NUMF-016`, `ORT-NUM-003`
- **Sources**: [HUWIKI-AKH12CH] (AkH12 dictionary entry; not in the rule text)
- **Validation**: -
- **Confidence**: `normative` (dictionary level, via a secondary source; verify in the AkH12 dictionary)

## SPLIT: hyphenation at line end (elválasztás)

Algorithm summary (normative content in the rules below):

```text
hyphenation_points(word):
  G = graphemes(word) using morpheme-aware digraph segmentation (SPLIT-020), archaic name
      letters (SPLIT-013), foreign one-sound letter groups (SPLIT-012), ch and x as one (SPLIT-010)
  expand simplified doubled digraphs into two graphemes (SPLIT-007)
  mark compound and prefix boundaries as preferred points (SPLIT-015, SPLIT-016, SPLIT-019)
  for each maximal consonant run C1..Cn between vowels Vi and Vj (vowels = syllable nuclei, SPLIT-001):
      n == 0: point between the vowels (SPLIT-004)
      n == 1: point before C1 (SPLIT-003)
      n >= 2: point before Cn (SPLIT-005, SPLIT-008); if Cn-1 == Cn (doubled letter) point between them (SPLIT-006, SPLIT-008)
  remove points that leave a single vowel letter alone at word start or end (SPLIT-004, recommended)
  words with < 2 vowels: no points (SPLIT-002); acronym rules (SPLIT-018)
```

### ORT-SPLIT-001 Syllables and the basic principle
- **Statement**: Hyphenation is based on syllabification; only for compounds are member boundaries considered. A word has as many syllables as vowels; a vowel alone may form a syllable; inside a word a syllable begins with a vowel or with exactly one short consonant (the first syllable may begin with any consonants). A polysyllabic word may be broken at any syllable boundary; the line-end hyphen is written at the end of the line (`ORT-SPLIT-017`).
- **Conditions**: simple common words and proper names, base and suffixed forms.
- **Output**: break points = syllable boundaries.
- **Examples**: `bál`, `font`, `György`, `Pest`; `asz-tal`, `há-zig`, `ka-to-na`, `ma-gá-nyos`, `me-ta-fo-ra`, `te-ker-vé-nyes`, `Csil-la`, `De-zső`, `Be-rety-tyó`, `Ke-re-csend`; `di-ó-nyi`, `e-u-ró`, `fi-a-i-é`, `te-a`, `É-va`, `Le-ó` (syllables; as line breaks not recommended, `ORT-SPLIT-004`); `le-ány`, `bugy-gyos`, `lajst-rom`, `pró-ba`, `vá-szon`; `ren-dületlenül`, `rendü-letlenül`, `rendület-lenül`, `rendületle-nül`, `fi-atal~fia-tal`.
- **Exceptions**: `ORT-SPLIT-002`, `ORT-SPLIT-004` (single-vowel pieces not recommended).
- **Predictability**: `rule`
- **Lexical input**: grapheme segmentation
- **Interactions**: `ORT-63-003` (syllable count), `PUN-DASH-017` (line-end hyphen), magyarispell `hy.py` implements an equivalent ordered-regex procedure (study only)
- **Sources**: [AkH12 §223-§224, §270]
- **Validation**: goldens; oracle:magyarispell (`hyph_hu_HU.dic` patterns)
- **Confidence**: `normative`

### ORT-SPLIT-002 Words with one vowel are not split
- **Statement**: Words containing only one (pronounced) vowel are not hyphenated, including foreign and archaic spellings where a letter group denotes one vowel.
- **Conditions**: one syllable nucleus by pronunciation.
- **Output**: no break.
- **Examples**: `föld`, `hajt`, `nyolc`, `part`, `strand`, `sztrájk`, `Bánk`, `Sásd`, `Zsolt`, `Brahms`, `Schmidt`, `York`; `air`, `blues`, `street`, `team`, `Maugham`, `Gaal`, `Soós`, `Georch`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: vowel-letter-group pronunciation for foreign/archaic words
- **Interactions**: `ORT-SPLIT-012`, `ORT-SPLIT-013`
- **Sources**: [AkH12 §225]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-003 One consonant between vowels goes to the next line
- **Statement**: A single short consonant between two vowels, whether written with one letter or a digraph/trigraph, goes to the next line; also in suffixed forms.
- **Conditions**: `V C V`, `C` one grapheme (`cs dz dzs gy ly ny sz ty zs` included).
- **Output**: `V-CV`.
- **Examples**: `be-tű`, `ko-sár`, `ba-tyu`, `ka-sza`, `ró-zsa`, `Gyu-la`, `Me-csek`; `holna-pig`, `maga-san`, `né-gyet`, `pa-don`, `apá-mé`, `ki-sebb`, `freu-dizmus`, `tiha-nyi`, `Kecskemé-ten`, `Mikló-sék`; by the same rule: `go-lyó`.
- **Exceptions**: compound boundaries (`ORT-SPLIT-015`: `vas-út`, `meg-int`).
- **Predictability**: `rule`
- **Lexical input**: digraph segmentation (`ORT-SPLIT-020`)
- **Interactions**: `WS-DIG`
- **Sources**: [AkH12 §226 c)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-004 Vowel sequences and single-vowel syllables
- **Statement**: Vowels forming syllables by themselves may stay on the line or move to the next, also in suffixed forms; syllable-initial vowels may move; two identical vowel letters denoting two vowels may be separated; the letter pairs `au`, `eu` may be split when necessary (their members can fall into different syllables). For aesthetic reasons a word-initial or word-final syllable consisting of a single vowel is not left alone at the line end or carried alone to the next line in print; breaks creating misleading word images (`bale-set`, `fela-dat`; advisory practice quotes §226 a) for `haza-i`, `emóci-ó`, `Le-a` as not recommended in print, so §236's compound-boundary allowance does not conflict `[ENYELV 2016-03-15]`, `[ENYELV 2014-02-01]`; `OQ-07-021` resolved 2026-09-16) are likewise not recommended.
- **Conditions**: `V V` sequences; word-initial/final single vowel syllable.
- **Output**: allowed or not-recommended break points as in Examples.
- **Examples**: allowed: `dia-dal~di-adal`, `ideá-lis~ide-ális`, `Leo-nóra~Le-onóra`, `árui-nak~áru-inak`, `könyvei-tek~könyve-itek`, `Annáé-ké~Anná-éké`, `tolnai-ak~tolna-iak`, `fi-ók`, `vi-asz`, `kalci-um`, `moza-ik`, `Mari-etta`, `Tri-eszt`, `ko-ordinál`, `váku-um`, `ablaká-ig`, `háza-ik`, `pesti-es`, `Vajdá-ék`, `ba-uxit`, `fe-udális`, `kala-uz`, `la-udáció`; not recommended: `a-lap`, `e-móció`, `Á-ron`, `haza-i`, `emóci-ó`, `Le-a`, `bale-set`, `fela-dat`.
- **Exceptions**: `fela-dat` is listed as not irregular in §236 (`ORT-SPLIT-016`) but not recommended in §226 a): treat as correct but dispreferred.
- **Predictability**: `rule` (allowed) + `tendency` (recommendation: suppress single-letter pieces; suppress misleading compound-internal splits by preferring member boundaries)
- **Lexical input**: compound boundaries for the misleading-image check (`baleset` = `bal#eset`)
- **Interactions**: `ORT-SPLIT-015`
- **Sources**: [AkH12 §224, §226 a)-b), §236]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-005 Two different consonants: first stays, second goes
- **Statement**: Of two different consonant graphemes between vowels, the first stays and the second moves; if the first is a doubled letter (single-letter or simplified digraph geminate), the whole doubled letter stays.
- **Conditions**: `V C1 C2 V`, `C1 != C2`; `C1` may be `GG`.
- **Output**: `VC1-C2V`; `VGG-C2V`.
- **Examples**: `am-per`, `ás-vány`, `fosz-fát`, `mor-zsa`, `tarisz-nya`, `tal-palatnyi`, `Bar-tók`, `Bony-hád`, `Er-zsi`; `bron-zot`, `ker-tig`, `pén-zért`, `recs-ki`, `töl-gyes`, `Heren-dig`, `Szabol-csé`; doubled first: `bizott-ság`, `hall-gat`, `több-let`, `könny-től`, `meggy-ből`, `rossz-kor`, `Kiss-nek`, `Papp-tól`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: grapheme segmentation
- **Interactions**: `ORT-LEN-021`
- **Sources**: [AkH12 §226 d)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-006 Doubled single letter between vowels: split in the middle
- **Statement**: A doubled single-letter consonant between vowels is split, one letter on each line, regardless of pronounced length; forms produced by three-letter simplification are split as written.
- **Conditions**: `V XX V`.
- **Output**: `VX-XV`.
- **Examples**: `ber-reg`, `kom-munális`, `mil-liméter`, `ton-na`, `Boz-zai`, `Job-bágy`, `Car-rara`, `Maupas-sant`, `ember-rel`, `hal-lak`, `hit-tem`, `job-ban`, `tet-tel`, `víz-zé`, `Szél-lért`, `Tallin-nig`; simplified: `töb-be`, `szeb-ből`, `or-ra`, `vádlot-tól`.
- **Exceptions**: `ORT-SPLIT-008` when more consonants precede.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-ASSIM-017`
- **Sources**: [AkH12 §61, §226 e)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-007 Simplified doubled digraph: both halves written in full
- **Statement**: A long consonant written as a simplified doubled digraph is split with the full digraph at the end of the line and again at the start of the next line.
- **Conditions**: `V G1G1G2.. V` simplified doubling (`ccs ddz ddzs ggy lly nny ssz tty zzs`) at the break.
- **Output**: `...G-G...` with full digraphs.
- **Examples**: `meny-nyi`, `ösz-sze`, `pogy-gyász`, `szity-tya`, `Boty-tyán`, `Hosz-szú`, `Ily-lyés`, `ágy-gyal`, `fodrász-szal`, `megy-gyel`, `galy-lyak`, `köny-nyes`, `Arany-nyal`, `Kodály-lyal`, `Nagy-gyal`; from §7 b): `locs-csan`, `edz-dzük`, `asz-szony-nyal`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-ASSIM-016`; the hyphenated text is not a spelling of the unbroken word (`REP` must rejoin to the simplified form: `asz-` + `szony` -> `asszony`)
- **Sources**: [AkH12 §7 b), §226 f)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-008 Three or more consonants: only the last goes; final doubled letter splits
- **Statement**: With more than two consonant graphemes between vowels only the last moves to the next line; if the last one is a doubled letter, one half stays and one half moves (for simplified digraph doubling, both written out).
- **Conditions**: `V C1..Cn V`, `n >= 3`, or `n >= 2` ending in a doubled letter.
- **Output**: `VC1..Cn-1-CnV`; doubled last: `...X-X...`.
- **Examples**: `cent-rum`, `ost-rom`, `temp-lom`, `tink-túra`, `lajst-rom`, `Dosztojevsz-kij`, `Ing-rid`, `Ojszt-rah`, `Veszp-rém`, `Würt-temberg`, `batiszt-ból`, `kilenc-kor`, `Hamburg-ban`, `bratsz-ki`; doubled last: `bors-sal`, `csoport-tól`, `karszt-tá`, `tömb-ben`, `vonz-za`, `kulcs-csal`, `rongy-gyá`, `Ford-dal`, `Gellért-től`, `Szabolcs-csal`, `játsz-szál`, `metsz-szük`.
- **Exceptions**: compounds and pseudo-compounds may prefer other points (`ORT-SPLIT-014`, `ORT-SPLIT-015`).
- **Predictability**: `rule`
- **Lexical input**: grapheme segmentation
- **Interactions**: `ORT-SPLIT-009`
- **Sources**: [AkH12 §224, §226 g)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-009 Suffixed forms follow the same syllable rules
- **Statement**: Each added suffix is syllabified by the same rules; the stem-suffix boundary is not a preferred break point.
- **Conditions**: inflected/derived simple words.
- **Output**: `adós-ság`, `adóssá-gok`, `adósságo-kat`, `há-zunk`, `házun-kat`.
- **Examples**: as Output.
- **Exceptions**: compound-based derivatives `ORT-SPLIT-016`; hyphen-attached suffixes (`ORT-SPLIT-017`, `ORT-SPLIT-018`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-SPLIT-003` to `ORT-SPLIT-008`
- **Sources**: [AkH12 §226 g)]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-010 ch and x count as one letter
- **Statement**: `ch` (one sound) and `x` (two sounds, one letter) each count as a single consonant letter: between vowels they move to the next line; at a stem-suffix junction before a vowel they move; `-val/-vá` forms split between the letter and the assimilated consonant; long `cch` splits `c-ch`.
- **Conditions**: `ch`, `x` graphemes.
- **Output**: `ch`/`x` treated as one short consonant grapheme by `ORT-SPLIT-003`, `ORT-SPLIT-005`, `ORT-SPLIT-008`.
- **Examples**: `ar-chívum`, `ma-chináció`, `or-chidea`, `pszi-chológia`, `Ri-chárd`, `Mün-chen`, `he-xameter`, `ma-ximum`, `pra-xis`, `Ale-xandra`, `Me-xikó`; suffixed: `almana-chot`, `ce-chem`, `pe-ches`, `bóra-xos`, `fi-xet`, `refle-xek`, `tele-xezünk`, `Bibera-chot`, `züri-chi`, `Féli-xé`; `pech-hel`, `bórax-szá`, `Bach-hal`, `Beatrix-szal~Beatrix-szel`; `Bac-chus`, `Pinoc-chio`.
- **Exceptions**: `c+h` across a morpheme boundary is two letters (`akác+hoz` -> `akác-hoz`, `ORT-SPLIT-020`; inference, not in §228).
- **Predictability**: `rule`
- **Lexical input**: morpheme boundary for `c|h`
- **Interactions**: `ORT-FOR-002`, `ORT-FOR-011`
- **Sources**: [AkH12 §228]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-011 dz and dzs are single graphemes regardless of length
- **Statement**: `dz` and `dzs` are handled as single multi-letter graphemes by the written form, not by pronounced length; doubled forms split with both written out.
- **Conditions**: `dz`, `dzs`.
- **Output**: `bo-dza`, `gyűrű-dzik`, `lopó-dzott`, `ma-dzag`, `pe-dzi`, `bri-dzset`, `mahara-dzsa`, `mene-dzser`, `tá-dzsik`, `brin-dza`, `halan-dzsázik`, `lán-dzsás`, `edz-het`, `ódz-kodik`, `Fu-dzsi`, `Ma-dzsar`, `Kiliman-dzsáró`, `edz-dze`, `lopódz-dzanak`, `bridzs-dzsel`.
- **Examples**: as Output.
- **Exceptions**: `d+z`, `d+zs` across morpheme boundaries are separate letters (`ORT-SPLIT-020`).
- **Predictability**: `rule`
- **Lexical input**: morpheme boundaries
- **Interactions**: `ORT-LEN-010`
- **Sources**: [AkH12 §227]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-012 Foreign-spelled words by Hungarian pronunciation
- **Statement**: Foreign-spelled words and names are hyphenated by syllables of their Hungarian pronunciation: letter groups denoting one vowel in one syllable are not split, and letter groups denoting one consonant are not split (like Hungarian digraphs).
- **Conditions**: foreign spelling.
- **Output**: `cou-chette`, `cou-lomb`, `Baude-laire`, `Coo-per`, `Mon-taigne`; `bron-chi-tis`, `ce-pha-lal-gi-a`, `com-pa-gnie`, `ery-the-ma`, `Bo-lo-gna`, `Chur-chill`, `Fi-scher`, `Gi-gli`, `Gior-gio`, `Wa-shing-ton`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule` given pronunciation
- **Lexical input**: foreign pronunciation with alignment of letter groups to sounds (G2P alignment)
- **Interactions**: `ORT-SPLIT-002`, `PH-G2P`, `ORT-FOR-012`
- **Sources**: [AkH12 §230]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-013 Archaic letters in family names
- **Statement**: Multi-letter archaic graphemes in family names are handled by their sound value (kept as units); doubled consonant letters not followed by another pronounced consonant are split one on each line regardless of sound value; in suffixed forms the archaic units are kept together except doubled letters.
- **Conditions**: family names with archaic letters.
- **Output**: `Beö-thy`, `De-seö`, `Thew-rewk`, `Ben-czúr`, `Fara-ghó`, `Szé-chy`, `War-gha`; doubled: `Bat-thyány`, `Kos-suth`; suffixed: `Thewrew-köt`, `Cze-tzet`, `Babi-tsé`, `Balo-ghék`, `kossu-thi`, `móri-czos`; doubled: `Pap-pot`, `Szél-lért`, `Kis-sék`.
- **Examples**: as Output.
- **Exceptions**: names treated as compounds break at the member boundary before `-ffy`, not between the doubled letters: `Pál-ffy` (two one-syllable members, [AkH12 §234]), `Dessew-ffy` (member boundary preferred for longer compounds, [AkH12 §235]) (`ORT-SPLIT-015`); the doubled-letter split of §229 applies to non-compound names.
- **Predictability**: `rule` given the name's letter-to-sound alignment
- **Lexical input**: name pronunciation (`ORT-NAME-003`)
- **Interactions**: `ORT-NAME-003`, `ORT-SPLIT-015`
- **Sources**: [AkH12 §229] (sound-value units, doubled letters), [AkH12 §234] (`Pál-ffy`), [AkH12 §235] (`Dessew-ffy`)
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-014 Foreign pseudo-compounds: by syllables; member split accepted
- **Statement**: Foreign words formed in the source language from elements not existing independently in Hungarian are not treated as compounds and are split by Hungarian syllabification (last consonant of a cluster moves); splitting at the better-known source elements is also acceptable, especially in technical texts; in doubt, break the word at another, unproblematic syllable boundary.
- **Conditions**: learned/foreign formations without independent Hungarian members.
- **Output**: `abszt-rakt`, `as-piráns`, `demok-rata`, `diszk-rét`, `fi-lantróp`, `inf-láció`, `inst-ruktor`, `kataszt-rófa`, `komp-resszor`, `konk-rét`, `obst-rukció`, `parag-rafus`, `prog-nózis`, `prog-ram`, `rek-lám`, `szind-róma`, `Miche-langelo`, `Belg-rád`, `Zág-ráb`; also accepted: `fotog-ráfia~foto-gráfia`, `mikrosz-kóp~mikro-szkóp`, `termosz-tát~termo-sztát`; safe alternatives in doubt (recommended, not to be avoided): `pa-ragrafus~paragra-fus`, `Mi-chelangelo~Michelan-gelo`.
- **Examples**: as Output.
- **Exceptions**: `ORT-SPLIT-019`.
- **Predictability**: `rule` (both variants valid)
- **Lexical input**: learned element inventory (for the alternative)
- **Interactions**: `ORT-63-005`, `ORT-SPLIT-019`
- **Sources**: [AkH12 §231]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-015 Compounds split at member boundaries
- **Statement**: Compounds recognisable to the average speaker are split at the member boundary even when it is not a syllable boundary; elsewhere members are syllabified normally. Two one-syllable members are split between them (also verbal prefix + verb); longer compounds are best split at a member boundary, but syllabic breaks inside members are also correct; `leg-` is treated as a member. Homographs may hyphenate differently.
- **Conditions**: `#` boundaries (compound, verbal prefix, `leg-`).
- **Output**: boundary break required instead of the syllabic break across the boundary (`vas-út`, not *`va-sút`); breaks inside members follow syllabification and are also correct for compounds of more than two syllables; the member boundary is the preferred point.
- **Examples**: `csak-is`, `épp-úgy`, `hón-alj`, `ing-ujj`, `kül-ügy`, `még-is`, `mind-egy`, `rend-őr`, `vagy-is`, `vas-út`, `Kis-ar`, `Pál-ffy`, `fel-ad`, `meg-öl`, `szét-üt`; `csal-étek`, `dél-után`, `egy-előre`, `egyszer-egy`, `elektron-optika`, `ion-stabilitás`, `kar-öltve`, `kis-asszony`, `külön-élés`, `szak-avatott`, `tölgy-erdő`, `ugyan-is`, `vírus-interferencia`, `viszont-eladó`, `zápor-eső`, `Dessew-ffy`, `Kis-oroszi`, `Magyar-atád`, `Nyír-egyháza~Nyíregy-háza`, `Zala-egerszeg~Zalaeger-szeg`; `leg-alább`, `leg-alul`, `leg-előre`, `leg-erősebb`; also correct: `alez-redes~alezre-des`, `elekt-ronoptika~elektronop-tika`, `legszeren-csésebb`, `Za-laegerszeg`; homographs: `me-gint` 'again' vs `meg-int` 'warn'; `gé-pelem` (verb) vs `gép-elem` (noun).
- **Exceptions**: only compounds recognisable to the average present-day speaker are split by members; others are syllabified (`ORT-SPLIT-014`, `ORT-SPLIT-019`).
- **Predictability**: `rule` given boundaries; `lexical` for opaque compounds and homograph sense
- **Lexical input**: compound/prefix boundaries (`DER-COMP`); sense for homographs
- **Interactions**: `ORT-SPLIT-004`, `ORT-SPLIT-016`, `ORT-SPLIT-020`
- **Sources**: [AkH12 §233-§235]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-016 Derivatives of compounds
- **Statement**: A derivative formed from a whole compound is split by the members of the base compound; plain syllabic breaks are not wrong.
- **Conditions**: `[A#B]+suffix`.
- **Output**: `rend-őri`, `vas-utas`, `víg-operai`, `fel-adat`, `meg-állás`, `nagy-atádi`; also correct: `rendő-ri`, `vasu-tas`, `vígope-rai`, `fela-dat`, `megál-lás`, `nagya-tádi`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: base compound boundaries
- **Interactions**: `ORT-SPLIT-015`
- **Sources**: [AkH12 §236]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-017 Breaking at an existing hyphen or en dash
- **Statement**: A word containing a hyphen or en dash may be broken at it; the hyphen (or dash) is written only at the end of the line, not repeated at the start of the next line, except exceptionally in specialist works (dictionaries, chemistry) to signal the hyphenated base form.
- **Conditions**: break at `-` or `–`.
- **Output**: `tűzzel-` / `vassal`, `Dumas-` / `nak`, `a MÁV-` / `ot`, `magyar–` / `angol szótár`; specialist: `földrajzinév-` / `-tár`, `Sziklás-` / `-hegység` (spelling dictionary), `nátrium-` / `-klorid` (chemistry).
- **Examples**: as Output.
- **Exceptions**: specialist repetition.
- **Predictability**: `rule`
- **Lexical input**: genre (for repetition)
- **Interactions**: `PUN-DASH-017`, `REP` (de-hyphenation must keep the hyphen when the break is at a lexical hyphen: ambiguous `tűzzel-` + `vassal`; decide by lexicon/morphology)
- **Sources**: [AkH12 §238, §270]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-018 Acronyms and blends
- **Statement**: Letter acronyms with no vowel or only one vowel are not split; those with two or more vowels may be split by syllables; a hyphenated suffix containing a vowel moves to the next line as a whole even if the hyphen is not at a syllable boundary. Blends are split by syllables in base and suffixed forms.
- **Conditions**: acronyms, blends.
- **Output**: no split: `BVSC`, `MLSZ`, `NOB`, `SZTK`, `MTESZ`, `ORFK`; split: `NA-TO`, `OR-FI`; suffix: `ELTE-` / `re`, `USA-` / `beli`, `ENSZ-` / `ért`; blends: `Ag-ro-co-op`, `Int-ransz-mas`, `Ma-hart`, `Ma-lév`, `Ma-va-dé`, `Röl-te-xig`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: acronym type
- **Interactions**: `ORT-ABBR-014`, `ORT-SPLIT-017`
- **Sources**: [AkH12 §232]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-019 Semi-foreign compounds
- **Statement**: Foreign words with one element independent in Hungarian (usually the second) and one element only occasionally independent (usually a learned prefix) are generally split at their junction; if the compound nature is less obvious, syllabic splitting is also correct; breaking elsewhere avoids doubt.
- **Conditions**: learned prefix/combining form + independent Hungarian word.
- **Output**: `centi-gramm`, `extra-kromoszomális`, `kilo-gramm`, `milli-gramm`, `anti-proton`, `infra-struktúra`, `inter-akció`, `melo-dráma`, `mikro-klíma`, `pre-klasszikus`, `proto-plazma`, `transz-urán`, `Jugo-szlávia`; less obvious: `de-presszió~dep-resszió`, `im-produktív~imp-roduktív`, `para-frázis~paraf-rázis`, `tranz-akció~tran-zakció`, `Ant-arktisz~An-tarktisz`; safe alternatives in doubt (recommended, not to be avoided): `in-terakció~interak-ció`, `preklasz-szikus`, `Antark-tisz`.
- **Examples**: as Output.
- **Exceptions**: -
- **Predictability**: `lexical` (degree of transparency) + `rule`
- **Lexical input**: learned element inventory; transparency flag
- **Interactions**: `ORT-SPLIT-014`, `ORT-63-005`
- **Sources**: [AkH12 §237]
- **Validation**: goldens
- **Confidence**: `normative`

### ORT-SPLIT-020 Digraph ambiguity at morpheme boundaries
- **Statement**: Letter sequences that look like digraphs may be two letters across a morpheme or compound boundary; grapheme segmentation for hyphenation (and for letter-based rules: doubling, syllable onset) must use morphological boundaries: `c|s`, `z|s`, `sz|s`, `d|z`, `d|zs`, `c|h`, `g|y`/`l|y`/`n|y`/`t|y` (only in foreign or archaic spellings).
- **Conditions**: candidate digraph spanning `+` or `#`.
- **Output**: split the letters: `köz+ség` -> `köz-ség` (not *`kö-zség`); `egész+ség` -> `egész-ség`; `ház#sor` -> `ház-sor`; `arc#sérülés` -> `arc-sérülés`; `akác+hoz` -> `akác-hoz`; `vad#zab` -> `vad-zab`; a real digraph after a boundary stays whole: `kő#zsák` -> `kő-zsák`.
- **Examples**: as Output.
- **Exceptions**: no native `s|z` junction is attested in the frequency list (`WS-DIG-006`).
- **Predictability**: `rule` given morphology; `lexical` otherwise
- **Lexical input**: morpheme and compound boundaries
- **Interactions**: `WS-DIG`, `DER-COMP`, `ORT-PRIN-005`; magyarispell `hy.py` uses Hunspell morphological fields for this (study only); espeak-ng `hu_list` lists such words (study only)
- **Sources**: [AkH12 §52, §224, §233]; [magyarispell hy.py] (study only)
- **Validation**: goldens; oracle:magyarispell
- **Confidence**: `consensus` (follows from the rules; not stated explicitly in AkH12)

## Open questions
1. `ORT-SUFX-002`: the full closed list of non-lengthening suffixes is not in AkH12; `-ként`, `-képp(en)`, `-szor`, `-féle`, `-ista`, `-izmus` come from SMNy3 table (188); marginal learned suffixes (`-izál`, `-ikus`, `-lógia`) are marked `?` there. Verify against a corpus probe (`almaként` vs `almáként`, `dadaizmus`).
2. `ORT-ASSIM-008`: resolved: AkH12 §83 is silent on the translative of `az/ez`; AkH11 §84 gives `azzá`, `ezzé` only, consistent with `PH` and `PND-DEM`.
3. `ORT-TOG-013`: spelling of `csiki-csuki` (requested example) is not in the AkH12 rule text and the online dictionary could not be queried; confirm in the AkH12 dictionary.
4. `ORT-TOG-024`: AkH12 §118 does not state whether a prefixed participle is a simple or a compound member; the rule now uses the §139 syllable criterion (one-syllable prefix = simple: `kiképzőtiszt`; two or more: `összekötő híd`, `előadó körút`) as an inference; confirm against further AkH12 dictionary entries.
5. `ORT-TOG-047`: the list of AkH11 -> AkH12 dictionary changes is taken from a Wikipedia summary; each item should be checked against the AkH12 dictionary before use as goldens.
6. `ORT-63-002`: AkH12 wording ("may be hyphenated") vs consistent dictionary hyphenation: decide whether a solid spelling of a >6-syllable multiple compound is flagged as an error or a tolerated variant.
7. `ORT-63-006`: main-boundary bracketing is semantic; a heuristic and an ambiguity policy are needed (e.g. `tervezőiroda-vezető` vs `tervező irodavezető`).
8. `ORT-63-003`: syllable counting for foreign-spelled members (pronounced vowels vs vowel letters, `team`, `Baudelaire`) is not stated in AkH12.
9. `ORT-NAME-003`/`ORT-NAME-004`: sound values of archaic letters are name-specific (`thy` = [ti] vs [tty], `s` = [zs] only in some names, `Kossuth` vowel length); a name pronunciation lexicon is required; no open source of such data was verified.
10. `ORT-NAME-024`/`ORT-NAME-025`: the §177 text (proper-name members keep capitals) and its (a) examples (`alcsi-holt-tiszai`) diverge; the encoded reading (a proper-name member keeps its capital unless it is the member that takes `-i`) fits all 14 examples but is an inference.
11. `ORT-NAME-026`: familiarity of a `-falva/-háza` name (deciding `-i` vs `-ai`) is not computable; both are accepted.
12. `ORT-FOR-012`: deciding "silent final letter or unusual final letter group" needs foreign pronunciation data; no licence-compatible pronunciation lexicon for foreign names was identified (emMorph `pron.lx1` and WikiPron are study-only/share-alike).
13. `ORT-FOR-018`: the Russian table is from secondary sources (Osiris Helyesírás summary on Hungarian Wikipedia) and AkH12 examples; verify every positional rule (ъ, ь before и, ё after л/р) against CSNMH85.
14. `ORT-FOR-019` to `ORT-FOR-023`: Ukrainian, Belarusian, Bulgarian, Macedonian, Serbian, Chinese finals, Japanese vowel length, Arabic, Persian, Hebrew, Indic tables are not encoded; primary sources (CSNMH85, KNMH81, FNH98) are not openly available.
15. `ORT-CAP-019`: name particles whose source convention capitalises them inside the full name (Belgian `Van`, Italian `Di`) are not addressed by AkH12.
16. `ORT-ABBR-010`, `ORT-ABBR-018`: spoken forms of signs with two readings (`kg` = kilogramm/kiló, `CNN` Hungarian vs English letter names) are usage-dependent; the table cells not attested in AkH12 need corpus validation (hyphenated tokens are split in the current frequency list, so `attest.py` cannot probe them).
17. `ORT-NUM-002`: suffixes on decimal numbers (`3,5-tel` vs `3,5-del`) depend on the reading (`három egész öt tized` vs `három és fél`); not covered by AkH12.
18. `ORT-NUM-010`: whether the year exception also applies to ordinal year expressions and to years written out in legal texts is not stated.
19. `ORT-SPLIT-004`: the "not recommended" single-vowel breaks: make suppression configurable (print vs permissive).
20. `ORT-SPLIT-017`: de-hyphenation of line breaks at lexical hyphens is ambiguous in plain text; the repair policy belongs to `REP`.
21. `ORT-SPLIT-004`: §226 a) calls `fela-dat` not recommended (misleading word image) while §236 lists such breaks as not irregular; the rule treats them as allowed but suppressed by default.
22. `ORT-LEN-005`: AkH12 lists `hindu` only as a noun with short final `u`; the adjective spelling needs dictionary confirmation.
23. RESOLVED 2026-09-16. All four dictionary-level claims are confirmed verbatim by the AkH12 editor's own change list [KESZLER2016 pp. 95-100]: `másodiki v. 2-i` -> `... v. másodikai v. 2-ai`; `három nullás v. 3:0-s` -> `... v. 3-0-s`; `vendéglátó-ipari` -> `vendéglátóipari`; `aquinói v. Aquinói Szent Tamás` -> `Aquinói Szent Tamás`.
24. Chapter 08 (not edited here): `PUN-DASH-006` states the spaced en dash between multiword names as required (AkH12 §264 b) permits it); `PUN-DASH-011` attributes a suffix hyphen to §294 (it is §265 d)); the `PUN-DASH-003` table points to `PUN-NUMF-012` for numerals above 2000 instead of `PUN-NUMF-011`.

## Sources used
All keys are in [sources.md](sources.md). Reliability notes for this chapter:
- `OH04`, `CSNMH85`, `KNMH81`, `FNH98`: not consulted directly (via `HUWIKI-CYR`, `HUWIKI-CHIN`).
- `KESZLER2016`: the AkH12 editor's change list, primary for dictionary-level edition claims.
- `HUWIKI-AKH12CH`: secondary source for dictionary-level edition claims.

## Terms
See [glossary.md](glossary.md).
