# 04 Pronouns, numerals, determiners (`PND`)

Scope: the definite article `a`/`az` and the indefinite article `egy` (form selection only), personal pronouns (személyes névmás) with their suppletive and case-stem paradigms, formal address pronouns, demonstratives (mutató névmás), interrogatives (kérdő névmás), relatives (vonatkozó névmás), reflexive (visszaható), reciprocal (kölcsönös) and possessive (birtokos) pronouns, indefinite, negative, universal and free-choice pronouns and quantifiers, postpositions (névutó) with governed case, personal suffixes and `-i` adjectives, cardinal numerals (tőszámnév) and their spelled-out composition, numeral inflection and suffixation on digits, ordinals (sorszámnév), fractions (törtszámnév), decimals, percentages, dates, clock times, measures and currency. Article usage (when an article appears), agreement, negative concord and word order are in `SYN`; the general spelling of suffix attachment, hyphenation and writing together or apart are in `ORT`; number, date and time punctuation (grouping, periods, dashes) is in `PUN`; noun stem classes, linking vowels, lowering and case allomorphy consumed here are in `NOM`; vowel harmony and pronunciation of spellings in `PH`.

## Area index
| Area | Topic |
|---|---|
| `ART` | `a`/`az` selection by pronunciation of the following word (words, digits, ordinals, Roman numerals, letters, acronyms, abbreviations, symbols, foreign words); `egy` |
| `PERS` | personal pronouns: nominative, accusative, case-stem forms, emphatic forms, pronominal possessor, formal address `ön`/`maga` |
| `DEM` | demonstratives: inventory, `z`-assimilation, full case paradigm, agreement with the head noun, internal determiners `e`/`ez`, demonstrative + postposition |
| `INT` | interrogatives: inventory, inflection, variants |
| `REL` | relatives: `a-` formation, inflection, `aki`/`ami`/`amely` |
| `REFL` | reflexive `maga`, reciprocal `egymás`, possessive pronouns `enyém` ... |
| `INDF` | indefinite, negative, universal, free-choice series; quantifiers `sok`, `kevés`, `néhány` ...; negative concord basics |
| `POSTP` | postposition inventory with governed case, personal-suffixed postpositions, `-i` adjectives, case suffix vs postposition |
| `CARD` | cardinal numerals: forms, composition, `két`/`kettő`, spelled-out orthography |
| `NUMINF` | numeral stems and suffixation: accusative, collective `-an`, `-szor`, `-s`, distributive, possessive; suffixes on digits by the last spoken word |
| `ORD` | ordinals, ordinal adverbs, digits with period, Roman numerals |
| `FRAC` | fractions, `fél`, decimals, percent, per mille |
| `DATE` | months, weekdays, day-of-month forms, years, decades, centuries, ranges, clock times, eras |
| `MEAS` | unit symbols, currency, suffixes on symbols by pronunciation |

## Conventions used in this chapter
- Notation, archiphonemes and harmony classes (`B`, `F`, `R`) as in the README; harmony values are outputs of `PH-HARM`, stem classes (`lowering`, `epenthetic`, `shortening`) are the `NOM-STEM`/`NOM-LOW` properties.
- "Spoken form" of a digit string, symbol, abbreviation or acronym is the word sequence a reader produces; suffix allomorphy and article choice are computed on it and the written suffix is the part of the spoken inflected form that follows the spoken stem (`4-gyel` because `négy` + `-vel` -> `néggyel`, suffix part `gyel`).
- Corpus buckets `[ATT n]` are the log-frequency buckets of the project frequency list (half-octave steps: a difference of `d` buckets is a frequency ratio of about `2^(d/2)`, so ratios quoted from buckets carry a factor of about 1.4 uncertainty); `-` is unattested. Lowercase tokens only; digit-hyphen forms are not in the list.
- Examples of normative range dashes use the en dash U+2013 (`1848–49-ben`); ordinary hyphens are U+002D.

---

## ART: articles

### PND-ART-001 Definite article allomorphs
- **Statement**: The definite article (határozott névelő) is `az` before a word whose pronunciation begins with a vowel and `a` before a word whose pronunciation begins with a consonant.
- **Conditions**:
  - The article is followed by the first pronounced element of the noun phrase (noun, adjective, numeral, pronoun, participle, adverb, digit string, symbol, abbreviation, acronym).
  - Vowel onset: any of `a á e é i í o ó ö ő u ú ü ű` pronounced as a vowel.
  - Consonant onset: everything else, including pronounced `h` and `j`-onsets written with a vowel letter (`PND-ART-002`).
- **Output**: `az` | `a`; no other forms; the article never inflects and never agrees.
- **Examples**: `az ablak`, `az út`, `az öreg ház`, `az én könyvem`, `az ő autója`, `az egyik`; `a ház`, `a hosszú út`, `a te könyved`, `a mi házunk`, `a hotel`, `a jó`.
- **Exceptions**: archaic or poetic `a` before a vowel (pre-19th-century texts, dialects) is not generated; recognise only.
- **Predictability**: `rule`
- **Lexical input**: pronunciation onset of the following element (`PH` for foreign spellings)
- **Interactions**: `PND-ART-002` .. `PND-ART-008` compute the onset for non-word input; article usage is `SYN`.
- **Sources**: [KVF98 pp. 94-95], [AkH12 §276]
- **Validation**: goldens; attestation (bigram probe in corpus text when available)
- **Confidence**: `consensus`

### PND-ART-002 Onset by pronunciation, not by letter
- **Statement**: For words whose spelling and pronunciation onsets differ, the pronounced onset decides.
- **Conditions**:
  - Foreign spellings with a vowel letter pronounced with an initial consonant (`[j]`, `[v]`): `u`, `eu`, `y`, `w`, `ou` onsets as pronounced.
  - Foreign spellings with a consonant letter not pronounced (initial silent `h` in French, Spanish, Italian names and words when the Hungarian pronunciation also omits it).
