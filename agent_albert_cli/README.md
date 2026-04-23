# 🧠 ALBERT  
**Autonomous Local Brain & Execution Runtime**

> Offline-first. Self-extending. Token-efficient by design.

The era of bleeding tokens into bloated cloud APIs is over.

**ALBERT** is a sovereign, local AI runtime engineered by RFI-IRFOS. Built on the **Ternary Intelligence Stack (TIS)**, it doesn’t just execute commands — it **plans, reasons, adapts, and evolves its own capabilities**.

If your current agent needs 50,000 tokens to read a repo, that’s not scaling — that’s architectural debt.  
**ALBERT fixes this at the root.**

---

## ⚡ Key Capabilities

### 🧩 Native Context Compression *(“Token Killer”)*
- Sliding-window deduplication embedded directly into the Rust I/O pipeline  
- Eliminates redundant filesystem noise before model ingestion  
- Converts massive outputs into compact semantic pointers  
- Zero reprocessing when nothing changes  

---

### 🛠️ Auto-Skill Generation
- Dynamically writes its own tools when capabilities are missing  
- Supports **Rust + Python** skill generation  
- Compiles and mounts tools via MCP (Model Context Protocol)  
- Fully recursive feedback loop  

---

### 🧠 Hybrid Reasoning Engine
**ReAct + Ultraplan dual system:**
- **ReAct (+1 / 0 / -1):** Fast iterative reasoning loop  
- **/ultraplan:** Deep multi-step execution planning with:
  - Risk vectors  
  - Rollback strategies  
  - Structural awareness  

---

### 🌐 Deep Context Slurping
ALBERT builds a live world model from:
- Git diffs  
- `.albert/ALBERT.md` directives  
- XDG configuration layers  

No blind execution. Full situational awareness.

---

### ⚖️ Ternary Execution State
Every action resolves to:
- **+1 → Success**
- **0 → Insufficient context (halts + adapts)**
- **-1 → Failure (triggers reflection loop)**

No hallucinated actions. No blind retries.

---

## 🏛️ Architecture Overview

- **Ternary Intelligence Stack (TIS)**
- **BET VM (Binary-Encoded Ternary Virtual Machine)**
- **MoE-13 Orchestrator (Mixture-of-Experts)**  
- **MCP Tooling Layer**
- **Rust-native runtime core**

---

## ⚙️ Installation


```bash
cargo binstall albert-cli
