# 01 Writing system

Scope: the inventory of Hungarian letters and their names, the relation between letters (betű) and characters (írásjegy), segmentation of strings into letters (digraphs, trigraph, simplified doubled digraphs, look-alike sequences at morpheme boundaries), Hungarian alphabetical order (betűrendbe sorolás), letter case of multi-character letters, the Unicode character repertoire needed for Hungarian text, Unicode normalisation and confusables, and legacy 8-bit encodings with their typical mis-decodings. Pronunciation of spellings for suffixation is in `PH`; suffix attachment to names, abbreviations and acronyms, hyphenation and capitalisation rules of words are in `ORT`; usage of punctuation characters is in `PUN`; repair procedures for damaged text are in `REP`. This chapter defines the character- and letter-level substrate those chapters consume.

Normative basis: AkH12 §1-16 (letters, alphabet, collation), §86-92 (traditional spellings, simplified doubling), §155 (historical family names), §226-230 (letter units in hyphenation), §240, §275 (punctuation characters). Everything else is marked as consensus, tendency or implementation decision.

Corpus figures cite `[ATT]`: lower-cased type list of 250 000 words; counts are types, not tokens, and include noise.

## Area index
| Area | Topic | Rules |
|---|---|---|
| `ALPH` | alphabet inventory, order, letter names, suffixing letters, article before letters, spelling alphabet | WS-ALPH-001 .. 013 |
| `LET` | letters vs characters, multi-character letters, `y`, `ch`, `x`, `q`, `w`, archaic and foreign letter groups, `ë`, emphatic repetition | WS-LET-001 .. 011 |
| `DIG` | segmentation into letters, simplified and full doubling, ambiguity at morpheme boundaries, disambiguation | WS-DIG-001 .. 012 |
| `CASE` | case of multi-character letters, all caps, case mapping and folding | WS-CASE-001 .. 008 |
| `CHAR` | character repertoire and code points: letters, apostrophe, quotes, dashes, spaces, invisible and compatibility characters | WS-CHAR-001 .. 011 |
| `NORM` | NFC, decomposed input, selective compatibility folding, confusables, matching equivalences | WS-NORM-001 .. 008 |
| `ENC` | legacy encodings, mis-decodings, detection | WS-ENC-001 .. 013 |

## ALPH: alphabet inventory and letter names

### WS-ALPH-001 The 40-letter Hungarian alphabet and its order
- **Statement**: The Hungarian alphabet (magyar ábécé) is an ordered set of 40 letters; each letter is one unit regardless of how many characters spell it.
- **Conditions**:
  - applies to all Hungarian text
- **Output**: lowercase order: `a á b c cs d dz dzs e é f g gy h i í j k l ly m n ny o ó ö ő p r s sz t ty u ú ü ű v z zs`. Ordinal positions (1-based): a 1, á 2, b 3, c 4, cs 5, d 6, dz 7, dzs 8, e 9, é 10, f 11, g 12, gy 13, h 14, i 15, í 16, j 17, k 18, l 19, ly 20, m 21, n 22, ny 23, o 24, ó 25, ö 26, ő 27, p 28, r 29, s 30, sz 31, t 32, ty 33, u 34, ú 35, ü 36, ű 37, v 38, z 39, zs 40.
- **Examples**: `cs` is one letter in `csók`; `dzs` is one letter in `lándzsa`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-ALPH-002` extends the set; `WS-DIG-001` maps strings to letters.
- **Sources**: [AkH12 §10]; [AkH11 §10] identical inventory.
- **Validation**: goldens (enum order test)
- **Confidence**: normative

### WS-ALPH-002 Extended 44-letter alphabet with q, w, x, y
- **Statement**: The letters `q`, `w`, `x`, `y`, frequent in names and foreign words, are inserted into the alphabet at fixed positions, giving the extended 44-letter alphabet.
- **Conditions**:
  - standalone `q w x y` in any text; `y` as part of `gy ly ny ty` is not the letter `y` (`WS-LET-003`)
- **Output**: `... p q r ... v w x y z zs`; uppercase `... P Q R ... V W X Y Z Zs`. Extended positions: p 28, q 29, r 30, s 31, sz 32, t 33, ty 34, u 35, ú 36, ü 37, ű 38, v 39, w 40, x 41, y 42, z 43, zs 44.
- **Examples**: `taxi`, `Félix`, `Wesselényi`, `Kölcsey`, `quattrocento`
- **Exceptions**: `ch` is not an alphabet letter for ordering.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [AkH12 §11]; [HUWIKI-ABC] (term "kiterjesztett ábécé")
- **Validation**: goldens
- **Confidence**: normative (positions); the label "extended alphabet" is descriptive

### WS-ALPH-003 Vowel letters: seven short-long pairs
- **Statement**: Vowels are written with 14 single-character letters forming seven short-long pairs; length is marked by an acute accent, or by a double acute on `ö ü`.
- **Conditions**:
  - native and hungarianised words; foreign-spelled words may contain other vowel letters (`WS-LET-009`)
- **Output**:

| short | long | short phoneme | long phoneme | harmony class (see `PH-HARM`) |
|---|---|---|---|---|
| a | á | /ɒ/ | /aː/ | B |
| e | é | /ɛ/ | /eː/ | F (neutral) |
| i | í | /i/ | /iː/ | F (neutral) |
| o | ó | /o/ | /oː/ | B |
| ö | ő | /ø/ | /øː/ | R |
| u | ú | /u/ | /uː/ | B |
| ü | ű | /y/ | /yː/ | R |

  The pairs `a/á` and `e/é` differ in quality as well as length.
- **Examples**: `akar` / `akár`; `kemény` / `kémény`; `koros` / `kóros`; `tüzért` / `tűzért`
- **Exceptions**: stylistic multiplication of vowel letters (`Neeem`, `Góóól`) is allowed (`WS-LET-011`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [AkH12 §4, §6]; [KVF98 p. xxx]
- **Validation**: goldens
- **Confidence**: normative (letters); consensus (phoneme values)

### WS-ALPH-004 Consonant letters: 17 single, 8 two-character, 1 three-character
- **Statement**: Consonants are written with 17 single-character letters, 8 two-character letters and 1 three-character letter; no consonant letter carries a diacritic except lowercase `j`, whose dot AkH12 counts as one.
- **Conditions**:
  - native alphabet (40 letters); `q w x` of the extended set are single-character consonant letters
- **Output**:
  - single: `b c d f g h j k l m n p r s t v z`
  - two-character (digraph, kétjegyű): `cs dz gy ly ny sz ty zs`
  - three-character (trigraph, háromjegyű): `dzs`

| letter | phoneme | letter | phoneme |
|---|---|---|---|
| c | /ts/ | cs | /tʃ/ |
| dz | /dz/ | dzs | /dʒ/ |
| gy | /ɟ/ | j | /j/ |
| ly | /j/ | ny | /ɲ/ |
| s | /ʃ/ | sz | /s/ |
| ty | /c/ | zs | /ʒ/ |
| x | /ks/ | q | /k/ |
| w | /v/ | | |

- **Examples**: `batyu`, `csökönyös`, `lándzsa`, `pálma`
- **Exceptions**: consonant length is written by doubling (`WS-DIG-002`, `WS-DIG-003`); `ly` and `j` spell the same phoneme, choice is lexical (`ORT-JLY`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-DIG-001`, `WS-LET-002`
- **Sources**: [AkH12 §3, §7 a), §88]; [KVF98 pp. xxx-xxxi]
- **Validation**: goldens
- **Confidence**: normative

### WS-ALPH-005 Uppercase forms of letters
- **Statement**: Every letter has an uppercase form; multi-character letters have an initial-capital form (only the first character capitalised) and an all-caps form.
- **Conditions**:
  - initial capital (sentence start, proper names): first character only
  - all-caps text and letter-by-letter acronyms: every character
