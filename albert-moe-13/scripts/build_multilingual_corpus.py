"""
build_multilingual_corpus.py
Builds the multilingual text corpus for Albert v3.0 tokenizer training.

Sources:
  - Wikipedia first-partition article dumps (bz2, plain XML, streamed)
  - Existing English corpus files from data/corpus/stage_*/

Languages: de, fr, es, pt, it, nl, pl  (+en from existing corpus)
Output:    data/corpus/multilingual/{lang}.txt  (~50-80MB each)

The script auto-discovers the latest dump date and partition filename
per language, so it stays up to date without hardcoded URLs.

Usage:
    python3 scripts/build_multilingual_corpus.py
    python3 scripts/build_multilingual_corpus.py --lang de fr
    python3 scripts/build_multilingual_corpus.py --lang de --target-mb 100
"""

import argparse
import bz2
import os
import re
import sys
import time
import urllib.request

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT  = os.path.dirname(SCRIPT_DIR)
OUT_DIR    = os.path.join(REPO_ROOT, "data", "corpus", "multilingual")

DEFAULT_TARGET_MB = 60
CHUNK_SIZE = 128 * 1024  # 128KB read chunks
UA = "AlbertV3CorpusBuilder/1.0 (contact@ternlang.com)"

LANGUAGES = ["de", "fr", "es", "pt", "it", "nl", "pl"]

WIKIDUMP_BASE = "https://dumps.wikimedia.org"


# ── Wiki markup stripping ──────────────────────────────────────────────────

def strip_wikitext(text: str) -> str:
    """Remove most MediaWiki markup, leaving readable prose."""
    # Remove templates {{...}} — iteratively to handle nesting
    for _ in range(4):
        text = re.sub(r"\{\{[^{}]*\}\}", "", text)
    # Remove tables {|...|} across multiple lines
    text = re.sub(r"\{\|.*?\|\}", "", text, flags=re.DOTALL)
    # Remove ref tags and their content
    text = re.sub(r"<ref[^>]*>.*?</ref>", "", text, flags=re.DOTALL)
    text = re.sub(r"<ref[^/]*/?>", "", text)
    # Remove HTML tags
    text = re.sub(r"<[^>]+>", "", text)
    # Convert wikilinks [[Link|Display]] → Display
    text = re.sub(r"\[\[(?:[^|\]]*\|)?([^\]]+)\]\]", r"\1", text)
    # Remove external links [http://... text] → text
    text = re.sub(r"\[https?://\S+\s+([^\]]+)\]", r"\1", text)
    text = re.sub(r"\[https?://\S+\]", "", text)
    # Remove File/Image/Category links
    text = re.sub(r"\[\[(?:File|Image|Datei|Bild|Fichier|Archivo|Arquivo|File|Bestand|Plik):.*?\]\]",
                  "", text, flags=re.IGNORECASE | re.DOTALL)
    # Remove section headings markup == heading ==
    text = re.sub(r"={2,}[^=]+=+", "", text)
    # Remove bold/italic markup
    text = text.replace("'''", "").replace("''", "")
    # Remove XML entities
    text = text.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">")
    text = text.replace("&quot;", '"').replace("&nbsp;", " ")
    text = re.sub(r"&\w+;", "", text)
    # Remove any remaining empty or unclosed ref tags
    text = re.sub(r"<ref[^>]*/?>", "", text)
    text = re.sub(r"</ref>", "", text)
    # Remove HTML comments
    text = re.sub(r"<!--.*?-->", "", text, flags=re.DOTALL)
    # Collapse whitespace
    text = re.sub(r"\n{3,}", "\n\n", text)
    text = re.sub(r"[ \t]+", " ", text)
    return text.strip()


def extract_articles(xml_bytes: bytes):
    """
    Yield (title, clean_text) from a Wikipedia XML dump chunk.
    Uses simple regex — much faster than full XML parsing for streaming.
    """
    title_re  = re.compile(rb"<title>(.*?)</title>")
    text_re   = re.compile(rb"<text[^>]*>(.*?)</text>", re.DOTALL)
    ns_re     = re.compile(rb"<ns>(\d+)</ns>")

    # Process page by page
    page_re = re.compile(rb"<page>(.*?)</page>", re.DOTALL)
    for m in page_re.finditer(xml_bytes):
        page = m.group(1)
        # Only main namespace (ns=0)
        ns_m = ns_re.search(page)
        if ns_m and ns_m.group(1) != b"0":
            continue
        title_m = title_re.search(page)
        text_m  = text_re.search(page)
        if not title_m or not text_m:
            continue
        title = title_m.group(1).decode("utf-8", errors="replace")
        raw   = text_m.group(1).decode("utf-8", errors="replace")
        # Skip redirects
        if raw.strip().lower().startswith("#redirect") or raw.strip().lower().startswith("#weiterleitung"):
            continue
        clean = strip_wikitext(raw)
        if len(clean) < 100:
            continue
        yield title, clean


# ── Download helpers ──────────────────────────────────────────────────────

def get_latest_dump_date(lang: str) -> str:
    """Return the most recent dated dump directory for a language wiki."""
    url = f"{WIKIDUMP_BASE}/{lang}wiki/"
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=15) as r:
        content = r.read().decode("utf-8", errors="replace")
    dates = re.findall(r'href="(\d{8})/"', content)
    if not dates:
        raise RuntimeError(f"No dated dumps found for {lang}wiki at {url}")
    return sorted(dates, reverse=True)[0]


