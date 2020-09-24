use std::fmt::Write;

use criterion::Bencher;
use v_escape::new;

cfg_if::cfg_if! {
    if #[cfg(all(v_simd, v_avx))] {
        new!(
            MyEscape,
            "#0->zero || #1->one || #2->two || #3->three || #4->four || #5->five || \
             #6->six || #7->seven || #8->eight || #9->nine",
             avx = true
        );
    } else if #[cfg(all(v_simd, v_sse))] {
        new!(
            MyEscape,
            "#0->zero || #1->one || #2->two || #3->three || #4->four || #5->five || \
             #6->six || #7->seven || #8->eight || #9->nine",
             avx = false
        );
    } else {
        new!(
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
            let _ = write!(writer, "{}", e);
        });
    }
}
