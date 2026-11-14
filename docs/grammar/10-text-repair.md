# 10 Text repair

Scope: detection and repair of damage that Hungarian text acquires between the writer and the corpus: character encoding confusions (mojibake), legacy font and glyph substitutions, PDF extraction artefacts, spacing damage, diacritic loss, typical native spelling errors, keyboard slips, OCR confusions and pre-reform orthography; plus the generation side (controlled noise producing repair training pairs that mirror each damage type) and the document-level quality signals that decide whether a repair is trusted. Normative spelling itself is specified in `ORT`, punctuation and spacing norms in `PUN`, character inventory and Unicode normalisation in `WS` (`WS-ENC`, `WS-NORM`), agreement and usage lint in `SYN` (the syntax lint (out of scope)). This chapter only specifies how damaged input is recognised and mapped back to the form those chapters define.

General conventions for every rule in this chapter:
- A repair is a function from a text span to a replacement span plus provenance (rule ID, version, original bytes); it never edits in place without recording the original [PC-KB repair.rs].
- Every repair has three outcomes, not two: `repair` (evidence sufficient), `keep` (evidence says no damage) and `flag` (damage evident, repair unsafe; the span or document is labelled, not rewritten) [PT-CLEAN "The font shift has three outcomes"].
- Detectors abstain when their evidence cannot run (too short, wrong language); abstention is never a verdict of damage [PT-CLEAN "A test that cannot run must abstain"].
- "Oracle" means the morphological analyser/generator of this library (`accepts(word) -> bool`, `analyses(word)`), "lexicon" the word-form list, "frequency" the corpus frequency bucket of `frequency.ron` (half-octave buckets; `-` unattested). Frequency lists contain damaged forms too (`fõ` bucket 13, `elõtt` 11, `egesz` 6), so attestation of a damaged form is never evidence that it is correct; always compare the damaged and repaired candidate.
- Examples show damaged input `->` repaired output. Bucket numbers after examples are `frequency.ron` buckets.

## Area index

| Area | Topic | Rules |
|---|---|---|
| `ENC` | encoding damage: mojibake under every code page confusion, double UTF-8, lossy transcoding, escapes, 7-bit conventions, mixed documents | REP-ENC-001 .. REP-ENC-016 |
| `FONT` | font and glyph substitutions: tilde and circumflex for double acute, other homoglyphs, shifted fonts, private-use and Symbol fonts, ligatures, invisible and combining characters | REP-FONT-001 .. REP-FONT-012 |
| `SPACE` | letterspacing, reversed runs, per-syllable fragments, line-end hyphens and doubled digraphs, typographic spaces, missing and spurious spaces, fusion and splits | REP-SPACE-001 .. REP-SPACE-012 |
| `DIAC` | diacritic loss detection, stripping classes, candidate generation, disambiguation, ambiguity typology, harmony-driven suffix restoration, partial restoration, guards | REP-DIAC-001 .. REP-DIAC-012 |
| `SPELL` | typical native spelling errors and their detectors | REP-SPELL-001 .. REP-SPELL-018 |
| `KEY` | keyboard layouts, adjacent-key slips, wrong active layout, y/z swap, missing keys, dead-key residue, typing noise | REP-KEY-001 .. REP-KEY-008 |
| `OCR` | OCR confusions, priorities, quotes, fusion, old prints, real-word OCR errors | REP-OCR-001 .. REP-OCR-007 |
| `ARCH` | pre-reform and archaic orthography: detection and optional normalisation | REP-ARCH-001 .. REP-ARCH-009 |
| `NOISE` | generation of damage for training pairs, per damage type, with parameters | REP-NOISE-001 .. REP-NOISE-011 |
| `QUAL` | quality signals, thresholds, pass ordering | REP-QUAL-001 .. REP-QUAL-011 |

Reference tables (full character mappings, keyboard tables, confusion lists) are collected in `## Reference tables` and cited as `Table T-...`.

## ENC: encoding damage

### REP-ENC-001 Legacy byte values of the Hungarian letters
- **Statement**: The 18 Hungarian accented letters have fixed byte values in each legacy code page; every encoding repair in this area is derived from these values and not from hand-written pair lists.
- **Conditions**:
  - Applies to all detectors and fixers in `ENC` and to the generators in `REP-NOISE-002`.
  - Code pages in scope: ISO-8859-2, windows-1250, CP852, MacCE (Apple Central European), CWI-2, ISO 646-HU (ISO-IR-086), windows-1252, ISO-8859-1, CP437, CP850, MacRoman, UTF-8.
- **Output**: Table T-ENC-1 (byte per letter per code page). Facts to keep in mind: (a) ISO-8859-2 and windows-1250 agree on all 18 Hungarian letters; they differ only in 0x80-0xBF (47 byte values): windows-1250 fills 0x80-0x9F with typographic marks and `Š Ś Ť Ž Ź š ś ť ž ź`, moves `Ą ą Ľ ľ` to other slots of 0xA0-0xBF and puts Latin-1 symbols (`© « ® ± µ ¶ ·`) where ISO-8859-2 has letters; (b) ISO-8859-1/windows-1252 agree with ISO-8859-2 on 14 of the 18 letters and differ exactly on `ő ű Ő Ű` (0xF5 0xFB 0xD5 0xDB), which is why Latin-2/Latin-1 confusion damages only the double acute letters; (c) CP852 and CWI-2 both keep the CP437 positions of `á é í ó ö ú ü É Ö Ü` but place `ő ű Á Í Ó Ő Ú Ű` at different bytes; (d) ISO 646-HU has only `Á É Ö Ü á é ö ü` (0x40 0x5B 0x5C 0x5D 0x60 0x7B 0x7C 0x7D) and the double acute accent at 0x7E; `í ó ú ő ű` and capitals do not exist in it.
- **Examples**: `ő` = U+0151 = UTF-8 `C5 91` = ISO-8859-2 `F5` = CP852 `8B` = MacCE `CE` = CWI-2 `93`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: restates `WS-ENC-002` with the additional columns needed for repair; REP-ENC-002 .. REP-ENC-011 consume the table.
- **Sources**: [UNI-MAP CP1250.TXT, 8859-2.TXT, CP852.TXT, CENTEURO.TXT]; [ENWIKI-CWI2]; [ENWIKI-646 HU row]; [MS-BF bestfit1252.txt]; [PY-CODECS] cross-checked against [UNI-MAP] with 0 mismatches in 0x80-0xFF for all eight vendor tables.
- **Validation**: `goldens` (round-trip every letter through every table); `oracle:codec` (byte tables regenerated from the Unicode mapping files).
- **Confidence**: `normative` (published code tables).

### REP-ENC-002 UTF-8 decoded as windows-1252 or ISO-8859-1
- **Statement**: Hungarian text stored as UTF-8 and decoded as windows-1252 or ISO-8859-1 shows every accented letter as a two-character sequence starting with `Ã` (U+00C3, lead byte C3) or `Å` (U+00C5, lead byte C5), and punctuation `„ ” – …` as three-character sequences starting with `â€`; the repair re-encodes with the wrong table and decodes as UTF-8.
- **Conditions**:
  - Signature (character level): `Ã` followed by one of the continuation images in column "as windows-1252" of Table T-ENC-2 (`¡ © ³ ¶ º ¼ ‰ “ – š œ`, U+00AD for `í`, and the C1 controls U+0081 U+008D for `Á Í`), or `Å` followed by `‘ ± °` or U+0090, or `â€` followed by `ž ™ “ ” ¦ ˜ œ` or U+009D, or `Â` followed by a Latin-1 symbol (`«»°§` NBSP).
  - General form (all languages): a code point in U+00C2..U+00F4 whose UTF-8 byte image (C2..F4) is a valid lead byte, followed by 1..3 code points whose windows-1252 byte images are all in 0x80..0xBF, forming a valid UTF-8 sequence.
  - WHATWG decoders map windows-1252 bytes 0x81 0x8D 0x8F 0x90 0x9D to U+0081 U+008D U+008F U+0090 U+009D; strict vendor decoders fail on them or emit U+FFFD. Both variants occur; the reverse map must accept C1 controls as those bytes, and must accept `?` or U+FFFD in that slot as a lost byte (see Exceptions).
  - ISO-8859-1 decoding differs from windows-1252 only in 0x80..0x9F: `Å‘` (1252) vs `Å` + U+0091 (8859-1) for `ő`; `â€ž` vs `â` + U+0080 + U+009E for `„`.
- **Output**:
  - Repair per maximal damaged run: encode the run with windows-1252 (C1 controls to their byte value), decode as UTF-8 strictly; if strict decode fails, try ISO-8859-1; accept only if the decode succeeds and the Hungarian plausibility score (REP-ENC-016) of the result is higher than of the input.
  - Apply per run, not per document, when the document is mixed (REP-ENC-015).
  - Iterate while the signature remains (double encoding, REP-ENC-005), at most 3 times.
- **Examples**: `KÃ©rem` -> `Kérem`; `Å‘sz` -> `ősz`; `tÅ±z` -> `tűz`; `Ã–rkÃ©ny` -> `Örkény`; `Ã<U+0081>rpÃ¡d` -> `Árpád`; `â€žIgenâ€<U+009D>` -> `„Igen”`.
- **Exceptions**:
  - A lost byte: `Ã?` or `Ã` + U+FFFD where the second byte was 0x81/0x8D/0x90/0x9D (strict decoder upstream). Only `Á Í Ő` (and `”` 3rd byte 9D) produce these; repair to the letter only if exactly one candidate makes the word acceptable to the oracle (`Ã?rpÃ¡d` -> `Árpád`), else `flag`.
  - Legitimate `Ã`, `Å`, `Â` in other languages: Portuguese capitals `SÃO`, `NÃO`, `MÃE` (followed by an uppercase ASCII letter, not by a continuation image), Scandinavian `Åland`, `Ångström` (followed by ASCII letter), French `Âge`. The signature requires the continuation image, so these do not match; guard additionally that the run decodes to a valid UTF-8 sequence.
  - Code, regex or escaped test data that discuss mojibake (e.g. this chapter): suppress in sources declared as code or documentation of encodings.
- **Predictability**: `rule`
- **Lexical input**: none (oracle only for lost-byte ambiguity)
- **Interactions**: repair side of `WS-ENC-006`; decoding of declared bytes stays with `WS-ENC-011`; before REP-ENC-006 (a Latin-2 repair on mojibake would corrupt it); before all `FONT`, `SPACE`, `DIAC` repairs; REP-QUAL-004 counts residue.
- **Sources**: [UNI-MAP CP1252.TXT]; [WHATWG-ENC index-windows-1252, labels]; [PT-CLEAN encoding.py `_utf8_latin1`]; [PC-KB quality.rs `misread`].
- **Validation**: `goldens` (Table T-ENC-2 rows round-trip); `attestation` (repaired words accepted by oracle); negative set: clean Portuguese/Swedish/French text unchanged.
- **Confidence**: `normative` (mechanism), `consensus` (signature thresholds).

### REP-ENC-003 UTF-8 decoded as windows-1250 or ISO-8859-2
- **Statement**: On Hungarian systems the default ANSI code page is windows-1250, so UTF-8 misdecoding in Hungarian sources most often shows `Ă` (lead byte C3) and `Ĺ` (lead byte C5) sequences; the repair re-encodes with windows-1250 (or ISO-8859-2) and decodes as UTF-8.
- **Conditions**:
  - Signature windows-1250: `Ă` followed by `ˇ © ł ¶ ş Ľ ‰ Ť “ – š ś`, U+00AD or U+0081, or `Ĺ` followed by `‘ ± °` or U+0090 (Table T-ENC-2 column "as windows-1250").
  - Signature ISO-8859-2: `Ă` followed by `Ą Š ł ś ş ź`, U+00AD or C1 controls, `Ĺ` followed by U+0091 `ą °` or U+0090.
  - windows-1250 undefined bytes (WHATWG maps to C1): 0x81 0x83 0x88 0x90 0x98.
  - Prior between the two families, **measured** 2026-09-16: `P(windows-1252 | mojibake) = 0.30, 95% CI [0.28, 0.33]` for Hungarian text, i.e. **windows-1250 is about 2.3x more common** (311 only-1252 against 710 only-1250 documents of 1,021 carrying a signature, over 9.64 M Hungarian documents; 4,720 against 9,082 affected Hungarian tokens). The direction never flips under the hit threshold (0.429 at >= 1 hit, 0.305 at >= 2, 0.261 at >= 3, 0.243 at >= 10) and per source it is the crawls that carry it. **A corpus-wide figure that does not exclude English reads 0.89 and points the other way**, because the English sources carry most of the mojibake.
- **Output**: As REP-ENC-002 with windows-1250 then ISO-8859-2 as the re-encoding tables. Try both families (1252 and 1250) and keep the result with the highest plausibility score; a single document may contain both if assembled from several sources.
- **Examples**: `KĂ©rem` -> `Kérem`; `szĂłtĂˇr` -> `szótár`; `Ĺ‘sz` -> `ősz`; `hasznĂĄlatĂĄhoz` (ISO-8859-2) -> `használatához`; `tĹ±z` -> `tűz`.
- **Exceptions**:
  - Romanian `Ă` (`ĂN`, `ĂLA` in capitals) and Slovak `Ĺ` (`ĹUBOŠ` rare) are followed by uppercase ASCII or space, not continuation images.
  - Legitimate `ł` after `Ă` does not occur in any Latin orthography; `Ăł` is a safe signature.
  - Prevalence caveat: mojibake reaches only 0.011% of Hungarian documents (1,021 of 9.64 M). The ratio above is well determined; the absolute rate is not a guide to what an uncleaned source looks like, because the crawl's own upstream has already fixed an unknown amount. Actionable gap found on the way: the cleaning pipeline's `_UTF8_AS_LATIN1` pattern has no cp1250 branch at all - no `Ă`, no `Ĺ` - so the family that is 70% of Hungarian mojibake is not detectable by the current cleaner even where it is declared.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: same position as REP-ENC-002; REP-ENC-015 decides per run.
- **Sources**: [UNI-MAP CP1250.TXT, 8859-2.TXT]; [WHATWG-ENC index-windows-1250]; [PY-CODECS] (signature lists computed); [PT-CLEAN]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens` (Table T-ENC-2); negative set Romanian and Slovak text. Signatures generated from the code-page tables rather than hand-listed, and validated against this rule's own examples and negative set (`GyÅ'rben`, `KÃ©rem` match 1252 only; `KĂ©rem`, `Ĺ'sz`, `tĹ±z` match 1250 only; `SÃO PAULO`, `Åland`, `Âge`, `ROMĂN` match neither). `Ä` (byte C4) decodes identically in both code pages and cannot discriminate, so it is counted as a third, ambiguous class.
- **Confidence**: `normative` (mechanism), `measured` (the frequency claim that 1250 dominates in Hungarian text was an inference from the Windows locale default and is now **confirmed** on a full scan; `OQ-10-001` resolved).

### REP-ENC-004 UTF-8 decoded as a DOS or Mac code page
- **Statement**: UTF-8 shown in a DOS console (CP852, CP437) produces box-drawing lead characters `├` (C3) and `┼` (C5); UTF-8 opened as MacRoman or MacCE produces `√` (C3) and `≈`/`Ň` (C5) leads; the repair re-encodes with that code page and decodes as UTF-8.
- **Conditions**:
  - CP852/CP437: `├` followed by the continuation image (`í ę ş │ Â ║ ╝ ü ë Ź ô ľ Ü ť` for CP852, Table T-ENC-2), `┼` followed by `Ĺ ▒ É ░` (CP852) or `æ ▒ É ░` (CP437); punctuation leads `ÔÇ` (CP852) / `ΓÇ` (CP437).
  - MacRoman: `√` + `° © ≠ ≥ ∂ ∫ º Å â ç ì ñ ö ú`, `≈` + `ë ± ê ∞`; MacCE: `√` + `° © ≠ ≥ ∂ ļ ľ Ā Č ć ď Ė ö ú`, `Ň` + `Ď Ī ź į`; punctuation lead `‚Ä`.
  - Box-drawing characters or `√ ≈` adjacent to Latin letters inside a word token.
- **Output**: Same procedure as REP-ENC-002 with the four tables; accept only by plausibility gain.
- **Examples**: `K├ęrem` (CP852) -> `Kérem`; `t┼▒z` -> `tűz`; `K√©rem` (MacRoman) -> `Kérem`; `≈ësz` (MacRoman) -> `ősz`.
- **Exceptions**: real box drawing in pseudo-graphic tables (runs of `─│┼├` without letters on both sides); mathematical `√`, `≈`, `≠` between digits or spaces. Guard: the lead must be followed by a continuation image and preceded or followed by a letter.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: with REP-ENC-002/003 in one candidate search (REP-ENC-015).
- **Sources**: [UNI-MAP CP852.TXT, CP437.TXT, CENTEURO.TXT, ROMAN.TXT].
- **Validation**: `goldens` (Table T-ENC-2); negative set: ASCII-art tables and formulas.
- **Confidence**: `normative`.

### REP-ENC-005 Double and repeated UTF-8 encoding
- **Statement**: Text misdecoded once and saved again as UTF-8 is misdecoded a second time, doubling the damage (`é` -> `Ã©` -> `ÃƒÂ©`); the repair applies the single-level inverse repeatedly until no signature remains or the result stops improving.
- **Conditions**:
  - windows-1252 twice: `Ãƒ` followed by `Â` + image, or `Ã…` followed by `â€˜` / `Â±` / `Â°` (Table T-ENC-3).
  - windows-1250 twice: `Ä‚` followed by an image, `Äą` followed by `â€`, `Â±`, `Â°`.
  - Mixed levels in one document (one paragraph once, another twice) are common in scraped comment threads and merged exports.
- **Output**: Iterate the REP-ENC-002/003 inverse per run; stop when (a) no signature, (b) strict decode fails, or (c) plausibility score does not increase; maximum 3 iterations. Record the level count in provenance.
- **Examples**: `ÃƒÂ©` -> `Ã©` -> `é`; `Ã…â€˜` -> `Å‘` -> `ő`; `Ä‚Ĺ‚` -> `Ăł` -> `ó`.
- **Exceptions**: a level that decodes to valid UTF-8 but lowers the plausibility score (rare accidental valid sequences) stops the iteration.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: REP-ENC-002, REP-ENC-003; REP-NOISE-002 generates levels 1..3.
- **Sources**: [PY-CODECS] (Table T-ENC-3 computed); [PT-CLEAN encoding.py] (single level only; iteration is added here).
- **Validation**: `goldens` (Table T-ENC-3); idempotence test (repair of repaired text changes nothing).
- **Confidence**: `normative`.

### REP-ENC-006 ISO-8859-2 or windows-1250 text decoded as ISO-8859-1 or windows-1252 (tilde and circumflex vowels)
- **Statement**: Single-byte Hungarian text decoded with a Western table keeps 14 letters and turns exactly `ő ű Ő Ű` into `õ û Õ Û`; the repair maps them back when the word or document is Hungarian and the repaired word is better attested than the damaged one.
- **Conditions**:
  - Characters `õ û Õ Û` present; and
  - Document-level evidence: share `wrong/(wrong + correct)` where wrong = count of `õûÕÛ` and correct = count of `őűŐŰ` is at least 0.20, and the document passes the Hungarian function-word test (REP-QUAL-008); or
  - Word-level evidence (preferred, needed for pockets and listings): the mapped word is accepted by the oracle and the unmapped word is not; or the mapped word is attested in the same document with correct double acute letters; or the mapped word carries another Hungarian-only accent combination.
  - The same bytes explain `ô` only via CWI-2 or font fallback (REP-ENC-009, REP-FONT-001), never via Latin-2/Latin-1.
- **Output**: `õ` -> `ő`, `û` -> `ű`, `Õ` -> `Ő`, `Û` -> `Ű` inside accepted words only. Document share is reported with the repair.
- **Examples**: `elõdöntõ` -> `elődöntő`; `fûtés` -> `fűtés`; `Gyõr` -> `Győr`; `KÕSZEG` -> `KŐSZEG`; `õ is` -> `ő is` (pronoun; accepted only with context, see Exceptions).
- **Exceptions**:
  - Estonian (`Võru`, `Rõuge`, `Emajõgi`, `õpetaja`), Portuguese (`São`, `põe`, `lições`), Vietnamese (`rõ`), Guarani, Livonian: `õ` is a letter. French (`flûte`, `sûr`, `dû`, `août` has `û`), Welsh `û`: `û` is a letter. Keep words the oracle rejects in both spellings (foreign names) unchanged.
  - Measured document-share band: an empty band between 0.36 and 0.75 over 12,000 web documents, but 0.16 was French loanwords while 0.17 and 0.19 were damage; no document cut is exact, which is why word-level evidence overrides.
  - One-letter word `õ`: lexically ambiguous with Estonian; map only if the sentence is Hungarian by REP-QUAL-008.
  - Inverse damage (real `û` or `õ` shown as `ű`/`ő`) is REP-ENC-007, not this rule.
- **Predictability**: `rule` (mechanism); `tendency` (document share threshold 0.20)
- **Lexical input**: oracle acceptance; per-document vocabulary of correct double acute words
- **Interactions**: repair side of `WS-ENC-003`, `WS-ENC-005` and `WS-NORM-004`; after REP-ENC-002..005; before REP-DIAC (a text with `õ` is not accent-stripped); REP-FONT-001 handles the same glyphs arising from fonts and is merged with this rule at runtime (one per-word decision).
- **Sources**: [UNI-MAP 8859-2.TXT, CP1252.TXT]; [PT-CLEAN cleaning.md "Latin-2 damage survives", encoding.py `_latin2`, `_latin2_words`, `_latin2_analysed`] (0.822% of long vowels wrong after document-level repair in 99,008 documents; 458 documents averaging 89 wrong characters); [PC-KB repair.rs `FontGaps`].
- **Validation**: `oracle:analyser` per word; `goldens` including `Võru`, `São`, `flûte` unchanged; `attestation` of mapped forms.
- **Confidence**: `consensus`.

### REP-ENC-007 Latin-1 letters of other languages decoded as ISO-8859-2 (inverse damage)
- **Statement**: Western single-byte text decoded as ISO-8859-2 or windows-1250 inside a Hungarian document turns foreign `õ û` into `ő ű` and other Western letters into Central European look-alikes (`ã` -> `ă`, `è` -> `č`, `ñ` -> `ń`, `ø` -> `ř`); the repair is the reverse table, applied only to tokens that are not Hungarian.
- **Conditions**:
  - A token containing `ő ű Ő Ű` that the oracle rejects, and whose inverse-mapped form is a known foreign word or name (lexicon of foreign names) or co-occurs with other inverse signatures in the same token or neighbouring tokens: `ŕ ă ĺ ć č ę ě ď đ ń ň ř ů ţ ˙` (Table T-ENC-5).
  - Characters `č ř ě ů` are also genuine Czech/Slovak letters: require that the token contains at least one letter of the Table T-ENC-5 image column that does not belong to the token's plausible language (e.g. `ń` plus `ő` in one token).
  - With no foreign-name lexicon, the evidence is the mixed-script conflict alone: a run of neighbouring rejected tokens is restored when it holds both `ő`/`ű` (Hungarian only) and an image letter Hungarian never writes. A lone `flűte`, `Espańa` or `Ĺngström` is left as written; the reverse table is derived from the code pages (ISO-8859-2 and windows-1250 against windows-1252), not listed.
- **Output**: Reverse map per token via Table T-ENC-5 (`ő` -> `õ`, `ű` -> `û`, `ŕ` -> `à`, `ă` -> `ã`, `ĺ` -> `å`, `ć` -> `æ`, `č` -> `è`, `ę` -> `ê`, `ě` -> `ì`, `ď` -> `ï`, `đ` -> `ð`, `ń` -> `ñ`, `ň` -> `ò`, `ř` -> `ø`, `ů` -> `ù`, `ţ` -> `þ`, `˙` -> `ÿ`, and capitals).
- **Examples**: `Săo Paulo` -> `São Paulo`; `flűte` -> `flûte`; `Espańa` -> `España`; `crčme brűlée` -> `crème brûlée`; `Ĺngström` -> `Ångström`.
- **Exceptions**: Czech, Slovak, Polish, Romanian words with `č ř ě ů ń ă ţ` are correct as written; require a mixed-script conflict before repairing. Hungarian words are never touched.
- **Predictability**: `tendency`
- **Lexical input**: foreign name lexicon; oracle rejection of the Hungarian reading
- **Interactions**: runs together with REP-ENC-006 (both are one per-token decision over the same byte alphabet); prior art observed `flűte` attested source-wide in an Italian passage [PT-CLEAN encoding.py `_latin2_words` note].
- **Sources**: [UNI-MAP 8859-2.TXT, CP1252.TXT]; [PT-CLEAN].
- **Validation**: `goldens`; negative set Czech/Polish text unchanged.
- **Confidence**: `consensus`.

### REP-ENC-008 DOS code page confusions (CP852, CP850, CP437, windows-1250)
- **Statement**: Files written by DOS programs in CP852 and read as windows-1250 (or the reverse, Windows text printed to a CP852 console) produce systematic substitutions of all 18 letters; CP852 read as CP437 damages `ő ű Á Í Ó Ő Ú Ű`, read as CP850 only `ő ű Ő Ű` (CP850 has `Á Í Ó Ú` at the CP852 positions).
- **Conditions**:
  - CP852 read as windows-1250: letters replaced by NBSP (`á`), `‚` (`é`), `ˇ` (`í`), `˘` (`ó`), `”` (`ö`), `‹` (`ő`), `Ł` (`ú`), U+0081 (`ü`), `ű` stays `ű` (both 0xFB), `µ` (`Á`), U+0090 (`É`), `Ö` (`Í`), `ŕ` (`Ó`), `™` (`Ö`), `Š` (`Ő`), `é` (`Ú`), `š` (`Ü`), `ë` (`Ű`) (Table T-ENC-4).
  - windows-1250 read as CP852: `ß Ú Ý ˇ ÷ § ˙ Ř ű` for `á é í ó ö ő ú ü ű` and box-drawing for several capitals.
  - CP852 read as CP437/CP850: `ő` -> `ï`, `ű` -> `√` (437) or `¹` (850), `Ő` -> `è`, `Ű` -> `δ`/`Ù`, `Á` -> `╡`/`Á`, `Í` -> `╓`/`Í`, `Ó` -> `α`/`Ó`, `Ú` -> `Θ`/`Ú`; lowercase `á é í ó ö ú ü` survive.
  - Detection: characters from the image columns inside word tokens at a rate implausible for Hungarian (typographic quotes `‚ ” ‹` and spacing accents `ˇ ˘` adjacent to letters on both sides).
- **Output**: Whole-run re-decode (encode with the wrong table, decode with the right one), candidate selection by plausibility score over all pairs in Table T-ENC-4.
- **Examples**: `k‚pvisel‹` (CP852 as 1250) -> `képviselő`; `t√z` (CP852 as CP437) -> `tűz`; `ßll` (1250 as CP852) -> `áll`.
- **Exceptions**: The NBSP image of `á` is invisible and may already have been normalised to a space by `REP-SPACE-006`; therefore ENC runs before SPACE. When the space has already replaced it (`h z` for `ház`), the damage is unrecoverable by table; route to `REP-SPACE-009` with the oracle.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: repair side of `WS-ENC-007`; REP-ENC-015 candidate search; must precede REP-SPACE-006.
- **Sources**: [UNI-MAP CP852.TXT, CP437.TXT, CP850.TXT, CP1250.TXT].
- **Validation**: `goldens` (Table T-ENC-4).
- **Confidence**: `normative`.

### REP-ENC-009 CWI-2 text read as CP437
- **Statement**: CWI-2, the Hungarian DOS code page of the 1980s and early 1990s, differs from CP437 only in the Hungarian letters, so CWI-2 text read as CP437 stays readable but shows `ô û º ÿ ì Å ò ù` for `ő ű Ő Ű Í Á Ó Ú`; this is one historical source of circumflex vowels standing for double acute.
- **Conditions**: characters `ô û` (and in capitals `º ÿ ì Å ò ù`) in Hungarian words; lowercase `á é í ó ö ú ü` and `É Ö Ü` intact.
- **Output**: `ô` -> `ő`, `û` -> `ű`, `º` -> `Ő`, `ÿ` -> `Ű`, `ì` -> `Í`, `Å` -> `Á`, `ò` -> `Ó`, `ù` -> `Ú`, per word, under the word-level evidence of REP-ENC-006.
- **Examples**: `erôs` -> `erős`; `ÅLLAT` -> `ÁLLAT`; `ÒRA` -> `ÓRA`; `Ùj` -> `Új`.
- **Exceptions**: French `ô` (`côte`, `hôtel`), Slovak `ô` (`kôň`, `stôl`), Portuguese `ô`; `Å` in Scandinavian names; `º` as ordinal indicator after digits (`1º`). Word-level oracle evidence is mandatory; capitals mapping only when the whole token is uppercase Hungarian.
- **Predictability**: `rule` (mechanism), `lexical` (per-word decision)
- **Lexical input**: oracle acceptance
- **Interactions**: merged with REP-ENC-006 and REP-FONT-001 into one per-word double-acute substitute decision.
- **Sources**: [ENWIKI-CWI2] (row 8x-Ax of the code chart); [UNI-MAP CP437.TXT]; [SZTE-KOD] (a Hungarian dictionary service accepts `ô` as input for `ő`, evidence that the substitution was in circulation); [PT-CLEAN]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens`; negative set French/Slovak; prevalence measured on a full scan.
- **Confidence**: `consensus` (code chart normative); **prevalence measured** (`OQ-10-004` resolved): of 129,870 Hungarian documents carrying `ô`, 0.025% (32) carry the clean CP437 box-drawing tell, 0.010% (13) carry both tells, and at the most generous reading 1.5% (2,011) carry either - and that 1.5% is an upper bound inflated by Scandinavian `Å` and Italian/French `ò ù ì` in the capitals tell, so only the box-drawing tell and the conjunction are trustworthy. **CWI-2 accounts for at most 1.5% and realistically well under 0.1% of `ô`-for-`ő` in this corpus**; the remaining 98.5%+ is typed substitution, Latin-1 or Latin-2 font fallback, or PDF font issues in otherwise clean modern text. Supporting: 104,554 of the 129,870 (80.5%) already had the Latin-2 repair fire on them, i.e. they carried `õ`/`û` damage the Latin-2 map recognised with `ô` left behind because that map does not cover it. **Keep the mapping table and weight this hypothesis near zero as a prior**; `REP-FONT-001`'s per-word decision is what does the work. Honest limit: the 13 both-tell documents are all scanned academic PDFs, which is where DOS-era material would in fact survive, but the date argument rests on those 13 and not on the `date` column, which is a crawl timestamp for 93% of the population.

### REP-ENC-010 MacCE and MacRoman single-byte confusions
- **Statement**: Classic Mac OS text in MacCE read as windows-1250 (or the reverse), and ISO-8859-2 read as MacRoman, produce distinctive substitutions; notably Latin-2 `ő` read as MacRoman is `ı` (dotless i), which must not be confused with the shifted-font fingerprint of REP-FONT-003.
- **Conditions**:
  - MacCE read as windows-1250: `á é í ó ö ő ú ü ű` -> `‡ Ž ’ — š Î ś ź ő` (Table T-ENC-4); note `ű` becomes `ő`.
  - windows-1250 read as MacCE: `Š ť Ū ů Ų ű ķ Ł Ż`.
  - ISO-8859-2 read as MacRoman: `á` -> `·`, `é` -> `È`, `í` -> `Ì`, `ó` -> `Û`, `ö` -> `ˆ`, `ő` -> `ı`, `ú` -> `˙`, `ü` -> `¸`, `ű` -> `˚`.
  - Discriminator from REP-FONT-003: in MacRoman damage `á é ö` are also damaged (to `· È ˆ`); in the shifted font they are intact.
- **Output**: Whole-run re-decode by the pair with the best plausibility score.
- **Examples**: `‡ll` (MacCE as 1250) -> `áll`; `ıszi` with `ˆ` elsewhere (8859-2 as MacRoman) -> `őszi`.
- **Exceptions**: typographic `‡ † — ’` in genuine windows-1250 text: require letters on both sides and a plausibility gain.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: repair side of `WS-ENC-008`; REP-ENC-015; REP-FONT-003 discriminator.
- **Sources**: [UNI-MAP CENTEURO.TXT, ROMAN.TXT, CP1250.TXT].
- **Validation**: `goldens` (Table T-ENC-4).
- **Confidence**: `normative` (tables); `tendency` (prevalence low, unmeasured).

### REP-ENC-011 Lossy transcoding into a code page without double acute letters
- **Statement**: Conversion from Unicode or Latin-2 into windows-1252 or ASCII with "best fit" or transliteration silently turns `ő ű Ő Ű` into `o u O U` while the other 14 letters survive; conversion with a replacement policy turns them into `?`; both leave a text in which only the double acute letters are missing.
- **Conditions**:
  - Best fit: document contains `á é í ó ö ú ü` at normal Hungarian rates (REP-QUAL-001) but `ő ű` count is zero over at least 200 letters of the o/u families; and the oracle accepts many `o`/`u` -> `ő`/`ű` substitutions (e.g. `elott`, `kerulo` is not this class because `ü` is also missing).
  - Replacement: `?` (or U+FFFD) inside word tokens between letters, at positions where `ő ű Ő Ű` would make the token acceptable (`Gy?r`, `f?z?`, `?sz`).
- **Output**:
  - Best fit: candidate set per `o` -> {`o`, `ő`}, per `u` -> {`u`, `ű`} only (not `ó ö ú ü`, which survived), disambiguated with REP-DIAC-004.
  - Replacement: candidate set {`ő`, `ű`, `Ő`, `Ű`} per `?`; accept the unique oracle-accepted candidate; if several are accepted (`?r` -> `őr`/`űr`), use frequency and context; else `flag`.
- **Examples**: `elotte` (best fit) -> `előtte`; `fuzfa` -> `fűzfa` (intact `á é` elsewhere); `Gy?r` -> `Győr`; `k?`-type ambiguity `?r` -> `őr` (bucket 11) vs `űr`: context decides.
- **Exceptions**: a genuine question mark inside a token (`Mi?Hogy?` without space, URLs with query strings `?id=`) is excluded by requiring letters on both sides and a word token without `=`, `/`, `&`.
- **Predictability**: `rule` (candidate restriction), `tendency` (disambiguation)
- **Lexical input**: oracle, frequency
- **Interactions**: repair side of `WS-ENC-004`; shares the candidate engine of REP-DIAC-003 with a restricted alphabet (REP-DIAC-002 class `B`).
- **Sources**: [MS-BF bestfit1252.txt] (U+0150 -> 0x4F, U+0151 -> 0x6F, U+0170 -> 0x55, U+0171 -> 0x75); [UNI-MAP CP1252.TXT].
- **Validation**: `goldens`; `oracle:analyser`.
- **Confidence**: `normative` (mapping), `consensus` (detection).

### REP-ENC-012 Replacement character U+FFFD
- **Statement**: U+FFFD marks bytes a decoder could not read; it is repaired only in the two contexts where the lost content is recoverable (a line-break hyphen inside a word, a single lost byte of a known mojibake sequence) and is otherwise counted, not guessed.
- **Conditions**:
  - Context A: U+FFFD between two letters inside a token and deleting it yields an oracle-accepted word; measured 93.3% of surviving U+FFFD in one PDF source were lost hyphens at doubled digraph line breaks (`vis` U+FFFD `sza`).
  - Context B: U+FFFD directly after a mojibake lead (`Ã`, `Å`, `Ă`, `Ĺ`) (REP-ENC-002 Exceptions).
  - Context C: U+FFFD standing for a whole lost letter in a Hungarian word where a unique accented letter makes the word acceptable (`v` U+FFFD `ros` -> `város`): allowed only if exactly one candidate from the 18 letters is accepted.
- **Output**: A: delete; B: the byte-inverse candidate; C: the unique letter; otherwise keep and count (REP-QUAL-005).
- **Examples**: `vis`U+FFFD`sza` -> `vissza`; `kavic`U+FFFD`csal` -> `kaviccsal`; `v`U+FFFD`ros` -> `város`.
- **Exceptions**: U+FFFD next to digits or punctuation: keep. Two or more consecutive U+FFFD: keep (multi-byte loss).
- **Predictability**: `rule` (A, B), `tendency` (C)
- **Lexical input**: oracle
- **Interactions**: runs before U+FFFD counting in REP-QUAL-005 so documents whose only damage is lost hyphens are not penalised; interacts with REP-SPACE-004 (doubled digraph line breaks).
- **Sources**: [PT-CLEAN cleaning.md "A lost hyphen inside a doubled digraph", encoding.py `_hyphen_replacement`] (93.3%; 4 of 4 dropped documents rescued in an 11 Mchar sample).
- **Validation**: `goldens`; `attestation`.
- **Confidence**: `consensus`.

