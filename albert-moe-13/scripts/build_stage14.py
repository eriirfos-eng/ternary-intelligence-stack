#!/usr/bin/env python3
"""
Stage 14 corpus builder — "Sovereign Intelligence"
Albert MoE-13 training data pipeline

Unlock condition: loss_avg ≤ 2.0 AND architecture ≥ 65L 8-stream cord
                  AND Bizon 4-GPU active (Modal T4 insufficient at this scale)

Sources:
  1. OSCAR 23.01             — filtered multilingual Common Crawl (~600B tokens)
  2. The Stack full          — all programming languages (~200B tokens)
  3. StackExchange full dump — all sites, all years (~30B tokens)
  4. mC4 multilingual        — filtered web text (~100B tokens)
  5. Mathematical proofs      — Lean/Coq/Isabelle formal corpora (~2B tokens)
  6. CC-licensed news        — large-scale journalism archives (~20B tokens)

Target: ~1T tokens (~4 TB text)
Storage: ~8 TB (raw + processed)
Compute: Bizon 4× GPU mandatory

Run:
    # Pre-download large datasets first (see --help):
    python scripts/build_stage14.py --download-oscar --oscar-path /data/oscar
    python scripts/build_stage14.py --download-stack --stack-path /data/thestack

    # Then build:
    python scripts/build_stage14.py \
        --oscar-path /data/oscar \
        --stack-path /data/thestack

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
CORPUS_DIR = SCRIPT_DIR.parent / "data" / "corpus" / "stage_14"
CORPUS_DIR.mkdir(parents=True, exist_ok=True)

UA    = "AlbertMoE13CorpusBot/1.1 (stage14 training corpus; contact contact@ternlang.com)"
EMAIL = "contact@ternlang.com"

# ── tuning ────────────────────────────────────────────────────────────────────
# At this scale, limits are set by storage and compute, not API rate limits.
# Adjust based on available disk and GPU memory.

OSCAR_MAX_PER_LANG      = 10_000_000    # documents per language
STACK_FULL_MAX_PER_LANG = 2_000_000     # code files per language
SE_FULL_MAX_PER_SITE    = 500_000       # Q&A threads per site
MC4_MAX_PER_LANG        = 5_000_000     # documents per language
PROOF_MAX               = 500_000       # formal proof statements
NEWS_MAX                = 1_000_000     # news articles

OSCAR_LANGUAGES = [
    "en", "de", "fr", "es", "pt", "it", "nl", "pl", "ru", "uk",
    "zh", "ja", "ko", "ar", "hi", "tr", "vi", "id", "fa", "cs",
    "ro", "hu", "fi", "da", "sv", "no", "sk", "bg", "ca", "hr",
]

STACK_ALL_LANGS = [
    "python", "rust", "typescript", "javascript", "go", "c", "cpp",
    "java", "scala", "haskell", "ocaml", "erlang", "elixir", "ruby",
    "php", "swift", "kotlin", "r", "julia", "lua", "shell", "sql",
]

SE_FULL_SITES = [
    "stackoverflow", "math", "physics", "chemistry", "biology",
    "cs", "english", "philosophy", "history", "economics",
    "linguistics", "law", "politics", "psychology", "worldbuilding",
    "scifi", "literature", "writing", "cooking", "fitness",
]

MC4_LANGUAGES = [
    "en", "de", "fr", "es", "pt", "ru", "zh", "ja", "ar", "ko",
]

PROOF_REPOS = [
    ("leanprover-community/mathlib4", "Lean4 Mathlib"),
    ("coq/coq",                       "Coq stdlib"),
    ("isabelle-community/isabelle",    "Isabelle"),
    ("UniMath/UniMath",                "Univalent Mathematics"),
]


# ── utilities ─────────────────────────────────────────────────────────────────

def log(msg: str):
    ts = datetime.now().strftime("%H:%M:%S")
    print(f"[{ts}] {msg}", flush=True)

def fetch(url: str, *, delay: float = 0.2, retries: int = 3,
          headers: dict | None = None) -> bytes | None:
    hdrs = {"User-Agent": UA}
    if headers:
        hdrs.update(headers)
    for attempt in range(retries):
        try:
            req = urllib.request.Request(url, headers=hdrs)
            with urllib.request.urlopen(req, timeout=60) as r:
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

def write_block(fh, block: str):
    fh.write(block.strip() + "\n\n" + "─" * 72 + "\n\n")
    fh.flush()

def require_datasets():
    try:
        import datasets  # type: ignore  # noqa
        return True
    except ImportError:
        log("  ERROR: pip install datasets huggingface_hub required for stage 14")
        return False


# ── 1. OSCAR multilingual filtered Common Crawl ───────────────────────────────

def build_oscar(out_dir: Path, oscar_path: str | None):
    log("OSCAR 23.01 — multilingual filtered Common Crawl ...")
    if not require_datasets():
        return

    from datasets import load_dataset  # type: ignore

    for lang in OSCAR_LANGUAGES:
        out_path = out_dir / f"oscar_{lang}.txt"
        if out_path.exists():
            log(f"  {lang}: already exists, skipping")
            continue

        log(f"  language: {lang}")
        count = 0

        try:
            if oscar_path and Path(oscar_path).exists():
                ds = load_dataset(
                    "oscar-corpus/OSCAR-2301",
                    language=lang,
                    data_dir=oscar_path,
                    split="train",
                    streaming=True,
                )
            else:
                ds = load_dataset(
                    "oscar-corpus/OSCAR-2301",
                    language=lang,
                    split="train",
                    streaming=True,
                    trust_remote_code=True,
                )

            with open(out_path, "w", encoding="utf-8") as fh:
                for sample in ds:
                    if count >= OSCAR_MAX_PER_LANG:
                        break
                    text = sample.get("text", "").strip()
                    if len(text) < 200:
                        continue
                    # OSCAR already filtered — write directly with minimal header
                    fh.write(f"[OSCAR-{lang.upper()}]\n{text[:6000]}\n\n─────\n\n")
                    fh.flush()
                    count += 1
                    if count % 100_000 == 0:
                        log(f"    {count:,} documents ...")

        except Exception as e:
            log(f"  error loading OSCAR {lang}: {e}")

        log(f"    → {count:,} documents → oscar_{lang}.txt")


# ── 2. The Stack full ─────────────────────────────────────────────────────────

def build_stack_full(out_dir: Path, stack_path: str | None):
    log("The Stack full — all programming languages ...")
    if not require_datasets():
        return

    from datasets import load_dataset  # type: ignore

    for lang in STACK_ALL_LANGS:
        out_path = out_dir / f"stack_{lang}.txt"
        if out_path.exists():
            log(f"  {lang}: already exists, skipping")
            continue

        log(f"  language: {lang}")
        count = 0

        try:
            kwargs = {
                "split": "train",
                "streaming": True,
                "trust_remote_code": True,
            }
            if stack_path and Path(stack_path).exists():
                ds = load_dataset("bigcode/the-stack", data_dir=stack_path, **kwargs)
            else:
                ds = load_dataset("bigcode/the-stack", data_dir=f"data/{lang}", **kwargs)

            with open(out_path, "w", encoding="utf-8") as fh:
                for sample in ds:
                    if count >= STACK_FULL_MAX_PER_LANG:
                        break
                    content = sample.get("content", "").strip()
                    if len(content) < 100 or len(content) > 50000:
                        continue
                    repo = sample.get("max_stars_repo_name", "")
                    path = sample.get("max_stars_repo_path", "")
                    fh.write(
                        f"[CODE-{lang.upper()}] {repo}/{path}\n"
                        f"{content}\n\n─────\n\n"
                    )
                    fh.flush()
                    count += 1
                    if count % 50_000 == 0:
                        log(f"    {count:,} files ...")

        except Exception as e:
            log(f"  error loading stack {lang}: {e}")

        log(f"    → {count:,} files → stack_{lang}.txt")


# ── 3. StackExchange full data dump ───────────────────────────────────────────

def build_se_full(out_path: Path):
    """
    Stack Exchange full data dump is available at archive.org as XML.
    This streams via the HuggingFace dataset version for convenience.
    """
    log("StackExchange full dump — streaming via HuggingFace ...")
    if not require_datasets():
        return

    from datasets import load_dataset  # type: ignore

    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for site in SE_FULL_SITES:
            log(f"  site: {site}")
            count = 0
            try:
                ds = load_dataset(
                    "HuggingFaceH4/stack-exchange-preferences",
                    split="train",
                    streaming=True,
                )
                for sample in ds:
                    if count >= SE_FULL_MAX_PER_SITE:
                        break
                    question = (sample.get("question") or "").strip()
                    answers  = sample.get("answers") or []
                    if not question or not answers:
                        continue
                    best = max(answers, key=lambda a: a.get("pm_score", 0), default=None)
                    if not best:
                        continue
                    answer_text = (best.get("text") or "").strip()
                    if len(answer_text) < 100:
                        continue
                    fh.write(
                        f"[STACKEXCHANGE-{site.upper()}]\n"
                        f"Q: {question[:1000]}\n"
                        f"A: {answer_text[:2000]}\n\n─────\n\n"
                    )
                    fh.flush()
                    count += 1
                    total += 1
            except Exception as e:
                log(f"  error loading {site}: {e}")
            log(f"    → {count:,} threads")
    log(f"StackExchange full done — {total:,} total → {out_path.name}")


# ── 4. mC4 multilingual ───────────────────────────────────────────────────────

def build_mc4(out_dir: Path):
    log("mC4 multilingual — streaming via HuggingFace ...")
    if not require_datasets():
        return

    from datasets import load_dataset  # type: ignore

    for lang in MC4_LANGUAGES:
        out_path = out_dir / f"mc4_{lang}.txt"
        if out_path.exists():
            log(f"  {lang}: already exists, skipping")
            continue

        log(f"  language: {lang}")
        count = 0
        try:
            ds = load_dataset("mc4", lang, split="train", streaming=True)
            with open(out_path, "w", encoding="utf-8") as fh:
                for sample in ds:
                    if count >= MC4_MAX_PER_LANG:
                        break
                    text = sample.get("text", "").strip()
                    if len(text) < 300:
                        continue
                    fh.write(f"[MC4-{lang.upper()}]\n{text[:5000]}\n\n─────\n\n")
                    fh.flush()
                    count += 1
                    if count % 100_000 == 0:
                        log(f"    {count:,} documents ...")
        except Exception as e:
            log(f"  error loading mC4 {lang}: {e}")
        log(f"    → {count:,} documents → mc4_{lang}.txt")


# ── 5. Formal mathematical proofs ─────────────────────────────────────────────

def build_formal_proofs(out_path: Path):
    log("Formal mathematical proofs — GitHub API ...")
    GH_TOKEN = os.getenv("GITHUB_TOKEN", "")
    headers  = {
        "Accept": "application/vnd.github+json",
        "User-Agent": UA,
        "X-GitHub-Api-Version": "2022-11-28",
    }
    if GH_TOKEN:
        headers["Authorization"] = f"Bearer {GH_TOKEN}"

    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for repo, label in PROOF_REPOS:
            log(f"  repo: {repo} ({label})")
            count = 0
            page = 1
            exts = {
                "Lean4 Mathlib": ".lean",
                "Coq stdlib": ".v",
                "Isabelle": ".thy",
                "Univalent Mathematics": ".v",
            }
            ext = exts.get(label, ".lean")

            while count < PROOF_MAX // len(PROOF_REPOS):
                url = (
                    f"https://api.github.com/repos/{repo}/git/trees/HEAD"
                    f"?recursive=1&per_page=100&page={page}"
                )
                data = fetch_json(url, headers=headers, delay=1.5)
                if not data:
                    break
                files = [
                    f for f in data.get("tree", [])
                    if f.get("type") == "blob" and f.get("path", "").endswith(ext)
                ]
                for f in files[:100]:
                    if count >= PROOF_MAX // len(PROOF_REPOS):
                        break
                    raw_url = (
                        f"https://raw.githubusercontent.com/{repo}/HEAD/{f['path']}"
                    )
                    raw = fetch(raw_url, headers={"User-Agent": UA}, delay=0.3)
                    if not raw:
                        continue
                    content = raw.decode("utf-8", errors="replace").strip()
                    if len(content) < 100:
                        continue
                    fh.write(
                        f"[FORMAL-PROOF — {label.upper()}]\n"
                        f"File: {f['path']}\n\n"
                        f"{content[:3000]}\n\n─────\n\n"
                    )
                    fh.flush()
                    count += 1
                    total += 1
                page += 1
                if not data.get("truncated") and len(files) == 0:
                    break
            log(f"    → {count} proof files")
    log(f"Formal proofs done — {total} total → {out_path.name}")


# ── 6. CC-licensed news archives ──────────────────────────────────────────────

def build_news(out_path: Path):
    log("CC-licensed news — streaming via HuggingFace ...")
    if not require_datasets():
        return

    from datasets import load_dataset  # type: ignore

    news_datasets = [
        ("cc_news", None, "text", "title", "date"),
        ("RealTimeData/bbc_news_alltime", None, "content", "title", "date"),
    ]

    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for ds_name, config, text_key, title_key, date_key in news_datasets:
            log(f"  dataset: {ds_name}")
            count = 0
            try:
                ds = load_dataset(ds_name, config, split="train", streaming=True) if config \
                     else load_dataset(ds_name, split="train", streaming=True)
                for sample in ds:
                    if count >= NEWS_MAX // len(news_datasets):
                        break
                    text  = (sample.get(text_key) or "").strip()
                    title = (sample.get(title_key) or "").strip()
                    date  = str(sample.get(date_key) or "")[:10]
                    if len(text) < 300:
                        continue
                    fh.write(
                        f"[NEWS — {date}]\n"
                        f"Title: {title}\n\n"
                        f"{text[:4000]}\n\n─────\n\n"
                    )
                    fh.flush()
                    count += 1
                    total += 1
            except Exception as e:
                log(f"  error loading {ds_name}: {e}")
            log(f"    → {count:,} articles")
    log(f"News done — {total:,} total → {out_path.name}")


# ── main ──────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description="Stage 14 corpus builder — Sovereign Intelligence")
    parser.add_argument("--oscar-path", default=None,
                        help="Path to pre-downloaded OSCAR 23.01 dump")
    parser.add_argument("--stack-path", default=None,
                        help="Path to pre-downloaded The Stack dump")
    parser.add_argument("--download-oscar", action="store_true",
                        help="Download OSCAR 23.01 before building")
    parser.add_argument("--download-stack", action="store_true",
                        help="Download The Stack before building")
    parser.add_argument("--skip-oscar", action="store_true")
    parser.add_argument("--skip-stack", action="store_true")
    parser.add_argument("--skip-se",    action="store_true")
    parser.add_argument("--skip-mc4",   action="store_true")
    parser.add_argument("--skip-proofs",action="store_true")
    parser.add_argument("--skip-news",  action="store_true")
    args = parser.parse_args()

    log("=" * 60)
    log("Stage 14 corpus builder — 'Sovereign Intelligence'")
    log(f"Unlock: loss_avg ≤ 2.0 AND ≥ 65L 8-stream cord AND Bizon 4-GPU")
    log(f"Target: ~1T tokens (~4 TB)")
    log(f"Output: {CORPUS_DIR}")
    log("WARNING: requires ~8 TB disk space and significant wall-clock time")
    log("         Run on Bizon 4-GPU with NVMe storage for best performance")
    log("=" * 60)

    if args.download_oscar:
        log("Pre-downloading OSCAR 23.01 ...")
        os.system(
            f"huggingface-cli download oscar-corpus/OSCAR-2301 "
            f"--repo-type dataset "
            f"--local-dir {args.oscar_path or '/data/oscar'}"
        )

    if args.download_stack:
        log("Pre-downloading The Stack ...")
        os.system(
            f"huggingface-cli download bigcode/the-stack "
            f"--repo-type dataset "
            f"--local-dir {args.stack_path or '/data/thestack'}"
        )

    if not args.skip_oscar:
        oscar_dir = CORPUS_DIR / "oscar"
        oscar_dir.mkdir(exist_ok=True)
        build_oscar(oscar_dir, args.oscar_path)

    if not args.skip_stack:
        stack_dir = CORPUS_DIR / "stack"
        stack_dir.mkdir(exist_ok=True)
        build_stack_full(stack_dir, args.stack_path)

    if not args.skip_se:
        build_se_full(CORPUS_DIR / "stackexchange_full.txt")

    if not args.skip_mc4:
        mc4_dir = CORPUS_DIR / "mc4"
        mc4_dir.mkdir(exist_ok=True)
        build_mc4(mc4_dir)

    if not args.skip_proofs:
        build_formal_proofs(CORPUS_DIR / "formal_proofs.txt")

    if not args.skip_news:
        build_news(CORPUS_DIR / "news_archives.txt")

    log("")
    log("=" * 60)
    log("Stage 14 complete. Run chaos complement next:")
    log("  python scripts/build_chaos_corpus.py --scale stage14")
    log("")
    log("IMPORTANT: Rebuild tokenizer after adding stage 14:")
    log("  python scripts/train_tokenizer_v3.py --stages 1,2,...,14")
    log("=" * 60)

if __name__ == "__main__":
    main()
