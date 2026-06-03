#!/usr/bin/env python3
"""
TIS KPI Dashboard - Public Generator (v1.6)
===========================================
- 10 Metric Cards / 7 Charts / 5 Tables.
- Structured 2-3-2 Layout.
- Unified Spatial Flow: Horizontal legend pills + Dropdown time selectors.
- Persistent Ledger: Traffic charts pull from full reconstructed history.
- Fluent animations and CSV Export.
"""

import json, os, sys, time, io, urllib.request, urllib.error, webbrowser, threading, re
from datetime import datetime, timezone
from http.server import HTTPServer, BaseHTTPRequestHandler

# -- Config -------------------------------------------------------------------
# Secrets come from the environment (Fly secrets in production) — NEVER hardcode them
# here, this file is committed. Locally, export them or fall back to empty (degraded fetch).
GITHUB_PAT       = (os.environ.get("KPI_GITHUB_TOKEN")
                    or os.environ.get("GITHUB_TOKEN")
                    or os.environ.get("GITHUB_PAT", ""))
HF_TOKEN         = os.environ.get("HF_TOKEN", "")
GITHUB_REPOS     = [
    "eriirfos-eng/ternary-intelligence-stack",
    "rfi-irfos/invisible-layer",
    "rfi-irfos/albert-spores",
    "rfi-irfos/rusty-penguin",
]
GITHUB_REPO      = GITHUB_REPOS[0]  # primary: referrers/paths
CRATES_USER      = 403632
# Output dir: production sets KPI_OUTPUT_DIR=/data/kpi (served at ternlang.com/kpi);
# locally defaults to the repo's docs/kpi. All log + html artifacts land here.
_BASE_DIR        = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DESKTOP          = os.environ.get("KPI_OUTPUT_DIR") or os.path.join(_BASE_DIR, "docs/kpi")
os.makedirs(DESKTOP, exist_ok=True)
LOG_FILE         = os.path.join(DESKTOP, "ternlang_kpi_log.json")
TRAFFIC_LOG_FILE    = os.path.join(DESKTOP, "ternlang_gh_traffic.json")
REFERRERS_LOG_FILE  = os.path.join(DESKTOP, "ternlang_gh_referrers.json")
HTML_FILE        = os.path.join(DESKTOP, "index.html")  # served as ternlang.com/kpi/
PORT             = 7329
REFRESH_INTERVAL = 60
OPENVSX_EXTS     = ["ternlang"]
HF_AUTHOR        = "rfi-irfos"
FOOTPRINT_WEIGHTS = { "views": 1.0, "clones": 2.0, "crates": 4.0, "openvsx": 5.0, "smithery": 0.5, "hf": 3.0 }

latest_data = {}

# -- Helpers ------------------------------------------------------------------
def fetch(url, headers=None):
    req = urllib.request.Request(url, headers=headers or {})
    try:
        with urllib.request.urlopen(req, timeout=15) as r: return json.loads(r.read())
    except Exception as e:
        print(f"Error fetching {url}: {e}", file=sys.stderr)
        return None

def gh(path):
    if not GITHUB_PAT: return None
    return fetch(f"https://api.github.com/{path}", {"Authorization": f"token {GITHUB_PAT}", "User-Agent": "ternlang-dash", "Accept": "application/vnd.github.v3+json"})

def ping_api():
    for url in ["https://ternlang.com/kpi", "https://ternlang.com/health", "https://ternlang.com/"]:
        try:
            start = time.time()
            urllib.request.urlopen(urllib.request.Request(url, headers={"User-Agent": "ternlang-dash"}), timeout=10)
            return round((time.time() - start) * 1000)
        except: continue
    return None

def _to_num(v):
    if v is None: return 0
    s = str(v).replace('▲','').replace('▼','').replace(',','').strip()
    if 'k' in s.lower(): return int(float(s.lower().replace('k','')) * 1000)
    if 'm' in s.lower(): return int(float(s.lower().replace('m','')) * 1000000)
    try: return float(s)
    except: return 0

def merge_gh_referrers(ref_data):
    log = {}
    if os.path.exists(REFERRERS_LOG_FILE):
        try:
            with open(REFERRERS_LOG_FILE) as f: log = json.load(f)
        except: pass
    for r in (ref_data or []):
        name = r["referrer"]
        new_c, new_u = r.get("count", 0), r.get("uniques", 0)
        if name not in log:
            log[name] = {"total_count": new_c, "total_uniques": new_u, "last_14d_count": new_c, "last_14d_uniques": new_u}
        else:
            log[name]["total_count"] += max(0, new_c - log[name].get("last_14d_count", 0))
            log[name]["total_uniques"] += max(0, new_u - log[name].get("last_14d_uniques", 0))
            log[name]["last_14d_count"] = new_c
            log[name]["last_14d_uniques"] = new_u
        log[name]["last_seen"] = datetime.now().strftime("%Y-%m-%d")
    with open(REFERRERS_LOG_FILE, "w") as f: json.dump(log, f, indent=2)
    return sorted(log.items(), key=lambda x: -x[1]["total_count"])

def repo_traffic_log_path(repo):
    slug = repo.split("/")[-1]
    return os.path.join(DESKTOP, f"gh_traffic_{slug}.json")

def migrate_legacy_traffic():
    tis_path = repo_traffic_log_path(GITHUB_REPOS[0])
    if not os.path.exists(tis_path) and os.path.exists(TRAFFIC_LOG_FILE):
        import shutil; shutil.copy2(TRAFFIC_LOG_FILE, tis_path)

def merge_gh_traffic(clone_series, views_series, log_file=None):
    if log_file is None: log_file = TRAFFIC_LOG_FILE
    log = {}
    if os.path.exists(log_file):
        try:
            with open(log_file) as f: log = json.load(f)
        except: pass
    for row in clone_series:
        day = row["timestamp"][:10]
        log.setdefault(day, {})["clones"] = row["count"]
        log[day]["clones_unique"] = row.get("uniques", 0)
    for row in views_series:
        day = row["timestamp"][:10]
        log.setdefault(day, {})["views"] = row["count"]
        log[day]["views_unique"] = row.get("uniques", 0)
    with open(log_file, "w") as f: json.dump(log, f, indent=2)
    return dict(sorted(log.items()))

