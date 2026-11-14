# Premise audit: do the rulebook and its golden sets state correct Hungarian?

Date: 2026-11-08. Scope: the golden sets `cargo xtask goldens` extracts (`goldens.ron`,
`accepts.ron`, `rejects.ron`) and the rule text behind them, checked against external sources
instead of against the engine.

## Oracles and their limits

| Oracle | What it answers | Limit |
|---|---|---|
| MTA "Helyes-e így?" (`helyesiras.mta.hu/helyesiras/default/suggest`, form POST) | token-level spelling verdict, `helyes` or `ismeretlen` with suggestions | over-accepts like an affix-file checker: `helyes` for `kettedik`, `házéul`, `alusszanak`, `egyedik`; weak evidence alone |
| AkH12 full text (`helyesiras.mta.hu/helyesiras/default/akh12#<paragraph>`) | normative orthography | silent on morphology beyond spelling |
| `WC2` (Webcorpus 2 frequency list, form, lemma, pos, freq) | attestation, per lemma | includes typos and accent-stripped text; a contrast needs a ratio, not a count |
| hunspell `hu_HU` | token acceptance | over-accepts |
| `ertelmezo.oszk.hu`, hu.wiktionary, RST12 | lexicographic and theoretical judgements | RST12 `%` marks are theoretical, not measured |

A verdict of "wrong" was recorded only on a normative source or a clear corpus contrast.

## Findings

### The negative golden set tested strings, not readings

A star in the rulebook forbids a reading: `DER-ADJ-002` says `autó` forms no relational `-i`
adjective. `rejects.ron` turned every star into "the checker must reject this string", and the
string is often a word under another analysis: `autói`, `fülei`, `háborúi` and `mellei` are
possessed plurals, `drappul` is an essive, `szú` and `báj` are nouns. Of the 176 chapter 06
stars, 131 forbid a string that does not exist, 19 forbid a reading of a valid word, 13 are wrong
stars and 13 are register judgements. The engine was partly tuned toward the string test and
rejects good words (`ünnepekkor` WC2 5,085, `ballagós`, `kocsicska`, `emlékezget`).

`cargo xtask eval` now scores a star of a derivation rule against its suffix. The suffix is the
record the rule heads, or a record whose head rule's heading names a suffix the rule's own
heading names. Only an accepted reading through that suffix is a miss. An accepted form is also
counted against its rule when the lexicon lists the starred string itself (lexicon and rule
disagree), when a reading carries no feature stack (its formation is unknown), or when the rule
names no suffix (the string is tested). The report lists each group with the readings.

An inflection star carries no suffix record, so the text's own contrast scopes it: the nearest
unstarred form before the star in its segment that shares its first two letters is its rival
(`` `lovat` only, `*lót` ``, `` `les`->`lesett` (`*lest` ``). The star counts against its rule when
a reading of it has the lemma and features of a reading of the rival. A rival clears only
inflections of listed words: a derivation reading or an unlisted bare reading still counts, so
`*semi` beside `semmi` stays a miss. Without a rival the string is tested.

### The extractor misread the rule text

| Misread | Example | Fix |
|---|---|---|
| markdown bold read as a star | ``**`szú`** is not a v-stem`` became `*szú` | a star is an odd run of `*` |
| an arrow's comma list ran on into prose | `Magyarország -> Máltán`, `kétli -> kétell` | more than two words between ticks end the list |
| a `;` inside a gloss split the segment, so the gloss's last form became the input | `fövő -> fővöm` for `fő` | `;` splits only outside parentheses |
| qualifiers dropped | `lakik -> lakol` (archaic), `esik -> esném` (traditional) | forms marked archaic, traditional, obsolete, dated, dialectal or regional are dropped; names are exempt |
| zero counts read as examples | `ötödikök` 0, `erdői` [ATT 0] | a zero count marks a form unattested |
| identifiers and labels read as forms | `p_back`, `lex_compound`, `blocked`, `true` | `_` is never in a form; label values in `Examples` and `Output` are written without ticks |
| line-break halves read as forms | `` `Dumas-` / `nak` `` | dash-ended slash pairs are dropped |
| chapters 07, 08 and 10 read as segmenting (`("02".."=06")` is a range up to the string `=06`) | `bogáncs-szívem`, `DNS-sel`, `meg-megáll` never reached `accepts.ron` | the range is `"02"..="06"`; a line-break rule's hyphenated example (`ka-to-na`) is scored as a split by `is_legal_split`, and template labels (`Name-suffix`) are unticked |

### Rule logic, by chapter

Chapters 02 to 05 hold: 281 of 295 sampled goldens are correct, and harmony classes, linking
vowels, lowering, imperatives, the definite conjugation, `-jA` and the v-stems are stated right.
Errors found:

| Rule | Claim | Evidence |
|---|---|---|
| `VRB-PAST-005` | `dz` counts as one short consonant, so `edztem` is a variant | `dz` is long after a vowel; `edzettem` 8,407 : `edztem` 24 [WC2] |
| `PH-VSTEM-004` | `falu` takes the `v` stem before every triggering suffix | only before the plural and the possessed plural: `falum` 3,585 : `falvam` 19, `falut` 55,528 : `falvat` 839 [WC2] |
| `PH-HARM-018`, `NOM-CASE-014` | `haver`, `alibi` vacillate before `-jA` (RST12 `%`) | `haverja` 10,264 : `haverje` 15 [WC2]; the opacity rule itself holds |
| `PND-CARD-004` | `kettedik` is an ordinal | only as the last member (`tizenkettedik`); `PND-ORD-001` stars it |
| `NOM-MODEL-003` | `-kor` takes no plural, `*ünnepekkor` 0 [WC2] | `ünnepekkor` 5,085 [WC2]; the restriction is semantic (time points), not morphotactic |
| `PND-POSTP-010` | `*mellei`, `*elei`, `*képesti` have 0 tokens | 10,484, 1,114 and 5,854 [WC2]; the claim is about the postposition reading |

Chapters 07 and 08 follow AkH12 closely: of about 100 rules compared sentence by sentence, about
90 are faithful, and no § locator is wrong. Errors found:
- `ORT-NUM-010` contradicts §291 a) on years. A spelled-out year is written solid
  (`kétezertizennégy`), and a year range takes the en dash (§264 c), §296).
- `PUN-NUMF-016` spaces the score (`2 : 1`), where §240 f) writes `6:3`.
- `ORT-SUFX-002` turns §29's principle into a closed list.
- `PUN-QUOT-012` and `PUN-QUOT-005` are stricter than §259 c) and §257 b).
- 53 rejected chapter 07 examples are principles the engine lacks: §127, §128, §289, §264 c)
  and §216 a).

