#![no_main]
use libfuzzer_sys::fuzz_target;

use v_htmlescape::{HTMLEscape, b_escape};

fuzz_target!(|data: &[u8]| {
    let _ = HTMLEscape::new(data).to_string();
    b_escape(data, &mut bytes::BytesMut::with_capacity(0));
    // fuzzed code goes here
});
