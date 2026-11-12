use criterion::Bencher;
use v_html5escape::escape_string;

pub fn escaping(corpus: &'static str) -> impl FnMut(&mut Bencher) {
    move |b: &mut Bencher| {
        let mut buf = String::with_capacity(corpus.len());

        b.iter(|| {
            buf.clear();
            escape_string(corpus, &mut buf);
        });
    }
}
