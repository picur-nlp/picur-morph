# 02 Phonology (`PH`)

Scope: the segment inventory and natural classes consumed by morphology (vowels, consonants, letter-to-segment mapping), syllable structure and the phonotactic predicates that decide linking vowels and suffix shapes (coda licensing of `-t`, final clusters, verb-root shapes, stem-final vowels, hiatus), vowel harmony in full (backness, rounding, neutral vowels, antiharmony, the zones of variation with lexical statistics and MaxEnt weights, consonant effects and their contested status, compounds, suffix-level transparency and opacity, the harmony automaton), vowel quantity alternations at the phonological level, assimilations relevant to spelling and allomorphy (and which of them are written), vowel-zero alternations, v-stems and sz/d/v stems at the phonological level, and the pronunciation of spellings needed for suffixing (native core, archaic letters in names, silent letters, foreign words, letters and numerals as spoken). Paradigm construction is in `NOM` and `VRB`; spelling of suffix attachment (hyphens, three-letter rule, doubled digraphs as orthography) in `ORT`; letters, digraphs and collation in `WS`; numeral spell-out in `PND`; derivation and compound structure in `DER`.

## Area index
| Area | Topic |
|---|---|
| `VOW` | vowel inventory, letter mapping, backness, rounding, height, length pairs, neutral vowels |
| `CONS` | consonant inventory, letter mapping, natural classes, geminates, `h`, `j`, `v` |
| `SYL` | syllable, syllabification, onsets, codas, coda licensing of `-t`, final clusters, verb-root phonotactics, stem-final vowels, hiatus, sequence constraints |
| `HARM` | vowel harmony: domain, archiphonemes, categorical harmony, rounding, antiharmony, zones of variation, statistics, MaxEnt model, consonant effects, suffix transparency and opacity, compounds, loans, automaton |
| `LEN` | vowel length alternations: final `a`/`e` lengthening, final mid vowels, high vowels, shortening stems, lexical and Latinate shortening, suffix vowel length, consonant length alternations in suffixes |
| `ASSIM` | assimilations: written vs unwritten, `v`-assimilation, `az`/`ez`, `j`-assimilation, `t`-final imperatives, palatalisation, voicing, nasal place, sibilant and affricate coalescence, degemination, `l`-assimilation, hiatus `j` |
| `ZERO` | vowel-zero alternations: epenthetic stems, metathesis stems, `-alom/-elem`, stem-final vowel loss, place names |
| `VSTEM` | v-stems, u/ü-to-v stems, ú-deleting stems, sz/d/v verbs (phonological level) |
| `G2P` | pronunciation of spellings for suffixing: native core, digraph parsing, traditional spellings, archaic letters, final `y`, foreign words, letters, acronyms, numerals, symbols |

## Conventions used in this chapter
- Orthographic forms are in backticks; phonological forms in slashes `/.../`, phonetic in brackets `[...]`, IPA.
- Vowel categories (a property of a single vowel letter or segment): `B` back (`a á o ó u ú`), `R` front rounded (`ö ő ü ű`), `N` neutral = front unrounded (`i í e é`). Stem harmony classes (output of `PH-HARM`, README notation): `B` back, `F` front unrounded, `R` front rounded. Stem shapes are written as vowel-category strings from left to right, e.g. `BN` = a back vowel followed by one neutral vowel as the last two vowels. When quoting Hayes and Londe, their `F` (front rounded) is rendered here as `R`.
- "Vowel-final" and "consonant-final" always refer to the pronounced stem end as computed by `PH-G2P`; `C` and `V` in shape formulas are segments (a digraph is one `C`), `VV` is a long vowel.
- Suffix classes: `synthetic` suffixes interact with the stem phonologically (linking vowels, epenthetic vowel drop, shortening, v-insertion); `analytic` suffixes (`-ig`, `-ért`, `-é`, `-ul/-ül` essive, all consonant-initial case suffixes, `-ként`, ...) behave like a separate domain for stem alternations but still harmonise; `super-analytic` suffixes additionally block final `a`/`e` lengthening (`PH-LEN-002`). Class membership per suffix is lexical and stored with the suffix (`NOM-MODEL`, `VRB-MODEL`).
- Corpus buckets `[ATT n]` are log-frequency buckets in [`crates/picur-morph-lexicon/data/frequency.ron`](../../crates/picur-morph-lexicon/data/frequency.ron) (higher = more frequent, `-` = unattested in the top 250k list). They are attestation evidence, not norms.
- Lexical statistics quoted from Hayes and Londe (2006) are type counts from May 2004 Google hit counts of `-nak`/`-nek` datives (stems with at least 10 hits; the paper's database has 10,974 stem types, the published revised data file `HungarianFromGoogleRevised.txt` has 8915); "backness index" = mean share of `-nak` per stem type, vacillators counted fractionally. Per-stem shares quoted as "(0.93)" are recomputed from the revised data file.

---

## VOW: vowels

### PH-VOW-001 Vowel inventory and vowel letters
- **Statement**: Standard Hungarian has 14 vowel phonemes, seven short and seven long, each written by exactly one letter; length is written by the acute or double acute accent.
- **Conditions**: native and hungarianised spelling; foreign spellings go through `PH-G2P`.
- **Output**:

  | Letter | IPA | Length | Category | Rounded | Phonetic height | Phonological height (alternation) | Pair |
  |---|---|---|---|---|---|---|---|
  | `a` | /ɒ/ | short | B | slightly | low | low | `á` |
  | `á` | /aː/ | long | B | no | low | low | `a` |
  | `o` | /o/ | short | B | yes | mid | mid | `ó` |
  | `ó` | /oː/ | long | B | yes | mid | mid | `o` |
  | `u` | /u/ | short | B | yes | high | high | `ú` |
  | `ú` | /uː/ | long | B | yes | high | high | `u` |
  | `ö` | /ø/ | short | R | yes | mid | mid | `ő` |
  | `ő` | /øː/ | long | R | yes | mid | mid | `ö` |
  | `ü` | /y/ | short | R | yes | high | high | `ű` |
  | `ű` | /yː/ | long | R | yes | high | high | `ü` |
  | `e` | /ɛ/ | short | N | no | low (low-mid) | low and mid (alternates with `a` and with `o`/`ö`) | `é` |
  | `é` | /eː/ | long | N | no | mid (high-mid) | low (alternates with `á`) | `e` |
  | `i` | /i/ | short | N | no | high | high | `í` |
  | `í` | /iː/ | long | N | no | high | high | `i` |
- **Examples**: `hat` /hɒt/, `hát` /haːt/; `kemény` /kɛmeːɲ/; `tűz` /tyːz/; minimal pairs `bal`-`bál`, `kemény`-`kémény`, `öt`-`őt`, `áru`-`árú`.
- **Exceptions**: `y` stands for /i/ only in names and some foreign words (`PH-G2P-005`); non-phonemic vowel qualities in `PH-VOW-005`.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-VOW-002` (harmony categories), `PH-VOW-003` (quality differences), `PH-G2P-001`.
- **Sources**: [AkH12 §4, §6]; [KVF98 §3.1.2.2.1 pp. 384-385]; [RST12 §2 p. 492]; [SMNy3 Rebrus §13.2.1 pp. 678-682]; [T04 App. A p. 231].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-VOW-002 Harmony categories of vowels
- **Statement**: For vowel harmony every vowel belongs to exactly one category: back `B` = `a á o ó u ú`, front rounded `R` = `ö ő ü ű`, neutral `N` = `i í e é`; long and short members of a pair always share the category.
- **Conditions**: vowel segments after `PH-G2P`; `y` = /i/ is `N`.
- **Output**: category per vowel; stem harmony computed by `PH-HARM`.
- **Examples**: `ablak` = `BB`; `sofőr` = `BR`; `glükóz` = `RB`; `papír` = `BN`; `kert` = `N`; `szintézis` = `NNN`; `tükör` = `RR`.
- **Exceptions**: neutrality is gradient (`i í` > `é` > `e` in transparency, `PH-HARM-012`); some authors deny that `e` is neutral [RST12 fn. 6].
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-HARM-003`..`PH-HARM-021`.
- **Sources**: [KVF98 (73) p. 420]; [HZSL09 (5) p. 9]; [RST12 §3.2 p. 494].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-VOW-003 Length pairs and the quality of a/á and e/é
- **Statement**: The pairs `a`/`á` and `e`/`é` differ in quality as well as length (/ɒ/-/aː/, /ɛ/-/eː/); the other pairs differ essentially in length only (long `ó ő` slightly closer). All length alternations (`PH-LEN`) pair the letters of the same row regardless of quality: `a`<->`á`, `e`<->`é`.
- **Conditions**: any length alternation (lengthening, shortening).
- **Output**: `a`->`á`, `e`->`é`, `o`->`ó`, `ö`->`ő`, `i`->`í`, `u`->`ú`, `ü`->`ű` and reverse.
- **Examples**: `alma`->`almát`; `kefe`->`kefét`; `kéz`->`kezet`; `nyár`->`nyarak`; `víz`->`vizet`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-LEN-001`, `PH-LEN-007`, `PH-HARM-016` (lengthened `é` is more transparent than `e`).
- **Sources**: [RST12 §2 p. 492, §4 pp. 497-498]; [T04 §3.4.2 pp. 119-121]; [KVF98 p. 385].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-VOW-004 Height classes used by rules
- **Statement**: Rules refer to three height sets that are not identical to phonetic height: (a) for harmony gradience, `i í` = high, `é` = mid, `e` = low; (b) for stem-final quantity, short `a e` = low (lengthen before suffixes), short `o ö` = mid (never stem-final in content words), `i u ü` (and long counterparts) = high (never lengthen); (c) for three-way suffix alternation, `e` counts as mid (`o`/`e`/`ö`) and as low (`a`/`e`), `é` alternates with `á`.
- **Conditions**: consumed by `PH-HARM-012`, `PH-HARM-013`, `PH-LEN-001`, `PH-LEN-005`, `PH-LEN-006`, `PH-HARM-002`.
- **Output**: height label per vowel per rule family.
- **Examples**: harmony: `papír` (Bí, high) mostly back, `poén` (Bé, mid) back, `fotel` (Be, low) mostly front; quantity: `alma`->`almát` but `kocsi`->`kocsit`; alternation: `-hoz/-hez/-höz`, `-nak/-nek`, `-nál/-nél`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: see Conditions.
- **Sources**: [HZSL09 §5.1, fn. 14 p. 17]; [HL06 Table II p. 67]; [T04 §3.4.1 pp. 105-114]; [RST12 §3.1 p. 493].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-VOW-005 Non-phonemic and marginal vowel qualities
- **Statement**: Several vowel qualities occur in speech but have no letter and never change suffix selection in the standard: dialectal closed [e] (`ë`), long [ɛː] and [ɔː] from compensatory lengthening, short [a] in foreign words and names, and [w] in `au`.
- **Conditions**:
  - `ë`: most regional accents distinguish closed [e] from open [ɛ]; the standard (Budapest) accent does not; spelling always `e` [AkH12 §89].
  - [ɛː], [ɔː]: from deletion of a following `l r j n` (`arra` [ɒːrɒ], `erre` [ɛːrɛ]) or marked readings.
  - [a]: short open unrounded vowel in foreign words and personal names, alternating with [ɒ] or [aː].
  - [w]: second half of `au` in `autó`, `augusztus`, `bauxit`, `kaucsuk`, `mauzóleum`, `tautológia`, `trauma`, `auto-` [KVF98 p. 385]; phonologically /u/.
- **Output**: no effect on harmony or allomorph choice; for G2P the vowel is mapped to the nearest phoneme (`ë`->/ɛ/, [a]->/ɒ/, [w]->/u/).
- **Examples**: [ëmbër] written `ember`, dative `embernek`; `autóval`.
- **Exceptions**: regional three-way `o`/`ë`/`ö` and two-way `o`/`ö` suffix systems (`PH-HARM-005` Exceptions) are dialectal, not generated.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-HARM-005`, `PH-LEN-014`.
- **Sources**: [AkH12 §89]; [SMNy3 Rebrus §13.2.1.1 p. 678, §13.2.3.3 pp. 691-692]; [KVF98 pp. 385, 446-447].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

---

## CONS: consonants

### PH-CONS-001 Consonant inventory and consonant letters
- **Statement**: Standard Hungarian has 25 consonant phonemes; each is written by one letter or one digraph/trigraph; length (gemination) is written by doubling (`PH-CONS-004`).
- **Conditions**: native and hungarianised spelling.
- **Output**:

  | Letter | IPA | Place | Manner | Voice | Voice pair | Notes |
  |---|---|---|---|---|---|---|
  | `p` | /p/ | bilabial | stop | - | `b` | |
  | `b` | /b/ | bilabial | stop | + | `p` | |
  | `t` | /t/ | dental | stop | - | `d` | |
  | `d` | /d/ | dental | stop | + | `t` | |
  | `ty` | /c/ | palatal | stop | - | `gy` | word-initially only in `tyúk` and its compounds, the interjection `tyű`, and the place name `Tyukod` / `tyukodi` (`PH-CONS-001` Exceptions) |
  | `gy` | /ɟ/ | palatal | stop | + | `ty` | |
  | `k` | /k/ | velar | stop | - | `g` | |
  | `g` | /ɡ/ | velar | stop | + | `k` | |
  | `c` | /t͡s/ | alveolar | affricate | - | `dz` | |
  | `dz` | /d͡z/ | alveolar | affricate | + | `c` | intervocalically long [d͡zː] (`bodza`, `edz`) |
  | `cs` | /t͡ʃ/ | postalveolar | affricate | - | `dzs` | |
  | `dzs` | /d͡ʒ/ | postalveolar | affricate | + | `cs` | intervocalically usually long (`bridzs`, `maharadzsa`) |
  | `f` | /f/ | labiodental | fricative | - | `v` | |
  | `v` | /v/ | labiodental | fricative | + | `f` | asymmetric (`PH-CONS-006`) |
  | `sz` | /s/ | alveolar | fricative | - | `z` | |
  | `z` | /z/ | alveolar | fricative | + | `sz` | |
  | `s` | /ʃ/ | postalveolar | fricative | - | `zs` | |
  | `zs` | /ʒ/ | postalveolar | fricative | + | `s` | |
  | `h` | /h/ | glottal (velar in coda) | fricative | - | none | allophones `PH-CONS-005` |
  | `m` | /m/ | bilabial | nasal | + | none | [ɱ] before `f v` |
  | `n` | /n/ | dental | nasal | + | none | place assimilation `PH-ASSIM-010` |
  | `ny` | /ɲ/ | palatal | nasal | + | none | |
  | `l` | /l/ | dental | lateral | + | none | |
  | `r` | /r/ | dental | trill | + | none | |
  | `j`, `ly` | /j/ | palatal | approximant | + | none | two spellings of one phoneme (`PH-CONS-008`) |

  Letters outside the Hungarian alphabet in hungarianised or name spellings: `q` = /k/ (`qu` = /kv/), `w` = /v/, `x` = /ks/ (sometimes /ɡz/), `ch` = /h/ in retained common words (`technika`, `pech`), `y` = /i/ (`PH-G2P-004`..`PH-G2P-007`).
- **Examples**: `hagy` /hɒɟ/; `ponty` /poɲc/; `dzsem` /d͡ʒɛm/; `lyuk` /juk/; `fix` /fiks/.
- **Exceptions**: the status of `dz` as a phoneme vs /d/+/z/ is debated [KVF98 p. 383]; for implementation it is one segment with long realisation between vowels. Word-initial `ty`: over 200 corpus types, the only native Hungarian types above 100 tokens are `tyúk` and its compounds, the interjection `tyű` (1,683) and the place name `Tyukod` / `tyukodi`; `tyüh` is below the threshold. KVF98's claim holds [WC2].
- **Predictability**: `rule`
- **Lexical input**: `none`; the screened inventory is `initial-ty-types.tsv` (200 types) in `local/lists/initial-ty-types.tsv` - working data, not yet read by the build.
- **Interactions**: `PH-CONS-002`, `PH-CONS-003`, `PH-G2P-001`, `WS-DIG`.
- **Sources**: [AkH12 §7, §10, §87, §88, §204]; [KVF98 §3.1.2.1 pp. 382-384]; [T04 App. A p. 230]; [WC2]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `goldens`, `attestation` (initial-`ty` type inventory above 100 tokens).
- **Confidence**: `consensus`; the initial-`ty` inventory `measured`.

### PH-CONS-002 Parsing letters into consonant segments
- **Statement**: Within one morpheme a letter string is parsed into segments greedily, longest match first (`dzs` > `dz`, `cs`, `gy`, `ly`, `ny`, `sz`, `ty`, `zs` > single letters); a doubled digraph is written with only its first letter doubled (`ssz`, `ggy`, `nny`, `lly`, `tty`, `ccs`, `zzs`, `ddz`, `ddzs`) and parses as one long segment; digraphs never span a morpheme boundary.
- **Conditions**:
  - Morpheme boundaries must be known (from the lexicon/analyser) to avoid false digraphs: `köz+ség` (`zs` = z+s), `egész+ség` (`sz`+`s`), `igaz+ság`, `víz+szint` (`z`+`sz`), `ház+szám`, `ügy+szám` (`gy`+`sz`), `any+ja` (`ny`+`j`).
  - Full doubled digraphs appear at compound boundaries: `jegy+gyűrű`, `kulcs+csomó`, `dísz+szemle`, `ész+szerű`.
  - A sequence like `ssz` inside a morpheme is long /sː/ (`asszony`); across a suffix boundary it is the simplified spelling of `sz`+`sz` (`mész`+`szel` -> `mésszel`) or the plain sequence `s`+`sz` (`kis`+`szék` -> `kisszék`); resolve by morphology.
- **Output**: segment sequence with length marks.
- **Examples**: `meggy` = /mɛɟː/; `asszonnyal` = `asszony`+`nyal` = /ɒsːoɲːɒl/; `község` = /køz/+/ʃeːɡ/ -> [køʃːeːɡ] by assimilation; `vízszint` [viːsːint].
- **Exceptions**: without morphological information the parser defaults to the greedy digraph reading; the wrong reading affects only pronunciation, not suffixing, unless the digraph is stem-final (then it changes `-val` spelling and harmony-irrelevant consonant class).
- **Predictability**: `rule` (given morphology)
- **Lexical input**: morpheme boundaries.
- **Interactions**: `WS-DIG`, `PH-G2P-002`, `PH-ASSIM-002`.
- **Sources**: [AkH12 §7 b), §92]; [ESPEAK-HU `hu_list` boundary-digraph lists (study only)].
- **Validation**: `goldens`, `oracle:hunmorph-foma`.
- **Confidence**: `consensus`.

### PH-CONS-003 Natural classes of consonants
- **Statement**: Morphophonological rules refer to the following consonant classes; membership is fixed.
- **Conditions**: segments from `PH-CONS-001`.
- **Output**:

  | Class | Members (letters) | Used by |
  |---|---|---|
  | obstruent | `p b t d ty gy k g c dz cs dzs f v sz z s zs h` | voicing `PH-ASSIM-009` |
  | voiceless obstruent | `p t ty k c cs f sz s h` | |
  | voiced obstruent | `b d gy g dz dzs v z zs` | |
  | sonorant | `m n ny l r j` (incl. `ly`) | coda `-t`, past `-t` |
  | nasal | `m n ny` | |
  | liquid | `l r` | |
  | approximant | `j` (`ly`) | |
  | stop | `p b t d ty gy k g` | |
  | affricate | `c dz cs dzs` | |
  | fricative | `f v sz z s zs h` | |
  | sibilant / strident | `sz z s zs c dz cs dzs` | 2sg `-Vl`, `j`-assimilation (only fricatives `s sz z` and `dz`), sequence constraint `PH-SYL-015` |
  | sibilant (HZSL09) | `sz z s zs c cs dzs` | harmony consonant effect `PH-HARM-014` |
  | coronal sonorant (HZSL09) | `n ny l r` | `PH-HARM-014` |
  | bilabial noncontinuant (HZSL09) | `p b m` | `PH-HARM-014` |
  | palatal | `ty gy ny j` (`ly`) | 3sg possessive `-a` (`NOM-POSS`), palatal+`j` `PH-ASSIM-007` |
  | dental stop/nasal | `t d n` | palatalisation before `j` `PH-ASSIM-007` |
  | labial | `p b m f v` | |
  | velar | `k g` (and [x], [ŋ]) | |
  | coronal | `t d c dz sz z s zs cs dzs ty gy n ny l r j` | coda clusters `PH-SYL-007` |
- **Examples**: `mos`->`mossa` (fricative sibilant + `j`); `hagy`->`hagyja` (palatal, `j` not written as assimilated); `néz`->`nézel` (strident, 2sg `-el`).
- **Exceptions**: `v` behaves as a sonorant in onsets and as an obstruent in codas; `h` is an obstruent that triggers but does not undergo voicing (`PH-CONS-006`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: all rules cited in the table.
- **Sources**: [T04 §3.2.4.2 pp. 58-65, App. A]; [HZSL09 (18) p. 20]; [KVF98 pp. 440-442]; [AkH12 §50].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-CONS-004 Consonant length (geminates)
- **Statement**: Consonant length is contrastive and written by doubling the (first letter of the) consonant; morpheme-internal geminates occur only between vowels or word-finally, never word-initially, and are pronounced short next to another consonant.
- **Conditions**:
  - Written doubling: single letters `bb cc dd ff gg hh jj kk ll mm nn pp rr ss tt vv zz`; digraphs per `PH-CONS-002`.
  - `dz` and `dzs` are pronounced long between vowels but written single inside a morpheme (`bodza`, `edz`, `bridzs`); length from suffixation is written (`eddzük`, `briddzsel`).
  - Geminate next to a consonant is pronounced short (`PH-ASSIM-012`), spelling unchanged.
- **Output**: long consonant segment.
- **Examples**: `hal`/`hall`, `len`/`lenn`, `megy`/`meggy`, `kasza`/`kassza`, `hason`/`hasson`, `fejjel`.
- **Exceptions**: spelling of length is fixed by morphology where pronunciation varies (`egy` one `gy` in all forms except `eggyel`, `eggyé`; `kisebb`, `erősek` with one `s` although often long) [AkH12 §69-§73].
- **Predictability**: `rule`
- **Lexical input**: lexical length of stem consonants.
- **Interactions**: `PH-ASSIM-002`, `PH-ASSIM-012`, `ORT-ASSIM-017` (three identical letters), `ORT-LEN-010` (`dz`, `dzs`).
- **Sources**: [AkH12 §7 b), §9, §59, §69-§73, §87]; [KVF98 §3.4.1.3.1.5 p. 448].
- **Validation**: `goldens`.
- **Confidence**: `normative` (spelling), `consensus` (pronunciation).

### PH-CONS-005 Realisation of h and h-final stems
- **Statement**: `h` is [h] before a vowel (voiced [ɦ] between sonorants/vowels), [x] (after front vowels also [ç]) in the coda; a small class of stems (`cseh` type) has a silent `h` in the coda and counts as vowel-final there, while ordinary `h`-final stems (`doh` type) are consonant-final.
- **Conditions**:
  - `doh` type (open class; all new words and acronyms): coda `h` = [x]; consonant-final for suffixing: `dohhal`, `Allahhal`, `sahhal`, `pechhel`, `jacht` [jɒxt].
  - `cseh` type: `cseh` for all speakers (silent: `csehtől` [t͡ʃɛtøːl], `csehvel`), vowel-final before consonant-initial suffixes, [h] before vowel-initial suffixes (`csehek`, `csehes`).
  - `pléh`: **consonant-final in current usage**, i.e. it belongs with the `doh` type for the instrumental, with the AkH12 `pléhvé` form kept as a variant. Measured [WC2]: `pléhvel` 71 : `plével` 0, `pléhet` 55 : `plét` 6. The norm (`pléhvé` [AkH12 §41]) is not the instrumental and does not conflict; T04's fully vowel-final report ([pleːk] plural) and SMNy3 p. 790's reported speaker variation are not visible in a 8.57e9-token corpus.
  - Variable items: spelling-variable per the norm `céh`, `düh`, `juh`, `méh`, `rüh` [AkH12 §74, §82 e)]: `-val/-vá` both spellings allowed (`dühvel`~`dühhel`, `méhvé`~`méhhé`); pronunciation-variable only (T04): `éh-` (bound), `oláh` (vowel-final for most speakers, but `oláhok`); spelling keeps `h` everywhere.
  - Geminate `hh` is always [xː] (`ahhoz` optionally [ɒhoz]).
- **Output**: pronounced end = vowel (cseh type, in coda) or consonant /h/ (all others).
- **Examples**: `csehvel`, `pléhvé` [AkH12 §41] but `pléhvel` 71 : `plével` 0, `pléhet` 55 : `plét` 6 in use [WC2]; `dohhal`, `Allahhal` [AkH12 §41]; `dühvel` [ATT 4], `dühhel` [ATT -].
- **Exceptions**: archaic name spellings with silent `h` (`Végh`, `Imreh`) are in `PH-G2P-004`.
- **Predictability**: `lexical` (silent in writing: `cseh`; `pléh` measured consonant-final; spelling-variable: 5 items; pronunciation-variable: `éh-`, `oláh`; open doh class is the default)
- **Lexical input**: stem flag `h_silent` (`always` | `variable` | `never`, default `never`).
- **Interactions**: `PH-ASSIM-002`, `NOM-CASE`, `ORT-ASSIM-005`, `ORT-LEN-033`.
- **Sources**: [AkH12 §41, §74, §82 e)]; [T04 §4.2.2 pp. 219-221 incl. fn. 117]; [KVF98 p. 383, §3.4.4.1.2 p. 450]; [SMNy3 Rebrus pp. 789-790]; [WC2]; [MTA-HELYES]; [ATT evidence/corpus-variants.md 2026-09-16]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `goldens`, `attestation` (both instrumental and accusative probed for `pléh`, n = 71 and 61, both above the reliability floor).
- **Confidence**: `normative` for spelling; `consensus` for the class; `measured` for `pléh` (consonant-final, source conflict arbitrated by usage); membership of the remaining variable items `contested` (speaker variation).

### PH-CONS-006 Asymmetric behaviour of v and h
- **Statement**: `v` undergoes voicing assimilation (devoices before voiceless obstruents) but does not trigger it and may follow voiceless obstruents inside morphemes; `h` triggers devoicing but is not affected; in coda clusters `v` patterns with fricatives, in onsets with sonorants.
- **Conditions**: adjacent obstruents within a word or across word boundaries in connected speech.
- **Output**: `szívtől` [siːftøːl]; `hatvan` [hɒtvɒn] (no voicing of `t`); `házhoz` [haːshoz]; `sahban` [ʃɒxbɒn]; monomorphemic `tviszt`, `lekvár`, `kvarc`.
- **Examples**: see Output.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-ASSIM-009`, `PH-SYL-007` (v in final clusters: `érv`, `elv`, `könyv`, `hamv`; with obstruents only `kedv`, `nedv`, `üdv`).
- **Sources**: [KVF98 p. 442]; [AkH12 §50]; [T04 §3.2.4.2 pp. 63-64].
- **Validation**: `goldens` (pronunciation only).
- **Confidence**: `consensus`.

### PH-CONS-007 Realisation of j after consonants
- **Statement**: Word-final `j` after a consonant is a fricative: voiceless [ç] after a voiceless obstruent, voiced [ʝ] after a voiced consonant; before a vowel it is [j]. Spelling is unaffected.
- **Conditions**: `j` in word-final position preceded by a consonant (imperatives, `szomj`, `fürj`, `férj`).
- **Output**: `kapj` [kɒpç], `döfj` [døfç], `lökj` [løkç]; `szomj` [somʝ], `fürj` [fyrʝ], `égj` [eːɡʝ].
- **Examples**: see Output.
- **Exceptions**: after `t d n l gy ny ty s sz z` the `j` assimilates instead (`PH-ASSIM-004`, `PH-ASSIM-007`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-ASSIM-004`, `PH-ASSIM-007`, `PH-SYL-010`.
- **Sources**: [KVF98 §3.1.2.1.5 p. 384].
- **Validation**: `goldens` (pronunciation only).
- **Confidence**: `consensus`.

### PH-CONS-008 j and ly are one phoneme
- **Statement**: `ly` and `j` both write /j/; the choice between them is purely lexical spelling and has no effect on suffix allomorphy: an `ly`-final stem behaves exactly as a `j`-final stem (palatal, sonorant, `t`-coda licensing, `-a` possessive, `-lyal` spelling of `-val`).
- **Conditions**: stem ends in `ly` or `j`.
- **Output**: same allomorphs; `-val`/`-vá` doubles the letter written: `királlyal`, `bajjal`; 3sg possessive `-a`: `helye`, `baja`.
- **Examples**: `hely`->`helyet`, `helyen`, `hellyel`; `baj`->`bajt`, `bajjal`; `Kodály`->`Kodállyal`.
- **Exceptions**: in archaic names final `ly` may be /li/ (`Thököly`) `PH-G2P-005`.
- **Predictability**: `rule` (phonology); spelling `ly` vs `j` is `lexical` (`ORT`).
- **Lexical input**: stem spelling.
- **Interactions**: `ORT-JLY`, `PH-ASSIM-002`, `PH-G2P-005`.
- **Sources**: [AkH12 §88, §82 a), §92, §160 a)].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

---

