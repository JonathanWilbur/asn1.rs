#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate x690;
use x690::ber::BER;
use x690::der::DER;
use x690::X690Codec;

fuzz_target!(|data: u32| {
    // These don't have to be correct. We are just checking that nothing causes
    // these to panic.
    let cases = vec![
        format!("{}", data),
        format!("{}0", data),
        format!("{}00", data),
        format!("{}000", data),
        format!("{}Z", data),
        format!("{}0Z", data),
        format!("{}00Z", data),
        format!("{}000Z", data),
        format!("{}.Z", data),
        format!("{}.0Z", data),
        format!("{}.00Z", data),
        format!("{}.1234Z", data),
        format!("{}+0900", data),
        format!("{}0-0800", data),
        format!("{}00+1030", data),
        format!("{}000.0+0445", data),
        format!("{}.Z+92.2904", data),
        format!("{},Z.0Z", data),
        format!("{}.+9999", data),
        format!("{}-9999", data),
    ];

    for case in cases {
        BER.validate_generalized_time_value(case.as_bytes());
        DER.validate_generalized_time_value(case.as_bytes());
        BER.validate_utc_time_value(case.as_bytes());
        DER.validate_utc_time_value(case.as_bytes());
    }

});
