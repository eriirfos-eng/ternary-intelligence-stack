// wmma_test.cu — minimal WMMA INT8 fragment instantiation test for SM7.5 (T4)
//
// Confirms that #include <mma.h> + using namespace nvcuda resolves the
// "incomplete type" error seen when only cuda_runtime.h was included.
// Compile: nvcc --gpu-architecture sm_75 -c wmma_test.cu -o /dev/null

#include <cuda_runtime.h>
#include <mma.h>
#include <stdint.h>

using namespace nvcuda;

__global__ void wmma_fragment_instantiation_test() {
    wmma::fragment<wmma::matrix_a,    8, 8, 32, signed char, wmma::row_major> a_frag;
    wmma::fragment<wmma::matrix_b,    8, 8, 32, signed char, wmma::col_major> b_frag;
    wmma::fragment<wmma::accumulator, 8, 8, 32, int>                          c_frag;
    wmma::fill_fragment(c_frag, 0);
    (void)a_frag; (void)b_frag;
}

extern "C" void wmma_test(cudaStream_t stream) {
    wmma_fragment_instantiation_test<<<1, 32, 0, stream>>>();
}
