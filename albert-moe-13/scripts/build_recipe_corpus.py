#!/usr/bin/env python3
"""
Stage 10 — Gourmet & Fusion Recipe corpus builder
Albert MoE-13 training data pipeline

Sources (all free, no API keys required):
  1. TheMealDB API       — free recipe database, global cuisines
  2. Open Recipe GitHub  — CC-licensed structured recipes
  3. Wikipedia cuisine   — technique articles with precise measurements
  4. Curated gourmet     — handcrafted fine-dining / fusion examples

Teaches: precise quantitative language, procedural sequencing,
         technique vocabulary, unit conversions, conditional steps,
         temperature precision, timing dependencies.

Run (from albert-moe-13/):
    python3 scripts/build_recipe_corpus.py
"""

import json
import os
import re
import sys
import time
import urllib.request
import urllib.parse
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent
CORPUS_DIR = SCRIPT_DIR.parent / "data" / "corpus" / "stage_10"
CORPUS_DIR.mkdir(parents=True, exist_ok=True)
OUT_PATH   = CORPUS_DIR / "gourmet_recipes.txt"

UA = "AlbertMoE13CorpusBot/1.0 (training corpus; contact rfi.irfos@gmail.com)"


# ── utilities ────────────────────────────────────────────────────────────────

def log(msg: str):
    print(f"[{time.strftime('%H:%M:%S')}] {msg}", flush=True)

def fetch(url: str, delay: float = 0.4, retries: int = 3) -> bytes | None:
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
                log(f"  fetch failed: {e}")
    return None

def fetch_json(url: str, **kw):
    raw = fetch(url, **kw)
    if not raw:
        return None
    try:
        return json.loads(raw)
    except Exception:
        return None

_TAG_RE = re.compile(r"<[^>]+>")
_ENT    = {"&amp;": "&", "&lt;": "<", "&gt;": ">", "&quot;": '"',
           "&apos;": "'", "&#39;": "'", "&nbsp;": " "}

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


# ── 1. TheMealDB — free recipe API ───────────────────────────────────────────

MEALDB = "https://www.themealdb.com/api/json/v1/1"

# All cuisine categories available in TheMealDB
MEALDB_CATEGORIES = [
    "Beef", "Chicken", "Dessert", "Lamb", "Pasta", "Pork",
    "Seafood", "Side", "Starter", "Vegan", "Vegetarian",
    "Breakfast", "Goat", "Miscellaneous",
]

# Areas (cuisines) for cross-cultural coverage
MEALDB_AREAS = [
    "French", "Italian", "Japanese", "Chinese", "Indian", "Mexican",
    "Thai", "Greek", "Spanish", "Turkish", "Moroccan", "British",
    "American", "Canadian", "Croatian", "Dutch", "Egyptian",
    "Filipino", "Irish", "Jamaican", "Kenyan", "Malaysian",
    "Polish", "Portuguese", "Russian", "Tunisian", "Vietnamese",
]

def mealdb_fetch_recipe(meal_id: str) -> dict | None:
    data = fetch_json(f"{MEALDB}/lookup.php?i={meal_id}", delay=0.3)
    if not data:
        return None
    meals = data.get("meals")
    return meals[0] if meals else None

def mealdb_list_by_area(area: str) -> list[str]:
    data = fetch_json(f"{MEALDB}/filter.php?a={urllib.parse.quote(area)}", delay=0.3)
    if not data or not data.get("meals"):
        return []
    return [m["idMeal"] for m in data["meals"]]

def format_mealdb_recipe(meal: dict) -> str | None:
    name      = meal.get("strMeal", "").strip()
    category  = meal.get("strCategory", "")
    area      = meal.get("strArea", "")
    tags      = meal.get("strTags") or ""
    youtube   = meal.get("strYoutube", "")
    source    = meal.get("strSource", "")
    instr     = strip_html(meal.get("strInstructions", "")).strip()

    if len(instr) < 100:
        return None

    # Build ingredient list
    ingredients = []
    for i in range(1, 21):
        ing   = (meal.get(f"strIngredient{i}") or "").strip()
        meas  = (meal.get(f"strMeasure{i}") or "").strip()
        if ing:
            line = f"  {meas:15s} {ing}" if meas else f"  {ing}"
            ingredients.append(line)

    lines = [
        f"══ RECIPE ══",
        f"Name     : {name}",
        f"Cuisine  : {area}",
        f"Category : {category}",
    ]
    if tags:
        lines.append(f"Tags     : {tags}")

    lines += [
        "",
        "[INGREDIENTS]",
        *ingredients,
        "",
        "[METHOD]",
        instr[:3000],
    ]

    return "\n".join(lines)

