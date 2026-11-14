#!/usr/bin/env python3
"""NOM-LOW-004 / NOM-LOW-007: lowering monomorphemic nouns and non-lowering adjectives.

Two independent diagnostics per lemma, because a minority of stems lower in one cell only:
  plural:     B `-ak` vs `-ok`, R `-ek` vs `-ök`   (F: quality identical, no signal)
  accusative: B `-at` vs `-ot`, R `-et` vs `-öt`, F `-et` vs bare `-t`
Agreement -> lowering / non-lowering; disagreement -> `variable` (the small variable group
of Tab21 Table 9). Alternating stems (epenthetic / shortening / v_stem) are lowering by
rule (NOM-LOW-005) and are not induced here. Derived and compound lemmas are excluded:
NOM-LOW-006 decides them from the suffix.
"""
import re
import sys

import listgen as L
import oracles as O

MD = O.morphdb_by_pos()
STR = O.morphdb_str()
LEXN = O.lexicon("noun")
LEXA = O.lexicon("adjective")

ALT = {"epenthetic", "shortening", "v_stem", "oa_stem", "u_stem"}
BARE_T = ("j", "l", "ly", "n", "ny", "r", "s", "sz", "z", "zs")

# NOM-LOW-006 decides these by rule, so they are not lexical members
DERIV = re.compile(
    r"(ság|ség|ás|és|zat|zet|alom|elem|mány|mény|vány|vény|ász|ész|nok|nök|ár|ér"
    r"|talan|telen|atlan|etlen|tlan|tlen|ékony|ékeny|szerű|nemű|beli|ódik|kodik"
    r"|ott|ett|ött|őtt|att|ítt|dt|lt|rt|nt|us|izmus|ista|ció|atív|ív)$")


def _splittable(lemma, vocab):
    """Compound test: lemma = A + B, both known, A at least 2 and B at least 3 letters."""
    for i in range(2, len(lemma) - 2):
        if lemma[:i] in vocab and lemma[i:] in vocab:
            return True
    return False


def _derived_s(lemma, vocab):
    """`fustos` = `fust` + adjectival -(V)s: NOM-LOW-006 decides it, not the lexicon."""
    if not lemma.endswith("s") or len(lemma) < 4 or lemma[-2] not in O.VOWELS:
        return False
    return lemma[:-2] in vocab or lemma[:-1] in vocab


def final_consonants(lemma):
    """Number of consonant graphemes in the final cluster."""
    i = max(lemma.rfind(c) for c in O.VOWELS if c in lemma) if any(c in O.VOWELS for c in lemma) else -1
    tail = lemma[i + 1:]
    n, j = 0, 0
    while j < len(tail):
        if tail[j:j + 3] == "dzs":
            j += 3
        elif tail[j:j + 2] in ("cs", "dz", "gy", "ly", "ny", "sz", "ty", "zs"):
            j += 2
        else:
            j += 1
        n += 1
    return n


def monomorphemic(lemma, vocab, pos="NOUN"):
    s = STR.get((lemma, pos), "")
    if "@" in s or "|" in s:
        return False
    if O.syllables(lemma) >= 2 and DERIV.search(lemma):
        return False
    if _derived_s(lemma, vocab):
        return False
    if _splittable(lemma, vocab):
        return False
    return True


