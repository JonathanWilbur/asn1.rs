#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate asn1;

fuzz_target!(|data: &[u8]| {
    let _ = wildboar_asn1::read_i64(data);
    let _ = wildboar_asn1::read_i128(data);
});