def find_first_partition_url(lang: str, date: str) -> str:
    """Find the URL of the first pages-articles-multistream partition."""
    listing_url = f"{WIKIDUMP_BASE}/{lang}wiki/{date}/"
    req = urllib.request.Request(listing_url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=15) as r:
        content = r.read().decode("utf-8", errors="replace")
    # Match first partition: pages-articles-multistream1.xml-pXpY.bz2
    m = re.search(
        r'href="(/[^"]+pages-articles-multistream1\.xml-p\d+p\d+\.bz2)"',
        content
    )
    if not m:
        # Fallback: look for any multistream1 file
        m = re.search(r'href="(/[^"]+multistream1[^"]+\.bz2)"', content)
    if not m:
        raise RuntimeError(f"No multistream1 partition found for {lang}wiki/{date}")
    path = m.group(1)
    return f"https://dumps.wikimedia.org{path}"


def download_language(lang: str, out_path: str, target_bytes: int) -> None:
    print(f"[{lang}] Discovering latest dump...")
    date = get_latest_dump_date(lang)
    url  = find_first_partition_url(lang, date)
    size_hint = ""
    print(f"[{lang}] {date} → {url.split('/')[-1]}{size_hint}")

    written = 0
    docs    = 0
    buf     = b""
    start   = time.time()

    req = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=60) as resp, \
         bz2.BZ2File(resp) as bz, \
         open(out_path, "w", encoding="utf-8") as out:

        while written < target_bytes:
            chunk = bz.read(CHUNK_SIZE)
            if not chunk:
                break
            buf += chunk

            # Process complete <page>...</page> blocks
            last_end = 0
            for pm in re.finditer(rb"</page>", buf):
                page_end = pm.end()
                page_chunk = buf[last_end:page_end]
                last_end = page_end

                for title, text in extract_articles(page_chunk):
                    out.write(text + "\n\n")
                    written += len(text.encode("utf-8"))
                    docs += 1
                    if written >= target_bytes:
                        break
                if written >= target_bytes:
                    break

            # Keep unprocessed tail
            buf = buf[last_end:]

            if docs > 0 and docs % 2000 == 0:
                mb = written / 1024 / 1024
                elapsed = time.time() - start
                print(f"  [{lang}] {docs:,} articles · {mb:.1f}MB · {elapsed:.0f}s")
                sys.stdout.flush()

    mb = written / 1024 / 1024
    elapsed = time.time() - start
    print(f"[{lang}] Done: {docs:,} articles · {mb:.1f}MB · {elapsed:.0f}s → {out_path}")


# ── English from existing corpus ─────────────────────────────────────────

def build_english(out_path: str) -> None:
    stage_dirs = [f"stage_{n}" for n in [3, 6, 7, 8]]
    written = 0
    with open(out_path, "w", encoding="utf-8") as out:
        for sd in stage_dirs:
            d = os.path.join(REPO_ROOT, "data", "corpus", sd)
            if not os.path.isdir(d):
                continue
            for fname in sorted(os.listdir(d)):
                if not fname.endswith(".txt"):
                    continue
                fpath = os.path.join(d, fname)
                with open(fpath, "r", encoding="utf-8", errors="replace") as f:
                    content = f.read()
                out.write(content)
                written += len(content.encode("utf-8"))
                print(f"  [en] {fname}: {len(content)//1024}KB")
    print(f"[en] Done: {written/1024/1024:.1f}MB → {out_path}")


# ── Main ──────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--lang", nargs="*", default=LANGUAGES)
    parser.add_argument("--target-mb", type=int, default=DEFAULT_TARGET_MB)
    parser.add_argument("--skip-en", action="store_true")
    args = parser.parse_args()

    os.makedirs(OUT_DIR, exist_ok=True)
    target_bytes = args.target_mb * 1024 * 1024

    # English
    en_path = os.path.join(OUT_DIR, "en.txt")
    if not args.skip_en:
        if os.path.exists(en_path) and os.path.getsize(en_path) > 1024 * 1024:
            print(f"[en] Already exists ({os.path.getsize(en_path)//1024//1024}MB) — skipping")
        else:
            build_english(en_path)

    # Foreign languages
    for lang in args.lang:
        if lang not in LANGUAGES:
            print(f"Unknown language: {lang} — available: {LANGUAGES}")
            continue
        out_path = os.path.join(OUT_DIR, f"{lang}.txt")
        if os.path.exists(out_path) and os.path.getsize(out_path) > 1024 * 1024:
            mb = os.path.getsize(out_path) / 1024 / 1024
            print(f"[{lang}] Already exists ({mb:.1f}MB) — skipping (delete to re-download)")
            continue
        try:
            download_language(lang, out_path, target_bytes)
        except Exception as e:
            print(f"[{lang}] ERROR: {e}")
            import traceback; traceback.print_exc()

    # Summary
    print("\n=== Corpus Summary ===")
    total = 0
    for lang in ["en"] + LANGUAGES:
        p = os.path.join(OUT_DIR, f"{lang}.txt")
        if os.path.exists(p):
            mb = os.path.getsize(p) / 1024 / 1024
            total += os.path.getsize(p)
            print(f"  {lang}: {mb:.1f}MB")
    print(f"  TOTAL: {total/1024/1024:.1f}MB")
    print(f"\nCorpus ready at: {OUT_DIR}")
    print("Next: python3 scripts/train_tokenizer_v3.py")


if __name__ == "__main__":
    main()
