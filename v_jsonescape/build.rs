fn main() {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if is_x86_feature_detected!("sse2") {
            println!("cargo:rustc-cfg=v_escape_sse");
        }
        if is_x86_feature_detected!("avx2") {
            println!("cargo:rustc-cfg=v_escape_avx");
        }
    }
}
