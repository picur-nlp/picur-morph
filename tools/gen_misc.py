#!/usr/bin/env python3
"""Smaller closed lists: derivational suffix inventories (OQ-06-006), lexical `-sAg`
linking vowels (OQ-06-001), shortened `-i` adjectives (OQ-06-007), non-lengthening
suffixes (OQ-07-001), initial `ty` (OQ-02-013), plural-taking determiner-like adjectives
(OQ-09-011), the fused `anelkul hogy` class (OQ-08-003) and settlement-name local cases
(OQ-03-013)."""
import csv
import sys
from pathlib import Path

import listgen as L
import oracles as O
import webcorpus as W

OUT = L.OUT
MD = O.morphdb_by_pos()
DB = W.connect()


def rows(name, header, data):
    OUT.mkdir(parents=True, exist_ok=True)
    with (OUT / f"{name}.tsv").open("w", encoding="utf-8", newline="") as fh:
        w = csv.writer(fh, delimiter="\t", lineterminator="\n")
        w.writerow(header)
        w.writerows(data)
    print(f"{name}: {len(data)} rows")


def lemma_freq(lemma, pos=None):
    q, a = "select sum(freq) from f where lemma=?", [lemma]
    if pos:
        q += " and pos=?"
        a.append(pos)
    return int(DB.execute(q, a).fetchone()[0] or 0)


# --- OQ-06-006: closed derivational inventories -----------------------------

SUFFIXES = {
    "-mÁny": ("mány", "mény"),
    "-vÁny": ("vány", "vény"),
    "-(O)dA": ("oda", "öde", "da", "de"),
    "-nOk": ("nok", "nök"),
    "-(O)nc": ("onc", "enc", "önc", "nc"),
    "-AtAg": ("atag", "eteg"),
    "-lAg": ("lag", "leg"),
}


def gen_deriv_inventories():
    data = []
    for label, ends in SUFFIXES.items():
        pos = "ADJ" if label in ("-AtAg", "-lAg") else "NOUN"
        seen = {}
        for src in (MD["NOUN"], MD["ADJ"], MD["ADV"] if "ADV" in MD else {}):
            for lemma in src:
                if not lemma or not lemma[0].islower() or not lemma.isalpha():
                    continue
                if not lemma.endswith(ends) or len(lemma) < 6:
                    continue
                seen[lemma] = max(seen.get(lemma, 0), lemma_freq(lemma))
        keep = {k: v for k, v in seen.items() if v >= 100}
        for lemma, fr in sorted(keep.items(), key=lambda kv: -kv[1]):
            data.append([label, lemma, fr, "attested" if fr >= 100 else "rare"])
        print(f"  {label}: {len(keep)} lemmas with >=100 corpus tokens "
              f"({len(seen)} declared in morphdb)")
    rows("derivational-inventories", ["suffix", "lemma", "corpus_freq", "status"], data)


# --- OQ-06-001: lexical linking vowel before -sAg ---------------------------

def gen_sag():
    """`frissesseg` vs `*frisseg`: which single-`s` bases take the linking vowel."""
    data = []
    bases = [l for l in MD["ADJ"] if l and l[0].islower() and l.isalpha()
             and l.endswith("s") and len(l) > 3]
    for b in bases:
        h = L.harmony(b)
        sag = "ság" if h == "B" else "ség"
        link = b + ("as" if h == "B" else ("es" if h != "R" else "ös")) + sag
        bare = b + sag
        c = W.counts([link, bare])
        n1, n2 = c[link], c[bare]
        if n1 + n2 < 50:
            continue
        share = n1 / (n1 + n2)
        dec = "linked" if share >= 0.9 else ("bare" if share <= 0.1 else "both")
        data.append([b, dec, link, n1, bare, n2, round(share, 3)])
    data.sort(key=lambda r: -(r[3] + r[5]))
    rows("sag-linking-vowel", ["base", "decision", "linked_form", "n_linked",
                               "bare_form", "n_bare", "share_linked"], data)
    print("  linked:", " ".join(r[0] for r in data if r[1] == "linked"))
    print("  both:", " ".join(r[0] for r in data if r[1] == "both")[:400])


# --- OQ-06-007: lexically shortened -i adjectives ---------------------------

SHORT_I = [("hét", "heti", "héti"), ("hó", "havi", "hói"), ("tó", "tavi", "tói"),
           ("mű", "művi", "műi"), ("kéz", "kezi", "kézi"), ("víz", "vizi", "vízi"),
           ("út", "uti", "úti"), ("tűz", "tüzi", "tűzi"), ("nyár", "nyari", "nyári"),
           ("tél", "teli", "téli"), ("ég", "egi", "égi"), ("fű", "füvi", "fűi"),
           ("ló", "lovi", "lói"), ("kő", "kövi", "kői"), ("szó", "szavi", "szói"),
           ("név", "nevi", "névi"), ("lé", "levi", "léi")]


