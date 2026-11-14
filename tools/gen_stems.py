#!/usr/bin/env python3
"""NOM-STEM-008 (shortening stems), PH-VSTEM-001..005 (v-stems), NOM-ACC-007/009
(bare accusative) and NOM-POSS-006/007 (-a/-ja exceptions)."""
import re
import sys

import listgen as L
import oracles as O
import webcorpus as W

MD = O.morphdb_by_pos()
STR = O.morphdb_str()
LEXN = O.lexicon("noun")
LEXA = O.lexicon("adjective")
LEXV = O.lexicon("verb")

SHORTENS = {"á": "a", "é": "e", "í": "i", "ú": "u", "ű": "ü"}
LONG = "áéíóúőű"


def kind(lemma, pos):
    st = STR.get((lemma, pos), "")
    return "compound" if "@" in st else ("derived" if "|" in st else "root")


# --- shortening stems -------------------------------------------------------

def shorten(lemma):
    """Shorten the last stem vowel; None if the shape conditions of NOM-STEM-007 fail."""
    idx = [i for i, c in enumerate(lemma) if c in O.VOWELS]
    if not idx:
        return None
    last = idx[-1]
    if lemma[last] not in SHORTENS:
        return None
    if any(lemma[i] in LONG for i in idx[:-1]):
        return None                              # only one long vowel allowed
    tail = lemma[last + 1:]
    if not tail or tail in ("dzs",):
        return None
    ncons = len(re.findall(r"dzs|cs|dz|gy|ly|ny|sz|ty|zs|[^aáeéiíoóöőuúüű]", tail))
    if ncons != 1:
        return None
    return lemma[:last] + SHORTENS[lemma[last]] + tail


def gen_shortening():
    items = []
    for pos, lex in (("NOUN", LEXN), ("ADJ", LEXA)):
        for lemma, flags in MD[pos].items():
            if not lemma or not lemma[0].islower() or not lemma.isalpha():
                continue
            sh = shorten(lemma)
            if not sh:
                continue
            h = L.harmony(lemma, back="back" in flags)
            pos_f = [L.plural(sh, h, True), L.accusative(sh, h, True)]
            neg_f = [L.plural(lemma, h, True), L.plural(lemma, h, False),
                     L.accusative(lemma, h, True), L.accusative(lemma, h, False)]
            prior = {"morphdb": "shortening" in flags,
                     "ours": "Shortening" in lex.get(lemma, set())}
            it = L.Vote(lemma, pos_f, neg_f, prior, wc_lemma=lemma,
                        wc_pos="N" if pos == "NOUN" else "Adj")
            it.pos_tag = pos
            it.kind = kind(lemma, pos)
            it.alt = sh
            it.syl = O.syllables(lemma)
            items.append(it)
    L.collect(items)
    keep = [it for it in items if it.n_pos + it.n_neg >= L.MIN_TOTAL or it.votes.get("morphdb")]
    L.write("shortening-stems", keep, ["pos", "kind", "alternant", "syllables"],
            lambda it: (it.pos_tag, it.kind, it.alt, it.syl))
    m = [it for it in keep if it.decision == "member"]
    print("shortening members:", len(m), "of", len(items), "shape candidates")
    print("mono:", " ".join(sorted(it.lemma for it in m if it.syl == 1)))
    print("poly root:", " ".join(sorted(it.lemma for it in m if it.syl > 1 and it.kind == "root")))
    print("poly complex:", len([it for it in m if it.syl > 1 and it.kind != "root"]))


# --- v-stems ----------------------------------------------------------------

VSHORT = {"ó": ("o", "a"), "ő": ("ö",), "é": ("e",), "ú": ("u",), "ű": ("ü",)}


