use criterion::{Bencher, Criterion, Throughput};

static HUGE: &str = include_str!("../data/sherlock-holmes-huge.txt");
// escapable characters replaced by 'a'
static HUGE_ED: &str = include_str!("../data/sherlock-holmes-escaped-huge.txt");

static SMALL: &str = include_str!("../data/sherlock-holmes-small.txt");
// escapable characters replaced by 'a'
static SMALL_ED: &str = include_str!("../data/sherlock-holmes-escaped-small.txt");

static TINY: &str = include_str!("../data/sherlock-holmes-tiny.txt");
// same size
static TINY_ED: &str = include_str!("../data/sherlock-holmes-escaped-tiny.txt");

static VERY_TINY: &str = "ab>cdefghijklmnopqrstuvw<xyz";
// escapable characters replaced by '.'
static VERY_TINY_ED: &str = "ab.cdefghijklmnopqrstuvw.xyz";

// Bad cases
static ULTRA_TINY: &str = "abcd<ef";
// escapable characters replaced by '.'
static ULTRA_TINY_ED: &str = "abcd.ef";

static ONE: &str = "<";
// escapable characters replaced by '1'
static ONE_ED: &str = "1";

static EMPTY: &str = "";

pub(crate) const CASES: [(&str, &str); 13] = [
    ("huge", HUGE),
    ("huge escaped", HUGE_ED),
    ("small", SMALL),
    ("small escaped", SMALL_ED),
    ("tiny", TINY),
    ("tiny escaped", TINY_ED),
    ("very tiny", VERY_TINY),
    ("very tiny escaped", VERY_TINY_ED),
    ("ultra tiny", ULTRA_TINY),
    ("ultra tiny escaped", ULTRA_TINY_ED),
    ("one", ONE),
    ("one escaped", ONE_ED),
    ("empty", EMPTY),
];

pub(crate) fn define(
    c: &mut Criterion,
    group_name: &str,
    bench_name: &str,
    corpus: &str,
    bench: impl FnMut(&mut Bencher) + 'static,
) {
    let tput = Throughput::Bytes(corpus.len() as u64);
    let mut benchmark = c.benchmark_group(group_name);
    benchmark.throughput(tput);
    benchmark.bench_function(bench_name, bench);
}

macro_rules! register_cases {
    ($c:ident, $group:expr, $bench_factory:path) => {{
        for (name, corpus) in $crate::common::CASES {
            $crate::common::define($c, $group, name, corpus, $bench_factory(corpus));
        }
    }};
}

pub(crate) use register_cases;
