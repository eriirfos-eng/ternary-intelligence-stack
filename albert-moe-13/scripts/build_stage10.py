#!/usr/bin/env python3
"""
Stage 10 corpus builder — "Long Solved Problems" + Human Internet Register
Albert MoE-13 training data pipeline

Sources (all free, no API keys required):
  1. Stack Exchange API  — cross-domain long-solved problem threads
  2. iFixit API          — appliance & device repair guides
  3. GitHub Issues API   — long-running technical bugs (optional token)
  4. dev.to API          — developer blogs (informal register + emojis)
  5. HN Algolia API      — Hacker News discussion threads

Output: data/corpus/stage_10/*.txt (plain text, [PROBLEM]/[ATTEMPTS]/[RESOLUTION] format)

Run:
    cd albert-moe-13
    python scripts/build_stage10.py

Optional GitHub token (raises rate limit 60→5000 req/hour):
    GITHUB_TOKEN=ghp_xxx python scripts/build_stage10.py
"""

import json
import os
import re
import sys
import time
import urllib.request
import urllib.parse
from datetime import datetime, timezone
from pathlib import Path

# ── paths ────────────────────────────────────────────────────────────────────
SCRIPT_DIR = Path(__file__).parent
CORPUS_DIR = SCRIPT_DIR.parent / "data" / "corpus" / "stage_10"
CORPUS_DIR.mkdir(parents=True, exist_ok=True)

UA = "AlbertMoE13CorpusBot/1.0 (training corpus; contact rfi.irfos@gmail.com)"

# ── tuning ───────────────────────────────────────────────────────────────────
SE_MIN_SCORE        = 5      # question quality floor
SE_MIN_ANSWERS      = 2      # need at least 2 attempts before resolution
SE_MAX_PER_SITE     = 150    # questions per SE site
IFIXIT_MAX_GUIDES   = 250    # repair guides to harvest
GH_MIN_COMMENTS     = 8      # min comments for a GitHub issue to be interesting
GH_MIN_DAYS_OPEN    = 14     # min days open before closed (shows struggle)
GH_MAX_PER_REPO     = 25     # issues per repo
DEVTO_MAX_ARTICLES  = 400    # dev.to blog posts
HN_MAX_STORIES      = 150    # HN threads

# Stack Exchange sites — chosen for cross-domain breadth
SE_SITES = [
    ("mechanics",        "Vehicle Repair"),
    ("diy",              "Home Improvement & DIY"),
    ("cooking",          "Cooking & Food"),
    ("electronics",      "Electronics & Hardware"),
    ("unix",             "Unix & Linux"),
    ("askubuntu",        "Ubuntu Linux"),
    ("superuser",        "Computer Hardware & Software"),
    ("music",            "Music Identification & Theory"),
    ("movies",           "Movies & TV Identification"),
    ("gardening",        "Gardening & Plants"),
    ("bicycles",         "Bicycle Repair"),
    ("woodworking",      "Woodworking & Crafts"),
    ("3dprinting",       "3D Printing Troubleshooting"),
    ("photography",      "Photography & Cameras"),
    ("ham",              "Amateur Radio"),
    ("homebrew",         "Homebrewing"),
    ("outdoors",         "Outdoor Activities"),
    ("pets",             "Pet Care & Veterinary"),
    ("aviation",         "Aviation"),
    ("sound",            "Audio & Sound Design"),
    ("engineering",      "Engineering"),
    ("interpersonal",    "Interpersonal Problem Solving"),
    ("retrocomputing",   "Retro Computing"),
    ("skeptics",         "Claims & Fact-Finding"),
]

# GitHub repos with rich long-running issue histories
GH_REPOS = [
    "rust-lang/rust",
    "python/cpython",
    "golang/go",
    "microsoft/TypeScript",
    "nodejs/node",
    "torvalds/linux",
    "vim/vim",
    "neovim/neovim",
    "apple/swift",
    "llvm/llvm-project",
    "docker/compose",
    "kubernetes/kubernetes",
    "ansible/ansible",
    "home-assistant/core",
    "jellyfin/jellyfin",
]


