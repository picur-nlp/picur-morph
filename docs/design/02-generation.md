# Generation

The single source of truth. Every claim the library makes about Hungarian morphology is made here
once; analysis ([`03-analysis.md`](03-analysis.md)) verifies against it rather than restating it.

## Shape

```rust
/// Rules: NOM-MODEL-003 (slot order), NOM-MODEL-004 (suffix classes)
pub fn generate(entry: &Entry, stack: &[Morph]) -> Result<String, GenError>;
pub fn generate_seam(entry: &Entry, stack: &[Morph]) -> Result<(String, Option<usize>), GenError>;
pub fn generate_compound(lemma: &str, stack: &[Morph]) -> Result<String, GenError>;
```

The stack is walked left to right. Each morph picks its suffix from the entry's category and the
form reached so far, and `attach` joins it. A derivational morph produces a new word with its own
category and stem properties (`DER-MODEL-010`), and the rest of the stack is judged against that
word, not the lemma. `generate_seam` also returns where the last suffix begins, which the acronym
and number code read the written suffix off.

`GenError::NoSuchCell` means the stack is not a cell of this entry (a verb has no plural, a noun no
tense). The cells the rulebook closes are refused here too, so that analysis, which regenerates,
cannot accept them either: the case-like suffixes `-nként`, `-stul`, `-képp` and `-nta` close an
unpossessed singular and `-kor` no plural; `-ként` and `-ul` follow no `-é`; the collective and
the multiplicative reach an adjective only when it is a quantifier (`sokan`, `kevésszer`); the
potential has no converb and no future participle; a personal pronoun inflects only by
suppletion. A derivational suffix carries the same kind of refusal in its record: `-i` takes no
`i`-final common noun, `-ít` no `-i` adjective, `-gAt` no medial verb, `-cska` no abstract noun,
and the privative on a verb needs a transitive one. `GenError::Unsupported` means the rule needs a fact the library does not have, and refuses
rather than guesses: a spoken name without its pronunciation, a `-Vt` locative on a town the
lexicon does not mark, an unproductive suffix whose result is not a listed word.

## The attachment step

Everything interesting happens in one function, in a fixed order, because each step depends on
the one before it:

1. **Alternant.** Which stem shape the suffix sees: `bokr-` before a vowel, `bokor` before a
   consonant, by the entry's traits and the suffix's onset.
2. **Linking vowel.** Class `One` suffixes take one after a consonant unless the coda is in the
   bare class; it is low after a lowering stem and only for a lowerable suffix (`NOM-LOW-001`), so
   `ház` gives `házak` and `házat` but `házon`.
3. **Harmony.** Archiphonemes resolve against the harmony of stem plus link. A bare alternant that
   has dropped its only non-neutral vowel keeps the lemma's class (`titk-` is back).
4. **Lengthening.** A stem-final `a`/`e` becomes `á`/`é` before a lengthening suffix, never before
   a linking vowel.
5. **Seam.** `-vAl` assimilates to a consonant and is written by doubling its first character
   (`ésszel`); the definite `j` assimilates to a sibilant; three identical letters simplify to two in
   a common word and keep a hyphen in a name (`Kiss-sel`, `ORT-NAME-012`).

## Natural classes, not lists

The bare accusative (`NOM-ACC-003`) is the coronal sonorants plus the sibilants, a predicate over
the consonant feature table. The default is overridden per lexeme in either direction
(`LinkedAccusative` on `orr`, `BareAccusative` on `stressz`), and the override is induced from
corpus counts, never hand-written.

The past marker (`VRB-PAST-001`) is `-tt` after a vowel, `-t` after a coronal sonorant and `-Ott`
otherwise; the geminates and the `-ad`/`-ed` verbs are lexical and carry the measured class. A
verb family that attests both classes makes both optional.

The preverbs are a closed class in the lexicon, and a prefixed verb inflects as its base
(`ORT-TOG-026`). A verb takes one preverb; the two that stack (`újra`, `tovább`) are marked.

## Derivation