Chapter 06 states its restrictions as semantic taxonomies where formal conditions exist:

| Rule | Stated | Better supported |
|---|---|---|
| `DER-ADJ-020` `-hAtÓ` | transitive base | the head is the object or a locative/path argument: `élhető` 44,897, `járható` 61,472, `síelhető` 1,102 [WC2] |
| `DER-ADJ-017` `-Ós` | no `-Ós` from activity verbs | the star holds for 'prone to V' only; 'involving V-ing' is productive: `beszélgetős` 16,113, `lövöldözős` 8,241 [WC2] |
| `DER-ADJ-010` `-AtlAn` | transfer verbs excluded | `kifizetetlen` 6,020, `fizetetlen` 2,259 [WC2]; the object must exist apart from the event, which is why creation verbs fail |
| `DER-VERB-008` `-ít` | absolute-property adjectives excluded | `butít` 2,536, `okosít` 266 [WC2] |
| `DER-ADJ-002` `-i` | body parts, materials and products excluded | holds for bare bases only: `közéleti` 144,988, `tankönyvi` 9,876, `füli` 1,218 [WC2]; `MSZP-i` is `helyes` |
| `DER-DIM-004` | concrete vs abstract | count vs mass: `hibácska` 94, `ügyecske` 49 [WC2] |

### Attestation claims

Most `[WC2]` counts in the rulebook reproduce exactly. The false ones are zeros asserted without
a lemma filter (`ünnepekkor`, `mellei`, `képesti`), and each drove an engine rejection.

## Measured effect of the tooling change

| Set | String test | Reading test | Chapter fixes | Logic round |
|---|---|---|---|---|
| goldens reproduced | 2131/2165 | 2130/2164 | 2133/2167 | 2151/2166 |
| starred forms kept out | 180/290 | 186/284 | 175/233 | 196/242 |
| example forms accepted | 12334/12577 | 12320/12506 | 12377/12532 | 13258/13494 |
| example texts unflagged | 143/143 | 143/143 | 145/145 | 145/145 |
| ledger covered / partial / unmeasured | 752 / 187 / 286 | 780 / 152 / 293 | 798 / 131 / 296 | 846 / 132 / 247 |

The logic round's example set is larger because the accept extractor stopped dropping hyphenated
forms in chapters 07, 08 and 10, and its starred set scopes inflection stars by their rival.

The starred set shrinks because wrong stars were removed and register judgements moved to `?`.
Clean-text regression (3,600 sentences): 0 newly rejected tokens, 0 repair proposals.
Round trip: nouns 26,016 and verbs 19,164 at 100%.

## Fixes: chapters 02-05

Counts are lemma-filtered Webcorpus 2 tokens.

### Rule logic

- `VRB-PAST-005`, `VRB-LINK-002`: `dz` after a vowel is long [d͡zː] (`PH-CONS-004`). A monosyllable in
  vowel + `dz` is therefore heavy, and takes `-Ott-` in every past cell: `edzettem` 8,407 : `edztem` 24,
  `pedzettem` 66 : `pedztem` 2, and `edzeni`, `pedzeni`. The condition is phonological, and it is
  computed in `verbal::heavy` and `past_nonfinal`. The polysyllabic `-(Ó)dz-` stems are excluded
  because their `dz` is short: `lopództam` 60 : `lopódzottam` 0, `lopódzni` 60 : `lopódzani` 14. The
  supplement row that kept `edztem` as a variant (`AlternationOptional`) is gone.
- `PH-VSTEM-004`, `NOM-STEM-011`, `NOM-POSS-010`: the `v` of `falu`, `daru`, `odú`, `tetű` appears
  before the plural `-k`, the possessed plural `-i` and derivatives. Every personal possessive takes
  the vowel stem, as the class shows: `falum` 3,585 : `falvam` 17, `falunk` 10,676 : `falvunk` 0,
  `darum` 59 : `darvam` 3, `odúm` 48 : `odvam` 6, `tetűm` 13 : `tetvem` 3, `falujuk` 1,448 : `falvuk` 3.
  The exception is the third singular `-A` of a noun that takes `-A`: `tetve` 187 : `tetűje` 17. This
  is a class condition (`VStem` + `UStem`), so no per-word list is needed:
  `alternation::free_personal_possessive`. AkH12 §30 names only `faluk` ~ `falvak` as a doublet.
- Open: `falva` (possessive of `falu`) is still accepted, because `AlternationOptional`, which `faluk`
  needs, also flips the `-jA` choice in `analysis::possessive_variant`. The string is a word anyway,
  the converb of `fal` (2,841).
- `PH-HARM-018`, `NOM-CASE-014`, `NOM-PPL-004`, `DER-MODEL-009`: `haver` is back (`haverja` 10,264 :
  `haverje` 15, `havernak` 3,259 : `havernek` 161), so its `HarmonyOptional` was dropped. `alibi` is front
  (`alibije` 920 : `alibija` 4, `alibinek` 450 : `alibinak` 9), so its supplement row was dropped and the
  computed front class stands. The opacity principle is now shown on `fotel`, which really vacillates:
  `fotelja` 258 : `fotelje` 276, `foteljének` 64, `foteljának` 21, `*foteljénak` 0. `agilis`
  vacillates in favour of back (`agilisnak` 144 : `agilisnek` 49, `agilisabb` 744 : `agilisebb` 255),
  so it now carries `Back` beside `HarmonyOptional`.
- `PND-CARD-004`, `PND-NUMINF-001`: `-kettedik` stands only as the last member of a compound.
  `PND-ORD-001`'s "[ATT 0]" was false (`kettedik` 216, `egyedik` 2,897), but the star stands against
  `második` 3,174,469.
- `NOM-MODEL-003`, `NOM-CASE-017`, `NOM-PL`: `-kor` follows a plural when the noun names an occasion
  or a recurring time point (`alkalmakkor` 26,471, `ünnepekkor` 5,085, `órákkor` 98). The engine no
  longer refuses Plural + `Tem`; it still refuses `-é`/`-ék` + `-kor`. The semantic restriction
  (`*asztalkor`) needs a time-noun class the lexicon does not have, so the engine accepts `-kor` on any
  noun. That gap is open. A list would be the only means, and none is justified for one suffix.