# ── utilities ────────────────────────────────────────────────────────────────

def log(msg: str):
    ts = datetime.now().strftime("%H:%M:%S")
    print(f"[{ts}] {msg}", flush=True)

def fetch(url: str, *, delay: float = 0.5, retries: int = 3) -> bytes | None:
    for attempt in range(retries):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": UA})
            with urllib.request.urlopen(req, timeout=20) as r:
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

_HTML_ENTITIES = {"&amp;": "&", "&lt;": "<", "&gt;": ">",
                  "&quot;": '"', "&apos;": "'", "&#39;": "'",
                  "&nbsp;": " ", "&#160;": " "}

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

def ts_to_dt(ts: int) -> datetime:
    return datetime.fromtimestamp(ts, tz=timezone.utc)

def days_between(ts1: int, ts2: int) -> int:
    return abs(int((ts2 - ts1) / 86400))


# ── 1. Stack Exchange ────────────────────────────────────────────────────────

SE_BASE = "https://api.stackexchange.com/2.3"

def se_url(path: str, params: dict) -> str:
    params["key"] = ""      # anonymous — 300 req/day; add your app key here for 10k
    qs = urllib.parse.urlencode({k: v for k, v in params.items() if v != ""})
    return f"{SE_BASE}{path}?{qs}"

def fetch_se_questions(site: str, page: int = 1) -> list[dict]:
    url = se_url("/questions", {
        "site":     site,
        "page":     page,
        "pagesize": 100,
        "sort":     "votes",
        "order":    "desc",
        "filter":   "withbody",
        "answers":  "true",
        "min":      SE_MIN_SCORE,
    })
    data = fetch_json(url, delay=1.2)
    if data and "items" in data:
        return data["items"]
    return []

def fetch_se_answers(site: str, question_id: int) -> list[dict]:
    url = se_url(f"/questions/{question_id}/answers", {
        "site":     site,
        "pagesize": 30,
        "sort":     "votes",
        "order":    "desc",
        "filter":   "withbody",
    })
    data = fetch_json(url, delay=0.8)
    if data and "items" in data:
        return data["items"]
    return []

def fetch_se_comments(site: str, question_id: int) -> list[dict]:
    url = se_url(f"/questions/{question_id}/comments", {
        "site":     site,
        "pagesize": 30,
        "sort":     "creation",
        "order":    "asc",
        "filter":   "withbody",
    })
    data = fetch_json(url, delay=0.8)
    if data and "items" in data:
        return data["items"]
    return []

def format_se_thread(q: dict, answers: list[dict], comments: list[dict],
                     site_label: str) -> str | None:
    if not q.get("accepted_answer_id"):
        return None
    if len(answers) < SE_MIN_ANSWERS:
        return None

    title  = q.get("title", "").strip()
    body   = strip_html(q.get("body", ""))
    tags   = " | ".join(q.get("tags", []))
    score  = q.get("score", 0)
    views  = q.get("view_count", 0)
    q_ts   = q.get("creation_date", 0)
    acc_id = q["accepted_answer_id"]

    # Find accepted answer
    accepted = next((a for a in answers if a["answer_id"] == acc_id), None)
    if not accepted:
        return None

    resolution_days = days_between(q_ts, accepted.get("creation_date", q_ts))
    non_accepted    = [a for a in answers if a["answer_id"] != acc_id]

    lines = [
        f"══ LONG-SOLVED PROBLEM ══",
        f"Source   : Stack Exchange / {site_label}",
        f"Title    : {title}",
        f"Tags     : {tags}",
        f"Stats    : score {score} | {views:,} views | {len(answers)} answers "
        f"| {len(comments)} comments | resolved in {resolution_days} days",
        "",
        "[PROBLEM]",
        body[:2000],  # cap body length
        "",
        "[ATTEMPTS]",
    ]

    # Comments on the question (early back-and-forth)
    for c in comments[:8]:
        author = c.get("owner", {}).get("display_name", "user")
        text   = strip_html(c.get("body", ""))
        if text:
            lines.append(f"> {author}: {text}")

    # Non-accepted answers in vote order (these are the "failed" attempts)
    for i, ans in enumerate(non_accepted[:4], 1):
        author   = ans.get("owner", {}).get("display_name", "user")
        ans_body = strip_html(ans.get("body", ""))[:800]
        ans_score= ans.get("score", 0)
        lines.append(f"\nAttempt {i} by {author} (score: {ans_score}):")
        lines.append(ans_body)

    # Resolution
    acc_author = accepted.get("owner", {}).get("display_name", "user")
    acc_body   = strip_html(accepted.get("body", ""))[:1500]
    acc_score  = accepted.get("score", 0)

    lines += [
        "",
        f"[RESOLUTION — {resolution_days} days later, accepted answer by {acc_author}, score {acc_score}]",
        acc_body,
    ]

    return "\n".join(lines)

