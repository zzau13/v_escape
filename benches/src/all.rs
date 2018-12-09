extern crate v_htmlescape;
#[macro_use]
extern crate criterion;

use criterion::{Bencher, Benchmark, Criterion, Throughput};
use v_htmlescape::Escape;

use std::fmt::Write;
use std::str;

static HUGE: &[u8] = include_bytes!("../data/sherlock-holmes-huge.txt");
// escapeable characters replaced by 'a'
static HUGE_ED: &[u8] = include_bytes!("../data/sherlock-holmes-escaped-huge.txt");
static SMALL: &[u8] = include_bytes!("../data/sherlock-holmes-small.txt");
// escapeable characters replaced by 'a'
static SMALL_ED: &[u8] = include_bytes!("../data/sherlock-holmes-escaped-small.txt");
static TINY: &[u8] = include_bytes!("../data/sherlock-holmes-tiny.txt");
// same size
static TINY_ED: &[u8] = include_bytes!("../data/sherlock-holmes-escaped-tiny.txt");
static EMPTY: &[u8] = &[];

fn functions(c: &mut Criterion) {

    let group = "Sizing";
    define(c, group, "huge", HUGE, sized(HUGE));
    define(c, group, "huge escaped", HUGE_ED, sized(HUGE_ED));
    define(c, group, "small", SMALL, sized(SMALL));
    define(c, group, "small escaped", SMALL_ED, sized(SMALL_ED));
    define(c, group, "tiny", TINY, sized(TINY));
    define(c, group, "tiny escaped", TINY_ED, sized(TINY_ED));
    define(c, group, "empty", EMPTY, sized(EMPTY));

    let group = "Escaping";
    define(c, group, "huge", HUGE, escaping(HUGE));
    define(c, group, "huge escaped", HUGE_ED, escaping(HUGE_ED));
    define(c, group, "small", SMALL, escaping(SMALL));
    define(c, group, "small escaped", SMALL_ED, escaping(SMALL_ED));
    define(c, group, "tiny", TINY, escaping(TINY));
    define(c, group, "tiny escaped", TINY_ED, escaping(TINY_ED));
    define(c, group, "empty", EMPTY, escaping(EMPTY));

    let group = "Sized Escaping";
    define(c, group, "huge", HUGE, size_escaping(HUGE));
    define(c, group, "huge escaped", HUGE_ED, size_escaping(HUGE_ED));
    define(c, group, "small", SMALL, size_escaping(SMALL));
    define(c, group, "small escaped", SMALL_ED, size_escaping(SMALL_ED));
    define(c, group, "tiny", TINY, size_escaping(TINY));
    define(c, group, "tiny escaped", TINY_ED, size_escaping(TINY_ED));
    define(c, group, "empty", EMPTY, size_escaping(EMPTY));

    let group = "std Writing";
    define(c, group, "huge", HUGE, writing(HUGE));
    define(c, group, "small", SMALL, writing(SMALL));
    define(c, group, "tiny", TINY, writing(TINY));
}

fn sized(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = Escape::new(corpus);

        b.iter(|| {
            e.size();
        });
    }
}

fn writing(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let mut writer = String::new();

        b.iter(|| {
            write!(writer, "{}", unsafe { str::from_utf8_unchecked(corpus) });
        });
    }
}

fn escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = Escape::new(corpus);
        let mut writer = String::new();

        b.iter(|| {
            write!(writer, "{}", e);
        });
    }
}

fn size_escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = Escape::new(corpus);
        let mut writer = String::with_capacity(e.size());

        b.iter(|| {
            write!(writer, "{}", e);
        });
    }
}

fn define(
    c: &mut Criterion,
    group_name: &str,
    bench_name: &str,
    corpus: &[u8],
    bench: impl FnMut(&mut Bencher) + 'static,
) {
    let tput = Throughput::Bytes(corpus.len() as u32);
    let benchmark = Benchmark::new(bench_name, bench).throughput(tput);
    c.bench(group_name, benchmark);
}

criterion_main!(benches);
criterion_group!(benches, functions);
