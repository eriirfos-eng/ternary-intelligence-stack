"""
build_chaos_corpus.py
Chaos protocol — 10% of albert. v3.0 tokenizer corpus.

The world is not clean. A model trained only on pristine resolved text
learns the wrong prior: it expects completeness. Chaos injection teaches
the model to hold uncertainty, handle noise, and recognize that some
questions have no answers.

Chaos types:
  1. Redacted corpus   — bible + stage text with [MASK]/[REDACTED] injections
  2. Truncated text    — clean arguments cut off mid-thought, no resolution
  3. OCR corruption    — character-level noise: l↔1, o↔0, dropped chars
  4. Open questions    — unanswered StackExchange questions (Math, CS, Philosophy)
  5. Code-switched     — inter-language paragraph mixing from Wikipedia corpus
  6. Masked academic   — academic abstracts at 20-40% masking rate

Output: data/corpus/chaos/chaos_en.txt    (~20MB)
        data/corpus/chaos/chaos_multi.txt  (~5MB multilingual)

Usage:
    python3 scripts/build_chaos_corpus.py
    python3 scripts/build_chaos_corpus.py --target-mb 30
"""

import argparse
import gzip
import html
import json
import os
import random
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT  = os.path.dirname(SCRIPT_DIR)
OUT_DIR    = os.path.join(REPO_ROOT, "data", "corpus", "chaos")

CORPUS_DIR    = os.path.join(REPO_ROOT, "data", "corpus")
STAGE_3_DIR   = os.path.join(CORPUS_DIR, "stage_3")
STAGE_6_DIR   = os.path.join(CORPUS_DIR, "stage_6")
STAGE_7_DIR   = os.path.join(CORPUS_DIR, "stage_7")
STAGE_10_DIR  = os.path.join(CORPUS_DIR, "stage_10")
MULTI_DIR     = os.path.join(CORPUS_DIR, "multilingual")
ACADEMIC_DIR  = os.path.join(CORPUS_DIR, "academic")

DEFAULT_TARGET_MB = 25
UA = "AlbertV3ChaosBuilder/1.0 (rfi.irfos@gmail.com)"

RNG = random.Random(42)   # reproducible chaos


# ── Helpers ───────────────────────────────────────────────────────────────────

def fetch(url: str, params: dict = None, retries: int = 3) -> bytes:
    if params:
        url = url + "?" + urllib.parse.urlencode(params)
    for attempt in range(retries):
        try:
            req = urllib.request.Request(url, headers={
                "User-Agent":      UA,
                "Accept-Encoding": "gzip",
                "Accept":          "application/json",
            })
            with urllib.request.urlopen(req, timeout=20) as r:
                raw = r.read()
                if r.info().get("Content-Encoding") == "gzip":
                    raw = gzip.decompress(raw)
                return raw
        except (urllib.error.URLError, OSError) as e:
            if attempt == retries - 1:
                raise
            time.sleep(3 * (attempt + 1))


def strip_html(text: str) -> str:
    text = re.sub(r"<[^>]+>", " ", text)
    text = html.unescape(text)
    text = re.sub(r"[ \t]+", " ", text)
    text = re.sub(r"\n{3,}", "\n\n", text)
    return text.strip()


def read_corpus_file(path: str) -> str:
    with open(path, "r", encoding="utf-8", errors="replace") as f:
        return f.read()


def paragraphs(text: str) -> list[str]:
    return [p.strip() for p in re.split(r"\n\n+", text) if len(p.strip()) > 60]


def sentences(para: str) -> list[str]:
    return [s.strip() for s in re.split(r"(?<=[.!?])\s+", para) if len(s.strip()) > 20]


def words(text: str) -> list[str]:
    return text.split()


# ── Chaos type 1: Redacted corpus ─────────────────────────────────────────────

MASK_TOKENS = ["[MASK]", "[REDACTED]", "[UNKNOWN]", "[???]"]

def mask_word(word: str, rng: random.Random) -> str:
    return rng.choice(MASK_TOKENS)


def apply_word_masking(text: str, rate: float, rng: random.Random) -> str:
    ws = text.split(" ")
    result = []
    for w in ws:
        if w and rng.random() < rate:
            result.append(mask_word(w, rng))
        else:
            result.append(w)
    return " ".join(result)


