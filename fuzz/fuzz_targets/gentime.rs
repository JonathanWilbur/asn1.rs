#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate asn1;
use wildboar_asn1::ISO8601Timestampable;

fuzz_target!(|data: &[u8]| {
    if let Ok(t) = wildboar_asn1::GeneralizedTime::try_from(data) {
        let _ = t.to_string();
        let _ = t.to_iso_8601_string();
    }
});
