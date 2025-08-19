#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate idm_frame;
use idm_frame::IdmFrameIter;

fuzz_target!(|data: &[u8]| {
    // We don't care about the results, we just want to make sure that the
    // functions don't panic. Just iterate over all of it.
    let _ = IdmFrameIter::new(data).count();

    // Test all Version 1 IDM PDUs
    let data1 = [
        [1].as_slice(),
        data,
    ].concat();
    // We don't care about the results, we just want to make sure that the
    // functions don't panic. Just iterate over all of it.
    let _ = IdmFrameIter::new(data1.as_slice()).count();

    // Test all Version 2 IDM PDUs
    let data2 = [
        [2].as_slice(),
        data,
    ].concat();
    // We don't care about the results, we just want to make sure that the
    // functions don't panic. Just iterate over all of it.
    let _ = IdmFrameIter::new(data2.as_slice()).count();
});
