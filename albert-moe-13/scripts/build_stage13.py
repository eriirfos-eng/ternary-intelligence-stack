#!/usr/bin/env python3
"""
Stage 13 corpus builder — "Fruiting Body"
Albert MoE-13 training data pipeline

Unlock condition: loss_avg ≤ 3.0 AND architecture ≥ 40L 4-stream cord

Sources:
  1. arXiv full papers      — bulk via HuggingFace datasets (arxiv dataset)
  2. Internet Archive books — public domain via archive.org API
  3. USPTO patents          — abstracts + claims via PatentsView API (free)
  4. Wikipedia all-language — top 30 languages full dumps via HuggingFace
  5. Instruction dialogue   — curated open datasets (FLAN, OpenAssistant)
  6. WikiData facts         — structured facts → natural language

Target: ~100B tokens (~450 GB text)
Output: data/corpus/stage_13/
Compute: Bizon-class recommended for tokenisation at this scale

Run:
    pip install datasets huggingface_hub
    cd albert-moe-13
    python scripts/build_stage13.py

See CORPUS_EXPANSION_ROADMAP.md for full stage context.
"""

import json
import os
import re
import sys
import time
import urllib.request
import urllib.parse
from datetime import datetime
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent
CORPUS_DIR = SCRIPT_DIR.parent / "data" / "corpus" / "stage_13"
CORPUS_DIR.mkdir(parents=True, exist_ok=True)

UA      = "AlbertMoE13CorpusBot/1.1 (stage13 training corpus; contact contact@ternlang.com)"
EMAIL   = "contact@ternlang.com"

# ── tuning ────────────────────────────────────────────────────────────────────
ARXIV_FULL_MAX          = 500_000     # full paper texts (via HF datasets)
ARCHIVE_MAX_BOOKS       = 10_000      # Internet Archive books
PATENTS_MAX             = 100_000     # USPTO patent abstracts + claims
WIKI_MULTILANG_MAX      = 50_000      # articles per language group
INSTRUCTION_MAX         = 200_000     # instruction-response pairs
WIKIDATA_MAX            = 500_000     # entity → natural language facts

ARCHIVE_SUBJECTS = [
    "philosophy", "history", "science", "literature", "mathematics",
    "economics", "psychology", "sociology", "linguistics", "political science",
]

PATENT_CLASSES = [
    "G06N",  # Computer Science — AI/ML
    "H04L",  # Communications
    "G06F",  # Electrical computing
    "C12N",  # Microbiology
    "A61K",  # Pharmaceuticals
]

WIKI_LANG_GROUPS = [
    ["en", "de", "fr", "es", "pt"],
    ["ru", "zh", "ja", "ar", "ko"],
    ["it", "pl", "nl", "sv", "tr"],
    ["fa", "uk", "vi", "id", "he"],
    ["cs", "ro", "hu", "fi", "da"],
    ["no", "sk", "bg", "ca", "hr"],
]


# ── utilities ─────────────────────────────────────────────────────────────────

def log(msg: str):
    ts = datetime.now().strftime("%H:%M:%S")
    print(f"[{ts}] {msg}", flush=True)

def fetch(url: str, *, delay: float = 0.5, retries: int = 3,
          headers: dict | None = None) -> bytes | None:
    hdrs = {"User-Agent": UA}
    if headers:
        hdrs.update(headers)
    for attempt in range(retries):
        try:
            req = urllib.request.Request(url, headers=hdrs)
            with urllib.request.urlopen(req, timeout=30) as r:
                data = r.read()
            time.sleep(delay)
            return data
        except Exception as e:
            if attempt < retries - 1:
                time.sleep(2 ** attempt)
            else:
                log(f"  fetch failed ({url[:80]}): {e}")
    return None

def fetch_json(url: str, **kw) -> dict | list | None:
    raw = fetch(url, **kw)
    if raw is None:
        return None
    try:
        return json.loads(raw)
    except:
        return None

