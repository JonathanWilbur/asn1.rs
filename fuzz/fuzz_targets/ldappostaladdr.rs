#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate ldappostaladdr;
use ldappostaladdr::{
    parse_postal_address,
    unescape_postal_address_line,
    escape_postal_address_line,
};

fuzz_target!(|input: &str| {
    let lines = parse_postal_address(input);
    for (line, bs_esc, dollar_esc) in lines {
        let unescaped = unescape_postal_address_line(line, bs_esc, dollar_esc);
        let _ = escape_postal_address_line(unescaped.as_ref());
    }
});