- `PND-POSTP-010`: the zeros were string counts. They are restated as the postposition + `-i`
  reading: `mellei` 10,484 and `elei` 1,114 are possessed plurals. `képesti` (5,854, `tavalyhoz
  képesti`) was a wrong star and is accepted beside `általi`.

### Variants and theory items

The extractor drops a `?`-marked form from goldens, accepts and rejects alike, so `?` marks a
marginal form without deleting it from the text. `#` marks a potential form.

| Form | Change | Evidence |
|---|---|---|
| `tehert` | `?`, marginal | `terhet` 91,363 : 356 |
| `háromon`, `ezeren` | `?`, marginal | `hármon` 967 : 12; `ezren` 116,976 : 753 |
| `falvam` | `?`, marginal | 17 : `falum` 3,585 |
| RST12 `martinizza`, `martinizi` | `#`, nonce verb | 1-3 tokens |
| `agilisság`, `agilisség` | `?` | 7 and 3 tokens; replaced as the golden by `fotel` |
| `recenzeál` | `?`; `regisztrál`, `szelektál` carry `PH-HARM-020` | `recenzeálok` 1 : `recenzálok` 6; `regisztrálok` 2,299 |

### Starred forms (item by item)

| Star | Class | Change | Evidence |
|---|---|---|---|
| `ünnepekkor`, `órákkor` | wrong star | the condition restated, engine accepts | above |
| `képesti` | wrong star | accepted | 5,854 |
| `szavat`, `szavon` | register | `?` | `szót` 534,648 : 1,408; `szón` 4,588 : 57 |
| `szájam` | register (colloquial) | `?` | 1,488 : `szám` (of `száj`) 15,453 |
| `többedik` | register (colloquial) | `?` | 3,793 : `sokadik` 47,567 |
| `tartlak` | register | `?` (above `VRB-LINK-003`'s 1% flag line) | 91 : `tartalak` 5,730 |
| `száguldtak` | register, minority | `?` | 118 : `száguldottak` 1,723 |
| `lót`, `falva`, `lest`, `boldogon` | another reading | reading named in the text | verb `lót`; converb of `fal`; accusative of `les`; superessive of `boldog` |
| `énné`, `énként` | another reading | pronoun reading named; the noun `én` 'self' is regular | 212, 347 |
| `elei`, `mellei` | another reading | restated | possessed plurals |
| `házék`, `asztalkor` | semantic overgeneration | left open | needs a person class and a time-noun class |
| `kettedik`, `egyedik`, `annyik`, `ennyik`, `nekük`, `őként`, `házatom`, `házéul` | correct star | kept | noise level against the norm |

### Extractor note

In `PH-HARM-018`, the arrow pair `alibi`->`alibijének` was dropped: the extractor read any gap
holding a `:` and a digit as a count comparison, and `RST12 ...:` qualified. A comparison is now a
gap of counts and a separator only, and a table cell's forms end at the first prose gap, as an
arrow list's do.

## Fixes: chapter 06

Each star of the chapter was classified (string does not exist / wrong derivation of a valid
string / wrong star / register) and the rule text was rewritten to the condition the evidence
supports. Counts are `[WC2]` form frequencies; "all forms of the stem" sums every form with that
prefix.

### Rule conditions

| Rule | Was | Now | Evidence |
|---|---|---|---|
| `DER-ADJ-019`, `DER-ADJ-020` `-hAtÓ` | transitive base; `*élhető` | the head is the object, or the place or path argument of an intransitive | `élhető` 44,897, `járható` 61,472, `lakható` 12,977, `síelhető` 1,102 |
| `DER-ADJ-017` `-Ós` | no `-Ós` from non-derived agentive verbs | the star holds for 'prone to V'; 'involving V-ing' is productive from activity verbs; `repülős`, `kötős` are `-Ó` noun + `-(O)s` | `beszélgetős` 16,113, `lövöldözős` 8,241, `túrázós` 1,373, `ballagós` 140; statives `tartalmazós` 2, `gyűlölős` 3 |
| `DER-ADJ-010` `-AtlAn` | resultative transitives; transfer verbs excluded | the object exists apart from the event, which excludes creation verbs; transfer verbs regular; stative items listed | `kifizetetlen` 6,020, `fizetetlen` 2,259, `íratlan` 18,804; `alkotatlan`, `alapítatlan` 0 |
| `DER-VERB-014` `-gAt` | stative bases excluded | the base needs a dynamic reading | `szeretget` 6,234 (all forms; `szeretgetni` 1,094 alone), `tudogat` 181, `ismerget` 150, `emlékezget` 70; `gyűlölget` 1, `birtokolgat` 2, `tartalmazgat` 4 |
| `DER-VERB-008` `-ít` | "detailing" and "absolute-property" adjectives excluded; non-basic colours excluded | no such class (`butít` 12,454, `szelídít` 10,877, `okosít` 6,138, all forms); the colour verbs are a closed list, which also explains `*bordósít` against the rule's own `-sít` clause | `lilul` 307 : `lilít` 2 against `barnul` 3,875 : `barnít` 456: no phonological or stratal split, so a list is the last resort |
| `DER-ADJ-002` `-i` | material, product, body-part, animal, abstract nouns excluded | blocked on simplex bases by the N + N compound; compound-final members take `-i`; anatomical and generic readings regular; acronym `-i` marginal (MTA accepts `MSZP-i`) | `közéleti` 144,988, `tankönyvi` 9,876, `hasi` 68,785, `nyaki` 36,805, `füli` 1,218 |
| `DER-DIM-004` | concrete vs abstract | count vs mass | `hibácska` 99, `ügyecske` 50, `munkácska` 45; mass `vizecske`, `pénzecske` lexical |
| `DER-PREF-003` | no reduplication with accomplishment verbs | productive on prefixed verbs; the rule's own `meg-megáll` and AkH12 §141 c) contradicted the exclusion | AkH12 §141 c) `meg-megállt`, `vissza-visszatérés` |
| `DER-COMP-015` | `*rohan-szalad` unproductive | AkH12 §101 b): an occasional pair of same-role words carrying the same suffix is free (`rohantak-szaladtak`); a bare pair lives only as a listed compound (not `apró-nagy`), so `?rohan-szalad` | AkH12 §101 b) |
| `DER-COMP-002` | `*virágmézek` | a mass head takes the sort plural | `virágmézek` 150 |
| `DER-NOUN-003` | `*alkalmazottság`, `*nézőség` (collective) | the strings are the productive 'being an N' reading; only the collective is excluded, stated without a starred token | `alkalmazottság` 42, `nézőség` 64, `nézőségem` 6 |
| `DER-NOUN-004` | `*aranyosság` beside its own exception `aranyosság` | `aranyos` 'containing gold' forms none; the string is 'cuteness' | `aranyosság` 169 |

