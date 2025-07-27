# LDAP Distinguished Name (DN) Parser

This Rust crate parses Lightweight Directory Access Protocol (LDAP)
Distinguished Names (DNs) according to the syntax in
[IETF RFC 4514](https://datatracker.ietf.org/doc/html/rfc4514). It does no
heap allocations or copies of strings, so it should be Blazing Fast(tm),
although this has not been tested against other implementations. It also does
not require the standard library or any dependencies.

This crate has been fuzz-tested. It checks for an odd number of trailing
slashes in attribute values to prevent injection attacks.

## Usage

You can parse LDAP distinguished names like so:

```rust
use ldapdn::parse::dn_from_str;

let s = "gn=Jonathan+sn=Wilbur,st=FL,c=US";
let dn_iter = dn_from_str(s);

let mut atav_count = 0;
for rdn in dn_iter {
    // Iteration #1: iterates over "gn=Jonathan+sn=Wilbur"
    // Iteration #2: iterates over "st=FL"
    // Iteration #3: iterates over "c=US"
    let rdn_iter = rdn.expect("empty rdn");
    for maybe_atav in rdn_iter {
        let atav = maybe_atav.expect("malformed attribute type and value");
        let (attr_type, attr_value) = atav;
        match attr_type {
            "gn" => assert_eq!(attr_value, "Jonathan"),
            "sn" => assert_eq!(attr_value, "Wilbur"),
            "st" => assert_eq!(attr_value, "FL"),
            "c" => assert_eq!(attr_value, "US"),
            _ => panic!(),
        };
        atav_count += 1;
    }
}
assert_eq!(atav_count, 4);
```

The attribute values returned a **NOT ESCAPED**. You will have to do this as a
separate step, if you believe you need it.

You can unescape LDAP attribute values using one of these four
functions:

- `unescape_ldap_value_inplace` - Unescape a mutable byte slice in place
- `unescape_ldap_value_string` - Unescape a mutable string in place
- `unescape_ldap_value_cow` - Unescape a mutable byte slice, returning a clone if unescaping occurred
- `unescape_ldap_value_string_cow` - Unescape a mutable string, returning a clone if unescaping occurred

Here is an example of usage:

```rust
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use ldapdn::escape::{
    unescape_ldap_value_inplace,
    unescape_ldap_value_string_cow,
};

let mut bytes = Vec::from(b"James \\\"Jim\\\" Smith\\, III");
let result = unescape_ldap_value_inplace(&mut bytes);
assert_eq!(&bytes[..result], b"James \"Jim\" Smith, III");

let mut s = String::from("James \\\"Jim\\\" Smith\\, III");
let unescaped = unescape_ldap_value_string_cow(&mut s).expect("bad utf-8 encoding");
let new_s = unescaped.as_ref();
assert_eq!(new_s, "James \"Jim\" Smith, III");
```

## `no_std` Usage

This crate can function without a standard library (`no_std`).

The lone feature flag `alloc` is enabled by default, and it turns on
`unescape_ldap_value_cow` and `unescape_ldap_value_string_cow`,
which are useful if it is inconvenient to obtain a mutable reference to an
attribute value string.

## AI Usage Statement

None of the code for parsing distinguished names was produced by an AI or LLM
of any kind. The code for unescaping attribute values used in distinguished
names was initially written by AI / LLM, but were then heavily modified. The
unit tests were written by AI / LLM, but verified by the author of this crate.
