#!/usr/bin/env python3
"""
Stage 10 — Trail & Travel corpus builder
Albert MoE-13 training data pipeline

Sources (all free, no API keys required):
  1. Wikivoyage API  — travel guides, hiking sections, "get around" directions
  2. Wikipedia       — famous long-distance trail route descriptions
  3. NPS.gov         — US National Park trail pages (public domain)

Output: data/corpus/stage_10/trails_and_travel.txt

Run (from albert-moe-13/):
    python3 scripts/build_trail_corpus.py
"""

import json
import os
import re
import sys
import time
import urllib.request
import urllib.parse
from pathlib import Path

SCRIPT_DIR  = Path(__file__).parent
CORPUS_DIR  = SCRIPT_DIR.parent / "data" / "corpus" / "stage_10"
CORPUS_DIR.mkdir(parents=True, exist_ok=True)
OUT_PATH    = CORPUS_DIR / "trails_and_travel.txt"

UA = "AlbertMoE13CorpusBot/1.0 (training corpus; contact contact@ternlang.com)"


# ── utilities ────────────────────────────────────────────────────────────────

def log(msg: str):
    ts = time.strftime("%H:%M:%S")
    print(f"[{ts}] {msg}", flush=True)

def fetch(url: str, delay: float = 0.4, retries: int = 3) -> bytes | None:
    for attempt in range(retries):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": UA})
            with urllib.request.urlopen(req, timeout=25) as r:
                data = r.read()
            time.sleep(delay)
            return data
        except Exception as e:
            if attempt < retries - 1:
                time.sleep(2 ** attempt)
            else:
                log(f"  fetch failed: {e} — {url[:80]}")
    return None

def fetch_json(url: str, **kw) -> dict | list | None:
    raw = fetch(url, **kw)
    if not raw:
        return None
    try:
        return json.loads(raw)
    except Exception:
        return None

_TAG_RE = re.compile(r"<[^>]+>")
_ENT    = {"&amp;": "&", "&lt;": "<", "&gt;": ">", "&quot;": '"',
           "&apos;": "'", "&#39;": "'", "&nbsp;": " ", "&#160;": " "}

def strip_html(html: str) -> str:
    if not html:
        return ""
    for ent, ch in _ENT.items():
        html = html.replace(ent, ch)
    html = _TAG_RE.sub(" ", html)
    return re.sub(r"[ \t]{3,}", " ", html).strip()

def write_block(fh, block: str):
    fh.write(block.strip() + "\n\n" + "─" * 72 + "\n\n")
    fh.flush()

def clean_wikitext(text: str) -> str:
    """Strip MediaWiki markup from plaintext extracts."""
    text = re.sub(r"\{\{[^}]*\}\}", "", text)       # templates
    text = re.sub(r"\[\[File:[^\]]*\]\]", "", text)  # file links
    text = re.sub(r"\[\[([^\]|]+\|)?([^\]]+)\]\]", r"\2", text)  # wikilinks
    text = re.sub(r"={2,6}([^=]+)={2,6}", r"\n\1\n", text)       # headers
    text = re.sub(r"'{2,3}", "", text)               # bold/italic
    text = re.sub(r"\n{3,}", "\n\n", text)
    return text.strip()


# ── 1. Wikivoyage ────────────────────────────────────────────────────────────

WV_API = "https://en.wikivoyage.org/w/api.php"

# Categories and search terms that yield hiking, cycling, trail content
WV_SEARCHES = [
    "hiking trail route",
    "mountain biking trail",
    "trekking route guide",
    "cycling route",
    "walking trail directions",
    "long distance trail",
    "national park hiking",
    "alpine trail",
    "coastal walking route",
    "pilgrimage route",
    "multi-day hike",
    "backpacking route",
    "trail running route",
    "via ferrata",
    "camino route",
]