def build_mealdb(fh) -> int:
    log("TheMealDB — fetching cross-cuisine recipes ...")
    total = 0
    seen  = set()

    for area in MEALDB_AREAS:
        ids = mealdb_list_by_area(area)
        log(f"  {area}: {len(ids)} recipes")
        for meal_id in ids[:15]:  # up to 15 per cuisine
            if meal_id in seen:
                continue
            seen.add(meal_id)
            meal  = mealdb_fetch_recipe(meal_id)
            block = format_mealdb_recipe(meal) if meal else None
            if block:
                write_block(fh, block)
                total += 1

    log(f"TheMealDB done — {total} recipes")
    return total


# ── 2. Wikipedia culinary technique articles ─────────────────────────────────

WP_API = "https://en.wikipedia.org/w/api.php"

# Wikipedia articles that are dense with precise technique, temperature,
# timing, and measurement language — the register we want
WP_CULINARY_TITLES = [
    # Core techniques
    "Sous-vide",
    "Tempering (chocolate)",
    "Maillard reaction",
    "Caramelization",
    "Emulsion",
    "Fermentation in food processing",
    "Pastry",
    "Choux pastry",
    "Puff pastry",
    "Shortcrust pastry",
    "Sourdough",
    "Bread",
    "Croissant",
    "Brioche",
    "Béchamel sauce",
    "Hollandaise sauce",
    "Velouté sauce",
    "Espagnole sauce",
    "Demi-glace",
    "Consommé",
    "Roux",
    "Ganache",
    "Crème brûlée",
    "Crème caramel",
    "Macaron",
    "Madeleine (cake)",
    "Profiterole",
    "Soufflé",
    "Mousse",
    "Pâté",
    "Terrine (food)",
    "Confit",
    "Braising",
    "Blanching (cooking)",
    "Deep frying",
    "Tempura",
    "Risotto",
    "Pasta",
    "Fresh pasta",
    "Gnocchi",
    "Ravioli",
    "Ramen",
    "Dashi",
    "Miso soup",
    "Sushi",
    "Sashimi",
    "Nigiri",
    "Mole sauce",
    "Curry",
    "Vindaloo",
    "Biryani",
    "Tagine",
    "Couscous",
    "Hummus",
    "Falafel",
    "Baklava",
    "Tiramisu",
    "Panna cotta",
    "Gelato",
    "Sorbet",
    "Ice cream",
    "Chocolate",
    "Praline",
    "Nougat",
    "Marzipan",
    "Fondant",
    "Caramel",
    "Toffee",
    "Fudge",
    "Meringue",
    "Pavlova (cake)",
    "Beurre blanc",
    "Reduction (cooking)",
    "Stock (food)",
    "Fond (cooking)",
    "Searing",
    "Brining",
    "Curing (food preservation)",
    "Smoking (cooking)",
    "Fermentation in winemaking",
    "Beer brewing",
    "Sourdough starter",
    "Kimchi",
    "Kombucha",
    "Cheesemaking",
    "Butter",
    "Clarified butter",
    "Ghee",
    "Crème fraîche",
    "Ricotta",
    "Mascarpone",
    "Fromage blanc",
]

def wp_fetch(title: str) -> str | None:
    t = urllib.parse.quote(title)
    url = (f"{WP_API}?action=query&prop=extracts&explaintext=1"
           f"&exintro=0&titles={t}&format=json")
    data = fetch_json(url, delay=0.35)
    if not data:
        return None
    pages = data.get("query", {}).get("pages", {})
    for p in pages.values():
        text = p.get("extract", "")
        if text and len(text) > 300:
            # Strip wikitext artifacts
            text = re.sub(r"\{\{[^}]*\}\}", "", text)
            text = re.sub(r"\[\[([^\]|]+\|)?([^\]]+)\]\]", r"\2", text)
            text = re.sub(r"={2,6}([^=]+)={2,6}", r"\n\1\n", text)
            text = re.sub(r"'{2,3}", "", text)
            text = re.sub(r"\n{3,}", "\n\n", text)
            return text.strip()
    return None

