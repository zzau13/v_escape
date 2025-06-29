use criterion::Bencher;
use v_htmlescape::escape_string;

pub fn escaping(corpus: &str) -> impl FnMut(&mut Bencher) {
    move |b: &mut Bencher| {
        let mut buf = String::with_capacity(corpus.len());

        b.iter(|| escape_string(corpus, &mut buf));
    }
}
