# Abstract Syntax Notation 1 (ASN.1) Library

This crate contains types and functions for using ASN.1 values. This library
has little to do with encoding or decoding such values according to
the Basic, Canonical, Distinguished, Packed, Octet, XML, JSON, General String,
or BACNet Encoding Rules. This library is purposely abstracted from any
particular encoding so that ASN.1 values can be translated between different
encodings. Other crates will use this library as a dependency to implement
encoding and decoding for the types defined in this crate.

This crate is intended to be high-performance and opines toward _more code_ in
exchange for _better performance_, notably using a lot of SIMD and other
optimizations. This is **not** a lightweight crate; it is intended to be a fast,
feature-complete, meticulous, and well-tested ASN.1 crate.

## Features

All universal types are supported, including the newer `TIME` subtypes: `DATE`,
`DATE-TIME`, `TIME-OF-DAY`, and `DURATION`.

### `SEQUENCE` or `SET` parsing features

This is not a complete solution, but this library provides `TagSelector` for
defining sequences of rules for matching components in a `SEQUENCE` or `SET`
data types, and `ComponentSpec` for more data, such as the component's name and
whether it is `OPTIONAL`.

### Context-Switching Types

This library defines `EXTERNAL`, `EMBEDDED_PDV`, and `CHARACTER_STRING`. All of
them can be printed.

### Display

All types defined in this crate implement `Display`, and when printed, they are
printed according to their ASN.1 _abstract_ syntax (which may differ from how
they are encoded).

### Detailed Errors

`ASN1Error`s are very detailed, printable, and you should be able to figure out
whatever your problem is based on the output:

```rust,ignore
pub struct ASN1Error {
    pub error_code: ASN1ErrorCode,
    pub component_name: Option<String>,
    pub tag: Option<Tag>,
    pub length: Option<usize>,
    pub constructed: Option<bool>,
    pub value_preview: Option<String>,
    pub bytes_read: Option<usize>, // The number of bytes into the IO read stream where this error appeared.
    pub values_read: Option<usize>, // The number of ASN.1 values into the IO read stream where this error appeared.
    pub err_source: Option<Box<dyn std::error::Error + 'static>>,
}
```

Of course, they implement `std::error::Error`.

### ASN.1 Types and Values

ASN.1 abstract values can be represented via `ASN1Value`. This is a pretty large
and complex enum. You probably should not use it unless you are parsing ASN.1
abstract values. When decoding presentation syntax (such as BER or DER), you
probably should represent presentation values using the decoding libraries
concept of a "node" or "value," whatever that might be.

```rust,ignore
pub enum ASN1Value {
    BitStringValue(BIT_STRING),
    BooleanValue(BOOLEAN),
    ChoiceValue(Box<ASN1Value>),
    EmbeddedPDVValue(EMBEDDED_PDV),
    EnumeratedValue(ENUMERATED),
    ExternalValue(EXTERNAL),
    InstanceOfValue(INSTANCE_OF),
    IntegerValue(INTEGER),
    IRIValue(OID_IRI),
    NullValue,
    /// ...
}
```

## Embedded and `no-std` Use

This library is not ideal for embedded use cases. It is intended to be complete
and compliant, not lightweight. Every valid ASN.1 requirement and capability is
expected to be supported correctly, and if it is not, please report it as a bug.

This library is not `no-std` and probably never will be, because so many things
in this library rely on `std`, and so few things outside of that are useful.

In embedded environments, it might be better for you to use encoding-specific,
simpler libraries that do the bare minimum.
[This](https://crates.io/crates/asn1_der) might be a suitable library for you
in this case. Also consider that, since this library is licensed under an FLOSS
license, you could just copy whatever code you need into your embedded project.

## `serde` Integration

I don't think this crate will support integration with `serde`, unless this is
requested. Its intended design is for dependent crates to implement most of the
encoding-specific logic.

## Fuzz Testing

In the root of this monorepo, run:

```sh
cargo fuzz run bitstring
cargo fuzz run date -- -max_len=10
cargo fuzz run datetime -- -max_len=20
cargo fuzz run time_of_day -- -max_len=8
cargo fuzz run asn1utils -- -max_len=8
cargo fuzz run duration -- -max_len=32
cargo fuzz run gentime -- -max_len=35
cargo fuzz run utctime -- -max_len=20
cargo fuzz run oid -- -max_len=24
```

These will run forever, so you will want to kill them by pressing `Ctrl+C`.

## Versioning

This is currently in beta. Be prepared for potentially large breaking changes
until `1.0.0` is released, after which I will adhere to semantic versioning.

However, I have successfully built applications using this library, so it is
somewhat battle-tested.

## To Do

- [ ] Document feature flags
  - [ ] Document that `smallvec` is disabled by default, because it _never_ outperformed a normal `Vec`.
- [ ] Benchmarking
- [ ] Update dependencies one last time
