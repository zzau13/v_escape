#![no_main]
use libfuzzer_sys::fuzz_target;

use v_html5escape::{escape_bytes, escape_fmt, escape_string};

fuzz_target!(|data: &[u8]| {
    let mut bytes = Vec::with_capacity(data.len());
    escape_bytes(data, &mut bytes);

    if let Ok(data) = std::str::from_utf8(data) {
        let mut string = String::with_capacity(data.len());
        escape_string(data, &mut string);
        assert_eq!(bytes, string.as_bytes());
        assert_eq!(bytes, escape_fmt(data).to_string().as_bytes());
    }
});
