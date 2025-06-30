// For debugging, particularly in CI, print out the byte order
// and architecture of the current target.
#[test]
fn sys_info() {
    std::eprintln!();
    #[cfg(target_arch = "x86_64")]
    std::eprintln!("RUNNING ON x86_64");
    #[cfg(target_arch = "aarch64")]
    std::eprintln!("RUNNING ON aarch64");
    #[cfg(target_arch = "wasm32")]
    std::eprintln!("RUNNING ON wasm32");
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "wasm32"
    )))]
    std::eprintln!("RUNNING ON Unknown architecture");

    #[cfg(target_endian = "little")]
    std::eprintln!("LITTLE ENDIAN");
    #[cfg(target_endian = "big")]
    std::eprintln!("BIG ENDIAN");
}
