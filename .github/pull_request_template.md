## Summary

<!-- What does this PR do, and why? One or two sentences. -->

## Rule IDs touched

<!-- List the docs/grammar/ rule IDs this PR implements, fixes, or cites in new doc
     comments (e.g. NOM-ACC-004, NOM-ACC-009). Write "none" if this PR does not touch
     rule-implementing code (e.g. it is pure tooling, CI, or docs). -->

## Checklist

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `cargo deny check` (no new dependency, or any new dependency is permissively
      licensed and free of advisories)
- [ ] Public items added/changed cite the rule IDs they implement, in a doc comment
- [ ] Test cases are goldens from [`docs/grammar/`](../docs/grammar/) `Examples` fields (via
      `cargo xtask goldens`), not hand-invented, or this PR does not add test cases
- [ ] Any new/changed committed data file states the `cargo xtask ...` command that
      regenerates it, or this PR does not touch a data file
- [ ] Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/)

## Additional context

<!-- Anything a reviewer needs that isn't obvious from the diff. -->