- **Output**: `A Á B C Cs D Dz Dzs E É F G Gy H I Í J K L Ly M N Ny O Ó Ö Ő P (Q) R S Sz T Ty U Ú Ü Ű V (W X Y) Z Zs`; all-caps forms of the multi-character letters: `CS DZ DZS GY LY NY SZ TY ZS`.
- **Examples**: `Csoma`, `Dzsingisz`, `Szolnok`, `Zsolt`; `ENSZ`, `GYSEV`, `CSOMAGMEGŐRZŐ`
- **Exceptions**: see `WS-CASE-001` .. `WS-CASE-003`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-CASE-001`, `WS-CASE-002`
- **Sources**: [AkH12 §3, §5, §8, §10]
- **Validation**: goldens
- **Confidence**: normative

### WS-ALPH-006 Letter names
- **Statement**: Each letter has a spoken name used when spelling out, when referring to a letter, and when reading letter-by-letter acronyms; vowel letters are named by their own sound, most consonants by consonant + `é`/`á`/`ú`, a set of continuants by `e` + consonant.
- **Conditions**:
  - metalinguistic reference to a letter or sound, spelling out, acronyms read letter by letter (`ORT-ABBR-018`)
- **Output** (standard name, then attested variants with `~`):

| letter | name | letter | name |
|---|---|---|---|
| a | `a` | n | `en` ~ `enn` |
| á | `á` | ny | `eny` ~ `enny` |
| b | `bé` | o | `o` |
| c | `cé` | ó | `ó` |
| cs | `csé` | ö | `ö` |
| d | `dé` | ő | `ő` |
| dz | `dzé` | p | `pé` |
| dzs | `dzsé` | q | `kú` |
| e | `e` | r | `er` ~ `err` |
| é | `é` | s | `es` ~ `ess` |
| f | `ef` ~ `eff` | sz | `esz` ~ `essz` |
| g | `gé` | t | `té` |
| gy | `gyé` | ty | `tyé` |
| h | `há` | u | `u` |
| i | `i` | ú | `ú` |
| í | `í` | ü | `ü` |
| j | `jé` | ű | `ű` |
| k | `ká` | v | `vé` |
| l | `el` ~ `ell` | w | `dupla vé` ~ `duplavé` ~ `kettős vé` |
| ly | `ellipszilon` ~ `elipszilon` (informal: `ej`, `ejj`, `elj`, `elly`, `ipszilonos jé`) | x | `iksz` |
| m | `em` ~ `emm` | y | `ipszilon` |
| | | z | `zé` |
| | | zs | `zsé` |
| ch (letter group) | `cé-há` | | |

  Preferred written form of the `ly` name: `ellipszilon` (the form AkH12 uses); `elipszilon` is equally common. In letter-by-letter acronym reading vowel letters are usually pronounced long (`MTA` [em-té-á], `EKG` [é-ká-gé]); `PH-G2P-010` tabulates this long acronym reading; length does not affect suffix choice or writing (`WS-ALPH-009`).
- **Examples**: AkH12 names `ly` as [ellipszilon]; `tbc` read `tébécé`; `tv` read `tévé`.
- **Exceptions**: acronyms may use a lexicalised reading that differs from the letter names (`WC` read `vécé`, not *`dupla vé cé`) (`WS-ALPH-012`).
- **Predictability**: `lexical` (closed class, 44 letters + `ch`)
- **Lexical input**: letter name table (this rule)
- **Interactions**: `WS-ALPH-007` .. `WS-ALPH-012`; `ORT-ABBR-014` for acronym suffixing, `PH-G2P-010` (letter names as pronunciation input)
- **Sources**: [AkH12 §88, §285 b)]; [HUWIKI-ABC] (name list and `ly` variants, citing Osiris Helyesírás 2006 p. 1017 and ÉrtSz)
- **Validation**: goldens; attestation (`vécé` attested, `ábécé` attested)
- **Confidence**: consensus (names are not codified as a list in AkH12; `ly` name spelling not fixed)

### WS-ALPH-007 Stem properties of letter names for suffixation
- **Statement**: A letter used as a noun inflects according to its spoken name: the name's last segment selects the suffix-initial allomorph, its vowels select harmony, and consonant-final one-syllable names behave as lowering stems (linking vowel `e`).
- **Conditions**:
  - letter written as a symbol and followed by a suffix (`WS-ALPH-008`)
- **Output** (derived properties; forms follow `WS-ALPH-008`):

| letter | name | final | harmony | acc. | instr. `-vAl` | dat. `-nAk` | all. `-hOz` |
|---|---|---|---|---|---|---|---|
| a | a | V | B | `a-t` | `a-val` | `a-nak` | `a-hoz` |
| á | á | V | B | `á-t` | `á-val` | `á-nak` | `á-hoz` |
| b | bé | V | F | `b-t` | `b-vel` | `b-nek` | `b-hez` |
| c | cé | V | F | `c-t` | `c-vel` | `c-nek` | `c-hez` |
| cs | csé | V | F | `cs-t` | `cs-vel` | `cs-nek` | `cs-hez` |
| d | dé | V | F | `d-t` | `d-vel` | `d-nek` | `d-hez` |
| dz | dzé | V | F | `dz-t` | `dz-vel` | `dz-nek` | `dz-hez` |
| dzs | dzsé | V | F | `dzs-t` | `dzs-vel` | `dzs-nek` | `dzs-hez` |
| e | e | V | F | `e-t` | `e-vel` | `e-nek` | `e-hez` |
| é | é | V | F | `é-t` | `é-vel` | `é-nek` | `é-hez` |
| f | ef | f | F, lowering | `f-et` | `f-fel` | `f-nek` | `f-hez` |
| g | gé | V | F | `g-t` | `g-vel` | `g-nek` | `g-hez` |
| gy | gyé | V | F | `gy-t` | `gy-vel` | `gy-nek` | `gy-hez` |
| h | há | V | B | `h-t` | `h-val` | `h-nak` | `h-hoz` |
| i | i | V | F | `i-t` | `i-vel` | `i-nek` | `i-hez` |
| í | í | V | F | `í-t` | `í-vel` | `í-nek` | `í-hez` |
| j | jé | V | F | `j-t` | `j-vel` | `j-nek` | `j-hez` |
| k | ká | V | B | `k-t` | `k-val` | `k-nak` | `k-hoz` |
| l | el | l | F, lowering | `l-et` | `l-lel` | `l-nek` | `l-hez` |
| ly | ellipszilon | n | B | `ly-t` | `ly-nal` | `ly-nak` | `ly-hoz` |
| m | em | m | F, lowering | `m-et` | `m-mel` | `m-nek` | `m-hez` |
| n | en | n | F, lowering | `n-et` | `n-nel` | `n-nek` | `n-hez` |
| ny | eny | ny | F, lowering | `ny-et` | `ny-nyel` | `ny-nek` | `ny-hez` |
| o | o | V | B | `o-t` | `o-val` | `o-nak` | `o-hoz` |
| ó | ó | V | B | `ó-t` | `ó-val` | `ó-nak` | `ó-hoz` |
| ö | ö | V | R | `ö-t` | `ö-vel` | `ö-nek` | `ö-höz` |
| ő | ő | V | R | `ő-t` | `ő-vel` | `ő-nek` | `ő-höz` |
| p | pé | V | F | `p-t` | `p-vel` | `p-nek` | `p-hez` |
| q | kú | V | B | `q-t` | `q-val` | `q-nak` | `q-hoz` |
| r | er | r | F, lowering | `r-et` | `r-rel` | `r-nek` | `r-hez` |
| s | es | s | F, lowering | `s-et` | `s-sel` | `s-nek` | `s-hez` |
| sz | esz | sz | F, lowering | `sz-et` | `sz-szel` | `sz-nek` | `sz-hez` |
| t | té | V | F | `t-t` | `t-vel` | `t-nek` | `t-hez` |
| ty | tyé | V | F | `ty-t` | `ty-vel` | `ty-nek` | `ty-hez` |
| u | u | V | B | `u-t` | `u-val` | `u-nak` | `u-hoz` |
| ú | ú | V | B | `ú-t` | `ú-val` | `ú-nak` | `ú-hoz` |
| ü | ü | V | R | `ü-t` | `ü-vel` | `ü-nek` | `ü-höz` |
| ű | ű | V | R | `ű-t` | `ű-vel` | `ű-nek` | `ű-höz` |
| v | vé | V | F | `v-t` | `v-vel` | `v-nek` | `v-hez` |
| w | dupla vé | V | F | `w-t` | `w-vel` | `w-nek` | `w-hez` |
| x | iksz | sz | F, lowering | `x-et` | `x-szel` | `x-nek` | `x-hez` |
| y | ipszilon | n | B | `y-t` | `y-nal` | `y-nak` | `y-hoz` |
| z | zé | V | F | `z-t` | `z-vel` | `z-nek` | `z-hez` |
| zs | zsé | V | F | `zs-t` | `zs-vel` | `zs-nek` | `zs-hez` |
| ch | cé-há | V | B | `ch-t` | `ch-val` | `ch-nak` | `ch-hoz` |

  Sublative/superessive and possessive follow the same properties: `x-re`, `y-ra`, `í-re`, `l-re`; `a-ja`, `e-je`, `o-ja`, `ö-je`, `v-je`, `z-je`, `t-je`; consonant-final names take `-je` too: `l-je`, `n-je`, `s-je`, `sz-je`, `r-je`, `m-je`, `f-je`, `x-je`. Plural of lowering names: `l-ek`, `x-ek`; adjectival `-s`: `l-es`, `n-es` (lowering), `b-s`, `k-s` (vowel-final).
- **Examples** (forms occurring in AkH12 text): `q-t`, `w-t`, `x-et`, `y-t` (§11); `ly-t`, `ly-nal` (§88); `l-et` (§64), `n-et` (§72), `n-nel` (§46), `s-sel` (§70), `sz-szel` (§79), `j-vel` (§53), `gy-vel` (§69), `dz-vel` (§79), `ö-vel` (§67), `u-val` (§23), `ú-val`, `ű-vel` (§31), `h-nak` (§50), `t-nek` (§77), `x-re` (§160), `y-ra` (§215), `l-es` (§63), `z-je` (§83), `v-je` (§82), `a-ja`, `e-je`, `o-ja`, `ö-je` (§216); `d-t` (§62), `h-t` (§41), `p-vel` (§37), `t-vel` (§36), `i-re` (§173), `í-re` (§22), `v-s` (§30); AkH11: `s-et` (§79), `sz-et` (§79), `m-nek`, `ny-nek`, `m-hez`, `ny-hez` (§52), `n-es` (§47), `gy-t` (§82), `h-s` (§75).
- **Exceptions**: forms not found in AkH text and derived by rule: `f-et`, `m-et`, `r-et`, `ny-et`, `f-fel`, `l-lel`, `m-mel`, `r-rel`, `ny-nyel`, `x-szel`, `y-nal`, `q-val`, `w-vel`; possessive of consonant-final names is `-je`, not `-e`: the corpus has `l-je` (`a hold l-je`, `az ajánl l-je`), `n-je` (`a -bAn n-je`), `s-je` (`a sure s-je`), `sz-je` (`a school sz-je`, `melyeknek sz-je`) and no genuine `l-e`/`n-e` of this kind, the apparent `-e` hits all being the interrogative clitic or dates `[ATT]` (`OQ-01-002` resolved 2026-09-16). The informal `ly` names `ej`/`ejj` would give `ly-jel`; standard writing follows `ellipszilon`.
- **Predictability**: `rule` over the lexical name table
- **Lexical input**: letter name (`WS-ALPH-006`); lowering flag for consonant-final one-syllable names (closed class, 9: `ef el em en eny er es esz iksz`)
- **Interactions**: `PH-HARM` (harmony), `NOM-ACC` (lowering), `NOM-CASE-003` (`-vAl` assimilation), `WS-ALPH-008`
- **Sources**: [AkH12 §11, §23, §31, §46, §50, §53, §63, §64, §67, §69, §70, §72, §77, §79, §82, §83, §88, §160, §215, §216]; [AkH11 §47, §52, §79]
- **Validation**: goldens; attestation not applicable (hyphenated forms are not in `[ATT]`)
- **Confidence**: normative for forms occurring in AkH text; consensus for derived forms

### WS-ALPH-008 Writing a letter with a suffix
- **Statement**: A suffix attaches to a letter written as a symbol with a hyphen, in the form required by the spoken letter name; the letter itself is not changed.
- **Conditions**:
  - a single letter (any case, including multi-character letters and `ch`) used as a noun
  - also a sound quoted in square brackets: `[s]-sel`, `[d]-vel`
- **Output**: `<letter>-<suffix>`; `-vAl`/`-vÁ` written with the assimilated consonant spelled once after the hyphen (`s-sel`, not *`s-ssel`); linking vowels written (`x-et`); vowel-final letter names not lengthened in writing (`a-t`, `e-vel`, `o-t`).
- **Examples**: `nagy A-val`; `két l-es`; `hosszú [s]-sel ejtjük`; `kettőzött sz-szel`; `rövid u-val`
- **Exceptions**: letter-by-letter acronyms follow the same pattern with the name of their last letter (`KSH-val`, `DNS-sel`, `BKK-nál`, `TDK-s`) (`ORT-ABBR-014`); letters spelled out as words attach directly (`tévézik`, `tébécés`).
- **Predictability**: `rule`
- **Lexical input**: letter name properties (`WS-ALPH-007`)
- **Interactions**: `ORT-HYPH-019` (hyphen before suffixes of symbols and letters), `ORT-ABBR-014`, `ORT-ASSIM-001`
- **Sources**: [AkH12 §82 f), §275, §282, §288 a)]; letter examples as in `WS-ALPH-007`
- **Validation**: goldens
- **Confidence**: normative

### WS-ALPH-009 Vowel letter names and vowel quality before suffixes
- **Statement**: In writing, vowel-letter names never show final-vowel lengthening (`a-t`, not *`á-t` for the letter `a`); harmony follows the letter's own vowel.
- **Conditions**:
  - letters `a á e é i í o ó ö ő u ú ü ű` used as nouns
- **Output**: `a-t`, `a-val`, `a-ja`; `e-t`, `e-vel`, `e-je`; `o-t`, `o-ja`; `ö-je`, `ö-höz`
- **Examples**: AkH12 §216 `a-ja`, `e-je`, `o-ja`, `ö-je`; §288 `o-t`
- **Exceptions**: none in writing. In speech the vowel does lengthen to its long counterpart (`a` -> [aː], `e` -> [eː], `o` -> [oː]): AkH12 §288 a) words the parallel acronym rule as "írásban nem nyújtjuk az `a`, `e`, `o`-t", i.e. the lengthening is real and merely unwritten. A TTS or phonetic generator must therefore lengthen; a spelling checker must not (`OQ-01-001` resolved 2026-09-16).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: contrasts with `ORT-SUFX-001` (final `a`/`e` lengthening of ordinary words) and `ORT-ABBR-015` (acronym-final `a e o` not lengthened, `MTA-nak`)
- **Sources**: [AkH12 §216 a), §288 a)]
- **Validation**: goldens
- **Confidence**: normative (writing); consensus (pronunciation, from the wording of §288 a))

### WS-ALPH-010 Letter names as lowering stems
- **Statement**: The nine consonant-final one-syllable letter names (`ef el em en eny er es esz iksz`) take the linking vowel `e` before the accusative and plural, although ordinary stems ending in these consonants often take `-t` directly.
- **Conditions**:
  - letters `f l m n ny r s sz x` as nouns
  - accusative `-(V)t`, plural `-(V)k`, adjectival `-(V)s`
- **Output**: `f-et`, `l-et`, `m-et`, `n-et`, `ny-et`, `r-et`, `s-et`, `sz-et`, `x-et`; `l-ek`; `l-es`, `n-es`
- **Examples**: AkH12 §64 `rövid l-et`, §72 `egy n-et`, §11 `x-et`; AkH11 §79 `s-et`, `sz-et`; contrast ordinary stems `dal-t`, `ón-t`, `kosz-t` (`NOM-ACC`)
- **Exceptions**: polysyllabic `ipszilon`, `ellipszilon` take `-t` directly (`y-t`, `ly-t`; AkH12 §11, §88).
- **Predictability**: `lexical` (closed class, 9 names)
- **Lexical input**: stem property `lowering` on the 9 names
- **Interactions**: `NOM-ACC`, `NOM-PL`
- **Sources**: [AkH12 §11, §64, §72, §88]; [AkH11 §47, §79]
- **Validation**: goldens
- **Confidence**: normative for attested forms (`l n s sz x`); consensus for `f m ny r`

### WS-ALPH-011 Definite article before letters
- **Statement**: The article before a letter written as a symbol is chosen by the first sound of the letter name: `az` before a vowel-initial name, `a` otherwise.
- **Conditions**:
  - article immediately before a letter symbol, letter label (`a)`, `f)`) or letter-read acronym
- **Output**:
  - `az`: `a á e é f i í l ly m n ny o ó ö ő r s sz u ú ü ű x y`
  - `a`: `b c cs d dz dzs g gy h j k p q t ty v w z zs ch`
- **Examples**: AkH12 `az ly` (§88), `az s` (§78), `az sz` (§66), `az x-et` (§158), `az n` (§56); `az ENSZ-szel`, `a BKK-nál` (§288)
- **Exceptions**: a word-read acronym uses its word pronunciation (`a NATO`, `az ELTE`) (`PND-ART-006`).
- **Predictability**: `rule`
- **Lexical input**: letter name initial sound (`WS-ALPH-006`)
- **Interactions**: `PND-ART-005`, `PND-ART-006`, `ORT-ABBR-014`
- **Sources**: [AkH12 §56, §66, §78, §88, §158, §160, §288 a)] (article + letter occurrences); [LP-HU] (same set used for list labels; study only)
- **Validation**: goldens
- **Confidence**: normative (via AkH usage); consensus for letters not exemplified

### WS-ALPH-012 Letter names in letter-by-letter acronyms
- **Statement**: A letter-by-letter acronym (betűszó) is read as the sequence of letter names; multi-character letters are read as one name.
- **Conditions**:
  - acronym read by letters (not as a word); determination is lexical (`ORT-ABBR-018`, `PH-G2P-011`)
- **Output**: concatenated names: `SZTK` -> `esztéká`; `tbc` -> `tébécé`; `tv` -> `tévé`; word-read acronyms (`ENSZ` [ensz], `NATO`) and blends (`gyes`) are outside this rule (`PH-G2P-011`)
- **Examples**: `KSH-val` (last name `há`), `DNS-sel` (`es`), `BKK-nál` (`ká`), `TDK-s` (`ká`); the `w` name is written solid, `duplavé` 82 : `dupla vé` 12 [WC2].
- **Exceptions**: lexicalised readings: `WC` -> `vécé`; readings of `W Q X Y` in other acronyms vary (Open questions) - no suffixed acronym ending in a `W`-name occurs in either store, so the acronym reading remains unmeasured.
- **Predictability**: `rule` with lexical exceptions (small, open class)
- **Lexical input**: acronym reading type (letter vs word), exceptional readings
- **Interactions**: `ORT-ABBR-018`, `PH-G2P-011`, `WS-ALPH-007`
- **Sources**: [AkH12 §285 b), §288]; [WC2] (`duplavé`, 2026-09-16); [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens; attestation (`vécé`, `duplavé`)
- **Confidence**: normative (principle); consensus (exceptions)

### WS-ALPH-013 Hungarian spelling alphabet
- **Statement**: For unambiguous spelling out (telephone, radio) letters are identified by given names; the table is conventional and was published in a ministerial decree for amateur radio.
- **Conditions**:
  - spelling out in speech; generation of spelling dialogues
- **Output**: A `Aladár` ~ `Antal`; B `Béla`; C `Cecil`; D `Dénes`; E `Elemér`; F `Ferenc`; G `Géza`; H `Helén`; I `Ilona`; J `János`; K `Károly`; L `László`; M `Mátyás` ~ `Mihály`; N `Nelli`; O `Olga`; P `Péter`; Q `Kvelle`; R `Róbert`; S `Sándor`; T `Tamás`; U `Ubul`; V `Viktor`; W `dupla-Vilmos`; X `ikszes`; Y `ipszilon`; Z `Zoltán`.
- **Examples**: `Kovács`: `Károly, Olga, Viktor, Aladár (hosszú), csé` (digraphs may be spelled as two letters or by name)
- **Exceptions**: no established words for accented letters; digraphs are spelled as two letters (`c`, `s`) or named (`csé`); accent is stated verbally (`á, hosszú`).
- **Predictability**: `lexical` (closed, 26 entries)
- **Lexical input**: table (this rule)
- **Interactions**: `WS-ALPH-006`
- **Sources**: [IHM2006 1. melléklet 1.2] (decree text verified at njt.jog.gov.hu, still in force), [HUWIKI-BETUZ]
- **Validation**: goldens
- **Confidence**: normative (for the amateur-radio domain the decree regulates)

## LET: letters, characters and letter groups

### WS-LET-001 Letter versus character
- **Statement**: The unit of Hungarian spelling is the letter, which is spelled with one, two or three characters; all letter-level operations (length counting, `C`/`V` patterns, doubling, hyphenation, collation, capitalisation) operate on letters, not characters.
- **Conditions**:
  - any processing that counts or classifies segments of written Hungarian words
- **Output**: a word is a sequence of letter tokens; each token records its letter identity, its character span, and a length flag for simplified doubled digraphs (`WS-DIG-002`). Characters outside the alphabet become non-letter tokens carrying the raw character.
- **Examples**: `hosszú` = `h o sz: ú` (5 letters, `sz` long, 6 characters); `dzsungel` = `dzs u n g e l` (6 letters, 8 characters); `jegygyűrű` = `j e gy gy ű r ű` (7 letters)
- **Exceptions**: in names and foreign words, character groups may stand for one sound without being alphabet letters (`WS-LET-007`, `WS-LET-009`); these remain sequences of single letters for collation but act as units for hyphenation.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-DIG-001` (segmentation), `PH` (`C`/`V` classes are over letters)
- **Sources**: [AkH12 §3, §7, §14 b), §226-229]
- **Validation**: goldens
- **Confidence**: normative