# High-value Wikivoyage article titles — destinations known for rich trail content
WV_DIRECT_TITLES = [
    "Hiking in the Alps",
    "Tour du Mont Blanc",
    "Appalachian Trail",
    "Pacific Crest Trail",
    "Camino de Santiago",
    "Hiking in New Zealand",
    "Dolomites",
    "Hiking in Patagonia",
    "Hiking in Norway",
    "Hiking in Scotland",
    "Hiking in Iceland",
    "Hiking in Japan",
    "Hiking in Peru",
    "GR 20",
    "Hiking in Morocco",
    "Hiking in Croatia",
    "Hiking in Switzerland",
    "Hiking in Austria",
    "Hiking in Slovenia",
    "Hiking in Nepal",
    "Annapurna Circuit",
    "Everest Base Camp trek",
    "Inca Trail",
    "Hiking in South Africa",
    "Hiking in Canada",
    "Hiking in Australia",
    "Hiking in the United States",
    "Mountain biking in Moab",
    "Mountain biking in Whistler",
    "Mountain biking in the Alps",
    "Cycling in the Netherlands",
    "Cycling in France",
    "EuroVelo routes",
    "Trans-Siberian Railway",     # travel narrative + directions
    "Silk Road",
    "Hiking in Georgia (country)",
    "West Highland Way",
    "Pennine Way",
    "Coast to Coast Walk",
    "Hadrian's Wall Path",
    "Thames Path",
    "Jordan Trail",
    "Via Alpina",
    "Haute Route",
    "Hiking in Madeira",
    "Hiking in the Azores",
    "Hiking in Tenerife",
    "Hiking in Corsica",
    "Hiking in Sardinia",
]

def wv_search_titles(query: str, limit: int = 30) -> list[str]:
    q = urllib.parse.quote(query)
    url = (f"{WV_API}?action=query&list=search&srsearch={q}"
           f"&srnamespace=0&srlimit={limit}&format=json")
    data = fetch_json(url, delay=0.5)
    if not data:
        return []
    return [hit["title"] for hit in data.get("query", {}).get("search", [])]

def wv_fetch_article(title: str) -> str | None:
    t = urllib.parse.quote(title)
    url = (f"{WV_API}?action=query&prop=extracts&explaintext=1"
           f"&titles={t}&format=json")
    data = fetch_json(url, delay=0.5)
    if not data:
        return None
    pages = data.get("query", {}).get("pages", {})
    for page in pages.values():
        text = page.get("extract", "")
        if text and len(text) > 300:
            return clean_wikitext(text)
    return None

def format_wv_article(title: str, text: str) -> str | None:
    if len(text) < 300:
        return None

    # Extract sections most likely to have directions
    # (Do, Get around, Go next, By foot, By bike, Hike, Trek)
    direction_keywords = [
        "do", "get around", "go next", "by foot", "by bike", "hike",
        "trek", "trail", "route", "path", "walk", "climb", "descend",
        "junction", "turn", "follow", "km", "mile", "head", "bear left",
        "bear right", "continue", "cross", "bridge", "summit", "pass",
        "trailhead", "waymark", "blaze", "marker", "signpost",
    ]
    text_lower = text.lower()
    score = sum(text_lower.count(kw) for kw in direction_keywords)
    if score < 5:
        return None  # skip articles with no directional content

    return (
        f"══ TRAIL & TRAVEL GUIDE ══\n"
        f"Source : Wikivoyage\n"
        f"Title  : {title}\n"
        f"\n"
        f"{text[:4000]}"
    )

def build_wikivoyage(fh) -> int:
    log("Wikivoyage — fetching hiking & travel guides ...")
    seen   = set()
    total  = 0

    # Direct high-value articles first
    for title in WV_DIRECT_TITLES:
        if title in seen:
            continue
        seen.add(title)
        text  = wv_fetch_article(title)
        block = format_wv_article(title, text) if text else None
        if block:
            write_block(fh, block)
            total += 1
            log(f"  ✓ {title} ({len(text):,} chars)")

    # Search-discovered articles
    for query in WV_SEARCHES:
        titles = wv_search_titles(query, limit=25)
        for title in titles:
            if title in seen:
                continue
            seen.add(title)
            text  = wv_fetch_article(title)
            block = format_wv_article(title, text) if text else None
            if block:
                write_block(fh, block)
                total += 1
        log(f"  query '{query}' → {len(titles)} candidates")

    log(f"Wikivoyage done — {total} articles")
    return total


