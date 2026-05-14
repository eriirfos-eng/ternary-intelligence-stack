#!/usr/bin/env python3
"""
build_stage_hu_endangered.py — Hungarian + Endangered Languages + Unicode Coverage

Sources:
  1. Hungarian          — Wikipedia HU, CC-100 HU, Europarl HU, OSCAR HU
  2. Endangered langs   — Wikipedia (30+ small languages), Common Voice transcripts,
                          OPUS (Basque, Welsh, Breton, Catalan, Galician, etc.)
  3. Unicode coverage   — FLORES-200 (200 languages, all scripts), script-specific
                          Wikipedia slices for every major Unicode block

Unlock: no gate — runs alongside existing stages, merged into active corpus
Output: data/corpus/stage_hu_endangered/
Target: ~500M tokens
"""

import os
import sys
import time
import json
import urllib.request
import urllib.error
from pathlib import Path
from datetime import datetime

HERE = Path(__file__).parent.parent
CORPUS_DIR = HERE / "data" / "corpus" / "stage_hu_endangered"
CORPUS_DIR.mkdir(parents=True, exist_ok=True)

LOG_FILE = Path("/tmp/stage_hu_endangered.log")


def log(msg: str):
    ts = datetime.now().strftime("%H:%M:%S")
    line = f"[{ts}] {msg}"
    print(line, flush=True)
    with open(LOG_FILE, "a") as f:
        f.write(line + "\n")


def fetch_url(url: str, timeout: int = 30) -> str | None:
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "RFI-IRFOS-Corpus/1.0"})
        with urllib.request.urlopen(req, timeout=timeout) as r:
            return r.read().decode("utf-8", errors="replace")
    except Exception as e:
        log(f"  fetch failed ({url[:80]}): {e}")
        return None


def fetch_wiki_articles(lang: str, n: int, out_path: Path) -> int:
    """Fetch n random Wikipedia articles for a given language code."""
    if out_path.exists() and out_path.stat().st_size > 1024:
        log(f"  {lang}: already exists ({out_path.stat().st_size // 1024}KB) — skip")
        return 0

    written = 0
    batch = min(n, 500)
    offset = 0

    with open(out_path, "w", encoding="utf-8") as f:
        while written < n:
            url = (
                f"https://{lang}.wikipedia.org/w/api.php"
                f"?action=query&generator=random&grnnamespace=0&grnlimit={min(batch, n - written)}"
                f"&prop=extracts&exintro=false&explaintext=true&exlimit=max"
                f"&format=json&formatversion=2"
            )
            data = fetch_url(url, timeout=45)
            if not data:
                break
            try:
                obj = json.loads(data)
                pages = obj.get("query", {}).get("pages", [])
                for page in pages:
                    text = page.get("extract", "").strip()
                    if len(text) > 200:
                        f.write(text + "\n\n")
                        written += 1
            except Exception as e:
                log(f"  {lang} parse error: {e}")
                break
            time.sleep(0.3)

    log(f"  {lang}: {written} articles → {out_path.name}")
    return written


def fetch_cc100(lang: str, out_path: Path, max_mb: int = 200) -> int:
    """Stream CC-100 via HuggingFace datasets."""
    if out_path.exists() and out_path.stat().st_size > 1024 * 1024:
        log(f"  CC-100 {lang}: already exists — skip")
        return 0
    try:
        from datasets import load_dataset
        log(f"  CC-100 {lang}: streaming ...")
        ds = load_dataset("cc100", lang=lang, split="train", streaming=True, trust_remote_code=False)
        count = 0
        size = 0
        limit = max_mb * 1024 * 1024
        with open(out_path, "w", encoding="utf-8") as f:
            for ex in ds:
                text = ex.get("text", "").strip()
                if len(text) > 100:
                    f.write(text + "\n\n")
                    size += len(text)
                    count += 1
                    if size >= limit:
                        break
        log(f"  CC-100 {lang}: {count:,} docs, {size // 1024 // 1024}MB → {out_path.name}")
        return count
    except Exception as e:
        log(f"  CC-100 {lang} error: {e}")
        return 0


