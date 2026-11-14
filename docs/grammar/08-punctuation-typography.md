# 08 Punctuation and typography

Scope: sentence-final marks, commas and every other punctuation mark (punctuation mark: `írásjel`), quotation, dashes and hyphens as punctuation, parentheses, ellipsis, apostrophe, the periods of abbreviations, the written form of numbers, dates and times, spacing and non-breaking spaces, lists, headings, letters and postal addresses. The norm is AkH12 §239-§300 (punctuation, abbreviations, acronyms, numbers, dates) with AkH11 §239-§299 differences noted. Where AkH12 is silent, typographic consensus and public style guidance are given with lower confidence. Spelling of suffixes after digits, abbreviations and acronyms, hyphen use inside words, capitalisation and end-of-line hyphenation belong to `ORT` and are only referenced here; spelled-out numerals and date vocabulary belong to `PND`; sentence segmentation as a pipeline stage belongs to sentence segmentation (out of scope); the character inventory and normalisation belong to `WS-CHAR`; encoding damage belongs to `REP`.

## Conventions used in this chapter

### Detector level
Every rule states in **Conditions** the minimum analysis a lint or generator needs to decide the rule. The levels are cumulative:

| Level | Name | Input needed |
|---|---|---|
| `L0` | characters | code points, regular expressions over the raw string |
| `L1` | tokens | tokenisation, closed word lists given in the rule, sentence boundaries |
| `L2` | morphology | lemma, part of speech, case, possessive, person/number, mood (`picur-morph` analyser) |
| `L3` | clauses | clause segmentation: one predicate per clause (finite verb, or nominal predicate with zero copula), clause type (main, subordinate, embedded), constituent boundaries |
| `L4` | meaning/prosody | speaker intent, pauses, scope; not decidable from form: the rule lists all variants and a lint must not flag any of them |

### Character names
Unicode code points are written `U+XXXX`. Marks used in examples: `„` U+201E, `”` U+201D, `»` U+00BB, `«` U+00AB, `’` U+2019, `–` U+2013 (en dash), `—` U+2014 (em dash), `−` U+2212 (minus), `…` U+2026, NBSP = U+00A0, NNBSP = U+202F (narrow no-break space), THIN = U+2009. In examples, `␣` is not used; NBSP is spelled out in words when it matters.

### Terms
`gondolatjel` (spaced dash, em-dash function) and `nagykötőjel` (unspaced en dash, range/relation function) are the same glyph `–` U+2013 in AkH12; they are distinguished by spacing and function. `kötőjel` is the hyphen `-`.

Out of scope: the comma and semicolon-colon families, which need the
clause segmentation this project does not build, and the list, heading and address families, which
are document layout rather than text quality. What is left is decidable from the character and token
stream, which is what normalising and ranking scraped text needs.

## Area index
| Area | Topic | Rules |
|---|---|---|
| `END` | sentence-final marks, stacking, sentence type, boundary cues | PUN-END-001..013 |
| `QUOT` | quotation marks, direct speech, dialogue dashes, quoted titles | PUN-QUOT-001..017 |
| `DASH` | hyphen, en dash (nagykötőjel, gondolatjel), em dash, minus | PUN-DASH-001..019 |
| `PAREN` | parentheses, brackets, slash | PUN-PAREN-001..011 |
| `ELL` | ellipsis | PUN-ELL-001..007 |
| `APOS` | apostrophe, prime, gloss quotes | PUN-APOS-001..006 |
| `ABBR` | periods of abbreviations, acronyms, symbols; sentence-end coincidence | PUN-ABBR-001..016 |
| `NUMF` | number formatting, units, currency, percent, degrees, ordinals, Roman numerals, phone numbers | PUN-NUMF-001..021 |
| `DATEF` | date and time formats, ranges, day suffixes | PUN-DATEF-001..016 |
| `SPACE` | spacing table, non-breaking and thin spaces | PUN-SPACE-001..007 |

## END: sentence-final marks

### PUN-END-001 Period after a declarative sentence
- **Statement**: A simple declarative sentence (kijelentő mondat) ends with a period (pont).
- **Conditions**:
  - sentence type declarative (no interrogative word or `-e` in the main clause, no imperative or optative marking, no exclamative intent)
  - detector level `L2` for type cues; intent `L4`
- **Output**: `.` attached to the last token, followed by a space or paragraph end.
- **Examples**: `Esik a hó.`; `A vonat késik.`
- **Exceptions**: sentence-final ellipsis replaces the period (PUN-END-005); a sentence ending in a period-abbreviation gets one period only (PUN-END-010).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-END-006, PUN-END-007 decide the mark of compound sentences; PUN-SPACE-001 spacing.
- **Sources**: `[AkH12 §241 a)]`, `[AkH11 §240 a)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-END-002 Question mark after an interrogative sentence
- **Statement**: A simple interrogative sentence (kérdő mondat) ends with a question mark (kérdőjel).
- **Conditions**:
  - main clause contains an interrogative pronoun or adverb in question function (`ki`, `mi`, `hol`, `hová`, `honnan`, `mikor`, `miért`, `hogyan`, `hogy` 'how', `hány`, `mennyi`, `melyik`, `milyen`, `mekkora`, `vajon`), or the interrogative clitic `-e` attached to the finite verb or focus, or a yes-no question marked by intonation only
  - intonation-only yes-no questions (`Szereted a zenét?`) are `L4`; the other cues are `L2` (homographs `ki` verbal prefix, `mi` 'we', `hogy` conjunction must be excluded morphologically)
- **Output**: `?` attached to the last token.
- **Examples**: `Ki jött?`; `Eljössz-e holnap?`; `Szereted a teát?`
- **Exceptions**: question-form exclamations and commands may take `!` (PUN-END-003); indirect questions inside a declarative main clause take the main clause mark (PUN-END-007); an embedded `?` inside a sentence (PUN-END-009).
- **Predictability**: `rule` (given sentence type); `tendency` for intonation-only questions
- **Lexical input**: closed list of interrogative words (15 above, plus inflected forms)
- **Interactions**: PUN-END-004 stacking; PUN-QUOT-004 inside quotations.
- **Sources**: `[AkH12 §241 c)]`, `[AkH11 §240 b)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-END-003 Exclamation mark after exclamative, optative and imperative sentences
- **Statement**: Exclamative (felkiáltó), optative (óhajtó) and imperative (felszólító) simple sentences end with an exclamation mark (felkiáltójel).
- **Conditions**:
  - exclamative: intent `L4`; typical cues `de` + adjective/adverb (`De szép!`), `milyen`/`mekkora`/`mennyire` without question intent, `hogy` + clause as exclamation
  - optative: `bárcsak`, `bár`, `csak`, `legalább` + conditional mood (`L2`)
  - imperative: finite verb in imperative (subjunctive) mood in the main clause, or prohibition words (`tilos`, `ne`) (`L2`)
- **Output**: `!`. Variants:
  - sentences imperative in form but conveying an indirect instruction or task may end with `.` (both accepted)
  - interrogative-form commands and exclamations may end with `!` or `?!` (AkH12 examples show both; AkH11 shows only `!`)
- **Examples**: `De hideg van!`; `Bárcsak itt lennél!`; `Csukd be az ajtót!`; `Nem hallgatsz már el?!`; task text: `Írj három mondatot.` ~ `Írj három mondatot!`
- **Exceptions**: -
- **Predictability**: `rule` for mood-marked imperatives and optatives; `tendency` otherwise
- **Lexical input**: optative particles (4 above)
- **Interactions**: PUN-END-004.
- **Sources**: `[AkH12 §241 d)]`, `[AkH11 §240 c)]`
- **Validation**: `goldens`
- **Confidence**: `normative`; variants `normative` (both forms explicitly accepted)

### PUN-END-004 Stacked sentence-final marks
- **Statement**: Heightened or mixed emotion may be marked by repeating or combining `?` and `!`.
- **Conditions**:
  - sentence-final position
  - writer intent `L4`; register: informal, literary
- **Output**: attested forms `?!`, `??`, `!!!`. The combined order attested in AkH is question mark first (`?!`). The reverse `!?` is not attested in AkH (not flagged as an error; see open questions). No spaces between stacked marks.
- **Examples**: `Ezt komolyan gondolod?!`; `Tényleg??`; `Soha!!!`
- **Exceptions**: formal and technical registers avoid stacking (tendency, no norm).
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-END-002, PUN-END-003.
- **Sources**: `[AkH12 §241 d)]`, `[AkH11 §240 c)]`
- **Validation**: `goldens`
- **Confidence**: `normative` for the attested forms; `contested` for `!?`

### PUN-END-005 Ellipsis as sentence-final mark
- **Statement**: A broken-off or unfinished sentence may end with an ellipsis (három pont), which then replaces the period.
- **Conditions**:
  - sentence is interrupted or intentionally unfinished (`L4`)
- **Output**: `…` (or `...`) attached to the last word; no additional period. `?` or `!` may follow the ellipsis when the sentence type requires it: `…?`, `…!`.
- **Examples**: `Mondd meg neki, hogy…`; `Hát ha tudnád…`; `És akkor…?`
- **Exceptions**: after an abbreviation period see PUN-ELL-005.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-ELL-001..005.
- **Sources**: `[AkH12 §241 b)]`, `[AkH12 §240 b)]`, `[AkH12 §251]` (example with `…?`); `[AkH11 §264]` (ellipsis for unfinished thought and omission, no spacing detail)
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-END-006 Final mark of a coordinate compound sentence
- **Statement**: A coordinate compound sentence (mellérendelő összetett mondat) takes the final mark required by its last clause.
- **Conditions**:
  - two or more coordinated clauses (`L3`)
  - type of the last clause (`L2`/`L4` as in PUN-END-001..003)
- **Output**: mark of the last clause.
- **Examples**: `Fázom, csukd be az ablakot!`; `Már késő, minek indulnánk el?`; `Siess, mert lekésed a vonatot.` (subordinate, see PUN-END-007); `Vártunk rá, de nem jött.`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: -
- **Sources**: `[AkH12 §242]`, `[AkH11 §241]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-END-007 Final mark of a subordinate compound sentence
- **Statement**: A subordinate compound sentence (alárendelő összetett mondat) takes the final mark required by its main clause, not by the subordinate clause.
- **Conditions**:
  - main clause and at least one subordinate clause (`L3`)
  - type of the main clause decides
- **Output**:
  - default: mark of the main clause; indirect questions under a declarative main clause end with `.`: `Nem tudom, ki jön.`; `Kérdezte, eljövök-e.`
  - variant A: when the main clause is only a frame (brief comment or attention getter such as `kérdem`, `mondom`, `javaslom`, `gondolom`, `hallod`), the mark may follow the subordinate clause: `Kérdezem, ez tényleg így volt?` ~ `Kérdezem, ez tényleg így volt.`
  - variant B: when the subordinate clause is in effect a quoted utterance, its mark may be used: `Hiába szóltam, hogy ne nyúljon hozzá!`
- **Examples**: `Tudni szeretném, mi történt.`; `Megmondtad neki, hogy várjon?`; `Csináld úgy, ahogy mondtam!`
- **Exceptions**: variants above are accepted alongside the default; a lint must not flag either.
- **Predictability**: `rule` for the default; variants `L4`
- **Lexical input**: frame verbs for variant A (open class; the listed items are typical)
- **Interactions**: PUN-QUOT-008.
- **Sources**: `[AkH12 §243]`, `[AkH11 §242]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-END-008 Dash after a sentence-final mark to open a new line of thought
- **Statement**: To signal that a partly separate train of thought begins, a spaced dash (gondolatjel) may follow the period, question mark or exclamation mark.
- **Conditions**:
  - after `.`, `?`, `!` inside running text (not at paragraph start)
  - intent `L4`
- **Output**: `. – ` then the next sentence with capital initial.
- **Examples**: `Ezt már megbeszéltük. – Térjünk át a következő kérdésre.`
- **Exceptions**: not to be confused with dialogue dashes (PUN-QUOT-012) or the dash introducing a quoting clause (PUN-QUOT-005).
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-012 spacing; sentence segmentation (out of scope): the dash does not end or start a sentence by itself.
- **Sources**: `[AkH12 §247]`, `[AkH11 §246]`
- **Validation**: `goldens`
- **Confidence**: `normative` (optional device)

### PUN-END-009 Question or exclamation mark inside a sentence
- **Statement**: In emotive, archaic or literary text a `?` or `!` may be inserted inside a sentence; the sentence then continues with a lowercase letter.
- **Conditions**:
  - `?` or `!` followed by space and a lowercase word that continues the same sentence (`L0` pattern `[?!] +[a-záéíóöőúüű]`; a code-point range such as `[à-ű]` also admits uppercase `Ő`, `Ű`); intent `L4`
- **Output**: no capital after the mark; no sentence boundary.
- **Examples**: `Ott állt, jaj! egyedül a sötétben.`; `Elment, képzeld! egy szó nélkül.` (literary)
- **Exceptions**: quotation followed by quoting clause (PUN-QUOT-005) produces the same surface pattern.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-END-013 segmentation cues; `ORT-CAP`.
- **Sources**: `[AkH12 §251]`, `[AkH11 §250]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-END-010 Sentence ending with an abbreviation
- **Statement**: When a sentence ends with an abbreviation written with a period, that period also serves as the sentence-final period; `?` and `!` are added after it.
- **Conditions**:
  - last token is a period-abbreviation (PUN-ABBR-001, list PUN-ABBR-012) (`L1`)
  - sentence type declarative -> single period; interrogative/exclamative -> abbreviation period + `?`/`!`
- **Output**: `stb.` (not `stb..`); question: `stb.?`; inside a closing parenthesis see PUN-PAREN-006.
- **Examples**: `Vettünk almát, körtét, szilvát stb.`; `Ott volt a szerk.?`
- **Exceptions**: parenthesised abbreviation at sentence end keeps its period inside and gets a sentence period outside: `(… stb.).` (PUN-PAREN-006).
- **Predictability**: `rule`
- **Lexical input**: abbreviation list with period flag (PUN-ABBR-012)
- **Interactions**: PUN-ABBR-006; sentence segmentation (out of scope): a period-abbreviation followed by a capitalised word is ambiguous between boundary and non-boundary.
- **Sources**: inferred from `[AkH12 §240 a)]` (period followed directly by another mark) and `[AkH12 §271]`; no explicit AkH point for the declarative case
- **Validation**: `goldens`
- **Confidence**: `consensus`

### PUN-END-011 Sentence type does not change inside quotations
- **Statement**: A quoted sentence keeps its own final mark inside the quotation marks when introduced by a colon; the enclosing sentence adds no second mark (see QUOT for all placements).
- **Conditions**: quotation introduced by colon and closing the sentence (`L1`)
- **Output**: `…mondta: „Holnap jövök.”` (no period after `”`); `…kérdezte: „Jössz?”`
- **Examples**: see PUN-QUOT-004
- **Exceptions**: integrated quotation: sentence mark after closing quote (PUN-QUOT-007).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-QUOT-004, PUN-QUOT-007.
- **Sources**: `[AkH12 §257 a)]`, `[AkH12 §258]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-END-012 Imperative-form sentences addressed as instruction in exercises and forms
- **Statement**: Instructions in forms and exercises that are imperative in form but convey an indirect instruction may end with a period; prohibition sentences take `!`.
- **Conditions**: imperative instruction in running text (`L2`); signage standing alone (layout)
- **Output**: `Töltse ki az űrlapot.` ~ `Töltse ki az űrlapot!`; prohibition `Tilos a dohányzás!`; as a standalone sign without mark `Tilos a dohányzás` (inference: extension of §266 direction signs)
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-END-003 variant.
- **Sources**: `[AkH12 §241 d)]`, `[AkH12 §266]`
- **Validation**: `goldens`
- **Confidence**: `normative` for the period variant and for `!` after prohibitions (§241 d) examples `Tilos az átjárás!`); `consensus` for signs without mark beyond §266 direction signs

