#!/usr/bin/env python3
"""PH-ZERO-001 / PH-ZERO-003: epenthetic (unstable-vowel) noun and verb stems.

Shape filter (necessary, from PH-ZERO-001): at least two syllables, `...V(V)C Vu C#`,
one consonant on each side of the unstable vowel, single final consonant, flanking
consonants not identical.
Diagnostic: the syncopated alternant against the full stem in a triggering cell -
nouns: plural and accusative; verbs: the `-ás/-és` deverbal noun.
"""
import re
import sys

import listgen as L
import oracles as O

MD = O.morphdb_by_pos()
STR = O.morphdb_str()


def kind(lemma, pos):
    """root / compound / derived - PH-ZERO-001 counts monomorphemic nouns separately."""
    st = STR.get((lemma, pos), "")
    if "@" in st:
        return "compound"
    if lemma.endswith(("alom", "elem")):
        return "deriv-alom"
    if "|" in st:
        return "derived"
    return "root"
LEXN = O.lexicon("noun")
LEXV = O.lexicon("verb")

DIGRAPH = ("cs", "dz", "gy", "ly", "ny", "sz", "ty", "zs")
UNSTABLE = "oeöaui"


def split_consonant(s):
    """Return the consonant sequence at the end of `s` as a list of graphemes."""
    out = []
    i = 0
    while i < len(s):
        if s[i:i + 3] == "dzs":
            out.append("dzs"); i += 3
        elif s[i:i + 2] in DIGRAPH:
            out.append(s[i:i + 2]); i += 2
        else:
            out.append(s[i]); i += 1
    return out


SHAPE = re.compile(r"^(.*[" + O.VOWELS + r"])([^" + O.VOWELS + r"]+)([" + UNSTABLE + r"])([^" + O.VOWELS + r"]+)$")


def shape_ok(lemma):
    """Split into (prefix, C1, Vu, C2) and check the single-consonant flanks."""
    m = SHAPE.match(lemma)
    if not m:
        return None
    head, c1, vu, c2 = m.groups()
    g1, g2 = split_consonant(c1), split_consonant(c2)
    if len(g1) != 1 or len(g2) != 1 or g1[0] == g2[0]:
        return None
    if O.syllables(lemma) < 2:
        return None
    return head + c1, vu, c2


def build_nouns():
    items = []
    for lemma, flags in MD["NOUN"].items():
        if not lemma or not lemma[0].islower() or not lemma.isalpha():
            continue
        sp = shape_ok(lemma)
        if not sp:
            continue
        bound, vu, c2 = sp
        h = L.harmony(lemma, back="back" in flags)
        syn = bound + c2
        pos_f = [L.plural(syn, h, True), L.plural(syn, h, False),
                 L.accusative(syn, h, True), L.accusative(syn, h, False)]
        neg_f = [L.plural(lemma, h, True), L.plural(lemma, h, False),
                 L.accusative(lemma, h, True), L.accusative(lemma, h, False)]
        prior = {"morphdb": "epenthetic" in flags, "ours": "Epenthetic" in LEXN.get(lemma, set())}
        it = L.Vote(lemma, pos_f, neg_f, prior, wc_lemma=lemma)
        it.vu = vu
        it.kind = kind(lemma, "NOUN")
        items.append(it)
    L.collect(items)
    return items


def build_verbs():
    items = []
    for lemma, flags in MD["VERB"].items():
        if not lemma or not lemma[0].islower() or not lemma.isalpha():
            continue
        if "preverb" in flags:
            continue
        stem = lemma[:-2] if lemma.endswith("ik") else lemma
        sp = shape_ok(stem)
        if not sp:
            continue
        bound, vu, c2 = sp
        h = L.harmony(stem, back="back" in flags)
        syn = bound + c2
        pos_f = [L.deverbal_as(syn, h)]
        neg_f = [L.deverbal_as(stem, h)]
        prior = {"morphdb": "epenthetic" in flags, "ours": "Epenthetic" in LEXV.get(lemma, set())}
        it = L.Vote(lemma, pos_f, neg_f, prior)
        it.vu = vu
        it.kind = kind(lemma, "VERB")
        items.append(it)
    L.collect(items, use_unimorph=False, use_kaikki=False)
    return items


def main():
    which = sys.argv[1] if len(sys.argv) > 1 else "both"
    if which in ("both", "noun"):
        items = build_nouns()
        keep = [it for it in items if it.n_pos + it.n_neg >= L.MIN_TOTAL or it.votes.get("morphdb")]
        L.write("epenthetic-nouns", keep, ["unstable_vowel", "kind"], lambda it: (it.vu, it.kind))
        m = [it for it in keep if it.decision == "member"]
        by = {}
        for it in m:
            by.setdefault(it.kind, []).append(it.lemma)
        print("epenthetic nouns:", len(m), {k: len(v) for k, v in by.items()}, "of", len(items), "candidates")
        print("roots:", " ".join(sorted(by.get("root", []))))
    if which in ("both", "verb"):
        items = build_verbs()
        keep = [it for it in items if it.n_pos + it.n_neg >= L.MIN_TOTAL or it.votes.get("morphdb")]
        L.write("epenthetic-verbs", keep, ["unstable_vowel", "kind"], lambda it: (it.vu, it.kind))
        m = [it for it in keep if it.decision == "member"]
        by = {}
        for it in m:
            by.setdefault(it.kind, []).append(it.lemma)
        print("epenthetic verbs:", len(m), {k: len(v) for k, v in by.items()}, "of", len(items), "candidates")
        print("roots:", " ".join(sorted(by.get("root", []))))


if __name__ == "__main__":
    main()
