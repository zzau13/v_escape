#![no_main]
use libfuzzer_sys::fuzz_target;

use v_htmlescape::{b_escape, VHtmlescape};

fuzz_target!(|data: &[u8]| {
    let _ = VHtmlescape::new(data).to_string();
    b_escape(data, &mut bytes::BytesMut::with_capacity(0));
    // fuzzed code goes here
});
