#![no_main]

use std::str::FromStr;

use libfuzzer_sys::fuzz_target;

extern crate asn1;

fuzz_target!(|dur: asn1::DURATION_EQUIVALENT| {
    dur.to_approximate_seconds();
    asn1::DURATION_EQUIVALENT::from_str(dur.to_string().as_str()).unwrap();
    assert_eq!(dur.clone(), dur.clone());
});