def build_stackexchange(out_path: Path):
    log("Stack Exchange — building long-solved problem threads ...")
    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for site, label in SE_SITES:
            log(f"  site: {site} ({label})")
            site_count = 0
            page = 1
            while site_count < SE_MAX_PER_SITE:
                questions = fetch_se_questions(site, page)
                if not questions:
                    break
                for q in questions:
                    if site_count >= SE_MAX_PER_SITE:
                        break
                    if not q.get("accepted_answer_id"):
                        continue
                    if q.get("answer_count", 0) < SE_MIN_ANSWERS:
                        continue
                    qid     = q["question_id"]
                    answers  = fetch_se_answers(site, qid)
                    comments = fetch_se_comments(site, qid)
                    block    = format_se_thread(q, answers, comments, label)
                    if block:
                        write_block(fh, block)
                        site_count += 1
                        total += 1
                page += 1
                if page > 5:
                    break
            log(f"    → {site_count} threads")
    log(f"Stack Exchange done — {total} total threads → {out_path.name}")


# ── 2. iFixit repair guides ──────────────────────────────────────────────────

IFIXIT_BASE = "https://www.ifixit.com/api/2.0"

def format_guide(guide: dict) -> str | None:
    title    = guide.get("title", "").strip()
    category = guide.get("category", "")
    subject  = guide.get("subject", "")
    intro    = strip_html(guide.get("introduction_rendered", ""))
    steps    = guide.get("steps", [])
    if not steps:
        return None

    lines = [
        f"══ REPAIR GUIDE ══",
        f"Device  : {subject} — {category}",
        f"Task    : {title}",
        f"Steps   : {len(steps)}",
    ]

    if intro:
        lines += ["", "[PROBLEM / CONTEXT]", intro[:600]]

    lines += ["", "[PROCEDURE]"]
    for i, step in enumerate(steps, 1):
        step_lines = step.get("lines", [])
        step_text  = " ".join(
            strip_html(ln.get("text_rendered", ""))
            for ln in step_lines
            if ln.get("bullet") not in ("icon_note", "icon_caution")
        ).strip()
        if step_text:
            lines.append(f"Step {i}: {step_text[:400]}")

    outro = strip_html(guide.get("conclusion_rendered", ""))
    if outro:
        lines += ["", "[RESOLUTION / NOTES]", outro[:400]]

    return "\n".join(lines)

def build_ifixit(out_path: Path):
    log("iFixit — fetching repair guides ...")
    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        offset = 0
        while total < IFIXIT_MAX_GUIDES:
            url  = f"{IFIXIT_BASE}/guides?limit=50&offset={offset}"
            data = fetch_json(url, delay=0.8)
            if not data:
                break
            for entry in data:
                if total >= IFIXIT_MAX_GUIDES:
                    break
                guide_id = entry.get("guideid")
                if not guide_id:
                    continue
                full = fetch_json(f"{IFIXIT_BASE}/guides/{guide_id}", delay=0.6)
                if not full:
                    continue
                block = format_guide(full)
                if block:
                    write_block(fh, block)
                    total += 1
            offset += 50
            if len(data) < 50:
                break
    log(f"iFixit done — {total} guides → {out_path.name}")


