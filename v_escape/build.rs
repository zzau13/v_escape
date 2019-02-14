#[path = "src/build_check.rs"]
mod build_check;

fn main() {
    crate::build_check::check_version();
}
