#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate teletex;

use teletex::teletex_to_utf8;

fuzz_target!(|data: &[u8]| {
    // We don't care about the results, we just want to make sure that the
    // functions don't panic.
    let _ = teletex_to_utf8(data).count();
});
