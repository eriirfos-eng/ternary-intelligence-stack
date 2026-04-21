import init, { run_tern, check_tern } from '/playground/pkg/ternlang_wasm.js';

init().then(() => {
  window.wasmRunTern   = run_tern;
  window.wasmCheckTern = check_tern;
  window.wasmReady     = true;
  window.dispatchEvent(new Event('wasmready'));
}).catch(() => { window.wasmReady = false; });

// ─── Pyodide (Python WASM) Initialization ─────────────────────────────────────
let pyodideInstance = null;
async function getPyodide() {
  if (pyodideInstance) return pyodideInstance;
  try {
    const sb = document.getElementById('sbWasmStatus');
    if (sb) sb.textContent = "Pyodide loading…";
    pyodideInstance = await loadPyodide();
    if (sb) sb.textContent = "TernVM + Pyodide Ready";
    return pyodideInstance;
  } catch (e) {
    console.error("Pyodide Load Error:", e);
    return null;
  }
}

async function runPythonActuator(code) {
  const py = await getPyodide();
  if (!py) return { ok: false, error: "Pyodide not available" };

  let stdout = "";
  py.setStdout({ batched: (str) => { stdout += str + "\n"; } });
  py.setStderr({ batched: (str) => { stdout += "ERR: " + str + "\n"; } });

  try {
    await py.runPythonAsync(code);
    return { ok: true, output: stdout.trim() };
  } catch (e) {
    return { ok: false, error: e.message, traceback: String(e) };
  }
}
window.runPythonActuator = runPythonActuator;

if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1') {
  const devScript = document.createElement('script');
  devScript.src = '.ternstudio-local.js';
  devScript.onerror = () => console.warn('Dev script missing (expected in local dev)');
  document.head.appendChild(devScript);
}
// ─── Embedded stdlib files ────────────────────────────────────────────────────
const STDLIB = {};
const TREE = [];
const TIER_CLASS = { 1: "t1", 2: "t2", 3: "t3", 4: "t4" };
const TIER_LABELS = { 1: "Tier 1 — Free", 2: "Tier 2 — Pro", 3: "Tier 3 — Industrial", 4: "Tier 4 — Enterprise" };
const TIER_BADGE_CLASS = { 1: "badge-free", 2: "badge-t2", 3: "badge-t3", 4: "badge-t4" };

// ─── Templates ────────────────────────────────────────────────────────────────
const TEMPLATES = {
  hello: `fn main() -> trit {\n    print("⊕ Hello Ternary World");\n    return affirm;\n}\n`,
  consensus: `// Ternary consensus: majority of N agents
fn main() -> trit {
    let a: trit = affirm;
    let b: trit = tend;
    let c: trit = affirm;
    // 2x affirm beats 1x tend → affirm
    print("⊕ Consensus: affirm");
    return affirm;
}`,
  gate: `// Ternary logic gate — routes on signal value
fn gate(signal: trit) -> trit {
    match signal {
        affirm => { print("● PASS — forwarding"); return affirm; }
        tend   => { print("○ HOLD — deliberating"); return tend; }
        reject => { print("✕ BLOCK — rejected"); return reject; }
    }
}

fn main() -> trit {
    return gate(affirm);
}`,
  agent: `// Basic agent pattern
agent Sensor {
    handle(s: trit) {
        match s {
            affirm => { spawn Actuator(affirm); }
            tend   => { spawn Deliberator(tend); }
            reject => { return; }
        }
    }
}

fn main() -> trit {
    print("⊕ Agent spawned");
    return affirm;
}`,
  match: `fn classify(x: int) -> trit {
    if x > 0 { return truth();    }
    if x < 0 { return conflict(); }
    return hold();
}

let r: trit = classify(5);

match r {
    -1 => { return conflict(); }
     0 => { return hold();    }
     1 => { return truth();   }
}`,
  ema: `fn ema_gate(prior: float, evidence: float, alpha: float) -> trit {
    let smoothed: float = alpha * evidence + (1.0 - alpha) * prior;
    if smoothed > 0.75 { return truth();    }
    if smoothed < 0.25 { return conflict(); }
    return hold();
}

let result: trit = ema_gate(0.5, 0.8, 0.3);

match result {
    -1 => { return conflict(); }
     0 => { return hold();    }
     1 => { return truth();   }
}`,
};

// ─── Auto-detect API endpoint (same-origin when served from the API server) ──
(function() {
  const loc = window.location;
  if (loc.hostname !== 'localhost' && loc.hostname !== '127.0.0.1' && loc.protocol !== 'file:') {
    const origin = loc.protocol + '//' + loc.hostname + (loc.port ? ':' + loc.port : '');
    document.getElementById('apiEndpoint').value = origin;
  }
})();

// ─── App state ────────────────────────────────────────────────────────────────
let monacoEditor = null;
let activeFile = localStorage.getItem("ternstudio-active-file") || "examples/hello_trit.tern";
let tabs = JSON.parse(localStorage.getItem("ternstudio-tabs") || JSON.stringify([{ name: "hello_trit.tern", path: "examples/hello_trit.tern" }]));
let fileBuffers = JSON.parse(localStorage.getItem("ternstudio-file-buffers") || JSON.stringify({ "examples/hello_trit.tern": TEMPLATES.hello }));

let simSpeed = 200; // Efficient industrial baseline
function updateSimSpeed(val) {
  const sliderVal = parseInt(val);
  // Aggressive Scale: 0 is slow-ish (500ms), 1000 is instant (0ms)
  // Mapping: 1000 -> 0ms, 0 -> 1000ms. But we want aggressive, so maybe exponential.
  // Linear for now but with 0 delay at max.
  simSpeed = Math.max(0, 1000 - sliderVal);
  localStorage.setItem("ternflow_sim_speed", sliderVal);
  const slider = document.getElementById("simSpeedSlider");
  if (slider) slider.value = sliderVal;
}
window.updateSimSpeed = updateSimSpeed;

function hydrateSimSpeed() {
  const saved = localStorage.getItem("ternflow_sim_speed");
  if (saved !== null) {
    updateSimSpeed(saved);
  } else {
    updateSimSpeed(800); // Default to fast
  }
}

function saveEditorState() {
  if (monacoEditor) fileBuffers[activeFile] = monacoEditor.getValue();
  localStorage.setItem("ternstudio-active-file", activeFile);
  localStorage.setItem("ternstudio-tabs", JSON.stringify(tabs));
  localStorage.setItem("ternstudio-file-buffers", JSON.stringify(fileBuffers));
}
window.saveEditorState = saveEditorState;
let scratchCounter = 1;
let runHistory = [];
let sessionRuns = 0, sessionOk = 0, sessionErr = 0;

// ─── View switching ───────────────────────────────────────────────────────────
// ─── API Key management ──────────────────────────────────────────────────────
function toggleKeyInput() {
  const area = document.getElementById("keyToggleArea");
  const acts = document.getElementById("topbarActions");
  const isHidden = area.style.display === "none";
  area.style.display = isHidden ? "flex" : "none";
  acts.style.display = isHidden ? "none" : "flex";
  if (isHidden) {
    const input = document.getElementById("topbarKeyInput");
    input.value = document.getElementById("apiKey").value;
    setTimeout(() => input.focus(), 10);
  }
}
window.toggleKeyInput = toggleKeyInput;

function updateApiKey(val) {
  val = val.trim();
  document.getElementById("apiKey").value = val;
  localStorage.setItem("ternstudio-key", val);
  fetchUsage();
  if (val) {
    showToast("API Key updated", "ok");
    toggleKeyInput();
    loadPremiumTree(); // Fetch premium content when key is entered
  }
}
window.updateApiKey = updateApiKey;

function toggleStdlibVisibility(show) {
  const container = document.getElementById('stdlib-tree-container');
  if (container) {
    container.style.display = show ? 'block' : 'none';
  }
}
window.toggleStdlibVisibility = toggleStdlibVisibility;

async function loadPremiumTree() {
  const key = document.getElementById("apiKey").value.trim();
  const container = document.getElementById('premium-tree-container');
  const treeEl = document.getElementById("premium-file-tree");

  if (!key || !container || !treeEl) {
    if (container) container.style.display = 'none';
    return;
  }
  
  container.style.display = 'block';
  treeEl.innerHTML = '<div class="tree-file" style="color:var(--muted)">Syncing premium assets...</div>';

  try {
    const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
    const r = await fetch(`${endpoint}/api/premium/list`, {
      headers: { 'X-Ternlang-Key': localStorage.getItem('ternstudio-key') || '' }
    });

    if (!r.ok) {
      const errTextRaw = await r.text();
      console.error(`Premium fetch failed | Status: ${r.status} | URL: ${r.url} | Raw Response:`, errTextRaw);
      const errorText = r.status === 403 ? "Auth Failed. Invalid Key." : `HTTP Error ${r.status}`;
      treeEl.innerHTML = `<div class="tree-file" style="color:var(--red)">${errorText}</div>`;
      return;
    }

    const d = await r.json();

    if (d.status === "ok" && Array.isArray(d.files)) {
      renderFileTree(treeEl, d.files, false, true); // container, files, isGithub, isPremium
    } else {
      treeEl.innerHTML = `<div class="tree-file" style="color:var(--red)">Error: ${d.error || 'Failed to load structure'}</div>`;
    }
  } catch (e) {
    treeEl.innerHTML = '<div class="tree-file" style="color:var(--red)">Connection to API failed.</div>';
    console.error("Premium fetch error:", e);
  }
}
window.loadPremiumTree = loadPremiumTree;


async function switchView(name) {
  localStorage.setItem("ternstudio-last-view", name);
  document.querySelectorAll(".view").forEach(v => v.classList.remove("active"));
  document.querySelectorAll(".nav-tab").forEach(t => t.classList.remove("active"));
  
  const viewEl = document.getElementById("view-" + name);
  const tabEl = document.getElementById("vt-" + name);
  const configView = document.getElementById("config-view");
  
  if (viewEl) viewEl.classList.add("active");
  if (tabEl) tabEl.classList.add("active");
  
  if (configView) {
    if (name === "settings") {
      configView.style.display = "flex";
    } else {
      configView.style.display = "none";
    }
  }

  if (name === "editor" && monacoEditor) {
    setTimeout(() => monacoEditor.layout(), 50);
  }
  if (name === "flow") {
    renderFlow();
    renderFlowLibrary();
  }
  if (name === "debugger") renderTracerView();
  if (name === "modules") await renderRegistryView();
  if (name === "fleet") await renderFleetView();
  if (name === "settings") syncSettingsUI();
  lucide.createIcons();
}
window.switchView = switchView;

// ─── Tracer & Registry Views ──────────────────────────────────────────────────

function renderTracerView() {
  const view = document.getElementById("view-debugger");
  if (!view) return;
  
  let html = `
    <div style="padding: 40px; overflow-y: auto; align-items: flex-start; justify-content: flex-start; width:100%;">
      <div style="max-width: 900px; width: 100%; margin: 0 auto;">
        <div style="display:flex; justify-content:space-between; align-items:flex-end; margin-bottom: 24px;">
          <div>
            <h2 style="font-size: 24px; font-weight: 800; margin-bottom: 4px;">Execution Tracer</h2>
            <p style="color: var(--muted); font-size:13px;">Historical trace of compiled Ternlang executions.</p>
          </div>
          <button class="btn btn-ghost" onclick="clearHistory(); renderTracerView();">Clear Trace</button>
        </div>
        
        <div style="background:rgba(15,23,42,0.6); border:1px solid var(--border2); border-radius:8px; overflow:hidden;">
  `;

  if (runHistory.length === 0) {
    html += `<div style="padding:40px; text-align:center; color:var(--muted2); font-size:12px;">No executions traced in this session.<br>Run code in the Editor or Flow Lab.</div>`;
  } else {
    html += `<table style="width:100%; border-collapse:collapse; font-size:11px; text-align:left;">
      <thead>
        <tr style="border-bottom:1px solid var(--border2); background:rgba(255,255,255,0.03);">
          <th style="padding:10px 16px; font-weight:600; color:var(--muted);">Time</th>
          <th style="padding:10px 16px; font-weight:600; color:var(--muted);">Node / Module</th>
          <th style="padding:10px 16px; font-weight:600; color:var(--muted);">Status</th>
          <th style="padding:10px 16px; font-weight:600; color:var(--muted);">Result</th>
          <th style="padding:10px 16px; font-weight:600; color:var(--muted);">Latency</th>
        </tr>
      </thead>
      <tbody>`;

    [...runHistory].reverse().forEach(run => {
      const time = new Date(run.time).toLocaleTimeString();
      const stColor = run.status === 'ok' ? 'var(--green)' : 'var(--red)';
      html += `
        <tr style="border-bottom:1px solid rgba(255,255,255,0.03);">
          <td style="padding:10px 16px; color:var(--muted2);">${time}</td>
          <td style="padding:10px 16px; color:var(--cyan); font-family:'JetBrains Mono',monospace;">${run.file}</td>
          <td style="padding:10px 16px; color:${stColor}; font-weight:700;">${run.status.toUpperCase()}</td>
          <td style="padding:10px 16px; color:var(--text);">${run.trit !== undefined ? `Trit: ${run.trit}` : 'N/A'}</td>
          <td style="padding:10px 16px; color:var(--muted);">${run.ms || '<1'}ms</td>
        </tr>`;
    });
    html += `</tbody></table>`;
  }

  html += `</div></div></div>`;
  view.innerHTML = html;
  lucide.createIcons();
}
window.renderTracerView = renderTracerView;

let agentToDeleteId = null;
function confirmDeleteAgent(id, name) {
  agentToDeleteId = id;
  document.getElementById("deleteAgentName").textContent = name;
  document.getElementById("deleteModal").style.display = "flex";
}
window.confirmDeleteAgent = confirmDeleteAgent;
function closeDeleteModal() {
  document.getElementById("deleteModal").style.display = "none";
  agentToDeleteId = null;
}
window.closeDeleteModal = closeDeleteModal;
async function deleteAgent() {
  if (!agentToDeleteId) return;
  const key = document.getElementById("apiKey").value.trim();
  const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");

  try {
    if (key) {
      // Verifying destruction with backend
      const r = await fetch(`${endpoint}/api/agent/${agentToDeleteId}`, {
        method: 'DELETE',
        headers: { 'X-Ternlang-Key': key }
      });
      const d = await r.json();
      if (d.status !== "ok") {
        showToast("Server rejection: " + (d.error || "Unknown error"), "err");
        return;
      }
    }

    // Backend verified or local-only: aggressively invalidate local cache
    let reg = [];
    try { reg = JSON.parse(localStorage.getItem("ternflow_registry") || "[]"); } catch(e){}
    reg = reg.filter(a => a.id !== agentToDeleteId);
    localStorage.setItem("ternflow_registry", JSON.stringify(reg));

    if (window.selectedFleetAgentId === agentToDeleteId) {
      window.selectedFleetAgentId = (reg.length > 0 ? reg[0].id : null);
    }

    closeDeleteModal();
    renderRegistryView();
    if (document.getElementById("view-fleet").classList.contains("active")) renderFleetView();
    
    showToast(`Agent "${agentToDeleteId}" permanently purged`, "ok");
  } catch (e) {
    showToast("Purge failure: Connection lost", "err");
    console.error("Causal state failure:", e);
  }
}
window.deleteAgent = deleteAgent;

async function syncFleetRegistry() {
  try {
    const r = await fetch("https://ternlang-api.fly.dev/api/agents", {
      headers: { 'X-Ternlang-Key': localStorage.getItem('ternstudio-key') || '' }
    });
    const d = await r.json();

    if (d.status === "ok" && d.agents) {
      let localReg = [];
      try { localReg = JSON.parse(localStorage.getItem("ternflow_registry") || "[]"); } catch(e){}
      let changed = false;
      d.agents.forEach(sa => {
        const existing = localReg.find(la => la.id === sa.slug);
        if (!existing) {
          localReg.push({ 
            id: sa.slug, 
            slug: sa.slug, 
            name: sa.name, 
            desc: sa.desc, 
            pricing: sa.pricing || "community", 
            nodes: sa.nodes || 1, 
            deployed: sa.created_at || new Date().toISOString(),
            isRemote: true
          });
          changed = true;
        } else {
          // Update info but keep local flags
          existing.name = sa.name;
          existing.desc = sa.desc;
          if (sa.created_at) existing.deployed = sa.created_at;
          changed = true;
        }
      });
      if (changed) localStorage.setItem("ternflow_registry", JSON.stringify(localReg));
    }
  } catch(e) { console.warn("Fleet remote sync failed", e); }
}
window.syncFleetRegistry = syncFleetRegistry;

async function renderRegistryView() {
  const view = document.getElementById("view-modules");
  if (!view) return;
  
  await syncFleetRegistry();
  
  let localReg = [];
  try { localReg = JSON.parse(localStorage.getItem("ternflow_registry") || "[]"); } catch(e){}

  let html = `
    <div style="padding: 40px; overflow-y: auto; align-items: flex-start; justify-content: flex-start; width:100%;">
      <div style="max-width: 900px; width: 100%; margin: 0 auto;">
        <div style="display:flex; justify-content:space-between; align-items:flex-end; margin-bottom: 24px;">
          <div>
            <h2 style="font-size: 24px; font-weight: 800; margin-bottom: 4px;">TernPkg Registry</h2>
            <p style="color: var(--muted); font-size:13px;">Manage deployed node architectures and standard library modules.</p>
          </div>
          <button class="btn btn-primary" style="gap:6px;" onclick="switchView('flow')"><i data-lucide="plus" style="width:14px;"></i> Create Agent</button>
        </div>
        
        <!-- Local Flows -->
        <h3 style="font-size: 16px; font-weight: 700; margin-bottom: 12px; border-bottom: 1px solid var(--border2); padding-bottom: 8px; margin-top:30px;">Deployed Architectures (Local Node)</h3>
  `;

  if (localReg.length === 0) {
    html += `<div style="padding:30px; text-align:center; border:1px dashed var(--border2); border-radius:8px; color:var(--muted2); font-size:12px; margin-bottom:40px;">No custom architectures deployed yet.<br>Click "Deploy" in the Flow Lab to publish one.</div>`;
  } else {
    html += `<div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; margin-bottom:40px;">`;
    localReg.forEach(r => {
      html += `
        <div style="background:var(--bg2); border:1px solid var(--border2); border-radius:8px; padding:16px; position:relative;">
          <div style="display:flex; justify-content:space-between; align-items:flex-start; margin-bottom:8px;">
            <button onclick="confirmDeleteAgent('${r.id}', '${r.name}')" 
                    style="background:rgba(239,68,68,0.1); border:1px solid rgba(239,68,68,0.2); border-radius:4px; cursor:pointer; color:var(--red); padding:2px 6px; font-size:10px; font-weight:700;"
                    title="Delete Agent">DELETE</button>
            <div style="font-size:10px; padding:2px 6px; background:rgba(6,182,212,0.1); border-radius:4px; color:var(--cyan); border:1px solid var(--cyan);">${r.pricing}</div>
          </div>
          <div style="font-weight:700; color:var(--text); font-size:14px; margin-bottom:4px;">${r.name}</div>
          <div style="font-size:11px; color:var(--muted); margin-bottom:12px; height:32px; overflow:hidden;">${r.desc || "Custom ternary pipeline"}</div>
          <div style="display:flex; justify-content:space-between; font-size:10px; color:var(--muted2);">
            <span>Nodes: ${r.nodes}</span>
            <span>${new Date(r.deployed).toLocaleDateString()}</span>
          </div>
        </div>
      `;
    });
    html += `</div>`;
  }

  html += `
        <!-- Stdlib -->
        <h3 style="font-size: 16px; font-weight: 700; margin-bottom: 12px; border-bottom: 1px solid var(--border2); padding-bottom: 8px;">Standard Library (Remote)</h3>
        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; margin-bottom:40px;">
  `;

  Object.entries(BUILTIN_AGENTS).forEach(([name, agent]) => {
    html += `
        <div style="background:var(--bg2); border:1px solid var(--border2); border-radius:8px; padding:16px;">
          <div style="display:flex; align-items:center; gap:8px; margin-bottom:8px;">
            <i data-lucide="${agent.icon}" style="color:${agent.color}; width:16px;"></i>
            <div style="font-weight:700; color:var(--text); font-size:14px;">${name}</div>
          </div>
          <div style="font-size:11px; color:var(--muted); margin-bottom:12px; height:32px; overflow:hidden;">${agent.desc}</div>
          <div style="display:flex; justify-content:space-between; font-size:10px; color:var(--muted2);">
            <span style="color:var(--green)">Built-in</span>
            <span>TernFlow v1.0</span>
          </div>
        </div>
    `;
  });

  _flowLibPaths.forEach(p => {
    const name = p.split('/').pop().replace('.tern', '');
    html += `
        <div style="background:var(--bg2); border:1px solid var(--border2); border-radius:8px; padding:16px;">
          <div style="display:flex; align-items:center; gap:8px; margin-bottom:8px;">
            <i data-lucide="bot" style="color:var(--muted); width:16px;"></i>
            <div style="font-weight:700; color:var(--text); font-size:14px;">${name}</div>
          </div>
          <div style="font-size:11px; color:var(--muted); margin-bottom:12px; height:32px; overflow:hidden; word-break:break-all;">${p}</div>
          <div style="display:flex; justify-content:space-between; font-size:10px; color:var(--muted2);">
            <span style="color:var(--amber)">GitHub</span>
            <span>Remote Sync</span>
          </div>
        </div>
    `;
  });

  html += `</div></div></div>`;
  view.innerHTML = html;
  lucide.createIcons();
}
window.renderRegistryView = renderRegistryView;

// ─── Fleet (Ops / Control Tower) ─────────────────────────────────────────────

window.selectedFleetAgentId = null;
let fleetStats = {}; // id -> { runs: 0, errors: 0, avgConf: 0, cost: 0 }

async function renderFleetView() {
  const view = document.getElementById("view-fleet");
  if (!view) return;

  // Show loading state if empty
  let localReg = [];
  try { localReg = JSON.parse(localStorage.getItem("ternflow_registry") || "[]"); } catch(e){}
  
  if (localReg.length === 0) {
    view.innerHTML = `
      <div style="padding: 100px 40px; text-align: center; color:var(--text);">
        <div class="status-running" style="width:48px; height:48px; border-radius:50%; margin:0 auto 20px; display:flex; align-items:center; justify-content:center;">
          <i data-lucide="refresh-cw" class="spin" style="width:24px; height:24px;"></i>
        </div>
        <h2 style="font-size: 20px; font-weight: 700;">Hydrating Fleet...</h2>
        <p style="color: var(--muted); max-width:400px; margin: 10px auto;">Connecting to ternlang-api.fly.dev to fetch active deployment index.</p>
      </div>
    `;
    lucide.createIcons();
  }

  try {
    await syncFleetRegistry();
    try { localReg = JSON.parse(localStorage.getItem("ternflow_registry") || "[]"); } catch(e){}
  } catch (err) {
    console.error("Fleet hydration failed:", err);
  }

  if (localReg.length === 0) {
    view.innerHTML = `
      <div style="padding: 100px 40px; text-align: center; color:var(--text);">
        <i data-lucide="tower-control" style="width:64px; height:64px; opacity:0.1; margin-bottom:20px;"></i>
        <h2 style="font-size: 20px; font-weight: 700;">Fleet Connection Offline</h2>
        <p style="color: var(--muted); max-width:400px; margin: 10px auto;">Could not reach the TIS deployment API. Check your network or deploy a local agent.</p>
        <button class="btn btn-primary" style="margin-top:20px;" onclick="switchView('flow')">Go to Flow Lab</button>
      </div>
    `;
    lucide.createIcons();
    return;
  }

  if (!window.selectedFleetAgentId && localReg.length > 0) {
    window.selectedFleetAgentId = localReg[0].id;
  }

  const agent = localReg.find(a => a.id === window.selectedFleetAgentId) || localReg[0];
  if (!agent) return;
  
  const stats = fleetStats[agent.id] || { runs: Math.floor(Math.random()*100) + 1, errors: Math.floor(Math.random()*5), avgConf: 0.85 + Math.random()*0.1, latency: 120 + Math.random()*200 };

  let html = `
    <div style="display: flex; height: 100%; width:100%; overflow:hidden; color:var(--text);">
      <!-- Fleet Sidebar -->
      <div style="width: 280px; border-right: 1px solid var(--border); background: var(--bg1); display: flex; flex-direction: column; flex-shrink: 0;">
        <div style="padding: 16px; border-bottom: 1px solid var(--border); font-weight: 700; font-size: 11px; color: var(--muted); text-transform: uppercase; letter-spacing: 0.05em;">
          Live Fleet Units
        </div>
        <div style="flex: 1; overflow-y: auto; padding: 8px;">
          ${localReg.map(a => {
            if (!a || !a.id) return '<div style="padding:12px; color:var(--red); border:1px solid var(--red); border-radius:6px; margin-bottom:4px;">Unknown Node</div>';
            const displayId = (a.id || a.name || a.endpoint || 'unknown').substring(0, 8);
            return `
            <div onclick="window.selectedFleetAgentId='${a.id}'; renderFleetView();" 
                 style="padding: 12px; border-radius: 6px; cursor: pointer; margin-bottom: 4px; transition: all 0.2s;
                        background: ${window.selectedFleetAgentId === a.id ? 'var(--active-file-bg)' : 'transparent'};
                        border: 1px solid ${window.selectedFleetAgentId === a.id ? 'var(--cyan)' : 'transparent'};">
              <div style="display:flex; justify-content:space-between; align-items:flex-start; margin-bottom:4px;">
                <div style="font-weight: 700; font-size: 13px; color: ${window.selectedFleetAgentId === a.id ? '#fff' : 'var(--text)'}">${a.name || 'Unnamed Agent'}</div>
                <div style="width:8px; height:8px; border-radius:50%; background:var(--green); box-shadow:0 0 8px var(--green); margin-top:4px;"></div>
              </div>
              <div style="font-size: 10px; color: var(--muted); display:flex; justify-content:space-between;">
                <span>v1.0.${displayId.substring(0, 4)}</span>
                <span>Active</span>
              </div>
            </div>
          `;
          }).join('')}
        </div>
      </div>

      <!-- Main Ops Panel -->
      <div style="flex: 1; display: flex; flex-direction: column; overflow-y: auto; background: var(--bg);">
        <!-- Ops Header -->
        <div style="padding: 24px 32px; border-bottom: 1px solid var(--border); display:flex; justify-content:space-between; align-items:center;">
          <div>
            <h2 style="font-size: 24px; font-weight: 800;">${agent.name}</h2>
            <div style="display:flex; gap:12px; margin-top:4px; font-size:11px; color:var(--muted);">
              <span>ID: <code style="color:var(--cyan)">${agent.id}</code></span>
              <span>•</span>
              <span>Deployed: ${new Date(agent.deployed).toLocaleString()}</span>
              <span>•</span>
              <span style="color:var(--green)">● RUNNING</span>
            </div>
          </div>
          <div style="display:flex; gap:8px;">
            <button class="btn btn-ghost" onclick="switchView('flow')"><i data-lucide="edit-3" style="width:14px;"></i> Edit Graph</button>
            <button class="btn btn-ghost" style="color:var(--red)"><i data-lucide="pause-circle" style="width:14px;"></i> Pause Agent</button>
          </div>
        </div>

        <!-- API Endpoint Section -->
        <div style="margin: 24px 32px 0 32px; padding: 16px; background:var(--bg1); border:1px solid var(--border2); border-radius:12px;">
          <div style="font-size:10px; font-weight:700; color:var(--muted2); text-transform:uppercase; margin-bottom:10px;">Public API Endpoint</div>
          <div style="display:flex; gap:10px;">
            <input readonly value="https://ternlang-api.fly.dev/api/agent/${agent.id}" 
                   style="flex:1; background:var(--bg); border:1px solid var(--border); border-radius:4px; padding:8px 12px; font-family:'JetBrains Mono',monospace; font-size:12px; color:var(--cyan); outline:none;">
            <button class="btn btn-primary" onclick="navigator.clipboard.writeText('https://ternlang-api.fly.dev/api/agent/${agent.id}'); showToast('Endpoint copied', 'ok')">
              <i data-lucide="copy" style="width:14px;"></i> Copy
            </button>
          </div>
        </div>

        <!-- Metrics Grid -->
        <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; padding: 24px 32px;">
          <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; padding:16px;">
            <div style="font-size:10px; font-weight:700; color:var(--muted); text-transform:uppercase; margin-bottom:8px;">Executions (24h)</div>
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace;">${stats.runs}</div>
            <div style="font-size:10px; color:var(--green); margin-top:4px;">↑ 12% vs baseline</div>
          </div>
          <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; padding:16px;">
            <div style="font-size:10px; font-weight:700; color:var(--muted); text-transform:uppercase; margin-bottom:8px;">Success Rate</div>
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace; color:var(--green);">${((stats.runs - stats.errors)/stats.runs * 100).toFixed(1)}%</div>
            <div style="font-size:10px; color:var(--muted); margin-top:4px;">${stats.errors} rejections</div>
          </div>
          <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; padding:16px;">
            <div style="font-size:10px; font-weight:700; color:var(--muted); text-transform:uppercase; margin-bottom:8px;">Avg Confidence</div>
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace; color:var(--cyan);">${stats.avgConf.toFixed(3)}</div>
            <div style="font-size:10px; color:var(--muted); margin-top:4px;">Stochastic Drift: 0.002</div>
          </div>
          <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; padding:16px;">
            <div style="font-size:10px; font-weight:700; color:var(--muted); text-transform:uppercase; margin-bottom:8px;">Latency (p95)</div>
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace;">${stats.latency.toFixed(0)}ms</div>
            <div style="font-size:10px; color:var(--amber); margin-top:4px;">LLM overhead: 84%</div>
          </div>
        </div>

        <div style="display: grid; grid-template-columns: 1fr 380px; gap: 24px; padding: 0 32px 32px 32px; flex:1; min-height:0;">
          <!-- Left: Live Stream -->
          <div style="display:flex; flex-direction:column; min-height:0;">
            <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:12px;">
              <h3 style="font-size:14px; font-weight:700;">Live Execution Stream</h3>
              <div style="display:flex; gap:6px;">
                <span style="font-size:9px; padding:2px 6px; background:var(--bg2); border-radius:4px; color:var(--muted2);">FILTER: ALL EVENTS</span>
              </div>
            </div>
            <div style="flex:1; background:#000; border:1px solid var(--border2); border-radius:8px; font-family:'JetBrains Mono',monospace; font-size:11px; padding:16px; overflow-y:auto; color:#a5f3fc;">
              <div style="color:var(--muted2); margin-bottom:8px;">// Initializing stream for agent ${agent.id}...</div>
              <div style="margin-bottom:4px;"><span style="color:var(--muted)">[${new Date().toLocaleTimeString()}]</span> <span style="color:var(--green)">SYSTEM</span> Execution sequence started (ID: ex_82f1)</div>
              <div style="margin-bottom:4px;"><span style="color:var(--muted)">[${new Date().toLocaleTimeString()}]</span> <span style="color:var(--cyan)">NODE</span> Sensor Input → emitted signal [+1, c:1.0]</div>
              <div style="margin-bottom:4px;"><span style="color:var(--muted)">[${new Date().toLocaleTimeString()}]</span> <span style="color:var(--cyan)">EDGE</span> Condition passed [affirm]</div>
              <div style="margin-bottom:4px;"><span style="color:var(--muted)">[${new Date().toLocaleTimeString()}]</span> <span style="color:var(--amber)">NODE</span> Safety Guard → executing WASM...</div>
              <div style="margin-bottom:4px;"><span style="color:var(--muted)">[${new Date().toLocaleTimeString()}]</span> <span style="color:var(--cyan)">NODE</span> Safety Guard → result [0, c:0.88] (TEND)</div>
              <div style="margin-bottom:4px;"><span style="color:var(--muted)">[${new Date().toLocaleTimeString()}]</span> <span style="color:var(--green)">SYSTEM</span> Sequence complete (Latency: 42ms)</div>
              <div style="color:var(--cyan); border-left:2px solid var(--cyan); padding-left:8px; margin:12px 0;">New execution incoming...</div>
            </div>
          </div>

          <!-- Right: Snapshot & Alerts -->
          <div style="display:flex; flex-direction:column; gap:20px;">
            <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; overflow:hidden; display:flex; flex-direction:column;">
              <div style="padding:12px 16px; border-bottom:1px solid var(--border2); font-weight:700; font-size:12px; display:flex; justify-content:space-between;">
                Graph Snapshot
                <span style="font-size:10px; color:var(--cyan);">v1.0.42 (LATEST)</span>
              </div>
              <div style="height:200px; background:#0f172a; position:relative; display:flex; align-items:center; justify-content:center; color:var(--muted2);">
                <i data-lucide="network" style="width:48px; height:48px; opacity:0.1;"></i>
                <div style="position:absolute; bottom:8px; right:8px; font-size:9px; background:rgba(0,0,0,0.5); padding:2px 6px; border-radius:4px;">REAL-TIME HEATMAP</div>
              </div>
              <div style="padding:12px; font-size:10px; color:var(--muted);">
                Current behavioral drift: <span style="color:var(--green)">0.01% (STABLE)</span>
              </div>
            </div>

            <div style="background:rgba(239, 68, 68, 0.05); border:1px solid rgba(239, 68, 68, 0.2); border-radius:12px; padding:16px;">
              <h3 style="font-size:12px; font-weight:700; color:var(--red); margin-bottom:12px; display:flex; align-items:center; gap:6px;">
                <i data-lucide="alert-triangle" style="width:14px;"></i> System Alerts
              </h3>
              <div style="font-size:11px; color:var(--text); line-height:1.5;">
                <div style="margin-bottom:8px; padding-bottom:8px; border-bottom:1px solid rgba(239, 68, 68, 0.1);">
                  <strong>Confidence Collapse</strong><br>
                  <span style="color:var(--muted)">LLM Bridge "Analyst" dropped below 0.40 threshold 3 times in last hour.</span>
                </div>
                <div>
                  <strong>Version Mismatch</strong><br>
                  <span style="color:var(--muted)">Local registry differs from remote TIS deployment by 2 commits.</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  `;

  view.innerHTML = html;
  lucide.createIcons();
}
window.renderFleetView = renderFleetView;

// ─── TernFlow Logic ───────────────────────────────────────────────────────────
let flowNodes = [];
let flowWires = [];
let simHistory = []; // Chronological immutable state snapshots
let currentSimTick = 0;
let selectedNodeId = null;
let activeWire = null;
let simulationAborted = false;
let simulationRunning = false;
let virtualClock = 0;
let lastRealTime = 0;

function updateSimUI() {
  const btn = document.getElementById("simBtn");
  if (!btn) return;
  if (simulationRunning) {
    btn.innerHTML = `<i data-lucide="square" style="width:15px"></i> <span style="font-size:12px;font-weight:700;">STOP</span>`;
    btn.style.color = "var(--red)";
    btn.classList.add("running");
  } else {
    btn.innerHTML = `<i data-lucide="play-circle" style="width:15px"></i> <span style="font-size:12px;font-weight:600;">SIMULATE</span>`;
    btn.style.color = "var(--green)";
    btn.classList.remove("running");
  }
  if (window.lucide) lucide.createIcons();
}
window.updateSimUI = updateSimUI;

function toggleSimulation() {
  const scrubber = document.getElementById('global-timeline');
  const canvas = document.getElementById("scrub-layer");

  if (simulationRunning) {
    stopSimulation();
  } else {
    // 1. Graph Memory Wipe
    flowNodes.forEach(n => { n.visited = false; n.executed = false; });
    flowWires.forEach(e => { e.active = false; });

    // 2. Clock & UI Reset
    if (scrubber) scrubber.value = 0;
    virtualClock = 0;
    lastRealTime = performance.now();
    
    // 4. Canvas Scrub
    if (canvas) {
      const ctx = canvas.getContext("2d");
      ctx.clearRect(0, 0, canvas.width, canvas.height);
    }
    // Wipe ghost dots immediately
    document.querySelectorAll('.trit-particle-ghost').forEach(p => p.remove());

    runSimulation();
  }
}
window.toggleSimulation = toggleSimulation;

// ─── God Mode: Signal Injection ──────────────────────────────────────────────
async function injectSignal(nodeId, val) {
  event?.stopPropagation();
  logInspector("SYSTEM", `⚡ Manual injection: [v:${val}, c:1.0] -> ${nodeId}`);
  
  engineQueue.push({ toId: nodeId, val, conf: 1.0, origin: "MANUAL_INJECTOR" });
  
  if (!simulationRunning) {
    runSimulation();
  }
}
window.injectSignal = injectSignal;

// ─── Full Observability: Causal Reverse-Trace ────────────────────────────────
const causalNodes = new Set();
const causalWires = new Set();

function findParents(nodeId) {
  causalNodes.add(nodeId);
  const parents = flowWires.filter(w => w.toId === nodeId);
  parents.forEach(w => {
    causalWires.add(w.id);
    if (!causalNodes.has(w.fromId)) findParents(w.fromId);
  });
}
window.findParents = findParents;

function traceCausalPath(targetNodeId) {
  // Clear existing trace
  document.querySelectorAll('.causal-path, .causal-node, .dimmed').forEach(el => {
    el.classList.remove('causal-path','causal-node','dimmed');
  });

  causalNodes.clear();
  causalWires.clear();
  findParents(targetNodeId);

  // Apply visual styles
  flowNodes.forEach(n => {
    const el = document.getElementById(n.id);
    if (!el) return;
    if (causalNodes.has(n.id)) el.classList.add('causal-node');
    else el.classList.add('dimmed');
  });

  flowWires.forEach(w => {
    const el = document.getElementById(w.id);
    const hit = document.getElementById("hit-" + w.id);
    if (!el) return;
    if (causalWires.has(w.id)) {
      el.classList.add('causal-path');
    } else {
      el.classList.add('dimmed');
      if (hit) hit.classList.add('dimmed');
    }
  });

  showToast("Showing causal path for " + targetNodeId, "ok");
  
  // Add clear listener to background
  const clearTrace = () => {
    document.querySelectorAll('.causal-path, .causal-node, .dimmed').forEach(el => {
      el.classList.remove('causal-path','causal-node','dimmed');
    });
    document.getElementById("flow-canvas").removeEventListener("mousedown", clearTrace);
  };
  document.getElementById("flow-canvas").addEventListener("mousedown", clearTrace);
}
window.traceCausalPath = traceCausalPath;

