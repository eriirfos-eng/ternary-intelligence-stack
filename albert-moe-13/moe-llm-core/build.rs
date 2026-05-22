fn main() {
    if std::env::var("CARGO_FEATURE_CUDA").is_err() {
        return;
    }

    println!("cargo:rerun-if-changed=cuda/ternary_gemm.cu");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let obj     = format!("{out_dir}/ternary_gemm.o");

    // Locate nvcc — check PATH first, then standard CUDA install paths.
    let nvcc = ["nvcc", "/usr/local/cuda/bin/nvcc"]
        .iter()
        .find(|&&p| std::process::Command::new(p).arg("--version").output().is_ok())
        .copied()
        .expect("nvcc not found — install CUDA toolkit or add it to PATH");

    // Compile ternary_gemm.cu.
    // SM 7.5 native for T4; PTX fallback for SM >= 7.5 (A10G, A100 are backward-compat).
    let status = std::process::Command::new(nvcc)
        .args([
            "cuda/ternary_gemm.cu",
            "-c", "-O3",
            "-gencode", "arch=compute_75,code=sm_75",
            "-gencode", "arch=compute_75,code=compute_75",
            "-o", &obj,
        ])
        .status()
        .expect("failed to spawn nvcc");
    assert!(status.success(), "nvcc: ternary_gemm.cu compilation failed");

    // Bundle into a static archive and link it.
    let lib = format!("{out_dir}/libternary_gemm.a");
    let status = std::process::Command::new("ar")
        .args(["crs", &lib, &obj])
        .status()
        .expect("ar failed");
    assert!(status.success(), "ar: failed to create libternary_gemm.a");

    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-lib=static=ternary_gemm");

    // CUDA runtime — prefer CUDA_HOME / CUDA_PATH env var, fall back to default path.
    let cuda_lib = std::env::var("CUDA_HOME")
        .or_else(|_| std::env::var("CUDA_PATH"))
        .map(|p| format!("{p}/lib64"))
        .unwrap_or_else(|_| "/usr/local/cuda/lib64".to_string());
    println!("cargo:rustc-link-search=native={cuda_lib}");
    println!("cargo:rustc-link-lib=cudart");
}