def apply_sentence_redaction(para: str, rate: float, rng: random.Random) -> str:
    sents = sentences(para)
    if not sents:
        return para
    result = []
    for s in sents:
        if rng.random() < rate:
            result.append("[PASSAGE REDACTED]")
        else:
            result.append(s)
    return " ".join(result)


def apply_section_redaction(paras: list[str], section_rate: float,
                             rng: random.Random) -> list[str]:
    result = []
    i = 0
    while i < len(paras):
        if rng.random() < section_rate and i + 3 < len(paras):
            # Blank out a block of 2-5 paragraphs
            block_size = rng.randint(2, 5)
            result.append("[SECTION REDACTED]")
            i += block_size
        else:
            result.append(paras[i])
            i += 1
    return result


def build_redacted_corpus(target_bytes: int) -> list[str]:
    """
    Apply [MASK]/[REDACTED] chaos to existing stage corpus files.
    Varies masking rate per passage: 10%, 20%, 40%, or sentence-level redaction.
    """
    sources = [
        os.path.join(STAGE_3_DIR, "bible.txt"),
        os.path.join(STAGE_3_DIR, "alice.txt"),
        os.path.join(STAGE_6_DIR, "simple_wikipedia.txt"),
        os.path.join(STAGE_7_DIR, "linux_docs.txt"),
        os.path.join(STAGE_10_DIR, "long_solved_threads.txt"),
        os.path.join(STAGE_10_DIR, "github_bugs.txt"),
    ]
    sources = [s for s in sources if os.path.exists(s)]
    texts = []
    written = 0

    print("  [chaos] Redacted corpus ...")
    for src_path in sources:
        if written >= target_bytes:
            break
        label = os.path.basename(src_path)
        raw = read_corpus_file(src_path)
        paras = paragraphs(raw)
        RNG.shuffle(paras)

        # Apply section-level redaction first (5% of sections)
        paras = apply_section_redaction(paras, 0.05, RNG)

        for para in paras:
            if written >= target_bytes:
                break
            mode = RNG.random()
            if mode < 0.30:
                # Light masking: 10% words
                corrupted = apply_word_masking(para, 0.10, RNG)
            elif mode < 0.55:
                # Medium masking: 20% words
                corrupted = apply_word_masking(para, 0.20, RNG)
            elif mode < 0.70:
                # Heavy masking: 40% words
                corrupted = apply_word_masking(para, 0.40, RNG)
            elif mode < 0.85:
                # Sentence-level redaction
                corrupted = apply_sentence_redaction(para, 0.25, RNG)
            else:
                # Pass through clean (10% clean paragraphs — contrast is important)
                corrupted = para

            if len(corrupted) > 40:
                texts.append(corrupted)
                written += len(corrupted.encode())

        print(f"  [chaos] {label}: {written/1024/1024:.1f}MB total")

    return texts


# ── Chaos type 2: Truncated text ──────────────────────────────────────────────

def truncate_at(text: str, frac: float) -> str:
    """Cut text off at frac of its length. No resolution, no closure."""
    chars = int(len(text) * frac)
    cut = text[:chars]
    # Sometimes cut mid-sentence (abrupt), sometimes at a sentence boundary
    if RNG.random() < 0.4:
        return cut.rstrip()
    # Find last sentence boundary before the cut
    m = list(re.finditer(r"[.!?]\s+", cut))
    if m and len(m) > 1:
        cut = cut[:m[-2].end()]
    return cut.rstrip()


def build_truncated_corpus(target_bytes: int) -> list[str]:
    """
    Take long-form clean text and cut it off mid-argument.
    Teaches: incompleteness, unresolved reasoning, hanging thoughts.
    """
    sources = [
        os.path.join(STAGE_6_DIR, "simple_wikipedia.txt"),
        os.path.join(STAGE_10_DIR, "long_solved_threads.txt"),
        os.path.join(STAGE_10_DIR, "repair_guides.txt"),
        os.path.join(STAGE_10_DIR, "dev_blogs.txt"),
        os.path.join(STAGE_10_DIR, "trails_and_travel.txt"),
    ]
    sources = [s for s in sources if os.path.exists(s)]
    texts = []
    written = 0

    print("  [chaos] Truncated corpus ...")
    for src_path in sources:
        if written >= target_bytes:
            break
        raw = read_corpus_file(src_path)
        paras = paragraphs(raw)
        RNG.shuffle(paras)

        # Group into multi-paragraph chunks, then truncate the chunk
        i = 0
        while i < len(paras) and written < target_bytes:
            chunk_size = RNG.randint(2, 6)
            chunk = "\n\n".join(paras[i:i+chunk_size])
            i += chunk_size
            if len(chunk) < 200:
                continue
            frac = RNG.uniform(0.25, 0.75)
            truncated = truncate_at(chunk, frac)
            if len(truncated) > 80:
                texts.append(truncated)
                written += len(truncated.encode())

        print(f"  [chaos] {os.path.basename(src_path)}: {written/1024/1024:.1f}MB total")

    return texts


