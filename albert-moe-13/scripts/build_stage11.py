#!/usr/bin/env python3
"""
Stage 11 corpus builder — "Deep Roots"
Albert MoE-13 training data pipeline

Unlock condition: loss_avg ≤ 7.0 AND cord surgery (dual-stream) complete

Sources (all free, no API keys required unless noted):
  1. Wikipedia multilingual  — RU, ZH, JA, AR, HI, KO via Wikimedia API
  2. arXiv abstracts         — all categories via arXiv API (free)
  3. EUR-Lex legislation     — EU law EN/DE/FR/ES (public domain)
  4. Wikisource              — public domain literary texts (multilingual)
  5. Mathematics SE          — proof-level Q&A via Stack Exchange API
  6. Science SE cluster      — Physics, Chemistry, Biology, CS Theory

Target: ~1.2B tokens (~5 GB text)
Output: data/corpus/stage_11/*.txt

Run:
    cd albert-moe-13
    python scripts/build_stage11.py

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
CORPUS_DIR = SCRIPT_DIR.parent / "data" / "corpus" / "stage_11"
CORPUS_DIR.mkdir(parents=True, exist_ok=True)

UA = "AlbertMoE13CorpusBot/1.1 (stage11 training corpus; contact contact@ternlang.com)"

# ── tuning ────────────────────────────────────────────────────────────────────
WIKI_ARTICLES_PER_LANG  = 5000
ARXIV_MAX_PER_CATEGORY  = 1000
EURLEX_MAX_DOCS         = 2000
WIKISOURCE_MAX_PER_LANG = 500
SE_SCIENCE_MAX_PER_SITE = 300

WIKI_LANGUAGES = [
    ("ru", "Russian"),
    ("zh", "Chinese"),
    ("ja", "Japanese"),
    ("ar", "Arabic"),
    ("hi", "Hindi"),
    ("ko", "Korean"),
]

ARXIV_CATEGORIES = [
    ("cs.AI",    "Artificial Intelligence"),
    ("cs.LG",    "Machine Learning"),
    ("cs.CL",    "Computation and Language"),
    ("math.CO",  "Combinatorics"),
    ("math.NT",  "Number Theory"),
    ("physics.gen-ph", "General Physics"),
    ("q-bio.NC", "Neurons and Cognition"),
    ("econ.TH",  "Economic Theory"),
    ("stat.ML",  "Statistics — ML"),
    ("cond-mat.dis-nn", "Disordered Systems"),
]

SCIENCE_SE_SITES = [
    ("math",          "Mathematics"),
    ("physics",       "Physics"),
    ("chemistry",     "Chemistry"),
    ("biology",       "Biology"),
    ("cs",            "Computer Science"),
    ("cogsci",        "Cognitive Science"),
    ("stats",         "Statistics"),
    ("mathoverflow",  "Mathematics Research"),
    ("philosophy",    "Philosophy"),
    ("linguistics",   "Linguistics"),
]

WIKISOURCE_LANGS = ["en", "de", "fr", "es", "it", "pl", "pt"]

EURLEX_LANGUAGES = ["EN", "DE", "FR", "ES"]
EURLEX_TYPES     = ["REG", "DIR", "DEC"]   # Regulation, Directive, Decision


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
            with urllib.request.urlopen(req, timeout=25) as r:
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
    except Exception as e:
        log(f"  json parse error: {e}")
        return None

_TAG_RE = re.compile(r"<[^>]+>")
_ENT_RE = re.compile(r"&(?:#\d+|#x[\da-fA-F]+|\w+);")
_WS_RE  = re.compile(r"[ \t]{3,}")
_HTML_ENTITIES = {
    "&amp;": "&", "&lt;": "<", "&gt;": ">",
    "&quot;": '"', "&apos;": "'", "&#39;": "'",
    "&nbsp;": " ", "&#160;": " ",
}

def strip_html(html: str) -> str:
    if not html:
        return ""
    for ent, char in _HTML_ENTITIES.items():
        html = html.replace(ent, char)
    html = _TAG_RE.sub(" ", html)
    html = _ENT_RE.sub(" ", html)
    html = _WS_RE.sub(" ", html)
    return html.strip()

def write_block(fh, block: str):
    fh.write(block.strip() + "\n\n" + "─" * 72 + "\n\n")
    fh.flush()


# ── 1. Wikipedia multilingual ─────────────────────────────────────────────────

def fetch_wiki_articles(lang: str, limit: int) -> list[dict]:
    """Fetch random articles via Wikimedia action API."""
    results = []
    seen = set()
    while len(results) < limit:
        url = (
            f"https://{lang}.wikipedia.org/w/api.php"
            f"?action=query&list=random&rnnamespace=0&rnlimit=20&format=json"
        )
        data = fetch_json(url, delay=0.3)
        if not data:
            break
        for page in data.get("query", {}).get("random", []):
            if len(results) >= limit:
                break
            pid = page["id"]
            if pid in seen:
                continue
            seen.add(pid)
            extract_url = (
                f"https://{lang}.wikipedia.org/w/api.php"
                f"?action=query&pageids={pid}&prop=extracts&explaintext=1"
                f"&exsectionformat=plain&format=json"
            )
            ext = fetch_json(extract_url, delay=0.4)
            if not ext:
                continue
            pages = ext.get("query", {}).get("pages", {})
            for p in pages.values():
                text = p.get("extract", "").strip()
                title = p.get("title", "")
                if len(text) > 300:
                    results.append({"title": title, "text": text, "lang": lang})
    return results

def build_wikipedia_multilingual(out_path: Path):
    log("Wikipedia multilingual — fetching articles ...")
    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for lang_code, lang_name in WIKI_LANGUAGES:
            log(f"  language: {lang_name} ({lang_code})")
            articles = fetch_wiki_articles(lang_code, WIKI_ARTICLES_PER_LANG)
            for art in articles:
                block = (
                    f"══ WIKIPEDIA — {lang_name.upper()} ══\n"
                    f"Title: {art['title']}\n"
                    f"Language: {lang_name} ({lang_code})\n\n"
                    f"{art['text'][:4000]}"
                )
                write_block(fh, block)
                total += 1
            log(f"    → {len(articles)} articles")
    log(f"Wikipedia multilingual done — {total} articles → {out_path.name}")


# ── 2. arXiv abstracts ────────────────────────────────────────────────────────

ARXIV_BASE = "https://export.arxiv.org/api/query"

def fetch_arxiv_abstracts(category: str, max_results: int) -> list[dict]:
    results = []
    start = 0
    batch = 100
    while len(results) < max_results:
        url = (
            f"{ARXIV_BASE}?search_query=cat:{category}"
            f"&start={start}&max_results={min(batch, max_results - len(results))}"
            f"&sortBy=submittedDate&sortOrder=descending"
        )
        raw = fetch(url, delay=3.0)   # arXiv asks for 3s between requests
        if not raw:
            break
        # Parse Atom XML
        text = raw.decode("utf-8", errors="replace")
        entries = re.findall(r"<entry>(.*?)</entry>", text, re.DOTALL)
        if not entries:
            break
        for entry in entries:
            title   = re.search(r"<title>(.*?)</title>", entry, re.DOTALL)
            summary = re.search(r"<summary>(.*?)</summary>", entry, re.DOTALL)
            authors = re.findall(r"<name>(.*?)</name>", entry)
            arxiv_id= re.search(r"<id>.*?/abs/([^<]+)</id>", entry)
            if title and summary:
                results.append({
                    "id":      arxiv_id.group(1).strip() if arxiv_id else "",
                    "title":   title.group(1).strip(),
                    "authors": authors[:5],
                    "summary": summary.group(1).strip(),
                })
        start += len(entries)
        if len(entries) < batch:
            break
    return results

def build_arxiv(out_path: Path):
    log("arXiv abstracts — fetching ...")
    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for cat, label in ARXIV_CATEGORIES:
            log(f"  category: {cat} ({label})")
            papers = fetch_arxiv_abstracts(cat, ARXIV_MAX_PER_CATEGORY)
            for p in papers:
                authors_str = ", ".join(p["authors"]) if p["authors"] else "unknown"
                block = (
                    f"══ ARXIV ABSTRACT — {label.upper()} ══\n"
                    f"ID      : {p['id']}\n"
                    f"Title   : {p['title']}\n"
                    f"Authors : {authors_str}\n"
                    f"Category: {cat}\n\n"
                    f"[ABSTRACT]\n{p['summary']}"
                )
                write_block(fh, block)
                total += 1
            log(f"    → {len(papers)} papers")
    log(f"arXiv abstracts done — {total} total → {out_path.name}")


# ── 3. EUR-Lex EU legislation ─────────────────────────────────────────────────

EURLEX_SPARQL = "https://publications.europa.eu/webapi/rdf/sparql"

def fetch_eurlex_docs(doc_type: str, language: str, limit: int) -> list[dict]:
    """Fetch EU legislative document titles + summaries via SPARQL."""
    query = f"""
    PREFIX cdm: <http://publications.europa.eu/ontology/cdm#>
    SELECT ?title ?uri WHERE {{
      ?doc a cdm:{doc_type.lower()} ;
           cdm:work_has_expression ?expr .
      ?expr cdm:expression_uses_language <http://publications.europa.eu/resource/authority/language/{language}> ;
            cdm:expression_title ?title .
      BIND(str(?doc) AS ?uri)
    }} LIMIT {limit}
    """
    url = EURLEX_SPARQL + "?" + urllib.parse.urlencode({
        "query": query, "format": "application/sparql-results+json"
    })
    data = fetch_json(url, delay=1.0)
    if not data:
        return []
    results = []
    for binding in data.get("results", {}).get("bindings", []):
        title = binding.get("title", {}).get("value", "")
        uri   = binding.get("uri", {}).get("value", "")
        if title:
            results.append({"title": title, "uri": uri, "lang": language, "type": doc_type})
    return results

def build_eurlex(out_path: Path):
    log("EUR-Lex legislation — fetching ...")
    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for lang in EURLEX_LANGUAGES:
            for doc_type in EURLEX_TYPES:
                log(f"  type: {doc_type} lang: {lang}")
                docs = fetch_eurlex_docs(doc_type, lang, EURLEX_MAX_DOCS // (len(EURLEX_LANGUAGES) * len(EURLEX_TYPES)))
                type_labels = {"REG": "REGULATION", "DIR": "DIRECTIVE", "DEC": "DECISION"}
                for doc in docs:
                    block = (
                        f"══ EU {type_labels.get(doc_type, doc_type)} — {lang} ══\n"
                        f"Title: {doc['title']}\n"
                        f"Source: EUR-Lex (public domain)\n"
                        f"URI: {doc['uri']}\n\n"
                        f"[LEGISLATIVE TEXT]\n{doc['title']}"
                    )
                    write_block(fh, block)
                    total += 1
                log(f"    → {len(docs)} documents")
    log(f"EUR-Lex done — {total} total → {out_path.name}")


# ── 4. Wikisource public domain texts ────────────────────────────────────────

def fetch_wikisource_texts(lang: str, max_texts: int) -> list[dict]:
    results = []
    seen = set()
    url = (
        f"https://{lang}.wikisource.org/w/api.php"
        f"?action=query&list=allpages&aplimit=50&apnamespace=0&format=json"
    )
    data = fetch_json(url, delay=0.5)
    if not data:
        return results
    pages = data.get("query", {}).get("allpages", [])
    for page in pages[:max_texts]:
        pid = page["pageid"]
        if pid in seen:
            continue
        seen.add(pid)
        ext_url = (
            f"https://{lang}.wikisource.org/w/api.php"
            f"?action=query&pageids={pid}&prop=extracts&explaintext=1&format=json"
        )
        ext = fetch_json(ext_url, delay=0.4)
        if not ext:
            continue
        for p in ext.get("query", {}).get("pages", {}).values():
            text = p.get("extract", "").strip()
            title = p.get("title", "")
            if len(text) > 500:
                results.append({"title": title, "text": text, "lang": lang})
    return results

def build_wikisource(out_path: Path):
    log("Wikisource public domain texts — fetching ...")
    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for lang in WIKISOURCE_LANGS:
            log(f"  lang: {lang}")
            texts = fetch_wikisource_texts(lang, WIKISOURCE_MAX_PER_LANG)
            for t in texts:
                block = (
                    f"══ WIKISOURCE — {lang.upper()} ══\n"
                    f"Title: {t['title']}\n"
                    f"Language: {lang}\n\n"
                    f"{t['text'][:5000]}"
                )
                write_block(fh, block)
                total += 1
            log(f"    → {len(texts)} texts")
    log(f"Wikisource done — {total} total → {out_path.name}")


# ── 5. Science Stack Exchange cluster ─────────────────────────────────────────

SE_BASE = "https://api.stackexchange.com/2.3"

def se_url(path: str, params: dict) -> str:
    params.setdefault("key", "")
    qs = urllib.parse.urlencode({k: v for k, v in params.items() if v != ""})
    return f"{SE_BASE}{path}?{qs}"

def fetch_se_science(site: str, max_q: int) -> list[dict]:
    results = []
    page = 1
    while len(results) < max_q:
        url = se_url("/questions", {
            "site": site, "page": page, "pagesize": 100,
            "sort": "votes", "order": "desc",
            "filter": "withbody", "min": 10,
        })
        data = fetch_json(url, delay=1.2)
        if not data or "items" not in data:
            break
        for q in data["items"]:
            if len(results) >= max_q:
                break
            if not q.get("accepted_answer_id"):
                continue
            # fetch accepted answer
            qid = q["question_id"]
            ans_url = se_url(f"/questions/{qid}/answers", {
                "site": site, "pagesize": 5, "sort": "votes",
                "order": "desc", "filter": "withbody",
            })
            ans_data = fetch_json(ans_url, delay=0.8)
            answers = ans_data.get("items", []) if ans_data else []
            accepted = next(
                (a for a in answers if a["answer_id"] == q["accepted_answer_id"]),
                None
            )
            if accepted:
                results.append({
                    "title":    q.get("title", ""),
                    "body":     strip_html(q.get("body", ""))[:1500],
                    "answer":   strip_html(accepted.get("body", ""))[:2000],
                    "tags":     q.get("tags", []),
                    "score":    q.get("score", 0),
                })
        page += 1
        if page > 10 or not data.get("has_more"):
            break
    return results

def build_science_se(out_path: Path):
    log("Science Stack Exchange cluster — fetching ...")
    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for site, label in SCIENCE_SE_SITES:
            log(f"  site: {site} ({label})")
            items = fetch_se_science(site, SE_SCIENCE_MAX_PER_SITE)
            for q in items:
                tags_str = " | ".join(q["tags"])
                block = (
                    f"══ {label.upper()} — QUESTION & ANSWER ══\n"
                    f"Title : {q['title']}\n"
                    f"Tags  : {tags_str}\n"
                    f"Score : {q['score']}\n\n"
                    f"[QUESTION]\n{q['body']}\n\n"
                    f"[ACCEPTED ANSWER]\n{q['answer']}"
                )
                write_block(fh, block)
                total += 1
            log(f"    → {len(items)} Q&As")
    log(f"Science SE done — {total} total → {out_path.name}")


# ── main ──────────────────────────────────────────────────────────────────────

def main():
    log("=" * 60)
    log("Stage 11 corpus builder — 'Deep Roots'")
    log(f"Unlock: loss_avg ≤ 7.0 AND cord surgery complete")
    log(f"Target: ~1.2B tokens (~5 GB)")
    log(f"Output: {CORPUS_DIR}")
    log("=" * 60)

    tasks = [
        ("wikipedia_multilingual.txt", build_wikipedia_multilingual),
        ("arxiv_abstracts.txt",        build_arxiv),
        ("eurlex_legislation.txt",     build_eurlex),
        ("wikisource_texts.txt",       build_wikisource),
        ("science_stackexchange.txt",  build_science_se),
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
    log("Stage 11 complete. Run chaos complement next:")
    log("  python scripts/build_chaos_corpus.py --scale stage11")
    log("")
    total_bytes = sum(
        f.stat().st_size for f in CORPUS_DIR.iterdir() if f.suffix == ".txt"
    )
    log(f"Total so far: {total_bytes / 1024 / 1024 / 1024:.2f} GB")
    log("=" * 60)

if __name__ == "__main__":
    main()
