"""
build_academic_corpus.py
Builds the academic whitepaper layer for Albert v3.0 tokenizer training.

All sources are open access — no API keys, no PDF parsing required.
Text is harvested via OAI-PMH (standard academic harvesting protocol)
and the SciELO REST API, returning structured Dublin Core metadata
(title + abstract as plain prose).

Sources:
  arXiv OAI-PMH    → English  (CS, math, physics, bio, econ, linguistics, +)
  HAL OAI-PMH      → French   (archives-ouvertes.fr — all disciplines)
  SciELO REST      → Spanish, Portuguese
  Zenodo OAI-PMH   → German, Italian, Dutch, Polish (multilingual open access)

Output: data/corpus/academic/{lang}.txt  (~10–20 MB each)

12 disciplines:
  Computer Science / AI · Mathematics · Physics · Biology / Medicine ·
  Economics · Linguistics · History · Philosophy · Law · Engineering ·
  Environmental Science · Social Sciences

Usage:
    python3 scripts/build_academic_corpus.py
    python3 scripts/build_academic_corpus.py --lang de fr
    python3 scripts/build_academic_corpus.py --target-mb 15
"""

import argparse
import os
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT  = os.path.dirname(SCRIPT_DIR)
OUT_DIR    = os.path.join(REPO_ROOT, "data", "corpus", "academic")

DEFAULT_TARGET_MB = 12
UA = "AlbertV3AcademicBuilder/1.0 (rfi.irfos@gmail.com)"

# OAI-PMH namespace
NS = {
    "oai":  "http://www.openarchives.org/OAI/2.0/",
    "dc":   "http://purl.org/dc/elements/1.1/",
    "oai_dc": "http://www.openarchives.org/OAI/2.0/oai_dc/",
}

# ── HTTP helpers ──────────────────────────────────────────────────────────────

def fetch(url: str, params: dict = None, retries: int = 4, delay: float = 3.0) -> bytes:
    if params:
        url = url + "?" + urllib.parse.urlencode(params)
    for attempt in range(retries):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": UA})
            with urllib.request.urlopen(req, timeout=30) as r:
                return r.read()
        except (urllib.error.URLError, OSError) as e:
            if attempt == retries - 1:
                raise
            time.sleep(delay * (attempt + 1))


def fetch_oai(base_url: str, params: dict) -> ET.Element:
    raw = fetch(base_url, params)
    return ET.fromstring(raw)


# ── OAI-PMH generic harvester ─────────────────────────────────────────────────

def harvest_oai(base_url: str, set_spec: str = None, lang_filter: str = None,
                max_records: int = 5000, wall_timeout: float = 0.0) -> list[str]:
    """
    Harvest title + abstract text from an OAI-PMH endpoint.
    Returns a list of cleaned text blocks.
    wall_timeout: stop after this many seconds (0 = unlimited).
    """
    texts = []
    params = {"verb": "ListRecords", "metadataPrefix": "oai_dc"}
    if set_spec:
        params["set"] = set_spec

    resumption = None
    harvest_start = time.time()
    while len(texts) < max_records:
        if wall_timeout > 0 and time.time() - harvest_start > wall_timeout:
            print(f"    harvest_oai: wall timeout {wall_timeout:.0f}s reached ({len(texts)} records)")
            break
        if resumption:
            root = fetch_oai(base_url, {"verb": "ListRecords", "resumptionToken": resumption})
        else:
            root = fetch_oai(base_url, params)

        lr = root.find("oai:ListRecords", NS)
        if lr is None:
            break

        for record in lr.findall("oai:record", NS):
            meta = record.find("oai:metadata", NS)
            if meta is None:
                continue
            dc = meta.find("oai_dc:dc", NS)
            if dc is None:
                continue

            # Language filter
            if lang_filter:
                lang_els = dc.findall("dc:language", NS)
                langs = [l.text or "" for l in lang_els]
                if not any(lang_filter.lower() in lg.lower() for lg in langs):
                    continue

            # Extract title(s) and description(s) / abstracts
            titles = [e.text or "" for e in dc.findall("dc:title", NS)]
            descs  = [e.text or "" for e in dc.findall("dc:description", NS)]

            block = " ".join(t.strip() for t in titles + descs if t.strip())
            block = clean_academic_text(block)
            if len(block) > 80:
                texts.append(block)

        # Pagination
        rt = lr.find("oai:resumptionToken", NS)
        if rt is not None and rt.text:
            resumption = rt.text.strip()
            # Brief pause — be polite to servers
            time.sleep(1.0)
        else:
            break

    return texts


