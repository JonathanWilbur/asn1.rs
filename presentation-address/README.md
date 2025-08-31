# OSI Presentations Addresses

Data type for the `PresentationAddress` data structure as defined in
[ITU-T Recommendation X.520](https://www.itu.int/itu-t/recommendations/rec.aspx?rec=X.520).
This is a data structure used to identify applications running over OSI
networking protocol stacks, but it has been modified in later versions of
ITU-T Rec. X.520 to also support IP networking.

## Presentation Address Semantics

A presentation address is defined (in ASN.1) as:

```asn1
PresentationAddress ::= SEQUENCE {
  pSelector   [0]  OCTET STRING OPTIONAL,
  sSelector   [1]  OCTET STRING OPTIONAL,
  tSelector   [2]  OCTET STRING OPTIONAL,
  nAddresses  [3]  SET SIZE (1..MAX) OF OCTET STRING,
  ... }
```

The ordering of N-addresses is not significant.

The RFC 1278 syntax expects all lower-layer selectors to be defined whenever a
given OSI layer has a selector defined, but the ASN.1 syntax does not require
this. It also appears that an empty selector (the "NIL selector") is technically
different from an absent selector, per
[ITU-T Rec. X.650](https://www.itu.int/rec/T-REC-X.650-199201-S), section 9.5.2.
So basically, if, say, the P-selector is present, but the S-selector is not,
there is no valid way to represent this in the RFC 1278 string.

However, upon reading the X.650 passage above (which is hardly comprehensible),
it seems like the the NIL-selector is not really different from an absent
selector for connection establishment, so it seems to me that, when converting
a `PresentationAddress` to an RFC 1278 string, using the empty string should be
considered equivalent to an absent string. Hence, this is what this crate
does.

## This Crate

This was split off from a more general X.500 crate into a separate crate because
it is used in X.500 and X.400 specifications, and probably in other places, too.

This crate is `no_std`, but requires `alloc`, since the central data type,
`PresentationAddress`, uses `Vec<u8>` to represent selectors.
Maybe a future version could use some sort of inlining approach, since most
selectors are very small (e.g. two bytes or so), then maybe this crate won't
need `alloc` either (but it would fail if it encountered selectors too large).

This crate was fuzz-tested.

## AI Usage Statement

No code, tests, or documentation was produced by AI or an LLM of any kind,
except for two small sections of code for parsing selector strings, which were
reviewed by the crate's author.

## Usage

There's not really much to this library other than `PresentationAddress`. You
can print it, since it implements `std::fmt::Display`, you can parse them, since
it implements `std::str::FromStr`. Both of these implementations adhere to
[IETF RFC 1278](https://datatracker.ietf.org/doc/html/rfc1278).

You cannot compare them using an implementation of `PartialEq`, `Eq`, or `Hash`.
The rationale for this is that NSAP addresses are basically boundlessly complex
where a byte-for-byte comparison would likely be technically incorrect--though
good enough for most use cases. It is also not obvious whether two
`PresentationAddress`es should be considered equal if one's N-addresses are a
subset of the other's, or only if both are exactly identical.

X.520's `presentationAddressMatch` matching rule considers a match to be subset,
but this might not be correct for your use case. It is better to leave the
implementation choice to be an explicit decision made by the crate's user.

To this end, you have two options provided by this crate:

- `PresentationAddress::is_naively_subset_of()`
- `PresentationAddress::is_naively_exactly()`

Both of these naively compare N-addresses byte-for-byte, even though there could
exist multiple ways to represent the same underlying address. The former only
checks that the `self` `PresentationAddress` has a subset of the N-addresses
that the `other` has, whereas the latter expects both to have the exact same
(though they may still appear in any order).

## Feature Flags

- `nsap-address`: Without this feature flag, only N-addresses of the
  `NS+<hex>` form can be parsed and displayed. This enables the whole range of
  [IETF RFC 1278](https://datatracker.ietf.org/doc/html/rfc1278) NSAP address
  string syntaxes to be parsed and displayed.
- `x690`: Enable encoding, decoding, and validating the encoding of
  `PresentationAddress` according to the Basic Encoding Rules (BER) defined
  in
  [ITU-T Recommendation X.690](https://www.itu.int/itu-t/recommendations/rec.aspx?rec=X.690)

The feature flags `nonstd`, `nonstddisplay`, `x25`, and `ecma117` are simply
forwarded to `nsap-address`. You should read
[the documentation](https://crates.io/crates/nsap-address#feature-flags) on
these within that crate. These also just affect the display and string parsing.