- **Output**: article by pronounced onset.
- **Examples**: `a Unix` (`[juniksz]`), `a Unicode`, `a Yale`, `a Yamaha`, `a Wales`, `a whisky`; `az Yves Saint Laurent` (`[ív]`), `az Ypres`; `a Huelva`, `a Hugo` (spelling pronunciation, 99.4% of article tokens before `h`-initial foreign names; `az Hugó` is pre-reform only).
- **Exceptions**: names with established spelling pronunciation in Hungarian follow it. Measured over six sources [ATT evidence/corpus-variants.md 2026-09-16]: `a Hu-` 94,695 : `az Hu-` 529, i.e. `a` in 99.4% of tokens, and the `az Hu-` contexts are almost entirely pre-reform literature (`az Hugó`, `az Humprey`, `az Huszain aga`) plus the acronym-like `az Hungexpo` - so the `az` variant before `h` is archaic rather than a live alternative. `y`-initial names remain genuinely split (`az Y-` 3,501 : `a Y-` 78,525, 4.3% `az`) and the `az` tokens track names pronounced with an initial vowel (`az Ybl`, `az Yves`), which is exactly what the rule predicts.
- **Predictability**: `lexical` (foreign pronunciation data, open class)
- **Lexical input**: pronunciation respelling of foreign items (`PH` foreign pronunciation)
- **Interactions**: `PND-ART-001`; same pronunciation drives suffixes (`ORT-FOR-016`).
- **Sources**: [AkH12 §276 principle: article follows the pronounced form], [LP-HU] (Unix, Unicode exceptions; study only), [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens; oracle:WikiPron respellings; attestation (regex scan over six registers, readings checked in 6 contexts per pattern)
- **Confidence**: `consensus` (principle); `measured` for `h`-initial names (`a`, 99.4%, the `az` variant archaic) and for `y`-initial names (genuinely split by pronunciation); `contested` only for individual names with two live pronunciations (`OQ-04-010` resolved)

### PND-ART-003 Article before cardinal numbers written in digits
- **Statement**: Before a number written in digits the article follows the first word of its spelled-out form (`PND-CARD-003`); `az` exactly when that word begins with `egy` or `öt`, or with `ezer`.
- **Conditions**: integer digit string, optional grouping spaces or periods (`PUN-NUMF`), optional sign, optional decimal part.
- **Output**: Let `D` be the digit string without separators and leading zeros, `n` its length, `d1` its first digit.
  - `az` if `d1` = `5` (`öt`, `ötven`, `ötszáz`, `ötezer`, `ötvenezer`, `ötmillió` ...).
  - `az` if `d1` = `1` and `n mod 3 = 1` (`1` `egy`, `1xxx` `ezer...`, `1 xxx xxx` `egymillió...`, `1 xxx xxx xxx` `egymilliárd...`).
  - `a` otherwise (`1x` `tíz/tizen-`, `1xx` `száz...`, `1x xxx` `tizen...ezer`, `1xx xxx` `százezer...`, digits `2 3 4 6 7 8 9`).
  - `0` (`nulla`): `a`. Negative numbers (`mínusz`): `a`.
- **Examples**: `az 1`, `az 5`, `az 50`, `az 500`, `az 1848-as`, `az 1956-os`, `az 5000`, `az 55 000`, `az 1 200 000`; `a 2`, `a 10`, `a 15`, `a 100`, `a 150`, `a 12 000`, `a 180 000`, `a 2024-es`, `a 0`, `a -5`.
- **Exceptions**: decimal and fraction notation depends on the chosen reading: `az 1,5` (`egy egész öt tized`) ~ `a 1,5` (`másfél`); `a ½` (`fél`); treat as variants (`PND-FRAC-007`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-CARD-003` (spelled-out form); `PND-ART-004` ordinals; `PND-NUMINF-010` suffixes.
- **Sources**: [AkH12 §276 principle], [LP-HU] (partial list 1, 5, 5x, 1xxx, 5xxx; study only); derived from `PND-CARD` composition
- **Validation**: goldens (exhaustive property test against the spell-out generator)
- **Confidence**: `consensus`

### PND-ART-004 Article before ordinals in digits and Roman numerals
- **Statement**: Before an ordinal written as digits with a period or as a Roman numeral the article follows the first word of the spelled-out ordinal, which starts with the same word as the cardinal except `első` (1.) and `második` (2.).
- **Conditions**: `N.` (digits + period) or Roman numeral with period in ordinal use (`PND-ORD-004`).
- **Output**: the `PND-ART-003` decision on the integer value; `1.` `első` and `2.` `második` give the same result as the cardinal rule (`az`, `a`).
- **Examples**: `az 1. hely`, `az 5. osztály`, `az 50. évforduló`, `az 1000. vendég`; `a 2. emelet`, `a 11. fejezet`, `a 100. adás`; `az I. kerület`, `az V. kerület`, `az L. évfolyam`, `az M.` (`ezredik`); `a II. kerület`, `a IV.-be jár`, `a X. kerület`, `a XI. kerület`, `a XIII. kerületben`.
- **Exceptions**: a Roman-looking letter used as a letter (`az X`, letter name `iksz`; `a C` `cé`; `az M` `em`) follows `PND-ART-005`; disambiguation by the period and context (`az X-generáció` vs `a X. kerület`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-ORD-004` (Roman value), `PND-ART-005`
- **Sources**: [AkH12 §276, §294], [LP-HU] (Roman exceptions i, ii, iv, x; study only)
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-ART-005 Article before letters and letter names
- **Statement**: Before a letter cited as a symbol (label, variable, list item, grade) the article follows the Hungarian name of the letter.
- **Conditions**: single letter or digraph used metalinguistically or as a label.
- **Output**: Hungarian letter names and resulting article:

| Letter | Name | Article | Letter | Name | Article |
|---|---|---|---|---|---|
| a á e é i í o ó ö ő u ú ü ű | the vowel | `az` | m | `em` | `az` |
| b | `bé` | `a` | n | `en` | `az` |
| c | `cé` | `a` | ny | `eny` | `az` |
| cs | `csé` | `a` | p | `pé` | `a` |
| d | `dé` | `a` | q | `kú` | `a` |
| dz | `dzé` | `a` | r | `er` | `az` |
| dzs | `dzsé` | `a` | s | `es` | `az` |
| f | `ef` | `az` | sz | `esz` | `az` |
| g | `gé` | `a` | t | `té` | `a` |
| gy | `gyé` | `a` | ty | `tyé` | `a` |
| h | `há` | `a` | v | `vé` | `a` |
| j | `jé` | `a` | w | `dupla vé` | `a` |
| k | `ká` | `a` | x | `iksz` | `az` |
| l | `el` | `az` | y | `ipszilon` | `az` |
| ly | `elipszilon` | `az` | z | `zé` | `a` |
| | | | zs | `zsé` | `a` |

- **Examples**: `az f) pont`, `az m betű`, `az x tengely`, `az y koordináta`, `az s betű`, `az A kategória`, `a b) pont`, `a c) pont`, `a h hang`, `a k szám`, `a z tengely`.
- **Exceptions**: letters read with a foreign name in context (mathematics in English-influenced usage, `y` as `[váj]`) are not generated.
- **Predictability**: `rule`
- **Lexical input**: letter name table (closed, 44 entries: 40 Hungarian letters + q w x y) [AkH12 §10-11 letters; names by consensus]
- **Interactions**: `PND-ART-006` (initialisms), `WS-ALPH`
- **Sources**: [AkH12 §10, §11, §276], [LP-HU] (vowel-initial names a e f i l m n o r s u x y; study only)
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-ART-006 Article before acronyms
- **Statement**: Before an acronym the article follows its spoken form: letter-by-letter initialisms by the first letter name (`PND-ART-005`), word-read acronyms and blends by their word pronunciation.
- **Conditions**: acronym type is a lexical property (`spelled` | `word`); unknown all-caps strings default to `spelled` if they contain no vowel letter or are not pronounceable as one Hungarian syllable chain, else lexicon lookup.
- **Output**: article by first pronounced segment.
- **Examples**: spelled: `az MTA` (`em-té-á`), `az MSZP`, `az SZTK`, `az FTC`, `az RTL`, `az LMP`, `az SMS`, `az MP3`, `az OTP`, `az EU`, `az USB`, `az URL`, `a KSH`, `a BKV`, `a HVG`, `a DNS`, `a CD`, `a DVD`, `a PDF`, `a GDP`, `a HR`; word-read: `a NATO` (`nátó`), `a MÁV`, `a MOL`, `a Mahart`, `a tb`, `az ELTE`, `az ENSZ` (`ensz`), `az UEFA`, `az UNESCO` (`uneszkó`), `az IKEA`, `az Iposz`, `az áfa`.
- **Exceptions**: acronyms with both readings produce both articles only when the readings differ in onset class (`az FBI` both ways; `a CNN` both ways; `az NBA` both ways): no conflict; conflicts are rare and lexical.
- **Predictability**: `lexical` (reading type per acronym, open class); `rule` once the reading is known
- **Lexical input**: acronym reading type and word pronunciation
- **Interactions**: `ORT-ABBR-014`, `ORT-ABBR-016`, `ORT-ABBR-018` (suffixes follow the same reading), `PND-ART-005`
- **Sources**: [AkH12 §276 (a MÁV, a Mahart, a tb; az OTP, az FTC, az MTA, az Iposz, az áfa), §285, §288]
- **Validation**: goldens; attestation
- **Confidence**: `normative` (principle and AkH examples); `consensus` for other items

### PND-ART-007 Article before abbreviations and symbols
- **Statement**: Abbreviations (rövidítés) and symbols are read as the full word they stand for, and the article follows that word.
- **Conditions**: abbreviation with period (`u.`, `dr.`), abbreviation without period (`Ft`, `km`), unit and currency symbols, signs (`%`, `§`, `°C`, `€`, `$`).
- **Output**: article by the onset of the expansion (`MEAS` expansions; abbreviation lexicon `ORT-ABBR`).
- **Examples**: `az igh.` (`igazgatóhelyettes`), `az i. m.` (`idézett mű`), `az ún.`, `az ill.`, `az ápr.`, `az okt.`, `az ifj.`, `az id.`, `az u.` (`utca`); `a szerk.`, `a f. k.`, `a mb. ea.`, `a kb.`, `a Bp.`, `a dr.`; `a %` (`százalék`), `a §` (`paragrafus`), `a km`, `a kg`, `a Ft`, `a °C`, `az €` (`euró`), `a $` (`dollár`), `a m²` (`négyzetméter`).
- **Exceptions**: abbreviations that are themselves read as letters (`kft.` `káefté`, `bt.` `béte`) follow the letter reading: `a kft.`, `a bt.`, `az nyrt.` (`enyerté`), `a zrt.`.
- **Predictability**: `lexical` (expansion per abbreviation, closed standard list AkH12 §284 of 125 entries, `ORT-ABBR-019`, plus open additions)
- **Lexical input**: abbreviation and symbol expansions
- **Interactions**: `PND-MEAS-001`, `ORT-ABBR-001`, `ORT-ABBR-019`
- **Sources**: [AkH12 §276, §279-§284]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-ART-008 Intervening punctuation and parentheticals
- **Statement**: Punctuation between the article and the next word is transparent; the first pronounced word after the article decides, including the first word of a parenthetical that directly follows the article.
- **Conditions**: opening quotation marks, brackets, dashes, emphasis markup, line breaks and hyphenation points between the article and the next word.
- **Output**: article by the first pronounced word after skipping non-pronounced characters.
- **Examples**: `az „ablak” szó`, `a „kapu” szó`, `az (egyébként régi) ház`, `a (nagyon) öreg ház`, `az [5] hivatkozás` (`öt`).
- **Exceptions**: a parenthetical that the reader skips aloud is not modelled; generation never inserts such a parenthetical.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-ART-001`, `WS-CHAR` (character classes)
- **Sources**: derived from the pronunciation principle [AkH12 §276]; [LP-HU] (word-based check; study only)
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-ART-009 No assimilation of the article in writing; indefinite egy
- **Statement**: The article is always written `a` or `az` regardless of pronunciation assimilation (`az` + consonant-initial word is never written, nor `a` assimilated); the indefinite article (határozatlan névelő) is invariably `egy`.
- **Conditions**: any article occurrence.
- **Output**:
  - Definite: `az` or `a` by `PND-ART-001` only; no forms `azt`, `abba` for the article (those are demonstrative forms, `PND-DEM-002`).
  - Indefinite: `egy` before any onset; never inflected, never lengthened, never `eggy-` (that spelling belongs only to the numeral's case forms `eggyel`, `eggyé`, `PND-NUMINF-001`).
  - The numeral `egy` and the article `egy` are homographs; they differ in stress and in the phonetic length of `gy` [KVF98 p. 95]; disambiguation is syntactic (`SYN`).
- **Examples**: `a könyv` (never `*az könyv`), `az ember` (never `*a ember`); `egy ház`, `egy alma`, `egy 5 éves gyerek`, `egy ötéves gyerek`.
- **Exceptions**: nonstandard `a` before vowels in transcribed speech is recognised as nonstandard (`REP`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-DEM-002`, `PND-NUMINF-001`, `REP`
- **Sources**: [KVF98 pp. 94-95], [AkH12 §69 (eggyel)]
- **Validation**: goldens
- **Confidence**: `normative` (spelling); `consensus` (description)

---

## PERS: personal pronouns

### PND-PERS-001 Nominative personal pronouns
- **Statement**: Personal pronouns distinguish three persons and two numbers; there is no gender, no inclusive/exclusive and no politeness distinction inside this set (formal address is `PND-PERS-010`).
- **Conditions**: nominative (subject, predicate, pronominal possessor base).
- **Output**:

| Person | Singular | Plural |
|---|---|---|
| 1 | `én` | `mi` |
| 2 | `te` | `ti` |
| 3 | `ő` | `ők` |

- **Examples**: `Én megyek.`, `Te maradsz.`, `Ő orvos.`, `Mi itt vagyunk.`, `Ti hol laktok?`, `Ők tanárok.`
- **Exceptions**: dialectal `mink`, `tik` are nonstandard (recognise only); `ő`/`ők` as subject or object refer to humans (and personified animals); inanimates are referred to by `az` (`PND-DEM`) [KVF98 p. 261] (tendency, not a grammatical ban).
- **Predictability**: `lexical` (closed class, 6 forms)
- **Lexical input**: pronoun table
- **Interactions**: subject pro-drop and agreement `SYN`; `PND-PERS-002` .. `PND-PERS-009`
- **Sources**: [KVF98 pp. 260-261], [SMNy3 pp. 609-611]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-PERS-002 Accusative personal pronouns
- **Statement**: The accusative forms of personal pronouns are suppletive in 1st and 2nd person and regular in 3rd person; 1pl and 2pl have two equal standard variants.
- **Conditions**: direct object.
- **Output**:

| Person | Singular | Plural |
|---|---|---|
| 1 | `engem` ~ `engemet` | `minket` ~ `bennünket` |
| 2 | `téged` ~ `tégedet` | `titeket` ~ `benneteket` |
| 3 | `őt` | `őket` |

  - Preferred: `engem`, `téged` (`engemet`, `tégedet` colloquial or regional: `engem` [ATT 17] vs `engemet` [ATT 8], about 23:1; `téged` [ATT 16] vs `tégedet` [ATT 5], about 45:1).
  - `minket` [ATT 18] vs `bennünket` [ATT 17], `titeket` [ATT 13] vs `benneteket` [ATT 12]: both about 1.4:1, free variation, no preference.
  - Verb agreement: 1st and 2nd person pronoun objects take the indefinite conjugation (1sg subject + 2nd person object: `-lak/-lek`); 3rd person objects take the definite conjugation (`VRB-PRES-005`).
- **Examples**: `Engem nem hívtak.`, `Téged szeretlek.`, `Szeretsz minket?`, `Várunk benneteket.`, `Látod őt?`, `Ismered őket?`
- **Exceptions**: none.
- **Predictability**: `lexical` (closed, 8 forms)
- **Lexical input**: pronoun table
- **Interactions**: reflexive objects `PND-REFL-002`
- **Sources**: [KVF98 pp. 261-262, 270], [ATT]
- **Validation**: goldens; attestation
- **Confidence**: `consensus`; variant preference `tendency`

### PND-PERS-003 Oblique cases: case stem + personal suffix
- **Statement**: All other cases of personal pronouns are formed by adding a possessive-type personal suffix to a pronominal stem of the case suffix; the pronoun itself is not a stem.
- **Conditions**: dative, instrumental, causal-final, inessive, elative, illative, superessive, sublative, delative, adessive, allative, ablative.
- **Output**:

| Case | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| dative `-nAk` | `nekem` | `neked` | `neki` | `nekünk` | `nektek` | `nekik` |
| instrumental `-vAl` | `velem` | `veled` | `vele` | `velünk` | `veletek` | `velük` |
| causal-final `-ért` | `értem` | `érted` | `érte` | `értünk` | `értetek` | `értük` |
| inessive `-bAn` | `bennem` | `benned` | `benne` | `bennünk` | `bennetek` | `bennük` |
| elative `-bÓl` | `belőlem` | `belőled` | `belőle` | `belőlünk` | `belőletek` | `belőlük` |
| illative `-bA` | `belém` | `beléd` | `bele` ~ `belé` | `belénk` | `belétek` | `beléjük` |
| superessive `-(O)n` | `rajtam` | `rajtad` | `rajta` | `rajtunk` | `rajtatok` | `rajtuk` |
| sublative `-rA` | `rám` | `rád` | `rá` | `ránk` | `rátok` | `rájuk` |
| delative `-rÓl` | `rólam` | `rólad` | `róla` | `rólunk` | `rólatok` | `róluk` |
| adessive `-nÁl` | `nálam` | `nálad` | `nála` | `nálunk` | `nálatok` | `náluk` |
| allative `-hOz` | `hozzám` | `hozzád` | `hozzá` | `hozzánk` | `hozzátok` | `hozzájuk` |
| ablative `-tÓl` | `tőlem` | `tőled` | `tőle` | `tőlünk` | `tőletek` | `tőlük` |

- **Examples**: `Adj nekem!`, `Veled megyek.`, `Bízom benned.`, `Rád gondolok.`, `Tőlük kaptam.`, `Hozzánk jött.`; 3sg illative `bele` is the default (67,815 : `belé` 16,465 as a pronoun [WC2, `[/N|Pro][Ill][3Sg]`]), `belé` is literary and emphatic.
- **Exceptions**: 3sg illative: `belé` in Kenesei's table and literary use, `bele` in current usage. The homograph is now controlled by the tag filter, the same strings also being verbal prefixes: as a pronoun `bele` 67,815 : `belé` 16,465 (80.5%), as a prefix `bele` 1,658,556 : `belé` 21,419 (98.7%) [WC2]. `belé` is not archaic - one pronoun token in five - and peaks in literary text, so both stay generated with `bele` as the formal default (`OQ-04-008` resolved).
- **Predictability**: `lexical` (closed, 72 forms; suffixes regular per `PND-PERS-004`)
- **Lexical input**: case-stem table (`nek-`, `vel-`, `ért-`, `benn-`, `belől-`, `belé-`, `rajt-`, `rá-`, `ról-`, `nál-`, `hozzá-`, `től-`)
- **Interactions**: `PND-PERS-004` (suffix allomorphy), `PND-PERS-007` (emphatic), `ORT-LEN` (single `l`, `n` in these forms: `belőlem`, `nálad`, `tőlünk`; but `benne`, `bennünk`) [AkH12 §72]
- **Sources**: [KVF98 pp. 270-271], [SMNy3 pp. 612-614], [AkH12 §72], [morphdb PERS_PRON_PARADIGM, PP_1SG..PP_3PL], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens (full table); attestation (tag-filtered pronoun vs prefix counts, plus a register split)
- **Confidence**: `consensus`; `measured` for the 3sg illative default, with a literary gradient for `belé`

### PND-PERS-004 Personal suffix allomorphy on case stems and postpositions
- **Statement**: The personal suffixes on pronominal case stems and on case-like postpositions are the singular-possessed possessive suffixes selected by stem end and stem harmony, with a zero 3sg after vowel-final stems and no `-a/-e` possessive marker.
- **Conditions**: stem = case stem (`PND-PERS-003`) or postposition (`PND-POSTP-005`); stem harmony `B` or `F`/`R` (front rounded stems take `e`, never `ö`, in the suffix vowel slot; `-ünk`, `-ük` are the only rounded front vowels).
- **Output**:

| Stem end | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| consonant, `B` | `-am` | `-ad` | `-a` | `-unk` | `-atok` | `-uk` |
| consonant, front | `-em` | `-ed` | `-e` | `-ünk` | `-etek` | `-ük` |
| long vowel `á`, `B` | `-m` | `-d` | zero | `-nk` | `-tok` | `-juk` |
| long vowel `é`, front | `-m` | `-d` | zero | `-nk` | `-tek` | `-jük` |

  - Front rounded stems do not take `ö` in the suffix: `mögöttem`, `mögöttetek`, `fölöttünk`, `tőletek`, `belőletek`, `mögületek`, `nélkületek`.
  - Irregular 3sg `neki` (not `*neke`), 3pl `nekik` (not `*nekük`); 2pl `nektek` (no linking vowel after `nek-`).
- **Examples**: `rajtam`, `rajtatok`, `rajtuk`; `nálam`, `nálatok`; `velem`, `veletek`, `velük`; `rám`, `rátok`, `rájuk`; `belém`, `belétek`, `beléjük`; `alattam`, `alatta`, `alattuk`; `felém`, `felé`, `feléjük`.
- **Exceptions**: after `é`-final postpositions a 3sg `-je` and after `á` a 3sg `-ja` are colloquial variants: `feléje` [ATT 8], `eléje` [ATT 7], `melléje` [ATT 5], `rája` [ATT 5]; generate the zero form.
- **Predictability**: `rule` (given the stem list)
- **Lexical input**: stem harmony, stem end
- **Interactions**: `NOM-POSS-001` (same suffix inventory), `PND-POSTP-005`
- **Sources**: [morphdb PP_1SG..PP_3PL], [KVF98 pp. 88, 270-271], [SMNy3 p. 612], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-PERS-005 Cases and suffixes unavailable to personal pronouns
- **Statement**: Personal pronouns have no forms for the non-local minor cases; the relation is expressed by a substitute or a paraphrase.
- **Conditions**: requested case = terminative `-ig`, temporal `-kor`, translative `-vÁ`, essive `-Ul`, essive-formal `-ként`, formal `-képp(en)`, sociative `-stUl`, distributive `-nként`, multiplicative `-szOr`, modal-essive `-An`, locative `-t/-tt`; also plural `-k` on `én`, `te`, `mi`, `ti`, possessive suffixes on the pronoun, `-é` on 1st and 2nd person pronouns (suppletive possessive pronouns instead, `PND-REFL-005`).
- **Output**:
  - terminative -> allative (`hozzám` 'up to me') or postposition `-ig` paraphrase (`egészen hozzám`);
  - translative, essive-formal -> not formed for the pronoun (`*énné`, `*énként`); paraphrase with `olyan, mint én`. The noun `én` 'self, ego' inflects regularly (`énné` 212, `énként` 347 [WC2]);
  - `ő` + `-é` -> `övé` (`PND-REFL-005`).
- **Examples**: `*énig`, `*tekor`, `*őként`; `Eljött egészen hozzánk.`
- **Exceptions**: 3sg `ő` behaves as a noun only with `-é` (suppletive `övé`) and the plural `ők`.
- **Predictability**: `lexical` (closed gap list)
- **Lexical input**: pronoun paradigm gap list
- **Interactions**: `NOM-CASE-001`, `PND-REFL-005`
- **Sources**: [KVF98 p. 270], [SMNy3 pp. 612-614]
- **Validation**: goldens (generator must refuse)
- **Confidence**: `consensus`

### PND-PERS-006 Archaic and literary variants of case forms
- **Statement**: Several case-stem forms have archaic or literary variants that are recognised and can be generated under an archaic register flag.
- **Conditions**: register `archaic` | `literary`.
- **Output**:
  - dative `nékem`, `néked`, `néki`, `nékünk`, `néktek`, `nékik` [ATT `nékem` 9];
  - sublative `reám`, `reád`, `reá`, `reánk`, `reátok`, `reájuk` (AkH lists `rá` ~ `reá` as an accepted doublet; `rám` [ATT 15] vs `reám` [ATT 8], about 11:1);
  - causal-final `érettem`, `éretted`, `érette`, `érettünk`, `érettetek`, `érettük` (AkH doublet `értem` ~ `érettem`; `értem` ambiguous with 'I understand');
  - 3sg illative `belé` (`PND-PERS-003`).
- **Examples**: `Reád gondolok.`, `Érettünk tette.`, `Nékem is jó.`
- **Exceptions**: none.
- **Predictability**: `lexical` (closed, 19 forms)
- **Lexical input**: register flag
- **Interactions**: `PND-PERS-003`
- **Sources**: [AkH12 §25 a) rá ~ reá, §44 értem ~ érettem], [KVF98 p. 271], [ATT]
- **Validation**: goldens; attestation
- **Confidence**: `normative` for `reá`, `érettem` as accepted doublets; `consensus` for register labels

### PND-PERS-007 Emphatic doubled forms
- **Statement**: For emphasis the nominative pronoun is prefixed to its case-stem form or suffixed postposition and written together with it; the prefix does not trigger harmony or assimilation, and 3pl uses `ő`.
- **Conditions**: case-stem forms (`PND-PERS-003`) and personal-suffixed postpositions (`PND-POSTP-005`); emphasis or contrast.
- **Output**: `én` / `te` / `ő` / `mi` / `ti` / `ő` + the suffixed form of the same person, one word.
- **Examples**: `énnekem`, `teneked`, `őneki`, `minekünk`, `tinektek`, `őnekik`; `énvelem`, `teveled`, `ővele`, `ővelük`; `énbennem`, `tenálad`, `őtőle`, `mihozzánk`, `őrá`, `énrólam`; `énmiattam`, `teáltalad`, `őutána`, `mimellettünk`, `tinélkületek`, `őközöttük`.
- **Exceptions**:
  - no `*ők-` prefix (`*őkvelük`);
  - no assimilation or harmony across the boundary (`énvelem`, not `*énnelem`; `őróluk`, not `*őrőlük`) [SMNy3 p. 613];
  - the accusative has no such form (`engem`, `téged` are already emphatic-capable); separated sequences with a different structure stay apart (`én magam`, `PND-REFL-003`);
  - reduplication `velem-velem` is not an emphatic formation (do not generate).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-TOG-031` (writing together), `ORT-LEN-031`, `PND-PERS-003`, `PND-POSTP-008`
- **Sources**: [AkH12 §122], [KVF98 p. 270], [SMNy3 p. 613], [ATT `énnekem` 5, `őrá` 6, `ővele` 4, `ővelük` -]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-PERS-008 Pronominal possessor
- **Statement**: A personal pronoun possessor stands in the nominative after the definite article, is optional (the possessive suffix carries person), and in 3pl is `ő`, not `ők`.
- **Conditions**: possessive noun phrase with a pronominal possessor.
- **Output**: `az én N-Poss1sg`, `a te N-Poss2sg`, `az ő N-Poss3sg`, `a mi N-Poss1pl`, `a ti N-Poss2pl`, `az ő N-Poss3pl`; dative possessor `nekem a N-Poss1sg` for extraction (`SYN`).
- **Examples**: `az én házam` ~ `a házam`, `a te könyved`, `az ő autója`, `a mi utcánk`, `a ti kertetek`, `az ő házuk` (not `*az ők házuk`); `Nekem a házam kicsi.`
- **Exceptions**: the formal pronouns possess like nouns: `az ön autója`, `az önök háza`, `a maga autója`, `a maguk háza` (`PND-PERS-010`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `NOM-POSS-001`, `SYN` (possessive constructions)
- **Sources**: [KVF98 pp. 94, 214-216], [SMNy3 pp. 595-596]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-PERS-009 Third person pronoun with postpositions and verbal prefixes
- **Statement**: The 3sg case-stem forms `hozzá`, `neki`, `rá`, `bele`, `rajta`, `utána` are homographous with verbal prefixes (igekötő); as pronouns they are written apart from the verb, as prefixes together.
- **Conditions**: sequence 3sg case form + verb.
- **Output**: pronoun reading (reference to a person or thing, stressed as pronoun): apart; prefix reading (lexicalised verb): together.
- **Examples**: `hozzá ment feleségül` ('married him') vs `hozzájárul`; `neki megy a levél` vs `nekimegy a szekrénynek`; `rá gondolok` vs `rágondolni is rossz`; personal-suffixed forms other than 3sg are always apart: `hozzám adták`, `nekünk jött`, `rád gondolok`.
- **Exceptions**: none.
- **Predictability**: `rule` given the reading; reading is `SYN`/semantic
- **Lexical input**: prefix-verb lexicon (`VRB-PREV`)
- **Interactions**: `ORT-TOG-030`, `VRB-PREV`
- **Sources**: [AkH12 §120 e)]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-PERS-010 Formal address pronouns ön, önök, maga, maguk
- **Statement**: Formal 2nd person address uses the pronouns `ön`/`önök` (more formal) and `maga`/`maguk` (less formal), which inflect as nouns (`ön`) or as the reflexive 3rd person forms (`maga`) and take 3rd person agreement throughout.
- **Conditions**: formal address (T/V choice is forms of address (out of scope)).
- **Output**:
  - Paradigm of `ön` (regular front consonant-final noun, harmony `R`): `ön`, `önt`, `önnek`, `önnel`, `önért`, `önben`, `önből`, `önbe`, `önön`, `önre`, `önről`, `önnél`, `önhöz`, `öntől`, `öné`; plural `önök`, `önöket`, `önöknek`, `önökkel`, `önökért`, `önökben`, `önökön`, `önökre`, `önöktől`, `önöké`.
  - Paradigm of `maga`/`maguk`: identical to reflexive 3sg/3pl (`PND-REFL-002`): `magát`, `magának`, `magával`, `magában`, `magára`, `magától`, `magáé`; `magukat`, `maguknak`, `magukkal`.
  - Agreement: verb 3sg/3pl (`Ön hol lakik?`, `Önök mit kérnek?`, `Maga honnan jön?`); as object the verb is definite (`Önt várom.`, `Magát keresem.`, `Önöket üdvözöljük.`), unlike `téged`/`titeket`.
  - Postpositions: noun-like, no personal suffix on the postposition: `ön mellett`, `ön nélkül`, `önök után`; with `maga`: `maga mellett`, `maga nélkül` (ambiguous with reflexive).
  - Possessor: `az ön autója`, `az önök háza`, `a maga kabátja`.
  - Pronoun + noun apposition: `maguk orvosok` possible, `*önök orvosok` not [KVF98 p. 269].
  - Capitalisation: lowercase by default inside a text; capital initial allowed for heightened respect (`Ön`, `Önök`, `Önöké`) [AkH12 §147]; both are correct.
- **Examples**: `Ön mit gondol?`, `Köszönöm önnek.`, `Önökkel beszélek.`, `Maga is jön?`, `Hozok magának egy kávét.`
- **Exceptions**: archaic `kegyed` [ATT 4], dialect `kend` are recognised only; `maga` in address can be perceived as impolite (sociolinguistic, forms of address (out of scope)).
- **Predictability**: `rule` (inflection); `lexical` (4 lemmas)
- **Lexical input**: none beyond lemmas
- **Interactions**: -
- **Sources**: [KVF98 pp. 266-269], [AkH12 §147], [ATT `önt` 15, `önnek` 17, `önöket` 13]
- **Validation**: goldens; attestation
- **Confidence**: `consensus`; capitalisation `normative`

### PND-PERS-011 Numeral and quantifier associated with a pronoun
- **Statement**: A number or quantity of persons referred to by a pronoun is expressed by the collective `-An` form of the numeral when the pronoun is the subject, and by the numeral with a possessive personal suffix otherwise.
- **Conditions**: pronoun + numeral or quantifier (`sok`, `kevés`, `néhány`, `mind`, `hány`).
- **Output**:
  - subject: collective adverbial (`PND-NUMINF-005`): `ketten vagyunk`, `hárman jöttek`, `sokan voltunk`;
  - non-subject: numeral stem + possessive suffix + case: `kettőnk`, `kettőtök`, `kettejük` ~ `kettőjük` (`kettejük` [ATT 10] vs `kettőjük` [ATT 6], about 4:1), `hármunk`, `hármatok`, `hármuk`; `mindkettőnk`, `mindkettőjük`, `mindhármunk`; `A fiú kettőnket várja.`, `mindhármunkat meghívták`;
  - emphatic pronoun before it takes the case: `minket kettőnket`, `nekik húszuknak`.
- **Examples**: `Mindketten ott voltunk.`, `Mind a hatan eljöttek.`, `Kettőnk közül ki megy?`
- **Exceptions**: possessive forms of numerals other than 2 are rare in the corpus (`hármuk` [ATT 1]; `hármunk`, `ötünk`, `tízünk` unattested, though KVF98 gives `huszuknak`, `mind a tízüknek`); generate by `NOM-POSS` on the numeral stem (`PND-NUMINF-001`) and mark `?` for low frequency.
- **Predictability**: `rule` (form); `tendency` (variant `kettejük`)
- **Lexical input**: numeral stems
- **Interactions**: -
- **Sources**: [KVF98 pp. 265-266], [ATT]
- **Validation**: goldens; attestation
- **Confidence**: `consensus`

---

## DEM: demonstratives

### PND-DEM-001 Demonstrative inventory
- **Statement**: Demonstratives form proximal (`e`-/`i`-) and distal (`a`-/`o`-/`ú`-) pairs across nominal, adjectival, numeral and adverbial categories, with emphatic `em-`/`am-`, identity `ugyan-` and collective `mind-` series.
- **Conditions**: lexical inventory.
- **Output**:

| Category | Proximal | Distal | Inflects |
|---|---|---|---|
| nominal 'this/that' | `ez` | `az` | case, plural, `-é` |
| emphatic contrast | `emez` | `amaz` | case, plural |
| identity | `ugyanez` | `ugyanaz` | case, plural |
| collective | `mindez` | `mindaz` | case, plural |
| quality | `ilyen`, `ily` | `olyan`, `oly` | adjectival (`PND-DEM-009`) |
| emphatic quality | `emilyen` | `amolyan` | adjectival |
| kind | `efféle`, `effajta`, `ilyenféle`, `ilyesféle`, `ilyesmi` | `afféle`, `affajta`, `olyanféle`, `olyasféle`, `olyasmi` | nominal/adjectival |
| size | `ekkora` | `akkora` | adjectival (`ekkorát`) |
| amount | `ennyi` | `annyi` | numeral-like (`ennyit`, `ennyien`, `ennyiszer`), no plural |
| place (at) | `itt`, `emitt` | `ott`, `amott` | `-ig` (`idáig`/`odáig` see below) |
| place (to) | `ide`, `emide` | `oda`, `amoda` | - |
| place (from) | `innen` ~ `innét` | `onnan` ~ `onnét` | - |
| direction | `erre` | `arra` | `-ról`: `erről`/`arról` |
| manner | `így` | `úgy` | - |
| time | `ekkor`, `most` | `akkor` | `ekkorra`, `akkorra` |
| extent | `eddig`, `idáig` | `addig`, `odáig` | `eddigre`, `addigra` |
| cause | `ezért`, `emiatt` | `azért`, `amiatt` | - |
| occasion | `ilyenkor` | `olyankor` | - |

- **Examples**: `Ez az enyém, az a tiéd.`, `Nem ezt kérem, hanem emezt.`, `ugyanaz a könyv`, `mindez igaz`, `ilyen idő`, `ekkora ház`, `ennyi pénz`, `Gyere ide!`, `Így kell csinálni.`
- **Exceptions**: `most` is not formally demonstrative; `innét`, `onnét` dated/regional variants (`honnét` [ATT 6] vs `honnan` [ATT 15] shows the same pattern).
- **Predictability**: `lexical` (closed, about 60 lemmas)
- **Lexical input**: demonstrative lexicon with category and proximity
- **Interactions**: correlatives `PND-REL-006`; `PND-DEM-002`
- **Sources**: [KVF98 pp. 275-278], [AkH12 §83, §88 b) (ily, oly spelled ly)]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-DEM-002 z-assimilation of ez, az before consonant-initial suffixes
- **Statement**: The final `z` of `ez`, `az` and of every demonstrative ending in them (`emez`, `amaz`, `ugyanez`, `ugyanaz`, `mindez`, `mindaz`) assimilates totally to the first consonant of a consonant-initial suffix, written by doubling that consonant letter (digraph: first letter doubled).
- **Conditions**: stem `...ez`/`...az` + suffix beginning with a consonant other than `t` of the accusative and other than `v` of `-vAl`/`-vÁ` (`PND-DEM-003`).
- **Output**: `e`/`a` + doubled suffix-initial consonant + rest of suffix; harmony `F` for `ez`, `B` for `az`.

| Case | `ez` | `az` |
|---|---|---|
| nominative | `ez` | `az` |
| accusative | `ezt` | `azt` |
| dative | `ennek` | `annak` |
| instrumental | `ezzel` ~ `evvel` | `azzal` ~ `avval` |
| causal-final | `ezért` | `azért` |
| translative | `ezzé` | `azzá` |
| terminative | `eddig` | `addig` |
| essive-formal | `ekként` | `akként` |
| formal | `ekképp` ~ `ekképpen` | `akképp` ~ `akképpen` |
| temporal | `ekkor` | `akkor` |
| inessive | `ebben` | `abban` |
| elative | `ebből` | `abból` |
| illative | `ebbe` | `abba` |
| superessive | `ezen` | `azon` |
| sublative | `erre` | `arra` |
| delative | `erről` | `arról` |
| adessive | `ennél` | `annál` |
| allative | `ehhez` | `ahhoz` |
| ablative | `ettől` | `attól` |
| anaphoric possessive | `ezé` | `azé` |
| `-beli` | `ebbeli` | `abbeli` |

- **Examples**: `amannak`, `mindettől`, `ugyanahhoz`, `ugyanabban`, `mindebben`, `mindezzel`, `ugyanazzal`.
- **Exceptions**:
  - terminative `eddig`/`addig` (historical `-d` + `-ig`): lexical, `*ezig`;
  - formal `ekképp(en)`/`akképp(en)` is the AkH spelling; `eképpen`/`aképpen` [ATT 3/-] occur beside `ekképpen` [ATT 6] and are nonnormative;
  - `emez`, `amaz` oblique forms other than `amannak` are rare (`emebben`, `amabban` unattested); generate by rule, mark `?`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-ASSIM-008`, `PND-DEM-003`, `PND-DEM-004`, `PND-DEM-011`
- **Sources**: [AkH12 §83, §37], [KVF98 pp. 277-278], [morphdb AZ_CASE, EZ_CASE], [ATT]
- **Validation**: goldens (full table)
- **Confidence**: `normative`

### PND-DEM-003 Non-assimilating suffixes after ez, az
- **Statement**: Vowel-initial suffixes and the accusative `-t` attach to `ez`/`az` without assimilation; plural and superessive take the linking vowel of a front (`ez`) or back (`az`) stem.
- **Conditions**: accusative `-t`, plural `-(O)k`, superessive `-(O)n`, `-ért`, `-é`.
- **Output**: `ezt`, `azt`; `ezek`, `azok` (linking `e`/`o`); `ezen`, `azon`; `ezért`, `azért`; `ezé`, `azé`; plural stems inflect regularly: `ezeket`, `azokat`, `ezeknek`, `azoknak`, `ezekkel`, `azokkal`, `ezekben`, `azokban`, `ezekért`, `azokért`, `ezeké`, `azoké`.
- **Examples**: `Ezeket kérem.`, `Azokról beszéltünk.`, `Ezeken a napokon.`
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `NOM-PL-002`, `NOM-CASE-011`
- **Sources**: [KVF98 pp. 277-278]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-DEM-004 Instrumental and translative of ez, az
- **Statement**: In the instrumental either the suffix `v` assimilates to `z` (`ezzel`, `azzal`) or the `z` assimilates to `v` (`evvel`, `avval`); both are standard, the `z` forms are strongly preferred; the translative has only the `z` form.
- **Conditions**: `ez`/`az` (and prefixed series) + `-vAl` or `-vÁ`.
- **Output**: instrumental `ezzel` ~ `evvel`, `azzal` ~ `avval` (`ezzel`, `azzal` [ATT 22] vs `evvel`, `avval` [ATT 8], about 128:1; generate `ezzel`, `azzal`); translative `ezzé`, `azzá` only.
- **Examples**: `Ezzel kész.`, `Avval a különbséggel, hogy...`, `Azzá vált.`
- **Exceptions**: `emezzel`, `amazzal`, `ugyanezzel` [ATT 7], `mindezzel` [ATT 8]: same rule; `evvel` variants of prefixed forms are rare.
- **Predictability**: `rule` (form); `tendency` (preference)
- **Lexical input**: none
- **Interactions**: `ORT-SUFX-006`, `NOM-CASE-003`
- **Sources**: [AkH12 §44, §83], [KVF98 p. 277], [ATT]
- **Validation**: goldens; attestation
- **Confidence**: `normative` (both correct); `tendency` (preference)

### PND-DEM-005 Plural of demonstratives
- **Statement**: Nominal and adjectival demonstratives take the plural; amount demonstratives do not.
- **Conditions**: plural reference.
- **Output**: `ezek`, `azok`, `emezek`, `amazok`, `ugyanezek`, `ugyanazok`, `mindezek`, `mindazok`, `ilyenek`, `olyanok`, `ekkorák`, `akkorák`, `effélék`, `olyasmik`; no `*ennyik`, `*annyik`.
- **Examples**: `Ilyeneket mondott.`, `Mindezek után...`, `Ekkorák a különbségek.`
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: category (`amount` has no plural)
- **Interactions**: `NOM-PL`; `ilyen`, `olyan` are non-lowering (`ilyenek`, `olyanok`), `ekkora` lengthens (`ekkorák`, `NOM-LENG-001`)
- **Sources**: [KVF98 p. 277]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-DEM-006 External demonstrative agrees with the head noun
- **Statement**: A demonstrative (`ez`, `az`, `emez`, `amaz`, `ugyanez`, `ugyanaz`, `mindez`, `mindaz`) used before a noun is followed by the definite article and repeats the case suffix, the plural `-k` and the anaphoric `-é` of the whole noun phrase; before a postposition the postposition is repeated and the demonstrative is `a`/`e` before a consonant-initial postposition and `az`/`ez` before a vowel-initial one.
- **Conditions**: `DEM + a/az + (modifiers) + N`.
- **Output**:
  - article obligatory: `ez a ház`, `*ez ház`;
  - case copied: `ebben a házban`, `azt a könyvet`, `ennek a lánynak`, `arra a székre`, `azzal a fiúval`;
  - plural copied: `ezek a házak`, `azokkal a fiúkkal`; with a numeral the noun is singular and so is the demonstrative: `ez a három ház`, `ezt a három könyvet`, `azzal a két fiúval`;
  - `-é` copied: `azé a lányé`;
  - possessive suffixes, possessed plural `-i` and `-ék` are not copied: `ez a házam`, `ezek a házaim`, `ennél a lányoméknál` (singular demonstrative);
  - postposition copied: `e mellett a ház mellett`, `a mögött a fa mögött`, `e fölé a kép fölé`; vowel-initial: `az alatt a fa alatt`, `az előtt a ház előtt`, `az után a film után`, `ez előtt a ház előtt`;
  - oblique-case-governing postpositions repeat the case, not the postposition: `azzal a fiúval együtt`, `ahhoz a házhoz képest`.
- **Examples**: `Ebben a városban lakom.`, `A kulcs az alatt a könyv alatt van.`, `A kulcs e mellett a könyv mellett van.`
- **Exceptions**: emphatic separated demonstrative + postposition is written apart (`PND-DEM-009`).
- **Predictability**: `rule`
- **Lexical input**: postposition onset
- **Interactions**: `PND-DEM-002`, `PND-POSTP-011`, `SYN` (NP structure)
- **Sources**: [KVF98 pp. 87, 95, 277-278], [SMNy3 pp. 606-609]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-DEM-007 Internal determiners e, ez, ezen, eme and a, az, azon, ama
- **Statement**: A formal or literary internal demonstrative stands directly before the noun phrase without the article and does not inflect: `e` before a consonant and `ez` before a vowel (proximal), `ezen`, `eme`; distal `azon`, `ama` (and archaic `a`/`az` in fixed phrases).
- **Conditions**: formal, official or literary register; no definite article follows.
- **Output**: undeclined determiner + inflected noun.
- **Examples**: `e házban`, `e célból`, `e héten`, `ez évben`, `ez ügyben`, `ez alkalommal`, `ezen alkalomból`, `ezen könyvek`, `eme sorok`, `ama napon`, `azon esetben`, `az idő tájt`.
- **Exceptions**: `ezen`/`azon` + article (`ezen a napon`) is the superessive of the external demonstrative (`PND-DEM-006`), not the internal determiner; a possessor may precede the internal determiner (`Anna ama kalapja`) [KVF98 p. 95].
- **Predictability**: `rule` (form); register `lexical`
- **Lexical input**: register flag
- **Interactions**: `PND-DEM-006`, `SYN`
- **Sources**: [KVF98 pp. 95-96], [AkH12 §121 (e szerint, e miatt)]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-DEM-008 Adjectival and numeral demonstratives in nominal use
- **Statement**: `ilyen`, `olyan`, `ekkora`, `akkora`, `ennyi`, `annyi` and their prefixed forms are undeclined before a noun and inflect like adjectives or numerals when the noun is omitted.
- **Conditions**: head noun absent, or predicative use.
- **Output**:
  - `ilyen`, `olyan`: accusative `ilyet` ~ `ilyent`, `olyat` ~ `olyant` (`ilyet`, `olyat` [ATT 14] vs `ilyent`, `olyant` [ATT 5], about 23:1; generate the truncated form); other cases on the full stem: `ilyennek`, `olyannal`, `ilyenben`, `ilyenné`, `olyankor`; plural `ilyenek`, `olyanok`;
  - `ekkora`, `akkora`: lengthening stem `ekkorá-`: `ekkorát`, `akkorának`, `akkorára`, plural `akkorák`;
  - `ennyi`, `annyi`: `ennyit`, `annyinak`, `ennyivel`, `annyiba`, `ennyiért`; collective `ennyien`, `annyian`; multiplicative `ennyiszer`, `annyiszor`.
- **Examples**: `Ilyet még nem láttam.`, `Akkorát esett.`, `Annyit ettem!`, `Ennyien még nem voltunk.`
- **Exceptions**: harmony: `ennyi` is front (`ennyiszer`, `ennyien`), `annyi` back (`a` + transparent `i`: `annyiszor`, `annyian`).
- **Predictability**: `rule` (truncated accusative on every lemma ending in `ilyen`/`olyan`: `ilyen`, `olyan`, `milyen`, `amilyen` and prefixed forms such as `ugyanolyan`, `bármilyen`, `akármilyen`; `akármilyet` [ATT 1], `ugyanolyat` [ATT 0])
- **Lexical input**: stem ending `ilyen`/`olyan`
- **Interactions**: `NOM-ADJ`, `NOM-LENG-001`, `PND-INT-003`
- **Sources**: [KVF98 pp. 275-277], [ATT]
- **Validation**: goldens; attestation
- **Confidence**: `consensus`; accusative preference `tendency`

### PND-DEM-009 Demonstrative + postposition compounds
- **Statement**: A demonstrative and a nominative-governing postposition are written together when they form an adverb, a correlative (utalószó) or a conjunction-like unit, with `a`/`e` before a consonant-initial and `az`/`ez` before a vowel-initial postposition; they are written apart when the demonstrative points emphatically.
- **Conditions**: `a`/`az`/`e`/`ez` + postposition from `PND-POSTP-002`, `PND-POSTP-003`.
- **Output**:
  - together: `afelé`, `efelé`, `afelől`, `efelől`, `afölött`, `efölött`, `amiatt`, `emiatt`, `amellett`, `emellett`, `anélkül`, `enélkül`, `ahelyett`, `ehelyett`, `aszerint`, `eszerint`, `evégett`, `aközben`, `eközben`, `amögött`, `emögött`, `azalatt`, `ezalatt`, `azelőtt`, `ezelőtt`, `azután`, `ezután`, `azóta`, `ezáltal`, `azáltal`;
  - apart (emphatic pointing): `a mögé tedd, amelyik elég széles`, `a szerint kell eljárni, nem e szerint`, `e miatt nem működik`, and always in `PND-DEM-006` agreement structures (`a köré a ház köré`).
  - superessive-governing adverbs: `ezenkívül` [ATT 14], `azonkívül` [ATT 8] ('besides') together; `ezen kívül` ('outside this') apart.
- **Examples**: `Ezelőtt másképp hívták.`, `Elment anélkül, hogy köszönt volna.`, `Azóta nem láttam.`, `Emellett dolgozik is.`
- **Exceptions**: `ezóta` is marginal (`?ezóta`, unattested); `aközött` rare.
- **Predictability**: `rule` (form); reading choice `SYN`/semantic
- **Lexical input**: postposition onset; reading
- **Interactions**: `ORT-ASSIM-010`, `ORT-TOG-029`, `PND-DEM-006`
- **Sources**: [AkH12 §84, §121], [ATT]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-DEM-010 ez, az + -féle, -fajta, -kora, -nyi
- **Statement**: `ez`/`az` combined with `-féle`, `-fajta`, `-kora` and `-nyi` show total `z`-assimilation written as a doubled consonant, always written as one word.
- **Conditions**: lexicalised combinations.
- **Output**: `efféle`, `afféle`, `effajta`, `affajta`, `ekkora`, `akkora`, `ennyi`, `annyi` (`z` + `ny` -> `nny`).
- **Examples**: `afféle ember`, `effajta dolgok`, `ekkora baj`.
- **Exceptions**: `ilyenféle`, `olyanféle`, `ilyesféle`, `olyasféle` are built on `ilyen`/`olyan` without assimilation.
- **Predictability**: `lexical` (closed, 8 forms)
- **Lexical input**: none
- **Interactions**: `ORT-ASSIM-009`
- **Sources**: [AkH12 §83 (`-fajta`, `-féle`, `-kora` only; `ennyi`, `annyi` by the same assimilation, not listed there)], [KVF98 p. 275]
- **Validation**: goldens
- **Confidence**: `normative` (`-fajta`, `-féle`, `-kora`); `consensus` (`-nyi`)

### PND-DEM-011 Emphatic and identity prefixes on demonstratives
- **Statement**: `ugyan-`, `mind-`, `szint-`, `csak-`, `épp-` combine with demonstratives into one word and the result inflects like its demonstrative base.
- **Conditions**: prefix directly before the demonstrative.
- **Output**: `ugyanez`, `ugyanaz`, `ugyanolyan`, `ugyanakkora`, `ugyanannyi`, `ugyanott`, `ugyanakkor`; `mindez`, `mindaz`, `mindannyi`; `szintolyan`, `szintakkora`, `szintannyi`; `csakolyan`, `csakannyi`; `éppolyan`, `éppakkora`, `éppannyi`; inflection `ugyanabban` [ATT 12], `ugyanazzal` [ATT 10], `mindettől` [ATT 3].
- **Examples**: `Ugyanazt kérem.`, `Éppannyit fizettem.`
- **Exceptions**: separated when the first element is not a prefix: `épp olyan esetben`, `csak annyi szükséges`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-TOG-031`, `PND-DEM-002`
- **Sources**: [AkH12 §83, §122], [KVF98 pp. 95-96, 275], [ATT]
- **Validation**: goldens
- **Confidence**: `normative`

---

## INT: interrogatives

### PND-INT-001 Interrogative inventory
- **Statement**: Interrogative pro-forms (kérdő névmás, kérdőszó) cover persons, things, selection, quality, size, quantity, order, place, direction, time, cause, manner and possession; the same forms introduce indirect questions.
- **Conditions**: lexical inventory.
- **Output**:

| Category | Form(s) | Inflects |
|---|---|---|
| person | `ki`, `kicsoda` | case, plural `kik` |
| thing | `mi`, `micsoda` | case, plural `mik` |
| selection | `melyik`, `mely` (literary) | case (`melyiket`), personal suffixes (`melyikünk`) |
| quality | `milyen`, `miféle`, `mifajta` (rare), `minő` (archaic) | adjectival |
| size | `mekkora` | adjectival (`mekkorát`) |
| quantity | `mennyi` (mass and count), `hány` (count) | numeral-like |
| order | `hányadik` | ordinal-like (`hányadikat`, `hányadika`) |
| place at / to / from | `hol`; `hová` ~ `hova`; `honnan` ~ `honnét` | - |
| direction | `merre`, `merről` | - |
| extent, time | `meddig`, `mikor`, `mióta`, `mikorra`, `mennyi ideig` | - |
| cause, purpose | `miért`, `minek`, `mi végett` | - |
| manner | `hogyan` ~ `hogy`, `miként`, `miképp(en)`, `hogyhogy` | - |
| degree | `mennyire`, `milyen` + adjective | - |
| possessor | `kié`, `kiké`; attributive `kinek a`, `minek a` | case (`kiét`) |

- **Examples**: `Ki az?`, `Mit keresel?`, `Melyiket kéred?`, `Hányan jöttek?`, `Hányadik sorban ülsz?`, `Hová mész?`, `Kié ez a táska?`, `Kinek a táskája ez?`
- **Exceptions**: `hanyadik` [ATT 1] is a colloquial pronunciation spelling of `hányadik` [ATT 7]; `mifajta` unattested in the corpus.
- **Predictability**: `lexical` (closed, about 35 lemmas)
- **Lexical input**: interrogative lexicon
- **Interactions**: `PND-REL-001`, `PND-INDF-001` (series built on these bases); question syntax `SYN`
- **Sources**: [KVF98 pp. 278-281, 347], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-INT-002 Inflection of ki and mi
- **Statement**: `ki` and `mi` inflect as front vowel-final nouns (harmony `F`, no lengthening), except that the terminative and temporal of `mi` are suppletive `meddig` and `mikor`.
- **Conditions**: case, plural, `-é`.
- **Output**:

| Case | `ki` | `mi` |
|---|---|---|
| accusative | `kit` | `mit` |
| dative | `kinek` | `minek` |
| instrumental | `kivel` | `mivel` |
| causal-final | `kiért` | `miért` |
| translative | `kivé` | `mivé` |
| inessive / elative / illative | `kiben`, `kiből`, `kibe` | `miben`, `miből`, `mibe` |
| superessive / sublative / delative | `kin`, `kire`, `kiről` | `min`, `mire`, `miről` |
| adessive / allative / ablative | `kinél`, `kihez`, `kitől` | `minél`, `mihez`, `mitől` |
| terminative | `?kiig` | `meddig` |
| temporal | - | `mikor` |
| essive-formal / formal | - | `miként`, `miképp(en)` |
| `-é` | `kié` | `mié` (rare) |
| plural | `kik`, `kiket`, `kiknek` ... | `mik`, `miket`, `miknek` ... |

- **Examples**: `Kivel beszéltél?`, `Miről szól?`, `Kiket vársz?`, `Mivé lett?`, `Mi után szaladtál?`
- **Exceptions**: `miért` is both causal-final 'for what' and 'why'; `minek` both dative and 'what for'; `mivel` also conjunction 'since'; `mikor` also relative/conjunction (colloquial).
- **Predictability**: `rule` with 3 lexical forms
- **Lexical input**: suppletive table
- **Interactions**: `NOM-CASE`, `PND-REL-002`, `PND-INDF-002`
- **Sources**: [KVF98 pp. 278-279, 347], [morphdb NON_PERS_PRON]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-INT-003 Inflection of melyik, milyen, mekkora, mennyi, hány, hányadik
- **Statement**: Selective, quality, size and quantity interrogatives are undeclined before a noun and inflect when used alone, by the stem classes of their demonstrative counterparts.
- **Conditions**: nominal use.
- **Output**:
  - `melyik`: `melyiket`, `melyiknek`, `melyikkel`; personal partitive `melyikünk`, `melyikőtök` ~ `melyiketek`, `melyikük`; plural `melyikek` (rare); `mely` (literary): `melyet`, `melyek`;
  - `milyen`: `milyet` [ATT 6] (`milyent` unattested), `milyennek`, `milyenek`; `miféle`: `mifélét`, `mifélék`;
  - `mekkora`: `mekkorát` [ATT 4], `mekkorák`;
  - `mennyi`: `mennyit`, `mennyiért`, `mennyibe`, `mennyien` [ATT 6], `mennyiszer`; no plural;
  - `hány` (lowering): `hányat`, `hányan` [ATT 11], `hányszor`, `hányas`, `hányadik`, `hányad`, `hányféle`; `hányban` ('in which year'), `hánykor` ('at what time'), `hánytól`, `hányig`;
  - `hányadik`: `hányadikat`, `hányadikban`, `hányadika` ('which day of the month'), `hányadikán`.
- **Examples**: `Mekkorát ugrott?`, `Mennyiért vetted?`, `Hányban születtél?`, `Hánykor indulunk?`, `Hányadika van ma?`
- **Exceptions**: none beyond the truncated accusative `milyet` (`PND-DEM-008`).
- **Predictability**: `rule`
- **Lexical input**: `hány` lowering; truncated accusative flag
- **Interactions**: `PND-DEM-008`, `PND-NUMINF`, `NOM-LOW`
- **Sources**: [KVF98 pp. 279-280], [morphdb NON_PERS_PRON], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-INT-004 Variants hová ~ hova, honnan ~ honnét, hogyan ~ hogy
- **Statement**: Some interrogative adverbs have two standard forms in free variation, inherited by derived relative and indefinite forms.
- **Conditions**: lexical doublets.
- **Output**: `hová` ~ `hova` (both [ATT 14], no preference; AkH lists the doublet); `honnan` ~ `honnét` (`honnan` [ATT 15] vs `honnét` [ATT 6], about 23:1; `honnét` dated/regional); `hogyan` ~ `hogy` (manner; `hogy` also the complementiser). Inherited: `ahová` ~ `ahova`, `valahová` ~ `valahova`, `sehová` ~ `sehova`, `mindenhová` ~ `mindenhova`, `bárhová` ~ `bárhova`; `ahogyan` ~ `ahogy`.
- **Examples**: `Hova mész?` = `Hová mész?`
- **Exceptions**: none.
- **Predictability**: `lexical` (closed, 3 doublets)
- **Lexical input**: doublet table
- **Interactions**: `ORT-SUFX-007`
- **Sources**: [AkH12 §25 a)], [KVF98 pp. 280-281], [ATT]
- **Validation**: goldens; attestation
- **Confidence**: `normative` (both correct); `tendency` for `honnét`

---

## REL: relatives

### PND-REL-001 Relative pro-forms: a- + interrogative
- **Statement**: Relative pronouns and adverbs (vonatkozó névmás) are formed by prefixing `a-` to an interrogative base and are written as one word.
- **Conditions**: interrogative base from `PND-INT-001`.
- **Output**:

| Base | Relative | Base | Relative |
|---|---|---|---|
| `ki` | `aki` | `hol` | `ahol` |
| `mi` | `ami` | `hová` ~ `hova` | `ahová` ~ `ahova` |
| `mely` | `amely` | `honnan` | `ahonnan` |
| `melyik` | `amelyik` | `merre` | `amerre` |
| `milyen` | `amilyen` | `mikor` | `amikor` |
| `mekkora` | `amekkora` | `mióta` | `amióta` |
| `mennyi` | `amennyi` | `meddig` | `ameddig` |
| `hány` | `ahány` | `miért` | `amiért` |
| `hányadik` | `ahányadik` | `hogy(an)` | `ahogy(an)` |
| `miként` | `amiként` | `mennyire` | `amennyire` |

  - Conjunctions from the same pattern: `amint`, `amíg`, `amennyiben` (`ahelyett` is demonstrative `a` + postposition, `PND-DEM-009`).
- **Examples**: `a férfi, akivel táncoltam`, `az utca, ahol lakunk`, `amikor tízéves voltam`, `annyi, amennyi kell`.
- **Exceptions**: no relative counterpart of `hogyhogy`; `akié` (from `kié`) is rare.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-INT-001`, `PND-REL-002`, clause syntax `SYN`
- **Sources**: [KVF98 pp. 281-282], [ATT `amely` 25, `ami` 25, `aki` 25, `ahol` 23, `amikor` 24, `ahogy` 21, `amerre` 9]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-REL-002 Inflection of relatives
- **Statement**: A relative inflects exactly like its interrogative base, including suppletive and truncated forms.
- **Conditions**: case, plural, `-é`, postposition.
- **Output**: `akit`, `akinek`, `akivel`, `akiért`, `akiben`, `akire`, `akitől`, `akik`, `akiket`, `akié`, `akiké`; `amit`, `aminek`, `amivel`, `amiből`, `amik`, `amiket`; `amelyet`, `amelynek`, `amellyel`, `amelyben`, `amelyek`, `amelyeket`; `amelyiket`, `amelyikben`; `amilyet`, `amekkorát`, `amennyit`, `ahányan`, `ahányszor`, `ameddig`; relative + postposition apart, the postposition unsuffixed: `aki mellett`, `ami után`, `amely alatt`, `aki nélkül`.
- **Examples**: `a könyvek, amiket vettél`, `a kutya, amely után szaladtam`, `akinek szól`.
- **Exceptions**: `amely` + `-vAl` is `amellyel` (`ly` doubled as `lly`, `ORT-ASSIM-016`); conjunction compounds written together (`miután`, `mielőtt`, `mialatt`, `mióta`) are distinct from interrogative + postposition `mi után` (apart).
- **Predictability**: `rule`
- **Lexical input**: base paradigm
- **Interactions**: `PND-INT-002`, `PND-INT-003`, `NOM-CASE-003`
- **Sources**: [KVF98 p. 282], [ATT `amelyet` 21, `amit` 24, `amelyek` 23, `amik` 16]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-REL-003 Choice by antecedent: aki, ami, amely, amelyik
- **Statement**: `aki` refers to persons; `amely` and `ami` refer to non-persons; `amelyik` selects from a set.
- **Conditions**: antecedent type.
- **Output**:
  - person antecedent: `aki` (`a fiú, aki...`); human collectives take `amely`/`ami` (`a bizottság, amely...`);
  - animals: `amely` preferred (`?a farkas, ami az úton áll`); pets and personified animals: `aki` colloquial;
  - mass and non-specific antecedents: `ami` preferred (`a pénz, amit kölcsönöztél`);
  - `amelyik`: 'which one of the set' (`Azt kérem, amelyik piros.`); colloquially also for `amely`.
- **Examples**: `az orvos, akihez megyek`, `a ház, amelyben lakunk`, `a kutya, amely ugat`.
- **Exceptions**: see `PND-REL-004` for the prescriptive `ami`/`amely` split.
- **Predictability**: `rule` (person vs non-person); `tendency` otherwise
- **Lexical input**: animacy of the antecedent
- **Interactions**: -
- **Sources**: [KVF98 pp. 281-282, 40-43]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-REL-004 Prescriptive distinction amely vs ami
- **Statement**: The prescriptive tradition uses `amely` when the antecedent is a noun (phrase) denoting a specific non-person, and `ami` when the antecedent is a pronoun, a quantifier, a nominalised adjective, the whole preceding clause, or absent; current usage extends `ami` to noun antecedents.
- **Conditions**: non-person antecedent.
- **Output**:
  - `amely` (formal, prescriptive): noun antecedent: `a könyv, amelyet olvasok`;
  - `ami` (all registers): antecedent `az`, `ez`, `valami`, `semmi`, `minden`, `sok`, `kevés`, `más`: `azt, amit mondtál`; `minden, ami szép`; superlative or neuter adjective: `a legjobb, ami történhetett`; whole clause: `Elkésett, ami bosszantott.`; headless: `Ami sok, az sok.`;
  - `ami` with a noun antecedent (`a könyv, amit olvasok`): standard in informal and increasingly in formal texts; generate `amely` in formal register, `ami` in informal, accept both.
- **Examples**: `a ház, amelyet eladtak` (formal) ~ `a ház, amit eladtak` (informal).
- **Exceptions**: plural with noun antecedent: `amelyek` preferred to `amik` (`amelyek` [ATT 23] vs `amik` [ATT 16], about 11:1), while singular `amely` and `ami` share bucket 25 (not separable by antecedent type in a word list).
- **Predictability**: `tendency` (no antecedent-typed corpus counts available; `Open questions`)
- **Lexical input**: antecedent type, register flag
- **Interactions**: -
- **Sources**: [KVF98 p. 282] (amely for countable/specific, colloquial ami, amelyik), [HELYES-AMI] (NYTI Helyes blog 2014-03-18: `amely` after a noun in careful writing, `ami` after a noun not an error; `ami` for clause and unnamed antecedents), [ATT]
- **Validation**: attestation (needs sentence corpus); goldens for the unambiguous `ami` contexts
- **Confidence**: `contested` for noun antecedents; `consensus` for pronoun, clause and headless antecedents

### PND-REL-005 Literary and colloquial relative forms
- **Statement**: Relative pro-forms without `a-` occur in literary style (`ki`, `mely`, `mi`, `hol`), and interrogative forms without `a-` occur as relatives in colloquial speech (`mikor`, `hol`).
- **Conditions**: register.
- **Output**: literary `ki` = `aki` (`Ki korán kel, aranyat lel.`), `mely` = `amely` (`a ház, mely...`, `mely` [ATT 22]); colloquial `mikor` = `amikor` (`mikor` [ATT 21]); `ahol` for time (`abban a percben, ahol...`) is nonstandard (generate `amikor`).
- **Examples**: `a hely, hol született` (literary); `Mikor megjött, szóltam.` (colloquial).
- **Exceptions**: none.
- **Predictability**: `lexical` (register)
- **Lexical input**: register flag
- **Interactions**: `PND-REL-001`
- **Sources**: [KVF98 pp. 281-282], [ATT]
- **Validation**: attestation
- **Confidence**: `consensus`

### PND-REL-006 Correlative pairs
- **Statement**: A relative clause is typically anticipated by the demonstrative of the same category in the main clause, in the case required there.
- **Conditions**: correlative (utalószó) construction.
- **Output**: `az ... aki/ami/amely`, `ott ... ahol`, `oda ... ahová`, `onnan ... ahonnan`, `akkor ... amikor`, `addig ... ameddig/amíg`, `azóta ... amióta`, `annyi ... amennyi`, `ahány ... annyi`, `olyan ... amilyen`, `akkora ... amekkora`, `úgy ... ahogy(an)`, `azért ... amiért`.
- **Examples**: `Azt kérem, amit ígértél.`, `Ott lakom, ahol te.`, `Annyit adok, amennyit kérsz.`
- **Exceptions**: the correlative may be omitted when its case matches (`Aki keres, talál.`).
- **Predictability**: `rule` (pairing)
- **Lexical input**: none
- **Interactions**: `PND-DEM-001`, `SYN`
- **Sources**: [KVF98 pp. 37-45, 264]
- **Validation**: goldens
- **Confidence**: `consensus`

---

## REFL: reflexive, reciprocal and possessive pronouns

### PND-REFL-001 Reflexive pronoun maga: nominative forms
- **Statement**: The reflexive pronoun is the stem `mag-` with possessive personal suffixes; it has a back harmony, lowering linking vowel `a`.
- **Conditions**: reflexive or emphatic use.
- **Output**:

| Person | Singular | Plural |
|---|---|---|
| 1 | `magam` | `magunk` |
| 2 | `magad` | `magatok` |
| 3 | `maga` | `maguk` |

- **Examples**: `Én magam varrtam.`, `Magunk is jövünk.`
- **Exceptions**: 3sg/3pl are homographous with the formal address pronouns (`PND-PERS-010`).
- **Predictability**: `lexical` (closed, 6 forms)
- **Lexical input**: none
- **Interactions**: `PND-REFL-002`, `PND-PERS-010`
- **Sources**: [KVF98 p. 271]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-REFL-002 Case inflection of the reflexive
- **Statement**: Case suffixes attach regularly to the personal-suffixed reflexive forms (stems `magam-`, `magad-`, `magá-`, `magunk-`, `magatok-`, `maguk-`); 3sg lengthens `a` to `á`; the 1sg and 2sg accusative suffix is optional.
- **Conditions**: all cases except those unavailable semantically.
- **Output**:

| Case | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| accusative | `magamat` ~ `magam` | `magadat` ~ `magad` | `magát` | `magunkat` | `magatokat` | `magukat` |
| dative | `magamnak` | `magadnak` | `magának` | `magunknak` | `magatoknak` | `maguknak` |
| instrumental | `magammal` | `magaddal` | `magával` | `magunkkal` | `magatokkal` | `magukkal` |
| inessive | `magamban` | `magadban` | `magában` | `magunkban` | `magatokban` | `magukban` |
| superessive | `magamon` | `magadon` | `magán` | `magunkon` | `magatokon` | `magukon` |
| sublative | `magamra` | `magadra` | `magára` | `magunkra` | `magatokra` | `magukra` |
| allative | `magamhoz` | `magadhoz` | `magához` | `magunkhoz` | `magatokhoz` | `magukhoz` |
| ablative | `magamtól` | `magadtól` | `magától` | `magunktól` | `magatoktól` | `maguktól` |
| causal-final | `magamért` | `magadért` | `magáért` | `magunkért` | `magatokért` | `magukért` |
| `-é` | `magamé` | `magadé` | `magáé` | `magunké` | `magatoké` | `maguké` |

  - Other cases (elative, illative, delative, adessive, translative, essive-formal) are equally regular: `magamból`, `magamba`, `magamról`, `magamnál`, `magává` (`magáévá tesz`).
  - Postpositions follow the nominative form: `maga mellett`, `magam előtt`, `magunk között`.
- **Examples**: `Látom magam(at) a tükörben.`, `Vett magának egy sálat.`, `Bízzatok magatokban!`, `Magától értetődik.`
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `NOM-CASE`, `NOM-POSS-011`, `NOM-LENG-001`
- **Sources**: [KVF98 pp. 271-272, 274]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-REFL-003 Emphatic and compound reflexives
- **Statement**: The reflexive is used appositively for emphasis and combines with `ön-`, `önnön-`, `egy-` into one word; `-d` ordinal + reflexive forms 'with N-1 others' compounds.
- **Conditions**: emphasis; compound formation.
- **Output**: `én magam` (apart), `önmagam`, `önmagad`, `önmaga`, `önmagunk`; `önnönmaga`, `önnönmagunk`, `önnönmagatok`; `egymagam`, `egymagad`, `egymaga` ('alone'); `másodmagammal`, `harmadmagával`, `századmagával`, `huszonötödmagunkkal`, `tizenkilenced-magunkkal` (hyphen by the 6:3 rule).
- **Examples**: `Önmagát hibáztatja.`, `Egymaga maradt.`, `Harmadmagával érkezett.`
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-TOG-031`, `ORT-TOG-040`, `ORT-63-002`, `PND-ORD-005`
- **Sources**: [AkH12 §122, §131], [KVF98 p. 272]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-REFL-004 Reciprocal pronoun egymás
- **Statement**: The single reciprocal pronoun `egymás` inflects as a regular back-harmony consonant-final noun, without person or number marking.
- **Conditions**: reciprocal reference to a plural antecedent.
- **Output**: `egymást`, `egymásnak`, `egymással`, `egymásért`, `egymásban`, `egymásból`, `egymásba`, `egymáson`, `egymásra`, `egymásról`, `egymásnál`, `egymáshoz`, `egymástól`, `egymásé`, `egymásét`; as possessor: `egymás haját`; with postpositions apart: `egymás mellett`, `egymás után`, `egymás között` ~ `egymás közt`, `egymás nélkül`; derivative written together: `egymásutániság`.
- **Examples**: `Szeretik egymást.`, `Ajándékot készítettek egymásnak.`, `A nyulak egymás után elpusztultak.`
- **Exceptions**: no plural, no personal suffix (`*egymásunk`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `NOM-CASE`, `ORT-TOG-036`
- **Sources**: [KVF98 pp. 272-274], [AkH12 §127], [ATT `egymást` 18]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-REFL-005 Possessive pronouns
- **Statement**: Independent possessive pronouns ('mine' ...) are suppletive in 1st and 2nd person and `ő` + `-é` in 3rd person; plural possession adds `-i-` before the person suffix.
- **Conditions**: predicative or elliptical possession.
- **Output**:

| Possessor | One possessed | Several possessed |
|---|---|---|
| 1sg | `enyém` | `enyéim` |
| 2sg | `tiéd` ~ `tied` | `tieid` |
| 3sg | `övé` | `övéi` |
| 1pl | `miénk` ~ `mienk` | `mieink` |
| 2pl | `tiétek` ~ `tietek` | `tieitek` |
| 3pl | `övék` | `övéik` |
| formal | `öné`, `önöké`; `magáé`, `maguké` | `önéi`, `önökéi`; `magáéi`, `magukéi` |
| reflexive | `magamé`, `magadé`, `magáé`, `magunké`, `magatoké`, `maguké` | `magaméi` ... |

  - Variants: AkH lists `mienk` ~ `miénk`, `tied` ~ `tiéd` as equal doublets; corpus: `tiéd` [ATT 10] vs `tied` [ATT 8] (2:1), `miénk` [ATT 10] vs `mienk` [ATT 6] (4:1), `tiétek` [ATT 4] vs `tietek` [ATT 1] (about 3:1). Generate the long-vowel forms, accept both.
  - Colloquial `enyémek` [ATT 3] for `enyéim` [ATT 1]: accept as nonstandard-colloquial, do not generate.
- **Examples**: `Az a kávé az enyém.`, `Ezek a könyvek az övéi.`, `A miénk nagyobb.`
- **Exceptions**: no `*övéje`, `*övéjük` [SMNy3 p. 596].
- **Predictability**: `lexical` (closed, 12 core forms + variants)
- **Lexical input**: possessive pronoun table
- **Interactions**: `NOM-ANP` (`-é`, `-éi`), `PND-REFL-006`
- **Sources**: [KVF98 pp. 273-274], [AkH12 §25 a)], [SMNy3 pp. 595-596], [ATT]
- **Validation**: goldens; attestation
- **Confidence**: `normative` (doublets); `tendency` (preference)

### PND-REFL-006 Inflection and use of possessive pronouns
- **Statement**: Possessive pronouns inflect as front-harmony nouns on their full form, usually with the definite article.
- **Conditions**: case marking of predicative/elliptical possessive pronouns.
- **Output**: `enyémet`, `enyémnek`, `enyémmel`, `enyémhez`; `tiédet` ~ `tiedet`, `tiédtől` ~ `tiedtől`; `övét`, `övének`, `övével`; `miénket`, `miénkkel`; `tiéteket`; `övéket`; plural possessed: `enyéimet`, `tieidet`, `övéit`, `mieinket`, `tieiteket`, `övéiket`; article: `az enyém`, `a tiéd`, `az övé`, `a miénk`.
- **Examples**: `Az én kutyám nem fél a tiédtől.`, `Az anyja az enyémet szereti.`
- **Exceptions**: without article in predicate position (`Ez enyém.`) is marked/poetic.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `NOM-ACC-001`, `NOM-ANP`, `SYN`
- **Sources**: [KVF98 pp. 273-274]
- **Validation**: goldens
- **Confidence**: `consensus`

---

## INDF: indefinite, negative, universal and free-choice pronouns; quantifiers

### PND-INDF-001 Pronoun series by prefix
- **Statement**: Indefinite (`vala-`), negative (`se-`, `sem-`, `semmi-`), universal (`minden-`), free-choice (`bár-`, `akár-`) and partitive (`né-`) series are formed by prefixing to interrogative bases, written as one word, with a closed set of irregular members.
- **Conditions**: interrogative base (`PND-INT-001`).
- **Output**:

| Base | `vala-` | negative | `minden-` | `bár-` | `akár-` |
|---|---|---|---|---|---|
| `ki` | `valaki` | `senki` | `mindenki` | `bárki` | `akárki` |
| `mi` | `valami` | `semmi` | `minden` | `bármi` | `akármi` |
| `hol` | `valahol` | `sehol` | `mindenhol` ~ `mindenütt` | `bárhol` | `akárhol` |
| `hová` | `valahová` | `sehová` | `mindenhová` | `bárhová` | `akárhová` |
| `honnan` | `valahonnan` | `sehonnan` | `mindenhonnan` | `bárhonnan` | `akárhonnan` |
| `mikor` | `valamikor`, `valaha` | `semmikor`, `soha` | `mindenkor`, `mindig` | `bármikor` | `akármikor` |
| `hogy(an)` | `valahogy(an)` | `sehogy(an)` | `mindenhogy(an)` (colloquial), `mindenképp(en)` | `bárhogy(an)` | `akárhogy(an)` |
| `merre` | `valamerre` | `semerre` | `mindenfelé` | `bármerre` | `akármerre` |
| `meddig` | - | `semeddig` | - | `bármeddig` | `akármeddig` |
| `melyik` | `valamelyik` | `semelyik` | `mindegyik` | `bármelyik` | `akármelyik` |
| `milyen` | `valamilyen` | `semmilyen` | `mindenféle` | `bármilyen` | `akármilyen` |
| `miféle` | `valamiféle` | `semmiféle` | `mindenféle` | `bármiféle` | `akármiféle` |
| `mekkora` | `valamekkora` | `semekkora` | - | `bármekkora` | `akármekkora` |
| `mennyi` | `valamennyi` | `semennyi` | - | `bármennyi` | `akármennyi` |
| `hány` | `valahány` | - | - | `bárhány` | `akárhány` |

  - `né-`: `néhány`, `némely`, `némelyik`, `némi`, `néha`, `néhol`, `némelykor`.
  - Irregular: `minden` (not `*mindenmi`), `semmi` (not `*semi`), `senki` (not `*seki`), `soha` (with `sohase`, `sohasem`, `sose`, `sosem`), `mindig`, `valaha`, `mindenütt`.
  - `valamennyi` is ambiguous: 'some amount' and 'all' (`valamennyi diák` 'every student', `valamennyien` 'all of us/them').
- **Examples**: `Valaki bejött.`, `Senki sem jött.`, `Bárki megcsinálhatja.`, `Mindenhol kerestem.`, `Néha eljön.`
- **Exceptions**: gaps marked `-` are unattested or nonstandard.
- **Predictability**: `rule` (formation) with `lexical` gaps and irregulars (closed, about 12)
- **Lexical input**: series table
- **Interactions**: `ORT-TOG-032`, `ORT-TOG-033`, `PND-INDF-002`, `PND-INDF-003`
- **Sources**: [AkH12 §123, §124], [KVF98 pp. 346-347], [morphdb NON_PERS_PRON (neg_pron, uni_pron filters)], [ATT `mindenhova` 7 = `mindenhová` 7, `valamennyien` 11]
- **Validation**: goldens
- **Confidence**: `normative` (writing); `consensus` (inventory)

### PND-INDF-002 Inflection of series members
- **Statement**: Members of the series inflect like their base, with the series prefix inert for harmony; `minden` inflects as a consonant-final front noun.
- **Conditions**: case, plural.
- **Output**: `valakit`, `valakinek`, `valakivel`, `valakik`; `valamit`, `valamiből`; `senkit`, `senkinek`, `senkivel`; `semmit`, `semmiből`, `semmiért`, `semmivé`; `mindenkit`, `mindenkinek`; `mindent`, `mindennek`, `mindennel`, `mindenben`, `mindenről`; `bárkit`, `bármit`, `akármivel`; `valamelyiket`, `mindegyiket`, `mindegyiknek`; `valamennyit`; `némelyik`, `némelyeket`.
- **Examples**: `Semmit sem értek.`, `Mindenkinek szól.`, `Bármivel játszhatsz.`
- **Exceptions**: `mindennek` is also the dative of `mindez` (`PND-DEM-002`); disambiguation syntactic.
- **Predictability**: `rule`
- **Lexical input**: base paradigm
- **Interactions**: `PND-INT-002`, `NOM-ACC-003` (`mindent`: bare `-t` after `n`)
- **Sources**: [KVF98 p. 347]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-INDF-003 Negative pronouns and negative concord (basics)
- **Statement**: Negative pronouns (`senki`, `semmi`, `sehol`, `soha`, `semmilyen`, `egyik ... sem` ...) co-occur with a clausal negator (`sem`/`se`/`nem`/`ne`); `se` and `sem` are written apart after a pronoun, but `sohase`, `sohasem`, `sose`, `sosem` are one word.
- **Conditions**: clause containing a negative pronoun.
- **Output**:
  - preverbal negative pronoun: `Senki sem jött.` ~ `Senki nem jött.`; `Semmit se láttam.`;
  - postverbal: `Nem jött senki.`, `Nem láttam semmit.`;
  - `se` ~ `sem` free variation (`se` more colloquial);
  - writing: `senki sem`, `semmi se`, `sehol sem`, `sehova se`; but `sohase`, `sose`, `sosem` [ATT `sosem` 17, `sose` 12, `sohase` 9].
  - `egyik ... sem` 'neither/none': `Egyik fiút sem látom.`
- **Examples**: see Output.
- **Exceptions**: in `se-` prefixed pronouns the prefix is part of the word (`sehol`), distinct from the separate particle.
- **Predictability**: `rule` (basics)
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [AkH12 §123], [KVF98 pp. 230-231, 346]
- **Validation**: goldens
- **Confidence**: `normative` (writing); `consensus` (concord basics)

### PND-INDF-004 Quantifiers sok, kevés, több, néhány, pár, számos, elég
- **Statement**: Closed-class quantifiers are undeclined before a singular noun and inflect as numerals when used alone, with stem alternations of their noun class.
- **Conditions**: nominal use; collective `-An`; multiplicative `-szOr`.
- **Output**:

| Quantifier | Accusative | Collective | `-szOr` | Other |
|---|---|---|---|---|
| `sok` | `sokat` (lowering) | `sokan` | `sokszor` | `sokadik`, comparative `több`, superlative `legtöbb` |
| `kevés` | `keveset` (shortening) | `kevesen` | `kevésszer` | `kevesebb`, `legkevesebb` |
| `több` | `többet` | `többen` | `többször` | `többes`; no ordinal |
| `néhány` | `néhányat` | `néhányan` | `néhányszor` | `egynéhány` |
| `pár`, `egypár` | `párat` | `páran` | `párszor` | 'a few' |
| `számos` | `?számosat` (unattested) | `számosan` | - | literary |
| `elég` | `eleget` (shortening) | - | - | - |
| `rengeteg`, `temérdek` | `rengeteget`, `temérdeket` | - | `rengetegszer` | - |
| `minden` | `mindent` | - | - | universal |
| `összes` | `az összeset` | - | - | with article |

  - Noun after the quantifier is singular and the verb agrees in singular: `sok ember jött`, `néhány könyvet vettem`.
- **Examples**: `Sokat dolgozik.`, `Kevesen voltunk.`, `Néhányszor jártam ott.`, `Eleget ettem.`
- **Exceptions**: `több` has only the colloquial ordinal `?többedik` ('for the umpteenth time', 3,793 : `sokadik` 47,567 [WC2]); `sokadik` is the standard quantifier ordinal besides `hányadik`.
- **Predictability**: `lexical` (closed, about 12) with `rule` inflection
- **Lexical input**: stem class (`kevés`, `elég` shortening; `sok`, `néhány`, `pár` lowering)
- **Interactions**: -
- **Sources**: [KVF98 pp. 229-230, 346], [ATT `sokan` 19, `kevesen` 14, `néhányan` 13, `többen` 17]
- **Validation**: goldens; attestation
- **Confidence**: `consensus`

### PND-INDF-005 mind, mindkét, mindkettő, mindhárom, mindannyi, mindnyájan
- **Statement**: The collective quantifier `mind` forms `mindkét` (attributive) / `mindkettő` (independent) and `mindhárom`; `mind a` + numeral works with all numerals; `mindannyian`, `mindnyájan` are collective adverbials.
- **Conditions**: universal quantification over a known set.
- **Output**: `mindkét kéz`, `mindkettőt`, `mindketten`, `mindkettőnk`; `mindhárom`, `mindhármat`, `mindhárman`, `mindhármunkat`; `mind a hat`, `mind a hatan`, `mind a tízüknek`; `mindannyian`, `mindnyájan` [ATT 11], `mindannyiunk`; `mind` alone as pronoun or adverb: `mind elmentek`.
- **Examples**: `Mindketten ott voltunk.`, `Mind a hatan eljöttek.`, `Mindhármunkat meghívták.`
- **Exceptions**: `*mindnégy` is not standard; use `mind a négy`.
- **Predictability**: `lexical` (closed)
- **Lexical input**: none
- **Interactions**: `PND-PERS-011`, `PND-CARD-004` (`két`/`kettő`)
- **Sources**: [KVF98 pp. 266, 346], [AkH12 §124], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-INDF-006 egyik, másik, más, egyéb, némelyik
- **Statement**: `egyik` and `másik` (with unique-identification `-ik`) are selective quantifiers that take personal suffixes and inflect regularly; `más` and `egyéb` mean 'other'.
- **Conditions**: selection from a set.
- **Output**: `egyik`, `egyiket`, `egyikünk`, `egyikőtök`, `egyikük`; `másik`, `másikat`, `másikunk`; `egyik a másikat`; `más`, `mást`, `mások`, `másokat`; `egyéb`, `egyebet` (shortening), `egyebek`; `némelyik`, `némelyikük`, `némelyek`.
- **Examples**: `Egyikük sem tudta.`, `Kérek egy másikat.`, `Mást mondott.`, `Egyebet nem kérek.`
- **Exceptions**: none.
- **Predictability**: `rule` with lexical stem classes
- **Lexical input**: `egyéb` shortening
- **Interactions**: `NOM-STEM-007`, `NOM-POSS`
- **Sources**: [KVF98 pp. 260, 268-269, 346], [ATT `egyikőtök` 2]
- **Validation**: goldens
- **Confidence**: `consensus`

---

## POSTP: postpositions

### PND-POSTP-001 Postposition vs case suffix
- **Statement**: A postposition (névutó) is a separate word written apart from its complement; unlike a case suffix it can be shared by coordinated complements, can take the `-i` adjectival suffix, and is not subject to vowel harmony with the complement.
- **Conditions**: distinguishing a relational element after a noun phrase.
- **Output**: diagnostics:
  - written apart (`ház mellett`, `ház-ban` is a suffix);
  - coordination: `Péter és Anna fölött` (postposition once) vs `Pétertől és Annától` (suffix on each conjunct, `*Péter és Annától`);
  - own harmony, no linking vowel, no stem alternation of the complement (`kéz alatt` vs `kezet`);
  - derivation `-i`: `ház melletti` (`PND-POSTP-010`); `*házbani`.
- **Examples**: `a bokor mögött`, `munka nélkül`, `szó szerint`, `többek között`.
- **Exceptions**: lexicalised compounds are solid (`délután`, `délelőtt`, `mindenekelőtt`, `rendszerint`, `hazafelé`, `munkanélküli` noun, `földalatti` noun) [AkH12 §108 b)].
- **Predictability**: `rule`
- **Lexical input**: postposition lexicon
- **Interactions**: `ORT-TOG-018`, `NOM-CASE-001`
- **Sources**: [KVF98 pp. 86-87], [SMNy3 pp. 604-606], [AkH12 §108]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-POSTP-002 Spatial postpositions governing the nominative: where, to where, from where
- **Statement**: Spatial case-like postpositions come in triads answering `hol?`, `hová?`, `honnan?` and govern an unmarked (nominative) noun phrase.
- **Conditions**: complement in nominative.
- **Output**:

| Meaning | `hol?` | `hová?` | `honnan?` |
|---|---|---|---|
| under | `alatt` | `alá` | `alól` |
| above, over | `fölött` ~ `felett` | `fölé` | `fölül` ~ `felül` |
| in front of, before | `előtt` | `elé` | `elől` |
| behind | `mögött` | `mögé` | `mögül` |
| beside | `mellett` | `mellé` | `mellől` |
| between, among | `között` ~ `közt` | `közé` | `közül` |
| around | `körül` | `köré` | - (`körül`) |
| towards / from the direction of | - | `felé` | `felől` |

  - `felett` [ATT 18] vs `fölött` [ATT 16] (1.4:1): free variants (AkH lists `fel` ~ `föl`); `között` [ATT 25] vs `közt` [ATT 18] (about 11:1): both standard, `között` preferred; `fölé` is 'to above', `felé` is 'towards' (generate `fölé` for the spatial triad).
- **Examples**: `a híd alatt`, `az asztal alá`, `a szőnyeg alól`, `a ház elé`, `a fák közül`, `a város felé`, `a Duna felől`.
- **Exceptions**: `fölül`/`felül` with superessive means 'beyond, in addition to' (`PND-POSTP-004`).
- **Predictability**: `lexical` (closed, 22 forms)
- **Lexical input**: postposition lexicon (meaning, direction)
- **Interactions**: `PND-POSTP-005`, `PND-POSTP-013`
- **Sources**: [KVF98 p. 89], [AkH12 §25 a), §34, §35, §38], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-POSTP-003 Temporal and abstract postpositions governing the nominative
- **Statement**: A further closed set of case-like postpositions governs the nominative.
- **Conditions**: complement in nominative.
- **Output**:

| Postposition | Meaning | Personal forms (`PND-POSTP-005`) |
|---|---|---|
| `után` | after | yes: `utánam` |
| `előtt` | before | yes |
| `alatt` | during, within | yes (spatial) |
| `óta` | since | no |
| `múlva` | (time) later, in | no |
| `közben` | during | no |
| `során` | in the course of | no |
| `folyamán` | during | no |
| `hosszat` | for (duration) | no |
| `felé` | towards, around (time) | yes |
| `táján`, `tájt` | around (time/place) | no |
| `szerint` | according to | yes: `szerintem` |
| `miatt` | because of | yes: `miattam` |
| `helyett` | instead of | yes: `helyettem` |
| `nélkül` | without | yes: `nélkülem`; also `nálam nélkül` |
| `iránt` | towards (attitude) | yes: `irántam` |
| `ellen` | against | yes: `ellenem` |
| `által` | by (agent), through | yes: `általam` |
| `végett` | for the purpose of | no |
| `gyanánt` | as | no |
| `folytán` | owing to | no |
| `mentén` | along | no |
| `ellenére`, `dacára` | despite | possessive-type (`PND-POSTP-009`) |

- **Examples**: `tegnap óta`, `egy hét múlva`, `munka közben`, `a vita során`, `órák hosszat`, `öt óra felé`, `szó szerint`, `az eső miatt`, `Péter helyett`, `cukor nélkül`, `a törvény ellen`, `a bizottság által`, `a folyó mentén`, `a nagy hó ellenére`.
- **Exceptions**: `ellenére`, `dacára` with an anaphoric demonstrative take the dative: `annak ellenére, hogy` [ATT `ellenére` 20]; `alapján`, `következtében`, `nyomán` are transitional (`PND-POSTP-009`).
- **Predictability**: `lexical` (closed, about 25)
- **Lexical input**: postposition lexicon with personal-form licence
- **Interactions**: `PND-POSTP-005`, `PND-POSTP-009`
- **Sources**: [KVF98 pp. 89-91, 247-254], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-POSTP-004 Postpositions governing an oblique case
- **Statement**: A closed set of postpositions governs a specific case suffix on the complement; they never take personal suffixes, and a pronominal complement is the case-marked personal pronoun.
- **Conditions**: complement in the governed case.
- **Output**:

| Governed case | Postpositions (meaning) | Example | With pronoun |
|---|---|---|---|
| superessive `-(O)n` | `át` (across, through), `keresztül` (through), `túl` (beyond), `innen` (on this side), `belül` (inside, within), `kívül` (outside, besides), `alul` (below), `felül` ~ `fölül` (above, beyond), `végig` (along) | `az erdőn át`, `a parkon keresztül`, `a hegyen túl`, `a Dunán innen`, `egy órán belül`, `a házon kívül`, `száz forinton felül`, `az utcán végig` | `rajtam kívül`, `rajtunk belül` |
| instrumental `-vAl` | `együtt` (together), `szemben` (opposite, against), `szemközt` (facing) | `Péterrel együtt`, `a házzal szemben` | `velem együtt`, `veled szemben` |
| allative `-hOz` | `képest` (compared to), `közel` (near), `mérten` (in proportion to), `híven` (faithful to), `hasonlóan` (similarly to) | `Szegedhez képest`, `a házhoz közel`, `a lehetőségekhez mérten` | `hozzám képest` |
| ablative `-tÓl` | `fogva` (from, since), `kezdve` (starting from) | `mától fogva`, `hétfőtől kezdve` | `tőlem kezdve` |
| adessive `-nÁl` | `fogva` (by virtue of) | `természeténél fogva` | - |
| sublative `-rA` | `nézve` (regarding), `vonatkozóan`, `tekintettel` | `Pálra nézve` | `rám nézve` |
| accusative `-t` | `kivéve` (except), `beleértve` (including), `leszámítva`, `tekintve` | `Pétert kivéve`, `az adót beleértve` | `engem kivéve` |
| dative `-nAk` | `köszönhetően` (thanks to), `ellenére` (anaphoric) | `a szerencsének köszönhetően` | `neked köszönhetően` |

- **Examples**: see table.
- **Exceptions**: `kivéve` also precedes a nominative complement (`kivéve Péter`); `fogva` has two governed cases with distinct meanings.
- **Predictability**: `lexical` (closed, about 30)
- **Lexical input**: governed case per postposition
- **Interactions**: `PND-PERS-003`, `PND-DEM-006` (case repetition on demonstrative)
- **Sources**: [KVF98 pp. 86-91, 340-341], [SMNy3 p. 613], [ATT `kívül` 21, `belül` 22, `képest` 19, `együtt` 22, `szemben` 21, `fogva` 17, `kezdve` 17, `nézve` 17]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-POSTP-005 Personal-suffixed postpositions
- **Statement**: A pronominal complement of a licensed nominative-governing postposition is expressed by a personal suffix on the postposition (`PND-PERS-004` allomorphs); the pronoun itself is omitted unless emphatic (`PND-POSTP-008`).
- **Conditions**: postposition with personal-form licence (`PND-POSTP-002`, `PND-POSTP-003`); 1st/2nd person or 3rd person pronoun complement.
- **Output**: reference paradigms:

| Postposition | 1sg | 2sg | 3sg | 1pl | 2pl | 3pl |
|---|---|---|---|---|---|---|
| `alatt` | `alattam` | `alattad` | `alatta` | `alattunk` | `alattatok` | `alattuk` |
| `alá` | `alám` | `alád` | `alá` | `alánk` | `alátok` | `alájuk` |
| `alól` | `alólam` | `alólad` | `alóla` | `alólunk` | `alólatok` | `alóluk` |
| `fölött` ~ `felett` | `fölöttem` | `fölötted` | `fölötte` | `fölöttünk` | `fölöttetek` | `fölöttük` |
| `fölé` | `fölém` | `föléd` | `fölé` | `fölénk` | `fölétek` | `föléjük` |
| `előtt` | `előttem` | `előtted` | `előtte` | `előttünk` | `előttetek` | `előttük` |
| `elé` | `elém` | `eléd` | `elé` | `elénk` | `elétek` | `eléjük` |
| `elől` | `előlem` | `előled` | `előle` | `előlünk` | `előletek` | `előlük` |
| `mögött` | `mögöttem` | `mögötted` | `mögötte` | `mögöttünk` | `mögöttetek` | `mögöttük` |
| `mögé` | `mögém` | `mögéd` | `mögé` | `mögénk` | `mögétek` | `mögéjük` |
| `mögül` | `mögülem` | `mögüled` | `mögüle` | `mögülünk` | `mögületek` | `mögülük` |
| `mellett` | `mellettem` | `melletted` | `mellette` | `mellettünk` | `mellettetek` | `mellettük` |
| `mellé` | `mellém` | `melléd` | `mellé` | `mellénk` | `mellétek` | `melléjük` |
| `mellől` | `mellőlem` | `mellőled` | `mellőle` | `mellőlünk` | `mellőletek` | `mellőlük` |
| `között` | `köztem` ~ `közöttem` | `közted` ~ `közötted` | `közte` ~ `közötte` | `köztünk` ~ `közöttünk` | `köztetek` ~ `közöttetek` | `köztük` ~ `közöttük` |
| `közé` | `közém` | `közéd` | `közé` | `közénk` | `közétek` | `közéjük` |
| `közül` | `közülem` | `közüled` | `közüle` | `közülünk` | `közületek` | `közülük` |
| `körül` | `körülöttem` | `körülötted` | `körülötte` | `körülöttünk` | `körülöttetek` | `körülöttük` |
| `köré` | `körém` | `köréd` | `köré` | `körénk` | `körétek` | `köréjük` |
| `felé` | `felém` | `feléd` | `felé` | `felénk` | `felétek` | `feléjük` |
| `felől` | `felőlem` | `felőled` | `felőle` | `felőlünk` | `felőletek` | `felőlük` |
| `után` | `utánam` | `utánad` | `utána` | `utánunk` | `utánatok` | `utánuk` |
| `szerint` | `szerintem` | `szerinted` | `szerinte` | `szerintünk` | `szerintetek` | `szerintük` |
| `miatt` | `miattam` | `miattad` | `miatta` | `miattunk` | `miattatok` | `miattuk` |
| `helyett` | `helyettem` | `helyetted` | `helyette` | `helyettünk` | `helyettetek` | `helyettük` |
| `nélkül` | `nélkülem` | `nélküled` | `nélküle` | `nélkülünk` | `nélkületek` | `nélkülük` |
| `iránt` | `irántam` | `irántad` | `iránta` | `irántunk` | `irántatok` | `irántuk` |
| `ellen` | `ellenem` | `ellened` | `ellene` | `ellenünk` | `ellenetek` | `ellenük` |
| `által` | `általam` | `általad` | `általa` | `általunk` | `általatok` | `általuk` |

  - `körül` uses the stem `körülött-` [ATT `körülöttem` 8, `körülem` -]; `között`: short stem `közt-` preferred (`köztük` [ATT 18] vs `közöttük` [ATT 14], about 4:1; `köztem` [ATT 5] vs `közöttem` [ATT 0]).
  - `közül` singular persons are semantically restricted (`közülünk`, `közületek`, `közülük` normal).
  - 3sg after `-é`/`-á`: zero (`elé`, `alá`); colloquial `-je`/`-ja` variants (`eléje`, `feléje`, `alája`) per `PND-PERS-004`.
- **Examples**: `Senki nem ül mellettem.`, `Utánad megyek.`, `Szerintem igaza van.`, `Miattuk késtünk.`, `Nélküled nem megyek.`
- **Exceptions**: no personal forms: `óta`, `múlva`, `közben`, `során`, `hosszat`, `végett`, `gyanánt`, `folytán`, `mentén`, `táján`, and all oblique-governing postpositions (`*együttem`, use `velem együtt`).
- **Predictability**: `rule` (suffixes) + `lexical` (licence list, stem `körülött-`)
- **Lexical input**: personal-form licence; alternate stems
- **Interactions**: `PND-PERS-004`, `PND-POSTP-006`, `ORT-LEN` (single `l`: `előlem`, `alólad`, `mellőlünk`) [AkH12 §72]
- **Sources**: [KVF98 pp. 88-90, 262-263], [SMNy3 pp. 612-613], [morphdb PP_*], [AkH12 §72], [ATT]
- **Validation**: goldens (full table); attestation
- **Confidence**: `consensus`

### PND-POSTP-006 nélkül with pronouns; other doublets
- **Statement**: `nélkül` with a pronoun has two standard constructions: the personal-suffixed postposition and the adessive pronoun + bare `nélkül`.
- **Conditions**: `nélkül` + personal pronoun.
- **Output**: `nélkülem` ~ `nálam nélkül`, `nélküled` ~ `nálad nélkül`, `nélküle` ~ `nála nélkül`, `nélkülünk` [ATT 4] ~ `nálunk nélkül`, `nélkületek` ~ `nálatok nélkül`, `nélkülük` ~ `náluk nélkül`; nouns never take the adessive: `Péter nélkül`, `*Péternél nélkül`.
- **Examples**: `Nélküled nem megyek.` = `Nálad nélkül nem megyek.`
- **Exceptions**: none.
- **Predictability**: `lexical` (1 postposition)
- **Lexical input**: none
- **Interactions**: `PND-POSTP-005`
- **Sources**: [KVF98 p. 91], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-POSTP-007 Dative pronoun extraction and prefix-like postpositions
- **Statement**: Some personal-suffixed postpositions allow the pronoun to appear as a dative outside the postpositional phrase, and spatial `alá`, `elé`, `mellé`, `mögé`, `közé`, `köré`, `fölé` also serve as verbal prefixes with a dative complement.
- **Conditions**: postposition in preverbal or predicative position.
- **Output**: `Neked senki nincs fölötted.`; `Mellé állították Pétert Annának.` / `Annának Pétert állították melléje.` (3sg `-je` obligatory in this order, `*... állították mellé`); `*Neked Anna érkezett helyetted` (not all postpositions allow extraction).
- **Examples**: `A sín alá ütött.` = `Alá ütött a sínnek.`; `Neked senki nincs fölötted.`
- **Exceptions**: extraction is lexically restricted; the list is not established in the sources.
- **Predictability**: `lexical` (open question on the full list)
- **Lexical input**: extraction licence
- **Interactions**: `VRB-PREV`, `SYN`
- **Sources**: [KVF98 pp. 90-91], [SMNy3 pp. 406-407 (postpositions as verbal prefixes)]
- **Validation**: goldens (limited)
- **Confidence**: `consensus` (phenomenon); `contested` (membership)

### PND-POSTP-008 Emphatic pronoun + personal-suffixed postposition
- **Statement**: For emphasis the nominative pronoun precedes the personal-suffixed postposition and is written together with it; 3pl uses `ő`.
- **Conditions**: `PND-POSTP-005` form + emphasis.
- **Output**: `énmiattam`, `teáltalad`, `őutána`, `mimellettünk`, `tinélkületek`, `őközöttük` (AkH examples); the pattern extends to every licensed postposition (`énszerintem`, `őmiatta`; unattested in the word list, spoken-style).
- **Examples**: `Őutána jöttem.`, `Énmiattam ne aggódj.`
- **Exceptions**: formal `ön`, `maga` do not form such words (`ön mellett`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-PERS-007`, `ORT-TOG-031`
- **Sources**: [AkH12 §122], [KVF98 p. 90 (te-fölötted)]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-POSTP-009 Transitional possessive-type postpositions
- **Statement**: Postposition-like possessed nouns in a case (`számára`, `részére`, `létére`, `nevében`, `helyén`, `alapján`, `következtében`, `nyomán`, `révén`, `ellenére`, `dacára`) take a possessive suffix before the case suffix, so pronominal forms are regular possessed nouns and nominal complements are possessors.
- **Conditions**: possessed-noun + case lexicalised as a relator.
- **Output**: `számomra`, `számodra`, `számára`, `számunkra`, `számotokra`, `számukra` [ATT `számomra` 17]; `részemre`, `részedre`, `részére`, `részünkre`, `részetekre`, `részükre` [ATT `részemre` 7, `részükre` 8]; `nevemben`, `nevében`; `létemre`, `gyerek létére`; `akaratom ellenére`; demonstrative complement in the dative (`annak alapján`, `ennek következtében`) or nominative (`ez alapján`).
- **Examples**: `Számomra ez fontos.`, `Részére küldtük.`, `Gyerek létére okos.`
- **Exceptions**: `ez alapján` vs `ennek alapján`: both in use; the prescriptive handbook tradition prefers `ennek alapján` without condemning `ez alapján` [ENY-alapjan].
- **Predictability**: `lexical` (open, grammaticalising class)
- **Lexical input**: relator lexicon
- **Interactions**: `NOM-POSS`, `NOM-CASE-015`
- **Sources**: [KVF98 p. 88 (szám-od-ra)], [SMNy3 p. 613 (third class: argument as possessor)], [ENY-alapjan], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`; `ez alapján` `contested`

### PND-POSTP-010 Postpositional adjectives with -i
- **Statement**: Many postpositions take the adjectival suffix `-i`, producing an attribute that is written apart from the complement like the postposition; directional (`hová?`) postpositions, postpositions containing a recognisable case suffix and some others do not.
- **Conditions**: postposition phrase used as a noun modifier.
- **Output**:
  - accept `-i`: `alatti`, `fölötti` ~ `feletti`, `előtti`, `mögötti`, `melletti`, `közötti` ~ `közti`, `körüli` [ATT 16], `utáni`, `szerinti`, `iránti`, `elleni`, `nélküli` [ATT 19], `miatti` [ATT 14], `általi` [ATT 14], `alóli` [ATT 10], `felőli` [ATT 9], `helyetti` [WC2 3281], `előli` [WC2 2597], `közüli` [WC2 1240], `mögüli` [WC2 393], `mellőli` [WC2 236] - all four `?` marks dropped, `mellőli` added; `közbeni` [ATT 10], `szembeni` [ATT 15], `kívüli` [ATT 17], `belüli` [ATT 17], `túli` [ATT 15], `felüli` [ATT 11], `aluli` [ATT 10]; also `ezelőtti` [WC2 161,844], `keresztüli` [WC2 35,746], `szemközti` [WC2 32,646], `inneni` [WC2 3,345], `körülötti` [WC2 1,095], `fölüli` [WC2 564], which the project list missed; truncating: `mentén` -> `menti` [ATT 14] (`Duna menti`), whose `-i` is not written on the postposition and is therefore not licensed with the others;
  - reject the postposition + `-i` reading of: `*alái`, `*elei`, `*mögéi`, `*mellei`, `*közéi`, `*feléi`, `*ótai`, `*múlvai`, `*áti`, `*fogvai`, `*kezdvei`, `*együtti`, `*végetti`, `*gyanánti`. The strings are at noise level against six-figure base frequencies (`múlvai` 141, `együtti` 135, `végetti` 115, `alái` 65, `áti` 58, `közéi` 40, `ótai` 32, `gyanánti` 12 [WC2]) or are words under another analysis: `mellei` 10,484 is the possessed plural of `mell`, `elei` 1,114 that of `elő` 'ancestor' [WC2]. The lative (`hová?`) restriction is exceptionless.
  - `képesti` [WC2 5,854] (`tavalyhoz képesti`) is attested like `általi` and accepted with it; it is criticised stylistically on the same grounds.
  - Writing: apart from the complement (`föld alatti`, `ház melletti`, `szó szerinti`, `idő előtti`, `munka nélküli jövedelem`), solid only when lexicalised (`földalatti` 'metro', `munkanélküli` noun, `délutáni`) [AkH12 §108].
- **Examples**: `a háború utáni évek`, `a Tiszán túli részek`, `a vele szembeni elvárás`.
- **Exceptions**: `honnan?` forms accept `-i` and are **all attested**; they are 2-4 orders of magnitude rarer than the `hol?` forms (`szerinti` 2,047,068, `közötti` 1,849,441, `elleni` 1,176,812, `nélküli` 1,089,479 against `mögüli` 393, `mellőli` 236), which tracks the base postposition frequency and is not an acceptability fact about `-i`. `általi` and `szembeni` are criticised stylistically but widely used (212,779 and 264,248).
- **Predictability**: `rule` (directional `hová?` forms never accept; all other postpositions accept) - the lative/non-lative split is exceptionless over N = 31 accepting plus 14 rejecting; `képesti`, whose base holds the old case ending `-st`, is the one case-containing form that takes `-i`
- **Lexical input**: `-i` licence per postposition, as a closed inventory of 36 accepting items
- **Interactions**: `ORT-TOG-018`, `DER` (`-i`)
- **Sources**: [SMNy3 pp. 259-260, 541-542], [AkH12 §108], [ATT], [WC2], [ATT evidence/productivity-tagset.md 2026-09-16]
- **Validation**: goldens; attestation. Webcorpus 2: 31/31 non-lative postpositions attested with `-i` (threshold 50 tokens), 14 of the 15 lative and case-containing items at noise level or attested only under another analysis, and `képesti` at 5,854; Tolerance Principle N = 31, e = 0, theta = 9 -> **productive**. The 250k-type project list, on which the four `?` marks rested, is 6 orders of magnitude too small for these items.
- **Confidence**: `consensus` (`OQ-04-007` resolved)

### PND-POSTP-011 Postpositions after demonstrative and relative complements
- **Statement**: A postposition after a demonstrative noun phrase is repeated after the demonstrative (`PND-DEM-006`); before an anaphoric `hogy`-clause the demonstrative + postposition may be a compound (`PND-DEM-009`); interrogative/relative + postposition stay apart.
- **Conditions**: complement is `ez`/`az` + NP, bare anaphoric `az`, or `ki`/`mi`/`aki`/`ami`.
- **Output**: `az alatt a fa alatt`, `e mellett a ház mellett`; `azután, hogy`, `anélkül, hogy`, `ahelyett, hogy`; `mi után`, `ki mellett`, `ami miatt`; `azzal a fiúval együtt` (oblique case repeated).
- **Examples**: `A kulcs az alatt a könyv alatt van.`, `Azután, hogy megérkezett, elaludt.`, `Miután megérkezett, elaludt.`
- **Exceptions**: conjunctions `miután`, `mielőtt`, `mialatt`, `miközben` are solid.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-DEM-006`, `PND-DEM-009`, `PND-REL-002`, comma placement `PUN`
- **Sources**: [KVF98 pp. 88-89, 278], [AkH12 §121]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-POSTP-012 Spelling of postposition endings
- **Statement**: Postposition endings have fixed length spellings: `-ól/-ől` long, `-ul/-ül` short (except `túl`), final `-tt` after a vowel and `-t` after a consonant (except `át`, `hosszat`), and personal forms with single `l`/`n` except `benne`, `bennünk` series.
- **Conditions**: any postposition or its personal form.
- **Output**: `alól`, `elől`, `felől`, `mellől`; `alul`, `belül`, `felül` ~ `fölül`, `keresztül`, `közül`, `mögül`, `nélkül`, `körül`, but `túl`; `alatt`, `fölött`, `között`, `mögött`, `mellett`, `előtt`, `iránt`, `közt`, `szerint`, but `át`, `hosszat`; `belőlem`, `előled`, `nálad`, `tőlünk`, `utánunk`, `alólam`.
- **Examples**: see Output.
- **Exceptions**: `elöl` ('in front', adverb, short) vs `elől` ('from before', postposition, long).
- **Predictability**: `lexical` (closed)
- **Lexical input**: spelling table
- **Interactions**: `ORT-LEN`, `REP` (typical errors `alol`, `nélkűl`)
- **Sources**: [AkH12 §14 c) elöl/elől, §34, §35, §38, §72]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-POSTP-013 Coordination with postpositions
- **Statement**: With coordinated nominal complements the postposition may appear once after the last conjunct; with pronominal complements each conjunct takes its own personal-suffixed postposition or case form.
- **Conditions**: coordinated complements.
- **Output**: `Péter és Anna fölött` (~ `Péter fölött és Anna fölött`); `fölötted és Péter fölött` (not `*te és Péter fölött`); `veled és Péterrel`; postpositions coordinated over one complement: `Péter fölött és mögött`; `között` requires plural or coordinated complement: `Péter és Anna között`, `köztünk`.
- **Examples**: `a ház és a kert mögött`.
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-POSTP-001`, `SYN`
- **Sources**: [KVF98 pp. 86-87], [SMNy3 pp. 604-606]
- **Validation**: goldens
- **Confidence**: `consensus`

---

## CARD: cardinal numerals

### PND-CARD-001 Simple cardinal numerals
- **Statement**: The cardinal numeral lexicon consists of 0-10, the tens, `száz`, `ezer` and the long-scale powers of a million; everything else is composed (`PND-CARD-003`).
- **Conditions**: lexical inventory.
- **Output**:

| Value | Form | Value | Form |
|---|---|---|---|
| 0 | `nulla` (`zéró`) | 20 | `húsz` |
| 1 | `egy` | 30 | `harminc` |
| 2 | `kettő` / `két` (`PND-CARD-004`) | 40 | `negyven` |
| 3 | `három` | 50 | `ötven` |
| 4 | `négy` | 60 | `hatvan` |
| 5 | `öt` | 70 | `hetven` |
| 6 | `hat` | 80 | `nyolcvan` |
| 7 | `hét` | 90 | `kilencven` |
| 8 | `nyolc` | 100 | `száz` |
| 9 | `kilenc` | 1000 | `ezer` |
| 10 | `tíz` | 10^6 | `millió` |
| | | 10^9 | `milliárd` |
| | | 10^12 | `billió` |
| | | 10^15 | `billiárd` |
| | | 10^18 | `trillió` |

  - Long scale: `billió` = 10^12 (not English billion); `milliárd` = 10^9; `trilliárd` 10^21, `kvadrillió` 10^24 exist but are rare.
  - Bound stems for 11-19 and 21-29: `tizen-`, `huszon-` (`PND-CARD-002`).
- **Examples**: `hét nap`, `harminc év`, `ezer forint`, `két milliárd`.
- **Exceptions**: `zéró` is a stylistic variant of `nulla`.
- **Predictability**: `lexical` (closed, 26 forms)
- **Lexical input**: numeral lexicon with stem classes (`PND-NUMINF-001`)
- **Interactions**: `PND-CARD-003`, `PND-NUMINF-001`
- **Sources**: [KVF98 pp. 341-342], [CLDR-RBNF hu spellout-cardinal]
- **Validation**: goldens; oracle:CLDR-RBNF
- **Confidence**: `consensus`

### PND-CARD-002 Teens and twenties: tizen-, huszon-
- **Statement**: 11-19 and 21-29 use the bound stems `tizen-` and `huszon-` (short `i`, `u`) + unit; other tens combine directly.
- **Conditions**: tens digit 1 or 2 and nonzero unit.
- **Output**: `tizenegy`, `tizenkettő`, `tizenhárom`, `tizennégy`, `tizenöt`, `tizenhat`, `tizenhét`, `tizennyolc`, `tizenkilenc`; `huszonegy` ... `huszonkilenc`; `harmincegy`, `negyvenkettő`, `kilencvenkilenc`.
- **Examples**: `tizenöt perc`, `huszonnégy óra`.
- **Exceptions**: `tíz`, `húsz` alone keep the long vowel (`PND-NUMINF-004`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-CARD-003`, `PND-NUMINF-004`
- **Sources**: [KVF98 p. 341], [AkH12 §27 a)], [CLDR-RBNF hu]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-CARD-003 Composition of spelled-out cardinals
- **Statement**: A cardinal is spelled out from the most significant three-digit group down; each group is `[hundreds][tens][units]` followed by its scale word, zero groups and zero digits are silent, and all words are joined per `PND-CARD-006`.
- **Conditions**: integer `N` >= 0.
- **Output**: algorithm:
  1. `N = 0` -> `nulla`.
  2. Split into groups `g_k ... g_1 g_0` of three digits from the right; scale words: `g_0` none, `g_1` `ezer`, `g_2` `millió`, `g_3` `milliárd`, `g_4` `billió`, `g_5` `billiárd`, `g_6` `trillió`.
  3. Group `g` (1-999): hundreds `h`: `h=1` -> `száz`, `h>=2` -> `U'(h)` + `száz`; tens `t`: `t=1` -> `tíz` if unit 0 else `tizen`; `t=2` -> `húsz` if unit 0 else `huszon`; `t>=3` -> tens word; units `u`: unit word, with `kettő` in final position and `két` before a scale word (`PND-CARD-004`); `U'(2)` = `két`.
  4. Group value 1 before a scale word: `ezer` (no `egy`) for `g_1 = 1`; `egymillió`, `egymilliárd`, `egybillió` for higher scales (`PND-CARD-005`).
  5. Join per `PND-CARD-006`.
