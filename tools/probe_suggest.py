#!/usr/bin/env python3
"""Tie-break the review queue with the official spelling checker, in one small batch.

Each pair is (question, form predicted by membership, form predicted by non-membership).
The checker only says whether a form is legal, so a pair where both come back `helyes`
stays in the queue.
"""
import csv

import listgen as L
import suggest as S
import webcorpus as W

PAIRS = [
    ("antiharmonic/ín", "ínnak", "ínnek"),
    ("antiharmonic/ívik", "ívnak", "ívnek"),
    ("antiharmonic/szid", "szidnak", "szidnek"),
    ("antiharmonic/férfi", "férfinak", "férfinek"),
    ("antiharmonic/derék", "derékban", "derékben"),
    ("v-stem/bő", "bővek", "bők"),
    ("v-stem/odú", "odvak", "odúk"),
    ("v-stem/szú", "szuvak", "szúk"),
    ("v-stem/hó-month", "havak", "hók"),
    ("v-stem/jó", "javak", "jók"),
    ("v-stem/mag", "magvak", "magok"),
    ("v-stem/pléh", "plével", "pléhvel"),
    ("bare-acc/szakáll", "szakállt", "szakállat"),
    ("bare-acc/oldal", "oldalt", "oldalat"),
    ("bare-acc/hall-noun", "hallt", "hallat"),
    ("shortening/ész", "eszet", "észt"),
    ("shortening/ér", "eret", "ért"),
    ("shortening/fél", "felet", "félt"),
    ("shortening/gúnár", "gunarak", "gúnárok"),
    ("shortening/elevén", "elevenek", "elevének"),
    ("u-delete/borjú", "borjat", "borjút"),
    ("u-delete/ajtó", "ajtajai", "ajtói"),
    ("poss/néne", "nénje", "nénéje"),
    ("adj/hű", "hűbb", "hívebb"),
    ("ll/javall", "javallott", "javallt"),
    ("ll/sugall", "sugallott", "sugallt"),
    ("ll/hull", "hullanak", "hullnak"),
    ("ll/mállik", "mállott", "mállt"),
    ("ll/áll", "állasz", "állsz"),
    ("epenth-verb/hangzik", "hangzani", "hangozni"),
    ("epenth-verb/ugrik", "ugrani", "ugorni"),
    ("epenth-verb/fürdik", "fürdeni", "fürödni"),
    ("epenth-verb/terem", "termettem", "teremtem"),
    ("past/edz", "edzettem", "edztem"),
    ("past/cseng", "csengettem", "csengtem"),
    ("past/ing", "ingottam", "ingtam"),
    ("past/leng", "lengettem", "lengtem"),
    ("past/peng", "pengettem", "pengtem"),
    ("past/reng", "rengettem", "rengtem"),
    ("past/teng", "tengettem", "tengtem"),
    ("past/ráng", "rángottam", "rángtam"),
    ("past/fing", "fingottam", "fingtam"),
    ("past/hat", "hatottam", "hattam"),
    ("past/told", "toldottak", "toldtak"),
    ("past/áld", "áldottak", "áldtak"),
    ("short-i/kő", "kövi", "kői"),
    ("short-i/lé", "levi", "léi"),
    ("low/öröm", "örömek", "örömök"),
    ("low/kölcsön", "kölcsönek", "kölcsönök"),
    ("low/rokon", "rokonak", "rokonok"),
    ("low/viszony", "viszonyak", "viszonyok"),
    ("low/báj", "bájak", "bájok"),
    ("sz-d/alszik", "alusznak", "alszanak"),
    ("sz-d/fekszik", "feküsznek", "fekszenek"),
    ("sz-d/haragszik", "haragusznak", "haragszanak"),
    ("epenth/bajusz", "bajszot", "bajuszt"),
    ("epenth/lepel", "leplet", "lepelt"),
    ("epenth/sarok", "sarka", "sarokja"),
]


def main():
    out = []
    for label, a, b in PAIRS:
        va, vb = S.pair(a, b)
        c = W.counts([a, b])
        verdict = ("both legal" if va == vb == "helyes" else
                   f"{a} only" if va == "helyes" and vb != "helyes" else
                   f"{b} only" if vb == "helyes" and va != "helyes" else
                   "neither/error")
        out.append([label, a, va, c[a], b, vb, c[b], verdict])
        print(out[-1])
    with (L.OUT / "suggest-tiebreaks.tsv").open("w", encoding="utf-8", newline="") as fh:
        w = csv.writer(fh, delimiter="\t", lineterminator="\n")
        w.writerow(["question", "form_a", "mta_a", "corpus_a",
                    "form_b", "mta_b", "corpus_b", "verdict"])
        w.writerows(out)
    print("queries spent:", S.spent())


if __name__ == "__main__":
    main()