# -- Core Logic ---------------------------------------------------------------
def do_fetch():
    global latest_data
    print(f"[{datetime.now().strftime('%H:%M')}] TIS Signal Sync...")

    log = []
    if os.path.exists(LOG_FILE):
        try:
            with open(LOG_FILE) as f: log = json.load(f)
        except: pass
    last_entry = log[-1] if log else {}

    # Fetch Data
    cd = fetch(f"https://crates.io/api/v1/crates?per_page=100&user_id={CRATES_USER}", {"User-Agent": "ternlang-dash"})
    crates_list = []
    if cd:
        for c in cd.get("crates", []):
            crates_list.append({ "name": c["name"], "downloads": c["downloads"], "recent": c.get("recent_downloads", 0), "version": c["max_version"], "updated": c["updated_at"] })
        crates_total = sum(c["downloads"] for c in crates_list)
    else:
        crates_total = last_entry.get("crates_total", 0)
        
    openvsx_responses = [fetch(f"https://open-vsx.org/api/rfi-irfos/{ext}") for ext in OPENVSX_EXTS]
    if all(r is None for r in openvsx_responses) and last_entry.get("openvsx_total", 0) > 0:
        openvsx_total = last_entry["openvsx_total"]
    else:
        openvsx_total = sum((r or {}).get("downloadCount", 0) for r in openvsx_responses)

    hf_headers = {"User-Agent": "ternlang-dash"}
    if HF_TOKEN: hf_headers["Authorization"] = f"Bearer {HF_TOKEN}"
    hf_models_raw = fetch(f"https://huggingface.co/api/models?author={HF_AUTHOR}&limit=50", hf_headers) or []
    hf_dl_total   = sum(m.get("downloads", 0) for m in hf_models_raw)
    hf_likes_total = sum(m.get("likes", 0) for m in hf_models_raw)
    hf_model_count = len(hf_models_raw)
    hf_albert_dl  = next((m.get("downloads", 0) for m in hf_models_raw if "albert" in m.get("modelId", m.get("id","")).lower() and "about" not in m.get("modelId", m.get("id","")).lower()), 0)
    hf_org_data   = fetch(f"https://huggingface.co/api/organizations/{HF_AUTHOR}", hf_headers) or {}
    hf_followers  = hf_org_data.get("followers", last_entry.get("hf_followers", 0))
    
    migrate_legacy_traffic()
    gh_stars, gh_watchers = 0, 0
    all_traffic_agg = {}
    for _repo in GITHUB_REPOS:
        _rd = gh(f"repos/{_repo}") or {}
        gh_stars += _rd.get("stargazers_count", 0)
        gh_watchers += _rd.get("subscribers_count", 0)
        _cd = gh(f"repos/{_repo}/traffic/clones") or {}
        _vd = gh(f"repos/{_repo}/traffic/views") or {}
        _rt = merge_gh_traffic(_cd.get("clones", []), _vd.get("views", []), log_file=repo_traffic_log_path(_repo))
        for day, v in _rt.items():
            agg = all_traffic_agg.setdefault(day, {"clones": 0, "clones_unique": 0, "views": 0, "views_unique": 0})
            agg["clones"] += v.get("clones", 0)
            agg["clones_unique"] += v.get("clones_unique", 0)
            agg["views"] += v.get("views", 0)
            agg["views_unique"] += v.get("views_unique", 0)
    all_traffic = dict(sorted(all_traffic_agg.items()))

    from collections import defaultdict
    _combined_refs = defaultdict(lambda: {"count": 0, "uniques": 0})
    for _repo in GITHUB_REPOS:
        for _ref in (gh(f"repos/{_repo}/traffic/popular/referrers") or []):
            _combined_refs[_ref["referrer"]]["count"] += _ref.get("count", 0)
            _combined_refs[_ref["referrer"]]["uniques"] += _ref.get("uniques", 0)
    ref_data = merge_gh_referrers([{"referrer": k, "count": v["count"], "uniques": v["uniques"]} for k, v in _combined_refs.items()])
    paths_raw = []
    for _repo in GITHUB_REPOS:
        paths_raw.extend(gh(f"repos/{_repo}/traffic/popular/paths") or [])
    paths_data = sorted(paths_raw, key=lambda x: -x.get("count", 0))[:10]
    unique_visitors_total = sum(v.get("views_unique", 0) for v in all_traffic.values())
    total_clones, total_views = sum(v.get("clones", 0) for v in all_traffic.values()), sum(v.get("views", 0) for v in all_traffic.values())
    
    smithery_calls = "0"
    try:
        with urllib.request.urlopen("https://smithery.ai/badge/rfi-irfos/ternlang", timeout=10) as r:
            svg = r.read().decode("utf-8")
            m = re.search(r'>\s*([▲▼]?\s*[\d\.]+[kMB]?)\s*</text>', svg.split("Calls")[-1])
            if m: smithery_calls = m.group(1).replace('▲','').replace('▼','').strip()
    except: pass
    sm_val = _to_num(smithery_calls)
    api_lat = ping_api()
    now_utc = datetime.now(timezone.utc).isoformat()
    
    total_footprint = int((total_views * FOOTPRINT_WEIGHTS["views"]) + (total_clones * FOOTPRINT_WEIGHTS["clones"]) + (crates_total * FOOTPRINT_WEIGHTS["crates"]) + (openvsx_total * FOOTPRINT_WEIGHTS["openvsx"]) + (sm_val * FOOTPRINT_WEIGHTS["smithery"]) + (hf_dl_total * FOOTPRINT_WEIGHTS["hf"]))

    snapshot = { "ts": now_utc, "total_footprint": total_footprint, "crates_total": crates_total, "openvsx_total": openvsx_total, "gh_stars": gh_stars, "gh_watchers": gh_watchers, "gh_clones": total_clones, "gh_views": total_views, "gh_unique_visitors_total": unique_visitors_total, "smithery_calls_weekly": smithery_calls, "api_latency_ms": api_lat, "hf_dl_total": hf_dl_total, "hf_albert_dl": hf_albert_dl, "hf_likes": hf_likes_total, "hf_models": hf_model_count, "hf_followers": hf_followers }
    log = [e for e in log if e.get("ts", "")[:10] != now_utc[:10]]
    log.append(snapshot); log = log[-365:]
    with open(LOG_FILE, "w") as f: json.dump(log, f, indent=2)

    # Historical Reconstruction
    days_sorted = sorted(all_traffic.keys())
    reconstructed_log = []
    cum_clones, cum_views = 0, 0
    traffic_cum = {}
    for day in days_sorted:
        cum_clones += all_traffic[day].get("clones", 0); cum_views += all_traffic[day].get("views", 0)
        traffic_cum[day] = (cum_clones, cum_views)
    log_map = { e["ts"][:10]: e for e in log }
    f_entry = log[0] if log else snapshot
    for day in days_sorted:
        c_clones, c_views = traffic_cum[day]
        if day in log_map:
            entry = dict(log_map[day]); entry["gh_clones"], entry["gh_views"] = c_clones, c_views
            reconstructed_log.append(entry)
        else:
            day_f = int((c_views * FOOTPRINT_WEIGHTS["views"]) + (c_clones * FOOTPRINT_WEIGHTS["clones"]) + (f_entry.get("crates_total",0) * FOOTPRINT_WEIGHTS["crates"]) + (f_entry.get("openvsx_total",0) * FOOTPRINT_WEIGHTS["openvsx"]) + (_to_num(f_entry.get("smithery_calls_weekly","0")) * FOOTPRINT_WEIGHTS["smithery"]))
            reconstructed_log.append({ "ts": day + "T00:00:00Z", "total_footprint": day_f, "crates_total": f_entry.get("crates_total",0), "openvsx_total": f_entry.get("openvsx_total",0), "gh_stars": f_entry.get("gh_stars",0), "gh_clones": c_clones, "gh_views": c_views, "smithery_calls_weekly": f_entry.get("smithery_calls_weekly","0"), "hf_dl_total": f_entry.get("hf_dl_total",0) })

    latest_data = { "snapshot": snapshot, "prev": log[-2] if len(log) >= 2 else snapshot, "chart_log": reconstructed_log, "momentum": compute_momentum(reconstructed_log), "macd": compute_macd(reconstructed_log), "crates_list": crates_list, "all_traffic": all_traffic, "top_referrers": ref_data, "top_paths": paths_data, "updated_at": datetime.now().strftime("%Y-%m-%d %H:%M:%S") }
    render_html()
    return latest_data

