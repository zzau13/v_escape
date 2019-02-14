use std::env;
use v_escape::check_version;
use version_check::is_nightly;

fn main() {
    enable_nightly();
    enable_simd_optimizations();
}

fn enable_nightly() {
    if is_nightly().unwrap_or(false) {
        println!("cargo:rustc-cfg=v_escape_benches_nightly");
    }
}

fn enable_simd_optimizations() {
    check_version();
    if is_env_set("CARGO_CFG_HTMLESCAPE_DISABLE_AUTO_SIMD") {
        return;
    }

    println!("cargo:rustc-cfg=v_simd");
    println!("cargo:rustc-cfg=v_sse");

    if !is_env_set("CARGO_CFG_HTMLESCAPE_DISABLE_AUTO_AVX") {
        println!("cargo:rustc-cfg=v_avx");
    }
}

fn is_env_set(name: &str) -> bool {
    env::var(name).is_ok()
}
