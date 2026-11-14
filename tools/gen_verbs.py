#!/usr/bin/env python3
"""VRB-PAST-007 (CC-final past), VRB-STEM-003 (t-final monosyllables),
VRB-STEM-006 (-ll and other geminate verbs), VRB-STEM-012 (sz-d(-v) verbs)."""
import re
import sys

import listgen as L
import oracles as O
import webcorpus as W

MD = O.morphdb_by_pos()
STR = O.morphdb_str()
LEXV = O.lexicon("verb")
VOW = O.VOWELS
DIG = ("dzs", "cs", "dz", "gy", "ly", "ny", "sz", "ty", "zs")


def graphemes(tail):
    out, i = [], 0
    while i < len(tail):
        if tail[i:i + 3] == "dzs":
            out.append("dzs"); i += 3
        elif tail[i:i + 2] in DIG:
            out.append(tail[i:i + 2]); i += 2
        else:
            out.append(tail[i]); i += 1
    return out


def coda(lemma):
    idx = [i for i, c in enumerate(lemma) if c in VOW]
    return graphemes(lemma[idx[-1] + 1:]) if idx else []


def stem_of(lemma):
    return lemma[:-2] if lemma.endswith("ik") and len(lemma) > 3 else lemma


def three_way(it):
    """Relabel a Vote as vowel / none / both from the corpus share."""
    total = it.n_pos + it.n_neg
    if total < L.MIN_TOTAL:
        return "review"
    share = it.n_pos / total
    return "vowel" if share >= 0.9 else ("none" if share <= 0.1 else "both")


# --- CC-final past ----------------------------------------------------------

def gen_cc_past():
    items = []
    for lemma, flags in MD["VERB"].items():
        if not lemma or not lemma[0].islower() or not lemma.isalpha():
            continue
        if "preverb" in flags:
            continue
        stem = stem_of(lemma)
        c = coda(stem)
        if len(c) != 2 or c[0] == c[1]:
            continue                             # geminates are VRB-STEM-006
        h = L.harmony(stem, back="back" in flags)
        pos_f = [L.past1sg(stem, h, True), stem + L.MID[h] + "tt" + ("ak" if h == "B" else "ek")]
        neg_f = [L.past1sg(stem, h, False), stem + "t" + ("ak" if h == "B" else "ek")]
        prior = {"morphdb": "Vtt_nonfinal_past" in flags
                 if {"Vtt_nonfinal_past", "t_nonfinal_past"} & flags else None,
                 "ours": "PastNonfinalVtt" in LEXV.get(lemma, set())
                 if {"PastNonfinalVtt", "PastNonfinalT"} & LEXV.get(lemma, set()) else None}
        it = L.Vote(lemma, pos_f, neg_f, {k: v for k, v in prior.items() if v is not None},
                    wc_lemma=lemma, wc_pos="V")
        it.coda = "".join(c)
        items.append(it)
    L.collect(items, use_unimorph=False, use_kaikki=False)
    for it in items:
        it.decision = three_way(it)
        it.confidence = "high" if it.n_pos + it.n_neg >= 200 else (
            "medium" if it.decision != "review" else "low")
    keep = [it for it in items if it.n_pos + it.n_neg >= L.MIN_TOTAL]
    L.write("cc-past-verbs", keep, ["coda"], lambda it: (it.coda,))
    for tag in ("vowel", "none", "both"):
        m = [it for it in keep if it.decision == tag]
        print(f"CC past {tag}: {len(m)}")
        if tag != "vowel":
            print("  ", " ".join(sorted(it.lemma for it in m)))


# --- t-final monosyllables --------------------------------------------------

def gen_t_final():
    items = []
    for lemma, flags in MD["VERB"].items():
        if not lemma or not lemma[0].islower() or not lemma.isalpha():
            continue
        stem = stem_of(lemma)
        if not stem.endswith("t") or O.syllables(stem) != 1:
            continue
        if len(coda(stem)) != 1:
            continue
        h = L.harmony(stem, back="back" in flags)
        pos_f = [L.past1sg(stem, h, True)]                    # kototem type
        neg_f = [stem + "t" + ("am" if h == "B" else "em")]   # lattam type
        prior = {"morphdb": "t_final_past" not in flags if
                 {"t_final_past", "Vtt_final_past"} & flags else None}
        it = L.Vote(lemma, pos_f, neg_f, {k: v for k, v in prior.items() if v is not None},
                    wc_lemma=lemma, wc_pos="V")
        it.vlen = "long" if O.vowels_of(stem)[-1] in "áéíóúőű" else "short"
        items.append(it)
    L.collect(items, use_unimorph=False, use_kaikki=False)
    for it in items:
        it.decision = {"vowel": "T-Ott", "none": "T-t", "both": "both"}.get(three_way(it), "review")
    keep = [it for it in items if it.n_pos + it.n_neg >= L.MIN_TOTAL]
    L.write("t-final-monosyllabic-verbs", keep, ["vowel_length"], lambda it: (it.vlen,))
    for tag in ("T-Ott", "T-t", "both", "review"):
        m = sorted(it.lemma for it in keep if it.decision == tag)
        print(f"t-final {tag}: {len(m)}  {' '.join(m)}")


# --- geminate (-ll etc.) verbs ---------------------------------------------