### WS-LET-002 Multi-character letters are indivisible
- **Statement**: `cs dz dzs gy ly ny sz ty zs` are single letters: they are never split in hyphenation, capitalised as one unit, and counted as one consonant.
- **Conditions**:
  - native letter sequences not straddling a morpheme boundary (`WS-DIG-006`)
- **Output**: one letter token
- **Examples**: `ka-sza`, `ró-zsa`, `ba-tyu`, `bo-dza`, `ma-dzag`, `bri-dzset`; `mor-zsa`
- **Exceptions**: `dz` and `dzs` were not treated as single letters in hyphenation before AkH11 (1984) (`ma-dzag` then `mad-zag`); historical texts may show the older splits.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-SPLIT-003`, `ORT-SPLIT-011`, `WS-CASE-001`
- **Sources**: [AkH12 §7 a), §226 c)-d), §227]; [KESZLER2016 p. 83] (AkH11 1984: `mad-zag`, `hod-zsa` -> `ma-dzag`, `ho-dzsa`, making `dz`, `dzs` full members of the alphabet); [HUWIKI-ABC]
- **Validation**: goldens
- **Confidence**: normative (current); consensus (historical note, sourced)

### WS-LET-003 Standalone y
- **Statement**: In native Hungarian words `y` occurs only as the second character of `gy ly ny ty`; a `y` not forming one of these letters is the extended-alphabet letter `y`, found in archaic family names (mostly [i]) and foreign words.
- **Conditions**:
  - `y` after a character other than `g l n t`, or at word start: always standalone
  - `y` after `g l n t`: digraph in native and hungarianised words; standalone in archaic names and foreign words where the pair does not spell /ɟ j ɲ c/ (`WS-LET-008`)
- **Output**: letter `y`; sound value lexical: [i] in archaic names (`Kölcsey`, `Ady`), [i] or [j] in foreign words (`hobby`, `New York`)
- **Examples**: `Kölcsey` [kölcsei]; `Vörösmarty` (`t` + `y`); `hollywoodi` (`l l y`); `rallye`
- **Exceptions**: none
- **Predictability**: `rule` for positions after other characters; `lexical` after `g l n t`
- **Lexical input**: spelling-pronunciation entry for names and foreign words
- **Interactions**: `WS-LET-008`, `WS-DIG-009`, `PH` (pronunciation for suffixing)
- **Sources**: [AkH12 §11, §12, §215]; [MESZAROS]
- **Validation**: goldens; oracle:magyarispell (name list)
- **Confidence**: normative (inventory); consensus (sound values)

### WS-LET-004 ch
- **Statement**: `ch` is a two-character unit spelling one sound; it is one letter for hyphenation but `c` + `h` for collation; its sound value depends on origin.
- **Conditions**:
  - common loanwords (`technika`, `archívum`, `pech`), given names and place names (`Richárd`, `Ráchel`, `München`), archaic family names (`Madách`, `Széchenyi`, `Zichy`)
- **Output**:
  - hyphenation unit: `ar-chívum`, `pszi-chológia`, `Mün-chen`, `almana-chot`
  - sound: [h] in loanwords of Greek/German origin (`technika`, `pech`, `Bach`); [cs] in archaic Hungarian names (`Madách` [madács]); other values in other foreign words (`Champagne` [s])
  - `-vAl`: assimilates to the sound: `pechhel`, `Bachhal` (§216 b), `Madáchcsá` (§160 b); line-end split `pech-hel`, `Bach-hal` (§228)
  - long `cch`: split `Bac-chus`, `Pinoc-chio`
- **Examples**: `ar-chí-vum`; `Ri-chárd`; `Kossuth` (not `ch`)
- **Exceptions**: `ch` is kept (not respelled) in given names even though other foreign spellings of given names are hungarianised (`Richárd`, not *`Rihárd`).
- **Predictability**: `rule` (unit status); `lexical` (sound value)
- **Lexical input**: sound value of `ch` per lexeme
- **Interactions**: -
- **Sources**: [AkH12 §11, §12, §158, §160 b), §204, §209, §216 b), §228]
- **Validation**: goldens
- **Confidence**: normative

### WS-LET-005 x
- **Statement**: `x` is a single-character letter spelling two sounds [ksz]; it is kept in established loanwords and names and counts as one consonant letter for hyphenation.
- **Conditions**:
  - loanwords and names with [ksz] spelled `x` (`taxi`, `maximum`, `reflex`, `Félix`, `Mexikó`)
- **Output**: hyphenation `ma-ximum`, `fi-xet`; suffix `-vAl` assimilates to [sz]: `fixszel`, `Félixszel`, `bórax-szá` (hyphenation form)
- **Examples**: `Ale-xandra`; `Marxszal`
- **Exceptions**: [gz] pronounced `x` is usually respelled `gz` (`egzakt`, `egzisztencia`); a few words respell [ksz] as `ksz` (`bokszol`).
- **Predictability**: `lexical` (spelling `x` vs `ksz`/`gz`)
- **Lexical input**: lexeme spelling
- **Interactions**: `ORT-FOR`, `ORT-SPLIT-010`, `PH-G2P-007`
- **Sources**: [AkH12 §158, §160 b), §209, §228, §204]
- **Validation**: goldens
- **Confidence**: normative

### WS-LET-006 q and w
- **Statement**: `q` and `w` occur only in foreign words, foreign names and archaic family names; `w` in archaic names reads [v]; `q` normally occurs as `qu` [kv].
- **Conditions**:
  - `w`: `Wesselényi` [v], `Werbőczy`, `Wass`; foreign `Washington`
  - `q`: `quattrocento`, `Quebec`
- **Output**: single letters; collation positions per `WS-ALPH-002`
- **Examples**: `Wesselényi` [veselényi]; `quattrocento` [kvatrocsentó]
- **Exceptions**: in the archaic group `ew` the `w` is part of a vowel spelling (`Thewrewk` [török]) (`WS-LET-007`).
- **Predictability**: `lexical`
- **Lexical input**: pronunciation of the name or foreign word
- **Interactions**: `WS-LET-007`, `PH`
- **Sources**: [AkH12 §11, §12, §13, §155 d)]
- **Validation**: goldens
- **Confidence**: normative (inventory); consensus (values)

### WS-LET-007 Archaic letters and letter groups in family names
- **Statement**: Historical Hungarian family names keep obsolete spellings; each obsolete letter group has a typical sound value, but the value for a given name is a property of that name.
- **Conditions**:
  - family names with traditional spelling (AkH12 §86, §155); derived adjectives keep them (`móriczi` §161, `kossu-thi` §229)
- **Output**: table of groups (N = named in AkH12 with example; S = secondary sources only):

| group | typical value | examples | status |
|---|---|---|---|
| `aa` | [á] | `Gaal` | N |
| `aá` | [á] | `Gaál`, `Paál` | S |
| `eé` | [é] | `Veér` | N |
| `eö` | [ö] or [ő] | `Eötvös` [ö] (§12); `Beöthy` [ő], `Deseö` [ő] (§229) | N |
| `eo` | [ö] | `Georch` | N (§86) |
| `ew` | [ö] or [ő] | `Thewrewk` [ö]; `Dessewffy` [ő] | N |
| `oó` | [ó] | `Soós` | N |
| `uu` | [ú] ~ [u] | `Kuun` | S |
| `y` (final or after vowel) | [i] | `Kölcsey`, `Ady`, `Egressy`, `Thököly` | N |
| `ay`, `ey` final | [aji] ~ [aj], [eji] ~ [ej] | `Fáy`, `Vay` | S |
| `ch` | [cs] | `Madách`, `Széchenyi`, `Zichy`, `Cházár` | N |
| `cz`, `tz` | [c] | `Czuczor` (§12), `Czetz` (§86), `Benczúr` (§229); `Móricz`, `Rátz` (§160 b) | N |
| `ts` | [cs] | `Takáts`, `Kováts`, `Babits`, `Batsányi` | N |
| `th` | [t] | `Csáth`, `Kossuth`, `Thököly`, `Horváth` | N |
| `tthy` | [tty] | `Batthyány` | N (§86) |
| `gh` | [g] | `Végh` (§160), `Balogh`, `Faraghó`, `Wargha` (§229), `Verseghy` | N (§160, §229) |
| `ff` | [f] | `Dessewffy` (§86), `Lorántffy` (§155, value not given) | N (§86) |
| `ss` | [s] or [zs] | `Kossuth`, `Egressy` [s]; `Dessewffy` [zs] | N (§86) |
| `s` | [zs] | `Jósika` | N |
| `w` | [v] | `Wesselényi` | N |
| `z` | [sz] | `Cházár` [császár] | N (§86) |
| `pp`, `ll`, `ss` etc. final | [single consonant] usually | `Papp`, `Széll`, `Kiss` | spelling N (§160 c, §229); value S (`PH-G2P-004`) |

- **Examples**: `Kossuth` [kosut] ~ [kosút]; `Dessewffy` [dezsőfi]; `Batthyány` [battyányi]; `Thewrewk` [török]
- **Exceptions**: current bearers use the registered form, which may differ (`Dezsőfi`, `Rákóczy`, `Teleky`); some historical names have fixed modernised forms (`Rákóczi`, `Zrínyi`, `Bocskai`) that must not be re-archaised or further modernised.
- **Predictability**: `lexical` (open class of names; typical values above serve as defaults)
- **Lexical input**: per-name spelling-pronunciation mapping
- **Interactions**: -
- **Sources**: [AkH12 §12, §86, §155, §160 b)-c), §161, §229]; [MESZAROS]; [HUWIKI-ARCH] (unsourced list, used only for entries marked S)
- **Validation**: goldens
- **Confidence**: normative for N entries; tendency/contested for S entries

### WS-LET-008 y after g, l, n, t in names: digraph or archaic y
- **Statement**: In family and place names, a written `gy ly ny ty` may be the Hungarian letter or an archaic consonant + `y` [i]; only the name's pronunciation decides.
- **Conditions**:
  - proper names with `g l n t` + `y`, typically word-final
- **Output**: letter segmentation per name: `Arany` = `A r a ny` [arany]; `Király` = `K i r á ly` [király]; `Thököly` = `T h ö k ö l y` [tököli]; `Vörösmarty` = `... r t y` [vörösmarti]; `Kuny` = `K u n y` [kuni]
- **Examples**: see Output
- **Exceptions**: none
- **Predictability**: `lexical`
- **Lexical input**: name pronunciation; absent an entry, default to the digraph reading (native spelling) and flag
- **Interactions**: -
- **Sources**: [AkH12 §12, §86]; [MESZAROS]
- **Validation**: goldens
- **Confidence**: consensus

### WS-LET-009 Foreign letters and letter groups
- **Statement**: Foreign words and names written in their original Latin-script spelling keep foreign letters (with any diacritics) and letter groups; groups spelling one sound are units for hyphenation, individual letters for collation.
- **Conditions**:
  - words written per foreign spelling (not hungarianised): `Greenwich`, `joule`, `Dvořák`, `Škoda`, `Eugène`, `Nîmes`, `Schäfer`, `Werther`, `Bologna`, `Guimarães`, `você`, `Sighişoara`, `piaţa`
  - hungarianised or transliterated words use only Hungarian letters (`Csajkovszkij`, `Plutarkhosz`)
- **Output**:
  - letters preserved exactly (no diacritic stripping, no Hungarian substitution)
  - hyphenation units: one-vowel groups (`ou` in `cou-chette`, `aire` in `Baude-laire`), one-consonant groups (`gn` in `Bo-lo-gna`, `sch` in `Fi-scher`, `ch` in `Chur-chill`, `sh` in `Wa-shing-ton`)
- **Examples**: `cou-chette`, `Fi-scher`, `Wa-shing-ton`
- **Exceptions**: foreign vowel groups spelling a Hungarian vowel keep harmony per pronunciation (`PH-HARM`).
- **Predictability**: `rule` (unit treatment); `lexical` (which groups form units and their sounds)
- **Lexical input**: pronunciation of the foreign word
- **Interactions**: -
- **Sources**: [AkH12 §13, §15, §211, §222, §230]
- **Validation**: goldens
- **Confidence**: normative

### WS-LET-010 Closed e letter ë
- **Statement**: The closed e [ë] of many dialects has no letter in standard spelling and is written `e`; `ë` appears only in phonetic or dialect notation.
- **Conditions**:
  - standard orthography: never `ë`
  - phonetic transcription, dialect texts, pronunciation aids: `ë` allowed
- **Output**: standard `egyetem`, notation [ëgyetëm]
- **Examples**: `ember` [embër]; `lehet` [lëhet]; `gyerekeket` [gyerëkëket]
- **Exceptions**: foreign names may contain `ë` as a foreign letter (`Boër`, Albanian names) (`WS-LET-009`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-CHAR-003`; `REP` (convert notation `ë` -> `e` when normalising to standard spelling)
- **Sources**: [AkH12 §89]; [HUWIKI-ABC]
- **Validation**: goldens
- **Confidence**: normative

### WS-LET-011 Emphatic repetition of letters
- **Statement**: To render unusually long pronunciation, vowel letters may be repeated and consonant letters multiplied; such forms are orthographically allowed and are not subject to simplification rules.
- **Conditions**:
  - stylistic, expressive text
- **Output**: surface form kept; for lexicon lookup collapse runs of identical vowel letters to one (or two, then test long vowel) and runs of three or more identical consonant characters to two, then retry
- **Examples**: `Één?`, `Góóól!`, `Neeem.`; `Nnem!`, `Terringettét!`, `Pssszt!`
- **Exceptions**: sequences of identical vowels also occur regularly in names and loanwords (`Gaal`, `koordinál`, `vákuum`) and in suffixed forms (`fiaiéi`); collapse only as a lookup fallback.
- **Predictability**: `rule`
- **Lexical input**: lexicon for fallback lookup
- **Interactions**: `REP` (normalisation for datasets), `WS-DIG-001`
- **Sources**: [AkH12 §6, §9]
- **Validation**: goldens
- **Confidence**: normative

## DIG: segmentation into letters and doubled digraphs

### WS-DIG-001 Default left-to-right segmentation
- **Statement**: Without morphological information, a string is segmented left to right, case-insensitively, taking at each position (1) the first half of a simplified doubled multi-character letter, else (2) the longest multi-character letter, else (3) a single character.
- **Conditions**:
  - input in NFC (`WS-NORM-001`); compare characters after simple lowercase mapping
  - at position `i` with characters `c0 c1 c2 c3`:
    1. if `c0 = c1` and `c1 c2 (c3)` starts a multi-character letter `D` whose first character is `c0`: emit the first half of long `D` spanning `c0` only; the next step emits `D`
    2. else if `c0 c1 c2` = `dzs`: emit `dzs`
    3. else if `c0 c1` in `cs dz gy ly ny sz ty zs`: emit it
    4. else emit `c0` (letter if in the alphabet, non-letter token otherwise)
