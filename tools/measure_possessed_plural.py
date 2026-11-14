"""Measures which possessed-plural allomorph a noun takes.

`NOM-PPL-001` derives the cell from the singular possessive's `-ja`/`-a` choice, and the corpus
shows the two cells parting company often: `barátja` is the right singular, while the plural is
`barátai` 116,592 against `barátjai` 591. Only the lemmas whose cell disagrees with the singular
are written out; the rest follow the singular, so nothing is restated.

    cargo xtask morphdb local/research-impl/morphdb_morphdb.hu/in
    python tools/measure_possessed_plural.py

Writes local/lists/possessed-plural.tsv.
"""

import collections
import sqlite3
import sys
from paths import LISTS, WC2_DB

DB = WC2_DB
IN = LISTS / "possessed-plural-candidates.tsv"
OUT = LISTS / "possessed-plural.tsv"
FLOOR = 20


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
            rows.append((lemma, "ja" if nj > na else "a", nj, na))

    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write("lemma\tpossessed_plural\tn_ja\tn_a\n")
        for row in rows:
            fh.write("\t".join(map(str, row)) + "\n")
    print(OUT, len(rows), "attested")
    print("  ", dict(collections.Counter(r[1] for r in rows)))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else IN))
