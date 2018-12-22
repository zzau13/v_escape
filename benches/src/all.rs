#![allow(unused_must_use)]
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate criterion;
use criterion::{Bencher, Benchmark, Criterion, Throughput};

use std::fmt::Write;
use std::str;

mod askama_escape;
#[cfg(all(v_escape_benches_nightly, feature = "with-rocket"))]
mod rocket;
mod v_escape;

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

// Bad cases
static ULTRA_TINY: &[u8] = b"abcd<ef";
// escapeable characters replaced by '.'
static ULTRA_TINY_ED: &[u8] = b"abcd.ef";

static EMPTY: &[u8] = &[];

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

        define($c, $group, "ultra tiny", ULTRA_TINY, $fun(ULTRA_TINY));
        define($c, $group, "ultra tiny escaped", ULTRA_TINY_ED, $fun(ULTRA_TINY_ED));

        define($c, $group, "empty", EMPTY, $fun(EMPTY));
    }};
}

macro_rules! v_escape {
    ($c:ident) => {
        use crate::v_escape::{escaping as v_e, sized as v_s};
        let group = "v_escape/Escaping";
        groups!($c, group, v_e);

        let group = "v_escape/Sizing";
        groups!($c, group, v_s);
    };
}

macro_rules! askama_escape {
    ($c:ident) => {
        use crate::askama_escape::escaping as a_e;
        let group = "askama_escape/Escaping";
        groups!($c, group, a_e);
    };
}

macro_rules! std_writing {
    ($c:ident) => {
        fn writing(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
            move |b: &mut Bencher| {
                let mut writer = String::new();

                b.iter(|| {
                    write!(writer, "{}", unsafe { str::from_utf8_unchecked(corpus) });
                });
            }
        }

        let group = "std Writing";
        groups!($c, group, writing);
    };
}

cfg_if! {
    if #[cfg(all(v_escape_benches_nightly, feature = "with-rocket"))] {
        fn functions(c: &mut Criterion) {
            use crate::rocket::escaping as r_e;
            let group = "rocket/Escaping";
            groups!(c, group, r_e);

            askama_escape!(c);
            v_escape!(c);
            std_writing!(c);
        }
    } else {
        fn functions(c: &mut Criterion) {
            askama_escape!(c);
            v_escape!(c);
            std_writing!(c);
        }
    }
}

criterion_main!(benches);
criterion_group!(benches, functions);
