use criterion::Bencher;
use std::fmt::Write;
use v_shellescape::{unix, windows};

pub fn unix_escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = unix::ShellEscape::new(corpus);
        let mut writer = String::new();

        b.iter(|| {
            write!(writer, "{}", e);
        });
    }
}

pub fn windows_escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = windows::ShellEscape::new(corpus);
        let mut writer = String::new();

        b.iter(|| {
            write!(writer, "{}", e);
        });
    }
}