def gen_geminate():
    items = []
    for lemma, flags in MD["VERB"].items():
        if not lemma or not lemma[0].islower() or not lemma.isalpha():
            continue
        if "preverb" in flags:
            continue
        stem = stem_of(lemma)
        c = coda(stem)
        if len(c) != 2 or c[0] != c[1]:
            continue
        h = L.harmony(stem, back="back" in flags)
        # linking vowel in the infinitive: hallani vs allni
        inf_v, inf_0 = L.infinitive(stem, h, True), L.infinitive(stem, h, False)
        past_v, past_0 = stem + L.MID[h] + "tt", stem + "t"
        it = L.Vote(lemma, [inf_v], [inf_0], {}, wc_lemma=lemma, wc_pos="V")
        it.past = L.Vote(lemma, [past_v], [past_0], {}, wc_lemma=lemma, wc_pos="V")
        it.gem = c[0]
        it.syl = O.syllables(stem)
        items.append(it)
    L.collect(items, use_unimorph=False, use_kaikki=False)
    L.collect([it.past for it in items], use_foma=False, use_hunspell=False,
              use_unimorph=False, use_kaikki=False)
    for it in items:
        it.link = three_way(it)
        it.past_cls = three_way(it.past)
        it.decision = f"link={it.link}"
        it.confidence = "high" if it.n_pos + it.n_neg >= 200 else "medium"
        it._d = f"{it.diag()} | {it.past.diag()}"
        it.diag = lambda s=it: s._d
    keep = [it for it in items if it.n_pos + it.n_neg >= L.MIN_TOTAL
            or it.past.n_pos + it.past.n_neg >= L.MIN_TOTAL]
    L.write("geminate-verbs", keep, ["geminate", "syllables", "linking_vowel", "past"],
            lambda it: (it.gem, it.syl, it.link, it.past_cls))
    for g in sorted(set(it.gem for it in keep)):
        rows = [it for it in keep if it.gem == g]
        print(f"-{g}{g}: {len(rows)}",
              " ".join(f"{it.lemma}({it.link}/{it.past_cls})" for it in sorted(rows, key=lambda x: x.lemma))[:1200])


# --- sz-d(-v) verbs ---------------------------------------------------------

# VRB-STEM-012 names these; morphdb's peeta.lexicon and our verb.ron both lack several
# of them (OQ-05-013), so they are seeded explicitly and then verified like the rest.
SEED = ("alszik", "fekszik", "nyugszik", "haragszik", "esküszik", "alkuszik", "cselekszik",
        "dicsekszik", "igyekszik", "emlékszik", "gyülekszik", "növekszik", "gyarapszik",
        "gazdagszik", "melegszik", "öregszik", "betegszik", "telepszik", "ülepszik",
        "mosakszik", "verekszik", "veszekszik", "vetekszik", "gyanakszik", "furakszik",
        "tolakszik", "dulakszik", "marakszik", "tanakszik", "elégszik", "menekszik",
        "törekszik", "megismerszik", "hallszik")


def gen_szd():
    items = []
    cand = {l: f for l, f in MD["VERB"].items()
            if l and l[0].islower() and l.isalpha() and stem_of(l).endswith("sz")}
    for l in SEED:
        cand.setdefault(l, set())
    for l in LEXV:
        if l and l[0].islower() and l.isalpha() and stem_of(l).endswith("sz"):
            cand.setdefault(l, MD["VERB"].get(l, set()))
    for lemma, flags in cand.items():
        stem = stem_of(lemma)
        base = stem[:-2]
        if not base or O.syllables(base) < 1:
            continue
        h = L.harmony(stem, back="back" in flags)
        # the d/z stem vowel is o/e/o"/u/u" and is not predictable from harmony alone
        dv = "aou" if h == "B" else ("eou" if h == "F" else "eou")
        dstems = [base + v + c for v in ("o", "e", "ö", "u", "ü") for c in ("d", "z")]
        pos_f, neg_f = [], []
        for d in dict.fromkeys(dstems):
            hd = L.harmony(d, back=(h == "B"))
            pos_f += [d + "t", d + "t" + ("em" if hd != "B" else "am"),
                      d + L.MID[hd] + "tt", d + "ni"]
        neg_f = [stem + "ni", stem + "t" + ("am" if h == "B" else "em")]
        prior = {"morphdb": bool({"sz-d_stem", "sz-d-v_stem"} & flags),
                 "ours": bool({"SzDStem", "SzDVStem"} & LEXV.get(lemma, set()))}
        it = L.Vote(lemma, pos_f, neg_f, prior, wc_lemma=lemma, wc_pos="V")
        it.vstem = "yes" if "sz-d-v_stem" in flags else ""
        # v-stem probe: alvo / fekvo type participle
        vs = base + ("v" if base and base[-1] not in VOW else "v")
        it.vprobe = L.Vote(lemma, [vs + ("ó" if h == "B" else "ő")], [stem + ("ó" if h == "B" else "ő")],
                           {}, wc_lemma=None)
        items.append(it)
    L.collect(items, use_unimorph=False, use_kaikki=False)
    L.collect([it.vprobe for it in items], use_foma=False, use_hunspell=False,
              use_unimorph=False, use_kaikki=False)
    keep = [it for it in items if it.n_pos + it.n_neg >= L.MIN_TOTAL or it.votes.get("morphdb")]
    for it in keep:
        it.vcorpus = "yes" if it.vprobe.n_pos >= 25 and it.vprobe.n_pos > it.vprobe.n_neg else ""
    L.write("sz-d-verbs", keep, ["morphdb_v_stem", "v_stem_attested"],
            lambda it: (it.vstem, it.vcorpus))
    m = [it for it in keep if it.decision == "member"]
    print("sz-d members:", len(m))
    print(" ".join(sorted(it.lemma + ("[v]" if it.vcorpus else "") for it in m)))
    print("review:", " ".join(sorted(it.lemma for it in keep if it.decision == "review")))


if __name__ == "__main__":
    which = sys.argv[1] if len(sys.argv) > 1 else "all"
    if which in ("all", "cc"):
        gen_cc_past()
    if which in ("all", "t"):
        gen_t_final()
    if which in ("all", "gem"):
        gen_geminate()
    if which in ("all", "szd"):
        gen_szd()