def compute_momentum(log):
    m = {"views": [], "clones": [], "crates": [], "openvsx": [], "smithery": [], "hf": []}
    for i in range(len(log)):
        if i == 0:
            for k in m: m[k].append(0)
            continue
        curr, prev = log[i], log[i-1]
        m["views"].append(_to_num(curr.get("gh_views",0)) - _to_num(prev.get("gh_views",0)))
        m["clones"].append(_to_num(curr.get("gh_clones",0)) - _to_num(prev.get("gh_clones",0)))
        m["crates"].append(_to_num(curr.get("crates_total",0)) - _to_num(prev.get("crates_total",0)))
        m["openvsx"].append(_to_num(curr.get("openvsx_total",0)) - _to_num(prev.get("openvsx_total",0)))
        m["smithery"].append(_to_num(curr.get("smithery_calls_weekly",0)) - _to_num(prev.get("smithery_calls_weekly",0)))
        m["hf"].append(_to_num(curr.get("hf_dl_total",0)) - _to_num(prev.get("hf_dl_total",0)))
    return m

def compute_macd(log):
    values = [_to_num(e.get("total_footprint", 0)) for e in log]
    if not values: return {"macd": [], "signal": [], "hist": []}
    def ema(data, period):
        k = 2 / (period + 1); res = []; curr = data[0]
        for v in data: curr = (v - curr) * k + curr; res.append(curr)
        return res
    e7, e14 = ema(values, 7), ema(values, 14); m_line = [f - s for f, s in zip(e7, e14)]; s_line = ema(m_line, 5)
    return {"macd": m_line, "signal": s_line, "hist": [mv - sv for mv, sv in zip(m_line, s_line)]}