# Keywords indicating precise culinary language
CULINARY_PRECISION_KW = [
    "°c", "°f", "degrees", "temperature", "gram", "ml", "litre",
    "tablespoon", "teaspoon", "cup", "minute", "hour", "second",
    "until", "reduce", "simmer", "whisk", "fold", "rest", "proof",
    "temper", "bloom", "deglaze", "emulsify", "season", "taste",
    "ratio", "proportion", "%", "brix", "ph",
]

def format_wp_culinary(title: str, text: str) -> str | None:
    if len(text) < 300:
        return None
    tl = text.lower()
    score = sum(tl.count(kw) for kw in CULINARY_PRECISION_KW)
    if score < 3:
        return None
    return (
        f"══ CULINARY TECHNIQUE ══\n"
        f"Source    : Wikipedia\n"
        f"Subject   : {title}\n"
        f"\n"
        f"{text[:4500]}"
    )

def build_wikipedia_culinary(fh) -> int:
    log("Wikipedia — fetching culinary technique articles ...")
    total = 0
    for title in WP_CULINARY_TITLES:
        text  = wp_fetch(title)
        block = format_wp_culinary(title, text) if text else None
        if block:
            write_block(fh, block)
            total += 1
            log(f"  ✓ {title}")
        else:
            log(f"  – {title} (skipped)")
    log(f"Wikipedia culinary done — {total} articles")
    return total


# ── 3. Open Recipe GitHub datasets ───────────────────────────────────────────
#
# Several CC-licensed recipe collections live on GitHub as JSON/YAML.
# We fetch a curated subset directly from raw.githubusercontent.com.

OPEN_RECIPE_URLS = [
    # Cocktails & drinks — precise measurements, technique
    "https://raw.githubusercontent.com/teijo/iba-cocktails/master/recipes/negroni.json",
    "https://raw.githubusercontent.com/teijo/iba-cocktails/master/recipes/old-fashioned.json",
    "https://raw.githubusercontent.com/teijo/iba-cocktails/master/recipes/martini.json",
    "https://raw.githubusercontent.com/teijo/iba-cocktails/master/recipes/manhattan.json",
    "https://raw.githubusercontent.com/teijo/iba-cocktails/master/recipes/daiquiri.json",
    "https://raw.githubusercontent.com/teijo/iba-cocktails/master/recipes/margarita.json",
    "https://raw.githubusercontent.com/teijo/iba-cocktails/master/recipes/mojito.json",
    "https://raw.githubusercontent.com/teijo/iba-cocktails/master/recipes/cosmopolitan.json",
    "https://raw.githubusercontent.com/teijo/iba-cocktails/master/recipes/sidecar.json",
    "https://raw.githubusercontent.com/teijo/iba-cocktails/master/recipes/between-the-sheets.json",
]

def format_open_recipe(data: dict) -> str | None:
    if isinstance(data, dict):
        name  = data.get("name", data.get("title", ""))
        ingrs = data.get("ingredients", [])
        steps = data.get("instructions", data.get("method", data.get("steps", [])))
        notes = data.get("notes", data.get("garnish", ""))

        if not name or (not ingrs and not steps):
            return None

        lines = [f"══ RECIPE (OPEN SOURCE) ══", f"Name  : {name}", "", "[INGREDIENTS]"]
        if isinstance(ingrs, list):
            for ing in ingrs:
                if isinstance(ing, dict):
                    amt  = ing.get("amount", ing.get("quantity", ""))
                    unit = ing.get("unit", "")
                    ingr = ing.get("ingredient", ing.get("name", ""))
                    lines.append(f"  {str(amt):6} {unit:8} {ingr}")
                else:
                    lines.append(f"  {ing}")
        lines += ["", "[METHOD]"]
        if isinstance(steps, list):
            for i, step in enumerate(steps, 1):
                if isinstance(step, dict):
                    lines.append(f"Step {i}: {step.get('step', step.get('text', ''))}")
                else:
                    lines.append(f"Step {i}: {step}")
        elif isinstance(steps, str):
            lines.append(steps[:1500])
        if notes:
            lines += ["", f"Notes: {notes}"]
        return "\n".join(lines)
    return None

