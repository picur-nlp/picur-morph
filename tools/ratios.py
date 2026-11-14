#!/usr/bin/env python3
"""Per-source counts and Wilson-CI ratios for competing surface forms.

    ratios.py mondtak mondottak
    ratios.py --by-source lajosként lajoskent
    ratios.py --json hűbb hívebb
"""
from __future__ import annotations

import argparse
import json
import sys

import corpus


def table(counts: dict, forms: list, buckets: dict, floor: int) -> list:
    """Rows of (bucket, per-form counts, total, share of forms[0], CI, reliable flag)."""
    rows = []
    for name, per in buckets.items():
        total = sum(per.values())
        p, lo, hi = corpus.wilson(per[forms[0]], total)
        reliable = total >= floor and max(per.values()) >= floor
        rows.append({
            "bucket": name,
            "counts": dict(per),
            "total": total,
            "share": p,
            "ci": [lo, hi],
            "reliable": reliable,
        })
    return rows


def render(rows: list, forms: list, title: str) -> str:
    w = max([len(r["bucket"]) for r in rows] + [len(title)])
    head = f"{title:<{w}}  " + "  ".join(f"{f:>12}" for f in forms) + f"{'total':>10}  {'share[0]':>9}  {'95% CI':>15}"
    out = [head, "-" * len(head)]
    for r in rows:
        cells = "  ".join(f"{r['counts'][f]:>12,}" for f in forms)
        ci = f"{r['ci'][0]:.3f}-{r['ci'][1]:.3f}"
        flag = "" if r["reliable"] else "  [below floor]"
        out.append(f"{r['bucket']:<{w}}  {cells}{r['total']:>10,}  {r['share']:>9.3f}  {ci:>15}{flag}")
    return "\n".join(out)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("forms", nargs="+", help="competing surface forms; the first is the numerator")
    ap.add_argument("--by-source", action="store_true", help="also print one row per source")
    ap.add_argument("--floor", type=int, default=corpus.FLOOR, help="reliability floor (default 20)")
    ap.add_argument("--wc2", action="store_true", help="add a Webcorpus 2 row (tag-filtered)")
    ap.add_argument("--tag", default="", help="emMorph tag regex for --wc2")
    ap.add_argument("--lemma", default="", help="lemma regex for --wc2")
    ap.add_argument("--json", action="store_true")
    a = ap.parse_args(argv)

    db = corpus.connect()
    counts = corpus.counts_for(db, a.forms)

    groups = corpus.ratio_rows(counts, a.forms, corpus.GROUP_OF.get)
    groups = {g: groups[g] for g in corpus.GROUP_ORDER if g in groups}
    allb = {"ALL": {f: sum(counts.get(f, {}).values()) for f in a.forms}}
    pub = corpus.ratio_rows(counts, a.forms, lambda s: "PERMISSIVE" if s in corpus.PERMISSIVE else None)

    result = {
        "forms": a.forms,
        "groups": table(counts, a.forms, groups, a.floor),
        "all": table(counts, a.forms, allb, a.floor),
        "permissive": table(counts, a.forms, pub, a.floor),
        "sources": table(counts, a.forms, corpus.ratio_rows(counts, a.forms, lambda s: s), a.floor),
    }
    if a.wc2:
        import wc2
        r = wc2.pair(a.forms, a.tag, a.lemma)
        r["bucket"] = "WEBCORPUS2" + (f" [{a.tag or a.lemma}]" if a.tag or a.lemma else "")
        result["wc2"] = [r]

    if a.json:
        json.dump(result, sys.stdout, ensure_ascii=False, indent=1)
        print()
        return 0

    print(render(result["groups"] + result["permissive"] + result["all"] + result.get("wc2", []),
                 a.forms, "bucket"))
    if a.by_source:
        print()
        rows = sorted(result["sources"], key=lambda r: -r["total"])
        for r in rows:
            r["bucket"] = f"{r['bucket']} ({corpus.GROUP_OF.get(r['bucket'], '?')}" \
                          f"{'' if r['bucket'] in corpus.PERMISSIVE else ', non-pub'})"
        print(render(rows, a.forms, "source"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
