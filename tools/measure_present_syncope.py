"""Measures whether an epenthetic verb syncopates in the present.

VRB-STEM-007 calls the syncope obligatory when the consonant after the unstable vowel is `g`, `z`
or `m`, but the corpus disagrees for a fifth of them, and the split is by cell rather than by
lexeme: `képez` gives `képzés` (664,766 : 78) and `képezi` (201,559 : 11,758). Only the verbs that
keep the full stem in the present are written out; the rest follow the rule.

The candidates come from the converter, which builds the syncopated one from the already-dropped
stem rather than by asking the engine: the engine's own gate is what is being measured, so for an
`l` verb both candidates came out identical and `közöl` was skipped as carrying no evidence.

    cargo xtask morphdb local/research-impl/morphdb_morphdb.hu/in
    python tools/measure_present_syncope.py

Writes local/lists/present-syncope.tsv.
"""

import collections
import sqlite3
import sys
from paths import LISTS, WC2_DB

DB = WC2_DB
IN = LISTS / "present-syncope-candidates.tsv"
OUT = LISTS / "present-syncope.tsv"
FLOOR = 20
# A minority form counts as attested at 200 tokens and one per cent of the cell.
OTHER_FLOOR = 200


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
            if len(parts) < 7:
                continue
            lemma, sync_def, full_def, sync_1sg, full_1sg, sync_past, full_past = parts[:7]

            # Each cell is decided on its own: the definite `-i` and the indefinite `-Ok` do not
            # agree for every verb, which is the whole reason this is measured per cell.
            def cell(sync_form, full_form):
                if sync_form == full_form:
                    return None, 0, 0
                s, f = freq(sync_form, lemma), freq(full_form, lemma)
                if s + f < FLOOR:
                    return None, False, s, f
                # Generation takes the winner; acceptance is more permissive, because a minority
                # form with real attestation is not an error. `képezi` outnumbers `képzi` 17 to 1
                # and both are Hungarian, so the cell reports a winner and a flag.
                winner = "full" if f > s else "syncopated"
                minority = min(s, f)
                other = minority >= OTHER_FLOOR and minority * 100 >= s + f
                return winner, other, s, f

            d, d_other, ds, df = cell(sync_def, full_def)
            o, o_other, os_, of = cell(sync_1sg, full_1sg)
            p_, p_other, ps, pf = cell(sync_past, full_past)
            if d is None and o is None and p_ is None:
                continue
            rows.append(
                (
                    lemma,
                    d or "-",
                    "yes" if d_other else "no",
                    o or "-",
                    "yes" if o_other else "no",
                    p_ or "-",
                    "yes" if p_other else "no",
                    ds, df, os_, of, ps, pf,
                )
            )

    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write(
            "lemma\tdefinite\tdefinite_other\tfirst\tfirst_other\tpast\tpast_other"
            "\tn_sync_def\tn_full_def\tn_sync_1sg\tn_full_1sg\tn_sync_past\tn_full_past\n"
        )
        for row in rows:
            fh.write("\t".join(map(str, row)) + "\n")
    print(f"{OUT}: {len(rows)} attested")
    for name, i in (
        ("definite", 1),
        ("definite_other", 2),
        ("first", 3),
        ("first_other", 4),
        ("past", 5),
        ("past_other", 6),
    ):
        print(f"  {name:9}", dict(collections.Counter(r[i] for r in rows)))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else IN))