def build_open_recipes(fh) -> int:
    log("Open Recipe GitHub datasets — fetching ...")
    total = 0
    for url in OPEN_RECIPE_URLS:
        data  = fetch_json(url, delay=0.3)
        block = format_open_recipe(data) if data else None
        if block:
            write_block(fh, block)
            total += 1
    log(f"Open recipes done — {total} recipes")
    return total


# ── 4. Handcrafted gourmet & fusion examples ─────────────────────────────────
#
# Dense examples of fine-dining register: precise temperatures, timing,
# gram-level measurements, technique vocabulary, plating instructions.

GOURMET_EXAMPLES = [
    ("Duck Confit with Lentils du Puy and Sauce Périgueux", """
Cuisine: French bistro / classic
Serves: 4
Difficulty: Intermediate–Advanced
Prep: 48 hours (curing) + 3 hours active

INGREDIENTS:
  4              duck legs, skin-on (approx. 280g each)
  28g            fleur de sel (or fine sea salt)
  8g             freshly cracked black pepper
  4g             dried thyme
  2g             ground juniper berry (6–8 berries, crushed)
  1g             ground bay leaf
  4 cloves       garlic, minced
  1.2kg          duck fat (rendered), for confit
  200g           Lentilles du Puy (French green lentils — do not substitute)
  1              small carrot, brunoise (3mm dice)
  1              shallot, finely minced
  1              celery stalk, brunoise
  100ml          dry white wine
  400ml          duck or chicken stock
  15g            unsalted butter, cold, cubed
  30g            black truffle (fresh or preserved), finely sliced (optional)
  50ml           Madeira wine
  200ml          veal jus (or rich beef stock reduced by half)
  5g             arrowroot

METHOD — CURE (48 hours before):
Combine salt, pepper, thyme, juniper, bay, and garlic. Rub each duck leg
thoroughly on all surfaces. Place in a non-reactive container, skin-side up.
Cover with cling film pressed directly to the surface and refrigerate for
24–48 hours. Longer cure produces deeper flavour but risks over-salinity.

METHOD — CONFIT:
Remove duck from cure. Rinse briefly under cold water; pat completely dry.
This step is non-negotiable — residual salt will seize the confit and prevent
the slow fat bath from functioning correctly.

Melt duck fat in a deep, heavy-based pan over low heat. Submerge duck legs
completely — fat must fully cover. Internal fat temperature must hold at
80–85°C. Below 75°C: collagen will not convert; above 90°C: the meat will
seize and dry. Use a probe thermometer; adjust burner accordingly. Confit for
2 hours 30 minutes. Test with a skewer — it should pierce the thickest part
of the thigh with zero resistance.

Remove legs carefully; rest on a rack over a tray for 15 minutes. At this point
legs may be cooled and stored submerged in their fat for up to 3 weeks refrigerated.

METHOD — LENTILS:
Rinse lentils; no soaking required. Sweat shallot and carrot in 10g butter over
medium heat until translucent, approximately 4 minutes. Add celery; cook 2 further
minutes. Add lentils; stir to coat in fat. Deglaze with white wine; allow to
reduce fully (approximately 2 minutes). Add stock; bring to a simmer. Cook
uncovered at a gentle simmer for 22–25 minutes until lentils are just tender —
they should offer slight resistance (al dente). Season only at the end; salt
added early causes the lentils to remain firm regardless of cooking time.
Finish with 5g cold butter stirred through off heat. Hold warm.

METHOD — SAUCE PÉRIGUEUX:
Reduce Madeira by two-thirds in a small saucepan. Add veal jus; bring to a simmer.
Dissolve arrowroot in 10ml cold water; whisk into sauce in a thin stream until
consistency coats the back of a spoon. Remove from heat; fold in truffle slices.
Do not boil after truffle is added — volatile aromatics dissipate above 85°C.

METHOD — FINISH:
Heat a heavy oven-proof skillet (cast iron preferred) over medium-high heat — no
added fat. Place duck legs skin-side down. Do not disturb for 3–4 minutes until
skin renders and achieves deep golden-brown. Transfer to 220°C oven for
8 minutes to finish heating through and fully crisp the skin.

PLATING:
Spoon lentils into centre of warm plate. Rest duck leg against lentils, skin
facing up. Sauce around (not over) the duck — moisture softens the crisped skin.
Garnish with a single flat-leaf parsley sprig or microherbs if available.
"""),

    ("Chocolate Fondant with Salted Caramel Core and Tonka Bean Ice Cream", """
Cuisine: Contemporary pastry / fine dining
Serves: 6
Difficulty: Advanced
Prep: 4 hours (ice cream) + 1 hour (fondant)
Critical timing: fondant must be served immediately upon removal from oven.

TONKA BEAN ICE CREAM — begin 4 hours before service:
  500ml    full-fat whole milk (3.5% fat minimum)
  250ml    double cream (35% fat)
  2        tonka beans, finely grated on a Microplane
  120g     egg yolk (approx. 6 large yolks)
  140g     caster sugar
  1 pinch  fleur de sel

Combine milk, cream, and tonka bean in a heavy saucepan. Heat over medium flame
to 80°C — do not boil. Remove from heat; cover and steep for 30 minutes.

Whisk yolks and sugar to ribbon stage (pale yellow, tripled in volume, leaves
a trail when ribbon falls back on itself — approximately 3–4 minutes by hand
or 90 seconds with a stand mixer on high).

Reheat cream mixture to 70°C. Pour one-third over yolk mixture while whisking
constantly (tempering — prevents scrambling). Pour tempered mixture back into pan.

Cook over low heat, stirring continuously with a heat-resistant spatula in a
figure-eight pattern, until custard reaches 82°C and coats the back of the
spatula (nappe consistency — a finger drawn through the coating leaves a clean
line). Remove immediately from heat; pour through a fine-mesh sieve. Add salt.
Cool over an ice bath to below 10°C, stirring occasionally. Churn in an ice
cream machine until thick (approximately 25–30 minutes depending on machine).
Transfer to a frozen container; allow to harden minimum 2 hours.

SALTED CARAMEL CORE:
  120g     caster sugar
  40ml     water
  90ml     double cream, warm
  35g      unsalted butter, room temperature
  4g       fleur de sel

Combine sugar and water in a clean, light-coloured saucepan (colour visibility
is essential for caramel work). Cook without stirring over medium-high heat.
Brush down sugar crystals on the sides of the pan with a wet pastry brush.
When sugar reaches amber — 170°C on a sugar thermometer, mahogany in colour,
first wisps of smoke just appearing — remove from heat. Carefully pour in warm
cream (mixture will foam violently; stand back). Whisk until smooth. Add butter
in three additions, whisking after each. Add salt. Pour into a small container
and freeze until solid (minimum 2 hours). Unmould; cut into 6 cubes of
approximately 10g each. Return to freezer until needed.

FONDANT:
  200g     70% dark chocolate, best quality available (Valrhona Guanaja or similar)
  180g     unsalted butter, plus extra for moulds
  4        whole eggs
  4        egg yolks
  80g      caster sugar
  30g      plain flour (00 grade), sifted
  Cocoa powder, for dusting moulds

Preheat oven to 200°C (fan: 190°C). Butter six 150ml individual pudding moulds
or ramekins meticulously — any gap causes sticking. Dust with cocoa; tap out excess.
Refrigerate moulds while preparing batter.

Melt chocolate and butter together over a bain-marie (bowl should not touch
water; steam only). Stir until fully combined and glossy; cool to 45°C.

Whisk whole eggs, yolks, and sugar until pale and thick (ribbon stage, 3 minutes).
Fold chocolate mixture into egg mixture in three additions — preserve as much
volume as possible. Fold in flour until just combined; do not overmix or develop
gluten.

Fill each mould to two-thirds. Press one frozen caramel cube into the centre of
each; cover completely with batter. At this point, fondants may be refrigerated
up to 6 hours.

Bake at 200°C for exactly 11 minutes (12 if refrigerated). The edges should
be set; the centre should wobble when gently shaken. Over-bake by 60 seconds
and the molten centre sets — there is no recovery. Run a thin knife around the
edge of each mould; invert immediately onto warm plates.

PLATING:
Dust fondant with cocoa. Place one scoop tonka ice cream to the left; make a
small salt flake crown on the fondant surface. Serve within 90 seconds.
"""),

    ("Miso-Glazed Black Cod (Nobu-style) — Saikyo Yaki", """
Cuisine: Japanese-Western fusion
Serves: 4
Prep: 48–72 hours marinating + 25 minutes cooking

This preparation is a direct interpretation of Nobu Matsuhisa's signature dish.
The three-day marinade is not decorative — the miso's enzymes require sustained
contact to fully penetrate and break down the surface proteins.

MISO MARINADE:
  80ml     sake
  80ml     mirin
  240g     white miso paste (Shiro miso — do not use red/Aka miso, too strong)
  120g     caster sugar

Bring sake and mirin to a boil in a small saucepan over medium heat. Maintain
a gentle boil for 20 seconds to burn off the alcohol. Reduce heat to low; add
miso and whisk to combine. Add sugar in three additions, whisking after each.
Remove from heat when sugar is fully dissolved and marinade is glossy.
Cool completely to room temperature before use — hot marinade will begin cooking
the fish surface and close the pores against further absorption.

FISH:
  4 × 200g  black cod fillets (Sablefish / Anoplopoma fimbria), skin on
             Substitute: Chilean sea bass if unavailable (adjust time by -2 min)

Check fillets for pin bones; remove with tweezers. Pat completely dry. Place
fillets in a non-reactive dish; coat all surfaces generously with cooled marinade.
Cover; refrigerate for 48 hours minimum, 72 hours preferred. Turn once at 24 hours.

COOKING:
Remove fillets from marinade; wipe off excess with a paper towel — a thick
residual layer burns before the fish cooks through. Discard marinade; do not reuse.

Method A (traditional): Place fillets under a preheated grill/broiler on the
highest rack, skin-side up. Grill for 8–10 minutes until skin is blistered and
deeply caramelised. The sugars in the miso will approach burning — this is correct;
do not pull early. If skin begins to blacken (not brown), lower rack by one position.
Internal temperature: 55°C for medium, 60°C for fully cooked.

Method B (pan then oven): Sear skin-side down in a dry non-stick pan over medium
heat for 3 minutes until blistered. Transfer to 200°C oven for 8 minutes.

REST: 2 minutes on a warm plate before plating.

ACCOMPANIMENT — YUZU BEURRE BLANC (optional):
  60ml     yuzu juice (or 40ml lime + 20ml lemon if unavailable)
  30ml     dry white wine
  20ml     rice wine vinegar
  1        shallot, finely minced
  150g     cold unsalted butter, cut into 1cm cubes

Reduce yuzu, wine, vinegar, and shallot by three-quarters over medium heat.
Reduce heat to minimum. Whisk in cold butter one cube at a time, allowing each
to fully incorporate before adding the next. The sauce must never exceed 80°C
or it will break. Strain; season; hold warm in a bain-marie.

PLATING:
Pool sauce on warm plate. Place cod skin-side up; skin must remain above the
sauce line to preserve crispness. Garnish: micro shiso, pickled ginger, or
a light scattering of toasted sesame seeds. Serve immediately.
"""),

    ("Pork Belly Ramen — full broth from scratch (Tonkotsu-style)", """
Cuisine: Japanese
Serves: 4 bowls
Prep: 18 hours total (broth) + 3 hours (components)
Note: this is a multi-day project. Begin broth 2 days before service.

TONKOTSU BROTH — Day 1:
  2kg      pork trotters (split lengthwise by your butcher)
  500g     pork neck bones
  300g     chicken carcasses (optional, adds clarity)
  Cold water to cover

Blanch all bones: place in a large pot; cover with cold water; bring to a rolling
boil over high heat. Boil 5 minutes — foam, blood, and impurities will rise.
Drain; rinse each bone individually under cold running water. This step is not
optional — skipping produces a murky, metallic-tasting broth.

Return cleaned bones to pot. Cover with fresh cold water (approximately 4 litres).
Bring to a rapid boil. This is Tonkotsu's defining step: maintain a HARD,
rolling boil for the entire cooking time. The aggressive boil emulsifies the
collagen and fat into a white, opaque broth. A gentle simmer produces a
transparent stock — not what we want here. Cook uncovered for 12 hours, adding
boiling water as needed to keep bones submerged.

Strain through a fine-mesh sieve; press solids firmly to extract all liquid.
Broth should be white, opaque, and coat the back of a spoon. Refrigerate overnight.

Day 2 — remove solidified fat cap (reserve for chashu or discard). You should
have approximately 2 litres of rich broth.

CHASHU (soy-braised pork belly):
  800g     pork belly, skin-on, rolled and tied at 3cm intervals
  100ml    soy sauce (Japanese shoyu)
  80ml     mirin
  60ml     sake
  40g      caster sugar
  200ml    water
  3 slices ginger
  2        spring onions, halved

Sear pork belly in a neutral oil over high heat on all sides until deeply browned
(approx. 8 minutes total). Transfer to a tight-fitting pot or Dutch oven.
Add remaining ingredients; liquid should reach halfway up the belly.
Braise covered at 150°C for 2 hours. Remove belly; rest 15 minutes. Slice
into 1.5cm rounds. Reserve braising liquid as tare.

TARE (seasoning concentrate):
  Reserved chashu braising liquid
Reduce braising liquid by half over medium heat. This is your seasoning base —
approximately 30–40ml per bowl.

RAMEN EGGS (Ajitsuke Tamago) — prepare Day 1:
  6        large eggs, room temperature
  100ml    soy sauce
  60ml     mirin
  60ml     sake
  200ml    water
  10g      sugar

Boil eggs for exactly 6 minutes 30 seconds for a soft, jammy yolk. Transfer
immediately to iced water; peel after 5 minutes. Combine soy, mirin, sake,
water, and sugar; simmer 3 minutes; cool. Marinate peeled eggs in this mixture
for 4–6 hours in the refrigerator. Halve at service.

ASSEMBLY (per bowl):
  200g     fresh ramen noodles (or 100g dried)
  300ml    tonkotsu broth, heated to a rolling boil
  30ml     tare
  2 slices chashu
  1        marinated egg, halved
  2 tbsp   menma (bamboo shoots)
  2        nori sheets
  Spring onion, finely sliced
  Sesame seeds, toasted
  1 tsp    togarashi (optional)
  Mayu (black garlic oil): 5 cloves garlic, blackened in a dry pan,
          blended with 2 tbsp neutral oil — add ½ tsp per bowl

Warm bowl with boiling water; drain. Add tare to bowl first. Ladle in hot broth;
stir to combine. Add noodles (cooked 90 seconds from fresh, 3 minutes from dry).
Arrange toppings precisely — this is as much presentation as food.
"""),
]

def build_gourmet_examples(fh) -> int:
    log("Writing handcrafted gourmet & fusion examples ...")
    for title, content in GOURMET_EXAMPLES:
        block = f"══ GOURMET RECIPE ══\nTitle: {title}\n\n{content.strip()}"
        write_block(fh, block)
    log(f"Gourmet examples done — {len(GOURMET_EXAMPLES)} recipes")
    return len(GOURMET_EXAMPLES)


# ── main ─────────────────────────────────────────────────────────────────────

def main():
    log("=" * 60)
    log("Gourmet & Fusion Recipe corpus builder — starting")
    log(f"Output: {OUT_PATH}")
    log("=" * 60)

    total = 0
    with open(OUT_PATH, "w", encoding="utf-8") as fh:
        total += build_gourmet_examples(fh)
        log("")
        total += build_open_recipes(fh)
        log("")
        total += build_mealdb(fh)
        log("")
        total += build_wikipedia_culinary(fh)

    size = OUT_PATH.stat().st_size
    log("")
    log("=" * 60)
    log(f"Done — {total} items → {OUT_PATH.name}")
    log(f"File size: {size / 1024 / 1024:.2f} MB")
    log("=" * 60)

if __name__ == "__main__":
    main()
