use std::env;

use version_check::is_min_version;

fn main() {
    enable_simd_optimizations();
}

fn enable_simd_optimizations() {
    if is_env_set("CARGO_CFG_SHELLESCAPE_DISABLE_AUTO_SIMD")
        || !is_min_version("1.27.0").map_or(false, |(yes, _)| yes)
    {
        println!("cargo:rustc-cfg=v_escape_nosimd");
        return;
    }

    println!("cargo:rustc-cfg=v_shellescape_simd");
    println!("cargo:rustc-cfg=v_shellescape_sse");

    if !is_env_set("CARGO_CFG_SHELLESCAPE_DISABLE_AUTO_AVX") {
        println!("cargo:rustc-cfg=v_shellescape_avx");
    }
}

fn is_env_set(name: &str) -> bool {
    env::var(name).is_ok()
}