### REP-ENC-013 Escaped representations left in text
- **Statement**: HTML character references, JSON and programming-language escapes, percent and quoted-printable encodings, RTF hex escapes and TeX accent commands that survive into plain text are decoded to the letters of Table T-ENC-6, exactly once.
- **Conditions**:
  - HTML named (`&aacute;` .. `&Udblac;`, including HTML5 `&odblac; &Odblac; &udblac; &Udblac;`), decimal `&#337;`, hex `&#x151;`; well-formed with terminating `;` (tolerate missing `;` only for the Latin-1 legacy names that HTML parsers accept without it, e.g. `&eacute`).
  - Double-escaped `&amp;aacute;`: decode once only when the source is scraped HTML text; a second decode is not idempotent (`&amp;nbsp;` -> `&nbsp;` -> NBSP).
  - JSON / JS `\u00e1`, `\u0151` (Rust `\u{151}`; Python `\u0151`): only in extracted text of JSON-bearing sources where the surrounding record is prose; prose inside a still-embedded JSON record is removed with the record rather than decoded.
  - Percent `%C3%A1`, `%C5%91` in prose (not in URLs); quoted-printable `=C3=A1`, `=F5` with soft line breaks `=` at line end in e-mail bodies; RTF `\'e1` inside RTF runs; TeX `\'a`, `\"o`, `\H{o}`, `\H o`, `\'{\i}` in plain text exported from LaTeX.
- **Output**: The letter; for quoted-printable and RTF the byte is decoded with the declared charset (default UTF-8 for `=C5=91` pairs, ISO-8859-2 for single `=F5`, windows-1250 for RTF with `\ansicpg1250`).
- **Examples**: `&odblac;r` -> `őr`; `k&ouml;nyv` -> `könyv`; `\u0151sz` -> `ősz`; `Gy%C5%91r` (prose) -> `Győr`; `=C3=A9s` -> `és`; `\H{o}r` -> `őr`.
- **Exceptions**:
  - Source code, HTML tutorials, URL tokens (`https://...%C3%A9...`), TeX documents meant to stay TeX: do not decode; the source declares whether the repair applies.
  - A literal entity discussed as text (`írja: &aacute;`) in an HTML tutorial is content.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: repair side of `WS-ENC-010` (which lists the legacy `&otilde; &ocirc; &ucirc;` substitutes, handled after decoding by REP-ENC-006 and REP-FONT-001); first in the pass order (REP-QUAL-011): decoded text may reveal mojibake (`&Atilde;&copy;` -> `Ã©` -> `é`); runs exactly once per document.
- **Sources**: [HTML5-ENT]; [PT-CLEAN cleaning.md "Cleaning runs exactly once"]; [PT-CLEAN "Serialised page data"]; [SZTE-KOD] (QP, HTML, TeX notations in use for Hungarian).
- **Validation**: `goldens` (Table T-ENC-6); idempotence test on sources flagged run-once.
- **Confidence**: `normative`.

### REP-ENC-014 Seven-bit Hungarian conventions
- **Statement**: Pre-Unicode e-mail, Usenet, telegram and packet radio practice wrote Hungarian letters with ASCII sequences (flying accent `a' o: o"`, digit notation `a1 o2 o3`, telegram doubling `aa oe oeoe`) or with the ISO 646-HU national variant (`@ [ \ ]`, backquote, `{ | } ~` for `Á É Ö Ü`, `á`, `é ö ü ˝`); these are decoded only when a document or block uses the convention consistently.
- **Conditions**:
  - Flying accent: tokens matching `[aeiou]['":~]` or `[AEIOU]['":~]` inside words at a rate of **at least one per 10 words**, with at least two distinct marks, and the decoded words oracle-accepted at >= 0.8. The previous rate of one per 20 words sits **below** the clean maximum measured over 4,000 clean segments (0.0638): one clean Hungarian segment in a thousand carries enough apostrophes and inch marks to clear it. Real flying-accent text runs at 0.31 and above, so at one per 10 words the false positives go to zero with no recall cost.
  - Digit notation: `[aeiouAEIOU][123]` inside alphabetic tokens (`a1rvi1ztu3ro3`) **at a rate of at least one per 20 words**; decoded words accepted >= 0.8. Measured: clean maximum 0.0263 against a damaged p05 of 0.2651, so 0.05 gives precision and recall 1.00000 and 0.10 is equally exact.
  - Telegram: `aa ee ii oo uu oe ue` in positions where the decoded word is accepted and the undecoded is not; consistent across the block.
  - ISO 646-HU: `@ [ \ ]`, backquote, `{ | }` inside alphabetic tokens (`|` = `ö`, `{` = `é`: `k|sz|n|m` -> `köszönöm`) **at a rate of at least one per 10 words**, with decoded acceptance >= 0.8; `~` after `o`/`u` is the double acute. Measured: clean maximum 0.0513 against a damaged p05 of 0.1981; at one per 20 words precision is 0.99975 (1 false positive of 4,000), at one per 10 words 1.00000 at recall 0.99975.
- **Output**: Table T-ENC-6 columns "flying accent", "123 notation", "telegram style"; ISO 646-HU per Table T-ENC-1. Decode per block; never per isolated token.
- **Examples**: `a'rvi'ztu"ro" tu:ko:rfu'ro'ge'p` -> `árvíztűrő tükörfúrógép`; `a1rvi1ztu3ro3` -> `árvíztűrő`; `aarviiztueueroeoe` -> `árvíztűrő`; `k|sz|n|m` (646-HU) -> `köszönöm`.
- **Exceptions**: Quoted speech `'` and English contractions (`don't`), inch/second marks `5'`, `3"`, emoticons `:)`, programming text; telegram-style decoding is unsafe for real doubled vowels (`kooperáció`, `vákuum`, `Aachen`) and for `oe`/`ue` in Hungarian and foreign words (`poén`, `Mueller`, `Goethe`): apply only with block-level consistency and oracle preference.
- **Predictability**: `tendency`
- **Lexical input**: oracle acceptance
- **Interactions**: repair side of `WS-NORM-006` and `WS-ENC-009`; before REP-DIAC-001 (a flying-accent text has zero accent density and would otherwise be treated as stripped).
- **Sources**: [HUWIKI-REPEK] (flying accent convention, telegram example); [SZTE-KOD] (flying accent, 123 notation table); [ENWIKI-646] (ISO-IR-086 row); [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens`; negative set English and code. Thresholds **measured by injection on 4,000 gold segments of >= 30 tokens per convention**, with the same 4,000 segments clean as negatives (they carry the real apostrophe, quotation-mark and bracket traffic of eight registers, which is exactly the confusion the Exceptions field is about): precision 1.00000 at the proposed rates, recall 0.985 with the 0.8 acceptance clause and 1.000 without it. The acceptance clause behaves identically on both classes (positives p05 0.8409 median 0.9375; negatives median 0.9375), because the decode is a no-op on clean text; it costs 1.5% recall and moves precision by less than 0.001 at every rate setting. It is kept as cheap insurance against a corpus this evaluation does not contain (foreign text with heavy apostrophe use), but **the rate is the discriminating clause, not the acceptance test**.
- **Confidence**: `consensus` (conventions documented), `tendency` (detection thresholds, now measured by injection rather than proposed).

### REP-ENC-015 Mixed documents and candidate search
- **Statement**: A document may contain clean text and several encoding damages at once (quoted older messages, merged exports, per-paragraph sources); encoding repair therefore decides per maximal damaged run, choosing among all code page hypotheses by plausibility gain, and never applies a whole-document map that would damage the clean part.
- **Conditions**:
  - Segment the text into runs: a run is a maximal span in which every token either is clean ASCII or carries the same damage signature family; clean tokens with correct Hungarian letters (`ő`, `á`) break a run for hypotheses that would alter them.
  - Hypotheses: {UTF-8 as windows-1252, as ISO-8859-1, as windows-1250, as ISO-8859-2, as CP852, as CP437, as MacCE, as MacRoman} x levels 1..3; {ISO-8859-2 as windows-1252}; {CP852 as windows-1250, as CP437, as CP850}; {windows-1250 as CP852, as MacCE}; {MacCE as windows-1250}; {CWI-2 as CP437}; {ISO-8859-2 as MacRoman}; {Latin-1 as ISO-8859-2 inverse}.
  - A hypothesis is admissible for a run only if its byte re-encoding is total (every character has a byte in the wrong table), the decode is strict, **and the decoded run contains no C1 control (U+0080..U+009F)**. A strict ISO-8859-2 decode accepts C1 bytes as controls, so strictness alone does not exclude them, and this is the whole of the residual false-positive mass for `{ISO-8859-2 as windows-1252}` - 15 of the 16 wrong decodes at margin 0.25 over 3,080 pairs.
- **Output**: For each run apply the admissible hypothesis with the highest plausibility score gain (REP-ENC-016), if the gain exceeds the threshold; otherwise keep. Provenance records hypothesis and level.
- **Examples**: a forum page with a clean post `Köszönöm, Győr` and a quoted `KÃ¶szÃ¶nÃ¶m, GyÅ‘r`: only the quote is repaired. A PDF with one shifted font region is `FONT`, not `ENC`: the discriminator is that byte-level hypotheses are not admissible (the characters `ı` and `ő` are both present and legitimate in the tables).
- **Exceptions**: runs shorter than one token with a single signature character are repaired only when the signature is unambiguous (two-character UTF-8 lead + image) and the result is oracle-accepted.
- **Predictability**: `rule`
- **Lexical input**: oracle, letter statistics
- **Interactions**: run-level counterpart of the document-level detection in `WS-ENC-011`; orchestrates REP-ENC-002..011; the per-document variant of REP-ENC-006 is a special case with one run.
- **Sources**: [PT-CLEAN "The font shift has three outcomes"] (analogous font-shift case: a whole-document map shipped 11 wrongly decoded mixed documents in a predecessor); [PT-CLEAN "Repairs are per source"]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens` with synthetic mixed documents from REP-NOISE-002; idempotence.
- **Confidence**: `consensus`.

### REP-ENC-016 Hungarian plausibility score for decoding candidates
- **Statement**: Candidate decodings are compared with a score that rewards Hungarian letters and oracle-accepted words and penalises characters that do not occur in Hungarian prose; the score is the only arbiter between encoding hypotheses.
- **Conditions**: computed on the run and its candidate; runs of fewer than 3 letters use only the character term.
- **Output**: `score = 2.0 * accepted_word_share + 1.0 * hungarian_letter_share - 3.0 * foreign_symbol_rate - 5.0 * box_drawing_in_word_rate`, where hungarian_letter_share = share of non-ASCII letters that are among the 18 Hungarian letters; foreign_symbol_rate = rate per letter of `Ã Å Ă Ĺ â€ ├ ┼ √ ≈ ‡ ˇ ˘ ‹ › õ û ı` inside word tokens. **The C1 term is no longer part of the score: a candidate containing any character in U+0080..U+009F is inadmissible** (`REP-ENC-015`). Accept a candidate when `score(candidate) - score(input) >= 0.10` and accepted_word_share does not decrease.
- **Examples**: `KĂ©rem` (score -0.6: word rejected, no Hungarian non-ASCII letter, `Ă` is 1 foreign symbol per 5 letters: 2.0 * 0 + 1.0 * 0 - 3.0 * 0.2) vs `Kérem` (3.0 = 2.0 * 1 + 1.0 * 1): gain 3.6, accept.
- **Exceptions**: the C1 term is corrected to a veto (see Validation). **The remaining four weights are not identifiable on single-damage pairs**: admissibility and the ordering constraint separate the classes on their own before the weights are consulted (median gain on damaged text 1.43 against a maximum gain over 19,757 clean segments of 0.001), so the numeric weights only have to preserve the sign of a gap that is never close. Refitting them needs mixed-encoding documents of the `REP-ENC-015` kind, where two admissible hypotheses both raise the accepted-word share; the pair file contains none, because its generator forbids composing encoding faults. The ordering constraint (no decrease of accepted words) is fixed.
- **Predictability**: `tendency` (weights proposed, not yet measured)
- **Lexical input**: oracle
- **Interactions**: used by REP-ENC-002..015; REP-QUAL-001 supplies letter baselines.
- **Sources**: design derived from [PT-CLEAN encoding.py `_utf8_latin1`] (accept only if the signature count decreases) and [PC-KB quality.rs] (foreign letter counting); [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens` on generated pairs; report precision/recall per hypothesis. **Measured on 3,080 mojibake pairs against 19,757 clean segments**, a true positive requiring exact recovery of the whole paragraph. With the C1 veto: precision 0.99887 / recall 0.85974 at margin 0.10, and 0.99864 / 0.95260 at margin 0.00. Without it: 0.96572 / 0.85974 at 0.10, 0.99182 / 0.62987 at 0.25 (the previous setting, below the 0.995 target), 1.00000 / 0.52500 at 0.50. **The veto meets the precision target with 64% more recall than raising the margin would**, and it does so by correcting a weight the rule already had rather than trading recall away; margin 0.10 is chosen over 0.00 (best F1, 0.97507) only because a strictly positive margin is a safer default for hypothesis families this evaluation does not contain. Recall is against exact whole-paragraph recovery and is capped at 0.9526 by composed damage. Diagnosis of the residual: at margin 0.25 all but one false positive came from a single hypothesis, `latin2_as_cp1252`, which is **not** choosing the wrong code page - it gets the Hungarian letters right (`zûrbõl` -> `zűrből`, `hétfõ` -> `hétfő`) and fails exact match only on the typographic quotation marks around them, which cp1252 encodes into the C1 byte range (`„` = 0x84, `”` = 0x94) where ISO-8859-2 has control characters. A further correction to how the rule reads: on clean text almost no hypothesis is even admissible and the best gain ever observed is 0.001, so **the accept margin is not protecting clean text; it is arbitrating between competing wrong hypotheses on damaged text**.
- **Confidence**: `measured` (C1 veto and accept margin); `tendency` (the remaining four weights).

## FONT: font and glyph substitutions

### REP-FONT-001 Double acute substitutes: tilde, circumflex and other marks on o and u
- **Statement**: `õ ô ũ û` and their capitals, whatever their origin (Latin-1 fonts, CWI-2 viewed as CP437, typed substitutes, OCR, legacy `&otilde;` entities), stand for `ő ű` inside Hungarian words; they are repaired by one per-word decision that maps the glyph to the double acute letter only when the repaired word is Hungarian and the unrepaired word is not.
- **Conditions**:
  - token contains one of `õ ô ǒ ŏ ò ō ȍ` (for `ő`), `û ũ ǔ ŭ ù ū ȕ` (for `ű`), or capitals `Õ Ô Ǒ Ŏ Ò Ō Ȍ Û Ũ Ǔ Ŭ Ù Ū Ȕ` (Table T-FONT-1);
  - decision order per token: (1) if the token as written is accepted by the oracle or the foreign lexicon, keep; (2) map every substitute glyph to the double acute letter of the same base and case; if the result is accepted, repair; (3) if a token mixes substitutes with correct `ő ű`, map only the substitutes; (4) otherwise keep and count as residue (REP-QUAL-004).
  - mapping `ô` -> `ő` and `õ` -> `ő` is also tried in tokens carrying `ö`-type ambiguity: `õ` is never mapped to `ö` or `ó`.
  - grave and macron variants (`ò ō ù ū`) are accepted as substitutes only when the document already contains tilde/circumflex substitutes or when the token has another Hungarian-only letter (`á é í ö ü`), because `ò ù` are Italian/French letters and `ō ū` occur in romanised Japanese.
- **Output**: `õ ô ǒ ŏ ò ō ȍ` -> `ő`; `û ũ ǔ ŭ ù ū ȕ` -> `ű`; capitals likewise; provenance records the glyph.
- **Examples**: `elsô` -> `első`; `elôtt` -> `előtt`; `mûködik` -> `működik`; `országgyûlés` -> `országgyűlés`; `KÖVETKEZÔ` -> `KÖVETKEZŐ`; `fũtés` -> `fűtés`; `hôtel` kept (French, oracle rejects `hőtel`); `Tõnu` kept; `Guimarães` kept.
- **Exceptions**:
  - Genuine letters: Estonian and Livonian `õ`; Portuguese `õ ô`; Vietnamese `õ ô ũ ù ò`; French `ô û ù`; Slovak `ô`; Italian `ò ù`; Guarani `õ ũ`; Welsh `û`; Kashubian `ò ô ù`; romanised Japanese `ō ū`; Latvian and Lithuanian `ū`.
  - A word accepted in both readings is lexically ambiguous only if both are Hungarian; since the substitutes are not Hungarian letters, the written form is never accepted by the Hungarian oracle and the rule degenerates to "repair if the mapped word is accepted".
  - Proper names unknown to both lexicons stay unrepaired (safe residue), unless the same document attests the mapped name with correct letters.
- **Predictability**: `rule` (mapping); `lexical` (per-word acceptance)
- **Lexical input**: oracle; foreign-word and foreign-name lexicon; per-document vocabulary
- **Interactions**: merges REP-ENC-006 and REP-ENC-009 into one decision (run once); detection facts in `WS-CHAR-002`, `WS-NORM-004`, `WS-ENC-005`; before REP-DIAC and REP-SPELL (a substitute is not a spelling error).
- **Sources**: [ATT] via `WS-CHAR-002` (962 types with `õ`, 279 with `û`, 395 with `ô`); [PT-CLEAN encoding.py `_latin2_analysed`] (map exactly when mapped form parses and original does not; `Flûte` stays, `elõdöntõ` maps); [PC-KB repair.rs `FontGaps`] (lexicon lists the word only with the letter restored); [SZTE-KOD] (`&otilde; &ucirc;` used for `ő ű`, `&ocirc;` accepted).
- **Validation**: `oracle:analyser`; `goldens` with the foreign keep-list; `attestation`.
- **Confidence**: `consensus`.

### REP-FONT-002 Decomposed, stacked and spacing-mark look-alikes of ő and ű
- **Statement**: Sequences that render like `ő ű` but are not the Hungarian code points (acute plus acute, diaeresis plus acute, base plus spacing double acute, base plus quote or prime) are normalised to `ő ű` inside Hungarian words; `ü` plus combining acute is special because NFC composes it to `ǘ` (U+01D8), which must be treated as `ű`.
- **Conditions** (after NFC, `WS-NORM-001`):
  - `o` + U+0301 + U+0301, `ó` + U+0301 (NFC of the former): look-alike of `ő`; `u` + U+0301 + U+0301, `ú` + U+0301: look-alike of `ű`
  - `ö` + U+0301 (NFC leaves it decomposed): look-alike of `ő`; `ü` + U+0301 composes to `ǘ` U+01D8: look-alike of `ű`; `ǜ` U+01DC (diaeresis + grave) likewise
  - `ő` + U+0301 or `ű` + U+0301 (a PDF extractor emits the precomposed letter and then a stray acute): the acute is spurious
  - base `o`/`u` followed by spacing U+02DD `˝`, U+2033 `″`, U+0022 `"` or U+201D `”` inside a word token (dead-key residue or 7-bit notation, REP-KEY-006, REP-ENC-014)
  - Cyrillic `ӧ` U+04E7, `ӳ` U+04F3, `ӱ` U+04F1 inside a Latin token (`WS-NORM-005`)
- **Output**: the Hungarian letter (`ő ű Ő Ű`, or `ö ü` for `ӧ ӱ`); the spurious acute after a precomposed accented letter is deleted.
- **Examples**: `kő`+U+0301 -> `kő`; `kó`+U+0301 -> `kő`; `elö`+U+0301 -> `elő`; `mǘködik` -> `működik`; `o˝r` -> `őr`; `kӧr` (Cyrillic) -> `kör`.
- **Exceptions**:
  - `ǘ ǜ` in Pinyin (`lǘ`, `nǚ`): keep when the token is not Hungarian by oracle.
  - A combining mark after an unaccented base in notation (`x̂`, `ȳ`, `Ẽ`) is content: only the listed bases and marks are touched.
  - `"` after a word inside quotation (`„Van"` typed with ASCII quotes) is punctuation: require a letter immediately after the mark for the 7-bit reading.
- **Predictability**: `rule`
- **Lexical input**: oracle acceptance for `"`, `”`, `″` cases
- **Interactions**: after `WS-NORM-001` (NFC composes the base first; prior art dropped the acute only after an already-accented base, which never matches `ü` + U+0301 because NFC has already produced `ǘ`); `WS-NORM-002`, `WS-NORM-005`.
- **Sources**: [UAX15] (compositions verified 2026-09-15: `o`+U+030B -> U+0151; `ü`+U+0301 -> U+01D8; `ö`+U+0301 not composed); [PT-CLEAN cleaning.md "A doubled acute NFC cannot compose"] (831 of about 1,050 combining marks in a fit sample were `ő` + U+0301, 8 were `ű` + U+0301).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-FONT-003 Shifted double acute font (TAMOP-type PDFs)
- **Statement**: Some PDFs (typically Hungarian educational material of the TÁMOP programme era) embed a font whose glyph encoding is shifted one slot across the double acute letters: a real `ő` is extracted as dotless `ı` (U+0131) and a real `ű` as `ő`, capitals `İ` (U+0130) for `Ő` and `Ő` for `Ű`; the decoder applies the inverse map only when the whole document is shifted.
- **Conditions**:
  - Fingerprint: dotless `ı`/`İ` count **>= 25** in the document and `ő`/`Ő` count >= 100 (every real `ű` became `ő`); for a source already confirmed as shifted, dotless count >= 10 and the `ő` count requirement is dropped. The dotless-count threshold does almost no work - moving it from 100 down to 5 with everything else held adds 67 documents of 1,317 - and lowering it to 25 costs no precision (20/20 by hand at either value); the `ő >= 100` requirement and the `ű`-share cap are what carry the rule;
  - Necessary consequence: `ű` count <= 2% of (`ő` + `ű`) (real `ű` effectively absent);
  - Spread: dotless `ı` occurs in >= 50% of 8 equal windows of the document (a font shift damages every stretch; quoted Turkish sits in one place);
  - Discriminator from ISO-8859-2 read as MacRoman (REP-ENC-010): `á é ö` are intact.
- **Output**: a repaired document carries no dotless `ı`, because the map is total; that is the test. No rate band separates damaged from clean documents: over 24.2 M documents the dotless rate is a smooth heavy tail from 1e-4 to 0.2 per word (21,808 documents above 0.02), and the 1e-4 to 5e-3 region (212 k documents) is one or two quoted Turkish names in healthy Hungarian.

  Three outcomes:
  - fully shifted (all conditions): translate `ı` into `ő`, `ő` into `ű`, `İ` into `Ő` and `Ő` into `Ű` over the document, simultaneously (not sequentially);
  - mixed font (fingerprint present but `ű` survives above 2%): do not apply the whole-document map; decode per region only if a region boundary is known from the PDF font resources; otherwise `flag` the document (`font_shift_residue`);
  - no fingerprint: keep.
  - After repair, residual dotless rate per word decides: damaged documents measured 0.067-0.085 dotless per word, clean ones <= 0.00064; a repaired document must fall to the clean band.
- **Examples**: `kerülı` -> `kerülő`; `mőködés` (shifted) -> `működés`; `İsz` -> `Ősz`.
- **Exceptions**: Turkish, Azerbaijani, Kazakh (Latin), Tatar text uses `ı İ` legitimately; the spread and `ű`-absence tests exclude quoted Turkish - measured, the `ő >= 100` clause alone removes every one of the 13 Turkish localisation files and Wiktionary articles found in a loosened band. Mathematical dotless `ı` (U+0131 before a combining hat) is rare and excluded by the count threshold. **Mixed documents from a crawl carrier**: document-hosting sites re-serving TAMOP-era PDFs mix clean page chrome ("2 évvel ezelőtt", "Látták: Átírás", carrying real `ő`) with a shifted body. The `ű`-share test still passes, because the chrome is small against a body with 100-500 shifted `ő`, so the three-outcome gate would choose "fully shifted" and turn `ezelőtt` into `ezelűtt`. **Route crawl-sourced documents to the per-region branch even when all fingerprint conditions hold.**
- **Predictability**: `rule` (map); `tendency` (thresholds, measured)
- **Lexical input**: none
- **Interactions**: before REP-FONT-001 and REP-DIAC (the shifted `ő` would otherwise be read as a correct letter); REP-QUAL-005 dotless rate; REP-NOISE-003 generates the forward shift.
- **Sources**: [PT-CLEAN cleaning.md "The font shift has three outcomes"] (125 PDFs: 1.6% fully shifted, 2.4% mixed font, 17% no text layer; a predecessor shipped 11 wrongly decoded mixed documents); [PT-CLEAN encoding.py `is_font_shifted`, `shift_spread`] (thresholds 100/10, 0.02, 0.5, 8 windows); [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens` (forward shift of clean text must be detected and inverted exactly); negative set Turkish. **Measured on a full scan of 24,183,772 documents**: 1,317 meet the fingerprint, and 20 of 20 drawn by hand from them are genuinely font-shifted Hungarian - 0 Turkish, 0 Azeri, 0 other - reading `fıbb` = `főbb`, `ülésérıl` = `üléséről`, `Képviselı` = `Képviselő`, `mőködı` = `működő`. A second sample of 20 from the loosened band (dotless 5-99, `ő` requirement dropped) is only 7 of 20 genuine, and all 13 false hits have `ő` count 0. Note also that **the 1,317 unrepaired shifted documents are not the ones the pipeline fixed**: `font_shift` fired on 1,490 documents, all in PDF sources, while these 1,317 are entirely in the crawls (1,312 of them from one document-hosting site re-serving the same TAMOP-era PDFs), i.e. the unrepaired half is unreachable by the current per-source repair declaration.
- **Confidence**: `measured` - the thresholds generalise for precision on 17,000 times more documents than the original 125 (`OQ-10-003` resolved). Two of the rule's clauses are load-bearing and one is not: dropping `ő >= 100` takes precision from 20/20 to 7/20 and raising the `ű`-share cap to 0.05 admits 2,498 unverified documents, while `dotless >= 100` can safely drop to 25. **The residual-rate test does not generalise** (see Output).

### REP-FONT-004 Offset subset fonts with letters in the punctuation range
- **Statement**: PDFs whose embedded font subset uses a private offset encoding extract runs where letters are shifted by a constant and accented letters land on ASCII punctuation (`|` for `ö`, `p` for `é`, `i` for `á`, `y` for `ó`); such runs are detected and flagged, and decoded only with a table recovered from the PDF font itself.
- **Conditions**:
  - Run of tokens with a high share of vowelless alphabetic tokens and letters adjacent to `| \ [ ] ^ &` inside tokens, not explainable by any code page hypothesis of REP-ENC-015;
  - Measured: neither vowelless-token share (worst 0.105, median 0.012) nor punctuation-adjacency (worst 0.111, then a smooth tail from 0.014) separated damaged from clean technical text in 336 theses (only 1 affected).
- **Output**: `flag` the run; a per-font substitution table may be solved as a monoalphabetic cipher over the run (constant offset hypothesis first, then frequency alignment against Hungarian letter frequencies) and applied only if >= 90% of decoded tokens are oracle-accepted.
- **Examples**: `Q\DOiERV]WyQ W|UWpQ iWKDODGiV` -> `nyalábosztón történő áthaladás` (observed; table recovered by hand).
- **Exceptions**: citations `[12]`, `R&D`, file paths, table rules produce the same surface signals; never gate on the signal alone.
- **Predictability**: `tendency`
- **Lexical input**: oracle for acceptance of a solved table
- **Interactions**: REP-QUAL-010 (label, not drop).
- **Sources**: [PT-CLEAN cleaning.md "A shifted font that stayed unfixed, and why"].
- **Validation**: `goldens` on synthetic offset runs (REP-NOISE-003).
- **Confidence**: `tendency`.

### REP-FONT-005 Private-use characters from symbol fonts
- **Statement**: Characters in the Private Use Area left by PDF extraction are decoded only through a published table for the font range they come from (Adobe Symbol at U+F020-U+F0FF, its bracket pieces at U+F8EB-U+F8FE) or through an observed convention with unambiguous evidence (ligature slots U+F000-U+F004); other private-use characters are dropped or kept per source declaration, never guessed.
- **Conditions**:
  - U+F020..U+F0FF: byte = code point - U+F000, decoded through the Adobe Symbol encoding (Greek letters, math operators, arrows, bullet U+2022 at byte B7, `≤` at A3, `≈` at BB, `∑` at E5);
  - U+F8EB..U+F8FE: bracket segments flattened to `( | [ { ) ] }`;
  - U+F000..U+F004 in text runs: `ff fi fl ffi ffl` (slot assignment as observed by prior art in PDF producers' fonts, not a published Unicode table; verify per source);
  - The source must declare that its PDFs set text in Symbol; a source whose PUA characters are Wingdings checkboxes (U+F0A3 as a form box in `Igen [] Nem []`) must declare `drop` instead, because Symbol decoding would turn the box into `≤`.
  - Other PUA blocks (U+E000..U+F8FF outside the ranges above, planes 15-16): per-document fonts without published tables.
- **Output**: decoded character; or deleted (`drop` declaration); or kept and counted (`keep` declaration for scholarly transcription fonts). A per-document table inferred from evidence (e.g. an ascending apparatus numbering at U+F661..U+F66A decoded to digits 0-9) is allowed only when the evidence pins the mapping and the table is scoped to that document.
- **Examples**: `j` U+F03D `1` -> `j = 1`; U+F0B7 list marker -> `•`; `speci`U+F001`káció` -> `specifikáció`; U+F0A3 in a form (drop source) -> removed.
- **Exceptions**: sources whose private-use range is content (critical editions, runic or Old Hungarian transcription fonts, `WS-CHAR-011`) declare `keep`.
- **Predictability**: `rule` (table), `lexical` (per-source declaration)
- **Lexical input**: source declaration
- **Interactions**: before REP-FONT-006 and REP-SPACE; REP-QUAL-005 counts residue.
- **Sources**: [PT-CLEAN formats/symbolfont.py] (34,223 PUA occurrences over 244 code points in a fit corpus; Symbol table; apparatus digit evidence); [PT-CLEAN cleaning.md "Decoding a private-use character needs to know the font"] (U+F0A3 54 times as Wingdings checkboxes).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-FONT-006 Presentation-form ligatures
- **Statement**: Unicode ligature characters U+FB00..U+FB06 are spelled out, so a word carrying one is the same word type as its plain spelling.
- **Conditions**: any of `ﬀ ﬁ ﬂ ﬃ ﬄ ﬅ ﬆ` in text.
- **Output**: `ﬀ` -> `ff`, `ﬁ` -> `fi`, `ﬂ` -> `fl`, `ﬃ` -> `ffi`, `ﬄ` -> `ffl`, `ﬅ` -> `st`, `ﬆ` -> `st`.
- **Examples**: `speciﬁkáció` -> `specifikáció`; `aﬃnitás` -> `affinitás`; `ﬂotta` -> `flotta`.
- **Exceptions**: none in prose; typographic source files that must round-trip keep the original in provenance.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: part of `WS-NORM-003` folding; the Hungarian spelling dictionary also treats these as input conversions [magyarispell hu.aff ICONV]; REP-OCR-001 covers ligatures split or merged by OCR.
- **Sources**: [UCD]; [PT-CLEAN encoding.py `_ligature`] (1,042 occurrences of U+FB01 in one fit corpus); [PC-KB repair.rs `Ligature`].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### REP-FONT-007 Mathematical alphanumeric symbols
- **Statement**: Characters from the Mathematical Alphanumeric Symbols block (U+1D400..U+1D7FF) emitted by formula extraction are folded to their base letters or digits by compatibility decomposition of that block only.
- **Conditions**: code points in U+1D400..U+1D7FF.
- **Output**: NFKC of the single character (`𝑇` -> `T`, `𝑚𝑖𝑛` -> `min`, `𝟏` -> `1`); the rest of the text is not NFKC-normalised.
- **Examples**: `𝑇 𝑅 𝑚𝑖𝑛 = 0,2𝑠` -> `T R min = 0,2s`.
- **Exceptions**: sources whose subject is mathematics typography may declare `keep`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: consistent with `WS-NORM-003` (no document-wide NFKC).
- **Sources**: [UAX15]; [PT-CLEAN cleaning.md "A formula's mathematical italics"].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-FONT-008 Soft hyphen
- **Statement**: U+00AD marks a permitted line-break position; it is deleted, never converted to a hyphen, because the word containing it is one word and not a compound.
- **Conditions**: U+00AD anywhere in text.
- **Output**: deleted; the position may be retained in provenance as a hyphenation hint.
- **Examples**: `szö`U+00AD`veg` -> `szöveg`; `vissza`U+00AD`tér` -> `visszatér`.
- **Exceptions**:
  - A soft hyphen at a doubled digraph break (`asz` U+00AD `szony`, produced by software that wrote out the digraph) is not a plain deletion: route to REP-SPACE-004 so `aszszony` is not produced.
  - U+00AD immediately after a mojibake lead (`Ã`U+00AD, `Ă`U+00AD) is the second byte of `í`: REP-ENC-002/003 run first.
- **Predictability**: `rule`
- **Lexical input**: none (oracle for the digraph exception)
- **Interactions**: `WS-CHAR-007`; after ENC; before REP-SPACE-004.
- **Sources**: [UCD]; [PT-CLEAN cleaning.md "Typographic whitespace is normalised"]; [AkH12 §226 f)] via `ORT-SPLIT-007`.
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-FONT-009 Zero-width, bidirectional and other format characters
- **Statement**: Characters of General Category Cf (zero width space, word joiner, BOM inside text, direction marks and embeddings, invisible operators) are removed from Hungarian prose, with two guards: ZWJ inside emoji sequences is kept, and bidirectional controls are kept inside right-to-left script runs.
- **Conditions**: any code point with General Category Cf; the set is derived from the Unicode Character Database, not enumerated by hand (a hand list of 31 code points missed 157 of the 188 code points of `Zs`, `Zl`, `Zp` and `Cf`).
- **Output**:
  - delete, except U+200D between two emoji or emoji modifiers, and U+200E/U+200F/U+202A..U+202E/U+2066..U+2069 adjacent to Hebrew or Arabic script;
  - U+FEFF at text start is a BOM (dropped at decoding, `WS-ENC-011`); inside text it is a zero width no-break space and deleted.
- **Examples**: `hogy`U+200B`a` is ambiguous: when U+200B stands between two letters of one word (`köszö`U+200B`nöm`) delete it; when it stands where a word space was lost (`hogy`U+200B`a`) replace with a space only if joining the letters yields a token the oracle rejects and splitting yields two accepted tokens.
- **Exceptions**: see Output.
- **Predictability**: `rule`
- **Lexical input**: oracle for the U+200B split/join decision
- **Interactions**: `WS-CHAR-007`; REP-SPACE-006 handles Zs/Zl/Zp.
- **Sources**: [UCD]; [PT-CLEAN cleaning.md "Typographic whitespace"] (20,925 invisible or typographic spaces in 83.2M characters, three quarters NBSP; derived-from-category rationale); [PC-KB repair.rs `INVISIBLE`] (ZWJ kept for emoji).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-FONT-010 Control characters
- **Statement**: C0 controls other than TAB, LF and CR, and C1 controls (U+0080..U+009F) are removed, but only after encoding repair, because C1 controls are the most reliable evidence of UTF-8 decoded as ISO-8859-1 and of undefined windows-125x bytes.
- **Conditions**: code points U+0000..U+0008, U+000B, U+000C, U+000E..U+001F, U+007F, U+0080..U+009F remaining after REP-ENC-002..015.
- **Output**: deleted; CR LF -> LF; lone CR -> LF; U+0085 NEL -> LF.
- **Examples**: `Å`U+0091`sz` before ENC -> `ősz`; a stray U+0007 -> removed.
- **Exceptions**: form feed U+000C is a page separator in `pdftotext` output: consumed by layout processing (REP-SPACE-010) before deletion.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: strictly after ENC; before SPACE.
- **Sources**: [UCD]; [PC-KB repair.rs `CONTROL`]; [PT-CLEAN encoding.py `_control`, `ORDER`].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-FONT-011 Mixed-script and dotless-i homoglyphs in single words
- **Statement**: A single Cyrillic or Greek look-alike, or a dotless `ı`, inside an otherwise Latin Hungarian word is replaced by the Latin letter when the repaired word is accepted; runs of four or more non-Latin letters are content.
- **Conditions**:
  - token with Latin letters plus one or two of: Cyrillic `а е і о р с у х ӧ ӱ ӳ` and capitals `А В Е К М Н О Р С Т Х`; Greek `ο ό α ν` and capitals `Α Β Ε Ζ Η Ι Κ Μ Ν Ο Ρ Τ Υ Χ`; `ı` (only outside REP-FONT-003 documents);
  - the UTS 39 skeleton of the token is oracle-accepted.
- **Output**: Latin letter; token-level provenance.
- **Examples**: `Мagyarország` (Cyrillic `М`) -> `Magyarország`; `kӧnyv` -> `könyv`; `mıért` -> `miért`.
- **Exceptions**: `NFκB`, `Θ(nαn)`, `α-sugárzás`, `β-karotin`, dialectological notation, Russian or Greek passages (90% of Cyrillic and Greek in academic sources sit in runs of four or more and are content); tokens with a hyphen between the Greek letter and the Hungarian word keep the Greek letter.
- **Predictability**: `rule`
- **Lexical input**: oracle
- **Interactions**: `WS-NORM-005`; REP-OCR-001 (OCR engines trained on multiple scripts produce the same substitutions).
- **Sources**: [UTS39]; [PT-CLEAN cleaning.md "Cyrillic and Greek in the academic sources is content"].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-FONT-012 Fraktur-era and early-print glyphs
- **Statement**: Long s `ſ`, superscript e above vowels (U+0364) and similar early-print glyphs in digitised older Hungarian texts are rendered to modern letters when the text is normalised; they are not errors in a diplomatic transcription.
- **Conditions**: `ſ` U+017F; `a`/`o`/`u` + U+0364 (early `ä ö ü` notation); `ꝛ` (r rotunda); used in prints before the 19th century and in some 19th-century typefaces.
- **Output** (normalising mode only): `ſ` -> `s` (`ſz` -> `sz`); `o` + U+0364 -> `ö`, `u` + U+0364 -> `ü`, `a` + U+0364 -> `ä` kept as foreign letter; length of `ö/ő`, `ü/ű` in such prints is not recoverable from the glyph and is left to REP-ARCH.
- **Examples**: `ſzép` -> `szép`; `ko`U+0364`nyv` -> `könyv`.
- **Exceptions**: diplomatic editions and sources declared `transcription` keep the glyphs.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `WS-NORM-003` (`ſ` folding); REP-OCR-006 (OCR confusion of `ſ` with `f`); REP-ARCH-001.
- **Sources**: [UCD]; [KOROMPAY18 pp. 96-97] (table of historical letter forms: `oͤ uͤ` notation for front rounded vowels, `ő ű` in modern form from the 19th century).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

## SPACE: spacing damage

### REP-SPACE-001 Letterspaced runs
- **Statement**: A run of single-letter tokens separated by single spaces, isolated from other letters on both sides, is letterspaced emphasis or a letterspaced heading (a convention of older Hungarian academic printing, and of PDF text layers generally) and is joined into one word, with guards against runs of real one-letter words and against formulas.
- **Conditions**:
  - Pattern: at least 3 letters, each followed by one space, tab or no-break space (`letter (sp letter){2,}`), with no letter immediately before the first or after the last; run after REP-SPACE-006 so NBSP-spaced runs match.
  - Guard A: keep if every letter of the run is a Hungarian one-letter word: `a á e é s` (articles, conjunction `s`, interjections `á é`); `és a s e` stays. **`ó` and `ő` are not included**: over 19,757 clean paragraphs they protect no observed clean run and cost one true run per 2,373 (`OQ-10-011`). Guard A does not fire on real text at all - the case it was written for (`és a s e nem`) does not occur in this negative set - and is kept only because its cost is one run in 2,373 while the damage it prevents is severe when it does occur; it should not be presented as a load-bearing clause.
  - Guard B: keep if the run has a capital after a lowercase letter (subscripted formula `a n r n P n`), unless REP-SPACE-002 identifies it as reversed.
  - Guard C: keep runs of letters separated by periods (initials `J. K.`) and list labels (`a) b) c)`).
  - Evidence: join if the joined form is oracle-accepted, or the run is all capitals (heading), or the run length **>= 3**; otherwise keep and label `letter_spaced`. The extra length was buying nothing: at >= 4 recall is 0.99663, at >= 3 it is 0.99958, with precision unchanged at 0.99664.
- **Output**: the joined word; only the isolated single-letter run is joined, surrounding words and initials stay (`AZ U. N. THRAK L O V A S ISTEN` -> `AZ U. N. THRAK LOVAS ISTEN`). Trailing absorbed one-letter word (`t a j e k o z t a t o j a a jog` -> `tajekoztatojaa jog`): split the last letter back off when the joined form without it is accepted and with it is rejected.
- **Examples**: `L O V A S` -> `LOVAS`; `semmi m á s !` -> `semmi más !`; `H I V A T A L B Ó L` -> `HIVATALBÓL`; `és a s e nem` kept.
- **Exceptions**: runs whose word gaps are as wide as letter gaps (`i r á n y í t ó s z e r e p e t`) cannot be segmented by joining: join the whole run only if a full segmentation into accepted words exists (REP-SPACE-003 DP over letters), else keep and label.
- **Predictability**: `rule` (pattern), `tendency` (guards)
- **Lexical input**: oracle
- **Interactions**: after REP-SPACE-006; before REP-SPACE-003; REP-QUAL-006 counts runs.
- **Sources**: [PT-CLEAN cleaning.md "Letterspacing is a typographic convention"] (19,708 documents of one source; repair touches 0.660% of 40,000 clean documents; guards A and B; one-letter set `a á e é s`); [PC-KB quality.rs `SPACED_RUN`]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens`; negative set of clean documents (count of changes). **Measured on 2,373 injected letterspaced runs against 19,757 clean paragraphs**: precision 0.99664, recall 0.99958 with guard {`a á e é s`} and run length >= 3; the guard-set extension to `ó ő` gives precision identical to five decimal places and recall 0.99916, so it is reverted. Guard A fires on 0 of the 8 clean runs found, and those 8 are the same under every setting: `abcde` 4, `abc` 2, `abcd` 1 (alphabet listings) and `árveréstkitűzi` 1 (a genuine letterspaced heading already in gold) - not one is a run of Hungarian one-letter words. **Caveat**: precision is bounded by a negative base of 8 runs, so the precision *differences* between settings rest on essentially no data and are reported as "identical" rather than as a ranking; the conclusion rests on the recall side (2,373 runs) and on the demonstration that neither guard set covers any observed clean run.
- **Confidence**: `measured` (guard set and run length).

