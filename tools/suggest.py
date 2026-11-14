#!/usr/bin/env python3
"""helyesiras.mta.hu suggest oracle: a legality filter, not a preference ranker.

Every response is cached under local/cache/suggest/ with a timestamp. The service is
rate-limited to one request per second and fails with HTTP 500 roughly a third of the
time, so each query is retried. Use it only to break ties the corpus cannot.
"""
from __future__ import annotations

import hashlib
import json
import sys
import time
import urllib.parse
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

from paths import CACHE as CACHE_ROOT

CACHE = CACHE_ROOT / "suggest"
URL = "https://helyesiras.mta.hu/helyesiras/default/suggest?q="
UA = "picur-morph research probe (contact: repo owner)"
BUDGET = CACHE / ".budget"
MAX_QUERIES = 400


def _path(form: str) -> Path:
    h = hashlib.sha1(form.encode("utf-8")).hexdigest()[:16]
    return CACHE / f"{h}.json"


def spent() -> int:
    return int(BUDGET.read_text()) if BUDGET.exists() else 0


def query(form: str, retries: int = 4) -> dict | None:
    """{'form','verdict','raw','ts'}; verdict is helyes / ismeretlen / unknown."""
    CACHE.mkdir(parents=True, exist_ok=True)
    p = _path(form)
    if p.exists():
        return json.loads(p.read_text(encoding="utf-8"))
    if spent() >= MAX_QUERIES:
        raise RuntimeError(f"suggest budget of {MAX_QUERIES} queries exhausted")
    req = urllib.request.Request(URL + urllib.parse.quote(form), headers={"User-Agent": UA})
    body = None
    for i in range(retries):
        try:
            with urllib.request.urlopen(req, timeout=20) as r:
                body = r.read().decode("utf-8", "replace")
            break
        except Exception as e:                    # the service returns 500 about a third of the time
            last = e
            time.sleep(1.5 * (i + 1))
    BUDGET.write_text(str(spent() + 1))
    if body is None:
        return None
    verdict = ("helyes" if "helyes" in body.lower() and "ismeretlen" not in body.lower()
               else "ismeretlen" if "ismeretlen" in body.lower() else "unknown")
    rec = {"form": form, "verdict": verdict, "raw": body[:2000],
           "ts": datetime.now(timezone.utc).isoformat()}
    p.write_text(json.dumps(rec, ensure_ascii=False, indent=1), encoding="utf-8")
    time.sleep(1.0)
    return rec


def pair(a: str, b: str) -> tuple:
    ra, rb = query(a), query(b)
    return (ra["verdict"] if ra else "error", rb["verdict"] if rb else "error")


if __name__ == "__main__":
    for w in sys.argv[1:]:
        r = query(w)
        print(w, r["verdict"] if r else "error")
    print("budget spent:", spent())
