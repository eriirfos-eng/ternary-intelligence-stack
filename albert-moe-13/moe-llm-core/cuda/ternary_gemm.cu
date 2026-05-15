// ternary_gemm.cu — WMMA INT8 ternary GEMM for albert. training
//
// Training shapes (batch=4, seq=256): M=1024, K/N ∈ {256, 1024}
// T4 tensor cores (SM 7.5): INT8 WMMA 8×8×32 → INT32 accumulator
//
// Pipeline per forward call:
//   1. row_absmax_kernel  : per-row abs-max of X_f32 → scales[M]
//   2. quantize_rows      : X_f32 → X_i8  (per-row scale, clamped to ±127)
//   3. wmma_gemm_kernel   : X_i8 @ W_i8^T → Y_i32  (tensor cores)
//   4. dequantize_rows    : Y_i32 → Y_f32  (scale * gamma per row)
//
// Backward: standard f32 matmul through candle autograd — STE unchanged.
// W is pre-quantised to {-1,0,+1} by ternary_quantize (unchanged).

#include <cuda_runtime.h>
#include <mma.h>
#include <stdint.h>

using namespace nvcuda;

// ── WMMA tile shape for INT8 on SM ≥ 7.2 ─────────────────────────────────
#define WMMA_M   8
#define WMMA_N   8
#define WMMA_K  32

// Block: 4×4 warps → 32×32 output tile, 512 threads
// Each warp owns one 8×8 WMMA output tile.
#define WARP_ROWS 4
#define WARP_COLS 4
#define BLOCK_M   (WARP_ROWS * WMMA_M)   // 32
#define BLOCK_N   (WARP_COLS * WMMA_N)   // 32
#define BLOCK_T   (WARP_ROWS * WARP_COLS * 32)  // 512

// ── Kernel 1: per-row abs-max of X_f32 ───────────────────────────────────
// One block per row; 256 threads reduce K columns.
// scales[row] = max(|X[row,:]|) / 127.0
__global__ void row_absmax_kernel(const float* X, float* scales, int M, int K) {
    __shared__ float smem[256];
    int row = blockIdx.x;
    if (row >= M) return;

    int tid = threadIdx.x;
    float lmax = 0.f;
    for (int col = tid; col < K; col += blockDim.x)
        lmax = fmaxf(lmax, fabsf(X[row * K + col]));
    smem[tid] = lmax;
    __syncthreads();

    for (int s = blockDim.x >> 1; s > 0; s >>= 1) {
        if (tid < s) smem[tid] = fmaxf(smem[tid], smem[tid + s]);
        __syncthreads();
    }
    if (tid == 0) scales[row] = (smem[0] > 1e-7f) ? smem[0] / 127.f : 1.f;
}

// ── Kernel 2: quantise X rows to INT8 ────────────────────────────────────
// 2D grid: (ceil(K/256), M). Each thread handles one element.
__global__ void quantize_rows_kernel(
    const float*  __restrict__ X,
    signed char*  __restrict__ X_i8,
    const float*  __restrict__ scales,
    int K
) {
    int row = blockIdx.y;
    int col = blockIdx.x * blockDim.x + threadIdx.x;
    if (col >= K) return;
    float inv = 1.f / scales[row];
    float v = X[row * K + col] * inv;
    v = fmaxf(-127.f, fminf(127.f, v));
    X_i8[row * K + col] = (int8_t)__float2int_rn(v);
}

// ── Kernel 3: WMMA INT8 GEMM ──────────────────────────────────────────────
// Y_i32[M,N] = X_i8[M,K] @ W_i8[N,K]^T
// Block covers BLOCK_M×BLOCK_N output; warps arranged WARP_ROWS×WARP_COLS.
// Cooperative smem load reduces global memory traffic by WARP_COLS (for X)
// and WARP_ROWS (for W) at each K-step.
//
// Assumes M, N divisible by BLOCK_M/BLOCK_N; K divisible by WMMA_K.
// All albert. training shapes satisfy this.
__global__ void wmma_gemm_kernel(
    const signed char* __restrict__ X,   // [M, K]
    const signed char* __restrict__ W,   // [N, K]
    int32_t*           __restrict__ Y,   // [M, N]
    int M, int N, int K
) {
    // Shared memory: X tile [BLOCK_M × WMMA_K] + W tile [BLOCK_N × WMMA_K]
    // = 32×32 + 32×32 = 2048 bytes — well within 64 KB/SM.
    __shared__ signed char smem_X[BLOCK_M * WMMA_K];
    __shared__ signed char smem_W[BLOCK_N * WMMA_K];

    int tid       = threadIdx.x;
    int warp_id   = tid / 32;
    int warp_row  = warp_id / WARP_COLS;
    int warp_col  = warp_id % WARP_COLS;

    int m_base = blockIdx.y * BLOCK_M;
    int n_base = blockIdx.x * BLOCK_N;

    // WMMA fragments — declared outside loop; loaded fresh each K-iteration.
    // signed char is the exact type CUDA's mma.h specialises for INT8 WMMA.
    wmma::fragment<wmma::accumulator, WMMA_M, WMMA_N, WMMA_K, int32_t> c_frag;
    wmma::fragment<wmma::matrix_a, WMMA_M, WMMA_N, WMMA_K, signed char, wmma::row_major> a_frag;
    wmma::fragment<wmma::matrix_b, WMMA_M, WMMA_N, WMMA_K, signed char, wmma::col_major> b_frag;
    wmma::fill_fragment(c_frag, 0);

    for (int k = 0; k < K; k += WMMA_K) {
        // ── Cooperative load: 512 threads fill 1024+1024 smem bytes ────────
        // Each thread loads exactly 2 bytes of X and 2 bytes of W.
        {
            int off0 = 2 * tid, off1 = off0 + 1;
            // X tile [BLOCK_M × WMMA_K], row-major
            smem_X[off0] = X[(m_base + off0 / WMMA_K) * K + k + off0 % WMMA_K];
            smem_X[off1] = X[(m_base + off1 / WMMA_K) * K + k + off1 % WMMA_K];
            // W tile [BLOCK_N × WMMA_K], row-major
            smem_W[off0] = W[(n_base + off0 / WMMA_K) * K + k + off0 % WMMA_K];
            smem_W[off1] = W[(n_base + off1 / WMMA_K) * K + k + off1 % WMMA_K];
        }
        __syncthreads();

        // ── WMMA fragment loads from smem ───────────────────────────────────
        wmma::load_matrix_sync(a_frag, smem_X + warp_row * WMMA_M * WMMA_K, WMMA_K);
        wmma::load_matrix_sync(b_frag, smem_W + warp_col * WMMA_N * WMMA_K, WMMA_K);
        wmma::mma_sync(c_frag, a_frag, b_frag, c_frag);

        __syncthreads();
    }

    // ── Store warp result to global Y ───────────────────────────────────────
    int32_t* y_ptr = Y + (m_base + warp_row * WMMA_M) * N + (n_base + warp_col * WMMA_N);
    wmma::store_matrix_sync(y_ptr, c_frag, N, wmma::mem_row_major);
}

