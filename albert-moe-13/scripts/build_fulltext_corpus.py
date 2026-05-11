"""
build_fulltext_corpus.py
Builds the full-paper academic layer for Albert v3.0 tokenizer training.

Unlike build_academic_corpus.py (abstracts only), this script fetches
complete papers with section structure: Introduction, Methodology/Methods,
Results, Discussion, Conclusion. This teaches the tokenizer — and later
the model — what good academic argument structure looks like across
12 disciplines and 8 languages.

Sources:
  ar5iv.org         → English full papers (arXiv LaTeX → clean HTML, all sections)
  SciELO JATS XML   → Spanish + Portuguese full papers
  HAL TEI XML       → French full papers (papers that have deposited full text)
  PMC OAI XML       → Biomedical English (JATS format, all sections)
  DOAJ              → Italian, Dutch, Polish (open-access journals, abstracts+)

Output: data/corpus/fulltext/{lang}.txt

Usage:
    python3 scripts/build_fulltext_corpus.py
    python3 scripts/build_fulltext_corpus.py --lang en fr
    python3 scripts/build_fulltext_corpus.py --lang en --target-mb 40
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
OUT_DIR    = os.path.join(REPO_ROOT, "data", "corpus", "fulltext")

DEFAULT_TARGET_MB = 30
UA = "AlbertV3FulltextBuilder/1.0 (rfi.irfos@gmail.com)"

ALL_LANGS = ["en", "fr", "es", "pt", "de", "it", "nl", "pl"]

# arXiv discipline sets — ordered by expected text richness
ARXIV_SETS = ["cs", "math", "physics", "q-bio", "econ", "stat", "eess"]


# ── HTTP ──────────────────────────────────────────────────────────────────────

def fetch(url: str, params: dict = None, retries: int = 4, delay: float = 3.0) -> bytes:
    if params:
        url = url + "?" + urllib.parse.urlencode(params)
    for attempt in range(retries):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": UA})
            with urllib.request.urlopen(req, timeout=45) as r:
                return r.read()
        except (urllib.error.URLError, OSError) as e:
            if attempt == retries - 1:
                raise
            time.sleep(delay * (attempt + 1))


# ── Text cleaning ─────────────────────────────────────────────────────────────

def clean_text(text: str) -> str:
    text = re.sub(r"\\[a-zA-Z]+\{[^}]*\}", " ", text)   # LaTeX commands
    text = re.sub(r"\\[a-zA-Z]+", " ", text)
    text = re.sub(r"<[^>]+>", " ", text)                 # HTML/XML tags
    text = re.sub(r"https?://\S+", " ", text)            # URLs
    text = re.sub(r"\b10\.\d{4,}/\S+", " ", text)       # DOIs
    text = text.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">")
    text = text.replace("&quot;", '"').replace("&nbsp;", " ")
    text = re.sub(r"&\w+;", " ", text)
    text = re.sub(r"[ \t]+", " ", text)
    text = re.sub(r"\n{3,}", "\n\n", text)
    return text.strip()


def strip_html_to_text(raw: bytes | str) -> str:
    """Strip HTML to plain text preserving paragraph and heading breaks."""
    if isinstance(raw, bytes):
        text = raw.decode("utf-8", errors="replace")
    else:
        text = raw
    # Remove noisy elements entirely
    for tag in ["script", "style", "math", "figure", "table", "nav",
                "header", "footer", "aside", "cite", "sup", "sub",
                "annotation", "semantics"]:
        text = re.sub(f"<{tag}[^>]*>.*?</{tag}>", " ",
                      text, flags=re.DOTALL | re.IGNORECASE)
    # Keep section headings as structural markers
    text = re.sub(r"<h[1-6][^>]*>(.*?)</h[1-6]>", r"\n\n\1\n\n",
                  text, flags=re.DOTALL | re.IGNORECASE)
    # Paragraph breaks
    text = re.sub(r"</p>|</div>|<br\s*/?>", "\n", text, flags=re.IGNORECASE)
    # Strip remaining tags
    text = re.sub(r"<[^>]+>", " ", text)
    return clean_text(text)


# ── OAI-PMH helpers ───────────────────────────────────────────────────────────

OAI_NS = {
    "oai":    "http://www.openarchives.org/OAI/2.0/",
    "dc":     "http://purl.org/dc/elements/1.1/",
    "oai_dc": "http://www.openarchives.org/OAI/2.0/oai_dc/",
}


def oai_list_records(base_url: str, params: dict, wall_timeout: float = 0.0):
    """Generator: yield (identifier, ET.Element metadata) from OAI-PMH.
    wall_timeout: stop after this many seconds (0 = unlimited)."""
    resumption = None
    start_t = time.time()
    while True:
        if wall_timeout > 0 and time.time() - start_t > wall_timeout:
            print(f"    oai_list_records: wall timeout {wall_timeout:.0f}s reached")
            return
        if resumption:
            p = {"verb": "ListRecords", "resumptionToken": resumption}
        else:
            p = params.copy()
        try:
            raw  = fetch(base_url, p)
            root = ET.fromstring(raw)
        except ET.ParseError as e:
            print(f"    OAI XML parse error: {e} — skipping batch")
            break

        lr = root.find("oai:ListRecords", OAI_NS)
        if lr is None:
            break

        for record in lr.findall("oai:record", OAI_NS):
            header = record.find("oai:header", OAI_NS)
            meta   = record.find("oai:metadata", OAI_NS)
            if header is None or meta is None:
                continue
            id_el = header.find("oai:identifier", OAI_NS)
            identifier = id_el.text.strip() if id_el is not None and id_el.text else ""
            yield identifier, meta

        rt = lr.find("oai:resumptionToken", OAI_NS)
        if rt is not None and rt.text and rt.text.strip():
            resumption = rt.text.strip()
            time.sleep(5.0)   # respect rate limits
        else:
            break


# ── English: arXiv via ar5iv ─────────────────────────────────────────────────

def get_arxiv_ids(discipline: str, count: int) -> list[str]:
    """Harvest arXiv IDs for a discipline from OAI-PMH."""
    ids = []
    params = {
        "verb":           "ListRecords",
        "metadataPrefix": "oai_dc",
        "set":            discipline,
    }
    for identifier, _ in oai_list_records("https://export.arxiv.org/oai2", params):
        # oai:arXiv.org:2301.12345 → 2301.12345
        arxiv_id = identifier.replace("oai:arXiv.org:", "").strip()
        if arxiv_id:
            ids.append(arxiv_id)
        if len(ids) >= count:
            break
    return ids


def fetch_ar5iv(arxiv_id: str) -> str:
    """Fetch full paper text from ar5iv.org (arXiv HTML mirror)."""
    url = f"https://ar5iv.org/html/{arxiv_id}"
    try:
        html = fetch(url, retries=2, delay=2.0)
    except Exception:
        return ""
    text = strip_html_to_text(html)
    # ar5iv error pages are short
    if len(text) < 800:
        return ""
    # Trim boilerplate: everything before "Abstract" or first real section
    m = re.search(r"\b(Abstract|Introduction)\b", text, re.IGNORECASE)
    if m:
        text = text[m.start():]
    return text


def harvest_en_fulltext(target_bytes: int) -> list[str]:
    texts  = []
    written = 0
    ids_per_set = max(30, target_bytes // (len(ARXIV_SETS) * 6000))  # ~6KB/paper

    for discipline in ARXIV_SETS:
        if written >= target_bytes:
            break
        print(f"  [en] ar5iv discipline={discipline} — harvesting IDs...")
        try:
            ids = get_arxiv_ids(discipline, ids_per_set)
        except Exception as e:
            print(f"  [en] {discipline}: ID harvest ERROR — {e}")
            continue
        print(f"  [en] {discipline}: {len(ids)} IDs, fetching full text...")
        for arxiv_id in ids:
            if written >= target_bytes:
                break
            text = fetch_ar5iv(arxiv_id)
            if text:
                texts.append(text)
                written += len(text.encode())
            time.sleep(0.5)   # polite crawl rate
        print(f"  [en] {discipline}: done · {written/1024/1024:.1f}MB total")

    return texts


# ── English: PMC biomedical full text ────────────────────────────────────────

PMC_NS = {
    "": "https://jats.nlm.nih.gov/ns/archiving/1.3/",
}

def extract_jats_sections(xml_bytes: bytes) -> str:
    """Extract section text from JATS XML (PMC / SciELO format)."""
    try:
        root = ET.fromstring(xml_bytes)
    except ET.ParseError:
        return ""
    parts = []
    # Walk all <sec> elements regardless of namespace
    raw = xml_bytes.decode("utf-8", errors="replace")
    # Strip XML tags keeping text and section titles
    raw = re.sub(r"<sec[^>]*>|</sec>", "\n\n", raw, flags=re.IGNORECASE)
    raw = re.sub(r"<title>(.*?)</title>", r"\n\n\1\n\n", raw, flags=re.DOTALL)
    raw = re.sub(r"<p>(.*?)</p>", r"\1\n", raw, flags=re.DOTALL)
    return clean_text(raw)


def harvest_en_pmc(target_bytes: int) -> list[str]:
    """Harvest biomedical full text from PubMed Central OAI-PMH."""
    base   = "https://www.ncbi.nlm.nih.gov/pmc/oai/oai.cgi"
    params = {
        "verb":           "ListRecords",
        "metadataPrefix": "pmc",
        "set":            "pmc-open",
    }
    texts   = []
    written = 0
    print("  [en] PMC biomedical full text...")
    for identifier, meta in oai_list_records(base, params):
        if written >= target_bytes:
            break
        # PMC OAI with pmc format returns inline full text in the metadata
        xml_str = ET.tostring(meta, encoding="unicode")
        text = extract_jats_sections(xml_str.encode())
        if len(text) > 500:
            texts.append(text)
            written += len(text.encode())
    print(f"  [en] PMC: {len(texts)} papers · {written/1024/1024:.1f}MB")
    return texts


# ── French: HAL (corrected) + PERSEE ────────────────────────────────────────

def harvest_fr_persee(target_bytes: int) -> list[str]:
    """
    Harvest French full text from PERSEE — portail de revues scientifiques
    françaises. OAI-PMH for article identifiers, HTML fetch for full text.
    Covers: history, philosophy, linguistics, law, medicine, natural sciences.
    """
    base  = "https://oai.persee.fr/api/openaire/oai"
    texts = []
    written = 0
    params = {"verb": "ListRecords", "metadataPrefix": "oai_dc"}

    print("  [fr] PERSEE (oai.persee.fr) ...")
    for identifier, _ in oai_list_records(base, params):
        if written >= target_bytes:
            break
        # oai:persee.fr:pharm_0014-2956_1960_num_36_155_2055 → strip prefix
        doc_id = identifier.replace("oai:persee.fr:", "").strip()
        if not doc_id:
            continue
        url = f"https://www.persee.fr/doc/{doc_id}"
        try:
            html = fetch(url, retries=2, delay=1.0)
            text = strip_html_to_text(html)
            if len(text) > 500:
                texts.append(text)
                written += len(text.encode())
        except Exception:
            continue
        time.sleep(0.3)

    print(f"  [fr] PERSEE: {len(texts)} articles · {written/1024/1024:.1f}MB")
    return texts


def harvest_fr_fulltext(target_bytes: int) -> list[str]:
    """
    Harvest French academic text from two sources:
    1. HAL — 599k French journal articles (title + abstract + keywords).
       No individual file fetching — HAL metadata alone gives rich French
       academic prose across all disciplines at high throughput.
    2. PERSEE — French national scientific archive, full HTML article pages.
       Covers: history, philosophy, linguistics, law, medicine, sciences.
    """
    import json
    texts   = []
    written = 0

    # ── HAL: 599k French journal articles, metadata only (fast + reliable) ───
    print("  [fr] HAL (599k French journal articles — title+abstract) ...")
    base_hal   = "https://api.archives-ouvertes.fr/search/"
    start      = 0
    rows       = 500
    hal_target = target_bytes // 2

    while written < hal_target:
        try:
            params = {
                "q":    "language_s:fr",
                "fq":   "docType_s:ART",
                "wt":   "json",
                "fl":   "title_s,abstract_s,keyword_s",
                "rows": rows,
                "start": start,
                "sort": "submittedDate_tdate desc",
            }
            raw  = fetch(base_hal, params)
            data = json.loads(raw)
            docs = data.get("response", {}).get("docs", [])
            if not docs:
                break

            batch_written = 0
            for doc in docs:
                parts = []
                for key in ("title_s", "abstract_s", "keyword_s"):
                    val = doc.get(key, [])
                    parts.extend(val if isinstance(val, list) else ([val] if val else []))
                block = clean_text(" ".join(p for p in parts if p))
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

    # ── PERSEE: full HTML articles for structural variety ────────────────────
    if written < target_bytes:
        print(f"  [fr] PERSEE (target: {(target_bytes-written)//1024//1024}MB more) ...")
        try:
            persee = harvest_fr_persee(target_bytes - written)
            texts.extend(persee)
            written = sum(len(t.encode()) for t in texts)
        except Exception as e:
            print(f"  [fr] PERSEE: ERROR — {e} — continuing with HAL data only")

    return texts


# ── Spanish + Portuguese: SciELO JATS XML ────────────────────────────────────

SCIELO_COLLECTIONS = {
    "es": ["chl", "col", "esp", "mex", "per", "ven", "arg"],
    "pt": ["scl", "prt"],
}


def harvest_scielo_fulltext(lang: str, target_bytes: int) -> list[str]:
    """Fetch full JATS XML papers from SciELO."""
    import json
    colls  = SCIELO_COLLECTIONS.get(lang, [])
    texts  = []
    written = 0

    for coll in colls:
        if written >= target_bytes:
            break
        offset = 0
        limit  = 50
        print(f"  [{lang}] SciELO full text collection={coll} ...")
        while written < target_bytes:
            try:
                id_url = (f"https://articlemeta.scielo.org/api/v1/article/identifiers/"
                          f"?collection={coll}&limit={limit}&offset={offset}")
                raw  = fetch(id_url)
                data = json.loads(raw)
                pids = [item.get("code", "") for item in data.get("objects", [])]
                if not pids:
                    break
                for pid in pids:
                    if written >= target_bytes:
                        break
                    try:
                        art_url = (f"https://articlemeta.scielo.org/api/v1/article/"
                                   f"?collection={coll}&pid={pid}&format=xmlpmc")
                        xml_bytes = fetch(art_url, retries=2, delay=2.0)
                        text = extract_jats_sections(xml_bytes)
                        if len(text) > 500:
                            texts.append(text)
                            written += len(text.encode())
                    except Exception:
                        continue
                    time.sleep(0.15)
                offset += limit
                time.sleep(0.5)
            except Exception as e:
                print(f"  [{lang}] SciELO {coll}: ERROR — {e}")
                break
        print(f"  [{lang}] {coll}: {len(texts)} papers · {written/1024/1024:.1f}MB total")

    return texts


# ── German + others: DOAJ full-text links ────────────────────────────────────

DOAJ_LANG_CODES = {
    "de": "German",
    "it": "Italian",
    "nl": "Dutch",
    "pl": "Polish",
}


def harvest_doaj_fulltext(lang: str, target_bytes: int) -> list[str]:
    """
    Fetch full-text articles via DOAJ API.
    DOAJ provides links to publisher HTML; we fetch and strip.
    Language names for DOAJ filter (they use English language names).
    """
    import json
    lang_name = DOAJ_LANG_CODES.get(lang, lang)
    base   = "https://doaj.org/api/articles"
    texts  = []
    written = 0
    page   = 1
    page_size = 100

    print(f"  [{lang}] DOAJ full text (language={lang_name}) ...")
    while written < target_bytes:
        try:
            params = {
                "search_type":  "query",
                "search_query": f"index.language:{lang_name}",
                "page":         page,
                "pageSize":     page_size,
            }
            raw  = fetch(base, params)
            data = json.loads(raw)
            results = data.get("results", [])
            if not results:
                break
            for article in results:
                if written >= target_bytes:
                    break
                # Try to get fulltext link
                link = article.get("bibjson", {}).get("link", [])
                fulltext_url = None
                for lnk in link:
                    if lnk.get("type") == "fulltext" and lnk.get("url"):
                        fulltext_url = lnk["url"]
                        break
                if not fulltext_url:
                    # Fall back to abstract
                    abstract = article.get("bibjson", {}).get("abstract", "")
                    title    = article.get("bibjson", {}).get("title", "")
                    block = clean_text(f"{title} {abstract}")
                    if len(block) > 80:
                        texts.append(block)
                        written += len(block.encode())
                    continue
                # Fetch full text HTML
                try:
                    html = fetch(fulltext_url, retries=2, delay=2.0)
                    text = strip_html_to_text(html)
                    if len(text) > 500:
                        texts.append(text)
                        written += len(text.encode())
                except Exception:
                    pass
                time.sleep(0.2)
            page += 1
            time.sleep(1.0)
        except Exception as e:
            print(f"  [{lang}] DOAJ: ERROR — {e}")
            break

    print(f"  [{lang}] DOAJ: {len(texts)} articles · {written/1024/1024:.1f}MB")
    return texts


# ── HAL generic harvester (any language, fulltext layer) ─────────────────────

def harvest_hal_lang_fulltext(lang: str, target_bytes: int) -> list[str]:
    """
    Harvest HAL academic text for any language — title + abstract + keywords.
    Used for ES/PT/DE/IT/NL/PL where dedicated full-text sources are unavailable.
    HAL coverage: es≈11k, it≈5.8k, de≈3.6k, pt≈3.5k, nl≈196, pl≈271.
    """
    import json
    base  = "https://api.archives-ouvertes.fr/search/"
    texts = []
    written = 0
    start  = 0
    rows   = 500

    print(f"  [{lang}] HAL fulltext layer (language_s:{lang}) ...")
    while written < target_bytes:
        try:
            params = {
                "q":    f"language_s:{lang}",
                "fq":   "docType_s:ART",
                "wt":   "json",
                "fl":   "title_s,abstract_s,keyword_s",
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
                for key in ("title_s", "abstract_s", "keyword_s"):
                    val = doc.get(key, [])
                    parts.extend(val if isinstance(val, list) else ([val] if val else []))
                block = clean_text(" ".join(p for p in parts if p))
                if len(block) > 80:
                    texts.append(block)
                    written      += len(block.encode())
                    batch_written += len(block.encode())
            print(f"  [{lang}] HAL page {start//rows+1}: +{len(docs)} docs "
                  f"(+{batch_written//1024}KB) · {written/1024/1024:.1f}MB total")
            start += rows
            time.sleep(1.0)
        except Exception as e:
            print(f"  [{lang}] HAL: ERROR — {e}")
            break

    print(f"  [{lang}] HAL done: {len(texts)} docs · {written/1024/1024:.1f}MB")
    return texts


def harvest_zenodo_lang_fulltext(lang_code: str, target_bytes: int) -> list[str]:
    """Zenodo OAI fallback for languages with insufficient HAL coverage."""
    base = "https://zenodo.org/oai2d"
    zenodo_sets = ["openaire", "user-zenodo"]
    texts = []
    written = 0

    for s in zenodo_sets:
        if written >= target_bytes:
            break
        print(f"  [{lang_code}] Zenodo set={s} lang={lang_code} ...")
        try:
            per_set = max(500, (target_bytes - written) // 350)
            for _, meta in oai_list_records(base, {
                "verb": "ListRecords", "metadataPrefix": "oai_dc", "set": s
            }, wall_timeout=90.0):
                if written >= target_bytes:
                    break
                dc = meta.find("oai_dc:dc", OAI_NS)
                if dc is None:
                    continue
                lang_els = dc.findall("dc:language", OAI_NS)
                langs = [l.text or "" for l in lang_els]
                if not any(lang_code.lower() in lg.lower() for lg in langs):
                    continue
                titles = [e.text or "" for e in dc.findall("dc:title", OAI_NS)]
                descs  = [e.text or "" for e in dc.findall("dc:description", OAI_NS)]
                block = clean_text(" ".join(t.strip() for t in titles + descs if t.strip()))
                if len(block) > 80:
                    texts.append(block)
                    written += len(block.encode())
                if len(texts) >= per_set:
                    break
            print(f"  [{lang_code}] Zenodo {s}: +{len(texts)} · {written/1024/1024:.1f}MB")
        except Exception as e:
            print(f"  [{lang_code}] Zenodo {s}: ERROR — {e}")
        time.sleep(2.0)

    return texts


# ── Dispatch ──────────────────────────────────────────────────────────────────

def build_language(lang: str, out_path: str, target_bytes: int) -> None:
    start = time.time()
    texts = []

    if lang == "en":
        # Primary: ar5iv full papers (structured sections)
        texts = harvest_en_fulltext(target_bytes * 3 // 4)
        written = sum(len(t.encode()) for t in texts)
        # Supplement with PMC biomedical
        if written < target_bytes:
            texts += harvest_en_pmc(target_bytes - written)

    elif lang == "fr":
        texts = harvest_fr_fulltext(target_bytes)

    else:
        # es, pt, de, it, nl, pl — HAL primary, Zenodo supplement
        texts = harvest_hal_lang_fulltext(lang, target_bytes)
        written = sum(len(t.encode()) for t in texts)
        if written < target_bytes:
            texts += harvest_zenodo_lang_fulltext(lang, target_bytes - written)

    written = sum(len(t.encode()) for t in texts)
    elapsed = time.time() - start

    if not texts:
        print(f"[{lang}] WARNING: No full text harvested")
        return

    with open(out_path, "w", encoding="utf-8") as out:
        for t in texts:
            out.write(t + "\n\n")

    print(f"[{lang}] Done: {len(texts):,} papers · {written/1024/1024:.1f}MB · "
          f"{elapsed:.0f}s → {out_path}")


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--lang",      nargs="*", default=ALL_LANGS)
    parser.add_argument("--target-mb", type=int,  default=DEFAULT_TARGET_MB)
    args = parser.parse_args()

    os.makedirs(OUT_DIR, exist_ok=True)
    target_bytes = args.target_mb * 1024 * 1024

    for lang in args.lang:
        if lang not in ALL_LANGS:
            print(f"Unknown language: {lang}")
            continue
        out_path = os.path.join(OUT_DIR, f"{lang}.txt")
        if os.path.exists(out_path) and os.path.getsize(out_path) > 1024 * 1024:
            mb = os.path.getsize(out_path) / 1024 / 1024
            print(f"[{lang}] Already exists ({mb:.1f}MB) — skipping")
            continue
        print(f"\n[{lang}] Full-text corpus (target: {args.target_mb}MB)...")
        try:
            build_language(lang, out_path, target_bytes)
        except Exception as e:
            print(f"[{lang}] ERROR: {e}")
            import traceback; traceback.print_exc()

    print("\n=== Full-Text Corpus Summary ===")
    total = 0
    for lang in ALL_LANGS:
        p = os.path.join(OUT_DIR, f"{lang}.txt")
        if os.path.exists(p):
            mb = os.path.getsize(p) / 1024 / 1024
            total += os.path.getsize(p)
            print(f"  {lang}: {mb:.1f}MB")
    print(f"  TOTAL: {total/1024/1024:.1f}MB")
    print(f"\nFull-text corpus ready at: {OUT_DIR}")
    print("Next: python3 scripts/train_tokenizer_v3.py  (picks up all three corpus layers)")


if __name__ == "__main__":
    main()