Stars of class "string does not exist" stay `*`; unattested words a corrected condition predicts
became `#` (potential: `ajándékozatlan`, `adományozatlan`, `agyonveretlen`, `agyoncsapatlan`,
`jámborít`, `konokít`, `fösvényít`, `kacérít`, `huncutít`, `cingárít`, `jogocska`, `levesevős`);
register forms became `?` with a label (`perci`, `könyvi`, `életi`, `MSZP-i`, `PM-i`, `koleszi`,
`napilag`, `kocsicska`, `szívész`, `tüdész`, `létezteti`, `rohan-szalad`); `?` and `#` forms are
neither goldens nor rejects (`goldens.rs` `unmarked`). Stars that forbid a reading of a valid string
name the reading (`*aranyi` 'golden', `*napbeli` 'of the day', `*olvasóak` as the noun's plural,
`*gombamérgező` 'poisoner using mushrooms', `*tetőház` in the part reading). `*fülei` was dropped:
it is the possessed plural and the `-i` adjective of `fül` is `füli`. `*zsürii` was respelt
`*zsűrii`. Goldens: `Chomsky -> chomskysta` (0, unsourced; the used form is `chomskyánus` 139) was
dropped, and `ért -> értetlen` moved to the stative lexical items, its meaning being active.

### Engine

- `-Ós` (`DER-ADJ-017`): the participle filter takes every verb's participle, a potential verb's
  excepted. The lexicon marks no verb stative, so the event reading cannot be confined to activity
  verbs; the cost is a handful of statives (below).
- `-(O)gAt` (`DER-VERB-014`): a medial suffix needs a base before it, so a monosyllable is a root,
  and a listed noun plus `-(O)z` is denominal: `emlékezget` is accepted, `ölelkezget` is not.
- Prefix reduplication (`DER-PREF-003`) is read from a hyphenated token whose first member is the
  preverb of the prefixed verb after it (`meg-megjön`); 49 such verbs were only listed before.
- Coordination (`DER-COMP-015`): a hyphenated pair whose members share category and feature stack
  and the first carries a suffix (`rohantak-szaladtak`, `sütnek-főznek`).
- `-né` as a semi-word ignored `DER-MODEL-005`'s lengthening, so `katonané` and `asztalné` were
  read as compounds; a semi-word spelled as a lengthening suffix no longer follows a short `a`/`e`.
- `xtask eval` counted the derived citation reading (`futós` as its own lemma) as a lexicon entry,
  which is where `repülős`, `futós`, `úszós`, `kötős`, `filmnézős` and `alkalmazottság` showed up
  as lexicon conflicts. None is a lexicon entry; only a real entry now counts as listed.

### Measured

| Set | Before | After |
|---|---|---|
| goldens reproduced | 2130/2164 | 2135/2169 |
| starred forms kept out | 186/284 | 180/240 |
| example forms accepted | 12320/12506 | 12335/12521 |
| ledger covered / partial / unmeasured | 780 / 153 / 293 | 785 / 146 / 295 |
| clean-text regression (`nk20k`) | - | 0 newly rejected, 0 repair proposals |

Overgeneration (paradigms of 32 verbs): the new `-Ós` forms a speaker rejects are the stative ones,
`tartalmazós` 2, `gyűlölős` 3, `létezős` 4, `hasonlítós` 11; `fedeget` (0) is a potential word.

### Left open

- Missing lexical properties: verb `stative` (`-Ós` event reading, `-gAt`, `-AtlAn`),
  `effected-object` (`-AtlAn`: `alkotatlan` is still formed), the argument frame of `-hAtÓ`
  (only `transitive` is carried, so every verb generates), noun `count`/`mass` (`DIM-004`;
  morphdb's `sing` is singulare tantum, not mass), the closed list of colour verbs (`-ít` is formed
  on `bézs`, `drapp`, `lila`), a hypocoristic flag on `i`-final nouns (`DIM-001`: `-cskA` stays
  refused on every `i`-final noun, so `?kocsicska` 29 and `zoknicska` 63 are rejected while
  `*csokicska` stays out), and the functional person noun of `DER-NOUN-003` (`Occupation` is
  carried by five nouns, so `nézőség` 'being a viewer' is rejected).
- `szívész`, `tüdész`: colloquial members of an unproductive suffix; the lexicon has no register
  tag to list them under, so they stay rejected.
- `kecskész` is read as the compound `kecs` + `kész`: compound overacceptance, not a lexicon
  entry.
- `?rohan-szalad`: AkH12 gives no bare verb pair outside listed compounds; MTA accepts it.

## Fixes: chapters 07-08

Date: 2026-11-08. Evidence: AkH12 text (`helyesiras.mta.hu/helyesiras/default/akh12#<N>`).

### Rule text

| Rule | Was | Now | AkH12 |
|---|---|---|---|
| `ORT-NUM-010` | a spelled-out year hyphenated (`kétezer-tizenhat`), range `2002-2012-ben` | a year is solid at any size; a range takes the en dash, suffix on the second member (`1848–49-ben`); the e-nyelv advisory that hyphenates years is not followed (`OQ-07-018` rewritten) | §291 a) "kivéve az évszámokat", "de: kétezertizennégy májusában"; §264 c), §296 |
| `PUN-NUMF-016` | score colon spaced on both sides, `2 : 1-re` | `6:3` and `6 : 3` both accepted, neither required: the prose says "egyik irányban sem tapad", the only example is `6:3` | §240 f) |
| `ORT-SUFX-002` | a closed, corpus-measured list stated as the norm, with the constructed `alfaszor`, `goetheizmus` | the principle: lengthening is the default and non-lengthening a suffix property; AkH12 names `-i`, `-ság/-ség` and the semi-words; the rest of the list is measured evidence; examples from AkH12 (`goethei`, `campanileszerű`) | §29, §216 a) |
| `PUN-QUOT-005` | `*„Q”, mondta` | the comma is outside the rule, not starred; only the dropped period is explicit | §257 b) |
| `PUN-QUOT-012` | every utterance opens a line | a new speaker opens a line; after a narrator clause the speech goes on in the same line | §259 c), d) |
| `ORT-ASSIM-020` | `játszik`, `hallatszik` "only tssz" | labelled an inference from §32 naming three verbs | §32, §80 |
| `ORT-TOG-008` | `*apró-nagy` | "no such compound" is a lexicon fact licensing the hyphen of `apraja-nagyja`, not a starred string | §101 b) |
| `ORT-LEN-017`, `-023`, `-024` | `-ik` verbs cited as bare stems (`máll`, `torkoll`) | citation forms (`mállik`, `torkollik`) | §64 |

