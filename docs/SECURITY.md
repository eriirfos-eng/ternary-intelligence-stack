# Security Policy

## Overview

Security within the Ternary Intelligence Stack (TIS) is treated as a system-level requirement, not an afterthought. The goal is to ensure integrity, reproducibility, and robustness across all components of the stack.

This policy defines how vulnerabilities are reported, evaluated, and resolved.

---

## Reporting a Vulnerability

If you discover a security issue, please report it responsibly:

* **Email:** [rfi.irfos@gmail.com](mailto:rfi.irfos@gmail.com)
* **Subject:** `[SECURITY] <short description>`

Include:

* A clear description of the issue
* Steps to reproduce
* Affected components (crate/module)
* Potential impact assessment (if known)

Do **not** open public issues for security vulnerabilities.

---

## Response Process

We aim to follow this timeline:

* **Acknowledgement:** within 48 hours
* **Initial assessment:** within 7 days
* **Fix or mitigation:** within 30 days (depending on severity and complexity)

If additional time is required, we will communicate transparently.

---

## Supported Versions

| Version        | Support Status |
| -------------- | -------------- |
| Latest release | Supported      |
| Older versions | Not supported  |

Security fixes are only guaranteed for the most recent stable version.

---

## Security Principles

### 1. Deterministic & Auditable Systems

All critical components should be reproducible and traceable. Hidden or non-deterministic behavior is treated as a risk.

### 2. Memory Safety by Default

The project relies on Rust’s safety guarantees:

* Unsafe code must be minimized and justified
* All unsafe blocks require explicit review and documentation

### 3. Minimal Attack Surface

* Avoid unnecessary dependencies
* Keep interfaces explicit and constrained
* Prefer static analysis and compile-time guarantees over runtime checks

### 4. Robustness Against Malformed Input

* All external inputs must be validated
* Parsers and loaders must fail safely (no silent corruption)

### 5. Transparency & Inspectability

* System behavior should be observable and debuggable
* Logs and artifacts should support post-hoc verification

---

## Disclosure Policy

* We follow responsible disclosure practices
* Reported vulnerabilities are not made public until a fix or mitigation is available
* Contributors who report valid issues may be acknowledged in release notes (upon request)

---

## Out of Scope

* Issues in unsupported versions
* Hypothetical vulnerabilities without reproducible evidence
* Non-security bugs (use issue tracker instead)

---

## Notes

This policy will evolve as the system matures. Contributions and improvements are welcome.