# ── Text cleaning ─────────────────────────────────────────────────────────────

def clean_academic_text(text: str) -> str:
    # Remove LaTeX commands  \command{...} and \command
    text = re.sub(r"\\[a-zA-Z]+\{[^}]*\}", " ", text)
    text = re.sub(r"\\[a-zA-Z]+", " ", text)
    # Remove HTML tags
    text = re.sub(r"<[^>]+>", " ", text)
    # Remove URLs
    text = re.sub(r"https?://\S+", " ", text)
    # Remove DOIs
    text = re.sub(r"\b10\.\d{4,}/\S+", " ", text)
    # Remove XML entities
    text = text.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">")
    text = text.replace("&quot;", '"').replace("&nbsp;", " ")
    text = re.sub(r"&\w+;", " ", text)
    # Collapse whitespace
    text = re.sub(r"[ \t]+", " ", text)
    text = re.sub(r"\n{2,}", "\n", text)
    return text.strip()


# ── Language-specific harvesters ──────────────────────────────────────────────

# arXiv discipline sets for English
ARXIV_SETS = [
    "cs",           # Computer Science (CS + AI)
    "math",         # Mathematics
    "physics",      # Physics
    "q-bio",        # Quantitative Biology / Medicine
    "econ",         # Economics
    "stat",         # Statistics
    "eess",         # Electrical Engineering & Systems
    "q-fin",        # Quantitative Finance
]