# ── 3. GitHub Issues ─────────────────────────────────────────────────────────

GH_BASE   = "https://api.github.com"
GH_TOKEN  = os.getenv("GITHUB_TOKEN", "")
GH_HEADERS = {"User-Agent": UA, "Accept": "application/vnd.github+json"}
if GH_TOKEN:
    GH_HEADERS["Authorization"] = f"token {GH_TOKEN}"

def gh_get(path: str) -> dict | list | None:
    url = f"{GH_BASE}{path}"
    raw = None
    for attempt in range(3):
        try:
            req = urllib.request.Request(url, headers=GH_HEADERS)
            with urllib.request.urlopen(req, timeout=20) as r:
                raw = r.read()
            time.sleep(0.8 if GH_TOKEN else 2.0)
            return json.loads(raw)
        except Exception as e:
            if "403" in str(e) or "429" in str(e):
                log("  GitHub rate limit hit — sleeping 60s")
                time.sleep(60)
            elif attempt < 2:
                time.sleep(3)
            else:
                log(f"  GitHub fetch failed: {e}")
    return None

def format_gh_issue(issue: dict, comments: list[dict], repo: str) -> str | None:
    title     = issue.get("title", "").strip()
    body      = strip_html(issue.get("body") or "")
    created   = issue.get("created_at", "")
    closed    = issue.get("closed_at", "")
    n_comments= issue.get("comments", 0)

    if n_comments < GH_MIN_COMMENTS:
        return None

    if created and closed:
        try:
            c1 = datetime.fromisoformat(created.replace("Z", "+00:00"))
            c2 = datetime.fromisoformat(closed.replace("Z", "+00:00"))
            days_open = (c2 - c1).days
        except Exception:
            days_open = 0
        if days_open < GH_MIN_DAYS_OPEN:
            return None
    else:
        days_open = "?"

    labels = ", ".join(lb["name"] for lb in issue.get("labels", []))
    url    = issue.get("html_url", "")

    lines = [
        f"══ GITHUB BUG THREAD ══",
        f"Repo    : {repo}",
        f"Issue   : {title}",
        f"Labels  : {labels or 'none'}",
        f"Stats   : {n_comments} comments | open {days_open} days",
        f"URL     : {url}",
        "",
        "[PROBLEM]",
        body[:1500],
        "",
        "[DISCUSSION / ATTEMPTS]",
    ]

    for c in comments[:12]:
        author = c.get("user", {}).get("login", "user")
        text   = strip_html(c.get("body") or "")[:600]
        if text:
            lines.append(f"\n@{author}:")
            lines.append(text)

    return "\n".join(lines)

def build_github(out_path: Path):
    log("GitHub — fetching long-running resolved issues ...")
    total = 0
    with open(out_path, "w", encoding="utf-8") as fh:
        for repo in GH_REPOS:
            log(f"  repo: {repo}")
            repo_count = 0
            page = 1
            while repo_count < GH_MAX_PER_REPO:
                issues = gh_get(
                    f"/repos/{repo}/issues?state=closed&sort=created"
                    f"&direction=asc&per_page=50&page={page}"
                )
                if not issues or not isinstance(issues, list):
                    break
                for issue in issues:
                    if repo_count >= GH_MAX_PER_REPO:
                        break
                    if "pull_request" in issue:
                        continue
                    iid      = issue["number"]
                    comments = gh_get(f"/repos/{repo}/issues/{iid}/comments?per_page=50")
                    block    = format_gh_issue(issue, comments or [], repo)
                    if block:
                        write_block(fh, block)
                        repo_count += 1
                        total += 1
                page += 1
                if page > 5 or len(issues) < 50:
                    break
            log(f"    → {repo_count} issues")
    log(f"GitHub done — {total} total issues → {out_path.name}")


