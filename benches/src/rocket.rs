use criterion::Bencher;
use rocket::http::RawStr;
use std::{fmt::Write, str};

pub fn escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = RawStr::from_str(str::from_utf8(corpus).unwrap());
        let mut writer = String::new();

        b.iter(|| {
            write!(writer, "{}", e.html_escape());
        });
    }
}