def harvest_en(target_bytes: int) -> list[str]:
    """Harvest English academic text from arXiv OAI-PMH."""
    base = "https://export.arxiv.org/oai2"
    texts = []
    written = 0
    per_set = max(200, (target_bytes // len(ARXIV_SETS)) // 400)  # ~400 chars/abstract

    for s in ARXIV_SETS:
        if written >= target_bytes:
            break
        print(f"  [en] arXiv set={s} ...")
        try:
            batch = harvest_oai(base, set_spec=s, max_records=per_set)
            texts.extend(batch)
            written += sum(len(t.encode()) for t in batch)
            print(f"  [en] {s}: +{len(batch)} records · {written/1024/1024:.1f}MB total")
        except Exception as e:
            print(f"  [en] {s}: ERROR — {e}")
        time.sleep(2.0)  # arXiv rate limit: 1 request/5s; OAI pages take ~2s each

    return texts


def harvest_fr(target_bytes: int) -> list[str]:
    """Harvest French academic text from HAL JSON search API.
    Uses the search endpoint (not OAI-PMH — their OAI returns malformed XML).
    Domain sub-filters (domainAllCodeLeaf_s) return 0 results; use a single
    broad query across all 599k French journal articles instead.
    """
    import json
    base  = "https://api.archives-ouvertes.fr/search/"
    texts = []
    written = 0
    start = 0
    rows  = 500

    print("  [fr] HAL (599k French journal articles — title+abstract) ...")
    while written < target_bytes:
        try:
            params = {
                "q":    "language_s:fr",
                "fq":   "docType_s:ART",
                "wt":   "json",
                "fl":   "title_s,abstract_s",
                "rows": rows,
                "start": start,
                "sort": "producedDate_tdate desc",
            }
            raw  = fetch(base, params)
            data = json.loads(raw)
            docs = data.get("response", {}).get("docs", [])
            if not docs:
                break
            batch_written = 0
            for doc in docs:
                parts = []
                for key in ("title_s", "abstract_s"):
                    val = doc.get(key, [])
                    if isinstance(val, list):
                        parts.extend(val)
                    elif val:
                        parts.append(val)
                block = clean_academic_text(" ".join(p for p in parts if p))
                if len(block) > 80:
                    texts.append(block)
                    written      += len(block.encode())
                    batch_written += len(block.encode())
            print(f"  [fr] HAL page {start//rows+1}: +{len(docs)} docs "
                  f"(+{batch_written//1024}KB) · {written/1024/1024:.1f}MB total")
            start += rows
            time.sleep(1.0)
        except Exception as e:
            print(f"  [fr] HAL: ERROR — {e}")
            break

    print(f"  [fr] HAL done: {len(texts)} docs · {written/1024/1024:.1f}MB")
    return texts


def harvest_zenodo_lang(lang_code: str, target_bytes: int) -> list[str]:
    """
    Harvest academic text from Zenodo OAI-PMH filtered by language.
    lang_code: ISO 639-1 (de, it, nl, pl, es, pt ...)
    """
    base = "https://zenodo.org/oai2d"
    # Zenodo has community sets; we use type:publication and filter by language
    # Sets: user-{community} or openaire — use openaire for broad coverage
    zenodo_sets = [
        "openaire",          # OpenAIRE European open science
        "user-zenodo",       # General Zenodo content
    ]
    texts = []
    written = 0

    for s in zenodo_sets:
        if written >= target_bytes:
            break
        print(f"  [{lang_code}] Zenodo set={s} lang={lang_code} ...")
        try:
            per_set = max(500, (target_bytes - written) // 350)
            batch = harvest_oai(base, set_spec=s, lang_filter=lang_code,
                                max_records=per_set, wall_timeout=90.0)
            texts.extend(batch)
            written += sum(len(t.encode()) for t in batch)
            print(f"  [{lang_code}] {s}: +{len(batch)} records · {written/1024/1024:.1f}MB total")
        except Exception as e:
            print(f"  [{lang_code}] Zenodo {s}: ERROR — {e}")
        time.sleep(2.0)

    return texts


SCIELO_COLLECTIONS = {
    "es": ["chl", "col", "esp", "mex", "per", "ven", "arg"],  # Spanish collections
    "pt": ["scl", "prt"],                                      # Portuguese collections
}

def harvest_scielo(lang_code: str, target_bytes: int) -> list[str]:
    """Harvest Spanish/Portuguese academic text from SciELO REST API."""
    collections = SCIELO_COLLECTIONS.get(lang_code, [])
    texts = []
    written = 0

    for coll in collections:
        if written >= target_bytes:
            break
        offset = 0
        limit  = 100
        print(f"  [{lang_code}] SciELO collection={coll} ...")
        while written < target_bytes:
            try:
                url = (f"https://articlemeta.scielo.org/api/v1/article/identifiers/"
                       f"?collection={coll}&limit={limit}&offset={offset}")
                raw = fetch(url)
                import json
                data = json.loads(raw)
                pids = [item.get("code", "") for item in data.get("objects", [])]
                if not pids:
                    break
                for pid in pids:
                    if written >= target_bytes:
                        break
                    try:
                        art_url = (f"https://articlemeta.scielo.org/api/v1/article/"
                                   f"?collection={coll}&pid={pid}")
                        art_raw = fetch(art_url)
                        art = json.loads(art_raw)
                        # Extract titles and abstracts in target language
                        titles = art.get("titles", {})
                        abstracts = art.get("abstracts", {})
                        parts = []
                        for lk in [lang_code, lang_code.upper()]:
                            if lk in titles:
                                parts.append(titles[lk])
                            if lk in abstracts:
                                parts.append(abstracts[lk])
                        if parts:
                            block = clean_academic_text(" ".join(parts))
                            if len(block) > 80:
                                texts.append(block)
                                written += len(block.encode())
                        time.sleep(0.1)
                    except Exception:
                        continue
                offset += limit
                time.sleep(0.5)
            except Exception as e:
                print(f"  [{lang_code}] SciELO {coll}: ERROR — {e}")
                break

    return texts


# ── Generic HAL harvester (any language) ─────────────────────────────────────

def harvest_hal_lang(lang: str, target_bytes: int) -> list[str]:
    """
    Harvest academic abstracts from HAL for any ISO 639-1 language code.
    HAL coverage: es=11k, de=3.6k, it=5.8k, pt=3.5k, nl=196, pl=271.
    Fast (~1 page/s), reliable — use before Zenodo fallback for all non-EN/FR.
    """
    import json
    base  = "https://api.archives-ouvertes.fr/search/"
    texts = []
    written = 0
    start_offset = 0
    rows = 500

    print(f"  [{lang}] HAL (language_s:{lang}) ...")
    while written < target_bytes:
        try:
            params = {
                "q":    f"language_s:{lang}",
                "fq":   "docType_s:ART",
                "wt":   "json",
                "fl":   "title_s,abstract_s",
                "rows": rows,
                "start": start_offset,
                "sort": "producedDate_tdate desc",
            }
            raw  = fetch(base, params)
            data = json.loads(raw)
            docs = data.get("response", {}).get("docs", [])
            if not docs:
                break
            batch_written = 0
            for doc in docs:
                parts = []
                for key in ("title_s", "abstract_s"):
                    val = doc.get(key, [])
                    if isinstance(val, list):
                        parts.extend(val)
                    elif val:
                        parts.append(val)
                block = clean_academic_text(" ".join(p for p in parts if p))
                if len(block) > 80:
                    texts.append(block)
                    written      += len(block.encode())
                    batch_written += len(block.encode())
            print(f"  [{lang}] HAL page {start_offset//rows+1}: +{len(docs)} docs "
                  f"(+{batch_written//1024}KB) · {written/1024/1024:.1f}MB total")
            start_offset += rows
            time.sleep(1.0)
        except Exception as e:
            print(f"  [{lang}] HAL: ERROR — {e}")
            break

    print(f"  [{lang}] HAL done: {len(texts)} docs · {written/1024/1024:.1f}MB")
    return texts


# ── Dispatch ──────────────────────────────────────────────────────────────────

def build_language(lang: str, out_path: str, target_bytes: int) -> None:
    start = time.time()
    texts = []

    if lang == "en":
        texts = harvest_en(target_bytes)
    elif lang == "fr":
        texts = harvest_fr(target_bytes)
        written = sum(len(t.encode()) for t in texts)
        if written < target_bytes:
            texts += harvest_zenodo_lang("fr", target_bytes - written)
    else:
        # es, pt, de, it, nl, pl — HAL primary, Zenodo supplement
        texts = harvest_hal_lang(lang, target_bytes)
        written = sum(len(t.encode()) for t in texts)
        if written < target_bytes:
            texts += harvest_zenodo_lang(lang, target_bytes - written)

    written = sum(len(t.encode()) for t in texts)
    elapsed = time.time() - start

    if not texts:
        print(f"[{lang}] WARNING: No text harvested — check network or source availability")
        return

    with open(out_path, "w", encoding="utf-8") as f:
        for t in texts:
            f.write(t + "\n\n")

    print(f"[{lang}] Done: {len(texts):,} records · {written/1024/1024:.1f}MB · {elapsed:.0f}s → {out_path}")


# ── Main ──────────────────────────────────────────────────────────────────────

ALL_LANGS = ["en", "fr", "es", "pt", "de", "it", "nl", "pl"]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--lang", nargs="*", default=ALL_LANGS)
    parser.add_argument("--target-mb", type=int, default=DEFAULT_TARGET_MB)
    args = parser.parse_args()

    os.makedirs(OUT_DIR, exist_ok=True)
    target_bytes = args.target_mb * 1024 * 1024

    for lang in args.lang:
        if lang not in ALL_LANGS:
            print(f"Unknown language: {lang}")
            continue
        out_path = os.path.join(OUT_DIR, f"{lang}.txt")
        if os.path.exists(out_path) and os.path.getsize(out_path) > 512 * 1024:
            mb = os.path.getsize(out_path) / 1024 / 1024
            print(f"[{lang}] Already exists ({mb:.1f}MB) — skipping (delete to re-harvest)")
            continue
        print(f"\n[{lang}] Harvesting academic corpus (target: {args.target_mb}MB)...")
        try:
            build_language(lang, out_path, target_bytes)
        except Exception as e:
            print(f"[{lang}] ERROR: {e}")
            import traceback; traceback.print_exc()

    # Summary
    print("\n=== Academic Corpus Summary ===")
    total = 0
    for lang in ALL_LANGS:
        p = os.path.join(OUT_DIR, f"{lang}.txt")
        if os.path.exists(p):
            mb = os.path.getsize(p) / 1024 / 1024
            total += os.path.getsize(p)
            print(f"  {lang}: {mb:.1f}MB")
    print(f"  TOTAL: {total/1024/1024:.1f}MB")
    print(f"\nAcademic corpus ready at: {OUT_DIR}")
    print("Merge with Wikipedia corpus, then: python3 scripts/train_tokenizer_v3.py")


if __name__ == "__main__":
    main()