// ── Kernel 4: dequantise Y_i32 → Y_f32 ───────────────────────────────────
// Y_f32[row, col] = Y_i32[row, col] * scales[row] * gamma
// 2D grid: (ceil(N/256), M).
__global__ void dequantize_rows_kernel(
    const int32_t* __restrict__ Y_i32,
    float*         __restrict__ Y_f32,
    const float*   __restrict__ scales,
    float gamma,
    int N
) {
    int row = blockIdx.y;
    int col = blockIdx.x * blockDim.x + threadIdx.x;
    if (col >= N) return;
    float s = scales[row] * gamma;
    Y_f32[row * N + col] = (float)Y_i32[row * N + col] * s;
}

// ── Old simple tiled kernel (kept for the W-quantise step) ────────────────
__global__ void quantize_kernel(const float* w_f32, signed char* w_i8, int size) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= size) return;
    float v = w_f32[idx];
    w_i8[idx] = (v > 0.f) ? (signed char)1 : ((v < 0.f) ? (signed char)-1 : (signed char)0);
}

// ── Public C API ──────────────────────────────────────────────────────────
extern "C" {

// Quantise ternary W: f32 {-γ,0,+γ} → int8 {-1,0,+1}
void ternary_quantize(uint64_t w_f32, uint64_t w_i8, int size, cudaStream_t stream) {
    int threads = 256;
    int blocks  = (size + threads - 1) / threads;
    quantize_kernel<<<blocks, threads, 0, stream>>>(
        (const float*)w_f32, (signed char*)w_i8, size);
}

// WMMA forward: Y_f32[M,N] = X_f32[M,K] @ W_i8[N,K]^T * gamma
// Scratch buffers (allocated in Rust cuda_fwd):
//   x_i8_buf  : int8  [M*K]
//   scales_buf : float [M]
//   y_i32_buf  : int32 [M*N]
void ternary_gemm_forward(
    uint64_t x_f32,
    uint64_t w_i8,
    uint64_t y_f32,
    uint64_t x_i8_buf,
    uint64_t scales_buf,
    uint64_t y_i32_buf,
    int M, int N, int K,
    float gamma,
    cudaStream_t stream
) {
    // 1. Per-row abs-max of X
    row_absmax_kernel<<<M, 256, 256 * sizeof(float), stream>>>(
        (const float*)x_f32, (float*)scales_buf, M, K);

    // 2. Quantise X rows → INT8
    dim3 qblk(256);
    dim3 qgrd((K + 255) / 256, M);
    quantize_rows_kernel<<<qgrd, qblk, 0, stream>>>(
        (const float*)x_f32, (signed char*)x_i8_buf, (const float*)scales_buf, K);

    // 3. WMMA INT8 GEMM
    dim3 wblk(BLOCK_T);
    dim3 wgrd((N + BLOCK_N - 1) / BLOCK_N, (M + BLOCK_M - 1) / BLOCK_M);
    wmma_gemm_kernel<<<wgrd, wblk, 0, stream>>>(
        (const signed char*)x_i8_buf, (const signed char*)w_i8, (int32_t*)y_i32_buf,
        M, N, K);

    // 4. Dequantise → FP32
    dim3 dblk(256);
    dim3 dgrd((N + 255) / 256, M);
    dequantize_rows_kernel<<<dgrd, dblk, 0, stream>>>(
        (const int32_t*)y_i32_buf, (float*)y_f32, (const float*)scales_buf,
        gamma, N);
}

} // extern "C"
