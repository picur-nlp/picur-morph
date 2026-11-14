# 09 Syntax

Scope: the four syntactic facts the morphology depends on; the rest of syntax is out of scope.

The project has two objectives: generating and ranking Hungarian text for training data, and
repairing damaged text. Clause-level syntax serves neither. Measured on 63,202 `NerKor` sentences,
the syntax detectors built against this chapter fired on 0.01% to 0.05% of them, which is no
discriminative power for ranking, and the rest of the chapter needs a parse the project does not
have and does not plan. What is left here are the rules other chapters reference: the agreement
class of a plural-form proper name, the principle behind the definite conjugation, case frames as
lexical data, and the interrogative clitic, which the checker has to accept as part of a word.

Morphological form building is referenced to `NOM`, `PND`, `VRB`; spelling to `ORT`; punctuation to
`PUN`.

## AGR: agreement

### SYN-AGR-010 Plural-form proper names and pluralia tantum
- **Statement**: Proper names that are formally plural (`az Egyesült Államok`, `a Kanári-szigetek`, `az Alpok`, band and book titles such as `A kőszívű ember fiai`) take singular agreement when they denote a single entity; common nouns that exist only in plural form (`halak` 'Pisces', `ikrek` 'Gemini', `gázművek`, `vízművek`) take singular or plural agreement according to the number of referents.
- **Conditions**:
  - subject head is a proper name ending in the plural suffix, or a lexically marked plurale tantum
- **Output**: proper name denoting one entity -> `Sing`; plurale tantum -> `Sing` (one referent) ~ `Plur` (several).
- **Examples**: `Az Amerikai Egyesült Államok hadat üzent.`; `Az Ámokfutók zenél a színpadon.`; `A kőszívű ember fiai érdekes regény.`; `Én halak vagyok.`; `Mi ikrek vagyunk.`; `ez az ikrek` ~ `ezek az ikrek`
- **Exceptions**: some speakers use singular `hal`, `iker` for the zodiac sign (`%Én hal vagyok.`); plural-form geographical names also occur with plural agreement (`Az Alpok magasak.`); SoH-NNP1 does not rule on this, so accept both and do not report.
- **Predictability**: `lexical` (proper-name gazetteer; pluralia tantum closed list of about 6 common nouns per source)
- **Lexical input**: proper-name flag; `plurale_tantum`
- **Interactions**: -
- **Sources**: [SoH-NNP1 Szabó §1.2.1.2.1 I.A pp.153-154], [SoH-NNP2 Dékány & Csirmaz §2.6.1.1.3.1 I pp.1054-1055]
- **Validation**: goldens
- **Confidence**: consensus

## DEF: definite and indefinite conjugation

### SYN-DEF-001 Principle of conjugation choice
- **Statement**: otherwise (indefinite object, no object, intransitive use, object in another case) it takes the indefinite conjugation.
- **Conditions**:
  - finite verb in any tense or mood; the choice is independent of word order and focus
  - object = the accusative (`Case=Acc`) phrase in the same clause, or a finite object clause, or the object of an infinitive complement of a transmitting matrix verb
  - dropped 3rd person singular definite object counts as definite
- **Output**: `Definite=Def` | `Definite=Ind` | `Definite=2`.
- **Examples**: `Látom a házat.`; `Látok egy házat.`; `Házat látok.`; `Látok.`; `Látlak.`; `Bízom Péterben.` (indefinite: object not accusative)
- **Exceptions**: cells where the definite and indefinite forms coincide give no signal and must never drive a lint: past 1sg (`láttam` def = indef), `-ik` verbs 1sg present (`eszem`), conditional 1pl (`látnánk`), conditional 1sg indefinite = 3pl definite for front stems (`kérnék`), 3sg indefinite `-ik` = 3pl definite front (`eszik`; `reggelizik` 'they breakfast on it' [KVF98 p.327]); mood syncretisms (`olvassuk` ind = imp) do not affect definiteness; full table in `VRB-PRES-007`.
- **Predictability**: `rule`
- **Lexical input**: verb transitivity
- **Interactions**: `VRB-PRES-001`, `VRB-PRES-002`, `VRB-IK-006`
- **Sources**: [KVF98 §2.1.3.6 pp.321-327], [SMNy3 Bartos §12.4 pp.636-656], [SoH-NNP1 Farkas & Alberti §1.1.1.4.1 pp.87-88]
- **Validation**: goldens; oracle:hunmorph-foma for `Definite`
- **Confidence**: normative

