#!/usr/bin/env python3
"""Form+lemma frequency index over Webcorpus2FrequencyList (MIT, Zenodo 10.5281/zenodo.17508385).

The parquet is streamed once into a sqlite table keyed by surface form, keeping the lemma and
the emMorph POS category so that a diagnostic form can be counted for the intended lemma only
(`eszak` under lemma `esz` vs the unrelated noun `eszak`).
"""
from __future__ import annotations

import re
import sqlite3
from pathlib import Path

from paths import WC2_DB as INDEX, WC2_PARQUET as PARQUET

POS = re.compile(r"^\[/([A-Za-z]+)")


def build(force: bool = False) -> Path:
    if INDEX.exists() and not force:
        return INDEX
    import pyarrow.parquet as pq
    INDEX.parent.mkdir(parents=True, exist_ok=True)
    tmp = INDEX.with_suffix(".tmp")
    tmp.unlink(missing_ok=True)
    db = sqlite3.connect(tmp)
    db.executescript("pragma journal_mode=off; pragma synchronous=off;"
                     "create table f(form text, lemma text, pos text, freq integer);")
    pf = pq.ParquetFile(PARQUET)
    for i in range(pf.num_row_groups):
        t = pf.read_row_group(i, columns=["form", "lemma", "xpostag", "freq"]).to_pydict()
        rows = []
        for form, lemma, tag, fr in zip(t["form"], t["lemma"], t["xpostag"], t["freq"]):
            if form is None:
                continue
            m = POS.match(tag or "")
            rows.append((form, lemma, m.group(1) if m else "", int(fr or 0)))
        db.executemany("insert into f values(?,?,?,?)", rows)
        db.commit()
    db.execute("create index f_form on f(form)")
    db.commit()
    db.close()
    tmp.replace(INDEX)
    return INDEX


_DB = None


def connect():
    global _DB
    if _DB is None:
        build()
        _DB = sqlite3.connect(f"file:{INDEX}?mode=ro", uri=True)
    return _DB


def counts(forms, lemma: str | None = None, pos: str | None = None) -> dict:
    """{form: freq}; with `lemma` set, only rows analysed as that lemma are counted."""
    forms = list(dict.fromkeys(forms))
    out = {f: 0 for f in forms}
    if not forms:
        return out
    db = connect()
    q = "select form, sum(freq) n from f where form in (%s)" % ",".join("?" * len(forms))
    args = list(forms)
    if lemma:
        q += " and lemma=?"
        args.append(lemma)
    if pos:
        q += " and pos=?"
        args.append(pos)
    q += " group by form"
    for form, n in db.execute(q, args):
        out[form] = int(n)
    return out


def lemma_freq(lemma: str, pos: str | None = None) -> int:
    db = connect()
    q, args = "select sum(freq) from f where lemma=?", [lemma]
    if pos:
        q += " and pos=?"
        args.append(pos)
    return int(db.execute(q, args).fetchone()[0] or 0)


if __name__ == "__main__":
    import time
    t = time.time()
    print(build(), round(time.time() - t, 1), "s")
    print(counts(["hidak", "hidek", "eszak", "eszek"], lemma=None))
    print(counts(["ak", "ok"]))
