#!/usr/bin/env python3
"""attest.py WORD... -> frequency bucket per word from frequency.ron ('-' = not in the table)."""
import re
import sys

from paths import LEXDATA

FREQ = dict(
    (m.group(1), int(m.group(2)))
    for m in re.finditer(r'\("([^"]+)",(\d+)\)', (LEXDATA / "frequency.ron").read_text(encoding="utf-8"))
)
for w in sys.argv[1:]:
    print(f"{w}\t{FREQ.get(w.lower(), '-')}")