### REP-SPACE-002 Reversed letterspaced runs from rotated text
- **Statement**: Text on a book spine or a rotated running head is extracted one glyph at a time in reverse; the run is recognised by a word-initial capital at its end and restored by reversing and splitting at capitals.
- **Conditions**:
  - letterspaced run of >= 6 letters refused by Guard B of REP-SPACE-001; and
  - the last letter is uppercase, or the reversed run contains more Hungarian non-palindromic digraphs (`gy ny ty ly cs`) than the run as written.
- **Output**: reverse the letters; insert a space before each uppercase letter that follows a letter; accept if the words are oracle-accepted or proper names.
- **Examples**: `t e z y g e J` -> `Jegyzet`; `n á t l o Z n n a m r e H` -> `Hermann Zoltán`.
- **Exceptions**: all-capital runs read forwards (REP-SPACE-001); runs ending lowercase (formulas) are not reversed.
- **Predictability**: `tendency`
- **Lexical input**: oracle
- **Interactions**: inside REP-SPACE-001.
- **Sources**: [PT-CLEAN cleaning.md "A page the extractor read backwards"] (2,223 runs per 15 Mchar of one source); [PT-CLEAN encoding.py `_is_reversed`].
- **Validation**: `goldens`.
- **Confidence**: `tendency`.

### REP-SPACE-003 Per-syllable fragmented words from PDF text layers
- **Statement**: PDF text layers that position each syllable as a separate span extract words broken into syllable-sized fragments (`A Kor mány ren de le tei`); adjacent fragments are joined by a full-coverage dynamic programme that maximises absorbed fragments under oracle acceptance, only in documents and runs with fragmentation evidence.
- **Conditions**:
  - Document trigger: share of lowercase alphabetic tokens of length <= 3 among all alphabetic tokens **>= 0.35, measured over a whole document of at least 300 words** (healthy Hungarian 0.239 median and 0.333 maximum over 1,978 documents; fragmented layers 0.743 median and 0.599 minimum over 127 - the two classes are separated by a clear gap from 0.333 to 0.599, and the previous trigger of 0.30 sat *below* the clean maximum, measuring precision 0.799, i.e. one clean document in sixty-two sent into the fragment-rejoin pass). Applied per block instead of per document the equivalent value is **0.45** (at 0.30 per block, precision is 0.299 with 2,857 false positives of 19,757).
  - Run: maximal sequence of alphabetic tokens (trailing punctuation ends the run and is re-attached).
  - Run evidence: at least 2 tokens, and at least 25% of the run, are lowercase, length <= 4, and rejected by the oracle as standalone words. Long or capitalised rejected tokens (foreign words, abbreviations) are not evidence. **This clause is not calibrated** and could not be: with an attestation lexicon as the oracle it fires on only 4% of damaged sentences and its precision never exceeds 0.83 at any swept setting (9,905 damaged runs against 109,180 clean runs), because Hungarian syllables are largely themselves words (`a`, `az`, `el`, `le`, `de`, `meg`, `ki`, `be`, `fel`) and only a morphological analyser can say "this is not a possible standalone word form in this position"; the damage generator also spaces a *stretch* of sentences, so undamaged sentences inside a damaged document dilute any per-sentence measure. Needs the analyser as the oracle plus pairs carrying per-sentence damage spans.
  - A spaced hyphen or en dash between two fragments is evidence on its own for that junction (line-break remnant), and exempts the group from the mean-length cap.
  - Group constraints: joined length >= 5; at most 12 fragments; mean fragment length <= 4.5 characters (unless bridged by a hyphen); joined form oracle-accepted.
- **Output**: DP over the run choosing the partition that maximises the number of fragments absorbed into accepted groups, ties broken by fewer groups; unjoined fragments stay as they were; the original dash token is re-emitted verbatim when not absorbed.
- **Examples**: `A Kor mány ren de le tei` -> `A Kormány rendeletei` (`A` is a standalone word and not evidence; the DP absorbs `Kor mány` and `ren de le tei` into two accepted groups); `ma ga tar tás` -> `magatartás`.
- **Exceptions**:
  - Two valid adjacent words that also form a valid compound (`nem csak` -> `nemcsak`, `egy nap` -> `egynap`, `s a vas` -> `savas`) are joined only inside a run that has evidence; an audit found 84% of joins in triggered documents were such merges when evidence was not required.
  - Residual split compounds (`magatartás minták`) where both halves are accepted remain; REP-SPELL-011 may join them by `ORT-TOG` rules.
