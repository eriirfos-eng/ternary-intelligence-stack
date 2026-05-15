#!/usr/bin/env python3
"""
test_wmma_training.py — validate WMMA + STE training stability on T4.

Runs a self-contained CUDA C mini-training loop:
  float shadow weights → quantize W to i8 → WMMA forward →
  MSE loss → STE backward (grad uses float W, float X) →
  SGD update → 100 steps

If loss descends: WMMA is safe for a fresh training run.
If loss diverges: there is a kernel or gradient bug independent of Adam shock.

Usage:
    modal run scripts/test_wmma_training.py
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

# Self-contained training loop in CUDA C.
# Architecture:  X [M=32, K=256]  →  WMMA  →  Y [M=32, N=256]
# Target:        Y_target [M=32, N=256]  (fixed random, drawn once)
# Loss:          MSE = sum((Y - Y_target)^2) / (M*N)
# STE backward:  dW_float = (dL/dY)^T @ X_float   (uses float X, ignores quant)
# Optimizer:     SGD  lr=0.01
# Steps:         100  (report every 10)
#
# gamma is recomputed each step as mean(|W_float|) — matches ternary_linear.rs.
# Expected: loss decreases monotonically (or near-monotonically) toward 0.
TRAIN_SRC = textwrap.dedent(r"""
#include <cuda_runtime.h>
#include <mma.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define M      32
#define N      256
#define K      256
#define STEPS  100
#define LR     0.01f
#define REPORT 10

// Forward declarations
extern "C" {
    void ternary_quantize(unsigned long long w_f32, unsigned long long w_i8,
                          int size, cudaStream_t stream);
    void ternary_gemm_forward(unsigned long long x_f32, unsigned long long w_i8,
                              unsigned long long y_f32,
                              unsigned long long x_i8_buf, unsigned long long scales_buf,
                              unsigned long long y_i32_buf,
                              int M, int N, int K, float gamma, cudaStream_t stream);
}

// CPU: MSE loss and STE gradient
// dL/dW_float[n,k] = sum_m( 2*(Y[m,n]-Ytar[m,n])/(M*N) * X_float[m,k] )
static float mse_and_grad(
    const float* Y, const float* Y_target,
    const float* X_float, const float* W_float,
    float* dW,          // [N, K] — accumulated gradient
    int m, int n, int k
) {
    float loss = 0.f;
    memset(dW, 0, n * k * sizeof(float));
    float scale = 2.f / (float)(m * n);
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            float err = Y[i * n + j] - Y_target[i * n + j];
            loss += err * err;
            float g = scale * err;
            for (int p = 0; p < k; p++) {
                dW[j * k + p] += g * X_float[i * k + p];
            }
        }
    }
    return loss / (float)(m * n);
}

int main() {
    srand(42);

    // --- Fixed random inputs (X never changes — we're learning W) ---
    float* h_X = (float*)malloc(M * K * sizeof(float));
    for (int i = 0; i < M * K; i++)
        h_X[i] = ((float)rand() / RAND_MAX) * 2.f - 1.f;

    // --- Fixed random target ---
    float* h_Ytar = (float*)malloc(M * N * sizeof(float));
    for (int i = 0; i < M * N; i++)
        h_Ytar[i] = ((float)rand() / RAND_MAX) * 2.f - 1.f;

    // --- Float shadow weights (initialised small, like ternary_linear.rs) ---
    float* h_W = (float*)malloc(N * K * sizeof(float));
    for (int i = 0; i < N * K; i++)
        h_W[i] = (((float)rand() / RAND_MAX) * 2.f - 1.f) * 0.05f;

    float* h_dW = (float*)malloc(N * K * sizeof(float));
    float* h_Y  = (float*)malloc(M * N * sizeof(float));

    // --- GPU buffers ---
    float       *d_X, *d_Y;
    signed char *d_W_i8;
    void        *d_scratch_x_i8, *d_scratch_scales, *d_scratch_y_i32;

    cudaMalloc(&d_X,   M * K * sizeof(float));
    cudaMalloc(&d_Y,   M * N * sizeof(float));
    cudaMalloc(&d_W_i8, N * K * sizeof(signed char));
    cudaMalloc(&d_scratch_x_i8,   M * K * sizeof(signed char));
    cudaMalloc(&d_scratch_scales, M * sizeof(float));
    cudaMalloc(&d_scratch_y_i32,  M * N * sizeof(int));

    cudaMemcpy(d_X, h_X, M * K * sizeof(float), cudaMemcpyHostToDevice);

    // --- Temp GPU buffer for float W (for ternary_quantize input) ---
    float* d_W_f32;
    cudaMalloc(&d_W_f32, N * K * sizeof(float));

    printf("step  loss\n");
    printf("----  --------\n");

    for (int step = 0; step < STEPS; step++) {
        // 1. Compute gamma = mean(|W_float|)
        float gamma = 0.f;
        for (int i = 0; i < N * K; i++) gamma += fabsf(h_W[i]);
        gamma /= (float)(N * K);
        if (gamma < 1e-9f) gamma = 1e-9f;

        // 2. Upload float W, quantize to i8 on GPU
        cudaMemcpy(d_W_f32, h_W, N * K * sizeof(float), cudaMemcpyHostToDevice);
        ternary_quantize(
            (unsigned long long)d_W_f32,
            (unsigned long long)d_W_i8,
            N * K, nullptr
        );

        // 3. WMMA forward
        ternary_gemm_forward(
            (unsigned long long)d_X,
            (unsigned long long)d_W_i8,
            (unsigned long long)d_Y,
            (unsigned long long)d_scratch_x_i8,
            (unsigned long long)d_scratch_scales,
            (unsigned long long)d_scratch_y_i32,
            M, N, K, gamma, nullptr
        );
        cudaDeviceSynchronize();

        // 4. Copy Y back, compute MSE + STE gradient on CPU
        cudaMemcpy(h_Y, d_Y, M * N * sizeof(float), cudaMemcpyDeviceToHost);
        float loss = mse_and_grad(h_Y, h_Ytar, h_X, h_W, h_dW, M, N, K);

        // 5. SGD update
        for (int i = 0; i < N * K; i++)
            h_W[i] -= LR * h_dW[i];

        if ((step % REPORT) == 0 || step == STEPS - 1)
            printf("%4d  %.6f\n", step, loss);
    }

    printf("\nDone. Loss should decrease from step 0 to step %d.\n", STEPS-1);
    return 0;
}
""")

app = modal.App("wmma-training-test")


@app.function(image=image, gpu="T4", timeout=300)
def test_training():
    import subprocess, sys

    with open("/test_wmma_training.cu", "w") as f:
        f.write(TRAIN_SRC)

    nvcc = "/usr/local/cuda/bin/nvcc"

    compile = subprocess.run([
        nvcc, "--gpu-architecture", "sm_75",
        "-O2", "-o", "/test_wmma_training",
        "/test_wmma_training.cu", "/ternary_gemm.cu",
    ], capture_output=True, text=True)

    if compile.returncode != 0:
        print("COMPILE FAIL")
        print(compile.stderr[-3000:])
        sys.exit(1)

    print("Compiled OK. Running 100-step training loop...")
    run = subprocess.run(["/test_wmma_training"], capture_output=True, text=True)
    print(run.stdout)
    if run.stderr:
        print(run.stderr[-1000:])
    sys.exit(run.returncode)


@app.local_entrypoint()
def main():
    test_training.remote()
