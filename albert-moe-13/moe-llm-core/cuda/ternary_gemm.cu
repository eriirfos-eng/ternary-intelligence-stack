// ternary_gemm.cu — ternary weight quantisation + WMMA INT8 GEMM
//
// ternary_quantize:     f32 {-γ,0,+γ} → int8 {-1,0,+1}  (sparse-cache path)
// ternary_gemm_forward: X_f32[M,K] × W_i8[N,K]^T → Y_f32[M,N]
//
//   Step 1: per-row abs-max quantise X_f32 → X_i8, store per-row scale
//   Step 2: WMMA INT8 GEMM (8×8×32 tiles, SM7.5+)  Y_i32 = X_i8 @ W_i8^T
//   Step 3: dequantise  Y_f32[m,n] = Y_i32[m,n] * x_scale[m] * gamma
//
// W is stored [N,K] row-major; treated as [K,N] col-major in WMMA (= W^T).
// K=256, N=256 for albert. — both are tile-aligned.  M is padded to ×8 in
// ternary_gemm_forward before the kernel is launched.
//
// Root cause of previous "incomplete type" failures: only cuda_runtime.h was
// included.  WMMA specialisations live in mma.h and require nvcuda namespace.

#include <cuda_runtime.h>
#include <mma.h>
#include <stdint.h>

using namespace nvcuda;

// ---------------------------------------------------------------------------
// Tile constants for WMMA 8×8×32 INT8 (SM7.5+)
// ---------------------------------------------------------------------------
#define WMMA_M 8
#define WMMA_N 8
#define WMMA_K 32

// ---------------------------------------------------------------------------
// Kernel 1: per-row abs-max quantise X_f32 → X_i8, store inverse scale
// ---------------------------------------------------------------------------
__global__ void quantize_x_kernel(
    const float*  __restrict__ x_f32,  // [M, K]
    signed char*  __restrict__ x_i8,   // [M, K]
    float*        __restrict__ scales, // [M]  (= amax / 127 — dequant multiplier)
    int M, int K
) {
    int row = blockIdx.x * blockDim.x + threadIdx.x;
    if (row >= M) return;

    const float* src = x_f32 + row * K;
    signed char* dst = x_i8  + row * K;

    float amax = 0.f;
    for (int k = 0; k < K; ++k) {
        float v = src[k];
        if (v < 0.f) v = -v;
        if (v > amax) amax = v;
    }

    float inv_scale = (amax > 0.f) ? (127.f / amax) : 1.f;
    scales[row] = (amax > 0.f) ? (amax / 127.f) : 1.f;

    for (int k = 0; k < K; ++k) {
        float q = src[k] * inv_scale;
        // clamp and round
        if      (q >  127.f) q =  127.f;
        else if (q < -127.f) q = -127.f;
        dst[k] = (signed char)__float2int_rn(q);
    }
}

// ---------------------------------------------------------------------------
// Kernel 2: WMMA INT8 GEMM — Y_i32[M,N] = X_i8[M,K] @ W_i8[N,K]^T
//
// Each block = one warp (32 threads) = one [WMMA_M × WMMA_N] output tile.
// Grid: (ceil(N/WMMA_N), ceil(M/WMMA_M))   — M must be padded to × WMMA_M.
//
// W is [N,K] row-major.  WMMA col_major matrix_b interprets a [K,N] memory
// layout as col-major, where element [k,n] lives at ptr[n*K + k].
// W[n,k] = w_ptr[n*K + k] — same address — so W can be read directly as
// a col-major B fragment without transposing.
// ---------------------------------------------------------------------------
__global__ void wmma_int8_gemm_kernel(
    const signed char* __restrict__ X,  // [M_pad, K]
    const signed char* __restrict__ W,  // [N,     K]
    int*               __restrict__ Y,  // [M_pad, N]
    int M_pad, int N, int K
) {
    int tile_m = blockIdx.y;
    int tile_n = blockIdx.x;

    wmma::fragment<wmma::accumulator, WMMA_M, WMMA_N, WMMA_K, int> c_frag;
    wmma::fill_fragment(c_frag, 0);

    for (int k = 0; k < K; k += WMMA_K) {
        wmma::fragment<wmma::matrix_a, WMMA_M, WMMA_N, WMMA_K, signed char, wmma::row_major> a_frag;
        wmma::fragment<wmma::matrix_b, WMMA_M, WMMA_N, WMMA_K, signed char, wmma::col_major> b_frag;

        // A tile: X[tile_m*8 : tile_m*8+8, k : k+32], stride = K
        wmma::load_matrix_sync(a_frag, X + tile_m * WMMA_M * K + k, K);

        // B tile (col_major W^T): W[tile_n*8 : tile_n*8+8, k : k+32], stride = K
        wmma::load_matrix_sync(b_frag, W + tile_n * WMMA_N * K + k, K);

        wmma::mma_sync(c_frag, a_frag, b_frag, c_frag);
    }

    wmma::store_matrix_sync(Y + tile_m * WMMA_M * N + tile_n * WMMA_N, c_frag, N,
                            wmma::mem_row_major);
}