- **Predictability**: `tendency`
- **Lexical input**: oracle (batch queries: collect all candidate groups in a first pass, answer in one call, decide in a second pass)
- **Interactions**: after REP-SPACE-001; before REP-SPACE-004; REP-NOISE-004 produces syllable-spaced input with the syllabifier of `ORT-SPLIT`.
- **Sources**: [PT-CLEAN encoding.py `_fragment_rejoin`, constants FRAG_JOIN_MIN 5, FRAG_MAX 12, FRAG_MEAN_LEN 4.5, FRAG_RUN_SHARE 0.25, FRAG_RUN_MIN_BAD 2, FRAG_EVIDENCE_LEN 4]; [PT-DAMAGE damage.py `syllable_spaced`]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens` from REP-NOISE-004 pairs; `oracle:analyser`; negative set clean text (no joins expected). **At 0.35 the document trigger measures precision 1.0000 and recall 1.0000 on 127 fragmented and 1,978 clean documents; at the previous 0.30, precision 0.799.** Anything in [0.34, 0.59] is equivalent on this data and 0.35 is chosen as the low end so that lightly fragmented layers are still caught. 1,978 clean documents is ample; **127 damaged documents is under 300 and that side is provisional**, though the decisive number - that clean documents reach only 0.333 - rests on the 1,978.
- **Confidence**: `tendency`; the document trigger is `measured`, the run-evidence clause remains unmeasured.

### REP-SPACE-004 Line-end hyphenation: rejoining and doubled digraph restoration
- **Statement**: A word broken at line end with a hyphen is rejoined by deleting the hyphen and the line break; when the break falls inside a simplified doubled multi-character letter, which AkH writes out in full on both lines (`asz-szony`), the simplified form is restored, except at compound boundaries where full doubling is standard.
- **Conditions**:
  - `<left>` + `-` (U+002D, U+2010, U+00AD, or U+FFFD per REP-ENC-012) + line break + optional indentation + `<right>` with `<right>` starting with a lowercase letter; or the same with a hyphen followed by a single space inside a reflowed line when REP-SPACE-003 or layout evidence says the break was at line end.
  - Digraph case: `<left>` ends with multi-character letter `D` in {`cs dz gy ly ny sz ty zs dzs`} and `<right>` starts with the same `D`.
  - Candidates: C1 = `<left><right>` (plain join); C2 = `<left minus D>` + first character of `D` + `<right>` (simplified doubling, digraph case only); C3 = `<left>-<right>` (genuine hyphen, REP-SPACE-005).
- **Output**:
  - digraph case: C2 if accepted and the morphological boundary at the junction is a suffix boundary or no boundary; C1 if accepted with a compound boundary at the junction; if both accepted, prefer the analysis with the compound boundary only when the analyser marks it (`kulcs|csomó`), else C2.
  - `dz`/`dzs` case follows the same logic (`edz-dze` -> `eddze`, `bridzs-dzsel` -> `briddzsel`).
  - non-digraph case: C1 if accepted; C3 if C1 rejected and C3 accepted; if neither accepted, C1 when both halves are rejected as standalone words (a break inside an unknown word), else keep the hyphen and label.
- **Examples**: `asz-` / `szony` -> `asszony`; `visz-` / `sza` -> `vissza`; `meny-` / `nyi` -> `mennyi`; `pogy-` / `gyász` -> `poggyász`; `Kodály-` / `lyal` -> `Kodállyal`; `kulcs-` / `csal` -> `kulccsal`; `kulcs-` / `csomó` -> `kulcscsomó`; `jegy-` / `gyűrű` -> `jegygyűrű`; `ren-` / `delet` -> `rendelet`; `Magyaror-` / `szág` -> `Magyarország`.
- **Exceptions**: three identical letters and proper names with doubled final letters keep their hyphen (`Kiss-` / `sel` -> `Kiss-sel`, `sakk-` / `kör` -> `sakk-kör`): C3 is chosen because C1/C2 are rejected (`ORT-ASSIM-017`); see REP-SPACE-005.
- **Predictability**: `rule` given morphology
- **Lexical input**: oracle with compound boundary marks
- **Interactions**: implements `WS-DIG-012`; forward rules `ORT-SPLIT-007`, `ORT-SPLIT-008`, `ORT-SPLIT-011`, `ORT-SPLIT-015`, `ORT-SPLIT-017`; after REP-FONT-008; REP-NOISE-004 `hard_wrap` generates the input.
- **Sources**: [AkH12 §226 f)-g), §227, §238]; [PT-DAMAGE damage.py `_hyphenated`, `hard_wrap`]; [PT-CLEAN cleaning.md "PDF without vocabulary"] (prior heuristic: lowercase continuation rejoins, capital keeps the hyphen); [L22 p. 420] (join when the hyphen-separated strings form a word in a word list).
- **Validation**: `goldens` for every digraph and for compound boundaries; `oracle:analyser`.
- **Confidence**: `normative` (forward rule), `consensus` (inverse).

### REP-SPACE-005 Genuine hyphens at line end
- **Statement**: A hyphen at line end may be a lexical hyphen (compound with a proper name, acronym, suffix after abbreviation or numeral, three identical letters, hyphenated coordinative compound) and must be kept when the line is rejoined; AkH writes such a hyphen only once, at the end of the line.
- **Conditions**: line-end hyphen where any of:
  - `<left>` ends with an uppercase letter, digit, `%`, `°`, or an abbreviation period (`MÁV-` / `ot`, `2024-` / `ben`, `stb.-` / `t`);
  - `<right>` begins with an uppercase letter (`Kelet-` / `Európa`), or `<left>` begins with an uppercase letter and C1 is rejected while C3 is accepted (`Petőfi-` / `szobor`);
  - C1 creates three identical consonant letters (`sakk-` / `kör`, `Széll-` / `lel`);
  - C3 is accepted and C1 is rejected by the oracle (`tűzzel-` / `vassal`, `élet-` / `halál`).
  - An en dash at line end (`magyar–` / `angol`) is always kept.
- **Output**: C3 `<left>-<right>` (or `<left>–<right>`).
- **Examples**: `MÁV-` / `ot` -> `MÁV-ot`; `Kossuth-` / `díj` -> `Kossuth-díj`; `sakk-` / `kör` -> `sakk-kör`; `tűzzel-` / `vassal` -> `tűzzel-vassal`; `magyar–` / `angol szótár` -> `magyar–angol szótár`.
- **Exceptions**: specialist texts repeat the hyphen at the start of the next line (`nátrium-` / `-klorid`): collapse the two hyphens to one.
- **Predictability**: `rule` (character contexts), `lexical` (oracle cases)
- **Lexical input**: oracle
- **Interactions**: `ORT-SPLIT-017`, `ORT-HYPH`, `ORT-ABBR-009`, `ORT-ASSIM-017`; decided jointly with REP-SPACE-004.
- **Sources**: [AkH12 §238, §270]; [PT-CLEAN cleaning.md "Hyphenation"] (capital after hyphen keeps it).
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### REP-SPACE-006 Typographic spaces and line separators
- **Statement**: Space separators other than U+0020 (Zs category) are normalised to U+0020 in training text, U+2028 and U+2029 to line breaks, and runs of spaces collapsed, unless the consumer requires typographic fidelity, in which case no-break spaces are kept only where `PUN-SPACE-004` allows them.
- **Conditions**: U+00A0, U+1680, U+2000..U+200A, U+202F, U+205F, U+3000 (Zs except U+0020); U+2028 (Zl), U+2029 (Zp); two or more consecutive U+0020 inside a line.
- **Output**:
  - plain mode: Zs -> U+0020; Zl, Zp -> LF (never deleted: deleting joins `sor`U+2028`sor` into one token); multiple spaces -> one space; spaces at line start/end trimmed.
  - typographic mode: keep U+00A0 or U+202F between a number and a unit or percent-like symbol, in digit grouping, and after `§`-type signs (`PUN-SPACE-004`, `PUN-SPACE-005`); convert all others to U+0020.
- **Examples**: `23`U+00A0`816` -> `23 816` (plain) or kept (typographic); `sor`U+2028`sor` -> `sor` LF `sor`.
- **Exceptions**: preformatted code blocks and tables keep runs of spaces.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: after ENC (the NBSP image of CP852 `á`, REP-ENC-008) and FONT; before REP-SPACE-001; `WS-CHAR-008`, `WS-NORM-007`.
- **Sources**: [UCD]; [PT-CLEAN encoding.py `_invisible`] (derived from categories; line separator to newline); [AkH12 §240, §274-§275] via `PUN-SPACE-004`.
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-SPACE-007 Missing space after punctuation
- **Statement**: A sentence-final mark, comma, semicolon or colon glued to a following word is separated by a space, with exemptions for decimals, times, abbreviations with internal periods, ordinal dates, URLs, e-mail addresses, file names and hyphenated suffixes.
- **Conditions**:
  - `[letter][.?!]` + `[Uppercase letter][lowercase letter]` (sentence boundary) ; `[letter or digit][,;:]` + `[letter]`; `[”»)]` + `[letter]`.
  - Exempt when the token containing the junction: matches a URL, e-mail, domain (`index.hu`, `www.`), file name (`[.][a-z]{2,4}$`); is a digit group on both sides (`3,5`, `10:35`, `2024.09.15.`, `1.2.3`); is an abbreviation written solid with internal periods in the source (`Bp.-en`, `ún.-t`); is an initial sequence (`J.K.`); contains `-` right after the period (suffix after abbreviation or ordinal, `stb.-t`, `3.-nak`).
- **Output**: insert U+0020 after the mark.
- **Examples**: `mondat.Következő` -> `mondat. Következő`; `alma,körte` -> `alma, körte`; `3,5` kept; `index.hu` kept; `10:35-kor` kept.
- **Exceptions**: listed in Conditions.
- **Predictability**: `rule`
- **Lexical input**: abbreviation list (`PUN-ABBR`, `ORT-ABBR`)
- **Interactions**: implements the insertion half of `PUN-SPACE-006`; after REP-SPACE-004 (a rejoined word may remove a false junction).
- **Sources**: [AkH12 §240] via `PUN-SPACE-001`; [PT-DAMAGE corrupt.py `squash_spaces`] (the commonest slip: no space after comma).
- **Validation**: `goldens`; `oracle:lightproof` for comparison.
- **Confidence**: `consensus`.

### REP-SPACE-008 Spurious space before punctuation and inside brackets
- **Statement**: A space before an attaching mark (`. , ; : ? ! ) ”`) or after an opening mark (`( „ »`) is removed; OCR and PDF extraction insert such spaces systematically.
- **Conditions**: ` +[.,;:?!)\]”«]` or `[(\[„»] +`, excluding: the spaced en dash ` – `, the ratio `6 : 3`, the omission mark `[…]`, a leading ellipsis, French-style spacing inside quoted French text.
- **Output**: delete the spaces.
- **Examples**: `Szia , Péter !` -> `Szia, Péter!`; `( lásd )` -> `(lásd)`; `„ Igen ”` -> `„Igen”`.
- **Exceptions**: as Conditions.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: implements the removal half of `PUN-SPACE-006`; REP-OCR-004.
- **Sources**: [AkH12 §240] via `PUN-SPACE-001`; [L22 p. 423] (OCR output often has an extra space before punctuation; gold must attach the mark).
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### REP-SPACE-009 Word fusion and word splits
- **Statement**: Lost word spaces (fusion) and inserted spaces inside words (splits), the most frequent OCR-level differences in Hungarian scanned books, are repaired with an oracle segmentation that prefers the reading with fewer rejected tokens and does not undo legitimate compounds or separations.
- **Conditions**:
  - Fusion candidate: token rejected by the oracle, length >= 6, with a split point where both parts are accepted (or the left part is a function word `a az egy és hogy nem is meg`), and the split pair is at least as frequent as the fused string.
  - Split candidate: two adjacent tokens where at least one is rejected, whose concatenation is accepted.
  - Both: if the fused token is itself accepted (`nemcsak`, `egyelőre`, `hogyha`), keep it; if both parts of a split are accepted and the join is accepted too, keep the split unless `ORT-TOG` requires the solid form (REP-SPELL-011).
- **Output**: the segmentation with the minimum number of rejected tokens; ties: keep the input.
- **Examples**: `ahogyan` kept (accepted); `hogya` -> `hogy a`; `azegész` -> `az egész`; `nagyon jó` kept; `gyö nyörű` -> `gyönyörű`; `kor mány` -> `kormány`.
- **Exceptions**: proper names and foreign words (rejected in both readings): keep. Text already processed by REP-SPACE-003 in fragment mode: skip.
- **Predictability**: `tendency`
- **Lexical input**: oracle, frequency
- **Interactions**: REP-OCR-004; REP-SPELL-011 (writing together or apart is a spelling decision, not a damage decision).
- **Sources**: [L22 p. 422 Table 2] (JiM test set, columns OCR -> silver text: OCR had a spurious space 3,523 times (word split) and lacked a space 360 times (fusion), OCR `é` for text `e` 581 times, OCR lacked `r` 232 times, OCR had an extra `y` 11 times, OCR `zd` for `v`, `űié` for `üle`); [PT-DAMAGE damage.py `split_fused`].
- **Validation**: `goldens` from REP-NOISE-008; `oracle:analyser`.
- **Confidence**: `tendency`.

### REP-SPACE-010 Hard line breaks, page furniture and paragraph reflow
- **Statement**: Line breaks inside paragraphs (from PDF extraction, OCR, fixed-width e-mail) are turned into spaces after hyphen handling, paragraph breaks are kept, and page furniture (running headers, footers, page numbers) interleaved with the text is removed before rejoining so it cannot split words or sentences.
- **Conditions**:
  - Paragraph break: blank line; or vertical gap larger than the document median line gap (geometry available); or a line ending in sentence-final punctuation followed by an indented or capitalised line when no geometry exists.
  - Running header/footer: a line in the top or bottom margin band whose text repeats across pages; page number: a margin line containing no letters. Position alone or repetition alone is insufficient.
  - Form feed U+000C marks page boundaries in `pdftotext` output.
- **Output**: remove furniture; apply REP-SPACE-004/005 at line ends; join remaining lines inside a paragraph with one space.
- **Examples**: `...a 45` LF `jó birssuhogót...` with `45` a page number -> `...a jó birssuhogót...`.
- **Exceptions**: verse, lists, tables, addresses and dialogue lines keep their line breaks (detect by short lines with no terminal hyphen and aligned starts, or source markup).
- **Predictability**: `tendency`
- **Lexical input**: none
- **Interactions**: before REP-SPACE-004 (a page number between `Magyaror-` and `szág` blocks the join); idempotent on already reflowed text (measured 0 changes of 106 documents).
- **Sources**: [PT-CLEAN cleaning.md "PDF without vocabulary and without font size", "The layout filter was tested against its own output"]; [L22 pp. 421-423] (page numbers and hyphenation as coherence errors in OCR output, e.g. a page number `45` inside a sentence).
- **Validation**: `goldens`; idempotence.
- **Confidence**: `consensus`.

### REP-SPACE-011 Detached suffix hyphens and spaced hyphens
- **Statement**: A suffix separated from its abbreviation, acronym, numeral or name by a space around the hyphen (`MÁV -ot`, `2024 -ben`, `EU - s`) is reattached; a spaced hyphen between words used as a dash is converted according to `PUN-DASH`.
- **Conditions**:
  - `<token> -<suffix>` or `<token> - <suffix>` where `<suffix>` is a Hungarian case, possessive or derivational suffix form (closed list from `NOM-CASE`, `DER`) and `<token>` is an acronym, abbreviation, digit string, symbol, or a word requiring a hyphen before suffixes (`ORT-ABBR-009`, `ORT-NUM`, `ORT-FOR`);
  - `<word> - <word>` where both are ordinary words: dash usage.
- **Output**: `<token>-<suffix>`; the spaced hyphen between words -> ` – ` (en dash with spaces) in typographic mode.
- **Examples**: `MÁV -ot` -> `MÁV-ot`; `2024 -ben` -> `2024-ben`; `EU - s` -> `EU-s`; `alma - körte` -> `alma – körte`.
- **Exceptions**: minus signs in arithmetic (`5 - 3`), ranges with digits (`1990 - 2000` -> en dash without spaces per `PUN-DASH`).
- **Predictability**: `rule`
- **Lexical input**: suffix inventory
- **Interactions**: `ORT-ABBR-009`, `PUN-DASH`, REP-SPELL-012.
- **Sources**: [AkH12 §282, §288, §265 d)] via `ORT-ABBR-009`.
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-SPACE-012 Doubled and trailing spaces, tabs inside prose
- **Statement**: Two or more spaces between words, tabs used as word gaps and trailing spaces are reduced to one space or removed; the damage is also a generator slip (15% of adjacent word pairs in a selected span).
- **Conditions**: `\S {2,}\S` or `\S\t+\S` inside a prose line.
- **Output**: one U+0020.
- **Examples**: `nagyon  jó` -> `nagyon jó`.
- **Exceptions**: preformatted blocks, tables, poetry indentation.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: part of REP-SPACE-006; `PUN-SPACE-002`.
- **Sources**: [AkH12 §240] via `PUN-SPACE-002`; [PT-DAMAGE corrupt.py `squash_spaces`].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

## DIAC: diacritic loss and restoration

### REP-DIAC-001 Detecting accent-stripped text
- **Statement**: A Hungarian text segment written without accents is recognised by an accent density far below the Hungarian baseline together with positive Hungarian evidence from accent-free function words and from oracle acceptance of restored forms; the detector abstains on short segments and never classifies foreign text as stripped Hungarian.
- **Conditions**:
  - Baselines (measured 2026-09-16, `REP-QUAL-001`): **47.77% of tokens, 11.76% of letters, 9.57% of characters**, on 9,644,687 Hungarian documents. The literature figures they replace: 43.16% of tokens [NS15], 35.75% of words and 6.59% of characters [LY20].
  - Segment size: >= 30 alphabetic tokens (sentence or paragraph); below that, borrow evidence from the surrounding document or abstain.
  - Stripped when: accented-letter share among letters **< 0.2%** (the classes are perfectly separated - the lowest clean segment of 19,757 sits at 0.66% and the highest stripped segment at 0.00% - so any cut in [0.0005, 0.05] behaves identically; 0.2% is chosen for 3.3x headroom instead of the previous 1.33x, for formula-heavy or heavily quoted Hungarian), and **>= 2 distinct** tokens from the accent-free Hungarian function-word signature {`es` (for `és`), `hogy`, `nem`, `egy`, `meg` (also for `még`), `van`, `volt`, `az`, `a`, `is`, `mar` (`már`), `igy` (`így`), `utan` (`után`), `mint`, `csak`, `de`, `ha`, `kell`, `lehet`, `nagyon`, `amikor`, `ahol`, `akkor`, `mert`, `vagy`} **as a cheap pre-filter only**, and **>= 50% of tokens rejected as written become accepted after the best restoration (REP-DIAC-004), computed only when at least 5 tokens are rejected as written**. The restoration clause is the test; it is the only clause that is not a Hungarian word list, and it is what keeps stripped English, Dutch and Latin out. The former clause requiring the stripped tokens `es`, `mar`, `igy`, `utan`, `ugy`, `tobb`, `kozott` to occur at least twice is **dropped**: it costs 38 points of recall and buys no precision.
  - Partially stripped (class detection): REP-DIAC-002.
- **Output**: segment label `stripped` with its class, or `accented`, or `abstain`.
- **Examples**: `arvizturo tukorfurogep` (2 tokens) -> `abstain` alone; `Tegnap a varosban voltam es lattam, hogy mar nyitva van a vasar.` -> `stripped` (`es`, `mar`, `varosban` restorable).
- **Exceptions**: English, Latin, Indonesian and other unaccented languages share `a`, `is`, `de`, `van` (Dutch/Afrikaans): the Hungarian-specific signature (`hogy`, `nem`, `egy`, `meg`, `es` + restoration acceptance) is required; URLs, code and identifiers are excluded from counts; flying-accent text (REP-ENC-014) is decoded first.
- **Predictability**: `tendency`
- **Lexical input**: function-word list; oracle
- **Interactions**: after ENC, FONT, SPACE; before REP-SPELL; REP-QUAL-001, REP-QUAL-008.
- **Sources**: [NS15 §5] (56.84% correct without accent, 47.09% for words with vowels); [LY20 Table 1] (hu: 6.59% diacritic characters, 35.75% diacritic words); [ATT]; [PC-HU quality.ron] (function word list); [PT-CLEAN hungarian.py `accent_density`]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens`; negative sets English, Dutch, Latin. Measured on 7,642 stripped and 19,757 clean paragraphs: the **previous** conjunction gives precision 1.00000 and recall 0.53088, i.e. it misses 47% of stripped Hungarian; the proposed one (`sig >= 2` and restoration clause `>= 0.50` over `>= 5` rejected tokens) gives precision 0.99920 and recall 0.98220. The restoration clause alone at 0.50 with the floor reaches 0.99920 / 0.98220, and the floor matters - without it false positives rise from 6 to 103, because a clean paragraph with one rejected token that happens to restore gives a ratio of 1.0. **Foreign negatives, partly measured** (2026-09-18): English is now covered. 63 paragraphs of English prose (3,813 words, the project's own design documents, which are unaccented and so are the hard case) were run through the detector and none was classified as stripped Hungarian, so the restoration clause does hold English out; the test is [`crates/picur-morph-repair/tests/foreign.rs`](../../crates/picur-morph-repair/tests/foreign.rs). Dutch, Latin and Indonesian are still unmeasured, and the sample is far short of the 2,000 segments this asks for. If the restoration clause does not sit near zero on those three as well, the signature pre-filter must go back up rather than down. The `>= 30 alphabetic tokens` floor never fires on this data (0.0% of the stripped paragraphs are shorter) and is left alone, untested.
- **Confidence**: `measured` (thresholds); `tendency` (foreign-language behaviour).

### REP-DIAC-002 Classes of partial accent loss
- **Statement**: Accent loss is often partial and systematic; the class determines which letters are candidates for restoration, which sharply reduces ambiguity, and is detected from which accented letters survive.
- **Conditions**: per document or segment; computed over >= 200 letters of the affected families, **which for {`ö`, `ő`} means about 10,000 letters (1,600 words) of running text and for {`ü`, `ű`} about 29,000 letters (4,500 words): classes B and C are document-level tests, never segment-level**. Family density per letter of running text: {`ö`, `ő`} 0.01950, {`ü`, `ű`} 0.00686, {`a`, `á`} 0.12500, {`e`, `é`} 0.13660, {`i`, `í`} 0.04762. On a 13,531-segment test set the `>= 20` floor alone leaves only 1,059 segments (7.8%) decidable for {`ö`, `ő`} and 37 (0.3%) for {`ü`, `ű`}. `share(x)` = count of `x` / count of its family; detection column below.
- **Output**: class label and candidate restriction:

| Class | Surviving letters | Missing letters | Cause | Detection | Candidates |
|---|---|---|---|---|---|
| `A` full | none | all 9 | foreign keyboard, SMS, e-mail address style, ASCII transliteration | REP-DIAC-001 | all of Table T-DIAC-1 |
| `B` double acute to base | `á é í ó ö ú ü` at baseline | `ő ű` -> `o u` | Windows best-fit into windows-1252, some transliterators | share(ő) < 0.02 while share(á), share(é), share(ö) within 50% of baseline; **the share(ű) < 0.02 test is evaluable only above 29,000 letters and is optional** | `o` -> {o, ő}; `u` -> {u, ű} |
| `C` double acute to umlaut | `á é í ó ö ú ü` | `ő ű` -> `ö ü` | German or other layouts with `ö ü`, fonts without double acute, handwriting habit | share(ő) < 0.02, share(ö) > 0.40 (baseline ö/(ö+ő) = **0.536 corpus-wide, but 0.39 in administrative and 0.60 in narrative text - the share(ö) > 0.40 clause puts undamaged administrative Hungarian on the wrong side of the test**). The clause is in any case redundant given share(ő) < 0.02 (sweeping it over 0.40 / 0.70 / 0.90 changes nothing, class C damage driving share(ö) to exactly 1.0) and is retained only as a guard against class B being read as C | `ö` -> {ö, ő}; `ü` -> {ü, ű} |
| `D` acute loss | `ö ü` (and sometimes `ő ű`) | `á é í ó ú` -> `a e i o u` | German layout, mobile keyboards with umlaut keys only | share(á) < 0.03 and share(é) < 0.03 while share(ö) near baseline | `a e i o u` -> acute; plus class C if `ő ű` also missing |
| `E` í loss only | all but `í` | `í` -> `i` | Hungarian layout on ANSI boards (no `OEM_102` key; `í` only by AltGr+J or dead acute), REP-KEY-005. **In this corpus the class fires mostly on causes it is not the repair for**: scanned theses (`bme-omikk` 7.6% of eligible documents, `szte-dokt` 5.2%), which is OCR acute loss, and pre-reform literary orthography (6.7% of `literature`) | **share(í) < 0.01 with at least 50 letters of the {i, í} family** (about 165 words); baseline **0.124** measured corpus-wide over 63.42 G letters, superseding 0.14 [NS15] / 0.13 [ATT]. At the previous 0.02 with a floor of 20 the measured precision is 0.954 - 452 of 9,291 decidable clean segments are labelled class E - because `í` is only 13.66% of its family and a 400-character clean paragraph routinely contains zero `í` | `i` -> {i, í} |
| `F` capitals only | lowercase accents at baseline | accents on capitals (`Á` -> `A`, `É` -> `E`) | all-caps headings typed without accents, some signage fonts | uppercase accented share < 10% of lowercase accented share | uppercase vowels only |
| `G` substitute glyphs | varies | `ő ű` -> `õ ô û` | fonts, code pages | REP-FONT-001 | not a DIAC case |

- **Examples** (damaged form of the gold word): `előtt` -> `elott` (A or B), `elött` (C); `kávézó` -> `kavezo` (A or D); `kerülő` -> `kerulö` (C+D); `írás` -> `irás` (E); `ÁLLAT` -> `ALLAT` (F).
- **Exceptions**: classes can combine (C+D is the typical German-keyboard pattern); detection uses surviving-letter ratios only when their counts are **>= 50 for the {i, í} and {ö, ő} families** (>= 20 elsewhere).
- **Predictability**: `tendency`
- **Lexical input**: letter baselines (REP-QUAL-001)
- **Interactions**: REP-DIAC-003 uses the class to restrict candidates; REP-ENC-011 is class B; REP-KEY-005 explains class E; `ORT-CAP-002` (accents are kept on capitals, so class F is an error).
- **Sources**: [NS15 Table 1] (per-vowel distributions used as baselines); [MS-BF bestfit1252.txt] (class B mapping); [KBDLAYOUT KBDHU] (class E cause); [PT-DAMAGE corrupt.py `strip_accents`] (partial loss is the common case); [AkH12 §6] via `ORT-CAP-002`; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens` from REP-NOISE-005 per class; confusion matrix of class detection. Measured 2026-09-16 by **injection** into 13,531 gold segments of > 400 characters, each also kept clean as its own negative (no labelled partially-stripped corpus exists; that is what `OQ-10-006` asks for): class D 1.0000 precision / 0.9996 recall on 13,527 decidable segments - exact, **confirmed, no change**, and the only class whose families are dense enough to decide on a paragraph; class C 0.9991 on 1,059; class B 0.9875 on 79 (**provisional**, under 300, and below target - class B is the hardest case because `ő` -> `o` shrinks the `ö`/`ő` family as well as skewing it); class E 0.9955 on 439 at the proposed floor (**provisional**). Real prevalence over the 1,067,174 Hungarian documents long enough to classify: B 0.245% (2,616), E 0.210% (2,246), **C 3 documents and D 4 documents, which must be recorded as unmeasured rather than as measured-zero** - n = 3 and n = 4 cannot distinguish "does not happen" from "the detector does not fire", and since class D's detector is exact by injection, the phenomenon is simply not in this corpus.
- **Confidence**: `tendency`; class D `measured`. Class E as specified is the weakest part of this rule.

### REP-DIAC-003 Candidate generation
- **Statement**: Every unaccented or partially accented word is expanded into the set of Hungarian spellings that differ only in accent marks, restricted by the loss class, and the set is filtered through the morphological analyser as a transducer composition rather than by enumerating strings.
- **Conditions**: token is alphabetic, not guarded by REP-DIAC-009, contains at least one vowel letter whose family has more than one member in the active class.
- **Output**:
  - Letter expansion (Table T-DIAC-1): `a` -> {a, á}; `e` -> {e, é}; `i` -> {i, í}; `o` -> {o, ó, ö, ő}; `u` -> {u, ú, ü, ű}; `ö` -> {ö, ő}; `ü` -> {ü, ű}; `ó` -> {ó, ő} only in class C+D mixtures; uppercase identically. Consonants are never changed.
  - Candidate count without the analyser is 2^(a+e+i) * 4^(o+u) (`tukorfurogep`: 2^1 * 4^4 = 512); therefore compose an accent-insertion transducer (each vowel arc emits its family) with the analyser and extract only accepted paths, each with its analysis (lemma, morpheme boundaries, tags).
  - If no accepted path exists (unknown stem), generate candidates from a character model limited to the 3 best per token and mark them `unverified`; they are applied only with REP-DIAC-008 policy `aggressive`.
- **Examples**: `kerek` -> {`kerek`, `kerék`, `kérek`} (all accepted; `kérék` rejected); `hid` -> {`híd`} (`hid` rejected; bucket `hid` 7 is damage); `tukorfurogep` -> {`tükörfúrógép`}. **91.6% of accent-stripped tokens have more than one attested accent-only respelling** (measured over 565,493 aligned tokens: 0 candidates 0.47%, 1 candidate 7.97%, 2 candidates 40.75%, 3 or more 50.81%): the analyser filter reduces the candidate count but does not usually reduce it to one, so this is genuinely a ranking problem for nine tokens in ten, not a near-deterministic map.
- **Exceptions**: tokens with an apostrophe, digits or internal capitals are not expanded (REP-DIAC-009).
- **Predictability**: `rule`
- **Lexical input**: analyser as transducer
- **Interactions**: REP-DIAC-002 (class restriction); REP-DIAC-004 (ranking); REP-ENC-011 uses the class B restriction.
- **Sources**: [NS15 §4.2] (special analyser version mapping unaccented forms to accented candidates with morpheme boundaries and tags); [LP-HU], [magyarispell hu.aff MAP] (same vowel families used as suggestion classes; study only); [WC2]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens` (candidate sets for Table T-DIAC-2 words); ambiguity profile measured on 565,493 aligned tokens of 7,642 accent-stripped pairs, candidates being the accent-only respellings attested in Webcorpus 2 at freq >= 2.
- **Confidence**: `consensus` (expansion is exhaustive by construction; composition is a design choice).

### REP-DIAC-004 Ranking candidates: analysis, frequency and context
- **Statement**: Candidates are ranked by a context model over the sentence combined with per-candidate priors from word-form frequency or, for unseen forms, from lemma frequency, inflection-pattern frequency and penalties for productive compounding and derivation; the best sequence is chosen jointly for the sentence, not word by word.
- **Conditions**: candidate sets from REP-DIAC-003 for every token of the segment.
- **Output**:
  - Prior for a seen form: log frequency of the accented form in the frequency list (damaged spellings excluded).
  - Prior for an unseen form: `score = -lc * #CMP - ld * #DER + log10 LEM + li * log10 INF`, with `#CMP` productive compound boundaries, `#DER` productive derivational suffixes, `LEM` lemma frequency, `INF` frequency of the inflectional suffix sequence; weights tuned on a development set.
  - Context: n-gram or neural language model over accented text; decode the sentence lattice (monotone, one output per input token) with Viterbi or beam search.
  - Confidence per token: posterior margin between best and second candidate; margins below the policy threshold are reported (REP-DIAC-008).
- **Examples**: `meg kell csinalni` -> `meg kell csinálni` (`meg` 29 vs `még` 27: context decides for the particle before `kell`); `meg nem jott` -> `még nem jött`.
- **Exceptions**: equal scores between free variants (REP-DIAC-010): keep the variant that is more frequent in the same document.
- **Predictability**: `tendency`
- **Lexical input**: analyser, frequency list, language model
- **Interactions**: REP-DIAC-005 .. 007 describe the hard cases; REP-QUAL-007 uses the accepted share.
- **Sources**: [ATT evidence/calibration.md 2026-09-16]; [NS15 §4.2 formula (1), Table 3] (baseline most-frequent-form 97.71% accuracy on words with vowels, SMT 98.44%, with analyser ranking 99.06%; relative error reduction 39.74% at 437.6M training words; OOV 0.81%); [LY20 Table 2] (transformer NMT, Hungarian F1 99.31%, accuracy 99.61%; Table 3: 99.63% on all words after manual correction of the reference); [NS15 §2] (earlier dictionary methods about 98%, grapheme-context ML 95%).
- **Validation**: held-out accuracy on REP-NOISE-005 class A pairs; report on words with vowels separately. **The frequency prior alone reaches 97.91% top-1 on 565,493 held-out tokens** (99.75% on the 45,073 unambiguous tokens, 98.07% on the 517,746 ambiguous ones), which is the unigram floor of this rule minus its context term. 84.6% of the residual error is ambiguous tokens the prior ranks wrongly, and `meg` / `még` alone is 12.3% of all errors and 14.5% of that class - a single lexical pair no unigram prior can ever get right and a bigram context model gets right most of the time. That quantifies what the context term has to buy: at best it removes about 85% of the residual error, taking top-1 from 97.91% toward 99.7%. A second finding bounds the measurement itself: 250 tokens whose gold is `es` show that the *gold* text is itself partially unaccented in places, so measured precision is a slight under-estimate, by at most a few tenths of a point.
- **Confidence**: `consensus`; the prior's contribution is `measured`.

### REP-DIAC-005 Lexical ambiguity typology
- **Statement**: A closed but large set of Hungarian word forms collapse onto the same unaccented string; restoration must treat these as context decisions and never as dictionary lookups, and the typology below guides evaluation and noise generation.
- **Conditions**: the candidate set of REP-DIAC-003 has two or more accepted members with different lemmas or parts of speech.
- **Output** (Table T-DIAC-2 lists representative sets with frequency buckets). Types:
  - T1 vowel length on `a/e` (low vowels differ in quality): `kerek` / `kerék` / `kérek`; `szel` / `szél`; `fel` / `fél`; `hal` / `hál`; `hat` / `hát`; `akar` / `akár`; `faj` / `fáj`; `var` / `vár`; `el` / `él`; `teli` / `téli`; `level` / `levél`.
  - T2 length on `i/o/u` (same quality): `kor` / `kór`; `irt` / `írt`; `ut` / `út`; `szor` / `szór`.
  - T3 rounding/fronting (`o/ö`, `u/ü`) with or without length: `kor` / `kór` / `kör`; `sor` / `sör`; `tor` / `tör` / `tőr`; `bor` / `bőr`; `ot` / `öt` / `őt`; `szor` / `ször` / `szőr` / `szór`; `ol` / `ól` / `öl` / `ől`; `fut` / `fűt`.
  - T4 function words vs content words: `meg` / `még`; `ra` / `rá`; `ne` / `né`; `el` / `él`.
  - T5 names vs common words: `Marton` / `Márton`; `Bosz` / `Bösz` / `Bősz`; all-capital acronyms (`MIG` -> not `míg`).
- **Examples**: see Table T-DIAC-2.
- **Exceptions**: -
- **Predictability**: `lexical`
- **Lexical input**: analyser; frequency
- **Interactions**: REP-DIAC-004; REP-NOISE-005 oversamples these sets for evaluation; REP-OCR-007 (OCR produces the same real-word confusions).
- **Sources**: [NS15 Table 4] (original -> system output among mismatches: `még` -> `meg`, `termek` -> `termék`, `vágyók` -> `vagyok`, `Márton` -> `Marton`, `Bösz` -> `Bősz`, `MIG` -> `míg`); [ATT] (buckets in Table T-DIAC-2, probed 2026-09-15).
- **Validation**: `attestation`; `goldens`.
- **Confidence**: `consensus`.

### REP-DIAC-006 Inflectional ambiguity: possessive, definiteness, mood
- **Statement**: Most residual restoration errors are paradigm-internal: a nominal form with and without the possessive `-é-`/`-á-` lengthening, and a verb form in the indefinite and definite conjugation or indicative and subjunctive, differ only in one accent; these require syntactic context (object definiteness, possessor, article) rather than lexical context.
- **Conditions**: accepted candidates share the lemma and differ in morphological features.
- **Output** (decision cues, applied as features of the context model):
  - Possessive vs plain (51% of context errors in one study): `gyereket` (acc.) / `gyerekét` (poss.3sg acc.); `gyereken` / `gyerekén`; `gyereke` / `gyereké` (anaphoric possessive); cue: preceding definite article + genitive noun or possessor pronoun (`a fiú gyerekét`, `az ő gyerekét`) favours the possessive; indefinite article or numeral favours the plain form.
  - Definite vs indefinite verb (26% of context errors): `hajtottak` / `hajtották`; `hajtanak` / `hajtanák`; `hajtana` / `hajtaná`; `tudtak` / `tudták`; cue: a definite object in the clause (`a`, `az`, proper name, possessed noun, `azt`, `ezt`, subordinate `hogy`-clause object) favours the definite form (`SYN-AGR`, `VRB-PRES-002`).
  - Present definite 3pl vs subjunctive 1sg: `irjak` -> `írják` / `írjak`; cue: first person subject or `hogy`/`akarom, hogy` subjunctive trigger.
  - Case suffix vs pronoun: `ra` -> `rá` (pronominal adverb) vs `-ra` detached; cue: token boundary.
- **Examples**: `A tanar kereste a gyereket` -> `A tanár kereste a gyereket` (plain: no possessor); `Peter gyereket kereste` -> `Péter gyerekét kereste` (possessor + definite verb); `Hajtottak a teheneket` -> `Hajtották a teheneket`.
- **Exceptions**: without a clause-level cue the more frequent form is chosen and the token is marked low-confidence.
- **Predictability**: `tendency`
- **Lexical input**: analyser features
- **Interactions**: `SYN-AGR` (definiteness agreement), `NOM-POSS`, `VRB-PRES`, `VRB-PAST`, `VRB-COND`; REP-DIAC-004.
- **Sources**: [NS15 §6] (51.01% of mismatches are contextually inadequate forms; of these 51% possessive vs non-possessive, 26% verb definiteness; examples cited there).
- **Validation**: targeted `goldens`; evaluation slice.
- **Confidence**: `consensus`.

### REP-DIAC-007 Harmony-driven restoration of suffix vowels
- **Statement**: The accent of a vowel in a harmonic suffix is predictable from the stem's harmony class once the stem is identified, so suffix vowels are restored by rule after stem restoration: `-bol` is `-ból` after a back stem and `-ből` after a front stem.
- **Conditions**: analyser segmentation of a candidate into stem + suffixes; the suffix is one of the harmonic suffixes with an accented variant.
- **Output**:

| Unaccented suffix | Back stem | Front stem | Rounded front stem (where distinct) |
|---|---|---|---|
| `-bol` | `-ból` | `-ből` | `-ből` |
| `-rol` | `-ról` | `-ről` | `-ről` |
| `-tol` | `-tól` | `-től` | `-től` |
| `-nal` / `-nel` | `-nál` | `-nél` | `-nél` |
| `-ert` | `-ért` | `-ért` | `-ért` |
| `-ig`, `-kor` | no accent | no accent | no accent |
| `-ul` (also the stripped `-ül`) | `-ul` | `-ül` | `-ül` |
| `-hoz` (also the stripped `-höz`), `-hez` | `-hoz` | `-hez` | `-höz` |
| `-on` (also the stripped `-ön`), `-en` | `-on` | `-en` | `-ön` |
| `-val` / `-vel` | no accent | no accent | no accent |
| `-va` / `-ve`: translative `-vá` / `-vé` or adverbial participle `-va` / `-ve` (analysis decides) | `-vá` or `-va` | `-vé` or `-ve` | `-vé` or `-ve` |
| `-sag` / `-seg` | `-ság` | `-ség` | `-ség` |
| `-ek` for associative `-ék` | `-ék` | `-ék` | `-ék` |
| possessive `-ja` / `-je` + case | `-ját`, `-jában` | `-jét`, `-jében` | same |
| final `a/e` of stem before suffix | `á` (`hazát`) | `é` (`kertjét`) | - |
| participle `-o` (stripped `-ó` or `-ő`) | `-ó` | `-ő` | `-ő` |
| adjective `-u` | `-ú` | `-ű` | `-ű` |
| `-it` (verb) | `-ít` | `-ít` | `-ít` |
| comparative `-bb`, superlative `leg-` | no accent | no accent | no accent |

  Stem-final lengthening before suffixes (`kutyat` -> `kutyát`, `almat` -> `almát`) follows `ORT-SUFX` and `NOM-LENG`; suffix vowels always long or short follow `ORT-LEN-013`, `ORT-LEN-014`.
- **Examples**: `hazbol` -> `házból`; `kertbol` -> `kertből`; `fold` + `bol` -> `földből`; `tanarnal` -> `tanárnál`; `szepseg` -> `szépség`; `iro` -> `író`; `hosszu` -> `hosszú`; `gyonyoru` -> `gyönyörű`.
- **Exceptions**: vacillating stems (`PH-HARM` B+N classes: `hotel`, `fotel`) accept both harmony variants: keep the variant that matches the text's other forms of the same stem.
- **Predictability**: `rule` given the stem
- **Lexical input**: stem harmony class (`PH-HARM`)
- **Interactions**: `PH-HARM-003` .. `PH-HARM-009`, `ORT-LEN-013`, `ORT-LEN-014`, `NOM-CASE`; REP-DIAC-003 (reduces the lattice: suffix arcs are deterministic).
- **Sources**: [AkH12 §34, §35] via `ORT-LEN-013`, `ORT-LEN-014`; harmony rules per `PH-HARM`.
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### REP-DIAC-008 Partial restoration policy and abstention
- **Statement**: Restoration rewrites a token only when its confidence clears the policy threshold; words that already carry accents are not re-accented except for the letters of the detected loss class; low-confidence tokens are left unaccented and labelled rather than guessed.
- **Conditions**:
  - token confidence = posterior margin (REP-DIAC-004);
  - policies: `conservative` (margin **>= 0.95**, oracle-accepted candidates only; the previous 0.9 measures precision 0.99489 and misses the 0.995 target, while 0.95 measures 0.99651), `balanced` (>= 0.6, confirmed: measured precision 0.9847, the right shape for a level below `conservative`), `aggressive` (best candidate always, unverified allowed; measured 0.9791).
  - **margin** is `p(best)` over the normalised candidate set; the pairwise reading `p(best)/(p(best)+p(second))` gives the same operating points within 0.003 precision, so the choice of reading is not load-bearing.
  - mixed segments (some words accented, some not): restore only tokens with zero accented letters, plus class-restricted letters in accented tokens.
- **Output**: restored tokens; per-token label `restored`, `kept`, `ambiguous` (candidates listed in provenance).
- **Examples**: `Jo napot kivanok` -> `Jó napot kívánok` (all margins high); `kerek` alone in `a kerek` without further context -> `ambiguous` under `conservative` (`kerek`/`kerék`).
- **Exceptions**: training-pair generation (REP-NOISE-005) always knows the gold and does not use this policy; evaluation reports each policy.
- **Predictability**: `rule` (policy); `tendency` (thresholds)
- **Lexical input**: none beyond REP-DIAC-004
- **Interactions**: REP-DIAC-001 .. 007; REP-QUAL-010.
- **Sources**: [NS15 §6, Table 4] (14.70% of mismatches were correct output and a further 3.55% equivalent forms; 3.72% came from accents inserted into file names and URLs, motivating abstention and guards); [WC2]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: **measured on 565,493 tokens of 7,642 accent-stripped pairs** (frequency prior only, no context model, so these are lower bounds on the full rule): `conservative` 0.9965 precision / 0.8890 recall, `balanced` 0.9847 / 0.9665, `aggressive` 0.9791 / 0.9780. Five-fold cross-validation (`sha1(id) mod 5`) puts the fold-to-fold precision spread at +-0.00024 at margin 0.95, so the 0.0016 gap between 0.9 and 0.95 is about seven standard deviations wide and the threshold move is not noise. Per register the unthresholded precision spread is 3.0 points, from `legal` 0.9910 to `narrative` 0.9611; `narrative` is the hard register - literary text has the widest form vocabulary and the most free-variant spellings - and is the only register that still misses 0.995 at margin 0.9 (0.9856), which 0.95 fixes. A per-register threshold is **not** proposed: the policy levels are a user-facing contract.
- **Confidence**: `measured`.

### REP-DIAC-009 Guards: tokens that must not receive accents
- **Statement**: URLs, e-mail addresses, file names, code identifiers, hashtags and handles, all-capital acronyms, measurement symbols, foreign-language spans and proper names without a Hungarian accented spelling are excluded from restoration.
- **Conditions**: token or span matching any of:
  - URL (`scheme://`, `www.`), domain (`\w+\.(hu|com|org|eu|net)`), e-mail (`@`), path or file name (`/`, `\\`, `\.\w{2,4}$`);
  - code: backtick spans, camelCase or snake_case identifiers, tokens with digits;
  - hashtags `#...`, handles `@...` (accented hashtags exist; keep as written);
  - all-caps token of length <= 5 that is a known acronym or not accepted as a word (`MIG`, `USA`, `NATO`);
  - units and symbols (`km`, `kg`, `mol`);
  - foreign-language span (language ID per sentence, REP-QUAL-008) or foreign lexicon hit (`Marco`, `Peter` as English name in English context);
  - proper names: capitalised token not sentence-initial whose accented variants are all common words (`Kovacs` -> `Kovács` is allowed because `Kovács` is a name; `Bosz` -> keep unless `Bősz` is attested in the document).
- **Output**: token excluded (`kept`).
- **Examples**: `felhasznalo@profinter.hu` kept; `www.valamiceg.hu` kept; `USA-ban` kept (not `USÁ-ban`); `Marton` kept unless the document attests `Márton`.
- **Exceptions**: user-facing restoration of names may use a name lexicon with accents (`Janos` -> `János`) under `balanced` policy.
- **Predictability**: `rule`
- **Lexical input**: acronym list, name lexicon, foreign lexicon
- **Interactions**: REP-DIAC-003; `PUN-SPACE-003` (URL and e-mail shape).
- **Sources**: [NS15 Table 4] (`felhasználó@profinter.hu`, `www.valamicég.hu` in references; `USÁ-ban` corrected to `USA-ban`; names turned into common words 3.55%).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-DIAC-010 Free variants are not restoration errors
- **Statement**: Where the norm allows two forms differing in accent or vowel, the restorer keeps whichever the context or document prefers and evaluation counts both as correct.
- **Conditions**: candidate pair in the doublet list of `ORT-PRIN-009` and `ORT-SUFX`: `hova` ~ `hová`; `levő` ~ `lévő`; `mienk` ~ `miénk`; `tied` ~ `tiéd`; `fel` ~ `föl`; `dől` ~ `dűl`; `csoda` ~ `csuda`; `seprű` ~ `söprű`; `rá` ~ `reá`; `lábas` ~ `lábos`.
- **Output**: any listed variant; prefer the one already used in the same document.
- **Examples**: `hova` -> `hova` or `hová`; `levo` -> `levő` or `lévő`.
- **Exceptions**: pairs that are not doublets are real ambiguities (REP-DIAC-005); `fele` 'half of it' vs `felé` 'towards' is one, although [NS15 Table 4] counted `fele` -> `felé` among equivalent forms (colloquial `fele` for the postposition), so evaluation must decide per sense.
- **Predictability**: `lexical` (closed list in AkH12 §25)
- **Lexical input**: doublet list
- **Interactions**: `ORT-PRIN-009`; REP-SPELL (not errors).
- **Sources**: [AkH12 §25] via `ORT-PRIN-009`; [NS15 Table 4] (`lévő` -> `levő` counted as equivalent).
- **Validation**: evaluation treats doublets as equal.
- **Confidence**: `normative`.

### REP-DIAC-011 Umlaut-for-double-acute restoration
- **Statement**: In class C text (`ö ü` written for `ő ű`), only `ö` and `ü` are candidates and the decision is between two accepted forms in a minority of cases; unambiguous cases are repaired by oracle acceptance alone.
- **Conditions**: class C detected (REP-DIAC-002), or a single token rejected with `ö`/`ü` and accepted with `ő`/`ű`.
- **Output**: `ö` -> `ő`, `ü` -> `ű` where only the double acute reading is accepted; where both are accepted, rank by REP-DIAC-004.
- **Examples**: `elött` -> `előtt`; `kerülö` -> `kerülő`; `tüz` -> `tűz`; ambiguous: `tüzért` (artillery officer) / `tűzért`; `öt` / `őt`; `ör` (not a word) -> `őr`; `fül` kept (`fűl` rejected).
- **Exceptions**: German words and names (`Müller`, `König`) are kept.
- **Predictability**: `lexical`
- **Lexical input**: oracle, frequency
- **Interactions**: `WS-NORM-004` (row `ö` in place of `ő`); REP-SPELL-003 (a native short/long `ö/ő` error has the same surface in class-free text).
- **Sources**: `WS-NORM-004`; [ATT] (`öt` 20, `őt` 20; `tüz` 3 vs `tűz` 15).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-DIAC-012 All-capital and small-capital segments
- **Statement**: Accents are kept on capitals in Hungarian; an all-capital segment without accents is restored like class F text, but acronyms inside it are guarded and a heading's words are disambiguated with the lowercase text of the document when the heading repeats it.
- **Conditions**: segment of >= 2 all-capital alphabetic tokens with uppercase accented share < 10% of the document's lowercase accented share.
- **Output**: restored capitals (`A` -> `Á`, `O` -> `Ó Ö Ő`, etc.); acronyms guarded (REP-DIAC-009).
- **Examples**: `ALTALANOS SZERZODESI FELTETELEK` -> `ÁLTALÁNOS SZERZŐDÉSI FELTÉTELEK`; `MAV` in a heading -> `MÁV` only if the document attests `MÁV`.
- **Exceptions**: letterspaced capitals are joined first (REP-SPACE-001).
- **Predictability**: `tendency`
- **Lexical input**: oracle (on lowercased tokens), document vocabulary
- **Interactions**: `ORT-CAP-002`, `WS-CASE`; REP-DIAC-002 class F.
- **Sources**: [AkH12 §6] via `ORT-CAP-002`.
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

## SPELL: typical native spelling errors

Common structure of every detector in this area: (1) candidate generation by the error-specific edit (the inverse of the typical slip); (2) acceptance of the corrected form by the oracle; (3) comparison of written and corrected forms: if the written form is rejected and exactly one correction is accepted, repair; if both are accepted (real-word error), a context rule or model decides and the default is to keep and report; (4) never correct a proper name, foreign word, quotation of nonstandard speech, or text declared as dialect or literary deviation (`ORT-PRIN-013`).

### REP-SPELL-001 j versus ly
- **Statement**: `j` and `ly` spell the same sound and are confused in both directions; since no complete rule exists, the detector is lexical (swap candidates checked by the oracle), and the partial regularities of AkH12 §88 serve as priors for unknown words and for ranking.
- **Conditions**:
  - Candidates: every `j` -> `ly` and every `ly` -> `j` in the token (including `lly` <-> `jj`, `llj` <-> `jj`, `lyj` <-> `jj` at suffix boundaries: `folyjon`, `gallyal`), one or two substitutions per token.
  - Priors when the oracle cannot decide (unknown stems): word-initial `ly` only in `lyuk` and its derivatives; final `[j]` in monosyllables mostly `j` except a closed list (`boly`, `foly(ik)`, `hely`, `mély`, `moly`, `súly`, `süly`, `gally`) and pronouns (`ily`, `mily`, `oly`, `mely`, `ilyen`, `olyan`, `milyen`, `amely`); polysyllables ending `-aj`/`-ej` mostly `j` (exceptions `karvaly`, `pehely`, `tavaly`, `zsindely`), other polysyllables mostly `ly` (exceptions `lakáj`, `muszáj`, `papagáj`); `-lya`/`-lye` endings favour `ly` (exceptions `bója`, `cserje`, `héja` (bird), `perje`).
- **Output**: the unique accepted variant; for real-word pairs keep and report.
- **Examples**: `hüje` -> `hülye` (`hüje` -, `hülye` 12); `bojong` -> `bolyong`; `fojik` -> `folyik`; `gója` -> `gólya`; `ijen` -> `ilyen`; `ojan` -> `olyan`; `pehej` -> `pehely`; `muszály` -> `muszáj` (3 vs 13; hypercorrection); `foljon`-type suffix error: `folyon` (imperative of `folyik`) -> `folyjon`.
- **Exceptions**: real-word pairs: `hej` (interjection) / `hely`; `bojt` (tassel) / `bolyt` (acc. of `boly`); `-juk` suffix vs `lyuk`. Names keep archaic `ly`/`y` (`Kodály`, `Mihály`, `Gergely`, `Ipoly`).
- **Predictability**: `lexical`
- **Lexical input**: oracle; §88 prior classes
- **Interactions**: `ORT-JLY-001` .. `ORT-JLY-006`; `ORT-ASSIM-018` (j-assimilation not written); REP-NOISE-006 generates `ly` <-> `j`.
- **Sources**: [AkH12 §88] via `ORT-JLY`; [ATT] (buckets probed 2026-09-15); [PC-HU noise.ron] (`ly`/`j` slips in both directions).
- **Validation**: `oracle:analyser`; `attestation`.
- **Confidence**: `normative` (target spellings), `consensus` (detector).

### REP-SPELL-002 Vowel length errors: i/í, u/ú, ü/ű
- **Statement**: Length of `i u ü` is not reliably distinguished in pronunciation by many speakers, producing both shortening and lengthening errors; detection combines the lexicon with the codified length of suffixes and word-final positions.
- **Conditions**:
  - Candidates: toggle length of each `i/í`, `u/ú`, `ü/ű` (one or two toggles per token).
  - Rule-backed positions (no lexicon needed): suffixes always long: `-ít` (and `-dít`, `-sít`), adjective `-ú/-ű/-jú/-jű`, `-tyú/-tyű`, `-ból/-ből`, `-ról/-ről`, `-tól/-től`, postpositional `-ól/-ől`; always short: `-ul/-ül` (derivational and case), `-stul/-stül`, postpositions and adverbs in `-ul/-ül` (except `túl`), `-szor/-szer/-ször`, `-kor`, `-ig`, word-final `-i` of suffixed forms; adjectives end in long `-ú/-ű`; monosyllabic nouns end in long `ú ű`; word-final `i` mostly short.
  - Stem alternation: `víz` / `vizet`, `híd` / `hidat`, `út` / `utat`, `tűz` / `tüzet` (kéz-type shortening before the same suffixes as `kéz`): an unalternated form in shortening position (`vízet`) or an alternated form elsewhere (`vizben`) is an error.
- **Output**: the unique accepted variant.
- **Examples**: `mindíg` -> `mindig` (8 vs 23); `ujság` -> `újság`; `tanitás` -> `tanítás`; `kivül` -> `kívül`; `hosszu` -> `hosszú`; `gyógyúl` -> `gyógyul`; `végűl` -> `végül`; `vízet` -> `vizet`; `hidon` -> `hídon`; `kapú` -> `kapu`; `budapestí` -> `budapesti`.
- **Exceptions**: short final `u/ü` nouns are a closed list (`adu`, `alku`, `áru`, `bábu`, `daru`, `falu`, `kapu`, `menü`, `eskü` ...) and hypocoristics (`anyu`, `Icu`); long final `í` in `sí`, `rí`, `zrí`. Doublets (`REP-DIAC-010`) are not errors.
- **Predictability**: `rule` (suffix and final positions), `lexical` (stems)
- **Lexical input**: oracle; closed lists from `ORT-LEN-004`
- **Interactions**: `ORT-LEN-002` .. `ORT-LEN-009`, `ORT-LEN-013`, `ORT-LEN-014`; distinguish from DIAC class E (REP-DIAC-002): a systematic `í` absence is a keyboard or class issue, an isolated one is spelling.
- **Sources**: [AkH12 §20-§24, §27, §34, §35] via `ORT-LEN`; [ATT].
- **Validation**: `goldens`; `oracle:analyser`.
- **Confidence**: `normative`.

### REP-SPELL-003 Vowel length errors: o/ó, ö/ő
- **Statement**: `ö/ő` and `o/ó` length errors are frequent in stems (`rendör`, `öszinte`) and in suffixes (`-ből` written `-böl`); detection uses the same toggle-and-accept scheme, with suffix length fixed by rule and nonstandard stem variants treated as errors.
- **Conditions**:
  - Candidates: toggle `ö/ő`, `o/ó` per token.
  - Rule-backed: `-ból/-ből`, `-ról/-ről`, `-tól/-től` long; participle `-ó/-ő` long; `-szor/-szer/-ször`, `-kor`, `-hoz/-hez/-höz` (short `o ö`); `-nál/-nél`.
  - Dialectal or archaic stem variants outside the standard (AkH11 §17 names `késér`, `ződ`, `nígy` as non-standard pronunciations; spelling follows the standard form): `ződ` -> `zöld`, `szöllő` -> `szőlő`.
- **Output**: accepted variant.
- **Examples**: `rendör` -> `rendőr`; `öszinte` -> `őszinte`; `elöre` -> `előre`; `kertböl` -> `kertből`; `szöllő` -> `szőlő`; `ződ` -> `zöld` (unless the text is marked dialect); `ütö` -> `ütő`.
- **Exceptions**: real-word pairs `öt`/`őt`, `elöl`/`elől` (REP-SPELL-014), `tör`/`tőr`, `kor`/`kór`, `bor`/`bór`; decide by context or keep.
- **Predictability**: `rule` (suffixes), `lexical` (stems)
- **Lexical input**: oracle
- **Interactions**: `ORT-LEN-013`, `ORT-LEN-014`, `ORT-PRIN-009`; REP-DIAC-011 (the same surface as class C loss).
- **Sources**: [AkH12 §34, §35] via `ORT-LEN`; [AkH11 §17] (standard vs dialectal pronunciation forms); `WS-NORM-004`; `szöllő` as a nonstandard form is cited from [HUWIKI-HELYESIRAS] and not verified in the AkH text.
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### REP-SPELL-004 Consonant length errors
- **Statement**: Doubled and single consonant letters are confused in stems (`bizotság`), in suffixes whose consonant doubles after vowels (past `-tt`, comparative `-bb`, imperative definite `-dd`), and at stem-suffix junctions (`hallottam`); detection is toggle-and-accept with suffix doubling rules as hard evidence.
- **Conditions**:
  - Candidates: single -> double and double -> single for each consonant letter (digraphs by REP-SPELL-005), at most two edits per token.
  - Rule-backed: accusative `-t` always single (`ORT-LEN-015`); `-képp(en)` double; comparative always `-bb` (`szebb`, `nagyobb`); past tense and past participle `-tt` after a linking vowel or a vowel-final stem (`kötött`, `hallott`), `-t` directly after the stem where `VRB-PAST` selects it (`várt`); locative `-tt` after a linking vowel (`Pécsett`, `Győrött`), `-t` directly after the stem (`Kaposvárt`); the short imperative definite 2sg `-d` doubles only after stem-final `d` (`mondd`, `küldd`, `add`) and in the irregular `vidd`, `hidd`, `tedd`, `edd`, `idd`.
  - Real-word guard: `kellet` (form of `kelleti`/noun contexts) vs `kellett` ('had to'): correct only when an infinitive or clause-level modal context is adjacent (`kellet csinálni` -> `kellett csinálni`).
- **Output**: accepted variant; context rule for guarded real-word pairs.
- **Examples**: `hallotam` -> `hallottam`; `bizotság` -> `bizottság`; `azonal` -> `azonnal`; `lessz` -> `lesz` (4 vs 24); `kissebb` -> `kisebb` (3 vs 20); `végkép` -> `végképp` (4 vs 11); `mond el` (imperative) -> `mondd el`; `kellet menni` -> `kellett menni`; `hellyett` -> `helyett`.
- **Exceptions**: real-word pairs: `hit`/`hitt`, `nézet`/`nézett`, `vitel`/`vittel`, `tol`/`toll`, `mond`/`mondd` (indicative vs imperative), `kellet`/`kellett`: context or keep.
- **Predictability**: `rule` (suffixes), `lexical` (stems)
- **Lexical input**: oracle; context features (infinitive, imperative clause)
- **Interactions**: `ORT-LEN-015` .. `ORT-LEN-023`; REP-NOISE-006 (`tt` -> `t`, `ll` -> `l` slips).
- **Sources**: [AkH12 §36-§38, §62] via `ORT-LEN`; [LP-HU] (`kellet` near infinitive; study only); [magyarispell hu.aff REP] (`kissebb`, `végkép`; study only); [ATT].
- **Validation**: `goldens`; `oracle:analyser`.
- **Confidence**: `normative`.

### REP-SPELL-005 Doubled multi-character letter errors
- **Statement**: A long multi-character consonant is written with the first letter doubled (`ssz`, `ggy`, `nny`, `lly`, `tty`, `ccs`, `zzs`, `ddz`, `ddzs`); errors are the single form (`menyi` for `mennyi`), the fully written form (`szsz`), and the simplified form at compound boundaries where the full form is required.
- **Conditions**:
  - Candidates: `X` -> `xX` (simplified doubling of digraph `X` = `x` + `y`), `xX` -> `X`, `XX` -> `xX` (full to simplified), and `xX` -> `XX` at an analyser-marked compound boundary.
  - Obsolete full doubling (`hoszszú`) in texts dated before the 20th century is REP-ARCH-004, not an error.
- **Output**: accepted variant with the boundary-aware choice of `ORT-ASSIM-016`.
- **Examples**: `menyi` -> `mennyi` (0 vs 17); `hoszú` -> `hosszú`; `visza` -> `vissza`; `aszony` -> `asszony`; `hoszszú` (modern text) -> `hosszú`; `kulccsomó` -> `kulcscsomó` (compound); `jeggyűrű` -> `jegygyűrű`; `ne játszunk` -> `ne játsszunk`.
- **Exceptions**: indicative/imperative real-word pairs (`játszunk` / `játsszunk`, `mossuk` both): decide by clause type (negation `ne`, `hogy`-subjunctive, sentence-initial imperative) else keep.
- **Predictability**: `rule` given morphology
- **Lexical input**: oracle with compound boundaries
- **Interactions**: `ORT-ASSIM-016`, `WS-DIG-002`, `WS-DIG-004`, `WS-DIG-010`; REP-SPACE-004.
- **Sources**: [AkH12 §92] via `ORT-ASSIM-016`; [LP-HU] (`ne játszunk` -> `ne játsszunk`; study only); [ATT].
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### REP-SPELL-006 Assimilation errors of -val/-vel and -vá/-vé
- **Statement**: The `v` of `-val/-vel` and `-vá/-vé` assimilates to a stem-final consonant and is written as a doubled consonant; typical errors are unassimilated `v` (`emberval`), single consonant (`ezel`, `Kossuthal`), assimilation to the written instead of the pronounced consonant for foreign and archaic spellings, and missing hyphen where three identical letters would arise with names.
- **Conditions**:
  - token ending in a `-Vl` or `-V` form after a consonant-final stem (analyser segmentation), compared with the generated correct form of `ORT-ASSIM-001` .. `ORT-ASSIM-007`;
  - names with archaic or foreign final letters: assimilate to the pronounced sound (`Kossuthtal`, `Móriczcal`, `Babitscsal`, `Bachhal`, `Balzackal`), hyphen after doubled final letters (`Kiss-sel`, `Papp-pal`, `Széll-lel`, `Bernadett-tel`), hyphen after silent final letters with assimilation to pronunciation (`Molière-rel`);
  - abbreviations, acronyms and digits: hyphen + pronounced assimilation (`AIDS-szel`, `4-gyel`, `15%-kal`, `Bp.-tel`).
- **Output**: generated correct form.
- **Examples**: `ezel` -> `ezzel` (4 vs 22); `Kossuthal` -> `Kossuthtal`; `Barossal` -> `Baross-sal`; `Mannal` -> `Mann-nal`; `Grimmel` -> `Grimm-mel`; `AIDS-cel` -> `AIDS-szel`; `%-al` -> `%-kal`; `Bachal` -> `Bachhal`; `pech-el` -> `pechhel`.
- **Exceptions**: doublets `ezzel`~`evvel`, `azzal`~`avval`; h-final words with variable pronunciation (`méhvel`~`méhhel`); AkH11-era `Bernadettel` in texts before 2016 (REP-SPELL-010).
- **Predictability**: `rule` given pronunciation of the final sound
- **Lexical input**: pronunciation of foreign and archaic finals (`ORT-FOR`, `ORT-NAME`)
- **Interactions**: `ORT-ASSIM-001` .. `ORT-ASSIM-009`, `ORT-HYPH-002`, `ORT-ABBR-009`, `ORT-NUM`.
- **Sources**: [AkH12 §41, §82, §93, §160, §216-§217, §282] via `ORT-ASSIM`, `ORT-FOR`; [magyarispell hu.aff REP] (`Barossal`, `Mannal`, `AIDS-cel` suggestion classes; study only).
- **Validation**: `goldens` generated by the `ORT` generator.
- **Confidence**: `normative`.

### REP-SPELL-007 Imperative and definite j-assimilation errors
- **Statement**: Imperative `-j` and the `j` of definite suffixes assimilate after `s sz z dz` and combine with `t`-final stems into `ss`, `ts`, `ssz`; errors include writing the pronunciation-agnostic `j` (`higyj`), omitting the doubling (`játszuk` for imperative `játsszuk`), and confusing `ts` and `ss` classes.
- **Conditions**: verb forms whose analysis is imperative (or ambiguous with indicative), compared with the forms generated by `ORT-ASSIM-011` .. `ORT-ASSIM-021` and `VRB-IMP`.
- **Output**: generated form.
- **Examples**: `higyj` -> `higgy`; `higyjél` -> `higgyél`; `tanísd` -> `tanítsd`; `fessd` -> `fesd`; `mosjon` -> `mosson`; `hozjon` -> `hozzon`; `látsunk` -> `lássunk`.
- **Exceptions**: `metsz`, `látszik`, `tetszik` allow `tssz` ~ `ss` (`látsszék` ~ `lássék`); indicative forms that coincide with the correct imperative of another verb. **`látsszék` is a standard minority variant (349 : `lássék` 1,572, i.e. 18%) and is not repaired**; `látszódjék` (1,579) is the neutral alternative and is as frequent as `lássék`. The repair rule must not "fix" `látsszék` [WC2].
- **Predictability**: `rule`
- **Lexical input**: verb stem class
- **Interactions**: `ORT-ASSIM-011` .. `ORT-ASSIM-021`, `VRB-IMP`; REP-SPELL-008 (suksükölés is the semantic confusion of the same forms).
- **Sources**: [AkH12 §42, §43, §53-§58, §76-§81] via `ORT-ASSIM`; [WC2]; [ATT evidence/corpus-variants.md 2026-09-16].
- **Validation**: `goldens` from the verb generator; `attestation` (`lássék` 1,572 : `látsszék` 349 : `látszódjék` 1,579, register split: `lássék` technical 606 / literary 227 / web 1,216, `látsszék` technical 4 / literary 10 / wiki 4).
- **Confidence**: `normative` for the assimilation rules; `measured` for the `látsszék` ~ `lássék` doublet - both are current, `lássék` preferred at 82%, and `látsszék` is a real minority form rather than a spelling error (`OQ-10-015`). The AkH12 §78 normativity check the question also asks for is still outstanding: the corpus decides usage, not normativity.

### REP-SPELL-008 Suksükölés, nákolás and ikes conjugation errors (usage)
- **Statement**: Three nonstandard usage patterns surface as spelling-like errors: indicative forms of `t`-final verbs replaced by imperative forms (suksükölés: `tanítsuk` for `tanítjuk`), 1sg conditional indefinite `-nék` replaced by `-nák` (nákolás: `innák` for `innék`), and `-k` for `-m` in 1sg present of `-ik` verbs (`eszek` for `eszem`); detection is syntactic and belongs to the syntax lint (out of scope); this rule specifies the repair interface.
- **Conditions**:
  - suksükölés: imperative form of a verb with stem-final `t` (`-ít`, `-t`, `-szt`, `-st` stems) in a clause without imperative/subjunctive licensing (no `hogy` with volitional matrix, no `ne`, not sentence-initial command, not `-e` question of permission);
  - nákolás: `-nák/-nék`-final conditional with an explicit 1sg subject (`én`) or 1sg context and no 3pl subject;
  - ikes: 1sg `-k` form of an `-ik` verb in formal register (standard `-m`; `-k` is accepted informally for many verbs).
- **Output**: indicative form (`tanítjuk`), `-nék` form (`innék`), `-m` form (`eszem`) in formal register; otherwise report only.
- **Examples**: `Mi ezt így tanítsuk.` (indicative meaning) -> `Mi ezt így tanítjuk.`; `Én is innák egy kávét.` -> `Én is innék egy kávét.`; `eszek` -> `eszem` (formal).
- **Exceptions**: genuine imperatives (`Tanítsuk meg neki!`), 3pl definite conditional (`ők innák meg`), informal register for `-ik` 1sg.
- **Predictability**: `tendency`
- **Lexical input**: verb class (`-ik`, t-final)
- **Interactions**: `SYN-DEF`, `VRB-IK`, `VRB-IMP`, `VRB-COND`.
- **Sources**: [LP-HU] (conditional and imperative checks; study only); verb forms per `VRB` chapter; nákolás definition (1sg indefinite conditional `-nák` for `-nék`, `innák`, `olvasnák`) per Hungarian language-advice sources (e-nyelv.hu, Vajdaság MA "A nákolás"), secondary.
- **Validation**: `goldens`; `oracle:lightproof` for comparison.
- **Confidence**: `consensus` (standard forms normative; register judgement contested for ikes forms).

### REP-SPELL-009 Illative for inessive and related case confusions (usage)
- **Statement**: The illative `-ba/-be` used for the inessive `-ban/-ben` (`a házba vagyok`), and less often the reverse, is a frequent nonstandard pattern; detection requires the verb's locative frame and belongs to the syntax lint (out of scope).
- **Conditions**: NP in `-ba/-be` governed by a static verb (`van`, `lakik`, `dolgozik`, `ül`, `marad`, `található`, `él`) without motion; or `-ban/-ben` governed by a motion verb (`megy`, `érkezik`, `tesz`, `beköltözik`).
- **Output**: `-ban/-ben` (static) or `-ba/-be` (motion) with harmony preserved.
- **Examples**: `a házba vagyok` -> `a házban vagyok`; `Budapestbe lakik` -> `Budapesten lakik` (locative of the place name per `NOM-CASE`).
- **Exceptions**: verbs accepting both frames with meaning difference (`beleszeret valakibe`; `bízik valakiben`); fixed expressions (`betegségbe esik`).
- **Predictability**: `tendency`
- **Lexical input**: verb government frames (`SYN-GOV`)
- **Interactions**: REP-NOISE-006 (`ban` -> `ba` slip).
- **Sources**: [PC-HU noise.ron] (`ban` -> `ba`, `ben` -> `be` ending slips); case functions per `NOM-CASE`.
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-SPELL-010 Superseded AkH11 spellings
- **Statement**: Spellings correct under AkH11 (1984) but changed in AkH12 (2015, AkH11 forms accepted until September 2016) are not errors in texts dated before the change and are normalised only on request.
- **Conditions**: document date or source date < 2016-09; forms listed in `ORT-TOG-047` and the AkH12 change list: `ésszerű` (now `észszerű`), given names with three identical consonants simplified (`Bernadettel`, now `Bernadett-tel`; `Mariannal`, now `Mariann-nal`), `-i` counted in 6:3 syllable count (`kerékpár-javítási` vs AkH12 `kerékpárjavítási`), dictionary-level together/apart changes.
- **Output**: keep for older texts; normalise to AkH12 in `modernise` mode.
- **Examples**: `ésszerű` (1998 text) kept; `ésszerű` (2024 text) -> `észszerű`; `Bernadettel` (2024) -> `Bernadett-tel`.
- **Exceptions**: undated text: report as `superseded` rather than repair.
- **Predictability**: `lexical` (closed change list)
- **Lexical input**: AkH11 to AkH12 change list; document date
- **Interactions**: `ORT-TOG-047`, `ORT-ASSIM-017`, `ORT-63-003`, REP-SPELL-006.
- **Sources**: [AkH12 §93, §139] via `ORT-ASSIM-017`, `ORT-63`; [AkH11 §94, §138]; [KOROMPAY18 p. 98] (summary of AkH12 changes: `Bernadett-tel`, `észszerű`, `méhvel`~`méhhel`).
- **Validation**: `goldens` with dated inputs.
- **Confidence**: `normative`.

### REP-SPELL-011 Writing together or apart
- **Statement**: Compounds written apart (`hosszú távú` is correct but `nem rég` is not), phrases written together (`nagyfokú` for `nagy fokú`), and missing 6:3 hyphens are detected by comparing the written segmentation with the segmentation generated by the `ORT-TOG` and `ORT-63` rules for the analysed structure.
- **Conditions**:
  - two adjacent tokens whose joined form is an accepted lexicalised compound with meaning change or an unmarked-relation compound (`ORT-TOG-002`, `ORT-TOG-019`, `ORT-TOG-020`), or a prefix-verb adjacency (`ORT-TOG-026`);
  - one token accepted only as a productive compound whose structure requires writing apart (marked relation `ORT-TOG-016`, compound member + attribute `ORT-TOG-021`, numeral + derived adjective `ORT-TOG-023`);
  - compound of 3+ members and > 6 syllables without hyphen, or <= 6 syllables with hyphen (`ORT-63-002`).
- **Output**: generated segmentation; ambiguous cases (literal vs lexicalised meaning, `ORT-TOG-002`: `gyors írás` / `gyorsírás`) are reported, not repaired.
- **Examples**: `nem rég` -> `nemrég`; `minden féle` -> `mindenféle`; `nagyfokú` -> `nagy fokú`; `hosszútávú` -> `hosszú távú`; `rámváró` -> `rám váró`; `kerékpárjavító műhely` kept.
- **Exceptions**: meaning-dependent pairs (`nagy bácsi` vs `nagybácsi`, `jó indulat` vs `jóindulat`): report only.
- **Predictability**: `rule` given structure; `lexical` for meaning-change compounds
- **Lexical input**: compound lexicon, analyser structure
- **Interactions**: `ORT-TOG-001` .. `ORT-TOG-047`, `ORT-63-001` .. `ORT-63-009`; REP-SPACE-003, REP-SPACE-009 (damage-level joins must not pre-empt this rule).
- **Sources**: [AkH12 §94-§141] via `ORT-TOG`, `ORT-63`; [magyarispell hu.aff REP] (suggestion classes `nagyfokú` -> `nagy fokú`, `rámváró` -> `rám váró`; study only).
- **Validation**: `goldens` from `ORT-TOG` examples.
- **Confidence**: `normative` (rules), `contested` (meaning-change judgements).

### REP-SPELL-012 Hyphen errors with suffixed abbreviations, acronyms, numerals and foreign names
- **Statement**: Suffixes attach with a hyphen to abbreviations, symbols, letter-by-letter acronyms and digits, but directly to word-like blends and to foreign names whose final letter is pronounced; the typical errors are a missing hyphen (`MÁVnál`), a superfluous hyphen (`Goethe-vel`), and a suffix allomorph chosen by spelling instead of pronunciation (`ENSZ-el` for `ENSZ-szel`).
- **Conditions**:
  - token = acronym/abbreviation/digits/symbol + suffix without hyphen: `[A-ZÁÉÍÓÖŐÚÜŰ]{2,}[a-záéíóöőúüű]+`, `\d+[a-záéíóöőúüű]+`, `%[a-záéíóöőúüű]+` (explicit letter sets: a range such as `á-ű` spans U+00E1..U+0171 and includes capitals and foreign letters) where the prefix part is a known acronym, digit string or symbol and the rest is a suffix sequence;
  - token = common word or Hungarian name + hyphen + suffix (`ház-ban`, `Péter-nek`): superfluous hyphen;
  - foreign name + hyphen + suffix where the final letter is pronounced (`Goethe-vel`, correct `Goethével`), or without hyphen where the final letter is silent (`Glasgowban`, correct `Glasgow-ban`);
  - suffix allomorph must follow the spoken form of the acronym or number (`MÁV-ot`, `ENSZ-szel`, `5-tel`, `3-mal`, `15%-kal`).
- **Output**: form generated by `ORT-ABBR-009`, `ORT-NUM`, `ORT-FOR`.
- **Examples**: `MÁVnál` -> `MÁV-nál`; `1848ban` -> `1848-ban`; `%-al` -> `%-kal`; `ház-ban` -> `házban`; `Goethe-vel` -> `Goethével`; `Glasgowban` -> `Glasgow-ban`; `Shakespeare-el` -> `Shakespeare-rel`; `ENSZ-el` -> `ENSZ-szel`.
- **Exceptions**: word-like blends take no hyphen (`Orexszel`, `áfát`); lowercase acronyms (`tv-ben`, `tbc-s`) still take the hyphen.
- **Predictability**: `rule` given pronunciation
- **Lexical input**: acronym readings, foreign name pronunciations
- **Interactions**: `ORT-ABBR-009`, `ORT-NUM`, `ORT-FOR`, `ORT-HYPH-015`; REP-SPACE-011.
- **Sources**: [AkH12 §215-§217, §265 d), §282, §288] via `ORT-ABBR-009`, `ORT-FOR`; [LP-HU] (numeral + suffix tables, `%-al` -> `%-kal`; study only).
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### REP-SPELL-013 Capitalisation errors
- **Statement**: Capitalised derived adjectives of place names (`Budapesti`), nationality and language words (`Magyar`), month and day names, and holidays are common capitalisation errors; the detector lowercases them except at sentence or heading start and inside institution names and titles, where capitals are correct.
- **Conditions**:
  - token inside a sentence (not first word, not after a colon introducing a sentence per `ORT-CAP-008`, not in a heading/sign/caption) that is: an `-i`/`-s` derivative of a one-word or solid geographic name (`Budapesti`, `Magyarországi`), a nationality or language (`Magyar`, `Angol`), a month (`Január`), a day (`Hétfő`), a holiday (`Karácsony`), a period or event name lowercase by `ORT-CAP-005`;
  - guard: the token starts a multiword institution, organisation, award, periodical or work title (`Budapesti Műszaki és Gazdaságtudományi Egyetem`, `Soproni Kisegítő Iskola`, `Magyar Nemzeti Bank`, `Magyar Közlöny`), detected by a following capitalised type word or a name lexicon (`ORT-CAP-020` .. `ORT-CAP-022`).
- **Output**: lowercase first letter (digraph-aware: `Gy` -> `gy`).
- **Examples**: `a Budapesti lakosok` -> `a budapesti lakosok`; `beszél Magyarul` -> `beszél magyarul`; `jövő Hétfőn` -> `jövő hétfőn`; `a Budapesti Műszaki Egyetem` kept.
- **Exceptions**: hyphenated name + noun adjectives keep the capital (`Kossuth-díjas`, `Árpád-kori`); multiword names keep capitals before `-i` with hyphen (`New York-i`).
- **Predictability**: `rule` with lexical guards
- **Lexical input**: institution and title lexicon; place-name lexicon
- **Interactions**: `ORT-CAP-005` .. `ORT-CAP-008`, `ORT-CAP-017`, `ORT-CAP-020`; sentence segmentation (out of scope) for sentence starts.
- **Sources**: [AkH12 §144, §173-§174, §167, §188 b)] via `ORT-CAP`; [LP-HU] (geographic name case checks; study only).
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### REP-SPELL-014 Confusable real-word pairs
- **Statement**: A small set of notoriously confused spellings, mostly pairs of existing words (two rows, `előről` and `eszencia`, are non-standard spellings kept here because they are usually taken for real words); each pair has a context cue and a default, and without a cue the text is kept.
- **Conditions**: the written word is one member of a listed pair and the context cue of the table is present.
- **Output**: the intended member per cue:

| Written | Intended | Meaning difference | Cue for repair |
|---|---|---|---|
| `egyenlőre` | `egyelőre` | 'to equal (sublative)' vs 'for the time being' | sentence adverb position, no comparison object: repair (`egyenlőre` 8, `egyelőre` 18) |
| `elöl` / `elől` | each other | 'at the front' (location) vs 'from before' (source, with a noun: `a ház elől`) | postposition after a noun or pronoun with source verb (`elmenekül`, `elrejt`): `elől`; standalone locative: `elöl` |
| `előről` | `elölről` | standard spelling of both 'from the front' and 'from the beginning' is `elölről` (`elölről kezdi`, `elölről nézve`); `előről` is a pronunciation spelling (`előről` 1 vs `elölről` 9) | always repair |
| `helység` / `helyiség` | each other | 'settlement' vs 'room' | room context (`iroda`, `mosdó`, `bérel`) -> `helyiség`; administrative context -> `helység` |
| `had` / `hadd` | `hadd` | 'army' vs 'let (me)' | before a verb in the subjunctive/1sg (`had menjek`) -> `hadd` |
| `eltusol` / `eltussol` | each other | 'showers away' vs 'hushes up' | object `ügy`, `botrány` -> `eltussol` |
| `eszencia` | `esszencia` | only `esszencia` is standard, in both senses 'extract' and 'essence' (`eszencia` 2 vs `esszencia` 5) | always repair |
| `hivatott` / `hívatott` | each other | 'called to (a vocation)' vs 'had someone called' | infinitive complement or `arra` -> `hivatott` |
| `fáradság` / `fáradtság` | each other | 'effort, trouble' vs 'tiredness' | `veszi a fáradságot` -> `fáradság`; state of body -> `fáradtság` |
| `kellet` / `kellett` | `kellett` | see REP-SPELL-004 | infinitive adjacent |
| `túros` / `túrós` | each other | `túrós` 'with curd'; `túros` 'with saddle sores' | idiom `közös lónak túros a háta` keeps `túros`; food context -> `túrós` |
| `tovább` + verb | solid or apart | prefix vs adverb (`továbbtanul` 'continues studies' vs `tovább tanul` 'studies longer') | report only (`ORT-TOG-030`) |

- **Examples**: see table.
- **Exceptions**: without a cue, keep and report.
- **Predictability**: `lexical` (closed list)
- **Lexical input**: pair list with cues
- **Interactions**: REP-SPELL-004, `ORT-TOG-030`, `ORT-PRIN`.
- **Sources**: [magyarispell tipikus/] (confusable pair files: `egyelőre`, `elől`, `eltussol`, `esszencia`, `hadd`, `hivatott`, `tovább`, `megfedd`, `túros`; study only, pairs restated); [ATT].
- **Validation**: `goldens`; manual evaluation slice.
- **Confidence**: `consensus`.

### REP-SPELL-015 Common misspelled word list: categories and data contract
- **Statement**: Beyond rule-derived detectors, a curated list maps frequent misspellings to their correct forms; each entry carries its error category so that the list can be audited against the rules of this area and regenerated when the oracle changes.
- **Conditions**: entry admitted only if the misspelling is rejected by the oracle, the correction is accepted, the misspelling is attested in `[ATT]` or an error corpus, and no rule of REP-SPELL-001 .. 013 already derives it (otherwise the rule, not the list, is the source).
- **Output**: record `{wrong, right, category, source, first_seen}` with categories: `JLY`, `LEN_V` (vowel length), `LEN_C` (consonant length), `DIG` (doubled digraph), `ASSIM`, `IMP`, `TOG`, `HYPH`, `CAP`, `LOAN` (loanword spelling: `komunikáció` -> `kommunikáció`), `HYPER` (hypercorrection: `muszály`), `ARCH` (obsolete), `CONF` (REP-SPELL-014 pairs).
- **Examples**: `{mindíg, mindig, LEN_V}`; `{muszály, muszáj, HYPER}`; `{kissebb, kisebb, LEN_C}`; `{hellyett, helyett, LEN_C}`; `{végkép, végképp, LEN_C}`.
- **Exceptions**: entries that are attested dialect or literary forms are tagged `deviation` and never auto-repaired.
- **Predictability**: `lexical`
- **Lexical input**: the list itself (size to be stated at build)
- **Interactions**: all REP-SPELL rules; REP-NOISE-006 samples list entries.
- **Sources**: [ATT] (attestation of `mindíg` 8, `muszály` 3, `kissebb` 3, `végkép` 4, `ezel` 4, `lessz` 4, `ujság` 5); [NS15 Table 4] (17.91% of restoration mismatches were uncorrected errors already in the web text, including 10.81% spelling errors other than punctuation and hyphenation).
- **Validation**: list regenerated and diffed against the oracle at build.
- **Confidence**: `consensus`.

### REP-SPELL-016 Three identical consonant letters
- **Statement**: Suffixation that would create three identical consonant letters writes two for common words, but proper names and compound boundaries keep all three sounds with a hyphen; errors are triple letters (`sakkkör`), simplification where a hyphen is required (`Széllel`, `sakkör`), and hyphenation of common words (`toll-lal`).
- **Conditions**: token with `CCC` of the same letter, or a name/compound whose generated form has a hyphen at a triple boundary, or a common word with a hyphen before a suffix starting with its final consonant.
- **Output**: generated form per `ORT-ASSIM-017`, `ORT-HYPH-001`, `ORT-HYPH-002`.
- **Examples**: `sakkkör` -> `sakk-kör`; `sakkör` -> `sakk-kör` (a compound of sakk and kör, so sakkör is rejected); `Széllel` -> `Széll-lel`; `toll-lal` -> `tollal`; `balettáncos` -> `balett-táncos`.
- **Exceptions**: AkH11-era given names (REP-SPELL-010).
- **Predictability**: `rule`
- **Lexical input**: name and compound analysis
- **Interactions**: `ORT-ASSIM-017`, `ORT-HYPH-001`, `ORT-HYPH-002`, REP-SPACE-005.
- **Sources**: [AkH12 §61, §93, §263 a)] via `ORT-ASSIM-017`, `ORT-HYPH`.
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### REP-SPELL-017 Foreign and loanword spelling errors
- **Statement**: Established loanwords have a Hungarian spelling fixed by the dictionary, and common errors are the source-language spelling, half-adapted spellings, and wrong consonant length (`komunikáció`); repairs use the dictionary and never adapt unestablished foreign words.
- **Conditions**: token rejected by the oracle, accepted after one edit from the loanword edit set {`c` <-> `k`, `ph` -> `f`, `th` -> `t`, `x` <-> `ksz`, `qu` -> `kv`, `y` -> `i`, single <-> double consonant, `sz` <-> `s`, `z` <-> `c` (`licensz` -> `licenc`)} and the correction is a dictionary loanword.
- **Output**: dictionary spelling.
- **Examples**: `komunikáció` -> `kommunikáció`; `licensz` -> `licenc`; `boksz` / `box` (both, meaning split: kept); `fondue` kept (foreign spelling accepted); `philosophia` for `filozófia` in modern text -> `filozófia` (REP-ARCH-006 if dated before 1954).
- **Exceptions**: words whose foreign spelling is standard (`show`, `copyright`, `couchette`); names.
- **Predictability**: `lexical`
- **Lexical input**: loanword dictionary
- **Interactions**: `ORT-FOR`; REP-ARCH-006.
- **Sources**: [AkH12 §202-§214] via `ORT-FOR`; [magyarispell hu.aff REP] (loanword suggestion classes; study only).
- **Validation**: `oracle:analyser`.
- **Confidence**: `consensus`.