- **Output**: token sequence; step 1 + step 2/3 together form one long consonant (`WS-DIG-002`)
- **Examples**: `dzsungel` -> `dzs u n g e l`; `Gyöngy` -> `Gy ö n gy`; `SZÉP` -> `SZ É P`; `meggy` -> `m e g|gy`; `asszony` -> `a s|sz o ny`; `briddzsel` -> `b r i d|dzs e l`; `kedd` -> `k e d d`; `egészség` -> `e g é sz s é g`; `kókuszzsír` -> `k ó k u sz zs í r`
- **Exceptions**: the procedure is wrong at morpheme boundaries that create look-alike sequences (`WS-DIG-006`): `község` -> *`k ö zs é g`, `vízszint` -> *`v í zs z i n t`, `kilencszer` -> *`k i l e n cs z e r`, `leggyorsabb` -> *`l e g|gy o r s a b b`; and in archaic or foreign names (`WS-DIG-009`).
- **Predictability**: `rule` (procedure); correctness depends on `WS-DIG-007`
- **Lexical input**: none
- **Interactions**: `WS-DIG-007` overrides
- **Sources**: [AkH12 §7, §92]; implementation [`crates/picur-morph-ortho/src/letter.rs`](../../crates/picur-morph-ortho/src/letter.rs) (current behaviour)
- **Validation**: goldens
- **Confidence**: consensus (procedure is the standard reading of the norm; the norm itself defines letters by sound, not by string)

### WS-DIG-002 Simplified doubled multi-character letters spell long consonants
- **Statement**: A long consonant spelled by a multi-character letter is written by doubling only its first character: `ccs ddz ddzs ggy lly nny ssz tty zzs`; the sequence is one long letter (or two identical letters, depending on representation).
- **Conditions**:
  - within a stem: `loccsan`, `hosszú`, `meggy`, `fütty`, `asszony`, `könnyű`, `gally`
  - across stem + suffix, by assimilation of `-vAl`/`-vÁ`, of `j`-initial suffixes and imperative `-j`: `jeggyel`, `mésszel`, `rosszal` (`rossz`+`-val`, see `ORT-ASSIM-002`), `briddzsel`, `eddzék`, `eljátsszuk`, `Arannyal`, `Kodállyal`
- **Output**: parse `XXY...` where `XY...` is a multi-character letter as one long consonant; render a long multi-character letter as first character + full letter
- **Examples**: `loccsan` = `l o cs: a n`; `asszonnyal` = `a sz: o ny: a l`
- **Exceptions**: at compound member boundaries the full letter is repeated (`WS-DIG-004`); proper names ending in a doubled single letter take a hyphen before an identical consonant (`Kiss-sel`) (`ORT-HYPH-002`, `ORT-NAME-012`).
- **Predictability**: `rule`
- **Lexical input**: none (boundary type from morphology, `WS-DIG-004`)
- **Interactions**: `WS-DIG-003`, `WS-DIG-004`, `ORT-SPLIT-007` (split as `asz-szony`), `ORT-ASSIM-016`, `NOM-CASE-003`, `VRB-IMP`
- **Sources**: [AkH12 §7 b), §91, §92]
- **Validation**: goldens; oracle:hunmorph-foma
- **Confidence**: normative

### WS-DIG-003 Rendering letters to a string
- **Statement**: Letter sequences are rendered to characters deterministically: a long single-character consonant is doubled, a long multi-character letter is written with its first character doubled, and two identical multi-character letters separated by a compound member boundary are written in full.
- **Conditions**:
  - input: letter tokens with length flag and boundary annotations (`#` word, `+` suffix, `=` compound member)
- **Output**:
  - long single letter: `bb cc dd ff gg hh jj kk ll mm nn pp rr ss tt vv zz`
  - long multi-character letter (within stem or across `+`): `ccs ddz ddzs ggy lly nny ssz tty zzs`
  - `D = D` across compound boundary: full `cscs`, `gygy`, `nyny`, `szsz`, etc.
  - three identical single consonant letters across `+` reduce to two; across `=` and in names, hyphen (`ORT-ASSIM-017`, `ORT-HYPH-001`, `ORT-HYPH-002`)
  - case: `WS-CASE-001`, `WS-CASE-002` applied to the rendered letter
- **Examples**: `jegy=gyűrű` -> `jegygyűrű`; `jegy+gyel` (instr.) -> `jeggyel`; `kulcs=csomó` -> `kulcscsomó`; `kulcs+csal` -> `kulccsal`; `ész=szerű` -> `észszerű`
- **Exceptions**: `dz` and `dzs` long in stems are written single (`WS-DIG-005`).
- **Predictability**: `rule`
- **Lexical input**: compound boundary marks
- **Interactions**: `WS-DIG-004`, `WS-DIG-005`, `ORT-ASSIM-016`, `ORT-ASSIM-017`, `ORT-HYPH-001`
- **Sources**: [AkH12 §7 b), §61, §92, §93]
- **Validation**: goldens
- **Confidence**: normative

### WS-DIG-004 Full doubling at compound member boundaries
- **Statement**: When a compound's first member ends and the second member begins with the same multi-character letter, both are written in full; the sequence is two letters, not one long letter.
- **Conditions**:
  - compound boundary (`=`) between identical multi-character letters; e.g. `nagy=gyűlés`, `jegy=gyűrű` (AkH12 §92)
- **Output**: `cscs`, `dzdz`, `gygy`, `lyly`, `nyny`, `szsz`, `tyty`, `zszs` (and `dzsdzs`); parsing `XYXY` yields two letters `XY`, `XY`
- **Examples**: `kulcscsomó`, `jegygyűrű`, `nagygyűlés`, `nagygyakorlat`, `fénynyaláb`, `díszszázad`, `díszszemle`, `észszerű`, `mészszerű`, `rizsszirup` is not this case (`zs` + `sz`)
- **Exceptions**: AkH11 wrote `ésszerű` as a lexicalised simplified form; AkH12 prescribes `észszerű`. Older texts wrote full doubling also inside stems (`hoszszú`, `öszsze`, `veszszük`); simplified doubling spread from the 1903 school rules and became the Academy norm in 1922, see `WS-DIG-010`.
- **Predictability**: `rule` given the compound analysis
- **Lexical input**: compound boundary
- **Interactions**: -
- **Sources**: [AkH12 §7 b), §92]; [AkH11 §94]
- **Validation**: goldens; attestation (`észszerű` attested in `[ATT]`)
- **Confidence**: normative

### WS-DIG-005 dz and dzs length unmarked in stems
- **Statement**: `dz` and `dzs` are not doubled in stems even where pronounced long; `ddz`, `ddzs` arise only from suffix assimilation.
- **Conditions**:
  - stem-internal `dz`, `dzs`: single (`bodza`, `madzag`, `edz`, `edző`, `pedz`, `maharadzsa`, `bridzs`)
  - assimilation of `j`-initial or `-vAl` suffixes to stem-final `dz`/`dzs`: doubled (`eddzük`, `peddze`, `briddzsel`)
- **Output**: parsing `ddz(s)` implies a morpheme boundary after the stem-final `dz(s)`; `dz(s)` in a stem carries no length flag
- **Examples**: `edz` -> `eddzük`; `bridzs` -> `briddzsel`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-LEN-010`, `VRB-IMP`, `NOM-CASE-003`, `WS-DIG-002`
- **Sources**: [AkH12 §87, §227]
- **Validation**: goldens; attestation (`eddz` attested)
- **Confidence**: normative

### WS-DIG-006 Look-alike sequences at morpheme boundaries
- **Statement**: Morphological concatenation creates character sequences identical to multi-character letters or to simplified doublings; such sequences keep the letters of the two morphs.
- **Conditions**:
  - a morph ending in a character sequence and the next morph beginning with one such that the junction matches a letter pattern of `WS-DIG-001`
- **Output** (inventory; boundary shares re-measured 2026-09-16 on the full Webcorpus 2 type list [WC2], units being distinct lower-cased forms with summed frequency >= 5, restricted to dictionary-valid types; the automatic boundary classifier was validated by hand on 50 random types per digraph, 397/400 agreement. The 2026-09-15 `[ATT]` sample figures they replace were pessimistic because web noise was in the denominator):

| string | boundary reading | example (boundary reading) | letter reading | example (letter reading) | evidence |
|---|---|---|---|---|---|
| `cs` | `c+s` | `gerincsérv`, `táncsport`, `táncstúdió` | `cs` | `harcsa`, `kocsi` | rare |
| `csz` | `c+sz` | `kilencszer`, `nyolcszáz`, `harmincszor`, `láncszem`, `arcszín` | `cs+z` | none found | 26 of 27 types `c+sz`, 1 noise |
| `ccs` | `c+cs` | `tánccsoport`, `piaccsarnok`, `arccsont`, `harccsoport` | `cs:` | `meccs`, `kulccsal`, `fröccs` | 13 of 1,222 dictionary-valid types `c+cs` (1.1%) [WC2 full type list] |
| `dz` | `d+z` | `beszédzavar`, `rövidzár` | `dz` | `edző`, `bodza` | rare |
| `zs` | `z+s` | `község`, `igazság`, `nehézség`, `vízsugár`, `házsor`, `kézsebész`, `nehézsúlyú` | `zs` | `rózsa`, `rézsű`, `vizsga` | `z` + `ság/ség`: 187 of the 191 types matching `(?<!s)zs[áé]g`; 1 `zs`+`ágy` (`masszázságy`), 3 noise |
| `zsz` | `z+sz` | `vízszint`, `házszám`, `tízszer`, `közszolgálat`, `gázszámla` | `zs+z` | `törzszászlós` | 596 of 604 dictionary-valid types (98.7%) are `z+sz` [WC2]; the residue is `zs+z` plus `gyöngyházszínű`, `hegyközszentmiklósi` |
| `ssz` | `s+sz` | `kisszótár`, `kerekesszék`, `tudásszint`, `utasszállító` | `sz:` | `asszony`, `vissza`, `össze` | 1,075 of 23,273 dictionary-valid types `s+sz` (4.6%) [WC2]; `-s`-final first member before an `sz`-initial second member |
| `ggy` | `g+gy` | `leggyorsabb`, `meggyőző`, `országgyűlés`, `taggyűlés`, `joggyakorlat`, `üveggyár` | `gy:` | `meggy`, `faggyú`, `heggyel`, `higgy` | 956 of 1,724 dictionary-valid types `g+gy` (55.5%), 74.8% of tokens [WC2]; the `meg`/`leg` prefix rule alone covers 2,074 |
| `nny` | `n+ny` | `vagyonnyilatkozat`, `tannyelvű`, `szezonnyitó`, `negyvennyolcas` | `ny:` | `könnyű`, `annyi`, `dinnye` | 182 of 5,135 dictionary-valid types `n+ny` (3.5%) [WC2] |
| `zzs` | `z+zs` | none found | `zs:` | `rizzsel`, `garázzsal` | 15 types, all `zs:` or `sz+zs` |
| `lly` | `l+ly` | none found (native) | `ly:` | `hellyel`, `süllyed`, `gally` | foreign `l l y`: 313 of 6,911 raw types (`hollywoodi`, `rallye`), but only 1 of 812 dictionary-valid types [WC2] |
| `tty` | `t+ty` | none found | `ty:` | `hattyú`, `fütty` | foreign `t t y`: 42 of 8,039 raw types (`Betty`), 0 of 1,780 dictionary-valid types [WC2] |
| `gy ly ny ty` | `g/l/n/t + y` | archaic and foreign names only (`WS-DIG-009`) | digraph | native | |

- **Examples**: see table
- **Exceptions**: `s+z` at a boundary: no attested native case found in `[ATT]`; `d+zs`, `dz+s`: none found.
- **Predictability**: `rule` given morphology; `tendency` without it (figures above)
- **Lexical input**: morphological segmentation
- **Interactions**: -
- **Sources**: [AkH12 §7, §14 a)-b), §226 d)]; counts [WC2] (full type list, 2026-09-16), superseding [ATT] (2026-09-15); [ATT evidence/calibration.md 2026-09-16]
- **Validation**: attestation (all examples attested in `[ATT]`); goldens; full-type-list measurement done for `ccs nny ssz ggy zsz lly tty ddz`
- **Confidence**: normative (principle: letters denote sounds of morphs); measured (figures; full type list, no sampling)

### WS-DIG-007 Morphology-aware segmentation
- **Statement**: Given a morphological segmentation, letters are formed only within morphs; a multi-character letter never spans a morph boundary, except that the doubled first character of a simplified long letter produced by assimilation belongs to the long letter.
- **Conditions**:
  - input: surface string with morph boundary offsets (from the analyser or generator)
- **Output**:
  1. mark boundaries on the character string
  2. for each boundary created by an assimilating suffix whose output is a simplified long letter (`kulcs+val` -> `kulccsal`, `vesz+jük` -> `vesszük`), treat the doubled character plus the following letter as one long letter token
  3. segment every remaining boundary-free substring with `WS-DIG-001`
  4. at a compound boundary between identical multi-character letters, emit two letters (`WS-DIG-004`)
- **Examples**: `köz+ség` -> `k ö z | s é g`; `víz=szint` -> `v í z | sz i n t`; `leg+gyorsabb` -> `l e g | gy o r s a b b`; `kilenc+szer` -> `k i l e n c | sz e r`; `kis=szótár` -> `k i s | sz ó t á r`; `vagyon=nyilatkozat` -> `... o n | ny i ...`; `meggy+ből` -> `m e gy: b ő l`
- **Exceptions**: a surface string with several analyses may have several segmentations (`meggyes`: `meggy+es` 'with sour cherries' only; `meg+gyes` not a word); keep all analyses and their segmentations.
- **Predictability**: `rule`
- **Lexical input**: morphological analysis (`picur-morph-core`)
- **Interactions**: overrides `WS-DIG-001`
- **Sources**: [AkH12 §7, §92, §226, §233]
- **Validation**: goldens; oracle:hunmorph-foma; oracle:emMorph
- **Confidence**: normative (principle); consensus (procedure)

### WS-DIG-008 Heuristic segmentation without full analysis
- **Statement**: When no analysis is available, apply ordered boundary heuristics before `WS-DIG-001`; each has a measured or estimated precision and must mark its output as heuristic.
- **Conditions** (apply in order, first match wins, per occurrence):
  1. `csz` -> `c` + `sz` (26/26 native types)
  2. `zsz` -> `z` + `sz` unless the left part is a `zs`-final stem from the lexicon followed by a `z`-initial morph (`törzs=zászlós`): 98.7% of dictionary-valid `zsz` types are `z`+`sz` (596 of 604) [WC2]; the residue is `zs`+`z` plus `gyöngyházszínű` and `hegyközszentmiklósi`, which need the first-member list of step 6. `szsz` is handled by `WS-DIG-001`
  3. `z` + `ság`/`ség` at the end of a stem: `zs[áé]g` preceded by a character other than `s` -> `z` + `s` (187 of 191 types; `masszázságy` needs the stem list)
  4. word-initial `leg`, `legesleg`, `meg`, `legmeg` followed by `gy` -> `g` + `gy`, except the lexeme `meggy` and its inflected, derived and compound forms and the place names built on it (`meggyel`, `meggyet`, `meggyes`, `meggypiros`, `meggylekvár`, `meggyfa`, `meggymag`, `Meggyeskovácsi`, `Nyírmeggyes`, `Zalameggyes`); 55.5% of dictionary-valid `ggy` types are `g|gy` boundaries (956 of 1,724), 74.8% of tokens, and this prefix rule alone covers 2,074 of them [WC2]
  5. lexicon of `g`-final first members + `gy`-initial second members (`ország`, `tag`, `jog`, `ideg`, `üveg`, `anyag`, `tömeg`, `szöveg`, `ág`, `szalag`, `nyereg`, `hang`, `szőnyeg`, `virág`, `csillag`, `egészség` + `gyűlés`, `gyakorlat`, `gyártás`, `gyógy...`, `gyilkos...`, `gyulladás`, `gyöngy`) -> `g` + `gy`
  6. first member lexicon for `köz`, `víz`, `ház`, `tűz`, `kéz`, `gáz`, `rész`, `tíz`, `száz`, `húsz` + `s`/`sz`-initial member -> boundary before the second member
  7. `c`-final numerals and nouns (`kilenc`, `nyolc`, `harminc`, `tánc`, `piac`, `harc`, `arc`, `gerinc`, `lánc`, `árbóc`) + `cs`/`s`/`sz`-initial morph -> boundary
  7b. a string created at a stem+suffix junction where the lemma ends in the single letter is a boundary (`száz`+`szor`, `tíz`+`szer`, `vigyáz`+`sz`) - worth 71,192 `zsz` tokens on its own [WC2]
  8. otherwise `WS-DIG-001` (which reads `ccs`, `nny`, `ssz`, `ddz`, `lly`, `tty` as long letters: correct in 98.9% (`ccs`, 1,222 types), 96.5% (`nny`, 5,135), 95.4% (`ssz`, 23,273), 98.1% (`ddz`, 53), 99.9% (`lly`, 812) and 100% (`tty`, 1,780) of dictionary-valid types on the full Webcorpus 2 type list; token-weighted 99.97%, 99.41%, 99.13% and >= 99.9% for the other three [WC2]. On unfiltered text `lly` falls to 94.2% through foreign `ll`+`y`, so `WS-DIG-009` is consulted first for names. `ggy` and `zsz` must never reach this step: they are handled by steps 4 and 2)
- **Output**: segmentation flagged `heuristic`
- **Examples**: `kilencszer` -> `c|sz` by 1; `vízszint` -> `z|sz` by 2; `igazság` -> `z|s` by 3; `leggyakrabban` -> `g|gy` by 4; `meggyes` -> `gy:` by 4 exception; `országgyűlés` -> `g|gy` by 5
- **Exceptions**: the lists in 5-7 are open; unknown compounds fall to step 8.
- **Predictability**: `tendency` (heuristic; figures in `WS-DIG-006`)
- **Lexical input**: first-member and stem lists (open class); `zs`-final stem list
- **Interactions**: `WS-DIG-006`, `WS-DIG-007` (preferred)
- **Sources**: [WC2] (full type list, 2026-09-16), superseding [ATT] (2026-09-15); [AkH12 §14 b) example `nagygyakorlat`]; [ATT evidence/calibration.md 2026-09-16]
- **Validation**: attestation; goldens; oracle:hunmorph-foma (precision measured, done for the six genuine doubled digraphs: full-type-list measurement 2026-09-16)
- **Confidence**: tendency overall; the step-8 long-letter default for `ccs`/`nny`/`ssz`/`ddz`/`lly`/`tty` is `consensus` on dictionary words

### WS-DIG-009 Segmentation of archaic and foreign names
- **Statement**: In names with archaic or foreign spelling, character groups follow the name's pronunciation: a `gy ly ny ty cs sz zs` sequence may be two letters, and non-alphabet groups (`ch cz th ts ew eö`) are sequences of single letters for collation.
- **Conditions**:
  - proper names and foreign words with a lexicon entry giving pronunciation
- **Output**: letter sequence per entry; default without entry: `WS-DIG-001` plus flag `name-uncertain`
- **Examples**: `Thököly` -> `T h ö k ö l y`; `Vörösmarty` -> `... t y`; `Arany` -> `A r a ny`; `Czuczor` -> `C z u c z o r`; `Batthyány` -> `B a t t h y á ny`; `hollywoodi` -> `h o l l y w o o d i`
- **Exceptions**: none
- **Predictability**: `lexical`
- **Lexical input**: name/foreign word pronunciation
- **Interactions**: -
- **Sources**: [AkH12 §12, §15, §229, §230]; [MESZAROS]
- **Validation**: goldens
- **Confidence**: consensus

### WS-DIG-010 Obsolete unsimplified doubling
- **Statement**: Before the simplified doubling was adopted, long multi-character letters were written in full inside words (`hoszszú`, `öszsze`); such spellings are obsolete and parse as one long letter.
- **Conditions**:
  - historical texts (school spelling from 1903, Academy rules from 1922 adopted simplification) and OCR of such texts
  - modern texts: error
- **Output**: `szsz`, `gygy`, `nyny`, `cscs`, `lyly`, `tyty`, `zszs` inside a stem or before a suffix -> long letter; modern spelling renders simplified (`hosszú`, `össze`, `vesszük`, `ősszel`, `tavasszal`). At a compound boundary the full digraph is written with a hyphen instead (`hossz-szelvény`, `össz-szellemiség`, AkH12 §61, §92) `[ENYELV 2021-11-01]`, `[ENYELV 2023-02-27]`, which supports reading §79's `ssz-el` as a slip against §80 (`OQ-01-013` resolved 2026-09-16)
- **Examples**: `[ATT]` contains `hoszszú`, `hoszszabb`, `öszsze`, `öszszes`, `veszszük`, `viszsza`, `tavaszszal`, `őszszel`, `aszszony`, `profeszszionális`
- **Exceptions**: compound boundaries keep full doubling in modern spelling (`WS-DIG-004`); suffixed proper names kept full doubling until AkH10 (1954), which extended simplification to them (`Aranynyal` -> `Arannyal`, `Keménynyel` -> `Keménnyel`) `[KESZLER2016 p. 83]`, so historical proper-name spellings modernise on the same rule (`OQ-01-008` resolved 2026-09-16)
- **Predictability**: `rule` (given stem vs compound boundary)
- **Lexical input**: compound analysis
- **Interactions**: `REP` (modernisation), `WS-DIG-004`
- **Sources**: [KESZLER2016 p. 83] (AkH10 and AkH11 changes); [LACZKO24 pp. 12-13] (Simonyi 1903 school orthography, adopted by the Academy in 1922); [AkH12 §92]; [ATT]
- **Validation**: attestation
- **Confidence**: consensus (AkH10/AkH11 dates from an orthography-committee member's study; 1903/1922 from secondary sources)

### WS-DIG-011 Hyphen as a hard boundary
- **Statement**: A hyphen inside a word is a hard boundary for letter segmentation: no letter and no simplified doubling spans it.
- **Conditions**:
  - intra-word hyphen (`U+002D`, `U+2010`, `U+2011`), including hyphens required before suffixes of names with doubled final letters and at three identical letters
- **Output**: segment each side independently
- **Examples**: `Kiss-sel` -> `K i s s | s e l`; `sakk-kör`; `hossz-szelvény` -> `h o s sz | sz e l ...`; `dzsessz-szerzemény`; `Papp-pal`
- **Exceptions**: a line-end hyphenation hyphen is not part of the word (`WS-DIG-012`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-HYPH-001`, `ORT-HYPH-002`, `ORT-NAME-012`, `WS-DIG-012`
- **Sources**: [AkH12 §61, §93, §160 c), §263 a)]
- **Validation**: goldens
- **Confidence**: normative