# ── Chaos type 3: OCR-style corruption ────────────────────────────────────────

# Realistic OCR confusions
OCR_SUBS = {
    "l": "1", "1": "l",
    "o": "0", "0": "o",
    "I": "l", "O": "0",
    "rn": "m", "m": "rn",
    "cl": "d", "d": "cl",
    "vv": "w", "w": "vv",
    "ii": "n",
}

def ocr_corrupt_word(word: str, rng: random.Random) -> str:
    if len(word) < 2:
        return word
    # 2-char substitutions first (less likely to break things badly)
    for two_char, replacement in [(k, v) for k, v in OCR_SUBS.items() if len(k) == 2]:
        if two_char in word and rng.random() < 0.3:
            word = word.replace(two_char, replacement, 1)
            return word
    # Single-char substitutions
    chars = list(word)
    for i, c in enumerate(chars):
        if c in OCR_SUBS and rng.random() < 0.15:
            chars[i] = OCR_SUBS[c]
    return "".join(chars)


def ocr_corrupt_text(text: str, corruption_rate: float, rng: random.Random) -> str:
    ws = text.split()
    result = []
    for w in ws:
        if rng.random() < corruption_rate:
            w = ocr_corrupt_word(w, rng)
        # Random character drop (very rare)
        if len(w) > 3 and rng.random() < 0.02:
            drop_pos = rng.randint(1, len(w) - 2)
            w = w[:drop_pos] + w[drop_pos+1:]
        # Adjacent char swap (rare)
        if len(w) > 3 and rng.random() < 0.01:
            i = rng.randint(0, len(w) - 2)
            w = w[:i] + w[i+1] + w[i] + w[i+2:]
        result.append(w)
    # Occasional word merge (remove space between two words)
    text_out = " ".join(result)
    text_out = re.sub(r" (?=[a-z]{1,4} )", lambda m: m.group() if rng.random() > 0.03 else "", text_out)
    return text_out


def build_ocr_corpus(target_bytes: int) -> list[str]:
    """Simulate OCR-scanned text: character confusions, merged words, drops."""
    sources = [
        os.path.join(STAGE_3_DIR, "bible.txt"),
        os.path.join(STAGE_6_DIR, "simple_wikipedia.txt"),
        os.path.join(STAGE_10_DIR, "dev_blogs.txt"),
    ]
    sources = [s for s in sources if os.path.exists(s)]
    texts = []
    written = 0

    print("  [chaos] OCR-corrupted corpus ...")
    for src_path in sources:
        if written >= target_bytes:
            break
        raw = read_corpus_file(src_path)
        paras = paragraphs(raw)
        RNG.shuffle(paras)

        for para in paras:
            if written >= target_bytes:
                break
            corruption = RNG.uniform(0.05, 0.20)
            corrupted = ocr_corrupt_text(para, corruption, RNG)
            if len(corrupted) > 60:
                texts.append(corrupted)
                written += len(corrupted.encode())

        print(f"  [chaos] OCR {os.path.basename(src_path)}: {written/1024/1024:.1f}MB total")

    return texts


# ── Chaos type 4: Open/unanswered questions ───────────────────────────────────

SE_SITES = [
    ("math",        "mathematics open problems"),
    ("cstheory",    "complexity theory open problems"),
    ("philosophy",  "unanswered philosophical questions"),
    ("linguistics", "unresolved linguistic questions"),
    ("physics",     "unsolved physics problems"),
    ("mathoverflow","open research problems"),
]


