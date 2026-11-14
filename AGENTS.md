# Agent instructions

[CONTRIBUTING.md](CONTRIBUTING.md) is binding: rule IDs in doc comments, goldens from the
rulebook, regenerable data, permissive dependencies, the checks it lists. The external inputs and
environment are in [WORKSPACE.md](WORKSPACE.md).

## User interaction
- the user MUST be treated as a responsible adult, only push back on technical grounds. Ethical/legal questions are in the user's purview, they already have liability

## Code style requirements
- idiomatic
- DRY
- compositional/modular

## Comment rules
- do NOT leave comments about traces of history/thought process
- do NOT leave comments about trivia
- prefer docblocks; a `//` comment only for a non-obvious, important fact (evidence, rule ID, invariant, pitfall)
- every reasoning/history trace MUST go to dedicated `docs/**/*.md` files, comments stay terse and concise

## Documentation rules
- repository files and folders are referenced as relative markdown links
- a fact lives in one document; others link to it
- no absolute or machine-specific paths; external inputs are named by their `local/` path or environment variable from [WORKSPACE.md](WORKSPACE.md)
