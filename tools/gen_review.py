#!/usr/bin/env python3
"""Cross-list agreement statistics and the ranked review queue."""
import csv
import json
import sys
from collections import Counter
from pathlib import Path

import listgen as L
import webcorpus as W

LISTS = L.OUT

# rules each list feeds, used to rank the review queue by blast radius
DEPS = {
    "antiharmonic": ["PH-HARM-008", "PH-HARM-009", "PH-HARM-010", "PH-HARM-013",
                     "VRB-STEM-013", "PH-ZERO-001"],
    "lowering-nouns": ["NOM-LOW-007", "NOM-LOW-008", "NOM-POSS-006", "NOM-POSS-007",
                       "NOM-ACC-005", "NOM-LINK-002"],
    "nonlowering-adjectives": ["NOM-LOW-004", "NOM-ADJ-001", "NOM-ADJ-010", "NOM-ACC-005"],
    "epenthetic-nouns": ["PH-ZERO-001", "PH-ZERO-002", "NOM-STEM-005", "NOM-STEM-006",
                         "NOM-ACC-008", "NOM-POSS-007"],
    "epenthetic-verbs": ["PH-ZERO-003", "VRB-STEM-007", "VRB-STEM-008", "VRB-PAST-009"],
    "shortening-stems": ["NOM-STEM-007", "NOM-STEM-008", "NOM-LOW-005", "PH-LEN-007",
                         "NOM-POSS-010"],
    "v-stems": ["PH-VSTEM-001", "PH-VSTEM-002", "PH-VSTEM-004", "PH-VSTEM-005",
                "NOM-STEM-010", "VRB-STEM-010"],
    "sz-d-verbs": ["VRB-STEM-012", "VRB-IRR-011", "VRB-PAST-010"],
    "geminate-verbs": ["VRB-STEM-006", "VRB-PAST-004", "VRB-LINK-002"],
    "t-final-monosyllabic-verbs": ["VRB-STEM-003", "VRB-PAST-006", "VRB-IMP-004"],
    "cc-past-verbs": ["VRB-PAST-007", "VRB-PAST-011", "VRB-PART-002"],
    "bare-accusative": ["NOM-ACC-007", "NOM-ACC-009", "NOM-ACC-003"],
    "poss3-ja-exceptions": ["NOM-POSS-006", "NOM-POSS-007", "NOM-POSS-008"],
    "neutral-final-loan-harmony": ["PH-HARM-013", "PH-HARM-019", "PH-HARM-022"],
}

UNSETTLED = {"review", "variable", "both", "link=review", "link=both"}


def read(name):
    path = LISTS / f"{name}.tsv"
    if not path.exists():
        return []
    with path.open(encoding="utf-8") as fh:
        return list(csv.DictReader(fh, delimiter="\t"))


def agreement():
    """Pairwise agreement between each declared oracle and the corpus decision."""
    stats = {}
    for name in DEPS:
        pair = Counter()
        for r in read(name):
            ev = r.get("evidence", "")
            if not ev or r["decision"] in UNSETTLED:
                continue
            votes = dict((p.split("=")[0], p.split("=")[1] == "+") for p in ev.split(";") if "=" in p)
            if "corpus" not in votes:
                continue
            for k, v in votes.items():
                if k == "corpus":
                    continue
                pair[(k, "agree" if v == votes["corpus"] else "disagree")] += 1
        oracles = sorted(set(k for k, _ in pair))
        stats[name] = {o: (pair[(o, "agree")], pair[(o, "agree")] + pair[(o, "disagree")])
                       for o in oracles}
    return stats


def queue():
    out = []
    for name, rules in DEPS.items():
        for r in read(name):
            if r["decision"] not in UNSETTLED:
                continue
            if int(r.get("n_pos") or 0) + int(r.get("n_neg") or 0) < 10:
                continue          # no diagnostic form is attested: nothing to review yet
            lemma = r["lemma"]
            lf = W.lemma_freq(lemma)
            out.append({
                "list": name, "lemma": lemma, "decision": r["decision"],
                "rules": len(rules), "rule_ids": ",".join(rules),
                "lemma_freq": lf,
                "diagnostics": r.get("diagnostics", ""),
                "n_pos": int(r.get("n_pos") or 0), "n_neg": int(r.get("n_neg") or 0),
                "score": len(rules) * (1 + (lf ** 0.5) / 100),
            })
    out.sort(key=lambda d: -d["score"])
    for i, d in enumerate(out, 1):
        d["rank"] = i
    cols = ["rank", "list", "lemma", "decision", "rules", "rule_ids", "lemma_freq",
            "n_pos", "n_neg", "diagnostics", "score"]
    with (LISTS / "review-queue.tsv").open("w", encoding="utf-8", newline="") as fh:
        w = csv.DictWriter(fh, cols, delimiter="\t", lineterminator="\n", extrasaction="ignore")
        w.writeheader()
        for d in out:
            d["score"] = round(d["score"], 2)
            w.writerow(d)
    print("review queue:", len(out))
    for d in out[:50]:
        print(f"  {d['rank']:3d} {d['list']:28s} {d['lemma']:20s} {d['decision']:10s} "
              f"freq={d['lemma_freq']:>9d} {d['diagnostics'][:70]}")
    return out


if __name__ == "__main__":
    st = agreement()
    print("== oracle agreement with the corpus decision (agree/total)")
    for name, d in st.items():
        if d:
            print(" ", name, {k: f"{a}/{n} ({100*a/n:.0f}%)" for k, (a, n) in d.items() if n})
    (LISTS / "agreement.json").write_text(
        json.dumps({k: {o: list(v) for o, v in d.items()} for k, d in st.items()},
                   indent=1, ensure_ascii=False), encoding="utf-8")
    queue()
