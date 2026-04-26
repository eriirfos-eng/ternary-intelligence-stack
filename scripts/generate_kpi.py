#!/usr/bin/env python3
"""
TIS KPI Dashboard - Public Generator (v1.1)
===========================================
- CI/CD Optimized version for GitHub Actions.
- Static generation (no server).
"""

import json, os, sys, time, io, urllib.request, urllib.error, webbrowser, threading, re
from datetime import datetime, timezone
from http.server import HTTPServer, BaseHTTPRequestHandler

# -- Config -------------------------------------------------------------------
GITHUB_PAT       = os.environ.get("KPI_GITHUB_TOKEN")
GITHUB_REPO      = "eriirfos-eng/ternary-intelligence-stack"
CRATES_USER      = 403632
BASE_DIR         = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT_DIR       = os.path.join(BASE_DIR, "docs/kpi")
LOG_FILE         = os.path.join(OUTPUT_DIR, "ternlang_kpi_log.json")
TRAFFIC_LOG_FILE = os.path.join(OUTPUT_DIR, "ternlang_gh_traffic.json")
HTML_FILE        = os.path.join(OUTPUT_DIR, "index.html")
PORT             = 7329
REFRESH_INTERVAL = 60
OPENVSX_EXTS     = ["ternlang"]
FOOTPRINT_WEIGHTS = {
    "views": 1.0,
    "clones": 2.0,
    "crates": 4.0,
    "openvsx": 5.0,
    "smithery": 0.5
}

latest_data = {}

# -- Helpers ------------------------------------------------------------------
def fetch(url, headers=None):
    req = urllib.request.Request(url, headers=headers or {})
    try:
        with urllib.request.urlopen(req, timeout=15) as r: return json.loads(r.read())
    except: return None

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

def merge_gh_traffic(clone_series, views_series):
    log = {}
    if os.path.exists(TRAFFIC_LOG_FILE):
        try:
            with open(TRAFFIC_LOG_FILE) as f: log = json.load(f)
        except: pass
    for row in clone_series:
        day = row["timestamp"][:10]
        log.setdefault(day, {})["clones"] = row["count"]
        log[day]["clones_unique"] = row.get("uniques", 0)
    for row in views_series:
        day = row["timestamp"][:10]
        log.setdefault(day, {})["views"] = row["count"]
        log[day]["views_unique"] = row.get("uniques", 0)
    with open(TRAFFIC_LOG_FILE, "w") as f: json.dump(log, f, indent=2)
    return dict(sorted(log.items()))

# -- Core Logic ---------------------------------------------------------------
def do_fetch():
    global latest_data
    print(f"[{datetime.now().strftime('%H:%M')}] TIS Signal Sync...")

    cd = fetch(f"https://crates.io/api/v1/crates?per_page=100&user_id={CRATES_USER}", {"User-Agent": "ternlang-dash"}) or {}
    crates_list = []
    for c in cd.get("crates", []):
        crates_list.append({
            "name": c["name"],
            "downloads": c["downloads"],
            "recent": c.get("recent_downloads", 0),
            "version": c["max_version"],
            "updated": c["updated_at"]
        })
    crates_total = sum(c["downloads"] for c in crates_list)
    openvsx_total = sum((fetch(f"https://open-vsx.org/api/rfi-irfos/{ext}") or {}).get("downloadCount", 0) for ext in OPENVSX_EXTS)
    repo = gh(f"repos/{GITHUB_REPO}") or {}
    gh_stars = repo.get("stargazers_count", 0)
    clones_data = gh(f"repos/{GITHUB_REPO}/traffic/clones") or {}
    views_data = gh(f"repos/{GITHUB_REPO}/traffic/views") or {}
    all_traffic = merge_gh_traffic(clones_data.get("clones", []) if clones_data else [], views_data.get("views", []) if views_data else [])
    
    total_clones = sum(v.get("clones", 0) for v in all_traffic.values())
    total_views = sum(v.get("views", 0) for v in all_traffic.values())
    
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
    
    raw_footprint = crates_total + openvsx_total + gh_stars + total_clones + total_views + sm_val
    total_footprint = int(
        (total_views * FOOTPRINT_WEIGHTS["views"]) +
        (total_clones * FOOTPRINT_WEIGHTS["clones"]) +
        (crates_total * FOOTPRINT_WEIGHTS["crates"]) +
        (openvsx_total * FOOTPRINT_WEIGHTS["openvsx"]) +
        (sm_val * FOOTPRINT_WEIGHTS["smithery"])
    )

    log = []
    if os.path.exists(LOG_FILE):
        try:
            with open(LOG_FILE) as f: log = json.load(f)
        except: pass
    
    snapshot = {
        "ts": now_utc, "total_footprint": total_footprint,
        "raw_footprint": raw_footprint,
        "crates_total": crates_total, "openvsx_total": openvsx_total,
        "gh_stars": gh_stars, "gh_clones": total_clones,
        "gh_views": total_views,
        "smithery_calls_weekly": smithery_calls, "api_latency_ms": api_lat
    }

    log = [e for e in log if e.get("ts", "")[:10] != now_utc[:10]]
    log.append(snapshot); log = log[-180:]
    with open(LOG_FILE, "w") as f: json.dump(log, f, indent=2)

    chart_log = []
    for e in log:
        entry = dict(e)
        if "gh_clones" not in entry and "gh_clones_14d" in entry: entry["gh_clones"] = entry["gh_clones_14d"]
        if "gh_views" not in entry and "gh_views_14d" in entry: entry["gh_views"] = entry["gh_views_14d"]
        chart_log.append(entry)

    if chart_log and chart_log[0]["ts"].startswith("2026-04-17"):
        chart_log.insert(0, { "ts": "2026-04-16T00:00:00Z", "total_footprint": 0, "crates_total":0, "openvsx_total":0, "gh_stars":0, "gh_clones":0, "gh_views":0, "smithery_calls_weekly":"0" })

    latest_data = {
        "snapshot": snapshot,
        "prev": log[-2] if len(log) >= 2 else snapshot,
        "chart_log": chart_log,
        "momentum": compute_momentum(chart_log),
        "macd": compute_macd(chart_log),
        "crates_list": crates_list,
        "all_traffic": all_traffic,
        "updated_at": datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    }
    render_html()
    return latest_data