### PUN-END-013 Cues that a `.`, `?` or `!` is not a sentence boundary
- **Statement**: A sentence-final-looking mark does not end the sentence in a closed set of contexts; segmenters and capitalisation checks must test them in this order.
- **Conditions** (first match wins; levels `L0`-`L2`):
  1. period inside a URL, e-mail address or file name (no space on either side: `www.mta.hu`) (PUN-SPACE-003)
  2. period after a digit ordinal or date part followed by a noun, lowercase word, suffix hyphen or another date part (`a 4. fejezet`, `1848. március 15-én`, `3.-nak`) (PUN-NUMF-009, PUN-DATEF-001)
  3. period of a period-abbreviation followed by a lowercase word or digit (`kb. húsz`, `Pf. 20`) (PUN-ABBR-001); followed by an uppercase word it is ambiguous (proper name vs new sentence): prefer non-boundary for abbreviations that normally precede names or numbers (`dr.`, `id.`, `ifj.`, `özv.`, `Szt.`, `u.`, `krt.`, `Pf.`, `sz.`, `l.`, `vö.`), else boundary
  4. `?` or `!` followed by a lowercase continuation (PUN-END-009)
  5. `?`/`!`/`.` + closing quote + ` – ` + lowercase quoting clause (PUN-QUOT-005, PUN-QUOT-012)
  6. mark followed by a comma, semicolon or colon (`„Na!”, „Gyere!” és hasonlók`)
  7. mark + closing quote + suffix hyphen (`„Ne már!”-ral`) (PUN-QUOT-011)
  8. period inside parentheses in mid-sentence (`(a sorban a 25.) nyert`)
  9. date + period + spaced dash + next date in date ranges (`2014. április 11. – 2014. október 10.`, PUN-DATEF-007); digit + period + dash + uppercase in datelines (`január 2. – Budapest`) is one unit
  10. ellipsis followed by lowercase (PUN-ELL-006)
- **Output**: boundary / non-boundary decision; capitalisation expectation of the next word (`ORT-CAP`).
- **Examples**: `A 3. helyen végzett.` (one sentence); `Megkérdezte: „Mikor jössz?” – és elment.` (one sentence)
- **Exceptions**: at paragraph end every final mark is a boundary.
- **Predictability**: `rule` for 1, 2, 4-10; `tendency` for 3 with uppercase follower
- **Lexical input**: abbreviation list (PUN-ABBR-012, larger tokenizer lists such as quntoken's ~450 entries as oracle only)
- **Interactions**: sentence segmentation (out of scope) consumes this rule.
- **Sources**: `[AkH12 §240 a)]`, `[AkH12 §251]`, `[AkH12 §257 b)]`, `[AkH12 §273]`; design cues from `[QUNTOKEN snt.qx]` (studied, not copied)
- **Validation**: `goldens`, `oracle:quntoken`
- **Confidence**: `consensus`

## QUOT: quotation marks and direct speech

### PUN-QUOT-001 Quotation mark inventory
- **Statement**: Hungarian uses low-high double quotation marks as the basic pair and inward-pointing guillemets as the inner pair.
- **Conditions**: any quotation (`L0`)
- **Output**:

| Level | Name | Open | Close | Code points |
|---|---|---|---|---|
| 1 | macskaköröm (idézőjel) | `„` | `”` | U+201E, U+201D |
| 2 | lúdláb (belső idézőjel) | `»` | `«` | U+00BB, U+00AB |
| 3 | félidézőjel | `’` | `’` | U+2019, U+2019 |

  - level 2 points inward (`»…«`), the opposite of French/German-Swiss usage
  - level 1 opens at the baseline and closes at the top; the closing glyph is U+201D (not U+201C as in German)
  - level 3 is not in AkH; used in practice for a third nesting level (tendency) and for meaning glosses (PUN-QUOT-016)
- **Examples**: `„Azt mondta: »gyere«”`
- **Exceptions**: plain-text fallback `"…"` is non-standard in edited text but frequent; normalisation PUN-QUOT-015.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `WS-CHAR`; PUN-QUOT-002, -015.
- **Sources**: `[AkH12 §240 j)]`, `[AkH12 §257]`, `[AkH11 §256]`, `[UNICODE]`, `[WP-IDJ]`
- **Validation**: `goldens`
- **Confidence**: `normative` (levels 1-2); `consensus` (level 3)

### PUN-QUOT-002 Nesting of quotation marks
- **Statement**: A quotation inside a `„…”` quotation uses `»…«`; a third level uses `’…’`.
- **Conditions**: nesting depth (`L0` stack)
- **Output**: depth 1 `„ ”`, depth 2 `» «`, depth 3 `’ ’`; adjacent closing marks are written without space: `«”`
- **Examples**: `„Így szólt: »Hívd ide a ’kis herceget’!«”` (depth 3 tendency)
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-QUOT-014.
- **Sources**: `[AkH12 §240 j)]`, `[AkH12 §257]`; level 3 `[WP-IDJ]`
- **Validation**: `goldens`
- **Confidence**: `normative` (2 levels); `consensus` (level 3)

### PUN-QUOT-003 Quotation mark spacing
- **Statement**: The opening quotation mark is preceded by a space and attaches to the following word, digit or mark; the closing quotation mark attaches to the preceding word, digit or mark and is followed by a space unless another attaching mark follows.
- **Conditions**: `L0`
- **Output**: ` „szó` ... `szó” `; `”.`, `”,`, `”!`, `”-ban` without space
- **Examples**: `a „hurkatöltőjéből” ömlik`; `„Kész.”`
- **Exceptions**: opening mark at line/paragraph start has no preceding space; a dash after a closing mark is spaced (`” – mondta`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-SPACE-001; `[LP-HU]` spacing rules.
- **Sources**: `[AkH12 §240 j)]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-QUOT-004 Quotation introduced by a colon
- **Statement**: After a colon, the quotation starts with a capital (unless the original has lowercase there), and the mark required by the quoted sentence stands before the closing quotation mark; no further mark follows it at sentence end; if the quoting sentence continues after the quotation, it continues in lowercase.
- **Conditions**: `:` + opening quote (`L0`)
- **Output**:
  - `…: „Q.”` / `…: „Q?”` / `…: „Q!”`
  - interrupted quoting sentence continues in lowercase after `”`: `A mondását: „Soha ne add fel!” ma is sokan idézik.` (AkH12 attests this with `!`; a mid-sentence quoted `.` is not exemplified)
  - quoted fragment beginning mid-sentence in the original keeps lowercase: `…: „már hullnak a levelek…”`
- **Examples**: `Kölcsey írta: „Ismerni a jót könnyebb.”`; `Megkérdezte: „Mi a neved?”`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-END-011.
- **Sources**: `[AkH12 §257 a)]`, `[AkH11 §256 a)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-QUOT-005 Quotation followed by the quoting clause
- **Statement**: When the quoting clause follows the quotation, it is attached with a spaced dash and starts in lowercase; the quotation keeps a final `?` or `!` but drops a final period.
- **Conditions**: closing quote + ` – ` + lowercase clause containing a verb of saying (`L1`/`L2`)
- **Output**: `„Q” – mondta X.`; `„Q?” – kérdezte X.`; `„Q!” – kiáltotta X.`
- **Examples**: `„Holnap jövök” – ígérte Anna.`; `„Ki az?” – szólt ki valaki.`
- **Exceptions**: `*„Q.” – mondta` is non-standard (§257 b): "a pontot azonban nem"). §257 b) prescribes the dash and does not forbid a comma, so `„Q”, mondta` is outside the rule rather than wrong, and a lint does not flag it.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-QUOT-012 (same pattern in dialogue without quotation marks); PUN-END-013 cue 5.
- **Sources**: `[AkH12 §257 b)]`, `[AkH11 §256 b)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-QUOT-006 Quotation interrupted by the quoting clause
- **Statement**: A quoting clause inserted into a quotation counts as an insertion: it is enclosed in spaced dashes and starts in lowercase; a mark that belongs at the break point of the quoted text is placed right after the second dash; the closing quotation mark follows the final mark of the quotation.
- **Conditions**: `„Q1 – quoting clause – Q2”` (`L1`)
- **Output**:
  - no mark at the break: `„Q1 – írja X – q2.”`
  - comma at the break: `„Q1 – kérdi X –, q2?”`
  - sentence end at the break: `„Q1 – mondta X. – Q2.”` (quoting clause closed by a period, new sentence capitalised)
- **Examples**: `„A nyelv – írja a szerző – élő rendszer.”`; `„Eljössz – kérdezte –, ha hívlak?”`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-QUOT-012 c); PUN-DASH-012.
- **Sources**: `[AkH12 §257 c)]`, `[AkH11 §256 c)]` (includes the sentence-end-at-break example)
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-QUOT-007 Integrated quotation
- **Statement**: A verbatim quotation woven into one's own sentence is put in quotation marks and begins with a lowercase letter if it starts with a common word (even if capitalised in the original); if the sentence ends with the quotation, the sentence-final mark comes after the closing quotation mark, and the quotation's own final period is dropped.
- **Conditions**: quotation is a constituent of the host sentence (no colon, no dash) (`L3`)
- **Output**: `…, amely „q”.`; `A „q” gondolat…`
- **Examples**: `Szerinte a munka „a szabadság útja”.`; `A „rendületlenül” jelszóvá vált.`
- **Exceptions**: a quoted `?` or `!` at the end of an integrated quotation: not regulated (see open questions); proper nouns keep their capitals.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-QUOT-011 suffixes; PUN-END-011.
- **Sources**: `[AkH12 §258]` (AkH12 adds explicitly that the period goes outside even when the original ends in a period), `[AkH11 §257]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-QUOT-008 Indirect quotation and linked direct speech take no quotation marks
- **Statement**: Paraphrased content quotation and linked direct speech introduced by `hogy` are not put in quotation marks.
- **Conditions**: `hogy` + reported content, or paraphrase (`L3`)
- **Output**: `Azt írta, hogy mindenki fizessen tagdíjat.`; `Mondtam neki, hogy nincs olyan, amit ne tennék meg érte.`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-END-007 variant B.
- **Sources**: `[AkH12 §258]`, `[AkH11 §257]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-QUOT-009 Proverbs and sayings with a comment
- **Statement**: Proverbs, sayings, slogans and well-known lines followed or preceded by a comment are not put in quotation marks, but punctuation and capitalisation follow the quotation rules.
- **Conditions**: proverb/saying + comment clause (`L4`)
- **Output**: `Proverb – comment.`; `Proverb? – comment.`
- **Examples**: `Lassan járj, tovább érsz – mondja a közmondás.`; `Ki mit tud? – kérdezzük tréfásan.`
- **Exceptions**: `-`
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-QUOT-005.
- **Sources**: `[AkH12 §260]`, `[AkH11 §259]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-QUOT-010 Quotation marks for irony or special sense
- **Statement**: Words used ironically or with a special connotation may be put in quotation marks inside a sentence.
- **Conditions**: intent `L4`
- **Output**: `„kedves” levél`; sentence-final mark outside: `…„segíteni”!`
- **Examples**: `Megkaptam a „szíves” válaszát.`
- **Exceptions**: AkH11 warned against overuse; AkH12 drops the warning.
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-QUOT-007 placement.
- **Sources**: `[AkH12 §272]`, `[AkH11 §272]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-QUOT-011 Quoted titles and suffixes after a closing quotation mark
- **Statement**: Titles inserted in running text need no marking, but may be put in quotation marks (or set in a different typeface); a suffix attached to a quoted title or quoted word follows the closing quotation mark with a hyphen, in the form required by the last word.
- **Conditions**: title or cited expression in quotation marks + suffix (`L0`/`L2`)
- **Output**: `a „Magyar szólások”-ban`; `a „Ne már!”-ral`; unmarked title: `a Rokonokban`; italic title: suffix attached directly.
- **Examples**: `Olvastad az „Egri csillagok”-at?`; `a „Pál utcai fiúk”-ról`
- **Exceptions**: preferred alternative avoids the hyphen: `az „Egri csillagok” című regényt`.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`, `ORT-ABBR`, `ORT-FOR` (suffix allomorph by pronunciation of the last word).
- **Sources**: `[AkH12 §199]`, `[AkH12 §200 a)]` (`„Az arany ember”-e`), `[AkH12 §273]`, `[AkH11 §273]`; `[QUNTOKEN snt.qx]` (`"Ne már!"-ral` pattern)
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-QUOT-012 Dialogue with dashes
- **Statement**: In fiction, dialogue is marked with dashes instead of quotation marks, and the quotation rules for capitals and punctuation apply. A new speaker's turn opens a new line with `– ` (§259 d)); after an inserted narrator clause the same speaker's speech goes on in the same line after another dash (§259 c): `– Nagyon vártalak már – fogadta a barátját. – Sok a teendőnk.`).
- **Conditions**: dialogue layout (`L1`: line starting with `–`, U+2013, followed by a space)
- **Output**:
  - a) narrator sentence before the speech: narrator sentence ends with `:`; speech on a new line `– Speech.` with no dash after the speech
  - b) speech before the narrator clause: `– Speech – narrator clause.`; `– Speech? – narrator clause.`; `– Speech! – narrator clause.` (final period of the speech dropped, `?`/`!` kept, narrator clause lowercase)
  - c) narrator clause inside the speech: `– Speech1 – narrator clause. – Speech2.` (break at sentence end); `– Speech1 – narrator clause –, speech2.` (comma at the break goes after the second dash); `– Speech1? – narrator clause. – Speech2.`
  - d) dialogue without narrator clauses: every turn on its own line starting with `– `
- **Examples**: `– Nem tudom – felelte halkan.`; `– Jössz? – kérdezte. – Várunk.`; `– Gyere – intett –, és hozd a kulcsot!`
- **Exceptions**: hyphen-minus or em dash as dialogue dash is non-standard (PUN-DASH-015, PUN-DASH-019).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-QUOT-005, -006; PUN-END-013 cue 5; sentence segmentation (out of scope) (a dialogue line with narrator clause is one sentence unit up to the narrator clause's period).
- **Sources**: `[AkH12 §259 a)-d)]`, `[AkH12 §240 g)]`, `[AkH11 §258]`; `[LP-HU]` dash rules
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-QUOT-013 Line-initial dash for list items
- **Statement**: Items of a vertical list may begin with a dash followed by a space.
- **Conditions**: vertical list (`L1` layout)
- **Output**: `– item`
- **Examples**: `– kenyér` newline `– tej`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: -
- **Sources**: `[AkH12 §240 g)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-QUOT-014 Coinciding closing marks
- **Statement**: When an inner and an outer quotation end together, the closing marks are written adjacent without space; the sentence-final mark is placed according to whether the outer quotation is introduced by a colon (inside) or integrated (outside).
- **Conditions**: `«”` sequence (`L0`)
- **Output**: `…: „… »…«.”` (by PUN-QUOT-004) or `… „… »…«”.` (integrated, PUN-QUOT-007); a lint must accept `…: „… »…«”.` as well (see Exceptions)
- **Examples**: `A felirat: „Itt nyugszik »a vándor«.”`
- **Exceptions**: the only AkH12 example of coinciding closing marks (§240 j), colon-introduced) puts the period after both marks (`…: „… »beszél a víz«”.`), contradicting the §257 a) placement; the §257 example `„Szemében »mesterségem« / … még.”` has no coinciding marks.
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-QUOT-004, -007.
- **Sources**: `[AkH12 §240 j)]` (period after both closing marks despite the colon), `[AkH12 §257 a)]` (mark of the quoted sentence inside the closing quote)
- **Validation**: `goldens`
- **Confidence**: `contested` (placement after a colon); `normative` (adjacency without space)

### PUN-QUOT-015 Normalising non-Hungarian quotation characters
- **Statement**: Straight quotes and foreign quotation conventions in Hungarian edited text are normalised to `„…”` (outer) and `»…«` (inner).
- **Conditions** (`L0`):
  - `"…"`, `“…”`, `”…”`, `„…“` (German closing U+201C), `‚…‘`, `,,…''` (typewriter substitutes), `>>…<<`, `«…»` (reversed guillemets)
  - pairing by position: opening = preceded by space/line start/opening bracket; closing = followed by space/punctuation/line end
- **Output**: `„…”`, inner `»…«`; reversed `«…»` -> `»…«`
- **Examples**: `"alma"` -> `„alma”`; `>>belső<<` -> `»belső«`
- **Exceptions**: code, inch marks (`5"`), seconds of arc (PUN-APOS-006).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `REP` (encoding damage); `WS-CHAR`.
- **Sources**: `[AkH12 §240 j)]`; normalisation design `[LP-HU]`; `[DEB-TIPO]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `consensus`

### PUN-QUOT-016 Meaning glosses in single quotation marks
- **Statement**: In linguistic and lexicographic text the meaning of a cited form is given between right single quotation marks.
- **Conditions**: metalinguistic gloss (`L4`, layout)
- **Output**: `form ’gloss’` with U+2019 on both sides
- **Examples**: `ház ’épület’`; `-l ’valamilyennek látszik’`
- **Exceptions**: `-`
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-APOS-006.
- **Sources**: usage throughout `[AkH12]` (e.g. §95); `[WP-IDJ]`; `[DEB-TIPO]`
- **Validation**: `goldens`
- **Confidence**: `consensus`

### PUN-QUOT-017 Verse quoted in running text
- **Statement**: Line breaks of verse quoted in running text are marked by a slash, stanza breaks by a double slash, each surrounded by spaces.
- **Conditions**: verse quotation run in (`L4`)
- **Output**: `line1 / line2 // line3`
- **Examples**: `„Szállj, szállj / kis madár”`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-PAREN-011.
- **Sources**: `[AkH12 §275]`
- **Validation**: `goldens`
- **Confidence**: `normative`