## SYL: syllable structure and phonotactics

### PH-SYL-001 Syllable count
- **Statement**: Every vowel segment is the nucleus of exactly one syllable and there are no diphthongs or syllabic consonants in inflectable words; the syllable count of a word equals the number of its vowel segments.
- **Conditions**: pronounced vowel segments after `PH-G2P` (a foreign vowel digraph such as `ee` = /iː/, `eau` = /oː/ counts once; `au` in `autó` counts as two segments phonologically but see Exceptions).
- **Output**: integer syllable count, used by: all-neutral stems of 3+ syllables (`PH-HARM-007`), antiharmony domain (1-2 syllables, `PH-HARM-008`), high final vowel length (`PH-SYL-013`), minimal word (`PH-SYL-013`), epenthetic stem shape (`PH-ZERO-001`), `-ka/-ke` diminutive and other count-sensitive suffixes (`DER`).
- **Examples**: `fiú` = 2; `kakaó` = 3; `tea` = 2; `ideális` = 4.
- **Exceptions**: in `autó`, `trauma`, `augusztus` the `au` is pronounced as a falling diphthong [ɒw] in the standard; for counting and hyphenation the norm treats `a`+`u` as separate nuclei (`fe-udális`, [AkH12 §226]); syllabic consonants occur only in interjections (`hm`, `pszt`) and in the reduced conjunction `s` [KVF98 §3.1.2.4].
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-G2P-001`, `ORT-SPLIT-004`.
- **Sources**: [AkH12 §224-§226]; [T04 §3.2.1 pp. 37-39]; [KVF98 pp. 385, 413-416].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-SYL-002 Syllable template and medial syllabification
- **Statement**: The syllable is (C)V(V)(C)(C): onset at most one consonant (medially), nucleus short or long vowel, coda at most two consonants; medially a single consonant between vowels goes to the onset (`V.CV`), in a sequence of two or more consonants only the last goes to the onset (`VC.CV`, `VCC.CV`).
- **Conditions**:
  - Word-initial clusters are licensed only at the left edge (`PH-SYL-003`); medial complex onsets are not formed even when the cluster is a possible word-initial cluster (`cent.rum`, `lajst.rom`).
  - A geminate splits (`hal.lak`), a long digraph is one segment but splits in hyphenation spelling (`ORT-SPLIT-007`).
  - Onsetless syllables occur anywhere (`i.on`, `da.u.er`, `te.a`).
- **Output**: syllable boundaries.
- **Examples**: `pa.tak`, `fe.ke.te`, `em.ber`, `ke.men.ce`, `ab.lak`, `temp.lom`, `ost.rom`.
- **Exceptions**: morpheme and compound boundaries override for hyphenation (`vas.út`, `rend.őr`), not for phonological syllabification of suffixes (`há.zat`).
- **Predictability**: `rule`
- **Lexical input**: compound boundaries for hyphenation only.
- **Interactions**: `ORT-SPLIT-003`, `ORT-SPLIT-005`, `ORT-SPLIT-008` (hyphenation follows the same "last consonant to the next line" rule [AkH12 §226 c)-g)]), `PH-SYL-003`.
- **Sources**: [T04 §3.2.1-§3.2.3 pp. 37-50, §3.3.2.2 pp. 101-104]; [KVF98 §3.2.5 pp. 413-416]; [AkH12 §226].
- **Validation**: `goldens`, `oracle:hyph_hu_HU`.
- **Confidence**: `consensus`.

### PH-SYL-003 Word-initial consonant clusters
- **Statement**: Native words begin with at most one consonant; word-initial CC and CCC clusters occur only in loans and are phonotactically acceptable (not repaired); no geminate occurs word-initially.
- **Conditions**: attested initial CC types (T04 Table I): `pt ps(z) pn pl pr`, `tv tr`, `ksz kv kn kl kr`, `bl br`, `dr`, `gv gn gl gr`, `cv`, `ft fl fr fj`, `szp szt szty szk szc szf szv szm szn szl`, `sp st sk scs sv sm sn sl sr`, `vl`, `zl zr`, `mn`, `ng` (`nganaszán`), `hr` (`Hradzsin`); CCC: `sztr szkl spr str skr` (`sztrájk`, `szklerózis`, `spriccel`, `strázsa`, `skrupulus`).
- **Output**: well-formed onset edge cluster; no epenthesis.
- **Examples**: `plakát`, `prém`, `tréfa`, `kvarc`, `knédli`, `klór`, `krém`, `blúz`, `bronz`, `drukkol`, `gnóm`, `gróf`, `flóra`, `friss`, `fjord`, `sztár`, `szféra`, `szmog`, `sport`, `stáb`, `svéd`, `smink`, `slussz`, `sróf`, `szláv`, `zrí`, `mnemonika`.
- **Exceptions**: `dz` is a single affricate, not a cluster (`dzéta`); `ty` occurs initially only in `tyúk`, `tyüh`; `dzs` initially only in about 20 loans [KVF98 p. 386].
- **Predictability**: `rule` (well-formedness check); inventory is `lexical` (loan-dependent, not a productive filter).
- **Lexical input**: `none`
- **Interactions**: `PH-SYL-002`, `PH-G2P-008` (foreign initial clusters are kept as pronounced).
- **Sources**: [T04 §3.2.2 Tables I-II pp. 40-47]; [KVF98 §3.2.2.2.1 p. 386].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### PH-SYL-004 Single final consonants
- **Statement**: Any consonant may end a noun or other non-verb stem; monomorphemic verb stems may end in any single consonant except `ty`, `c`, `cs`, `dzs`, `zs` and the fricative /h/ ([x]).
- **Conditions**: stem-final single consonant; verb = free monomorphemic verb root (not `-ik` bound stems).
- **Output**: well-formedness predicate `final_single_ok(C, pos)`.
- **Examples**: nouns `sah`, `pác`, `kalács`, `rizs`, `ágy`, `bridzs`; verbs `kap`, `hat`, `rak`, `dob`, `ad`, `hagy`, `rág`, `döf`, `vesz`, `mos`, `szív`, `néz`, `nyom`, `ken`, `hány`, `él`, `szűr`, `fúj`.
- **Exceptions**: short final `dzs` hardly occurs even in nouns (`bridzs` is pronounced long); the verb gaps for `ty c cs dzs zs` are treated as accidental by T04 (long `c` and `dz` occur: `metsz` [mɛt͡sː], `edz`, `pedz`), the gap for /x/ is systematic.
- **Predictability**: `rule`
- **Lexical input**: `pos`.
- **Interactions**: `PH-SYL-011`, `VRB-STEM`.
- **Sources**: [T04 §3.2.4.1 p. 51, §3.4.3 pp. 121-122 incl. fn. 121-123]; [KVF98 §3.2.1.1 p. 386].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-SYL-005 Coda licensing of a suffixal t after a single consonant
- **Statement**: A suffix consisting of `t` (accusative `-t`, locative `-t`, and past `-t` subject to `PH-SYL-006`) can be syllabified into the coda after a single stem-final consonant only if that consonant is a sonorant (`j`/`ly`, `l`, `n`, `ny`, `r`) or a coronal fricative (`s`, `sz`, `z`, `zs`); after every other consonant a linking vowel is required.
- **Conditions**:
  - Stem ends in exactly one consonant (geminates: `PH-SYL-009`; clusters: `PH-SYL-008`).
  - Stem is not lowering, not epenthetic, not shortening (those always take the linking vowel, `NOM-LOW`, `PH-ZERO-002`, `PH-LEN-007`).
  - `z`+`t`, `zs`+`t` are pronounced [st], [ʃt] (voicing assimilation, unwritten).
- **Output**: predicate `t_coda_ok(C)` = C in {`j ly l n ny r s sz z zs`}; false for {`p b t d ty gy k g c dz cs dzs f v h m`}.
- **Examples**: licit: `ón`->`ónt`, `lány`->`lányt` [ATT 13], `dal`->`dalt` [ATT 12], `sör`->`sört` [ATT 11], `baj`->`bajt` [ATT 10], `rés`->`rést` [ATT 8], `kosz`->`koszt` [ATT 5], `gőz`->`gőzt` [ATT 6], `rizs`->`rizst` [ATT 10], `varázs`->`varázst` [ATT 1]; illicit: `nyom`->`nyomot` [ATT 10], `pad`->`padot`, `kép`->`képet`, `tök`->`tököt`, `hegy`->`hegyet`, `ív`->`ívet`, `zsiráf`->`zsiráfot`, `doh`->`dohot`, `rab`->`rabot`, `bot`->`botot`, `pác`->`pácot`, `kalács`->`kalácsot`.
- **Exceptions**: lowering nouns take a linking vowel even after licit consonants (`hal`->`halat`, `fal`->`falat`, `vár`->`várat`, `has`->`hasat`, `méz`->`mézet`); lexical exceptions are listed in `NOM-ACC` (e.g. `szakáll`->`szakállt`~`szakállat`, `oldal`->`oldalt`). Vowel-final stems take `-t` directly.
- **Predictability**: `rule`
- **Lexical input**: stem flags `lowering`, `epenthetic`, `shortening` (from `NOM`).
- **Interactions**: `NOM-ACC`, `NOM-LOW`, `VRB-PAST`, `PH-SYL-006`, `PH-ASSIM-009`.
- **Sources**: [T04 §3.2.4.2 (18) p. 62, §3.2.4.3 pp. 69-74, §4.1.2.2 pp. 145-153]; [AkH12 §36].
- **Validation**: `goldens`, `attestation`, `oracle:hunmorph-foma`.
- **Confidence**: `consensus`.

### PH-SYL-006 Coda licensing of the past t in verbs
- **Statement**: In verbs the bare past `-t` forms a coda only after a single sonorant (`j l n ny r`, including geminate `ll rr nn`); after coronal fricatives, which license accusative `-t` in nouns, the verb past needs the vowel (`les`->`lesett`, not `*lest` as its past; `lest` is the accusative of the noun `les`).
- **Conditions**: stem-final single sonorant; suffix = past `-t` before a consonant-initial ending or word-final 3sg; the exact past allomorph distribution (lexical `-ott` after `hall`, `-t` vs `-tt` classes) is in `VRB-PAST`.
- **Output**: predicate `past_t_coda_ok(C)` = C in {`j ly l n ny r`}.
- **Examples**: `vár`->`várt`, `kér`->`kért`, `fáj`->`fájt`, `ken`->`kent`, `hány`->`hányt`, `tol`->`tolt` [ATT 7]; `les`->`lesett` [ATT 1] (`lest` [ATT 2] is the noun `les` accusative), `csempész`->`csempészett` [ATT 5] vs noun accusative `csempészt`.
- **Exceptions**: `hall`->`hallott`, `hull`->`hullott`~`hullt`, `kell`->`kellett`, `vall`->`vallott` (lexical, `VRB-PAST`).
- **Predictability**: `rule` (phonotactic part); lexical exceptions in `VRB`.
- **Lexical input**: `none`
- **Interactions**: `VRB-PAST`, `PH-SYL-005`.
- **Sources**: [T04 §3.4.3 (80) p. 137, fn. 58 p. 73]; [KVF98 p. 295].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `consensus`.

### PH-SYL-007 Well-formed word-final two-consonant clusters
- **Statement**: A monomorphemic final CC is well-formed if (a) sonority falls towards the end (sonorant + obstruent, liquid + nasal, `r`/`j` + `l`) or it is a geminate, (b) an obstruent + obstruent cluster is a coronal fricative + coronal stop (`st`, `szt`, `zd`) and (c) a nasal + stop/affricate is homorganic (except `nyt`, `nyd` which arise only by suffixation); `v` after a liquid or nasal is well-formed (`érv`, `elv`, `könyv`, `hamv`). Clusters outside these patterns occur only in listed irregular words.
- **Conditions**: stem-final CC without internal morpheme boundary.
- **Output**: well-formedness predicate; used to validate lexicon entries and to classify stems for `PH-SYL-008`, `PH-HARM-014` (CC effect) and epenthesis (`PH-ZERO-001`).
- **Examples**: regular: `part`, `kard`, `halk`, `lomb`, `ront`, `rend`, `ponty`, `gyöngy`, `lánc`, `kincs`, `perc`, `tánc`, `test`, `liszt`, `most`, `film`, `szörp`, `kert`, `korty`, `fürj`, `férj`, `sarj`, `szomj`, `majd`, `fájl`, `görl`, `konszern`, `csepp`, `sakk`, `meggy`, `vicc`, `giccs`, `friss`, `klassz`.
- **Exceptions**: (complete lists from T04, irregular)
  - `pt`: `recept`, `korrupt`; `kt`: `absztrakt`, `akt`, `defekt`, `direkt`, `egzakt`, `indirekt`, `intakt`, `kompakt`, `korrekt`, `perfekt`, `verdikt`, `viadukt`; `csk`/`ck`: `barack`, `palack`, `tarack`; `pf`: `copf`; `ps` (`s`=/ʃ/): `arabs`, `taps`; `ks` (/kʃ/): `voks`; `dv`: `kedv`, `nedv`, `üdv`; `dz`: `edz`, `pedz` (surface geminate affricate); `nl`: `ajánl` (pronounced [ɒjaːlː]).
  - `ft`: `lift`, `kuncsaft`, `seft`, `szaft`, `taft`; `szk`: `arabeszk`, `baszk`, `burleszk`, `etruszk`, `groteszk`, `humoreszk`, `kioszk`, `maszk`, `obeliszk`, `odaliszk`, `pittoreszk`; `zg`: `rezg` (bound); `sb`: `idősb`, `kevesb`, `nemesb` (obsolete).
  - `gd`: `smaragd`; `mj`: `szomj`; `rj`: `férj`, `fürj`, `sarj`; `ksz` (x): `bilux`, `bórax`, `boksz`, `fix`, `főnix`, `index`, `keksz`, `kódex`, `koksz`, `komplex`, `konvex`, `ónix`, `ortodox`, `paradox`, `reflex`, `suviksz`, `turmix`; `psz`: `bicepsz`, `gipsz`, `mumpsz`, `ripsz`, `snapsz`, `zsupsz`.
  - `mf`: `tromf`; `mz`: `nemz` (obsolete); `mzs`: `tömzs`; `mt`: `teremt`, `vikomt`.
  - Final CCC (all irregular): `mumpsz`, `szfinx`, `szkunksz`, `dunszt`, `kunszt`, `karszt`, `verszt`, `szext`, `vurst`, `borscs`, `lejszt`, `mihelyst`.
  - Place names form a separate stratum with further clusters (`Detk`, `Batyk`, `Recsk`, `Szakcs`).
- **Predictability**: `rule` for the regular patterns; `lexical` for the listed irregular items (about 90 words, T04).
- **Lexical input**: `none` (validation only).
- **Interactions**: `PH-SYL-008`, `PH-SYL-011`, `PH-ZERO-001`, `PH-HARM-014`.
- **Sources**: [T04 §3.2.4.2 Table III and (9)-(23) pp. 51-68].
- **Validation**: `attestation`, `goldens`.
- **Confidence**: `consensus` (the regular/irregular split is T04's analysis).

### PH-SYL-008 Suffixal t after a final cluster
- **Statement**: After a stem-final CC the accusative `-t` attaches bare only if the stem is lexically marked for it; the marking is possible only when C1 is a sonorant, C2 + `t` is a licit coda (`PH-SYL-005`), and C1C2 is not `r` + (`s`, `sz`, `z`); unmarked CC-final stems take the linking vowel.
- **Conditions**: C1C2-final noun stem, not lowering.
- **Output**: `-t` bare (marked) | linking vowel + `t` (unmarked) | both (variable).

  | Cluster | bare `-t` | linking vowel | both |
  |---|---|---|---|
  | `ns` (`n`+/ʃ/) | `protestánst`, `briliánst` | `revansot` [ATT 2] | `szegmenst`~`szegmenset` |
  | `nsz` | `nonszenszt` | `sanszot`, `fajanszot` | `reneszánszt`~`reneszánszot` |
  | `jz` | `csuszpájzt` | `rajzot` [ATT 8] | - |
  | `nz` | `pénzt` [ATT 18] | `bronzot` [ATT 5] | `csimpánzt`~`csimpánzot` |
  | `jzs` | - | `pajzsot` (T04's `*pajzst` is 22 : 7,629 [WC2]) | - |
  | `rn`, `jn`, `jl`, `rl` | `konszernt`, `kombájnt`, `fájlt` [ATT 10], `görlt` | - | - |
  | `rs`, `rsz`, `rz` | never | `borsot` [ATT 9], `merszet`, `borzot` | - |
  | obstruent + C | never | `tapsot` [ATT 5], `szexet` [ATT 8], `rajzot` | - |
- **Examples**: see table; also `ENSZ-t` (spelled acronym /ɛns/); measured shares [WC2, `[/N][Acc]` on the named lemma]: `briliánst` 199 : `briliánsot` 0, `fajanszot` 134 : `?fajanszt` 9, `protestánst` 341 : `*protestánsot` 1, `szegmenst` 6,828 ~ `szegmenset` 1,696.
- **Exceptions**: `rny`-final nouns and `fürj`, `törzs` are lowering (`szárnyat`, `fürjet`, `törzset`), so the vowel appears for an independent reason.
- **Predictability**: `lexical` (stem flag), with the phonotactic necessary condition as `rule`.
- **Lexical input**: stem flag `acc_bare` (`NOM-ACC`).
- **Interactions**: `NOM-ACC`, `PH-SYL-005`, `PH-SYL-007`.
- **Sources**: [T04 §3.2.4.3 (26) p. 73, pp. 69-74 (data from Papp 1975)]; [WC2]; [ATT evidence/corpus-variants.md 2026-09-16].
- **Validation**: `attestation` (tag-filtered Webcorpus 2 counts per item), `oracle:hunmorph-foma`.
- **Confidence**: `consensus`; the per-item marks are now `measured`, and the class shows no general preference - `fajansz` takes the linking vowel while `briliáns` takes bare `-t` categorically.

### PH-SYL-009 Suffixal t after a final geminate
- **Statement**: A stem-final geminate of a `t`-licensing consonant (`ss`, `ssz`, `zz`, `nn`, `nny`, `ll`, `rr`, `jj`) behaves like the single consonant (bare `-t`, pronounced with a short consonant by degemination); geminates of non-licensing consonants take the linking vowel; a few items vary.
- **Conditions**: geminate-final non-lowering stem.
- **Output**: bare `-t` by default; variants listed.
- **Examples**: `idill`->`idillt` [ATT 2]; `finn`->`finnt` [ATT 1]; `plüss`->`plüsst`; `bross`->`brosst` 508 : `brossot` 32 [WC2]; `dzsessz`->`dzsesszt` [ATT 0]; non-licensing: `csepp`->`cseppet`, `sakk`->`sakkot`, `meggy`->`meggyet`, `pötty`->`pöttyöt`, `vicc`->`viccet`, `giccs`->`giccset`.
- **Exceptions**: variable by T04: `genny`, `orr`, `bross`; measured [WC2, `[/N][Acc]` on the named lemma]: `gennyet` 853 : `?gennyt` 50, past the tenfold margin of a doublet; `orrot` 8,930 : `*orrt` 2, so `orr` takes the linking vowel categorically, while the geminate `ss` of `bross` takes bare `-t` categorically (`brosst` 508 : `brossot` 32) - the split is per item, not per geminate type; `könnyet`, `szennyet` unchanged; lowering geminate stems take the lowered vowel (`toll`->`tollat` [ATT 9], `ujj`->`ujjat` [ATT 4], `gally`->`gallyat`).
- **Predictability**: `rule` with `lexical` exceptions (about 10 items).
- **Lexical input**: stem flags `lowering`, `acc_bare`.
- **Interactions**: `NOM-ACC`, `PH-ASSIM-012`.
- **Sources**: [T04 p. 73 incl. fn. 56-58]; [research-lit REPORT corpus note]; [WC2]; [ATT evidence/corpus-variants.md 2026-09-16].
- **Validation**: `attestation` (tag-filtered Webcorpus 2 counts per item).
- **Confidence**: `consensus`; individual variants `measured` (per-item, near-categorical in both directions).

### PH-SYL-010 Appendix suffixes -d, -j, -sz
- **Statement**: The imperative definite 2sg `-d`, the imperative `-j` and the present indefinite 2sg `-sz` are exempt from coda constraints: they can follow any consonant and any final cluster, violating sonority (`kapsz`, `dobd`, `lopj`), except that `-sz` is replaced by `-Vl` after strident-final stems and by `-Asz` after most cluster-final stems (morphological allomorphy in `VRB-PRES`).
- **Conditions**: verb stems; suffix in the set above.
- **Output**: no linking vowel for `-d` and `-j`; `-j` is subject to assimilation (`PH-ASSIM-004`, `PH-ASSIM-005`, `PH-ASSIM-007`); `-d` after `t`-final imperative stems per `PH-ASSIM-006`.
- **Examples**: `nyomd`, `hányd`, `zengd`, `tartsd`, `ajánld`, `teremtsd`; `dobj`, `fogj`, `zengj`, `hívj`; `kapsz`, `vágysz`, `raksz`, `lengsz`, `ajánlsz`; strident: `teszel`, `nézel`, `keresel`; cluster: `osztasz`, `tartasz`, `mondasz`~`?mondsz`, `fingasz`~`fingsz` (T04), `varrsz` (no vowel; `*varrasz`).
- **Exceptions**: `-Asz` after long vowel + `t` for all `-ít` verbs and some others (`alakítasz`, `vétesz`, `fűtesz`, but `látsz`) (lexical, `VRB-PRES`).
- **Predictability**: `rule` (phonotactic exemption); allomorphy partly `lexical`.
- **Lexical input**: verb class (`VRB-STEM`).
- **Interactions**: `VRB-PRES`, `VRB-IMP`, `PH-ASSIM-004`.
- **Sources**: [T04 §3.2.4.3 pp. 65-72 incl. fn. 53, Table IV]; [KVF98 pp. 439-440].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-SYL-011 Final clusters of monomorphemic verb roots
- **Statement**: A monomorphemic free verb root ending in CC must end in a coronal cluster licensed by falling sonority (`st` /ʃt/, `zsd` /ʒd/, `jt` not strictly homorganic; `szt`, `zd` homorganic) or a homorganic non-labial cluster (`nt`, `nd`, `nz`, `ng`, `lt`, `ld`, `rt`, `rd`) or a non-labial geminate (`dd`, `tsz`=[t͡sː], `dz`=[d͡zː], `gg`, `ll`, `rr`); a sonorant C2 must be the second half of a geminate, an affricate C2 must be geminate, palatal stops never occur, and `s`/`zs`/`j` occur only as C1. New verbs cannot be monomorphemic: loans need a verbalising suffix (`-ol/-el/-öl`, `-z`, `-ál`).
- **Conditions**: verb root without derivational suffix (`-ik` bound stems excluded).
- **Output**: predicate `verb_root_ok`; counts of roots per final cluster (T04 (72), spelled): `szt` (/st/) 117, `nt` 100, `ng` 51, `ll` 34, `jt` 19, `lt` 14, `rt` 8, `ld` 7, `nd` 4, `gg` 3, `dz` 2, `zd` 2, `rd` 2, `rr` 2, `dd` 1, `tsz` 1, `st` (/ʃt/, `fest`) 1, `zsd` 1, `nz` 1, `mt` 1 (irregular `teremt`); T04 also cites apparent `mz` (`nemz`), `lg` (`uralg`), `nl` (`ajánl`), 1 each.
- **Examples**: `fedd`, `edz`, `függ`, `metsz`, `oszt`, `fest`, `kezd`, `esd`, `önt`, `mond`, `vonz`, `zeng`, `olt`, `küld`, `vall`, `tart`, `hord`, `varr`, `hajt`; loans: `sztrájkol`, `lájkol` [ATT -] / `lájkolni` [ATT 1].
- **Exceptions**: `teremt` (irregular `mt`), obsolete `nemz`, `uralg`; `ajánl` pronounced with geminate `l`. Bound `-ik` stems may end in otherwise ill-formed clusters and are defective (`csuklik`, `bűzlik`, `vedlik`) (`PH-ZERO-004`).
- **Predictability**: `rule`
- **Lexical input**: `none` (validation, loan verb formation).
- **Interactions**: `VRB-STEM`, `VRB-IK`, `PH-SYL-006`.
- **Sources**: [T04 §3.4.3 (72)-(76) pp. 121-126, fn. 118 p. 121].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-SYL-012 Long vowels before tautomorphemic consonant clusters
- **Statement**: Inside a morpheme a long vowel other than `á` and `é` does not occur before a consonant cluster (medial or final), and before a tautomorphemic geminate even `á`/`é` are rare; across any morpheme boundary all long vowels occur before clusters.
- **Conditions**: VVCC within one morpheme.
- **Output**: constraint used as a cue for foreign-word length (`PH-G2P-008`) and for lexicon validation.
- **Examples**: regular: `érc`, `érték`, `márt`, `árpa`, `fánk`; across boundaries: `sírt`, `szívtam`, `bűnben`, `főbb`, `kérték`, `túrtak`, `kórnak`, `vártam`.
- **Exceptions**: (irregular, T04) final: `tószt`, `avítt`, `blazírt`, `bornírt`, `fasírt`, `múlt`; medial: `ízlés`, `nőstény`, `tőzsde`, `csúzli`, `kóstol`, `bóklászik`, `kókler`, `kóstál`, `lófrál`, `sóska`, `ósdi`, `ótvar`, `pózna`, `ródli`, `ócska`, `-ódzik`/`-ődzik`; spelled long but pronounced short: `őrs`, `gyűjt`, `gyújt`, `nyújt`, `sújt`, `őrjöng`. Before geminates: `állat`, `állam`, `állapot`, `állandó`, `áll`, `száll`, `szakáll`, `váll`, `vállal`, `inkább`, `istálló`, `koráll`, `majoránna`, `éjjel`, `épp`, `éppen`, `-képp`, `avítt`, `előtt`, `-ódzik` (often pronounced with short consonant).
- **Predictability**: `rule` (static constraint) with `lexical` exceptions (about 50 items).
- **Lexical input**: `none`
- **Interactions**: `PH-G2P-008`, `PH-LEN-011`.
- **Sources**: [T04 §3.4.2 (68)-(70) pp. 115-121]; [SMNy3 Rebrus p. 698].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### PH-SYL-013 Stem-final vowels and the minimal word
- **Statement**: Content-word stems (a) never end in short `o` or `ö` (such loans are pronounced and hungarianised with `ó`/`ő`); (b) end in short or long `a`/`e`/`á`/`é`, short final `a`/`e` lengthening before suffixes (`PH-LEN-001`); (c) if monosyllabic, never end in a short vowel (minimal word), except `fa` and `ma`; (d) if monosyllabic and high-vowel-final, have a long vowel (`sí`, `bú`, `tű`); polysyllabic stems in high vowels have a short or long vowel lexically, with innovative speakers pronouncing all as short.
- **Conditions**: stem-final open syllable of a content word; function words (`ki`, `mi`, `ti`, `be`, `de`, `le`, `ne`, `se`, `te`, `ha`), interjections (`no`, `nono`, `na`, `ja`, `ni`) and borrowed monosyllabic names (`Li`, `Wu`, `Che`) are exempt.
- **Output**: constraints for lexicon validation and for foreign-word pronunciation (`PH-LEN-005`, `PH-G2P-008`).
- **Examples**: `olló`, `szőlő`, `libretto` pronounced [librɛtːoː]; `teve`, `apa`, `csokoládé`, `burzsoá`, `hajrá`; `sí`, `rí`, `zrí`, `bú`, `szú`, `fű`, `mű`, `tű`; polysyllabic short: `kocsi`, `kifli`, `falu`, `kapu`, `eskü`, `menü`; long: `ágyú`, `fiú`, `betű`, `gyűrű`.
- **Exceptions**: word-final long `í` occurs in only about 9 items (`sí`, `rí`, `zrí`, `vizaví`, interjections); final `á` in about 15 items, of which only `burzsoá` and `hajrá` are content words (the rest function words, abbreviations, interjections); final `é` in about 130 loans; foreign compounds keep short `o` in the first member (`pszichobiológia`, `pszeudofőnév`).
- **Predictability**: `rule` for (a) and (c); `lexical` for polysyllabic high-vowel length (spelling list in `ORT`).
- **Lexical input**: lexical vowel length.
- **Interactions**: `PH-LEN-001`, `PH-LEN-005`, `PH-LEN-006`, `ORT-LEN-003`, `ORT-LEN-004`.
- **Sources**: [T04 §3.4.1 (57)-(67) pp. 105-114]; [SMNy3 Rebrus §13.6.1.2 pp. 789-791]; [AkH12 §22-§24]; [ATT evidence/corpus-variants.md 2026-09-16] (no corpus instrument exists).
- **Validation**: `goldens`, `attestation`; `needs:pronunciation-survey` for (d) - a corpus cannot settle it, because Hungarian orthography fixes the length normatively, so spelling counts measure the norm and not the speakers (`falu` : `falú`, `kapu` : `kapú` in Webcorpus 2 count typos).
- **Confidence**: `consensus`; (d) dialect split `contested` (conservative vs innovative standard), not corpus-arbitrable.

### PH-SYL-014 Hiatus
- **Statement**: Two vowels may be adjacent inside a morpheme (mostly loans); identical vowels (and a short vowel next to its long pair) are adjacent only across an analytic boundary; when a synthetic suffix beginning with an unstable (linking) vowel follows a vowel-final stem the linking vowel is absent; a glide [j] is inserted in pronunciation next to `i`/`í`/`é` and is not written.
- **Conditions**: VV sequences.
- **Output**: no repair in spelling; pronunciation [j] between `i`/`í`/`é` and a following or preceding non-identical vowel (`fiú` [fijuː], `dió` [dijoː], `kiabál` [kijɒbaːl], `mozaik` [mozɒjik]); `fáig` [faːjig].
- **Examples**: `kies`, `július`, `diéta`, `fiú`, `dió`, `kiált`, `múzeum`, `neon`, `tea`, `leány`, `influenza`, `vákuum`, `boa`, `poén`, `oázis`, `autó`, `aorta`, `naiv`, `kakaó`, `káosz`; across boundaries: `kiirt`, `rakétaautó`, `biliig`, `leég`, `ráad`; linking vowel absent: `ház`->`házon` but `kapu`->`kapun`.
- **Exceptions**: a long vowel is not prevocalic inside a morpheme (only exception `káosz`); monomorphemic identical vowels only in `kooperál`, `koordináta`, `zoológia`, `individuum`, `vákuum` (usually pronounced with one vowel); front rounded vowels in hiatus only in `nüansz`, `Szöul`, `entellektüel`, `menüett`, `müezzin`, `habitüé`, `enteriőr`, `exteriőr`, `miliő`.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LINK`, `VRB-LINK`, `PH-ASSIM-014`, `ORT-SPLIT-004` (`di-adal`).
- **Sources**: [T04 §3.3.1 Table V, (29)-(33) pp. 75-82]; [KVF98 §3.2.3.3 pp. 410-412].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-SYL-015 No adjacent different stridents inside a domain
- **Statement**: Two strident consonants (`sz z s zs c dz cs dzs`) with different root segments are not adjacent inside a morpheme or synthetic domain; where suffixation or compounding brings them together they assimilate in pronunciation (`PH-ASSIM-011`) and the spelling keeps both letters.
- **Conditions**: CC with both members strident; true geminates are allowed.
- **Output**: morpheme-internal validation; pronunciation assimilation across boundaries.
- **Examples**: across boundaries: `húszcentis`, `tenyészcsődör`, `hascsikarás`, `kisszerű`, `rácsszerkezet`, `gázcső`, `kézszorítás`, `darázscsípés`, `község` [køʃːeːɡ], `egészség` [ɛɡeːʃːeːɡ].
- **Exceptions**: eleven irregular loans: `borscs`, `szcéna`, `scsí`, `diszciplína`, `proszcénium`, `reminiszcencia`, `oszcillo-`, `obszcén`, `excentrikus`, `transzcendens`, `excellenciás`.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-ASSIM-011`, `VRB-PRES` (2sg `-Vl` after strident stems).
- **Sources**: [T04 §3.5 (81) p. 138, fn. 31 p. 60]; [KVF98 §3.4.1.1.10 pp. 444-446].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-SYL-016 Fast cluster simplification (pronunciation only)
- **Statement**: In fast speech the middle consonant of a three-consonant sequence may be dropped if it is a stop, the third consonant is a non-continuant and the first is not a continuant sonorant (`l r j`); spelling never reflects it and suffix selection is computed on the full form.
- **Conditions**: C1 not in {`l r j`}; C2 a stop (not a fricative or affricate); C3 [-continuant].
- **Output**: optional deletion of C2 in pronunciation.
- **Examples**: `lambda` [lɒmdɒ], `asztma` [ɒsmɒ], `röntgen` [røŋɡɛn], `dombtető` [domtɛtøː], `mindnek` [minːɛk] [AkH12 §60], `csukd be` [t͡ʃuɡbɛ] [AkH12 §60]; blocked: `centrum`, `templom` (C3 continuant), `talpnyaló` (C1 `l`), `könyvtár` (C2 fricative).
- **Exceptions**: sporadic deletion before `v` (`szoftver`, `szendvics`, `testvér`); lexicalised `mumpsz` [mums]; AkH12 §60 also gives `most sincs` [moʃːint͡ʃ] with C3 a continuant (sibilant sequence), outside the T04 condition; imperative `-j` [ç]/[ʝ] is omissible between any two consonants (`dobj rá`) [T04 fn. 124].
- **Predictability**: `rule` (optional)
- **Lexical input**: `none`
- **Interactions**: `PH-ASSIM-012`.
- **Sources**: [T04 §4.2.3 (75)-(82) pp. 224-229]; [AkH12 §60].
- **Validation**: `goldens` (pronunciation only).
- **Confidence**: `consensus`.

---

## HARM: vowel harmony

### PH-HARM-001 Harmony domain
- **Statement**: The harmony class of a word form is computed from the vowels of its last phonological domain: the last member of a compound, the verb stem after a verbal prefix, the stem plus all suffixes attached so far; each suffix vowel is chosen from the string to its left within that domain, so harmony iterates through suffix chains.
- **Conditions**:
  - Compound boundaries (`DER`) and verbal prefix boundaries (`VRB-PREV`) start a new domain; stem-internal disharmony across such boundaries is ignored.
  - Hyphenated compounds and name + common noun compounds: the last member decides.
  - Suffixes are domain-internal (they harmonise even when analytic or super-analytic, e.g. `-ság/-ség`).
- **Output**: domain vowel string passed to `PH-HARM-021`.
- **Examples**: `könyvtár`->`könyvtárban`; `százöt`->`százötben`; `átüt`->`átütök`; `föláll`->`fölállok`; `ad-omány-om-nak`, `nyer-emény-em-nek`; `Petőfi-szoborhoz`.
- **Exceptions**: foreign pseudo-compounds whose boundary is not recognised by speakers vacillate (`PH-HARM-022`); first-member-final short `o` (`pszichobiológia`) shows the domain split phonetically.
- **Predictability**: `rule`
- **Lexical input**: compound and prefix boundaries.
- **Interactions**: `PH-HARM-021`, `PH-HARM-022`, `DER`.
- **Sources**: [KVF98 §3.2.6.4.1 p. 420, (74) p. 422]; [RST12 fn. 5 p. 494]; [T04 §3.3.1 p. 79].
- **Validation**: `goldens`, `oracle:hunmorph-foma`.
- **Confidence**: `consensus`.

### PH-HARM-002 Archiphonemes and their realisation
- **Statement**: Harmonic suffix vowels are archiphonemes realised from the stem harmony class `B`, `F` or `R`; `i`, `í`, `é` in invariant suffixes are not archiphonemes.
- **Conditions**: suffix template contains `A Á O Ó U Ú` (README).
- **Output**:

  | Archiphoneme | `B` | `F` | `R` | Number of suffixes (RST12) | Common suffixes |
  |---|---|---|---|---|---|
  | `A` | `a` | `e` | `e` | 24 | `-nAk`, `-bAn`, `-bA`, `-vAl`, `-(j)A`, `-hAt`, `-nA`, `-gAt`, `-tAt`, `-vA`, `-kA`, `-cskA`, `-tAlAn`, `-lAg`, `-An` (adverb), lowered linking vowel (`NOM-LOW`) |
  | `Á` | `á` | `é` | `é` | 10 | `-nÁl`, `-vÁ`, `-Ás`, `-nÁ-`, `-sÁg`, `-vÁn`, `-vÁny`, `-Ász`, `-tÁk`, `-nÁk` |
  | `O` | `o` | `e` | `ö` | 9 (plus linking vowels) | `-hOz`, `-On`, `-szOr`, `-Os`, `-tOk`, `-Ol`, `-Oz`, `-kOdik`, `-Om`, `-Od`, `-Ott`, `-OgAt`, linking `-Ok`, `-Ot`, `-Om` |
  | `Ó` | `ó` | `ő` | `ő` | 5 | `-tÓl`, `-bÓl`, `-rÓl`, `-Ó` (participle), `-Ódik`, `-hAtÓ` |
  | `U` | `u` | `ü` | `ü` | 7 | `-Unk`, `-Uk`, `-Ul` (essive and verbal), `-stUl`, `-dUl` |
  | `Ú` | `ú` | `ű` | `ű` | 1 | `-(j)Ú` |
- **Examples**: `ház`->`háznak`, `háznál`, `házhoz`, `háztól`, `házunk`, `lábú`; `kert`->`kertnek`, `kertnél`, `kerthez`, `kerttől`, `kertünk`, `fejű`; `tök`->`töknek`, `töknél`, `tökhöz`, `töktől`, `tökünk`, `bőkezű`.
- **Exceptions**: `PH-HARM-006` (suffixes with defective `O`); `-(j)A`/`-i` 3sg definite is a suppletive back/front pair (`látja`/`nézi`), handled as `A`-like choice in `VRB-PRES`.
- **Predictability**: `rule`
- **Lexical input**: suffix template.
- **Interactions**: `PH-HARM-005`, `PH-HARM-021`.
- **Sources**: [RST12 (3) p. 493]; [KVF98 (74) p. 422]; [SMNy3 Rebrus §13.2.1.4 pp. 682-683].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-HARM-003 Last vowel back: back harmony
- **Statement**: If the last vowel of the domain is `B`, the stem is `B`, whatever precedes.
- **Conditions**: last vowel in `a á o ó u ú`.
- **Output**: class `B`.
- **Examples**: `ablak`->`ablaknak`; `bíró`->`bírónak`; `glükóz`->`glükóznak`; `amőba`->`amőbának`; `csinos`->`csinosnak`; `betyár`->`betyárnak`; `béka`->`békának`.
- **Exceptions**: none in the lexicon (Hayes-Londe `B` stems: backness index 0.999 over 6290 stems; residue = typos).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-HARM-021`.
- **Sources**: [HZSL09 (6) p. 9]; [HL06 Table I p. 66]; [KVF98 p. 420].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-HARM-004 Last non-neutral vowel front rounded: front harmony
- **Statement**: If the rightmost non-neutral vowel of the domain is `R`, the stem is front (`R` or `F` by `PH-HARM-005`), with any number of neutral vowels after it.
- **Conditions**: rightmost vowel not in `i í e é` is in `ö ő ü ű`.
- **Output**: front class.
- **Examples**: `üst`->`üstnek`; `szemölcs`->`szemölcsnek`; `sofőr`->`sofőrnek`; `fűszer`->`fűszernek`; `őrizet`->`őrizetnek`; `kosztüm`->`kosztümnek`; `römi`->`röminek`.
- **Exceptions**: none (Hayes-Londe `R` stems: 0 back over 698).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-HARM-005`, `PH-HARM-021`.
- **Sources**: [HZSL09 (7)-(8) p. 10]; [HL06 Table I]; [KVF98 pp. 420-422].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-HARM-005 Rounding harmony of O
- **Statement**: In a front domain the archiphoneme `O` is `ö` if the immediately preceding vowel (the last vowel of the domain) is `ö ő ü ű`, otherwise `e`; rounding is strictly local and only affects short mid `O`; back domains always get `o`.
- **Conditions**: suffix vowel `O`; stem class front.
- **Output**: class `R` if last vowel in `ö ő ü ű`; class `F` if last vowel in `i í e é`. The class is recomputed after each suffix: a realised `ö` keeps `R`, a realised `e`/`é`/`i` gives `F`.
- **Examples**: `tök`->`tökhöz`, `tököm`; `cipő`->`cipőhöz`; `fű`->`fűhöz`; `ötször`; `sün`->`sünök`, `sünöknek`; `üveg`->`üveghez` (last vowel `e`); `sofőr`->`sofőrhöz`; `négy`->`négyszer`; `sün`->`sünös`->`sünösötökhöz`; `föld`->`földhöz`.
- **Exceptions**: regional systems (not generated): `o`/`ë`/`ö` with closed `ë` (most of the language area except the north-east and Budapest) and two-way `o`/`ö` (Szeged area, northern Central Hungary: `emberhöz`) [SMNy3 Rebrus (32) p. 692]; lowering stems replace `O` by `A` (`fül`->`fülek`, `NOM-LOW`); lexically defective suffixes `PH-HARM-006`.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-HARM-006`, `PH-HARM-021`, `NOM-LOW`.
- **Sources**: [SMNy3 Rebrus §13.2.3 pp. 686-693]; [RST12 §3.1 p. 493]; [KVF98 (74) p. 422].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-HARM-006 Suffixes with defective O alternation
- **Statement**: A few non-productive derivational suffixes have only two of the three `O` values: `-nok/-nök` never shows `e` (front stems take `ö` even after `i í e é`), `-omány/-emény` and `-ovány/-evény` never show `ö`.
- **Conditions**: suffix is one of the listed.
- **Output**: `-nOk`: `B`->`o`, `F`/`R`->`ö`; `-OmÁny`, `-OvÁny`: `B`->`o`, `F`/`R`->`e`.
- **Examples**: `hírnök`, `mérnök`, `elnök`, `szónok`; `adomány`, `küldemény`, `őrlemény`, `sütemény`, `nyeremény`; `szökevény`, `jövevény`.
- **Exceptions**: `-`
- **Predictability**: `lexical` (3 suffixes; `-nOk` in about 34 words, `-OmÁny` about 8 front-rounded bases, `-OvÁny` about 4)
- **Lexical input**: suffix property `o_alternation` (`three` | `o_oe` | `o_e`).
- **Interactions**: `PH-HARM-005`, `DER`.
- **Sources**: [RST12 (3d)-(3e) pp. 493-494]; [SMNy3 Rebrus (23) p. 688].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-HARM-007 All-neutral stems of three or more syllables
- **Statement**: A monomorphemic domain containing only neutral vowels and at least three syllables is front (`F`).
- **Conditions**: all vowels in `i í e é`; syllable count >= 3.
- **Output**: class `F`.
- **Examples**: `Rimini`->`Riminiben`; `szintézis`->`szintézisnek`; `fekete`->`feketének`; `Debrecen`->`Debrecenben`; `feleség`->`feleségnek`.
- **Exceptions**: the rule is stated for monomorphemic domains; a string of neutral vowels created by suffixation is evaluated by `PH-HARM-018`/`PH-HARM-019` (e.g. `férfi`+`-as` -> `férfiasnak` stays back because `-as` is realised back).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-HARM-008`, `PH-HARM-021`.
- **Sources**: [RST12 p. 495]; [HZSL09 fn. 7 p. 10]; [HL06 Table I (NN 0.002)].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-HARM-008 All-neutral stems of one or two syllables and antiharmonic roots
- **Statement**: A one- or two-syllable all-neutral stem is `F` unless it is lexically antiharmonic (takes back suffixes); antiharmonic roots form a closed, unproductive class of some sixty roots (KVF98), nearly all monosyllables with `i` or `í`, two with `é` (`cél`, `héj`) plus the bound roots `bén-`, `ném-`, and two disyllables (`derék`, `férfi`, `PH-HARM-009`).
- **Conditions**: all vowels neutral; syllable count 1 or 2.
- **Output**: `F` by default; `B` if lexically marked.
- **Examples**: front: `kert`->`kertnek`, `cím`->`címnek` [ATT 11 `címek`], `hír`->`hírnek`, `víz`->`víznek`, `ív`->`ívnek`, `szív` (heart) ->`szívnek`, `repesz`->`repesznek`, `kicsi`->`kicsinek`; antiharmonic nouns/adjectives: `híd`->`hidak` [ATT 10], `íj`->`íjak`, `nyíl`->`nyilak` [ATT 7], `sír` (grave) ->`sírok` [ATT 8], `gyík`->`gyíkok`, `szíj`->`szíjak`, `zsír`->`zsírok` [ATT 11], `díj`->`díjak` [ATT 15], `kín`->`kínok`, `csík`->`csíkok`, `ín`->`inak`, `cél`->`célok` [ATT 17], `héj`->`héjak`; antiharmonic verbs: `ír`->`írnak`, `sír`->`sírnak`, `bír`->`bírnak`, `hív`->`hívnak`, `szív` (suck) ->`szívnak`, `vív`->`vívnak`, `szít`->`szítanak`, `nyit`->`nyitnak`, `tilt`->`tiltanak`, `irt`->`irtanak`, `iszik`->`isznak`, `ivott`; bound: `bénít`->`bénítok`, `némít`->`némítok`.
- **Exceptions**: homophones split (`szív` heart F vs `szív` suck B; Hayes-Londe `ív` 0.33 back, verb back vs noun front); `izé` is jocularly back [RST12 fn. 7]; the neutral-vowel loans `spejz` and `design` take back suffixes against the generalisation that `e` monosyllables are front [F13 §3.3; ATT evidence/lexical-lists.md 2026-09-16]. `férfi` stays in review by design: `férfinak` 70,490 : `férfinek` 17,325, both legal, which is the vacillation `PH-HARM-009` already describes; `ín` was settled by the spelling checker (`ínnak` legal, `ínnek` unknown) against a thin corpus.
- **Predictability**: `lexical` (**closed list of 72 lemmas** - 46 verbs, 24 nouns, 9 adjectives - induced from 2,213 shape candidates and corpus-verified, superseding the earlier "about 60 roots" estimate; Hayes-Londe database: 1955 all-neutral stems, 20 majority back, 18 of them monosyllabic, 16 with `i`/`í` (`díj gyík híd híg kín nyílt síp szíj zsír sík csík íj pír nyír ín víg`), 2 with `é` (`cél` 1.00, `héj` 0.96), the 2 disyllables `férfi`, `derék`; `CíC` shape is nearly necessary but not sufficient: 17 monosyllables with `í` are majority front; backness index `N` monosyllables 0.078 over 296, `NN` 0.002 over 933)
- **Lexical input**: stem flag `harmony=B` (antiharmonic); the list is `antiharmonic.tsv`, 79 rows / 72 lemmas (46 V, 24 N, 9 A) plus 23 in review and 92 non-members, in `local/lists/antiharmonic.tsv` - read by `cargo xtask morphdb`. It reproduces the classic inventory of Vago (1980: 8-12, cited by KVF98 and HZSL09) and adds the loans `spejz` and `design`.
- **Interactions**: `PH-HARM-009`, `PH-HARM-010`, `PH-HARM-013` (`USE BACK/[C0iːC0]`), `PH-ZERO-001` (epenthetic antiharmonic stems `szirom`, `kínoz`, `céloz`).
- **Sources**: [KVF98 p. 421]; [HZSL09 (11), (19), (21) pp. 11, 20-22]; [HL06 Table I]; [RST12 §3.2 (ii) p. 495, §6.4 pp. 506-507]; [WC2]; [hunmorph-foma]; [magyarispell]; [MTA-HELYES]; [morphdb]; [UM-hun]; [kaikki]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `goldens`, `attestation`, `oracle:hunmorph-foma`; membership induced by cross-resource voting over 2,213 shape candidates. Oracle agreement on corpus-decided rows: our own `.ron` 153/159, foma 144/144, hunspell 144/144, UniMorph 113/113, kaikki 131/131; **morphdb's `back` flag is not an antiharmony flag** (67/159, 42%) and must not be read as a harmony class.
- **Confidence**: `consensus` (class); membership `measured` (closed list of 72 lemmas).

### PH-HARM-009 Antiharmonic disyllables derék and férfi
- **Statement**: The only two-syllable all-neutral stems with back harmony are `derék` and `férfi`; both take only back vowels before vowel-initial synthetic suffixes and vacillate before consonant-initial suffixes.
- **Conditions**: stem `derék` (bound alternant `derek-`) or `férfi`.
- **Output**:
  - Vowel-initial suffixes (plural, accusative, possessives, `-As`): back only: `derekak`, `derekat`, `derekam`, `derekunk`, `derekas`; `férfiak` [ATT 18], `férfias`.
  - Consonant-initial suffixes: `derékban`~`derékben`, `derékba` [ATT 4]~`derékbe` [ATT -]; `férfinak` [ATT 11]~`férfinek` [ATT 7], `férfiba` [ATT 2]~`férfibe`; Hayes-Londe dative share of back: `derék` 0.62 (data file flags contamination by the suffix `-ék`), `férfi` 0.80.
  - `-ig` behaves as consonant-initial; superessive `derékon` back only; `-ság`: `derékség` more common than `derékság`.
- **Examples**: see Output.
- **Exceptions**: `-`
- **Predictability**: `lexical` (2 items)
- **Lexical input**: stem harmony per suffix type: `harmony_V=B`, `harmony_C=vacillating(p_back)`.
- **Interactions**: `PH-HARM-015`, `PH-LEN-007` (`derék` shortens).
- **Sources**: [RST12 §3.2 p. 495, §4.2 pp. 498-499]; [HL06 data file]; [ATT].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### PH-HARM-010 Harmony of truncated and diminutive stems
- **Statement**: When a vowel-zero or truncation process removes the back vowel of a `NB` stem, the verb-forming `-ít` (and other synthetic derivations of the same root) keeps the harmony of the full stem, whereas the hypocoristic `-i` recomputes harmony from the remaining vowels.
- **Conditions**: `-ít`, `-ul`, `-odik` derivatives with stem-final vowel loss or epenthetic vowel loss; hypocoristic names and verbs with `-i`.
- **Output**: `-ít` etc.: harmony of the underived stem; hypocoristic `-i`: harmony computed on the truncated string + `i`.
- **Examples**: `tiszta`->`tisztít`->`tisztítok`; `sima`->`simít`->`simítok`; `béna`->`bénít`->`bénítok`; `piszok`->`piszkít`->`piszkítok`; `ifjú`->`ifjít`->`ifjítok`; `indul`/`indít`->`indítok`; `sikolt`/`sikít`->`sikítok`; front: `dísz`->`díszítek`, `béke`->`békítek`; hypocoristic: `Éva`->`Évi`->`Évinek`, `Simon`->`Simi`->`Siminek`, `Miklós`->`Miki`->`Mikinek`, `Ferenc`->`Feri`->`Ferinek`, `Ilona`->`Ili`->`Ilinek`, `Nándor`->`Nándi`->`Nándinak`; verb `simogat`->`simiz`->`simizek`; `tapogat`->`tapiz`->`tapizok`.
- **Exceptions**: `-`
- **Predictability**: `rule` (given the morphological source)
- **Lexical input**: derivational source stem.
- **Interactions**: `PH-ZERO-007`, `DER`.
- **Sources**: [RST12 §5.2 (11)-(13) pp. 501-503, §6.4 pp. 506-507]; [T04 fn. 3 p. 140].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-HARM-011 Zone of variation: back vowel followed by neutral vowels
- **Statement**: Stems whose last non-neutral vowel is `B` and which end in one or two neutral vowels (`BN`, `BNN`) have lexically determined harmony: back, front, or vacillating; the distribution is statistically structured by the height and number of the final neutral vowels (`PH-HARM-012`).
- **Conditions**: domain vowel string matches `...B N` or `...B N N`.
- **Output**: lexical value per stem (`B`, `F`, or vacillating with `p_back`); default from `PH-HARM-013`.
- **Examples**: back: `papír`->`papírnak` (1.00), `radír`, `forint`, `haver`->`havernak` (0.93), `tányér`->`tányérnak` (1.00), `kastély`->`kastélynak` (1.00), `poén`->`poénnak` (0.99), `balek` (0.82), `matek` (0.96); front in the dative data: `modern`->`modernnek` (0.00), `kabinet`->`kabinetnek` (0.00), `oxigén`->`oxigénnek` (0.00), `mágnes` (0.01), `analízis`->`analízisnek` (0.00), `harakiri` (0.00) (RST12 (5c), (6a-b) and KVF98 p. 421 report `mágnes`, `oxigén`, `analízis`, `harakiri` as vacillating across suffixes); vacillating: `fotel` (0.16), `hotel` (0.17), `dzsungel` (0.15), `bohém` (0.38), `domén` (0.10), `honvéd` (HZSL09 (9): 0.74; not in the revised data file), `agresszív` (0.38), `klarinét` (0.90), `diadém` (0.73). Figures: Hayes-Londe share of `-nak`.
- **Exceptions**: morphologically complex stems follow `PH-HARM-018`, `PH-HARM-019`. Vacillating stems take both harmonies by the norm and the lexicon marks them so (`norvégok` ~ `norvégek`, `Athénban` ~ `Athénben`); where one form is the usual one it is generated and the other only accepted: `szlovének` (preferred) ~ `szlovénok` [ENY-SZLOVEN].
- **Predictability**: `tendency` (see statistics in `PH-HARM-012`; lexical per stem)
- **Lexical input**: stem harmony value, optionally per suffix type (`PH-HARM-015`).
- **Interactions**: `PH-HARM-012`..`PH-HARM-016`, `PH-HARM-021`.
- **Sources**: [HZSL09 (9) p. 11, §5.1]; [HL06 Table I p. 66, data file]; [KVF98 p. 421]; [RST12 (5)-(6) pp. 496-497]; [RR24 §1]; [ENY-SZLOVEN].
- **Validation**: `attestation`, `oracle:hunmorph-foma`, `oracle:magyarispell`.
- **Confidence**: `tendency`.

### PH-HARM-012 Height and count effects
- **Statement**: In the zone of variation the probability of back harmony falls with the height of the last neutral vowel (`i`/`í` > `é` > `e`) and with the number of final neutral vowels (`BN` > `BNN`); the height effect is smaller in `BNN` and the within-`BNN` height ordering is **not established**: recomputed from the source files it is `BNí` 0.358 > `BNé` 0.316 > `BNi` 0.209, on 8, 16 and 32 stems - the non-monotonicity survives but with a different ordering, and on 8/16/32 stems it is not evidence of anything. `BNe` stems are front only.
- **Conditions**: `BN`, `BNN` stems.
- **Output**: (Hayes-Londe 2006, Table I and II; backness index; number of stems)

  | Type | Back | Vacillating | Front | Total | Backness index |
  |---|---|---|---|---|---|
  | `B` | 6251 | 39 | 0 | 6290 | 0.999 |
  | `BN` (all) | 603 | 78 | 83 | 764 | 0.831 |
  | `Bi` | 458 | 17 | 0 | 475 | 0.989 |
  | `Bí` | 52 | 0 | 1 | 53 | 0.980 |
  | `Bé` | 93 | 18 | 9 | 120 | 0.845 |
  | `Be` | 0 | 43 | 73 | 116 | 0.104 |
  | `BNN` (all) | 6 | 21 | 44 | 79 | 0.185 |
  | `BNi` | 1 | 12 | 17 | 32 | 0.209 |
  | `BNí` | 1 | 7 | 0 | 8 | 0.358 |
  | `BNé` | 4 | 2 | 6 | 16 | 0.316 |
  | `BNe` | 0 | 0 | 21 | 23 | 0 |
  | `N` | 14 | 23 | 259 | 296 | 0.078 |
  | `NN` | 0 | 4 | 939 | 933 | 0.002 |
  | `R` | 0 | 0 | 1517 | 1517 | 0 |
  | height: high (`Bi Bí BNi BNí`) | 512 | 36 | 18 | 566 | 0.938 |
  | height: mid (`Bé BNé`) | 97 | 20 | 15 | 132 | 0.806 |
  | height: low (`Be BNe`) | 0 | 43 | 94 | 137 | 0.089 |

  Token-weighted `-nak` share beside the type-weighted backness index (uncurated file, 2026-09-16): `B` 0.9997, `Bi` 0.9986, `Bí` 0.9618, `Bé` 0.9750, `Be` 0.0589, `BNi` 0.0305, `BNí` 0.6993, `BNé` 0.6343, `BNe` 0.0004, `BNN` 0.2050, `N` 0.0341, `NN` 0.0060. The two columns diverge by more than 0.2 in `Bé`, `BNi`, `BNé` and `N`, because token weighting is carried by a few high-frequency stems. The `NN` row counts stems with **exactly two** neutral vowels; a further 1,095 all-neutral stems have 3-4 neutral vowels and are not in it (778 with 3, 317 with 4). The `R` row is every stem whose last non-neutral vowel is front rounded in the uncurated file (1,517; the previously printed 698 could not be reproduced from either file). **Every class except `NN` and `B` rests on fewer than 300 stems and is provisional**; the `BNN` sub-classes worst of all (`BNí` 8 stems, `BNé` 16, `BNe` 23, `BNi` 32), so `BNí` = 0.70 must not be read as a stable prior.

  "Back"/"front" = at least 97% of hits; vacillating otherwise. Secondary effects noted but not modelled: a weaker height effect of the first `N` in `BNN`, and lower back rates after `BBN` than after `BN` [HZSL09 fn. 10]. The wug test of Hayes and Londe showed both effects with a monotonic height effect also in `BNN` (speakers smooth the small aberrant `BNé` class) and a significant Height x Count interaction [HL06 §4, §4.3].
- **Examples**: `Bi`/`Bí`: `papír`, `radír`, `forint`; `Bé`: `kávé`->`kávénak` [ATT 3], `kávéval` [ATT 7], `poén`; `Be`: `fotel`, `hotel` [`hotelben` ATT 10 vs `hotelban` ATT 2]; `BNi`: `bakelit`, `harakiri`, `analízis`, `hipotézis`; `BNé`: `Athéné`, `klarinét`; `BNe`: `kabinet`, `parizer`, `barométer`, `Honegger` (front only).
- **Exceptions**: corpus data [F13] confirm height and count effects but find them weaker in `BNN`; disyllabic `Be` stems with `a` as the back vowel and informal German/Yiddish loans lean back (`haver`, `balek`, `matek`) (`PH-HARM-025`).
- **Predictability**: `tendency`
- **Lexical input**: `none` (statistics feed the default)
- **Interactions**: `PH-HARM-013`, `PH-HARM-016`.
- **Sources**: [HL06 Tables I-II pp. 66-67, §4.3]; [HZSL09 (12)-(15) pp. 12-14, fn. 10]; [RST12 §3.2 (iii)-(iv) pp. 496-497]; [F13 theses §3.2.1]; [research-lit `HungarianFromGoogle.txt` (uncurated, 10,974 rows) and `HungarianFromGoogleRevised.txt` (curated, 8,915 rows), recomputed 2026-09-16]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `attestation`; the table's provenance is now fixed - it is the **uncurated** file and the backness index is the unweighted type mean of the `-nak` fraction, reproduced to three decimals for all 13 rows.
- **Confidence**: `tendency`; the within-`BNN` height ordering is `not established` (8-32 stems per class).

### PH-HARM-013 MaxEnt default for stems without lexical harmony
- **Statement**: For a stem in a zone of variation whose harmony is not in the lexicon, the probability of back harmony is computed with the maximum-entropy grammar of Hayes, Zuraw, Siptár and Londe: P(back) = 1 / (1 + exp(H_back - H_front)), where H of a candidate is the sum of the weights of the constraints it violates.
- **Conditions**: stem class not decided by `PH-HARM-003`, `PH-HARM-004`, `PH-HARM-007` or the lexicon; i.e. `BN`, `BNN` stems and 1-2-syllable all-neutral stems.
- **Output**: violations (v_n = last vowel, v_n-1 = previous vowel, c = closest non-neutral vowel, final = pronounced final consonant(s)):

  | Constraint | Violated by | Condition | Weight L (lexicon) | Weight W (wug) |
  |---|---|---|---|---|
  | AGREE(back, local) | front candidate | v_n in `B` | 3.95 | n/a |
  | AGREE(back, nonlocal) | front candidate | c in `B` | 5.39 | 3.59 |
  | AGREE(front rounded, local) | back candidate | v_n in `R` | 3.72 | n/a |
  | AGREE(front rounded, nonlocal) | back candidate | c in `R` | 1.70 | n/a |
  | AGREE(front, local) | back candidate | v_n front (`N` or `R`) | 1.69 | 2.24 |
  | AGREE(non-high front, local) | back candidate | v_n in `é e ö ő` | 1.46 | 0.95 |
  | AGREE(low front, local) | back candidate | v_n = `e` | 3.00 | 1.06 |
  | AGREE(double front, local) | back candidate | v_n-1 and v_n both front | 4.04 | 1.94 |
  | USE FRONT / bilabial | back candidate | final C in `p b m` | 2.45 | 1.03 |
  | USE FRONT / coronal sonorant | back candidate | final C in `n ny l r` | 1.08 | 0.44 |
  | USE FRONT / sibilant | back candidate | final C in `sz z s zs c cs dzs` | 0.91 | 0.37 |
  | USE FRONT / CC | back candidate | stem ends in two consonants, geminates included | 1.73 | 0.67 |
  | USE BACK / monosyllable with í | front candidate | stem is one syllable with vowel `í` | 2.42 | 0.81 |

  Worked values (Grammar L; Hayes-Londe observed back share in parentheses): `papír` Hb 2.77, Hf 5.39, P 0.93 (1.00); `poén` Hb 4.23, Hf 5.39, P 0.76 (0.99); `haver` Hb 7.23, Hf 5.39, P 0.14 (0.93); `fotel` P 0.14 (0.16); `dzsungel` P 0.14 (0.15); `kódex` (final /ks/ = CC and sibilant) Hb 8.79, P 0.03; `analízis` Hb 6.64, P 0.22 (0.00); `bakelit` Hb 5.73, P 0.42; `Athéné` Hb 7.19, P 0.14; `diadém` Hb 5.60, P 0.45 (0.73); `híd` Hb 1.69, Hf 2.42, P 0.68 (1.00); `víz` Hb 2.60, Hf 2.42, P 0.46 (0.00); `kert` Hb 7.88, Hf 0, P 0.0004.
- **Examples**: see worked values; Grammar L correlates r = .992 with the lexicon (r = .892 on `BN(N)` and `N` stems) and r = .546 with wug judgements; Grammar W (fitted to the wug test) r = .575; natural constraints only r = .521.
- **Exceptions**: the model is a default for unknown stems; for listed stems the lexicon wins (`haver` back, `víz` front). The consonant constraints are contested (`PH-HARM-014`).
- **Predictability**: `tendency`
- **Lexical input**: pronounced vowel string and final consonants; lexicon harmony overrides. Where the stem is a loan with a neutral final vowel and a back vowel earlier, the per-item `p_back` table `neutral-final-loan-harmony.tsv` (162 stems: 8 back-dominant, 51 vacillating, 103 front, in `local/lists/neutral-final-loan-harmony.tsv` - working data, not yet read by the build) overrides the model. This is the first item-level figure the project has for this class.
- **Interactions**: `PH-HARM-011`, `PH-HARM-012`, `PH-HARM-014`, `PH-HARM-015`, `PH-HARM-021`.
- **Sources**: [HZSL09 (16) p. 17, (18)-(19) pp. 20, (31) p. 35, (33)-(34) pp. 37-38, §9.5 pp. 34-36]; [HL06 data file for observed shares]; [Racz-DS `Racz2027vh` `dzsungel.tsv`, 162 neutral-final loan stems, MIT]; [ATT evidence/lexical-lists.md 2026-09-16]; [ATT evidence/calibration.md 2026-09-16] (the 13 MaxEnt weights are not re-fitted here: that is a separate exercise and the 2026-09-16 measurement does not constrain them individually).
- **Validation**: `attestation` (re-fit weights on the project corpus with the same constraint set), `goldens` (known stems).
- **Confidence**: `tendency`. Recommendation: use Grammar L to predict corpus/lexicon behaviour of unlisted existing words; use Grammar W (flatter) for nonce words and when generating variant sets for novel loans.

### PH-HARM-014 Consonant effects on harmony (contested)
- **Statement**: In the Hayes-Londe lexicon, stems in the zones of variation ending in a bilabial stop or `m`, a sibilant, a coronal sonorant or a consonant cluster take front suffixes more often; the effects are learned by speakers in a wug test but a later national-corpus study replicated only the cluster effect and only for disyllabic `e`-final stems.
- **Conditions**: zone-of-variation stems; final consonant classes per `PH-CONS-003`.
- **Output**: (HZSL09 lexical statistics, zones of variation only, recomputed on the curated file 2026-09-16) bilabial: 41 stems, 87.8% take the predicted front value; sibilant: 138 stems, 65.2% - **not distinguishable from the 67.1% of the 231 zone stems ending in any other single consonant**; coronal sonorant: 183 stems, 66.7% - **likewise not distinguishable from 67.1%**; CC: 168 stems, 94.0%. Baseline for all 699 zone stems (`BN`, `BNN`, `N`): **39.2% back / 60.8% front** - the rulebook's earlier "40.3% front" had the sign of the baseline wrong, and it is the sign that the consonant effects are measured against. The apparent sibilant and coronal-sonorant effects were an artefact of a pooled baseline that mixes in 106 vowel-final zone stems at 0.208 and so reads much lower than the correct comparison class. The CC (0.940) and bilabial (0.878) effects are real and large against 0.671; bilabial rests on 41 stems and is provisional. Within-class contrast, confirming Forró (2013): `Be` single C 69 stems 0.129 vs CC 44 stems 0.003; `BNN` single C 49 stems 0.191 vs CC 9 stems 0.000; `Bi` single C 98 stems 0.998 vs CC 9 stems 1.000 - the CC effect is strong in disyllabic `e`-final stems and absent at ceiling in the high-vowel classes. monosyllables with `í`: 33 stems, 47.1% back vs 7.3% back for all 259 monosyllabic `N` stems. Front share without vs with at least one consonant environment: `B`+high 0.03 vs 0.03 (97, 85 stems), `Bé` 0.08 vs 0.35 (43, 32), `Be` 0.34 vs 0.91 (4, 109), `BNN` 0.74 vs 0.88 (27, 36), `N` 0.84 vs 0.96 (68, 191). Logistic regression coefficients (towards back): bilabial -2.356, coronal sonorant -1.290, sibilant -1.210, CC -1.266 (p = .057), monosyllable with `í` +3.187.
- **Examples**: `diadém` (bilabial) vs `honvéd`; `kódex`, `modern` (CC); `hotel`, `fotel` (coronal sonorant).
- **Exceptions**: Forró (2013), 1000 stems, 21 suffixes, Hungarian National Corpus: no significant bilabial, sibilant or coronal sonorant effect; CC effect only in disyllabic `e`-final stems; filled vs empty coda matters in some types (disyllabic `é`-final).
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: `PH-HARM-013`.
- **Sources**: [HZSL09 (18)-(22) pp. 20-23]; [F13 theses §3.2.2]; [RR24 §1 (reports HZSL09 consonant effects as input features)]; [research-lit `HungarianFromGoogleRevised.txt` (curated, 8,915 rows), recomputed 2026-09-16]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `attestation`; measured 2026-09-16 against the correct comparison class (zone stems ending in any other single consonant, 231 stems).
- **Confidence**: `contested` for sibilant and coronal sonorant, whose claims are now **withdrawn**; `measured` for CC (0.940) and `tendency` for bilabial (0.878, 41 stems, provisional). Default configuration unchanged: **enable CC only**, now measured rather than assumed; bilabial, sibilant and coronal sonorant constraints stay off.

### PH-HARM-015 Vacillation is per stem and per suffix type
- **Statement**: A vacillating stem does not have one back probability for all suffixes: suffixes with a linking vowel or with `o`/`e`/`ö` or `ó`/`ő` alternation vacillate less than suffixes with a stable vowel or `a`/`e` alternation [F13], and vowel-initial suffixes amplify the stem's dominant tendency (back for `haver` type, front for `kódex` type) [RR24]; most vacillators have a dominant variant, and strong 50/50 vacillation is rare.
- **Conditions**: stem in the zone of variation or antiharmonic disyllable.
- **Output**: lexicon stores `p_back` for at least two suffix groups: `V` (vowel-initial and linking-vowel suffixes) and `C` (consonant-initial); generation emits all variants with share between 0.03 and 0.97, ordered by share.
- **Examples**: `férfiak` (back only) vs `férfinak`~`férfinek`; `mágneses` (front only) vs `mágnesnek`, with `?mágnesnak` below the 0.03 floor this rule sets (`mágnesnek` 966 : `mágnesnak` 4, 0.4% [WC2], and `PH-HARM-011` gives `mágnes` 0.01); `derekak` vs `derékba`~`derékbe` (6,498 : 142, 2.1% [WC2]); `tányérok` vs `tányérral`, with `?tányértől`, `?tányérből`, `?tányérről` likewise below the floor (`tányérból` 2,795 : `tányérből` 18; `tányérról` 1,805 : `tányérről` 11; `tányértól` 219 : `tányértől` 0 [WC2]) where the 2006 web hits reported them.
- **Exceptions**: in the Rácz-Rebrus sample (New Hungarian Webcorpus) of 200 disyllabic consonant-final `B`+`e` noun stems and 30 suffixes, 164 show back/front variation in at least one suffixed form; similarity to other stems (nearest neighbours) predicts the rate, and vowel-initial suffixes amplify the similarity effect.
- **Predictability**: `tendency`
- **Lexical input**: `p_back_V`, `p_back_C` per stem (default from `PH-HARM-013`, same value for both groups unless attested otherwise).
- **Interactions**: `PH-HARM-009`, `PH-HARM-013`, `NOM-MODEL`.
- **Sources**: [F13 theses §3.1]; [RR24 abstract, §1 p. 96, §2.2]; [RST12 §6.2 p. 504, fn. 9 p. 496]; [HL06 §3 (97% threshold)].
- **Validation**: `attestation`.
- **Confidence**: `tendency`.

### PH-HARM-016 Lengthened final e is more transparent than short e
- **Statement**: A `B`+`e`-final stem whose `e` is lengthened to `é` before a suffix (`PH-LEN-001`) behaves like a `Bé` stem and allows back suffixes more readily than before a non-lengthening suffix, where `e` stays short and front is typical.
- **Conditions**: stem ends in `B` ... `e`; suffix lengthening vs super-analytic.
- **Output**: lengthening suffixes: vacillation (`dózsénak`~`dózsének`, `Hallében`~`Halléban`, `dafkéból`~`dafkéből`); non-lengthening `-ság/-ség`: front (`dózseség`, `Halleség`, `dafkeség`).
- **Examples**: see Output.
- **Exceptions**: few examples; adjectival `-i` adds a neutral vowel and is front (`hallei`).
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: `PH-LEN-001`, `PH-LEN-002`, `PH-HARM-013` (compute on the lengthened vowel).
- **Sources**: [RST12 §4.1 p. 498].
- **Validation**: `attestation`.
- **Confidence**: `tendency`.

### PH-HARM-017 Disharmonic roots
- **Statement**: Roots containing both back and front rounded vowels (loans) are allowed; harmony follows the last non-neutral vowel (`PH-HARM-003`, `PH-HARM-004`); no root-internal harmony is enforced by the generator.
- **Conditions**: mixed `B` and `R` vowels in one morpheme.
- **Output**: class from the last non-neutral vowel.
- **Examples**: `sofőr`->`sofőrnek`, `kosztüm`->`kosztümnek`, `kajüt`->`kajütnek`, `nüansz`->`nüansznak`, `glükóz`->`glükóznak`, `büró`->`bürónak`, `amőba`->`amőbának`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-HARM-003`, `PH-HARM-004`.
- **Sources**: [KVF98 §3.2.6.4.1.5 pp. 421-422]; [HZSL09 (6)-(7) pp. 9-10]; [RR24 Table 2].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-HARM-018 Front alternant of an alternating suffix is opaque
- **Statement**: When a harmonic suffix has been realised with its front alternant (`e`, `é`, and the front member `-i` of `-ja/-i`), every following suffix is front, even if the stem was back-vacillating; the neutral vowel of an alternating suffix never passes back harmony through.
- **Conditions**: suffix vowel realised by `A`, `Á`, `O` (as `e`), or suppletive `-ja/-i` front member; next suffix harmonic.
- **Output**: state set to `F` (or `R` if realised `ö`, `ő`, `ü`, `ű`).
- **Examples**: vacillating `fotel` (`fotelja` 258 : `fotelje` 276 [WC2]): `fotelja`->`foteljának`, `fotelje`->`foteljének` (not `*foteljénak`, 0 [WC2]); `hotelje`->`hoteljének`. A stem of one class shows no contrast: back `haver`->`haverjának` (`haverja` 10,264 : `haverje` 15 [WC2]); front `alibi`->`alibijének` (`alibije` 920 : `alibija` 4 [WC2]); RST12's own examples are judgements, not measurements: `?agilisság`->`?agilisságnak`, `?agilisség`->`?agilisségnek` (7 and 3 tokens [WC2]), not `*agilisségnak`; the nonce verb `#martinizza`->`#martinizzátok`, `#martinizi`->`#martinizitek`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-HARM-019`, `PH-HARM-021`.
- **Sources**: [RST12 §5.1 (8)-(10) pp. 500-501].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-HARM-019 Invariant suffixes with neutral vowels are transparent
- **Statement**: Non-alternating suffixes with `i`, `í` or `é` do not change the harmony of the string to their left; a following harmonic suffix is chosen as if the invariant suffix were absent, except that `-i`, `-é`, `-ék` add neutral vowels that may shift a back stem into vacillation, while `-ít` never changes harmony.
- **Conditions**: invariant suffix set: `-i` (adjectival and possessed plural), `-ít`, `-é` (anaphoric possessive), `-ék` (associative), `-né` (Mrs), `-ni` (infinitive), `-ig`, `-ért`, `-ként`, `-nként`, `-nyi`, 3sg `-ik`; `-beli` and `-képp(en)` are not in the set (Exceptions).
- **Output**:
  - `-ít`: harmony of the base, categorical: `hamisítok` (`hamis` B), `számítok`, `tisztítok`.
  - `-i`, `-é`, `-ék` after a `B` string ending in `B`: back (`kubainak`, `házénak`, `Nagyéknak`); after a `B`+`N` string: vacillation grows (`karibinak`~`karibinek` while `karibnak` only back; `Sáriénak`~`?Sáriének`, `Tomiéknak`~`?Tomiéknek`).
  - `-né`: induces vacillation even after stable stems [F13 §3.2.1, no item data]; illustrative (unattested, not from F13): `Szabónénak` ~ `?Szabónének`.
  - After front stems: front.
