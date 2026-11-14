# Security Policy

## Scope

`picur-morph` is a pure-Rust morphology, orthography, and text-repair library. It has:

- **no `unsafe` code** - every crate carries `unsafe_code = "forbid"` at the workspace
  level, enforced by `cargo clippy` in CI;
- **no network access at runtime** - the library never makes a network call;
- **no filesystem or OS access at runtime** - data is compiled in via `include_str!`, and
  the crate builds for `wasm32-unknown-unknown`.

Given that, the realistic security surface is narrow: a malicious or malformed input
string causing a panic, unbounded resource use (denial of service), or, in principle, a
memory-safety issue in a dependency. Incorrect linguistic output (a wrong generated or
analysed form) is a correctness bug, not a security issue; report it with the normal
["incorrect form" issue template](.github/ISSUE_TEMPLATE/incorrect_form.md) instead.

## Reporting a Vulnerability

Do not open a public issue for a suspected vulnerability. Instead, email **lazos@lazos.me**
with:

- a description of the issue and its impact;
- steps to reproduce, including the input that triggers it, if applicable;
- the crate and version (or commit) affected.

You should get an acknowledgement within a few days. If the report is confirmed, a fix
will be prepared and a coordinated disclosure timeline agreed with you before any public
advisory or release.

## Dependencies

Every dependency's license and RustSec advisory status is checked on every push by
`cargo deny check` (see [`deny.toml`](deny.toml)); an advisory affecting a dependency in the graph
fails CI. If you find a vulnerable dependency that CI has not caught (e.g. an advisory
published after the last CI run), reporting it by email as above is still appreciated so
a release can be cut promptly.
