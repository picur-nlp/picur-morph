#!/usr/bin/env python3
"""Shared machinery for the closed lexical lists: form building and cross-oracle voting."""
from __future__ import annotations

import csv
from dataclasses import dataclass, field
from pathlib import Path

import oracles as O
import webcorpus as W

from paths import LISTS as OUT

# corpus thresholds
MIN_TOTAL = 25       # below this the corpus abstains
DECISIVE = 0.95      # share of one variant that settles membership on its own
LEAN = 0.80          # weaker corpus lean: counts as one vote


# --- surface form building --------------------------------------------------

def harmony(lemma: str, back: bool = False) -> str:
    """B (back), R (front rounded), F (front unrounded).

    `back` is the lexical antiharmony flag and only applies to all-neutral stems; morphdb
    carries `back` and `front` on the same lemma often enough that trusting it elsewhere
    turns front stems into back ones.
    """
    v = O.vowels_of(lemma)
    if any(c in O.BACK_V for c in v):
        return "B"
    if back and v and all(c in O.NEUTRAL_V for c in v):
        return "B"
    if v and v[-1] in "öőüű":
        return "R"
    return "F"


LOW = {"B": "a", "R": "e", "F": "e"}
MID = {"B": "o", "R": "ö", "F": "e"}


def plural(stem: str, h: str, low: bool) -> str:
    return stem + (LOW if low else MID)[h] + "k"


def accusative(stem: str, h: str, low: bool, bare: bool = False) -> str:
    return stem + ("" if bare else (LOW if low else MID)[h]) + "t"


def dative(stem: str, h: str) -> str:
    return stem + ("nak" if h == "B" else "nek")


def poss3(stem: str, h: str, ja: bool) -> str:
    v = "a" if h == "B" else "e"
    return stem + ("j" if ja else "") + v


def poss1(stem: str, h: str, low: bool) -> str:
    return stem + (LOW if low else MID)[h] + "m"


def superessive(stem: str, h: str) -> str:
    return stem + MID[h] + "n"


def adverb_n(stem: str, h: str, low: bool) -> str:
    return stem + (LOW if low else MID)[h] + "n"


def past1sg(stem: str, h: str, vowel: bool) -> str:
    """-ttam/-ottam style 1sg indefinite past."""
    if vowel:
        return stem + MID[h] + "tt" + ("am" if h == "B" else "em")
    return stem + "t" + ("am" if h == "B" else "em")


def infinitive(stem: str, h: str, vowel: bool) -> str:
    if vowel:
        return stem + LOW[h] + "ni"
    return stem + "ni"


def pres3pl(stem: str, h: str, vowel: bool) -> str:
    if vowel:
        return stem + LOW[h] + ("nak" if h == "B" else "nek")
    return stem + ("nak" if h == "B" else "nek")


def deverbal_as(stem: str, h: str) -> str:
    return stem + ("ás" if h == "B" else "és")


# --- voting -----------------------------------------------------------------

@dataclass
class Vote:
    """One lemma's evidence for a binary membership question.

    `pos` forms are predicted by membership, `neg` forms by non-membership.
    """
    lemma: str
    pos: list
    neg: list
    prior: dict = field(default_factory=dict)   # oracle name -> True/False/None
    wc_lemma: str | None = None                 # emMorph lemma to restrict corpus counts to
    wc_pos: str | None = None
    n_pos: int = 0
    n_neg: int = 0
    votes: dict = field(default_factory=dict)
    decision: str = "review"
    confidence: str = "low"
    why: str = ""

    def summary(self) -> str:
        return ";".join(f"{k}={'+' if v else '-'}" for k, v in self.votes.items() if v is not None)

    def diag(self) -> str:
        return f"{'|'.join(self.pos)}={self.n_pos} {'|'.join(self.neg)}={self.n_neg}"


