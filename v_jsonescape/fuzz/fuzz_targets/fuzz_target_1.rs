#![no_main]
use libfuzzer_sys::fuzz_target;

use v_jsonescape::{escape_fmt, escape_string};

fuzz_target!(|data: &[u8]| {
    if let Ok(data) = std::str::from_utf8(data) {
        let mut buf = String::with_capacity(data.len());
        let _ = escape_string(data, &mut buf);
        let _ = escape_fmt(data);
    }
});
