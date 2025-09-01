#![no_main]

use std::str::FromStr;

use libfuzzer_sys::fuzz_target;

extern crate x690;
use x690::{X690Codec, BER};

extern crate presentation_address;
use presentation_address::PresentationAddress;

fuzz_target!(|input: (&[u8], &str)| {
    let (input_b, input_s) = input;
    let _ = PresentationAddress::from_str(input_s);
    if let Ok((_, el)) = BER.decode_from_slice(input_b) {
        let _ = PresentationAddress::try_from(&el);
    }
});