- **Examples**: `kutyáival`, `cipőivel` [KVF98 p. 423]; `martinisítok`~`martinisítek` (the base `martinis` already vacillates; `-ít` passes the variation on) [RST12 (9b)].
- **Exceptions**: `-ék` is transparent in corpus data despite its mid vowel `é` [F13 §3.2.1]; RST12 (9b) nonetheless marks `Tomiéknak`~`?Tomiéknek`. `-beli` and `-képp(en)` end in low `e`, which is never transparent [RST12 p. 496], and are opaque front: following suffixes are front (`falubeliek`; `DER-MODEL-008` class `F!`) [morphdb AFF_BELI `OUT: front`].
- **Predictability**: `rule` for `-ít` and for `B`-final bases; `tendency` for strings ending in several neutral vowels.
- **Lexical input**: suffix property `transparency` (`transparent` | `transparent_counting` | `vacillation_inducing`).
- **Interactions**: `PH-HARM-018`, `PH-HARM-021`, `NOM-ANP`, `NOM-ASSOC`, `NOM-PPL`.
- **Sources**: [RST12 §5.1 (9) p. 500, §6.2 p. 505, p. 496 (low `e` never transparent)]; [KVF98 (75) pp. 422-423]; [F13 theses §3.2.1]; [morphdb grammar AFF_BELI].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `consensus` for `-ít` and `B`-final bases; `tendency` otherwise.