## DASH: hyphen, en dash, em dash, minus

### PUN-DASH-001 Dash-like character inventory
- **Statement**: Hungarian distinguishes the hyphen (kötőjel), the en dash used both as `nagykötőjel` (unspaced) and `gondolatjel` (spaced), and the minus sign; the em dash is not part of the AkH12 system.
- **Conditions**: `L0`
- **Output**:

| Name | Glyph | Code point | Spacing | Function |
|---|---|---|---|---|
| kötőjel (hyphen) | `-` | U+002D (or U+2010) | none | compounds, suffixes after digits/abbreviations, approximation, suspension, line-end break |
| nem törhető kötőjel | `‑` | U+2011 | none | hyphen that must not break (e.g. before a cited suffix `-ság`) |
| feltételes elválasztójel | (invisible) | U+00AD | none | optional line-break point |
| nagykötőjel | `–` | U+2013 | none (spaced for complex members) | from-to, relation of names/peoples |
| gondolatjel | `–` | U+2013 | space on both sides | insertion, afterthought, dialogue, quoting clause |
| kvirtmínusz (em dash) | `—` | U+2014 | - | not used by AkH12; legacy gondolatjel (PUN-DASH-015) |
| mínuszjel | `−` | U+2212 | see PUN-DASH-016 | subtraction, negative numbers |

  - AkH12 states the nagykötőjel has the same length as the gondolatjel
- **Examples**: `egy-egy`; `Budapest–Bécs`; `Jön – ha ráér.`; `−5 °C`
- **Exceptions**: figure dash U+2012 and horizontal bar U+2015 are not used.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `WS-CHAR`; PUN-DASH-002..019.
- **Sources**: `[AkH12 §240 g), k), l)]`, `[AkH12 §275]`, `[UNICODE]`, `[WP-GONDJ]`, `[DEB-TIPO]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DASH-002 Hyphen spacing and suspended compounds
- **Statement**: The hyphen attaches to both neighbours, except in suspended compounds, where it is followed by a space when the shared second member is omitted, and preceded by a space when the shared first member is omitted.
- **Conditions** (`L0`/`L2`):
  - shared second member omitted: `WORD-` followed by `,` or by a space + `és`|`s`|`vagy`|`meg`|`illetve`, ... full compound
  - shared first member omitted: full compound ... (`,`|` és`|` vagy`) ` -WORD`
- **Output**: `tej-, hús- és halipar`; `bel- és külföldi`; `gépgyártó, -szerelő és -javító üzem`; `bortermelő és -értékesítő`
- **Examples**: as above
- **Exceptions**: at line end see PUN-DASH-017.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-HYPH`; PUN-SPACE-001.
- **Sources**: `[AkH12 §240 k)]`, `[AkH12 §263 c)]`, `[AkH11 §262 c)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DASH-003 Functions of the hyphen (index)
- **Statement**: The hyphen is used in exactly the following functions; word-internal ones are specified in `ORT`.
- **Conditions**: `L0` + context
- **Output**:

| Function | Example | Owner |
|---|---|---|
| three identical consonant letters at a boundary | `sakk-kör`, `Széll-lel` | `ORT-HYPH` |
| reduplication, coordinative and twin compounds | `egy-egy`, `süt-főz`, `dimbes-dombos` | `ORT-HYPH` |
| compounds with names, 6:3 rule, multi-member compounds | `Petőfi-szobor`, `csapatzászló-avatás` | `ORT-HYPH` |
| suspended compounds | `tej- és vajtermék` | PUN-DASH-002 |
| numerals above 2000 | `ötezer-tíz` | PUN-NUMF-011 |
| double family names, geographic names | `Kovács-Tóth`, `Holt-Tisza` | ORT-HYPH-011 |
| interrogative clitic `-e` | `tudod-e` | `ORT-HYPH` |
| approximation/alternative between numbers | `5-6 nap`, `egy-két óra` | PUN-DASH-004 |
| distributive repetition | `2-2 alma`, `húsz-húsz forint` | PUN-DASH-004 |
| suffix after digits, symbols, abbreviations, acronyms, quoted titles | `12-t`, `%-os`, `u.-ban`, `MTA-t`, `„…”-ban` | `ORT-NUM`, `ORT-ABBR`, `ORT-FOR`, PUN-QUOT-011 |
| suffix after silent-letter foreign names, multi-word names + derivational suffix | `Glasgow-ban`, `New York-i` | `ORT-NUM`, `ORT-ABBR`, `ORT-FOR` |
| sport score | `24-17` | PUN-NUMF-016 |
| line-end hyphenation | `asz-tal` | `ORT-SPLIT` |
| letter-number identifiers (variant) | `ABC-123`, `B12-vitamin` | PUN-DASH-011 |

- **Examples**: see table
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: see table.
- **Sources**: `[AkH12 §240 k)]`, `[AkH12 §263 a)-i)]`, `[AkH12 §265 a)-d)]`, `[AkH12 §270]`, `[AkH11 §262]`, `[AkH11 §265]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DASH-004 Hyphen for approximation versus en dash for ranges
- **Statement**: Between numbers (in words or digits) a hyphen expresses approximation or alternative ('five or six'), and an en dash expresses a from-to interval with defined endpoints.
- **Conditions**:
  - hyphen: count of entities is uncertain or alternative: `5-6 darab`, `10-12 éves`, `nyolc-tíz napra`, `egy-két ember`; distributive repetition: `2-2`, `húsz-húsz`
  - en dash: interval over an ordered scale with both endpoints included: page/paragraph/class/house-number ranges (followed by ordinal period `10–12.`), years (`1848–49`), dates, times (`9–17 óráig`), route/relations (PUN-DASH-007)
  - detector (`L1`/`L4`): `\d+[-–]\d+\.` (ordinal period) -> en dash; year pairs -> en dash; `\d+[-–]\d+ (darab|ember|fő|nap|éves|perc|óra)` without `-ig`/`-tól` -> hyphen tendency; spelled-out numerals with `-` -> keep hyphen
- **Output**: as above; when meaning is unclear both are accepted.
- **Examples**: `3-4 hét múlva` (approx.); `a 3–4. héten` (interval, ordinals); `1–4. osztály`; `8-10 fok` ~ `8–10 fok` (ambiguous)
- **Exceptions**: `-`
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-007; `[LP-HU]` converts page and year ranges to en dash.
- **Sources**: `[AkH12 §263 i)]`, `[AkH12 §264 c)]`, `[AkH11 §262 i)]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative` (principle); `tendency` (heuristic)

### PUN-DASH-005 En dash between names of peoples, languages, disciplines
- **Statement**: The relation of two or more peoples, languages, subjects or similar is expressed with an unspaced en dash.
- **Conditions**: two or more ethnonyms/language/discipline names in a relation (`L2` lexical class)
- **Output**: `angol–magyar szótár`; `magyar–olasz mérkőzés`; `biológia–kémia szak`; in names `Osztrák–Magyar Monarchia`, `Magyar–Lengyel Baráti Társaság`
- **Examples**: as above
- **Exceptions**: suffixes PUN-DASH-009.
- **Predictability**: `rule`
- **Lexical input**: ethnonym/language/discipline classes
- **Interactions**: `[LP-HU]` ethnonym pair rule.
- **Sources**: `[AkH12 §264 a)]` (AkH12 adds disciplines `szak`), `[AkH11 §263 a)]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-DASH-006 En dash between proper names in a joint relation
- **Statement**: Two or more proper names in a joint relation (co-authors of a work named after them, opposing teams, joint places) are linked with an unspaced en dash.
- **Conditions**: `NAME–NAME` (single-word names) (`L2`)
- **Output**: `Bárczi–Országh-féle szótár`; `Fradi–Újpest rangadó`
- **Examples**: as above
- **Exceptions**: with multi-word names the en dash may also be spaced (optional, PUN-DASH-008): `Rácz Endre – Szemere Gyula` ~ `Rácz Endre–Szemere Gyula`; double family names of one person use a hyphen (ORT-HYPH-011).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-008, -009.
- **Sources**: `[AkH12 §264 b)]`, `[AkH11 §263 b)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DASH-007 En dash for from-to relations
- **Statement**: Words and numbers expressing a from-to relation are linked with an unspaced en dash.
- **Conditions**: route, interval, range (`L4`; cues: place names, numbers with ordinal period, month names, compass words)
- **Output**: `Budapest–Győr`; `Duna–Tisza-csatorna`; `1990–1994`; `a 45–47. oldalon`; `az 5–8. osztály`; `Petőfi utca 12–14.`; `az ötödik–nyolcadik osztály`; `észak–déli irány`; `július–augusztus`
- **Examples**: as above
- **Exceptions**: approximation uses a hyphen (PUN-DASH-004); complex members spaced (PUN-DASH-008).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DATEF-006, -007.
- **Sources**: `[AkH12 §264 c)]`, `[AkH11 §263 c)]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-DASH-008 Spaced en dash between complex members
- **Statement**: When the members linked by a nagykötőjel are multi-word or otherwise complex (multi-word personal names, members already containing an en dash, dates), the en dash may be surrounded by spaces; between two full or nearly full dates the spaced form is prescribed.
- **Conditions**: at least one member contains a space or an internal en dash (`L0`); full-date members (`L1`)
- **Output**:
  - optional (both accepted, spaced form usual): `Kiss Anna – Nagy Béla: Cím` ~ `Kiss Anna–Nagy Béla: Cím`; `Szojuz–24 – Szaljut–5`; `f. hó 5. – f. hó 25.`
  - prescribed: `2023. május 2. – 2024. június 30.`; `2014. I. 15. – VIII. 23.`
- **Examples**: as above
- **Exceptions**: day range inside one month stays unspaced: `2024. május 2–6.` (PUN-DATEF-007).
- **Predictability**: `rule` (dates); `tendency` (other complex members)
- **Lexical input**: `none`
- **Interactions**: PUN-DATEF-007; ORT-HYPH-013.
- **Sources**: `[AkH12 §264 b), c)]` ("may also be fitted without attaching" for complex cases), `[AkH12 §296]` (full dates: the dash attaches on neither side), `[AkH12 §240 l)]` (typographic note: does not attach with complex members), `[AkH11 §263 c)]` ("exceptionally")
- **Validation**: `goldens`
- **Confidence**: `normative`; §240 l) (declarative) and §264 (optional) differ in wording, the optional reading is followed for non-date members

### PUN-DASH-009 Suffixes on en dash constructions
- **Statement**: Suffixes attach to the last member of an en dash construction: directly (no hyphen) when the last member is a word, with a hyphen when it is a digit, Roman numeral or abbreviation; derived `-i` adjectives are written in lowercase throughout.
- **Conditions**: `X–Y` + suffix (`L0` last-member class)
- **Output**: `angol–magyarból`; `Budapest–Bécsen át`; `tavasszal–nyáron`; `budapest–bécsi`; `kelet–nyugati`; `június–júliusi`; `1848–49-ben`; `1914–18-as`; `I–IV.-ben`; `128–9.-en`
- **Examples**: as above
- **Exceptions**: both members may be inflected in adverbial pairs (`tavasszal–nyáron`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`, `ORT-ABBR`, `ORT-FOR`, `ORT-CAP` (lowercase adjectives from place-name pairs).
- **Sources**: `[AkH12 §264 a)-c)]`, `[AkH11 §263]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DASH-010 Shortened second member of numeric ranges
- **Statement**: The second member of a numeric range may omit the leading digits it shares with the first.
- **Conditions**: `N1–N2` with N2 written as the trailing digits of the full number (`L0`)
- **Output**: attested forms: years with two final digits `1848–49`, `2013–14.`; page with one final digit `128–9.`; full forms always correct `1848–1849`
- **Examples**: as above
- **Exceptions**: shortening across a century boundary is not used (`1998–2003` written in full) (consensus).
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-DATEF-006.
- **Sources**: `[AkH12 §240 l)]`, `[AkH12 §264 c)]`, `[AkH12 §296]`
- **Validation**: `goldens`
- **Confidence**: `normative` (attested forms); `consensus` (century boundary)

### PUN-DASH-011 Letter-number identifiers and type designations
- **Statement**: Combinations of letters and digits in identifiers, model and route names may be written in several ways with no substantive difference; suffixes are attached with a hyphen after a final digit.
- **Conditions**: identifier token mixing letters and digits (`L0`)
- **Output**: variants `TU–154`, `Apollo–11`, `Forma–1` (en dash); `ABC-123`, `B12-vitamin` (hyphen); `M7`, `H1N1`, `123456HA` (solid); `M2 metróvonal`, `123456 KL`, `NB I.` (space); suffixes `M7-es`, `TU–154-gyel`
- **Examples**: as above
- **Exceptions**: official systems (licence plates, document numbers) follow their own format.
- **Predictability**: `lexical` (per identifier)
- **Lexical input**: identifier format from its owner
- **Interactions**: AkH11 required en dash for type designations and hyphen before suffixes; `[LP-HU]` still suggests en dash.
- **Sources**: `[AkH12 §294]` (variants; `NB I.` with period, as in `[AkH11 §292]` and ORT-HYPH-016), `[AkH12 §265 d)]` (hyphen before suffixes after digits; §294 itself says nothing about suffixes), `[AkH11 §263 d)]`, `[LP-HU]`
- **Validation**: `goldens`
- **Confidence**: `normative` (free variation since AkH12)

### PUN-DASH-012 Gondolatjel: spaced dash for insertions and afterthoughts
- **Statement**: The gondolatjel is an en dash with a space on both sides; for insertions it is used in pairs; a punctuation mark following the second dash attaches to it without space; a single dash introduces an appended afterthought.
- **Conditions** (`L0` spacing; `L3` pairing):
  - pair: `A1 – insertion – A2`; if the host needs `,` `;` `:` at the insertion point: `– insertion –,`
  - insertion reaching the sentence end: the second dash is not written; the sentence-final mark follows the insertion (inference from the §253 closing-mark logic; no AkH example)
  - single: `Sentence – afterthought.`
- **Output**: as above; a lone unpaired dash inside a sentence that resumes the host clause is an error when its partner is missing (`L3`).
- **Examples**: `A csapat – meglepetésre – nyert.`; `Nem biztos – csak valószínű –, hogy késik.`; `Kemény munka volt – főleg a hőségben.`
- **Exceptions**: dash + other punctuation: no space between them.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-QUOT-005, -006, -012.
- **Sources**: `[AkH12 §240 g)]`, `[AkH12 §249 b)]`, `[AkH12 §251]`-`[AkH12 §253]`, `[AkH11 §250]`-`[AkH11 §252]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-DASH-013 Dash after sentence-final mark
- **Statement**: See PUN-END-008; the dash is spaced and follows `.`, `?`, `!` after one space.
- **Conditions**: `[.?!] – ` (`L0`)
- **Output**: `. – `
- **Examples**: `Vége. – Most jön a java.`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-END-008.
- **Sources**: `[AkH12 §247]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DASH-014 Gondolatjel next to numbers
- **Statement**: A gondolatjel is spaced also when adjacent to digits, which distinguishes it from the unspaced range dash.
- **Conditions**: `\d – ` vs `\d–\d` (`L0`)
- **Output**: `Az eredmény 12 – nem rossz.` (insertion/afterthought) vs `12–15` (range)
- **Examples**: as above
- **Exceptions**: spaced range dash between complex date members (PUN-DASH-008).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-007, -008.
- **Sources**: `[AkH12 §240 g)]`, `[AkH12 §240 l)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DASH-015 Em dash as legacy or variant gondolatjel
- **Statement**: The em dash `—` (U+2014) is not part of the AkH12 system; older Hungarian printing and some style sheets used the em dash for the gondolatjel; modern Hungarian typesetting uses the en dash for both functions.
- **Conditions**: `—` in Hungarian text (`L0`)
- **Output**: normalise `—` with or without surrounding spaces to ` – ` (spaced en dash) in gondolatjel function; unspaced `—` between numbers to `–`
- **Examples**: `Jön—ha ráér.` -> `Jön – ha ráér.`
- **Exceptions**: reproducing the typography of a historical source.
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-019; `REP`.
- **Sources**: `[WP-GONDJ]` (modern practice uses only the en dash; older printings used em width); `[DEB-TIPO]` (still recommends em dash for gondolatjel); `[AkH12 §240 l)]` (equal length of both dashes); `[LP-HU]` (normalises `—` to `–`)
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `contested`