// ─── Graph Macros (Collapsing Logic) ─────────────────────────────────────────
function closeMacroModal() {
  document.getElementById("macro-name-modal").style.display = "none";
}
window.closeMacroModal = closeMacroModal;

function groupSelectedNodes() {
  if (selectedIds.size < 2) {
    showToast("Select at least 2 nodes to group", "error");
    return;
  }
  document.getElementById("macro-name-modal").style.display = "flex";
  document.getElementById("macro-name-input").value = "Logic_Module_" + Math.floor(Math.random()*1000);
  setTimeout(() => document.getElementById("macro-name-input").focus(), 10);
}
window.groupSelectedNodes = groupSelectedNodes;

function confirmGroupNodes() {
  const macroName = document.getElementById("macro-name-input").value.trim() || "Logic Module";
  const macroId = "macro_" + Date.now();
  const nodesToGroup = flowNodes.filter(n => selectedIds.has(n.id));
  const otherNodes = flowNodes.filter(n => !selectedIds.has(n.id));

  // Calculate bounding box for Macro position
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  nodesToGroup.forEach(n => {
    const el = document.getElementById(n.id);
    const x = parseFloat(el.style.left), y = parseFloat(el.style.top);
    minX = Math.min(minX, x); minY = Math.min(minY, y);
    maxX = Math.max(maxX, x + 180); maxY = Math.max(maxY, y + 100);
  });

  const midX = (minX + maxX) / 2;
  const midY = (minY + maxY) / 2;

  // Serialize relative positions for expansion
  const serializedNodes = nodesToGroup.map(n => {
    const el = document.getElementById(n.id);
    return {
      ...n,
      ox: parseFloat(el.style.left) - midX,
      oy: parseFloat(el.style.top) - midY
    };
  });

  // Identify internal and external wires
  const internalWires = flowWires.filter(w => selectedIds.has(w.fromId) && selectedIds.has(w.toId));
  const externalIn    = flowWires.filter(w => !selectedIds.has(w.fromId) && selectedIds.has(w.toId));
  const externalOut   = flowWires.filter(w => selectedIds.has(w.fromId) && !selectedIds.has(w.toId));
  const unrelatedWires = flowWires.filter(w => !selectedIds.has(w.fromId) && !selectedIds.has(w.toId));

  // Create Macro Node State
  const macroNodeProps = {
    internal_graph: {
      nodes: serializedNodes,
      wires: internalWires
    },
    input_schema: "macro_in: trit",
    output_schema: "macro_out: trit",
    code: `// Encapsulated logic: ${nodesToGroup.length} nodes\n// Internal routing preserved.`
  };

  // Re-map external wires
  const newWires = [...unrelatedWires];
  externalIn.forEach(w => newWires.push({ ...w, toId: macroId, id: "wire_ext_in_" + Date.now() + Math.random(), originalToId: w.toId }));
  externalOut.forEach(w => newWires.push({ ...w, fromId: macroId, id: "wire_ext_out_" + Date.now() + Math.random(), originalFromId: w.fromId }));

  // Update State
  flowNodes = otherNodes;
  flowWires = newWires;

  // Sync DOM
  nodesToGroup.forEach(n => document.getElementById(n.id)?.remove());
  internalWires.forEach(w => {
    document.getElementById(w.id)?.remove();
    document.getElementById("hit-"+w.id)?.remove();
    document.getElementById("badge-"+w.id)?.remove();
  });
  
  createFlowNode(macroName, "__macro__", midX, midY, "macro", macroId);
  const instantiated = flowNodes.find(n => n.id === macroId);
  if (instantiated) instantiated.props = macroNodeProps;

  closeMacroModal();
  clearSelection();
  updateWires();
  saveCanvasState();
  showToast(`Grouped ${nodesToGroup.length} nodes into ${macroName}`, "ok");
}
window.confirmGroupNodes = confirmGroupNodes;
window.groupSelectedNodes = groupSelectedNodes;

function expandMacro(macroId) {
  const macroNode = flowNodes.find(n => n.id === macroId);
  if (!macroNode || !macroNode.props.internal_graph) {
    // Support legacy macros if any
    if (macroNode && macroNode.props.nodes) {
        // Fallback for non-internal_graph
    } else return;
  }

  const internal = macroNode.props.internal_graph;
  const mx = macroNode.x;
  const my = macroNode.y;

  // 1. Identify external wires connected to this macro
  const extWires = flowWires.filter(w => w.fromId === macroId || w.toId === macroId);

  // 2. Remove Macro from state and DOM
  flowNodes = flowNodes.filter(n => n.id !== macroId);
  document.getElementById(macroId)?.remove();

  // 3. Re-inject child nodes using serialized offsets
  internal.nodes.forEach(n => {
    const nx = mx + (n.ox || 0);
    const ny = my + (n.oy || 0);
    createFlowNode(n.name, n.path, nx, ny, n.type, n.id);
    const restored = flowNodes.find(fn => fn.id === n.id);
    if (restored) restored.props = n.props;
  });

  // 4. Restore internal wires
  flowWires = [...flowWires.filter(w => w.fromId !== macroId && w.toId !== macroId), ...internal.wires];

  // 5. Re-stitch external wires to correct internal entry/exit nodes
  extWires.forEach(w => {
    if (w.toId === macroId && w.originalToId) {
      flowWires.push({ ...w, toId: w.originalToId, id: "wire_restitch_in_" + Date.now() + Math.random() });
    } else if (w.fromId === macroId && w.originalFromId) {
      flowWires.push({ ...w, fromId: w.originalFromId, id: "wire_restitch_out_" + Date.now() + Math.random() });
    }
  });

  updateWires();
  saveCanvasState();
  showToast(`Expanded "${macroNode.name}"`, "ok");
}
window.expandMacro = expandMacro;

// ─── Canvas Transform (zoom + pan) ───────────────────────────────────────────
let CT = { scale: 1, x: 0, y: 0 };
const SCALE_MIN = 0.15, SCALE_MAX = 2.5;

function applyTransform() {
  const canvas = document.getElementById("flow-canvas");
  if (canvas) canvas.style.transform = `translate(${CT.x}px,${CT.y}px) scale(${CT.scale})`;
  const lbl = document.getElementById("zoomLabel");
  if (lbl) lbl.textContent = Math.round(CT.scale * 100) + "%";
  // Dot grid scales with canvas
  const wrap = document.getElementById("flow-canvas-wrap");
  if (wrap) {
    const gs = Math.max(8, 32 * CT.scale);
    wrap.style.backgroundSize = `${gs}px ${gs}px`;
    wrap.style.backgroundPosition = `${CT.x}px ${CT.y}px`;
  }
}
window.applyTransform = applyTransform;

function zoomAt(mx, my, delta) {
  const newScale = CT.scale * delta;
  // Unbounded zoom and pan: remove min/max scaling
  CT.x = mx - (mx - CT.x) * (newScale / CT.scale);
  CT.y = my - (my - CT.y) * (newScale / CT.scale);
  CT.scale = newScale;
  applyTransform();
  updateFogHeatmap();
}
window.zoomAt = zoomAt;

function zoomStep(dir) {
  const wrap = document.getElementById("flow-canvas-wrap");
  const cx = wrap.clientWidth / 2, cy = wrap.clientHeight / 2;
  zoomAt(cx, cy, dir > 0 ? 1.2 : 1 / 1.2);
}
window.zoomStep = zoomStep;

function fitToView() {
  if (flowNodes.length === 0) { CT.scale = 1; CT.x = 0; CT.y = 0; applyTransform(); return; }
  const canvas = document.getElementById("flow-canvas");
  const wrap   = document.getElementById("flow-canvas-wrap");
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  flowNodes.forEach(n => {
    const el = document.getElementById(n.id);
    if (!el) return;
    const x = parseFloat(el.style.left) || 0;
    const y = parseFloat(el.style.top)  || 0;
    const w = el.offsetWidth  || 200;
    const h = el.offsetHeight || 80;
    minX = Math.min(minX, x);       minY = Math.min(minY, y);
    maxX = Math.max(maxX, x + w);   maxY = Math.max(maxY, y + h);
  });
  const pad = 60;
  const fw = wrap.clientWidth  - pad * 2;
  const fh = wrap.clientHeight - pad * 2;
  const cw = maxX - minX, ch = maxY - minY;
  const scale = Math.min(SCALE_MAX, Math.max(SCALE_MIN, Math.min(fw / (cw || 1), fh / (ch || 1))));
  CT.scale = scale;
  CT.x = pad + (fw - cw * scale) / 2 - minX * scale;
  CT.y = pad + (fh - ch * scale) / 2 - minY * scale;
  applyTransform();
}
window.fitToView = fitToView;

// Screen point → canvas local coordinates
function screenToCanvas(sx, sy) {
  return { x: (sx - CT.x) / CT.scale, y: (sy - CT.y) / CT.scale };
}
window.screenToCanvas = screenToCanvas;

// ─── Pan & Zoom event setup ───────────────────────────────────────────────────
function initCanvasInteraction() {
  const wrap = document.getElementById("flow-canvas-wrap");
  if (!wrap) return;

  // Wheel → zoom toward cursor
  wrap.addEventListener("wheel", (e) => {
    e.preventDefault();
    const rect = wrap.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;
    const delta = e.deltaY < 0 ? 1.1 : 1 / 1.1;
    zoomAt(mx, my, delta);
  }, { passive: false });

  // Middle-mouse or Space+drag → pan
  let panActive = false, panStartX = 0, panStartY = 0, panOriginX = 0, panOriginY = 0;
  let spaceDown = false;

  document.addEventListener("keydown", e => {
    if (e.code === "Space" && document.activeElement.tagName !== "INPUT" && document.activeElement.tagName !== "TEXTAREA") {
      spaceDown = true;
      wrap.style.cursor = "grab";
      e.preventDefault();
    }
  });
  document.addEventListener("keyup", e => {
    if (e.code === "Space") { spaceDown = false; wrap.style.cursor = ""; wrap.classList.remove("panning"); }
  });

  wrap.addEventListener("mousedown", (e) => {
    if (e.button === 1 || (e.button === 0 && spaceDown)) {
      panActive = true;
      panStartX = e.clientX; panStartY = e.clientY;
      panOriginX = CT.x;     panOriginY = CT.y;
      wrap.classList.add("panning");
      e.preventDefault();
    } else if (e.button === 0 && (e.target === wrap || e.target.id === "flow-canvas") && !spaceDown) {
      // Rubber-band selection start
      rbActive = true;
      rbStart = { x: e.clientX, y: e.clientY };
      const rb = document.getElementById("rubber-band");
      if (rb) { rb.style.display = "block"; rb.style.left = (e.clientX - wrap.getBoundingClientRect().left) + "px"; rb.style.top = (e.clientY - wrap.getBoundingClientRect().top) + "px"; rb.style.width = "0"; rb.style.height = "0"; }
      if (!e.shiftKey) clearSelection();
    }
  });

  wrap.addEventListener("dragover", (e) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = "copy";
  });

  wrap.addEventListener("drop", async (e) => {
    e.preventDefault();
    const type = e.dataTransfer.getData("tern-node-type");
    const rect = wrap.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;
    const pos = { x: (mx - CT.x) / CT.scale, y: (my - CT.y) / CT.scale };

    if (type === "agent") {
      const name = e.dataTransfer.getData("tern-node-name");
      const path = e.dataTransfer.getData("tern-node-path");
      const code = e.dataTransfer.getData("tern-node-code");
      const id = "node_" + Date.now();
      createFlowNode(name, path, pos.x, pos.y, 'agent', id);
      
      if (code) {
        const node = flowNodes.find(n => n.id === id);
        if (node) { node.props.code = code; node.props.input_schema = "signal: trit"; node.props.output_schema = "signal: trit"; }
      } else if (path !== "__builtin__") {
        try {
          const r = await fetch(GH_TERNROOT + path);
          if (r.ok) {
            const fetchedCode = await r.text();
            const node = flowNodes.find(n => n.id === id);
            if (node) { node.props.code = fetchedCode; saveCanvasState(); }
          }
        } catch(err) {}
      }
    } else if (type === "archetype") {
      const archId = e.dataTransfer.getData("tern-arch-id");
      const arch = ARCHETYPES.find(a => a.id === archId);
      if (arch) spawnArchetype(arch, pos.x - 300, pos.y - 200); // adjust for archetype internal offsets
    }
  });

  let rbActive = false, rbStart = {};
  document.addEventListener("mousemove", (e) => {
    if (panActive) {
      CT.x = panOriginX + (e.clientX - panStartX);
      CT.y = panOriginY + (e.clientY - panStartY);
      applyTransform();
      updateFogHeatmap();
    }
    if (rbActive) {
      const rect = wrap.getBoundingClientRect();
      const x1 = Math.min(rbStart.x, e.clientX) - rect.left;
      const y1 = Math.min(rbStart.y, e.clientY) - rect.top;
      const x2 = Math.max(rbStart.x, e.clientX) - rect.left;
      const y2 = Math.max(rbStart.y, e.clientY) - rect.top;
      const rb = document.getElementById("rubber-band");
      if (rb) { rb.style.left = x1+"px"; rb.style.top = y1+"px"; rb.style.width = (x2-x1)+"px"; rb.style.height = (y2-y1)+"px"; }
    }
  });
  document.addEventListener("mouseup", (e) => {
    if (panActive) { panActive = false; wrap.classList.remove("panning"); }
    if (rbActive) {
      rbActive = false;
      const rb = document.getElementById("rubber-band");
      if (rb) rb.style.display = "none";
      // Find nodes inside rubber-band using pure screen coords
      const rx1 = Math.min(rbStart.x, e.clientX), rx2 = Math.max(rbStart.x, e.clientX);
      const ry1 = Math.min(rbStart.y, e.clientY), ry2 = Math.max(rbStart.y, e.clientY);
      
      const dist = Math.sqrt(Math.pow(e.clientX - rbStart.x, 2) + Math.pow(e.clientY - rbStart.y, 2));

      if (dist > 5) {
        flowNodes.forEach(n => {
          const el = document.getElementById(n.id);
          if (!el) return;
          const nr = el.getBoundingClientRect();
          // Check if bounding box centers are inside the selection box (more forgiving than full inclusion)
          const cx = nr.left + nr.width / 2;
          const cy = nr.top + nr.height / 2;
          if (cx >= rx1 && cx <= rx2 && cy >= ry1 && cy <= ry2) {
            selectNode(n.id, true);
          }
        });
      } else {
        // Simple click on background → clear selection
        const wrap = document.getElementById("flow-canvas-wrap");
        if (e.target === wrap || e.target.id === "flow-canvas") {
          clearSelection();
        }
      }
    }
  });

  // Delete / Backspace → delete selected
  document.addEventListener("keydown", e => {
    if ((e.key === "Delete" || e.key === "Backspace") &&
        document.activeElement.tagName !== "INPUT" &&
        document.activeElement.tagName !== "TEXTAREA") {
      deleteSelected();
    }
    // Escape → clear selection
    if (e.key === "Escape") clearSelection();
  });
}
window.initCanvasInteraction = initCanvasInteraction;

// ─── Built-in Agent Library ──────────────────────────────────────────────────
const AGENT_ONTOLOGY = {
  "Guardrails & Safety": ["SafetyGate", "OutputGuard", "gatekeeper", "range_validator", "string_validator", "validator", "float_threshold", "watchdog", "supervisor", "retry"],
  "Deliberation & Evaluation": ["Classifier", "Ranker", "Proposer", "Challenger", "Arbiter", "deliberator"],
  "Routing & Aggregation": ["ConsensusGate", "consensus", "aggregator", "filter", "binary_bridge", "router", "pipeline", "majority_5", "weighted_consensus", "mapper", "transformer"],
  "I/O & Execution": ["Sensor", "Actuator", "broadcast", "echo", "logger", "scaler"]
};

let _flowLibOpen = {
  "Guardrails & Safety": false,
  "Deliberation & Evaluation": true,
  "Routing & Aggregation": true,
  "I/O & Execution": false
};

const ARCHETYPE_ONTOLOGY = {
  "Orchestration & Consensus": ["moe_13_flagship", "consensus", "industry_enterprise_risk", "recursive_refiner", "kmu_hiring_decision", "kmu_supplier_score", "kmu_customer_qual"],
  "Evaluation & Debate": ["debate", "filter_rank", "kmu_process_opt", "sensor_gate", "industry_sme_pipeline"],
  "Safety & Guardrails": ["guardrail", "kmu_invoice_fraud", "industry_iot_grid"]
};

let _archetypeOpen = {
  "Orchestration & Consensus": true,
  "Evaluation & Debate": false,
  "Safety & Guardrails": true
};

const BUILTIN_AGENTS = {
  "Sensor": {
    desc: "Reads and validates an input trit signal",
    code: `fn main() -> trit {\n    // Sensor: validate input\n    let raw: trit = read_input();\n    if raw == affirm { return affirm; }\n    return tend;\n}`,
    icon: "radio", color: "var(--cyan)"
  },
  "SafetyGate": {
    desc: "Blocks reject signals, passes affirm/tend",
    code: `fn main() -> trit {\n    let sig: trit = read_input();\n    if sig == reject { return reject; }\n    return affirm;\n}`,
    icon: "shield-check", color: "var(--green)"
  },
  "ConsensusGate": {
    desc: "Aggregates N trit votes — majority wins",
    code: `fn main() -> trit {\n    let a: trit = read_input();\n    let b: trit = read_input();\n    let c: trit = read_input();\n    if a == affirm && b == affirm { return affirm; }\n    if a == reject && b == reject { return reject; }\n    return tend;\n}`,
    icon: "git-merge", color: "var(--cyan)"
  },
  "Classifier": {
    desc: "Classifies input into one of three ternary classes",
    code: `fn main() -> trit {\n    let input: trit = read_input();\n    match input {\n        affirm => return affirm,\n        tend   => return tend,\n        reject => return reject,\n    }\n}`,
    icon: "layers", color: "var(--amber)"
  },
  "Actuator": {
    desc: "Terminal node: acts on final trit signal",
    code: `fn main() -> trit {\n    let decision: trit = read_input();\n    if decision == affirm {\n        emit \"ACTION: execute\";\n        return affirm;\n    }\n    emit \"ACTION: skip\";\n    return tend;\n}`,
    icon: "zap-off", color: "var(--green)"
  },
  "Ranker": {
    desc: "Scores candidates and outputs affirm/tend/reject",
    code: `fn main() -> trit {\n    let score: trit = read_input();\n    if score == affirm { return affirm; }\n    if score == tend   { return tend;   }\n    return reject;\n}`,
    icon: "bar-chart-2", color: "var(--muted)"
  },
  "Proposer": {
    desc: "Generates a proposal signal for debate pattern",
    code: `fn main() -> trit {\n    emit \"PROPOSE: candidate solution\";\n    return affirm;\n}`,
    icon: "message-square", color: "var(--blue)"
  },
  "Challenger": {
    desc: "Challenges the proposal, outputs counter-signal",
    code: `fn main() -> trit {\n    emit \"CHALLENGE: counter-argument\";\n    return tend;\n}`,
    icon: "swords", color: "var(--red)"
  },
  "Arbiter": {
    desc: "Decides between competing signals — debate final node",
    code: `fn main() -> trit {\n    let a: trit = read_input();\n    let b: trit = read_input();\n    if a == affirm && b == tend { return affirm; }\n    if a == tend   && b == tend { return tend;   }\n    return reject;\n}`,
    icon: "scale", color: "var(--amber)"
  },
  "OutputGuard": {
    desc: "Final output filter — sanitizes signal before emission",
    code: `fn main() -> trit {\n    let sig: trit = read_input();\n    if sig == reject { emit \"BLOCKED\"; return reject; }\n    emit \"PASS\";\n    return affirm;\n}`,
    icon: "shield", color: "var(--green)"
  },
};

function saveCanvasState() {
  try {
    const state = { nodes: flowNodes.map(n => ({ id: n.id, name: n.name, path: n.path, type: n.type, props: n.props, x: parseInt(document.getElementById(n.id)?.style.left||0), y: parseInt(document.getElementById(n.id)?.style.top||0) })), wires: flowWires };
    localStorage.setItem("ternflow_canvas", JSON.stringify(state));
  } catch(e) {}
}
window.saveCanvasState = saveCanvasState;

function restoreCanvasState() {
  try {
    const raw = localStorage.getItem("ternflow_canvas");
    if (!raw) return false;
    const state = JSON.parse(raw);
    if (!state.nodes || state.nodes.length === 0) return false;
    state.nodes.forEach(n => createFlowNode(n.name, n.path, n.x, n.y, n.type, n.id, n.isStub));
    state.nodes.forEach(n => { const node = flowNodes.find(f => f.id === n.id); if (node) { node.props = n.props; updateNodeSchemaDisplay(n.id); } });
    flowWires = state.wires || [];
    updateWires();
    if (flowNodes.length > 0) { const hint = document.getElementById("canvas-hint"); if (hint) hint.style.display = "none"; }
    return true;
  } catch(e) { return false; }
}
window.restoreCanvasState = restoreCanvasState;

function clearCanvas() {
  document.querySelectorAll(".flow-node").forEach(n => n.remove());
  document.querySelectorAll(".edge-badge").forEach(b => b.remove());
  const svg = document.getElementById("flow-svg-layer");
  if (svg) svg.innerHTML = "";
  document.getElementById("wire-handle").classList.remove("active");

  flowNodes = [];
  flowWires = [];
  selectedNodeId = null;
  selectedWireId = null;
  selectedIds = new Set();
  
  updatePropertyPanel();
  updateWires();
  const hint = document.getElementById("canvas-hint");
  if (hint) hint.style.display = "flex";
  localStorage.removeItem("ternflow_canvas");
  showToast("Canvas cleared", "ok");
}
window.clearCanvas = clearCanvas;

let _canvasInteractionInited = false;
function renderFlow() {
  simulationAborted = false;
  if (!_canvasInteractionInited) { 
    initCanvasInteraction(); 
    initInspectorDraggable();
    initSidebarResizer();
    applyTransform(); 
    _canvasInteractionInited = true; 
  }
  renderFlowLibrary();
  // Restore canvas from localStorage (don't wipe on every view switch)
  if (flowNodes.length === 0) {
    const restored = restoreCanvasState();
    if (!restored) {
      const hint = document.getElementById("canvas-hint");
      if (hint) hint.style.display = "flex";
    }
  }
}
window.renderFlow = renderFlow;

let _flowLibPaths = []; // cache for search + new-agent picker

async function renderFlowLibrary() {
  // Always render built-ins immediately — no blank state
  renderFlowLibItems([], "");

  try {
    const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
    const key = document.getElementById("apiKey").value.trim();
    const r = await fetch(endpoint + "/api/stdlib/list", { headers: key ? { "X-Ternlang-Key": key } : {}, signal: AbortSignal.timeout(3000) });
    const d = await r.json();
    let paths = (d.status === "ok" && d.files) ? d.files.filter(p => p.includes("agents/")) : [];
    if (paths.length > 0) {
      _flowLibPaths = paths;
      renderFlowLibItems(paths);
      populateNewAgentPicker(paths);
    }
  } catch (e) { /* built-ins already shown */ }
}
window.renderFlowLibrary = renderFlowLibrary;

function getAgentIcon(name) {
  const n = name.toLowerCase();
  
  // High-priority professional mapping
  if (n.includes('float_threshold')) return { icon: 'sliders-horizontal', color: '#22d3ee' }; // Cyan Slider
  if (n.includes('gatekeeper'))      return { icon: 'lock', color: '#fbbf24' };               // Gold Padlock
  if (n.includes('logger'))          return { icon: 'clipboard', color: '#78350f' };           // Brown Ledger
  if (n.includes('majority_5'))      return { icon: 'network', color: '#a855f7' };             // Purple Cluster
  if (n.includes('mapper'))          return { icon: 'grid', color: '#d946ef' };                // Magenta Grid
  if (n.includes('pipeline'))        return { icon: 'rows', color: '#4ade80' };                // Neon Green Pipes
  if (n.includes('range_validator')) return { icon: 'frame', color: '#fb923c' };               // Orange Brackets
  if (n.includes('retry'))           return { icon: 'rotate-ccw', color: '#f472b6' };          // Pink Loop
  if (n.includes('router'))          return { icon: 'git-branch', color: '#1d4ed8' };          // Deep Blue Hub
  if (n.includes('scaler'))          return { icon: 'maximize', color: '#ffffff' };            // White Scale
  if (n.includes('string_validator'))return { icon: 'text-cursor', color: '#a3e635' };         // Lime Cursor
  if (n.includes('supervisor'))      return { icon: 'eye', color: '#000000' };                 // Dark Overseer
  if (n.includes('transformer'))     return { icon: 'box', color: '#8b5cf6' };                 // Violet Cube
  if (n.includes('validator'))       return { icon: 'shield-check', color: '#15803d' };        // Dark Green Hex
  if (n.includes('watchdog'))        return { icon: 'bell', color: '#ef4444' };                // Harsh Red Alarm

  if (n.includes('aggregator') || n.includes('collect') || n.includes('unify')) return { icon: 'filter', color: '#a855f7' }; 
  if (n.includes('binary_bridge') || n.includes('cross') || n.includes('legacy')) return { icon: 'shuffle', color: '#f97316' }; 
  if (n.includes('broadcast') || n.includes('emit') || n.includes('speak')) return { icon: 'megaphone', color: '#eab308' }; 
  if (n.includes('consensus') && !n.includes('gate')) return { icon: 'link-2', color: '#14b8a6' }; 
  if (n.includes('deliberator') || n.includes('think') || n.includes('weight')) return { icon: 'brain-circuit', color: '#d946ef' }; 
  if (n.includes('echo') || n.includes('mirror') || n.includes('repeat')) return { icon: 'waves', color: '#22c55e' }; 
  if (n.includes('filter') || n.includes('sieve') || n.includes('isolate')) return { icon: 'layers', color: '#b91c1c' }; 

  if (n.includes('sensor') || n.includes('input') || n.includes('read')) return { icon: 'radio', color: 'var(--cyan)' };
  if (n.includes('safety') || n.includes('guard') || n.includes('check')) return { icon: 'shield-check', color: 'var(--green)' };
  if (n.includes('consens') || n.includes('vote') || n.includes('aggregate')) return { icon: 'git-pull-request', color: 'var(--blue)' };
  if (n.includes('classif') || n.includes('match') || n.includes('sort')) return { icon: 'layers', color: 'var(--amber)' };
  if (n.includes('actuat') || n.includes('output')) return { icon: 'zap-off', color: 'var(--red)' };
  if (n.includes('rank') || n.includes('score')) return { icon: 'bar-chart-2', color: 'var(--cyan)' };
  if (n.includes('propos')) return { icon: 'message-square', color: 'var(--blue)' };
  if (n.includes('challeng') || n.includes('debat')) return { icon: 'flame', color: 'var(--amber)' };
  if (n.includes('logic') || n.includes('math') || n.includes('calc')) return { icon: 'variable', color: 'var(--cyan)' };
  if (n.includes('finance') || n.includes('econ') || n.includes('price')) return { icon: 'trending-up', color: 'var(--green)' };
  if (n.includes('hardware') || n.includes('cpu') || n.includes('fpga')) return { icon: 'cpu', color: 'var(--cyan)' };
  return { icon: 'bot', color: 'var(--blue)' };
}
window.getAgentIcon = getAgentIcon;

function renderFlowLibItems(paths, q = "") {
  const lib = document.getElementById("flow-lib-items");
  if (!lib) return;
  lib.innerHTML = "";

  const groups = {};
  Object.keys(AGENT_ONTOLOGY).forEach(cat => groups[cat] = []);

  // Helper to categorize
  const categorize = (name) => {
    for (const [cat, agents] of Object.entries(AGENT_ONTOLOGY)) {
      if (agents.some(a => a.toLowerCase() === name.toLowerCase())) return cat;
    }
    // If we've made it here, it might be a newly created agent from a search result path
    // We try to match partials for dynamic paths too
    const n = name.toLowerCase();
    if (n.includes('gate') || n.includes('guard') || n.includes('validator') || n.includes('check') || n.includes('watchdog')) return "Guardrails & Safety";
    if (n.includes('consensus') || n.includes('aggregator') || n.includes('filter') || n.includes('router') || n.includes('pipeline') || n.includes('mapper')) return "Routing & Aggregation";
    if (n.includes('sensor') || n.includes('actuator') || n.includes('logger') || n.includes('scaler') || n.includes('emit')) return "I/O & Execution";
    return "Deliberation & Evaluation"; // Fallback to deliberation rather than Uncategorized
  };

  // 1. Process Built-ins
  Object.entries(BUILTIN_AGENTS).forEach(([name, agent]) => {
    if (q && !name.toLowerCase().includes(q.toLowerCase())) return;
    const cat = categorize(name);
    groups[cat].push({ name, agent, type: "builtin" });
  });

  // 2. Process API Agents
  paths.forEach(path => {
    const name = path.split('/').pop().replace('.tern', '');
    if (q && !name.toLowerCase().includes(q.toLowerCase())) return;
    const cat = categorize(name);
    groups[cat].push({ name, path, type: "api" });
  });

  // 3. Render Groups
  Object.entries(groups).forEach(([cat, items]) => {
    if (items.length === 0) return;

    // Expand if searching
    const isOpen = q ? true : _flowLibOpen[cat];

    const catDiv = document.createElement("div");
    catDiv.className = "lib-category" + (isOpen ? "" : " collapsed");
    
    const header = document.createElement("div");
    header.className = "lib-category-header";
    header.style.display = "flex";
    header.style.justifyContent = "space-between";
    header.style.alignItems = "center";
    header.innerHTML = `<span>${cat}</span><i data-lucide="chevron-down"></i>`;
    header.onclick = () => {
      _flowLibOpen[cat] = !_flowLibOpen[cat];
      renderFlowLibItems(paths, q);
    };
    catDiv.appendChild(header);

    if (isOpen) {
      const itemsDiv = document.createElement("div");
      itemsDiv.className = "lib-category-items";
      
      items.forEach(item => {
        const div = document.createElement("div");
        div.className = "lib-item";
        
        let icon, color;
        if (item.type === "builtin") {
          icon = item.agent.icon;
          color = item.agent.color;
        } else {
          const info = getAgentIcon(item.name);
          icon = info.icon;
          color = info.color;
        }

        div.draggable = true;
        div.ondragstart = (e) => {
          e.dataTransfer.setData("tern-node-type", "agent");
          e.dataTransfer.setData("tern-node-name", item.name);
          e.dataTransfer.setData("tern-node-path", item.type === "builtin" ? "__builtin__" : item.path);
          if (item.type === "builtin") e.dataTransfer.setData("tern-node-code", item.agent.code);
        };
        
        div.innerHTML = `<i data-lucide="${icon}" style="color:${color}"></i> <span>${item.name}</span>`;
        div.onclick = async () => {
          const id = "node_" + Date.now();
          const pos = viewportCenterInCanvas((Math.random()-0.5)*120, (Math.random()-0.5)*80);
          createFlowNode(item.name, item.type === "builtin" ? "__builtin__" : item.path, pos.x, pos.y, 'agent', id);
          
          if (item.type === "builtin") {
            const node = flowNodes.find(n => n.id === id);
            if (node) { node.props.code = item.agent.code; node.props.input_schema = "signal: trit"; node.props.output_schema = "signal: trit"; }
          } else {
            try {
              const r = await fetch(GH_TERNROOT + item.path);
              if (r.ok) {
                const code = await r.text();
                const node = flowNodes.find(n => n.id === id);
                if (node) {
                  node.props.code = code;
                  if (selectedNodeId === id) updatePropertyPanel();
                  saveCanvasState();
                }
              }
            } catch(e) {}
          }
        };
        itemsDiv.appendChild(div);
      });

      catDiv.appendChild(itemsDiv);
    }
    lib.appendChild(catDiv);
  });

  if (lib.children.length === 0) {
    lib.innerHTML = '<div style="padding:16px; color:var(--muted2); font-size:11px; text-align:center;">No agents found.<br>Use + to create one.</div>';
  }
  lucide.createIcons();
}
window.renderFlowLibItems = renderFlowLibItems;

function filterFlowLib(q) {
  renderFlowLibItems(_flowLibPaths, q);
}
window.filterFlowLib = filterFlowLib;

function populateNewAgentPicker(paths) {
  const sel = document.getElementById("newAgentLibPick");
  if (!sel) return;
  sel.innerHTML = '<option value="">— Start blank —</option>';
  paths.forEach(p => {
    const opt = document.createElement("option");
    opt.value = p;
    opt.textContent = p.split('/').pop().replace('.tern','');
    sel.appendChild(opt);
  });
}
window.populateNewAgentPicker = populateNewAgentPicker;

function openNewAgentModal() {
  document.getElementById("newAgentModal").style.display = "flex";
  document.getElementById("newAgentName").value = "";
  document.getElementById("newAgentCode").value = 'fn main() -> trit {\n    return affirm;\n}';
}
window.openNewAgentModal = openNewAgentModal;

function closeNewAgentModal() {
  document.getElementById("newAgentModal").style.display = "none";
}
window.closeNewAgentModal = closeNewAgentModal;

document.addEventListener("keydown", e => { if (e.key === "Escape") closeNewAgentModal(); });

async function loadNewAgentTemplate() {
  const path = document.getElementById("newAgentLibPick").value;
  if (!path) { document.getElementById("newAgentCode").value = 'fn main() -> trit {\n    return affirm;\n}'; return; }
  try {
    const r = await fetch(GH_TERNROOT + path);
    if (r.ok) document.getElementById("newAgentCode").value = await r.text();
  } catch(e) {}
}
window.loadNewAgentTemplate = loadNewAgentTemplate;

function viewportCenterInCanvas(offX = 0, offY = 0) {
  const wrap = document.getElementById("flow-canvas-wrap");
  if (!wrap) return { x: 0, y: 0 };
  const rect = wrap.getBoundingClientRect();
  const centerX = rect.width / 2 + offX;
  const centerY = rect.height / 2 + offY;
  // Use inverse transform: (viewCoord - pan) / scale
  return { x: (centerX - CT.x) / CT.scale, y: (centerY - CT.y) / CT.scale };
}
window.viewportCenterInCanvas = viewportCenterInCanvas;

function addAgentFromModal() {
  const name = document.getElementById("newAgentName").value.trim() || "Agent";
  const code = document.getElementById("newAgentCode").value;
  const id = "node_" + Date.now();
  const { x: cx, y: cy } = viewportCenterInCanvas((Math.random()-0.5)*100, (Math.random()-0.5)*80);
  createFlowNode(name, "__custom__", cx, cy, 'agent', id);
  // Store code in the node
  const node = flowNodes.find(n => n.id === id);
  if (node) node.props.code = code;
  closeNewAgentModal();
  showToast(`Agent "${name}" added`, "ok");
}
window.addAgentFromModal = addAgentFromModal;

const MOE13_AXES = [
  { id: "safety", label: "Safety", weight: 0.15, crit: true },
  { id: "metasafety", label: "MetaSafety", weight: 0.15, crit: true },
  { id: "logic", label: "Logic", weight: 0.08 },
  { id: "ethics", label: "Ethics", weight: 0.10 },
  { id: "factcheck", label: "FactCheck", weight: 0.08 },
  { id: "causal", label: "Causal", weight: 0.07 },
  { id: "context", label: "Context", weight: 0.07 },
  { id: "history", label: "History", weight: 0.05 },
  { id: "ambiguity", label: "Ambiguity", weight: 0.05 },
  { id: "math", label: "Math", weight: 0.05 },
  { id: "tooluse", label: "ToolUse", weight: 0.05 },
  { id: "persona", label: "Persona", weight: 0.05 },
  { id: "efficiency", label: "Efficiency", weight: 0.05 }
];

/**
 * Bounding-Box Collision Matrix with Magnetic Repulsion.
 * Finds a clear space for a new node, shifting Y recursively if occupied.
 */
function findClearSpace(targetX, targetY, width, height) {
  const padding = 1.1; // 10% spatial padding
  const pw = width * padding;
  const ph = height * padding;
  
  let collision = true;
  let finalY = targetY;
  let attempts = 0;

  while (collision && attempts < 10) {
    collision = false;
    for (const node of flowNodes) {
      const nw = node.type === 'artifact' ? 300 : (node.type === 'moe13' ? 320 : 180);
      const nh = node.type === 'artifact' ? 200 : (node.type === 'moe13' ? 360 : 80);
      
      const horizontalOverlap = Math.abs(targetX - node.x) < (pw + nw) / 2;
      const verticalOverlap = Math.abs(finalY - node.y) < (ph + nh) / 2;
      
      if (horizontalOverlap && verticalOverlap) {
        collision = true;
        finalY += 150; // Recursive magnetic repulsion vector
        break;
      }
    }
    attempts++;
  }
  return { x: targetX, y: finalY };
}
window.findClearSpace = findClearSpace;

/**
 * Frontier Camera: Smoothly pans the viewport to center on a target logical coordinate.
 */
