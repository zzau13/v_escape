use criterion::Bencher;
use std::fmt::Write;

new_escape!(
    MyEscape,
    "#0->zero || #1->one || #2->two || #3->three || #4->four || #5->five || \
     #6->six || #7->seven || #8->eight || #9->nine"
);

pub fn escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = MyEscape::new(corpus);
        let mut writer = String::new();

        b.iter(|| {
            write!(writer, "{}", e);
        });
    }
}