### PUN-DASH-016 Minus and plus signs
- **Statement**: The minus sign `−` (U+2212) marks subtraction and negative values; as a sign of a single number it attaches to the digits; binary operators are spaced in typeset text, though AkH examples are inconsistent.
- **Conditions** (`L0`):
  - unary: `[−+±]\d` attached: `−5`, `+12 °C`, `±0,5`
  - binary: `a − b`, `2 + 2 = 4` (typographic consensus); AkH examples `4−2`, `(2+3)`, `2+2 = 4`, `ő + ő = ők`
- **Output**: prefer U+2212 over hyphen-minus for negative numbers and subtraction in typeset text; hyphen-minus accepted in plain text. The equals sign takes a space on both sides; the unspaced `2+3` in AkH12 §275 is an example, not a prescription, so unspaced arithmetic operators are not errors `[ENYELV 2022-07-14]`, `[ENYELV 2025-10-16]` (`OQ-08-016` resolved 2026-09-16).
- **Examples**: `−3 fok`; `10 − 4 = 6`
- **Exceptions**: suffix after a negative number: `−5-tel` (hyphen, `ORT-NUM`, `ORT-ABBR`, `ORT-FOR`).
- **Predictability**: `tendency` (spacing); `rule` (sign attached)
- **Lexical input**: `none`
- **Interactions**: PUN-NUMF-004; `[LP-HU]` minus normalisation.
- **Sources**: `[AkH12 §275]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `consensus`; AkH examples `contested` for operator spacing

### PUN-DASH-017 Hyphen and en dash at line end
- **Statement**: When a hyphenated word is broken at its hyphen, the hyphen stays at the end of the line and is not repeated at the start of the next line, except optionally in specialist works; an en dash at a line end also serves as the line-break mark.
- **Conditions**: line break at `-` or `–` (layout)
- **Output**: `tűzzel-` / `vassal`; `magyar–` / `angol`; specialist variant `nátrium-` / `-klorid`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-SPLIT`.
- **Sources**: `[AkH12 §238]`, `[AkH12 §240 k), l)]`, `[AkH12 §270]`, `[AkH11 §238]`, `[AkH11 §270]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DASH-018 En dash constructions versus hyphenated compounds with names
- **Statement**: A relation between two names (co-authors, route, match) takes an en dash; a compound formed from a name and a common noun or a double family name takes a hyphen.
- **Conditions**: `NAME1?NAME2` (`L2`: are both proper names in relation -> en dash; one person's double name -> hyphen)
- **Output**: `Szabó–Kovács szótár` (two authors) vs `Szabó-Kovács Anna` (one person); `Petőfi-szobor` (name + noun, hyphen)
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule` given referents; `L4` otherwise
- **Lexical input**: person entity knowledge
- **Interactions**: PUN-DASH-006; `ORT-CAP`.
- **Sources**: `[AkH12 §263 e), f)]`, `[AkH12 §264 b)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DASH-019 Normalising hyphen-minus used as a dash
- **Statement**: A hyphen-minus used in dash functions is replaced by the en dash with the spacing of its function.
- **Conditions** (`L0`, first match):
  1. line start `- ` or `-Word` in dialogue/list -> `– `
  2. ` - ` (spaced) -> ` – `
  3. year pair `\b1\d{3}-\d{2,4}\b`/`\b20\d{2}-\d{2,4}\b` -> en dash
  4. page range after `o.`, `oldal`, `pp.`, `l.` -> en dash
  5. house number range after `utca|u.|út|tér|krt.` -> en dash
  6. `–` with spaces in a simple single-token range (`12 – 15`) -> `12–15`
- **Output**: as above
- **Examples**: `1990-2000` -> `1990–2000`; `jön - ha ráér` -> `jön – ha ráér`
- **Exceptions**: approximation `5-6` (PUN-DASH-004); ISO date `2024-01-05` (PUN-DATEF-004); identifiers (PUN-DASH-011); compound hyphens.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-004, -015.
- **Sources**: `[AkH12 §240 g), l)]`, `[AkH12 §264 c)]`; normalisation design `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `consensus`

## PAREN: parentheses, brackets, slash

### PUN-PAREN-001 Bracket types and nesting
- **Statement**: Round parentheses are the default bracket; square, angle, curly brackets and slashes serve mostly to mark a second level inside parentheses or special functions.
- **Conditions**: `L0`
- **Output**:

| Name | Glyphs | Code points | Use |
|---|---|---|---|
| kerek zárójel | `( )` | U+0028/U+0029 | default insertion, remarks, labels `a)` |
| szögletes zárójel | `[ ]` | U+005B/U+005D | editorial insertion, gloss `[= …]`, omission `[…]`, inner level |
| csúcsos zárójel | `⟨ ⟩` | U+27E8/U+27E9 (AkH online text shows U+3008/U+3009) | inner level, special notation |
| kapcsos zárójel | `{ }` | U+007B/U+007D | inner level (rare) |
| ferde zárójel | `/ /` | U+002F | special notation (e.g. phonemic) |

  - nesting default (tendency): `( … [ … ] … )`
- **Examples**: `(lásd [1] alatt)`
- **Exceptions**: mathematics uses its own nesting order.
- **Predictability**: `rule` (types); `tendency` (nesting order)
- **Lexical input**: `none`
- **Interactions**: PUN-PAREN-003.
- **Sources**: `[AkH12 §240 i)]`, `[UNICODE]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-PAREN-002 Parenthesis spacing
- **Statement**: The opening parenthesis is preceded by a space and attaches to what follows; the closing parenthesis attaches to what precedes and to a following punctuation mark (except a dash), and is followed by a space before a word; inside a word (alternative reading) both attach.
- **Conditions**: `L0`
- **Output**: ` (szó)`, `(szó),`, `(szó).`, `(szó) – `, `írásjel(ek)`, `Hara(n)g`
- **Examples**: `Kérte (és sürgette), hogy jöjjön.`; `tanuló(k)`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-SPACE-001; `[LP-HU]` spacing rules.
- **Sources**: `[AkH12 §240 i)]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-PAREN-003 Square brackets for editorial material
- **Statement**: Square brackets mark words not in the original text, explanations or synonymous rewordings inside quotations, and omissions from quoted text.
- **Conditions**: quotation or edited source (`L4`)
- **Output**: `[ti. …]`, `[= …]`, `[…]`
- **Examples**: `„Jobb ott maradni… [ti. otthon]”`; `megcsókolván [= miután megcsókolta]`; `„Előre […] a néppel”`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-ELL-004.
- **Sources**: `[AkH12 §240 b), i)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-PAREN-004 Closing parenthesis after list labels
- **Statement**: Letter and number labels of enumerations take a closing parenthesis only.
- **Conditions**: label at item start or in cross-reference (`L0`)
- **Output**: `a)`, `b)`, `1)`, `α)`, `γ)`; not `(a)` in AkH style
- **Examples**: `a) pont`; `[vö. 244. d)]`
- **Exceptions**: `(a)` full parentheses are common in legal and international styles (tendency, not AkH).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: -
- **Sources**: `[AkH12 §240 i)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-PAREN-005 Punctuation at the insertion point goes after the closing parenthesis
- **Statement**: If a parenthesised insertion stands where the host sentence needs a punctuation mark, that mark is written after the closing parenthesis; the insertion itself carries only its own `?`, `!` or abbreviation period.
- **Conditions**: `( … )` followed by a host mark (`L0`)
- **Output**: `…(insertion), …`; `…(insertion): …`; `…(valljuk be!) …`; `…(kb. 20 fő), …`
- **Examples**: `Felvették a jelentkezők felét (negyven főt), a többit elutasították.`; `Egy dolog zavarta (és ezt nem titkolta): a zaj.`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-012.
- **Sources**: `[AkH12 §252]`, `[AkH12 §253]`, `[AkH11 §252]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-PAREN-006 Parenthesis at sentence end
- **Statement**: A parenthesised part that belongs to the sentence and stands at its end is followed by the sentence-final mark after the closing parenthesis; an abbreviation period at the end of the parenthesised part is kept inside, and the sentence-final mark is still written outside.
- **Conditions**: sentence ends with `)` and the parenthesised part is not an independent sentence (`L3`)
- **Output**: `…(példa).`; `…(6-kor)!`; `…(autók, buszok stb.).`
- **Examples**: `Egyes szavak ige és főnév is lehetnek (pl. les, nyom).`; `Ne felejtsd el (holnap reggel)!`
- **Exceptions**: independent sentence in parentheses (PUN-PAREN-007).
- **Predictability**: `rule`
- **Lexical input**: abbreviation list (PUN-ABBR-012)
- **Interactions**: PUN-END-010.
- **Sources**: `[AkH12 §253]`, `[AkH12 §271]`, `[AkH11 §252]`, `[AkH11 §271]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-PAREN-007 Independent sentence in parentheses
- **Statement**: A complete independent sentence placed in parentheses after another sentence starts with a capital and carries its own final mark inside the parentheses; the preceding sentence is closed normally.
- **Conditions**: `. (` + capitalised sentence + `[.?!])` (`L0`/`L3`)
- **Output**: `Sentence1. (Sentence2.)`
- **Examples**: `Mindenki egyetértett. (A többiek el sem jöttek.)`
- **Exceptions**: see PUN-PAREN-008.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-PAREN-006; sentence segmentation (out of scope).
- **Sources**: `[AkH12 §253]`, `[AkH11 §252]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-PAREN-008 Parenthesised question or exclamation followed by a period
- **Statement**: AkH12 also shows a parenthesised question or exclamation followed by a period outside the parentheses, with no space between the closing parenthesis and the period.
- **Conditions**: `(… ?).` or `(… !).` (`L0`)
- **Output**: `(Ki tudja, miért?).`; `(Kár érte!).`
- **Examples**: as above
- **Exceptions**: conflicts with PUN-PAREN-007 when the parenthesised part is a full sentence after a closed sentence. Editorial practice does not write two closing marks at a sentence end, and a parenthetical inserted into a sentence begins lowercase and keeps its own mark except a period `[ENYELV 2022-01-27]`, `[ENYELV 2019-09-19]`. Prefer one closing mark; the AkH12 example is an inconsistency in the source (`OQ-08-005` resolved 2026-09-16).
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-PAREN-007.
- **Sources**: `[AkH12 §240 c), d)]` (spacing examples)
- **Validation**: `goldens`
- **Confidence**: `contested`

### PUN-PAREN-009 Year in parentheses without period
- **Statement**: A year standing alone in parentheses may be written without the period.
- **Conditions**: `(\d{4})` (`L0`)
- **Output**: `(1526)` ~ `(1526.)`; preferred without period (tendency)
- **Examples**: `A mohácsi csata (1526) sorsdöntő volt.`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DATEF-005.
- **Sources**: `[AkH12 §297 c)]`, `[AkH11 §295]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-PAREN-010 Alternatives for marking insertions
- **Statement**: Parenthetical words, phrases, appositives and inserted clauses may be marked by paired commas, paired dashes or parentheses; the three are graded from weakest to strongest separation.
- **Conditions**: insertion (`L3`), choice `L4`
- **Output**: enum `{comma-pair, dash-pair, parentheses}`; mixed pairs (`, … –`, `( … ,`) are errors.
- **Examples**: `István, sajnos, nem jön.` ~ `István – sajnos – nem jön.` ~ `István (sajnos) nem jön.`
- **Exceptions**: `-`
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-012.
- **Sources**: `[AkH12 §249 b)]`, `[AkH12 §251]`, `[AkH12 §252]`
- **Validation**: `goldens`
- **Confidence**: `normative` (alternatives); `consensus` (grading)

### PUN-PAREN-011 Slash
- **Statement**: The slash (ferde vonal) marks fractions, alternatives, verse line breaks, school years and two-year spans, and sub-numbers in addresses and class names; its spacing depends on the function.
- **Conditions**: `L0` + context
- **Output**:

| Function | Form | Spacing |
|---|---|---|
| fraction | `1/2`, `3/4-es` | none |
| alternative | `és/vagy`, `kérő/kérelmező` | none |
| per unit | `km/h`, `Ft/kg` | none |
| year span | `2013/14.`, `1944/45` | none |
| house/door/class sub-number | `28/A`, `4/b`, `10/B`, `IV/3-as` | none |
| file/document number | `3434/1992. évi` | none |
| verse line break | `sor / sor` | spaces |
| stanza break | `sor // sor` | spaces |

- **Examples**: see table
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-QUOT-017; PUN-DATEF-006.
- **Sources**: `[AkH12 §275]`, `[AkH12 §292]`, `[AkH12 §294]`, `[AkH12 §296]`
- **Validation**: `goldens`
- **Confidence**: `normative`

## ELL: ellipsis