### REP-SPELL-018 Informal abbreviations and chat spellings
- **Statement**: Chat and forum text uses abbreviations and phonetic spellings (`vmi`, `vki`, `sztem`, `pl` without period, `köszi`, `lécci`, `asszem`, `h` for `hogy`, digit rebus `lát6`, `jó8`); they are register markers, expanded only in a `formalise` mode and never in corpus repair.
- **Conditions**: token in the chat abbreviation list or matching digit-rebus patterns `[a-záéíóöőúüű]+[0-9]` where the digit name completes a word (`6` `hat`, `8` `nyolc`/`éjt`).
- **Output**: `formalise` mode: `vmi` -> `valami`, `vki` -> `valaki`, `vmit` -> `valamit`, `sztem` -> `szerintem`, `pl` -> `pl.`, `köszi` -> `köszönöm`, `asszem` -> `azt hiszem`, `h` (clause-initial) -> `hogy`, `lát6` -> `láthat`; other modes: keep.
- **Examples**: see Output.
- **Exceptions**: `h` as hour symbol after digits (`10 h`); `pl` in code.
- **Predictability**: `lexical`
- **Lexical input**: abbreviation list
- **Interactions**: REP-NOISE-006 (reverse direction generates chat-style noise).
- **Sources**: [PC-HU noise.ron] (`valami` -> `vmi`, `szerintem` -> `sztem`, `például` -> `pl`, `köszönöm` -> `köszi`); [KOROMPAY18 p. 98] (`lát6`, `jó8` in SMS spelling); [magyarispell hu.aff REP] (`asszem`; study only).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

## KEY: keyboard confusions

### REP-KEY-001 Hungarian keyboard layouts
- **Statement**: The Windows Hungarian layout (KBDHU) is QWERTZ with `0` left of `1`, `ö ü ó` on the right of the digit row, `ő ú` right of `p`, `é á ű` right of `l`, and `í` on the ISO-only key left of `y`; the Hungarian 101-key layout (KBDHU1) is QWERTY with `í` on the backquote key and `0` only via AltGr; all keyboard-noise models and wrong-layout repairs are derived from these tables.
- **Conditions**: modelling typing slips and layout confusions for Hungarian text.
- **Output**:
  - KBDHU by physical key: Table T-KEY-1 (base, Shift, AltGr; dead keys marked).
  - KBDHU AltGr layer essentials: `\` Q, `|` W, `Ä` E, `€` U, `Í` I, `÷` ő, `×` ú, `ä` A, `đ` S, `Đ` D, `[` F, `]` G, `í` J, `ł` K, `Ł` L, `$` é, `ß` á, `¤` ű, `<` í-key and M, `>` Y and period, `#` X, `&` C, `@` V, `{` B, `}` N, `;` comma, `*` minus key; dead keys `ˇ ^ ˘ ° ˛ ˙ ´ ˝ ¨ ¸` on AltGr+2 3 4 5 6 8 9 ö ü ó, `~` AltGr+1, backquote AltGr+7.
  - KBDHU1 differences: backquote key gives `í Í` (AltGr `0`); `Y` and `Z` in QWERTY positions; `ű Ű` key AltGr `\` (not `¤`); AltGr+R `§`, AltGr+T `¤`; no AltGr on `ü` and `ó`; the AltGr accent characters on the digit row (`ˇ ^ ˘ ° ˛ ˙ ´ ˝`) are plain spacing characters: KBDHU1 has no dead keys at all.
  - ANSI 104-key boards lack scan code 56: with KBDHU, `í` is reachable only as AltGr+J or with the dead acute (AltGr+9, then `i`).
- **Examples**: typing `0` on KBDHU uses the key US boards label backquote; typing `ö` uses the key US boards label `0`.
- **Exceptions**: macOS and Linux (xkb `hu`) Hungarian layouts differ in AltGr layers and in the digit-row position of `0`; model them separately when the source platform is known.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: REP-KEY-002 .. 008, REP-NOISE-007, REP-DIAC-002 class E.
- **Sources**: [KBDLAYOUT KBDHU, KBDHU1] (KLC files generated from `kbdhu.dll` version 10.0.25393.1 and `kbdhu1.dll` version 10.0.29667.1000, retrieved 2026-09-15; column 6 = AltGr; KBDHU1 has no `DEADKEY` section); [HUWIKI-REPEK] (ten dead keys on AltGr + top row on the Hungarian layout, none on the 101-key layout).
- **Validation**: `goldens` (Table T-KEY-1 regenerated from the KLC files).
- **Confidence**: `normative` (vendor layout data).

### REP-KEY-002 Adjacent-key substitution geometry
- **Statement**: A keyboard slip replaces a letter by a physically adjacent key; adjacency is derived from key-centre geometry of the actual board, not from row strings, because row stagger decides diagonal neighbours.
- **Conditions**:
  - ISO board key centres (key widths as unit): digit row key `i` at x = i starting with the `0`/backquote key at 0; letter row 1 at x = 1.5 + i starting at Q; home row at 1.75 + i starting at A; bottom row at 1.25 + i starting at the `í` key (scan 56).
  - Neighbours: same row with |dx| = 1, or adjacent row with |dx| <= 0.75.
- **Output**: Table T-KEY-2 (neighbours of every Hungarian letter key); substitution candidates in repair are restricted to these neighbours.
- **Examples**: `é` neighbours `p ő l á . -`; `í` neighbours `a y`; `z` neighbours `6 7 t u g h`.
- **Exceptions**: prior art (`0123456789öüó|qwertzuiopőú|asdfghjkléáű|íyxcvbnm,.-` with row offsets 0, 0.5, 0.75, 1.25 and Euclidean reach 1.15) places the letter rows one key left of their physical position relative to the digit row (`q` under `0`/`1` instead of `1`/`2`) and the bottom row one key right relative to the home row (`í` between `a` and `s`); the geometry above supersedes it.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: REP-KEY-007 (repair candidates), REP-NOISE-007 (generation).
- **Sources**: [KBDLAYOUT KBDHU]; ISO/IEC 9995 physical row stagger as implemented on standard ISO boards (Tab 1.5, Caps 1.75, left Shift 1.25 key widths); [PT-DAMAGE corrupt.py `LAYOUT`, `STAGGER`, `REACH`] and [magyarispell hu.aff KEY] (prior art compared; study only for the latter).
- **Validation**: `goldens` (neighbour table).
- **Confidence**: `consensus`.

### REP-KEY-003 Wrong active layout (US versus Hungarian)
- **Statement**: Text typed with the US layout active while the typist expects Hungarian (or the reverse) maps every Hungarian-specific key to its US character and swaps `y`/`z`; the damage is a deterministic key-by-key substitution recoverable with Table T-KEY-1 when a segment shows the signature.
- **Conditions**:
  - HU intended, US active: characters `; ' [ ] \ = - 0` and Shift `: " { } | + _ )` inside word tokens at positions where `é á ő ú ű ó ü ö` (`É Á Ő Ú Ű Ó Ü Ö`) make the word accepted; `y`/`z` swapped; typed `-` appears as `/`, `?` as `<`, `:` as `>`; digit-row Shift symbols differ (`"` typed as `@`, `+` as `#`, `!` as `$`, `/` as `^`, `=` as `&`, `(` as `*`, `)` as `(`).
  - US intended, HU active: `;` appears as `é`, `'` as `á`, `[` as `ő`, `]` as `ú`, `\` as `ű`, `=` as `ó`, `-` as `ü`, `0` as `ö`, backquote as `0`, `/` as `-`, `y`/`z` swapped (common in code, URLs, English passwords typed into text).
  - Segment evidence: >= 3 substituted characters and >= 70% of affected tokens accepted after inverse mapping.
- **Output**: inverse key map over the segment (Table T-KEY-1 columns US base/shift <-> HU base/shift).
- **Examples**: `k;sz` -> `kész`; `h'z` -> `ház`; `t[r` -> `tőr`; `egz` -> `egy`, `hogz` -> `hogy` (y/z); `2ö22` (US habit, HU layout active) -> `2022`.
- **Exceptions**: programming text, emoticons (`;)`), regular punctuation; the segment test prevents single-character repairs.
- **Predictability**: `rule`
- **Lexical input**: oracle for segment acceptance
- **Interactions**: REP-KEY-004 (y/z alone), REP-KEY-005.
- **Sources**: [KBDLAYOUT KBDHU]; US layout (KBDUS) by physical key per the same KLC scan codes.
- **Validation**: `goldens` generated by REP-NOISE-007 layout mode.
- **Confidence**: `normative` (mapping); `tendency` (segment thresholds).

### REP-KEY-004 y/z swap
- **Statement**: QWERTY/QWERTZ confusion swaps `y` and `z` only; in Hungarian the swap is conspicuous because `gy ny ty ly` become `gz nz tz lz` and `sz zs` become `sy ys`.
- **Conditions**: bigram rates in a segment: count of `gz nz tz lz sy` in word tokens exceeds the count of `gy ny ty ly sz`, or individual tokens rejected as written and accepted after swapping all `y`/`z` (`hogz`, `egz`, `nagz`, `yöld`).
- **Output**: swap `y` <-> `z` in the rejected tokens (segment mode: all tokens).
- **Examples**: `hogz` -> `hogy`; `egz` -> `egy`; `yöld` -> `zöld`; `kösyönöm` -> `köszönöm`.
- **Exceptions**: legitimate `gz` in `egzotikus`, `egzakt`, `egzisztencia`; `tz` in names (`Kovátz`, `Horvátz` archaic) and German words; token-level repair only when the swapped form is accepted and the written one rejected.
- **Predictability**: `rule`
- **Lexical input**: oracle
- **Interactions**: part of REP-KEY-003.
- **Sources**: [KBDLAYOUT KBDHU, KBDHU1] (KBDHU1 is QWERTY, KBDHU is QWERTZ); [ATT] (`hogz`, `egz`, `nagz` unattested; `egzotikus` 11).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-KEY-005 Hard-to-reach and missing keys
- **Statement**: Characters that need AltGr or a key absent from the board are systematically dropped or replaced by the base character of the same key: on ANSI boards with KBDHU `í` is typed as `i` (class E, REP-DIAC-002); a failed AltGr chord yields the key's base letter (`v` for `@`, `q` for `\`, `w` for `|`, `f g` for `[ ]`, `b n` for `{ }`, `x` for `#`, `c` for `&`, `é` for `$`, `-` for `*`, `m` for `<`).
- **Conditions**:
  - `í` loss: class E detection (REP-DIAC-002);
  - failed AltGr: e-mail-like tokens with `v` where `@` is required (`nevvgmail.com` -> `nev@gmail.com`), brackets missing in structured text; only in tokens whose repaired form matches a strict pattern (e-mail, URL).
- **Output**: `i` -> `í` via REP-DIAC; pattern-restricted AltGr repair.
- **Examples**: `irás` -> `írás` (class E); `pistavfreemail.hu` -> `pista@freemail.hu`.
- **Exceptions**: e-mail repair only when the local part and domain are plausible and the domain exists in a list; never in prose words.
- **Predictability**: `tendency`
- **Lexical input**: domain list
- **Interactions**: REP-DIAC-002, REP-KEY-001.
- **Sources**: [KBDLAYOUT KBDHU] (scan 56 `í`; AltGr+J `í`; AltGr assignments).
- **Validation**: `goldens`.
- **Confidence**: `tendency`.

### REP-KEY-006 Dead-key residue
- **Statement**: A dead key pressed before a letter it cannot combine with, or followed by space, leaves a spacing accent character (`˝ ¨ ´ ˇ ^ ˘ ˙ ¸ °`) next to the letter; in Hungarian text this appears as `o˝`/`˝o` for `ő`, `u¨`/`¨u` for `ü`, `´a` for `á`, and is repaired by composing the mark with the adjacent vowel.
- **Conditions**: U+02DD, U+00A8, U+00B4, U+02C7, U+005E (inside words only), U+02D8, U+02D9, U+00B8 directly before or after a vowel letter inside a word token; the composed letter makes the token accepted.
- **Output**: compose: `˝` + `o/u` -> `ő/ű`; `¨` + `o/u` -> `ö/ü`; `´` + `a e i o u` -> `á é í ó ú`; other marks: remove when the token without the mark is accepted.
- **Examples**: `˝ot` -> `őt`; `k¨onyv` -> `könyv`; `´allat` -> `állat`; `o˝sz` -> `ősz`.
- **Exceptions**: `^` in formulas and code; `´` as apostrophe substitute (`hál´ istennek`, handled by `WS-NORM-007`); `°` after digits.
- **Predictability**: `rule`
- **Lexical input**: oracle
- **Interactions**: REP-FONT-002; `WS-NORM-006`.
- **Sources**: [KBDLAYOUT KBDHU] (dead key inventory); [HUWIKI-REPEK] (a dead key followed by space yields the spacing mark; invalid combinations produce the mark plus the letter or are ignored).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-KEY-007 Typing slip repair: substitution, insertion, deletion, transposition, case
- **Statement**: Isolated typing slips (adjacent-key substitution, doubled key, dropped key, swapped neighbours, Shift or CapsLock errors) are repaired by generating edit-distance-1 candidates weighted by keyboard geometry and accepting a unique oracle-accepted candidate or the best-ranked one under context.
- **Conditions**: token rejected by the oracle (non-word slips); real-word slips are left to context models and not auto-repaired. One context needs no model: a personal infinitive needs an impersonal governor (`VRB-INF-002`, `VRB-INF-003`), so standing as a sentence of its own it is no reading, and the token is repaired as a non-word slip (`köszönnöm` for `köszönöm`).
- **Output**:
  - candidates: substitution by a Table T-KEY-2 neighbour (cost 1.0), substitution by a non-neighbour (cost 2.5), insertion of a duplicate of an adjacent letter (cost 0.8), deletion of a duplicated letter (cost 0.8), deletion of any letter (cost 1.2), insertion of a neighbour of an adjacent letter (cost 1.2), transposition of adjacent letters (cost 1.0), accent-only edits (REP-DIAC, cost 0.5);
  - case: `hOGY` pattern (first lowercase, rest uppercase) -> CapsLock inversion `Hogy`; `HOgy` (two initial capitals followed by lowercase) -> `Hogy` unless an acronym;
  - accept the lowest-cost accepted candidate if its cost is at least 0.5 lower than the next accepted candidate, else rank by frequency and context; else keep.
