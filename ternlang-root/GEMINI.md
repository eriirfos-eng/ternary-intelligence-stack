# GEMINI.md — Ternlang Precision Parameter Sheet
# RFI-IRFOS · ternlang.com · v1.9 (2026-04-18)

# Core Mandates
1. Trit States: +1 (affirm), 0 (hold/tend), -1 (reject).
2. v1.0.0 Stable Syntax is the ground truth.
3. Strategic Goal: AWS Lambda + App Store for decision systems.

---

## Verified Syntax Idioms (v1.0.0+)

| Feature | Verified Pattern | Note |
|---------|------------------|------|
| **Casting** | `let x: float = cast(val_int);` | `cast(expr)` only. No type in parens. |
| **Math** | `let x: float = abs(neg_float);` | Global built-ins: `abs`, `pow`, `min`, `max`, `sqrt`. |
| **Arrays (Fixed)** | `let a: int[5];` or `let a: trittensor<5>;` | `int[N]` and `float[N]` are stable for fixed sizes. |
| **Array (Dynamic)** | `fn f(a: trit[])` | `type[]` works in parameters/signatures. |
| **Hold Keyword** | `let b: trit = hold;` | `hold` is a keyword alias for `tend` (0). |
| **Built-ins** | `len(string)`, `invert(trit)` | Fully functional in v1.0.0. No stack overflow. |
| **Struct Init** | `let p: Point = Point {x: 1, y: 2};` | Must include type name before brace. |
| **Match** | `match x { _ => { ... } }` | Wildcard `_` supported. Multi-val `1, 2 =>` PENDING. |
| **Logic** | `a && b`, `a || b` | Uses standard double-ampersand/pipe tokens. |

---

## Strategic Roadmap: "Deploy as Product"

The "Killer Feature" for TIS is the total removal of friction between **Logic Creation** and **Monetization**.

### Vision: The Decision App Store
TIS is not a library; it is a deployment target. Developers write Decision Systems in Ternary, and with one click/command, they are live products.

### Proposed CLI/IDE Workflow:
`ternlang deploy --product`

1. **Prompts:** 
   - Product Name
   - Description
   - Pricing Tier (Free / Monthly / Per-Call)
2. **Action:**
   - Packages `.tern` logic into a high-performance BET container.
   - Deploys to `ternlang-api` infrastructure (Fly.io).
   - Generates a unique API Endpoint + Sandbox UI.
   - Automatically lists in the `ternlang.com` Marketplace.

---


---

## Ternlang Studio (TernFlow / Flow Lab)

**File Location:** `ternlang-root/ternlang-studio/index.html`
**Deployment:** Served dynamically by `ternlang-api` (`include_str!`). Modifying the studio requires rebuilding and deploying the API (`cd ternlang-root && fly deploy`).

**Core Architectural Rules (Semantic Isolation):**
- **Deterministic Nodes:** Execute verified `.tern` code. They are purely signal-native. They MUST NOT expose probabilistic fields (temperature, tokens, prompts). UI focuses on "Intent" and "I/O Contract".
- **LLM Bridge Nodes (`type: external`):** Act as adapters. They encapsulate ALL probabilistic controls (Provider, API keys, Prompts, Temperature, Tokens). Raw LLM text NEVER enters the graph; it MUST be mapped to a structured Ternary Signal (Classification, Score, JSON) with a fallback to `0` (uncertainty).
- **Edge Logic (Reasoning Routes):** Wires route based on ternary conditions (`affirm`, `tend`, `reject`, `!reject`, `!tend`, `all`). Failure behaviors explicitly drop, route to fallback, or convert to uncertainty.

**UI/UX & Modding Guidelines:**
- **Strangler Pattern:** Modularization and UI refactors must happen safely. Experimental UI features (like the Semantic Property Panels) are gated behind feature flags (e.g., `ENABLE_NEW_PROPERTIES_UI = false`) to protect the live deployment.
- **Canvas Interaction:** Ports use expanded `::after` hitboxes (`z-index: 20`) for snapping. The `active-wire` uses `pointer-events: none` to prevent blocking `mouseup` events.
- **Library & Archetypes:** Built-in agents have fixed colors/icons. Custom agents are dynamically fetched from the GitHub API. The "KMU Archetype Pack" provides 5 pre-wired Austrian SME workflows (Process Opt, Supplier Scoring, Customer Qual, Invoice Fraud, Hiring Decision).


## Unresolved Issues (Triage 2026-04-18)

- **[VM-STRUCT-001]** Field access on returned structs causes stack underflow.
  - *Workaround:* Assign to local variable first if possible, but deep nesting is currently unstable.
- **[MOD-004]** Path resolution in named imports.
  - *Workaround:* Use relative paths `from "../dir/file.tern"`.
- **[COMP-OP-001]** `@sparseskip` annotation is correctly parsed but opcode emission is pending.
- **[BET-013]** Named imports with unimported transitive dependencies cause stack overflow.
  - *Workaround:* Use `import *` for complex dependency chains.
