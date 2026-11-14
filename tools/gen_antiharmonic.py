#!/usr/bin/env python3
"""PH-HARM-008 / VRB-STEM-013: the antiharmonic (back-suffixing all-neutral) root list.

Shape filter from the rule: all vowels neutral (i i: e e:), 1-2 syllables.
Diagnostic: dative `-nak` vs `-nek` for nominals, present 3pl `-nak/-anak` vs `-nek/-enek`
for verbs.
"""
import csv
from pathlib import Path

import listgen as L
import oracles as O
from paths import LOCAL

MD = O.morphdb_by_pos()
LEX = {"NOUN": O.lexicon("noun"), "ADJ": O.lexicon("adjective"), "VERB": O.lexicon("verb")}
SUF = {"NOUN": "noun", "ADJ": "adjective", "VERB": "verb"}


STR = O.morphdb_str()


def complex_stem(lemma, pos):
    """Compound or preverbed: excluded, the rule is about roots."""
    s = STR.get((lemma, pos), "")
    if "@" in s or "|" in s:
        return True
    if "preverb" in MD[pos].get(lemma, set()):
        return True
    parts = LEX[pos].get(lemma)
    return False


def candidates(pos):
    out = set(MD[pos]) | set(LEX[pos])
    keep = []
    for lemma in out:
        if not lemma or not lemma[0].islower() or not lemma.isalpha():
            continue
        if O.harmony_shape(lemma) != "N":
            continue
        if not 1 <= O.syllables(lemma) <= 2:
            continue
        if complex_stem(lemma, pos):
            continue
        keep.append(lemma)
    return sorted(keep)


def build(pos):
    items = []
    for lemma in candidates(pos):
        md = MD[pos].get(lemma, set())
        lex = LEX[pos].get(lemma, set())
        prior = {"morphdb": "back" in md, "ours": "Back" in lex}
        if pos == "VERB":
            stem = lemma[:-2] if lemma.endswith("ik") else lemma
            heavy = stem[-1] in "tsz" or (len(stem) > 1 and stem[-2] not in O.VOWELS)
            pos_f = [stem + ("anak" if heavy else "nak")]
            neg_f = [stem + ("enek" if heavy else "nek")]
            # accept either linking pattern for robustness
            pos_f.append(stem + "nak" if heavy else stem + "anak")
            neg_f.append(stem + "nek" if heavy else stem + "enek")
        else:
            # dative only: -nak/-nek never collides with an unrelated word, unlike -ak/-ok
            pos_f, neg_f = [lemma + "nak"], [lemma + "nek"]
        items.append(L.Vote(lemma, pos_f, neg_f, prior, wc_lemma=lemma))
    # foma/hunspell accept both members of many harmony pairs; keep them but they mostly abstain
    L.collect(items)
    return items


RACZ = LOCAL / "research-racz/Racz2027vh/dat/dzsungel.tsv"


def loan_stems():
    """Racz2027vh (MIT): loanwords whose last vowel is neutral, with back/front counts.

    All-neutral members belong to PH-HARM-008 (the `spejz` subclass); the rest are the
    vacillating stems of PH-HARM-013 and are written to their own list.
    """
    if not RACZ.exists():
        return []
    items = []
    with RACZ.open(encoding="utf-8") as fh:
        rd = csv.DictReader(fh, delimiter="\t")
        for r in rd:
            stem = r["stem"]
            v = O.vowels_of(stem)
            if not v or v[-1] not in O.NEUTRAL_V:
                continue
            back, front = int(r["back"]), int(r["front"])
            it = L.Vote(stem, [stem + "nak"], [stem + "nek"], {}, wc_lemma=stem)
            it.n_pos, it.n_neg = back, front
            total = back + front
            share = back / total if total else 0
            it.votes = {"racz2027vh": share >= 0.5, "morphdb": "back" in MD["NOUN"].get(stem, set())}
            it.decision = ("member" if share >= 0.9 else
                           "non-member" if share <= 0.1 else "variable")
            it.confidence = "high" if total >= 200 else "medium"
            it.why = f"Racz2027vh corpus {back}/{total} back"
            it.pos_tag = "LOAN-antiharmonic" if O.harmony_shape(stem) == "N" else "LOAN-vacillating"
            items.append(it)
    return items


def main():
    allitems = []
    for pos in ("NOUN", "ADJ", "VERB"):
        items = build(pos)
        for it in items:
            it.pos_tag = pos
        allitems += items
    # keep only rows with any positive signal or with corpus mass, to keep the file usable
    loans = loan_stems()
    L.write("neutral-final-loan-harmony", loans, ["class"], lambda it: (it.pos_tag,))
    print("loan stems (Racz2027vh, MIT):", len(loans),
          "back-dominant:", len([i for i in loans if i.decision == "member"]),
          "variable:", len([i for i in loans if i.decision == "variable"]))
    allitems += [i for i in loans if i.pos_tag == "LOAN-antiharmonic"]
    keep = [it for it in allitems
            if it.decision == "member" or it.votes.get("morphdb") or it.votes.get("ours")
            or it.decision == "variable"
            or (it.decision == "review" and it.n_pos + it.n_neg >= L.MIN_TOTAL and it.n_pos > it.n_neg * 0.2)]
    L.write("antiharmonic", keep, ["pos"], lambda it: (it.pos_tag,))
    mem = [it for it in keep if it.decision == "member"]
    print("members:", len(mem))
    print(" ".join(sorted(it.lemma for it in mem)))
    print("review:", " ".join(sorted(it.lemma for it in keep if it.decision == "review")))
    print("candidates screened:", len(allitems))


if __name__ == "__main__":
    main()
