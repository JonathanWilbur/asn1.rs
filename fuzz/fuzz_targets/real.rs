#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate x690;
use x690::ber::BER;
use x690::der::DER;
use x690::X690Codec;

fuzz_target!(|data: &[u8]| {
    // These don't have to be correct. We are just checking that nothing causes
    // these to panic.
    let cases = vec![
        Vec::from(data),
        Vec::from([
            vec![0x81],
            Vec::from(data),
        ].concat()),
        Vec::from([
            vec![0x80],
            Vec::from(data),
        ].concat()),
        Vec::from([
            vec![0x82],
            Vec::from(data),
        ].concat()),
        Vec::from([
            vec![0x83],
            Vec::from(data),
        ].concat()),
        Vec::from([
            vec![0x01],
            Vec::from(data),
        ].concat()),
        Vec::from([
            vec![0x02],
            Vec::from(data),
        ].concat()),
        Vec::from([
            vec![0x03],
            Vec::from(data),
        ].concat()),
    ];

    for case in cases {
        BER.validate_real_value(case.as_slice());
        DER.validate_real_value(case.as_slice());
        BER.validate_real_value(case.as_slice());
        DER.validate_real_value(case.as_slice());
        BER.decode_real_value(case.as_slice());
        DER.decode_real_value(case.as_slice());
        BER.decode_real_value(case.as_slice());
        DER.decode_real_value(case.as_slice());
    }

});
