"""Measures which third-person possessive allomorph a noun takes.

`NOM-POSS-006` calls the choice lexical with no learnable default. The list it replaces is an
*exceptions* list, so it could only correct a lemma someone already doubted: `utodja` is 38,421
against `utoda` 23,241 and was marked `-a`. Asking every noun settles it.

Both are written out, because the trait is what generation reads and there is no default to fall
back on; a lemma the corpus cannot settle is left to the exceptions list.

    cargo xtask morphdb local/research-impl/morphdb_morphdb.hu/in
    python tools/measure_possessive_allomorph.py

Writes local/lists/possessive-allomorph.tsv.
"""

import collections
import sqlite3
import sys
from paths import LISTS, WC2_DB

DB = WC2_DB
IN = LISTS / "possessive-allomorph-candidates.tsv"
OUT = LISTS / "possessive-allomorph.tsv"
FLOOR = 20
# A minority form counts as attested at 200 tokens and one per cent of the pair, and is then
# something to accept rather than something to flag: `utoda` is 23,241 beside `utodja` 38,421.
OTHER_FLOOR = 200


def main(path=IN):
    db = sqlite3.connect(DB)

    def freq(form, lemma):
        row = db.execute(
            "select coalesce(sum(freq),0) from f where form=? and lemma=? and pos='N'",
            (form, lemma),
        ).fetchone()
        return row[0] or 0

    rows = []
    with open(path, encoding="utf-8") as fh:
        for line in fh.read().splitlines()[1:]:
            parts = line.split("\t")
            if len(parts) < 3:
                continue
            lemma, ja, a = parts
            nj, na = freq(ja, lemma), freq(a, lemma)
            if nj + na < FLOOR:
                continue
            minority = min(nj, na)
            other = minority >= OTHER_FLOOR and minority * 100 >= nj + na
            rows.append((lemma, "ja" if nj > na else "a", "yes" if other else "no", nj, na))

    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write("lemma\tallomorph\tother\tn_ja\tn_a\n")
        for row in rows:
            fh.write("\t".join(map(str, row)) + "\n")
    print(OUT, len(rows), "attested")
    print("  ", dict(collections.Counter(r[1] for r in rows)))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else IN))