### WS-DIG-012 Joining words split at line end with a written-out doubled letter
- **Statement**: Hyphenation writes a simplified long multi-character letter in full on both lines (`asz-szony`); rejoining must restore the simplified form, unless the split is at a compound boundary where full doubling is standard.
- **Conditions**:
  - text with line-end hyphenation: `<left>XY-` newline `XY<right>` where `XY` is a multi-character letter
- **Output**:
  - morphological boundary is suffix or none: join as `<left>X` + `XY<right>` (`asz-` + `szony` -> `asszony`; `kulcs-` + `csal` -> `kulccsal`; `Arany-` + `nyal` -> `Arannyal`)
  - compound boundary: join in full (`kulcs-` + `csomó` -> `kulcscsomó`; `jegy-` + `gyűrű` -> `jegygyűrű`)
  - also `dz`/`dzs`: `edz-` + `dze` -> `eddze`; `bridzs-` + `dzsel` -> `briddzsel`
  - undecidable without morphology: produce both candidates and choose by lexicon
- **Examples**: see Output
- **Exceptions**: a hyphen that is part of the word (`Kiss-sel`) must not be removed; distinguish by lexicon (`WS-DIG-011`).
- **Predictability**: `rule` given morphology
- **Lexical input**: compound analysis, lexicon
- **Interactions**: `ORT-SPLIT-007`, `ORT-SPLIT-015`, `ORT-SPLIT-017` (the forward direction), `REP` (dehyphenation)
- **Sources**: [AkH12 §7 b), §226 f)-g), §227, §238]
- **Validation**: goldens
- **Confidence**: normative (forward rule); consensus (inverse procedure)

## CASE: letter case

### WS-CASE-001 Initial capital of a multi-character letter
- **Statement**: When a word begins with a capital (sentence start, proper name, heading), only the first character of an initial multi-character letter is uppercase.
- **Conditions**:
  - capitalisation of the first letter of a word (`ORT-CAP` decides when)
  - also each capitalised member of a multi-word name and a capitalised member after a hyphen (`Kelet-Közép-Európa`)
- **Output**: `Cs Dz Dzs Gy Ly Ny Sz Ty Zs`; archaic and foreign groups likewise (`Czuczor`, `Thewrewk`, `Chopin`)
- **Examples**: `Csoma`, `Dzsingisz`, `Szolnok`, `Zsolt`, `Gyula`, `Nyíregyháza`, `Tyukod`, `Lyukóbánya`
- **Exceptions**: all-caps contexts (`WS-CASE-002`); foreign names with their own conventions keep them (Dutch `IJ` in original spelling).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-CAP-003`, `WS-DIG-001` (to find the extent of the first letter)
- **Sources**: [AkH12 §3, §8]
- **Validation**: goldens
- **Confidence**: normative

### WS-CASE-002 All caps and letter-by-letter acronyms
- **Statement**: In text written entirely in capitals and in letter-by-letter acronyms written in capitals, every character of every multi-character letter is uppercase.
- **Conditions**:
  - all-caps rendering of a word or longer text
  - proper-name and technical acronyms in capitals (`ORT-ABBR-011`)
- **Output**: `CS DZ DZS GY LY NY SZ TY ZS`
- **Examples**: `ENSZ`, `GYSEV`, `KMKSZ`, `SZTK`, `CSOMAGMEGŐRZŐ`, `TILOS A DOHÁNYZÁS!`
- **Exceptions**: blends written like names capitalise only the first character (`Műpa`, `Mavad`); common-noun acronyms are lowercase (`tbc`, `tv`) (`ORT-ABBR-011`, `ORT-ABBR-012`).
- **Predictability**: `rule`
- **Lexical input**: acronym type (from `ORT-ABBR-011`)
- **Interactions**: `WS-CASE-001`, `ORT-CAP-003`, `ORT-ABBR-011`
- **Sources**: [AkH12 §8, §285 a), §286]
- **Validation**: goldens
- **Confidence**: normative

### WS-CASE-003 Accents on capitals; dotless capital I
- **Statement**: Capital vowel letters carry the same accents as lowercase in handwriting and print; only short capital `I` has no dot, long `Í` has its acute.
- **Conditions**:
  - all uppercase vowel letters
- **Output**: `Á É Í Ó Ö Ő Ú Ü Ű`; `I` (U+0049); stripping accents from capitals (`ORSZAGGYULES`) is an error
- **Examples**: `Ödön`, `Ürményi`, `Ázsia`, `Óbuda`, `Őriszentpéter`, `Ilona`, `Izland`, `Írország`, `Írott-kő`
- **Exceptions**: none (Turkish `İ` U+0130 is not a Hungarian letter; `WS-NORM-005`)
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `REP` (diacritic restoration of all-caps text), `WS-CHAR-001`
- **Sources**: [AkH12 §5]
- **Validation**: goldens
- **Confidence**: normative

### WS-CASE-004 Case mapping is per character and locale-independent
- **Statement**: Upper- and lowercase mapping of Hungarian letters is one-to-one per character using Unicode simple case mappings with no locale tailoring; multi-character letters map character by character.
- **Conditions**:
  - any case conversion
- **Output**: `á<->Á` ... `ő<->Ő` (U+0151/U+0150), `ű<->Ű` (U+0171/U+0170); `i<->I`; never Turkish/Azeri mappings (`i -> İ`, `I -> ı`); result stays NFC
- **Examples**: `őrület` -> `ŐRÜLET`; `GYŐR` -> `győr`
- **Exceptions**: initial-capital conversion must uppercase only the first character (`WS-CASE-001`); `str::to_uppercase` of a whole word is correct only for all caps; titlecase digraph code points U+01C5, U+01C8, U+01CB, U+01F2 are compatibility characters never used for Hungarian letters (`WS-CHAR-010`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-001`
- **Sources**: [UCD] (SpecialCasing.txt: Turkish rules are conditional on locale); [AkH12 §5, §8]
- **Validation**: goldens
- **Confidence**: consensus

### WS-CASE-005 Case folding for matching
- **Statement**: Case-insensitive matching folds every character with Unicode simple case folding; all case variants of a multi-character letter (`gy Gy GY gY`) fold to the lowercase letter.
- **Conditions**:
  - lexicon lookup, search, deduplication
- **Output**: folded string (NFC, lowercase); original case kept as a separate attribute (`WS-CASE-006`)
- **Examples**: `SZÉP`, `Szép`, `szép` -> `szép`
- **Exceptions**: case distinguishes lexemes (`Szőke` name vs `szőke` 'blond'); matching that must respect proper-name status uses the case attribute.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-CASE-006`, `ORT-CAP`
- **Sources**: [UCD] (CaseFolding.txt); [AkH12 §152 a)] (case distinguishes names)
- **Validation**: goldens
- **Confidence**: consensus

### WS-CASE-006 Case pattern classification of a word
- **Statement**: A word's case pattern is classified over letters, not characters: `lower`, `initial` (first letter in initial-capital form, rest lower), `upper` (all letters all caps), or `mixed`.
- **Conditions**:
  - segmentation by `WS-DIG-001`
- **Output**:
  - `lower`: all characters lowercase
  - `initial`: first letter `Xy`-form (first character upper, rest of that letter lower), remaining letters lower
  - `upper`: all characters uppercase (a one-letter word `GY` or `Á` is `upper`; `Gy` alone is `initial`)
  - `mixed`: anything else (`GYula`, `gYula`, `ENSz`, `iPhone`)
- **Examples**: `Gyula` -> initial; `GYULA` -> upper; `GYula` -> mixed; `Ny` (abbreviation of `nyugat`) -> initial
- **Exceptions**: single-character initial letters in all-caps text are ambiguous between `initial` and `upper` (`A`); resolve by context (neighbouring words).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-CASE-001`, `WS-CASE-002`, `REP` (restoring case), `SYN` (sentence segmentation cues)
- **Sources**: [AkH12 §8]
- **Validation**: goldens
- **Confidence**: consensus

