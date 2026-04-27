use criterion::{criterion_group, criterion_main, Criterion};

mod common;
#[path = "v_latexescape.rs"]
mod v_latex;

fn functions(c: &mut Criterion) {
    common::register_cases!(c, "v_latexescape/Escaping", v_latex::escaping);
}

criterion_group!(benches, functions);
criterion_main!(benches);
