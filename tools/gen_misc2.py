#!/usr/bin/env python3
"""NOM-PROP-006 settlement local cases (OQ-03-013), DER-VERB-002 epenthetic `-(O)z`
(OQ-06-009), VRB-CAUS-001 causative selection (OQ-05-012) and WS-DIG-006 native
digraph junctions (OQ-01-009)."""
import csv
import re
import sys
from collections import Counter
from pathlib import Path

import listgen as L
import oracles as O
import webcorpus as W

MD = O.morphdb_by_pos()
GEO = O.MORPHDB / "proper_noun_geo.lexicon"


# --- OQ-03-013: settlement names, surface vs interior local case -------------

def gen_settlements():
    """Candidates are frequent capitalised noun lemmas; only place names take both
    the superessive and the inessive productively, so the diagnostic selects them."""
    # the frequency list is lowercased, so the settlement inventory comes from morphdb
    names = []
    for src in (GEO, O.MORPHDB / "proper_noun.lexicon"):
        txt = src.read_bytes().decode("iso-8859-2")
        for line in txt.splitlines():
            if not line or line.startswith("#"):
                continue
            w = line.split("\t")[0].strip()
            if w[:1].isupper() and w.isalpha() and len(w) > 3:
                names.append(w.lower())
    data = []
    for n in dict.fromkeys(names):
        h = L.harmony(n)
        sup = [n + "on", n + "en", n + "ön", n + "n"]
        ine = [n + "ban", n + "ben"]
        c = W.counts(sup + ine)
        s, i = sum(c[f] for f in sup), sum(c[f] for f in ine)
        if s + i < 200:
            continue
        share = s / (s + i)
        dec = "surface" if share >= 0.8 else ("interior" if share <= 0.2 else "both")
        data.append([n, dec, s, i, round(share, 3), n[-3:], O.syllables(n)])
    data.sort(key=lambda r: -(r[2] + r[3]))
    with (L.OUT / "settlement-local-cases.tsv").open("w", encoding="utf-8", newline="") as fh:
        w = csv.writer(fh, delimiter="\t", lineterminator="\n")
        w.writerow(["name", "decision", "n_superessive", "n_inessive", "share_surface",
                    "ending", "syllables"])
        w.writerows(data)
    cnt = Counter(r[1] for r in data)
    print("settlements:", len(data), dict(cnt))
    # ending-based tendency the rule asks for
    by_end = {}
    for r in data:
        by_end.setdefault(r[5], Counter())[r[1]] += 1
    strong = [(e, c) for e, c in by_end.items() if sum(c.values()) >= 8]
    strong.sort(key=lambda t: -sum(t[1].values()))
    for e, c in strong[:25]:
        tot = sum(c.values())
        print(f"  -{e}: n={tot} surface={c['surface']} interior={c['interior']} both={c['both']}")


# --- OQ-06-009: epenthetic noun + -(O)z ------------------------------------

def gen_epenth_z():
    rows = []
    with (L.OUT / "epenthetic-nouns.tsv").open(encoding="utf-8") as fh:
        rows = [r for r in csv.DictReader(fh, delimiter="\t") if r["decision"] == "member"]
    data = []
    for r in rows:
        lemma = r["lemma"]
        vu = r["unstable_vowel"]
        i = lemma.rfind(vu)
        if i < 1:
            continue
        syn = lemma[:i] + lemma[i + 1:]
        h = L.harmony(lemma)
        z = L.MID[h] + "z"
        drop, keep = syn + z, lemma + z
        c = W.counts([drop, keep])
        if c[drop] + c[keep] < 25:
            continue
        share = c[drop] / (c[drop] + c[keep])
        dec = "drops" if share >= 0.9 else ("keeps" if share <= 0.1 else "both")
        data.append([lemma, dec, drop, c[drop], keep, c[keep], round(share, 3),
                     r["kind"], O.syllables(lemma)])
    data.sort(key=lambda x: -(x[3] + x[5]))
    with (L.OUT / "epenthetic-z-derivation.tsv").open("w", encoding="utf-8", newline="") as fh:
        w = csv.writer(fh, delimiter="\t", lineterminator="\n")
        w.writerow(["lemma", "decision", "syncopated", "n_syncopated", "full", "n_full",
                    "share_syncopated", "kind", "syllables"])
        w.writerows(data)
    print("epenthetic -(O)z:", len(data), dict(Counter(r[1] for r in data)))
    print("  drops:", " ".join(r[0] for r in data if r[1] == "drops")[:600])
    print("  keeps:", " ".join(r[0] for r in data if r[1] == "keeps")[:600])


