#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate asn1;
use asn1::ISO8601Timestampable;

fuzz_target!(|data: &[u8]| {
    if let Ok(t) = asn1::UTCTime::try_from(data) {
        let _ = t.to_string();
        let _ = t.to_iso_8601_string();
    }
});
