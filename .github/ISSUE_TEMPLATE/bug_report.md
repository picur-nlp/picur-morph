---
name: Bug report
about: Something crashes, panics, or otherwise misbehaves (not a wrong Hungarian form)
title: ""
labels: bug
assignees: ""
---

**Note:** if this is about a specific Hungarian word or form being wrong (analysis or
generation), please use the "Incorrect form" template instead; it collects the
information needed to trace the issue back to a rule.

## Describe the bug

A clear, concise description of what the bug is.

## To reproduce

Minimal code (or CLI invocation) that triggers it:

```rust
// paste a minimal repro here
```

## Expected behavior

What you expected to happen instead.

## Observed behavior

What actually happened. Include the full panic message / backtrace if there is one.

## Environment

- `picur-morph` version or commit:
- Rust version (`rustc --version`):
- Target (e.g. `x86_64-unknown-linux-gnu`, `wasm32-unknown-unknown`):
- OS:

## Additional context

Anything else relevant (input encoding, crate feature flags enabled, etc.).