# ── 2. Wikipedia long-distance trail articles ────────────────────────────────

WP_API = "https://en.wikipedia.org/w/api.php"

# Wikipedia articles with known km-by-km route descriptions
WP_TRAIL_TITLES = [
    "Appalachian Trail",
    "Pacific Crest Trail",
    "Continental Divide Trail",
    "Tour du Mont Blanc",
    "GR 20",
    "West Highland Way",
    "Pennine Way",
    "Camino de Santiago",
    "Annapurna Circuit",
    "Everest Base Camp",
    "Inca Trail",
    "Te Araroa",
    "Australian Alps Walking Track",
    "Bibbulmun Track",
    "Larapinta Trail",
    "Kungsleden",
    "St. Olav Ways",
    "Via Francigena",
    "Lycian Way",
    "Jordan Trail",
    "Israel National Trail",
    "Via Alpina",
    "Haute Route",
    "Bernese Alps traverse",
    "John Muir Trail",
    "Long Trail (Vermont)",
    "Colorado Trail",
    "Arizona Trail",
    "Florida Trail",
    "Ice Age Trail",
    "North Country Trail",
    "Ouray Via Ferrata",
    "Tour de Mont Blanc",
    "Walker's Haute Route",
    "Alta Via 1",
    "Alta Via 2",
    "Dolomiti Alta Via",
    "E1 European long distance path",
    "E4 European long distance path",
    "Trans Canada Trail",
    "Bruce Trail",
    "Great Divide Mountain Bike Route",
    "Transamerica Trail",
    "Pacific Coast Route",
    "EuroVelo 6",
    "EuroVelo 15",
    "Mawson Trail",
    "Snowdonia trails",
    "Lake District fells",
    "Munro (Scottish mountains)",
    "Via Rhôna",
    "Rhine Cycle Route",
    "Danube Cycle Way",
]

def wp_fetch_article(title: str) -> str | None:
    t = urllib.parse.quote(title)
    url = (f"{WP_API}?action=query&prop=extracts&explaintext=1"
           f"&exsectionformat=plain&titles={t}&format=json")
    data = fetch_json(url, delay=0.4)
    if not data:
        return None
    pages = data.get("query", {}).get("pages", {})
    for page in pages.values():
        text = page.get("extract", "")
        if text and len(text) > 500:
            return clean_wikitext(text)
    return None

def format_wp_trail(title: str, text: str) -> str | None:
    if len(text) < 500:
        return None

    direction_kw = [
        "km", "mile", "trail", "route", "path", "section", "segment",
        "trailhead", "junction", "summit", "pass", "ascent", "descent",
        "north", "south", "east", "west", "follows", "continues",
        "turn", "head", "cross", "reach", "traverse", "via", "along",
    ]
    score = sum(text.lower().count(kw) for kw in direction_kw)
    if score < 10:
        return None

    return (
        f"══ TRAIL DESCRIPTION ══\n"
        f"Source : Wikipedia\n"
        f"Trail  : {title}\n"
        f"\n"
        f"{text[:5000]}"
    )

def build_wikipedia_trails(fh) -> int:
    log("Wikipedia — fetching long-distance trail articles ...")
    total = 0
    seen  = set()
    for title in WP_TRAIL_TITLES:
        if title in seen:
            continue
        seen.add(title)
        text  = wp_fetch_article(title)
        block = format_wp_trail(title, text) if text else None
        if block:
            write_block(fh, block)
            total += 1
            log(f"  ✓ {title} ({len(text):,} chars)")
        else:
            log(f"  – {title} (skipped — low directional content)")
    log(f"Wikipedia trails done — {total} articles")
    return total


