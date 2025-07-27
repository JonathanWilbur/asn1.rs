# LDAP Distinguished Name (DN) Parser

This Rust crate parses Lightweight Directory Access Protocol (LDAP)
Distinguished Names (DNs) according to the syntax in
[IETF RFC 4514](https://datatracker.ietf.org/doc/html/rfc4514). It does no
heap allocations or copies of strings, so it should be Blazing Fast(tm),
although this has not been tested against other implementations. It also does
not require the standard library or any dependencies.

## Usage

```rust,ignore
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

## `no_std` Usage

Just set `features` to `[]` in your `Cargo.toml`.

## AI Usage Statement

None of the code for parsing distinguished names was produced by an AI or LLM
of any kind. The code for unescaping attribute values used in distinguished
names was initially written by AI / LLM, and the unit tests were as well; both
were validated by the author.
