extern crate v_htmlescape;
#[macro_use]
extern crate criterion;

use criterion::Criterion;
use v_htmlescape::Escape;

use std::fmt::Write;

criterion_main!(benches);
criterion_group!(benches, functions);

fn functions(c: &mut Criterion) {
    // Sizing
    c.bench_function("Sizing No Escaping 1 MB", sized_no_escaping_long);
    c.bench_function("Sizing Escaping 1 MB at 3.125%", sized_escaping_long);
    c.bench_function("Sizing Escaping 1 MB left 3%", sized_escaping_r_long);
    c.bench_function("Sizing Escaping 1 MB right 3%", sized_escaping_l_long);

    // Formatting
    c.bench_function("No Escapable 1 bytes", no_escaping_short);
    c.bench_function("False Positive 1 bytes", false_positive_short);
    c.bench_function("Escaping 1 bytes", escaping_short);

    c.bench_function("No Escapable 10 bytes", no_escaping_10);
    c.bench_function("False Positive 10 bytes", false_positive_10);
    c.bench_function("Escaping 10 b at 10%", escaping_10);

    c.bench_function("No Escapable 30 bytes", no_escaping_30);
    c.bench_function("False Positive 30 b", false_positive_30);
    c.bench_function("Escaping 30 b at 3.33%", escaping_30);

    c.bench_function("No Escapable 130 b", no_escaping);
    c.bench_function("False Positive 130 b", false_positive);
    c.bench_function("Escaping 130 b at 3.08%", escaping);

    c.bench_function("No Escapable tweet", no_escaping_tweet);
    c.bench_function("False Positive tweet", false_positive_tweet);
    c.bench_function("Escaping tweet at 2.86%", escaping_tweet);

    c.bench_function("No Escaping 1 MB", no_escaping_long);
    c.bench_function("False Positive 1 MB", false_positive_long);
    c.bench_function("Escaping 1 MB at 3.125%", escaping_long);
    c.bench_function("Escaping 1 MB left 3%", escaping_r_long);
    c.bench_function("Escaping 1 MB right 3%", escaping_l_long);
    c.bench_function("Escaping 1 MB false-positive at 3%", escaping_f_long);

    c.bench_function("Sized No Escapable 1 bytes", size_no_escaping_short);
    c.bench_function("Sized False Positive 1 bytes", size_false_positive_short);
    c.bench_function("Sized Escaping 1 bytes", size_escaping_short);

    c.bench_function("Sized No Escapable 10 bytes", size_no_escaping_10);
    c.bench_function("Sized False Positive 10 bytes", size_false_positive_10);
    c.bench_function("Sized Escaping 10 b at 10%", size_escaping_10);

    c.bench_function("Sized No Escapable 30 bytes", size_no_escaping_30);
    c.bench_function("Sized False Positive 30 b", size_false_positive_30);
    c.bench_function("Sized Escaping 30 b at 3.33%", size_escaping_30);

    c.bench_function("Sized No Escapable 130 b", size_no_escaping);
    c.bench_function("Sized False Positive 130 b", size_false_positive);
    c.bench_function("Sized Escaping 130 b at 3.08%", size_escaping);

    c.bench_function("Sized No Escapable tweet", size_no_escaping_tweet);
    c.bench_function("Sized False Positive tweet", size_false_positive_tweet);
    c.bench_function("Sized Escaping tweet at 2.86%", size_escaping_tweet);

    c.bench_function("Sized No Escaping 1 MB", size_no_escaping_long);
    c.bench_function("Sized False Positive 1 MB", size_false_positive_long);
    c.bench_function("Sized Escaping 1 MB at 3.125%", size_escaping_long);
    c.bench_function("Sized Escaping 1 MB left 3%", size_escaping_r_long);
    c.bench_function("Sized Escaping 1 MB right 3%", size_escaping_l_long);
    c.bench_function(
        "Sized Escaping 1 MB false-positive at 3%",
        size_escaping_f_long,
    );
}

static A: &str = "a";
static E: &str = "<";
static ED: &str = "&lt;";
// between 35, 36, 37, 61 in ascii table and no escapable 1 / 64
static F: &str = "=";

fn sized_escaping_long(b: &mut criterion::Bencher) {
    // 1 MB at 3.125% escape
    let s = [&A.repeat(15), E, &A.repeat(16)].join("").repeat(32 * 1024);
    let string = s.as_bytes();
    let e = Escape::new(string);

    b.iter(|| {
        e.size();
    });
}

fn sized_no_escaping_long(b: &mut criterion::Bencher) {
    let s = A.repeat(1024 * 1024);
    let string = s.as_bytes();
    let e = Escape::new(string);

    b.iter(|| {
        e.size();
    });
}

fn sized_escaping_r_long(b: &mut criterion::Bencher) {
    let _3 = 3 * ((1024 * 1024) / 100);
    let s = [A.repeat(1024 * 1024 - _3), E.repeat(_3)].join("");
    let string = s.as_bytes();
    let e = Escape::new(string);

    b.iter(|| {
        e.size();
    });
}