# ── 4. dev.to blogs (informal register + emojis) ────────────────────────────

DEVTO_BASE = "https://dev.to/api"

# Tags that produce informal, emoji-rich, conversational posts
DEVTO_TAGS = [
    "discuss", "explainlikeimfive", "showdev", "watercooler",
    "weeklyretro", "career", "beginners", "help", "rant",
    "productivity", "opensource", "webdev", "javascript",
    "python", "rust", "devops", "linux", "terminal",
]

def format_devto_article(art: dict) -> str | None:
    title       = art.get("title", "").strip()
    description = art.get("description", "").strip()
    tags        = ", ".join(art.get("tag_list", []))
    reactions   = art.get("positive_reactions_count", 0)
    comments    = art.get("comments_count", 0)
    reading_min = art.get("reading_time_minutes", 0)
    author      = art.get("user", {}).get("name", "")
    body        = strip_html(art.get("body_html", art.get("body_markdown", "")))

    if len(body) < 200:
        return None

    lines = [
        f"══ DEVELOPER BLOG POST ══",
        f"Author  : {author}",
        f"Title   : {title}",
        f"Tags    : {tags}",
        f"Stats   : ♥ {reactions} | 💬 {comments} comments | ~{reading_min} min read",
        "",
        description,
        "",
        body[:3000],
    ]
    return "\n".join(lines)

def build_devto(out_path: Path):
    log("dev.to — fetching blog posts ...")
    total   = 0
    seen    = set()
    with open(out_path, "w", encoding="utf-8") as fh:
        for tag in DEVTO_TAGS:
            if total >= DEVTO_MAX_ARTICLES:
                break
            log(f"  tag: #{tag}")
            for page in range(1, 6):
                if total >= DEVTO_MAX_ARTICLES:
                    break
                url  = f"{DEVTO_BASE}/articles?tag={tag}&page={page}&per_page=30"
                data = fetch_json(url, delay=0.6)
                if not data:
                    break
                for art in data:
                    art_id = art.get("id")
                    if not art_id or art_id in seen:
                        continue
                    seen.add(art_id)
                    # Fetch full article body
                    full = fetch_json(f"{DEVTO_BASE}/articles/{art_id}", delay=0.5)
                    if not full:
                        continue
                    block = format_devto_article(full)
                    if block:
                        write_block(fh, block)
                        total += 1
                if len(data) < 30:
                    break
    log(f"dev.to done — {total} articles → {out_path.name}")


# ── 5. Hacker News (Algolia API) ─────────────────────────────────────────────

HN_BASE = "https://hn.algolia.com/api/v1"

HN_QUERIES = [
    "how did you solve",
    "finally fixed",
    "root cause found",
    "after months debugging",
    "turns out the problem was",
    "solved it turned out",
    "bug finally found",
    "mystery solved",
    "ask hn how do you",
    "ask hn what was the hardest",
]

def format_hn_thread(story: dict, hits: list[dict]) -> str | None:
    title    = story.get("title", "").strip()
    author   = story.get("author", "")
    url      = story.get("url", "")
    points   = story.get("points", 0)
    body     = story.get("story_text") or ""
    body     = strip_html(body)[:800]

    if not title:
        return None

    lines = [
        f"══ HACKER NEWS DISCUSSION ══",
        f"Title   : {title}",
        f"Author  : {author}",
        f"Points  : {points}",
    ]
    if url:
        lines.append(f"URL     : {url}")
    if body:
        lines += ["", body]
    lines += ["", "[DISCUSSION]"]

    # Top comments
    comments = sorted(hits, key=lambda h: h.get("points", 0) or 0, reverse=True)
    for c in comments[:15]:
        if c.get("author") == story.get("author") and not c.get("comment_text"):
            continue
        text = strip_html(c.get("comment_text") or "")[:500]
        if text:
            lines.append(f"\n@{c.get('author', 'hn_user')}:")
            lines.append(text)

    return "\n".join(lines)