def compute_momentum(log):
    m = {"views": [], "clones": [], "crates": [], "openvsx": [], "smithery": []}
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
    return m

def compute_macd(log):
    values = [_to_num(e.get("total_footprint", 0)) for e in log]
    if not values: return {"macd": [], "signal": [], "hist": []}
    
    def ema(data, period):
        k = 2 / (period + 1)
        res = []
        curr = data[0]
        for v in data:
            curr = (v - curr) * k + curr
            res.append(curr)
        return res
        
    ema7 = ema(values, 7)
    ema14 = ema(values, 14)
    macd_line = [f - s for f, s in zip(ema7, ema14)]
    signal_line = ema(macd_line, 5)
    hist = [m - s for m, s in zip(macd_line, signal_line)]
    
    return {"macd": macd_line, "signal": signal_line, "hist": hist}

def render_html():
    d = latest_data
    s = d["snapshot"]
    p = d["prev"]
    
    # -- Signal Bar Logic --
    macd_hist = d["macd"]["hist"]
    macd_val = macd_hist[-1] if macd_hist else 0
    sentiment = "BULLISH" if macd_val > 0 else ("BEARISH" if macd_val < 0 else "STABLE")
    sent_cls = "sig-bullish" if sentiment == "BULLISH" else ("sig-bearish" if sentiment == "BEARISH" else "")
    
    recent_logs = d["chart_log"][-7:]
    if len(recent_logs) >= 2:
        v_start = _to_num(recent_logs[0].get("total_footprint", 0))
        v_end = _to_num(recent_logs[-1].get("total_footprint", 0))
        daily_v = (v_end - v_start) / max(1, len(recent_logs)-1)
    else:
        daily_v = 0
    
    target = 100000
    curr_f = _to_num(s["total_footprint"])
    eta_days = round((target - curr_f) / daily_v) if daily_v > 0 and curr_f < target else "N/A"
    
    m = d["momentum"]
    drivers = { "Clones": m["clones"][-1], "Views": m["views"][-1], "Crates": m["crates"][-1], "OpenVSX": m["openvsx"][-1] }
    primary_driver = max(drivers, key=drivers.get) if any(drivers.values()) else "None"
    
    signal_bar_html = f"""
    <div class="signal-bar">
      <div class="sig-item">SENTIMENT: <span class="{sent_cls}">{sentiment}</span></div>
      <div class="sig-item">TARGET 100K: <span>{eta_days} DAYS</span></div>
      <div class="sig-item">PRIMARY DRIVER: <span>{primary_driver}</span></div>
      <div class="sig-item" style="margin-left:auto">SIGNAL STRENGTH: <span>{abs(macd_val):.2f}</span></div>
    </div>"""
    
    def trend_box(curr, key, id_pfx):
        c, b = _to_num(curr), _to_num(p.get(key, 0))
        diff = c - b
        pct = (diff / b * 100) if b else 0
        color = "var(--green)" if diff > 0 else ("var(--red)" if diff < 0 else "var(--muted)")
        arrow = "▲" if diff > 0 else ("▼" if diff < 0 else "•")
        return f'<div class="card-trend" id="{id_pfx}-trend" style="color:{color}"><div class="trend-top">{arrow} {pct:+.1f}%</div><div class="trend-bottom">({diff:+g})</div></div>'

    kpis = [
        ("Total Footprint", s["total_footprint"], "total_footprint", "total", "c-purple", True),
        ("Total Clones", s["gh_clones"], "gh_clones", "clones", "", True),
        ("Total Views", s["gh_views"], "gh_views", "views", "", True),
        ("GitHub Stars", s["gh_stars"], "gh_stars", "stars", "", False),
        ("Smithery Calls", s["smithery_calls_weekly"], "smithery_calls_weekly", "smithery", "", False),
        ("Crates Downloads", s["crates_total"], "crates_total", "crates", "", True),
        ("Open VSX", s["openvsx_total"], "openvsx_total", "openvsx", "", True),
    ]
    
    cards_html = ""
    for label, val, key, id_pfx, cls, is_num in kpis:
        val_str = f"{val:,}" if is_num else str(val)
        cards_html += f"""
        <div class="kpi-card {cls}">
          <div class="card-left">
            <div class="label">{label}</div>
            <div class="value" id="val-{id_pfx}">{val_str}</div>
          </div>
          {trend_box(val, key, id_pfx)}
        </div>"""

    # -- Impact Score Mapping --
    f_map = { e["ts"][:10]: _to_num(e.get("total_footprint", 0)) for e in d["chart_log"] }
    def get_impact(day, clones, views):
        curr_f = f_map.get(day, 0)
        sorted_days = sorted(f_map.keys())
        try:
            idx = sorted_days.index(day)
            prev_f = f_map[sorted_days[idx-1]] if idx > 0 else curr_f
        except: prev_f = curr_f
        delta = curr_f - prev_f
        traffic = clones + views
        if delta > 50: return 1
        if delta <= 0 and traffic > 100: return -1
        return 0

    rows = []
    for day, v in sorted(d["all_traffic"].items(), reverse=True):
        c, vw = v.get("clones", 0), v.get("views", 0)
        iv = get_impact(day, c, vw)
        icls = "impact-pos" if iv > 0 else ("impact-neg" if iv < 0 else "impact-neu")
        badge = f'<span class="impact-badge {icls}">{iv:+d}</span>'
        rows.append(f'<tr><td>{day}</td><td class="num">{c} <span style="font-size:9px;color:var(--muted)">({v.get("clones_unique",0)})</span></td><td class="num">{vw} <span style="font-size:9px;color:var(--muted)">({v.get("views_unique",0)})</span></td><td>{badge}</td></tr>')
    traffic_rows = "\n".join(rows)

    crates_rows = "\n".join(f'<tr><td>{c["name"]}</td><td class="num" style="color:var(--blue)">v{c["version"]}</td><td class="num">{c["downloads"]:,}</td><td class="num" style="color:var(--green)">+{c["recent"]:,}</td></tr>' for c in sorted(d["crates_list"], key=lambda x: -x["downloads"]))
    lat = s["api_latency_ms"]
    lat_txt, lat_color = (f"{lat}ms", "var(--green)") if lat and lat < 500 else (f"{lat}ms" if lat else "timeout", "var(--red)")

    html = f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8"><title>RFI-IRFOS KPI | {sentiment} {curr_f:,}</title>