### Engine

- Phrase written solid (`ORT-TOG-036`, `ORT-TOG-037`): a case-marked nominal plus a verb or
  adjective head is one word when the head carries two lexeme-forming steps
  (`házhozszállítási`) or `leg-` turns its comparative into a superlative
  (`legszívhezszólóbb`). One step, or a bare comparative, finds no reading (`szívhezszólóbb`
  stays rejected). The seam is searched at every character boundary, because `házhoz|szállítási`
  falls inside the letter `zs`. The dependent is tested against the case stacks alone: with the
  full search space the path cost 35% on `check` and doubled `repair`; with the case stacks the
  cost is within noise (16.7 s to 17.0 s, 35.2 s to 36.1 s on nk20k).
- A compound head's category is the output of its last derivation, not its base
  (`trafótekercselés`, `ORT-HYPH-005`, §289 b)). The head test read the verb `tekercsel` and
  refused it.
- The negative prefix geminates only before the `mi-` bases (`semmi`, `semmilyen`); before
  other bases it is `se`, `m`-initial ones included (`semekkora`, §123). `*semmekkora` was
  accepted before and is rejected now.
- A spelled-out numeral above 2000 parses in both spellings, the amount and the year (§291 a));
  a token cannot tell which it is.
- A spoken name's derivative that lengthens the final vowel lost its tail, because derivation
  reports no seam: `Goethe` [gőte] gave no `goethés` (§216 a)). The lengthened spoken stem is
  now stripped too.
- An en-dash pair whose last member is the lowercase derivative of a name lowercases its other
  members (`budapest–bécsi`, `moszkva–párizsi`, §264 c), §161). Checked in `check_text`, since
  the tokeniser splits at the dash.
- Lint: a dash at a line start after a newline was flagged by `PUN-DASH-012` as missing a space,
  so every dialogue turn after the first was flagged (§259 d)). A line edge on either side now
  exempts it.
- Extractor: letters in parentheses glued to a word are an optional ending and the full form is
  kept (`fehérl(ik)` gives `fehérlik`, `-képp(en)` gives `-képpen`).

### Measured

| Set | Before | After |
|---|---|---|
| goldens reproduced | 2130/2164 | 2130/2164 |
| starred forms kept out | 186/284 | 186/283 (`apró-nagy` no longer starred) |
| example forms accepted | 12320/12506 | 12346/12507 |
| chapter 07 examples | 2419/2492 | 2438/2493 |
| chapter 08 examples | 62/64 | 64/64 |
| lint texts unflagged | 143/143 | 145/145 |
| ledger covered | 780 | 792 |

Clean text (nk20k): 0 newly rejected, 2 newly accepted (`állatábrázolás`,
`lapockaveregetéssel`, both derived-head compounds), 0 repair proposals, lint findings 1606
before and after, identical.

### Open

- Meaning-dependent solid spellings stay lexical: `tizenkétszög` (§105), `tízezerforintos`
  (§117, the banknote), `desztillálótorony` (§118, fused meaning), `méghogy` (§107). A participle
  member is admitted only when listed, which is what keeps the meaning test lexical.
- `leghasznothajtóbb`: `hajtó` has no comparative in the lexicon.
- Name-derived forms that need the name listed: `tóthpáli` (`Tóthpál`), `helirbeli`, `mavados`
  (the blends `Helir`, `Mavad`), `wrocławi` (unlisted name, accepted only as unknown).
- `petőfieskedők`: `-kOdik` is not a productive derivation (chapter 06).
- `kinyittatik`, `oltatik`: the passive `-tAtik` is not generated.
- Lexical gaps, not fixed: `mesd`, `messe`, `tessem`, `lássék` (§32 doublets), `keveslem`,
  `kicsinyli`, `kékellik`, `zöldellt` (§64-§65 doublets), `szók`, `varjúk` (§30), `árbóc`,
  `biennálé`, `hozsánna`, `immunis`, `reverzibilis`, `spré`, `szpré`, `ímél`, `pészméker`,
  `löncsöl`, `barrelnyi`, `kollegialitás`, `pasztőroltás`, `henry`, `newton`, `pascal`.
- Foreign-spelled words and transliterations are correct but unlisted (§212-§213, §217-§222):
  `couchette`, `crescendo`, `kapitälchen`, `myocarditis`, `rinascimento`, `señor`, `air`,
  `street`, `bektási`, `hidzsra`, `khitón`.

## Fixes: names and repair

Date: 2026-11-08. Evidence: AkH12 §172, §175, §179, §181; MTA advisor; the code pages as
`encoding_rs` and `oem_cp` define them; Webcorpus 2 counts.

### Names (chapter 07)

- `tó` -> `tavi` (`ORT-NAME-022`, §175): `PH-LEN-002` names `tavi`, `havi`, `művi` as `-i` on the
  v-stem's bound alternant, lexicalised. The engine gives that form as a variant of the `-i`
  cell wherever the bound alternant plus `-i` is a listed adjective, and a name's derivative
  takes its head member's (`Huron-tavi`, `Szent Anna-tavi`, `velencei-tavi`). `lovi`, `kövi` are
  not listed, so they are not formed. The evaluator's bare-entry path accepts a variant among
  the cell's forms, as the listed-entry path already did.
- `Ipoly Eurorégió` -> `Ipoly eurorégióbeli` (`ORT-NAME-028`, §179): two gaps. `eurorégió` was
  unanalysable, so it counted as a name and kept its capital; MTA accepts `eurorégió`,
  `eurozóna`, `euroszkeptikus`, `eurobajnok` and rejects `euro` alone, so `euro` is a bound
  first member (`BoundMember`, a lexical fact). And a semi-word after a multiword name
  hyphenated the name as a proper one; §179 writes `-i` and `-beli` alike, so the semi-word now
  takes the element casing of the `-i` derivative.
- `Csörsz-árok` -> `Csörszárok utca`, `Ferenc-hegy` -> `Ferenchegyi út` (`ORT-NAME-030`, §181):
  a capitalised word hyphenated to a lowercase common noun before a public place generic is
  written solid, bare or with `-i` on the head. The checker accepts the solid form in that
  position and rejects the hyphenated one there. The generics are a semantic class, so they are
  listed (`PublicPlace`: the §181 words plus the rest of the class).