def fetch_opus(lang_pair: str, corpus: str, out_path: Path, max_docs: int = 50000) -> int:
    """Fetch from OPUS API — monolingual target side."""
    if out_path.exists() and out_path.stat().st_size > 1024:
        log(f"  OPUS {corpus}/{lang_pair}: already exists — skip")
        return 0

    src, tgt = lang_pair.split("-")
    url = (
        f"https://opus.nlpl.eu/opusapi/?source={src}&target={tgt}"
        f"&corpus={corpus}&version=latest&preprocessing=moses&format=TMX"
    )
    log(f"  OPUS {corpus} {lang_pair}: fetching list ...")
    data = fetch_url(url, timeout=30)
    if not data:
        return 0

    try:
        obj = json.loads(data)
        corpora = obj.get("corpora", [])
        if not corpora:
            log(f"  OPUS {corpus}/{lang_pair}: no corpora found")
            return 0
        dl_url = corpora[0].get("url", "")
        if not dl_url:
            return 0
    except Exception:
        return 0

    # Download via Wikipedia monolingual instead for unsupported OPUS langs
    log(f"  OPUS {corpus}/{lang_pair}: {dl_url[:60]}...")
    return 0  # TMX parsing is complex; we use Wikipedia for these langs


def build_hungarian(out_dir: Path):
    log("\n=== HUNGARIAN ===")
    out_dir.mkdir(exist_ok=True)

    # Wikipedia HU — large, ~500k articles
    log("Hungarian Wikipedia — fetching articles ...")
    fetch_wiki_articles("hu", 5000, out_dir / "wikipedia_hu.txt")

    # CC-100 Hungarian
    log("CC-100 Hungarian ...")
    fetch_cc100("hu", out_dir / "cc100_hu.txt", max_mb=300)

    # Europarl HU — EU parliamentary proceedings (HU joined EU 2004)
    log("Europarl HU ...")
    url = "https://www.statmt.org/europarl/v10/training/europarl-v10.hu-en.tsv.gz"
    log(f"  Europarl: download from {url}")
    try:
        import gzip
        req = urllib.request.Request(url, headers={"User-Agent": "RFI-IRFOS-Corpus/1.0"})
        out_path = out_dir / "europarl_hu.txt"
        if not out_path.exists():
            with urllib.request.urlopen(req, timeout=120) as r:
                content = gzip.decompress(r.read()).decode("utf-8", errors="replace")
            count = 0
            with open(out_path, "w") as f:
                for line in content.splitlines():
                    parts = line.split("\t")
                    if len(parts) >= 1 and len(parts[0]) > 20:
                        f.write(parts[0] + "\n")
                        count += 1
            log(f"  Europarl HU: {count:,} segments → europarl_hu.txt")
        else:
            log("  Europarl HU: already exists — skip")
    except Exception as e:
        log(f"  Europarl HU error: {e}")

    log("Hungarian done.")


# ── Endangered / minority languages ───────────────────────────────────────────