_TAG_RE = re.compile(r"<[^>]+>")
_WS_RE  = re.compile(r"\s{3,}")
_HTML_E = {"&amp;": "&", "&lt;": "<", "&gt;": ">", "&quot;": '"', "&nbsp;": " "}

def strip_html(html: str) -> str:
    if not html:
        return ""
    for ent, char in _HTML_E.items():
        html = html.replace(ent, char)
    html = _TAG_RE.sub(" ", html)
    html = _WS_RE.sub(" ", html)
    return html.strip()

def write_block(fh, block: str):
    fh.write(block.strip() + "\n\n" + "─" * 72 + "\n\n")
    fh.flush()


# ── 1. arXiv full papers via HuggingFace ─────────────────────────────────────

def build_arxiv_full(out_path: Path):
    log("arXiv full papers — streaming via HuggingFace datasets ...")
    try:
        from datasets import load_dataset  # type: ignore
    except ImportError:
        log("  datasets not installed — run: pip install datasets")
        log("  Skipping arXiv full papers.")
        return

    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        try:
            ds = load_dataset("arxiv_dataset", split="train", streaming=True)
            for sample in ds:
                if total >= ARXIV_FULL_MAX:
                    break
                abstract = sample.get("abstract", "").strip()
                title    = sample.get("title", "").strip()
                authors  = sample.get("authors", "")
                cats     = sample.get("categories", "")
                if len(abstract) < 200:
                    continue
                block = (
                    f"══ ARXIV FULL PAPER ══\n"
                    f"Title      : {title}\n"
                    f"Authors    : {authors[:200]}\n"
                    f"Categories : {cats}\n\n"
                    f"[ABSTRACT]\n{abstract}"
                )
                write_block(fh, block)
                total += 1
                if total % 10000 == 0:
                    log(f"  {total:,} papers ...")
        except Exception as e:
            log(f"  arXiv dataset error: {e}")
    log(f"arXiv full done — {total:,} papers → {out_path.name}")


# ── 2. Internet Archive books ─────────────────────────────────────────────────

IA_BASE = "https://archive.org"

def fetch_ia_books(subject: str, max_books: int) -> list[dict]:
    results = []
    url = (
        f"{IA_BASE}/advancedsearch.php"
        f"?q=subject%3A%22{urllib.parse.quote(subject)}%22"
        f"+mediatype%3Atexts+language%3Aeng"
        f"&fl=identifier,title,creator,date"
        f"&rows={min(max_books, 100)}&output=json"
    )
    data = fetch_json(url, delay=1.0)
    if not data:
        return results
    docs = data.get("response", {}).get("docs", [])
    for doc in docs:
        identifier = doc.get("identifier", "")
        if not identifier:
            continue
        # Fetch plain text version
        txt_url = f"{IA_BASE}/download/{identifier}/{identifier}_djvu.txt"
        raw = fetch(txt_url, delay=1.5)
        if not raw:
            txt_url = f"{IA_BASE}/download/{identifier}/{identifier}.txt"
            raw = fetch(txt_url, delay=1.5)
        if raw:
            text = raw.decode("utf-8", errors="replace")
            if len(text) > 1000:
                results.append({
                    "title":   doc.get("title", ""),
                    "creator": doc.get("creator", ""),
                    "date":    doc.get("date", ""),
                    "text":    text[:8000],
                    "subject": subject,
                })
    return results