<script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
<script src="https://cdn.jsdelivr.net/npm/chartjs-plugin-datalabels@2"></script>
<style>
  @import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;700;800&display=swap');
  :root {{ --bg:#0d1117; --bg2:#161b22; --bg3:#21262d; --border:#30363d; --text:#e6edf3; --muted:#ffffff; --green:#3fb950; --blue:#58a6ff; --purple:#a371f7; --orange:#d29922; --red:#f85149; }}
  body.light {{ background: linear-gradient(135deg, #f6f8fa 0%, #e9eff5 100%); --bg:#f6f8fa; --bg2:rgba(255,255,255,0.9); --bg3:#f0f2f4; --border:#d0d7de; --text:#1f2328; --muted:#000000; }}
  * {{ box-sizing:border-box; margin:0; padding:0; }}
  body {{ background: linear-gradient(135deg, #0d1117 0%, #0a192f 50%, #003a4d 100%); color:var(--text); font-family:'JetBrains Mono', monospace; margin:0; font-size:18px; transition: .2s; overflow-x:hidden; background-attachment: fixed; }}
  header {{ background:rgba(22, 27, 34, 0.8); border-bottom:2px solid var(--border); padding:20px 40px; display:flex; align-items:center; justify-content:space-between; position:sticky; top:0; z-index:100; backdrop-filter: blur(10px); }}
  body.light header {{ background:rgba(255, 255, 255, 0.8); border-bottom-color:rgba(0,0,0,0.05); }}
  .hdr-right {{ display:flex; gap:16px; }}
  .icon-btn {{ background:var(--bg3); border:1px solid var(--border); border-radius:12px; width:52px; height:52px; display:flex; align-items:center; justify-content:center; color:var(--text); cursor:pointer; transition:all 0.3s cubic-bezier(0.4, 0, 0.2, 1); }}
  .icon-btn:hover {{ background:var(--border); transform: translateY(-2px); box-shadow: 0 4px 12px rgba(0,0,0,0.3); }}
  .icon-btn svg {{ width:20px; height:20px; fill:currentColor; }}
  .btn-green {{ background:var(--green); border:none; color:#fff; box-shadow: 0 0 15px rgba(63, 185, 80, 0.2); }}
  .btn-green:hover {{ background:#46c95d; box-shadow: 0 0 20px rgba(63, 185, 80, 0.4); }}
  .btn-green.loading svg {{ animation: spin 1s linear infinite; }}
  @keyframes spin {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }}
  main {{ max-width:1650px; margin:0 auto; padding:24px; }}
  .kpi-grid {{ display:grid; grid-template-columns:repeat(4, 1fr); gap:20px; margin-bottom:32px; }}
  .kpi-card {{ background:rgba(22, 27, 34, 0.4); border:1px solid rgba(255,255,255,0.1); border-radius:16px; padding:32px; border-top:6px solid var(--border); display:flex; justify-content:space-between; align-items: flex-end; min-height:160px; backdrop-filter: blur(12px); box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.4); transition: all 0.3s ease; }}
  .kpi-card:hover {{ transform: translateY(-8px); box-shadow: 0 16px 48px 0 rgba(0, 0, 0, 0.6); }}
  body.light .kpi-card {{ background:rgba(255, 255, 255, 0.5); border-color:rgba(0,0,0,0.05); }}
  .card-left {{ flex: 1; }}
  .label {{ font-size:15px; text-transform:uppercase; color:var(--muted); font-weight:800; margin-bottom:16px; letter-spacing:1px; }}
  .value {{ font-size:42px; font-weight:800; line-height:1; letter-spacing:-1.5px; }}
  .card-trend {{ text-align:right; display:flex; flex-direction:column; justify-content:flex-end; font-weight:800; }}
  .trend-top {{ font-size:20px; margin-bottom:6px; white-space:nowrap; }}
  .trend-bottom {{ font-size:17px; opacity:1.0; white-space:nowrap; }}
  .row-top {{ display:grid; grid-template-columns: 2fr 1fr; gap:32px; margin-bottom:32px; align-items: stretch; }}
  .bottom-floor {{ display:grid; grid-template-columns: 1fr 1fr; gap:32px; height: 850px; max-width:1600px; align-items: stretch; }}
  .panel {{ background:rgba(22, 27, 34, 0.4); border:1px solid rgba(255,255,255,0.1); border-radius:16px; padding:32px; position:relative; display:flex; flex-direction:column; backdrop-filter: blur(12px); box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.5); }}
  body.light .panel {{ background:rgba(255, 255, 255, 0.5); border-color:rgba(0,0,0,0.05); }}
  .panel-h {{ font-size:16px; font-weight:800; color:var(--muted); text-transform:uppercase; margin-bottom:20px; border-bottom:1px solid var(--border); padding-bottom:12px; display:flex; justify-content:space-between; letter-spacing:1.5px; }}
  table {{ width:100%; border-collapse:collapse; font-size:17px; }}
  th {{ text-align:left; font-size:13px; color:var(--muted); padding:16px 20px; background:var(--bg3); text-transform:uppercase; font-weight:800; }}
  td {{ padding:16px 20px; border-bottom:1px solid var(--border); }}
  tr:hover {{ background:rgba(255,255,255,0.03); }}
  td.num {{ text-align:right; font-weight:700; font-variant-numeric:tabular-nums; }}
  .ch-up {{ color:var(--green); }}
  .ch-dn {{ color:var(--red); }}
  .ch-na {{ color:var(--muted); }}
  .signal-bar {{ background:rgba(0,0,0,0.3); border-bottom:1px solid var(--border); padding:12px 40px; display:flex; gap:32px; font-size:14px; font-weight:800; letter-spacing:1px; color:var(--muted); backdrop-filter: blur(10px); }}
  body.light .signal-bar {{ background:rgba(255,255,255,0.7); border-bottom-color:rgba(0,0,0,0.05); }}
  .sig-item span {{ color:var(--text); margin-left:8px; }}
  .sig-bullish {{ color:var(--green) !important; }}
  .sig-bearish {{ color:var(--red) !important; }}
  .legend-container {{ display:flex; flex-wrap:wrap; gap:12px; margin-bottom:16px; width:100%; }}
  .legend-pill {{ display:flex; align-items:center; gap:8px; padding:6px 14px; border-radius:50px; background:rgba(255,255,255,0.05); border:1px solid rgba(255,255,255,0.1); cursor:pointer; font-size:12px; font-weight:800; color:var(--muted); transition: all 0.2s ease; }}
  .legend-pill.active {{ background:rgba(255,255,255,0.15); border-color:var(--text); color:var(--text); }}
  .legend-pill .color-dot {{ width:10px; height:10px; border-radius:50%; }}
  body.light .legend-pill {{ background:rgba(0,0,0,0.03); border-color:rgba(0,0,0,0.1); }}
  .impact-badge {{ padding:4px 10px; border-radius:4px; font-weight:900; font-size:11px; letter-spacing:1px; }}
  .impact-pos {{ background:rgba(63,185,80,0.15); color:var(--green); border:1px solid var(--green); }}
  .impact-neu {{ background:rgba(255,255,255,0.05); color:var(--muted); border:1px solid var(--border); }}
  .impact-neg {{ background:rgba(248,81,73,0.15); color:var(--red); border:1px solid var(--red); }}
  #loading-bar {{ position:fixed; top:0; left:0; height:3px; background:var(--blue); width:0; transition:width 0.3s; z-index:200; }}
</style>
</head>
<body>
<div id="loading-bar"></div>
<header>
  <div><h1 style="margin:0;font-size:20px;letter-spacing:-0.5px;font-weight:800">RFI-IRFOS <span style="color:var(--blue)">KPI</span></h1><div style="font-size:11px;color:var(--muted);font-weight:700;letter-spacing:1px">STATUS: NOMINAL • SYNC: {d["updated_at"]}</div></div>
  <div class="hdr-right">
    <button class="icon-btn" onclick="toggleTheme()" id="themeBtn" title="Toggle Theme">
      <svg id="theme-icon" viewBox="0 0 24 24"><path d="M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1zM5.99 4.58c-.39-.39-1.03-.39-1.41 0s-.39 1.03 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41L5.99 4.58zm12.37 12.37c-.39-.39-1.03-.39-1.41 0s-.39 1.03 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41l-1.06-1.06zm1.06-12.37c-.39-.39-1.03-.39-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06c.39-.39.39-1.03 0-1.41zm-12.37 12.37c-.39-.39-1.03-.39-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06c.39-.39.39-1.03 0-1.41z"/></svg>
    </button>
    <button class="icon-btn btn-green" id="refreshBtn" onclick="refreshDashboardData()" title="Refresh Data">
      <svg viewBox="0 0 24 24"><path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
    </button>
  </div>
</header>
{signal_bar_html}
<main>
  <div class="kpi-grid">
    {cards_html}
    <div class="kpi-card" id="card-lat">
      <div class="card-left">
        <div class="label">Operational Latency</div>
        <div class="value" id="val-lat" style="color:{lat_color}">{lat_txt}</div>
      </div>
      <div class="card-trend" id="del-lat" style="color:var(--muted)">
        <div class="trend-top">✦ SIGNAL</div>
        <div class="trend-bottom">LIVE</div>
      </div>
    </div>
  </div>
  <div class="row-top">
    <div class="panel">
      <div class="panel-h">Footprint Pulse <span>Historical Density</span></div>
      <div id="lgd-growth" class="legend-container"></div>
      <div style="height:400px"><canvas id="growthChart"></canvas></div>
    </div>
    <div class="panel" style="display:flex; flex-direction:column; align-items:center;">
      <div class="panel-h" style="width:100%">Resource Distribution <span>Balanced Weights</span></div>
      <div id="lgd-dist" class="legend-container" style="justify-content:center"></div>
      <div style="height:400px; width:340px;"><canvas id="distChart"></canvas></div>
    </div>
  </div>
  <div class="row-top">
    <div class="panel">
      <div class="panel-h">Momentum Shift <span>Delta Tracking</span></div>
      <div id="lgd-momentum" class="legend-container"></div>
      <div style="height:400px"><canvas id="momentumChart"></canvas></div>
    </div>
    <div class="panel">
      <div class="panel-h">MACD Momentum Tracker <span>7-14-5 Trend Convergence</span></div>
      <div id="lgd-macd" class="legend-container"></div>
      <div style="height:400px; position:relative;"><canvas id="macdChart"></canvas></div>
    </div>
  </div>
  <div class="bottom-floor">
    <div style="display: flex; flex-direction: column; gap: 24px; height: 100%;">
      <div class="panel" style="padding:0; overflow:hidden;">
        <div class="panel-h" style="padding:15px;background:var(--bg2);z-index:10;margin:0">Recent Volatility <span>Delta Tracking</span></div>
        <table id="table-changes"><thead><tr><th>Metric</th><th class="num">Current</th><th class="num">Change</th><th class="num">Trend %</th></tr></thead><tbody>
          {render_changes_rows(s, p)}
        </tbody></table>
      </div>
      <div class="panel" style="flex: 1; min-height: 0; overflow-y: auto; padding: 0;">
        <div class="panel-h" style="padding:15px; position:sticky; top:0; background:var(--bg2); z-index:10; margin:0">Crates Detail <span>Granular Downloads</span></div>
        <table id="table-crates"><thead><tr><th>Crate Name</th><th class="num">Version</th><th class="num">Total DL</th><th class="num">Recent</th></tr></thead><tbody>{crates_rows}</tbody></table>
      </div>
    </div>
    <div class="panel" style="height: 100%; overflow-y: auto; padding: 0;">
      <div class="panel-h" style="padding:15px; position:sticky; top:0; background:var(--bg2); z-index:10; margin:0">Network Traffic <span>14-Day Rescued Log</span></div>
      <table id="table-traffic"><thead><tr><th>Timestamp</th><th class="num">Clones</th><th class="num">Views</th><th>TIS Impact</th></tr></thead><tbody>{traffic_rows}</tbody></table>
    </div>
  </div>
</main>
<script>
const P = {{ pur:'#a371f7', blu:'#58a6ff', grn:'#3fb950', org:'#d29922', red:'#f85149', teal:'#39d353' }};
let growthChart, distChart, momentumChart, macdChart;
function getThemeColor() {{ return getComputedStyle(document.documentElement).getPropertyValue('--text').trim(); }}
function createCustomLegend(chart, containerId) {{
    const container = document.getElementById(containerId);
    if (!container) return;
    container.innerHTML = '';
    chart.data.datasets.forEach((dataset, index) => {{
        const pill = document.createElement('div');
        pill.className = 'legend-pill' + (chart.isDatasetVisible(index) ? ' active' : '');
        pill.innerHTML = `<div class="color-dot" style="background:${{dataset.borderColor || dataset.backgroundColor}}"></div><span>${{dataset.label}}</span>`;
        pill.onclick = () => {{
            const visible = chart.isDatasetVisible(index);
            chart.setDatasetVisibility(index, !visible);
            chart.update();
            pill.classList.toggle('active');
        }};
        container.appendChild(pill);
    }});
}}
function initCharts(data) {{
    const L = data.chart_log; const s = data.snapshot;
    const textColor = getThemeColor(); const isLight = document.body.classList.contains('light');
    const gridColor = isLight ? 'rgba(0,0,0,0.08)' : 'rgba(255,255,255,0.05)';
    Chart.defaults.color = textColor; Chart.defaults.font.family = "'JetBrains Mono', monospace";
    Chart.register(ChartDataLabels);
    const sm_val = (val) => {{
        const str = String(val).replace(/[▲▼,]/g, '').trim();
        return str.toLowerCase().endsWith('k') ? parseFloat(str)*1000 : (parseFloat(str)||0);
    }};
    if(growthChart) growthChart.destroy(); if(distChart) distChart.destroy();
    if(momentumChart) momentumChart.destroy(); if(macdChart) macdChart.destroy();
    growthChart = new Chart(document.getElementById('growthChart').getContext('2d'), {{
      type: 'line',
      data: {{
        labels: L.map(e=>e.ts.slice(5,10)),
        datasets: [
          {{ label:'Footprint', data:L.map(e=>e.total_footprint), borderColor:P.pur, fill:false, tension:0.4, borderWidth:4, pointRadius:4, pointBackgroundColor:P.pur }},
          {{ label:'Crates', data:L.map(e=>e.crates_total), borderColor:P.grn, tension:0.4, pointRadius:0, borderWidth:2, borderDash:[5,5] }},
          {{ label:'OpenVSX', data:L.map(e=>e.openvsx_total), borderColor:P.blu, tension:0.4, pointRadius:0, borderWidth:2, borderDash:[5,5] }},
          {{ label:'Total Clones', data:L.map(e=>e.gh_clones), borderColor:P.teal, tension:0.4, pointRadius:0, borderWidth:2 }},
          {{ label:'Views', data:L.map(e=>e.gh_views), borderColor:P.org, tension:0.4, pointRadius:0, borderWidth:2 }},
          {{ label:'Smithery', data:L.map(e=>sm_val(e.smithery_calls_weekly)), borderColor:P.red, tension:0.4, pointRadius:0, borderWidth:2 }}
        ]
      }},
      options: {{ responsive:true, maintainAspectRatio:false, interaction: {{ mode: 'index', intersect: false }},
        plugins: {{ datalabels: {{ display: false }}, legend: {{ display: false }}, tooltip: {{ backgroundColor: isLight ? 'rgba(255, 255, 255, 0.95)' : 'rgba(22, 27, 34, 0.9)', titleColor: isLight ? '#1f2328' : '#e6edf3', bodyColor: isLight ? '#1f2328' : '#e6edf3', borderColor: 'var(--border)', borderWidth: 1 }} }},
        scales:{{ x:{{ grid:{{ display:false }}, ticks:{{ font:{{ size: 12 }} }} }}, y:{{ grid:{{ color: gridColor }}, ticks:{{ font:{{ size: 12 }}, callback: v => v >= 1000 ? (v/1000).toFixed(1)+'k' : v }} }} }}
      }}
    }});
    createCustomLegend(growthChart, 'lgd-growth');
    distChart = new Chart(document.getElementById('distChart'), {{
      type: 'doughnut',
      data: {{
        labels: ['Crates', 'OpenVSX', 'Total Clones', 'Views', 'Smithery', 'Stars'],
        datasets: [{{ 
            data: [s.crates_total, s.openvsx_total, s.gh_clones, s.gh_views, sm_val(s.smithery_calls_weekly), s.gh_stars], 
            backgroundColor: [P.grn, P.blu, P.pur, P.org, P.red, P.teal], 
            borderWidth: 0, 
            hoverOffset: 15, 
            borderRadius: 10, 
            spacing: 5,
            offset: [0,0,0,0,0,0]
        }}]
      }},
      options: {{ 
        cutout:'85%', 
        maintainAspectRatio: false,
        layout: {{ padding: 45 }},
        plugins:{{ 
            datalabels: {{
                display: (ctx) => ctx.dataset.offset[ctx.dataIndex] > 0,
                color: '#000000',
                backgroundColor: 'rgba(255, 255, 255, 0.95)',
                borderColor: 'rgba(0, 0, 0, 0.1)',
                borderWidth: 1,
                borderRadius: 8,
                padding: 10,
                font: {{ weight: '800', size: 13 }},
                formatter: (val, ctx) => {{
                    let total = ctx.dataset.data.reduce((a, b) => a + b, 0);
                    let pct = (val / total * 100).toFixed(1);
                    return `${{val.toLocaleString()}}\n(${{pct}}%)`;
                }},
                textAlign: 'center',
                anchor: 'end',
                align: 'end',
                offset: 15
            }},
            legend:{{ display: false }}, 
            tooltip: {{ 
                backgroundColor: isLight ? 'rgba(255, 255, 255, 0.95)' : 'rgba(22, 27, 34, 0.9)', 
                titleColor: isLight ? '#1f2328' : '#e6edf3', 
                bodyColor: isLight ? '#1f2328' : '#e6edf3', 
                borderWidth: 1, borderColor: 'var(--border)',
                callbacks: {{
                    label: function(item) {{
                        let total = item.dataset.data.reduce((a, b) => a + b, 0);
                        let val = item.raw;
                        let pct = (val / total * 100).toFixed(1);
                        return `${{item.label}}: ${{val.toLocaleString()}} (${{pct}}%)`;
                    }}
                }}
            }} 
        }} 
      }}
    }});
    const dLgd = document.getElementById('lgd-dist');
    if (dLgd) {{ 
        dLgd.innerHTML = ''; 
        distChart.data.labels.forEach((lbl, i) => {{ 
            const pill = document.createElement('div'); 
            pill.className = 'legend-pill'; 
            pill.innerHTML = `<div class="color-dot" style="background:${{distChart.data.datasets[0].backgroundColor[i]}}"></div><span>${{lbl}}</span>`; 
            pill.onclick = () => {{
                const ds = distChart.data.datasets[0];
                const isH = ds.offset[i] === 25;
                ds.offset[i] = isH ? 0 : 25;
                pill.classList.toggle('active', !isH);
                distChart.update();
            }};
            dLgd.appendChild(pill); 
        }}); 
    }}
    if (data.momentum) {{
        momentumChart = new Chart(document.getElementById('momentumChart'), {{
          type: 'line',
          data: {{
            labels: L.map(e=>e.ts.slice(5,10)),
            datasets: [
              {{ label:'Views Δ', data:data.momentum.views, borderColor:P.org, tension:0.3, pointRadius:4, pointHoverRadius:6, borderWidth:3 }},
              {{ label:'Clones Δ', data:data.momentum.clones, borderColor:P.teal, tension:0.3, pointRadius:4, pointHoverRadius:6, borderWidth:3 }},
              {{ label:'Crates Δ', data:data.momentum.crates, borderColor:P.grn, tension:0.3, pointRadius:4, pointHoverRadius:6, borderWidth:3, borderDash:[5,5] }},
              {{ label:'OpenVSX Δ', data:data.momentum.openvsx, borderColor:P.blu, tension:0.3, pointRadius:4, pointHoverRadius:6, borderWidth:3, borderDash:[5,5] }},
              {{ label:'Smithery Δ', data:data.momentum.smithery, borderColor:P.red, tension:0.3, pointRadius:4, pointHoverRadius:6, borderWidth:3 }}
            ]
          }},
          options: {{ responsive:true, maintainAspectRatio:false, interaction: {{ mode: 'index', intersect: false }}, plugins: {{ datalabels: {{ display: false }}, legend: {{ display: false }} }},
            scales: {{ x:{{ grid:{{display:false}}, ticks:{{font:{{size:12}}}} }}, y:{{ grid:{{color:gridColor}}, ticks:{{font:{{size:12}}}} }} }}
          }}
        }});
        createCustomLegend(momentumChart, 'lgd-momentum');
    }}
    if (data.macd) {{
        macdChart = new Chart(document.getElementById('macdChart'), {{
          type: 'line',
          data: {{
            labels: L.map(e=>e.ts.slice(5,10)),
            datasets: [
              {{ label:'MACD (7,14)', data:data.macd.macd, borderColor:P.blu, tension:0.4, pointRadius:4, pointHoverRadius:6, borderWidth:3 }},
              {{ label:'Signal (5)', data:data.macd.signal, borderColor:P.org, tension:0.4, pointRadius:4, pointHoverRadius:6, borderWidth:3, borderDash:[5,5] }},
              {{ label:'Histogram', data:data.macd.hist, type:'bar', backgroundColor: data.macd.hist.map(v => v >= 0 ? P.grn+'88' : P.red+'88'), borderWidth:0 }}
            ]
          }},
          options: {{ responsive:true, maintainAspectRatio:false, interaction: {{ mode: 'index', intersect: false }}, plugins: {{ datalabels: {{ display: false }}, legend: {{ display: false }} }},
            scales: {{ x:{{ grid:{{display:false}}, ticks:{{font:{{size:12}}}} }}, y:{{ grid:{{color:gridColor}}, ticks:{{font:{{size:12}}}} }} }}
          }}
        }});
        createCustomLegend(macdChart, 'lgd-macd');
    }}
}}
async function refreshDashboardData() {{
    const btn = document.getElementById('refreshBtn'); const bar = document.getElementById('loading-bar');
    btn.classList.add('loading'); btn.disabled = true; bar.style.width = "40%";
    try {{ const res = await fetch('/api/data'); const data = await res.json(); window.location.reload(); }}
    catch(e) {{ btn.classList.remove('loading'); btn.style.background = "var(--red)"; }}
}}
function toggleTheme() {{
    document.body.classList.toggle('light'); const isLight = document.body.classList.contains('light');
    localStorage.setItem('tis-theme', isLight ? 'light' : 'dark');
    const icon = document.getElementById('theme-icon');
    if (isLight) icon.innerHTML = '<path d="M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9 9-4.03 9-9c0-.46-.04-.92-.1-1.36-.98 1.37-2.58 2.26-4.4 2.26-3.03 0-5.5-2.47-5.5-5.5 0-1.82.89-3.42 2.26-4.4-.44-.06-.9-.1-1.36-.1z"/>';
    else icon.innerHTML = '<path d="M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1zM5.99 4.58c-.39-.39-1.03-.39-1.41 0s-.39 1.03 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41L5.99 4.58zm12.37 12.37c-.39-.39-1.03-.39-1.41 0s-.39 1.03 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41l-1.06-1.06zm1.06-12.37c-.39-.39-1.03-.39-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06c.39-.39.39-1.03 0-1.41zm-12.37 12.37c-.39-.39-1.03-.39-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06c.39-.39.39-1.03 0-1.41z"/>';
    initCharts(latest_data_js);
}}
const latest_data_js = {json.dumps(d)};
(function() {{
    const saved = localStorage.getItem('tis-theme') || 'dark';
    if (saved === 'light') {{ document.body.classList.add('light'); document.getElementById('theme-icon').innerHTML = '<path d="M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9 9-4.03 9-9c0-.46-.04-.92-.1-1.36-.98 1.37-2.58 2.26-4.4 2.26-3.03 0-5.5-2.47-5.5-5.5 0-1.82.89-3.42 2.26-4.4-.44-.06-.9-.1-1.36-.1z"/>'; }}
    initCharts(latest_data_js);
}})();
</script>
</body></html>"""
    with open(HTML_FILE, "w") as f: f.write(html)

def render_changes_rows(s, p):
    def _to_num(v):
        if v is None: return 0
        s = str(v).replace('▲','').replace('▼','').replace(',','').strip()
        if 'k' in s.lower(): return int(float(s.lower().replace('k','')) * 1000)
        if 'm' in s.lower(): return int(float(s.lower().replace('m','')) * 1000000)
        try: return float(s)
        except: return 0
        
    def _row(curr, prev, label):
        c, pr = _to_num(curr), _to_num(prev)
        diff, pct = c - pr, (c-pr)/pr*100 if pr else 0
        cls = "ch-up" if diff > 0 else ("ch-dn" if diff < 0 else "ch-na")
        val_str = f"{curr:,}" if isinstance(curr, (int, float)) else str(curr)
        
        # Signal Quality Logic
        status = "STABLE"
        if pct > 5: status = "SURGE"
        if pct < -5: status = "DROP"
        if diff == 0: status = "FLAT"
        
        return f'<tr><td>{label} <span style="font-size:9px;opacity:0.6">[{status}]</span></td><td class="num">{val_str}</td><td class="num {cls}">{diff:+g}</td><td class="num {cls}">{pct:+.1f}%</td></tr>'

    return "\n".join([
        _row(s["total_footprint"], p.get("total_footprint"), "Total Footprint"),
        _row(s["crates_total"], p.get("crates_total"), "Crates.io Downloads"),
        _row(s["openvsx_total"], p.get("openvsx_total"), "OpenVSX Installs"),
        _row(s["gh_clones"], p.get("gh_clones"), "Total GitHub Clones"),
        _row(s["smithery_calls_weekly"], p.get("smithery_calls_weekly"), "Smithery Calls")
    ])

if __name__ == '__main__':
    do_fetch()