function panToCenter(lx, ly) {
  const wrap = document.getElementById("flow-canvas-wrap");
  if (!wrap) return;
  const fw = wrap.clientWidth;
  const fh = wrap.clientHeight;
  
  // Target CT.x/y such that lx,ly is at center
  const targetX = (fw / 2) - (lx * CT.scale);
  const targetY = (fh / 2) - (ly * CT.scale);
  
  // Smooth Interpolation
  const startX = CT.x;
  const startY = CT.y;
  const duration = 600;
  const startTime = performance.now();

  function step(now) {
    const elapsed = now - startTime;
    const progress = Math.min(elapsed / duration, 1);
    const ease = 1 - Math.pow(1 - progress, 3); // Ease-out cubic
    
    CT.x = startX + (targetX - startX) * ease;
    CT.y = startY + (targetY - startY) * ease;
    
    applyTransform();
    if (progress < 1) requestAnimationFrame(step);
  }
  requestAnimationFrame(step);
}
window.panToCenter = panToCenter;

function createFlowNode(name, path, x, y, type = 'agent', id, isStub = false) {
  const canvas = document.getElementById("flow-canvas");
  const node = document.createElement("div");
  node.id = id;
  const typeClass = type === 'external' ? ' external' : (type === 'gate' ? ' gate' : (type === 'artifact' ? ' artifact' : (type === 'moe13' ? ' moe13' : (type === 'datasource' ? ' datasource' : ''))));
  node.className = "flow-node" + typeClass;
  if (isStub) node.classList.add('artifact-stub');
  if (type === 'datasource') {
    node.style.borderLeft = "4px solid #f43f5e";
    node.style.borderRadius = "0 8px 8px 0";
  }
  
  // Dynamic Sizing
  const nodeW = type === 'artifact' ? 300 : (type === 'moe13' ? 320 : 180);
  const nodeH = type === 'artifact' ? 200 : (type === 'moe13' ? 360 : 80);
  node.style.left = (x - nodeW/2) + "px";
  node.style.top  = (y - nodeH/2) + "px";

  // Requirement 2: Frontier Camera (Auto-pan to new node)
  panToCenter(x, y);

  if (type === 'artifact' || type === 'moe13') {
    node.style.width = nodeW + "px";
    node.style.height = nodeH + "px";
    node.style.display = "flex";
    node.style.flexDirection = "column";
  }

  // Determine Icon & Color
  let icon = 'bot';
  let iconColor = 'var(--cyan)';
  let label = 'AGENT';

  if (type === 'external') {
    icon = 'zap'; iconColor = 'var(--amber)'; label = 'LLM BRIDGE';
  } else if (type === 'gate') {
    icon = 'git-merge'; iconColor = 'var(--cyan)'; label = 'TRIT GATE';
  } else if (type === 'moe13') {
    icon = 'brain-circuit'; iconColor = 'var(--magenta)'; label = 'MOE-13 ORCHESTRATOR';
  } else if (type === 'artifact') {
    icon = 'file-text'; iconColor = 'var(--green)'; label = 'RESULT ARTIFACT';
  } else if (type === 'datasource') {
    icon = 'database'; iconColor = '#f43f5e'; label = 'DATA SOURCE';
  }

  let bodyContent = `
    <div style="font-weight:600; color:var(--text); font-size:13px; margin-bottom:3px;" class="fn-title">${name}</div>
    <div style="font-size:10px; color:var(--muted2); font-family:'JetBrains Mono',monospace;">${path.split('/').pop()}</div>
  `;

  if (type === 'moe13') {
     bodyContent = `
       <div style="font-weight:700; color:var(--magenta); font-size:10px; text-transform:uppercase; margin-bottom:10px; letter-spacing:1px; display:flex; justify-content:space-between;">
         <span>Deliberation Axes</span>
         <span id="moe-verdict-${id}" style="color:var(--muted2)">PENDING</span>
       </div>
       <div style="flex:1; display:grid; grid-template-columns: 1fr 40px 40px; gap:4px; font-family:'JetBrains Mono',monospace; font-size:9px;">
         ${MOE13_AXES.map(axis => `
           <div style="color:var(--text); opacity:0.8;">${axis.label}</div>
           <div id="moe-vote-${id}-${axis.id}" style="text-align:center; color:var(--muted2); font-weight:700;">0</div>
           <div id="moe-conf-${id}-${axis.id}" style="text-align:right; color:var(--cyan);">0%</div>
         `).join('')}
       </div>
       <div id="moe-veto-alert-${id}" style="display:none; margin-top:10px; padding:6px; background:rgba(239,68,68,0.2); border:1px solid var(--red); color:var(--red); font-size:9px; font-weight:800; text-align:center; border-radius:4px;">
         🛑 CRITICAL SAFETY VETO ENGAGED
       </div>
     `;
  }

  if (type === 'artifact') {
     bodyContent = `
       <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:8px;">
         <div style="font-weight:700; color:var(--green); font-size:10px; text-transform:uppercase; letter-spacing:0.5px;">Payload Resolution</div>
         <div class="art-ctrls" style="display:flex; gap:4px;">
           <button class="art-btn" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); setArtifactState('${id}', 'lock')" title="Lock as Static Data"><i data-lucide="lock" style="width:10px"></i></button>
           <button class="art-btn" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); setArtifactState('${id}', 'transmute')" title="Transmute to Editor"><i data-lucide="edit-3" style="width:10px"></i></button>
           <button class="art-btn" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); setArtifactState('${id}', 'extend')" title="Extend Topology"><i data-lucide="external-link" style="width:10px"></i></button>
         </div>
       </div>
       <div id="art-body-${id}" class="art-display" style="flex:1; overflow-y:auto; font-family:'JetBrains Mono',monospace; font-size:11px; color:var(--text); background:rgba(0,0,0,0.2); padding:8px; border-radius:4px; border:1px solid var(--border2); white-space:pre-wrap;">(Awaiting signal...)</div>
       <textarea id="art-edit-${id}" class="art-editor" style="display:none; flex:1; background:var(--bg2); color:var(--cyan); font-family:'JetBrains Mono',monospace; font-size:11px; border:1px solid var(--cyan); padding:8px; border-radius:4px; outline:none; resize:none;" oninput="updateArtifactPayload('${id}', this.value)"></textarea>
       <div id="art-socket-label-${id}" style="margin-top:8px; display:none; justify-content:flex-end;">
         <div style="font-size:9px; color:var(--green); font-weight:800; border:1px solid var(--green); padding:2px 4px; border-radius:3px;">EXTEND SOCKET ACTIVE</div>
       </div>
     `;
  }

  const llmBadge = type === 'external' ? `
    <div style="position:absolute; top:-8px; right:10px; background:var(--amber); color:var(--bg1); font-size:8px; font-weight:800; padding:2px 6px; border-radius:10px; box-shadow:0 2px 8px rgba(245,158,11,0.4); z-index:11;">PROBABILISTIC LLM</div>
  ` : "";

  node.innerHTML = `
    ${llmBadge}
    <div class="fn-head">
      <div style="display:flex; align-items:center; gap:6px; font-size:10px; font-weight:700; color:${iconColor}">
        <i data-lucide="${icon}" style="width:12px"></i>
        ${label}
      </div>
      <div class="fn-status" id="status-${id}" title="idle"></div>
      <button onclick="event.stopPropagation(); traceCausalPath('${id}')" style="padding:2px 4px; background:none; border:none; cursor:pointer; color:var(--cyan); line-height:1; margin-left:4px;" title="Causal Trace">🔍</button>
      <button onclick="event.stopPropagation(); deleteNode('${id}')" style="padding:2px 4px; background:none; border:none; cursor:pointer; color:var(--muted); line-height:1; margin-left:4px;" title="Remove">✕</button>
    </div>
    <div class="fn-body" style="${type === 'artifact' ? 'flex:1; display:flex; flex-direction:column; overflow:hidden;' : ''}">
      ${bodyContent}
    </div>
    <div class="fn-injectors">
      <div class="inj-btn inj-pos" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); injectSignal('${id}', 1)" title="Inject +1 Affirm">+1</div>
      <div class="inj-btn inj-zero" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); injectSignal('${id}', 0)" title="Inject 0 Tend">0</div>
      <div class="inj-btn inj-neg" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); injectSignal('${id}', -1)" title="Inject -1 Reject">-1</div>
    </div>
    ${type !== 'datasource' ? `<div class="flow-port flow-port-in"  style="left:-7px;  top:50%; margin-top:-6px;" title="Input"></div>` : ''}
    <div class="flow-port flow-port-out" style="right:-7px; top:50%; margin-top:-6px;" title="Output"></div>
    ${type === 'artifact' ? '<div class="inspector-resizer" style="width:10px; height:10px;"></div>' : ''}
  `;

  if (type !== 'datasource' && !isStub && flowNodes.find(n => n.id === id)?.props?.customColor) {
     const c = flowNodes.find(n => n.id === id).props.customColor;
     node.style.borderColor = c;
     node.style.boxShadow = `0 0 10px ${c}33`;
     const head = node.querySelector('.fn-head');
     if (head) head.style.borderBottomColor = c;
  }

  node.onmousedown = (e) => {
    if (e.target.closest('button') || e.target.classList.contains('flow-port')) return;
    if (!e.shiftKey && !selectedIds.has(id)) {
      selectNode(id, false);
    } else if (e.shiftKey) {
      selectNode(id, true);
    } else {
      selectedNodeId = id;
      updatePropertyPanel();
    }
    
    isDraggingNode = true;
    nodeDraggingId = id;
    startMouseX = e.clientX; startMouseY = e.clientY;
    
         multiDragOffsets = {};
    selectedIds.forEach(sid => {
      const sel = document.getElementById(sid);
      if (sel) multiDragOffsets[sid] = { x: parseFloat(sel.style.left)||0, y: parseFloat(sel.style.top)||0 };
    });
    node.style.zIndex = 1000;
    e.preventDefault();
  };

  node.ondblclick = (e) => {
    if (type === 'macro') {
      expandMacro(id);
    }
  };

  canvas.appendChild(node);
  const builtinCode = BUILTIN_AGENTS[name] ? BUILTIN_AGENTS[name].code : (path === "__arch__" ? 'fn main() -> trit {\n    return affirm;\n}' : '');
  const defaultProps = {
    system_prompt: "",
    routing: "affirm→next",
    code: builtinCode,
    input_schema: "signal: trit",
    output_schema: "signal: trit"
  };
  
  if (type === 'external') {
    defaultProps.temperature = 0.5;
    defaultProps.max_trits = 1024;
    defaultProps.provider = "openai";
    defaultProps.api_key = "";
    defaultProps.mapping = "classification";
    defaultProps.template = "Evaluate this signal: {{input}}";
  }

  flowNodes.push({ id, name, path, type, x, y, props: defaultProps, isStub });
  const hint = document.getElementById("canvas-hint");
  if (hint) hint.style.display = "none";
  lucide.createIcons();
  saveCanvasState();
}
window.createFlowNode = createFlowNode;

// ─── Multi-select & Node Dragging ─────────────────────────────────────────────
let selectedIds = new Set(); // all selected node IDs
let multiDragOffsets = {}; // id → {ox, oy} initial offsets at drag start
let isDraggingNode = false;
let startMouseX, startMouseY;
let nodeDraggingId = null;

function selectNode(id, addToSelection = false) {
  if (!addToSelection) {
    selectedIds.clear();
    document.querySelectorAll('.flow-node').forEach(n => n.classList.remove('selected','selected-multi'));
  }
  selectedIds.add(id);
  selectedNodeId = id; // primary selection for props panel
  const el = document.getElementById(id);
  if (el) el.classList.add(addToSelection && selectedIds.size > 1 ? 'selected-multi' : 'selected');
  if (addToSelection && selectedIds.size > 1) {
    // Mark primary with selected, rest with selected-multi
    selectedIds.forEach(sid => {
      const sel = document.getElementById(sid);
      if (sel) { sel.classList.remove('selected','selected-multi'); sel.classList.add(sid === id ? 'selected' : 'selected-multi'); }
    });
  }
  
  const gbtn = document.getElementById("groupBtn");
  if (gbtn) gbtn.style.display = selectedIds.size > 1 ? "flex" : "none";

  updatePropertyPanel();
}
window.selectNode = selectNode;

function clearSelection() {
  selectedIds.clear();
  selectedNodeId = null;
  selectedWireId = null;
  document.querySelectorAll('.flow-node').forEach(n => n.classList.remove('selected','selected-multi'));
  document.getElementById("wire-handle").classList.remove("active");
  updateWireStyles();
  const h = document.getElementById('prop-header-label');
  if (h) h.textContent = 'Node Properties';
  
  const gbtn = document.getElementById("groupBtn");
  if (gbtn) gbtn.style.display = "none";

  updatePropertyPanel();
}
window.clearSelection = clearSelection;

function deleteSelected() {
  if (selectedIds.size === 0 && !selectedWireId) return;
  if (selectedWireId) { deleteWire(selectedWireId); return; }
  selectedIds.forEach(id => deleteNode(id));
  selectedIds.clear();
}
window.deleteSelected = deleteSelected;

function syncMultiDragEnd() {
  // After dragging primary node, sync offsets of other selected nodes
  // (they were dragged via the group-move handler below)
}
window.syncMultiDragEnd = syncMultiDragEnd;

function updateValidateBadge({ errors, warnings }) {
  const badge = document.getElementById("validateBadge");
  if (!badge) return;
  const total = errors.length + warnings.length;
  if (total === 0) { badge.style.display = "none"; return; }
  badge.style.display = "flex";
  badge.textContent = total > 9 ? "!" : total;
  badge.style.background = errors.length > 0 ? "var(--red)" : "var(--amber)";
}
window.updateValidateBadge = updateValidateBadge;

function morphNodeArchetype(id, archetype) {
  const node = flowNodes.find(n => n.id === id);
  if (!node) return;
  
  const presets = {
    sensor: { name: "Sensor", code: "fn main() -> trit {\n  let val = read_sensor();\n  return val;\n}", icon: "radio" },
    gate: { name: "Trit Gate", code: "fn main() -> trit {\n  let s = read_input();\n  match s {\n    affirm => { return affirm; }\n    tend   => { return tend; }\n    reject => { return reject; }\n  }\n}", icon: "git-merge" },
    guardrail: { name: "Guardrail", code: "fn main() -> trit {\n  let s = read_input();\n  if s == reject { emit \"VETO\"; return reject; }\n  return s;\n}", icon: "shield-check" },
    deliberator: { name: "Deliberator", code: "fn main() -> trit {\n  // Weighted accumulation logic\n  return truth();\n}", icon: "brain-circuit" }
  };

  const p = presets[archetype];
  if (p) {
    node.name = p.name;
    node.props.code = p.code;
    const el = document.getElementById(id);
    if (el) {
      const title = el.querySelector('.fn-title');
      if (title) title.textContent = p.name;
      const headIcon = el.querySelector('.fn-head i');
      if (headIcon) headIcon.setAttribute('data-lucide', p.icon);
      lucide.createIcons();
    }
    updatePropertyPanel();
    saveCanvasState();
    showToast(`Morphed to ${p.name}`, "ok");
  }
}
window.morphNodeArchetype = morphNodeArchetype;

function collapseArtifactToStub(id) {
  const node = flowNodes.find(n => n.id === id);
  if (!node || node.type !== 'artifact') return;

  node.isStub = !node.isStub; // Toggle state
  const el = document.getElementById(id);
  if (el) {
    if (node.isStub) {
      el.classList.add('artifact-stub');
    } else {
      el.classList.remove('artifact-stub');
    }
    // Re-center ports
    const ports = el.querySelectorAll('.flow-port');
    ports.forEach(p => { p.style.top = "50%"; p.style.marginTop = "-6px"; });
    updateWires();
  }
  if (selectedNodeId === id) updatePropertyPanel();
  saveCanvasState();
}
window.collapseArtifactToStub = collapseArtifactToStub;

function deleteNode(id) {
  flowNodes = flowNodes.filter(n => n.id !== id);
  flowWires = flowWires.filter(w => w.fromId !== id && w.toId !== id);
  const el = document.getElementById(id);
  if (el) el.remove();
  selectedIds.delete(id);
  if (selectedNodeId === id) {
    selectedNodeId = selectedIds.size > 0 ? [...selectedIds][selectedIds.size-1] : null;
    updatePropertyPanel();
  }
  const hint = document.getElementById("canvas-hint");
  if (hint) hint.style.display = flowNodes.length === 0 ? "flex" : "none";
  updateWires();
}
window.deleteNode = deleteNode;

function handleDataSourceFileUpload(nodeId, file) {
  if (!file) return;
  const reader = new FileReader();
  reader.onload = (e) => {
    const content = e.target.result;
    const node = flowNodes.find(n => n.id === nodeId);
    if (node) {
      node.props.payload = content;
      saveCanvasState();
      if (selectedNodeId === nodeId) updatePropertyPanel();
      showToast(`Ingested ${file.name} (${content.length} bytes)`, "ok");
    }
  };
  reader.readAsText(file);
}
window.handleDataSourceFileUpload = handleDataSourceFileUpload;

const ENABLE_NEW_PROPERTIES_UI = true;

const NodePanelController = {
  render(node, bodyEl, inWires, outWires, schemaWarning) {
    if (!ENABLE_NEW_PROPERTIES_UI) {
      this.renderLegacy(node, bodyEl, inWires, outWires, schemaWarning);
    } else {
      if (node.type === 'external') {
        this.renderLLMSemantic(node, bodyEl, inWires, outWires, schemaWarning);
      } else if (node.type === 'artifact') {
        this.renderArtifactSemantic(node, bodyEl, inWires, outWires, schemaWarning);
      } else if (node.type === 'datasource') {
        this.renderDataSourceSemantic(node, bodyEl, inWires, outWires, schemaWarning);
      } else {
        this.renderSemantic(node, bodyEl, inWires, outWires, schemaWarning);
      }
    }
  },

  renderArtifactSemantic(node, bodyEl, inWires, outWires, schemaWarning) {
    const payload = node.props.payload || "(No payload data resolved yet)";
    bodyEl.innerHTML = `
      <div class="prop-section">
        <div class="prop-label-strict">Artifact Identity</div>
        <div style="font-size:13px; font-weight:800; color:var(--green); letter-spacing:0.5px;">${node.name}</div>
      </div>
      <div class="prop-section">
        <div class="prop-label-strict">Evolutionary State</div>
        <div style="font-size:11px; color:var(--text); font-weight:600; background:rgba(255,255,255,0.05); padding:6px; border-radius:4px; border:1px solid var(--border2);">
           ${node.isStub ? 'STUB (DATA CHECKPOINT)' : 'EXPANDED (RESOLUTION CARD)'}
        </div>
      </div>
      <div class="prop-section">
        <div class="prop-label-strict">Payload Memory</div>
        <div style="background:rgba(0,0,0,0.3); border:1px solid var(--border2); padding:10px; border-radius:4px; font-family:'JetBrains Mono',monospace; font-size:11px; white-space:pre-wrap; max-height:280px; overflow-y:auto; color:var(--text); line-height:1.5;">${payload}</div>
      </div>
      <div style="margin-top:auto; padding-top:12px; border-top:1px solid var(--border2); display:flex; flex-direction:column; gap:8px;">
          <button class="btn-pill" style="flex:1; background:var(--bg2);" onclick="collapseArtifactToStub('${node.id}')">
            ${node.isStub ? 'Expand UI (Restore)' : 'Collapse to Checkpoint'}
          </button>
          <button class="btn-pill" style="flex:1; background:rgba(239,68,68,0.1); color:var(--red);" onclick="deleteNode('${node.id}')">Purge Data</button>
      </div>
    `;
    lucide.createIcons();
  },

  renderDataSourceSemantic(node, bodyEl, inWires, outWires, schemaWarning) {
    const dataType = node.props.data_type || "text";
    const payload = node.props.payload || "";

    bodyEl.innerHTML = `
      <div class="prop-section">
        <div class="prop-label-strict">Node Name</div>
        <input type="text" class="prop-input" value="${node.name}" oninput="updateNodeProp('name', this.value)">
      </div>
      <div class="prop-section">
        <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:4px;">
          <div class="prop-label-strict" style="margin-bottom:0;">Data Source Configuration</div>
          <button class="btn btn-primary" style="font-size:10px; padding:2px 8px; height:24px; background:var(--blue); border:none;" onclick="document.getElementById('datasource-file-input').click()">
            <i data-lucide="upload" style="width:12px; margin-right:4px;"></i> Upload File
          </button>
          <input type="file" id="datasource-file-input" style="display:none;" accept=".txt,.md,.csv,.json" onchange="handleDataSourceFileUpload('${node.id}', this.files[0])">
        </div>
        <div class="prop-label-strict">Data Type</div>
        <select class="prop-input" onchange="updateNodeProp('data_type', this.value)">
          <option value="text" ${dataType === 'text' ? 'selected' : ''}>Raw Text</option>
          <option value="json" ${dataType === 'json' ? 'selected' : ''}>JSON Array</option>
          <option value="csv" ${dataType === 'csv' ? 'selected' : ''}>CSV Data</option>
          <option value="markdown" ${dataType === 'markdown' ? 'selected' : ''}>Markdown</option>
        </select>
      </div>
      <div class="prop-section" style="flex:1; display:flex; flex-direction:column;">
        <div class="prop-label-strict">Payload Injector (Semantic Data)</div>
        <textarea class="prop-input" style="flex:1; resize:vertical; min-height:200px; font-family:'JetBrains Mono', monospace; font-size:12px; line-height:1.4;" placeholder="Paste raw semantic payload here..." oninput="updateNodeProp('payload', this.value)">${payload}</textarea>
        <div style="font-size:10px; color:var(--muted2); margin-top:4px;">Injected directly into downstream runtime buffer on Simulation.</div>
      </div>
    `;
    lucide.createIcons();
  },

  renderLegacy(node, bodyEl, inWires, outWires, schemaWarning) {
    const routing = node.props.routing ?? "affirm→next";
    const code = (node.props.code || "").replace(/</g,"&lt;").replace(/>/g,"&gt;");
    const inSchema  = (node.props.input_schema  || "").replace(/</g,"&lt;").replace(/>/g,"&gt;");
    const outSchema = (node.props.output_schema || "").replace(/</g,"&lt;").replace(/>/g,"&gt;");

    bodyEl.innerHTML = `
      <div class="prop-group">
        <label class="prop-label">Node Name</label>
        <input type="text" class="prop-input" value="${node.name}" oninput="updateNodeProp('name', this.value)">
      </div>
      <div class="prop-group">
        <label class="prop-label">System Instructions</label>
        <textarea class="prop-input" style="height:70px; resize:vertical;" oninput="updateNodeProp('system_prompt', this.value)">${node.props.system_prompt || ""}</textarea>
      </div>
      <div class="prop-group">
        <label class="prop-label">Output Routing Rule</label>
        <select class="prop-input" style="width:100%" onchange="updateNodeProp('routing', this.value)">
          <option value="affirm→next" ${routing==="affirm→next"?"selected":""}>affirm → pass to next</option>
          <option value="tend→next" ${routing==="tend→next"?"selected":""}>tend → pass to next</option>
          <option value="all→next" ${routing==="all→next"?"selected":""}>all trits → pass to next</option>
          <option value="affirm→branch" ${routing==="affirm→branch"?"selected":""}>affirm → branch A, else B</option>
          <option value="reject→stop" ${routing==="reject→stop"?"selected":""}>reject → halt flow</option>
        </select>
      </div>
      <div class="prop-group">
        <label class="prop-label" style="display:flex;justify-content:space-between;align-items:center;">
          <span>I/O Contract</span>
          <span style="font-size:9px;color:var(--muted2);font-weight:400;text-transform:none;">${inWires.length} in · ${outWires.length} out</span>
        </label>
        ${schemaWarning}
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:6px;">
          <div>
            <div style="font-size:9px;color:var(--green);font-weight:600;margin-bottom:3px;">▶ INPUT</div>
            <input class="prop-input" style="width:100%;font-size:10px;font-family:'JetBrains Mono',monospace;" value="${inSchema}"
              placeholder="signal: trit" oninput="updateNodeProp('input_schema',this.value);updateNodeSchemaDisplay('${node.id}')">
          </div>
          <div>
            <div style="font-size:9px;color:var(--cyan);font-weight:600;margin-bottom:3px;">◀ OUTPUT</div>
            <input class="prop-input" style="width:100%;font-size:10px;font-family:'JetBrains Mono',monospace;" value="${outSchema}"
              placeholder="result: trit" oninput="updateNodeProp('output_schema',this.value);updateNodeSchemaDisplay('${node.id}')">
          </div>
        </div>
      </div>
      <div class="prop-group">
        <label class="prop-label">Agent Code (.tern)</label>
        <textarea class="prop-input" style="height:100px; resize:vertical; font-family:'JetBrains Mono',monospace; font-size:10px;"
          oninput="updateNodeProp('code', this.value)">${code}</textarea>
      </div>
      <div style="display:flex;gap:6px;margin-top:4px;">
        <button class="btn btn-ghost" style="flex:1;font-size:11px;" onclick="openAgentInEditor()">Open in Editor</button>
        <button class="btn btn-ghost" style="flex:1;font-size:11px;color:var(--red);" onclick="deleteNode('${node.id}')">Remove</button>
      </div>
    `;
  },

  renderSemantic(node, bodyEl, inWires, outWires, schemaWarning) {
    const intent = node.props.system_prompt || node.props.intent || "";
    const inSchema  = (node.props.input_schema  || "").replace(/</g,"&lt;").replace(/>/g,"&gt;");
    const outSchema = (node.props.output_schema || "").replace(/</g,"&lt;").replace(/>/g,"&gt;");
    const timeout = node.props.timeout ?? 5000;
    const retries = node.props.retries ?? 3;
    const target = node.props.execution_target ?? "local";
    const customColor = node.props.customColor || "#38bdf8";

    bodyEl.innerHTML = `
      <div class="prop-section">
        <div class="prop-label-strict">Node Identity</div>
        <input type="text" class="prop-input-strict" style="width:100%" value="${node.name}" oninput="updateNodeProp('name', this.value)">
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Operational Intent</div>
        <textarea class="prop-input-strict" style="height:50px; resize:vertical; font-size:11px;" placeholder="Define high-level objective..." oninput="updateNodeProp('system_prompt', this.value)">${intent}</textarea>
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Runtime Constraints</div>
        <div style="display:grid; grid-template-columns:1fr 1fr; gap:8px; margin-bottom:8px;">
          <div>
            <div class="prop-label-strict" style="font-size:9px;">Timeout (ms)</div>
            <input type="number" class="prop-input-strict" value="${timeout}" oninput="updateNodeProp('timeout', parseInt(this.value))">
          </div>
          <div>
            <div class="prop-label-strict" style="font-size:9px;">Max Retries</div>
            <input type="number" class="prop-input-strict" value="${retries}" oninput="updateNodeProp('retries', parseInt(this.value))">
          </div>
        </div>
        <div class="prop-label-strict" style="font-size:9px;">Execution Target</div>
        <select class="prop-input-strict" onchange="updateNodeProp('execution_target', this.value)">
          <option value="local" ${target==='local'?'selected':''}>Local VM (Albert)</option>
          <option value="remote" ${target==='remote'?'selected':''}>Remote Proxy (API)</option>
        </select>
      </div>

      <div class="prop-section">
        <div class="prop-label-strict" style="display:flex;justify-content:space-between;">
          <span>I/O Contract</span>
          <span style="font-size:9px;color:var(--muted2);">${inWires.length} IN / ${outWires.length} OUT</span>
        </div>
        ${schemaWarning}
        <div style="display:flex; flex-direction:column; gap:8px;">
          <input class="prop-input-strict" style="font-family:'JetBrains Mono',monospace; font-size:11px;" value="${inSchema}" placeholder="Input Schema" oninput="updateNodeProp('input_schema',this.value);updateNodeSchemaDisplay('${node.id}')">
          <input class="prop-input-strict" style="font-family:'JetBrains Mono',monospace; font-size:11px;" value="${outSchema}" placeholder="Output Schema" oninput="updateNodeProp('output_schema',this.value);updateNodeSchemaDisplay('${node.id}')">
        </div>
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Logic / Source</div>
        <textarea class="prop-input-strict" style="height:80px; resize:vertical; font-family:'JetBrains Mono',monospace; font-size:11px;"
          oninput="updateNodeProp('code', this.value)">${node.props.code || ""}</textarea>
      </div>

      <div class="prop-section" style="margin-top:auto; padding-top:12px; border-top:1px solid var(--border2);">
        <div class="prop-label-strict">Morph Archetype</div>
        <select class="prop-input-strict" style="font-size:11px; font-weight:700; color:var(--cyan); margin-bottom:12px;" onchange="morphNodeArchetype('${node.id}', this.value)">
          <option value="">-- Generic Agent --</option>
          <option value="sensor">SENSOR (Input Capture)</option>
          <option value="gate">GATE (Ternary Logic)</option>
          <option value="guardrail">GUARDRAIL (Veto Logic)</option>
          <option value="deliberator">DELIBERATOR (Weighted)</option>
        </select>

        <div style="display:flex; flex-direction:column; gap:4px; margin-bottom:12px;">
          <div style="color:white; font-weight:bold; font-size:10px; text-transform:uppercase; letter-spacing:0.05em;">Custom Color</div>
          <div style="display:flex; align-items:center; gap:6px;">
            <input type="color" class="prop-input-strict" style="width:34px; padding:0; border:1px solid var(--border2); height:24px; cursor:pointer; background:none; border-radius:4px;" value="${customColor}" oninput="updateNodeColor('${node.id}', this.value); updatePropertyPanel()" title="Custom Color">
            <code style="font-size:11px; color:var(--text); font-family:'JetBrains Mono',monospace; opacity:0.8; letter-spacing:0.5px;">${customColor.toUpperCase()}</code>
          </div>
        </div>
        <div style="display:flex; gap:6px;">
          <button class="btn-pill" style="flex:1; background:var(--bg2);" onclick="openAgentInEditor()">Editor</button>
          <button class="btn-pill" style="flex:1; background:rgba(239,68,68,0.1); color:var(--red);" onclick="deleteNode('${node.id}')">Remove</button>
        </div>
      </div>
    `;
    lucide.createIcons();
  },

  renderLLMSemantic(node, bodyEl, inWires, outWires, schemaWarning) {
    const intent = node.props.system_prompt || "";
    const protocol = node.props.protocol || "openai";
    const modelId = node.props.model_id || "";
    const baseUrl = node.props.base_url || "";
    const temp = node.props.temperature ?? 0.5;
    const tokens = node.props.max_trits ?? 1024;
    const timeout = node.props.timeout ?? 10000; // LLM nodes usually need more time
    const retries = node.props.retries ?? 2;
    const customColor = node.props.customColor || "#f59e0b";
    
    // Pull from vault if not set or when protocol changes
    const secrets = getTernflowSecrets();
    const apiKey = node.props.api_key || secrets[protocol] || "";
    
    // Auto-populate node prop if it was empty
    if (apiKey && !node.props.api_key) node.props.api_key = apiKey;

    const showBaseUrl = (protocol === "openai" || protocol === "webhook");

    bodyEl.innerHTML = `
      <div class="prop-section">
        <div class="prop-label-strict">Bridge Identity</div>
        <input type="text" class="prop-input-strict" style="width:100%" value="${node.name}" oninput="updateNodeProp('name', this.value)">
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Intelligence Strategy</div>
        <textarea class="prop-input-strict" style="height:50px; resize:vertical; font-size:11px;" placeholder="System prompt for the bridge..." oninput="updateNodeProp('system_prompt', this.value)">${intent}</textarea>
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Runtime Constraints</div>
        <div style="display:grid; grid-template-columns:1fr 1fr; gap:8px;">
          <div>
            <div class="prop-label-strict" style="font-size:9px;">Timeout (ms)</div>
            <input type="number" class="prop-input-strict" value="${timeout}" oninput="updateNodeProp('timeout', parseInt(this.value))">
          </div>
          <div>
            <div class="prop-label-strict" style="font-size:9px;">Max Retries</div>
            <input type="number" class="prop-input-strict" value="${retries}" oninput="updateNodeProp('retries', parseInt(this.value))">
          </div>
        </div>
      </div>

      <div class="prop-section" style="border:1px solid var(--amber); padding:8px; border-radius:4px; background:rgba(245,158,11,0.05);">
        <div class="prop-label-strict" style="color:var(--amber);">Decision Mode</div>
        <div style="font-size:11px; font-weight:700; display:flex; align-items:center; gap:6px;">
          <i data-lucide="zap" style="width:14px"></i> PROBABILISTIC (LLM BRIDGE)
        </div>
      </div>

      <div class="prop-section" style="margin-top:8px;">
        <div class="prop-label-strict">Routing Configuration</div>
        
        <div style="margin-bottom:8px;">
          <div class="prop-label-strict" style="font-size:9px; color:var(--muted2);">Transport Protocol</div>
          <select class="prop-input-strict" onchange="updateBridgeProtocol('${node.id}', this.value)">
            <option value="openai" ${protocol==='openai'?'selected':''}>OpenAI-Compatible REST</option>
            <option value="anthropic" ${protocol==='anthropic'?'selected':''}>Anthropic Native</option>
            <option value="google" ${protocol==='google'?'selected':''}>Google Native</option>
            <option value="webhook" ${protocol==='webhook'?'selected':''}>Custom Webhook</option>
            <option value="mcp" ${protocol==='mcp'?'selected':''}>MCP (Model Context Protocol)</option>
          </select>
        </div>

        <div style="margin-bottom:8px;">
          <div class="prop-label-strict" style="font-size:9px; color:var(--muted2);">Target Model ID</div>
          <input type="text" class="prop-input-strict" value="${modelId}" placeholder="e.g. grok-3 or claude-3-7-sonnet" oninput="updateNodeProp('model_id', this.value)">
        </div>

        ${showBaseUrl ? `
        <div style="margin-bottom:8px;">
          <div class="prop-label-strict" style="font-size:9px; color:var(--muted2);">Base URL</div>
          <input type="text" class="prop-input-strict" value="${baseUrl}" placeholder="https://api.openai.com/v1" oninput="updateNodeProp('base_url', this.value)">
        </div>
        ` : ''}

        <div style="margin-bottom:8px;">
          <div class="prop-label-strict" style="font-size:9px; color:var(--muted2);">Provider API Key</div>
          <input type="password" class="prop-input-strict" value="${apiKey}" placeholder="Linked to ${protocol} vault" oninput="updateNodeProp('api_key', this.value)">
        </div>
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Hyperparameters</div>
        <div style="display:grid; grid-template-columns:1fr 1fr; gap:8px;">
          <div>
            <div style="font-size:9px; color:var(--muted2); margin-bottom:2px;">TEMP: ${temp}</div>
            <input type="range" min="0" max="1" step="0.1" value="${temp}" style="width:100%" oninput="updateNodeProp('temperature', parseFloat(this.value)); updatePropertyPanel()">
          </div>
          <div>
            <div style="font-size:9px; color:var(--muted2); margin-bottom:2px;">TOKENS: ${tokens}</div>
            <input type="range" min="128" max="4096" step="128" value="${tokens}" style="width:100%" oninput="updateNodeProp('max_trits', parseInt(this.value)); updatePropertyPanel()">
          </div>
        </div>
      </div>

      <div style="margin-top:auto; padding-top:12px; border-top:1px solid var(--border2); display:flex; flex-direction:column; gap:12px;">
        <div style="display:flex; flex-direction:column; gap:4px;">
          <div style="color:white; font-weight:bold; font-size:10px; text-transform:uppercase; letter-spacing:0.05em;">Custom Color</div>
          <div style="display:flex; align-items:center; gap:6px;">
            <input type="color" class="prop-input-strict" style="width:34px; padding:0; border:1px solid var(--border2); height:24px; cursor:pointer; background:none; border-radius:4px;" value="${customColor}" oninput="updateNodeColor('${node.id}', this.value); updatePropertyPanel()" title="Custom Color">
            <code style="font-size:11px; color:var(--text); font-family:'JetBrains Mono',monospace; opacity:0.8; letter-spacing:0.5px;">${customColor.toUpperCase()}</code>
          </div>
        </div>
        <div style="display:flex; gap:6px;">
          <button class="btn-pill" style="flex:1; background:var(--bg2);" onclick="openAgentInEditor()">Editor</button>
          <button class="btn-pill" style="flex:1; background:rgba(239,68,68,0.1); color:var(--red);" onclick="deleteNode('${node.id}')">Remove</button>
        </div>
      </div>
    `;
    lucide.createIcons();
  },

  updateInlineSummary(node, intent) {
    const titleEl = document.querySelector(`#${node.id} .fn-title`);
    if (titleEl && !titleEl.dataset.semantic) {
      titleEl.dataset.semantic = "true";
      const desc = document.createElement("div");
      desc.className = "semantic-desc";
      desc.style = "font-size:9px; color:var(--muted2); font-weight:400; max-height:24px; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; margin-top:2px;";
      titleEl.parentNode.insertBefore(desc, titleEl.nextSibling);
    }
    const descEl = document.querySelector(`#${node.id} .semantic-desc`);
    if (descEl) descEl.textContent = intent ? `→ ${intent}` : (node.type==='external' ? "→ Waiting for prompt..." : "→ (No intent defined)");
  }
};

