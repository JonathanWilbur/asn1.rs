#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate x520_stringprep;

use x520_stringprep::{
    x520_stringprep_case_exact_str,
    x520_stringprep_case_ignore_str,
};

fuzz_target!(|data: &str| {
    // We don't care about the results, we just want to make sure that the
    // functions don't panic.
    let _ = x520_stringprep_case_exact_str(data).count();
    let _ = x520_stringprep_case_ignore_str(data).count();
});
