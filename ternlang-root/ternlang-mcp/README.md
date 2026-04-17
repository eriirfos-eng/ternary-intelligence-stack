# Ternary Intelligence Stack — MCP Server

**Add a third decision state to any AI agent.**

Every AI today is forced to answer yes or no — even when the evidence is contradictory, incomplete, or genuinely uncertain. Ternlang adds **hold** (trit = 0): not indecision, but a first-class routing instruction that tells the agent to gather more data before committing.

- **Website:** https://ternlang.com  
- **Publisher:** RFI-IRFOS (ZVR: 1015608684) · Graz, Austria  
- **License:** BSL-1.1  
- **EU AI Act:** Articles 13/14/15 compliant design  

---

## Tools

All 19 tools are available on the community tier — no API key required.

### Core Trit Primitives

| Tool | Description |
|------|-------------|
| `trit_decide` | Convert float evidence signals → ternary decision (reject/hold/affirm) with confidence score |
| `trit_vector` | Multi-dimensional weighted evidence aggregation across named dimensions |
| `trit_consensus` | Balanced ternary addition of two trit signals (truth+conflict=hold) |
| `trit_action_gate` | Safety-veto gate: hard-block dimensions veto unconditionally; others vote |

### BET VM Execution

| Tool | Description |
|------|-------------|
| `trit_eval` | Evaluate a ternary expression on the live BET (Balanced Ternary Execution) VM |
| `ternlang_run` | Compile and run a full `.tern` program on the BET VM |
| `trit_translate` | Translate Python/SQL/JSON rules → `.tern` with the hold zone made explicit |

### MoE-13 Orchestration

| Tool | Description |
|------|-------------|
| `moe_orchestrate` | Route a query through 13 domain experts with dual-key synergistic routing |
| `moe_deliberate` | EMA-based iterative convergence — accumulates evidence until confident |
| `trit_debate` | Two competing claims → structured 3-way verdict (AGREEMENT / CONFLICT / HOLD) |
| `trit_calibrate` | Detect binary habituation in an AI agent's decision log |
| `trit_uncertainty_map` | Annotate text sentence-by-sentence with ternary confidence signals |

### EcoCore + Audit

| Tool | Description |
|------|-------------|
| `trit_eco_check` | Dual human + ecocentric perspective on a proposed action |
| `trit_audit` | EU AI Act compliance report (Articles 13/14) on a decision log |
| `audit_ternary_logic` | Triadic code compliance check — detect binary habituation in code |
| `get_industrial_standards` | List current RFI-IRFOS triadic industrial standards |

### ML Primitives

| Tool | Description |
|------|-------------|
| `quantize_weights` | Float weights → balanced ternary via BitNet thresholding |
| `sparse_benchmark` | Ternary matmul efficiency benchmark (up to 122x at 99%+ sparsity) |
| `tsql_join` | T-SQL Triadic Join — partial matches routed to Deliberative Hold |

---

## Installation

### Option 1 — HTTP (no install, recommended)

Point your MCP client at the live endpoint:

```json
{
  "mcpServers": {
    "ternlang": {
      "url": "https://ternlang.com/mcp"
    }
  }
}
```

### Option 2 — Smithery CLI

```bash
npx @smithery/cli install rfi-irfos/ternlang --client claude
```

### Option 3 — Local via cargo

```bash
cargo install ternlang-mcp
```

Then add to your MCP config:

```json
{
  "mcpServers": {
    "ternlang": {
      "command": "ternlang-mcp"
    }
  }
}
```

### API Key (optional)

Community tier tools are fully available without a key. Pro, Industrial, and Enterprise tiers unlock premium memory and analytics tools. Obtain a key at https://ternlang.com/activate and pass it as `X-Ternlang-Key` header or via the `apiKey` config field.

---

## Usage

### Basic ternary decision

```json
{
  "tool": "trit_decide",
  "arguments": {
    "evidence": [0.8, -0.2, 0.6, 0.1],
    "min_confidence": 0.7
  }
}
```

**Response:**
```json
{
  "scalar": 0.325,
  "trit": 1,
  "zone": "affirm",
  "confidence": 0.73,
  "is_actionable": true
}
```

### Multi-dimensional reasoning

```json
{
  "tool": "trit_vector",
  "arguments": {
    "dimensions": [
      { "label": "safety_check",   "value": 0.9,  "weight": 2.0 },
      { "label": "user_consent",   "value": 1.0,  "weight": 1.5 },
      { "label": "cost_benefit",   "value": -0.3, "weight": 1.0 },
      { "label": "legal_review",   "value": 0.1,  "weight": 1.0 }
    ],
    "min_confidence": 0.6
  }
}
```

### Full MoE-13 orchestration

```json
{
  "tool": "moe_orchestrate",
  "arguments": {
    "query": "Should we deploy this model to production without further safety testing?"
  }
}
```

### Detect binary habituation in your AI pipeline

```json
{
  "tool": "trit_calibrate",
  "arguments": {
    "decisions": [
      { "input": "Is this safe?",   "output": "yes", "confidence": 0.51 },
      { "input": "Should we act?",  "output": "yes", "confidence": 0.53 },
      { "input": "Is data clean?",  "output": "no",  "confidence": 0.55 }
    ]
  }
}
```

---

## The Hold State

> **trit = 0 is not an error.** It is an active routing instruction: *gather more evidence before committing.*

When a tool returns `"zone": "tend"` or `"trit": 0`, the correct response is to surface the uncertainty and ask the user for the missing information — not to collapse it into a binary guess.

This is the core innovation of the Ternary Intelligence Stack: epistemic humility as a first-class computational primitive.

---

## Links

- **Docs:** https://ternlang.com/docs  
- **Activate Pro:** https://ternlang.com/activate  
- **GitHub:** https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-  
- **Smithery:** https://smithery.ai/server/rfi-irfos/ternlang  