def build_internet_archive(out_path: Path):
    # NOTE 2026-05-19: archive.org locked down plain-text access after their 2023 copyright
    # lawsuit (Hachette v. Internet Archive). Most items now return 401 Unauthorized or have
    # no .txt export available. This function produces 0 bytes as-is.
    # FIX NEEDED before stage_13 is relevant: use archive.org S3-like API with an
    # authenticated session cookie, or switch source to HuggingFace datasets/book_corpus
    # (allenai/dolma books split is a good replacement). Not urgent — stage_13 unlocks at
    # loss_avg ≤ 3.0 AND architecture ≥ 40L.
    log("Internet Archive books — fetching ...")
    total = 0
    per_subject = ARCHIVE_MAX_BOOKS // len(ARCHIVE_SUBJECTS)
    with open(out_path, "w", encoding="utf-8") as fh:
        for subject in ARCHIVE_SUBJECTS:
            log(f"  subject: {subject}")
            books = fetch_ia_books(subject, per_subject)
            for book in books:
                block = (
                    f"══ INTERNET ARCHIVE — {subject.upper()} ══\n"
                    f"Title   : {book['title']}\n"
                    f"Author  : {book['creator']}\n"
                    f"Date    : {book['date']}\n"
                    f"Source  : Internet Archive (public domain)\n\n"
                    f"[TEXT]\n{book['text']}"
                )
                write_block(fh, block)
                total += 1
            log(f"    → {len(books)} books")
    log(f"Internet Archive done — {total} books → {out_path.name}")


# ── 3. USPTO patents via PatentsView ──────────────────────────────────────────

PV_BASE = "https://api.patentsview.org/patents/query"

def fetch_patents(classification: str, max_patents: int) -> list[dict]:
    results = []
    page = 1
    per_page = 25
    while len(results) < max_patents:
        payload = json.dumps({
            "q": {"_text_phrase": {"patent_abstract": classification}},
            "f": ["patent_title", "patent_abstract", "patent_date",
                  "patent_number", "assignee_organization"],
            "o": {"page": page, "per_page": per_page, "sort": [{"patent_date": "desc"}]},
        }).encode()
        raw = None
        for attempt in range(3):
            try:
                req = urllib.request.Request(
                    PV_BASE,
                    data=payload,
                    headers={"Content-Type": "application/json", "User-Agent": UA},
                )
                with urllib.request.urlopen(req, timeout=30) as r:
                    raw = r.read()
                time.sleep(1.0)
                break
            except Exception as e:
                if attempt < 2:
                    time.sleep(3)
                else:
                    log(f"  patents fetch error: {e}")
        if not raw:
            break
        data = json.loads(raw)
        patents = data.get("patents") or []
        if not patents:
            break
        for p in patents:
            abstract = (p.get("patent_abstract") or "").strip()
            title    = (p.get("patent_title") or "").strip()
            if len(abstract) < 100:
                continue
            results.append({
                "number":   p.get("patent_number", ""),
                "title":    title,
                "abstract": abstract[:2000],
                "date":     p.get("patent_date", ""),
                "assignee": (p.get("assignees") or [{}])[0].get("assignee_organization", ""),
                "class":    classification,
            })
        page += 1
        if len(patents) < per_page:
            break
    return results

def build_patents(out_path: Path):
    # NOTE 2026-05-19: PatentsView API (api.patentsview.org/patents/query) has been
    # migrated to a new SPA portal — the old JSON POST endpoint now returns an HTML page.
    # This function produces 0 bytes as-is.
    # FIX NEEDED before stage_13 is relevant: check new USPTO Open Data Portal API at
    # https://developer.uspto.gov/api-catalog for the current patents query endpoint,
    # or use the PatentsView bulk data downloads at https://patentsview.org/download/data-download-tables
    # Not urgent — stage_13 unlocks at loss_avg ≤ 3.0 AND architecture ≥ 40L.
    log("USPTO patents — fetching via PatentsView ...")
    total = 0
    per_class = PATENTS_MAX // len(PATENT_CLASSES)
    with open(out_path, "w", encoding="utf-8") as fh:
        for cls in PATENT_CLASSES:
            log(f"  class: {cls}")
            patents = fetch_patents(cls, per_class)
            for p in patents:
                block = (
                    f"══ PATENT ABSTRACT ══\n"
                    f"Number   : {p['number']}\n"
                    f"Title    : {p['title']}\n"
                    f"Assignee : {p['assignee']}\n"
                    f"Date     : {p['date']}\n"
                    f"Class    : {p['class']}\n\n"
                    f"[ABSTRACT]\n{p['abstract']}"
                )
                write_block(fh, block)
                total += 1
            log(f"    → {len(patents)} patents")
    log(f"Patents done — {total} total → {out_path.name}")