def gen_short_i():
    data = []
    for base, short, full in SHORT_I:
        c = W.counts([short, full])
        n1, n2 = c[short], c[full]
        dec = "shortened" if n1 > 5 * max(1, n2) else ("full" if n2 > 5 * max(1, n1) else "both")
        if n1 + n2 < 25:
            dec = "unattested"
        data.append([base, dec, short, n1, full, n2])
    rows("shortened-i-adjectives", ["base", "decision", "short_form", "n_short",
                                    "full_form", "n_full"], data)
    for r in data:
        print("  ", r)


# --- OQ-07-001: non-lengthening suffixes ------------------------------------

NONLENG = ["ként", "féle", "fajta", "szerű", "nemű", "beli", "kor", "szor", "izmus",
           "izál", "ikus", "lógia", "nyi", "stul", "ság", "kodik", "s", "z"]


def gen_nonlengthening():
    """`almakent` vs `almakent` with a lengthened stem: does the suffix lengthen final a/e?"""
    data = []
    for suf in NONLENG:
        tot_short = tot_long = 0
        ex = []
        for base in ("alma", "iskola", "kutya", "munka", "óra", "utca", "villa", "medve",
                     "epe", "fecske", "körte", "mese", "zene", "béka", "dada"):
            h = L.harmony(base)
            short = base + suf
            long = base[:-1] + ("á" if base[-1] == "a" else "é") + suf
            c = W.counts([short, long])
            if c[short] + c[long] < 20:
                continue
            tot_short += c[short]
            tot_long += c[long]
            ex.append(f"{short}={c[short]}/{long}={c[long]}")
        tot = tot_short + tot_long
        if tot < 20:
            data.append([suf, "unattested", 0, 0, ""])
            continue
        share = tot_short / tot
        dec = ("non-lengthening" if share >= 0.9 else
               "lengthening" if share <= 0.1 else "both")
        data.append([suf, dec, tot_short, tot_long, "; ".join(ex[:4])])
    rows("non-lengthening-suffixes", ["suffix", "decision", "n_short_stem",
                                      "n_lengthened_stem", "examples"], data)
    for r in data:
        print("  ", r[:4])


# --- OQ-02-013: word-initial ty ---------------------------------------------

def gen_initial_ty():
    seen = {}
    for r in DB.execute("select lemma, sum(freq) n from f where lemma like 'ty%' "
                        "group by lemma having n >= 100 order by n desc limit 200"):
        seen[r[0]] = int(r[1])
    data = [[l, n] for l, n in seen.items() if l[:2] == "ty"]
    rows("initial-ty-types", ["lemma", "corpus_freq"], data)
    print("  ", " ".join(f"{l}({n})" for l, n in data[:40]))


# --- OQ-09-011: plural-taking determiner-like adjectives --------------------

DETLIKE = ["bizonyos", "mindenféle", "egyéb", "további", "különféle", "számos",
           "különböző", "többféle", "sokféle", "némely", "némi", "egynémely"]


def gen_detlike():
    data = []
    for w in DETLIKE:
        h = L.harmony(w)
        pl = [w + "ak", w + "ok", w + "ek", w + "ök"]
        c = W.counts(pl + [w])
        n_pl = sum(c[f] for f in pl)
        data.append([w, "pluralises" if n_pl >= 100 else "no plural", c[w], n_pl,
                     ",".join(f"{f}={c[f]}" for f in pl if c[f])])
    rows("determiner-like-adjectives", ["word", "decision", "n_bare", "n_plural", "forms"], data)
    for r in data:
        print("  ", r[:4])


# --- OQ-08-003: fused `anelkul hogy` class ----------------------------------

FUSED = ["anélkül", "azonkívül", "amellett", "feltéve", "azzal", "attól", "ahelyett",
         "azután", "azelőtt", "afelett", "aziránt", "azonfelül", "azáltal", "azzal együtt"]


def gen_fused():
    data = []
    for w in FUSED:
        key = w.split()[0]
        n = lemma_freq(key)
        data.append([w, n, "candidate" if n >= 500 else "rare"])
    rows("fused-hogy-class", ["expression", "corpus_freq_of_head", "status"], data)
    print("  (corpus frequency of the head word only; the bigram `X hogy` needs a KWIC pass)")


if __name__ == "__main__":
    which = sys.argv[1] if len(sys.argv) > 1 else "all"
    todo = {"deriv": gen_deriv_inventories, "sag": gen_sag, "shorti": gen_short_i,
            "nonleng": gen_nonlengthening, "ty": gen_initial_ty, "det": gen_detlike,
            "fused": gen_fused}
    for k, fn in todo.items():
        if which in ("all", k):
            print(f"== {k}")
            fn()