def fetch_se_unanswered(site: str, pages: int = 3) -> list[str]:
    """Fetch unanswered questions from StackExchange."""
    texts = []
    for page in range(1, pages + 1):
        try:
            params = {
                "order":    "desc",
                "sort":     "votes",
                "site":     site,
                "pagesize": 100,
                "page":     page,
                "filter":   "withbody",
            }
            raw  = fetch("https://api.stackexchange.com/2.3/questions/unanswered", params)
            data = json.loads(raw)
            for item in data.get("items", []):
                title = item.get("title", "")
                body  = strip_html(item.get("body", ""))
                if title and body and len(body) > 80:
                    # Format as question without answer — the key chaos signal
                    text = f"Question: {title}\n\n{body}\n\n[NO ACCEPTED ANSWER]"
                    texts.append(text)
            time.sleep(1.0)   # SE rate limit
            if not data.get("has_more", False):
                break
        except Exception as e:
            print(f"  [chaos] SE {site}: {e}")
            break
    return texts


def fetch_wikipedia_open_problems() -> list[str]:
    """Scrape Wikipedia's open problems pages — text that poses but never resolves."""
    open_problem_pages = [
        "List_of_unsolved_problems_in_mathematics",
        "List_of_unsolved_problems_in_computer_science",
        "List_of_unsolved_problems_in_physics",
        "List_of_unsolved_problems_in_linguistics",
        "List_of_unsolved_problems_in_neuroscience",
        "List_of_unsolved_problems_in_philosophy",
        "List_of_unsolved_problems_in_economics",
    ]
    texts = []
    base = "https://en.wikipedia.org/w/api.php"
    for page in open_problem_pages:
        try:
            params = {
                "action":      "query",
                "titles":      page,
                "prop":        "extracts",
                "exintro":     "false",
                "explaintext": "true",
                "format":      "json",
            }
            raw  = fetch(base, params)
            data = json.loads(raw)
            pages_data = data.get("query", {}).get("pages", {})
            for pid, pdata in pages_data.items():
                if pid == "-1":
                    continue
                extract = pdata.get("extract", "")
                if len(extract) > 200:
                    # Prepend marker so model knows this is unresolved territory
                    texts.append(f"[UNSOLVED PROBLEMS]\n\n{extract}")
                    print(f"  [chaos] Wikipedia open problems: {page} ({len(extract)//1024}KB)")
            time.sleep(0.5)
        except Exception as e:
            print(f"  [chaos] Wikipedia {page}: {e}")
    return texts


def build_open_questions_corpus(target_bytes: int) -> list[str]:
    texts = []
    written = 0

    print("  [chaos] Open/unanswered questions ...")

    # Wikipedia open problem pages first (no rate limit concern)
    wiki_texts = fetch_wikipedia_open_problems()
    for t in wiki_texts:
        texts.append(t)
        written += len(t.encode())

    # StackExchange unanswered questions
    for site, label in SE_SITES:
        if written >= target_bytes:
            break
        print(f"  [chaos] SE/{site} unanswered ...")
        batch = fetch_se_unanswered(site, pages=2)
        for t in batch:
            texts.append(t)
            written += len(t.encode())
        print(f"  [chaos] SE/{site}: +{len(batch)} · {written/1024/1024:.1f}MB total")
        time.sleep(2.0)

    return texts


# ── Chaos type 5: Code-switched multilingual ──────────────────────────────────

def build_codeswitched_corpus(target_bytes: int) -> list[str]:
    """
    Mix languages within paragraphs: inject non-English sentences into English
    paragraphs and vice versa. Realistic — multilingual humans do this.
    """
    lang_files = {}
    for lang in ["de", "fr", "es", "pt", "it", "nl", "pl"]:
        p = os.path.join(MULTI_DIR, f"{lang}.txt")
        if os.path.exists(p):
            lang_files[lang] = paragraphs(read_corpus_file(p))

    en_paras = []
    en_path = os.path.join(MULTI_DIR, "en.txt")
    if os.path.exists(en_path):
        en_paras = paragraphs(read_corpus_file(en_path))

    if not en_paras or not lang_files:
        print("  [chaos] Code-switched: insufficient source material, skipping")
        return []

    texts = []
    written = 0
    langs = list(lang_files.keys())
    print("  [chaos] Code-switched multilingual ...")

    RNG.shuffle(en_paras)
    for en_para in en_paras:
        if written >= target_bytes:
            break
        # Split English paragraph into sentences
        sents = sentences(en_para)
        if len(sents) < 3:
            continue

        # Inject 1-2 foreign sentences at random positions
        injections = RNG.randint(1, min(2, len(sents) - 1))
        result_sents = list(sents)
        for _ in range(injections):
            foreign_lang = RNG.choice(langs)
            foreign_paras = lang_files[foreign_lang]
            if not foreign_paras:
                continue
            foreign_para = RNG.choice(foreign_paras)
            foreign_sents = sentences(foreign_para)
            if not foreign_sents:
                continue
            foreign_sent = RNG.choice(foreign_sents)
            insert_pos = RNG.randint(1, len(result_sents) - 1)
            result_sents.insert(insert_pos, foreign_sent)

        mixed = " ".join(result_sents)
        if len(mixed) > 100:
            texts.append(mixed)
            written += len(mixed.encode())

    print(f"  [chaos] Code-switched: {len(texts)} paragraphs · {written/1024/1024:.1f}MB")
    return texts


