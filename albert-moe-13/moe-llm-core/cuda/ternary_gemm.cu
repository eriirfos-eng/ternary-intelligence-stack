// ternary_gemm.cu — ternary weight quantisation + fused WMMA INT8 GEMM
//
// ternary_quantize:          f32 {-γ,0,+γ} → int8 {-1,0,+1}  (sparse-cache path)
// wmma_int8_fused_kernel:    single-kernel path — fuses quantize_x + WMMA + dequantize
// ternary_gemm_forward:      public entry point (calls fused kernel)
//
// Fused kernel per-block (one warp = 32 threads, one [16×16] output tile):
//   Shared memory layout  (20KB per block, well within T4's 64KB/SM):
//     [0, 16KB)   Xf  — X tile as float32   [WMMA_M=16][K=256]
//     [16KB, 20KB) Xi — X tile as int8      [WMMA_M=16][K=256]
//
//   1. Load X_f32 tile from global → Xf smem  (1 global read, no write)
//   2. Warp-shuffle per-row abs-max → scale[16] in registers (all threads identical)
//   3. Quantise Xf → Xi in smem                (smem only, no global write)
//   4. K-loop WMMA:  a_frag ← Xi smem          (smem read, fast)
//                    b_frag ← W_i8 global       (one read, already int8)
//                    mma_sync accumulate
//   5. Store int32 accumulator to Xf smem (reused), dequantise → Y_f32 global  (1 write)
//
// Eliminates vs the previous 3-kernel pipeline:
//   - M*K global write + read for X_i8
//   - M*N global write + read for Y_i32
//   - M global write + read for per-row scales
//   - 2 extra kernel launches

#include <cuda_runtime.h>
#include <mma.h>
#include <stdint.h>

using namespace nvcuda;

#define WMMA_M 16
#define WMMA_N 16
#define WMMA_K 16

// ---------------------------------------------------------------------------
// ternary_quantize — sign-only quantise for sparse-cache path (unchanged)
// ---------------------------------------------------------------------------
__global__ void quantize_kernel(const float* w_f32, signed char* w_i8, int size) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= size) return;
    float v = w_f32[idx];
    w_i8[idx] = (v > 0.f) ? (signed char)1 : ((v < 0.f) ? (signed char)-1 : (signed char)0);
}