- **Examples**: `14` `tizennégy`; `896` `nyolcszázkilencvenhat`; `1772` `ezerhétszázhetvenkettő`; `2000` `kétezer`; `2001` `kétezer-egy`; `3016` `háromezer-tizenhat`; `47 563` `negyvenhétezer-ötszázhatvanhárom`; `53 000` `ötvenháromezer`; `1 200 000` `egymillió-kétszázezer`; `7 490 530` `hétmillió-négyszázkilencvenezer-ötszázharminc`; `62 000 000` `hatvankétmillió`; `2 000 500` `kétmillió-ötszáz`.
- **Exceptions**: `egyszáz`, `egyezer` in formal money writing (`PND-CARD-005`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-CARD-004`, `PND-CARD-005`, `PND-CARD-006`, `PND-ART-003`
- **Sources**: [AkH12 §291 a), §263 d)], [KVF98 pp. 341-342], [CLDR-RBNF hu] (lacks the hyphen rule), [num2words lang_HU] (study only)
- **Validation**: goldens; oracle:CLDR-RBNF (word sequence, ignoring hyphens and soft hyphens)
- **Confidence**: `normative`

### PND-CARD-004 két vs kettő
- **Statement**: `két` is the attributive form of 2 (before an expressed noun and inside compounds before a scale word or `-szer`, `-féle`, etc.); `kettő` is the independent form (counting, predicate, head omitted, final position of a standalone number).
- **Conditions**: the numeral 2 as a word or as the last element of a compound (`tizenkét`, `huszonkét`, `-kétszáz-`).
- **Output**:
  - attributive: `két ház`, `tizenkét ember`, `huszonkét forint`, `százharminckét oldal`, `kétezer-két forint`;
  - scale-internal: always `két`: `kétszáz`, `kétezer`, `kétmillió`, `kétszázkétezer`;
  - independent: `kettő`, `tizenkettő`, `huszonkettő`, `Hány? Kettő.`, `Azt a kettőt kérem.`, `fél kettő` (clock);
  - inflected numeral: `kettőt`, `kettővel`, `kettőnek`; derived: `kétszer`, `kétféle`, `kétéves`, but `kettes`, `ketten`, `ketted` on the stem `kett-` (`PND-NUMINF-003`), and the ordinal `-kettedik` only as the last member of a compound (`tizenkettedik`); alone the ordinal is `második` (`PND-ORD-001`).
  - Colloquial or clarifying `kettő` attributively (`kettő darab`, to avoid confusion with `hét`) [KVF98 p. 343]: accept, do not generate.
- **Examples**: see Output.
- **Exceptions**: `mindkét` / `mindkettő` follow the same split (`PND-INDF-005`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [KVF98 p. 343], [CLDR-RBNF hu spellout-cardinal-initial], [ATT `kétszer` 17, `kettőször` -]
- **Validation**: goldens; oracle:CLDR-RBNF
- **Confidence**: `consensus`

### PND-CARD-005 egy before száz, ezer, millió
- **Statement**: `száz` and `ezer` normally stand without `egy`; `millió`, `milliárd` and higher scale words take `egy` for the value 1; `egyszáz`, `egyezer` are used where unambiguity matters (money, legal texts); inside a larger numeral the hundreds digit takes `egy` about 2:1 (`ezeregyszáz` 1,304 : `ezerszáz` 694 [WC2]), while a non-1 hundreds digit never does (`ezerkilencszáz` 415 : `ezerkilencegyszáz` 0).
- **Conditions**: group value 1 before a scale word, or hundreds digit 1.
- **Output**: `száz` ~ `egyszáz`, `ezer` ~ `egyezer`, `százezer`, `egymillió`, `egymilliárd`; inside larger numbers: `egymillió-ezer` ~ `egymillió-egyezer` (1 001 000).
- **Examples**: `ezer forint`, `egyezer forint` (cheque), `egymillió lakos`; `millió ember` (idiomatic 'a million/millions of people').
- **Exceptions**: none.
- **Predictability**: `tendency`, now quantified for the hundreds digit; the `egymillió-ezer` ~ `egymillió-egyezer` contrast remains **insufficient evidence** (0 : 0 tokens)
- **Lexical input**: register flag (`financial`)
- **Interactions**: `PND-CARD-003`
- **Sources**: [KVF98 p. 342 (egymillió)], [CLDR-RBNF hu spellout-cardinal (1000: ezer; 10^6: egymillió); spellout-cardinal-verbose (egyszáz, egyezer)], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens; attestation (whole-word forms, Webcorpus 2)
- **Confidence**: `consensus`; `measured` for the hundreds digit (both forms current, `ezeregyszáz` preferred at 65%); inner `egyezer` after a scale word still `contested`, unattested either way (`OQ-04-003` partly resolved)

### PND-CARD-006 Writing spelled-out numerals together or with hyphens
- **Statement**: Spelled-out cardinals and ordinals up to 2000 are one word; above 2000 only round thousands and round millions (no nonzero lower groups after the last scale word) are one word, otherwise hyphens separate the three-digit groups; years are always one word.
- **Conditions**: spelled-out cardinal or ordinal.
- **Output**:
  - `N <= 2000`: one word (`ezerkilencszázkilencvenkilenc`, `kétezer`, `ezeregyedik`);
  - `N > 2000`, group boundary after `ezer`, `millió`, `milliárd` ... followed by a nonzero remainder: hyphen after each scale word that is followed by further words (`kétezer-egy`, `tizenegyezer-huszonnyolc`, `hárommillió-ötszáznegyvenezer-kétszázharmincnyolc`, `négyezer-hatszázötödik`);
  - no hyphen when nothing follows the scale word (`ötvenháromezer`, `hatvankétmillió`, `hatmilliomodik`, `kétszázezer`);
  - year exception: `kétezertizennégy májusában` (year) vs `kétezer-tizennégy forint` (quantity).
- **Examples**: see Output.
- **Exceptions**: years: AkH exempts years from the hyphen rule and writes them solid (`kétezertizennégy`); the solid year spelling applies to every year above 2000 (`kétezeregy`, `kétezerhuszonnégyben`).
- **Predictability**: `rule`
- **Lexical input**: `year` flag
- **Interactions**: `ORT-NUM-010`, `ORT-HYPH-012`, `PUN-NUMF-011`, `PND-CARD-003`, `PND-DATE-007`
- **Sources**: [AkH12 §291 a), §263 d)], [KVF98 p. 341]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-CARD-007 Words vs digits
- **Statement**: There is no binding rule for writing numbers in words or digits; the practice is words for short numbers in continuous (especially literary) text and for inflected, compounded or postpositional numerals, digits for long numbers, dates, times, money, measures and statistics; round numbers above 999 may combine digits and a scale word.
- **Conditions**: generation style choice.
- **Output**: words: `öt`, `húsz`, `ezer`, `tízmillió`, `huszonötöt`, `ezernek`, `tízféle`, `hatvan után`, `harmadmagával`, `kéthetenként`; digits: `388 személy`, `65 tégla`, `du. 5 órakor`, `2890 Ft`, `15 méter`; mixed: `107 ezer`, `6 millió`.
- **Examples**: see Output.
- **Exceptions**: none.
- **Predictability**: `tendency` (no counts)
- **Lexical input**: style flag
- **Interactions**: `ORT-NUM-001`, `PUN-NUMF-001`
- **Sources**: [AkH12 §290]
- **Validation**: -
- **Confidence**: `normative` (recommendation)

### PND-CARD-008 Digit grouping and decimal comma (pointer)
- **Statement**: Numbers of five or more digits are grouped in threes from the right by a space (or period); four-digit numbers are grouped only in columns with longer numbers; the decimal separator is a comma without spaces.
- **Conditions**: digit numerals.
- **Output**: `20 611` ~ `20.611`, `5 602 164`; `2890`; `3,16`, `20 611,413`.
- **Examples**: see Output.
- **Exceptions**: years, postal codes, identifiers are not grouped (`1848`, `Pf. 3301`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: authoritative rules `ORT-NUM-005`, `PUN-NUMF-002`, `PUN-NUMF-003`; reading `PND-FRAC-005`
- **Sources**: [AkH12 §291 b), §293, §294]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-CARD-009 Numeral + derived word compounds
- **Statement**: A simple cardinal (or `sok`, `több`, `fél`) + a simple word with `-i`, `-Ú`, `-jÚ`, `-s`, `-nyi` is one word; if either member is itself a compound, the two are written apart; with digits always apart; `-féle`, `-fajta` solid after words, hyphenated after digits; `-nként`, `-nta/-nte` units follow the same pattern.
- **Conditions**: numeral + derived adjective/adverb.
- **Output**: `kétéves`, `ötméteres`, `harmincnapi`, `háromlábú`, `húszfőnyi`, `sokmilliós`; apart: `tizenöt méteres`, `öt négyzetméteres`, `huszonnégy tagú`, `száztizenhat napi`; digits: `2 éves`, `9 havi`, `40 köbcentiméteres`; `tízféle` vs `10-féle`; `kéthetenként`, `ötévente`, `tizenöt naponként`, `5 évente`; lexicalised: `ötszázforintos`, `tízezerforintos` (banknote).
- **Examples**: see Output.
- **Exceptions**: banknote and coin names solid regardless of structure (meaning change).
- **Predictability**: `rule`
- **Lexical input**: compound status of members (`DER-COMP`)
- **Interactions**: `ORT-TOG-023`, `ORT-TOG-021`, `ORT-HYPH-004`
- **Sources**: [AkH12 §115, §117, §132]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-CARD-010 Approximate, alternative and distributive numeral pairs
- **Statement**: Two numerals expressing approximation or alternative, and a repeated numeral with distributive meaning, are joined by a hyphen, in words and in digits.
- **Conditions**: `N-M` approximate (adjacent or near values), `N-N` distributive.
- **Output**: `egy-két ember`, `nyolc-tíz napra`, `5-6 darab`, `10-12 éves`; `két-két alma`, `húsz-húsz deka`, `2-2 almát`; range 'from-to' uses the en dash instead (`10–12. oldal`, `PND-DATE-010`).
- **Examples**: see Output.
- **Exceptions**: none.
- **Predictability**: `rule` (given the meaning)
- **Lexical input**: reading approximate vs range
- **Interactions**: `ORT-HYPH-010`, `ORT-HYPH-013`, `ORT-NUM-013`, `PUN-NUMF-013`
- **Sources**: [AkH12 §263 i), §264 c)]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-CARD-011 Numeral + singular noun; inflection only in the absence of the noun
- **Statement**: A cardinal or quantifier before a noun is undeclined and the noun stays singular; case and plural go on the numeral only when the noun is omitted; big round numerals may be pluralised as nouns.
- **Conditions**: numeral in a noun phrase.
- **Output**: `három ház`, `három házban`, `ezt a három könyvet`; head omitted: `Hármat kérek.`, `A hatból elvettem egyet.`; nominal plurals: `ezrek`, `százak`, `milliók` [ATT 9], `százezrek` ('thousands of people', 'large sums'); arithmetic: `hathoz hozzáadunk egyet`, `kétszer három`.
- **Examples**: `Ezrek nézték meg a kiállítást.`, `Milliókat keresett.`
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [KVF98 pp. 254, 342-343]
- **Validation**: goldens
- **Confidence**: `consensus`

---

## NUMINF: numeral inflection and suffixation

### PND-NUMINF-001 Numeral stem table
- **Statement**: Each basic numeral has fixed stem properties (harmony, lowering, epenthesis, shortening, special stems) from which all inflected and derived forms follow by the `NOM` rules; the table lists the properties and the key forms.
- **Conditions**: basic numeral as the last (or only) element of a numeral.
- **Output**:

| Numeral | Harm. | Class | Acc. | Instr. | Supess. | Coll. `-An` | `-szOr` | `-s` | Ordinal / fraction stem |
|---|---|---|---|---|---|---|---|---|---|
| `nulla` | B | a-final, lengthening | `nullát` | `nullával` | `nullán` | - | `nullaszor` | `nullás` | `nullad-` (`nulladik`) |
| `egy` | F | regular (front unrounded: lowering not observable) | `egyet` | `eggyel` | `egyen` | - (`egyedül`) | `egyszer` | `egyes` | `első` / `egyed-` in compounds |
| `kettő` / `két` / `kett-` | R | special (`PND-NUMINF-003`) | `kettőt` | `kettővel` | `kettőn` | `ketten` | `kétszer` | `kettes` | `második` / `-kettedik` in compounds, `ketted` |
| `három` | B | epenthetic + shortening (`hárm-`, `harm-`) | `hármat` | `hárommal` | `hármon` | `hárman` | `háromszor` | `hármas` | `harmad-` |
| `négy` | F | shortening only in ordinal/fraction (`negy-`) | `négyet` | `néggyel` | `négyen` | `négyen` | `négyszer` | `négyes` | `negyed-` |
| `öt` | R | non-lowering (linking `ö`) | `ötöt` | `öttel` | `ötön` | `öten` | `ötször` | `ötös` | `ötöd-` |
| `hat` | B | non-lowering (linking `o`) | `hatot` | `hattal` | `haton` | `hatan` | `hatszor` | `hatos` | `hatod-` |
| `hét` | F | shortening (`het-`) in accusative, collective, `-s`, ordinal, fraction | `hetet` | `héttel` | `héten` | `heten` | `hétszer` | `hetes` | `heted-` |
| `nyolc` | B | lowering | `nyolcat` | `nyolccal` | `nyolcon` | `nyolcan` | `nyolcszor` | `nyolcas` | `nyolcad-` |
| `kilenc` | F | lowering | `kilencet` | `kilenccel` | `kilencen` | `kilencen` | `kilencszer` | `kilences` | `kilenced-` |
| `tíz` | F | lowering; `tiz-` only in ordinal, fraction, `tizen-` | `tízet` | `tízzel` | `tízen` | `tízen` | `tízszer` | `tízes` | `tized-` |
| `húsz` | B | lowering; `husz-` only in ordinal, fraction, `huszon-` | `húszat` | `hússzal` | `húszon` | `húszan` | `hússzor` | `húszas` | `huszad-` |
| `harminc` | B | lowering | `harmincat` | `harminccal` | `harmincon` | `harmincan` | `harmincszor` | `harmincas` | `harmincad-` |
| `negyven` | F | lowering | `negyvenet` | `negyvennel` | `negyvenen` | `negyvenen` | `negyvenszer` | `negyvenes` | `negyvened-` |
| `ötven` | F | lowering | `ötvenet` | `ötvennel` | `ötvenen` | `ötvenen` | `ötvenszer` | `ötvenes` | `ötvened-` |
| `hatvan` | B | lowering | `hatvanat` | `hatvannal` | `hatvanon` | `hatvanan` | `hatvanszor` | `hatvanas` | `hatvanad-` |
| `hetven` | F | lowering | `hetvenet` | `hetvennel` | `hetvenen` | `hetvenen` | `hetvenszer` | `hetvenes` | `hetvened-` |
| `nyolcvan` | B | lowering | `nyolcvanat` | `nyolcvannal` | `nyolcvanon` | `nyolcvanan` | `nyolcvanszor` | `nyolcvanas` | `nyolcvanad-` |
| `kilencven` | F | lowering | `kilencvenet` | `kilencvennel` | `kilencvenen` | `kilencvenen` | `kilencvenszer` | `kilencvenes` | `kilencvened-` |
| `száz` | B | lowering | `százat` | `százzal` | `százon` | `százan` | `százszor` | `százas` | `század-` |
| `ezer` | F | epenthetic (`ezr-`), lowering | `ezret` | `ezerrel` | `ezren` | `ezren` | `ezerszer` | `ezres` | `ezred-` |
| `millió` | B | vowel-final | `milliót` | `millióval` | `millión` | `millióan` | `milliószor` | `milliós` | `milliomod-` |
| `milliárd` | B | non-lowering (`o`) | `milliárdot` | `milliárddal` | `milliárdon` | `milliárdan` | `milliárdszor` | `milliárdos` | `milliárdod-` |
| `billió` | B | vowel-final | `billiót` | `billióval` | `billión` | `billióan` | `billiószor` | `billiós` | `billiomod-` |

  - Other cases are regular on the free stem: dative `-nAk` (`háromnak`, `ezernek`), inessive (`ezerben`, `háromban`), allative with rounding (`kettőhöz`, `öthöz`, `ezerhez`), `-ig`, `-kor`, `-ért` invariant.
  - Harmony `R` selects `ö` in three-way suffixes (`ötöt`, `ötös`, `ötször`, `kettőhöz`, `ötödik`).
  - Epenthetic stems take the bound stem before the superessive: `hármon`, `ezren` (`NOM-STEM-005`); the free-stem forms are marginal (`hármon` 967 : `?háromon` 12, `ezren` 116,976 : `?ezeren` 753 [WC2]).
- **Examples**: `hármat kérek`, `hárman vagyunk`, `ezret fizettem`, `ezren voltak`, `a húszas évek`, `hússzor`, `eggyel több`, `ötször annyi`.
- **Exceptions**: `nulladik` exists; `nulla` has no collective (`nullán` is only the superessive); `tizet` [ATT 2] beside normative `tízet` [ATT 5], `tízedik` [ATT 5] beside normative `tizedik` [ATT 12] are nonstandard spellings (`REP`).
- **Predictability**: `lexical` (closed, 24 numerals; forms by rule from the properties)
- **Lexical input**: numeral stem properties (this table)
- **Interactions**: `NOM-STEM-005`, `NOM-STEM-007`, `NOM-LOW-005`, `NOM-ACC`, `NOM-CASE-003`, `NOM-CASE-011`, `PND-NUMINF-002`
- **Sources**: [AkH12 §27 a), §35, §69], [KVF98 pp. 341-345], [morphdb NUM, AFF_D, AFF_AN_NUM, GUESS_NUM], [CLDR-RBNF hu ordinals], [ATT `ezret` 10, `hármat` 11, `hetet` 12, `eggyel` 10, `ezrek` 8]
- **Validation**: goldens (full table); attestation
- **Confidence**: `consensus`; `tízet`, `hússzor` `normative`

### PND-NUMINF-002 Compound numerals inflect on the last element
- **Statement**: A composed numeral inflects and derives only on its last word, with that word's stem properties; the rest of the compound is inert.
- **Conditions**: composed numeral (`PND-CARD-003`).
- **Output**: last element + suffix per `PND-NUMINF-001`; harmony from the last element.
- **Examples**: `huszonhármat`, `tizenkettőt`, `tizenketten`, `kétszázat`, `kétezret`, `negyvenhétezer-ötszázhatvanhármat`, `ezerkilencszázötvenhatban`, `tizenkétszer`, `huszonötös`, `százhúszas`, `hatvanezren`.
- **Exceptions**: in `két`/`kettő` compounds the final-2 rule applies (`tizenkét` attributive, `tizenkettő` independent, `tizenkétszer`, `tizenkettes`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-NUMINF-001`, `PND-NUMINF-010` (digits use the same last-word rule)
- **Sources**: [KVF98 pp. 343-344], [AkH12 §290 (huszonötöt)]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-NUMINF-003 Stems of 2: kettő, két, kett-
- **Statement**: The numeral 2 has three stems selected by the following morpheme: `két` before an expressed noun, scale words, `-szOr`, `-féle`, `-fajta` and in compounds with a following word; `kett-` before vowel-initial derivational suffixes (`-es`, `-en`, `-ed`, `-edik`, `-ecske`); `kettő` elsewhere (independent use, case suffixes, possessive suffixes).
- **Conditions**: 2 as a word or last element.
- **Output**: `két` (`kétszer`, `kétféle`, `kétéves`, `kétszáz`); `kett-` (`kettes`, `ketten`, `ketted`, `tizenkettedik`, `kettesben` [ATT 8], `kettesével` [ATT 5]); `kettő` (`kettőt`, `kettőnek`, `kettővel`, `kettőnk`, `kettőtök`, `kettőjük` ~ `kettejük`).
- **Examples**: see Output.
- **Exceptions**: 3pl possessive `kettejük` (stem `kett-` + `-ejük`) preferred to `kettőjük` (`PND-PERS-011`); ordinal standalone is suppletive `második`.
- **Predictability**: `rule` (by suffix class)
- **Lexical input**: suffix class
- **Interactions**: `PND-CARD-004`, `PND-ORD-001`
- **Sources**: [KVF98 pp. 343-345], [CLDR-RBNF hu], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-NUMINF-004 tíz, húsz: long vowel except in ordinal, fraction and bound stems
- **Statement**: `tíz` and `húsz` keep the long vowel in all inflected and most derived forms and shorten only in the ordinal, the fraction and the bound compound stems.
- **Conditions**: suffix or compound after `tíz`, `húsz`.
- **Output**: long: `tízet`, `tízen`, `tízes`, `tízszer`, `tízzel`, `húszat`, `húszan`, `húszas`, `hússzor`, `hússzal`; short: `tizedik`, `tized`, `tizen-`, `huszadik`, `huszad`, `huszon-`.
- **Examples**: `tízszer`, `a tizedik`, `egy tized`, `tizenöt`, `huszonegy`, `hússzor`.
- **Exceptions**: `hússzor`: `húsz` + `szor` written `ssz` (doubled digraph simplification).
- **Predictability**: `lexical` (2 numerals)
- **Lexical input**: shortening restricted to ordinal/fraction
- **Interactions**: `NOM-STEM-009`, `ORT-ASSIM-016`
- **Sources**: [AkH12 §27 a), §35 (hússzor)]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-NUMINF-005 Collective -an/-en
- **Statement**: The modal-essive `-An` on numerals and quantity words gives the number of persons taking part ('as N people'); it is used for human subjects and has only the `a`/`e` allomorphs (no `o`/`ö`).
- **Conditions**: numeral or quantifier (`sok`, `kevés`, `néhány`, `pár`, `több`, `hány`, `ennyi`, `annyi`, `mennyi`, `mind`); referent human (or treated as such).
- **Output**: consonant-final stem + `-an` (B) / `-en` (F, R): `hárman`, `öten`, `ketten`, `ezren`, `sokan`, `kevesen`, `többen`, `néhányan`, `hányan`; vowel-final: `-an`/`-en` after the vowel without lengthening: `millióan`, `ennyien`, `annyian`, `mennyien`.
- **Examples**: `Hárman jöttek.`, `Ketten vagyunk.`, `Sokezren nézték meg.`, `Mind a hatan eljöttek.`
- **Exceptions**: `egy` has no collective (`egyedül`); `nulla` has none either (`nullán` is only the superessive).
- **Predictability**: `rule`
- **Lexical input**: numeral stem (`PND-NUMINF-001`)
- **Interactions**: -
- **Sources**: [KVF98 pp. 190, 265-266], [morphdb AFF_AN_NUM], [ATT `hárman` 12, `öten` 9, `ezren` 12, `millióan` 10]
- **Validation**: goldens; attestation
- **Confidence**: `consensus`