### PUN-ELL-001 Ellipsis character
- **Statement**: The ellipsis (három pont) is written as three periods, typographically as the single character `…` (U+2026); two periods or four or more periods are not ellipsis forms.
- **Conditions**: `L0`: `\.{2}` not part of `\.{3}`; `\.{4,}`
- **Output**: `...` -> `…` (typeset); `..` -> `.` or `…` (ask); `....` -> `…` + `.` only when an abbreviation period precedes (PUN-ELL-005), else `…`
- **Examples**: `Hát…`
- **Exceptions**: leader dots in tables of contents.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `WS-CHAR`; `[LP-HU]` ellipsis rules.
- **Sources**: `[AkH12 §240 b)]`, `[DEB-TIPO]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `consensus`

### PUN-ELL-002 Ellipsis after the interrupted text
- **Statement**: An ellipsis marking an unfinished thought or an omission after a word attaches to that word without space and is followed by a space (or by an attaching mark).
- **Conditions**: `word…` (`L0`)
- **Output**: `hogy… `; `hogy…?`; `hogy…”`
- **Examples**: `Nem tudom, hogy…`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-SPACE-001.
- **Sources**: `[AkH12 §240 b)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ELL-003 Ellipsis for an omitted sentence beginning
- **Statement**: When the beginning of a sentence is omitted, the ellipsis is preceded by a space (after the previous sentence's last character) and attaches to the first word of the truncated text.
- **Conditions**: `…word` at sentence start (`L0`)
- **Output**: `Vége. …s akkor elindultunk.`
- **Examples**: `„…és rendezni végre közös dolgainkat”`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-END-013 cue 10 (continuation lowercase).
- **Sources**: `[AkH12 §240 b)]`, `[AkH11 §264]` (example only)
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ELL-004 Bracketed ellipsis in quotations
- **Statement**: An omission from a quoted text is marked by an ellipsis in square brackets, distinguishing it from an ellipsis of one's own text; it is spaced like a word.
- **Conditions**: inside a quotation (`L1`)
- **Output**: ` […] `; attached to a following mark: `[…],`
- **Examples**: `„Előre hát […], a néppel tűzön-vízen át”`
- **Exceptions**: `(…)` round brackets are a non-AkH variant (tendency).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-PAREN-003; `[LP-HU]` `[...]` -> `[…]`.
- **Sources**: `[AkH12 §240 b)]` (new in AkH12; AkH11 §264 used bare three dots for omissions)
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-ELL-005 Ellipsis combined with other marks
- **Statement**: A sentence-final ellipsis takes no period; `?` and `!` follow it without space; a comma may follow it; a closing quotation mark or parenthesis attaches to it.
- **Conditions**: `…` + mark (`L0`)
- **Output**: `…?`, `…!`, `…,`, `…”`, `…)`; `*….` (ellipsis + period) is avoided except when a period-abbreviation precedes the ellipsis, where the abbreviation period is kept: `stb.…` (consensus)
- **Examples**: `És aztán…?`; `„Várj…”`
- **Exceptions**: `-`
- **Predictability**: `rule` (no period); `tendency` (abbreviation case)
- **Lexical input**: abbreviation list
- **Interactions**: PUN-END-005.
- **Sources**: `[AkH12 §241 b)]`, `[AkH12 §251]` (`…?` example); abbreviation case: consensus
- **Validation**: `goldens`
- **Confidence**: `normative` (except abbreviation case `consensus`)

### PUN-ELL-006 Continuation after a mid-sentence ellipsis
- **Statement**: If the sentence continues after an ellipsis (hesitation, pause), it continues in lowercase after a space; a capital after an ellipsis starts a new sentence.
- **Conditions**: `… ` + next word (`L0`)
- **Output**: `De hát… nem is tudom.` (one sentence); `Várt… Aztán elment.` (two sentences)
- **Examples**: as above
- **Exceptions**: proper nouns.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-END-013 cue 10; `ORT-CAP`.
- **Sources**: `[AkH11 §264]` (literary example with lowercase continuation); `[AkH12 §146]` (capital after sentence end)
- **Validation**: `goldens`
- **Confidence**: `consensus`

### PUN-ELL-007 No spaces inside the ellipsis
- **Statement**: The three periods of an ellipsis are not spaced apart from each other in Hungarian text.
- **Conditions**: `. . .` (`L0`)
- **Output**: `. . .` -> `…`
- **Examples**: `Hát . . .` -> `Hát…`
- **Exceptions**: reproducing a source.
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-ELL-001.
- **Sources**: `[AkH12 §240 b)]` (examples use the single glyph); `[DEB-TIPO]`
- **Validation**: `goldens`
- **Confidence**: `consensus`

## APOS: apostrophe

### PUN-APOS-001 Apostrophe character
- **Statement**: The apostrophe (hiányjel, aposztróf) is `’` (U+2019); the straight `'` (U+0027) is a plain-text substitute; the acute accent `´` (U+00B4) and grave `` ` `` are not apostrophes.
- **Conditions**: `L0`
- **Output**: `'` and `´` between letters or before two digits -> `’`
- **Examples**: `hál'` -> `hál’`; `´95` -> `’95`
- **Exceptions**: code, primes (PUN-APOS-006).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `WS-CHAR`; `[LP-HU]` apostrophe rules.
- **Sources**: `[AkH12 §275]`, `[UNICODE]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative` (glyph); `consensus` (substitutes)

### PUN-APOS-002 Apostrophe for omitted sounds
- **Statement**: The apostrophe marks omitted sounds or letters, mainly at the end of a word in colloquial, dialectal or poetic forms and fixed phrases.
- **Conditions**: truncated word (`L4`); fixed phrase `hál’ istennek`
- **Output**: apostrophe attached to the truncated word at the side of the omission; space after when the word ends.
- **Examples**: `hál’ istennek`; `mer’` (for `mert`); `oszt’` (for `aztán`, dialectal)
- **Exceptions**: standard spelling never uses apostrophes for elision in ordinary words.
- **Predictability**: `lexical`
- **Lexical input**: fixed phrases (`hál’ istennek`)
- **Interactions**: `-`
- **Sources**: `[AkH12 §275]`, `[AkH11 §275]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-APOS-003 Apostrophe in shortened years
- **Statement**: A year shortened to its last two digits is written with a leading apostrophe; suffixes follow with a hyphen.
- **Conditions**: `’\d{2}` (`L0`)
- **Output**: `’95` (= 1995); `’56-os`; `a ’90-es évek`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`, `ORT-ABBR`, `ORT-FOR`; PUN-DATEF-014.
- **Sources**: `[AkH12 §275]`; suffix pattern `[LP-HU]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-APOS-004 Apostrophes in foreign proper names are kept
- **Statement**: Apostrophes belonging to the original spelling of foreign names are kept.
- **Conditions**: foreign name with apostrophe (`L2` proper noun)
- **Output**: `O’Neill`, `d’Artagnan`, `D’Alembert`
- **Examples**: as above
- **Exceptions**: the apostrophe does not affect suffixation; only the final letter does, by the ordinary `[AkH12 §217]` test. Corpus `[ATT]`: names ending in a pronounced `l`, `n`, `r` take the suffix directly in 88.8% of hits (4,726 direct vs 596 hyphenated: `O’Nealtől`, `O’Connorról`, `O’Nealt`), while names ending in a silent or unusual letter take the hyphen in 59.0% (1,098 hyphenated vs 764 direct: `O’Toole-nak`, `O’Malley-vel`, `O’Kelley-t`). Never add a hyphen merely because the name contains an apostrophe (`OQ-08-010` resolved 2026-09-16).
- **Predictability**: `rule` (suffixation); `lexical` (spelling)
- **Lexical input**: name spelling
- **Interactions**: `ORT-NUM`, `ORT-ABBR`, `ORT-FOR`, `[AkH12 §217]`.
- **Sources**: `[AkH12 §214]` (Latin-script foreign names keep their original spelling, name particles such as `d’` listed); individual apostrophe names: consensus
- **Validation**: `goldens`
- **Confidence**: `consensus`

### PUN-APOS-005 No apostrophe before suffixes
- **Statement**: Suffixes are never attached with an apostrophe; after digits, symbols, abbreviations and acronyms the hyphen is used, elsewhere suffixes attach directly.
- **Conditions**: `’` or `'` between a stem and a Hungarian suffix (`L2`)
- **Output**: `*CD’k` -> `CD-k`; `*2020’ban` -> `2020-ban`; `*Péter’t` -> `Pétert`; a name's own apostrophe is kept and the suffix follows with a hyphen: `McDonald’s-ban` (PUN-APOS-004)
- **Examples**: as above
- **Exceptions**: names containing an apostrophe (PUN-APOS-004).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`, `ORT-ABBR`, `ORT-FOR`; `REP` (English-influenced errors).
- **Sources**: `[AkH12 §265]`, `[AkH12 §282]`, `[AkH12 §288]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-APOS-006 Apostrophe versus prime and gloss quote
- **Statement**: Minutes and seconds of arc, feet and inches use the prime `′` (U+2032) and double prime `″` (U+2033), not the apostrophe or quotation marks; meaning glosses use `’…’` (PUN-QUOT-016).
- **Conditions**: digit + `'`/`"` in coordinates or measurements (`L0`)
- **Output**: `47° 13′ 5″`
- **Examples**: `12′`; `30″`
- **Exceptions**: plain-text substitutes `'` `"` accepted outside typeset text.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-NUMF-006.
- **Sources**: `[AkH12 §279 b)]`, `[UNICODE]`
- **Validation**: `goldens`
- **Confidence**: `normative`

## ABBR: punctuation of abbreviations, symbols and acronyms

Form, case, suffixation and the full list of abbreviations are specified in `ORT-ABBR`; this area fixes periods, spaces and the interaction with sentence punctuation.

### PUN-ABBR-001 Period after most abbreviations
- **Statement**: Most abbreviations (rövidítés) of words and names end with a period.
- **Conditions**: token is a clipped or contracted written form read out as the full word (`L1` lexicon)
- **Output**: `aug.`, `du.`, `kb.`, `özv.`, `Bp.`, `dec.`, `Salg.`, `ált.`, `gk.`, `rkp.`, `Szfv.`
- **Examples**: as above
- **Exceptions**: PUN-ABBR-004 (no-period classes); PUN-ABBR-005 (acronyms).
- **Predictability**: `lexical`
- **Lexical input**: abbreviation list with period flag (`ORT-ABBR`; AkH12 §284: 125 entries, 111 with period)
- **Interactions**: PUN-END-010; PUN-END-013 cue 3.
- **Sources**: `[AkH12 §267 b)]`, `[AkH12 §278]`, `[AkH11 §267 b)]`, `[AkH11 §278]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ABBR-002 Abbreviations of multi-word expressions
- **Statement**: The abbreviation of an expression written as separate words usually has one period-closed element per word, separated by spaces; a number of common ones are written as one element.
- **Conditions**: abbreviation expanding to N separately written words (`L1`)
- **Output**:
  - one element per word: `a. m.` (annyi mint), `i. sz.`, `i. e.`, `Kr. e.`, `Kr. u.`, `s. k.`, `r. k.`, `m. v.`, `é. n.`, `f. hó`, `f. év`, `a. n.`, `(P. H.)`
  - fused although the expansion is several words: `stb.` (s a többi), `vö.` (vesd össze), `NB.` (nota bene)
  - not multi-word: `pl.`, `kb.`, `ún.`, `tkp.`, `ti.`, `ui.`, `ua.`, `uo.`, `úm.`, `l.`/`ld.` abbreviate single words (PUN-ABBR-001)
  - spaces between elements should be no-break (PUN-SPACE-004)
- **Examples**: as above
- **Exceptions**: AkH11 abbreviations removed or replaced in AkH12: `B. ú. é. k.` (AkH12 treats `BÚÉK` as acronym), `N. N.` and `X. Y.` (placeholders for an unknown name)
- **Predictability**: `lexical`
- **Lexical input**: abbreviation list
- **Interactions**: PUN-SPACE-004; `ORT-ABBR`.
- **Sources**: `[AkH12 §277 a)]`, `[AkH12 §284]`, `[AkH11 §277 a)]`, `[AkH11 §282]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ABBR-003 Case of abbreviations at sentence start and in all-caps text
- **Statement**: A lowercase abbreviation at the start of a sentence is capitalised; standardised symbols keep their form even in all-caps text.
- **Conditions**: sentence-initial position; all-caps context (`L0`)
- **Output**: `Kb. húszan jöttek.`; `Csüt. indulunk.`; `ÁRA: 150 Ft`; `TERÜLET: 100 m²`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: symbol list (units, currency)
- **Interactions**: `ORT-ABBR`, `ORT-CAP`.
- **Sources**: `[AkH12 §277 b)]`, `[AkH11 §277 b)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ABBR-004 Abbreviations and symbols without period
- **Statement**: No period follows unit symbols, currency symbols and codes, compass point abbreviations, international vehicle codes, chemical element symbols, physical quantity symbols, mathematical function names, and abbreviations whose end is a complete compound member; periods may also be dropped from abbreviations used massively in dictionaries if listed separately.
- **Conditions** (`L1` class lexicons):

| Class | Examples |
|---|---|
| SI and other unit symbols | `m`, `km`, `cm`, `kg`, `dkg`~`dag`, `l`~`L`, `dl`, `min`, `h`, `kWh`, `GHz`, `kB`, `MB`, `μm`, `kΩ`, `°C`, `m²`, `km/h` |
| currency | `Ft`, `Kr`, `Fr`, `$`, `€`, `£`, `¥`, `HUF`, `EUR`, `USD`, `GBP` |
| compass points | `É`, `D`, `K`, `Ny`, `ÉK`, `ÉNy`, `DK`, `DNy` |
| vehicle country codes | `H`, `A`, `D` |
| chemical symbols | `S`, `Cl`, `Na`, `O` |
| physical quantities | `a`, `v`, `m`, `t` |
| mathematics | `log`, `lg`, `sin`, `cos` |
| ending in a full compound member | `uő`, `Bpest`, `Bföldvár`, `Szfvár` |
| other listed in AkH12 §284 | `db`, `ffi`, `vki`, `vmi` |
| dictionary mass use (optional) | `nm`, `rég`, `ol` |

- **Output**: no period; `Ft.`, `db.` in running text are errors (except as sentence-final period).
- **Examples**: `2500 Ft`; `10 db`; `ÉK-i szél`
- **Exceptions**: AkH11 §278 text gave `uaz` (ugyanaz) as a no-period example while its §282 list has `ua.`; AkH12 has only `ua.`. AkH11 had `f` (fillér) and `Ft`-based `E Ft`, `M Ft`, `Mrd Ft` in the §282 list and `Rbl` (rubel) in §278; AkH12 has none of them. AkH12 §280 renders the pound sign as `₤` (U+20A4 LIRA SIGN); the pound sign is `£` (U+00A3).
- **Predictability**: `lexical`
- **Lexical input**: class lexicons above (`ORT-ABBR`)
- **Interactions**: PUN-NUMF-007, -008; `[LP-HU]` flags `db.`/`Ft.` before lowercase word or `,;:`.
- **Sources**: `[AkH12 §279 a)]`, `[AkH12 §280]`, `[AkH12 §281]`, `[AkH12 §284]`, `[AkH11 §278]`, `[AkH11 §282]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-ABBR-005 No periods in acronyms
- **Statement**: Acronyms (mozaikszó: letter acronyms and blends) take no period between or after their elements.
- **Conditions**: acronym token (`L1`)
- **Output**: `MTI`, `ENSZ`, `NATO`, `tbc`, `tv`, `Mahart`, `áfa`, `BÚÉK`; `*M.T.I.` -> `MTI`
- **Examples**: as above
- **Exceptions**: foreign dotted abbreviations inside foreign names or quotations keep their source form (tendency).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-ABBR`.
- **Sources**: `[AkH12 §287]`, `[AkH11 §285]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ABBR-006 Abbreviation period at sentence end
- **Statement**: A period-abbreviation at sentence end is not followed by a second period; see PUN-END-010.
- **Conditions**: `ABBR.` + end of sentence (`L1`)
- **Output**: `…stb.`; `…stb.?`; parenthesis case `(… stb.).`
- **Examples**: `Megjött a szerk.`; `Ez is benne van, ld. a 3. old.`
- **Exceptions**: PUN-PAREN-006.
- **Predictability**: `rule`
- **Lexical input**: abbreviation list
- **Interactions**: PUN-END-010.
- **Sources**: `[AkH12 §240 a)]`, `[AkH12 §271]`
- **Validation**: `goldens`
- **Confidence**: `consensus`

