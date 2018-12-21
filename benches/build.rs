use version_check::is_nightly;

fn main() {
    if is_nightly().unwrap_or(false) {
        println!("cargo:rustc-cfg=v_escape_benches_nightly");
    }
}
