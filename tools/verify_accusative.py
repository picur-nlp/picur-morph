"""Verifies the accusative of each candidate against tag- and lemma-filtered corpus counts."""
import sys
import wc2

pairs = []
for line in open(sys.argv[1], encoding="utf-8"):
    f = line.rstrip("\n").split("\t")
    if len(f) == 3 and all(f):
        pairs.append(tuple(f))

out = []
B = 4000
for i in range(0, len(pairs), B):
    chunk = pairs[i:i+B]
    forms = sorted({f for _, b, l in chunk for f in (b, l)})
    # one query for the chunk, then split by lemma using the row detail
    rows = wc2.rows(forms)
    byform = {}
    for form, lem, xpos, freq in rows:
        if "Acc" not in (xpos or ""):
            continue
        byform.setdefault((form, lem), 0)
        byform[(form, lem)] += freq
    for lemma, bare, linked in chunk:
        b = byform.get((bare, lemma), 0)
        l = byform.get((linked, lemma), 0)
        if b == 0 and l == 0:
            continue
        out.append((lemma, "bare" if b > l else "linked", b, l))
    print(f"  {min(i+B,len(pairs))}/{len(pairs)}", file=sys.stderr)

with open(sys.argv[2], "w", encoding="utf-8") as fh:
    fh.write("lemma\tdecision\tbare_n\tlinked_n\n")
    for lemma, d, b, l in out:
        fh.write(f"{lemma}\t{d}\t{b}\t{l}\n")
print(f"{len(out)} decided of {len(pairs)} candidates")
