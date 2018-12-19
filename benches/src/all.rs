#![allow(unused_must_use)]

use criterion::{Bencher, Benchmark, Criterion, Throughput};
use v_htmlescape::HTMLEscape as Escape;

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

static VERY_TINY: &[u8] = b"ab>cdefghijklmnopqrstuvw<xyz";
// escapeable characters replaced by '.'
static VERY_TINY_ED: &[u8] = b"ab.cdefghijklmnopqrstuvw.xyz";

static VV_TINY: &[u8] = b"abcd<efghijklm";
// escapeable characters replaced by '.'
static VV_TINY_ED: &[u8] = b"abcd.efghijklm";

// Avx 11 characters limit performance
static ULTRA_TINY: &[u8] = b"abcd<efghij";
// escapeable characters replaced by '.'
static ULTRA_TINY_ED: &[u8] = b"abcd.efghij";

// Bad cases
static ULTRA_V_TINY: &[u8] = b"abcd<ef";
// escapeable characters replaced by '.'
static ULTRA_V_TINY_ED: &[u8] = b"abcd.ef";

static EMPTY: &[u8] = &[];

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

#[rustfmt::skip]
macro_rules! groups {
    ($c:ident, $group:ident, $fun:ident) => {{
        define($c, $group, "huge", HUGE, $fun(HUGE));
        define($c, $group, "huge escaped", HUGE_ED, $fun(HUGE_ED));

        define($c, $group, "small", SMALL, $fun(SMALL));
        define($c, $group, "small escaped", SMALL_ED, $fun(SMALL_ED));

        define($c, $group, "tiny", TINY, $fun(TINY));
        define($c, $group, "tiny escaped", TINY_ED, $fun(TINY_ED));

        define($c, $group, "very tiny", VERY_TINY, $fun(VERY_TINY));
        define($c, $group, "very tiny escaped", VERY_TINY_ED, $fun(VERY_TINY_ED));

        define($c, $group, "very very tiny", VV_TINY, $fun(VV_TINY));
        define($c, $group, "very very tiny escaped", VV_TINY_ED, $fun(VV_TINY_ED));

        define($c, $group, "ultra tiny", ULTRA_TINY, $fun(ULTRA_TINY));
        define($c, $group, "ultra tiny escaped", ULTRA_TINY_ED, $fun(ULTRA_TINY_ED));

        define($c, $group, "ultra very tiny", ULTRA_V_TINY, $fun(ULTRA_V_TINY));
        define($c, $group, "ultra very tiny escaped", ULTRA_V_TINY_ED, $fun(ULTRA_V_TINY_ED));

        define($c, $group, "empty", EMPTY, $fun(EMPTY));
    }};
}

fn functions(c: &mut Criterion) {
    let group = "Sizing";
    groups!(c, group, sized);

    let group = "Escaping";
    groups!(c, group, escaping);

    let group = "Sized Escaping";
    groups!(c, group, size_escaping);

    let group = "std Writing";
    groups!(c, group, writing);
}

criterion_main!(benches);
criterion_group!(benches, functions);