def collect(items: list, use_foma=True, use_hunspell=True, use_unimorph=True, use_kaikki=True):
    """Fill in corpus counts and oracle votes for a batch of Vote objects."""
    forms = [f for it in items for f in it.pos + it.neg]
    fm = O.foma_ok(forms) if use_foma else {}
    hs = O.hunspell(forms) if use_hunspell else {}
    for it in items:
        c = W.counts(it.pos + it.neg, lemma=it.wc_lemma, pos=it.wc_pos)
        it.n_pos = sum(c.get(f) or 0 for f in it.pos)
        it.n_neg = sum(c.get(f) or 0 for f in it.neg)
        v = dict(it.prior)
        total = it.n_pos + it.n_neg
        if total >= MIN_TOTAL:
            share = it.n_pos / total
            v["corpus"] = True if share >= LEAN else (False if share <= 1 - LEAN else None)
        else:
            v["corpus"] = None
        if use_foma:
            p, n = any(fm.get(f) for f in it.pos), any(fm.get(f) for f in it.neg)
            v["foma"] = True if p and not n else (False if n and not p else None)
        if use_hunspell:
            p, n = any(hs.get(f) for f in it.pos), any(hs.get(f) for f in it.neg)
            v["hunspell"] = True if p and not n else (False if n and not p else None)
        if use_unimorph:
            p = any(O.unimorph_has(it.lemma, f) for f in it.pos)
            n = any(O.unimorph_has(it.lemma, f) for f in it.neg)
            v["unimorph"] = True if p and not n else (False if n and not p else None)
        if use_kaikki:
            p = any(O.kaikki_has(it.lemma, f) for f in it.pos)
            n = any(O.kaikki_has(it.lemma, f) for f in it.neg)
            v["kaikki"] = True if p and not n else (False if n and not p else None)
        it.votes = v
        _decide(it)
    return items


def _decide(it: Vote):
    total = it.n_pos + it.n_neg
    share = it.n_pos / total if total else None
    cast = [v for v in it.votes.values() if v is not None]
    yes, no = cast.count(True), cast.count(False)
    if total >= MIN_TOTAL and share is not None and (share >= DECISIVE or share <= 1 - DECISIVE):
        it.decision = "member" if share >= DECISIVE else "non-member"
        it.confidence = "high"
        it.why = f"corpus decisive {it.n_pos}/{total}"
        # a declared oracle contradicting a decisive corpus is worth flagging
        if (it.decision == "member" and no and not yes) or (it.decision == "non-member" and yes and not no):
            it.confidence = "medium"
            it.why += "; oracles disagree"
        return
    if cast and yes and not no:
        it.decision, it.confidence, it.why = "member", ("medium" if len(cast) > 1 else "low"), "oracles unanimous"
        return
    if cast and no and not yes:
        it.decision, it.confidence, it.why = "non-member", ("medium" if len(cast) > 1 else "low"), "oracles unanimous"
        return
    it.decision, it.confidence = "review", "low"
    it.why = "no evidence" if not cast and not total else f"split {yes}+/{no}- corpus {it.n_pos}/{it.n_neg}"


def write(name: str, items: list, extra_cols=None, extra=None):
    """Write a list TSV: lemma, decision, evidence, confidence, diagnostics."""
    OUT.mkdir(parents=True, exist_ok=True)
    path = OUT / f"{name}.tsv"
    cols = ["lemma", "decision", "evidence", "confidence", "diagnostics", "n_pos", "n_neg"]
    if extra_cols:
        cols += extra_cols
    with path.open("w", encoding="utf-8", newline="") as fh:
        w = csv.writer(fh, delimiter="\t", lineterminator="\n")
        w.writerow(cols)
        for it in sorted(items, key=lambda x: (x.decision, -(x.n_pos + x.n_neg), x.lemma)):
            row = [it.lemma, it.decision, it.summary(), it.confidence, it.diag(), it.n_pos, it.n_neg]
            if extra_cols:
                row += list(extra(it))
            w.writerow(row)
    counts = {}
    for it in items:
        counts[it.decision] = counts.get(it.decision, 0) + 1
    print(f"{name}: {len(items)} rows {counts}")
    return counts
