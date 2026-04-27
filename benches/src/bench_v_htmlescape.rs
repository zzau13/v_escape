use criterion::{Criterion, criterion_group, criterion_main};

mod common;
#[path = "v_htmlescape.rs"]
mod v_html;

fn functions(c: &mut Criterion) {
    common::register_cases!(c, "v_htmlescape/Escaping", v_html::escaping);
}

criterion_group!(benches, functions);
criterion_main!(benches);
