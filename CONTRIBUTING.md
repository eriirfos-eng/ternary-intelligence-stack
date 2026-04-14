# Join the Logic Frontier: Contributing to TIS

The world is trapped in binary thinking. We are building the alternative. 

Contributing to the Ternary Intelligence Stack (TIS) isn't just about code; it's about shifting the paradigm of computation from "Forced Guessing" to "Honest Deliberation."

## The Trit-0 Mentality
Every contribution must respect the **Hold (0)** state. If your code forces a binary choice where uncertainty exists, it will be **Rejected (-1)**. We value precision over speed, and logic over scaling.

## The Styrian Loop (Development)
We build from Graz, Austria, but our logic is universal.
1. **Forge**: Use the Rust toolchain. We rely on its type-safety to enforce our ternary logic.
   ```bash
   cd ternlang-root && cargo build
   ```
2. **Deliberate**: Run the test suite. If a test returns `0` (uncertainty), investigate the edge case.
   ```bash
   cargo test --workspace
   ```
3. **Commit**: Speak in the imperative. Tell the history what you *forced* the machine to do.
   - `feat(core): unleash TSPARSE_MATMUL optimization`
   - `fix(vm): stabilize register overflow in BET-013`
   - `stdlib(math): add consensus logic for medical triage`

## The Tri-State Review Protocol
Your Pull Requests will face the same logic as our VM:
- **Affirm (+1)**: Clean code, passing tests, high signal. Merged.
- **Hold (0)**: Architectural questions or missing edge cases. We discuss until consensus.
- **Reject (-1)**: Speculative features, hallucinated logic, or binary-only shortcuts.

## Style: "Clean, Mean, and Lean"
- **No Fluff**: We don't use "TODO" or "Fixme". We define the architecture or we don't write it.
- **Sovereignty**: Ensure your code respects the EU AI Act mandates embedded in our `SECURITY.md`.
- **Formatting**: 4 spaces. Rustfmt. No trailing whitespace. Keep it surgical.

---
*By contributing, you are helping RFI-IRFOS secure European technological sovereignty. Welcome to the resistance.*
