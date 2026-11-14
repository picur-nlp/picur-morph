"""Measures which `ó`/`ő` nouns take the `ajtó` possessive alternation.

`NOM-STEM-013` replaces the final vowel with `a`/`e` plus `j` before the possessive: `ajtó` gives
`ajtaja`, `idő` gives `ideje`. The class is closed and unpredictable from the spelling, and it is a
possessive-only alternation, so the plural and the accusative say nothing about it. The corpus
decides each lemma, with a tag and lemma filter so a homograph cannot vote.

    cargo xtask morphdb local/research-impl/morphdb_morphdb.hu/in
    python tools/measure_possessive_alt.py

Writes local/lists/possessive-alt.tsv.
"""

import collections
import sqlite3
import sys
from paths import LISTS, WC2_DB

DB = WC2_DB
IN = LISTS / "possessive-alt-candidates.tsv"
OUT = LISTS / "possessive-alt.tsv"
FLOOR = 10


def decide(alt, plain):
    if alt + plain < FLOOR:
        return "-"
    if alt > 3 * plain:
        return "alt"
    if plain > 3 * alt:
        return "plain"
    return "both"


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
            if len(parts) < 7:
                continue
            lemma, _pos, alt, plain, alt_ppl, alt_ppl_j, plain_ppl = parts[:7]
            a, p = freq(alt, lemma), freq(plain, lemma)
            # The stem question, independent of the allomorph: the alternating possessed plural is
            # `ajtajai` with the `j` and `szulei` without it, so whichever is attested answers it.
            ap = max(freq(alt_ppl, lemma), freq(alt_ppl_j, lemma))
            pp = freq(plain_ppl, lemma)
            rows.append((lemma, decide(a, p), decide(ap, pp), a, p, ap, pp))

    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write(
            "lemma\tpossessive\tpossessed_plural\tn_alt\tn_plain\tn_alt_ppl\tn_plain_ppl\n"
        )
        for row in rows:
            fh.write("\t".join(map(str, row)) + "\n")
    print(OUT, len(rows), "rows")
    for name, i in (("possessive", 1), ("possessed_plural", 2)):
        print(f"  {name:17}", dict(collections.Counter(r[i] for r in rows)))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else IN))
