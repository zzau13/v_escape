use std::env;

use version_check::is_min_version;

fn main() {
    enable_simd_optimizations();
}

fn enable_simd_optimizations() {
    if is_env_set("CARGO_CFG_HTMLESCAPE_DISABLE_AUTO_SIMD") {
        println!("cargo:rustc-cfg=v_escape_nosimd");
        return;
    }
    if !is_min_version("1.27.0")
        .map(|(yes, _)| yes)
        .unwrap_or(false)
    {
        println!("cargo:rustc-cfg=v_escape_nosimd");
        return;
    }

    println!("cargo:rustc-cfg=v_htmlescape_simd");
    println!("cargo:rustc-cfg=v_htmlescape_sse");

    if is_env_set("CARGO_CFG_HTMLESCAPE_DISABLE_AUTO_AVX") {
        return;
    }

    println!("cargo:rustc-cfg=v_htmlescape_avx");
}

fn is_env_set(name: &str) -> bool {
    env::var(name).is_ok()
}
