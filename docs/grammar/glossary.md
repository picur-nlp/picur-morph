# Glossary

Hungarian grammatical and typographic terms used in the rulebook, with the English term used in rule text and the Universal Dependencies (UD; hu_szeged treebank usage noted where it differs) / UniMorph mapping. `Rule areas` lists the chapter prefixes that use the term. Where chapters used different English terms, one is chosen and the others follow as `also`.

| Hungarian term | English term | UD / UniMorph mapping | Rule areas |
|---|---|---|---|
| ábécé | alphabet | - | `WS` |
| ajakkerekítéses (labiális) / ajakréses (illabiális) | rounded / unrounded | - | `PH` |
| alany | subject | `nsubj` | `SYN` |
| alanyeset | nominative | `Case=Nom` / `NOM` | `NOM` |
| alanyi (általános, határozatlan) ragozás | indefinite (subjective) conjugation | `Definite=Ind` / `INDF` | `VRB`, `SYN` |
| alaptag | head | - | `DER` |
| alárendelő szókapcsolat | subordinate phrase | - | `ORT` |
| államnév, közterületnév | state name, public place (street) name | - | `ORT` |
| állandó cím, egyedi cím | permanent (serial) title, individual title | - | `ORT` |
| állandó jelző | fixed epithet | - | `ORT` |
| állapothatározó -ul/-ül (essivus) | essive | `Case=Ess` / - | `NOM` |
| állatnév, tárgynév | animal name, object name | - | `ORT` |
| állítmány (névszói) | nominal predicate | head of clause; copula `cop` when overt | `SYN` |
| általános latin betűs ábécé | general Latin alphabet (26 letters) | - | `WS` |
| általános névmás | universal pronoun | `PronType=Tot` / - | `PND` |
| AltGr-billentyű | AltGr key | - | `REP` |
| antiharmonikus (veláris i-t tartalmazó) tő | antiharmonic stem (`híd` type) | - | `PH` |
| anyagnév | mass (material) noun | - | `DER` |
| anyagnévi jelző | material attribute | - | `ORT` |
| arab szám | Arabic numeral (digits) | `NumForm=Digit` / - | `PND` |
| archifonéma | archiphoneme | - | `PH` |
| asszonynévképző (-né) | 'wife of' suffix | - | `DER` |
| átírás | transliteration (also: transcription) | - | `PH`, `ORT` |
| átlátszó / átlátszatlan | transparent / opaque | - | `PH` |
| beálló melléknévi igenév (`-andó/-endő`) | future participle | `VerbForm=PartFut` / `V.PTCP;FUT` | `VRB`, `DER` |
| becéző képző | hypocoristic suffix | - | `DER` |
| befejezett melléknévi igenév (`-t/-tt`) | past participle | `VerbForm=PartPast` / `V.PTCP;PST` | `VRB`, `DER` |
| belviszonyragok: inessivus, illativus, elativus | inessive, illative, elative | `Ine`, `Ill`, `Ela` / `IN+ESS`, `IN+ALL`, `IN+ABL` | `NOM` |
| betű | letter (unit of the alphabet) | - | `WS`, `ORT` |
| betűkészlet, font | font | - | `REP` |
| betűnév | letter name | - | `WS` |
| betűrend | alphabetical order | - | `WS` |
| betűrendbe sorolás | collation, alphabetisation | - | `WS` |
| betűzés | spelling out | - | `WS` |
| betűzési ábécé | spelling alphabet | - | `WS` |
| billentyűzetkiosztás | keyboard layout | - | `REP` |
| birtokjel | anaphoric possessive `-é` | `Number[psed]=Sing` / `LGSPEC1` | `NOM` |
| birtokos eset | genitive (possessor `-nAk`) | `Case=Gen` / - | `NOM` |
| birtokos jelző | possessor | `nmod:att`; `Case=Nom` or `Case=Dat` | `SYN` |
| birtokos névmás | possessive pronoun | `PronType=Prs`, `Poss=Yes` / `PSS` | `PND` |
| birtokos személyjel | possessive suffix | `Person[psor]`, `Number[psor]` | `SYN` |
| birtokos személyrag | possessive personal suffix | `Number[psor]`, `Person[psor]` / `PSS1S`..`PSS3P` | `NOM` |
| birtoktöbbesítő jel (`-i-`) | possessed plural marker | `Number[psed]=Plur` (hu treebank convention; verify) | `SYN` |
| birtokviszonyjel | general possessive marker `-(j)A` | - | `NOM` |
| cím, címsor | title, heading | - | `PUN` |
| címzés | addressing (postal) | - | `PUN` |
| cselekvésnév (-ás/-és) | action nominal | `VerbForm=Vnoun` (UD-hu) / - | `DER` |
| csillagászati név | astronomical name | - | `ORT` |
| csupa nagybetűs | all caps | - | `WS` |
| distributivus -nként | distributive | UD `Case=Dis` (7 treebank tokens, kept separate from `-ként`); our model `Case=Distributivus` | `NOM` |
| distributivus-temporalis -nta/-nte | distributive-temporal | - | `NOM` |
| dupla kódolás | double encoding | - | `REP` |
| egybeírás, különírás | writing together (solid), writing apart | - | `ORT`, `REP` |
| egyenes idézet, tartalmi idézet | direct quotation, content (indirect) quotation | - | `PUN` |
| egyes szám | singular | `Number=Sing` / `SG` | `VRB` |
| egyeztetés | agreement | `Person`, `Number` on verb | `SYN` |
| egyjegyű betű | single-character letter | - | `WS` |
| egyszerűsített kettőzés | simplified doubling (`ssz` for `szsz`) | - | `WS` |
| egyszerűsítő írásmód | simplification principle | - | `ORT` |
| ékezet | accent mark (acute, double acute, diaeresis) | - | `WS`, `REP` |
| ékezet nélküli szöveg | accent-stripped text | - | `REP` |
| ékezetvesztés | accent loss | - | `REP` |
| ékezet-visszaállítás | diacritic restoration | - | `REP` |
| elcsúszott betűkészlet | shifted font encoding | - | `REP` |
| elemezhetőség | analysable share | - | `REP` |
| élőfej, élőláb, oldalszám | running head, running foot, page number | - | `REP` |
| elölségi (palatális) harmónia | backness harmony | - | `PH` |
| előtag | first member (modifier) | - | `DER` |
| első, második, harmadik személy | first, second, third person | `Person=1\|2\|3` / `1`, `2`, `3` | `VRB` |
| elütés | typo, keyboard slip | - | `REP` |
| elválasztás, sorvégi elválasztás | line-end hyphenation (also: hyphenation, line-end word division) | - | `WS`, `ORT`, `REP` |
| elválasztójel | line-break hyphen | - | `PUN` |
| elvonás | backformation | - | `DER` |
| elvont főnév (-ság/-ség) | abstract noun | - | `DER` |
| endocentrikus / alárendelő összetétel | endocentric / subordinative compound | - | `DER` |
| eredményhatározó (translativus-factivus), átváltozás ragja (`-vÁ`) | translative | `Case=Tra` / `TRANS` | `PH`, `NOM` |
| értelmező | appositive | - | `PUN` |
| esetragos (ragvonzó) névutó | case-assigning postposition | `case` + complement `Case=Sup\|Ins\|All` | `SYN` |
| eszközhatározó eset, eszközhatározó rag (`-vAl`) | instrumental-comitative (also: instrumental) | `Case=Ins` / `INS` | `PH`, `NOM` |
| évszám | year number | - | `PUN` |
| exocentrikus összetétel | exocentric compound | - | `DER` |
| fantomtő | phantom stem | - | `DER` |
| félidézőjel | single quotation mark (gloss quote) | - | `PUN` |
| felkiáltójel | exclamation mark | - | `PUN` |
| felsőfok | superlative | `Degree=Sup` / `SPRL` | `NOM` |
| felsőfok jele `leg-` | superlative prefix | `Degree=Sup` / `SPRL` | `DER` |
| felsorolás | enumeration, list | - | `PUN` |
| félszó | semi-word (bound compound member) | - | `DER` |
| felszólító (kötő) mód | imperative / subjunctive | `Mood=Imp` | `SYN` |
| felszólító mód, felszólító (kötő) mód | imperative-subjunctive (also: imperative / subjunctive) | `Mood=Imp` / `IMP` (subjunctive use `SBJV`) | `PH`, `VRB` |
| feltételes kötőjel | soft hyphen | - | `REP` |
| feltételes mód | conditional | `Mood=Cnd` / `COND` | `VRB`, `SYN` |
| feltételes múlt (`volna`) | past conditional | phrase-level `Mood=Cnd` + `Tense=Past` / `COND;PST` | `VRB` |
| ferde vonal | slash | - | `PUN` |
| foglalkozásnév | occupational noun | - | `DER` |
| fókusz | focus | - | `SYN` |
| földrajzi név, földrajzi köznév | geographic name, geographic common noun | - | `ORT` |
| folyamatos melléknévi igenév (`-ó/-ő`) | present participle | `VerbForm=PartPres` / `V.PTCP;PRS` | `VRB`, `DER` |
| főmondat, mellékmondat | main clause, subordinate clause | - | `PUN` |
| főnév | noun | `NOUN` / `N` | `NOM` |
| főnévi igenév | infinitive | `VerbForm=Inf`; `xcomp` / `csubj` / `NFIN` | `VRB`, `SYN` |
| főnévi minőségjelző | noun qualifier | - | `ORT` |
| főnévképző | noun-forming suffix | `NOUN` output | `DER` |
| formalis -képp(en) | formal | UD `Case=Abs`, the same value as `-ként` (the two are not distinguished in the UD projection); our model `Case=Modalis`. Pronominal and lexicalised forms are ADV with `PronType` and no `Case`: `mindenképp(en)` `PronType=Tot`, `semmiképp(en)` `PronType=Neg`, `miképp(en)` `PronType=Int`; `tulajdonképpen`, `voltaképpen` no features | `NOM` |
| fosztóképző | privative suffix | `Polarity=Neg` (derivational, MISC) / `PRIV` | `DER` |
| funkciószó | function word | - | `REP` |
| gemináta, hosszú mássalhangzó | geminate, long consonant | - | `PH` |
| gondolatjel | dash (spaced; parenthetic or dialogue) | UPOS `PUNCT` | `WS`, `ORT`, `PUN` |
| gót betű (fraktúr) | Fraktur | - | `REP` |
| gyakorító ige (`-gat/-get`) | frequentative verb | `Aspect=Iter` / `ITER` | `VRB` |
| gyakorító képző | frequentative suffix | `Aspect=Iter` / `ITER` | `DER` |
| gyűjtő (-an/-en) | collective (modal-essive on numerals) | `Case=Ess` / `ESS` | `PND` |
| gyűjtőnév | collective noun | - | `DER` |
| hagyományos írásmód | traditional principle (also: traditional spelling) | - | `PH`, `ORT` |
| hangátvetés | metathesis | - | `NOM` |
| hangátvető tő | metathesis stem | - | `PH` |
| hangkivető (epentetikus) tő | epenthetic (unstable vowel) stem | - | `PH` |
| hangkivető (epentetikus) tő | epenthetic stem (unstable vowel, vowel-zero alternation) | - | `NOM`, `VRB`, `DER` |
| hangrend | harmony class (also: vowel harmony) | - | `NOM` |
| hangsor-szabályosság (fonotaktika) | phonotactics | - | `PH` |
| hármas betűtorlódás | three identical letters | - | `NOM` |
| három pont, hármaspont | ellipsis | - | `PUN` |
| háromjegyű betű | three-character letter, trigraph (`dzs`) | - | `WS` |
| hasonulás | assimilation | - | `PH`, `VRB`, `ORT` |
| határeset (terminativus) | terminative | `Case=Ter` / `TERM` | `NOM` |
| határozatlan névelő | indefinite article | `Definite=Ind`, `PronType=Art` / `INDF` | `PND` |
| határozatlan névmás | indefinite pronoun | `PronType=Ind` / `INDF` | `PND` |
| határozatlan számnév | quantifier (indefinite numeral) | `DET` + `PronType=Ind` / - | `PND` |
| határozó | adverbial | `obl`, `advmod` (subtypes `advmod:locy`, `advmod:tlocy`, `advmod:mode`) | `SYN` |
| határozói igenév (`-va/-ve`, `-ván/-vén`) | converb (adverbial participle) (also: adverbial participle) | `VerbForm=Conv`; `advcl` / `V.CVB` | `VRB`, `PUN`, `SYN` |
| határozószó | adverb | - | `ORT` |
| határozószó-képző | adverb-forming suffix | `ADV` output | `DER` |
| határozott névelő | definite article | `Definite=Def`, `PronType=Art` / `DEF` | `PND` |
| határozott (tárgyas) ragozás | definite (objective) conjugation | `Definite=Def` / `DEF` | `PH`, `VRB`, `SYN` |
| ható ige (`-hat/-het`) | potential (verb) | `Mood=Pot` - bare for the indicative, **never `Ind,Pot`** (`Pot` replaces `Ind`), with `Tense=Pres\|Past` kept; stacked as `Mood=Cnd,Pot` and `Mood=Imp,Pot`; lemma is the non-potential stem, the suffix being inflectional / UniMorph `POT` (bare `V;POT` only, a UniMorph coverage artefact we diverge from) | `VRB`, `DER` |
| helyesírás | orthography, spelling | - | `ORT` |
| helyesírási hiba | spelling error | - | `REP` |
| helyesírási reform | spelling reform | - | `REP` |
| hézag (hiátus) | hiatus | - | `PH` |
| hiányjel, aposztróf | apostrophe | UPOS `PUNCT` | `WS`, `PUN` |
| hiányos ige | defective verb | - | `VRB` |
| hibás kódolás, mojibake, "krikszkraksz" | mojibake (misdecoded text) | - | `REP` |
| hivatali nyelv | officialese | lint category | `SYN` |
| homonímia elkerülése | homonymy avoidance | - | `DER` |
| hosszú s (ſ) | long s | - | `REP` |
| hullámvonalas o, kalapos u | o with tilde, u with circumflex (substitutes for `ő`, `ű`) | - | `REP` |
| idéző mondat | quoting clause | - | `PUN` |
| idézőjel, macskaköröm | quotation marks `„ ”` (low-high double) | UPOS `PUNCT` | `WS`, `PUN` |
| időhatározó -kor (temporalis) | temporal | `Case=Tem` / - | `NOM` |
| ige | verb | UPOS `VERB` / `V` | `VRB` |
| igei módosító | verbal modifier | position class; `compound:preverb`, `obj`, `obl`, `xcomp` | `SYN` |
| igeképző | verb-forming suffix | `VERB` output | `DER` |
| igekötő | verbal prefix (also: preverb, particle) | `compound:preverb` when separated (hu_szeged); lemma-internal when joined | `VRB`, `DER`, `ORT`, `SYN` |
| igető, tő | verb stem | - | `VRB` |
| ikerszó | twin word (echo compound) | - | `DER`, `ORT` |
| ikes ige | `-ik` verb | lexical feature (none in UD) | `VRB` |
| ikes ragozás | -ik conjugation | - | `REP` |
| iktelen ige | non-`-ik` verb | - | `VRB` |
| indulatszó | interjection | - | `PUN` |
| ingadozó hangrendű tő | vacillating stem | - | `PH` |
| intézménynév | institution name | - | `ORT` |
| irányítószám | postal code | - | `PUN` |
| írásjegy, jegy | character (component of a letter) | - | `WS` |
| írásjel | punctuation mark | - | `PUN` |
| jelen idő | present tense | `Tense=Pres` / `PRS` | `VRB` |
| jelentéstömörítő összetétel | condensed-meaning compound | - | `ORT` |
| jelentésváltozás | meaning change | - | `ORT` |
| jelölt / jelöletlen viszony | marked / unmarked relation | - | `ORT` |
| jelző (melléknévi) | attributive adjective | `amod:att` | `SYN` |
| j-ly hiba | j/ly error | - | `REP` |
| jottista, ipszilonista | proponents of morphological vs phonetic j-suffix spelling (18th-19th c.) | - | `REP` |
| jövevényszó, idegen szó | loanword, foreign word | - | `ORT` |
| jövő idő (`fog` + inf.) | future (periphrastic) | analytic; `lesz`: `Tense=Fut` not used in UD-hu, tagged `Tense=Pres` / `FUT` in UniMorph | `VRB` |
| kapcsolt egyenes beszéd | linked direct speech | - | `PUN` |
| karakterkódolás | character encoding | - | `WS`, `REP` |
| keltezés | date (date writing) (also: dating) | - | `PND`, `ORT`, `PUN` |
| -ként (essivus-formalis) | essive-formal | UD `Case=Abs` (treebank, all 57 relevant tokens, + `hu/feat/Case.html`, which documents the suffix `-ként` with the example `éveként`); our model `Case=Formalis`; UniMorph `FRML`. Note `emmorph2ud2` dissents with `Case=Ess`, but `Ess` is already taken in Szeged by the `-an`/`-en` manner adverbs and `Frm` does not exist in UD, so `Abs` wins on interoperability despite being a bad name | `NOM` |
| képzés helye szerinti hasonulás | place assimilation | - | `PH` |
| képző | derivational suffix | - | `DER` |
| kérdő névmás | interrogative pronoun | `PronType=Int` / `INTER` | `PND` |
| kérdő partikula (`-e`) | interrogative clitic | a **separate token** whose surface form and lemma are both `-e`, **with the leading hyphen**; `UPOS=ADV`, `PronType=Int`, `deprel=advmod:que`, head the finite verb. `vajon` is treated identically; `Polite` is unused and `UPOS=PART` is not used for `-e`. Distractor: bare `e`/`E` without a hyphen is the demonstrative determiner `PronType=Dem`, so the tokeniser must not strip the hyphen | `SYN` |
| kérdőjel | question mark | - | `PUN` |
| kérdőszó | interrogative word | `PronType=Int` | `SYN` |
| kerekségi (labiális) harmónia | rounding harmony | - | `PH` |
| kétjegyű betű | two-character letter (digraph) | - | `WS`, `PH` |
| kettős családnév | double family name | - | `ORT` |
| kettős ékezet (hosszú ékezet) | double acute accent | - | `REP` |
| kettőspont | colon | - | `PUN` |
| kettőzött betű | doubled letter | - | `WS`, `ORT` |
| kettőzött többjegyű betű | doubled multi-character letter | - | `REP` |
| kicsinyítő képző | diminutive suffix | - / `DIM` (derivational) | `DER` |
| kiejtés szerinti írásmód | pronunciation principle (also: phonemic spelling principle) | - | `PH`, `ORT` |
| kiemelő -ik | partitive/selective `-ik` | - | `NOM` |
| kijelentő / kérdő / felkiáltó / óhajtó / felszólító mondat | declarative / interrogative / exclamative / optative / imperative sentence | - | `PUN` |
| kijelentő mód | indicative | `Mood=Ind` / `IND` | `VRB`, `SYN` |
| kis / nagy kezdőbetű | lowercase / capital initial | - | `ORT` |
| kisbetű / nagybetű | lowercase / uppercase letter | - | `WS` |
| kiterjesztett ábécé | extended alphabet (with `q w x y`) | - | `WS` |
| kitüntetés, díj | decoration, award | - | `ORT` |
| kódlap | code page | - | `WS`, `REP` |
| kölcsönös névmás | reciprocal pronoun | `PronType=Rcp` / `RECP` | `PND`, `SYN` |
| konverzió, szófajváltás | conversion | - | `DER` |
| kötőhangzó | linking vowel | - | `PH`, `NOM`, `VRB`, `DER` |
| kötőjel | hyphen | UPOS `PUNCT` | `WS`, `ORT`, `PUN` |
| kötőszó | conjunction | `cc` (coordinating), `mark` (subordinating) | `PUN`, `SYN` |
| kötött tő, tőváltozat | bound stem, stem alternant | - | `NOM` |
| közbevetés | parenthetical insertion | - | `PUN` |
| középfok | comparative | `Degree=Cmp` / `CMPR` | `NOM` |
| köznevesülés | common-noun formation from a proper name | - | `ORT` |
| külviszonyragok (felület): superessivus, sublativus, delativus | superessive, sublative, delative | `Sup`, `Sbl`, `Del` / `ON+ESS`, `ON+ALL`, `ON+ABL` | `NOM` |
| külviszonyragok (közelség): adessivus, allativus, ablativus | adessive, allative, ablative | `Ade`, `All`, `Abl` / `AT+ESS`, `AT+ALL`, `AT+ABL` | `NOM` |
| `-lAk` alak | 1sg subject with 2nd person object form | `Definite=2` (the subject's `Number=Sing\|Person=1` alongside); **not** `Person[obj]=2`, which is not a UD feature at all and cannot validate - `Person[psor]=2` is in the validator's `unused_lvalues` - so the object's person and number lives in our own model as `Object=2` / UniMorph `LGSPEC3` (no `DEF`/`INDF`, the third value of the same dimension) | `VRB`, `SYN` |
| létige, kopula (`van`, `lesz`) | copula / existential verb | `cop` (predicative) or root `VERB` (existential) | `SYN` |
| lexikai akadályozás | lexical blocking | - | `DER` |
| lexikalizálódás | lexicalisation | - | `DER` |
| ligatúra | ligature | - | `REP` |
| locativus -t/-tt | locative | `Case=Loc` / - | `NOM` |
| lúdlábidézőjel, lúdláb, belső idézőjel | guillemets `» «` (inner quotation marks) | UPOS `PUNCT` | `WS`, `PUN` |
| magánhangzó | vowel | - | `WS`, `PH` |
| magánhangzó-harmónia, illeszkedés | vowel harmony | - | `PH` |
| magas (elülső) magánhangzó | front vowel | - | `PH` |
| magázás, önözés | formal (V) address with `maga`, `ön` | `Polite=Form` (universal feature, unused in hu_szeged) / `FORM` | `PND`, `SYN` |
| márkanév | brand name | - | `ORT` |
| mássalhangzó | consonant | - | `WS`, `PH` |
| mássalhangzó-kiesés | consonant deletion (cluster simplification) | - | `PH` |
| mássalhangzó-rövidülés | degemination (also: consonant shortening) | - | `PH`, `ORT` |
| mássalhangzó-torlódás | consonant cluster | - | `PH` |
| megengedő névmás | free-choice pronoun | `PronType=Ind` / - | `PND` |
| megszólítás | vocative, form of address (also: salutation) | `vocative` | `ORT`, `PUN`, `SYN` |
| mellékjel | diacritic | - | `WS` |
| mellékmondat (alanyi, tárgyi, határozói) | subordinate clause (subject, object, adverbial) | `csubj`, `ccomp:obj`, `ccomp:obl`, `advcl`; relative `acl` | `SYN` |
| melléknév | adjective | `ADJ` / `ADJ` | `NOM` |
| melléknévi igenév | participle | `VerbForm=PartPres\|PartPast\|PartFut`; `acl` / `amod:att` | `SYN` |
| melléknévképző | adjective-forming suffix | `ADJ` output | `DER` |
| mellérendelő / alárendelő összetett mondat | coordinate / subordinate compound sentence | - | `PUN` |
| mellérendelő összetétel | coordinative compound | - | `DER`, `ORT` |
| mély / magas hangrendű | back / front harmonic | - | `VRB` |
| mély (hátsó) magánhangzó | back vowel | - | `PH` |
| mennyiségjelző, kvantor | quantifier | `det` or `nummod`; `PronType=Tot\|Ind\|Neg` | `SYN` |
| mértékegység | unit of measurement (also: unit of measure) | `NOUN` / `N` | `PND`, `PUN` |
| mértékegység jele | unit symbol | - | `ORT` |
| mértékképző (-nyi) | measure suffix | - | `DER` |
| mínuszjel | minus sign | - | `PUN` |
| módhatározó képző -an/-en, -ul/-ül, -lag/-leg | adverbialiser | `ADV` / - | `NOM` |
| mondathatár | sentence boundary | sentence split | `SYN` |
| mondatszó | sentence word | - | `PUN` |
| mozaikszó; betűszó; szóösszevonás | acronym (general); letter acronym (initialism, read letter by letter or as a word); blend (clipped compound) | `Abbr=Yes` | `WS`, `PH`, `ORT`, `PUN` |
| mozzanatos képző | momentaneous suffix | - | `DER` |
| múlt idő, múlt idő jele (`-t/-tt`) | past tense | `Tense=Past` / `PST` | `PH`, `VRB` |
| mutató névmás | demonstrative pronoun / determiner | `PronType=Dem`; `det` when attributive / `DEM` | `PND`, `SYN` |
| műveltető ige (`-at/-et`, `-tat/-tet`) | causative verb | `Voice=Cau` / `CAUS` | `VRB` |
| műveltető képző | causative suffix | `Voice=Cau` / `CAUS` | `DER` |
| nagy kezdőbetű | initial capital | - | `WS` |
| nagykötőjel | en dash (unspaced; range or relation) | UPOS `PUNCT` | `WS`, `ORT`, `PUN` |
| nákolás | `-nák` for the 1sg indefinite conditional `-nék` | repair category (`REP-SPELL-008`) | `REP` |
| nem nyújtó (szuperanalitikus) toldalék | non-lengthening (super-analytic) suffix | - | `PH` |
| nem törhető szóköz, nem törő szóköz | no-break space | - | `PUN`, `REP` |
| néma betű | silent letter | - | `PH`, `ORT` |
| népnév | ethnonym | - | `DER` |
| névelő | article | `DET` + `PronType=Art` / `ART` | `PND`, `SYN` |
| névmás | pronoun | `PRON` / `PRO` | `PND` |
| névutó | postposition | `ADP` + `AdpType=Post`; relation `case` / `ADP` | `PND`, `ORT`, `SYN` |
| névutómelléknév | postpositional adjective (`-i`) | `ADJ` / `ADJ` | `PND` |
| nyelvfelismerés | language identification | - | `REP` |
| nyitótő, alsó kötőhangzós tő | lowering stem (takes the lowered linking vowel) | - | `PH`, `NOM`, `DER` |
| nyújtó / nem nyújtó (szuperanalitikus) toldalék | lengthening / non-lengthening (super-analytic) suffix | - | `ORT` |
| OCR-hiba | OCR error | - | `REP` |
| okhatározó (causalis-finalis) | causal-final | `Case=Cau` / `PRP` | `NOM` |
| optikai karakterfelismerés (OCR) | optical character recognition | - | `REP` |
| összeolvadás | coalescence | - | `PH`, `ORT` |
| összetett szó; összetételi tag, előtag, utótag | compound; compound member, first member, last member | `Compound=Yes` (MISC, not in UD-hu core) | `DER`, `ORT` |
| osztó | distributive | `NumType=Dist` / `DISTR` | `PND` |
| páros kötőszó | paired (correlative) conjunction | - | `PUN` |
| pont | period, full stop | - | `PUN` |
| pontosvessző | semicolon | - | `PUN` |
| pótlónyúlás | compensatory lengthening | - | `PH` |
| rag, jel | inflectional suffix (case, marker) | `Case` etc. / various | `DER` |
| régies betű, régies írásmód | archaic letter, traditional spelling (in family names) | - | `WS`, `PH`, `ORT` |
| régies helyesírás | archaic orthography | - | `REP` |
| rendhagyó ige | irregular verb | - | `VRB` |
| repülő ékezet (holt billentyű) | dead key; also the 7-bit flying-accent notation | - | `REP` |
| részes eset | dative | `Case=Dat` / `DAT` | `NOM` |
| ritkított szedés | letterspacing | - | `REP` |
| római szám | Roman numeral | `NumForm=Roman` / - | `PND`, `PUN` |
| rövid / hosszú magánhangzó | short / long vowel | - | `WS`, `PH` |
| rövidítés | abbreviation | - | `ORT`, `PUN` |
| rövidülő tő | shortening stem | - | `PH`, `NOM`, `DER` |
| segédige | auxiliary | `aux` | `SYN` |
| semleges magánhangzó | neutral vowel | - | `PH` |
| semleges mondat | neutral sentence | - | `SYN` |
| sociativus -stul/-stül | sociative | **no UD `Case`** - a recorded decision, not an omission: `Com` is not among the 22 permitted hu values (`Case.lvalues = []`), so emitting it is a validation error, and the treebank, UniMorph and `emmorph2ud2` are all silent. UD projection `UPOS=NOUN`, `Case` omitted, `deprel=obl`; our model `Case=Sociativus`. The case is lost entirely in UD; an upstream issue to add `Com` for hu is the only principled fix | `NOM` |
| sorszámnév | ordinal numeral | `ADJ` + `NumType=Ord` / `NUM` + `ORD` | `PND`, `PUN` |
| suksükölés | imperative-subjunctive definite form of a `t`-final verb used for the indicative | repair category (`REP-SPELL-008`) | `REP` |
| szabad tő | free stem | - | `NOM` |
| számjegy, tőszámnév, sorszámnév, törtszámnév | digit, cardinal numeral, ordinal numeral, fractional numeral | - | `ORT` |
| számnév | numeral | `NUM`; `nummod` / `NUM` | `PND`, `SYN` |
| sz-d-v tő, sz-d-v tövű ige | sz-d-v stem (verb) (also: sz/d/v verb) | - | `PH`, `VRB` |
| személyes névmás | personal pronoun | `PRON` + `PronType=Prs` / `PRO` | `PND` |
| személynév, családnév (vezetéknév), utónév (keresztnév) | personal name, family name (surname), given name | - | `ORT` |
| személyrag | personal suffix | `Person`, `Number` / `1`, `2`, `3`, `SG`, `PL` | `VRB` |
| személyragos főnévi igenév | personal (inflected) infinitive (also: person-marked infinitive) | `VerbForm=Inf` + `Person`, `Number` / `NFIN;1;SG` | `VRB`, `SYN` |
| személyragos névutó | personal-suffixed postposition | `ADP` + `Person`, `Number` / `ADP` + `PSS1S` ... | `PND` |
| szenvedő ige (`-tatik/-tetik`) | passive (archaic) | `Voice=Pass` / `PASS` | `VRB` |
| szintetikus / analitikus / szuperanalitikus toldalék(olás) | synthetic / analytic / super-analytic suffix(ation) | - | `NOM`, `DER` |
| sziszegő / susogó hang | sibilant (alveolar / postalveolar) | - | `PH` |
| szóelemző írásmód | morphological (word-analysis) principle (also: morphemic spelling principle) | - | `PH`, `ORT` |
| szóismétlés, szókettőzés | reduplication (also: word repetition) | - | `DER`, `ORT` |
| szóképzés | derivation | - / `DERIV` (not a UniMorph inflectional feature) | `DER` |
| szóköz | space | - | `PUN` |
| szóösszeragadás | word fusion (lost space) | - | `REP` |
| szóösszetétel | compounding | - | `DER` |
| szorzó (-szor/-szer/-ször) | multiplicative | `NumType=Mult` / - | `PND` |
| szószétesés | word split (inserted space) | - | `REP` |
| szótag | syllable | - | `PH` |
| szótagmag, kezdet, kóda | nucleus, onset, coda | - | `PH` |
| szótagokra esett szöveg | per-syllable fragmented text | - | `REP` |
| szótagszámlálás, 6:3 szabály | syllable counting, the 6:3 rule | - | `ORT` |
| szövegjavítás | text repair | - | `REP` |
| tagadó létige (`nincs`) | negative existential | lemma `van`, `Polarity=Neg` / `NEG` | `VRB` |
| tagadó névmás | negative pronoun (n-word) | `PronType=Neg` / `NEG` | `PND`, `SYN` |
| tagadószó (`nem`, `ne`, `sem`) | negation particle | `advmod`; `Polarity=Neg` | `SYN` |
| tagmondat | clause | - | `PUN`, `SYN` |
| tanítópár | training pair | - | `REP` |
| tapadó írásjel | attaching punctuation mark | - | `PUN` |
| tárgy | direct object | `obj`; `Case=Acc` | `SYN` |
| tárgyeset, tárgyrag (`-t`) | accusative | `Case=Acc` / `ACC` | `PH`, `NOM` |
| társas többes, -ék | associative plural | UD `Number=Plur` with the bare-base lemma and no extra feature (treebank and `emmorph2ud2` agree); our model `Number=Assoc`. **The associative/plural distinction is lost in the UD projection** (`Kovácsék` 'the Kovács family' and `Kovácsok` 'people named Kovács' both give `Number=Plur`); UD has no associative feature and inventing one would fail validation | `NOM` |
| tegezés | informal (T) address | `Polite=Infm` (universal feature, unused in hu_szeged) / `INFM` | `PND`, `SYN` |
| teljes hasonulás | total assimilation | - | `PH` |
| teljes hasonulás (`v`) | v-assimilation | - | `NOM` |
| teljes kettőzés | full doubling at compound boundary | - | `WS` |
| termékeny képzés | productive derivation | - | `DER` |
| tizedesvessző | decimal comma | - | `PUN` |
| tő, szótő | stem | - | `NOM` |
| többes birtokjel | possessed plural marker `-i` | `Number=Plur` / `PL` | `NOM` |
| többes birtokjel (-éi) | plural anaphoric possessive `-éi` | `Number[psed]=Plur` / `LGSPEC2` | `NOM` |
| többes szám | plural | `Number=Plur` / `PL` | `VRB` |
| többesjel | plural marker | `Number=Plur` / `PL` | `NOM` |
| többszörös összetétel | multiple compound | - | `ORT` |
| tőcsonkítás | truncation | - | `DER` |
| toldalék (rag, jel, képző) | suffix (case suffix, marker, derivational suffix) | - | `ORT` |
| topik | topic | - | `SYN` |
| törtszámnév | fraction | `NumType=Frac` / `NUM` + `FRAC` | `PND` |
| tőszámnév | cardinal numeral | `NumType=Card` / `NUM` + `CARD` | `PND` |
| tővégi nyúlás, tővégi magánhangzó-nyúlás | final low vowel lengthening (`a`/`e` -> `á`/`é`) (also: stem-final vowel lengthening) | - | `PH`, `NOM`, `DER`, `ORT` |
| tulajdonnév; köznév, közszó | proper name; common noun, common word (also: proper noun) | UPOS `PROPN` | `WS`, `NOM`, `ORT` |
| túlzófok (`legesleg-`) | excessive | `Degree=Sup` in hu_szeged (UD universal `Degree=Abs` unused) / `SPRL` (no distinct value) | `NOM`, `DER` |
| utalószó | correlative pronoun (anticipatory demonstrative) (also: pointer word) | `obj` / `obl` pronoun with a clausal dependent | `PND`, `PUN`, `SYN` |
| utótag | last member (head) (also: second member) | - | `DER` |
| vágyakozó (`-hatnék`) | desiderative noun | NOUN, derivational feature | `VRB` |
| vegyes hangrendű szó | mixed-vowel word | - | `PH` |
| vessző | comma | - | `PUN` |
| visszaható / mediális ige, képző (`-ódik/-ődik`, `-kodik`) | medio-reflexive verb / suffix (also: reflexive, medial, medio-passive) | `Voice=Mid` (project analysis layer, unused in hu_szeged) / `MID` | `VRB`, `DER` |
| visszaható névmás | reflexive pronoun | `PronType=Prs`, `Reflex=Yes` / `RFLX` | `PND`, `SYN` |
| vonatkozó névmás | relative pronoun | `PronType=Rel` / `REL` | `PND`, `SYN` |
| vonzat | governed argument, valency frame | `obj`, `obl`, `iobj` | `SYN` |
| v-tő, v-vel bővülő tő, v-s tő | v-stem | - | `PH`, `NOM`, `VRB`, `DER` |
| zajgenerálás | noise generation | - | `REP` |
| zárójel (kerek, szögletes, csúcsos, kapcsos) | bracket (round, square, angle, curly) | - | `PUN` |
| zár-rés hang (affrikáta) | affricate | - | `PH` |
| zárt ë | closed e (dialect vowel) | - | `WS` |
| zöngés / zöngétlen | voiced / voiceless | - | `PH` |
| zöngésség szerinti hasonulás | voicing assimilation | - | `PH` |