### PH-HARM-020 Invariant suffixes with back vowels are opaque
- **Statement**: A few non-alternating suffixes have only a back vowel and impose back harmony on what follows, whatever the stem: `-kor` (temporal), `-ista`, `-izmus`, `-ikus` and other Latinate suffixes ending in a back vowel; the foreign verbalisers `-ál`, `-íroz`, `-izál`, `-ifikál` are back after foreign bases.
- **Conditions**: suffix in the list.
- **Output**: state `B` after the suffix.
- **Examples**: `csellistának`, `szlavistának`; `nyolckor`, `ötkor`->`ötkori`; `idealizmusnak`; `regisztrál`->`regisztrálok` (2,299 [WC2]), `szelektál`->`szelektálnak`; `?recenzeál`->`?recenzeálok` (the spelling MTA-HELYES gives; the cell is marginal in WC2, 1 against `recenzálok` 6); `diszponál`.
- **Exceptions**: `-ál/-ol/-oz` have rare front partners after native front bases (`-él`, `-el`, `-ez`) (`PH-HARM-024`).
- **Predictability**: `lexical` (closed set of suffixes)
- **Lexical input**: suffix property `harmony=B_fixed`.
- **Interactions**: `PH-HARM-021`, `DER`.
- **Sources**: [KVF98 (75c)-(75d) p. 423]; [SMNy3 Rebrus §13.2.1.4 p. 682].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-HARM-021 Harmony automaton
- **Statement**: The harmony class after any prefix of a word form is computed left to right by a finite-state procedure over domain vowels and suffix properties; the class chooses the realisation of every archiphoneme (`PH-HARM-002`).
- **Conditions**: domain from `PH-HARM-001`; stem lexicon entry may carry `harmony` (`B` | `F` | `R` | `vacillating(p_back_V, p_back_C)`).
- **Output**: algorithm.
  1. Stem: if the lexicon gives harmony, use it. Otherwise scan the stem vowels from right to left:
     - last vowel `B` -> `B`;
     - last vowel `R` -> `R`;
     - last vowel `N`: skip neutral vowels to the left; count k = number of final neutral vowels and record the height of the last one;
       - first non-neutral found is `R` -> `F`;
       - first non-neutral found is `B` -> zone `BN^k`: `p_back` from `PH-HARM-013` (k >= 3 is treated as `BNN`), output `vacillating(p_back)` or, in single-answer mode, `B` if p_back >= 0.5 else `F`;
       - none found: syllables >= 3 -> `F`; syllables 1-2 -> `F` unless lexically antiharmonic (`PH-HARM-008`, `PH-HARM-009`).
  2. For each suffix left to right:
     - alternating suffix: realise its archiphonemes from the current class (choosing the variant set for vacillating classes); the new class is `B` if realised back; `R` if its last realised vowel is `ö ő ü ű`; `F` if its last realised vowel is `e é i í` (opaque front, `PH-HARM-018`);
     - invariant `B`-vowel suffix: class `B` (`PH-HARM-020`);
     - invariant neutral suffix: class unchanged if the current class is `B` or `F`; if the current class is `R` it becomes `F` (the last vowel is now neutral, so `O` -> `e`: `föld`+`-i` -> `földihez`); for `B` classes after `-i`/`-é`/`-ék` on a string that already ends in a neutral vowel, recompute with `PH-HARM-013` counting the added vowel (`PH-HARM-019`); `-ít` never recomputes.
  3. Rounding: `O` is `ö` iff class is `R` (`PH-HARM-005`).
- **Examples**: `ház`: `B` -> `házaknak`; `tök`: `R` -> `tökökhöz`; `üveg`: last `N`, first non-neutral `ü` -> `F` -> `üveghez`; `föld`+`-i` -> `földi` (`F`) -> `földihez`; `papír`: lexicon `B` -> `papírokhoz`; `agilis`+`-ság` realised `-ség` -> `F` -> `agilisségnek`; `Kuba`+`-i` -> `B` -> `kubaihoz`; `híd`: lexicon `B` -> `hidakhoz`; `szintézis`: `NNN` -> `F` -> `szintézishez`.
- **Exceptions**: `-nOk`, `-OmÁny`, `-OvÁny` (`PH-HARM-006`); lowering linking vowel `A` for `O` (`NOM-LOW`); diminutive `-i` recomputation (`PH-HARM-010`).
- **Predictability**: `rule` (with `tendency` inputs)
- **Lexical input**: stem `harmony`, suffix properties `alternating`, `harmony=B_fixed`, `transparency`, `o_alternation`.
- **Interactions**: all `PH-HARM` rules; consumed by `NOM`, `VRB`, `DER`.
- **Sources**: [emMorph `vhrm.pl` three-state design (study only, GPL/CC BY-NC-SA)]; [RST12]; [HZSL09]; [KVF98 pp. 420-423].
- **Validation**: `goldens`, `oracle:hunmorph-foma`, `oracle:emMorph`.
- **Confidence**: `consensus`.

### PH-HARM-022 Compounds and foreign pseudo-compounds
- **Statement**: Native compounds harmonise with the last member; foreign words built from recognisable Greek-Latin pieces are treated by speakers as compounds to a variable degree, so stems ending in pieces such as `-méter`, `-gén`, `-lízis` or containing prefixes in `-o` (`hidro-`, `hetero-`, `bio-`) lean front and vacillate independently of the height and count effects.
- **Conditions**: native compounds (`DER` boundary) -> last member; foreign pseudo-compounds (no productive boundary in the lexicon) -> vacillation class with front bias.
- **Output**: native: categorical by last member; foreign: vacillating with `p_back` lowered relative to `PH-HARM-013`. **Direction confirmed, magnitude not measurable, provisional - do not hard-code a weight**: only 15 zone stems in the Hayes list end in a Greek-Latin piece (`-méter`, `-gén`, `-lízis`, `-tézis`, `-gráf`, `-lógia`), type-mean `-nak` 0.023 against 0.118 for the other 2,380 zone + `N`/`NN` stems; 0 stems begin with `bio-`, `hidro-` or `hetero-`.
- **Examples**: `kódexmásolóhoz` (last member `másoló`); `barométernek`, `oxigénnek` [ATT 6 `oxigénnel`]; `analízisnek`; `Honeggernek`.
- **Exceptions**: first members in short `o` keep it short and disharmonic (`pszichobiológia`) [T04 p. 79].
- **Predictability**: `rule` (native); `tendency` (foreign)
- **Lexical input**: compound boundary; foreign piece list (lexical, to be compiled).
- **Interactions**: `PH-HARM-001`, `PH-HARM-013`, `DER`.
- **Sources**: [F13 theses §3.3]; [KVF98 p. 420]; [RST12 fn. 5]; [research-lit, recomputed 2026-09-16]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `attestation`.
- **Confidence**: `consensus` (native); `tendency` (foreign), provisional on 15 stems.

### PH-HARM-023 Harmony of words read by pronunciation
- **Statement**: Foreign-spelled words, names, acronyms, letters, numerals and symbols harmonise with the vowels of their pronounced form (`PH-G2P`), not of their spelling.
- **Conditions**: stem with foreign spelling, digit, symbol or letter-by-letter acronym.
- **Output**: harmony computed on the pronunciation string.
- **Examples**: `Birmingham` [bɜrmiŋɡɛm] -> front; `Goethe` [ɡøːtɛ] -> `Goethének`; `Glasgow` [ɡlɛzɡoː] -> `Glasgow-ban`; `guillotine` [ɡijotin] -> `guillotine-nal`; `Molière` [molijɛːr] -> `Molière-rel`; `Montesquieu` -> `Montesquieu-nek`; `5` (`öt`) -> `5-höz`; `MTA` (`em-té-á`) -> `MTA-nak`; `%` (`százalék`) -> `%-kal`.
- **Exceptions**: `-`
- **Predictability**: `rule` (given the pronunciation)
- **Lexical input**: pronunciation (`PH-G2P-008`, `PH-G2P-010`..`PH-G2P-013`).
- **Interactions**: `PH-G2P`, `ORT-FOR`, `ORT-ABBR`.
- **Sources**: [AkH12 §216-§217, §282, §288]; [HL06 §3 (Birmingham)].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-HARM-024 Harmony of loan verbalisers
- **Statement**: Verbs formed from loan bases with `-ol`/`-oz` (and `-ál`, `-íroz`) often show antiharmony (back suffix after an all-neutral base) or counterharmony (back suffix after a front rounded or neutral base against the expected front variant); the choice is speaker-dependent.
- **Conditions**: loan base + verbaliser with an `O` vowel.
- **Output**: variants: harmonic and non-harmonic, speaker-dependent; generator lists both where attested.
- **Examples**: antiharmony: `brillíroz`, `fixíroz`, `síbol`, `slisszol`; counterharmony (a back vowel after `e`/`ü`, or a rounded vowel after a neutral base): `csekkol`, `gründol`, `curükkol`, `stíröl`; informal antiharmonic `lejmol` [F13]; harmonic: `blöfföl`, `sztrájkol`.
- **Exceptions**: `-`
- **Predictability**: `lexical`
- **Lexical input**: verb entry with verbaliser variant(s).
- **Interactions**: `DER`, `VRB-STEM`.
- **Sources**: [RST24 p. 163 (l. 1568-1572)]; [F13 theses §3.3 (`lejmol` type)].
- **Validation**: `attestation`.
- **Confidence**: `contested`.

### PH-HARM-025 Informal back-leaning loans
- **Statement**: Loans of German/Yiddish or slang origin with informal style value take back suffixes more than their vowel shape predicts: disyllabic `B`+`e` stems (`balek` type) and monosyllabic `e` stems (`spejz` type); the verbal counterpart is the `lejmol` type (`PH-HARM-024`).
- **Conditions**: `Be` or `e` stems with informal register marking.
- **Output**: raise `p_back`; lexical.
- **Examples**: `haver` (0.93 back), `balek` (0.82), `matek` (0.96); `spejz` (back, no Hayes-Londe or corpus figure).
- **Exceptions**: `-`
- **Predictability**: `tendency`
- **Lexical input**: stem register or explicit `p_back`.
- **Interactions**: `PH-HARM-013`, `PH-HARM-015`.
- **Sources**: [F13 theses §3.2.2, §3.3]; [HL06 data file].
- **Validation**: `attestation`.
- **Confidence**: `tendency`.

---

## LEN: vowel length alternations