- `Che` -> `Chet` (`PH-LEN-003`): the rule names monosyllabic loans in short `e`. No native
  content word has that shape (the monosyllables in `e` are `de`, `ne`, `le`, `be`, `se`), so
  the condition is formal: a one-vowel stem ending in `e` does not lengthen. `Ocse` still gives
  `Ocsét`. The pronounced-ending mechanism of §216 a) (`goethés`) does not apply: `Che` is
  pronounced vowel-final and would lengthen under it.

### Repair (chapter 10)

- Code pages: the hand-held CP852 table is replaced by `oem_cp`'s, and CP437, CP850 and
  ISO-8859-1 proper are added (`encoding_rs` maps the ISO-8859-1 label to windows-1252, which
  has no C1 controls). `t√z` -> `tűz` is CP852 read as CP437; `Å` U+0091 `sz` -> `ősz` is
  UTF-8 read as ISO-8859-1 (`REP-FONT-010`).
- `REP-ENC-007`: the reverse table is derived (ISO-8859-2 and windows-1250 encode,
  windows-1252 decodes) and the hand table `WesternAsCentral` is gone. The rule's example
  `Sőo Paulo` was wrong: Latin-1 `ã` (E3) is Latin-2 `ă`, so the damage is `Săo`; `ő` comes from
  `õ`. With no foreign lexicon, a run of rejected tokens is restored only when it mixes `ő`/`ű`
  with an image letter Hungarian never writes. A first version counted any two image letters
  and rewrote `leértékelődéséből` (twice in nk20k): a rejected word with `ő` is far likelier an
  unlisted Hungarian word. A lone `flűte`, `Espańa`, `Săo` stays as written, and their goldens
  are reproduced by the derived table only.
- `REP-ENC-011`: the best-fit map is derived (the letters windows-1252 cannot write, each to its
  base). A text with the other accents but no `ő`/`ű` over 200 `o`/`u` letters gets them back on
  rejected tokens, commonest candidate first, abstaining on a tie.
- `REP-ENC-013`: TeX accents (`\H{o}`, `\'a`, `{\'a}`, `\'{\i}`) are decoded from the plain TeX
  marks. The golden read `\Hr` because the extractor stripped a brace inside a code span as a
  gloss; braces are literal inside code now.
- `REP-DIAC-003`: the candidate set is exposed as alternatives (`accent_alternatives`); an
  accepted `kerek` is never rewritten.
- `REP-DIAC-012`: an all-capital word with several accepted restorations is ranked by frequency
  as a lowercase one is (`FELTETELEK` has `feltételek` and the rare `féltételek`).
- `REP-SPACE-009`: the fused string had to be unattested, which kept `hogya` (12,116 in
  Webcorpus 2). The rule asks for the pair to be at least as frequent; with unigram buckets the
  pair is estimated as chance co-occurrence, `f(a) + f(b) - 45`, the offset fixed by the article
  `a` being 9.2% of the tokens at bucket 38. `hogy a` scores 25 against 5; `egyazon` (6 against
  9) and `mindenkorra` (0 against 8) stand.
- `REP-KEY-007`: `köszönnöm` is a word (personal infinitive), and the rule leaves real-word slips
  to context. A personal infinitive needs an impersonal governor (`VRB-INF-002`), so as a
  sentence on its own it has no reading and the slip repair runs. In a longer sentence it is
  left alone.

### Measured

| Set | Before | After |
|---|---|---|
| goldens reproduced | 2133/2167 | 2152/2167 |
| chapter 07 goldens | 197/203 | 203/203 |
| chapter 10 goldens | 284/311 | 296/311 |
| example forms accepted | 13176/13498 | 13176/13498 |
| starred forms kept out | 179/233 | 179/233 |
| ledger covered | 817 | 829 |

Clean text (nk20k): 0 newly rejected, 0 repair proposals. `picur-morph repair` on nk20k: 37.4 s
before, 40.6 s after.

### Open

- `REP-DIAC-004` `meg nem jott` -> `még nem jött`: the rule's context term is a language model;
  unigram frequency prefers `meg` (29 against 27) and no bigram data is available.
- Hyphenated name derivatives with unlisted members (`bödd-bakodi`, `csörnöc-herpenyői`,
  `öreg-cselőtei`, `alcsi-holt-tiszai`, `zöld-foki-szigeteki`, `zala–somogyi-határároki`,
  `schöpf-mereis`): `Bödd`, `Csörnöc`, `Herpenyő`, `Cselőte`, `Alcsi` are not in the lexicon,
  and the three-member `Alcsi-Holt-Tisza` is rejected outright.
- Foreign names whose ending is silent or spoken otherwise (`bordeaux-i`, `lille-i`,
  `sydney-i`, `monsieur-nek`, `toulouse-lautreci`, `châlons-en-champagne-i`, `ploieşti-i`):
  `ORT-FOR-012` makes the pronunciation lexical, and these have no `Spoken` entries.
- `Makk-kal` (an unlisted family name homographic with `makk`), `fogarasi-havasokbeli` (`-beli`
  after a plural head is not analysed), `barbecue-szósz`, `couchette-rendelés`, `gizes-gazos`.
- Abbreviation plus suffix (`u.-ban`, `cm-rel`, `°-os`, `tv-zik`) and addresses: not attempted.

## Fixes: derivation logic

The derivations chapter 06 states but the engine lacked, and the conditions on a derivation's
base that the rules state or the corpus supports. Each condition is a formal one: the shape of
the base, its own derivational structure, or blocking by a rival word. `[WC2]` counts are lemma
sums over all forms; "types" are lemmas of at least the stated number of tokens.

### Missing derivations

