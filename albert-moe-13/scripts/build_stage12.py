#!/usr/bin/env python3
"""
Stage 12 corpus builder — "Mycelial Reach"
Albert MoE-13 training data pipeline

Unlock condition: loss_avg ≤ 5.0 AND architecture ≥ 25L dual-stream

Sources:
  1. The Stack (Python, Rust, TypeScript, C, Go) — code via HuggingFace datasets
  2. PubMed abstracts          — biomedical via NCBI E-utilities (free)
  3. CourtListener             — US court opinions (public domain)
  4. CrossRef academic         — multi-domain paper abstracts
  5. EU case law (EUR-Lex)     — legal reasoning EN/DE/FR
  6. Wikipedia EN full         — complete English articles

Target: ~10B tokens (~45 GB text)
Output: data/corpus/stage_12/*.txt

Run:
    cd albert-moe-13
    python scripts/build_stage12.py

    # For The Stack (large — requires huggingface_hub):
    pip install huggingface_hub datasets
    python scripts/build_stage12.py --with-stack

See CORPUS_EXPANSION_ROADMAP.md for full stage context.
"""

import argparse
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
CORPUS_DIR = SCRIPT_DIR.parent / "data" / "corpus" / "stage_12"
CORPUS_DIR.mkdir(parents=True, exist_ok=True)

UA = "AlbertMoE13CorpusBot/1.1 (stage12 training corpus; contact contact@ternlang.com)"

# ── tuning ────────────────────────────────────────────────────────────────────
PUBMED_MAX              = 50_000      # abstracts
COURTLISTENER_MAX       = 5_000       # opinions
CROSSREF_MAX_PER_DOMAIN = 2_000       # abstracts
WIKI_EN_MAX             = 100_000     # full articles
STACK_MAX_PER_LANG      = 50_000      # code files

STACK_LANGUAGES = ["python", "rust", "typescript", "c", "go", "cpp", "java"]

CROSSREF_QUERIES = [
    ("machine learning",     "Artificial Intelligence"),
    ("climate change",       "Earth Sciences"),
    ("neuroscience",         "Brain & Cognition"),
    ("quantum mechanics",    "Physics"),
    ("evolutionary biology", "Biology"),
    ("economics game theory","Economics"),
    ("materials science",    "Materials"),
    ("astrophysics",         "Space Science"),
]

EURLEX_CASE_LAW_TYPES = ["JUDG", "ORD", "OPIN"]


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
_WS_RE  = re.compile(r"[ \t]{3,}")
_HTML_ENTITIES = {
    "&amp;": "&", "&lt;": "<", "&gt;": ">",
    "&quot;": '"', "&apos;": "'", "&nbsp;": " ",
}

def strip_html(html: str) -> str:
    if not html:
        return ""
    for ent, char in _HTML_ENTITIES.items():
        html = html.replace(ent, char)
    html = _TAG_RE.sub(" ", html)
    html = _WS_RE.sub(" ", html)
    return html.strip()

def write_block(fh, block: str):
    fh.write(block.strip() + "\n\n" + "─" * 72 + "\n\n")
    fh.flush()


# ── 1. PubMed abstracts ───────────────────────────────────────────────────────

NCBI_BASE   = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils"
NCBI_EMAIL  = "contact@ternlang.com"

def fetch_pubmed_ids(query: str, max_results: int) -> list[str]:
    url = (
        f"{NCBI_BASE}/esearch.fcgi?db=pubmed&term={urllib.parse.quote(query)}"
        f"&retmax={max_results}&retmode=json&email={NCBI_EMAIL}"
    )
    data = fetch_json(url, delay=0.4)
    if not data:
        return []
    return data.get("esearchresult", {}).get("idlist", [])

