fn main() {
    treecr_cuda();
}

fn treecr_cuda() {
    let mut nvcc = cc::Build::new();
    nvcc.cuda(true);
    nvcc.flag("-O2").flag("-D__ADX__").flag("-arch=sm_70").flag("-Xcompiler").flag("-Wno-subobject-linkage").flag("-Xcompiler").flag("-O3");
    if let Some(include) = std::env::var_os("DEP_BLST_C_SRC") {
        nvcc.include(&include);
    }
    if let Some(include) = std::env::var_os("DEP_SPPARK_ROOT") {
        nvcc.include(include);
    }
    nvcc.flag("-Xcompiler").flag("-Wno-subobject-linkage");
    nvcc.flag("-Xcompiler").flag("-Wno-unused-function");

    println!("cargo:rustc-link-arg=-lblst");
    println!("cargo:rustc-link-arg=-lpc2");
    println!("cargo:rerun-if-changed=cuda");
    println!("cargo:rerun-if-env-changed=CXXFLAGS");
}
