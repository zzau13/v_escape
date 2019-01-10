use criterion::Bencher;
use std::fmt::Write;
use v_htmlescape::sized::HTMLEscape as Escape;

pub fn sized(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = Escape::new(corpus);

        b.iter(|| {
            e.size();
        });
    }
}

pub fn escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = Escape::new(corpus);
        let mut writer = String::new();

        b.iter(|| {
            write!(writer, "{}", e);
        });
    }
}