### PND-NUMINF-006 Multiplicative -szor/-szer/-ször and derivatives
- **Statement**: The multiplicative `-szOr` (always short vowel) forms 'N times'; it takes `-i` (occasion adjective) and `-Os` ('N-fold') derivatives.
- **Conditions**: cardinal, quantifier, `hány`, `ennyi`/`annyi`.
- **Output**: `egyszer`, `kétszer`, `háromszor`, `négyszer`, `ötször`, `hatszor`, `hétszer`, `nyolcszor`, `kilencszer`, `tízszer`, `hússzor`, `százszor`, `ezerszer`, `milliószor`, `sokszor`, `hányszor`; `-i`: `háromszori csengetés`, `kétszeri`; `-Os`: `kétszeres`, `háromszoros`, `négyszeres`, `ötszörös`, `tízszeres` [ATT `kétszeres` 12, `ötszörös` 8].
- **Examples**: `Háromszor csengettem.`, `kétszer három`, `háromszoros bajnok`.
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-LEN` (short vowel), `PND-ORD-005` (ordinal `-Odszor`)
- **Sources**: [AkH12 §35], [KVF98 pp. 345], [ATT]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-NUMINF-007 -s adjectives and number names
- **Statement**: `-(O)s` on a numeral forms the name of the digit/number, a denomination (coin, note, bus line, grade, size, year of make) and an adjective ('of N'), with the lowering linking vowel from the stem table.
- **Conditions**: cardinal (or digit, `PND-NUMINF-010`).
- **Output**: `egyes`, `kettes`, `hármas`, `négyes`, `ötös`, `hatos`, `hetes`, `nyolcas`, `kilences`, `tízes`, `húszas`, `harmincas`, `negyvenes`, `ötvenes`, `hatvanas`, `hetvenes`, `nyolcvanas`, `kilencvenes`, `százas`, `ezres`, `milliós`, `milliárdos`, `nullás`; `a hetes busz`, `egy tízes`, `a hatvanas évek`, `egy 1987-es Honda`.
- **Examples**: `Ötöst kapott.`, `Nincs egy tízesed?`
- **Exceptions**: `kétes` is a separate adjective 'doubtful' (not the number name).
- **Predictability**: `rule`
- **Lexical input**: stem table
- **Interactions**: `NOM-LOW`, `PND-DATE-009`
- **Sources**: [KVF98 pp. 344-345]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-NUMINF-008 Distributive forms
- **Statement**: Distribution 'N at a time' is expressed by `-(O)s` + possessive + instrumental (`-sával/-sével`), by `-(j)A` + instrumental on round numbers ('by the hundreds'), by `egyenként`, or by hyphenated repetition (`PND-CARD-010`).
- **Conditions**: distributive meaning.
- **Output**: `egyesével` [ATT 9], `kettesével`, `hármasával` [ATT 3], `négyesével`, `ötösével`, `tízesével`; `százával` [ATT 5], `ezrével` [ATT 5], `tucatjával`; `egyenként` [ATT 13]; `két-két`, `2-2`.
- **Examples**: `Kettesével álljatok sorba!`, `Ezrével érkeztek a levelek.`
- **Exceptions**: `-nként` on other numerals (`?kettenként`, `?hármanként`) is unattested; generate `-sával/-sével` forms instead.
- **Predictability**: `rule` (form); `lexical` choice between `-sával` and `-jával` (round numerals and measure nouns take `-jával`)
- **Lexical input**: none
- **Interactions**: `NOM-CASE-018`, `PND-CARD-010`
- **Sources**: [ATT]; [KVF98 p. 229 (partitive/distributive expressions)]
- **Validation**: attestation
- **Confidence**: `consensus`

### PND-NUMINF-009 Possessive forms of numerals
- **Statement**: Numerals take possessive suffixes in two uses: partitive with personal reference ('the N of us') and dates (on the ordinal) (`PND-DATE-003`); the stem is the one selected for vowel-initial suffixes in `PND-NUMINF-001` (for 2: `kettő-` except 3pl `kettejük` ~ `kettőjük`, `PND-NUMINF-003`).
- **Conditions**: numeral + possessive suffix.
- **Output**: partitive: `kettőnk`, `kettőtök`, `kettejük` ~ `kettőjük`, `hármunk`, `hármatok`, `hármuk` [ATT 1], `mindhármunk`, `négyünk`, `tízük`, `húszuk`, `húszunk`, `húszuknak` - `húsz` keeps the **long** vowel throughout the possessive paradigm (`PND-NUMINF-004`; the wider `[/Num][Poss]` paradigm is 107 : 0 long, and `huszuknak` has 0 attestations) [WC2]; day: `elseje`, `ötödike`. Number names in `-(O)s` possess as nouns (`az ötöse` 'his grade five', `PND-NUMINF-007`).
- **Examples**: `A fiú kettőnket várja.`, `Húszuknak küldtünk meghívót.`
- **Exceptions**: forms other than those of 2 are rare [ATT `hármuk` 1; `hármunk`, `négyünk`, `húszuk` -]; generate only on demand. The `húsz` vowel length is settled in favour of AkH12 and UniMorph against KVF98 p. 266: the exact `-uk` cell is at the reliability floor (long 15 : short 3, n = 18) but the wider possessive paradigm is categorical (`húszán` 44, `húsza` 29, `húszával` 18, `húszunkat` 16, short forms 0), and KVF98's `huszuknak` has 0 attestations [WC2]. The `-Ad` fraction derivative (`huszad-`) shortens by a separate rule (`PND-NUMINF-004`) and is excluded from that count.
- **Predictability**: `rule`
- **Lexical input**: stem table
- **Interactions**: `NOM-POSS`, `PND-PERS-011`, `PND-DATE-003`, `PND-NUMINF-004`
- **Sources**: [KVF98 pp. 265-266, 344], [ATT], [AkH12 §27 a)], [UM-hun], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens; attestation (all `[/Num][Poss...]` forms of lemma `húsz`)
- **Confidence**: `consensus`; rare forms `tendency`; `measured` for the long `húsz-` possessive stem (`OQ-04-012` resolved)

### PND-NUMINF-010 Suffixes on digits: hyphen and last spoken word
- **Statement**: A suffix on a number written in digits is attached with a hyphen and has the form it has in the spoken inflected numeral; allomorphy (harmony, linking vowel, lowering, epenthesis, `v`-assimilation) is determined by the last spoken word of the number.
- **Conditions**: integer in digits (optionally with grouping) + inflectional or derivational suffix.
- **Output**:
  - Last spoken word `L` of integer `N`:
    1. `N = 0` -> `nulla`.
    2. Let `k` be the number of trailing all-zero three-digit groups; if `N mod 1000 != 0` then `k = 0`.
    3. If `k = 0`: last group `g = N mod 1000`; units digit `u != 0` -> unit word (`egy`, `kettő`, `három` ... `kilenc`); else tens digit `t != 0` -> tens word (`tíz`, `húsz`, `harminc` ... `kilencven`); else `száz`.
    4. If `k >= 1`: `L` = scale word of group `k` (`ezer`, `millió`, `milliárd`, `billió` ...).
  - Written suffix = spoken form of `L` + suffix, minus the spoken form of `L` (`PND-NUMINF-011` table).
  - The period of an ordinal stays before the hyphen (`PND-ORD-003`); years and days: no period (`PND-DATE-005`).
- **Examples**: `4-et`, `12-t`, `19-szer`, `67-es`, `32-en` (collective), `32-n` (superessive), `4-gyel`, `5-tel`, `6-tal`, `100-zal`, `1000-rel`, `2000-ben`, `1848-ban`, `1900-ban`, `2020-ban`, `2010-ben`.
- **Exceptions**: large round numbers are normally written with a word scale (`3 millió`, `3 milliót`) [AkH12 §290]; the digit form with a suffix (`3 000 000-t`, last word `millió`) is correct but uncommon.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-NUM-002`, `ORT-HYPH-019`, `PUN-NUMF-021`, `PND-NUMINF-011`, `PND-ART-003`
- **Sources**: [AkH12 §82 f), §265 d), §298], [LP-HU numeral suffix tables] (study only), [morphdb GUESS_NUM] (digit guesser classes)
- **Validation**: goldens (exhaustive over every last word of `PND-NUMINF-011` and each suffix)
- **Confidence**: `normative`