## GOV: government

### SYN-GOV-001 Case frames as lexical data
- **Statement**: Verbs, adjectives, deverbal nouns and postpositions select their non-subject arguments in lexically fixed cases or postpositions; the frame is data attached to the lemma (with sense) and is consumed by generation, argument identification for agreement and the misgovernment detector.
- **Conditions**:
  - frame format per sense: `lemma` + sense id + ordered slots `[role, realisation, optionality]`, realisation one of `Nom Acc Dat Ins Ine Ill Ela Sup Sub Del Ade All Abl Ter Tra Cau Ess-For P:<postposition> Inf Cl:hogy[mood] Cl:q LOC DIR SRC MANNER`, optionality `obl|opt`
  - semantic frames `LOC`, `DIR`, `SRC` accept any case/postposition of that local class
  - clausal slots record the correlate case and mood
- **Output**: a lemma may have several frames; a phrase matches a frame if its case/postposition equals the slot realisation; a clause may contain additional adjuncts in any case.
- **Examples**: `vár: [theme, Sub|Acc, opt]`; `gondol#1 'think of': [theme, Sub, obl]`; `gondol#2 'think sth about': [content, Acc, obl], [topic, Del, opt]`; `beszél: [addressee, Ins, opt], [topic, Del, opt]`; `függ: [source, Abl, obl]`; `büszke: [theme, Sub, opt]`; `lakik: [place, LOC, obl]`; `ér#arrive: [goal, DIR, obl]`
- **Exceptions**: frames are open-class lexical data; no completeness claim.
- **Predictability**: `lexical` (open class
- **Lexical input**: valency lexicon (licence-clean source to be determined, see Open questions)
- **Interactions**: SYN-DEF-001 (only `Acc` slots are objects)
- **Sources**: [KVF98 §1.2.1.2.4 pp.71-73], [KVF98 §1.2.1.2.5 p.73]
- **Validation**: goldens; attestation of frame instances
- **Confidence**: consensus

## Q: questions

### SYN-Q-002 The interrogative clitic -e
- **Statement**: `-e` marks yes/no questions; it is optional in main clauses (adding an 'indeed / is it so' nuance) and obligatory in embedded yes/no questions; it attaches with a hyphen to the finite verb, or, in verbless clauses, to the predicate; the verb keeps its neutral or focus-driven prefix order.
- **Conditions**:
  - host: finite verb of the clause; verbless clause: nominal/adjectival predicate
  - embedded: complement clause of `kérdez`, `tud`, `kíváncsi`, `kételkedik`, `nem biztos` etc. whose content is a yes/no question
- **Output**: `V-e`; `nem V-e PFX`; `Pred-e`; embedded with optional `hogy`.
- **Examples**: `Péter beteg volt-e?`; `Anna Szegeden dolgozik-e?`; `Anna megtalálta-e a választ?`; `Péter nem volt-e beteg?`; `Igaz-e?`; `Anna azt kérdezte, (hogy) Péter beteg volt-e.`; `Nem tudom, hogy írjunk-e ide példát.`; `Érdekes a kérdés, hogy Anna tanul-e.`
- **Exceptions**: the hyphen is orthographic (`AkH12 §263 h)`); `-e` is not the possessive `-e` (`háza`), nor the demonstrative `e`.
- **Predictability**: `rule`
- **Lexical input**: none
- **Interactions**: `PUN-END-007` (period after declarative main clause with embedded question)
- **Sources**: [KVF98 §1.1.1.2.1.1 pp.2-3], [KVF98 §1.1.2.2.4 pp.31-32 ex.85], [AkH12 §263 h)], [AkH12 §243]
- **Validation**: goldens
- **Confidence**: normative
