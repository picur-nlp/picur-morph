"""Measures which cells a noun's lowering marking reaches.

`NOM-LOW-001` marks lowering per lexeme, and the plural and the accusative come apart for a
measurable class: `olaj` takes `olajat` 156,574 against `olajot` 12, but `olajok` 43,083 against
`olajak` 3,789. Only the nouns whose plural is not lowering are written out; the rest keep the
marking for both cells, so nothing is restated.

    cargo xtask morphdb local/research-impl/morphdb_morphdb.hu/in
    python tools/measure_lowering.py

Writes local/lists/lowering-cells.tsv.
"""

import collections
import sqlite3
import sys
from paths import LISTS, WC2_DB

DB = WC2_DB
IN = LISTS / "lowering-candidates.tsv"
OUT = LISTS / "lowering-cells.tsv"
FLOOR = 20
# An exemption removes a form from the language, so it needs more evidence than a confirmation
# does. `masik` is tagged an adjective for 220,235 of its accusatives and our entry is a noun, so
# the 179 noun-tagged tokens were enough to exempt the lexeme and lose `masikat` altogether.
EXEMPT_FLOOR = 200


def decide(low, plain):
    if low + plain < FLOOR:
        return "-"
    # Symmetric bars: adding a marking and removing one both change which forms are words, so
    # each needs real support rather than a bare majority.
    if low > 3 * plain:
        return "low" if low >= EXEMPT_FLOOR else "-"
    if plain > 3 * low:
        return "plain" if plain >= EXEMPT_FLOOR else "-"
    return "both"


def main(path=IN):
    db = sqlite3.connect(DB)

    # The tag filter is what keeps a homograph from voting: `piros` is primarily an adjective, so
    # a noun-only query scored `pirosak` at zero and made the whole lexeme look non-lowering.
    #
    # Our categories are coarser than the corpus tagset, so each maps to a set. The closed-class
    # quantifiers are the reason: `nehanyat` 86,124 and `parat` 57,127 are tagged Num while our
    # entries are nouns, and a noun-only query found neither. For `par` it found the unrelated
    # noun accusative `part` 26,964 instead and called the lexeme non-lowering.
    TAG = {"Noun": ("N", "Num"), "Adjective": ("Adj", "Num")}

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
            if len(parts) < 8:
                continue
            lemma, pos, low_pl, plain_pl, low_acc, plain_acc, low_poss, plain_poss = parts[:8]
            tag = TAG.get(pos)
            if not tag:
                continue

            def cell(a, b, lemma=lemma, tag=tag):
                if a == b:
                    return "-", 0, 0
                x, y = freq(a, lemma, tag), freq(b, lemma, tag)
                return decide(x, y), x, y

            p, pl_low, pl_plain = cell(low_pl, plain_pl)
            a, acc_low, acc_plain = cell(low_acc, plain_acc)
            o, po_low, po_plain = cell(low_poss, plain_poss)
            if p == "-" and a == "-" and o == "-":
                continue
            rows.append(
                (lemma, pos, p, a, o, pl_low, pl_plain, acc_low, acc_plain, po_low, po_plain)
            )

    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write(
            "lemma\tpos\tplural\taccusative\tpossessive"
            "\tn_low_pl\tn_plain_pl\tn_low_acc\tn_plain_acc\tn_low_poss\tn_plain_poss\n"
        )
        for row in rows:
            fh.write("\t".join(map(str, row)) + "\n")
    print(OUT, len(rows), "attested")
    for name, i in (("plural", 2), ("accusative", 3), ("possessive", 4)):
        print(f"  {name:11}", dict(collections.Counter(r[i] for r in rows)))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else IN))