def gen_vstem():
    items = []
    for pos, lex, trait in (("NOUN", LEXN, "VStem"), ("ADJ", LEXA, "VStem"), ("VERB", LEXV, "VStem")):
        for lemma, flags in MD[pos].items():
            if not lemma or not lemma[0].islower() or not lemma.isalpha():
                continue
            if lemma[-1] not in VSHORT or O.syllables(lemma) != 1:
                continue
            h = L.harmony(lemma, back="back" in flags)
            base = lemma[:-1]
            # both the shortening (PH-VSTEM-002) and the non-shortening (PH-VSTEM-001) alternant
            vstems = [base + short + "v" for short in VSHORT[lemma[-1]]] + [lemma + "v"]
            pos_f = []
            for v in vstems:
                hv = L.harmony(v, back=(h == "B"))
                if pos == "VERB":
                    pos_f += [L.deverbal_as(v, hv), v + L.MID[hv] + "k"]
                else:
                    pos_f += [L.plural(v, hv, True), L.accusative(v, hv, True)]
            pos_f = [f for f in dict.fromkeys(pos_f) if f != lemma]
            neg_f = ([L.deverbal_as(lemma, h), lemma + L.MID[h] + "k"] if pos == "VERB"
                     else [lemma + "k", lemma + "t"])
            prior = {"morphdb": "v_stem" in flags, "ours": trait in lex.get(lemma, set())}
            it = L.Vote(lemma, pos_f, neg_f, prior, wc_lemma=lemma,
                        wc_pos={"NOUN": "N", "ADJ": "Adj", "VERB": "V"}[pos])
            it.pos_tag = pos
            items.append(it)
    # disyllabic u/u:/u"-final nouns (PH-VSTEM-004) and u:-deleting stems (PH-VSTEM-005)
    for lemma, flags in MD["NOUN"].items():
        if not lemma or not lemma[0].islower() or not lemma.isalpha():
            continue
        if O.syllables(lemma) != 2 or lemma[-1] not in "uúű":
            continue
        h = L.harmony(lemma, back="back" in flags)
        base = lemma[:-1]
        short = {"u": "u", "ú": "u", "ű": "ü"}[lemma[-1]]
        pos_f = [L.plural(base + "v", h, True), L.plural(base, h, True)]
        neg_f = [lemma + "k"]
        prior = {"morphdb": "v_stem" in flags or "u_stem" in flags,
                 "ours": bool({"VStem", "UStem"} & LEXN.get(lemma, set()))}
        it = L.Vote(lemma, pos_f, neg_f, prior, wc_lemma=lemma, wc_pos="N")
        it.pos_tag = "NOUN-disyll"
        items.append(it)
    L.collect(items)
    keep = [it for it in items if it.n_pos + it.n_neg >= L.MIN_TOTAL or it.votes.get("morphdb")]
    L.write("v-stems", keep, ["pos"], lambda it: (it.pos_tag,))
    for tag in ("member", "review"):
        m = sorted(f"{it.lemma}/{it.pos_tag}" for it in keep if it.decision == tag)
        print(f"v-stems {tag}: {len(m)}  {' '.join(m)}")


# --- bare accusative --------------------------------------------------------

SONORANT_S = ("js", "ls", "ns", "rs", "jsz", "lsz", "nsz", "rsz", "jz", "lz", "nz", "rz",
              "jzs", "lzs", "nzs", "rzs", "jn", "ln", "rn", "jl", "rl", "jj", "lj", "nj", "rj")
GEMINATES = ("bb", "cc", "dd", "ff", "gg", "jj", "kk", "ll", "mm", "nn", "pp", "rr", "ss",
             "tt", "vv", "zz", "ccs", "ggy", "lly", "nny", "ssz", "tty", "zzs")


def gen_bare_acc():
    """NOM-ACC-007 (lowering stems with bare -t) and NOM-ACC-009 (cluster stems)."""
    items = []
    for pos, lex in (("NOUN", LEXN), ("ADJ", LEXA)):
        for lemma, flags in MD[pos].items():
            if not lemma or not lemma[0].islower() or not lemma.isalpha() or len(lemma) < 3:
                continue
            if lemma[-1] in O.VOWELS:
                continue
            cluster = lemma.endswith(SONORANT_S) or lemma.endswith(GEMINATES)
            low = "low" in flags
            if not (cluster or low):
                continue
            h = L.harmony(lemma, back="back" in flags)
            pos_f = [lemma + "t"]
            neg_f = [L.accusative(lemma, h, low), L.accusative(lemma, h, not low)]
            prior = {"morphdb": "bareT" in flags if ("bareT" in flags or "non_bareT" in flags) else None,
                     "ours": "BareAccusative" in lex.get(lemma, set())}
            prior = {k: v for k, v in prior.items() if v is not None}
            # no POS restriction here: taggers split `okost` (N) from `okosat` (Adj)
            it = L.Vote(lemma, pos_f, neg_f, prior, wc_lemma=lemma)
            it.pos_tag = pos
            it.shape = "cluster" if cluster else "single-C"
            # measure lowering on the plural instead of trusting the morphdb flag
            it.low_probe = L.Vote(lemma, [L.plural(lemma, h, True)], [L.plural(lemma, h, False)],
                                  {}, wc_lemma=lemma) if h != "F" else None
            items.append(it)
    L.collect(items)
    L.collect([it.low_probe for it in items if it.low_probe], use_foma=False, use_hunspell=False,
              use_unimorph=False, use_kaikki=False)
    for it in items:
        lw = it.low_probe.decision if it.low_probe else "unknown"
        it.cls = ("lowering+" if lw == "member" else "") + it.shape
    keep = [it for it in items if it.n_pos + it.n_neg >= L.MIN_TOTAL]
    L.write("bare-accusative", keep, ["pos", "class"], lambda it: (it.pos_tag, it.cls))
    m = [it for it in keep if it.decision == "member"]
    print("bare accusative members:", len(m), "of", len(items), "candidates")
    print("lowering stems with bare -t:",
          " ".join(sorted(it.lemma for it in m if it.cls.startswith("lowering"))))


