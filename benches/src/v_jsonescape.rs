use criterion::Bencher;
use v_jsonescape::b_escape;

pub fn escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let mut writer = String::with_capacity(corpus.len());

        b.iter(|| b_escape(corpus, &mut writer));
    }
}
