#![no_main]
use libfuzzer_sys::fuzz_target;

use v_jsonescape::{b_escape, VJsonescape};

fuzz_target!(|data: &[u8]| {
    let _ = VJsonescape::new(data).to_string();
    b_escape(data, &mut bytes::BytesMut::with_capacity(0));
    // fuzzed code goes here
});