# --- OQ-05-012: causative -At vs -tAt --------------------------------------

def gen_causative():
    data = []
    for lemma, flags in MD["VERB"].items():
        if not lemma or not lemma[0].islower() or not lemma.isalpha():
            continue
        if O.syllables(lemma) != 1 or lemma[-1] in O.VOWELS:
            continue
        h = L.harmony(lemma)
        a, ta = lemma + ("at" if h == "B" else "et"), lemma + ("tat" if h == "B" else "tet")
        c = W.counts([a + "ni", ta + "ni", a + "ott", ta + "ott", a + "ta", ta + "ta"])
        n1 = c[a + "ni"] + c[a + "ott"] + c[a + "ta"]
        n2 = c[ta + "ni"] + c[ta + "ott"] + c[ta + "ta"]
        if n1 + n2 < 25:
            continue
        share = n1 / (n1 + n2)
        dec = "-At" if share >= 0.9 else ("-tAt" if share <= 0.1 else "both")
        data.append([lemma, dec, a, n1, ta, n2, round(share, 3), lemma[-1]])
    data.sort(key=lambda r: -(r[3] + r[5]))
    with (L.OUT / "causative-selection.tsv").open("w", encoding="utf-8", newline="") as fh:
        w = csv.writer(fh, delimiter="\t", lineterminator="\n")
        w.writerow(["verb", "decision", "At_stem", "n_At", "tAt_stem", "n_tAt",
                    "share_At", "final_C"])
        w.writerows(data)
    print("causative:", len(data), dict(Counter(r[1] for r in data)))
    print("  -At:", " ".join(r[0] for r in data if r[1] == "-At")[:500])
    print("  both:", " ".join(r[0] for r in data if r[1] == "both")[:300])


# --- OQ-01-009: native s+z, d+zs, z+zs, l+ly, t+ty junctions ---------------

JUNCTIONS = {"sz": ("s", "z"), "dzs": ("d", "zs"), "zzs": ("z", "zs"),
             "lly": ("l", "ly"), "tty": ("t", "ty"), "ccs": ("c", "cs"),
             "ggy": ("g", "gy"), "nny": ("n", "ny")}


def gen_junctions():
    """Which types containing a digraph string are analysed as two morphemes by foma."""
    db = W.connect()
    data = []
    for seq in JUNCTIONS:
        hits = [r for r in db.execute(
            "select form, sum(freq) n from f where form like ? and pos='N' "
            "group by form having n >= 2000 order by n desc limit 400", ("%" + seq + "%",))]
        forms = [h[0] for h in hits]
        ana = O.foma(forms)
        split = 0
        ex = []
        for f, n in hits:
            a = ana.get(f, set())
            if any("+" in x and x.count("+") >= 2 for x in a):
                split += 1
                if len(ex) < 8:
                    ex.append(f)
        data.append([seq, len(forms), split, "; ".join(ex)])
        print(f"  {seq}: {len(forms)} frequent noun types, {split} with a multi-morpheme analysis")
    with (L.OUT / "digraph-junctions.tsv").open("w", encoding="utf-8", newline="") as fh:
        w = csv.writer(fh, delimiter="\t", lineterminator="\n")
        w.writerow(["sequence", "n_types", "n_multi_morpheme", "examples"])
        w.writerows(data)


if __name__ == "__main__":
    which = sys.argv[1] if len(sys.argv) > 1 else "all"
    todo = {"settle": gen_settlements, "epz": gen_epenth_z,
            "caus": gen_causative, "junc": gen_junctions}
    for k, fn in todo.items():
        if which in ("all", k):
            print(f"== {k}")
            fn()
