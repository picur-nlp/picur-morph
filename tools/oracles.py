#!/usr/bin/env python3
"""Oracle loaders for the lexical-list induction.

Licence discipline:
  morphdb (CC BY 2.5), hunmorph-foma (BSD), hunspell hu_HU (MPL), our own lexicon
  -> strings may be copied into our lists.
  kaikki-hu, UniMorph hun (CC BY-SA) -> boolean agree/disagree signals only, never strings.
"""
from __future__ import annotations

import json
import os
import re
import subprocess
import sys
from collections import defaultdict
from functools import lru_cache
from pathlib import Path

from paths import CACHE, LEXDATA, RESEARCH

MORPHDB = RESEARCH / "morphdb_morphdb.hu/in"
FOMA = RESEARCH / "r0ller_hunmorph-foma"
UNIMORPH = RESEARCH / "unimorph_hun/hun"
KAIKKI = RESEARCH / "kaikki/kaikki.org-dictionary-Hungarian.jsonl"

VOWELS = "aáeéiíoóöőuúüű"
BACK_V = "aáoóuú"
FRONT_V = "eéiíöőüű"
NEUTRAL_V = "iíeé"


# --- morphdb ----------------------------------------------------------------

@lru_cache(maxsize=1)
def morphdb() -> dict:
    """{(lemma, pos): set(flags)} from peeta.lexicon; '#'-prefixed (rejected) entries skipped."""
    out = defaultdict(set)
    txt = (MORPHDB / "peeta.lexicon").read_bytes().decode("iso-8859-2")
    for line in txt.splitlines():
        if not line or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) < 2:
            continue
        lemma, rest = parts[0], parts[1]
        rest = re.split(r"\s+(?:SRC:|DOM:|STR:|USAGE:)", rest)[0]
        toks = [t for t in rest.split() if t and t not in (",;", ",")]
        if not toks:
            continue
        pos, flags = toks[0], set(toks[1:])
        out[(lemma, pos)] |= flags
    return dict(out)


@lru_cache(maxsize=1)
def morphdb_by_pos() -> dict:
    """{pos: {lemma: set(flags)}}"""
    out = defaultdict(dict)
    for (lemma, pos), flags in morphdb().items():
        out[pos][lemma] = flags
    return dict(out)


@lru_cache(maxsize=1)
def morphdb_str() -> dict:
    """{(lemma, pos): STR field} - derivational/compound split, used to test monomorphemicity.

    POS-keyed: the verb `fal` is split `f|al` while the noun `fal` is a root.
    """
    out = {}
    txt = (MORPHDB / "peeta.lexicon").read_bytes().decode("iso-8859-2")
    for line in txt.splitlines():
        if not line or line.startswith("#"):
            continue
        m = re.search(r"STR:\s*(\S+)", line)
        if not m:
            continue
        parts = line.split("\t")
        if len(parts) < 2:
            continue
        pos = parts[1].split()[0]
        out.setdefault((parts[0], pos), m.group(1))
    return out


# --- our lexicon ------------------------------------------------------------

ENTRY = re.compile(r'\(lemma:"((?:[^"\\]|\\.)*)"(?:,parts:\[[^\]]*\])?(?:,traits:\[([^\]]*)\])?\)')


@lru_cache(maxsize=8)
def lexicon(pos: str) -> dict:
    """{lemma: set(traits)} from our own *.ron (pos in noun/adjective/verb)."""
    out = {}
    for m in ENTRY.finditer((LEXDATA / f"{pos}.ron").read_text(encoding="utf-8")):
        traits = set(t.strip() for t in (m.group(2) or "").split(",") if t.strip())
        out[m.group(1)] = traits
    return out


# --- hunmorph-foma ----------------------------------------------------------

FSTS = ("hunnoun.fst", "hunverb.fst", "hunadj.fst", "hunmisc.fst")


