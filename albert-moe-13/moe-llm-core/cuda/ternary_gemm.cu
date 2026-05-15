// ternary_gemm.cu — ternary weight quantisation kernel + GEMM stub
//
// ternary_quantize: f32 {-γ,0,+γ} → int8 {-1,0,+1}  (used by sparse-cache path)
// ternary_gemm_forward: stub — actual GEMM handled by candle cuBLAS matmul path
//
// WMMA INT8 (8×8×32) was attempted but wmma::fragment for INT8 fails to
// instantiate on the CUDA 12.1 / SM7.5 Modal container ("incomplete type").
// Root cause needs an interactive nvcc shell to diagnose properly.
// Deferred post-SPRIND. cuBLAS F32 path is used in the interim.

#include <cuda_runtime.h>
#include <stdint.h>

__global__ void quantize_kernel(const float* w_f32, signed char* w_i8, int size) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= size) return;
    float v = w_f32[idx];
    w_i8[idx] = (v > 0.f) ? (signed char)1 : ((v < 0.f) ? (signed char)-1 : (signed char)0);
}

extern "C" {

void ternary_quantize(uint64_t w_f32, uint64_t w_i8, int size, cudaStream_t stream) {
    int threads = 256, blocks = (size + 255) / 256;
    quantize_kernel<<<blocks, threads, 0, stream>>>(
        (const float*)w_f32, (signed char*)w_i8, size);
}

// Stub — not called. Actual matmul done by candle cuBLAS via ternary_linear.rs.
void ternary_gemm_forward(
    uint64_t x_f32, uint64_t w_i8, uint64_t y_f32,
    uint64_t x_i8_buf, uint64_t scales_buf, uint64_t y_i32_buf,
    int M, int N, int K, float gamma, cudaStream_t stream
) { (void)x_f32; (void)w_i8; (void)y_f32; (void)x_i8_buf;
    (void)scales_buf; (void)y_i32_buf; (void)M; (void)N; (void)K;
    (void)gamma; (void)stream; }

} // extern "C"
