use version_check::{is_min_version, is_nightly};

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
    if !is_min_version("1.27.0").map_or(false, |(yes, _)| yes) {
        println!("cargo:rustc-cfg=v_escape_nosimd");
    }
}