Thirty-two suffixes (`DerId`) are records (`Der`) with a template, input and output categories, linking class,
lengthening, productivity, the stem they attach to, a filter on that stem's shape where the rule
closes a class by shape (`-i` after no `i`, `-gAt` after no medial verb) and a filter on the base
lexeme where the rule closes it by a trait. The medio-passive `-Ódik` is one of them, coined on a
transitive verb's syncopated stem (`íródik`, `ismétlődik`) and conjugating on as an `-ik` verb.
`DER-MODEL-012` draws the line: a productive suffix coins words; an unproductive one (`-Ár`,
`-Ász`, `-(O)l`, `-(A)ll`, `-Odik`, `-(O)ntA`) applies only where the result is a listed word, so
`titkár` reads as `titok` plus `-Ár` and `asztalár` is never coined. An adjective derived from a
one-word name is lowercase (`ORT-NAME-022`), an i-final name writes one `i` (`ORT-NAME-020`), and
a multiword name keeps its capitals with the suffix after a hyphen.

A `P-dom` suffix generates only inside the domain its rule states, and it needs that domain to be
computable. Three shapes of domain occur. A shape in the spelling is a `stem_filter`: `-lAg` wants
an `-i` adjective, the habitual `-Ós` a frequentative or durative participle (`nézegetős`, not
`*futós`), and the onomatopoeic `-(O)g` and `-An` a monosyllabic interjection whose final consonant
is written long (`bummog`, `zuttyan`). A lexeme trait is an `entry_filter`: `-sÁg` on a noun wants
the occupational person noun `DER-NOUN-003` names, so `menedzserség` is coined and `*székség` is
not. A semantic class with neither (resultativity for the privative, `agentive` for `-Ós`,
countability for a compound head) is left generating, and the rulebook's starred forms for it stay
accepted; `DER-ADJ-010`'s own text says so.

Harmony after a derivational suffix follows `DER-MODEL-008` and `DER-MODEL-009`. A suffix that
wrote a harmonic vowel has fixed the class, so the derivative carries none of the base's harmony
traits and `agilisság` gives `agilisságnak` only. An invariant suffix, and a type `O` suffix after
a vowel-final stem that wrote no linking vowel, is transparent: the base's traits pass through, so
`karib` gives `karibinak ~ karibinek` and `martini` gives `martinizza ~ martinizi`. The one `F!`
suffix that is not a semi-word, `-képp`, is a harmony domain of its own, listed as
`OpaqueFrontEnding`, which is what makes it `válaszképpen` and not `*válaszképpon`.

## Numerals, acronyms and spoken names

A number is suffixed from its reading: `digits` and `digits_day` write `5-öt`, `1-jén`, `20-án`
(`PND-NUMINF`). An acronym or a letter is suffixed from its letter names where it is read out, and
from its last letter where it is read as a word; `written_suffixes` lists what a stem of unknown
pronunciation could take, and `licenses_nominal` and `licenses_foreign` accept a hyphenated suffix
on that basis (`ORT-ABBR`, `ORT-FOR-012`). A name spelled foreign carries its pronunciation (`alt`)
or only its last sound and harmony, and is suffixed from those facts alone.

## Testing

Goldens come from the rulebook: `cargo xtask goldens` extracts input and output pairs from the
`Examples` fields into [`docs/grammar/goldens.ron`](../grammar/goldens.ron), and `cargo xtask eval` reproduces them, by
chapter and by every mechanism the engine has, setting aside what the design refuses to generate
and what a rule declares lexical. The same command extracts every form the rulebook stars into
[`docs/grammar/rejects.ron`](../grammar/rejects.ron), and `eval` requires the checker to reject each; a starred form the
checker accepts is listed, because the rule that stars it is either violated or, as often,
naming a reading the form does not have. Paradigm
tests under [`crates/picur-morph-core/tests/`](../../crates/picur-morph-core/tests/) drive whole paradigms from the committed lexicon, and
the round-trip tests regenerate every analysis; `cargo xtask roundtrip` does the same over the
checker's whole search space ([`03-analysis.md`](03-analysis.md)). `cargo xtask accept <list>` reports what the checker
accepts over a frequency-ranked word list, which is the number that moves when a stem class is
wrong.