def build(pos, lex):
    vocab = set()
    for src in (MD["NOUN"], MD["ADJ"], LEXN, LEXA):
        vocab |= set(k for k in src if k[:1].islower() and len(k) >= 3)
    rows = []
    for lemma, flags in MD[pos].items():
        if not lemma or not lemma[0].islower() or not lemma.isalpha() or len(lemma) < 2:
            continue
        if lemma[-1] in O.VOWELS or flags & ALT:
            continue
        if lemma[-1] == "k" and len(lemma) > 3 and lemma[-2] in "aeoö" and lemma[:-2] in vocab:
            continue                          # a plural listed as a lemma
        if not monomorphemic(lemma, vocab, pos):
            continue
        h = L.harmony(lemma, back="back" in flags)
        prior = {"morphdb": "low" in flags, "ours": "Low" in lex.get(lemma, set())}
        wpos = "N" if pos == "NOUN" else "Adj"
        pl = acc = None
        if h != "F":
            pl = L.Vote(lemma, [L.plural(lemma, h, True)], [L.plural(lemma, h, False)],
                        dict(prior), wc_lemma=lemma, wc_pos=wpos)
            acc = L.Vote(lemma, [L.accusative(lemma, h, True)], [L.accusative(lemma, h, False)],
                         dict(prior), wc_lemma=lemma, wc_pos=wpos)
        elif lemma.endswith(BARE_T) and final_consonants(lemma) == 1:
            acc = L.Vote(lemma, [lemma + "et"], [lemma + "t"], dict(prior),
                         wc_lemma=lemma, wc_pos=wpos)
        else:
            continue                          # F stem with no surface contrast at all
        rows.append((lemma, h, pl, acc, flags))
    batch = [v for _, _, pl, acc, _ in rows for v in (pl, acc) if v]
    L.collect(batch)
    return rows


def merge(rows):
    out = []
    for lemma, h, pl, acc, flags in rows:
        # a cell with too little corpus mass cannot make a stem look variable
        def cell(v):
            if v is None or v.n_pos + v.n_neg < L.MIN_TOTAL:
                return None
            return v.decision
        d = {"pl": cell(pl), "acc": cell(acc)}
        got = [x for x in d.values() if x in ("member", "non-member")]
        if len(set(got)) == 1 and got:
            decision = "lowering" if got[0] == "member" else "non-lowering"
            conf = max((v.confidence for v in (pl, acc) if v), key=lambda c: "lmh".index(c[0]))
            conf = "high" if len(got) == 2 and conf == "high" else conf
            why = "plural+accusative agree" if len(got) == 2 else "one cell only"
        elif len(set(got)) == 2:
            decision, conf, why = "variable", "medium", f"plural {d['pl']}, accusative {d['acc']}"
        else:
            decision, conf, why = "review", "low", "no decisive cell"
        it = L.Vote(lemma, [], [], {})
        it.decision, it.confidence, it.why = decision, conf, why
        it.votes = dict((pl or acc).votes)
        it.n_pos = sum(v.n_pos for v in (pl, acc) if v)
        it.n_neg = sum(v.n_neg for v in (pl, acc) if v)
        it.h = h
        it._diag = " ".join(v.diag() for v in (pl, acc) if v)
        it.diag = lambda s=it: s._diag
        it.cells = f"pl={d['pl']},acc={d['acc']}"
        it.also_adj = "yes" if lemma in MD["ADJ"] else ""
        it.also_verb = "yes" if lemma in MD["VERB"] else ""
        out.append(it)
    return out


def main():
    which = sys.argv[1] if len(sys.argv) > 1 else "both"
    if which in ("both", "noun"):
        items = merge(build("NOUN", LEXN))
        keep = [it for it in items if it.n_pos + it.n_neg >= L.MIN_TOTAL or it.votes.get("morphdb")]
        L.write("lowering-nouns", keep, ["harmony", "cells", "why", "also_adjective", "also_verb"],
                lambda it: (it.h, it.cells, it.why, it.also_adj, it.also_verb))
        for tag in ("lowering", "variable"):
            m = sorted(it.lemma for it in keep if it.decision == tag and not it.also_adj)
            print(f"nouns {tag} (noun-only): {len(m)}")
            print(" ".join(m))
        print("nouns screened:", len(items))
    if which in ("both", "adj"):
        items = merge(build("ADJ", LEXA))
        keep = [it for it in items if it.n_pos + it.n_neg >= L.MIN_TOTAL or it.votes.get("morphdb")]
        L.write("nonlowering-adjectives", keep, ["harmony", "cells", "why", "also_noun", "also_verb"],
                lambda it: (it.h, it.cells, it.why, "yes" if it.lemma in MD["NOUN"] else "", it.also_verb))
        for tag in ("non-lowering", "variable"):
            m = sorted(it.lemma for it in keep if it.decision == tag)
            print(f"adjectives {tag}: {len(m)}")
            print(" ".join(m))
        print("adjectives screened:", len(items))


if __name__ == "__main__":
    main()
