use criterion::{Criterion, criterion_group, criterion_main};

mod common;
#[path = "v_html5escape.rs"]
mod v_html5;

fn functions(c: &mut Criterion) {
    common::register_cases!(c, "v_html5escape/Escaping", v_html5::escaping);
}

criterion_group!(benches, functions);
criterion_main!(benches);
