// cuda_kernel.rs — TernaryGemmOp: WMMA INT8 forward, F32 backward via candle CustomOp2.
//
// Forward: per-row quantise X_f32 → X_i8, WMMA INT8 GEMM (tensor cores), dequantise → Y_f32.
// Backward: standard f32 matmul (candle autograd) — STE gradient path unchanged.
//
// Only compiled when the `cuda` feature is enabled.

#![cfg(feature = "cuda")]

use candle_core::{CpuStorage, Layout, Result, Shape, Tensor};
use candle_core::cuda_backend::{CudaStorage, CudaStorageSlice, WrapErr};
use candle_core::cuda_backend::cudarc::driver::{DevicePtr, DeviceSlice as _};

type CUstream = *mut std::ffi::c_void;

unsafe extern "C" {
    fn ternary_quantize(w_f32: u64, w_i8: u64, size: i32, stream: CUstream);

    fn ternary_gemm_forward(
        x_f32:      u64,
        w_i8:       u64,
        y_f32:      u64,
        x_i8_buf:   u64,
        scales_buf: u64,
        y_i32_buf:  u64,
        m: i32, n: i32, k: i32,
        gamma: f32,
        stream: CUstream,
    );
}

pub struct TernaryGemmOp {
    pub gamma: f32,
    pub m: usize,
    pub n: usize,
    pub k: usize,
}

impl candle_core::CustomOp2 for TernaryGemmOp {
    fn name(&self) -> &'static str { "ternary_gemm" }

    // CPU fallback: plain f32 matmul — identical numerics to candle's standard path.
    fn cpu_fwd(
        &self,
        xs: &CpuStorage, _xl: &Layout,
        ws: &CpuStorage, _wl: &Layout,
    ) -> Result<(CpuStorage, Shape)> {
        let x = xs.as_slice::<f32>()?;
        let w = ws.as_slice::<f32>()?;
        let (m, n, k) = (self.m, self.n, self.k);
        let mut y = vec![0.0f32; m * n];
        for i in 0..m {
            for j in 0..n {
                let mut acc = 0.0f32;
                for p in 0..k { acc += x[i*k + p] * w[j*k + p]; }
                y[i*n + j] = acc;
            }
        }
        Ok((CpuStorage::F32(y), Shape::from_dims(&[m, n])))
    }

    // CUDA forward: WMMA INT8 16×16×16 tensor-core GEMM (SM7.2+).
    // Inputs: x [M,K] f32,  w_ternary [N,K] f32 ({-γ,0,+γ})
    // Output: y [M,N] f32
    fn cuda_fwd(
        &self,
        xs: &CudaStorage, xl: &Layout,
        ws: &CudaStorage, wl: &Layout,
    ) -> Result<(CudaStorage, Shape)> {
        let dev = xs.device.clone();
        let (m, n, k) = (self.m as i32, self.n as i32, self.k as i32);

        // M_pad: M rounded up to WMMA tile (16) — scratch buffers must cover padded rows.
        let m_pad = ((self.m + 15) / 16) * 16;

        let x_slice = xs.as_cuda_slice::<f32>()?;
        let w_slice = ws.as_cuda_slice::<f32>()?;

        let x_off = xl.contiguous_offsets()
            .ok_or_else(|| candle_core::Error::Msg("TernaryGemm: x not contiguous".into()))?.0;
        let w_off = wl.contiguous_offsets()
            .ok_or_else(|| candle_core::Error::Msg("TernaryGemm: w not contiguous".into()))?.0;

        let x_ptr:     u64 = *x_slice.slice(x_off..).device_ptr();
        let w_f32_ptr: u64 = *w_slice.slice(w_off..).device_ptr();

        // Quantise W: f32 {-γ,0,+γ} → int8 {-1,0,+1}
        let w_i8_buf = unsafe { dev.alloc::<u8>((n * k) as usize) }.w()?;
        let w_i8_ptr: u64 = *w_i8_buf.device_ptr();

        // Scratch: X_i8 [M_pad,K], per-row scales [M], Y_i32 [M_pad,N]
        let x_i8_buf   = unsafe { dev.alloc::<u8>(m_pad * self.k) }.w()?;
        let scales_buf = unsafe { dev.alloc::<f32>(self.m) }.w()?;
        let y_i32_buf  = unsafe { dev.alloc::<u32>(m_pad * self.n) }.w()?;

        // Output: Y_f32 [M,N]
        let y_buf = unsafe { dev.alloc::<f32>((m * n) as usize) }.w()?;
        let y_ptr: u64 = *y_buf.device_ptr();

        let stream: CUstream = std::ptr::null_mut();

        unsafe {
            ternary_quantize(w_f32_ptr, w_i8_ptr, n * k, stream);
            ternary_gemm_forward(
                x_ptr, w_i8_ptr, y_ptr,
                *x_i8_buf.device_ptr(),
                *scales_buf.device_ptr(),
                *y_i32_buf.device_ptr(),
                m, n, k,
                self.gamma,
                stream,
            );
        }

        Ok((
            CudaStorage { slice: CudaStorageSlice::F32(y_buf), device: dev },
            Shape::from_dims(&[self.m, self.n]),
        ))
    }

    // Backward: standard f32 matmul — STE gradient path.
    fn bwd(
        &self,
        x: &Tensor,
        w: &Tensor,
        _res: &Tensor,
        grad: &Tensor,
    ) -> Result<(Option<Tensor>, Option<Tensor>)> {
        let dx = grad.matmul(w)?;
        let dw = grad.t()?.contiguous()?.matmul(x)?;
        Ok((Some(dx), Some(dw)))
    }
}