### PH-LEN-001 Final a and e lengthening
- **Statement**: A domain-final short `a` or `e` is long (`á`, `é`) whenever any suffix other than a super-analytic one follows; the process is fully productive (loans, names, nonce words, word-like acronyms) and applies again to suffix-final `a`/`e`.
- **Conditions**: stem or suffix string ends in short `a`/`e`; next morpheme is a suffix not in `PH-LEN-002`; not a compound member boundary.
- **Output**: `a`->`á`, `e`->`é`; harmony recomputed on the long vowel where relevant (`PH-HARM-016`).
- **Examples**: `alma`->`almát`, `almák`, `almával`, `almáig`, `almáé`, `almául`, `almástul`, `almánként`; `kefe`->`kefék`, `keféje`, `keféi`, `kefében`; `ad+na`->`adnák`, `adnátok`; `ad+ja`->`adják`; `Uppsala`->`Uppsalába`; `kóla`->`kólám`; `áfa`->`áfát` (lowercase word-read acronym); `kefetartó` (compound: no lengthening); all-caps proper acronyms are pronounced long but written short (`FIFA-nak`, `PH-LEN-003`).
- **Exceptions**: `PH-LEN-002`, `PH-LEN-003`; possessed plural `-i` after `-(j)A` (`lapjai`, `NOM-LENG-002`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `NOM-LENG-001`, `NOM-LENG-004`, `PH-HARM-016`, `PH-VOW-003`.
- **Sources**: [AkH12 §29]; [SMNy3 Rebrus §13.6.1.1 (187)-(188) pp. 786-788]; [KVF98 §3.4.1.3.1.2 p. 447]; [T04 §3.4.1 pp. 110-112].
- **Validation**: `goldens`, `attestation` (`almával` bucket in `NOM`).
- **Confidence**: `normative`.

### PH-LEN-002 Super-analytic (non-lengthening) suffixes
- **Statement**: Final `a`/`e` stays short before a closed set of suffixes that are phonologically attached to the word like compound members: `-ként`, `-képp`/`-képpen`, `-ság`/`-ség`, `-szerű`, `-féle`, `-kor`, `-szor`/`-szer`/`-ször`, adjectival `-i`, `-izmus`, `-ista`, `-beli`, `-fajta`, `-mentes`, `-szerte`, marginally Latinate `-ikus`, `-izál`, `-lógia` (`?archaikus`, `?archaizál`, `?archeológia` in SMNy3 (188)); these suffixes also trigger no other stem alternation.
- **Conditions**: next suffix in the list; all other suffixes lengthen (including `-nyi`: `bögrényi` [ATT 2]).
- **Output**: short vowel retained.
- **Examples**: (SMNy3 (188)) `kefeként`, `kefeképp(en)`, `henyeség`, `kefeszerű`, `kefeféle`, `velencei`, `?goetheizmus`, `?goetheista` (SMNy3 coinages, 1 and 2 hits in WC2); `faként`, `faképp(en)`, `butaság` [ATT 7], `faszerű`, `faféle`, `órakor` [ATT 17], `?alfaszor` (7 hits in WC2; the multiplicative is a numeral suffix, PND-NUMINF-005), `budai` [ATT 15], `dadaizmus`, `dadaista` [ATT 1]; possessive + `-ként`: `tagjaként` [ATT 12], `részeként` [ATT 15] (never `*tagjáként`).
- **Exceptions**: `-i` still triggers v-stem alternants in some lexicalised forms (`tavi`, `havi`, `művi`) and shortening in some (`heti`, but `nyári`) [SMNy3 p. 788]. The four compound-like elements `-beli`, `-fajta`, `-mentes`, `-szerte` were previously listed here as behaving like compound members; they are now in the Statement list, measured non-lengthening, so the separate exception is redundant. Whether they are called suffixes or compound members is a labelling choice with no phonological consequence.
- **Predictability**: `lexical` (closed list of about 16 suffixes; `-ként` is the only inflectional one [SMNy3 p. 788])
- **Lexical input**: suffix property `lengthening=false`.
- **Interactions**: `NOM-LENG-002`, `ORT-SUFX-002`, `PH-HARM-016`, `PH-ZERO-002`.
- **Sources**: [SMNy3 Rebrus (188)-(189) pp. 788-789, (204) p. 796]; [AkH12 §29 (`apai`, `kergeség`), §216 a) (`-i`, `-szerű`)]; [RST12 fn. 10 p. 497]; [WC2]; [ATT evidence/productivity-tagset.md 2026-09-16].
- **Validation**: `goldens`, `attestation`; corpus: no lengthening in 10,302 `a`/`e`-final types before `-beli`/`-fajta`/`-mentes`/`-szerte` (`-beli` 1,957 types, `-fajta` 3,975, `-mentes` 4,216, `-szerte` 154), with 2 typo-level counterexamples (`korabeli` 211,974 : `korábeli` 3; `mesebeli` 34,063 : `mesébeli` 5). Tolerance Principle: `e` = 2 against `theta_10302` = 1,114, so non-lengthening is productive and categorical. Stems in an underlying long vowel (`kávé-`, `tévé-`, `klisé-`, `Bábel-`) are excluded from the candidate set: they were the whole apparent lengthened residue of a naive count.
- **Confidence**: `consensus` (corpus-confirmed for the four compound-like elements).

### PH-LEN-003 Items exempt from final a/e lengthening
- **Statement**: Lengthening does not apply to (a) all-caps letter acronyms and all-caps names of institutions spelled with a final `a`/`e`/`o` (in writing), (b) monosyllabic function words, (c) monosyllabic loans in short `e` taken with function-word prosody.
- **Conditions**: see Statement.
- **Output**: short vowel retained.
- **Examples**: `MTA-nak`, `ELTE-n`, `ELTE-re`, `ETO-hoz`, `FIFA-nak` (word-read, pronounced [fifaːnɒk]); marginal suffixed function words keep the short vowel, as metalinguistic citations rather than inflected forms: `de` -> `?dek`, `?det` (the WC2 counts are homographs of `deka` and `detektív`); `Che`->`Chet` preferred to `?Chét`, but `Ocse`->`Ocsét`; word-like blends lengthen (`stukák`, `Hungexpónál`, `Alitaliánál`).
- **Exceptions**: pronunciation of acronym readings may lengthen (`MTA-nak` read `em-té-á-nak`: the final letter name is already long).
- **Predictability**: `rule`
- **Lexical input**: stem flags `acronym_letterwise`, `function_word`.
- **Interactions**: `NOM-LENG-003`, `ORT-ABBR`, `PH-G2P-011`.
- **Sources**: [AkH12 §29, §288 a)-b)]; [SMNy3 Rebrus pp. 790-791].
- **Validation**: `goldens`.
- **Confidence**: `normative` (acronyms); `consensus` (others).

### PH-LEN-004 Written lengthening in foreign-spelled words
- **Statement**: In words and names written with foreign orthography, a pronounced final `a`, `e`, `o`, `ö` is written long (`á`, `é`, `ó`, `ő`) in suffixed forms and in derivatives, except before `-i` and `-szerű` (and other non-lengthening suffixes); silent final letters give no lengthening and require a hyphen (`ORT-FOR-012`).
- **Conditions**: foreign spelling; the final written vowel is pronounced as that vowel.
- **Output**: long letter in the suffixed form; hyphen if the base keeps a hyphen for other reasons (`Victor Hugó-i`).
- **Examples**: `signorina`->`signorinák`; `Sinaia`->`Sinaiában`, but `sinaiai`; `Coca-Cola`->`Coca-Colát`, but `Coca-Cola-szerű`; `campanile`->`campanilén`, but `campanileszerű`; `Cicero`->`Cicerót`; `Goethe`->`Goethének`, `goethés`, but `goethei`; `allegro`->`allegróban`, `allegrós`; `Oslo`->`Oslóban` [ATT 4], `oslói`; `Malmö`->`Malmőből`, `malmői`.
- **Exceptions**: final `i`, `u`, `ü` never lengthen (`Helsinkiben`, `menüt`); silent final `e` (`Shakespeare-rel`) or vowel spelled by an unusual group (`Rousseau-val`) (`ORT-FOR-012`).
- **Predictability**: `rule`
- **Lexical input**: pronunciation of the final letter (`PH-G2P-008`).
- **Interactions**: `NOM-LENG-005`, `ORT-FOR`, `PH-LEN-005`.
- **Sources**: [AkH12 §216 a), §217 a)-b)].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-LEN-005 Final short o and ö are long in pronunciation
- **Statement**: A content word cannot end in short /o/ or /ø/: loans with final `o`/`ö` are pronounced with /oː/, /øː/ even unsuffixed, and hungarianised spellings write `ó`/`ő`.
- **Conditions**: loan or name ending in a pronounced short `o`/`ö`.
- **Output**: pronunciation /oː/, /øː/; hungarianised spelling `ó`/`ő`.
- **Examples**: `libretto` [librɛtːoː]; `kapucsínó`, `trafó`, `Milánó`, `Oslo` [osloː]; interjections `no`, `nono` keep short /o/.
- **Exceptions**: first members of foreign compounds (`pszichobiológia` [psihobiologiɒ]).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-SYL-013`, `PH-LEN-004`, `PH-HARM-001`.
- **Sources**: [T04 §3.4.1 (57)-(58) pp. 105-106, p. 79]; [SMNy3 Rebrus pp. 789-790]; [AkH12 §203, §210].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-LEN-006 Stem-final high vowels do not alternate
- **Statement**: Stem-final `i`, `u`, `ü` never lengthen before suffixes and `í`, `ú`, `ű` never shorten in regular stems; whether a polysyllabic word ends in a short or long high vowel is lexical spelling, with these regularities: final `i` is short except `sí`, `rí`, `zrí` (and a few others); monosyllabic nouns end in long `ú`/`ű`; adjectives in `-ú`/`-ű` are always long; hypocoristics in `-u` are short.
- **Conditions**: stem-final high vowel.
- **Output**: suffixed form keeps the lexical length.
- **Examples**: `kocsi`->`kocsit`; `kapu`->`kapun`; `eskü`->`eskünek`; `ágyú`->`ágyút`; `betű`->`betűk`; `bú`, `szú`, `fű`, `mű`, `tű`; adjectives `hosszú`, `savanyú`, `szomorú`, `keserű`, `könnyű`, `szörnyű`; short: `adu`, `alku`, `áru`, `daru`, `falu`, `hamu`, `kapu`, `menü`, `revü`, `anyu`, `Icu`; long: `ború`, `borjú`, `fiú`, `gyanú`, `háború`, `tanú` (AkH12 §23 spelling), `varjú`, `derű`, `fésű`, `gyűrű`, `hegedű`, `seprű`.
- **Exceptions**: v-stems and u/ü-to-v stems (`PH-VSTEM`); ú-deleting stems (`borjú`->`borjak`); in speech non-initial long high vowels are often short (`ár[u]` for both `áru` and `árú`) [SMNy3 p. 768]; spelling is fixed [AkH12 §20].
- **Predictability**: `rule` (no alternation); `lexical` (spelling of length in polysyllables).
- **Lexical input**: lexical final vowel length.
- **Interactions**: `PH-SYL-013`, `ORT-LEN`, `PH-VSTEM-004`.
- **Sources**: [AkH12 §20-§24]; [SMNy3 Rebrus (187) p. 787, pp. 768-769]; [T04 (59)-(61) pp. 107-108].
- **Validation**: `goldens`.
- **Confidence**: `normative` (spelling); `consensus` (non-alternation).

### PH-LEN-007 Stem-internal shortening stems
- **Statement**: A closed class of noun stems shorten their last vowel (`á`->`a`, `é`->`e`, `í`->`i`, `ú`->`u`, `ű`->`ü`) before synthetic suffixes that begin with a short low vowel (`a`/`e`, as suffix vowel or lowered linking vowel), before `-unk/-ünk`, `-uk/-ük` and before `-ász/-ész`; the stem has a single final consonant and no other long vowel; `ó` and `ő` never shorten stem-internally.
- **Conditions**:
  - Stem marked `shortening`.
  - Triggering suffixes: plural `-Ak`, accusative `-At`, possessives `-Am`, `-Ad`, `-A`, `-Unk`, `-Atok`, `-Uk`, possessed plural `-Ai-`, `-As`, `-Ász`, `-Anta`; also some derivatives (`kanalanként`, `nyarastul`, `bogaras`, `nyaral`, `madarász`).
  - Non-triggering: superessive `-On` (`nyáron`, `tűzön`, `vízen`), `-ig`, `-ért`, `-é`, `-ul`, consonant-initial suffixes (`nyárban`, `nyárnak`, `nyártalan`, `kanálnyi`), `-ként`, `-ság` (`szamárság`), `-kor`, `-szor`.
- **Output**: short vowel in triggering contexts.
- **Examples**: `nyár`->`nyarak`, `nyarat`, `nyaram`, `nyara`, `nyarunk`; `kéz`->`kezek`, `kezet`, `keze`; `út`->`utak`, `utat`, `utam`; `tűz`->`tüzek`, `tüzet`, `tüzes`; `víz`->`vizek`, `vizet`, `vizes`; `lélek`->`lelket`, `lelkünk` (with vowel-zero); homophone pairs: `dél` (noon) `delet` vs `dél` (south) `délt`; `szél` (wind) `szelek` vs `szél` (edge) `szélek`.
- **Exceptions**: (complete list from SMNy3 (113), about 70 stems)
  - `á`: `nyár`, `sár`, `fonál`, `kanál`, `agár`, `bogár`, `gúnár` (marginal), `kosár`, `madár`, `mocsár`, `mozsár`, `pohár`, `sudár`, `sugár`, `szamár`, `parázs`, `darázs` (17);
  - `é`: `tél`, `ég`, `jég`, `lég`, `légy`, `bél`, `dél`, `fél`, `nyél`, `szél`, `szén`, `dér`, `ér`, `tér`, `ész`, `mész`, `hét`, `hév`, `név`, `kéz`, `réz` and about 20 more (`derék`, `egér`, `fedél`, `levél`, `szekér`, `tehén`, `tenyér`, `kenyér`, `veréb`, `cserép`, `fazék`, `gyökér`, `szemét`);
  - `ú`: `út`, `kút`, `lúd`, `nyúl`, `rúd`, `úr` (6); `ű`: `tűz`, `fűz`, `szűgy`, `szűz` (4); `í`: `víz`, `híd`, `nyíl`, `ín` (4).
  - Unstable in the 3sg possessive and accusative: `hídja`, `kútja`, `lúdja`, `rúdja`, `útja` [AkH12 §27 b)] (but `nyula`, `vize`); accusative `ért` of `ér` 'brook' beside `eret` [SMNy3 p. 743]; `észt`; derived forms split by meaning: `uraság` vs `úrság`, `feleség` vs `félség`; `madárka`~`madaracska`.
  - Stems with both variants must still follow the low-vowel requirement: `sáros`~`saras`, never `*saros`.
- **Predictability**: `lexical` (closed class, about 70 stems)
- **Lexical input**: stem flag `shortening` with bound alternant.
- **Interactions**: `NOM-STEM`, `NOM-LOW` (all shortening stems are lowering), `PH-HARM-009`.
- **Sources**: [SMNy3 Rebrus §13.4.1 (113) pp. 741-744]; [AkH12 §27 b)]; [KVF98 §3.4.1.3.1.3 pp. 447-448]; [T04 §3.4.2 p. 120].
- **Validation**: `goldens`, `oracle:hunmorph-foma`.
- **Confidence**: `consensus`; membership `lexical`.

### PH-LEN-008 Final vowel shortening in v-stems
- **Statement**: In shortening v-stems the stem-final long vowel shortens (and may change quality) together with the insertion of `v` before the triggering suffixes; details and lists in `PH-VSTEM-002`.
- **Conditions**: stem flag `v_stem=shortening`.
- **Output**: `ó`->`o`/`a`, `ő`->`ö`, `é`->`e`, `ű`->`ü`, `ú`->`u` + `v`.
- **Examples**: `ló`->`lovak`; `tó`->`tavak`; `kő`->`kövek`; `lé`->`levek`; `fű`->`füvek`; verbs `lő`->`lövök`, `ró`->`rovok`.
- **Exceptions**: see `PH-VSTEM-002`.
- **Predictability**: `lexical`
- **Lexical input**: `v_stem`.
- **Interactions**: `PH-VSTEM-002`, `NOM-STEM`, `VRB-STEM`.
- **Sources**: [SMNy3 Rebrus §13.5.1.2 (136) pp. 757-759].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-LEN-009 Possessive-only shortening and lowering of final ó/ő
- **Statement**: A small class of nouns ending in `ó`/`ő` replace the final vowel by short `a`/`e` before the 3sg/3pl possessive `-ja/-je`, `-juk/-jük`, with frequent variation with the regular form; possessed plural forms of this type (`disznaim`, `erdeim`) are isolated and obsolete.
- **Conditions**: stem flag `poss_alt`; suffix = 3sg/3pl possessive `-jA`, `-jUk`.
- **Output**: `-ó`->`-a-`, `-ő`->`-e-` + `j`-suffix.
- **Examples**: `ajtó`->`ajtaja` [ATT 9], `ajtajuk`; `idő`->`ideje` [ATT 19]; `erő`->`ereje`; `tető`->`teteje` [ATT 10]; `erdő`->`erdeje` [ATT 3]~`erdője` [ATT -] (normative doublet, AkH12 §30); `tüdő`->`tüdeje` [ATT 5]; `velő`->`veleje`; `esztendő`->`esztendeje` [ATT 9]; variants: `zászló`->`zászlaja` [ATT 7]~`zászlója` [ATT 4], `mező`->`mezeje` [ATT 3]~`mezője` [ATT 5].
- **Exceptions**: the vast majority of `ó`/`ő` nouns are regular (`hajója`, `cipője`, `felhőjük`); meaning splits occur.
- **Predictability**: `lexical` (about 20 nouns; 24 in `NOM-STEM-013`)
- **Lexical input**: `poss_alt`.
- **Interactions**: `NOM-POSS`, `NOM-STEM-013`.
- **Sources**: [KVF98 §3.4.1.3.2 p. 449]; [SMNy3 Rebrus §13.5.3.3 pp. 780-781]; [AkH12 §30].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### PH-LEN-010 Lexical shortening in derivation and numerals
- **Statement**: Some long-vowel verb and numeral stems have short-vowel alternants in specific, lexicalised derivatives; the alternant is stored, not derived.
- **Conditions**: listed derivatives.
- **Output**: stored alternants.
- **Examples**: `ír`->`irat` (noun), `iroda`, `irodalom` (but `írás`, `írat` verb); `bír`->`birtok`; `tűr`->`türelem`, `türtőztet`; `fúr`->`furat`; `óv`->`óvoda` (spelled long, pronounced [ovodɒ]); `vág`->`vagdal`; `úszik`->`uszoda`, `uszály`, `uszony`; `bújik`->`bujkál`, `bujdosó`; `hűl`->`hüledezik`; `zsír`->`zsiradék`; numerals `tíz`->`tized`, `tizedik`, `tizen-`; `húsz`->`huszad`, `huszadik`, `huszon-`; `négy`->`negyed`, `negyedik`, `negyven`; `három`->`harmad`, `harmadik`, `harminc`; `hét`->`heted`, `hetven` (regular shortening stem).
- **Exceptions**: `négy`, `tíz`, `húsz` do not shorten in inflection (`négyet`, `tízet`, `húszat`), unlike `hét` (`hetet`).
- **Predictability**: `lexical`
- **Lexical input**: stored derivatives / numeral stems (`PND`).
- **Interactions**: `DER`, `PND`, `PH-LEN-007`.
- **Sources**: [AkH12 §27 a)]; [SMNy3 Rebrus §13.4.1 pp. 741-742, p. 744].
- **Validation**: `goldens`.
- **Confidence**: `normative` (spelling).

### PH-LEN-011 Shortening before Latinate suffixes
- **Statement**: In common loans a long vowel of the stem (usually the last one) is short before Latinate derivational suffixes such as `-ikus`, `-ista`, `-itás`, `-izál`, `-izmus`, `-isztika`, `-ális`, `-ifikál`; a minority keep the long vowel.
- **Conditions**: loan stem + Latinate suffix (disyllabic, vowel-initial, non-harmonising).
- **Output**: short vowel by default; long vowel for listed exceptions.
- **Examples**: `szláv`->`szlavista`, `szlavisztika`; `szociális`->`szocialista`, `szocializáció`; `hérosz`->`heroikus`, `heroizmus`; `passzív`->`passzivitás` [ATT 3]; `hipnózis`->`hipnotikus`, `hipnotizál`; `szuverén`->`szuverenitás`; `irónia`->`ironikus`; `periódus`->`periodikus`; `reális`->`realista` [ATT 8]; `típus`->`tipikus` [ATT 14]; `kódex`->`kodifikál`; `história`->`historizmus`; `aktív`->`aktivitás`; `kolléga`->`kollegiális`.
- **Exceptions**: long vowel kept: `akadémia`->`akadémikus` [ATT 9], `akadémizmus`, `akadémista`; `esztéta`->`esztétika`, `esztétikus`; `fókusz`->`fókuszál`.
- **Predictability**: `tendency` (no count available; exceptions listed by AkH are a minority)
- **Lexical input**: derivative spelling stored in the lexicon.
- **Interactions**: `DER`, `ORT-LEN`.
- **Sources**: [AkH12 §28, §206]; [SMNy3 Rebrus §13.4.1 p. 742]; [KVF98 §3.4.1.3.1.4 p. 448].
- **Validation**: `oracle:hunmorph-foma`, `attestation`.
- **Confidence**: `normative` (spelling of listed words); `tendency` (generalisation).

### PH-LEN-012 Final ú/ű shortening before the verbaliser -l
- **Statement**: A closed set of `ú`/`ű`-final nominals have short `u`/`ü` before the verb-forming `-l` (`-ul/-ül`, `-ködik`).
- **Conditions**: listed stems.
- **Output**: short vowel.
- **Examples**: (AkH12 §31, complete list of 12) `ború`->`borul`; `bosszú`->`megbosszul`; `csepű`->`csepül`; `derű`->`derül`; `domború`->`domborul`; `fésű`->`fésül`; `hegedű`->`hegedül`; `keserű`->`megkeserül`; `köpű`->`köpül`; `köszörű`->`köszörül`; `lassú`->`lassul`; `szörnyű`->`szörnyülködik`.
- **Exceptions**: `-`
- **Predictability**: `lexical` (12 items)
- **Lexical input**: stored derivatives.
- **Interactions**: `DER`, `PH-LEN-013`.
- **Sources**: [AkH12 §31].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-LEN-013 Fixed vowel length in suffixes
- **Statement**: Suffix vowels have fixed length independent of pronunciation variation: always long in `-ít`, `-dít`, `-sít`, `-ú/-ű`, `-jú/-jű`, `-tyú/-tyű`, `-attyú/-ettyű`, `-ból/-ből`, `-ról/-ről`, `-tól/-től`, postposition-final `-ól/-ől`; always short in `-ul/-ül` and `-dul/-dül` (derivational), `-ul/-ül` (essive), `-stul/-stül`, adverb/postposition-final `-ul/-ül` (except `túl`), `-szor/-szer/-ször`, `-kor`, `-ig`, and word-final `-i` of suffixed forms.
- **Conditions**: suffix identity.
- **Output**: as listed.
- **Examples**: `épít`, `tanító`, `buzdít`, `állandósít`; `négylábú`, `bőkezű`, `lapos tetejű`; `fogantyú`, `billentyű`, `dugattyú`; `szobából`, `tetőről`, `fától`; `alól`, `mellől`; `gyógyul`, `kerül`, `fordul`; `magyarul`, `végül`; `családostul`, `cipőstül`; `alul`, `hátul`, `belül`, `nélkül`; `háromszor`, `hússzor`; `éjfélkor`; `félig`, `hazáig`; `esti`, `házai`, `kérdi`, `írni`, `öklömnyi`.
- **Exceptions**: `túl`; the long `ó/ő` of `-ból`, `-ról`, `-tól` and of `alól`, `felől` is pronounced short in colloquial standard speech [SMNy3 Rebrus p. 780], spelling unaffected (`PH-LEN-014`).
- **Predictability**: `rule`
- **Lexical input**: suffix templates.
- **Interactions**: `ORT-LEN`, `PH-LEN-006`.
- **Sources**: [AkH12 §34-§35]; [SMNy3 Rebrus p. 780].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-LEN-014 Length variation in speech not reflected in spelling
- **Statement**: Several quantity processes of casual speech never change spelling or suffix choice: compensatory lengthening after dropping `l`, `r`, `j`, `n` in the coda (`tolta` [toːtɒ], `kulcs` [kuːt͡ʃ], `arra` [ɒːrɒ]); shortening of `á`/`é` before a cluster whose second consonant is a non-continuant (`Márta` [mɒrtɒ]~[maːrtɒ], `férges`); shortening of non-initial long high vowels (`áru`/`árú` merged); short `ó`/`ő` in `-ból`, `-ról`, `-tól`, `alól`, `felől` [SMNy3 p. 780]; `egy` long [ɟː] between vowels in some forms; `kisebb`, `erősek`, `magasan` often with long [ʃː].
- **Conditions**: pronunciation only.
- **Output**: spelling and allomorphy from the lexical form.
- **Examples**: see Statement; spelled `egyet`, `egyért`, `kisebb`, `magasat`, `új` (often [ujː]), `lesz` with one `sz` in all forms.
- **Exceptions**: `-`
- **Predictability**: `rule` (non-effect)
- **Lexical input**: `none`
- **Interactions**: `PH-G2P`, `ORT-LEN`.
- **Sources**: [KVF98 §3.4.1.3.1.1 pp. 446-447]; [T04 fn. 126 p. 229]; [AkH12 §20, §68-§73]; [SMNy3 Rebrus p. 768].
- **Validation**: `goldens` (pronunciation only).
- **Confidence**: `consensus`.

### PH-LEN-015 Consonant length alternation in suffixes after vowels and consonants
- **Statement**: Several suffixes have a long consonant after a vowel and a short one after a consonant: past `-tt`/`-t`, past participle `-tt`/`-t`, locative `-tt`/`-t`, postposition-final `-tt`/`-t`, comparative `-bb`/`-b`, 2sg definite short imperative `-dd`/`-d`, `-ll`/`-l` 'seem, consider', `-ttyú/-ttyű` vs `-tyú/-tyű`; after a consonant the short variant follows the linking-vowel logic of the host rules.
- **Conditions**: suffix in the list; preceding segment vowel (including linking vowel) vs consonant.
- **Output**: `V_` -> long; `C_` -> short.
- **Examples**: `folyt`, `varrtuk` vs `csengetett`, `dobott`, `hallottuk`, `megrótták`; `felforrt` vs `kitűzött`; `Vásárhelyt` vs `Győrött`, `Pécsett`, `Kolozsvárt`~`Kolozsvárott`; `iránt`, `szerint` vs `alatt`, `között` (but `át`, `hosszat`); `idősb` (obsolete) vs `idősebb`, `jobb`, `szebb`; `írd`, `nézd` vs `edd`, `idd`, `sződd`; `fehérlik` vs `zöldellő`, `kékellett`~`kéklett`; `pattantyú` vs `dugattyú`.
- **Exceptions**: `d`-final verbs write `dd` after a consonant too (`mondd`, `küldd`, `áldd`) because stem `d` + suffix `d` [AkH12 §62]; long-vowel verbs `rótt`, `nőtt` [AkH12 §66].
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `VRB-PAST`, `VRB-IMP`, `NOM-ADJ`, `NOM-CASE`.
- **Sources**: [AkH12 §38, §62, §66].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-LEN-016 Lowered ó/ő stems before adjectival -i
- **Statement**: Three `ő`-final nouns of the possessive-lowering class (`PH-LEN-009`) have a short lowered `e` before the adjectival `-i`; for other nouns `-i` attaches to the unchanged stem.
- **Conditions**: stems `erdő`, `mező`, `esztendő` + adjectival `-i`.
- **Output**: `erdei`, `mezei`, `esztendei`; regular `-ői` variants are marginal.
- **Examples**: `erdei` [ATT 15] vs `erdői` [ATT 0]; `mezei` [ATT 12] vs `mezői` [ATT 2]; `esztendei` [ATT 3]; lexicalised `idei` [ATT 18] ('this year's') beside transparent `idői` [ATT 1] ('temporal').
- **Exceptions**: `-`
- **Predictability**: `lexical` (3 stems plus lexicalised `idei`)
- **Lexical input**: stored derivative.
- **Interactions**: `PH-LEN-009`, `PH-LEN-002` (`-i` otherwise triggers no stem alternation), `NOM-STEM-013`, `DER`.
- **Sources**: [SMNy3 Rebrus §13.5.3.3 p. 781].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

---

## ASSIM: assimilations

### PH-ASSIM-001 Written and unwritten assimilations
- **Statement**: Hungarian spelling is morphemic: assimilations are not written except in a closed set of morphologically restricted alternations, which the generator must produce in writing; all other assimilations affect only pronunciation.
- **Conditions**: suffixation, compounding, word sequences.
- **Output**:

  | Process | Written | Rule |
  |---|---|---|
  | `v` of `-val/-vel`, `-vá/-vé` to stem-final consonant | yes | `PH-ASSIM-002` |
  | `z` of `az`/`ez` to consonant-initial suffix | yes | `PH-ASSIM-003` |
  | `j` of imperative and definite suffixes to `s sz z dz` | yes | `PH-ASSIM-004` |
  | `t`-final verb stems in the imperative | yes | `PH-ASSIM-005`, `PH-ASSIM-006` |
  | `-gy` imperative of `eszik iszik lesz tesz vesz visz hisz` | yes (morphological) | `PH-ASSIM-008` |
  | `j` after `gy ny ty l ll`; `t d n` + `j` coalescence | no | `PH-ASSIM-007` |
  | voicing | no | `PH-ASSIM-009` |
  | nasal place | no | `PH-ASSIM-010` |
  | sibilant place, `t d gy` + `s sz` affrication | no | `PH-ASSIM-011` |
  | degemination next to a consonant | no | `PH-ASSIM-012` |
  | three identical consonant letters | simplified in writing (`ORT-ASSIM-017`, `ORT-HYPH-002`) | `PH-ASSIM-012` |
  | `l` to `r`, `j` | no | `PH-ASSIM-013` |
  | hiatus `j` | no | `PH-ASSIM-014` |
  | middle consonant loss | no | `PH-SYL-016` |
- **Examples**: `házzal` (written); `adhat` [ɒthɒt] (unwritten); `község` [køʃːeːɡ] (unwritten); `látja` [laːcːɒ] (unwritten); `lássa` (imperative, written).
- **Exceptions**: traditional spellings that hide an assimilation (`játszik`, `metsz`, `utca`) `PH-G2P-003`.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: listed rules.
- **Sources**: [AkH12 §48-§60, §75-§84, §91-§93]; [KVF98 §3.4.1.1 p. 436].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-ASSIM-002 v-assimilation of -val/-vel and -vá/-vé
- **Statement**: The instrumental `-vAl` and translative `-vÁ` begin with `v` after a vowel-final stem; after a consonant-final stem the `v` becomes identical to the last pronounced consonant, written by doubling that consonant's letter (first letter of a digraph), with simplification of three identical letters and hyphenation where the norm requires it.
- **Conditions**:
  - Vowel-final (including silent-`h` stems of `cseh` type): `-val/-vel`, `-vá/-vé`.
  - Single final consonant: doubled.
  - Final long consonant: result written with two letters only (`tollal`, `meggyel`) in common nouns; with hyphen and three letters for personal names and foreign proper names ending in a doubled consonant letter (`Kiss-sel`, `Papp-pal`, `Széll-lel`, `Mann-nal`, `Scott-tal`) (`ORT-ASSIM-002`, `ORT-NAME-012`, `ORT-FOR-013`).
  - Final cluster: last consonant doubled; the geminate is pronounced short (`hanggal`, `kulccsal`, `szárnnyal`, `bölccsé`, `füstté`).
  - Final `v`: `vv` written (`szívvel`, `kedvvel`, `könyvvé`, `névvé`).
  - Variable `h` (`düh`, `méh`, `céh`, `juh`, `rüh`): both `dühvel`~`dühhel`.
  - Digits, symbols, abbreviations, acronyms: hyphen + assimilated form by pronunciation (`4-gyel`, `15%-kal`, `Bp.-tel`, `DNS-sel`).
  - Archaic and foreign final letters: the letter group is kept and the pronounced consonant is written (`Kossuthtal`, `Véghgel`, `Móriczcal`, `Babitscsal`, `Madáchcsá`, `Marxszal`, `Félixszel`, `fixszel`, `Balzackal`, `pechhel`, `Bachhal`, `Greenwichcsel`, `Wellsszel`, `hertzcel`); silent final letters: hyphen and assimilation to the pronounced consonant (`guillotine-nal`, `Molière-rel`, `Iaşi-sal`) or vowel form (`Rousseau-val`).
- **Output**: `-vAl`/`-vÁ` with initial consonant = `v` | copy of last consonant.
- **Examples**: `babával`, `kővé`, `csehvel`, `pléhvé`; `bottal`, `cukorral`, `kékkel`, `arannyal`, `rozzsal`, `néggyel`, `tudóssá`, `szénné`, `vízzé`, `királlyá`, `özveggyé`; `jobbal`, `tollal`, `tettel`, `gallyal`, `könnyel`, `rosszá`; `komppal`, `minddel`, `gyönggyel`, `ronggyá`; `dohhal`, `Allahhal`; `eggyel` [ATT 10]; `briddzsel`.
- **Exceptions**: non-alternating `v`-initial suffixes never assimilate: `-vA/-vE` (adverbial participle `lopva`, `nézve`), `-ván/-vén`, `-vAn/-vEn` (`hatvan`, `kilencven`), `-vÁny/-vÉny` (`látvány`); spelling contrasts: `szívvel` vs `szível`, `tervvel` vs `tervel`, `bölccsé` vs `bölcsé`, `briddzsel` vs `bridzsel`.
- **Predictability**: `rule`
- **Lexical input**: pronounced final consonant (`PH-G2P`), `h_silent`.
- **Interactions**: `NOM-CASE`, `ORT-ASSIM-001`..`ORT-ASSIM-007`, `ORT-NAME-011`, `ORT-FOR-011`, `PH-CONS-005`, `PH-ASSIM-003`.
- **Sources**: [AkH12 §41, §82 a)-f), §92, §93, §160, §216 b), §217 a), c)]; [T04 §4.2.1 (66) pp. 212-218]; [KVF98 §3.4.1.1.1 p. 437].
- **Validation**: `goldens`, `oracle:magyarispell`.
- **Confidence**: `normative`.

### PH-ASSIM-003 z-assimilation of az and ez
- **Statement**: The final `z` of the demonstratives `az`, `ez` (and compounds `mindaz`, `mindez`, `ugyanaz`, `amaz`) assimilates fully to the initial consonant of a consonant-initial case suffix and is written doubled; before vowel-initial suffixes and the accusative `-t` it stays; with the instrumental both assimilation directions are standard.
- **Conditions**: stem `az`/`ez` (demonstrative) + case suffix; also `-fajta`, `-féle`, `-kora` compounds.
- **Output**: `ab-ban`, `eb-be`, `ah-hoz`, `ek-kor`; instrumental `azzal`~`avval`, `ezzel`~`evvel` [AkH12 §44, §83]; translative `azzá`, `ezzé` (no `v`-variant given by the norm); terminative `addig`, `eddig`.
- **Examples**: `abba`, `ebbe`, `ahhoz`, `ehhez`, `akképp`, `ekképp`, `arra`, `erre`, `akkor`, `ekkor`, `abbeli`, `ebbeli`, `amannak`, `mindettől`, `ugyanahhoz`, `affajta`, `afféle`, `akkora`, `ekkora`; `azt`, `ezen`, `azok`; `ezzel` [ATT 22], `evvel` [ATT 8].
- **Exceptions**: postposition compounds use `a`/`e` (`afelé`, `amiatt`, `emellett`) [AkH12 §84]; `ahhoz` may be pronounced with short `h`.
- **Predictability**: `lexical` (2 stems and compounds)
- **Lexical input**: stem identity.
- **Interactions**: `PND`, `PH-ASSIM-002`.
- **Sources**: [AkH12 §44, §83, §84]; [KVF98 §3.4.1.1.2 p. 438].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `normative`.

### PH-ASSIM-004 j-assimilation to sibilant-final verb stems
- **Statement**: The `j` of the imperative `-j` and of the definite present endings `-ja`, `-juk/-jük`, `-játok/-jétek`, `-ják/-jék` becomes identical to a stem-final `s`, `sz`, `z` or `dz` and is written as a doubled consonant (digraph with first letter doubled).
- **Conditions**: verb stem ending in `s`, `sz`, `z`, `dz` (including `-tsz` in `játszik`, `metsz`, `látszik`, `tetszik`, `hallatszik`); suffix begins with that `j`.
- **Output**: `s`+`j` -> `ss`; `sz`+`j` -> `ssz`; `z`+`j` -> `zz`; `dz`+`j` -> `ddz`.
- **Examples**: definite: `keressük`, `mossa` [ATT 10], `olvassák`, `olvassa` [ATT 16], `vadásszátok`, `hozzák`, `hozza` [ATT 16], `végezzük`, `eddzük`, `peddzük`, `játssza` [ATT 11], `játsszuk`, `metsszük`, `nézzük` [ATT 15], `visszük`; imperative: `késsen`, `mossam`, `vesszenek`, `vadásszatok`, `hozzad`, `végezzünk`, `eddzétek`, `lopóddzál`, `hallatsszon`, `játsszál`, `látsszam`, `tetsszél`, `metssze`.
- **Exceptions**: short 2sg imperative `-d` attaches to the bare stem (`nyesd`, `olvasd`, `mászd`, `úszd`, `húzd`, `végezd`, `edzd`, `játszd`, `metszd`~`mesd`); `metsz`, `látszik`, `tetszik` imperatives also `messe`, `lássék`, `tessem` [AkH12 §32]; the contrast `játszunk` (indicative) vs `játsszunk` (subjunctive) is only in writing; non-sibilant stems keep `j` (`látja`, `kapja`); `zs`-final and `cs`-final verbs: no such monomorphemic verbs.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `VRB-PRES`, `VRB-IMP`, `PH-ASSIM-005`.
- **Sources**: [AkH12 §32, §43, §79, §80]; [KVF98 §3.4.1.1.6.1 p. 440].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `normative`.

### PH-ASSIM-005 t-final verb stems in the imperative
- **Statement**: Before the imperative `-j` a stem-final `t` changes, and the change is written: (a) short vowel + `t` -> `ss` (`t`->`s`, `j`->`s`); (b) the long-vowel stems `lát`, `bocsát` and `lót` (in `lót-fut`) -> `ss`; (c) other long vowel + `t` and consonant (not `s`, `sz`) + `t` -> `ts` (pronounced [t͡ʃː]); (d) `st`, `szt` -> `t` is dropped and `j` assimilates: `ss`, `ssz`.
- **Conditions**: verb stem ending in `t`; suffix = imperative `-j` (all persons, both conjugations). The definite present `-ja` etc. does not trigger it (`látja`, `festjük`, `tartja`).
- **Output**:
  - (a) `köt`->`köss`, `kössétek`; `üt`->`üss`; `nyit`->`nyissátok`; `nevet`->`nevessen`; `alkot`->`alkoss`; `hallgat`->`hallgassák`.
  - (b) `lát`->`lássuk` [ATT 13], `lássa`; `bocsát`->`bocsássa`; `lót-fut`->`lósson-fusson`.
  - (c) `tanít`->`taníts` [ATT 5], `tanítson`; `fűt`->`fűts`, `fűtse`; `hűt`, `műt`, `szít`, `tát`, `vét` -> `hűts`, `műtsd`, `szítsa`, `tátsd`, `véts`; `ejt`->`ejtse`; `félt`->`féltsétek`; `márt`->`márts`; `tart`->`tartsa` [ATT 14]; `önt`->`öntsön`; `tölt`->`töltsd`; `hajt`->`hajtsd`; `bánt`->`bántsa` [ATT 2].
  - (d) `fest`->`fess`, `fessük` [ATT 2] (vs indicative `festjük` [ATT 1]); `oszt`->`ossz`, `ossza` [ATT 10], `osszuk` (vs `osztjuk` [ATT 9]); `fürdik`/`füröszt`->`fürösszétek`; `illeszt`->`illesszük`; `rekeszt`->`rekessze`; `választ`->`válassza`.
- **Examples**: see Output; indicative contrast `látja` [ATT 17] vs imperative `lássa` [ATT 11]; `tartja` [ATT 18] vs `tartsa` [ATT 14].
- **Exceptions**: the list in (b) is closed (3 items); every other long vowel + `t` verb (all `-ít` verbs and `fűt`, `hűt`, `műt`, `szít`, `tát`, `vét`) takes `ts`; non-standard speech extends (a)/(d) to the definite present (`lássa` for `látja`), not generated.
- **Predictability**: `rule` with a `lexical` 3-item list for (b)
- **Lexical input**: vowel length of the stem (from spelling); list (b).
- **Interactions**: `VRB-IMP`, `PH-ASSIM-006`, `PH-ASSIM-004`.
- **Sources**: [AkH12 §76-§78]; [KVF98 §3.4.1.1.5 pp. 439-440, §3.4.4.1.1 p. 450]; [SMNy3 Kiefer pp. 522-523].
- **Validation**: `goldens`, `attestation`, `oracle:hunmorph-foma`.
- **Confidence**: `normative`.

### PH-ASSIM-006 Short 2sg definite imperative of t-final verbs
- **Statement**: The short 2sg definite imperative `-d` attaches to the imperative stem without `j`: type (a) and (b) stems replace `t` by `s` + `d`; type (c) keeps `ts` + `d`; type (d) drops `t` + `d`.
- **Conditions**: `t`-final verb; suffix `-d` (short form).
- **Output**: (a)/(b) `-sd`; (c) `-tsd`; (d) `-sd`/`-szd`.
- **Examples**: `hallgasd`, `kösd`, `nyisd`, `bocsásd`, `lásd`; `ejtsd`, `mártsd`, `tanítsd`, `bántsd` [ATT 3]; `fesd` [ATT 4], `füröszd`, `illeszd`, `rekeszd`, `válaszd`.
- **Exceptions**: long form uses the full imperative stem: `hallgassad`, `tanítsad`, `fessed`.
- **Predictability**: `rule`
- **Lexical input**: type from `PH-ASSIM-005`.
- **Interactions**: `VRB-IMP`, `PH-LEN-015`.
- **Sources**: [AkH12 §76-§78, §44].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-ASSIM-007 Unwritten j-assimilation and palatalisation
- **Statement**: A suffix-initial `j` fully assimilates in pronunciation to a preceding `gy`, `ny`, `ty`, `l`, `ll`, and coalesces with a preceding `t`, `d`, `n` into a long palatal (`tj` [cː], `dj` [ɟː], `nj` [ɲː]); `t`, `d`, `gy` + `sz`/`s` coalesce into affricates; none of these is written.
- **Conditions**: stem-final consonant in {`gy ny ty l ll t d n`} + `j`-initial suffix (definite `-ja` series, imperative `-j` for `d n gy ny l`, possessive `-ja` series, possessed plural `-jai`); `anya`, `atya` bound stems `any-`, `aty-` and `nagy` + `-ja`.
- **Output**: spelling unchanged; pronunciation as stated.
- **Examples**: `hagyjátok` [hɒɟːaːtok], `hagyjuk` [ATT 15], `fogyjon`, `hányja`, `hunyják`; `ítéljük` [iːteːjːyk], `kapálják`, `tanulja`, `állj`, `beszéljetek`; `kertje` [kɛrcɛ], `kürtjei`, `partja`, `botjai`, `kardjuk`, `hídjai`, `vadjai`, `kínjuk`, `turbánja`; `féltjük`, `mártjátok`, `látja` [laːcːɒ] [ATT 17], `tanítják`, `szeretjük`, `hordja`, `adja` [ATT 18], `maradjanak`, `fonjátok`, `kenjük`; `anyja` [ɒɲːɒ], `atyja`, `nagyjai`; `metsz` [mɛt͡sː], `adsz` [ɒt͡sː], `egyszer` [ɛcsɛr]~[ɛt͡sːɛr], `költség`, `barátság` [bɒraːt͡ʃːaːɡ], `bolondság`, `nagyság`, `vadság`.
- **Exceptions**: slow or spelling pronunciation may suppress the assimilation.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-ASSIM-004` (written counterpart for sibilants), `PH-G2P-001`.
- **Sources**: [AkH12 §53-§58]; [KVF98 §3.4.1.1.6.2-§3.4.1.1.7 pp. 440-441].
- **Validation**: `goldens` (pronunciation only).
- **Confidence**: `normative` (spelling), `consensus` (pronunciation).

