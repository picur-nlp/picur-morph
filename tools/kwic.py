#!/usr/bin/env python3
"""Concordancer over local/corpora. Counts token hits per source and keeps sample contexts.

One parquet pass serves a whole batch of patterns, so give every competing form at once.

    kwic.py --sources small bele belé
    kwic.py --sources gazette,gutenberg --near 'orr|orrom' orrt
    kwic.py --regex '\\bami\\s+a\\b' --contexts 5

Results are cached under local/cache/kwic/ keyed by the whole request.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import multiprocessing as mp
import re
import sys
from pathlib import Path

import pyarrow.dataset as ds

import corpus

CACHE = corpus.CACHE / "kwic"

SMALL = ("gazette", "gutenberg")
"""The sources a full pass reads in minutes rather than hours."""

BIG = ("hplt", "hq")

ALIASES = {"small": SMALL, "big": BIG, "all": SMALL + BIG}


def resolve_sources(spec: str) -> list:
    out = []
    for part in spec.split(","):
        out.extend(ALIASES.get(part, (part,)))
    return [s for s in dict.fromkeys(out) if (corpus.CORPORA / s).is_dir()]


def key(args: dict) -> Path:
    h = hashlib.sha256(json.dumps(args, sort_keys=True, ensure_ascii=False).encode()).hexdigest()[:16]
    return CACHE / f"{h}.json"


_JOB = {}


def _setup(patterns, near, contexts, regex, ignore_case):
    flags = re.IGNORECASE if ignore_case else 0
    def build(p):
        return re.compile(p if regex else r"(?<![\w-])" + re.escape(p) + r"(?![\w-])", flags)
    _JOB.update(
        patterns=patterns,
        contexts=contexts,
        any_re=re.compile("|".join(f"(?:{p if regex else re.escape(p)})" for p in patterns), flags),
        pat_re={p: build(p) for p in patterns},
        near_re=re.compile(near, flags) if near else None,
    )


def _scan_file(job) -> dict:
    """One parquet file against the whole pattern batch."""
    src, path = job
    res = {p: {"counts": {}, "contexts": []} for p in _JOB["patterns"]}
    limit = _JOB["contexts"]
    dataset = ds.dataset([path], format="parquet")
    if "text" not in dataset.schema.names:
        return res
    for batch in dataset.to_batches(columns=["text"], batch_size=512):
        for text in batch.column("text").to_pylist():
            if text is None or not _JOB["any_re"].search(text):
                continue
            if _JOB["near_re"] is not None and not _JOB["near_re"].search(text):
                continue
            for p, rx in _JOB["pat_re"].items():
                hits = list(rx.finditer(text))
                if not hits:
                    continue
                r = res[p]
                r["counts"][src] = r["counts"].get(src, 0) + len(hits)
                for m in hits[:limit]:
                    if len(r["contexts"]) >= limit:
                        break
                    a, b = max(0, m.start() - 90), min(len(text), m.end() + 90)
                    r["contexts"].append({"source": src, "text": " ".join(text[a:b].split())})
    return res


def scan(patterns: list, sources: list, near: str, contexts: int, regex: bool,
         ignore_case: bool, jobs: int = 12) -> dict:
    files = [(s, str(f)) for s in sources for f in sorted((corpus.CORPORA / s).rglob("*.parquet"))]
    res = {p: {"counts": {}, "contexts": []} for p in patterns}
    args = (patterns, near, contexts, regex, ignore_case)
    with mp.Pool(min(jobs, max(1, len(files))), initializer=_setup, initargs=args) as pool:
        for part in pool.imap_unordered(_scan_file, files, chunksize=1):
            for p, r in part.items():
                for s, n in r["counts"].items():
                    res[p]["counts"][s] = res[p]["counts"].get(s, 0) + n
                room = contexts - len(res[p]["contexts"])
                if room > 0:
                    res[p]["contexts"].extend(r["contexts"][:room])
    return res


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("patterns", nargs="+", help="surface forms, or regexes with --regex")
    ap.add_argument("--sources", default="small", help="comma list; aliases small, big, all")
    ap.add_argument("--near", default="", help="regex that must also occur in the document (homograph control)")
    ap.add_argument("--contexts", type=int, default=8, help="sample contexts kept per pattern")
    ap.add_argument("--regex", action="store_true")
    ap.add_argument("-i", "--ignore-case", action="store_true")
    ap.add_argument("--jobs", type=int, default=12)
    ap.add_argument("--no-cache", action="store_true")
    ap.add_argument("--json", action="store_true")
    a = ap.parse_args(argv)

    sources = resolve_sources(a.sources)
    req = {"p": a.patterns, "s": sources, "n": a.near, "c": a.contexts,
           "r": a.regex, "i": a.ignore_case}
    path = key(req)
    if path.exists() and not a.no_cache:
        res = json.loads(path.read_text(encoding="utf-8"))
    else:
        res = scan(a.patterns, sources, a.near, a.contexts, a.regex, a.ignore_case, a.jobs)
        CACHE.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(res, ensure_ascii=False), encoding="utf-8")

    if a.json:
        json.dump(res, sys.stdout, ensure_ascii=False, indent=1)
        print()
        return 0

    for p, r in res.items():
        total = sum(r["counts"].values())
        pub = sum(n for s, n in r["counts"].items() if s in corpus.PERMISSIVE)
        print(f"== {p}  total={total}  permissive={pub}")
        for s, n in sorted(r["counts"].items(), key=lambda x: -x[1]):
            print(f"   {s:<22} {n:>8,}  {corpus.GROUP_OF.get(s, '-')}"
                  f"{'' if s in corpus.PERMISSIVE else '  [non-pub]'}")
        for c in r["contexts"]:
            print(f"   . [{c['source']}] {c['text']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