def render_html():
    d = latest_data; s, p = d["snapshot"], d["prev"]
    macd_v = d["macd"]["hist"][-1] if d["macd"]["hist"] else 0
    sent = "BULLISH" if macd_v > 0 else ("BEARISH" if macd_v < 0 else "STABLE")
    sent_cls = "sig-bullish" if sent == "BULLISH" else ("sig-bearish" if sent == "BEARISH" else "")
    recent_l = d["chart_log"][-7:]
    daily_v = (_to_num(recent_l[-1].get("total_footprint", 0)) - _to_num(recent_l[0].get("total_footprint", 0))) / max(1, len(recent_l)-1) if len(recent_l) >= 2 else 0
    _milestones = [100_000, 150_000, 200_000, 250_000, 500_000, 750_000, 1_000_000, 2_000_000, 5_000_000, 10_000_000]
    _cur_fp = _to_num(s["total_footprint"])
    _next_target = next((m for m in _milestones if m > _cur_fp), int(_cur_fp * 2))
    _target_label = f"{_next_target // 1_000_000}M" if _next_target >= 1_000_000 else f"{_next_target // 1_000}K"
    eta_days = round((_next_target - _cur_fp) / daily_v) if daily_v > 0 else "N/A"
    
    def trend_box(curr, key):
        if key == "api_latency_ms": return ""
        c, b = _to_num(curr), _to_num(p.get(key, 0)); diff = c - b; pct = (diff / b * 100) if b else 0
        color = "var(--green)" if diff > 0 else ("var(--red)" if diff < 0 else "var(--muted)")
        arrow = "▲" if diff > 0 else ("▼" if diff < 0 else "•")
        return f'<div class="card-trend" style="color:{color}"><div class="trend-top">{arrow} {pct:+.1f}%</div><div class="trend-bottom">({diff:+g})</div></div>'

    cards_html = ""
    cards = [
        ("Total Footprint", s["total_footprint"], "total_footprint", "c-teal", True),
        ("Total Clones", s["gh_clones"], "gh_clones", "", True),
        ("Total Views", s["gh_views"], "gh_views", "", True),
        ("Visitors (All-Time)", s["gh_unique_visitors_total"], "gh_unique_visitors_total", "", True),
        ("GH Stars", s["gh_stars"], "gh_stars", "", False),
        ("GH Watchers", s["gh_watchers"], "gh_watchers", "", False),
        ("Smithery Calls", s["smithery_calls_weekly"], "smithery_calls_weekly", "", False),
        ("Crates DL", s["crates_total"], "crates_total", "", True),
        ("Open VSX", s["openvsx_total"], "openvsx_total", "", True),
        ("Latency", f"{s['api_latency_ms']}ms" if s.get('api_latency_ms') else "N/A", "api_latency_ms", "", False),
    ]
    for lbl, val, key, cls, is_num in cards:
        v_str = f"{val:,}" if is_num else str(val)
        cards_html += f'<div class="kpi-card {cls}"><div class="card-left"><div class="label">{lbl}</div><div class="value">{v_str}</div></div>{trend_box(val, key)}</div>'

    hf_cards_html = ""
    hf_cards = [
        ("HF Downloads", s.get("hf_dl_total", 0), "hf_dl_total", "", True),
        ("albert. DL", s.get("hf_albert_dl", 0), "hf_albert_dl", "", True),
        ("HF Likes", s.get("hf_likes", 0), "hf_likes", "", True),
        ("HF Models", s.get("hf_models", 0), "hf_models", "", False),
        ("HF Followers", s.get("hf_followers", 0), "hf_followers", "", True),
    ]
    for lbl, val, key, cls, is_num in hf_cards:
        v_str = f"{val:,}" if is_num else str(val)
        hf_cards_html += f'<div class="kpi-card {cls}"><div class="card-left"><div class="label">{lbl}</div><div class="value">{v_str}</div></div>{trend_box(val, key)}</div>'

    def row(l, cur, pr):
        c, p_v = _to_num(cur), _to_num(pr); d_v, pct = c - p_v, (c-p_v)/p_v*100 if p_v else 0
        cls = "ch-up" if d_v > 0 else ("ch-dn" if d_v < 0 else "ch-na")
        cur_display = f"{cur:,}" if isinstance(cur, (int, float)) else str(cur)
        return f'<div class="grid-row vol-grid"><div>{l}</div><div class="num">{cur_display}</div><div class="num {cls}">{d_v:+g}</div><div class="num {cls}">{pct:+.1f}%</div></div>'
    vol_rows = "\n".join([row("Footprint", s["total_footprint"], p.get("total_footprint")), row("Crates", s["crates_total"], p.get("crates_total")), row("OpenVSX", s["openvsx_total"], p.get("openvsx_total")), row("Clones", s["gh_clones"], p.get("gh_clones")), row("Smithery", s["smithery_calls_weekly"], p.get("smithery_calls_weekly"))])
    def stat_card(label, cur, prev_val):
        c, pv = _to_num(cur), _to_num(prev_val)
        d_v, pct = c - pv, (c-pv)/pv*100 if pv else 0
        cls = "ch-up" if d_v > 0 else ("ch-dn" if d_v < 0 else "ch-na")
        arrow = "▲" if d_v > 0 else ("▼" if d_v < 0 else "•")
        disp = f"{cur:,}" if isinstance(cur, (int, float)) else str(cur)
        return f'<div class="vol-stat"><div class="vol-stat-label">{label}</div><div class="vol-stat-val">{disp}</div><div class="vol-stat-delta {cls}">{arrow} {pct:+.1f}% ({d_v:+g})</div></div>'
    vol_banner = "".join([stat_card("Total Footprint", s["total_footprint"], p.get("total_footprint")), stat_card("Crates DL", s["crates_total"], p.get("crates_total")), stat_card("OpenVSX", s["openvsx_total"], p.get("openvsx_total")), stat_card("GH Clones", s["gh_clones"], p.get("gh_clones")), stat_card("Smithery", s["smithery_calls_weekly"], p.get("smithery_calls_weekly"))])
    ref_rows = "\n".join(f'<div class="grid-row traffic-grid"><div>{name}</div><div class="num">{v["total_count"]}</div><div class="num">{v["total_uniques"]}</div><div class="center">{v.get("last_seen","—")}</div></div>' for name, v in d.get("top_referrers", []))
    path_rows = "\n".join(f'<div class="grid-row traffic-grid"><div>{pv["path"]}</div><div class="num">{pv["count"]}</div><div class="num">{pv["uniques"]}</div><div class="center">-</div></div>' for pv in d.get("top_paths", []))
    traffic_rows = "\n".join(f'<div class="grid-row traffic-grid"><div>{day}</div><div class="num">{v.get("clones",0)}</div><div class="num">{v.get("views",0)}</div><div class="center">-</div></div>' for day, v in sorted(d["all_traffic"].items(), reverse=True))
    crates_rows = "\n".join(f'<div class="grid-row crates-grid"><div>{c["name"]}</div><div class="num" style="color:var(--blue)">v{c["version"]}</div><div class="num">{c["downloads"]:,}</div><div class="num" style="color:var(--green)">+{c.get("recent",0):,}</div></div>' for c in sorted(d["crates_list"], key=lambda x: -x["downloads"]))
    sig_bar = f'<div class="signal-bar"><div class="sig-item">SENTIMENT: <span class="{sent_cls}">{sent}</span></div><div class="sig-item">TARGET {_target_label}: <span>{eta_days} DAYS</span></div><div class="sig-item">SYNC: <span>{d["updated_at"]}</span></div><div class="sig-item" style="margin-left:auto">SIGNAL STRENGTH: <span>{abs(macd_v):.2f}</span></div></div>'

    def filter_html(k, default=0):
        sel = lambda v: ' selected' if v == default else ''
        return f'<div class="dropdown-pill"><select onchange="setFilter(\'{k}\',this.value)"><option value="0"{sel(0)}>ALL</option><option value="30"{sel(30)}>1M</option><option value="14"{sel(14)}>2W</option><option value="7"{sel(7)}>1W</option></select></div>'

    html = f"""<!DOCTYPE html>
<html lang="en"><head><meta charset="UTF-8"><title>RFI-IRFOS KPI | {sent} {s["total_footprint"]:,}</title>
<link rel="icon" type="image/png" href="/favicon.png"><link rel="shortcut icon" href="/favicon.png">
<script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
<style>
  @import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;700;800&display=swap');
  :root {{ --bg:#0d1117; --bg2:#161b22; --bg3:#21262d; --border:#30363d; --text:#e6edf3; --muted:#ffffff; --green:#3fb950; --blue:#58a6ff; --purple:#a371f7; --orange:#d29922; --red:#f85149; }}
  body.light {{ background: linear-gradient(135deg, #e8edf2 0%, #d4dde8 100%); --bg:#edf0f4; --bg2:rgba(255,255,255,0.95); --bg3:#e2e8ef; --border:#c8d0da; --text:#1a1f26; --muted:#1a1f26; }}
  body.light .panel {{ background:rgba(255,255,255,0.85); border-color:rgba(0,0,0,0.08); }}
  body.light .kpi-card {{ background:rgba(255,255,255,0.85); border-color:rgba(0,0,0,0.08); }}
  body.light .signal-bar {{ background:rgba(255,255,255,0.85); }}
  body.light .tab-bar {{ background:rgba(255,255,255,0.95); }}
  body.light .grid-header {{ background:#dde3ea; }}
  body.light .grid-row {{ border-bottom-color:rgba(0,0,0,0.06); }}
  body.light .legend-pill {{ background:rgba(0,0,0,0.05); border-color:rgba(0,0,0,0.12); color:var(--text); }}
  body.light .legend-pill.active {{ background:rgba(0,0,0,0.12); border-color:var(--text); }}
  body.light .dropdown-pill select {{ background:rgba(0,0,0,0.06); border-color:rgba(0,0,0,0.15); color:var(--text); }}
  * {{ box-sizing:border-box; margin:0; padding:0; }}
  body {{ background: linear-gradient(135deg, #0d1117 0%, #0a192f 50%, #003a4d 100%); color:var(--text); font-family:'JetBrains Mono', monospace; font-size:14px; overflow-x:hidden; background-attachment: fixed; }}
  header {{ background:rgba(22, 27, 34, 0.8); border-bottom:2px solid var(--border); padding:10px 24px; display:flex; align-items:center; justify-content:space-between; position:sticky; top:0; z-index:100; backdrop-filter: blur(10px); }}
  body.light header {{ background:rgba(255, 255, 255, 0.8); border-bottom-color:rgba(0,0,0,0.05); }}
  .hdr-right {{ display:flex; gap:16px; }}
  .icon-btn {{ background:var(--bg3); border:1px solid var(--border); border-radius:10px; width:36px; height:36px; display:flex; align-items:center; justify-content:center; color:var(--text); cursor:pointer; transition:all 0.3s; }}
  .icon-btn svg {{ width:20px; height:20px; fill:currentColor; }}
  .btn-green {{ background:var(--green); border:none; }}
  main {{ max-width:1560px; margin:0 auto; padding:16px 28px; }}
  .tab-bar {{ background:rgba(13,17,23,0.95); border-bottom:1px solid var(--border); display:flex; gap:0; padding:0 28px; backdrop-filter:blur(10px); position:sticky; top:53px; z-index:90; }}
  .tab-btn {{ padding:9px 18px; font-size:10px; font-weight:800; text-transform:uppercase; letter-spacing:0.6px; color:var(--muted); opacity:0.4; cursor:pointer; border-bottom:2px solid transparent; transition:all 0.2s; user-select:none; }}
  .tab-btn:hover {{ opacity:0.75; }}
  .tab-btn.active {{ opacity:1; color:var(--blue); border-bottom-color:var(--blue); }}
  .tab-pane {{ display:none; }}
  .tab-pane.active {{ display:block; }}
  .kpi-grid {{ display:grid; grid-template-columns:repeat(5, 1fr); gap:10px; margin-bottom:14px; }}
  .kpi-card {{ background:rgba(22, 27, 34, 0.4); border:1px solid rgba(255,255,255,0.08); border-radius:14px; padding:14px; border-top:3px solid rgba(255,255,255,0.06); display:flex; justify-content:space-between; align-items: flex-end; min-height:95px; backdrop-filter: blur(16px); will-change:transform; transition: transform 0.45s cubic-bezier(0.23,1,0.32,1), box-shadow 0.45s ease; }}
  .kpi-card:hover {{ transform: translateY(-9px); box-shadow: 0 2px 6px rgba(0,0,0,0.25), 0 12px 40px rgba(0,0,0,0.35), 0 32px 80px rgba(0,0,0,0.18), inset 0 1px 0 rgba(255,255,255,0.07); }}
  .c-teal {{ border-top:3px solid #2dd4bf; box-shadow: 0 0 24px rgba(45,212,191,0.12), inset 0 1px 0 rgba(45,212,191,0.08); }}
  .c-teal .value {{ color:#2dd4bf; }}
  .c-teal:hover {{ box-shadow: 0 2px 6px rgba(0,0,0,0.25), 0 12px 40px rgba(45,212,191,0.18), 0 32px 80px rgba(45,212,191,0.10), inset 0 1px 0 rgba(45,212,191,0.12); }}
  .section-label {{ font-size:9px; font-weight:800; text-transform:uppercase; letter-spacing:0.12em; color:var(--muted); opacity:0.4; padding: 10px 2px 4px; }}
  .label {{ font-size:10px; text-transform:uppercase; color:var(--muted); font-weight:800; margin-bottom:5px; opacity:0.7; }}
  .value {{ font-size:21px; font-weight:800; line-height:1; letter-spacing:-1px; }}
  .card-trend {{ text-align:right; font-weight:800; }}
  .trend-top {{ font-size:12px; }} .trend-bottom {{ font-size:10px; opacity:0.8; }}
  .row-top2 {{ display:grid; grid-template-columns:1fr 1fr; gap:14px; margin-bottom:14px; }}
  .row-mid2 {{ display:grid; grid-template-columns:1fr 1fr; gap:14px; margin-bottom:14px; }}
  .row-bottom {{ display:grid; grid-template-columns:1fr 1fr 0.6fr; gap:14px; margin-bottom:14px; }}
  .panel {{ background:rgba(22, 27, 34, 0.4); border:1px solid rgba(255,255,255,0.1); border-radius:14px; padding:14px; display:flex; flex-direction:column; backdrop-filter: blur(12px); }}
  .panel-h {{ font-size:11px; font-weight:800; color:var(--muted); text-transform:uppercase; margin-bottom:8px; border-bottom:1px solid var(--border); padding-bottom:7px; display:flex; justify-content:space-between; align-items:center; gap:8px; }}
  .grid-table {{ display: flex; flex-direction: column; width: 100%; font-size:11px; }}
  .grid-row {{ display: grid; border-bottom: 1px solid var(--border); padding: 6px 10px; }}
  .grid-header {{ background: var(--bg3); font-size: 11px; text-transform: uppercase; font-weight: 800; position: sticky; top: 0; z-index: 20; }}
  .num {{ text-align: right; font-weight: 700; }} .center {{ text-align: center; }}
  .vol-grid {{ grid-template-columns: 1.5fr 1fr 1fr 1fr; }} .traffic-grid {{ grid-template-columns: 1.5fr 0.8fr 0.8fr 0.6fr; }} .crates-grid {{ grid-template-columns: 1.5fr 0.8fr 1fr 1fr; }}
  .impact-badge {{ display: inline-block; padding: 2px 8px; border-radius: 4px; font-weight: 900; font-size: 10px; min-width: 30px; }}
  .impact-pos {{ background:rgba(63,185,80,0.15); color:var(--green); border:1px solid var(--green); }}
  .impact-neu {{ background:rgba(255,255,255,0.05); color:var(--muted); border:1px solid var(--border); }}
  .impact-neg {{ background:rgba(248,81,73,0.15); color:var(--red); border:1px solid var(--red); }}
  .legend-container {{ display:flex; flex-wrap:nowrap; gap:8px; margin-bottom:8px; align-items:center; min-height:26px; width:100%; }}
  .legend-pill {{ display:flex; align-items:center; gap:5px; padding:3px 8px; border-radius:50px; background:rgba(255,255,255,0.05); border:1px solid rgba(255,255,255,0.1); cursor:pointer; font-size:11px; font-weight:800; color:var(--muted); white-space:nowrap; }}
  .legend-pill.active {{ background:rgba(255,255,255,0.15); border-color:var(--text); color:var(--text); }}
  .legend-pill .color-dot {{ width:10px; height:10px; border-radius:50%; }}
  .signal-bar {{ background:rgba(0,0,0,0.3); border-bottom:1px solid var(--border); padding:5px 24px; display:flex; gap:20px; font-size:11px; font-weight:800; color:var(--muted); backdrop-filter: blur(10px); }}
  .sig-item span {{ color:var(--text); margin-left:6px; }}
  #loading-bar {{ position:fixed; top:0; left:0; height:3px; background:var(--blue); width:0; transition:width 0.3s; z-index:200; }}
  .dropdown-pill {{ position:relative; margin-left:auto; }}
  .dropdown-pill select {{ background:rgba(255,255,255,0.05); border:1px solid rgba(255,255,255,0.1); border-radius:50px; color:var(--text); font-family:'JetBrains Mono', monospace; font-size:10px; font-weight:800; padding:3px 22px 3px 9px; cursor:pointer; appearance:none; -webkit-appearance:none; }}
  .dropdown-pill select:focus {{ outline:none; border-color:var(--blue); }}
  .dropdown-pill::after {{ content:'▼'; position:absolute; right:12px; top:50%; transform:translateY(-50%); font-size:10px; pointer-events:none; opacity:0.6; }}
  .row-mid3 {{ display:grid; grid-template-columns:1fr 1fr 0.62fr; gap:14px; margin-bottom:14px; }}
  .donut-hub {{ position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); text-align: center; pointer-events: none; width: 130px; }}
  .hub-label {{ font-size: 9px; font-weight: 800; color: var(--muted); text-transform: uppercase; letter-spacing:0.1em; margin-bottom:2px; transition:color 0.3s; }}
  .hub-value {{ font-size: 20px; font-weight: 800; color: var(--text); line-height:1.1; transition:color 0.3s; }}
  .hub-sub {{ font-size: 9px; color: var(--muted); margin-top:4px; opacity:0.7; line-height:1.4; }}
  .ch-up {{ color: var(--green); }} .ch-dn {{ color: var(--red); }} .ch-na {{ color: var(--muted); }}
  .bottom-floor {{ display:grid; grid-template-columns: 1fr 1fr 1fr; gap:16px; }}
  .data-main {{ display:grid; grid-template-columns:1.4fr 1fr 1.3fr 1.1fr; gap:16px; }}
  #tab-data .grid-table {{ font-size:12px; }}
  #tab-data .grid-row {{ padding:8px 12px; }}
  #tab-data .grid-header {{ font-size:10px; }}
  .vol-banner {{ display:grid; grid-template-columns:repeat(5,1fr); gap:12px; margin-bottom:16px; }}
  .vol-stat {{ background:rgba(22,27,34,0.4); border:1px solid rgba(255,255,255,0.1); border-radius:12px; padding:14px 16px; backdrop-filter:blur(12px); }}
  body.light .vol-stat {{ background:rgba(255,255,255,0.85); border-color:rgba(0,0,0,0.08); }}
  .vol-stat-label {{ font-size:9px; font-weight:800; text-transform:uppercase; opacity:0.55; margin-bottom:4px; }}
  .vol-stat-val {{ font-size:20px; font-weight:800; letter-spacing:-0.5px; margin-bottom:4px; }}
  .vol-stat-delta {{ font-size:11px; font-weight:700; }}
</style></head>
<body><div id="loading-bar"></div>
<header>
  <div><h1 style="margin:0;font-size:20px;">RFI-IRFOS <span style="color:var(--blue)">KPI</span></h1></div>
  <div class="hdr-right">
    <button class="icon-btn" onclick="exportCSV()"><svg viewBox="0 0 24 24"><path d="M19 12v7H5v-7H3v7c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-7h-2zm-6 .67l2.59-2.58L17 11.5l-5 5-5-5 1.41-1.41L11 12.67V3h2v9.67z"/></svg></button>
    <button class="icon-btn" onclick="toggleTheme()"><svg id="theme-icon" viewBox="0 0 24 24"><path d="M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1z"/></svg></button>
    <button class="icon-btn btn-green" id="refreshBtn" onclick="refreshData()"><svg viewBox="0 0 24 24"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg></button>
  </div>
</header>
{sig_bar}
<div class="tab-bar"><div class="tab-btn active" data-tab="overview" onclick="switchTab('overview')">Overview</div><div class="tab-btn" data-tab="data" onclick="switchTab('data')">Data &amp; Tables</div></div>
<main>
  <div id="tab-overview" class="tab-pane active">
  <div class="kpi-grid">{cards_html}</div>
  <div class="section-label">Hugging Face</div>
  <div class="kpi-grid">{hf_cards_html}</div>
  <div class="row-top2">
    <div class="panel"><div class="panel-h">Footprint Pulse{filter_html('growth')}</div><div class="legend-container"><div id="lgd-growth" style="display:flex;gap:8px"></div></div><div style="height:300px"><canvas id="growthChart"></canvas></div></div>
    <div class="panel"><div class="panel-h">GH Traffic Reach{filter_html('reach')}</div><div class="legend-container"><div id="lgd-reach" style="display:flex;gap:8px"></div></div><div style="height:300px"><canvas id="reachChart"></canvas></div></div>
  </div>
  <div class="row-mid3">
    <div class="panel"><div class="panel-h">Momentum Shift{filter_html('momentum')}</div><div class="legend-container"><div id="lgd-momentum" style="display:flex;gap:8px"></div></div><div style="height:260px"><canvas id="momentumChart"></canvas></div></div>
    <div class="panel"><div class="panel-h">MACD Momentum{filter_html('macd')}</div><div class="legend-container"><div id="lgd-macd" style="display:flex;gap:8px"></div></div><div style="height:260px"><canvas id="macdChart"></canvas></div></div>
    <div class="panel"><div class="panel-h">Footprint Mix</div><div id="lgd-dist" class="legend-container" style="justify-content:center;flex-wrap:wrap;gap:6px"></div><div style="height:220px;position:relative;margin-top:4px"><canvas id="distChart"></canvas><div class="donut-hub"><div class="hub-label" id="hub-label">FOOTPRINT</div><div class="hub-value" id="hub-value">{s["total_footprint"]:,}</div><div class="hub-sub" id="hub-sub"></div></div></div></div>
  </div>
  </div>
  <div id="tab-data" class="tab-pane">
  <div class="vol-banner">{vol_banner}</div>
  <div class="data-main">
    <div class="panel" style="padding:0; overflow-y:auto;"><div class="panel-h" style="padding:14px 16px;position:sticky;top:0;background:var(--bg2);z-index:10;margin:0;">Crates Detail</div><div class="grid-table"><div class="grid-row crates-grid grid-header"><div>Package</div><div class="num">Version</div><div class="num">Downloads</div><div class="num">New</div></div>{crates_rows}</div></div>
    <div style="display:flex;flex-direction:column;gap:16px;">
      <div class="panel" style="padding:0; overflow-y:auto;"><div class="panel-h" style="padding:14px 16px;position:sticky;top:0;background:var(--bg2);z-index:10;margin:0;">Referring Sites (All-Time)</div><div class="grid-table"><div class="grid-row traffic-grid grid-header"><div>Referrer</div><div class="num">Views</div><div class="num">Uniques</div><div class="center">Last Seen</div></div>{ref_rows}</div></div>
    </div>
    <div class="panel" style="padding:0; overflow-y:auto;"><div class="panel-h" style="padding:14px 16px;position:sticky;top:0;background:var(--bg2);z-index:10;margin:0;">Popular Content</div><div class="grid-table"><div class="grid-row traffic-grid grid-header"><div>Path</div><div class="num">Views</div><div class="num">Uniques</div><div class="center">Impact</div></div>{path_rows}</div></div>
    <div class="panel" style="padding:0; overflow-y:auto;"><div class="panel-h" style="padding:14px 16px;position:sticky;top:0;background:var(--bg2);z-index:10;margin:0;">Recent Traffic</div><div class="grid-table"><div class="grid-row traffic-grid grid-header"><div>Date</div><div class="num">Clones</div><div class="num">Views</div><div class="center">Impact</div></div>{traffic_rows}</div></div>
  </div>
  </div>
</main>
<script>
const P = {{ teal:'#2dd4bf', grn:'#4ade80', blu:'#60a5fa', vio:'#a78bfa', org:'#fb923c', amb:'#fbbf24', cyn:'#22d3ee', red:'#f87171', sig:'#f59e0b', pur:'#c084fc' }};
let charts = {{}}, filters = {{ growth:0, reach:0, momentum:0, macd:0 }};
const _to_num = (v) => {{ if (v===null||v===undefined) return 0; if (typeof v==='number') return v; const s=String(v).replace(/[▲▼,]/g,'').trim(); if(s.toLowerCase().endsWith('k')) return parseFloat(s)*1000; if(s.toLowerCase().endsWith('m')) return parseFloat(s)*1000000; return parseFloat(s)||0; }};
function createLegend(chart, id) {{
    const c = document.getElementById(id); if (!c) return; c.innerHTML = '';
    chart.data.datasets.forEach((ds, i) => {{
        if (ds.label === 'Vol') return;
        const p = document.createElement('div'); p.className = 'legend-pill active';
        p.innerHTML = `<div class="color-dot" style="background:${{ds.borderColor||ds.backgroundColor}}"></div><span>${{ds.label}}</span>`;
        p.onclick = () => {{ chart.setDatasetVisibility(i, !chart.isDatasetVisible(i)); chart.update(); p.classList.toggle('active'); }};
        c.appendChild(p);
    }});
}}
function setFilter(ck, d) {{ filters[ck] = parseInt(d); updateChart(ck); }}
function updateChart(k) {{
    const data = latest_data_js; let L = data.chart_log; if (filters[k] > 0) L = L.slice(-filters[k]);
    const labels = L.map(e=>e.ts.slice(5,10));
    if (k === 'growth') {{ charts.growth.data.labels = labels; charts.growth.data.datasets[0].data = L.map(e=>e.total_footprint); charts.growth.data.datasets[1].data = L.map(e=>e.crates_total); charts.growth.data.datasets[2].data = L.map(e=>e.openvsx_total); charts.growth.data.datasets[3].data = L.map(e=>e.gh_clones); charts.growth.data.datasets[4].data = L.map(e=>e.gh_views); charts.growth.data.datasets[5].data = L.map(e=>e.hf_dl_total||0); charts.growth.update(); }}
    if (k === 'reach') {{
        charts.reach.data.labels = labels;
        charts.reach.data.datasets[0].data = L.map((e,i) => i===0?0:_to_num(e.gh_views)-_to_num(L[i-1].gh_views));
        charts.reach.data.datasets[1].data = L.map(e => data.all_traffic[e.ts.slice(0,10)]?.views_unique || 0);
        charts.reach.update();
    }}
    if (k === 'momentum') {{ const s = (arr) => filters.momentum > 0 ? arr.slice(-filters.momentum) : arr; charts.momentum.data.labels = labels; charts.momentum.data.datasets[0].data = s(data.momentum.views); charts.momentum.data.datasets[1].data = s(data.momentum.clones); charts.momentum.data.datasets[2].data = s(data.momentum.crates); charts.momentum.data.datasets[3].data = s(data.momentum.openvsx); charts.momentum.data.datasets[4].data = s(data.momentum.hf||[]); charts.momentum.update(); }}
    if (k === 'macd') {{
        const sl = (arr) => filters.macd > 0 ? arr.slice(-filters.macd) : arr;
        const hist = sl(data.macd.hist);
        charts.macd.data.labels = labels;
        charts.macd.data.datasets[0].data = sl(data.macd.macd);
        charts.macd.data.datasets[1].data = sl(data.macd.signal);
        charts.macd.data.datasets[2].data = hist;
        charts.macd.data.datasets[2].backgroundColor = hist.map(v => v >= 0 ? 'rgba(63,185,80,0.8)' : 'rgba(248,81,73,0.8)');
        charts.macd.data.datasets[2].borderColor = hist.map(v => v >= 0 ? '#3fb950' : '#f85149');
        charts.macd.data.datasets[2].borderWidth = 1;
        charts.macd.data.datasets[3].data = L.map((e,i) => i===0 ? 0 : e.total_footprint - L[i-1].total_footprint);
        charts.macd.update();
    }}
}}
function exportCSV() {{
    const L = latest_data_js.chart_log; let csv = "Timestamp,Footprint,Crates,OpenVSX,Clones,Views\\n";
    L.forEach(e => csv += `${{e.ts}},${{e.total_footprint}},${{e.crates_total}},${{e.openvsx_total}},${{e.gh_clones}},${{e.gh_views}}\\n`);
    const a = document.createElement('a'); a.href = window.URL.createObjectURL(new Blob([csv], {{type:'text/csv'}})); a.download = 'ternlang_kpi.csv'; a.click();
}}
function initCharts(data) {{
    const isL = document.body.classList.contains('light'); const gC = isL ? 'rgba(0,0,0,0.08)' : 'rgba(255,255,255,0.05)';
    Chart.defaults.color = isL ? '#1f2328' : '#e6edf3'; Chart.defaults.font.family = "'JetBrains Mono', monospace";
    const anim = {{ animation:{{ duration:600, easing:'easeInOutQuart' }} }};
    const common = {{ responsive:true, maintainAspectRatio:false, animation:anim.animation, plugins:{{legend:{{display:false}}}}, elements:{{point:{{radius:3, hoverRadius:6}}}}, scales:{{x:{{grid:{{display:false}}, ticks:{{maxTicksLimit:8}}}}, y:{{grid:{{color:gC}}, ticks:{{maxTicksLimit:6}}}} }} }};
    charts.growth = new Chart(document.getElementById('growthChart'), {{ type:'line', data: {{ labels:[], datasets: [{{label:'Total', borderColor:P.teal, borderWidth:3, tension:0.4, fill:false, data:[]}}, {{label:'Crates', borderColor:P.grn, borderDash:[5,5], borderWidth:2, tension:0.4, fill:false, data:[]}}, {{label:'OpenVSX', borderColor:P.blu, borderDash:[5,5], borderWidth:2, tension:0.4, fill:false, data:[]}}, {{label:'Clones', borderColor:P.pur, borderWidth:2, tension:0.4, fill:false, data:[]}}, {{label:'Views', borderColor:P.org, borderWidth:2, tension:0.4, fill:false, data:[]}}, {{label:'HF DL', borderColor:'#f59e0b', borderDash:[3,3], borderWidth:2, tension:0.4, fill:false, data:[]}}] }}, options: common }});
    charts.reach = new Chart(document.getElementById('reachChart'), {{ type:'line', data: {{ labels:[], datasets: [{{label:'Daily Views', borderColor:P.org, borderWidth:2.5, tension:0.4, fill:false, data:[]}}, {{label:'Uniques', borderColor:P.cyn, borderDash:[5,5], borderWidth:2.5, tension:0.4, fill:false, data:[]}}] }}, options: common }});
    charts.momentum = new Chart(document.getElementById('momentumChart'), {{ type:'line', data: {{ labels:[], datasets: [{{label:'Views Δ', borderColor:P.org, borderWidth:2.5, tension:0.35, fill:false, data:[]}}, {{label:'Clones Δ', borderColor:P.vio, borderWidth:2.5, tension:0.35, fill:false, data:[]}}, {{label:'Crates Δ', borderColor:P.grn, borderDash:[5,5], borderWidth:2, tension:0.35, fill:false, data:[]}}, {{label:'OpenVSX Δ', borderColor:P.blu, borderDash:[5,5], borderWidth:2, tension:0.35, fill:false, data:[]}}, {{label:'HF Δ', borderColor:P.amb, borderDash:[3,3], borderWidth:2, tension:0.35, fill:false, data:[]}}] }}, options: common }});
    charts.macd = new Chart(document.getElementById('macdChart'), {{ type:'bar', data: {{ labels:[], datasets: [{{label:'MACD', type:'line', borderColor:P.cyn, borderWidth:2, tension:0.4, fill:false, pointRadius:2, data:[]}}, {{label:'Signal', type:'line', borderColor:P.sig, borderDash:[5,5], borderWidth:1.5, tension:0.4, fill:false, pointRadius:2, data:[]}}, {{label:'Hist', type:'bar', backgroundColor:[], borderColor:[], borderWidth:1, data:[]}}, {{label:'Vol', type:'bar', data:[], backgroundColor:P.vio+'22', yAxisID:'y1'}}] }}, options: {{ ...common, scales:{{ ...common.scales, y1:{{ position:'right', grid:{{display:false}}, display:false }} }} }} }});
    const _s = data.snapshot;
    const _wt = {{ crates:4.0, openvsx:5.0, clones:2.0, views:1.0, hf:3.0 }};
    const _distLabels = ['Crates','OpenVSX','Clones','Views','HF DL'];
    const _distColors = [P.grn, P.blu, P.vio, P.org, P.amb];
    const _rawData = [_s.crates_total||0, _s.openvsx_total||0, _s.gh_clones||0, _s.gh_views||0, _s.hf_dl_total||0];
    const _wtArr = [4.0, 5.0, 2.0, 1.0, 3.0];
    const _distData = _rawData.map((v,i) => v * _wtArr[i]);
    const _distTotal = _distData.reduce((a,b)=>a+b,0) || 1;
    let _selSlice = -1;
    function _donutReset() {{
        _selSlice = -1;
        charts.dist.data.datasets[0].offset = [0,0,0,0,0];
        charts.dist.update('active');
        document.getElementById('hub-label').textContent = 'FOOTPRINT';
        document.getElementById('hub-value').textContent = (_s.total_footprint||0).toLocaleString();
        document.getElementById('hub-value').style.color = 'var(--text)';
        document.getElementById('hub-sub').textContent = '';
        document.querySelectorAll('#lgd-dist .legend-pill').forEach(p => {{ p.style.opacity='1'; p.style.transform='scale(1)'; }});
    }}
    function _donutFocus(idx) {{
        _selSlice = idx;
        const off = [0,0,0,0,0]; off[idx] = 18;
        charts.dist.data.datasets[0].offset = off;
        charts.dist.update('active');
        const pct = (_distData[idx]/_distTotal*100).toFixed(1);
        document.getElementById('hub-label').textContent = _distLabels[idx].toUpperCase();
        document.getElementById('hub-value').textContent = _rawData[idx].toLocaleString();
        document.getElementById('hub-value').style.color = _distColors[idx];
        document.getElementById('hub-sub').textContent = `×${{_wtArr[idx]}} weight · ${{pct}}% of mix`;
        document.querySelectorAll('#lgd-dist .legend-pill').forEach((p,i) => {{
            p.style.opacity = i===idx ? '1' : '0.25';
            p.style.transform = i===idx ? 'scale(1.08)' : 'scale(1)';
        }});
    }}
    charts.dist = new Chart(document.getElementById('distChart'), {{
        type:'doughnut',
        data:{{ labels:_distLabels, datasets:[{{ data:_distData, backgroundColor:_distColors.map(c=>c+'bb'), hoverBackgroundColor:_distColors, borderWidth:3, borderColor:'rgba(13,17,23,0.95)', hoverBorderColor:_distColors, hoverOffset:10, offset:[0,0,0,0,0], cutout:'74%' }}] }},
        options:{{ responsive:true, maintainAspectRatio:false, animation:{{animateScale:true, animateRotate:true, duration:700, easing:'easeInOutQuart'}}, plugins:{{legend:{{display:false}}, tooltip:{{enabled:false}}}},
            onClick:(evt, els) => {{ if (!els.length) {{ _donutReset(); return; }} const i=els[0].index; if(_selSlice===i) _donutReset(); else _donutFocus(i); }}
        }}
    }});
    document.getElementById('distChart').style.cursor = 'pointer';
    (function() {{
        const c = document.getElementById('lgd-dist'); if (!c) return; c.innerHTML = '';
        _distLabels.forEach((lbl,i) => {{
            const p = document.createElement('div'); p.className='legend-pill active';
            p.style.cssText='transition:opacity 0.25s ease,transform 0.25s ease;cursor:pointer';
            p.innerHTML=`<div class="color-dot" style="background:${{_distColors[i]}}"></div><span>${{lbl}}</span>`;
            p.onclick=()=>{{ if(_selSlice===i) _donutReset(); else _donutFocus(i); }};
            c.appendChild(p);
        }});
    }})();
    createLegend(charts.growth, 'lgd-growth'); createLegend(charts.reach, 'lgd-reach'); createLegend(charts.momentum, 'lgd-momentum'); createLegend(charts.macd, 'lgd-macd');
    ['growth', 'reach', 'momentum', 'macd'].forEach(updateChart);
}}
function switchTab(name) {{
  document.querySelectorAll('.tab-pane').forEach(p => p.classList.remove('active'));
  document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
  document.getElementById('tab-' + name).classList.add('active');
  document.querySelector('[data-tab="' + name + '"]').classList.add('active');
}}
async function refreshData() {{ const b = document.getElementById('refreshBtn'); b.classList.add('loading'); await fetch('/api/data'); window.location.reload(); }}
function toggleTheme() {{
  document.body.classList.toggle('light');
  localStorage.setItem('tis-theme', document.body.classList.contains('light') ? 'light' : 'dark');
  const isL = document.body.classList.contains('light');
  const gC = isL ? 'rgba(0,0,0,0.08)' : 'rgba(255,255,255,0.05)';
  Chart.defaults.color = isL ? '#1f2328' : '#e6edf3';
  Object.values(charts).forEach(c => {{
    if (c.options.scales) {{
      if (c.options.scales.x) c.options.scales.x.ticks = {{...c.options.scales.x.ticks, color: Chart.defaults.color}};
      if (c.options.scales.y) {{ c.options.scales.y.grid.color = gC; c.options.scales.y.ticks = {{...c.options.scales.y.ticks, color: Chart.defaults.color}}; }}
      if (c.options.scales.y1) c.options.scales.y1.ticks = {{...c.options.scales.y1.ticks, color: Chart.defaults.color}};
    }}
    c.update('none');
  }});
}}
const latest_data_js = {json.dumps(latest_data)};
(function() {{ if (localStorage.getItem('tis-theme')==='light') document.body.classList.add('light'); initCharts(latest_data_js); }})();
</script></body></html>"""
    with open(HTML_FILE, "w") as f: f.write(html)

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/api/data':
            data = do_fetch(); self.send_response(200); self.send_header('Content-Type', 'application/json'); self.end_headers()
            self.wfile.write(json.dumps(data).encode())
        elif self.path == '/favicon.png':
            fav = os.path.join(DESKTOP, 'favicon.png')
            try:
                with open(fav, 'rb') as f: self.send_response(200); self.send_header('Content-Type', 'image/png'); self.end_headers(); self.wfile.write(f.read())
            except: self.send_error(404)
        else:
            try:
                with open(HTML_FILE, 'rb') as f: self.send_response(200); self.send_header('Content-Type', 'text/html'); self.end_headers(); self.wfile.write(f.read())
            except: self.send_error(404)
    def log_message(self, *args): pass

if __name__ == '__main__':
    do_fetch()
    if '--serve' in sys.argv:
        threading.Thread(target=lambda: (time.sleep(REFRESH_INTERVAL), do_fetch()), daemon=True).start()
        def _open(): time.sleep(1.2); webbrowser.open(f'http://localhost:{PORT}')
        threading.Thread(target=_open, daemon=True).start()
        try: HTTPServer(('localhost', PORT), Handler).serve_forever()
        except KeyboardInterrupt: pass