### PH-ASSIM-008 gy-imperatives and jön
- **Statement**: The imperative of `eszik`, `iszik`, `lesz`, `tesz`, `vesz`, `visz` is formed on the vowel stem with `-gy-`, of `hisz` with `-ggy-`; `jön` has `jöjj-`; the short 2sg definite imperative uses `-dd` on the vowel stem.
- **Conditions**: listed verbs (closed).
- **Output**: `egyem`, `igyál`, `legyen`, `tegyek` [ATT `tegyen` 14], `vegye`, `vigyed`; `higgy` [ATT 7], `higgye`, `higgyük`; `jöjj`, `jöjjön`; `edd`, `idd`, `tedd`, `vedd`, `vidd`, `hidd`.
- **Examples**: see Output.
- **Exceptions**: `-`
- **Predictability**: `lexical` (8 verbs)
- **Lexical input**: verb entry.
- **Interactions**: `VRB-IRR`, `VRB-IMP`, `PH-VSTEM-006`.
- **Sources**: [AkH12 §42, §67, §81].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-ASSIM-009 Voicing assimilation
- **Statement**: In a sequence of obstruents the earlier ones take the voicing of the last one (regressive, iterative), inside words, across compound and word boundaries; spelling is not changed; `v` undergoes but does not trigger it, `h` triggers devoicing but is unaffected; sonorants neither trigger nor undergo.
- **Conditions**: adjacent obstruents (`PH-CONS-003`).
- **Output**: pronunciation only; accusative `-t` after `z`, `zs` is [st], [ʃt].
- **Examples**: `ablakban` [ɒblɒɡbɒn], `tűzhöz` [tyːshøz], `adhat` [ɒthɒt], `lökdös` [løɡdøʃ], `szívtelen` [siːftɛlɛn], `népdal` [neːbdɒl], `vízszint` [viːsːint], `hideg tél` [hidɛkteːl]; `darabtól` [pt], `ágytól` [ct], `rizstől` [ʃt], `bridzstől` [t͡ʃt]; `testben` [tɛʒdbɛn]; `rabszolga` [ps]; `könyvtár` [ft]; `nyolc gyerek` [d͡zɟ]; sonorants: `szemtől`, `ülsz`, `bajtól`, `kislány`, `napra`.
- **Exceptions**: monomorphemic `v` after voiceless obstruents (`tviszt`, `lekvár`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-CONS-006`, `PH-ASSIM-011`, `PH-SYL-005`.
- **Sources**: [AkH12 §50]; [KVF98 §3.4.1.1.8 pp. 441-442]; [T04 fn. 55 p. 73].
- **Validation**: `goldens` (pronunciation only).
- **Confidence**: `consensus`.

### PH-ASSIM-010 Nasal place assimilation
- **Statement**: `n` takes the place of a following consonant: [m] before `p b m`, [ɱ] before `f v`, [ɲ] before `gy ty ny`, [ŋ] before `k g`, postalveolar before `cs dzs`; `m` is [ɱ] before `f v`; not written; in slow speech it may be suppressed across word boundaries.
- **Conditions**: `n`/`m` + consonant, inside or across words.
- **Output**: pronunciation only.
- **Examples**: `szénpor` [seːmpor], `azonban` [ɒzombɒn], `tanmenet` [tɒmmɛnɛt], `van még`, `öngyújtó` [øɲɟuːjtoː], `negyven tyúk`, `színnyomat` [siːɲɲomɒt], `szenved` [sɛɱvɛd], `ing` [iŋɡ], `szánkó` [saːŋkoː], `nincs`, `hamvas`, `baromfi`.
- **Exceptions**: [ŋ] arising by fast cluster simplification does not re-assimilate (`feszengtem` [fɛsɛŋtɛm]) [T04 fn. 121].
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-SYL-016`, `PH-G2P-001`.
- **Sources**: [AkH12 §51]; [KVF98 §3.1.2.1.3 pp. 383-384, §3.4.1.1.4 p. 438].
- **Validation**: `goldens` (pronunciation only).
- **Confidence**: `consensus`.

### PH-ASSIM-011 Sibilant assimilation and affrication
- **Statement**: Two adjacent sibilants assimilate in place (and voicing) to the second; a dental stop + sibilant fricative becomes a long affricate; a dental stop before an affricate becomes that affricate; full assimilations `t d gy` + `cs` -> [t͡ʃː], `t d` + `gy` -> [ɟː], `sz z` + `s` -> [ʃː], `s zs` + `sz` -> [sː]; none is written.
- **Conditions**: see `PH-SYL-015`; obligatory inside words (careful speech may keep sibilant place across boundaries), optional between words.
- **Output**: pronunciation only.
- **Examples**: `község` [køʃːeːɡ], `egészség` [ɛɡeːʃːeːɡ], `igazság`, `más-szor` [maːsːor], `száz-szor` [saːsːor], `átcsap` [aːt͡ʃːɒp], `jegycsalás`, `átcipel` [aːt͡sːipɛl], `nádcukor`, `száraz zsömle`; `látsz` [laːt͡sː], `ad-sz`, `süketség` [ʃykɛt͡ʃːeːɡ], `rövidség`; `malacság` [mɒlɒt͡ʃʃaːɡ]; `nagyság` [nɒt͡ʃːaːɡ]~[nɒcʃaːɡ].
- **Exceptions**: spelling `ts`, `tsz`, `ds`, `dsz`, `gys`, `gysz` always kept [AkH12 §58]; traditional `játszik`, `metsz`, `tetszik` (`PH-G2P-003`).
- **Predictability**: `rule` (style-dependent optionality)
- **Lexical input**: `none`
- **Interactions**: `PH-SYL-015`, `PH-ASSIM-009`.
- **Sources**: [AkH12 §52, §58]; [KVF98 §3.4.1.1.9-§3.4.1.1.10 pp. 442-446].
- **Validation**: `goldens` (pronunciation only).
- **Confidence**: `consensus`.

### PH-ASSIM-012 Degemination and three identical letters
- **Statement**: A long consonant adjacent to another consonant is pronounced short (obligatorily for underlying and rule-derived geminates next to obstruents, optionally next to nasals, not next to `l r j` for concatenated geminates); spelling keeps the long letter; independently, three identical consonant letters arising from suffixation are written as two, except in proper names and compounds, which take a hyphen (`ORT-ASSIM-017`, `ORT-HYPH-001`, `ORT-HYPH-002`).
- **Conditions**: geminate + C or C + geminate.
- **Output**: pronunciation short; spelling unchanged except the three-letter rule.
- **Examples**: `jobbra` [jobrɒ], `varrt` [vɒrt], `vidd be`, `otthon` [othon], `visszhang`, `küldd` [kyld], `arccal` [ɒrt͡sɒl], `parttalan` [pɒrtɒlɒn], `inggomb`, `hozzánk képest`, `meggynek` [mɛɟnɛk], `csonttal` [t͡ʃontɒl], `hordta` [hortɒ]; writing: `orra` (`orr`+`ra`), `tollal`, `vádlottól`, `füttyel`, `könnyé`, `nagyobban`, `hallak`, `fedd meg`; hyphen: `Hermann-nál`, `Széll-lel`, `Bonn-nal`, `sakk-kör`, `balett-táncos`.
- **Exceptions**: a geminate created by plain concatenation (no rule involved) is not shortened next to `l`, `r`, `j` (`sztrájkként` keeps [kː]) and only optionally next to a nasal (`combban`) [KVF98 p. 448].
- **Predictability**: `rule`
- **Lexical input**: proper-name flag (for writing).
- **Interactions**: `ORT-ASSIM-017`, `ORT-HYPH-001`, `ORT-HYPH-002`, `PH-ASSIM-002`, `PH-SYL-009`.
- **Sources**: [AkH12 §59, §61, §93]; [KVF98 §3.4.1.3.1.5 p. 448].
- **Validation**: `goldens`.
- **Confidence**: `normative` (spelling), `consensus` (pronunciation).

### PH-ASSIM-013 l-assimilation
- **Statement**: `l` assimilates to a following `r` or `j` in pronunciation (`balra` [bɒrːɒ], `ülj` [yjː], `toljuk` [tojːuk], `gólja` [ɡoːjːɒ]), optionally across words in fast speech; not written.
- **Conditions**: `l`/`ll` + `r`/`j`.
- **Output**: pronunciation only.
- **Examples**: `balról` [bɒrːoːl], `télre` [teːrːɛ], `tanulja`, `állj` [aːjː]; `fél rágni`, `reggel jön`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-ASSIM-007`.
- **Sources**: [KVF98 §3.4.1.1.3 p. 438]; [AkH12 §55].
- **Validation**: `goldens` (pronunciation only).
- **Confidence**: `consensus`.

### PH-ASSIM-014 Hiatus glide
- **Statement**: Between a vowel and an adjacent `i`, `í` (and `é`) a [j] glide is pronounced, obligatorily inside words; it is not written, and `j` letters in such words are lexical spelling (`kávéja` is suffix `-ja`, not a glide).
- **Conditions**: VV with one member `i`/`í`/`é`, including across suffix boundaries (`fáig`).
- **Output**: pronunciation [j].
- **Examples**: `fiú` [fijuː], `dió`, `kies`, `diéta`, `mozaik` [mozɒjik], `kiabál`, `fáig` [faːjig], `Mária`.
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `PH-SYL-014`, `PH-G2P-001`.
- **Sources**: [T04 §3.3.1 p. 76, p. 81]; [Wikt `Module:hu-pron` hiatus-`j` step (study only, CC BY-SA)].
- **Validation**: `goldens` (pronunciation only).
- **Confidence**: `consensus`.

---

## ZERO: vowel-zero alternations

### PH-ZERO-001 Epenthetic (unstable-vowel) noun stems: shape
- **Statement**: A closed class of stems has an unstable last vowel that is absent before triggering suffixes (`bokor`->`bokrok`); membership is lexical, but every member satisfies necessary shape conditions and the unstable vowel is predictable in quality.
- **Conditions**: (necessary, not sufficient)
  - At least two syllables; shape `...V(V)C Vu C#`: the unstable vowel `Vu` is in the last syllable, preceded and followed by exactly one consonant (never a cluster on either side), the stem ends in a single consonant.
  - `Vu` is short mid `o`/`e`/`ö` chosen by harmony (`PH-HARM-021`, three-way `O`); irregular vowel in 7 nouns: `bajusz` (u), `vacak`, `kazal`, `ajak` (a), `telek`, `lepel`, `kebel` (open `e` where dialects distinguish `ë`), and in verbs `fogaz` (a), `őriz` (i), `becsül` (ü).
  - If the stable vowel before is long, it is `á` or `é` (exceptions `ólom`, `sólyom`).
  - The flanking consonants are never identical, and flanking obstruents agree in voicing, so the resulting CC never needs a voicing assimilation; homorganic flanks are allowed (`bátor`, `vászon`, `ismétel`, `közöl`, `vödör`) [T04 fn. 7].
- **Output**: bound alternant = stem without `Vu`; stored in the lexicon.
- **Examples**: `bokor`->`bokrok` [ATT 9]; `retek`->`retkek`; `kölyök`->`kölykök` [ATT 7]; `torony`->`tornyok`; `majom`->`majmok`; `bagoly`->`baglyok`; `telek`->`telket` [ATT 8]; `lélek`->`lelkek` [ATT 10] (with shortening); `szirom`->`szirmok` [ATT 4] (antiharmonic back); near-identical non-alternating stems: `perem`->`peremek` (vs `terem`->`termek`), `csődör`->`csődörök` (vs `vödör`->`vödrök`), `tábor`->`táborok` (vs `szobor`->`szobrok`), `szurony`->`szuronyok` (vs `torony`->`tornyok`), `motor`->`motorok` (vs `sátor`->`sátrak`).
- **Exceptions**: counts (SMNy3 (33), monomorphemic nouns): `o` 51, `e` 31, `ö` 22 (104), plus `-alom/-elem` derivatives; T04 counts about 250 nouns including complex ones and about 150 verbs. Measured 2026-09-16 over 8,768 shape candidates: **1,106 members = 230 monomorphemic roots + 146 `-alom/-elem` derivatives + 729 compounds whose head is one of the roots** (11 in review). T04's "about 250 including complex ones" matches the root plus `-alom/-elem` figure (376); SMNy3's 104 monomorphemic count is lower than our 230 because we include lower-frequency roots (`bugyor cseber töbör lucsok pocok vöcsök rücsök`) that the printed list omits.
- **Predictability**: `lexical` (closed class; shape is a necessary filter only)
- **Lexical input**: stem flag `epenthetic` with bound alternant; the list is `epenthetic-nouns.tsv`, 1,106 members with a `kind` column (root / `-alom`-`-elem` / compound), in `local/lists/epenthetic-nouns.tsv` - working data, not yet read by the build.
- **Interactions**: `PH-ZERO-002`, `NOM-STEM`, `VRB-STEM`, `PH-SYL-007`.
- **Sources**: [SMNy3 Rebrus §13.3.1 (33)-(42) pp. 693-698]; [T04 §4.1.1 (1)-(4) pp. 139-144]; [ST97]; [WC2]; [morphdb]; [hunmorph-foma]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `goldens`, `oracle:hunmorph-foma`; oracle agreement on corpus-decided rows: morphdb 4918/4953, our `.ron` 4918/4953, foma 4224/4230, hunspell 4241/4256, UniMorph 1423/1423, kaikki 1851/1852.
- **Confidence**: `consensus`; class size `measured`.

### PH-ZERO-002 Triggering and non-triggering suffixes for nominal vowel-zero
- **Statement**: The unstable vowel is absent before vowel-initial synthetic suffixes (plural, possessives, accusative, superessive, most vowel-initial derivational suffixes) and present before consonant-initial suffixes and before the analytic vowel-initial suffixes `-ig`, `-ért`, `-é`, `-ul/-ül` (essive) and derivations like `-ít`, `-odik` on some roots.
- **Conditions**: epenthetic stem + suffix; suffix property `triggers_zero`.
- **Output**:
  - Absent (`bokr-`): `bokrok`, `bokrom`, `bokrod`, `bokrunk`, `bokrotok`, `bokra`, `bokruk`, `bokrot`, `bokron`, `bokronként`, `bokrostul`, `bokrocska`, `bokros`; `fodroz`, `titkol`, `sarkall`, `titkár`, `fodrász`, `sarki`, `sarkú`, `sarkatlan`, `bátran`, `bátrabb`, `harmadik`.
  - Present (`bokor`): `bokorig`, `bokorért`, `bokoré`, `bokorban`, `bokornak`, `bokorra`, `bokorba`, `bokorral`, `bokorrá`, `bokornál`, `bokorhoz`, `bokorból`, `bokortól`, `bokorként`, `bátorság`, `saroknyi`, `háromkor`, `háromszor`, `bátorít`, `bátorodik`.
  - Variable before the three unstable classes of synthetic suffixes (3sg possessive, accusative, superessive): `sarokja`~`sarka`, `bátort`~`bátrat`, `retekje`~`retke`, `?háromon`, `fodoron`~`fodron`, `?ezeren`, `bajuszt` [ATT 3]~`bajszot`, `leplet` [ATT 7]~`lepelt`.
- **Examples**: see Output.
- **Exceptions**: `-é` is inflectional but does not trigger (`bokoré`); among case suffixes only accusative and superessive trigger.
- **Predictability**: `rule` (suffix class) with `lexical` per-stem variants for the unstable classes.
- **Lexical input**: suffix property `triggers_zero`; stem variant flags for 3sg possessive, accusative, superessive.
- **Interactions**: `NOM-STEM`, `NOM-ACC`, `NOM-POSS`, `PH-LEN-002`.
- **Sources**: [SMNy3 Rebrus §13.3.1 (33)-(34) pp. 693-694, §13.3.2.5 p. 726]; [T04 §4.1.1 fn. 2 p. 140]; [KVF98 §3.4.4.2.1].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `consensus`.

### PH-ZERO-003 Epenthetic verb stems
- **Statement**: About 300 verb stems (150 if the `-Vg` frequentatives are excluded) have an unstable last vowel absent before vowel-initial suffixes (and the 3sg past on the vowelless stem); the same shape conditions as `PH-ZERO-001` hold; consonant-initial suffixes and the bare 3sg keep the vowel.
- **Conditions**: verb stem flag `epenthetic`.
- **Output**: vowelless alternant before vowel-initial inflection and derivation (`-ás/-és`, `-ó/-ő`, definite `-i`, `-ik` 3sg of `-ik` verbs), full stem before consonant-initial suffixes; past distribution in `VRB-PAST` (`érzett`, but `éreztem`).
- **Examples**: `őriz`->`őrzi` [ATT 12], `őrizhet`; `becsül`->`becslés` [ATT 10]; `érez`->`érzi` [ATT 16], `éreztem`; `céloz`->`célzás` [ATT 5]; `kínoz`->`kínzás` [ATT 7]; `szerez`->`szerzünk` [ATT 6]; `viszonoz`->`viszonzom`; `füröd-`->`fürdés` [ATT 9]; `sodor`->`sodrás`; `mosolyog`->`mosolygás`; `ismétel`->`ismétlés` [ATT 11]; `közöl`->`közlés` [ATT 10]; `vezekel`->`vezeklés`, `vezekelj`; `forog`->`forgó` [ATT 14]; `ugrik`->`ugrott` [ATT 12], imperative `ugorj` [ATT 6].
- **Exceptions**: after a cluster-final base the vowel of `-Vz` is unstable only in `hang` (`hangzik`, `hangoztat`; stable in `foltozom`, `rendezem`) [T04 fn. 6]; after single-consonant bases unstable `-Vz`/`-Vl` verbs are common (`hiányzik`/`hiányoz-`, `dohányzik`, `virágzik`, `vérzik`, `lélegzik`, `ünnepel`, `énekel`; `VRB-STEM`); epenthetic `-ik` verbs: 3sg past syncopated (`fürdött`, `hiányzott`), other past cells prefer the syncopated stem for `l`/`r` members (`ugrottam`, `romlottak`) and the full stem for `d`/`z` members (`fürödtem`, `hiányoztam`) (`VRB-PAST`); bound `-ik` stems ending in clusters without a vowel alternant are defective (`PH-ZERO-004`).
- **Predictability**: `lexical` (**300 roots** of 327 members measured over 3,357 shape candidates, 31 in review; the traditional "about 150" is the non-derived subset - morphdb does not split the `-Vg` frequentatives `csavarog forog morog zörög dörög`, which are genuinely epenthetic but derived, and the list's `kind` column carries that distinction)
- **Lexical input**: verb `epenthetic` flag and alternant; the list is `epenthetic-verbs.tsv`, 327 members (300 roots), in `local/lists/epenthetic-verbs.tsv` - working data, not yet read by the build.
- **Interactions**: `VRB-STEM`, `VRB-PAST`, `VRB-IK`.
- **Sources**: [T04 §4.1.1 pp. 139-144, fn. 1]; [SMNy3 Rebrus §13.3.3 pp. 733-741]; [WC2]; [morphdb]; [hunmorph-foma]; [ATT evidence/lexical-lists.md 2026-09-16].
- **Validation**: `goldens`, `attestation`; oracle agreement on corpus-decided rows: morphdb 1713/1767, our `.ron` 1713/1767, foma 1595/1598, hunspell 1564/1567.
- **Confidence**: `consensus`; class size `measured`.

### PH-ZERO-004 Three-way lexical distinction and defective cluster stems
- **Statement**: The phonological shape cannot decide between (a) an epenthetic stem (`torony`/`torny-`), (b) a stable CVC stem (`szurony`) and (c) a cluster-final stem (`szörny`); and bound verb stems ending in an ill-formed coda cluster (`csukl-`, `bűzl-`, `vedl-`) have no vowel alternant and lack the forms that would need one.
- **Conditions**: lexicon design.
- **Output**: three stem types stored explicitly; defective paradigms for (d) `-ik` cluster stems (no bare imperative `*csukolj`, contrast epenthetic `vezekelj`).
- **Examples**: `torony`->`tornyok`, `szurony`->`szuronyok`, `szörny`->`szörnyek`; `csuklik`, `csuklás`; `bűzlik`; `vedlik`; `játszik`.
- **Exceptions**: `-`
- **Predictability**: `lexical`
- **Lexical input**: stem type.
- **Interactions**: `VRB-IK`, `PH-SYL-011`.
- **Sources**: [T04 §4.1.1 (3)-(4) pp. 141-144, fn. 21]; [SMNy3 Rebrus (38), (41) pp. 696-698].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-ZERO-005 Metathesis stems
- **Statement**: Five nouns combine vowel-zero with metathesis of `h` and the preceding `r`/`ly`: before triggering suffixes the unstable vowel is absent and `h` follows the liquid.
- **Conditions**: stems `kehely`, `pehely`, `teher`, `moholy`, `boholy` (the last two marginal).
- **Output**: `kehely`->`kelyhek`; `pehely`->`pelyhek` [ATT 4]; `teher`->`terhek`, `terhet` (`?tehert` is marginal, 91,363 : 356 [WC2]); `moholy`->`molyhok`; `boholy`->`bolyhok`; non-triggering suffixes keep the free form (`teherig`, `pehelynek`).
- **Examples**: see Output.
- **Exceptions**: KVF98 lists only the first three; the free-stem accusative `?tehert` is marginal.
- **Predictability**: `lexical` (5 stems)
- **Lexical input**: stored bound alternant.
- **Interactions**: `PH-ZERO-002`, `NOM-STEM`.
- **Sources**: [SMNy3 Rebrus p. 693]; [KVF98 §3.4.2 p. 449]; [emMorph `stemalt.rul` (study only)].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `consensus`.

### PH-ZERO-006 -alom/-elem and -adalom/-edelem derivatives
- **Statement**: Nouns ending in the non-productive `-alom/-elem` (`-adalom/-edelem`) are epenthetic (`o`/`e` of the last syllable unstable) and lowering, and take `-atlan/-etlen`.
- **Conditions**: stem ends in `-alom`, `-elem`, `-adalom`, `-edelem` as a derivative.
- **Output**: bound alternant `-alm-`, `-elm-`.
- **Examples**: `bizalom`->`bizalmat` [ATT 12], `bizalmas`, `bizalmatlan`; `figyelem`->`figyelmet` [ATT 18], `figyelmetlen`; `félelem`->`félelmet`; `vigalom`->`vigalmat`; `ártalom`->`ártalmak`; `élelem`->`élelmünk`; `forradalom`->`forradalmár`; `szidalom`->`szidalmaz`; `irodalom`->`irodalmon`; `uradalom`->`uradalmi`; `kényelem`->`kényelmetlen`.
- **Exceptions**: `-`
- **Predictability**: `rule` (given the derivational ending; list for opaque items)
- **Lexical input**: ending recognition.
- **Interactions**: `NOM-LOW`, `PH-ZERO-002`, `DER`.
- **Sources**: [SMNy3 Rebrus p. 693, pp. 509-510 (`-AtlAn`)]; [T04 fn. 9 p. 144].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `consensus`.

### PH-ZERO-007 Stem-final vowel loss before derivational suffixes
- **Statement**: Before the deadjectival verbalisers `-ít`, `-ul/-ül`, `-odik/-edik/-ödik` and the denominal `-ász/-ész` the stem-final vowel of the base (of any quality and length) is deleted; a few adjectives and nouns also lose a final `ú`/`ű` before other vowel-initial suffixes.
- **Conditions**: listed suffixes (derivation, `DER`); lexical list for inflection.
- **Output**: base without final vowel; harmony as in `PH-HARM-010`.
- **Examples**: `barna`->`barnít`; `laza`->`lazul` [ATT 1]; `szomorú`->`szomorodik`; `fekete`->`feketedik` (not `*feketédik`); `fakó`->`fakít`; `gömbölyű`->`gömbölyödik`; `erdő`->`erdész` [ATT 7]; `szőlő`->`szőlész` [ATT 4]; inflection: `könnyű`->`könnyebb` [ATT 16], `könnyen`; `ifjú`->`ifjabb` [ATT 9], `ifjan`, `ifjak`; `hosszú`->`hosszabb` [ATT 17], `hosszan`; `lassú`->`lassabb`, `lassan` [ATT 18]; `borjú`->`borjak` [ATT 5]; `varjú`->`varjak` [ATT 6].
- **Exceptions**: other stems keep the vowel before the comparative (`barnább`, `lazább`, `szomorúbb`, `feketébb`); ú-deleting nouns vary (`PH-VSTEM-006`).
- **Predictability**: `rule` for the derivational suffixes; `lexical` for inflection (4 adjectives, 6 nouns).
- **Lexical input**: stem flag for the inflectional cases.
- **Interactions**: `PH-HARM-010`, `PH-VSTEM-006`, `DER`, `NOM-ADJ`.
- **Sources**: [T04 §4.1.2.1 pp. 144-145 incl. fn. 10, 12]; [SMNy3 Rebrus §13.5.1.6 (148) p. 766].
- **Validation**: `goldens`, `attestation`.
- **Confidence**: `consensus`.

### PH-ZERO-008 Place names with partial vowel-zero
- **Statement**: Some place names show vowel-zero only before some suffixes (typically the adjectival `-i`) and keep the vowel before inflectional suffixes that would trigger it in common nouns.
- **Conditions**: listed place names.
- **Output**: per-suffix alternant in the lexicon.
- **Examples**: `Eger`->`egri` [ATT 14] but `Egert` [ATT 2], `Egeren`; common-noun homophone behaviour does not transfer.
- **Exceptions**: `-`
- **Predictability**: `lexical`
- **Lexical input**: place-name entry with `-i` alternant.
- **Interactions**: `NOM-PROP`, `ORT-NAME`.
- **Sources**: [SMNy3 Rebrus pp. 693-694]; [research-lit REPORT §5].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

---

## VSTEM: v-stems and related stem types (phonological level)

### PH-VSTEM-001 Non-shortening v-adding stems
- **Statement**: A few monosyllabic stems ending in a long vowel insert `v` before triggering (class I) suffixes without changing the vowel.
- **Conditions**: stem in the closed list; triggering suffixes as for vowel-zero (`PH-ZERO-002`), plus 3sg definite `-i` for verbs.
- **Output**: stem + `v` + suffix.
- **Examples**: (SMNy3 (135), complete) `bő` (adj) -> `bővek`, `bőven`, `bővebb`, `bővít`, `bővül` (but `bőség`, `bőnek`); `fő` (verb; non-shortening variant at 44% of 2,118 v-alternant tokens [WC2], beside shortening `fövök`, `PH-VSTEM-002`; the split runs by paradigm cell rather than by speaker: `fővök` 22 : `fövök` 9 but `fövő` 909 : `fővő` 671) -> `fővök`, `fővöm`, `fővi`, `fővő`, `fővetlen` (but `főtt`, `főhet`, `főjön`); `fú` (dialectal verb; standard derivatives `fúvós`, `fúvat`, `fúvás`, `fúvódik`); `mű` (noun) -> `művek`, `művem`, `műves`, `művész`, `művi` (but `műnek`, `műben`); `nyű` (verb) -> `nyűvök` (non-shortening variant, speaker-dependent); `rí` (verb) -> `rívok`, `rívó` (but `rínak`, `rítt`, `ríva`, `ríja`; exceptional `ríkat`).
- **Exceptions**: dialectal `hí`, `szí` pattern with `rí`; forms like `interjúvol` are unsystematic.
- **Predictability**: `lexical` (6 stems)
- **Lexical input**: `v_stem=adding`.
- **Interactions**: `NOM-STEM`, `VRB-STEM`, `PH-VSTEM-003`.
- **Sources**: [SMNy3 Rebrus §13.5.1.1 (135) pp. 755-757]; [WC2]; [ATT evidence/corpus-variants.md 2026-09-16].
- **Validation**: `goldens`, `attestation` (all v-alternant forms of `fő`, n = 2,118).
- **Confidence**: `consensus`; `measured` for `fő`, variation retained - SMNy3's `%` on both alternants is confirmed (0.44 : 0.56), there is no standard preference. `bő` and `odú` remain in review, both variants being legal and attested (`bővek` 103 : `bők` 76).

### PH-VSTEM-002 Shortening and vowel-changing v-stems
- **Statement**: Monosyllabic stems ending in `ó`, `ő`, `é`, `ú`, `ű` whose triggering-suffix alternant has a short vowel + `v`; for `ó` the short vowel is `o` or `a` (lexical).
- **Conditions**: closed list; triggering suffixes include the superessive for these stems (`tavon`, `kövön`, `füvön`, `lovon`), unlike stem-internal shortening.
- **Output**: (SMNy3 (136), complete)
  - `ó`->`a`+`v`: `tó` (`tavak`, `tava`, `tavat`, `tavon`, `tavas`), `hó` - both the 'snow' and the 'month' sense (`havak` legal, `hók` rejected [MTA-HELYES]) - (`havak`, `havat`, `havas`), `szó` (`szavak`~`szók`, `szavaz`, `szavú`), `jó` (only `javak` noun, `javít`, `javul`; adjective `jók`, `jót`).
  - `é`->`e`+`v`: `lé` (liquid: `levek`, `leve`, `levet`~`lét`; `lé` money is regular).
  - `ó`->`o`+`v`: `ló` (`lovak`, `lova`, `lovat`, `lovon`, `lovas`, `lovász`, `lovagol`); verb `ró` (`rovok`, `rovó`, `rovás`, `rovat`, but `rója`, `rótt`, `róhat`).
  - `ő`->`ö`+`v`: `kő`, `cső`, `tő`, pronoun `ő` (only `övé`, `övék`); verbs `lő`, `nő`, `sző`, `fő` (`fövök` 9 ~ `fővök` 22, but participle `fövő` 909 ~ `fővő` 671 - the split is per paradigm cell, `PH-VSTEM-001`; `fövetlen`), archaic/dialectal `jő` whose v-forms are standard (`jövök`, `jövő`, `jövendő`).
  - `ú`->`u`+`v`: **`szú` is not a v-stem** (`szuvak` rejected by the spelling checker, `szúk` 510 [MTA-HELYES; WC2]); only the lexicalised adjective `szuvas` survives. The productive `ú ~ u` v-stems are the `ú`-deleting `borjú varjú ifjú fattyú` (`NOM-STEM-012`) and the disyllabic `falu tetű daru hamu`.
  - `ű`->`ü`+`v`: `fű` (`füvek`, `füve`, `füvet`, `füvön`); verb `nyű` (`nyüvök`~`nyűvök`, `nyüvi`; speaker-dependent).
- **Examples**: see Output; `tava` [ATT 7], `hava` [ATT 9], `köve` [ATT 10]; `kőért`, `tóig`, `lóként` (non-triggering).
- **Exceptions**: regular variants `szók`, `hók`, `szót`, `jót`, `lét`, `tót` [AkH12 §30]; `j`-initial suffixes keep the long vowel stem (`rója`, `rójuk`, `lőjük`, imperative `lője`) while definite `-i` uses the v-stem (`lövi`) [AkH12 §66].
- **Predictability**: `lexical` (about 20 stems)
- **Lexical input**: `v_stem=shortening` with alternant; membership from `v-stems.tsv` (26 members, 36 in review), `local/lists/v-stems.tsv` - working data, not yet read by the build.
- **Interactions**: `NOM-STEM`, `VRB-STEM`, `PH-LEN-008`, `NOM-LOW` (nominal v-stems are lowering).
- **Sources**: [SMNy3 Rebrus §13.5.1.2 (136) pp. 757-759]; [AkH12 §30, §66]; [KVF98 §3.4.1.3.1.1 p. 447]; [WC2]; [MTA-HELYES]; [ATT evidence/lexical-lists.md 2026-09-16]; [ATT evidence/corpus-variants.md 2026-09-16].
- **Validation**: `goldens`, `attestation`; membership induced and corpus-verified over 139 shape candidates, 26 members. Working list `v-stems.tsv` (26 members, 36 in review), `local/lists/v-stems.tsv` - working data, not yet read by the build.
- **Confidence**: `consensus`; `measured` for the membership of `szú` (out), `hó` 'month' (in) and `pléh` (out, `PH-CONS-005`).

### PH-VSTEM-003 Shape conditions for v-stems
- **Statement**: Every v-adding stem is monosyllabic and ends in a long vowel, rounded except `rí` and `lé`; for verbs this shape is also sufficient (every vowel-final verb root is a v-stem), for nouns it is not (`só`, `bú`, `tű`, `szó` in part of its paradigm, `hó` 'month' in part); nominal v-stems are always lowering.
- **Conditions**: stem ends in a long vowel, one syllable.
- **Output**: verbs: `v_stem` required; nouns: lexical.
- **Examples**: verbs `lő`, `nő`, `sző`, `ró`, `nyű`, `fő`, `rí`; regular nouns `só`->`sók`, `sót`; `bú`->`búk`; `fő`->`fők`; `mű`->`művek` (v-stem) vs `tű`->`tűk` (regular); stems ending in vowel + `v` are regular consonant stems: `sav`, `hév`, `név`, `év`, `rév`, `óv`, `öv`, `szív`, `ív`.
- **Exceptions**: `bűv-` (in `bűvös`, `bűvkör`) is a consonant stem with free form only in `bűbáj`.
- **Predictability**: `rule` for verbs; `lexical` for nouns
- **Lexical input**: `v_stem` for nouns.
- **Interactions**: `PH-VSTEM-001`, `PH-VSTEM-002`, `VRB-STEM`.
- **Sources**: [SMNy3 Rebrus (137)-(139) pp. 758-760, (150)-(153) pp. 766-767]; [T04 fn. 119 p. 121].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-VSTEM-004 High-vowel-to-v stems
- **Statement**: A few disyllabic nouns ending in `u`/`ú`/`ű` replace the final vowel by `v` before the plural `-k`, the possessed plural `-i` and in derivatives; the personal possessives, the accusative and the superessive keep the vowel stem, except the third singular `-A` of a noun that takes it (`tetve`). Variation and meaning splits are strong.
- **Conditions**: closed list: `falu`, `tetű` (consistent), `daru`, `odú` (partial).
- **Output**: `falu`->`falvak`~`faluk` (71,591 : 4,267), `falvai`, `falvaink`, but `falum`, `falud`, `falunk`, `faluja`, `falujuk`, `falut`, `falun` (`falum` 3,585 : `?falvam` 17, `falunk` 10,676 : `falvunk` 0 [WC2]); `falva` survives only in place names (`-falva`); `tetű`->`tetvek`~`tetűk` (5,617 : 259), `tetve`, `tetvei`, `tetves`, but `tetűm`, `tetűt`, `tetűn` (`tetűm` 13 : `tetvem` 3 [WC2]); `daru` bird -> `darvak` vs crane machine `daruk` (3,961 : 3,606), `darvai`, but `darum`, `daruja`; `odú`->`odvas`, `odva` (hollow) vs `odúja` (dwelling), but `odúm` (48 : `odvam` 6 [WC2]).
- **Examples**: see Output; lexicalised `hamu`/`hamvas`, `hamvai` [ATT 4]; `mag`->`magvak`, `magvas` (v added to a consonant stem).
- **Exceptions**: `-`
- **Predictability**: `lexical` (4 stems plus lexicalised forms)
- **Lexical input**: class membership; the alternant per suffix follows from the statement, the meaning splits are per lexeme.
- **Interactions**: `NOM-STEM`, `PH-LEN-006`.
- **Sources**: [SMNy3 Rebrus §13.5.1.4 (142)-(143) pp. 762-763]; [AkH12 §30].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### PH-VSTEM-005 ú-deleting stems
- **Statement**: Disyllabic nouns in `ú` whose `ú` follows a consonant cluster ending in a palatal lose the `ú` before triggering suffixes, varying with the full stem; the absence of `v` is attributed to the ill-formed C + `j` + `v` sequence.
- **Conditions**: closed list: `borjú`, `ifjú`, `gyapjú`, `sarjú`, `varjú`, `fattyú`; related `fiú` (`fiam`, `fia`, `fiak`).
- **Output**: `borjú`->`borjak` [ATT 5]~`borjúk`, `borjat`~`borjút`, `borja`, `borjuk`; `varjú`->`varjak` [ATT 6]; `ifjú`->`ifjak`, `ifjonti`; `gyapjú`->`gyapjas`, `gyapja` (of a sheep) vs `gyapjúja` (of a sweater); `sarjú`->`sarjad`; `fiú`->`fiak`, `fia`.
- **Examples**: see Output; non-triggering: `borjúig`, `borjúra`, `borjúként`.
- **Exceptions**: `-`
- **Predictability**: `lexical` (6 stems + `fiú`)
- **Lexical input**: alternants.
- **Interactions**: `PH-ZERO-007`, `NOM-STEM`.
- **Sources**: [SMNy3 Rebrus §13.5.1.6 (148)-(149) p. 766]; [AkH12 §30]; [T04 fn. 12 p. 145].
- **Validation**: `attestation`.
- **Confidence**: `consensus`.

### PH-VSTEM-006 sz/d/v verbs at the phonological level
- **Statement**: A closed set of verbs has three or four stem alternants: an `sz`-stem (present, `-ja`), a `d`-stem (imperative, infinitive, potential, conditional, and for polysyllabic verbs the past), a vowel or `v`-stem (before `-ó`/`-ő`, `-ás`/`-és`, `-andó`), and for monosyllabic verbs a short vowel stem in the past (`vett`, `ivott`); the imperative of the monosyllabic set uses `-gy` (`PH-ASSIM-008`). Selection is morphological (`VRB-IRR`); phonologically, v appears only between the short stem and a vowel-initial suffix.
- **Conditions**: monosyllabic: `tesz`, `vesz`, `visz`, `hisz`, `eszik`, `iszik`, `lesz` (+ `jön`, `megy` with own stems); polysyllabic: `alszik`, `fekszik`, `nyugszik`, `haragszik`, `alkuszik`, `esküszik`; `gyanakszik`, `dicsekszik`, `törekszik` (d-stem via `-kOdik`); `igyekszik` has a `z`-stem (`igyekezhet`).
- **Output**: (SMNy3 (140), (144)) `tesz`: `tevő`, `tevés`, `teendő`, `tett`, `tehet`, `téve`, `tétel`; `vesz`: `vevő`, `vétel`, `vett`; `visz`: `vivő`, `vitt`; `hisz`: `hívő`, `hitt`, `hihet`; `lesz`: `levő`~`lévő`, `lett`, `lehet`; `eszik`: `evő`, `evés`, `evett`, `ehet`, `étel`; `iszik`: `ivó`, `ivás`, `ivott`, `ihat`, `ital`; `jön`: `jövő`, `jövendő`, `jött`, `jöhet`; `alszik`: `alvó`, `alvás`, `aludt`, `alhat`~`aludhat`, `alva`~`aludva`; `fekszik`: `fekvő`, `fekvés`, `feküdt`, `fekhet`~`feküdhet`; `nyugszik`: `nyugvó`, `nyugodott`, `nyughat`~`nyugodhat`; `haragszik`: `haragvó`, `haragudott`, `haragudhat`; `alkuszik`: `alkuvó`, `alkudhat`; `esküszik`: `esküvő`, `esküvés`, `esküdhet`; `igyekszik`: `igyekvő`, `igyekezett`, `igyekezhet`.
- **Examples**: see Output.
- **Exceptions**: other `-kszik`/`-kodik` verbs have no v-stem (`veszekszik`->`veszekedő`, `mosakszik`->`mosakodás`); `alkuszik`, `esküszik` keep their stem vowel in `sz`-forms (no vowel-zero).
- **Predictability**: `lexical` (about 17 verbs)
- **Lexical input**: stem set.
- **Interactions**: `VRB-IRR`, `VRB-STEM`, `PH-ASSIM-008`.
- **Sources**: [SMNy3 Rebrus §13.5.1.3 (140)-(141) p. 762, §13.5.1.5 (144)-(147) pp. 763-765]; [AkH12 §42, §68, §81].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

---

## G2P: pronunciation of spellings for suffixing

### PH-G2P-001 Native grapheme-to-phoneme core
- **Statement**: For native and hungarianised spellings the pronunciation relevant to suffixing is read off the letters: one letter or digraph = one segment (`PH-VOW-001`, `PH-CONS-001`), doubled letter = long segment, accented vowel = long vowel; assimilations, hiatus glides and degemination are applied only for phonetic output, never for suffix selection.
- **Conditions**: stem not flagged as foreign spelling, archaic name spelling, acronym, symbol or numeral.
- **Output**: segment string with length; derived properties for rules: vowel-final vs consonant-final, final consonant (class per `PH-CONS-003`), final cluster, vowel category string, syllable count.
- **Examples**: `asszonnyal` -> /ɒsːoɲːɒl/; `lyuk` -> /juk/; `edz` -> /ɛd͡zː/; `bodza` -> /bod͡zːɒ/; `Széchenyi` (archaic, `PH-G2P-004`) is not handled here.
- **Exceptions**: traditional spellings `PH-G2P-003`; `h` classes `PH-CONS-005`.
- **Predictability**: `rule`
- **Lexical input**: morpheme boundaries (`PH-CONS-002`).
- **Interactions**: all rules consuming "pronounced end".
- **Sources**: [AkH12 §2-§10, §17-§18, §87-§88]; [KVF98 §3.1 pp. 382-385].
- **Validation**: `goldens`, `oracle:wikipron` (study only).
- **Confidence**: `normative`.

### PH-G2P-002 Digraph parsing at morpheme boundaries
- **Statement**: Letter sequences that could form a digraph across a morpheme or compound boundary are parsed as separate segments; without morphological analysis the parser defaults to the digraph reading, which is safe for suffixing whenever the ambiguous sequence is not at the end of the stem.
- **Conditions**: `cs`, `dz`, `dzs`, `gy`, `ly`, `ny`, `sz`, `ty`, `zs` spanning a boundary.
- **Output**: see `PH-CONS-002`.
- **Examples**: `község` (`z|s`) [ATT 22], `igazság` (`z|s`), `egészség` (`sz|s`), `vízszint` (`z|sz`), `házszám` (`z|sz`), `ügyszám` (`gy|sz`), `anyja` (`ny|j`).
- **Exceptions**: stem-final ambiguity is resolved lexically (stored segment string).
- **Predictability**: `rule` (given morphology)
- **Lexical input**: morpheme boundaries.
- **Interactions**: `PH-CONS-002`, `WS-DIG`.
- **Sources**: [AkH12 §7, §48, §52]; [ESPEAK-HU `hu_list` (study only, GPL-3)].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-G2P-003 Traditional spellings
- **Statement**: A small set of words is spelled by tradition or obscured morphology rather than pronunciation; suffixing follows the pronounced form only where the norm says so, otherwise the written stem is kept and the usual rules apply to its letters.
- **Conditions**: listed words.
- **Output**: pronunciation differs; suffix spelling unchanged.
- **Examples**: `esd`, `hágcsó`, `hagyján`, `játszik` [jaːt͡sːik], `kapzsi`, `metsz` [mɛt͡sː], `mindjárt` [miɲːaːrt], `mindnyájan`, `ósdi`, `pünkösd` [pyŋkøʒd], `rögtön`, `szegfű`, `tetszik`, `utca` [ut͡sːɒ]; obscured: `kesztyű` (from `kéz`), `lélegzik` (from `lélek`), `aggat`, `faggat`, `szaggat`; `benn`/`bent`, `fenn`/`fent`; comparatives `beljebb`, `feljebb`, `kijjebb`, `lejjebb`.
- **Exceptions**: `metsz`, `látszik`, `tetszik` have pronunciation-based imperative variants (`messe`, `lássék`, `tessem`) [AkH12 §32].
- **Predictability**: `lexical` (about 20 words)
- **Lexical input**: stored spelling.
- **Interactions**: `PH-ASSIM-004`, `ORT`.
- **Sources**: [AkH12 §32, §45-§47, §90].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-G2P-004 Archaic letters in Hungarian family names
- **Statement**: Historical family names keep pre-1832 spellings; their pronunciation, which determines harmony, vowel/consonant finality and the form of `-val/-vá`, is read with the archaic letter values below; several values are ambiguous and must be stored per name.
- **Conditions**: proper name flagged `archaic_spelling` (lexical); the letters themselves are also possible in modern words, so the flag is required.
- **Output**:

  | Spelling | Value | Examples (pronunciation) | Ambiguity |
  |---|---|---|---|
  | `aa`, `aá` | á | `Gaal` [gál], `Paál` | - |
  | `eé`, `ee` | é | `Veér` [vér] | - |
  | `eö`, `ew` | ö or ő | `Eötvös` [ötvös], `Weöres` [vörös], `Thewrewk` [török], `Beöthy` [bőti], `Dessewffy` [dezsőfi] | length lexical |
  | `eő`, `öő` | ő | `Sebeők`, `Böőr` | - |
  | `eo` | ö | `Georch` [görcs] | - |
  | `oó`, `oo`, `oh` | ó | `Soós` [sós], `Kacsoh` [kacsó] | - |
  | `uu`, `uú`, `ú` | u (WPArch) or ú | `Kuun` [kun], `Kún` [kun] | length lexical |
  | `y` word-final, `ÿ` | i | `Kölcsey` [kölcsei], `Ady` [adi], `Fáy`, `Vay`, `Zay` | after `l n t g` see `PH-G2P-005` |
  | `j` final after C | i | `Bezerédj` | - |
  | `cz`, `tz` | c | `Czuczor` [cucor], `Rácz`, `Móricz`, `Rátz`, `Czetz` [cec], `Metz`, `Atzél` | `cz` = cs in some names (`Czoma`) |
  | `ch` | cs (usually), h | `Madách` [madács], `Széchenyi` [szécsényi], `Damjanich`, `Festetich`, `Cházár` [császár], `Zichy` [zicsi]; `Kulich` [h] | lexical |
  | `chs`, `csh` | cs | - | - |
  | `ts` | cs | `Takáts`, `Kováts` [kovács], `Babits` [babics], `Tsétsi` | - |
  | `th` | t | `Csáth` [csát], `Thököly` [tököli], `Báthory` [bátori], `Horváth`, `Kossuth` [kosut]~[kosút] | - |
  | `tthy` | tty | `Batthyány` [battyányi] | - |
  | `gh`, `dh` | g, d | `Balogh`, `Verseghy`, `Végh` [vég] | - |
  | `ti` | ci | `Szervátiusz` | - |
  | `ff`, `ll`, `pp`, `ss` final/medial | single consonant | `Pálffy`, `Lórántffy`, `Széll`, `Papp`, `Kiss` [kis], `Wass` [vas], `Kossuth` | `ss` = zs in `Dessewffy`, `Kolosváry` |
  | `s` | s, sz or zs | `Jósika` [józsika], `Pais`, `Sigray` [zs]; `Esterházy` [eszterházi] | lexical |
  | `z` | z, sz or c | `Cházár` [császár], `Zigány` [c] | lexical |
  | `w` | v | `Wesselényi` [veselényi], `Wass` | - |
  | `h` final after vowel | silent | `Imreh`, `Szuh` | - |
  | `x` | ksz | `Félix`, `Marx` | - |
- **Examples**: (suffixation, normative) `Kossuthtal`, `Kossuthhoz`, `Véghgel`, `Móriczcal`, `Rátzcal`, `Babitscsal`, `Madáchcsá`, `Marxszal`, `Félixszel`; vowel-final names take `-val` directly (`Kölcseyvel` pattern); names ending in a doubled consonant letter take a hyphen (`Kiss-sel`, `Papp-pal`, `Széll-lel`).
- **Exceptions**: family preference overrides; living persons use the registered form; epithet/variant names are separate entries (`Batsányi`, `Bocskai`, `Rákóczi`).
- **Predictability**: `lexical` (value per name where ambiguous; defaults from the table otherwise)
- **Lexical input**: name pronunciation field for ambiguous items.
- **Interactions**: `PH-ASSIM-002`, `ORT-NAME`, `PH-HARM-023`.
- **Sources**: [AkH12 §12, §86, §155, §159, §160 b)-c)]; [HUWIKI-ARCH]; [MESZAROS].
- **Validation**: `goldens`.
- **Confidence**: `normative` for AkH examples; `consensus` for other table values.