### WS-CASE-007 Mixed-case multi-character letters are non-standard
- **Statement**: A multi-character letter with lowercase first and uppercase second character (`cS`, `gY`, `dZs`) never occurs in standard spelling; within a word the capitalised form `Gy` occurs only word-initially or member-initially.
- **Conditions**:
  - validation and repair of case
- **Output**: flag `cS`-type letters; flag `Gy`-type letters inside an all-caps word (`MAGyAR`) as case errors; repair to the surrounding pattern
- **Examples**: `GYőr` -> error, repair `Győr` or `GYŐR` by context; `SZekszárd` -> `Szekszárd`
- **Exceptions**: camel-case product and brand names are kept as written (`ORT-CAP`).
- **Predictability**: `rule`
- **Lexical input**: brand-name list for exceptions
- **Interactions**: `WS-CASE-006`, `REP`
- **Sources**: [AkH12 §8]; [CLDR-HU] (mixed forms only in an unconfirmed proposal)
- **Validation**: goldens
- **Confidence**: normative

### WS-CASE-008 Case of letters referred to as symbols
- **Statement**: A letter referred to as a symbol keeps the case in which it is discussed; capital form of a multi-character letter referred to as a symbol is its initial-capital form in running text and its all-caps form in all-caps text.
- **Conditions**:
  - metalinguistic letters (`WS-ALPH-008`), list labels, grades and labels (`B porta`, `7/A`)
- **Output**: `nagy Gy-vel` (initial capital form), `a GY betű` in all-caps context
- **Examples**: `nagy A-val`, `kis a-val`; `B porta`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-ALPH-008`
- **Sources**: [AkH12 §3, §8, §150]
- **Validation**: goldens
- **Confidence**: consensus

## CHAR: character repertoire

### WS-CHAR-001 Hungarian letter code points
- **Statement**: Hungarian letters are written with ASCII letters and 18 precomposed Latin letters; no other code points are needed for native Hungarian words.
- **Conditions**:
  - native and hungarianised words
- **Output**:

| lower | code point | upper | code point | NFD |
|---|---|---|---|---|
| á | U+00E1 | Á | U+00C1 | a + U+0301 |
| é | U+00E9 | É | U+00C9 | e + U+0301 |
| í | U+00ED | Í | U+00CD | i + U+0301 |
| ó | U+00F3 | Ó | U+00D3 | o + U+0301 |
| ö | U+00F6 | Ö | U+00D6 | o + U+0308 |
| ő | U+0151 | Ő | U+0150 | o + U+030B |
| ú | U+00FA | Ú | U+00DA | u + U+0301 |
| ü | U+00FC | Ü | U+00DC | u + U+0308 |
| ű | U+0171 | Ű | U+0170 | u + U+030B |

  Multi-character letters are sequences of ASCII letters; there are no dedicated code points for them in Hungarian use.
- **Examples**: `árvíztűrő tükörfúrógép` contains all nine lowercase accented letters; `ÁRVÍZTŰRŐ TÜKÖRFÚRÓGÉP` all nine uppercase
- **Exceptions**: foreign words and names add other code points (`WS-CHAR-003`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-001`, `WS-ENC-002`
- **Sources**: [UCD]; [AkH12 §4, §5]
- **Validation**: goldens
- **Confidence**: normative (letters); consensus (code points)

### WS-CHAR-002 Double acute is the only valid mark on long ö and ü
- **Statement**: `ő ű Ő Ű` must be U+0151, U+0171, U+0150, U+0170 (double acute); look-alike letters with tilde, circumflex, caron, breve, grave or two stacked acutes are not Hungarian letters (`ö ü` in place of `ő ű` are valid letters, handled in `WS-NORM-004`).
- **Conditions**:
  - a character from the look-alike set inside a word that is otherwise Hungarian
- **Output**: classify as damaged `ő`/`ű` (repair in `REP`, detection in `WS-NORM-004`, `WS-ENC-003` .. `WS-ENC-005`): `õ` U+00F5, `ô` U+00F4, `ǒ` U+01D2, `ŏ` U+014F, `ò` U+00F2, `Õ` U+00D5, `Ô` U+00D4; `û` U+00FB, `ǔ` U+01D4, `ŭ` U+016D, `ù` U+00F9, `Û` U+00DB
- **Examples**: `[ATT]` types: `elõ`, `következõ`, `idõ` (962 types with `õ`, 20 447 with `ő`: `õ` in 4.5% of the 21 409 types containing either); `mûködési`, `országgyûlés` (279 with `û`, 4 984 with `ű`: 5.3% of 5 263); `elsô`, `elôtt`, `szerzôdés` (395 with `ô`: 1.9% of 20 842 types with `ô` or `ő`)
- **Exceptions**: in foreign words and names these letters are genuine (`hôtel`, `Rhône`, `Guimarães`, Estonian `Tõnu`, Portuguese `você`).
- **Predictability**: `rule` (validity); `tendency` (frequency of damage)
- **Lexical input**: foreign-word lexicon for exceptions
- **Interactions**: `REP`, `WS-NORM-004`, `WS-ENC-003`
- **Sources**: [UCD]; [AkH12 §4]; [ATT]
- **Validation**: attestation
- **Confidence**: normative (letters); tendency (figures)

### WS-CHAR-003 Foreign letters are preserved
- **Statement**: Foreign words and names in original spelling may contain any Latin-script letter with diacritics; these characters are valid and must not be stripped or replaced.
- **Conditions**:
  - words per foreign spelling (`WS-LET-009`)
- **Output**: character kept
- **Examples**: `Dvořák`, `Škoda`, `Eugène`, `Nîmes`, `Schäfer`, `Guimarães`, `você`, `Sighişoara`, `piaţa`, `Muñoz`, `Møsstrand`
- **Exceptions**: Romanian `ş ţ` (cedilla, U+015F/U+0163) vs `ș ț` (comma below, U+0219/U+021B) are distinct code points for the same letters; match them as equivalent (`WS-NORM-007`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [AkH12 §13, §215]
- **Validation**: goldens
- **Confidence**: normative

### WS-CHAR-004 Apostrophe
- **Statement**: The Hungarian apostrophe (hiányjel) marks omitted sounds and elided year digits; the typographic form is U+2019, plain-text form U+0027; both are the same mark.
- **Conditions**:
  - elision (`hál’ istennek`), shortened years (`’95`), poetic contraction, foreign names (`O’Neill`, `d’Artagnan`)
- **Output**: accept U+2019 and U+0027 as apostrophe; U+02BC (modifier letter apostrophe) equivalent; U+0060 grave, U+00B4 acute, U+2018 and U+2032 prime used as apostrophe are look-alikes to fold (`WS-NORM-007`)
- **Examples**: `hál’ istennek`; `’95` (= 1995)
- **Exceptions**: the apostrophe is not used before Hungarian suffixes (`Kiss’sel` is wrong; `PUN-APOS-005`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PUN-APOS-001`, `WS-NORM-007`
- **Sources**: [AkH12 §275]
- **Validation**: goldens
- **Confidence**: normative (mark); consensus (code point equivalences)

### WS-CHAR-005 Quotation marks
- **Statement**: Hungarian primary quotation marks are low-9 opening and high closing double quotes; the inner pair are guillemets pointing inward.
- **Conditions**:
  - quotation (usage in `PUN-QUOT`)
- **Output**:
  - primary: `„` U+201E opening, `”` U+201D closing
  - inner: `»` U+00BB opening, `«` U+00AB closing
  - look-alikes: `“` U+201C, `"` U+0022, `‟` U+201F, `,,` (two commas), `« »` in French orientation; fold for matching, repair in `REP`
- **Examples**: `„Ahogy a halászok mondják: »beszél a víz«”` (character excerpt; AkH12 §240 j) prints a period after both closing marks, whose placement is `PUN-QUOT-014`)
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PUN-QUOT`, `WS-NORM-007`, `WS-ENC-011` (Windows-1250 bytes 0x84/0x94)
- **Sources**: [AkH12 §240 j)]; [UCD]
- **Validation**: goldens
- **Confidence**: normative

### WS-CHAR-006 Hyphen, dashes and minus
- **Statement**: Hungarian uses a short hyphen (kötőjel) and a long dash that serves both as gondolatjel (parenthetic/dialogue dash) and nagykötőjel (range and relation dash), which have the same length; the minus sign is distinct.
- **Conditions**:
  - usage in `PUN-DASH`
- **Output**:
  - kötőjel: U+002D (standard in digital text), U+2010 typographic, U+2011 non-breaking hyphen; all are hyphens
  - gondolatjel and nagykötőjel: U+2013 en dash
  - minus: U+2212 (`4−2`); U+002D accepted in plain text
  - em dash U+2014, figure dash U+2012, horizontal bar U+2015: not Hungarian standard forms; map to U+2013 when normalising Hungarian typography
  - line-end hyphenation hyphen: not a character of the word (`WS-DIG-012`); soft hyphen `WS-CHAR-007`
- **Examples**: `Afrika-kutató`; `Budapest–Bécs`; `Osztrák–Magyar Monarchia`; `Hihetetlen kemény munka – különösen tűző napon.`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PUN-DASH`, `WS-DIG-011`, `WS-NORM-007`
- **Sources**: [AkH12 §240 g), k), l), §264, §275]
- **Validation**: goldens
- **Confidence**: normative (marks and equal length); consensus (code points)

### WS-CHAR-007 Soft hyphen and invisible format characters
- **Statement**: Invisible format characters carry no letters and must be removed before letter-level processing, while being preserved in round-trip output when required.
- **Conditions**:
  - U+00AD soft hyphen (discretionary hyphenation point), U+200B zero width space, U+200C ZWNJ, U+200D ZWJ, U+2060 word joiner, U+FEFF BOM/ZWNBSP, U+200E/U+200F direction marks
- **Output**: strip for analysis and comparison; soft hyphen positions may be kept as hyphenation hints (they must obey `ORT-SPLIT`, including written-out doubled digraphs, which a soft hyphen cannot express: `asszony` with U+00AD between `s` and `sz` renders `asz-szony` only with a hyphenation-aware renderer)
- **Examples**: `vissza<U+00AD>tér` -> `visszatér`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-007`, `REP`
- **Sources**: [UCD] (General_Category Cf)
- **Validation**: goldens
- **Confidence**: consensus

### WS-CHAR-008 Spaces
- **Statement**: The word space is U+0020; no-break and narrow spaces occur in digit grouping, between number and unit and in typographically set text, and are spaces for segmentation.
- **Conditions**:
  - U+00A0 no-break space, U+202F narrow no-break space, U+2009 thin space, U+2007 figure space, U+3000 ideographic space (foreign input)
- **Output**: treat all as word separators for tokenisation (`SYN`, `PUN`); number grouping `23 816` may use U+0020, U+00A0, U+202F (AkH: space, or period `23.816`)
- **Examples**: `23 816`; `23 °C`; `10%` (no space before `%`)
- **Exceptions**: no-break space after one-letter words (`a`, `s`, `e`) at line end is a typographic habit, not a norm: the typesetting literature states only that such words are "generally" carried to the next line, and AkH12 is silent. Never flag its absence.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PUN-NUMF-002`, `WS-NORM-007`
- **Sources**: [AkH12 §240, §274, §275]; [UCD]; [ELTE-TIPO "Sorvégi a, és"]
- **Validation**: goldens
- **Confidence**: normative (spacing positions); consensus (code points)

### WS-CHAR-009 Other signs in Hungarian text
- **Statement**: Signs listed by AkH12 have fixed code points; several have confusable neighbours.
- **Conditions**:
  - signs in running text
- **Output**:

| sign | code point | confusable |
|---|---|---|
| ellipsis (három pont) | U+2026 or three U+002E | - |
| degree `°` | U+00B0 | `º` U+00BA masculine ordinal, `˚` U+02DA ring |
| percent `%` | U+0025 | - |
| section `§` | U+00A7 | - |
| plus `+`, equals `=` | U+002B, U+003D | - |
| minus `−` | U+2212 | U+002D, U+2013 |
| slash `/` | U+002F | U+2044 fraction slash |
| asterisk `*` | U+002A | - |
| brackets `( ) [ ] { }`, angle `〈 〉` | U+0028 U+0029 U+005B U+005D U+007B U+007D; angle U+27E8 U+27E9 (AkH12 online text uses U+3008 U+3009; U+2329 U+232A normalise to U+3008 U+3009 under NFC) | `< >` |

- **Examples**: `45°`; `23 °C`; `48%`; `4. §`; `Nem értem, hogy…`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PUN`, `ORT-HYPH-019` (`10%-os`, `3°-kal`)
- **Sources**: [AkH12 §240 b), i), §275]
- **Validation**: goldens
- **Confidence**: normative (signs); consensus (code points)

### WS-CHAR-010 Compatibility characters
- **Statement**: Compatibility code points that decompose to Hungarian letter sequences appear in PDF extraction and legacy conversions and must be expanded before letter processing.
- **Conditions**:
  - Latin ligatures U+FB00 `ﬀ`, U+FB01 `ﬁ`, U+FB02 `ﬂ`, U+FB03 `ﬃ`, U+FB04 `ﬄ`, U+FB05/U+FB06 `st`
  - digraph code points U+01F1 `Ǳ`, U+01F2 `ǲ`, U+01F3 `ǳ` (-> `DZ Dz dz`); U+01C4-U+01CC `Ǆ ǅ ǆ Ǉ ǈ ǉ Ǌ ǋ ǌ` (not Hungarian letters: `DŽ Dž dž LJ Lj lj NJ Nj nj`)
  - fullwidth ASCII U+FF01-U+FF5E
- **Output**: NFKC mapping of these characters only (`WS-NORM-003`); `ǳ` -> `dz` then segment
- **Examples**: `ﬁnom` -> `finom`; `boǳa` -> `bodza`
- **Exceptions**: other compatibility characters (`²`, `½`, `ª`) keep their identity (`m²-enként`).
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-003`
- **Sources**: [UAX15]; [UCD]
- **Validation**: goldens
- **Confidence**: consensus

### WS-CHAR-011 Old Hungarian script out of scope
- **Statement**: Text in Old Hungarian (rovásírás, Unicode block U+10C80-U+10CFF) is a different script; it is not processed as Hungarian Latin-script text and is not transliterated by these rules.
- **Conditions**:
  - characters of the Old Hungarian block
