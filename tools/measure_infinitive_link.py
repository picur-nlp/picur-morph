"""Measures whether a verb takes the linking vowel before the infinitive `-ni`.

`VRB-LINK-002` computes heaviness from the coda, which is right for a cluster (`tartani`) and
wrong for several single-consonant stems: the forms are `edzeni` and `bocsatani`, not `edzni` and
`bocsatni`, and `ved` takes both. The corpus decides each verb.

    cargo xtask morphdb local/research-impl/morphdb_morphdb.hu/in
    python tools/measure_infinitive_link.py

Writes local/lists/infinitive-link.tsv.
"""

import collections
import sqlite3
import sys
from paths import LISTS, WC2_DB

DB = WC2_DB
IN = LISTS / "infinitive-candidates.tsv"
OUT = LISTS / "infinitive-link.tsv"
FLOOR = 20


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
            if len(parts) < 3:
                continue
            lemma, linked, bare = parts
            nj, na = freq(linked, lemma), freq(bare, lemma)
            if nj + na < FLOOR:
                continue
            # Generation takes the winner; a minority form with real attestation is accepted.
            minority = min(nj, na)
            other = minority >= 200 and minority * 100 >= nj + na
            rows.append(
                (lemma, "linked" if nj > na else "bare", "yes" if other else "no", nj, na)
            )

    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write("lemma\tinfinitive\tother\tn_linked\tn_bare\n")
        for row in rows:
            fh.write("\t".join(map(str, row)) + "\n")
    print(OUT, len(rows), "attested")
    print("  ", dict(collections.Counter(r[1] for r in rows)))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else IN))
