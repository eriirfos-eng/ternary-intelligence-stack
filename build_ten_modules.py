import os
import subprocess

modules = [
    ("math/factorial.tern", "fn factorial(n: int) -> int { if (n <= 1) { return 1; } return n * factorial(n - 1); } fn main() -> trit { if (factorial(5) == 120) { return affirm; } return reject; }"),
    ("math/fibonacci.tern", "fn fib(n: int) -> int { if (n <= 1) { return n; } return fib(n - 1) + fib(n - 2); } fn main() -> trit { if (fib(6) == 8) { return affirm; } return reject; }"),
    ("lib/stack.tern", "fn push(s: int[], top: int, val: int) -> int { s[top] = val; return top + 1; } fn main() -> trit { let s: int[5] = 0; let top: int = push(s, 0, 42); if (s[0] == 42 && top == 1) { return affirm; } return reject; }"),
    ("lib/queue.tern", "fn enqueue(q: int[], tail: int, val: int) -> int { q[tail] = val; return tail + 1; } fn main() -> trit { let q: int[5] = 0; let tail: int = enqueue(q, 0, 7); if (q[0] == 7 && tail == 1) { return affirm; } return reject; }"),
    ("lib/max_array.tern", "fn max_arr(a: int[], len: int) -> int { let m: int = a[0]; let i: int = 1; while (i < len) { if (a[i] > m) { m = a[i]; } i = i + 1; } return m; } fn main() -> trit { let a: int[3] = 0; a[0] = 5; a[1] = 15; a[2] = 10; if (max_arr(a, 3) == 15) { return affirm; } return reject; }"),
    ("lib/min_array.tern", "fn min_arr(a: int[], len: int) -> int { let m: int = a[0]; let i: int = 1; while (i < len) { if (a[i] < m) { m = a[i]; } i = i + 1; } return m; } fn main() -> trit { let a: int[3] = 0; a[0] = 5; a[1] = 15; a[2] = 10; if (min_arr(a, 3) == 5) { return affirm; } return reject; }"),
    ("std/abs_float.tern", "fn abs_f(x: float) -> float { if (x < 0.0) { return -x; } return x; } fn main() -> trit { if (abs_f(-3.14) == 3.14) { return affirm; } return reject; }"),
    ("std/clamp.tern", "fn clamp(x: int, min: int, max: int) -> int { if (x < min) { return min; } if (x > max) { return max; } return x; } fn main() -> trit { if (clamp(10, 0, 5) == 5) { return affirm; } return reject; }"),
    ("crypto/parity.tern", "fn parity(n: int) -> int { let c: int = 0; let x: int = n; while (x > 0) { c = c + (x % 2); x = x / 2; } return c % 2; } fn main() -> trit { if (parity(7) == 1) { return affirm; } return reject; }"),
    ("math/power.tern", "fn power(base: int, exp: int) -> int { let res: int = 1; let i: int = 0; while (i < exp) { res = res * base; i = i + 1; } return res; } fn main() -> trit { if (power(2, 4) == 16) { return affirm; } return reject; }")
]

for path, code in modules:
    full_path = f"ternlang-root/stdlib/{path}"
    os.makedirs(os.path.dirname(full_path), exist_ok=True)
    with open(full_path, "w") as f:
        f.write(code)
    try:
        subprocess.run(["./target/debug/ternlang", "run", full_path], check=True, capture_output=True)
        subprocess.run(["git", "add", full_path], check=True)
        subprocess.run(["git", "commit", "-m", f"stdlib/{path}: add module"], check=True)
    except Exception as e:
        print(f"Failed {path}: {e}")
