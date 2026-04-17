# Ternlang Verified Patterns & Idioms
# RFI-IRFOS · Ground Truth for v0.3.3+

This file contains verified implementation patterns. Use these to avoid common parser errors and VM type mismatches.

---

## 1. 2D Tensor Initialization
**Constraint:** Nested brackets `[[n,n],[n,n]]` are not supported in literals.
**Pattern:** Declare with dimensions, initialize with a flat array.

```tern
// Correct
let m: trittensor<2 x 2> = [1, 0, -1, 1];

// Usage (Flat Indexing)
let val = m[1]; // Accesses (0, 1) in 2x2
```

---

## 2. Consensus Truth Table
**Behavior:** Verified empirical results for `consensus(a, b)`.

| A | B | Result |
|---|---|---|
| affirm (1) | tend (0) | **affirm (1)** |
| affirm (1) | reject (-1) | **tend (0)** |
| reject (-1) | tend (0) | **reject (-1)** |
| tend (0) | tend (0) | **tend (0)** |

---

## 3. Trit Saturation
**Constraint:** Scalar assignments do not automatically saturate. Tensors do.
**Pattern:** Use a temporary tensor or explicit `match` for safety when converting `int` to `trit`.

```tern
// Safe Saturation Pattern
let t: trittensor<1> = [0];
t[0] = 5; // Automatically becomes affirm (1)
let safe_trit: trit = t[0];
```

---

## 4. String Handling
**Constraint:** `len(string)` triggers `BET-007`.
**Pattern:** Use empty string comparison for presence checks.

```tern
// Correct
if s == "" { return reject; }
```

---

## 5. Control Flow
**Status:** `else if` and ternary `while` are fully stable.
**Pattern:** 

```tern
// Verified stable
if a > b { ... } else if a < b { ... } else { ... }

while cond {
    // positive path
} else {
    // zero path
} else {
    // negative path
}
```

---

## 6. Return in Match
**Status:** Verified stable.
**Pattern:**

```tern
match x {
    -1 => { return reject; }
     0 => { return tend; }
     1 => { return affirm; }
}
```