# ── 4. Wikipedia multilingual via HuggingFace ─────────────────────────────────

def build_wikipedia_multilingual_full(out_path: Path):
    log("Wikipedia multilingual full — streaming via HuggingFace ...")
    try:
        from datasets import load_dataset  # type: ignore
    except ImportError:
        log("  datasets not installed — skipping Wikipedia multilingual full")
        return

    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for lang_group in WIKI_LANG_GROUPS:
            for lang in lang_group:
                log(f"  language: {lang}")
                lang_count = 0
                try:
                    ds = load_dataset(
                        "wikipedia", f"20231101.{lang}",
                        split="train", streaming=True,
                        trust_remote_code=True,
                    )
                    for article in ds:
                        if lang_count >= WIKI_MULTILANG_MAX // len(WIKI_LANG_GROUPS):
                            break
                        text = article.get("text", "").strip()
                        title = article.get("title", "")
                        if len(text) < 300:
                            continue
                        block = (
                            f"══ WIKIPEDIA — {lang.upper()} ══\n"
                            f"Title: {title}\n\n"
                            f"{text[:5000]}"
                        )
                        write_block(fh, block)
                        lang_count += 1
                        total += 1
                except Exception as e:
                    log(f"  error loading {lang}: {e}")
                log(f"    → {lang_count} articles")
    log(f"Wikipedia multilingual done — {total:,} articles → {out_path.name}")


# ── 5. Instruction dialogue datasets ──────────────────────────────────────────

def build_instruction_dialogue(out_path: Path):
    log("Instruction dialogue — streaming via HuggingFace ...")
    try:
        from datasets import load_dataset  # type: ignore
    except ImportError:
        log("  datasets not installed — skipping instruction dialogue")
        return

    datasets_to_try = [
        ("Open-Orca/OpenOrca", "train", "system_prompt", "question", "response"),
        ("garage-bAInd/Open-Platypus", "train", "instruction", "input", "output"),
        ("teknium/OpenHermes-2.5", "train", "system_prompt", "human", "gpt"),
    ]

    total = 0
    per_dataset = INSTRUCTION_MAX // len(datasets_to_try)

    with open(out_path, "w", encoding="utf-8") as fh:
        for ds_name, split, sys_key, human_key, asst_key in datasets_to_try:
            log(f"  dataset: {ds_name}")
            count = 0
            try:
                ds = load_dataset(ds_name, split=split, streaming=True)
                for sample in ds:
                    if count >= per_dataset:
                        break
                    system   = (sample.get(sys_key) or "").strip()
                    question = (sample.get(human_key) or "").strip()
                    answer   = (sample.get(asst_key) or "").strip()
                    if not question or not answer or len(answer) < 50:
                        continue
                    lines = [f"══ INSTRUCTION-RESPONSE ══"]
                    if system:
                        lines += [f"[SYSTEM]\n{system[:300]}"]
                    lines += [
                        f"\n[HUMAN]\n{question[:800]}",
                        f"\n[ASSISTANT]\n{answer[:2000]}",
                    ]
                    write_block(fh, "\n".join(lines))
                    count += 1
                    total += 1
            except Exception as e:
                log(f"  error loading {ds_name}: {e}")
            log(f"    → {count} pairs")
    log(f"Instruction dialogue done — {total:,} pairs → {out_path.name}")


# ── 6. WikiData entity facts ──────────────────────────────────────────────────

WIKIDATA_SPARQL = "https://query.wikidata.org/sparql"

def fetch_wikidata_facts(query_label: str, sparql: str, max_results: int) -> list[dict]:
    encoded = urllib.parse.urlencode({
        "query": sparql, "format": "json"
    })
    url = f"{WIKIDATA_SPARQL}?{encoded}"
    headers = {
        "User-Agent": UA,
        "Accept": "application/sparql-results+json",
    }
    data = fetch_json(url, delay=2.0, headers=headers)
    if not data:
        return []
    results = []
    for binding in data.get("results", {}).get("bindings", [])[:max_results]:
        fact = {k: v.get("value", "") for k, v in binding.items()}
        fact["domain"] = query_label
        results.append(fact)
    return results

