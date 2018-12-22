use criterion::Bencher;
use std::fmt::Write;

use askama_escape::escape;

use std::str;

pub fn escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = escape(str::from_utf8(corpus).unwrap());
        let mut writer = String::new();

        b.iter(|| {
            write!(writer, "{}", e);
        });
    }
}