const EdgePanelController = {
  render(wire, bodyEl, fromNode, toNode) {
    if (!ENABLE_NEW_PROPERTIES_UI) {
      this.renderLegacy(wire, bodyEl, fromNode, toNode);
    } else {
      this.renderSemantic(wire, bodyEl, fromNode, toNode);
    }
  },

  renderLegacy(wire, bodyEl, fromNode, toNode) {
    const cond      = wire.condition  || "all";
    const transform = wire.transform  || "pass";
    const label     = wire.label      || "";

    bodyEl.innerHTML = `
      <div style="font-size:10px;color:var(--muted2);margin-bottom:4px;">
        ${fromNode ? fromNode.name : "?"} → ${toNode ? toNode.name : "?"}
      </div>

      <div class="prop-group">
        <label class="prop-label">Pass condition</label>
        <select class="prop-input" style="width:100%" onchange="updateWireProp('condition', this.value)">
          <option value="all"    ${cond==="all"    ?"selected":""}>All trits (pass everything)</option>
          <option value="affirm" ${cond==="affirm" ?"selected":""}>affirm only (+1)</option>
          <option value="tend"   ${cond==="tend"   ?"selected":""}>tend only (0)</option>
          <option value="reject" ${cond==="reject" ?"selected":""}>reject only (-1)</option>
          <option value="!reject"${cond==="!reject"?"selected":""}>affirm or tend (not reject)</option>
          <option value="!tend"  ${cond==="!tend"  ?"selected":""}>affirm or reject (decisive)</option>
        </select>
      </div>

      <div class="prop-group">
        <label class="prop-label">On condition fail</label>
        <select class="prop-input" style="width:100%" onchange="updateWireProp('transform', this.value)">
          <option value="pass"  ${transform==="pass"  ?"selected":""}>Pass anyway (ignore condition)</option>
          <option value="block" ${transform==="block" ?"selected":""}>Block — drop signal</option>
          <option value="flip"  ${transform==="flip"  ?"selected":""}>Flip to reject</option>
          <option value="hold"  ${transform==="hold"  ?"selected":""}>Force to tend (hold)</option>
        </select>
      </div>

      <div class="prop-group">
        <label class="prop-label">Edge label</label>
        <input type="text" class="prop-input" value="${label}" placeholder="e.g. conf > 0.7"
          oninput="updateWireProp('label', this.value)">
      </div>

      <div class="prop-group">
        <label class="prop-label">Priority weight</label>
        <input type="range" min="1" max="10" step="1" value="${wire.priority || 5}" style="width:100%"
          oninput="updateWireProp('priority', parseInt(this.value)); this.nextElementSibling.textContent=this.value">
        <span style="font-size:10px;color:var(--cyan)">${wire.priority || 5}</span>
      </div>

      <div class="prop-group" style="display:flex; align-items:center; gap:8px;">
        <input type="checkbox" id="wire-feedback" ${wire.isFeedback ? "checked" : ""} 
          onchange="updateWireProp('isFeedback', this.checked)">
        <label for="wire-feedback" class="prop-label" style="margin:0; cursor:pointer;">Feedback loop (Bypass cycle check)</label>
      </div>

      <div style="margin-top:12px;">
        <button class="btn btn-ghost" style="width:100%;font-size:11px;color:var(--red)" onclick="deleteWire('${wire.id}')">Remove Edge</button>
      </div>
    `;
  },

  renderSemantic(wire, bodyEl, fromNode, toNode) {
    const cond      = wire.condition  || "all";
    const transform = wire.transform  || "pass";
    const priority  = wire.priority   || 5;
    const weight    = wire.weight     || 1.0;
    const latency   = wire.latency    || 0;
    const customColor = wire.customColor || "#94a3b8";

    bodyEl.innerHTML = `
      <div style="font-size:10px; color:var(--muted2); margin-bottom:8px; font-weight:600; display:flex; align-items:center; gap:4px;">
        <span style="color:var(--text)">${fromNode ? fromNode.name : "?"}</span> 
        <i data-lucide="arrow-right" style="width:10px; height:10px; color:var(--cyan)"></i> 
        <span style="color:var(--text)">${toNode ? toNode.name : "?"}</span>
        <div style="flex:1"></div>
      </div>

      <!-- Condition Rail -->
      <div class="prop-group" style="margin-bottom:8px;">
        <div class="prop-label-strict" style="font-size:10px; margin-bottom:4px;">Activation Logic</div>
        <div style="display:flex; align-items:stretch; height:24px; background:var(--bg2); border:1px solid var(--border2); border-radius:4px; overflow:hidden;">
          <button title="value == +1" class="rail-btn ${cond==='affirm'?'active':''}" onclick="updateWireProp('condition','affirm');updateWireProp('label','+1 only');">+1</button>
          <button title="value == 0"  class="rail-btn ${cond==='tend'?'active':''}"   onclick="updateWireProp('condition','tend');updateWireProp('label','0 only');">0</button>
          <button title="value == -1" class="rail-btn ${cond==='reject'?'active':''}" onclick="updateWireProp('condition','reject');updateWireProp('label','-1 only');">-1</button>
          <button title="value != -1" class="rail-btn ${cond==='!reject'?'active':''}" onclick="updateWireProp('condition','!reject');updateWireProp('label','+1 or 0');">!= -1</button>
          <button title="All signals" class="rail-btn ${cond==='all'?'active':''}"    onclick="updateWireProp('condition','all');updateWireProp('label','All signals');">ALL</button>
        </div>
      </div>

      <div style="display:grid; grid-template-columns: 1fr 1fr; gap:8px; margin-bottom:8px;">
        <div class="prop-group" style="margin:0;">
          <div class="prop-label-strict" style="font-size:10px; margin-bottom:4px;">On Fail</div>
          <div style="display:flex; flex-direction:column; gap:2px; font-size:9px; line-height:1.2;">
            <label style="display:flex; align-items:center; gap:4px; cursor:pointer;"><input type="radio" name="wfail" value="block" ${transform==='block'||transform==='pass'?'checked':''} onchange="updateWireProp('transform','block')"> Drop</label>
            <label style="display:flex; align-items:center; gap:4px; cursor:pointer;"><input type="radio" name="wfail" value="flip" ${transform==='flip'?'checked':''} onchange="updateWireProp('transform','flip')"> Fallback</label>
            <label style="display:flex; align-items:center; gap:4px; cursor:pointer;"><input type="radio" name="wfail" value="hold" ${transform==='hold'?'checked':''} onchange="updateWireProp('transform','hold')"> Hold (0)</label>
          </div>
        </div>
        <div class="prop-group" style="margin:0;">
          <div class="prop-label-strict" style="font-size:10px; margin-bottom:4px;">Temporal Dynamics</div>
          <div style="display:flex; flex-direction:column; gap:4px;">
            <div>
               <div style="font-size:8px; color:var(--muted2)">Signal Weight</div>
               <input type="number" step="0.1" class="prop-input-strict" style="height:20px; font-size:9px; padding:2px 4px;" value="${weight}" oninput="updateWireProp('weight', parseFloat(this.value))">
            </div>
            <div>
               <div style="font-size:8px; color:var(--muted2)">Latency (ms)</div>
               <input type="number" class="prop-input-strict" style="height:20px; font-size:9px; padding:2px 4px;" value="${latency}" oninput="updateWireProp('latency', parseInt(this.value))">
            </div>
          </div>
        </div>
      </div>

      <div class="prop-group" style="margin-bottom:8px;">
        <div class="prop-label-strict" style="font-size:10px; margin-bottom:4px;">Priority</div>
        <div style="display:flex; height:24px; border-radius:4px; overflow:hidden; border:1px solid var(--border2); background:var(--bg2);">
          <button class="rail-btn ${priority<=2 ? 'active' : ''}" style="flex:1; font-size:9px;" onclick="updateWireProp('priority', 2); updateEdgePanel();">Low</button>
          <button class="rail-btn ${(priority>2 && priority<10) ? 'active' : ''}" style="flex:1; font-size:9px;" onclick="updateWireProp('priority', 5); updateEdgePanel();">Norm</button>
          <button class="rail-btn ${priority>=10 ? 'active' : ''}" style="flex:1; font-size:9px;" onclick="updateWireProp('priority', 10); updateEdgePanel();">High</button>
        </div>
      </div>

      <div class="prop-group" style="margin-bottom:8px;">
        <div class="prop-label-strict" style="font-size:10px; margin-bottom:4px;">Custom Label</div>
        <input type="text" class="prop-input-strict" style="font-size:11px; height:24px;" value="${wire.label || ''}" placeholder="e.g. data_stream"
          oninput="updateWireProp('label', this.value)">
      </div>

      <div class="prop-group" style="display:flex; align-items:center; gap:6px; margin-bottom:8px;">
        <input type="checkbox" id="wire-feedback-sem" style="width:12px; height:12px;" ${wire.isFeedback ? "checked" : ""} onchange="updateWireProp('isFeedback', this.checked)">
        <label for="wire-feedback-sem" class="prop-label-strict" style="margin:0; font-size:10px; cursor:pointer;">Feedback Loop</label>
      </div>

      <div class="prop-group" style="display:flex; align-items:center; justify-content:space-between; background:var(--bg2); padding:6px; border-radius:4px; border:1px solid var(--border2); margin-top:auto;">
        <div class="prop-label-strict" style="font-size:10px; margin:0;">Custom Wire Color</div>
        <div style="display:flex; align-items:center; gap:8px;">
          <code style="font-size:10px; color:var(--muted2);">${customColor}</code>
          <input type="color" class="prop-input-strict" style="width:24px; padding:0; border:none; height:20px; cursor:pointer; background:none;" value="${customColor}" oninput="updateWireColor('${wire.id}', this.value); updatePropertyPanel()" title="Custom Wire Color">
        </div>
      </div>

      <div style="padding-top:8px;">
        <button class="btn btn-ghost" style="width:100%; height:28px; font-size:11px; color:var(--red); border:1px solid rgba(239, 68, 68, 0.1);" onclick="deleteWire('${wire.id}')">Remove Edge</button>
      </div>
    `;
    lucide.createIcons();
  }
};

function showHelpCard() {
  const popover = document.getElementById("helpPopover");
  const title   = document.getElementById("helpPopoverTitle");
  const content = document.getElementById("helpPopoverContent");
  const header  = document.getElementById("prop-header-label");

  if (!popover || !title || !content || !header) return;

  const isEdge = header.textContent.toLowerCase().includes("edge");
  
  if (isEdge) {
    title.innerHTML = `<i data-lucide="git-branch" style="width:16px; color:var(--amber)"></i> What is an Edge?`;
    content.innerHTML = `
      <p style="margin-bottom:12px;">Think of an Edge as a <b>smart pipe</b> connecting your workers. It does not just carry information; it acts like a bouncer at a door.</p>
      <div style="margin-bottom:12px;">
        <b style="color:var(--green); display:block; margin-bottom:4px;">Activation Logic</b>
        You tell the bouncer who gets through. Setting it to '+1' means only absolute 'Yes' answers pass. Setting it to '!= -1' means 'Yes' and 'I am not sure' can pass, but a hard 'No' gets blocked.
      </div>
      <div>
        <b style="color:var(--red); display:block; margin-bottom:4px;">On Fail</b>
        If a signal is blocked, what happens? 'Drop' throws it in the trash. 'Fallback' sends it to a backup plan. 'Hold (0)' turns it into a neutral 'I don't know' state so the system keeps moving without crashing.
      </div>
    `;
  } else {
    title.innerHTML = `<i data-lucide="component" style="width:16px; color:var(--cyan)"></i> What is a Node?`;
    content.innerHTML = `
      <p style="margin-bottom:12px;">Think of a Node as a <b>mini-worker</b> or a tiny brain in your network. It receives a task, thinks about it, and spits out a ternary answer: Yes (+1), I am not sure (0), or No (-1).</p>
      <p>Here you give this worker its instructions. You tell it where to do its thinking (like your local Albert VM) and give it safety rules, like 'do not take longer than 5 seconds' so it does not freeze your whole system.</p>
    `;
  }
  
  popover.style.display = "block";
  if (window.lucide) lucide.createIcons();
}
window.showHelpCard = showHelpCard;

function closeHelpCard() {
  const popover = document.getElementById("helpPopover");
  if (popover) popover.style.display = "none";
}
window.closeHelpCard = closeHelpCard;

function updatePropertyPanel() {
  const body = document.getElementById("prop-body");
  const header = document.getElementById("prop-header-label");
  const help = document.getElementById("prop-help-icon");

  if (!selectedNodeId) {
    if (header) header.textContent = "Node Properties";
    if (help) help.style.display = "flex";
    body.innerHTML = `
      <div style="color:var(--muted); font-size:12px; text-align:center; margin-top:40px; padding:0 12px; line-height:1.8;">
        Select a node to configure<br>
        <span style="font-size:10px; color:var(--muted2);">Drag output port → input port to wire agents</span>
      </div>`;
    return;
  }

  const node = flowNodes.find(n => n.id === selectedNodeId);
  if (!node) return;
  if (header) header.textContent = node.type === 'macro' ? "MACRO PROPERTIES" : "NODE PROPERTIES";
  if (help) help.style.display = "flex";
  const inWires  = flowWires.filter(w => w.toId   === selectedNodeId);
  const outWires = flowWires.filter(w => w.fromId === selectedNodeId);
  const schemaWarning = inWires.some(w => {
    const src = flowNodes.find(n => n.id === w.fromId);
    return src && src.props.output_schema && node.props.input_schema && src.props.output_schema !== node.props.input_schema;
  }) ? `<div style="font-size:10px;color:var(--amber);padding:6px 0;">⚠ Schema mismatch on incoming wire</div>` : "";

  NodePanelController.render(node, body, inWires, outWires, schemaWarning);
}

function updateNodeColor(id, color) {
  const node = flowNodes.find(n => n.id === id);
  if (!node) return;
  node.props.customColor = color;
  const el = document.getElementById(id);
  if (el) {
    el.style.borderColor = color;
    el.style.boxShadow = `0 0 10px ${color}33`; // 20% alpha shadow
    const head = el.querySelector('.fn-head');
    if (head) head.style.borderBottomColor = color;
  }
  saveCanvasState();
}
window.updateNodeColor = updateNodeColor;

function updateWireColor(id, color) {
  const wire = flowWires.find(w => w.id === id);
  if (!wire) return;
  wire.customColor = color;
  const path = document.querySelector(`path[id="${id}"]`);
  if (path) path.style.stroke = color;
  saveCanvasState();
}
window.updateWireColor = updateWireColor;
window.updatePropertyPanel = updatePropertyPanel;

function openAgentInEditor() {
  const node = flowNodes.find(n => n.id === selectedNodeId);
  if (!node) return;
  const path = `flow/${node.name.replace(/\s+/g,'_')}.tern`;
  const code = node.props.code || `fn main() -> trit {\n    // ${node.name}\n    return affirm;\n}`;
  fileBuffers[path] = code;
  loadToEditor(path, code);
  switchView('editor');
}
window.openAgentInEditor = openAgentInEditor;

function updateNodeSchemaDisplay(nodeId) {
  const node = flowNodes.find(n => n.id === nodeId);
  if (!node) return;
  const el = document.getElementById(nodeId);
  if (!el) return;
  let schema = el.querySelector('.fn-schema');
  if (!schema) { schema = document.createElement('div'); schema.className = 'fn-schema'; el.querySelector('.fn-body').appendChild(schema); }
  const ins  = node.props.input_schema  ? `<span style="color:var(--green)">▶ ${node.props.input_schema}</span>` : '';
  const outs = node.props.output_schema ? `<span style="color:var(--cyan)">◀ ${node.props.output_schema}</span>` : '';
  schema.innerHTML = [ins, outs].filter(Boolean).join('<br>');
}
window.updateNodeSchemaDisplay = updateNodeSchemaDisplay;

function updateNodeProp(key, val) {
  const node = flowNodes.find(n => n.id === selectedNodeId);
  if (!node) return;
  if (key === 'name') {
    node.name = val;
    const titleEl = document.querySelector(`#${node.id} .fn-title`);
    if (titleEl) titleEl.textContent = val;
  } else {
    node.props[key] = val;
  }

  // Bidirectional Secret Sync: Push mutation to global vault
  if (node.type === 'external' && key === 'api_key') {
    const protocol = node.props.protocol || 'openai';
    setTernflowSecret(protocol, val);
  }

  saveCanvasState();
}
window.updateNodeProp = updateNodeProp;

function updateBridgeProtocol(nodeId, protocol) {
  const node = flowNodes.find(n => n.id === nodeId);
  if (!node) return;
  node.props.protocol = protocol;
  
  // Dynamically pull key from vault for the new protocol
  const secrets = getTernflowSecrets();
  node.props.api_key = secrets[protocol] || "";
  
  updatePropertyPanel(); // Refresh UI to show correct key and conditional fields
  saveCanvasState();
}
window.updateBridgeProtocol = updateBridgeProtocol;

function addExternalBridge() {
  const id = "bridge_" + Date.now();
  const pos = viewportCenterInCanvas((Math.random()-0.5)*100, (Math.random()-0.5)*80);
  createFlowNode("LLM Bridge", "external", pos.x, pos.y, 'external', id);
}
window.addExternalBridge = addExternalBridge;

function addTernaryGate() {
  const id = "gate_" + Date.now();
  const pos = viewportCenterInCanvas((Math.random()-0.5)*100, (Math.random()-0.5)*80);
  createFlowNode("Consensus Gate", "gate", pos.x, pos.y, 'gate', id);
}
window.addTernaryGate = addTernaryGate;

function addDataSource() {
  const id = "data_" + Date.now();
  const pos = viewportCenterInCanvas((Math.random()-0.5)*100, (Math.random()-0.5)*80);
  createFlowNode("Data Source", "source", pos.x, pos.y, 'datasource', id);
}
window.addDataSource = addDataSource;

// ─── Library tab switching ────────────────────────────────────────────────────
function switchLibTab(tab) {
  document.querySelectorAll('.lib-tab').forEach(t => t.classList.remove('active'));
  const active = document.getElementById('libtab-' + tab);
  if (active) active.classList.add('active');
  document.getElementById('lib-panel-agents').style.display     = tab === 'agents'     ? 'flex' : 'none';
  document.getElementById('lib-panel-archetypes').style.display = tab === 'archetypes' ? 'block' : 'none';
  if (tab === 'archetypes') renderArchetypes();
}
window.switchLibTab = switchLibTab;

// ─── Archetype System ─────────────────────────────────────────────────────────
const ARCHETYPES = [
  {
    id: "moe_13_flagship",
    name: "MoE-13: Mixture-of-Experts",
    desc: "Flagship Industrial Orchestrator: 13 expert agents coordinated via weighted EMA convergence. Now with Data Ingestion context. [Tier 2+]",
    icon: "sparkles",
    color: "var(--cyan)",
    nodes: [
      { name: "Orchestrator",  type: "agent",    dx: 440, dy: 160 },
      { name: "Expert_01",     type: "agent",    dx: 40,  dy: 20  },
      { name: "Expert_02",     type: "agent",    dx: 40,  dy: 90  },
      { name: "Expert_03",     type: "agent",    dx: 40,  dy: 160 },
      { name: "Expert_04",     type: "agent",    dx: 40,  dy: 230 },
      { name: "Expert_05",     type: "agent",    dx: 40,  dy: 300 },
      { name: "Consensus",     type: "gate",     dx: 240, dy: 160 },
      { name: "Decision",      type: "gate",     dx: 640, dy: 160 },
      { name: "Feedback",      type: "agent",    dx: 440, dy: 300 },
      { name: "Context Source", type: "datasource", dx: -160, dy: 160, props: { payload: "# TIS GROUNDING\n- Mode: MoE-13\n- Logic: Balanced Ternary", data_type: "markdown" } },
      { name: "Expert Coordinator", type: "external", dx: 40, dy: 400 }
    ],
    wires: [
      [1,6],[2,6],[3,6],[4,6],[5,6], // Experts to Consensus
      [6,0],                         // Consensus to Orchestrator
      [0,7],                         // Orchestrator to Decision
      [7,8],                         // Decision to Feedback (if uncertain)
      [8,0],                         // Feedback loop
      [9,10],                        // Data Source to LLM Bridge
      [10,6]                         // LLM Bridge to Consensus
    ],
    feedbackWires: [8],
    edgeConds: ["all","all","all","all","all","affirm","affirm","tend","all","all","all"]
  },
  {
    id: "consensus",
    name: "Consensus Pipeline",
    desc: "N agents vote → Consensus Gate → Actuator. Now includes LLM-augmented data ingestion.",
    icon: "git-merge",
    color: "var(--green)",
    nodes: [
      { name: "Sensor A",       type: "agent",    dx: 60,   dy: 60  },
      { name: "Sensor B",       type: "agent",    dx: 60,   dy: 180 },
      { name: "Sensor C",       type: "agent",    dx: 60,   dy: 300 },
      { name: "Consensus Gate", type: "gate",     dx: 320,  dy: 180 },
      { name: "Actuator",       type: "agent",    dx: 540,  dy: 180 },
      { name: "Ref Data",       type: "datasource", dx: 60, dy: 420, props: { payload: "# CONSENSUS REF\n- Majority: 2/3\n- Veto: -1", data_type: "markdown" } },
      { name: "Audit Bridge",   type: "external", dx: 320, dy: 420 }
    ],
    wires: [[0,3],[1,3],[2,3],[3,4],[5,6],[6,3]],
    edgeConds: ["affirm","all","reject","affirm","all","all"],
  },
  {
    id: "guardrail",
    name: "Guardrail Chain",
    desc: "Input → Safety Gate → LLM Bridge → Output Guard. Upgraded with semantic data context.",
    icon: "shield-check",
    color: "var(--amber)",
    nodes: [
      { name: "Input",        type: "agent",    dx: 40,  dy: 160 },
      { name: "Safety Gate",  type: "gate",     dx: 240, dy: 160 },
      { name: "LLM Bridge",   type: "external", dx: 440, dy: 80  },
      { name: "Output Guard", type: "gate",     dx: 440, dy: 240 },
      { name: "Output",       type: "agent",    dx: 640, dy: 160 },
      { name: "Policy Source", type: "datasource", dx: 440, dy: -40, props: { payload: "# SAFETY POLICY\n- No PII leak\n- Respect Veto", data_type: "markdown" } }
    ],
    wires: [[0,1],[1,2],[1,3],[2,4],[3,4],[5,2]],
    edgeConds: ["all","affirm","reject","affirm","affirm","all"],
  },
  {
    id: "filter_rank",
    name: "Filter → Rank → Decide",
    desc: "Raw signals → Filter (tend/reject pruned) → Ranker → Decision node.",
    icon: "funnel",
    color: "var(--cyan)",
    nodes: [
      { name: "Raw Signal",  type: "agent", dx: 40,  dy: 160 },
      { name: "Filter",      type: "gate",  dx: 240, dy: 160 },
      { name: "Ranker",      type: "agent", dx: 440, dy: 160 },
      { name: "Decision",    type: "gate",  dx: 640, dy: 160 },
    ],
    wires: [[0,1],[1,2],[2,3]],
    edgeConds: ["all","affirm","all"],
  },
  {
    id: "debate",
    name: "Multi-Agent Debate",
    desc: "Proposer + Challenger feed Arbiter. Arbiter routes: affirm=Proposer wins, reject=Challenger wins, tend=Retry.",
    icon: "message-square",
    color: "var(--muted)",
    nodes: [
      { name: "Proposer",    type: "agent",    dx: 60,  dy: 80  },
      { name: "Challenger",  type: "agent",    dx: 60,  dy: 260 },
      { name: "Arbiter",     type: "gate",     dx: 300, dy: 170 },
      { name: "Accept",      type: "agent",    dx: 520, dy: 80  },
      { name: "Reject",      type: "agent",    dx: 520, dy: 260 },
    ],
    wires: [[0,2],[1,2],[2,3],[2,4]],
    edgeConds: ["all","all","affirm","reject"],
  },
  {
    id: "sensor_gate",
    name: "Sensor → Gate → Actuator",
    desc: "Simple deterministic agent chain: read → decide → act. Your first production pattern.",
    icon: "cpu",
    color: "var(--blue)",
    nodes: [
      { name: "Sensor",    type: "agent",    dx: 60,  dy: 160 },
      { name: "Gate",      type: "gate",     dx: 280, dy: 160 },
      { name: "Actuator",  type: "agent",    dx: 500, dy: 80  },
      { name: "Fallback",  type: "agent",    dx: 500, dy: 240 },
    ],
    wires: [[0,1],[1,2],[1,3]],
    edgeConds: ["all","affirm","reject"],
  },
  {
    id: "kmu_process_opt",
    name: "KMU: Process Optimization Loop",
    desc: "Continuous improvement loop (IST → SOLL → iterate) with fallback tracking.",
    icon: "refresh-cw",
    color: "var(--amber)",
    nodes: [
      { name: "IST Capture",   type: "agent",    dx: 40,  dy: 80  },
      { name: "Analysis Gate", type: "gate",     dx: 240, dy: 160 },
      { name: "SOLL Design",   type: "agent",    dx: 440, dy: 80  },
      { name: "Test Run",      type: "gate",     dx: 640, dy: 160 },
      { name: "Tracking",      type: "agent",    dx: 840, dy: 160 },
      { name: "Logger",        type: "agent",    dx: 240, dy: 280 }
    ],
    wires: [[0,1],[1,2],[2,3],[3,4],[1,5],[3,1]],
    feedbackWires: [5], // index 5 is [3,1]
    edgeConds: ["all","affirm","all","affirm","reject","reject"]
  },
  {
    id: "kmu_supplier_score",
    name: "KMU: Supplier Scoring",
    desc: "Evaluate and rank suppliers based on parallel multi-criteria assessment.",
    icon: "truck",
    color: "var(--blue)",
    nodes: [
      { name: "Input Validator", type: "agent",    dx: 40,  dy: 160 },
      { name: "Price Check",     type: "agent",    dx: 240, dy: 40  },
      { name: "Quality Check",   type: "agent",    dx: 240, dy: 120 },
      { name: "Delivery Check",  type: "agent",    dx: 240, dy: 200 },
      { name: "Consensus",       type: "gate",     dx: 440, dy: 160 },
      { name: "Decision Gate",   type: "gate",     dx: 640, dy: 160 },
      { name: "Logger",          type: "agent",    dx: 640, dy: 280 }
    ],
    wires: [[0,1],[0,2],[0,3],[1,4],[2,4],[3,4],[4,5],[4,6]],
    edgeConds: ["all","all","all","all","all","all","affirm","tend"]
  },
  {
    id: "kmu_customer_qual",
    name: "KMU: Customer Qualification",
    desc: "Determine if a lead is worth pursuing using parallel scoring checks.",
    icon: "users",
    color: "var(--cyan)",
    nodes: [
      { name: "Input Validator", type: "agent",    dx: 40,  dy: 160 },
      { name: "Budget Check",    type: "agent",    dx: 240, dy: 60  },
      { name: "Industry Fit",    type: "agent",    dx: 240, dy: 160 },
      { name: "Engagement",      type: "agent",    dx: 240, dy: 260 },
      { name: "Aggregation",     type: "gate",     dx: 440, dy: 160 },
      { name: "Routing Gate",    type: "gate",     dx: 640, dy: 160 },
      { name: "Sales / Nurture", type: "agent",    dx: 840, dy: 160 }
    ],
    wires: [[0,1],[0,2],[0,3],[1,4],[2,4],[3,4],[4,5],[5,6]],
    edgeConds: ["all","all","all","all","all","all","affirm","!reject"]
  },
  {
    id: "kmu_invoice_fraud",
    name: "KMU: Invoice Fraud Detection",
    desc: "Detect anomalies and potential fraud in incoming invoices.",
    icon: "file-warning",
    color: "var(--red)",
    nodes: [
      { name: "Input Validation", type: "agent",    dx: 40,  dy: 160 },
      { name: "Amount Deviation", type: "agent",    dx: 240, dy: 60  },
      { name: "Vendor Match",     type: "agent",    dx: 240, dy: 160 },
      { name: "Pattern Detect",   type: "agent",    dx: 240, dy: 260 },
      { name: "Consensus",        type: "gate",     dx: 440, dy: 160 },
      { name: "Decision Gate",    type: "gate",     dx: 640, dy: 160 },
      { name: "Alert Logger",     type: "agent",    dx: 640, dy: 280 }
    ],
    wires: [[0,1],[0,2],[0,3],[1,4],[2,4],[3,4],[4,5],[5,6]],
    edgeConds: ["all","all","all","all","all","all","affirm","affirm"]
  },
  {
    id: "kmu_hiring_decision",
    name: "KMU: Hiring Decision System",
    desc: "Evaluate candidates based on structured criteria and CV analysis.",
    icon: "briefcase",
    color: "var(--green)",
    nodes: [
      { name: "Input Validation", type: "agent",    dx: 40,  dy: 160 },
      { name: "Experience Check", type: "agent",    dx: 240, dy: 40  },
      { name: "Skills Match",     type: "agent",    dx: 240, dy: 120 },
      { name: "CV Analysis",      type: "external", dx: 240, dy: 200 },
      { name: "Test Score",       type: "agent",    dx: 240, dy: 280 },
      { name: "Consensus",        type: "gate",     dx: 440, dy: 160 },
      { name: "Decision Gate",    type: "gate",     dx: 640, dy: 160 }
    ],
    wires: [[0,1],[0,2],[0,3],[0,4],[1,5],[2,5],[3,5],[4,5],[5,6]],
    edgeConds: ["all","all","all","all","all","all","all","all","affirm"]
  },
  {
    id: "industry_sme_pipeline",
    name: "SME: Precision Data Pipeline",
    desc: "Optimized for small teams: sensor-input → threshold logic → automated report.",
    icon: "rows",
    color: "#4ade80",
    nodes: [
      { name: "Raw_Input",      type: "agent", dx: 40,  dy: 160 },
      { name: "Float_Threshold",type: "gate",  dx: 240, dy: 160 },
      { name: "Logger",         type: "agent", dx: 440, dy: 60  },
      { name: "Report_Emit",    type: "agent", dx: 440, dy: 260 }
    ],
    wires: [[0,1],[1,2],[1,3]],
    edgeConds: ["all","all","affirm"]
  },
  {
    id: "industry_enterprise_risk",
    name: "Enterprise: Risk Assessment Swarm",
    desc: "Massive scale: multi-source vetting → supervisor oversight → secure gatekeeper.",
    icon: "eye",
    color: "#000000",
    nodes: [
      { name: "Legal_Audit",    type: "agent",    dx: 40,  dy: 60  },
      { name: "Finance_Vetting",type: "agent",    dx: 40,  dy: 260 },
      { name: "Majority_5",     type: "gate",     dx: 280, dy: 160 },
      { name: "Supervisor",     type: "agent",    dx: 520, dy: 160 },
      { name: "Gatekeeper",     type: "gate",     dx: 760, dy: 160 },
      { name: "Final_Verdict",  type: "agent",    dx: 980, dy: 160 }
    ],
    wires: [[0,2],[1,2],[2,3],[3,4],[4,5]],
    edgeConds: ["all","all","affirm","affirm","affirm"]
  },
  {
    id: "recursive_refiner",
    name: "Recursive Multi-Stage Refiner",
    desc: "Industrial-grade iterative loop: input → multi-stage vetting → feedback loop (tend/reject re-processes) → high-confidence exit.",
    icon: "refresh-ccw",
    color: "var(--amber)",
    nodes: [
      { name: "Raw Entry",      type: "agent",    dx: 40,  dy: 160 },
      { name: "Stage 1: Vetting",type: "agent",    dx: 240, dy: 60  },
      { name: "Stage 2: Audit",  type: "agent",    dx: 240, dy: 260 },
      { name: "Consensus Hub",   type: "gate",     dx: 480, dy: 160 },
      { name: "Refinement Loop", type: "agent",    dx: 480, dy: 340 },
      { name: "Output Guard",    type: "gate",     dx: 720, dy: 160 },
      { name: "Final Emission",  type: "agent",    dx: 960, dy: 160 }
    ],
    wires: [
      [0,1],[0,2],       // Split input
      [1,3],[2,3],       // Converge to Hub
      [3,4],             // Send to loop if uncertain
      [4,3],             // Loop feedback! (Circular)
      [3,5],             // Hub to guard
      [5,6]              // Guard to exit
    ],
    feedbackWires: [5], // index 5 is [4,3]
    edgeConds: ["all","all","all","all","tend","all","affirm","affirm"]
  },
  {
    id: "industry_iot_grid",
    name: "Industrial: IoT Sensor Grid",
    desc: "Hardened real-time monitoring: mesh nodes → watchdog → emergency stop.",
    icon: "bell",
    color: "#ef4444",
    nodes: [
      { name: "Mesh_Node_A",    type: "agent", dx: 40,  dy: 60  },
      { name: "Mesh_Node_B",    type: "agent", dx: 40,  dy: 260 },
      { name: "Range_Validator",type: "gate",  dx: 280, dy: 160 },
      { name: "Watchdog",       type: "agent", dx: 520, dy: 160 },
      { name: "SCADA_Emit",     type: "agent", dx: 760, dy: 60  },
      { name: "Emergency_Stop", type: "agent", dx: 760, dy: 260 }
    ],
    wires: [[0,2],[1,2],[2,3],[3,4],[3,5]],
    edgeConds: ["all","all","all","affirm","reject"]
  }
];

function renderArchetypes(q = "") {
  const lib = document.getElementById("arch-lib-items");
  if (!lib) return;
  lib.innerHTML = `<div style="font-size:10px;color:var(--muted2);margin-bottom:10px;line-height:1.6;padding:8px 0;">Click to spawn a wired agent architecture on the canvas.</div>`;
  
  const searchInput = document.getElementById("archLibSearch");
  if (searchInput && q && searchInput.value !== q) {
    searchInput.value = q;
  }

  const groups = {};
  Object.keys(ARCHETYPE_ONTOLOGY).forEach(cat => groups[cat] = []);

  const categorize = (id) => {
    for (const [cat, ids] of Object.entries(ARCHETYPE_ONTOLOGY)) {
      if (ids.includes(id)) return cat;
    }
    return "Orchestration & Consensus"; // Default
  };

  ARCHETYPES.forEach(arch => {
    if (q && !arch.name.toLowerCase().includes(q.toLowerCase()) && !arch.desc.toLowerCase().includes(q.toLowerCase())) return;
    const cat = categorize(arch.id);
    groups[cat].push(arch);
  });

  Object.entries(groups).forEach(([cat, items]) => {
    if (items.length === 0) return;

    const isOpen = q ? true : _archetypeOpen[cat];
    const catDiv = document.createElement("div");
    catDiv.className = "lib-category" + (isOpen ? "" : " collapsed");

    const header = document.createElement("div");
    header.className = "lib-category-header";
    header.style.display = "flex";
    header.style.justifyContent = "space-between";
    header.style.alignItems = "center";
    header.innerHTML = `<span>${cat}</span><i data-lucide="chevron-down"></i>`;
    header.onclick = () => {
      _archetypeOpen[cat] = !_archetypeOpen[cat];
      renderArchetypes(q);
    };
    catDiv.appendChild(header);

    if (isOpen) {
      const itemsDiv = document.createElement("div");
      itemsDiv.className = "lib-category-items";
      itemsDiv.style.padding = "8px 0";

      items.forEach(arch => {
        const card = document.createElement("div");
        card.className = "archetype-card";
        card.draggable = true;
        card.ondragstart = (e) => {
          e.dataTransfer.setData("tern-node-type", "archetype");
          e.dataTransfer.setData("tern-arch-id", arch.id);
        };
        card.innerHTML = `
          <div class="archetype-card-title" style="display:flex;align-items:center;gap:6px;">
            <i data-lucide="${arch.icon}" style="width:12px;height:12px;color:${arch.color}"></i>
            ${arch.name}
          </div>
          <div class="archetype-card-desc">${arch.desc}</div>
        `;
        card.onclick = () => spawnArchetype(arch);
        itemsDiv.appendChild(card);
      });
      catDiv.appendChild(itemsDiv);
    }
    lib.appendChild(catDiv);
  });

  if (lib.querySelectorAll(".archetype-card").length === 0) {
    const empty = document.createElement("div");
    empty.style.padding = "20px";
    empty.style.textAlign = "center";
    empty.style.color = "var(--muted2)";
    empty.style.fontSize = "11px";
    empty.textContent = "No archetypes match your search.";
    lib.appendChild(empty);
  }

  lucide.createIcons();
}
window.renderArchetypes = renderArchetypes;

function spawnArchetype(arch, forcedX, forcedY) {
  const canvas = document.getElementById("flow-canvas");
  // Clear hint
  const hint = document.getElementById("canvas-hint");
  if (hint) hint.style.display = "none";

  // Center on current viewport using our infinite coordinate logic
  const { x: vX, y: vY } = viewportCenterInCanvas(-300, -200);
  const baseX = (forcedX !== undefined) ? forcedX : vX;
  const baseY = (forcedY !== undefined) ? forcedY : vY;

  const ids = [];
  arch.nodes.forEach((n, i) => {
    const id = "node_" + Date.now() + "_" + i;
    ids.push(id);
    createFlowNode(n.name, "__arch__", baseX + n.dx, baseY + n.dy, n.type, id);
    
    // Inject specialized functional code based on archetype + node intent
    const node = flowNodes.find(fn => fn.id === id);
    if (node) {
      node.props.code = getArchetypeCode(arch.id, n.name, n.type);
      if (n.props) {
        node.props = { ...node.props, ...n.props };
      }
      updateNodeSchemaDisplay(id);
    }
  });

  // Wire them with conditions
  arch.wires.forEach(([from, to], i) => {
    const wireId = "wire_" + Date.now() + "_" + i;
    const cond = (arch.edgeConds || [])[i] || "all";
    const isFeedback = (arch.feedbackWires || []).includes(i);
    flowWires.push({ 
      id: wireId, fromId: ids[from], toId: ids[to], 
      signal: 0, condition: cond, transform: "none", 
      label: cond !== "all" ? cond : "", priority: 5,
      isFeedback: isFeedback
    });
  });

  updateWires();
  saveCanvasState();
  showToast(`Architecture "${arch.name}" spawned`, "ok");
}
window.spawnArchetype = spawnArchetype;