- **Examples**: `orsyág` -> `ország` (`y` is a neighbour of `s`); `köszönöm` typed `köszönnöm` -> `köszönöm`; `mnidig` -> `mindig`; `hOGY` -> `Hogy`.
- **Exceptions**: digits introduced by a slip (`or5zág`) are repaired only in prose tokens, never in numbers; proper names and foreign words are left.
- **Predictability**: `tendency` (costs are initial values)
- **Lexical input**: oracle, frequency
- **Interactions**: after DIAC; REP-NOISE-007 (generation with the same edit types).
- **Sources**: [PT-DAMAGE corrupt.py `typos`] (slip mix: neighbour key 50%, doubled letter 25%, dropped letter 25%; rate 4% of words); [NS15 Table 4] (uncorrected errors already present in web text made up 17.91% of restoration mismatches; motivates keeping slip repair separate from accent repair).
- **Validation**: `goldens` from REP-NOISE-007; precision on clean text.
- **Confidence**: `tendency`.

### REP-KEY-008 Mobile keyboard and autocorrect artefacts
- **Statement**: Mobile input produces long-press accent errors (the neighbouring accent variant chosen: `ö` for `ő`, `ó` for `ő`), autocorrect substitutions into other languages' words, and missing accents on words the keyboard dictionary does not know; these are handled as DIAC class C/D errors and as real-word errors, not as layout slips.
- **Conditions**: long-press variants on `o` and `u` keys contain `ó ö ő` / `ú ü ű` in a popup row: a wrong member of the family inside an otherwise accented token.
- **Output**: family-restricted candidates `o` family and `u` family (REP-DIAC-003 with the written accent as a non-final choice); autocorrect foreign words (`the`, `and`) inside Hungarian sentences are reported, not repaired.
- **Examples**: `kerülö` -> `kerülő`; `elsó` (intended `első`) -> `első`.
- **Exceptions**: real words (`ót`/`őt`/`öt`): context.
- **Predictability**: `tendency`
- **Lexical input**: oracle
- **Interactions**: REP-DIAC-011, REP-DIAC-004.
- **Sources**: [PT-DAMAGE corrupt.py `strip_accents` note] (people type the accents their keyboard makes easy and skip the rest); layout popups are platform-specific and not tabulated here (Open questions).
- **Validation**: `goldens`.
- **Confidence**: `tendency`.

## OCR: optical character recognition confusions

### REP-OCR-001 OCR confusion inventory for Hungarian
- **Statement**: OCR of Hungarian print produces a characteristic confusion set dominated by word-space errors (spurious spaces far more often than lost ones in the JiM data), accent errors (lost, added or misread marks, especially on `ő ű í é`) and the Latin-script shape confusions shared with other languages (`rn`/`m`, `cl`/`d`, `li`/`h`, `1`/`l`/`I`, `0`/`O`); repair candidates are generated only from this inventory with per-pair costs.
- **Conditions**: text from scanned sources (source declaration `ocr`), or segments with REP-QUAL signals of OCR damage (analysable share below the source baseline, isolated digits inside words).
- **Output**: Table T-OCR-1 (confusion pairs, direction, relative cost). Accent confusions: `é`<->`e`, `á`->`a`, `í`->`i`/`l`/`1`, `ó`->`o`/`ö`, `ö`->`o`/`ó`, `ő`->`ö`/`o`/`õ`/`ô`/`6`, `ú`->`u`, `ü`->`u`/`ii`, `ű`->`ü`/`u`/`û`; letter shapes: `rn`<->`m`, `cl`->`d`, `li`->`h`, `h`->`b`, `in`->`m`, `ri`->`n`, `u`<->`n`, `e`<->`c`, `t`<->`f`, `g`<->`q`, `l`<->`1`<->`I`<->`|`, `O`<->`0`, `S`<->`5`, `B`<->`8`, `Z`<->`2`, `ſ`->`f` (old prints); ligatures: `fi` `fl` `ff` merged into ligature glyphs or split with a space; quotes: `„` -> `,,`, `”` -> `''` or `"`.
- **Examples**: `bizonyitott` -> `bizonyított`; `kerülö` -> `kerülő`; `rnegye` -> `megye`; `clolgozik` -> `dolgozik`; `1ehet` -> `lehet`; `tör-vény` hyphen remnants per REP-SPACE-004.
- **Exceptions**: real digits in text (`1848`, `2 kg`): digit-letter confusions are candidates only inside tokens that contain letters on both sides of the digit.
- **Predictability**: `tendency`
- **Lexical input**: none (costs)
- **Interactions**: REP-OCR-002 (candidate search); REP-NOISE-008 (generator uses the same table).
- **Sources**: [L22 p. 422 Table 2] (JiM corpus of Jókai and Mikszáth OCR vs publisher e-text, columns OCR -> silver text: OCR had a spurious space 3,523 times (word split) and lacked a space 360 times (fusion), OCR `é` for text `e` 581 times, OCR lacked `r` 232 times, OCR had an extra `y` 11 times, OCR `zd` for `v`, `űié` for `üle`; annotated 8,593 distinct OCR error types per [M24 §4.3]); [M24 §4.2] (`„` recognised as `,,` is one of the most common Hungarian OCR errors); [PT-DAMAGE damage.py `CONFUSIONS`] (inventory including `ő`->`õ`: a small printed double acute reads as a tilde or diaeresis); [PC-HU noise.ron `ocr.confusions`].
- **Validation**: `goldens`; measured confusion matrix on a parallel OCR corpus when available (Open questions).
- **Confidence**: `tendency`.

### REP-OCR-002 Candidate search and repair priorities
- **Statement**: OCR repair applies cheap high-precision classes first and context-dependent classes last: (1) structural (furniture, hyphenation, spacing), (2) punctuation glyphs, (3) non-word tokens repaired by accent-only edits, (4) non-word tokens repaired by one shape confusion, (5) fused and split words, (6) real-word errors only with a context model; each stage re-runs the oracle on its output.
- **Conditions**: OCR-declared text or segments flagged by REP-QUAL-007.
- **Output**: per stage:
  1. REP-SPACE-010, REP-SPACE-004/005;
  2. REP-OCR-003;
  3. accent-only candidates from Table T-OCR-1 (cost <= 1 edit): accept unique accepted candidate;
  4. one shape confusion (or one shape + one accent): accept unique accepted candidate with frequency above the damaged form;
  5. REP-SPACE-009;
  6. REP-OCR-007.
  Edit budget per token: 2 confusions; tokens needing more are flagged, not repaired.
- **Examples**: `kerülö` stage 3 -> `kerülő`; `rnegye` stage 4 -> `megye`; `azegész` stage 5 -> `az egész`; `öt` for `őt` only stage 6.
- **Exceptions**: foreign-language blocks are excluded (REP-QUAL-008).
- **Predictability**: `tendency`
- **Lexical input**: oracle, frequency, context model (stage 6)
- **Interactions**: REP-DIAC (stage 3 shares the candidate engine with a cost model instead of a loss class); detection + correction split mirrors the two-stage approach where a detector narrows positions to reduce false-positive corrections.
- **Sources**: [L22 §4-§5] (combining a detector with a correction model reduces false positives; detection cannot mark spaces, so spaces are treated as potentially erroneous); [M24 §6.2] (the prior mT5 corrector of Laki et al.: 60 of 1,981 error-free sentences changed, false-positive rate 2.98%).
- **Validation**: precision per stage on a held-out parallel set; false-positive rate on clean sentences reported.
- **Confidence**: `tendency`.

### REP-OCR-003 Quotation mark and punctuation glyph confusions
- **Statement**: The Hungarian low-9 opening quote `„` is frequently recognised as two commas `,,` and the closing `”` as `''`, `"` or `“`; dashes are recognised as hyphens; the repair maps these by position.
- **Conditions**:
  - `,,` immediately before a letter or digit, preceded by space, line start or opening bracket: opening quote;
  - `''` or `"` or `“` immediately after a letter or punctuation and followed by space, punctuation or line end, when an opening `„` or `,,` precedes in the same sentence or paragraph: closing quote;
  - spaced hyphen ` - ` between words: dash per REP-SPACE-011.
- **Output**: `„` and `”` (`PUN-QUOT-001`).
- **Examples**: `,,Igen'' - mondta` -> `„Igen” – mondta`.
- **Exceptions**: `,,` inside numbers or code; English-style quoted text in foreign blocks.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PUN-QUOT-001`, `PUN-DASH`, `WS-CHAR-005`.
- **Sources**: [M24 §4.2 step 6]; [magyarispell tipikus/irasjelek] (typewriter-style `,,` and `''` in Hungarian text; study only).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-OCR-004 Page furniture and coherence damage in OCR output
- **Statement**: OCR text interleaves page numbers, running heads, footnote markers and line-break hyphens with the body; these are removed or repaired structurally before any character-level repair, because they break tokens and confuse detectors.
- **Conditions**: digits alone or with hyphen at line boundaries (`45`, `-5`), misrecognised page numbers (`ioo` for `100`), repeated headers; spurious spaces before punctuation.
- **Output**: REP-SPACE-010 (furniture), REP-SPACE-004/005 (hyphens), REP-SPACE-008 (spaces).
- **Examples**: `vágva egy 45 jó birssuhogót` -> `vágva egy jó birssuhogót` when `45` is a page number at a page boundary.
- **Exceptions**: numbers that belong to the sentence: require page-boundary evidence (form feed, margin position, sequence continuity of page numbers).
- **Predictability**: `tendency`
- **Lexical input**: none
- **Interactions**: first in REP-OCR-002.
- **Sources**: [L22 pp. 421-423] (coherence errors: page numbers `5`, `-5`, `ioo` in OCR; example of a detector confused by an interleaved page number `45`).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-OCR-005 Accent-mark degradation in low-resolution or worn print
- **Statement**: The double acute is the most fragile Hungarian mark in OCR: at low resolution or with worn type it is read as a diaeresis, tilde, circumflex, acute, or dropped; dots and acutes on `i í` are confused with each other and with `l 1`; repair favours `ő ű` restoration when the document's `ő ű` share is far below the Hungarian baseline.
- **Conditions**: document `ő` share of the o-family < 50% of baseline (baseline `ő` 14.10% of `o`-family letters, `ű` 10.32% of `u`-family) while `ö ü` shares are above baseline; or `õ ô` present (REP-FONT-001 route).
- **Output**: raise the prior of `ő ű` candidates in REP-OCR-002 stage 3 by the ratio of baseline to observed share (capped at 5).
- **Examples**: a scanned book with `ö` 28% and `ő` 2% of the o-family: `elött` repaired with high confidence; `öt`/`őt` still context-decided.
- **Exceptions**: texts in which `ő` is genuinely rare (short lists, names): require >= 500 o-family letters.
- **Predictability**: `tendency`
- **Lexical input**: letter baselines
- **Interactions**: REP-DIAC-002 class C (same surface, different cause); REP-QUAL-002.
- **Sources**: [NS15 Table 1] (baselines); [PT-DAMAGE damage.py `CONFUSIONS` note]; [PC-HU noise.ron] (`ő`->`õ`, `ű`->`û` OCR confusions).
- **Validation**: evaluation on degraded scans (Open questions).
- **Confidence**: `tendency`.

### REP-OCR-006 Old prints and Fraktur-era texts
- **Statement**: OCR of pre-20th-century Hungarian prints meets long `ſ`, historical letter groups (`cz`, `ts`, `eö`), `oͤ uͤ` notation, ligatures `ch ck tz` and unusual accent shapes; OCR errors must be separated from genuine archaic spelling so that repair does not modernise what the source wrote.
- **Conditions**: document period (source metadata) before 1900, or REP-ARCH-001 score above threshold.
- **Output**: character-level OCR repair restricted to shape confusions that yield an archaic or modern accepted form under an archaic-aware lexicon (`ſ` misread as `f`: `fzép` -> `szép`, or `ſzép` where the archaic spelling is kept); orthographic modernisation is a separate optional pass (REP-ARCH-009).
- **Examples**: `fzeretet` (OCR of `ſzeretet`) -> `szeretet` (normalising mode); `Móricz` kept; `eſztendő` -> `esztendő`.
- **Exceptions**: diplomatic transcriptions keep `ſ`.
- **Predictability**: `tendency`
- **Lexical input**: archaic-aware lexicon
- **Interactions**: REP-FONT-012, REP-ARCH.
- **Sources**: [KOROMPAY18 pp. 96-97] (historical letter forms and when modern forms were established); `WS-NORM-003` (`ſ` folding).
- **Validation**: `goldens` on a small period sample (Open questions).
- **Confidence**: `tendency`.

### REP-OCR-007 Real-word OCR errors
- **Statement**: Many OCR confusions yield other valid Hungarian words (`őt` -> `öt`, `kör` -> `kor`, `szél` -> `szel`, `meg` <-> `még`); these are repaired only by a context model with a confusion-aware channel, never by lexicon lookup, and are reported under conservative policy.
- **Conditions**: token accepted by the oracle, but a Table T-OCR-1 neighbour is also accepted and the context model prefers the neighbour with posterior margin above the policy threshold (REP-DIAC-008 policies).
- **Output**: neighbour word or `ambiguous` label.
- **Examples**: `Láttam öt a boltban` -> `Láttam őt a boltban` (verb + definite object pronoun context); `a kor közepén` vs `a kör közepén`: keep unless context decides.
- **Exceptions**: -
- **Predictability**: `tendency`
- **Lexical input**: context model, confusion costs
- **Interactions**: REP-DIAC-005 (same ambiguity sets).
- **Sources**: [NS15 §6] (contextually inadequate forms are the majority of residual errors in the analogous accent task).
- **Validation**: evaluation slice.
- **Confidence**: `tendency`.

## ARCH: archaic and pre-reform orthography

### REP-ARCH-001 Detecting pre-reform orthography
- **Statement**: Texts written before the 1922 Academy rules (and older layers before 1832) show a small set of orthographic markers in common words (`cz` for `c`, `ts` for `cs` in older Protestant usage, `ds` for `dzs`, unsimplified doubled digraphs `szsz`, `-tya/-tye` phonetic forms, `y` for `i` (unverified), long `ſ`); a document score from these markers labels the text as archaic so that REP-SPELL does not treat historical spelling as error.
- **Conditions**: lowercase common-word tokens (not names) containing:
  - `cz` (score 1.0 per token): `czél`, `czukor`, `kanczellár`; names excluded (REP-ARCH-008);
  - unsimplified doubling `szsz gygy nyny lyly tyty zszs cscs` inside a stem (1.0): `hoszszú`, `aszszony`, `öszsze`, `veszszük`;
  - `ts` where modern `cs` is required and the token is rejected with `ts` but accepted with `cs` (0.8): `kints` -> `kincs`;
  - `ds`/`dsz` for `dzs` (0.8): `lándsa` for `lándzsa`;
  - `-ttya/-ttye/-tya` for `-tja/-tje` (ipszilonista, pronunciation-based usage) (0.6): `láttya`, `kerttye`;
  - archaic morphology cues (0.3, supportive only): `vala`, `valék`, `mondá`, `-ván/-vén` in narrative past, `-ék` preterite;
  - `ſ` (1.0) (REP-FONT-012).
  - Marker weights are estimates (prior art scores `cz` only). Rate = score / word tokens; label `pre_reform` at rate >= 0.01 (strict sources) or >= 0.02 (lenient); `archaic_literary` when only morphology cues fire.
- **Output**: document label with rate; per-token marks.
- **Examples**: `A czukor ára hoszszú idő óta ...` -> `pre_reform`; `vala` alone -> archaic_literary (morphology cue only).
- **Exceptions**: family and place names with archaic letters (REP-ARCH-008); quoted historical titles inside modern text (label the span, not the document).
- **Predictability**: `tendency` (weights and rates not taken from a cited prior-art constant are estimates)
- **Lexical input**: name lexicon; oracle for `ts`/`ds` checks
- **Interactions**: gates REP-SPELL (archaic spans are not repaired as errors); REP-QUAL-009.
- **Sources**: [KOROMPAY18 pp. 93-95] (`tz`-`ts` Protestant vs `cz`-`cs` Catholic tradition, settled for `cz`-`cs` by Kazinczy early 19th century; 1832 rules alphabet with `cz`; `ds` for `dzs` 1856, `dz` 1901; `c` from Simonyi's 1903 school spelling, sanctioned by the Academy in 1922; jottista vs ipszilonista dispute `látja` vs `láttya`); [PT-CLEAN hungarian.py `PRE_REFORM`, catalog gates 0.01/0.02]; [PC-HU quality.ron `archaic: ["cz"]`]; `WS-DIG-010` (unsimplified doubling in `[ATT]`).
- **Validation**: `goldens` on dated samples.
- **Confidence**: `tendency`.

### REP-ARCH-002 cz -> c normalisation
- **Statement**: In common words, `cz` of pre-1922 spelling is normalised to `c` (and `ccz`/`czcz` to `cc`) in modernising mode; proper names keep `cz`.
- **Conditions**: modernising mode; lowercase token (or sentence-initial capital of a common word) containing `cz`, not in the name lexicon, and the `c` form is oracle-accepted; `czcz` and `ccz` (doubled) map to `cc`
- **Output**: `cz` -> `c`; doubled -> `cc`; suffixed derivatives of names keep the name spelling (`móriczi`).
- **Examples**: `czukor` -> `cukor`; `czél` -> `cél`; `rácz` (common noun) -> `rác`; `Móricz Zsigmond` kept; `Czuczor Gergely` kept.
- **Exceptions**: Polish and Czech words (`czas`, `Czech` in English), chemical and Latin terms; names used as common nouns.
- **Predictability**: `rule` with name guard
- **Lexical input**: name lexicon; oracle
- **Interactions**: `WS-LET-007`, `ORT-NAME`; REP-ARCH-008.
- **Sources**: [KOROMPAY18 p. 94]; [AkH12 §86] via `WS-LET-007` (archaic letters kept in family names).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-ARCH-003 ts, tz, ds -> cs, c, dzs normalisation
- **Statement**: Older Protestant-tradition `ts` (for `cs`) and `tz` (for `c`), and 19th-century `ds` (for `dzs`), are normalised in modernising mode only where the modern form is accepted and the old form is not, because `ts` and `ds` are also legitimate sequences across morpheme boundaries (`kétszer`, `látszik`, `fáradtság`, `vadság`).
- **Conditions**: token rejected with the old sequence and accepted after replacing it; the sequence is not at a morpheme boundary in any analysis of the modern form; names excluded.
- **Output**: `ts` -> `cs`, `tz` -> `c`, `ds` -> `dzs`.
- **Examples**: `kints` -> `kincs`; `ketske` -> `kecske`; `tzifra` -> `cifra`; `lándsa` -> `lándzsa`; `Babits`, `Kováts`, `Rátz` kept (names); `vadság` kept (boundary `vad|ság`).
- **Exceptions**: morpheme boundaries (`kétszer`, `fáradtság`), names.
- **Predictability**: `lexical`
- **Lexical input**: oracle with boundaries; name lexicon
- **Interactions**: `WS-LET-007`, `WS-DIG-006`.
- **Sources**: [KOROMPAY18 pp. 93-94].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-ARCH-004 Unsimplified doubled multi-character letters
- **Statement**: Full writing of long multi-character consonants inside stems and before suffixes (`hoszszú`, `aszszony`, `öszsze`) is obsolete and is simplified in modernising mode; at compound boundaries the full form is modern standard and stays.
- **Conditions**: `szsz gygy nyny lyly tyty zszs cscs` (and `dzdz`, `dzsdzs` rare) in a token; analyser boundary check.
- **Output**: no compound boundary inside the sequence: `szsz` -> `ssz`, `gygy` -> `ggy`, `nyny` -> `nny`, `lyly` -> `lly`, `tyty` -> `tty`, `zszs` -> `zzs`, `cscs` -> `ccs`; compound boundary: keep (`jegygyűrű`, `kulcscsomó`, `díszszázad`).
- **Examples**: `hoszszú` -> `hosszú`; `veszszük` -> `vesszük`; `tavaszszal` -> `tavasszal`; `aszszony` -> `asszony`; `profeszszionális` -> `professzionális`; `kulcscsomó` kept.
- **Exceptions**: suffixed proper names kept the full form until AkH10 (1954) (`Aranynyal`, `Keménynyel`) `[KESZLER2016 p. 83]`, so a pre-1954 text may show them alongside already-simplified common words; archival transcription mode keeps all.
- **Predictability**: `rule` given morphology
- **Lexical input**: compound boundaries
- **Interactions**: `WS-DIG-010`, `WS-DIG-004`, `ORT-ASSIM-016`; REP-SPELL-005 (the same form in modern text is an error).
- **Sources**: `WS-DIG-010` (`[ATT]` attestations `hoszszú`, `öszsze`, `veszszük`, `tavaszszal`, `aszszony`, `profeszszionális`); [LACZKO24 pp. 12-13] (Simonyi's 1903 school orthography `Az új helyesírás`, rejected by the Academy, published via the education ministry and spread through schools, the press and book publishing; the Academy adopted its reforms almost in full in 1922); [KESZLER2016 p. 83] (AkH10 1954 extends simplification to suffixed proper names); [HUWIKI-HELYESIRAS] (1832 rules unstable between `asszony` and `aszszony`); [AkH12 §92].
- **Validation**: `goldens`.
- **Confidence**: `consensus` (1903/1922 dating from academic studies; `OQ-10-007` resolved 2026-09-16).

### REP-ARCH-005 Phonetic j-assimilation spellings (láttya type) and y for i
- **Statement**: Spellings that write the pronounced assimilation of stem-final `t`/`d`/`n`/`l` with suffix `j` (`láttya`, `aggya`, `kerttye`) belong to the pronunciation-based (ipszilonista) tradition that Kazinczy's authority decided against, and to dialect representation; `y` for `i` in common words appears in older prints (claim not verified in the cited sources); both are normalised only in modernising mode.
- **Conditions**: token rejected by the oracle; replacing `tty`/`ty` + vowel by `tj` + vowel, `ggy` by `dj`, `nny` by `nj`, `lly` by `lj` gives an accepted form with analyser boundary before `j`; or replacing word-final `y` by `i` gives an accepted non-name word.
- **Output**: morphological spelling (`látja`, `adja`, `kertje`, `bánja`).
- **Examples**: `láttya` -> `látja`; `aggya` -> `adja`; `kerttye` -> `kertje`; `bánnya` -> `bánja`.
- **Exceptions**: modern dialect-marked literary text (`ORT-PRIN-013`); names with archaic `y` (`Kölcsey`, `Batthyány`, `Széchenyi`).
- **Predictability**: `rule` given morphology
- **Lexical input**: oracle
- **Interactions**: `ORT-PRIN`, `ORT-ASSIM-018`; REP-ARCH-008.
- **Sources**: [KOROMPAY18 pp. 93-94] (jottista `látja` (Révai) vs ipszilonista `láttya` (Verseghy); Kazinczy decided for the morphological principle); [AkH12 §48] via `ORT-PRIN`.
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-ARCH-006 Pre-1954 foreign-word and classical-name spelling
- **Statement**: Before AkH10 (1954) Latinate and Greek spellings of established loanwords and names were common (`philosophia`, `injectio`, `Odysseus`); AkH10 made Hungarian spelling of established loanwords binding and changed Greek name transcription (`Odüsszeusz`); modernising mode maps dated spellings to the current dictionary form through a lexicon, never by letter rules alone.
- **Conditions**: document dated before 1954 (or `pre_reform` label); token found in a historical-to-modern loanword/name mapping lexicon.
- **Output**: modern dictionary spelling.
- **Examples**: `philosophia` -> `filozófia`; `injectio` -> `injekció`; `Odysseus` -> `Odüsszeusz`; `ethika` -> `etika`.
- **Exceptions**: Latin quotations and scientific binomials stay; names of persons keep their own spelling.
- **Predictability**: `lexical`
- **Lexical input**: mapping lexicon (size stated at build)
- **Interactions**: `ORT-FOR`; REP-SPELL-017.
- **Sources**: [KOROMPAY18 pp. 94-95] (AkH10 1954: binding Hungarian spelling of foreign words such as `injekció`, `egzotikus`; Greek names by pronunciation, `Odysseus` -> `Odüsszeusz`; `edzze` -> `eddze`).
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-ARCH-007 Changes of AkH10 and AkH11 visible in spelling
- **Statement**: Two well-documented 20th-century changes are normalised in modernising mode: the doubling of `dz` before `j`-assimilated suffixes (`edzze` before 1954, `eddze` after) and the hyphenation treatment of `dz`/`dzs` as single letters (`bod-za`, `maharad-zsa` before 1984; `bo-dza`, `mahara-dzsa` after, relevant only for REP-SPACE-004 rejoining).
- **Conditions**: token `edzze`-type (long `dz` written `dzz`) in pre-1954 text in `modernise` mode (in text dated after 1954 it is a REP-SPELL-005 error); line-break splits `d-z`, `d-zs` in pre-1984 prints.
- **Output**: `edzze` -> `eddze`; rejoining `bod-` / `za` -> `bodza` (plain join).
- **Examples**: `edzze` -> `eddze`; `maharad-` / `zsa` -> `maharadzsa`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `ORT-SPLIT-011`, `WS-DIG-005`; REP-SPACE-004.
- **Sources**: [KOROMPAY18 pp. 94-95].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-ARCH-008 Archaic letters in names are not damage
- **Statement**: Family names, some given names and place-derived names keep historical letters and letter groups (`cz`, `ts`, `th`, `ch`, `gh`, `ph`, `aa`, `eé`, `eö`, `ew`, `oó`, `ö`+`e`, `y` for `i`, `w` for `v`); every ARCH, SPELL and DIAC repair must skip them, and their suffixed and derived forms follow `ORT-NAME` (`Kossuthtal`, `móriczi`).
- **Conditions**: capitalised token in a name lexicon or matching the archaic name pattern (`WS-LET-007` groups) in a name position (preceded by a given name or title, or followed by a given name in Hungarian name order).
- **Output**: keep.
- **Examples**: `Móricz`, `Babits`, `Kossuth`, `Széchenyi`, `Batthyány`, `Dessewffy`, `Eötvös`, `Beöthy`, `Madách`, `Thaly`, `Gaal`, `Veér`; `móriczi` (derived adjective) kept.
- **Exceptions**: a name spelled in modern letters by its bearer (`Babics`) is a different name, not a repair target.
- **Predictability**: `lexical`
- **Lexical input**: name lexicon; `WS-LET-007` patterns
- **Interactions**: `WS-LET-007`, `WS-LET-008`, `ORT-NAME`, `ORT-SPLIT-013`; guards REP-ARCH-002 .. 005, REP-SPELL, REP-DIAC-009.
- **Sources**: [AkH12 §86, §155] via `WS-LET-007`.
- **Validation**: `goldens`.
- **Confidence**: `normative`.

### REP-ARCH-009 Normalisation policy for archaic text
- **Statement**: Archaic orthography is data, not damage: repair pipelines label it and leave it unchanged by default; modernisation is an explicit, separate, provenance-recording transformation, and archaic text is never used as the gold side of modern-spelling repair pairs.
- **Conditions**: any text labelled `pre_reform` or `archaic_literary`.
- **Output**: modes: `keep` (default; SPELL and DIAC disabled on labelled spans), `modernise` (REP-ARCH-002 .. 007 applied; morphology and lexis untouched: `vala`, `mondá` stay), `transcribe` (keep everything including `ſ`).
- **Examples**: a 1890 novel in `keep` mode is stored as written with label `pre_reform`.
- **Exceptions**: -
- **Predictability**: `rule` (policy)
- **Lexical input**: none
- **Interactions**: REP-NOISE-010 (gold pools exclude archaic text); REP-QUAL-009.
- **Sources**: [PT-DAMAGE repair-axes.md "Gold pools are modern-orthography sources only"] (a crawled archaic translation passed the analyser at >= 0.90; source restriction removes the bulk, not every instance); [PT-CLEAN cleaning.md "The cleaned stage records the verdict"] (`pre_reform_orthography` is a column, not a drop).
- **Validation**: audit of gold pools for ARCH labels.
- **Confidence**: `consensus`.

## NOISE: generating damage for repair training pairs

### REP-NOISE-001 Pair contract
- **Statement**: A repair training pair is `(damaged, gold)` where gold is human-written or oracle-verified clean text and damaged is produced from gold by declared generators; the pair is correct by construction, the damaged side never appears in any stream without its gold, and every pair records the generators, their parameters and the seed.
- **Conditions**:
  - Gold eligibility: modern-orthography sources only (no `pre_reform`/`archaic_literary` labels, REP-ARCH-009); REP-QUAL gates: analysable share >= 0.90 (>= 0.95 for spacing-fragment gold), language Hungarian, mojibake, replacement and PUA counts zero, no residual `õ û ô ı` in Hungarian words.
  - Machine-repaired text is not gold (it inherits repair residue, e.g. `H a a tanuló` joined to `Haa tanuló`).
  - Lane assignment is fixed at generation: `sha1(id) mod 10` in 0-6 `sft`, 7-8 `cpt`, 9 `eval`; consumers filter by the stored lane and never recompute.
  - Clean-copy share: 25-35% of pairs have `damaged == gold` (repair models must learn to leave clean text alone); the reference OCR-correction corpus used 33.6% error-free sentences.
- **Output**: record `{id, lane, gold, damaged, generators: [{rule, version, params}], seed, fired: [rule...]}`; a pair whose generators changed nothing is either kept as a clean copy (if the quota is not filled) or discarded, never labelled with a damage it does not carry.
- **Examples**: `{gold: "Győrben", damaged: "GyÅ‘rben", generators: [REP-NOISE-002 utf8-as-cp1252 level 1]}`.
- **Exceptions**: -
- **Predictability**: `rule` (contract)
- **Lexical input**: none
- **Interactions**: REP-NOISE-002 .. 011; REP-QUAL gates.
- **Sources**: [PT-DAMAGE repair-axes.md] (lanes, gold pools, analysable thresholds, degraded text only inside a pair); [PT-DAMAGE damage.py docstring] (human original as gold, machine-repaired gold inherits residue); [M24 §4] (33.6% error-free sentences in training data).
- **Validation**: schema check; replay from seed reproduces the pair exactly.
- **Confidence**: `consensus`.

### REP-NOISE-002 Encoding damage generators
- **Statement**: Encoding damage is generated by actual byte-level round trips through the confusions of `ENC`, not by substituting pair lists, so every generated string is one a real decoder would produce.
- **Conditions** (one encoding generator per pair; encoding faults do not compose with each other or follow accent loss):
  - `utf8_as` with code page drawn from {**windows-1250: 0.35, windows-1252: 0.25**, ISO-8859-1: 0.05, ISO-8859-2: 0.10, CP852: 0.08, CP437: 0.04, MacRoman: 0.05, MacCE: 0.03, WHATWG C1 variant of 1252: 0.05} - the two leading weights are **swapped** against the measurement of `REP-ENC-003`, where `P(1252 | mojibake)` is 0.30 for Hungarian text, i.e. 0.30 / 0.70 of the 1252+1250 mass; level drawn from {1: 0.85, 2: 0.13, 3: 0.02}; scope {whole text: 0.6, one paragraph or quoted block: 0.3, one sentence: 0.1} (mixed documents, REP-ENC-015).
  - `latin2_as_latin1` (`ő ű Ő Ű` -> `õ û Õ Û`): scope as above; variant `ô` for `ő` (CWI-2 as CP437 or typed substitute) with probability 0.2.
  - `best_fit` (`ő ű` -> `o u`), `replacement` (`ő ű` -> `?` or U+FFFD).
  - `dos_as_windows` (CP852 <-> windows-1250), `mac_as_windows`.
  - `escape`: HTML named (0.5), numeric (0.3), JSON `\u` (0.2) for accented letters, per character with share s.
  - `seven_bit`: flying accent or 123 notation or telegram style (whole text).
  - Strict decoders: 20% of `utf8_as` pairs replace bytes undefined in the target table by `?` instead of C1 controls.
- **Output**: damaged string; generator record with code page, level, scope.
- **Examples**: gold `Kérem, győzzön!` -> `utf8_as windows-1250`: `KĂ©rem, gyĹ‘zzĂ¶n!`; -> `latin2_as_latin1`: `Kérem, gyõzzön!`.
- **Exceptions**: gold without any affected character yields a no-op (REP-NOISE-001 handling).
- **Predictability**: `rule` (generation); `tendency` (weights proposed, to be matched to measured corpus rates)
- **Lexical input**: none
- **Interactions**: REP-ENC-001 .. 016 must invert every generated case (REP-NOISE-011).
- **Sources**: [PT-DAMAGE corrupt.py `mojibake`, `latin2`; damage.py `SINGLE`] (encoding faults do not compose); [PT-CLEAN cleaning.md] (Latin-2 damage is the third largest repair class, 534,403 occurrences in one corpus); code pages per Table T-ENC-2..5; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: REP-NOISE-011; the two leading `utf8_as` weights are now matched to a measured corpus rate rather than assumed.
- **Confidence**: `tendency`; the 1250/1252 split is `measured`.

### REP-NOISE-003 Font and glyph damage generators
- **Statement**: Font damage generators reproduce substitute glyphs, the shifted double acute font, private-use leftovers, ligatures, soft hyphens and invisible characters at rates and scopes observed in PDF-derived text.
- **Conditions** (parameters):
  - `substitute`: `ő` -> {`õ`: 0.6, `ô`: 0.35, `ǒ`: 0.05}, `ű` -> {`û`: 0.9, `ũ`: 0.1}, share s in [0.5, 1.0] of occurrences (fonts damage consistently).
  - `font_shift`: `ő` -> `ı`, `ű` -> `ő`, `Ő` -> `İ`, `Ű` -> `Ő` over the whole text (0.8) or one region (0.2, generating the mixed-font `flag` case).
  - `offset_font`: constant code point offset applied to one region (rare, 0.02 of font pairs; for detector training only).
  - `pua`: replace a Symbol-range character in formulas by U+F000 + byte; ligature letters `ff fi fl ffi ffl` -> U+FB00..U+FB04 (0.7) or U+F000..U+F004 (0.3).
  - `soft_hyphen`: insert U+00AD at `ORT-SPLIT` break points with probability 0.02 per break point.
  - `invisible`: insert U+200B, U+2060 or U+FEFF between letters at rate 0.001 per character; NBSP instead of a space at 0.02 per space.
  - `doubled_acute`: append U+0301 after `ő ű á é` at rate 0.3 per occurrence in one paragraph.
- **Output**: damaged string.
- **Examples**: `működés` -> `mőködés` (font_shift), `mûködés` (substitute).
- **Exceptions**: -
- **Predictability**: `tendency` (weights and rates not taken from a cited prior-art constant are estimates)
- **Lexical input**: syllabifier for soft hyphen positions
- **Interactions**: REP-FONT-001 .. 012.
- **Sources**: [PT-DAMAGE damage.py `font_shift`, `ligatures`, `entities`]; [PT-CLEAN cleaning.md] (measured artefact classes).
- **Validation**: REP-NOISE-011.
- **Confidence**: `tendency`.

### REP-NOISE-004 Spacing damage generators
- **Statement**: Spacing generators reproduce PDF text-layer shapes: letterspaced emphasis, per-syllable spans, hard line wraps with Hungarian hyphenation (including written-out doubled digraphs), lost hyphens as U+FFFD, fused and split words, missing and spurious spaces around punctuation.
- **Conditions** (parameters) (share s drawn per pair in [0.3, 1.0]):
  - `letter_spaced`: choose `round(3 s)` words of length >= 3; join letters with a single space (0.7) or NBSP (0.3); all-caps variant for headings (0.3).
  - `syllable_spaced`: one contiguous stretch covering `round(s * sentences)` sentences; every word split into syllables by `ORT-SPLIT` syllabification (one vowel per syllable; a single consonant opens the next syllable; of a cluster only the last consonant moves; multi-character letters indivisible).
  - `hard_wrap`: line width uniform in [45, 80] characters; a word of length >= 6 crossing the margin is hyphenated with probability s at an `ORT-SPLIT` break point; a break inside a simplified doubled digraph writes both halves in full (`vissza` -> `visz-` / `sza`).
  - `lost_hyphen`: at doubled digraph sites (`ccs ggy lly nny ssz tty zzs`) insert U+FFFD after the first letter for `round(s * sites)` sites.
  - `split_fused`: split a word of length >= 7 at a random interior point with probability 0.05 s; delete a space between two words with probability 0.04 s.
  - `punct_space`: delete the space after `, . ; : ? !` with probability s; insert a space before them with probability 0.3 s; double a word space with probability 0.15 s.
- **Output**: damaged string.
- **Examples**: `A Kormány rendeletei` -> `A Kor mány ren de le tei`; `asszony` at a line end -> `asz-` LF `szony`.
- **Exceptions**: -
- **Predictability**: `rule` (syllabification), `tendency` (rates) (weights and rates not taken from a cited prior-art constant are estimates)
- **Lexical input**: syllabifier (`ORT-SPLIT`)
- **Interactions**: REP-SPACE-001 .. 012.
- **Sources**: [PT-DAMAGE damage.py `letter_spaced`, `syllable_spaced`, `hard_wrap`, `lost_hyphen`, `split_fused`; corrupt.py `squash_spaces`]; [AkH12 §226] via `ORT-SPLIT-007`.
- **Validation**: REP-NOISE-011.
- **Confidence**: `tendency`.

### REP-NOISE-005 Diacritic loss generators
- **Statement**: Diacritic loss is generated per class of REP-DIAC-002 with a per-letter loss probability, because partial loss is the common real case; full loss is one class among several.
- **Conditions** (parameters; class weights for accent-loss pairs; share s per pair):
  - `A` full: 0.35; s = 1.0 (0.6) or uniform [0.15, 1.0] per accented letter (0.4). Mapping `á é í ó ö ő ú ü ű` -> `a e i o o o u u u` (not Unicode decomposition: `ő` becomes `o`, not `ö`).
  - `B` double acute to base: 0.10; `ő ű Ő Ű` -> `o u O U`, s = 1.0.
  - `C` double acute to umlaut: 0.15; `ő ű` -> `ö ü`, s in [0.7, 1.0].
  - `C+D` German keyboard: 0.10; acute letters lose accent with s in [0.7, 1.0], `ő ű` -> `ö ü`.
  - `E` í only: 0.10; `í` -> `i`, s = 1.0.
  - `F` capitals: 0.05; accents removed on uppercase letters only.
  - `mobile`: 0.15; family-internal wrong choice (`ő` -> `ö` or `ó`, `ű` -> `ü` or `ú`) at s in [0.05, 0.2], plus class A loss at s in [0.1, 0.4].
  - Composition: accent loss composes with case loss (lowercasing share) and sentence punctuation loss; it never precedes encoding generators.
  - **Measured prevalence in the project corpus**, over the 1,067,174 Hungarian documents long enough to classify: class B 0.245%, class E 0.210%, classes C and D 3 and 4 documents respectively - **unmeasured, not measured-zero**, since n = 3 and n = 4 cannot distinguish "does not happen" from "the detector does not fire". Fully accent-stripped Hungarian is 0.024% of Hungarian documents (2,320 of 9.64 M, of which 2,289 are in one web crawl). The generator's weights above are a training-mix decision and need not match these rates, but the gap should be deliberate.
  - Evaluation lane oversamples sentences containing Table T-DIAC-2 ambiguity sets (at least 30% of eval pairs).
- **Output**: damaged string with class label.
- **Examples**: `Győr előtt kérem` -> A: `Gyor elott kerem`; B: `Gyor elott kérem`; C: `Györ elött kérem`; E unaffected.
- **Exceptions**: guarded tokens (URLs, e-mails, code; REP-DIAC-009) are not damaged, so the model does not learn to accent them.
- **Predictability**: `tendency` (weights proposed)
- **Lexical input**: none
- **Interactions**: REP-DIAC-001 .. 012; REP-NOISE-010.
- **Sources**: [PT-DAMAGE corrupt.py `ACCENTS`, `strip_accents`] (bare-vowel mapping; partial loss common); [PT-DAMAGE repair-axes.md] (deaccent substrate); [NS15], [LY20] (full-loss training setups); [ATT evidence/calibration.md 2026-09-16].
- **Validation**: REP-NOISE-011; class detector accuracy on generated classes (`REP-DIAC-002`).
- **Confidence**: `tendency` (weights); `measured` (real prevalence of each class).

### REP-NOISE-006 Native spelling error generators
- **Statement**: Spelling noise applies the inverse of each REP-SPELL detector at controlled rates, choosing only edits that produce a form a native writer produces (non-words and the documented real-word confusions), never random character noise.
- **Conditions** (parameters) (per eligible site; overall target 1-3% of words):
  - `jly`: `ly` -> `j` (0.6 of j/ly edits), `j` -> `ly` (0.4, hypercorrection) at sites where the result is attested as a misspelling or rejected by the oracle.
  - `len_vowel`: `í` -> `i` (0.30), `ú` -> `u` (0.15), `ű` -> `ü` (0.15), `ő` -> `ö` (0.15), `ó` -> `o` (0.10), `i` -> `í` (0.10, e.g. `mindíg`), `u` -> `ú` (0.05); suffix-rule sites (`-ul` -> `-úl`, `-ból` -> `-bol`) at 0.3 of vowel edits.
  - `len_cons`: `tt` -> `t` (0.35), `ll` -> `l` (0.20), `nn` -> `n` (0.10), single -> double (0.20: `kissebb`, `lessz`), `dd` -> `d` in imperatives (0.15).
  - `digraph`: `ssz` -> `sz` (0.4), `nny` -> `ny` (0.2), `ggy` -> `gy` (0.2), full doubling `szsz` (0.1), imperative `tsz` -> `sz`/`ssz` confusions (0.1).
  - `assim`: `-val/-vel` unassimilated or single consonant (`ezel`), name hyphen omitted (`Barossal`).
  - `case_usage`: `-ban` -> `-ba`, `-ben` -> `-be` (static verb contexts only).
  - `tog`: join two words from a list of commonly joined phrases (`nem tudom` -> `nemtudom`) or split a lexicalised compound (`nemrég` -> `nem rég`).
  - `cap`: capitalise a derived place adjective or month/day name mid-sentence.
  - `hyph`: drop the hyphen before a suffix on an acronym or numeral (`MÁV-nál` -> `MÁVnál`).
  - `chat`: abbreviations (`valami` -> `vmi`, `szerintem` -> `sztem`, `például` -> `pl`, `köszönöm` -> `köszi`) only in informal-register pairs.
  - `list`: substitute a REP-SPELL-015 list entry (`mindig` -> `mindíg`).
- **Output**: damaged string.
- **Examples**: `Kellett volna azonnal szólnod.` -> `Kellet volna azonal szólnod.`
- **Exceptions**: proper names, quotations and foreign spans are not edited.
- **Predictability**: `tendency` (weights and rates not taken from a cited prior-art constant are estimates)
- **Lexical input**: oracle, misspelling list, phrase lists
- **Interactions**: REP-SPELL-001 .. 018.
- **Sources**: [PC-HU noise.ron `ortho`] (slips `ly`<->`j`, `í`->`i`, `ú`->`u`, `ő`->`ö`, `ű`->`ü`, `ó`->`o`, `tt`->`t`, `ll`->`l`, `ssz`->`sz`; endings `ban`->`ba`; abbreviations; fused pairs); [ATT].
- **Validation**: REP-NOISE-011.
- **Confidence**: `tendency`.

### REP-NOISE-007 Keyboard noise generators
- **Statement**: Keyboard noise uses the Hungarian layout geometry (REP-KEY-002) for substitutions and the layout tables for wrong-layout and y/z damage.
- **Conditions** (parameters):
  - `typos`: select words of length >= 4 with probability 0.04 s; slip position not the first or last letter; slip type: neighbour substitution 0.45 (neighbour from Table T-KEY-2, digits and punctuation allowed), doubled letter 0.20, dropped letter 0.20, transposition 0.10, case slip 0.05; if nothing was selected in a short text, slip the longest word.
  - `layout`: whole segment typed with the other layout (REP-KEY-003), probability 0.02 of keyboard pairs.
  - `yz`: swap `y` and `z` in a segment, 0.03 of keyboard pairs.
  - `altgr_fail`: replace AltGr characters by base letters in e-mail-like tokens, 0.01.
  - `dead_key`: insert a spacing mark before a vowel instead of composing (`˝o`), 0.01.
  - Never change digits inside numbers (only letter runs are slip sites).
- **Output**: damaged string.
- **Examples**: `ország` -> `orsyág`; `hogy egy` -> `hogz egz`.
- **Exceptions**: -
- **Predictability**: `tendency` (weights and rates not taken from a cited prior-art constant are estimates)
- **Lexical input**: layout tables
- **Interactions**: REP-KEY-001 .. 008.
- **Sources**: [PT-DAMAGE corrupt.py `typos`, `TYPO_RATE` 0.04, `NEIGHBOURS`] (rate above about 1/20 of words makes text unreadable; digits never selected as slip sites); [KBDLAYOUT KBDHU].
- **Validation**: REP-NOISE-011.
- **Confidence**: `tendency`.

### REP-NOISE-008 OCR noise generators
- **Statement**: OCR noise applies Table T-OCR-1 confusions weighted by their costs, word fusion and splitting, quote glyph errors, furniture insertion and hard wraps, at word error rates matching real scanned text.
- **Conditions** (parameters):
  - `misread`: per word probability 0.12 s; choose one confusion site uniformly among matching Table T-OCR-1 pairs weighted by 1/cost; accent confusions receive 50% of the weight mass.
  - `fusion_split`: split of a word of length >= 7 at 0.05 s per word, deletion of a space at 0.005 s per space (splitting about 10x more frequent than fusion, matching the JiM counts 3,523 spurious spaces vs 360 lost spaces; rates are estimates).
  - `quotes`: `„` -> `,,` (0.6), `”` -> `''` (0.4) per occurrence.
  - `furniture`: insert a page number or running head at page boundaries every 35-45 lines (0.3 of OCR pairs).
  - `wrap`: REP-NOISE-004 `hard_wrap`.
  - Target intensity: mean character error rate about 3-12% and word error rate about 10-12% for heavy-noise pairs (reference corpus averages CER 12.35%, WER 11.74%), with light-noise pairs at CER 1-3%.
- **Output**: damaged string with measured CER/WER stored.
- **Examples**: `A tűzoltók megérkeztek „azonnal”.` -> `A tüzoltók rnegérkeztek ,,azonnal''.`
- **Exceptions**: -
- **Predictability**: `tendency` (weights and rates not taken from a cited prior-art constant are estimates)
- **Lexical input**: confusion table
- **Interactions**: REP-OCR-001 .. 007.
- **Sources**: [PT-DAMAGE damage.py `misread`, `OCR_RATE` 0.12, `split_fused`]; [L22 Table 2]; [M24 §4] (CER 12.354%, WER 11.739% across the training set).
- **Validation**: REP-NOISE-011.
- **Confidence**: `tendency`.

### REP-NOISE-009 Archaic orthography generators (detection only)
- **Statement**: Reverse modernisation (`c` -> `cz`, `ssz` -> `szsz`, `látja` -> `láttya`) is generated only to train and evaluate the archaic-orthography detector and the optional modernising transform; such pairs never enter general repair lanes as damaged input for modern gold.
- **Conditions** (parameters): `cz` for every `c` in common words (1.0 within the scope), `szsz`-type full doubling (1.0), `ts` for `cs` (0.3 of archaic pairs), `láttya`-type (0.2), `ſ` for `s` word-internally (0.1, old-print style); names untouched.
- **Output**: archaic-styled string labelled `arch_generated`.
- **Examples**: `A cukor ára hosszú ideje` -> `A czukor ára hoszszú ideje`.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: name lexicon
- **Interactions**: REP-ARCH-001 .. 009.
- **Sources**: [KOROMPAY18 pp. 93-95]; `WS-DIG-010`.
- **Validation**: REP-NOISE-011.
- **Confidence**: `consensus`.

### REP-NOISE-010 Composition, intensity and curriculum
- **Statement**: Real damaged text carries several faults at once; pairs combine generators under composition constraints, each generator at an independently drawn intensity, and the mixture spans light to heavy damage.
- **Conditions**:
  - Composable families: {diacritic loss, case loss, sentence punctuation loss, spacing slips, typos, native spelling errors}; mix size uniform in {1, 2, 3, 4}; order shuffled except that spelling and typo edits precede accent loss (a writer misspells, then the keyboard loses accents).
  - Non-composable: each encoding generator (exclusive); font shift (exclusive with encoding); PDF spacing (`letter_spaced`, `syllable_spaced`, `hard_wrap`, `lost_hyphen`, ligatures: up to 3 together) may combine with font generators; OCR (`misread`, `fusion_split`, `wrap`, `quotes`, `furniture`: up to 3) excludes typing generators.
  - Encoding generators never follow accent loss (nothing left to damage) and never precede a second encoding generator except `level` repetition.
  - Per-generator share lower bound 0.15 (composable) or 0.3 (PDF/OCR), upper bound 1.0.
  - Curriculum: light (1 generator, share <= 0.4) 40%, medium 40%, heavy 20%; eval lane stratified by family.
- **Output**: pair with ordered generator list.
- **Examples**: `accents(0.6) + case(1.0)`: `Holnap Győrbe megyünk.` -> `holnap gyorbe megyunk.`
- **Exceptions**: -
- **Predictability**: `tendency` (weights and rates not taken from a cited prior-art constant are estimates)
- **Lexical input**: none
- **Interactions**: REP-NOISE-002 .. 009.
- **Sources**: [PT-DAMAGE corrupt.py `MIXABLE`, `MIX_SIZES`, `MIN_SHARE` 0.15, `COMBINED`; damage.py `KINDS`, `SINGLE`, `MIN_SHARE` 0.3].
- **Validation**: distribution report per lane.
- **Confidence**: `tendency`.

### REP-NOISE-011 Validating generated pairs against detectors
- **Statement**: Every generator is validated by its inverse: the matching REP detector must fire on the damaged side, the repair must reproduce gold (or a documented acceptable variant) for deterministic damage, and must not change the gold side; failures are either detector bugs or generator bugs and block the release of that generator version.
- **Conditions**:
  - Deterministic damage (ENC byte round trips, font shift, escapes, 7-bit notations, y/z and layout swaps, lost hyphen, hard wrap, letterspacing without one-letter-word runs): exact recovery required, target 100% on generated pairs.
  - Ambiguous damage (DIAC, OCR, typos, spelling): report recovery accuracy by class and ambiguity set; precision on clean copies must be >= 99.5% of tokens unchanged.
  - Idempotence: `repair(repair(x)) == repair(x)` for all pairs; entity decoding runs once and is exempt only by the run-once contract (REP-ENC-013).
  - Negative sets: foreign-language texts with `õ û ô ı Ã Ă` legitimately (Estonian, Portuguese, French, Slovak, Turkish, Romanian), code, formulas.
- **Output**: validation report per generator version; failing cases stored as goldens.
- **Examples**: generated `KĂ©rem` must be repaired to `Kérem` by REP-ENC-003; generated `Võru` inside Hungarian text must survive REP-FONT-001.
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: all areas.
- **Sources**: [PT-CLEAN cleaning.md "The layout filter was tested against its own output"] (negative-set discipline: run the transform on text that should not need it and count).
- **Validation**: this rule is the validation procedure.
- **Confidence**: `consensus`.

## QUAL: text quality signals

### REP-QUAL-001 Accent density and per-letter distribution
- **Statement**: The share of accented letters and the distribution of each vowel family over its accented members are the primary signals for accent loss, loss class, substitute glyphs and wrong decoding; they are measured per document and per block and compared with Hungarian baselines.
- **Conditions**: >= 200 letters for density; >= 200 letters of a family for its distribution; compute on word tokens excluding URLs, code and all-caps acronyms. **These are corpus-wide values and the register spread is wider than the damage they are used to detect**: accented-token share runs 37.2% (`dialogue`) to 51.8% (`legal`), and `ö/(ö+ő)` runs 0.39 (`administrative`) to 0.60 (`narrative`), against fixed baselines of 48% and 0.53. Compare a document against its own source distribution where one is known. Two consequences the detectors must carry: `dialogue` (subtitle and parliamentary text) at 37.2% will read as partially stripped to a detector tuned on 43-48%, and undamaged `administrative` Hungarian sits **below** the `share(ö) > 0.40` line that `REP-DIAC-002` class C tests.
- **Output** (baselines):
  - accented tokens: **47.77% of tokens, 11.76% of letters, 9.57% of characters**, measured 2026-09-16 on 9,644,687 Hungarian documents (77.92 Gchar, 63.42 G letters, 10.40 G word tokens) of the project corpus. `[ATT]`'s 11.8% of letters is confirmed; **`[LY20]`'s 6.59% of characters is 31% low** and `[NS15]`'s 43.16% of tokens is 10% low against this corpus. The pair-gold cross-check (19,038 distinct paragraphs, 8.71 M letters) gives 11.90% / 9.82% / 49.69%, and the `web` register alone reproduces it, so the systematic gap is the 2.17 M non-web Hungarian documents, whose accent rates are lower;
  - per family (Table T-DIAC-1): `a` 70.33 / `á` 29.66; `e` 73.40 / `é` 26.59; `i` 86.04 / `í` 13.95; `o` 55.41 / `ó` 14.65 / `ö` 15.82 / `ő` 14.10; `u` 46.96 / `ú` 12.72 / `ü` 29.98 / `ű` 10.32 (percent) [NS15 Table 1]; `[ATT]` estimate agrees within 2.7 points for every letter. **Confirmed corpus-wide within 2 points for every letter except the `u` family**, where this corpus has 4.8 points more plain `u` (51.72) and 3.5 points fewer `ü` (26.50); both double-acute letters run about 1.2 points below NS15 (`ő` 12.89 vs 14.10, `ű` 8.94 vs 10.32), partly register mix and partly real unrepaired damage still in the corpus. Derived baselines measured twice: `ö/(ö+ő)` = **0.5358** corpus-wide and 0.531 on the pair gold, confirming the rulebook's 0.53; `í/(i+í)` = **0.1239** corpus-wide and 0.1366 on the pair gold, i.e. the corpus sits at `[ATT]`'s 0.13 rather than `[NS15]`'s 0.14.
  - signals: `accent_density` = accented letters / letters; `family_share(x)`; `class` per REP-DIAC-002.
- **Examples**: a document with `é` 0.1% of `e`-family and `ö` 0% is class A or foreign; with `ő` 0% but `ö` 30% is class C.
- **Exceptions**: technical and formula-heavy text has low density by character count; use token-level measures and block exclusion; accent density is a weak language cue and is used only when the function-word test cannot run.
- **Predictability**: `tendency`
- **Lexical input**: none
- **Interactions**: REP-DIAC-001, 002; REP-OCR-005; REP-ENC-016.
- **Sources**: [NS15]; [LY20]; [ATT]; [PT-CLEAN cleaning.md "Gates decide with the strongest evidence available"] (accent density wrong on technical text); [ATT evidence/calibration.md 2026-09-16].
- **Validation**: baselines **recomputed** on the project corpus 2026-09-16 - a full scan of 9,644,687 Hungarian documents (63.42 G letters) plus an independent measurement on the 19,038 gold paragraphs every precision figure in this chapter is scored against; the two agree to within 0.7 points on every axis once the register mix is accounted for.
- **Confidence**: `measured` (baselines and register spread); `tendency` (their use as a damage signal).

### REP-QUAL-002 Double acute letters as a Hungarian cue and a damage cue
- **Statement**: `ő` and `ű` are nearly exclusive to Hungarian, so their presence is strong positive language evidence and their absence in a long Hungarian-looking block is evidence of damage (encoding, font, OCR or accent class B/C).
- **Conditions**: blocks of about 1,200 characters (paragraphs merged).
- **Output**:
  - `has_double_acute` per block; measured: 99.4% of Hungarian blocks contain `ő` or `ű`, against 22% of German and 3% of Russian blocks;
  - a block without `ő ű` but with `õ û ô ı` or class B/C statistics is routed to the matching repair before language classification.
- **Examples**: a German abstract block without `ő ű` goes to the language classifier; an archaic Hungarian poem with `ő` is kept Hungarian even if a classifier says English.
- **Exceptions**: blocks under 150 words cannot be classified (of blocks failing the guard, 150 words and up were 95.1% foreign, 60-149 words 38.5% Hungarian).
- **Predictability**: `tendency`
- **Lexical input**: none
- **Interactions**: REP-QUAL-008; REP-ENC-006; REP-FONT-003.
- **Sources**: [PT-CLEAN cleaning.md "A foreign abstract is dropped by the block"] (99.4% / 22% / 3%; 1,200-character blocks; 150+ words 95.1% foreign among guard failures).
- **Validation**: recomputation on a labelled block sample.
- **Confidence**: `tendency`.

### REP-QUAL-003 Function-word evidence and abstention
- **Statement**: Hungarian is identified from function words scored against rival languages; below 150 words the test abstains (returns no verdict), and a short fragment is judged with its surrounding document as evidence; accent-free variants of Hungarian function words count as Hungarian evidence for stripped text.
- **Conditions**: function-word list (articles, conjunctions, particles, pronouns, auxiliaries, postpositions: `a az egy és s hogy nem is meg van volt lesz de ha már csak még mint mert vagy sem pedig így akkor most itt ott el ki be fel le át ez azt ezt ezek azok ami amely aki amit amikor ahol kell lehet nincs minden sok nagyon után között szerint alatt során való én te ő mi ti ők igen nagy első új számára illetve valamint azonban tehát hiszen`), plus stripped forms (`es mar igy utan kozott ugy tobb szamara`).
- **Output**: `hungarian`, `foreign`, or `None` (abstain); listings (TV schedules, fixture lists) carry few function words and must use oracle acceptance of content words instead.
- **Examples**: `Gsz.: Kálinger (3)` fixture list: function-word test abstains; oracle acceptance of the lowercase content tokens decides.
- **Exceptions**: a test that cannot run must abstain, never convict.
- **Predictability**: `tendency`
- **Lexical input**: function-word list
- **Interactions**: REP-DIAC-001, REP-ENC-006, REP-QUAL-008.
- **Sources**: [PC-HU quality.ron `function_words`]; [PT-CLEAN hungarian.py `MIN_WORDS_FOR_LANGUAGE` 150; cleaning.md "A test that cannot run must abstain"; encoding.py `looks_hungarian` (5 function-word hits)].
- **Validation**: `goldens`; abstention rate report.
- **Confidence**: `consensus`.

### REP-QUAL-004 Mojibake and substitute-glyph counts
- **Statement**: Residual damage is counted by signature family after repair, so a document's remaining encoding and font damage is a measurable column.
- **Conditions**: after all ENC and FONT repairs.
- **Output** (counts):
  - `mojibake_1252`: `Ã Å Â` followed by a continuation image (REP-ENC-002); `mojibake_1250`: `Ă Ĺ` + image (REP-ENC-003); `mojibake_dos`: `├ ┼` + image; `mojibake_mac`: `√ ≈ Ň` + image;
  - `font_gaps`: count of `õ û ô Õ Û Ô` in word tokens (prior art counts `õ û Õ Û`; `ô Ô` added per REP-ENC-009); `gap_known`: lowercase words rejected as written and accepted with the double acute letter (unrepaired but repairable);
  - `wrong_long_vowel_share`: `font_gaps / (font_gaps + count(ő ű Ő Ű))`.
- **Examples**: after repair a measured residue of 0.822% wrong long vowels over 99,008 documents concentrated in 458 documents (listings and pocket damage) motivated word-level repair.
- **Exceptions**: foreign words with legitimate glyphs are excluded by `gap_known`.
- **Predictability**: `rule`
- **Lexical input**: oracle for `gap_known`
- **Interactions**: REP-ENC, REP-FONT.
- **Sources**: [PC-KB quality.rs `Measures` (`mojibake`, `font_gaps`, `gap_known`), `misread`]; [PT-CLEAN cleaning.md "Latin-2 damage survives the repair"].
- **Validation**: `goldens`.
- **Confidence**: `consensus`.

### REP-QUAL-005 Replacement, private-use, control and dotless-i rates
- **Statement**: Lost bytes, undecoded private-use glyphs, control characters and dotless `ı` are counted per document and judged by rate, not by presence.
- **Conditions**: after REP-ENC-012 (lost hyphens removed) and REP-FONT-003 .. 005, 010.
- **Output**:
  - `replacement_rate` = U+FFFD / characters; threshold for `lost_bytes` label: > 1 per 10,000 characters (a presence flag rejected 21,248 documents, 47.2% of them with exactly one U+FFFD and 88.9% with at most five, in documents averaging 27,000 characters; the rate keeps 89.9% of those while still rejecting 47% of the OCR-damaged source);
  - `private_use` count (U+E000..U+F8FF, planes 15-16);
  - `dotless_rate` = (`ı` + `İ`) / words; shifted-font documents **0.028-0.21 with no gap**, and clean **after repair exactly 0** because the map is total (REP-FONT-003). 1.04% of all documents (252,558 of 24.2 M) carry some dotless `ı`, almost all of it one or two quoted Turkish names in healthy Hungarian, so this is a rate to rank on and not a band to threshold;
  - `cjk_density` (> 0.001 of characters marks non-Hungarian material in code and mixed sources).
- **Examples**: 1 U+FFFD in 27,000 characters: below threshold, no label.
- **Exceptions**: -
- **Predictability**: `tendency`
- **Lexical input**: none
- **Interactions**: REP-QUAL-010.
- **Sources**: [PT-CLEAN cleaning.md "Lost bytes are judged by rate"] (21,248 documents, 47.2% with exactly one U+FFFD, 88.9% with <= 5; `max_replacement` 1/10,000); [PT-CLEAN "A Hungarian model's code slice should not teach it Chinese"] (`max_cjk` 1/1,000); [PC-KB quality.rs]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens`; the `dotless_rate` band recomputed on a full scan of 24,183,772 documents.
- **Confidence**: `tendency`; `dotless_rate` `measured`.