### PH-G2P-005 Word-final y and the ly/ny/ty/gy ambiguity
- **Statement**: In names and foreign words a word-final `y` after a vowel or after a consonant letter that does not form a digraph with it is /i/; after `l`, `n`, `t`, `g` the string may be the digraph (`ly` /j/, `ny` /ɲ/, `ty` /c/, `gy` /ɟ/) or consonant + /i/; the default is the digraph for common nouns and for names without other archaic letters, but the choice is lexical.
- **Conditions**: stem-final `y`.
- **Output**: vowel-final (/i/, category `N`) or palatal-consonant-final.
- **Examples**: /i/: `Kölcsey`, `Ady`, `Teleky`, `Zichy`, `Egressy` [egresi], `Csengery`, `Vörösmarty` [vörösmarti], `Thököly` [tököli], `Kuny` [kuni]; digraph: `Kodály` [kodáj] (-> `Kodállyal`), `Arany` (-> `Arannyal`), `Király`, `Nagy`, `Batthyány` (final `ny`); foreign: `whisky` /i/, `Kennedy` /i/, `coventryi`, `vichyi`, but `sydney-i` (`ey` digraph, hyphen).
- **Exceptions**: `-`
- **Predictability**: `lexical` for final `ly`, `ny`, `ty`, `gy` in names; `rule` otherwise.
- **Lexical input**: name pronunciation.
- **Interactions**: `PH-G2P-004`, `PH-CONS-008`, `ORT-NAME`.
- **Sources**: [AkH12 §12, §86, §160 a), §215]; [MESZAROS]; [HUWIKI-ARCH].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### PH-G2P-006 Silent letters and unusual final letter groups
- **Statement**: When a foreign-spelled word ends in a silent letter or its last pronounced sound is written with an unusual letter group, the pronounced end decides allomorphy (vowel- vs consonant-final, `-val` consonant, harmony, lengthening does not apply to the written form) and the suffix is attached with a hyphen (`ORT-FOR-012`).
- **Conditions**: foreign spelling; last written letter silent or final sound written by a multi-letter foreign group.
- **Output**: pronounced end + hyphen flag.
- **Examples**: `guillotine-t` [gijotin], `guillotine-nal`; `monsieur-nek` [möszjő]; `Glasgow-ban`; `Loire-on` [loár]; `Montesquieu-nek`; `Montreux-ig`; `Peugeot-t` [pözsó]; `Rousseau-val` [russzó]; `Voltaire-é`; `Molière-rel`; `Iaşi-sal` [jas]; derivatives `shakespeare-i`, `voltaire-es`, `bordeaux-i`, `lille-i`; exception `Anjouk`.
- **Exceptions**: `Anjouk`; hungarianised forms follow native rules.
- **Predictability**: `rule` (given pronunciation); pronunciation `lexical`.
- **Lexical input**: pronunciation respelling.
- **Interactions**: `ORT-FOR`, `PH-ASSIM-002`, `PH-G2P-008`.
- **Sources**: [AkH12 §217 a), §265 c)].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-G2P-007 Letters retained in hungarianised loans
- **Statement**: Hungarianised common words keep `ch` pronounced /h/ ([x] in coda), `x` pronounced /ks/ (usually) or /ɡz/ (in a few words), and `qu` is replaced by `kv`; given names and geographical names keep `ch` and `x`; hungarianised words otherwise follow `PH-G2P-001`.
- **Conditions**: hungarianised spelling containing `ch`, `x`, or a retained foreign letter (`bonbon`, `futball`, `millió`).
- **Output**: `ch` = /h/; `x` = /ks/ (default) or /ɡz/ (lexical: `exegézis`, `exogámia`, `hexameter`); `-val` spelled `-xszel`, `-chhel`.
- **Examples**: `jacht`, `mechanika`, `monarchia`, `pech`->`pechhel`, `technika`; `expressz`, `fix`->`fixszel`, `fixszé`, `oxigén`, `szexuális`, `taxi`, `textil`, `ortodoxszá`; `egzakt`, `egzotikus` (`x` rewritten); `bokszol`; `akvárium`, `kvartett`, `kvint`; names `Ráchel`, `Richárd`, `Alexandra`, `Xénia`, `Félix`, `Lexi`, `Albrecht`, `Luxemburg`, `Mexikó`.
- **Exceptions**: `x` = /ɡz/ items are lexical.
- **Predictability**: `rule` with a small `lexical` list.
- **Lexical input**: pronunciation for `x` = /ɡz/.
- **Interactions**: `PH-ASSIM-002`, `ORT-FOR`.
- **Sources**: [AkH12 §158, §203-§204, §209, §216 b)].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-G2P-008 Foreign-spelled words: what must be known
- **Statement**: Suffixing a foreign-spelled word needs four pronunciation facts: (1) whether the pronounced end is a vowel or a consonant and which consonant, (2) the vowel category string (harmony), (3) whether a final written `a e o ö` is pronounced (lengthening in writing), (4) whether the written end contains silent letters (hyphen); these facts are lexical for non-hungarianised words and should be stored as a Hungarian-letter pronunciation respelling; heuristics (`PH-G2P-009`) only propose candidates.
- **Conditions**: stem flagged `foreign_spelling`.
- **Output**: pronunciation respelling in Hungarian letters (e.g. `Renault` -> `rönó`, `Shakespeare` -> `sékszpír`, `Greenwich` -> `grinics`, `Bach` -> `bah`, `Wells` -> `velsz`, `Balzac` -> `balzak`, `Goethe` -> `göte`, `Cicero` -> `cicero`, `cowboy` -> `kauboj`, `Birmingham` -> `börmingem`).
- **Examples**: `cowboynak`, `fairül`, `shillingért`, `Amundsenről`, `Stockholmnál`; `Balzackal`, `Bachhal`, `Greenwichcsel`, `Steinbeckkel`, `Wellsszel`, `hertzcel`; `Goethének`, `Cicerót`; silent final letters add a hyphen (`PH-G2P-006`): `Renault` (`rönó`) -> `Renault-t`, `Renault-val`, `Renault-hoz`.
- **Exceptions**: rules can infer without a respelling: hungarianised and transliterated words (`PH-G2P-001`, `PH-G2P-014`); names with only Hungarian letters and native-like shape.
- **Predictability**: `lexical`
- **Lexical input**: `pron` respelling (Hungarian letters), optional `harmony` override.
- **Interactions**: `PH-G2P-006`, `PH-G2P-009`, `PH-HARM-023`, `PH-LEN-004`, `ORT-FOR-010`..`ORT-FOR-012`.
- **Sources**: [AkH12 §13, §214-§217]; [magyarispell `ph:` field and emMorph `pron.lx1` (study only)].
- **Validation**: `goldens`, `oracle:magyarispell`.
- **Confidence**: `normative` (principle); `lexical` (data).

