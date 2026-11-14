# 06 Derivation and compounding

Scope: derivational suffixes of Hungarian (noun-, adjective-, verb- and adverb-forming), diminutives and hypocoristics, prefix-like elements (verbal prefixes as compound members, `leg-`, bound first members), compounds (structure, head, harmony and allomorphy inherited from the last member, boundary spelling effects, algorithmic splitting) and the lexical side of word formation (lexicalisation, blocking, opaque derivatives). For every suffix the chapter gives the properties an engine needs: input and output category, allomorphs and their selection, linking vowel type, harmony behaviour, low vowel lengthening, stem alternant selection, truncation, output lowering, output 3sg possessive preference and productivity. Inflectional allomorphy (plural, cases, possessives, verb conjugation) is owned by `NOM`, `VRB` and `PND`; spelling of compounds (writing together or apart, hyphens, 6:3 rule) by `ORT`; harmony computation by `PH-HARM`. This chapter states what derivation and compounding feed into those rules.

Terminology: "derivational suffix" (képző), "linking vowel" (kötőhangzó), "low vowel lengthening" (tővégi nyílt magánhangzó nyúlása, LVL), "lowering stem" (nyitótő), "epenthetic stem" (hangkivető tő), "v-stem" (v-vel bővülő tő), "shortening stem" (rövidülő tő), "semi-word" (félszó), "verbal prefix" (igekötő).

## Area index
| Area | Topic |
|---|---|
| `DER-MODEL` | derivation vs inflection, the suffix property model, linking vowel types, LVL, stem alternants, truncation, harmony classes, output properties, productivity, master table |
| `DER-NOUN` | noun-forming suffixes |
| `DER-ADJ` | adjective-forming suffixes, semi-word adjectival elements, loan adjective endings |
| `DER-VERB` | verb-forming suffixes, loan verbalisers |
| `DER-ADV` | adverb-forming suffixes, choice of `-An`, `-Ul`, `-lAg` |
| `DER-DIM` | diminutives and hypocoristics |
| `DER-PREF` | verbal prefixes as compound members, `leg-`, bound and foreign first members |
| `DER-COMP` | compounds: structure, head, inheritance, boundary effects, splitting |
| `DER-LEX` | lexicalised derivatives, blocking, opaque and potential words |

## Notation used in this chapter
In addition to the README notation:
- Linking vowel types (see `DER-MODEL-004`): `-` none; `O` plural-type (mid `o/e/ö` after non-lowering stems, low `a/e` after lowering stems, absent after vowels); `A` fixed low `a/e`; `M` fixed mid (`o/e/ö` or a subset, independent of lowering); `V` suffix-initial vowel present after every stem.
- Harmony classes (see `DER-MODEL-008`): `H2` two-way backness alternation (`a/e`, `á/é`, `ó/ő`, `u/ü`, `ú/ű`); `H3` three-way `o/e/ö`; `H2r` two-way rounded `o/ö`; `N` invariant, neutral vowel, transparent; `B!` invariant back, opaque; `F!` invariant front, opaque; `base` harmony computed from the untruncated base.
- LVL (see `DER-MODEL-005`): `y` lengthens stem-final `a/e`; `n` does not; `-` never follows `a/e`.
- Stem (see `DER-MODEL-006`): `free` citation form; `bound` epenthetic vowel dropped, v-stem and shortened alternant used; `bound(ep)` only the epenthetic drop; `bound(-ep)` shortening and v-stem alternants but the epenthetic vowel kept; `lex` lexically specified.
- Productivity (see `DER-MODEL-012`): `P` productive; `P-dom` productive in a stated domain; `R` unproductive but regular on a closed class; `L` lexical only.
- Stress (see `DER-COMP-016`): a leading `'` marks primary stress and a `,` inside a word secondary stress. A stress-marked string is a prosodic annotation, not a spelling, so it is no golden.

## DER-MODEL

### DER-MODEL-001 Criteria separating derivation, inflection and compounding
- **Statement**: A suffix is classified as derivational, inflectional or semi-word (compound member) by one-directional sufficient criteria; no single criterion is necessary.
- **Conditions**:
  - Derivational if any holds: it changes the word class of the base (`olvas` V -> `olvasás` N); it changes argument structure (causatives, privative on verbs); at least one output is lexicalised with non-compositional meaning (`írás` 'script', `látogat`); it can be followed by another derivational suffix (`ház-anként-i`, `négy-szer-es`)
  - Inflectional if any holds: it takes part in agreement (person, number, definiteness); it attaches productively to pronouns and phrases (case suffixes, `-ék` on possessed nouns `szomszédaimék`); it can follow another inflectional suffix
  - Semi-word (compound member, see `DER-COMP-017`) if the element permits coordination ellipsis from both directions (`kutya- és macskaszerű`), does not undergo or trigger LVL, and forms its own harmony domain
  - A suffix that is not followed by any further suffix is not thereby inflectional: derived adverbs are terminal (`DER-ADV-010`)
- **Output**: one of derivational, inflectional, `semi-word`. Classification used by this rulebook:
  - derivational: all suffixes in the master table (`DER-MODEL-016`), including adverb-forming `-An`, `-Ul`, `-lAg` (surface forms owned by `NOM-ADJ-010`..`NOM-ADJ-012`)
  - case-like, derivational by the Rebrus criteria but inflected in `NOM` (forms owned there, status and further derivation here): `-kor` (`NOM-CASE-017`), `-nként` (`NOM-CASE-018`), `-stUl` (`NOM-CASE-019`), `-képp(en)` (`NOM-CASE-020`), `-ntA` (`NOM-CASE-021`)
  - inflectional, owned elsewhere: `-hAt` (`VRB-MODEL-003`), associative `-ék` (`NOM-ASSOC-001`), anaphoric `-é` (`NOM-ANP-001`), essive-formal `-ként` (`NOM-CASE-008`), essive `-Ul` on nouns (`NOM-CASE-009`), terminative `-ig` (`NOM-CASE-007`), multiplicative `-szOr` (`PND`), participles and converbs (`VRB-PART`, `VRB-ADVP`)
  - semi-word: `-szerű`, `-féle`, `-fajta`, `-forma`, `-nemű`, `-rét`, `-rétű`, `-mentes`, `-né`; `-beli` is borderline (semi-word by coordination `tér- és időbeli` and lack of LVL; suffix-like because it is not an independent word)
- **Examples**: `szép` -> `szépség` (class change, derivational); `lát` -> `látogat` (lexicalised, derivational); `szomszédaimék` (after possessive, inflectional); `kutya- és macskaszerű` (ellipsis, semi-word); `*ír- és olvasás` (no ellipsis of a derivational suffix's base)
- **Exceptions**: `-beli` follows plural and possessive suffixes (`falumbeli`, `hónapokbeli`), which a derivational suffix otherwise cannot; treated as semi-word-like (`DER-ADJ-012`). Participle suffixes change class but are inflectional by the paradigm criterion; their outputs convert to adjectives (`DER-MODEL-015`).
- **Predictability**: `rule` (per suffix, decided once and stored in the suffix record)
- **Lexical input**: none
- **Interactions**: `DER-MODEL-003` (ordering), `DER-COMP-001`, `DER-COMP-017`; `NOM-CASE` for the case list
- **Sources**: [SMNy3 pp. 94-99 (Keszler 2.3), pp. 105-110 (Kiefer-Ladányi 3.3 criteria 10, 11, 12, 14, 21), pp. 123-125], [SMNy3 pp. 668-669 (Rebrus 13.1.3 criteria 4)], [SMNy3 pp. 54-56 (semi-words)], [KVF98 pp. 351, 353, 359]
- **Validation**: goldens (classification table), oracle:emMorph (tag inventory comparison)
- **Confidence**: contested: SMNy3 Kiefer-Ladányi treat `-An`, `-Ul` as derivational and `-hAt`, `-ék` as inflectional; KVF98 lists `-hat/-het` as "generally regarded derivational" but argues inflectional; Rebrus classifies `-ként` as case but `-kor`, `-nként`, `-képp`, `-stUl`, `-szOr`, `-ntA` as derivational; AkH12 calls `-szerű`, `-féle`, `-fajta` utótag (compound member)

### DER-MODEL-002 Suffix record fields
- **Statement**: Every derivational suffix is stored as one record whose fields fully determine attachment, surface form and the properties of the output stem.
- **Conditions**:
  - `id`: rule ID of the suffix
  - `template`: archiphoneme template (`-(O)s`, `-sÁg`, `-(j)Ú`)
  - `input`: allowed base categories (N, ADJ, V, NUM, ADV, POSTP, PRON) with semantic and morphological conditions
  - `output`: category of the derivative
  - `allomorphs`: ordered list of (condition -> form)
  - `link`: linking vowel type (`DER-MODEL-004`)
  - `harmony`: harmony class (`DER-MODEL-008`)
  - `lvl`: triggers low vowel lengthening or not (`DER-MODEL-005`)
  - `stem`: stem alternant selected (`DER-MODEL-006`)
  - `trunc`: deletes stem-final vowel or a listed ending (`DER-MODEL-007`)
  - `out_lowering`: lowering value of the output stem (`DER-MODEL-010`)
  - `out_poss3`: 3sg possessive variant of the output (`DER-MODEL-011`)
  - `out_features`: other inherited or assigned features (v-stem never; epenthetic only for `-alom`; `-ik` for verbs)
  - `productivity`: `P`, `P-dom`, `R`, `L` (`DER-MODEL-012`)
  - `blocks_after` / `allows_after`: morphotactic restrictions (`DER-MODEL-003`)
  - `semi_word`: boolean (`DER-COMP-017`)
- **Output**: a record per suffix; the master table (`DER-MODEL-016`) gives the values for every suffix in this chapter
- **Examples**: `-sÁg`: input ADJ (P), N occupational (P-dom); output N; allomorphs `-ság/-ség`; link `-` (lexical `-esség` 15 items); harmony `H2`; lvl `n`; stem free; trunc no; out_lowering no; out_poss3 `-a`; productivity P
- **Exceptions**: none; lexical exceptions of a suffix are stored as lexicon entries (`DER-LEX-001`), not in the record
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: all `DER-MODEL` rules
- **Sources**: [SMNy3 pp. 113-118 (Kiefer-Ladányi 3.5, information required by a word-formation rule)], [SMNy3 pp. 671-677 (Rebrus 13.1.4 notation: `]` non-lengthening, truncating, `}` terminal)], [morphdb grammar AFF_* blocks]
- **Validation**: goldens
- **Confidence**: consensus

### DER-MODEL-003 Ordering of derivation, compounding and inflection
- **Statement**: Derivational suffixes precede inflectional suffixes; derivation and compounding feed each other freely; inflected forms do not feed derivation or compounding except in the listed cases.
- **Conditions**:
  - order inside a word form: (prefix) + stem + derivational suffixes* + inflectional suffixes
  - a compound may be derived (`rendszerfelelősség`, `vaskezű`) and a derivative may be a compound member (`újságolvasás`)
  - derivational suffixes may stack, subject to each suffix's input conditions (`barát-ság-talan`, `ver-het-etlen-ség`)
  - derived adverbs in `-An`, `-Ul`, `-képp`, `-stUl`, `-ntA` are terminal (`DER-ADV-010`)
- **Output**: morphotactic acceptance or rejection of a suffix sequence
- **Examples**: `szép-ség-es`; `*szép-ek-ség`; `*lát-unk-ás`; `ház-anként-i`; `tulajdon-képpen-i`; `múlt-kor-i`; `utó-lag-os`
- **Exceptions**:
  - `-beli` after plural and possessive: `falumbeli`, `városodbeli`, `(az elmúlt) hónapokbeli` (`DER-ADJ-012`)
  - `-i` after case-like derivational suffixes and after postpositions: `fejenkénti`, `hajnalkori`, `utáni`, `mögötti` (`DER-ADJ-001`)
  - lexicalised case forms reanalysed as adverbs or prefixes (`hátra`, `agyon`, `nagybani`) are lexicon entries (`DER-LEX-008`)
  - `-hAt` (inflectional) is followed only by `-hatÓ`, `-hatatlan`, `-hatnék`, which are stored as unanalysed suffixes (`DER-ADJ-019`, `DER-ADJ-020`)
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-MODEL-001`, `DER-MODEL-013`; `VRB-POT`
- **Sources**: [SMNy3 pp. 108-110 (13, 13', criterion 14)], [SMNy3 pp. 123-124], [SMNy3 p. 139 (`-beli` after inflection)], [SMNy3 pp. 668-669]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-MODEL-004 Linking vowel types of derivational suffixes
- **Statement**: Each derivational suffix belongs to exactly one linking vowel type, which decides whether a vowel appears between stem and suffix and its quality.
- **Conditions**:
  - type `-` (none): suffix is consonant-initial and never takes a vowel: `-ság`, `-talan` (C-final default), `-nyi`, `-beli`, `-szerű`, `-ka`, `-né`, `-kor`, `-lAg`
  - type `O` (plural-type): after a consonant-final stem a vowel appears; its quality is that of the plural linking vowel of the same stem (mid `o/e/ö` by `H3` for non-lowering stems, low `a/e` for lowering stems, `NOM-LOW`); after a vowel-final stem no vowel appears and LVL applies: `-(O)s`, `-(O)cskA`, `-(O)z`, `-(O)l`, `-(O)nként`, `-(O)stUl`, `-(O)skOdik`, `-(O)sít`, `-(O)sOdik`
  - type `A` (fixed low): vowel `a/e` after consonant-final stems regardless of lowering: privative `-atlan/-etlen` on verbs (`DER-ADJ-010`), `-(A)lOm` (`DER-NOUN-012`), `-(A)dék`, `-(A)lék` (`DER-NOUN-023`), `-(A)ttyÚ`, `-(A)ll` (`DER-VERB-011`)
  - type `M` (fixed mid): vowel quality independent of lowering; the suffix defines the vowel set: `-(O)gAt` (`o/e/ö`), `-(O)mÁny` (`o/e` only), `-(O)vÁny` (`o/e` only), `-(O)dA` (`o/ö` only)
  - type `V` (suffix-initial vowel): the vowel belongs to the suffix and appears after every consonant-final stem; on vowel-final stems the suffix truncates or does not apply: `-Ás`, `-Ó`, `-At`, `-Ász`, `-ista`, `-izmus`, `-itás`, `-ít`, `-Ul`, `-Odik`, `-Ódik`, `-ál`, `-izál`
- **Output**: presence and quality of the vowel between stem and suffix
- **Examples**: `ház` (lowering) -> `házas`, `házacska`; `hód` (non-lowering) -> `hódos`, `hódocska`; `föld` -> `földes`; `gödör` -> `gödröcske`; `tanul` -> `tanulatlan` (type A); `ad` -> `adogat` (type M, `ad` has no lowering relevance); `gyűjt` -> `gyűjtemény` (type M, never `*gyűjtömény`); `önt` -> `öntöde` (type M, never `*öntede`)
- **Exceptions**: adjectives (all lowering except `nagy`, `agg`, ethnonyms and language names, `vak`, `gazdag`, `szabad`, `vastag`) take low vowels with type `O`: `piros` -> `pirosas`, `vörös` -> `vöröses`; `havonta`, `havonként` use mid `o` although `hó/hav-` is lowering (lexical, 2 items)
- **Predictability**: `rule` (type per suffix); vowel quality inherits the lexical lowering value of the stem
- **Lexical input**: stem property `lowering` (`NOM-LOW`); stem harmony class
- **Interactions**: `DER-MODEL-005`, `DER-MODEL-006`, `NOM-LOW`, `PH-HARM`; applies after stem alternant selection
- **Sources**: [SMNy3 pp. 118-119 (3.6 linking vowel vs suffix-initial vowel)], [SMNy3 pp. 672-677 (Rebrus 13.1.4 per-suffix notes: `-(O)gAt`, `-(o/e)mÁny`, `-(o/e)vÁny`, `-(O)dA`)], [T04 pp. 157-161], [morphdb grammar AFF_S, AFF_KA, AFF_Z, AFF_L, AFF_STUL, AFF_NKÉNT (low/non_low selection)], [morphdb derivation README (NTA: havonta lexical)]
- **Validation**: goldens, attestation (`házacska`, `havonta`), oracle:hunmorph-foma
- **Confidence**: consensus

### DER-MODEL-005 Low vowel lengthening classes
- **Statement**: A stem-final `a` or `e` becomes `á` or `é` before lengthening suffixes and stays short before non-lengthening suffixes and before compound members; the value is a fixed property of the suffix.
- **Conditions**:
  - stem ends in short `a` or `e` (common noun, adjective, name spelled natively, word-like blend)
  - lengthening (`y`) derivational suffixes: `-(O)s`, `-(O)cskA`, `-tlAn`, `-(j)Ú`, `-(O)z`, `-(O)l`, `-(A)ll`, `-(O)skOdik`, `-nyi`, `-né`, `-(O)stUl`, `-(O)nként`, `-An`, `-Ul` (adverb), `-(O)sít`, `-(O)sOdik`, `-(O)sdi`
  - non-lengthening (`n`) derivational suffixes: `-sÁg`, `-i` (relational adjective), `-beli`, `-szerű`, `-féle`, `-fajta`, `-forma`, `-nemű`, `-rét(ű)`, `-mentes`, `-kor`, `-képp(en)`, `-ntA`, `-szOr` (`PND`); `-ista`, `-izmus` when they do not truncate (`dadaista`, `dadaizmus`)
  - truncating suffixes delete the vowel instead (`DER-MODEL-007`): `-ít`, `-Ul` (verb), `-Odik`, `-Ász`, `-ista` and `-izmus` (usually), hypocoristic `-i`
  - not applicable (`-`): `-kA` (never after `a/e`, `DER-DIM-001`), deverbal suffixes (verb stems never end in `a/e`)
- **Output**: `á/é` before `y` suffixes; unchanged before `n` suffixes
- **Examples**: `bunda` -> `bundácska`; `ruha` -> `ruhátlan`; `szoknya` -> `(rövid) szoknyájú`; `tonna` -> `tonnányi`; `Katona` -> `Katonáné`; `vacsora` -> `vacsorázik`; `drága` -> `drágán`; `francia` -> `franciául`; `óra` -> `óránként`; `buta` -> `butaság`; `anya` -> `anyaság`, `anyai`; `szakma` -> `szakmabeli`; `óra` -> `órakor`; `hülye` -> `hülyeség`; `Coca-Cola` -> `Coca-Cola-szerű`
- **Exceptions**: all-capital acronyms never lengthen (`ELTE-s`, `MTA-beli`, `ORT-ABBR`); foreign-spelled words and names lengthen a pronounced final `a/e` by pronunciation, but not before non-lengthening suffixes (`Goethének`, `goethés`, but `goethei`, `Coca-Cola-szerű`; `ORT-FOR`); marginally inflected function words keep the short vowel (`NOM-LENG-003`)
- **Predictability**: `rule`
- **Lexical input**: none for common words; stem property `acronym`, `foreign_spelling`
- **Interactions**: `NOM-LENG-001` (inflectional LVL), `DER-MODEL-013`, `DER-COMP-005` (no LVL at compound boundary), `ORT-SUFX-001`, `ORT-SUFX-002`
- **Sources**: [AkH12 §29, §216 a), §288], [SMNy3 pp. 787-788 (Rebrus 13.6.1.1, table 188: non-lengthening `-ként`, `-képp(en)`, `-ság`, `-szerű`, `-féle`, `-kor`, `-szor`, `-i`, `-izmus`, `-ista` (`dadaista`), marginal `-ológia`, `-izál`, `-ikus`; generalisation 189: non-lengthening suffixes are "superanalytic")], [SMNy3 pp. 671-677 (`]` marks)], [RST12 p. 497 fn. 10 (`anya-ság`, `hülye-ség`)], [morphdb grammar (lengthened / non_lengthened features per AFF block)]
- **Validation**: goldens, attestation (`katonáné`, `tonnányi`, `butaság`, `anyaság`; `*butáság`, `*anyáság` unattested), oracle:hunmorph-foma
- **Confidence**: consensus; `-ntA` after `a/e` is rare and its value is inferred from morphdb (`non_lengthened`)

### DER-MODEL-006 Stem alternant selected by a derivational suffix
- **Statement**: A derivational suffix attaches either to the free (citation) stem or to the bound stem, where the bound stem is the alternant used before the plural: epenthetic vowel dropped, v-stem, shortened vowel.
- **Conditions**:
  - `free`: no epenthetic drop, no v insertion, no shortening: `-sÁg`, `-kA`, `-nyi`, `-beli`, `-szerű`, `-féle`, `-fajta`, `-mentes`, `-né`, `-kor`, `-képp(en)`, C-initial privative `-tAlAn/-tlAn`
  - `bound`: all three alternations apply where the stem has them: `-(O)s`, `-(O)cskA`, `-(j)Ú`, `-(O)z`, `-(O)l`, `-(A)ll`, `-An`, `-(A)bb`, `-(O)nként`, `-ntA`, `-(O)stUl`, `-Ul` (verb), `-Ász`, `-Ár`, V-initial privative `-AtlAn`, deverbal `-Ás`, `-Ó`, `-At`, `-AtlAn`, `-Ódik` (verb stems: epenthetic drop and sz-d-v stem choice, `VRB-STEM-011`, `VRB-STEM-012`)
  - `bound(-ep)`: shortening and v-stem apply, the epenthetic vowel is kept: `-ít`, `-Odik` (`bátorít`, `bátorodik`, but `nehezít`, `nehezedik`, `bővít`); epenthetic drop before `-ít` only in listed bound roots (`piszkít`)
  - `bound(ep)`: relational `-i`; the epenthetic vowel drops only in the `-Alom`/`-Elem` nouns (`hatalmi`, `fegyelmi`, `birodalmi`) and otherwise stays (`ároki`, `bereki`, `nyeregi` [AkH12 §176], `toronyi` 2 : `tornyi` 0, `ürömi` 3 : `ürmi` 0 [WC2]), dropping only lexically (`egri`, `sarki`); v-stem and shortening lexical
  - `-(O)gAt`: free stem for epenthetic verbs, sz-stem for sz-d-v verbs, v-stem for v-stem verbs (`DER-VERB-013`)
- **Output**: the stem string to which the suffix (with its linking vowel) is concatenated
- **Examples**: `bokor` -> `bokros`, `bokrocska`, but `bokorka`, `#bokorság`; `ló` -> `lovas`, `lovacska`, `lovász`; `kő` -> `köves`, `kövez`; `kéz` -> `kezes`, `kezecske`, `(bal)kezű`, `kezel`; `levél` -> `leveles`, `levelecske`, but `levélke`; `víz` -> `vizes`, `(tiszta) vizű`, but `vízi`, `víztelen`; `nehéz` -> `nehezen`, `nehezít`, but `nehézség`; `bátor` -> `bátran`, `bátrabb`, but `bátorság`, `bátorít`, `bátorodik`; `titok` -> `titkár`, `titkol`; `sarok` -> `sarkall`, `sarki`, `sarkú`, but `saroknyi`; `fodor` -> `fodrász`, `fodroz`; `alkalom` -> `alkalmanként`; `hét` -> `hetenként`, `hetente`, `heti`; `irodalom` -> `irodalmi`; `Eger` -> `egri`; `söpör` -> `söprés`, `söprő`, `söpretlen`, but `söpörget`
- **Exceptions**:
  - relational `-i`: shortening, v-stem and epenthetic drop only lexically: `heti`, `havi`, `tavi`, `egri`, `sarki` vs `kézi`, `vízi`, `nyári`, `téli`, `ároki` (closed list, `DER-ADJ-002`)
  - `-(O)z` on epenthetic nouns varies lexically: `tükröz` vs `sátoroz`
  - `szó` -> `szótlan`, `szószerű` (free) vs `(egy)szavas` (bound)
  - place names with epenthetic shape drop only before some suffixes (`Eger` -> `egri` but `Egert`, `NOM-STEM-005`)
- **Predictability**: `rule` (per suffix); `lexical` for the `-i` exceptions and for epenthetic `-z` stems
- **Lexical input**: stem properties `epenthetic`, `v_stem`, `shortening` (`NOM-STEM-005`, `NOM-STEM-010`, `NOM-STEM-007`, `VRB-STEM-011`, `VRB-STEM-012`)
- **Interactions**: before `DER-MODEL-004` (linking vowel), `DER-MODEL-005`; `DER-MODEL-013`
- **Sources**: [SMNy3 pp. 693-694 (Rebrus 13.3.1, table 33 p. 694: triggering `-(O)nként`, `-(O)stUl`, `-(O)cskA`, `-(O)s`, `-(O)z`, `-(O)l`, `-(A)ll`, `-Ár`, `-Ász`, `-i`, `-Ú`, `-AtlAn`, `-An`, `-(A)bb`, `-Od`; non-triggering `-ít`, `-Odik`, `-ként`, `-ság`, `-nyi`, `-kor`, `-szor`)], [RST12 p. 502 ex. 11 (`piszk-ít`)], [SMNy3 pp. 716-718 (13.3.2.5 analyticity)], [SMNy3 p. 788 (`-i` in v-stems and shortening stems: `tavi`, `havi`, `nyári`, `heti`)], [SMNy3 p. 168 (`-Ász` on v-stems and epenthetic stems)], [emMorph stemalt.rul (study only)], [magyarispell fonev_kepzo.aff comments `<kézi> <*kezi>`, `<bokri>` (study only)]
- **Validation**: goldens, attestation (`vizű`, `kezű`, `lelkű`, `irodalmi`, `egri`, `alkalmanként`, `hetenként`), oracle:hunmorph-foma
- **Confidence**: consensus for the suffix classes; `tendency` for epenthetic nouns before `-(O)z`

### DER-MODEL-007 Truncating suffixes
- **Statement**: Truncating suffixes delete the stem-final vowel (or a listed Latinate ending) before attaching; harmony of the suffix and of all following suffixes is computed from the untruncated base, except for hypocoristic `-i`.
- **Conditions**:
  - vowel-final base (for `-ít`, `-Ul`, `-Odik`: adjective of two or more syllables; for `-Ász`: noun)
  - truncating suffixes: `-ít` (`DER-VERB-008`), verbal `-Ul` (`DER-VERB-009`), `-Odik` on polysyllabic vowel-final adjectives (`DER-VERB-009`), `-Ász` (`DER-NOUN-013`), `-ista` (`DER-NOUN-015`) and `-izmus` (`DER-NOUN-016`) usually (non-truncating `dadaista`, `dadaizmus`, `goetheista`, `hinduizmus` keep the short vowel, no LVL), `-itás` (removes `-is`, shortens the preceding long vowel, `DER-NOUN-017`), `-izál` (removes `-is`, `DER-VERB-006`), `-ál` on Latinate nouns (removes `-ció`, `-ia`, `DER-VERB-003`), hypocoristic `-i` (prosodic truncation, `DER-DIM-005`)
  - adverbial `-An` truncates only lexically: `hosszú` -> `hosszan`, `lassú` -> `lassan`, `ifjú` -> `ifjan`, `könnyű` -> `könnyen` (`DER-ADV-002`)
- **Output**: base minus final vowel (or ending) + suffix; harmony class value base except hypocoristic `-i`, which takes harmony from the truncated string
- **Examples**: `barna` -> `barnít`, `barnul`; `fekete` -> `feketedik` (not `*feketédik`); `szomorú` -> `szomorodik`; `tiszta` -> `tisztít` -> `tisztítok` (back from `tiszta`); `béke` -> `békít` -> `békítek`; `sima` -> `simít` -> `simítok`; `zene` -> `zenész`; `nyomda` -> `nyomdász`; `zongora` -> `zongorista`; `labilis` -> `labilitás`; `digitális` -> `digitalizál`; `Simon` -> `Simi` -> `Siminek` (front from `Sim-`)
- **Exceptions**: `karcsú` -> `karcsúsít`, `olcsó` -> `olcsósít` use `-sít` instead of truncating `-ít` (`DER-VERB-008`); `lila` -> `lilul` but `*lilít` (lexical)
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-MODEL-008`, `PH-HARM`; truncation precedes harmony evaluation of later suffixes
- **Sources**: [SMNy3 pp. 159-163 (`-Ul`, `-Vdik`, `-ít` truncation)], [SMNy3 p. 168 (`-Ász`)], [SMNy3 pp. 129, 163 (`-itás`, `-izál`)], [SMNy3 pp. 674, 677 (Rebrus: truncating `-Ász`, `-izál`, `-íroz`, `-ifikál`, `-ál`, `-ít`, `-Odik`, `-Ul`, `-ista`, `-izmus`, `-ikus`)], [T04 p. 145 (stem-final vowel-zero alternation; `feketedik` evidence)], [RST12 pp. 501-503 (`tiszt-ít-ok`, `bék-ít-ek` vs `Sim-i-nek`)], [morphdb derivation README (ÍT, UL lexical exceptions `lilul`)]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-MODEL-008 Harmony classes of derivational suffixes
- **Statement**: Every derivational suffix has one harmony class that determines its own vowel and the harmony value passed to following suffixes.
- **Conditions**:
  - `H2`: alternates for backness only, vowel chosen by `PH-HARM` from the preceding string: `-sÁg`, `-Ás`, `-Ó`, `-At`, `-tAlAn`, `-(j)Ú`, `-kA`, `-(O)cskA` (final vowel), `-lAg`, `-An`, `-Ul`, `-hAtÓ`, `-AndÓ`, `-Ász`, `-(O)gAt` (final vowel)
  - `H3`: alternates `o/e/ö` (linking vowels of type `O` and of `-(O)gAt`, `-(O)nc`, and suffixes `-kOdik`, `-Odik`); the type `M` linking vowels of `-(O)mÁny`, `-(O)vÁny` are two-way `o/e`
  - `H2r`: `o/ö` only, no front unrounded alternant: `-nok/-nök` (`elnök`, `védnök`), linking vowel of `-(O)dA` (`öntöde`)
  - `N` invariant, containing only `i`, `í`, `é`, transparent: following suffixes harmonise with the base as if the suffix were absent, with vacillation increasing with the number of neutral vowels (`PH-HARM-019`): `-i` (relational), `-ít`, `-ív` (loan), `-nyi`, `-né`, `-nként` (only `-i` follows), derivational `-ék` (`maradékot`, `keveréket`), `-ik` (ordinal)
  - `B!` invariant back, opaque (`PH-HARM-020`): following suffixes are back: `-kor`, `-ista`, `-izmus`, `-itás`, `-ál`, `-izál`, `-íroz`, `-ikus`, `-ális`, `-fajta`
  - `F!` invariant with `e`, `ö`, `ő`, `ü` or `ű`, opaque: following suffixes are front: `-beli`, `-szerű`, `-féle`, `-nemű`, `-mentes`, `-képp(en)` (no back variant; only `-i` can follow)
  - `base`: truncating suffixes (`DER-MODEL-007`)
- **Output**: suffix vowel(s) and the harmony state after the suffix
- **Examples**: `hamis` -> `hamisít` -> `hamisítok`; `Kuba` -> `kubai` -> `kubainak`; `karib` -> `karibi` -> `karibinak ~ karibinek`; `cselló` -> `csellista` -> `csellistának`; `definiál` -> `definiálok`; `falu` -> `falubeli` -> `falubeliek`; `kutya` -> `kutyaszerű` -> `kutyaszerűek`; `Szabó` -> `Szabóné` -> `Szabónénak ~ ?Szabónének`; `sör` -> `sörfajta` -> `sörfajták`; `modern` -> `modernizál` -> `modernizálok`
- **Exceptions**: none within the class; per-stem vacillation of the base is `PH-HARM` data
- **Predictability**: `rule`
- **Lexical input**: stem harmony class and vacillation parameter (`PH-HARM`)
- **Interactions**: `DER-MODEL-009`, `PH-HARM`, `DER-COMP-008`
- **Sources**: [RST12 pp. 495-496 (low `e` never transparent), pp. 500-503 (5.1 transparency of nonalternating neutral suffixes, list 8)], [KVF98 pp. 422-423 (invariant suffixes `-kor` back opaque, `-ista`)], [F13 theses §3.2.1 (`-né`)], [SMNy3 pp. 671-677 (`-nok/nök` no front unrounded variant, `-(O)dA` no `e` linking, `-képpen` no back variant)], [SMNy3 p. 674 (`-ékony` has no `*-éköny`)], [morphdb grammar AFF_FÉLE, AFF_MENTES, AFF_BELI, AFF_NEMŰ (OUT: front); AFF_NÉ also OUT front, i.e. morphdb does not model the `-né` vacillation]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus; vacillation after `N` suffixes is `tendency` (`PH-HARM`)

### DER-MODEL-009 The selected alternant of a harmonic suffix fixes harmony
- **Statement**: Once a harmonic (`H2`, `H3`) suffix has selected its front or back alternant, all following suffixes agree with that alternant, even when the base vacillates.
- **Conditions**:
  - base vacillates for harmony (back vowel followed by neutral vowels, `PH-HARM`)
  - suffix is alternating (`-sÁg`, `-(j)A` possessive, `-(O)z` with `-i`, `-(O)s`)
- **Output**: front alternant -> all later suffixes front; back alternant -> all later suffixes back; no further vacillation
- **Examples**: `fotel` -> `fotelja` -> `foteljának`; `fotelje` -> `foteljének` (`*foteljénak`); `?agilisság` -> `?agilisságnak` (`*agilisságnek`); `?agilisség` -> `?agilisségnek` (`*agilisségnak`), both marginal (7 and 3 tokens [WC2], `PH-HARM-018`); `haver` is back (`haverja` 10,264 : `?haverje` 15 [WC2]); the RST12 nonce verb `#martinizza` -> `#martinizzátok`, `#martinizi` -> `#martinizitek`
- **Exceptions**: invariant `N` suffixes do not fix harmony (`DER-MODEL-008`): `karibinak ~ karibinek`
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-MODEL-008`, `PH-HARM`
- **Sources**: [RST12 pp. 500-501 (examples 9, 10)]
- **Validation**: goldens
- **Confidence**: consensus

### DER-MODEL-010 Lowering value of derivational outputs
- **Statement**: Derived stems are non-lowering unless the derivational suffix forms adjectives; adjective-forming suffixes yield lowering stems, with listed exceptions.
- **Conditions**:
  - output category ADJ -> lowering: `-(O)s`, `-i`, `-(j)Ú`, `-nyi`, `-beli`, `-szerű`, `-nemű`, `-rétű`, `-mentes` (visible only in the accusative, `NOM-LOW-003`), `-ékony`, `-Ós`, participles `-Ó`, `-(O)tt`, `-AndÓ`, `-hAtÓ`, loan adjectives `-ikus`, `-ális`, `-ív`, `-ózus`
  - output category N or V -> non-lowering: `-sÁg`, `-Ás`, `-Ó` (noun), `-At`, `-mÁny`, `-vÁny`, `-Ász`, `-ista`, `-izmus`, `-itás`, `-nok`, `-ár`, `-(O)s` (noun), `-kA`, `-(O)cskA`, `-né`, `-(O)dA`
  - vowel-final lowering adjective: linking vowel before plural is optional or obligatory per `NOM-LOW` (obligatory after `-i`: `budaiak`; optional after `-Ó`: `bántók ~ bántóak`)
- **Output**: `out_lowering` value in the suffix record
- **Examples**: `só` -> `sós` -> `sósak`; `harc` -> `harcos` (noun) -> `harcosok`; `írott` -> `írottat`; `tanulékony` -> `tanulékonyak`; `vörös` (adj) -> `vörösek` vs `vörös` (noun 'communist') -> `vörösök`; `komikus` (adj) -> `komikusak` vs (noun) `komikusok`; `barátság` -> `barátságok`; `írás` -> `írásokat`
- **Exceptions**:
  - privative `-tAlAn/-AtlAn` forms adjectives but is non-lowering: `tanulatlanok`, `boldogtalanok`
  - `-alom/-elem` forms nouns but is lowering (and epenthetic): `hatalmak`, `jutalmat`
  - `-van/-ven` (tens numerals) is lowering: `hatvanat` (`PND-NUMINF-001`)
  - adjectives `nagy`, `agg`, `vak`, `gazdag`, `szabad`, `vastag`, ethnonyms and language names are non-lowering roots; derivation does not change a root's lexical value
- **Predictability**: `rule` with the three listed suffix exceptions
- **Lexical input**: base category of the output (conversion changes it, `DER-MODEL-015`)
- **Interactions**: `NOM-LOW`, `DER-MODEL-015`, `DER-ADV-002` (the adverb vowel follows the same value)
- **Sources**: [T04 pp. 160-161 (derivational suffixes lower only if adjective-forming; fn. 36 `-tlan`, `-van`; fn. 37 `nagy`, `agg`)], [T04 p. 145 fn. 9 (`-alom` lowering)], [T04 p. 156-157 (`vörös-ek`/`vörös-ök`, `komikus-ak`/`komikus-ok`)], [RST24 p. 165 (ethnonyms and language names never lower)], [emMorph stemalt.rul (lowering for `-i`, `-beli`, `-nyi`, `-ú`, `-ó`, `-ható`, `-andó`, `-szerű`, `-alom`; not `-talan`) (study only)]
- **Validation**: goldens, attestation (`vastagok`, `gazdagok`, `szabadok`), oracle:hunmorph-foma
- **Confidence**: consensus for the rule; `tendency` for new loan adjectives (RST24: vacillation)

### DER-MODEL-011 Third person possessive variant of derivational outputs
- **Statement**: The 3sg possessive variant (`-a/-e` vs `-ja/-je`) of a derived noun is determined by the final segment of the derivative per `NOM-POSS-005`..`NOM-POSS-007`, overridden by suffix-specific values listed here.
- **Conditions**:
  - vowel-final derivative -> `-ja/-je` (`-Ó`, `-kA`, `-(O)cskA`, `-(O)dA`, `-ista`, `-i`, `-né`)
  - sibilant- or palatal-final derivative -> `-a/-e` by `NOM-POSS-005` (`-Ás`, `-Ász`, `-(O)s` noun, `-izmus`, `-itás`, `-(O)nc`, `-mÁny`, `-vÁny`)
  - suffix override `-a/-e` regardless of general tendency (`NOM-POSS-007`): `-sÁg`, `-At`, `-zAt`, `-alom` (bound stem), `-ár/-ér`, `-nok/-nök`, derivational `-ék`
- **Output**: `out_poss3` value
- **Examples**: `barátság` -> `barátsága`; `írás` -> `írása`; `olvasó` -> `olvasója`; `irat` -> `irata`; `tudomány` -> `tudománya`; `látvány` -> `látványa`; `hatalom` -> `hatalma`; `óvoda` -> `óvodája`; `tanár` -> `tanára`; `elnök` -> `elnöke`; `maradék` -> `maradéka`; `zenész` -> `zenésze`; `asztalka` -> `asztalkája`
- **Exceptions**: `újság` 'newspaper' -> `újságja` (lexical, 1 item; `újsága` 'its novelty' regular)
- **Predictability**: `rule` with 1 lexical exception
- **Lexical input**: none
- **Interactions**: `NOM-POSS-005`..`NOM-POSS-007`
- **Sources**: [SMNy3 pp. 509-510 (Kiefer: `-sÁg` always `-a`, `újságja`)], [SMNy3 pp. 797-805 (Rebrus 13.6.3)], [morphdb grammar AFF_SÁG, AFF_MENTES (`poss_sg_A`)]
- **Validation**: goldens, attestation (`újságja`, `újsága`, `barátsága`), oracle:hunmorph-foma
- **Confidence**: consensus for `-sÁg`; `tendency` for the other overrides (no counts published)

### DER-MODEL-012 Productivity classes and generation policy
- **Statement**: Each suffix has a productivity class that controls whether the engine may generate unlisted derivatives and how analysis treats them.
- **Conditions**:
  - `P`: new words derivable for any base satisfying the input conditions, meaning compositional (`-sÁg` on adjectives, `-Ás`, `-(O)s`, `-i`, `-tAlAn`, `-(O)z`, `-(O)gAt`, `-kA/-(O)cskA` on concrete nouns, `-An`, `-hAtÓ`)
  - `P-dom`: productive only in a stated semantic or morphological domain (`-sÁg` on occupational nouns; `-Ul` adverb on privatives and language names; `-lAg` on `-i` adjectives of disciplines; `-(O)sít` on ethnonyms; `-ál` on polysyllabic back loan verb stems; `-(O)g/-An` on onomatopoeic roots)
  - `R`: unproductive, the base class is closed, rule describes existing words (`-Ul` verb on monosyllabic adjectives, `-mÁny/-mény`, `-At`)
  - `L`: unproductive, no rule; derivatives are lexicon entries with optional segmentation (`-Ász`, `-nok`, `-ár`, `-(O)nc`, `-zAt`, `-ong`, `-AtAg`)
- **Output**: generation: `P` free; `P-dom` only when base carries the domain feature; `R` and `L` only listed words. Analysis: all classes segmentable; `R`/`L` segmentation marked lexicalised
- **Examples**: `klónozás` (P, unlisted OK); `menedzserség` (P-dom, occupational noun); `*drappul` (R, closed class excludes loans); `#kecskész` (L, not generated)
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: semantic features consumed by `P-dom` suffixes (occupation, ethnonym, language, discipline, concrete, onomatopoeic, colour)
- **Interactions**: `DER-LEX-002` (blocking overrides `P`), `DER-LEX-006`
- **Sources**: [SMNy3 pp. 111-118 (3.4 productivity, regularity, frequency; 3.5)], [SMNy3 pp. 166-168 (4.5 unproductive regular derivation)], [KVF98 p. 351 (productivity per Papp 1969)]
- **Validation**: goldens, attestation
- **Confidence**: consensus for the scale; per-suffix class assignments `contested` where noted in the suffix rule

### DER-MODEL-013 Analytic, synthetic and truncating attachment
- **Statement**: The suffix property "analytic" used by this rulebook is the conjunction of `lvl = n` and `stem = free`; "synthetic" is `stem = bound`; `lvl = y` with `stem = free` is "semi-analytic"; truncating suffixes form a fourth class.
- **Conditions**:
  - analytic (Rebrus "superanalytic", `]`): no LVL, no epenthetic drop, no v-stem, no shortening: `-sÁg`, `-beli`, `-szerű`, `-féle`, `-fajta`, `-forma`, `-nemű`, `-rét(ű)`, `-mentes`, `-kor`, `-képp(en)`
  - semi-analytic: LVL but free stem: `-nyi`, `-né`, `-tlAn/-tAlAn` (non-lowering allomorphs); `-kA` has a free stem and never meets `a/e` (LVL not applicable)
  - synthetic: bound stem, LVL: `-(O)s`, `-(O)cskA`, `-(j)Ú`, `-(O)z`, `-(O)l`, `-(A)ll`, `-An`, `-(O)nként`, `-(O)stUl`, `-AtlAn`, all deverbal suffixes on verbs; bound stem without LVL: `-(O)ntA`
  - `bound(-ep)`: `-ít`, `-Odik` (shortening and v-stem, no epenthetic drop; `DER-MODEL-006`)
  - mixed: relational `-i` (no LVL, epenthetic drop, lexical shortening and v-stem)
  - truncating: `DER-MODEL-007`
  - verb-internal "quasi-analytic" suffix classes (`VRB`) are a separate classification of inflection
- **Output**: attachment class stored with the suffix; compound members behave as analytic (`DER-COMP-005`)
- **Examples**: `butaság`, `#bokorság` (analytic); `butácska`, `bokrocska` (synthetic); `bokorka`, `tonnányi`, `saroknyi` (semi-analytic); `irodalmi`, `anyai` (mixed); `bátorít`, `nehezít` (`bound(-ep)`)
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-MODEL-005`, `DER-MODEL-006`
- **Sources**: [SMNy3 pp. 716-718 (13.3.2.5)], [SMNy3 pp. 786-796 (13.6: superanalytic non-lengthening suffixes)], [morphdb grammar (analytic, synthetic_noun, lengthened features)]
- **Validation**: goldens
- **Confidence**: consensus for the facts; the label mapping is this rulebook's convention

### DER-MODEL-014 Suffix-initial consonant assimilation and spelling at the suffix boundary
- **Statement**: Consonant-initial derivational suffixes are spelled unchanged after any consonant; only `-vAl`-type assimilation (inflectional) and the three-identical-letter simplification apply, and semi-words follow compound spelling.
- **Conditions**:
  - suffix boundary (not compound boundary)
  - three identical consonant letters produced by a derivational suffix are simplified to two (`ORT-ASSIM-017`)
  - semi-words (`-szerű`, `-féle`, `-fajta`) behave as compound members: doubled digraphs are not simplified and three identical letters get a hyphen (`DER-COMP-018`, `DER-COMP-019`)
- **Output**: concatenation with the listed simplifications
- **Examples**: `egész` -> `egészség`; `kopasz` -> `kopaszság`; `okos` -> `okosság`; `gipsz` -> `gipszszerű`; `ész` -> `észszerű`; `expressz` -> `expressz-szerű`; `puff` -> `puff-féle`
- **Exceptions**: `ésszerű` is the pre-2015 (AkH11) spelling; accepted as legacy variant for analysis only (`REP`)
- **Predictability**: `rule`
- **Lexical input**: `semi_word` flag of the suffix
- **Interactions**: `ORT-ASSIM-016`, `ORT-ASSIM-017`, `ORT-HYPH`, `DER-COMP-018`
- **Sources**: [AkH12 §61, §92, §93, §132], [SMNy3 p. 56 (`észszerű` as compound structure)]
- **Validation**: goldens, attestation (`észszerű` 6, `ésszerű` 12 in the corpus list, legacy dominance)
- **Confidence**: normative

### DER-MODEL-015 Conversion (zero derivation)
- **Statement**: Some category changes happen without a suffix; the converted word inherits the form and receives the category-dependent properties of its new class.
- **Conditions**:
  - present participle `-Ó` -> adjective (`fárasztó feladat`) and -> agent, instrument or place noun (`olvasó`, `hűtő`, `ebédlő`), `DER-NOUN-008`
  - past participle `-(O)tt` -> adjective (`írott`, `ijedt`), `DER-ADJ-022`
  - `-(O)s` adjective -> occupational noun (`órás`), `DER-NOUN-014`
  - adjective -> noun by lexical conversion only (not productive): `vörös` 'communist', `komikus` 'comedian', `szárnyas` 'poultry'
  - ethnonym adjective <-> noun (`magyar`, `koreai`): consonant-final ethnonyms are non-lowering in both uses (`magyarok`); `-i` ethnonyms keep the obligatory low linking vowel of `-i` (`koreaiak`, `NOM-PL-003`)
- **Output**: same string, new category; lowering recomputed: noun conversions non-lowering, adjective conversions lowering (`DER-MODEL-010`), except stems whose linking vowel is fixed by the suffix (`-i`: `koreaiak` as noun and adjective)
- **Examples**: `vörös` ADJ -> `vörösek`; `vörös` N -> `vörösök`; `szárnyas` ADJ -> `szárnyasak`, N -> `szárnyasok`; `harcos` N -> `harcosok`
- **Exceptions**: adjective-to-noun conversion is not productive; converted nouns are lexicon entries (`DER-LEX-001`)
- **Predictability**: `rule` for participles and `-(O)s`; `lexical` for adjective -> noun
- **Lexical input**: lexicon lists converted nouns
- **Interactions**: `DER-MODEL-010`, `NOM-LOW`, `VRB-PART`
- **Sources**: [SMNy3 p. 102 (conversion)], [SMNy3 pp. 148-150 (`-(V)s` noun by conversion, rule 30)], [T04 pp. 156-157 fn. 27]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-MODEL-016 Master suffix table is normative data
- **Statement**: The master table below lists the record values (`DER-MODEL-002`) of every derivational suffix and semi-word of this chapter; the per-suffix rules give conditions, examples and exceptions.
- **Conditions**: table rows are keyed by the rule ID that owns the suffix
- **Output**: table values; where a cell says "see rule" the rule text governs
- **Examples**: row `DER-NOUN-001` gives `-sÁg`: ADJ/N -> N, `-`, `H2`, LVL `n`, stem free, lowering `n`, poss3 `-a`, `P`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: all suffix rules
- **Sources**: compiled from the sources of the individual rules
- **Validation**: goldens (one golden set per row)
- **Confidence**: per row, as in the owning rule

## Master table of derivational suffixes
Values per `DER-MODEL-002`. `Low` = lowering value of the output stem (`y`, `n`, `opt` optional linking vowel after vowel-final output, `-` not applicable). `Poss3` = 3sg possessive of the output noun (`-` for verbs, adverbs, adjectives rarely possessed). `Trunc` = truncation (`V` stem-final vowel, `is` Latinate ending, `pros` prosodic). Owned-elsewhere rows give the owning rule.

| Rule | Suffix | In -> Out | Allomorphs | Link | Harm | LVL | Stem | Trunc | Low | Poss3 | Prod |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `DER-NOUN-001` | `-sÁg` | ADJ -> N | `-ság/-ség`; lexical `-asság/-esség` | `-` | `H2` | n | free | - | n | `-a` | P |
| `DER-NOUN-003` | `-sÁg` | N (occupation, function) -> N | `-ság/-ség` | `-` | `H2` | n | free | - | n | `-a` | P-dom |
| `DER-NOUN-006` | `-Ás` | V -> N | `-ás/-és` | `V` | `H2` | - | bound (verb) | - | n | `-a` | P |
| `DER-NOUN-008` | `-Ó` | V -> N (agent, instrument, place) | `-ó/-ő` | `V` | `H2` | - | bound (verb) | - | n | `-ja` | P |
| `DER-NOUN-009` | `-At` | V -> N (result) | `-at/-et` | `V` | `H2` | - | bound (verb) | - | n | `-a` | R |
| `DER-NOUN-010` | `-(O)mÁny` | V -> N (result) | `-mány/-mény`, `-omány/-emény` | `M` (`o/e`) | `H2` | - | bound (verb) | - | n | `-a` | R |
| `DER-NOUN-011` | `-(O)vÁny` | V -> N | `-vány/-vény`, `-ovány/-evény` | `M` (`o/e`) | `H2` | - | bound (verb) | - | n | `-a` | L |
| `DER-NOUN-012` | `-(A)lOm`, `-(A)dAlOm` | V, ADJ -> N | `-alom/-elem`, `-adalom/-edelem`, `-dalom` (`fájdalom`) | `A` | `H2` (both vowels `a/e`, `o/e`; no `ö`) | - | bound | - | y (epenthetic output) | `-a` | L |
| `DER-NOUN-013` | `-Ász` | N -> N (occupation) | `-ász/-ész` | `V` | `base` | - | bound | V | n | `-a` | L |
| `DER-NOUN-013` | `-Ászat` | N in `-Ász` -> N (field, institution) | `-ászat/-észet` | `V` | `H2` | - | free | - | n | `-a` | P-dom |
| `DER-NOUN-014` | `-(O)s` | ADJ `-(O)s` -> N (conversion) | as `DER-ADJ-005` | `O` | `H3` | y | bound | - | n | `-a` | P-dom |
| `DER-NOUN-015` | `-ista` | N, bound Latinate -> N | `-ista` | `V` | `B!` | n (when not truncating) | free | V (usual; lexical non-truncation), `-ium` | n | `-ja` | P-dom |
| `DER-NOUN-016` | `-izmus` | N, ADJ, bound -> N | `-izmus` | `V` | `B!` | n (when not truncating) | free | V (usual; lexical non-truncation), `is` | n | `-a` | P-dom |
| `DER-NOUN-017` | `-itás` | foreign ADJ -> N | `-itás` | `V` | `B!` | - | free | `is` + length | n | `-a` | P-dom |
| `DER-NOUN-018` | `-nOk` | V, ADJ, N -> N | `-nok/-nök` | `-` | `H2r` | - | free | - | n | `-a` | L |
| `DER-NOUN-019` | `-Ár` | N, bound -> N | `-ár/-ér` | `V` | `H2` | - | bound (`titkár`) | V (lexical) | n | `-a` | L |
| `DER-NOUN-020` | `-(O)nc` | V, ADJ -> N | `-c`, `-onc/-enc/-önc` | `M` | `H3` | - | free | - | n | `-a` | L |
| `DER-NOUN-021` | `-zAt` | N, V -> N (collective) | `-zat/-zet`, `-azat/-ezet` | lex | `H2` | - | free | - | n | `-a` | L |
| `DER-NOUN-022` | `-(O)dA` | V, N -> N (place) | `-da/-de`, `-oda/-öde` | `M` (`o/ö`) | `H2`, link `H2r` | - | bound (verb) | - | n | `-ja` | L |
| `DER-NOUN-023` | `-ék`, `-(A)dék`, `-(A)lék` | V -> N | as listed | `A` | `N` (`é` transparent: `maradékot`); linking vowel `H2` | - | bound | - | n | `-a` | L |
| `DER-NOUN-024` | `-né` | N (person) -> N | `-né` | `-` | `N` | y | free | - | - | `-ja` | P |
| `DER-NOUN-025` | `-ÓkA` | V -> N (instrument) | `-óka/-őke` | `V` | `H2` | - | bound | - | - | `-ja` | L |
| `DER-NOUN-025` | `-(A)ttyÚ` | V -> N (instrument) | `-tyú/-tyű`, `-attyú/-ettyű` | `A` | `H2` | - | bound | - | - | `-ja` | L |
| `DER-NOUN-025` | `-(O)sdi` | N (person, role) -> N (children's game) | `-sdi`, `-osdi/-asdi/-esdi/-ösdi` | `O` | link `H3`, `-di` `N` | y | bound | - | - | `-ja` | P-dom |
| `DER-ADJ-001` | `-i` | N, ADV, POSTP -> ADJ | `-i`; zero after `i` | `-` | `N` | n | bound(ep) | - | y (obligatory link) | - | P |
| `DER-ADJ-005` | `-(O)s` | N, ADJ, NUM, PART -> ADJ | `-s`, `-os/-as/-es/-ös` | `O` | `H3` | y | bound | - | y | - | P |
| `DER-ADJ-008` | `-(j)Ú` | (MOD +) N -> ADJ | `-ú/-ű`, `-jú/-jű` | `-` | `H2` | y | bound, poss stem | - | y | - | P |
| `DER-ADJ-009` | `-tAlAn`, `-tlAn`, `-AtlAn` | N, ADJ -> ADJ | `-talan/-telen`, `-tlan/-tlen`, `-atlan/-etlen` | `-`/`A` | `H2` | y | free / bound | - | n | - | P |
| `DER-ADJ-010` | `-AtlAn` | V (transitive) -> ADJ | `-atlan/-etlen` | `A` | `H2` | - | bound (verb) | - | n | - | P |
| `DER-ADJ-012` | `-beli` | N, N+infl -> ADJ | `-beli` | `-` | `F!` | n | free | - | y | - | P-dom |
| `DER-ADJ-013` | `-nyi` | N (measure) -> ADJ/NUM | `-nyi` | `-` | `N` | y | free | - | y | - | P |
| `DER-ADJ-014` | `-szerű` | N, ADJ, PRON -> ADJ | `-szerű` | `-` | `F!` | n | free | - | y | - | P (semi-word) |
| `DER-ADJ-015` | `-féle`, `-fajta`, `-forma`, `-nemű`, `-rét`, `-rétű` | N, NUM, PRON -> ADJ | invariant | `-` | `F!`, `B!` (`-fajta`, `-forma`) | n | free | - | n / y (`-nemű`, `-rétű`) | - | P (semi-word) |
| `DER-ADJ-016` | `-mentes` | N -> ADJ | `-mentes` | `-` | `F!` | n | free | - | y | - | P (semi-word) |
| `DER-ADJ-017` | `-Ós` | V -> ADJ (via `-Ó` + `-s`) | `-ós/-ős` | `V` | `H2` | - | bound (verb) | - | y | - | P-dom |
| `DER-ADJ-018` | `-ékOny` | V -> ADJ | `-ékony/-ékeny` | `V` | `H2` | - | bound (verb) | - | y | - | L |
| `DER-ADJ-019` | `-hAtÓ` | V (transitive) -> ADJ | `-ható/-hető` | `-` | `H2` | - | analytic stem `SA` as for `-hAt` (`VRB-POT-001`) | - | opt | - | P |
| `DER-ADJ-020` | `-hAtAtlAn` | V -> ADJ | `-hatatlan/-hetetlen` | `-` | `H2` | - | analytic stem `SA` (`VRB-POT-001`) | - | n | - | P |
| `DER-ADJ-021` | `-AndÓ` | V -> ADJ | owned by `VRB-PART-003` | `V` | `H2` | - | bound | - | opt | - | P |
| `DER-ADJ-022` | `-Ó`, `-(O)tt` | PART -> ADJ (conversion) | owned by `VRB-PART-001`, `VRB-PART-002` | - | - | - | - | - | opt / y | - | P |
| `DER-ADJ-023` | `-ikus`, `-ális`, `-áris`, `-ív`, `-ózus`, `-áns`, `-ens` | bound Latinate -> ADJ | invariant | `V` | `B!` / `N` (`-ív`) / lexical ending choice (`-áns` back, `-ens` front; not a harmonic alternation) | - | free | - | y | - | L (paradigm correspondences R) |
| `DER-ADJ-024` | `-AtAg`, `-Ag` | V, bound -> ADJ/N | `-atag/-eteg`, `-ag/-eg` | `V` | `H2` | - | bound | - | y | - | L |
| `DER-VERB-001` | `-(O)z` | N, ADJ -> V | `-z`, `-oz/-az/-ez/-öz` (`-ik` optional) | `O` | `H3` | y | bound | - | - | - | P |
| `DER-VERB-001` | `-(O)l` | N, loan V -> V | `-l`, `-ol/-al/-el/-öl` | `O` | `H3` | y | bound | - | - | - | P-dom |
| `DER-VERB-003` | `-ál` | loan V stem, Latinate N -> V | `-ál` | `V` | `B!` | - | free | `ció`, `ia` | - | - | P-dom |
| `DER-VERB-006` | `-izál` | foreign ADJ, N -> V | `-izál` | `V` | `B!` | - | free | `is` + length | - | - | P-dom |
| `DER-VERB-007` | `-íroz`, `-ifikál` | bound loan -> V | invariant | `V` | `B!` | - | free | V (lexical) | - | - | L |
| `DER-VERB-008` | `-ít` | ADJ, bound -> V (causative) | `-ít` | `V` | `N`/`base` | - | bound(-ep): shortening and v-stem, epenthetic vowel kept (`bátorít`; `piszkít` lexical) | V | - | - | P-dom |
| `DER-VERB-008` | `-sít` | ADJ in `ó/ú/ű`, N -> V | `-sít` | `-` | `N` | y | free | - | - | - | P-dom |
| `DER-VERB-009` | `-Ul` | monosyllabic ADJ, bound -> V | `-ul/-ül` | `V` | `base` | - | bound | V | - | - | R |
| `DER-VERB-009` | `-Odik` | polysyllabic ADJ -> V | `-odik/-edik/-ödik` | `V` | `base`/`H3` | - | bound(-ep): shortening, epenthetic vowel kept (`bátorodik`) | V | - | - | P-dom |
| `DER-VERB-010` | `-(O)sít`, `-(O)sOdik` | ethnonym -> V | `-osít/-esít/-ösít`, `-osodik/-esedik/-ösödik` | `O` | `H3` | y | free | - | - | - | P-dom |
| `DER-VERB-011` | `-(A)ll` | ADJ (quantity), N -> V | `-ll`, `-all/-ell` | `A` | `H2` | y | bound (`sarkall`) | - | - | - | L |
| `DER-VERB-012` | `-kOdik`, `-(O)skOdik` | N (occupation), ADJ -> V | `-kodik/-kedik/-ködik`, `-skodik/-skedik/-sködik`, `-oskodik/-askodik/-eskedik/-ösködik` | `-` (`-kOdik`) / `O` (`-(O)skOdik`) | `H3` | y | free (`-kOdik`) / bound (`-(O)skOdik`) | - | - | - | P-dom |
| `DER-VERB-013` | `-(O)gAt` | V -> V | `-gat/-get`, `-ogat/-eget/-öget` | `M` | `H3`+`H2` | - | see rule | - | - | - | P |
| `DER-VERB-015` | `-Ódik` | V -> V (medio-passive) | `-ódik/-ődik` | `V` | `H2` | - | bound (verb) | - | - | - | P-dom |
| `DER-VERB-016` | `-(A)kOdik`, `-kOzik`, `-kÓdik` | V -> V (reflexive) | as listed | `A` | `H3` | - | see rule | - | - | - | L |
| `DER-VERB-017` | `-dogál` etc. | V -> V | see rule | - | `H2` | - | bound | - | - | - | L |
| `DER-VERB-018` | `-(O)g`, `-An` | onomatopoeic root -> V | `-og/-eg/-ög`, `-an/-en` | `M` / `V` | `H3` / `H2` | - | root | - | - | - | P-dom |
| `DER-VERB-019` | `-Ad`, `-Aszt`, `-int` | bound roots -> V | as listed | `V` | `H2` | - | root | - | - | - | L |
| `DER-VERB-022` | `-Ász(ik)` | N (animal, plant) -> V | `-ász(ik)/-ész(ik)` | `V` | `base` | - | bound | V | - | - | L |
| `DER-VERB-020` | `-tAt`, `-At`, `-tAtik` | V -> V | owned by `VRB-CAUS-001` | - | `H2` | - | - | - | - | - | P |
| `DER-ADV-001` | `-An` | ADJ -> ADV | owned by `NOM-ADJ-010` | `O` (always vowel after C) | `H3`/`H2` | y | bound | lexical | - | - | P |
| `DER-ADV-003` | `-Ul` | ADJ (privative, language) -> ADV | owned by `NOM-ADJ-011` | `V` | `H2` | y | free | - | - | - | P-dom |
| `DER-ADV-004` | `-lAg` | ADJ (`-i`, `-Ó`, `-Ú`), N -> ADV | `-lag/-leg` | `-` | `H2` | n | free | - | - | - | P-dom |
| `DER-ADV-006` | `-képp(en)` | N, PRON -> ADV | owned by `NOM-CASE-020` | `-` | `F!` | n | free | - | - | - | L |
| `DER-ADV-007` | `-kor` | N (time), NUM -> ADV | owned by `NOM-CASE-017` | `-` | `B!` | n | free | - | - | - | P-dom |
| `DER-ADV-008` | `-(O)nként` | N, NUM -> ADV | owned by `NOM-CASE-018` | `O` | `H3` link | y | bound | - | - | - | P |
| `DER-ADV-008` | `-(O)ntA` | N (time) -> ADV | owned by `NOM-CASE-021` | `O` | `H3`+`H2` | n | bound | - | - | - | L |
| `DER-ADV-008` | `-(O)stUl` | N -> ADV | owned by `NOM-CASE-019` | `O` | `H3`+`H2` | y | bound | - | - | - | P-dom |
| `DER-DIM-001` | `-kA` | N (C-final not stop, `i`-final) -> N | `-ka/-ke` | `-` | `H2` | - | free | - | n | `-ja` | P |
| `DER-DIM-001` | `-(O)cskA` | N -> N | `-cska/-cske`, `-ocska/-acska/-ecske/-öcske` | `O` | `H3`+`H2` | y | bound | - | n | `-ja` | P |
| `DER-DIM-005` | hypocoristic `-i` | personal name, N -> N | `-i` | `-` | truncated | - | truncated | pros | n | `-ja` | P-dom |
| `DER-DIM-006` | `-ci`, `-csi`, `-kó`, `-us`, `-u`, `-ó` | name, N -> N | as listed | `-` | as listed | - | truncated | pros | n | `-ja`/`-a` | L |
| `DER-PREF-004` | `leg-`, `legesleg-` | comparative ADJ -> ADJ | owned by `NOM-ADJ-006` | - | none | - | - | - | y | - | P |

## DER-NOUN

### DER-NOUN-001 -sÁg on adjectives: abstract property nouns
- **Statement**: `-ság/-ség` derives abstract nouns ('being ADJ, ADJ-ness') from adjectives, including derived and participial adjectives; it is fully productive subject to the restrictions in `DER-NOUN-004`.
- **Conditions**:
  - base: root adjective, derived adjective (`-(O)s` with abstract base, `-tlAn`, `-Ú`, `-i`), past participle used adjectivally (`-(O)tt`), present participle adjective (`hasonló`)
  - allomorph by harmony of the base: `-ság` after back harmony, `-ség` after front (`PH-HARM`)
  - attaches to the free stem, no linking vowel, no LVL (`DER-MODEL-013`)
- **Output**: noun `ADJ + ság/ség`
- **Examples**: `jó` -> `jóság`; `szép` -> `szépség`; `hosszú` -> `hosszúság`; `széles` -> `szélesség`; `beteg` -> `betegség`; `lusta` -> `lustaság`; `hülye` -> `hülyeség`; `nehéz` -> `nehézség`; `bátor` -> `bátorság`; `elszigetelt` -> `elszigeteltség`; `pénztelen` -> `pénztelenség`; `gyöngeelméjű` -> `gyöngeelméjűség`; `hasonló` -> `hasonlóság`; `ijedt` -> `ijedtség`; `figyelmes` -> `figyelmesség`
- **Exceptions**: see `DER-NOUN-002` (linking vowel items), `DER-NOUN-004` (restrictions, blocking)
- **Predictability**: `rule`
- **Lexical input**: base category ADJ; base harmony
- **Interactions**: `DER-NOUN-002`, `DER-NOUN-004`, `DER-NOUN-005`, `DER-MODEL-009` (front alternant opaque: `agilisség-nek`), `PH-HARM-018`
- **Sources**: [SMNy3 pp. 127-128 (4.1.1)], [KVF98 pp. 351, 356], [morphdb grammar AFF_SÁG (analytic non_lengthened)]
- **Validation**: goldens, attestation (`betegség` 19, `lustaság` 5, `elszigeteltség` 4, `figyelmesség` 4), oracle:hunmorph-foma
- **Confidence**: consensus

### DER-NOUN-002 -sÁg allomorphy and morphophonology
- **Statement**: `-sÁg` has no linking vowel and does not trigger LVL or stem alternations; the linked form is essentially a one-item exception. Over 779 `-s`-final adjective bases only `friss` takes the linked form categorically (`frissesség`), `nyers` and `gyors` vary, and every other base is bare [WC2]; the traditional "15 lexical items" is not reproducible from the corpus.
- **Conditions**:
  - after a vowel: stem-final `a/e` stays short
  - after a consonant: direct attachment, including after `s`, `sz`, `z`, `zs` (no assimilation in spelling)
  - base vacillating between back and front: `-ség` is preferred when the base ends in short unlengthened `e` (non-lengthening suffix leaves `e` less transparent)
- **Output**: `-ság/-ség`; lexical `-asság/-esség`
- **Examples**: `buta` -> `butaság` (`*butáság`); `anya` -> `anyaság`; `kerge` -> `kergeség`; `igaz` -> `igazság`; `okos` -> `okosság`; `egész` -> `egészség`; `dózse` -> `dózseség` (front preferred) vs `dózsénak ~ dózsének`
- **Exceptions**: lexical linking-vowel forms (SMNy count 15): with geminate `ss` (Rebrus lists all 5): `vigasság` (`víg`, shortened), `bölcsesség` (`bölcs`), `szüzesség` (`szűz`, shortened), `frissesség` (`friss`), `nyersesség` (`nyers`); geminate without linking vowel: `békesség` (`béke`); with linking vowel and single `s` (not listed by Rebrus, attested): `sokaság` (`sok`), `rosszaság` (`rossz`), deverbal `nyereség`, `veszteség`, `vereség`, `eleség`; the rest to be listed from a dictionary (see `## Open questions`); `tisztesség` is regular (`tisztes` + `-ség`); non-adjectival bases with `-sÁg` are lexical (`DER-NOUN-004`)
- **Predictability**: `rule` + `lexical` (1 categorical item plus 2 variable, on `-s`-final bases; the deverbal and geminate forms listed under Exceptions are separate lexical entries)
- **Lexical input**: stored `-esség` forms, from `sag-linking-vowel.tsv` (779 bases screened) in `local/lists/sag-linking-vowel.tsv` - working data, not yet read by the build
- **Interactions**: `DER-MODEL-005`, `PH-HARM-016` (lengthened `e` transparency), `ORT-ASSIM-017`
- **Sources**: [SMNy3 p. 677 (Rebrus 13.1.4.10: `-(A/Os)sÁg`, 15 linking-vowel forms, 5 with geminate, `békesség`; verbal bases `eleség`, `nyereség`, `veszteség`, `vereség`)], [AkH12 §29 (`kergeség`)], [RST12 p. 497 (`dózse-ség`)]
- **Validation**: goldens, attestation; 779 `-s`-final adjective bases screened with both diagnostic forms [WC2]. Also `butaság` 7, `anyaság` 7, `bölcsesség` 11, `bölcseség` 2 (misspelling), `frissesség` 5, `békesség` 10, `vigasság` 2, `szüzesség` 3, `sokaság` 8, `nyereség` 12
- **Confidence**: consensus; `measured` for the `-s`-final bases (`OQ-06-001` resolved: only `frissesség` is categorical, `nyers` and `gyors` vary, no further single-`s` base takes the linked form). The sources' "15 lexical items" could not be reproduced

### DER-NOUN-003 -sÁg on nouns
- **Statement**: On nouns `-sÁg` is productive only with occupational and functional person nouns ('being an N, N-hood'); collective and other denominal uses are lexicalised.
- **Conditions**:
  - productive: base is an occupational or functional person noun usable as a classifying predicate (`X menedzser`)
  - lexicalised: collective (`katonaság`, `hallgatóság`, `vezetőség`, `hegység`, `erdőség`), institution (`igazgatóság`, `ügynökség`), idiomatic (`állatság`, `marhaság`, `disznóság`), other (`háborúság`)
  - the productive reading is blocked where the form exists with a lexicalised sense; the substitute is `-(O)skOdik` + `-Ás` (`katonáskodás`, `ügynökösködés`)
- **Output**: noun
- **Examples**: `menedzser` -> `menedzserség`; `bróker` -> `brókerség`; `szupermen` -> `szupermenség`; `vezér` -> `vezérség`; `igazgató` -> `igazgatóság` (lexicalised 'directorate'); `katona` -> `katonaság` (lexicalised 'army'); `#katonaság` 'being a soldier' -> `katonáskodás`
- **Exceptions**: not productive with animal nouns (`??oroszlánság`, `??tigrisség`), concrete object nouns (`*székség`, `*könyvség`), abstract `-Ás` nouns; not productive as a collective: `alkalmazottság` 42 and `nézőség` 64 [WC2] are the productive 'being an N' reading ('employee status', `nézőségem`), never 'the staff', 'the audience'
- **Predictability**: `rule` (occupation domain), `lexical` (other denominal forms)
- **Lexical input**: noun feature `occupation`/`function`; lexicon entries for lexicalised `-sÁg` nouns
- **Interactions**: `DER-NOUN-001`, `DER-VERB-012`, `DER-LEX-002`
- **Sources**: [SMNy3 pp. 127-128 (rule 2)], [KVF98 p. 351 (`vezérség`, `igazgatóság`, `katonaság`, `hegység`)]
- **Validation**: goldens, attestation (`igazgatóság` 16, `katonaság` 10, `hegység` 12)
- **Confidence**: consensus

### DER-NOUN-004 Restrictions on -sÁg
- **Statement**: `-sÁg` does not apply productively to `-nyi` adjectives, to concrete-base `-(O)s` adjectives, to `-Ás` nouns, and where an existing noun blocks it.
- **Conditions**:
  - base in `-nyi`: excluded (`*literség` from `liternyi`: `*liternyiség`, `*kanálnyiság`)
  - base `-(O)s` adjective: allowed only if the `-(O)s` base noun is abstract (`figyelmesség`, `aggodalmasság`, `akaratosság`), excluded if concrete (`*sörösség`, `*szekrényesség`, `*hagymásság`; `aranyos` 'containing gold' has none, `aranyosság` being 'cuteness' only)
  - base `-Ás` noun: excluded
  - lexical blocking by an existing synonymous noun: `#szorgalmasság` (`szorgalom`), `#bánatosság` (`bánat`), `#egészségesség` (`egészség`)
  - Latinate adjectives in `-(iszt)ikus`, `-mán`, `-mer`, `-ózus` take `-sÁg`, not `-itás` (`utópisztikusság`, `specifikusság`, `polimerség`, `rigorózusság`)
- **Output**: rejection in generation; blocked forms are potential words and accepted in analysis with flag blocked (`DER-LEX-002`)
- **Examples**: `figyelmes` -> `figyelmesség`; `sörös` -> `*sörösség`; `statikus` -> `statikusság`; `morózus` -> `morózusság`
- **Exceptions**: `aranyosság` 'cuteness' exists (from `aranyos` 'cute', abstract sense)
- **Predictability**: `rule` (morphological restrictions); `lexical` (blocking)
- **Lexical input**: semantic feature `abstract` of the `-(O)s` base; blocking links
- **Interactions**: `DER-NOUN-017`, `DER-LEX-002`
- **Sources**: [SMNy3 pp. 114, 127, 129 (4.1.1, 4.1.3)], [SMNy3 p. 119 (3.7 blocking `szorgalmasság`, `egészségesség`, `bánatosság`)]
- **Validation**: goldens
- **Confidence**: consensus for `-nyi`, `-Ás`; `tendency` for blocking (`egészségesség` occurs in technical use)

### DER-NOUN-005 Output properties of -sÁg nouns
- **Statement**: `-sÁg` nouns are non-lowering, take `-a/-e` in the 3sg possessive and admit `-i`, `-Ú`, `-(O)s`, `-tAlAn` as further derivation.
- **Conditions**: any `-sÁg` noun
- **Output**: lowering `n`; poss3 `-a`; plural `-ok/-ek`; further derivation per suffix input conditions
- **Examples**: `barátság` -> `barátságok`, `barátsága`, `barátságos`, `barátságtalan`; `gazdaság` -> `gazdasági` -> `gazdaságilag`; `minőség` -> `minőségi`; `nagy kiterjedésű`; `szépségű`
- **Exceptions**: `újság` 'newspaper' -> `újságja` (`DER-MODEL-011`)
- **Predictability**: `rule` with 1 lexical exception
- **Lexical input**: none
- **Interactions**: `DER-MODEL-010`, `DER-MODEL-011`, `NOM-POSS-007`
- **Sources**: [SMNy3 pp. 509-510], [T04 pp. 160-161], [morphdb grammar AFF_SÁG (poss_sg_A, OUT aff_i aff_jú)]
- **Validation**: goldens, attestation (`barátsága` 6, `újságja` 2)
- **Confidence**: consensus

### DER-NOUN-006 -Ás: deverbal event and result nouns
- **Statement**: `-ás/-és` derives nouns from verbs; it is the most productive deverbal suffix, applying to any verb except `-hAt` forms and auxiliaries.
- **Conditions**:
  - base: any verb, including derived and prefixed verbs (`elolvas`, `dolgozgat`, `klónoz`)
  - allomorph by harmony: `-ás` back, `-és` front
  - stem: the verb's derivational stem: epenthetic stems in their vowelless form (`VRB-STEM-007`, `VRB-STEM-008`), sz-d-v verbs in the v-stem (`VRB-STEM-011`, `VRB-STEM-012`), v-stems with `v` (`VRB-STEM-010`), `-ik` removed
  - morphological restriction: not after `-hAt` (`*olvashatás`); lexicalised exceptions (`láthatás`) are entries
- **Output**: noun `V + ás/és`; process, simple event or result reading (argument inheritance: `SYN`)
- **Examples**: `olvas` -> `olvasás`; `beszélget` -> `beszélgetés`; `klónoz` -> `klónozás`; `internetezik` -> `internetezés`; `söpör` -> `söprés`; `ugrik` -> `ugrás`; `mozog` -> `mozgás`; `érez` -> `érzés`; `fürdik` -> `fürdés`; `becsül` -> `becsülés ~ becslés` (meaning split: 'esteem' vs 'estimate'); `tesz` -> `tevés`; `eszik` -> `evés`; `iszik` -> `ivás`; `alszik` -> `alvás`; `lő` -> `lövés`; `megold` -> `megoldás`
- **Exceptions**: blocked by existing nouns: `#hívés` (`hit`), `#szeretés` (`szeretet`), `#gyűlölés` (`gyűlölet`), `#félés` (`félelem`), `#tisztelés` (`tisztelet`) (`DER-LEX-002`); `van` has no `-Ás` noun (`lét`, `levés` lexical)
- **Predictability**: `rule`
- **Lexical input**: verb stem class (`VRB-STEM-014`); blocking links
- **Interactions**: `DER-NOUN-007`, `DER-LEX-002`, `DER-COMP-013` (deverbal compounds), `VRB-STEM`
- **Sources**: [SMNy3 pp. 106, 112 (productivity, `-hat` restriction)], [SMNy3 pp. 256-259 (6.1.1.1)], [KVF98 pp. 354-355], [morphdb grammar AFF_ÁS (synthetic_derivation)]
- **Validation**: goldens, attestation (`klónozás` 4, `internetezés` 4, `becslés` 10, `söprés` 0, `alvás` 13; `hívés` unattested), oracle:hunmorph-foma
- **Confidence**: consensus

### DER-NOUN-007 Output properties of -Ás nouns
- **Statement**: `-Ás` nouns are non-lowering, take `-a/-e` in the 3sg possessive and admit `-i`, `-(O)s`, `-Ú`, `-beli` as further derivation, but not `-sÁg`.
- **Conditions**: any `-Ás` noun
- **Output**: lowering `n` (`írások`, `írást` bare accusative after `s`); poss3 `-a`; derivation: `-i` (`mérési`), `-(O)s` (`gázfűtéses`), `-Ú` (`nagy kiterjedésű`), `-beli` (`látásbeli`)
- **Examples**: `írás` -> `írása`, `írások`, `írásos`, `írásbeli`; `mérés` -> `mérési`; `fűtés` -> `fűtési`; `pihenés` -> `pihenési`
- **Exceptions**: lexicalised meanings (`írás` 'script', `képzés` 'word formation' (linguistics), `világítás` 'lighting system') are separate senses of one entry (`DER-LEX-001`)
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-MODEL-010`, `DER-ADJ-001`, `DER-ADJ-012`, `NOM-ACC-003`
- **Sources**: [SMNy3 p. 107 (lexicalised `írás`, `képzés`, `világítás`)], [SMNy3 p. 136 (`mérés+i`)], [T04 p. 161], [morphdb grammar AFF_ÁS (OUT aff_i aff_s aff_jú)]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-NOUN-008 -Ó: agent, instrument and place nouns
- **Statement**: Nouns in `-ó/-ő` denoting the agent, instrument or place of an action are converted from present participles; the conversion is productive for agents and instruments but excludes verbs in `-hAt`, `-tAt`, `-tAtik`.
- **Conditions**:
  - base: verb; stem as for the present participle (`VRB-PART-001`): epenthetic stems vowelless, sz-d-v in v-stem
  - reading: agent (`olvasó`, `nyomozó`, `dolgozó`), instrument (`hűtő`, `evező`, `hamutartó`), place (`ebédlő`, `olvasó` 'reading room', `társalgó`), simple event (lexicalised: `kijáró`)
  - excluded bases: `-hAt`, `-(t)At`, `-(t)Atik` (`*megoldhatóN`, `*olvastatóN` as agent); lexicalised `öltöztető` is an entry
- **Output**: noun, non-lowering (`olvasók`, `olvasót`), poss3 `-ja/-je`
- **Examples**: `olvas` -> `olvasó`; `nyomoz` -> `nyomozó`; `hűt` -> `hűtő`; `ebédel` -> `ebédlő`; `vesz` -> `vevő`; `tesz` -> `tevő`; `hisz` -> `hívő`; `alszik` -> `alvó` (adj) ; `újságot ír` -> `újságíró`; `ás` -> `ásó`
- **Exceptions**: adjective vs noun lowering: participle adjective optional linking vowel (`bántóak ~ bántók`), noun none (`olvasók`; `*olvasóak` as the noun's plural, the string being the adjective's)
- **Predictability**: `rule` (form); reading and existence of noun sense `lexical`
- **Lexical input**: noun sense lexicon
- **Interactions**: `VRB-PART-001`, `DER-MODEL-015`, `DER-COMP-013`, `NOM-PL-003`
- **Sources**: [SMNy3 p. 106 (restriction on `-hat`, `-tat`, `-tatik`)], [SMNy3 pp. 312-336 (6.2 -Ó nouns)], [KVF98 p. 355]
- **Validation**: goldens, attestation (`olvasó` 15, `hűtő` 11, `ebédlő` 6, `vevő` 17, `hívő` 12, `öltöztető` 1)
- **Confidence**: consensus

### DER-NOUN-009 -At: result nouns
- **Statement**: `-at/-et` derives result nouns from transitive verbs; it is unproductive, and its derivatives are lexicon entries with transparent segmentation.
- **Conditions**:
  - base: transitive verb, consonant-final, not `t`-final (`t`-final verbs take `-mÁny`, `DER-NOUN-010`)
  - vowel `a/e` by harmony (type `V`)
  - transparent reading 'result of V-ing' only with result-object verbs; other derivatives are idiomatic
- **Output**: noun, non-lowering, poss3 `-a/-e`
- **Examples**: `ír` -> `irat` (with lexical shortening `í` -> `i`); `váj` -> `vájat`; `keres` -> `kereset`; `idéz` -> `idézet`; `fon` -> `fonat`; idiomatic: `néz` -> `nézet`, `olvas` -> `olvasat`, `mond` -> `mondat`; intransitive lexicalised: `indulat`, `kikelet`, `járat`
- **Exceptions**: `*épít+et`, `*alkot+at`, `*készít+et` (would coincide with causatives); `*talál+at` 'thing found' (but `találat` 'hit' lexicalised); `-at/-et` on `-Ász` nouns forms field nouns (`DER-NOUN-013`)
- **Predictability**: `lexical` (closed set; SMNy gives a rule for the closed class with `R` status)
- **Lexical input**: lexicon entries
- **Interactions**: `DER-NOUN-010`, `PH-LEN-010` (`irat` shortening), `VRB-CAUS-001` (homonymy avoidance)
- **Sources**: [SMNy3 pp. 167-168 (4.5.1)], [KVF98 p. 356]
- **Validation**: attestation (`irat` 12, `kereset` 10, `idézet` 12, `nézet` 11, `olvasat` 5, `fonat` 2)
- **Confidence**: consensus

### DER-NOUN-010 -(O)mÁny: result nouns
- **Statement**: `-mány/-mény` derives result nouns from verbs (89 derivatives, unproductive); a linking vowel `o/e` appears after most monosyllabic stems and never after polysyllabic ones, and it is never `ö`.
- **Conditions**:
  - monosyllabic short-vowel stem (23 items): linking vowel, except `festmény`
  - monosyllabic long-vowel stem (12 items): mostly linking vowel (`állomány`, `látomány`, `vélemény`, `tétemény`, `gyűjtemény`, `bérlemény`, `érzemény`, `őrlemény`); without: `élmény`, `zsákmány`, `vívmány`, `gyártmány`
  - polysyllabic stem incl. prefixed verbs (54 items): no linking vowel
  - linking vowel is `o` after back and `e` after any front stem (`gyűjtemény`, `őrlemény`, not `*gyűjtömény`)
  - `t`-final verbs use `-mÁny` where `-At` is excluded
  - analogical new forms are jocular (`írómány`, `keresmény`, `főzemény`)
- **Output**: noun, non-lowering, poss3 `-a`
- **Examples**: `tud` -> `tudomány`; `hagy` -> `hagyomány`; `vet` -> `vetemény`; `gyárt` -> `gyártmány`; `készít` -> `készítmény`; `alkot` -> `alkotmány`; `épít` -> `építmény`; `olvas` -> `olvasmány`; `talál` -> `találmány`; `bővít` -> `bővítmény`; `kiad` -> `kiadmány`; `ellát` -> `ellátmány`
- **Exceptions**: listed above; the complete list of 89 is a lexicon asset
- **Predictability**: `lexical` (89 items; linking vowel `rule` by syllable count with the listed exceptions)
- **Lexical input**: lexicon entries
- **Interactions**: `DER-NOUN-009`, `DER-MODEL-004` (type `M`)
- **Sources**: [SMNy3 p. 674 (Rebrus 13.1.4.4 counts {89}, {23}, {12}, {54})], [SMNy3 pp. 116-118, 167-168], [KVF98 p. 356]. Attested lemma counts with at least 100 corpus tokens are far above Rebrus' figures - `-mÁny` 402, `-vÁny` 223, `-(O)dA` 260, `-nOk` 159, `-(O)nc` 90, `-AtAg` 58, `-lAg` 86 [WC2] - because Rebrus is evidently counting non-compound, non-prefixed dictionary entries while ours include compounds (`keresztrejtvény`, `főmérnök`). The root/compound split is not computed for this list, so the comparison is not yet apples to apples. Inventories in `derivational-inventories.tsv` (1,185 rows), `local/lists/derivational-inventories.tsv` - working data, not yet read by the build. [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: attestation (`tudomány` 16, `vélemény` 15, `gyűjtemény` 13, `élmény` 15, `festmény` 10, `készítmény` 14, `kiadmány` 1)
- **Confidence**: consensus

### DER-NOUN-011 -(O)vÁny: deverbal nouns
- **Statement**: `-vány/-vény` forms deverbal nouns (78 items, unproductive); a linking vowel appears only in 4 of 35 monosyllabic stems and never after polysyllabic stems; linking vowel quality as `DER-NOUN-010`.
- **Conditions**: linking vowel items: `ingovány`, `szökevény`, `szövevény`, `jövevény`; all other monosyllabic and all polysyllabic stems attach directly
- **Output**: noun, non-lowering, poss3 `-a`
- **Examples**: `lát` -> `látvány`; `köt` -> `kötvény`; `nyomtat` -> `nyomtatvány`; `ültet` -> `ültetvény`; `szökik` -> `szökevény`; `jön` -> `jövevény`
- **Exceptions**: listed
- **Predictability**: `lexical` (78 items)
- **Lexical input**: lexicon entries
- **Interactions**: `DER-NOUN-010`
- **Sources**: [SMNy3 p. 674 (Rebrus: {78}, {35}, {4}, {43})]. Attested lemma counts with at least 100 corpus tokens are far above Rebrus' figures - `-mÁny` 402, `-vÁny` 223, `-(O)dA` 260, `-nOk` 159, `-(O)nc` 90, `-AtAg` 58, `-lAg` 86 [WC2] - because Rebrus is evidently counting non-compound, non-prefixed dictionary entries while ours include compounds (`keresztrejtvény`, `főmérnök`). The root/compound split is not computed for this list, so the comparison is not yet apples to apples. Inventories in `derivational-inventories.tsv` (1,185 rows), `local/lists/derivational-inventories.tsv` - working data, not yet read by the build. [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: attestation (`látvány` 13, `kötvény` 12, `nyomtatvány` 12, `ültetvény` 7, `szökevény` 4, `jövevény` 6)
- **Confidence**: consensus

### DER-NOUN-012 -(A)lOm and -(A)dAlOm
- **Statement**: The unproductive suffixes `-alom/-elem` and `-adalom/-edelem` form abstract nouns whose stems are epenthetic and lowering.
- **Conditions**:
  - linking vowel `a/e` (type `A`) with monosyllabic short-vowel bases (16 items; no polysyllabic bases); `d`-final bases take `-Al(O)m` rather than `-(A)dAl(O)m`; `fájdalom` without linking vowel
  - both suffix vowels alternate for backness only (`-alom/-elem`, never `ö`: `gyötrelem`)
  - the second vowel is unstable: it drops before vowel-initial suffixes (`NOM-STEM-005`)
  - output is lowering despite being noun-forming (`DER-MODEL-010`)
  - lexically irregular vowel: `birodalom`, `bizodalom`, `irodalom` vs `álladalom`, `forradalom`, `hiedelem`, `diadalom`
- **Output**: noun; bound stem `-alm-/-elm-`; lowering; poss3 `-a`
- **Examples**: `hatalom` -> `hatalmat`, `hatalma`, `hatalmas`; `jutalom` -> `jutalmaz`; `félelem` -> `félelmetes`; `szerelem` -> `szerelmes`, `szerelmi`; `irodalom` -> `irodalmi`, `irodalmár`; `forradalom` -> `forradalmár`; `bizalom` -> `bizalmatlan`; `fegyelem` -> `fegyelmetlen`; `nyugalom` -> `nyugalmas`
- **Exceptions**: listed irregular linking vowels
- **Predictability**: `lexical` (closed set)
- **Lexical input**: lexicon entries marked `epenthetic`, `lowering`
- **Interactions**: `NOM-STEM-005`, `NOM-LOW-006`, `DER-ADJ-009` (`-AtlAn` after these stems)
- **Sources**: [SMNy3 p. 674 (Rebrus 13.1.4.4 `-Al(O)m`, `-(A)dAl(O)m` {16}, `fájdalom`, irregular vowels)], [SMNy3 p. 693 (`ártalmak`, `élelmünk`, `bizalmas`, `forradalmár`, `szidalmaz`, `irodalmon`, `uradalmi`)], [T04 p. 145 fn. 9]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-NOUN-013 -Ász and -Ászat: occupation and field nouns
- **Statement**: `-ász/-ész` forms occupation nouns from concrete nouns; it truncates stem-final vowels and uses bound stems; it is unproductive (only a morphological pattern); `-ászat/-észet` on its outputs forms field and institution nouns.
- **Conditions**:
  - vowel-final base: final vowel deleted (`DER-MODEL-007`); harmony from the untruncated base
  - v-stem base: `v` alternant (`lovász`, `művész`); epenthetic base: vowelless alternant (`szobrász`, `lelkész`, `fodrász`)
  - some forms are conversions of `-Ász(ik)` verbs meaning 'hunt, collect' (`halász`, `vadász`, `madarász`, `lepkész`, `eprész`, `nyulász`, `egerész`, `rákász`, `solymász`, `agarász`, `DER-VERB-022`)
  - `-at/-et` added to an `-Ász` noun: field or institution (`kertészet`, `fogászat`, `szemészet`, `gégészet`, `vadászat`, `halászat`); productive for medical and technical fields
- **Output**: noun, non-lowering (`halászok`), poss3 `-a`
- **Examples**: `juh` -> `juhász`; `jog` -> `jogász`; `fül` -> `fülész`; `gép` -> `gépész`; `kert` -> `kertész`; `seb` -> `sebész`; `nyomda` -> `nyomdász`; `cipő` -> `cipész`; `kohó` -> `kohász`; `zene` -> `zenész`; `torna` -> `tornász`; `gazda` -> `gazdász`; `gomba` -> `gombász`; `gége` -> `gégész`; `szőlő` -> `szőlész`; `ló` -> `lovász`; `mű` -> `művész`; `szobor` -> `szobrász`; `lélek` -> `lelkész`; verbal bases: `szab` -> `szabász`, `szül` -> `szülész`
- **Exceptions**: not formed: `*kecskész`, `*orrász`; colloquial medical clippings `?szívész` 15 'cardiologist', `?tüdész` 4 'pulmonologist' [WC2]; blocked by other occupation nouns: `irodalmár` (`*irodalmász`), `csizmadia` (`*csizmász`)
- **Predictability**: `lexical` (derivatives listed; SMNy counts 14 truncating members in Rebrus' list)
- **Lexical input**: lexicon entries
- **Interactions**: `DER-MODEL-006`, `DER-MODEL-007`, `DER-NOUN-014` (rival `-(O)s` blocked: `#cipős`, `#kertes` occupation)
- **Sources**: [SMNy3 p. 168 (4.5.2)], [SMNy3 p. 677 (Rebrus 13.1.4.10 `ÁSZ` {14})], [SMNy3 p. 694 table 33 (`fodrász`)], [KVF98 pp. 351-353 (`kertészet`, `fogászat`)]
- **Validation**: attestation (`kertész` 12, `cipész` 5, `zenész` 12, `lovász` 8, `szobrász` 9, `lelkész` 12, `nyomdász` 6, `tornász` 5, `kohász` 3, `juhász` 15)
- **Confidence**: consensus

### DER-NOUN-014 -(O)s occupational and collective nouns
- **Statement**: Occupation nouns in `-(O)s` arise by conversion from `-(O)s` adjectives meaning 'having or working with N' or 'belonging to institution N'; they are productive in these domains, subject to blocking, while collective and numeral nouns in `-(O)s` are lexical.
- **Conditions**:
  - source adjective types: sportsperson by equipment (`bobos`, `gördeszkás`), seller by goods (`újságos`, `zöldséges`, `halas`), worker with object (`traktoros`, `kukás`, `szoftveres`, `órás`, `butikos`, `trafikos`), musician by instrument (`dobos`, `klarinétos`, `szaxofonos`, `citerás`), member of institution (`főiskolás`, `postás`, `tévés`, `APEH-es`)
  - allomorphy identical to the adjective (`DER-ADJ-005`)
  - blocked by: `-Ó` compounds or verbs (`#korcsolyás` -> `korcsolyázó`, `#síes` -> `síelő`, `#diszkoszos` -> `diszkoszvető`, `#rádiós` 'repairer' -> `rádiószerelő`); non-derived occupation nouns (`#kenyeres` -> `pék`, `#húsos` -> `hentes`); `-Ász` nouns (`#cipős` -> `cipész`, `#kertes` -> `kertész`, `#vadas` -> `vadász`); `-ista` nouns (`#zongorás` -> `zongorista`, `#gimnáziumos` -> `gimnazista`, `#egyetemes` -> `egyetemista`)
  - not productive: collective place nouns (`akácos`, `nádas`, `gyümölcsös`, `tölgyes`), numeral nouns beyond the closed numeral base (`ötös`, `tízes`, `PND`)
- **Output**: noun; non-lowering (`harcosok`, `asztalosok` vs adjective `sósak`); poss3 `-a`
- **Examples**: `óra` -> `órás`; `posta` -> `postás`; `fazék` -> `fazekas`; `bolt` -> `boltos`; `kártya` -> `kártyás`; `autó` -> `autós`; `asztal` -> `asztalos`; `zöldség` -> `zöldséges`
- **Exceptions**: listed blocking; `mangós`, `fügés`, `kivis` are adjectives only (no collective noun)
- **Predictability**: `rule` (domain) + `lexical` (blocking)
- **Lexical input**: base noun features `equipment`, `goods`, `instrument`, `institution`; blocking links
- **Interactions**: `DER-ADJ-005`, `DER-MODEL-015`, `DER-LEX-002`
- **Sources**: [SMNy3 pp. 148-150 (4.2.4.2, rules 30, 31)], [KVF98 p. 351 (850)], [T04 p. 161 (`harc-os-ok`)]
- **Validation**: attestation (`órás` 16, `postás` 9, `fazekas` 13, `asztalos` 10, `zöldséges` 10)
- **Confidence**: consensus

### DER-NOUN-015 -ista
- **Statement**: `-ista` forms person nouns (profession, adherent, student) from nouns and Latinate bound stems; it usually truncates stem-final vowels and replaces Latinate endings, never lengthens, and is invariant back.
- **Conditions**:
  - vowel-final base: final vowel usually deleted (`zongora` -> `zongorista`); lexically kept short, without LVL (`dada` -> `dadaista`, `Goethe` -> `goetheista`)
  - `-ium`, `-izmus` bases: ending replaced (`gimnázium` -> `gimnazista` with shortening, `humanizmus` -> `humanista`)
  - consonant-final base: direct (`futball` -> `futballista`, `egyetem` -> `egyetemista`)
  - harmony `B!`: following suffixes back (`csellistának`)
  - LVL never (non-lengthening when not truncating, `ORT-SUFX-002`)
- **Output**: noun, non-lowering; poss3 `-ja`; as adjective in `-ista` + `-i` not formed (use `-ista` attributively or `-izmus`-based `-i`)
- **Examples**: `zongora` -> `zongorista`; `cselló` -> `csellista`; `gimnázium` -> `gimnazista`; `egyetem` -> `egyetemista`; `futball` -> `futballista`; `buddha` -> `buddhista`; bound: `bolsevista`, `pesszimista`, `humanista`, `kommunista`
- **Exceptions**: many members are loans with bound stems (`pesszimista`); segmentation marked `lexicalised`
- **Predictability**: `rule` (form); `lexical` (existence)
- **Lexical input**: lexicon entries; `-ium` ending list
- **Interactions**: `DER-MODEL-007`, `PH-HARM-020`, `PH-LEN-011` (shortening before Latinate suffixes), `DER-NOUN-014` (blocking of `-(O)s`)
- **Sources**: [SMNy3 p. 677 (Rebrus 13.1.4.10 Latin suffixes: `-ista` "generally" truncates)], [SMNy3 p. 788 table 188 (`dadaista`, `goetheista` non-lengthening)], [AkH12 §28 (shortening `szlavista`)], [KVF98 pp. 354 (860), 422-423], [SMNy3 p. 149 (`gimnazista`, `egyetemista` block `-(V)s`)]
- **Validation**: attestation (`zongorista` 8, `gimnazista` 7, `egyetemista` 10, `futballista` 9)
- **Confidence**: consensus

### DER-NOUN-016 -izmus
- **Statement**: `-izmus` forms nouns of doctrines and tendencies from Latinate adjectives, nouns and bound stems; it truncates like `-ista` and is invariant back; `-ista`/`-izmus`/`-ikus` form a correspondence set on the same bound stem.
- **Conditions**:
  - base ending `-is`, `-ális`, `-ikus`: ending removed, penultimate long vowel shortened (`reális` -> `realizmus`, `szociális` -> `szocializmus`)
  - noun or name base: consonant-final direct (`Marx` -> `marxizmus`); vowel-final usually truncated, lexically kept short without LVL (`dadaizmus`, `goetheizmus`, `hinduizmus`)
  - harmony `B!`
- **Output**: noun, non-lowering (`izmusok`), poss3 `-a`
- **Examples**: `reális` -> `realizmus`; `ideális` -> `idealizmus`; `kommunista` <-> `kommunizmus`; `humanista` <-> `humanizmus`; `turista` <-> `turizmus`
- **Exceptions**: bound-stem members are lexical
- **Predictability**: `rule` (correspondences), `lexical` (existence)
- **Lexical input**: bound Latinate stem with correspondence flags
- **Interactions**: `DER-NOUN-015`, `DER-ADJ-023`, `PH-LEN-011`
- **Sources**: [SMNy3 p. 677], [SMNy3 p. 788 table 188 (`dadaizmus`, `goetheizmus`)], [KVF98 p. 423]
- **Validation**: attestation (`realizmus` 7, `kommunizmus` 11, `hinduizmus` 4)
- **Confidence**: consensus

### DER-NOUN-017 -itás
- **Statement**: `-itás` forms abstract nouns from foreign adjectives; it removes `-is`, shortens a preceding long vowel, and is invariant back; it is productive only on the foreign layer.
- **Conditions**:
  - base: foreign adjective (`mobil`, `modern`, `labilis`, `dekoratív`)
  - ending `-is` removed: `labilis` -> `labil-itás`; `virtuális` -> `virtual-itás`
  - long vowel before the suffix shortened: `á` -> `a`, `í` -> `i` (`dekoratív` -> `dekorativitás`, `kommunikatív` -> `kommunikativitás`)
  - excluded bases: `-(iszt)ikus`, `-mán`, `-mer`, `-ózus` (these take `-sÁg`, `DER-NOUN-004`); bases not derivable into compounds
- **Output**: noun, non-lowering, poss3 `-a`
- **Examples**: `mobil` -> `mobilitás`; `modern` -> `modernitás`; `labilis` -> `labilitás`; `digitális` -> `digitalitás`; `formális` -> `formalitás`; `legális` -> `legalitás`; `aktuális` -> `aktualitás`; `zseniális` -> `zsenialitás`
- **Exceptions**: directly borrowed nouns without Hungarian adjective (`identitás`, `kapacitás`, `atrocitás`) are entries; `univerzális` -> `univerzalitás` vs loan `universitas`
- **Predictability**: `rule` (foreign adjective class), `lexical` (base membership in the foreign layer)
- **Lexical input**: adjective feature `foreign_latinate`
- **Interactions**: `PH-LEN-011`, `DER-VERB-006` (same truncation), `DER-ADJ-023`
- **Sources**: [SMNy3 pp. 129 (4.1.3, rule 3)], [SMNy3 p. 125 (`-itás` among derivational suffixes)]
- **Validation**: attestation (`mobilitás` 12, `modernitás` 6, `labilitás` 2, `virtualitás` 7, `identitás` 12)
- **Confidence**: consensus

### DER-NOUN-018 -nOk/-nök
- **Statement**: `-nok/-nök` is an unproductive person-noun suffix (36 items) with only rounded alternants.
- **Conditions**: back base -> `-nok`; front base (also unrounded) -> `-nök`; no `-nek` alternant
- **Output**: noun, non-lowering (`elnökök`), poss3 `-a/-e` (`elnöke`)
- **Examples**: verbal bases: `akarnok`, `írnok`, `gyakornok`, `látnok`, `mérnök`, `védnök`, `vésnök`, `ülnök`; adjectival: `álnok`, `elnök`, `főnök`; nominal: `bajnok`, `dalnok`, `hírnök`
- **Exceptions**: none
- **Predictability**: `lexical` (36 items)
- **Lexical input**: lexicon entries
- **Interactions**: `DER-MODEL-008` (`H2r`)
- **Sources**: [SMNy3 p. 677 (Rebrus `-nok/nök` {36}, {8} verbal)]. Attested lemma counts with at least 100 corpus tokens are far above Rebrus' figures - `-mÁny` 402, `-vÁny` 223, `-(O)dA` 260, `-nOk` 159, `-(O)nc` 90, `-AtAg` 58, `-lAg` 86 [WC2] - because Rebrus is evidently counting non-compound, non-prefixed dictionary entries while ours include compounds (`keresztrejtvény`, `főmérnök`). The root/compound split is not computed for this list, so the comparison is not yet apples to apples. Inventories in `derivational-inventories.tsv` (1,185 rows), `local/lists/derivational-inventories.tsv` - working data, not yet read by the build. [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: attestation (`elnök` 20, `mérnök` 14, `bajnok` 15, `gyakornok` 9, `védnök` 3)
- **Confidence**: consensus

### DER-NOUN-019 -Ár/-ér
- **Statement**: `-ár/-ér` is an unproductive person-noun suffix on nouns and bound verbal stems, with occasional truncation.
- **Conditions**: back -> `-ár`, front -> `-ér`; vowel-final base truncated (`csata` -> `csatár`); epenthetic base vowelless (`titok` -> `titkár`)
- **Output**: noun, non-lowering, poss3 `-a/-e`
- **Examples**: denominal: `bankár`, `kulcsár`, `forradalmár`, `irodalmár`, `tanár`, `pincér`, `tüzér`, `külügyér`, `csatár`; bound verbal: `csillár`, `idomár`, `vezér`, `tündér`
- **Exceptions**: `tündér` shows consonant insertion (`tűnik`)
- **Predictability**: `lexical` (Rebrus: {14}, of which {8} denominal by the most likely reading of the count notation)
- **Lexical input**: lexicon entries
- **Interactions**: `DER-MODEL-007`
- **Sources**: [SMNy3 p. 674 (Rebrus `Ár` {14}, denominal {8}; truncation `csata` -> `csatár`, insertion `tündér`)], [SMNy3 p. 694 table 33 (`titkár` epenthetic drop)]
- **Validation**: attestation (`tanár` 18, `bankár` 8, `csatár` 11, `pincér` 9, `forradalmár` 7, `irodalmár` 4)
- **Confidence**: consensus

### DER-NOUN-020 -(O)nc
- **Statement**: `-c`/`-onc/-enc/-önc` is an unproductive person-noun suffix (about 15 items) on verbs and adjectives.
- **Conditions**: bases ending in vowel + `n` take `-c` only (`suhanc`, `különc`); others take the three-way linking vowel
- **Output**: noun, non-lowering, poss3 `-a/-e`
- **Examples**: `újonc`, `ifjonc`, `különc`, `küldönc`, `lelenc`, `védenc`, `tolonc`, `suhanc`, `kedvenc`, `bérenc`, `kegyenc`, `kéjenc`; analogical nonce `költönc`, `nyomoronc`
- **Exceptions**: none
- **Predictability**: `lexical` (about 15 items)
- **Lexical input**: lexicon entries
- **Interactions**: `DER-LEX-006` (analogical nonce words)
- **Sources**: [SMNy3 p. 677 (Rebrus `-(On)c` {15})], [SMNy3 p. 112 (`költönc`, `nyomoronc` analogical)]. Attested lemma counts with at least 100 corpus tokens are far above Rebrus' figures - `-mÁny` 402, `-vÁny` 223, `-(O)dA` 260, `-nOk` 159, `-(O)nc` 90, `-AtAg` 58, `-lAg` 86 [WC2] - because Rebrus is evidently counting non-compound, non-prefixed dictionary entries while ours include compounds (`keresztrejtvény`, `főmérnök`). The root/compound split is not computed for this list, so the comparison is not yet apples to apples. Inventories in `derivational-inventories.tsv` (1,185 rows), `local/lists/derivational-inventories.tsv` - working data, not yet read by the build. [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: attestation (`újonc` 11, `kedvenc` 17, `különc` 7, `küldönc` 1)
- **Confidence**: consensus

### DER-NOUN-021 -zAt
- **Statement**: `-zat/-zet` forms collective and structural nouns; it is unproductive and its linking vowel is lexical.
- **Conditions**: listed forms only; linking vowel `a/e` in some (`tollazat`, `gépezet`), none in others (`csillagzat`, `kőzet`); items with a coexisting verb (`fogaz` -> `fogazat`, `hangzik` -> `hangzat`) are ambiguous with verb + `-At` and are stored unsegmented
- **Output**: noun, non-lowering, poss3 `-a/-e`
- **Examples**: `toll` -> `tollazat`; `gép` -> `gépezet`; `csillag` -> `csillagzat`; `kő` -> `kőzet`
- **Exceptions**: forms like `szerkezet`, `öltözet`, `szervezet` are verb + `-et` (`szerkez-`, `öltöz-`, `szervez`), not `-zAt`
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `DER-NOUN-009`
- **Sources**: [SMNy3 p. 677 (Rebrus lists `-zAt`)], [HUWIKI-SZOKEPZES (`csillagzat`)]
- **Validation**: attestation (`tollazat` 2, `fogazat` 5, `gépezet` 8, `csillagzat` 1, `kőzet` 8)
- **Confidence**: consensus

### DER-NOUN-022 -(O)dA: place nouns
- **Statement**: `-da/-de` forms place nouns (about 30 items, unproductive, language-reform origin); the linking vowel is `o` or `ö`, never `e`.
- **Conditions**:
  - monosyllabic short-vowel verb stems (13): linking vowel (`mosoda`, `öntöde`, `sütöde`), except `csapda`, `fogda`, `nyomda`
  - monosyllabic long-vowel stems (6): mostly none (`csúszda`, `járda`, `zárda`, `zúzda`), linking vowel in `óvoda`, `szálloda`
  - polysyllabic stems (11): none (`étkezde`, `kifőzde`)
  - `ír` shortens as a simplex (`iroda`) but not in compounds (`távírda`; also `pénzverde`)
  - some nominal bases: `áruda`, `csillagda`, `jósda`, `bölcsőde`, `zenede`, `cukrászda`
  - front stems with linking vowel take `ö` (`öntöde`, `sütöde`); final `a/e` by harmony
- **Output**: noun (vowel-final), poss3 `-ja/-je`
- **Examples**: `óvoda`, `iroda`, `nyomda`, `járda`, `csapda`, `zárda`, `csúszda`, `szálloda`, `öntöde`, `sütöde`, `mosoda`, `bölcsőde`, `cukrászda`, `kifőzde`, `étkezde`, `jósda`
- **Exceptions**: listed
- **Predictability**: `lexical` (about 30 items; linking vowel `rule` by syllable count with exceptions)
- **Lexical input**: lexicon entries
- **Interactions**: `DER-COMP-009` (`távírda`, lexical compound behaviour)
- **Sources**: [SMNy3 p. 674 (Rebrus `-(O)dA` {30}, {13}, {6}, {11})]. Attested lemma counts with at least 100 corpus tokens are far above Rebrus' figures - `-mÁny` 402, `-vÁny` 223, `-(O)dA` 260, `-nOk` 159, `-(O)nc` 90, `-AtAg` 58, `-lAg` 86 [WC2] - because Rebrus is evidently counting non-compound, non-prefixed dictionary entries while ours include compounds (`keresztrejtvény`, `főmérnök`). The root/compound split is not computed for this list, so the comparison is not yet apples to apples. Inventories in `derivational-inventories.tsv` (1,185 rows), `local/lists/derivational-inventories.tsv` - working data, not yet read by the build. [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: attestation (`óvoda` 19, `iroda` 18, `nyomda` 11, `öntöde` 1, `sütöde` 0, `mosoda` 8, `bölcsőde` 13, `cukrászda` 8, `étkezde` 2, `jósda` 2)
- **Confidence**: consensus

### DER-NOUN-023 Derivational -ék, -(A)dék, -(A)lék
- **Statement**: The unproductive deverbal suffixes `-ék`, `-adék/-edék`, `-alék/-elék` form result and substance nouns; they are distinct from associative `-ék` (`NOM-ASSOC-001`).
- **Conditions**:
  - `-ék` invariant: `marad` -> `maradék`, `kever` -> `keverék`, `készül` -> `készülék`, `fedez` -> `fedezék`, `emlék`
  - `-adék/-edék`: `hull` -> `hulladék`, `tör` -> `töredék`, `ül` -> `üledék`
  - `-alék/-elék`: `tart` -> `tartalék`, `tölt` -> `töltelék`
  - distinguishing associative `-ék`: associative attaches to person nouns and names and may follow possessives (`Katáék`, `szomszédaimék`); derivational `-ék` attaches to verbs
- **Output**: noun, non-lowering, poss3 `-a/-e` (`maradéka`, `tartaléka`); `é` is transparent for following suffixes (`maradékot`, `készüléket`)
- **Examples**: listed
- **Exceptions**: none
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `NOM-ASSOC-001`
- **Sources**: [SMNy3 p. 674 (Rebrus `-ék (unproductive mark)`, `-(A)dék (unproductive mark)`, `-(A)lék (unproductive mark)`)], [SMNy3 p. 123 (associative `-ék` inflectional)]
- **Validation**: attestation (`maradék` 14, `keverék` 13, `készülék` 17, `hulladék` 16, `töredék` 7, `üledék` 6, `tartalék` 13, `töltelék` 9, `fedezék` 2, `emlék` 11)
- **Confidence**: consensus

### DER-NOUN-024 -né: 'wife of'
- **Statement**: `-né` forms the name or designation of a married woman from a person's surname, full name or occupational noun; it is fully productive, lengthens stem-final `a/e`, has no linking vowel and is a transparent neutral vowel for harmony (back stems take back suffixes in 99.1% of 9,569 case tokens over 13 lemmas: `Szabónénak`, `Istvánnéval`) [WC2].
- **Conditions**:
  - base: family name (`Kovácsné`), full name (`Kovács Jánosné`, `-né` on the given name), occupational or title noun (`orvosné` 'doctor's wife', `grófné`)
  - LVL: `Katona` -> `Katonáné`
  - names ending in a double consonant letter identical to `n`: hyphen (`Wittmann-né`, `ORT-HYPH-002`)
  - SMNy3 classifies `-né` as a compound member (semi-word) because it permits coordination ellipsis; this rulebook keeps its suffix record for allomorphy
- **Output**: noun (proper or common), poss3 `-ja`; harmony `N`, transparent: back stems keep back suffixes (`Szabónénak`; measured `-nak` 5,632 : `-nek` 53, `-val` 3,591 : `-vel` 25, `-ról` 261 : `-ről` 7 over 13 back-stem `-né` lemmas) (`PH-HARM-019`)
- **Examples**: `Kovács` -> `Kovácsné`; `Szabó` -> `Szabóné`; `Katona` -> `Katonáné`; `Kiss János` -> `Kiss Jánosné`; `orvos` -> `orvosné` (contrast `orvosnő` 'female doctor'); `gróf` -> `grófné` (contrast `grófnő`)
- **Exceptions**: `-nő` compounds are different lexemes (`DER-COMP`)
- **Predictability**: `rule`
- **Lexical input**: noun feature `person`
- **Interactions**: `DER-MODEL-005`, `PH-HARM-019`, `ORT-HYPH-002`, `ORT-NAME`
- **Sources**: [KVF98 p. 354 (859)], [SMNy3 p. 123 (ellipsis, compound member)], [AkH12 §93 (`Wittmann-né`)], [morphdb grammar AFF_NÉ (analytic lengthened)], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: attestation (`Kovácsné` 8, `katonáné` 2, `*katonané` unattested; all case forms of 13 back-stem `-né` lemmas, 9,569 tokens), goldens
- **Confidence**: `measured`. The chapter's transparency claim was an assumption and is now measured at 99.1% back (`OQ-06-003` resolved); there is no vacillation to speak of

### DER-NOUN-025 Minor noun-forming suffixes: -ÓkA, -(A)ttyÚ, -(O)sdi
- **Statement**: `-óka/-őke` (instrument diminutives), `-tyú/-tyű`, `-attyú/-ettyű` (instruments) are lexical; `-(O)sdi` forms 'playing at N' nouns - children's role games, and pejoratively 'going through the motions of N' - from person and role nouns, from `-Ó` deverbal participles, and from institution and abstract activity nouns; it is productive. Child-language `-ikA` is **not** generated.
- **Conditions**:
  - `-ÓkA`: `járóka`, `fúvóka`, `hajtóka`, `szívóka`, `bújócska` and `fogócska` (with `-cska`); lexical
  - `-(A)ttyÚ`: `fogantyú`, `sarkantyú`, `dugattyú`, `szivattyú`, `csengettyű`, `pörgettyű`, `billentyű`, `töpörtyű`; vowel always long (`ORT-LEN-013`)
  - `-(O)sdi`: base is a person or role noun (`katonásdi` 1336, `indiánosdi` 322, `rendőrösdi` 75, `orvososdi` 117, `doktorosdi` 80), an `-Ó` deverbal participle (`kiszorítósdi` 1096, `nyomozósdi` 367, `szembekötősdi` 478, `kitalálósdi` 195, `bújósdi` 84), or an institution or abstract activity noun (`egyenlősdi` 1360, `demokráciásdi` 84, `bizottságosdi` 343, `népszavazósdi` 105, `fesztiválosdi` 56); `-sdi` after vowels with LVL (`katona` -> `katonásdi`), plural-type linking vowel after consonants (`bolt` -> `boltosdi`, `indián` -> `indiánosdi`, `rendőr` -> `rendőrösdi`); `-di` is invariant, the linking vowel harmonises (`H3`); coordinated `papás-mamásdi`
- **Output**: nouns, poss3 `-ja/-je`
- **Examples**: listed
- **Exceptions**: **child-language `-ikA` is not generated**: only `nénike` [WC2 4124] and `bácsika` [WC2 3442] are frequent, and both are lexicalised with no live base carrying a diminutive contrast; `anyika` 126 and `apika` 78 follow, and nothing else clears noise (N = 4, of which only 2 are synchronically transparent). Place-name `-i` adjectives (`Nádasd` -> `nádasdi`, `Kövesd` -> `kövesdi` and about 20 similar) and the monomorphemic `ósdi`, `élősdi`, `kisdi` are not `-(O)sdi` derivatives and are excluded from the count.
- **Predictability**: `lexical` (`-ÓkA`, `-(A)ttyÚ`, `-ikA`); `rule` (`-(O)sdi` on person/role nouns, `-Ó` participles and institution/activity nouns)
- **Lexical input**: lexicon entries
- **Interactions**: `DER-DIM-001`, `DER-NOUN-014`
- **Sources**: [SMNy3 pp. 674, 677 (Rebrus `-(A)ttyÚ`, `-ÓkA`; `-(A/O)sdi` among noun -> noun suffixes)], [SMNy3 p. 130 (`anyóka`, `szívóka`, `hajtóka` lexicalised)], [AkH12 §34 (`-tyú`, `-attyú`)], [WC2], [ATT evidence/productivity-tagset.md 2026-09-16]
- **Validation**: attestation. Webcorpus 2: **63 genuine `-(O)sdi` derivational types at freq >= 50, 18,477 tokens** (`egyenlősdi` 1360, `katonásdi` 1336, `kiszorítósdi` 1096, `háborúsdi` 666, `demokráciásdi` 84), place-name `-i` adjectives excluded. The earlier note "`-sdi` forms unattested in the corpus list" was an artefact of the 250k-type list and is withdrawn. Also `járóka` 3, `fúvóka` 5, `bújócska` 1, `fogantyú` 6, `szivattyú` 11, `billentyű` 8
- **Confidence**: `consensus` for the lists and for `-(O)sdi` productivity (`OQ-06-008` resolved). The Tolerance test fails against the **old** base class (`e` = 39 vs `theta_63` = 15), which is a failure of the stated restriction to person and role nouns, not of the suffix: many of the 63 types are transparently novel (`demokráciásdi`, `népszavazósdi`, `fesztiválosdi`, `párkapcsolatosdi`) and the meaning has extended from 'children's role game' to the pejorative 'going through the motions of X'. Restated over the wider base class the rule passes

### DER-NOUN-026 Unconfirmed -mA
- **Statement**: A noun-forming `-ma/-me` has been named in word-formation inventories. The candidate set is two items with no shared live base, so no rule is induceable and it is not modelled.
- **Conditions**: candidate items `szakma`, `kelme`
- **Output**: items stored as monomorphemic entries
- **Examples**: `szakma` -> `szakmai`, `szakmabeli`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `DER-LEX-005`
- **Sources**: none confirming; [WC2]; [ATT evidence/productivity-tagset.md 2026-09-16]
- **Validation**: attestation (Webcorpus 2 lemma frequencies: `szakma` 312,290, `kelme` 2,793 - both robustly attested **as words**, which is not the question). The Tolerance Principle is **inapplicable at N = 2**: any threshold computed over a two-member set is meaningless, and `szak-` and `kel-` are not independently available bases with the right meaning. A corpus search over 36.9M types returned no further `-ma`/`-me` deverbal noun analysable as base + `-mA` with a live base; the frequent `-ma`/`-me` noun lemmas are loans (`téma`, `dilemma`, `program`), possessed forms, or monomorphemic
- **Confidence**: `consensus` - the **modelling decision** is settled (listed, not a synchronic suffix, `OQ-06-002` retired); the etymology is not settled and need not be

## DER-ADJ

### DER-ADJ-001 Relational -i: bases and meanings
- **Statement**: `-i` derives relational adjectives expressing a relation REL(N1, N2) between the base and the modified noun; it is productive for defined noun classes and for time and place adverbs and postpositions.
- **Conditions**:
  - productive base classes (disjunctive): occupation, function or position nouns (`professzori`, `orvosi`, `elnöki`, `tanári`); personal names (family name or full name, `-i` on the last word: `kossuthi`, `Arany János-i`); institution nouns (`bírósági`, `minisztériumi`, `egyetemi`); place names (`magyarországi`, `balatoni`, `alföldi`, `budapesti`); abstract `-Ás` nouns (`mérési`, `látási`, `fűtési`); arts and disciplines (`építészeti`, `teológiai`, `irodalmi`, `zenei`, `etikai`); time nouns (`órai`, `napi`, `havi`, `évi`, `délelőtti`, `tavaszi`)
  - adverbs and postpositions (closed classes, not productive but general): `mai`, `tegnapi`, `ottani`, `benti`, `közeli`, `utáni`, `mögötti`, `általi`; not on directional (lative) postpositions (`*mögéi`)
  - after case-like derivational suffixes: `-kor` (`hajnalkori`), `-nként` (`fejenkénti`), `-képpen` (`tulajdonképpeni`), `-szOr` (`nyolcszori`)
  - after a postpositional phrase written apart (`föld alatti`, `ORT-TOG-018`)
- **Output**: adjective
- **Examples**: listed
- **Exceptions**: excluded classes (`DER-ADJ-002`)
- **Predictability**: `rule` (classes), `tendency` inside classes (time nouns: colloquial `?perci` 203 beside `perces` [WC2])
- **Lexical input**: noun semantic class (`occupation`, `institution`, `place`, `discipline`, `time`, `person_name`); `-Ás` morphology
- **Interactions**: `DER-ADJ-002`, `DER-ADJ-003`, `DER-ADJ-012` (rival `-beli`), `DER-ADJ-005` (rival `-(O)s`), `ORT-CAP` (lowercase derivatives of names)
- **Sources**: [SMNy3 pp. 135-138 (4.2.2)], [KVF98 pp. 364, 369 (893), (909)], [SMNy3 p. 675 (Rebrus: `-i` after adverbs, postpositions, `-képpen`, `-nként`, `-szOr`)]
- **Validation**: attestation (`orvosi` 18, `bírósági` 16, `magyarországi` 20, `építészeti` 15, `zenei` 17, `napi` 20, `tulajdonképpeni` 9, `fejenkénti` 1, `utáni` 18, `mai` 20), oracle:hunmorph-foma
- **Confidence**: consensus

### DER-ADJ-002 Relational -i: exclusions and blocking
- **Statement**: On a simplex common noun `-i` is blocked where an N + N compound or a rival suffix expresses the relation, and avoided where it would be homonymous with the possessed plural; an `i`-final common noun takes none. As the last member of a compound the same nouns take `-i` freely, and the generic, anatomical and geographic readings are regular.
- **Conditions**:
  - simplex material, product, intellectual-product, collective and abstract nouns: the 'made of N, of N' reading is blocked by the N + N compound of `DER-COMP-011` (`aranygyűrű`, `fakanál`, `könyvborító`): `*bronzi`, `*szekrényi`, `*ceruzai`, `*festményi`, `*csoporti`, `*tömegi`, `*munkai`, `*tulipáni`, and in that reading `*aranyi` 'golden', `*bori` 'of wine', `*fai` 'wooden', `*békei` 'of peace' (as strings these are the adjectives of the names `Aranyi`, `Bor`, `Béke`, or accent-stripped `fái`, `békéi`)
  - the same noun as the last member of a compound: `-i` regular (`közéleti` 144,988, `magánéleti` 25,712, `tankönyvi` 9,876, `telekkönyvi` 4,439, `munkacsoporti` 566 [WC2]); the simplex forms occur at low frequency in the same sense (`?könyvi` 1,480, `?életi` 1,874 [WC2])
  - body parts: regular in the anatomical-location reading (`hasi` 68,785, `nyaki` 36,805, `háti` 13,289, `feji` 2,296, `füli` 1,218, `orri` 117 [WC2]); blocked where a compound names the relation (`fejfájás`)
  - animals and plants: the generic reading is regular (`kutyai hűség`, `állati üvöltés`, `növényi szár`); geographic readings of material nouns are regular (`homoki bor`, `vízi közlekedés`)
  - `i`-final common noun: excluded (`*rabbii`, `*zsűrii`); `i`-final place names: no second `i` written (`Tamási` -> `tamási`, `ORT-SUFX-004`)
  - acronyms and clippings: `-(O)s` or `-beli` preferred (`MSZP-s`, `PM-beli`, `koleszos` 1,204 [WC2]); `-i` occurs in the press (`?MSZP-i`, `?PM-i`, `?koleszi`; MTA accepts `MSZP-i`), an institution acronym being an institution noun of `DER-ADJ-001`
  - vowel-final bases whose possessed plural ends in `-i` without a linking vowel: `-i` avoided by homonymy (`DER-LEX-003`), for `-ció` nouns (`privatizációs`; `*privatizációi` as the adjective) and some native `-Ó` nouns (`folyói` reads as the possessed plural), while `bírói`, `költői` are formed; `autói` and `háborúi` are possessed plurals, not `-i` adjectives
- **Output**: no `-i` derivative in generation where a block applies; rival suffix per condition
- **Examples**: `privatizáció` -> `privatizációs`; `adaptáció` -> `adaptációs`; `MSZP` -> `MSZP-s`; `utca` -> `utcai` ~ `utcabeli` (different meaning); `hegy` -> `hegyi (tó)` vs `hegyes (vidék)`
- **Exceptions**: in speech `-i` forms of blocked bases occur as input to `-lAg` (`újságilag`, `DER-ADV-004`)
- **Predictability**: `rule` (`i`-final common nouns) + `tendency` (compound blocking, homonymy avoidance)
- **Lexical input**: compound lexicon (for compound blocking); simplex vs compound base; acronym flag; the semantic class only separates the blocked reading from the generic, anatomical and geographic ones
- **Interactions**: `DER-ADJ-001`, `DER-ADJ-012`, `DER-COMP-011`, `DER-LEX-003`, `ORT-SUFX-004`
- **Sources**: [SMNy3 pp. 136-138 (restrictions 1-7, a, b, homonymy avoidance), p. 120 (3.7 example 2)], [WC2]
- **Validation**: goldens, attestation (counts above; `privatizációs` 7, `növényi` 16, `állati` 14, `homoki` 7)
- **Confidence**: consensus for the `i`-final and homonymy conditions; SMNy3's class list states as exclusions what the corpus shows to be blocks on simplex bases, and the simplex/compound split is a hypothesis consistent with [WC2]

### DER-ADJ-003 Relational -i: morphophonology and output
- **Statement**: `-i` does not trigger LVL, selects the vowelless alternant of epenthetic stems, uses shortened and v-stem alternants only lexically, is a transparent neutral vowel for harmony, and yields a lowering adjective with an obligatory linking vowel before the plural.
- **Conditions**:
  - LVL: none (`anyai`, `budai`, `Ráckeve` -> `ráckevei`)
  - epenthetic stems: vowel dropped (`irodalom` -> `irodalmi`, `történelem` -> `történelmi`, `társadalom` -> `társadalmi`, `lélek` -> `lelki`, `Eger` -> `egri`)
  - shortening stems: lexical: shortened `heti` (`hét`) and `levi` (`levél`); unshortened `kézi`, `vízi`, `úti`, `tűzi` (AkH12 §27 b), `nyári`, `égi`, `téli` - all six confirmed unshortened [WC2]
  - v-stems: lexical: `tavi` (`tó`), `havi` (`hó` 'month'), `művi` (`mű`), `kövi` (`kő`; `kövi` 7,326 : `kői` 632 [WC2]); these are mainly compound first members or attributive-only. `lovi` and `teli` are homograph-contaminated and are not counted
  - stems ending in `-falva`, `-háza`, `-halma`, `-telke`, `-völgye`, `-földe`: final `a/e` dropped (`Hollóháza` -> `hollóházi`) with optional full form for little-known names (`ORT-NAME`)
  - harmony `N`: next suffix follows the base (`kubainak`), vacillation increases after back + neutral bases (`karibinak ~ karibinek`)
  - output lowering with obligatory linking vowel: `városiak`, `budaiak` (`*budaik`); `-i` adjectives behave as lowering (opening) stems before all suffixes (`udvarias`, `udvariatlan`)
  - further derivation: `-sÁg` (`gazdaságiság` rare, `emberiség` lexical), `-(O)s` (`városias`), `-AtlAn` (`udvariatlan`), `-lAg` (`zeneileg`), `-An` (restricted, `DER-ADV-001`)
- **Output**: adjective in `-i`
- **Examples**: listed
- **Exceptions**: `nyári` vs `heti` unpredictable (closed list of **6**: `heti`, `havi`, `tavi`, `művi`, `kövi`, `levi`; other shortening and v-stems unshortened, confirmed for `kézi vízi úti tűzi nyári égi`); `-i` on personal names follows `ORT-NAME` (`shakespeare-i`)
- **Predictability**: `rule` + `lexical` (6 shortened / v items)
- **Lexical input**: stem classes; list of lexically shortened `-i` forms, from `shortened-i-adjectives.tsv` (17 probed: 6 shortened, 5 both, 6 full) in `local/lists/shortened-i-adjectives.tsv` - working data, not yet read by the build
- **Interactions**: `DER-MODEL-005`, `DER-MODEL-006`, `DER-MODEL-010`, `NOM-PL-003`, `PH-HARM-019`, `ORT-NAME`
- **Sources**: [SMNy3 p. 788 (`tavi`, `havi`, `művi`, `nyári`, `heti`)], [SMNy3 p. 133 (`udvari+as`, `udvari+ak`: `-i` creates opening stems)], [SMNy3 p. 145 (`párizsi+s` vs `párizsi+as`)], [AkH12 §27 b) (`kézi`, `vízi`, `úti`, `tűzi`), §29, §173, §174 c)], [RST12 p. 500 (9)], [magyarispell fonev_kepzo.aff comments (study only)]
- **Validation**: attestation (`kövi` 7,326 : `kői` 632 [WC2]; `irodalmi` 16, `lelki` 17, `egri` 14, `heti` 18, `havi` 18, `tavi` 9, `kézi` 15, `vízi` 14, `nyári` 18, `téli` 17), goldens
- **Confidence**: consensus; the shortened list is `measured` (`OQ-06-007` resolved: 6 members, `kövi` and `levi` joining the four)

### DER-ADJ-004 -i on multiword and hyphenated names
- **Statement**: With multiword or hyphenated bases `-i` attaches to the last word; capitalisation and hyphenation of the derivative are orthographic and owned by `ORT`.
- **Conditions**: base is a multiword proper name, a hyphenated geographic name, or a phrase written apart
- **Output**: morphological attachment to the last word; spelling per `ORT-CAP`, `ORT-NAME`, `ORT-HYPH-017`
- **Examples**: `Arany János` -> `Arany János-i`; `Új-Zéland` -> `új-zélandi`; `föld alatt` -> `föld alatti`; `New York` -> `New York-i`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: multiword name segmentation
- **Interactions**: `ORT-CAP`, `ORT-NAME`, `ORT-HYPH-017`, `ORT-TOG-036`
- **Sources**: [AkH12 §162, §167, §176-178, §217 b)], [SMNy3 p. 136 (morphologically on the given name)]
- **Validation**: goldens
- **Confidence**: normative

### DER-ADJ-005 -(O)s adjectives: allomorphy
- **Statement**: The adjective suffix `-(O)s` is `-s` after vowels (with LVL) and a plural-type linking vowel + `s` after consonants; it selects the bound stem.
- **Conditions**:
  - vowel-final base: `-s`, stem-final `a/e` lengthened
  - consonant-final non-lowering base: `-os/-es/-ös` by `H3`
  - consonant-final lowering base (lowering nouns, all adjectives except `DER-MODEL-010` exceptions, `-i` adjectives): `-as/-es`
  - `i`-final noun base (not an `-i` adjective): `-s` (`hokis`, `párizsi` N 'sausage' -> `párizsis`)
  - `-i` adjective base: `-as/-es` (`párizsi` ADJ -> `párizsias`, `városi` -> `városias`)
  - bound stem: epenthetic drop (`bokros`, `álmos`), v-stem (`lovas`, `köves`, `havas`, `füves`, `csöves`), shortening (`vizes`, `tüzes`, `leveles`, `kenyeres`, `szekeres`, `madaras`, `kezes`)
  - acronym and digit bases: suffix after hyphen by pronunciation (`ORT-ABBR`, `ORT-NUM`)
- **Output**: adjective; `-s`, `-os`, `-as`, `-es`, `-ös`
- **Examples**: `fa` -> `fás`; `gördeszka` -> `gördeszkás`; `hoki` -> `hokis`; `ernyő` -> `ernyős`; `föld` -> `földes`; `cukor` -> `cukros`; `dob` -> `dobos`; `gomb` -> `gombos`; `ház` -> `házas`; `fül` -> `füles`; `tulipán` -> `tulipános`; `tigris` -> `tigrises`; `szerencse` -> `szerencsés`; `hatalom` -> `hatalmas`; `függöny` -> `függönyös`; `ezüst` -> `ezüstös`; `kék` -> `kékes`; `sárga` -> `sárgás`; `piros` -> `pirosas`; `vörös` -> `vöröses`; `szegedi` -> `szegedies`; `Fidesz` -> `fideszes`; `kft.` -> `kft.-s` (`ORT-ABBR`, AkH12 §282)
- **Exceptions**: `drapp` -> `drappos` (colour loan, non-lowering pattern); `nagy`, `vak`, `gazdag`, `szabad` and consonant-final ethnonyms take mid vowels after back and front rounded stems (`angolos`, `magyaros`, `oroszos`, `görögös`)
- **Predictability**: `rule`
- **Lexical input**: stem `lowering`, stem alternants
- **Interactions**: `DER-MODEL-004`, `DER-MODEL-006`, `NOM-LOW`, `NOM-LINK-002`
- **Sources**: [SMNy3 pp. 142-146 (4.2.4)], [SMNy3 p. 675 (Rebrus `-(A/O)s`: `házias`, `vöröses`, `pirosas`, `kekis`)], [KVF98 pp. 362-363, 367 (887), (903)], [morphdb grammar AFF_S]
- **Validation**: attestation (`városias` 4, `vidékies` 3, `magyaros` 9, `kékes` 9, `pirosas` 4, `hokis` 3, `fideszes` 13, `kockás` 9), oracle:hunmorph-foma
- **Confidence**: consensus

### DER-ADJ-006 -(O)s adjectives: bases and meanings
- **Statement**: `-(O)s` is productive with nouns denoting a characteristic part, accessory or feature of the modified entity, with institution nouns, measure and time nouns, `-ció` nouns, and with colour adjectives, ethnonyms and some `-i` adjectives.
- **Conditions**:
  - noun bases: 'having, equipped with N' (object, tool, animal, plant: `dobos`, `sapkás`, `kutyás`, `bokros`, `fás`, `füles kosár`, `gombos`, `tulipános takaró`); 'belonging to institution N' (`iskolás`, `főiskolás`, acronyms `MSZP-s`); 'characterised by N' (part or feature not inalienable); 'resembling person N' (`kossuthos`, `pistás`, `tanáros`); 'with N as constant measure' (`réteges`, `kockás`, `kilós`, `literes`); 'of duration N' (`órás`, `napos`, `hetes`, `perces`); 'related to N' for `-(ác)ió` nouns (`privatizációs`)
  - adjective bases: colour adjectives 'ADJ-ish' (`kékes`, `sárgás`, `szürkés`, `pirosas`; new colours `bézses`, `kekis`, `drappos`), ethnonyms and language names 'in the style of' (`franciás`, `angolos`, `szerbes`); `-i` adjectives of place (`párizsias`, `vidékies`) and of countries not used as ethnonyms (`magyarországias`, `angliaias`)
  - excluded: `-i` adjectives from personal names (`*kossuthias` 2), except where the `-i` adjective is written as the name, an `i`-final one (`petőfies` 33, `jókaias` 22, `széchenyies` 17 [WC2]), from deverbal `-Ás` nouns (`*pihenésies`: `-Ás` + `-i` + `-(O)s` is under 200 tokens in all [WC2], while abstract bases take it: `ünnepies` 1,165, `irodalmias` 780), from time-unit nouns (`*napias` 11; seasons are not excluded: `nyárias` 12,714, `tavaszias` 10,429, `őszies` 5,747 [WC2]); the `-itás` quality noun of a listed adjective, which that adjective blocks (`*stabilitásos` 6 against `stabilitás` 187,552 and `stabil`; `kreativitásos` 12, `aktivitásos` 5); an `-itás` noun with no such adjective takes the suffix (`annuitásos` 3,733, `paritásos` 2,184, `prioritásos` 1,711, `kvalitásos` 1,557 [WC2])
  - inalienable parts use `-(j)Ú` (`DER-ADJ-008`); `-(O)s` with a modifier forms a secondary compound (`kétágyas`, `DER-COMP-014`)
  - participle bases give `-Ós` adjectives (`DER-ADJ-017`)
- **Output**: adjective; compound colour names `kékeszöld`, `zöldessárga` (`ORT-TOG-025`)
- **Examples**: listed
- **Exceptions**: `-i` and `-(O)s` contrast in meaning: `tavi barlang` 'cave in a lake' vs `tavas barlang` 'cave with a lake'; `hegyes vidék` vs `hegyi tó`; `háromnapos csecsemő` vs `harmadik napi pihenés`; `X napos ~ X napi munka` free variation only with a numeral and an event noun
- **Predictability**: `rule` (classes), `tendency` (items)
- **Lexical input**: semantic classes of base nouns and adjectives
- **Interactions**: `DER-ADJ-001`, `DER-ADJ-008`, `DER-NOUN-014`, `DER-COMP-014`
- **Sources**: [SMNy3 pp. 142-148 (4.2.4.1, rules 27, 28, table 29)], [KVF98 pp. 362-363 (887)-(889), 367 (903), 370 (913)]
- **Validation**: attestation (`kutyás`, `iskolás` 12, `kilós` 9, `literes` 13, `napos` 16, `perces` 15, `privatizációs` 7, `tavas` 1, `hegyes` 10, `hegyi` 14)
- **Confidence**: consensus

### DER-ADJ-007 -(O)s adjectives: output properties
- **Statement**: `-(O)s` adjectives are lowering, take `-a/-e` in the 3sg possessive when used as nouns, and feed `-sÁg`, `-An`, `-Odik`, `-ít`, `-kOdik`, `-(O)z`, `-Ul` (via privative) and comparative.
- **Conditions**: any `-(O)s` adjective
- **Output**: lowering `y` (`sósak`, `sósat`); bare accusative not used (`sósat`, not `*sóst`; the string is the accusative of the name `Sós`)
- **Examples**: `sós` -> `sósabb`, `sósan`, `sósság`; `vizes` -> `vizesedik`, `vizesít`; `ködös` -> `ködösödik`, `ködösít`; `okos` -> `okoskodik`; `hatalmas` -> `hatalmasan`
- **Exceptions**: converted nouns are non-lowering (`DER-NOUN-014`)
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-MODEL-010`, `DER-VERB-008`, `DER-VERB-009`, `DER-VERB-012`
- **Sources**: [T04 pp. 157, 160-161], [SMNy3 pp. 160-162 (`-(V)s` + `-Vdik`, `-ít`)], [morphdb grammar AFF_S (OUT aff_l aff_z aff_ság aff_ít aff_an aff_ul aff_bb)]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-ADJ-008 -(j)Ú: inalienable possession adjectives
- **Statement**: `-ú/-ű` (after consonants) and `-jú/-jű` (after vowels) form adjectives 'having an N (of a stated kind)' from inalienable parts and dimensions; the base normally needs a modifier, the vowel is always long, LVL applies and the base appears in its possessive stem.
- **Conditions**:
  - consonant-final base: `-ú/-ű` without `j`, regardless of the base's 3sg possessive (`kertje` but `nagy kertű`)
  - vowel-final base: `-jú/-jű` on the possessive stem: LVL (`szoknyájú`, `szempillájú`, `elméjű`), possessive-only vowel change (`tető` -> `tetejű`, `idő` -> `idejű`, `erő` -> `erejű`, `ajtó` -> `ajtajú`, `NOM-STEM-013`)
  - bound stem: shortening (`kéz` -> `kezű`, `víz` -> `vizű`, `tűz` -> `tüzű`, `levél` -> `levelű`), epenthetic drop (`lélek` -> `lelkű`, `bajusz` -> `bajszú`)
  - syntax: base is `[MOD N]` where MOD is an adjective, numeral or phrase (`hosszú lábú`, `három fejű` written `háromfejű`, `nagyon rövid hajú`, `Annáénál rövidebb hajú`); bare `N+Ú` only if lexicalised (`bőkezű`, `jószívű` as compounds, `lábú` as bound)
  - semantic: inalienable part, dimension or property (`láb`, `fej`, `haj`, `ár`, `méret`, `kiterjedés`, `hang`, `szag`); alienable nouns allowed only with a phrasal modifier
  - harmony `H2` `ú/ű`
- **Output**: adjective, lowering (`hosszú lábúak`); orthography of `MOD + N-Ú` per `ORT-TOG-023`
- **Examples**: `hosszú lábú`, `három fejű` -> `háromfejű`; `rövid hajú`, `egyenáramú`, `fehér falú`, `mérsékelt árú`, `négylábú`, `bőkezű`, `jószívű`, `nagy kiterjedésű`, `tiszta vizű`, `erős rugójú`, `hosszú szempillájú`, `éles elméjű`, `lapos tetejű`, `rövid idejű`, `nagy erejű`, `vaskezű`
- **Exceptions**: none for form; semantics `tendency`
- **Predictability**: `rule`
- **Lexical input**: possessive stem alternants; noun feature `inalienable`
- **Interactions**: `NOM-STEM-013`, `DER-MODEL-005`, `DER-MODEL-006`, `DER-COMP-014`, `ORT-LEN-013`, `ORT-TOG-023`
- **Sources**: [AkH12 §34, §117], [SMNy3 pp. 104-105, 115 (bracketing), pp. 546-558 (11.3)], [KVF98 pp. 363-364 (890)-(892)], [morphdb grammar AFF_JÚ (oa_stem, C_final -ú)]
- **Validation**: attestation (`vizű` 7, `kezű` 8, `lelkű` 7, `tetejű` 5, `idejű` 13, `erejű` 12, `elméjű` 3, `bajszú` 0, `ajtajú` 0), goldens
- **Confidence**: normative (vowel length); consensus (allomorphy)

### DER-ADJ-009 Privative -tAlAn on nouns and adjectives
- **Statement**: The privative ('without N') has the allomorphs `-tlan/-tlen` after vowels, `-talan/-telen` after consonants and `-atlan/-etlen` after lowering and epenthetic consonant-final stems; it is productive with nouns denoting separable components or accompanying properties.
- **Conditions**:
  - vowel-final base: `-tlan/-tlen`, LVL (`ruhátlan`, `sótlan`, `felhőtlen`, `erőtlen`, `szótlan`)
  - consonant-final non-lowering base: `-talan/-telen`, free stem, no linking vowel (`családtalan`, `könyvtelen`, `szívtelen`, `víztelen`)
  - consonant-final lowering base: `-atlan/-etlen` on the bound stem (`fogatlan`, `lábatlan`, `tollatlan`, `fületlen`, `nyeletlen`, `szarvatlan`)
  - epenthetic bases: `-atlan/-etlen` on the vowelless stem (`álmatlan`, `lelketlen`, `szemérmetlen`; all `-alom/-elem` nouns: `bizalmatlan`, `tartalmatlan`, `fegyelmetlen`, `figyelmetlen`, `kényelmetlen`)
  - non-lowering bases with lexical `-atlan` (4): `gondatlan`, `pártatlan`, `pontatlan`, `álmatlan`
  - doublets with meaning split: `gondatlan` 'careless' / `gondtalan` 'carefree'; `tárgyatlan` 'intransitive' / `tárgytalan` 'pointless'; `lelketlen` 'heartless' / `lélektelen` 'soulless'; `végetlen` (literary) / `végtelen`
  - `-i` adjective bases: `-atlan/-etlen` (opening stem): `udvariatlan`, `lovagiatlan`, `művészietlen`, `nemzetietlen`, `nőietlen`
  - `-szerű` base: `-tlen` (`szabályszerűtlen`, `rádiószerűtlen`)
  - semantic condition: part-whole relation of type 'whole and typical component' or 'thing and accompanying property' (`nyeletlen kapa`, `fületlen csésze`, `sótlan étel`, `stílustalan írás`); not with collectives or quantities (`*fátlan erdő` in the sense 'forest without trees' is odd); inalienable parts only figuratively (`szívtelen ember`, `lelketlen férfi`)
  - root adjective bases: not productive; lexicalised: `bátortalan`, `boldogtalan`, `gyöngédtelen`, `tisztátlan`, `szaporátlan`, `magyartalan`, `igaztalan`, `komolytalan`, `hűtlen`
- **Output**: adjective, non-lowering (`boldogtalanok`); adverb in `-Ul` (`DER-ADV-003`); negation of the corresponding `-(O)s` adjective when not lexicalised (`családos` / `családtalan`; lexicalised non-equivalent: `gyanútlan` != `nem gyanús`, `tanácstalan` != `nem tanácsos`)
- **Examples**: listed
- **Exceptions**: listed lexical `-atlan` forms and doublets (about 36 lexicalised `-AtlAn` forms on nouns, 8 on epenthetic stems)
- **Predictability**: `rule` (allomorph by stem class) + `lexical` (4 non-lowering `-atlan`, doublets)
- **Lexical input**: stem `lowering`, `epenthetic`; doublet list
- **Interactions**: `DER-MODEL-004`, `DER-MODEL-006`, `DER-ADJ-011`, `DER-ADV-003`, `NOM-LOW-006`
- **Sources**: [SMNy3 pp. 132-135 (4.2.1, forms 7, 9, 10, 13-17)], [SMNy3 p. 675 (Rebrus `-(A)t(A)lAn`: {36} lexicalised, {8} epenthetic, {4} non-opening, doublets)], [KVF98 pp. 365, 367 (895), (904)], [morphdb grammar AFF_TALAN]
- **Validation**: attestation (`sótlan` 4, `felhőtlen` 9, `szívtelen` 4, `lábatlan` 5, `fogatlan` 3, `álmatlan` 6, `lelketlen` 5, `gondatlan` 8, `gondtalan` 8, `pártatlan` 7, `pontatlan` 9, `udvariatlan` 2, `ruhátlan` 2), oracle:hunmorph-foma
- **Confidence**: consensus

### DER-ADJ-010 Privative -tAlAn on verbs, written -AtlAn
- **Statement**: On verbs the privative is always `-atlan/-etlen` and is productive on transitive verbs whose object exists apart from the event, giving the passive reading 'not V-ed'; intransitive and stative bases occur only in lexicalised words.
- **Conditions**:
  - base: a transitive verb whose object exists independently of the event (an affected or located theme), prefixed or not (`megold`, `kitakarít`, `borotvál`, `olvas`, `megír`, `kipróbál`, `élez`, `fizet`, `kifizet`); the adjective says the event has not happened to that object. Telicity is not required: `váratlan` 212,108, `olvasatlan` 10,233 [WC2]
  - excluded: verbs whose object comes into being through the event (effected object), since no object exists of which the event could be denied (`*alkotatlan`, `*alapítatlan`, both 0 [WC2]); `ír` and `épít` take the suffix where the object exists beforehand (`íratlan` 18,804, `megíratlan` 874, `építetlen` 43 'unbuilt (plot)' [WC2]); verbs whose object is itself an event (`*abbahagyatlan`)
  - transfer and manner-of-action verbs are not excluded (`fizetetlen` 2,259, `kifizetetlen` 6,020 [WC2]); their unattested members are potential words (`#ajándékozatlan`, `#adományozatlan`, `#agyonveretlen`, `#agyoncsapatlan`; cf. `veretlen` 19,753 [WC2])
  - stem: vowelless epenthetic stem, v-stem (`VRB-PART-004`: `söpretlen`, `fövetlen`); after `-ít` the suffix is `-atlan/-etlen` spelled with one `t` (`tanítatlan`, `hamisítatlan`, `ORT-LEN-018`)
  - morphologically the suffix attaches to the verb stem, not to the participle (`*megtanultatlan`)
  - excluded: intransitive (`*ugrálatlan`, `*repületlen`), stative (`*kedveletlen`)
  - lexicalised: `átlátszatlan`, `független`, `nyugtalan`, `szüntelen`, `látatlan`, `hallatlan`, `ingatlan`, `untalan`, `védtelen`, `nincstelen`; on stative bases `ismeretlen` 542,366, `tudatlan` 23,520 [WC2], `szeretetlen` (ÉrtSz: 'unloving' from `szeretet`, archaic 'unloved'), `értetlen` (active 'uncomprehending')
- **Output**: adjective, non-lowering; semantic opposite of the past participle (`megtanult` / `megtanulatlan`, `várt` / `váratlan`)
- **Examples**: `megold` -> `megoldatlan`; `kitakarít` -> `kitakarítatlan`; `borotvál` -> `borotválatlan`; `tanul` -> `tanulatlan`; `vasal` -> `vasalatlan`; `fizet` -> `fizetetlen`; `kifizet` -> `kifizetetlen`; `tölt` -> `töltetlen`; `mos` -> `mosatlan`; `fest` -> `festetlen`; `söpör` -> `söpretlen`
- **Exceptions**: listed lexicalised forms
- **Predictability**: `rule` (transitive verbs with an independent object) + `lexical`
- **Lexical input**: verb `transitive` (carried); `effected-object` (creation verbs) and `stative` (not carried, so the engine checks transitivity only)
- **Interactions**: `VRB-PART-004` (stem), `DER-ADJ-020`, `DER-ADV-003`
- **Sources**: [SMNy3 pp. 132-135 (8, 12)], [SMNy3 p. 675 (Rebrus deverbal `-(A)tlAn`, `-tAlAn` minimal: `untalan`, `nincstelen`, `védtelen`, `szüntelen`, `nyugtalan`)], [AkH12 §39], [KVF98 pp. 366-367 (900)], [WC2], [ATT evidence/productivity-tagset.md 2026-09-16], [ÉrtSz]
- **Validation**: attestation; Webcorpus 2: 41,794 `-AtlAn` adjective lemma types, 23.1M tokens; over N = 4,064 frequent transitive verbs e = 2,038 lack any privative (theta = 489, so e/theta = 4.2), so transitivity alone does not decide; the residue's creation verbs are 0, its transfer verbs are attested (counts above). Also `megoldatlan` 8, `olvasatlan` 1, `átlátszatlan` 4, `független` 18, `nyugtalan` 9, `szüntelen` 9, `hallatlan` 7, `ingatlan` 18
- **Confidence**: consensus for the suffix; the independent-object condition is a diagnosis consistent with [WC2], not a settled rule, until the lexicon carries an effected-object feature ([`evidence/premise-audit.md`](evidence/premise-audit.md))

### DER-ADJ-011 Output of privative adjectives
- **Statement**: Privative adjectives are non-lowering, form adverbs only with `-Ul`, feed `-sÁg`, `-kOdik`, `-ít`, `-Odik` (the last two lexically), and do not take `-An`.
- **Conditions**: any `-tAlAn/-tlAn/-AtlAn` adjective
- **Output**: plural `-ok/-ek` (`tanulatlanok`, `boldogtalanok`); adverb `-ul/-ül` (`bátortalanul`, `feltétlenül`); `-sÁg` (`pártatlanság`, `ügyetlenség`); `-kOdik` (`nyugtalankodik`, `ügyetlenkedik`); `-ít` (`portalanít`, `lombtalanít`, not `*lábatlanít`); `-Odik` with `el-` (`elsótlanodik`, `elbátortalanodik`, `elerőtlenedik`)
- **Examples**: listed
- **Exceptions**: `-ít`/`-Odik` coverage is **lexical, measured**: 215 of 782 frequent privatives feed such a verb, against a Tolerance threshold of 117, so it is not a productive default and must not be generated. The productive-looking band is removal of a physical substance or a legal property (`méregtelenít` 20,136, `portalanít` 3,908, `zsírtalanít` 3,458, `szőrtelenít` 3,335, `légtelenít` 2,979, `áramtalanít` 11,118, `ivartalanít` 8,544, `érvénytelenít` 16,762, `hiteltelenít` 3,698, `hatástalanít` 14,331), but that is a **ranking hint, not a licence to generate**: `e` exceeds `theta` by a factor of 4.8 even inside the semantic band.
- **Predictability**: `rule` (lowering, adverb); `lexical` (verbs) - unchanged, now with evidence
- **Lexical input**: the 215-item list of privatives that feed an `-ít`/`-Odik` verb
- **Interactions**: `DER-MODEL-010`, `DER-ADV-003`, `DER-VERB-008`, `DER-VERB-009`, `DER-VERB-012`, `NOM-ADJ-011`
- **Sources**: [T04 p. 161 fn. 36], [SMNy3 pp. 160-162, 164-165], [morphdb grammar AFF_TALAN (OUT aff_z aff_kodik aff_ít aff_ság aff_s aff_bb aff_ul aff_an)], [WC2], [ATT evidence/productivity-tagset.md 2026-09-16]
- **Validation**: attestation; Webcorpus 2: 2,111 privative-based verb lemma types, 253,826 tokens; the chapter's own contrast confirmed, `portalanít` 3,908 vs `*lábatlanít` 0; over N = 782 privatives at freq >= 1,000, e = 567 lack any such verb (theta = 117). Also `bátortalanul` 2, `feltétlenül` 16, `tudatlanul` 2; goldens
- **Confidence**: consensus; `measured` (`OQ-06-010` resolved: no semantic subclass licenses generation, keep the list)

### DER-ADJ-012 -beli
- **Statement**: `-beli` forms containment-relation adjectives ('located in, belonging within N'); it is productive with country names, continent, region and city names whose locative is inessive, institutions, arts, collectives, products and abstract nouns; it is non-lengthening, invariant front, and may follow plural and possessive suffixes.
- **Conditions**:
  - productive bases: country names (`németországbeli`, `olaszországbeli`, `albániabeli`); continents, regions and cities with inessive `-bAn` locative (`afrikabeli`, `ázsiabeli`, `párizsbeli`, `varsóbeli`), not with superessive locative (`*budapestbeli`, `*távol-keletbeli`, `*eszékbeli`, `NOM-PROP-006`); institutions (`parlamentbeli`, `egyetembeli`); arts requiring skill (`építészetbeli`, `festészetbeli`; disciplines marginal `?teológiabeli`); collectives (`csoportbeli`, `osztálybeli`); products incl. intellectual (`szekrénybeli`, `autóbeli`, `könyvbeli`, `filmbeli`); abstract nouns (`békebeli`, `háborúbeli`, `munkabeli`, `életbeli`); `-Ás` nouns 'arising within' (`látásbeli`, `főzésbeli`)
  - time nouns: excluded in the singular (`*napbeli` 'of the day', `*tavaszbeli`; `napbeli` 'in the sun' is `nap` 'sun'), acceptable in the plural (`(az elmúlt) hónapokbeli`)
  - after possessive: `falumbeli`, `városodbeli`, `utcátokbeli`, `házunkbeli`
  - acronyms and clippings allowed (`PM-beli`, `koleszbeli`) unlike `-i`
  - no LVL, i.e. non-lengthening (`PH-LEN-002`): `korabeli` 211,974, `mesebeli` 34,063, `szakmabeli` 11,811, `utcabeli`, `afrikabeli`, against `korábeli` 3 and `mesébeli` 5, which are typo-level [WC2]; harmony `F!` (`falubeliek`)
  - rival `-i`: `utcai verekedés` (typical location) vs `utcabeli verekedés` (in a particular street); `reformkori ~ reformkorbeli`; `intézeti` / `intézetbeli`; person nouns only `-i`
  - lexicalised: `számbeli`, `élvonalbeli`, `szóbeli`, `írásbeli`, `vérbeli`, `ténybeli`, `pénzbeli`, `észbeli`
  - archaic variant `-béli` (analysis only)
- **Output**: adjective, lowering (`falubeliek` with obligatory linking vowel); derivation `-sÁg` (`osztálybeliség` rare); adverb in `-An` (`mesebelien` 324, `vérbelien` 57) and in `-lAg` (`számbelileg` 957, `időbelileg` 925, `térbelileg` 803), both now allowed (`DER-ADV-001`, `DER-ADV-004`, `DER-ADV-005`)
- **Examples**: listed
- **Exceptions**: listed
- **Predictability**: `rule` (classes) + `lexical`
- **Lexical input**: place-name locative type (`NOM-PROP-006`); semantic classes
- **Interactions**: `DER-ADJ-001`, `DER-MODEL-003` (after inflection), `DER-MODEL-005`, `DER-COMP-017`, `NOM-PROP-006`
- **Sources**: [SMNy3 pp. 139-142 (4.2.3, 4.2.3.1)], [KVF98 pp. 369-370 (911)], [SMNy3 p. 675 (coordination `tér- és időbeli`)], [SMNy3 p. 125 (`-lAg` after `-beli`; arbitrated correct)], [morphdb grammar AFF_BELI (`-béli` archaic)], [WC2], [ATT evidence/productivity-tagset.md 2026-09-16], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: attestation (`korabeli` 211,974 : `korábeli` 3, `mesebeli` 34,063 : `mesébeli` 5, `szakmabeli` 11,811 : `szakmábeli` 0 [WC2]; `filmbeli` 5, `csoportbeli` 0, `számbeli` 5, `szóbeli` 16, `vérbeli` 8, `pénzbeli` 14, `élvonalbeli` 11, `írásbeli` 17)
- **Confidence**: consensus; `measured` for non-lengthening (`OQ-02-008`) and for the `-An` / `-lAg` adverbs (`OQ-06-011`, `OQ-06-016`)

### DER-ADJ-013 -nyi: quantity adjectives
- **Statement**: `-nyi` forms approximate-measure adjectives ('an amount the size of N') from any concrete noun or unit noun; it is invariant, transparent, LVL-triggering, attaches to the free stem, and its outputs admit neither `-sÁg` nor `-An`.
- **Conditions**:
  - base: concrete noun usable as a container or measure (`kanál`, `tányér`, `pohár`, `vödör`, `ajtó`, `ház`, `marok`, `polc`), unit noun (`akó`, `liter`, `kiló`, `deka`), abstract quantity nouns (`pillanatnyi`, `hajszálnyi`)
  - free stem: no shortening (`kanálnyi`), no epenthetic drop (`vödörnyi`), LVL (`tonnányi`, `ládányi`)
  - numeral + `N-nyi` written together for simple members (`húszfőnyi`, `ORT-TOG-023`)
  - usable attributively and predicatively
- **Output**: adjective (quantifier-like), lowering; `-nyian` numeral adverb not formed productively; `*-nyiság`
- **Examples**: `kanál` -> `kanálnyi`; `pohár` -> `pohárnyi`; `vödör` -> `vödörnyi`; `ajtó` -> `ajtónyi`; `ház` -> `háznyi`; `liter` -> `liternyi`; `kiló` -> `kilónyi`; `deka` -> `dekányi`; `tonna` -> `tonnányi`; `marok` -> `maroknyi`; `ököl` -> `öklömnyi` (lexical, with possessive)
- **Exceptions**: `maréknyi` from `marék` (variant base); `öklömnyi`, `picinyi` lexical
- **Predictability**: `rule`
- **Lexical input**: noun feature `concrete`
- **Interactions**: `DER-MODEL-005`, `DER-NOUN-004`, `DER-ADV-001`
- **Sources**: [SMNy3 p. 152 (4.2.6, rule 36)], [SMNy3 pp. 114, 127 (`*liternyiség`)], [KVF98 p. 365 (896)], [AkH12 §35 (`öklömnyi`), §117 (`húszfőnyi`)], [morphdb grammar AFF_NYI (analytic lengthened)]
- **Validation**: attestation (`kanálnyi` 4, `pohárnyi` 1, `maréknyi` 6, `tonnányi` 6; `*tonnanyi` unattested)
- **Confidence**: consensus

### DER-ADJ-014 -szerű: similarity (semi-word)
- **Statement**: `-szerű` ('N-like, having the character of N') attaches to nouns, adjectives and pronouns; it behaves as a compound member: no LVL, invariant front harmony domain, full spelling of doubled `sz` at the boundary, coordination ellipsis.
- **Conditions**:
  - base: common noun, adjective, pronoun (`ilyenszerű`), proper name (hyphen, `ORT-HYPH-004`)
  - spelling: `sz`-final base keeps both (`gipszszerű`, `észszerű`, `mészszerű`); `ssz`-final base: hyphen (`expressz-szerű`, `ORT-HYPH-001`)
  - pronunciation: `t+sz` not coalesced as within a word (`szigetszerű` like `Margitsziget`)
  - lexicalised: `nagyszerű`, `észszerű`, `szabályszerű`; `egyszerű` (from `egy` + `szer` + `ű`) is monomorphemic for the engine
- **Output**: adjective, lowering (`kutyaszerűek`); feeds `-sÁg` (`észszerűség`), `-tlen` (`szabályszerűtlen`), `-lAg` (`természetszerűleg`), `-An` (`észszerűen`)
- **Examples**: `könyv` -> `könyvszerű`; `rádió` -> `rádiószerű`; `fa` -> `faszerű`; `kutya` -> `kutyaszerű`; `Petőfi` -> `Petőfi-szerű`; `Coca-Cola` -> `Coca-Cola-szerű`
- **Exceptions**: `ésszerű` AkH11 legacy spelling (`DER-MODEL-014`)
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-COMP-017`, `DER-COMP-018`, `ORT-TOG-041`, `ORT-HYPH-004`, `WS-DIG-004`
- **Sources**: [AkH12 §92, §132, §133, §216 a)], [SMNy3 pp. 55-56, 123, 675], [KVF98 p. 365 (897)], [morphdb grammar AFF_SZERŰ]
- **Validation**: attestation (`észszerű` 6, `ésszerű` 12), goldens
- **Confidence**: normative (spelling); consensus (semi-word status)

### DER-ADJ-015 -féle, -fajta, -forma, -nemű, -rét, -rétű (semi-words)
- **Statement**: The kind- and form-denoting elements `-féle`, `-fajta`, `-forma`, `-nemű`, `-rét`, `-rétű` attach to nouns, numerals and pronouns as compound members: no LVL, own invariant harmony, coordination ellipsis, compound spelling.
- **Conditions**:
  - `-féle` 'kind of' (front: `kutyafélék`); `-fajta` 'sort of' (back: `sörfajták`); `-forma` 'shaped like, resembling' (back); `-nemű` 'of the kind' (front, lowering: `egyneműek`); `-rét` 'fold' (`negyedrét`), `-rétű` 'layered' (`sokrétű`)
  - with numerals and pronouns: `tízféle`, `többféle`, `másfajta`, `efféle`, `affajta` (`az`/`ez` assimilation, `ORT-ASSIM-009`), `egynemű`
  - with digits: hyphen (`10-féle`); with proper names: hyphen (`Eötvös-féle`, `Illyés-fajta`, `Kossuth-forma`, `ORT-HYPH-004`)
  - `-féle` nouns in taxonomy: `pókféle`, `bükkfélék` (plural noun use)
  - three identical letters: hyphen (`puff-féle`)
- **Output**: adjective (and taxonomic noun for `-féle`); lowering `n` for `-féle`, `-fajta`, `-forma` (vowel-final), `y` for `-nemű`, `-rétű`
- **Examples**: listed
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-COMP-017`, `ORT-TOG-041`, `ORT-HYPH-004`, `ORT-ASSIM-009`
- **Sources**: [AkH12 §132, §133, §83 (`affajta`, `afféle`)], [SMNy3 pp. 55-56, 123, 441, 675], [morphdb grammar AFF_FÉLE, AFF_FAJTA, AFF_NEMŰ]
- **Validation**: goldens
- **Confidence**: normative (spelling); consensus (status)

### DER-ADJ-016 -mentes
- **Statement**: `-mentes` ('free of N') forms adjectives from mass and abstract nouns; it is a semi-word (N + ADJ compound head `mentes`), non-lengthening and invariant front.
- **Conditions**:
  - base: substance or defect noun (`hiba`, `adó`, `vám`, `cukor`, `glutén`, `zsír`, `vita`)
  - relation to syntax: `hibától mentes` -> `hibamentes`
- **Output**: adjective, lowering by `DER-MODEL-010` (front unrounded: visible only in the accusative linking vowel, `hibamenteset`, not `*hibamentest`, `NOM-LOW-003`); poss3 `-a/-e` if nominalised (morphdb `poss_sg_A`); feeds `-ít` (`mentesít` is from `mentes`), `-sÁg` (`adómentesség`)
- **Examples**: `hibamentes`, `adómentes`, `vámmentes`, `cukormentes`, `zsírmentes`, `vitamentes` (with no LVL: `hibamentes`, not `*hibámentes`)
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: noun feature `mass`/`abstract`
- **Interactions**: `DER-COMP-017`, `DER-COMP-003` (N + ADJ pattern)
- **Sources**: [KVF98 p. 379 (941 a) `hiba-mentes`)], [morphdb grammar AFF_MENTES (analytic non_lengthened, front)]
- **Validation**: attestation (`hibamentes` 4, `adómentes` 10)
- **Confidence**: consensus

### DER-ADJ-017 -Ós: adjectives from verbs
- **Statement**: Adjectives in `-ós/-ős` are the `-s` adjective of the present participle, in two readings: subject-oriented 'habitually V-ing, prone to V', productive from non-agentive verbs and from frequentative or durative verbs; and event-oriented 'involving V-ing', on an event or artefact noun, productive from activity verbs.
- **Conditions**:
  - 'prone to V': non-agentive intransitive (`fázik` -> `fázós`, `mosolyog` -> `mosolygós`, `pityereg` -> `pityergős`, `reszket` -> `reszketős`); verbs with a frequentative or durative suffix even if agentive (`futkároz` -> `futkározós`, `rohangál` -> `rohangálós`, `üldögél` -> `üldögélős`, `nézeget` -> `nézegetős`, `kötöget` -> `kötögetős`); a non-derived agentive verb gives no subject-oriented adjective, so `úszós`, `futós`, `ballagós` do not mean 'prone to swim, run, walk'
  - 'involving V-ing': any activity verb, agentive and non-derived ones included (`beszélgetős` 16,113, `lövöldözős` 8,241, `verekedős` 7,605, `főzős` 5,476, `úszós` 4,007, `futós` 2,967, `ugrálós` 2,929, `éneklős` 1,853, `táncolós` 1,629, `túrázós` 1,373, `ballagós` 140 [WC2]: `úszós edzés`, `ballagós ruha`); not from stative verbs (`tartalmazós` 2, `gyűlölős` 3, `birtokolós` 0 [WC2])
  - verb phrases via their `-Ó` compound (`levélíró` -> `levélírós`, `filmnéző` -> `filmnézős` 531 [WC2]; `*levelet írós`); an unattested one is a potential word (`#levesevős`)
  - an `-Ó` noun takes `-(O)s` by `DER-ADJ-005` ('with or about N'), which is not this suffix: `repülő` 'aircraft' -> `repülős` 15,845, `kötő` 'apron' -> `kötős` [WC2]
  - non-subject readings of the prone type are lexicalised (`rágós hús`, `csiklandós`)
  - blocked by unproductive `-ékony` synonyms where no meaning split: `#fáradós` (`fáradékony`), `#gyúlós` (`gyúlékony`), `#romlós` (`romlékony`); coexisting with split: `omlós` / `omlékony`, `tanulós` / `tanulékony`
- **Output**: adjective, lowering (`fázósak`)
- **Examples**: `fázós`, `mosolygós`, `spórolós`, `érzelgős`, `veszekedős`, `csípős`, `harapós`, `ropogós`, `félős`, `viszketős`, `kacsintós`, `beszélgetős`, `lövöldözős`, `túrázós`
- **Exceptions**: listed
- **Predictability**: `rule` (verb classes) + `lexical` (blocking)
- **Lexical input**: verb features `agentive` (prone reading) and `stative` (event reading); neither is carried, so generation takes the participle of every verb
- **Interactions**: `VRB-PART-001`, `DER-ADJ-005`, `DER-ADJ-018`, `DER-LEX-002`
- **Sources**: [SMNy3 pp. 150-152 (4.2.5, rule 35)], [KVF98 p. 367 (901)], [WC2]
- **Validation**: attestation (`fázós` 2, `mosolygós` 8, `csípős` 10, `harapós` 4, `ropogós` 10, `félős` 3, `omlós` 7, `rágós` 2; event-reading counts above)
- **Confidence**: consensus for the prone reading; the event reading is measured [WC2] and absent from SMNy3, whose exclusion of non-derived agentive verbs holds for the prone reading only

### DER-ADJ-018 -ékOny: disposition adjectives
- **Statement**: `-ékony/-ékeny` ('prone to V, V-able') is an unproductive deverbal adjective suffix with two alternants only (no front rounded `*-éköny`).
- **Conditions**: back -> `-ékony`; front (also rounded) -> `-ékeny` (`törékeny`)
- **Output**: adjective, lowering (`tanulékonyak`)
- **Examples**: `tanulékony`, `omlékony`, `gyúlékony`, `romlékony`, `fáradékony`, `lobbanékony`, `hiszékeny`, `feledékeny`, `engedékeny`, `törékeny`, `érzékeny`
- **Exceptions**: none
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `DER-ADJ-017`, `DER-MODEL-010`
- **Sources**: [SMNy3 p. 674 (Rebrus `-ékony`)], [SMNy3 p. 151], [T04 p. 161 (`tanul-ékony-ak`)]
- **Validation**: attestation (`gyúlékony` 10, `hiszékeny` 5, `törékeny` 10, `érzékeny` 15)
- **Confidence**: consensus

### DER-ADJ-019 -hAtÓ: modal adjectives
- **Statement**: `-ható/-hető` ('V-able') is an unanalysed complex suffix producing adjectives whose head noun is an internal argument of the verb: the object of a transitive verb, or the place or path argument of an intransitive one; forms are owned by `VRB-PART-004`, productivity and meaning here.
- **Conditions**:
  - transitive base: the head is the object (`olvasható könyv`)
  - intransitive base: the head is the place or path of the event (`élhető város` 44,897, `járható út` 61,472, `lakható ház` 12,977, `hajózható` 4,684, `futható` 2,702, `horgászható` 1,593, `síelhető` 1,102, `úszható` 820, `kerékpározható` 698, `parkolható` 188 [WC2]); productive, not lexicalised
  - an oblique argument heads it where the verb governs one (`megbízható` 'trustworthy': `megbízik vkiben`; `hozzáférhető`: `hozzáfér vmihez`)
  - not analysed as `-hAt` + `-Ó`: the `-Ó` participle of a potential verb would have an agentive subject reading, which `-hAtÓ` lacks
- **Output**: adjective, lowering with optional linking vowel (`olvashatók ~ olvashatóak`); feeds `-sÁg` (`olvashatóság`), `-An`, `-lAg` (`láthatólag`)
- **Examples**: `olvas` -> `olvasható`; `nyit` -> `nyitható`; `említ` -> `említhető`; `eszik` -> `ehető`; `iszik` -> `iható`; `hisz` -> `hihető`; `él` -> `élhető`; `jár` -> `járható`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: verb argument frame (object, place or path argument); only `transitive` is carried, so generation takes every verb
- **Interactions**: `VRB-PART-004`, `VRB-POT-001`, `DER-ADJ-020`
- **Sources**: [SMNy3 pp. 124-125 (complex suffix argument)], [KVF98 p. 366 (898)], [morphdb grammar AFF_HATÓ], [WC2]
- **Validation**: goldens, oracle:hunmorph-foma, attestation (counts above)
- **Confidence**: consensus for transitive bases; the intransitive condition is measured [WC2], against the sources' transitive-only statement

### DER-ADJ-020 -hAtAtlAn: negative modal adjectives
- **Statement**: `-hatatlan/-hetetlen` ('un-V-able') has the distribution of `-hAtÓ`; forms owned by `VRB-PART-004`; several members lack a positive counterpart or differ from it in meaning.
- **Conditions**: the head is the object or the place or path argument, as for `-hAtÓ` (`DER-ADJ-019`); lexical members without a positive `-hAtÓ`: `nyughatatlan`; with a meaning split: `élhetetlen` 'inept' beside `élhető` 'livable'
- **Output**: adjective, non-lowering; adverb `-Ul` (`elkerülhetetlenül`); feeds `-sÁg`, `-kOdik` (`élhetetlenkedik`)
- **Examples**: `olvashatatlan`, `említhetetlen`, `megbízhatatlan`, `hozzáférhetetlen`, `elképzelhetetlen`, `ehetetlen`, `ihatatlan`, `törhetetlen`, `járhatatlan`
- **Exceptions**: listed
- **Predictability**: `rule` + `lexical`
- **Lexical input**: as `DER-ADJ-019`
- **Interactions**: `VRB-PART-004`, `DER-ADJ-010`, `DER-ADJ-011`
- **Sources**: [SMNy3 pp. 124-125], [KVF98 p. 366 (899)], [morphdb grammar AFF_HATATLAN]
- **Validation**: goldens
- **Confidence**: consensus

### DER-ADJ-021 -AndÓ: future participle adjectives
- **Statement**: `-andó/-endő` ('to be V-ed') attaches to transitive achievement verbs; forms, stems and inflection are owned by `VRB-PART-003`; the output is an adjective with optional linking vowel.
- **Conditions**: transitive base (`elvégez`, `kinyit`); literary or official register
- **Output**: adjective, lowering, optional linking vowel (`elvégzendők ~ elvégzendőek`)
- **Examples**: `elvégez` -> `elvégzendő`; `kinyit` -> `kinyitandó`; `megold` -> `megoldandó`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: verb `transitive`
- **Interactions**: `VRB-PART-003`, `NOM-PL-003`
- **Sources**: [KVF98 p. 367 (902)], [SMNy3 pp. 372-382 (7.2)]
- **Validation**: goldens
- **Confidence**: consensus

### DER-ADJ-022 Participles converted to adjectives
- **Statement**: Present `-Ó` and past `-(O)tt` participles convert to adjectives; the converted adjective is gradable and predicative, the participle is not; forms are owned by `VRB-PART-001`, `VRB-PART-002`.
- **Conditions**: conversion is productive for `-Ó` with psychological causatives (`fárasztó`, `szórakoztató`, `meglepő`) and for `-(O)tt` resultatives (`ijedt`, `fáradt`, `írott`); lexically listed otherwise
- **Output**: adjective, lowering; `-An` adverb (`meglepően`, `fáradtan`), `-sÁg` (`fáradtság`), `-lAg` (`visszamenőleg`)
- **Examples**: `a lovakat fárasztó istállómester` (participle) vs `fárasztó feladat` (adjective, `fárasztóbb`)
- **Exceptions**: past participle in `-ott` where the past tense has `-t` is lexical (`írott` vs `írt`, `VRB-PART-002`)
- **Predictability**: `rule` (classes) + `lexical`
- **Lexical input**: verb semantic class
- **Interactions**: `DER-MODEL-015`, `VRB-PART-001`, `VRB-PART-002`, `VRB-PART-005`
- **Sources**: [SMNy3 p. 102], [SMNy3 pp. 368-372 (7.1.7, 7.1.8)], [KVF98 p. 365]
- **Validation**: goldens
- **Confidence**: consensus

### DER-ADJ-023 Latinate adjective endings
- **Statement**: Adjectives borrowed with the endings `-ikus`, `-ális`, `-áris`, `-ív`, `-ózus`, `-áns`, `-ens` form lexical classes whose members share correspondences with `-ika`, `-itás`, `-izmus`, `-izál`, `-ista` forms on the same bound stem; the endings fix harmony and the adjectives are lowering.
- **Conditions**:
  - harmony: `-ikus`, `-ális`, `-áris`, `-ózus` back opaque (`normálisak`, `komikusak`); `-ív` neutral transparent: harmony from the stem (`aktívak`, `pozitívak`, `szelektívek`, `effektívek`); `-áns` and `-ens` are separate lexical endings, not harmonic alternants (`elegánsak`, `intelligensek`)
  - correspondences (rule on bound stem X): `X-ikus` <-> `X-ika` N (`logikus` / `logika`, `fizikus` N / `fizika`); `X-ális` <-> `X-alitás` <-> `X-alizál` <-> `X-alizmus` (`reális`, `realitás`, `realizál`, `realizmus`); `X-ív` <-> `X-ivitás` (`aktív`, `aktivitás`); `-ózus` <-> `-óz-ság` (`morózusság`), not `*-izál`
  - adjective vs noun conversion: `komikus` ADJ lowering / N non-lowering (`DER-MODEL-015`)
  - 3sg possessive of converted nouns: `-a` (`fizikusa`)
- **Output**: lexicon entries with correspondence links; generation of correspondents only if the target is listed or the target suffix is `P-dom` for foreign stems (`-itás`, `-izál`)
- **Examples**: `logikus`, `historikus`, `szimbolikus`, `normális`, `formális`, `virtuális`, `populáris`, `aktív`, `kreatív`, `dekoratív`, `pozitív`, `morózus`, `rigorózus`, `elegáns`, `intelligens`
- **Exceptions**: `-ív` adjectives with an `-ál` verb do not take `-izál` (`#kreativizál`, `DER-VERB-006`)
- **Predictability**: `lexical` (membership), `rule` (correspondence forms, harmony)
- **Lexical input**: bound Latinate stem with correspondence flags
- **Interactions**: `DER-NOUN-015`..`DER-NOUN-017`, `DER-VERB-006`, `PH-HARM-020`, `PH-LEN-011`, `DER-MODEL-010`
- **Sources**: [SMNy3 pp. 129, 163 (4.1.3, 4.3.9)], [SMNy3 p. 677 (Rebrus Latin suffixes truncating)], [T04 pp. 156-157 (`komikus-ak` / `komikus-ok`)], [emMorph stemalt.rul (Latin `-id`, `-kus`, `-ns` lowering; `-ív` `-ja`) (study only)]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-ADJ-024 -AtAg, -Ag
- **Statement**: `-atag/-eteg` and `-ag/-eg` are unproductive adjective (and noun) suffixes on bound or verbal stems.
- **Conditions**: listed words only; several share bound stems with `-Ad`, `-Odik`, `-Ez` verbs
- **Output**: adjective or noun, lowering (adjectives)
- **Examples**: `-AtAg` (24 items): `álmatag`, `hervatag`, `korhatag`, `lankatag`, `olvatag`, `roskatag`, `sápatag`, `élveteg`, `förmeteg`, `réveteg`, `téveteg`, `förgeteg`; nouns `sivatag`, `zuhatag`; denominal `balgatag`, `gyermeteg`, `szörnyeteg`; `-Ag`: `hallgatag`, `reszketeg`, `viszketeg`, `avatag`, `forgatag`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `DER-LEX-005`
- **Sources**: [SMNy3 pp. 674-675 (Rebrus `-AtAg` {24}, `-Ag (unproductive mark)`)]. Attested lemma counts with at least 100 corpus tokens are far above Rebrus' figures - `-mÁny` 402, `-vÁny` 223, `-(O)dA` 260, `-nOk` 159, `-(O)nc` 90, `-AtAg` 58, `-lAg` 86 [WC2] - because Rebrus is evidently counting non-compound, non-prefixed dictionary entries while ours include compounds (`keresztrejtvény`, `főmérnök`). The root/compound split is not computed for this list, so the comparison is not yet apples to apples. Inventories in `derivational-inventories.tsv` (1,185 rows), `local/lists/derivational-inventories.tsv` - working data, not yet read by the build. [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: attestation (`sivatag` 9, `zuhatag` 3, `hallgatag` 6, `gyermeteg` 3, `szörnyeteg` 8)
- **Confidence**: consensus

### DER-ADJ-025 Numeral-based adjectives and nouns (pointer)
- **Statement**: Fraction `-Od`, ordinal `-(O)dik`, collective `-(O)s` on numerals (`ötös`), `-szOrI`, `-szOrOs`, `-An` aggregate (`öten`) and tens `-vAn` are numeral morphology owned by `PND`; they are derivational by `DER-MODEL-001` and their record values are those given there.
- **Conditions**: numeral base
- **Output**: see `PND`
- **Examples**: `hat` -> `hatod`, `hatodik`, `hatos`, `hatszori`, `hatszoros`, `hatan`; `nyolc` -> `nyolcvan` (lowering `nyolcvanat`)
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: numeral stems
- **Interactions**: `PND`, `DER-MODEL-010`
- **Sources**: [SMNy3 pp. 675-676 (Rebrus `-Od/Ad`, `-ik`, `-szOr(tA)`)], [KVF98 p. 370 (913), (914)], [T04 p. 161 fn. 36 (`-van` lowering)]
- **Validation**: goldens
- **Confidence**: consensus

## DER-VERB

### DER-VERB-001 Choosing -(O)z or -(O)l for new denominal verbs
- **Statement**: The rival denominal verbalisers `-(O)z` and `-(O)l` ('be occupied with N') are in complementary distribution in new formations: `-(O)l` after monosyllabic consonant-final bases not ending in `r` or `l`, `-(O)z` elsewhere; lexical field and homonymy override the phonological rule.
- **Conditions**: (ordered, first match wins)
  1. lexical entry exists (including meaning-split doublets, `DER-VERB-004`)
  2. base is a verb-derived English `-ing` noun of action: `-(O)l` (`lízingel`, `shoppingol`, `joggingol`; doublets `tréningel ~ tréningezik`)
  3. base is a sport or game noun: `-(O)z` (`bobozik`, `dartsozik`, `quake` -> `quake-ezik`); doublets possible (`szörfözik ~ szörföl`)
  4. base is a loan verb stem without noun use: see `DER-VERB-003`
  5. homonymy with an existing verb: the other suffix (`kertez` because `kertel` exists)
  6. vowel-final base: `-(O)z` (`diszkózik`, `videózik`, `magnózik`, `legózik`)
  7. `r`- or `l`-final base: `-(O)z` (`e-mailezik`, `futballozik`, `radíroz`)
  8. monosyllabic consonant-final base: `-(O)l` (`bokszol`, `parkol`, `voksol`, `stoppol`, `szkennel`, `printel`, `kódol`)
  9. elsewhere (polysyllabic consonant-final): `-(O)z` (`internetezik`, `számítógépezik`, `telefonozik`)
- **Output**: choice of verbaliser; allomorphy per `DER-VERB-002`
- **Examples**: listed
- **Exceptions**: historical `-l` verbs on vowel-final bases are being replaced by `-z` (`vacsorál` -> `vacsorázik`, `pipál` -> `pipázik`); the reverse change is not attested
- **Predictability**: `rule` (new formations) + `lexical` (existing verbs)
- **Lexical input**: noun features `sport_game`, `ing_action_loan`; existing-verb lexicon
- **Interactions**: `DER-VERB-002`, `DER-VERB-003`, `DER-VERB-004`, `DER-LEX-003`
- **Sources**: [SMNy3 pp. 152-154 (4.3.1, rules 37, 38)], [SMNy3 pp. 112 (test items), 120 (`kertez`)], [KVF98 pp. 357-358 (869), (870)]
- **Validation**: goldens (new-word set), oracle:hunmorph-foma
- **Confidence**: consensus (rule from a nonce-word study reported by SMNy3); `tendency` for items

### DER-VERB-002 -(O)z and -(O)l allomorphy
- **Statement**: `-(O)z` and `-(O)l` attach with a plural-type linking vowel to the bound stem, lengthen stem-final `a/e`, and form verbs that optionally take `-ik` (`-z` intransitives mostly with `-ik`).
- **Conditions**:
  - vowel-final base: `-z`, `-l`; LVL (`vacsorázik`, `kaszál`, `kártyázik`)
  - consonant-final non-lowering base: `-oz/-ez/-öz`, `-ol/-el/-öl` (`gombol`, `dobol`, `rajzol`, `kódol`, `sörözik`, `bokszol`)
  - consonant-final lowering base: `-az/-ez`, `-al/-el` (`falaz`, `vasal`, `fogaz`, `ágyaz`, `talpal`, `szárnyal`)
  - bound stem: v-stems (`havazik`, `kövez`), shortening stems (`kezel`, `nyaral`, `telel`, `vizez`), epenthetic stems: **26 of the 28 epenthetic nouns with an attested `-(O)z` verb drop the vowel** (`tükröz`, `dolgoz`, `halmoz`, `értelmez`, `használ`); only `kebel` and `sátor` keep it (`sátoroz`) [WC2]
  - intransitive activity verbs from `-z`: citation form with `-ik` (`autózik`, `rádiózik`, `ülésezik`); transitive: without (`keretez`, `csontoz`); both forms exist with prefixes (`bűnözik` / `lebűnöz`)
  - `-ú/-ű`-final bases before `-l`: final vowel shortened (`PH-LEN-012`)
- **Output**: verb stem; T-class per `VRB-STEM-014`
- **Examples**: `keret` -> `keretez`; `rádió` -> `rádiózik`; `autó` -> `autózik`; `fal` -> `falaz`; `bácsi` -> `bácsiz`; `csont` -> `csontoz`; `ülés` -> `ülésezik`; `talp` -> `talpal`; `kasza` -> `kaszál`; `szónok` -> `szónokol`; `tank` -> `tankol`
- **Exceptions**: lexical epenthetic behaviour; `-ik` presence lexical
- **Predictability**: `rule` + `lexical` (`-ik`, epenthetic stems)
- **Lexical input**: stem `lowering`, alternants; verb `-ik` flag
- **Interactions**: `DER-MODEL-004`, `DER-MODEL-006`, `VRB-IK-007`, `PH-LEN-012`
- **Sources**: [KVF98 pp. 357-358], [SMNy3 p. 673 (Rebrus `-(O/A)z(ik)`, `-(O/A)l(ik)`: linking vowel may drop before vowel-initial suffixes: `elnökölő` ~ `elnöklő`, `ebédelő` ~ `ebédlő`)], [morphdb grammar AFF_Z, AFF_L], [morphdb derivation README (Z: `bűnöz/bűnözik`; L: `hegedül` lexical)], [WC2], [ATT evidence/lexical-lists.md 2026-09-16]
- **Validation**: goldens, oracle:hunmorph-foma; 28 epenthetic nouns with an attested `-(O)z` verb probed, list `epenthetic-z-derivation.tsv` in `local/lists/epenthetic-z-derivation.tsv` - working data, not yet read by the build
- **Confidence**: consensus; the drop/keep split is `measured` (26 drop, 2 keep; `OQ-06-009` resolved)

### DER-VERB-003 Integrating loan verbs: -ál and -(O)l
- **Statement**: Foreign verb stems without Hungarian noun use are integrated with `-(O)l` if monosyllabic or polysyllabic with front harmony, and with `-ál` if polysyllabic with back or mixed harmony; Latinate nouns in `-áció`/`-ció` correspond to `-ál` verbs.
- **Conditions**:
  - monosyllabic loan verb stem: `-(O)l` (`klikkel`, `bútol`, `printel`, `szével`, `tapol`)
  - polysyllabic front loan stem: `-(O)l` (`menedzsel`, `editel` 'issue an edit command')
  - polysyllabic back or mixed loan stem: `-ál` (`formattál`, `installál`, `editál` 'edit', `tabulál`); nonce `logodál ~ logodol`, `ratilál`
  - Latinate correspondence: `-áció` N <-> `-ál` V (`koncentráció` / `koncentrál`, `kommunikáció` / `kommunikál`, `definíció` / `definiál`, `reakció` / `reagál` with lexical consonant change)
  - `-ál` is invariant back and opaque (`definiálok`)
  - `-ál` does not attach to monosyllabic stems in nonce tests (`*zegál` -> `zegel`)
- **Output**: verb
- **Examples**: listed; `telefonál` (lexical `-ál` on a noun), `diszkvalifikál`, `reagál`
- **Exceptions**: back-harmony monosyllables take back `-ol` (`bútol`, `tapol`); doublets with meaning split (`editel` / `editál`)
- **Predictability**: `rule` + `lexical`
- **Lexical input**: loan stem flag `verb_origin`; Latinate correspondence links
- **Interactions**: `DER-VERB-001`, `DER-VERB-005`, `DER-VERB-006`, `PH-HARM-020`, `PH-HARM-024`
- **Sources**: [SMNy3 pp. 153-155 (4.3.1, 4.3.2, rule 39)], [SMNy3 p. 125 (`-ál`, `-izál`, `-itás` as suffixes)], [KVF98 p. 358 (873 a)]
- **Validation**: attestation (`reagál` 12, `definiál` 3, `koncentrál` 10, `kommunikál` 8, `telefonál` 5), goldens
- **Confidence**: consensus; `tendency` for new loans

### DER-VERB-004 Lexical doublets and meaning splits of denominal verbs
- **Statement**: Parallel `-z`/`-l` (and `-l`/`-ál`) formations on one base either coexist, are replaced by the `-z` form, or split in meaning; each existing verb is a lexicon entry.
- **Conditions**: base with two attested verbalisations
- **Output**: coexisting synonyms (`síel ~ sízik`, `bridzsel ~ bridzsezik`, `szörföl ~ szörfözik`); replacement (`pipál` -> `pipázik`, `vacsorál` -> `vacsorázik`); meaning split (`okoz` 'cause' / `okol` 'blame'; `soroz` 'enlist, serialise' / `sorol` 'list'; `szemez` 'eye up' / `szemel` 'pick'; `kertel` 'beat about the bush, fence' / `kertez` 'garden')
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries with sense
- **Interactions**: `DER-VERB-001`, `DER-LEX-004`
- **Sources**: [SMNy3 p. 120 (3.7)]
- **Validation**: attestation (`okoz` 17, `sorol` 6)
- **Confidence**: consensus

### DER-VERB-005 Harmony of loan verbalisers
- **Statement**: Loan verbs formed with `-(O)l`, `-(O)z`, `-íroz` may show antiharmony or counterharmony; the form is lexical and speaker-dependent (`PH-HARM-024`).
- **Conditions**: loan base, often without free noun use in Hungarian
- **Output**: listed forms; generator emits the harmonic form for unlisted new verbs
- **Examples**: antiharmony `brillíroz`, `fixíroz`, `síbol`, `slisszol`; counterharmony `csekkol`, `gründol`, `curükkol`, `stíröl`; harmonic `blöfföl`, `sztrájkol`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: verb entries
- **Interactions**: `PH-HARM-024`, `DER-VERB-003`
- **Sources**: [RST24 p. 165], [F13 theses §3.3]
- **Validation**: attestation
- **Confidence**: contested

### DER-VERB-006 -izál
- **Statement**: `-izál` forms transitive verbs 'make ADJ / treat as N' from foreign adjectives and nouns; it removes `-is` and shortens the preceding long vowel like `-itás`, is invariant back, and is blocked when an `-ál` verb corresponds to the adjective.
- **Conditions**:
  - bases: foreign adjectives (`modern`, `profán`, `szinkron`, `immun`, `stabil`, `nacionális`, `digitális`, `virtuális`, `mobilis`), nouns (`atom`, `bagatell`, `humor`, `politika` -> `politizál` with truncation)
  - `-is`-ending adjectives: `digitális` -> `digitalizál`, `virtuális` -> `virtualizál`
  - excluded: `-ózus` adjectives (`*morózusizál`); `-ív` adjectives whose base corresponds to an `-ál` verb (`#kreativizál`: `kreál`; `#dekorativizál`: `dekorál`; `#kommunikativizál`: `kommunikál`; `#adaptivizál`: `adaptál`)
  - harmony `B!` (`modernizálok`)
- **Output**: verb, usually transitive (intransitive: `humorizál`, `politizál`)
- **Examples**: `modernizál`, `profanizál`, `szinkronizál`, `immunizál`, `stabilizál`, `monopolizál`, `rivalizál`, `nacionalizál`, `digitalizál`, `mobilizál`, `atomizál`, `bagatellizál`
- **Exceptions**: listed
- **Predictability**: `rule` (foreign layer) + `lexical` (base membership)
- **Lexical input**: `foreign_latinate`; correspondence to `-ál` verbs
- **Interactions**: `DER-NOUN-017`, `DER-ADJ-023`, `PH-LEN-011`, `PH-HARM-020`
- **Sources**: [SMNy3 pp. 163-164 (4.3.9, rule 56)], [KVF98 p. 358 (873 b)]
- **Validation**: attestation (`politizál` 4, `szinkronizál` 0), goldens
- **Confidence**: consensus

### DER-VERB-007 -íroz and -ifikál
- **Statement**: `-íroz` (German `-ieren`) and `-ifikál` (Latin `-ificare`) occur only in lexicalised loan verbs; both are invariant back.
- **Conditions**: listed items; `-íroz` stems often antiharmonic (`DER-VERB-005`)
- **Output**: lexicon entries
- **Examples**: `-íroz`: `blamíroz`, `masíroz`, `brillíroz`, `fixíroz`, `kommandíroz`, `stafíroz`; `-ifikál`: `kodifikál`, `kvalifikál`, `diszkvalifikál`, `identifikál`, `specifikál`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `PH-HARM-020`, `DER-VERB-005`
- **Sources**: [SMNy3 p. 673 (Rebrus `-íroz`, `-ifikál` truncating)], [RST24 p. 165]
- **Validation**: goldens
- **Confidence**: consensus

### DER-VERB-008 -ít: deadjectival causatives and -sít
- **Statement**: `-ít` forms transitive verbs 'make ADJ'; it truncates the final vowel of vowel-final adjectives, takes harmony-transparent `í` with harmony from the untruncated base, and is replaced by `-sít` after adjectives in `ó`, `ú`, `ű` whose truncation is blocked.
- **Conditions**:
  - bases: dimension, colour (closed list, below), quality and human-trait adjectives (`sötét`, `széles`, `magas`, `mély`, `szép`, `vak`, `kemény`, `bolond`, `erős`); comparatives (`nagyobbít`, `kisebbít`, `hosszabbít`); `-(O)s` adjectives denoting saturation of a surface or space (`párásít`, `ködösít`, `füvesít`, `vizesít`); some privatives (`portalanít`, `lombtalanít`)
  - human-trait adjectives are regular: `bátorít` 119,953, `nemesít` 50,581, `fiatalít` 26,870, `butít` 12,454, `szelídít` 10,877, `okosít` 6,138, `szegényít` 1,809 [WC2, all forms of the stem]; unattested ones are potential words, not excluded (`#jámborít`, `#konokít`, `#fösvényít`, `#kacérít`, `#huncutít`, `#cingárít`)
  - colour: a closed list (`R`): `fehérít`, `pirosít`, `zöldít`, `barnít`, `szürkít`, `feketít`, `kékít`, `sárgít`, `vörösít` [WC2]; a colour outside it forms none, whatever its shape selects: `*bordósít` though `ó`-final adjectives take `-sít`, `*bézsít`, `*drappít`, `*lilít` (`lila` and `barna` differ in neither phonology nor stratum: `lilul` 307 : `lilít` 2, `barnul` 3,875 : `barnít` 456 [WC2])
  - excluded: `-i` adjectives (`*művészisít`)
  - vowel-final base: final vowel deleted (`fekete` -> `feketít`, `barna` -> `barnít`, `szőke` -> `szőkít`, `fakó` -> `fakít`, `szomorú` -> `szomorít`, `tiszta` -> `tisztít`, `sima` -> `simít`)
  - stem `bound(-ep)`: shortening (`nehéz` -> `nehezít`), v-stem (`bő` -> `bővít`); the epenthetic vowel is kept (`bátor` -> `bátorít`), dropped only in listed bound-root items (`piszkít`)
  - `-sít` instead: `karcsú` -> `karcsúsít`, `olcsó` -> `olcsósít`, `egyszerű` -> `egyszerűsít`, `állandó` -> `állandósít`; also on nouns 'make into N' (`parkosít`, `bálásít`, `DER-VERB-010`)
  - bound roots: `ind-ít` / `ind-ul`, `sik-ít`, `állít`, `szállít` (verbal bases, lexical)
  - harmony: following suffixes from the untruncated base (`tisztítok`, `simítok`, `békítek`, `hamisítok`)
  - vowel always long (`ORT-LEN-013`)
- **Output**: transitive verb, agentive subject; inflection class `VRB-STEM-004`
- **Examples**: listed
- **Exceptions**: `lila` -> `lilul` but `*lilít`; lexical `sárgít`, `barnít` stored (morphdb)
- **Predictability**: `rule` (form) + `lexical` (colour verbs) + `tendency` (unattested trait adjectives)
- **Lexical input**: the closed list of colour verbs (not carried, so the engine forms `-ít` on every colour adjective); `-(O)s` semantic subclass `saturation`
- **Interactions**: `DER-MODEL-007`, `DER-VERB-009` (parallel intransitive), `PH-HARM-019`, `VRB-STEM-004`, `VRB-CAUS-001`
- **Sources**: [SMNy3 pp. 162-163 (4.3.7, rule 53)], [SMNy3 p. 694 table 33 (`bátorít` non-triggering)], [RST12 pp. 501-503], [KVF98 pp. 358, 361 (871), (883)], [T04 p. 145], [AkH12 §34], [morphdb derivation README (ÍT lexical `sárgít`, `barnít`, `lilul/*lilít`)]
- **Validation**: attestation (`tisztít` 6, `indít` 12, `hosszabbít` 3, `bővít` 1, `vakít` 1, `bátorít` 4; `bátrít` unattested), goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-VERB-009 -Ul and -Odik: 'become ADJ'
- **Statement**: Inchoative verbs 'become ADJ' take `-ul/-ül` after monosyllabic (or truncated disyllabic vowel-final) adjectives and `-odik/-edik/-ödik` after polysyllabic adjectives; `-Ul` is unproductive (closed base class), `-Odik` is productive on saturation `-(O)s` adjectives.
- **Conditions**:
  - `-Ul`: monosyllabic adjectives (`zöldül`, `kékül`, `mélyül`, `szépül`, `vakul`, `bolondul`); disyllabic vowel-final adjectives with truncation (`barnul`, `sárgul`, `ferdül`, `szőkül`, `lassul`, `butul`, `bénul`); not with loans (`*drappul`, `*bézsül`, `*kekül`)
  - `-Odik`: polysyllabic adjectives (`keményedik`, `erősödik`, `laposodik`, `gazdagodik`, `sötétedik`); vowel-final ones truncated (`fekete` -> `feketedik`, `szomorú` -> `szomorodik`, `domború` -> `domborodik`, `szigorú` -> `szigorodik`, `gömbölyű` -> `gömbölyödik`); comparatives lexically (`nagyobbodik`, `kisebbedik`, `hosszabbodik`; `*magasabbodik`); `-(O)s` saturation adjectives (`párásodik`, `ködösödik`, `füvesedik`, `vizesedik`, not `*tavasodik`, `*csipkésedik`); privatives with `el-` lexically (`elsótlanodik` [WC2 low], `elbizonytalanodik` [WC2 4159], `elbátortalanodik`, `elerőtlenedik`) - a lexical list, not generated by rule (`DER-ADJ-011`)
  - `-Odik` also on some nouns of time (`tavaszodik`, `alkonyodik`, `reggeledik`, `hajnalodik`, `esteledik`)
  - excluded for `-Odik`: detailing and absolute adjectives (`*kacérodik`, `*konokodik`), loans (`*modernedik`, `*privátodik`), other derived adjectives (`*művészetiedik`, `*intézetbeliedik`)
  - exceptions to the syllable rule: `rövidül`, `(el)csendesül`, `könnyebbül`
  - doublets with meaning split: `(el)sötétül` / `(be)sötétedik`
  - harmony from the untruncated base; `-Odik` takes shortened stems (`nehezedik`) but keeps the epenthetic vowel (`bátorodik`, SMNy3 table 33)
  - verbal `-Ul` also on bound roots and verbs (`nyomul`, `vonul`, `merül`, `indul`, `gyógyul`), lexical
- **Output**: intransitive `-ik` (`-Odik`) or non-`-ik` (`-Ul`) verb; inflection per `VRB-CAUS-003`, `VRB-CAUS-004`
- **Examples**: listed
- **Exceptions**: listed; SMNy notes `-Ul` generalisations as a closed class; `lila` -> `lilul`
- **Predictability**: `rule` (syllable count, truncation) + `lexical` (closed `-Ul` class, exceptions)
- **Lexical input**: adjective classes; lexicon for `-Ul` members
- **Interactions**: `DER-MODEL-006`, `DER-MODEL-007`, `DER-VERB-008`, `VRB-CAUS-003`, `VRB-CAUS-004`, `PH-HARM`
- **Sources**: [SMNy3 pp. 116-117 (rule 29), pp. 159-162 (4.3.6, rules 51, 52)], [SMNy3 p. 673 (Rebrus `Odik` exceptions `tavaszodik` etc.)], [T04 p. 145], [KVF98 pp. 361-362 (884), (886)]
- **Validation**: attestation (`szépül` 3, `zöldül` 1, `sárgul` 1, `barnul` 2, `lassul` 7, `gyógyul` 8, `erősödik` 10, `gazdagodik` 6, `sötétedik` 4, `hosszabbodik` 2), goldens
- **Confidence**: consensus

### DER-VERB-010 -(O)sít and -(O)sOdik on ethnonyms and nouns
- **Statement**: With ethnonym bases `-(O)sít` 'make into ETHN' and `-(O)sOdik` 'become ETHN' are productive complex suffixes (mostly with prefix `el-`); on other nouns `-(O)sít` 'provide with N, make into N' is lexically restricted.
- **Conditions**:
  - ethnonym (non-lowering, mid linking vowel): `el+orosz+osít`, `el+német+esít`, `el+angol+osít`, `magyarosít`, `bantusít`, `baszkosít`, `finnesít`, `litvánosít`, `hopisít`; intransitive `baszkosodik`, `finnesedik`, `albánosodik`, `görögösödik`, `elmagyarosodik`
  - meaning is 'ETHN', not 'ETHN-ish' (distinct from `-(O)s` + `-ít`: `nedvesít` 'make wet')
  - nouns: `parkosít`, `villamosít`, `államosít`, `bálásít`, `szabványosít`; many analysable as `-(O)s` + `-ít`
- **Output**: verb
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `rule` (ethnonyms) + `lexical` (nouns)
- **Lexical input**: noun/adjective feature `ethnonym`
- **Interactions**: `DER-VERB-008`, `DER-VERB-009`, `DER-ADJ-005`
- **Sources**: [SMNy3 p. 163 (4.3.8, rules 54, 55)], [KVF98 p. 358 (871)]
- **Validation**: goldens
- **Confidence**: consensus

### DER-VERB-011 -(A)ll: 'consider too ADJ'
- **Statement**: `-ll`/`-all/-ell` forms transitive verbs 'consider ADJ, find too ADJ' from a small set of quantity and dimension adjectives (and lexically from nouns: `sarkall`); after consonants the linking vowel `a/e` alternates with its absence, and the vowelless form is degeminated to `-l-` (`kevesli ~ keveselli`).
- **Conditions**: bases `sok`, `kevés`, `drága`, `olcsó`, `nagy`, `kicsi`, `hosszú`, `rövid`, `kék` (intransitive `kékellik`), `sárga` (`sárgállik`); noun `sarok` (`sarkall`, epenthetic drop); bound stem (`DER-MODEL-006`)
- **Output**: `sokall`, `kevesell` (3sg definite `keveselli ~ kevesli`), `drágáll`, `kicsinyell`, `hosszall`, `rövidell`, `kékellik ~ kéklik`, `sárgállik`; `#olcsóll` (1 token) and `#nagyall` (0) are unattested potential members [WC2], and the MTA advisor knows neither
- **Examples**: listed
- **Exceptions**: all lexical
- **Predictability**: `lexical` (about 10 items)
- **Lexical input**: lexicon entries
- **Interactions**: `VRB-STEM-006` (geminate stems)
- **Sources**: [SMNy3 p. 673 (Rebrus `-(A)ll(ik)`: `keves-l-i ~ keves-ell-i`, `kék-l-ik ~ kék-ell-ik`, degemination)], [SMNy3 p. 694 table 33 (`sarkall`)]
- **Validation**: attestation (`kevesli` 1), goldens
- **Confidence**: consensus for the suffix; item list `tendency`

### DER-VERB-012 -kOdik and -(O)skOdik: 'act as N, behave ADJ'
- **Statement**: `-kodik/-kedik/-ködik` forms intransitive `-ik` verbs 'act as N (occupation, function)' and 'behave in an ADJ way'; the `s`-extended allomorph `-skodik` (after vowels) or `-oskodik/-askodik/-eskedik/-ösködik` is obligatory after vowels and `k`, `g`, usual after the other stops, and variable elsewhere.
- **Conditions**:
  - noun bases: occupation or function nouns (`nyelvészkedik`, `technikuskodik`, `óráskodik`, `kántorkodik`, `színészkedik`, `tanárkodik`, `menedzserkedik`); human-property adjective bases: privatives (`nyugtalankodik`, `ügyetlenkedik`) and `-(O)s` adjectives (`okoskodik`, `udvariaskodik`, `kedveskedik`), root adjectives lexically (`keménykedik`, `erősködik`)
  - after a vowel: `-skodik/-skedik/-sködik` with LVL (`bíróskodik`, `nevelősködik`, `katonáskodik`, `atyáskodik`)
  - after `k`, `g`: `-oskodik/-eskedik/-ösködik` obligatory (`mérnökösködik`, `elnökösködik`, `szónokoskodik`; morphdb excludes `-kodik` after `k`, `g`)
  - after `p t b d`: `-(O)skOdik` usual (`paposkodik`, `apátoskodik`), not excluded
  - after other consonants: `-kodik/-kedik/-ködik` (`tanárkodik`, `hősködik`)
  - variation after non-stops: `bankárkodik ~ bankároskodik`, `bébiszitterkedik ~ ?bébiszittereskedik`, `szponzorkodik ~ ?szponzoroskodik`, `dékánkodik ~ ?dékánoskodik`
  - adjective bases (privatives, `-(O)s` adjectives) end in `n` or `s` and take `-kOdik`; the domain is formal and productive: 556 `-(O)s` + `-kOdik` lemmas and 158 privative + `-kOdik` lemmas of at least 5 tokens [WC2] (`fontoskodik`, `bizonytalankodik`, `nagyoskodik`)
  - `-(O)skOdik` is `-(O)s` plus `-kOdik`: `katona` -> `katonás` -> `katonáskodik`, `mérnök` -> `mérnökös` -> `mérnökösködik`, `Petőfi` -> `petőfi` -> `petőfies` -> `petőfieskedik`
  - harmony `H3`; linking vowel of `-(O)skOdik` as `DER-MODEL-004` type `O` (lowering back bases `-askodik`; front rounded lowering bases `-eskedik`); `-(O)skOdik` takes the bound stem, `-kOdik` the free stem (morphdb synthetic vs analytic)
  - semantic: human property or role
  - further derivation `-Ás` (`katonáskodás`)
- **Output**: intransitive `-ik` verb; `-gAt` not allowed after it (`*legénykedget`)
- **Examples**: listed
- **Exceptions**: `-lkodik` (`buzgólkodik`) unproductive
- **Predictability**: `rule` (allomorph by final segment) + `tendency` (non-stop variation)
- **Lexical input**: noun feature `occupation` (bare `-kOdik` on nouns: `tanárkodik`); none for the adjective bases, which the final `s` or the privative suffix identifies
- **Interactions**: `DER-NOUN-003`, `DER-VERB-013`, `VRB-CAUS-003`
- **Sources**: [SMNy3 pp. 114, 155-156 (4.3.3, rules 43, 44; `s`-less variant excluded only after vowels and `k`, `g`)], [KVF98 pp. 358, 362 (872), (885)], [morphdb grammar AFF_KODIK]
- **Validation**: attestation (`okoskodik` 1, `katonáskodás` 1), goldens
- **Confidence**: consensus

### DER-VERB-013 -(O)gAt: frequentative and diminutive verbs
- **Statement**: `-(O)gAt` forms frequentative ('repeatedly V') verbs from punctual verbs and diminutive ('V a little, leisurely') verbs from durative verbs; it is productive; monosyllabic stems take a mid linking vowel, polysyllabic stems none.
- **Conditions**:
  - monosyllabic stem: `-ogat/-eget/-öget` (`adogat`, `váltogat`, `eszeget`, `tépeget`, `főzöget`, `kötöget`, `ütöget`, `lépeget`)
  - polysyllabic stem: `-gat/-get` (`olvasgat`, `sétálgat`, `beszélget`)
  - sz-d-v verbs: `sz`-stem (`teszeget`, `iszogat`, `eszeget`); v-stem verbs: `v`-stem (`növöget`, `jövöget`)
  - epenthetic verbs: free (vowelled) stem (`hímezget`, `udvarolgat`, `söpörget`; morphdb counts them as polysyllabic: `kotorgat`, `*kotrogat`)
  - frequentative reading: base denotes a single, momentary event or has a prefixed perfective partner (`nyitogat` / `kinyit`, `csókolgat` / `megcsókol`, `ütöget` / `megüt`, `üzenget`)
  - diminutive reading: base denotes a durative activity (`sétálgat`, `olvasgat`, `dolgozgat`, `jegyezget`)
  - intransitive new bases accepted (`tornázgat`, `számítógépezget`, `internetezget`, `emailezget`, `kólázgat`)
  - `g`-final stems: not attested with `-gat` in the grammar model (morphdb excludes)
  - argument structure unchanged
- **Output**: verb; inflection T-class of `-gat` stems (`VRB-STEM-003`)
- **Examples**: listed
- **Exceptions**: `DER-VERB-014`
- **Predictability**: `rule`
- **Lexical input**: verb aspectual class (`punctual`/`durative`), stem class
- **Interactions**: `DER-VERB-014`, `VRB-STEM-011`, `VRB-STEM-010`, `VRB-STEM-007`
- **Sources**: [SMNy3 pp. 156-158 (4.3.4, rules 46, 47)], [SMNy3 p. 672 (Rebrus `-(O)gAt`)], [KVF98 p. 360 (878)], [morphdb grammar AFF_GAT], [morphdb derivation README (GAT: `kotorgat`, `söpröget` lexical)]
- **Validation**: attestation (`nézeget` 1), goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-VERB-014 Restrictions and blocking of -(O)gAt
- **Statement**: `-(O)gAt` needs a base with a dynamic reading; it does not attach to reflexive-medial derived verbs, and verbs whose frequentative is lexicalised with another suffix weakly block it; lexicalised `-gAt` verbs are entries.
- **Conditions**:
  - a stative verb takes it in its dynamic reading (`szeret` 'caress' -> `szeretget`, `szeretgetni` 1,094; `emlékezik` 'reminisce' -> `emlékezget` 70; `ismer` 'get to know' -> `ismerget` 150; `tud` -> `tudogat` 181); a verb with no dynamic reading takes none (`*birtokolgat` 2, `*tartalmazgat` 4, `*gyűlölget` 1) [WC2, all forms of the stem]
  - after `-Ódik`, `-kOdik`, `-kÓdik`, `-kOzik`, `-kÓzik`, `-Odik` excluded: `*legénykedget`, `*buzgólkodgat`, `*gazdagodgat`, `*lelkesedget`, `*törülközget`, `*ölelkezget`; a monosyllabic root (`szed` -> `szedeget`) and a denominal `-(O)z` verb (`emlék` -> `emlékezik` -> `emlékezget`) end in the same letters without the suffix
  - weakly blocked by frequent lexical frequentatives: `#szaladgat` (`szaladgál`), `#harapgat` (`harapdál`), `#heverget` (`heverészik`; 29), `#ülöget` (`üldögél`), `#állogat` (`álldogál`; 168), `#hullogat` (`hulldogál`), `#hajlogat` (`hajladozik`) [WC2]
  - not blocked by less frequent synonyms: `lopogat` / `lopkod` / `lopdos`; `rugdal` / `rúgdos`; `tépeget` / `tépdel` / `tépked` / `tépdes`
  - lexicalised `-gAt` verbs with non-compositional meaning: `látogat`, `beszélget`, `mosogat`, `hallgat`, `fosztogat`
- **Output**: rejection or blocked flag
- **Examples**: `szeret` -> `szeretget`; `emlékezik` -> `emlékezget`
- **Exceptions**: -
- **Predictability**: `rule` (medial suffixes) + `tendency` (dynamic reading, blocking)
- **Lexical input**: dynamic vs stative reading (not carried; the engine checks the medial suffixes only); blocking links
- **Interactions**: `DER-VERB-013`, `DER-LEX-002`, `DER-VERB-017`, `DER-VERB-001`
- **Sources**: [SMNy3 pp. 107, 115, 157-158], [KVF98 p. 360 (879)], [WC2]
- **Validation**: attestation (`szaladgál` 4, `üldögél` 4, `álldogál` 2; counts above), goldens
- **Confidence**: consensus for the medial exclusion; the dynamic-reading condition replaces the sources' stative exclusion on [WC2] evidence

### DER-VERB-015 -Ódik: medio-passive verbs
- **Statement**: `-ódik/-ődik` forms intransitive medio-passive or non-agentive verbs from transitive verbs on the vowelless (syncopated) stem; it is moderately productive and its inflection is owned by `VRB-CAUS-003`.
- **Conditions**: transitive base (`ismétel`, `üt`, `érez`, `mos`, `ír`, `köt`, `nyit`, `csuk`); epenthetic stems vowelless (`ismétlődik`, `érződik`); prefixed forms common (`elintéződik`)
- **Output**: `-ik` verb; `-Ás` derivative (`ütődés`), not `-gAt`
- **Examples**: `ismétel` -> `ismétlődik`; `üt` -> `ütődik`; `érez` -> `érződik`; `mos` -> `mosódik`; `ír` -> `íródik`; `köt` -> `kötődik`
- **Exceptions**: interpretation reflexive vs passive is lexical
- **Predictability**: `rule` (form) + `tendency` (existence)
- **Lexical input**: verb transitivity; epenthetic stem
- **Interactions**: `VRB-CAUS-003`, `DER-VERB-014`
- **Sources**: [KVF98 pp. 360-361 (881)], [SMNy3 p. 672 (Rebrus `-Ódik`)], [morphdb grammar AFF_ÓDIK]
- **Validation**: attestation (`ismétlődik` 6), oracle:hunmorph-foma
- **Confidence**: consensus; productivity `contested` (KVF98 doubtful)

### DER-VERB-016 Reflexive -(A)kOdik, -(A)kOzik, -kÓdik, -kÓzik
- **Statement**: The reflexive suffixes are unproductive: monosyllabic bases take the linking vowel form, polysyllabic bases the vowelless form, some the long-vowel form.
- **Conditions**:
  - monosyllabic: `-akodik/-ekedik` (`mosakodik`, `fenekedik`, `törekedik`, `verekedik`), `-akozik/-ekezik` (same distribution per Rebrus, no item given)
  - polysyllabic: `-kodik/-kedik/-ködik`, `-kozik/-kezik/-közik` (`fésülködik`, `öltözködik`, `törülközik`, `ölelkezik`)
  - long vowel: `-kódik/-kődik`, `-kózik/-kőzik` (`bánkódik`)
  - some have `sz` doublets (`mosakszik`, `VRB-STEM-012`)
- **Output**: `-ik` verbs (lexical)
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `VRB-CAUS-003`, `VRB-STEM-012`
- **Sources**: [SMNy3 p. 672 (Rebrus reflexive/medial)], [KVF98 pp. 360-361 (881)]
- **Validation**: attestation (`öltözködik` 3), goldens
- **Confidence**: consensus

### DER-VERB-017 Unproductive aspectual verb suffixes
- **Statement**: Frequentative, continuative and momentaneous suffixes other than `-(O)gAt` are unproductive; their verbs are lexicon entries and block `-(O)gAt` (`DER-VERB-014`).
- **Conditions**: listed suffixes and items
- **Output**: lexicon entries with segmentation marked lexicalised
- **Examples**: `-dogál/-degél/-dögél`: `álldogál`, `üldögél`, `hulldogál`; `-dos/-des/-dös`: `rugdos`, `lökdös`, `vagdos`, `csipdes`, `bökdös`, `lopdos`; `-dal/-del`, `-dál/-dél`: `tépdel`, `rugdal`, `harapdál`; `-gál/-gél`: `szaladgál`; `-kál/-kél`: `járkál`; `-ál` frequentative: `ugrál`, `dobál`; `-kod/-ked` (non-reflexive): `lopkod`, `kapkod`; `-(A)dOz(ik)`: `hajladozik`; `-ong/-eng/-öng`: `zsibong`, `mereng`, `dülöng`; `-ész(ik)`: `heverészik`; `-int/-ent`: `legyint`, `kacsint`, `csippent`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `DER-VERB-014`, `DER-LEX-005`
- **Sources**: [SMNy3 pp. 113, 157-158, 672 (Rebrus: `-Ong (unproductive mark)`, `-Oz(ik) (unproductive mark)`, `-(O)g (unproductive mark)`, `-AN (unproductive mark)`, `-int (unproductive mark)`)], [KVF98 p. 360], [HUWIKI-SZOKEPZES (`rugdos`, `bökdös`)]
- **Validation**: attestation (`ugrál` 6, `járkál` 4, `kacsint` 2, `legyint` 3, `mereng` 1), goldens
- **Confidence**: consensus

### DER-VERB-018 Onomatopoeic -(O)g and -An
- **Statement**: Durative `-og/-eg/-ög` and momentaneous `-an/-en` form verbs of sound emission from monosyllabic consonant-final onomatopoeic roots; they are productive on new or nonce roots, with the momentaneous form usually geminating the final consonant.
- **Conditions**:
  - root: monosyllabic, consonant-final, sound-imitating (possibly nonce)
  - durative: root + `-og/-eg/-ög` or `-g` after a nasal (`cseng`, `zsong`, `bong`)
  - momentaneous: root with geminated final consonant + `-an/-en` (`koppan`, `puffan`, `zizzen`, `rezzen`, `dörren`, `pattan`)
  - pairs: `kopog` / `koppan`, `pufog` / `puffan`, `zizeg` / `zizzen`, `rezeg` / `rezzen`, `dörög` / `dörren`, `pattog` / `pattan`, `serceg` / `sercen`
  - causative partner `-int` lexical (`koppint`)
- **Output**: intransitive verbs
- **Examples**: listed; nonce `zuttyog` / `zuttyan`, `bummog` / `bumman`, `bizzeg` / `bizzen`
- **Exceptions**: -
- **Predictability**: `rule` (on onomatopoeic roots)
- **Lexical input**: root flag `onomatopoeic`
- **Interactions**: `DER-VERB-017`
- **Sources**: [SMNy3 pp. 158-159 (4.3.5, rules 48, 49)], [SMNy3 p. 672 (gemination)]
- **Validation**: attestation (`kopog` 4, `koppan` 1, `csattan` 3), goldens
- **Confidence**: consensus

### DER-VERB-019 Lexical transitivity pairs: -Ad/-Aszt, -Ul/-ít, -Ul/-it, -ik/-t
- **Statement**: Intransitive-transitive verb pairs built on bound roots with `-ad/-ed` / `-aszt/-eszt`, `-ul/-ül` / `-ít`, `-dul/-dít` are lexical; segmentation is recorded for analysis only.
- **Conditions**: listed pairs
- **Output**: lexicon entries with pair links
- **Examples**: `fárad` / `fáraszt`; `hervad` / `hervaszt`; `ijed` / `ijeszt`; `alvad` / `alvaszt`; `süllyed` / `süllyeszt`; `indul` / `indít`; `gyógyul` / `gyógyít`; `épül` / `épít`; `fordul` / `fordít`; `mozdul` / `mozdít`; `pezsdül` / `pezsdít`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: pair links
- **Interactions**: `VRB-STEM-005` (`-ad/-ed` stems), `DER-VERB-008`
- **Sources**: [SMNy3 p. 672 (Rebrus `-Ad (unproductive mark)` pairs with `-Aszt`)], [SMNy3 pp. 181-195 (Komlósy 5.2.1 medial pairs)], [AkH12 §34, §35 (`-dít`, `-dul`)]
- **Validation**: attestation (`fárad` 5, `ijed` 3, `indul` 17, `indít` 12), goldens
- **Confidence**: consensus

### DER-VERB-020 Causative and passive (pointer)
- **Statement**: Causative `-tAt/-At` and archaic passive `-tAtik/-Atik` are derivational verb suffixes whose allomorph selection and inflection are owned by `VRB-CAUS-001`..`VRB-CAUS-004`; this rule fixes their record values and restrictions.
- **Conditions**:
  - record: V -> V; harmony `H2`; no LVL; productive (`P`) for agentive verbs
  - excluded bases: medial verbs, involuntary agentive verbs, stative experiencer verbs (`*tartalmaztat`; literary and theological `?létezteti` 20 [WC2])
  - lexicalised causatives with shifted meaning are entries (`biztat`, `éltet`, `vágtat`, `folytat`, `nyomtat`, `törtet`; regular partners `járat`, `vágat`, `nyomat`)
  - passive: transitive bases only, 3rd person; register **archaic and legal, but current** - `adatik` 29,912, `mondatik` 1,962, `tartatik` 1,543, `említtetik` 1,096, `hivatik` 192 [WC2]; over all forms 987 `-tAtik` lemmas have at least 5 tokens (`foglaltatik` 16,038, `megnyittatik` 1,071, `oltatik` 48, `kinyittatik` 25), so it is productive in that register. The suffix is labelled by register rather than excluded
  - passive allomorph: the causative's (`VRB-CAUS-001`) plus `-ik`: `adatik`, `oltatik`, `foglaltatik`, `taníttatik`
- **Output**: see `VRB-CAUS`
- **Examples**: `olvas` -> `olvastat`; `ír` -> `írat`; `ül` -> `ültet`; `ír` -> `íratik`
- **Exceptions**: listed
- **Predictability**: `rule` + `lexical`
- **Lexical input**: verb semantic class
- **Interactions**: `VRB-CAUS-001`, `VRB-CAUS-002`, `VRB-CAUS-004`
- **Sources**: [SMNy3 pp. 106, 672-673 (Rebrus `-(t)At` groups i-v)], [SMNy3 p. 122 table 36 (`-(t)At`, `-(t)At[ik]` listed as productive; arbitrated correct)], [SMNy3 pp. 203-211 (Komlósy 5.4.1 excluded groups)], [KVF98 pp. 359-361 (876), (882)], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens; attestation (five `-(t)Atik` lemmas above the floor)
- **Confidence**: consensus; the register of the passive is `measured` - it is current in legal and archaic register, which arbitrates `OQ-06-016` in favour of SMNy3 table 36 against the chapter's earlier exclusion. The third SMNy3 contradiction, verbal `-Ul` (table 36 vs p. 161), is **insufficient evidence**: the contrast is about epenthetic behaviour rather than existence and was not measured

### DER-VERB-021 Potential -hAt (pointer)
- **Statement**: `-hAt` is inflectional in this rulebook (`VRB-MODEL-003`); it never feeds derivation except through the unanalysed suffixes `-hatÓ`, `-hatatlan`, `-hatnék`.
- **Conditions**: `-hAt` verb form
- **Output**: `*olvashatás`, `*olvashatandó`, `*olvashatott (könyv)`; `olvasható`, `olvashatatlan`, `olvashatnék`
- **Examples**: listed
- **Exceptions**: lexicalised `láthatás` (legal term) is an entry
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-MODEL-003`, `VRB-POT-003`, `DER-ADJ-019`, `DER-ADJ-020`
- **Sources**: [SMNy3 pp. 124-125], [KVF98 p. 359 (875)]
- **Validation**: goldens
- **Confidence**: contested (`VRB-MODEL-003`)

### DER-VERB-022 Denominal -Ász(ik) 'hunt, gather N'
- **Statement**: `-ász(ik)/-ész(ik)` forms verbs 'hunt or gather N' from animal and plant nouns; unproductive; several convert to occupation nouns (`DER-NOUN-013`).
- **Conditions**: listed items; bound stem (`madár` -> `madarász`, `nyúl` -> `nyulász`, `eper` -> `eprész`, `egér` -> `egerész`)
- **Output**: `-ik` verb (`halászik`, `vadászik`, `gombászik`, `horgászik`); noun by conversion
- **Examples**: `hal` -> `halászik`, `halász`; `vad` -> `vadászik`, `vadász`; `lepke` -> `lepkészik`, `lepkész`; `rák` -> `rákászik`; `sólyom` -> `solymász`; `agár` -> `agarász`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `DER-NOUN-013`, `DER-MODEL-015`
- **Sources**: [SMNy3 p. 168], [SMNy3 p. 673 (Rebrus `Ász(ik)` truncating)]
- **Validation**: attestation (`vadászik` 6, `horgászik` 1)
- **Confidence**: consensus

## DER-ADV

### DER-ADV-001 -An: domain of the default manner adverb suffix
- **Statement**: `-An` forms manner adverbs from adjectives of all kinds (root, participial, `-(O)s`, `-(j)Ú`, comparative), **including `-i` and `-beli` adjectives** (400 `-i` lemmas above 20 tokens, 270,605 tokens: `kíváncsian`, `mesterien`, `drámaian`, `költőien`, `mesebelien`), but not `-nyi` adjectives or privatives; nouns do not take it; its surface forms are owned by `NOM-ADJ-010`.
- **Conditions**:
  - allowed bases: root adjectives (`szép`, `meleg`, `vidám`), `-Ó` and `-(O)tt` participial adjectives (`meglepően`, `fáradtan`), `-(O)s` (`alaposan`), `-(j)Ú` (`egyhangúan`), `-szerű` (`észszerűen`), comparatives (`gyorsabban`, `alaposabban`), `-hAtÓ` (`láthatóan`)
  - allowed, measured: `-i` adjectives - 4,868 distinct `-i` lemmas have an `-An` adverb (282,678 tokens), of which **400 above a frequency floor of 20** (270,605 tokens), headed by `kíváncsian` 66,536, `forradalmian` 23,999, `mesterien` 18,727, `drámaian` 18,682, `pokolian` 6,622, `istenien` 5,663, `egyedien` 5,013, `művészien` 4,863, `érzékien` 3,711, `ördögien` 2,727, `gyermekien` 2,588, `költőien` 2,524, `festőien` 1,854, `emberien` 1,675, `szolgaian` 1,568, `barátian` 1,305 [WC2]. This is 100x more frequent than `-lAg` on the same bases (120 lemmas, 2,808 tokens). `-beli` likewise takes both (`mesebelien` 324, `vérbelien` 57 against `számbelileg` 957)
  - excluded (no productive formation): `-nyi`; privative adjectives take `-Ul` (`DER-ADV-003`)
  - disjunctive rule order: `-lAg` conditions (`DER-ADV-004`), then `-Ul` conditions (`DER-ADV-003`), else `-An` (`DER-ADV-005`)
- **Output**: adverb (terminal)
- **Examples**: `vidám` -> `vidáman`; `meleg` -> `melegen`; `alapos` -> `alaposan`; `gyorsabb` -> `gyorsabban`; `szomorú` -> `szomorúan`; `meglepő` -> `meglepően`; `fáradt` -> `fáradtan`
- **Exceptions**: lexicalised: `hamarosan`, `korán`, `gyakran`, `újonnan`, `jócskán`, `hosszan`, `titkon`; `jó` -> `jól`
- **Predictability**: `rule`, with the `-i` restriction removed
- **Lexical input**: adjective morphology
- **Interactions**: `NOM-ADJ-010` (forms), `DER-ADV-002`, `DER-ADV-003`, `DER-ADV-004`, `DER-ADV-005`
- **Sources**: [SMNy3 p. 164 (4.4.1, rule 58)], [KVF98 pp. 371-372 (917)], [SMNy3 p. 676 (Rebrus `-An}` terminal)], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: attestation (`hamarosan` 18, `gyakran` 20, `korán` 14, `újonnan` 15; type count over `[_Manner/Adv]` with an `-i` lemma, 400 lemmas above freq 20; English strings the pattern also catches - `russian`, `italian`, `dorian`, `lucian`, `varian` - excluded by inspection of the top 25, where they account for about 6% of tokens), goldens
- **Confidence**: `measured`; `OQ-06-011` resolved in KVF98's favour against SMNy3 - `-An` is productive on `-i` adjectives and generation should be enabled

### DER-ADV-002 -An: vowel choice, truncation and stem alternants (summary for derivation)
- **Statement**: The adverb vowel after consonant-final adjectives follows the adjective's lowering value (low `a/e` for lowering, mid `o/ö` for non-lowering), the suffix selects the bound stem, and a closed set of `ú/ű`-final adjectives truncate; full form rules in `NOM-ADJ-010`.
- **Conditions**:
  - lowering adjective: `-an/-en` (`pirosan`, `halkan`, `tágan`); non-lowering: `-on` (`nagyon`, `vakon`, `gazdagon`, `szabadon`, `vastagon`); front unrounded: `-en` either way
  - bound stem: shortening (`nehéz` -> `nehezen`, `kevés` -> `kevesen`), epenthetic (`bátor` -> `bátran`), v-stem (`bő` -> `bőven`, `hű` -> `híven ~ hűen`)
  - lexical truncation of `ú/ű`: `hosszú` -> `hosszan`, `lassú` -> `lassan`, `ifjú` -> `ifjan`, `könnyű` -> `könnyen`; others keep the vowel (`szomorúan`, `egyszerűen`)
  - lexical vowel-final adjectives in `ó/ő` take `-n` (`olcsón`, `későn`); participles take `-an/-en` (`bántóan`)
- **Output**: see `NOM-ADJ-010`
- **Examples**: listed
- **Exceptions**: `boldog` -> `boldogan` (`NOM-ADJ-010`)
- **Predictability**: `rule` + `lexical` (truncating set of 4, `boldogan`)
- **Lexical input**: `lowering`, alternants
- **Interactions**: `NOM-ADJ-010`, `NOM-ADJ-008`, `DER-MODEL-010`
- **Sources**: [T04 pp. 155-156], [morphdb derivation README (AN: `vakon`, `gazdagon`, `middle_an` `szabadon`)], [SMNy3 p. 164]
- **Validation**: attestation (`vakon` 9, `gazdagon` 10, `vastagon` 9, `tágan` 5, `hosszan` 14, `lassan` 18, `ifjan` 1, `könnyen` 19, `bőven` 14, `híven` 11, `hűen` 9, `bátran` 14, `nehezen` 17)
- **Confidence**: consensus

### DER-ADV-003 -Ul: privative and language adverbs
- **Statement**: `-ul/-ül` is the productive adverb suffix for privative adjectives of every base type and for language names (including `-i` language adjectives); elsewhere it is lexical; surface forms in `NOM-ADJ-011`.
- **Conditions**:
  - privative adjective (`-tAlAn`, `-tlAn`, `-AtlAn`, `-hAtAtlAn`): obligatory `-Ul` (`bátortalanul`, `vigyázatlanul`, `terméketlenül`, `feltétlenül`, `elkerülhetetlenül`)
  - language name: `angolul`, `franciául` (LVL), `szuahéliül`, `máltaiul` (after `-i`)
  - not with `-Ó` adjectives (`*olvashatóul`, `*mérhetőül`)
  - lexical: `rosszul`, `vadul`, `csúful`, `orvul`, `józanul`, `egyedül`, `bolondul`, `gonoszul`, `idegenül`; doublet `hirtelen ~ hirtelenül`
  - distinct from the essive case `-Ul` on nouns (`jutalmul`, `feleségül`, `NOM-CASE-009`)
- **Output**: adverb (terminal)
- **Examples**: listed
- **Exceptions**: listed lexical items
- **Predictability**: `rule` (two classes) + `lexical`
- **Lexical input**: adjective features `privative`, `language`
- **Interactions**: `NOM-ADJ-011`, `NOM-CASE-009`, `DER-ADJ-011`
- **Sources**: [SMNy3 pp. 114, 164-165 (4.4.2, rule 60)], [KVF98 p. 372 (917 d-e), (918)]
- **Validation**: attestation (`bátortalanul` 2, `feltétlenül` 16, `tudatlanul` 2, `angolul` 14, `franciául` 9, `rosszul` 16, `vadul` 8, `orvul` 2, `egyedül` 18, `hirtelenül` 1)
- **Confidence**: consensus

### DER-ADV-004 -lAg: domains
- **Statement**: `-lag/-leg` is productive in the standard language only on `-i` adjectives of disciplines and abstract artistic notions ('from the point of view of'), spreading in speech to other `-i` adjectives and potential `-i` forms; it also forms adverbs from `-Ó` participles and `-Ú` adjectives, and a closed set from nouns and adjectives.
- **Conditions**:
  - standard productive: `-i` adjectives from disciplines and arts (`logikailag`, `nyelvészetileg`, `irodalmilag`, `zeneileg`, `gazdaságilag`, `gyakorlatilag`)
  - spoken extension: occupational and institutional `-i` adjectives (`orvosilag`, `jogászilag`, `tanárilag`, `tanszékileg`, `színházilag`), potential `-i` forms of feature and product nouns (`méretileg`, `színileg`, `szekrényileg`, `újságilag`)
  - present participles (22 of 46 in Rebrus' count), productive in the manner reading: 399 `-Ó` + `-lAg` lemmas of at least 20 tokens [WC2] (`visszamenőleg`, `hallgatólag` 277, `láthatólag`, `serkentőleg` 2,519, `bátorítólag` 618, `gyógyítólag` 505); `-Ú` adjectives, lexical (73 lemmas of at least 20 tokens, mostly `-szerű`): `egyhangúlag`, `sajátkezűleg`, `valószínűleg`, `természetszerűleg`
  - lexical (nouns): `aránylag`, `viszonylag`, `egyetemleg`, `esetleg`, `jelenleg`, `névleg`, `színleg`, `tettleg`, `tényleg`, `végleg`; (adjectives): `főleg`, `mellesleg`, `némileg`, `utólag`, `újólag`, `pillanatnyilag`
  - `-beli` adjectives: productive (`számbelileg` 957, `időbelileg` 925, `térbelileg` 803, `szóbelileg` 378, `ténybelileg` 212, `hitbelileg` 84, `pénzbelileg` 80, `írásbelileg` 43 - 8 lemmas above the floor [WC2]); the earlier exclusion is removed
  - not with place and time `-i` adjectives in the standard (`*egerszegileg`; colloquial `?napilag` 87 [WC2])
  - harmony `H2`; no LVL issue (bases end in `i`, `ó/ő`, `ú/ű` or consonant)
  - further derivation possible: `-(O)s` (`utólagos`, `esetleges`, `viszonylagos`), then `-An`, `-sÁg`
  - meaning contrast with `-An`: `egyhangúlag` 'unanimously' vs `egyhangúan` 'monotonously'
- **Output**: adverb (not terminal)
- **Examples**: listed
- **Exceptions**: listed
- **Predictability**: `rule` (discipline domain) + `lexical`
- **Lexical input**: `-i` base semantic class
- **Interactions**: `NOM-ADJ-012`, `DER-ADJ-001`, `DER-ADV-005`
- **Sources**: [SMNy3 pp. 141-142, 165-166 (4.4.3, rule 61)], [SMNy3 p. 125 (allows `-lAg` after `-beli`; arbitrated correct)], [SMNy3 p. 676 (Rebrus `-lAg` {46}: {22} `-Ó`, {5} `-i`, {3} `-Ű`)], [KVF98 pp. 371-372 (919)], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: attestation (`számbelileg` 957, `időbelileg` 925, `térbelileg` 803 [WC2]; `logikailag` 6, `zeneileg` 7, `gazdaságilag` 11, `orvosilag` 6, `egyhangúlag` 11, `egyhangúan` 6, `valószínűleg` 19, `visszamenőleg` 9, `utólagos` 13, `esetleges` 17)
- **Confidence**: consensus; spoken extension `tendency`; `measured` for `-lAg` after `-beli` (`OQ-06-016`, SMNy3 p. 125 right)

### DER-ADV-005 Ordering of the three adverb suffixes
- **Statement**: For an adjective base the adverb suffix is chosen by disjunctive rules in the order `-lAg`, `-Ul`, `-An`; `-beli` and `-nyi` adjectives have no productive adverb.
- **Conditions**:
  1. base in `-i` of a discipline or art: `-lAg`
  2. base privative or language name: `-Ul`
  3. base in `-nyi`: none. `-beli` bases take both `-An` and `-lAg` (`mesebelien` 324, `vérbelien` 57; `számbelileg` 957, `időbelileg` 925, `térbelileg` 803 [WC2]), so rule 3 no longer excludes them
  4. lexical entry for the base (`jól`, `rosszul`, `nagyon`, `hosszan`)
  5. else: `-An`
- **Output**: one adverb (plus stored lexical doublets)
- **Examples**: `zenei` -> `zeneileg`; `bátortalan` -> `bátortalanul`; `angol` -> `angolul`; `szép` -> `szépen`; `osztálybeli` -> none
- **Exceptions**: lexical doublets with meaning split (`egyhangúlag` / `egyhangúan`)
- **Predictability**: `rule`
- **Lexical input**: as the component rules
- **Interactions**: `DER-ADV-001`, `DER-ADV-003`, `DER-ADV-004`
- **Sources**: [SMNy3 p. 166 (rule 62; no productive adverb from `-beli`, `-nyi`)], [SMNy3 p. 125 (lists `-lAg` on `-i` and `-beli` adjectives)], [WC2], [ATT evidence/corpus-variants.md 2026-09-16]
- **Validation**: goldens; attestation (8 `-belileg` lemmas above the floor)
- **Confidence**: consensus; the `-beli` + `-lAg` conflict is **arbitrated in favour of SMNy3 p. 125** against pp. 165-166 (`OQ-06-016`): the formation is productive, and the chapter now follows p. 125 for `-beli`

### DER-ADV-006 -képp(en)
- **Statement**: `-képp ~ -képpen` ('by way of, as') is a lexically restricted formal-case-like suffix on nouns and pronouns; forms in `NOM-CASE-020`; `-képpen` may take `-i`, `-képp` is terminal.
- **Conditions**: bases: nouns `ajándék`, `büntetés`, `válasz`, `példa`, `fizetség`; pronouns `más`, `e` (`ekképp`); lexicalised `tulajdonképpen`, `voltaképpen`; invariant (no back variant `*-kappan`)
- **Output**: adverb; `-képpeni` adjective (`tulajdonképpeni`)
- **Examples**: `ajándékképpen`, `büntetésképpen`, `válaszképpen`, `másképp ~ másképpen`, `ekképp`, `tulajdonképpen` -> `tulajdonképpeni`, `voltaképpen`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: stored forms
- **Interactions**: `NOM-CASE-020`, `DER-ADJ-001`
- **Sources**: [AkH12 §37], [SMNy3 p. 676 (Rebrus `]-képp}`, `]-képpen`)]
- **Validation**: attestation (`tulajdonképpeni` 9, `másképp` 14, `ekképp` 4, `válaszképpen` 1)
- **Confidence**: consensus

### DER-ADV-007 -kor
- **Statement**: `-kor` ('at time') forms time adverbs from time nouns and numerals; forms in `NOM-CASE-017`; it is analytic, invariant back and may be followed by `-i`.
- **Conditions**: base: event and day-part nouns, clock times (`NOM-CASE-017`); `-kori` adjective (`hajnalkori`, `ötkori`); distinguish compounds with the noun `kor` 'age, era' (`középkor`, `gyerekkor`, `reformkor`), whose `-i` derivatives are `középkori`, `gyerekkori`
- **Output**: adverb; `-kori` adjective
- **Examples**: `karácsonykor`, `éjfélkor`, `ötkor`, `órakor` (no LVL); `ötkori`; `középkori` (compound + `-i`)
- **Exceptions**: -
- **Predictability**: `rule` (form) + `lexical` (licensing)
- **Lexical input**: `time_noun`; compound lexicon for `kor` compounds
- **Interactions**: `NOM-CASE-017`, `DER-ADJ-001`, `DER-COMP-022`
- **Sources**: [SMNy3 p. 677 (`]-kor}`)], [KVF98 pp. 370-371 (915)]
- **Validation**: attestation (`karácsonykor` 11, `éjfélkor` 8, `ötkor` 6, `középkori` 14, `gyerekkori` 10)
- **Confidence**: consensus

### DER-ADV-008 -(O)nként, -(O)ntA, -(O)stUl
- **Statement**: Distributive `-nként`, distributive-temporal `-ntA` and sociative `-stUl` are case-like derivational suffixes; forms in `NOM-CASE-018`, `NOM-CASE-021`, `NOM-CASE-019`; only `-nként` may be followed by `-i`.
- **Conditions**: see owning rules; `-nkénti` adjective (`fejenkénti`, `óránkénti`); `-ntA` and `-stUl` terminal; simple numeral + time noun + `-nként` or `-nta/-nte` is solid (`kéthetenként`, `háromhavonta`, `ötévente`), apart with complex members or digits (`tizenöt naponként`, `5 évente`; `ORT-TOG-023`)
- **Output**: adverbs
- **Examples**: `fejenként` -> `fejenkénti`; `hetente`; `havonta` (mid vowel despite lowering `hav-`, lexical); `családostul`
- **Exceptions**: `havonta`, `havonként` mid vowel (`DER-MODEL-004`)
- **Predictability**: `rule` (forms, `NOM-CASE-018`, `NOM-CASE-019`), `lexical` (`NOM-CASE-021`, `havonta`)
- **Lexical input**: as owning rules
- **Interactions**: `NOM-CASE-018`, `NOM-CASE-019`, `NOM-CASE-021`, `DER-ADJ-001`
- **Sources**: [SMNy3 pp. 668-669, 676-677], [AkH12 §117], [morphdb derivation README (NTA: `havonta` lexical)]
- **Validation**: attestation (`fejenkénti` 1, `havonta` 15, `havonként` 4, `havanta` unattested)
- **Confidence**: consensus

### DER-ADV-009 Adverbs from verbs, nouns and numerals (pointers)
- **Statement**: Converbs in `-vA` (`VRB-ADVP`), essive-formal `-ként` (`NOM-CASE-008`), multiplicative `-szOr` and aggregate numeral `-An` (`PND`) are not modelled as derivation; a converb is reclassified as an adverb only as a lexicon entry.
- **Conditions**: lexicalised converb adverbs: `fordítva` 'conversely', `kivéve` 'except', `állva`; postposition-like converbs `kezdve`, `fogva`, `múlva`, `nézve` (`PND`)
- **Output**: lexicon entries
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `VRB-ADVP`, `NOM-CASE-008`, `PND`
- **Sources**: [KVF98 pp. 370-371 (915), (916), p. 374 (924)]
- **Validation**: goldens
- **Confidence**: consensus

### DER-ADV-010 Terminal status of derived adverbs
- **Statement**: Adverbs formed with `-An`, `-Ul`, `-képp`, `-stUl`, `-ntA` accept no further suffix; `-lAg`, `-képpen`, `-kor`, `-nként`, `-szOr` adverbs accept listed adjective-forming suffixes.
- **Conditions**:
  - terminal: `-An`, `-Ul`, `-képp`, `-stUl`, `-ntA`
  - followed by `-i`: `-képpen`, `-kor`, `-nként`, `-szOr` (`tulajdonképpeni`, `hajnalkori`, `fejenkénti`, `nyolcszori`)
  - followed by `-(O)s`: `-lAg` (`utólagos`, `viszonylagos`), `-szOr` (`ötszörös`)
  - lexicalised adverbs from inflected forms may take `-i` (`nagybani`, `mostani`, `ottani`)
- **Output**: morphotactic filter
- **Examples**: `*szépenes`; `*angolului`; `utólagosan`; `ötszörösen`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-MODEL-003`
- **Sources**: [SMNy3 pp. 113 (derived ADV not further derivable), 668-669 (criterion 4b), 675-676]
- **Validation**: goldens
- **Confidence**: consensus

## DER-DIM

### DER-DIM-001 Choosing -kA or -(O)cskA
- **Statement**: The diminutive suffixes `-ka/-ke` and `-(O)cska/-(O)cske` are productive on concrete nouns; `-(O)cskA` is the general one, `-kA` is excluded after monosyllabic consonant-final stems, after polysyllabic stop-final stems and after `a/e`-final stems, and is preferred after `i`-final stems.
- **Conditions**: (first match wins)
  1. a lexicalised diminutive exists for the base: use it (blocking is weak: parallel ad hoc forms remain possible)
  2. base in hypocoristic `-i` (`csoki`, `Feri`, `öcsi`): `-kA` only (`csokika`, `Ferike`, `öcsike`; `*csokicska`)
  3. other `i`-final base: `-kA` preferred (`nénike`, `cumika`, `tepsike`); `-cskA` marginal and item-level (`zoknicska` 63 beside `zoknika` 5, `?kocsicska` 29 beside `kocsika` 442, `*nénicske` 2 beside `nénike` 4,124 [WC2])
  4. `a/e`-final base: `-cskA` only, with LVL (`bundácska`, `csárdácska`, `bögrécske`, `kefécske`, `macskácska`, `kecskécske`; `*kapáka`, `*almáka`, `*zergéke`)
  5. other vowel-final base: `-cskA` (`folyócska`, `kígyócska`, `autócska`, `tetőcske`, `esőcske`, `ágyúcska`, `varjúcska`, `seprűcske`, `gyűrűcske`); `-kA` in lexicalised items (`anyóka`, `ülőke`)
  6. monosyllabic consonant-final base: `-(O)cskA` only (`lábacska`, `lépecske`, `hódocska`, `kutacska`, `fogacska`, `gyíkocska`, `szívecske`, `tollacska`, `övecske`; `*lábka`, `*szívke`)
  7. polysyllabic base ending in a stop, where **stop = `p t k b d g ty gy`** (the palatal stops included, measured): `-(O)cskA` only (`családocska`, `beszédecske`, `játékocska`, `bádogocska`, `bozótocska`, `oszlopocska`; `ty`/`gy` bases `hegyecske` 196 : `hegyke` 23, `ágyacska` 252 : `ágyka` 0, `atyácska` 425 : `atyka` 1, `kutyácska` 326 : `kutyka` 10 [WC2])
  8. other polysyllabic consonant-final base (sonorant, fricative, affricate): both possible (`asztalka ~ asztalocska`, `gödörke ~ gödröcske`, `ikonka ~ ikonocska`, `citromka ~ citromocska`, `bárányka ~ bárányocska`); if one form is an established word the other is not used; otherwise `-(O)cskA` preferred
- **Output**: diminutive noun
- **Examples**: listed; both on one base: `levélke ~ levelecske`, `tálka ~ tálacska`
- **Exceptions**: monosyllables with established `-kA`: `lányka`, `tálka` (SMNy3 itself gives `tál+ka` beside `tál+acska`); attested violations of rule 7 used colloquially (`galambka`, `cserépke`); **lexicalised `-ka` on stop-final stems without a diminutive reading**: `padka` [WC2 4009], `lapka` [WC2 11035], `kalapka` [WC2 102] - these have no `-(O)cskA` competitor and are not derived synchronically; the one item that resists rule 7 is `bátyka` 16 : `bátyácska` 20
- **Predictability**: `rule` + `tendency` (rule 8 preference)
- **Lexical input**: existing diminutive entries; hypocoristic flag
- **Interactions**: `DER-DIM-002`, `DER-DIM-003`, `DER-DIM-004`, `DER-MODEL-005`
- **Sources**: [SMNy3 pp. 106, 111-112, 129-131 (4.1.4, rule 4)], [KVF98 p. 353 (854), (855)], [magyarispell fonev_kepzo.aff (`-ka` only non-compound >= 2 syllables, study only)], [WC2], [ATT evidence/productivity-tagset.md 2026-09-16]
- **Validation**: attestation; Webcorpus 2: `ty`/`gy`-final stems take `-(O)cskA` over `-kA` by 8.5x (`hegyecske` 196 / `hegyke` 23), 33x (`kutyácska` 326 / `kutyka` 10), 425x (`atyácska` 425 / `atyka` 1) and infinitely (`ágyacska` 252 / `ágyka` 0), against the controls `kalapka` 102 (`p`), `madárka` 6,942 (`r`), `nénike` 4,124 (`i`) and `könyvecske` 24,182 : `könyvke` 0 (`v`). Tolerance Principle over the 5 corpus-attested `ty`/`gy` diminutive types: e = 1, theta = 3. Also `asztalka` 3, `bárányka` 1, `tálka` 2, `levélke` 1, `tavacska` 2, `lovacska` 1, `lányka` 5, `fiúcska` 5; goldens
- **Confidence**: consensus; the stop condition of rule 7 now **includes the palatal stops by measurement** rather than by inference (`OQ-06-005` resolved), with the caveat that N = 5 corpus-attested `ty`/`gy` diminutives - a wug test with novel `ty`/`gy`-final bases is the only way to raise it

### DER-DIM-002 -kA allomorphy
- **Statement**: `-ka/-ke` attaches to the free stem without linking vowel and harmonises in two ways.
- **Conditions**: free stem (no shortening `levélke`, no epenthetic drop `bokorka`, `gödörke`); after `i`: direct
- **Output**: `-ka` (back), `-ke` (front)
- **Examples**: `asztal` -> `asztalka`; `levél` -> `levélke`; `bokor` -> `bokorka`; `madár` -> `madárka`; `pohár` -> `pohárka`; `János` -> `Jánoska`; `Péter` -> `Péterke`; `néni` -> `nénike`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-MODEL-013` (semi-analytic)
- **Sources**: [SMNy3 p. 130], [morphdb grammar AFF_KA]
- **Validation**: attestation (`jánoska` 2, `pohárka` 2), goldens
- **Confidence**: consensus

### DER-DIM-003 -(O)cskA allomorphy
- **Statement**: `-(O)cskA` takes the plural-type linking vowel after consonants, LVL after `a/e`, and the bound stem; the final vowel is `a/e`.
- **Conditions**:
  - vowel-final: `-cska/-cske`
  - consonant-final non-lowering: `-ocska/-ecske/-öcske` (`hódocska`, `gyíkocska`, `családocska`, `könyvecske`, `gödröcske`)
  - consonant-final lowering: `-acska/-ecske` (`lábacska`, `házacska`, `tálacska`, `fogacska`, `tollacska`)
  - bound stem: v-stem (`lovacska`, `tavacska`, `kövecske`), shortening (`kezecske`, `levelecske`, `poharacska`, `kanalacska`, `kutacska`), epenthetic (`bokrocska`, `gödröcske`, `fémvödröcske`)
- **Output**: `-cska`, `-ocska`, `-acska`, `-ecske`, `-öcske`
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: stem `lowering`, alternants
- **Interactions**: `DER-MODEL-004`, `DER-MODEL-006`
- **Sources**: [SMNy3 pp. 108, 129-130], [SMNy3 p. 677 (Rebrus `-((A/O)cs)kA`)], [morphdb grammar AFF_KA (low/non_low selection)]
- **Validation**: attestation (`tavacska` 2, `lovacska` 1, `ágacska` 1, `könyvecske` 6), goldens
- **Confidence**: consensus

### DER-DIM-004 Semantic restrictions and pragmatic uses of -kA and -(O)cskA
- **Statement**: Diminutives are productive on count nouns, concrete or abstract; on mass nouns and adjectives they are lexicalised endearments; their meaning is DIM, pragmatically often affection rather than size.
- **Conditions**:
  - count nouns, abstract ones included: `hibácska` 99, `ügyecske` 50, `munkácska` 45, `problémácska` 25, `gondocska` 25 [WC2, all forms of the stem]; an unattested one is a potential word (`#jogocska`)
  - excluded: mass and uncountable nouns (`*unalomka`, `*levegőske`), the suffix-marked abstracts among them (`-sÁg`, `-(A)lOm`, `-Ás`: `*boldogságocska`); size or abstract-property adjectives in diminutive sense (`*megtermettke`)
  - lexicalised: on mass nouns `vizecske`, `pénzecske` 336 [WC2]; endearing adjectives `kicsike`, `picike`, `aprócska`, `laposka`, `vékonyka`, `szegényke`, `vidámka`; nouns `kisfiúcska`, `nénike`, `nővérke`, `ketteske`
  - lexicalised diminutives with specialised meaning: `járóka`, `fúvóka`, `estike`, `hajtóka`, `szívóka`
  - further derivation: `-sÁg`, `-(O)s`, `-Ú`, `-szerű`, `-féle`, `-mentes`, `-stUl`, `-beli`, `-fajta` possible (morphdb)
- **Output**: noun; non-lowering; poss3 `-ja/-je`
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `rule` + `lexical`
- **Lexical input**: noun `count` / `mass` (not carried: morphdb's `sing` marks singulare tantum, and a mass noun takes the sort plural, `virágmézek`); the engine refuses the suffix-marked abstracts only
- **Interactions**: `DER-DIM-001`, `DER-LEX-001`, `DER-COMP-002`
- **Sources**: [SMNy3 pp. 96, 106, 107, 130-131], [morphdb grammar AFF_KA], [WC2]
- **Validation**: attestation (`kicsike` 5, `picike` 5, `vékonyka` 4, `szegényke` 3; counts above)
- **Confidence**: consensus for the endearments; the count/mass condition replaces SMNy3's concrete/abstract one on [WC2] evidence

### DER-DIM-005 Hypocoristic -i on personal names
- **Statement**: The hypocoristic `-i` attaches to a truncated given name consisting of the maximal well-formed initial syllable (C)V(C)(C) built from the name's segments, with vowel length of the first vowel optionally neutralised to short; the result is a productive but not word-based pattern with many lexical exceptions.
- **Conditions**:
  - truncation base: first syllable maximised to the right with a licit coda: `Ferenc` -> `Fer-`, `Andrea` -> `And-`, `Ágnes` -> `Ág-` (`*Ágn-`, coda `gn` illicit), `Imre` -> `Im-`, `István` -> `Ist-`, `Viktória` -> `Vik-`, `Erzsébet` -> `Erzs-`, `Zsuzsanna` -> `Zsuzs-`, `Katalin` -> `Kat-`, `Szilvia` -> `Szilv-`, `Miklós` -> `Mik-`, `Ádám` -> `Ád-`
  - long first vowel shortens in a lexical subset: `Dániel` -> `Dani`, `Gábor` -> `Gabi`, `János` -> `Jani`, `Péter` -> `Peti`; kept in others: `Ádi`, `Ági`, `Józsi`
  - the truncated base must not end in a cluster that is an illicit coda; CCC codas excluded
  - harmony: from the truncated string (`Simon` -> `Siminek`, `Éva` -> `Évinek`, `Ilona` -> `Ilinek`, `Miklós` -> `Mikinek`, `Nándor` -> `Nándinak`)
  - LVL not applicable (truncation)
  - further suffixation: `-kA` (`Ferike`, `Andika`, `Zsuzsika`, `Petike`)
- **Output**: noun (proper name), poss3 `-ja/-je`
- **Examples**: `Ferenc` -> `Feri`; `Teréz` -> `Teri`; `Klára` -> `Klári`; `Aladár` -> `Ali`; `Zsuzsanna` -> `Zsuzsi`; `Ágnes` -> `Ági`; `István` -> `Isti`; `Viktória` -> `Viki`; `Imre` -> `Imi`
- **Exceptions**: not derivable by the syllable rule: `Zoltán` -> `Zoli` (`Zolti` marginal), `Oszkár` -> `Oszi` (`*Oszki`); reduplicative or historical: `András`, `Endre` -> `Bandi`; `Anna` -> `Panni`; `István` -> `Pisti`, `Pista`; `Albert` -> `Berci`; `Alfréd` -> `Frédi`; `Antal` -> `Tóni`; `Éva` -> `Vica`; `Ödön` -> `Dönci`; all lexical
- **Predictability**: `tendency` (syllable rule), `lexical` (exceptions and chosen form per name)
- **Lexical input**: name lexicon with stored hypocoristics
- **Interactions**: `DER-DIM-006`, `DER-DIM-007`, `PH-HARM-008`, `PH-SYL-002`
- **Sources**: [SMNy3 pp. 104, 131-132 (4.1.5)], [RST12 pp. 502-503 (examples 12, 13)], [KVF98 p. 353 (856)]
- **Validation**: attestation (`feri` 9, `zsuzsi` 8, `kati` 10, `erzsi` 7, `ági` 10, `imi` 7, `viki` 7, `zoli` 9, `dani` 10, `gabi` 10, `jani` 7, `peti` 9, `Bandi` 7, `Panni` 7, `Pista` 10, `Berci` 4, `Tóni` 4, `Vica` 3), goldens
- **Confidence**: consensus (pattern); `tendency` (predictions)

### DER-DIM-006 Other hypocoristic suffixes on names
- **Statement**: Name hypocoristics with `-ci`, `-csi`, `-ka`, `-kó`, `-us`, `-a`, `-u` are lexical per name; they combine with the same truncation bases as `-i`.
- **Conditions**: stored per name; frequent patterns:
  - `-ci`/`-csi` after CV or CVC truncation: `László` -> `Laci`; `Albert` -> `Berci`; `Károly` -> `Karcsi`; `János` -> `Jancsi`; `Mária` -> `Marcsi`; `Julianna`, `Judit` -> `Juci`
  - `-ka` on full or truncated names: `Mária` -> `Marika`; `József` -> `Jóska`; `György` -> `Gyurka`
  - `-kó`: `János` -> `Jankó`; `Pál` -> `Palkó`
  - `-us`: `Katalin` -> `Katus`; `Béla` -> `Bélus`
  - `-a`: `István` -> `Pista`; `Anna` -> `Ancsa`; `Julianna` -> `Julcsa` (with `-csa`)
  - `-i` + `-ka/-ke`: `Évike`, `Katika`, `Petike`
- **Output**: noun (proper name)
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: name lexicon
- **Interactions**: `DER-DIM-005`
- **Sources**: [SMNy3 p. 677 (Rebrus `-((A/O)cs)kA; -kus, -us, -u; -kó; -a, -ca`)], [SMNy3 p. 132]
- **Validation**: attestation (`Laci` 11, `Karcsi` 5, `Jancsi` 8, `Marcsi` 6, `Juci` 4, `Marika` 8, `Jóska` 8, `Gyurka` 4, `Jankó` 7, `Palkó` 6, `Katus` 7, `Ancsa` 2, `Julcsa` 2, `Évike` 2)
- **Confidence**: consensus

### DER-DIM-007 Hypocoristic clippings of common nouns
- **Statement**: Colloquial clipped nouns use the same truncation + `-i` pattern (and `-ó`, `-csi`, `-ci`, `-u`); they are lexical entries of informal register.
- **Conditions**: stored items; register `informal`
- **Output**: nouns, poss3 `-ja/-je`
- **Examples**: `csokoládé` -> `csoki`; `fagylalt` -> `fagyi`; `óvoda` -> `ovi` (first vowel shortened); `uborka` -> `ubi`; `cigaretta` -> `cigi`; `unokatestvér` -> `uncsi`; `pulóver` -> `pulcsi`; `testvér` -> `tesó`; `nagymama` -> `nagyi`; `anya` -> `anyu`, `anyuci`, `anyuka`; `apa` -> `apu`, `apuci`, `apuka`; `iskola` -> `suli` (German loan base); `információ` -> `infó`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `DER-DIM-005`, `DER-DIM-001` (rule 2: `csokika`)
- **Sources**: [SMNy3 p. 132 (`csoki`, `fagyi`, `ovi`, `ubi`)], [KVF98 p. 353 (857)]
- **Validation**: attestation (`csoki` 11, `fagyi` 7, `ovi` 8, `ubi` 3, `cigi` 8, `uncsi` 3, `pulcsi` 3, `tesó` 4, `nagyi` 8, `anyu` 9, `anyuka` 11, `anyuci` 2, `suli` 10, `infó` 9)
- **Confidence**: consensus

### DER-DIM-008 -us, -ci and -i endearments of common nouns; child language -ikA
- **Statement**: Endearment forms `-us` (`kutyus`, `cicus`), `-ci` (`anyuci`) and child-language body-part forms `-ika/-ike` (`hajika`, `szemike`) are lexical or occasional; the `-ikA` forms contain a potential hypocoristic `-i` followed by `-kA`.
- **Conditions**: animals and kinship terms (`-us`, `-ci`); body parts (`-i` + `-kA`)
- **Output**: nouns
- **Examples**: `kutya` -> `kutyus`; `cica` -> `cicus`; `haj` -> `hajika`; `szem` -> `szemike`; `fül` -> `fülike`; `láb` -> `lábika`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `DER-DIM-001`, `DER-DIM-005`
- **Sources**: [SMNy3 p. 132 (`haj+i+ka`, `szem+i+ke`, `fül+i+ke`, `láb+i+ka`)], [SMNy3 p. 96 (`kutyuska`)]
- **Validation**: attestation (`kutyus` 8, `cicus` 2)
- **Confidence**: consensus

## DER-PREF

### DER-PREF-001 Verbal prefixes as compound-like members
- **Statement**: A verbal prefix adjacent to its verb forms one orthographic and morphological word with it, but remains a separable syntactic word; in derivatives of prefixed verbs the prefix is frozen inside the word.
- **Conditions**:
  - inventory and separability: `VRB-PREV-001`, `VRB-PREV-006`
  - writing together or apart: `VRB-PREV-002`, `ORT-TOG-026`, `ORT-TOG-027`
  - derivatives (`-Ás`, `-Ó`, `-At`, `-mÁny`, `-vÁny`, `-hAtÓ`, `-AtlAn`, `-(O)tt`): prefix fixed (`feltétel`, `átirat`, `kiküldetés`, `kiadvány`, `megoldatlan`); only `nem` may intervene (`meg nem értés`, `be nem avatkozás`, written apart, `ORT-TOG-027`)
  - derivation applies to the prefixed verb as a whole: `[meg-old]-ás`, not `meg-[old-ás]`
  - `-gAt` frequentatives pair with the prefixed perfective (`kinyit` / `nyitogat`) and may take a prefix themselves (`kinyitogat`)
- **Output**: analysis `PREFIX=VERB+SUFFIX` with the prefix boundary marked as compound-level
- **Examples**: `megold` -> `megoldás`, `megoldó`, `megoldatlan`, `megoldható`; `kiad` -> `kiadvány`, `kiadás`; `feltesz` -> `feltétel`; `átír` -> `átirat`
- **Exceptions**: lexicalised derivatives whose unprefixed base is not used (`előadás`, `kivétel`) are entries with segmentation
- **Predictability**: `rule`
- **Lexical input**: prefix inventory (`VRB-PREV-001`)
- **Interactions**: `VRB-PREV-001`..`VRB-PREV-006`, `DER-PREF-002`, `ORT-63-004`, `ORT-SPLIT-015`
- **Sources**: [SMNy3 pp. 52-54 (prefix frozen in derivatives)], [SMNy3 pp. 383-439 (8)], [AkH12 §120, §139, §234]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-PREF-002 Prefixes and prefix-like first members do not affect suffix selection
- **Statement**: Harmony, linking vowels, stem alternations and suffix choice of a prefixed or prefix-compounded word are computed from the base; the prefix is invisible to them.
- **Conditions**: any verbal prefix; any bound or free first member (`DER-PREF-005`), `leg-`
- **Output**: suffix selection as for the base
- **Examples**: `elolvas` -> `elolvasás`, `elolvasok` (front prefix, back suffix); `kinyit` -> `kinyitogat` (monosyllabic `nyit` selects `-ogat`); `főigazgató` -> `főigazgatónak`; `legszebb` -> `legszebben`; `ősanya` -> `ősanyák`
- **Exceptions**: none
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-PREV-005`, `DER-COMP-008`, `PH-HARM-022`
- **Sources**: [SMNy3 p. 514], [RST12 fn. 5], [KVF98 p. 420]
- **Validation**: goldens
- **Confidence**: consensus

### DER-PREF-003 Reduplicated and paired prefixes
- **Statement**: Prefix reduplication (`ki-kinéz`) expresses the repeated or intermittent occurrence of the event of a prefixed verb and is productive; opposite prefix pairs (`ki-be jár`) are written apart from the verb; writing and constraints owned by `VRB-PREV-003`, `ORT-TOG-028`.
- **Conditions**: a prefixed verb whose event can recur, whatever its aspect class (`meg-megáll`, `meg-megjön`, `meg-megérti`); the copy is the verb's own preverb, joined by a hyphen
- **Output**: see owning rules
- **Examples**: `meg-megáll`, `meg-megjön`, `ki-kimegy`, `vissza-visszatér`, `vissza-visszatérés`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `VRB-PREV-003`, `ORT-TOG-028`
- **Sources**: [KVF98 p. 381 (945)], [AkH12 §120 d), §141 c) (`meg-megállt`, `vissza-visszatérés`)]
- **Validation**: goldens
- **Confidence**: consensus; the accomplishment exclusion KVF98 states is contradicted by its own `meg-megáll` and by AkH12

### DER-PREF-004 leg- and legesleg-
- **Statement**: `leg-` (superlative) and `legesleg-` (excessive) are the only true prefixes of Hungarian nominal morphology; they attach to comparatives, and lexically to a few ordinal-like adjectives and adverbs; forms owned by `NOM-ADJ-006`.
- **Conditions**:
  - productive: comparative adjectives and adverbs (`legnagyobb`, `legjobban`)
  - lexical non-comparative bases: `legelső`, `legutolsó`, `legalsó`, `legfelső`, `leghátsó`, `legelöl`, `legalul`, `legfelül`, `legelőre`, `legelőször`, `legutoljára`, `leginkább`, `legalább`
  - on a phrase written apart the superlative is written solid (`legszívhezszólóbb`, `ORT-TOG-037`)
  - hyphenation at `leg-` boundary (`leg-alább`, `ORT-SPLIT-015`); `leg+gy` sequences are two letters (`leggyorsabb`, `WS-DIG-006`)
- **Output**: adjective or adverb
- **Examples**: listed
- **Exceptions**: listed
- **Predictability**: `rule` + `lexical`
- **Lexical input**: lexical list of non-comparative bases
- **Interactions**: `NOM-ADJ-006`, `ORT-TOG-007`, `ORT-TOG-037`, `WS-DIG-006`
- **Sources**: [SMNy3 p. 676 (Rebrus `leg-`, `legesleg-`: only prefixes; `legelső`, `legelőre`, `legelőször`, `legutoljára`, `legalsó`)], [AkH12 §100, §128, §235], [morphdb derivation README (LEGBB: `legelső`, `leghátsó` lexical)]
- **Validation**: attestation (`legnagyobb` 22, `legelső` 12, `legalább` 21), goldens
- **Confidence**: consensus

### DER-PREF-005 Bound and prefix-like first members
- **Statement**: A closed set of non-independent or semantically specialised first elements forms compounds in series (`al-`, `fő-`, `ál-`, ...); they are compound members, not derivational prefixes, and do not influence the head.
- **Conditions**:
  - non-independent (AkH list, always solid): `al-` 'sub-, vice-' (`alelnök`, `alezredes`, `alhadnagy`, `alorvos`), `bel-` 'inner' (`belföld`, `belügy`), `elő-` 'fore-' (`előtag`, `előszoba`), `ét-` (`étvágy`, `étlap`), `gyógy-` 'medical' (`gyógyszer`, `gyógytorna`), `kultúr-` (`kultúrház`), `kül-` 'outer, foreign' (`külügy`, `külföld`), `pót-` 'spare' (`pótágy`)
  - semantically specialised free words used serially: `fő-` 'chief, main' (`főigazgató`, `főváros`, `fővezér`), `ál-` 'false' (`álnév`, `álmennyezet`, `álszakáll`), `ős-` 'primeval' (`ősanya`, `őskor`), `ön-` 'self' (`önkritika`, `önéletrajz`, `önuralom`), `mű-` 'artificial' (`műanyag`, `műbőr`), `zug-` 'clandestine' (`zugivó`), `társ-` 'co-' (`társszerző`), `utó-` 'after-' (`utóhatás`, `utókúra`), `túl-` 'over-' (`túlóra`, `túlsúly`, `túlterhelés`), `újra-` 're-' (`újraindítás`, `újrahasznosítás`, deverbal from the prefix `újra`), `nagy-`, `kis-` in kinship and rank (`nagyapa`, `kisasszony`)
  - bound elements cannot be elided in coordination (`*al- és főorvos` odd, `mellék- és főépület` fine), unlike semi-words (`DER-COMP-017`)
  - `nem` + adjective or participle (`nem dohányzó`) is written apart and not a prefix (`SYN`)
- **Output**: compound analysis with first member marked bound or prefixoid
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `lexical` (closed list)
- **Lexical input**: first-member list with meaning
- **Interactions**: `ORT-TOG-038`, `DER-COMP-002`, `DER-COMP-021`, `DER-PREF-002`
- **Sources**: [AkH12 §129], [SMNy3 pp. 54, 110, 441-442 (prefix-like `al-`, `fő-`, `ál-`, `zug-`, `gyógy-`, `kül-`)], [SMNy3 p. 450 (`ön-`, `elő-`, `utó-` lexicalised)]
- **Validation**: attestation (`alelnök` 12, `álnév` 4, `túlóra` 5, `túlsúly` 9, `újraindítás` 6, `újrahasznosítás` 8, `főváros` 18, `kisasszony` 10, `nagyanya` 10)
- **Confidence**: normative (AkH list); consensus (others)

### DER-PREF-006 Foreign prefixes and combining forms
- **Statement**: Foreign prefixes and combining forms (`anti-`, `poszt-`, `ex-`, `szuper-`, `mini-`, `multi-`, `bio-`, `hidro-`, `-lógia`, `-gráfia`, `-méter`) are compound members when the element or its partner is used independently, otherwise parts of a monomorphemic loan; this status decides 6:3 counting, hyphenation and harmony tendencies.
- **Conditions**:
  - member status: element used alone (`mini`, `szuper`, `bio`) or attached to an independent Hungarian word (`antialkoholista`, `miniszoknya`, `exférj`): member (`ORT-63-005`)
  - otherwise (`biológia`, `hidrogén`, `demokrácia`): unsegmented loan for spelling; pseudo-compound for harmony (`PH-HARM-022`) and hyphenation (`ORT-SPLIT-014`, `ORT-SPLIT-019`)
  - harmony of pseudo-compounds: last piece tends to govern (`kilométernek`, `oxigénnel`, `kilogrammnak`)
- **Output**: member flag
- **Examples**: `miniszoknya-viselet`; `antialkoholista-klub`; `biotechnológia`; `kilométer` -> `kilométernek`
- **Exceptions**: -
- **Predictability**: `lexical` (element list) + `rule` (status test)
- **Lexical input**: combining-form list with independence flag
- **Interactions**: `ORT-63-005`, `ORT-SPLIT-014`, `ORT-SPLIT-019`, `PH-HARM-022`
- **Sources**: [AkH12 §139, §231, §237], [SMNy3 p. 54 (`bio-techno-lógia`, `hispan-isztika`)], [F13 theses §3.3]
- **Validation**: goldens, attestation
- **Confidence**: normative (spelling status); `tendency` (harmony)

## DER-COMP

### DER-COMP-001 Definition and diagnostics of compounds
- **Statement**: A compound is a morphological formation of two or more word-level members that is a single word for syntax: no syntactic operation can alter its inside, its members cannot be modified or inflected separately, and it carries compound stress on the first syllable.
- **Conditions**: (diagnostics, the first is decisive)
  - no insertion: no word or inflectional suffix can intervene between members (`lábszár` vs `lábad szára` two words; `*nagyobb herceg` for `nagyherceg`; comparative `(leg)jópofább`, not `*jobbpofa`)
  - no separate modification: `szép aranygyűrű` modifies the whole
  - compound stress on the first syllable of the first member; secondary stress possible on later members in long compounds
  - first member has generic reference (no anaphora to it)
  - coordination ellipsis of a shared member is possible for word-like members (`cserép- és téglagyár`, `férfiing és -nadrág`), not for bound first members (`*utófájdalom és -hatás`) and not for suffixes (`*ír- és olvasás`)
  - the orthographic solid spelling is a consequence (`ORT-TOG`), not the definition: `zsíros kenyér` 'bread and dripping' is a compound by the diagnostics although written apart
- **Output**: compound analysis; bare-noun object + verb (`levelet ír`) and prefix + verb are not compounds by the insertion diagnostic
- **Examples**: `nagyherceg`, `jópofa`, `hidegfront`, `menyasszonytánc`, `képeskönyv` (vs phrase `képes könyv`)
- **Exceptions**: exocentric and coordinative compounds (`DER-COMP-015`) lack a head but pass the insertion diagnostic
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-MODEL-001`, `DER-COMP-002`, `DER-COMP-016`, `DER-COMP-026`, `ORT-TOG-001`
- **Sources**: [SMNy3 pp. 51-54 (Keszler 2.1.2.2)], [SMNy3 pp. 441-443 (Kiefer 9.1)], [KVF98 pp. 374-375]
- **Validation**: goldens
- **Confidence**: consensus

### DER-COMP-002 Head and category of endocentric compounds
- **Statement**: In endocentric compounds the rightmost member is the head: it fixes the word class, the inflection class and (mostly) the semantic class of the compound.
- **Conditions**: binary structure `[X Y]` with `Y` a noun or adjective (or a deverbal noun/adjective)
- **Output**: category = category of `Y`; inflection per `Y` (`DER-COMP-007`..`DER-COMP-009`); countability per `Y` (`konyhaszékek`; a mass head takes the sort plural 'kinds of N': `virágmézek` 150 [WC2])
- **Examples**: `anyatej` N; `virágüzlet` N; `férfiing` N; `hidegfront` [ADJ + N] N; `kőkemény` [N + ADJ] ADJ; `zenetudomány` 'science of music'; `asztalláb` 'leg of a table'; `nagyharang` 'big bell'
- **Exceptions**: exocentric compounds (`DER-COMP-015`); lexicalised compounds whose head reading is opaque (`nyugdíj`) still take inflection from the last member
- **Predictability**: `rule`
- **Lexical input**: category of the last member
- **Interactions**: `DER-COMP-003`, `DER-COMP-007`
- **Sources**: [SMNy3 pp. 443-448 (9.2.1, 9.3)], [KVF98 p. 374]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-COMP-003 Compound patterns: productive and unproductive
- **Statement**: Productive endocentric compounding combines N or ADJ first members with N or ADJ heads; patterns with a verb or adverb head or first member, and pronoun or postposition members, occur only in lexicalised or secondary formations.
- **Conditions**:
  - productive: `N -> {N, ADJ} + N` (`adókártya`, `bankautomata`, `feketeszoftver`, `keresőgép`); `ADJ -> {N, ADJ} + ADJ` (`kőkemény`, `vaksötét`, `méregdrága`, `sötétbarna`); N + ADJ and ADJ + ADJ less frequent in new words
  - participle heads count as ADJ/N: `hamutartó`, `újságíró`, `hajótisztító`; past participle heads: `porlepett`, `jogfosztott`
  - colour ADJ + ADJ: derived first member endocentric (`kékeszöld`), underived pair coordinative (`fekete-fehér`, `DER-COMP-015`)
  - unproductive, lexical: `V + N` (`látcső`, `lépcső`, `lakbér`, `védjegy`), `PRO + N` (`önuralom`, `magaviselet`), `N + POSTP` (`délelőtt`, `délután`), `ADV + ADV` (`idehaza`), `NUM + N` except numerals
  - secondary (derived from a phrase or backformed, not a compound pattern): `ADV + N` from `ADV + V` nominalisation (`továbbtanulás`, `újraegyesítés`), `N + V` backformations (`DER-COMP-024`), `NUM + N + suffix` (`DER-COMP-014`)
  - compound numerals: own rules (`PND`)
- **Output**: pattern classification used by generation (only productive patterns generated freely) and by splitting (`DER-COMP-022`)
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: categories of members
- **Interactions**: `DER-COMP-014`, `DER-COMP-022`, `DER-COMP-024`, `PND`
- **Sources**: [SMNy3 pp. 449-453 (9.4, table 11, rule 12)], [KVF98 pp. 375-380]
- **Validation**: goldens
- **Confidence**: consensus

### DER-COMP-004 Binary and recursive structure
- **Statement**: Productive compounds are binary and recursive: a compound member may itself be a compound or derivative, and a new member is added only at an edge, never inside an existing compound.
- **Conditions**: `[[repülő gép] gyártás]`; `víz -> vízgyűjtő -> vízgyűjtővezeték -> vízgyűjtővezetékszerelő` (not insertion of `gyűjtő` into `vízvezetékszerelő`)
- **Output**: bracketed structure with a main boundary; the main boundary is where a 6:3 hyphen goes (`ORT-63-006`)
- **Examples**: `repülőgépgyártás` = `[repülőgép][gyártás]`; `rendőrfőtörzsőrmester` = `[rendőr][fő[törzs[őrmester]]]`; `háztartásigép-javítás` = `[[háztartás-i gép] javítás]`
- **Exceptions**: flat readings are semantically equivalent for syntax; bracketing matters for hyphenation and member counting
- **Predictability**: `rule` (structure); bracketing choice `tendency` (semantic)
- **Lexical input**: member lexicon
- **Interactions**: `ORT-63-002`, `ORT-63-006`, `DER-COMP-023`
- **Sources**: [SMNy3 pp. 441-442, 447-449 (figures 6-10)], [KVF98 p. 381 (946)], [AkH12 §139]
- **Validation**: goldens
- **Confidence**: consensus

### DER-COMP-005 Member form at the boundary
- **Statement**: Hungarian compounds have no linking element; a non-final member appears in its citation (free) form, with no low vowel lengthening, no stem alternation and no inflection.
- **Conditions**: boundary between members
- **Output**: concatenation of citation forms
- **Examples**: `alma` + `fa` -> `almafa` (`*almáfa`); `kutya` + `ház` -> `kutyaház`; `fa` + `asztal` -> `faasztal`; `kő` + `kemény` -> `kőkemény` (not `*kövkemény`); `tó` + `part` -> `tópart`; `bokor` + `ugró` -> `bokorugró`; `ló` + `erő` -> `lóerő`; `kéz` + `írás` -> `kézírás`
- **Exceptions**: lexicalised inflected or derived first members (`DER-COMP-006`); bound first members with their own form (`DER-PREF-005`); lexicalised `-i` compound forms with v-stem or shortening (`tavirózsa`, `havilottó`, `DER-ADJ-003`)
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-MODEL-005`, `DER-MODEL-013`, `DER-COMP-006`
- **Sources**: [SMNy3 p. 796 (no LVL in compounds: `fakés`, `faasztal`)], [SMNy3 pp. 107-108, 441]
- **Validation**: goldens, attestation (`almafa` 4, `kutyaház` 1)
- **Confidence**: consensus

### DER-COMP-006 Lexicalised compounds with inflected or derived first members
- **Statement**: First members carrying case, possessive or plural morphology occur only in lexicalised compounds, which are lexicon entries; derived first members (`-i`, `-(O)s`, `-Ó`, `-Ás`) are regular.
- **Conditions**:
  - case-marked first member: `napraforgó`, `tűzrőlpattant`, `égbekiáltó`, `partraszállás`, `életbelépés`, `kézhezvétel`, `rendbehozatal`
  - possessive-marked head with bare first member: `barátfüle`, `istennyila`, `vásárfia`; plural + possessive: `bolondokháza`; accusative: `idejétmúlt`
  - derived first members (regular): `képeskönyv`, `söröspohár`, `erőspaprika`, `égitest`, `háziasszony`, `takarítónő`, `hűtőszekrény`, `hálóterem`, `belépőjegy`
  - superlative on a phrase written apart: `legszívhezszólóbb` (`DER-PREF-004`)
- **Output**: lexicon entries; inflection on the last element (`napraforgók`, `barátfülét`)
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `lexical` (inflected first members); `rule` (derived first members)
- **Lexical input**: lexicon entries
- **Interactions**: `ORT-TOG-016`, `ORT-TOG-043`, `DER-COMP-005`
- **Sources**: [AkH12 §106, §128, §135], [KVF98 pp. 376-377 (931 b), (932)]
- **Validation**: attestation (`napraforgó` 11, `égbekiáltó` 3)
- **Confidence**: consensus

### DER-COMP-007 Inflection on the last member only
- **Statement**: Subordinative, exocentric and fused coordinative compounds are inflected only on the last member; loose coordinative compounds and twin words inflect on both members.
- **Conditions**:
  - endocentric: suffix on the last member (`vasutak`, `aranygyűrűt`)
  - fused coordinative (solid): last member (`búbánatos`, `hadseregben`, `rúgkapálnak`)
  - loose coordinative (hyphenated): both members (`sütnek-főznek`, `eget-földet`, `irult-pirult`)
  - see `DER-COMP-015` for types
- **Output**: suffix position(s)
- **Examples**: listed
- **Exceptions**: variable items (`hírneve ~ híre-neve`, `ORT-TOG-012`)
- **Predictability**: `rule` + `lexical` (coordinative type per item)
- **Lexical input**: compound type
- **Interactions**: `DER-COMP-015`, `ORT-TOG-008`..`ORT-TOG-014`
- **Sources**: [SMNy3 pp. 444-445], [AkH12 §101-§104]
- **Validation**: goldens
- **Confidence**: normative (writing); consensus (inflection)

### DER-COMP-008 Harmony of compounds
- **Statement**: Each compound member is its own harmony domain; suffixes harmonise with the last member only.
- **Conditions**: any native compound; prefix-like and bound first members included; for foreign pseudo-compounds see `DER-PREF-006`, `PH-HARM-022`
- **Output**: suffix vowel from the last member's harmony class (including its vacillation and antiharmony)
- **Examples**: `könyvtár` -> `könyvtárak`; `lábszár` -> `lábszárak`; `hídfő` -> `hídfőt`, `hídfőnek`; `kódexmásoló` -> `kódexmásolóhoz`; `vashíd` -> `vashidat`; `ablaküveg` -> `ablaküvegek`; `szépíró` -> `szépírónak`
- **Exceptions**: lexicalised compounds treated as simplex (`DER-COMP-010`)
- **Predictability**: `rule`
- **Lexical input**: compound boundary
- **Interactions**: `PH-HARM-001`, `PH-HARM-022`, `DER-COMP-009`, `DER-COMP-010`
- **Sources**: [RST12 fn. 5], [SMNy3 p. 442 (compound not a phonological word for harmony: `könyvvásár`)], [KVF98 p. 420]
- **Validation**: attestation (`könyvtárak` 12, `hídfőt` 1), goldens
- **Confidence**: consensus

### DER-COMP-009 Stem class inherited from the last member
- **Statement**: A compound inherits all inflectional stem properties of its last member: lowering, epenthetic, shortening, v-stem, possessive-only alternants, `-a/-ja` choice, accusative type and antiharmony.
- **Conditions**: endocentric compound
- **Output**: inflection as the last member
- **Examples**: lowering + shortening `út` -> `vasút` -> `vasutak`, `vasutat`, `vasútja`; v-stem `ló` -> `faló` -> `falovak`; epenthetic `bokor` -> `virágbokor` -> `virágbokrok`; shortening `kéz` -> `balkéz` -> `balkezek`; antiharmonic `híd` -> `vashíd` -> `vashidak`; lowering `ház` -> `kutyaház` -> `kutyaházak`; possessive-only `ajtó` -> `szekrényajtó` -> `szekrényajtaja`
- **Exceptions**: lexicalised compounds (`DER-COMP-010`); `ír` shortens in simplex `iroda` but not in `távírda` (derivation, `DER-NOUN-022`)
- **Predictability**: `rule`
- **Lexical input**: last member's stem properties (`NOM-STEM`, `NOM-LOW`, `NOM-POSS`)
- **Interactions**: `NOM-STEM-001`, `NOM-LOW`, `NOM-POSS-007`, `NOM-ACC`
- **Sources**: [SMNy3 p. 443 (head fixes inflection)], [SMNy3 p. 674 (`távírda`)], [emMorph and morphdb compound handling (COMPOUND: NOM NOM) (study)]
- **Validation**: attestation (`vasutak` 8), goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-COMP-010 Lexicalised compounds treated as simplex
- **Statement**: Compounds whose members are no longer transparent for speakers are stored as simplex stems with their own properties; the engine must not recompute harmony or stem class from a historical last member.
- **Conditions**: lexicon flag `opaque_compound`
- **Output**: properties from the lexicon entry
- **Examples**: `férfi` (historically `fér` + `fi`): back before vowel-initial suffixes (`férfiak`, `férfias`), vacillating before consonant-initial (`férfinak ~ férfinek`, `PH-HARM-009`); `hónap` -> `hónapok` (regular like `nap`); `délután` -> `délutánok`; `kisasszony`; `nyugdíj` (no 'díj' reading); `ebéd`; `ország` not a compound
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: opaque compound list
- **Interactions**: `PH-HARM-009`, `DER-COMP-008`, `DER-COMP-023`
- **Sources**: [RST12 pp. 498-499 (`férfi`)], [SMNy3 p. 448 (`nyugdíj`, `szavaz`, `készülék` treated as unanalysed)]
- **Validation**: attestation (`férfiak` 18, `hónapok` 13, `délutánok` 2)
- **Confidence**: consensus

### DER-COMP-011 Semantically predictable N + N subtypes
- **Statement**: Some N + N and N + ADJ subtypes have a predictable interpretation; generation of new compounds in these subtypes is meaning-safe, others are morphotactically possible but semantically unpredictable.
- **Conditions**: (predictable readings)
  - material + artefact: 'N2 made of N1' (`favilla`, `rézfazék`, `ezüstóra`, `nádbútor`)
  - concrete N1 + storage place: 'N2 for keeping N1' (`jégverem`, `fegyverraktár`, `nyúlketrec`, `lóistálló`)
  - concrete N1 + set noun: 'set of N1' (`lánycsapat`, `juhnyáj`, `kőkupac`)
  - concrete N1 + part noun: 'part of N1' (`konyhaablak`, `háztető`, `nadrágzseb`, `biciklikerék`)
  - liquid + `csepp`, powder + `felhő` (`esőcsepp`, `olajcsepp`, `porfelhő`, `füstfelhő`)
  - concrete N + colour ADJ: 'coloured like N' (`aranysárga`, `hollófekete`, `fűzöld`, `tengerzöld`)
  - deverbal head: `DER-COMP-013`
  - ADJ + N compounds are never semantically productive (they are lexicalised phrases)
- **Output**: interpretation label for generated compounds
- **Examples**: listed
- **Exceptions**: whole-part order: the whole is the first member (`háztető`; `*tetőház` in the part reading, `tetőház` 'rooftop house' being the locative type)
- **Predictability**: `rule` (subtypes)
- **Lexical input**: noun semantic classes (`material`, `artefact`, `container`, `set`, `part`, `liquid`, `powder`)
- **Interactions**: `DER-COMP-003`, `DER-COMP-013`, `ORT-TOG-020`, `ORT-TOG-022`
- **Sources**: [SMNy3 pp. 452-455 (9.4 subtypes 1-6)], [SMNy3 pp. 144-145 (whole-part order)]
- **Validation**: goldens
- **Confidence**: consensus

### DER-COMP-012 Derivation of compounds
- **Statement**: A compound can be derived if it satisfies the input conditions of the derivational suffix; in practice nominal suffixes and the listed verbalisers apply, while suffixes requiring a verbal or foreign-adjective base do not.
- **Conditions**:
  - applicable: `-sÁg` (`rendszerfelelősség`), `-kA` (`viráglevélke`), `-(O)cskA` (`fémvödröcske`), `-i` (`gépkocsivezetői`), `-(j)Ú` (`vaskezű`), `-(O)s` (`levélírós`), `-beli` (`reformkorbeli`), `-nyi` (`vizespohárnyi`), `-(O)z` (`műtrágyáz`, `nagybőgőzik`), `-kOdik` (`rendszerfelelősködik`)
  - not applicable: `-itás` (no foreign-adjective compounds), `-Ás`, `-Ó` (no verbal compounds except prefixed verbs), privative rarely
  - the derivative's form follows the last member (`fémvödröcske` with epenthetic `vödör`)
  - ambiguity of attachment: `faasztalka` = `[faasztal]ka` or `fa[asztalka]`; `magánovi` = `[magánóvoda]` clipped or `magán[ovi]`
- **Output**: derived compound
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `DER-MODEL-003`, `DER-COMP-009`, `ORT-SPLIT-016`, `ORT-63-003`
- **Sources**: [SMNy3 pp. 107-108 (3.3, list of suffixes on compounds)]
- **Validation**: goldens
- **Confidence**: consensus

### DER-COMP-013 Compounds with deverbal heads
- **Statement**: With `-Ás` and `-Ó` heads the first member realises an argument of the base verb: theme/patient and instrument are possible for `-Ás`, non-intentional subjects for `-Ás` of intransitives, while `-Ó` heads take only the theme/patient.
- **Conditions**:
  - `-Ás` head: object (`levélírás`, `gyerekellátás`), non-agentive subject (`hóesés`, `fejfájás`, `gyermeksírás`, `áremelkedés`, `darázscsípés`, `kutyaharapás`), instrument (`olajfestés`, `gázmérgezés`), location; intentional actor excluded
  - `-Ó` head: theme/patient only (`újságíró`, `hamutartó`, `arcképfestő`, `gyomormérgező`); not agent or instrument (`*gombamérgező` 'poisoner using mushrooms', `*gázmérgező` 'poisoner using gas'; the theme reading 'one who poisons mushrooms, gas' is regular)
  - the first member is a bare stem (no case, generic reference)
- **Output**: compound noun or adjective with argument reading
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: verb argument structure
- **Interactions**: `DER-NOUN-006`, `DER-NOUN-008`, `SYN`
- **Sources**: [KVF98 pp. 375-376 (927)-(930)], [SMNy3 pp. 465-475 (9.5.2)]
- **Validation**: attestation (`hóesés` 4, `levélírás` 2, `újságíró` 14, `fejfájás` 12), goldens
- **Confidence**: consensus

### DER-COMP-014 Secondary compounds from phrases
- **Statement**: A phrase `[MOD N]` or `[NUM N]` followed by `-(j)Ú`, `-(O)s`, `-i`, `-nyi` forms an adjective whose spelling as one word depends on member structure; the unsuffixed `MOD+N` string is not an independent compound, and exocentric `ADJ+N` person nouns are formed from `-(j)Ú` phrases by dropping the suffix.
- **Conditions**:
  - `NUM + N + suffix`: `ötéves`, `háromlábú`, `kétüléses`, `húszfőnyi`, `harmincnapi` (`értékű`, `üléses` do not occur as adjectives without a modifier; SMNy3 p. 451)
  - `ADJ + N + -(j)Ú`: `hosszú lábú`, `nagy fejű` (written apart), lexicalised solid `bőkezű`, `jószívű`
  - writing together or apart: `ORT-TOG-023`
  - exocentric from `-(j)Ú` phrases, productive for persons: `kopasz fejű` -> `kopaszfej`, `görbe lábú` -> `görbeláb`, `nagy orrú` -> `nagyorr`
- **Output**: adjective; exocentric noun
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: numeral simplicity (`ORT-TOG-023`)
- **Interactions**: `DER-ADJ-008`, `DER-ADJ-006`, `ORT-TOG-023`, `DER-COMP-015`
- **Sources**: [SMNy3 pp. 444-445 (exocentric `kopaszfej`), p. 451 (secondary compounds)], [AkH12 §117]
- **Validation**: attestation (`ötéves` 11, `négylábú` 7, `kétüléses` 5)
- **Confidence**: normative (spelling); consensus (morphology)

### DER-COMP-015 Exocentric, coordinative, reduplicative and twin compounds
- **Statement**: Non-endocentric compounds are unproductive except twin words and productive exocentric person nouns; their inflection and spelling depend on the type.
- **Conditions**:
  - exocentric (no head, noun): `hiszekegy`, `egyszeregy`, `jöttment`, `huzavona`, `nyakigláb`, `kékszakáll`, `kopaszfej` type (`DER-COMP-014`)
  - coordinative proper: `adásvétel`, `jár-kel`, `él-hal`, `eszik-iszik`, `süt-főz`; a pair of bare words lives only as a listed compound (not `apró-nagy`, AkH12 §101 b)), so `?rohan-szalad` is marginal; an occasional pair of two words of the same role carrying the same suffix, related or opposite in meaning, is free and hyphenated (`rohantak-szaladtak`, `kínos-fájdalmas`, `sírva-nevetve`, AkH12 §101 b))
  - reduplication of full forms (closed classes): `alig-alig`, `egyszer-egyszer`, `ki-ki`, `egy-egy`, `sok-sok`; modified: `réges-régi`, `nőttön-nő`, `megy-mendegél`
  - twin words (ikerszó): one member a modified copy, often meaningless: `dimbes-dombos`, `irul-pirul`, `izeg-mozog`, `csigabiga`, `terefere`; productive with vowel alternation `i/a`, `e/ö`, `i/u` (`zimmeg-zümmög`, `kipeg-kopog`, `piffeg-puffog`)
  - harmony per member for twin and coordinative types (`irult-pirult`, `sütnek-főznek`)
  - spelling: `ORT-TOG-004`..`ORT-TOG-014`, `ORT-HYPH-008`
- **Output**: type flag, inflection per `DER-COMP-007`
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `lexical` (except twin-word pattern `tendency`)
- **Lexical input**: compound type
- **Interactions**: `DER-COMP-007`, `ORT-TOG-004`..`ORT-TOG-014`
- **Sources**: [SMNy3 pp. 444-446 (9.2.2-9.2.5)], [AkH12 §97-§104], [KVF98 p. 375 (926)]
- **Validation**: attestation (`huzavona` 6, `adásvétel` 7, `egyszeregy` 1), goldens
- **Confidence**: consensus

### DER-COMP-016 Compound stress
- **Statement**: Compounds carry primary stress on the first syllable of the first member; in long compounds a later member may carry secondary stress; a phrase has stress on both words.
- **Conditions**: compound vs phrase with identical segments
- **Output**: prosodic annotation for TTS and for disambiguating `zöld asztal` / `zöldasztal`
- **Examples**: `'zöldasztal` vs `'zöld 'asztal`; `'kerekasztal-beszélgetés`; `'vérnyomásmérőkészülék,gyártó`
- **Exceptions**: compound numerals of additive type may have several primary stresses (`'tizen'három`)
- **Predictability**: `rule`
- **Lexical input**: compound boundary
- **Interactions**: `PH`, `PND`
- **Sources**: [SMNy3 p. 442 (property 5), p. 451 (numerals)]
- **Validation**: goldens
- **Confidence**: consensus

### DER-COMP-017 Semi-words as compound members
- **Statement**: Semi-words (`-szerű`, `-féle`, `-fajta`, `-forma`, `-nemű`, `-rét`, `-rétű`, `-mentes`, `-né`, borderline `-beli`) are non-independent elements that behave as last compound members for LVL, harmony domain, spelling at the boundary and coordination ellipsis, while their allomorphy and semantics are recorded as suffixes in `DER-ADJ` and `DER-NOUN`.
- **Conditions**:
  - coordination ellipsis from both directions: `kutya- és macskaszerű`, `kutya- vagy macskaféle`, `sör- és borfajták`, `tér- és időbeli`, `város- és falubeliek`
  - no LVL at the boundary (`kutyaszerű`, `faféle`) except `-né` (`Katonáné`)
  - boundary spelling: doubled digraphs written in full (`gipszszerű`), three identical letters with hyphen (`expressz-szerű`, `puff-féle`)
  - attach to pronouns unlike normal compounds (`efféle`, `ilyenszerű`, `annyifajta`)
  - assimilation not coalescent as inside words (`szigetszerű` like `Margitsziget`)
  - distinguished from derivational suffixes that fail the ellipsis test in one direction and harmonise (`*kék- vagy zöldes`)
- **Output**: flag `semi_word` in the suffix record (`DER-MODEL-002`); boundary treated as compound boundary by `ORT` and splitting
- **Examples**: listed
- **Exceptions**: `-beli` is not used after pronouns; `-né` lengthens
- **Predictability**: `rule`
- **Lexical input**: semi-word list
- **Interactions**: `DER-MODEL-001`, `DER-MODEL-014`, `DER-ADJ-012`..`DER-ADJ-016`, `DER-NOUN-024`, `ORT-TOG-041`, `ORT-HYPH-004`, `ORT-HYPH-007`
- **Sources**: [SMNy3 pp. 54-56 (Keszler: semi-words, `észszerű`, `szigetszerű`)], [SMNy3 pp. 123 (ellipsis 39, 40), 441 (property 2), 675 (Rebrus: compound-like `-szerű`, `-féle`, `-fajta`, `-beli`)], [AkH12 §92, §132]
- **Validation**: goldens
- **Confidence**: consensus

### DER-COMP-018 Letter sequences at compound boundaries
- **Statement**: At a compound (and semi-word, prefix) boundary the letters of each member are kept intact: identical doubled multi-character letters are written in full, and character sequences spanning the boundary are parsed per member, not as a multi-character letter.
- **Conditions**:
  - identical digraphs meeting: full doubling (`kulcscsomó`, `jegygyűrű`, `nagygyűlés`, `fénynyaláb`, `díszszázad`, `észszerű`, `mészszerű`), `WS-DIG-004`, `ORT-ASSIM-016`
  - look-alike sequences: `z+s` (`vízsugár`, `házsor`, `kézsebész`), `z+sz` (`vízszint`, `házszám`), `s+sz` (`kisszótár`), `g+gy` (`üveggyár`, `országgyűlés`), `n+ny` (`tannyelvű`), `c+cs` (`tánccsoport`), `c+s` (`táncsport`), `d+z` (`beszédzavar`); inventory and frequencies in `WS-DIG-006`
  - consequences: syllabification and hyphenation at the member boundary (`víz-su-gár`, `ORT-SPLIT-015`, `ORT-SPLIT-020`); pronunciation per member (`vízsugár` [z s], not [zs])
- **Output**: letter segmentation constrained by the compound analysis
- **Examples**: listed
- **Exceptions**: `ésszerű` legacy (AkH11)
- **Predictability**: `rule` given the analysis; `tendency` without it (`WS-DIG-008`)
- **Lexical input**: compound boundary
- **Interactions**: `WS-DIG-004`, `WS-DIG-006`, `WS-DIG-007`, `ORT-ASSIM-016`, `ORT-SPLIT-020`, `DER-COMP-022`
- **Sources**: [AkH12 §92, §226 f), §233-§236], [SMNy3 p. 56]
- **Validation**: attestation (`vízsugár` 4, `jegygyűrű` 1, `nagygyűlés` 5, `fénynyaláb` 0, `észszerű` 6), goldens
- **Confidence**: normative

### DER-COMP-019 Three identical consonant letters at a compound boundary
- **Statement**: When a member ending in a long consonant letter meets a member beginning with the same letter, the full spelling is kept and the members are joined with a hyphen; this differs from suffixation, where three letters are simplified to two.
- **Conditions**: compound or semi-word boundary; owned by `ORT-HYPH-001`; suffix case `ORT-ASSIM-017`
- **Output**: hyphenated compound; each further member of such a compound also gets a hyphen (`ORT-63-008`)
- **Examples**: `sakk-kör`, `balett-táncos`, `váll-lövés`, `hossz-szelvény`, `dzsessz-szerzemény`, `ott-tartózkodás`, `expressz-szerű`, `puff-féle`, `sakk-készlet-gyűjtemény`, `rossz-szívű` 'wicked'
- **Exceptions**: suffixation simplifies instead (`tollal`, `többe`, `hallak`; `ORT-ASSIM-017`); proper names hyphenate before suffixes (`Wittmann-né`, `ORT-HYPH-002`)
- **Predictability**: `rule`
- **Lexical input**: compound boundary
- **Interactions**: `ORT-HYPH-001`, `ORT-ASSIM-017`, `ORT-63-008`
- **Sources**: [AkH12 §61, §93, §140 (`sakk-készlet-gyűjtemény`)]
- **Validation**: goldens
- **Confidence**: normative

### DER-COMP-020 Writing compounds (pointers)
- **Statement**: Whether a combination is written solid, apart or hyphenated, and where a long compound is hyphenated, is decided by `ORT`; the morphological compound analysis of this chapter is an input to those rules.
- **Conditions**: meaning change and unmarked relations (`ORT-TOG-002`, `ORT-TOG-019`, `ORT-TOG-020`); structure-dependent writing (`ORT-TOG-021`..`ORT-TOG-025`); proper-name and acronym members (`ORT-HYPH-003`, `ORT-HYPH-005`); 6:3 rule (`ORT-63-001`..`ORT-63-009`); phrase + member (`ORT-TOG-045`, `ORT-TOG-046`)
- **Output**: see `ORT`
- **Examples**: `gyorsírás` vs `gyors írás`; `csapatzászló-avatás`; `Petőfi-szobor`; `hidegvíz-csap`
- **Exceptions**: -
- **Predictability**: `rule` (`ORT-TOG`, `ORT-63`, `ORT-HYPH`)
- **Lexical input**: member count, member simplicity, syllable count
- **Interactions**: `ORT-TOG`, `ORT-63`, `ORT-HYPH`
- **Sources**: [AkH12 §94-§141]
- **Validation**: goldens
- **Confidence**: normative

### DER-COMP-021 Compound splitting: candidate generation
- **Statement**: Engineering proposal (not a linguistic source rule): compound analysis of an unknown word generates all segmentations into a sequence of lexicon members followed by an inflected or derived last member, using a lexicon of free stems, bound first members, semi-words and verbal prefixes.
- **Conditions**:
  - input: surface word; letters segmented per `WS-DIG` with digraph ambiguity left open at candidate boundaries (`WS-DIG-006`, `WS-DIG-007`)
  - hyphen: a hard boundary (`WS-DIG-011`), but the right side may be a compound member (`Petőfi-szobor`, `sakk-kör`, 6:3 hyphen), a semi-word (`expressz-szerű`, `Eötvös-féle`) or an inflectional or derivational suffix after a name, acronym, digit or abbreviation (`ELTE-s`, `Wittmann-né`, `kft.-s`, `ORT-HYPH-002`, `ORT-ABBR`); all three readings are candidates
  - last member: must parse as stem + derivational and inflectional suffixes by the full nominal or verbal analyser (`NOM`, `VRB`, `DER`); category N, ADJ, participle, `-Ás`/`-Ó` noun, numeral or semi-word; category V only directly after a verbal prefix (`meg|int`)
  - non-final members: citation forms from the member lexicon (N, ADJ, NUM stems; derived stems with `-i`, `-(O)s`, `-Ó`, `-Ás`, `-kA`; bound first members `DER-PREF-005`; foreign combining forms `DER-PREF-006`; verbal prefixes only before a verb or a deverbal head; `leg-`, `legesleg-` only before a comparative)
  - stems of one or two letters (digraphs counted as one letter) only from a closed list of real members (`ág`, `ár`, `ész`, `fa`, `fő`, `hó`, `ív`, `kő`, `ló`, `mű`, `ős`, `út`, `víz`, ...), to limit noise
  - lexicon whole-word entries (lexicalised compounds, opaque compounds, derivatives) are also candidates
- **Output**: set of candidate analyses with member boundaries and categories
- **Examples**: `vízsugár` -> {`víz|sugár`}; `karóra` -> {`kar|óra`, `karó+ra` (sublative, not a compound)}; `legelőre` -> {`legelő+re`, `leg|előre` (superlative prefix)}; `megint` -> {`megint` ADV, `meg|int` V}; `rendőrfőtörzsőrmester` -> {`rend|őr|fő|törzs|őr|mester`, with lexicon members `rendőr`, `törzsőrmester`}
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: member lexicon with categories, bound first-member list, semi-word list, prefix list
- **Interactions**: `DER-COMP-022`, `DER-COMP-023`, `WS-DIG-006`, `WS-DIG-007`, `WS-DIG-011`, `ORT-HYPH-001`, `ORT-HYPH-002`, `ORT-HYPH-004`
- **Sources**: [SMNy3 pp. 449-453 (patterns)], [AkH12 §61, §93, §133, §139, §282 (hyphen uses)], [magyarispell py/hy.py (compound fields used for hyphenation, study only)], [morphdb grammar COMPOUND: NOM NOM (study)]
- **Validation**: oracle:hunmorph-foma, oracle:magyarispell, goldens
- **Confidence**: engineering proposal; the linguistic inputs (patterns, member forms) are consensus

### DER-COMP-022 Compound splitting: constraints
- **Statement**: Engineering proposal: a candidate compound analysis is rejected if it violates member-form, pattern, boundary-spelling or suffix-precedence constraints.
- **Conditions**: (reject if any holds)
  - non-final member is inflected (case, plural, possessive) unless the whole string is a listed lexicalised compound (`DER-COMP-006`)
  - non-final member shows LVL or a bound stem alternant (`*almáfa`, `*kövkemény`) unless listed (`tavirózsa`, `havilottó`); semi-word `-né` is exempt from the LVL check (`Katonáné`)
  - pattern not productive and not listed: `V + N` (`*olvaskönyv`), `PRO + N`, `N + POSTP`, `ADV + N` other than prefix + deverbal head (`DER-COMP-003`)
  - a derivational suffix analysis of the final string competes with a member analysis: prefer the suffix for non-semi-word suffixes (`szépség` is `szép+ség`, not `szép|ség`; `kertész` not `kert|ész`), prefer the member for semi-words (`kutyaszerű` = `kutya|szerű`)
  - boundary inside a default digraph parse: allowed only if both sides are members and the split letters are those of `WS-DIG-006` (`víz|sugár` over `ví|zsugár`)
  - identical digraphs at the boundary: the split requires full doubling (`kulcs|csomó` from `kulcscsomó`); a simplified spelling at a compound boundary (`kulcsomó`, legacy `ésszerű`) is analysed only in repair or legacy mode (`REP`, `DER-MODEL-014`)
  - three identical consonant letters: a compound boundary there requires the hyphen (`sakk-kör`); unhyphenated `sakkör` is analysed only in repair mode; a solid split between a geminate and the same letter (`toll|lal`) is a suffix boundary, not a compound boundary (`ORT-HYPH-001`, `ORT-ASSIM-017`)
  - verbal prefix followed by a non-verbal, non-deverbal head (`*meg|asztal`); one-syllable prefixes are not counted as separate members for 6:3 (`ORT-63-004`)
  - numeral compounds are left to `PND`
- **Output**: filtered candidate set; an unlisted solid `ADJ + N` candidate is not rejected but flagged `unlisted_adj_n` (morphotactically productive, but only lexicalised `ADJ + N` combinations are solid, SMNy3 p. 455, `ORT-TOG-002`; compositional ones are phrases written apart)
- **Examples**: `szépség` (suffix); `kutyaszerű` (semi-word member); `almafa` accepted, `*almáfa` rejected; `folyóirat` = `folyó|irat`; `kerékpár` = `kerék|pár`; `zöldasztal` accepted only as a listed compound
- **Exceptions**: listed lexical entries override
- **Predictability**: `rule`
- **Lexical input**: as `DER-COMP-021`; lexicalised compound list
- **Interactions**: `DER-COMP-003`, `DER-COMP-005`, `DER-COMP-006`, `DER-COMP-011`, `DER-COMP-017`, `DER-COMP-018`, `DER-COMP-019`, `DER-COMP-023`, `ORT-ASSIM-016`, `ORT-HYPH-001`, `ORT-TOG-002`
- **Sources**: [SMNy3 pp. 107-110, 449-455], [AkH12 §61, §92, §93, §95], [ATT (boundary letter counts reported in `WS-DIG-006`)]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus for the linguistic constraints; engineering proposal for suffix-first precedence, the `ADJ + N` filter and the repair-mode handling

### DER-COMP-023 Compound splitting: ranking and ambiguity output
- **Statement**: Engineering proposal: surviving analyses are ranked by lexical listing, number of members, member frequency and pattern productivity; genuine ambiguities are returned with all readings and scores rather than resolved.
- **Conditions**: (ordering criteria, applied in order; a later criterion only breaks ties of the earlier ones)
  1. whole word listed in the lexicon (simplex, lexicalised compound, derivative): rank first; compositional readings are still returned, flagged
  2. fewer members preferred (`rendőr|főtörzsőrmester` over six single members), with listed sub-compounds counted as one member
  3. productive pattern (`N + N`, listed `ADJ + N`) over rare pattern (`N + ADJ`, `ADJ + ADJ`) and over flagged `unlisted_adj_n`
  4. higher geometric mean of member corpus frequencies (attestation data; the mean, not the product, so that member count is decided only by criterion 2)
  5. if the input contains a hyphen between two compound members, analyses whose main boundary (`DER-COMP-004`) is at the hyphen are kept first (`ORT-63-006`)
- **Output**: ranked list; flag ambiguous if the top two differ in category or segmentation and both are lexically supported
- **Examples**: `karóra` 'wristwatch' (`kar|óra`, listed) vs `karó+ra` 'onto a stake' (inflected simplex): both returned, context decides; `legelőre` (`legelő+re` 'onto the pasture' vs `leg|előre` 'foremost'); `megint` 'again' vs `meg|int` 'admonish' (hyphenation `me-gint` vs `meg-int`, AkH12 §233); `szárazföld` (lexicalised, solid) vs phrase `száraz föld`
- **Exceptions**: -
- **Predictability**: `tendency` (ranking)
- **Lexical input**: frequency list; lexicon
- **Interactions**: `DER-COMP-004`, `DER-COMP-021`, `DER-COMP-022`, `ORT-SPLIT-015`, `ORT-63-006`
- **Sources**: [AkH12 §95, §139, §233], [SMNy3 p. 108 (`faasztalka` double analysis)]
- **Validation**: goldens (ambiguity set), attestation
- **Confidence**: engineering proposal (ranking); the ambiguity facts are `normative` (AkH12 §233)

### DER-COMP-024 Backformed N + V and ADV + V compounds
- **Statement**: Verbs of the form N + V (`gyógykezel`, `apróhirdet`) are backformations from `-Ás` compounds and are lexical; ADV + V with a verbal prefix or adverbial prefix are prefixed verbs, not compounds.
- **Conditions**: listed N + V verbs: `apróhirdet`, `bájcseveg`, `gyógykezel`, `képzettársít`, `tényfeltár`, `városnéz`, `házkutat`, `bűnüldöz`, `idegenvezet`, `ismeretterjeszt`, `agymos`; they are often defective (`?gyógykezelem`)
- **Output**: lexicon entries; generation of new N + V verbs disabled
- **Examples**: `gyógykezelés` -> `gyógykezel`; `apróhirdetés` -> `apróhirdet`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `DER-COMP-003`, `DER-PREF-001`
- **Sources**: [SMNy3 p. 451 (backformation)], [KVF98 pp. 377-378]
- **Validation**: attestation (`apróhirdetés` 5, `házkutatás` 6)
- **Confidence**: consensus

### DER-COMP-025 Numerals and pronoun compounds (pointer)
- **Statement**: Compound numerals (`kétezer-kétszázharmincnégy`) and pronoun formations (`valaki`, `semmilyen`, `ugyanez`, `ezelőtt`) are compounds by form but are generated by `PND` and `ORT-TOG-029`..`ORT-TOG-033`.
- **Conditions**: numeral or pronoun members
- **Output**: see `PND`
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: -
- **Interactions**: `PND`, `ORT-TOG-029`..`ORT-TOG-033`, `ORT-HYPH-012`
- **Sources**: [SMNy3 p. 451], [AkH12 §121-§124]
- **Validation**: goldens
- **Confidence**: normative

### DER-COMP-026 Coordination ellipsis of compound members
- **Statement**: A member shared by coordinated compounds may be elided in all but one conjunct and is marked with a hyphen; the ellipsis is possible only for word-level members and semi-words, not for bound members or suffixes.
- **Conditions**: shared last member (common): `cserép- és téglagyár`, `regény- és versírás`, `bükk-, mogyoró- és nyírfafélék`; shared first member (rarer): `férfiing és -nadrág`, `virágföld és -tápszer`; not with bound members: `*utófájdalom és -hatás`, `*alvezér vagy -főnök` (but `bel- és külügy`, `mellék- és főépület` acceptable); not with suffixes `*kéz- vagy lábon`
- **Output**: analysis restores the elided member for each conjunct
- **Examples**: listed
- **Exceptions**: acceptability for bound members varies (`bel- és külügy`)
- **Predictability**: `rule` + `tendency`
- **Lexical input**: member type (`word`, `semi_word`, `bound`)
- **Interactions**: `ORT-HYPH-007`, `DER-COMP-017`, `SYN`
- **Sources**: [SMNy3 pp. 110, 441 (property 1, 2)], [SMNy3 pp. 476-482 (9.6 ellipsis conditions)], [AkH12 §132, §263 c)]
- **Validation**: goldens
- **Confidence**: consensus; bound-member acceptability `contested`

## DER-LEX

### DER-LEX-001 Lexicalised derivatives are lexicon entries
- **Statement**: A derivative (or compound) must be stored when its meaning is not derivable from base and suffix, when its base is not an existing word, when its suffix is unproductive, or when its form deviates from the suffix rule; it keeps a segmentation for analysis.
- **Conditions**: (store if any holds)
  - non-compositional meaning: `újság`, `igazgató`, `adós`, `ebédlő`, `helyettes`, `szemtelen`, `látogat`, `beszélget`, `mosogat`, `hallgat`, `fosztogat`, `írás` 'script', `képzés` 'word formation', `járóka`, `estike`
  - suffix class `R` or `L` (`DER-MODEL-012`): all `-mÁny`, `-vÁny`, `-At`, `-Ász`, `-nOk`, `-Ár`, `-(O)nc`, `-zAt`, `-(O)dA`, `-ékOny`, `-AtAg` words
  - form deviation from the suffix rule: `irat` (shortening), `újságja`, `havonta`, `gondatlan`, `csatár` (truncation)
  - converted nouns from adjectives (`vörös` N)
- **Output**: lexicon entry with fields: lemma, category, segmentation, lexicalised flag, sense(s), overridden properties
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `rule` (storage criteria)
- **Lexical input**: lexicon
- **Interactions**: `DER-MODEL-012`, `DER-LEX-005`, `DER-LEX-007`
- **Sources**: [SMNy3 pp. 107, 111, 114-116 (3.3, 3.4, 3.5, rules 24-26)], [KVF98 p. 351]
- **Validation**: oracle:MÉKsz (dictionary headword comparison), goldens
- **Confidence**: consensus

### DER-LEX-002 Lexical blocking
- **Statement**: An existing word with the meaning of a potential regular derivative blocks the derivative in generation; the blocked form is not ungrammatical (marked `#`) and is accepted in analysis with a `blocked` flag; blocking is a tendency and can be overridden by meaning split.
- **Conditions**:
  - blocker is an existing word (root, unproductive derivative, or rival derivative) with the same meaning
  - blocker frequency matters: frequent unproductive forms block productive ones (`üldögél` blocks `#ülöget`)
  - diminutives are not subject to blocking (ad hoc parallel forms remain possible)
- **Output**: generation: emit the blocker; analysis: accept blocked form, flag blocked, link to blocker
- **Examples**: `-Ás`: `#szeretés` / `szeretet`, `#gyűlölés` / `gyűlölet`, `#félés` / `félelem`, `#hívés` / `hit`, `#tisztelés` / `tisztelet`; `-sÁg`: `#szorgalmasság` / `szorgalom`, `#egészségesség` / `egészség`, `#bánatosság` / `bánat`; `-(O)s` occupation: `#teniszes` / `teniszező`, `#diszkoszos` / `diszkoszvető`, `#kenyeres` / `pék`, `#húsos` / `hentes`, `#cipős` / `cipész`, `#zongorás` / `zongorista`, `#egyetemes` / `egyetemista`; `-(O)gAt`: `#szaladgat` / `szaladgál`, `#harapgat` / `harapdál`, `#heverget` / `heverészik`, `#ülöget` / `üldögél`, `#állogat` / `álldogál`; `-sÁg` on nouns: `#katonaság` 'being a soldier' / `katonáskodás`; `-Ós`: `#fáradós` / `fáradékony`
- **Exceptions**: synonym coexistence (`síel ~ sízik`), gradual replacement (`vacsorál` -> `vacsorázik`), meaning split (`DER-LEX-004`)
- **Predictability**: `tendency`
- **Lexical input**: blocking links in the lexicon
- **Interactions**: `DER-LEX-004`, `DER-LEX-006`, `DER-DIM-004`
- **Sources**: [SMNy3 pp. 106, 111, 119-120 (3.7, rule 31)], [SMNy3 pp. 128, 148-149, 151, 158], [KVF98 p. 360 (879)]
- **Validation**: attestation (`hívés` unattested; `hit` 15), goldens
- **Confidence**: consensus (principle); `tendency` (individual links)

### DER-LEX-003 Homonymy avoidance
- **Statement**: A derivative that would coincide with an inflected form or an existing derivative of the same base is avoided in favour of a rival suffix; the principle is not absolute.
- **Conditions**:
  - `-i` on vowel-final bases whose possessed-plural 3sg form is identical: avoided for `-ció` nouns (`privatizációs`, not `*privatizációi` as the adjective), and for some native `-Ó` nouns (`folyói`), tolerated for `bírói`, `költői`
  - verbaliser choice avoiding an existing verb: `kertez` (not `kertel`)
  - `-At` result nouns avoided on `t`-final verbs where they would coincide with causatives (`*építet`, use `építmény`)
- **Output**: rival suffix in generation
- **Examples**: listed
- **Exceptions**: `bírói`, `költői` (homonymous with `bírói`, `költői` 'his judges/poets')
- **Predictability**: `tendency`
- **Lexical input**: paradigm of the base
- **Interactions**: `DER-ADJ-002`, `DER-VERB-001`, `DER-NOUN-009`
- **Sources**: [SMNy3 pp. 120-121 (rule 32), pp. 137-138, 167]
- **Validation**: goldens
- **Confidence**: consensus

### DER-LEX-004 Doublets and meaning split
- **Statement**: Where two derivatives of one base with rival suffixes both exist, they either coexist as synonyms or split in meaning; each is a lexicon entry with its sense.
- **Conditions**: pairs from rival suffixes or allomorphs
- **Output**: lexicon entries with senses
- **Examples**: `okoz` / `okol`; `soroz` / `sorol`; `szemez` / `szemel`; `gondatlan` / `gondtalan`; `tárgyatlan` / `tárgytalan`; `lelketlen` / `lélektelen`; `végetlen` / `végtelen`; `(el)sötétül` / `(be)sötétedik`; `omlós` / `omlékony`; `tanulós` / `tanulékony`; `utcai` / `utcabeli`; `hegyi` / `hegyes`; `egyhangúlag` / `egyhangúan`; `becsülés` / `becslés`; `orvosné` / `orvosnő`; `editel` / `editál`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon senses
- **Interactions**: `DER-LEX-002`, `NOM-POSS-009`
- **Sources**: [SMNy3 pp. 120, 151, 159-160], [SMNy3 p. 675 (privative doublets)]
- **Validation**: goldens
- **Confidence**: consensus

### DER-LEX-005 Opaque derivatives and phantom stems
- **Statement**: Words containing a historically identifiable but synchronically unproductive suffix on a non-existent base (phantom stem) are monomorphemic lexicon entries; they are reanalysed as derivatives only when the base exists and the suffix is productive.
- **Conditions**:
  - phantom base + unproductive suffix: monomorphemic (`visel`, `szürcsöl`, `horzsol`, `vonszol`; `alvad`, `fárad`; `zsibong`, `mereng`; `sivatag`; `ajándék`, `játék`)
  - existing base + productive suffix: reanalysed (`tapsol` = `taps+ol`, `roncsol` = `roncs+ol`, `dörzsöl` = `dörzs+öl`)
  - existing base + unproductive suffix with transparent meaning: segmentable, lexical (`gyártmány`, `tudomány`)
  - bound Latinate stems: segmentable by correspondence sets (`DER-ADJ-023`)
- **Output**: lexicon entry with segmentation = none, etymological, or synchronic
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `rule` (classification) + `lexical`
- **Lexical input**: base existence; suffix productivity class
- **Interactions**: `DER-LEX-001`, `DER-VERB-017`, `DER-VERB-019`
- **Sources**: [SMNy3 pp. 115-116 (23, rules 25, 26, phantom stems)], [SMNy3 pp. 112-113 (`-ong` unproductive)]
- **Validation**: goldens
- **Confidence**: consensus

### DER-LEX-006 Existing, potential and occasional words
- **Statement**: The engine distinguishes existing words (listed or corpus-attested), potential words (output of a productive rule, including blocked forms) and occasional analogical formations (following a pattern, not a rule); only the first two are generated, all three are analysable.
- **Conditions**:
  - existing: lexicon or attestation above threshold
  - potential: rule output with productivity `P` or `P-dom` and satisfied conditions (`fénytelenít`, `orvoskodik`, `utópisztikusság`)
  - occasional/analogical: rule violation or pattern extension (`galambka`, `cserépke`, `költönc`, `nyomoronc`, `forrasztmány`, `vasalmány`, poetic `hamaritánus`)
  - nonce test words for productivity (`csereg`, `gizon`, `zsábor`) are potential words
- **Output**: status flag existing | potential | blocked | occasional
- **Examples**: listed
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: lexicon, frequency list
- **Interactions**: `DER-MODEL-012`, `DER-LEX-002`
- **Sources**: [SMNy3 pp. 111-113 (3.4), pp. 117-118 (analogy)]
- **Validation**: attestation, goldens
- **Confidence**: consensus

### DER-LEX-007 Inflection of lexicalised derivatives
- **Statement**: Lexicalised derivatives inflect by the output properties of their suffix (`DER-MODEL-010`, `DER-MODEL-011`) unless the entry overrides a property.
- **Conditions**: lexicon entry with segmentation
- **Output**: default properties from the suffix record; overrides from the entry
- **Examples**: `újság` (override poss3 `-ja`); `irodalom` (suffix `-alom`: lowering, epenthetic); `hatalmas` (`-(O)s` lowering: `hatalmasak`); `igazgató` (noun `-Ó`: `igazgatók`, `igazgatója`)
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: overrides
- **Interactions**: `DER-MODEL-010`, `DER-MODEL-011`, `NOM-STEM-001`
- **Sources**: [SMNy3 pp. 509-510], [T04 pp. 160-161]
- **Validation**: goldens, oracle:hunmorph-foma
- **Confidence**: consensus

### DER-LEX-008 Lexicalised inflected forms are not derivation
- **Statement**: Inflected forms that have become adverbs, postpositions or prefixes lose their inflectional analysis and are lexicon entries of the new category; they may then take derivational suffixes.
- **Conditions**: grammaticalised case forms
- **Output**: lexicon entries: `hátra` (`hát` + sublative) ADV/prefix; `agyon` (`agy` + superessive) prefix; `délben`, `nyáron` time adverbs remain analysable as cases; `nagyban` -> `nagybani`; `körül`, `mellett` postpositions (`PND`)
- **Examples**: `hátra` -> `hátrább`, `hátrány`; `nagyban` -> `nagybani`; `mellett` -> `melletti`
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: lexicon entries
- **Interactions**: `DER-MODEL-003`, `VRB-PREV-001`, `PND`
- **Sources**: [SMNy3 p. 107 (`hátra`, `agyon`)], [SMNy3 p. 668 (`nagybani`)], [KVF98 p. 373 (postpositions from case forms)]
- **Validation**: goldens
- **Confidence**: consensus

## Open questions
1. Full list of the 15 lexical linking-vowel `-sÁg` forms (`DER-NOUN-002`): SMNy3 gives the count and the 6 geminate members; candidates with single `s` (`sokaság`, `rosszaság`, `nyereség`, `veszteség`, `vereség`, `eleség`) are not confirmed as members of Rebrus' count; extract from a dictionary (MÉKsz) and check AkH12 spelling (`frissesség` vs `*frisség`).
2. Status of `-mA` (`szakma`, `kelme`) as a suffix (`DER-NOUN-026`): no primary source read confirms it; needs a historical-morphology source (Tompa 1961, Keszler: Magyar grammatika).
3. Harmony of `-né` after back stems (`Szabónénak ~ ?Szabónének`) is reported as vacillating (F13 via `PH-HARM-019`); corpus counts of case forms after `-né` are needed to set per-context probabilities.
4. Value of LVL before `-ntA` after `a/e`-final bases (`DER-MODEL-005`) is taken from morphdb only; no attested base exists in the corpus list (`óránta` unattested).
5. Whether the stop condition blocking `-kA` (`DER-DIM-001` rule 7) includes palatal stops `ty`, `gy` is inferred, not stated; test with `hegyke ~ hegyecske`, `kalapka`, `bogárka` in a larger corpus.
6. Complete closed lists are not published in the sources read: `-mÁny` (89), `-vÁny` (78), `-(O)dA` (30), `-nOk` (36), `-(O)nc` (15), `-AtAg` (24), lexicalised privatives with `-atlan` (36), `-lAg` adverbs (46); compile from Rebrus' counts plus dictionary extraction.
7. `-i` on shortening and v-stems (`heti`, `havi`, `tavi`, `művi` vs `kézi`, `vízi`, `úti`, `tűzi`, `nyári`, `téli`): the list of 4 lexically shortened forms may be incomplete (`lovi`, `kövi`, `hói` do not occur; AkH12 §27 b) confirms unshortened `kézi`, `vízi`, `úti`, `tűzi`).
8. Productivity and interpretation of `-sdi` (`katonásdi`) and of the child-language `-ikA` forms have no corpus support in the 250k list; decide whether to generate them.
9. Epenthetic nouns before `-(O)z` (`tükröz` vs `sátoroz`) lack a predictive condition; a lexicon-wide survey is needed.
10. Semantic subclasses for `-Odik`/`-ít` on privative adjectives (`elsótlanodik`, `portalanít` vs `*lábatlanít`) are unresolved in SMNy3 and here.
11. `-An` after `-i` adjectives (`zeneien`, `anyaian`): SMNy3 excludes productive formation, KVF98 lists examples; attestation of such forms in the corpus is nil, so generation is disabled pending data.
12. Compound splitting ranking (`DER-COMP-023`) is an engineering proposal; it needs evaluation against an annotated compound set (oracles hunmorph-foma and magyarispell disagree on many long compounds).
13. Semi-word status of `-beli` and `-mentes` for hyphen and 6:3 counting is not settled by AkH12 (only `-fajta`, `-féle`, `-forma`, `-nemű`, `-rét`, `-rétű`, `-szerű` are named as utótag); `ORT` must decide how they count.
14. SMNy3 image tables 33 (p. 694), 34-38 (pp. 122-123) and 188 (p. 788) were checked against the PDF pages (verification 2026-09-15). Remaining gap: Rebrus' table 33 does not list `-kA`, verbal `-Ul`, `-(O)dA` or `-mÁny`; their epenthetic behaviour (`bokorka`, `söpörget`) rests on morphdb and examples.
15. `-lAt` (`javaslat`) is listed by Rebrus (SMNy3 p. 677) but not modelled here; decide between a lexical `DER-NOUN` record and monomorphemic storage.

## Sources used
All keys are in [sources.md](sources.md). Reliability notes for this chapter:
- `Tompa61`, `Papp69`: not accessed, cited through `SMNy3` and `KVF98`.

## Terms
See [glossary.md](glossary.md).
