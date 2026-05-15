#!/usr/bin/env python3
"""
test_wmma_numerics.py — compile + run a numerical correctness test for the
fused WMMA kernel against a CPU reference on the actual Modal T4 container.

Usage:
    modal run scripts/test_wmma_numerics.py
"""

import os
import textwrap

_HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

import modal

image = (
    modal.Image.from_registry(
        "nvidia/cuda:12.1.0-cudnn8-devel-ubuntu22.04",
        add_python="3.11",
    )
    .apt_install("build-essential")
    .add_local_file(
        os.path.join(_HERE, "moe-llm-core/cuda/ternary_gemm.cu"),
        remote_path="/ternary_gemm.cu",
    )
)

# Self-contained test program — compiled alongside ternary_gemm.cu on the container.
TEST_SRC = textwrap.dedent(r"""
#include <cuda_runtime.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

// Forward declarations from ternary_gemm.cu
extern "C" {
    void ternary_quantize(unsigned long long w_f32, unsigned long long w_i8,
                          int size, cudaStream_t stream);
    void ternary_gemm_forward(unsigned long long x_f32, unsigned long long w_i8,
                              unsigned long long y_f32,
                              unsigned long long x_i8_buf, unsigned long long scales_buf,
                              unsigned long long y_i32_buf,
                              int M, int N, int K, float gamma, cudaStream_t stream);
}

#define M   16
#define N   16
#define K   256
#define GAMMA 1.5f

int main() {
    // --- Build test inputs on CPU ---
    float  h_X[M * K];
    float  h_W_f32[N * K];  // ternary f32: +GAMMA, 0, -GAMMA
    signed char h_W_i8[N * K];

    srand(42);
    for (int i = 0; i < M * K; i++)
        h_X[i] = ((float)rand() / RAND_MAX) * 2.f - 1.f;

    // W: random ternary {-GAMMA, 0, +GAMMA}  (25% each sign, 50% zero)
    for (int i = 0; i < N * K; i++) {
        int r = rand() % 4;
        h_W_f32[i] = (r == 0) ? GAMMA : (r == 1) ? -GAMMA : 0.f;
        h_W_i8[i]  = (r == 0) ? 1     : (r == 1) ? -1     : 0;
    }

    // --- CPU reference: per-row abs-max quantise X, int matmul, dequant ---
    float h_Y_ref[M * N] = {0};
    for (int m = 0; m < M; m++) {
        float amax = 0.f;
        for (int k = 0; k < K; k++) amax = fmaxf(amax, fabsf(h_X[m * K + k]));
        float scale   = (amax > 0.f) ? (127.f / amax) : 1.f;
        float dequant = (amax > 0.f) ? (amax / 127.f) : 1.f;
        for (int n = 0; n < N; n++) {
            int acc = 0;
            for (int k = 0; k < K; k++) {
                signed char xi = (signed char)rintf(fmaxf(-127.f, fminf(127.f,
                                                    h_X[m*K+k] * scale)));
                acc += (int)xi * (int)h_W_i8[n * K + k];
            }
            h_Y_ref[m * N + n] = (float)acc * dequant * GAMMA;
        }
    }

    // --- GPU buffers ---
    float       *d_X, *d_Y;
    signed char *d_W_i8;
    void        *d_scratch_x_i8, *d_scratch_scales, *d_scratch_y_i32;

    cudaMalloc(&d_X,   M * K * sizeof(float));
    cudaMalloc(&d_Y,   M * N * sizeof(float));
    cudaMalloc(&d_W_i8, N * K * sizeof(signed char));
    cudaMalloc(&d_scratch_x_i8,     M * K * sizeof(signed char));
    cudaMalloc(&d_scratch_scales,   M * sizeof(float));
    cudaMalloc(&d_scratch_y_i32,    M * N * sizeof(int));

    cudaMemcpy(d_X,    h_X,    M * K * sizeof(float),       cudaMemcpyHostToDevice);
    cudaMemcpy(d_W_i8, h_W_i8, N * K * sizeof(signed char), cudaMemcpyHostToDevice);

    // --- Run fused kernel ---
    ternary_gemm_forward(
        (unsigned long long)d_X,
        (unsigned long long)d_W_i8,
        (unsigned long long)d_Y,
        (unsigned long long)d_scratch_x_i8,
        (unsigned long long)d_scratch_scales,
        (unsigned long long)d_scratch_y_i32,
        M, N, K, GAMMA, nullptr
    );
    cudaDeviceSynchronize();

    // --- Copy back and compare ---
    float h_Y[M * N];
    cudaMemcpy(h_Y, d_Y, M * N * sizeof(float), cudaMemcpyDeviceToHost);

    int errors = 0;
    float max_err = 0.f;
    for (int i = 0; i < M * N; i++) {
        float err = fabsf(h_Y[i] - h_Y_ref[i]);
        if (err > max_err) max_err = err;
        // Tolerance: quantisation introduces ~0.5 LSB per element, K=256 accumulations
        // → expected max error ≈ 0.5 * (amax/127) * K * gamma ≈ few units
        if (err > 5.0f) {
            if (errors < 5)
                printf("  MISMATCH [%d,%d]: got %.4f  ref %.4f  err %.4f\n",
                       i/N, i%N, h_Y[i], h_Y_ref[i], err);
            errors++;
        }
    }

    if (errors == 0)
        printf("NUMERICS OK -- max_err=%.6f over %d elements\n", max_err, M*N);
    else
        printf("NUMERICS FAIL -- %d/%d elements wrong, max_err=%.4f\n",
               errors, M*N, max_err);

    return (errors > 0) ? 1 : 0;
}
""")

app = modal.App("wmma-numerics-test")


@app.function(image=image, gpu="T4", timeout=180)
def test_numerics():
    import subprocess, sys

    with open("/test_numerics.cu", "w") as f:
        f.write(TEST_SRC)

    nvcc = "/usr/local/cuda/bin/nvcc"

    # Compile test + ternary_gemm.cu together
    compile = subprocess.run([
        nvcc, "--gpu-architecture", "sm_75",
        "-O2", "-o", "/test_numerics",
        "/test_numerics.cu", "/ternary_gemm.cu",
    ], capture_output=True, text=True)

    if compile.returncode != 0:
        print("COMPILE FAIL")
        print(compile.stderr[-3000:])
        sys.exit(1)

    print("Compiled OK. Running...")
    run = subprocess.run(["/test_numerics"], capture_output=True, text=True)
    print(run.stdout)
    if run.stderr: print(run.stderr)
    sys.exit(run.returncode)


@app.local_entrypoint()
def main():
    test_numerics.remote()