// ---------------------------------------------------------------------------
// wmma_int8_fused_kernel
//
// Grid  : (N/WMMA_N, M_pad/WMMA_M)   — one block per [16×16] output tile
// Block : 32 threads (one warp)
// Smem  : WMMA_M * K * (sizeof(float) + sizeof(signed char))  = 20KB for K=256
//
// W_i8 is [N, K] row-major and is treated as [K, N] col-major in WMMA (= W^T).
// K and N must both be multiples of 16.  M is padded to a multiple of 16 by the
// caller (extra rows zeroed → contribute zero to the output, safe to ignore).
// ---------------------------------------------------------------------------
__global__ void wmma_int8_fused_kernel(
    const float*       __restrict__ X_f32,  // [M_pad, K]  f32  input
    const signed char* __restrict__ W_i8,   // [N,     K]  i8   ternary {-1,0,+1}
    float*             __restrict__ Y_f32,  // [M,     N]  f32  output
    int M, int N, int K,
    float gamma
) {
    const int tile_m = blockIdx.y;
    const int tile_n = blockIdx.x;
    const int tid    = threadIdx.x;  // 0..31

    extern __shared__ char smem[];
    float*       Xf = (float*)smem;                      // [WMMA_M][K] f32
    signed char* Xi = (signed char*)(Xf + WMMA_M * K);  // [WMMA_M][K] i8

    // -----------------------------------------------------------------------
    // 1. Load X_f32 tile into shared memory.
    //    Each thread covers K/32 consecutive columns for all WMMA_M rows.
    //    Rows where global_row >= M are zeroed (padding guard).
    // -----------------------------------------------------------------------
    const int CPT = K / 32;  // columns per thread — 8 for K=256
    for (int row = 0; row < WMMA_M; ++row) {
        int grow = tile_m * WMMA_M + row;
        for (int c = 0; c < CPT; ++c) {
            int col = tid * CPT + c;
            Xf[row * K + col] = (grow < M) ? X_f32[grow * K + col] : 0.f;
        }
    }
    __syncwarp();

    // -----------------------------------------------------------------------
    // 2. Per-row abs-max → quantisation scales.
    //    Each thread computes a partial max over its CPT columns, then a
    //    warp-level __shfl_xor_sync all-reduce gives every thread the global
    //    per-row max.  Result: row_scale[row] and row_dequant[row] in regs.
    // -----------------------------------------------------------------------
    float row_scale[WMMA_M], row_dequant[WMMA_M];
    for (int row = 0; row < WMMA_M; ++row) {
        float lmax = 0.f;
        for (int c = 0; c < CPT; ++c)
            lmax = fmaxf(lmax, fabsf(Xf[row * K + tid * CPT + c]));
        for (int mask = 16; mask > 0; mask >>= 1)
            lmax = fmaxf(lmax, __shfl_xor_sync(0xffffffffu, lmax, mask));
        row_scale[row]   = (lmax > 0.f) ? (127.f / lmax) : 1.f;
        row_dequant[row] = (lmax > 0.f) ? (lmax / 127.f) : 1.f;
    }

    // -----------------------------------------------------------------------
    // 3. Quantise Xf → Xi (smem only — no global traffic).
    // -----------------------------------------------------------------------
    for (int row = 0; row < WMMA_M; ++row) {
        for (int c = 0; c < CPT; ++c) {
            int col = tid * CPT + c;
            float q = Xf[row * K + col] * row_scale[row];
            Xi[row * K + col] = (signed char)__float2int_rn(fmaxf(-127.f, fminf(127.f, q)));
        }
    }
    __syncwarp();

    // -----------------------------------------------------------------------
    // 4. WMMA INT8 16×16×16 accumulation over K.
    //    A = Xi (smem, row-major):     X_i8[tile_m*16 : +16, k : k+16]
    //    B = W_i8 (global, col-major): W_i8[tile_n*16 : +16, k : k+16]
    //    C = int32 accumulator [WMMA_M × WMMA_N]
    // -----------------------------------------------------------------------
    wmma::fragment<wmma::accumulator, WMMA_M, WMMA_N, WMMA_K, int> c_frag;
    wmma::fill_fragment(c_frag, 0);

    for (int k = 0; k < K; k += WMMA_K) {
        wmma::fragment<wmma::matrix_a, WMMA_M, WMMA_N, WMMA_K, signed char, wmma::row_major> a_frag;
        wmma::fragment<wmma::matrix_b, WMMA_M, WMMA_N, WMMA_K, signed char, wmma::col_major> b_frag;
        wmma::load_matrix_sync(a_frag, Xi + k,                          K);  // smem, stride=K
        wmma::load_matrix_sync(b_frag, W_i8 + tile_n * WMMA_N * K + k, K);  // global, stride=K
        wmma::mma_sync(c_frag, a_frag, b_frag, c_frag);
    }

    // -----------------------------------------------------------------------
    // 5. Store int32 accumulator → reused Xf smem, then dequantise → Y_f32.
    //    Xf is 16KB; the accumulator is 16×16×4 = 1KB — safe to alias.
    //    Each thread writes WMMA_M*WMMA_N/32 = 8 output elements.
    // -----------------------------------------------------------------------
    int* Ys = (int*)Xf;
    wmma::store_matrix_sync(Ys, c_frag, WMMA_N, wmma::mem_row_major);
    __syncwarp();

    for (int i = 0; i < WMMA_M * WMMA_N / 32; ++i) {
        int idx  = tid + i * 32;
        int row  = idx / WMMA_N;
        int col  = idx % WMMA_N;
        int grow = tile_m * WMMA_M + row;
        int gcol = tile_n * WMMA_N + col;
        if (grow < M && gcol < N)
            Y_f32[grow * N + gcol] = (float)Ys[idx] * row_dequant[row] * gamma;
    }
}

// ---------------------------------------------------------------------------
// C API
// ---------------------------------------------------------------------------
extern "C" {

void ternary_quantize(uint64_t w_f32, uint64_t w_i8, int size, cudaStream_t stream) {
    quantize_kernel<<<(size + 255) / 256, 256, 0, stream>>>(
        (const float*)w_f32, (signed char*)w_i8, size);
}

// Scratch buffers (x_i8_buf, scales_buf, y_i32_buf) were used by the old
// 3-kernel pipeline and are still allocated by cuda_kernel.rs.  The fused
// kernel uses smem instead so they are ignored here.
void ternary_gemm_forward(
    uint64_t x_f32,      // [M,     K] f32  input activations
    uint64_t w_i8,       // [N,     K] i8   ternary weights {-1,0,+1}
    uint64_t y_f32,      // [M,     N] f32  output
    uint64_t x_i8_buf,   // unused (kept for ABI compatibility)
    uint64_t scales_buf, // unused
    uint64_t y_i32_buf,  // unused
    int M, int N, int K, float gamma, cudaStream_t stream
) {
    (void)x_i8_buf; (void)scales_buf; (void)y_i32_buf;

    int M_pad = ((M + WMMA_M - 1) / WMMA_M) * WMMA_M;

    // smem: Xf [WMMA_M][K] f32 + Xi [WMMA_M][K] i8
    size_t smem_bytes = (size_t)WMMA_M * K * (sizeof(float) + sizeof(signed char));

    dim3 grid(N / WMMA_N, M_pad / WMMA_M);
    wmma_int8_fused_kernel<<<grid, 32, smem_bytes, stream>>>(
        (const float*)x_f32,
        (const signed char*)w_i8,
        (float*)y_f32,
        M, N, K, gamma);
}

} // extern "C"
