#!/usr/bin/env python3
"""Webcorpus 2 frequency-list lookups with emMorph tag filtering.

Rácz, Webcorpus2FrequencyList (MIT), Zenodo 10.5281/zenodo.17508385: 36,936,726 form+lemma+tag
rows over a 9e9-token corpus. No register split, no 400k cut-off, and a tag per row, which is
what makes it the homograph control.

    wc2.py orrt orrot                  # every lemma x tag row per form
    wc2.py --tag '\\[/N\\]\\[Acc\\]' orrt orrot
    wc2.py --lemma orr --tag Acc orrt orrot
"""
from __future__ import annotations

import argparse
import re
import sys

import duckdb

import corpus

PARQUET = corpus.CACHE.parent / "research-racz/Webcorpus2FrequencyList/frequencies.parquet"
CITE = "Webcorpus 2 frequency list (Racz, MIT, Zenodo 10.5281/zenodo.17508385)"


def rows(forms: list) -> list:
    """(form, lemma, xpostag, freq) for every form given, exact match, case-sensitive."""
    ph = ",".join("?" * len(forms))
    q = (f"select form, lemma, xpostag, cast(freq as bigint) from read_parquet('{PARQUET}') "
         f"where form in ({ph}) order by freq desc")
    return duckdb.execute(q, forms).fetchall()


def totals(forms: list, tag: str = "", lemma: str = "") -> dict:
    """{form: summed freq} after optional regex filters on xpostag and lemma."""
    trx = re.compile(tag) if tag else None
    lrx = re.compile(f"^{lemma}$") if lemma else None
    out = {f: 0 for f in forms}
    for form, lem, xpos, freq in rows(forms):
        if trx is not None and not trx.search(xpos or ""):
            continue
        if lrx is not None and not lrx.match(lem or ""):
            continue
        out[form] += freq
    return out


def pair(forms: list, tag: str = "", lemma: str = "") -> dict:
    t = totals(forms, tag, lemma)
    n = sum(t.values())
    p, lo, hi = corpus.wilson(t[forms[0]], n)
    return {"counts": t, "total": n, "share": p, "ci": [lo, hi],
            "reliable": n >= corpus.FLOOR and max(t.values()) >= corpus.FLOOR}


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("forms", nargs="+")
    ap.add_argument("--tag", default="", help="regex the emMorph xpostag must match")
    ap.add_argument("--lemma", default="", help="regex the lemma must match in full")
    ap.add_argument("--raw", action="store_true", help="print every lemma x tag row")
    a = ap.parse_args(argv)

    if a.raw or not a.tag and not a.lemma:
        for form, lem, xpos, freq in rows(a.forms):
            print(f"{freq:>10,}  {form:<24} {lem:<20} {xpos}")
        print()
    r = pair(a.forms, a.tag, a.lemma)
    print("filtered totals:", "  ".join(f"{f}={r['counts'][f]:,}" for f in a.forms))
    print(f"share[{a.forms[0]}] = {r['share']:.4f}  95% CI {r['ci'][0]:.4f}-{r['ci'][1]:.4f}"
          f"  n={r['total']:,}{'' if r['reliable'] else '  [below floor]'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
