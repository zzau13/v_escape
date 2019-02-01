#![allow(unused_must_use)]
#![allow(unused_macros)]

#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate criterion;
#[macro_use]
extern crate v_escape;
use criterion::{Bencher, Benchmark, Criterion, Throughput};

use std::str;

#[cfg(feature = "with-compare")]
mod askama_escape;
#[cfg(all(
    v_escape_benches_nightly,
    feature = "with-rocket",
    feature = "with-compare"
))]
mod rocket;
#[cfg(feature = "with-compare")]
#[path = "shell-escape.rs"]
mod shell_escape;
#[path = "v_escape.rs"]
mod v;
#[path = "v_htmlescape.rs"]
mod v_html;
#[path = "v_latexescape.rs"]
mod v_latex;
#[path = "v_shellescape.rs"]
mod v_shell;

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

        define($c, $group, "one", ONE, $fun(ONE));
        define($c, $group, "one escaped", ONE_ED, $fun(ONE_ED));

        define($c, $group, "empty", EMPTY, $fun(EMPTY));
    }};
}

macro_rules! v_shellescape {
    ($c:ident) => {
        use crate::v_shell::{unix_escaping as v_su, windows_escaping as v_sw};
        let group = "v_shellescape/unix/Escaping";
        groups!($c, group, v_su);

        let group = "v_shellescape/windows/Escaping";
        groups!($c, group, v_sw);
    };
}
macro_rules! v_escape {
    ($c:ident) => {
        use crate::v::escaping as v_e;
        let group = "v_escape/ascii numbers RANGE/Escaping";
        groups!($c, group, v_e);

        use crate::v_html::escaping as v_h;
        let group = "v_htmlescape/Escaping";
        groups!($c, group, v_h);

        use crate::v_latex::escaping as v_l;
        let group = "v_latexescape/Escaping";
        groups!($c, group, v_l);

        v_shellescape!($c);
    };
}

macro_rules! askama_escape {
    ($c:ident) => {
        use crate::askama_escape::escaping as a_e;
        let group = "askama_escape/Escaping";
        groups!($c, group, a_e);
    };
}

macro_rules! shell_escape {
    ($c:ident) => {
        use crate::shell_escape::{unix_escaping as se_su, windows_escaping as se_sw};
        let group = "shell-escape/unix/Escaping";
        groups!($c, group, se_su);

        let group = "shell-escape/windows/Escaping";
        groups!($c, group, se_sw);
    };
}

macro_rules! std_writing {
    ($c:ident) => {
        use std::fmt::Write;

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
    if #[cfg(all(v_escape_benches_nightly, feature = "with-rocket", feature = "with-compare"))] {
        fn functions(c: &mut Criterion) {
            use crate::rocket::escaping as r_e;
            let group = "rocket/Escaping";
            groups!(c, group, r_e);

            askama_escape!(c);
            shell_escape!(c);
            std_writing!(c);
            v_escape!(c);

        }
    } else if #[cfg(feature = "with-compare")] {
        fn functions(c: &mut Criterion) {
            askama_escape!(c);
            shell_escape!(c);
            std_writing!(c);
            v_escape!(c);
        }
    } else {
        fn functions(c: &mut Criterion) {
            v_escape!(c);
        }
    }
}

criterion_main!(benches);
criterion_group!(benches, functions);
