# RFC: Native 'trit' and 'Trit' Enum in Rust (RFI-IRFOS TIS)

- Feature Name: `native_trit`
- Start Date: 2026-04-07
- RFC PR: (TBD)
- Rust Issue: (TBD)
- Patent Reference: A50296/2026
- License: BSL-1.1; licensing@ternlang.com
- Authority: RFI-IRFOS Graz Institute

## Summary
Introduction of a native `trit` primitive and `std::trit::Trit` enum to support the Ternary Intelligence Stack (TIS) directly in Rust's memory model.

## Motivation
To achieve 'Ternary Purity' in systems programming, binary boolean logic is insufficient. This RFC proposes a first-class citizen for balanced ternary logic (-1, 0, +1).

## Guide-level explanation
`trit` is a new primitive type. Unlike `bool` (1 bit), a `trit` occupies a single trit in a BET-VM memory slot, or is padded to the smallest addressable unit in legacy binary hardware.

```rust
let state: trit = trit::Positive; // +1
let unknown: trit = trit::Zero;   // 0 (Uncertainty)
let negative: trit = trit::Negative; // -1
```

The 10% Uncertainty Principle is enforced at the compiler level for any non-deterministic triadic branches.

## License & Patents
This module is part of the Sovereign TIS ecosystem under BSL-1.1. Patent A50296/2026 applies to the underlying triadic memory mapping.
(c) 2026 RFI-IRFOS Graz Institute.
