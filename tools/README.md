# Corpus evidence tools

Python scripts that produce the measured lists under `local/lists/` and the figures in
[docs/grammar/evidence/](../docs/grammar/evidence/). Inputs, outputs and environment setup are in
[WORKSPACE.md](../WORKSPACE.md). Run from the repository root; paths resolve through
[`paths.py`](paths.py).

Use Webcorpus 2 for the ratio and the homograph control, the count index for the per-source
split, and [`kwic.py`](kwic.py) for anything the two tokenisers throw away.

## Tools

### [`corpus.py`](corpus.py)

Shared metadata and the count index. `GROUPS` maps every source to `official / literary / web`;
`FLOOR` is the 20-token reliability floor; `wilson()` is the 95% score interval.

First use builds `local/cache/counts.sqlite` from the `local/cache/counts.json` that
`cargo xtask frequency` writes. Delete that file to rebuild. Forms are case-folded.

### [`ratios.py`](ratios.py)

Per-source-group counts and Wilson CIs for competing forms, plus optional Webcorpus 2 row.

```sh
python tools/ratios.py mondtak mondottak --by-source
python tools/ratios.py --wc2 --tag Pst mondtak mondottak
python tools/ratios.py --json szabónénak szabónének
```

Rows below the floor are flagged `[below floor]`.

### [`wc2.py`](wc2.py)

Webcorpus 2 lookups with emMorph tag filtering, which is the homograph control.

```sh
python tools/wc2.py orrt orrot --raw
python tools/wc2.py --tag Acc --lemma orr orrot orrt
```

`--raw` prints every lemma x tag row so the filter can be chosen from the data.

### [`kwic.py`](kwic.py)

Concordancer over `local/corpora/`. One parquet pass serves the whole pattern batch, so pass every
competing form in a single call; per-form scanning wastes the pass.

```sh
python tools/kwic.py --sources small --contexts 4 bele belé
python tools/kwic.py --regex --jobs 20 --sources small \
  '(?<![\w-])15-i(?![\w-])' '(?<![\w-])15-ei(?![\w-])'
python tools/kwic.py --sources gutenberg --near 'orr|orrom' orrt
```

- `--sources`: comma list, or the aliases `small` (`gazette`, `gutenberg`), `big` (`hplt`, `hq`),
  `all`.
- `--near REGEX`: the document must also match, which is the homograph control for forms the tag
  set cannot separate.
- `--regex`: patterns are regexes; without it a pattern is an exact token match
  (`(?<![\w-])form(?![\w-])`).
- Results cache under `local/cache/kwic/<hash>.json` keyed by the whole request; `--no-cache`
  forces a rescan.
- Files without a `text` column are skipped.

### Measured lexical lists

- `measure_*.py` - one per measured property; reads the `*-candidates.tsv` written by
  `cargo xtask morphdb` and writes the decided list that the next `cargo xtask morphdb` run reads.
  Order and inputs: [docs/design/06-reproducibility.md](../docs/design/06-reproducibility.md).
- [`verify_accusative.py`](verify_accusative.py) - the bare against linked accusative, tag- and lemma-filtered.
- [`listgen.py`](listgen.py) and the `gen_*.py` family - diagnostic-form building, candidate enumeration,
  voting across [`oracles.py`](oracles.py) and output; one generator per list family.
- [`oracles.py`](oracles.py) - loaders for morphdb, foma, hunspell, UniMorph and kaikki, and the licence split
  that decides which figures may be published.
- [`gen_review.py`](gen_review.py) - the ranked review queue (`rules x (1 + sqrt(lemma_freq)/100)`), excluding rows
  no evidence could decide.

### Lookups

- [`attest.py`](attest.py) - frequency bucket of each word in the committed `frequency.ron`.
- [`suggest.py`](suggest.py) / [`probe_suggest.py`](probe_suggest.py) - rate-limited, timestamp-cached client for the official
  spelling checker, used only where the corpus was silent or contradicted a source.
- [`enyelv.py`](enyelv.py) - AND-term search over the cached e-nyelv.hu advisory archive.

## Reliability and licence rules

- Do not quote a ratio unless every variant has n >= 20 in the bucket being quoted (`FLOOR`).
- Webcorpus 2 counts are publishable (MIT derived counts, cite Zenodo 10.5281/zenodo.17508385);
  the underlying corpus text is not redistributable.