def build_hackernews(out_path: Path):
    log("Hacker News — fetching discussion threads ...")
    total = 0
    seen  = set()
    with open(out_path, "w", encoding="utf-8") as fh:
        for query in HN_QUERIES:
            if total >= HN_MAX_STORIES:
                break
            q = urllib.parse.quote(query)
            url = f"{HN_BASE}/search?query={q}&tags=story&hitsPerPage=20"
            data = fetch_json(url, delay=0.8)
            if not data:
                continue
            for hit in data.get("hits", []):
                story_id = hit.get("objectID")
                if not story_id or story_id in seen:
                    continue
                seen.add(story_id)
                # Fetch full thread
                thread = fetch_json(f"{HN_BASE}/items/{story_id}", delay=0.5)
                if not thread:
                    continue
                children = thread.get("children", [])
                # Flatten all comments
                all_comments = []
                def flatten(nodes):
                    for n in nodes:
                        all_comments.append(n)
                        flatten(n.get("children", []))
                flatten(children)
                block = format_hn_thread(thread, all_comments)
                if block:
                    write_block(fh, block)
                    total += 1
    log(f"Hacker News done — {total} threads → {out_path.name}")


# ── 6. Emoji & informal register supplement ──────────────────────────────────
#
# Curated examples of how humans actually write online — emoji in context,
# casual problem descriptions, celebratory resolutions.
# Small but high-density signal for informal register learning.

EMOJI_EXAMPLES = [
    ("washing machine", """
User: hey so my washing machine (Samsung WF45R6100AW) just started showing E4 error 💀
anyone know what this means? tried googling but got 50 different answers lol

Reply 1: E4 is usually a water inlet issue! Check if the hot water hose is kinked 🚰
Reply 2: Same happened to me!! turned out the mesh filter on the inlet was CLOGGED with sediment
OP: omg just checked the filter and it was absolutely disgusting 😭 gonna clean it now brb
OP UPDATE: CLEANED IT AND IT WORKS 🎉🎉🎉 thank you so much!! been dealing with this for 3 weeks lol
"""),
    ("phone slow", """
User: my phone has been SO slow lately, like unbearably slow 😭 it's a Pixel 6a running Android 14
everything lags, apps take forever to open, battery dies super fast too
tried restarting, didn't help. anyone have ideas??

Reply 1: have you tried clearing the cache partition? that fixed mine after the Android 14 update
Reply 2: Go to Settings > Apps > see if something is using insane battery in the background
Reply 3: check if Google Play Services is updating, that kills performance temporarily ☠️
OP: @Reply2 FOUND IT — some random weather app was using 47% battery in the background 😭😭
deleted it and my phone is back to normal. literally runs like new now 🙏 thank you!!
"""),
    ("song identification", """
User: okay this is gonna sound insane but im trying to find a song i heard like 3 years ago 😅
all i remember is: it was kinda slow, electronic, had a female vocalist, the main lyric was
something like "running through the night" or "running to you" or something with "running"
it had a very dreamy vibe, not really pop, more like indie electronic?? please help 🙏🙏

Reply 1: Could it be Chvrches? They have a few songs that fit that vibe
Reply 2: "Running With The Wolves" by Aurora maybe?
Reply 3: Sounds like it could be Flume or Odesza style
OP: @Reply2 OMG THAT'S IT THAT'S THE SONG 😭😭😭 RUNNING WITH THE WOLVES - AURORA
i've been looking for this for LITERAL YEARS thank you so much i'm crying a little 🥺
"""),
    ("code bug", """
User: spent 6 hours on this bug today and i want to flip a table 🙃
the weirdest thing: my async function works perfectly in development but crashes in production
with "cannot read properties of undefined (reading 'map')" but ONLY sometimes
like 20% of requests fail, 80% are fine??? race condition maybe??

Reply 1: classic race condition 😅 what's the async operation and what are you mapping over?
Reply 2: are you awaiting everything properly? missing await is my #1 source of these bugs lol
Reply 3: could be an API returning inconsistent response shapes? add a console.log of the full response
OP: UPDATE: @Reply3 you were right!! the external API was sometimes returning {data: [...]}
and sometimes just returning [...] directly 💀 added a normalization step and it's been
100% stable for the last 2 hours. i cannot believe i missed that. thanks everyone ❤️
"""),
    ("laptop overheating", """
User: my laptop (ThinkPad T480) is running SO hot lately, like 95°C under any load 🔥
fans are spinning full blast constantly, performance has tanked
bought it 4 years ago, worked great until like 2 months ago, what happened??

Reply 1: thermal paste!! 4 years is exactly when it starts drying out
Reply 2: also check if the fan vents are clogged with dust, that's usually step 1
Reply 3: ThinkPads are super DIY friendly, repasting is like 30 min job
OP: cleaned the vents first — SO much dust came out 😭 you could knit a sweater from it
OP UPDATE: repasted with Arctic MX-4, now running at 62°C under the same load 🎉
from 95°C to 62°C just from paste + dust cleaning. should have done this ages ago lmao
"""),
]

