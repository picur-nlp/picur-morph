"""Workspace paths; layout in WORKSPACE.md."""
from __future__ import annotations

import os
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
LOCAL = Path(os.environ.get("PICUR_LOCAL", REPO / "local"))
LISTS = LOCAL / "lists"
CACHE = LOCAL / "cache"
CORPORA = LOCAL / "corpora"
COUNTS = CACHE / "counts.json"
RESEARCH = LOCAL / "research-impl"
LEXDATA = REPO / "crates/picur-morph-lexicon/data"
WC2_PARQUET = LOCAL / "research-racz/Webcorpus2FrequencyList/frequencies.parquet"
WC2_DB = CACHE / "webcorpus2.sqlite"
ENYELV = Path(os.environ.get("PICUR_ENYELV", CACHE / "enyelv/qa.jsonl"))
