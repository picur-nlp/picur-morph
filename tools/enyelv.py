#!/usr/bin/env python3
"""Search the cached e-nyelv.hu advisory archive (43,125 answered questions).

Usage:
  enyelv.py "term1" "term2" ...        AND search over term+question+answer
  enyelv.py -n 5 -c 300 "-stul"        top 5 hits, 300-char snippets
  enyelv.py -q "regex" ...             treat terms as regexes

Prints: date, url, headword, and a snippet around the first match in the answer.
Source: `paths.ENYELV` (JSONL cache of the e-nyelv.hu archive, see WORKSPACE.md).
Advisory answers are secondary sources: paraphrase, cite url + date.
"""

import argparse
import json
import re
import sys
from pathlib import Path

from paths import ENYELV as QA


def load():
    return [json.loads(line) for line in QA.open(encoding="utf-8")]


def snippet(text, pattern, width):
    m = pattern.search(text)
    if not m:
        return text[:width].replace("\n", " ")
    start = max(0, m.start() - width // 3)
    return text[start : start + width].replace("\n", " ")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("terms", nargs="+")
    ap.add_argument("-n", type=int, default=5, help="max hits")
    ap.add_argument("-c", type=int, default=260, help="snippet chars")
    ap.add_argument("-q", action="store_true", help="terms are regexes")
    ap.add_argument("--count", action="store_true", help="print only the hit count")
    args = ap.parse_args()

    pats = [re.compile(t if args.q else re.escape(t), re.I) for t in args.terms]
    hits = []
    for row in load():
        blob = f"{row['term']} {row['question']} {row['answer']}"
        if all(p.search(blob) for p in pats):
            hits.append(row)
    hits.sort(key=lambda r: r["date"], reverse=True)

    print(f"# {' + '.join(args.terms)}: {len(hits)} hits")
    if args.count:
        return
    for row in hits[: args.n]:
        print(f"\n[{row['date']}] {row['term']}\n  {row['url']}")
        print(f"  Q: {snippet(row['question'], pats[0], args.c)}")
        print(f"  A: {snippet(row['answer'], pats[0], args.c)}")


if __name__ == "__main__":
    sys.exit(main())