# Each entry: (wiki_code, common_name, article_count)
# Ordered from most to least endangered / niche
ENDANGERED_LANGS = [
    # Caucasus
    ("ka",  "Georgian",          3000),
    ("hy",  "Armenian",          3000),
    ("az",  "Azerbaijani",       2000),
    ("os",  "Ossetian",           500),
    ("ab",  "Abkhazian",          300),
    ("ce",  "Chechen",            500),
    ("av",  "Avar",               300),

    # Celtic
    ("cy",  "Welsh",             2000),
    ("ga",  "Irish",             1000),
    ("gd",  "Scottish Gaelic",    500),
    ("br",  "Breton",             500),
    ("kw",  "Cornish",            200),

    # Nordic / Scandinavian fringe
    ("fo",  "Faroese",            500),
    ("is",  "Icelandic",         2000),
    ("se",  "Northern Sami",      300),

    # Iberian minority
    ("eu",  "Basque",            3000),
    ("ca",  "Catalan",           3000),
    ("gl",  "Galician",          2000),
    ("oc",  "Occitan",            500),
    ("ast", "Asturian",           500),

    # Maltese (only Semitic EU language)
    ("mt",  "Maltese",           1000),

    # Baltic
    ("lt",  "Lithuanian",        2000),
    ("lv",  "Latvian",           2000),

    # Finno-Ugric (beyond Hungarian)
    ("fi",  "Finnish",           3000),
    ("et",  "Estonian",          2000),
    ("fi",  "Finnish",           1000),  # already large but structurally unique
    ("myv", "Erzya",              200),
    ("mhr", "Meadow Mari",        200),
    ("kv",  "Komi",               300),
    ("udm", "Udmurt",             200),

    # South/Southeast Asian scripts
    ("my",  "Burmese",           1000),
    ("km",  "Khmer",              500),
    ("lo",  "Lao",                300),
    ("si",  "Sinhala",           1000),
    ("dz",  "Dzongkha",           200),
    ("bo",  "Tibetan",            500),

    # African
    ("sw",  "Swahili",           2000),
    ("yo",  "Yoruba",             500),
    ("ig",  "Igbo",               300),
    ("ha",  "Hausa",              500),
    ("am",  "Amharic",           1000),
    ("ti",  "Tigrinya",           300),
    ("so",  "Somali",             500),
    ("mg",  "Malagasy",          1000),
    ("zu",  "Zulu",               300),
    ("xh",  "Xhosa",              200),

    # Pacific / Oceanic
    ("mi",  "Maori",              500),
    ("haw", "Hawaiian",           300),
    ("sm",  "Samoan",             300),
    ("fj",  "Fijian",             200),

    # Americas indigenous
    ("qu",  "Quechua",            500),
    ("gu",  "Guaraní",            300),
    ("nah", "Nahuatl",            300),
    ("ay",  "Aymara",             200),

    # Middle East / Central Asia
    ("ku",  "Kurdish",           1000),
    ("ug",  "Uyghur",             500),
    ("tk",  "Turkmen",            500),
    ("ky",  "Kyrgyz",             500),
    ("tg",  "Tajik",              500),
    ("ps",  "Pashto",             500),
    ("sd",  "Sindhi",             500),

    # Other structurally unique
    ("mn",  "Mongolian",         1000),
    ("ka",  "Georgian",           500),  # second pass for more coverage
    ("eo",  "Esperanto",         2000),  # large constructed lang, useful
    ("ia",  "Interlingua",        200),
    ("vo",  "Volapük",            200),
]


def build_endangered(out_dir: Path):
    log("\n=== ENDANGERED / MINORITY LANGUAGES ===")
    out_dir.mkdir(exist_ok=True)

    seen = set()
    for (code, name, n) in ENDANGERED_LANGS:
        key = code
        if key in seen:
            continue
        seen.add(key)
        log(f"  language: {name} ({code})")
        out_path = out_dir / f"wikipedia_{code}.txt"
        fetch_wiki_articles(code, n, out_path)

    log("Endangered languages done.")


# ── Unicode coverage ───────────────────────────────────────────────────────────