def fetch_pubmed_abstracts(ids: list[str]) -> list[dict]:
    results = []
    batch_size = 100
    for i in range(0, len(ids), batch_size):
        batch = ids[i:i + batch_size]
        id_str = ",".join(batch)
        url = (
            f"{NCBI_BASE}/efetch.fcgi?db=pubmed&id={id_str}"
            f"&rettype=abstract&retmode=xml&email={NCBI_EMAIL}"
        )
        raw = fetch(url, delay=0.4)
        if not raw:
            continue
        text = raw.decode("utf-8", errors="replace")
        # Extract article data from XML
        articles = re.findall(r"<PubmedArticle>(.*?)</PubmedArticle>", text, re.DOTALL)
        for art in articles:
            title_m    = re.search(r"<ArticleTitle>(.*?)</ArticleTitle>", art, re.DOTALL)
            abstract_m = re.search(r"<AbstractText[^>]*>(.*?)</AbstractText>", art, re.DOTALL)
            journal_m  = re.search(r"<Title>(.*?)</Title>", art, re.DOTALL)
            year_m     = re.search(r"<Year>(\d{4})</Year>", art)
            if title_m and abstract_m:
                results.append({
                    "title":    strip_html(title_m.group(1)),
                    "abstract": strip_html(abstract_m.group(1)),
                    "journal":  strip_html(journal_m.group(1)) if journal_m else "",
                    "year":     year_m.group(1) if year_m else "",
                })
    return results

def build_pubmed(out_path: Path):
    log("PubMed abstracts — fetching ...")
    queries = [
        "machine learning neural network",
        "CRISPR gene editing",
        "climate change ecology",
        "neuroscience cognition",
        "protein structure folding",
        "cancer immunotherapy",
        "antibiotic resistance",
        "stem cell regeneration",
        "epidemiology public health",
        "quantum biology",
    ]
    total = 0
    per_query = PUBMED_MAX // len(queries)
    with open(out_path, "w", encoding="utf-8") as fh:
        for query in queries:
            log(f"  query: {query}")
            ids = fetch_pubmed_ids(query, per_query)
            abstracts = fetch_pubmed_abstracts(ids)
            for a in abstracts:
                block = (
                    f"══ PUBMED BIOMEDICAL ABSTRACT ══\n"
                    f"Title   : {a['title']}\n"
                    f"Journal : {a['journal']}\n"
                    f"Year    : {a['year']}\n\n"
                    f"[ABSTRACT]\n{a['abstract']}"
                )
                write_block(fh, block)
                total += 1
            log(f"    → {len(abstracts)} abstracts")
    log(f"PubMed done — {total} total → {out_path.name}")


# ── 2. CourtListener US court opinions ───────────────────────────────────────

CL_BASE = "https://www.courtlistener.com/api/rest/v3"

def fetch_courtlistener(max_opinions: int) -> list[dict]:
    results = []
    url = f"{CL_BASE}/opinions/?format=json&page_size=20&order_by=-date_created"
    while len(results) < max_opinions and url:
        data = fetch_json(url, delay=1.0)
        if not data:
            break
        for op in data.get("results", []):
            text = strip_html(op.get("plain_text") or op.get("html_with_citations") or "")
            if len(text) < 500:
                continue
            results.append({
                "case":   op.get("absolute_url", ""),
                "date":   op.get("date_created", "")[:10],
                "text":   text[:4000],
            })
        url = data.get("next")
        if len(results) >= max_opinions:
            break
    return results

def build_courtlistener(out_path: Path):
    log("CourtListener US court opinions — fetching ...")
    opinions = fetch_courtlistener(COURTLISTENER_MAX)
    with open(out_path, "w", encoding="utf-8") as fh:
        for op in opinions:
            block = (
                f"══ US COURT OPINION ══\n"
                f"Date: {op['date']}\n"
                f"Source: CourtListener (public domain)\n\n"
                f"[LEGAL REASONING]\n{op['text']}"
            )
            write_block(fh, block)
    log(f"CourtListener done — {len(opinions)} opinions → {out_path.name}")


# ── 3. CrossRef academic abstracts ───────────────────────────────────────────

CROSSREF_BASE = "https://api.crossref.org/works"

