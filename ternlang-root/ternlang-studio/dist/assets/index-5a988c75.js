(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))o(i);new MutationObserver(i=>{for(const r of i)if(r.type==="childList")for(const a of r.addedNodes)a.tagName==="LINK"&&a.rel==="modulepreload"&&o(a)}).observe(document,{childList:!0,subtree:!0});function n(i){const r={};return i.integrity&&(r.integrity=i.integrity),i.referrerPolicy&&(r.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?r.credentials="include":i.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function o(i){if(i.ep)return;i.ep=!0;const r=n(i);fetch(i.href,r)}})();function mo(e){let t,n;try{const o=an(e,F.__wbindgen_malloc,F.__wbindgen_realloc),i=ze,r=F.check_tern(o,i);return t=r[0],n=r[1],rn(r[0],r[1])}finally{F.__wbindgen_free(t,n,1)}}function fo(e){let t,n;try{const o=an(e,F.__wbindgen_malloc,F.__wbindgen_realloc),i=ze,r=F.run_tern(o,i);return t=r[0],n=r[1],rn(r[0],r[1])}finally{F.__wbindgen_free(t,n,1)}}function go(){return{__proto__:null,"./ternlang_wasm_bg.js":{__proto__:null,__wbindgen_init_externref_table:function(){const t=F.__wbindgen_externrefs,n=t.grow(4);t.set(0,void 0),t.set(n+0,void 0),t.set(n+1,null),t.set(n+2,!0),t.set(n+3,!1)}}}}function rn(e,t){return e=e>>>0,ho(e,t)}let ye=null;function Ae(){return(ye===null||ye.byteLength===0)&&(ye=new Uint8Array(F.memory.buffer)),ye}function an(e,t,n){if(n===void 0){const s=ve.encode(e),l=t(s.length,1)>>>0;return Ae().subarray(l,l+s.length).set(s),ze=s.length,l}let o=e.length,i=t(o,1)>>>0;const r=Ae();let a=0;for(;a<o;a++){const s=e.charCodeAt(a);if(s>127)break;r[i+a]=s}if(a!==o){a!==0&&(e=e.slice(a)),i=n(i,o,o=a+e.length*3,1)>>>0;const s=Ae().subarray(i+a,i+o),l=ve.encodeInto(e,s);a+=l.written,i=n(i,o,a,1)>>>0}return ze=a,i}let Me=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});Me.decode();const yo=2146435072;let lt=0;function ho(e,t){return lt+=t,lt>=yo&&(Me=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}),Me.decode(),lt=t),Me.decode(Ae().subarray(e,e+t))}const ve=new TextEncoder;"encodeInto"in ve||(ve.encodeInto=function(e,t){const n=ve.encode(e);return t.set(n),{read:e.length,written:n.length}});let ze=0,F;function vo(e,t){return F=e.exports,ye=null,F.__wbindgen_start(),F}async function xo(e,t){if(typeof Response=="function"&&e instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(e,t)}catch(i){if(e.ok&&n(e.type)&&e.headers.get("Content-Type")!=="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",i);else throw i}const o=await e.arrayBuffer();return await WebAssembly.instantiate(o,t)}else{const o=await WebAssembly.instantiate(e,t);return o instanceof WebAssembly.Instance?{instance:o,module:e}:o}function n(o){switch(o){case"basic":case"cors":case"default":return!0}return!1}}async function wo(e){if(F!==void 0)return F;e!==void 0&&(Object.getPrototypeOf(e)===Object.prototype?{module_or_path:e}=e:console.warn("using deprecated parameters for the initialization function; pass a single object instead")),e===void 0&&(e=new URL("/assets/ternlang_wasm_bg-13f2e166.wasm",self.location));const t=go();(typeof e=="string"||typeof Request=="function"&&e instanceof Request||typeof URL=="function"&&e instanceof URL)&&(e=fetch(e));const{instance:n,module:o}=await xo(await e,t);return vo(n)}wo().then(()=>{window.wasmRunTern=fo,window.wasmCheckTern=mo,window.wasmReady=!0,window.dispatchEvent(new Event("wasmready"))}).catch(()=>{window.wasmReady=!1});let $e=null;async function bo(){if($e)return $e;try{const e=document.getElementById("sbWasmStatus");return e&&(e.textContent="Pyodide loading…"),$e=await loadPyodide(),e&&(e.textContent="TernVM + Pyodide Ready"),$e}catch(e){return console.error("Pyodide Load Error:",e),null}}async function sn(e){const t=await bo();if(!t)return{ok:!1,error:"Pyodide not available"};let n="";t.setStdout({batched:o=>{n+=o+`
`}}),t.setStderr({batched:o=>{n+="ERR: "+o+`
`}});try{return await t.runPythonAsync(e),{ok:!0,output:n.trim()}}catch(o){return{ok:!1,error:o.message,traceback:String(o)}}}window.runPythonActuator=sn;if(window.location.hostname==="localhost"||window.location.hostname==="127.0.0.1"){const e=document.createElement("script");e.src=".ternstudio-local.js",e.onerror=()=>console.warn("Dev script missing (expected in local dev)"),document.head.appendChild(e)}const Re={1:"Tier 1 — Free",2:"Tier 2 — Pro",3:"Tier 3 — Industrial",4:"Tier 4 — Enterprise"},mt={1:"badge-free",2:"badge-t2",3:"badge-t3",4:"badge-t4"},ue={hello:`fn main() -> trit {
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
}`};(function(){const e=window.location;if(e.hostname!=="localhost"&&e.hostname!=="127.0.0.1"&&e.protocol!=="file:"){const t=e.protocol+"//"+e.hostname+(e.port?":"+e.port:"");document.getElementById("apiEndpoint").value=t}})();let _=null,W=localStorage.getItem("ternstudio-active-file")||"examples/hello_trit.tern",q=JSON.parse(localStorage.getItem("ternstudio-tabs")||JSON.stringify([{name:"hello_trit.tern",path:"examples/hello_trit.tern"}])),P=JSON.parse(localStorage.getItem("ternstudio-file-buffers")||JSON.stringify({"examples/hello_trit.tern":ue.hello})),ln=200;function ft(e){const t=parseInt(e);ln=Math.max(0,1e3-t),localStorage.setItem("ternflow_sim_speed",t);const n=document.getElementById("simSpeedSlider");n&&(n.value=t)}window.updateSimSpeed=ft;function Eo(){const e=localStorage.getItem("ternflow_sim_speed");ft(e!==null?e:800)}function Ce(){_&&(P[W]=_.getValue()),localStorage.setItem("ternstudio-active-file",W),localStorage.setItem("ternstudio-tabs",JSON.stringify(q)),localStorage.setItem("ternstudio-file-buffers",JSON.stringify(P))}window.saveEditorState=Ce;let Io=1,Q=[],Yt=0,Jt=0,Xt=0;function dn(){const e=document.getElementById("keyToggleArea"),t=document.getElementById("topbarActions"),n=e.style.display==="none";if(e.style.display=n?"flex":"none",t.style.display=n?"none":"flex",n){const o=document.getElementById("topbarKeyInput");o.value=document.getElementById("apiKey").value,setTimeout(()=>o.focus(),10)}}window.toggleKeyInput=dn;function cn(e){e=e.trim(),document.getElementById("apiKey").value=e,localStorage.setItem("ternstudio-key",e),Ye(),e&&(I("API Key updated","ok"),dn(),De())}window.updateApiKey=cn;function So(e){const t=document.getElementById("stdlib-tree-container");t&&(t.style.display=e?"block":"none")}window.toggleStdlibVisibility=So;async function De(){const e=document.getElementById("apiKey").value.trim(),t=document.getElementById("premium-tree-container"),n=document.getElementById("premium-file-tree");if(!e||!t||!n){t&&(t.style.display="none");return}t.style.display="block",n.innerHTML='<div class="tree-file" style="color:var(--muted)">Syncing premium assets...</div>';try{const o=document.getElementById("apiEndpoint").value.replace(/\/$/,""),i=await fetch(`${o}/api/premium/list`,{headers:{"X-Ternlang-Key":localStorage.getItem("ternstudio-key")||""}});if(!i.ok){const a=await i.text();console.error(`Premium fetch failed | Status: ${i.status} | URL: ${i.url} | Raw Response:`,a);const s=i.status===403?"Auth Failed. Invalid Key.":`HTTP Error ${i.status}`;n.innerHTML=`<div class="tree-file" style="color:var(--red)">${s}</div>`;return}const r=await i.json();r.status==="ok"&&Array.isArray(r.files)?nt(n,r.files,!1,!0):n.innerHTML=`<div class="tree-file" style="color:var(--red)">Error: ${r.error||"Failed to load structure"}</div>`}catch(o){n.innerHTML='<div class="tree-file" style="color:var(--red)">Connection to API failed.</div>',console.error("Premium fetch error:",o)}}window.loadPremiumTree=De;async function Y(e){localStorage.setItem("ternstudio-last-view",e),document.querySelectorAll(".view").forEach(i=>i.classList.remove("active")),document.querySelectorAll(".nav-tab").forEach(i=>i.classList.remove("active"));const t=document.getElementById("view-"+e),n=document.getElementById("vt-"+e),o=document.getElementById("config-view");t&&t.classList.add("active"),n&&n.classList.add("active"),o&&(e==="settings"?o.style.display="flex":o.style.display="none"),e==="editor"&&_&&setTimeout(()=>_.layout(),50),e==="flow"&&(wn(),Se()),e==="debugger"&&pn(),e==="modules"&&await St(),e==="translator"&&await un(),e==="fleet"&&await kt(),e==="settings"&&uo(),lucide.createIcons()}window.switchView=Y;function ko(e){const[t,n]=React.useState([]),[o,i]=React.useState(!1);return React.useEffect(()=>{const r=a=>{n(s=>[a.detail,...s].slice(0,1e3))};return window.addEventListener("ternlang_local_trace",r),()=>window.removeEventListener("ternlang_local_trace",r)},[]),React.useEffect(()=>{if(!e)return;let r,a,s=!1;function l(){r=new WebSocket(e),r.onopen=()=>{i(!0)},r.onmessage=d=>{try{if(d.data==="connected")return;const c=JSON.parse(d.data);n(p=>[c,...p].slice(0,1e3))}catch{}},r.onclose=()=>{i(!1),s||(a=setTimeout(l,3e3))},r.onerror=()=>{}}return l(),()=>{s=!0,clearTimeout(a),r&&r.close()}},[e]),{telemetry:t,isConnected:o,clearTelemetry:()=>n([])}}function To({apiEndpoint:e}){const t=e.replace("http","ws")+"/api/tracer/ws",{telemetry:n,isConnected:o,clearTelemetry:i}=ko(t),r=a=>a===1?"#10b981":a===-1?"#ef4444":"#f59e0b";return React.createElement("div",{style:{padding:"24px",color:"#f1f5f9",fontFamily:"Inter, sans-serif"}},React.createElement("div",{style:{display:"flex",justifyContent:"space-between",alignItems:"center",marginBottom:"20px"}},React.createElement("h2",{style:{fontSize:"18px",fontWeight:"800",margin:0}},React.createElement("span",{style:{color:o?"#10b981":"#ef4444",marginRight:"8px"}},"●"),"Execution Tracer Pipeline"),React.createElement("button",{className:"btn btn-ghost",onClick:i,style:{fontSize:"12px"}},"Clear Trace")),React.createElement("div",{style:{background:"#1e293b",border:"1px solid #334155",borderRadius:"12px",overflow:"hidden"}},React.createElement("table",{style:{width:"100%",borderCollapse:"collapse",fontSize:"12px",textAlign:"left"}},React.createElement("thead",{style:{background:"#334155",color:"#cbd5e1",textTransform:"uppercase",letterSpacing:"0.05em"}},React.createElement("tr",{},["Timestamp","Node ID","Event","Result","Latency","Causal"].map(a=>React.createElement("th",{key:a,style:{padding:"12px 16px"}},a)))),React.createElement("tbody",{},n.length===0?React.createElement("tr",{},React.createElement("td",{colSpan:6,style:{padding:"4rem",textAlign:"center",color:"var(--muted)",fontStyle:"italic",fontSize:"1.1rem"}},"Awaiting telemetry firehose...")):n.map(a=>React.createElement("tr",{key:a.trace_id,style:{borderBottom:"1px solid #334155",opacity:a.sparse_dropped?.5:1}},React.createElement("td",{style:{padding:"12px 16px",color:"#cbd5e1"}},new Date(a.timestamp_ms).toLocaleTimeString()),React.createElement("td",{style:{padding:"12px 16px",fontWeight:"700"}},a.node_id),React.createElement("td",{style:{padding:"12px 16px"}},a.event_type),React.createElement("td",{style:{padding:"12px 16px"}},React.createElement("span",{style:{padding:"2px 8px",borderRadius:"4px",background:r(a.signal_out)+"22",color:r(a.signal_out),fontWeight:"800"}},a.signal_out>0?"+1":a.signal_out<0?"-1":"0")),React.createElement("td",{style:{padding:"12px 16px"}},`${a.latency_ms}ms`,a.sparse_dropped&&React.createElement("span",{style:{marginLeft:"8px",background:"#38bdf8",color:"#0f172a",fontSize:"9px",padding:"2px 6px",borderRadius:"4px",fontWeight:"900"}},"BYPASSED")),React.createElement("td",{style:{padding:"8px 16px"}},React.createElement("button",{className:"btn",style:{fontSize:"9px",height:"20px",background:"var(--bg2)",border:"1px solid var(--border2)",color:"var(--cyan)"},onClick:()=>window.downloadCausalArtifact(a.trace_id)},"Artifact"))))))))}function Co(){const e=React.useRef(null),n=window.location.hostname==="localhost"||window.location.hostname==="127.0.0.1"?"http://localhost:5000":"https://ternlang-api.fly.dev/translator";return React.useEffect(()=>{const o=()=>{if(e.current){console.log("[TranslatorView] Iframe loaded, transmitting API key...");const r=localStorage.getItem("ternstudio-key")||"";e.current.contentWindow.postMessage({type:"TIS_AUTH_BRIDGE",key:r},n)}},i=e.current;if(i)return i.addEventListener("load",o),()=>i.removeEventListener("load",o)},[n]),React.createElement("div",{style:{width:"100%",height:"calc(100vh - 64px)",overflow:"hidden",background:"var(--bg)"}},React.createElement("iframe",{ref:e,src:n,style:{width:"100%",height:"100%",border:"0"},sandbox:"allow-scripts allow-same-origin allow-forms allow-popups",title:"Ternlang Translator"}))}let dt=null,ct=null;async function pn(){const e=document.getElementById("view-debugger");if(!e||!window.ReactDOM)return;const t=document.getElementById("apiEndpoint").value;if(!dt){e.innerHTML='<div id="tracer-react-mount" style="width:100%;"></div>';const n=document.getElementById("tracer-react-mount");dt=ReactDOM.createRoot(n),dt.render(React.createElement(To,{apiEndpoint:t}))}}window.renderTracerView=pn;async function un(){const e=document.getElementById("view-translator");if(!(!e||!window.ReactDOM)&&!ct){e.innerHTML='<div id="translator-react-mount" style="width:100%;"></div>';const t=document.getElementById("translator-react-mount");ct=ReactDOM.createRoot(t),ct.render(React.createElement(Co))}}window.renderTranslatorView=un;let oe=null;function _o(e,t){oe=e,document.getElementById("deleteAgentName").textContent=t,document.getElementById("deleteModal").style.display="flex"}window.confirmDeleteAgent=_o;function mn(){document.getElementById("deleteModal").style.display="none",oe=null}window.closeDeleteModal=mn;async function Bo(){if(!oe)return;const e=document.getElementById("apiKey").value.trim(),t=document.getElementById("apiEndpoint").value.replace(/\/$/,"");try{if(e){const i=await(await fetch(`${t}/api/agent/${oe}`,{method:"DELETE",headers:{"X-Ternlang-Key":e}})).json();if(i.status!=="ok"){I("Server rejection: "+(i.error||"Unknown error"),"err");return}}let n=[];try{n=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}n=n.filter(o=>o.id!==oe),localStorage.setItem("ternflow_registry",JSON.stringify(n)),window.selectedFleetAgentId===oe&&(window.selectedFleetAgentId=n.length>0?n[0].id:null),mn(),St(),document.getElementById("view-fleet").classList.contains("active")&&kt(),I(`Agent "${oe}" permanently purged`,"ok")}catch(n){I("Purge failure: Connection lost","err"),console.error("Causal state failure:",n)}}window.deleteAgent=Bo;async function Ee(){try{const t=await(await fetch("https://ternlang-api.fly.dev/api/agents",{headers:{"X-Ternlang-Key":localStorage.getItem("ternstudio-key")||""}})).json();if(t.status==="ok"&&t.agents){let n=[];try{n=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}let o=!1;t.agents.forEach(i=>{const r=n.find(a=>a.id===i.slug);r?(r.name=i.name,r.desc=i.desc,i.created_at&&(r.deployed=i.created_at),o=!0):(n.push({id:i.slug,slug:i.slug,name:i.name,desc:i.desc,pricing:i.pricing||"community",nodes:i.nodes||1,deployed:i.created_at||new Date().toISOString(),isRemote:!0}),o=!0)}),o&&localStorage.setItem("ternflow_registry",JSON.stringify(n))}}catch(e){console.warn("Fleet remote sync failed",e)}}window.syncFleetRegistry=Ee;async function St(){const e=document.getElementById("view-modules");if(!e)return;await Ee();let t=[];try{t=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}let n=`
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
  `,Object.entries(Fe).forEach(([o,i])=>{n+=`
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
    `}),Bt.forEach(o=>{const i=o.split("/").pop().replace(".tern","");n+=`
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
    `}),n+="</div></div></div>",e.innerHTML=n,lucide.createIcons()}window.renderRegistryView=St;window.selectedFleetAgentId=null;let Lo={};async function kt(){const e=document.getElementById("view-fleet");if(!e)return;let t=[];try{t=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}t.length===0&&(e.innerHTML=`
      <div style="padding: 100px 40px; text-align: center; color:var(--text);">
        <div class="status-running" style="width:48px; height:48px; border-radius:50%; margin:0 auto 20px; display:flex; align-items:center; justify-content:center;">
          <i data-lucide="refresh-cw" class="spin" style="width:24px; height:24px;"></i>
        </div>
        <h2 style="font-size: 20px; font-weight: 700;">Hydrating Fleet...</h2>
        <p style="color: var(--muted); max-width:400px; margin: 10px auto;">Connecting to ternlang-api.fly.dev to fetch active deployment index.</p>
      </div>
    `,lucide.createIcons());try{await Ee();try{t=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}}catch(r){console.error("Fleet hydration failed:",r)}if(t.length===0){e.innerHTML=`
      <div style="padding: 100px 40px; text-align: center; color:var(--text);">
        <i data-lucide="tower-control" style="width:64px; height:64px; opacity:0.1; margin-bottom:20px;"></i>
        <h2 style="font-size: 20px; font-weight: 700;">Fleet Connection Offline</h2>
        <p style="color: var(--muted); max-width:400px; margin: 10px auto;">Could not reach the TIS deployment API. Check your network or deploy a local agent.</p>
        <button class="btn btn-primary" style="margin-top:20px;" onclick="switchView('flow')">Go to Flow Lab</button>
      </div>
    `,lucide.createIcons();return}!window.selectedFleetAgentId&&t.length>0&&(window.selectedFleetAgentId=t[0].id);const n=t.find(r=>r.id===window.selectedFleetAgentId)||t[0];if(!n)return;const o=Lo[n.id]||null;let i=`
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
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace;">${o?o.runs:"—"}</div>
            <div style="font-size:10px; color:var(--muted); margin-top:4px;">${o?"recorded runs":"no data yet"}</div>
          </div>
          <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; padding:16px;">
            <div style="font-size:10px; font-weight:700; color:var(--muted); text-transform:uppercase; margin-bottom:8px;">Success Rate</div>
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace; color:var(--green);">${o?((o.runs-o.errors)/o.runs*100).toFixed(1)+"%":"—"}</div>
            <div style="font-size:10px; color:var(--muted); margin-top:4px;">${o?o.errors+" rejections":"run a simulation first"}</div>
          </div>
          <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; padding:16px;">
            <div style="font-size:10px; font-weight:700; color:var(--muted); text-transform:uppercase; margin-bottom:8px;">Avg Confidence</div>
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace; color:var(--cyan);">${o?o.avgConf.toFixed(3):"—"}</div>
            <div style="font-size:10px; color:var(--muted); margin-top:4px;">${o?"from completed runs":"no data yet"}</div>
          </div>
          <div style="background:var(--bg1); border:1px solid var(--border2); border-radius:12px; padding:16px;">
            <div style="font-size:10px; font-weight:700; color:var(--muted); text-transform:uppercase; margin-bottom:8px;">Latency (p95)</div>
            <div style="font-size:24px; font-weight:800; font-family:'JetBrains Mono',monospace;">${o?o.latency.toFixed(0)+"ms":"—"}</div>
            <div style="font-size:10px; color:var(--muted); margin-top:4px;">${o?"measured":"no data yet"}</div>
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
            <div style="flex:1; background:#000; border:1px solid var(--border2); border-radius:8px; font-family:'JetBrains Mono',monospace; font-size:11px; padding:16px; overflow-y:auto; color:#a5f3fc;" id="fleet-stream-${n.id}">
              <div style="color:var(--muted2);">// Run a simulation in the Lab to see live events here.</div>
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
                Snapshot updates after each simulation run.
              </div>
            </div>

            <div style="background:rgba(239, 68, 68, 0.05); border:1px solid rgba(239, 68, 68, 0.2); border-radius:12px; padding:16px;">
              <h3 style="font-size:12px; font-weight:700; color:var(--red); margin-bottom:12px; display:flex; align-items:center; gap:6px;">
                <i data-lucide="alert-triangle" style="width:14px;"></i> System Alerts
              </h3>
              <div style="font-size:11px; color:var(--muted); line-height:1.5;">
                No alerts. Alerts will appear here when agent thresholds are breached during simulation.
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  `;e.innerHTML=i,lucide.createIcons()}window.renderFleetView=kt;let x=[],E=[],O=[],z=null,$=null,H=!1,Oe=!1,D="idle";window.executionState="idle";window.simulationRunning=!1;window.simulationAborted=!1;let G=0,le=0;function Xe(){const e=document.getElementById("simBtn");e&&(D==="running"?(e.innerHTML='<i data-lucide="pause" style="width:15px"></i> <span style="font-size:12px;font-weight:700;">PAUSE</span>',e.style.color="var(--amber)",e.classList.add("running")):D==="paused"?(e.innerHTML='<i data-lucide="play" style="width:15px"></i> <span style="font-size:12px;font-weight:700;">RESUME</span>',e.style.color="var(--green)",e.classList.add("running")):(e.innerHTML='<i data-lucide="play-circle" style="width:15px"></i> <span style="font-size:12px;font-weight:600;">SIMULATE</span>',e.style.color="var(--green)",e.classList.remove("running")),window.lucide&&lucide.createIcons())}window.updateSimUI=Xe;function ie(e){if(window.executionState===e)return;const t=window.executionState;console.log(`[TernFlow] Execution State -> ${e}`),window.executionState=e,D=e,Oe=e==="running",e==="idle"?H=!0:H=!1,e==="running"?I(t==="paused"?"Execution resumed":"Simulation started","info"):e==="paused"&&I("Execution paused","warn"),Xe(),window.dispatchEvent(new CustomEvent("executionstatechange",{detail:{state:e}}))}window.setExecutionState=ie;function Tt(){return D!=="paused"?Promise.resolve():new Promise(e=>{const t=n=>{(n.detail.state==="running"||n.detail.state==="idle")&&(window.removeEventListener("executionstatechange",t),e())};window.addEventListener("executionstatechange",t)})}function $o(){D==="running"?ie("paused"):D==="paused"?(ie("running"),le=performance.now(),requestAnimationFrame(window.currentDriveTimeline)):fn()}window.toggleSimulation=$o;function fn(){const e=document.getElementById("global-timeline"),t=document.getElementById("scrub-layer");x.forEach(n=>{n.visited=!1,n.executed=!1}),E.forEach(n=>{n.active=!1}),e&&(e.value=0),G=0,le=performance.now(),t&&t.getContext("2d").clearRect(0,0,t.width,t.height),document.querySelectorAll(".trit-particle-ghost").forEach(n=>n.remove()),Dn()}window.startNewSimulation=fn;function Ao(){ie("idle"),I("Simulation stopped and reset","warn"),G=0;const e=document.getElementById("global-timeline");e&&(e.value=0);const t=document.getElementById("scrub-layer");t&&t.getContext("2d").clearRect(0,0,t.width,t.height),document.querySelectorAll(".trit-particle-ghost").forEach(n=>n.remove())}window.stopSimulation=Ao;function Mo(){const[e,t]=React.useState(window.executionState||"idle");React.useEffect(()=>{const s=l=>t(l.detail.state);return window.addEventListener("executionstatechange",s),()=>window.removeEventListener("executionstatechange",s)},[]),React.useEffect(()=>{window.lucide&&window.lucide.createIcons()},[e]);const n={display:"inline-flex",alignItems:"center",height:"32px",width:"140px",background:"var(--bg1)",border:"1px solid var(--border2)",borderRadius:"16px",overflow:"hidden",boxShadow:"var(--shadow)"},o=(s,l)=>({flex:1,display:"flex",alignItems:"center",justifyContent:"center",height:"100%",cursor:"pointer",transition:"all 0.2s ease",background:s?l:"transparent",color:s?"#fff":"var(--text)",borderRight:"1px solid var(--border2)",pointerEvents:"auto"}),i=s=>{s.preventDefault(),s.stopPropagation();const l=window.executionState||"idle";console.log("[ControlBar] Play Invoked. Current state:",l),l==="paused"?window.setExecutionState("running"):l==="idle"&&window.startNewSimulation()},r=s=>{s.preventDefault(),s.stopPropagation(),console.log("[ControlBar] Pause Invoked. Current state:",window.executionState),window.executionState==="running"&&window.setExecutionState("paused")},a=s=>{s.preventDefault(),s.stopPropagation(),console.log("[ControlBar] Stop Invoked."),window.stopSimulation()};return React.createElement("div",{style:n},React.createElement("div",{style:o(e==="running","#10b981"),title:"Play / Resume",onClick:i},React.createElement("i",{"data-lucide":"play",style:{width:"14px",pointerEvents:"none"}})),React.createElement("div",{style:o(e==="paused","#f59e0b"),title:"Pause",onClick:r},React.createElement("i",{"data-lucide":"pause",style:{width:"14px",pointerEvents:"none"}})),React.createElement("div",{style:{...o(!1,"transparent"),borderRight:"none"},title:"Stop & Reset",onClick:a,onMouseEnter:s=>s.currentTarget.style.color="#ef4444",onMouseLeave:s=>s.currentTarget.style.color="#f1f5f9"},React.createElement("i",{"data-lucide":"square",style:{width:"14px",pointerEvents:"none"}})))}function gt(){const e=document.getElementById("control-bar-mount");if(!e||!window.ReactDOM)return;ReactDOM.createRoot(e).render(React.createElement(Mo))}window.mountControlBar=gt;async function yt(e,t){event==null||event.stopPropagation(),A("SYSTEM",`⚡ Manual injection: [v:${t}, c:1.0] -> ${e}`),A("SYSTEM",`⚡ Manual injection logged: [v:${t}, c:1.0] -> ${e}. Awaiting explicit Play to execute.`)}window.injectSignal=yt;const je=new Set,ht=new Set;function Ct(e){je.add(e),E.filter(n=>n.toId===e).forEach(n=>{ht.add(n.id),je.has(n.fromId)||Ct(n.fromId)})}window.findParents=Ct;function Ro(e){document.querySelectorAll(".causal-path, .causal-node, .dimmed").forEach(n=>{n.classList.remove("causal-path","causal-node","dimmed")}),je.clear(),ht.clear(),Ct(e),x.forEach(n=>{const o=document.getElementById(n.id);o&&(je.has(n.id)?o.classList.add("causal-node"):o.classList.add("dimmed"))}),E.forEach(n=>{const o=document.getElementById(n.id),i=document.getElementById("hit-"+n.id);o&&(ht.has(n.id)?o.classList.add("causal-path"):(o.classList.add("dimmed"),i&&i.classList.add("dimmed")))}),I("Showing causal path for "+e,"ok");const t=document.getElementById("flow-canvas-wrap");if(t){const n=o=>{o.target.closest("#flow-inspector")||o.target.closest(".timeline-container")||o.target.closest(".flow-node")||o.target.classList.contains("flow-wire")||o.target.classList.contains("wire-hit")||(document.querySelectorAll(".causal-path, .causal-node, .dimmed").forEach(i=>{i.classList.remove("causal-path","causal-node","dimmed")}),t.removeEventListener("mousedown",n))};t._causalClearFn&&t.removeEventListener("mousedown",t._causalClearFn),t._causalClearFn=n,t.addEventListener("mousedown",n)}}window.traceCausalPath=Ro;function gn(){document.getElementById("macro-name-modal").style.display="none"}window.closeMacroModal=gn;function yn(){if(L.size<2){I("Select at least 2 nodes to group","error");return}document.getElementById("macro-name-modal").style.display="flex",document.getElementById("macro-name-input").value="Logic_Module_"+Math.floor(Math.random()*1e3),setTimeout(()=>document.getElementById("macro-name-input").focus(),10)}window.groupSelectedNodes=yn;function No(){const e=document.getElementById("macro-name-input").value.trim()||"Logic Module",t="macro_"+Date.now(),n=x.filter(v=>L.has(v.id)),o=x.filter(v=>!L.has(v.id));let i=1/0,r=1/0,a=-1/0,s=-1/0;n.forEach(v=>{const w=document.getElementById(v.id),k=parseFloat(w.style.left),B=parseFloat(w.style.top);i=Math.min(i,k),r=Math.min(r,B),a=Math.max(a,k+180),s=Math.max(s,B+100)});const l=(i+a)/2,d=(r+s)/2,c=n.map(v=>{const w=document.getElementById(v.id);return{...v,ox:parseFloat(w.style.left)-l,oy:parseFloat(w.style.top)-d}}),p=E.filter(v=>L.has(v.fromId)&&L.has(v.toId)),g=E.filter(v=>!L.has(v.fromId)&&L.has(v.toId)),f=E.filter(v=>L.has(v.fromId)&&!L.has(v.toId)),u=E.filter(v=>!L.has(v.fromId)&&!L.has(v.toId)),m={internal_graph:{nodes:c,wires:p},input_schema:"macro_in: trit",output_schema:"macro_out: trit",code:`// Encapsulated logic: ${n.length} nodes
// Internal routing preserved.`},h=[...u];g.forEach(v=>h.push({...v,toId:t,id:"wire_ext_in_"+Date.now()+Math.random(),originalToId:v.toId})),f.forEach(v=>h.push({...v,fromId:t,id:"wire_ext_out_"+Date.now()+Math.random(),originalFromId:v.fromId})),x=o,E=h,n.forEach(v=>{var w;return(w=document.getElementById(v.id))==null?void 0:w.remove()}),p.forEach(v=>{var w,k,B;(w=document.getElementById(v.id))==null||w.remove(),(k=document.getElementById("hit-"+v.id))==null||k.remove(),(B=document.getElementById("badge-"+v.id))==null||B.remove()}),K(e,"__macro__",l,d,"macro",t);const y=x.find(v=>v.id===t);y&&(y.props=m),gn(),xe(),N(),R(),I(`Grouped ${n.length} nodes into ${e}`,"ok")}window.confirmGroupNodes=No;window.groupSelectedNodes=yn;function hn(e){var a;const t=x.find(s=>s.id===e);if((!t||!t.props.internal_graph)&&!(t&&t.props.nodes))return;const n=t.props.internal_graph,o=t.x,i=t.y,r=E.filter(s=>s.fromId===e||s.toId===e);x=x.filter(s=>s.id!==e),(a=document.getElementById(e))==null||a.remove(),n.nodes.forEach(s=>{const l=o+(s.ox||0),d=i+(s.oy||0);K(s.name,s.path,l,d,s.type,s.id);const c=x.find(p=>p.id===s.id);c&&(c.props=s.props)}),E=[...E.filter(s=>s.fromId!==e&&s.toId!==e),...n.wires],r.forEach(s=>{s.toId===e&&s.originalToId?E.push({...s,toId:s.originalToId,id:"wire_restitch_in_"+Date.now()+Math.random()}):s.fromId===e&&s.originalFromId&&E.push({...s,fromId:s.originalFromId,id:"wire_restitch_out_"+Date.now()+Math.random()})}),N(),R(),I(`Expanded "${t.name}"`,"ok")}window.expandMacro=hn;let b={scale:1,x:0,y:0};const Po=.15,zo=2.5;function re(){const e=document.getElementById("flow-canvas");e&&(e.style.transform=`translate(${b.x}px,${b.y}px) scale(${b.scale})`);const t=document.getElementById("zoomLabel");t&&(t.textContent=Math.round(b.scale*100)+"%");const n=document.getElementById("flow-canvas-wrap");if(n){const o=Math.max(8,32*b.scale);n.style.backgroundSize=`${o}px ${o}px`,n.style.backgroundPosition=`${b.x}px ${b.y}px`}D==="paused"&&typeof Te=="function"&&Te(G)}window.applyTransform=re;function _t(e,t,n){const o=b.scale*n;b.x=e-(e-b.x)*(o/b.scale),b.y=t-(t-b.y)*(o/b.scale),b.scale=o,re(),fe()}window.zoomAt=_t;function Do(e){const t=document.getElementById("flow-canvas-wrap"),n=t.clientWidth/2,o=t.clientHeight/2;_t(n,o,e>0?1.2:1/1.2)}window.zoomStep=Do;function Oo(){if(x.length===0){b.scale=1,b.x=0,b.y=0,re();return}document.getElementById("flow-canvas");const e=document.getElementById("flow-canvas-wrap");let t=1/0,n=1/0,o=-1/0,i=-1/0;x.forEach(p=>{const g=document.getElementById(p.id);if(!g)return;const f=parseFloat(g.style.left)||0,u=parseFloat(g.style.top)||0,m=g.offsetWidth||200,h=g.offsetHeight||80;t=Math.min(t,f),n=Math.min(n,u),o=Math.max(o,f+m),i=Math.max(i,u+h)});const r=60,a=e.clientWidth-r*2,s=e.clientHeight-r*2,l=o-t,d=i-n,c=Math.min(zo,Math.max(Po,Math.min(a/(l||1),s/(d||1))));b.scale=c,b.x=r+(a-l*c)/2-t*c,b.y=r+(s-d*c)/2-n*c,re()}window.fitToView=Oo;function Qe(e,t){return{x:(e-b.x)/b.scale,y:(t-b.y)/b.scale}}window.screenToCanvas=Qe;function vn(){const e=document.getElementById("flow-canvas-wrap");if(!e)return;let t=0,n=0,o=0,i=null;function r(){if(Math.abs(t)<1e-4){i=null;return}_t(n,o,1+t),t*=.82,i=requestAnimationFrame(r)}e.addEventListener("wheel",u=>{u.preventDefault();const m=e.getBoundingClientRect();n=u.clientX-m.left,o=u.clientY-m.top;const y=-(u.deltaMode===1?u.deltaY*16:u.deltaY)*8e-4;t=(t+y)*.6+y*.4,i||(i=requestAnimationFrame(r))},{passive:!1});let a=!1,s=0,l=0,d=0,c=0,p=!1;document.addEventListener("keydown",u=>{u.code==="Space"&&document.activeElement.tagName!=="INPUT"&&document.activeElement.tagName!=="TEXTAREA"&&(p=!0,e.style.cursor="grab",u.preventDefault())}),document.addEventListener("keyup",u=>{u.code==="Space"&&(p=!1,e.style.cursor="",e.classList.remove("panning"))}),e.addEventListener("mousedown",u=>{if(u.button===1||u.button===0&&p){a=!0,s=u.clientX,l=u.clientY,d=b.x,c=b.y,e.classList.add("panning");const m=document.getElementById("flow-canvas");m&&(m.style.transition="none"),u.preventDefault()}else if(u.button===0&&(u.target===e||u.target.id==="flow-canvas")&&!p){g=!0,f={x:u.clientX,y:u.clientY};const m=document.getElementById("rubber-band");m&&(m.style.display="block",m.style.left=u.clientX-e.getBoundingClientRect().left+"px",m.style.top=u.clientY-e.getBoundingClientRect().top+"px",m.style.width="0",m.style.height="0"),u.shiftKey||xe()}}),e.addEventListener("dragover",u=>{u.preventDefault(),u.dataTransfer.dropEffect="copy"}),e.addEventListener("drop",async u=>{u.preventDefault();const m=u.dataTransfer.getData("tern-node-type"),h=e.getBoundingClientRect(),y=u.clientX-h.left,v=u.clientY-h.top,w={x:(y-b.x)/b.scale,y:(v-b.y)/b.scale};if(m==="agent"){const k=u.dataTransfer.getData("tern-node-name"),B=u.dataTransfer.getData("tern-node-path"),S=u.dataTransfer.getData("tern-node-code"),T="node_"+Date.now();if(K(k,B,w.x,w.y,"agent",T),S){const C=x.find(M=>M.id===T);C&&(C.props.code=S,C.props.input_schema="signal: trit",C.props.output_schema="signal: trit")}else if(B!=="__builtin__")try{const C=await fetch(tt+B);if(C.ok){const M=await C.text(),Z=x.find(Le=>Le.id===T);Z&&(Z.props.code=M,R())}}catch{}}else if(m==="archetype"){const k=u.dataTransfer.getData("tern-arch-id"),B=$n.find(S=>S.id===k);B&&Rt(B,w.x-300,w.y-200)}});let g=!1,f={};document.addEventListener("mousemove",u=>{if(a&&(b.x=d+(u.clientX-s),b.y=c+(u.clientY-l),re(),fe()),g){const m=e.getBoundingClientRect(),h=Math.min(f.x,u.clientX)-m.left,y=Math.min(f.y,u.clientY)-m.top,v=Math.max(f.x,u.clientX)-m.left,w=Math.max(f.y,u.clientY)-m.top,k=document.getElementById("rubber-band");k&&(k.style.left=h+"px",k.style.top=y+"px",k.style.width=v-h+"px",k.style.height=w-y+"px")}}),document.addEventListener("mouseup",u=>{if(a){a=!1,e.classList.remove("panning");const m=document.getElementById("flow-canvas");m&&(m.style.transition="")}if(g){g=!1;const m=document.getElementById("rubber-band");m&&(m.style.display="none");const h=Math.min(f.x,u.clientX),y=Math.max(f.x,u.clientX),v=Math.min(f.y,u.clientY),w=Math.max(f.y,u.clientY);if(Math.sqrt(Math.pow(u.clientX-f.x,2)+Math.pow(u.clientY-f.y,2))>5)x.forEach(B=>{const S=document.getElementById(B.id);if(!S)return;const T=S.getBoundingClientRect(),C=T.left+T.width/2,M=T.top+T.height/2;C>=h&&C<=y&&M>=v&&M<=w&&We(B.id,!0)});else{const B=document.getElementById("flow-canvas-wrap");(u.target===B||u.target.id==="flow-canvas")&&xe()}}}),document.addEventListener("keydown",u=>{const m=document.activeElement.tagName==="INPUT"||document.activeElement.tagName==="TEXTAREA"||document.activeElement.isContentEditable;if((u.ctrlKey||u.metaKey)&&!u.shiftKey&&u.key==="z"){u.preventDefault(),_n();return}if((u.ctrlKey||u.metaKey)&&(u.key==="y"||u.shiftKey&&u.key==="Z")){u.preventDefault(),Bn();return}if((u.key==="Delete"||u.key==="Backspace")&&!m&&Tn(),u.key==="F6"&&(u.preventDefault(),typeof window.toggleAlbertPanel=="function"&&window.toggleAlbertPanel()),(u.ctrlKey||u.metaKey)&&u.key==="c"&&!m&&L.size>0){u.preventDefault();const h=[...L];ne=h.map(y=>{const v=x.find(k=>k.id===y),w=document.getElementById(y);return{node:JSON.parse(JSON.stringify(v)),x:w?parseFloat(w.style.left):0,y:w?parseFloat(w.style.top):0}}),ne._internalWires=E.filter(y=>h.includes(y.fromId)&&h.includes(y.toId)).map(y=>JSON.parse(JSON.stringify(y))),I(`Copied ${h.length} node${h.length>1?"s":""}`,"info")}if((u.ctrlKey||u.metaKey)&&u.key==="v"&&!m&&ne.length>0){u.preventDefault(),me();const h=40,y={};ne.forEach(({node:v,x:w,y:k})=>{const B="node-"+Date.now()+"-"+Math.random().toString(36).slice(2,7);y[v.id]=B;const S={...JSON.parse(JSON.stringify(v)),id:B};x.push(S);const T=document.getElementById("flow-canvas"),C=document.createElement("div");C.id=B;const M=document.getElementById(v.id),Z=M?M.className.replace("flow-node","").trim():"";C.className="flow-node"+(Z?" "+Z:""),C.style.left=w+h+"px",C.style.top=k+h+"px",M&&(C.innerHTML=M.innerHTML,C.style.cssText=M.style.cssText,C.style.left=w+h+"px",C.style.top=k+h+"px"),C.id=B,T.appendChild(C),C.onmousedown=M?M.onmousedown:null}),(ne._internalWires||[]).forEach(v=>{const w={...JSON.parse(JSON.stringify(v)),id:"wire-"+Date.now()+"-"+Math.random().toString(36).slice(2,7),fromId:y[v.fromId],toId:y[v.toId]};E.push(w)}),R(),N(),I(`Pasted ${ne.length} node${ne.length>1?"s":""}`,"info")}u.key==="Escape"&&xe()})}window.initCanvasInteraction=vn;const Qt={"Guardrails & Safety":["SafetyGate","OutputGuard","gatekeeper","range_validator","string_validator","validator","float_threshold","watchdog","supervisor","retry"],"Deliberation & Evaluation":["Classifier","Ranker","Proposer","Challenger","Arbiter","deliberator"],"Routing & Aggregation":["ConsensusGate","consensus","aggregator","filter","binary_bridge","router","pipeline","majority_5","weighted_consensus","mapper","transformer"],"Memory & Persistence":["ContextBuffer","EpisodicRecall","StateInjector","sqlite_bridge"],"Sparse Math & Compute":["SparseMatMul","WeightPruner","TernaryQuantizer"],"Interoperability & Protocol":["MCPBridge","LocalNodeSync","VetoOrchestrator"],"I/O & Execution":["Sensor","Actuator","broadcast","echo","logger","scaler"]};let pt={"Guardrails & Safety":!1,"Deliberation & Evaluation":!1,"Routing & Aggregation":!1,"Memory & Persistence":!1,"Sparse Math & Compute":!1,"Interoperability & Protocol":!1,"I/O & Execution":!1};const Zt={"Orchestration & Consensus":["moe_13_flagship","consensus","industry_enterprise_risk","recursive_refiner","kmu_hiring_decision","kmu_supplier_score","kmu_customer_qual"],"Evaluation & Debate":["debate","filter_rank","kmu_process_opt","sensor_gate","industry_sme_pipeline"],"Safety & Guardrails":["guardrail","kmu_invoice_fraud","industry_iot_grid"],"Memory & Persistence":["local_rag_pipeline","episodic_reflection"],"High-Performance Compute":["quantized_sparse_accelerator"],"External Interoperability":["hard_gated_mcp","swarm_consensus"]};let ut={"Orchestration & Consensus":!1,"Evaluation & Debate":!1,"Safety & Guardrails":!1,"Memory & Persistence":!1,"High-Performance Compute":!1,"External Interoperability":!1};const Ie={delayTimer:null,show:function(e,t,n){const o=document.getElementById("global-tooltip");if(!o)return;o.textContent=e,o.style.left=t+15+"px",o.style.top=n+10+"px",o.classList.add("visible");const i=o.getBoundingClientRect();i.right>window.innerWidth&&(o.style.left=t-i.width-15+"px")},hide:function(){this.delayTimer&&clearTimeout(this.delayTimer);const e=document.getElementById("global-tooltip");e&&e.classList.remove("visible")},startDelay:function(e,t,n){this.hide(),this.delayTimer=setTimeout(()=>{this.show(e,t,n)},400)}};window.TooltipController=Ie;const Fe={Sensor:{desc:"The starting point. It reads external data and brings it into the ternary workflow.",code:`fn main() -> trit {
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
}`,icon:"octagon",color:"var(--red)"}};function R(){try{const e={nodes:x.map(t=>{var n,o;return{id:t.id,name:t.name,path:t.path,type:t.type,props:t.props,x:parseInt(((n=document.getElementById(t.id))==null?void 0:n.style.left)||0),y:parseInt(((o=document.getElementById(t.id))==null?void 0:o.style.top)||0),parentId:t.parentId||null,isStub:t.isStub||!1}}),wires:E};localStorage.setItem("ternflow_canvas",JSON.stringify(e))}catch{}}window.saveCanvasState=R;function xn(){try{const e=localStorage.getItem("ternflow_canvas");if(!e)return!1;const t=JSON.parse(e);if(!t.nodes||t.nodes.length===0)return!1;if(t.nodes.forEach(n=>K(n.name,n.path,n.x,n.y,n.type,n.id,n.isStub)),t.nodes.forEach(n=>{const o=x.find(i=>i.id===n.id);o&&(o.props=n.props,o.parentId=n.parentId||null,o.isStub=n.isStub||!1,Ze(n.id))}),E=t.wires||[],N(),x.length>0){const n=document.getElementById("canvas-hint");n&&(n.style.display="none")}return!0}catch{return!1}}window.restoreCanvasState=xn;function jo(){document.querySelectorAll(".flow-node").forEach(o=>o.remove()),document.querySelectorAll(".edge-badge").forEach(o=>o.remove());const e=document.getElementById("flow-svg-layer");e&&(e.innerHTML=""),document.getElementById("wire-handle").classList.remove("active");const t=document.getElementById("scrub-layer");t&&t.getContext("2d").clearRect(0,0,t.width,t.height),window.globalScheduledEvents=[],x=[],E=[],z=null,U=null,L=new Set,j(),N();const n=document.getElementById("canvas-hint");n&&(n.style.display="flex"),localStorage.removeItem("ternflow_canvas"),I("Canvas cleared","ok")}window.clearCanvas=jo;let en=!1;function wn(){if(H=!1,en||(vn(),Hn(),Wn(),re(),en=!0),Se(),x.length===0&&!xn()){const t=document.getElementById("canvas-hint");t&&(t.style.display="flex")}}window.renderFlow=wn;let Bt=[];async function Se(){ke([],"");try{const e=document.getElementById("apiEndpoint").value.replace(/\/$/,""),t=document.getElementById("apiKey").value.trim(),o=await(await fetch(e+"/api/stdlib/list",{headers:t?{"X-Ternlang-Key":t}:{},signal:AbortSignal.timeout(3e3)})).json();let i=o.status==="ok"&&o.files?o.files.filter(r=>r.includes("agents/")):[];i.length>0&&(Bt=i,ke(i),En(i))}catch{}}window.renderFlowLibrary=Se;function bn(e){const t=e.toLowerCase();return t.includes("float_threshold")?{icon:"sliders-horizontal",color:"#22d3ee"}:t.includes("gatekeeper")?{icon:"lock",color:"#fbbf24"}:t.includes("logger")?{icon:"clipboard",color:"#78350f"}:t.includes("majority_5")?{icon:"network",color:"#a855f7"}:t.includes("mapper")?{icon:"grid",color:"#d946ef"}:t.includes("pipeline")?{icon:"rows",color:"#4ade80"}:t.includes("range_validator")?{icon:"frame",color:"#fb923c"}:t.includes("retry")?{icon:"rotate-ccw",color:"#f472b6"}:t.includes("router")?{icon:"git-branch",color:"#1d4ed8"}:t.includes("scaler")?{icon:"maximize",color:"#ffffff"}:t.includes("string_validator")?{icon:"text-cursor",color:"#a3e635"}:t.includes("supervisor")?{icon:"eye",color:"#000000"}:t.includes("transformer")?{icon:"box",color:"#8b5cf6"}:t.includes("validator")?{icon:"shield-check",color:"#15803d"}:t.includes("watchdog")?{icon:"bell",color:"#ef4444"}:t.includes("aggregator")||t.includes("collect")||t.includes("unify")?{icon:"filter",color:"#a855f7"}:t.includes("binary_bridge")||t.includes("cross")||t.includes("legacy")?{icon:"shuffle",color:"#f97316"}:t.includes("broadcast")||t.includes("emit")||t.includes("speak")?{icon:"megaphone",color:"#eab308"}:t.includes("consensus")&&!t.includes("gate")?{icon:"link-2",color:"#14b8a6"}:t.includes("deliberator")||t.includes("think")||t.includes("weight")?{icon:"brain-circuit",color:"#d946ef"}:t.includes("echo")||t.includes("mirror")||t.includes("repeat")?{icon:"waves",color:"#22c55e"}:t.includes("filter")||t.includes("sieve")||t.includes("isolate")?{icon:"layers",color:"#b91c1c"}:t.includes("sensor")||t.includes("input")||t.includes("read")?{icon:"radio",color:"var(--cyan)"}:t.includes("safety")||t.includes("guard")||t.includes("check")?{icon:"shield-check",color:"var(--green)"}:t.includes("consens")||t.includes("vote")||t.includes("aggregate")?{icon:"git-pull-request",color:"var(--blue)"}:t.includes("classif")||t.includes("match")||t.includes("sort")?{icon:"layers",color:"var(--amber)"}:t.includes("actuat")||t.includes("output")?{icon:"zap-off",color:"var(--red)"}:t.includes("rank")||t.includes("score")?{icon:"bar-chart-2",color:"var(--cyan)"}:t.includes("propos")?{icon:"message-square",color:"var(--blue)"}:t.includes("challeng")||t.includes("debat")?{icon:"flame",color:"var(--amber)"}:t.includes("logic")||t.includes("math")||t.includes("calc")?{icon:"variable",color:"var(--cyan)"}:t.includes("finance")||t.includes("econ")||t.includes("price")?{icon:"trending-up",color:"var(--green)"}:t.includes("hardware")||t.includes("cpu")||t.includes("fpga")?{icon:"cpu",color:"var(--cyan)"}:{icon:"bot",color:"var(--blue)"}}window.getAgentIcon=bn;function ke(e,t=""){const n=document.getElementById("flow-lib-items");if(!n)return;n.innerHTML="";const o={};Object.keys(Qt).forEach(r=>o[r]=[]);const i=r=>{for(const[a,s]of Object.entries(Qt))if(s.some(l=>l.toLowerCase()===r.toLowerCase()))return a;return console.warn(`[Taxonomy] Agent "${r}" is not mapped to any strict category. Dropping from render tree.`),null};Object.entries(Fe).forEach(([r,a])=>{if(t&&!r.toLowerCase().includes(t.toLowerCase()))return;const s=i(r);s&&o[s].push({name:r,agent:a,type:"builtin"})}),e.forEach(r=>{const a=r.split("/").pop().replace(".tern","");if(t&&!a.toLowerCase().includes(t.toLowerCase()))return;const s=i(a);s&&o[s].push({name:a,path:r,type:"api"})}),Object.entries(o).forEach(([r,a])=>{if(a.length===0)return;const s=t?!0:pt[r],l=document.createElement("div");l.className="lib-category"+(s?"":" collapsed");const d=document.createElement("div");if(d.className="lib-category-header",d.style.display="flex",d.style.justifyContent="space-between",d.style.alignItems="center",d.innerHTML=`<span>${r}</span><i data-lucide="chevron-down"></i>`,d.onclick=()=>{pt[r]=!pt[r],ke(e,t)},l.appendChild(d),s){const c=document.createElement("div");c.className="lib-category-items",a.forEach(p=>{const g=document.createElement("div");g.className="lib-item";let f,u;if(p.type==="builtin")f=p.agent.icon,u=p.agent.color;else{const m=bn(p.name);f=m.icon,u=m.color}g.draggable=!0,g.ondragstart=m=>{m.dataTransfer.setData("tern-node-type","agent"),m.dataTransfer.setData("tern-node-name",p.name),m.dataTransfer.setData("tern-node-path",p.type==="builtin"?"__builtin__":p.path),p.type==="builtin"&&m.dataTransfer.setData("tern-node-code",p.agent.code)},g.innerHTML=`<i data-lucide="${f}" style="color:${u}"></i> <span>${p.name}</span>`,g.onmouseenter=m=>{const h=p.type==="builtin"?p.agent.desc:"Custom ternary pipeline defined in "+p.path;Ie.startDelay(h,m.clientX,m.clientY)},g.onmouseleave=()=>Ie.hide(),g.onclick=async()=>{const m="node_"+Date.now(),h=ae((Math.random()-.5)*120,(Math.random()-.5)*80);if(K(p.name,p.type==="builtin"?"__builtin__":p.path,h.x,h.y,"agent",m),p.type==="builtin"){const y=x.find(v=>v.id===m);y&&(y.props.code=p.agent.code,y.props.input_schema="signal: trit",y.props.output_schema="signal: trit")}else try{const y=await fetch(tt+p.path);if(y.ok){const v=await y.text(),w=x.find(k=>k.id===m);w&&(w.props.code=v,z===m&&j(),R())}}catch{}},c.appendChild(g)}),l.appendChild(c)}n.appendChild(l)}),n.children.length===0&&(n.innerHTML='<div style="padding:16px; color:var(--muted2); font-size:11px; text-align:center;">No agents found.<br>Use + to create one.</div>'),lucide.createIcons()}window.renderFlowLibItems=ke;function Fo(e){ke(Bt,e)}window.filterFlowLib=Fo;function En(e){const t=document.getElementById("newAgentLibPick");t&&(t.innerHTML='<option value="">— Start blank —</option>',e.forEach(n=>{const o=document.createElement("option");o.value=n,o.textContent=n.split("/").pop().replace(".tern",""),t.appendChild(o)}))}window.populateNewAgentPicker=En;function Ho(){document.getElementById("newAgentModal").style.display="flex",document.getElementById("newAgentName").value="",document.getElementById("newAgentCode").value=`fn main() -> trit {
    return affirm;
}`}window.openNewAgentModal=Ho;function Lt(){document.getElementById("newAgentModal").style.display="none"}window.closeNewAgentModal=Lt;document.addEventListener("keydown",e=>{e.key==="Escape"&&Lt()});async function Wo(){const e=document.getElementById("newAgentLibPick").value;if(!e){document.getElementById("newAgentCode").value=`fn main() -> trit {
    return affirm;
}`;return}try{const t=await fetch(tt+e);t.ok&&(document.getElementById("newAgentCode").value=await t.text())}catch{}}window.loadNewAgentTemplate=Wo;function ae(e=0,t=0){const n=document.getElementById("flow-canvas-wrap");if(!n)return{x:0,y:0};const o=n.getBoundingClientRect(),i=o.width/2+e,r=o.height/2+t;return{x:(i-b.x)/b.scale,y:(r-b.y)/b.scale}}window.viewportCenterInCanvas=ae;function qo(){const e=document.getElementById("newAgentName").value.trim()||"Agent",t=document.getElementById("newAgentCode").value,n="node_"+Date.now(),{x:o,y:i}=ae((Math.random()-.5)*100,(Math.random()-.5)*80);K(e,"__custom__",o,i,"agent",n);const r=x.find(a=>a.id===n);r&&(r.props.code=t),Lt(),I(`Agent "${e}" added`,"ok")}window.addAgentFromModal=qo;const Ko=[{id:"safety",label:"Safety",weight:.15,crit:!0},{id:"metasafety",label:"MetaSafety",weight:.15,crit:!0},{id:"logic",label:"Logic",weight:.08},{id:"ethics",label:"Ethics",weight:.1},{id:"factcheck",label:"FactCheck",weight:.08},{id:"causal",label:"Causal",weight:.07},{id:"context",label:"Context",weight:.07},{id:"history",label:"History",weight:.05},{id:"ambiguity",label:"Ambiguity",weight:.05},{id:"math",label:"Math",weight:.05},{id:"tooluse",label:"ToolUse",weight:.05},{id:"persona",label:"Persona",weight:.05},{id:"efficiency",label:"Efficiency",weight:.05}];function In(e,t,n,o){const r=n*1.1,a=o*1.1;let s=!0,l=t,d=0;for(;s&&d<10;){s=!1;for(const c of x){const p=c.type==="artifact"?300:c.type==="moe13"?320:180,g=c.type==="artifact"?200:c.type==="moe13"?360:80,f=Math.abs(e-c.x)<(r+p)/2,u=Math.abs(l-c.y)<(a+g)/2;if(f&&u){s=!0,l+=150;break}}d++}return{x:e,y:l}}window.findClearSpace=In;function $t(e,t){const n=document.getElementById("flow-canvas-wrap");if(!n)return;const o=n.clientWidth,i=n.clientHeight,r=o/2-e*b.scale,a=i/2-t*b.scale,s=b.x,l=b.y,d=600,c=performance.now();function p(g){const f=g-c,u=Math.min(f/d,1),m=1-Math.pow(1-u,3);b.x=s+(r-s)*m,b.y=l+(a-l)*m,re(),u<1&&requestAnimationFrame(p)}requestAnimationFrame(p)}window.panToCenter=$t;function K(e,t,n,o,i="agent",r,a=!1){var k,B;me();const s=document.getElementById("flow-canvas"),l=document.createElement("div");l.id=r;const d=i==="external"?" external":i==="gate"?" gate":i==="artifact"?" artifact":i==="moe13"?" moe13":i==="datasource"?" datasource":"";l.className="flow-node"+d,a&&l.classList.add("artifact-stub"),i==="datasource"&&(l.style.borderLeft="4px solid #f43f5e",l.style.borderRadius="0 8px 8px 0");const c=i==="artifact"?300:i==="moe13"?320:180,p=i==="artifact"?200:i==="moe13"?360:80;l.style.left=n-c/2+"px",l.style.top=o-p/2+"px",$t(n,o),(i==="artifact"||i==="moe13")&&(l.style.width=c+"px",l.style.height=p+"px",l.style.display="flex",l.style.flexDirection="column");let g="bot",f="var(--cyan)",u="AGENT";i==="external"?(g="zap",f="var(--amber)",u="LLM BRIDGE"):i==="gate"?(g="git-merge",f="var(--cyan)",u="TRIT GATE"):i==="moe13"?(g="brain-circuit",f="var(--magenta)",u="MOE-13 ORCHESTRATOR"):i==="artifact"?(g="file-text",f="var(--green)",u="RESULT ARTIFACT"):i==="datasource"&&(g="database",f="#f43f5e",u="DATA SOURCE");let m=`
    <div style="font-weight:600; color:var(--text); font-size:13px; margin-bottom:3px;" class="fn-title">${e}</div>
    <div style="font-size:10px; color:var(--muted2); font-family:'JetBrains Mono',monospace;">${t.split("/").pop()}</div>
  `;i==="moe13"&&(m=`
       <div style="font-weight:700; color:var(--magenta); font-size:10px; text-transform:uppercase; margin-bottom:10px; letter-spacing:1px; display:flex; justify-content:space-between;">
         <span>Deliberation Axes</span>
         <span id="moe-verdict-${r}" style="color:var(--muted2)">PENDING</span>
       </div>
       <div style="flex:1; display:grid; grid-template-columns: 1fr 40px 40px; gap:4px; font-family:'JetBrains Mono',monospace; font-size:9px;">
         ${Ko.map(S=>`
           <div style="color:var(--text); opacity:0.8;">${S.label}</div>
           <div id="moe-vote-${r}-${S.id}" style="text-align:center; color:var(--muted2); font-weight:700;">0</div>
           <div id="moe-conf-${r}-${S.id}" style="text-align:right; color:var(--cyan);">0%</div>
         `).join("")}
       </div>
       <div id="moe-veto-alert-${r}" style="display:none; margin-top:10px; padding:6px; background:rgba(239,68,68,0.2); border:1px solid var(--red); color:var(--red); font-size:9px; font-weight:800; text-align:center; border-radius:4px;">
         🛑 CRITICAL SAFETY VETO ENGAGED
       </div>
     `),i==="artifact"&&(m=`
       <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:8px;">
         <div style="font-weight:700; color:var(--green); font-size:10px; text-transform:uppercase; letter-spacing:0.5px;">Payload Resolution</div>
         <div class="art-ctrls" style="display:flex; gap:4px;">
           <button class="art-btn" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); setArtifactState('${r}', 'lock')" title="Lock as Static Data"><i data-lucide="lock" style="width:10px"></i></button>
           <button class="art-btn" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); setArtifactState('${r}', 'transmute')" title="Transmute to Editor"><i data-lucide="edit-3" style="width:10px"></i></button>
           <button class="art-btn" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); setArtifactState('${r}', 'extend')" title="Extend Topology"><i data-lucide="external-link" style="width:10px"></i></button>
         </div>
       </div>
       <div id="art-body-${r}" class="art-display" style="flex:1; overflow-y:auto; background:rgba(0,0,0,0.2); padding:8px; border-radius:4px; border:1px solid var(--border2);">
         <pre style="margin:0; font-family:'JetBrains Mono',monospace; font-size:11px; color:var(--text); white-space:pre-wrap;">(Awaiting signal...)</pre>
       </div>
       <textarea id="art-edit-${r}" class="art-editor" style="display:none; flex:1; background:var(--bg2); color:var(--cyan); font-family:'JetBrains Mono',monospace; font-size:11px; border:1px solid var(--cyan); padding:8px; border-radius:4px; outline:none; resize:none;" oninput="updateArtifactPayload('${r}', this.value)"></textarea>
       <div id="art-socket-label-${r}" style="margin-top:8px; display:none; justify-content:flex-end;">
         <div style="font-size:9px; color:var(--green); font-weight:800; border:1px solid var(--green); padding:2px 4px; border-radius:3px;">EXTEND SOCKET ACTIVE</div>
       </div>
     `);const h=i==="external"?`
    <div style="position:absolute; top:-8px; right:10px; background:var(--amber); color:var(--bg1); font-size:8px; font-weight:800; padding:2px 6px; border-radius:10px; box-shadow:0 2px 8px rgba(245,158,11,0.4); z-index:11;">PROBABILISTIC LLM</div>
  `:"";if(l.innerHTML=`
    ${h}
    <div class="fn-head">
      <div style="display:flex; align-items:center; gap:6px; font-size:10px; font-weight:700; color:${f}">
        <i data-lucide="${g}" style="width:12px"></i>
        ${u}
      </div>
      <div class="fn-status" id="status-${r}" title="idle"></div>
      <button onclick="event.stopPropagation(); traceCausalPath('${r}')" style="padding:2px 4px; background:none; border:none; cursor:pointer; color:var(--cyan); line-height:1; margin-left:4px;" title="Causal Trace">🔍</button>
      <button onclick="event.stopPropagation(); deleteNode('${r}')" style="padding:2px 4px; background:none; border:none; cursor:pointer; color:var(--muted); line-height:1; margin-left:4px;" title="Remove">✕</button>
    </div>
    <div class="fn-body" style="${i==="artifact"?"flex:1; display:flex; flex-direction:column; overflow:hidden;":""}">
      ${m}
    </div>
    <div class="fn-injectors">
      <div class="inj-btn inj-pos" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); injectSignal('${r}', 1)" title="Inject +1 Affirm">+1</div>
      <div class="inj-btn inj-zero" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); injectSignal('${r}', 0)" title="Inject 0 Tend">0</div>
      <div class="inj-btn inj-neg" onmousedown="event.stopPropagation()" onclick="event.stopPropagation(); injectSignal('${r}', -1)" title="Inject -1 Reject">-1</div>
    </div>
    ${i!=="datasource"?'<div class="flow-port flow-port-in"  style="left:-7px;  top:50%; margin-top:-6px;" title="Input"></div>':""}
    <!-- Phase 3: Multi-Timeline Ternary Routing Ports -->
    <div class="flow-port flow-port-out port-affirm" style="right:-7px; top:25%; margin-top:-6px; background:var(--green); border-color:var(--green);" title="Affirm (+1)"></div>
    <div class="flow-port flow-port-out port-neutral" style="right:-7px; top:50%; margin-top:-6px; background:var(--muted); border-color:var(--muted);" title="Neutral (0)"></div>
    <div class="flow-port flow-port-out port-reject" style="right:-7px; top:75%; margin-top:-6px; background:var(--red); border-color:var(--red);" title="Reject (-1)"></div>
    ${i==="artifact"?'<div class="inspector-resizer" style="width:10px; height:10px;"></div>':""}
  `,i!=="datasource"&&!a){const S=(f.startsWith("var"),f);S.startsWith("#")?l.style.backgroundColor=S+"33":l.style.backgroundColor=`color-mix(in srgb, ${S}, transparent 80%)`,l.style.borderColor=S}if(i!=="datasource"&&!a&&((B=(k=x.find(S=>S.id===r))==null?void 0:k.props)!=null&&B.customColor)){const S=x.find(C=>C.id===r).props.customColor;l.style.borderColor=S,S.startsWith("#")?l.style.backgroundColor=S+"33":l.style.backgroundColor=`color-mix(in srgb, ${S}, transparent 80%)`,l.style.boxShadow=`0 0 10px ${S}33`;const T=l.querySelector(".fn-head");T&&(T.style.borderBottomColor=S)}l.onmousedown=S=>{S.target.closest("button")||S.target.classList.contains("flow-port")||(!S.shiftKey&&!L.has(r)?We(r,!1):S.shiftKey?We(r,!0):(z=r,j()),me(),de=!0,V=r,Sn=S.clientX,kn=S.clientY,He={},L.forEach(T=>{const C=document.getElementById(T);C&&(He[T]={x:parseFloat(C.style.left)||0,y:parseFloat(C.style.top)||0})}),l.style.zIndex=1e3,S.preventDefault())},l.ondblclick=S=>{i==="macro"&&hn(r)},s.appendChild(l);const v={system_prompt:"",routing:"affirm→next",code:Fe[e]?Fe[e].code:t==="__arch__"?`fn main() -> trit {
    return affirm;
}`:"",input_schema:"signal: trit",output_schema:"signal: trit"};i==="external"&&(v.temperature=.5,v.max_trits=1024,v.provider="openai",v.api_key="",v.mapping="classification",v.template="Evaluate this signal: {{input}}"),x.push({id:r,name:e,path:t,type:i,x:n,y:o,props:v,isStub:a});const w=document.getElementById("canvas-hint");w&&(w.style.display="none"),lucide.createIcons(),R()}window.createFlowNode=K;let L=new Set,He={},de=!1,Sn,kn,V=null;function We(e,t=!1){t||(L.clear(),document.querySelectorAll(".flow-node").forEach(i=>i.classList.remove("selected","selected-multi"))),L.add(e),z=e;const n=document.getElementById(e);n&&n.classList.add(t&&L.size>1?"selected-multi":"selected"),t&&L.size>1&&L.forEach(i=>{const r=document.getElementById(i);r&&(r.classList.remove("selected","selected-multi"),r.classList.add(i===e?"selected":"selected-multi"))});const o=document.getElementById("groupBtn");o&&(o.style.display=L.size>1?"flex":"none"),j()}window.selectNode=We;function xe(){L.clear(),z=null,U=null,document.querySelectorAll(".flow-node").forEach(n=>n.classList.remove("selected","selected-multi")),document.getElementById("wire-handle").classList.remove("active"),et();const e=document.getElementById("prop-header-label");e&&(e.textContent="Node Properties");const t=document.getElementById("groupBtn");t&&(t.style.display="none"),j()}window.clearSelection=xe;function Tn(){if(!(L.size===0&&!U)){if(U){Mn(U);return}L.forEach(e=>Ke(e)),L.clear()}}window.deleteSelected=Tn;const ce=[],qe=[],Uo=50;let vt=!1,ne=[];function At(){return{nodes:x.map(e=>{var t,n;return{id:e.id,name:e.name,path:e.path,type:e.type,props:JSON.parse(JSON.stringify(e.props)),x:parseFloat(((t=document.getElementById(e.id))==null?void 0:t.style.left)||0),y:parseFloat(((n=document.getElementById(e.id))==null?void 0:n.style.top)||0),parentId:e.parentId||null,isStub:e.isStub||!1}}),wires:JSON.parse(JSON.stringify(E))}}function me(){vt||(ce.push(At()),ce.length>Uo&&ce.shift(),qe.length=0)}function Cn(e){vt=!0,document.querySelectorAll(".flow-node").forEach(i=>i.remove()),document.querySelectorAll(".edge-badge").forEach(i=>i.remove());const t=document.getElementById("flow-svg-layer");t&&(t.innerHTML="");const n=document.getElementById("scrub-layer");n&&n.getContext("2d").clearRect(0,0,n.width,n.height),x=[],E=[],z=null,U=null,L=new Set,e.nodes.forEach(i=>K(i.name,i.path,i.x,i.y,i.type,i.id,i.isStub)),e.nodes.forEach(i=>{const r=x.find(a=>a.id===i.id);r&&(r.props=JSON.parse(JSON.stringify(i.props)),r.parentId=i.parentId||null,Ze(i.id))}),E=JSON.parse(JSON.stringify(e.wires)),N(),j();const o=document.getElementById("canvas-hint");o&&(o.style.display=x.length===0?"flex":"none"),R(),vt=!1}function _n(){if(ce.length===0){I("Nothing to undo","info");return}qe.push(At()),Cn(ce.pop()),I("Undo","ok")}function Bn(){if(qe.length===0){I("Nothing to redo","info");return}ce.push(At()),Cn(qe.pop()),I("Redo","ok")}window.undoStep=_n;window.redoStep=Bn;function Go({errors:e,warnings:t}){const n=document.getElementById("validateBadge");if(!n)return;const o=e.length+t.length;if(o===0){n.style.display="none";return}n.style.display="flex",n.textContent=o>9?"!":o,n.style.background=e.length>0?"var(--red)":"var(--amber)"}window.updateValidateBadge=Go;function Vo(e,t){const n=x.find(r=>r.id===e);if(!n)return;const i={sensor:{name:"Sensor",code:`fn main() -> trit {
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
}`,icon:"brain-circuit"}}[t];if(i){n.name=i.name,n.props.code=i.code;const r=document.getElementById(e);if(r){const a=r.querySelector(".fn-title");a&&(a.textContent=i.name);const s=r.querySelector(".fn-head i");s&&s.setAttribute("data-lucide",i.icon),lucide.createIcons()}j(),R(),I(`Morphed to ${i.name}`,"ok")}}window.morphNodeArchetype=Vo;function Ln(e){const t=x.find(o=>o.id===e);if(!t||t.type!=="artifact")return;t.isStub=!t.isStub;const n=document.getElementById(e);n&&(t.isStub?n.classList.add("artifact-stub"):n.classList.remove("artifact-stub"),n.querySelectorAll(".flow-port").forEach(i=>{i.style.top="50%",i.style.marginTop="-6px"}),N()),z===e&&j(),R()}window.collapseArtifactToStub=Ln;function Ke(e){me(),x=x.filter(i=>i.id!==e),E=E.filter(i=>i.fromId!==e&&i.toId!==e);const t=document.getElementById(e);t&&t.remove(),L.delete(e),z===e&&(z=L.size>0?[...L][L.size-1]:null,j());const n=document.getElementById("canvas-hint");n&&(n.style.display=x.length===0?"flex":"none");const o=document.getElementById("scrub-layer");o&&o.getContext("2d").clearRect(0,0,o.width,o.height),x.length===0&&(window.globalScheduledEvents=[]),N(),R()}window.deleteNode=Ke;function Yo(e,t){if(!t)return;const n=new FileReader;n.onload=o=>{const i=o.target.result,r=x.find(a=>a.id===e);r&&(r.props.payload=i,R(),z===e&&j(),I(`Ingested ${t.name} (${i.length} bytes)`,"ok"))},n.readAsText(t)}window.handleDataSourceFileUpload=Yo;async function Jo(e){const t=x.find(o=>o.id===e),n=document.getElementById(`sql-input-${e}`).value;if(n){t.props.sql_query=n,I("Executing SQL Query...","ok");try{const i=await(await fetch("/api/data/query",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({sql:n})})).json();i.status==="ok"?(t.props.payload=JSON.stringify(i.results,null,2),R(),z===e&&j(),I(`Query successful. ${i.results.length} rows injected.`,"ok")):I(`SQL Error: ${i.error}`,"err")}catch(o){I(`Bridge Error: ${o.message}`,"err")}}}window.runSqlQuery=Jo;async function Xo(e){if(e){I("Generating Causal Artifact...","ok");try{const t=await fetch(`/api/data/artifact/${e}`);if(!t.ok)throw new Error("Artifact not found in session memory.");const n=await t.text(),o=new Blob([n],{type:"text/markdown"}),i=URL.createObjectURL(o),r=document.createElement("a");r.href=i,r.download=`causal-artifact-${e.substring(0,8)}.md`,document.body.appendChild(r),r.click(),document.body.removeChild(r),URL.revokeObjectURL(i)}catch(t){I(t.message,"err")}}}window.downloadCausalArtifact=Xo;const Qo={render(e,t,n,o,i){e.type==="external"?this.renderLLMSemantic(e,t,n,o,i):e.type==="artifact"?this.renderArtifactSemantic(e,t,n,o,i):e.type==="datasource"?this.renderDataSourceSemantic(e,t,n,o,i):this.renderSemantic(e,t,n,o,i)},renderArtifactSemantic(e,t,n,o,i){const r=e.props.payload||"(No payload data resolved yet)";t.innerHTML=`
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
          <input type="file" id="datasource-file-input" style="display:none;" accept=".txt,.md,.csv,.json,.yaml,.yml" onchange="handleDataSourceFileUpload('${e.id}', this.files[0])">
        </div>
        <div class="prop-label-strict">Data Type</div>
        <select class="prop-input" onchange="updateNodeProp('data_type', this.value)">
          <option value="text" ${r==="text"?"selected":""}>Raw Text</option>
          <option value="json" ${r==="json"?"selected":""}>JSON Array</option>
          <option value="yaml" ${r==="yaml"?"selected":""}>YAML Logic</option>
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
    `,lucide.createIcons()},renderLegacy(e,t,n,o,i){const r=e.props.routing??"affirm→next",a=(e.props.code||"").replace(/</g,"&lt;").replace(/>/g,"&gt;"),s=(e.props.input_schema||"").replace(/</g,"&lt;").replace(/>/g,"&gt;"),l=(e.props.output_schema||"").replace(/</g,"&lt;").replace(/>/g,"&gt;");t.innerHTML=`
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
            <input class="prop-input" style="width:100%;font-size:10px;font-family:'JetBrains Mono',monospace;" value="${l}"
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
    `},renderSemantic(e,t,n,o,i){const r=e.props.system_prompt||e.props.intent||"",a=(e.props.input_schema||"").replace(/</g,"&lt;").replace(/>/g,"&gt;"),s=(e.props.output_schema||"").replace(/</g,"&lt;").replace(/>/g,"&gt;"),l=e.props.timeout??5e3,d=e.props.retries??3,c=e.props.execution_target??"local",p=e.props.customColor||"#38bdf8";t.innerHTML=`
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
            <input type="number" class="prop-input-strict" value="${l}" oninput="updateNodeProp('timeout', parseInt(this.value))">
          </div>
          <div>
            <div class="prop-label-strict" style="font-size:9px;">Max Retries</div>
            <input type="number" class="prop-input-strict" value="${d}" oninput="updateNodeProp('retries', parseInt(this.value))">
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
            <input type="color" id="color-pick-${e.id}" class="prop-input-strict" style="width:34px; padding:0; border:1px solid var(--border2); height:24px; cursor:pointer; background:none; border-radius:4px;" value="${p}" oninput="previewNodeColor('${e.id}', this.value)" onchange="updateNodeColor('${e.id}', this.value)" title="Custom Color">
            <code id="color-hex-${e.id}" style="font-size:11px; color:var(--text); font-family:'JetBrains Mono',monospace; opacity:0.8; letter-spacing:0.5px;">${p.toUpperCase()}</code>
          </div>
        </div>
        <div style="display:flex; gap:6px;">
          <button class="btn-pill" style="flex:1; background:var(--bg2);" onclick="openAgentInEditor()">Editor</button>
          <button class="btn-pill" style="flex:1; background:rgba(239,68,68,0.1); color:var(--red);" onclick="deleteNode('${e.id}')">Remove</button>
        </div>
      </div>
    `,lucide.createIcons()},renderLLMSemantic(e,t,n,o,i){const r=e.props.system_prompt||"",a=e.props.protocol||"openai",s=e.props.model_id||"",l=e.props.base_url||"",d=e.props.temperature??.5,c=e.props.max_trits??1024,p=e.props.timeout??1e4,g=e.props.retries??2,f=e.props.customColor||"#f59e0b",u=_e(),m=e.props.api_key||u[a]||"";m&&!e.props.api_key&&(e.props.api_key=m);const h=a==="openai"||a==="webhook";t.innerHTML=`
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
            <input type="number" class="prop-input-strict" value="${g}" oninput="updateNodeProp('retries', parseInt(this.value))">
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

        ${h?`
        <div style="margin-bottom:8px;">
          <div class="prop-label-strict" style="font-size:9px; color:var(--muted2);">Base URL</div>
          <input type="text" class="prop-input-strict" value="${l}" placeholder="https://api.openai.com/v1" oninput="updateNodeProp('base_url', this.value)">
        </div>
        `:""}

        <div style="margin-bottom:8px;">
          <div class="prop-label-strict" style="font-size:9px; color:var(--muted2);">Provider API Key</div>
          <input type="password" class="prop-input-strict" value="${m}" placeholder="Linked to ${a} vault" oninput="updateNodeProp('api_key', this.value)">
        </div>
      </div>

      <div class="prop-section">
        <div class="prop-label-strict">Hyperparameters</div>
        <div style="display:grid; grid-template-columns:1fr 1fr; gap:8px;">
          <div>
            <div id="lbl-temp-${e.id}" style="font-size:9px; color:var(--muted2); margin-bottom:2px;">TEMP: ${d}</div>
            <input type="range" min="0" max="2" step="0.01" value="${d}" style="width:100%" oninput="updateNodeProp('temperature', parseFloat(this.value)); document.getElementById('lbl-temp-${e.id}').textContent='TEMP: '+parseFloat(this.value).toFixed(2)">
          </div>
          <div>
            <div id="lbl-tokens-${e.id}" style="font-size:9px; color:var(--muted2); margin-bottom:2px;">TOKENS: ${c}</div>
            <input type="range" min="128" max="16384" step="64" value="${c}" style="width:100%" oninput="updateNodeProp('max_trits', parseInt(this.value)); document.getElementById('lbl-tokens-${e.id}').textContent='TOKENS: '+this.value">
          </div>
        </div>
      </div>

      <div style="margin-top:auto; padding-top:12px; border-top:1px solid var(--border2); display:flex; flex-direction:column; gap:12px;">
        <div style="display:flex; flex-direction:column; gap:4px;">
          <div style="color:white; font-weight:bold; font-size:10px; text-transform:uppercase; letter-spacing:0.05em;">Custom Color</div>
          <div style="display:flex; align-items:center; gap:6px;">
            <input type="color" id="color-pick-${e.id}" class="prop-input-strict" style="width:34px; padding:0; border:1px solid var(--border2); height:24px; cursor:pointer; background:none; border-radius:4px;" value="${f}" oninput="previewNodeColor('${e.id}', this.value)" onchange="updateNodeColor('${e.id}', this.value)" title="Custom Color">
            <code id="color-hex-${e.id}" style="font-size:11px; color:var(--text); font-family:'JetBrains Mono',monospace; opacity:0.8; letter-spacing:0.5px;">${f.toUpperCase()}</code>
          </div>
        </div>
        <div style="display:flex; gap:6px;">
          <button class="btn-pill" style="flex:1; background:var(--bg2);" onclick="openAgentInEditor()">Editor</button>
          <button class="btn-pill" style="flex:1; background:rgba(239,68,68,0.1); color:var(--red);" onclick="deleteNode('${e.id}')">Remove</button>
        </div>
      </div>
    `,lucide.createIcons()},updateInlineSummary(e,t){const n=document.querySelector(`#${e.id} .fn-title`);if(n&&!n.dataset.semantic){n.dataset.semantic="true";const i=document.createElement("div");i.className="semantic-desc",i.style="font-size:9px; color:var(--muted2); font-weight:400; max-height:24px; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; margin-top:2px;",n.parentNode.insertBefore(i,n.nextSibling)}const o=document.querySelector(`#${e.id} .semantic-desc`);o&&(o.textContent=t?`→ ${t}`:e.type==="external"?"→ Waiting for prompt...":"→ (No intent defined)")}},Zo={render(e,t,n,o){this.renderSemantic(e,t,n,o)},renderLegacy(e,t,n,o){const i=String(e.condition||"all"),r=e.transform||"pass",a=e.label||"";t.innerHTML=`
      <div style="font-size:10px;color:var(--muted2);margin-bottom:4px;">
        ${n?n.name:"?"} → ${o?o.name:"?"}
      </div>

      <div class="prop-group">
        <label class="prop-label">Pass condition</label>
        <select class="prop-input" style="width:100%" onchange="updateWireProp('condition', this.value)">
          <option value="all" ${i==="all"?"selected":""}>All trits (pass everything)</option>
          <option value="1"   ${i==="1"?"selected":""}>affirm only (+1)</option>
          <option value="0"   ${i==="0"?"selected":""}>tend only (0)</option>
          <option value="-1"  ${i==="-1"?"selected":""}>reject only (-1)</option>
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
    `},renderSemantic(e,t,n,o){const i=e.condition||"all",r=e.transform||"pass",a=e.priority||5,s=e.weight||1,l=e.latency||0,d=e.customColor||"#94a3b8";t.innerHTML=`
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
          <button title="value == +1" class="rail-btn ${String(i)==="1"?"active":""}" onclick="updateWireProp('condition','1');updateWireProp('label','+1 only');">+1</button>
          <button title="value == 0"  class="rail-btn ${String(i)==="0"?"active":""}"   onclick="updateWireProp('condition','0');updateWireProp('label','0 only');">0</button>
          <button title="value == -1" class="rail-btn ${String(i)==="-1"?"active":""}" onclick="updateWireProp('condition','-1');updateWireProp('label','-1 only');">-1</button>
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
               <input type="number" class="prop-input-strict" style="height:20px; font-size:9px; padding:2px 4px;" value="${l}" oninput="updateWireProp('latency', parseInt(this.value))">
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
          <code id="color-hex-${e.id}" style="font-size:10px; color:var(--muted2);">${d}</code>
          <input type="color" id="color-pick-${e.id}" class="prop-input-strict" style="width:24px; padding:0; border:none; height:20px; cursor:pointer; background:none;" value="${d}" oninput="previewWireColor('${e.id}', this.value)" onchange="updateWireColor('${e.id}', this.value)" title="Custom Wire Color">
        </div>
      </div>

      <div style="padding-top:8px;">
        <button class="btn btn-ghost" style="width:100%; height:28px; font-size:11px; color:var(--red); border:1px solid rgba(239, 68, 68, 0.1);" onclick="deleteWire('${e.id}')">Remove Edge</button>
      </div>
    `,lucide.createIcons()}};function ei(){const e=document.getElementById("helpPopover"),t=document.getElementById("helpPopoverTitle"),n=document.getElementById("helpPopoverContent"),o=document.getElementById("prop-header-label");if(!e||!t||!n||!o)return;o.textContent.toLowerCase().includes("edge")?(t.innerHTML='<i data-lucide="git-branch" style="width:16px; color:var(--amber)"></i> What is an Edge?',n.innerHTML=`
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
    `),e.style.display="block",window.lucide&&lucide.createIcons()}window.showHelpCard=ei;function ti(){const e=document.getElementById("helpPopover");e&&(e.style.display="none")}window.closeHelpCard=ti;function j(){const e=document.getElementById("prop-body"),t=document.getElementById("prop-header-label"),n=document.getElementById("prop-help-icon");if(!z){t&&(t.textContent="Node Properties"),n&&(n.style.display="flex"),e.innerHTML=`
      <div style="color:var(--muted); font-size:12px; text-align:center; margin-top:40px; padding:0 12px; line-height:1.8;">
        Select a node to configure<br>
        <span style="font-size:10px; color:var(--muted2);">Drag output port → input port to wire agents</span>
      </div>`;return}const o=x.find(s=>s.id===z);if(!o)return;t&&(t.textContent=o.type==="macro"?"MACRO PROPERTIES":"NODE PROPERTIES"),n&&(n.style.display="flex");const i=E.filter(s=>s.toId===z),r=E.filter(s=>s.fromId===z),a=i.some(s=>{const l=x.find(d=>d.id===s.fromId);return l&&l.props.output_schema&&o.props.input_schema&&l.props.output_schema!==o.props.input_schema})?'<div style="font-size:10px;color:var(--amber);padding:6px 0;">⚠ Schema mismatch on incoming wire</div>':"";Qo.render(o,e,i,r,a)}function ni(e,t){const n=x.find(i=>i.id===e);if(!n)return;n.props.customColor=t;const o=document.getElementById(e);if(o){o.style.borderColor=t,o.style.backgroundColor=t+"33",o.style.boxShadow=`0 0 10px ${t}33`;const i=o.querySelector(".fn-head");i&&(i.style.borderBottomColor=t)}R()}window.updateNodeColor=ni;function oi(e,t){const n=x.find(r=>r.id===e);if(!n)return;n.props.customColor=t;const o=document.getElementById(e);if(o){o.style.borderColor=t,o.style.backgroundColor=t+"33",o.style.boxShadow=`0 0 10px ${t}33`;const r=o.querySelector(".fn-head");r&&(r.style.borderBottomColor=t)}const i=document.getElementById("color-hex-"+e);i&&(i.textContent=t.toUpperCase())}window.previewNodeColor=oi;function ii(e,t){const n=E.find(i=>i.id===e);if(!n)return;n.customColor=t;const o=document.querySelector(`path[id="${e}"]`);o&&(o.style.stroke=t),R()}window.updateWireColor=ii;function ri(e,t){const n=E.find(r=>r.id===e);if(!n)return;n.customColor=t;const o=document.querySelector(`path[id="${e}"]`);o&&(o.style.stroke=t);const i=document.getElementById("color-hex-"+e);i&&(i.textContent=t.toUpperCase())}window.previewWireColor=ri;window.updatePropertyPanel=j;function ai(){const e=x.find(o=>o.id===z);if(!e)return;const t=`flow/${e.name.replace(/\s+/g,"_")}.tern`,n=e.props.code||`fn main() -> trit {
    // ${e.name}
    return affirm;
}`;P[t]=n,X(t,n),Y("editor")}window.openAgentInEditor=ai;function Ze(e){const t=x.find(a=>a.id===e);if(!t)return;const n=document.getElementById(e);if(!n)return;let o=n.querySelector(".fn-schema");o||(o=document.createElement("div"),o.className="fn-schema",n.querySelector(".fn-body").appendChild(o));const i=t.props.input_schema?`<span style="color:var(--green)">▶ ${t.props.input_schema}</span>`:"",r=t.props.output_schema?`<span style="color:var(--cyan)">◀ ${t.props.output_schema}</span>`:"";o.innerHTML=[i,r].filter(Boolean).join("<br>")}window.updateNodeSchemaDisplay=Ze;function si(e,t){const n=x.find(o=>o.id===z);if(n){if(e==="name"){n.name=t;const o=document.querySelector(`#${n.id} .fn-title`);o&&(o.textContent=t)}else n.props[e]=t;if(n.type==="external"&&e==="api_key"){const o=n.props.protocol||"openai";Wt(o,t)}R()}}window.updateNodeProp=si;function li(e,t){const n=x.find(i=>i.id===e);if(!n)return;n.props.protocol=t;const o=_e();n.props.api_key=o[t]||"",j(),R()}window.updateBridgeProtocol=li;function di(){const e="bridge_"+Date.now(),t=ae((Math.random()-.5)*100,(Math.random()-.5)*80);K("LLM Bridge","external",t.x,t.y,"external",e)}window.addExternalBridge=di;function ci(){const e="gate_"+Date.now(),t=ae((Math.random()-.5)*100,(Math.random()-.5)*80);K("Consensus Gate","gate",t.x,t.y,"gate",e)}window.addTernaryGate=ci;function pi(){const e="data_"+Date.now(),t=ae((Math.random()-.5)*100,(Math.random()-.5)*80);K("Data Source","source",t.x,t.y,"datasource",e)}window.addDataSource=pi;function ui(e){document.querySelectorAll(".lib-tab").forEach(n=>n.classList.remove("active"));const t=document.getElementById("libtab-"+e);t&&t.classList.add("active"),document.getElementById("lib-panel-agents").style.display=e==="agents"?"flex":"none",document.getElementById("lib-panel-archetypes").style.display=e==="archetypes"?"flex":"none",e==="archetypes"&&Mt()}window.switchLibTab=ui;const $n=[{id:"moe_13_flagship",name:"MoE-13 Flagship",desc:"The ultimate AI brain. It uses 13 specialized experts working together to solve incredibly complex problems.",icon:"layers",color:"var(--cyan)",nodes:[{name:"Orchestrator",type:"agent",dx:440,dy:160},{name:"Expert_01",type:"agent",dx:40,dy:20},{name:"Expert_02",type:"agent",dx:40,dy:90},{name:"Expert_03",type:"agent",dx:40,dy:160},{name:"Expert_04",type:"agent",dx:40,dy:230},{name:"Expert_05",type:"agent",dx:40,dy:300},{name:"Consensus",type:"gate",dx:240,dy:160},{name:"Decision",type:"gate",dx:640,dy:160},{name:"Feedback",type:"agent",dx:440,dy:300},{name:"Context Source",type:"datasource",dx:-160,dy:160,props:{payload:`# TIS GROUNDING
- Mode: MoE-13
- Logic: Balanced Ternary`,data_type:"markdown"}},{name:"Expert Coordinator",type:"external",dx:40,dy:400}],wires:[[1,6],[2,6],[3,6],[4,6],[5,6],[6,0],[0,7],[7,8],[8,0],[9,10],[10,6]],feedbackWires:[8],edgeConds:["all","all","all","all","all","affirm","affirm","tend","all","all","all"]},{id:"consensus",name:"Consensus Pipeline",desc:"A team of voters. Multiple agents look at the same data and use majority rule to make a safe decision.",icon:"git-merge",color:"var(--green)",nodes:[{name:"Sensor A",type:"agent",dx:60,dy:60},{name:"Sensor B",type:"agent",dx:60,dy:180},{name:"Sensor C",type:"agent",dx:60,dy:300},{name:"Consensus Gate",type:"gate",dx:320,dy:180},{name:"Actuator",type:"agent",dx:540,dy:180},{name:"Ref Data",type:"datasource",dx:60,dy:420,props:{payload:`# CONSENSUS REF
- Majority: 2/3
- Veto: -1`,data_type:"markdown"}},{name:"Audit Bridge",type:"external",dx:320,dy:420}],wires:[[0,3],[1,3],[2,3],[3,4],[5,6],[6,3]],edgeConds:["affirm","all","reject","affirm","all","all"]},{id:"guardrail",name:"Guardrail Chain",desc:"A high-security pipeline. It checks data before and after the AI processes it to ensure absolute safety.",icon:"shield-check",color:"var(--amber)",nodes:[{name:"Input",type:"agent",dx:40,dy:160},{name:"Safety Gate",type:"gate",dx:240,dy:160},{name:"LLM Bridge",type:"external",dx:440,dy:80},{name:"Output Guard",type:"gate",dx:440,dy:240},{name:"Output",type:"agent",dx:640,dy:160},{name:"Policy Source",type:"datasource",dx:440,dy:-40,props:{payload:`# SAFETY POLICY
- No PII leak
- Respect Veto`,data_type:"markdown"}}],wires:[[0,1],[1,2],[1,3],[2,4],[3,4],[5,2]],edgeConds:["all","affirm","reject","affirm","affirm","all"]},{id:"filter_rank",name:"Filter → Rank → Decide",desc:"The sorting machine. It quickly filters out bad options and ranks the good ones to find the winner.",icon:"funnel",color:"var(--cyan)",nodes:[{name:"Raw Signal",type:"agent",dx:40,dy:160},{name:"Filter",type:"gate",dx:240,dy:160},{name:"Ranker",type:"agent",dx:440,dy:160},{name:"Decision",type:"gate",dx:640,dy:160}],wires:[[0,1],[1,2],[2,3]],edgeConds:["all","affirm","all"]},{id:"debate",name:"Multi-Agent Debate",desc:"A virtual courtroom. One agent proposes an idea, another attacks it, and a judge decides the winner.",icon:"message-square",color:"var(--muted)",nodes:[{name:"Proposer",type:"agent",dx:60,dy:80},{name:"Challenger",type:"agent",dx:60,dy:260},{name:"Arbiter",type:"gate",dx:300,dy:170},{name:"Accept",type:"agent",dx:520,dy:80},{name:"Reject",type:"agent",dx:520,dy:260}],wires:[[0,2],[1,2],[2,3],[2,4]],edgeConds:["all","all","affirm","reject"]},{id:"sensor_gate",name:"Sensor → Gate → Actuator",desc:"The simplest AI workflow. It reads data, makes one decision, and takes an action.",icon:"cpu",color:"var(--blue)",nodes:[{name:"Sensor",type:"agent",dx:60,dy:160},{name:"Gate",type:"gate",dx:280,dy:160},{name:"Actuator",type:"agent",dx:500,dy:80},{name:"Fallback",type:"agent",dx:500,dy:240}],wires:[[0,1],[1,2],[1,3]],edgeConds:["all","affirm","reject"]},{id:"kmu_process_opt",name:"KMU: Process Optimization Loop",desc:"An automated manager. It constantly analyzes a business process and loops back to improve it.",icon:"refresh-cw",color:"var(--amber)",nodes:[{name:"IST Capture",type:"agent",dx:40,dy:80},{name:"Analysis Gate",type:"gate",dx:240,dy:160},{name:"SOLL Design",type:"agent",dx:440,dy:80},{name:"Test Run",type:"gate",dx:640,dy:160},{name:"Tracking",type:"agent",dx:840,dy:160},{name:"Logger",type:"agent",dx:240,dy:280}],wires:[[0,1],[1,2],[2,3],[3,4],[1,5],[3,1]],feedbackWires:[5],edgeConds:["all","affirm","all","affirm","reject","reject"]},{id:"kmu_supplier_score",name:"KMU: Supplier Scoring",desc:"The purchasing agent. It automatically grades suppliers based on price, quality, and delivery speed.",icon:"truck",color:"var(--blue)",nodes:[{name:"Input Validator",type:"agent",dx:40,dy:160},{name:"Price Check",type:"agent",dx:240,dy:40},{name:"Quality Check",type:"agent",dx:240,dy:120},{name:"Delivery Check",type:"agent",dx:240,dy:200},{name:"Consensus",type:"gate",dx:440,dy:160},{name:"Decision Gate",type:"gate",dx:640,dy:160},{name:"Logger",type:"agent",dx:640,dy:280}],wires:[[0,1],[0,2],[0,3],[1,4],[2,4],[3,4],[4,5],[4,6]],edgeConds:["all","all","all","all","all","all","affirm","tend"]},{id:"kmu_customer_qual",name:"KMU: Customer Qualification",desc:"The sales assistant. It checks incoming leads to see if they match your ideal customer profile.",icon:"users",color:"var(--cyan)",nodes:[{name:"Input Validator",type:"agent",dx:40,dy:160},{name:"Budget Check",type:"agent",dx:240,dy:60},{name:"Industry Fit",type:"agent",dx:240,dy:160},{name:"Engagement",type:"agent",dx:240,dy:260},{name:"Aggregation",type:"gate",dx:440,dy:160},{name:"Routing Gate",type:"gate",dx:640,dy:160},{name:"Sales / Nurture",type:"agent",dx:840,dy:160}],wires:[[0,1],[0,2],[0,3],[1,4],[2,4],[3,4],[4,5],[5,6]],edgeConds:["all","all","all","all","all","all","affirm","!reject"]},{id:"kmu_invoice_fraud",name:"KMU: Invoice Fraud Detection",desc:"The accountant. It scans incoming invoices for weird numbers or mismatched vendor details.",icon:"file-warning",color:"var(--red)",nodes:[{name:"Input Validation",type:"agent",dx:40,dy:160},{name:"Amount Deviation",type:"agent",dx:240,dy:60},{name:"Vendor Match",type:"agent",dx:240,dy:160},{name:"Pattern Detect",type:"agent",dx:240,dy:260},{name:"Consensus",type:"gate",dx:440,dy:160},{name:"Decision Gate",type:"gate",dx:640,dy:160},{name:"Alert Logger",type:"agent",dx:640,dy:280}],wires:[[0,1],[0,2],[0,3],[1,4],[2,4],[3,4],[4,5],[5,6]],edgeConds:["all","all","all","all","all","all","affirm","affirm"]},{id:"kmu_hiring_decision",name:"KMU: Hiring Decision System",desc:"The HR screener. It reads resumes and scores candidates to save you time.",icon:"briefcase",color:"var(--green)",nodes:[{name:"Input Validation",type:"agent",dx:40,dy:160},{name:"Experience Check",type:"agent",dx:240,dy:40},{name:"Skills Match",type:"agent",dx:240,dy:120},{name:"CV Analysis",type:"external",dx:240,dy:200},{name:"Test Score",type:"agent",dx:240,dy:280},{name:"Consensus",type:"gate",dx:440,dy:160},{name:"Decision Gate",type:"gate",dx:640,dy:160}],wires:[[0,1],[0,2],[0,3],[0,4],[1,5],[2,5],[3,5],[4,5],[5,6]],edgeConds:["all","all","all","all","all","all","all","all","affirm"]},{id:"industry_sme_pipeline",name:"SME: Precision Data Pipeline",desc:"A quick data processor. It grabs numbers, checks if they cross a line, and spits out a report.",icon:"rows",color:"#4ade80",nodes:[{name:"Raw_Input",type:"agent",dx:40,dy:160},{name:"Float_Threshold",type:"gate",dx:240,dy:160},{name:"Logger",type:"agent",dx:440,dy:60},{name:"Report_Emit",type:"agent",dx:440,dy:260}],wires:[[0,1],[1,2],[1,3]],edgeConds:["all","all","affirm"]},{id:"industry_enterprise_risk",name:"Enterprise: Risk Assessment Swarm",desc:"A corporate risk team. It sends data through multiple legal and financial checks before moving forward.",icon:"eye",color:"#000000",nodes:[{name:"Legal_Audit",type:"agent",dx:40,dy:60},{name:"Finance_Vetting",type:"agent",dx:40,dy:260},{name:"Majority_5",type:"gate",dx:280,dy:160},{name:"Supervisor",type:"agent",dx:520,dy:160},{name:"Gatekeeper",type:"gate",dx:760,dy:160},{name:"Final_Verdict",type:"agent",dx:980,dy:160}],wires:[[0,2],[1,2],[2,3],[3,4],[4,5]],edgeConds:["all","all","affirm","affirm","affirm"]},{id:"recursive_refiner",name:"Recursive Multi-Stage Refiner",desc:"The perfectionist. It loops a task over and over until the AI is 100 percent sure it got it right.",icon:"refresh-ccw",color:"var(--amber)",nodes:[{name:"Raw Entry",type:"agent",dx:40,dy:160},{name:"Stage 1: Vetting",type:"agent",dx:240,dy:60},{name:"Stage 2: Audit",type:"agent",dx:240,dy:260},{name:"Consensus Hub",type:"gate",dx:480,dy:160},{name:"Refinement Loop",type:"agent",dx:480,dy:340},{name:"Output Guard",type:"gate",dx:720,dy:160},{name:"Final Emission",type:"agent",dx:960,dy:160}],wires:[[0,1],[0,2],[1,3],[2,3],[3,4],[4,3],[3,5],[5,6]],feedbackWires:[5],edgeConds:["all","all","all","all","tend","all","affirm","affirm"]},{id:"industry_iot_grid",name:"Industrial: IoT Sensor Grid",desc:"The factory monitor. It watches live sensors and hits the emergency stop if things look dangerous.",icon:"bell",color:"#ef4444",nodes:[{name:"Mesh_Node_A",type:"agent",dx:40,dy:60},{name:"Mesh_Node_B",type:"agent",dx:40,dy:260},{name:"Range_Validator",type:"gate",dx:280,dy:160},{name:"Watchdog",type:"agent",dx:520,dy:160},{name:"SCADA_Emit",type:"agent",dx:760,dy:60},{name:"Emergency_Stop",type:"agent",dx:760,dy:260}],wires:[[0,2],[1,2],[2,3],[3,4],[3,5]],edgeConds:["all","all","all","affirm","reject"]},{id:"local_rag_pipeline",name:"Local RAG Pipeline",desc:"A private researcher. It reads your local files to answer questions without sending data to the cloud.",icon:"database",color:"var(--cyan)",nodes:[{name:"Input_Signal",type:"agent",dx:40,dy:160},{name:"SQLite_Bridge",type:"agent",dx:240,dy:160},{name:"Context_Buffer",type:"agent",dx:440,dy:160},{name:"Evaluator",type:"agent",dx:640,dy:160}],wires:[[0,1],[1,2],[2,3]],edgeConds:["all","affirm","all"]},{id:"episodic_reflection",name:"Episodic Reflection Loop",desc:"A self-improving AI. It looks at its past mistakes and automatically corrects itself over time.",icon:"history",color:"var(--amber)",nodes:[{name:"Processor",type:"agent",dx:40,dy:160},{name:"Episodic_Recall",type:"agent",dx:240,dy:260},{name:"State_Injector",type:"agent",dx:240,dy:60},{name:"Decision_Gate",type:"gate",dx:440,dy:160}],wires:[[0,1],[1,2],[2,0],[0,3]],feedbackWires:[2],edgeConds:["all","all","tend","affirm"]},{id:"quantized_sparse_accelerator",name:"Quantized Sparse Accelerator",desc:"The speed demon. It strips away useless data so the AI can run incredibly fast on weak hardware.",icon:"zap",color:"var(--cyan)",nodes:[{name:"Quantizer",type:"agent",dx:40,dy:160},{name:"Sparse_Core",type:"agent",dx:240,dy:160},{name:"Weight_Filter",type:"gate",dx:440,dy:160},{name:"Accelerated_Out",type:"agent",dx:640,dy:160}],wires:[[0,1],[1,2],[2,3]],edgeConds:["all","all","!tend"]},{id:"hard_gated_mcp",name:"Hard-Gated MCP Bridge",desc:"A safe tool-user. It lets the AI use external apps but will instantly pull the plug if it acts weird.",icon:"shield-alert",color:"var(--red)",nodes:[{name:"MCP_Bridge",type:"agent",dx:40,dy:160},{name:"Veto_Orchestrator",type:"agent",dx:240,dy:160},{name:"Safe_Execution",type:"agent",dx:440,dy:160}],wires:[[0,1],[1,2]],edgeConds:["all","affirm"]},{id:"swarm_consensus",name:"Swarm Consensus (Albert)",desc:"A decentralized network. It forces multiple independent computers to agree before taking action.",icon:"network",color:"var(--green)",nodes:[{name:"Node_A",type:"agent",dx:40,dy:60},{name:"Node_B",type:"agent",dx:40,dy:260},{name:"Fleet_Sync",type:"agent",dx:240,dy:160},{name:"Consensus_Gate",type:"gate",dx:440,dy:160},{name:"Unified_State",type:"agent",dx:640,dy:160}],wires:[[0,2],[1,2],[2,3],[3,4]],edgeConds:["all","all","all","affirm"]}];function Mt(e=""){const t=document.getElementById("arch-lib-items");if(!t)return;t.innerHTML='<div style="font-size:10px;color:var(--muted2);margin-bottom:10px;line-height:1.6;padding:8px 0;">Click to spawn a wired agent architecture on the canvas.</div>';const n=document.getElementById("archLibSearch");n&&e&&n.value!==e&&(n.value=e);const o={};Object.keys(Zt).forEach(r=>o[r]=[]);const i=r=>{for(const[a,s]of Object.entries(Zt))if(s.includes(r))return a;return"Orchestration & Consensus"};if($n.forEach(r=>{if(e&&!r.name.toLowerCase().includes(e.toLowerCase())&&!r.desc.toLowerCase().includes(e.toLowerCase()))return;const a=i(r.id);o[a].push(r)}),Object.entries(o).forEach(([r,a])=>{if(a.length===0)return;const s=e?!0:ut[r],l=document.createElement("div");l.className="lib-category"+(s?"":" collapsed");const d=document.createElement("div");if(d.className="lib-category-header",d.style.display="flex",d.style.justifyContent="space-between",d.style.alignItems="center",d.innerHTML=`<span>${r}</span><i data-lucide="chevron-down"></i>`,d.onclick=()=>{ut[r]=!ut[r],Mt(e)},l.appendChild(d),s){const c=document.createElement("div");c.className="lib-category-items",c.style.padding="8px 0",a.forEach(p=>{const g=document.createElement("div");g.className="archetype-card",g.onmouseenter=f=>{Ie.startDelay(p.desc,f.clientX,f.clientY)},g.onmouseleave=()=>Ie.hide(),g.draggable=!0,g.ondragstart=f=>{f.dataTransfer.setData("tern-node-type","archetype"),f.dataTransfer.setData("tern-arch-id",p.id)},g.innerHTML=`
          <div class="archetype-card-title" style="display:flex;align-items:center;gap:6px;">
            <i data-lucide="${p.icon}" style="width:12px;height:12px;color:${p.color}"></i>
            ${p.name}
          </div>
          <div class="archetype-card-desc">${p.desc}</div>
        `,g.onclick=()=>Rt(p),c.appendChild(g)}),l.appendChild(c)}t.appendChild(l)}),t.querySelectorAll(".archetype-card").length===0){const r=document.createElement("div");r.id="no-arch-matches",r.style.padding="20px",r.style.textAlign="center",r.style.color="var(--muted2)",r.style.fontSize="11px",r.textContent="No archetypes match your search.",t.appendChild(r)}lucide.createIcons()}window.renderArchetypes=Mt;function Rt(e,t,n){document.getElementById("flow-canvas");const o=document.getElementById("canvas-hint");o&&(o.style.display="none");const{x:i,y:r}=ae(-300,-200),a=t!==void 0?t:i,s=n!==void 0?n:r,l=[];e.nodes.forEach((d,c)=>{const p="node_"+Date.now()+"_"+c;l.push(p),K(d.name,"__arch__",a+d.dx,s+d.dy,d.type,p);const g=x.find(f=>f.id===p);g&&(g.props.code=An(e.id,d.name,d.type),d.props&&(g.props={...g.props,...d.props}),Ze(p))}),e.wires.forEach(([d,c],p)=>{const g="wire_"+Date.now()+"_"+p,f=(e.edgeConds||[])[p]||"all",u=(e.feedbackWires||[]).includes(p);E.push({id:g,fromId:l[d],toId:l[c],signal:0,condition:f,transform:"none",label:f!=="all"?f:"",priority:5,isFeedback:u})}),N(),R(),I(`Architecture "${e.name}" spawned`,"ok")}window.spawnArchetype=Rt;function An(e,t,n){const o=t.toLowerCase();if(n==="gate")return o.includes("consensus")||o.includes("majority")||o.includes("aggregation")?`fn main() -> trit {
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
}`}window.getArchetypeCode=An;let U=null;function xt(e){U=e,z=null;const t=document.getElementById("prop-header-label"),n=document.getElementById("prop-help-icon");t&&(t.textContent="EDGE PROPERTIES"),n&&(n.style.display="flex"),document.querySelectorAll(".flow-node").forEach(o=>o.classList.remove("selected")),et(),Nt()}window.selectWire=xt;function et(){document.querySelectorAll(".flow-wire").forEach(e=>{e.classList.remove("selected-wire");const t=e.getAttribute("data-wire-id"),n=E.find(i=>i.id===t);if(!n)return;e.classList.remove("cond-affirm","cond-tend","cond-reject","cond-all");const o=String(n.condition)==="1"?"affirm":String(n.condition)==="0"?"tend":String(n.condition)==="-1"?"reject":String(n.condition).replace("!","");n.condition&&n.condition!=="all"&&e.classList.add("cond-"+o),t===U&&e.classList.add("selected-wire")})}window.updateWireStyles=et;function Nt(){const e=document.getElementById("prop-header-label"),t=document.getElementById("prop-body"),n=document.getElementById("prop-help-icon"),o=E.find(a=>a.id===U);if(!o){j();return}e&&(e.textContent="Edge Properties"),n&&(n.style.display="flex");const i=x.find(a=>a.id===o.fromId),r=x.find(a=>a.id===o.toId);Zo.render(o,t,i,r)}window.updateEdgePanel=Nt;function mi(e,t){const n=E.find(o=>o.id===U);n&&(n[e]=t,e==="condition"&&(n.label=t!=="all"?t:"",Nt()),et(),N())}window.updateWireProp=mi;function Mn(e){me(),E=E.filter(t=>t.id!==e),U=null,document.getElementById("wire-handle").classList.remove("active"),N(),j(),document.getElementById("prop-header-label")&&(document.getElementById("prop-header-label").textContent="Node Properties")}window.deleteWire=Mn;function fi(){const e=document.getElementById("flow-inspector"),t=document.getElementById("ins-toggle-icon");e&&(e.classList.contains("inspector-minimized")?(e.classList.replace("inspector-minimized","inspector-expanded"),t&&t.setAttribute("data-lucide","chevron-down"),localStorage.setItem("ternflow-inspector-minimized","false")):(e.classList.replace("inspector-expanded","inspector-minimized"),t&&t.setAttribute("data-lucide","chevron-up"),e.style.height="",localStorage.setItem("ternflow-inspector-minimized","true")),window.lucide&&lucide.createIcons())}window.toggleInspector=fi;function A(e,t){const n=document.getElementById("ins-body");if(!n)return;const o=new Date().toLocaleTimeString([],{hour12:!1,hour:"2-digit",minute:"2-digit",second:"2-digit"}),i=document.createElement("div");i.className="ins-row",i.innerHTML=`
    <span class="ins-time">[${o}]</span>
    <span class="ins-node">${e}</span>
    <span class="ins-msg">${t}</span>
  `,n.appendChild(i),n.scrollTop=n.scrollHeight}window.logInspector=A;function Rn(){const e={},t={};x.forEach(a=>{e[a.id]=0,t[a.id]=[]}),E.forEach(a=>{a.isFeedback||t[a.fromId]&&(t[a.fromId].push(a.toId),e[a.toId]=(e[a.toId]||0)+1)});const n=x.filter(a=>e[a.id]===0).map(a=>a.id),o=[];for(;n.length;){const a=n.shift();o.push(a),(t[a]||[]).forEach(s=>{--e[s]===0&&n.push(s)})}const i=o.length<x.length,r=i?x.filter(a=>!o.includes(a.id)):[];return{order:o,hasCycle:i,cycleNodes:r}}window.topoSort=Rn;function Nn(){const e=[],t=[];if(document.querySelectorAll(".flow-node").forEach(i=>i.classList.remove("node-error","node-warn")),document.querySelectorAll(".node-badge").forEach(i=>i.remove()),x.length===0)return e.push({msg:"Canvas is empty",nodeId:null}),{errors:e,warnings:t};const{hasCycle:n,cycleNodes:o}=Rn();return n&&o.forEach(i=>{e.push({msg:`Cycle detected at "${i.name}"`,nodeId:i.id}),he(i.id,"error","↺")}),x.forEach(i=>{var d;const r=E.filter(c=>c.toId===i.id),a=E.filter(c=>c.fromId===i.id),s=r.length===0,l=a.length===0;s&&l&&x.length>1&&(t.push({msg:`"${i.name}" is isolated — not connected`,nodeId:i.id}),he(i.id,"warn","⚠")),i.type==="agent"&&!((d=i.props.code)!=null&&d.trim())&&(t.push({msg:`"${i.name}" has no .tern code`,nodeId:i.id}),a.length||he(i.id,"warn","?")),r.forEach(c=>{const p=x.find(g=>g.id===c.fromId);p&&p.props.output_schema&&i.props.input_schema&&p.props.output_schema.trim()!==i.props.input_schema.trim()&&(t.push({msg:`Schema mismatch: "${p.name}" → "${i.name}"`,nodeId:i.id}),he(i.id,"warn","≠"))}),r.length>1&&i.type==="agent"&&t.push({msg:`"${i.name}" has ${r.length} inputs — consider a Gate node for consensus`,nodeId:i.id})}),{errors:e,warnings:t}}window.validateGraph=Nn;function he(e,t,n){const o=document.getElementById(e);if(!o)return;o.classList.add(t==="error"?"node-error":"node-warn");const i=document.createElement("div");i.className="node-badge node-badge-"+t,i.textContent=n,o.appendChild(i)}window.markNode=he;function Pn(e,t){const n=document.getElementById("prop-header-label"),o=document.getElementById("prop-body"),i=document.getElementById("prop-help-icon");if(n&&(n.textContent="Graph Validation"),i&&(i.style.display="none"),e.length+t.length===0){o.innerHTML='<div style="color:var(--green);font-size:13px;text-align:center;margin-top:40px;">✓ Graph is valid</div><div style="color:var(--muted);font-size:11px;text-align:center;margin-top:8px;">No errors or warnings found.</div>';return}const a=[...e.map(s=>({...s,level:"error"})),...t.map(s=>({...s,level:"warn"}))];o.innerHTML=`
    <div style="font-size:11px;color:var(--muted2);margin-bottom:12px;">${e.length} error${e.length!==1?"s":""}, ${t.length} warning${t.length!==1?"s":""}</div>
    ${a.map(s=>`
      <div style="display:flex;gap:8px;padding:8px;margin-bottom:4px;border-radius:6px;background:${s.level==="error"?"rgba(239,68,68,0.08)":"rgba(245,158,11,0.08)"};border:1px solid ${s.level==="error"?"var(--red)":"var(--amber)"};cursor:${s.nodeId?"pointer":"default"};"
           ${s.nodeId?`onclick="selectNode('${s.nodeId}');updatePropertyPanel()"`:""}>
        <span style="color:${s.level==="error"?"var(--red)":"var(--amber)"};font-size:14px;flex-shrink:0;">${s.level==="error"?"✗":"⚠"}</span>
        <span style="font-size:11px;color:var(--text);line-height:1.5;">${s.msg}</span>
      </div>`).join("")}
    <button class="btn btn-ghost" style="width:100%;margin-top:12px;font-size:11px;" onclick="document.querySelectorAll('.flow-node').forEach(el=>el.classList.remove('node-error','node-warn'));document.querySelectorAll('.node-badge').forEach(b=>b.remove());updatePropertyPanel();">Clear markers</button>
  `}window.showValidationPanel=Pn;const gi={merge(e){if(!e.length)return{val:0,conf:0};let t=0,n=0;if(e.forEach(a=>{n+=a.val*a.conf,t+=a.conf}),t===0)return{val:0,conf:0};const o=n/t,i=o>.33?1:o<-.33?-1:0,r=t/e.length;return{val:i,conf:Math.min(1,r)}},transform(e,t){let n={...e};const o=t.priority?(11-t.priority)*.02:.05;if(n.conf=Math.max(0,n.conf-o),t.condition&&t.condition!=="all"){const i=String(t.condition);if(!((i==="1"||i==="affirm")&&n.val===1||(i==="0"||i==="tend")&&n.val===0||(i==="-1"||i==="reject")&&n.val===-1||i==="!reject"&&n.val!==-1||i==="!tend"&&n.val!==0)){if(t.transform==="block")return null;t.transform==="flip"&&(n.val=-n.val,n.conf*=.8),t.transform==="hold"&&(n.val=0,n.conf*=.5)}}return n}},we=[],zn=2e3;function fe(){const e=document.getElementById("flow-fog-canvas"),t=document.getElementById("flow-canvas-wrap");if(!e||!t)return;const n=e.getContext("2d");(e.width!==t.clientWidth||e.height!==t.clientHeight)&&(e.width=t.clientWidth,e.height=t.clientHeight),n.clearRect(0,0,e.width,e.height),n.fillStyle="rgba(10, 15, 25, 0.35)",n.fillRect(0,0,e.width,e.height),x.forEach(o=>{const i=o.x*b.scale+b.x+90*b.scale,r=o.y*b.scale+b.y+40*b.scale;if(i<-500||i>e.width+500||r<-500||r>e.height+500)return;const a=document.getElementById("status-"+o.id);let s=0;a&&(a.classList.contains("ok")?s=1:a.classList.contains("err")?s=-1:a.classList.contains("run")&&(s=0));const l=350*b.scale,d=n.createRadialGradient(i,r,40*b.scale,i,r,l);s===1?(d.addColorStop(0,"rgba(34, 197, 94, 0.3)"),d.addColorStop(1,"rgba(10, 12, 16, 0)")):s===-1?(d.addColorStop(0,"rgba(239, 68, 68, 0.3)"),d.addColorStop(1,"rgba(10, 12, 16, 0)")):(d.addColorStop(0,"rgba(168, 85, 247, 0.2)"),d.addColorStop(1,"rgba(10, 12, 16, 0)")),n.globalCompositeOperation="screen",n.fillStyle=d,n.beginPath(),n.arc(i,r,l,0,Math.PI*2),n.fill()})}window.updateFogHeatmap=fe;async function Dn(){if(D!=="running"){ie("running");try{const{errors:e,warnings:t}=Nn();if(e.length>0){Pn(e,t),I(`${e.length} error${e.length>1?"s":""} — fix before simulating`,"error");return}x.forEach(p=>{p.visited=!1,p.executed=!1,p.props&&(p.props.status="")}),E.forEach(p=>{p.active=!1,p.signal=0});const n=document.getElementById("scrub-layer");n&&n.getContext("2d").clearRect(0,0,n.width,n.height);const o=document.getElementById("global-timeline");o&&(o.value=0),On();const i=document.getElementById("simStopBtn");i&&(i.style.display="inline-flex");const r=document.getElementById("flow-inspector");if(r){r.classList.add("active");const p=localStorage.getItem("ternflow-inspector-minimized")==="true";if(!p&&r.classList.contains("inspector-minimized")){r.classList.replace("inspector-minimized","inspector-expanded");const g=document.getElementById("ins-toggle-icon");g&&g.setAttribute("data-lucide","chevron-down"),window.lucide&&lucide.createIcons()}else if(p&&r.classList.contains("inspector-expanded")){r.classList.replace("inspector-expanded","inspector-minimized");const g=document.getElementById("ins-toggle-icon");g&&g.setAttribute("data-lucide","chevron-up"),window.lucide&&lucide.createIcons()}}const a=document.getElementById("ins-body");a&&(a.innerHTML=""),document.querySelectorAll(".trit-particle-ghost").forEach(p=>p.remove()),document.querySelectorAll(".flow-node").forEach(p=>p.classList.remove("pulse-affirm","pulse-reject","pulse-hold","node-error","node-warn")),x.forEach(p=>pe(p.id,"")),we.length=0,A("SYSTEM","🚀 TernFlow Engine v2 initialized"),fe(),jn(0),x.filter(p=>!E.some(g=>g.toId===p.id)).forEach(p=>{we.push({toId:p.id,val:1,conf:1,origin:"ROOT"})});const l=[...we],{scheduledEvents:d,maxSimDuration:c}=await yi(l);window.globalScheduledEvents=d,console.log("DEBUG -> Events generated:",d.length,"| Total Duration:",c),H=!1,(d.length===0||c===0)&&console.warn("[DIAGNOSTIC] Simulation data empty. Check roots and latencies."),await Pt(d,c)}catch(e){console.error("Simulation Start Failure:",e),I("Simulation failed to initialize","err"),window.TERNLANG_CRITICAL_DEBUG&&window.TERNLANG_CRITICAL_DEBUG.push({ts:Date.now(),msg:"runSimulation Crash",error:e.message})}finally{Oe&&!H||(Oe=!1,Xe())}}}window.runSimulation=Dn;async function yi(e){const t=[];let n=0;const o=e,i={};for(;o.length>0&&t.length<zn;){if(D==="idle")return{scheduledEvents:[],maxSimDuration:0};if(D==="paused"&&await Tt(),D==="idle")return{scheduledEvents:[],maxSimDuration:0};if(H)break;const r=o.shift(),a=x.find(f=>f.id===r.toId);if(!a)continue;const d=(r.absEndTime||0)+150;i[a.id]=Math.max(i[a.id]||0,d),n=Math.max(n,d);const c=await Un(a,r.val,!0);if(H)break;const p={val:c,conf:r.conf,origin:a.id},g=E.filter(f=>f.fromId===a.id);for(const f of g){const u=gi.transform(p,f);if(u){const m=i[a.id],h=parseFloat(f.latency),y=isNaN(h)?ln:h,v=m+y,w={wireId:f.id,val:u.val,conf:u.conf,startTime:m,endTime:v,duration:y,fromId:f.fromId,toId:f.toId};t.push(w),n=Math.max(n,v),o.push({toId:f.toId,...u,absEndTime:v})}}}return we.length=0,{scheduledEvents:t,maxSimDuration:n}}async function Pt(e,t){const n=document.getElementById("global-timeline"),o=document.getElementById("timeline-tick-label"),i=new Set;n&&(n.value=0,n.max=t),le=performance.now(),G=0,ie("running");const r=async()=>{if(D==="idle"){console.log("[TernFlow] Engine Terminated (Stopped).");return}if(D==="paused"){if(console.log("[TernFlow] Engine Yielded (Paused). Awaiting resumption..."),await Tt(),D==="idle")return;le=performance.now()}const a=performance.now(),s=a-le;le=a,G+=s,G>=t&&(G=t,ie("idle"),A("SYSTEM","✓ Pre-flight playback complete")),n&&(n.value=G,o&&(o.textContent=`TIME: ${(G/1e3).toFixed(2)}s`)),Te(G,e,i),D==="running"&&requestAnimationFrame(r)};window.currentDriveTimeline=r,requestAnimationFrame(r)}window.runSimulationCore=Pt;let ee=null,Ne=null;function On(){O=[],ee=null,Ne=null,wt=-1;const e=document.getElementById("scrub-layer");e&&e.getContext("2d").clearRect(0,0,e.width,e.height)}window.resetSimHistory=On;function jn(e,t=[],n=0,o=500){(O.length===0||!ee)&&(ee={nodes:x.map(s=>{const l=document.getElementById(s.id),d=l?l.classList.contains("pulse-affirm")?"affirm":l.classList.contains("pulse-reject")?"reject":l.classList.contains("pulse-hold")?"hold":"":"";return{id:s.id,status:s.props.status||"",pulse:d}}),wires:E.map(s=>({id:s.id,signal:s.signal||0}))},Ne=JSON.parse(JSON.stringify(ee)));const i={tick:e,activeSignals:t,nodeDeltas:[],wireDeltas:[],startTime:n,duration:o};if(x.forEach(s=>{const l=document.getElementById(s.id),d=l?l.classList.contains("pulse-affirm")?"affirm":l.classList.contains("pulse-reject")?"reject":l.classList.contains("pulse-hold")?"hold":"":"",c=s.props.status||"",p=Ne.nodes.find(g=>g.id===s.id);p&&(p.status!==c||p.pulse!==d)&&(i.nodeDeltas.push({id:s.id,status:c,pulse:d}),p.status=c,p.pulse=d)}),E.forEach(s=>{const l=s.signal||0,d=Ne.wires.find(c=>c.id===s.id);d&&d.signal!==l&&(i.wireDeltas.push({id:s.id,signal:l}),d.signal=l)}),O.push(i),O.length>zn){const s=O.shift();s.nodeDeltas.forEach(l=>{const d=ee.nodes.find(c=>c.id===l.id);d&&(d.status=l.status,d.pulse=l.pulse)}),s.wireDeltas.forEach(l=>{const d=ee.wires.find(c=>c.id===l.id);d&&(d.signal=l.signal)})}const r=document.getElementById("global-timeline"),a=n+o;r&&(r.max=a)}window.captureSimSnapshot=jn;function hi(){const e=document.getElementById("global-timeline"),t=document.getElementById("timeline-tick-label");if(O.length===0)return;const n=O[0].startTime,o=O[O.length-1],i=o.startTime+o.duration;e&&(e.min=n,e.max=i,e.value=i),t&&(t.textContent=`TIME: ${(i/1e3).toFixed(2)}s`)}window.showTimeline=hi;function vi(e){const t=parseFloat(e),n=document.getElementById("timeline-tick-label");n&&(n.textContent=`TIME: ${(t/1e3).toFixed(2)}s`),zt(t)}window.scrubToTimeline=vi;let wt=-1,bt=null,Fn=0;function zt(e){Fn=parseFloat(e),bt||(bt=requestAnimationFrame(xi))}window.requestScrub=zt;function xi(){bt=null;const e=Fn;if(O.length===0||!ee)return;let t=O.findIndex(i=>e>=i.startTime&&e<i.startTime+i.duration);t===-1&&(e>=O[O.length-1].startTime?t=O.length-1:t=0);const n=O[t],o=n.tick;if(Math.min(1,Math.max(0,(e-n.startTime)/n.duration)),o!==wt){wt=o;const i=JSON.parse(JSON.stringify(ee));for(let r=0;r<=t&&r<O.length;r++){const a=O[r];a.nodeDeltas.forEach(s=>{const l=i.nodes.find(d=>d.id===s.id);l&&(l.status=s.status,l.pulse=s.pulse)}),a.wireDeltas.forEach(s=>{const l=i.wires.find(d=>d.id===s.id);l&&(l.signal=s.signal)})}i.nodes.forEach(r=>{const a=document.getElementById(r.id);a&&(a.classList.remove("pulse-affirm","pulse-reject","pulse-hold"),r.pulse&&a.classList.add("pulse-"+r.pulse),pe(r.id,r.status));const s=x.find(l=>l.id===r.id);s&&(s.props.status=r.status)}),i.wires.forEach(r=>{const a=E.find(s=>s.id===r.id);a&&(a.signal=r.signal)}),document.querySelectorAll(".trit-particle-ghost").forEach(r=>r.remove()),N(),fe()}Te(e),G=e}function Te(e,t=[],n=null){try{const o=document.getElementById("scrub-layer"),i=document.getElementById("flow-canvas-wrap");if(!o||!i)return;const r=o.getContext("2d");(o.width!==i.clientWidth||o.height!==i.clientHeight)&&(o.width=i.clientWidth,o.height=i.clientHeight),r.clearRect(0,0,o.width,o.height);const a=t.length>0?t:window.globalScheduledEvents||[];t.length>0&&(window.globalScheduledEvents=t);const s=new Set,l={};a.forEach(d=>{if(e>=d.startTime&&e<=d.endTime){const g=(e-d.startTime)/d.duration,f=E.find(u=>u.id===d.wireId);if(f){l[f.id]={signal:d.val,alpha:1};const u=document.getElementById(f.id);if(u)try{const m=u.getTotalLength(),h=u.getPointAtLength(g*m),y=h.x*b.scale+b.x,v=h.y*b.scale+b.y,w=d.val===1?"#22c55e":d.val===-1?"#ef4444":"#f59e0b";r.beginPath(),r.fillStyle=w,r.shadowColor=w,r.shadowBlur=10*b.scale;const k=(6+8*(d.conf||1))*b.scale;r.arc(y,v,k,0,Math.PI*2),r.fill(),r.shadowBlur=0}catch{}}}const p=150;if(e>=d.startTime-p&&e<=d.startTime&&s.add(d.fromId),e>=d.endTime&&e<=d.endTime+p&&s.add(d.toId),n&&e>=d.endTime&&!n.has(d.wireId+"_"+d.endTime)){const g=x.find(u=>u.id===d.toId),f=x.find(u=>u.id===d.fromId);if(g){const u=d.val===1?"+1 (Affirm)":d.val===-1?"-1 (Reject)":"0 (Tend)";A(g.name,`Signal arrival from ${f?f.name:"ROOT"} -> ${u}`),n.add(d.wireId+"_"+d.endTime);const m=E.filter(y=>y.fromId===d.toId),h=x.find(y=>y.id===d.toId);if(m.length===0&&h&&h.type!=="artifact")Vn(h,d.val);else if(h&&h.type==="artifact"){const y=document.getElementById(`art-body-${h.id}`);if(y){const v={source:f?f.name:"Unknown",resolved_signal:d.val,status:"Resolved",context_bridge:h.props.runtime_buffer||{}},w=jsyaml.dump(v),k=y.querySelector("pre");k?k.textContent=w:y.textContent=w,y.style.color=d.val===1?"var(--green)":d.val===-1?"var(--red)":"var(--text)"}}}}}),document.querySelectorAll(".flow-node").forEach(d=>{const c=d.getAttribute("id");d.classList.remove("pulse-affirm","pulse-reject","pulse-hold"),s.has(c)&&d.classList.add("pulse-affirm")}),E.forEach(d=>{const c=l[d.id],p=document.getElementById(d.id);p&&(p.classList.remove("active-1","active-n1","active-0"),c&&p.classList.add(`active-${c.signal===1?"1":c.signal===-1?"n1":"0"}`))})}catch(o){window.TERNLANG_CRITICAL_DEBUG&&window.TERNLANG_CRITICAL_DEBUG.push({ts:performance.now(),msg:"renderScrubLayer Silent Crash Prevented",error:o.message})}}function wi(e){zt(e)}window.scrubSimulation=wi;function Hn(){const e=document.getElementById("flow-inspector"),t=e?e.querySelector(".ins-head"):null;if(!e||!t)return;if(localStorage.getItem("ternflow-inspector-minimized")==="true"){e.classList.replace("inspector-expanded","inspector-minimized");const h=document.getElementById("ins-toggle-icon");h&&h.setAttribute("data-lucide","chevron-up"),window.lucide&&lucide.createIcons()}else{e.classList.replace("inspector-minimized","inspector-expanded");const h=document.getElementById("ins-toggle-icon");h&&h.setAttribute("data-lucide","chevron-down"),window.lucide&&lucide.createIcons()}let o=!1,i=!1,r,a,s=null,l=null,d=null,c=null;function p(){const h=e.getBoundingClientRect();return c=(e.offsetParent||document.documentElement).getBoundingClientRect(),e.style.transition="max-height 0.25s cubic-bezier(0.4, 0, 0.2, 1)",e.style.transform="none",e.style.bottom="auto",e.style.left=h.left-c.left+"px",e.style.top=h.top-c.top+"px",h}t.onmousedown=h=>{if(h.target.closest("button"))return;o=!0;const y=p();r=h.clientX-y.left,a=h.clientY-y.top,document.addEventListener("mousemove",u),document.addEventListener("mouseup",m),h.preventDefault()};const g=[{name:"n",dx:0,dy:-1},{name:"s",dx:0,dy:1},{name:"e",dx:1,dy:0},{name:"w",dx:-1,dy:0},{name:"ne",dx:1,dy:-1},{name:"nw",dx:-1,dy:-1},{name:"se",dx:1,dy:1},{name:"sw",dx:-1,dy:1}],f=document.getElementById("flow-inspector-resizer");f&&f.remove(),g.forEach(({name:h,dx:y,dy:v})=>{const w=document.createElement("div");w.className=`ins-resize-handle ins-r-${h}`,e.appendChild(w),w.addEventListener("mousedown",k=>{k.stopPropagation(),k.preventDefault(),i=!0,s={dx:y,dy:v},l=p(),d={x:k.clientX,y:k.clientY},document.addEventListener("mousemove",u),document.addEventListener("mouseup",m)})});function u(h){if(o)e.style.left=h.clientX-r-c.left+"px",e.style.top=h.clientY-a-c.top+"px";else if(i&&s&&l&&d){const y=h.clientX-d.x,v=h.clientY-d.y,{dx:w,dy:k}=s,B=c;if(w===1)e.style.width=Math.max(250,l.width+y)+"px";else if(w===-1){const S=Math.max(250,l.width-y);e.style.width=S+"px",e.style.left=l.right-S-B.left+"px"}if(k===1)e.style.height=Math.max(80,l.height+v)+"px",e.style.maxHeight="none";else if(k===-1){const S=Math.max(80,l.height-v);e.style.height=S+"px",e.style.maxHeight="none",e.style.top=l.bottom-S-B.top+"px"}}}function m(){o=!1,i=!1,s=null,l=null,d=null,c=null,e.style.transition="",document.removeEventListener("mousemove",u),document.removeEventListener("mouseup",m)}}window.initInspectorDraggable=Hn;function be(){const e=document.getElementById("flow-library"),t=document.getElementById("flow-props"),n=document.querySelector(".timeline-container");if(!e||!t||!n)return;const o=e.offsetWidth,i=t.offsetWidth;n.style.left=o+"px",n.style.width=`calc(100% - ${o+i}px)`}window.updateTimelineBridge=be;function Wn(){const e=document.getElementById("flow-library"),t=document.getElementById("flow-sidebar-resizer");if(!e||!t)return;t.addEventListener("mousedown",i=>{t.classList.add("active");const r=s=>{const l=s.clientX;l>=180&&l<=500&&(e.style.width=l+"px",be())},a=()=>{t.classList.remove("active"),document.removeEventListener("mousemove",r),document.removeEventListener("mouseup",a)};document.addEventListener("mousemove",r),document.addEventListener("mouseup",a)}),qn();const n=new ResizeObserver(()=>be());n.observe(e);const o=document.getElementById("flow-props");o&&n.observe(o),be()}window.initSidebarResizer=Wn;function qn(){const e=document.getElementById("flow-props"),t=document.getElementById("flow-props-resizer");!e||!t||t.addEventListener("mousedown",n=>{t.classList.add("active");const o=e.getBoundingClientRect();n.clientX-o.left;const i=a=>{const l=window.innerWidth-a.clientX;l>=250&&l<=500&&(e.style.width=l+"px",be())},r=()=>{t.classList.remove("active"),document.removeEventListener("mousemove",i),document.removeEventListener("mouseup",r)};document.addEventListener("mousemove",i),document.addEventListener("mouseup",r)})}window.initRightSidebarResizer=qn;function pe(e,t){const n=document.getElementById("status-"+e);n&&(n.className="fn-status"+(t?" "+t:""),n.title=t||"idle")}window.setNodeStatus=pe;async function Kn(e,t){var g;let n=(e.props.template||"{{input}}").replace("{{input}}",t===1?"affirm":t===-1?"reject":"tend");const o=e.props.system_prompt||"You are a ternary logic processor. Output only +1 (affirm), 0 (tend), or -1 (reject).",i=e.props.protocol||"openai",r=e.props.model_id||"";let a=n;e.props.runtime_buffer&&e.props.runtime_buffer.data&&(a=`Here is the ingested data for your analysis:

<context><data_payload>
${e.props.runtime_buffer.data}
</data_payload></context>

User: ${n}`);const s=Math.ceil((o.length+a.length)/4);let l=8192;const d=r.toLowerCase();if(d.includes("gemini-1.5")?l=1048576:d.includes("gemini")?l=32768:d.includes("claude-3")?l=2e5:d.includes("gpt-4")?l=128e3:d.includes("gpt-3.5")||d.includes("gpt-35")?l=16384:d.includes("grok")?l=131072:i==="anthropic"?l=2e5:i==="google"&&(l=1048576),s>l*.8)return A(e.name,`❌ Token Safety Halt: Payload (${s} tk) exceeds 80% of ${l} tk context window.`),-1;A(e.name,`🌐 Calling LLM [${i}:${r||"default"}] (${s} tk)…`);const c=_e(),p=c[i]||c.google||e.props.api_key||"";try{const f=Date.now(),u=await fetch("/api/run",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({source:`// LLM Bridge Proxy
fn main() -> trit { return hold; }`,sql:e.props.sql_query||"",llm_config:{system:o,prompt:a,protocol:i,model_id:r,api_key:p,base_url:e.props.base_url,temperature:e.props.temperature??.1,max_tokens:512}})}),m=Date.now()-f;if(!u.ok)throw new Error(`HTTP ${u.status}`);const h=await u.json(),y=String(h.result||"").toLowerCase();let v=0,w=.5;return y.includes("confidence: 1.0")||y.includes("conf: 1.0")||y.includes("score: 1.0")?w=1:(y.includes("confidence: -1.0")||y.includes("conf: -1.0"))&&(w=-1),y.includes("+1")||y.includes("affirm")||y.includes("yes")?v=1:(y.includes("-1")||y.includes("reject")||y.includes("no"))&&(v=-1),v===-1&&w>.9&&(A(e.name,"🛑 HARD REJECT: LLM returned high-confidence rejection. Aborting simulation."),H=!0),window.dispatchEvent(new CustomEvent("ternlang_local_trace",{detail:{trace_id:"llm-"+Date.now(),timestamp_ms:Date.now(),node_id:e.id,event_type:"LLM_Bridge_Reasoning",signal_in:t,signal_out:v,latency_ms:m,sparse_dropped:!1,reasoning:h.result||"No reasoning path returned."}})),w===1?(A(e.name,`✅ TAP: High Confidence (${v===1?"+1":v===-1?"-1":"0"}) -> Autonomous Execution.`),v):(A(e.name,`🟡 TAP: Ambiguity detected (conf: ${w}) -> Freezing for Operator.`),e.props.pending_actuator={code:h.result,paths:["Affirm (+1)","Reject (-1)"]},0)}catch(f){return A(e.name,`❌ LLM Error: ${f.message}. Falling back to deterministic mapper.`),(g=e.props.system_prompt)!=null&&g.toLowerCase().includes("safety")?1:t}}window.executeLLMNode=Kn;async function Un(e,t,n=!1){if(D==="idle"||(D==="paused"&&await Tt(),D==="idle")||H)return 0;const o=document.getElementById(e.id);if(!o&&!n)return t;n||(pe(e.id,"run"),o&&o.classList.remove("pulse-affirm","pulse-reject","pulse-hold"),window.dispatchEvent(new CustomEvent("ternlang_local_trace",{detail:{trace_id:"local-"+Date.now()+"-"+Math.random().toString(36).substr(2,5),timestamp_ms:Date.now(),node_id:e.id,event_type:e.type==="external"?"LLM_Bridge":"Logic_Eval",signal_in:t,signal_out:0,latency_ms:e.type==="external"?0:50}})));let i=t;if(e.type==="external")i=await Kn(e,t);else if(e.type==="moe13")i=await Gn(e,t);else if(e.type==="datasource"){const s=e.props.payload||"";let l=e.props.data_type||"text",d=s;if(l==="yaml"||l==="yml"||l==="json")try{l==="json"?d=JSON.parse(s):d=jsyaml.load(s),l="object"}catch(p){console.warn(`[DataSource] Parse failed for type ${l}:`,p)}n||A(e.name,`📡 Injecting Payload: [${l}] ${typeof d=="object"?"Structured Object":s.substring(0,20)}...`),E.filter(p=>p.fromId===e.id).forEach(p=>{const g=x.find(f=>f.id===p.toId);g&&(g.props.runtime_buffer={type:l,data:d},n||A("SYSTEM",`💾 Buffered ${typeof d=="object"?"object":s.length+" bytes"} to ${g.name}`))}),i=1}else{const s=e.props.code||"";if(s.trim()){n||A(e.name,"⚡ Executing logic…");const l=rt(s);if(l.ok){i=l.trit===1?1:l.trit===-1?-1:0;const d=i===1?"+1 AFFIRM":i===-1?"-1 REJECT":"0 TEND";n||A(e.name,`→ ${d}${l.output&&l.output.length?" · "+l.output.join(", "):""}`)}else return n||(A(e.name,`✗ ${l.error||"error"}`),pe(e.id,"err"),o&&o.classList.add("pulse-reject")),n||await new Promise(d=>setTimeout(d,600)),-1}else{const l=t===1?"+1 AFFIRM":t===-1?"-1 REJECT":"0 TEND";n||A(e.name,`→ ${l} (passthrough)`)}}if(H)return i;if(!n){const s=i===1?"pulse-affirm":i===-1?"pulse-reject":"pulse-hold";o&&o.classList.add(s),pe(e.id,i===1?"ok":i===-1?"err":"run")}if(e.props.pending_actuator)return n||(A("SYSTEM",`🟡 TAP: State 0 Suspension at "${e.name}". Awaiting Operator…`),H=!0),0;const r=E.filter(s=>s.fromId===e.id),a=x.filter(s=>s.type==="artifact"&&s.parentId===e.id);return r.length===0&&e.type!=="artifact"&&(n||(A("SYSTEM","🛑 Terminal Payload Detected — Hard Halt engaged."),H=!0)),a.forEach(s=>{const l=document.getElementById(`art-body-${s.id}`);if(l){const d={source:e.name,resolved_signal:i,status:"Halt Emitted",context_bridge:e.props.runtime_buffer||{}},c=jsyaml.dump(d);l.innerHTML=`<pre style="margin:0; font-family:'JetBrains Mono',monospace; font-size:11px; white-space:pre-wrap;">${c}</pre>`,l.style.color=i===1?"var(--green)":i===-1?"var(--red)":"var(--text)"}}),n||await new Promise(s=>setTimeout(s,500)),i}window.simulateNode=Un;async function Gn(e,t){A(e.name,"🧠 MOE-13: Initiating Live Deliberation Cycle...");const n=e.id,o=document.getElementById(`moe-veto-alert-${n}`),i=document.getElementById(`moe-verdict-${n}`);o&&(o.style.display="none"),i&&(i.textContent="QUERYING BACKEND...",i.style.color="var(--magenta)");try{const r=await fetch("/api/moe/orchestrate",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({query:e.props.system_prompt||"Evaluate the current triadic signal state.",evidence:[t===1?1:t===-1?-1:0,.5,.5,.5,.5,.5]})});if(!r.ok)throw new Error(`HTTP ${r.status}`);const a=await r.json();return a.verdicts&&a.verdicts.forEach(s=>{const l=document.getElementById(`moe-vote-${n}-${s.expert_id}`),d=document.getElementById(`moe-conf-${n}-${s.expert_id}`);l&&(l.textContent=s.trit===1?"+1":s.trit===-1?"-1":"0",l.style.color=s.trit===1?"var(--green)":s.trit===-1?"var(--red)":"var(--muted2)"),d&&(d.textContent=Math.round(s.confidence*100)+"%")}),a.safety_vetoed&&(o&&(o.style.display="block"),A(e.name,"🛑 CRITICAL VETO: Safety experts triggered Hard Halt."),H=!0),a.trit===-1&&a.confidence>.9&&(A(e.name,`🛑 HARD REJECT: High-confidence rejection (${(a.confidence*100).toFixed(1)}%) triggered Hard Halt.`),H=!0),i&&(i.textContent=a.label.toUpperCase(),i.style.color=a.trit===1?"var(--green)":a.trit===-1?"var(--red)":"var(--amber)"),A(e.name,`✓ Deliberation Complete. Verdict: ${a.trit} (${a.label}) | Conf: ${(a.confidence*100).toFixed(1)}%`),window.dispatchEvent(new CustomEvent("ternlang_local_trace",{detail:{trace_id:"moe-"+Date.now(),timestamp_ms:Date.now(),node_id:e.id,event_type:"MoE-13_Orchestration",signal_in:t,signal_out:a.trit,latency_ms:150,reasoning:a.verdicts?a.verdicts.map(s=>`${s.expert_name}: ${s.reasoning}`).join(" | "):""}})),a.trit}catch(r){return A(e.name,`❌ MoE-13 Error: ${r.message}. Falling back to Tend.`),i&&(i.textContent="OFFLINE",i.style.color="var(--muted2)"),0}}window.executeMOE13=Gn;function Vn(e,t){const n=!!e.props.pending_actuator;let o=n?"Suspended":"Resolved",i=e.props.runtime_buffer||{};if(n){const y=e.props.pending_actuator;y.error?(o="WASM Runtime Error",i={error:y.error,failed_code:y.last_failed_code}):(o="TAP Suspension",i={proposed_logic:y.code,divergent_paths:y.paths})}const r={source:e.name,resolved_signal:t,status:o,context_bridge:i},a=jsyaml.dump(r),s=x.find(y=>y.type==="artifact"&&y.parentId===e.id);if(s){s.props.payload=a;const y=document.getElementById(`art-body-${s.id}`);y&&(y.innerHTML=`<pre style="margin:0; font-family:'JetBrains Mono',monospace; font-size:11px; white-space:pre-wrap;">${a}</pre>`,y.style.color=n?"var(--amber)":t===1?"var(--green)":t===-1?"var(--red)":"var(--text)",n&&tn(s.id,e.id)),z===s.id&&j();return}const l=document.getElementById(e.id);if(!l)return;const d=parseFloat(l.style.left),c=parseFloat(l.style.top),p="art_"+Date.now(),g=n?"TAP: "+e.name:"Result: "+e.name,f=In(d+350,c,300,200);K(g,"__artifact__",f.x,f.y,"artifact",p),$t(f.x,f.y);const u=x.find(y=>y.id===p);u&&(u.parentId=e.id,u.props.state="lock",u.props.payload=a);const m=document.getElementById(`art-body-${p}`);m&&(m.innerHTML=`<pre style="margin:0; font-family:'JetBrains Mono',monospace; font-size:11px; white-space:pre-wrap;">${a}</pre>`,m.style.color=n?"var(--amber)":t===1?"var(--green)":t===-1?"var(--red)":"var(--text)",n&&tn(p,e.id));const h="wire_art_"+Date.now();E.push({id:h,fromId:e.id,toId:p,condition:"all",transform:"none",label:n?"TAP PENDING":"RESULT",priority:10}),N(),R(),lucide.createIcons()}function tn(e,t){const n=document.getElementById(`art-body-${e}`);if(!n)return;const o=document.createElement("div");o.style.marginTop="12px",o.style.display="flex",o.style.gap="8px",o.innerHTML=`
    <button class="btn-pill" style="background:var(--green); color:white; border:none; padding:4px 12px; font-size:10px; cursor:pointer;" onclick="resolveTAP('${e}', '${t}', 1)">Approve (+1)</button>
    <button class="btn-pill" style="background:var(--red); color:white; border:none; padding:4px 12px; font-size:10px; cursor:pointer;" onclick="resolveTAP('${e}', '${t}', -1)">Reject (-1)</button>
  `,n.appendChild(o)}async function bi(e,t,n){const o=x.find(r=>r.id===t);if(!o)return;const i=o.props.pending_actuator;if(n===1&&i&&i.code){A(o.name,"⚙️  TAP Execution Loop: Initiating WASM Sandbox...");const r=await sn(i.code);r.ok?(A(o.name,`✅ WASM Success: Output captured (${r.output.length} bytes).`),o.props.runtime_buffer={type:"text",data:r.output},delete o.props.pending_actuator,Ke(e),yt(t,1)):(A(o.name,"❌ WASM Runtime Error: Reverting to State 0..."),o.props.pending_actuator.error=r.traceback||r.error,o.props.pending_actuator.last_failed_code=i.code,Vn(o,0),j())}else delete o.props.pending_actuator,A("SYSTEM",`✅ TAP RESOLVED: Operator injected ${n===1?"+1":"-1"} to "${o.name}".`),Ke(e),yt(t,n)}window.resolveTAP=bi;function Yn(e,t){const n=x.find(o=>o.id===e);if(n&&(n.props.payload=t,n.parentId)){const o=x.find(i=>i.id===n.parentId);o&&o.props.pending_actuator&&(o.props.pending_actuator.code=t)}}window.updateArtifactPayload=Yn;function Jn(e,t){const n=x.find(d=>d.id===e);if(!n)return;n.props.state=t;const o=document.getElementById(e),i=document.getElementById(`art-body-${e}`),r=document.getElementById(`art-edit-${e}`),a=document.getElementById(`art-socket-label-${e}`);if(o.classList.remove("state-lock","state-transmute","state-extend"),o.classList.add(`state-${t}`),t==="lock"){if(i.style.display="block",r.style.display="none",a.style.display="none",r.value){const d=i.querySelector("pre");d?d.textContent=r.value:i.textContent=r.value,Yn(e,r.value)}}else if(t==="transmute"){i.style.display="none",r.style.display="block",a.style.display="none";const d=i.querySelector("pre");r.value=d?d.textContent:i.textContent}else t==="extend"&&(i.style.display="block",r.style.display="none",a.style.display="flex");o.querySelectorAll(".art-btn").forEach(d=>d.classList.remove("active"));const s=t==="lock"?0:t==="transmute"?1:2,l=o.querySelectorAll(".art-btn");l[s]&&l[s].classList.add("active"),N(),R()}window.setArtifactState=Jn;function Xn(){x.filter(t=>t.type==="artifact").forEach(t=>{E=E.filter(o=>o.fromId!==t.id&&o.toId!==t.id);const n=document.getElementById(t.id);n&&n.remove()}),x=x.filter(t=>t.type!=="artifact"),N()}window.clearResultArtifacts=Xn;function N(){fe();let e=document.getElementById("flow-svg-layer");if(!e){const t=document.getElementById("flow-canvas");if(!t)return;e=document.createElementNS("http://www.w3.org/2000/svg","svg"),e.id="flow-svg-layer",t.prepend(e)}if(e.innerHTML="",document.querySelectorAll(".edge-badge").forEach(t=>t.remove()),x.forEach(t=>{const n=document.getElementById(t.id);n&&(E.some(i=>i.fromId===t.id)?n.classList.add("has-output"):n.classList.remove("has-output"))}),E.forEach(t=>{const n=document.getElementById(t.fromId),o=document.getElementById(t.toId);if(!n||!o)return;let i=".flow-port-out";String(t.condition)==="1"?i=".port-affirm":String(t.condition)==="0"?i=".port-neutral":String(t.condition)==="-1"?i=".port-reject":String(t.condition)==="!reject"&&(i=".port-affirm");let r=n.querySelector(i);r||(r=n.querySelector(".flow-port-out"));const a=o.querySelector(".flow-port-in");if(!r||!a)return;const s=te(r),l=te(a);Ue(s,l,t.id,t.signal,t,t.confidence)}),$){const t=$.start,n=$.end;Ue(t,n,"active-wire",0,null);const o=x.find(r=>r.id===$.fromId);let i=document.getElementById("evolution-ghost");if(o&&o.type==="artifact"&&o.props.state==="extend"&&$.fromIsOutput){i||(i=document.createElement("div"),i.id="evolution-ghost",i.className="flow-node agent ghost-node",i.innerHTML=`
            <div class="fn-head" style="opacity:0.6; pointer-events:none;">
              <div style="display:flex; align-items:center; gap:6px; font-size:10px; font-weight:700; color:var(--cyan)">
                <i data-lucide="bot" style="width:12px"></i>TRANSMUTED
              </div>
            </div>
            <div class="fn-body" style="padding:10px; font-size:9px; color:var(--muted); text-align:center; pointer-events:none;">RELEASE TO EVOLVE</div>
          `,document.getElementById("flow-canvas").appendChild(i),lucide.createIcons()),i.style.left=n.x-90+"px",i.style.top=n.y-40+"px",i.style.display="block";const r=document.getElementById("active-wire");r&&(r.style.stroke="var(--cyan)",r.style.strokeWidth="3",r.style.strokeDasharray="5 3",r.style.opacity="0.8")}else i&&(i.style.display="none")}else{const t=document.getElementById("evolution-ghost");t&&(t.style.display="none")}}window.updateWires=N;function Qn(e,t,n){const o=t.x-e.x,i=t.y-e.y;if(n&&n.cp)return`M ${e.x} ${e.y} Q ${n.cp.x} ${n.cp.y} ${t.x} ${t.y}`;let r=null;const a=n?n.fromId:null,s=n?n.toId:null;for(const d of x){if(d.id===a||d.id===s)continue;const c=d.type==="artifact"?300:d.type==="moe13"?320:180,p=d.type==="artifact"?200:d.type==="moe13"?360:80,g=20,f=d.x-c/2-g,u=d.x+c/2+g,m=d.y-p/2-g,h=d.y+p/2+g,y=e.x+o/2,v=e.y+i/2;if(y>f&&y<u&&v>m&&v<h){r=d;break}}if(r){r.type==="artifact"||r.type;const d=r.type==="artifact"?200:r.type==="moe13"?360:80,c=e.y<r.y?r.y-d/2-40:r.y+d/2+40;return`M ${e.x} ${e.y} Q ${r.x} ${c} ${t.x} ${t.y}`}if(o<60){const d=Math.max(120,Math.abs(i)*.4);return`M ${e.x} ${e.y} C ${e.x+d} ${e.y}, ${t.x-d} ${t.y}, ${t.x} ${t.y}`}const l=o*.5;return`M ${e.x} ${e.y} C ${e.x+l} ${e.y}, ${t.x-l} ${t.y}, ${t.x} ${t.y}`}window.computeWirePath=Qn;async function Ei(e,t,n=1){const o=document.getElementById(e.fromId),i=document.getElementById(e.toId);if(!o||!i)return;const r=o.querySelector(".flow-port-out"),a=i.querySelector(".flow-port-in"),s=te(r),l=te(a);Ue(s,l,e.id,t,e,n)}window.animateSignal=Ei;function te(e){const t=e.getBoundingClientRect(),n=document.getElementById("flow-canvas-wrap").getBoundingClientRect();return{x:(t.left-n.left-b.x+t.width/2)/b.scale,y:(t.top-n.top-b.y+t.height/2)/b.scale}}window.getPortPos=te;function Ue(e,t,n,o,i,r=1){const a=document.getElementById("flow-svg-layer"),s=Qn(e,t,i);let l=a.querySelector(`path[id="${n}"]`),d=!1;l||(l=document.createElementNS("http://www.w3.org/2000/svg","path"),l.setAttribute("id",n),d=!0),l.setAttribute("d",s),l.setAttribute("data-wire-id",n);let c="flow-wire";if(o!==void 0&&(c+=` active-${o===1?"1":o===-1?"n1":"0"}`),i&&i.condition&&i.condition!=="all"){const f=String(i.condition)==="1"?"affirm":String(i.condition)==="0"?"tend":String(i.condition)==="-1"?"reject":String(i.condition).replace("!","");c+=" cond-"+f}if(n===U&&(c+=" selected-wire"),l.setAttribute("class",c),n==="active-wire"?(l.style.pointerEvents="none",l.style.stroke="var(--cyan)",l.style.strokeWidth="3",l.style.strokeDasharray="5 3",l.style.opacity="0.8"):i&&i.customColor?l.style.stroke=i.customColor:l.style.stroke="",l.style.opacity=n==="active-wire"?.8:.2+.8*r,l.style.strokeWidth=n==="active-wire"?3:1.5+2.5*r,l.style.transition="opacity 0.3s, stroke-width 0.3s, stroke 0.3s",d&&n!=="active-wire"&&i){l.style.pointerEvents="stroke",l.addEventListener("mousedown",u=>{u.preventDefault(),xt(n),Et(n)});const f=document.createElementNS("http://www.w3.org/2000/svg","path");f.setAttribute("d",s),f.setAttribute("fill","none"),f.setAttribute("stroke","transparent"),f.setAttribute("stroke-width","18"),f.setAttribute("class","wire-hit"),f.id="hit-"+n,f.style.pointerEvents="stroke",f.addEventListener("mousedown",u=>{u.preventDefault(),xt(n),Et(n)}),a.appendChild(f)}else if(!d&&i){const f=document.getElementById("hit-"+n);f&&f.setAttribute("d",s)}d&&a.appendChild(l);let p,g;if(i&&i.cp?(p=.25*e.x+.5*i.cp.x+.25*t.x,g=.25*e.y+.5*i.cp.y+.25*t.y):(p=(e.x+t.x)/2,g=(e.y+t.y)/2,t.x-e.x<60&&(p+=60)),i&&i.label){let f=document.getElementById("badge-"+n);if(!f){f=document.createElement("div"),f.id="badge-"+n;const m=document.getElementById("flow-canvas");m&&m.appendChild(f)}const u=i.condition&&i.condition!=="all"?" cond-"+(String(i.condition)==="1"?"affirm":String(i.condition)==="0"?"tend":String(i.condition)==="-1"?"reject":String(i.condition).replace("!","")):"";f.className="edge-badge"+u,f.style.left=p+"px",f.style.top=g+"px",f.textContent=i.label,f.style.opacity=.4+.6*r}if(n===U){const f=document.getElementById("wire-handle");f.style.left=p+"px",f.style.top=g+"px"}}window.drawWire=Ue;function Et(e){const t=E.find(d=>d.id===e);if(!t)return;const n=document.getElementById(t.fromId),o=document.getElementById(t.toId);if(!n||!o)return;const i=te(n.querySelector(".flow-port-out")),r=te(o.querySelector(".flow-port-in"));t.cp||(t.cp={x:(i.x+r.x)/2,y:(i.y+r.y)/2});const a=document.getElementById("wire-handle"),s=.25*i.x+.5*t.cp.x+.25*r.x,l=.25*i.y+.5*t.cp.y+.25*r.y;a.style.left=s+"px",a.style.top=l+"px",a.classList.add("active"),Zn(a,(d,c)=>{t.cp.x=(d-.25*i.x-.25*r.x)/.5,t.cp.y=(c-.25*i.y-.25*r.y)/.5,N()})}window.showWireHandles=Et;function Zn(e,t){e.onmousedown=n=>{n.stopPropagation();const o=n.clientX,i=n.clientY,r=parseFloat(e.style.left),a=parseFloat(e.style.top),s=d=>{const c=(d.clientX-o)/b.scale,p=(d.clientY-i)/b.scale,g=r+c,f=a+p;t(g,f)},l=()=>{document.removeEventListener("mousemove",s),document.removeEventListener("mouseup",l),R()};document.addEventListener("mousemove",s),document.addEventListener("mouseup",l)}}window.setupHandleDrag=Zn;function Ii(){if(x.length===0){I("No nodes to export","error");return}const e=[],t={},n={};x.forEach(c=>{t[c.id]=0,n[c.id]=[]}),E.forEach(c=>{n[c.fromId]&&t[c.toId]!==void 0&&(n[c.fromId].push(c.toId),t[c.toId]++)});const o=x.filter(c=>t[c.id]===0).map(c=>c.id);for(;o.length>0;){const c=o.shift(),p=x.find(g=>g.id===c);p&&e.push(p),(n[c]||[]).forEach(g=>{t[g]--,t[g]===0&&o.push(g)})}const i=x.filter(c=>!e.find(p=>p.id===c.id));e.push(...i);let r=`// Generated by TernFlow Orchestrator
`;r+=`// Swarm definition: Topological Order Optimized

`;const a=new Set;x.forEach(c=>{c.type==="agent"&&a.add(c.path)}),x.some(c=>c.type==="external")&&a.add("stdlib/agents/binary_bridge.tern"),a.forEach(c=>{c&&(r+=`// from "${c}" import *;
`)}),r+=`
fn main() -> trit {
`,e.forEach(c=>{const p=c.id.replace(/node_|bridge_|gate_/g,"a");c.type==="agent"?r+=`    let ${p}: agentref = spawn ${c.name};
`:c.type==="external"?r+=`    let ${p}: agentref = spawn LLMGateway;
`:r+=`    let ${p}: agentref = spawn TritVote;
`}),r+=`
    // Execution logic (Dependency Aware)
`;const s={};e.forEach(c=>{const p=c.id.replace(/node_|bridge_|gate_/g,"a"),g=E.filter(u=>u.toId===c.id);g.length===0?r+=`    send ${p} affirm;
`:g.forEach((u,m)=>{const h=s[u.fromId]||"affirm";r+=`    send ${p} ${h}; // from ${u.fromId}
`});const f=`res_${p}`;r+=`    let ${f}: trit = await ${p};
`,s[c.id]=f});const l=e[e.length-1],d=l?s[l.id]:"affirm";r+=`
    return ${d};
}
`,Ot(),_&&_.setValue(r),Y("editor"),I("Swarm exported to Editor (Topo-Sorted)","ok")}window.exportFlowCode=Ii;function Si(){var t;if(x.length===0){I("Add agents to the canvas first","error");return}const e=((t=x[0])==null?void 0:t.name)||"";document.getElementById("deployName").value=e,document.getElementById("deployDesc").value="",document.getElementById("deployInput").value="",document.getElementById("deploy-progress").style.display="none",document.getElementById("deploy-confirm-btns").style.display="flex",document.getElementById("deploy-result").style.display="none",Dt(1),document.getElementById("deployModal").style.display="flex",lucide.createIcons()}window.openDeployModal=Si;function ki(){document.getElementById("deployModal").style.display="none"}window.closeDeployModal=ki;function Dt(e){[1,2,3].forEach(t=>{document.getElementById("deploy-step-"+t).style.display=t===e?"flex":"none";const n=document.getElementById("deploy-step-"+t+"-tab");n&&(n.style.cssText=t===e?"flex:1;padding:8px;text-align:center;font-size:10px;font-weight:700;background:rgba(1,118,211,0.2);color:var(--blue);border-right:"+(t<3?"1px solid var(--border2)":"none"):"flex:1;padding:8px;text-align:center;font-size:10px;font-weight:700;color:var(--muted2);border-right:"+(t<3?"1px solid var(--border2)":"none"))}),e===3&&eo()}window.deployStep=Dt;function eo(){var s;const e=document.getElementById("deployName").value.trim()||"my-agent",t=document.getElementById("deployDesc").value.trim()||"—",n=document.getElementById("deployInput").value.trim()||"{ signal: trit }",o=((s=document.querySelector('input[name="deployPricing"]:checked'))==null?void 0:s.value)||"free",i=e.toLowerCase().replace(/\s+/g,"-").replace(/[^a-z0-9-]/g,""),r=x.length,a=E.length;document.getElementById("deploy-summary").innerHTML=`
    <div style="color:var(--muted2);font-size:10px;margin-bottom:8px;letter-spacing:0.05em;">DEPLOYMENT PLAN</div>
    <div><span style="color:var(--muted)">name:    </span><span style="color:var(--cyan)">${e}</span></div>
    <div><span style="color:var(--muted)">slug:    </span><span style="color:var(--text)">/api/agent/${i}</span></div>
    <div><span style="color:var(--muted)">nodes:   </span><span style="color:var(--text)">${r} agents, ${a} wires</span></div>
    <div><span style="color:var(--muted)">pricing: </span><span style="color:${o==="free"?"var(--green)":o==="private"?"var(--muted)":"var(--amber)"}">${o}</span></div>
    <div><span style="color:var(--muted)">input:   </span><span style="color:var(--muted2);font-size:11px;">${n}</span></div>
    <div><span style="color:var(--muted)">desc:    </span><span style="color:var(--muted2);font-size:11px;">${t}</span></div>
  `}window.buildDeploySummary=eo;async function Ti(){var d;Xn();const e=document.getElementById("deployName").value.trim(),t=document.getElementById("deployDesc").value.trim(),n=document.getElementById("deployInput").value.trim(),o=((d=document.querySelector('input[name="deployPricing"]:checked'))==null?void 0:d.value)||"free",i=e.toLowerCase().replace(/\s+/g,"-").replace(/[^a-z0-9-]/g,"");if(!e){I("Enter a product name","error"),Dt(1);return}document.getElementById("deploy-confirm-btns").style.display="none";const r=document.getElementById("deploy-progress");r.style.display="flex";const a=(c,p,g)=>{const f=document.getElementById(c);if(!f)return;f.className="deploy-step-row"+(p==="done"?" done":p==="error"?" error":"");const u=p==="done"?"✓":p==="error"?"✗":"⏳";f.innerHTML=`<span class="dstep-icon">${u}</span> ${g}`};await new Promise(c=>setTimeout(c,600)),a("dstep-compile","done","Flow compiled to .tern");let s=`// Deployed by TernStudio — ${e}
`;s+=`// Nodes: ${x.length} · Wires: ${E.length}

`,x.forEach(c=>{s+=`// Agent: ${c.name}
`,c.props.code&&(s+=c.props.code+`

`)}),await new Promise(c=>setTimeout(c,800));try{const c=document.getElementById("apiEndpoint").value.replace(/\/$/,""),p=document.getElementById("apiKey").value.trim(),g={name:e,slug:i,desc:t,input_schema:n,pricing:o,nodes:x.length,wires:E.length,code:s},f=await fetch(c+"/api/agents/publish",{method:"POST",headers:{"Content-Type":"application/json",...p?{"X-Ternlang-Key":p}:{}},body:JSON.stringify(g)}),u=await f.json();if(f.ok&&u.status==="ok"){a("dstep-register","done",`Endpoint registered: /api/agent/${i}`),await new Promise(y=>setTimeout(y,600)),a("dstep-publish","done","Live on runtime ✓");const m=document.getElementById("deploy-result");m.style.display="block",m.innerHTML=`
       <div style="font-size:13px;font-weight:700;color:var(--green);margin-bottom:8px;">🚀 Deployed successfully!</div>
       <div style="font-family:'JetBrains Mono',monospace;font-size:11px;color:var(--text);line-height:2;">
         <div>Endpoint: <span style="color:var(--cyan)">${c}/api/agent/${i}</span></div>
         <div>Pricing: <span style="color:var(--amber)">${o}</span></div>
         <div style="margin-top:8px;font-size:10px;color:var(--muted2);">Share your endpoint with consumers — they need an API key to call it.</div>
       </div>
      `;let h=[];try{h=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}h.find(y=>y.slug===i)||(h.push({id:i,slug:i,name:e,desc:t,pricing:o,nodes:x.length,deployed:new Date().toISOString()}),localStorage.setItem("ternflow_registry",JSON.stringify(h))),I(`Deployed: ${e}`,"ok")}else throw new Error(u.error||"Server error")}catch(c){console.warn("Deploy error, saving locally:",c),a("dstep-register","done","Registered locally"),await new Promise(f=>setTimeout(f,500)),a("dstep-publish","done","Saved to local fleet registry");const p=document.getElementById("deploy-result");p.style.display="block";const g=document.getElementById("apiEndpoint").value.replace(/\/$/,"");p.innerHTML=`
    <div style="font-size:13px;font-weight:700;color:var(--amber);margin-bottom:8px;">⚠ Saved locally</div>
    <div style="font-family:'JetBrains Mono',monospace;font-size:11px;color:var(--text);line-height:1.8;">
      Endpoint: <span style="color:var(--cyan)">${g}/api/agent/${i}</span><br>
      Status: <span style="color:var(--amber)">Local-Only (Sync Pending)</span>
    </div>
    `,I(`${e} saved to Fleet`,"ok")}let l=[];try{l=JSON.parse(localStorage.getItem("ternflow_registry")||"[]")}catch{}l.find(c=>c.slug===i)||(l.push({id:i,slug:i,name:e,desc:t,pricing:o,nodes:x.length,deployed:new Date().toISOString()}),localStorage.setItem("ternflow_registry",JSON.stringify(l)))}window.executeProductDeploy=Ti;function to(e){var n;if(e.button!==0||e.target.classList.contains("flow-wire")||e.target.classList.contains("wire-hit"))return;const t=e.target.closest(".flow-port");if(t){e.stopPropagation(),e.preventDefault();const o=t.closest(".flow-node"),i=o?o.id:((n=t.closest("[id]"))==null?void 0:n.id)||t.id||"panel",r=document.getElementById("flow-canvas-wrap").getBoundingClientRect();let a="all";t.classList.contains("port-affirm")?a=1:t.classList.contains("port-neutral")?a=0:t.classList.contains("port-reject")&&(a=-1),console.log(`[Ternary] Wire Drag Start: node=${i}, value=${a}`),$={fromId:i,fromIsOutput:t.classList.contains("flow-port-out"),start:te(t),end:Qe(e.clientX-r.left,e.clientY-r.top),routingValue:a},window._activeWireFromId=i,de=!1,V=null,N()}else $=null,N()}document.addEventListener("mousedown",to);window.onMouseDown=to;function no(e){const t=document.getElementById("flow-canvas-wrap");if(!t)return;const n=t.getBoundingClientRect();if($){$.end=Qe(e.clientX-n.left,e.clientY-n.top),document.querySelectorAll(".flow-port").forEach(r=>r.classList.remove("magnet"));let o=null,i=40;document.querySelectorAll(".flow-port").forEach(r=>{const a=r.closest(".flow-node"),s=r.closest("#albert-panel"),l=a?a.id:s?"albert-panel":null;if(!l||l===$.fromId)return;const d=r.classList.contains("flow-port-out");if($.fromIsOutput===d)return;const c=r.getBoundingClientRect(),p=Math.sqrt(Math.pow(e.clientX-(c.left+c.width/2),2)+Math.pow(e.clientY-(c.top+c.height/2),2));p<i&&(i=p,o=r)}),o&&o.classList.add("magnet"),N()}else if(de&&V){const o=(e.clientX-Sn)/b.scale,i=(e.clientY-kn)/b.scale;if(L.size>1)L.forEach(r=>{const a=document.getElementById(r),s=He[r];a&&s&&(a.style.left=s.x+o+"px",a.style.top=s.y+i+"px")});else{const r=document.getElementById(V),a=He[V];r&&a&&(r.style.left=a.x+o+"px",r.style.top=a.y+i+"px")}N()}}document.addEventListener("mousemove",no);window.onMouseMove=no;function oo(e){if(de){if(L.size>1)L.forEach(t=>{const n=x.find(i=>i.id===t),o=document.getElementById(t);if(n&&o){const i=n.type==="artifact"?300:n.type==="moe13"?320:180,r=n.type==="artifact"?200:n.type==="moe13"?360:80;n.x=parseFloat(o.style.left)+i/2,n.y=parseFloat(o.style.top)+r/2}});else if(V){const t=x.find(o=>o.id===V),n=document.getElementById(V);if(t&&n){const o=t.type==="artifact"?300:t.type==="moe13"?320:180,i=t.type==="artifact"?200:t.type==="moe13"?360:80;t.x=parseFloat(n.style.left)+o/2,t.y=parseFloat(n.style.top)+i/2}}if(V){const t=document.getElementById(V);t&&(t.style.zIndex=10)}if(de=!1,V=null,window.globalScheduledEvents&&window.globalScheduledEvents.length>0)Te(G);else{const t=document.getElementById("scrub-layer");t&&t.getContext("2d").clearRect(0,0,t.width,t.height)}R()}if($)try{const n=document.querySelector(".flow-port.magnet")||e.target.closest(".flow-port");if(n){const o=n.id==="albert-port-in",i=n.id==="albert-port-out";if((o||i)&&$.fromId!=="albert-panel"){window._activeWireFromId=$.fromId;const a=new MouseEvent("mouseup",{clientX:e.clientX,clientY:e.clientY});n.dispatchEvent(a),$=null,N();return}const r=n.closest(".flow-node");if(r&&r.id!==$.fromId){const a=n.classList.contains("flow-port-out");if($.fromIsOutput!==a){const s="wire_"+Date.now(),l=$.fromIsOutput?$.fromId:r.id,d=$.fromIsOutput?r.id:$.fromId,c=$.routingValue==="all"?"all":$.routingValue;if(E.some(u=>u.fromId===l&&u.toId===d&&String(u.condition)===String(c))){I("Connection already exists","info");return}const g=c===1?"+1 Affirm":c===-1?"-1 Reject":c===0?"0 Neutral":"All signals";me(),E.push({id:s,fromId:l,toId:d,signal:0,confidence:1,condition:c,transform:"pass",priority:5,label:g}),R();const f=x.find(u=>u.id===l);f&&f.type==="artifact"&&Ln(l)}}}else if($.fromIsOutput){const o=x.find(s=>s.id===$.fromId),i=document.getElementById("flow-canvas-wrap").getBoundingClientRect(),r=Qe(e.clientX-i.left,e.clientY-i.top),a=Math.sqrt(Math.pow(r.x-$.start.x,2)+Math.pow(r.y-$.start.y,2));if(o&&o.type==="artifact"&&o.props.state==="extend"&&a>40){const s=$.end.x,l=$.end.y,d="node_"+Date.now(),c=document.getElementById(`art-body-${o.id}`),p=c?c.querySelector("pre"):null,g=p?p.textContent:c?c.textContent:"";let f={},u=g;try{const h=jsyaml.load(g);h&&typeof h=="object"&&(f=h.context_bridge||{},u=jsyaml.dump(h))}catch{u=`/*
`+g+`
*/`}K("Transmuted Agent","__custom__",s,l,"agent",d);const m=x.find(h=>h.id===d);if(m){m.props.context_bridge=f;const h=jsyaml.dump(f);m.props.code=`// Inherited Context Bridge:
/*
${h}*/

// Source Payload:
/*
${u}*/

fn main() -> trit {
    return truth();
}`;const y="wire_evo_"+Date.now();E.push({id:y,fromId:o.id,toId:d,signal:0,confidence:1,condition:"all",transform:"pass",priority:5,label:"EVOLUTION"}),Jn(o.id,"lock"),A("SYSTEM","🧬 Evolution Triggered — New node ready.")}}}}catch(t){console.error("Critical error in mouseup:",t)}document.querySelectorAll(".flow-port").forEach(t=>t.classList.remove("magnet")),$=null,de=!1,V=null,N()}document.addEventListener("mouseup",oo);window.onMouseUp=oo;async function Ci(e){H=!1,Oe=!0,Xe(),we.push({toId:e,val:1,conf:1,origin:"CONTINUATION"}),D!=="running"&&await Pt()}window.resumeSimulationFrom=Ci;const _i="https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/",tt=_i+"ternlang-root/",nn="https://api.github.com/repos/eriirfos-eng/ternary-intelligence-stack/contents/ternlang-root/",Bi=["core","ternary","std","showcase","bughunt","testing","bench","benchmarks","classical","errors","tutorials","lib"];async function Pe(){const e=document.getElementById("file-tree");e.innerHTML='<div style="padding:10px; color:var(--muted); font-size:11px;">Loading from GitHub…</div>';try{const t=[];for(const n of Bi)try{const o=await fetch(nn+`stdlib/${n}`);if(!o.ok)continue;const i=await o.json();Array.isArray(i)&&i.filter(r=>r.name.endsWith(".tern")).forEach(r=>t.push(`stdlib/${n}/${r.name}`))}catch{}try{const n=await fetch(nn+"examples");if(n.ok){const o=await n.json();Array.isArray(o)&&o.filter(i=>i.name.endsWith(".tern")).forEach(i=>t.push(`examples/${i.name}`))}}catch{}if(t.length===0){Ge(e);return}nt(e,t,!0)}catch{Ge(e)}}window.loadGithubTree=Pe;function Ge(e){io(e)}window.showNoKeyMessage=Ge;function io(e){e.innerHTML="";const t=document.createElement("div");t.style.cssText="padding:6px 12px; font-size:10px; color:var(--muted2); background:rgba(255,255,255,0.02); border-bottom:1px solid var(--border); display:flex; justify-content:space-between; align-items:center;",t.innerHTML='<span>Built-in Examples</span><span style="cursor:pointer;color:var(--cyan)" onclick="toggleKeyInput()">+ Add key</span>',e.appendChild(t),[{name:"hello_trit.tern",path:"examples/hello_trit.tern",key:"hello"},{name:"consensus.tern",path:"examples/consensus.tern",key:"consensus"},{name:"match_signal.tern",path:"examples/match_signal.tern",key:"match"},{name:"trit_gate.tern",path:"examples/trit_gate.tern",key:"gate"},{name:"agent_basic.tern",path:"examples/agent_basic.tern",key:"agent"}].forEach(({name:o,path:i,key:r})=>{const a=document.createElement("div");a.className="tree-file",a.dataset.path=i,a.dataset.builtin="1",a.innerHTML=`<i data-lucide="file-code" style="width:12px;height:12px"></i> ${o}`,a.onclick=()=>{P[i]=P[i]||ue[r]||ue.hello,X(i,P[i]),Y("editor")},e.appendChild(a)}),e.dataset.loaded="true",J(),lucide.createIcons()}window.renderBuiltinTree=io;function nt(e,t,n=!1,o=!1){const i={};if(t.forEach(r=>{const a=r.split("/");if(a.length<2)return;const s=a[1];i[s]||(i[s]=[]),i[s].push(r)}),Object.keys(i).length===0){o||Ge(e);return}if(e.innerHTML="",n){const r=document.createElement("div");r.style.cssText="padding:6px 12px; font-size:10px; color:var(--amber); background:rgba(245,158,11,0.08); border-bottom:1px solid var(--border);",r.innerHTML='Tier 1 — GitHub · <span style="cursor:pointer;color:var(--cyan)" onclick="toggleKeyInput()">Enter key for full access</span>',e.appendChild(r)}Object.keys(i).sort().forEach(r=>{const a=document.createElement("div");a.className="tree-section";const s=document.createElement("div");s.className="tree-dir collapsed",s.innerHTML=`<span class="arrow">▸</span> <i data-lucide="folder" style="width:12px; height:12px"></i> ${r}/`;const l=document.createElement("div");l.className="tree-files hidden",s.onclick=()=>{const d=l.classList.toggle("hidden");s.classList.toggle("collapsed",d),s.querySelector(".arrow").textContent=d?"▸":"▾",d?l.innerHTML="":(i[r].forEach(c=>{const p=c.split("/").pop(),g=document.createElement("div");g.className="tree-file",g.dataset.path=c,g.dataset.github=n?"1":"",g.dataset.premium=o?"1":"",g.innerHTML=`<i data-lucide="file-code" style="width:12px; height:12px"></i> ${p}`,g.onclick=()=>{ro(c,n,o),Y("editor")},l.appendChild(g)}),J(),lucide.createIcons({root:l}))},a.appendChild(s),a.appendChild(l),e.appendChild(a)}),e.dataset.loaded="true",J(),lucide.createIcons({root:e})}window.renderFileTree=nt;async function Ve(e=!1){const t=document.getElementById("file-tree");if(!e&&t.dataset.loaded==="true"){J();return}t.innerHTML='<div style="padding:10px; color:var(--muted); font-size:11px;">Loading library…</div>';try{const n=document.getElementById("apiEndpoint").value.replace(/\/$/,""),o=document.getElementById("apiKey").value.trim(),r=await(await fetch(n+"/api/stdlib/list",{headers:o?{"X-Ternlang-Key":o}:{}})).json();if(r.status!=="ok"){await Pe();return}if(!r.files||r.files.length===0){await Pe();return}nt(t,r.files,!1),t.dataset.loaded="true",J(),lucide.createIcons()}catch{await Pe()}}window.buildFileTree=Ve;function J(){document.querySelectorAll(".tree-file").forEach(e=>{e.classList.toggle("active",e.dataset.path===W)})}window.refreshTreeHighlight=J;function se(){const e=document.getElementById("editorTabs");e.innerHTML="",q.forEach(({name:t,path:n})=>{const o=document.createElement("div");o.className="tab"+(n===W?" active":""),o.innerHTML=`<span style="flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;">${t}</span><button class="tab-close" onclick="closeTab('${CSS.escape(n)}',event)" title="Close">✕</button>`,o.onclick=()=>ot(n),e.appendChild(o)})}window.renderTabs=se;function Li(e,t){t&&t.stopPropagation();const n=q.findIndex(o=>o.path===e);if(n!==-1){if(q.splice(n,1),q.length===0){Ot();return}if(W===e){const o=q[Math.min(n,q.length-1)];ot(o.path)}else se()}}window.closeTab=Li;function $i(){document.getElementById("localFileInput").click()}window.triggerImportFile=$i;function Ai(e){const t=e.target.files[0];if(!t)return;const n=new FileReader;n.onload=o=>{const i=o.target.result,r="local/"+t.name;P[r]=i,X(r,i),Y("editor"),I(`Imported ${t.name}`,"ok")},n.readAsText(t),e.target.value=""}window.importLocalFile=Ai;function ot(e){_&&(P[W]=_.getValue()),W=e,_&&_.setValue(P[e]||""),se(),J(),Ce();const t=document.getElementById("sbFile");t&&(t.textContent=e.split("/").pop())}window.switchToTab=ot;async function ro(e,t=!1,n=!1){if(_&&(P[W]=_.getValue()),P[e]){X(e,P[e]);return}const o=document.querySelector(`.tree-file[data-path="${CSS.escape(e)}"]`),i=n||o&&o.dataset.premium==="1",r=t||o&&o.dataset.github==="1";if(i)try{const a=document.getElementById("apiEndpoint").value.replace(/\/$/,""),s=await fetch(`${a}/api/premium/file?path=${encodeURIComponent(e)}`,{headers:{"X-Ternlang-Key":localStorage.getItem("ternstudio-key")||""}});if(!s.ok)throw new Error(`Auth failed or file not found (${s.status})`);const l=await s.json();if(l.content)P[e]=l.content,X(e,l.content);else throw new Error("Invalid response from server");return}catch(a){I(`Failed to load premium file: ${a.message}`,"err");return}if(r){try{const a=await fetch(tt+e);if(a.ok){const s=await a.text();P[e]=s,X(e,s);return}}catch{}I("Failed to load from GitHub","err");return}try{const a=document.getElementById("apiEndpoint").value.replace(/\/$/,""),s=document.getElementById("apiKey").value.trim(),d=await(await fetch(a+"/api/stdlib/read/"+e,{headers:s?{"X-Ternlang-Key":s}:{}})).json();d.status==="ok"?(P[e]=d.content,X(e,d.content)):I(d.error||"Failed to read file","err")}catch{I("Connection Error","err")}}window.openFile=ro;function X(e,t){if(q.find(i=>i.path===e)||q.push({name:e.split("/").pop(),path:e}),W=e,_){_.setValue(t);const i=_.getModel();e.split(".").pop()==="tern"&&monaco.editor.setModelLanguage(i,"ternlang")}se(),J(),Ce();const o=document.getElementById("sbFile");o&&(o.textContent=e.split("/").pop())}window.loadToEditor=X;function Ot(){const e=`scratch_${Io++}.tern`,t=`scratch/${e}`;P[t]=ue.hello,q.push({name:e,path:t}),ot(t),Ce()}window.newFile=Ot;function Mi(e){_&&_.setValue(ue[e]||"")}window.insertTemplate=Mi;function Ri(e){document.querySelectorAll(".sidebar-panel").forEach(t=>t.classList.remove("active")),document.querySelectorAll(".act-btn").forEach(t=>t.classList.remove("active")),document.getElementById("panel-"+e).classList.add("active"),document.getElementById("act-"+e).classList.add("active")}window.switchSidebarPanel=Ri;function Ni(e){e.preventDefault();const t=document.getElementById("editor-sidebar"),n=e.clientX,o=t.offsetWidth,i=a=>{const s=Math.max(120,Math.min(400,o+a.clientX-n));document.documentElement.style.setProperty("--sidebar-w",s+"px"),_&&_.layout()},r=()=>{document.removeEventListener("mousemove",i),document.removeEventListener("mouseup",r)};document.addEventListener("mousemove",i),document.addEventListener("mouseup",r)}window.startSidebarResize=Ni;function Pi(e){e.preventDefault();const t=document.getElementById("output-panel"),n=e.clientY,o=t.offsetHeight,i=a=>{const s=Math.max(80,Math.min(window.innerHeight*.6,o+n-a.clientY));document.documentElement.style.setProperty("--output-h",s+"px"),_&&_.layout()},r=()=>{document.removeEventListener("mousemove",i),document.removeEventListener("mouseup",r)};document.addEventListener("mousemove",i),document.addEventListener("mouseup",r)}window.startOutputResize=Pi;function zi(e,t){document.querySelectorAll(".out-tab").forEach(n=>n.classList.remove("active")),document.querySelectorAll(".out-panel").forEach(n=>n.classList.remove("visible")),t.classList.add("active"),document.getElementById("panel-"+e).classList.add("visible"),e==="api"&&(document.getElementById("apiEndpointBase").textContent=document.getElementById("apiEndpoint").value)}window.switchOutTab=zi;function it(e,t){const n=document.getElementById("vmStatus");n&&(n.className="status-pill status-"+e,n.textContent=t);const o=document.getElementById("connDot");o&&(o.className="sb-dot"+(e==="error"?" err":e==="running"?" warn":""))}window.setStatus=it;function Di(){it("idle","● Idle"),document.getElementById("printOutput").textContent="— no output —",document.getElementById("printOutput").className="print-output empty",document.getElementById("section-meta").style.display="none",document.getElementById("section-regs").style.display="none",document.getElementById("section-error").style.display="none"}window.clearOutput=Di;window.addEventListener("wasmready",()=>{const e=document.getElementById("wasmBadge");e&&(e.style.opacity="1");const t=document.getElementById("sbWasmStatus");t&&(t.textContent="⚡ WASM",t.style.color="var(--green)")});function jt(e){const t=e.status==="ok";it(t?"ok":"error",t?e._wasm?"⚡ OK (WASM)":"✓ OK":"✕ Error"),document.getElementById("section-meta").style.display="block";const n=e._wasm?e._ms!=null?`${e._ms}ms WASM`:"WASM":e.bytecode_bytes!=null?e.bytecode_bytes+"B":"—";document.getElementById("metaBytes").textContent=n,document.getElementById("metaStatus").textContent=t?"exited ok":"vm error",document.getElementById("metaStatus").style.color=t?"var(--green)":"var(--red)";const o=document.getElementById("printOutput");o.innerHTML="",o.className="print-output";const i=document.createElement("div");if(i.className="term-header",i.textContent=`Ternary Intelligence Stack — BET-VM v1.0.0 (${e._wasm?"WASM":"API"})`,o.appendChild(i),e.output&&e.output.length>0&&e.output.forEach(a=>{const s=document.createElement("div");s.className="term-line",s.innerHTML=`<span class="term-prompt">></span><span>${a}</span>`,o.appendChild(s)}),t){const a=document.createElement("div");a.className="term-line term-success",a.style.marginTop="8px",a.textContent=`● Program exited successfully. [trit:${e.trit}]`,o.appendChild(a)}else{const a=document.createElement("div");a.className="term-err-line";let s=e.error||"Unknown runtime error";if(s.includes("ExpectedToken")){const d=s.match(/ExpectedToken\("(.*?)",\s+"(.*?)"\)/);d&&(s=`Syntax Error: Expected ${d[1]}, but found ${d[2]}. Check your semicolons!`)}else if(s.includes("UndefinedSymbol")){const d=s.match(/UndefinedSymbol\("(.*?)"\)/);d&&(s=`Reference Error: Symbol '${d[1]}' is not defined.`)}a.innerHTML=`<strong>VM_ERROR:</strong> ${s}`,o.appendChild(a);const l=document.createElement("div");l.className="term-line term-dim",l.textContent="● Process terminated with non-zero exit code.",o.appendChild(l)}if(e.registers&&e.registers.length>0){document.getElementById("section-regs").style.display="block";const a=document.getElementById("regTable");a.innerHTML="",e.registers.forEach((s,l)=>{const d=document.createElement("tr"),c=String(s);let p="";c.includes("Affirm")||c.includes("Truth")?p="reg-row-affirm":c.includes("Reject")||c.includes("Conflict")?p="reg-row-reject":(c.includes("Tend")||c==="Trit(Tend)")&&(p="reg-row-zero"),d.className=p,d.innerHTML=`<td>r${l}</td><td>${c}</td>`,a.appendChild(d)})}const r=document.getElementById("section-error");!t&&e.error?(r.style.display="block",document.getElementById("errorOutput").textContent=e.error):r.style.display="none",Ft(e.registers||[])}window.showResult=jt;function Ft(e=[]){const t=document.getElementById("logic-field-grid");if(!t)return;t.innerHTML="";const n=Math.max(27,Math.min(64,e.length||0));for(let o=0;o<n;o++){const i=document.createElement("div");if(i.className="trit-cell",i.title=`r${o}`,e[o]!=null){const r=String(e[o]);r.includes("Affirm")||r.includes("Truth")?(i.classList.add("active-affirm"),i.textContent="+"):r.includes("Reject")||r.includes("Conflict")?(i.classList.add("active-reject"),i.textContent="-"):(i.classList.add("active-tend"),i.textContent="0")}else i.textContent=".",i.style.opacity="0.3";t.appendChild(i)}}window.renderLogicField=Ft;function ao(e){if(e=e.trim(),!e)return e;const t=/fn\s+main\s*\(/.test(e),n=/^fn\s+\w+/m.test(e);return t?e:n?e+`

fn main() -> trit { return hold; }`:`fn main() -> trit {
${e.split(`
`).map(i=>{let r=i.trim();return r&&!r.endsWith(";")&&!r.endsWith("}")&&!r.endsWith("{")&&!r.startsWith("//")?i+";":i}).join(`
`)}
    return hold;
}`}window.prepareTernCode=ao;function rt(e){if(!window.wasmReady||!window.wasmRunTern)return{ok:!1,error:"BET-VM (WASM) not loaded. Check network."};const t=ao(e);try{const n=window.wasmRunTern(t),o=JSON.parse(n);return{ok:o.ok,output:o.output||[],trit:o.trit??0,label:o.label||"hold",registers:o.registers||[],error:o.error||null,cycles:o.cycles||0}}catch(n){const o={type:"WASM_PANIC",error:n.message,stack:n.stack,payload_len:t.length,timestamp:new Date().toISOString()};return console.error("🛑 TERNLANG_CRITICAL_DEBUG:",o),{ok:!1,error:"VM_PANIC: "+n.message,traceback:n.stack}}}window.runTernCode=rt;async function It(){if(!_)return;P[W]=_.getValue();const e=_.getValue();if(!e.trim())return;const t=Date.now(),n=document.getElementById("runBtn");n.classList.add("running"),n.textContent="● Running…",it("running","● Running…"),document.getElementById("view-editor").classList.contains("active")||Y("editor"),document.querySelectorAll(".out-tab").forEach(a=>a.classList.remove("active")),document.querySelectorAll(".out-panel").forEach(a=>a.classList.remove("visible")),document.querySelector(".out-tab").classList.add("active"),document.getElementById("panel-output").classList.add("visible");const o=rt(e),i=Date.now()-t,r={status:o.ok?"ok":"error",output:o.output,trit:o.trit,label:o.label,registers:o.registers,error:o.error,bytecode_bytes:o.cycles,_ms:i,_wasm:!0};jt(r),so(W,o.ok,i,r),n.classList.remove("running"),n.innerHTML='<span>▶ Run</span><span class="kbd">Ctrl+↵</span>'}window.runCode=It;function so(e,t,n,o){Yt++,t?Jt++:Xt++,document.getElementById("statRuns").textContent=Yt,document.getElementById("statOk").textContent=Jt,document.getElementById("statErr").textContent=Xt;const i={path:e,ok:t,ms:n,ts:new Date().toLocaleTimeString(),code:_?_.getValue():"",data:o};Q.unshift(i),Q.length>20&&Q.pop(),Ht(),lo()}window.addRunToHistory=so;function Ht(){const e=document.getElementById("history-list");if(Q.length===0){e.innerHTML='<div style="padding:12px; font-size:11px; color:var(--muted); font-style:italic;">No runs yet</div>';return}e.innerHTML=Q.map((t,n)=>`
    <div class="hist-item" onclick="restoreRun(${n})">
      <div class="hist-name">${t.path.split("/").pop()}</div>
      <div class="hist-meta">
        <span class="${t.ok?"hist-ok":"hist-err"}">${t.ok?"✓ ok":"✕ err"}</span>
        <span>${t.ts}</span>
        <span>${t.ms}ms</span>
      </div>
    </div>`).join("")}window.renderHistory=Ht;function Oi(e){const t=Q[e];t&&(q.find(n=>n.path===t.path)||q.push({name:t.path.split("/").pop(),path:t.path}),W=t.path,P[t.path]=t.code,_&&_.setValue(t.code),se(),J(),jt(t.data),Y("editor"))}window.restoreRun=Oi;function ji(){Q=[],Ht()}window.clearHistory=ji;function lo(){const e=document.getElementById("dashRunList");if(Q.length===0){e.innerHTML='<div class="dash-run-empty">No runs yet this session</div>';return}e.innerHTML=Q.slice(0,5).map(t=>`
    <div class="dash-run-item">
      <span class="run-status ${t.ok?"run-ok":"run-err"}">${t.ok?"✓":"✕"}</span>
      <span class="run-name">${t.path.split("/").pop()}</span>
      <span style="font-size:10px; color:var(--muted);">${t.ts}</span>
    </div>`).join("")}window.updateDashboardRuns=lo;function Fi(){if(!_)return;const e=_.getValue(),t="#code="+btoa(encodeURIComponent(e)),n=location.origin+location.pathname+t;navigator.clipboard.writeText(n).then(()=>I("Share URL copied to clipboard","ok")).catch(()=>I("Copy failed — check browser permissions","err"))}window.shareCode=Fi;function Hi(){if(!_)return;const e=_.getValue(),t=W.split("/").pop()||"scratch.tern",n=new Blob([e],{type:"text/plain"}),o=document.createElement("a");o.href=URL.createObjectURL(n),o.download=t,o.click(),URL.revokeObjectURL(o.href),I("Downloaded "+t,"ok")}window.downloadCode=Hi;function I(e,t=""){const n=document.getElementById("toast-container"),o=document.createElement("div");o.className="toast"+(t?" "+t:""),o.textContent=e,n.appendChild(o),setTimeout(()=>o.remove(),3e3)}window.showToast=I;function co(){const e=location.hash;if(e.startsWith("#code="))try{const t=decodeURIComponent(atob(e.slice(6))),n="scratch/shared.tern";P[n]=t,q.find(o=>o.path===n)||q.push({name:"shared.tern",path:n}),W=n,_&&_.setValue(t),se(),J(),document.getElementById("sbFile").textContent="shared.tern",Y("editor"),I("Loaded shared code","ok")}catch{I("Failed to decode shared URL","err")}}window.loadFromHash=co;async function Wi(){const e=document.getElementById("apiEndpoint").value.replace(/\/$/,"");try{const n=await(await fetch(e+"/health")).json();document.getElementById("apiResponse").textContent=JSON.stringify(n,null,2)}catch(t){document.getElementById("apiResponse").textContent=String(t)}}window.tryHealth=Wi;async function qi(){const e=document.getElementById("apiEndpoint").value.replace(/\/$/,""),t=document.getElementById("apiKey").value.trim();try{const o=await(await fetch(e+"/api/usage",{headers:t?{"X-Ternlang-Key":t}:{}})).json();document.getElementById("apiResponse").textContent=JSON.stringify(o,null,2)}catch(n){document.getElementById("apiResponse").textContent=String(n)}}window.tryApiUsage=qi;async function Ki(){const e=document.getElementById("apiEndpoint").value.replace(/\/$/,""),t=document.getElementById("apiKey").value.trim(),o={code:_?_.getValue():""};t&&(o.key=t);try{const r=await(await fetch(e+"/api/run",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(o)})).json();document.getElementById("apiResponse").textContent=JSON.stringify(r,null,2)}catch(i){document.getElementById("apiResponse").textContent=String(i)}}window.tryApiRun=Ki;const Ui={hello:`fn main() -> trit {
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
}`};function Gi(e){document.getElementById("dashReplInput").value=Ui[e]||""}window.setReplSnippet=Gi;function Vi(){const e=document.getElementById("dashReplInput").value.trim();if(!e)return;const t="playground/repl.tern";P[t]=e,X(t,e),Y("editor")}window.openReplInEditor=Vi;async function Yi(){const e=document.getElementById("dashReplInput").value.trim(),t=document.getElementById("dashReplRes");if(!e)return;t.style.color="var(--muted)",t.textContent="running…";const n=Date.now(),o=rt(e),i=Date.now()-n;if(o.ok){const r=o.trit,a=o.label||(r===1?"affirm":r===-1?"reject":"tend"),s=r===1?"var(--green)":r===-1?"var(--red)":"var(--amber)",l=r===1?"+1":r===-1?"-1":"0";t.style.color=s;let d=`${l}  ${a.toUpperCase()}  [${i}ms WASM]`;o.output&&o.output.length&&(d+=`
`+o.output.join(`
`)),t.textContent=d}else t.style.color="var(--red)",t.textContent=o.error||"Error"}window.runReplExpr=Yi;function po(){const e=localStorage.getItem("ternstudio-key"),t=localStorage.getItem("ternstudio-save-key")==="1";e&&t?(document.getElementById("apiKey").value=e,document.getElementById("saveKeyCheck").checked=!0):document.getElementById("saveKeyCheck").checked=t}window.initKeyPersistence=po;function Ji(){const e=document.getElementById("saveKeyCheck").checked;if(localStorage.setItem("ternstudio-save-key",e?"1":"0"),e){let t=(document.getElementById("settingsNewKey")||{}).value||"";t.trim()||(t=document.getElementById("apiKey").value.trim()),t&&(document.getElementById("apiKey").value=t.trim(),localStorage.setItem("ternstudio-key",t.trim()),at())}else localStorage.removeItem("ternstudio-key")}window.toggleSaveKey=Ji;function Xi(){const e=(document.getElementById("settingsNewKey")||{}).value||"";e.trim()&&(cn(e.trim()),document.getElementById("settingsNewKey").value="")}window.applySettingsKey=Xi;function at(){const e=(document.getElementById("apiKey")||{value:""}).value.trim(),t=e.length>12?e.slice(0,8)+"…"+e.slice(-4):e||"—",n=document.getElementById("settingsKeyDisplay");n&&(n.textContent=t)}window.syncSettingsKeyDisplay=at;function Qi(){const e=document.getElementById("apiKey").value.trim();e&&navigator.clipboard.writeText(e).then(()=>I("Key copied","ok"))}window.copyKey=Qi;function Zi(){document.getElementById("apiKey").value="",localStorage.removeItem("ternstudio-key"),st(),at(),I("Key cleared","ok")}window.clearKey=Zi;function er(){const e=document.getElementById("settingsEndpoint").value.trim();e&&(document.getElementById("apiEndpoint").value=e,Je(),I("Endpoint updated","ok"))}window.applyEndpoint=er;function uo(){document.getElementById("saveKeyCheck").checked=localStorage.getItem("ternstudio-save-key")==="1",at();const e=document.documentElement.getAttribute("data-theme")||"dark";document.getElementById("settingsTheme").value=e,qt()}window.syncSettingsUI=uo;function tr(){const e=parseInt(document.getElementById("settingsFontSize").value),t=document.getElementById("settingsMinimap").value==="true",n=document.getElementById("settingsWordWrap").value;document.documentElement.style.fontSize=e+"px",_&&_.updateOptions({fontSize:e,minimap:{enabled:t},wordWrap:n})}window.applyEditorSettings=tr;function nr(){const e=document.getElementById("settingsTheme").value;localStorage.setItem("ternstudio-theme",e),Be(e),_&&monaco.editor.setTheme(e==="light"?"ternstudio-light":"ternstudio-dark")}window.applyThemeFromSettings=nr;function _e(){try{return JSON.parse(localStorage.getItem("ternflow_secrets")||"{}")}catch{return{}}}function Wt(e,t){const n=_e();t?n[e]=t:delete n[e],localStorage.setItem("ternflow_secrets",JSON.stringify(n)),qt()}window.setTernflowSecret=Wt;function or(){const e=document.getElementById("newSecretProvider").value,t=document.getElementById("newSecretKey").value.trim();t&&(Wt(e,t),document.getElementById("newSecretKey").value="",I(`Secret for ${e} updated`,"ok"))}window.addVaultSecret=or;function qt(){const e=document.getElementById("secretsVaultList");if(!e)return;const t=_e(),n=["openai","anthropic","google","custom"];e.innerHTML=n.map(o=>{const i=t[o]||"",r=i?i.slice(0,8)+"…"+i.slice(-4):"Not set";return`
      <div style="display:grid; grid-template-columns:140px 1fr auto; gap:12px; align-items:center; padding-bottom:12px; border-bottom:1px solid var(--border2);">
        <div style="font-size:12px; font-weight:600; color:var(--text); text-transform:capitalize;">${o}</div>
        <div style="font-family:'JetBrains Mono',monospace; font-size:11px; color:${i?"var(--cyan)":"var(--muted2)"};">${r}</div>
        <button class="settings-btn" onclick="setTernflowSecret('${o}', '')" style="color:var(--red); border-color:rgba(239,68,68,0.2); transition:background 0.2s;" onmouseover="this.style.background='rgba(239,68,68,0.1)'" onmouseout="this.style.background='transparent'">Clear</button>
      </div>
    `}).join("")}window.renderVaultUI=qt;const on={1:["● /api/run — execute .tern programs","● /health, /studio","○ Trit decisions + vectors (Tier 2+)","○ MoE-13 orchestrator (Tier 2+)","○ Deliberation engine (Tier 2+)"],2:["● /api/run","● /api/trit_decide, /api/trit_vector","● /api/trit_consensus, /api/trit_gate","● /api/moe/orchestrate","● Deliberation + coalition engine","○ Industrial endpoints (Tier 3+)"],3:["● All Tier 2 endpoints","● /api/v1/taas/infer","● /api/stream/* SSE endpoints","● Sparse benchmark + quantization","○ Enterprise SLA (Tier 4)"],4:["● All endpoints, unlimited","● Enterprise SLA + dedicated support","● Custom rate limits","● Private deployment options"]};function st(){document.getElementById("usageKeyDisplay").textContent="No key — anonymous access",document.getElementById("usageTierBadge").className="tb-badge badge-free",document.getElementById("usageTierBadge").textContent="Tier 1",document.getElementById("usage-quota-section").style.display="none",document.getElementById("usage-unlimited-section").style.display="none",document.getElementById("usageError").style.display="none",Kt(1),Ut(1)}window.renderUsageAnon=st;function Kt(e){const t=document.getElementById("tierBenefitsList"),n=on[e]||on[1];t.innerHTML=n.map((i,r)=>`<div class="tier-benefit ${i.startsWith("○")?"muted-benefit":""}">${i}</div>`).join("");const o=document.getElementById("upgradeBtn");o.style.display=e>=4?"none":"inline-block"}window.renderTierBenefits=Kt;function Ut(e){const t=document.getElementById("tierBadge");t&&(t.textContent=Re[e]||"Tier 1",t.className="tier-badge tb-badge "+(mt[e]||"badge-free"));const n=document.getElementById("sbTier");n&&(n.textContent=Re[e]||"Tier 1 — Open Core");const o=document.getElementById("topbarUpskillBtn");o&&(o.style.display=e<=1?"flex":"none");const i=document.getElementById("dashTierBadge");i&&(i.textContent=Re[e]||"Tier 1 — Free",i.className="tb-badge "+(mt[e]||"badge-free"))}window.updateTopbarTier=Ut;async function Ye(){const e=document.getElementById("apiEndpoint").value.replace(/\/$/,""),t=document.getElementById("apiKey").value.trim();if(!t){st();return}try{const n=await fetch(e+"/api/usage",{headers:{"X-Ternlang-Key":t},signal:AbortSignal.timeout(5e3)});if(!n.ok){const s=await n.text();document.getElementById("usageError").style.display="block",document.getElementById("usageErrorMsg").textContent=`HTTP ${n.status} — ${s.slice(0,200)}`;return}const o=await n.json();document.getElementById("usageError").style.display="none";const i=t.length>12?t.slice(0,8)+"…"+t.slice(-4):t;document.getElementById("topbarKeyInput").value=t;const r=o.tier||1,a=document.getElementById("usageTierBadge").dataset.tier;if(a&&a!=r&&(document.getElementById("file-tree").dataset.loaded="false",Se(),Ve()),document.getElementById("usageTierBadge").dataset.tier=r,document.getElementById("usageKeyDisplay").textContent=i,Ut(r),document.getElementById("usageTierBadge").textContent=Re[r]||"Tier 1",document.getElementById("usageTierBadge").className="tb-badge "+(mt[r]||"badge-free"),Kt(r),Ve(!0),Se(),o.limit===null||o.limit===void 0||r>=4)document.getElementById("usage-quota-section").style.display="none",document.getElementById("usage-unlimited-section").style.display="block",document.getElementById("dash-unlimited-badge").style.display="block",document.getElementById("dash-quota-wrap").style.display="none";else{document.getElementById("usage-unlimited-section").style.display="none",document.getElementById("usage-quota-section").style.display="block";const s=o.calls_this_month||0,l=o.limit||0,d=l>0?Math.min(100,Math.round(s/l*100)):0,c=Math.max(0,l-s);document.getElementById("usageUsed").textContent=s.toLocaleString(),document.getElementById("usageLimit").textContent=l.toLocaleString();const p=document.getElementById("usageBarFill");p.style.width=d+"%",p.className="usage-bar-fill "+(d>=90?"crit":d>=70?"warn":"ok"),document.getElementById("usageMeta").textContent=`${d}% used · ${c.toLocaleString()} remaining · resets 1st of month`;const g=document.getElementById("dashUsageBar");g.style.width=d+"%",g.className="usage-bar-fill "+(d>=90?"crit":d>=70?"warn":"ok"),document.getElementById("dashUsageMeta").textContent=`${s.toLocaleString()} / ${l.toLocaleString()} · ${d}%`}}catch(n){document.getElementById("usageError").style.display="block",document.getElementById("usageErrorMsg").textContent=String(n)}}window.fetchUsage=Ye;async function Je(){const e=document.getElementById("apiEndpoint").value.replace(/\/$/,""),t=document.getElementById("dashVmDot"),n=document.getElementById("dashVmLabel");try{if((await fetch(e+"/health",{signal:AbortSignal.timeout(4e3)})).ok){const i=document.getElementById("connLabel");i&&(i.textContent="Connected · "+e.replace("https://",""));const r=document.getElementById("connDot");r&&(r.className="sb-dot"),t&&(t.className="dot"),n&&(n.textContent="Online · "+e.replace("https://",""))}else throw new Error("not ok")}catch{const o=document.getElementById("connLabel");o&&(o.textContent="Offline");const i=document.getElementById("connDot");i&&(i.className="sb-dot err"),t&&(t.className="dot err"),n&&(n.textContent="Offline")}}window.checkConnection=Je;function ir(){document.getElementById("upskillModal").style.display="none",document.body.style.overflow=""}document.addEventListener("keydown",e=>{e.key==="Escape"&&ir()});function Be(e){const t=document.documentElement,n=document.getElementById("themeBtn"),o=document.getElementById("themeIcon");e==="light"?(t.setAttribute("data-theme","light"),n&&(n.title="Switch to dark mode"),o&&o.setAttribute("data-lucide","sun")):(t.removeAttribute("data-theme"),n&&(n.title="Switch to light mode"),o&&o.setAttribute("data-lucide","moon")),typeof lucide<"u"&&lucide.createIcons()}window.applyTheme=Be;function rr(){const t=(document.documentElement.getAttribute("data-theme")==="light"?"light":"dark")==="light"?"dark":"light";localStorage.setItem("ternstudio-theme",t),Be(t),_&&monaco.editor.setTheme(t==="light"?"ternstudio-light":"ternstudio-dark")}window.toggleTheme=rr;(function(){const e=localStorage.getItem("ternstudio-theme")||"dark";Be(e)})();require.config({paths:{vs:"https://cdn.jsdelivr.net/npm/monaco-editor@0.52.0/min/vs"}});require(["vs/editor/editor.main"],function(){monaco.languages.register({id:"ternlang"}),monaco.languages.setMonarchTokensProvider("ternlang",{keywords:["fn","let","return","match","if","else","while","for","in","break","continue","spawn","send","await"],types:["trit","int","float","bool","void"],builtins:["truth","hold","conflict","consensus","print","println","affirm","reject","tend"],directives:["sparseskip","inline","export"],tokenizer:{root:[[/@[a-zA-Z_]+/,"keyword.directive"],[/\/\/.*$/,"comment"],[/\b(fn|let|return|match|if|else|while|for|in|break|continue|spawn|send|await)\b/,"keyword"],[/\b(trit|int|float|trittensor|void)\b/,"type"],[/\b(truth|hold|conflict|consensus|print|println|affirm|reject|tend)\b/,"support.function"],[/-1\b/,"number.trit.reject"],[/\b0\b/,"number.trit.hold"],[/\b1\b/,"number.trit.affirm"],[/\b\d+\.\d+\b/,"number.float"],[/\b\d+\b/,"number"],[/"([^"\\]|\\.)*"/,"string"],[/->/,"operator"],[/=>/,"operator"],[/[+\-*\/=!<>?%]/,"operator"],[/[{}()\[\];,:]/,"delimiter"],[/[a-zA-Z_]\w*/,"identifier"]]}}),monaco.editor.defineTheme("ternstudio-dark",{base:"vs-dark",inherit:!0,rules:[{token:"comment",foreground:"3a5060",fontStyle:"italic"},{token:"keyword",foreground:"00c8ff",fontStyle:"bold"},{token:"keyword.directive",foreground:"ffaa00",fontStyle:"bold"},{token:"type",foreground:"80e8ff"},{token:"support.function",foreground:"00e87a"},{token:"number.trit.affirm",foreground:"00e87a",fontStyle:"bold"},{token:"number.trit.reject",foreground:"ff3b55",fontStyle:"bold"},{token:"number.trit.hold",foreground:"ffaa00",fontStyle:"bold"},{token:"number.float",foreground:"a8d8ff"},{token:"number",foreground:"a8d8ff"},{token:"string",foreground:"c8f0a0"},{token:"operator",foreground:"6090b0"},{token:"delimiter",foreground:"405060"},{token:"identifier",foreground:"c8d4e0"}],colors:{"editor.background":"#080b10","editor.foreground":"#c8d4e0","editor.lineHighlightBackground":"#0d1219","editorCursor.foreground":"#00c8ff","editor.selectionBackground":"#0d2040","editorLineNumber.foreground":"#2a3a4a","editorLineNumber.activeForeground":"#5a7a9a","editorIndentGuide.background":"#1a2530","editorGutter.background":"#080b10","scrollbar.shadow":"#00000000","scrollbarSlider.background":"#1d2835","scrollbarSlider.hoverBackground":"#253040"}}),monaco.editor.defineTheme("ternstudio-light",{base:"vs",inherit:!0,rules:[{token:"comment",foreground:"7a9aaa",fontStyle:"italic"},{token:"keyword",foreground:"0060bb",fontStyle:"bold"},{token:"keyword.directive",foreground:"9a5000",fontStyle:"bold"},{token:"type",foreground:"006090"},{token:"support.function",foreground:"007040"},{token:"number.trit.affirm",foreground:"007040",fontStyle:"bold"},{token:"number.trit.reject",foreground:"bb001a",fontStyle:"bold"},{token:"number.trit.hold",foreground:"9a5000",fontStyle:"bold"},{token:"number.float",foreground:"005090"},{token:"number",foreground:"005090"},{token:"string",foreground:"306820"},{token:"operator",foreground:"507090"},{token:"delimiter",foreground:"708090"},{token:"identifier",foreground:"1a2535"}],colors:{"editor.background":"#f8fafc","editor.foreground":"#1a2535","editor.lineHighlightBackground":"#eef2f6","editorCursor.foreground":"#0070cc","editor.selectionBackground":"#cce0ff","editorLineNumber.foreground":"#a0aab5","editorLineNumber.activeForeground":"#607080","editorIndentGuide.background":"#d8dde3","editorGutter.background":"#f0f2f5","scrollbar.shadow":"#00000010","scrollbarSlider.background":"#c8ced4","scrollbarSlider.hoverBackground":"#b0b8c0"}});const e=localStorage.getItem("ternstudio-theme")||"dark";Be(e),_=monaco.editor.create(document.getElementById("monaco-container"),{value:P[W]||ue.hello,language:"ternlang",theme:e==="light"?"ternstudio-light":"ternstudio-dark",fontFamily:"'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'Consolas', monospace",fontSize:13,lineHeight:20,minimap:{enabled:!1},scrollBeyondLastLine:!1,renderWhitespace:"boundary",bracketPairColorization:{enabled:!0},guides:{bracketPairs:!0},suggest:{showWords:!1},quickSuggestions:!1,padding:{top:12,bottom:12},overviewRulerLanes:0,hideCursorInOverviewRuler:!0,renderLineHighlight:"gutter",scrollbar:{verticalScrollbarSize:5,horizontalScrollbarSize:5}}),_.onDidChangeModelContent(()=>{Ce()}),_.onDidChangeCursorPosition(a=>{const s=document.getElementById("cursorPos");s&&(s.textContent=`Ln ${a.position.lineNumber}, Col ${a.position.column}`)}),window.addEventListener("resize",()=>_.layout()),_.addCommand(monaco.KeyCode.F5,()=>It()),_.addCommand(monaco.KeyMod.CtrlCmd|monaco.KeyCode.Enter,()=>It()),po();const t=localStorage.getItem("ternstudio-key")||"";t?(document.getElementById("apiKey").value=t,document.getElementById("topbarKeyInput").value=t,Ye(),Ee(),De()):window.TERNSTUDIO_DEV_KEY?(document.getElementById("apiKey").value=window.TERNSTUDIO_DEV_KEY,document.getElementById("topbarKeyInput").value=window.TERNSTUDIO_DEV_KEY,localStorage.setItem("ternstudio-key",window.TERNSTUDIO_DEV_KEY),Ye(),Ee(),De()):(st(),Ve()),se(),Je(),setInterval(Je,3e4),co();const n=JSON.parse(localStorage.getItem("ternflow_registry")||"[]"),o=[{id:"agent",slug:"agent",name:"Agent",desc:"Custom ternary pipeline",pricing:"per_call",nodes:2,deployed:"2026-04-19T00:00:00Z"},{id:"mesh-node-a",slug:"mesh-node-a",name:"Mesh_Node_A",desc:"Custom ternary pipeline",pricing:"private",nodes:8,deployed:"2026-04-19T00:00:00Z"}];let i=!1;o.forEach(a=>{n.find(s=>s.id===a.id)||(n.push(a),i=!0)}),i&&localStorage.setItem("ternflow_registry",JSON.stringify(n));const r=localStorage.getItem("ternstudio-last-view")||"dashboard";Eo(),Y(r),typeof gt=="function"?gt():window.addEventListener("load",()=>{window.mountControlBar&&window.mountControlBar()}),Ft([]),typeof lucide<"u"&&lucide.createIcons()});document.addEventListener("DOMContentLoaded",()=>{typeof lucide<"u"&&lucide.createIcons()});(function(){let t=null,n=!1;const o=[];let i=null;function r(){const m=document.createElement("div");m.id="albert-panel",m.style.cssText=`
      position: fixed;
      right: 20px; top: 80px;
      width: 380px; height: 480px;
      background: #0d0d0d;
      border: 1px solid #2a2a2a;
      border-radius: 10px;
      display: flex; flex-direction: column;
      z-index: 9500;
      box-shadow: 0 8px 32px rgba(0,0,0,0.6);
      font-family: 'JetBrains Mono', 'Fira Code', monospace;
      overflow: hidden;
      user-select: none;
    `,m.innerHTML=`
      <!-- Input port (left side) -->
      <div class="flow-port flow-port-in" id="albert-port-in"
           style="position:absolute; left:-7px; top:50%; margin-top:-6px;
                  background:#00c878; border:2px solid #00c878;
                  width:12px; height:12px; border-radius:50%; cursor:crosshair;
                  z-index:10;" title="Workflow context input"></div>

      <!-- Output port (right side) -->
      <div class="flow-port flow-port-out" id="albert-port-out"
           style="position:absolute; right:-7px; top:50%; margin-top:-6px;
                  background:#00c8ff; border:2px solid #00c8ff;
                  width:12px; height:12px; border-radius:50%; cursor:crosshair;
                  z-index:10;" title="Albert output / workflow commands"></div>

      <!-- Header -->
      <div id="albert-header" style="
        padding: 10px 14px; display: flex; align-items: center; gap: 8px;
        background: #111; border-bottom: 1px solid #222; cursor: grab; flex-shrink:0;
      ">
        <span style="color:#00dc78; font-weight:700; font-size:13px;">◆ Albert</span>
        <span style="color:#444; font-size:11px; flex:1;">co-pilot</span>
        <span id="albert-wired-badge" style="display:none; color:#00c8ff; font-size:10px;
              background:#00c8ff18; padding:2px 6px; border-radius:4px; border:1px solid #00c8ff44;">
          wired
        </span>
        <button id="albert-close" style="
          background:none; border:none; color:#444; cursor:pointer; font-size:16px;
          padding:0 4px; line-height:1;
        " title="Close (F6)">✕</button>
      </div>

      <!-- Chat area -->
      <div id="albert-chat" style="
        flex:1; overflow-y:auto; padding:12px; display:flex; flex-direction:column; gap:8px;
        scrollbar-width:thin; scrollbar-color:#222 transparent;
      ">
        <div style="color:#333; font-size:11px; text-align:center; margin-top:20px;">
          ◆ Albert is ready<br>
          <span style="color:#222; font-size:10px;">Wire a node to give him workflow context</span>
        </div>
      </div>

      <!-- Input bar -->
      <div style="padding:10px; border-top:1px solid #1a1a1a; display:flex; gap:6px; flex-shrink:0;">
        <input id="albert-input" type="text" placeholder="Prompt Albert…" style="
          flex:1; background:#1a1a1a; border:1px solid #2a2a2a; border-radius:6px;
          color:#ddd; font-family:inherit; font-size:12px; padding:7px 10px;
          outline:none;
        "/>
        <button id="albert-send" style="
          background:#00dc7822; border:1px solid #00dc7844; border-radius:6px;
          color:#00dc78; font-size:12px; padding:6px 10px; cursor:pointer; font-weight:700;
        " title="Send (Enter)">↵</button>
      </div>

      <!-- API key setup row — hidden once key is saved -->
      <div id="albert-key-row" style="
        padding:6px 10px; border-top:1px solid #1a1a1a; display:flex; gap:6px; flex-shrink:0;
        align-items:center;
      ">
        <span style="color:#555; font-size:10px; flex-shrink:0;">Gemini key</span>
        <input id="albert-key-input" type="password" placeholder="AIzaSy…" style="
          flex:1; background:#1a1a1a; border:1px solid #2a2a2a; border-radius:4px;
          color:#aaa; font-family:inherit; font-size:10px; padding:4px 7px; outline:none;
        "/>
        <button id="albert-key-save" style="
          background:#00dc7814; border:1px solid #00dc7830; border-radius:4px;
          color:#00dc78; font-size:10px; padding:4px 8px; cursor:pointer;
        ">Save</button>
      </div>

      <!-- Status strip -->
      <div id="albert-status" style="
        padding:4px 14px; font-size:10px; color:#333; background:#0a0a0a; flex-shrink:0;
      ">⠸ idle</div>

      <!-- Resize handle -->
      <div id="albert-resize" style="
        position:absolute; bottom:0; right:0; width:14px; height:14px; cursor:nwse-resize;
        background:linear-gradient(135deg, transparent 50%, #333 50%);
        border-radius:0 0 10px 0; z-index:20;
      " title="Resize"></div>
    `;let h=null;const y=m.querySelector("#albert-header");y.addEventListener("mousedown",T=>{if(T.button!==0)return;const C=m.getBoundingClientRect();h={mx:T.clientX,my:T.clientY,ex:C.left,ey:C.top},y.style.cursor="grabbing"}),document.addEventListener("mousemove",T=>{if(!h)return;const C=T.clientX-h.mx,M=T.clientY-h.my;m.style.left=h.ex+C+"px",m.style.top=h.ey+M+"px",m.style.right="auto"}),document.addEventListener("mouseup",()=>{h=null,v=null,y.style.cursor="grab"});let v=null;m.querySelector("#albert-resize").addEventListener("mousedown",T=>{T.stopPropagation(),T.preventDefault();const C=m.getBoundingClientRect();v={mx:T.clientX,my:T.clientY,w:C.width,h:C.height}}),document.addEventListener("mousemove",T=>{if(!v)return;const C=Math.max(280,v.w+(T.clientX-v.mx)),M=Math.max(220,v.h+(T.clientY-v.my));m.style.width=C+"px",m.style.height=M+"px"}),m.querySelector("#albert-close").addEventListener("click",u);const w=m.querySelector("#albert-key-row"),k=m.querySelector("#albert-key-input");function B(){localStorage.getItem("albert_gemini_key")?w.style.display="none":w.style.display="flex"}return B(),m.querySelector("#albert-key-save").addEventListener("click",()=>{const T=k.value.trim();T&&(localStorage.setItem("albert_gemini_key",T),k.value="",B(),a("⠸ key saved — ready"))}),k.addEventListener("keydown",T=>{T.key==="Enter"&&m.querySelector("#albert-key-save").click()}),m.querySelector("#albert-input").addEventListener("keydown",T=>{T.key==="Enter"&&!T.shiftKey&&(T.preventDefault(),g())}),m.querySelector("#albert-send").addEventListener("click",g),m.querySelector("#albert-port-in").addEventListener("mouseup",()=>{if(window._activeWireFromId&&window._activeWireFromId!=="albert-panel"){i=window._activeWireFromId;const T=document.getElementById("albert-wired-badge"),C=(typeof x<"u"?x:[]).find(M=>M.id===i);T&&(T.style.display="inline",T.textContent=`⟵ ${(C==null?void 0:C.name)||i}`),a(`⠸ wired to "${(C==null?void 0:C.name)||i}"`),window._activeWireFromId=null}}),m.querySelector("#albert-port-in").addEventListener("mousedown",T=>{T.stopPropagation()}),document.body.appendChild(m),m}function a(m){const h=document.getElementById("albert-status");h&&(h.textContent=m)}function s(m,h){o.push({role:m,text:h});const y=document.getElementById("albert-chat");if(!y)return;const v=m==="user",w=document.createElement("div");w.style.cssText=`
      display:flex; flex-direction:column;
      align-items:${v?"flex-end":"flex-start"};
      gap:2px;
    `,w.innerHTML=`
      <div style="
        max-width:90%; padding:7px 10px; border-radius:8px; font-size:11px; line-height:1.5;
        background:${v?"#001e12":"#111"};
        border:1px solid ${v?"#00dc7830":"#222"};
        color:${v?"#00dc78":"#ccc"};
        white-space:pre-wrap; word-break:break-word;
      ">${l(h)}</div>
      <div style="font-size:9px; color:#333; padding:0 4px;">${v?"you":"◆ albert"}</div>
    `,y.appendChild(w),y.scrollTop=y.scrollHeight}function l(m){return m.replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;")}function d(){if(!x||x.length===0)return"Canvas is empty.";const m=x.slice(0,20).map(y=>{var v;return`- ${y.type} "${y.name||y.id}"${(v=y.props)!=null&&v.code?" [has code]":""}`}).join(`
`),h=i?`
The user wired node "${i}" directly to my input port.
`:"";return`Canvas has ${x.length} node(s) and ${E.length} wire(s).
Nodes:
${m}${h}`}async function c(m){var S,T,C,M,Z,Le,Gt,Vt;const h=localStorage.getItem("albert_gemini_key")||((S=(document.getElementById("geminiApiKey")||{}).value)==null?void 0:S.trim())||"";if(!h){const ge=document.getElementById("albert-key-row");return ge&&(ge.style.display="flex",(T=document.getElementById("albert-key-input"))==null||T.focus()),"No Gemini API key set. Enter your key in the row above and press Save."}const y=m.map(ge=>({role:ge.role==="assistant"?"model":"user",parts:[{text:ge.text}]})),w={system_instruction:{parts:[{text:`You are Albert, a sovereign AI co-pilot embedded in Ternlang Studio — a visual workflow canvas.
You help users build, debug, and optimise Ternlang signal-flow workflows.
Current workspace context:
${d()}

When asked to create a node, respond with a JSON action block:
{"action":"create_node","type":"agent","name":"NodeName","x":400,"y":200}
When asked to connect nodes:
{"action":"create_wire","fromId":"nodeA","toId":"nodeB"}
When asked to validate:
{"action":"validate"}
Otherwise, respond naturally and concisely.`}]},contents:y,generationConfig:{maxOutputTokens:512,temperature:.7}},k=await fetch(`https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key=${h}`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(w)}),B=await k.json();return k.ok?((Vt=(Gt=(Le=(Z=(M=B.candidates)==null?void 0:M[0])==null?void 0:Z.content)==null?void 0:Le.parts)==null?void 0:Gt[0])==null?void 0:Vt.text)||"(empty response)":`API error: ${((C=B.error)==null?void 0:C.message)||k.status}`}function p(m){const h=/\{[^}]*"action"[^}]*\}/g;let y=m;for(const v of m.matchAll(h))try{const w=JSON.parse(v[0]);if(w.action==="create_node"&&typeof addFlowNode=="function")addFlowNode(w.type||"agent",w.x||300,w.y||200,w.name),y=y.replace(v[0],`[created node "${w.name||w.type}"]`);else if(w.action==="create_wire"&&w.fromId&&w.toId){const k=x.find(S=>S.id===w.fromId||S.name===w.fromId),B=x.find(S=>S.id===w.toId||S.name===w.toId);k&&B&&(E.push({id:`w-albert-${Date.now()}`,fromId:k.id,toId:B.id}),renderWires(),y=y.replace(v[0],`[wired "${w.fromId}" → "${w.toId}"]`))}else w.action==="validate"&&typeof runValidation=="function"&&(runValidation(),y=y.replace(v[0],"[validation triggered]"))}catch{}return y}async function g(){const m=document.getElementById("albert-input");if(!m)return;const h=m.value.trim();if(!h)return;m.value="",m.disabled=!0,s("user",h),a("⠼ Thinking…");const y=o.map(v=>({role:v.role,text:v.text}));try{const v=await c(y),w=p(v);s("assistant",w),a("⠸ idle")}catch(v){s("assistant",`Error: ${v.message}`),a("⠸ idle")}finally{m.disabled=!1,m.focus()}}function f(){t||(t=r()),t.style.display="flex",n=!0,setTimeout(()=>{var m;return(m=document.getElementById("albert-input"))==null?void 0:m.focus()},50)}function u(){t&&(t.style.display="none"),n=!1}window.toggleAlbertPanel=function(){n?u():f()}})();
