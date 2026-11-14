# Analysis

Analysis is search over generation. It proposes candidates, then accepts one only if regenerating
it reproduces the input exactly. No morphological claim is made here that is not already made in
[`02-generation.md`](02-generation.md).

```rust
pub fn analyse(surface: &str, stacks: &Stacks) -> Verdict;
pub fn is_known(surface: &str, stacks: &Stacks) -> bool;      // the checker's fast path
pub fn guess(surface: &str, stacks: &Stacks) -> Vec<Analysis>; // readings on an unlisted stem

pub enum Verdict {
    Known(Vec<Analysis>),   // the readings that regenerate the surface
    WellFormed,             // no listed stem, but a possible Hungarian word
    IllFormed,              // not a word: a wrong suffix shape, an impossible opening, damage
}
```

## The search space

The caller owns the search space: a `Stacks` built from the feature stacks it wants considered.
The checker's space (`picur_morph_spell::stacks`) holds the nominal, verbal, derivational and
participial stacks of running text. Every stack is keyed by the endings its last morph can write,
asked on probe stems of each shape, so a surface is tried only against the stacks whose ending it
carries; the stacks whose ending depends on the stem are open and always tried.

## The stem index

Keyed on **stable prefixes**: the longest prefix of a lemma that no alternation can change, never
ending inside a digraph. `bokor` is epenthetic, so its key is `bok`; a capitalised lemma is keyed
lowercase as well, because a name's `-i` adjective is lowercase. A surface is looked up by its
prefixes, longest first, and an entry whose lemma is longer than the surface can bear is skipped
before generation is tried.

## Verification

`generate(entry, stack) == surface`, byte for byte. The analyser runs that generation toward its
target (`generate_toward`): after each step the form minus its last two letters must still be a
prefix of the surface, or the stack is abandoned there. A step only appends or rewrites the seam,
so nothing a later step does can recover, and the result when there is one is generation's own.
On running text that cuts a third of the analyser's time, because most candidate stacks fail at
their first suffix. The variants an entry admits beside its own
traits (the other past class where both are attested, the other possessive allomorph, a free stem
where the alternation is optional, the long imperative, the formal converb `-vÁn`) are tried the
same way: each is an entry with the other value or a form built beside generation's, and it
counts only if it regenerates the surface; a variant stem that stands for some cells only, the
archaic `alusz-` of the present indicative, is tried in those alone. `forms(entry, stack)` is
the public face of that list, every spelling of one cell the engine vouches for, generation's
own answer first.

A form with two legal analyses produces two: `orra` is both 'his nose' and 'onto his nose'.
Readings are returned shorter stack first, and never disambiguated.

## The two directions agree

Every form generation writes must analyse back to the lemma and stack that wrote it, over the
checker's own search space and not a test's smaller one. `cargo xtask roundtrip [<n>] [<class>]`
checks that for the `n` commonest lemmas of each open class, every stack of the space and every
spelling `forms` vouches for, and classes each miss by what analysis returned instead: nothing,
the lemma under another stack, or another lemma. A miss is a defect in one of the directions, and
each has been: the audit is what found the suppletive stems inflected under the lemma's harmony
(`*lennom` for `lennem`), the superlative possessives the prefix path could not reach, the
comparative of `jó` taking the lemma's `java` allomorph (`*jobba`), and the formal `-jAm` the
ending index did not key. What remains is a form that is two cells at once (`legjobban` the adverb
and the superessive are both returned, and the audit counts the stack it asked for).

## Compounds

Hungarian compounds solidly and productively, so a word is also tried as non-final members from
the lexicon plus a head that analyses on its own (`DER-COMP-021`). The constraints are stated, not
tuned: a member is a citation form; a one- or two-letter member comes from a closed list, a
numeral or a preverb; a preverb is followed by the head or a deverbal member; a verb takes one
preverb; a name neither heads nor opens a coined compound (`Kossuth-díj` takes its hyphen); and a coined compound has at most one coined seam,
so a longer word is built on a compound the lexicon already lists. Semi-words (`-szerű`, `-féle`,
`-né`) close a compound and derive its category (`DER-COMP-017`). A hyphenated token is read at
each hyphen with the left side a word, a name, a letter or an acronym and the right side an
analysed head.

## The verdict on an unlisted word

`WellFormed` is the default for a word no stem produces, and `IllFormed` is earned:

- the shape is not Hungarian (no vowel, a doubled consonant or `y` opening a lowercase word);
- a listed lemma of an inflecting category is followed by a string the stacks write as a suffix
  and none generates (`házben`, `orrt`);
- the checker adds a third test over the lexicon: a listed word or a form of one differs from it
  only in accents (`kivül`, `hosszu`), found by an accent-insensitive regenerate-and-compare.

A capitalised word is exempt from all three: an unlisted name is not a misspelling of whatever it
resembles.

## The guesser

`guess` gives readings to a word whose stem no lexicon lists, from the suffix rules alone: every
stack is tried on every prefix that could be a stem, longest stem first, and a cut that regenerates
the surface is offered with that prefix as its lemma. A stem must end the way a Hungarian word may
(`PH-SYL-007`) and a verb stem in a consonant. The verdict is unchanged: a guess is a reading, not
a vouching, and the CLI marks it.

## The checker

`picur_morph_spell::check` maps the verdict to `Status::{Ok, Unknown, Wrong}` and adds what a
token needs beyond a word: a capitalised token is also tried lowercase, an all-capital one in
lowercase; a letter acronym or a single letter is `Unknown`; a hyphenated suffix is accepted on an
acronym by its reading where the shape fixes it (`ENSZ-szel`) and by every reading otherwise, and on an
abbreviation, a number or a capitalised base by the suffix it could carry; a name with
a Latin letter the alphabet lacks, and a hyphenated compound of a name and a word, are `Unknown`
rather than `Wrong`. Results are memoised per word.

`analyses` returns the readings on the same paths `check` takes, and `guesses` the guessed ones
where no proper reading exists.