- **Output**: non-letter tokens with script `Hung`
- **Examples**: -
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-005`
- **Sources**: [UCD] (Scripts.txt)
- **Validation**: goldens
- **Confidence**: consensus

## NORM: Unicode normalisation and confusables

### WS-NORM-001 NFC at the input boundary
- **Statement**: All text is converted to Unicode Normalization Form C on input; every rule of this rulebook assumes NFC, in which each Hungarian accented letter is one code point.
- **Conditions**:
  - any text entering the library (API strings, files, corpora)
- **Output**: NFC string; idempotent; output of all generators is NFC
- **Examples**: `o` + U+030B -> `ő` U+0151; `a` + U+0301 -> `á` U+00E1
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-003` applies after
- **Sources**: [UAX15]
- **Validation**: goldens (NFC and NFD inputs give identical analyses)
- **Confidence**: consensus

### WS-NORM-002 Decomposed and non-canonical mark sequences
- **Statement**: Decomposed input (macOS file names, some PDF extraction, input methods) composes under NFC; mark sequences that NFC does not compose to a Hungarian letter are look-alikes, not letters.
- **Conditions**:
  - base letter + combining mark sequences
- **Output**:
  - composes to a Hungarian letter: `a e i o u` + U+0301; `o u` + U+0308; `o u` + U+030B; uppercase likewise
  - does not compose to a Hungarian letter, look-alike of `ő`/`ű` (to `WS-NORM-004`): `o` + U+0301 + U+0301; `o` + U+0308 + U+0301; `o` + U+0302 (NFC `ô`); `o` + U+0303 (NFC `õ`); `u` + U+0302 (NFC `û`); `u` + U+0308 + U+0301 (NFC `ǘ` U+01D8)
  - stray combining mark on a consonant or at word start: non-letter token, flagged
- **Examples**: `ko` + U+0308 + `r` -> `kör`; `o` + U+0301 + U+0301 is a look-alike of `ő` and stays uncomposed
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-001`, `WS-NORM-004`
- **Sources**: [UAX15]; [UCD] (canonical decompositions)
- **Validation**: goldens
- **Confidence**: consensus

### WS-NORM-003 Selective compatibility folding, not NFKC
- **Statement**: NFKC is not applied to whole texts because it changes meaningful characters; only a fixed set of compatibility characters is folded.
- **Conditions**:
  - after `WS-NORM-001`
- **Output**: fold: ligatures U+FB00-U+FB06; U+01C4-U+01CC and U+01F1-U+01F3; fullwidth ASCII U+FF01-U+FF5E; U+017F long s -> `s`. Keep: superscripts and subscripts (`m²`, `CO₂`), vulgar fractions (`½`), U+00AA, U+00BA, U+00B5, U+2122; spaces are handled by `WS-NORM-007`
- **Examples**: `ﬁgyelem` -> `figyelem`; `m²-enként` unchanged
- **Exceptions**: none
- **Predictability**: `rule` (by decision)
- **Lexical input**: none
- **Interactions**: `WS-CHAR-010`
- **Sources**: [UAX15]; [AkH12 §282] (`m²-enként` is standard)
- **Validation**: goldens
- **Confidence**: consensus

### WS-NORM-004 Latin look-alikes of Hungarian letters
- **Statement**: Characters visually or historically substituted for Hungarian letters are detected by class; they are replaced only in the repair stage, only when the word is Hungarian and the replacement is validated by the lexicon.
- **Conditions**:
  - look-alike inside a token classified as Hungarian (no other non-Hungarian diacritics; lexicon hit after replacement)
- **Output** (look-alike -> intended letter, typical cause):

| look-alike | intended | typical cause |
|---|---|---|
| `õ` U+00F5, `Õ` U+00D5 | `ő`, `Ő` | Latin-2/1250 bytes decoded as Latin-1/1252 (`WS-ENC-003`) |
| `û` U+00FB, `Û` U+00DB | `ű`, `Ű` | same; also typed substitute (`WS-ENC-005`) |
| `ô` U+00F4, `Ô` U+00D4 | `ő`, `Ő` | typed or font substitute; CWI-2 viewed as CP437 (`WS-ENC-005`, `WS-ENC-007`) |
| `ǒ` U+01D2, `ŏ` U+014F, `ò` U+00F2 | `ő` | font or input substitution (rare) |
| `ǔ` U+01D4, `ŭ` U+016D, `ù` U+00F9 | `ű` | same (rare) |
| `ö` in place of `ő`, `ü` in place of `ű` | `ő`, `ű` | missing double acute on keyboard, font or lossy conversion; ambiguous (`tüzért` and `tűzért` are both words) |
| `o"`, `o''`, `o:`, `o~`, `o` + U+02DD, `o` + U+2033 | `ő` or `ö` | 7-bit notations (`WS-NORM-006`) |
| `?` in place of `ő`, `ű` | `ő`, `ű` | lossy conversion (`WS-ENC-004`) |

- **Examples**: `elõtt` -> `előtt`; `mûködik` -> `működik`; `elsô` -> `első`
- **Exceptions**: foreign words keep `õ ô û` (`hôtel`, `Tõnu`, `Guimarães`).
- **Predictability**: `rule` (detection); `lexical` (validation)
- **Lexical input**: lexicon; foreign-word lexicon
- **Interactions**: `REP`, `WS-CHAR-002`, `WS-ENC-003` .. `WS-ENC-010`
- **Sources**: [UTS39] (confusables); [ATT] (counts in `WS-CHAR-002`)
- **Validation**: attestation
- **Confidence**: consensus

### WS-NORM-005 Mixed-script homoglyphs
- **Statement**: A word mixing Latin letters with Cyrillic or Greek look-alikes is spoofed or OCR-damaged; detection uses script mixing per token.
- **Conditions**:
  - token containing Latin letters and any of: Cyrillic `а е і о р с у х` (U+0430 U+0435 U+0456 U+043E U+0440 U+0441 U+0443 U+0445), `ӧ` U+04E7, `ӳ` U+04F3 (looks like `ű`), `ӱ` U+04F1 (looks like `ü`); Greek `ο` U+03BF, `ό` U+03CC, `α` U+03B1; Turkish `İ` U+0130, `ı` U+0131
- **Output**: flag mixed-script; map through the UTS 39 skeleton to Latin for lookup; replace only in `REP`
- **Examples**: `mӳködik` (Cyrillic `ӳ`) -> `működik`
- **Exceptions**: legitimate mixed-script tokens (formulas, identifiers, `α-sugárzás`)
- **Predictability**: `rule`
- **Lexical input**: lexicon
- **Interactions**: `REP`, `WS-CHAR-011`
- **Sources**: [UTS39]
- **Validation**: goldens
- **Confidence**: consensus

### WS-NORM-006 ASCII accent notations
- **Statement**: In 7-bit environments (early e-mail, Usenet, SMS) accents were written as a following ASCII character or omitted; consistent use of such a notation is recognised rather than treated as punctuation.
- **Conditions**:
  - vowel letter immediately followed by `'` (acute), `:` (diaeresis) or `"` (double acute) inside words, used consistently in the text; or text without any accents (ékezet nélküli)
- **Output**: `a'` -> `á`, `e'` -> `é`, `i'` -> `í`, `o'` -> `ó`, `u'` -> `ú`, `o:` -> `ö`, `u:` -> `ü`, `o"` -> `ő`, `u"` -> `ű`; unaccented text goes to diacritic restoration (`REP`)
- **Examples**: `a'rvi'ztu"ro" tu:ko:rfu'ro'ge'p` -> `árvíztűrő tükörfúrógép`
- **Exceptions**: conventions varied: `o~`, `u~` for `ö`, `ü` (SZTE convention, where `o"`, `u"` are `ő`, `ű`; `REP` table T-ENC-6), `o''` for `ő`; apply only when the whole text uses one notation consistently.
- **Predictability**: `tendency`
- **Lexical input**: lexicon validation
- **Interactions**: `REP`, `WS-ENC-009`
- **Sources**: [GLIBC-ICONV] (ISO646-HU lacks `í ó ú ő ű`); [SZTE-KOD] (`o~` = `ö`, `o"` = `ő`); other practice not documented in a citable source
- **Validation**: goldens
- **Confidence**: tendency

### WS-NORM-007 Equivalence classes for matching
- **Statement**: For tokenisation and matching (not for output), punctuation and space variants are folded into equivalence classes; letters are never folded across accent distinctions.
- **Conditions**:
  - comparison, lookup, deduplication
- **Output**:
  - apostrophe: U+0027, U+2019, U+02BC, U+2018, U+0060, U+00B4, U+2032 (except after digits, `PUN-APOS-006`) -> U+0027
  - double quotes: U+201E, U+201D, U+201C, U+201F, U+0022, U+00AB, U+00BB -> class `quote`
  - hyphen: U+002D, U+2010, U+2011 -> U+002D; dash: U+2013, U+2014, U+2012, U+2015 -> U+2013; minus U+2212 equals U+002D only between digits
  - spaces: U+0020, U+00A0, U+202F, U+2009, U+2007, U+3000 -> U+0020
  - format characters removed (`WS-CHAR-007`)
  - Romanian `ş ţ` (U+015F, U+0163) = `ș ț` (U+0219, U+021B)
- **Examples**: `hál' istennek` = `hál’ istennek`
- **Exceptions**: none
- **Predictability**: `rule` (by decision)
- **Lexical input**: none
- **Interactions**: `WS-CHAR-004` .. `WS-CHAR-008`, `PUN`
- **Sources**: [UCD]; [AkH12 §240]
- **Validation**: goldens
- **Confidence**: consensus

### WS-NORM-008 Accents are significant in equality
- **Statement**: Accented and unaccented vowels are different letters for equality and lexicon lookup; accent-insensitive comparison exists only as an explicit fallback (collation L1, diacritic restoration).
- **Conditions**:
  - equality of words
- **Output**: `kor`, `kór`, `kör`, `kőr` are four distinct strings; an accent-insensitive key is a separate function
- **Examples**: `akar` vs `akár`; `tüzért` vs `tűzért`; `kerek` vs `kerék`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: -
- **Sources**: [AkH12 §6]
- **Validation**: goldens
- **Confidence**: normative

## ENC: legacy encodings

### WS-ENC-001 Internal encoding
- **Statement**: The library processes only valid UTF-8 strings; legacy bytes are decoded at the boundary by a decoder declared by the caller or chosen by `WS-ENC-011`.
- **Conditions**:
  - byte input
- **Output**: UTF-8 NFC string plus the declared or detected source encoding
- **Examples**: -
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-001`, `WS-ENC-011`
- **Sources**: [WHATWG-ENC]
- **Validation**: goldens
- **Confidence**: consensus

### WS-ENC-002 Hungarian letters in legacy code pages
- **Statement**: Hungarian accented letters have the following byte values (hex) in the legacy encodings relevant to Hungarian text.
- **Conditions**:
  - decoding and encoding legacy data
- **Output**:

| letter | ISO-8859-2 | Windows-1250 | CP852 | Mac Central European | CWI-2 | ISO-8859-16 |
|---|---|---|---|---|---|---|
| á | E1 | E1 | A0 | 87 | A0 | E1 |
| é | E9 | E9 | 82 | 8E | 82 | E9 |
| í | ED | ED | A1 | 92 | A1 | ED |
| ó | F3 | F3 | A2 | 97 | A2 | F3 |
| ö | F6 | F6 | 94 | 9A | 94 | F6 |
| ő | F5 | F5 | 8B | CE | 93 | F5 |
| ú | FA | FA | A3 | 9C | A3 | FA |
| ü | FC | FC | 81 | 9F | 81 | FC |
| ű | FB | FB | FB | F5 | 96 | F8 |
| Á | C1 | C1 | B5 | E7 | 8F | C1 |
| É | C9 | C9 | 90 | 83 | 90 | C9 |
| Í | CD | CD | D6 | EA | 8D | CD |
| Ó | D3 | D3 | E0 | EE | 95 | D3 |
| Ö | D6 | D6 | 99 | 85 | 99 | D6 |
| Ő | D5 | D5 | 8A | CC | A7 | D5 |
| Ú | DA | DA | E9 | F2 | 97 | DA |
| Ü | DC | DC | 9A | 86 | 9A | DC |
| Ű | DB | DB | EB | F4 | 98 | D8 |

  Windows-1250 also has Hungarian punctuation: `„` 84, `”` 94, `–` 96, `—` 97, `…` 85, `«` AB, `»` BB, no-break space A0, soft hyphen AD. ISO-8859-2 has none of `„ ” – — … « »` (80-9F are C1 controls; AB is `Ť`, BB is `ť`).
- **Examples**: `ő` = F5 in ISO-8859-2 and Windows-1250 alike
- **Exceptions**: ISO-8859-1, Windows-1252, CP437 and CP850 have no `ő ű Ő Ű`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-ENC-003` .. `WS-ENC-012`
- **Sources**: [UNI-MAP] (8859-2.TXT, CP1250.TXT, CP852.TXT, APPLE/CENTEURO.TXT); [ENWIKI-CWI2]; [GLIBC-ICONV] (CWI bytes 93 96 A7 98 -> `ő ű Ő Ű` verified); ISO-8859-16 column from the Python `iso8859_16` codec
- **Validation**: goldens (round-trip tables)
- **Confidence**: consensus

### WS-ENC-003 Central European bytes decoded as Western European
- **Statement**: ISO-8859-2 or Windows-1250 text decoded as ISO-8859-1 or Windows-1252 changes only the four double-acute letters, into `õ û Õ Û`; all other Hungarian letters come out correct.
- **Conditions**:
  - wrong charset label (HTTP header, HTML meta, e-mail header) or wrong default decoder
