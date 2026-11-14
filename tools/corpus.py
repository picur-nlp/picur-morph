#!/usr/bin/env python3
"""Shared corpus metadata and the count index used by ratios.py and kwic.py."""
from __future__ import annotations

import json
import math
import sqlite3
from dataclasses import dataclass
from pathlib import Path

from paths import CACHE, CORPORA, COUNTS

INDEX = CACHE / "counts.sqlite"

GROUPS = {
    "official": ("gazette",),
    "literary": ("gutenberg",),
    "web": ("hplt", "hq"),
}

PERMISSIVE = {s for srcs in GROUPS.values() for s in srcs}
"""Sources whose figures may be published: every source under `local/corpora/`."""

GROUP_OF = {src: g for g, srcs in GROUPS.items() for src in srcs}
GROUP_ORDER = ("official", "literary", "web")

FLOOR = 20
"""Minimum absolute count of a variant in a source before its ratio is quoted."""


def publishable(sources) -> bool:
    return all(s in PERMISSIVE for s in sources)


# --- rate index -------------------------------------------------------------

def build_index(force: bool = False) -> Path:
    """One-off conversion of the `cargo xtask frequency` counts into a form-keyed sqlite table."""
    if INDEX.exists() and not force:
        return INDEX
    CACHE.mkdir(parents=True, exist_ok=True)
    tmp = INDEX.with_suffix(".tmp")
    tmp.unlink(missing_ok=True)
    data = json.load(COUNTS.open(encoding="utf-8"))
    scanned = data["scanned"]
    db = sqlite3.connect(tmp)
    db.executescript(
        "pragma journal_mode=off; pragma synchronous=off;"
        "create table counts(form text, source text, n integer);"
        "create table scanned(source text primary key, chars integer);"
    )
    db.executemany("insert into scanned values(?,?)", scanned.items())
    for source, forms in data["words"].items():
        db.executemany("insert into counts values(?,?,?)", ((f, source, n) for f, n in forms.items()))
    db.commit()
    db.execute("create index counts_form on counts(form)")
    db.commit()
    db.close()
    tmp.replace(INDEX)
    return INDEX


def connect() -> sqlite3.Connection:
    build_index()
    db = sqlite3.connect(f"file:{INDEX}?mode=ro", uri=True)
    db.row_factory = sqlite3.Row
    return db


def scanned_chars(db) -> dict:
    return {r["source"]: r["chars"] for r in db.execute("select * from scanned")}


def counts_for(db, forms) -> dict:
    """{form: {source: absolute count}} for the given surface forms."""
    out = {f: {} for f in forms}
    qs = ",".join("?" * len(forms))
    where = "form in (%s)" % qs
    args = list(forms)
    for r in db.execute(f"select form, source, sum(n) n from counts where {where} group by form, source", args):
        out[r["form"]][r["source"]] = r["n"]
    return out


# --- statistics -------------------------------------------------------------

def wilson(k: int, n: int, z: float = 1.959963985) -> tuple:
    """Wilson score interval for k successes out of n."""
    if n == 0:
        return (0.0, 0.0, 1.0)
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return (p, max(0.0, c - h), min(1.0, c + h))


@dataclass
class Cell:
    """One competing form's count in one bucket."""
    form: str
    n: int


def ratio_rows(counts: dict, forms: list, bucket_of) -> dict:
    """Aggregate {form: {source: n}} into {bucket: {form: n}} using bucket_of(source)."""
    out = {}
    for form in forms:
        for source, n in counts.get(form, {}).items():
            b = bucket_of(source)
            if b is None:
                continue
            out.setdefault(b, {f: 0 for f in forms})[form] += n
    return out
