#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate x690;
use x690::ber::BER;
use x690::der::DER;
use x690::X690Codec;
use bytes::Bytes;

fuzz_target!(|data: &[u8]| {
    let datab1 = Bytes::copy_from_slice(data);
    let datab2 = Bytes::copy_from_slice(data);
    if let Ok((_, el)) = BER.decode_from_slice(data) {
        let _ = BER.decode_any(&el);
        let _ = DER.decode_any(&el);
    }
    if let Ok((_, el)) = BER.decode_from_bytes(datab1) {
        let _ = BER.decode_any(&el);
        let _ = DER.decode_any(&el);
    }
    if let Ok((_, el)) = DER.decode_from_slice(data) {
        let _ = BER.decode_any(&el);
        let _ = DER.decode_any(&el);
    }
    if let Ok((_, el)) = DER.decode_from_bytes(datab2) {
        let _ = BER.decode_any(&el);
        let _ = DER.decode_any(&el);
    }
});
