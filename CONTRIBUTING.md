# Contributing to the Ternary Intelligence Stack (TIS)

## Overview

Thank you for your interest in contributing to TIS. This project focuses on building deterministic, auditable, and reproducible systems for ternary-native computation.

Contributions are welcome across:

* Core logic and runtime
* Compiler and language tooling (Ternlang)
* Model and training infrastructure
* Documentation and testing

---

## Contributing to albert. Training (no GPU needed)

The simplest way to contribute is to donate CPU time to albert.'s training. No ML background, no GPU, no configuration — just two commands:

```bash
git clone https://github.com/eriirfos-eng/albert-spores ~/projects/albert-spores
bash ~/projects/albert-spores/install.sh
```

Then run `albert-train` in a fresh terminal. Your machine trains for as long as you like and packages each epoch as a *spore* — a checkpoint fragment that gets blended into the live model. See **[github.com/eriirfos-eng/albert-spores](https://github.com/eriirfos-eng/albert-spores)** for full details.

Watch albert. in real time at **[ternlang.com/talk](https://ternlang.com/talk)**.

---

## Development Setup

### Requirements

* Rust (latest stable)
* Cargo

### Build

```bash
cargo build --workspace
```

### Test

```bash
cargo test --workspace
```

All tests must pass before submitting a pull request.

---

## Contribution Workflow

1. **Fork the repository**

2. **Create a feature branch**

   ```bash
   git checkout -b feat/your-feature-name
   ```

3. **Implement your changes**

4. **Run tests and format code**

   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --workspace
   ```

5. **Commit using clear, imperative messages**

   * `feat(core): add sparse matmul kernel`
   * `fix(vm): resolve register overflow edge case`
   * `docs(api): clarify tokenizer behavior`

6. **Open a Pull Request**

---

## Code Guidelines

### 1. Determinism First

* Avoid hidden state or non-reproducible behavior
* Ensure outputs are consistent under identical inputs

### 2. Explicitness Over Assumption

* Handle edge cases explicitly
* Do not rely on implicit defaults where correctness matters

### 3. Safety

* Prefer safe Rust
* Any `unsafe` code must be:

  * strictly necessary
  * documented
  * reviewed carefully

### 4. Minimalism

* Avoid unnecessary abstractions
* Keep modules focused and composable

---

## Pull Request Review

All pull requests are reviewed with the following criteria:

* **Correctness**: Does the code behave as intended?
* **Clarity**: Is the logic understandable and well-structured?
* **Test Coverage**: Are edge cases covered?
* **Impact**: Does this improve the system meaningfully?

PR outcomes:

* **Approved** → merged
* **Changes requested** → iterate and resubmit
* **Rejected** → not aligned with project scope or quality standards

---

## What to Avoid

* Incomplete features without clear scope
* Unverified assumptions or “placeholder logic”
* Large, unfocused pull requests
* Breaking changes without justification

---

## Documentation

* Update relevant documentation for any functional change
* Keep explanations concise and technical
* Avoid speculative or marketing-style language

---

## Community & Conduct

* Be respectful and constructive
* Focus discussions on technical merit
* Disagreements are resolved through evidence and reasoning

---

## Notes

This project prioritizes correctness, reproducibility, and long-term maintainability over speed of iteration.

If you’re unsure about a change, open a discussion before implementing it.
