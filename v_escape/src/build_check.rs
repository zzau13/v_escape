use version_check::is_min_version;

pub fn check_version() {
    if !is_min_version("1.27.0").map_or(false, |(yes, _)| yes) {
        println!("cargo:rustc-cfg=v_escape_nosimd");
    }
    // https://github.com/rust-lang/rust/issues/50154
    if !is_min_version("1.34.0").map_or(false, |(yes, _)| yes) {
        println!("cargo:rustc-cfg=v_escape_noavx");
    }
}
