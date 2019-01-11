use criterion::Bencher;
use shell_escape::{unix, windows};
use std::borrow::Cow::Borrowed;
use std::fmt::Write;
use std::str::from_utf8_unchecked;

pub fn unix_escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let mut writer = String::new();

        b.iter(move || {
            write!(
                writer,
                "{}",
                unix::escape(Borrowed(unsafe { from_utf8_unchecked(corpus) }))
            );
        });
    }
}

pub fn windows_escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let mut writer = String::new();

        b.iter(move || {
            write!(
                writer,
                "{}",
                windows::escape(Borrowed(unsafe { from_utf8_unchecked(corpus) }))
            );
        });
    }
}
