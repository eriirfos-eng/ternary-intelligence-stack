// wmma_test.cu — minimal WMMA INT8 fragment instantiation test for SM7.5 (T4)
//
// INT8 WMMA shapes for SM7.2+: 16x16x16, 8x32x16, 32x8x16.
// 8x8x32 is INT4 (subint8) — not INT8.  Previous failures used the wrong shape.
// Compile: nvcc --gpu-architecture sm_75 -c wmma_test.cu -o /dev/null

#include <cuda_runtime.h>
#include <mma.h>
#include <stdint.h>

using namespace nvcuda;

__global__ void wmma_fragment_instantiation_test() {
    // 16x16x16 INT8 — the correct shape for SM7.5 WMMA with signed char
    wmma::fragment<wmma::matrix_a,    16, 16, 16, signed char, wmma::row_major> a_frag;
    wmma::fragment<wmma::matrix_b,    16, 16, 16, signed char, wmma::col_major> b_frag;
    wmma::fragment<wmma::accumulator, 16, 16, 16, int>                          c_frag;
    wmma::fill_fragment(c_frag, 0);
    (void)a_frag; (void)b_frag;
}

extern "C" void wmma_test(cudaStream_t stream) {
    wmma_fragment_instantiation_test<<<1, 32, 0, stream>>>();
}
