#!/usr/bin/env python3
"""
Download and prepare additional training corpora for Albert MoE-13.
All files land in data/corpus_staged/ — move to data/corpus/ when ready to train on them.

Sources:
  1. Simple English Wikipedia (via Wikimedia REST API, batch article extracts)
  2. Project Gutenberg classics (already downloaded — skip if present)
  3. EU AI Act (GitHub plaintext mirror)
  4. Linux kernel docs (RST source files from kernel.org)
  5. TLDR pages (concise Unix command documentation)
"""

import os
import sys
import json
import time
import urllib.request
import urllib.parse
import gzip
import re

OUT_DIR = os.path.join(os.path.dirname(__file__), "..", "data", "corpus_staged")
os.makedirs(OUT_DIR, exist_ok=True)

UA = "AlbertCorpusBot/1.0 (training corpus; contact rfi.irfos@gmail.com)"

def log(msg):
    print(f"[corpus] {msg}", flush=True)

def fetch_url(url, timeout=30):
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=timeout) as r:
        return r.read()

def download(url, dest, timeout=60):
    if os.path.exists(dest) and os.path.getsize(dest) > 1000:
        log(f"  already have {os.path.basename(dest)}, skipping")
        return True
    log(f"  downloading {os.path.basename(dest)} ...")
    try:
        data = fetch_url(url, timeout)
        with open(dest, 'wb') as f:
            f.write(data)
        log(f"  saved {len(data)//1024}KB → {os.path.basename(dest)}")
        return True
    except Exception as e:
        log(f"  FAILED: {e}")
        return False


# ── 1. Simple English Wikipedia via API ──────────────────────────────────────
def fetch_simple_wiki():
    out = os.path.join(OUT_DIR, "simple_wikipedia.txt")
    if os.path.exists(out) and os.path.getsize(out) > 100_000:
        log("simple_wikipedia.txt already exists, skipping")
        return

    log("Fetching Simple English Wikipedia via API (this takes ~5 minutes) ...")
    api = "https://simple.wikipedia.org/w/api.php"

    # Step 1: collect page IDs
    page_ids = []
    cont = ""
    while len(page_ids) < 200_000:
        url = (f"{api}?action=query&list=allpages&aplimit=500"
               f"&apnamespace=0&format=json")
        if cont:
            url += f"&apcontinue={urllib.parse.quote(cont)}"
        try:
            data = json.loads(fetch_url(url, timeout=30))
            page_ids.extend(p["pageid"] for p in data["query"]["allpages"])
            if "continue" not in data:
                break
            cont = data["continue"]["apcontinue"]
        except Exception as e:
            log(f"  listing error: {e}")
            break
    log(f"  found {len(page_ids):,} articles")

    # Step 2: fetch extracts in batches of 50
    articles = []
    for i in range(0, len(page_ids), 50):
        batch = "|".join(str(p) for p in page_ids[i:i+50])
        url = (f"{api}?action=query&prop=extracts&explaintext=1"
               f"&exlimit=50&pageids={batch}&format=json")
        try:
            data = json.loads(fetch_url(url, timeout=30))
            for page in data["query"]["pages"].values():
                text = page.get("extract", "")
                if text and len(text) > 150:
                    articles.append(text.strip())
        except Exception as e:
            log(f"  batch error at {i}: {e}")
        if i % 5000 == 0 and i > 0:
            log(f"  {i:,}/{len(page_ids):,} articles fetched ...")
        time.sleep(0.05)   # polite rate limiting

    combined = "\n\n".join(articles)
    with open(out, "w", encoding="utf-8") as f:
        f.write(combined)
    log(f"  wrote {len(combined):,} chars ({len(articles):,} articles) → simple_wikipedia.txt")


# ── 2. Project Gutenberg classics ────────────────────────────────────────────
GUTENBERG_BOOKS = [
    ("https://www.gutenberg.org/files/2701/2701-0.txt",  "gutenberg_moby_dick.txt"),
    ("https://www.gutenberg.org/files/1342/1342-0.txt",  "gutenberg_pride_prejudice.txt"),
    ("https://www.gutenberg.org/files/84/84-0.txt",      "gutenberg_frankenstein.txt"),
    ("https://www.gutenberg.org/files/1661/1661-0.txt",  "gutenberg_sherlock_holmes.txt"),
    ("https://www.gutenberg.org/files/98/98-0.txt",      "gutenberg_tale_two_cities.txt"),
    ("https://www.gutenberg.org/files/2600/2600-0.txt",  "gutenberg_war_and_peace.txt"),
    ("https://www.gutenberg.org/files/4300/4300-0.txt",  "gutenberg_ulysses.txt"),
    ("https://www.gutenberg.org/files/76/76-0.txt",      "gutenberg_huck_finn.txt"),
    ("https://www.gutenberg.org/files/1400/1400-0.txt",  "gutenberg_great_expectations.txt"),
    ("https://www.gutenberg.org/files/1232/1232-0.txt",  "gutenberg_the_prince.txt"),
    ("https://www.gutenberg.org/files/2554/2554-0.txt",  "gutenberg_crime_punishment.txt"),
    ("https://www.gutenberg.org/files/174/174-0.txt",    "gutenberg_picture_dorian_gray.txt"),
]

