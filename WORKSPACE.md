# Workspace

The library, its tests and CI need only the Rust toolchain. Regenerating the committed data and
reproducing the measurements in [docs/grammar/evidence/](docs/grammar/evidence/) needs external
inputs, laid out here. What each input feeds is in
[docs/design/06-reproducibility.md](docs/design/06-reproducibility.md).

## Toolchain

| tool | for |
|---|---|
| Rust, as pinned in [`rust-toolchain.toml`](rust-toolchain.toml) | everything |
| [`just`](https://github.com/casey/just) | `just check`, see [`justfile`](justfile) |
| [`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny) | the licence gate, see [`deny.toml`](deny.toml) |
| Python 3.12 with [`tools/requirements.txt`](tools/requirements.txt) | [`tools/`](tools/README.md) |
| `hunspell` with the `hu_HU` dictionary, `foma` (`flookup`) | the oracles in [`tools/oracles.py`](tools/oracles.py) |
| [`hf`](https://huggingface.co/docs/huggingface_hub/guides/cli) | downloading the corpora |

```sh
python3 -m venv local/venv
local/venv/bin/pip install -r tools/requirements.txt huggingface_hub
```

## Environment

| variable | default | what |
|---|---|---|
| `PICUR_LOCAL` | `local/` | root of the external inputs below |
| `PICUR_ENYELV` | `local/cache/enyelv/qa.jsonl` | cached e-nyelv.hu advisory archive, one answered question per line |

Paths resolve in [`tools/paths.py`](tools/paths.py). `cargo xtask` reads `local/` relative to the
repository root.

## Corpora

`cargo xtask frequency` counts these Hugging Face shards, pinned by revision; the counts, and so
[`frequency.ron`](crates/picur-morph-lexicon/data/frequency.ron), depend on exactly this set.

| source | dataset | revision | shards | licence |
|---|---|---|---|---|
| `hplt` | [`HPLT/HPLT2.0_cleaned`](https://huggingface.co/datasets/HPLT/HPLT2.0_cleaned) | `d1324a5283f762ee62c2a5c81de08fc6450ea540` | `hun_Latn_1/train-000[0-2]*` (30) | CC0-1.0 |
| `hq` | [`epfml/FineWeb2-HQ`](https://huggingface.co/datasets/epfml/FineWeb2-HQ) | `c0c06e94fd3a44ae9e802b2b0fc533817601eb5e` | `hun_Latn/000_0000[0-7].parquet` (8) | ODC-BY-1.0 |
| `gazette` | [`lazos/gazette-hu`](https://huggingface.co/datasets/lazos/gazette-hu) | `67fb3f22c09de5de2852e184d590cf158b3a27a0` | `data/*.parquet` (15) | CC0-1.0 |
| `gutenberg` | [`lazos/gutenberg-hu`](https://huggingface.co/datasets/lazos/gutenberg-hu) | `96c8e97794ef0b43a1e5dd61da7067c273f82e0f` | `data/*.parquet` (2) | public domain |

```sh
get() { hf download "$1" --repo-type dataset --revision "$2" --include "$3" --local-dir "local/corpora/$4"; }
get HPLT/HPLT2.0_cleaned d1324a5283f762ee62c2a5c81de08fc6450ea540 'hun_Latn_1/train-000[0-2]*' hplt
get epfml/FineWeb2-HQ c0c06e94fd3a44ae9e802b2b0fc533817601eb5e 'hun_Latn/000_0000[0-7].parquet' hq
get lazos/gazette-hu 67fb3f22c09de5de2852e184d590cf158b3a27a0 'data/*.parquet' gazette
get lazos/gutenberg-hu 96c8e97794ef0b43a1e5dd61da7067c273f82e0f 'data/*.parquet' gutenberg
cargo xtask frequency
```

About 15 GB on disk.

## `local/`

Gitignored. Nothing under it is committed or shipped; the licence of each source decides whether
anything derived from it may be (see [docs/design/07-dependencies.md](docs/design/07-dependencies.md)).

| path | content | source |
|---|---|---|
| `lists/` | measured lexical lists, read by `cargo xtask morphdb` | written by [`tools/`](tools/README.md) and by `cargo xtask morphdb` (`*-candidates.tsv`) |
| `cache/webcorpus2.sqlite` | form, lemma, tag and count index | built by [`tools/webcorpus.py`](tools/webcorpus.py) from the Webcorpus 2 parquet |
| `corpora/` | the pinned corpus shards | see [Corpora](#corpora) |
| `cache/counts.json`, `cache/counts.sqlite` | per-source form counts and their index | `cargo xtask frequency`, then [`tools/corpus.py`](tools/corpus.py) |
| `cache/kwic/`, `cache/suggest/` | request caches | [`tools/kwic.py`](tools/kwic.py), [`tools/suggest.py`](tools/suggest.py) |
| `research-impl/morphdb_morphdb.hu/` | morphdb.hu at commit `1984ce20c414280e01a3546ac4d365746587109e` (CC BY 2.5) | <https://github.com/morphdb/morphdb.hu> |
| `research-impl/ud_hungarian-szeged/` | UD Hungarian-Szeged at `8f08ba648e6f6345212a721b7cb1ad78c5924f0e` (CC BY-NC-SA 3.0) | <https://github.com/UniversalDependencies/UD_Hungarian-Szeged> |
| `research-impl/nytud_nerkor/` | NerKor at `7c9c5b5f1f3ec7f94e724e0b92f3afa8eb262d00` (CC BY-SA 4.0) | <https://github.com/nytud/NYTK-NerKor> |
| `research-impl/r0ller_hunmorph-foma/` | hunmorph-foma FSTs at `8cefbdda6a4a510d8a93e4a29617d5dcf01e1f6a` (BSD-3) | <https://github.com/r0ller/hunmorph-foma> |
| `research-impl/unimorph_hun/` | UniMorph Hungarian at `20d5e04211bbab3dd689c99637356a8eab7c5fd7` (CC BY-SA 3.0) | <https://github.com/unimorph/hun> |
| `research-impl/kaikki/kaikki.org-dictionary-Hungarian.jsonl` | Wiktionary extract (CC BY-SA) | <https://kaikki.org/dictionary/Hungarian/> |
| `cache/enyelv/qa.jsonl` | e-nyelv.hu advisory archive, one answered question per line | <https://e-nyelv.hu/> |
| `research-impl/<owner>_<repo>/` | other implementations read for study | the GitHub repository named by the directory |
| `research-racz/Webcorpus2FrequencyList/frequencies.parquet` | Webcorpus 2 frequency list (MIT, Zenodo `10.5281/zenodo.17508385`) | <https://github.com/petyaracz/Webcorpus2FrequencyList> |
| `research-racz/<repo>/` | Rácz and co-authors' experiment data | `https://github.com/petyaracz/<repo>` |
| `research-method/` | UD Szeged concatenated as `hu_all.conllu`, UD `feats.json`, UniMorph `hun`, emmorph2ud mappings | see [docs/grammar/evidence/productivity-tagset.md](docs/grammar/evidence/productivity-tagset.md) |
| `research-lit/`, `research-akh/` | text extracts of the literature and of AkH11/AkH12 | cited in [docs/grammar/sources.md](docs/grammar/sources.md) |
