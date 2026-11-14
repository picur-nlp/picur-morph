"""Measures the past marker of the verbs whose past VRB-PAST-004/008 call lexical.

Both axes are measured separately, because they come apart: `függ` is `függött` word-finally but
`függtem` before a person suffix. The decision is the corpus majority above a 20-occurrence floor;
an exact tie or an unattested verb is left unmarked so the rules decide.

    python tools/measure_past.py

Writes local/lists/geminate-past.tsv and local/lists/ad-past.tsv.
"""

import collections
import sqlite3
import subprocess
import sys
from paths import LISTS, WC2_DB

DB = WC2_DB
PERSON = ("tam", "tem", "tál", "tél", "tunk", "tünk", "tatok", "tetek", "tak", "tek")
LINKED = ("am", "em", "ál", "él", "unk", "ünk", "atok", "etek", "ak", "ek")
FLOOR = 20


def decide(bare, linked):
    if bare + linked < FLOOR or bare == linked:
        return "-"
    return "t" if bare > linked else "ott"


def measure(db, lemmas, vowels, out):
    def freq(form):
        row = db.execute("select coalesce(sum(freq),0) from f where form=?", (form,)).fetchone()
        return row[0] or 0

    rows = []
    for lemma in lemmas:
        final_t = freq(lemma + "t")
        final_o = sum(freq(lemma + v) for v in vowels)
        nonfinal_t = sum(freq(lemma + s) for s in PERSON)
        nonfinal_o = sum(freq(lemma + v + s) for v in vowels for s in LINKED)
        rows.append(
            (
                lemma,
                decide(final_t, final_o),
                decide(nonfinal_t, nonfinal_o),
                final_t,
                final_o,
                nonfinal_t,
                nonfinal_o,
            )
        )
    with open(out, "w", encoding="utf-8") as fh:
        fh.write("lemma\tfinal\tnonfinal\tn_final_t\tn_final_ott\tn_nonfinal_t\tn_nonfinal_ott\n")
        for row in rows:
            fh.write("\t".join(map(str, row)) + "\n")
    print(
        out,
        len(rows),
        "final:",
        dict(collections.Counter(r[1] for r in rows)),
        "nonfinal:",
        dict(collections.Counter(r[2] for r in rows)),
    )


def geminates():
    with open(LISTS / "geminate-verbs.tsv", encoding="utf-8") as fh:
        return [line.split("\t")[0] for line in fh.read().splitlines()[1:] if line.strip()]


def ad_verbs():
    """Every `-ad`/`-ed` verb the lexicon knows, listed by the helper example."""
    out = subprocess.run(
        ["cargo", "run", "-q", "--release", "-p", "picur-morph-core", "--example", "ad_verbs"],
        capture_output=True,
        text=True,
        check=True,
    )
    return [line.strip() for line in out.stdout.splitlines() if line.strip()]


def main():
    db = sqlite3.connect(DB)
    measure(db, geminates(), ("ott", "ett", "ött"), LISTS / "geminate-past.tsv")
    measure(db, ad_verbs(), ("ott", "ett"), LISTS / "ad-past.tsv")
    return 0


if __name__ == "__main__":
    sys.exit(main())
