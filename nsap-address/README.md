# X.213 NSAP Library

ITU-T Recommendation X.213 Network Service Access Point (NSAP) address parsing
and printing. These address types were designed for use in OSI networking, but
have full compatibility with IP networking. Since OSI networking is ancient
history, this library prioritizes the IP networking aspects, but everything
should be supported.

## This crate

This module decodes and encodes a Network Service Access Point (NSAP) to and
from the "preferred binary encoding" described in Annex A, Section A.5.3 of
[ITU-T Recommendation X.213 (2001)](https://www.itu.int/rec/T-REC-X.213-200110-I/en).

In addition to this, it displays and decodes NSAPs to and from human-readable
strings according to the procedures defined in
[IETF RFC 1278](https://datatracker.ietf.org/doc/rfc1278/), drawing on
additional information found in
[IETF RFC 1277](https://datatracker.ietf.org/doc/html/rfc1277).

There are some deviations to the above, however. Since the human-friendly string
syntax was defined, new AFIs were added, including one for directly representing
IP addresses and another for representing URLs. As such this library extends the
specification, but should be completely backwards compatible with it.

You should **not** expect an NSAP decoded from a string to encode back into the
same exact string. You should **not** expect an NSAP decoded from bytes to
encode back into the same exact bytes. You should **not** expect all NSAP
syntaxes to be recognized everywhere; your application and dependencies should
handle unrecognized NSAP syntaxes gracefully.

This crate is `no_std` and supports almost everything with no `alloc` as well;
the only thing `alloc` gives you is better URL encoding and decoding, per
ITU-T Recommendation X.519. Only extremely short URLs are supported without
`alloc`.

## Usage

An NSAP address is represented using `X213NetworkAddress`. You can construct
this using several methods:

- `X213NetworkAddress::from_vec()`
- `X213NetworkAddress::from_vec_unchecked()`
- `X213NetworkAddress::from_slice()`
- `X213NetworkAddress::from_slice_unchecked()`
- `X213NetworkAddress::from_ip()`
- `X213NetworkAddress::from_ipv4()`
- `X213NetworkAddress::from_ipv6()`
- `X213NetworkAddress::from_itot_url()`
- `X213NetworkAddress::from_non_osi_url()`
- `X213NetworkAddress::from_itot_socket_addr()`
- `X213NetworkAddress::try_from()` (for `Vec<u8>` or `&[u8]`)
- `X213NetworkAddress::from_str()`
- `X213NetworkAddress::from()` (for `&IpAddr`, `&Ipv4Addr`, or `&Ipv6Addr`)

The `X213NetworkAddress` type implements `Display`, which prints the NSAP
address according to [IETF RFC 1278](https://datatracker.ietf.org/doc/rfc1278/).

If you need to pass NSAP addresses between heterogeneous systems, consider using
`to_ns_string` or `fmt_as_ns_string()` instead. This will always encode the
NSAP address using the `NS+` syntax like: `NS+A433BB93C1`. These are not
human-friendly, but they are really easy to parse and completely unambiguous.

There is no way in this crate to compare two NSAP addresses. The rationale for
this is documented along with `X213NetworkAddress`, but if you want to compare
the addresses byte-for-byte, you can use `get_octets()`, like so:

```rust,ignore
assert_eq!(addr1.get_octets(), addr2.get_octets());
```

This crate also supports several functions for working specifically with IP
networking, above and beyond those already named, which are:

- `X213NetworkAddress::get_url()`
- `X213NetworkAddress::get_ip()`
- `X213NetworkAddress::get_rfc1277_socket()`

In a TCP/IP only environment, you'll likely want to invoke all three of these
methods until one of them returns a result, then perform the appropriate
connection setup.

Here is an example usage of this crate:

```rust
extern crate alloc;
use alloc::vec::Vec;
use nsap_address::X213NetworkAddress;
use nsap_address::AFI_F69_DEC_LEADING_ZERO;
use core::net::{IpAddr, Ipv4Addr};
use core::str::FromStr;

let addrstr = "TELEX+00728722+RFC-1006+03+255.0.0.2+65535+2";
let addr = X213NetworkAddress::from_str(addrstr).unwrap();
let (_, sock, _) = addr.get_rfc1277_socket().unwrap();
assert_eq!(sock.ip(), &Ipv4Addr::new(255, 0, 0, 2));
assert_eq!(sock.port(), 65535);
let idi_digits: Vec<u8> = addr.idi_digits().unwrap().collect();
assert_eq!(idi_digits.as_slice(), &[ 0, 0, 7, 2, 8, 7, 2, 2 ]);
assert_eq!(addr.get_octets(), &[
  AFI_F69_DEC_LEADING_ZERO, // AFI
  0x00, 0x72, 0x87, 0x22, // Telex 00728722
  0x03, // Network prefix (0x03 = Internet)
  0x25, 0x50, 0x00, 0x00, 0x00, 0x02, // IP address
  0x65, 0x53, 0x50, 0x00, 0x02, // port + tset
]);
let ns_string = addr.to_ns_string();
assert_eq!(ns_string.as_str(), "NS+5400728722032550000000026553500002");
```

## Feature Flags

- `alloc`: heap allocation, which enables URL encoding and decoding for URLs
- `x25`: Parsing of X.25 DSP strings, per
  [IETF RFC 1278](https://datatracker.ietf.org/doc/html/rfc1278)
- `ecma117`: Parsing of ECMA 177 DSP strings, per
  [IETF RFC 1278](https://datatracker.ietf.org/doc/html/rfc1278)
- `nonstd`: Parsing for non-standard syntaxes: those defined by this crate, but
  not in any standard, which are the `IP4`, `IP6`, `URL`, `ICP`, and `IND`.
- `nonstddisplay`: implementation of `std::fmt::Display` for the above
  non-standard syntaxes. This is a separate feature flag so that, if you have an
  application that passes NSAPs as strings, you can ensure that you only use
  standardized syntaxes, while still being able to parse the non-standard ones.

`alloc` is the only one enabled by default. Keeping `x25` off is a good idea
unless, somehow, you are still using or working with X.25 networks. The ECMA 117
syntax was not widely used, so you are unlikely to encounter that as well; for
this reason, you might want to keep `ecma117` off. `nonstd` and `nonstddisplay`
will be big quality-of-life improvements for you in almost all cases.

## AI Usage Statement

No code, tests, or documentation was produced by AI or an LLM of any kind,
except for two small internal functions in `utils`, which were reviewed by the
crate's author.

## Deviations from IETF RFC 1278

- `ICP` and `IND` AFIs recognized in the `<afi>-<idi>-<dsp>` syntax
- `IP4`, `IP6`, and `URL` schemes supported in `FromStr` and `Display`

## To Do

- [ ] Use `faster_hex` instead
- [x] Other socket address encoding functions
- [x] Remaining TODO / FIXME
- [x] Re-name things?
- [ ] One last format
- [ ] One last test and fuzz testing
- [ ] Build with stable
- [ ] likely / unlikely
- [x] Readme Documentation

## To Do (Future)

- [ ] Support [GOSIP NSAP addressing](https://medium.com/@jacstech/jacs-nsap-structure-8cb9a809228b)
