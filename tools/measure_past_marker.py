"""Measures the past marker of every verb, per cell.

`VRB-PAST-001` chooses between `-t` and `-Ott` by the coda, and the choice is lexical for a large
class and splits by cell: `függ` is `függött` word-finally but `függtem` before a person suffix.
The existing lists cover the geminates and the `-ad` verbs by hand; this asks the same two
questions of every verb, so a lexeme needs no naming to be settled.

An exemption needs more evidence than a confirmation, because dropping a marking removes a form
from the language. A cell whose two candidates coincide carries no evidence either way.

    cargo xtask morphdb local/research-impl/morphdb_morphdb.hu/in
    python tools/measure_past_marker.py

Writes local/lists/past-marker.tsv.
"""

import collections
import sqlite3
import sys
from paths import LISTS, WC2_DB

DB = WC2_DB
IN = LISTS / "past-candidates.tsv"
OUT = LISTS / "past-marker.tsv"
FLOOR = 20
# A minority form counts as attested at 200 tokens and one per cent of the cell, and is then
# something to accept rather than something to flag.
OTHER_FLOOR = 200


def decide(t, ott):
    if t + ott < FLOOR:
        return "-"
    if t > 3 * ott:
        return "t"
    if ott > 3 * t:
        return "ott"
    return "-"


def main(path=IN):
    db = sqlite3.connect(DB)

    def freq(form, lemma):
        row = db.execute(
            "select coalesce(sum(freq),0) from f where form=? and lemma=? and pos='V'",
            (form, lemma),
        ).fetchone()
        return row[0] or 0

    rows = []
    with open(path, encoding="utf-8") as fh:
        for line in fh.read().splitlines()[1:]:
            parts = line.split("\t")
            if len(parts) < 5:
                continue
            lemma, final_t, final_ott, nonfinal_t, nonfinal_ott = parts

            def cell(a, b, lemma=lemma):
                if a == b:
                    return "-", 0, 0
                x, y = freq(a, lemma), freq(b, lemma)
                return decide(x, y), x, y

            f, ft, fo = cell(final_t, final_ott)
            n, nt, no = cell(nonfinal_t, nonfinal_ott)
            if f == "-" and n == "-":
                continue
            other = any(
                min(x, y) >= OTHER_FLOOR and min(x, y) * 100 >= x + y
                for x, y in ((ft, fo), (nt, no))
            )
            rows.append((lemma, f, n, "yes" if other else "no", ft, fo, nt, no))

    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write(
            "lemma\tfinal\tnonfinal\tother"
            "\tn_final_t\tn_final_ott\tn_nonfinal_t\tn_nonfinal_ott\n"
        )
        for row in rows:
            fh.write("\t".join(map(str, row)) + "\n")
    print(OUT, len(rows), "attested")
    for name, i in (("final", 1), ("nonfinal", 2)):
        print(f"  {name:9}", dict(collections.Counter(r[i] for r in rows)))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else IN))
