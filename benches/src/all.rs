use std::str;

#[macro_use]
extern crate criterion;
use criterion::{Bencher, BenchmarkGroup, Criterion, Throughput};

#[path = "v_htmlescape.rs"]
mod v_html;
#[path = "v_jsonescape.rs"]
mod v_json;
#[path = "v_latexescape.rs"]
mod v_latex;

static HUGE: &[u8] = include_bytes!("../data/sherlock-holmes-huge.txt");
// escapable characters replaced by 'a'
static HUGE_ED: &[u8] = include_bytes!("../data/sherlock-holmes-escaped-huge.txt");

static SMALL: &[u8] = include_bytes!("../data/sherlock-holmes-small.txt");
// escapable characters replaced by 'a'
static SMALL_ED: &[u8] = include_bytes!("../data/sherlock-holmes-escaped-small.txt");

static TINY: &[u8] = include_bytes!("../data/sherlock-holmes-tiny.txt");
// same size
static TINY_ED: &[u8] = include_bytes!("../data/sherlock-holmes-escaped-tiny.txt");

static VERY_TINY: &[u8] = b"ab>cdefghijklmnopqrstuvw<xyz";
// escapable characters replaced by '.'
static VERY_TINY_ED: &[u8] = b"ab.cdefghijklmnopqrstuvw.xyz";

// Bad cases
static ULTRA_TINY: &[u8] = b"abcd<ef";
// escapable characters replaced by '.'
static ULTRA_TINY_ED: &[u8] = b"abcd.ef";

static ONE: &[u8] = b"<";
// escapable characters replaced by '1'
static ONE_ED: &[u8] = b"1";

static EMPTY: &[u8] = &[];

fn define(
    c: &mut Criterion,
    group_name: &str,
    bench_name: &str,
    corpus: &[u8],
    bench: impl FnMut(&mut Bencher) + 'static,
) {
    let tput = Throughput::Bytes(corpus.len() as u64);
    let mut benchmark = c.benchmark_group(group_name);
    benchmark.throughput(tput);
    benchmark.bench_function(bench_name, bench);
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

        define($c, $group, "one", ONE, $fun(ONE));
        define($c, $group, "one escaped", ONE_ED, $fun(ONE_ED));

        define($c, $group, "empty", EMPTY, $fun(EMPTY));
    }};
}

macro_rules! v_escape {
    ($c:ident) => {
        use crate::v_html::escaping as v_h;
        let group = "v_htmlescape/Escaping";
        groups!($c, group, v_h);

        use crate::v_json::escaping as v_j;
        let group = "v_jsonescape/Escaping";
        groups!($c, group, v_j);

        use crate::v_latex::escaping as v_l;
        let group = "v_latexescape/Escaping";
        groups!($c, group, v_l);
    };
}

macro_rules! std_writing {
    ($c:ident) => {
        use std::fmt::Write;

        fn writing(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
            move |b: &mut Bencher| {
                let mut writer = String::with_capacity(corpus.len());

                b.iter(|| {
                    let _ = write!(writer, "{}", unsafe { str::from_utf8_unchecked(corpus) });
                });
            }
        }

        let group = "std Writing";
        groups!($c, group, writing);
    };
}

fn functions(c: &mut Criterion) {
    v_escape!(c);
    std_writing!(c);
}

criterion_main!(benches);
criterion_group!(benches, functions);