# ── 3. NPS.gov trail descriptions ────────────────────────────────────────────

# NPS has a public API at developer.nps.gov but requires a key.
# Instead we use their sitemap-accessible trail description pages.
# These are structured plain-text trail guides — public domain (US federal).

NPS_PARKS = [
    ("yosemite",         "Yosemite National Park"),
    ("grca",             "Grand Canyon National Park"),
    ("zion",             "Zion National Park"),
    ("romo",             "Rocky Mountain National Park"),
    ("acad",             "Acadia National Park"),
    ("olym",             "Olympic National Park"),
    ("shen",             "Shenandoah National Park"),
    ("grsm",             "Great Smoky Mountains"),
    ("glac",             "Glacier National Park"),
    ("yell",             "Yellowstone National Park"),
    ("ever",             "Everglades National Park"),
    ("jotr",             "Joshua Tree National Park"),
    ("dena",             "Denali National Park"),
    ("arch",             "Arches National Park"),
    ("care",             "Canyonlands National Park"),
    ("brca",             "Bryce Canyon National Park"),
    ("care",             "Capitol Reef National Park"),
    ("seki",             "Sequoia & Kings Canyon"),
    ("mora",             "Mount Rainier National Park"),
    ("crla",             "Crater Lake National Park"),
]

def nps_fetch_trails(park_code: str, park_name: str) -> list[str]:
    """Fetch trail list from NPS trails endpoint (no key required for basic data)."""
    url = f"https://www.nps.gov/{park_code}/planyourvisit/hiking.htm"
    raw = fetch(url, delay=0.6)
    if not raw:
        return []

    html = raw.decode("utf-8", errors="ignore")

    # Extract trail name + description blocks from the NPS hiking page
    blocks = []

    # NPS pages use consistent h3/h4 for trail names and p for descriptions
    trail_pattern = re.compile(
        r"<h[34][^>]*>([^<]{5,80}(?:Trail|Path|Route|Loop|Walk|Hike)[^<]*)</h[34]>"
        r"(.*?)</(?:section|article|div)",
        re.IGNORECASE | re.DOTALL
    )
    for m in trail_pattern.finditer(html):
        trail_name = strip_html(m.group(1)).strip()
        body       = strip_html(m.group(2)).strip()
        if len(body) > 100:
            blocks.append(f"Trail: {trail_name}\n{body[:800]}")

    return blocks

def build_nps(fh) -> int:
    log("NPS.gov — fetching US National Park trail descriptions ...")
    total = 0
    for code, name in NPS_PARKS:
        trails = nps_fetch_trails(code, name)
        if trails:
            block = (
                f"══ NATIONAL PARK TRAIL GUIDE ══\n"
                f"Source : NPS.gov (public domain)\n"
                f"Park   : {name}\n"
                f"\n"
                + "\n\n".join(trails)
            )
            write_block(fh, block)
            total += len(trails)
            log(f"  ✓ {name}: {len(trails)} trails")
        else:
            log(f"  – {name}: no structured trails found")
    log(f"NPS done — {total} trail descriptions")
    return total


# ── 4. Handcrafted trail narrative examples ───────────────────────────────────
#
# Dense examples of the spatial/procedural register we want Albert to absorb:
# landmark-based navigation, distance estimation, conditional routing,
# elevation cues, surface descriptions, waypoint sequencing.

