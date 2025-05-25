#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate asn1;

fuzz_target!(|data: &[u8]| {
    let _ = wildboar_asn1::TIME_OF_DAY::try_from(data);
});