def fetch_crossref(query: str, max_results: int) -> list[dict]:
    results = []
    offset = 0
    rows = 100
    while len(results) < max_results:
        url = (
            f"{CROSSREF_BASE}?query={urllib.parse.quote(query)}"
            f"&rows={min(rows, max_results - len(results))}"
            f"&offset={offset}&select=title,abstract,author,published,DOI"
            f"&mailto={NCBI_EMAIL}"
        )
        data = fetch_json(url, delay=0.8)
        if not data:
            break
        items = data.get("message", {}).get("items", [])
        if not items:
            break
        for item in items:
            abstract = item.get("abstract", "")
            if not abstract or len(abstract) < 100:
                continue
            title = " ".join(item.get("title", [""])[:1])
            authors = item.get("author", [])
            author_str = ", ".join(
                f"{a.get('given','')} {a.get('family','')}".strip()
                for a in authors[:3]
            )
            pub = item.get("published", {}).get("date-parts", [[""]])[0]
            year = str(pub[0]) if pub else ""
            results.append({
                "title":    strip_html(title),
                "authors":  author_str,
                "year":     year,
                "doi":      item.get("DOI", ""),
                "abstract": strip_html(abstract)[:2000],
            })
        offset += len(items)
        if len(items) < rows:
            break
    return results

def build_crossref(out_path: Path):
    log("CrossRef academic abstracts — fetching ...")
    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for query, domain in CROSSREF_QUERIES:
            log(f"  domain: {domain}")
            papers = fetch_crossref(query, CROSSREF_MAX_PER_DOMAIN)
            for p in papers:
                block = (
                    f"══ ACADEMIC ABSTRACT — {domain.upper()} ══\n"
                    f"Title   : {p['title']}\n"
                    f"Authors : {p['authors']}\n"
                    f"Year    : {p['year']}\n"
                    f"DOI     : {p['doi']}\n\n"
                    f"[ABSTRACT]\n{p['abstract']}"
                )
                write_block(fh, block)
                total += 1
            log(f"    → {len(papers)} papers")
    log(f"CrossRef done — {total} total → {out_path.name}")


# ── 4. Wikipedia EN full ──────────────────────────────────────────────────────

def fetch_wiki_en_batch(titles_or_random: bool = True, continuation: str | None = None) -> tuple[list[dict], str | None]:
    if titles_or_random:
        url = (
            "https://en.wikipedia.org/w/api.php"
            "?action=query&list=random&rnnamespace=0&rnlimit=20&format=json"
        )
        data = fetch_json(url, delay=0.3)
        if not data:
            return [], None
        pids = [str(p["id"]) for p in data.get("query", {}).get("random", [])]
    else:
        return [], None

    if not pids:
        return [], None

    ext_url = (
        f"https://en.wikipedia.org/w/api.php"
        f"?action=query&pageids={'|'.join(pids)}"
        f"&prop=extracts&explaintext=1&exsectionformat=plain&format=json"
    )
    ext = fetch_json(ext_url, delay=0.4)
    if not ext:
        return [], None

    results = []
    for p in ext.get("query", {}).get("pages", {}).values():
        text = p.get("extract", "").strip()
        if len(text) > 400:
            results.append({"title": p.get("title", ""), "text": text})
    return results, None

def build_wikipedia_en(out_path: Path):
    log("Wikipedia EN full — fetching ...")
    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        while total < WIKI_EN_MAX:
            batch, _ = fetch_wiki_en_batch()
            for art in batch:
                block = (
                    f"══ WIKIPEDIA ENGLISH ══\n"
                    f"Title: {art['title']}\n\n"
                    f"{art['text'][:6000]}"
                )
                write_block(fh, block)
                total += 1
                if total >= WIKI_EN_MAX:
                    break
            if total % 1000 == 0:
                log(f"  {total} articles ...")
    log(f"Wikipedia EN done — {total} articles → {out_path.name}")


# ── 5. The Stack (code) ───────────────────────────────────────────────────────

def build_the_stack(out_path: Path):
    """
    Fetch code samples from The Stack via HuggingFace datasets streaming.
    Requires: pip install datasets huggingface_hub
    Falls back to GitHub search API if datasets not available.
    """
    log("The Stack (code) — attempting HuggingFace datasets ...")
    try:
        from datasets import load_dataset  # type: ignore
    except ImportError:
        log("  datasets not installed — skipping The Stack (run: pip install datasets)")
        log("  Falling back to GitHub search API for code samples ...")
        build_code_github_fallback(out_path)
        return

    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for lang in STACK_LANGUAGES:
            log(f"  language: {lang}")
            lang_count = 0
            try:
                ds = load_dataset(
                    "bigcode/the-stack",
                    data_dir=f"data/{lang}",
                    split="train",
                    streaming=True,
                    trust_remote_code=True,
                )
                for sample in ds:
                    if lang_count >= STACK_MAX_PER_LANG:
                        break
                    content = sample.get("content", "")
                    if len(content) < 100 or len(content) > 10000:
                        continue
                    repo = sample.get("max_stars_repo_name", "unknown")
                    path = sample.get("max_stars_repo_path", "")
                    block = (
                        f"══ SOURCE CODE — {lang.upper()} ══\n"
                        f"Repository : {repo}\n"
                        f"Path       : {path}\n"
                        f"Language   : {lang}\n\n"
                        f"[CODE]\n{content}"
                    )
                    write_block(fh, block)
                    lang_count += 1
                    total += 1
            except Exception as e:
                log(f"  error loading {lang}: {e}")
            log(f"    → {lang_count} files")
    log(f"The Stack done — {total} files → {out_path.name}")