### REP-QUAL-006 Letterspacing and fragmentation rates
- **Statement**: Letterspaced runs and syllable fragmentation are measured before and after spacing repair; the residue after repair distinguishes typographic convention (repaired) from wrecked text layers (label).
- **Conditions**: word tokens.
- **Output**: `spaced_runs` = runs of **>= 3** single-letter tokens (counted once per run); `letter_spaced_rate` = runs of >= 3 single-letter tokens / words; `short_word_share` = lowercase tokens of length <= 3 / alphabetic tokens (healthy **0.24**; trigger **0.35** per document of at least 300 words; damaged layers **> 0.55**); residue after REP-SPACE-001/003 labels `letter_spaced` beyond repair.
- **Examples**: `h á z a k` counts one run.
- **Exceptions**: formulas and single-letter variables inflate the counts; exclude math spans.
- **Predictability**: `tendency`
- **Lexical input**: none
- **Interactions**: REP-SPACE-001 .. 003.
- **Sources**: [PC-KB quality.rs `spaced_runs`]; [PT-CLEAN hungarian.py `LETTER_SPACED`; encoding.py `FRAG_TRIGGER`]; [ATT evidence/calibration.md 2026-09-16].
- **Validation**: `goldens`; the run-length and short-word bands are measured (2,373 injected runs; 2,105 documents).
- **Confidence**: `tendency`; the reported bands are `measured`.

### REP-QUAL-007 Analysable share
- **Statement**: The share of lowercase word tokens accepted by the morphological oracle is the most informative single quality signal for Hungarian text: it falls with OCR damage, fragmentation, encoding residue, accent loss and foreign content, and it decides gold eligibility.
- **Conditions**: lowercase alphabetic tokens (capitalised tokens excluded to avoid names and sentence starts); computed once per document in the cleaning pass and reused.
- **Output**: `analysable` = accepted / lowercase tokens; reference points: an OCR-damaged PDF source measured 81.0%; gold thresholds 0.90 (general) and 0.95 (spacing-fragment gold).
- **Examples**: a stripped-accent paragraph can score below 0.5 before DIAC repair and above 0.95 after.
- **Exceptions**: technical vocabulary and neologisms lower the share without damage; compare against the source's own distribution.
- **Predictability**: `tendency`
- **Lexical input**: oracle
- **Interactions**: REP-NOISE-001 (gold gate); REP-OCR-002; REP-QUAL-010.
- **Sources**: [PT-CLEAN cleaning.md] (81.0% analysability for the OCR-damaged source; one computation for all sources); [PT-DAMAGE repair-axes.md] (0.90, 0.95 thresholds); [PC-KB quality.rs `known`, `lower`].
- **Validation**: `goldens`; per-source distribution report.
- **Confidence**: `tendency`.

### REP-QUAL-008 Foreign blocks inside Hungarian documents
- **Statement**: Foreign abstracts, quotations and appendices inside Hungarian documents are detected per block and excluded from Hungarian repairs (and optionally removed for training), using the double-acute guard before any classifier.
- **Conditions**: paragraphs merged into blocks of about 1,200 characters; guard: a block containing `ő` or `ű` is Hungarian without classification; otherwise classify if the block has >= 150 words; shorter unguarded blocks inherit the document verdict.
- **Output**: block label `hu` or language code; Hungarian repairs (DIAC, SPELL, FONT-001 mapping) disabled on foreign blocks.
- **Examples**: the English abstract of a Hungarian thesis is labelled `en`; an archaic Hungarian poem classified as English at confidence 1.0 is kept `hu` by the guard.
- **Exceptions**: stripped Hungarian blocks (no `ő ű`) fall to the classifier and to REP-DIAC-001's stripped-Hungarian signature, which runs first.
- **Predictability**: `tendency`
- **Lexical input**: language classifier
- **Interactions**: REP-QUAL-002, 003; REP-DIAC-001.
- **Sources**: [PT-CLEAN cleaning.md "A foreign abstract is dropped by the block, not by the document"] (9.6%, 9.2%, 7.1% of three academic sources is foreign text inside kept documents).
- **Validation**: labelled block sample.
- **Confidence**: `tendency`.

### REP-QUAL-009 Pre-reform orthography rate
- **Statement**: The rate of archaic markers per word (REP-ARCH-001) is stored as a column and used to exclude archaic text from gold pools and to switch SPELL repairs off.
- **Conditions**: lowercase tokens.
- **Output**: `pre_reform` = marker-weighted count / words; label thresholds 0.01 (strict) and 0.02 (lenient) (prior art: per-source gates of 0.01 and 0.02 on the `cz` rate only).
- **Examples**: a 19th-century newspaper page with `czél`, `hoszszú`: rate well above 0.02.
- **Exceptions**: names are excluded from the count.
- **Predictability**: `tendency`
- **Lexical input**: name lexicon
- **Interactions**: REP-ARCH-001, 009; REP-NOISE-001.
- **Sources**: [PT-CLEAN hungarian.py `pre_reform_rate` (lowercase `cz` words per word); catalog.py `max_pre_reform` 0.01 and 0.02 on two sources]; [PC-HU quality.ron `archaic`].
- **Validation**: dated sample.
- **Confidence**: `tendency`.

### REP-QUAL-010 Measure, label, and decide downstream
- **Statement**: Quality signals are stored as columns and labelled spans; repair stages rewrite only what they can repair, drop only what extraction got wrong (and a better extractor would replace), and leave content judgements (short, foreign, archaic, accent-flat, unanalysable crawl text) to the consumer's thresholds.
- **Conditions**: every document leaving the repair stage.
- **Output**:
  - columns: all REP-QUAL-001 .. 009 measures, repair counts per rule, residue counts;
  - labelled spans: `spaced`, `mojibake`, `replacement`, `private_use`, `foreign`, `boilerplate`, `pre_reform` (a label marks text; it drops nothing);
  - withheld (extraction failure): lost bytes above rate, font-shift residue (mixed fonts), letterspacing beyond repair, and on PDF sources only too-few-words and unanalysable; withheld ids with reasons are written for a re-extraction worklist.
  - three outcomes per repair: repair, keep, flag.
- **Examples**: a Wiktionary one-line gloss is kept with `too_short` as a column, not dropped.
- **Exceptions**: -
- **Predictability**: `rule` (policy)
- **Lexical input**: none
- **Interactions**: all areas.
- **Sources**: [PT-CLEAN cleaning.md "The cleaned stage records the verdict instead of enacting it", "Repair the document, do not discard it"]; [PC-KB labels.rs] (labels mark text; recipes decide exclusion).
- **Validation**: schema check.
- **Confidence**: `consensus`.

### REP-QUAL-011 Pass order
- **Statement**: Repairs run in a fixed order in which each stage's evidence is not destroyed by an earlier stage and each gate judges the repaired text.
- **Conditions**: all sources; a source enables a subset of repairs it was measured to need (an empty measurement means not audited, not clean).
- **Output** (order):
  1. byte decoding and encoding detection (`WS-ENC-011`), NFC (`WS-NORM-001`);
  2. escapes and entities, exactly once (REP-ENC-013);
  3. private-use decoding or dropping (REP-FONT-005); presentation ligatures (REP-FONT-006);
  4. mojibake and code page confusions per run (REP-ENC-002 .. 011, 015), 7-bit conventions (REP-ENC-014);
  5. shifted font (REP-FONT-003); substitute glyphs per word (REP-FONT-001, with REP-ENC-006/007/009 merged); decomposed look-alikes and doubled acute (REP-FONT-002, after NFC);
  6. U+FFFD lost hyphens (REP-ENC-012); controls (REP-FONT-010); invisible format characters (REP-FONT-009); soft hyphen (REP-FONT-008); mathematical alphanumerics (REP-FONT-007); homoglyphs (REP-FONT-011);
  7. typographic spaces (REP-SPACE-006); page furniture and reflow (REP-SPACE-010); line-end hyphens (REP-SPACE-004/005); letterspacing (REP-SPACE-001/002); syllable fragments (REP-SPACE-003); punctuation spacing (REP-SPACE-007/008/011/012);
  8. measures for gates (REP-QUAL-001 .. 009) and block language labels (REP-QUAL-008);
  9. archaic label (REP-ARCH-001); optional modernisation (REP-ARCH-009);
  10. diacritic restoration (REP-DIAC), on Hungarian non-archaic blocks;
  11. OCR character repair (REP-OCR) on OCR sources; keyboard slips (REP-KEY); word fusion/split (REP-SPACE-009);
  12. spelling (REP-SPELL), report-only by default for real-word and usage classes;
  13. final NFC and re-measurement.
- **Examples**: running letterspacing before NBSP normalisation misses runs set with NBSP; running Latin-2 mapping before mojibake repair corrupts `Ã`-sequences; running controls removal before ENC destroys the C1 evidence.
- **Exceptions**: entity decoding is not idempotent: a stage that has already run must not run again on its output.
- **Predictability**: `rule`
- **Lexical input**: source repair declarations
- **Interactions**: all areas.
- **Sources**: [PT-CLEAN encoding.py `ORDER` and its notes] (letterspacing after invisible-space normalisation; doubled acute after NFC; lost hyphen before replacement counting); [PT-CLEAN cleaning.md "Repairs are per source, never universal", "Cleaning runs exactly once"].
- **Validation**: pipeline test on generated multi-damage pairs (REP-NOISE-010); idempotence.
- **Confidence**: `consensus`.

## Reference tables

Tables T-ENC-1 .. T-ENC-6 and T-KEY-1 .. T-KEY-2 were generated on 2026-09-15 from the Python 3 codecs (cross-checked against the Unicode mapping files with zero mismatches), the WHATWG single-byte indexes, the HTML5 named character references and the KBDHU KLC file; invisible and control characters are written `<U+XXXX>`; `\|` in a cell is a literal vertical bar.

### Table T-ENC-1 Hungarian letters in legacy single-byte encodings

| Letter | Unicode | ISO-8859-2 | windows-1250 | CP852 | MacCE | CWI-2 | ISO 646-HU | Windows best fit to windows-1252 |
|---|---|---|---|---|---|---|---|---|
| `á` | U+00E1 | E1 | E1 | A0 | 87 | A0 | 60 | `á` |
| `é` | U+00E9 | E9 | E9 | 82 | 8E | 82 | 7B | `é` |
| `í` | U+00ED | ED | ED | A1 | 92 | A1 | - | `í` |
| `ó` | U+00F3 | F3 | F3 | A2 | 97 | A2 | - | `ó` |
| `ö` | U+00F6 | F6 | F6 | 94 | 9A | 94 | 7C | `ö` |
| `ő` | U+0151 | F5 | F5 | 8B | CE | 93 | - | `o` |
| `ú` | U+00FA | FA | FA | A3 | 9C | A3 | - | `ú` |
| `ü` | U+00FC | FC | FC | 81 | 9F | 81 | 7D | `ü` |
| `ű` | U+0171 | FB | FB | FB | F5 | 96 | - | `u` |
| `Á` | U+00C1 | C1 | C1 | B5 | E7 | 8F | 40 | `Á` |
| `É` | U+00C9 | C9 | C9 | 90 | 83 | 90 | 5B | `É` |
| `Í` | U+00CD | CD | CD | D6 | EA | 8D | - | `Í` |
| `Ó` | U+00D3 | D3 | D3 | E0 | EE | 95 | - | `Ó` |
| `Ö` | U+00D6 | D6 | D6 | 99 | 85 | 99 | 5C | `Ö` |
| `Ő` | U+0150 | D5 | D5 | 8A | CC | A7 | - | `O` |
| `Ú` | U+00DA | DA | DA | E9 | F2 | 97 | - | `Ú` |
| `Ü` | U+00DC | DC | DC | 9A | 86 | 9A | 5D | `Ü` |
| `Ű` | U+0170 | DB | DB | EB | F4 | 98 | - | `U` |