TRAIL_EXAMPLES = [
    ("Tour du Mont Blanc — Day 4: Les Contamines to Les Chapieux", """
Depart Les Contamines (1167m) via the main street heading south, following TMB
red-and-white blazes through the old village. Cross the Pont de la Téna bridge
and bear right onto the trail that runs parallel to the Bon Nant river. The path
climbs gently through mixed forest for the first 3km — good footing, wide track,
suitable for trekking poles stowed. At the Notre-Dame de la Gorge chapel (1210m)
the trail steepens considerably. Follow the old Roman road — paved with large
flat stones — as it switchbacks up through increasingly open terrain.

Above the treeline (approx. 1500m) the valley opens dramatically. Continue on
the left bank of the river, ignoring the right-bank variant unless the main trail
is flooded after rain. The path levels briefly at the Plan Jovet alpage (1700m)
where cows graze from July to September — stay on the marked trail through the
pasture and close any gates behind you.

The final push to the Col du Bonhomme (2329m) gains 600m in roughly 4km. The
gradient is consistent rather than brutal — approximately 12% average. At the
col the wind can be ferocious; add a layer before stopping. From the col the
TMB splits: right fork descends to Les Contamines (return route, ignore), left
fork continues to the Col de la Croix du Bonhomme (2483m) — a further 20 min
on a clear rocky path with good visibility. On cloudy days stay on the main path
as the terrain becomes confusing in mist.

Descend south on a zigzagging trail across loose scree — slow, careful footwork
required. The path improves below 2100m as it enters the upper Vallée des Glaciers.
Follow the valley floor 4km to Les Chapieux (1549m). Total distance: 19km.
Elevation gain: 1400m. Elevation loss: 1000m. Allow 6-7 hours.
"""),

    ("Moab — Slickrock Bike Trail full loop description", """
The Slickrock Bike Trail begins at the Sand Flats Recreation Area trailhead
(elev. 1768m), approximately 4km east of Moab on Sand Flats Road. Register and
pay the day-use fee at the self-service kiosk before setting out. Helmets mandatory.

The 2.2km practice loop departs immediately right from the trailhead — do this
first if you have not ridden slickrock before. The surface is Navajo sandstone,
orange-pink and grippy when dry, dangerously slick when wet. Do not ride after
rain or when afternoon thunderstorms are forecast.

Main loop (19km): Follow the white-painted dotted line. This is not optional —
navigation without the dots in the undulating sandstone terrain is genuinely
disorienting. The line marks the safest line, not necessarily the most obvious one.

First technical section (km 1.5-3): Three consecutive steep sandstone humps with
short flat sections between. The descents are steeper than they appear — approach
slowly, weight back, resist the front brake. Riders who dab here typically
underestimated the pitch angle.

Echo Overlook (km 7): Short spur left, 200m return. Views down into Echo Canyon
and across to the La Sal Mountains. Worth the stop. Rejoin main loop, continue
clockwise.

The "Abyss" section (km 9-11): The longest sustained technical sequence. Multiple
lines exist; the marked line is conservative. Advanced riders may pick steeper
alternatives but these are not signed and self-rescue applies. Exposed sandstone
drops of 2-4m on both sides in places — focus is required.

Final 5km returns to trailhead on generally easier terrain with one final steep
climb (km 17) that sends many riders off the bike for a brief push. Resist
rushing this section — fatigue-related crashes spike in the last hour.
Total moving time: 2.5-4 hours depending on fitness and conditions.
"""),

    ("Cinque Terre coastal path — Monterosso to Vernazza", """
The classic Cinque Terre coastal walk from Monterosso al Mare to Vernazza
(approximately 3.8km, allow 90 minutes one-way) begins at the northern end of
Monterosso's old town. Look for the Cinque Terre National Park sign and the
green-and-white blazes of Trail 592.

The trail climbs immediately and steeply from sea level. Stone steps — many of
them — switchback up the hillside through terraced vineyards. The terraces date
from medieval times; the dry-stone walls are maintained by the last remaining
terrace farmers. In June the rosemary and thyme overhang the path and the air
is thick with scent.

At approximately 200m elevation the path levels onto a ridge traverse. The views
open left (south-west) to the deep blue Ligurian Sea and right to the terraced
hillsides. Do not leave the marked path — the terrace drops are severe and
unstaffed medical response in this terrain is slow.

The path descends sharply into a small valley (the fosso, a seasonal watercourse)
and immediately climbs again — this undulating ridge-and-valley pattern repeats
three times before the final descent to Vernazza. The total elevation gain is
approximately 550m; total loss approximately 590m.

The final descent to Vernazza is the steepest section. Loose shale on top of
bedrock in the upper section; take short steps, do not lean back. The path
passes through a tunnel (natural rock arch, 4m long, no artificial lighting)
before emerging above the harbor. From the arch to Vernazza harbor: 10 minutes.

Trail 592 continues to Corniglia; the junction is signed at the northern edge
of Vernazza. Allow extra time in summer — this section is heavily trafficked
and overtaking on the narrow path requires coordination.
"""),

    ("Singalila Ridge — Sandakphu to Phalut (Himalaya)", """
The Sandakphu to Phalut section of the Singalila Ridge trek is among the finest
high-altitude walks in the eastern Himalaya. The route follows the ridge that
marks the border between West Bengal (India) and Nepal, offering unobstructed
views of four of the world's five highest peaks on clear mornings.

Start from Sandakphu (3636m) before 6am on clear days — clouds build reliably
from mid-morning and obscure the Kangchenjunga massif by 9am. The trail heads
south-east along the main ridge track, a broad dirt jeep road for the first 4km
that narrows to a footpath at the Sabarkum shelter (3590m).

From Sabarkum the path undulates along the ridge at 3400-3600m. Navigation is
trivial in clear conditions — keep Nepal on your left, India on your right. In
cloud, follow the ridge crest and do not descend into either valley. The rhodo-
dendron forests on both flanks bloom crimson and pink in April; the trail becomes
a tunnel through the blooms at 3200-3400m elevation.

Kala Pokhri (Black Lake, 3650m) at approximately km 12 — a small sacred lake
where swimming is prohibited. Fill water here; the next reliable source is
Phalut. The climb from Kala Pokhri to Phalut summit (3600m) is gradual but
the altitude is noticeable. Allow 20 minutes longer than the distance suggests.

Phalut (3636m, tied with Sandakphu as the highest point in West Bengal) offers
a trekkers' hut and a basic dhaba. On exceptional days the Kanchenjunga panorama
extends 180 degrees — Makalu and Lhotse visible to the west, Everest just
clearing the intervening ridges. Most trekkers return to Sandakphu the same day
(total: 22km, allow 7-8 hours) or continue south to Rammam on the longer
circuit.
"""),
]

