use version_check::is_min_version;

fn main() {
    enable_simd_optimizations();
}

fn enable_simd_optimizations() {
    // TODO: ERROR when attribute is in macro used in another crate
    if !is_min_version("1.27.0").map_or(false, |(yes, _)| yes) {
        println!("cargo:rustc-cfg=v_escape_nosimd");
    }
}