### PUN-ABBR-007 Abbreviation period followed by another mark
- **Statement**: After an abbreviation period, a comma, semicolon, colon, closing parenthesis, closing quotation mark, `?` or `!` follows without space.
- **Conditions**: `ABBR.` + mark (`L0`)
- **Output**: `stb.,`; `pl.)`; `ún.”`
- **Examples**: `alma, körte stb., valamint…`
- **Exceptions**: `pl.:` (abbreviation plus colon) is acceptable and current: the pattern occurs in 1,142 advisory answers `[ENYELV usage 2026-09-16]`. It is redundant only in the sense that the abbreviation already introduces examples; do not flag it (`OQ-08-008` resolved).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-SPACE-001.
- **Sources**: `[AkH12 §240 a)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ABBR-008 Period kept before a suffix or compound hyphen
- **Statement**: An abbreviation keeps its period before the hyphen that attaches a suffix or a compound member.
- **Conditions**: `ABBR.` + `-` + suffix or word (`L0`)
- **Output**: `u.-ban`, `ker.-nek`, `Bp.-en`, `fszla.-kivonat`; no-period items: `Ft-ot`, `mm-beosztás`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-ABBR` (suffix form by pronunciation).
- **Sources**: `[AkH12 §265 d)]`, `[AkH12 §282]`, `[AkH12 §283]`, `[AkH11 §280]`, `[AkH11 §281]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ABBR-009 Titles and name prefixes before names
- **Statement**: `dr.`, `id.`, `ifj.`, `özv.`, `prof.`, `gr.`, `br.` before a name are lowercase inside a sentence and capitalised only in highlighted positions (signature, address, list, visiting card) and at sentence start; the period is always written.
- **Conditions**: title abbreviation + personal name (`L2`); position (`L1` layout)
- **Output**: `Ott volt dr. Kiss Anna is.`; address line `Dr. Kiss Anna`; `ifj. Nagy Béla`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: title abbreviation list
- **Interactions**: `ORT-CAP`; PUN-SPACE-004 (no-break space after the title).
- **Sources**: `[AkH12 §151]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ABBR-010 Initials of personal names
- **Statement**: An initial standing for a given name is a capital letter followed by a period; consecutive initials are separated by spaces; a digraph initial keeps both letters.
- **Conditions**: name initial (`L2`)
- **Output**: `Kiss J. Péter`; `J. P. Morgan`; `Sz. Kovács Anna`; `Gy. Tóth`; `Zs. Nagy`
- **Examples**: as above
- **Exceptions**: initials without periods in foreign names follow the source (`JFK` as acronym).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-SPACE-004 (no-break); `WS-DIG` (digraph initials).
- **Sources**: consensus; digraph capital form `[AkH12 §277 b)]` examples principle (case kept); no dedicated AkH point
- **Validation**: `goldens`
- **Confidence**: `consensus`

### PUN-ABBR-011 Unit and currency symbols: attachment
- **Statement**: Prefix and base unit symbol are written together without space or mark; compound units use `/` or juxtaposition without spaces; area and volume use superscript exponents.
- **Conditions**: unit symbol (`L1`)
- **Output**: `km`, `kWh`, `km/h`, `m²`, `cm³`; plain-text `m2` -> `m²` in typeset text (tendency)
- **Examples**: `120 km/h`; `10 kWh`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: SI prefix and unit lists
- **Interactions**: PUN-NUMF-007; `[LP-HU]` superscript suggestion.
- **Sources**: `[AkH12 §279 a), b)]`, `[BIPM-SI]`, `[LP-HU]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ABBR-012 No-period items of the AkH12 standard list
- **Statement**: Of the 125 items in the AkH12 standard abbreviation list, exactly 14 are written without period; all others end in a period; the full list is in `ORT-ABBR`.
- **Conditions**: lookup (`L1`)
- **Output**:

| Form | Expansion |
|---|---|
| `D` | dél |
| `db` | darab |
| `DK` | délkelet |
| `DNy` | délnyugat |
| `É` | észak |
| `ÉK` | északkelet |
| `ÉNy` | északnyugat |
| `ffi` | férfi |
| `Ft` | forint |
| `K` | kelet |
| `Ny` | nyugat |
| `uő` | ugyanő |
| `vki` | valaki |
| `vmi` | valami |

  - `(P. H.)` (pecsét helye) is listed with parentheses
  - homographs with period: `hg.` (hegység, herceg), `min.` (minimum, miniszter), `m.` (magyar, megye), `sz.` (szám, század), `üi.` (ügyintéző, ügyirat)
- **Examples**: `Ui.` (utóirat) vs `ui.` (ugyanis)
- **Exceptions**: `-`
- **Predictability**: `lexical`
- **Lexical input**: AkH12 §284 list (125 entries)
- **Interactions**: PUN-END-013 cue 3; PUN-ABBR-004.
- **Sources**: `[AkH12 §284]`, `[AkH11 §282]` (different list including unit symbols)
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ABBR-013 Month and weekday abbreviations
- **Statement**: Month abbreviations end in a period and have fixed forms; weekday abbreviations are not fixed by AkH, several one- and two-letter forms are acceptable and carry no period, and a document must use one series consistently.
- **Conditions**: date context (`L1`)
- **Output**: months: `jan.`, `febr.`, `márc.`, `ápr.`, `máj.`, `jún.`, `júl.`, `aug.`, `szept.`, `okt.`, `nov.`, `dec.`; weekday `csüt.` (the only AkH12 form); accepted weekday series without period `[ENYELV 2026-01-20]`: `H`/`Hé`, `K`/`Ke`, `Sz`/`Sze`, `Cs`/`Csüt`, `P`/`Pé`, `Sz`/`Szo`, `V`/`Va`/`Vas`, one series per document
- **Examples**: `2024. febr. 5.`
- **Exceptions**: `feb.`, `szep.`, `márc` without period are non-standard (tendency).
- **Predictability**: `lexical`
- **Lexical input**: 12 month forms; weekday series (3 variants x 7 days)
- **Interactions**: PUN-DATEF-001, -010.
- **Sources**: `[AkH12 §284]`, `[AkH12 §295]`, `[ENYELV 2026-01-20]`
- **Validation**: `goldens`
- **Confidence**: `normative` (months, `csüt.`); `consensus` (weekday series, advisory precedent; `OQ-08-014` resolved 2026-09-16)

### PUN-ABBR-014 Article before abbreviations and acronyms
- **Statement**: The definite article before an abbreviation follows the pronounced full form; before an acronym it follows the pronounced acronym.
- **Conditions**: article + abbreviation/acronym (`L1`)
- **Output**: `az igh.`, `az i. m.`, `a szerk.`, `a f. k.`; `az MTA` [em-té-á], `a MÁV`, `az áfa`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: expansion/pronunciation of each item
- **Interactions**: `PND-ART`, `ORT-ABBR`.
- **Sources**: `[AkH12 §276]`, `[AkH11 §276]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ABBR-015 `stb.` versus `etc.`
- **Statement**: The Hungarian abbreviation for 'and so on' is `stb.`; `etc.` is foreign; `stb` without period is an error except as a word in dictionary lists.
- **Conditions**: `L0`
- **Output**: `stb.`
- **Examples**: `alma, körte stb.`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: -
- **Sources**: `[AkH12 §277 a)]`, `[AkH12 §284]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-ABBR-016 Street and address abbreviations
- **Statement**: In addresses `u.` (utca), `krt.` (körút), `em.` (emelet), `fszt.` (földszint), `mfszt.` (magasföldszint), `Pf.` (postafiók) are written with a period; `út`, `tér`, `sétány` are not abbreviated in postal addresses.
- **Conditions**: address context (`L1`)
- **Output**: `Petőfi u. 8.`; `Teréz krt. 2.`; `II. em. 3.`; `Pf. 20`
- **Examples**: as above
- **Exceptions**: postal element abbreviations additionally used by Magyar Posta: `hrsz.`, `ép.`, `lph.`
- **Predictability**: `lexical`
- **Lexical input**: list above
- **Interactions**: -
- **Sources**: `[AkH12 §284]`, `[POSTA26 §2.8]`
- **Validation**: `goldens`
- **Confidence**: `normative`

## NUMF: number formatting

### PUN-NUMF-001 Digits or words
- **Statement**: There is no strict rule; literary running text prefers words for numbers expressible by a short word and for numbers inside suffixed forms, compounds and postpositional phrases; digits are preferred for long numbers and for times, amounts of money, measurements and statistics; round numbers above 999 may combine digits and a word.
- **Conditions**: register and number length (`L4`)
- **Output**: `öt`, `húsz`, `ezer`; `huszonötöt`, `tízféle`, `hatvan után`; `388 személy`, `du. 5 órakor`, `2890 Ft`, `15 méter`; `107 ezer`, `6 millió`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`, `PND-CARD`.
- **Sources**: `[AkH12 §290]`, `[AkH11 §288]`
- **Validation**: `goldens`
- **Confidence**: `normative` (as a tendency statement)

### PUN-NUMF-002 Digit grouping
- **Statement**: Numbers of five or more digits are grouped in threes counted from the right, separated by a space (typographically a no-break space, preferably narrow) or, as a variant, a period; four-digit numbers are grouped only when aligned in a column with longer ones.
- **Conditions** (`L0`):
  - integer part with >=5 digits
  - not a year, postal code, PO box number, telephone number, identifier, account number, page or serial number
- **Output**: `23 816` (U+202F or U+00A0 preferred; U+0020 accepted) ~ `23.816`; `5 602 164`; `2500` (not grouped); column: `4 210` aligned with `21 126`; decimal part not grouped: `20 611,413`
- **Examples**: `12 500 Ft`; `1 000 000`
- **Exceptions**: the period variant collides with ordinal and date periods and should not be generated (tendency); AkH11 allowed only the space.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`; PUN-SPACE-005; `[LP-HU]` narrow no-break space suggestion.
- **Sources**: `[AkH12 §274]`, `[AkH12 §291 b)]`, `[AkH11 §274]`, `[AkH11 §289 b)]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`; separator code point `consensus`

### PUN-NUMF-003 Decimal comma
- **Statement**: The decimal separator is a comma without spaces; a decimal point is non-standard in Hungarian text.
- **Conditions**: decimal fraction (`L0`); `\d+\.\d+` is ambiguous with times (`10.35`), ordinals + digits and period-grouped thousands
- **Output**: `3,16`; `37,5`; `0,5 l`; `−2,5 °C`
- **Examples**: as above
- **Exceptions**: programming code, version numbers (`v1.3`), IP addresses; `\d{1,2}\.\d{2}` after `-kor`-like context is a time (PUN-DATEF-012).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `[LP-HU]` decimal point rules.
- **Sources**: `[AkH12 §274]`, `[AkH12 §293]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-NUMF-004 Signs of numbers
- **Statement**: Sign characters attach to the number: minus `−` (U+2212), plus `+`, plus-minus `±`.
- **Conditions**: signed number (`L0`)
- **Output**: `−15 °C`; `+3,2%`; `±0,5 mm`
- **Examples**: as above
- **Exceptions**: hyphen-minus accepted in plain text.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-016.
- **Sources**: `[AkH12 §275]`, `[LP-HU]`
- **Validation**: `goldens`
- **Confidence**: `consensus`

### PUN-NUMF-005 Percent and per mille
- **Statement**: The percent sign attaches to the number without space; suffixes attach with a hyphen in the pronounced form; per mille follows the same pattern.
- **Conditions**: `\d+ ?%` (`L0`)
- **Output**: `48%`; `12,5%`; `10%-os`; `6%-kal`; `3‰`
- **Examples**: `48 %` -> `48%`
- **Exceptions**: SI/ISO conventions require a space before `%`; the Hungarian norm overrides them in Hungarian text. Percentage point is written out (`2 százalékpont`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-ABBR` (spoken form `százalék`); `[LP-HU]` removes the space.
- **Sources**: `[AkH12 §275]`, `[AkH12 §265 d)]`, `[WP-SZAZ]`, `[ENY-SZAZ]`, `[ENEKES]`, `[BIPM-SI]` (contrasting convention)
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-NUMF-006 Degrees
- **Statement**: The degree sign attaches to the number for angles; for temperatures the unit symbol `°C` is separated from the number by a space; geographic coordinates separate degree, minute and second groups by spaces.
- **Conditions** (`L0`):
  - angle: `\d+°` not followed by `C`/`F`
  - temperature: `\d+ °C`
  - coordinates: `\d+° \d+′ \d+″`
- **Output**: `45°`, `90°`; `23 °C`; `47° 13′ 5″`; suffixes `3°-kal`, `100 °C-on`
- **Examples**: `23°C` -> `23 °C`; `23 C°` -> `23 °C`
- **Exceptions**: the masculine ordinal indicator `º` (U+00BA), which appears in the online AkH12 text, is a look-alike and must be normalised to `°` (U+00B0).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-APOS-006; `WS-CHAR`; `[LP-HU]` no-break space before `°C`.
- **Sources**: `[AkH12 §275]`, `[AkH12 §279 b), c)]`, `[AkH12 §282]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-NUMF-007 Number and unit
- **Statement**: A unit symbol is separated from the preceding number by a space (typographically no-break) and has no period; in running text units may be written out.
- **Conditions**: number + unit symbol (`L1`)
- **Output**: `100 km`, `1 kg`, `10 kWh`, `120 km/h`, `220 W`, `15 méter`
- **Examples**: `5kg` -> `5 kg`
- **Exceptions**: angle degree (PUN-NUMF-006); percent (PUN-NUMF-005).
- **Predictability**: `rule`
- **Lexical input**: unit symbol list
- **Interactions**: PUN-SPACE-004; `ORT-ABBR`.
- **Sources**: `[AkH12 §279 c)]`, `[AkH11 §279]` (refers to decree)
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-NUMF-008 Amounts of money
- **Statement**: Currency units follow the rules of units: the amount comes first, then a space and the currency symbol, abbreviation or name; three-letter ISO codes are used in finance.
- **Conditions**: number + currency (`L1`)
- **Output**: `2890 Ft`; `12 500 Ft`; `25 €` ~ `25 euró`; `10 $` ~ `10 dollár`; `HUF 5000` ~ `5000 HUF` (finance, both orders in use); `3 millió Ft`
- **Examples**: `Ft2890` -> `2890 Ft`
- **Exceptions**: prefixed symbols (`$10`, `€25`) are foreign conventions (tendency: avoid in Hungarian text); AkH12 dropped AkH11's `E Ft`, `M Ft`, `Mrd Ft` from the list but they remain in business use (tendency).
- **Predictability**: `rule` (order and space for `Ft`); `tendency` (foreign symbols and ISO codes)
- **Lexical input**: currency list
- **Interactions**: PUN-ABBR-004; `ORT-ABBR`.
- **Sources**: `[AkH12 §280]`, `[AkH12 §279 c)]`, `[AkH12 §290]`, `[AkH11 §282]`, `[AkH11 §288]`
- **Validation**: `goldens`
- **Confidence**: `normative` (`Ft`); `consensus` (others)

