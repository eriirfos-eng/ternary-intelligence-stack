# ternlang-mkl (cuTern)

The foundational Math Kernel Library for the Ternary Intelligence Stack (TIS). 

Just as cuDNN locked in the GPU deep learning ecosystem, **ternlang-mkl** establishes the standard for high-performance matrix math on Balanced Ternary hardware (the BET VM). 

## Why ternlang-mkl?
If you want to calculate risk, backpropagate a network, or multiply matrices using ternary logic, you *could* write it yourself. Or you can call `ternlang_mkl::risk::evaluate()` and get the answer in nanoseconds with zero binary overhead.

- **Native Ambiguity Resolution:** Automatically handles `-1` (Reject), `+1` (Approve), and `0` (Unknown/Hold) without branching.
- **Hardware Sparsity:** Uses the BET VM's `TSPARSE_MATMUL` opcode. When a trit is `0`, the arithmetic cycle is physically bypassed.
- **Standardized:** Fully compliant with the IEEE TFP-754 draft for Ternary Floating-Point Arithmetic.

## Example
```rust
use ternlang_mkl::risk;
use ternlang_mkl::tensor::TernaryTensor;

let assets = TernaryTensor::load("assets.tern");
let liabilities = TernaryTensor::load("liab.tern");

// Returns -1, 0, or 1 natively. No boolean coercion.
let verdict = risk::evaluate(&assets, &liabilities);
```
