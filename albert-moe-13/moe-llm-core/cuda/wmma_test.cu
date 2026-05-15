// wmma_test.cu — WMMA INT8 fragment + fused kernel smem launch test for SM7.5 (T4)
//
// INT8 WMMA shapes for SM7.2+: 16x16x16, 8x32x16, 32x8x16.
// Compile: nvcc --gpu-architecture sm_75 -c wmma_test.cu -o /dev/null

#include <cuda_runtime.h>
#include <mma.h>
#include <stdint.h>

using namespace nvcuda;

#define WMMA_M 16
#define WMMA_N 16
#define WMMA_K 16

__global__ void wmma_fragment_instantiation_test() {
    wmma::fragment<wmma::matrix_a,    16, 16, 16, signed char, wmma::row_major> a_frag;
    wmma::fragment<wmma::matrix_b,    16, 16, 16, signed char, wmma::col_major> b_frag;
    wmma::fragment<wmma::accumulator, 16, 16, 16, int>                          c_frag;
    wmma::fill_fragment(c_frag, 0);
    (void)a_frag; (void)b_frag;
}

// Smoke-test the fused kernel smem layout and launch config (K=256, no actual compute)
__global__ void fused_smem_test() {
    extern __shared__ char smem[];
    float*       Xf = (float*)smem;
    signed char* Xi = (signed char*)(Xf + WMMA_M * 256);
    (void)Xf; (void)Xi;
}

extern "C" void wmma_test(cudaStream_t stream) {
    wmma_fragment_instantiation_test<<<1, 32, 0, stream>>>();
    // smem: 16*256*(4+1) = 20480 bytes
    fused_smem_test<<<1, 32, 20480, stream>>>();
}
