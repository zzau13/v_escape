use std::env;

fn main() {
    enable_simd_optimizations();
}

fn enable_simd_optimizations() {
    println!("cargo:rerun-if-env-changed=CARGO_CFG_JSONESCAPE_DISABLE_AUTO_SIMD");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_JSONESCAPE_DISABLE_AUTO_AVX");
    if is_env_set("CARGO_CFG_JSONESCAPE_DISABLE_AUTO_SIMD") {
        return;
    }

    println!("cargo:rustc-cfg=v_jsonescape_simd");
    println!("cargo:rustc-cfg=v_jsonescape_sse");

    if !is_env_set("CARGO_CFG_JSONESCAPE_DISABLE_AUTO_AVX") {
        println!("cargo:rustc-cfg=v_jsonescape_avx");
    }
}

fn is_env_set(name: &str) -> bool {
    env::var(name).is_ok()
}