# ── Chaos type 6: Masked academic ─────────────────────────────────────────────

def build_masked_academic(target_bytes: int) -> list[str]:
    """
    Academic abstracts with 20-40% word masking.
    Teaches: incomplete technical argument, unknown terminology.
    """
    academic_en = os.path.join(ACADEMIC_DIR, "en.txt")
    if not os.path.exists(academic_en):
        print("  [chaos] Masked academic: academic/en.txt not found, skipping")
        return []

    raw = read_corpus_file(academic_en)
    paras = paragraphs(raw)
    RNG.shuffle(paras)

    texts = []
    written = 0
    print("  [chaos] Masked academic ...")

    for para in paras:
        if written >= target_bytes:
            break
        # Vary masking intensity: 20%, 30%, or 40%
        rate = RNG.choice([0.20, 0.30, 0.40])
        masked = apply_word_masking(para, rate, RNG)
        if len(masked) > 60:
            texts.append(masked)
            written += len(masked.encode())

    print(f"  [chaos] Masked academic: {len(texts)} abstracts · {written/1024/1024:.1f}MB")
    return texts


# ── Chaos type 7: Failed trades + financial chaos ────────────────────────────

# Realistic trade setup templates (intentionally incomplete / no outcome)
TRADE_TEMPLATES = [
    "Setup: {sym} long at {entry:.2f}. Target {tp:.2f} ({tp_pct:.1f}%). Stop {sl:.2f}. "
    "Thesis: {thesis}. Watching for {trigger}.",

    "{sym} short idea — entry zone {entry:.2f}–{entry2:.2f}, target {tp:.2f}, "
    "invalidation above {sl:.2f}. Risk/reward {rr:.1f}:1. Catalyst: {thesis}.",

    "Trade alert: {sym} breakout above {entry:.2f} targeting {tp:.2f}. "
    "Tight stop at {sl:.2f}. Setup valid until {expiry}.",

    "Position update: {sym} @ {entry:.2f} — stop moved to {sl:.2f} (breakeven). "
    "Trade running but not yet at target. Current P&L:",

    "Closed {sym} position. Entry {entry:.2f}, exit {exit:.2f}. "
    "P&L: {pnl:.2f} ({pnl_pct:.1f}%). Reason: {reason}",

    "WATCHLIST {sym}: waiting for {condition}. If {trigger} then entry at {entry:.2f}, "
    "target {tp:.2f}. Setup has not triggered.",

    "Account update: {sym} down {loss:.1f}% from entry. Stop not hit yet. "
    "Original thesis {thesis} is now questionable.",
]

SYMBOLS = [
    "EURUSD", "BTCUSD", "GOLD", "SPX", "AAPL", "TSLA", "NVDA", "GBPUSD",
    "USDJPY", "CRUDE", "ETH", "SILVER", "NASDAQ", "DAX", "NIKKEI",
    "XRP", "SOL", "MSFT", "AMZN", "META",
]

THESES = [
    "momentum continuation after breakout",
    "support level holding",
    "earnings beat expected",
    "macro tailwind from rate decision",
    "technical pattern completion (H&S)",
    "liquidity grab below key level",
    "divergence on RSI at resistance",
    "failed breakdown back into range",
    "news catalyst not yet priced in",
    "institutional accumulation zone",
    "volume profile gap fill",
    "double bottom confirmed",
]

TRIGGERS = [
    "close above daily resistance",
    "volume spike on breakout",
    "rejection of 200 EMA",
    "FOMC decision",
    "earnings announcement",
    "CPI print",
    "4h candle close",
    "London session open",
    "retest of breakout level",
]