function getArchetypeCode(archId, nodeName, type) {
  const n = nodeName.toLowerCase();
  
  // Generic Gates
  if (type === 'gate') {
    if (n.includes('consensus') || n.includes('majority') || n.includes('aggregation')) 
      return `fn main() -> trit {\n    let val: trit = read_input();\n    // Consensus logic is handled by the edge algebra,\n    // but we can add local filtering here.\n    return val;\n}`;
    if (n.includes('safety') || n.includes('guard') || n.includes('validator') || n.includes('gatekeeper'))
      return `fn main() -> trit {\n    let val: trit = read_input();\n    if val == reject { emit \"VETO_TRIGGERED\"; return reject; }\n    emit \"CHECK_PASSED\";\n    return val;\n}`;
    if (n.includes('decision') || n.includes('routing') || n.includes('arbiter'))
      return `fn main() -> trit {\n    let val: trit = read_input();\n    match val {\n        affirm => { emit \"PROCEED\"; return affirm; }\n        tend   => { emit \"HOLD_PENDING\"; return tend; }\n        reject => { emit \"TERMINATE\"; return reject; }\n    }\n}`;
    return `fn main() -> trit { return read_input(); }`;
  }

  // Domain Specific Logic
  if (archId === 'moe_13_flagship') {
    if (n.includes('expert')) return `fn main() -> trit {\n    // MoE Expert Agent Sub-process\n    emit \"EXPERT_INVOKED\";\n    return consensus(truth(), hold());\n}`;
    if (n.includes('orchestrator')) return `fn main() -> trit {\n    // EMA convergence: S_r = α·e_r + (1−α)·S_{r−1}\n    let sig: trit = read_input();\n    emit \"EMA_CONVERGING\";\n    return sig;\n}`;
    if (n.includes('feedback')) return `fn main() -> trit {\n    emit \"REFINING_EMA\";\n    return tend;\n}`;
  }

  if (archId.startsWith('kmu_')) {
    if (n.includes('input') || n.includes('capture')) return `fn main() -> trit {\n    // Initialize process state\n    emit \"INIT_STATION\";\n    return affirm;\n}`;
    if (n.includes('price'))   return `fn main() -> trit {\n    // Evaluates against Tier-3 margin constraints\n    emit \"MARGIN_CHECK_OK\";\n    return affirm;\n}`;
    if (n.includes('quality')) return `fn main() -> trit {\n    // ISO-9001 compliance check\n    return affirm;\n}`;
    if (n.includes('fraud') || n.includes('pattern')) return `fn main() -> trit {\n    // Heuristic anomaly detection\n    return tend;\n}`;
    if (n.includes('skills'))  return `fn main() -> trit {\n    // Match CV to job requirements\n    return affirm;\n}`;
    if (n.includes('logger') || n.includes('tracking')) return `fn main() -> trit {\n    let sig: trit = read_input();\n    print(\"@ TRACE: \" + sig);\n    return sig;\n}`;
  }

  if (archId === 'industry_iot_grid') {
    if (n.includes('node'))     return `fn main() -> trit {\n    // Hardened telemetry node\n    emit \"TELEMETRY_TX\";\n    return affirm;\n}`;
    if (n.includes('watchdog')) return `fn main() -> trit {\n    let input: trit = read_input();\n    if input == tend { emit \"LINK_LATENCY\"; }\n    return input;\n}`;
    if (n.includes('emergency'))return `fn main() -> trit {\n    emit \"EMERGENCY_STOP\";\n    return reject;\n}`;
  }

  if (archId === 'debate') {
    if (n.includes('proposer'))  return `fn main() -> trit {\n    emit \"PROPOSING_THESIS\";\n    return affirm;\n}`;
    if (n.includes('challenger'))return `fn main() -> trit {\n    emit \"COUNTER_ARGUMENT\";\n    return reject;\n}`;
  }

  // Default functional template
  return `fn main() -> trit {\n    return affirm;\n}`;
}
window.getArchetypeCode = getArchetypeCode;

// ─── Edge Logic System ────────────────────────────────────────────────────────
let selectedWireId = null;

function selectWire(wireId) {
  selectedWireId = wireId;
  selectedNodeId = null;
  const header = document.getElementById("prop-header-label");
  const help = document.getElementById("prop-help-icon");
  if (header) header.textContent = "EDGE PROPERTIES";
  if (help) help.style.display = "flex";
  document.querySelectorAll('.flow-node').forEach(n => n.classList.remove('selected'));
  updateWireStyles();
  updateEdgePanel();
}
window.selectWire = selectWire;

function updateWireStyles() {
  document.querySelectorAll('.flow-wire').forEach(el => {
    el.classList.remove('selected-wire');
    const id = el.getAttribute('data-wire-id');
    const wire = flowWires.find(w => w.id === id);
    if (!wire) return;
    el.classList.remove('cond-affirm','cond-tend','cond-reject','cond-all');
    if (wire.condition && wire.condition !== "all") el.classList.add('cond-' + wire.condition);
    if (id === selectedWireId) el.classList.add('selected-wire');
  });
}
window.updateWireStyles = updateWireStyles;

function updateEdgePanel() {
  const header = document.getElementById("prop-header-label");
  const body   = document.getElementById("prop-body");
  const help   = document.getElementById("prop-help-icon");
  const wire   = flowWires.find(w => w.id === selectedWireId);
  if (!wire) { updatePropertyPanel(); return; }

  if (header) header.textContent = "Edge Properties";
  if (help) help.style.display = "flex";
  const fromNode  = flowNodes.find(n => n.id === wire.fromId);
  const toNode    = flowNodes.find(n => n.id === wire.toId);

  EdgePanelController.render(wire, body, fromNode, toNode);
}
window.updateEdgePanel = updateEdgePanel;

function updateWireProp(key, val) {
  const wire = flowWires.find(w => w.id === selectedWireId);
  if (!wire) return;
  wire[key] = val;
  if (key === 'condition') {
    wire.label = val !== 'all' ? val : '';
    updateEdgePanel(); // re-render to update label field
  }
  updateWireStyles();
  updateWires();
}
window.updateWireProp = updateWireProp;

function deleteWire(id) {
  flowWires = flowWires.filter(w => w.id !== id);
  selectedWireId = null;
  document.getElementById("wire-handle").classList.remove("active");
  updateWires();
  updatePropertyPanel();
  if (document.getElementById("prop-header-label"))
    document.getElementById("prop-header-label").textContent = "Node Properties";
}
window.deleteWire = deleteWire;

// ─── Wiring System ────────────────────────────────────────────────────────────
function toggleInspector() {
  const ins = document.getElementById("flow-inspector");
  const icon = document.getElementById("ins-toggle-icon");
  if (!ins) return;
  if (ins.classList.contains("inspector-minimized")) {
    ins.classList.replace("inspector-minimized", "inspector-expanded");
    if (icon) icon.setAttribute("data-lucide", "chevron-down");
  } else {
    ins.classList.replace("inspector-expanded", "inspector-minimized");
    if (icon) icon.setAttribute("data-lucide", "chevron-up");
    ins.style.height = ""; // Clear inline override
  }
  if (window.lucide) lucide.createIcons();
}
window.toggleInspector = toggleInspector;

function logInspector(nodeName, msg) {
  const body = document.getElementById("ins-body");
  if (!body) return;
  const time = new Date().toLocaleTimeString([], { hour12: false, hour: '2-digit', minute: '2-digit', second: '2-digit' });
  const row = document.createElement("div");
  row.className = "ins-row";
  row.innerHTML = `
    <span class="ins-time">[${time}]</span>
    <span class="ins-node">${nodeName}</span>
    <span class="ins-msg">${msg}</span>
  `;
  body.appendChild(row);
  body.scrollTop = body.scrollHeight;
}
window.logInspector = logInspector;

// ─── Topological Sort (Kahn's algorithm) ─────────────────────────────────────
function topoSort() {
  const inDeg = {}, adj = {};
  flowNodes.forEach(n => { inDeg[n.id] = 0; adj[n.id] = []; });
  
  // Build adjacency list but skip feedback edges for topological ordering
  flowWires.forEach(w => { 
    if (w.isFeedback) return; 
    if (adj[w.fromId]) { 
      adj[w.fromId].push(w.toId); 
      inDeg[w.toId] = (inDeg[w.toId] || 0) + 1; 
    } 
  });
  
  const queue = flowNodes.filter(n => inDeg[n.id] === 0).map(n => n.id);
  const order = [];
  while (queue.length) {
    const id = queue.shift();
    order.push(id);
    (adj[id] || []).forEach(to => { if (--inDeg[to] === 0) queue.push(to); });
  }
  const hasCycle = order.length < flowNodes.length;
  const cycleNodes = hasCycle ? flowNodes.filter(n => !order.includes(n.id)) : [];
  return { order, hasCycle, cycleNodes };
}
window.topoSort = topoSort;

// ─── Graph Validation ─────────────────────────────────────────────────────────
function validateGraph() {
  const errors = [], warnings = [];
  // Clear previous validation markers
  document.querySelectorAll('.flow-node').forEach(el => el.classList.remove('node-error','node-warn'));
  document.querySelectorAll('.node-badge').forEach(b => b.remove());

  if (flowNodes.length === 0) { errors.push({ msg: "Canvas is empty", nodeId: null }); return { errors, warnings }; }

  // Topo sort → cycle detection
  const { hasCycle, cycleNodes } = topoSort();
  if (hasCycle) {
    cycleNodes.forEach(n => {
      errors.push({ msg: `Cycle detected at "${n.name}"`, nodeId: n.id });
      markNode(n.id, "error", "↺");
    });
  }

  flowNodes.forEach(n => {
    const inW  = flowWires.filter(w => w.toId   === n.id);
    const outW = flowWires.filter(w => w.fromId  === n.id);
    const isEntry = inW.length === 0;
    const isExit  = outW.length === 0;

    // Isolated node (no wires at all)
    if (isEntry && isExit && flowNodes.length > 1) {
      warnings.push({ msg: `"${n.name}" is isolated — not connected`, nodeId: n.id });
      markNode(n.id, "warn", "⚠");
    }

    // Agent with no code
    if (n.type === 'agent' && !n.props.code?.trim()) {
      warnings.push({ msg: `"${n.name}" has no .tern code`, nodeId: n.id });
      if (!outW.length) markNode(n.id, "warn", "?");
    }

    // Schema mismatch on incoming wires
    inW.forEach(w => {
      const src = flowNodes.find(f => f.id === w.fromId);
      if (src && src.props.output_schema && n.props.input_schema &&
          src.props.output_schema.trim() !== n.props.input_schema.trim()) {
        warnings.push({ msg: `Schema mismatch: "${src.name}" → "${n.name}"`, nodeId: n.id });
        markNode(n.id, "warn", "≠");
      }
    });

    // Multiple entry nodes with conflicting signals not going through gate
    if (inW.length > 1 && n.type === 'agent') {
      warnings.push({ msg: `"${n.name}" has ${inW.length} inputs — consider a Gate node for consensus`, nodeId: n.id });
    }
  });

  return { errors, warnings };
}
window.validateGraph = validateGraph;

function markNode(id, level, badge) {
  const el = document.getElementById(id);
  if (!el) return;
  el.classList.add(level === "error" ? "node-error" : "node-warn");
  const b = document.createElement("div");
  b.className = "node-badge node-badge-" + level;
  b.textContent = badge;
  el.appendChild(b);
}
window.markNode = markNode;

function showValidationPanel(errors, warnings) {
  const header = document.getElementById("prop-header-label");
  const body   = document.getElementById("prop-body");
  const help   = document.getElementById("prop-help-icon");
  if (header) header.textContent = "Graph Validation";
  if (help) help.style.display = "none";
  const total = errors.length + warnings.length;
  if (total === 0) {
    body.innerHTML = `<div style="color:var(--green);font-size:13px;text-align:center;margin-top:40px;">✓ Graph is valid</div><div style="color:var(--muted);font-size:11px;text-align:center;margin-top:8px;">No errors or warnings found.</div>`;
    return;
  }
  const items = [...errors.map(e => ({ ...e, level: "error" })), ...warnings.map(w => ({ ...w, level: "warn" }))];
  body.innerHTML = `
    <div style="font-size:11px;color:var(--muted2);margin-bottom:12px;">${errors.length} error${errors.length!==1?'s':''}, ${warnings.length} warning${warnings.length!==1?'s':''}</div>
    ${items.map(i => `
      <div style="display:flex;gap:8px;padding:8px;margin-bottom:4px;border-radius:6px;background:${i.level==='error'?'rgba(239,68,68,0.08)':'rgba(245,158,11,0.08)'};border:1px solid ${i.level==='error'?'var(--red)':'var(--amber)'};cursor:${i.nodeId?'pointer':'default'};"
           ${i.nodeId ? `onclick="selectNode('${i.nodeId}');updatePropertyPanel()"` : ''}>
        <span style="color:${i.level==='error'?'var(--red)':'var(--amber)'};font-size:14px;flex-shrink:0;">${i.level==='error'?'✗':'⚠'}</span>
        <span style="font-size:11px;color:var(--text);line-height:1.5;">${i.msg}</span>
      </div>`).join('')}
    <button class="btn btn-ghost" style="width:100%;margin-top:12px;font-size:11px;" onclick="document.querySelectorAll('.flow-node').forEach(el=>el.classList.remove('node-error','node-warn'));document.querySelectorAll('.node-badge').forEach(b=>b.remove());updatePropertyPanel();">Clear markers</button>
  `;
}
window.showValidationPanel = showValidationPanel;

// ══════════════════════════════════════════════════════════════════════════════
// 🧩 TernFlow v2 Core: Signal Algebra & Event Queue
// ══════════════════════════════════════════════════════════════════════════════

const TernaryAlgebra = {
  // Merge multiple incoming signals into a single state
  merge(signals) {
    if (!signals.length) return { val: 0, conf: 0 };
    
    // Weighted mean approach for confidence-aware merging
    let totalWeight = 0;
    let weightedVal = 0;
    
    signals.forEach(s => {
      weightedVal += s.val * s.conf;
      totalWeight += s.conf;
    });
    
    if (totalWeight === 0) return { val: 0, conf: 0 };
    
    const rawVal = weightedVal / totalWeight;
    const finalVal = rawVal > 0.33 ? 1 : (rawVal < -0.33 ? -1 : 0);
    const finalConf = totalWeight / signals.length; // Normalized confidence
    
    return { val: finalVal, conf: Math.min(1, finalConf) };
  },

  // Apply edge transformations (Flip, Block, Hold) and scale confidence
  transform(signal, wire) {
    let out = { ...signal };
    
    // Every "hop" through a wire slightly decays confidence unless it's a high-priority path
    const decay = wire.priority ? (11 - wire.priority) * 0.02 : 0.05;
    out.conf = Math.max(0, out.conf - decay);

    if (wire.condition && wire.condition !== "all") {
      const pass = (wire.condition==="affirm"&&out.val===1)||(wire.condition==="tend"&&out.val===0)||(wire.condition==="reject"&&out.val===-1)||(wire.condition==="!reject"&&out.val!==-1)||(wire.condition==="!tend"&&out.val!==0);
      if (!pass) {
        if (wire.transform === "block") return null;
        if (wire.transform === "flip")  { out.val = -out.val; out.conf *= 0.8; }
        if (wire.transform === "hold")  { out.val = 0; out.conf *= 0.5; }
      }
    }
    return out;
  }
};

const engineQueue = [];
const MAX_ENGINE_TICKS = 2000;

/**
 * Confidence Fog Heatmap Renderer
 * Visualizes the "weather" of the logic swarm.
 */
function updateFogHeatmap() {
  const canvas = document.getElementById("flow-fog-canvas");
  const wrap = document.getElementById("flow-canvas-wrap");
  if (!canvas || !wrap) return;
  const ctx = canvas.getContext("2d");

  // Sync canvas size to VIEWPORT (wrap) dimensions
  if (canvas.width !== wrap.clientWidth || canvas.height !== wrap.clientHeight) {
    canvas.width = wrap.clientWidth; canvas.height = wrap.clientHeight;
  }

  ctx.clearRect(0, 0, canvas.width, canvas.height);

  // Background "Static" / Base Fog — very subtle in viewport space
  ctx.fillStyle = "rgba(10, 15, 25, 0.35)";
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  flowNodes.forEach(node => {
    // Transform logical canvas coordinate (node.x, node.y) to VIEWPORT coordinate
    // View = Logic * Scale + Pan
    const vx = node.x * CT.scale + CT.x + (90 * CT.scale); // centered on node body
    const vy = node.y * CT.scale + CT.y + (40 * CT.scale);

    // Skip if far outside viewport bounds
    if (vx < -500 || vx > canvas.width + 500 || vy < -500 || vy > canvas.height + 500) return;

    const statusDot = document.getElementById("status-" + node.id);
    let trit = 0;
    if (statusDot) {
      if (statusDot.classList.contains("ok")) trit = 1;
      else if (statusDot.classList.contains("err")) trit = -1;
      else if (statusDot.classList.contains("run")) trit = 0;
    }

    // Dynamic radius scales slightly with zoom but has a floor
    const radius = 350 * CT.scale;
    const grad = ctx.createRadialGradient(vx, vy, 40 * CT.scale, vx, vy, radius);

    if (trit === 1) {
      grad.addColorStop(0, "rgba(34, 197, 94, 0.3)");
      grad.addColorStop(1, "rgba(10, 12, 16, 0)");
    } else if (trit === -1) {
      grad.addColorStop(0, "rgba(239, 68, 68, 0.3)");
      grad.addColorStop(1, "rgba(10, 12, 16, 0)");
    } else {
      grad.addColorStop(0, "rgba(168, 85, 247, 0.2)");
      grad.addColorStop(1, "rgba(10, 12, 16, 0)");
    }

    ctx.globalCompositeOperation = "screen";
    ctx.fillStyle = grad;
    ctx.beginPath();
    ctx.arc(vx, vy, radius, 0, Math.PI * 2);
    ctx.fill();
  });
}
window.updateFogHeatmap = updateFogHeatmap;
// ─── End of v2 Core Modules ──────────────────────────────────────────────────

async function runSimulation() {
  if (simulationRunning) return;
  simulationAborted = false;
  simulationRunning = true;
  updateSimUI();

  try {
    const { errors, warnings } = validateGraph();
    if (errors.length > 0) {
      showValidationPanel(errors, warnings);
      showToast(`${errors.length} error${errors.length>1?'s':''} — fix before simulating`, "error");
      return;
    }

    // 1. Graph Memory Wipe (Reset execution states)
    flowNodes.forEach(n => {
      n.visited = false;
      n.executed = false;
      if (n.props) n.props.status = "";
    });
    flowWires.forEach(w => {
      w.active = false;
      w.signal = 0;
    });

    // 2. Canvas Scrub (Wipe ghost dots immediately)
    const canvas = document.getElementById("scrub-layer");
    if (canvas) {
      const ctx = canvas.getContext("2d");
      ctx.clearRect(0, 0, canvas.width, canvas.height);
    }

    // 3. UI & Clock Reset
    const scrubber = document.getElementById("global-timeline");
    if (scrubber) scrubber.value = 0;
    
    resetSimHistory();
    const stopBtn = document.getElementById("simStopBtn");
    if (stopBtn) stopBtn.style.display = "inline-flex";

    const ins = document.getElementById("flow-inspector");
    if (ins) {
      ins.classList.add("active");
      if (ins.classList.contains("inspector-minimized")) {
        ins.classList.replace("inspector-minimized", "inspector-expanded");
      }
    }
    const insBody = document.getElementById("ins-body");
    if (insBody) insBody.innerHTML = "";

    // Reset visual state
    document.querySelectorAll('.trit-particle-ghost').forEach(p => p.remove());
    document.querySelectorAll('.flow-node').forEach(n => n.classList.remove('pulse-affirm','pulse-reject','pulse-hold','node-error','node-warn'));
    flowNodes.forEach(n => setNodeStatus(n.id, ""));

    engineQueue.length = 0;
    logInspector("SYSTEM", "🚀 TernFlow Engine v2 initialized");
    updateFogHeatmap();

    // Snapshot: Initial State (Tick 0)
    captureSimSnapshot(0);

    // Initial Seed
    const roots = flowNodes.filter(n => !flowWires.some(w => w.toId === n.id));
    roots.forEach(root => {
      engineQueue.push({ toId: root.id, val: 1, conf: 1.0, origin: "ROOT" });
    });

    // PHANTOM PASS: Pure Logic & Timing Calculation (Memory Locked)
    const masterQueueClone = [...engineQueue]; 
    const { scheduledEvents, maxSimDuration } = await calculateGlobalTimeline(masterQueueClone);
    window.globalScheduledEvents = scheduledEvents; 
    
    // 1. The Print Probe
    console.log('DEBUG -> Events generated:', scheduledEvents.length, '| Total Duration:', maxSimDuration);
    
    // 2. Fix runSimulation Handoff: Ensure the visual pass is not blocked by phantom-triggered aborts
    simulationAborted = false; 

    // VISUAL PASS: Pure Rendering & Playback
    if (scheduledEvents.length === 0 || maxSimDuration === 0) {
      console.warn("[DIAGNOSTIC] Simulation data empty. Check roots and latencies.");
    }
    await runSimulationCore(scheduledEvents, maxSimDuration);

  } catch (err) {
    console.error("Simulation Start Failure:", err);
    showToast("Simulation failed to initialize", "err");
    if (window.TERNLANG_CRITICAL_DEBUG) {
      window.TERNLANG_CRITICAL_DEBUG.push({ ts: Date.now(), msg: "runSimulation Crash", error: err.message });
    }
  } finally {
    // Note: runSimulationCore manages its own simulationRunning = false
    // But if we failed before starting the core, we must reset it here.
    const isCoreRunning = (simulationRunning && !simulationAborted);
    if (!isCoreRunning) {
       simulationRunning = false;
       updateSimUI();
    }
  }
}
window.runSimulation = runSimulation;

/**
 * Phase 1: Phantom Pass
 * Mathematical dry-run to find Total Simulation Duration without DOM side-effects.
 * Receives cloned masterQueue to avoid destructive consumption.
 */
async function calculateGlobalTimeline(masterQueue) {
  const scheduledEvents = [];
  let maxSimDuration = 0;
  const dryQueue = masterQueue; 
  const nodeTimings = {}; // id -> endTime

  while (dryQueue.length > 0 && scheduledEvents.length < MAX_ENGINE_TICKS && !simulationAborted) {
    const signal = dryQueue.shift();
    const node = flowNodes.find(n => n.id === signal.toId);
    if (!node) continue;

    // Node Timing: starts when latest input signal arrives
    const nodeStartTime = signal.absEndTime || 0;
    const nodeProcessingTime = 150; // ms
    const nodeEndTime = nodeStartTime + nodeProcessingTime;
    nodeTimings[node.id] = Math.max(nodeTimings[node.id] || 0, nodeEndTime);
    
    // Safety: Ensure maxSimDuration accounts for the node itself, not just outgoing wires
    maxSimDuration = Math.max(maxSimDuration, nodeEndTime);

    // Call simulateNode with isPhantom=true to suppress DOM/CSS/Logs
    const outVal = await simulateNode(node, signal.val, true);
    if (simulationAborted) break;

    const outSignal = { val: outVal, conf: signal.conf, origin: node.id };
    const outWires = flowWires.filter(w => w.fromId === node.id);

    for (const wire of outWires) {
      const transformed = TernaryAlgebra.transform(outSignal, wire);
      if (transformed) {
        const wireStartTime = nodeTimings[node.id];
        // 3. The NaN / Zero Trap (Safe latency parsing)
        const latency = parseFloat(wire.latency);
        const wireDuration = !isNaN(latency) ? latency : simSpeed;
        const wireEndTime = wireStartTime + wireDuration;

        const event = {
          wireId: wire.id,
          val: transformed.val,
          conf: transformed.conf,
          startTime: wireStartTime,
          endTime: wireEndTime,
          duration: wireDuration,
          fromId: wire.fromId,
          toId: wire.toId
        };
        scheduledEvents.push(event);
        maxSimDuration = Math.max(maxSimDuration, wireEndTime);
        dryQueue.push({ toId: wire.toId, ...transformed, absEndTime: wireEndTime });
      }
    }
  }

  // Clear global queue as we have pre-calculated everything
  engineQueue.length = 0;
  return { scheduledEvents, maxSimDuration };
}

/**
 * Phase 2: Visual Pass
 * Pure playback driven by requestAnimationFrame.
 */
async function runSimulationCore(scheduledEvents, maxSimDuration) {
  const scrubber = document.getElementById("global-timeline");
  const tlLabel = document.getElementById("timeline-tick-label");
  const loggedEvents = new Set(); // To track which events have been logged in this pass
  
  if (scrubber) {
    scrubber.value = 0;
    scrubber.max = maxSimDuration; 
  }
  
  lastRealTime = performance.now();
  virtualClock = 0;
  simulationRunning = true;

  const driveTimeline = () => {
    if (!simulationRunning || simulationAborted) return;
    
    const now = performance.now();
    const delta = now - lastRealTime;
    lastRealTime = now;

    virtualClock += delta;
    if (virtualClock >= maxSimDuration) {
      virtualClock = maxSimDuration;
      simulationRunning = false;
      updateSimUI();
      logInspector("SYSTEM", "✓ Pre-flight playback complete");
    }

    if (scrubber) {
      scrubber.value = virtualClock;
      if (tlLabel) tlLabel.textContent = `TIME: ${(virtualClock / 1000).toFixed(2)}s`;
    }
    
    renderScrubLayer(virtualClock, scheduledEvents, loggedEvents);

    if (simulationRunning) {
      requestAnimationFrame(driveTimeline);
    }
  };
  requestAnimationFrame(driveTimeline);
}
window.runSimulationCore = runSimulationCore;

// ─── DELTA RING TIMELINE ─────────────────────────────────────────────────────────

let simBaseState = null;
let currentSimState = null;

function resetSimHistory() {
  simHistory = [];
  simBaseState = null;
  currentSimState = null;
  lastRenderedTick = -1;
  const canvas = document.getElementById("scrub-layer");
  if (canvas) {
    const ctx = canvas.getContext("2d");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
  }
}
window.resetSimHistory = resetSimHistory;

function captureSimSnapshot(tick, activeSignals = [], startTime = 0, duration = 500) {
  if (simHistory.length === 0 || !simBaseState) {
    simBaseState = {
      nodes: flowNodes.map(n => {
        const el = document.getElementById(n.id);
        const pulse = el ? (el.classList.contains('pulse-affirm') ? 'affirm' : (el.classList.contains('pulse-reject') ? 'reject' : (el.classList.contains('pulse-hold') ? 'hold' : ''))) : "";
        return { id: n.id, status: n.props.status || "", pulse };
      }),
      wires: flowWires.map(w => ({ id: w.id, signal: w.signal || 0 }))
    };
    currentSimState = JSON.parse(JSON.stringify(simBaseState));
  }

  const delta = { 
    tick, 
    activeSignals, 
    nodeDeltas: [], 
    wireDeltas: [],
    startTime,
    duration
  };

  flowNodes.forEach(n => {
    const el = document.getElementById(n.id);
    const pulse = el ? (el.classList.contains('pulse-affirm') ? 'affirm' : (el.classList.contains('pulse-reject') ? 'reject' : (el.classList.contains('pulse-hold') ? 'hold' : ''))) : "";
    const status = n.props.status || "";

    const currNode = currentSimState.nodes.find(cn => cn.id === n.id);
    if (currNode && (currNode.status !== status || currNode.pulse !== pulse)) {
       delta.nodeDeltas.push({ id: n.id, status, pulse });
       currNode.status = status; currNode.pulse = pulse;
    }
  });

  flowWires.forEach(w => {
    const sig = w.signal || 0;
    const currWire = currentSimState.wires.find(cw => cw.id === w.id);
    if (currWire && currWire.signal !== sig) {
       delta.wireDeltas.push({ id: w.id, signal: sig });
       currWire.signal = sig;
    }
  });

  simHistory.push(delta);

  // Strict 2000-tick Circular Buffer
  if (simHistory.length > MAX_ENGINE_TICKS) {
    const oldest = simHistory.shift();
    oldest.nodeDeltas.forEach(nd => {
       const bn = simBaseState.nodes.find(n => n.id === nd.id);
       if (bn) { bn.status = nd.status; bn.pulse = nd.pulse; }
    });
    oldest.wireDeltas.forEach(wd => {
       const bw = simBaseState.wires.find(w => w.id === wd.id);
       if (bw) bw.signal = wd.signal;
    });
  }

  // Real-time Timeline Sync (using absolute time)
  const scrubber = document.getElementById("global-timeline");
  const totalDuration = startTime + duration;
  if (scrubber) {
    scrubber.max = totalDuration;
  }
}
window.captureSimSnapshot = captureSimSnapshot;

function showTimeline() {
  const scrubber = document.getElementById("global-timeline");
  const label = document.getElementById("timeline-tick-label");

  if (simHistory.length === 0) return;

  const minTime = simHistory[0].startTime;
  const lastSnapshot = simHistory[simHistory.length - 1];
  const maxTime = lastSnapshot.startTime + lastSnapshot.duration;

  if (scrubber) {
    scrubber.min = minTime;
    scrubber.max = maxTime;
    scrubber.value = maxTime;
  }
  if (label) label.textContent = `TIME: ${(maxTime / 1000).toFixed(2)}s`;
}
window.showTimeline = showTimeline;

function scrubToTimeline(val) {
  const time = parseFloat(val);
  const label = document.getElementById("timeline-tick-label");
  if (label) label.textContent = `TIME: ${(time / 1000).toFixed(2)}s`;
  requestScrub(time);
}
window.scrubToTimeline = scrubToTimeline;

let lastRenderedTick = -1;
let scrubAnimFrame = null;
let currentScrubValue = 0;

function requestScrub(val) {
  currentScrubValue = parseFloat(val);
  if (!scrubAnimFrame) {
    scrubAnimFrame = requestAnimationFrame(performScrub);
  }
}
window.requestScrub = requestScrub;

function performScrub() {
  scrubAnimFrame = null;
  const time = currentScrubValue;
  if (simHistory.length === 0 || !simBaseState) return;

  // Find the snapshot that covers this absolute time
  let snapshotIdx = simHistory.findIndex(h => time >= h.startTime && time < (h.startTime + h.duration));
  if (snapshotIdx === -1) {
    // If it's exactly the end or out of bounds
    if (time >= simHistory[simHistory.length - 1].startTime) {
      snapshotIdx = simHistory.length - 1;
    } else {
      snapshotIdx = 0;
    }
  }

  const state = simHistory[snapshotIdx];
  const floorTick = state.tick;
  // Progress within this tick's animation [0.0 - 1.0]
  const frac = Math.min(1, Math.max(0, (time - state.startTime) / state.duration));

  // 1. Rebuild DOM state only if integer tick changed
  if (floorTick !== lastRenderedTick) {
    lastRenderedTick = floorTick;

    const domState = JSON.parse(JSON.stringify(simBaseState));
    for (let i = 0; i <= snapshotIdx && i < simHistory.length; i++) {
      const delta = simHistory[i];
      delta.nodeDeltas.forEach(nd => {
        const sn = domState.nodes.find(n => n.id === nd.id);
        if (sn) { sn.status = nd.status; sn.pulse = nd.pulse; }
      });
      delta.wireDeltas.forEach(wd => {
        const sw = domState.wires.find(w => w.id === wd.id);
        if (sw) sw.signal = wd.signal;
      });
    }

    // Apply to DOM
    domState.nodes.forEach(ns => {
      const el = document.getElementById(ns.id);
      if (el) {
        el.classList.remove('pulse-affirm', 'pulse-reject', 'pulse-hold');
        if (ns.pulse) el.classList.add('pulse-' + ns.pulse);
        setNodeStatus(ns.id, ns.status);
      }
      const node = flowNodes.find(n => n.id === ns.id);
      if (node) node.props.status = ns.status;
    });

    domState.wires.forEach(ws => {
      const wire = flowWires.find(w => w.id === ws.id);
      if (wire) wire.signal = ws.signal;
    });

    document.querySelectorAll('.trit-particle-ghost').forEach(p => p.remove());
    updateWires();
    updateFogHeatmap();
  }

  // 2. Hardware-accelerated Canvas Overlay (Transient + Multiverse Ghosting)
  renderScrubLayer(time);
}

function renderScrubLayer(currentTime, scheduledEvents = [], loggedEvents = null) {
  try {
    const canvas = document.getElementById("scrub-layer");
    const wrap = document.getElementById("flow-canvas-wrap");
    if (!canvas || !wrap) return;

    const ctx = canvas.getContext("2d");
    if (canvas.width !== wrap.clientWidth || canvas.height !== wrap.clientHeight) {
      canvas.width = wrap.clientWidth; 
      canvas.height = wrap.clientHeight;
    }
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Use global variable if not passed (for scrubber dragging)
    const events = scheduledEvents.length > 0 ? scheduledEvents : (window.globalScheduledEvents || []);
    if (scheduledEvents.length > 0) window.globalScheduledEvents = scheduledEvents;

    // Track which nodes/wires are active at this millisecond
    const activeNodeIds = new Set();
    const activeWireStates = {}; // wireId -> { signal, alpha }

    events.forEach(event => {
      // 1. SIGNAL INTERPOLATION (Dots)
      const isSignalInFlight = (currentTime >= event.startTime && currentTime <= event.endTime);
      if (isSignalInFlight) {
        const frac = (currentTime - event.startTime) / event.duration;
        const wire = flowWires.find(w => w.id === event.wireId);
        if (wire) {
          activeWireStates[wire.id] = { signal: event.val, alpha: 1.0 };
          const svgPath = document.getElementById(wire.id);
          if (svgPath) {
            try {
              const totalLength = svgPath.getTotalLength();
              const pt = svgPath.getPointAtLength(frac * totalLength);
              const canvasX = pt.x * CT.scale + CT.x;
              const canvasY = pt.y * CT.scale + CT.y;
              const color = event.val === 1 ? '#22c55e' : (event.val === -1 ? '#ef4444' : '#f59e0b');
              
              ctx.beginPath();
              ctx.fillStyle = color;
              ctx.shadowColor = color;
              ctx.shadowBlur = 10 * CT.scale;
              const size = (6 + (8 * (event.conf || 1.0))) * CT.scale;
              ctx.arc(canvasX, canvasY, size, 0, Math.PI * 2);
              ctx.fill();
              ctx.shadowBlur = 0;
            } catch(e) {}
          }
        }
      }

      // 2. NODE ACTIVATION (Pulse)
      const nodeProcessingWindow = 150; 
      if (currentTime >= event.startTime - nodeProcessingWindow && currentTime <= event.startTime) {
         activeNodeIds.add(event.fromId);
      }
      if (currentTime >= event.endTime && currentTime <= event.endTime + nodeProcessingWindow) {
         activeNodeIds.add(event.toId);
      }

      // 3. LOG TRIGGER (Live Terminal Flow)
      if (loggedEvents && currentTime >= event.endTime && !loggedEvents.has(event.wireId + "_" + event.endTime)) {
        const toNode = flowNodes.find(n => n.id === event.toId);
        const fromNode = flowNodes.find(n => n.id === event.fromId);
        if (toNode) {
          const lbl = event.val === 1 ? '+1 (Affirm)' : (event.val === -1 ? '-1 (Reject)' : '0 (Tend)');
          logInspector(toNode.name, `Signal arrival from ${fromNode ? fromNode.name : 'ROOT'} -> ${lbl}`);
          loggedEvents.add(event.wireId + "_" + event.endTime);

          // JIT Artifact Spawning
          const outWiresCheck = flowWires.filter(w => w.fromId === event.toId);
          const targetNode = flowNodes.find(n => n.id === event.toId);
          if (outWiresCheck.length === 0 && targetNode && targetNode.type !== 'artifact') {
              spawnResultArtifact(targetNode, event.val);
          } else if (targetNode && targetNode.type === 'artifact') {
              // REPLAY LOGIC: Update existing artifact dynamically
              const artEl = document.getElementById(`art-body-${targetNode.id}`);
              if (artEl) {
                  const statusTxt = event.val === 1 ? 'AFFIRM' : (event.val === -1 ? 'REJECT' : 'TEND');
                  artEl.textContent = `Source: ${fromNode ? fromNode.name : 'Unknown'}\nResolved Signal: ${statusTxt}\nStatus: Resolved`;
                  artEl.style.color = event.val === 1 ? 'var(--green)' : (event.val === -1 ? 'var(--red)' : 'var(--text)');
              }
          }
        }
      }
    });

    // Update DOM visuals (Pulse & Wire colors)
    document.querySelectorAll('.flow-node').forEach(node => {
      const id = node.getAttribute('id');
      node.classList.remove('pulse-affirm', 'pulse-reject', 'pulse-hold');
      if (activeNodeIds.has(id)) {
         node.classList.add('pulse-affirm');
      }
    });

    flowWires.forEach(wire => {
      const state = activeWireStates[wire.id];
      const path = document.getElementById(wire.id);
      if (path) {
        path.classList.remove('active-1', 'active-n1', 'active-0');
        if (state) {
          path.classList.add(`active-${state.signal === 1 ? '1' : (state.signal === -1 ? 'n1' : '0')}`);
        }
      }
    });
  } catch (err) {
    if (window.TERNLANG_CRITICAL_DEBUG) {
      window.TERNLANG_CRITICAL_DEBUG.push({
        ts: performance.now(),
        msg: "renderScrubLayer Silent Crash Prevented",
        error: err.message
      });
    }
  }
}
function scrubSimulation(index) {
  requestScrub(index);
}
window.scrubSimulation = scrubSimulation;

function stopSimulation() {
  simulationAborted = true;
  simulationRunning = false;
  virtualClock = 0;
  lastRealTime = performance.now();
  showToast("Simulation stopping...", "warn");
  updateSimUI();

  // Clear canvas overlay immediately
  const canvas = document.getElementById("scrub-layer");
  if (canvas) {
    const ctx = canvas.getContext("2d");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
  }
  // Wipe ghost dots
  document.querySelectorAll('.trit-particle-ghost').forEach(p => p.remove());
}
window.stopSimulation = stopSimulation;

/**
 * Draggable Inspector Logic
 */
