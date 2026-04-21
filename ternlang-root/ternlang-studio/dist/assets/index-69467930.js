(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))o(i);new MutationObserver(i=>{for(const r of i)if(r.type==="childList")for(const a of r.addedNodes)a.tagName==="LINK"&&a.rel==="modulepreload"&&o(a)}).observe(document,{childList:!0,subtree:!0});function n(i){const r={};return i.integrity&&(r.integrity=i.integrity),i.referrerPolicy&&(r.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?r.credentials="include":i.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function o(i){if(i.ep)return;i.ep=!0;const r=n(i);fetch(i.href,r)}})();function Jn(e){let t,n;try{const o=Ut(e,z.__wbindgen_malloc,z.__wbindgen_realloc),i=Be,r=z.check_tern(o,i);return t=r[0],n=r[1],Gt(r[0],r[1])}finally{z.__wbindgen_free(t,n,1)}}function Xn(e){let t,n;try{const o=Ut(e,z.__wbindgen_malloc,z.__wbindgen_realloc),i=Be,r=z.run_tern(o,i);return t=r[0],n=r[1],Gt(r[0],r[1])}finally{z.__wbindgen_free(t,n,1)}}function Qn(){return{__proto__:null,"./ternlang_wasm_bg.js":{__proto__:null,__wbindgen_init_externref_table:function(){const t=z.__wbindgen_externrefs,n=t.grow(4);t.set(0,void 0),t.set(n+0,void 0),t.set(n+1,null),t.set(n+2,!0),t.set(n+3,!1)}}}}function Gt(e,t){return e=e>>>0,eo(e,t)}let pe=null;function Se(){return(pe===null||pe.byteLength===0)&&(pe=new Uint8Array(z.memory.buffer)),pe}function Ut(e,t,n){if(n===void 0){const s=me.encode(e),d=t(s.length,1)>>>0;return Se().subarray(d,d+s.length).set(s),Be=s.length,d}let o=e.length,i=t(o,1)>>>0;const r=Se();let a=0;for(;a<o;a++){const s=e.charCodeAt(a);if(s>127)break;r[i+a]=s}if(a!==o){a!==0&&(e=e.slice(a)),i=n(i,o,o=a+e.length*3,1)>>>0;const s=Se().subarray(i+a,i+o),d=me.encodeInto(e,s);a+=d.written,i=n(i,o,a,1)>>>0}return Be=a,i}let ke=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});ke.decode();const Zn=2146435072;let Qe=0;function eo(e,t){return Qe+=t,Qe>=Zn&&(ke=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}),ke.decode(),Qe=t),ke.decode(Se().subarray(e,e+t))}const me=new TextEncoder;"encodeInto"in me||(me.encodeInto=function(e,t){const n=me.encode(e);return t.set(n),{read:e.length,written:n.length}});let Be=0,z;function to(e,t){return z=e.exports,pe=null,z.__wbindgen_start(),z}async function no(e,t){if(typeof Response=="function"&&e instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(e,t)}catch(i){if(e.ok&&n(e.type)&&e.headers.get("Content-Type")!=="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",i);else throw i}const o=await e.arrayBuffer();return await WebAssembly.instantiate(o,t)}else{const o=await WebAssembly.instantiate(e,t);return o instanceof WebAssembly.Instance?{instance:o,module:e}:o}function n(o){switch(o){case"basic":case"cors":case"default":return!0}return!1}}async function oo(e){if(z!==void 0)return z;e!==void 0&&(Object.getPrototypeOf(e)===Object.prototype?{module_or_path:e}=e:console.warn("using deprecated parameters for the initialization function; pass a single object instead")),e===void 0&&(e=new URL("/assets/ternlang_wasm_bg-13f2e166.wasm",self.location));const t=Qn();(typeof e=="string"||typeof Request=="function"&&e instanceof Request||typeof URL=="function"&&e instanceof URL)&&(e=fetch(e));const{instance:n,module:o}=await no(await e,t);return to(n)}oo().then(()=>{window.wasmRunTern=Xn,window.wasmCheckTern=Jn,window.wasmReady=!0,window.dispatchEvent(new Event("wasmready"))}).catch(()=>{window.wasmReady=!1});let Ie=null;async function io(){if(Ie)return Ie;try{const e=document.getElementById("sbWasmStatus");return e&&(e.textContent="Pyodide loading…"),Ie=await loadPyodide(),e&&(e.textContent="TernVM + Pyodide Ready"),Ie}catch(e){return console.error("Pyodide Load Error:",e),null}}async function Kt(e){const t=await io();if(!t)return{ok:!1,error:"Pyodide not available"};let n="";t.setStdout({batched:o=>{n+=o+`
`}}),t.setStderr({batched:o=>{n+="ERR: "+o+`
`}});try{return await t.runPythonAsync(e),{ok:!0,output:n.trim()}}catch(o){return{ok:!1,error:o.message,traceback:String(o)}}}window.runPythonActuator=Kt;if(window.location.hostname==="localhost"||window.location.hostname==="127.0.0.1"){const e=document.createElement("script");e.src=".ternstudio-local.js",e.onerror=()=>console.warn("Dev script missing (expected in local dev)"),document.head.appendChild(e)}const Te={1:"Tier 1 — Free",2:"Tier 2 — Pro",3:"Tier 3 — Industrial",4:"Tier 4 — Enterprise"},nt={1:"badge-free",2:"badge-t2",3:"badge-t3",4:"badge-t4"},ae={hello:`fn main() -> trit {
    print("⊕ Hello Ternary World");
    return affirm;
}
`,consensus:`// Ternary consensus: majority of N agents
fn main() -> trit {
    let a: trit = affirm;
    let b: trit = tend;
    let c: trit = affirm;
    // 2x affirm beats 1x tend → affirm
    print("⊕ Consensus: affirm");
    return affirm;
}`,gate:`// Ternary logic gate — routes on signal value
fn gate(signal: trit) -> trit {
    match signal {
        affirm => { print("● PASS — forwarding"); return affirm; }
        tend   => { print("○ HOLD — deliberating"); return tend; }
        reject => { print("✕ BLOCK — rejected"); return reject; }
    }
}

fn main() -> trit {
    return gate(affirm);
}`,agent:`// Basic agent pattern
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
}`,match:`fn classify(x: int) -> trit {
    if x > 0 { return truth();    }
    if x < 0 { return conflict(); }
    return hold();
}

let r: trit = classify(5);

match r {
    -1 => { return conflict(); }
     0 => { return hold();    }
     1 => { return truth();   }
}`,ema:`fn ema_gate(prior: float, evidence: float, alpha: float) -> trit {
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
}`};(function(){const e=window.location;if(e.hostname!=="localhost"&&e.hostname!=="127.0.0.1"&&e.protocol!=="file:"){const t=e.protocol+"//"+e.hostname+(e.port?":"+e.port:"");document.getElementById("apiEndpoint").value=t}})();let b=null,O=localStorage.getItem("ternstudio-active-file")||"examples/hello_trit.tern",j=JSON.parse(localStorage.getItem("ternstudio-tabs")||JSON.stringify([{name:"hello_trit.tern",path:"examples/hello_trit.tern"}])),B=JSON.parse(localStorage.getItem("ternstudio-file-buffers")||JSON.stringify({"examples/hello_trit.tern":ae.hello})),Vt=200;function ot(e){const t=parseInt(e);Vt=Math.max(0,1e3-t),localStorage.setItem("ternflow_sim_speed",t);const n=document.getElementById("simSpeedSlider");n&&(n.value=t)}window.updateSimSpeed=ot;function ro(){const e=localStorage.getItem("ternflow_sim_speed");ot(e!==null?e:800)}function be(){b&&(B[O]=b.getValue()),localStorage.setItem("ternstudio-active-file",O),localStorage.setItem("ternstudio-tabs",JSON.stringify(j)),localStorage.setItem("ternstudio-file-buffers",JSON.stringify(B))}window.saveEditorState=be;let ao=1,Y=[],Pt=0,Dt=0,zt=0;function Yt(){const e=document.getElementById("keyToggleArea"),t=document.getElementById("topbarActions"),n=e.style.display==="none";if(e.style.display=n?"flex":"none",t.style.display=n?"none":"flex",n){const o=document.getElementById("topbarKeyInput");o.value=document.getElementById("apiKey").value,setTimeout(()=>o.focus(),10)}}window.toggleKeyInput=Yt;function Jt(e){e=e.trim(),document.getElementById("apiKey").value=e,localStorage.setItem("ternstudio-key",e),Oe(),e&&(E("API Key updated","ok"),Yt(),Le())}window.updateApiKey=Jt;function so(e){const t=document.getElementById("stdlib-tree-container");t&&(t.style.display=e?"block":"none")}window.toggleStdlibVisibility=so;async function Le(){const e=document.getElementById("apiKey").value.trim(),t=document.getElementById("premium-tree-container"),n=document.getElementById("premium-file-tree");if(!e||!t||!n){t&&(t.style.display="none");return}t.style.display="block",n.innerHTML='<div class="tree-file" style="color:var(--muted)">Syncing premium assets...</div>';try{const o=document.getElementById("apiEndpoint").value.replace(/\/$/,""),i=await fetch(`${o}/api/premium/list`,{headers:{"X-Ternlang-Key":localStorage.getItem("ternstudio-key")||""}});if(!i.ok){const a=await i.text();console.error(`Premium fetch failed | Status: ${i.status} | URL: ${i.url} | Raw Response:`,a);const s=i.status===403?"Auth Failed. Invalid Key.":`HTTP Error ${i.status}`;n.innerHTML=`<div class="tree-file" style="color:var(--red)">${s}</div>`;return}const r=await i.json();r.status==="ok"&&Array.isArray(r.files)?Ge(n,r.files,!1,!0):n.innerHTML=`<div class="tree-file" style="color:var(--red)">Error: ${r.error||"Failed to load structure"}</div>`}catch(o){n.innerHTML='<div class="tree-file" style="color:var(--red)">Connection to API failed.</div>',console.error("Premium fetch error:",o)}}window.loadPremiumTree=Le;async function G(e){localStorage.setItem("ternstudio-last-view",e),document.querySelectorAll(".view").forEach(i=>i.classList.remove("active")),document.querySelectorAll(".nav-tab").forEach(i=>i.classList.remove("active"));const t=document.getElementById("view-"+e),n=document.getElementById("vt-"+e),o=document.getElementById("config-view");t&&t.classList.add("active"),n&&n.classList.add("active"),o&&(e==="settings"?o.style.display="flex":o.style.display="none"),e==="editor"&&b&&setTimeout(()=>b.layout(),50),e==="flow"&&(an(),xe()),e==="debugger"&&Xt(),e==="modules"&&await ut(),e==="fleet"&&await mt(),e==="settings"&&Yn(),lucide.createIcons()}window.switchView=G;function lo(e){const[t,n]=React.useState([]),[o,i]=React.useState(!1);return React.useEffect(()=>{const r=a=>{n(s=>[a.detail,...s].slice(0,1e3))};return window.addEventListener("ternlang_local_trace",r),()=>window.removeEventListener("ternlang_local_trace",r)},[]),React.useEffect(()=>{if(!e)return;const r=new WebSocket(e);return r.onopen=()=>{console.log("[Tracer WS] Handshake successful."),i(!0)},r.onmessage=a=>{console.log("[Tracer WS] Payload:",a.data);try{if(a.data==="connected")return;const s=JSON.parse(a.data);n(d=>[s,...d].slice(0,1e3))}catch{}},r.onclose=()=>i(!1),r.onerror=()=>console.error("[Tracer WS] Connection failed."),()=>r.close()},[e]),{telemetry:t,isConnected:o,clearTelemetry:()=>n([])}}function co({apiEndpoint:e}){const t=e.replace("http","ws")+"/api/tracer/ws",{telemetry:n,isConnected:o,clearTelemetry:i}=lo(t),r=a=>a===1?"#10b981":a===-1?"#ef4444":"#f59e0b";return React.createElement("div",{style:{padding:"24px",color:"#f1f5f9",fontFamily:"Inter, sans-serif"}},React.createElement("div",{style:{display:"flex",justifyContent:"space-between",alignItems:"center",marginBottom:"20px"}},React.createElement("h2",{style:{fontSize:"18px",fontWeight:"800",margin:0}},React.createElement("span",{style:{color:o?"#10b981":"#ef4444",marginRight:"8px"}},"●"),"Execution Tracer Pipeline"),React.createElement("button",{className:"btn btn-ghost",onClick:i,style:{fontSize:"12px"}},"Clear Trace")),React.createElement("div",{style:{background:"#1e293b",border:"1px solid #334155",borderRadius:"12px",overflow:"hidden"}},React.createElement("table",{style:{width:"100%",borderCollapse:"collapse",fontSize:"12px",textAlign:"left"}},React.createElement("thead",{style:{background:"#334155",color:"#cbd5e1",textTransform:"uppercase",letterSpacing:"0.05em"}},React.createElement("tr",{},["Timestamp","Node ID","Event","Result","Latency","Causal"].map(a=>React.createElement("th",{key:a,style:{padding:"12px 16px"}},a)))),React.createElement("tbody",{},n.length===0?React.createElement("tr",{},React.createElement("td",{colSpan:6,style:{padding:"4rem",textAlign:"center",color:"var(--muted)",fontStyle:"italic",fontSize:"1.1rem"}},"Awaiting telemetry firehose...")):n.map(a=>React.createElement("tr",{key:a.trace_id,style:{borderBottom:"1px solid #334155",opacity:a.sparse_dropped?.5:1}},React.createElement("td",{style:{padding:"12px 16px",color:"#cbd5e1"}},new Date(a.timestamp_ms).toLocaleTimeString()),React.createElement("td",{style:{padding:"12px 16px",fontWeight:"700"}},a.node_id),React.createElement("td",{style:{padding:"12px 16px"}},a.event_type),React.createElement("td",{style:{padding:"12px 16px"}},React.createElement("span",{style:{padding:"2px 8px",borderRadius:"4px",background:r(a.signal_out)+"22",color:r(a.signal_out),fontWeight:"800"}},a.signal_out>0?"+1":a.signal_out<0?"-1":"0")),React.createElement("td",{style:{padding:"12px 16px"}},`${a.latency_ms}ms`,a.sparse_dropped&&React.createElement("span",{style:{marginLeft:"8px",background:"#38bdf8",color:"#0f172a",fontSize:"9px",padding:"2px 6px",borderRadius:"4px",fontWeight:"900"}},"BYPASSED")),React.createElement("td",{style:{padding:"8px 16px"}},React.createElement("button",{className:"btn",style:{fontSize:"9px",height:"20px",background:"var(--bg2)",border:"1px solid var(--border2)",color:"var(--cyan)"},onClick:()=>window.downloadCausalArtifact(a.trace_id)},"Artifact"))))))))}let Ze=null;async function Xt(){const e=document.getElementById("view-debugger");if(!e||!window.ReactDOM)return;const t=document.getElementById("apiEndpoint").value;if(!Ze){e.innerHTML='<div id="tracer-react-mount" style="width:100%;"></div>';const n=document.getElementById("tracer-react-mount");Ze=ReactDOM.createRoot(n),Ze.render(React.createElement(co,{apiEndpoint:t}))}}window.renderTracerView=Xt;let Q=null;function po(e,t){Q=e,document.getElementById("deleteAgentName").textContent=t,document.getElementById("deleteModal").style.display="flex"}window.confirmDeleteAgent=po;function Qt(){document.getElementById("deleteModal").style.display="none",Q=null}window.closeDeleteModal=Qt;async function uo(){if(!Q)return;const e=document.getElementById("apiKey").value.trim(),t=document.getElementById("apiEndpoint").value.replace(/\/$/,"");try{if(e){const i=await(await fetch(`${t}/api/agent/${Q}`,{method:"DELETE",headers:{"X-Ternlang-Key":e}})).json();if(i.status!=="ok"){E("Server rejection: "+(i.error||"Unknown error"),"err");return}}let n=[];try{n=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}n=n.filter(o=>o.id!==Q),localStorage.setItem("ternflow_registry",JSON.stringify(n)),window.selectedFleetAgentId===Q&&(window.selectedFleetAgentId=n.length>0?n[0].id:null),Qt(),ut(),document.getElementById("view-fleet").classList.contains("active")&&mt(),E(`Agent "${Q}" permanently purged`,"ok")}catch(n){E("Purge failure: Connection lost","err"),console.error("Causal state failure:",n)}}window.deleteAgent=uo;async function he(){try{const t=await(await fetch("https://ternlang-api.fly.dev/api/agents",{headers:{"X-Ternlang-Key":localStorage.getItem("ternstudio-key")||""}})).json();if(t.status==="ok"&&t.agents){let n=[];try{n=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}let o=!1;t.agents.forEach(i=>{const r=n.find(a=>a.id===i.slug);r?(r.name=i.name,r.desc=i.desc,i.created_at&&(r.deployed=i.created_at),o=!0):(n.push({id:i.slug,slug:i.slug,name:i.name,desc:i.desc,pricing:i.pricing||"community",nodes:i.nodes||1,deployed:i.created_at||new Date().toISOString(),isRemote:!0}),o=!0)}),o&&localStorage.setItem("ternflow_registry",JSON.stringify(n))}}catch(e){console.warn("Fleet remote sync failed",e)}}window.syncFleetRegistry=he;async function ut(){const e=document.getElementById("view-modules");if(!e)return;await he();let t=[];try{t=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}let n=`
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
  `;t.length===0?n+='<div style="padding:30px; text-align:center; border:1px dashed var(--border2); border-radius:8px; color:var(--muted2); font-size:12px; margin-bottom:40px;">No custom architectures deployed yet.<br>Click "Deploy" in the Flow Lab to publish one.</div>':(n+='<div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; margin-bottom:40px;">',t.forEach(o=>{n+=`
        <div style="background:var(--bg2); border:1px solid var(--border2); border-radius:8px; padding:16px; position:relative;">
          <div style="display:flex; justify-content:space-between; align-items:flex-start; margin-bottom:8px;">
            <button onclick="confirmDeleteAgent('${o.id}', '${o.name}')" 
                    style="background:rgba(239,68,68,0.1); border:1px solid rgba(239,68,68,0.2); border-radius:4px; cursor:pointer; color:var(--red); padding:2px 6px; font-size:10px; font-weight:700;"
                    title="Delete Agent">DELETE</button>
            <div style="font-size:10px; padding:2px 6px; background:rgba(6,182,212,0.1); border-radius:4px; color:var(--cyan); border:1px solid var(--cyan);">${o.pricing}</div>
          </div>
          <div style="font-weight:700; color:var(--text); font-size:14px; margin-bottom:4px;">${o.name}</div>
          <div style="font-size:11px; color:var(--muted); margin-bottom:12px; height:32px; overflow:hidden;">${o.desc||"Custom ternary pipeline"}</div>
          <div style="display:flex; justify-content:space-between; font-size:10px; color:var(--muted2);">
            <span>Nodes: ${o.nodes}</span>
            <span>${new Date(o.deployed).toLocaleDateString()}</span>
          </div>
        </div>
      `}),n+="</div>"),n+=`
        <!-- Stdlib -->
        <h3 style="font-size: 16px; font-weight: 700; margin-bottom: 12px; border-bottom: 1px solid var(--border2); padding-bottom: 8px;">Standard Library (Remote)</h3>
        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; margin-bottom:40px;">
  `,Object.entries(Ae).forEach(([o,i])=>{n+=`
        <div style="background:var(--bg2); border:1px solid var(--border2); border-radius:8px; padding:16px;">
          <div style="display:flex; align-items:center; gap:8px; margin-bottom:8px;">
            <i data-lucide="${i.icon}" style="color:${i.color}; width:16px;"></i>
            <div style="font-weight:700; color:var(--text); font-size:14px;">${o}</div>
          </div>
          <div style="font-size:11px; color:var(--muted); margin-bottom:12px; height:32px; overflow:hidden;">${i.desc}</div>
          <div style="display:flex; justify-content:space-between; font-size:10px; color:var(--muted2);">
            <span style="color:var(--green)">Built-in</span>
            <span>TernFlow v1.0</span>
          </div>
        </div>
    `}),ht.forEach(o=>{const i=o.split("/").pop().replace(".tern","");n+=`
        <div style="background:var(--bg2); border:1px solid var(--border2); border-radius:8px; padding:16px;">
          <div style="display:flex; align-items:center; gap:8px; margin-bottom:8px;">
            <i data-lucide="bot" style="color:var(--muted); width:16px;"></i>
            <div style="font-weight:700; color:var(--text); font-size:14px;">${i}</div>
          </div>
          <div style="font-size:11px; color:var(--muted); margin-bottom:12px; height:32px; overflow:hidden; word-break:break-all;">${o}</div>
          <div style="display:flex; justify-content:space-between; font-size:10px; color:var(--muted2);">
            <span style="color:var(--amber)">GitHub</span>
            <span>Remote Sync</span>
          </div>
        </div>
    `}),n+="</div></div></div>",e.innerHTML=n,lucide.createIcons()}window.renderRegistryView=ut;window.selectedFleetAgentId=null;let mo={};async function mt(){const e=document.getElementById("view-fleet");if(!e)return;let t=[];try{t=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}t.length===0&&(e.innerHTML=`
      <div style="padding: 100px 40px; text-align: center; color:var(--text);">
        <div class="status-running" style="width:48px; height:48px; border-radius:50%; margin:0 auto 20px; display:flex; align-items:center; justify-content:center;">
          <i data-lucide="refresh-cw" class="spin" style="width:24px; height:24px;"></i>
        </div>
        <h2 style="font-size: 20px; font-weight: 700;">Hydrating Fleet...</h2>
        <p style="color: var(--muted); max-width:400px; margin: 10px auto;">Connecting to ternlang-api.fly.dev to fetch active deployment index.</p>
      </div>
    `,lucide.createIcons());try{await he();try{t=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}}catch(r){console.error("Fleet hydration failed:",r)}if(t.length===0){e.innerHTML=`
      <div style="padding: 100px 40px; text-align: center; color:var(--text);">
        <i data-lucide="tower-control" style="width:64px; height:64px; opacity:0.1; margin-bottom:20px;"></i>
        <h2 style="font-size: 20px; font-weight: 700;">Fleet Connection Offline</h2>
        <p style="color: var(--muted); max-width:400px; margin: 10px auto;">Could not reach the TIS deployment API. Check your network or deploy a local agent.</p>
        <button class="btn btn-primary" style="margin-top:20px;" onclick="switchView('flow')">Go to Flow Lab</button>
      </div>
    `,lucide.createIcons();return}!window.selectedFleetAgentId&&t.length>0&&(window.selectedFleetAgentId=t[0].id);const n=t.find(r=>r.id===window.selectedFleetAgentId)||t[0];if(!n)return;const o=mo[n.id]||{runs:Math.floor(Math.random()*100)+1,errors:Math.floor(Math.random()*5),avgConf:.85+Math.random()*.1,latency:120+Math.random()*200};let i=`
    <div style="display: flex; height: 100%; width:100%; overflow:hidden; color:var(--text);">
      <!-- Fleet Sidebar -->
      <div style="width: 280px; border-right: 1px solid var(--border); background: var(--bg1); display: flex; flex-direction: column; flex-shrink: 0;">
        <div style="padding: 16px; border-bottom: 1px solid var(--border); font-weight: 700; font-size: 11px; color: var(--muted); text-transform: uppercase; letter-spacing: 0.05em;">
          Live Fleet Units
        </div>
        <div style="flex: 1; overflow-y: auto; padding: 8px;">
          ${t.map(r=>{if(!r||!r.id)return'<div style="padding:12px; color:var(--red); border:1px solid var(--red); border-radius:6px; margin-bottom:4px;">Unknown Node</div>';const a=(r.id||r.name||r.endpoint||"unknown").substring(0,8);return`
            <div onclick="window.selectedFleetAgentId='${r.id}'; renderFleetView();" 
                 style="padding: 12px; border-radius: 6px; cursor: pointer; margin-bottom: 4px; transition: all 0.2s;
                        background: ${window.selectedFleetAgentId===r.id?"var(--active-file-bg)":"transparent"};
                        border: 1px solid ${window.selectedFleetAgentId===r.id?"var(--cyan)":"transparent"};">
              <div style="display:flex; justify-content:space-between; align-items:flex-start; margin-bottom:4px;">
                <div style="font-weight: 700; font-size: 13px; color: ${window.selectedFleetAgentId===r.id?"#fff":"var(--text)"}">${r.name||"Unnamed Agent"}</div>
                <div style="width:8px; height:8px; border-radius:50%; background:var(--green); box-shadow:0 0 8px var(--green); margin-top:4px;"></div>
              </div>
              <div style="font-size: 10px; color: var(--muted); display:flex; justify-content:space-between;">
                <span>v1.0.${a.substring(0,4)}</span>
                <span>Active</span>
              </div>
            </div>
          `}).join("")}
        </div>
      </div>

      <!-- Main Ops Panel -->
      <div style="flex: 1; display: flex; flex-direction: column; overflow-y: auto; background: var(--bg);">
        <!-- Ops Header -->
        <div style="padding: 24px 32px; border-bottom: 1px solid var(--border); display:flex; justify-content:space-between; align-items:center;">
          <div>
            <h2 style="font-size: 24px; font-weight: 800;">${n.name}</h2>
            <div style="display:flex; gap:12px; margin-top:4px; font-size:11px; color:var(--muted);">
              <span>ID: <code style="color:var(--cyan)">${n.id}</code></span>
              <span>•</span>
              <span>Deployed: ${new Date(n.deployed).toLocaleString()}</span>
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
            <input readonly value="https://ternlang-api.fly.dev/api/agent/${n.id}" 
                   style="flex:1; background:var(--bg); border:1px solid var(--border); border-radius:4px; padding:8px 12px; font-family:'JetBrains Mono',monospace; font-size:12px; color:var(--cyan); outline:none;">
            <button class="btn btn-primary" onclick="navigator.clipboard.writeText('https://ternlang-api.fly.dev/api/agent/${n.id}'); showToast('Endpoint copied', 'ok')">
              <i data-lucide="copy" style="width:14px;"></i> Copy
            </button>
          </div>
        </div>

        <!-- Metrics Grid -->
        <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; padding: 24px 32px;">
          <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; padding:16px;">
            <div style="font-size:10px; font-weight:700; color:var(--muted); text-transform:uppercase; margin-bottom:8px;">Executions (24h)</div>
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace;">${o.runs}</div>
            <div style="font-size:10px; color:var(--green); margin-top:4px;">↑ 12% vs baseline</div>
          </div>
          <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; padding:16px;">
            <div style="font-size:10px; font-weight:700; color:var(--muted); text-transform:uppercase; margin-bottom:8px;">Success Rate</div>
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace; color:var(--green);">${((o.runs-o.errors)/o.runs*100).toFixed(1)}%</div>
            <div style="font-size:10px; color:var(--muted); margin-top:4px;">${o.errors} rejections</div>
          </div>
          <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; padding:16px;">
            <div style="font-size:10px; font-weight:700; color:var(--muted); text-transform:uppercase; margin-bottom:8px;">Avg Confidence</div>
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace; color:var(--cyan);">${o.avgConf.toFixed(3)}</div>
            <div style="font-size:10px; color:var(--muted); margin-top:4px;">Stochastic Drift: 0.002</div>
          </div>
          <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; padding:16px;">
            <div style="font-size:10px; font-weight:700; color:var(--muted); text-transform:uppercase; margin-bottom:8px;">Latency (p95)</div>
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace;">${o.latency.toFixed(0)}ms</div>
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
              <div style="color:var(--muted2); margin-bottom:8px;">// Initializing stream for agent ${n.id}...</div>
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
  `;e.innerHTML=i,lucide.createIcons()}window.renderFleetView=mt;let f=[],x=[],R=[],M=null,_=null,N=!1,se=!1,P="idle";window.executionState="idle";window.simulationRunning=!1;window.simulationAborted=!1;let U=0,oe=0;function Fe(){const e=document.getElementById("simBtn");e&&(P==="running"?(e.innerHTML='<i data-lucide="pause" style="width:15px"></i> <span style="font-size:12px;font-weight:700;">PAUSE</span>',e.style.color="var(--amber)",e.classList.add("running")):P==="paused"?(e.innerHTML='<i data-lucide="play" style="width:15px"></i> <span style="font-size:12px;font-weight:700;">RESUME</span>',e.style.color="var(--green)",e.classList.add("running")):(e.innerHTML='<i data-lucide="play-circle" style="width:15px"></i> <span style="font-size:12px;font-weight:600;">SIMULATE</span>',e.style.color="var(--green)",e.classList.remove("running")),window.lucide&&lucide.createIcons())}window.updateSimUI=Fe;function Z(e){if(window.executionState===e)return;const t=window.executionState;console.log(`[TernFlow] Execution State -> ${e}`),window.executionState=e,P=e,se=e==="running",e==="idle"?N=!0:N=!1,e==="running"?E(t==="paused"?"Execution resumed":"Simulation started","info"):e==="paused"&&E("Execution paused","warn"),Fe(),window.dispatchEvent(new CustomEvent("executionstatechange",{detail:{state:e}}))}window.setExecutionState=Z;function ft(){return P!=="paused"?Promise.resolve():new Promise(e=>{const t=n=>{(n.detail.state==="running"||n.detail.state==="idle")&&(window.removeEventListener("executionstatechange",t),e())};window.addEventListener("executionstatechange",t)})}function fo(){P==="running"?Z("paused"):P==="paused"?(Z("running"),oe=performance.now(),requestAnimationFrame(window.currentDriveTimeline)):Zt()}window.toggleSimulation=fo;function Zt(){const e=document.getElementById("global-timeline"),t=document.getElementById("scrub-layer");f.forEach(n=>{n.visited=!1,n.executed=!1}),x.forEach(n=>{n.active=!1}),e&&(e.value=0),U=0,oe=performance.now(),t&&t.getContext("2d").clearRect(0,0,t.width,t.height),document.querySelectorAll(".trit-particle-ghost").forEach(n=>n.remove()),St()}window.startNewSimulation=Zt;function go(){Z("idle"),E("Simulation stopped and reset","warn"),U=0;const e=document.getElementById("global-timeline");e&&(e.value=0);const t=document.getElementById("scrub-layer");t&&t.getContext("2d").clearRect(0,0,t.width,t.height),document.querySelectorAll(".trit-particle-ghost").forEach(n=>n.remove())}window.stopSimulation=go;function yo(){const[e,t]=React.useState(window.executionState||"idle");React.useEffect(()=>{const s=d=>t(d.detail.state);return window.addEventListener("executionstatechange",s),()=>window.removeEventListener("executionstatechange",s)},[]),React.useEffect(()=>{window.lucide&&window.lucide.createIcons()},[e]);const n={display:"inline-flex",alignItems:"center",height:"32px",width:"140px",background:"var(--bg1)",border:"1px solid var(--border2)",borderRadius:"16px",overflow:"hidden",boxShadow:"var(--shadow)"},o=(s,d)=>({flex:1,display:"flex",alignItems:"center",justifyContent:"center",height:"100%",cursor:"pointer",transition:"all 0.2s ease",background:s?d:"transparent",color:s?"#fff":"var(--text)",borderRight:"1px solid var(--border2)",pointerEvents:"auto"}),i=s=>{s.preventDefault(),s.stopPropagation();const d=window.executionState||"idle";console.log("[ControlBar] Play Invoked. Current state:",d),d==="paused"?window.setExecutionState("running"):d==="idle"&&window.startNewSimulation()},r=s=>{s.preventDefault(),s.stopPropagation(),console.log("[ControlBar] Pause Invoked. Current state:",window.executionState),window.executionState==="running"&&window.setExecutionState("paused")},a=s=>{s.preventDefault(),s.stopPropagation(),console.log("[ControlBar] Stop Invoked."),window.stopSimulation()};return React.createElement("div",{style:n},React.createElement("div",{style:o(e==="running","#10b981"),title:"Play / Resume",onClick:i},React.createElement("i",{"data-lucide":"play",style:{width:"14px",pointerEvents:"none"}})),React.createElement("div",{style:o(e==="paused","#f59e0b"),title:"Pause",onClick:r},React.createElement("i",{"data-lucide":"pause",style:{width:"14px",pointerEvents:"none"}})),React.createElement("div",{style:{...o(!1,"transparent"),borderRight:"none"},title:"Stop & Reset",onClick:a,onMouseEnter:s=>s.currentTarget.style.color="#ef4444",onMouseLeave:s=>s.currentTarget.style.color="#f1f5f9"},React.createElement("i",{"data-lucide":"square",style:{width:"14px",pointerEvents:"none"}})))}function it(){const e=document.getElementById("control-bar-mount");if(!e||!window.ReactDOM)return;ReactDOM.createRoot(e).render(React.createElement(yo))}window.mountControlBar=it;async function rt(e,t){event==null||event.stopPropagation(),T("SYSTEM",`⚡ Manual injection: [v:${t}, c:1.0] -> ${e}`),ie.push({toId:e,val:t,conf:1,origin:"MANUAL_INJECTOR"}),se||St()}window.injectSignal=rt;const $e=new Set,at=new Set;function gt(e){$e.add(e),x.filter(n=>n.toId===e).forEach(n=>{at.add(n.id),$e.has(n.fromId)||gt(n.fromId)})}window.findParents=gt;function ho(e){document.querySelectorAll(".causal-path, .causal-node, .dimmed").forEach(n=>{n.classList.remove("causal-path","causal-node","dimmed")}),$e.clear(),at.clear(),gt(e),f.forEach(n=>{const o=document.getElementById(n.id);o&&($e.has(n.id)?o.classList.add("causal-node"):o.classList.add("dimmed"))}),x.forEach(n=>{const o=document.getElementById(n.id),i=document.getElementById("hit-"+n.id);o&&(at.has(n.id)?o.classList.add("causal-path"):(o.classList.add("dimmed"),i&&i.classList.add("dimmed")))}),E("Showing causal path for "+e,"ok");const t=()=>{document.querySelectorAll(".causal-path, .causal-node, .dimmed").forEach(n=>{n.classList.remove("causal-path","causal-node","dimmed")}),document.getElementById("flow-canvas").removeEventListener("mousedown",t)};document.getElementById("flow-canvas").addEventListener("mousedown",t)}window.traceCausalPath=ho;function en(){document.getElementById("macro-name-modal").style.display="none"}window.closeMacroModal=en;function tn(){if(k.size<2){E("Select at least 2 nodes to group","error");return}document.getElementById("macro-name-modal").style.display="flex",document.getElementById("macro-name-input").value="Logic_Module_"+Math.floor(Math.random()*1e3),setTimeout(()=>document.getElementById("macro-name-input").focus(),10)}window.groupSelectedNodes=tn;function vo(){const e=document.getElementById("macro-name-input").value.trim()||"Logic Module",t="macro_"+Date.now(),n=f.filter(y=>k.has(y.id)),o=f.filter(y=>!k.has(y.id));let i=1/0,r=1/0,a=-1/0,s=-1/0;n.forEach(y=>{const S=document.getElementById(y.id),$=parseFloat(S.style.left),W=parseFloat(S.style.top);i=Math.min(i,$),r=Math.min(r,W),a=Math.max(a,$+180),s=Math.max(s,W+100)});const d=(i+a)/2,l=(r+s)/2,c=n.map(y=>{const S=document.getElementById(y.id);return{...y,ox:parseFloat(S.style.left)-d,oy:parseFloat(S.style.top)-l}}),p=x.filter(y=>k.has(y.fromId)&&k.has(y.toId)),m=x.filter(y=>!k.has(y.fromId)&&k.has(y.toId)),u=x.filter(y=>k.has(y.fromId)&&!k.has(y.toId)),g=x.filter(y=>!k.has(y.fromId)&&!k.has(y.toId)),v={internal_graph:{nodes:c,wires:p},input_schema:"macro_in: trit",output_schema:"macro_out: trit",code:`// Encapsulated logic: ${n.length} nodes
// Internal routing preserved.`},w=[...g];m.forEach(y=>w.push({...y,toId:t,id:"wire_ext_in_"+Date.now()+Math.random(),originalToId:y.toId})),u.forEach(y=>w.push({...y,fromId:t,id:"wire_ext_out_"+Date.now()+Math.random(),originalFromId:y.fromId})),f=o,x=w,n.forEach(y=>{var S;return(S=document.getElementById(y.id))==null?void 0:S.remove()}),p.forEach(y=>{var S,$,W;(S=document.getElementById(y.id))==null||S.remove(),($=document.getElementById("hit-"+y.id))==null||$.remove(),(W=document.getElementById("badge-"+y.id))==null||W.remove()}),F(e,"__macro__",d,l,"macro",t);const I=f.find(y=>y.id===t);I&&(I.props=v),en(),ge(),A(),L(),E(`Grouped ${n.length} nodes into ${e}`,"ok")}window.confirmGroupNodes=vo;window.groupSelectedNodes=tn;function nn(e){var a;const t=f.find(s=>s.id===e);if((!t||!t.props.internal_graph)&&!(t&&t.props.nodes))return;const n=t.props.internal_graph,o=t.x,i=t.y,r=x.filter(s=>s.fromId===e||s.toId===e);f=f.filter(s=>s.id!==e),(a=document.getElementById(e))==null||a.remove(),n.nodes.forEach(s=>{const d=o+(s.ox||0),l=i+(s.oy||0);F(s.name,s.path,d,l,s.type,s.id);const c=f.find(p=>p.id===s.id);c&&(c.props=s.props)}),x=[...x.filter(s=>s.fromId!==e&&s.toId!==e),...n.wires],r.forEach(s=>{s.toId===e&&s.originalToId?x.push({...s,toId:s.originalToId,id:"wire_restitch_in_"+Date.now()+Math.random()}):s.fromId===e&&s.originalFromId&&x.push({...s,fromId:s.originalFromId,id:"wire_restitch_out_"+Date.now()+Math.random()})}),A(),L(),E(`Expanded "${t.name}"`,"ok")}window.expandMacro=nn;let h={scale:1,x:0,y:0};const xo=.15,wo=2.5;function ee(){const e=document.getElementById("flow-canvas");e&&(e.style.transform=`translate(${h.x}px,${h.y}px) scale(${h.scale})`);const t=document.getElementById("zoomLabel");t&&(t.textContent=Math.round(h.scale*100)+"%");const n=document.getElementById("flow-canvas-wrap");if(n){const o=Math.max(8,32*h.scale);n.style.backgroundSize=`${o}px ${o}px`,n.style.backgroundPosition=`${h.x}px ${h.y}px`}}window.applyTransform=ee;function yt(e,t,n){const o=h.scale*n;h.x=e-(e-h.x)*(o/h.scale),h.y=t-(t-h.y)*(o/h.scale),h.scale=o,ee(),le()}window.zoomAt=yt;function bo(e){const t=document.getElementById("flow-canvas-wrap"),n=t.clientWidth/2,o=t.clientHeight/2;yt(n,o,e>0?1.2:1/1.2)}window.zoomStep=bo;function Eo(){if(f.length===0){h.scale=1,h.x=0,h.y=0,ee();return}document.getElementById("flow-canvas");const e=document.getElementById("flow-canvas-wrap");let t=1/0,n=1/0,o=-1/0,i=-1/0;f.forEach(p=>{const m=document.getElementById(p.id);if(!m)return;const u=parseFloat(m.style.left)||0,g=parseFloat(m.style.top)||0,v=m.offsetWidth||200,w=m.offsetHeight||80;t=Math.min(t,u),n=Math.min(n,g),o=Math.max(o,u+v),i=Math.max(i,g+w)});const r=60,a=e.clientWidth-r*2,s=e.clientHeight-r*2,d=o-t,l=i-n,c=Math.min(wo,Math.max(xo,Math.min(a/(d||1),s/(l||1))));h.scale=c,h.x=r+(a-d*c)/2-t*c,h.y=r+(s-l*c)/2-n*c,ee()}window.fitToView=Eo;function He(e,t){return{x:(e-h.x)/h.scale,y:(t-h.y)/h.scale}}window.screenToCanvas=He;function on(){const e=document.getElementById("flow-canvas-wrap");if(!e)return;e.addEventListener("wheel",l=>{l.preventDefault();const c=e.getBoundingClientRect(),p=l.clientX-c.left,m=l.clientY-c.top,u=l.deltaY<0?1.1:1/1.1;yt(p,m,u)},{passive:!1});let t=!1,n=0,o=0,i=0,r=0,a=!1;document.addEventListener("keydown",l=>{l.code==="Space"&&document.activeElement.tagName!=="INPUT"&&document.activeElement.tagName!=="TEXTAREA"&&(a=!0,e.style.cursor="grab",l.preventDefault())}),document.addEventListener("keyup",l=>{l.code==="Space"&&(a=!1,e.style.cursor="",e.classList.remove("panning"))}),e.addEventListener("mousedown",l=>{if(l.button===1||l.button===0&&a)t=!0,n=l.clientX,o=l.clientY,i=h.x,r=h.y,e.classList.add("panning"),l.preventDefault();else if(l.button===0&&(l.target===e||l.target.id==="flow-canvas")&&!a){s=!0,d={x:l.clientX,y:l.clientY};const c=document.getElementById("rubber-band");c&&(c.style.display="block",c.style.left=l.clientX-e.getBoundingClientRect().left+"px",c.style.top=l.clientY-e.getBoundingClientRect().top+"px",c.style.width="0",c.style.height="0"),l.shiftKey||ge()}}),e.addEventListener("dragover",l=>{l.preventDefault(),l.dataTransfer.dropEffect="copy"}),e.addEventListener("drop",async l=>{l.preventDefault();const c=l.dataTransfer.getData("tern-node-type"),p=e.getBoundingClientRect(),m=l.clientX-p.left,u=l.clientY-p.top,g={x:(m-h.x)/h.scale,y:(u-h.y)/h.scale};if(c==="agent"){const v=l.dataTransfer.getData("tern-node-name"),w=l.dataTransfer.getData("tern-node-path"),I=l.dataTransfer.getData("tern-node-code"),y="node_"+Date.now();if(F(v,w,g.x,g.y,"agent",y),I){const S=f.find($=>$.id===y);S&&(S.props.code=I,S.props.input_schema="signal: trit",S.props.output_schema="signal: trit")}else if(w!=="__builtin__")try{const S=await fetch(qe+w);if(S.ok){const $=await S.text(),W=f.find(C=>C.id===y);W&&(W.props.code=$,L())}}catch{}}else if(c==="archetype"){const v=l.dataTransfer.getData("tern-arch-id"),w=fn.find(I=>I.id===v);w&&Et(w,g.x-300,g.y-200)}});let s=!1,d={};document.addEventListener("mousemove",l=>{if(t&&(h.x=i+(l.clientX-n),h.y=r+(l.clientY-o),ee(),le()),s){const c=e.getBoundingClientRect(),p=Math.min(d.x,l.clientX)-c.left,m=Math.min(d.y,l.clientY)-c.top,u=Math.max(d.x,l.clientX)-c.left,g=Math.max(d.y,l.clientY)-c.top,v=document.getElementById("rubber-band");v&&(v.style.left=p+"px",v.style.top=m+"px",v.style.width=u-p+"px",v.style.height=g-m+"px")}}),document.addEventListener("mouseup",l=>{if(t&&(t=!1,e.classList.remove("panning")),s){s=!1;const c=document.getElementById("rubber-band");c&&(c.style.display="none");const p=Math.min(d.x,l.clientX),m=Math.max(d.x,l.clientX),u=Math.min(d.y,l.clientY),g=Math.max(d.y,l.clientY);if(Math.sqrt(Math.pow(l.clientX-d.x,2)+Math.pow(l.clientY-d.y,2))>5)f.forEach(w=>{const I=document.getElementById(w.id);if(!I)return;const y=I.getBoundingClientRect(),S=y.left+y.width/2,$=y.top+y.height/2;S>=p&&S<=m&&$>=u&&$<=g&&Re(w.id,!0)});else{const w=document.getElementById("flow-canvas-wrap");(l.target===w||l.target.id==="flow-canvas")&&ge()}}}),document.addEventListener("keydown",l=>{(l.key==="Delete"||l.key==="Backspace")&&document.activeElement.tagName!=="INPUT"&&document.activeElement.tagName!=="TEXTAREA"&&un(),l.key==="Escape"&&ge()})}window.initCanvasInteraction=on;const Ot={"Guardrails & Safety":["SafetyGate","OutputGuard","gatekeeper","range_validator","string_validator","validator","float_threshold","watchdog","supervisor","retry"],"Deliberation & Evaluation":["Classifier","Ranker","Proposer","Challenger","Arbiter","deliberator"],"Routing & Aggregation":["ConsensusGate","consensus","aggregator","filter","binary_bridge","router","pipeline","majority_5","weighted_consensus","mapper","transformer"],"Memory & Persistence":["ContextBuffer","EpisodicRecall","StateInjector","sqlite_bridge"],"Sparse Math & Compute":["SparseMatMul","WeightPruner","TernaryQuantizer"],"Interoperability & Protocol":["MCPBridge","LocalNodeSync","VetoOrchestrator"],"I/O & Execution":["Sensor","Actuator","broadcast","echo","logger","scaler"]};let et={"Guardrails & Safety":!1,"Deliberation & Evaluation":!1,"Routing & Aggregation":!1,"Memory & Persistence":!1,"Sparse Math & Compute":!1,"Interoperability & Protocol":!1,"I/O & Execution":!1};const jt={"Orchestration & Consensus":["moe_13_flagship","consensus","industry_enterprise_risk","recursive_refiner","kmu_hiring_decision","kmu_supplier_score","kmu_customer_qual"],"Evaluation & Debate":["debate","filter_rank","kmu_process_opt","sensor_gate","industry_sme_pipeline"],"Safety & Guardrails":["guardrail","kmu_invoice_fraud","industry_iot_grid"],"Memory & Persistence":["local_rag_pipeline","episodic_reflection"],"High-Performance Compute":["quantized_sparse_accelerator"],"External Interoperability":["hard_gated_mcp","swarm_consensus"]};let tt={"Orchestration & Consensus":!1,"Evaluation & Debate":!1,"Safety & Guardrails":!1,"Memory & Persistence":!1,"High-Performance Compute":!1,"External Interoperability":!1};const ve={delayTimer:null,show:function(e,t,n){const o=document.getElementById("global-tooltip");if(!o)return;o.textContent=e,o.style.left=t+15+"px",o.style.top=n+10+"px",o.classList.add("visible");const i=o.getBoundingClientRect();i.right>window.innerWidth&&(o.style.left=t-i.width-15+"px")},hide:function(){this.delayTimer&&clearTimeout(this.delayTimer);const e=document.getElementById("global-tooltip");e&&e.classList.remove("visible")},startDelay:function(e,t,n){this.hide(),this.delayTimer=setTimeout(()=>{this.show(e,t,n)},400)}};window.TooltipController=ve;const Ae={Sensor:{desc:"The starting point. It reads external data and brings it into the ternary workflow.",code:`fn main() -> trit {
    // Sensor: validate input
    let raw: trit = read_input();
    if raw == affirm { return affirm; }
    return tend;
}`,icon:"radio",color:"var(--cyan)"},SafetyGate:{desc:"A strict bouncer. It immediately kills the workflow if the incoming signal is flagged as dangerous.",code:`fn main() -> trit {
    let sig: trit = read_input();
    if sig == reject { return reject; }
    return affirm;
}`,icon:"shield-check",color:"var(--green)"},ConsensusGate:{desc:"The decider. It looks at multiple opinions and moves forward with the majority vote.",code:`fn main() -> trit {
    let a: trit = read_input();
    let b: trit = read_input();
    let c: trit = read_input();
    if a == affirm && b == affirm { return affirm; }
    if a == reject && b == reject { return reject; }
    return tend;
}`,icon:"git-merge",color:"var(--cyan)"},Classifier:{desc:"The sorter. It looks at raw data and categorizes it into true, false, or uncertain.",code:`fn main() -> trit {
    let input: trit = read_input();
    match input {
        affirm => return affirm,
        tend   => return tend,
        reject => return reject,
    }
}`,icon:"layers",color:"var(--amber)"},Actuator:{desc:"The final step. It triggers real-world actions like sending an email or saving a file.",code:`fn main() -> trit {
    let decision: trit = read_input();
    if decision == affirm {
        emit "ACTION: execute";
        return affirm;
    }
    emit "ACTION: skip";
    return tend;
}`,icon:"zap-off",color:"var(--green)"},Ranker:{desc:"The judge. It scores different options and picks the best one based on your rules.",code:`fn main() -> trit {
    let score: trit = read_input();
    if score == affirm { return affirm; }
    if score == tend   { return tend;   }
    return reject;
}`,icon:"bar-chart-2",color:"var(--muted)"},Proposer:{desc:"The idea generator. It throws out a potential solution for other agents to debate.",code:`fn main() -> trit {
    emit "PROPOSE: candidate solution";
    return affirm;
}`,icon:"message-square",color:"var(--blue)"},Challenger:{desc:"The devil's advocate. It deliberately tries to find flaws in proposed solutions.",code:`fn main() -> trit {
    emit "CHALLENGE: counter-argument";
    return tend;
}`,icon:"swords",color:"var(--red)"},Arbiter:{desc:"The final judge in a debate. It listens to all sides and makes the ultimate call.",code:`fn main() -> trit {
    let a: trit = read_input();
    let b: trit = read_input();
    if a == affirm && b == tend { return affirm; }
    if a == tend   && b == tend { return tend;   }
    return reject;
}`,icon:"scale",color:"var(--amber)"},OutputGuard:{desc:"The last line of defense. It checks the final output before the AI is allowed to act.",code:`fn main() -> trit {
    let sig: trit = read_input();
    if sig == reject { emit "BLOCKED"; return reject; }
    emit "PASS";
    return affirm;
}`,icon:"shield",color:"var(--green)"},ContextBuffer:{desc:"Short-term memory. It holds onto recent inputs so the AI remembers what you just talked about.",code:`fn main() -> trit {
    let ctx: string = read_context();
    if ctx.length > 0 { return affirm; }
    return tend;
}`,icon:"database",color:"var(--cyan)"},EpisodicRecall:{desc:"Long-term memory. It searches past conversations to find similar situations.",code:`fn main() -> trit {
    let key: trit = read_input();
    let found: bool = recall(key);
    return found ? affirm : reject;
}`,icon:"history",color:"var(--amber)"},StateInjector:{desc:"The override switch. It forces the AI into a specific state, ignoring normal logic.",code:`fn main() -> trit {
    inject_state(affirm);
    return affirm;
}`,icon:"microchip",color:"var(--blue)"},sqlite_bridge:{desc:"The database connector. It saves and loads information directly from your local hard drive.",code:`fn main() -> trit {
    db_execute("INSERT INTO logs (val) VALUES (+1)");
    return affirm;
}`,icon:"table",color:"var(--muted)"},SparseMatMul:{desc:"The math engine. It multiplies numbers but skips zero-states entirely to run incredibly fast on the Albert VM.",code:`@sparseskip
fn main() -> trit {
    return matmul_step();
}`,icon:"grid",color:"var(--cyan)"},WeightPruner:{desc:"The optimizer. It cleans up the AI's brain by deleting weak connections to make it run faster.",code:`fn main() -> trit {
    prune_weights(0.1);
    return affirm;
}`,icon:"scissors",color:"var(--red)"},TernaryQuantizer:{desc:"The simplifier. It turns messy, complex numbers into simple true, false, or uncertain signals.",code:`fn main() -> trit {
    let val: float = read_float();
    return quantize(val);
}`,icon:"binary",color:"var(--blue)"},MCPBridge:{desc:"The tool connector. It allows your agents to talk to external apps and APIs.",code:`fn main() -> trit {
    return mcp_call("identity");
}`,icon:"external-link",color:"var(--green)"},LocalNodeSync:{desc:"The network cable. It keeps multiple AI brains on the same page so they can work together.",code:`fn main() -> trit {
    sync_fleet();
    return affirm;
}`,icon:"refresh-cw",color:"var(--cyan)"},VetoOrchestrator:{desc:"The ultimate kill switch. If even one agent says no, the whole process shuts down immediately.",code:`fn main() -> trit {
    let votes: trit[] = read_all();
    if votes.contains(reject) { return reject; }
    return affirm;
}`,icon:"octagon",color:"var(--red)"}};function L(){try{const e={nodes:f.map(t=>{var n,o;return{id:t.id,name:t.name,path:t.path,type:t.type,props:t.props,x:parseInt(((n=document.getElementById(t.id))==null?void 0:n.style.left)||0),y:parseInt(((o=document.getElementById(t.id))==null?void 0:o.style.top)||0)}}),wires:x};localStorage.setItem("ternflow_canvas",JSON.stringify(e))}catch{}}window.saveCanvasState=L;function rn(){try{const e=localStorage.getItem("ternflow_canvas");if(!e)return!1;const t=JSON.parse(e);if(!t.nodes||t.nodes.length===0)return!1;if(t.nodes.forEach(n=>F(n.name,n.path,n.x,n.y,n.type,n.id,n.isStub)),t.nodes.forEach(n=>{const o=f.find(i=>i.id===n.id);o&&(o.props=n.props,wt(n.id))}),x=t.wires||[],A(),f.length>0){const n=document.getElementById("canvas-hint");n&&(n.style.display="none")}return!0}catch{return!1}}window.restoreCanvasState=rn;function Io(){document.querySelectorAll(".flow-node").forEach(n=>n.remove()),document.querySelectorAll(".edge-badge").forEach(n=>n.remove());const e=document.getElementById("flow-svg-layer");e&&(e.innerHTML=""),document.getElementById("wire-handle").classList.remove("active"),f=[],x=[],M=null,H=null,k=new Set,D(),A();const t=document.getElementById("canvas-hint");t&&(t.style.display="flex"),localStorage.removeItem("ternflow_canvas"),E("Canvas cleared","ok")}window.clearCanvas=Io;let Ft=!1;function an(){if(N=!1,Ft||(on(),kn(),Tn(),ee(),Ft=!0),xe(),f.length===0&&!rn()){const t=document.getElementById("canvas-hint");t&&(t.style.display="flex")}}window.renderFlow=an;let ht=[];async function xe(){we([],"");try{const e=document.getElementById("apiEndpoint").value.replace(/\/$/,""),t=document.getElementById("apiKey").value.trim(),o=await(await fetch(e+"/api/stdlib/list",{headers:t?{"X-Ternlang-Key":t}:{},signal:AbortSignal.timeout(3e3)})).json();let i=o.status==="ok"&&o.files?o.files.filter(r=>r.includes("agents/")):[];i.length>0&&(ht=i,we(i),ln(i))}catch{}}window.renderFlowLibrary=xe;function sn(e){const t=e.toLowerCase();return t.includes("float_threshold")?{icon:"sliders-horizontal",color:"#22d3ee"}:t.includes("gatekeeper")?{icon:"lock",color:"#fbbf24"}:t.includes("logger")?{icon:"clipboard",color:"#78350f"}:t.includes("majority_5")?{icon:"network",color:"#a855f7"}:t.includes("mapper")?{icon:"grid",color:"#d946ef"}:t.includes("pipeline")?{icon:"rows",color:"#4ade80"}:t.includes("range_validator")?{icon:"frame",color:"#fb923c"}:t.includes("retry")?{icon:"rotate-ccw",color:"#f472b6"}:t.includes("router")?{icon:"git-branch",color:"#1d4ed8"}:t.includes("scaler")?{icon:"maximize",color:"#ffffff"}:t.includes("string_validator")?{icon:"text-cursor",color:"#a3e635"}:t.includes("supervisor")?{icon:"eye",color:"#000000"}:t.includes("transformer")?{icon:"box",color:"#8b5cf6"}:t.includes("validator")?{icon:"shield-check",color:"#15803d"}:t.includes("watchdog")?{icon:"bell",color:"#ef4444"}:t.includes("aggregator")||t.includes("collect")||t.includes("unify")?{icon:"filter",color:"#a855f7"}:t.includes("binary_bridge")||t.includes("cross")||t.includes("legacy")?{icon:"shuffle",color:"#f97316"}:t.includes("broadcast")||t.includes("emit")||t.includes("speak")?{icon:"megaphone",color:"#eab308"}:t.includes("consensus")&&!t.includes("gate")?{icon:"link-2",color:"#14b8a6"}:t.includes("deliberator")||t.includes("think")||t.includes("weight")?{icon:"brain-circuit",color:"#d946ef"}:t.includes("echo")||t.includes("mirror")||t.includes("repeat")?{icon:"waves",color:"#22c55e"}:t.includes("filter")||t.includes("sieve")||t.includes("isolate")?{icon:"layers",color:"#b91c1c"}:t.includes("sensor")||t.includes("input")||t.includes("read")?{icon:"radio",color:"var(--cyan)"}:t.includes("safety")||t.includes("guard")||t.includes("check")?{icon:"shield-check",color:"var(--green)"}:t.includes("consens")||t.includes("vote")||t.includes("aggregate")?{icon:"git-pull-request",color:"var(--blue)"}:t.includes("classif")||t.includes("match")||t.includes("sort")?{icon:"layers",color:"var(--amber)"}:t.includes("actuat")||t.includes("output")?{icon:"zap-off",color:"var(--red)"}:t.includes("rank")||t.includes("score")?{icon:"bar-chart-2",color:"var(--cyan)"}:t.includes("propos")?{icon:"message-square",color:"var(--blue)"}:t.includes("challeng")||t.includes("debat")?{icon:"flame",color:"var(--amber)"}:t.includes("logic")||t.includes("math")||t.includes("calc")?{icon:"variable",color:"var(--cyan)"}:t.includes("finance")||t.includes("econ")||t.includes("price")?{icon:"trending-up",color:"var(--green)"}:t.includes("hardware")||t.includes("cpu")||t.includes("fpga")?{icon:"cpu",color:"var(--cyan)"}:{icon:"bot",color:"var(--blue)"}}window.getAgentIcon=sn;function we(e,t=""){const n=document.getElementById("flow-lib-items");if(!n)return;n.innerHTML="";const o={};Object.keys(Ot).forEach(r=>o[r]=[]);const i=r=>{for(const[a,s]of Object.entries(Ot))if(s.some(d=>d.toLowerCase()===r.toLowerCase()))return a;return console.warn(`[Taxonomy] Agent "${r}" is not mapped to any strict category. Dropping from render tree.`),null};Object.entries(Ae).forEach(([r,a])=>{if(t&&!r.toLowerCase().includes(t.toLowerCase()))return;const s=i(r);s&&o[s].push({name:r,agent:a,type:"builtin"})}),e.forEach(r=>{const a=r.split("/").pop().replace(".tern","");if(t&&!a.toLowerCase().includes(t.toLowerCase()))return;const s=i(a);s&&o[s].push({name:a,path:r,type:"api"})}),Object.entries(o).forEach(([r,a])=>{if(a.length===0)return;const s=t?!0:et[r],d=document.createElement("div");d.className="lib-category"+(s?"":" collapsed");const l=document.createElement("div");if(l.className="lib-category-header",l.style.display="flex",l.style.justifyContent="space-between",l.style.alignItems="center",l.innerHTML=`<span>${r}</span><i data-lucide="chevron-down"></i>`,l.onclick=()=>{et[r]=!et[r],we(e,t)},d.appendChild(l),s){const c=document.createElement("div");c.className="lib-category-items",a.forEach(p=>{const m=document.createElement("div");m.className="lib-item";let u,g;if(p.type==="builtin")u=p.agent.icon,g=p.agent.color;else{const v=sn(p.name);u=v.icon,g=v.color}m.draggable=!0,m.ondragstart=v=>{v.dataTransfer.setData("tern-node-type","agent"),v.dataTransfer.setData("tern-node-name",p.name),v.dataTransfer.setData("tern-node-path",p.type==="builtin"?"__builtin__":p.path),p.type==="builtin"&&v.dataTransfer.setData("tern-node-code",p.agent.code)},m.innerHTML=`<i data-lucide="${u}" style="color:${g}"></i> <span>${p.name}</span>`,m.onmouseenter=v=>{const w=p.type==="builtin"?p.agent.desc:"Custom ternary pipeline defined in "+p.path;ve.startDelay(w,v.clientX,v.clientY)},m.onmouseleave=()=>ve.hide(),m.onclick=async()=>{const v="node_"+Date.now(),w=te((Math.random()-.5)*120,(Math.random()-.5)*80);if(F(p.name,p.type==="builtin"?"__builtin__":p.path,w.x,w.y,"agent",v),p.type==="builtin"){const I=f.find(y=>y.id===v);I&&(I.props.code=p.agent.code,I.props.input_schema="signal: trit",I.props.output_schema="signal: trit")}else try{const I=await fetch(qe+p.path);if(I.ok){const y=await I.text(),S=f.find($=>$.id===v);S&&(S.props.code=y,M===v&&D(),L())}}catch{}},c.appendChild(m)}),d.appendChild(c)}n.appendChild(d)}),n.children.length===0&&(n.innerHTML='<div style="padding:16px; color:var(--muted2); font-size:11px; text-align:center;">No agents found.<br>Use + to create one.</div>'),lucide.createIcons()}window.renderFlowLibItems=we;function So(e){we(ht,e)}window.filterFlowLib=So;function ln(e){const t=document.getElementById("newAgentLibPick");t&&(t.innerHTML='<option value="">— Start blank —</option>',e.forEach(n=>{const o=document.createElement("option");o.value=n,o.textContent=n.split("/").pop().replace(".tern",""),t.appendChild(o)}))}window.populateNewAgentPicker=ln;function ko(){document.getElementById("newAgentModal").style.display="flex",document.getElementById("newAgentName").value="",document.getElementById("newAgentCode").value=`fn main() -> trit {
    return affirm;
}`}window.openNewAgentModal=ko;function vt(){document.getElementById("newAgentModal").style.display="none"}window.closeNewAgentModal=vt;document.addEventListener("keydown",e=>{e.key==="Escape"&&vt()});async function To(){const e=document.getElementById("newAgentLibPick").value;if(!e){document.getElementById("newAgentCode").value=`fn main() -> trit {
    return affirm;
}`;return}try{const t=await fetch(qe+e);t.ok&&(document.getElementById("newAgentCode").value=await t.text())}catch{}}window.loadNewAgentTemplate=To;function te(e=0,t=0){const n=document.getElementById("flow-canvas-wrap");if(!n)return{x:0,y:0};const o=n.getBoundingClientRect(),i=o.width/2+e,r=o.height/2+t;return{x:(i-h.x)/h.scale,y:(r-h.y)/h.scale}}window.viewportCenterInCanvas=te;function Co(){const e=document.getElementById("newAgentName").value.trim()||"Agent",t=document.getElementById("newAgentCode").value,n="node_"+Date.now(),{x:o,y:i}=te((Math.random()-.5)*100,(Math.random()-.5)*80);F(e,"__custom__",o,i,"agent",n);const r=f.find(a=>a.id===n);r&&(r.props.code=t),vt(),E(`Agent "${e}" added`,"ok")}window.addAgentFromModal=Co;const _o=[{id:"safety",label:"Safety",weight:.15,crit:!0},{id:"metasafety",label:"MetaSafety",weight:.15,crit:!0},{id:"logic",label:"Logic",weight:.08},{id:"ethics",label:"Ethics",weight:.1},{id:"factcheck",label:"FactCheck",weight:.08},{id:"causal",label:"Causal",weight:.07},{id:"context",label:"Context",weight:.07},{id:"history",label:"History",weight:.05},{id:"ambiguity",label:"Ambiguity",weight:.05},{id:"math",label:"Math",weight:.05},{id:"tooluse",label:"ToolUse",weight:.05},{id:"persona",label:"Persona",weight:.05},{id:"efficiency",label:"Efficiency",weight:.05}];function dn(e,t,n,o){const r=n*1.1,a=o*1.1;let s=!0,d=t,l=0;for(;s&&l<10;){s=!1;for(const c of f){const p=c.type==="artifact"?300:c.type==="moe13"?320:180,m=c.type==="artifact"?200:c.type==="moe13"?360:80,u=Math.abs(e-c.x)<(r+p)/2,g=Math.abs(d-c.y)<(a+m)/2;if(u&&g){s=!0,d+=150;break}}l++}return{x:e,y:d}}window.findClearSpace=dn;function xt(e,t){const n=document.getElementById("flow-canvas-wrap");if(!n)return;const o=n.clientWidth,i=n.clientHeight,r=o/2-e*h.scale,a=i/2-t*h.scale,s=h.x,d=h.y,l=600,c=performance.now();function p(m){const u=m-c,g=Math.min(u/l,1),v=1-Math.pow(1-g,3);h.x=s+(r-s)*v,h.y=d+(a-d)*v,ee(),g<1&&requestAnimationFrame(p)}requestAnimationFrame(p)}window.panToCenter=xt;function F(e,t,n,o,i="agent",r,a=!1){var $,W;const s=document.getElementById("flow-canvas"),d=document.createElement("div");d.id=r;const l=i==="external"?" external":i==="gate"?" gate":i==="artifact"?" artifact":i==="moe13"?" moe13":i==="datasource"?" datasource":"";d.className="flow-node"+l,a&&d.classList.add("artifact-stub"),i==="datasource"&&(d.style.borderLeft="4px solid #f43f5e",d.style.borderRadius="0 8px 8px 0");const c=i==="artifact"?300:i==="moe13"?320:180,p=i==="artifact"?200:i==="moe13"?360:80;d.style.left=n-c/2+"px",d.style.top=o-p/2+"px",xt(n,o),(i==="artifact"||i==="moe13")&&(d.style.width=c+"px",d.style.height=p+"px",d.style.display="flex",d.style.flexDirection="column");let m="bot",u="var(--cyan)",g="AGENT";i==="external"?(m="zap",u="var(--amber)",g="LLM BRIDGE"):i==="gate"?(m="git-merge",u="var(--cyan)",g="TRIT GATE"):i==="moe13"?(m="brain-circuit",u="var(--magenta)",g="MOE-13 ORCHESTRATOR"):i==="artifact"?(m="file-text",u="var(--green)",g="RESULT ARTIFACT"):i==="datasource"&&(m="database",u="#f43f5e",g="DATA SOURCE");let v=`
    <div style="font-weight:600; color:var(--text); font-size:13px; margin-bottom:3px;" class="fn-title">${e}</div>
    <div style="font-size:10px; color:var(--muted2); font-family:'JetBrains Mono',monospace;">${t.split("/").pop()}</div>
  `;i==="moe13"&&(v=`
       <div style="font-weight:700; color:var(--magenta); font-size:10px; text-transform:uppercase; margin-bottom:10px; letter-spacing:1px; display:flex; justify-content:space-between;">
         <span>Deliberation Axes</span>
         <span id="moe-verdict-${r}" style="color:var(--muted2)">PENDING</span>
       </div>
       <div style="flex:1; display:grid; grid-template-columns: 1fr 40px 40px; gap:4px; font-family:'JetBrains Mono',monospace; font-size:9px;">
         ${_o.map(C=>`
           <div style="color:var(--text); opacity:0.8;">${C.label}</div>
           <div id="moe-vote-${r}-${C.id}" style="text-align:center; color:var(--muted2); font-weight:700;">0</div>
           <div id="moe-conf-${r}-${C.id}" style="text-align:right; color:var(--cyan);">0%</div>
         `).join("")}
       </div>
       <div id="moe-veto-alert-${r}" style="display:none; margin-top:10px; padding:6px; background:rgba(239,68,68,0.2); border:1px solid var(--red); color:var(--red); font-size:9px; font-weight:800; text-align:center; border-radius:4px;">
         🛑 CRITICAL SAFETY VETO ENGAGED
       </div>
     `),i==="artifact"&&(v=`
       <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:8px;">
         <div style="font-weight:700; color:var(--green); font-size:10px; text-transform:uppercase; letter-spacing:0.5px;">Payload Resolution</div>
         <div class="art-ctrls" style="display:flex; gap:4px;">
           <button class="art-btn" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); setArtifactState('${r}', 'lock')" title="Lock as Static Data"><i data-lucide="lock" style="width:10px"></i></button>
           <button class="art-btn" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); setArtifactState('${r}', 'transmute')" title="Transmute to Editor"><i data-lucide="edit-3" style="width:10px"></i></button>
           <button class="art-btn" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); setArtifactState('${r}', 'extend')" title="Extend Topology"><i data-lucide="external-link" style="width:10px"></i></button>
         </div>
       </div>
       <div id="art-body-${r}" class="art-display" style="flex:1; overflow-y:auto; font-family:'JetBrains Mono',monospace; font-size:11px; color:var(--text); background:rgba(0,0,0,0.2); padding:8px; border-radius:4px; border:1px solid var(--border2); white-space:pre-wrap;">(Awaiting signal...)</div>
       <textarea id="art-edit-${r}" class="art-editor" style="display:none; flex:1; background:var(--bg2); color:var(--cyan); font-family:'JetBrains Mono',monospace; font-size:11px; border:1px solid var(--cyan); padding:8px; border-radius:4px; outline:none; resize:none;" oninput="updateArtifactPayload('${r}', this.value)"></textarea>
       <div id="art-socket-label-${r}" style="margin-top:8px; display:none; justify-content:flex-end;">
         <div style="font-size:9px; color:var(--green); font-weight:800; border:1px solid var(--green); padding:2px 4px; border-radius:3px;">EXTEND SOCKET ACTIVE</div>
       </div>
     `);const w=i==="external"?`
    <div style="position:absolute; top:-8px; right:10px; background:var(--amber); color:var(--bg1); font-size:8px; font-weight:800; padding:2px 6px; border-radius:10px; box-shadow:0 2px 8px rgba(245,158,11,0.4); z-index:11;">PROBABILISTIC LLM</div>
  `:"";if(d.innerHTML=`
    ${w}
    <div class="fn-head">
      <div style="display:flex; align-items:center; gap:6px; font-size:10px; font-weight:700; color:${u}">
        <i data-lucide="${m}" style="width:12px"></i>
        ${g}
      </div>
      <div class="fn-status" id="status-${r}" title="idle"></div>
      <button onclick="event.stopPropagation(); traceCausalPath('${r}')" style="padding:2px 4px; background:none; border:none; cursor:pointer; color:var(--cyan); line-height:1; margin-left:4px;" title="Causal Trace">🔍</button>
      <button onclick="event.stopPropagation(); deleteNode('${r}')" style="padding:2px 4px; background:none; border:none; cursor:pointer; color:var(--muted); line-height:1; margin-left:4px;" title="Remove">✕</button>
    </div>
    <div class="fn-body" style="${i==="artifact"?"flex:1; display:flex; flex-direction:column; overflow:hidden;":""}">
      ${v}
    </div>
    <div class="fn-injectors">
      <div class="inj-btn inj-pos" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); injectSignal('${r}', 1)" title="Inject +1 Affirm">+1</div>
      <div class="inj-btn inj-zero" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); injectSignal('${r}', 0)" title="Inject 0 Tend">0</div>
      <div class="inj-btn inj-neg" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); injectSignal('${r}', -1)" title="Inject -1 Reject">-1</div>
    </div>
    ${i!=="datasource"?'<div class="flow-port flow-port-in"  style="left:-7px;  top:50%; margin-top:-6px;" title="Input"></div>':""}
    <div class="flow-port flow-port-out" style="right:-7px; top:50%; margin-top:-6px;" title="Output"></div>
    ${i==="artifact"?'<div class="inspector-resizer" style="width:10px; height:10px;"></div>':""}
  `,i!=="datasource"&&!a){const C=(u.startsWith("var"),u);C.startsWith("#")?d.style.backgroundColor=C+"33":d.style.backgroundColor=`color-mix(in srgb, ${C}, transparent 80%)`,d.style.borderColor=C}if(i!=="datasource"&&!a&&((W=($=f.find(C=>C.id===r))==null?void 0:$.props)!=null&&W.customColor)){const C=f.find(ce=>ce.id===r).props.customColor;d.style.borderColor=C,C.startsWith("#")?d.style.backgroundColor=C+"33":d.style.backgroundColor=`color-mix(in srgb, ${C}, transparent 80%)`,d.style.boxShadow=`0 0 10px ${C}33`;const de=d.querySelector(".fn-head");de&&(de.style.borderBottomColor=C)}d.onmousedown=C=>{C.target.closest("button")||C.target.classList.contains("flow-port")||(!C.shiftKey&&!k.has(r)?Re(r,!1):C.shiftKey?Re(r,!0):(M=r,D()),fe=!0,q=r,cn=C.clientX,pn=C.clientY,Me={},k.forEach(de=>{const ce=document.getElementById(de);ce&&(Me[de]={x:parseFloat(ce.style.left)||0,y:parseFloat(ce.style.top)||0})}),d.style.zIndex=1e3,C.preventDefault())},d.ondblclick=C=>{i==="macro"&&nn(r)},s.appendChild(d);const y={system_prompt:"",routing:"affirm→next",code:Ae[e]?Ae[e].code:t==="__arch__"?`fn main() -> trit {
    return affirm;
}`:"",input_schema:"signal: trit",output_schema:"signal: trit"};i==="external"&&(y.temperature=.5,y.max_trits=1024,y.provider="openai",y.api_key="",y.mapping="classification",y.template="Evaluate this signal: {{input}}"),f.push({id:r,name:e,path:t,type:i,x:n,y:o,props:y,isStub:a});const S=document.getElementById("canvas-hint");S&&(S.style.display="none"),lucide.createIcons(),L()}window.createFlowNode=F;let k=new Set,Me={},fe=!1,cn,pn,q=null;function Re(e,t=!1){t||(k.clear(),document.querySelectorAll(".flow-node").forEach(i=>i.classList.remove("selected","selected-multi"))),k.add(e),M=e;const n=document.getElementById(e);n&&n.classList.add(t&&k.size>1?"selected-multi":"selected"),t&&k.size>1&&k.forEach(i=>{const r=document.getElementById(i);r&&(r.classList.remove("selected","selected-multi"),r.classList.add(i===e?"selected":"selected-multi"))});const o=document.getElementById("groupBtn");o&&(o.style.display=k.size>1?"flex":"none"),D()}window.selectNode=Re;function ge(){k.clear(),M=null,H=null,document.querySelectorAll(".flow-node").forEach(n=>n.classList.remove("selected","selected-multi")),document.getElementById("wire-handle").classList.remove("active"),We();const e=document.getElementById("prop-header-label");e&&(e.textContent="Node Properties");const t=document.getElementById("groupBtn");t&&(t.style.display="none"),D()}window.clearSelection=ge;function un(){if(!(k.size===0&&!H)){if(H){yn(H);return}k.forEach(e=>Ne(e)),k.clear()}}window.deleteSelected=un;function Bo(){}window.syncMultiDragEnd=Bo;function Lo({errors:e,warnings:t}){const n=document.getElementById("validateBadge");if(!n)return;const o=e.length+t.length;if(o===0){n.style.display="none";return}n.style.display="flex",n.textContent=o>9?"!":o,n.style.background=e.length>0?"var(--red)":"var(--amber)"}window.updateValidateBadge=Lo;function $o(e,t){const n=f.find(r=>r.id===e);if(!n)return;const i={sensor:{name:"Sensor",code:`fn main() -> trit {
  let val = read_sensor();
  return val;
}`,icon:"radio"},gate:{name:"Trit Gate",code:`fn main() -> trit {
  let s = read_input();
  match s {
    affirm => { return affirm; }
    tend   => { return tend; }
    reject => { return reject; }
  }
}`,icon:"git-merge"},guardrail:{name:"Guardrail",code:`fn main() -> trit {
  let s = read_input();
  if s == reject { emit "VETO"; return reject; }
  return s;
}`,icon:"shield-check"},deliberator:{name:"Deliberator",code:`fn main() -> trit {
  // Weighted accumulation logic
  return truth();
}`,icon:"brain-circuit"}}[t];if(i){n.name=i.name,n.props.code=i.code;const r=document.getElementById(e);if(r){const a=r.querySelector(".fn-title");a&&(a.textContent=i.name);const s=r.querySelector(".fn-head i");s&&s.setAttribute("data-lucide",i.icon),lucide.createIcons()}D(),L(),E(`Morphed to ${i.name}`,"ok")}}window.morphNodeArchetype=$o;function mn(e){const t=f.find(o=>o.id===e);if(!t||t.type!=="artifact")return;t.isStub=!t.isStub;const n=document.getElementById(e);n&&(t.isStub?n.classList.add("artifact-stub"):n.classList.remove("artifact-stub"),n.querySelectorAll(".flow-port").forEach(i=>{i.style.top="50%",i.style.marginTop="-6px"}),A()),M===e&&D(),L()}window.collapseArtifactToStub=mn;function Ne(e){f=f.filter(o=>o.id!==e),x=x.filter(o=>o.fromId!==e&&o.toId!==e);const t=document.getElementById(e);t&&t.remove(),k.delete(e),M===e&&(M=k.size>0?[...k][k.size-1]:null,D());const n=document.getElementById("canvas-hint");n&&(n.style.display=f.length===0?"flex":"none"),A()}window.deleteNode=Ne;function Ao(e,t){if(!t)return;const n=new FileReader;n.onload=o=>{const i=o.target.result,r=f.find(a=>a.id===e);r&&(r.props.payload=i,L(),M===e&&D(),E(`Ingested ${t.name} (${i.length} bytes)`,"ok"))},n.readAsText(t)}window.handleDataSourceFileUpload=Ao;async function Mo(e){const t=f.find(o=>o.id===e),n=document.getElementById(`sql-input-${e}`).value;if(n){t.props.sql_query=n,E("Executing SQL Query...","ok");try{const i=await(await fetch("/api/data/query",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({sql:n})})).json();i.status==="ok"?(t.props.payload=JSON.stringify(i.results,null,2),L(),M===e&&D(),E(`Query successful. ${i.results.length} rows injected.`,"ok")):E(`SQL Error: ${i.error}`,"err")}catch(o){E(`Bridge Error: ${o.message}`,"err")}}}window.runSqlQuery=Mo;async function Ro(e){if(e){E("Generating Causal Artifact...","ok");try{const t=await fetch(`/api/data/artifact/${e}`);if(!t.ok)throw new Error("Artifact not found in session memory.");const n=await t.text(),o=new Blob([n],{type:"text/markdown"}),i=URL.createObjectURL(o),r=document.createElement("a");r.href=i,r.download=`causal-artifact-${e.substring(0,8)}.md`,document.body.appendChild(r),r.click(),document.body.removeChild(r),URL.revokeObjectURL(i)}catch(t){E(t.message,"err")}}}window.downloadCausalArtifact=Ro;const No={render(e,t,n,o,i){e.type==="external"?this.renderLLMSemantic(e,t,n,o,i):e.type==="artifact"?this.renderArtifactSemantic(e,t,n,o,i):e.type==="datasource"?this.renderDataSourceSemantic(e,t,n,o,i):this.renderSemantic(e,t,n,o,i)},renderArtifactSemantic(e,t,n,o,i){const r=e.props.payload||"(No payload data resolved yet)";t.innerHTML=`
      <div class="prop-section">
        <div class="prop-label-strict">Artifact Identity</div>
        <div style="font-size:13px; font-weight:800; color:var(--green); letter-spacing:0.5px;">${e.name}</div>
      </div>
      <div class="prop-section">
        <div class="prop-label-strict">Evolutionary State</div>
        <div style="font-size:11px; color:var(--text); font-weight:600; background:rgba(255,255,255,0.05); padding:6px; border-radius:4px; border:1px solid var(--border2);">
           ${e.isStub?"STUB (DATA CHECKPOINT)":"EXPANDED (RESOLUTION CARD)"}
        </div>
      </div>
      <div class="prop-section">
        <div class="prop-label-strict">Payload Memory</div>
        <div style="background:rgba(0,0,0,0.3); border:1px solid var(--border2); padding:10px; border-radius:4px; font-family:'JetBrains Mono',monospace; font-size:11px; white-space:pre-wrap; max-height:280px; overflow-y:auto; color:var(--text); line-height:1.5;">${r}</div>
      </div>
      <div style="margin-top:auto; padding-top:12px; border-top:1px solid var(--border2); display:flex; flex-direction:column; gap:8px;">
          <button class="btn-pill" style="flex:1; background:var(--bg2);" onclick="collapseArtifactToStub('${e.id}')">
            ${e.isStub?"Expand UI (Restore)":"Collapse to Checkpoint"}
          </button>
          <button class="btn-pill" style="flex:1; background:rgba(239,68,68,0.1); color:var(--red);" onclick="deleteNode('${e.id}')">Purge Data</button>
      </div>
    `,lucide.createIcons()},renderDataSourceSemantic(e,t,n,o,i){const r=e.props.data_type||"text",a=e.props.payload||"";t.innerHTML=`
      <div class="prop-section">
        <div class="prop-label-strict">Node Name</div>
        <input type="text" class="prop-input" value="${e.name}" oninput="updateNodeProp('name', this.value)">
      </div>
      <div class="prop-section">
        <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:4px;">
          <div class="prop-label-strict" style="margin-bottom:0;">Data Source Configuration</div>
          <button class="btn btn-primary" style="font-size:10px; padding:2px 8px; height:24px; background:var(--blue); border:none;" onclick="document.getElementById('datasource-file-input').click()">
            <i data-lucide="upload" style="width:12px; margin-right:4px;"></i> Upload File
          </button>
          <input type="file" id="datasource-file-input" style="display:none;" accept=".txt,.md,.csv,.json" onchange="handleDataSourceFileUpload('${e.id}', this.files[0])">
        </div>
        <div class="prop-label-strict">Data Type</div>
        <select class="prop-input" onchange="updateNodeProp('data_type', this.value)">
          <option value="text" ${r==="text"?"selected":""}>Raw Text</option>
          <option value="json" ${r==="json"?"selected":""}>JSON Array</option>
          <option value="csv" ${r==="csv"?"selected":""}>CSV Data</option>
          <option value="markdown" ${r==="markdown"?"selected":""}>Markdown</option>
        </select>
      </div>
      <div class="prop-section" style="flex:1; display:flex; flex-direction:column;">
        <div class="prop-label-strict">SQL Bridge (Persistent SQLite)</div>
        <textarea id="sql-input-${e.id}" class="prop-input" style="height:60px; font-family:monospace; margin-bottom:8px;" placeholder="SELECT * FROM table...">${e.props.sql_query||""}</textarea>
        <button class="btn btn-primary" style="height:28px; font-size:11px; margin-bottom:12px;" onclick="runSqlQuery('${e.id}')">Execute Query</button>

        <div class="prop-label-strict">Payload Injector (Semantic Data)</div>
        <textarea class="prop-input" style="flex:1; resize:vertical; min-height:200px; font-family:'JetBrains Mono', monospace; font-size:12px; line-height:1.4;" placeholder="Paste raw semantic payload here..." oninput="updateNodeProp('payload', this.value)">${a}</textarea>
        <div style="font-size:10px; color:var(--muted2); margin-top:4px;">Injected directly into downstream runtime buffer on Simulation.</div>
      </div>
    `,lucide.createIcons()},renderLegacy(e,t,n,o,i){const r=e.props.routing??"affirm→next",a=(e.props.code||"").replace(/</g,"&lt;").replace(/>/g,"&gt;"),s=(e.props.input_schema||"").replace(/</g,"&lt;").replace(/>/g,"&gt;"),d=(e.props.output_schema||"").replace(/</g,"&lt;").replace(/>/g,"&gt;");t.innerHTML=`
      <div class="prop-group">
        <label class="prop-label">Node Name</label>
        <input type="text" class="prop-input" value="${e.name}" oninput="updateNodeProp('name', this.value)">
      </div>
      <div class="prop-group">
        <label class="prop-label">System Instructions</label>
        <textarea class="prop-input" style="height:70px; resize:vertical;" oninput="updateNodeProp('system_prompt', this.value)">${e.props.system_prompt||""}</textarea>
      </div>
      <div class="prop-group">
        <label class="prop-label">Output Routing Rule</label>
        <select class="prop-input" style="width:100%" onchange="updateNodeProp('routing', this.value)">
          <option value="affirm→next" ${r==="affirm→next"?"selected":""}>affirm → pass to next</option>
          <option value="tend→next" ${r==="tend→next"?"selected":""}>tend → pass to next</option>
          <option value="all→next" ${r==="all→next"?"selected":""}>all trits → pass to next</option>
          <option value="affirm→branch" ${r==="affirm→branch"?"selected":""}>affirm → branch A, else B</option>
          <option value="reject→stop" ${r==="reject→stop"?"selected":""}>reject → halt flow</option>
        </select>
      </div>
      <div class="prop-group">
        <label class="prop-label" style="display:flex;justify-content:space-between;align-items:center;">
          <span>I/O Contract</span>
          <span style="font-size:9px;color:var(--muted2);font-weight:400;text-transform:none;">${n.length} in · ${o.length} out</span>
        </label>
        ${i}
        <div style="display:grid;grid-template-columns:1fr 1fr;gap:6px;">
          <div>
            <div style="font-size:9px;color:var(--green);font-weight:600;margin-bottom:3px;">▶ INPUT</div>
            <input class="prop-input" style="width:100%;font-size:10px;font-family:'JetBrains Mono',monospace;" value="${s}"
              placeholder="signal: trit" oninput="updateNodeProp('input_schema',this.value);updateNodeSchemaDisplay('${e.id}')">
          </div>
          <div>
            <div style="font-size:9px;color:var(--cyan);font-weight:600;margin-bottom:3px;">◀ OUTPUT</div>
            <input class="prop-input" style="width:100%;font-size:10px;font-family:'JetBrains Mono',monospace;" value="${d}"
              placeholder="result: trit" oninput="updateNodeProp('output_schema',this.value);updateNodeSchemaDisplay('${e.id}')">
          </div>
        </div>
      </div>
      <div class="prop-group">
        <label class="prop-label">Agent Code (.tern)</label>
        <textarea class="prop-input" style="height:100px; resize:vertical; font-family:'JetBrains Mono',monospace; font-size:10px;"
          oninput="updateNodeProp('code', this.value)">${a}</textarea>
      </div>
      <div style="display:flex;gap:6px;margin-top:4px;">
        <button class="btn btn-ghost" style="flex:1;font-size:11px;" onclick="openAgentInEditor()">Open in Editor</button>
        <button class="btn btn-ghost" style="flex:1;font-size:11px;color:var(--red);" onclick="deleteNode('${e.id}')">Remove</button>
      </div>
    `},renderSemantic(e,t,n,o,i){const r=e.props.system_prompt||e.props.intent||"",a=(e.props.input_schema||"").replace(/</g,"&lt;").replace(/>/g,"&gt;"),s=(e.props.output_schema||"").replace(/</g,"&lt;").replace(/>/g,"&gt;"),d=e.props.timeout??5e3,l=e.props.retries??3,c=e.props.execution_target??"local",p=e.props.customColor||"#38bdf8";t.innerHTML=`
      <div class="prop-section">
        <div class="prop-label-strict">Node Identity</div>
        <input type="text" class="prop-input-strict" style="width:100%" value="${e.name}" oninput="updateNodeProp('name', this.value)">
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Operational Intent</div>
        <textarea class="prop-input-strict" style="height:50px; resize:vertical; font-size:11px;" placeholder="Define high-level objective..." oninput="updateNodeProp('system_prompt', this.value)">${r}</textarea>
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Runtime Constraints</div>
        <div style="display:grid; grid-template-columns:1fr 1fr; gap:8px; margin-bottom:8px;">
          <div>
            <div class="prop-label-strict" style="font-size:9px;">Timeout (ms)</div>
            <input type="number" class="prop-input-strict" value="${d}" oninput="updateNodeProp('timeout', parseInt(this.value))">
          </div>
          <div>
            <div class="prop-label-strict" style="font-size:9px;">Max Retries</div>
            <input type="number" class="prop-input-strict" value="${l}" oninput="updateNodeProp('retries', parseInt(this.value))">
          </div>
        </div>
        <div class="prop-label-strict" style="font-size:9px;">Execution Target</div>
        <select class="prop-input-strict" onchange="updateNodeProp('execution_target', this.value)">
          <option value="local" ${c==="local"?"selected":""}>Local VM (Albert)</option>
          <option value="remote" ${c==="remote"?"selected":""}>Remote Proxy (API)</option>
        </select>
      </div>

      <div class="prop-section">
        <div class="prop-label-strict" style="display:flex;justify-content:space-between;">
          <span>I/O Contract</span>
          <span style="font-size:9px;color:var(--muted2);">${n.length} IN / ${o.length} OUT</span>
        </div>
        ${i}
        <div style="display:flex; flex-direction:column; gap:8px;">
          <input class="prop-input-strict" style="font-family:'JetBrains Mono',monospace; font-size:11px;" value="${a}" placeholder="Input Schema" oninput="updateNodeProp('input_schema',this.value);updateNodeSchemaDisplay('${e.id}')">
          <input class="prop-input-strict" style="font-family:'JetBrains Mono',monospace; font-size:11px;" value="${s}" placeholder="Output Schema" oninput="updateNodeProp('output_schema',this.value);updateNodeSchemaDisplay('${e.id}')">
        </div>
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Logic / Source</div>
        <textarea class="prop-input-strict" style="height:80px; resize:vertical; font-family:'JetBrains Mono',monospace; font-size:11px;"
          oninput="updateNodeProp('code', this.value)">${e.props.code||""}</textarea>
      </div>

      <div class="prop-section" style="margin-top:auto; padding-top:12px; border-top:1px solid var(--border2);">
        <div class="prop-label-strict">Morph Archetype</div>
        <select class="prop-input-strict" style="font-size:11px; font-weight:700; color:var(--cyan); margin-bottom:12px;" onchange="morphNodeArchetype('${e.id}', this.value)">
          <option value="">-- Generic Agent --</option>
          <option value="sensor">SENSOR (Input Capture)</option>
          <option value="gate">GATE (Ternary Logic)</option>
          <option value="guardrail">GUARDRAIL (Veto Logic)</option>
          <option value="deliberator">DELIBERATOR (Weighted)</option>
        </select>

        <div style="display:flex; flex-direction:column; gap:4px; margin-bottom:12px;">
          <div style="color:white; font-weight:bold; font-size:10px; text-transform:uppercase; letter-spacing:0.05em;">Custom Color</div>
          <div style="display:flex; align-items:center; gap:6px;">
            <input type="color" class="prop-input-strict" style="width:34px; padding:0; border:1px solid var(--border2); height:24px; cursor:pointer; background:none; border-radius:4px;" value="${p}" oninput="updateNodeColor('${e.id}', this.value); updatePropertyPanel()" title="Custom Color">
            <code style="font-size:11px; color:var(--text); font-family:'JetBrains Mono',monospace; opacity:0.8; letter-spacing:0.5px;">${p.toUpperCase()}</code>
          </div>
        </div>
        <div style="display:flex; gap:6px;">
          <button class="btn-pill" style="flex:1; background:var(--bg2);" onclick="openAgentInEditor()">Editor</button>
          <button class="btn-pill" style="flex:1; background:rgba(239,68,68,0.1); color:var(--red);" onclick="deleteNode('${e.id}')">Remove</button>
        </div>
      </div>
    `,lucide.createIcons()},renderLLMSemantic(e,t,n,o,i){const r=e.props.system_prompt||"",a=e.props.protocol||"openai",s=e.props.model_id||"",d=e.props.base_url||"",l=e.props.temperature??.5,c=e.props.max_trits??1024,p=e.props.timeout??1e4,m=e.props.retries??2,u=e.props.customColor||"#f59e0b",g=Je(),v=e.props.api_key||g[a]||"";v&&!e.props.api_key&&(e.props.api_key=v);const w=a==="openai"||a==="webhook";t.innerHTML=`
      <div class="prop-section">
        <div class="prop-label-strict">Bridge Identity</div>
        <input type="text" class="prop-input-strict" style="width:100%" value="${e.name}" oninput="updateNodeProp('name', this.value)">
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Intelligence Strategy</div>
        <textarea class="prop-input-strict" style="height:50px; resize:vertical; font-size:11px;" placeholder="System prompt for the bridge..." oninput="updateNodeProp('system_prompt', this.value)">${r}</textarea>
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Runtime Constraints</div>
        <div style="display:grid; grid-template-columns:1fr 1fr; gap:8px;">
          <div>
            <div class="prop-label-strict" style="font-size:9px;">Timeout (ms)</div>
            <input type="number" class="prop-input-strict" value="${p}" oninput="updateNodeProp('timeout', parseInt(this.value))">
          </div>
          <div>
            <div class="prop-label-strict" style="font-size:9px;">Max Retries</div>
            <input type="number" class="prop-input-strict" value="${m}" oninput="updateNodeProp('retries', parseInt(this.value))">
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
          <select class="prop-input-strict" onchange="updateBridgeProtocol('${e.id}', this.value)">
            <option value="openai" ${a==="openai"?"selected":""}>OpenAI-Compatible REST</option>
            <option value="anthropic" ${a==="anthropic"?"selected":""}>Anthropic Native</option>
            <option value="google" ${a==="google"?"selected":""}>Google Native</option>
            <option value="webhook" ${a==="webhook"?"selected":""}>Custom Webhook</option>
            <option value="mcp" ${a==="mcp"?"selected":""}>MCP (Model Context Protocol)</option>
          </select>
        </div>

        <div style="margin-bottom:8px;">
          <div class="prop-label-strict" style="font-size:9px; color:var(--muted2);">Target Model ID</div>
          <input type="text" class="prop-input-strict" value="${s}" placeholder="e.g. grok-3 or claude-3-7-sonnet" oninput="updateNodeProp('model_id', this.value)">
        </div>

        ${w?`
        <div style="margin-bottom:8px;">
          <div class="prop-label-strict" style="font-size:9px; color:var(--muted2);">Base URL</div>
          <input type="text" class="prop-input-strict" value="${d}" placeholder="https://api.openai.com/v1" oninput="updateNodeProp('base_url', this.value)">
        </div>
        `:""}

        <div style="margin-bottom:8px;">
          <div class="prop-label-strict" style="font-size:9px; color:var(--muted2);">Provider API Key</div>
          <input type="password" class="prop-input-strict" value="${v}" placeholder="Linked to ${a} vault" oninput="updateNodeProp('api_key', this.value)">
        </div>
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Hyperparameters</div>
        <div style="display:grid; grid-template-columns:1fr 1fr; gap:8px;">
          <div>
            <div style="font-size:9px; color:var(--muted2); margin-bottom:2px;">TEMP: ${l}</div>
            <input type="range" min="0" max="1" step="0.1" value="${l}" style="width:100%" oninput="updateNodeProp('temperature', parseFloat(this.value)); updatePropertyPanel()">
          </div>
          <div>
            <div style="font-size:9px; color:var(--muted2); margin-bottom:2px;">TOKENS: ${c}</div>
            <input type="range" min="128" max="4096" step="128" value="${c}" style="width:100%" oninput="updateNodeProp('max_trits', parseInt(this.value)); updatePropertyPanel()">
          </div>
        </div>
      </div>

      <div style="margin-top:auto; padding-top:12px; border-top:1px solid var(--border2); display:flex; flex-direction:column; gap:12px;">
        <div style="display:flex; flex-direction:column; gap:4px;">
          <div style="color:white; font-weight:bold; font-size:10px; text-transform:uppercase; letter-spacing:0.05em;">Custom Color</div>
          <div style="display:flex; align-items:center; gap:6px;">
            <input type="color" class="prop-input-strict" style="width:34px; padding:0; border:1px solid var(--border2); height:24px; cursor:pointer; background:none; border-radius:4px;" value="${u}" oninput="updateNodeColor('${e.id}', this.value); updatePropertyPanel()" title="Custom Color">
            <code style="font-size:11px; color:var(--text); font-family:'JetBrains Mono',monospace; opacity:0.8; letter-spacing:0.5px;">${u.toUpperCase()}</code>
          </div>
        </div>
        <div style="display:flex; gap:6px;">
          <button class="btn-pill" style="flex:1; background:var(--bg2);" onclick="openAgentInEditor()">Editor</button>
          <button class="btn-pill" style="flex:1; background:rgba(239,68,68,0.1); color:var(--red);" onclick="deleteNode('${e.id}')">Remove</button>
        </div>
      </div>
    `,lucide.createIcons()},updateInlineSummary(e,t){const n=document.querySelector(`#${e.id} .fn-title`);if(n&&!n.dataset.semantic){n.dataset.semantic="true";const i=document.createElement("div");i.className="semantic-desc",i.style="font-size:9px; color:var(--muted2); font-weight:400; max-height:24px; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; margin-top:2px;",n.parentNode.insertBefore(i,n.nextSibling)}const o=document.querySelector(`#${e.id} .semantic-desc`);o&&(o.textContent=t?`→ ${t}`:e.type==="external"?"→ Waiting for prompt...":"→ (No intent defined)")}},Po={render(e,t,n,o){this.renderSemantic(e,t,n,o)},renderLegacy(e,t,n,o){const i=e.condition||"all",r=e.transform||"pass",a=e.label||"";t.innerHTML=`
      <div style="font-size:10px;color:var(--muted2);margin-bottom:4px;">
        ${n?n.name:"?"} → ${o?o.name:"?"}
      </div>

      <div class="prop-group">
        <label class="prop-label">Pass condition</label>
        <select class="prop-input" style="width:100%" onchange="updateWireProp('condition', this.value)">
          <option value="all"    ${i==="all"?"selected":""}>All trits (pass everything)</option>
          <option value="affirm" ${i==="affirm"?"selected":""}>affirm only (+1)</option>
          <option value="tend"   ${i==="tend"?"selected":""}>tend only (0)</option>
          <option value="reject" ${i==="reject"?"selected":""}>reject only (-1)</option>
          <option value="!reject"${i==="!reject"?"selected":""}>affirm or tend (not reject)</option>
          <option value="!tend"  ${i==="!tend"?"selected":""}>affirm or reject (decisive)</option>
        </select>
      </div>

      <div class="prop-group">
        <label class="prop-label">On condition fail</label>
        <select class="prop-input" style="width:100%" onchange="updateWireProp('transform', this.value)">
          <option value="pass"  ${r==="pass"?"selected":""}>Pass anyway (ignore condition)</option>
          <option value="block" ${r==="block"?"selected":""}>Block — drop signal</option>
          <option value="flip"  ${r==="flip"?"selected":""}>Flip to reject</option>
          <option value="hold"  ${r==="hold"?"selected":""}>Force to tend (hold)</option>
        </select>
      </div>

      <div class="prop-group">
        <label class="prop-label">Edge label</label>
        <input type="text" class="prop-input" value="${a}" placeholder="e.g. conf > 0.7"
          oninput="updateWireProp('label', this.value)">
      </div>

      <div class="prop-group">
        <label class="prop-label">Priority weight</label>
        <input type="range" min="1" max="10" step="1" value="${e.priority||5}" style="width:100%"
          oninput="updateWireProp('priority', parseInt(this.value)); this.nextElementSibling.textContent=this.value">
        <span style="font-size:10px;color:var(--cyan)">${e.priority||5}</span>
      </div>

      <div class="prop-group" style="display:flex; align-items:center; gap:8px;">
        <input type="checkbox" id="wire-feedback" ${e.isFeedback?"checked":""} 
          onchange="updateWireProp('isFeedback', this.checked)">
        <label for="wire-feedback" class="prop-label" style="margin:0; cursor:pointer;">Feedback loop (Bypass cycle check)</label>
      </div>

      <div style="margin-top:12px;">
        <button class="btn btn-ghost" style="width:100%;font-size:11px;color:var(--red)" onclick="deleteWire('${e.id}')">Remove Edge</button>
      </div>
    `},renderSemantic(e,t,n,o){const i=e.condition||"all",r=e.transform||"pass",a=e.priority||5,s=e.weight||1,d=e.latency||0,l=e.customColor||"#94a3b8";t.innerHTML=`
      <div style="font-size:10px; color:var(--muted2); margin-bottom:8px; font-weight:600; display:flex; align-items:center; gap:4px;">
        <span style="color:var(--text)">${n?n.name:"?"}</span> 
        <i data-lucide="arrow-right" style="width:10px; height:10px; color:var(--cyan)"></i> 
        <span style="color:var(--text)">${o?o.name:"?"}</span>
        <div style="flex:1"></div>
      </div>

      <!-- Condition Rail -->
      <div class="prop-group" style="margin-bottom:8px;">
        <div class="prop-label-strict" style="font-size:10px; margin-bottom:4px;">Activation Logic</div>
        <div style="display:flex; align-items:stretch; height:24px; background:var(--bg2); border:1px solid var(--border2); border-radius:4px; overflow:hidden;">
          <button title="value == +1" class="rail-btn ${i==="affirm"?"active":""}" onclick="updateWireProp('condition','affirm');updateWireProp('label','+1 only');">+1</button>
          <button title="value == 0"  class="rail-btn ${i==="tend"?"active":""}"   onclick="updateWireProp('condition','tend');updateWireProp('label','0 only');">0</button>
          <button title="value == -1" class="rail-btn ${i==="reject"?"active":""}" onclick="updateWireProp('condition','reject');updateWireProp('label','-1 only');">-1</button>
          <button title="value != -1" class="rail-btn ${i==="!reject"?"active":""}" onclick="updateWireProp('condition','!reject');updateWireProp('label','+1 or 0');">!= -1</button>
          <button title="All signals" class="rail-btn ${i==="all"?"active":""}"    onclick="updateWireProp('condition','all');updateWireProp('label','All signals');">ALL</button>
        </div>
      </div>

      <div style="display:grid; grid-template-columns: 1fr 1fr; gap:8px; margin-bottom:8px;">
        <div class="prop-group" style="margin:0;">
          <div class="prop-label-strict" style="font-size:10px; margin-bottom:4px;">On Fail</div>
          <div style="display:flex; flex-direction:column; gap:2px; font-size:9px; line-height:1.2;">
            <label style="display:flex; align-items:center; gap:4px; cursor:pointer;"><input type="radio" name="wfail" value="block" ${r==="block"||r==="pass"?"checked":""} onchange="updateWireProp('transform','block')"> Drop</label>
            <label style="display:flex; align-items:center; gap:4px; cursor:pointer;"><input type="radio" name="wfail" value="flip" ${r==="flip"?"checked":""} onchange="updateWireProp('transform','flip')"> Fallback</label>
            <label style="display:flex; align-items:center; gap:4px; cursor:pointer;"><input type="radio" name="wfail" value="hold" ${r==="hold"?"checked":""} onchange="updateWireProp('transform','hold')"> Hold (0)</label>
          </div>
        </div>
        <div class="prop-group" style="margin:0;">
          <div class="prop-label-strict" style="font-size:10px; margin-bottom:4px;">Temporal Dynamics</div>
          <div style="display:flex; flex-direction:column; gap:4px;">
            <div>
               <div style="font-size:8px; color:var(--muted2)">Signal Weight</div>
               <input type="number" step="0.1" class="prop-input-strict" style="height:20px; font-size:9px; padding:2px 4px;" value="${s}" oninput="updateWireProp('weight', parseFloat(this.value))">
            </div>
            <div>
               <div style="font-size:8px; color:var(--muted2)">Latency (ms)</div>
               <input type="number" class="prop-input-strict" style="height:20px; font-size:9px; padding:2px 4px;" value="${d}" oninput="updateWireProp('latency', parseInt(this.value))">
            </div>
          </div>
        </div>
      </div>

      <div class="prop-group" style="margin-bottom:8px;">
        <div class="prop-label-strict" style="font-size:10px; margin-bottom:4px;">Priority</div>
        <div style="display:flex; height:24px; border-radius:4px; overflow:hidden; border:1px solid var(--border2); background:var(--bg2);">
          <button class="rail-btn ${a<=2?"active":""}" style="flex:1; font-size:9px;" onclick="updateWireProp('priority', 2); updateEdgePanel();">Low</button>
          <button class="rail-btn ${a>2&&a<10?"active":""}" style="flex:1; font-size:9px;" onclick="updateWireProp('priority', 5); updateEdgePanel();">Norm</button>
          <button class="rail-btn ${a>=10?"active":""}" style="flex:1; font-size:9px;" onclick="updateWireProp('priority', 10); updateEdgePanel();">High</button>
        </div>
      </div>

      <div class="prop-group" style="margin-bottom:8px;">
        <div class="prop-label-strict" style="font-size:10px; margin-bottom:4px;">Custom Label</div>
        <input type="text" class="prop-input-strict" style="font-size:11px; height:24px;" value="${e.label||""}" placeholder="e.g. data_stream"
          oninput="updateWireProp('label', this.value)">
      </div>

      <div class="prop-group" style="display:flex; align-items:center; gap:6px; margin-bottom:8px;">
        <input type="checkbox" id="wire-feedback-sem" style="width:12px; height:12px;" ${e.isFeedback?"checked":""} onchange="updateWireProp('isFeedback', this.checked)">
        <label for="wire-feedback-sem" class="prop-label-strict" style="margin:0; font-size:10px; cursor:pointer;">Feedback Loop</label>
      </div>

      <div class="prop-group" style="display:flex; align-items:center; justify-content:space-between; background:var(--bg2); padding:6px; border-radius:4px; border:1px solid var(--border2); margin-top:auto;">
        <div class="prop-label-strict" style="font-size:10px; margin:0;">Custom Wire Color</div>
        <div style="display:flex; align-items:center; gap:8px;">
          <code style="font-size:10px; color:var(--muted2);">${l}</code>
          <input type="color" class="prop-input-strict" style="width:24px; padding:0; border:none; height:20px; cursor:pointer; background:none;" value="${l}" oninput="updateWireColor('${e.id}', this.value); updatePropertyPanel()" title="Custom Wire Color">
        </div>
      </div>

      <div style="padding-top:8px;">
        <button class="btn btn-ghost" style="width:100%; height:28px; font-size:11px; color:var(--red); border:1px solid rgba(239, 68, 68, 0.1);" onclick="deleteWire('${e.id}')">Remove Edge</button>
      </div>
    `,lucide.createIcons()}};function Do(){const e=document.getElementById("helpPopover"),t=document.getElementById("helpPopoverTitle"),n=document.getElementById("helpPopoverContent"),o=document.getElementById("prop-header-label");if(!e||!t||!n||!o)return;o.textContent.toLowerCase().includes("edge")?(t.innerHTML='<i data-lucide="git-branch" style="width:16px; color:var(--amber)"></i> What is an Edge?',n.innerHTML=`
      <p style="margin-bottom:12px;">Think of an Edge as a <b>smart pipe</b> connecting your workers. It does not just carry information; it acts like a bouncer at a door.</p>
      <div style="margin-bottom:12px;">
        <b style="color:var(--green); display:block; margin-bottom:4px;">Activation Logic</b>
        You tell the bouncer who gets through. Setting it to '+1' means only absolute 'Yes' answers pass. Setting it to '!= -1' means 'Yes' and 'I am not sure' can pass, but a hard 'No' gets blocked.
      </div>
      <div>
        <b style="color:var(--red); display:block; margin-bottom:4px;">On Fail</b>
        If a signal is blocked, what happens? 'Drop' throws it in the trash. 'Fallback' sends it to a backup plan. 'Hold (0)' turns it into a neutral 'I don't know' state so the system keeps moving without crashing.
      </div>
    `):(t.innerHTML='<i data-lucide="component" style="width:16px; color:var(--cyan)"></i> What is a Node?',n.innerHTML=`
      <p style="margin-bottom:12px;">Think of a Node as a <b>mini-worker</b> or a tiny brain in your network. It receives a task, thinks about it, and spits out a ternary answer: Yes (+1), I am not sure (0), or No (-1).</p>
      <p>Here you give this worker its instructions. You tell it where to do its thinking (like your local Albert VM) and give it safety rules, like 'do not take longer than 5 seconds' so it does not freeze your whole system.</p>
    `),e.style.display="block",window.lucide&&lucide.createIcons()}window.showHelpCard=Do;function zo(){const e=document.getElementById("helpPopover");e&&(e.style.display="none")}window.closeHelpCard=zo;function D(){const e=document.getElementById("prop-body"),t=document.getElementById("prop-header-label"),n=document.getElementById("prop-help-icon");if(!M){t&&(t.textContent="Node Properties"),n&&(n.style.display="flex"),e.innerHTML=`
      <div style="color:var(--muted); font-size:12px; text-align:center; margin-top:40px; padding:0 12px; line-height:1.8;">
        Select a node to configure<br>
        <span style="font-size:10px; color:var(--muted2);">Drag output port → input port to wire agents</span>
      </div>`;return}const o=f.find(s=>s.id===M);if(!o)return;t&&(t.textContent=o.type==="macro"?"MACRO PROPERTIES":"NODE PROPERTIES"),n&&(n.style.display="flex");const i=x.filter(s=>s.toId===M),r=x.filter(s=>s.fromId===M),a=i.some(s=>{const d=f.find(l=>l.id===s.fromId);return d&&d.props.output_schema&&o.props.input_schema&&d.props.output_schema!==o.props.input_schema})?'<div style="font-size:10px;color:var(--amber);padding:6px 0;">⚠ Schema mismatch on incoming wire</div>':"";No.render(o,e,i,r,a)}function Oo(e,t){const n=f.find(i=>i.id===e);if(!n)return;n.props.customColor=t;const o=document.getElementById(e);if(o){o.style.borderColor=t,o.style.boxShadow=`0 0 10px ${t}33`;const i=o.querySelector(".fn-head");i&&(i.style.borderBottomColor=t)}L()}window.updateNodeColor=Oo;function jo(e,t){const n=x.find(i=>i.id===e);if(!n)return;n.customColor=t;const o=document.querySelector(`path[id="${e}"]`);o&&(o.style.stroke=t),L()}window.updateWireColor=jo;window.updatePropertyPanel=D;function Fo(){const e=f.find(o=>o.id===M);if(!e)return;const t=`flow/${e.name.replace(/\s+/g,"_")}.tern`,n=e.props.code||`fn main() -> trit {
    // ${e.name}
    return affirm;
}`;B[t]=n,V(t,n),G("editor")}window.openAgentInEditor=Fo;function wt(e){const t=f.find(a=>a.id===e);if(!t)return;const n=document.getElementById(e);if(!n)return;let o=n.querySelector(".fn-schema");o||(o=document.createElement("div"),o.className="fn-schema",n.querySelector(".fn-body").appendChild(o));const i=t.props.input_schema?`<span style="color:var(--green)">▶ ${t.props.input_schema}</span>`:"",r=t.props.output_schema?`<span style="color:var(--cyan)">◀ ${t.props.output_schema}</span>`:"";o.innerHTML=[i,r].filter(Boolean).join("<br>")}window.updateNodeSchemaDisplay=wt;function Ho(e,t){const n=f.find(o=>o.id===M);if(n){if(e==="name"){n.name=t;const o=document.querySelector(`#${n.id} .fn-title`);o&&(o.textContent=t)}else n.props[e]=t;if(n.type==="external"&&e==="api_key"){const o=n.props.protocol||"openai";At(o,t)}L()}}window.updateNodeProp=Ho;function Wo(e,t){const n=f.find(i=>i.id===e);if(!n)return;n.props.protocol=t;const o=Je();n.props.api_key=o[t]||"",D(),L()}window.updateBridgeProtocol=Wo;function qo(){const e="bridge_"+Date.now(),t=te((Math.random()-.5)*100,(Math.random()-.5)*80);F("LLM Bridge","external",t.x,t.y,"external",e)}window.addExternalBridge=qo;function Go(){const e="gate_"+Date.now(),t=te((Math.random()-.5)*100,(Math.random()-.5)*80);F("Consensus Gate","gate",t.x,t.y,"gate",e)}window.addTernaryGate=Go;function Uo(){const e="data_"+Date.now(),t=te((Math.random()-.5)*100,(Math.random()-.5)*80);F("Data Source","source",t.x,t.y,"datasource",e)}window.addDataSource=Uo;function Ko(e){document.querySelectorAll(".lib-tab").forEach(n=>n.classList.remove("active"));const t=document.getElementById("libtab-"+e);t&&t.classList.add("active"),document.getElementById("lib-panel-agents").style.display=e==="agents"?"flex":"none",document.getElementById("lib-panel-archetypes").style.display=e==="archetypes"?"flex":"none",e==="archetypes"&&bt()}window.switchLibTab=Ko;const fn=[{id:"moe_13_flagship",name:"MoE-13 Flagship",desc:"The ultimate AI brain. It uses 13 specialized experts working together to solve incredibly complex problems.",icon:"layers",color:"var(--cyan)",nodes:[{name:"Orchestrator",type:"agent",dx:440,dy:160},{name:"Expert_01",type:"agent",dx:40,dy:20},{name:"Expert_02",type:"agent",dx:40,dy:90},{name:"Expert_03",type:"agent",dx:40,dy:160},{name:"Expert_04",type:"agent",dx:40,dy:230},{name:"Expert_05",type:"agent",dx:40,dy:300},{name:"Consensus",type:"gate",dx:240,dy:160},{name:"Decision",type:"gate",dx:640,dy:160},{name:"Feedback",type:"agent",dx:440,dy:300},{name:"Context Source",type:"datasource",dx:-160,dy:160,props:{payload:`# TIS GROUNDING
- Mode: MoE-13
- Logic: Balanced Ternary`,data_type:"markdown"}},{name:"Expert Coordinator",type:"external",dx:40,dy:400}],wires:[[1,6],[2,6],[3,6],[4,6],[5,6],[6,0],[0,7],[7,8],[8,0],[9,10],[10,6]],feedbackWires:[8],edgeConds:["all","all","all","all","all","affirm","affirm","tend","all","all","all"]},{id:"consensus",name:"Consensus Pipeline",desc:"A team of voters. Multiple agents look at the same data and use majority rule to make a safe decision.",icon:"git-merge",color:"var(--green)",nodes:[{name:"Sensor A",type:"agent",dx:60,dy:60},{name:"Sensor B",type:"agent",dx:60,dy:180},{name:"Sensor C",type:"agent",dx:60,dy:300},{name:"Consensus Gate",type:"gate",dx:320,dy:180},{name:"Actuator",type:"agent",dx:540,dy:180},{name:"Ref Data",type:"datasource",dx:60,dy:420,props:{payload:`# CONSENSUS REF
- Majority: 2/3
- Veto: -1`,data_type:"markdown"}},{name:"Audit Bridge",type:"external",dx:320,dy:420}],wires:[[0,3],[1,3],[2,3],[3,4],[5,6],[6,3]],edgeConds:["affirm","all","reject","affirm","all","all"]},{id:"guardrail",name:"Guardrail Chain",desc:"A high-security pipeline. It checks data before and after the AI processes it to ensure absolute safety.",icon:"shield-check",color:"var(--amber)",nodes:[{name:"Input",type:"agent",dx:40,dy:160},{name:"Safety Gate",type:"gate",dx:240,dy:160},{name:"LLM Bridge",type:"external",dx:440,dy:80},{name:"Output Guard",type:"gate",dx:440,dy:240},{name:"Output",type:"agent",dx:640,dy:160},{name:"Policy Source",type:"datasource",dx:440,dy:-40,props:{payload:`# SAFETY POLICY
- No PII leak
- Respect Veto`,data_type:"markdown"}}],wires:[[0,1],[1,2],[1,3],[2,4],[3,4],[5,2]],edgeConds:["all","affirm","reject","affirm","affirm","all"]},{id:"filter_rank",name:"Filter → Rank → Decide",desc:"The sorting machine. It quickly filters out bad options and ranks the good ones to find the winner.",icon:"funnel",color:"var(--cyan)",nodes:[{name:"Raw Signal",type:"agent",dx:40,dy:160},{name:"Filter",type:"gate",dx:240,dy:160},{name:"Ranker",type:"agent",dx:440,dy:160},{name:"Decision",type:"gate",dx:640,dy:160}],wires:[[0,1],[1,2],[2,3]],edgeConds:["all","affirm","all"]},{id:"debate",name:"Multi-Agent Debate",desc:"A virtual courtroom. One agent proposes an idea, another attacks it, and a judge decides the winner.",icon:"message-square",color:"var(--muted)",nodes:[{name:"Proposer",type:"agent",dx:60,dy:80},{name:"Challenger",type:"agent",dx:60,dy:260},{name:"Arbiter",type:"gate",dx:300,dy:170},{name:"Accept",type:"agent",dx:520,dy:80},{name:"Reject",type:"agent",dx:520,dy:260}],wires:[[0,2],[1,2],[2,3],[2,4]],edgeConds:["all","all","affirm","reject"]},{id:"sensor_gate",name:"Sensor → Gate → Actuator",desc:"The simplest AI workflow. It reads data, makes one decision, and takes an action.",icon:"cpu",color:"var(--blue)",nodes:[{name:"Sensor",type:"agent",dx:60,dy:160},{name:"Gate",type:"gate",dx:280,dy:160},{name:"Actuator",type:"agent",dx:500,dy:80},{name:"Fallback",type:"agent",dx:500,dy:240}],wires:[[0,1],[1,2],[1,3]],edgeConds:["all","affirm","reject"]},{id:"kmu_process_opt",name:"KMU: Process Optimization Loop",desc:"An automated manager. It constantly analyzes a business process and loops back to improve it.",icon:"refresh-cw",color:"var(--amber)",nodes:[{name:"IST Capture",type:"agent",dx:40,dy:80},{name:"Analysis Gate",type:"gate",dx:240,dy:160},{name:"SOLL Design",type:"agent",dx:440,dy:80},{name:"Test Run",type:"gate",dx:640,dy:160},{name:"Tracking",type:"agent",dx:840,dy:160},{name:"Logger",type:"agent",dx:240,dy:280}],wires:[[0,1],[1,2],[2,3],[3,4],[1,5],[3,1]],feedbackWires:[5],edgeConds:["all","affirm","all","affirm","reject","reject"]},{id:"kmu_supplier_score",name:"KMU: Supplier Scoring",desc:"The purchasing agent. It automatically grades suppliers based on price, quality, and delivery speed.",icon:"truck",color:"var(--blue)",nodes:[{name:"Input Validator",type:"agent",dx:40,dy:160},{name:"Price Check",type:"agent",dx:240,dy:40},{name:"Quality Check",type:"agent",dx:240,dy:120},{name:"Delivery Check",type:"agent",dx:240,dy:200},{name:"Consensus",type:"gate",dx:440,dy:160},{name:"Decision Gate",type:"gate",dx:640,dy:160},{name:"Logger",type:"agent",dx:640,dy:280}],wires:[[0,1],[0,2],[0,3],[1,4],[2,4],[3,4],[4,5],[4,6]],edgeConds:["all","all","all","all","all","all","affirm","tend"]},{id:"kmu_customer_qual",name:"KMU: Customer Qualification",desc:"The sales assistant. It checks incoming leads to see if they match your ideal customer profile.",icon:"users",color:"var(--cyan)",nodes:[{name:"Input Validator",type:"agent",dx:40,dy:160},{name:"Budget Check",type:"agent",dx:240,dy:60},{name:"Industry Fit",type:"agent",dx:240,dy:160},{name:"Engagement",type:"agent",dx:240,dy:260},{name:"Aggregation",type:"gate",dx:440,dy:160},{name:"Routing Gate",type:"gate",dx:640,dy:160},{name:"Sales / Nurture",type:"agent",dx:840,dy:160}],wires:[[0,1],[0,2],[0,3],[1,4],[2,4],[3,4],[4,5],[5,6]],edgeConds:["all","all","all","all","all","all","affirm","!reject"]},{id:"kmu_invoice_fraud",name:"KMU: Invoice Fraud Detection",desc:"The accountant. It scans incoming invoices for weird numbers or mismatched vendor details.",icon:"file-warning",color:"var(--red)",nodes:[{name:"Input Validation",type:"agent",dx:40,dy:160},{name:"Amount Deviation",type:"agent",dx:240,dy:60},{name:"Vendor Match",type:"agent",dx:240,dy:160},{name:"Pattern Detect",type:"agent",dx:240,dy:260},{name:"Consensus",type:"gate",dx:440,dy:160},{name:"Decision Gate",type:"gate",dx:640,dy:160},{name:"Alert Logger",type:"agent",dx:640,dy:280}],wires:[[0,1],[0,2],[0,3],[1,4],[2,4],[3,4],[4,5],[5,6]],edgeConds:["all","all","all","all","all","all","affirm","affirm"]},{id:"kmu_hiring_decision",name:"KMU: Hiring Decision System",desc:"The HR screener. It reads resumes and scores candidates to save you time.",icon:"briefcase",color:"var(--green)",nodes:[{name:"Input Validation",type:"agent",dx:40,dy:160},{name:"Experience Check",type:"agent",dx:240,dy:40},{name:"Skills Match",type:"agent",dx:240,dy:120},{name:"CV Analysis",type:"external",dx:240,dy:200},{name:"Test Score",type:"agent",dx:240,dy:280},{name:"Consensus",type:"gate",dx:440,dy:160},{name:"Decision Gate",type:"gate",dx:640,dy:160}],wires:[[0,1],[0,2],[0,3],[0,4],[1,5],[2,5],[3,5],[4,5],[5,6]],edgeConds:["all","all","all","all","all","all","all","all","affirm"]},{id:"industry_sme_pipeline",name:"SME: Precision Data Pipeline",desc:"A quick data processor. It grabs numbers, checks if they cross a line, and spits out a report.",icon:"rows",color:"#4ade80",nodes:[{name:"Raw_Input",type:"agent",dx:40,dy:160},{name:"Float_Threshold",type:"gate",dx:240,dy:160},{name:"Logger",type:"agent",dx:440,dy:60},{name:"Report_Emit",type:"agent",dx:440,dy:260}],wires:[[0,1],[1,2],[1,3]],edgeConds:["all","all","affirm"]},{id:"industry_enterprise_risk",name:"Enterprise: Risk Assessment Swarm",desc:"A corporate risk team. It sends data through multiple legal and financial checks before moving forward.",icon:"eye",color:"#000000",nodes:[{name:"Legal_Audit",type:"agent",dx:40,dy:60},{name:"Finance_Vetting",type:"agent",dx:40,dy:260},{name:"Majority_5",type:"gate",dx:280,dy:160},{name:"Supervisor",type:"agent",dx:520,dy:160},{name:"Gatekeeper",type:"gate",dx:760,dy:160},{name:"Final_Verdict",type:"agent",dx:980,dy:160}],wires:[[0,2],[1,2],[2,3],[3,4],[4,5]],edgeConds:["all","all","affirm","affirm","affirm"]},{id:"recursive_refiner",name:"Recursive Multi-Stage Refiner",desc:"The perfectionist. It loops a task over and over until the AI is 100 percent sure it got it right.",icon:"refresh-ccw",color:"var(--amber)",nodes:[{name:"Raw Entry",type:"agent",dx:40,dy:160},{name:"Stage 1: Vetting",type:"agent",dx:240,dy:60},{name:"Stage 2: Audit",type:"agent",dx:240,dy:260},{name:"Consensus Hub",type:"gate",dx:480,dy:160},{name:"Refinement Loop",type:"agent",dx:480,dy:340},{name:"Output Guard",type:"gate",dx:720,dy:160},{name:"Final Emission",type:"agent",dx:960,dy:160}],wires:[[0,1],[0,2],[1,3],[2,3],[3,4],[4,3],[3,5],[5,6]],feedbackWires:[5],edgeConds:["all","all","all","all","tend","all","affirm","affirm"]},{id:"industry_iot_grid",name:"Industrial: IoT Sensor Grid",desc:"The factory monitor. It watches live sensors and hits the emergency stop if things look dangerous.",icon:"bell",color:"#ef4444",nodes:[{name:"Mesh_Node_A",type:"agent",dx:40,dy:60},{name:"Mesh_Node_B",type:"agent",dx:40,dy:260},{name:"Range_Validator",type:"gate",dx:280,dy:160},{name:"Watchdog",type:"agent",dx:520,dy:160},{name:"SCADA_Emit",type:"agent",dx:760,dy:60},{name:"Emergency_Stop",type:"agent",dx:760,dy:260}],wires:[[0,2],[1,2],[2,3],[3,4],[3,5]],edgeConds:["all","all","all","affirm","reject"]},{id:"local_rag_pipeline",name:"Local RAG Pipeline",desc:"A private researcher. It reads your local files to answer questions without sending data to the cloud.",icon:"database",color:"var(--cyan)",nodes:[{name:"Input_Signal",type:"agent",dx:40,dy:160},{name:"SQLite_Bridge",type:"agent",dx:240,dy:160},{name:"Context_Buffer",type:"agent",dx:440,dy:160},{name:"Evaluator",type:"agent",dx:640,dy:160}],wires:[[0,1],[1,2],[2,3]],edgeConds:["all","affirm","all"]},{id:"episodic_reflection",name:"Episodic Reflection Loop",desc:"A self-improving AI. It looks at its past mistakes and automatically corrects itself over time.",icon:"history",color:"var(--amber)",nodes:[{name:"Processor",type:"agent",dx:40,dy:160},{name:"Episodic_Recall",type:"agent",dx:240,dy:260},{name:"State_Injector",type:"agent",dx:240,dy:60},{name:"Decision_Gate",type:"gate",dx:440,dy:160}],wires:[[0,1],[1,2],[2,0],[0,3]],feedbackWires:[2],edgeConds:["all","all","tend","affirm"]},{id:"quantized_sparse_accelerator",name:"Quantized Sparse Accelerator",desc:"The speed demon. It strips away useless data so the AI can run incredibly fast on weak hardware.",icon:"zap",color:"var(--cyan)",nodes:[{name:"Quantizer",type:"agent",dx:40,dy:160},{name:"Sparse_Core",type:"agent",dx:240,dy:160},{name:"Weight_Filter",type:"gate",dx:440,dy:160},{name:"Accelerated_Out",type:"agent",dx:640,dy:160}],wires:[[0,1],[1,2],[2,3]],edgeConds:["all","all","!tend"]},{id:"hard_gated_mcp",name:"Hard-Gated MCP Bridge",desc:"A safe tool-user. It lets the AI use external apps but will instantly pull the plug if it acts weird.",icon:"shield-alert",color:"var(--red)",nodes:[{name:"MCP_Bridge",type:"agent",dx:40,dy:160},{name:"Veto_Orchestrator",type:"agent",dx:240,dy:160},{name:"Safe_Execution",type:"agent",dx:440,dy:160}],wires:[[0,1],[1,2]],edgeConds:["all","affirm"]},{id:"swarm_consensus",name:"Swarm Consensus (Albert)",desc:"A decentralized network. It forces multiple independent computers to agree before taking action.",icon:"network",color:"var(--green)",nodes:[{name:"Node_A",type:"agent",dx:40,dy:60},{name:"Node_B",type:"agent",dx:40,dy:260},{name:"Fleet_Sync",type:"agent",dx:240,dy:160},{name:"Consensus_Gate",type:"gate",dx:440,dy:160},{name:"Unified_State",type:"agent",dx:640,dy:160}],wires:[[0,2],[1,2],[2,3],[3,4]],edgeConds:["all","all","all","affirm"]}];function bt(e=""){const t=document.getElementById("arch-lib-items");if(!t)return;t.innerHTML='<div style="font-size:10px;color:var(--muted2);margin-bottom:10px;line-height:1.6;padding:8px 0;">Click to spawn a wired agent architecture on the canvas.</div>';const n=document.getElementById("archLibSearch");n&&e&&n.value!==e&&(n.value=e);const o={};Object.keys(jt).forEach(r=>o[r]=[]);const i=r=>{for(const[a,s]of Object.entries(jt))if(s.includes(r))return a;return"Orchestration & Consensus"};if(fn.forEach(r=>{if(e&&!r.name.toLowerCase().includes(e.toLowerCase())&&!r.desc.toLowerCase().includes(e.toLowerCase()))return;const a=i(r.id);o[a].push(r)}),Object.entries(o).forEach(([r,a])=>{if(a.length===0)return;const s=e?!0:tt[r],d=document.createElement("div");d.className="lib-category"+(s?"":" collapsed");const l=document.createElement("div");if(l.className="lib-category-header",l.style.display="flex",l.style.justifyContent="space-between",l.style.alignItems="center",l.innerHTML=`<span>${r}</span><i data-lucide="chevron-down"></i>`,l.onclick=()=>{tt[r]=!tt[r],bt(e)},d.appendChild(l),s){const c=document.createElement("div");c.className="lib-category-items",c.style.padding="8px 0",a.forEach(p=>{const m=document.createElement("div");m.className="archetype-card",m.onmouseenter=u=>{ve.startDelay(p.desc,u.clientX,u.clientY)},m.onmouseleave=()=>ve.hide(),m.draggable=!0,m.ondragstart=u=>{u.dataTransfer.setData("tern-node-type","archetype"),u.dataTransfer.setData("tern-arch-id",p.id)},m.innerHTML=`
          <div class="archetype-card-title" style="display:flex;align-items:center;gap:6px;">
            <i data-lucide="${p.icon}" style="width:12px;height:12px;color:${p.color}"></i>
            ${p.name}
          </div>
          <div class="archetype-card-desc">${p.desc}</div>
        `,m.onclick=()=>Et(p),c.appendChild(m)}),d.appendChild(c)}t.appendChild(d)}),t.querySelectorAll(".archetype-card").length===0){const r=document.createElement("div");r.id="no-arch-matches",r.style.padding="20px",r.style.textAlign="center",r.style.color="var(--muted2)",r.style.fontSize="11px",r.textContent="No archetypes match your search.",t.appendChild(r)}lucide.createIcons()}window.renderArchetypes=bt;function Et(e,t,n){document.getElementById("flow-canvas");const o=document.getElementById("canvas-hint");o&&(o.style.display="none");const{x:i,y:r}=te(-300,-200),a=t!==void 0?t:i,s=n!==void 0?n:r,d=[];e.nodes.forEach((l,c)=>{const p="node_"+Date.now()+"_"+c;d.push(p),F(l.name,"__arch__",a+l.dx,s+l.dy,l.type,p);const m=f.find(u=>u.id===p);m&&(m.props.code=gn(e.id,l.name,l.type),l.props&&(m.props={...m.props,...l.props}),wt(p))}),e.wires.forEach(([l,c],p)=>{const m="wire_"+Date.now()+"_"+p,u=(e.edgeConds||[])[p]||"all",g=(e.feedbackWires||[]).includes(p);x.push({id:m,fromId:d[l],toId:d[c],signal:0,condition:u,transform:"none",label:u!=="all"?u:"",priority:5,isFeedback:g})}),A(),L(),E(`Architecture "${e.name}" spawned`,"ok")}window.spawnArchetype=Et;function gn(e,t,n){const o=t.toLowerCase();if(n==="gate")return o.includes("consensus")||o.includes("majority")||o.includes("aggregation")?`fn main() -> trit {
    let val: trit = read_input();
    // Consensus logic is handled by the edge algebra,
    // but we can add local filtering here.
    return val;
}`:o.includes("safety")||o.includes("guard")||o.includes("validator")||o.includes("gatekeeper")?`fn main() -> trit {
    let val: trit = read_input();
    if val == reject { emit "VETO_TRIGGERED"; return reject; }
    emit "CHECK_PASSED";
    return val;
}`:o.includes("decision")||o.includes("routing")||o.includes("arbiter")?`fn main() -> trit {
    let val: trit = read_input();
    match val {
        affirm => { emit "PROCEED"; return affirm; }
        tend   => { emit "HOLD_PENDING"; return tend; }
        reject => { emit "TERMINATE"; return reject; }
    }
}`:"fn main() -> trit { return read_input(); }";if(e==="moe_13_flagship"){if(o.includes("expert"))return`fn main() -> trit {
    // MoE Expert Agent Sub-process
    emit "EXPERT_INVOKED";
    return consensus(truth(), hold());
}`;if(o.includes("orchestrator"))return`fn main() -> trit {
    // EMA convergence: S_r = α·e_r + (1−α)·S_{r−1}
    let sig: trit = read_input();
    emit "EMA_CONVERGING";
    return sig;
}`;if(o.includes("feedback"))return`fn main() -> trit {
    emit "REFINING_EMA";
    return tend;
}`}if(e.startsWith("kmu_")){if(o.includes("input")||o.includes("capture"))return`fn main() -> trit {
    // Initialize process state
    emit "INIT_STATION";
    return affirm;
}`;if(o.includes("price"))return`fn main() -> trit {
    // Evaluates against Tier-3 margin constraints
    emit "MARGIN_CHECK_OK";
    return affirm;
}`;if(o.includes("quality"))return`fn main() -> trit {
    // ISO-9001 compliance check
    return affirm;
}`;if(o.includes("fraud")||o.includes("pattern"))return`fn main() -> trit {
    // Heuristic anomaly detection
    return tend;
}`;if(o.includes("skills"))return`fn main() -> trit {
    // Match CV to job requirements
    return affirm;
}`;if(o.includes("logger")||o.includes("tracking"))return`fn main() -> trit {
    let sig: trit = read_input();
    print("@ TRACE: " + sig);
    return sig;
}`}if(e==="industry_iot_grid"){if(o.includes("node"))return`fn main() -> trit {
    // Hardened telemetry node
    emit "TELEMETRY_TX";
    return affirm;
}`;if(o.includes("watchdog"))return`fn main() -> trit {
    let input: trit = read_input();
    if input == tend { emit "LINK_LATENCY"; }
    return input;
}`;if(o.includes("emergency"))return`fn main() -> trit {
    emit "EMERGENCY_STOP";
    return reject;
}`}if(e==="debate"){if(o.includes("proposer"))return`fn main() -> trit {
    emit "PROPOSING_THESIS";
    return affirm;
}`;if(o.includes("challenger"))return`fn main() -> trit {
    emit "COUNTER_ARGUMENT";
    return reject;
}`}if(e==="local_rag_pipeline"){if(o.includes("sqlite"))return`fn main() -> trit {
    db_execute("SELECT vector FROM index WHERE state = +1");
    return affirm;
}`;if(o.includes("context"))return`fn main() -> trit {
    let ctx = read_context();
    emit "CTX_LENGTH: " + ctx.length;
    return affirm;
}`}if(e==="episodic_reflection"){if(o.includes("recall"))return`fn main() -> trit {
    let res = recall(read_input());
    return res ? affirm : tend;
}`;if(o.includes("injector"))return`fn main() -> trit {
    inject_state(tend);
    return tend;
}`}if(e==="quantized_sparse_accelerator"){if(o.includes("quantizer"))return`fn main() -> trit {
    return quantize(read_float());
}`;if(o.includes("sparse"))return`@sparseskip
fn main() -> trit {
    return matmul_step();
}`}if(e==="hard_gated_mcp"){if(o.includes("mcp"))return`fn main() -> trit {
    return mcp_call("tools/fetch");
}`;if(o.includes("veto"))return`fn main() -> trit {
    let v = read_input();
    if v == reject { return reject; }
    return affirm;
}`}return e==="swarm_consensus"&&o.includes("fleet")?`fn main() -> trit {
    sync_fleet();
    return affirm;
}`:`fn main() -> trit {
    return affirm;
}`}window.getArchetypeCode=gn;let H=null;function st(e){H=e,M=null;const t=document.getElementById("prop-header-label"),n=document.getElementById("prop-help-icon");t&&(t.textContent="EDGE PROPERTIES"),n&&(n.style.display="flex"),document.querySelectorAll(".flow-node").forEach(o=>o.classList.remove("selected")),We(),It()}window.selectWire=st;function We(){document.querySelectorAll(".flow-wire").forEach(e=>{e.classList.remove("selected-wire");const t=e.getAttribute("data-wire-id"),n=x.find(o=>o.id===t);n&&(e.classList.remove("cond-affirm","cond-tend","cond-reject","cond-all"),n.condition&&n.condition!=="all"&&e.classList.add("cond-"+n.condition),t===H&&e.classList.add("selected-wire"))})}window.updateWireStyles=We;function It(){const e=document.getElementById("prop-header-label"),t=document.getElementById("prop-body"),n=document.getElementById("prop-help-icon"),o=x.find(a=>a.id===H);if(!o){D();return}e&&(e.textContent="Edge Properties"),n&&(n.style.display="flex");const i=f.find(a=>a.id===o.fromId),r=f.find(a=>a.id===o.toId);Po.render(o,t,i,r)}window.updateEdgePanel=It;function Vo(e,t){const n=x.find(o=>o.id===H);n&&(n[e]=t,e==="condition"&&(n.label=t!=="all"?t:"",It()),We(),A())}window.updateWireProp=Vo;function yn(e){x=x.filter(t=>t.id!==e),H=null,document.getElementById("wire-handle").classList.remove("active"),A(),D(),document.getElementById("prop-header-label")&&(document.getElementById("prop-header-label").textContent="Node Properties")}window.deleteWire=yn;function Yo(){const e=document.getElementById("flow-inspector"),t=document.getElementById("ins-toggle-icon");e&&(e.classList.contains("inspector-minimized")?(e.classList.replace("inspector-minimized","inspector-expanded"),t&&t.setAttribute("data-lucide","chevron-down")):(e.classList.replace("inspector-expanded","inspector-minimized"),t&&t.setAttribute("data-lucide","chevron-up"),e.style.height=""),window.lucide&&lucide.createIcons())}window.toggleInspector=Yo;function T(e,t){const n=document.getElementById("ins-body");if(!n)return;const o=new Date().toLocaleTimeString([],{hour12:!1,hour:"2-digit",minute:"2-digit",second:"2-digit"}),i=document.createElement("div");i.className="ins-row",i.innerHTML=`
    <span class="ins-time">[${o}]</span>
    <span class="ins-node">${e}</span>
    <span class="ins-msg">${t}</span>
  `,n.appendChild(i),n.scrollTop=n.scrollHeight}window.logInspector=T;function hn(){const e={},t={};f.forEach(a=>{e[a.id]=0,t[a.id]=[]}),x.forEach(a=>{a.isFeedback||t[a.fromId]&&(t[a.fromId].push(a.toId),e[a.toId]=(e[a.toId]||0)+1)});const n=f.filter(a=>e[a.id]===0).map(a=>a.id),o=[];for(;n.length;){const a=n.shift();o.push(a),(t[a]||[]).forEach(s=>{--e[s]===0&&n.push(s)})}const i=o.length<f.length,r=i?f.filter(a=>!o.includes(a.id)):[];return{order:o,hasCycle:i,cycleNodes:r}}window.topoSort=hn;function vn(){const e=[],t=[];if(document.querySelectorAll(".flow-node").forEach(i=>i.classList.remove("node-error","node-warn")),document.querySelectorAll(".node-badge").forEach(i=>i.remove()),f.length===0)return e.push({msg:"Canvas is empty",nodeId:null}),{errors:e,warnings:t};const{hasCycle:n,cycleNodes:o}=hn();return n&&o.forEach(i=>{e.push({msg:`Cycle detected at "${i.name}"`,nodeId:i.id}),ue(i.id,"error","↺")}),f.forEach(i=>{var l;const r=x.filter(c=>c.toId===i.id),a=x.filter(c=>c.fromId===i.id),s=r.length===0,d=a.length===0;s&&d&&f.length>1&&(t.push({msg:`"${i.name}" is isolated — not connected`,nodeId:i.id}),ue(i.id,"warn","⚠")),i.type==="agent"&&!((l=i.props.code)!=null&&l.trim())&&(t.push({msg:`"${i.name}" has no .tern code`,nodeId:i.id}),a.length||ue(i.id,"warn","?")),r.forEach(c=>{const p=f.find(m=>m.id===c.fromId);p&&p.props.output_schema&&i.props.input_schema&&p.props.output_schema.trim()!==i.props.input_schema.trim()&&(t.push({msg:`Schema mismatch: "${p.name}" → "${i.name}"`,nodeId:i.id}),ue(i.id,"warn","≠"))}),r.length>1&&i.type==="agent"&&t.push({msg:`"${i.name}" has ${r.length} inputs — consider a Gate node for consensus`,nodeId:i.id})}),{errors:e,warnings:t}}window.validateGraph=vn;function ue(e,t,n){const o=document.getElementById(e);if(!o)return;o.classList.add(t==="error"?"node-error":"node-warn");const i=document.createElement("div");i.className="node-badge node-badge-"+t,i.textContent=n,o.appendChild(i)}window.markNode=ue;function xn(e,t){const n=document.getElementById("prop-header-label"),o=document.getElementById("prop-body"),i=document.getElementById("prop-help-icon");if(n&&(n.textContent="Graph Validation"),i&&(i.style.display="none"),e.length+t.length===0){o.innerHTML='<div style="color:var(--green);font-size:13px;text-align:center;margin-top:40px;">✓ Graph is valid</div><div style="color:var(--muted);font-size:11px;text-align:center;margin-top:8px;">No errors or warnings found.</div>';return}const a=[...e.map(s=>({...s,level:"error"})),...t.map(s=>({...s,level:"warn"}))];o.innerHTML=`
    <div style="font-size:11px;color:var(--muted2);margin-bottom:12px;">${e.length} error${e.length!==1?"s":""}, ${t.length} warning${t.length!==1?"s":""}</div>
    ${a.map(s=>`
      <div style="display:flex;gap:8px;padding:8px;margin-bottom:4px;border-radius:6px;background:${s.level==="error"?"rgba(239,68,68,0.08)":"rgba(245,158,11,0.08)"};border:1px solid ${s.level==="error"?"var(--red)":"var(--amber)"};cursor:${s.nodeId?"pointer":"default"};"
           ${s.nodeId?`onclick="selectNode('${s.nodeId}');updatePropertyPanel()"`:""}>
        <span style="color:${s.level==="error"?"var(--red)":"var(--amber)"};font-size:14px;flex-shrink:0;">${s.level==="error"?"✗":"⚠"}</span>
        <span style="font-size:11px;color:var(--text);line-height:1.5;">${s.msg}</span>
      </div>`).join("")}
    <button class="btn btn-ghost" style="width:100%;margin-top:12px;font-size:11px;" onclick="document.querySelectorAll('.flow-node').forEach(el=>el.classList.remove('node-error','node-warn'));document.querySelectorAll('.node-badge').forEach(b=>b.remove());updatePropertyPanel();">Clear markers</button>
  `}window.showValidationPanel=xn;const Jo={merge(e){if(!e.length)return{val:0,conf:0};let t=0,n=0;if(e.forEach(a=>{n+=a.val*a.conf,t+=a.conf}),t===0)return{val:0,conf:0};const o=n/t,i=o>.33?1:o<-.33?-1:0,r=t/e.length;return{val:i,conf:Math.min(1,r)}},transform(e,t){let n={...e};const o=t.priority?(11-t.priority)*.02:.05;if(n.conf=Math.max(0,n.conf-o),t.condition&&t.condition!=="all"&&!(t.condition==="affirm"&&n.val===1||t.condition==="tend"&&n.val===0||t.condition==="reject"&&n.val===-1||t.condition==="!reject"&&n.val!==-1||t.condition==="!tend"&&n.val!==0)){if(t.transform==="block")return null;t.transform==="flip"&&(n.val=-n.val,n.conf*=.8),t.transform==="hold"&&(n.val=0,n.conf*=.5)}return n}},ie=[],wn=2e3;function le(){const e=document.getElementById("flow-fog-canvas"),t=document.getElementById("flow-canvas-wrap");if(!e||!t)return;const n=e.getContext("2d");(e.width!==t.clientWidth||e.height!==t.clientHeight)&&(e.width=t.clientWidth,e.height=t.clientHeight),n.clearRect(0,0,e.width,e.height),n.fillStyle="rgba(10, 15, 25, 0.35)",n.fillRect(0,0,e.width,e.height),f.forEach(o=>{const i=o.x*h.scale+h.x+90*h.scale,r=o.y*h.scale+h.y+40*h.scale;if(i<-500||i>e.width+500||r<-500||r>e.height+500)return;const a=document.getElementById("status-"+o.id);let s=0;a&&(a.classList.contains("ok")?s=1:a.classList.contains("err")?s=-1:a.classList.contains("run")&&(s=0));const d=350*h.scale,l=n.createRadialGradient(i,r,40*h.scale,i,r,d);s===1?(l.addColorStop(0,"rgba(34, 197, 94, 0.3)"),l.addColorStop(1,"rgba(10, 12, 16, 0)")):s===-1?(l.addColorStop(0,"rgba(239, 68, 68, 0.3)"),l.addColorStop(1,"rgba(10, 12, 16, 0)")):(l.addColorStop(0,"rgba(168, 85, 247, 0.2)"),l.addColorStop(1,"rgba(10, 12, 16, 0)")),n.globalCompositeOperation="screen",n.fillStyle=l,n.beginPath(),n.arc(i,r,d,0,Math.PI*2),n.fill()})}window.updateFogHeatmap=le;async function St(){if(P!=="running"){Z("running");try{const{errors:e,warnings:t}=vn();if(e.length>0){xn(e,t),E(`${e.length} error${e.length>1?"s":""} — fix before simulating`,"error");return}f.forEach(p=>{p.visited=!1,p.executed=!1,p.props&&(p.props.status="")}),x.forEach(p=>{p.active=!1,p.signal=0});const n=document.getElementById("scrub-layer");n&&n.getContext("2d").clearRect(0,0,n.width,n.height);const o=document.getElementById("global-timeline");o&&(o.value=0),bn();const i=document.getElementById("simStopBtn");i&&(i.style.display="inline-flex");const r=document.getElementById("flow-inspector");r&&(r.classList.add("active"),r.classList.contains("inspector-minimized")&&r.classList.replace("inspector-minimized","inspector-expanded"));const a=document.getElementById("ins-body");a&&(a.innerHTML=""),document.querySelectorAll(".trit-particle-ghost").forEach(p=>p.remove()),document.querySelectorAll(".flow-node").forEach(p=>p.classList.remove("pulse-affirm","pulse-reject","pulse-hold","node-error","node-warn")),f.forEach(p=>re(p.id,"")),ie.length=0,T("SYSTEM","🚀 TernFlow Engine v2 initialized"),le(),En(0),f.filter(p=>!x.some(m=>m.toId===p.id)).forEach(p=>{ie.push({toId:p.id,val:1,conf:1,origin:"ROOT"})});const d=[...ie],{scheduledEvents:l,maxSimDuration:c}=await Xo(d);window.globalScheduledEvents=l,console.log("DEBUG -> Events generated:",l.length,"| Total Duration:",c),N=!1,(l.length===0||c===0)&&console.warn("[DIAGNOSTIC] Simulation data empty. Check roots and latencies."),await kt(l,c)}catch(e){console.error("Simulation Start Failure:",e),E("Simulation failed to initialize","err"),window.TERNLANG_CRITICAL_DEBUG&&window.TERNLANG_CRITICAL_DEBUG.push({ts:Date.now(),msg:"runSimulation Crash",error:e.message})}finally{se&&!N||(se=!1,Fe())}}}window.runSimulation=St;async function Xo(e){const t=[];let n=0;const o=e,i={};for(;o.length>0&&t.length<wn;){if(P==="idle")return{scheduledEvents:[],maxSimDuration:0};if(P==="paused"&&await ft(),P==="idle")return{scheduledEvents:[],maxSimDuration:0};if(N)break;const r=o.shift(),a=f.find(u=>u.id===r.toId);if(!a)continue;const l=(r.absEndTime||0)+150;i[a.id]=Math.max(i[a.id]||0,l),n=Math.max(n,l);const c=await Bn(a,r.val,!0);if(N)break;const p={val:c,conf:r.conf,origin:a.id},m=x.filter(u=>u.fromId===a.id);for(const u of m){const g=Jo.transform(p,u);if(g){const v=i[a.id],w=parseFloat(u.latency),I=isNaN(w)?Vt:w,y=v+I,S={wireId:u.id,val:g.val,conf:g.conf,startTime:v,endTime:y,duration:I,fromId:u.fromId,toId:u.toId};t.push(S),n=Math.max(n,y),o.push({toId:u.toId,...g,absEndTime:y})}}}return ie.length=0,{scheduledEvents:t,maxSimDuration:n}}async function kt(e,t){const n=document.getElementById("global-timeline"),o=document.getElementById("timeline-tick-label"),i=new Set;n&&(n.value=0,n.max=t),oe=performance.now(),U=0,Z("running");const r=async()=>{if(P==="idle"){console.log("[TernFlow] Engine Terminated (Stopped).");return}if(P==="paused"){if(console.log("[TernFlow] Engine Yielded (Paused). Awaiting resumption..."),await ft(),P==="idle")return;oe=performance.now()}const a=performance.now(),s=a-oe;oe=a,U+=s,U>=t&&(U=t,Z("idle"),T("SYSTEM","✓ Pre-flight playback complete")),n&&(n.value=U,o&&(o.textContent=`TIME: ${(U/1e3).toFixed(2)}s`)),Sn(U,e,i),P==="running"&&requestAnimationFrame(r)};window.currentDriveTimeline=r,requestAnimationFrame(r)}window.runSimulationCore=kt;let J=null,Ce=null;function bn(){R=[],J=null,Ce=null,lt=-1;const e=document.getElementById("scrub-layer");e&&e.getContext("2d").clearRect(0,0,e.width,e.height)}window.resetSimHistory=bn;function En(e,t=[],n=0,o=500){(R.length===0||!J)&&(J={nodes:f.map(s=>{const d=document.getElementById(s.id),l=d?d.classList.contains("pulse-affirm")?"affirm":d.classList.contains("pulse-reject")?"reject":d.classList.contains("pulse-hold")?"hold":"":"";return{id:s.id,status:s.props.status||"",pulse:l}}),wires:x.map(s=>({id:s.id,signal:s.signal||0}))},Ce=JSON.parse(JSON.stringify(J)));const i={tick:e,activeSignals:t,nodeDeltas:[],wireDeltas:[],startTime:n,duration:o};if(f.forEach(s=>{const d=document.getElementById(s.id),l=d?d.classList.contains("pulse-affirm")?"affirm":d.classList.contains("pulse-reject")?"reject":d.classList.contains("pulse-hold")?"hold":"":"",c=s.props.status||"",p=Ce.nodes.find(m=>m.id===s.id);p&&(p.status!==c||p.pulse!==l)&&(i.nodeDeltas.push({id:s.id,status:c,pulse:l}),p.status=c,p.pulse=l)}),x.forEach(s=>{const d=s.signal||0,l=Ce.wires.find(c=>c.id===s.id);l&&l.signal!==d&&(i.wireDeltas.push({id:s.id,signal:d}),l.signal=d)}),R.push(i),R.length>wn){const s=R.shift();s.nodeDeltas.forEach(d=>{const l=J.nodes.find(c=>c.id===d.id);l&&(l.status=d.status,l.pulse=d.pulse)}),s.wireDeltas.forEach(d=>{const l=J.wires.find(c=>c.id===d.id);l&&(l.signal=d.signal)})}const r=document.getElementById("global-timeline"),a=n+o;r&&(r.max=a)}window.captureSimSnapshot=En;function Qo(){const e=document.getElementById("global-timeline"),t=document.getElementById("timeline-tick-label");if(R.length===0)return;const n=R[0].startTime,o=R[R.length-1],i=o.startTime+o.duration;e&&(e.min=n,e.max=i,e.value=i),t&&(t.textContent=`TIME: ${(i/1e3).toFixed(2)}s`)}window.showTimeline=Qo;function Zo(e){const t=parseFloat(e),n=document.getElementById("timeline-tick-label");n&&(n.textContent=`TIME: ${(t/1e3).toFixed(2)}s`),Tt(t)}window.scrubToTimeline=Zo;let lt=-1,dt=null,In=0;function Tt(e){In=parseFloat(e),dt||(dt=requestAnimationFrame(ei))}window.requestScrub=Tt;function ei(){dt=null;const e=In;if(R.length===0||!J)return;let t=R.findIndex(i=>e>=i.startTime&&e<i.startTime+i.duration);t===-1&&(e>=R[R.length-1].startTime?t=R.length-1:t=0);const n=R[t],o=n.tick;if(Math.min(1,Math.max(0,(e-n.startTime)/n.duration)),o!==lt){lt=o;const i=JSON.parse(JSON.stringify(J));for(let r=0;r<=t&&r<R.length;r++){const a=R[r];a.nodeDeltas.forEach(s=>{const d=i.nodes.find(l=>l.id===s.id);d&&(d.status=s.status,d.pulse=s.pulse)}),a.wireDeltas.forEach(s=>{const d=i.wires.find(l=>l.id===s.id);d&&(d.signal=s.signal)})}i.nodes.forEach(r=>{const a=document.getElementById(r.id);a&&(a.classList.remove("pulse-affirm","pulse-reject","pulse-hold"),r.pulse&&a.classList.add("pulse-"+r.pulse),re(r.id,r.status));const s=f.find(d=>d.id===r.id);s&&(s.props.status=r.status)}),i.wires.forEach(r=>{const a=x.find(s=>s.id===r.id);a&&(a.signal=r.signal)}),document.querySelectorAll(".trit-particle-ghost").forEach(r=>r.remove()),A(),le()}Sn(e),U=e}function Sn(e,t=[],n=null){try{const o=document.getElementById("scrub-layer"),i=document.getElementById("flow-canvas-wrap");if(!o||!i)return;const r=o.getContext("2d");(o.width!==i.clientWidth||o.height!==i.clientHeight)&&(o.width=i.clientWidth,o.height=i.clientHeight),r.clearRect(0,0,o.width,o.height);const a=t.length>0?t:window.globalScheduledEvents||[];t.length>0&&(window.globalScheduledEvents=t);const s=new Set,d={};a.forEach(l=>{if(e>=l.startTime&&e<=l.endTime){const m=(e-l.startTime)/l.duration,u=x.find(g=>g.id===l.wireId);if(u){d[u.id]={signal:l.val,alpha:1};const g=document.getElementById(u.id);if(g)try{const v=g.getTotalLength(),w=g.getPointAtLength(m*v),I=w.x*h.scale+h.x,y=w.y*h.scale+h.y,S=l.val===1?"#22c55e":l.val===-1?"#ef4444":"#f59e0b";r.beginPath(),r.fillStyle=S,r.shadowColor=S,r.shadowBlur=10*h.scale;const $=(6+8*(l.conf||1))*h.scale;r.arc(I,y,$,0,Math.PI*2),r.fill(),r.shadowBlur=0}catch{}}}const p=150;if(e>=l.startTime-p&&e<=l.startTime&&s.add(l.fromId),e>=l.endTime&&e<=l.endTime+p&&s.add(l.toId),n&&e>=l.endTime&&!n.has(l.wireId+"_"+l.endTime)){const m=f.find(g=>g.id===l.toId),u=f.find(g=>g.id===l.fromId);if(m){const g=l.val===1?"+1 (Affirm)":l.val===-1?"-1 (Reject)":"0 (Tend)";T(m.name,`Signal arrival from ${u?u.name:"ROOT"} -> ${g}`),n.add(l.wireId+"_"+l.endTime);const v=x.filter(I=>I.fromId===l.toId),w=f.find(I=>I.id===l.toId);if(v.length===0&&w&&w.type!=="artifact")$n(w,l.val);else if(w&&w.type==="artifact"){const I=document.getElementById(`art-body-${w.id}`);if(I){const y=l.val===1?"AFFIRM":l.val===-1?"REJECT":"TEND";I.textContent=`Source: ${u?u.name:"Unknown"}
Resolved Signal: ${y}
Status: Resolved`,I.style.color=l.val===1?"var(--green)":l.val===-1?"var(--red)":"var(--text)"}}}}}),document.querySelectorAll(".flow-node").forEach(l=>{const c=l.getAttribute("id");l.classList.remove("pulse-affirm","pulse-reject","pulse-hold"),s.has(c)&&l.classList.add("pulse-affirm")}),x.forEach(l=>{const c=d[l.id],p=document.getElementById(l.id);p&&(p.classList.remove("active-1","active-n1","active-0"),c&&p.classList.add(`active-${c.signal===1?"1":c.signal===-1?"n1":"0"}`))})}catch(o){window.TERNLANG_CRITICAL_DEBUG&&window.TERNLANG_CRITICAL_DEBUG.push({ts:performance.now(),msg:"renderScrubLayer Silent Crash Prevented",error:o.message})}}function ti(e){Tt(e)}window.scrubSimulation=ti;function kn(){const e=document.getElementById("flow-inspector"),t=e.querySelector(".ins-head"),n=document.getElementById("flow-inspector-resizer");if(!e||!t)return;let o=!1,i=!1,r,a;t.onmousedown=l=>{if(l.target.closest("button"))return;o=!0;const c=e.getBoundingClientRect();r=l.clientX-c.left,a=l.clientY-c.top,e.style.transform="none",e.style.bottom="auto",e.style.left=c.left+"px",e.style.top=c.top+"px",document.addEventListener("mousemove",s),document.addEventListener("mouseup",d),l.preventDefault()},n&&(n.onmousedown=l=>{i=!0;const c=e.getBoundingClientRect();a=l.clientY-c.top,l.preventDefault(),document.addEventListener("mousemove",s),document.addEventListener("mouseup",d)});function s(l){if(o)e.style.left=l.clientX-r+"px",e.style.top=l.clientY-a+"px";else if(i){const c=e.getBoundingClientRect(),p=Math.max(250,l.clientX-c.left),m=Math.max(32,l.clientY-a);e.style.width=p+"px",e.style.height=m+"px"}}function d(){o=!1,i=!1,document.removeEventListener("mousemove",s),document.removeEventListener("mouseup",d)}}window.initInspectorDraggable=kn;function ye(){const e=document.getElementById("flow-library"),t=document.getElementById("flow-props"),n=document.querySelector(".timeline-container");if(!e||!t||!n)return;const o=e.offsetWidth,i=t.offsetWidth;n.style.left=o+"px",n.style.width=`calc(100% - ${o+i}px)`}window.updateTimelineBridge=ye;function Tn(){const e=document.getElementById("flow-library"),t=document.getElementById("flow-sidebar-resizer");if(!e||!t)return;t.addEventListener("mousedown",i=>{t.classList.add("active");const r=s=>{const d=s.clientX;d>=180&&d<=500&&(e.style.width=d+"px",ye())},a=()=>{t.classList.remove("active"),document.removeEventListener("mousemove",r),document.removeEventListener("mouseup",a)};document.addEventListener("mousemove",r),document.addEventListener("mouseup",a)}),Cn();const n=new ResizeObserver(()=>ye());n.observe(e);const o=document.getElementById("flow-props");o&&n.observe(o),ye()}window.initSidebarResizer=Tn;function Cn(){const e=document.getElementById("flow-props"),t=document.getElementById("flow-props-resizer");!e||!t||t.addEventListener("mousedown",n=>{t.classList.add("active");const o=e.getBoundingClientRect();n.clientX-o.left;const i=a=>{const d=window.innerWidth-a.clientX;d>=250&&d<=500&&(e.style.width=d+"px",ye())},r=()=>{t.classList.remove("active"),document.removeEventListener("mousemove",i),document.removeEventListener("mouseup",r)};document.addEventListener("mousemove",i),document.addEventListener("mouseup",r)})}window.initRightSidebarResizer=Cn;function re(e,t){const n=document.getElementById("status-"+e);n&&(n.className="fn-status"+(t?" "+t:""),n.title=t||"idle")}window.setNodeStatus=re;async function _n(e,t){var c;let n=(e.props.template||"{{input}}").replace("{{input}}",t===1?"affirm":t===-1?"reject":"tend");const o=e.props.system_prompt||"You are a ternary logic processor. Output only +1 (affirm), 0 (tend), or -1 (reject).",i=e.props.protocol||"openai",r=e.props.model_id||"";let a=n;e.props.runtime_buffer&&e.props.runtime_buffer.data&&(a=`Here is the ingested data for your analysis:

<context><data_payload>
${e.props.runtime_buffer.data}
</data_payload></context>

User: ${n}`);const s=Math.ceil((o.length+a.length)/4);let d=8192;const l=r.toLowerCase();if(l.includes("gemini-1.5")?d=1048576:l.includes("gemini")?d=32768:l.includes("claude-3")?d=2e5:l.includes("gpt-4")?d=128e3:l.includes("gpt-3.5")||l.includes("gpt-35")?d=16384:l.includes("grok")?d=131072:i==="anthropic"?d=2e5:i==="google"&&(d=1048576),s>d*.8)return T(e.name,`❌ Token Safety Halt: Payload (${s} tk) exceeds 80% of ${d} tk context window.`),-1;T(e.name,`🌐 Calling LLM [${i}:${r||"default"}] (${s} tk)…`);try{const p=await fetch("/api/run",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({source:`// LLM Bridge Proxy
fn main() -> trit { return hold; }`,sql:e.props.sql_query||"",llm_config:{system:o,prompt:a,protocol:i,model_id:r,api_key:localStorage.getItem("ternstudio-gemini-key")||e.props.api_key||"",base_url:e.props.base_url,temperature:.1,max_tokens:150}})});if(!p.ok)throw new Error(`HTTP ${p.status}`);const m=await p.json(),u=String(m.result||"").toLowerCase();let g=0,v=.5;return u.includes("confidence: 1.0")||u.includes("conf: 1.0")||u.includes("score: 1.0")?v=1:(u.includes("confidence: -1.0")||u.includes("conf: -1.0"))&&(v=-1),u.includes("+1")||u.includes("affirm")||u.includes("yes")?g=1:(u.includes("-1")||u.includes("reject")||u.includes("no"))&&(g=-1),g===-1&&v>.9&&(T(e.name,"🛑 HARD REJECT: LLM returned high-confidence rejection. Aborting simulation."),N=!0),window.dispatchEvent(new CustomEvent("ternlang_local_trace",{detail:{trace_id:"llm-"+Date.now(),timestamp_ms:Date.now(),node_id:e.id,event_type:"LLM_Bridge_Reasoning",signal_in:t,signal_out:g,latency_ms:500,reasoning:m.result||"No reasoning path returned."}})),v===1?(T(e.name,`✅ TAP: High Confidence (${g===1?"+1":g===-1?"-1":"0"}) -> Autonomous Execution.`),g):(T(e.name,`🟡 TAP: Ambiguity detected (conf: ${v}) -> Freezing for Operator.`),e.props.pending_actuator={code:m.result,paths:["Affirm (+1)","Reject (-1)"]},0)}catch(p){return T(e.name,`❌ LLM Error: ${p.message}. Falling back to deterministic mapper.`),(c=e.props.system_prompt)!=null&&c.toLowerCase().includes("safety")?1:t}}window.executeLLMNode=_n;async function Bn(e,t,n=!1){if(P==="idle"||(P==="paused"&&await ft(),P==="idle")||N)return 0;if(N)return t;const o=document.getElementById(e.id);if(!o&&!n)return t;n||(re(e.id,"run"),o&&o.classList.remove("pulse-affirm","pulse-reject","pulse-hold"),window.dispatchEvent(new CustomEvent("ternlang_local_trace",{detail:{trace_id:"local-"+Date.now()+"-"+Math.random().toString(36).substr(2,5),timestamp_ms:Date.now(),node_id:e.id,event_type:e.type==="external"?"LLM_Bridge":"Logic_Eval",signal_in:t,signal_out:0,latency_ms:e.type==="external"?0:50}})));let i=t;if(e.type==="external")i=await _n(e,t);else if(e.type==="moe13")i=await Ln(e,t);else if(e.type==="datasource"){const s=e.props.payload||"";n||T(e.name,`📡 Injecting Payload: [${e.props.data_type||"text"}] ${s.substring(0,20)}...`),x.filter(l=>l.fromId===e.id).forEach(l=>{const c=f.find(p=>p.id===l.toId);c&&(c.props.runtime_buffer={type:e.props.data_type||"text",data:s},n||T("SYSTEM",`💾 Buffered ${s.length} bytes to ${c.name}`))}),i=1}else{const s=e.props.code||"";if(s.trim()){n||T(e.name,"⚡ Executing logic…");const d=Ve(s);if(d.ok){i=d.trit===1?1:d.trit===-1?-1:0;const l=i===1?"+1 AFFIRM":i===-1?"-1 REJECT":"0 TEND";n||T(e.name,`→ ${l}${d.output&&d.output.length?" · "+d.output.join(", "):""}`)}else return n||(T(e.name,`✗ ${d.error||"error"}`),re(e.id,"err"),o&&o.classList.add("pulse-reject")),n||await new Promise(l=>setTimeout(l,600)),-1}else{const d=t===1?"+1 AFFIRM":t===-1?"-1 REJECT":"0 TEND";n||T(e.name,`→ ${d} (passthrough)`)}}if(N)return i;if(!n){const s=i===1?"pulse-affirm":i===-1?"pulse-reject":"pulse-hold";o&&o.classList.add(s),re(e.id,i===1?"ok":i===-1?"err":"run")}if(e.props.pending_actuator)return n||(T("SYSTEM",`🟡 TAP: State 0 Suspension at "${e.name}". Awaiting Operator…`),N=!0),0;const r=x.filter(s=>s.fromId===e.id),a=f.filter(s=>s.type==="artifact"&&s.parentId===e.id);return r.length===0&&e.type!=="artifact"&&(n||(T("SYSTEM","🛑 Terminal Payload Detected — Hard Halt engaged."),N=!0)),a.forEach(s=>{const d=document.getElementById(`art-body-${s.id}`);d&&(d.textContent=`Source: ${e.name}
Resolved Signal: ${i===1?"AFFIRM":i===-1?"REJECT":"TEND"}
Status: Halt Emitted`,d.style.color=i===1?"var(--green)":i===-1?"var(--red)":"var(--text)")}),n||await new Promise(s=>setTimeout(s,500)),i}window.simulateNode=Bn;async function Ln(e,t){T(e.name,"🧠 MOE-13: Initiating Live Deliberation Cycle...");const n=e.id,o=document.getElementById(`moe-veto-alert-${n}`),i=document.getElementById(`moe-verdict-${n}`);o&&(o.style.display="none"),i&&(i.textContent="QUERYING BACKEND...",i.style.color="var(--magenta)");try{const r=await fetch("/api/moe/orchestrate",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({query:e.props.system_prompt||"Evaluate the current triadic signal state.",evidence:[t===1?1:t===-1?-1:0,.5,.5,.5,.5,.5]})});if(!r.ok)throw new Error(`HTTP ${r.status}`);const a=await r.json();return a.verdicts&&a.verdicts.forEach(s=>{const d=document.getElementById(`moe-vote-${n}-${s.expert_id}`),l=document.getElementById(`moe-conf-${n}-${s.expert_id}`);d&&(d.textContent=s.trit===1?"+1":s.trit===-1?"-1":"0",d.style.color=s.trit===1?"var(--green)":s.trit===-1?"var(--red)":"var(--muted2)"),l&&(l.textContent=Math.round(s.confidence*100)+"%")}),a.safety_vetoed&&(o&&(o.style.display="block"),T(e.name,"🛑 CRITICAL VETO: Safety experts triggered Hard Halt."),N=!0),a.trit===-1&&a.confidence>.9&&(T(e.name,`🛑 HARD REJECT: High-confidence rejection (${(a.confidence*100).toFixed(1)}%) triggered Hard Halt.`),N=!0),i&&(i.textContent=a.label.toUpperCase(),i.style.color=a.trit===1?"var(--green)":a.trit===-1?"var(--red)":"var(--amber)"),T(e.name,`✓ Deliberation Complete. Verdict: ${a.trit} (${a.label}) | Conf: ${(a.confidence*100).toFixed(1)}%`),window.dispatchEvent(new CustomEvent("ternlang_local_trace",{detail:{trace_id:"moe-"+Date.now(),timestamp_ms:Date.now(),node_id:e.id,event_type:"MoE-13_Orchestration",signal_in:t,signal_out:a.trit,latency_ms:150,reasoning:a.verdicts?a.verdicts.map(s=>`${s.expert_name}: ${s.reasoning}`).join(" | "):""}})),a.trit}catch(r){return T(e.name,`❌ MoE-13 Error: ${r.message}. Falling back to Tend.`),i&&(i.textContent="OFFLINE",i.style.color="var(--muted2)"),0}}window.executeMOE13=Ln;function $n(e,t){let n=`Source: ${e.name}
Resolved Signal: ${t===1?"AFFIRM":t===-1?"REJECT":"TEND"}
Status: Resolved`;const o=!!e.props.pending_actuator;if(o){const g=e.props.pending_actuator;g.error?n=`❌ WASM RUNTIME ERROR

Traceback:
${g.error}

Failed Code:
${g.last_failed_code}

Action: Please edit in 'transmute' mode or reject.`:n=`⚠️  TAP SUSPENSION (STATE 0)

Proposed Actuator Logic:
${g.code}

Divergent Paths:
- ${g.paths.join(`
- `)}

Action: Awaiting Operator Approval.`}const i=f.find(g=>g.type==="artifact"&&g.parentId===e.id);if(i){i.props.payload=n;const g=document.getElementById(`art-body-${i.id}`);g&&(g.textContent=n,g.style.color=o?"var(--amber)":t===1?"var(--green)":t===-1?"var(--red)":"var(--text)",o&&Ht(i.id,e.id)),M===i.id&&D();return}const r=document.getElementById(e.id);if(!r)return;const a=parseFloat(r.style.left),s=parseFloat(r.style.top),d="art_"+Date.now(),l=o?"TAP: "+e.name:"Result: "+e.name,c=dn(a+350,s,300,200);F(l,"__artifact__",c.x,c.y,"artifact",d),xt(c.x,c.y);const p=f.find(g=>g.id===d);p&&(p.parentId=e.id,p.props.state="lock",p.props.payload=n);const m=document.getElementById(`art-body-${d}`);m&&(m.textContent=n,m.style.color=o?"var(--amber)":t===1?"var(--green)":t===-1?"var(--red)":"var(--text)",o&&Ht(d,e.id));const u="wire_art_"+Date.now();x.push({id:u,fromId:e.id,toId:d,condition:"all",transform:"none",label:o?"TAP PENDING":"RESULT",priority:10}),A(),L(),lucide.createIcons()}function Ht(e,t){const n=document.getElementById(`art-body-${e}`);if(!n)return;const o=document.createElement("div");o.style.marginTop="12px",o.style.display="flex",o.style.gap="8px",o.innerHTML=`
    <button class="btn-pill" style="background:var(--green); color:white; border:none; padding:4px 12px; font-size:10px; cursor:pointer;" onclick="resolveTAP('${e}', '${t}', 1)">Approve (+1)</button>
    <button class="btn-pill" style="background:var(--red); color:white; border:none; padding:4px 12px; font-size:10px; cursor:pointer;" onclick="resolveTAP('${e}', '${t}', -1)">Reject (-1)</button>
  `,n.appendChild(o)}async function ni(e,t,n){const o=f.find(r=>r.id===t);if(!o)return;const i=o.props.pending_actuator;if(n===1&&i&&i.code){T(o.name,"⚙️  TAP Execution Loop: Initiating WASM Sandbox...");const r=await Kt(i.code);r.ok?(T(o.name,`✅ WASM Success: Output captured (${r.output.length} bytes).`),o.props.runtime_buffer={type:"text",data:r.output},delete o.props.pending_actuator,Ne(e),rt(t,1)):(T(o.name,"❌ WASM Runtime Error: Reverting to State 0..."),o.props.pending_actuator.error=r.traceback||r.error,o.props.pending_actuator.last_failed_code=i.code,$n(o,0),D())}else delete o.props.pending_actuator,T("SYSTEM",`✅ TAP RESOLVED: Operator injected ${n===1?"+1":"-1"} to "${o.name}".`),Ne(e),rt(t,n)}window.resolveTAP=ni;function An(e,t){const n=f.find(o=>o.id===e);if(n&&(n.props.payload=t,n.parentId)){const o=f.find(i=>i.id===n.parentId);o&&o.props.pending_actuator&&(o.props.pending_actuator.code=t)}}window.updateArtifactPayload=An;function Mn(e,t){const n=f.find(l=>l.id===e);if(!n)return;n.props.state=t;const o=document.getElementById(e),i=document.getElementById(`art-body-${e}`),r=document.getElementById(`art-edit-${e}`),a=document.getElementById(`art-socket-label-${e}`);o.classList.remove("state-lock","state-transmute","state-extend"),o.classList.add(`state-${t}`),t==="lock"?(i.style.display="block",r.style.display="none",a.style.display="none",r.value&&(i.textContent=r.value,An(e,r.value))):t==="transmute"?(i.style.display="none",r.style.display="block",a.style.display="none",r.value=i.textContent):t==="extend"&&(i.style.display="block",r.style.display="none",a.style.display="flex"),o.querySelectorAll(".art-btn").forEach(l=>l.classList.remove("active"));const s=t==="lock"?0:t==="transmute"?1:2,d=o.querySelectorAll(".art-btn");d[s]&&d[s].classList.add("active"),A(),L()}window.setArtifactState=Mn;function Rn(){f.filter(t=>t.type==="artifact").forEach(t=>{x=x.filter(o=>o.fromId!==t.id&&o.toId!==t.id);const n=document.getElementById(t.id);n&&n.remove()}),f=f.filter(t=>t.type!=="artifact"),A()}window.clearResultArtifacts=Rn;function A(){le();let e=document.getElementById("flow-svg-layer");if(!e){const t=document.getElementById("flow-canvas");if(!t)return;e=document.createElementNS("http://www.w3.org/2000/svg","svg"),e.id="flow-svg-layer",t.prepend(e)}if(e.innerHTML="",document.querySelectorAll(".edge-badge").forEach(t=>t.remove()),f.forEach(t=>{const n=document.getElementById(t.id);n&&(x.some(i=>i.fromId===t.id)?n.classList.add("has-output"):n.classList.remove("has-output"))}),x.forEach(t=>{const n=document.getElementById(t.fromId),o=document.getElementById(t.toId);if(!n||!o)return;const i=n.querySelector(".flow-port-out"),r=o.querySelector(".flow-port-in");if(!i||!r)return;const a=X(i),s=X(r);Pe(a,s,t.id,t.signal,t,t.confidence)}),_){const t=_.start,n=_.end;Pe(t,n,"active-wire",0,null);const o=f.find(r=>r.id===_.fromId);let i=document.getElementById("evolution-ghost");if(o&&o.type==="artifact"&&o.props.state==="extend"&&_.fromIsOutput){i||(i=document.createElement("div"),i.id="evolution-ghost",i.className="flow-node agent ghost-node",i.innerHTML=`
            <div class="fn-head" style="opacity:0.6; pointer-events:none;">
              <div style="display:flex; align-items:center; gap:6px; font-size:10px; font-weight:700; color:var(--cyan)">
                <i data-lucide="bot" style="width:12px"></i>TRANSMUTED
              </div>
            </div>
            <div class="fn-body" style="padding:10px; font-size:9px; color:var(--muted); text-align:center; pointer-events:none;">RELEASE TO EVOLVE</div>
          `,document.getElementById("flow-canvas").appendChild(i),lucide.createIcons()),i.style.left=n.x-90+"px",i.style.top=n.y-40+"px",i.style.display="block";const r=document.getElementById("active-wire");r&&(r.style.stroke="var(--cyan)",r.style.strokeWidth="3",r.style.strokeDasharray="5 3",r.style.opacity="0.8")}else i&&(i.style.display="none")}else{const t=document.getElementById("evolution-ghost");t&&(t.style.display="none")}}window.updateWires=A;function Nn(e,t,n){const o=t.x-e.x,i=t.y-e.y;if(n&&n.cp)return`M ${e.x} ${e.y} Q ${n.cp.x} ${n.cp.y} ${t.x} ${t.y}`;let r=null;const a=n?n.fromId:null,s=n?n.toId:null;for(const l of f){if(l.id===a||l.id===s)continue;const c=l.type==="artifact"?300:l.type==="moe13"?320:180,p=l.type==="artifact"?200:l.type==="moe13"?360:80,m=20,u=l.x-c/2-m,g=l.x+c/2+m,v=l.y-p/2-m,w=l.y+p/2+m,I=e.x+o/2,y=e.y+i/2;if(I>u&&I<g&&y>v&&y<w){r=l;break}}if(r){r.type==="artifact"||r.type;const l=r.type==="artifact"?200:r.type==="moe13"?360:80,c=e.y<r.y?r.y-l/2-40:r.y+l/2+40;return`M ${e.x} ${e.y} Q ${r.x} ${c} ${t.x} ${t.y}`}if(o<60){const l=Math.max(120,Math.abs(i)*.4);return`M ${e.x} ${e.y} C ${e.x+l} ${e.y}, ${t.x-l} ${t.y}, ${t.x} ${t.y}`}const d=o*.5;return`M ${e.x} ${e.y} C ${e.x+d} ${e.y}, ${t.x-d} ${t.y}, ${t.x} ${t.y}`}window.computeWirePath=Nn;async function oi(e,t,n=1){const o=document.getElementById(e.fromId),i=document.getElementById(e.toId);if(!o||!i)return;const r=o.querySelector(".flow-port-out"),a=i.querySelector(".flow-port-in"),s=X(r),d=X(a);Pe(s,d,e.id,t,e,n)}window.animateSignal=oi;function X(e){const t=e.getBoundingClientRect(),n=document.getElementById("flow-canvas-wrap").getBoundingClientRect();return{x:(t.left-n.left-h.x+t.width/2)/h.scale,y:(t.top-n.top-h.y+t.height/2)/h.scale}}window.getPortPos=X;function Pe(e,t,n,o,i,r=1){const a=document.getElementById("flow-svg-layer"),s=Nn(e,t,i);let d=a.querySelector(`path[id="${n}"]`),l=!1;d||(d=document.createElementNS("http://www.w3.org/2000/svg","path"),d.setAttribute("id",n),l=!0),d.setAttribute("d",s),d.setAttribute("data-wire-id",n);let c="flow-wire";if(o!==void 0&&(c+=` active-${o===1?"1":o===-1?"n1":"0"}`),i&&i.condition&&i.condition!=="all"&&(c+=" cond-"+i.condition.replace("!","")),n===H&&(c+=" selected-wire"),d.setAttribute("class",c),i&&i.customColor&&o===void 0?d.style.stroke=i.customColor:d.style.stroke="",d.style.opacity=.2+.8*r,d.style.strokeWidth=1.5+2.5*r,d.style.transition="opacity 0.3s, stroke-width 0.3s, stroke 0.3s",n==="active-wire"&&(d.style.pointerEvents="none"),l&&n!=="active-wire"&&i){d.style.pointerEvents="stroke",d.addEventListener("click",g=>{g.stopPropagation(),st(n),ct(n)});const u=document.createElementNS("http://www.w3.org/2000/svg","path");u.setAttribute("d",s),u.setAttribute("fill","none"),u.setAttribute("stroke","transparent"),u.setAttribute("stroke-width","18"),u.setAttribute("class","wire-hit"),u.id="hit-"+n,u.style.pointerEvents="stroke",u.addEventListener("click",g=>{g.stopPropagation(),st(n),ct(n)}),a.appendChild(u)}else if(!l&&i){const u=document.getElementById("hit-"+n);u&&u.setAttribute("d",s)}l&&a.appendChild(d);let p,m;if(i&&i.cp?(p=.25*e.x+.5*i.cp.x+.25*t.x,m=.25*e.y+.5*i.cp.y+.25*t.y):(p=(e.x+t.x)/2,m=(e.y+t.y)/2,t.x-e.x<60&&(p+=60)),i&&i.label){let u=document.getElementById("badge-"+n);if(!u){u=document.createElement("div"),u.id="badge-"+n;const g=document.getElementById("flow-canvas");g&&g.appendChild(u)}u.className="edge-badge"+(i.condition&&i.condition!=="all"?" cond-"+i.condition.replace("!",""):""),u.style.left=p+"px",u.style.top=m+"px",u.textContent=i.label,u.style.opacity=.4+.6*r}if(n===H){const u=document.getElementById("wire-handle");u.style.left=p+"px",u.style.top=m+"px"}}window.drawWire=Pe;function ct(e){const t=x.find(l=>l.id===e);if(!t)return;const n=document.getElementById(t.fromId),o=document.getElementById(t.toId);if(!n||!o)return;const i=X(n.querySelector(".flow-port-out")),r=X(o.querySelector(".flow-port-in"));t.cp||(t.cp={x:(i.x+r.x)/2,y:(i.y+r.y)/2});const a=document.getElementById("wire-handle"),s=.25*i.x+.5*t.cp.x+.25*r.x,d=.25*i.y+.5*t.cp.y+.25*r.y;a.style.left=s+"px",a.style.top=d+"px",a.classList.add("active"),Pn(a,(l,c)=>{t.cp.x=(l-.25*i.x-.25*r.x)/.5,t.cp.y=(c-.25*i.y-.25*r.y)/.5,A()})}window.showWireHandles=ct;function Pn(e,t){e.onmousedown=n=>{n.stopPropagation();const o=n.clientX,i=n.clientY,r=parseFloat(e.style.left),a=parseFloat(e.style.top),s=l=>{const c=(l.clientX-o)/h.scale,p=(l.clientY-i)/h.scale,m=r+c,u=a+p;t(m,u)},d=()=>{document.removeEventListener("mousemove",s),document.removeEventListener("mouseup",d),L()};document.addEventListener("mousemove",s),document.addEventListener("mouseup",d)}}window.setupHandleDrag=Pn;function ii(){if(f.length===0){E("No nodes to export","error");return}const e=[],t={},n={};f.forEach(c=>{t[c.id]=0,n[c.id]=[]}),x.forEach(c=>{n[c.fromId]&&t[c.toId]!==void 0&&(n[c.fromId].push(c.toId),t[c.toId]++)});const o=f.filter(c=>t[c.id]===0).map(c=>c.id);for(;o.length>0;){const c=o.shift(),p=f.find(m=>m.id===c);p&&e.push(p),(n[c]||[]).forEach(m=>{t[m]--,t[m]===0&&o.push(m)})}const i=f.filter(c=>!e.find(p=>p.id===c.id));e.push(...i);let r=`// Generated by TernFlow Orchestrator
`;r+=`// Swarm definition: Topological Order Optimized

`;const a=new Set;f.forEach(c=>{c.type==="agent"&&a.add(c.path)}),f.some(c=>c.type==="external")&&a.add("stdlib/agents/binary_bridge.tern"),a.forEach(c=>{c&&(r+=`// from "${c}" import *;
`)}),r+=`
fn main() -> trit {
`,e.forEach(c=>{const p=c.id.replace(/node_|bridge_|gate_/g,"a");c.type==="agent"?r+=`    let ${p}: agentref = spawn ${c.name};
`:c.type==="external"?r+=`    let ${p}: agentref = spawn LLMGateway;
`:r+=`    let ${p}: agentref = spawn TritVote;
`}),r+=`
    // Execution logic (Dependency Aware)
`;const s={};e.forEach(c=>{const p=c.id.replace(/node_|bridge_|gate_/g,"a"),m=x.filter(g=>g.toId===c.id);m.length===0?r+=`    send ${p} affirm;
`:m.forEach((g,v)=>{const w=s[g.fromId]||"affirm";r+=`    send ${p} ${w}; // from ${g.fromId}
`});const u=`res_${p}`;r+=`    let ${u}: trit = await ${p};
`,s[c.id]=u});const d=e[e.length-1],l=d?s[d.id]:"affirm";r+=`
    return ${l};
}
`,_t(),b&&b.setValue(r),G("editor"),E("Swarm exported to Editor (Topo-Sorted)","ok")}window.exportFlowCode=ii;function ri(){var t;if(f.length===0){E("Add agents to the canvas first","error");return}const e=((t=f[0])==null?void 0:t.name)||"";document.getElementById("deployName").value=e,document.getElementById("deployDesc").value="",document.getElementById("deployInput").value="",document.getElementById("deploy-progress").style.display="none",document.getElementById("deploy-confirm-btns").style.display="flex",document.getElementById("deploy-result").style.display="none",Ct(1),document.getElementById("deployModal").style.display="flex",lucide.createIcons()}window.openDeployModal=ri;function ai(){document.getElementById("deployModal").style.display="none"}window.closeDeployModal=ai;function Ct(e){[1,2,3].forEach(t=>{document.getElementById("deploy-step-"+t).style.display=t===e?"flex":"none";const n=document.getElementById("deploy-step-"+t+"-tab");n&&(n.style.cssText=t===e?"flex:1;padding:8px;text-align:center;font-size:10px;font-weight:700;background:rgba(1,118,211,0.2);color:var(--blue);border-right:"+(t<3?"1px solid var(--border2)":"none"):"flex:1;padding:8px;text-align:center;font-size:10px;font-weight:700;color:var(--muted2);border-right:"+(t<3?"1px solid var(--border2)":"none"))}),e===3&&Dn()}window.deployStep=Ct;function Dn(){var s;const e=document.getElementById("deployName").value.trim()||"my-agent",t=document.getElementById("deployDesc").value.trim()||"—",n=document.getElementById("deployInput").value.trim()||"{ signal: trit }",o=((s=document.querySelector('input[name="deployPricing"]:checked'))==null?void 0:s.value)||"free",i=e.toLowerCase().replace(/\s+/g,"-").replace(/[^a-z0-9-]/g,""),r=f.length,a=x.length;document.getElementById("deploy-summary").innerHTML=`
    <div style="color:var(--muted2);font-size:10px;margin-bottom:8px;letter-spacing:0.05em;">DEPLOYMENT PLAN</div>
    <div><span style="color:var(--muted)">name:    </span><span style="color:var(--cyan)">${e}</span></div>
    <div><span style="color:var(--muted)">slug:    </span><span style="color:var(--text)">/api/agent/${i}</span></div>
    <div><span style="color:var(--muted)">nodes:   </span><span style="color:var(--text)">${r} agents, ${a} wires</span></div>
    <div><span style="color:var(--muted)">pricing: </span><span style="color:${o==="free"?"var(--green)":o==="private"?"var(--muted)":"var(--amber)"}">${o}</span></div>
    <div><span style="color:var(--muted)">input:   </span><span style="color:var(--muted2);font-size:11px;">${n}</span></div>
    <div><span style="color:var(--muted)">desc:    </span><span style="color:var(--muted2);font-size:11px;">${t}</span></div>
  `}window.buildDeploySummary=Dn;async function si(){var l;Rn();const e=document.getElementById("deployName").value.trim(),t=document.getElementById("deployDesc").value.trim(),n=document.getElementById("deployInput").value.trim(),o=((l=document.querySelector('input[name="deployPricing"]:checked'))==null?void 0:l.value)||"free",i=e.toLowerCase().replace(/\s+/g,"-").replace(/[^a-z0-9-]/g,"");if(!e){E("Enter a product name","error"),Ct(1);return}document.getElementById("deploy-confirm-btns").style.display="none";const r=document.getElementById("deploy-progress");r.style.display="flex";const a=(c,p,m)=>{const u=document.getElementById(c);if(!u)return;u.className="deploy-step-row"+(p==="done"?" done":p==="error"?" error":"");const g=p==="done"?"✓":p==="error"?"✗":"⏳";u.innerHTML=`<span class="dstep-icon">${g}</span> ${m}`};await new Promise(c=>setTimeout(c,600)),a("dstep-compile","done","Flow compiled to .tern");let s=`// Deployed by TernStudio — ${e}
`;s+=`// Nodes: ${f.length} · Wires: ${x.length}

`,f.forEach(c=>{s+=`// Agent: ${c.name}
`,c.props.code&&(s+=c.props.code+`

`)}),await new Promise(c=>setTimeout(c,800));try{const c=document.getElementById("apiEndpoint").value.replace(/\/$/,""),p=document.getElementById("apiKey").value.trim(),m={name:e,slug:i,desc:t,input_schema:n,pricing:o,nodes:f.length,wires:x.length,code:s},u=await fetch(c+"/api/agents/publish",{method:"POST",headers:{"Content-Type":"application/json",...p?{"X-Ternlang-Key":p}:{}},body:JSON.stringify(m)}),g=await u.json();if(u.ok&&g.status==="ok"){a("dstep-register","done",`Endpoint registered: /api/agent/${i}`),await new Promise(I=>setTimeout(I,600)),a("dstep-publish","done","Live on runtime ✓");const v=document.getElementById("deploy-result");v.style.display="block",v.innerHTML=`
       <div style="font-size:13px;font-weight:700;color:var(--green);margin-bottom:8px;">🚀 Deployed successfully!</div>
       <div style="font-family:'JetBrains Mono',monospace;font-size:11px;color:var(--text);line-height:2;">
         <div>Endpoint: <span style="color:var(--cyan)">${c}/api/agent/${i}</span></div>
         <div>Pricing: <span style="color:var(--amber)">${o}</span></div>
         <div style="margin-top:8px;font-size:10px;color:var(--muted2);">Share your endpoint with consumers — they need an API key to call it.</div>
       </div>
      `;let w=[];try{w=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}w.find(I=>I.slug===i)||(w.push({id:i,slug:i,name:e,desc:t,pricing:o,nodes:f.length,deployed:new Date().toISOString()}),localStorage.setItem("ternflow_registry",JSON.stringify(w))),E(`Deployed: ${e}`,"ok")}else throw new Error(g.error||"Server error")}catch(c){console.warn("Deploy error, saving locally:",c),a("dstep-register","done","Registered locally"),await new Promise(u=>setTimeout(u,500)),a("dstep-publish","done","Saved to local fleet registry");const p=document.getElementById("deploy-result");p.style.display="block";const m=document.getElementById("apiEndpoint").value.replace(/\/$/,"");p.innerHTML=`
    <div style="font-size:13px;font-weight:700;color:var(--amber);margin-bottom:8px;">⚠ Saved locally</div>
    <div style="font-family:'JetBrains Mono',monospace;font-size:11px;color:var(--text);line-height:1.8;">
      Endpoint: <span style="color:var(--cyan)">${m}/api/agent/${i}</span><br>
      Status: <span style="color:var(--amber)">Local-Only (Sync Pending)</span>
    </div>
    `,E(`${e} saved to Fleet`,"ok")}let d=[];try{d=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}d.find(c=>c.slug===i)||(d.push({id:i,slug:i,name:e,desc:t,pricing:o,nodes:f.length,deployed:new Date().toISOString()}),localStorage.setItem("ternflow_registry",JSON.stringify(d)))}window.executeProductDeploy=si;function zn(e){if(e.button!==0)return;const t=e.target.closest(".flow-port");if(t){const n=t.closest(".flow-node"),o=document.getElementById("flow-canvas-wrap").getBoundingClientRect();_={fromId:n.id,fromIsOutput:t.classList.contains("flow-port-out"),start:X(t),end:He(e.clientX-o.left,e.clientY-o.top)}}else _=null}document.addEventListener("mousedown",zn);window.onMouseDown=zn;function On(e){const t=document.getElementById("flow-canvas-wrap").getBoundingClientRect();if(_){_.end=He(e.clientX-t.left,e.clientY-t.top),document.querySelectorAll(".flow-port-in").forEach(i=>i.classList.remove("magnet"));let n=null,o=40;document.querySelectorAll(".flow-port-in").forEach(i=>{const r=i.getBoundingClientRect(),a=Math.sqrt(Math.pow(e.clientX-(r.left+r.width/2),2)+Math.pow(e.clientY-(r.top+r.height/2),2));a<o&&(o=a,n=i)}),n&&n.classList.add("magnet"),A()}else if(fe&&q){const n=(e.clientX-cn)/h.scale,o=(e.clientY-pn)/h.scale;if(k.size>1)k.forEach(i=>{const r=document.getElementById(i),a=Me[i];r&&a&&(r.style.left=a.x+n+"px",r.style.top=a.y+o+"px")});else{const i=document.getElementById(q),r=Me[q];i&&r&&(i.style.left=r.x+n+"px",i.style.top=r.y+o+"px")}A()}}document.addEventListener("mousemove",On);window.onMouseMove=On;function jn(e){if(fe){if(k.size>1)k.forEach(t=>{const n=f.find(i=>i.id===t),o=document.getElementById(t);n&&o&&(n.x=parseFloat(o.style.left)+(parseFloat(o.style.width)||180)/2,n.y=parseFloat(o.style.top)+(parseFloat(o.style.height)||80)/2)});else if(q){const t=f.find(o=>o.id===q),n=document.getElementById(q);if(t&&n){const o=t.type==="artifact"?300:t.type==="moe13"?320:180,i=t.type==="artifact"?200:t.type==="moe13"?360:80;t.x=parseFloat(n.style.left)+o/2,t.y=parseFloat(n.style.top)+i/2}}if(q){const t=document.getElementById(q);t&&(t.style.zIndex=10)}fe=!1,q=null,L()}if(_)try{const n=document.querySelector(".flow-port.magnet")||e.target.closest(".flow-port");if(n){const o=n.closest(".flow-node");if(o&&o.id!==_.fromId){const i=n.classList.contains("flow-port-out");if(_.fromIsOutput!==i){const r="wire_"+Date.now(),a=_.fromIsOutput?_.fromId:o.id,s=_.fromIsOutput?o.id:_.fromId;x.push({id:r,fromId:a,toId:s,signal:0,confidence:1,condition:"all",transform:"pass",priority:5,label:"All signals"}),L();const d=f.find(l=>l.id===a);d&&d.type==="artifact"&&mn(a),N&&!se&&(T("SYSTEM","🔌 Continuation Detected — Resuming Engine."),Fn(s))}}}else if(_.fromIsOutput){const o=f.find(s=>s.id===_.fromId),i=document.getElementById("flow-canvas-wrap").getBoundingClientRect(),r=He(e.clientX-i.left,e.clientY-i.top),a=Math.sqrt(Math.pow(r.x-_.start.x,2)+Math.pow(r.y-_.start.y,2));if(o&&o.type==="artifact"&&o.props.state==="extend"&&a>40){const s=_.end.x,d=_.end.y,l="node_"+Date.now(),c=document.getElementById(`art-body-${o.id}`),p=c?c.textContent:"";F("Transmuted Agent","__custom__",s,d,"agent",l);const m=f.find(u=>u.id===l);if(m){m.props.code=`// Inherited Payload:
/*
${p}
*/
return truth();`;const u="wire_evo_"+Date.now();x.push({id:u,fromId:o.id,toId:l,signal:0,confidence:1,condition:"all",transform:"pass",priority:5,label:"EVOLUTION"}),Mn(o.id,"lock"),A(),L(),T("SYSTEM","🧬 Evolution Triggered — New node ready.")}}}}catch(t){console.error("Critical error in mouseup:",t)}finally{document.querySelectorAll(".flow-port").forEach(t=>t.classList.remove("magnet")),_=null,fe=!1,q=null,A()}}document.addEventListener("mouseup",jn);window.onMouseUp=jn;async function Fn(e){N=!1,se=!0,Fe(),ie.push({toId:e,val:1,conf:1,origin:"CONTINUATION"}),await kt()}window.resumeSimulationFrom=Fn;const li="https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/",qe=li+"ternlang-root/",Wt="https://api.github.com/repos/eriirfos-eng/ternary-intelligence-stack/contents/ternlang-root/",di=["core","ternary","std","showcase","bughunt","testing","bench","benchmarks","classical","errors","tutorials","lib"];async function _e(){const e=document.getElementById("file-tree");e.innerHTML='<div style="padding:10px; color:var(--muted); font-size:11px;">Loading from GitHub…</div>';try{const t=[];for(const n of di)try{const o=await fetch(Wt+`stdlib/${n}`);if(!o.ok)continue;const i=await o.json();Array.isArray(i)&&i.filter(r=>r.name.endsWith(".tern")).forEach(r=>t.push(`stdlib/${n}/${r.name}`))}catch{}try{const n=await fetch(Wt+"examples");if(n.ok){const o=await n.json();Array.isArray(o)&&o.filter(i=>i.name.endsWith(".tern")).forEach(i=>t.push(`examples/${i.name}`))}}catch{}if(t.length===0){De(e);return}Ge(e,t,!0)}catch{De(e)}}window.loadGithubTree=_e;function De(e){Hn(e)}window.showNoKeyMessage=De;function Hn(e){e.innerHTML="";const t=document.createElement("div");t.style.cssText="padding:6px 12px; font-size:10px; color:var(--muted2); background:rgba(255,255,255,0.02); border-bottom:1px solid var(--border); display:flex; justify-content:space-between; align-items:center;",t.innerHTML='<span>Built-in Examples</span><span style="cursor:pointer;color:var(--cyan)" onclick="toggleKeyInput()">+ Add key</span>',e.appendChild(t),[{name:"hello_trit.tern",path:"examples/hello_trit.tern",key:"hello"},{name:"consensus.tern",path:"examples/consensus.tern",key:"consensus"},{name:"match_signal.tern",path:"examples/match_signal.tern",key:"match"},{name:"trit_gate.tern",path:"examples/trit_gate.tern",key:"gate"},{name:"agent_basic.tern",path:"examples/agent_basic.tern",key:"agent"}].forEach(({name:o,path:i,key:r})=>{const a=document.createElement("div");a.className="tree-file",a.dataset.path=i,a.dataset.builtin="1",a.innerHTML=`<i data-lucide="file-code" style="width:12px;height:12px"></i> ${o}`,a.onclick=()=>{B[i]=B[i]||ae[r]||ae.hello,V(i,B[i]),G("editor")},e.appendChild(a)}),e.dataset.loaded="true",K(),lucide.createIcons()}window.renderBuiltinTree=Hn;function Ge(e,t,n=!1,o=!1){const i={};if(t.forEach(r=>{const a=r.split("/");if(a.length<2)return;const s=a[1];i[s]||(i[s]=[]),i[s].push(r)}),Object.keys(i).length===0){o||De(e);return}if(e.innerHTML="",n){const r=document.createElement("div");r.style.cssText="padding:6px 12px; font-size:10px; color:var(--amber); background:rgba(245,158,11,0.08); border-bottom:1px solid var(--border);",r.innerHTML='Tier 1 — GitHub · <span style="cursor:pointer;color:var(--cyan)" onclick="toggleKeyInput()">Enter key for full access</span>',e.appendChild(r)}Object.keys(i).sort().forEach(r=>{const a=document.createElement("div");a.className="tree-section";const s=document.createElement("div");s.className="tree-dir collapsed",s.innerHTML=`<span class="arrow">▸</span> <i data-lucide="folder" style="width:12px; height:12px"></i> ${r}/`;const d=document.createElement("div");d.className="tree-files hidden",s.onclick=()=>{const l=d.classList.toggle("hidden");s.classList.toggle("collapsed",l),s.querySelector(".arrow").textContent=l?"▸":"▾",l?d.innerHTML="":(i[r].forEach(c=>{const p=c.split("/").pop(),m=document.createElement("div");m.className="tree-file",m.dataset.path=c,m.dataset.github=n?"1":"",m.dataset.premium=o?"1":"",m.innerHTML=`<i data-lucide="file-code" style="width:12px; height:12px"></i> ${p}`,m.onclick=()=>{Wn(c,n,o),G("editor")},d.appendChild(m)}),K(),lucide.createIcons({root:d}))},a.appendChild(s),a.appendChild(d),e.appendChild(a)}),e.dataset.loaded="true",K(),lucide.createIcons({root:e})}window.renderFileTree=Ge;async function ze(e=!1){const t=document.getElementById("file-tree");if(!e&&t.dataset.loaded==="true"){K();return}t.innerHTML='<div style="padding:10px; color:var(--muted); font-size:11px;">Loading library…</div>';try{const n=document.getElementById("apiEndpoint").value.replace(/\/$/,""),o=document.getElementById("apiKey").value.trim(),r=await(await fetch(n+"/api/stdlib/list",{headers:o?{"X-Ternlang-Key":o}:{}})).json();if(r.status!=="ok"){await _e();return}if(!r.files||r.files.length===0){await _e();return}Ge(t,r.files,!1),t.dataset.loaded="true",K(),lucide.createIcons()}catch{await _e()}}window.buildFileTree=ze;function K(){document.querySelectorAll(".tree-file").forEach(e=>{e.classList.toggle("active",e.dataset.path===O)})}window.refreshTreeHighlight=K;function ne(){const e=document.getElementById("editorTabs");e.innerHTML="",j.forEach(({name:t,path:n})=>{const o=document.createElement("div");o.className="tab"+(n===O?" active":""),o.innerHTML=`<span style="flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;">${t}</span><button class="tab-close" onclick="closeTab('${CSS.escape(n)}',event)" title="Close">✕</button>`,o.onclick=()=>Ue(n),e.appendChild(o)})}window.renderTabs=ne;function ci(e,t){t&&t.stopPropagation();const n=j.findIndex(o=>o.path===e);if(n!==-1){if(j.splice(n,1),j.length===0){_t();return}if(O===e){const o=j[Math.min(n,j.length-1)];Ue(o.path)}else ne()}}window.closeTab=ci;function pi(){document.getElementById("localFileInput").click()}window.triggerImportFile=pi;function ui(e){const t=e.target.files[0];if(!t)return;const n=new FileReader;n.onload=o=>{const i=o.target.result,r="local/"+t.name;B[r]=i,V(r,i),G("editor"),E(`Imported ${t.name}`,"ok")},n.readAsText(t),e.target.value=""}window.importLocalFile=ui;function Ue(e){b&&(B[O]=b.getValue()),O=e,b&&b.setValue(B[e]||""),ne(),K(),be();const t=document.getElementById("sbFile");t&&(t.textContent=e.split("/").pop())}window.switchToTab=Ue;async function Wn(e,t=!1,n=!1){if(b&&(B[O]=b.getValue()),B[e]){V(e,B[e]);return}const o=document.querySelector(`.tree-file[data-path="${CSS.escape(e)}"]`),i=n||o&&o.dataset.premium==="1",r=t||o&&o.dataset.github==="1";if(i)try{const a=document.getElementById("apiEndpoint").value.replace(/\/$/,""),s=await fetch(`${a}/api/premium/file?path=${encodeURIComponent(e)}`,{headers:{"X-Ternlang-Key":localStorage.getItem("ternstudio-key")||""}});if(!s.ok)throw new Error(`Auth failed or file not found (${s.status})`);const d=await s.json();if(d.content)B[e]=d.content,V(e,d.content);else throw new Error("Invalid response from server");return}catch(a){E(`Failed to load premium file: ${a.message}`,"err");return}if(r){try{const a=await fetch(qe+e);if(a.ok){const s=await a.text();B[e]=s,V(e,s);return}}catch{}E("Failed to load from GitHub","err");return}try{const a=document.getElementById("apiEndpoint").value.replace(/\/$/,""),s=document.getElementById("apiKey").value.trim(),l=await(await fetch(a+"/api/stdlib/read/"+e,{headers:s?{"X-Ternlang-Key":s}:{}})).json();l.status==="ok"?(B[e]=l.content,V(e,l.content)):E(l.error||"Failed to read file","err")}catch{E("Connection Error","err")}}window.openFile=Wn;function V(e,t){if(j.find(i=>i.path===e)||j.push({name:e.split("/").pop(),path:e}),O=e,b){b.setValue(t);const i=b.getModel();e.split(".").pop()==="tern"&&monaco.editor.setModelLanguage(i,"ternlang")}ne(),K(),be();const o=document.getElementById("sbFile");o&&(o.textContent=e.split("/").pop())}window.loadToEditor=V;function _t(){const e=`scratch_${ao++}.tern`,t=`scratch/${e}`;B[t]=ae.hello,j.push({name:e,path:t}),Ue(t),be()}window.newFile=_t;function mi(e){b&&b.setValue(ae[e]||"")}window.insertTemplate=mi;function fi(e){document.querySelectorAll(".sidebar-panel").forEach(t=>t.classList.remove("active")),document.querySelectorAll(".act-btn").forEach(t=>t.classList.remove("active")),document.getElementById("panel-"+e).classList.add("active"),document.getElementById("act-"+e).classList.add("active")}window.switchSidebarPanel=fi;function gi(e){e.preventDefault();const t=document.getElementById("editor-sidebar"),n=e.clientX,o=t.offsetWidth,i=a=>{const s=Math.max(120,Math.min(400,o+a.clientX-n));document.documentElement.style.setProperty("--sidebar-w",s+"px"),b&&b.layout()},r=()=>{document.removeEventListener("mousemove",i),document.removeEventListener("mouseup",r)};document.addEventListener("mousemove",i),document.addEventListener("mouseup",r)}window.startSidebarResize=gi;function yi(e){e.preventDefault();const t=document.getElementById("output-panel"),n=e.clientY,o=t.offsetHeight,i=a=>{const s=Math.max(80,Math.min(window.innerHeight*.6,o+n-a.clientY));document.documentElement.style.setProperty("--output-h",s+"px"),b&&b.layout()},r=()=>{document.removeEventListener("mousemove",i),document.removeEventListener("mouseup",r)};document.addEventListener("mousemove",i),document.addEventListener("mouseup",r)}window.startOutputResize=yi;function hi(e,t){document.querySelectorAll(".out-tab").forEach(n=>n.classList.remove("active")),document.querySelectorAll(".out-panel").forEach(n=>n.classList.remove("visible")),t.classList.add("active"),document.getElementById("panel-"+e).classList.add("visible"),e==="api"&&(document.getElementById("apiEndpointBase").textContent=document.getElementById("apiEndpoint").value)}window.switchOutTab=hi;function Ke(e,t){const n=document.getElementById("vmStatus");n&&(n.className="status-pill status-"+e,n.textContent=t);const o=document.getElementById("connDot");o&&(o.className="sb-dot"+(e==="error"?" err":e==="running"?" warn":""))}window.setStatus=Ke;function vi(){Ke("idle","● Idle"),document.getElementById("printOutput").textContent="— no output —",document.getElementById("printOutput").className="print-output empty",document.getElementById("section-meta").style.display="none",document.getElementById("section-regs").style.display="none",document.getElementById("section-error").style.display="none"}window.clearOutput=vi;window.addEventListener("wasmready",()=>{const e=document.getElementById("wasmBadge");e&&(e.style.opacity="1");const t=document.getElementById("sbWasmStatus");t&&(t.textContent="⚡ WASM",t.style.color="var(--green)")});function Bt(e){const t=e.status==="ok";Ke(t?"ok":"error",t?e._wasm?"⚡ OK (WASM)":"✓ OK":"✕ Error"),document.getElementById("section-meta").style.display="block";const n=e._wasm?e._ms!=null?`${e._ms}ms WASM`:"WASM":e.bytecode_bytes!=null?e.bytecode_bytes+"B":"—";document.getElementById("metaBytes").textContent=n,document.getElementById("metaStatus").textContent=t?"exited ok":"vm error",document.getElementById("metaStatus").style.color=t?"var(--green)":"var(--red)";const o=document.getElementById("printOutput");o.innerHTML="",o.className="print-output";const i=document.createElement("div");if(i.className="term-header",i.textContent=`Ternary Intelligence Stack — BET-VM v1.0.0 (${e._wasm?"WASM":"API"})`,o.appendChild(i),e.output&&e.output.length>0&&e.output.forEach(a=>{const s=document.createElement("div");s.className="term-line",s.innerHTML=`<span class="term-prompt">></span><span>${a}</span>`,o.appendChild(s)}),t){const a=document.createElement("div");a.className="term-line term-success",a.style.marginTop="8px",a.textContent=`● Program exited successfully. [trit:${e.trit}]`,o.appendChild(a)}else{const a=document.createElement("div");a.className="term-err-line";let s=e.error||"Unknown runtime error";if(s.includes("ExpectedToken")){const l=s.match(/ExpectedToken\("(.*?)",\s+"(.*?)"\)/);l&&(s=`Syntax Error: Expected ${l[1]}, but found ${l[2]}. Check your semicolons!`)}else if(s.includes("UndefinedSymbol")){const l=s.match(/UndefinedSymbol\("(.*?)"\)/);l&&(s=`Reference Error: Symbol '${l[1]}' is not defined.`)}a.innerHTML=`<strong>VM_ERROR:</strong> ${s}`,o.appendChild(a);const d=document.createElement("div");d.className="term-line term-dim",d.textContent="● Process terminated with non-zero exit code.",o.appendChild(d)}if(e.registers&&e.registers.length>0){document.getElementById("section-regs").style.display="block";const a=document.getElementById("regTable");a.innerHTML="",e.registers.forEach((s,d)=>{const l=document.createElement("tr"),c=String(s);let p="";c.includes("Affirm")||c.includes("Truth")?p="reg-row-affirm":c.includes("Reject")||c.includes("Conflict")?p="reg-row-reject":(c.includes("Tend")||c==="Trit(Tend)")&&(p="reg-row-zero"),l.className=p,l.innerHTML=`<td>r${d}</td><td>${c}</td>`,a.appendChild(l)})}const r=document.getElementById("section-error");!t&&e.error?(r.style.display="block",document.getElementById("errorOutput").textContent=e.error):r.style.display="none",Lt(e.registers||[])}window.showResult=Bt;function Lt(e=[]){const t=document.getElementById("logic-field-grid");if(!t)return;t.innerHTML="";const n=Math.max(27,Math.min(64,e.length||0));for(let o=0;o<n;o++){const i=document.createElement("div");if(i.className="trit-cell",i.title=`r${o}`,e[o]!=null){const r=String(e[o]);r.includes("Affirm")||r.includes("Truth")?(i.classList.add("active-affirm"),i.textContent="+"):r.includes("Reject")||r.includes("Conflict")?(i.classList.add("active-reject"),i.textContent="-"):(i.classList.add("active-tend"),i.textContent="0")}else i.textContent=".",i.style.opacity="0.3";t.appendChild(i)}}window.renderLogicField=Lt;function qn(e){if(e=e.trim(),!e)return e;const t=/fn\s+main\s*\(/.test(e),n=/^fn\s+\w+/m.test(e);return t?e:n?e+`

fn main() -> trit { return hold; }`:`fn main() -> trit {
${e.split(`
`).map(i=>{let r=i.trim();return r&&!r.endsWith(";")&&!r.endsWith("}")&&!r.endsWith("{")&&!r.startsWith("//")?i+";":i}).join(`
`)}
    return hold;
}`}window.prepareTernCode=qn;function Ve(e){if(!window.wasmReady||!window.wasmRunTern)return{ok:!1,error:"BET-VM (WASM) not loaded. Check network."};const t=qn(e);try{const n=window.wasmRunTern(t),o=JSON.parse(n);return{ok:o.ok,output:o.output||[],trit:o.trit??0,label:o.label||"hold",registers:o.registers||[],error:o.error||null,cycles:o.cycles||0}}catch(n){const o={type:"WASM_PANIC",error:n.message,stack:n.stack,payload_len:t.length,timestamp:new Date().toISOString()};return console.error("🛑 TERNLANG_CRITICAL_DEBUG:",o),{ok:!1,error:"VM_PANIC: "+n.message,traceback:n.stack}}}window.runTernCode=Ve;async function pt(){if(!b)return;B[O]=b.getValue();const e=b.getValue();if(!e.trim())return;const t=Date.now(),n=document.getElementById("runBtn");n.classList.add("running"),n.textContent="● Running…",Ke("running","● Running…"),document.getElementById("view-editor").classList.contains("active")||G("editor"),document.querySelectorAll(".out-tab").forEach(a=>a.classList.remove("active")),document.querySelectorAll(".out-panel").forEach(a=>a.classList.remove("visible")),document.querySelector(".out-tab").classList.add("active"),document.getElementById("panel-output").classList.add("visible");const o=Ve(e),i=Date.now()-t,r={status:o.ok?"ok":"error",output:o.output,trit:o.trit,label:o.label,registers:o.registers,error:o.error,bytecode_bytes:o.cycles,_ms:i,_wasm:!0};Bt(r),Gn(O,o.ok,i,r),n.classList.remove("running"),n.innerHTML='<span>▶ Run</span><span class="kbd">Ctrl+↵</span>'}window.runCode=pt;function Gn(e,t,n,o){Pt++,t?Dt++:zt++,document.getElementById("statRuns").textContent=Pt,document.getElementById("statOk").textContent=Dt,document.getElementById("statErr").textContent=zt;const i={path:e,ok:t,ms:n,ts:new Date().toLocaleTimeString(),code:b?b.getValue():"",data:o};Y.unshift(i),Y.length>20&&Y.pop(),$t(),Un()}window.addRunToHistory=Gn;function $t(){const e=document.getElementById("history-list");if(Y.length===0){e.innerHTML='<div style="padding:12px; font-size:11px; color:var(--muted); font-style:italic;">No runs yet</div>';return}e.innerHTML=Y.map((t,n)=>`
    <div class="hist-item" onclick="restoreRun(${n})">
      <div class="hist-name">${t.path.split("/").pop()}</div>
      <div class="hist-meta">
        <span class="${t.ok?"hist-ok":"hist-err"}">${t.ok?"✓ ok":"✕ err"}</span>
        <span>${t.ts}</span>
        <span>${t.ms}ms</span>
      </div>
    </div>`).join("")}window.renderHistory=$t;function xi(e){const t=Y[e];t&&(j.find(n=>n.path===t.path)||j.push({name:t.path.split("/").pop(),path:t.path}),O=t.path,B[t.path]=t.code,b&&b.setValue(t.code),ne(),K(),Bt(t.data),G("editor"))}window.restoreRun=xi;function wi(){Y=[],$t()}window.clearHistory=wi;function Un(){const e=document.getElementById("dashRunList");if(Y.length===0){e.innerHTML='<div class="dash-run-empty">No runs yet this session</div>';return}e.innerHTML=Y.slice(0,5).map(t=>`
    <div class="dash-run-item">
      <span class="run-status ${t.ok?"run-ok":"run-err"}">${t.ok?"✓":"✕"}</span>
      <span class="run-name">${t.path.split("/").pop()}</span>
      <span style="font-size:10px; color:var(--muted);">${t.ts}</span>
    </div>`).join("")}window.updateDashboardRuns=Un;function bi(){if(!b)return;const e=b.getValue(),t="#code="+btoa(encodeURIComponent(e)),n=location.origin+location.pathname+t;navigator.clipboard.writeText(n).then(()=>E("Share URL copied to clipboard","ok")).catch(()=>E("Copy failed — check browser permissions","err"))}window.shareCode=bi;function Ei(){if(!b)return;const e=b.getValue(),t=O.split("/").pop()||"scratch.tern",n=new Blob([e],{type:"text/plain"}),o=document.createElement("a");o.href=URL.createObjectURL(n),o.download=t,o.click(),URL.revokeObjectURL(o.href),E("Downloaded "+t,"ok")}window.downloadCode=Ei;function E(e,t=""){const n=document.getElementById("toast-container"),o=document.createElement("div");o.className="toast"+(t?" "+t:""),o.textContent=e,n.appendChild(o),setTimeout(()=>o.remove(),3e3)}window.showToast=E;function Kn(){const e=location.hash;if(e.startsWith("#code="))try{const t=decodeURIComponent(atob(e.slice(6))),n="scratch/shared.tern";B[n]=t,j.find(o=>o.path===n)||j.push({name:"shared.tern",path:n}),O=n,b&&b.setValue(t),ne(),K(),document.getElementById("sbFile").textContent="shared.tern",G("editor"),E("Loaded shared code","ok")}catch{E("Failed to decode shared URL","err")}}window.loadFromHash=Kn;async function Ii(){const e=document.getElementById("apiEndpoint").value.replace(/\/$/,"");try{const n=await(await fetch(e+"/health")).json();document.getElementById("apiResponse").textContent=JSON.stringify(n,null,2)}catch(t){document.getElementById("apiResponse").textContent=String(t)}}window.tryHealth=Ii;async function Si(){const e=document.getElementById("apiEndpoint").value.replace(/\/$/,""),t=document.getElementById("apiKey").value.trim();try{const o=await(await fetch(e+"/api/usage",{headers:t?{"X-Ternlang-Key":t}:{}})).json();document.getElementById("apiResponse").textContent=JSON.stringify(o,null,2)}catch(n){document.getElementById("apiResponse").textContent=String(n)}}window.tryApiUsage=Si;async function ki(){const e=document.getElementById("apiEndpoint").value.replace(/\/$/,""),t=document.getElementById("apiKey").value.trim(),o={code:b?b.getValue():""};t&&(o.key=t);try{const r=await(await fetch(e+"/api/run",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(o)})).json();document.getElementById("apiResponse").textContent=JSON.stringify(r,null,2)}catch(i){document.getElementById("apiResponse").textContent=String(i)}}window.tryApiRun=ki;const Ti={hello:`fn main() -> trit {
    print("@ Hello Ternary World");
    return affirm;
}`,consensus:`fn main() -> trit {
    let a: trit = affirm;
    let b: trit = tend;
    let c: trit = affirm;
    // Majority: 2 affirm, 1 tend → affirm
    return a;
}`,match:`fn main() -> trit {
    let sig: trit = tend;
    match sig {
        affirm => { print("YES"); }
        tend   => { print("HOLD — deliberate"); }
        reject => { print("NO"); }
    }
    return sig;
}`,dec13:`fn main() -> trit {
    // 13 in balanced ternary = +-+  (+9 -3 +1)
    let val: int = 13;
    print("13 = +-+ (balanced ternary)");
    return affirm;
}`};function Ci(e){document.getElementById("dashReplInput").value=Ti[e]||""}window.setReplSnippet=Ci;function _i(){const e=document.getElementById("dashReplInput").value.trim();if(!e)return;const t="playground/repl.tern";B[t]=e,V(t,e),G("editor")}window.openReplInEditor=_i;async function Bi(){const e=document.getElementById("dashReplInput").value.trim(),t=document.getElementById("dashReplRes");if(!e)return;t.style.color="var(--muted)",t.textContent="running…";const n=Date.now(),o=Ve(e),i=Date.now()-n;if(o.ok){const r=o.trit,a=o.label||(r===1?"affirm":r===-1?"reject":"tend"),s=r===1?"var(--green)":r===-1?"var(--red)":"var(--amber)",d=r===1?"+1":r===-1?"-1":"0";t.style.color=s;let l=`${d}  ${a.toUpperCase()}  [${i}ms WASM]`;o.output&&o.output.length&&(l+=`
`+o.output.join(`
`)),t.textContent=l}else t.style.color="var(--red)",t.textContent=o.error||"Error"}window.runReplExpr=Bi;function Vn(){const e=localStorage.getItem("ternstudio-key"),t=localStorage.getItem("ternstudio-save-key")==="1";e&&t?(document.getElementById("apiKey").value=e,document.getElementById("saveKeyCheck").checked=!0):document.getElementById("saveKeyCheck").checked=t}window.initKeyPersistence=Vn;function Li(){const e=document.getElementById("saveKeyCheck").checked;if(localStorage.setItem("ternstudio-save-key",e?"1":"0"),e){let t=(document.getElementById("settingsNewKey")||{}).value||"";t.trim()||(t=document.getElementById("apiKey").value.trim()),t&&(document.getElementById("apiKey").value=t.trim(),localStorage.setItem("ternstudio-key",t.trim()),Ye())}else localStorage.removeItem("ternstudio-key")}window.toggleSaveKey=Li;function $i(){const e=(document.getElementById("settingsNewKey")||{}).value||"";e.trim()&&(Jt(e.trim()),document.getElementById("settingsNewKey").value="")}window.applySettingsKey=$i;function Ye(){const e=(document.getElementById("apiKey")||{value:""}).value.trim(),t=e.length>12?e.slice(0,8)+"…"+e.slice(-4):e||"—",n=document.getElementById("settingsKeyDisplay");n&&(n.textContent=t)}window.syncSettingsKeyDisplay=Ye;function Ai(){const e=document.getElementById("apiKey").value.trim();e&&navigator.clipboard.writeText(e).then(()=>E("Key copied","ok"))}window.copyKey=Ai;function Mi(){document.getElementById("apiKey").value="",localStorage.removeItem("ternstudio-key"),Xe(),Ye(),E("Key cleared","ok")}window.clearKey=Mi;function Ri(){const e=document.getElementById("settingsEndpoint").value.trim();e&&(document.getElementById("apiEndpoint").value=e,je(),E("Endpoint updated","ok"))}window.applyEndpoint=Ri;function Yn(){document.getElementById("saveKeyCheck").checked=localStorage.getItem("ternstudio-save-key")==="1",Ye();const e=document.documentElement.getAttribute("data-theme")||"dark";document.getElementById("settingsTheme").value=e,Mt()}window.syncSettingsUI=Yn;function Ni(){const e=parseInt(document.getElementById("settingsFontSize").value),t=document.getElementById("settingsMinimap").value==="true",n=document.getElementById("settingsWordWrap").value;document.documentElement.style.fontSize=e+"px",b&&b.updateOptions({fontSize:e,minimap:{enabled:t},wordWrap:n})}window.applyEditorSettings=Ni;function Pi(){const e=document.getElementById("settingsTheme").value;localStorage.setItem("ternstudio-theme",e),Ee(e),b&&monaco.editor.setTheme(e==="light"?"ternstudio-light":"ternstudio-dark")}window.applyThemeFromSettings=Pi;function Je(){try{return JSON.parse(localStorage.getItem("ternflow_secrets")||"{}")}catch{return{}}}function At(e,t){const n=Je();t?n[e]=t:delete n[e],localStorage.setItem("ternflow_secrets",JSON.stringify(n)),Mt()}window.setTernflowSecret=At;function Di(){const e=document.getElementById("newSecretProvider").value,t=document.getElementById("newSecretKey").value.trim();t&&(At(e,t),document.getElementById("newSecretKey").value="",E(`Secret for ${e} updated`,"ok"))}window.addVaultSecret=Di;function Mt(){const e=document.getElementById("secretsVaultList");if(!e)return;const t=Je(),n=["openai","anthropic","google","custom"];e.innerHTML=n.map(o=>{const i=t[o]||"",r=i?i.slice(0,8)+"…"+i.slice(-4):"Not set";return`
      <div style="display:grid; grid-template-columns:140px 1fr auto; gap:12px; align-items:center; padding-bottom:12px; border-bottom:1px solid var(--border2);">
        <div style="font-size:12px; font-weight:600; color:var(--text); text-transform:capitalize;">${o}</div>
        <div style="font-family:'JetBrains Mono',monospace; font-size:11px; color:${i?"var(--cyan)":"var(--muted2)"};">${r}</div>
        <button class="settings-btn" onclick="setTernflowSecret('${o}', '')" style="color:var(--red); border-color:rgba(239,68,68,0.2); transition:background 0.2s;" onmouseover="this.style.background='rgba(239,68,68,0.1)'" onmouseout="this.style.background='transparent'">Clear</button>
      </div>
    `}).join("")}window.renderVaultUI=Mt;const qt={1:["● /api/run — execute .tern programs","● /health, /studio","○ Trit decisions + vectors (Tier 2+)","○ MoE-13 orchestrator (Tier 2+)","○ Deliberation engine (Tier 2+)"],2:["● /api/run","● /api/trit_decide, /api/trit_vector","● /api/trit_consensus, /api/trit_gate","● /api/moe/orchestrate","● Deliberation + coalition engine","○ Industrial endpoints (Tier 3+)"],3:["● All Tier 2 endpoints","● /api/v1/taas/infer","● /api/stream/* SSE endpoints","● Sparse benchmark + quantization","○ Enterprise SLA (Tier 4)"],4:["● All endpoints, unlimited","● Enterprise SLA + dedicated support","● Custom rate limits","● Private deployment options"]};function Xe(){document.getElementById("usageKeyDisplay").textContent="No key — anonymous access",document.getElementById("usageTierBadge").className="tb-badge badge-free",document.getElementById("usageTierBadge").textContent="Tier 1",document.getElementById("usage-quota-section").style.display="none",document.getElementById("usage-unlimited-section").style.display="none",document.getElementById("usageError").style.display="none",Rt(1),Nt(1)}window.renderUsageAnon=Xe;function Rt(e){const t=document.getElementById("tierBenefitsList"),n=qt[e]||qt[1];t.innerHTML=n.map((i,r)=>`<div class="tier-benefit ${i.startsWith("○")?"muted-benefit":""}">${i}</div>`).join("");const o=document.getElementById("upgradeBtn");o.style.display=e>=4?"none":"inline-block"}window.renderTierBenefits=Rt;function Nt(e){const t=document.getElementById("tierBadge");t&&(t.textContent=Te[e]||"Tier 1",t.className="tier-badge tb-badge "+(nt[e]||"badge-free"));const n=document.getElementById("sbTier");n&&(n.textContent=Te[e]||"Tier 1 — Open Core");const o=document.getElementById("topbarUpskillBtn");o&&(o.style.display=e<=1?"flex":"none");const i=document.getElementById("dashTierBadge");i&&(i.textContent=Te[e]||"Tier 1 — Free",i.className="tb-badge "+(nt[e]||"badge-free"))}window.updateTopbarTier=Nt;async function Oe(){const e=document.getElementById("apiEndpoint").value.replace(/\/$/,""),t=document.getElementById("apiKey").value.trim();if(!t){Xe();return}try{const n=await fetch(e+"/api/usage",{headers:{"X-Ternlang-Key":t},signal:AbortSignal.timeout(5e3)});if(!n.ok){const s=await n.text();document.getElementById("usageError").style.display="block",document.getElementById("usageErrorMsg").textContent=`HTTP ${n.status} — ${s.slice(0,200)}`;return}const o=await n.json();document.getElementById("usageError").style.display="none";const i=t.length>12?t.slice(0,8)+"…"+t.slice(-4):t;document.getElementById("topbarKeyInput").value=t;const r=o.tier||1,a=document.getElementById("usageTierBadge").dataset.tier;if(a&&a!=r&&(document.getElementById("file-tree").dataset.loaded="false",xe(),ze()),document.getElementById("usageTierBadge").dataset.tier=r,document.getElementById("usageKeyDisplay").textContent=i,Nt(r),document.getElementById("usageTierBadge").textContent=Te[r]||"Tier 1",document.getElementById("usageTierBadge").className="tb-badge "+(nt[r]||"badge-free"),Rt(r),ze(!0),xe(),o.limit===null||o.limit===void 0||r>=4)document.getElementById("usage-quota-section").style.display="none",document.getElementById("usage-unlimited-section").style.display="block",document.getElementById("dash-unlimited-badge").style.display="block",document.getElementById("dash-quota-wrap").style.display="none";else{document.getElementById("usage-unlimited-section").style.display="none",document.getElementById("usage-quota-section").style.display="block";const s=o.calls_this_month||0,d=o.limit||0,l=d>0?Math.min(100,Math.round(s/d*100)):0,c=Math.max(0,d-s);document.getElementById("usageUsed").textContent=s.toLocaleString(),document.getElementById("usageLimit").textContent=d.toLocaleString();const p=document.getElementById("usageBarFill");p.style.width=l+"%",p.className="usage-bar-fill "+(l>=90?"crit":l>=70?"warn":"ok"),document.getElementById("usageMeta").textContent=`${l}% used · ${c.toLocaleString()} remaining · resets 1st of month`;const m=document.getElementById("dashUsageBar");m.style.width=l+"%",m.className="usage-bar-fill "+(l>=90?"crit":l>=70?"warn":"ok"),document.getElementById("dashUsageMeta").textContent=`${s.toLocaleString()} / ${d.toLocaleString()} · ${l}%`}}catch(n){document.getElementById("usageError").style.display="block",document.getElementById("usageErrorMsg").textContent=String(n)}}window.fetchUsage=Oe;async function je(){const e=document.getElementById("apiEndpoint").value.replace(/\/$/,""),t=document.getElementById("dashVmDot"),n=document.getElementById("dashVmLabel");try{if((await fetch(e+"/health",{signal:AbortSignal.timeout(4e3)})).ok){const i=document.getElementById("connLabel");i&&(i.textContent="Connected · "+e.replace("https://",""));const r=document.getElementById("connDot");r&&(r.className="sb-dot"),t&&(t.className="dot"),n&&(n.textContent="Online · "+e.replace("https://",""))}else throw new Error("not ok")}catch{const o=document.getElementById("connLabel");o&&(o.textContent="Offline");const i=document.getElementById("connDot");i&&(i.className="sb-dot err"),t&&(t.className="dot err"),n&&(n.textContent="Offline")}}window.checkConnection=je;function zi(){document.getElementById("upskillModal").style.display="none",document.body.style.overflow=""}document.addEventListener("keydown",e=>{e.key==="Escape"&&zi()});function Ee(e){const t=document.documentElement,n=document.getElementById("themeBtn"),o=document.getElementById("themeIcon");e==="light"?(t.setAttribute("data-theme","light"),n&&(n.title="Switch to dark mode"),o&&o.setAttribute("data-lucide","sun")):(t.removeAttribute("data-theme"),n&&(n.title="Switch to light mode"),o&&o.setAttribute("data-lucide","moon")),typeof lucide<"u"&&lucide.createIcons()}window.applyTheme=Ee;function Oi(){const t=(document.documentElement.getAttribute("data-theme")==="light"?"light":"dark")==="light"?"dark":"light";localStorage.setItem("ternstudio-theme",t),Ee(t),b&&monaco.editor.setTheme(t==="light"?"ternstudio-light":"ternstudio-dark")}window.toggleTheme=Oi;(function(){const e=localStorage.getItem("ternstudio-theme")||"dark";Ee(e)})();require.config({paths:{vs:"https://cdn.jsdelivr.net/npm/monaco-editor@0.52.0/min/vs"}});require(["vs/editor/editor.main"],function(){monaco.languages.register({id:"ternlang"}),monaco.languages.setMonarchTokensProvider("ternlang",{keywords:["fn","let","return","match","if","else","while","for","in","break","continue","spawn","send","await"],types:["trit","int","float","bool","void"],builtins:["truth","hold","conflict","consensus","print","println","affirm","reject","tend"],directives:["sparseskip","inline","export"],tokenizer:{root:[[/@[a-zA-Z_]+/,"keyword.directive"],[/\/\/.*$/,"comment"],[/\b(fn|let|return|match|if|else|while|for|in|break|continue|spawn|send|await)\b/,"keyword"],[/\b(trit|int|float|trittensor|void)\b/,"type"],[/\b(truth|hold|conflict|consensus|print|println|affirm|reject|tend)\b/,"support.function"],[/-1\b/,"number.trit.reject"],[/\b0\b/,"number.trit.hold"],[/\b1\b/,"number.trit.affirm"],[/\b\d+\.\d+\b/,"number.float"],[/\b\d+\b/,"number"],[/"([^"\\]|\\.)*"/,"string"],[/->/,"operator"],[/=>/,"operator"],[/[+\-*\/=!<>?%]/,"operator"],[/[{}()\[\];,:]/,"delimiter"],[/[a-zA-Z_]\w*/,"identifier"]]}}),monaco.editor.defineTheme("ternstudio-dark",{base:"vs-dark",inherit:!0,rules:[{token:"comment",foreground:"3a5060",fontStyle:"italic"},{token:"keyword",foreground:"00c8ff",fontStyle:"bold"},{token:"keyword.directive",foreground:"ffaa00",fontStyle:"bold"},{token:"type",foreground:"80e8ff"},{token:"support.function",foreground:"00e87a"},{token:"number.trit.affirm",foreground:"00e87a",fontStyle:"bold"},{token:"number.trit.reject",foreground:"ff3b55",fontStyle:"bold"},{token:"number.trit.hold",foreground:"ffaa00",fontStyle:"bold"},{token:"number.float",foreground:"a8d8ff"},{token:"number",foreground:"a8d8ff"},{token:"string",foreground:"c8f0a0"},{token:"operator",foreground:"6090b0"},{token:"delimiter",foreground:"405060"},{token:"identifier",foreground:"c8d4e0"}],colors:{"editor.background":"#080b10","editor.foreground":"#c8d4e0","editor.lineHighlightBackground":"#0d1219","editorCursor.foreground":"#00c8ff","editor.selectionBackground":"#0d2040","editorLineNumber.foreground":"#2a3a4a","editorLineNumber.activeForeground":"#5a7a9a","editorIndentGuide.background":"#1a2530","editorGutter.background":"#080b10","scrollbar.shadow":"#00000000","scrollbarSlider.background":"#1d2835","scrollbarSlider.hoverBackground":"#253040"}}),monaco.editor.defineTheme("ternstudio-light",{base:"vs",inherit:!0,rules:[{token:"comment",foreground:"7a9aaa",fontStyle:"italic"},{token:"keyword",foreground:"0060bb",fontStyle:"bold"},{token:"keyword.directive",foreground:"9a5000",fontStyle:"bold"},{token:"type",foreground:"006090"},{token:"support.function",foreground:"007040"},{token:"number.trit.affirm",foreground:"007040",fontStyle:"bold"},{token:"number.trit.reject",foreground:"bb001a",fontStyle:"bold"},{token:"number.trit.hold",foreground:"9a5000",fontStyle:"bold"},{token:"number.float",foreground:"005090"},{token:"number",foreground:"005090"},{token:"string",foreground:"306820"},{token:"operator",foreground:"507090"},{token:"delimiter",foreground:"708090"},{token:"identifier",foreground:"1a2535"}],colors:{"editor.background":"#f8fafc","editor.foreground":"#1a2535","editor.lineHighlightBackground":"#eef2f6","editorCursor.foreground":"#0070cc","editor.selectionBackground":"#cce0ff","editorLineNumber.foreground":"#a0aab5","editorLineNumber.activeForeground":"#607080","editorIndentGuide.background":"#d8dde3","editorGutter.background":"#f0f2f5","scrollbar.shadow":"#00000010","scrollbarSlider.background":"#c8ced4","scrollbarSlider.hoverBackground":"#b0b8c0"}});const e=localStorage.getItem("ternstudio-theme")||"dark";Ee(e),b=monaco.editor.create(document.getElementById("monaco-container"),{value:B[O]||ae.hello,language:"ternlang",theme:e==="light"?"ternstudio-light":"ternstudio-dark",fontFamily:"'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'Consolas', monospace",fontSize:13,lineHeight:20,minimap:{enabled:!1},scrollBeyondLastLine:!1,renderWhitespace:"boundary",bracketPairColorization:{enabled:!0},guides:{bracketPairs:!0},suggest:{showWords:!1},quickSuggestions:!1,padding:{top:12,bottom:12},overviewRulerLanes:0,hideCursorInOverviewRuler:!0,renderLineHighlight:"gutter",scrollbar:{verticalScrollbarSize:5,horizontalScrollbarSize:5}}),b.onDidChangeModelContent(()=>{be()}),b.onDidChangeCursorPosition(a=>{const s=document.getElementById("cursorPos");s&&(s.textContent=`Ln ${a.position.lineNumber}, Col ${a.position.column}`)}),window.addEventListener("resize",()=>b.layout()),b.addCommand(monaco.KeyCode.F5,()=>pt()),b.addCommand(monaco.KeyMod.CtrlCmd|monaco.KeyCode.Enter,()=>pt()),Vn();const t=localStorage.getItem("ternstudio-key")||"";t?(document.getElementById("apiKey").value=t,document.getElementById("topbarKeyInput").value=t,Oe(),he(),Le()):window.TERNSTUDIO_DEV_KEY?(document.getElementById("apiKey").value=window.TERNSTUDIO_DEV_KEY,document.getElementById("topbarKeyInput").value=window.TERNSTUDIO_DEV_KEY,localStorage.setItem("ternstudio-key",window.TERNSTUDIO_DEV_KEY),Oe(),he(),Le()):(Xe(),ze()),ne(),je(),setInterval(je,3e4),Kn();const n=JSON.parse(localStorage.getItem("ternflow_registry")||"[]"),o=[{id:"agent",slug:"agent",name:"Agent",desc:"Custom ternary pipeline",pricing:"per_call",nodes:2,deployed:"2026-04-19T00:00:00Z"},{id:"mesh-node-a",slug:"mesh-node-a",name:"Mesh_Node_A",desc:"Custom ternary pipeline",pricing:"private",nodes:8,deployed:"2026-04-19T00:00:00Z"}];let i=!1;o.forEach(a=>{n.find(s=>s.id===a.id)||(n.push(a),i=!0)}),i&&localStorage.setItem("ternflow_registry",JSON.stringify(n));const r=localStorage.getItem("ternstudio-last-view")||"dashboard";ro(),G(r),typeof it=="function"?it():window.addEventListener("load",()=>{window.mountControlBar&&window.mountControlBar()}),Lt([]),typeof lucide<"u"&&lucide.createIcons()});document.addEventListener("DOMContentLoaded",()=>{typeof lucide<"u"&&lucide.createIcons()});
