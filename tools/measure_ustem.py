"""Measures which `-ú`/`-ű` nominals drop the final vowel before a suffix.

`NOM-STEM-012` drops it for the u-stem class and `NOM-ADJ-005` names four adjectives that do,
but morphdb carries the trait for nouns only and never for adjectives, so the class cannot be
read off the flags: `ifjú` was unmarked, which left `ifjak` ill formed, while `szomorú` really
does keep its vowel (`szomorúak`).

    cargo xtask morphdb local/research-impl/morphdb_morphdb.hu/in
    python tools/measure_ustem.py

Writes local/lists/ustem.tsv.
"""

import collections
import sqlite3
import sys
from paths import LISTS, WC2_DB

DB = WC2_DB
IN = LISTS / "ustem-candidates.tsv"
OUT = LISTS / "ustem.tsv"
FLOOR = 20
# Adding the trait deletes the form that keeps the vowel, so it needs more than a bare majority.
MARK_FLOOR = 200


def decide(drop, keep):
    if drop + keep < FLOOR:
        return "-"
    if drop > 3 * keep:
        return "drop" if drop >= MARK_FLOOR else "-"
    if keep > 3 * drop:
        return "keep"
    return "both"


def main(path=IN):
    db = sqlite3.connect(DB)
    # Our categories are coarser than the corpus tagset, and these words cross it constantly:
    # `ifjú` is both a noun and an adjective.
    TAG = {"Noun": ("N", "Adj", "Num"), "Adjective": ("Adj", "N", "Num")}

    def freq(form, lemma, tags):
        marks = ",".join("?" * len(tags))
        row = db.execute(
            f"select coalesce(sum(freq),0) from f where form=? and lemma=? and pos in ({marks})",
            (form, lemma, *tags),
        ).fetchone()
        return row[0] or 0

    rows = []
    with open(path, encoding="utf-8") as fh:
        for line in fh.read().splitlines()[1:]:
            parts = line.split("\t")
            if len(parts) < 6:
                continue
            lemma, pos, drop_pl, keep_pl, drop_acc, keep_acc = parts[:6]
            tag = TAG.get(pos)
            if not tag:
                continue

            def cell(a, b, lemma=lemma, tag=tag):
                if a == b:
                    return "-", 0, 0
                x, y = freq(a, lemma, tag), freq(b, lemma, tag)
                return decide(x, y), x, y

            p, pl_drop, pl_keep = cell(drop_pl, keep_pl)
            a, acc_drop, acc_keep = cell(drop_acc, keep_acc)
            if p == "-" and a == "-":
                continue
            rows.append((lemma, pos, p, a, pl_drop, pl_keep, acc_drop, acc_keep))

    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write(
            "lemma\tpos\tplural\taccusative"
            "\tn_drop_pl\tn_keep_pl\tn_drop_acc\tn_keep_acc\n"
        )
        for row in rows:
            fh.write("\t".join(map(str, row)) + "\n")
    print(OUT, len(rows), "attested")
    for name, i in (("plural", 2), ("accusative", 3)):
        print(f"  {name:11}", dict(collections.Counter(r[i] for r in rows)))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else IN))