# --- 3sg possessive -a/-ja --------------------------------------------------

TAB21 = {"intercept": 3.53}


def poss_model_z(lemma, h, low):
    """Tab21 Table 17 logistic model for p(-ja) on a consonant-final noun."""
    z = 3.53
    if low:
        z -= 3.71
    last = lemma[-1]
    pair = lemma[-2:]
    if pair in ("cs", "sz", "zs", "dz"):
        last = pair
    if last in ("s", "sz", "zs", "z", "c", "cs", "dz"):
        z -= 11.07
    elif last in ("f", "v", "h"):
        z -= 1.03
    elif last in ("m", "n", "ny"):
        z -= 2.07
    elif last in ("l", "r", "j", "ly"):
        z -= 4.06
    if last in ("p", "b", "m", "f", "v"):
        z -= 2.22
    elif last in ("ny", "ty", "gy", "j", "ly"):
        z -= 9.25
    elif last in ("k", "g"):
        z -= 3.54
    if h != "B":
        z -= 1.98
    vs = O.vowels_of(lemma)
    if vs:
        v = vs[-1]
        if v in "iíuúüű":
            z += 1.85
        elif v in "aáe":
            z += 0.77
        if v in LONG:
            z += 1.30
    if lemma.endswith(GEMINATES):
        z += 2.43
    if O.syllables(lemma) > 1:
        z += 0.79
    return z


DERIV_N = re.compile(
    r"(ság|ség|ás|és|zat|zet|alom|elem|mány|mény|vány|vény|ász|ész|nok|nök"
    r"|talan|telen|atlan|etlen|tlan|tlen|ott|ett|ött|us|izmus|ista|ció|ív)$")


def gen_poss():
    items = []
    for lemma, flags in MD["NOUN"].items():
        if not lemma or not lemma[0].islower() or not lemma.isalpha() or len(lemma) < 3:
            continue
        if lemma[-1] in O.VOWELS:
            continue
        if kind(lemma, "NOUN") != "root":
            continue                              # NOM-POSS-007 decides complex stems
        if O.syllables(lemma) >= 2 and DERIV_N.search(lemma):
            continue
        h = L.harmony(lemma, back="back" in flags)
        low = "low" in flags
        z = poss_model_z(lemma, h, low)
        pred_ja = z >= 0
        ja, a = L.poss3(lemma, h, True), L.poss3(lemma, h, False)
        prior = {"morphdb": "poss_jA" in flags if ("poss_jA" in flags or "poss_A" in flags) else None,
                 "ours": "PossessiveJa" in LEXN.get(lemma, set())
                 if {"PossessiveJa", "PossessiveA"} & LEXN.get(lemma, set()) else None}
        prior = {k: v for k, v in prior.items() if v is not None}
        it = L.Vote(lemma, [ja], [a], prior, wc_lemma=lemma, wc_pos="N")
        it.pred = "ja" if pred_ja else "a"
        it.z = round(z, 2)
        items.append(it)
    L.collect(items, use_foma=False, use_hunspell=False)
    keep = [it for it in items if it.n_pos + it.n_neg >= 100]
    for it in keep:
        actual = "ja" if it.decision == "member" else ("a" if it.decision == "non-member" else "var")
        it.actual = actual
        it.agrees = "yes" if actual == it.pred else ("-" if actual == "var" else "no")
    L.write("poss3-ja-exceptions", keep, ["model_pred", "z", "actual", "model_agrees"],
            lambda it: (it.pred, it.z, it.actual, it.agrees))
    n = [it for it in keep if it.agrees in ("yes", "no")]
    bad = [it for it in n if it.agrees == "no"]
    print(f"poss3: {len(keep)} stems counted, model accuracy "
          f"{100 * (len(n) - len(bad)) / max(1, len(n)):.1f}% ({len(bad)} exceptions)")
    print("variable:", len([it for it in keep if it.actual == "var"]))


if __name__ == "__main__":
    which = sys.argv[1] if len(sys.argv) > 1 else "all"
    if which in ("all", "shortening"):
        gen_shortening()
    if which in ("all", "vstem"):
        gen_vstem()
    if which in ("all", "acc"):
        gen_bare_acc()
    if which in ("all", "poss"):
        gen_poss()
