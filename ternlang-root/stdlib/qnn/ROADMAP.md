# stdlib/qnn/ — Qutrit Neural Network Standard Library

Reference implementations for Qutrit Neural Networks (QNN) as described in the RFI-IRFOS whitepaper (DOI: 10.17605/OSF.IO/TZ7DC) and the Kepp 2026 QNN paper (DOI: 10.17605/OSF.IO/X96HS).

## Status: Planned

Example QNN programs (examples 251–265) are currently in `examples/` root.  
This directory will house the stdlib modules for production use.

## Planned modules

| File | Concept | Tier |
|------|---------|------|
| `qutrit_gate.tern` | Single-qutrit rotation gate | 3 |
| `qutrit_hadamard.tern` | Ternary Hadamard transform | 3 |
| `qutrit_entangle.tern` | Two-qutrit entanglement protocol | 3 |
| `qnn_layer.tern` | Qutrit linear layer (forward pass) | 3 |
| `qnn_measure.tern` | Measurement collapse → trit | 3 |
| `qnn_inference.tern` | Full QNN inference pipeline | 3 |
| `qutrit_teleport.tern` | Ternary teleportation protocol | 3 |
| `qnn_grover.tern` | Ternary Grover search | 3 |
| `qnn_vqe.tern` | Variational Quantum Eigensolver (ternary) | 3 |
| `qnn_qaoa.tern` | QAOA for ternary combinatorics | 3 |

## See also

- `../../examples/251_qutrit_*` through `265_*` — working QNN examples
- `../nn/` — classical ternary neural network stdlib
- `spec/standards/T-QUT-v1.0.md` — Qutrit Bridge Standard