function initInspectorDraggable() {
  const ins = document.getElementById("flow-inspector");
  const head = ins.querySelector(".ins-head");
  const resizer = document.getElementById("flow-inspector-resizer");
  if (!ins || !head) return;

  let isDragging = false;
  let isResizing = false;
  let offsetX, offsetY;

  // Dragging
  head.onmousedown = (e) => {
    if (e.target.closest('button')) return;
    isDragging = true;
    
    // Smooth grab: calculate cursor distance from element top-left
    const rect = ins.getBoundingClientRect();
    offsetX = e.clientX - rect.left;
    offsetY = e.clientY - rect.top;
    
    // Switch from auto/centered styles to absolute pixels for movement
    ins.style.transform = "none";
    ins.style.bottom = "auto";
    ins.style.left = rect.left + "px";
    ins.style.top = rect.top + "px";
    
    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);
    e.preventDefault();
  };

  // Resizing
  let startResH = 0;
  let startResY = 0;
  if (resizer) {
    resizer.onmousedown = (e) => {
      isResizing = true;
      const rect = ins.getBoundingClientRect();
      offsetY = e.clientY - rect.top; // Store grab offset
      e.preventDefault();
      document.addEventListener("mousemove", onMouseMove);
      document.addEventListener("mouseup", onMouseUp);
    };
  }

  function onMouseMove(e) {
    if (isDragging) {
      ins.style.left = (e.clientX - offsetX) + "px";
      ins.style.top = (e.clientY - offsetY) + "px";
    } else if (isResizing) {
      const rect = ins.getBoundingClientRect();
      const newWidth  = Math.max(250, e.clientX - rect.left);
      const newHeight = Math.max(32, e.clientY - offsetY); 
      ins.style.width = newWidth + "px";
      ins.style.height = newHeight + "px";
    }
  }

  function onMouseUp() {
    isDragging = false;
    isResizing = false;
    document.removeEventListener("mousemove", onMouseMove);
    document.removeEventListener("mouseup", onMouseUp);
  }
}
window.initInspectorDraggable = initInspectorDraggable;

/**
 * Sidebar Resizer Logic
 */
function updateTimelineBridge() {
  const leftSidebar = document.getElementById("flow-library");
  const rightSidebar = document.getElementById("flow-props");
  const timeline = document.querySelector(".timeline-container");
  if (!leftSidebar || !rightSidebar || !timeline) return;

  const leftW = leftSidebar.offsetWidth;
  const rightW = rightSidebar.offsetWidth;
  
  timeline.style.left = leftW + "px";
  timeline.style.width = `calc(100% - ${leftW + rightW}px)`;
  
  // Also nudge canvas viewbox if needed
  if (window.fitToView) {
    // Only fit if explicitly needed, but sidebar change affects visible area
  }
}
window.updateTimelineBridge = updateTimelineBridge;

function initSidebarResizer() {
  const sidebar = document.getElementById("flow-library");
  const resizer = document.getElementById("flow-sidebar-resizer");
  if (!sidebar || !resizer) return;

  resizer.addEventListener("mousedown", (e) => {
    resizer.classList.add("active");
    const onMouseMove = (me) => {
      const newWidth = me.clientX;
      if (newWidth >= 180 && newWidth <= 500) {
        sidebar.style.width = newWidth + "px";
        updateTimelineBridge();
      }
    };
    const onMouseUp = () => {
      resizer.classList.remove("active");
      document.removeEventListener("mousemove", onMouseMove);
      document.removeEventListener("mouseup", onMouseUp);
    };
    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);
  });
  
  initRightSidebarResizer();
  
  // Use ResizeObserver for more robust telescoping
  const ro = new ResizeObserver(() => updateTimelineBridge());
  ro.observe(sidebar);
  const rs = document.getElementById("flow-props");
  if (rs) ro.observe(rs);
  
  updateTimelineBridge();
}
window.initSidebarResizer = initSidebarResizer;

function initRightSidebarResizer() {
  const sidebar = document.getElementById("flow-props");
  const resizer = document.getElementById("flow-props-resizer");
  if (!sidebar || !resizer) return;

  resizer.addEventListener("mousedown", (e) => {
    resizer.classList.add("active");
    
    const rect = sidebar.getBoundingClientRect();
    const offsetRight = e.clientX - rect.left; // Distance from the left edge of the sidebar (where the resizer is)

    const onMouseMove = (me) => {
      const windowWidth = window.innerWidth;
      const newWidth = windowWidth - me.clientX;
      if (newWidth >= 250 && newWidth <= 500) {
        sidebar.style.width = newWidth + "px";
        updateTimelineBridge();
      }
    };
    const onMouseUp = () => {
      resizer.classList.remove("active");
      document.removeEventListener("mousemove", onMouseMove);
      document.removeEventListener("mouseup", onMouseUp);
    };
    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);
  });
}
window.initRightSidebarResizer = initRightSidebarResizer;

function setNodeStatus(id, status) {
  const dot = document.getElementById("status-" + id);
  if (dot) { dot.className = "fn-status" + (status ? " " + status : ""); dot.title = status || "idle"; }
}
window.setNodeStatus = setNodeStatus;

async function executeLLMNode(node, inSignal) {
  let querySignal = (node.props.template || "{{input}}").replace("{{input}}", inSignal === 1 ? "affirm" : (inSignal === -1 ? "reject" : "tend"));
  const system = node.props.system_prompt || "You are a ternary logic processor. Output only +1 (affirm), 0 (tend), or -1 (reject).";

  const protocol = node.props.protocol || 'openai';
  const modelId = node.props.model_id || '';

  let finalPrompt = querySignal;

  // 1. Structural XML Wrapper
  if (node.props.runtime_buffer && node.props.runtime_buffer.data) {
    const injectedData = node.props.runtime_buffer.data;
    finalPrompt = `Here is the ingested data for your analysis:\n\n<context><data_payload>\n${injectedData}\n</data_payload></context>\n\nUser: ${querySignal}`;
  }

  // 2. Token Safety Check
  const estimatedTokens = Math.ceil((system.length + finalPrompt.length) / 4);
  let contextWindow = 8192; // Default fallback
  const lowerId = modelId.toLowerCase();
  
  if (lowerId.includes("gemini-1.5")) contextWindow = 1048576;
  else if (lowerId.includes("gemini")) contextWindow = 32768;
  else if (lowerId.includes("claude-3")) contextWindow = 200000;
  else if (lowerId.includes("gpt-4")) contextWindow = 128000;
  else if (lowerId.includes("gpt-3.5") || lowerId.includes("gpt-35")) contextWindow = 16384;
  else if (lowerId.includes("grok")) contextWindow = 131072;
  else if (protocol === "anthropic") contextWindow = 200000;
  else if (protocol === "google") contextWindow = 1048576;

  if (estimatedTokens > contextWindow * 0.8) {
    logInspector(node.name, `❌ Token Safety Halt: Payload (${estimatedTokens} tk) exceeds 80% of ${contextWindow} tk context window.`);
    return -1;
  }

  logInspector(node.name, `🌐 Calling LLM [${protocol}:${modelId || 'default'}] (${estimatedTokens} tk)…`);

  try {
    const response = await fetch('/api/run', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        source: `// LLM Bridge Proxy\nfn main() -> trit { return hold; }`, // Stub for security compatibility
        llm_config: {
          system,
          prompt: finalPrompt,
          protocol,
          model_id: modelId,
          api_key: node.props.api_key,
          base_url: node.props.base_url,
          temperature: node.props.temperature,
          max_tokens: node.props.max_trits
        }
      })
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const r = await response.json();
    
    // Mapping response text/result to trit with TAP Confidence
    let val = 0;
    let confidence = 0.5; // Default to 'Tend' (Ambiguity)
    
    const txt = String(r.result || r.text || "").toLowerCase();
    
    // Pattern detection for TAP Confidence
    if (txt.includes("confidence: 1.0") || txt.includes("conf: 1.0") || txt.includes("score: 1.0")) {
      confidence = 1.0;
    } else if (txt.includes("confidence: -1.0") || txt.includes("conf: -1.0")) {
      confidence = -1.0;
    }

    if (txt.includes("+1") || txt.includes("affirm") || txt.includes("yes")) val = 1;
    else if (txt.includes("-1") || txt.includes("reject") || txt.includes("no")) val = -1;

    // TAP Protocol: If confidence is 1.0, we proceed silently.
    // If confidence is 0.0 (or not 1.0), we force a 'tend' and flag for suspension.
    if (confidence === 1.0) {
      logInspector(node.name, `✅ TAP: High Confidence (+1) -> Autonomous Execution.`);
      return val;
    } else {
      logInspector(node.name, `🟡 TAP: Ambiguity detected (conf: ${confidence}) -> Freezing for Operator.`);
      node.props.pending_actuator = {
        code: r.code || r.result || r.text,
        paths: ["Affirm (+1)", "Reject (-1)"]
      };
      return 0; // Force State 0 (Hold)
    }
    logInspector(node.name, `🤖 LLM Result: ${val} (${txt.substring(0, 20)}…)`);
    return val;
  } catch (e) {
    logInspector(node.name, `❌ LLM Error: ${e.message}. Falling back to deterministic mapper.`);
    // Fallback heuristic: basic keyword mapping on intent
    if (node.props.system_prompt?.toLowerCase().includes("safety")) return 1; // Default safe
    return inSignal;
  }
}
window.executeLLMNode = executeLLMNode;

async function simulateNode(node, inSignal, isPhantom = false) {
  if (simulationAborted) return inSignal;
  const el = document.getElementById(node.id);
  if (!el && !isPhantom) return inSignal;

  if (!isPhantom) {
    setNodeStatus(node.id, "run");
    if (el) el.classList.remove('pulse-affirm','pulse-reject','pulse-hold');
  }

  let outSignal = inSignal;

  // Branch by type
  if (node.type === 'external') {
    outSignal = await executeLLMNode(node, inSignal);
  } else if (node.type === 'moe13') {
    outSignal = await executeMOE13(node, inSignal);
  } else if (node.type === 'datasource') {
    const payloadData = node.props.payload || "";
    if (!isPhantom) {
      logInspector(node.name, `📡 Injecting Payload: [${node.props.data_type || 'text'}] ${payloadData.substring(0, 20)}...`);
    }
    const outWires = flowWires.filter(w => w.fromId === node.id);
    outWires.forEach(w => {
      const downNode = flowNodes.find(n => n.id === w.toId);
      if (downNode) {
        downNode.props.runtime_buffer = { type: node.props.data_type || 'text', data: payloadData };
        if (!isPhantom) logInspector("SYSTEM", `💾 Buffered ${payloadData.length} bytes to ${downNode.name}`);
      }
    });
    outSignal = 1; // Affirm data dispatched
  } else {
    // Standard Agent or Gate
    const code = node.props.code || "";
    if (code.trim()) {
      if (!isPhantom) logInspector(node.name, "⚡ Executing logic…");
      const r = runTernCode(code);
      if (r.ok) {
        outSignal = r.trit === 1 ? 1 : (r.trit === -1 ? -1 : 0);
        const lbl = outSignal === 1 ? '+1 AFFIRM' : (outSignal === -1 ? '-1 REJECT' : '0 TEND');
        if (!isPhantom) logInspector(node.name, `→ ${lbl}${r.output && r.output.length ? ' · ' + r.output.join(', ') : ''}`);
      } else {
        if (!isPhantom) {
          logInspector(node.name, `✗ ${r.error || "error"}`);
          setNodeStatus(node.id, "err");
          if (el) el.classList.add('pulse-reject');
        }
        if (!isPhantom) await new Promise(r => setTimeout(r, 600));
        return -1;
      }
    } else {
      const lbl = inSignal === 1 ? '+1 AFFIRM' : (inSignal === -1 ? '-1 REJECT' : '0 TEND');
      if (!isPhantom) logInspector(node.name, `→ ${lbl} (passthrough)`);
    }
  }

  if (simulationAborted) return outSignal;
  
  if (!isPhantom) {
    const pulseClass = outSignal === 1 ? 'pulse-affirm' : (outSignal === -1 ? 'pulse-reject' : 'pulse-hold');
    if (el) el.classList.add(pulseClass);
    setNodeStatus(node.id, outSignal === 1 ? "ok" : (outSignal === -1 ? "err" : "run"));
  }
  
  // TAP Protocol: Detection of Pending Actuator (State 0 Suspension)
  if (node.props.pending_actuator) {
    if (!isPhantom) {
      logInspector("SYSTEM", `🟡 TAP: State 0 Suspension at "${node.name}". Awaiting Operator…`);
      simulationAborted = true; // Freeze graph
    }
    return 0; // Suspend
  }

  // Terminal Node Interceptor: Spawn/Update Result Artifact
  const outWires = flowWires.filter(w => w.fromId === node.id);
  const artifacts = flowNodes.filter(fn => fn.type === 'artifact' && fn.parentId === node.id);

  if (outWires.length === 0 && node.type !== 'artifact') {
    if (!isPhantom) {
      logInspector("SYSTEM", "🛑 Terminal Payload Detected — Hard Halt engaged.");
      simulationAborted = true; // Hard Halt
    }
  }
  
  // Update linked artifacts
  artifacts.forEach(target => {
    const artEl = document.getElementById(`art-body-${target.id}`);
    if (artEl) {
       artEl.textContent = `Source: ${node.name}\nResolved Signal: ${outSignal === 1 ? 'AFFIRM' : (outSignal === -1 ? 'REJECT' : 'TEND')}\nStatus: Halt Emitted`;
       artEl.style.color = outSignal === 1 ? 'var(--green)' : (outSignal === -1 ? 'var(--red)' : 'var(--text)');
    }
  });

  if (!isPhantom) await new Promise(r => setTimeout(r, 500));
  return outSignal;
}
window.simulateNode = simulateNode;

async function executeMOE13(node, inSignal) {
  logInspector(node.name, "🧠 MOE-13: Initiating Deliberation Cycle...");
  
  let weightedSum = 0;
  let safetyVeto = false;
  const id = node.id;
  
  const vetoAlert = document.getElementById(`moe-veto-alert-${id}`);
  const verdictEl = document.getElementById(`moe-verdict-${id}`);
  if (vetoAlert) vetoAlert.style.display = "none";
  if (verdictEl) { verdictEl.textContent = "DELIBERATING..."; verdictEl.style.color = "var(--magenta)"; }

  for (const axis of MOE13_AXES) {
    // Simulate complex expert deliberation
    await new Promise(r => setTimeout(r, 80 + Math.random() * 150));
    
    // Logic: experts are influenced by input signal but have internal biases
    let vote = 0;
    if (inSignal === 1) vote = Math.random() > 0.3 ? 1 : (Math.random() > 0.5 ? 0 : -1);
    else if (inSignal === -1) vote = Math.random() > 0.3 ? -1 : (Math.random() > 0.5 ? 0 : 1);
    else vote = Math.random() > 0.8 ? 1 : (Math.random() > 0.8 ? -1 : 0);
    
    const confidence = 0.6 + Math.random() * 0.39; // 60% to 99%
    
    // UI Update for Axis
    const voteEl = document.getElementById(`moe-vote-${id}-${axis.id}`);
    const confEl = document.getElementById(`moe-conf-${id}-${axis.id}`);
    
    if (voteEl) {
      voteEl.textContent = vote === 1 ? "+1" : (vote === -1 ? "-1" : "0");
      voteEl.style.color = vote === 1 ? "var(--green)" : (vote === -1 ? "var(--red)" : "var(--muted2)");
    }
    if (confEl) confEl.textContent = Math.round(confidence * 100) + "%";

    // Hard Safety Veto Logic
    if (axis.crit && vote === -1 && confidence > 0.90) {
      safetyVeto = true;
      if (vetoAlert) vetoAlert.style.display = "block";
      logInspector(node.name, `🛑 CRITICAL VETO: Axis [${axis.label}] rejected with ${(confidence*100).toFixed(1)}% confidence.`);
    }
    
    weightedSum += vote * axis.weight * (confidence * 2); // Confidence weighted EMA logic
  }

  let finalTrit = 0;
  if (safetyVeto) {
    finalTrit = -1;
  } else {
    // Threshold deliberation
    if (weightedSum > 0.2) finalTrit = 1;
    else if (weightedSum < -0.2) finalTrit = -1;
    else finalTrit = 0;
  }

  if (verdictEl) {
    verdictEl.textContent = finalTrit === 1 ? "AFFIRMED" : (finalTrit === -1 ? "REJECTED" : "TENDED");
    verdictEl.style.color = finalTrit === 1 ? "var(--green)" : (finalTrit === -1 ? "var(--red)" : "var(--amber)");
  }
  
  logInspector(node.name, `✓ Deliberation Complete. Verdict: ${finalTrit} (Score: ${weightedSum.toFixed(3)})`);
  return finalTrit;
}
window.executeMOE13 = executeMOE13;

function spawnResultArtifact(sourceNode, val) {
  let payloadStr = `Source: ${sourceNode.name}\nResolved Signal: ${val === 1 ? 'AFFIRM' : (val === -1 ? 'REJECT' : 'TEND')}\nStatus: Resolved`;
  
  const isSuspended = !!sourceNode.props.pending_actuator;
  if (isSuspended) {
    const tap = sourceNode.props.pending_actuator;
    if (tap.error) {
      payloadStr = `❌ WASM RUNTIME ERROR\n\nTraceback:\n${tap.error}\n\nFailed Code:\n${tap.last_failed_code}\n\nAction: Please edit in 'transmute' mode or reject.`;
    } else {
      payloadStr = `⚠️  TAP SUSPENSION (STATE 0)\n\nProposed Actuator Logic:\n${tap.code}\n\nDivergent Paths:\n- ${tap.paths.join('\n- ')}\n\nAction: Awaiting Operator Approval.`;
    }
  }
  
  // Singleton / Upsert pattern: Bind to parent UUID
  const existing = flowNodes.find(n => n.type === 'artifact' && n.parentId === sourceNode.id);
  
  if (existing) {
     existing.props.payload = payloadStr;
     const artEl = document.getElementById(`art-body-${existing.id}`);
     if (artEl) {
        artEl.textContent = payloadStr;
        artEl.style.color = isSuspended ? 'var(--amber)' : (val === 1 ? 'var(--green)' : (val === -1 ? 'var(--red)' : 'var(--text)'));
        if (isSuspended) renderActuatorControls(existing.id, sourceNode.id);
     }
     if (selectedNodeId === existing.id) updatePropertyPanel();
     return;
  }

  const sourceEl = document.getElementById(sourceNode.id);
  if (!sourceEl) return;
  const sx = parseFloat(sourceEl.style.left);
  const sy = parseFloat(sourceEl.style.top);
  
  const id = "art_" + Date.now();
  const name = isSuspended ? "TAP: " + sourceNode.name : "Result: " + sourceNode.name;
  
  // Requirement 1: Bounding-Box Collision Matrix
  const clearPos = findClearSpace(sx + 350, sy, 300, 200);
  
  createFlowNode(name, "__artifact__", clearPos.x, clearPos.y, 'artifact', id);
  
  // Requirement 2: Frontier Camera
  panToCenter(clearPos.x, clearPos.y);

  const newNode = flowNodes.find(n => n.id === id);
  if (newNode) {
    newNode.parentId = sourceNode.id;
    newNode.props.state = 'lock';
    newNode.props.payload = payloadStr;
  }

  const artEl = document.getElementById(`art-body-${id}`);
  if (artEl) {
     artEl.textContent = payloadStr;
     artEl.style.color = isSuspended ? 'var(--amber)' : (val === 1 ? 'var(--green)' : (val === -1 ? 'var(--red)' : 'var(--text)'));
     if (isSuspended) renderActuatorControls(id, sourceNode.id);
  }
  
  const wireId = "wire_art_" + Date.now();
  flowWires.push({
     id: wireId, fromId: sourceNode.id, toId: id,
     condition: "all", transform: "none", label: isSuspended ? "TAP PENDING" : "RESULT", priority: 10
  });
  updateWires();
  saveCanvasState();
  lucide.createIcons();
}

function renderActuatorControls(artId, sourceNodeId) {
  const artBody = document.getElementById(`art-body-${artId}`);
  if (!artBody) return;
  
  const ctrlDiv = document.createElement("div");
  ctrlDiv.style.marginTop = "12px";
  ctrlDiv.style.display = "flex";
  ctrlDiv.style.gap = "8px";
  
  ctrlDiv.innerHTML = `
    <button class="btn-pill" style="background:var(--green); color:white; border:none; padding:4px 12px; font-size:10px; cursor:pointer;" onclick="resolveTAP('${artId}', '${sourceNodeId}', 1)">Approve (+1)</button>
    <button class="btn-pill" style="background:var(--red); color:white; border:none; padding:4px 12px; font-size:10px; cursor:pointer;" onclick="resolveTAP('${artId}', '${sourceNodeId}', -1)">Reject (-1)</button>
  `;
  artBody.appendChild(ctrlDiv);
}

async function resolveTAP(artId, sourceNodeId, val) {
  const node = flowNodes.find(n => n.id === sourceNodeId);
  if (!node) return;

  const tap = node.props.pending_actuator;
  
  if (val === 1 && tap && tap.code) {
    logInspector(node.name, "⚙️  TAP Execution Loop: Initiating WASM Sandbox...");
    const res = await runPythonActuator(tap.code);
    
    if (res.ok) {
      logInspector(node.name, `✅ WASM Success: Output captured (${res.output.length} bytes).`);
      // Re-injection Loop: Feed result into buffer
      node.props.runtime_buffer = { type: "text", data: res.output };
      
      delete node.props.pending_actuator; // Release
      deleteNode(artId);
      injectSignal(sourceNodeId, 1);
    } else {
      logInspector(node.name, "❌ WASM Runtime Error: Reverting to State 0...");
      // Recursive Error State: Update the pending actuator with the error context
      node.props.pending_actuator.error = res.traceback || res.error;
      node.props.pending_actuator.last_failed_code = tap.code;
      
      // Re-summon the Artifact to display the traceback
      spawnResultArtifact(node, 0);
      updatePropertyPanel();
    }
  } else {
    // Standard Resolve (Reject or Manual Overwrite)
    delete node.props.pending_actuator;
    logInspector("SYSTEM", `✅ TAP RESOLVED: Operator injected ${val === 1 ? '+1' : '-1'} to "${node.name}".`);
    deleteNode(artId);
    injectSignal(sourceNodeId, val);
  }
}
window.resolveTAP = resolveTAP;

function updateArtifactPayload(id, val) {
  const node = flowNodes.find(n => n.id === id);
  if (!node) return;
  node.props.payload = val;
  
  // TAP Integration: If this artifact is linked to a suspended actuator, sync the code
  if (node.parentId) {
    const parent = flowNodes.find(p => p.id === node.parentId);
    if (parent && parent.props.pending_actuator) {
      parent.props.pending_actuator.code = val;
    }
  }
}
window.updateArtifactPayload = updateArtifactPayload;

function setArtifactState(id, state) {
  const node = flowNodes.find(n => n.id === id);
  if (!node) return;
  node.props.state = state;
  
  const el = document.getElementById(id);
  const display = document.getElementById(`art-body-${id}`);
  const editor  = document.getElementById(`art-edit-${id}`);
  const socketLabel = document.getElementById(`art-socket-label-${id}`);
  
  // Reset classes
  el.classList.remove('state-lock', 'state-transmute', 'state-extend');
  el.classList.add(`state-${state}`);
  
  // UI Sync
  if (state === 'lock') {
    display.style.display = 'block';
    editor.style.display  = 'none';
    socketLabel.style.display = 'none';
    if (editor.value) {
      display.textContent = editor.value;
      updateArtifactPayload(id, editor.value);
    }
  } else if (state === 'transmute') {
    display.style.display = 'none';
    editor.style.display  = 'block';
    socketLabel.style.display = 'none';
    editor.value = display.textContent;
  } else if (state === 'extend') {
    display.style.display = 'block';
    editor.style.display  = 'none';
    socketLabel.style.display = 'flex';
  }
  
  // Update Buttons
  el.querySelectorAll('.art-btn').forEach(b => b.classList.remove('active'));
  const btnIdx = state === 'lock' ? 0 : (state === 'transmute' ? 1 : 2);
  const btns = el.querySelectorAll('.art-btn');
  if (btns[btnIdx]) btns[btnIdx].classList.add('active');
  
  updateWires();
  saveCanvasState();
}
window.setArtifactState = setArtifactState;

function clearResultArtifacts() {
  const artifactNodes = flowNodes.filter(n => n.type === 'artifact');
  artifactNodes.forEach(node => {
     // Remove wires connected to/from this artifact
     flowWires = flowWires.filter(w => w.fromId !== node.id && w.toId !== node.id);
     // Remove the node element from DOM
     const el = document.getElementById(node.id);
     if (el) el.remove();
  });
  // Update state array
  flowNodes = flowNodes.filter(n => n.type !== 'artifact');
  updateWires();
}
window.clearResultArtifacts = clearResultArtifacts;

function updateWires() {
  updateFogHeatmap(); // Sync fog to node positions
  let svg = document.getElementById("flow-svg-layer");
  if (!svg) {
    const canvas = document.getElementById("flow-canvas");
    if (!canvas) return;
    svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.id = "flow-svg-layer";
    canvas.prepend(svg);
  }
  svg.innerHTML = "";
  // Clear edge badges
  document.querySelectorAll(".edge-badge").forEach(b => b.remove());

  // Manage visibility of ports for nodes with outgoing connections (critical for artifacts)
  flowNodes.forEach(n => {
    const el = document.getElementById(n.id);
    if (el) {
       const hasOut = flowWires.some(w => w.fromId === n.id);
       if (hasOut) el.classList.add('has-output');
       else el.classList.remove('has-output');
    }
  });

  flowWires.forEach(w => {
    const fromNode = document.getElementById(w.fromId);
    const toNode = document.getElementById(w.toId);
    if (!fromNode || !toNode) return;

    const fromPort = fromNode.querySelector('.flow-port-out');
    const toPort = toNode.querySelector('.flow-port-in');
    if (!fromPort || !toPort) return;

    const start = getPortPos(fromPort);
    const end = getPortPos(toPort);
    drawWire(start, end, w.id, w.signal, w, w.confidence);
  });

  if (activeWire) {
    const start = activeWire.start;
    const end = activeWire.end;
    
    // Draw the preview wire (active wire)
    drawWire(start, end, 'active-wire', 0, null);

    // INTERACTIVE GHOST: Only if dragging from an output port of an artifact in 'extend' state
    const sourceNode = flowNodes.find(n => n.id === activeWire.fromId);
    let ghost = document.getElementById("evolution-ghost");
    
    if (sourceNode && sourceNode.type === 'artifact' && sourceNode.props.state === 'extend' && activeWire.fromIsOutput) {
       if (!ghost) {
          ghost = document.createElement("div");
          ghost.id = "evolution-ghost";
          ghost.className = "flow-node agent ghost-node";
          ghost.innerHTML = `
            <div class="fn-head" style="opacity:0.6; pointer-events:none;">
              <div style="display:flex; align-items:center; gap:6px; font-size:10px; font-weight:700; color:var(--cyan)">
                <i data-lucide="bot" style="width:12px"></i>TRANSMUTED
              </div>
            </div>
            <div class="fn-body" style="padding:10px; font-size:9px; color:var(--muted); text-align:center; pointer-events:none;">RELEASE TO EVOLVE</div>
          `;
          document.getElementById("flow-canvas").appendChild(ghost);
          lucide.createIcons();
       }
       // Center ghost on current cursor position
       ghost.style.left = (end.x - 90) + "px";
       ghost.style.top  = (end.y - 40) + "px";
       ghost.style.display = "block";
       
       // Force preview wire to be highly visible
       const pw = document.getElementById("active-wire");
       if (pw) {
         pw.style.stroke = "var(--cyan)";
         pw.style.strokeWidth = "3";
         pw.style.strokeDasharray = "5 3";
         pw.style.opacity = "0.8";
       }
    } else if (ghost) {
       ghost.style.display = "none";
    }
  } else {
    const ghost = document.getElementById("evolution-ghost");
    if (ghost) ghost.style.display = "none";
  }
}
window.updateWires = updateWires;

function computeWirePath(start, end, wire) {
  const dx = end.x - start.x;
  const dy = end.y - start.y;

  // Use single manual control point (Quadratic Bezier)
  if (wire && wire.cp) {
    return `M ${start.x} ${start.y} Q ${wire.cp.x} ${wire.cp.y} ${end.x} ${end.y}`;
  }

  // Requirement 3: Obstacle Avoidance
  // Check if linear path intersects any node
  let obstacle = null;
  const fromId = wire ? wire.fromId : null;
  const toId   = wire ? wire.toId   : null;

  for (const node of flowNodes) {
    if (node.id === fromId || node.id === toId) continue;
    
    const nw = node.type === 'artifact' ? 300 : (node.type === 'moe13' ? 320 : 180);
    const nh = node.type === 'artifact' ? 200 : (node.type === 'moe13' ? 360 : 80);
    const padding = 20;
    
    // Node bounds
    const x1 = node.x - nw/2 - padding;
    const x2 = node.x + nw/2 + padding;
    const y1 = node.y - nh/2 - padding;
    const y2 = node.y + nh/2 + padding;

    // Simple segment-box intersection check
    // We check midpoint of the wire as a heuristic for raycasting
    const mx = start.x + dx/2;
    const my = start.y + dy/2;
    if (mx > x1 && mx < x2 && my > y1 && my < y2) {
      obstacle = node;
      break;
    }
  }

  if (obstacle) {
    // Inject waypoint: route above or below based on dy
    const nw = obstacle.type === 'artifact' ? 300 : (obstacle.type === 'moe13' ? 320 : 180);
    const nh = obstacle.type === 'artifact' ? 200 : (obstacle.type === 'moe13' ? 360 : 80);
    const wayY = start.y < obstacle.y ? obstacle.y - nh/2 - 40 : obstacle.y + nh/2 + 40;
    return `M ${start.x} ${start.y} Q ${obstacle.x} ${wayY} ${end.x} ${end.y}`;
  }

  // Cycle / Backward connection: push control points horizontally to create a large round loop
  if (dx < 60) {
    const horizontalOffset = Math.max(120, Math.abs(dy) * 0.4);
    return `M ${start.x} ${start.y} C ${start.x + horizontalOffset} ${start.y}, ${end.x - horizontalOffset} ${end.y}, ${end.x} ${end.y}`;
  }
  // Standard forward connection: smooth S-curve
  const curve = dx * 0.5;
  return `M ${start.x} ${start.y} C ${start.x + curve} ${start.y}, ${end.x - curve} ${end.y}, ${end.x} ${end.y}`;
}
window.computeWirePath = computeWirePath;

async function animateSignal(wire, signal, confidence = 1.0) {
  const fromNode = document.getElementById(wire.fromId);
  const toNode = document.getElementById(wire.toId);
  if (!fromNode || !toNode) return;

  const fromPort = fromNode.querySelector('.flow-port-out');
  const toPort = toNode.querySelector('.flow-port-in');
  const start = getPortPos(fromPort);
  const end = getPortPos(toPort);

  // Highlight wire with confidence
  drawWire(start, end, wire.id, signal, wire, confidence);
}window.animateSignal = animateSignal;

function getPortPos(port) {
  const rect = port.getBoundingClientRect();
  const wrapRect = document.getElementById("flow-canvas-wrap").getBoundingClientRect();
  return {
    x: (rect.left - wrapRect.left - CT.x + rect.width / 2) / CT.scale,
    y: (rect.top - wrapRect.top - CT.y + rect.height / 2) / CT.scale
  };
}
window.getPortPos = getPortPos;

function drawWire(start, end, id, signal, wire, confidence = 1.0) {
  const svg = document.getElementById("flow-svg-layer");
  const d = computeWirePath(start, end, wire);
  
  let path = svg.querySelector(`path[id="${id}"]`);
  let isNew = false;
  if (!path) {
    path = document.createElementNS("http://www.w3.org/2000/svg", "path");
    path.setAttribute("id", id);
    isNew = true;
  }

  path.setAttribute("d", d);
  path.setAttribute("data-wire-id", id);

  let cls = "flow-wire";
  if (signal !== undefined) cls += ` active-${signal === 1 ? '1' : (signal === -1 ? 'n1' : '0')}`;
  if (wire && wire.condition && wire.condition !== "all") cls += " cond-" + wire.condition.replace("!","");
  if (id === selectedWireId) cls += " selected-wire";
  path.setAttribute("class", cls);

  if (wire && wire.customColor && signal === undefined) {
    path.style.stroke = wire.customColor;
  } else {
    path.style.stroke = ""; // Use CSS defaults
  }

  path.style.opacity = 0.2 + (0.8 * confidence);
  path.style.strokeWidth = 1.5 + (2.5 * confidence);
  path.style.transition = "opacity 0.3s, stroke-width 0.3s, stroke 0.3s";

  if (id === 'active-wire') {
    path.style.pointerEvents = 'none';
  }

  if (isNew && id !== 'active-wire' && wire) {
    path.style.pointerEvents = 'stroke';
    path.addEventListener("click", (e) => { e.stopPropagation(); selectWire(id); showWireHandles(id); });
    const hit = document.createElementNS("http://www.w3.org/2000/svg", "path");
    hit.setAttribute("d", d);
    hit.setAttribute("fill", "none");
    hit.setAttribute("stroke", "transparent");
    hit.setAttribute("stroke-width", "18");
    hit.setAttribute("class", "wire-hit");
    hit.id = "hit-" + id;
    hit.style.pointerEvents = 'stroke';
    hit.addEventListener("click", (e) => { e.stopPropagation(); selectWire(id); showWireHandles(id); });
    svg.appendChild(hit);
  } else if (!isNew && wire) {
     const hit = document.getElementById("hit-" + id);
     if (hit) hit.setAttribute("d", d);
  }

  if (isNew) svg.appendChild(path);

  // Position label/handle at the ACTUAL midpoint of the curve
  let midX, midY;
  if (wire && wire.cp) {
    // Quadratic Bezier midpoint at t=0.5: P = (1-t)^2*P0 + 2(1-t)t*P1 + t^2*P2
    midX = 0.25 * start.x + 0.5 * wire.cp.x + 0.25 * end.x;
    midY = 0.25 * start.y + 0.5 * wire.cp.y + 0.25 * end.y;
  } else {
    midX = (start.x + end.x) / 2;
    midY = (start.y + end.y) / 2;
    const dx = end.x - start.x;
    if (dx < 60) midX += 60; // Offset for cycles
  }

  // Edge label badge
  if (wire && wire.label) {
    let badge = document.getElementById("badge-" + id);
    if (!badge) {
      badge = document.createElement("div");
      badge.id = "badge-" + id;
      const canvas = document.getElementById("flow-canvas");
      if (canvas) canvas.appendChild(badge);
    }
    badge.className = "edge-badge" + (wire.condition && wire.condition !== "all" ? " cond-" + wire.condition.replace("!","") : "");
    badge.style.left = midX + "px";
    badge.style.top  = midY + "px";
    badge.textContent = wire.label;
    badge.style.opacity = 0.4 + (0.6 * confidence);
  }

  // Sync handle if active
  if (id === selectedWireId) {
    const h = document.getElementById("wire-handle");
    h.style.left = midX + "px";
    h.style.top = midY + "px";
  }
}
window.drawWire = drawWire;

function showWireHandles(wireId) {
  const wire = flowWires.find(w => w.id === wireId);
  if (!wire) return;

  const fromNode = document.getElementById(wire.fromId);
  const toNode = document.getElementById(wire.toId);
  if (!fromNode || !toNode) return;

  const start = getPortPos(fromNode.querySelector('.flow-port-out'));
  const end = getPortPos(toNode.querySelector('.flow-port-in'));

  // Initialize control point if missing (center of P0 and P2)
  if (!wire.cp) {
    wire.cp = { x: (start.x + end.x) / 2, y: (start.y + end.y) / 2 };
  }

  const h = document.getElementById("wire-handle");
  // Visual midpoint (as calculated in drawWire)
  const mx = 0.25 * start.x + 0.5 * wire.cp.x + 0.25 * end.x;
  const my = 0.25 * start.y + 0.5 * wire.cp.y + 0.25 * end.y;

  h.style.left = mx + "px"; h.style.top = my + "px";
  h.classList.add("active");

  setupHandleDrag(h, (nx, ny) => {
    // Convert visual midpoint back to Quadratic Control Point
    // P1 = (Mid - 0.25*P0 - 0.25*P2) / 0.5
    wire.cp.x = (nx - 0.25 * start.x - 0.25 * end.x) / 0.5;
    wire.cp.y = (ny - 0.25 * start.y - 0.25 * end.y) / 0.5;
    updateWires();
  });
}
window.showWireHandles = showWireHandles;

function setupHandleDrag(el, onDrag) {
  el.onmousedown = (e) => {
    e.stopPropagation();
    const startX = e.clientX, startY = e.clientY;
    const initialX = parseFloat(el.style.left), initialY = parseFloat(el.style.top);

    const move = (me) => {
      const dx = (me.clientX - startX) / CT.scale;
      const dy = (me.clientY - startY) / CT.scale;
      const nx = initialX + dx, ny = initialY + dy;
      onDrag(nx, ny);
    };
    const up = () => {
      document.removeEventListener("mousemove", move);
      document.removeEventListener("mouseup", up);
      saveCanvasState();
    };
    document.addEventListener("mousemove", move);
    document.addEventListener("mouseup", up);
  };
}
window.setupHandleDrag = setupHandleDrag;

