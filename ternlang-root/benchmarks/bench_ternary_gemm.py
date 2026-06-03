"""Honest measurement of the multiply-free, zero-skip ternary GEMM.

This is the dense, side-effect-free numeric domain that N1 (in TERNARY_FINDINGS)
predicts @sparseskip should pay in: Y = X @ W with W in {-1, 0, +1}.

We measure three things and report them separately, because they are not the
same claim:

  1. correctness  -- the multiply-free / zero-skip result equals dense X @ W.
  2. arithmetic   -- multiplies and adds, dense vs ternary. This is the real,
                     hardware-independent win: a ternary matmul needs ZERO
                     multiplies, and only the non-zero weights contribute an
                     add. Reduction is exact and large.
  3. wall-clock   -- dense BLAS matmul vs portable "multiply-free" formulations
                     (mask decomposition and sparse zero-skip) across sparsity.
                     This is where we stay honest: stock framework ops cannot
                     cash the arithmetic win, so we report whether/where a
                     portable path crosses, and otherwise say plainly that the
                     speedup needs a custom kernel (bit-serial / LUT, the
                     bitnet.cpp / T-MAC approach).

Run:  KERAS_BACKEND=jax python bench_ternary_gemm.py
"""

import time

import jax
import jax.numpy as jnp
import numpy as np
from jax.experimental import sparse as jsparse


def make_ternary(k, n, p_zero, seed=0):
    """Ternary weight matrix [k, n] with a target zero fraction p_zero."""
    rng = np.random.default_rng(seed)
    nonzero = rng.random((k, n)) >= p_zero
    signs = rng.integers(0, 2, size=(k, n), dtype=np.int8) * 2 - 1
    return (nonzero * signs).astype(np.float32)


def op_counts(b, k, n, measured_nnz):
    """Exact arithmetic op-counts for Y = X @ W, [b,k] @ [k,n]."""
    macs = b * k * n
    dense_mul = macs  # one multiply per MAC
    dense_add = macs  # one add per MAC
    tern_mul = 0  # weights are +-1: no multiply at all
    tern_add = b * int(measured_nnz)  # one add/sub per non-zero weight, per row
    return dense_mul, dense_add, tern_mul, tern_add


def timeit(fn, *args, warmup=3, iters=30):
    for _ in range(warmup):
        jax.block_until_ready(fn(*args))
    t0 = time.perf_counter()
    for _ in range(iters):
        out = fn(*args)
    jax.block_until_ready(out)
    return (time.perf_counter() - t0) / iters


@jax.jit
def dense_matmul(x, w):
    return x @ w


@jax.jit
def mask_decomp(x, w_pos, w_neg):
    # "Multiply-free" by construction: products are with {0,1} masks, so the
    # math is adds and subtracts only. On hardware it is still two BLAS matmuls.
    return x @ w_pos - x @ w_neg


@jax.jit
def sparse_skip(w_t_sparse, x):
    # True zero-skip: only the non-zero +-1 entries take part.
    # X @ W == (W^T @ X^T)^T, with W^T held sparse.
    return (w_t_sparse @ x.T).T


def main():
    b, k, n = 64, 2048, 2048
    x = jnp.asarray(
        np.random.default_rng(1).standard_normal((b, k)).astype(np.float32)
    )

    print(f"Y = X[{b},{k}] @ W[{k},{n}],  W in {{-1,0,+1}}")
    print("fp32 dense matmul (XLA BLAS) is the baseline.\n")
    header = (
        f"{'zero%':>6} {'dense ms':>9} {'mask ms':>8} {'sparse ms':>10} "
        f"{'best vs dense':>14} {'mul elim':>9} {'add elim':>9} {'ok':>4}"
    )
    print(header)
    print("-" * len(header))

    for p in [0.30, 0.50, 0.6667, 0.80, 0.90, 0.95, 0.98, 0.99]:
        w = make_ternary(k, n, p, seed=0)
        nnz = int((w != 0).sum())
        wj = jnp.asarray(w)
        w_pos = jnp.asarray((w > 0).astype(np.float32))
        w_neg = jnp.asarray((w < 0).astype(np.float32))
        w_t_sparse = jsparse.BCOO.fromdense(wj.T)

        y_dense = dense_matmul(x, wj)
        y_mask = mask_decomp(x, w_pos, w_neg)
        y_sparse = sparse_skip(w_t_sparse, x)
        ok = (
            float(jnp.max(jnp.abs(y_dense - y_mask))) < 1e-2
            and float(jnp.max(jnp.abs(y_dense - y_sparse))) < 1e-2
        )

        t_dense = timeit(dense_matmul, x, wj)
        t_mask = timeit(mask_decomp, x, w_pos, w_neg)
        t_sparse = timeit(sparse_skip, w_t_sparse, x)

        dmul, dadd, tmul, tadd = op_counts(b, k, n, nnz)
        best = min(t_mask, t_sparse)
        print(
            f"{p:6.0%} {t_dense * 1e3:9.3f} {t_mask * 1e3:8.3f} "
            f"{t_sparse * 1e3:10.3f} {t_dense / best:13.2f}x "
            f"{1 - tmul / dmul:9.0%} {1 - tadd / dadd:9.0%} {str(ok):>4}"
        )

    print(
        "\nmul elim / add elim are the exact arithmetic reductions (hardware "
        "independent).\n'best vs dense' > 1.0 means a portable path beat BLAS; "
        "<= 1.0 means the\narithmetic win is real but needs a custom kernel to "
        "become wall-clock."
    )


if __name__ == "__main__":
    main()