### PUN-NUMF-009 Ordinal period
- **Statement**: Ordinal numbers written in digits (Arabic or Roman) take a period, which is kept before a suffix hyphen; in a range only the second number takes it; house numbers after a street name take a period though read as cardinals.
- **Conditions**: ordinal reading (`L2`: following noun, case, or context); house number after street type
- **Output**: `1. osztály`, `3. sor`, `1978. évi`, `IX. kerület`; `3.-nak`, `8.-ba`, `10.-kel`; `a 10–12. oldalon`; `Tutaj u. 4.`, `4.-ben`; `4/B-ben` (no period after letter part)
- **Examples**: as above
- **Exceptions**: dates: day and year digits take no period before suffixes (PUN-DATEF-008); the article before digit ordinals follows pronunciation (`PND-ART`).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`; PUN-END-013 cue 2.
- **Sources**: `[AkH12 §267 a)]`, `[AkH12 §292]`, `[AkH11 §290]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-NUMF-010 Roman numerals
- **Statement**: Roman numerals are used only for ordinals, in traditional contexts, always followed by a period in ordinal use; Arabic and Roman forms are equally correct for volumes, congresses, centuries and districts; school classes use Arabic numerals.
- **Conditions**: `[IVXLCDM]+` in ordinal use (`L1`)
- **Output**: `IV. Béla`, `II. Rákóczi Ferenc`; `XVI. század` ~ `16. század`; `XIX. kerület` ~ `19. kerület`; `2013. IX. 12.`; `Petőfi u. 8. III. 2.`; `IV/3-as`; `X. agyideg`; classes `2. osztály`, `4. a` ~ `4/a`
- **Examples**: as above
- **Exceptions**: league name `NB I.` keeps the ordinal period (AkH11 §292 mid-list; AkH12 §294 sentence-final, same form; ORT-HYPH-016); in `Kovács II. Béla` the numeral follows the family name.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`; `PND-ART` (`a IV.`, `az V.`).
- **Sources**: `[AkH12 §294]`, `[AkH11 §292]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-NUMF-011 Spelled-out numbers: hyphen above 2000
- **Statement**: Spelled-out cardinals and ordinals are one word up to 2000; above 2000 only round thousands and millions are one word, otherwise hyphens separate the groups of three counted from the right; years are exempt (one word).
- **Conditions**: spelled-out numeral value > 2000 and not divisible into a single round group; year reading (`L4`/context: `-ban`, `évben`, date)
- **Output**: `ezerkilencszázkilencvenkilenc`; `kétezer`; `kétezer-egy`; `negyvenhétezer-ötszázhatvanhárom`; `hétmillió-négyszázkilencvenezer-ötszázharminc`; `négyezer-hatszázötödik`; year `kétezertizennégy`
- **Examples**: as above
- **Exceptions**: AkH11 applied the hyphen to years as well; AkH12 exempts years.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`, `PND-CARD` (owners of the full algorithm).
- **Sources**: `[AkH12 §263 d)]`, `[AkH12 §291 a)]`, `[AkH11 §289 a)]`, `[HUWIKI-AKH12CH]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-NUMF-012 Fractions
- **Statement**: Fractions in digits use a slash without spaces; typographic fraction characters are acceptable; suffixes attach with a hyphen.
- **Conditions**: `\d+/\d+` (`L0`)
- **Output**: `1/2` ~ `½`; `3/4-es`; mixed number `2 1/2` ~ `2½`
- **Examples**: as above
- **Exceptions**: dates and address sub-numbers also use slashes (PUN-PAREN-011).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM` (spelled fractions); `[LP-HU]` fraction glyph suggestions.
- **Sources**: `[AkH12 §275]`, `[LP-HU]`
- **Validation**: `goldens`
- **Confidence**: `normative` (slash); `tendency` (glyphs)

### PUN-NUMF-013 Approximate, alternative and distributive number pairs
- **Statement**: See PUN-DASH-004: hyphen for approximation and distribution, en dash for ranges.
- **Conditions**: number pair (`L0`)
- **Output**: `5-6 darab`; `2-2 alma`; `5–6. oldal`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-004; `ORT-NUM`.
- **Sources**: `[AkH12 §263 i)]`, `[AkH12 §264 c)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-NUMF-014 Ranges with units and symbols
- **Statement**: In a numeric range with a unit, the unit symbol is normally written once after the second number; the percent sign likewise.
- **Conditions**: `N1–N2 UNIT` (`L0`)
- **Output**: `5–10 km` ~ `5 km – 10 km`; `10–15%` ~ `10%–15%`; negative endpoints are clearer spelled with `és … között`: `−5 és +5 °C között`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-007, -008.
- **Sources**: consensus; no AkH point
- **Validation**: `goldens`
- **Confidence**: `tendency`

### PUN-NUMF-015 Paragraph sign
- **Statement**: The section sign `§` follows the ordinal number with a space; suffixes attach with a hyphen.
- **Conditions**: `\d+\. ?§` (`L0`)
- **Output**: `4. §`; `a 15. §-ban`; `§` alone before a number in lists is a foreign convention (tendency)
- **Examples**: `12.§` -> `12. §`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-SPACE-004; `ORT-ABBR`.
- **Sources**: `[AkH12 §265 d)]`, `[AkH12 §275]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-NUMF-016 Sports scores
- **Statement**: A score is written with a hyphen without spaces or with a colon; derived adjectives with `-s` are written in letters or with either digit form. The colon's spacing is not settled by §240 f): its prose says the score colon "egyik irányban sem tapad" (attaches to neither side), and its only example is the unspaced `6:3`. Both spellings are accepted and neither is required.
- **Conditions**: match result (`L4`)
- **Output**: `24-17`; `6:3` ~ `6 : 3`; `három nullás` ~ `3:0-s` ~ `3-0-s`
- **Examples**: `Az évszázad mérkőzésének eredménye 6:3.`; `2:1-re nyertek.`; `2 : 1-re nyertek.`
- **Exceptions**: -
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`.
- **Sources**: `[AkH12 §240 f), k)]`, `[HUWIKI-AKH12CH]`
- **Validation**: `goldens`
- **Confidence**: `normative`; spacing `contested`

### PUN-NUMF-017 Telephone numbers
- **Statement**: Telephone numbers are not regulated by AkH; they are grouped by the numbering plan (prefix, area code, subscriber number), not by thousands, and several conventions coexist.
- **Conditions**: telephone number (`L1` cues `tel.`, `+36`, `06`)
- **Output** (all in use; international form preferred for data): `+36 1 234 5678`; `+36 30 123 4567`; `06 1 234 5678`; `(06 1) 234-5678`; `06-1/234-5678`; `06-30/123-4567`
- **Examples**: as above
- **Exceptions**: never apply thousands grouping (PUN-NUMF-002) or decimal logic to telephone numbers. A suffix follows the reading of the last block, which the grouping determines: `450-8200-as` (read `negyvenezer-kétszáz`... i.e. the four-digit block) `[ENYELV 2009-05-21]`.
- **Predictability**: `tendency`
- **Lexical input**: Hungarian area codes (`1` Budapest; two-digit geographic codes; mobile `20`, `30`, `31`, `50`, `70`)
- **Interactions**: PUN-NUMF-002.
- **Sources**: `[ENYELV 2009-05-21]`, `[ENYELV 2018-06-28]`; AkH does not regulate the format (`OQ-08-009` resolved 2026-09-16: no norm exists, suffix follows the reading)
- **Validation**: `goldens`
- **Confidence**: `contested`

### PUN-NUMF-018 Letter-number combinations
- **Statement**: See PUN-DASH-011: free variation among solid, hyphen, en dash and space forms.
- **Conditions**: identifier (`L0`)
- **Output**: `B12-vitamin`, `M7-es`, `M2`, `H1N1`, `ABC-123`, `Forma–1`, `NB I.`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `lexical`
- **Lexical input**: identifier formats
- **Interactions**: PUN-DASH-011.
- **Sources**: `[AkH12 §294]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-NUMF-019 Multiplication sign and dimensions
- **Statement**: In typeset text the multiplication sign `×` (U+00D7) is used for products and dimensions, spaced on both sides; the letter `x` is a plain-text substitute.
- **Conditions**: `\d+ ?[x×] ?\d+` (`L0`)
- **Output**: `20 × 30 cm`; `3 × 4 = 12`
- **Examples**: `20x30` -> `20 × 30`
- **Exceptions**: AkH11 listed the middle dot `·` as multiplication sign; AkH12 lists none.
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-016.
- **Sources**: `[AkH11 §275]`, `[DEB-TIPO]` (secondary summaries); no AkH12 point
- **Validation**: `goldens`
- **Confidence**: `tendency`

### PUN-NUMF-020 Semicolons between listed decimal numbers
- **Statement**: A list of decimal numbers is separated by semicolons to avoid confusion with decimal commas.
- **Conditions**: list of >=2 numbers of which at least one has a decimal comma (`L0`)
- **Output**: `3,16; 37,5; 20 611,413`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: -
- **Sources**: `[AkH12 §293]` (list of examples separated by semicolons)
- **Validation**: `goldens`
- **Confidence**: `consensus`

### PUN-NUMF-021 Number followed by a suffix
- **Statement**: Suffixes attach to digits with a hyphen, in the form of the spoken number; details are in `ORT-NUM` and `PND-NUMINF`.
- **Conditions**: digits + suffix (`L2`)
- **Output**: `4-et`, `12-t`, `19-szer`, `32-en` (harmincketten) vs `32-n` (harminckettőn), `67-es`
- **Examples**: as above
- **Exceptions**: ordinals keep their period (PUN-NUMF-009); dates drop it (PUN-DATEF-008).
- **Predictability**: `rule`
- **Lexical input**: spoken form of the last number word
- **Interactions**: `ORT-NUM`, `PND-NUMINF`.
- **Sources**: `[AkH12 §265 d)]`
- **Validation**: `goldens`
- **Confidence**: `normative`

## DATEF: dates and times

### PUN-DATEF-001 Full date forms
- **Statement**: Dates are written year-month-day; the year in Arabic digits followed by a period; the month written out, abbreviated, or in Roman or Arabic digits followed by a period; the day in Arabic digits followed by a period, or with a hyphenated suffix and no period; a space follows each period.
- **Conditions**: full date (`L0`/`L1`)
- **Output**:

| Month form | Example | With day suffix |
|---|---|---|
| written out | `2014. február 28.` | `2014. február 28-án` |
| abbreviated | `2014. febr. 28.` | `2014. febr. 28-a` |
| Roman | `2014. II. 28.` | `2014. II. 28-án` |
| Arabic | `2014. 02. 28.` | `2014. 02. 28-án` |

  - Arabic month with leading zero as in AkH12; day with leading zero (`2014. 02. 05.`) is common in forms (tendency)
  - spaceless `2014.02.28.` is non-standard: insert spaces (`[LP-HU]`)
  - month names lowercase (`ORT-CAP`)
- **Examples**: as above
- **Exceptions**: ISO form (PUN-DATEF-004).
- **Predictability**: `rule`
- **Lexical input**: month names and abbreviations (PUN-ABBR-013)
- **Interactions**: `ORT-NUM`; PUN-END-013 cue 2 (the final day period may coincide with the sentence period: write one).
- **Sources**: `[AkH12 §267 c)]`, `[AkH12 §295]`, `[AkH11 §293]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-DATEF-002 Date at sentence end
- **Statement**: A date whose day period falls at the end of a declarative sentence is followed by one period only.
- **Conditions**: `DAY.` + sentence end (`L1`)
- **Output**: `Született 1990. május 3.`
- **Examples**: as above
- **Exceptions**: `?`/`!` added after the period: `…május 3.?` (consensus by analogy with PUN-END-010).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-END-010.
- **Sources**: inferred from `[AkH12 §240 a)]`, `[AkH12 §295]`
- **Validation**: `goldens`
- **Confidence**: `consensus`

### PUN-DATEF-003 Order and punctuation of partial dates
- **Statement**: Year-month and month-day partial dates follow the same order and period rules as full dates.
- **Conditions**: partial date (`L1`)
- **Output**: `2014. február`; `február 28.`; `febr. 28-án`; `II. 28.`
- **Examples**: `március 15-én`
- **Exceptions**: year + month in grammatical relation (PUN-DATEF-005).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DATEF-005, -008.
- **Sources**: `[AkH12 §295]`, `[AkH12 §296]`, `[AkH12 §298]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DATEF-004 ISO 8601 numeric dates
- **Statement**: The hyphenated `YYYY-MM-DD` form is not an AkH12 date form; AkH11 noted it as spreading; it is acceptable in technical, tabular and machine-readable contexts.
- **Conditions**: `\d{4}-\d{2}-\d{2}` (`L0`)
- **Output**: running text: `2024. 01. 05.` or `2024. január 5.`; data/technical: `2024-01-05`
- **Examples**: as above
- **Exceptions**: do not convert inside code, identifiers, file names, URLs.
- **Predictability**: `tendency`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-019 (not an en dash context).
- **Sources**: `[AkH11 §293]`, `[AkH12 §295]` (omits it), `[ISO8601]`
- **Validation**: `goldens`
- **Confidence**: `contested`

### PUN-DATEF-005 Period or no period after the year
- **Statement**: The year takes a period when it is not in a grammatical relation with the following month name or time word; it takes no period when followed by a postposition or postposition-derived adjective, when it is the possessor of the next word, or when it is the subject; the period may be omitted on inscriptions, title pages and in parentheses.
- **Conditions** (`L2`, subject case `L3`); decide on the word the year is syntactically related to, not on the next token (`1849 tavaszi hónapjai`: the year is the possessor of `hónapjai`, although `tavaszi` follows it):
  - period: month name without possessive suffix (`1848. március 9.`, `1848. márciusban`), `-i` adjective heading the year's phrase (`1848. áprilisi`, `1944. őszi`, `1849. tavaszi–nyári`), `évi` (`2014. évi`), month name followed by a possessed noun or postposition that relates to the month, not the year (`1956. október végén`, `1956. október havában`, `1956. október folyamán`)
  - no period: postposition or postposition adjective (`1848 előtt`, `1848 előtti`, `1849 után`, `1956 folyamán`, `1867 és 1896 között`); possessed word (`1526 augusztusában`, `1944 őszén`, `1848 hősei`, `1956 októberének végén`, `1848 első felében`, `1849 tavaszi hónapjai`, `1944/45 nehéz telén`); year as subject (`1492 fordulópont volt.`)
  - optional omission: epitaphs, plaques, title pages, parentheses (`(1526)`)
- **Output**: as above
- **Examples**: as above
- **Exceptions**: suffixed year digits never take a period (PUN-DATEF-008).
- **Predictability**: `rule`
- **Lexical input**: postposition list (`PND-POSTP`), possessive morphology
- **Interactions**: `ORT-NUM`; `[LP-HU]` year-period rules.
- **Sources**: `[AkH12 §296]`, `[AkH12 §297 a)-c)]`, `[AkH11 §294]`, `[AkH11 §295]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-DATEF-006 Year and century ranges
- **Statement**: In linked ordinal years or centuries only the second member takes the period; the link is an unspaced en dash or a slash, and the second year may be shortened.
- **Conditions**: `YEAR[–/]YEAR` + ordinal context (`L0`)
- **Output**: `2013–2014. évi` ~ `2013/2014.` ~ `2013–14.` ~ `2013/14. évi`; `20–21. század` ~ `XX–XXI. század`; without ordinal context `1941–1945` (no period, e.g. as subject), suffix `1848–49-ben`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-007, -010; PUN-PAREN-011.
- **Sources**: `[AkH12 §296]`, `[AkH12 §264 c)]`, `[AkH11 §294]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DATEF-007 Day ranges and ranges between full dates
- **Statement**: Days within one month are linked by an unspaced en dash with the period after the second day; two full or nearly full dates are linked by an en dash with spaces on both sides, and the first date keeps its final period.
- **Conditions** (`L0`):
  - same month: `MONTH D1–D2.`
  - different months or years: `DATE1. – DATE2.`
- **Output**: `2014. április 28–30.`; `2014. április 11. – 2014. október 10.`; `2014. I. 15. – VIII. 23.`; `f. hó 5. – f. hó 25.`; months only: `1945. március–április`
- **Examples**: as above
- **Exceptions**: the MTA NYTI advisory blog (non-binding) shows the unspaced form between cross-month dates (`2015. február 1.–2015. március 31.`) and calls the spaced form typographically defensible; AkH12 §296 prescribes the spaced form. A lint must not flag the unspaced cross-month form.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-008; sentence segmentation (out of scope) (a date ending in a period followed by ` – ` is not a sentence boundary).
- **Sources**: `[AkH12 §296]`, `[AkH12 §264 c)]`, `[HELYES-DATE]`, `[ENY-DATE]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DATEF-008 Suffixes on day and year digits
- **Statement**: Suffixes attach to year and day digits with a hyphen and without period; the day's possessive form (`-a`/`-e`, `-ja`/`-je`) and its case forms follow the spoken ordinal.
- **Conditions**: year or day digits + suffix (`L2`)
- **Output**:
  - years: `1838-as`, `1514-ben`, `2020-ig`
  - days: `15-én`, `6-ig` ~ `6-áig`, `12-től` ~ `12-étől`, `28-a`, `28-án`
  - day 1: `1.` ~ `1-je`; `1-én` ~ `1-jén`; `1-ig` ~ `1-jéig`; adjective `1-i` ~ `1-ji` ~ `1-jei` (AkH11: only `1-je`, `1-jén`, `1-jéig`, `1-ji`, `1-jei`)
  - vowel of the day possessive by day number (from the spoken ordinal, e.g. `másodika`, `negyedike`):

