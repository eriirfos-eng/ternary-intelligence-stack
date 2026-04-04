# Ternlang Language Reference

**RFI-IRFOS Ternary Intelligence Stack — v0.1.2**

This document is the complete language reference for Ternlang: every keyword, type, operator, built-in function, stdlib module, and diagnostic code, with examples. It is derived directly from the compiler source (`ternlang-core`).

---

## Table of Contents

1. [The Trit — Core Concept](#1-the-trit--core-concept)
2. [Lexical Rules](#2-lexical-rules)
3. [Types](#3-types)
4. [Literals](#4-literals)
5. [Operators](#5-operators)
6. [Expressions](#6-expressions)
7. [Statements](#7-statements)
8. [Functions](#8-functions)
9. [Structs](#9-structs)
10. [Agents (Actor Model)](#10-agents-actor-model)
11. [Modules and `use`](#11-modules-and-use)
12. [Built-in Functions](#12-built-in-functions)
13. [Standard Library](#13-standard-library)
14. [Directives](#14-directives)
15. [Grammar (EBNF)](#15-grammar-ebnf)
16. [Diagnostic Codes](#16-diagnostic-codes)

---

## 1. The Trit — Core Concept

Ternlang's primitive is the **trit** — a ternary digit with exactly three states:

| Value | Symbol   | Meaning                              |
|-------|----------|--------------------------------------|
| `1`   | Truth    | Confirmed, positive, active          |
| `0`   | Hold     | Uncertain, neutral, suspended        |
| `-1`  | Conflict | Negated, negative, failed            |

All control flow branches on these three states. A `bool` maps to `{-1, 1}` but loses the crucial third state. Ternlang keeps it.

**Balanced ternary arithmetic** uses base-3 with digits `{-1, 0, 1}`. Addition produces a (sum, carry) pair; multiplication is sign-preserving.

---

## 2. Lexical Rules

### Comments

```ternlang
// This is a line comment. Block comments are not supported.
```

### Whitespace

Spaces, tabs, and newlines are insignificant between tokens.

### Identifiers

```
[a-zA-Z_][a-zA-Z0-9_]*
```

Keywords take priority over identifiers (priority 3 in the lexer).

### String Literals

Double-quoted, no escape sequences in v0.1.2:

```ternlang
"hello world"
```

### Integer Literals

```
[0-9]+   // decimal non-negative integer
```

### Trit Literals

Exactly `-1`, `0`, or `1`. These are distinct tokens, not integer expressions.

---

## 3. Types

### Primitive Types

| Keyword          | Description                                    | Example                         |
|------------------|------------------------------------------------|---------------------------------|
| `trit`           | Ternary value: -1, 0, or +1                   | `let x: trit = 1;`             |
| `int`            | 64-bit signed integer                          | `let n: int = 42;`             |
| `float`          | Floating-point number                          | `let f: float;`                |
| `bool`           | Boolean (binary; prefer trit)                 | `let b: bool;`                 |
| `string`         | UTF-8 string                                   | `let s: string = "ok";`        |
| `agentref`       | Handle to a running agent instance             | `let v: agentref = spawn Voter;` |

### Tensor Types

```ternlang
trittensor<N x M>       // 2D tensor, N rows, M columns
trittensor<N>           // 1D tensor (single dimension)
trittensor<N x M x K>   // 3D tensor (any number of dims, separated by `x`)
```

All elements are trits. The dimension syntax uses the identifier `x` as a separator.

```ternlang
let weights: trittensor<128 x 256>;
let bias:    trittensor<128>;
```

### Named (Struct) Types

Any defined struct name is a valid type:

```ternlang
struct Signal { value: trit, weight: trit }

fn process(s: Signal) -> trit {
    return s.value;
}
```

### Type Coercion

Use `cast(expr)` to coerce between compatible types. The target type is inferred from the binding:

```ternlang
let flag: bool = true;
let t: trit = cast(flag);
```

---

## 4. Literals

| Literal        | Type        | Example                     |
|----------------|-------------|-----------------------------|
| `-1`           | `trit`      | `let c: trit = -1;`        |
| `0`            | `trit`      | `let h: trit = 0;`         |
| `1`            | `trit`      | `let t: trit = 1;`         |
| `42`           | `int`       | `let n: int = 42;`         |
| `"text"`       | `string`    | `let s: string = "hi";`    |

> **Note:** `-1` as a trit literal is a single token. `-42` for integers is the unary negation operator applied to `42`.

---

## 5. Operators

### Arithmetic Operators

| Operator | Description            | Types            | Example           |
|----------|------------------------|------------------|-------------------|
| `+`      | Addition               | `int`, `trit`    | `a + b`           |
| `-`      | Subtraction / Negation | `int`, `trit`    | `a - b`, `-x`     |
| `*`      | Multiplication         | `int`, `trit`    | `a * b`           |

For trits, `+` and `*` follow balanced ternary arithmetic. To negate a trit variable use the `invert()` built-in or the unary `-` operator.

### Comparison Operators

| Operator | Description    | Example    |
|----------|----------------|------------|
| `==`     | Equal          | `a == b`   |
| `!=`     | Not equal      | `a != b`   |

### Logical Operators

| Operator | Description | Example      |
|----------|-------------|--------------|
| `&&`     | Logical AND | `a && b`     |
| `\|\|`   | Logical OR  | `a \|\| b`   |

### Assignment

| Operator | Description               | Example              |
|----------|---------------------------|----------------------|
| `=`      | Field assignment           | `obj.field = value;` |

Variable bindings use `let` (see Statements). There is no standalone variable reassignment — use struct field mutation.

### Other Operators / Tokens

| Token | Description                                              |
|-------|----------------------------------------------------------|
| `->`  | Function return type separator                           |
| `=>`  | Match arm separator                                      |
| `.`   | Field access                                             |
| `?`   | Ternary branch (after `if`/`while`) or conflict propagation (postfix on expression) |
| `::`  | Module path separator                                    |
| `:`   | Type annotation separator                                |
| `;`   | Statement terminator                                     |
| `@`   | Directive prefix                                         |

### Operator Precedence (lowest to highest)

| Precedence | Operators          |
|------------|--------------------|
| 0 (lowest) | `\|\|`             |
| 1          | `&&`               |
| 2          | `==`, `!=`         |
| 3          | `+`, `-`           |
| 4 (highest)| `*`                |

Parentheses `( )` override precedence as usual.

---

## 6. Expressions

### Trit Literal

```ternlang
-1    // conflict
0     // hold
1     // truth
```

### Integer / String Literals

```ternlang
42
"hello"
```

### Identifier

```ternlang
x
my_var
```

### Binary Operation

```ternlang
a + b
x * y
signal == 1
```

### Unary Negation

```ternlang
-x        // negates int or trit
```

For trit variables `-x` is equivalent to `invert(x)`.

### Function Call

```ternlang
consensus(a, b)
invert(x)
matmul(W, input)
my_fn(arg1, arg2)
```

### Field Access

```ternlang
signal.value
node.weight
```

Chains are supported: `a.b.c`

### `cast(expr)` — Type Coercion

```ternlang
let t: trit = cast(some_int);
let n: int  = cast(some_trit);
```

The target type is determined by the surrounding `let` binding. The semantic checker treats `cast` as always type-compatible.

### `spawn` — Create Agent Instance

```ternlang
let v: agentref = spawn Voter;                        // local agent
let r: agentref = spawn remote "10.0.0.1:7373" Voter; // remote agent
```

Evaluates to an `agentref`.

### `await` — Receive from Agent

```ternlang
let reply: trit = await v;
```

Blocks until the agent's mailbox delivers a message. Returns `trit`.

### `nodeid` — Current Node Address

```ternlang
let addr: string = nodeid;
```

Returns the address of the current Ternlang node as a `string`.

### Conflict Propagation `expr?`

```ternlang
let result: trit = risky_fn()?;
```

If `risky_fn()` returns `-1` (conflict), the current function immediately returns `-1`. Otherwise execution continues with the value. Only valid on trit-returning expressions.

This is Ternlang's equivalent of Rust's `?` operator, adapted for ternary error signaling.

```ternlang
fn pipeline(x: trit) -> trit {
    let a: trit = step_one(x)?;   // short-circuits on conflict
    let b: trit = step_two(a)?;
    return b;
}
```

---

## 7. Statements

### Variable Binding — `let`

```ternlang
let name: type = expr;
let mut name: type = expr;   // mut is parsed but currently advisory
let name: type;              // defaults to 0 (hold)
```

Examples:

```ternlang
let x: trit = 1;
let n: int = 42;
let weights: trittensor<128 x 256>;
let v: agentref = spawn Voter;
```

### Ternary Conditional — `if ? else else`

Unlike binary `if/else`, Ternlang's conditional has **three branches** — one for each trit state. All three are required.

```ternlang
if condition ? {
    // executed when condition == 1 (truth)
} else {
    // executed when condition == 0 (hold)
} else {
    // executed when condition == -1 (conflict)
}
```

The condition must be of type `trit`. The `?` token separates the condition from the first branch.

```ternlang
fn classify(signal: trit) -> trit {
    if signal ? {
        return 1;    // confirmed
    } else {
        return 0;    // uncertain
    } else {
        return -1;   // rejected
    }
}
```

### Exhaustive Pattern Match — `match`

Matches a trit value against all three states. **All three arms are required** — the compiler enforces exhaustiveness.

```ternlang
match expr {
    1  => stmt
    0  => stmt
    -1 => stmt
}
```

Arms can be blocks:

```ternlang
match vote {
    1  => { return 1; }
    0  => { return 0; }
    -1 => { return -1; }
}
```

Missing any arm produces `[PARSE-004] Non-exhaustive match`.

### Ternary While — `while ? else else`

A while loop with three branches per iteration:

```ternlang
while condition ? {
    // body when condition == 1 (truth): continue iterating
} else {
    // body when condition == 0 (hold): pause/idle branch
} else {
    // body when condition == -1 (conflict): error/exit branch
}
```

### For-In Loop — `for in`

Iterates over a collection or tensor:

```ternlang
for item in weights {
    // item: trit (each element of the collection)
}
```

The loop variable is typed as `trit` within the body.

### Infinite Loop — `loop`

```ternlang
loop {
    // runs forever until break
    if done ? { break; } else { } else { break; }
}
```

### Break and Continue

```ternlang
break;      // exit the enclosing loop
continue;   // skip to next iteration
```

Both require a semicolon.

### Return Statement

```ternlang
return expr;
```

Must match the function's declared return type. The compiler checks this statically (`[FN-002]`).

### Send Message — `send`

Send a message to an agent's mailbox:

```ternlang
send v 1;           // send trit 1 to agent referenced by v
send worker signal; // send variable signal to worker
```

No semicolon after the message — the semicolon terminates the send statement.

### Field Assignment

```ternlang
obj.field = value;
```

Requires `obj` to be a struct variable. The field type must match `value`'s type (`[TYPE-001]` otherwise).

### Use / Import

```ternlang
use std::trit;
use std::math;
use ml::inference;
use mymod::utils;   // user-defined module: looks for mymod/utils.tern
```

Must end with `;`. Can appear inside function bodies — the stdlib loader injects the module's functions into the program scope.

### Block

A sequence of statements enclosed in braces:

```ternlang
{
    let x: trit = 1;
    return x;
}
```

Blocks introduce a new lexical scope.

### Decorated Statement — `@directive`

Apply a compiler directive to a statement (see [Directives](#14-directives)):

```ternlang
@sparseskip let result: trittensor<128 x 256> = matmul(W, x);
```

---

## 8. Functions

### Declaration

```ternlang
fn name(param1: type1, param2: type2) -> return_type {
    // body
    return expr;
}
```

Functions are top-level declarations (alongside structs and agents). Forward references are allowed — the semantic checker registers all function signatures before checking bodies.

### Examples

```ternlang
fn invert(x: trit) -> trit {
    match x {
         1 => { return -1; }
         0 => { return  0; }
        -1 => { return  1; }
    }
}

fn dot(a: trit, b: trit) -> trit {
    return consensus(a, b);
}

fn run(W: trittensor<128 x 256>, x: trittensor<256>) -> trittensor<128> {
    @sparseskip let out: trittensor<128 x 256> = matmul(W, x);
    return out;
}
```

### No Closures or Lambdas

v0.1.2 has no anonymous functions or closures. All callable units are top-level `fn` declarations or agent methods.

---

## 9. Structs

Named, typed record types. Fields can be any type, including other structs and tensors.

### Definition

```ternlang
struct Name {
    field1: type1,
    field2: type2
}
```

Trailing comma is optional on the last field.

### Usage

```ternlang
struct Neuron {
    signal: trit,
    weight: trit
}

fn fire(n: Neuron) -> trit {
    return consensus(n.signal, n.weight);
}
```

### Field Assignment (mutation)

```ternlang
fn update(n: Neuron, new_sig: trit) -> trit {
    n.signal = new_sig;
    return n.signal;
}
```

---

## 10. Agents (Actor Model)

Agents are concurrent computation units that communicate via message passing. They implement Ternlang's actor model.

### Definition

```ternlang
agent Name {
    fn handle(msg: trit) -> trit {
        // process message
        return msg;
    }
}
```

An agent body contains one or more methods. In v0.1.2, the primary method is `handle`.

### Spawning Agents

```ternlang
// Local agent on the current node
let v: agentref = spawn Voter;

// Remote agent on another node
let r: agentref = spawn remote "192.168.1.10:7373" Voter;
```

`spawn` evaluates to an `agentref` — a handle to the running instance.

### Sending Messages

```ternlang
send v 1;          // send trit 1 to agent v
send v my_signal;  // send trit variable
```

### Receiving Results

```ternlang
let reply: trit = await v;
```

### Full Agent Example

```ternlang
agent Voter {
    fn handle(msg: trit) -> trit {
        match msg {
             1 => { return  1; }
             0 => { return  0; }
            -1 => { return -1; }
        }
    }
}

fn run_vote(signal: trit) -> trit {
    let v: agentref = spawn Voter;
    send v signal;
    let result: trit = await v;
    return result;
}
```

### Distributed Agents

```ternlang
fn distributed_decision() -> trit {
    let node_a: agentref = spawn remote "10.0.0.1:7373" Analyst;
    let node_b: agentref = spawn remote "10.0.0.2:7373" Analyst;
    let addr: string = nodeid;   // this node's address
    send node_a 1;
    send node_b 1;
    let a: trit = await node_a;
    let b: trit = await node_b;
    return consensus(a, b);
}
```

---

## 11. Modules and `use`

### Built-in Standard Library

Import with `use path::to::module;` inside any function body.

| Module          | Provides                                              |
|-----------------|-------------------------------------------------------|
| `std::trit`     | `abs`, `min`, `max`, `clamp`, `threshold`, `sign`, `majority` |
| `std::math`     | `ternadd3`, `neg`, `balance`, `step`, `rectify`       |
| `std::tensor`   | `zeros`, `sparse_mm`, `dense_mm`                      |
| `std::io`       | `print_trit`, `print_trit_labeled`, `print_trit_num`, `print_tensor`, `newline` |
| `ml::quantize`  | `quantize_one`, `hard_threshold`, `soft_threshold`    |
| `ml::inference` | `linear`, `linear_dense`, `attend`, `decide`          |

### User-Defined Modules

Any `.tern` file can be imported as a module. The resolver looks for `<segment>/<segment>.tern` relative to the source file's directory:

```ternlang
use mymod::utils;   // loads ./mymod/utils.tern
```

### Resolution Order

1. Built-in stdlib (embedded at compile time, zero I/O)
2. User-defined file (relative to source file directory)
3. If neither matches: `[MOD-001]` warning, module silently skipped

---

## 12. Built-in Functions

These functions are always available without any `use` statement. They are registered directly in the semantic analyzer.

### Trit Operations

| Function                             | Returns | Description                                                     |
|--------------------------------------|---------|-----------------------------------------------------------------|
| `consensus(a: trit, b: trit) -> trit` | `trit`  | Balanced ternary multiply. `consensus(1,1)=1`, `consensus(1,0)=1`, `consensus(1,-1)=-1`, `consensus(0,x)=0` |
| `invert(x: trit) -> trit`            | `trit`  | Negate: `1→-1`, `0→0`, `-1→1`                                  |
| `truth() -> trit`                    | `trit`  | Returns `1` (constant truth)                                    |
| `hold() -> trit`                     | `trit`  | Returns `0` (constant hold/neutral)                             |
| `conflict() -> trit`                 | `trit`  | Returns `-1` (constant conflict)                                |
| `mul(a: trit, b: trit) -> trit`      | `trit`  | Alias for `consensus`. Sign-preserving multiplication.          |

**`consensus` truth table:**

| `a`  | `b`  | `consensus(a,b)` |
|------|------|-----------------|
| `1`  | `1`  | `1`             |
| `1`  | `0`  | `1`             |
| `1`  | `-1` | `-1`            |
| `0`  | `1`  | `1`             |
| `0`  | `0`  | `0`             |
| `0`  | `-1` | `-1`            |
| `-1` | `1`  | `-1`            |
| `-1` | `0`  | `-1`            |
| `-1` | `-1` | `-1`            |

### Tensor Operations (variadic)

| Function                     | Returns          | Description                                       |
|------------------------------|------------------|---------------------------------------------------|
| `matmul(a, b)`               | `trittensor`     | Sparse-aware ternary matrix multiply              |
| `sparsity(t)`                | `int`            | Count of zero trits in tensor                     |
| `shape(t)`                   | `int`            | Number of elements in tensor                      |
| `zeros(size)`                | `trittensor`     | Allocate hold-initialised tensor                  |

### I/O (variadic)

| Function         | Returns | Description                     |
|------------------|---------|---------------------------------|
| `print(...)`     | `trit`  | Print value(s) to stdout        |
| `println(...)`   | `trit`  | Print with trailing newline     |

### Math (exact signatures)

| Function                         | Returns | Description              |
|----------------------------------|---------|--------------------------|
| `abs(x: int) -> int`             | `int`   | Absolute value of int    |
| `min(a: int, b: int) -> int`     | `int`   | Minimum of two ints      |
| `max(a: int, b: int) -> int`     | `int`   | Maximum of two ints      |

> **Note:** For trit `abs`/`min`/`max`, use `use std::trit;` — those versions operate on trits.

### ML Built-ins (variadic)

| Function         | Returns      | Description                       |
|------------------|--------------|-----------------------------------|
| `quantize(...)`  | `trittensor` | Quantize float tensor to trits    |
| `threshold(...)` | `float`      | Compute quantization threshold    |
| `forward(...)`   | `trittensor` | Run a forward pass (inference)    |
| `argmax(...)`    | `int`        | Index of maximum-valued trit      |

### Type Coercion

| Function      | Returns | Description                          |
|---------------|---------|--------------------------------------|
| `cast(expr)`  | target  | Coerce to type inferred from context |

---

## 13. Standard Library

### `std::trit`

```ternlang
use std::trit;
```

| Function                                   | Description                                                         |
|--------------------------------------------|---------------------------------------------------------------------|
| `abs(x: trit) -> trit`                     | `-1 → 1`, `0 → 0`, `1 → 1`                                        |
| `min(a: trit, b: trit) -> trit`            | Minimum of two trits                                               |
| `max(a: trit, b: trit) -> trit`            | Maximum of two trits                                               |
| `clamp(x: trit, lo: trit, hi: trit) -> trit` | Clamp x to range [lo, hi]                                       |
| `threshold(x: trit) -> trit`              | `1 → 1`, `0 → -1`, `-1 → -1` (binary-like collapse)              |
| `sign(x: trit) -> trit`                   | Identity — documents that sign is meaningful                       |
| `majority(a: trit, b: trit, c: trit) -> trit` | Majority vote of three trits; returns state held by ≥ 2       |

```ternlang
fn vote(a: trit, b: trit, c: trit) -> trit {
    use std::trit;
    return majority(a, b, c);
}
```

---

### `std::math`

```ternlang
use std::math;
```

| Function                              | Description                                              |
|---------------------------------------|----------------------------------------------------------|
| `ternadd3(a: trit, b: trit, c: trit) -> trit` | Balanced ternary 3-input addition with carry     |
| `neg(x: trit) -> trit`               | Alias for `invert` — negate a trit                       |
| `balance(x: trit, damping: trit) -> trit` | Dampen oscillating signals toward hold               |
| `step(x: trit) -> trit`              | Hold → Truth, Truth → Truth, Conflict → Conflict (activation analogue) |
| `rectify(x: trit) -> trit`           | Conflict → Hold, else identity. Ternary ReLU analogue.   |

```ternlang
fn activate(x: trit) -> trit {
    use std::math;
    return rectify(x);   // ternary ReLU: conflict becomes hold
}
```

---

### `std::tensor`

```ternlang
use std::tensor;
```

| Function                                                 | Description                               |
|----------------------------------------------------------|-------------------------------------------|
| `zeros(size: int) -> trittensor<1 x 1>`                 | Allocate hold-initialised tensor          |
| `sparse_mm(a: trittensor, b: trittensor) -> trittensor` | Sparse matrix multiply (skips zeros)      |
| `dense_mm(a: trittensor, b: trittensor) -> trittensor`  | Dense matrix multiply (no zero skipping)  |

```ternlang
fn run_layer(W: trittensor<128 x 256>, x: trittensor<256>) -> trittensor<128 x 1> {
    use std::tensor;
    return sparse_mm(W, x);
}
```

---

### `std::io`

```ternlang
use std::io;
```

| Function                                         | Description                              |
|--------------------------------------------------|------------------------------------------|
| `print_trit(x: trit) -> trit`                   | Print trit as symbolic name              |
| `print_trit_labeled(label: string, x: trit) -> trit` | Print trit with label prefix        |
| `print_trit_num(x: trit) -> trit`               | Print trit as numeric `-1`, `0`, or `1` |
| `print_tensor(t: trittensor<1 x 1>) -> trit`    | Print tensor contents                    |
| `newline() -> trit`                             | Print newline, return hold               |

```ternlang
fn log_decision(label: string, d: trit) -> trit {
    use std::io;
    return print_trit_labeled(label, d);
}
```

---

### `ml::quantize`

```ternlang
use ml::quantize;
```

| Function                                         | Description                                                  |
|--------------------------------------------------|--------------------------------------------------------------|
| `quantize_one(x: trit, threshold: trit) -> trit` | Identity with `@sparseskip` annotation for already-ternary values |
| `hard_threshold(x: trit) -> trit`               | `1 → 1`, `0 → -1`, `-1 → -1`. Dense binary-like collapse.   |
| `soft_threshold(x: trit) -> trit`               | Identity — signals that the value is already at threshold    |

---

### `ml::inference`

```ternlang
use ml::inference;
```

| Function                                                         | Description                                                 |
|------------------------------------------------------------------|-------------------------------------------------------------|
| `linear(W: trittensor, x: trittensor) -> trittensor`            | Sparse ternary linear layer (skips zero-weight connections) |
| `linear_dense(W: trittensor, x: trittensor) -> trittensor`      | Dense ternary linear layer (baseline comparison)            |
| `attend(q: trit, k: trit) -> trit`                              | Ternary attention score via `consensus(q, k)`               |
| `decide(evidence: trittensor) -> trit`                          | Reduce tensor to single trit decision via sparsity          |

```ternlang
fn infer(W: trittensor<128 x 256>, x: trittensor<256>) -> trit {
    use ml::inference;
    let logits: trittensor<128 x 1> = linear(W, x);
    return decide(logits);
}
```

---

## 14. Directives

Directives are compile-time annotations prefixed with `@`. They are applied to the immediately following statement.

### `@sparseskip`

Marks a tensor operation for zero-skipping optimization. Any multiplication involving a zero trit is skipped entirely — the primary performance mechanism of the ternary stack.

```ternlang
@sparseskip let out: trittensor<128 x 256> = matmul(W, x);
```

This is the ternary analogue of sparse matrix acceleration. In quantized models where ~50% of weights are zero, `@sparseskip` can halve effective computation.

**Semantics:** The directive is passed through as a `Decorated` AST node. The backend (BET bytecode / LLVM codegen) emits zero-branch guards around the operation.

---

## 15. Grammar (EBNF)

```ebnf
program       ::= (struct_def | agent_def | function)*

struct_def    ::= "struct" IDENT "{" field_list "}"
field_list    ::= (field ("," field)*)? ","?
field         ::= IDENT ":" type

agent_def     ::= "agent" IDENT "{" function* "}"

function      ::= "fn" IDENT "(" param_list ")" "->" type block
param_list    ::= (param ("," param)*)?
param         ::= IDENT ":" type

block         ::= "{" stmt* "}"

stmt          ::=   "@" directive stmt
                |   "use" module_path ";"
                |   "let" "mut"? IDENT ":" type ("=" expr)? ";"
                |   "if" expr "?" block "else" block "else" block
                |   "match" expr "{" match_arm match_arm match_arm "}"
                |   "while" expr "?" block "else" block "else" block
                |   "for" IDENT "in" expr block
                |   "loop" block
                |   "send" expr expr ";"
                |   "return" expr ";"
                |   "break" ";"
                |   "continue" ";"
                |   block
                |   IDENT "." IDENT "=" expr ";"
                |   expr ";"

match_arm     ::= trit_literal "=>" stmt

directive     ::= "sparseskip" | IDENT

module_path   ::= IDENT ("::" IDENT)*

expr          ::= expr BINOP expr
                | unary_expr

unary_expr    ::= "-" primary_expr
                | primary_expr ("." IDENT | "?")*

primary_expr  ::=   trit_literal
                |   INT_LITERAL
                |   STRING_LITERAL
                |   "spawn" ("remote" STRING_LITERAL)? IDENT
                |   "await" unary_expr
                |   "nodeid"
                |   "cast" "(" expr ")"
                |   IDENT "(" arg_list ")"
                |   IDENT
                |   "(" expr ")"

arg_list      ::= (expr ("," expr)*)?

type          ::=   "trit"
                |   "trittensor" "<" dim ("x" dim)* ">"
                |   "agentref"
                |   "int"
                |   "float"
                |   "bool"
                |   "string"
                |   IDENT

dim           ::= INT_LITERAL | trit_literal    // trit_literal covers "0" and "1"

trit_literal  ::= "-1" | "0" | "1"

BINOP         ::= "+" | "-" | "*" | "==" | "!=" | "&&" | "||"
```

---

## 16. Diagnostic Codes

All compiler errors include a bracketed code for easy grepping and tooling integration.

### Parse Errors (`[PARSE-*]`)

| Code         | Trigger                                         | Example message                                                                 |
|--------------|-------------------------------------------------|---------------------------------------------------------------------------------|
| `[PARSE-001]`| Unexpected token in the input stream            | `[PARSE-001] Unexpected token 'FatArrow' — the lexer hit something it didn't expect.` |
| `[PARSE-002]`| Expected a specific token but found another     | `[PARSE-002] Expected Semicolon but found 'RBrace'. The grammar demands it.`   |
| `[PARSE-003]`| A trit literal with an invalid value            | `[PARSE-003] '2' is not a valid trit. Trits are -1, 0, or +1. No in-betweens.` |
| `[PARSE-004]`| `match` block missing one or more of -1/0/1 arms | `[PARSE-004] Non-exhaustive match: match missing arms: 0 (hold). Ternary has three states — cover all three.` |

### Semantic / Type Errors (`[TYPE-*]`, `[SCOPE-*]`, `[STRUCT-*]`, `[FN-*]`, `[PROP-*]`)

| Code          | Trigger                                        | Example message                                                                     |
|---------------|------------------------------------------------|-------------------------------------------------------------------------------------|
| `[TYPE-001]`  | Type mismatch in binding, argument, or return  | `[TYPE-001] Type mismatch: expected Trit, found Int. Binary types don't map cleanly to ternary space.` |
| `[SCOPE-001]` | Identifier used before declaration             | `[SCOPE-001] 'ghost_var' is undefined. Hold state — declare before use.`           |
| `[STRUCT-001]`| Unknown struct type referenced                 | `[STRUCT-001] Struct 'Foo' doesn't exist. The type system can't find it.`          |
| `[STRUCT-002]`| Field name not defined on struct               | `[STRUCT-002] Struct 'Signal' has no field 'bias'. Check your definition.`         |
| `[FN-001]`    | Call to undefined function                     | `[FN-001] 'mystery_fn' is not defined. Did you forget to declare it or import its module?` |
| `[FN-002]`    | Return type doesn't match function declaration | `[FN-002] Function 'decide' declared return type Trit but returned Int.`           |
| `[FN-003]`    | Wrong number of arguments                      | `[FN-003] 'consensus' expects 2 arg(s), got 1. Arity is not optional.`            |
| `[FN-004]`    | Argument type mismatch                         | `[FN-004] 'invert' arg 0: expected Trit, found Int. Types travel with their values.` |
| `[PROP-001]`  | `?` propagation on a non-trit expression       | `[PROP-001] '?' used on a Int expression. Only trit-returning functions can signal conflict.` |

### Module Errors (`[MOD-*]`)

| Code        | Trigger                              | Example message                                                                 |
|-------------|--------------------------------------|---------------------------------------------------------------------------------|
| `[MOD-000]` | A module file failed to parse        | `[MOD-000] Failed to parse module 'mymod::utils': [PARSE-001] ...`             |
| `[MOD-001]` | `use` path not found in stdlib or filesystem | `[MOD-001] Unknown module 'std::nonexistent' — no stdlib match and no file found. Did you mean std::trit?` |

### BET Bytecode Errors (`[BET-*]`)

BET (Binary Encoding of Ternary) is the internal wire format that packs 4 trits per byte using 2-bit codes: `NegOne=01`, `Zero=11`, `PosOne=10`. The invalid pattern `00` is a fault.

| Code        | Trigger                              | Description                                                 |
|-------------|--------------------------------------|-------------------------------------------------------------|
| `[BET-001]` | Invalid BET byte state (`0b00`)      | Encountered the forbidden null bit pattern during unpack     |
| `[BET-002]` to `[BET-007]` | BET VM runtime faults  | Instruction decode errors, stack violations, agent mailbox faults, etc. |

---

## Quick Reference Card

```ternlang
// The three states
let truth:    trit = 1;
let hold:     trit = 0;
let conflict: trit = -1;

// Decision logic
fn decide(a: trit, b: trit) -> trit {
    if consensus(a, b) ? {
        return 1;    // both agree positive
    } else {
        return 0;    // uncertain
    } else {
        return -1;   // conflict
    }
}

// Exhaustive match (all 3 arms required)
match signal {
     1 => { return  1; }
     0 => { return  0; }
    -1 => { return -1; }
}

// Conflict propagation
fn pipeline(x: trit) -> trit {
    let a: trit = stage_one(x)?;   // returns -1 immediately on conflict
    let b: trit = stage_two(a)?;
    return b;
}

// Sparse ML layer
fn layer(W: trittensor<128 x 256>, x: trittensor<256>) -> trittensor<128 x 1> {
    use ml::inference;
    return linear(W, x);   // @sparseskip applied inside linear()
}

// Agent message passing
agent Voter {
    fn handle(msg: trit) -> trit { return msg; }
}

fn run() -> trit {
    let v: agentref = spawn Voter;
    send v 1;
    return await v;
}
```

---

*Ternlang Language Reference — RFI-IRFOS Ternary Intelligence Stack v0.1.2*
*Source of truth: `ternlang-core/src/{lexer,ast,parser,semantic}.rs` and `ternlang-core/stdlib/`*
