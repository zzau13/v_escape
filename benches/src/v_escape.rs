use criterion::Bencher;
use std::fmt::Write;

cfg_if! {
    if #[cfg(all(v_simd, v_avx))] {
        new_escape!(
            MyEscape,
            "#0->zero || #1->one || #2->two || #3->three || #4->four || #5->five || \
             #6->six || #7->seven || #8->eight || #9->nine",
             avx = false
        );
    } else if #[cfg(all(v_simd, v_sse))] {
        new_escape!(
            MyEscape,
            "#0->zero || #1->one || #2->two || #3->three || #4->four || #5->five || \
             #6->six || #7->seven || #8->eight || #9->nine",
             avx = false
        );
    } else {
        new_escape!(
            MyEscape,
            "#0->zero || #1->one || #2->two || #3->three || #4->four || #5->five || \
             #6->six || #7->seven || #8->eight || #9->nine",
             simd = false
        );
    }
}

pub fn escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = MyEscape::new(corpus);
        let mut writer = String::new();

        b.iter(|| {
            write!(writer, "{}", e);
        });
    }
}