| Days | Possessive | Superessive |
|---|---|---|
| 1 | `-je` | `-jén` ~ `-én` |
| 2, 3, 6, 8, 13, 16, 18, 20, 23, 26, 28, 30 | `-a` | `-án` |
| 4, 5, 7, 9, 10, 11, 12, 14, 15, 17, 19, 21, 22, 24, 25, 27, 29, 31 | `-e` | `-én` |

- **Examples**: `március 15-e`, `május 2-án`, `augusztus 20-a`, `október 23-án`, `december 31-én`
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`, `PND-NUMINF`; `[LP-HU]` wrong-vowel day rules.
- **Sources**: `[AkH12 §295]`, `[AkH12 §298]`, `[AkH11 §296]`, `[LP-HU]` (vowel classes cross-checked)
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-DATEF-009 Day number before a postposition
- **Statement**: When a postposition follows a day number, the day takes either a period or the hyphenated possessive form; a bare number is wrong.
- **Conditions**: `DAY` + postposition (`között`, `óta`, `után`, `előtt`, `körül`) (`L2`)
- **Output**: `december 20. és 31. között` ~ `december 20-a és 31-e között`; `december 10. óta` ~ `december 10-e óta`; `*december 10 óta`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: postposition list
- **Interactions**: PUN-DATEF-008.
- **Sources**: `[AkH12 §299]`, `[AkH11 §297]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-DATEF-010 Weekday with a date
- **Statement**: A weekday added to a date is written in lowercase after the date, commonly separated by a comma or put in parentheses.
- **Conditions**: date + weekday (`L1`)
- **Output**: `2024. január 5., péntek` ~ `2024. január 5. (péntek)`; before the date: `pénteken, január 5-én`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `tendency`
- **Lexical input**: weekday names
- **Interactions**: `ORT-CAP`.
- **Sources**: consensus; no AkH point
- **Validation**: `goldens`
- **Confidence**: `tendency`

### PUN-DATEF-011 Relative date abbreviations
- **Statement**: `f. hó` (folyó hó), `f. év` (folyó év) and similar relative date abbreviations keep their periods and space; the day after them follows date punctuation.
- **Conditions**: `L1`
- **Output**: `f. hó 5-én`; `f. hó 5. – f. hó 25.`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `lexical`
- **Lexical input**: AkH12 §284 list
- **Interactions**: PUN-ABBR-002.
- **Sources**: `[AkH12 §264 c)]`, `[AkH12 §284]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DATEF-012 Time of day
- **Statement**: In running text hours and minutes are often written out; in digits hour and minute are separated by a period or a colon without spaces; suffixes attach with a hyphen.
- **Conditions**: clock time (`L0`: `\b([01]?\d|2[0-3])[.:][0-5]\d\b`, disambiguated from decimals by context: `-kor`, `-ig`, `-tól`, `óra`, `indul`, `érkezik`)
- **Output**: `10 óra 25 perckor`; `10.35` ~ `10:35`; `10.35-kor` ~ `10:35-kor`; `10.35-ös`; `du. 5 órakor`
- **Examples**: `Az indulás időpontja: 10:35.`
- **Exceptions**: AkH11 allowed only the period; seconds (`10:35:20`), leading zero hours (`08:05`) and the 24-hour clock are common practice without AkH regulation (tendency).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`; PUN-NUMF-003.
- **Sources**: `[AkH12 §240 a), f)]`, `[AkH12 §300]`, `[AkH11 §299]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DATEF-013 Time ranges
- **Statement**: Time intervals are linked by an unspaced en dash; the unit word or suffix follows the second member.
- **Conditions**: `TIME–TIME` (`L0`)
- **Output**: `9–17 óráig`; `10.00–12.00` ~ `10:00–12:00`; `8-tól 10-ig` (suffix alternative)
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DASH-007.
- **Sources**: `[AkH12 §264 c)]` (from-to principle); time-specific forms: consensus
- **Validation**: `goldens`
- **Confidence**: `consensus`

### PUN-DATEF-014 Decades and shortened years
- **Statement**: Decades are written with digits and a hyphenated `-as/-es` adjective; a shortened year takes an apostrophe.
- **Conditions**: `L0`
- **Output**: `az 1990-es évek`; `a ’90-es évek`; `’56`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-APOS-003; PUN-DATEF-008.
- **Sources**: `[AkH12 §275]`, `[AkH12 §298]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DATEF-015 Centuries
- **Statement**: Centuries are written with an Arabic or Roman ordinal followed by a period.
- **Conditions**: `L1` (`század`)
- **Output**: `16. század` ~ `XVI. század`; `a XX. században`; range `20–21. század`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-NUMF-010; PUN-DATEF-006.
- **Sources**: `[AkH12 §294]`, `[AkH12 §296]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-DATEF-016 Years inside noun phrases
- **Statement**: A year used as an attribute takes a period before `évi` and before `-i` adjectives written as separate words, and a hyphenated suffix without period when suffixed directly.
- **Conditions**: `L2`
- **Output**: `az 1983. évi`; `a 2014-es év`; `az 1848-as forradalom`
- **Examples**: as above
- **Exceptions**: `-`
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-DATEF-005, -008.
- **Sources**: `[AkH12 §267 a)]`, `[AkH12 §296]`, `[AkH12 §298]`
- **Validation**: `goldens`
- **Confidence**: `normative`

## SPACE: spacing

### PUN-SPACE-001 Attachment table of punctuation marks
- **Statement**: Each mark has a fixed attachment to its left and right neighbours; exceptions are listed per mark.
- **Conditions**: every mark (`L0`)
- **Output**:

| Mark | Space before | Space after | Exceptions |
|---|---|---|---|
| `.` | no | yes | no space after when another attaching mark follows (`stb.,`); none inside times `10.35`, URLs, e-mail addresses; none before suffix hyphen `3.-nak` |
| `…` | no (after interrupted word) | yes | space before, none after when the sentence start is omitted (`…és`); `[…]` spaced as a word |
| `?` `!` | no | yes | no space when an attaching mark follows (`?)`, `!”`, `?!`) |
| `,` | no | yes | decimal comma: no space after (`316,26`) |
| `;` | no | yes | - |
| `:` | no | yes | clock time `10:35` no spaces; score `6:3` or `6 : 3` (PUN-NUMF-016) |
| `„` `»` (opening) | yes | no | line start |
| `”` `«` (closing) | no | yes | no space before attaching mark or suffix hyphen (`”.`, `”-ban`) |
| `(` `[` | yes | no | inside a word: no space before (`írásjel(ek)`) |
| `)` `]` | no | yes | no space before attaching mark; space before a dash (`) –`); inside a word no space after (`Hara(n)g`) |
| `–` gondolatjel | yes | yes | no space between it and a following `,` `;` `:` `.` |
| `–` nagykötőjel | no | no | spaced between full dates; optionally spaced between other complex members (PUN-DASH-008) |
| `-` kötőjel | no | no | suspended compounds (PUN-DASH-002) |
| `/` | no | no | verse breaks ` / `, ` // ` |
| `%` `‰` | no | yes | - |
| `°` (angle) | no | yes | `°C` preceded by a space |
| `§` | yes | yes | suffix hyphen `§-ban` |
| `’` apostrophe | no (word-final elision) | yes | year `’95`: space before, none after |
| `×` | yes | yes | - |
| unary `−` `+` `±` | yes | no | - |

- **Examples**: see table
- **Exceptions**: in table
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: all areas; `WS-CHAR` (space code points); `[LP-HU]` spacing detectors.
- **Sources**: `[AkH12 §240]`, `[AkH12 §240 a)-l)]`, `[AkH12 §275]`, `[AkH12 §279 c)]`; AkH11 gives only the general statement and refers to typewriting standards `[AkH11 §239]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

### PUN-SPACE-002 One word space
- **Statement**: Words and marks are separated by exactly one space; runs of spaces and tabs used for alignment are non-standard in running text; no double space after sentences.
- **Conditions**: `  +` between tokens (`L0`)
- **Output**: collapse to one space (U+0020 or the no-break variant required by PUN-SPACE-004)
- **Examples**: `Vége.  Új mondat.` -> `Vége. Új mondat.`
- **Exceptions**: preformatted text, tables, code.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `REP` (letterspacing damage).
- **Sources**: `[AkH12 §240]`; `[LP-HU]`; `[DEB-TIPO]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `consensus`

### PUN-SPACE-003 No spaces in URLs and e-mail addresses
- **Statement**: Periods and other characters inside internet addresses take no space on either side; a sentence may end after an address with an ordinary period.
- **Conditions**: URL/e-mail token (`L0`)
- **Output**: `www.mta.hu`; `nev@pelda.hu`; sentence end `…a www.mta.hu oldalon.`
- **Examples**: as above
- **Exceptions**: a sentence-final period directly after a URL is ambiguous for machines; rephrasing so the URL is not last is a tendency, not a norm.
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-END-013 cue 1; sentence segmentation (out of scope).
- **Sources**: `[AkH12 §240 a)]`; `[QUNTOKEN snt.qx]`
- **Validation**: `goldens`
- **Confidence**: `normative`

### PUN-SPACE-004 No-break space contexts
- **Statement**: Where AkH requires a space between elements that belong together, typographic practice uses a no-break space so that the elements are not separated at a line break.
- **Conditions** (`L1`):
  - number and unit/currency: `100 km`, `2890 Ft`, `23 °C`
  - digit groups: `23 816` (narrow no-break, PUN-SPACE-005)
  - ordinal and noun in fixed designations: `IV. Béla`, `XIX. kerület`, `V. kerület`, `4. §`, `15. §`
  - title abbreviation and name: `dr. Kiss`, `ifj. Nagy`
  - initials: `J. P.`, `Kiss J. Péter`
  - multi-part abbreviations: `i. sz.`, `Kr. e.`, `i. m.`, `i. h.`, `f. hó`
  - address elements: `Pf. 20`, `u. 4.`, `II. em.`
  - date elements: between year and month and between month and day (`2024. január 5.`) (tendency)
  - dialogue dash and first word (`– Nem`) (tendency)
- **Output**: U+00A0 (or U+202F in digit groups)
- **Examples**: as above
- **Exceptions**: plain-text output may use U+0020 everywhere; one-letter words (`a`, `s`, `e`) bound to the next word is a typographic habit only (`WS-CHAR`).
- **Predictability**: `tendency`
- **Lexical input**: abbreviation and unit lists
- **Interactions**: `WS-CHAR` (all spaces are separators for tokenisation).
- **Sources**: `[DEB-TIPO]` (number + unit, `V. kerület`, `i. m.`, `i. h.`); `[LP-HU]` (no-break space before `°C` and in dates); AkH positions `[AkH12 §240]`, `[AkH12 §279 c)]`
- **Validation**: `goldens`
- **Confidence**: `consensus`

### PUN-SPACE-005 Narrow and thin spaces
- **Statement**: Hungarian uses no thin spaces before `?`, `!`, `:` or `;` (unlike French); a narrow no-break space is recommended only as the digit group separator.
- **Conditions**: U+2009, U+202F, U+00A0 before `?!:;` (`L0`)
- **Output**: remove spaces before `?!:;`; digit groups U+202F
- **Examples**: `Miért ?` -> `Miért?`
- **Exceptions**: the spaced score colon `6 : 3` (PUN-NUMF-016).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-NUMF-002; PUN-SPACE-001.
- **Sources**: `[AkH12 §240 c), d), f), h)]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative` (no space before marks); `consensus` (U+202F)

### PUN-SPACE-006 Detecting spacing errors around marks
- **Statement**: A lint flags a space before an attaching mark and a missing space after a mark that requires one, with the exemptions of PUN-SPACE-001.
- **Conditions** (`L0`):
  - ` +[.?!,:;)”«]` -> remove space (except `6 : 3`, ` – `, `[…]`, ellipsis for omitted sentence start)
  - `[a-záéíóöőúüű][?!,:;”][A-Za-zÁÉÍÓÖŐÚÜŰáéíóöőúüű]` -> insert space (except URLs, e-mail addresses, code)
  - `[a-záéíóöőúüű]\.[A-ZÁÉÍÓÖŐÚÜŰ]` -> insert space (except abbreviations with internal periods written solid in the source, URLs)
  - letter classes are listed explicitly: code-point ranges such as `[á-ű]` and `[Á-Ű]` span U+00E1..U+0171 and U+00C1..U+0170 and include letters of the other case
  - `[(„»] +` -> remove space
- **Output**: diagnostics with fixes
- **Examples**: `Szia ,Péter!` -> `Szia, Péter!`
- **Exceptions**: listed
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: PUN-SPACE-001; `REP`.
- **Sources**: `[AkH12 §240]`; detector design `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `consensus`

### PUN-SPACE-007 Space in number-suffix and number-word combinations
- **Statement**: No space separates a digit, symbol or abbreviation from its hyphenated suffix; a space separates a number from a following word it modifies.
- **Conditions**: `L0`
- **Output**: `10%-os`, `12-én`, `MTA-t`; `3 alma`, `107 ezer`; `* -ban` (space before suffix hyphen) is an error
- **Examples**: `2024 -ben` -> `2024-ben`
- **Exceptions**: suspended compounds (PUN-DASH-002).
- **Predictability**: `rule`
- **Lexical input**: `none`
- **Interactions**: `ORT-NUM`, `ORT-ABBR`; `[LP-HU]` rule "suffix without space".
- **Sources**: `[AkH12 §265 d)]`, `[AkH12 §290]`, `[LP-HU]`
- **Validation**: `goldens`, `oracle:lightproof`
- **Confidence**: `normative`

## Open questions
1. `!?` versus `?!`: AkH attests only `?!`; is `!?` to be flagged? (PUN-END-004)
2. Sports score colon: AkH12 §240 f) text says the colon attaches to neither side (`6 : 3`), its only example, online, is `6:3`; printed AkH12 not checked. Both are accepted until it is. (PUN-NUMF-016)
3. Members of the fused `anélkül hogy` class beyond the three attested (`azonkívül hogy`, `amellett hogy`, `feltéve hogy`): AkH says "stb."; Lightproof exempts `feltéve`.
4. Integrated quotation ending in `?` or `!` (`Azt kiáltotta, hogy „Segítség!”`): mark placement not regulated. (PUN-QUOT-007)
5. AkH12 §240 c), d) examples `(Ki tudja, miért?).` conflict with §253 (independent sentence in parentheses carries its mark inside, no outer period). (PUN-PAREN-008)
6. Resolved 2026-09-15: `NB I.` with period (AkH11 §292 mid-list; AkH12 §294 compatible; aligned with ORT-HYPH-016). (PUN-NUMF-010)
7. Close apposition without comma (`a barátom Péter`, `a költő Petőfi`): not covered by AkH; current rule treats it as tendency.
8. `pl.:` and `például:` followed by colon: not regulated. (PUN-ABBR-007)
9. Telephone number formats: no normative source consulted; NMHH or MSZ guidance may exist. (PUN-NUMF-017)
10. Suffixes on foreign names with apostrophe (`O’Neillnek`?) are not regulated in AkH12. (PUN-APOS-004)
11. Em dash: some Hungarian university style sheets still prescribe `—` for gondolatjel; whether the library should flag it or only normalise on request. (PUN-DASH-015)
12. Lowercase continuation after `Kedves X,` in e-mails (English influence): attestation rate unknown.
13. No-break space after one-letter words and in dates: typographic habit strength unmeasured; Gyurgyák's handbook not consulted directly. (PUN-SPACE-004)
14. Weekday abbreviations other than `csüt.` have no normative form. (PUN-ABBR-013)
15. Ratio and map scale colon spacing (`1:100 000`) not regulated.
16. Operator spacing: AkH12 §275 examples mix `2+3`, `2+2 = 4`, `ő + ő = ők`. (PUN-DASH-016)

## Sources used
All keys are in [sources.md](sources.md). Reliability notes for this chapter:
- `ENEKES`: excerpt only.
- `ISO8601`: status inferred from AkH11 §293 and secondary sources.
- `DEB-TIPO`: its Gyurgyák citation not consulted directly.

## Terms
See [glossary.md](glossary.md).