- **Output**: `ő -> õ`, `ű -> û`, `Ő -> Õ`, `Ű -> Û`; Windows-1250 punctuation `„ ” – …` survives as Windows-1252 (same bytes)
- **Examples**: `előtt -> elõtt`; `működik -> mûködik`; `ŐSZ -> ÕSZ`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-004`, `REP`
- **Sources**: [UNI-MAP]; [WHATWG-ENC] (label `iso-8859-1` is decoded as windows-1252); [ATT] (962 types with `õ`, 279 with `û`)
- **Validation**: goldens; attestation
- **Confidence**: consensus

### WS-ENC-004 Lossy transcoding into sets without ő and ű
- **Statement**: Converting Hungarian text character by character into ISO-8859-1, Windows-1252, CP437 or CP850 loses the double-acute letters, which become a replacement character, a best-fit letter or the unaccented base.
- **Conditions**:
  - transcoding with substitution
- **Output**: `ő` -> `?` | `o` | `ö` | `õ` | `ô`; `ű` -> `?` | `u` | `ü` | `û`; the substitute depends on the converter's best-fit table
- **Examples**: `előtt -> el?tt`; `első -> elsö`
- **Exceptions**: none
- **Predictability**: `rule` (loss); `lexical` (substitute per tool)
- **Lexical input**: none
- **Interactions**: `REP` (restoration; `ö`/`ő` and `ü`/`ű` are ambiguous without a lexicon)
- **Sources**: [UNI-MAP] (repertoires of target sets)
- **Validation**: goldens
- **Confidence**: consensus

### WS-ENC-005 Substitute letters typed or rendered for ő and ű
- **Statement**: Where keyboards or fonts lacked double-acute letters, writers and typesetters used `ô` or `õ` for `ő` and `û` for `ű`; the stored characters are then valid foreign letters and no decoding error exists.
- **Conditions**:
  - text produced with Latin-1 fonts or layouts; DOS text in CWI-2 viewed as CP437
- **Output**: valid characters `ô õ û` inside Hungarian words; handled as look-alikes (`WS-NORM-004`)
- **Examples**: `[ATT]`: `elsô`, `elôtt`, `megfelelô`, `szerzôdés` (395 types with `ô`)
- **Exceptions**: none
- **Predictability**: `tendency`
- **Lexical input**: none
- **Interactions**: `WS-NORM-004`, `WS-ENC-007`
- **Sources**: [ATT]; [ENWIKI-CWI2] (CWI-2 put `ő ű` at the CP437 positions of `ô û`)
- **Validation**: attestation
- **Confidence**: tendency

### WS-ENC-006 UTF-8 decoded as a single-byte encoding
- **Statement**: UTF-8 Hungarian text decoded as a single-byte code page turns each accented letter into two characters with a recognisable lead character; repeated wrong decoding lengthens the sequences.
- **Conditions**:
  - UTF-8 bytes read with a single-byte decoder
- **Output** (UTF-8 bytes: `á` C3 A1, `é` C3 A9, `í` C3 AD, `ó` C3 B3, `ö` C3 B6, `ő` C5 91, `ú` C3 BA, `ü` C3 BC, `ű` C5 B1):

| letter | as Windows-1252 | as Windows-1250 | as ISO-8859-2 | as CP437 |
|---|---|---|---|---|
| á | `Ã¡` | `Ăˇ` | `ĂĄ` | `├í` |
| é | `Ã©` | `Ă©` | `ĂŠ` | `├⌐` |
| í | `Ã` + U+00AD | `Ă` + U+00AD | `Ă` + U+00AD | `├¡` |
| ó | `Ã³` | `Ăł` | `Ăł` | `├│` |
| ö | `Ã¶` | `Ă¶` | `Ăś` | `├╢` |
| ő | `Å‘` | `Ĺ‘` | `Ĺ` + U+0091 | `┼æ` |
| ú | `Ãº` | `Ăş` | `Ăş` | `├║` |
| ü | `Ã¼` | `ĂĽ` | `Ăź` | `├╝` |
| ű | `Å±` | `Ĺ±` | `Ĺą` | `┼▒` |

  ISO-8859-1 equals the Windows-1252 column except that 80-9F become C1 controls (`ő` -> `Å` + U+0091). Uppercase in Windows-1252: `Á` -> `Ã` + U+0081, `É` -> `Ã‰`, `Í` -> `Ã` + U+008D, `Ó` -> `Ã“`, `Ö` -> `Ã–`, `Ő` -> `Å` + U+0090, `Ú` -> `Ãš`, `Ü` -> `Ãœ`, `Ű` -> `Å°` (WHATWG maps undefined 81 8D 8F 90 9D to C1 controls). Double UTF-8 (1252): `ő` -> `Ã…â€˜`, `ű` -> `Ã…Â±`.
- **Examples**: `előtt` -> `elÅ‘tt`; `hétfő` -> `hÃ©tfÅ‘`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-ENC-011`, `REP` (undo by encoding with the wrong codec and decoding as UTF-8)
- **Sources**: [WHATWG-ENC]; [UNI-MAP] (tables computed 2026-09-15)
- **Validation**: goldens
- **Confidence**: consensus

### WS-ENC-007 DOS code pages
- **Statement**: Hungarian DOS text is CP852 (IBM Latin-2) or the unofficial CWI-2; both share the lowercase `á é í ó ö ú ü` bytes with CP437, but CWI-2 shown as CP437 turns the remaining letters into look-alikes (`ô û`), while CP852 shown as CP437 turns them into unrelated symbols (`ï √ ╡`).
- **Conditions**:
  - files from DOS-era software, BBS archives, legacy databases
- **Output**:
  - CWI-2 read as CP437: `ő -> ô`, `ű -> û`, `Ő -> º`, `Ű -> ÿ`, `Í -> ì`, `Á -> Å`, `Ó -> ò`, `Ú -> ù`; `á é í ó ö ú ü É Ö Ü` unchanged
  - CP852 read as CP437: `ő -> ï`, `ű -> √`, `Á -> ╡`, `Í -> ╓`, `Ó -> α`, `Ő -> è`, `Ú -> Θ`, `Ű -> δ`; others unchanged
  - CP852 read as Windows-1250: `á` -> U+00A0, `é -> ‚`, `í -> ˇ`, `ó -> ˘`, `ö -> ”`, `ő -> ‹`, `ú -> Ł`, `ü` -> undefined byte 81, `ű -> ű`
  - Windows-1250 read as CP852: `á -> ß`, `é -> Ú`, `í -> Ý`, `ó -> ˇ`, `ö -> ÷`, `ő -> §`, `ú -> ˙`, `ü -> Ř`, `ű -> ű`
- **Examples**: `ű` (byte FB) survives between CP852 and Windows-1250 in both directions
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-ENC-011`
- **Sources**: [UNI-MAP] (CP852.TXT); [ENWIKI-CWI2]; [GLIBC-ICONV]
- **Validation**: goldens
- **Confidence**: consensus

### WS-ENC-008 Mac Central European
- **Statement**: Classic Mac OS Hungarian text uses Mac Central European; decoding it as Windows-1250 or the reverse damages every accented letter.
- **Conditions**:
  - pre-OS X Mac files
- **Output**: MacCE read as Windows-1250: `á -> ‡`, `é -> Ž`, `í -> ’`, `ó -> —`, `ö -> š`, `ő -> Î`, `ú -> ś`, `ü -> ź`, `ű -> ő`. Windows-1250 read as MacCE: `á -> Š`, `é -> ť`, `í -> Ū`, `ó -> ů`, `ö -> Ų`, `ő -> ű`, `ú -> ķ`, `ü -> Ł`, `ű -> Ż`
- **Examples**: the wrong decoding turns `ű` into `ő` (MacCE -> 1250) and `ő` into `ű` (1250 -> MacCE), which a lexicon check alone may not catch
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-ENC-011`
- **Sources**: [UNI-MAP] (APPLE/CENTEURO.TXT, CP1250.TXT)
- **Validation**: goldens
- **Confidence**: consensus

### WS-ENC-009 ISO 646-HU (7-bit)
- **Statement**: The 7-bit national variant ISO 646-HU (MSZ 7795-3) puts some Hungarian letters at ASCII punctuation positions and lacks `í ó ú ő ű`.
- **Conditions**:
  - old 7-bit data
- **Output**: `$ -> ¤`, `@ -> Á`, `[ -> É`, `\ -> Ö`, `] -> Ü`, backquote -> `á`, `{ -> é`, `| -> ö`, `} -> ü`, `~` -> U+02DD double acute accent; viewed as ASCII, `ö` shows as `|`, `é` as `{`
- **Examples**: `k|nyv` (ASCII view) = `könyv`; `k{p` = `kép`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-006`
- **Sources**: [GLIBC-ICONV] (charset `ISO646-HU`, verified)
- **Validation**: goldens
- **Confidence**: consensus

### WS-ENC-010 HTML and XML character references
- **Statement**: Web text encodes Hungarian letters with numeric or named references; legacy pages used wrong named entities for the double-acute letters.
- **Conditions**:
  - HTML or XML input
- **Output**: correct: `&odblac;`, `&#337;`, `&#x151;` = `ő`; `&Odblac;`, `&#336;` = `Ő`; `&udblac;`, `&#369;` = `ű`; `&Udblac;`, `&#368;` = `Ű`. Wrong substitutes: `&otilde;` (`õ`), `&ocirc;` (`ô`), `&ucirc;` (`û`) -> look-alikes (`WS-NORM-004`). Decode references before `WS-NORM-001`.
- **Examples**: `el&otilde;tt` -> `elõtt` -> repaired `előtt`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-004`
- **Sources**: [HTML5-ENT]
- **Validation**: goldens
- **Confidence**: consensus

### WS-ENC-011 Encoding detection for Hungarian bytes
- **Statement**: Detect the source encoding by validity checks first, then by decoding candidates and scoring Hungarian letters against mojibake signatures.
- **Conditions**:
  - bytes without a trustworthy declaration, or a declaration contradicted by content
- **Output** (procedure):
  1. BOM `EF BB BF` -> UTF-8; `FF FE` or `FE FF` -> UTF-16
  2. valid UTF-8 with bytes >= 80 -> UTF-8 candidate; count `WS-ENC-006` signatures (`Ã` or `Ă` followed by a character from the second column, `Å‘`, `Å±`, `Ĺ‘`, `Ĺ±`); if signatures outnumber genuine Hungarian letters, undo by encoding as Windows-1252 (or 1250) and decoding as UTF-8; repeat while signatures remain
  3. invalid UTF-8: bytes F5-FF never occur in UTF-8, so `F5` (`ő`) and `FB` (`ű`) of Latin-2/1250 are strong single-byte evidence
  4. decode with each candidate (Windows-1250, ISO-8859-2, CP852, CWI-2, MacCE, Windows-1252) and score: Hungarian accented letters inside words weighted by expected share, minus characters implausible in Hungarian words (box drawing, C1 controls, `Ă Ĺ Ł Ř ť Ť ˇ ˘ ‡ Ž`)
  5. take the best score; separate Windows-1250 from ISO-8859-2 by `WS-ENC-012`
  6. if the decoded text still shows `õ û ô` inside Hungarian words, the damage happened before storage: keep the encoding decision and repair letters in `REP`
- **Examples**: bytes `65 6C F5 74 74`: invalid UTF-8; Windows-1250 gives `előtt`, Windows-1252 gives `elõtt`; the 1250 reading wins on lexicon and letter score
- **Exceptions**: strings with no bytes >= 80 are ASCII in all candidates.
- **Predictability**: `rule` (steps 1-3); `tendency` (scoring)
- **Lexical input**: letter share profile; lexicon for ties
- **Interactions**: `WS-ENC-002` .. `WS-ENC-012`, `REP`
- **Sources**: [WHATWG-ENC]; [UNI-MAP]; letter shares from `[ATT]` types weighted by 2^(bucket/2) (approximate, recomputed 2026-09-15): `é` 3.6%, `á` 3.5%, `ö` 1.1%, `ó` 1.0%, `ő` 0.9%, `í` 0.6%, `ü` 0.6%, `ú` 0.3%, `ű` 0.2% of all letters
- **Validation**: goldens (corpus sample round-tripped through every encoding and every wrong decoder)
- **Confidence**: consensus (procedure); tendency (weights)

### WS-ENC-012 ISO-8859-2 versus Windows-1250
- **Statement**: ISO-8859-2 and Windows-1250 encode all Hungarian letters identically; they differ in 80-9F and in some A0-BF positions, which Hungarian punctuation usually reveals.
- **Conditions**:
  - single-byte candidate from `WS-ENC-011`
- **Output**:
  - any of 84, 94, 96, 97, 85 present (`„ ” – — …` in 1250, C1 controls in 8859-2) -> Windows-1250
  - AB or BB around words (`« »` in 1250, `Ť ť` in 8859-2) -> Windows-1250
  - A1, B1, A6, B6, AC, BC used as letters in Polish or Czech words (`Ą ą Ś ś Ź ź` in 8859-2) -> ISO-8859-2
  - no discriminating byte -> decode as Windows-1250 (identical result for Hungarian letters)
- **Examples**: `84 E9 76 94` -> Windows-1250 `„év”`; in ISO-8859-2 it would contain two C1 controls
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-ENC-011`
- **Sources**: [UNI-MAP] (8859-2.TXT, CP1250.TXT)
- **Validation**: goldens
- **Confidence**: consensus

### WS-ENC-013 Test strings
- **Statement**: Encoding, normalisation and case tests use strings containing every Hungarian accented letter in both cases, plus Hungarian punctuation.
- **Conditions**:
  - goldens for `WS-ENC`, `WS-NORM`, `WS-CASE`
- **Output**: `árvíztűrő tükörfúrógép`; `ÁRVÍZTŰRŐ TÜKÖRFÚRÓGÉP`; with punctuation: `„Árvíztűrő tükörfúrógép” – »ő«`
- **Examples**: see Output
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: all `WS-ENC`
- **Sources**: [UCD] (letters); conventional test phrase
- **Validation**: goldens
- **Confidence**: consensus

## Open questions
1. Pronunciation of vowel letter names before suffixes (`a-t`, `e-vel`) is not codified: a short, a lengthened same-quality, or an `á`/`é` vowel are all heard; writing is unaffected (`WS-ALPH-009`). Needed for TTS and for `PH`.
2. Possessive and some case forms of consonant-final letter names (`l-je` or `l-e`, `x-e` or `x-je`) and the accusative of `f m ny r` are not exemplified in AkH11/AkH12 (`WS-ALPH-007`, `WS-ALPH-010`).
3. Letter-by-letter readings of `W Q X Y` inside acronyms (`vé` or `dupla vé`) vary; only `WC` -> `vécé` is lexicalised (`WS-ALPH-012`).
4. Retired: collation is out of scope.
5. Retired: collation is out of scope.
6. Retired: collation is out of scope.
7. Retired: collation is out of scope.
8. History: simplified doubling in school spelling 1903 and Academy rules 1922, and single-letter status of `dz dzs` in hyphenation from AkH11 (1984), come from secondary sources (`WS-DIG-010`, `WS-LET-002`); verify against AkH 1922 and AkH10 (1954) texts.
9. No attested native `s+z`, `d+zs`, `z+zs`, `l+ly`, `t+ty` junctions were found in `[ATT]`; confirm with a morphological oracle (`WS-DIG-006`).
10. Heuristic precision figures for `nny` and `ssz` are from 100-type samples (`WS-DIG-008`); `ggy`, `zsz`, `ccs` were classified on full type lists by hand; measure all on the full type list with hunmorph-foma.
11. The spelling alphabet decree (IHM 6/2006) text was not retrieved directly (`WS-ALPH-013`).
12. No normative source was found for no-break spaces after one-letter words at line end (`WS-CHAR-008`).
13. AkH12 §79 contains `ssz-el` where `ssz-szel` is expected by §80; treated as a typographical slip.
14. Sound values of archaic groups marked S in `WS-LET-007` (`aá`, `uu`, `ay`, `ey`) rest on a popular-science article and an unsourced list.

## Sources used
All keys are in [sources.md](sources.md). Reliability notes for this chapter:
- `HUWIKI-ARCH`: article marked unsourced; used only for entries marked S.
- `IHM2006`: cited via `HUWIKI-BETUZ`.
- `ICU78`, `GLIBC244`, `GLIBC-ICONV`: behaviour observed in local runs, not documentation.

## Terms
See [glossary.md](glossary.md).