def foma(forms, fsts=FSTS) -> dict:
    """{form: set(analyses)}; empty set = rejected by every transducer."""
    forms = list(dict.fromkeys(forms))
    out = {f: set() for f in forms}
    if not forms:
        return out
    payload = "\n".join(forms) + "\n"
    for fst in fsts:
        p = subprocess.run(["flookup", str(FOMA / fst)], input=payload,
                           capture_output=True, text=True)
        for line in p.stdout.splitlines():
            if "\t" not in line:
                continue
            form, ana = line.split("\t", 1)
            if ana != "+?" and form in out:
                out[form].add(ana)
    return out


def foma_ok(forms, fsts=FSTS) -> dict:
    return {f: bool(a) for f, a in foma(forms, fsts).items()}


# --- hunspell ---------------------------------------------------------------

HUNSPELL_DIC = "/usr/share/hunspell/hu_HU"


def hunspell(forms) -> dict:
    """{form: accepted?} via `hunspell -l` (lists misspellings)."""
    forms = list(dict.fromkeys(forms))
    if not forms:
        return {}
    p = subprocess.run(["hunspell", "-d", HUNSPELL_DIC, "-l"],
                       input="\n".join(forms) + "\n", capture_output=True, text=True)
    bad = set(p.stdout.split())
    return {f: f not in bad for f in forms}


# --- UniMorph (CC BY-SA: boolean signal only) -------------------------------

@lru_cache(maxsize=1)
def _unimorph() -> dict:
    out = defaultdict(set)
    for line in UNIMORPH.read_text(encoding="utf-8").splitlines():
        p = line.split("\t")
        if len(p) == 3:
            out[p[0]].add(p[1])
    return dict(out)


def unimorph_has(lemma: str, form: str) -> bool:
    return form in _unimorph().get(lemma, ())


def unimorph_forms_count(lemma: str) -> int:
    return len(_unimorph().get(lemma, ()))


# --- kaikki (CC BY-SA: boolean signal only) ---------------------------------

@lru_cache(maxsize=1)
def _kaikki() -> dict:
    out = defaultdict(set)
    if not KAIKKI.exists():
        return {}
    with KAIKKI.open(encoding="utf-8") as fh:
        for line in fh:
            try:
                d = json.loads(line)
            except ValueError:
                continue
            w = d.get("word")
            if not w:
                continue
            for f in d.get("forms", ()):
                if isinstance(f, dict) and f.get("form"):
                    out[w].add(f["form"])
    return dict(out)


def kaikki_has(lemma: str, form: str) -> bool:
    return form in _kaikki().get(lemma, ())


def kaikki_known(lemma: str) -> bool:
    return lemma in _kaikki()


# --- corpus -----------------------------------------------------------------

@lru_cache(maxsize=1)
def _freq() -> dict:
    txt = (LEXDATA / "frequency.ron").read_text(encoding="utf-8")
    return {m.group(1): int(m.group(2)) for m in re.finditer(r'\("([^"]+)",(\d+)\)', txt)}


def freq(form: str) -> int:
    """Bucket from frequency.ron (half-octave rank); 0 = unattested in the top-250k list."""
    return _freq().get(form.lower(), 0)


_DB = None


def corpus_counts(forms) -> dict:
    """{form: total absolute count} over the corpus count index, if it is built."""
    global _DB
    forms = list(dict.fromkeys(forms))
    try:
        sys.path.insert(0, str(Path(__file__).resolve().parent))
        import corpus as C
        if _DB is None:
            _DB = C.connect()
        raw = C.counts_for(_DB, forms)
        return {f: sum(v.values()) for f, v in raw.items()}
    except Exception:
        return {f: None for f in forms}


# --- phonology helpers ------------------------------------------------------

def vowels_of(s: str) -> str:
    return "".join(c for c in s.lower() if c in VOWELS)


def syllables(s: str) -> int:
    return len(vowels_of(s))


def harmony_shape(s: str) -> str:
    """B if a back vowel is present, N if all vowels are neutral, F otherwise."""
    v = vowels_of(s)
    if any(c in BACK_V for c in v):
        return "B"
    if v and all(c in NEUTRAL_V for c in v):
        return "N"
    return "F"


def is_rounded_front(s: str) -> bool:
    return any(c in "öőüű" for c in vowels_of(s))


def back_pair(stem: str, back: str, front: str) -> tuple:
    return stem + back, stem + front
