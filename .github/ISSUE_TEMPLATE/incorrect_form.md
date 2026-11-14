---
name: Incorrect form
about: A generated or analysed Hungarian form is wrong
title: ""
labels: incorrect-form
assignees: ""
---

This template is for cases where the library produces (or accepts) a Hungarian form that
is wrong, or rejects one that should be valid. Filling in all fields below lets the issue
be traced to the rule that is missing, mis-scoped, or wrong, per the traceability
discipline described in [`CONTRIBUTING.md`](../../CONTRIBUTING.md).

## Input

The lemma and/or surface form and the operation performed (generation or analysis),
including the requested features (case, number, person, etc.) if generating:

```
lemma:      
features:   
operation:  generation | analysis
```

## Expected form

What the correct output should be, and how you know (a reference, a native-speaker
judgement, a corpus attestation, AkH12 section, etc.):

```
expected:   
source:     
```

## Observed form

What the library actually produced (or `panic` / `None` / rejected as invalid, if
applicable):

```
observed:   
```

## Rule ID (if known)

If you can identify the rule in [`docs/grammar/`](../../docs/grammar/) that governs this form, cite its ID
(e.g. `NOM-ACC-004`). If you are not sure, leave this blank; do not guess.

```
rule id:    
```

## Additional context

Dialectal or register notes, whether this is a productive pattern vs. an idiosyncratic
exception, or anything else that would help distinguish "the rule is wrong" from
"this word needs a lexical trait".