FAIL_REASONS = [
    "stop loss hit — market reversed",
    "invalidated by unexpected news",
    "trade never triggered — moved without me",
    "breakout was a fakeout",
    "risk management — cut early",
    "correlation breakdown with correlated asset",
    "slippage on stop execution",
    "pattern failed at the last moment",
    "macro event overrode the setup",
]


def make_trade_chunk(rng: random.Random) -> str:
    sym    = rng.choice(SYMBOLS)
    entry  = rng.uniform(50, 50000)
    spread = entry * rng.uniform(0.01, 0.05)
    tp     = entry + spread * rng.uniform(1.5, 4.0) * rng.choice([-1, 1])
    sl     = entry - spread * rng.uniform(0.5, 1.5) * rng.choice([-1, 1])
    rr     = abs(tp - entry) / max(abs(sl - entry), 0.01)
    tp_pct = abs(tp - entry) / entry * 100
    pnl    = entry - tp if rng.random() < 0.60 else tp - entry   # 60% losses
    pnl_pct = pnl / entry * 100
    loss   = abs(pnl_pct)

    tmpl = rng.choice(TRADE_TEMPLATES)
    try:
        text = tmpl.format(
            sym=sym, entry=entry, entry2=entry+spread*0.3,
            tp=tp, sl=sl, tp_pct=tp_pct, rr=rr,
            thesis=rng.choice(THESES), trigger=rng.choice(TRIGGERS),
            condition=rng.choice(TRIGGERS), expiry=f"EOD {rng.randint(1,28)}/{rng.randint(1,12)}",
            exit=tp, pnl=pnl, pnl_pct=pnl_pct, loss=loss,
            reason=rng.choice(FAIL_REASONS),
        )
    except KeyError:
        return ""
    return text


def fetch_sec_enforcement_actions(count: int = 50) -> list[str]:
    """
    Fetch SEC enforcement press releases — public record of failed / fraudulent
    financial actions. Each is a narrative of a trade or scheme that went wrong.
    """
    texts = []
    try:
        # SEC EDGAR full-text search API — enforcement actions
        params = {
            "q":          '"cease-and-desist" OR "fraud" OR "manipulation"',
            "dateRange":  "custom",
            "startdt":    "2015-01-01",
            "enddt":      "2024-12-31",
            "forms":      "34-12g4",
        }
        # Use SEC EDGAR press releases endpoint (no auth required)
        url = "https://efts.sec.gov/LATEST/search-index?q=%22fraud%22+%22trading%22&dateRange=custom&startdt=2020-01-01&enddt=2024-12-31&_source=period_of_report,file_date,display_names,period_of_report,file_num,form_type,biz_location,inc_states&hits.hits._source=period_of_report&hits.hits.total.value=true"
        raw  = fetch(url, retries=2)
        data = json.loads(raw)
        hits = data.get("hits", {}).get("hits", [])
        for h in hits[:count]:
            src = h.get("_source", {})
            name = src.get("display_names", [""])[0] if src.get("display_names") else ""
            form = src.get("form_type", "")
            date = src.get("period_of_report", "")
            if name:
                texts.append(f"SEC Filing: {form} filed {date}\nEntity: {name}\n"
                             f"[DOCUMENT CONTENT REDACTED — ENFORCEMENT ACTION]")
    except Exception as e:
        print(f"  [chaos] SEC API: {e}")

    # Fallback: SEC litigation releases RSS
    try:
        rss_url = "https://www.sec.gov/rss/litigation/litreleases.xml"
        raw  = fetch(rss_url, retries=2)
        raw_str = raw.decode("utf-8", errors="replace")
        # Extract titles and descriptions from RSS
        titles = re.findall(r"<title>(.*?)</title>", raw_str)
        descs  = re.findall(r"<description>(.*?)</description>", raw_str, re.DOTALL)
        for title, desc in zip(titles[2:], descs[2:]):   # skip channel-level items
            clean_title = strip_html(title)
            clean_desc  = strip_html(desc)
            if len(clean_desc) > 80:
                texts.append(f"SEC Litigation Release: {clean_title}\n\n{clean_desc}")
        print(f"  [chaos] SEC RSS: {len(texts)} enforcement actions")
    except Exception as e:
        print(f"  [chaos] SEC RSS: {e}")

    return texts


