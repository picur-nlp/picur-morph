# Contributing

The rulebook is the specification and the code implements it, never the other way around. Read
this before sending a PR.

## The core discipline

Every rule in [docs/grammar/](docs/grammar/README.md) has an ID (e.g. `NOM-ACC-004`), a source
locator, a predictability level and a confidence level.

- **Code cites the rule IDs it implements**, in a doc comment above the function or type:

  ```rust
  /// Accusative `-t` attaches bare after a sibilant, with a linking vowel otherwise.
  /// Rules: NOM-ACC-004, NOM-ACC-009
  fn accusative(stem: &Stem) -> String { ... }
  ```

  `cargo xtask coverage` reports which rule IDs appear in the codebase. A PR that implements a
  rule without citing its ID is incomplete.

- **Tests come from the rulebook.** Goldens are extracted by `cargo xtask goldens` from each
  rule's `Examples` field. If a form needs a test case that is not already an example in
  [docs/grammar/](docs/grammar/README.md), add the example to the rule first (with its source),
  then regenerate the goldens; do not write a one-off test with no rule ID behind it.

- **Data files are regenerable.** Anything committed under a crate's `data/` directory has a
  command that produces it, named in
  [docs/design/06-reproducibility.md](docs/design/06-reproducibility.md); the inputs it needs are
  laid out in [WORKSPACE.md](WORKSPACE.md). Nothing derived is committed without running that
  command.

- **Dependencies are permissive.** Runtime and dev dependencies carry a permissive licence (MIT,
  Apache-2.0, BSD, Unlicense, Unicode-3.0, CC0-1.0, or a compatible dual/OR of these); see
  [docs/design/07-dependencies.md](docs/design/07-dependencies.md). `cargo deny check` enforces
  this in CI. Do not add a dependency that needs an exception in [`deny.toml`](deny.toml) without
  discussing it in the PR first.

- **Prose is ASCII.** Comments, docs and commit messages use `-`, `->`, `...` and `'`, never their
  typographic counterparts (dashes, arrows, ellipsis, curly quotes). Hungarian letters, `§`
  locators and characters that are themselves the subject (inside code spans, string literals and
  rule examples) are exempt.

A change to [docs/grammar/](docs/grammar/README.md) itself (adding, correcting or resourcing a
rule) is a specification change and is reviewed as such: state the source for any new or changed
claim.

## Checks

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo deny check
```

`just check` runs the fmt, clippy and test subset. CI ([`.github/workflows/ci.yml`](.github/workflows/ci.yml))
runs all of the above, plus a library-only clippy pass and a `wasm32-unknown-unknown` build, on
the toolchain pinned in [`rust-toolchain.toml`](rust-toolchain.toml).

## Commit messages

[Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `docs:`,
`refactor:`, `test:`, `chore:`, ...), with a terse subject and rationale in the body only when the
diff does not show it. Release versions are derived from these messages, so a mistyped commit
(e.g. `fix:` for a breaking change) is a wrong release.

## Pull requests

- One logical change per PR. A rulebook correction and an engine change are two PRs.
- Run the checks above before opening the PR.
- Fill in the [PR template](.github/pull_request_template.md), including the rule IDs the change
  touches.

## Releasing

All crates share the workspace version. `cargo package --workspace --exclude xtask` builds and
verifies every crate in dependency order; `cargo publish --workspace --exclude xtask` uploads them
in that order. `picur-morph-core` reaches `picur-morph-spell` only as a path dev-dependency, which
publishing strips, because the two would otherwise depend on each other.
