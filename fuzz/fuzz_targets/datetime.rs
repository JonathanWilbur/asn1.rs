#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate asn1;

fuzz_target!(|data: &[u8]| {
    let _ = asn1::DATE_TIME::try_from(data);
});
