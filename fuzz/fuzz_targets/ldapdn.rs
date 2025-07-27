#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate ldapdn;

fuzz_target!(|input: (&str, &[u8])| {
    let (s, b) = input;
    let _ = ldapdn::parse::dn_from_str(s);
    let _ = ldapdn::escape::unescape_ldap_value_string(s.to_string().as_mut_str());
    let _ = ldapdn::escape::unescape_ldap_value_string_cow(s);
    let _ = ldapdn::escape::unescape_ldap_value_inplace(b.to_vec().as_mut_slice());
    let _ = ldapdn::escape::unescape_ldap_value_cow(b);
});
