use criterion::{criterion_group, criterion_main, Criterion};

mod common;
#[path = "v_jsonescape.rs"]
mod v_json;

fn functions(c: &mut Criterion) {
    common::register_cases!(c, "v_jsonescape/Escaping", v_json::escaping);
}

criterion_group!(benches, functions);
criterion_main!(benches);