// ---------------------------------------------------------------------------
// Kernel 3: dequantise — Y_f32[m,n] = Y_i32[m,n] * x_scale[m] * gamma
// ---------------------------------------------------------------------------
__global__ void dequantize_kernel(
    const int*   __restrict__ y_i32,   // [M_pad, N]
    const float* __restrict__ x_scale, // [M]
    float*       __restrict__ y_f32,   // [M, N]
    int M, int N, int M_pad,
    float gamma
) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= M * N) return;
    int m = idx / N;
    int n = idx % N;
    y_f32[idx] = (float)y_i32[m * N + n] * x_scale[m] * gamma;
    (void)M_pad;
}

// ---------------------------------------------------------------------------
// quantize_kernel — kept for the sparse-cache path (unchanged)
// ---------------------------------------------------------------------------
__global__ void quantize_kernel(const float* w_f32, signed char* w_i8, int size) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= size) return;
    float v = w_f32[idx];
    w_i8[idx] = (v > 0.f) ? (signed char)1 : ((v < 0.f) ? (signed char)-1 : (signed char)0);
}

// ---------------------------------------------------------------------------
// C API
// ---------------------------------------------------------------------------
extern "C" {

void ternary_quantize(uint64_t w_f32, uint64_t w_i8, int size, cudaStream_t stream) {
    int threads = 256, blocks = (size + 255) / 256;
    quantize_kernel<<<blocks, threads, 0, stream>>>(
        (const float*)w_f32, (signed char*)w_i8, size);
}

// x_i8_buf:   caller-allocated [M_pad * K] int8  scratch
// scales_buf: caller-allocated [M]         f32   scratch
// y_i32_buf:  caller-allocated [M_pad * N] int32 scratch
// M_pad:      M rounded up to next multiple of WMMA_M (8) — caller must pass this
//             and allocate buffers accordingly.  y_f32 is [M * N] (not padded).
void ternary_gemm_forward(
    uint64_t x_f32,      // [M,     K] f32  input activations
    uint64_t w_i8,       // [N,     K] i8   ternary weights {-1,0,+1}
    uint64_t y_f32,      // [M,     N] f32  output
    uint64_t x_i8_buf,   // [M_pad, K] i8   scratch: quantised X
    uint64_t scales_buf, // [M]        f32  scratch: per-row dequant scale
    uint64_t y_i32_buf,  // [M_pad, N] i32  scratch: WMMA accumulator output
    int M, int N, int K, float gamma, cudaStream_t stream
) {
    int M_pad = ((M + WMMA_M - 1) / WMMA_M) * WMMA_M;

    // Step 1: per-row quantise X_f32 → X_i8
    // Zero the padding rows so WMMA reads 0 for any m in [M, M_pad).
    if (M_pad > M) {
        cudaMemsetAsync((signed char*)x_i8_buf + M * K, 0,
                        (size_t)(M_pad - M) * K, stream);
    }
    {
        int threads = 128, blocks = (M + 127) / 128;
        quantize_x_kernel<<<blocks, threads, 0, stream>>>(
            (const float*)x_f32,
            (signed char*)x_i8_buf,
            (float*)scales_buf,
            M, K);
    }

    // Step 2: WMMA INT8 GEMM
    {
        dim3 grid(N / WMMA_N, M_pad / WMMA_M);
        wmma_int8_gemm_kernel<<<grid, 32, 0, stream>>>(
            (const signed char*)x_i8_buf,
            (const signed char*)w_i8,
            (int*)y_i32_buf,
            M_pad, N, K);
    }

    // Step 3: dequantise
    {
        int total = M * N;
        int threads = 256, blocks = (total + 255) / 256;
        dequantize_kernel<<<blocks, threads, 0, stream>>>(
            (const int*)y_i32_buf,
            (const float*)scales_buf,
            (float*)y_f32,
            M, N, M_pad, gamma);
    }
}

} // extern "C"
