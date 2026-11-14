# Core model

Types shared by every crate above `ortho`. Where the rulebook states a closed inventory, it is an
enum; where it states a computation, it is a function, never a field.

## Letters, not characters

`WS-LET-001`: the unit of Hungarian spelling is the letter, and a letter is one, two or three
characters. Every length count, consonant and vowel pattern, doubling, hyphenation and
capitalisation operates on letters.

```rust
pub enum Letter { A, AAcute, B, C, Cs, D, Dz, Dzs, E, EAcute, /* ... */ Zs }

pub struct Span<'a> { pub letter: Letter, pub text: &'a str, pub at: usize }

/// `ssz` -> [Sz, Sz]; `dzs` -> [Dzs]; `meggy` -> [M, E, Gy, Gy].
/// Rules: WS-LET-001, WS-LET-002, WS-DIG-002
pub fn letters(s: &str) -> impl Iterator<Item = Span<'_>>;
```

Simplified doubling is ambiguous without morphology: `ssz` is one long `sz` inside a stem and two
letters across a compound seam (`WS-DIG-010`). `letters` resolves the stem case; the compound code
knows the seam and handles the other.

Segmentation is the innermost operation of the checker, which asks it for the same few dozen
stems thousands of times per word, so the seamless case is memoised per thread in a small
direct-mapped table keyed by the string; a seam-aware call is not, because its answer depends on
the seams as well.

The consonants carry a feature table (`features`, `Consonant { place, manner, voiced }`, with
`is_sonorant` and `is_sibilant` over it),
which is the alphabet's phonology as a definition. `bare_t_class` (`NOM-ACC-003`) and
`well_formed_final_cluster` (`PH-SYL-007`) are predicates over it, not lists.

## Harmony as a computed property

```rust
pub enum Harmony { Back, Front, FrontRounded }

/// Backness from the last non-neutral vowel; two trailing neutrals turn a back stem front
/// (`PH-HARM-012`); the `í` of `-ít` does not count (`PH-HARM-019`). Rounding from the last vowel.
/// Rules: PH-HARM-001, PH-HARM-003, PH-HARM-005, PH-HARM-012, PH-HARM-019
pub fn harmony_of(word: &str) -> Harmony;
```

An entry carries a harmony trait only where it contradicts its spelling (`híd`, `április`); the
rest is computed, and a derived `-i` adjective takes its base's backness.

## Suffix templates over archiphonemes

A suffix is stored once as a template; its allomorphs are computed.

```rust
pub enum Arch { A, AAcute, O, OAcute, U, UAcute }          // PH-HARM-002
pub enum Seg { Lit(&'static str), Var(Arch), AssimV, AssimJ }

pub struct Suffix {
    pub template: &'static [Seg],
    pub class: SuffixClass,     // One: optional linking vowel; Two: attaches directly; Three: invariant
    pub lengthens: bool,        // stem-final a/e -> á/é
    pub lowerable: bool,        // a lowering stem makes the link low; false for the superessive
    pub bare_coda: bool,        // the bare-coda class of NOM-ACC-003 applies
    pub free_stem: bool,        // a shortening stem keeps its long vowel (superessive)
    pub high_vowel_link: bool,  // a vowel-final gradable stem still links low (plural)
}
```

The inessive is `[Var(A), Lit("n")]` preceded by `b`; `-ban` and `-ben` appear nowhere in the
source. `Arch::O` is the only archiphoneme that distinguishes `FrontRounded`. `AssimV` is the `v`
of `-vAl`/`-vÁ` that assimilates to a consonant; `AssimJ` the `j` of the definite conjugation that
assimilates to a sibilant.

The case inventory is `NOM-CASE-001`: 24 `Case` variants, including the case-like `Formalis`,
`Distributivus`, `Sociativus`, `Modalis`, `DistributivusTemporalis` and the lexical `Loc`.

## Entries

```rust
pub struct Entry {
    pub lemma: Box<str>,
    pub parts: Box<[u32]>,          // compound seam offsets into the lemma
    pub pos: Pos,
    pub traits: TraitSet,           // bitset over the 56 traits
    pub alt: Option<Box<str>>,      // full pronunciation of a name spelled foreign (ORT-FOR-012)
    pub last_sound: Option<Box<str>>, // or only its last sound
}
```

Stem alternants are computed from the traits (`alternant`, `bound_alternant`), never stored:
`ló` and `lov` would otherwise be two places to disagree.

## Morphs and the feature stack

```rust
pub enum Morph {
    Plural, PossessedPlural, Anaphoric,
    Possessive { person: Person, number: Number },
    Comparative, Superlative, Excessive, Associative, Collective, Multiplicative,
    Derive(DerId),                                  // 32 derivational suffixes
    Potential, Infinitive,
    PresentParticiple, PastParticiple, FutureParticiple, Converb,
    Verbal { mood: Mood, tense: Tense, person: Person, number: Number, definite: Definite },
    Case(Case),
}

pub struct Analysis { pub lemma: Box<str>, pub pos: Pos, pub stack: Vec<Morph> }
```

The stack is ordered innermost first. `project` maps a reading to UD (`Ud { upos, feats }`); the
projection is lossy and one-way, and where UD has no value (`Sociativus`) the case is omitted rather
than invented. `segment_reading` and `segment_surface` cut a surface into `Piece`s with a `Role`
(`Preverb`, `Member`, `Stem`, `Affix`), which is how the CLI shows a segmentation.

## What is deliberately not a type

- **Allomorphs.** Computed from template plus harmony.
- **Stem alternants.** Computed from traits plus the suffix's onset.
- **Linking vowels.** Computed from stem shape plus suffix class.
- **Harmony class**, except where a stem contradicts its spelling.
- **Compound membership.** Computed by splitting against the lexicon, except for listed compounds,
  whose `parts` record the seams.