| Rule | Was | Now | Evidence |
|---|---|---|---|
| `DER-VERB-020` passive `-tAtik` | listed items only (`adatik`, `íratik`) | the causative's allomorph plus `-ik` on a transitive verb, third person, indefinite | 987 `-tAtik` types of at least 5 tokens: `foglaltatik` 16,038, `megnyittatik` 1,071, `oltatik` 48, `kinyittatik` 25 |
| `DER-VERB-012` `-kOdik` | listed items only | on an adjective whose final `s` follows a vowel (an `-(O)s` adjective) or a privative; a noun's `-(O)skOdik` is `-(O)s` plus `-kOdik` | 556 `-(O)s` + `-kOdik` and 158 privative + `-kOdik` types of at least 5 tokens: `fontoskodik` 2,611, `bizonytalankodik` 4,183, `nagyoskodik` 6, `egyénieskedik` 517 |
| `DER-ADV-004` `-lAg` | `-i` adjectives only | also present participles | 399 `-Ó` + `-lAg` types of at least 20 tokens: `serkentőleg` 2,519, `bátorítólag` 618, `hallgatólag` 277; the `-Ú` bases (73 types, mostly `-szerű`) stay lexical |
| `DER-ADJ-006` `-ias` | only on a listed `-i` adjective | the `-i` + `-(O)s` stack | 595 `-ias`/`-ies` types of at least 20 tokens (foreign plurals included), among them unlisted `kórházias`, `fővárosias`, `mérnökies`, `szabolcsias` |
| `DER-VERB-011` `-(A)ll` | `rövidell` missing | listed (lexical rule) | `rövidell` 31; `nagyall` 0 and `olcsóll` 1 are `#`, MTA knows neither |
| `NOM-CASE-021` `-ntA` | `éjjelente` missing; `hónaponta` called unattested | listed (lexical rule): `éjjelente` 9,857, `éjente` 181, `hajnalonta` 975, `hónaponta` 421, `vasárnaponta` 276, `hétköznaponta` 114, `hétvégente` 6,710 | `koronta` 0 is `#` |

`-tAtik` and `-kOdik` are `P`/`P-dom` with a formal domain, so they generate. The passive
conjugates in the third person only and `-kOdik` in the indefinite only, both being intransitive;
restricting the cells also keeps the checker within its time budget.

### Conditions on the base

| Rule | Star | Condition | Evidence |
|---|---|---|---|
| `DER-ADJ-006` | `*pihenésies` | `-(O)s` takes no `-i` adjective built on a deverbal `-Ás` noun; the base is recognised by regenerating it from a listed verb | `-Ás` + `-i` + `-(O)s` under 200 tokens in all, against abstract bases `ünnepies` 1,165, `irodalmias` 780; `-Ás` + `-(O)s` itself is productive (`hatásos`, `tárgyalásos`) |
| `DER-ADJ-006` | `*stabilitásos` | the `-itás` quality noun of a listed adjective is blocked by that adjective (root, root with its last vowel long, `-is`, `-ív`) | `stabilitásos` 6 against `stabilitás` 187,552; `kreativitásos` 12, `aktivitásos` 5; with no rival adjective `annuitásos` 3,733, `paritásos` 2,184, `prioritásos` 1,711, `kvalitásos` 1,557 |
| `DER-ADJ-006` | `*kossuthias` | a name's `-i` adjective takes `-(O)s` only where it is written as the name, an `i`-final one | `petőfies` 33, `jókaias` 22, `széchenyies` 17 against `kossuthias` 2, `bartókias` 1, `móriczias` 1 |
| `DER-ADV-010` | `*szépenes` | an `-An`, `-Ul`, `-képp`, `-stUl` or `-ntA` adverb takes no suffix, also where the lexicon lists it as an adjective (`szépen`); the base is recognised by regenerating it from a listed word; a postposition is exempt (`belüli`) | the rule's own statement |
| `DER-VERB-012` | `*okosaskodik` | `-(O)skOdik` is a noun's; an adjective takes `-kOdik` directly | `okoskodik` 23,645 |

The `DER-ADJ-006` time-noun exclusion was wrong as a class: seasons take `-ias` (`nyárias`
12,714, `tavaszias` 10,429, `őszies` 5,747). The rule now excludes time units, which is a
semantic class, so `*napias` (11) stays formed.

### Measured

| Set | Before | After |
|---|---|---|
| goldens reproduced | 2133/2167 | 2133/2167 |
| starred forms kept out | 175/233 | 179/233 |
| example forms accepted | 12377/12532 | 12387/12534 |
| chapter 06 starred kept out | - | 93/133 |
| ledger covered | 798 | 804 |
| clean-text regression (`nk20k`) | - | 0 newly rejected, 0 repair proposals |
| roundtrip 20 verb / noun | - | 100% / 100% |
| `picur-morph check` on `nk20k` | 17.4 s | 18.7 s (+8%) |

Newly accepted examples: `éjente`, `éjjelente`, `hajnalonta`, `hétvégente`, `hallgatólag`,
`kinyittatik`, `oltatik`, `rövidell`. Newly rejected stars: `pihenésies`, `stabilitásos`,
`szépenes`. No example form is newly rejected.

Overgeneration (paradigms of 23 lemmas): the passive forms of every transitive verb
(`olvastatik`, `hallgattatik`) are attested in the register; `-kOdik` and `-ias` give potential
words on nouns that are also adjectives (`gyorsoskodik`, `pirosias`, `asztalias`), since the
human-property domain the rule names is semantic.

### Left open

- `*napias`, `*tavaszbeli`, `*napbeli` 'of the day': time-unit nouns are a semantic class.
- `*távol-keletbeli`: the locative type of a place name (`NOM-PROP-006`) is not in the lexicon.
- `*abbahagyatlan`, `*alapítatlan`, `*alkotatlan`, `*kedveletlen`, `*tartalmaztat`: event
  objects, effected objects and stativity are not carried (listed under chapter 06).
- `*jobbpofa`, `*tetőház`, `*gázmérgező`, `*gombamérgező`: the star forbids a reading of a valid
  string (comparative `jobb`, the part reading, the instrument reading); no string condition
  applies.
- `DER-ADJ-002` `-i` on bare nouns (`*aranyi`, `*fai`): semantic classes, unchanged.
- `hollywoodias` (258): a place name's unlisted `-i` adjective takes `-ias`, but the engine cannot
  tell a place name from a personal one, so consonant-final names are refused as a whole.
- `hetentei`: `-ntA` on a shortening stem (`hét`) is not recognised as terminal.
- The noun half of `DER-VERB-012` (bare `-kOdik` on occupation nouns, `tanárkodik`) needs the
  `Occupation` trait, which five nouns carry.

## Fixes: inflection logic

Date: 2026-11-08. Evidence: WC2 (`webcorpus2.sqlite`, form:lemma counts), AkH12, MTA checker
(over-accepts, used only as a lower bound), project doublet measure (both forms at 20 or more
and within a tenfold margin).

### Lowering on derived adjectives

`DER-MODEL-010` says an adjective output lowers; `Der.out_lowering` was false for `-Os`
(`DER-ADJ-005`), `-i`, `-Ú` and `-nyi`, so `*sóst`, `*ködösök` were generated. They follow the
master table now. `-hAtÓ` is optional in the table and stays unmarked. Adjective semi-word heads
(`-mentes`, `DER-ADJ-016`) lower and take the linking plural. The noun `mentes` came from the
converter alone: WC2 has no noun form but `mentest`, the unaccented `mentést`; it made every
`-mentes` compound a non-lowering noun and is now dropped (`NOT_A_LEMMA`).

