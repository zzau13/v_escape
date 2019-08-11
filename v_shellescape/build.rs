use std::env;

fn main() {
    if is_env_set("CARGO_CFG_SHELLESCAPE_DISABLE_AUTO_SIMD") {
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