### Table T-ENC-2 UTF-8 bytes of Hungarian letters decoded by a single-byte code page

C1 control characters produced by ISO-8859-1 decoding, or by WHATWG decoding of bytes undefined in the vendor table, are written `<U+00xx>`.

| Letter | UTF-8 bytes | as windows-1252 | as ISO-8859-1 | as windows-1250 | as ISO-8859-2 | as CP852 | as CP437 | as MacCE | as MacRoman |
|---|---|---|---|---|---|---|---|---|---|
| `á` | C3 A1 | `Ã¡` | `Ã¡` | `Ăˇ` | `ĂĄ` | `├í` | `├í` | `√°` | `√°` |
| `é` | C3 A9 | `Ã©` | `Ã©` | `Ă©` | `ĂŠ` | `├ę` | `├⌐` | `√©` | `√©` |
| `í` | C3 AD | `Ã<U+00AD>` | `Ã<U+00AD>` | `Ă<U+00AD>` | `Ă<U+00AD>` | `├ş` | `├¡` | `√≠` | `√≠` |
| `ó` | C3 B3 | `Ã³` | `Ã³` | `Ăł` | `Ăł` | `├│` | `├│` | `√≥` | `√≥` |
| `ö` | C3 B6 | `Ã¶` | `Ã¶` | `Ă¶` | `Ăś` | `├Â` | `├╢` | `√∂` | `√∂` |
| `ő` | C5 91 | `Å‘` | `Å<U+0091>` | `Ĺ‘` | `Ĺ<U+0091>` | `┼Ĺ` | `┼æ` | `ŇĎ` | `≈ë` |
| `ú` | C3 BA | `Ãº` | `Ãº` | `Ăş` | `Ăş` | `├║` | `├║` | `√ļ` | `√∫` |
| `ü` | C3 BC | `Ã¼` | `Ã¼` | `ĂĽ` | `Ăź` | `├╝` | `├╝` | `√ľ` | `√º` |
| `ű` | C5 B1 | `Å±` | `Å±` | `Ĺ±` | `Ĺą` | `┼▒` | `┼▒` | `ŇĪ` | `≈±` |
| `Á` | C3 81 | `Ã<U+0081>` | `Ã<U+0081>` | `Ă<U+0081>` | `Ă<U+0081>` | `├ü` | `├ü` | `√Ā` | `√Å` |
| `É` | C3 89 | `Ã‰` | `Ã<U+0089>` | `Ă‰` | `Ă<U+0089>` | `├ë` | `├ë` | `√Č` | `√â` |
| `Í` | C3 8D | `Ã<U+008D>` | `Ã<U+008D>` | `ĂŤ` | `Ă<U+008D>` | `├Ź` | `├ì` | `√ć` | `√ç` |
| `Ó` | C3 93 | `Ã“` | `Ã<U+0093>` | `Ă“` | `Ă<U+0093>` | `├ô` | `├ô` | `√ď` | `√ì` |
| `Ö` | C3 96 | `Ã–` | `Ã<U+0096>` | `Ă–` | `Ă<U+0096>` | `├ľ` | `├û` | `√Ė` | `√ñ` |
| `Ő` | C5 90 | `Å<U+0090>` | `Å<U+0090>` | `Ĺ<U+0090>` | `Ĺ<U+0090>` | `┼É` | `┼É` | `Ňź` | `≈ê` |
| `Ú` | C3 9A | `Ãš` | `Ã<U+009A>` | `Ăš` | `Ă<U+009A>` | `├Ü` | `├Ü` | `√ö` | `√ö` |
| `Ü` | C3 9C | `Ãœ` | `Ã<U+009C>` | `Ăś` | `Ă<U+009C>` | `├ť` | `├£` | `√ú` | `√ú` |
| `Ű` | C5 B0 | `Å°` | `Å°` | `Ĺ°` | `Ĺ°` | `┼░` | `┼░` | `Ňį` | `≈∞` |
| `„` | E2 80 9E | `â€ž` | `â<U+0080><U+009E>` | `â€ž` | `â<U+0080><U+009E>` | `ÔÇ×` | `ΓÇ₧` | `‚Äě` | `‚Äû` |
| `”` | E2 80 9D | `â€<U+009D>` | `â<U+0080><U+009D>` | `â€ť` | `â<U+0080><U+009D>` | `ÔÇŁ` | `ΓÇ¥` | `‚ÄĚ` | `‚Äù` |
| `’` | E2 80 99 | `â€™` | `â<U+0080><U+0099>` | `â€™` | `â<U+0080><U+0099>` | `ÔÇÖ` | `ΓÇÖ` | `‚Äô` | `‚Äô` |
| `–` | E2 80 93 | `â€“` | `â<U+0080><U+0093>` | `â€“` | `â<U+0080><U+0093>` | `ÔÇô` | `ΓÇô` | `‚Äď` | `‚Äì` |
| `—` | E2 80 94 | `â€”` | `â<U+0080><U+0094>` | `â€”` | `â<U+0080><U+0094>` | `ÔÇö` | `ΓÇö` | `‚ÄĒ` | `‚Äî` |
| `…` | E2 80 A6 | `â€¦` | `â<U+0080>¦` | `â€¦` | `â<U+0080>Ś` | `ÔÇŽ` | `ΓÇª` | `‚Ä¶` | `‚Ä¶` |
| `«` | C2 AB | `Â«` | `Â«` | `Â«` | `ÂŤ` | `┬ź` | `┬½` | `¬ę` | `¬´` |
| `»` | C2 BB | `Â»` | `Â»` | `Â»` | `Âť` | `┬╗` | `┬╗` | `¬Ľ` | `¬ª` |
| `<U+00A0>` | C2 A0 | `Â<U+00A0>` | `Â<U+00A0>` | `Â<U+00A0>` | `Â<U+00A0>` | `┬á` | `┬á` | `¬†` | `¬†` |
| `€` | E2 82 AC | `â‚¬` | `â<U+0082>¬` | `â‚¬` | `â<U+0082>Ź` | `ÔéČ` | `Γé¼` | `‚ā¨` | `‚Ç¨` |
| `°` | C2 B0 | `Â°` | `Â°` | `Â°` | `Â°` | `┬░` | `┬░` | `¬į` | `¬∞` |
| `§` | C2 A7 | `Â§` | `Â§` | `Â§` | `Â§` | `┬ž` | `┬º` | `¬ß` | `¬ß` |

### Table T-ENC-3 Double UTF-8 (UTF-8 -> single-byte -> UTF-8 -> same single-byte)

| Letter | once as windows-1252 | twice as windows-1252 | once as windows-1250 | twice as windows-1250 |
|---|---|---|---|---|
| `á` | `Ã¡` | `ÃƒÂ¡` | `Ăˇ` | `Ä‚Ë‡` |
| `é` | `Ã©` | `ÃƒÂ©` | `Ă©` | `Ä‚Â©` |
| `í` | `Ã<U+00AD>` | `ÃƒÂ<U+00AD>` | `Ă<U+00AD>` | `Ä‚Â<U+00AD>` |
| `ó` | `Ã³` | `ÃƒÂ³` | `Ăł` | `Ä‚Ĺ‚` |
| `ö` | `Ã¶` | `ÃƒÂ¶` | `Ă¶` | `Ä‚Â¶` |
| `ő` | `Å‘` | `Ã…â€˜` | `Ĺ‘` | `Äąâ€<U+0098>` |
| `ú` | `Ãº` | `ÃƒÂº` | `Ăş` | `Ä‚Ĺź` |
| `ü` | `Ã¼` | `ÃƒÂ¼` | `ĂĽ` | `Ä‚Ä˝` |
| `ű` | `Å±` | `Ã…Â±` | `Ĺ±` | `ÄąÂ±` |
| `Á` | `Ã<U+0081>` | `ÃƒÂ<U+0081>` | `Ă<U+0081>` | `Ä‚Â<U+0081>` |
| `É` | `Ã‰` | `Ãƒâ€°` | `Ă‰` | `Ä‚â€°` |
| `Í` | `Ã<U+008D>` | `ÃƒÂ<U+008D>` | `ĂŤ` | `Ä‚Ĺ¤` |
| `Ó` | `Ã“` | `Ãƒâ€œ` | `Ă“` | `Ä‚â€ś` |
| `Ö` | `Ã–` | `Ãƒâ€“` | `Ă–` | `Ä‚â€“` |
| `Ő` | `Å<U+0090>` | `Ã…Â<U+0090>` | `Ĺ<U+0090>` | `ÄąÂ<U+0090>` |
| `Ú` | `Ãš` | `ÃƒÅ¡` | `Ăš` | `Ä‚Ĺˇ` |
| `Ü` | `Ãœ` | `ÃƒÅ“` | `Ăś` | `Ä‚Ĺ›` |
| `Ű` | `Å°` | `Ã…Â°` | `Ĺ°` | `ÄąÂ°` |

### Table T-ENC-4 Single-byte Hungarian text decoded with the wrong single-byte table

| Letter | ISO-8859-2/windows-1250 read as windows-1252 or ISO-8859-1 | CP852 read as CP437 | CP852 read as CP850 | CP852 read as windows-1250 | windows-1250 read as CP852 | MacCE read as windows-1250 | windows-1250 read as MacCE | ISO-8859-2 read as MacRoman | CWI-2 read as CP437 |
|---|---|---|---|---|---|---|---|---|---|
| `á` | `á` | `á` | `á` | `<U+00A0>` | `ß` | `‡` | `Š` | `·` | `á` |
| `é` | `é` | `é` | `é` | `‚` | `Ú` | `Ž` | `ť` | `È` | `é` |
| `í` | `í` | `í` | `í` | `ˇ` | `Ý` | `’` | `Ū` | `Ì` | `í` |
| `ó` | `ó` | `ó` | `ó` | `˘` | `ˇ` | `—` | `ů` | `Û` | `ó` |
| `ö` | `ö` | `ö` | `ö` | `”` | `÷` | `š` | `Ų` | `ˆ` | `ö` |
| `ő` | `õ` | `ï` | `ï` | `‹` | `§` | `Î` | `ű` | `ı` | `ô` |
| `ú` | `ú` | `ú` | `ú` | `Ł` | `˙` | `ś` | `ķ` | `˙` | `ú` |
| `ü` | `ü` | `ü` | `ü` | `<U+0081>` | `Ř` | `ź` | `Ł` | `¸` | `ü` |
| `ű` | `û` | `√` | `¹` | `ű` | `ű` | `ő` | `Ż` | `˚` | `û` |
| `Á` | `Á` | `╡` | `Á` | `µ` | `┴` | `ç` | `Ń` | `¡` | `Å` |
| `É` | `É` | `É` | `É` | `<U+0090>` | `╔` | `<U+0083>` | `…` | `…` | `É` |
| `Í` | `Í` | `╓` | `Í` | `Ö` | `═` | `ę` | `Õ` | `Õ` | `ì` |
| `Ó` | `Ó` | `α` | `Ó` | `ŕ` | `Ë` | `î` | `”` | `”` | `ò` |
| `Ö` | `Ö` | `Ö` | `Ö` | `™` | `Í` | `…` | `÷` | `÷` | `Ö` |
| `Ő` | `Õ` | `è` | `è` | `Š` | `Ň` | `Ě` | `’` | `’` | `º` |
| `Ú` | `Ú` | `Θ` | `Ú` | `é` | `┌` | `ň` | `ŕ` | `⁄` | `ù` |
| `Ü` | `Ü` | `Ü` | `Ü` | `š` | `▄` | `†` | `‹` | `‹` | `Ü` |
| `Ű` | `Û` | `δ` | `Ù` | `ë` | `█` | `ô` | `Ř` | `€` | `ÿ` |

### Table T-ENC-5 Latin-1 letters of other languages read as ISO-8859-2 (inverse damage)

| Written | Read as ISO-8859-2 | Written | Read as ISO-8859-2 |
|---|---|---|---|
| `à` | `ŕ` | `ã` | `ă` |
| `å` | `ĺ` | `æ` | `ć` |
| `è` | `č` | `ê` | `ę` |
| `ì` | `ě` | `ï` | `ď` |
| `ð` | `đ` | `ñ` | `ń` |
| `ò` | `ň` | `õ` | `ő` |
| `ø` | `ř` | `ù` | `ů` |
| `û` | `ű` | `þ` | `ţ` |
| `ÿ` | `˙` | `À` | `Ŕ` |
| `Ã` | `Ă` | `Å` | `Ĺ` |
| `Æ` | `Ć` | `È` | `Č` |
| `Ê` | `Ę` | `Ì` | `Ě` |
| `Ï` | `Ď` | `Ð` | `Đ` |
| `Ñ` | `Ń` | `Ò` | `Ň` |
| `Õ` | `Ő` | `Ø` | `Ř` |
| `Ù` | `Ů` | `Û` | `Ű` |
| `Þ` | `Ţ` | - | - |

### Table T-ENC-6 Escaped and 7-bit spellings of Hungarian letters

| Letter | HTML named (HTML5) | HTML decimal / hex | JSON / JS (Rust: `\u{e1}`) | percent (UTF-8) | quoted-printable (UTF-8) | quoted-printable (ISO-8859-2) | RTF (ansicpg1250) | TeX | flying accent | 123 notation | telegram style |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `á` | `&aacute;` | `&#225;` `&#xE1;` | `"\u00e1"` | `%C3%A1` | `=C3=A1` | `=E1` | `\'e1` | `\'a` | `a'` | `a1` | `aa` |
| `é` | `&eacute;` | `&#233;` `&#xE9;` | `"\u00e9"` | `%C3%A9` | `=C3=A9` | `=E9` | `\'e9` | `\'e` | `e'` | `e1` | `ee` |
| `í` | `&iacute;` | `&#237;` `&#xED;` | `"\u00ed"` | `%C3%AD` | `=C3=AD` | `=ED` | `\'ed` | `\'{\i}` `\'\i` | `i'` | `i1` | `ii` |
| `ó` | `&oacute;` | `&#243;` `&#xF3;` | `"\u00f3"` | `%C3%B3` | `=C3=B3` | `=F3` | `\'f3` | `\'o` | `o'` | `o1` | `oo` |
| `ö` | `&ouml;` | `&#246;` `&#xF6;` | `"\u00f6"` | `%C3%B6` | `=C3=B6` | `=F6` | `\'f6` | `\"o` | `o: or o~` | `o2` | `oe` |
| `ő` | `&odblac;` | `&#337;` `&#x151;` | `"\u0151"` | `%C5%91` | `=C5=91` | `=F5` | `\'f5` | `\H{o}` | `o"` | `o3` | `oeoe` |
| `ú` | `&uacute;` | `&#250;` `&#xFA;` | `"\u00fa"` | `%C3%BA` | `=C3=BA` | `=FA` | `\'fa` | `\'u` | `u'` | `u1` | `uu` |
| `ü` | `&uuml;` | `&#252;` `&#xFC;` | `"\u00fc"` | `%C3%BC` | `=C3=BC` | `=FC` | `\'fc` | `\"u` | `u: or u~` | `u2` | `ue` |
| `ű` | `&udblac;` | `&#369;` `&#x171;` | `"\u0171"` | `%C5%B1` | `=C5=B1` | `=FB` | `\'fb` | `\H{u}` | `u"` | `u3` | `ueue` |
| `Á` | `&Aacute;` | `&#193;` `&#xC1;` | `"\u00c1"` | `%C3%81` | `=C3=81` | `=C1` | `\'c1` | `\'A` | `A'` | `A1` | `Aa` |
| `É` | `&Eacute;` | `&#201;` `&#xC9;` | `"\u00c9"` | `%C3%89` | `=C3=89` | `=C9` | `\'c9` | `\'E` | `E'` | `E1` | `Ee` |
| `Í` | `&Iacute;` | `&#205;` `&#xCD;` | `"\u00cd"` | `%C3%8D` | `=C3=8D` | `=CD` | `\'cd` | `\'I` | `I'` | `I1` | `Ii` |
| `Ó` | `&Oacute;` | `&#211;` `&#xD3;` | `"\u00d3"` | `%C3%93` | `=C3=93` | `=D3` | `\'d3` | `\'O` | `O'` | `O1` | `Oo` |
| `Ö` | `&Ouml;` | `&#214;` `&#xD6;` | `"\u00d6"` | `%C3%96` | `=C3=96` | `=D6` | `\'d6` | `\"O` | `O: or O~` | `O2` | `Oe` |
| `Ő` | `&Odblac;` | `&#336;` `&#x150;` | `"\u0150"` | `%C5%90` | `=C5=90` | `=D5` | `\'d5` | `\H{O}` | `O"` | `O3` | `Oeoe` |
| `Ú` | `&Uacute;` | `&#218;` `&#xDA;` | `"\u00da"` | `%C3%9A` | `=C3=9A` | `=DA` | `\'da` | `\'U` | `U'` | `U1` | `Uu` |
| `Ü` | `&Uuml;` | `&#220;` `&#xDC;` | `"\u00dc"` | `%C3%9C` | `=C3=9C` | `=DC` | `\'dc` | `\"U` | `U: or U~` | `U2` | `Ue` |
| `Ű` | `&Udblac;` | `&#368;` `&#x170;` | `"\u0170"` | `%C5%B0` | `=C5=B0` | `=DB` | `\'db` | `\H{U}` | `U"` | `U3` | `Ueue` |


### Table T-FONT-1 Look-alikes of the double acute letters

| Look-alike | Code point(s) | Stands for | Typical origin | Genuine in |
|---|---|---|---|---|
| `õ` `Õ` | U+00F5, U+00D5 | `ő` `Ő` | ISO-8859-2 read as Latin-1; Latin-1 fonts; `&otilde;` | Estonian, Portuguese, Vietnamese, Guarani, Livonian |
| `ô` `Ô` | U+00F4, U+00D4 | `ő` `Ő` | CWI-2 read as CP437; typed or font substitute; `&ocirc;` | French, Portuguese, Slovak, Vietnamese, Kashubian |
| `û` `Û` | U+00FB, U+00DB | `ű` `Ű` | ISO-8859-2 read as Latin-1; CWI-2 as CP437; `&ucirc;` | French, Welsh |
| `ũ` `Ũ` | U+0169, U+0168 | `ű` `Ű` | font substitute (rare) | Vietnamese, Guarani |
| `ǒ` `ŏ` `ò` `ō` `ȍ` | U+01D2, U+014F, U+00F2, U+014D, U+020D | `ő` | font or input substitution (rare) | Pinyin, Italian, romanised Japanese, Serbo-Croatian phonetics |
| `ǔ` `ŭ` `ù` `ū` `ȕ` | U+01D4, U+016D, U+00F9, U+016B, U+0215 | `ű` | same | Pinyin, Esperanto, Italian, French, Latvian, Lithuanian |
| `ǘ` `ǜ` | U+01D8, U+01DC | `ű` | `ü` + U+0301 composed by NFC | Pinyin |
| `ö` + U+0301 | U+00F6 U+0301 | `ő` | stacked marks, PDF extraction (not composed by NFC) | - |
| `ó` + U+0301, `ő` + U+0301, `ú` + U+0301, `ű` + U+0301 | - | `ő`, `ő`, `ű`, `ű` | doubled acute from PDF extraction | - |
| `o` + U+02DD, `u` + U+02DD, U+02DD + `o` | - | `ő`, `ű` | dead key followed by space, then letter | - |
| `o"` `o''` `o3` `oeoe` | ASCII | `ő` (`o~` is `ö`, not `ő`) | 7-bit conventions (REP-ENC-014) | - |
| `ı` (in shifted documents) | U+0131 | `ő` | shifted font (REP-FONT-003) | Turkish, Azerbaijani |
| `ő` (in shifted documents) | U+0151 | `ű` | shifted font | - |
| `ӧ` `ӳ` `ӱ` | U+04E7, U+04F3, U+04F1 | `ö`, `ű`, `ü` | mixed-script input or OCR | Udmurt, Mari, Khakas, Chuvash (`ӳ`) and other Cyrillic orthographies |
| `?` U+FFFD | - | `ő ű` (and others) | lossy transcoding | - |

### Table T-DIAC-1 Vowel families for accent restoration

| Unaccented | Family (candidates) | Share in running text (NS15, 1.8M tokens) | Estimate from `[ATT]` (bucket-weighted) |
|---|---|---|---|
| `a` | `a` `á` | 70.33 / 29.66 | 71.97 / 28.03 |
| `e` | `e` `é` | 73.40 / 26.59 | 73.75 / 26.25 |
| `i` | `i` `í` | 86.04 / 13.95 | 87.23 / 12.77 |
| `o` | `o` `ó` `ö` `ő` | 55.41 / 14.65 / 15.82 / 14.10 | 57.33 / 14.60 / 15.27 / 12.80 |
| `u` | `u` `ú` `ü` `ű` | 46.96 / 12.72 / 29.98 / 10.32 | 49.63 / 13.39 / 28.11 / 8.87 |
| `ö` (class C) | `ö` `ő` | 52.9 / 47.1 (derived) | 54.4 / 45.6 (derived) |
| `ü` (class C) | `ü` `ű` | 74.4 / 25.6 (derived) | 76.0 / 24.0 (derived) |
| uppercase | same families in uppercase | - | - |
| consonants | never changed | - | - |

### Table T-DIAC-2 Representative ambiguity sets (frequency buckets from `[ATT]`, probed 2026-09-15)

Buckets are half-octave frequency classes (higher is more frequent, `-` unattested). Buckets of unaccented forms that are not words (`egesz` 6, `kez` 17, `hid` 7) measure damaged text in the corpus.

| Unaccented string | Accepted accented forms (bucket) | Type |
|---|---|---|
| `kerek` | `kerek` 14 'round', `kerék` 11 'wheel', `kérek` 12 'I ask' | T1 |
| `szel` | `szel` 12 'slices', `szél` 16 'wind; edge' | T1 |
| `fel` | `fel` 25 (prefix), `fél` 21 'half; fears' | T1 |
| `hal` | `hal` 15 'fish; dies', `hál` 7 'sleeps (archaic)' | T1 |
| `akar` | `akar` 18 'wants', `akár` 22 'even' | T1 |
| `faj` | `faj` 14 'species', `fáj` 14 'hurts' | T1 |
| `var` | `var` 10 'scab', `vár` 18 'waits; castle' | T1 |
| `el` | `el` 27 (prefix), `él` 18 'lives; edge' | T1/T4 |
| `teli` | `teli` 15 'full', `téli` 17 'winter (adj)' | T1 |
| `level` | `level` (rejected), `levél` 16 'letter; leaf' | T1 (`level` 10 is damage) |
| `kor` | `kor` 19 'age', `kór` 13 'disease', `kör` 17 'circle' | T2/T3 |
| `irt` | `irt` 8 'exterminates', `írt` 18 'wrote' | T2 |
| `ut` | `út` 21 'road' (`ut` 10 is damage or abbreviation) | T2 |
| `szor` | `-szor` 11 (suffix), `-ször` 9 (suffix), `szőr` 8 'hair', `szór` 4 'scatters' | T2/T3 |
| `sor` | `sor` 20 'row', `sör` 12 'beer' | T3 |
| `tor` | `tor` 10 'feast', `tör` 16 'breaks', `tőr` 6 'dagger' | T3 |
| `bor` | `bor` 15 'wine', `bőr` 17 'skin' | T3 |
| `ot` | `öt` 20 'five', `őt` 20 'him/her', `ót` 7 (rare) | T3 |
| `ol` | `ól` 6 'sty', `öl` 9 'kills; lap', `ől` 7 (suffix variant) | T3 |
| `fut` | `fut` 13 'runs', `fűt` 5 'heats' | T3 |
| `meg` | `meg` 29 (prefix), `még` 27 'still' | T4 |
| `ra` | `-ra` 20 (detached suffix), `rá` 22 'onto it' | T4 |
| `ne` | `ne` 24 'don't', `né` 12 'Mrs (suffix)' | T4 |
| `gyereket` | `gyereket` 14 'child (acc)', `gyerekét` 8 'his/her child (acc)' | inflectional (REP-DIAC-006) |
| `hajtottak` | `hajtottak` 11 (indef.), `hajtották` 9 (def.) | inflectional |
| `tudtak` | `tudtak` 15, `tudták` 16 | inflectional |
| `irjak` | `írják` 13 'they write it', `írjak` 7 'I should write' | inflectional |
| `termek` | `termek` 8 'halls', `termék` 20 'product' | T1 |
| `gepet` | `gépet` 13 'machine (acc)', `gépét` 6 'its machine (acc)' | inflectional |
| `vagyok` | `vagyok` 21 'I am', `vágyok` 2 'I long', `vágyók` 8 'those who long' | T1 |
| `Marton` | `Marton` (name), `Márton` (name) | T5 |

### Table T-OCR-1 OCR confusions for Hungarian print

Cost is the edit cost used in candidate search (lower is more likely); direction `->` is printed letter to OCR output. Costs are ranked estimates, not measured rates; JiM counts [L22 Table 2] are given where they exist.

| Printed | OCR output | Cost | Notes |
|---|---|---|---|
| `é` | `e` | 0.3 | estimate (accent loss in print OCR) |
| `e` | `é` | 0.4 | JiM: OCR `é` for text `e` 581 times |
| `á` | `a` | 0.4 | |
| `í` | `i` | 0.4 | |
| `í` | `l`, `1` | 0.8 | |
| `ó` | `o` | 0.4 | |
| `ó` | `ö` | 0.6 | |
| `ö` | `o`, `ó` | 0.5 | |
| `ő` | `ö` | 0.3 | double acute read as diaeresis |
| `ő` | `o` | 0.5 | |
| `ő` | `õ`, `ô`, `ó` | 0.5 | double acute read as tilde, circumflex, acute |
| `ő` | `6` | 1.0 | |
| `ú` | `u` | 0.4 | |
| `ü` | `u` | 0.5 | |
| `ü` | `ii` | 0.9 | |
| `ű` | `ü` | 0.3 | |
| `ű` | `u`, `û`, `ú` | 0.5 | |
| `É` `Á` `Ö` `Ő` `Ü` `Ű` | unaccented capital | 0.4 | capitals lose marks at small sizes |
| `m` | `rn` | 0.6 | |
| `rn` | `m` | 0.6 | |
| `d` | `cl` | 0.7 | |
| `h` | `li`, `b` | 0.8 | |
| `m` | `in`, `nn` | 0.8 | |
| `n` | `ri`, `u` | 0.8 | |
| `u` | `n` | 0.8 | |
| `e` | `c` | 0.7 | |
| `c` | `e` | 0.7 | |
| `t` | `f` | 0.8 | |
| `f` | `t` | 0.8 | |
| `g` | `q` | 0.9 | |
| `l` | `1`, `I`, `\|` | 0.6 | |
| `I` | `l`, `1` | 0.6 | |
| `O` `o` | `0` | 0.7 | |
| `S` | `5` | 0.9 | |
| `B` | `8` | 0.9 | |
| `Z` | `2` | 0.9 | |
| (none) | `y` inserted | 1.0 | JiM: extra `y` 11 times |
| `r` | (deleted) | 0.9 | JiM: `r` missing 232 times |
| `v` | `zd` | 1.0 | JiM |
| `üle` | `űié` | 1.0 | JiM: 19 times |
| `ſ` | `f` | 0.4 | pre-1850 prints |
| `fi` `fl` `ff` | ligature glyph or `f i` | 0.5 | |
| `„` | `,,` | 0.2 | very frequent |
| `”` | `''`, `"`, `“` | 0.3 | |
| `–` | `-` | 0.3 | |
| (none) | space inserted | 0.5 | JiM: 3,523 word splits |
| space | (deleted) | 0.9 | JiM: 360 word fusions |


### Table T-KEY-1 Hungarian layout (KBDHU) against US layout, by physical key

Scan code set 1. `OEM_102` (scan 56) exists only on ISO 102/105-key boards; ANSI 101/104-key boards lack it.

| Scan | US base | US shift | HU base | HU shift | HU AltGr |
|---|---|---|---|---|---|
| 29 | `` ` `` | `~` | `0` | `§` | - |
| 02 | `1` | `!` | `1` | `'` | `~` |
| 03 | `2` | `@` | `2` | `"` | `ˇ (dead)` |
| 04 | `3` | `#` | `3` | `+` | `^ (dead)` |
| 05 | `4` | `$` | `4` | `!` | `˘ (dead)` |
| 06 | `5` | `%` | `5` | `%` | `° (dead)` |
| 07 | `6` | `^` | `6` | `/` | `˛ (dead)` |
| 08 | `7` | `&` | `7` | `=` | `` ` `` |
| 09 | `8` | `*` | `8` | `(` | `˙ (dead)` |
| 0A | `9` | `(` | `9` | `)` | `´ (dead)` |
| 0B | `0` | `)` | `ö` | `Ö` | `˝ (dead)` |
| 0C | `-` | `_` | `ü` | `Ü` | `¨ (dead)` |
| 0D | `=` | `+` | `ó` | `Ó` | `¸ (dead)` |
| 10 | `q` | `Q` | `q` | `Q` | `\` |
| 11 | `w` | `W` | `w` | `W` | `\|` |
| 12 | `e` | `E` | `e` | `E` | `Ä` |
| 13 | `r` | `R` | `r` | `R` | - |
| 14 | `t` | `T` | `t` | `T` | - |
| 15 | `y` | `Y` | `z` | `Z` | - |
| 16 | `u` | `U` | `u` | `U` | `€` |
| 17 | `i` | `I` | `i` | `I` | `Í` |
| 18 | `o` | `O` | `o` | `O` | - |
| 19 | `p` | `P` | `p` | `P` | - |
| 1A | `[` | `{` | `ő` | `Ő` | `÷` |
| 1B | `]` | `}` | `ú` | `Ú` | `×` |
| 1E | `a` | `A` | `a` | `A` | `ä` |
| 1F | `s` | `S` | `s` | `S` | `đ` |
| 20 | `d` | `D` | `d` | `D` | `Đ` |
| 21 | `f` | `F` | `f` | `F` | `[` |
| 22 | `g` | `G` | `g` | `G` | `]` |
| 23 | `h` | `H` | `h` | `H` | - |
| 24 | `j` | `J` | `j` | `J` | `í` |
| 25 | `k` | `K` | `k` | `K` | `ł` |
| 26 | `l` | `L` | `l` | `L` | `Ł` |
| 27 | `;` | `:` | `é` | `É` | `$` |
| 28 | `'` | `"` | `á` | `Á` | `ß` |
| 2B | `\` | `\|` | `ű` | `Ű` | `¤` |
| 56 | - | - | `í` | `Í` | `<` |
| 2C | `z` | `Z` | `y` | `Y` | `>` |
| 2D | `x` | `X` | `x` | `X` | `#` |
| 2E | `c` | `C` | `c` | `C` | `&` |
| 2F | `v` | `V` | `v` | `V` | `@` |
| 30 | `b` | `B` | `b` | `B` | `{` |
| 31 | `n` | `N` | `n` | `N` | `}` |
| 32 | `m` | `M` | `m` | `M` | `<` |
| 33 | `,` | `<` | `,` | `?` | `;` |
| 34 | `.` | `>` | `.` | `:` | `>` |
| 35 | `/` | `?` | `-` | `_` | `*` |

### Table T-KEY-2 Physical neighbours of Hungarian letter keys (KBDHU, ISO board)

Key centres: row 0 at x = i, row 1 at 1.5 + i, row 2 at 1.75 + i, row 3 (starting with `OEM_102`) at 1.25 + i; neighbour = same row distance 1, or adjacent row with horizontal distance <= 0.75.

| Key | Neighbours (HU base characters) |
|---|---|
| `ö` | `9` `ü` `o` `p` |
| `ü` | `ö` `ó` `p` `ő` |
| `ó` | `ü` `ő` `ú` |
| `q` | `1` `2` `w` `a` |
| `w` | `2` `3` `q` `e` `a` `s` |
| `e` | `3` `4` `w` `r` `s` `d` |
| `r` | `4` `5` `e` `t` `d` `f` |
| `t` | `5` `6` `r` `z` `f` `g` |
| `z` | `6` `7` `t` `u` `g` `h` |
| `u` | `7` `8` `z` `i` `h` `j` |
| `i` | `8` `9` `u` `o` `j` `k` |
| `o` | `9` `ö` `i` `p` `k` `l` |
| `p` | `ö` `ü` `o` `ő` `l` `é` |
| `ő` | `ü` `ó` `p` `ú` `é` `á` |
| `ú` | `ó` `ő` `á` `ű` |
| `a` | `q` `w` `s` `í` `y` |
| `s` | `w` `e` `a` `d` `y` `x` |
| `d` | `e` `r` `s` `f` `x` `c` |
| `f` | `r` `t` `d` `g` `c` `v` |
| `g` | `t` `z` `f` `h` `v` `b` |
| `h` | `z` `u` `g` `j` `b` `n` |
| `j` | `u` `i` `h` `k` `n` `m` |
| `k` | `i` `o` `j` `l` `m` `,` |
| `l` | `o` `p` `k` `é` `,` `.` |
| `é` | `p` `ő` `l` `á` `.` `-` |
| `á` | `ő` `ú` `é` `ű` `-` |
| `ű` | `ú` `á` |
| `í` | `a` `y` |
| `y` | `a` `s` `í` `x` |
| `x` | `s` `d` `y` `c` |
| `c` | `d` `f` `x` `v` |
| `v` | `f` `g` `c` `b` |
| `b` | `g` `h` `v` `n` |
| `n` | `h` `j` `b` `m` |
| `m` | `j` `k` `n` `,` |


## Open questions
1. Relative frequency of UTF-8 misdecoding as windows-1250 vs windows-1252 in Hungarian corpora is inferred from the Windows ANSI default, not measured (REP-ENC-003); the weights in REP-NOISE-002 should be refitted from signature counts of REP-QUAL-004 on the project corpora.
2. Weights and acceptance margin of the plausibility score (REP-ENC-016) and all thresholds of REP-ENC-014 (7-bit conventions) are proposals without measurement.
3. The document-share threshold 0.20 for Latin-2 damage and the shifted-font thresholds (REP-ENC-006, REP-FONT-003) come from one project's corpora (12,000 web documents; 125 PDFs); they need confirmation on other sources.
4. CWI-2 prevalence in current corpora is unknown; `ô` for `ő` may equally come from typed substitution (REP-ENC-009, REP-FONT-001).
5. No published Hungarian OCR confusion matrix with per-pair rates was found; Table T-OCR-1 costs are ranked estimates anchored on the counts of the JiM parallel corpus [L22 Table 2] (read with the table's column order OCR -> silver) and the inventory of prior generators. The full 8,593-type error analysis of [L22]/[M24] is not public; request or rebuild it from a parallel scan/e-text corpus.
6. Loss-class weights in REP-NOISE-005 and the class detection thresholds in REP-DIAC-002 are not measured on real partially accented text (forums, SMS corpora); a labelled sample is needed.
7. The date at which simplified doubled digraphs (`ssz` for `szsz`) became general practice is given by secondary sources only (1903 school rules, 1922 Academy rules per `WS-DIG-010`; 1832 rules unstable); verify against the 1877, 1901 and 1922 rule texts before dating ARCH labels (REP-ARCH-001, REP-ARCH-004).
8. `szöllő` as a named nonstandard form was not found in the AkH11 or AkH12 text; only the general statement of AkH11 §17 on dialectal pronunciations was verified (REP-SPELL-003).
9. Mobile keyboard long-press inventories and autocorrect behaviour for Hungarian (Gboard, SwiftKey, iOS) were not tabulated (REP-KEY-008); macOS and xkb `hu` layouts differ from KBDHU and are not tabulated (REP-KEY-001).
10. Row-stagger geometry in REP-KEY-002 follows standard ISO boards; laptop boards with shortened left Shift or relocated `í`/`0` keys are not covered.
11. The one-letter-word guard of REP-SPACE-001 adds `ó` and `ő` to the prior-art set `a á e é s`; the effect on join precision is unmeasured.
12. Whether the inessive/illative confusion (REP-SPELL-009) and ikes `-k` forms (REP-SPELL-008) should be repaired or only reported depends on register policy. Nákolás (1sg `-nák`) is detected by REP-SPELL-008 (resolved 2026-09-15).
13. Pre-1954 loanword mapping lexicon (REP-ARCH-006) and the Latinate-to-Hungarian name map have no open source identified.
14. Soft-hyphen placement inside doubled digraphs by word processors (REP-FONT-008) was not observed on real documents; the routing to REP-SPACE-004 is precautionary.

## Sources used
All keys are in [sources.md](sources.md). Reliability notes for this chapter:
- `KOROMPAY18`: publisher sample pages only.
- `PT-*`, `PC-*`: same-owner project material, not independent evidence.

## Terms
See [glossary.md](glossary.md).