fn sized_escaping_l_long(b: &mut criterion::Bencher) {
    let _3 = 3 * ((1024 * 1024) / 100);
    let s = [E.repeat(_3), A.repeat(1024 * 1024 - _3)].join("");
    let string = s.as_bytes();
    let e = Escape::new(string);

    b.iter(|| {
        e.size();
    });
}

// 1 byte
fn escaping_short(b: &mut criterion::Bencher) {
    let string = E.as_bytes();
    let mut writer = String::with_capacity(ED.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(string));
    });
}

fn no_escaping_short(b: &mut criterion::Bencher) {
    let no_escape = A.as_bytes();
    let mut writer = String::with_capacity(A.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

fn false_positive_short(b: &mut criterion::Bencher) {
    let no_escape = F.as_bytes();
    let mut writer = String::with_capacity(F.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

// 10 bytes
fn escaping_10(b: &mut criterion::Bencher) {
    // 10 bytes at 10% escape
    let s = [A, A, A, A, A, E, A, A, A, A, A].join("");
    let string = s.as_bytes();
    let mut writer = String::with_capacity(9 * A.len() + ED.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(string));
    });
}

fn no_escaping_10(b: &mut criterion::Bencher) {
    let s = A.repeat(10);
    let no_escape = s.as_bytes();
    let mut writer = String::with_capacity(10 * A.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

fn false_positive_10(b: &mut criterion::Bencher) {
    let s = F.repeat(10);
    let no_escape = s.as_bytes();
    let mut writer = String::with_capacity(10 * F.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

// 30 bytes
fn escaping_30(b: &mut criterion::Bencher) {
    // 30 bytes at 3.33% escape
    let s = [&A.repeat(15), E, &A.repeat(14)].join("");
    let string = s.as_bytes();
    let mut writer = String::with_capacity(29 * A.len() + ED.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(string));
    });
}

fn no_escaping_30(b: &mut criterion::Bencher) {
    let s = A.repeat(30);
    let no_escape = s.as_bytes();
    let mut writer = String::with_capacity(30 * A.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

fn false_positive_30(b: &mut criterion::Bencher) {
    let s = F.repeat(30);
    let no_escape = s.as_bytes();

    let mut writer = String::with_capacity(30 * F.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

// 130 bytes
fn escaping(b: &mut criterion::Bencher) {
    // 130 bytes at 3.08% escape
    let s = [
        &A.repeat(25),
        E,
        &A.repeat(25),
        E,
        &A.repeat(25),
        E,
        &A.repeat(25),
        E,
        &A.repeat(26),
    ]
        .join("");
    let string = s.as_bytes();
    let mut writer = String::with_capacity(126 * A.len() + 4 * ED.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(string));
    });
}

fn no_escaping(b: &mut criterion::Bencher) {
    let s = A.repeat(130);
    let no_escape = s.as_bytes();
    let mut writer = String::with_capacity(130 * A.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

fn false_positive(b: &mut criterion::Bencher) {
    let s = F.repeat(130);
    let no_escape = s.as_bytes();
    let mut writer = String::with_capacity(130 * F.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

// 280 bytes
fn escaping_tweet(b: &mut criterion::Bencher) {
    // 280 bytes at 2.86% escape
    let s = [
        &A.repeat(30),
        E,
        &A.repeat(25),
        E,
        &A.repeat(25),
        E,
        &A.repeat(25),
        E,
        &A.repeat(31),
    ]
        .join("")
        .repeat(2);
    let string = s.as_bytes();
    let mut writer = String::with_capacity(272 * A.len() + 8 * ED.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(string));
    });
}

fn no_escaping_tweet(b: &mut criterion::Bencher) {
    let s = A.repeat(280);
    let no_escape = s.as_bytes();
    let mut writer = String::with_capacity(280 * A.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

fn false_positive_tweet(b: &mut criterion::Bencher) {
    let s = F.repeat(280);
    let no_escape = s.as_bytes();
    let mut writer = String::with_capacity(280 * F.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

// 1 MB
fn escaping_long(b: &mut criterion::Bencher) {
    // 1 MB at 3.125% escape
    let s = [&A.repeat(15), E, &A.repeat(16)].join("").repeat(32 * 1024);
    let string = s.as_bytes();
    let mut writer = String::with_capacity(32 * 1024 * (31 * A.len() + ED.len()));

    b.iter(|| {
        write!(writer, "{}", Escape::new(string));
    });
}

fn no_escaping_long(b: &mut criterion::Bencher) {
    let s = A.repeat(1024 * 1024);
    let no_escape = s.as_bytes();
    let mut writer = String::with_capacity(1024 * 1024 * A.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

fn false_positive_long(b: &mut criterion::Bencher) {
    let s = F.repeat(1024 * 1024);
    let no_escape = s.as_bytes();
    let mut writer = String::with_capacity(1024 * 1024 * F.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(no_escape));
    });
}

fn escaping_r_long(b: &mut criterion::Bencher) {
    let _3 = 3 * ((1024 * 1024) / 100);
    let s = [A.repeat(1024 * 1024 - _3), E.repeat(_3)].join("");
    let string = s.as_bytes();
    let mut writer = String::with_capacity((1024 * 1024 - _3) * A.len() + _3 * ED.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(string));
    });
}

fn escaping_l_long(b: &mut criterion::Bencher) {
    let _3 = 3 * ((1024 * 1024) / 100);
    let s = [E.repeat(_3), A.repeat(1024 * 1024 - _3)].join("");
    let string = s.as_bytes();
    let mut writer = String::with_capacity((1024 * 1024 - _3) * A.len() + _3 * ED.len());

    b.iter(|| {
        write!(writer, "{}", Escape::new(string));
    });
}

fn escaping_f_long(b: &mut criterion::Bencher) {
    // 1 MB at 3.125% escape
    let s = [&F.repeat(15), E, &F.repeat(16)].join("").repeat(32 * 1024);
    let string = s.as_bytes();
    let mut writer = String::with_capacity(32 * 1024 * (31 * F.len() + ED.len()));

    b.iter(|| {
        write!(writer, "{}", Escape::new(string));
    });
}

// 1 byte
fn size_escaping_short(b: &mut criterion::Bencher) {
    let string = E.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_no_escaping_short(b: &mut criterion::Bencher) {
    let string = A.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_false_positive_short(b: &mut criterion::Bencher) {
    let string = F.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

// 10 bytes
fn size_escaping_10(b: &mut criterion::Bencher) {
    // 10 bytes at 10% escape
    let s = [A, A, A, A, A, E, A, A, A, A, A].join("");
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", Escape::new(string));
    });
}

fn size_no_escaping_10(b: &mut criterion::Bencher) {
    let s = A.repeat(10);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_false_positive_10(b: &mut criterion::Bencher) {
    let s = F.repeat(10);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

// 30 bytes
fn size_escaping_30(b: &mut criterion::Bencher) {
    // 30 bytes at 3.33% escape
    let s = [&A.repeat(15), E, &A.repeat(14)].join("");
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_no_escaping_30(b: &mut criterion::Bencher) {
    let s = A.repeat(30);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_false_positive_30(b: &mut criterion::Bencher) {
    let s = F.repeat(30);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

// 130 bytes
fn size_escaping(b: &mut criterion::Bencher) {
    // 130 bytes at 3.08% escape
    let s = [
        &A.repeat(25),
        E,
        &A.repeat(25),
        E,
        &A.repeat(25),
        E,
        &A.repeat(25),
        E,
        &A.repeat(26),
    ]
        .join("");
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_no_escaping(b: &mut criterion::Bencher) {
    let s = A.repeat(130);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_false_positive(b: &mut criterion::Bencher) {
    let s = F.repeat(130);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

// 280 bytes
fn size_escaping_tweet(b: &mut criterion::Bencher) {
    // 280 bytes at 2.86% escape
    let s = [
        &A.repeat(30),
        E,
        &A.repeat(25),
        E,
        &A.repeat(25),
        E,
        &A.repeat(25),
        E,
        &A.repeat(31),
    ]
        .join("")
        .repeat(2);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_no_escaping_tweet(b: &mut criterion::Bencher) {
    let s = A.repeat(280);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_false_positive_tweet(b: &mut criterion::Bencher) {
    let s = F.repeat(280);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

// 1 MB
fn size_escaping_long(b: &mut criterion::Bencher) {
    // 1 MB at 3.125% escape
    let s = [&A.repeat(15), E, &A.repeat(16)].join("").repeat(32 * 1024);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_no_escaping_long(b: &mut criterion::Bencher) {
    let s = A.repeat(1024 * 1024);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_false_positive_long(b: &mut criterion::Bencher) {
    let s = F.repeat(1024 * 1024);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_escaping_r_long(b: &mut criterion::Bencher) {
    let _3 = 3 * ((1024 * 1024) / 100);
    let s = [A.repeat(1024 * 1024 - _3), E.repeat(_3)].join("");
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_escaping_l_long(b: &mut criterion::Bencher) {
    let _3 = 3 * ((1024 * 1024) / 100);
    let s = [E.repeat(_3), A.repeat(1024 * 1024 - _3)].join("");
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}

fn size_escaping_f_long(b: &mut criterion::Bencher) {
    // 1 MB at 3.125% escape
    let s = [&F.repeat(15), E, &F.repeat(16)].join("").repeat(32 * 1024);
    let string = s.as_bytes();
    let e = Escape::new(string);
    let mut writer = String::with_capacity(e.size());

    b.iter(|| {
        write!(writer, "{}", e);
    });
}