def build_emoji_supplement(out_path: Path):
    log("Writing emoji/informal register supplement ...")
    with open(out_path, "w", encoding="utf-8") as fh:
        for topic, content in EMOJI_EXAMPLES:
            block = f"══ INFORMAL PROBLEM SOLVING — {topic.upper()} ══\n{content.strip()}"
            write_block(fh, block)

        # Also include a short explainer of emoji semantic role
        fh.write(
            "══ NOTE ON INFORMAL REGISTER ══\n"
            "In informal online communication, emoji serve semantic functions:\n"
            "😭 — expressing frustration or overwhelm\n"
            "🎉 — celebrating a resolution\n"
            "💀 / ☠️ — hyperbolic reaction to a bad situation\n"
            "🙏 — gratitude\n"
            "🔥 — something is very hot (literal or figurative)\n"
            "lol / lmao — hedging; reduces intensity of complaints\n"
            "'UPDATE:' — signals resolution after failed attempts\n"
            "ALL CAPS — emphasis on an emotional peak moment\n"
            "The pattern: problem description → community attempts → OP resolution update\n"
            "mirrors the [PROBLEM][ATTEMPTS][RESOLUTION] structure of formal debugging.\n"
        )
    log(f"Emoji supplement done → {out_path.name}")


# ── main ─────────────────────────────────────────────────────────────────────

def main():
    log("=" * 60)
    log("Stage 10 corpus builder — starting")
    log(f"Output: {CORPUS_DIR}")
    log(f"GitHub token: {'present ✓' if GH_TOKEN else 'absent (60 req/hour limit)'}")
    log("=" * 60)

    tasks = [
        ("long_solved_threads.txt",   build_stackexchange),
        ("repair_guides.txt",         build_ifixit),
        ("github_bugs.txt",           build_github),
        ("dev_blogs.txt",             build_devto),
        ("hn_discussions.txt",        build_hackernews),
        ("emoji_informal_register.txt", build_emoji_supplement),
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
            import traceback
            traceback.print_exc()

    log("")
    log("=" * 60)
    log("All sources complete. Stage 10 corpus:")
    total_bytes = 0
    for f in sorted(CORPUS_DIR.iterdir()):
        if f.suffix == ".txt":
            size = f.stat().st_size
            total_bytes += size
            log(f"  {f.name:45s}  {size / 1024 / 1024:.2f} MB")
    log(f"  {'TOTAL':45s}  {total_bytes / 1024 / 1024:.2f} MB")
    log("=" * 60)

if __name__ == "__main__":
    main()