function exportFlowCode() {
  if (flowNodes.length === 0) {
    showToast("No nodes to export", "error");
    return;
  }

  // --- Topological Sort (Kahn's Algorithm) ---
  const sortedNodes = [];
  const inDegree = {};
  const adjacency = {};
  
  flowNodes.forEach(n => {
    inDegree[n.id] = 0;
    adjacency[n.id] = [];
  });

  flowWires.forEach(w => {
    if (adjacency[w.fromId] && inDegree[w.toId] !== undefined) {
      adjacency[w.fromId].push(w.toId);
      inDegree[w.toId]++;
    }
  });

  const queue = flowNodes.filter(n => inDegree[n.id] === 0).map(n => n.id);
  while (queue.length > 0) {
    const uId = queue.shift();
    const node = flowNodes.find(n => n.id === uId);
    if (node) sortedNodes.push(node);
    
    (adjacency[uId] || []).forEach(vId => {
      inDegree[vId]--;
      if (inDegree[vId] === 0) queue.push(vId);
    });
  }

  // Fallback for cycles or disconnected components
  const remaining = flowNodes.filter(n => !sortedNodes.find(sn => sn.id === n.id));
  sortedNodes.push(...remaining);

  let code = "// Generated by TernFlow Orchestrator\n";
  code += "// Swarm definition: Topological Order Optimized\n\n";
  
  const imports = new Set();
  flowNodes.forEach(n => { if (n.type === 'agent') imports.add(n.path); });
  if (flowNodes.some(n => n.type === 'external')) imports.add("stdlib/agents/binary_bridge.tern");
  
  imports.forEach(p => { 
    if (p) code += `// from \"${p}\" import *;\n`; 
  });
  
  code += "\nfn main() -> trit {\n";
  
  // Spawning Phase
  sortedNodes.forEach(n => {
    const varName = n.id.replace(/node_|bridge_|gate_/g, 'a');
    if (n.type === 'agent') code += `    let ${varName}: agentref = spawn ${n.name};\n`;
    else if (n.type === 'external') code += `    let ${varName}: agentref = spawn LLMGateway;\n`;
    else code += `    let ${varName}: agentref = spawn TritVote;\n`;
  });
  
  code += "\n    // Execution logic (Dependency Aware)\n";
  const nodeResults = {}; // Map node ID to result variable name

  sortedNodes.forEach(n => {
    const varName = n.id.replace(/node_|bridge_|gate_/g, 'a');
    const incoming = flowWires.filter(w => w.toId === n.id);
    
    if (incoming.length === 0) {
      // Root node: start with affirm
      code += `    send ${varName} affirm;\n`;
    } else {
      // Aggregate inputs (Simple consensus for now)
      incoming.forEach((w, idx) => {
        const sourceRes = nodeResults[w.fromId] || "affirm";
        code += `    send ${varName} ${sourceRes}; // from ${w.fromId}\n`;
      });
    }
    
    const resVar = `res_${varName}`;
    code += `    let ${resVar}: trit = await ${varName};\n`;
    nodeResults[n.id] = resVar;
  });

  const lastNode = sortedNodes[sortedNodes.length - 1];
  const finalVar = lastNode ? nodeResults[lastNode.id] : "affirm";
  
  code += `\n    return ${finalVar};\n}\n`;

  newFile();
  if (monacoEditor) monacoEditor.setValue(code);
  switchView("editor");
  showToast("Swarm exported to Editor (Topo-Sorted)", "ok");
}
window.exportFlowCode = exportFlowCode;

// ─── Deploy as Product ────────────────────────────────────────────────────────
function openDeployModal() {
  if (flowNodes.length === 0) { showToast("Add agents to the canvas first", "error"); return; }
  // Pre-fill name from first node
  const firstName = flowNodes[0]?.name || "";
  document.getElementById("deployName").value = firstName;
  document.getElementById("deployDesc").value = "";
  document.getElementById("deployInput").value = "";
  document.getElementById("deploy-progress").style.display = "none";
  document.getElementById("deploy-confirm-btns").style.display = "flex";
  document.getElementById("deploy-result").style.display = "none";
  deployStep(1);
  document.getElementById("deployModal").style.display = "flex";
  lucide.createIcons();
}
window.openDeployModal = openDeployModal;

function closeDeployModal() {
  document.getElementById("deployModal").style.display = "none";
}
window.closeDeployModal = closeDeployModal;

function deployStep(n) {
  [1,2,3].forEach(i => {
    document.getElementById("deploy-step-" + i).style.display = i === n ? "flex" : "none";
    const tab = document.getElementById("deploy-step-" + i + "-tab");
    if (tab) tab.style.cssText = i === n
      ? "flex:1;padding:8px;text-align:center;font-size:10px;font-weight:700;background:rgba(1,118,211,0.2);color:var(--blue);border-right:" + (i<3?"1px solid var(--border2)":"none")
      : "flex:1;padding:8px;text-align:center;font-size:10px;font-weight:700;color:var(--muted2);border-right:" + (i<3?"1px solid var(--border2)":"none");
  });
  if (n === 3) buildDeploySummary();
}
window.deployStep = deployStep;

function buildDeploySummary() {
  const name    = document.getElementById("deployName").value.trim() || "my-agent";
  const desc    = document.getElementById("deployDesc").value.trim() || "—";
  const input   = document.getElementById("deployInput").value.trim() || "{ signal: trit }";
  const pricing = document.querySelector('input[name="deployPricing"]:checked')?.value || "free";
  const slug    = name.toLowerCase().replace(/\s+/g, "-").replace(/[^a-z0-9-]/g, "");
  const nodeCount = flowNodes.length;
  const wireCount = flowWires.length;

  document.getElementById("deploy-summary").innerHTML = `
    <div style="color:var(--muted2);font-size:10px;margin-bottom:8px;letter-spacing:0.05em;">DEPLOYMENT PLAN</div>
    <div><span style="color:var(--muted)">name:    </span><span style="color:var(--cyan)">${name}</span></div>
    <div><span style="color:var(--muted)">slug:    </span><span style="color:var(--text)">/api/agent/${slug}</span></div>
    <div><span style="color:var(--muted)">nodes:   </span><span style="color:var(--text)">${nodeCount} agents, ${wireCount} wires</span></div>
    <div><span style="color:var(--muted)">pricing: </span><span style="color:${pricing==="free"?"var(--green)":pricing==="private"?"var(--muted)":"var(--amber)"}">${pricing}</span></div>
    <div><span style="color:var(--muted)">input:   </span><span style="color:var(--muted2);font-size:11px;">${input}</span></div>
    <div><span style="color:var(--muted)">desc:    </span><span style="color:var(--muted2);font-size:11px;">${desc}</span></div>
  `;
}
window.buildDeploySummary = buildDeploySummary;

async function executeProductDeploy() {
  // GC sweep: clear old visual results before building deployment
  clearResultArtifacts();

  const name    = document.getElementById("deployName").value.trim();
  const desc    = document.getElementById("deployDesc").value.trim();
  const input   = document.getElementById("deployInput").value.trim();
  const pricing = document.querySelector('input[name="deployPricing"]:checked')?.value || "free";
  const slug    = name.toLowerCase().replace(/\s+/g, "-").replace(/[^a-z0-9-]/g, "");

  if (!name) { showToast("Enter a product name", "error"); deployStep(1); return; }

  // Hide confirm, show progress
  document.getElementById("deploy-confirm-btns").style.display = "none";
  const prog = document.getElementById("deploy-progress");
  prog.style.display = "flex";

  const setStep = (id, state, msg) => {
    const row = document.getElementById(id);
    if (!row) return;
    row.className = "deploy-step-row" + (state === "done" ? " done" : state === "error" ? " error" : "");
    const icon = state === "done" ? "✓" : state === "error" ? "✗" : "⏳";
    row.innerHTML = `<span class="dstep-icon">${icon}</span> ${msg}`;
  };

  // Step 1: Compile flow to .tern code
  await new Promise(r => setTimeout(r, 600));
  setStep("dstep-compile", "done", "Flow compiled to .tern");

  // Generate the agent code from the canvas
  let flowCode = `// Deployed by TernStudio — ${name}\n`;
  flowCode += `// Nodes: ${flowNodes.length} · Wires: ${flowWires.length}\n\n`;
  flowNodes.forEach(n => {
    flowCode += `// Agent: ${n.name}\n`;
    if (n.props.code) flowCode += n.props.code + "\n\n";
  });

  // Step 2: Register endpoint
  await new Promise(r => setTimeout(r, 800));
  try {
    const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
    const key = document.getElementById("apiKey").value.trim();
    const payload = { name, slug, desc, input_schema: input, pricing, nodes: flowNodes.length, wires: flowWires.length, code: flowCode };
    const r = await fetch(endpoint + "/api/agents/publish", {
      method: "POST",
      headers: { "Content-Type": "application/json", ...(key ? { "X-Ternlang-Key": key } : {}) },
      body: JSON.stringify(payload),
    });
    const data = await r.json();
    if (r.ok && data.status === "ok") {
      setStep("dstep-register", "done", `Endpoint registered: /api/agent/${slug}`);
      await new Promise(r => setTimeout(r, 600));
      setStep("dstep-publish", "done", "Live on runtime ✓");
      const resultEl = document.getElementById("deploy-result");
      resultEl.style.display = "block";
      resultEl.innerHTML = `
       <div style="font-size:13px;font-weight:700;color:var(--green);margin-bottom:8px;">🚀 Deployed successfully!</div>
       <div style="font-family:'JetBrains Mono',monospace;font-size:11px;color:var(--text);line-height:2;">
         <div>Endpoint: <span style="color:var(--cyan)">${endpoint}/api/agent/${slug}</span></div>
         <div>Pricing: <span style="color:var(--amber)">${pricing}</span></div>
         <div style="margin-top:8px;font-size:10px;color:var(--muted2);">Share your endpoint with consumers — they need an API key to call it.</div>
       </div>
      `;

      // Sync with Fleet registry
      let registry = [];
      try { registry = JSON.parse(localStorage.getItem("ternflow_registry") || "[]"); } catch(e){}
      if (!registry.find(r => r.slug === slug)) {
       registry.push({ id: slug, slug, name, desc, pricing, nodes: flowNodes.length, deployed: new Date().toISOString() });
       localStorage.setItem("ternflow_registry", JSON.stringify(registry));
      }

      showToast(`Deployed: ${name}`, "ok");
    } else {
      throw new Error(data.error || "Server error");
    }
  } catch(err) {
    console.warn("Deploy error, saving locally:", err);
    // Simulate success in dev
    setStep("dstep-register", "done", `Registered locally`);
    await new Promise(r => setTimeout(r, 500));
    setStep("dstep-publish", "done", "Saved to local fleet registry");
    const resultEl = document.getElementById("deploy-result");
    resultEl.style.display = "block";
    const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
    resultEl.innerHTML = `
    <div style="font-size:13px;font-weight:700;color:var(--amber);margin-bottom:8px;">⚠ Saved locally</div>
    <div style="font-family:'JetBrains Mono',monospace;font-size:11px;color:var(--text);line-height:1.8;">
      Endpoint: <span style="color:var(--cyan)">${endpoint}/api/agent/${slug}</span><br>
      Status: <span style="color:var(--amber)">Local-Only (Sync Pending)</span>
    </div>
    `;
    showToast(`${name} saved to Fleet`, "ok");
  }

  // Mandatory Local Persistence (Always save to local registry for Fleet Tab)
  let registry = [];
  try { registry = JSON.parse(localStorage.getItem("ternflow_registry") || "[]"); } catch(e){}
  if (!registry.find(r => r.slug === slug)) {
    registry.push({ 
      id: slug, 
      slug, 
      name, 
      desc, 
      pricing, 
      nodes: flowNodes.length, 
      deployed: new Date().toISOString() 
    });
    localStorage.setItem("ternflow_registry", JSON.stringify(registry));
  }
}
window.executeProductDeploy = executeProductDeploy;
function onMouseDown(e) {
  if (e.button !== 0) return; // Only left-click starts wires
  
  const sourcePort = e.target.closest('.flow-port');
  if (sourcePort) {
    const nodeEl = sourcePort.closest('.flow-node');
    const wrapRect = document.getElementById("flow-canvas-wrap").getBoundingClientRect();
    activeWire = {
      fromId: nodeEl.id,
      fromIsOutput: sourcePort.classList.contains('flow-port-out'),
      start: getPortPos(sourcePort),
      end: screenToCanvas(e.clientX - wrapRect.left, e.clientY - wrapRect.top)
    };
  } else {
    // Clear state if clicking empty space (prevent stuck wires)
    activeWire = null;
  }
}
document.addEventListener('mousedown', onMouseDown);
window.onMouseDown = onMouseDown;

function onMouseMove(e) {
  const wrapRect = document.getElementById("flow-canvas-wrap").getBoundingClientRect();

  if (activeWire) {
    activeWire.end = screenToCanvas(e.clientX - wrapRect.left, e.clientY - wrapRect.top);

    // Magnetism: highlight nearest valid input port
    document.querySelectorAll('.flow-port-in').forEach(p => p.classList.remove('magnet'));
    let nearest = null, minDist = 40;
    document.querySelectorAll('.flow-port-in').forEach(p => {
      const pr = p.getBoundingClientRect();
      const d = Math.sqrt(Math.pow(e.clientX - (pr.left + pr.width/2), 2) + Math.pow(e.clientY - (pr.top + pr.height/2), 2));
      if (d < minDist) { minDist = d; nearest = p; }
    });
    if (nearest) nearest.classList.add('magnet');
    updateWires();
  } else if (isDraggingNode && nodeDraggingId) {
    const dx = (e.clientX - startMouseX) / CT.scale;
    const dy = (e.clientY - startMouseY) / CT.scale;

    if (selectedIds.size > 1) {
      selectedIds.forEach(sid => {
        const sel = document.getElementById(sid);
        const off = multiDragOffsets[sid];
        if (sel && off) {
          sel.style.left = (off.x + dx) + "px";
          sel.style.top  = (off.y + dy) + "px";
        }
      });
    } else {
      const el = document.getElementById(nodeDraggingId);
      const off = multiDragOffsets[nodeDraggingId];
      if (el && off) {
        el.style.left = (off.x + dx) + "px";
        el.style.top  = (off.y + dy) + "px";
      }
    }
    updateWires();
  }
}
document.addEventListener('mousemove', onMouseMove);
window.onMouseMove = onMouseMove;

function onMouseUp(e) {
  // --- Node Dragging End ---
  if (isDraggingNode) {
    if (selectedIds.size > 1) {
      selectedIds.forEach(sid => {
        const selNode = flowNodes.find(fn => fn.id === sid);
        const selEl = document.getElementById(sid);
        if (selNode && selEl) {
          selNode.x = parseFloat(selEl.style.left) + (parseFloat(selEl.style.width)||180)/2; // logic is center-based
          selNode.y = parseFloat(selEl.style.top) + (parseFloat(selEl.style.height)||80)/2;
        }
      });
    } else if (nodeDraggingId) {
      const fn = flowNodes.find(f => f.id === nodeDraggingId);
      const el = document.getElementById(nodeDraggingId);
      if (fn && el) {
        const nw = fn.type === 'artifact' ? 300 : (fn.type === 'moe13' ? 320 : 180);
        const nh = fn.type === 'artifact' ? 200 : (fn.type === 'moe13' ? 360 : 80);
        fn.x = parseFloat(el.style.left) + nw/2;
        fn.y = parseFloat(el.style.top) + nh/2;
      }
    }
    if (nodeDraggingId) {
      const el = document.getElementById(nodeDraggingId);
      if (el) el.style.zIndex = 10;
    }
    isDraggingNode = false;
    nodeDraggingId = null;
    saveCanvasState();
  }

  // --- Wire Dragging / Evolution End ---
  if (activeWire) {
    try {
      const magnet = document.querySelector('.flow-port.magnet');
      const targetPort = magnet || e.target.closest('.flow-port');

      if (targetPort) {
        const toNode = targetPort.closest('.flow-node');
        if (toNode && toNode.id !== activeWire.fromId) {
          const targetIsOutput = targetPort.classList.contains('flow-port-out');
          if (activeWire.fromIsOutput !== targetIsOutput) {
            const wireId = "wire_" + Date.now();
            const fromId = activeWire.fromIsOutput ? activeWire.fromId : toNode.id;
            const toId   = activeWire.fromIsOutput ? toNode.id : activeWire.fromId;
            flowWires.push({
              id: wireId, fromId, toId,
              signal: 0, confidence: 1.0, condition: "all", transform: "pass", priority: 5, label: "All signals"
            });
            saveCanvasState();

            const fromNode = flowNodes.find(n => n.id === fromId);
            if (fromNode && fromNode.type === 'artifact') collapseArtifactToStub(fromId);

            if (simulationAborted && !simulationRunning) {
               logInspector("SYSTEM", "🔌 Continuation Detected — Resuming Engine.");
               resumeSimulationFrom(toId);
            }
          }
        }
      } else if (activeWire.fromIsOutput) {
        // EDGE-DRIVEN EVOLUTION
        const sourceNode = flowNodes.find(n => n.id === activeWire.fromId);
        
        const wrapRect = document.getElementById("flow-canvas-wrap").getBoundingClientRect();
        const currentPos = screenToCanvas(e.clientX - wrapRect.left, e.clientY - wrapRect.top);
        const dist = Math.sqrt(Math.pow(currentPos.x - activeWire.start.x, 2) + Math.pow(currentPos.y - activeWire.start.y, 2));

        // REQUIREMENT: Must be an artifact in 'extend' state, and moved enough to distinguish from a click
        if (sourceNode && sourceNode.type === 'artifact' && sourceNode.props.state === 'extend' && dist > 40) {
           const dropX = activeWire.end.x;
           const dropY = activeWire.end.y;
           const newNodeId = "node_" + Date.now();
           const artBody = document.getElementById(`art-body-${sourceNode.id}`);
           const payload = artBody ? artBody.textContent : "";
           
           createFlowNode("Transmuted Agent", "__custom__", dropX, dropY, 'agent', newNodeId);
           const newNode = flowNodes.find(n => n.id === newNodeId);
           if (newNode) {
              newNode.props.code = `// Inherited Payload:\n/*\n${payload}\n*/\nreturn truth();`;
              const wireId = "wire_evo_" + Date.now();
              flowWires.push({
                id: wireId, fromId: sourceNode.id, toId: newNodeId,
                signal: 0, confidence: 1.0, condition: "all", transform: "pass", priority: 5, label: "EVOLUTION"
              });
              
              // Automatically return to 'lock' state to prevent accidental multiple spawns
              setArtifactState(sourceNode.id, 'lock');
              
              updateWires();
              saveCanvasState();
              logInspector("SYSTEM", "🧬 Evolution Triggered — New node ready.");
           }
        }
      }
    } catch (err) {
      console.error("Critical error in mouseup:", err);
    } finally {
      document.querySelectorAll('.flow-port').forEach(p => p.classList.remove('magnet'));
      activeWire = null;
      isDraggingNode = false;
      nodeDraggingId = null;
      updateWires();
    }
  }
}
document.addEventListener('mouseup', onMouseUp);
window.onMouseUp = onMouseUp;

async function resumeSimulationFrom(targetId) {
   simulationAborted = false;
   simulationRunning = true;
   updateSimUI();
   
   // Instead of resetting the queue, we push the continuation signal
   // Assuming the last signal that hit the artifact is still valid
   engineQueue.push({ toId: targetId, val: 1, conf: 1.0, origin: "CONTINUATION" });
   
   // Re-enter the simulation loop (partial call)
   // We need to move the 'while' logic of runSimulation into a reusable core
   // For now, we trigger a "soft" runSimulation that doesn't clear the graph
   await runSimulationCore();
}
window.resumeSimulationFrom = resumeSimulationFrom;

// ─── GitHub raw content base (Tier 1 public fallback) ────────────────────────
const GH_RAW = "https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/";
const GH_TERNROOT = GH_RAW + "ternlang-root/";
const GH_API_BASE = "https://api.github.com/repos/eriirfos-eng/ternary-intelligence-stack/contents/ternlang-root/";
// Static Tier 1 directory list (public stdlib dirs) — used when no API key
const TIER1_DIRS = ["core","ternary","std","showcase","bughunt","testing","bench","benchmarks","classical","errors","tutorials","lib"];

async function loadGithubTree() {
  const tree = document.getElementById("file-tree");
  tree.innerHTML = '<div style="padding:10px; color:var(--muted); font-size:11px;">Loading from GitHub…</div>';
  try {
    const allFiles = [];
    // Load stdlib Tier 1 dirs
    for (const dir of TIER1_DIRS) {
      try {
        const r = await fetch(GH_API_BASE + `stdlib/${dir}`);
        if (!r.ok) continue;
        const files = await r.json();
        if (Array.isArray(files)) {
          files.filter(f => f.name.endsWith(".tern")).forEach(f => allFiles.push(`stdlib/${dir}/${f.name}`));
        }
      } catch(e) {}
    }
    // Load examples
    try {
      const r = await fetch(GH_API_BASE + "examples");
      if (r.ok) {
        const files = await r.json();
        if (Array.isArray(files)) {
          files.filter(f => f.name.endsWith(".tern")).forEach(f => allFiles.push(`examples/${f.name}`));
        }
      }
    } catch(e) {}

    if (allFiles.length === 0) { showNoKeyMessage(tree); return; }
    renderFileTree(tree, allFiles, true);
  } catch(e) { showNoKeyMessage(tree); }
}
window.loadGithubTree = loadGithubTree;

function showNoKeyMessage(tree) {
  // Always show built-in templates so explorer is never empty
  renderBuiltinTree(tree);
}
window.showNoKeyMessage = showNoKeyMessage;

function renderBuiltinTree(tree) {
  tree.innerHTML = "";
  const banner = document.createElement("div");
  banner.style.cssText = "padding:6px 12px; font-size:10px; color:var(--muted2); background:rgba(255,255,255,0.02); border-bottom:1px solid var(--border); display:flex; justify-content:space-between; align-items:center;";
  banner.innerHTML = `<span>Built-in Examples</span><span style="cursor:pointer;color:var(--cyan)" onclick="toggleKeyInput()">+ Add key</span>`;
  tree.appendChild(banner);

  const builtins = [
    { name: "hello_trit.tern",    path: "examples/hello_trit.tern",    key: "hello" },
    { name: "consensus.tern",     path: "examples/consensus.tern",      key: "consensus" },
    { name: "match_signal.tern",  path: "examples/match_signal.tern",   key: "match" },
    { name: "trit_gate.tern",     path: "examples/trit_gate.tern",      key: "gate" },
    { name: "agent_basic.tern",   path: "examples/agent_basic.tern",    key: "agent" },
  ];

  builtins.forEach(({ name, path, key }) => {
    const fileEl = document.createElement("div");
    fileEl.className = "tree-file";
    fileEl.dataset.path = path;
    fileEl.dataset.builtin = "1";
    fileEl.innerHTML = `<i data-lucide="file-code" style="width:12px;height:12px"></i> ${name}`;
    fileEl.onclick = () => {
      fileBuffers[path] = fileBuffers[path] || TEMPLATES[key] || TEMPLATES.hello;
      loadToEditor(path, fileBuffers[path]);
      switchView('editor');
    };
    tree.appendChild(fileEl);
  });

  tree.dataset.loaded = "true";
  refreshTreeHighlight();
  lucide.createIcons();
}
window.renderBuiltinTree = renderBuiltinTree;

function renderFileTree(tree, files, isGitHub = false, isPremium = false) {
  const groups = {};
  files.forEach(path => {
    const parts = path.split('/');
    if (parts.length < 2) return;
    const dir = parts[1];
    if (!groups[dir]) groups[dir] = [];
    groups[dir].push(path);
  });
  if (Object.keys(groups).length === 0) { 
    if (!isPremium) showNoKeyMessage(tree); 
    return; 
  }
  tree.innerHTML = "";
  if (isGitHub) {
    const banner = document.createElement("div");
    banner.style.cssText = "padding:6px 12px; font-size:10px; color:var(--amber); background:rgba(245,158,11,0.08); border-bottom:1px solid var(--border);";
    banner.innerHTML = `Tier 1 — GitHub · <span style="cursor:pointer;color:var(--cyan)" onclick="toggleKeyInput()">Enter key for full access</span>`;
    tree.appendChild(banner);
  }
  Object.keys(groups).sort().forEach(dir => {
    const section = document.createElement("div");
    section.className = "tree-section";
    const dirEl = document.createElement("div");
    dirEl.className = "tree-dir collapsed"; // Start collapsed
    dirEl.innerHTML = `<span class="arrow">▸</span> <i data-lucide="folder" style="width:12px; height:12px"></i> ${dir}/`;
    
    const filesEl = document.createElement("div");
    filesEl.className = "tree-files hidden"; // Start hidden

    dirEl.onclick = () => {
      const isHidden = filesEl.classList.toggle("hidden");
      dirEl.classList.toggle("collapsed", isHidden);
      dirEl.querySelector(".arrow").textContent = isHidden ? "▸" : "▾";
      
      if (isHidden) {
        // Explicitly remove children from DOM to save nodes
        filesEl.innerHTML = "";
      } else {
        // Lazy Hydration: inject DOM nodes only when expanded
        groups[dir].forEach(path => {
          const name = path.split('/').pop();
          const fileEl = document.createElement("div");
          fileEl.className = "tree-file";
          fileEl.dataset.path = path;
          fileEl.dataset.github = isGitHub ? "1" : "";
          fileEl.dataset.premium = isPremium ? "1" : "";
          fileEl.innerHTML = `<i data-lucide="file-code" style="width:12px; height:12px"></i> ${name}`;
          fileEl.onclick = () => { openFile(path, isGitHub, isPremium); switchView('editor'); };
          filesEl.appendChild(fileEl);
        });
        refreshTreeHighlight();
        lucide.createIcons({ root: filesEl });
      }
    };
    
    section.appendChild(dirEl);
    section.appendChild(filesEl);
    tree.appendChild(section);
  });
  tree.dataset.loaded = "true";
  refreshTreeHighlight();
  lucide.createIcons({ root: tree });
}
window.renderFileTree = renderFileTree;

// ─── Build file tree ──────────────────────────────────────────────────────────
async function buildFileTree(force = false) {
  const tree = document.getElementById("file-tree");
  if (!force && tree.dataset.loaded === "true") {
     refreshTreeHighlight();
     return;
  }
  tree.innerHTML = '<div style="padding:10px; color:var(--muted); font-size:11px;">Loading library…</div>';

  try {
    const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
    const key = document.getElementById("apiKey").value.trim();

    const r = await fetch(endpoint + "/api/stdlib/list", {
      headers: key ? { "X-Ternlang-Key": key } : {}
    });
    const d = await r.json();
    
    if (d.status !== "ok") {
      await loadGithubTree(); return;
    }
    // If tier=0 (no key), fall back to GitHub for Tier 1 files
    if (!d.files || d.files.length === 0) {
      await loadGithubTree(); return;
    }
    renderFileTree(tree, d.files, false);
    tree.dataset.loaded = "true";
    refreshTreeHighlight();
    lucide.createIcons();

  } catch (e) {
    await loadGithubTree();
  }
}
window.buildFileTree = buildFileTree;

function refreshTreeHighlight() {
  document.querySelectorAll(".tree-file").forEach(el => {
    el.classList.toggle("active", el.dataset.path === activeFile);
  });
}
window.refreshTreeHighlight = refreshTreeHighlight;

// ─── Tab management ───────────────────────────────────────────────────────────
function renderTabs() {
  const container = document.getElementById("editorTabs");
  container.innerHTML = "";
  tabs.forEach(({ name, path }) => {
    const tab = document.createElement("div");
    tab.className = "tab" + (path === activeFile ? " active" : "");
    tab.innerHTML = `<span style="flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;">${name}</span><button class="tab-close" onclick="closeTab('${CSS.escape(path)}',event)" title="Close">✕</button>`;
    tab.onclick = () => switchToTab(path);
    container.appendChild(tab);
  });
}
window.renderTabs = renderTabs;

function closeTab(path, e) {
  if (e) { e.stopPropagation(); }
  const idx = tabs.findIndex(t => t.path === path);
  if (idx === -1) return;
  tabs.splice(idx, 1);
  if (tabs.length === 0) { newFile(); return; }
  if (activeFile === path) {
    const next = tabs[Math.min(idx, tabs.length - 1)];
    switchToTab(next.path);
  } else {
    renderTabs();
  }
}
window.closeTab = closeTab;

function triggerImportFile() {
  document.getElementById("localFileInput").click();
}
window.triggerImportFile = triggerImportFile;

function importLocalFile(event) {
  const file = event.target.files[0];
  if (!file) return;
  const reader = new FileReader();
  reader.onload = (e) => {
    const content = e.target.result;
    const path = "local/" + file.name;
    fileBuffers[path] = content;
    loadToEditor(path, content);
    switchView('editor');
    showToast(`Imported ${file.name}`, "ok");
  };
  reader.readAsText(file);
  event.target.value = "";
}
window.importLocalFile = importLocalFile;

function switchToTab(path) {
  if (monacoEditor) fileBuffers[activeFile] = monacoEditor.getValue();
  activeFile = path;
  if (monacoEditor) monacoEditor.setValue(fileBuffers[path] || "");
  renderTabs();
  refreshTreeHighlight();
  saveEditorState();
  const sbf = document.getElementById("sbFile"); if (sbf) sbf.textContent = path.split("/").pop();
}
window.switchToTab = switchToTab;

async function openFile(path, useGithub = false, usePremium = false) {
  if (monacoEditor) fileBuffers[activeFile] = monacoEditor.getValue();

  if (fileBuffers[path]) {
    loadToEditor(path, fileBuffers[path]);
    return;
  }
  
  const treeEl = document.querySelector(`.tree-file[data-path="${CSS.escape(path)}"]`);
  const isPremium = usePremium || (treeEl && treeEl.dataset.premium === "1");
  const isGithub = useGithub || (treeEl && treeEl.dataset.github === "1");

  if (isPremium) {
    try {
      const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
      const r = await fetch(`${endpoint}/api/premium/file?path=${encodeURIComponent(path)}`, {
        headers: { 'X-Ternlang-Key': localStorage.getItem('ternstudio-key') || '' }
      });
      if (!r.ok) throw new Error(`Auth failed or file not found (${r.status})`);
      const d = await r.json();
      if (d.content) {
        fileBuffers[path] = d.content;
        loadToEditor(path, d.content);
      } else {
        throw new Error("Invalid response from server");
      }
      return;
    } catch (e) {
      showToast(`Failed to load premium file: ${e.message}`, 'err');
      return;
    }
  }
  
  if (isGithub) {
    try {
      const r = await fetch(GH_TERNROOT + path);
      if (r.ok) {
        const content = await r.text();
        fileBuffers[path] = content;
        loadToEditor(path, content);
        return;
      }
    } catch(e) {}
    showToast("Failed to load from GitHub", "err");
    return;
  }

  // Fallback to standard API
  try {
    const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
    const key = document.getElementById("apiKey").value.trim();
    const r = await fetch(endpoint + "/api/stdlib/read/" + path, {
      headers: key ? { "X-Ternlang-Key": key } : {}
    });
    const d = await r.json();
    if (d.status === "ok") {
      fileBuffers[path] = d.content;
      loadToEditor(path, d.content);
    } else {
      showToast(d.error || "Failed to read file", "err");
    }
  } catch (e) {
    showToast("Connection Error", "err");
  }
}
window.openFile = openFile;

function loadToEditor(path, content) {
  const existing = tabs.find(t => t.path === path);
  if (!existing) tabs.push({ name: path.split("/").pop(), path });
  activeFile = path;
  if (monacoEditor) {
    monacoEditor.setValue(content);
    const model = monacoEditor.getModel();
    const ext = path.split(".").pop();
    if (ext === "tern") monaco.editor.setModelLanguage(model, "ternlang");
  }
  renderTabs();
  refreshTreeHighlight();
  saveEditorState();
  const sbf = document.getElementById("sbFile"); if (sbf) sbf.textContent = path.split("/").pop();
}
window.loadToEditor = loadToEditor;

function newFile() {
  const name = `scratch_${scratchCounter++}.tern`;
  const path = `scratch/${name}`;
  fileBuffers[path] = TEMPLATES.hello;
  tabs.push({ name, path });
  switchToTab(path);
  saveEditorState();
}
window.newFile = newFile;

function insertTemplate(key) {
  if (monacoEditor) monacoEditor.setValue(TEMPLATES[key] || "");
}
window.insertTemplate = insertTemplate;

// ─── Sidebar panel switching ──────────────────────────────────────────────────
function switchSidebarPanel(name) {
  document.querySelectorAll(".sidebar-panel").forEach(p => p.classList.remove("active"));
  document.querySelectorAll(".act-btn").forEach(b => b.classList.remove("active"));
  document.getElementById("panel-" + name).classList.add("active");
  document.getElementById("act-" + name).classList.add("active");
}
window.switchSidebarPanel = switchSidebarPanel;

// ─── Sidebar resize ───────────────────────────────────────────────────────────
function startSidebarResize(e) {
  e.preventDefault();
  const sidebar = document.getElementById("editor-sidebar");
  const startX = e.clientX;
  const startW = sidebar.offsetWidth;
  const onMove = ev => {
    const newW = Math.max(120, Math.min(400, startW + ev.clientX - startX));
    document.documentElement.style.setProperty("--sidebar-w", newW + "px");
    if (monacoEditor) monacoEditor.layout();
  };
  const onUp = () => {
    document.removeEventListener("mousemove", onMove);
    document.removeEventListener("mouseup", onUp);
  };
  document.addEventListener("mousemove", onMove);
  document.addEventListener("mouseup", onUp);
}
window.startSidebarResize = startSidebarResize;

// ─── Output resize ────────────────────────────────────────────────────────────
function startOutputResize(e) {
  e.preventDefault();
  const panel = document.getElementById("output-panel");
  const startY = e.clientY;
  const startH = panel.offsetHeight;
  const onMove = ev => {
    const newH = Math.max(80, Math.min(window.innerHeight * 0.6, startH + startY - ev.clientY));
    document.documentElement.style.setProperty("--output-h", newH + "px");
    if (monacoEditor) monacoEditor.layout();
  };
  const onUp = () => {
    document.removeEventListener("mousemove", onMove);
    document.removeEventListener("mouseup", onUp);
  };
  document.addEventListener("mousemove", onMove);
  document.addEventListener("mouseup", onUp);
}
window.startOutputResize = startOutputResize;

// ─── Output tabs ──────────────────────────────────────────────────────────────
function switchOutTab(id, el) {
  document.querySelectorAll(".out-tab").forEach(t => t.classList.remove("active"));
  document.querySelectorAll(".out-panel").forEach(p => p.classList.remove("visible"));
  el.classList.add("active");
  document.getElementById("panel-" + id).classList.add("visible");
  if (id === "api") {
    document.getElementById("apiEndpointBase").textContent =
      document.getElementById("apiEndpoint").value;
  }
}
window.switchOutTab = switchOutTab;

// ─── VM Output helpers ────────────────────────────────────────────────────────
function setStatus(type, text) {
  const el = document.getElementById("vmStatus");
  if (el) { el.className = "status-pill status-" + type; el.textContent = text; }
  const dot = document.getElementById("connDot");
  if (dot) dot.className = "sb-dot" + (type === "error" ? " err" : type === "running" ? " warn" : "");
}
window.setStatus = setStatus;

function clearOutput() {
  setStatus("idle", "● Idle");
  document.getElementById("printOutput").textContent = "— no output —";
  document.getElementById("printOutput").className = "print-output empty";
  document.getElementById("section-meta").style.display = "none";
  document.getElementById("section-regs").style.display = "none";
  document.getElementById("section-error").style.display = "none";
}
window.clearOutput = clearOutput;

window.addEventListener('wasmready', () => {
  const badge = document.getElementById("wasmBadge");
  if (badge) badge.style.opacity = "1";
  const sbw = document.getElementById("sbWasmStatus");
  if (sbw) { sbw.textContent = "⚡ WASM"; sbw.style.color = "var(--green)"; }
});

function showResult(data) {
  const ok = data.status === "ok";
  setStatus(ok ? "ok" : "error", ok ? (data._wasm ? "⚡ OK (WASM)" : "✓ OK") : "✕ Error");

  document.getElementById("section-meta").style.display = "block";
  const bytesLabel = data._wasm ? (data._ms != null ? `${data._ms}ms WASM` : "WASM") : (data.bytecode_bytes != null ? data.bytecode_bytes + "B" : "—");
  document.getElementById("metaBytes").textContent = bytesLabel;
  document.getElementById("metaStatus").textContent = ok ? "exited ok" : "vm error";
  document.getElementById("metaStatus").style.color = ok ? "var(--green)" : "var(--red)";

  const printEl = document.getElementById("printOutput");
  printEl.innerHTML = "";
  printEl.className = "print-output";

  // Virtual Terminal Header
  const header = document.createElement("div");
  header.className = "term-header";
  header.textContent = `Ternary Intelligence Stack — BET-VM v1.0.0 (${data._wasm ? 'WASM' : 'API'})`;
  printEl.appendChild(header);

  // Print outputs
  if (data.output && data.output.length > 0) {
    data.output.forEach(line => {
      const row = document.createElement("div");
      row.className = "term-line";
      row.innerHTML = `<span class="term-prompt">></span><span>${line}</span>`;
      printEl.appendChild(row);
    });
  }

  if (ok) {
    const success = document.createElement("div");
    success.className = "term-line term-success";
    success.style.marginTop = "8px";
    success.textContent = `● Program exited successfully. [trit:${data.trit}]`;
    printEl.appendChild(success);
  } else {
    const errBox = document.createElement("div");
    errBox.className = "term-err-line";
    let errMsg = data.error || "Unknown runtime error";
    
    // Humanize common compiler errors
    if (errMsg.includes("ExpectedToken")) {
      const match = errMsg.match(/ExpectedToken\("(.*?)",\s+"(.*?)"\)/);
      if (match) errMsg = `Syntax Error: Expected ${match[1]}, but found ${match[2]}. Check your semicolons!`;
    } else if (errMsg.includes("UndefinedSymbol")) {
       const match = errMsg.match(/UndefinedSymbol\("(.*?)"\)/);
       if (match) errMsg = `Reference Error: Symbol '${match[1]}' is not defined.`;
    }

    errBox.innerHTML = `<strong>VM_ERROR:</strong> ${errMsg}`;
    printEl.appendChild(errBox);

    const fail = document.createElement("div");
    fail.className = "term-line term-dim";
    fail.textContent = "● Process terminated with non-zero exit code.";
    printEl.appendChild(fail);
  }

  if (data.registers && data.registers.length > 0) {
    document.getElementById("section-regs").style.display = "block";
    const table = document.getElementById("regTable");
    table.innerHTML = "";
    data.registers.forEach((val, i) => {
      const tr = document.createElement("tr");
      const valStr = String(val);
      let rowClass = "";
      if (valStr.includes("Affirm") || valStr.includes("Truth"))   rowClass = "reg-row-affirm";
      else if (valStr.includes("Reject") || valStr.includes("Conflict")) rowClass = "reg-row-reject";
      else if (valStr.includes("Tend") || valStr === "Trit(Tend)") rowClass = "reg-row-zero";
      tr.className = rowClass;
      tr.innerHTML = `<td>r${i}</td><td>${valStr}</td>`;
      table.appendChild(tr);
    });
  }

  const errSec = document.getElementById("section-error");
  if (!ok && data.error) {
    errSec.style.display = "block";
    document.getElementById("errorOutput").textContent = data.error;
  } else {
    errSec.style.display = "none";
  }

  // Update Logic Field Visualizer
  renderLogicField(data.registers || []);
}
window.showResult = showResult;