### PND-NUMINF-011 Suffix table for digits by last spoken word
- **Statement**: The written suffix after the hyphen for each possible last spoken word is fixed; this table is the implementation lookup.
- **Conditions**: last spoken word from `PND-NUMINF-010`.
- **Output**:

| Last word (digits ending) | Acc. | Dat. | Instr./Transl. | Iness. | Supess. | Collective | Subl. | Allat. | `-s` | `-szOr` |
|---|---|---|---|---|---|---|---|---|---|---|
| `egy` (1) | `-et` | `-nek` | `-gyel` / `-gyé` | `-ben` | `-en` | - | `-re` | `-hez` | `-es` | `-szer` |
| `kettő` (2) | `-t` | `-nek` | `-vel` / `-vé` | `-ben` | `-n` | `-en` | `-re` | `-höz` | `-es` | `-szer` |
| `három` (3) | `-at` | `-nak` | `-mal` / `-má` | `-ban` | `-on` | `-an` | `-ra` | `-hoz` | `-as` | `-szor` |
| `négy` (4) | `-et` | `-nek` | `-gyel` / `-gyé` | `-ben` | `-en` | `-en` | `-re` | `-hez` | `-es` | `-szer` |
| `öt` (5) | `-öt` | `-nek` | `-tel` / `-té` | `-ben` | `-ön` | `-en` | `-re` | `-höz` | `-ös` | `-ször` |
| `hat` (6) | `-ot` | `-nak` | `-tal` / `-tá` | `-ban` | `-on` | `-an` | `-ra` | `-hoz` | `-os` | `-szor` |
| `hét` (7) | `-et` | `-nek` | `-tel` / `-té` | `-ben` | `-en` | `-en` | `-re` | `-hez` | `-es` | `-szer` |
| `nyolc` (8) | `-at` | `-nak` | `-cal` / `-cá` | `-ban` | `-on` | `-an` | `-ra` | `-hoz` | `-as` | `-szor` |
| `kilenc` (9) | `-et` | `-nek` | `-cel` / `-cé` | `-ben` | `-en` | `-en` | `-re` | `-hez` | `-es` | `-szer` |
| `tíz` (10) | `-et` | `-nek` | `-zel` / `-zé` | `-ben` | `-en` | `-en` | `-re` | `-hez` | `-es` | `-szer` |
| `húsz` (20) | `-at` | `-nak` | `-szal` / `-szá` | `-ban` | `-on` | `-an` | `-ra` | `-hoz` | `-as` | `-szor` |
| `harminc` (30) | `-at` | `-nak` | `-cal` / `-cá` | `-ban` | `-on` | `-an` | `-ra` | `-hoz` | `-as` | `-szor` |
| `negyven` (40), `ötven` (50), `hetven` (70), `kilencven` (90) | `-et` | `-nek` | `-nel` / `-né` | `-ben` | `-en` | `-en` | `-re` | `-hez` | `-es` | `-szer` |
| `hatvan` (60), `nyolcvan` (80) | `-at` | `-nak` | `-nal` / `-ná` | `-ban` | `-on` | `-an` | `-ra` | `-hoz` | `-as` | `-szor` |
| `száz` (x00) | `-at` | `-nak` | `-zal` / `-zá` | `-ban` | `-on` | `-an` | `-ra` | `-hoz` | `-as` | `-szor` |
| `ezer` (x 000) | `-et` | `-nek` | `-rel` / `-ré` | `-ben` | `-en` | `-en` | `-re` | `-hez` | `-es` | `-szer` |
| `millió` (x 000 000) | `-t` | `-nak` | `-val` / `-vá` | `-ban` | `-n` | `-an` | `-ra` | `-hoz` | `-s` | `-szor` |
| `milliárd` (x 000 000 000) | `-ot` | `-nak` | `-dal` / `-dá` | `-ban` | `-on` | `-an` | `-ra` | `-hoz` | `-os` | `-szor` |
| `nulla` (0) | `-t` | `-nak` | `-val` / `-vá` | `-ban` | `-n` | - | `-ra` | `-hoz` | `-s` | `-szor` |

  - Invariant suffixes: `-kor`, `-ig`, `-ért`, `-ként`, `-i`: `5-kor`, `10-ig`, `100-ért`, `3-ként`.
  - Other harmonic suffixes follow the harmony column: `-bÓl`, `-tÓl`, `-rÓl` (`5-ből`, `6-ból`), `-nÁl` (`3-nál`, `4-nél`), `-Os` + further suffixes (`67-es`, `80%-os`).
  - `-szOrOs`: `2-szeres`, `3-szoros`, `5-szörös`; `-féle` after digits: `10-féle`.
  - Stem changes of the spoken word are not written after the hyphen: `0-t`, `0-val` (`nullát`, `nullával`), `1000-et` (`ezret`), like the unlengthened vowel of all-caps acronyms (`MTA-t`, `MTA-nak`) [AkH12 §29, §288 a)] (analogy; consensus).