def build_financial_chaos(target_bytes: int) -> list[str]:
    """
    Failed trades, incomplete setups, and SEC enforcement actions.

    The market is the ultimate chaos signal: a well-reasoned setup can fail,
    a confident thesis can be wrong, a position can stop out at the worst moment.
    Teaching the model this prevents it from assuming all structured reasoning
    resolves correctly.

    Sources:
    - Synthetic trade setups (60% losing, 40% winning/incomplete — realistic)
    - SEC enforcement press releases (real-world failed financial schemes)
    - Truncated position updates (trade in progress, no conclusion)
    """
    texts = []
    written = 0

    print("  [chaos] Financial chaos (failed trades + SEC enforcement) ...")

    # Synthetic trade setups — fast, local
    n_synthetic = max(200, target_bytes // 400)
    for _ in range(n_synthetic):
        if written >= target_bytes * 0.70:   # synthetic covers 70% of budget
            break
        chunk = make_trade_chunk(RNG)
        if chunk:
            texts.append(chunk)
            written += len(chunk.encode())

    print(f"  [chaos] synthetic trades: {len(texts)} · {written/1024/1024:.2f}MB")

    # SEC enforcement actions (public record of bad trades)
    if written < target_bytes:
        sec_texts = fetch_sec_enforcement_actions(100)
        for t in sec_texts:
            texts.append(t)
            written += len(t.encode())
        print(f"  [chaos] SEC actions: +{len(sec_texts)} · {written/1024/1024:.2f}MB total")

    # Pad with truncated trade narratives if needed
    if written < target_bytes:
        stage10_files = [
            os.path.join(STAGE_10_DIR, "hn_discussions.txt"),
            os.path.join(STAGE_10_DIR, "github_bugs.txt"),
        ]
        for src in stage10_files:
            if written >= target_bytes or not os.path.exists(src):
                continue
            raw   = read_corpus_file(src)
            paras = paragraphs(raw)
            RNG.shuffle(paras)
            for para in paras[:200]:
                if written >= target_bytes:
                    break
                # Inject financial framing + truncate — simulate a trade gone wrong
                prefix = f"[SETUP NOTE — {RNG.choice(SYMBOLS)}]\n"
                truncated = truncate_at(prefix + para, RNG.uniform(0.3, 0.6))
                if len(truncated) > 60:
                    texts.append(truncated)
                    written += len(truncated.encode())

    print(f"  [chaos] Financial chaos total: {len(texts)} items · {written/1024/1024:.1f}MB")
    return texts


# ── Multilingual chaos file ────────────────────────────────────────────────────

def build_multilingual_chaos(target_bytes: int) -> list[str]:
    """
    Apply redaction and truncation to non-English Wikipedia corpus.
    Same chaos types as English but across all 7 other languages.
    """
    texts = []
    written = 0
    per_lang = target_bytes // 7

    print("  [chaos] Multilingual chaos ...")
    for lang in ["de", "fr", "es", "pt", "it", "nl", "pl"]:
        if written >= target_bytes:
            break
        p = os.path.join(MULTI_DIR, f"{lang}.txt")
        if not os.path.exists(p):
            continue
        raw = read_corpus_file(p)
        paras = paragraphs(raw)
        RNG.shuffle(paras)
        lang_written = 0

        for para in paras:
            if lang_written >= per_lang:
                break
            mode = RNG.random()
            if mode < 0.4:
                corrupted = apply_word_masking(para, RNG.uniform(0.10, 0.30), RNG)
            elif mode < 0.65:
                frac = RNG.uniform(0.3, 0.7)
                corrupted = truncate_at(para, frac)
            elif mode < 0.80:
                corrupted = apply_sentence_redaction(para, 0.2, RNG)
            else:
                corrupted = ocr_corrupt_text(para, 0.10, RNG)

            if len(corrupted) > 40:
                texts.append(corrupted)
                lang_written += len(corrupted.encode())
                written     += len(corrupted.encode())

        print(f"  [chaos] [{lang}] {lang_written//1024}KB · total {written/1024/1024:.1f}MB")

    return texts


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--target-mb", type=int, default=DEFAULT_TARGET_MB)
    parser.add_argument("--seed",      type=int, default=42)
    args = parser.parse_args()

    global RNG
    RNG = random.Random(args.seed)

    os.makedirs(OUT_DIR, exist_ok=True)
    target_bytes = args.target_mb * 1024 * 1024

    # Budget allocation (percentages of target)
    budgets = {
        "redacted":    int(target_bytes * 0.27),   # 27% — Bible + stage corpus redaction
        "truncated":   int(target_bytes * 0.20),   # 20% — cut-off reasoning
        "ocr":         int(target_bytes * 0.10),   # 10% — OCR noise
        "open_q":      int(target_bytes * 0.15),   # 15% — unanswered questions
        "codeswitched":int(target_bytes * 0.08),   # 8%  — language mixing
        "masked_acad": int(target_bytes * 0.10),   # 10% — masked academic
        "financial":   int(target_bytes * 0.10),   # 10% — failed trades + SEC enforcement
    }

    print(f"\n=== Chaos Protocol: target {args.target_mb}MB ===")
    print("Budget allocation:")
    for k, v in budgets.items():
        print(f"  {k}: {v/1024/1024:.1f}MB")
    print()

    # ── English chaos ──────────────────────────────────────────────────────────
    en_path = os.path.join(OUT_DIR, "chaos_en.txt")
    if os.path.exists(en_path) and os.path.getsize(en_path) > 512 * 1024:
        mb = os.path.getsize(en_path) / 1024 / 1024
        print(f"chaos_en.txt already exists ({mb:.1f}MB) — skipping English chaos")
    else:
        all_en = []

        t1 = build_redacted_corpus(budgets["redacted"])
        print(f"  redacted: {len(t1)} chunks")
        all_en.extend(t1)

        t2 = build_truncated_corpus(budgets["truncated"])
        print(f"  truncated: {len(t2)} chunks")
        all_en.extend(t2)

        t3 = build_ocr_corpus(budgets["ocr"])
        print(f"  ocr: {len(t3)} chunks")
        all_en.extend(t3)

        t4 = build_open_questions_corpus(budgets["open_q"])
        print(f"  open_questions: {len(t4)} items")
        all_en.extend(t4)

        t5 = build_codeswitched_corpus(budgets["codeswitched"])
        print(f"  code_switched: {len(t5)} paragraphs")
        all_en.extend(t5)

        t6 = build_masked_academic(budgets["masked_acad"])
        print(f"  masked_academic: {len(t6)} abstracts")
        all_en.extend(t6)

        t7 = build_financial_chaos(budgets["financial"])
        print(f"  financial_chaos: {len(t7)} items")
        all_en.extend(t7)

        RNG.shuffle(all_en)   # Mix chaos types — don't cluster them

        total_bytes = sum(len(t.encode()) for t in all_en)
        with open(en_path, "w", encoding="utf-8") as f:
            for chunk in all_en:
                f.write(chunk + "\n\n")

        print(f"\nchaos_en.txt: {len(all_en):,} chunks · {total_bytes/1024/1024:.1f}MB → {en_path}")

    # ── Multilingual chaos ─────────────────────────────────────────────────────
    multi_path = os.path.join(OUT_DIR, "chaos_multi.txt")
    multi_target = 5 * 1024 * 1024
    if os.path.exists(multi_path) and os.path.getsize(multi_path) > 512 * 1024:
        mb = os.path.getsize(multi_path) / 1024 / 1024
        print(f"chaos_multi.txt already exists ({mb:.1f}MB) — skipping")
    else:
        multi_chunks = build_multilingual_chaos(multi_target)
        RNG.shuffle(multi_chunks)
        total_bytes = sum(len(t.encode()) for t in multi_chunks)
        with open(multi_path, "w", encoding="utf-8") as f:
            for chunk in multi_chunks:
                f.write(chunk + "\n\n")
        print(f"\nchaos_multi.txt: {len(multi_chunks):,} chunks · {total_bytes/1024/1024:.1f}MB → {multi_path}")

    # ── Summary ────────────────────────────────────────────────────────────────
    print("\n=== Chaos Corpus Summary ===")
    total = 0
    for fname in ["chaos_en.txt", "chaos_financial.txt", "chaos_multi.txt"]:
        p = os.path.join(OUT_DIR, fname)
        if os.path.exists(p):
            mb = os.path.getsize(p) / 1024 / 1024
            total += os.path.getsize(p)
            print(f"  {fname}: {mb:.1f}MB")
    print(f"  TOTAL: {total/1024/1024:.1f}MB")
    print(f"\nChaos corpus ready at: {OUT_DIR}")
    print("It will be picked up automatically by train_tokenizer_v3.py")


if __name__ == "__main__":
    main()