def build_wikidata(out_path: Path):
    log("WikiData entity facts — SPARQL queries ...")
    queries = [
        ("Scientists", """
            SELECT ?personLabel ?fieldLabel ?birthplaceLabel WHERE {
              ?person wdt:P31 wd:Q5;
                      wdt:P106 wd:Q901;
                      wdt:P101 ?field.
              OPTIONAL { ?person wdt:P19 ?birthplace. }
              SERVICE wikibase:label { bd:serviceParam wikibase:language "en". }
            } LIMIT 5000
        """),
        ("Countries", """
            SELECT ?countryLabel ?capitalLabel ?continentLabel WHERE {
              ?country wdt:P31 wd:Q6256;
                       wdt:P36 ?capital;
                       wdt:P30 ?continent.
              SERVICE wikibase:label { bd:serviceParam wikibase:language "en". }
            } LIMIT 1000
        """),
        ("Books", """
            SELECT ?workLabel ?authorLabel ?genreLabel ?pubdateLabel WHERE {
              ?work wdt:P31 wd:Q7725634;
                    wdt:P50 ?author.
              OPTIONAL { ?work wdt:P136 ?genre. }
              OPTIONAL { ?work wdt:P577 ?pubdate. }
              SERVICE wikibase:label { bd:serviceParam wikibase:language "en". }
            } LIMIT 5000
        """),
    ]

    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for domain, sparql in queries:
            log(f"  domain: {domain}")
            facts = fetch_wikidata_facts(domain, sparql, WIKIDATA_MAX // len(queries))
            for fact in facts:
                # Convert structured bindings to natural language
                domain_label = fact.pop("domain", "")
                fields = [f"{k}: {v}" for k, v in fact.items() if v and not v.startswith("http")]
                if not fields:
                    continue
                block = (
                    f"══ WIKIDATA FACT — {domain_label.upper()} ══\n"
                    + "\n".join(fields)
                )
                write_block(fh, block)
                total += 1
            log(f"    → {len(facts)} facts")
    log(f"WikiData done — {total} facts → {out_path.name}")


# ── main ──────────────────────────────────────────────────────────────────────

def main():
    log("=" * 60)
    log("Stage 13 corpus builder — 'Fruiting Body'")
    log(f"Unlock: loss_avg ≤ 3.0 AND architecture ≥ 40L 4-stream cord")
    log(f"Target: ~100B tokens (~450 GB)")
    log(f"Output: {CORPUS_DIR}")
    log("NOTE: This stage requires HuggingFace datasets and significant")
    log("      disk space (~500 GB). Bizon-class hardware recommended.")
    log("=" * 60)

    tasks = [
        ("arxiv_full_papers.txt",            build_arxiv_full),
        ("internet_archive_books.txt",        build_internet_archive),
        ("uspto_patents.txt",                 build_patents),
        ("wikipedia_multilingual_full.txt",   build_wikipedia_multilingual_full),
        ("instruction_dialogue.txt",          build_instruction_dialogue),
        ("wikidata_facts.txt",               build_wikidata),
    ]

    for fname, builder in tasks:
        out = CORPUS_DIR / fname
        log("")
        log(f"▶ {fname}")
        try:
            builder(out)
        except KeyboardInterrupt:
            log("Interrupted — partial output saved.")
            sys.exit(0)
        except Exception as e:
            log(f"ERROR in {fname}: {e}")
            import traceback; traceback.print_exc()

    log("")
    log("=" * 60)
    log("Stage 13 complete. Run chaos complement next:")
    log("  python scripts/build_chaos_corpus.py --scale stage13")
    total_bytes = sum(
        f.stat().st_size for f in CORPUS_DIR.iterdir() if f.suffix == ".txt"
    )
    log(f"Total: {total_bytes / 1024 / 1024 / 1024:.1f} GB")
    log("=" * 60)

if __name__ == "__main__":
    main()
