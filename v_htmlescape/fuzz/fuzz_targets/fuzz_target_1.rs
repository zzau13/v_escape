#![no_main]
use libfuzzer_sys::fuzz_target;

use v_htmlescape::{b_escape, HTMLEscape};

fuzz_target!(|data: &[u8]| {
    let _ = HTMLEscape::new(data).to_string();
    b_escape(data, &mut Vec::with_capacity(0));
});
