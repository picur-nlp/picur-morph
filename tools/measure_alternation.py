"""Measures, per cell, whether a stem alternation is real.

The alternation traits are marked per lexeme but the alternation is often cell-restricted: `szó`
binds in the plural (`szavak`) and stays free in the accusative (`szót`), while the adjective `jó`
does not alternate at all (`jók`, `jót`). The corpus decides each cell, with a tag and lemma filter
so a homograph cannot vote (`lét` is its own noun, not the accusative of `lé`).

    cargo xtask morphdb local/research-impl/morphdb_morphdb.hu/in
    python tools/measure_alternation.py

The candidates come from the converter, which builds them from the morphdb flags. Reading them off
the committed lexicon instead is a loop: a lexeme the last measurement corrected no longer carries
the trait, so it emits no candidate and the flags win again on the next conversion.

Writes local/lists/stem-alternation.tsv.
"""

import collections
import sqlite3
import sys
from paths import LISTS, WC2_DB

DB = WC2_DB
IN = LISTS / "alternation-candidates.tsv"
OUT = LISTS / "stem-alternation.tsv"
TAG = {"Noun": "N", "Adjective": "Adj"}
FLOOR = 10


def decide(bound, free):
    if bound + free < FLOOR:
        return "-"
    if bound > 3 * free:
        return "bound"
    if free > 3 * bound:
        return "free"
    return "both"


def main(path=IN):
    db = sqlite3.connect(DB)

    def freq(form, lemma, tag):
        row = db.execute(
            "select coalesce(sum(freq),0) from f where form=? and lemma=? and pos=?",
            (form, lemma, tag),
        ).fetchone()
        return row[0] or 0

    rows = []
    with open(path, encoding="utf-8") as fh:
        for line in fh.read().splitlines()[1:]:
            parts = line.split("\t")
            if len(parts) < 6:
                continue
            lemma, pos, bound_pl, free_pl, bound_acc, free_acc = parts[:6]
            bound_poss, free_poss = parts[6], parts[7]
            tag = TAG.get(pos)
            if not tag:
                continue
            bp, fp = freq(bound_pl, lemma, tag), freq(free_pl, lemma, tag)
            ba, fa = freq(bound_acc, lemma, tag), freq(free_acc, lemma, tag)
            bo, fo = freq(bound_poss, lemma, tag), freq(free_poss, lemma, tag)
            rows.append(
                (lemma, pos, decide(bp, fp), decide(ba, fa), decide(bo, fo), bp, fp, ba, fa, bo, fo)
            )

    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write(
            "lemma\tpos\tplural\taccusative\tpossessive"
            "\tn_bound_pl\tn_free_pl\tn_bound_acc\tn_free_acc\tn_bound_poss\tn_free_poss\n"
        )
        for row in rows:
            fh.write("\t".join(map(str, row)) + "\n")
    print(OUT, len(rows), "rows")
    print("  plural:    ", dict(collections.Counter(r[2] for r in rows)))
    print("  accusative:", dict(collections.Counter(r[3] for r in rows)))
    print("  possessive:", dict(collections.Counter(r[4] for r in rows)))
    for key, n in collections.Counter((r[2], r[3]) for r in rows).most_common():
        print(f"    {key[0]:6} / {key[1]:6}  {n}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else IN))