def fetch_gutenberg():
    for url, fname in GUTENBERG_BOOKS:
        download(url, os.path.join(OUT_DIR, fname))


# ── 3. EU AI Act — GitHub plaintext mirror ───────────────────────────────────
def fetch_eu_ai_act():
    out = os.path.join(OUT_DIR, "eu_ai_act.txt")
    if os.path.exists(out) and os.path.getsize(out) > 10_000:
        log("eu_ai_act.txt already exists, skipping")
        return

    # Use the official EUR-Lex plain-text export (format=TEXT in Accept header)
    urls = [
        # GitHub mirrors of the EU AI Act text
        "https://raw.githubusercontent.com/Regulaite/eu-ai-act/main/EU-AI-Act.txt",
        "https://raw.githubusercontent.com/ai-act/text/main/eu_ai_act.txt",
        # EUR-Lex direct text format
        "https://eur-lex.europa.eu/legal-content/EN/TXT/TXT/?uri=CELEX%3A32024R1689",
    ]
    for url in urls:
        try:
            log(f"  trying {url.split('/')[-1]} ...")
            req = urllib.request.Request(url, headers={
                "User-Agent": UA,
                "Accept": "text/plain, text/html",
            })
            with urllib.request.urlopen(req, timeout=30) as r:
                raw = r.read().decode("utf-8", errors="ignore")
            # Strip any HTML if needed
            if "<html" in raw.lower():
                raw = re.sub(r'<[^>]+>', ' ', raw)
                raw = re.sub(r'&\w+;', ' ', raw)
                raw = re.sub(r'\s{3,}', '\n\n', raw)
            if len(raw) > 10_000:
                with open(out, "w", encoding="utf-8") as f:
                    f.write(raw)
                log(f"  wrote {len(raw):,} chars → eu_ai_act.txt")
                return
        except Exception as e:
            log(f"  failed: {e}")
    log("  WARNING: could not fetch EU AI Act from any source")


# ── 4. Linux kernel documentation (RST source files) ─────────────────────────
KERNEL_DOCS = [
    "admin-guide/README",
    "admin-guide/kernel-parameters",
    "admin-guide/sysctl/kernel",
    "process/coding-style",
    "process/submitting-patches",
    "dev-tools/testing-overview",
    "core-api/kernel-api",
    "filesystems/vfs",
    "mm/memory-model",
    "networking/ip-sysctl",
]

def fetch_linux_docs():
    out = os.path.join(OUT_DIR, "linux_docs.txt")
    if os.path.exists(out) and os.path.getsize(out) > 50_000:
        log("linux_docs.txt already exists, skipping")
        return

    base = "https://www.kernel.org/doc/html/latest/_sources"
    collected = []
    for doc in KERNEL_DOCS:
        for ext in [".rst.txt", ".txt"]:
            url = f"{base}/{doc}{ext}"
            try:
                text = fetch_url(url, timeout=20).decode("utf-8", errors="ignore")
                if len(text) > 500:
                    collected.append(text)
                    log(f"  fetched {doc}{ext} ({len(text)//1024}KB)")
                    break
            except Exception:
                pass

    if collected:
        combined = "\n\n".join(collected)
        with open(out, "w", encoding="utf-8") as f:
            f.write(combined)
        log(f"  wrote {len(combined):,} chars → linux_docs.txt")
    else:
        log("  WARNING: could not fetch any kernel docs")


# ── 5. TLDR pages — concise Unix command docs ─────────────────────────────────
def fetch_tldr():
    out = os.path.join(OUT_DIR, "tldr_pages.txt")
    if os.path.exists(out) and os.path.getsize(out) > 10_000:
        log("tldr_pages.txt already exists, skipping")
        return

    # TLDR pages GitHub archive (single zip with all pages)
    zip_url = "https://github.com/tldr-pages/tldr/releases/latest/download/tldr.zip"
    zip_path = "/tmp/tldr.zip"
    try:
        log("  downloading TLDR pages ...")
        data = fetch_url(zip_url, timeout=60)
        with open(zip_path, "wb") as f:
            f.write(data)

        import zipfile
        collected = []
        with zipfile.ZipFile(zip_path) as z:
            for name in z.namelist():
                if name.endswith(".md") and "/pages/" in name:
                    text = z.read(name).decode("utf-8", errors="ignore")
                    collected.append(text)

        combined = "\n\n".join(collected)
        with open(out, "w", encoding="utf-8") as f:
            f.write(combined)
        log(f"  wrote {len(combined):,} chars ({len(collected)} pages) → tldr_pages.txt")
    except Exception as e:
        log(f"  WARNING: could not fetch TLDR pages: {e}")


if __name__ == "__main__":
    log("Starting corpus download ...")
    for fn in [fetch_gutenberg, fetch_simple_wiki, fetch_eu_ai_act,
               fetch_linux_docs, fetch_tldr]:
        try:
            fn()
        except Exception as e:
            log(f"WARNING: {fn.__name__} crashed: {e}")

    log("\nDone. Files in data/corpus_staged/:")
    total = 0
    for f in sorted(os.listdir(OUT_DIR)):
        size = os.path.getsize(os.path.join(OUT_DIR, f))
        total += size
        log(f"  {f:45s}  {size/1024/1024:.1f} MB")
    log(f"  {'TOTAL':45s}  {total/1024/1024:.1f} MB")
