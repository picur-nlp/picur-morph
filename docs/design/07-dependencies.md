# Dependencies

The crate family is `picur-morph`. No Hungarian morphological analyser exists in Rust, so the
engine (generation, analysis, the lexicon) is this project's own; what is reused is everything
around it.

## Policy

Every runtime dependency is:

1. **Permissively licensed**: MIT, Apache-2.0, BSD, Unlicense, Unicode-3.0, CC0, or a dual of
   these. [`deny.toml`](../../deny.toml) holds the allow-list and `cargo deny check` enforces it, so a copyleft crate
   anywhere in the graph fails CI.
2. **Free of advisories**, checked against RustSec by the same command.
3. **Pure Rust**: no `*-sys` crate, no C toolchain.
4. **`wasm32-unknown-unknown` clean**: no filesystem, no threads, no network at runtime. CI builds
   the workspace for that target.

Runtime dependencies are few on purpose: a source-of-truth library that pulls in forty crates is
harder to audit than the problem it solves.

## Runtime

| crate | licence | used by | role |
|---|---|---|---|
| `serde` | MIT OR Apache-2.0 | `lexicon`, `core` | derives on the schema and feature types |
| `smallvec` | MIT OR Apache-2.0 | `ortho` | inline letter spans |
| `compact_str` | MIT | `core` | inline suffix tails |
| `encoding_rs` | (Apache-2.0 OR MIT) AND BSD-3-Clause | `ortho` | legacy code pages for mojibake detection and repair |
| `oem_cp` | MIT | `ortho` | DOS code page tables (437, 850, 852) |

The lexicon RON is parsed by a reader of its own, not by a RON crate: the data is a flat list of
rows and the reader is a few lines.

Deliberately not dependencies: `unicode-segmentation` (the unit is the Hungarian letter, not the
grapheme), `regex` (the patterns are literal or morphological), `fst` (the stem index is a sorted
`Vec` with `partition_point`).

## Binary and tooling

| crate | licence | used by | role |
|---|---|---|---|
| `clap` | MIT OR Apache-2.0 | `picur-morph`, `cli` feature | command line |
| `serde_json` | MIT OR Apache-2.0 | `picur-morph`, `cli` feature; `xtask` | JSON and JSON Lines output |
| `encoding_rs` | (Apache-2.0 OR MIT) AND BSD-3-Clause | `xtask` | morphdb is ISO-8859-2 |
| `parquet`, `arrow-array` | Apache-2.0 | `xtask` | reads the corpus shards for `cargo xtask frequency` |

`xtask` is a workspace member and is not published.

## CI

Every push and pull request ([`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)), on the toolchain pinned in
[`rust-toolchain.toml`](../../rust-toolchain.toml), which is the MSRV:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`, with pedantic lints and
  `unsafe_code = "forbid"` from the workspace manifest, and again on `picur-morph` without the
  default `cli` feature
- `cargo test --workspace`
- `cargo doc --no-deps` with warnings denied
- `cargo build --workspace --target wasm32-unknown-unknown --exclude xtask`
- `cargo deny check`

`just check` ([`justfile`](../../justfile)) runs the fmt, clippy and test subset locally.

Only `main` saves the Rust build cache; other refs restore it. After each successful CI run on
`main`, [`.github/workflows/cache-prune.yml`](../../.github/workflows/cache-prune.yml) keeps the
newest cache per job and deletes the rest.

Dependabot ([`.github/dependabot.yml`](../../.github/dependabot.yml)) proposes Cargo and GitHub Actions
updates weekly and Python tool updates monthly; each update still has to pass the checks above.

## What makes the project reviewable

Traceability rather than badges: every rule has an ID, every public item cites the IDs it
implements, every data file states the command that regenerates it ([`06-reproducibility.md`](06-reproducibility.md)), and
every claim in the rulebook carries a source locator. A reviewer who doubts `orrot` can follow
`NOM-ACC-006` to the corpus counts that settled it.
