# picur-morph

Hungarian morphology, orthography and text repair, in pure Rust.

**Hungarian only.** The name carries no promise of a language-neutral framework: the letter
inventory, vowel harmony, the stem alternations and the orthography rules are all Hungarian, and
there is no abstraction layer waiting for a second language. If that changes it will be a different
crate, not a refactor of this one.

## What it does

- **Analysis**: surface form to lemma plus an ordered feature stack, N-best, with UD and UniMorph
  projections.
- **Generation**: lemma plus features to surface form. This is the single source of truth; analysis
  is search over it, verified by regenerating and comparing.
- **Orthography**: letters and digraphs, syllabification, hyphenation, capitalisation,
  the `6:3` rule, compound spelling.
- **Text repair**: encoding damage, OCR and keyboard confusions, split words, historical
  spelling, with every fix licensed by the checker.
- **Lint**: punctuation and typography decidable from the character stream (spacing, dashes,
  quotation marks, apostrophes, number and date formats, the article before a numeral).
- **Quality**: one score per document from what the checker accepts and the damage it sees.

No FFI, no runtime loading of foreign formats, no network at runtime. Data is compiled in and the
crate builds for `wasm32-unknown-unknown`.

## Use it

One dependency, no configuration. The lexicon and frequency tables are compiled in; there is no
data file or environment variable to set, and every function is safe to call from any thread. The
default `cli` feature builds the binary and pulls in `clap`; a library user turns it off.

```toml
[dependencies]
picur-morph = { version = "0.1", default-features = false }
```

```rust
use picur_morph::{Mode, Status, analyses, check, quality, repair};

assert_eq!(check("házaimban"), Status::Ok);
let readings = analyses("belenézett");        // lemma, category, feature stack per reading
let fixes = repair("elÅ‘tt", Mode::Conservative);  // WS-ENC-006: elÅ‘tt -> előtt
let score = quality("A ház nagy.").score();   // 0.0..=1.0, with the parts it is made of
```

The `picur-morph` binary exposes the same functions over files or standard input, plain text or one
JSON object per line:

```text
cargo install picur-morph

picur-morph check    text.txt                                # tokens the checker does not accept
picur-morph analyse  házaimban belenézett                    # readings, UD features, segmentation
picur-morph repair   scan.txt                                # repaired text; --fixes lists them
picur-morph rank     --jsonl text docs.jsonl --threads 16    # one score object per document
picur-morph lint     text.txt --fix                          # punctuation and typography
picur-morph paradigm alszik --pos verb                       # every form the checker would accept
```

Every subcommand's flags are its whole configuration; `picur-morph <command> --help` lists them.

## Why it exists

Hungarian is well served by research tools and badly served by libraries you can depend on. The
existing options are C bindings, affix-file spellcheckers, or truncation stemmers; none gives you a
lemma with features, and none generates. This crate is built to be a source of truth for NLP and
LLM datasets, which sets a higher bar than "usually right": every claim has to be checkable.

## How it is specified

[`docs/grammar/`](docs/grammar/) is the specification: **1,257 numbered rules** across ten chapters, each with its
conditions, output, exceptions, predictability, source locator and confidence level. Every rule
cites where it comes from, whether that is AkH12, a reference grammar, corpus attestation or a
language-advisory precedent, and [`docs/grammar/open-questions.md`](docs/grammar/open-questions.md) tracks what is still unresolved
rather than hiding it.

The code traces back to it. Public items name the rule IDs they implement, and
`cargo xtask coverage` reports which rule IDs the code cites. If you doubt a form, you can follow
its rule ID to the evidence that settled it: `orrot` against `orrt` rests on corpus counts of
8,930 to 2, and that number is in the rulebook.

[`docs/design/`](docs/design/) explains the implementation: the type model, the generation engine, how analysis is
defined in terms of it, and where the line between computed logic and stored lexical data runs.

## Licence

Apache-2.0. Dependencies are permissive-only, enforced in CI by `cargo-deny`. Data is derived from
permissively licensed sources with attribution in `NOTICE`; non-permissive resources are used only
as local oracles and nothing derived from them is committed.

## Status

Version 0.1. The specification is complete and evidence-backed. The measure of the implementation
is the rulebook's own examples: `cargo xtask eval` regenerates every `input -> output` pair in the
ten chapters and reproduces 99.3% of the 2,166 it scores, and 88.0% of all 3,148 rows once the
982 it sets aside (unproductive suffixes, outputs a rule declares lexical, repairs that need
context) are counted too. Every word form the rulebook shows as an example is an acceptance test,
and the checker accepts 98.3% of the 13,494; the rulebook's starred forms are negative goldens,
and the checker rejects 81% of the 242, the rest being words under another reading (`lest`,
`meglepőn`) or forbidden on semantic grounds no rule can compute (`*kutyai`, `*asztalkor`).
`cargo xtask ledger` gives every one of the 1,257 rules a status from those measurements: 846
covered, 132 partial, 32 set aside, 247 with nothing to measure yet. Where the norm allows two forms, the usual one is generated
and the other accepted. The citation count that `cargo xtask coverage` prints is traceability,
not coverage: one generic function implements many rules, and the lexical and tendency rules have
no code to cite. Measured against hand-annotated Hungarian (UD Szeged), the analyser reads 97.1%
of the tokens and agrees on the lemma for 96.5% of those; hunmorph-foma on the same tokens has the
gold lemma among its readings for 86% against this analyser's 93% by the same count. It does not
disambiguate between readings, and does not try to.
Where a rule needs a lexical fact the library does not have, it refuses rather than guesses.
[`docs/design/README.md`](docs/design/README.md) carries the full table and the commands that produce each figure.