- **Examples**: `4-gyel`, `5-tel`, `6-tal`, `8-cal`, `9-cel`, `10-zel`, `20-szal`, `30-cal`, `40-nel`, `60-nal`, `100-zal`, `1000-rel`; `3-má`, `6-tá`, `8-cá`, `20-szá`; `2-höz`, `5-höz`, `3-hoz`, `7-hez`; `1-et`, `2-t`, `3-at`, `5-öt`, `6-ot`, `7-et`, `8-at`, `20-at`, `40-et`, `100-at`, `1000-et`; `1-es`, `2-es`, `3-as`, `5-ös`, `6-os`, `20-as`, `1000-es`; `15-ben`, `1956-ban`, `2000-ben`.
- **Exceptions**: none (readings that change the last word, `PND-NUMINF-012`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-NUMINF-001`, `PND-NUMINF-010`, `ORT-NUM-002` (same table), `ORT-ASSIM-007`
- **Sources**: [AkH12 §82 f) (4-gyel), §265 d) (4-et, 12-t, 19-szer, 32-en, 32-n, 67-es)], [LP-HU tables] (study only; agree with this table), [morphdb GUESS_NUM]
- **Validation**: goldens
- **Confidence**: `normative` (principle and AkH examples); `consensus` (remaining cells, derived)

### PND-NUMINF-012 Collective vs superessive on digits; alternative readings
- **Statement**: The collective `-An` and superessive `-(O)n` are homographous after digits whose last word is front unrounded consonant-final and are distinct otherwise; where a digit string admits more than one reading, each reading gives its own suffix.
- **Conditions**: `-An` vs `-(O)n`; ambiguous readings.
- **Output**:
  - distinct: last word `kettő` (`2-en` vs `2-n`), `három` (`3-an` vs `3-on`), `öt` (`5-en` vs `5-ön`), `hat`, `nyolc`, `húsz`, `harminc`, `hatvan`, `nyolcvan`, `száz` (`-an` vs `-on`), `millió` (`-an` vs `-n`);
  - identical `-en`: `egy` (superessive only), `négy`, `hét`, `kilenc`, `tíz`, `negyven`, `ötven`, `hetven`, `kilencven`, `ezer`;
  - alternative readings: telephone and code numbers read in digit groups (`06-1 ...`); house numbers read as cardinals (`4.-ben` `négyben`, `PND-ORD-003`); `1,5` read `egy egész öt tized` vs `másfél` (`PND-FRAC-007`).
- **Examples**: `32-en voltunk` (`harmincketten`) vs `a 32-n` (`harminckettőn`); `5-en` (`öten`) vs `5-ön` (`ötön`).
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: reading
- **Interactions**: `PND-NUMINF-005`, `PND-NUMINF-011`, `ORT-NUM-004`, `ORT-NUM-007`
- **Sources**: [AkH12 §265 d), §292 (house numbers)]
- **Validation**: goldens
- **Confidence**: `normative`

---

## ORD: ordinals

### PND-ORD-001 Ordinal suffix -Adik / -Odik
- **Statement**: Ordinals are formed with `-dik` after a linking vowel on the numeral's ordinal stem (`PND-NUMINF-001`): `a` after lowering back stems, `o` after non-lowering back stems, `e` after front unrounded, `ö` after front rounded; 1 and 2 are suppletive when standing alone but regular as the last element of a compound.
- **Conditions**: cardinal (simple or compound).
- **Output**: last element -> ordinal:

| Cardinal | Ordinal | Cardinal | Ordinal |
|---|---|---|---|
| `egy` | `első`; in compounds `-egyedik` | `húsz` | `huszadik` |
| `kettő` | `második`; in compounds `-kettedik` | `harminc` | `harmincadik` |
| `három` | `harmadik` | `negyven` | `negyvenedik` |
| `négy` | `negyedik` | `ötven` | `ötvenedik` |
| `öt` | `ötödik` | `hatvan` | `hatvanadik` |
| `hat` | `hatodik` | `hetven` | `hetvenedik` |
| `hét` | `hetedik` | `nyolcvan` | `nyolcvanadik` |
| `nyolc` | `nyolcadik` | `kilencven` | `kilencvenedik` |
| `kilenc` | `kilencedik` | `száz` | `századik` |
| `tíz` | `tizedik` | `ezer` | `ezredik` |
| `nulla` | `nulladik` | `millió` | `milliomodik` |
| | | `milliárd` | `milliárdodik` |
| | | `billió` | `billiomodik` |

- **Examples**: `tizenegyedik`, `tizenkettedik`, `huszonkettedik` [ATT 3], `harmincegyedik`, `százegyedik`, `százkettedik`, `kétszázadik`, `ezeregyedik`, `kétezredik`, `négyezer-hatszázötödik`, `hatmilliomodik`; also `hányadik`, `sokadik`, `utolsó` ('last'), `n-edik`, `x-edik`.
- **Exceptions**: `egyedik` and `kettedik` do not stand alone (`*egyedik`, `*kettedik`): `kettedik` 216 : `második` 3,174,469 and `egyedik` 2,897 : `első` over 10 million [WC2]; `millió` uses the stem `milliom-`.
- **Predictability**: `rule` with `lexical` suppletion (2 items) and stem table
- **Lexical input**: numeral ordinal stems
- **Interactions**: `PND-NUMINF-001`, `PND-CARD-006` (hyphens), `PND-ORD-002`
- **Sources**: [KVF98 pp. 343-344], [CLDR-RBNF hu spellout-ordinal], [AkH12 §291 a) (nyolcszázkilencvenhatodik, hatmilliomodik, négyezer-hatszázötödik)], [ATT]
- **Validation**: goldens; oracle:CLDR-RBNF
- **Confidence**: `consensus`

### PND-ORD-002 Inflection of ordinals
- **Statement**: Ordinals inflect as adjectives/nouns ending in `-ik` (harmony by the ordinal suffix vowel; `első` as a front rounded vowel-final stem); `első` has the irregular 3sg possessive `elseje`. The back-harmony plural varies between `-ikak` and `-ikok` with no clear default (`harmadikok` 3,200 : `harmadikak` 1,576, but `hatodikak` 610 : `hatodikok` 434 [WC2]); the front plural is always `-ikek` (`ötödikek` 1,635 : `ötödikök` 0).
- **Conditions**: ordinal used as noun or with case.
- **Output**: `harmadikat`, `harmadiknak`, `harmadikban`, `ötödiket`, `ötödikhez`, `századikat`; `elsőt`, `elsőnek`, `elsővel`, `elsőhöz`, `elsők`; `másodikat`, `másodikhoz`; possessive 3sg: `elseje`, `másodika`, `harmadika`, `negyedike`, `ötödike` (`PND-DATE-003`); plural `az elsők` [ATT 11], back ordinals `harmadikok` 3,200 ~ `harmadikak` 1,576, `hatodikak` 610 ~ `hatodikok` 434 (at pattern level `-ikok` 40,655 : `-ikak` 8,682, 82%, but per item the two are within a factor of 2), front `ötödikek` 1,635 : `ötödikök` 0, `negyedikek` [ATT 2] (pattern level `-ikek` 13,803 : `-ikök` 209).
- **Examples**: `A harmadikban ülök.`, `Elsőnek érkezett.`
- **Exceptions**: accusative of back ordinals has `a` (`másodikat` [ATT 9], `harmadikat` [ATT 8]; `-ot` near-unattested, `NOM-ADJ-009`); the back plural is genuinely item-split, so both `-ikak` and `-ikok` are generated with no preferred variant (`OQ-04-004` resolved; the earlier "ATT 0 vs 2" is superseded).
- **Predictability**: `rule`
- **Lexical input**: `elseje` irregular
- **Interactions**: `NOM-ACC`, `NOM-PL`, `PND-DATE-003`
- **Sources**: [KVF98 p. 344], [ATT `elseje` 7, `elsején` 11], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens; attestation (`[Pl]` per item and at pattern level)
- **Confidence**: `consensus`; `measured` for the plural linking vowel - both current and item-split in the back series, categorical `-ikek` in the front series

### PND-ORD-003 Ordinals in digits: period and suffixes
- **Statement**: An ordinal written in digits takes a period; a suffix is attached after the period with a hyphen in the form of the spoken ordinal; dates are excepted, and a house number after a street name keeps the period although it is read as a cardinal.
- **Conditions**: digit ordinal.
- **Output**:
  - `N.` + `-` + suffix of the spoken ordinal: last word ordinal harmony (`-adik`/`-odik`: back suffixes; `-edik`/`-ödik`: front unrounded suffixes, the final `i` blocks rounding; `első`: front rounded; `második`: back).
  - Ranges: period only after the second number (`a 10–12. oldalon`, `az 1–4. osztályban`).
  - House numbers: `Tutaj u. 4.-ben` (read `négyben`), but `4/B-ben`.
  - Dates: no period before the suffix (`PND-DATE-005`).
- **Examples**: `1. osztály`, `3. sor`, `1978. évi`, `3.-nak futott be` (`harmadiknak`), `a 8.-ba jár` (`nyolcadikba`), `a 10.-kel` (`tizedikkel`), `az 5.-hez` (`ötödikhez`), `az 1.-t` (`elsőt`), `az 1.-vel` (`elsővel`), `a 2.-at` (`másodikat`), `a 128–9.-en`.
- **Exceptions**: the house-number reading (`4.-ben` = `négyben`) is a lexicalised exception to the ordinal reading.
- **Predictability**: `rule`
- **Lexical input**: context: house number vs ordinal
- **Interactions**: `ORT-NUM-006`, `ORT-NUM-007`, `ORT-HYPH-013`, `PUN-NUMF-009`, `PND-ART-004`
- **Sources**: [AkH12 §292, §264 c)]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-ORD-004 Roman numerals
- **Statement**: Roman numerals are used only for ordinals, with a period; they are traditional for rulers, popes, districts, centuries (optional), months in numeric dates, volumes and some professional terms; suffixes follow the spoken ordinal with a hyphen after the period.
- **Conditions**: Roman numeral in text.
- **Output**:
  - Value: standard additive-subtractive reading (`I`=1, `V`=5, `X`=10, `L`=50, `C`=100, `D`=500, `M`=1000; smaller before larger subtracts); valid range 1-3999.
  - Uses: `IV. Béla`, `II. Rákóczi Ferenc`, `VII. Gergely`; `XIII. kerület` ~ `13. kerület`; `XVI. század` ~ `16. század`; `III. kötet` ~ `3. kötet`; `2013. IX. 12.`; `Petőfi u. 8. III. 2.` (floor); `IV/3-as tábla`; school classes use Arabic (`4. a`, `4/a`).
  - Suffix: `a IV.-be jár`, `az I–IV.-ben`, `a XX. században` (suffix on the noun).
  - Ruler names: the suffix goes on the name: `IV. Bélának`, `II. Rákóczi Ferenccel`.
- **Examples**: see Output.
- **Exceptions**: designations: `NB I.` (league; `ORT-HYPH-016`), `Forma–1` (en dash, no period); letter-digit codes (`B12-vitamin`, `M7-es autópálya`) are not ordinals.
- **Predictability**: `rule` (reading and suffix); `lexical` (usage domains)
- **Lexical input**: context (name, district, century)
- **Interactions**: `PND-ART-004`, `ORT-NUM-008`, `ORT-HYPH-016`, `PUN-NUMF-010`, `PND-DATE-009`
- **Sources**: [AkH12 §294, §265 d), §264 c)]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-ORD-005 Ordinal adverbs, -d compounds and ordinal + maga
- **Statement**: The ordinal stem without `-ik` forms the iterative ordinal adverb with `-szOr` ('for the Nth time'), attributive `-d` compounds written together, and `-d` + `maga` compounds.
- **Conditions**: ordinal stem (`harmad-`, `negyed-` ... ; `első` -> `első-`/`elő-`, `második` -> `másod-`).
- **Output**:
  - iterative: `először` [ATT 21], `másodszor` [ATT 13], `harmadszor`, `negyedszer`, `ötödször`, `hatodszor`, `hetedszer`, `nyolcadszor`, `kilencedszer`, `tizedszer`, `huszadszor`, `századszor`, `ezredszer`, `tizenkettedszer`, `hányadszor`, `sokadszor`; `-i`: `negyedszeri`; last: `utoljára`, `utolszor`;
  - `-d` compounds: `másodrendű`, `ötödéves`, `hetedízben`, `ezredévi`, `tizenötödrangú`, `harmadfél` (archaic 2.5);
  - with `maga`: `másodmagával`, `harmadmagával` (`PND-REFL-003`).
- **Examples**: `Negyedszer is csengettem.`, `a negyedszeri csengetés`, `másodéves hallgató`.
- **Exceptions**: `először` from `első` (not `*elsőszer`); `elsőéves` beside `másodéves`.
- **Predictability**: `rule` with 2 suppletive stems
- **Lexical input**: ordinal stems
- **Interactions**: `ORT-TOG-039`, `ORT-TOG-040`, `PND-NUMINF-006`
- **Sources**: [KVF98 p. 345], [AkH12 §130, §131], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`; writing `normative`

---

## FRAC: fractions, decimals, percentages

### PND-FRAC-001 Fraction denominators -Ad / -Od
- **Statement**: The denominator of a fraction is the ordinal without `-ik` (suffix `-d` after the ordinal linking vowel); the numerator is a cardinal; 2 as denominator is `ketted` only in arithmetic, otherwise `fél`.
- **Conditions**: fraction `a/b`.
- **Output**: `harmad`, `negyed`, `ötöd`, `hatod`, `heted`, `nyolcad`, `kilenced`, `tized`, `huszad`, `harmincad`, `negyvened`, `század`, `ezred`, `tízezred`, `százezred`, `milliomod`, `milliárdod`; compound: `tizenketted`, `huszonegyed`; `ketted` (arithmetic `öt ketted`); `egyed` is not a fraction word ('individual'); `hányad` [ATT 8].
  - Reading: numerator (attributive form, `két`) + denominator: `három nyolcad`, `hét kilenced`, `két harmad`, `egy tized`, `öt ketted`.
- **Examples**: `3/8` `három nyolcad`; `2/3` `két harmad`; `1/100` `egy század`.
- **Exceptions**: `1/2` `fél` / `egy ketted` (arithmetic); `1/4` `negyed`; `3/4` `három negyed` as a noun, `háromnegyed` as an attribute (`PND-FRAC-002`).
- **Predictability**: `rule`
- **Lexical input**: ordinal stems
- **Interactions**: `PND-ORD-001`, `PND-NUMINF-001`
- **Sources**: [KVF98 p. 344], [morphdb AFF_D]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-FRAC-002 Writing fraction phrases together or apart
- **Statement**: A fraction with a cardinal numerator is written as two words when it is a noun, and as one word when the whole fraction is an attribute; clock-time fractions follow the attributive pattern.
- **Conditions**: cardinal + fraction word.
- **Output**: noun: `egy negyed`, `két harmad`, `harmincnyolc egész egy tized`; attributive: `az ország kétharmad része`, `háromnegyed óra múlva`, `kétharmados többség`; clock: `negyed kilenckor`, `fél tizenkettőig`, `háromnegyed négyre`.
- **Examples**: see Output.
- **Exceptions**: none.
- **Predictability**: `rule` (given noun vs attribute)
- **Lexical input**: syntactic function
- **Interactions**: `ORT-NUM-015`, `PUN-NUMF-012`, `PND-DATE-011`
- **Sources**: [AkH12 §293]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-FRAC-003 Part-of forms: fele, harmada, negyede
- **Statement**: 'N-th of X' is the fraction word with a 3sg (or other) possessive suffix after the possessor: `fél` -> `fele`, `harmad` -> `harmada`, `negyed` -> `negyede`.
- **Conditions**: fraction of a quantity.
- **Output**: `fele` [ATT 16] (shortening `fél` -> `fel-`), `harmada` [ATT 10], `negyede` [ATT 9], `ötöde` [ATT 6], `hatoda`, `hetede`, `nyolcada`, `kilencede`, `tizede` [ATT 5], `százada`, `ezrede`; plural possessor `felük`, `harmaduk`; with numerals (noun-like, apart per AkH12 §293): `két harmada`, `három negyede`; `a fele`, `fele annyi`, `fele-fele arányban`; solid `kétharmada` [ATT 10], `egyharmada` [ATT 10] is widespread (lexicalised `kétharmad` 'two-thirds majority'): accept, do not generate.
- **Examples**: `a pénz harmada`, `a lakosság két harmada`, `az út fele`.
- **Exceptions**: `százada` is also 'his century / his company' (homograph).
- **Predictability**: `rule`
- **Lexical input**: `fél` shortening
- **Interactions**: `NOM-POSS-005`, `NOM-STEM-007`
- **Sources**: [ATT]; [KVF98 p. 228 (part-whole)]; [AkH12 §293]
- **Validation**: goldens; attestation
- **Confidence**: `consensus`; solid `kétharmada` `contested`

### PND-FRAC-004 fél, másfél, N és fél
- **Statement**: 'half' is `fél` as an attribute and unit (`fél kiló`), and halves above one are `másfél` (1.5) and `N és fél`; archaic ordinal-based `harmadfél` (2.5), `negyedfél` (3.5).
- **Conditions**: half quantities.
- **Output**: `fél`, `fél óra`, `másfél` [ATT 16], `másfél liter`, `két és fél`, `három és fél év`; archaic `harmadfél` [ATT 0]; inflection: `felet` (accusative, shortening), `félszer`, `másfélszer`, `két és félszer`, `másfélszeres`; derivatives `felez`, `feles`.
- **Examples**: `Másfél órát vártam.`, `két és fél kiló`.
- **Exceptions**: in clock time `fél` + next hour (`PND-DATE-011`).
- **Predictability**: `lexical` (closed)
- **Lexical input**: none
- **Interactions**: `PND-FRAC-007`, `PND-DATE-011`
- **Sources**: [AkH12 §130 (harmadfél)], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-FRAC-005 Decimal numbers: writing and reading
- **Statement**: Decimals use a comma without spaces; the standard reading is integer part + `egész` + decimal digits as a cardinal + denominator of the corresponding power of ten; colloquial readings drop the denominator or use `vessző`.
- **Conditions**: decimal number.
- **Output**:
  - denominators by number of decimals: 1 `tized`, 2 `század`, 3 `ezred`, 4 `tízezred`, 5 `százezred`, 6 `milliomod`;
  - `3,16` `három egész tizenhat század`; `37,5` `harminchét egész öt tized`; `0,25` `nulla egész huszonöt század`; `20 611,413` `húszezer-hatszáztizenegy egész négyszáztizenhárom ezred`;
  - colloquial: `három egész tizenhat`, `három vessző tizenhat`; `0,5` `fél`, `1,5` `másfél`, `2,5` `két és fél`.
- **Examples**: see Output.
- **Exceptions**: decimal point in software, sports times and some technical contexts (`3.16`) is nonstandard in Hungarian text (`PUN-NUMF-003`, `REP`).
- **Predictability**: `rule` (standard reading)
- **Lexical input**: register flag for reading
- **Interactions**: `PUN-NUMF-003`, `PND-FRAC-007`
- **Sources**: [AkH12 §293], [CLDR-RBNF hu (`egész`)]
- **Validation**: goldens
- **Confidence**: `normative` (comma); `consensus` (standard reading); `tendency` (colloquial)

### PND-FRAC-006 Percent and per mille
- **Statement**: `%` is read `százalék` (back harmony: vacillating stem realised back) and `‰` `ezrelék` (front); suffixes on the sign follow those words with a hyphen.
- **Conditions**: number + `%` / `‰`.
- **Output**: `15%` `tizenöt százalék`; `%-ot`, `%-nak`, `%-kal`, `%-ban`, `%-on`, `%-ra`, `%-os`, `%-a` ('N percent of'), `%-nyi`; `‰-et`, `‰-kel`, `‰-es`, `‰-ben`; spelled: `százalékos`, `ezrelékes`, `százalékpont`.
- **Examples**: `15%-kal nőtt`, `80%-os`, `a lakosság 30%-a`, `6%-kal`, `0,5‰-es véralkoholszint`.
- **Exceptions**: spacing between number and `%` is `PUN-NUMF-005` (no space).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-MEAS-001`, `PH-HARM` (vacillation `százalék` fixed back)
- **Sources**: [AkH12 §82 f) (15%-kal), §265 d) (80%-os, 6%-kal)]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-FRAC-007 Suffixes and article after decimals and fractions depend on the reading
- **Statement**: For decimals and fractions in digits the first and last spoken words depend on the reading, and the article and suffix follow the reading actually intended; no norm fixes the reading, so the implementation takes it as input with a documented default.
- **Conditions**: decimal or fraction in digits + suffix, or preceded by the article.
- **Output**:
  - readings of `N,d`: (R1) `N egész d DENOM` (standard, `PND-FRAC-005`): last word the denominator (`tized` F, `század` B, `ezred` F); (R2) `N egész d` (colloquial): last word from `d` as a cardinal (`PND-NUMINF-010`); (R3) for `,5` only: `N és fél` / `másfél` / `fél`: last word `fél` (F);
  - suffix by reading: `1,2` + instrumental: R1 `1,2-del` (`tizeddel`), R2 `1,2-vel` (`kettővel`); `2,5` + `-szOr`: R1 `2,5-szer` (`tizedszer`), R2 `2,5-ször` (`ötször`), R3 `2,5-szer` (`félszer`); `3,14` + inessive: R1 `3,14-ban` (`században`), R2 `3,14-ben` (`tizennégyben`);
  - article by first word: `az 1,5` (R1, R2 `egy`) ~ `a 1,5` (R3 `másfél`); `a 0,5` (both `nulla` and `fél`); `a ½` (`fél`); `az 5/8` (`öt`);
  - default for generation: avoid suffixing decimals where possible (use a noun: `1,2 százalékkal`, `2,5-szeres` with R3 for `,5`); otherwise R2, which the corpus confirms as the majority reading (66.5%).
  - measured split `[ATT]`: R1 and R2 are separable because no Hungarian digit name ends in `d`, so a `-del`/`-dal` suffix can only come from a denominator (`tizeddel`, `századdal`). R2 4,460 hits (`-tel`/`-tal` 3,036, `-vel`/`-val` 1,424) against R1 2,245. Within R2 the suffix tracks the final digit as predicted: 89.7% of `-vel`/`-val` follow a final `2` (`kettővel`), while `-del`/`-dal` is spread across all final digits, being digit-independent. Both spellings of `1,2-vel` ~ `1,2-del` are therefore correct and encode different readings; neither is an error.
- **Examples**: `2,5-szeres` (`két és félszeres`), `1,5-szer` (`másfélszer`), `az 1,5 literes`.
- **Exceptions**: signs after the number take over the last word (`1,2%-kal`, `3,5 km-rel`).
- **Predictability**: `rule` given the reading; `tendency` for which reading a writer picks
- **Lexical input**: reading (R1/R2/R3)
- **Interactions**: `PND-FRAC-005`, `PND-NUMINF-010`, `PND-ART-003`
- **Sources**: derived from the pronunciation principle [AkH12 §265 d), §276, §282]; no explicit norm found; [ATT evidence/norm-gaps-online.md] for the reading split
- **Validation**: goldens per reading; attestation done, 6,705 suffixed decimals classified by reading
- **Confidence**: `rule` given the reading, `consensus` for the R2 default (`OQ-04-002`, `OQ-07-017` resolved 2026-09-16); a lint must never flag either spelling

---

## DATE: dates and times

### PND-DATE-001 Month names
- **Statement**: Month names are lowercase common nouns with fixed harmony and standard abbreviations with a period; 'in month' is the inessive.
- **Conditions**: month reference.
- **Output**:

| Month | Abbr. | Inessive | Harmony | Month | Abbr. | Inessive | Harmony |
|---|---|---|---|---|---|---|---|
| `január` | `jan.` | `januárban` | B | `július` | `júl.` | `júliusban` | B |
| `február` | `febr.` | `februárban` | B | `augusztus` | `aug.` | `augusztusban` | B |
| `március` | `márc.` | `márciusban` | B | `szeptember` | `szept.` | `szeptemberben` | F |
| `április` | `ápr.` | `áprilisban` | B | `október` | `okt.` | `októberben` | F |
| `május` | `máj.` | `májusban` | B | `november` | `nov.` | `novemberben` | F |
| `június` | `jún.` | `júniusban` | B | `december` | `dec.` | `decemberben` | F |

  - Other forms: `márciusi`, `májusig`, `decembertől`, `január elején`, `február végén`, `május közepén`; possessed after a year: `1848 márciusában`, `2014 májusában`; `október havában`.
- **Examples**: `Márciusban születtem.`, `április elseje`.
- **Exceptions**: none.
- **Predictability**: `lexical` (closed, 12)
- **Lexical input**: month table
- **Interactions**: `ORT-CAP` (lowercase), `PND-DATE-004`
- **Sources**: [AkH12 §284 (abbreviations), §295, §296], [KVF98 p. 249]
- **Validation**: goldens
- **Confidence**: `normative` (abbreviations); `consensus`

### PND-DATE-002 Weekdays, day parts, seasons
- **Statement**: Weekday names take the superessive for 'on day' except `vasárnap`, which is used bare; day parts and seasons have lexical time-adverbial forms.
- **Conditions**: time adverbial.
- **Output**:
  - `hétfő` -> `hétfőn` [ATT 17], `kedd` -> `kedden`, `szerda` -> `szerdán`, `csütörtök` -> `csütörtökön`, `péntek` -> `pénteken`, `szombat` -> `szombaton`, `vasárnap` -> `vasárnap` (`vasárnapon` [ATT 6] only 'on a Sunday (as an occasion)'); habitual `hétfőnként`, `vasárnaponként`; the only weekday abbreviation in the AkH list is `csüt.`.
  - Day parts: `reggel`, `délelőtt` (`de.`), `délben`, `délután` (`du.`), `este`, `éjjel`, `éjszaka`, `hajnalban`, `éjfélkor`.
  - Seasons: `tavasszal` [ATT 14], `nyáron`, `ősszel`, `télen`.
- **Examples**: `Kedden jövök.`, `Vasárnap pihenünk.`, `du. 5 órakor`, `Ősszel indul.`
- **Exceptions**: see Output.
- **Predictability**: `lexical` (closed)
- **Lexical input**: time adverbial table
- **Interactions**: `NOM-CASE-011`, `NOM-CASE-003` (`tavasszal`, `ősszel` are instrumental forms)
- **Sources**: [KVF98 pp. 248-250], [AkH12 §284 (csüt., de., du.)], [ATT]
- **Validation**: goldens
- **Confidence**: `consensus`

### PND-DATE-003 Day of the month: ordinal + 3sg possessive
- **Statement**: The day of the month is the ordinal with a 3sg possessive suffix (`elseje`, `másodika` ...), and 'on that day' adds the superessive with lengthening (`-án`/`-én`).
- **Conditions**: day number 1-31.
- **Output**:

| Day | Name | On the day | Digits | Day | Name | On the day | Digits |
|---|---|---|---|---|---|---|---|
| 1 | `elseje` | `elsején` | `1-je`, `1-jén` ~ `1-én` | 17 | `tizenhetedike` | `tizenhetedikén` | `17-e`, `17-én` |
| 2 | `másodika` | `másodikán` | `2-a`, `2-án` | 18 | `tizennyolcadika` | `tizennyolcadikán` | `18-a`, `18-án` |
| 3 | `harmadika` | `harmadikán` | `3-a`, `3-án` | 19 | `tizenkilencedike` | `tizenkilencedikén` | `19-e`, `19-én` |
| 4 | `negyedike` | `negyedikén` | `4-e`, `4-én` | 20 | `huszadika` | `huszadikán` | `20-a`, `20-án` |
| 5 | `ötödike` | `ötödikén` | `5-e`, `5-én` | 21 | `huszonegyedike` | `huszonegyedikén` | `21-e`, `21-én` |
| 6 | `hatodika` | `hatodikán` | `6-a`, `6-án` | 22 | `huszonkettedike` | `huszonkettedikén` | `22-e`, `22-én` |
| 7 | `hetedike` | `hetedikén` | `7-e`, `7-én` | 23 | `huszonharmadika` | `huszonharmadikán` | `23-a`, `23-án` |
| 8 | `nyolcadika` | `nyolcadikán` | `8-a`, `8-án` | 24 | `huszonnegyedike` | `huszonnegyedikén` | `24-e`, `24-én` |
| 9 | `kilencedike` | `kilencedikén` | `9-e`, `9-én` | 25 | `huszonötödike` | `huszonötödikén` | `25-e`, `25-én` |
| 10 | `tizedike` | `tizedikén` | `10-e`, `10-én` | 26 | `huszonhatodika` | `huszonhatodikán` | `26-a`, `26-án` |
| 11 | `tizenegyedike` | `tizenegyedikén` | `11-e`, `11-én` | 27 | `huszonhetedike` | `huszonhetedikén` | `27-e`, `27-én` |
| 12 | `tizenkettedike` | `tizenkettedikén` | `12-e`, `12-én` | 28 | `huszonnyolcadika` | `huszonnyolcadikán` | `28-a`, `28-án` |
| 13 | `tizenharmadika` | `tizenharmadikán` | `13-a`, `13-án` | 29 | `huszonkilencedike` | `huszonkilencedikén` | `29-e`, `29-én` |
| 14 | `tizennegyedike` | `tizennegyedikén` | `14-e`, `14-én` | 30 | `harmincadika` | `harmincadikán` | `30-a`, `30-án` |
| 15 | `tizenötödike` | `tizenötödikén` | `15-e`, `15-én` | 31 | `harmincegyedike` | `harmincegyedikén` | `31-e`, `31-én` |
| 16 | `tizenhatodika` | `tizenhatodikán` | `16-a`, `16-án` | | | | |

  - Rule: back ordinal (`-adik`, `-odik`, `második`) -> `-a`, `-án`; front (`-edik`, `-ödik`) -> `-e`, `-én`; `első` -> irregular `elseje`, `elsején`.
  - Further cases on the possessed stem (lengthened): `-áig`/`-éig` (`hatodikáig`, `elsejéig` [ATT 3]), `-ától`/`-étől` (`tizenkettedikétől`, `elsejétől`), `-ára`/`-ére`, `-át`/`-ét`.
- **Examples**: `május harmadikán`, `április huszonhetedikén születtem`, `Hányadika van ma? Tizenötödike.`
- **Exceptions**: `elseje` irregular (`első` + `-je` with `ő` -> `e`).
- **Predictability**: `rule` + 1 lexical form
- **Lexical input**: none
- **Interactions**: `PND-ORD-002`, `NOM-POSS-004`, `NOM-LENG-001`, `PND-DATE-005`
- **Sources**: [KVF98 p. 344], [AkH12 §298], [ATT `elsején` 11, `másodikán` 5, `negyedikén` 3]
- **Validation**: goldens (full table)
- **Confidence**: `consensus`

### PND-DATE-004 Written date formats (summary)
- **Statement**: A full date is year (Arabic, followed by a period) + month (name, abbreviation, Roman or two-digit Arabic) + day (Arabic followed by a period, or a hyphenated suffix without period); the punctuation details are authoritative in `PUN`.
- **Conditions**: full date in digits.
- **Output**: `2014. február 28.`, `2014. febr. 28.`, `2014. II. 28.`, `2014. 02. 28.`; with suffix: `2014. febr. 28-a`, `2014. 02. 28-án`, `1848. márc. 15-én`; ISO 8601 `2014-02-28` in technical data (not an AkH format).
- **Examples**: see Output.
- **Exceptions**: see `PND-DATE-008` for when the year has no period.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-NUM-020`, `PUN-DATEF-001`, `PND-DATE-005`, `PND-DATE-008`
- **Sources**: [AkH12 §295]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-DATE-005 Suffixes on day numbers
- **Statement**: A day number takes its suffix with a hyphen and no period, in the form of the spoken possessed ordinal (`PND-DATE-003`); for 1 the `j` of `elseje` may be written or omitted; for `-ig`, `-tól/-től` the possessive may be omitted; before a postposition the day takes a period or the hyphenated possessive.
- **Conditions**: day number in digits + suffix or postposition.
- **Output**:
  - superessive: `2-án`, `15-én`, `1-jén` ~ `1-én`; nominative: `28-a`, `1-je` ~ `1.`;
  - terminative and ablative: `6-ig` ~ `6-áig`, `12-től` ~ `12-étől`, `1-ig` ~ `1-jéig`, `december 25-től január 27-ig`;
  - before a postposition: `december 20. és 31. között` ~ `december 20-a és 31-e között`; `december 10. óta` ~ `december 10-e óta`.
  - Choice for generation: `-án/-én` with possessive; `1-jén` (both correct; no counts).
- **Examples**: `1848. március 15-én`, `július 6-ig`, `2015. szeptember 1-jétől`.
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-DATE-003`, `PND-NUMINF-010`, `ORT-NUM-017`, `ORT-NUM-019`, `PUN-DATEF-008`, `PUN-DATEF-009`
- **Sources**: [AkH12 §295, §298, §299]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-DATE-006 -i adjectives from dates
- **Statement**: The `-i` adjective of a day ('of the Nth') is written `N-i` (the default, 89% of tokens and 93% in official prose) or `N-ei`/`N-ai` (with the possessive), for 1 also `1-ji`, `1-jei`; the spelled-out form traditionally drops the possessive vowel (`tizenötödiki`).
- **Conditions**: date + `-i`.
- **Output**: `március 15-i` ~ `március 15-ei`; `május 1-i` ~ `1-ji` ~ `1-jei`; `10-i` ~ `10-ei`; back-ordinal days by analogy `20-i` ~ `?20-ai`; spelled: `elseji` ~ `elsejei` [ATT `elsejei` 7, `elseji` -], `tizenötödiki`; generate `N-i`.
- **Examples**: `a március 15-i ünnepség`, `a május 1-jei felvonulás`.
- **Exceptions**: the choice is register-conditioned, with a monotone gradient [ATT evidence/corpus-variants.md 2026-09-16]: `N-i` share literary 99% > technical 95% > official 93% > web 89% > news 88% > wiki 81% > spoken 53%. Spoken transcription is the only register at parity, which is expected because it transcribes the read form. The `-jei` variant is concentrated in the gazette and legislative sources, i.e. it is the official spelling for the 1st of the month.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-DATE-005`, `ORT-NUM-017`
- **Sources**: [AkH12 §298 (1-i, 1-ji, 1-jei)], [ENY-15i] (15-i ~ 15-ei per the dictionary part of the rules; the 2017 Magyar helyesírási szótár shows only 15-i), Nyelvművelő kéziszótár via [ENY-15i] (tizenötödiki), [ATT], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens; attestation (252,845 hyphenated tokens over seven registers; these forms exist in no word list, Webcorpus 2 having zero digit-initial hyphenated forms)
- **Confidence**: `normative` for 1; `measured` for the `N-i` default (89% overall, 93% official) and register-conditioned (`OQ-04-005` resolved)

### PND-DATE-007 Years: suffixes and spelled-out form
- **Statement**: A year in digits takes suffixes with a hyphen by its last spoken word (`PND-NUMINF-010`); spelled out, a year is one word (no group hyphens).
- **Conditions**: year.
- **Output**: `1848-ban`, `1514-ben`, `1900-ban`, `1956-ban`, `1990-ben`, `2000-ben`, `2003-ban`, `2010-ben`, `2020-ban`, `2024-ben`; `1838-as árvíz`, `1956-os`, `2000-es`, `1848-tól`, `2025-ig`, `1989 óta`; spelled: `ezernyolcszáznegyvennyolcban`, `kétezertizennégy`, `kétezerhuszonnégyben`.
- **Examples**: `Magyarország 1514-ben`, `az 1838-as árvíz`.
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-NUMINF-011`, `PND-CARD-006`, `PND-DATE-008`
- **Sources**: [AkH12 §291 a) (years), §298]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-DATE-008 Period after the year
- **Statement**: The year takes a period when it is not grammatically linked to the following word (full dates, `évi`, adjectives in `-i`, month in the nominative); it takes no period when followed by a postposition or postpositional adjective, when it is a possessor, when it is the subject, and optionally on inscriptions, title pages and in parentheses.
- **Conditions**: year followed by another word.
- **Output**: period: `1848. március 9.`, `1848. márciusban`, `1848. áprilisi`, `2014. évi`, `1944. őszi`, `1956. október folyamán`, `1956. október végén`; no period: `1848 márciusában`, `1944 őszén`, `1956 folyamán`, `1848 előtt`, `1848 előtti`, `1867 és 1896 között`, `1849 mártírjai`, `1492 fordulópont volt`, `(1526)`.
- **Examples**: see Output.
- **Exceptions**: in combined years only the second takes the period (`2013–2014. évi`, `2013/14. évi`).
- **Predictability**: `rule` (given the syntactic relation)
- **Lexical input**: syntactic relation
- **Interactions**: `ORT-NUM-014`, `ORT-NUM-016`, `PUN-DATEF-005`
- **Sources**: [AkH12 §296, §297]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-DATE-009 Decades and centuries
- **Statement**: Decades are named with the `-(O)s` form of the tens numeral + `évek` (in digits `-as`/`-es` after the full year or the tens); centuries are ordinals + `század`, in Arabic or Roman numerals, and the suffix goes on `század`.
- **Conditions**: decade or century reference.
- **Output**: `a hatvanas évek`, `a 60-as évek`, `az 1960-as évek`, `a kilencvenes évek`, `a 90-es években`, `a 2000-es évek`, `a 2010-es években`; `a 20. század`, `a XX. század`, `a 20. században`, `a XIX. század végén`, `20–21. század` ~ `XX–XXI. század`; `a huszadik század`.
- **Examples**: `az 1990-es évek elején`, `a XVI. században`.
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PND-NUMINF-007`, `PND-ORD-004`, `PND-ART-003`, `PUN-DATEF-014`, `PUN-DATEF-015`
- **Sources**: [AkH12 §294, §296], [KVF98 pp. 250, 345]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-DATE-010 Ranges of years, dates and numbers
- **Statement**: A from-to range is joined by an en dash without spaces between simple members (with spaces between full dates); suffixes attach to the last member (with a hyphen after digits); ordinal periods only after the last member.
- **Conditions**: range.
- **Output**: `1848–49-ben`, `1914–18-as`, `1703–1711 szabadságharca`, `2013–2014. évi`, `2014. április 28–30.`, `2014. április 11. – 2014. október 10.`, `a 128–9. lapon`, `a 128–9.-en`, `az I–IV.-ben`, `június–augusztus hónapban`, `Budapest–Bécs`; approximate numbers use a hyphen instead (`PND-CARD-010`).
- **Examples**: see Output.
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-HYPH-013`, `ORT-NUM-018`, `PUN-DATEF-006`, `PUN-DATEF-007`, `PND-CARD-010`
- **Sources**: [AkH12 §264 c), §296, §297 (`1703–1711 szabadságharca`)]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-DATE-011 Clock times
- **Statement**: Times in digits separate hours and minutes by a period or colon without space and take hyphenated suffixes by the last spoken word; written minutes are always read, including `:00` (`tíz nulla-nulla` or `tíz óra`), so there is no bare-hour reading of a written `HH:00`. Spoken times use `óra`/`perc` or the fraction system `negyed`/`fél`/`háromnegyed` + next hour.
- **Conditions**: time of day.
- **Output**:
  - written: `10.35` ~ `10:35`; `10.35-kor` ~ `10:35-kor`, `10:35-ig`, `10:35-ös` (`harmincötös`), `10:30-as` (`harmincas`), `16.25-kor`; words: `10 óra 25 perckor`, `du. 5 órakor`;
  - spoken full: `tíz óra harmincöt perc`, `tíz harmincöt`; hour-only: `tízkor`, `tizenkettőkor` [ATT `kettőkor` 6, `kétkor` -] (independent `kettő`);
  - fraction system (12-hour, next hour): 10:15 `negyed tizenegy`, 10:30 `fél tizenegy`, 10:45 `háromnegyed tizenegy`; `negyed kilenckor`, `fél tizenkettőig`, `háromnegyed négyre`, `fél kettő` (never `*fél két`);
  - relative: `öt perccel múlt tíz`, `tíz óra múlt öt perccel`, `öt perc múlva tizenegy`; `délben`, `éjfélkor`;
  - day part: `reggel 8-kor`, `de. 10 órakor`, `este 9-kor`.
- **Examples**: `A vonat 8:15-kor indul.`, `Fél hétkor találkozunk.`
- **Exceptions**: generate `-ra` for `HH:00`: both full readings end in a back `-a` word (`tíz nulla-nulla` -> `nullára`, `tíz óra` -> `órára`). `HH:00-re` does occur (387 against 1,187 `-ra`, `[ATT]`: `7:00-re jártam dolgozni`, `04:00-re oda is értem`) from a bare-hour reading that skips the written `:00`; it is the minority and judged incorrect (native-speaker judgement 2026-09-16), so `warning`, not `error`. The fraction reading does not surface in suffixes at all: `HH:30-ra` 2,343 against `HH:30-re` 22, so a written `10:30` is suffixed from `tíz harminc`, not from `fél tizenegy` (`OQ-04-011` resolved 2026-09-16).
- **Predictability**: `rule` (given the reading)
- **Lexical input**: reading; default reading for generation of `HH:00` is `óra` (`10 órára`)
- **Interactions**: `ORT-NUM-009`, `PUN-DATEF-012`, `PND-FRAC-002`, `PND-NUMINF-011`; same reading-dependent suffixation as `PND-FRAC-007` (decimals)
- **Sources**: [AkH12 §293, §300, §265 d)], [KVF98 pp. 247-248], [ATT]; native-speaker judgement 2026-09-16 (`10:00` is always `-ra`; `tíz harminc` -> `-ra` is the anti-example showing other times split)
- **Validation**: goldens; attestation. Counts of `\d+:\d+-r[ae]` need a homograph filter, since sports scores (`50:30-ra vertük`) and durations (`4:30-re` of a skating programme) share the notation
- **Confidence**: `normative` (writing); `consensus` (spoken system); `rule` given the reading, for suffixation

### PND-DATE-012 Eras
- **Statement**: Era designations are abbreviations with periods and spaces, placed before the year; suffixes go on the year.
- **Conditions**: era marking.
- **Output**: `i. e.` (`időszámításunk előtt`), `i. sz.` (`időszámításunk szerint`), `Kr. e.` (`Krisztus előtt`), `Kr. u.` (`Krisztus után`); `i. e. 44-ben`, `Kr. u. 476-ban`, `i. sz. 2. század`, `i. e. 5. századi`.
- **Examples**: `Kr. e. 753-ban`, `az i. sz. 1. században`.
- **Exceptions**: none.
- **Predictability**: `lexical` (closed, 4)
- **Lexical input**: abbreviation list
- **Interactions**: `ORT-ABBR-019`, `PND-ART-007` (article follows the expanded abbreviation: `az i. sz. 2. század`, `a Kr. e. 1. évezred`)
- **Sources**: [AkH12 §277 a), §284]
- **Validation**: goldens
- **Confidence**: `normative`

---

## MEAS: measures and currency

### PND-MEAS-001 Unit symbols and suffixes
- **Statement**: Unit and currency symbols have no period, are separated from the number by a space (except the geometric degree sign), and take suffixes with a hyphen in the form required by the pronounced unit name, with its harmony, linking vowel and `v`-assimilation.
- **Conditions**: number + unit symbol (+ suffix).
- **Output**: `100 km`, `1 kg`, `10 kWh`, `120 km/h`, `220 W`, `45°`; suffix = spoken unit name + suffix minus spoken unit name (`PND-MEAS-002`).
- **Examples**: `Ft-ot`, `Ft-tal`, `K-ot`, `K-mal` (`káliummal`), `m²-enként`, `m²-es`, `°C-on`, `°C-kal`, `°C-os`, `100 °C-on`.
- **Exceptions**: symbols read in a shortened colloquial form (`PND-MEAS-003`).
- **Predictability**: `rule`
- **Lexical input**: unit name per symbol
- **Interactions**: `ORT-HYPH-019`, `ORT-ABBR-006`, `ORT-ABBR-009`, `PUN-NUMF-007` (spacing)
- **Sources**: [AkH12 §279, §280, §282]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-MEAS-002 Unit table
- **Statement**: The common units have fixed spoken names and suffix forms.
- **Conditions**: unit symbol.
- **Output**:

| Symbol | Spoken name | Harm. | Acc. | Instr. | `-s` | Supess. |
|---|---|---|---|---|---|---|
| `m` | `méter` | F | `m-t` | `m-rel` | `m-es` | `m-en` |
| `km`, `cm`, `mm`, `dm` | `kilométer`, `centiméter`, `milliméter`, `deciméter` | F | `-t` | `-rel` | `-es` | `-en` |
| `m²`, `km²`, `m³`, `cm³` | `négyzetméter`, `négyzetkilométer`, `köbméter`, `köbcentiméter` | F | `-t` | `-rel` | `-es` | `-en` |
| `g`, `kg`, `dkg` ~ `dag`, `mg` | `gramm`, `kilogramm`, `dekagramm`, `milligramm` | B | `-ot` | `-mal` | `-os` | `-on` |
| `t` | `tonna` | B | `t-t` | `t-val` | `t-s` | `t-n` |
| `l` ~ `L`, `dl`, `cl`, `ml` | `liter`, `deciliter`, `centiliter`, `milliliter` | F | `-t` | `-rel` | `-es` | `-en` |
| `s` | `másodperc` | F | `s-et` | `s-cel` | `s-es` | `s-en` |
| `min` | `perc` | F | `min-et` | `min-cel` | `min-es` | `min-en` |
| `h` | `óra` | B | `h-t` | `h-val` | `h-s` | `h-n` |
| `km/h` | `kilométer per óra` | B | `km/h-t` | `km/h-val` | `km/h-s` | `km/h-n` |
| `W`, `kW`, `MW` | `watt`, `kilowatt`, `megawatt` | B | `-ot` | `-tal` | `-os` | `-on` |
| `kWh` | `kilowattóra` | B | `kWh-t` | `kWh-val` | `kWh-s` | `kWh-n` |
| `V` | `volt` | B | `V-ot` | `V-tal` | `V-os` | `V-on` |
| `A` | `amper` | F | `A-t` | `A-rel` | `A-es` | `A-en` |
| `Hz`, `kHz`, `MHz`, `GHz` | `hertz`, `kilohertz`, `megahertz`, `gigahertz` | F | `-et` | `-cel` | `-es` | `-en` |
| `°C` | `Celsius-fok` | B | `°C-ot` | `°C-kal` | `°C-os` | `°C-on` |
| `°` | `fok` | B | `°-ot` | `°-kal` | `°-os` | `°-on` |
| `K` (temperature) | `kelvin` | F | `K-t` | `K-nel` | `K-es` | `K-en` |
| `%` | `százalék` | B | `%-ot` | `%-kal` | `%-os` | `%-on` |
| `‰` | `ezrelék` | F | `‰-et` | `‰-kel` | `‰-es` | `‰-en` |
| `ha` | `hektár` | B | `ha-t` | `ha-ral` | `ha-os` | `ha-on` |
| `kB`, `MB`, `GB`, `TB` | `kilobájt`, `megabájt`, `gigabájt`, `terabájt` | B | `-ot` | `-tal` | `-os` | `-on` |

  - Other suffixes follow the harmony column (`km-re`, `kg-ra`, `l-ben`, `h-ig`, `°C-ig`, `m²-enként`, `km-enként`, `kg-onként`).
- **Examples**: `5 km-re van`, `10 kg-os`, `2 l-es`, `3 m-es`, `100 km/h-val`, `230 V-os`, `50 Hz-es`.
- **Exceptions**: `K` is also the symbol of `kálium` (`K-ot`, `K-mal`) and `kelvin` (`K-t`, `K-nel`): disambiguate by context.
- **Predictability**: `lexical` (unit names, open class) + `rule` (suffixes)
- **Lexical input**: spoken unit name
- **Interactions**: `PND-MEAS-001`, `NOM-ACC`, `NOM-CASE-003`, `NOM-CASE-005` (`grammal`, `wattal`: three letters -> two)
- **Sources**: [AkH12 §279, §282 (Ft-ot, Ft-tal, K-ot, K-mal, °C-on, °C-kal, m²-enként)], [AkH12 §216 b) (hertzcel)]
- **Validation**: goldens
- **Confidence**: `normative` (principle and AkH cells); `consensus` (other cells, derived)

### PND-MEAS-003 Colloquial unit names change the suffix
- **Statement**: Several symbols have an accepted short spoken name, and a text written for that reading takes the corresponding suffix.
- **Conditions**: short reading chosen.
- **Output**: `kg` `kiló`: `kg-t`, `kg-val`, `kg-s` (vs `kg-ot`, `kg-mal`, `kg-os`); `dkg` `deka`: `dkg-t`, `dkg-val`, `dkg-s`; `cm` `centi`: `cm-t`, `cm-vel`, `cm-s` (vs `cm-rel`, `cm-es`); `dl` `deci`: `dl-t`, `dl-vel`; `GB` `giga`: `GB-t`, `GB-val`, `GB-s`; `MB` `mega`.
- **Examples**: `két kiló` = `2 kg`; in edited text the full-name reading dominates: `kg-ot` : `kg-t` = 19,315 : 191 [ATT evidence/corpus-variants.md 2026-09-16].
- **Exceptions**: none; the default reading is now settled as the full unit name (`OQ-04-009`, `OQ-07-016`).
- **Predictability**: `lexical` (AkH names the short readings for `cm`, `dl`, `dkg`, `kg`)
- **Lexical input**: reading flag; default = full unit name
- **Interactions**: `PND-MEAS-002`, `ORT-ABBR-010`
- **Sources**: [AkH12 §279 a) (centi, deci, deka, kiló)], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens per reading; attestation (regex scan over four sources; these are hyphenated tokens, absent from both word lists)
- **Confidence**: `consensus`; the default choice is `measured` - the full-name reading is 98.8% for `kg` and 97.5% for `dkg`

### PND-MEAS-004 Currency
- **Statement**: Currency amounts are written in digits followed by a space and the currency symbol or name; suffixes on symbols follow the Hungarian currency name; ISO codes are used in financial texts.
- **Conditions**: money amount.
- **Output**:

| Symbol / code | Name | Acc. | Instr. | `-s` |
|---|---|---|---|---|
| `Ft`, `HUF` | `forint` | `Ft-ot` | `Ft-tal` | `Ft-os` |
| `€`, `EUR` | `euró` | `€-t` | `€-val` | `€-s` |
| `$`, `USD` | `dollár` | `$-t` | `$-ral` | `$-os` |
| `£`, `GBP` | `font` | `£-ot` | `£-tal` | `£-os` |
| `¥`, `JPY` | `jen` | `¥-t` | `¥-nel` | `¥-es` |
| `Fr`, `CHF` | `frank` | `Fr-ot` | `Fr-kal` | `Fr-os` |
| `Kr` | `korona` | `Kr-t` | `Kr-val` | `Kr-s` |

  - Amounts: `2890 Ft`, `2500 Ft`, `15 €` ~ `15 euró`, `100 USD`; thousands abbreviations `eFt` (`ezer forint`), `M Ft` ~ `millió Ft`, `Mrd Ft` (`milliárd forint`).
  - ISO codes: suffix by the currency name (`HUF-ot` for `forintot`); a letter-by-letter reading would give different forms and is not generated.
- **Examples**: `2500 Ft-ért`, `10 €-val`, `100 $-ral`.
- **Exceptions**: foreign currencies are named by their Hungarian names (`dollár`, `font`, `frank`, `euró`, `jen`, `korona`, `rubel`).
- **Predictability**: `lexical` (names) + `rule` (suffixes)
- **Lexical input**: currency name table
- **Interactions**: `PND-MEAS-001`, `PND-ART-007`, `ORT-ABBR-008`, `PUN-NUMF-008`
- **Sources**: [AkH12 §280, §282, §290]
- **Validation**: goldens
- **Confidence**: `normative` (principle); `consensus` (cells)

### PND-MEAS-005 Symbol + word compounds and numeral + unit adjectives
- **Statement**: A symbol or abbreviation compounded with a word is joined by a hyphen; a numeral in digits + unit adjective is written apart; a spelled-out simple numeral + simple unit adjective is solid.
- **Conditions**: compound or adjective with a unit.
- **Output**: `mm-beosztás`, `fszla.-kivonat`, `kWh-díj`; `5 km-es`, `10 kg-os`, `2 éves`; `ötméteres`, `háromliteres`, `tizenöt méteres`, `öt négyzetméteres`, `tízdekás`.
- **Examples**: see Output.
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: compound status
- **Interactions**: `ORT-HYPH-006`, `ORT-ABBR-007`, `ORT-NUM-012`, `PND-CARD-009`
- **Sources**: [AkH12 §115, §117, §283]
- **Validation**: goldens
- **Confidence**: `normative`

### PND-MEAS-006 Measure phrases
- **Statement**: A measure word between the numeral and the noun forms a quantity phrase in which the measured noun is unmarked and singular and the whole phrase inflects on the noun.
- **Conditions**: numeral + measure noun + noun.
- **Output**: `két kiló alma`, `három liter tejet`, `egy pohár vízzel`, `öt darab tojás`, `fél kiló kenyérért`; measure noun with `-nyi`: `egy maréknyi`, `kanálnyi`.
- **Examples**: `Vettem két kiló almát.`
- **Exceptions**: none.
- **Predictability**: `rule`
- **Lexical input**: measure noun class
- **Interactions**: -
- **Sources**: [KVF98 pp. 96, 219-220]
- **Validation**: goldens
- **Confidence**: `consensus`

---

## Open questions
- `PND-OPEN-1` (`PND-REL-004`): share of `ami` vs `amely` with noun antecedents by register; needs a sentence corpus with antecedent typing (word-list buckets are equal, 25/25).
- `PND-OPEN-2` (`PND-FRAC-007`): which reading (denominator, bare cardinal, `és fél`) writers use when suffixing decimals in digits (`1,2-vel` vs `1,2-del`); no norm and no digit forms in the word list.
- `PND-OPEN-3` (`PND-CARD-005`): `egymillió-ezer` vs `egymillió-egyezer`, `ezerszáz` vs `ezeregyszáz` inside larger spelled-out numbers; CLDR uses bare `ezer`, `száz`; no attestation.
- `PND-OPEN-4` (`PND-ORD-002`): plural linking vowel of back-harmony ordinals (`harmadikak` [ATT 0] vs `harmadikok` [ATT 2]); accusative is clearly `-at`.
- `PND-OPEN-5` (`PND-DATE-006`): preference between `15-i` and `15-ei` in current edited text; the 2017 dictionary shows only `15-i`, the rules' dictionary part both.
- `PND-OPEN-6` (`PND-POSTP-007`): which personal-suffixed postpositions allow dative extraction of the pronoun; no list in the sources.
- `PND-OPEN-7` (`PND-POSTP-010`): `-i` acceptability for `honnan?` postpositions (`mögüli`, `közüli`, `előli`) and for `helyetti`; corpus counts are near zero.
- `PND-OPEN-8` (`PND-PERS-003`): whether `bele` or `belé` should be the default 3sg illative pronoun in formal generation; corpus counts are confounded by the verbal prefix `bele-`.
- `PND-OPEN-9` (`PND-MEAS-003`): default reading of `kg`, `cm`, `dkg` for suffixing in general text (full name vs short name).
- `PND-OPEN-10` (`PND-ART-002`): pronunciation (and so article) of foreign names with initial `h` or `y` in Hungarian usage (`Hugo`, `Huelva`); needs a pronunciation lexicon decision in `PH`.
- `PND-OPEN-11` (`PND-DATE-011`): suffix of `10:00` when the minutes are written but not read (`10:00-ra` vs `10:00-re`).

## Sources used
All keys are in [sources.md](sources.md). Reliability notes for this chapter:
- `LP-HU`, `num2words`: study only.

## Terms
See [glossary.md](glossary.md).