def build_trail_examples(fh) -> int:
    log("Writing handcrafted trail narrative examples ...")
    for title, content in TRAIL_EXAMPLES:
        block = (
            f"══ TRAIL NARRATIVE ══\n"
            f"Source  : curated example\n"
            f"Route   : {title}\n"
            f"\n"
            f"{content.strip()}"
        )
        write_block(fh, block)
    log(f"Trail examples done — {len(TRAIL_EXAMPLES)} routes")
    return len(TRAIL_EXAMPLES)


# ── main ─────────────────────────────────────────────────────────────────────

def main():
    log("=" * 60)
    log("Trail & Travel corpus builder — starting")
    log(f"Output: {OUT_PATH}")
    log("=" * 60)

    total = 0
    with open(OUT_PATH, "w", encoding="utf-8") as fh:
        # Handcrafted examples first — dense, high-quality signal
        total += build_trail_examples(fh)
        log("")

        # Wikipedia long-distance trail articles
        total += build_wikipedia_trails(fh)
        log("")

        # Wikivoyage travel & hiking guides
        total += build_wikivoyage(fh)
        log("")

        # NPS trail descriptions
        total += build_nps(fh)

    size = OUT_PATH.stat().st_size
    log("")
    log("=" * 60)
    log(f"Done — {total} items → {OUT_PATH.name}")
    log(f"File size: {size / 1024 / 1024:.2f} MB")
    log("=" * 60)

if __name__ == "__main__":
    main()