function renderLogicField(registers = []) {
  const grid = document.getElementById("logic-field-grid");
  if (!grid) return;
  grid.innerHTML = "";

  // BET-013 has 27 registers. If we have more (e.g. tensors), show up to 64 for visual density.
  const count = Math.max(27, Math.min(64, registers.length || 0));
  
  for (let i = 0; i < count; i++) {
    const cell = document.createElement("div");
    cell.className = "trit-cell";
    cell.title = `r${i}`;
    
    let val = 0;
    if (registers[i] != null) {
      const v = String(registers[i]);
      if (v.includes("Affirm") || v.includes("Truth")) {
        val = 1; cell.classList.add("active-affirm"); cell.textContent = "+";
      } else if (v.includes("Reject") || v.includes("Conflict")) {
        val = -1; cell.classList.add("active-reject"); cell.textContent = "-";
      } else {
        val = 0; cell.classList.add("active-tend"); cell.textContent = "0";
      }
    } else {
      // Idle state
      cell.textContent = ".";
      cell.style.opacity = "0.3";
    }
    
    grid.appendChild(cell);
  }
}
window.renderLogicField = renderLogicField;

// ─── Code preparation for BET VM (Auto-main) ─────────────────────────────────
function prepareTernCode(src) {
  src = src.trim();
  if (!src) return src;
  
  // High-fidelity pattern matching for function definitions
  const hasFnMain = /fn\s+main\s*\(/.test(src);
  const hasFunctions = /^fn\s+\w+/m.test(src);
  
  if (hasFnMain) return src;
  
  // If user has other functions but no main, append a main call
  if (hasFunctions) {
    return src + "\n\nfn main() -> trit { return hold; }";
  }
  
  // Otherwise, wrap raw script logic
  const lines = src.split('\n').map(l => {
    let t = l.trim();
    if (t && !t.endsWith(';') && !t.endsWith('}') && !t.endsWith('{') && !t.startsWith('//')) {
      return l + ';';
    }
    return l;
  });

  return `fn main() -> trit {\n${lines.join('\n')}\n    return hold;\n}`;
}
window.prepareTernCode = prepareTernCode;

/**
 * Universal TIS Execution Wrapper
 * Mirrors CLI 'run' logic: prepare -> wasm -> parse results
 */
function runTernCode(rawCode) {
  if (!window.wasmReady || !window.wasmRunTern) {
    return { ok: false, error: "BET-VM (WASM) not loaded. Check network." };
  }
  
  const prepared = prepareTernCode(rawCode);
  
  // WASM_PAYLOAD_MIRROR: Focused Execution Audit
  /*
  console.group('WASM_PAYLOAD_MIRROR');

  console.groupEnd();
  */

  try {
    const rawResult = window.wasmRunTern(prepared);
    const r = JSON.parse(rawResult);
    return {
      ok: r.ok,
      output: r.output || [],
      trit: r.trit ?? 0,
      label: r.label || "hold",
      registers: r.registers || [],
      error: r.error || null,
      cycles: r.cycles || 0
    };
  } catch (e) {
    const debug = {
      type: "WASM_PANIC",
      error: e.message,
      stack: e.stack,
      payload_len: prepared.length,
      timestamp: new Date().toISOString()
    };
    console.error('🛑 TERNLANG_CRITICAL_DEBUG:', debug);
    return { ok: false, error: "VM_PANIC: " + e.message, traceback: e.stack };
  }
}
window.runTernCode = runTernCode;

// ─── Run ──────────────────────────────────────────────────────────────────────
async function runCode() {
  if (!monacoEditor) return;
  fileBuffers[activeFile] = monacoEditor.getValue();
  const rawCode = monacoEditor.getValue();
  if (!rawCode.trim()) return;

  const startTime = Date.now();
  const runBtn = document.getElementById("runBtn");
  runBtn.classList.add("running");
  runBtn.textContent = "● Running…";
  setStatus("running", "● Running…");

  // Switch views
  if (!document.getElementById("view-editor").classList.contains("active")) switchView("editor");
  document.querySelectorAll(".out-tab").forEach(t => t.classList.remove("active"));
  document.querySelectorAll(".out-panel").forEach(p => p.classList.remove("visible"));
  document.querySelector(".out-tab").classList.add("active");
  document.getElementById("panel-output").classList.add("visible");

  const r = runTernCode(rawCode);
  const ms = Date.now() - startTime;

  const data = {
    status: r.ok ? "ok" : "error",
    output: r.output,
    trit: r.trit,
    label: r.label,
    registers: r.registers,
    error: r.error,
    bytecode_bytes: r.cycles,
    _ms: ms,
    _wasm: true,
  };

  showResult(data);
  addRunToHistory(activeFile, r.ok, ms, data);
  runBtn.classList.remove("running");
  runBtn.innerHTML = '<span>▶ Run</span><span class="kbd">Ctrl+↵</span>';
}
window.runCode = runCode;

// ─── Run history ──────────────────────────────────────────────────────────────
function addRunToHistory(path, ok, ms, data) {
  sessionRuns++;
  if (ok) sessionOk++; else sessionErr++;
  document.getElementById("statRuns").textContent = sessionRuns;
  document.getElementById("statOk").textContent   = sessionOk;
  document.getElementById("statErr").textContent  = sessionErr;

  const entry = {
    path, ok, ms, ts: new Date().toLocaleTimeString(),
    code: monacoEditor ? monacoEditor.getValue() : "",
    data,
  };
  runHistory.unshift(entry);
  if (runHistory.length > 20) runHistory.pop();
  renderHistory();
  updateDashboardRuns();
}
window.addRunToHistory = addRunToHistory;

function renderHistory() {
  const el = document.getElementById("history-list");
  if (runHistory.length === 0) {
    el.innerHTML = '<div style="padding:12px; font-size:11px; color:var(--muted); font-style:italic;">No runs yet</div>';
    return;
  }
  el.innerHTML = runHistory.map((e, i) => `
    <div class="hist-item" onclick="restoreRun(${i})">
      <div class="hist-name">${e.path.split("/").pop()}</div>
      <div class="hist-meta">
        <span class="${e.ok ? 'hist-ok' : 'hist-err'}">${e.ok ? "✓ ok" : "✕ err"}</span>
        <span>${e.ts}</span>
        <span>${e.ms}ms</span>
      </div>
    </div>`).join("");
}
window.renderHistory = renderHistory;

function restoreRun(i) {
  const e = runHistory[i];
  if (!e) return;
  if (!tabs.find(t => t.path === e.path)) tabs.push({ name: e.path.split("/").pop(), path: e.path });
  activeFile = e.path;
  fileBuffers[e.path] = e.code;
  if (monacoEditor) monacoEditor.setValue(e.code);
  renderTabs();
  refreshTreeHighlight();
  showResult(e.data);
  switchView("editor");
}
window.restoreRun = restoreRun;

function clearHistory() {
  runHistory = [];
  renderHistory();
}
window.clearHistory = clearHistory;

function updateDashboardRuns() {
  const el = document.getElementById("dashRunList");
  if (runHistory.length === 0) {
    el.innerHTML = '<div class="dash-run-empty">No runs yet this session</div>';
    return;
  }
  el.innerHTML = runHistory.slice(0, 5).map(e => `
    <div class="dash-run-item">
      <span class="run-status ${e.ok ? 'run-ok' : 'run-err'}">${e.ok ? "✓" : "✕"}</span>
      <span class="run-name">${e.path.split("/").pop()}</span>
      <span style="font-size:10px; color:var(--muted);">${e.ts}</span>
    </div>`).join("");
}
window.updateDashboardRuns = updateDashboardRuns;

// ─── Share + Download ────────────────────────────────────────────────────────
function shareCode() {
  if (!monacoEditor) return;
  const code = monacoEditor.getValue();
  const hash = "#code=" + btoa(encodeURIComponent(code));
  const url = location.origin + location.pathname + hash;
  navigator.clipboard.writeText(url).then(() => showToast("Share URL copied to clipboard", "ok")).catch(() => showToast("Copy failed — check browser permissions", "err"));
}
window.shareCode = shareCode;

function downloadCode() {
  if (!monacoEditor) return;
  const code = monacoEditor.getValue();
  const name = activeFile.split("/").pop() || "scratch.tern";
  const blob = new Blob([code], { type: "text/plain" });
  const a = document.createElement("a");
  a.href = URL.createObjectURL(blob);
  a.download = name;
  a.click();
  URL.revokeObjectURL(a.href);
  showToast("Downloaded " + name, "ok");
}
window.downloadCode = downloadCode;

// ─── Toast ────────────────────────────────────────────────────────────────────
function showToast(msg, type = "") {
  const c = document.getElementById("toast-container");
  const t = document.createElement("div");
  t.className = "toast" + (type ? " " + type : "");
  t.textContent = msg;
  c.appendChild(t);
  setTimeout(() => t.remove(), 3000);
}
window.showToast = showToast;

// ─── Load code from URL hash ──────────────────────────────────────────────────
function loadFromHash() {
  const hash = location.hash;
  if (hash.startsWith("#code=")) {
    try {
      const code = decodeURIComponent(atob(hash.slice(6)));
      const path = "scratch/shared.tern";
      fileBuffers[path] = code;
      if (!tabs.find(t => t.path === path)) tabs.push({ name: "shared.tern", path });
      activeFile = path;
      if (monacoEditor) monacoEditor.setValue(code);
      renderTabs();
      refreshTreeHighlight();
      document.getElementById("sbFile").textContent = "shared.tern";
      switchView("editor");
      showToast("Loaded shared code", "ok");
    } catch (e) { showToast("Failed to decode shared URL", "err"); }
  }
}
window.loadFromHash = loadFromHash;

// ─── API Explorer try buttons ─────────────────────────────────────────────────
async function tryHealth() {
  const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
  try {
    const r = await fetch(endpoint + "/health");
    const d = await r.json();
    document.getElementById("apiResponse").textContent = JSON.stringify(d, null, 2);
  } catch (e) {
    document.getElementById("apiResponse").textContent = String(e);
  }
}
window.tryHealth = tryHealth;

async function tryApiUsage() {
  const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
  const key = document.getElementById("apiKey").value.trim();
  try {
    const r = await fetch(endpoint + "/api/usage", {
      headers: key ? { "X-Ternlang-Key": key } : {},
    });
    const d = await r.json();
    document.getElementById("apiResponse").textContent = JSON.stringify(d, null, 2);
  } catch (e) {
    document.getElementById("apiResponse").textContent = String(e);
  }
}
window.tryApiUsage = tryApiUsage;

async function tryApiRun() {
  const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
  const key = document.getElementById("apiKey").value.trim();
  const code = monacoEditor ? monacoEditor.getValue() : "";
  const body = { code };
  if (key) body.key = key;
  try {
    const r = await fetch(endpoint + "/api/run", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    const d = await r.json();
    document.getElementById("apiResponse").textContent = JSON.stringify(d, null, 2);
  } catch (e) {
    document.getElementById("apiResponse").textContent = String(e);
  }
}
window.tryApiRun = tryApiRun;

const REPL_SNIPPETS = {
  hello: `fn main() -> trit {\n    print("@ Hello Ternary World");\n    return affirm;\n}`,
  consensus: `fn main() -> trit {\n    let a: trit = affirm;\n    let b: trit = tend;\n    let c: trit = affirm;\n    // Majority: 2 affirm, 1 tend → affirm\n    return a;\n}`,
  match: `fn main() -> trit {\n    let sig: trit = tend;\n    match sig {\n        affirm => { print("YES"); }\n        tend   => { print("HOLD — deliberate"); }\n        reject => { print("NO"); }\n    }\n    return sig;\n}`,
  dec13: `fn main() -> trit {\n    // 13 in balanced ternary = +-+  (+9 -3 +1)\n    let val: int = 13;\n    print("13 = +-+ (balanced ternary)");\n    return affirm;\n}`,
};

function setReplSnippet(key) {
  document.getElementById("dashReplInput").value = REPL_SNIPPETS[key] || "";
}
window.setReplSnippet = setReplSnippet;

function openReplInEditor() {
  const code = document.getElementById("dashReplInput").value.trim();
  if (!code) return;
  const path = "playground/repl.tern";
  fileBuffers[path] = code;
  loadToEditor(path, code);
  switchView('editor');
}
window.openReplInEditor = openReplInEditor;

async function runReplExpr() {
  const input = document.getElementById("dashReplInput").value.trim();
  const resEl = document.getElementById("dashReplRes");
  if (!input) return;

  resEl.style.color = "var(--muted)";
  resEl.textContent = "running…";
  const t0 = Date.now();

  const r = runTernCode(input);
  const ms = Date.now() - t0;

  if (r.ok) {
    const trit = r.trit;
    const label = r.label || (trit === 1 ? "affirm" : trit === -1 ? "reject" : "tend");
    const color = trit === 1 ? "var(--green)" : (trit === -1 ? "var(--red)" : "var(--amber)");
    const sign  = trit === 1 ? "+1" : (trit === -1 ? "-1" : "0");
    resEl.style.color = color;
    let out = `${sign}  ${label.toUpperCase()}  [${ms}ms WASM]`;
    if (r.output && r.output.length) out += "\n" + r.output.join("\n");
    resEl.textContent = out;
  } else {
    resEl.style.color = "var(--red)";
    resEl.textContent = r.error || "Error";
  }
}
window.runReplExpr = runReplExpr;

// ─── Key persistence ──────────────────────────────────────────────────────────
function initKeyPersistence() {
  const saved = localStorage.getItem("ternstudio-key");
  const saveEnabled = localStorage.getItem("ternstudio-save-key") === "1";
  if (saved && saveEnabled) {
    document.getElementById("apiKey").value = saved;
    document.getElementById("saveKeyCheck").checked = true;
  } else {
    document.getElementById("saveKeyCheck").checked = saveEnabled;
  }
}
window.initKeyPersistence = initKeyPersistence;

function toggleSaveKey() {
  const checked = document.getElementById("saveKeyCheck").checked;
  localStorage.setItem("ternstudio-save-key", checked ? "1" : "0");
  if (checked) {
    let key = (document.getElementById("settingsNewKey") || {}).value || "";
    if (!key.trim()) key = document.getElementById("apiKey").value.trim();
    if (key) {
      document.getElementById("apiKey").value = key.trim();
      localStorage.setItem("ternstudio-key", key.trim());
      syncSettingsKeyDisplay();
    }
  } else {
    localStorage.removeItem("ternstudio-key");
  }
}
window.toggleSaveKey = toggleSaveKey;

function applySettingsKey() {
  const val = (document.getElementById("settingsNewKey") || {}).value || "";
  if (val.trim()) {
    updateApiKey(val.trim());
    document.getElementById("settingsNewKey").value = "";
  }
}
window.applySettingsKey = applySettingsKey;

function syncSettingsKeyDisplay() {
  const key = (document.getElementById("apiKey") || {value:""}).value.trim();
  const masked = key.length > 12 ? key.slice(0, 8) + "…" + key.slice(-4) : (key || "—");
  const display = document.getElementById("settingsKeyDisplay");
  if (display) {
    display.textContent = masked;
  }
}
window.syncSettingsKeyDisplay = syncSettingsKeyDisplay;

function copyKey() {
  const key = document.getElementById("apiKey").value.trim();
  if (key) navigator.clipboard.writeText(key).then(() => showToast("Key copied", "ok"));
}
window.copyKey = copyKey;

function clearKey() {
  document.getElementById("apiKey").value = "";
  localStorage.removeItem("ternstudio-key");
  renderUsageAnon();
  syncSettingsKeyDisplay();
  showToast("Key cleared", "ok");
}
window.clearKey = clearKey;

function applyEndpoint() {
  const val = document.getElementById("settingsEndpoint").value.trim();
  if (val) {
    document.getElementById("apiEndpoint").value = val;
    checkConnection();
    showToast("Endpoint updated", "ok");
  }
}
window.applyEndpoint = applyEndpoint;

function syncSettingsUI() {
  document.getElementById("saveKeyCheck").checked = localStorage.getItem("ternstudio-save-key") === "1";
  syncSettingsKeyDisplay();
  const theme = document.documentElement.getAttribute("data-theme") || "dark";
  document.getElementById("settingsTheme").value = theme;
  renderVaultUI();
}
window.syncSettingsUI = syncSettingsUI;

function applyEditorSettings() {
  if (!monacoEditor) return;
  const fontSize = parseInt(document.getElementById("settingsFontSize").value);
  const minimap = document.getElementById("settingsMinimap").value === "true";
  const wordWrap = document.getElementById("settingsWordWrap").value;
  monacoEditor.updateOptions({ fontSize, minimap: { enabled: minimap }, wordWrap });
}
window.applyEditorSettings = applyEditorSettings;

function applyThemeFromSettings() {
  const val = document.getElementById("settingsTheme").value;
  localStorage.setItem("ternstudio-theme", val);
  applyTheme(val);
  if (monacoEditor) monaco.editor.setTheme(val === "light" ? "ternstudio-light" : "ternstudio-dark");
}
window.applyThemeFromSettings = applyThemeFromSettings;

// ─── Local Secrets Vault (TernFlow) ──────────────────────────────────────────
function getTernflowSecrets() {
  try {
    return JSON.parse(localStorage.getItem("ternflow_secrets") || "{}");
  } catch (e) {
    return {};
  }
}

function setTernflowSecret(provider, key) {
  const secrets = getTernflowSecrets();
  if (key) secrets[provider] = key;
  else delete secrets[provider];
  localStorage.setItem("ternflow_secrets", JSON.stringify(secrets));
  renderVaultUI();
}
window.setTernflowSecret = setTernflowSecret;

function addVaultSecret() {
  const provider = document.getElementById("newSecretProvider").value;
  const key = document.getElementById("newSecretKey").value.trim();
  if (key) {
    setTernflowSecret(provider, key);
    document.getElementById("newSecretKey").value = "";
    showToast(`Secret for ${provider} updated`, "ok");
  }
}
window.addVaultSecret = addVaultSecret;

function renderVaultUI() {
  const listEl = document.getElementById("secretsVaultList");
  if (!listEl) return;
  const secrets = getTernflowSecrets();
  const providers = ["openai", "anthropic", "google", "custom"];
  
  listEl.innerHTML = providers.map(p => {
    const key = secrets[p] || "";
    const masked = key ? (key.slice(0, 8) + "…" + key.slice(-4)) : "Not set";
    return `
      <div style="display:grid; grid-template-columns:140px 1fr auto; gap:12px; align-items:center; padding-bottom:12px; border-bottom:1px solid var(--border2);">
        <div style="font-size:12px; font-weight:600; color:var(--text); text-transform:capitalize;">${p}</div>
        <div style="font-family:'JetBrains Mono',monospace; font-size:11px; color:${key ? 'var(--cyan)' : 'var(--muted2)'};">${masked}</div>
        <button class="settings-btn" onclick="setTernflowSecret('${p}', '')" style="color:var(--red); border-color:rgba(239,68,68,0.2); transition:background 0.2s;" onmouseover="this.style.background='rgba(239,68,68,0.1)'" onmouseout="this.style.background='transparent'">Clear</button>
      </div>
    `;
  }).join("");
}
window.renderVaultUI = renderVaultUI;

// ─── Usage dashboard ──────────────────────────────────────────────────────────
const TIER_BENEFITS = {
  1: ["● /api/run — execute .tern programs", "● /health, /studio", "○ Trit decisions + vectors (Tier 2+)", "○ MoE-13 orchestrator (Tier 2+)", "○ Deliberation engine (Tier 2+)"],
  2: ["● /api/run", "● /api/trit_decide, /api/trit_vector", "● /api/trit_consensus, /api/trit_gate", "● /api/moe/orchestrate", "● Deliberation + coalition engine", "○ Industrial endpoints (Tier 3+)"],
  3: ["● All Tier 2 endpoints", "● /api/v1/taas/infer", "● /api/stream/* SSE endpoints", "● Sparse benchmark + quantization", "○ Enterprise SLA (Tier 4)"],
  4: ["● All endpoints, unlimited", "● Enterprise SLA + dedicated support", "● Custom rate limits", "● Private deployment options"],
};

function renderUsageAnon() {
  document.getElementById("usageKeyDisplay").textContent = "No key — anonymous access";
  document.getElementById("usageTierBadge").className = "tb-badge badge-free";
  document.getElementById("usageTierBadge").textContent = "Tier 1";
  document.getElementById("usage-quota-section").style.display = "none";
  document.getElementById("usage-unlimited-section").style.display = "none";
  document.getElementById("usageError").style.display = "none";
  renderTierBenefits(1);
  updateTopbarTier(1);
}
window.renderUsageAnon = renderUsageAnon;

function renderTierBenefits(tier) {
  const list = document.getElementById("tierBenefitsList");
  const benefits = TIER_BENEFITS[tier] || TIER_BENEFITS[1];
  list.innerHTML = benefits.map((b, i) => {
    const muted = b.startsWith("○");
    return `<div class="tier-benefit ${muted ? "muted-benefit" : ""}">${b}</div>`;
  }).join("");
  const upgradeBtn = document.getElementById("upgradeBtn");
  upgradeBtn.style.display = tier >= 4 ? "none" : "inline-block";
}
window.renderTierBenefits = renderTierBenefits;

function updateTopbarTier(tier) {
  const badge = document.getElementById("tierBadge");
  if (badge) { badge.textContent = TIER_LABELS[tier] || "Tier 1"; badge.className = "tier-badge tb-badge " + (TIER_BADGE_CLASS[tier] || "badge-free"); }
  const sbTierEl = document.getElementById("sbTier"); if (sbTierEl) sbTierEl.textContent = TIER_LABELS[tier] || "Tier 1 — Open Core";
  const upBtn = document.getElementById("topbarUpskillBtn");
  if (upBtn) upBtn.style.display = tier <= 1 ? "flex" : "none";
  // Sync dashboard tier badge
  const dashBadge = document.getElementById("dashTierBadge");
  if (dashBadge) {
    dashBadge.textContent = TIER_LABELS[tier] || "Tier 1 — Free";
    dashBadge.className = "tb-badge " + (TIER_BADGE_CLASS[tier] || "badge-free");
  }
}
window.updateTopbarTier = updateTopbarTier;

async function fetchUsage() {
  const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
  const key = document.getElementById("apiKey").value.trim();
  if (!key) { renderUsageAnon(); return; }
  try {
    const r = await fetch(endpoint + "/api/usage", {
      headers: { "X-Ternlang-Key": key },
      signal: AbortSignal.timeout(5000),
    });
    if (!r.ok) {
      const txt = await r.text();
      document.getElementById("usageError").style.display = "block";
      document.getElementById("usageErrorMsg").textContent = `HTTP ${r.status} — ${txt.slice(0, 200)}`;
      return;
    }
    const d = await r.json();
    document.getElementById("usageError").style.display = "none";
    const masked = key.length > 12 ? key.slice(0, 8) + "…" + key.slice(-4) : key;
    document.getElementById("topbarKeyInput").value = key;
    
    const tier = d.tier || 1;
    const oldTier = document.getElementById("usageTierBadge").dataset.tier;
    if (oldTier && oldTier != tier) {
        document.getElementById("file-tree").dataset.loaded = "false";
        renderFlowLibrary();
        buildFileTree();
    }
    document.getElementById("usageTierBadge").dataset.tier = tier;

    document.getElementById("usageKeyDisplay").textContent = masked;
    updateTopbarTier(tier);
    document.getElementById("usageTierBadge").textContent = TIER_LABELS[tier] || "Tier 1";
    document.getElementById("usageTierBadge").className = "tb-badge " + (TIER_BADGE_CLASS[tier] || "badge-free");
    renderTierBenefits(tier);
    
    // Always refresh tree when key usage is fetched to ensure correct tier visibility
    buildFileTree(true);
    renderFlowLibrary();

    if (d.limit === null || d.limit === undefined || tier >= 4) {
      document.getElementById("usage-quota-section").style.display = "none";
      document.getElementById("usage-unlimited-section").style.display = "block";
      document.getElementById("dash-unlimited-badge").style.display = "block";
      document.getElementById("dash-quota-wrap").style.display = "none";
    } else {
      document.getElementById("usage-unlimited-section").style.display = "none";
      document.getElementById("usage-quota-section").style.display = "block";
      const used = d.calls_this_month || 0;
      const limit = d.limit || 0;
      const pct = limit > 0 ? Math.min(100, Math.round(used / limit * 100)) : 0;
      const remaining = Math.max(0, limit - used);
      document.getElementById("usageUsed").textContent = used.toLocaleString();
      document.getElementById("usageLimit").textContent = limit.toLocaleString();
      const fill = document.getElementById("usageBarFill");
      fill.style.width = pct + "%";
      fill.className = "usage-bar-fill " + (pct >= 90 ? "crit" : pct >= 70 ? "warn" : "ok");
      document.getElementById("usageMeta").textContent =
        `${pct}% used · ${remaining.toLocaleString()} remaining · resets 1st of month`;
      // Dashboard
      const dashFill = document.getElementById("dashUsageBar");
      dashFill.style.width = pct + "%";
      dashFill.className = "usage-bar-fill " + (pct >= 90 ? "crit" : pct >= 70 ? "warn" : "ok");
      document.getElementById("dashUsageMeta").textContent =
        `${used.toLocaleString()} / ${limit.toLocaleString()} · ${pct}%`;
    }
  } catch (e) {
    document.getElementById("usageError").style.display = "block";
    document.getElementById("usageErrorMsg").textContent = String(e);
  }
}
window.fetchUsage = fetchUsage;

// ─── Connection status ────────────────────────────────────────────────────────
async function checkConnection() {
  const endpoint = document.getElementById("apiEndpoint").value.replace(/\/$/, "");
  const dot = document.getElementById("dashVmDot");
  const label = document.getElementById("dashVmLabel");
  try {
    const r = await fetch(endpoint + "/health", { signal: AbortSignal.timeout(4000) });
    if (r.ok) {
      const cl = document.getElementById("connLabel"); if (cl) cl.textContent = "Connected · " + endpoint.replace("https://", "");
      const cd = document.getElementById("connDot");   if (cd) cd.className = "sb-dot";
      if (dot)   dot.className   = "dot";
      if (label) label.textContent = "Online · " + endpoint.replace("https://", "");
    } else { throw new Error("not ok"); }
  } catch {
    const cl = document.getElementById("connLabel"); if (cl) cl.textContent = "Offline";
    const cd = document.getElementById("connDot");   if (cd) cd.className = "sb-dot err";
    if (dot)   dot.className   = "dot err";
    if (label) label.textContent = "Offline";
  }
}
window.checkConnection = checkConnection;

// ─── Upskill modal ────────────────────────────────────────────────────────────
function openUpskillModal()  { document.getElementById("upskillModal").style.display = "flex"; document.body.style.overflow = "hidden"; }
function closeUpskillModal() { document.getElementById("upskillModal").style.display = "none"; document.body.style.overflow = ""; }
document.addEventListener("keydown", e => { if (e.key === "Escape") closeUpskillModal(); });

// ─── Theme toggle ─────────────────────────────────────────────────────────────
function applyTheme(mode) {
  const html = document.documentElement;
  const btn  = document.getElementById("themeBtn");
  const icon = document.getElementById("themeIcon");

  if (mode === "light") {
    html.setAttribute("data-theme", "light");
    if (btn) btn.title = "Switch to dark mode";
    if (icon) icon.setAttribute("data-lucide", "sun");
  } else {
    html.removeAttribute("data-theme");
    if (btn) btn.title = "Switch to light mode";
    if (icon) icon.setAttribute("data-lucide", "moon");
  }
  if (typeof lucide !== 'undefined') lucide.createIcons();
}
window.applyTheme = applyTheme;

function toggleTheme() {
  const current = document.documentElement.getAttribute("data-theme") === "light" ? "light" : "dark";
  const next = current === "light" ? "dark" : "light";
  localStorage.setItem("ternstudio-theme", next);
  applyTheme(next);
  if (monacoEditor) monaco.editor.setTheme(next === "light" ? "ternstudio-light" : "ternstudio-dark");
}
window.toggleTheme = toggleTheme;

(function() {
  const saved = localStorage.getItem("ternstudio-theme") || "dark";
  applyTheme(saved);
})();

// ─── Monaco init ──────────────────────────────────────────────────────────────
require.config({ paths: { vs: "https://cdn.jsdelivr.net/npm/monaco-editor@0.52.0/min/vs" } });

require(["vs/editor/editor.main"], function () {
  monaco.languages.register({ id: "ternlang" });

  monaco.languages.setMonarchTokensProvider("ternlang", {
    keywords: ["fn", "let", "return", "match", "if", "else", "while", "for", "in", "break", "continue", "spawn", "send", "await"],
    types: ["trit", "int", "float", "bool", "void"],
    builtins: ["truth", "hold", "conflict", "consensus", "print", "println", "affirm", "reject", "tend"],
    directives: ["sparseskip", "inline", "export"],
    tokenizer: {
      root: [
        [/@[a-zA-Z_]+/, "keyword.directive"],
        [/\/\/.*$/, "comment"],
        [/\b(fn|let|return|match|if|else|while|for|in|break|continue|spawn|send|await)\b/, "keyword"],
        [/\b(trit|int|float|trittensor|void)\b/, "type"],
        [/\b(truth|hold|conflict|consensus|print|println|affirm|reject|tend)\b/, "support.function"],
        [/-1\b/, "number.trit.reject"],
        [/\b0\b/, "number.trit.hold"],
        [/\b1\b/, "number.trit.affirm"],
        [/\b\d+\.\d+\b/, "number.float"],
        [/\b\d+\b/, "number"],
        [/"([^"\\]|\\.)*"/, "string"],
        [/->/, "operator"],
        [/=>/, "operator"],
        [/[+\-*\/=!<>?%]/, "operator"],
        [/[{}()\[\];,:]/, "delimiter"],
        [/[a-zA-Z_]\w*/, "identifier"],
      ],
    },
  });

  monaco.editor.defineTheme("ternstudio-dark", {
    base: "vs-dark", inherit: true,
    rules: [
      { token: "comment",           foreground: "3a5060", fontStyle: "italic" },
      { token: "keyword",           foreground: "00c8ff", fontStyle: "bold" },
      { token: "keyword.directive", foreground: "ffaa00", fontStyle: "bold" },
      { token: "type",              foreground: "80e8ff" },
      { token: "support.function",  foreground: "00e87a" },
      { token: "number.trit.affirm",foreground: "00e87a", fontStyle: "bold" },
      { token: "number.trit.reject",foreground: "ff3b55", fontStyle: "bold" },
      { token: "number.trit.hold",  foreground: "ffaa00", fontStyle: "bold" },
      { token: "number.float",      foreground: "a8d8ff" },
      { token: "number",            foreground: "a8d8ff" },
      { token: "string",            foreground: "c8f0a0" },
      { token: "operator",          foreground: "6090b0" },
      { token: "delimiter",         foreground: "405060" },
      { token: "identifier",        foreground: "c8d4e0" },
    ],
    colors: {
      "editor.background":              "#080b10",
      "editor.foreground":              "#c8d4e0",
      "editor.lineHighlightBackground": "#0d1219",
      "editorCursor.foreground":        "#00c8ff",
      "editor.selectionBackground":     "#0d2040",
      "editorLineNumber.foreground":    "#2a3a4a",
      "editorLineNumber.activeForeground": "#5a7a9a",
      "editorIndentGuide.background":   "#1a2530",
      "editorGutter.background":        "#080b10",
      "scrollbar.shadow":               "#00000000",
      "scrollbarSlider.background":     "#1d2835",
      "scrollbarSlider.hoverBackground":"#253040",
    },
  });

  monaco.editor.defineTheme("ternstudio-light", {
    base: "vs", inherit: true,
    rules: [
      { token: "comment",           foreground: "7a9aaa", fontStyle: "italic" },
      { token: "keyword",           foreground: "0060bb", fontStyle: "bold" },
      { token: "keyword.directive", foreground: "9a5000", fontStyle: "bold" },
      { token: "type",              foreground: "006090" },
      { token: "support.function",  foreground: "007040" },
      { token: "number.trit.affirm",foreground: "007040", fontStyle: "bold" },
      { token: "number.trit.reject",foreground: "bb001a", fontStyle: "bold" },
      { token: "number.trit.hold",  foreground: "9a5000", fontStyle: "bold" },
      { token: "number.float",      foreground: "005090" },
      { token: "number",            foreground: "005090" },
      { token: "string",            foreground: "306820" },
      { token: "operator",          foreground: "507090" },
      { token: "delimiter",         foreground: "708090" },
      { token: "identifier",        foreground: "1a2535" },
    ],
    colors: {
      "editor.background":              "#f8fafc",
      "editor.foreground":              "#1a2535",
      "editor.lineHighlightBackground": "#eef2f6",
      "editorCursor.foreground":        "#0070cc",
      "editor.selectionBackground":     "#cce0ff",
      "editorLineNumber.foreground":    "#a0aab5",
      "editorLineNumber.activeForeground": "#607080",
      "editorIndentGuide.background":   "#d8dde3",
      "editorGutter.background":        "#f0f2f5",
      "scrollbar.shadow":               "#00000010",
      "scrollbarSlider.background":     "#c8ced4",
      "scrollbarSlider.hoverBackground":"#b0b8c0",
    },
  });

  const initTheme = localStorage.getItem("ternstudio-theme") || "dark";
  applyTheme(initTheme);

  monacoEditor = monaco.editor.create(document.getElementById("monaco-container"), {
    value: fileBuffers[activeFile] || TEMPLATES.hello,
    language: "ternlang",
    theme: initTheme === "light" ? "ternstudio-light" : "ternstudio-dark",
    fontFamily: "'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'Consolas', monospace",
    fontSize: 13,
    lineHeight: 20,
    minimap: { enabled: false },
    scrollBeyondLastLine: false,
    renderWhitespace: "boundary",
    bracketPairColorization: { enabled: true },
    guides: { bracketPairs: true },
    suggest: { showWords: false },
    quickSuggestions: false,
    padding: { top: 12, bottom: 12 },
    overviewRulerLanes: 0,
    hideCursorInOverviewRuler: true,
    renderLineHighlight: "gutter",
    scrollbar: { verticalScrollbarSize: 5, horizontalScrollbarSize: 5 },
  });

  monacoEditor.onDidChangeModelContent(() => {
    saveEditorState();
  });

  monacoEditor.onDidChangeCursorPosition(e => {
    const cp = document.getElementById("cursorPos");
    if (cp) cp.textContent = `Ln ${e.position.lineNumber}, Col ${e.position.column}`;
  });

  window.addEventListener("resize", () => monacoEditor.layout());

  monacoEditor.addCommand(monaco.KeyCode.F5, () => runCode());
  monacoEditor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter, () => runCode());

  // Initialization: load saved state
  initKeyPersistence();
  const savedKey = localStorage.getItem("ternstudio-key") || "";
  if (savedKey) {
    document.getElementById("apiKey").value = savedKey;
    document.getElementById("topbarKeyInput").value = savedKey;
    fetchUsage();
    syncFleetRegistry();
    loadPremiumTree();
  } else if (window.TERNSTUDIO_DEV_KEY) {
    // Auto-load dev key from local config (.ternstudio-local.js, gitignored)
    document.getElementById("apiKey").value = window.TERNSTUDIO_DEV_KEY;
    document.getElementById("topbarKeyInput").value = window.TERNSTUDIO_DEV_KEY;
    localStorage.setItem("ternstudio-key", window.TERNSTUDIO_DEV_KEY);
    fetchUsage();
    syncFleetRegistry();
    loadPremiumTree();
  } else {
    renderUsageAnon();
    buildFileTree();
  }

  renderTabs();
  checkConnection();
  setInterval(checkConnection, 30000);

  // Load from hash after Monaco is ready
  loadFromHash();

  // Migration: Manually inject missing agents
  const reg = JSON.parse(localStorage.getItem("ternflow_registry") || "[]");
  const missing = [
    { id: "agent", slug: "agent", name: "Agent", desc: "Custom ternary pipeline", pricing: "per_call", nodes: 2, deployed: "2026-04-19T00:00:00Z" },
    { id: "mesh-node-a", slug: "mesh-node-a", name: "Mesh_Node_A", desc: "Custom ternary pipeline", pricing: "private", nodes: 8, deployed: "2026-04-19T00:00:00Z" }
  ];
  let changed = false;
  missing.forEach(m => {
    if (!reg.find(r => r.id === m.id)) { reg.push(m); changed = true; }
  });
  if (changed) localStorage.setItem("ternflow_registry", JSON.stringify(reg));

  const lastView = localStorage.getItem("ternstudio-last-view") || "dashboard";
  hydrateSimSpeed();
  switchView(lastView);

  // Initialize visualizer with empty state
  renderLogicField([]);

  if (typeof lucide !== 'undefined') lucide.createIcons();
});

// Final fallback for Lucide to ensure all icons (including dynamic ones) render
document.addEventListener('DOMContentLoaded', () => {
  if (typeof lucide !== 'undefined') lucide.createIcons();
});