def build_code_github_fallback(out_path: Path):
    """Fallback: GitHub Search API code samples (60 req/hour without token)."""
    GH_TOKEN = os.getenv("GITHUB_TOKEN", "")
    headers  = {"Accept": "application/vnd.github+json", "User-Agent": UA}
    if GH_TOKEN:
        headers["Authorization"] = f"token {GH_TOKEN}"

    queries = [
        ("language:python stars:>100", "Python"),
        ("language:rust stars:>50",    "Rust"),
        ("language:go stars:>100",     "Go"),
        ("language:c stars:>200",      "C"),
    ]

    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for query, lang_label in queries:
            url = (
                f"https://api.github.com/search/code"
                f"?q={urllib.parse.quote(query)}&per_page=30"
            )
            try:
                raw = fetch(url, headers=headers, delay=2.0)
                if not raw:
                    continue
                data = json.loads(raw)
                for item in data.get("items", [])[:30]:
                    raw_url = item.get("git_url", "").replace(
                        "api.github.com/repos", "raw.githubusercontent.com"
                    ).replace("/git/blobs/", "/")
                    if not raw_url:
                        continue
                    code = fetch(raw_url, delay=1.0, headers=headers)
                    if not code:
                        continue
                    text = code.decode("utf-8", errors="replace")[:5000]
                    block = (
                        f"══ SOURCE CODE — {lang_label.upper()} ══\n"
                        f"Repository: {item.get('repository', {}).get('full_name', '')}\n"
                        f"Path: {item.get('path', '')}\n\n"
                        f"[CODE]\n{text}"
                    )
                    write_block(fh, block)
                    total += 1
            except Exception as e:
                log(f"  GitHub fallback error ({lang_label}): {e}")
    log(f"Code fallback done — {total} files → {out_path.name}")


# ── main ──────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description="Stage 12 corpus builder")
    parser.add_argument("--with-stack", action="store_true",
                        help="Include The Stack via HuggingFace datasets (requires datasets lib)")
    parser.add_argument("--skip-wiki", action="store_true",
                        help="Skip Wikipedia EN (large, takes several hours)")
    args = parser.parse_args()

    log("=" * 60)
    log("Stage 12 corpus builder — 'Mycelial Reach'")
    log(f"Unlock: loss_avg ≤ 5.0 AND architecture ≥ 25L dual-stream")
    log(f"Target: ~10B tokens (~45 GB)")
    log(f"Output: {CORPUS_DIR}")
    log("=" * 60)

    tasks = [
        ("pubmed_abstracts.txt",       build_pubmed),
        ("courtlistener_opinions.txt", build_courtlistener),
        ("crossref_abstracts.txt",     build_crossref),
    ]

    if not args.skip_wiki:
        tasks.append(("wikipedia_en_full.txt", build_wikipedia_en))

    if args.with_stack:
        tasks.append(("the_stack_code.txt", build_the_stack))
    else:
        log("  (skipping The Stack — pass --with-stack to include)")
        tasks.append(("code_github_samples.txt", build_code_github_fallback))

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
    log("Stage 12 complete. Run chaos complement next:")
    log("  python scripts/build_chaos_corpus.py --scale stage12")
    total_bytes = sum(
        f.stat().st_size for f in CORPUS_DIR.iterdir() if f.suffix == ".txt"
    )
    log(f"Total: {total_bytes / 1024 / 1024 / 1024:.2f} GB")
    log("=" * 60)

if __name__ == "__main__":
    main()
