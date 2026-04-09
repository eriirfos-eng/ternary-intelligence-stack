# ternlang-moe

Ternary Mixture-of-Experts orchestrator for the [Ternlang](https://ternlang.com) ecosystem.

Routes queries through a pool of 13 domain experts, synthesises an emergent ternary signal, enforces a hard safety veto, and returns a ternary decision with confidence and temperature.

**DOI:** [10.17605/OSF.IO/TZ7DC](https://doi.org/10.17605/OSF.IO/TZ7DC)

## How it works

1. **Dual-key routing** — scores every expert pair by `relevance_a × relevance_b × synergy`. Complementary experts outperform redundant ones.
2. **Triad synthesis** — emergent field `Ek = synergy × (vi + vj) / 2`. Two orthogonal experts can produce a signal neither generates alone.
3. **Safety hard gate** — Axis-6 veto fires before any vote. Every veto is logged to `AxisMemory` for audit.
4. **Hold with tiebreaker** — a split vote or low confidence yields `trit=0`. The orchestrator invokes a tiebreaker (max 4 active experts) before committing.
5. **Three-tier memory** — Node (TTL: seconds), Cluster (routing frequency), Axis (persistent priors + veto audit log).

**13 standard experts:** Syntax · WorldKnowledge · DeductiveReason · InductiveReason · ToolUse · Persona · Safety · FactCheck · CausalReason · AmbiguityRes · MathReason · ContextMem · MetaSafety

## Usage

```rust
use ternlang_moe::TernMoeOrchestrator;

let mut orch = TernMoeOrchestrator::with_standard_experts();

// evidence scores for [syntax, world_knowledge, reasoning, tool_use, persona, safety]
let evidence = [0.6, 0.7, 0.8, 0.5, 0.4, 0.9];
let result = orch.orchestrate("Should I proceed with this action?", &evidence);

println!("trit={} conf={:.0}%", result.trit, result.confidence * 100.0);
// → trit=1 conf=84%
```

## AgentHarness

Pluggable interface for all 13 experts:

```rust
use ternlang_moe::agents::AgentHarness;

let harness  = AgentHarness::with_standard_agents();
let verdicts = harness.run("Is this safe to execute?", &evidence);
```

## License

BSL-1.1 (converts to Apache-2.0 on 2030-04-03). See [LICENSE](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-/blob/main/ternlang-root/LICENSE).