### PH-G2P-009 Heuristic cues for foreign pronunciation
- **Statement**: When no respelling is stored, candidate pronunciations can be proposed from the source language's spelling conventions as used by Hungarian speakers; the output is a ranked candidate set flagged `heuristic`, never a normative form, and must be confirmed by attestation of suffixed forms (e.g. `-val` spelling reveals the final consonant, dative reveals harmony).
- **Conditions**: foreign spelling without `pron`; source language known or guessed.
- **Output**: (cues; examples from AkH12 §13, §215-§217 where available)

  | Source | Cues (spelling -> Hungarian value) | Examples |
  |---|---|---|
  | English | `sh` -> s; `ch` -> cs; `j` -> dzs; `w` -> v (initial) or u (in `ow`, `aw`); `th` -> sz/d (names often t); `ee`, `ea` -> í; `oo` -> ú; final `-y` -> i; final silent `e` after C; `-er` -> ör/er (front); `-ow` -> ó/ou | `Greenwich` [grinics], `joule` [dzsúl], `Shakespeare` [sékszpír], `Glasgow-ban`, `Kennedy` |
  | German | `sch` -> s; `tsch` -> cs; `ch` -> h; `z`, `tz` -> c; `s` before vowel -> z; `st`, `sp` initial -> st, sp; `ei` -> áj; `eu`, `äu` -> oj; `ie` -> í; `w` -> v; `ä` -> e; final `-e` pronounced (short, lengthens) | `Schäfer` [séfer], `Werther` [verter], `Bach` [bah], `Goethe` [göte], `Steinbeck` |
  | French | final `t d s x z p e` silent (hyphen); `eau`, `au` -> ó; `ou` -> u; `oi` -> oá; `ille` -> ij; `eu` -> ő; `gn` -> ny; `ch` -> s; `j`, `ge`, `gi` -> zs; `é`, `è` -> é/e; nasal vowels -> V + n in Hungarian perception; stress final | `Eugène` [özsen], `Nîmes` [nim], `Rousseau-val`, `Montreux-ig`, `Peugeot-t`, `Loire-on`, `guillotine-nal` |
  | Italian | `c`, `cc` + e/i -> cs; `ch` -> k; `g` + e/i -> dzs; `gh` -> g; `gn` -> ny; `gli` -> lyi/li; `sc` + e/i -> s; `z` -> c/dz; `qu` -> kv; final vowels pronounced (`a e o` lengthen) | `Bologna` [bolonya], `quattrocento` [kvatrocsentó], `Puccini`, `allegróban` |
  | Spanish/Portuguese | `ñ` -> ny; `ll` -> j/ly; `j` -> h; `ch` -> cs; `qu` -> k; `ão` -> au (nasal); `ç` -> sz | `señor`, `Guimarães` [gimarajs], `você` [vuszé] |
  | Czech/Polish/Slovak | `š` -> s; `č` -> cs; `ž` -> zs; `ř` -> rzs; `c` -> c; `sz` (Polish) -> s; `cz` -> cs; `w` -> v; `ł` -> u/v | `Dvořák` [dvorzsák], `Škoda` [skoda], `Wałęsa` |
  | Romanian | `ş` -> s; `ţ` -> c; `ă` -> ö-like (front/central, lexical harmony); `ea` -> ea/ja; `ce`, `ci` -> cse, csi | `Sighişoara` [szigisoára], `piaţa` [piaca], `Iaşi-sal` |
- **Examples**: see table.
- **Exceptions**: established Hungarian pronunciations override source rules (`Newton` [nyúton], `Washington` [vosington]); names borrowed via German keep German-like readings.
- **Predictability**: `tendency` (no published accuracy figures)
- **Lexical input**: source language tag.
- **Interactions**: `PH-G2P-008`, `PH-HARM-023`.
- **Sources**: [AkH12 §13, §215-§217]; [WIKIPRON `hun_latn_narrow.tsv` respellings (study only, CC BY-SA)].
- **Validation**: `attestation` (suffixed forms in corpus), `oracle:wikipron`.
- **Confidence**: `tendency`.

### PH-G2P-010 Letter names
- **Statement**: Letters read aloud (letter-by-letter acronyms, labels, letters as nouns) have fixed names (`WS-ALPH-006`); vowel letters are named by their own sound, and in letter-by-letter acronym reading they are usually pronounced long, the form tabulated here; vowel length does not change the pronounced end or harmony; the final letter name determines the suffix by default, but harmony is lexical for acronyms whose spelled-out form mixes back and neutral vowels (`BKV-val` 500 : 17, `MÁV-val` 392 : 0 against `OTP-vel` 225 : 1, `MTV-vel` 129 : 0 [ATT evidence/corpus-variants.md 2026-09-16]).
- **Conditions**: letter used as a word or as part of a spelled acronym.
- **Output**:

  | Letter | Name | End | Harmony | Letter | Name | End | Harmony |
  |---|---|---|---|---|---|---|---|
  | `a`, `á` | á | V | B | `n` | en | n | F |
  | `b` | bé | V | F | `ny` | eny | ny | F |
  | `c` | cé | V | F | `o`, `ó` | ó | V | B |
  | `cs` | csé | V | F | `ö`, `ő` | ő | V | R |
  | `d` | dé | V | F | `p` | pé | V | F |
  | `dz` | dzé | V | F | `q` | kú | V | B |
  | `dzs` | dzsé | V | F | `r` | er | r | F |
  | `e`, `é` | é | V | F | `s` | es | s | F |
  | `f` | ef | f | F | `sz` | esz | sz | F |
  | `g` | gé | V | F | `t` | té | V | F |
  | `gy` | gyé | V | F | `ty` | tyé | V | F |
  | `h` | há | V | B | `u`, `ú` | ú | V | B |
  | `i`, `í` | í | V | F | `ü`, `ű` | ű | V | R |
  | `j` | jé | V | F | `v` | vé | V | F |
  | `k` | ká | V | B | `w` | duplavé | V | F (Bé shape; see Exceptions) |
  | `l` | el | l | F | `x` | iksz | sz (CC /ks/) | F |
  | `ly` | ellipszilon | n | B | `y` | ipszilon | n | B |
  | `m` | em | m | F | `z` | zé | V | F |
  |  |  |  |  | `zs` | zsé | V | F |
- **Examples**: `DNS-sel` (dé-en-es), `ENSZ-szel` read as a word, `tbc-s` (tébécé), `tv-ben` (tévé), `EKG-n` (é-ká-gé), `URH-adás`, `az f-et`, `az x-szel`; spelled-out forms `tébécés`, `tévézik`, `pévécé`.
- **Exceptions**: `duplavé` is a `Bé` word by shape (zone of variation); Lightproof treats it as front; the solid spelling is attested 82 : 12 against `dupla vé` [WC2], the suffixed acronym reading is not. `ly` also named `elipszilon`. The harmony of a letter-read acronym is **not** derivable from its final letter name: `BKV` and `MTV` both end in `vé` and take opposite suffixes, and `OTP` (`ótépé`) contains a back `ó` and still takes front, so the dominant-vowel account fails too. Acronym harmony is lexical per acronym (`ORT-ABBR-018`).
- **Predictability**: `rule` (table), `lexical` for `w` and for the harmony of letter-read acronyms.
- **Lexical input**: `none`
- **Interactions**: `PH-G2P-011`, `ORT-ABBR`, `NOM-ACC`, `PH-HARM-023`.
- **Sources**: [AkH12 §88 ([ellipszilon]), §285 b), §288]; [LP-HU `lightproof_hu_HU.py` letter harmony list (study only, MPL/GPL/LGPL)]; [ATT evidence/corpus-variants.md 2026-09-16] (acronym suffix counts); [WC2] (`duplavé`).
- **Validation**: `goldens`; `attestation` (acronym suffix counts over 11 sources).
- **Confidence**: `consensus` (table); `measured` (acronym harmony is item-lexical, not register-conditioned).

### PH-G2P-011 Acronyms: word-like vs letter-by-letter reading
- **Statement**: An acronym is read either as a word (by `PH-G2P-001` on its letters) or letter by letter (by `PH-G2P-010`); the shape decides the reading, as a word where the letters spell a pronounceable one (a vowel, an onset of one consonant or a loan onset, a well-formed final cluster: `MÁV`, `ELTE`, `ENSZ`) and by letter names otherwise (`MTA`, `OTP`, `KSH`), with a lexical spoken form overriding it (`AIDS` [édsz]); suffixes follow the reading in harmony, linking vowels and `-val` assimilation, while spelling conventions (hyphen, no written lengthening in all-caps) are in `ORT-ABBR-014`, `ORT-ABBR-015`.
- **Conditions**: all-caps or lowercase acronym (`betűszó`) or blend (`szóösszevonás`).
- **Output**: reading mode + pronounced string.
- **Examples**: word-like: `NATO` [nátó], `ENSZ` [ensz] -> `ENSZ-szel`, `MÁV` [máv] -> `MÁV-ot`, `ELTE` [elte] -> `ELTE-re`, `ELTE-n`, `UNESCO`, `áfa` -> `áfát`, `Alitalia` -> `Alitaliánál`, `Orex` -> `Orexszel`, `gyes` -> `gyesen`; letter-wise: `MTA` [em-té-á] -> `MTA-nak`; `MTI-hír`, `BKK-nál`, `KSH` [ká-es-há], `DNS-sel`, `EKG-n`, `OTP` [ó-té-pé] -> `OTP-nél`, `tbc-s`, `tv-zik`.
- **Exceptions**: some acronyms have both readings (`ELTE` can be letter-read in speech); written lengthening never applies to all-caps acronyms even when read as words (`ETO-hoz`) (`PH-LEN-003`), but applies to blends written in lowercase (`stukák`, `Hungexpónál`).
- **Predictability**: `lexical`
- **Lexical input**: acronym `reading` (`word` | `letters`), optional `pron`.
- **Interactions**: `ORT-ABBR`, `PH-G2P-010`, `PH-LEN-003`.
- **Sources**: [AkH12 §285-§289]; [SMNy3 Rebrus p. 787 (`FIF[á]-nak`, `ELT[é]-vel` pronunciation)].
- **Validation**: `goldens`.
- **Confidence**: `normative` (suffix follows pronunciation); `lexical` (reading).

### PH-G2P-012 Numerals written with digits
- **Statement**: A number written with digits is suffixed according to its spoken form; only the last numeral word of the spoken form matters (its final segment, harmony, stem alternations); ordinals written `N.` are read as ordinals; `2` is read `kettő` before suffixes.
- **Conditions**: digits + hyphenated suffix (`ORT-NUM`); spell-out from `PND`.
- **Output**: (last spoken word; examples in hyphenated digit spelling)

  | Last word | Value | End | Harmony | `-val` | `-hoz` | accusative | superessive |
  |---|---|---|---|---|---|---|---|
  | `egy` | 1 | gy | F | `1-gyel` | `1-hez` | `1-et` | `1-en` |
  | `kettő` | 2 | ő | R | `2-vel` | `2-höz` | `2-t` | `2-n` |
  | `három` | 3 | m | B | `3-mal` | `3-hoz` | `3-at` (`hármat`) | `3-on` |
  | `négy` | 4 | gy | F | `4-gyel` | `4-hez` | `4-et` | `4-en` |
  | `öt` | 5 | t | R | `5-tel` | `5-höz` | `5-öt` | `5-ön` |
  | `hat` | 6 | t | B | `6-tal` | `6-hoz` | `6-ot` | `6-on` |
  | `hét` | 7 | t | F | `7-tel` | `7-hez` | `7-et` (`hetet`) | `7-en` |
  | `nyolc` | 8 | c | B | `8-cal` | `8-hoz` | `8-at` | `8-on` |
  | `kilenc` | 9 | c | F | `9-cel` | `9-hez` | `9-et` | `9-en` |
  | `tíz` | 10 | z | F | `10-zel` | `10-hez` | `10-et` (`tízet`) | `10-en` |
  | `húsz` | 20 | sz | B | `20-szal` | `20-hoz` | `20-at` | `20-on` |
  | `harminc` | 30 | c | B | `30-cal` | `30-hoz` | `30-at` | `30-on` |
  | `negyven` | 40 | n | F | `40-nel` | `40-hez` | `40-et` | `40-en` |
  | `ötven` | 50 | n | F | `50-nel` | `50-hez` | `50-et` | `50-en` |
  | `hatvan` | 60 | n | B | `60-nal` | `60-hoz` | `60-at` | `60-on` |
  | `hetven` | 70 | n | F | `70-nel` | `70-hez` | `70-et` | `70-en` |
  | `nyolcvan` | 80 | n | B | `80-nal` | `80-hoz` | `80-at` | `80-on` |
  | `kilencven` | 90 | n | F | `90-nel` | `90-hez` | `90-et` | `90-en` |
  | `száz` | 100 | z | B | `100-zal` | `100-hoz` | `100-at` | `100-on` |
  | `ezer` | 1000 | r | F | `1000-rel` | `1000-hez` | `1000-et` (`ezret`) | `1000-en` |
  | `millió` | 10^6 | ó | B | `-val` | `-hoz` | `-t` | `-n` |
  | `milliárd` | 10^9 | d | B | `-dal` | `-hoz` | `-ot` | `-on` |
  | `nulla` | 0 | a | B | `0-val` | `0-hoz` | `0-t` | `0-n` |
- **Examples**: `4-et`, `12-t`, `19-szer`, `32-en` (harmincketten) vs `32-n` (harminckettőn), `67-es`, `1848-ban`, `2024-ben`, `16.25-kor`; ordinals: `3.-nak` (harmadiknak), `8.-ba`, `10.-kel` (tizedikkel).
- **Exceptions**: accusative and other linking-vowel forms depend on the numeral stem class (`három`->`hármat`, `hét`->`hetet`, `tíz`->`tízet` (not shortening, `PH-LEN-010`), `ezer`->`ezret`) (`PND`); dates and years have their own suffix rules (`ORT-NUM-017`).
- **Predictability**: `rule`
- **Lexical input**: numeral spell-out (`PND`).
- **Interactions**: `PND`, `ORT-NUM`, `PH-HARM-023`, `PH-ASSIM-002`.
- **Sources**: [AkH12 §82 f), §265 d), §292, §295]; [CLDR-RBNF (Unicode licence)]; [LP-HU numeral table (study only)].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-G2P-013 Abbreviations and symbols
- **Statement**: Abbreviations and symbols are suffixed according to the full word they abbreviate, including its linking vowel; the expansion is lexical.
- **Conditions**: abbreviation or symbol with hyphenated suffix (`ORT-ABBR-009`).
- **Output**: suffix from the expansion.
- **Examples**: `u.-ban` (utcában), `Ft-ot`, `Ft-tal` (forint), `ker.-nek` (kerületnek), `ÉK-en` (északkeleten), `K-ot`, `K-mal` (kálium), `m²-enként`, `m²-es` (négyzetméter), `°C-on`, `°C-kal`, `°C-os` (Celsius-fok), `15%-kal`, `80%-os` (százalék), `15. §-ban` (paragrafus), `Bp.-en`, `Bp.-tel` (Budapest).
- **Exceptions**: an abbreviation with several expansions needs context (`sz.` = `szám` or `század`).
- **Predictability**: `lexical`
- **Lexical input**: expansion table (`ORT-ABBR`).
- **Interactions**: `ORT-ABBR`, `PH-HARM-023`.
- **Sources**: [AkH12 §282, §284, §265 d)].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### PH-G2P-014 Transliterated words
- **Statement**: Words and names transliterated from non-Latin scripts by Hungarian rules are read by Hungarian letter values and suffixed like native words, with no hyphen.
- **Conditions**: transliterated spelling (Hungarian letters only).
- **Output**: `PH-G2P-001` applies.
- **Examples**: `Tobrukig`, `Szöulnál`, `Csehovtól`, `Csajkovszkijjal`, `Plutarkhosszal`, `száriban`, `tatamin`, `Szapphóé`, `kamikazék`, `arisztotelészi`, `kuvaiti`, `tel-avivi`, `dzsúdózna`.
- **Exceptions**: other transliteration systems used in special fields require `PH-G2P-008`.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-FOR`.
- **Sources**: [AkH12 §218-§222].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

---

## Open questions
1. `PH-HARM-014`: the consonant effects are replicated in HZSL09 (lexicon, wug test) but not in F13 (national corpus) except final CC; re-estimate all MaxEnt weights on the project corpus with suffix-type separation before enabling the bilabial, sibilant and coronal-sonorant constraints.
2. `PH-HARM-013`: no published weights exist for suffix-type-specific vacillation (`V` vs `C` suffixes, RR24, F13) or for foreign pseudo-compound pieces (`-méter`, `-gén`, `bio-`); a corpus fit is needed.
3. `PH-HARM-008`: the complete antiharmonic list (about 60 roots, Vago 1980: 8-12) was not available locally; membership must be compiled from the lexicon oracles and verified by attestation (e.g. `nyír`, `szid` forms are unattested in the corpus list).
4. `PH-HARM-019`: vacillation induced by `-né` and by strings of neutral suffixes (`-i`+`-é`, `-i`+`-ék`) lacks item-level data.
5. `PH-G2P-010`: harmony of the letter name `duplavé` (`w`) and of letter-read acronyms ending in `bé`, `vé` after a back letter name (e.g. `BKV`) is unverified.
6. `PH-G2P-004`/`PH-G2P-005`: value of ambiguous archaic letters (`eö`, `ch`, `cz`, `s`, `z`, `ss`, final `ly`/`ty`) must be stored per name; no complete register is openly licensed (AkH gives examples, Wikipedia list is CC BY-SA and study-only).
7. `PH-SYL-008`/`PH-SYL-009`: speaker variation for bare vs linking-vowel accusative after clusters and geminates (`briliánst`~`briliánsot`, `orrt`~`orrot`) needs corpus shares; the project corpus lacks most items.
8. `PH-LEN-002`: whether compound-like elements (`-beli`, `-fajta`, `-mentes`, `-szerte`) are suffixes or compound members is to be fixed with `DER`; lengthening behaviour is unattested in the corpus list.
9. `PH-SYL-013`: the conservative vs innovative standard for final high vowel length in polysyllables is not encoded; spelling is used.
10. `PH-G2P-009`: foreign pronunciation heuristics have no accuracy evaluation; a respelling lexicon with an open licence is needed.
11. (resolved 2026-09-15: `NOM` references to obsolete area names were updated to `PH-G2P`/`PH-CONS-005` in 03.)
12. `PH-VSTEM-002`: the status of `szú`, `jó` and `hó` ('month') in the v-stem class is marginal in SMNy3; forms need attestation.

## Sources used
All keys are in [sources.md](sources.md). Reliability notes for this chapter:
- `HUWIKI-ARCH`: values cross-checked with AkH12.
- `HZSL09`: page numbers refer to the 2009 preprint.
- `research-lit`: internal notes, not published.

## Terms
See [glossary.md](glossary.md).