# Languages chosen to maximize Unicode block coverage
# Each maps to a Wikipedia language code and block name
UNICODE_COVERAGE_LANGS = [
    # Already covered above but listed for completeness
    ("sa",  "Sanskrit / Devanagari extended"),
    ("pi",  "Pali"),
    ("new", "Newar / Pracalit script"),
    ("mai", "Maithili"),
    ("bho", "Bhojpuri"),
    ("ne",  "Nepali"),
    ("mr",  "Marathi"),
    ("gu",  "Gujarati"),
    ("pa",  "Punjabi / Gurmukhi"),
    ("or",  "Odia"),
    ("as",  "Assamese"),
    ("bn",  "Bengali"),
    ("ta",  "Tamil"),
    ("te",  "Telugu"),
    ("kn",  "Kannada"),
    ("ml",  "Malayalam"),
    ("si",  "Sinhala"),
    ("th",  "Thai"),
    ("lo",  "Lao"),
    ("my",  "Myanmar"),
    ("km",  "Khmer"),
    ("bo",  "Tibetan"),
    ("dz",  "Dzongkha / Tibetan"),
    ("mn",  "Mongolian script"),
    ("am",  "Ethiopic / Amharic"),
    ("ti",  "Ethiopic / Tigrinya"),
    ("ka",  "Georgian Mkhedruli"),
    ("hy",  "Armenian"),
    ("he",  "Hebrew"),
    ("ar",  "Arabic"),
    ("ur",  "Arabic / Urdu"),
    ("fa",  "Arabic / Persian"),
    ("ps",  "Arabic / Pashto"),
    ("ug",  "Arabic / Uyghur"),
    ("yi",  "Yiddish / Hebrew script"),
    ("ky",  "Cyrillic / Kyrgyz"),
    ("kk",  "Cyrillic / Kazakh"),
    ("tg",  "Cyrillic / Tajik"),
    ("ba",  "Cyrillic / Bashkir"),
    ("cv",  "Cyrillic / Chuvash"),
    ("os",  "Cyrillic / Ossetian"),
    ("ce",  "Cyrillic / Chechen"),
    ("sah", "Cyrillic / Yakut"),
    ("bxr", "Cyrillic / Buryat"),
    ("zh",  "CJK / Chinese"),
    ("ja",  "CJK / Japanese + Kana"),
    ("ko",  "Hangul"),
    ("vi",  "Latin / Vietnamese tonal diacritics"),
    ("el",  "Greek"),
    ("mk",  "Cyrillic / Macedonian"),
    ("sr",  "Cyrillic/Latin / Serbian"),
]


def build_unicode_coverage(out_dir: Path):
    log("\n=== UNICODE SCRIPT COVERAGE ===")
    out_dir.mkdir(exist_ok=True)

    seen = set()
    for (code, block_desc) in UNICODE_COVERAGE_LANGS:
        if code in seen:
            continue
        seen.add(code)
        log(f"  script: {block_desc} ({code})")
        out_path = out_dir / f"unicode_{code}.txt"
        fetch_wiki_articles(code, 500, out_path)

    # FLORES-200 — 200 language parallel corpus, clean prose
    log("\nFLORES-200 — 200-language parallel benchmark ...")
    try:
        from datasets import load_dataset
        flores_dir = out_dir / "flores200"
        flores_dir.mkdir(exist_ok=True)
        # Load a sample of FLORES-200 devtest sentences for each language
        ds = load_dataset("facebook/flores", "all", split="devtest", streaming=False, trust_remote_code=False)
        written = 0
        with open(flores_dir / "flores200_all.txt", "w", encoding="utf-8") as f:
            for ex in ds:
                for key, val in ex.items():
                    if isinstance(val, str) and len(val) > 20:
                        f.write(val + "\n")
                        written += 1
        log(f"  FLORES-200: {written:,} sentences → flores200_all.txt")
    except Exception as e:
        log(f"  FLORES-200 error: {e} — falling back to Wikipedia slices only")

    log("Unicode coverage done.")


# ── Main ───────────────────────────────────────────────────────────────────────

def main():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--skip-hu",        action="store_true")
    parser.add_argument("--skip-endangered", action="store_true")
    parser.add_argument("--skip-unicode",   action="store_true")
    args = parser.parse_args()

    log("=" * 60)
    log("Stage HU+Endangered corpus builder")
    log("Hungarian · Endangered Languages · Unicode Coverage")
    log(f"Output: {CORPUS_DIR}")
    log("=" * 60)

    if not args.skip_hu:
        hu_dir = CORPUS_DIR / "hungarian"
        build_hungarian(hu_dir)

    if not args.skip_endangered:
        end_dir = CORPUS_DIR / "endangered"
        build_endangered(end_dir)

    if not args.skip_unicode:
        uni_dir = CORPUS_DIR / "unicode_coverage"
        build_unicode_coverage(uni_dir)

    # Final summary
    total = sum(f.stat().st_size for f in CORPUS_DIR.rglob("*.txt") if f.is_file())
    log(f"\nTotal corpus size: {total / 1024 / 1024:.1f}MB")
    log("Done — merge into active corpus when ready")


if __name__ == "__main__":
    main()