Spot-check of derived adjectives in WC2: the plural lowers almost without exception
(`ködösek` 660 : `ködösök` 3); the bare accusative of `-Os` is a minority that reaches parity on
some stems (`sóst` 302 : `sósat` 2568, `csontost` 89 : `csontosat` 82, `ködöst` 17 :
`ködöset` 12; `gluténmentest` 68 : `gluténmenteset` 211). The engine follows AkH12 §71
(`erőset`, `magasat`) and `DER-ADJ-007`; the bare minority stays `OQ-03-020`.

### Accusative after clusters

`NOM-ACC-006`, `-007`, `-009` and `PH-SYL-008`, `-009` starred or required forms against the
corpus. Measured: `pajzsot`, `*pajzst` 22; `fajanszot`, `?fajanszt` 9; `protestánst` 341 :
`*protestánsot` 1; `nonszenszt` 69 : 0; `?gennyt` 853 : 50; `*orrt`; `*oldalat` 272,452 :
1,066. The engine's condition (linking vowel by default, bare as a lexical property) matches the
rule; the defects were the doc rows and the missing `BareAccusative` on `protestáns` and
`nonszensz`. A phonotactic guard on bare `-t` was tried and dropped: letter-read acronyms
(`hl`, `tsz`) take a bare `-t` after any cluster, and every native-spelled `BareAccusative` entry
already satisfies `PH-SYL-008`.

### `falva`

The converter mapped the possessive doublet list (`utóda ~ utódja`) onto `AlternationOptional`,
which also licenses the free-stem plural doublet (`faluk ~ falvak`). `falu` thus read `falva` as
its possessive. The two choices are now separate traits: `PossessiveOptional` for `-A ~ -jA`,
`AlternationOptional` for the stem. The possessive axis is no longer applied to verbs (110
verbs carried possessive traits). `falva` reads only as the converb of `fal`; `fiú` keeps
`fia ~ fiúja` (`NOM-POSS-009`) through the supplement.

### Single forms

- `boldogak`: 3,534 against `boldogok` 90,431, marked `?`.
- `májak`, `májat`: the abbreviation row `máj.` replaced the noun `máj` in the supplement, as did
  those of `dag`, `prof`, `áll`. The store now keeps several supplement rows of one lemma and
  part of speech; only an abbreviation and its homograph word may share a key.
- `lassúbb`: 11,354 beside `lassabb` 60,918, a doublet by the project measure; `hosszúbb` 19,
  `könnyűbb` 90, `ifjúbb` 4 are not. `NOM-ADJ-005` is lexical, so the doublet is a one-item list
  (`Role::ComparativeDoublet`) read by an analysis variant that builds the regular comparative,
  superlative and excessive.
- `utai`: 241 against `útjai` 28,629; AkH12 §27 names only `útja`, `útjuk`. Marked `?`.
- `egyéként`: 895 in WC2, all `egyébként` with the `b` dropped; `máséként` 6,
  `mindenkiéként` 0. Starred.

### `házatom`

No list licenses it: it is the compound `ház|atom` (`DER-COMP-023`). `NOM-MODEL-003` now
illustrates the stack order with `*házbanom` and the segmentation `*ház+at+om`.

### Measured

| Set | Before | After |
|---|---|---|
| goldens reproduced | 2133/2167 | 2132/2166 |
| starred forms kept out | 175/233 | 187/242 |
| example forms accepted | 12377/12532 | 12389/12526 |
| lint texts unflagged | 145/145 | 145/145 |
| ledger covered | 798 | 807 |
| roundtrip noun / adjective / verb | 100% each | 26015/26015, 45279/45279, 19164/19164 |

Clean text (nk20k): 0 newly rejected, 2 no longer rejected, 0 repair proposals.

### Open

- Bare `-t` on `-Os` and `-mentes` adjectives (`OQ-03-020`).
- Compound readings over-accept starred strings: `mentest` (`men|test`), `kertest`.
- 14 high-confidence bare-accusative nouns of the raw list are missing from the verified list.
- The `-rét` semi-word is `n` in the master table and lowers now; untested, rare.
- `BareAccusative` on `oláh`, `pléh`, `jobb` and similar is unverified.

## Open

Lexical properties the engine lacks, each blocking a formal condition already stated in the rule:
- verb stativity (`DER-ADJ-017` `-Ós`, `DER-VERB-014` `-gAt`, `DER-ADJ-010` `-AtlAn`): stative
  `tartalmazós`, `létezős` are generated;
- effected object (`DER-ADJ-010`): `alkotatlan` is generated;
- the argument frame of `-hAtÓ` (`DER-ADJ-020`);
- noun count/mass (`DER-DIM-004`);
- a person class and a time-noun class (`házék`, `asztalkor` are accepted);
- a functional-person trait (`nézőség` is rejected) and a hypocoristic flag (`kocsicska`);
- the colour-verb list of `DER-VERB-008` is incomplete (`bézsít`, `lilít` are generated).

Hyphenation (`ORT-SPLIT`), measured once the segmenting range was fixed. Seams now come from each
reading's segmentation, a seam replaces the syllable break between its two vowels, `ch` is one
consonant at a line break, and a homograph has one hyphenation per reading. Still missing:
- the parts of listed compounds (`csak-is`, `vagy-is`, `rend-őr`, `Nyír-egyháza`): lexicon data;
- historical name digraphs (`Bat-thyány`, `Ben-czúr`, `Beö-thy`), which need the name's
  pronunciation (`ORT-NAME-003`);
- foreign words split by their pronunciation (`Wa-shing-ton`, `cou-chette`) and learned elements
  (`foto-gráfia`, `infra-struktúra`).

Cost: on 3,600 sentences `picur-morph check` went from 17.2 s to 19.3 s and `picur-morph repair` from
36.1 s to 43.5 s over the logic round (new derivations and repair probes, each within its own
budget). Repair calls the checker per candidate, so it carries every analysis cost twice over.

Other:
- Bare stem-slot values (`forg`, `alsz`, `feksz`) remain in `accepts.ron`: the same rows are
  stem-extraction goldens.
- Chapter 07 examples that need meaning (`tizenkétszög`, `méghogy`), unlisted names and foreign
  words, the passive `-tAtik` and `-kOdik`: listed under "Fixes: chapters 07-08".
