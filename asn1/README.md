# Abstract Syntax Notation 1 (ASN.1) Library

This library contains types and functions for using ASN.1 values. This library
has little to do with encoding or decoding such values according to
the Basic, Canonical, Distinguished, Packed, Octet, XML, JSON, General String,
or BACNet Encoding Rules. This library is purposely abstracted from any
particular encoding so that ASN.1 values can be translated between different
encodings.

## Showcase

What follows is a showcase of what can be done with this library. Note that I
haven't tested with all of the examples below; they might not work.

### Variable-Length Integer Decoding

Decode variable-length integers, which are used in multiple different encodings:

```rust
assert_eq!(read_i64(&[ 0x01, 0x05 ]), Ok(256 + 5))
assert_eq!(read_i128(&[ 0x01, 0x05 ]), Ok(256 + 5))
```

### String Validation

You can validate strings to be of type `PrintableString` or `NumericString`.

```rust
assert!(is_printable_str("Testeroo"));
assert!(!is_printable_str("Book with 'F*ck' in the title"));
assert!(is_numeric_str("0280 6082 0502"));
assert!(!is_numeric_str("deadbeef"));
```

### `OBJECT IDENTIFIER`

You can create, parse, print, compare, check prefixes and suffixes of
`OBJECT IDENTIFIER`s:

```rust
let oid1 = OBJECT_IDENTIFIER::from_str("1.3.6.4.1").unwrap();
let oid2 = oid!(1,3,6,4,1);
assert_eq!(oid1, oid2);
assert_eq!(oid1.to_string(), "1.3.6.4.1");
assert_eq!(oid1.to_asn1_string(), "{ 1 3 6 4 1 }");
assert_eq!(oid1.to_iri_string(), "/1/3/6/4/1");
assert!(oid1.starts_with(&roid!(1,3,6,1)));
assert!(oid1.ends_with(&roid!(6,4,1)));
```

### `RELATIVE-OID`

The features for `RELATIVE-OID` are nearly the same as those for
`OBJECT IDENTIFIER`:

```rust
let roid1 = RELATIVE_OID::from_str("1.3.6.4.1").unwrap();
let roid2 = roid!(1,3,6,4,1);
assert_eq!(roid1, roid2);
assert_eq!(roid1.to_string(), "1.3.6.4.1");
assert!(roid1.starts_with(&roid!(1,3,6,1)));
assert!(roid1.ends_with(&roid!(6,4,1)));
```

### `UTCTime`

You can parse values of `UTCTime` and print them back as strings or convert them
to ISO 8601 timestamps:

```rust
let t1 = UTCTime::from_str("010203040506+0415").unwrap();
assert_eq!(t1.to_iso_8601_string(), "2001-02-03T04:05:06+0415");
assert_eq!(t1.to_string(), "010203040506+0415");
```

There are no functions for comparison or ordering defined in this library,
because this is not a time library, and I intentionally want to keep this very
core library very slim in terms of dependencies. You can easily pass the fields
of the `UTCTime` struct into a time library-specific data structure for
equation and ordering.

### `GeneralizedTime`

You can parse values of `UTCTime` and print them back as strings or convert them
to ISO 8601 timestamps:

```rust
let t1 = GeneralizedTime::from_str("20210203040607.32895292-0503").unwrap();
assert_eq!(t1.to_iso_8601_string(), "2021-02-03T04:06:07.32895292-0503");
assert_eq!(t1.to_string(), "20210203040607.32895292-0503");
```

### `DATE`

You can parse, print, compare, and sort `DATE` values:

```rust
let d1 = DATE::from_str("2022-04-23").unwrap();
let d2 = DATE::new(2022, 04, 23);
let d3 = DATE::new(2022, 04, 24);
assert_eq!(d1.year, 2022);
assert_eq!(d1.month, 4);
assert_eq!(d1.day, 23);
assert_eq!(d1.to_string(), "2022-04-23");
assert_eq!(d1, d2);
assert!(d3 > d2);
```

### `TIME-OF-DAY`

You can parse, print, compare, and sort `TIME-OF-DAY` values:

```rust
let t1 = TIME_OF_DAY::from_str("20:19:18").unwrap();
let t2 = TIME_OF_DAY::new(20, 19, 18);
let t3 = TIME_OF_DAY::new(20, 19, 19);
assert_eq!(t1.hour, 20);
assert_eq!(t1.minute, 19);
assert_eq!(t1.second, 18);
assert_eq!(t1.to_string(), "20:19:18");
assert_eq!(t1, t2);
assert!(t3 > t2);
```

### `DATE-TIME`

You can parse, print, compare, and sort `DATE-TIME` values:

```rust
let d1 = DATE_TIME::from_str("2022-04-23T20:19:18").unwrap();
let d2 = DATE_TIME::new(2022, 04, 23, 20, 19, 18);
let d3 = DATE_TIME::new(2022, 04, 24, 20, 19, 18);
let d4 = DATE_TIME::new(2022, 04, 23, 20, 19, 19);
assert_eq!(d1.date.year, 2022);
assert_eq!(d1.date.month, 4);
assert_eq!(d1.date.day, 23);
assert_eq!(d1.time.hour, 20);
assert_eq!(d1.time.minute, 19);
assert_eq!(d1.time.second, 18);
assert_eq!(d1, d2);
assert!(d3 > d2);
assert!(d4 > d2);
```

### `DURATION`

You can parse and print `DURATION` values (via the `DURATION_EQUIVALENT` struct):

```rust
let dur = DURATION_EQUIVALENT::from_str("P5Y6M1W23DT25H65M222.00505S").unwrap();
assert_eq!(dur.years, 5);
assert_eq!(dur.months, 6);
assert_eq!(dur.weeks, 1);
assert_eq!(dur.days, 23);
assert_eq!(dur.hours, 25);
assert_eq!(dur.minutes, 65);
assert_eq!(dur.seconds, 222);
assert_eq!(dur.fractional_part, Some(FractionalPart { number_of_digits: 5, fractional_value: 505 }));
assert_eq!(dur.to_string(), "P5Y6M1W23DT25H65M222.00505S");
```

### `BIT STRING`

You can create new `BIT STRING`s using multiple techniques, get and set bits,
get the length, print as strings, and join `BIT STRINGS`:

```rust
let bs1 = BIT_STRING::new();
let bs2 = BIT_STRING::with_capacity(8); // Units in bits
let bs3 = BIT_STRING::from_bin("100111010101");
let bs4 = BIT_STRING::from_bits(&[ 0b1110_1101, 0b1010_0101 ]);
let bs5 = BIT_STRING::from_bytes(vec![ 1, 7 ]); // Produces 0b0000_0001 0b0000_0111
let bs6 = BIT_STRING::with_bits_set(&[ 1, 3, 5, 7 ]); // Produces 0b0101_0101
assert_eq!(bs6.get(0), Some(false));
bs6.set(0, true); // Returns true if the bit string was extended. Produces 0b1101_0101.
assert_eq!(bs6.get(0), Some(true));
assert_eq!(bs6.len(), 8);
assert_eq!(bs6.to_string(), "'11010101'B"); // This is the ASN.1 notation.
assert_eq!(join_bit_strings(&[ bs1, bs2 ]), bs1); // Both are empty.
```

### `SEQUENCE` or `SET` parsing features

This is not a complete solution, but this library provides `TagSelector` for
defining sequences of rules for matching components in a `SEQUENCE` or `SET`
data types, and `ComponentSpec` for more data, such as the component's name and
whether it is `OPTIONAL`.

### Context-Switching Types

This library defines `EXTERNAL`, `EMBEDDED_PDV`, and `CHARACTER_STRING`. All of
them can be printed.

### Detailed Errors

`ASN1Error`s are very detailed, printable, and you should be able to figure out
whatever your problem is based on the output:

```rust
pub struct ASN1Error {
    pub error_code: ASN1ErrorCode,
    pub component_name: Option<String>,
    pub tag: Option<Tag>,
    pub length: Option<usize>,
    pub constructed: Option<bool>,
    pub value_preview: Option<String>,
    pub bytes_read: Option<usize>, // The number of bytes into the IO read stream where this error appeared.
    pub values_read: Option<usize>, // The number of ASN.1 values into the IO read stream where this error appeared.
    pub io_error: Option<Error>,
}
```

### ASN.1 Types and Values

ASN.1 abstract values can be represented via `ASN1Value`. This is a pretty large
and complex enum. You probably should not use it unless you are parsing ASN.1
abstract values. When decoding presentation syntax (such as BER or DER), you
probably should represent presentation values using the decoding libraries
concept of a "node" or "value," whatever that might be.

```rust
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

In embedded environments, it might be better for you to use encoding-specific,
simpler libraries that do the bare minimum.
[This](https://crates.io/crates/asn1_der) might be a suitable library for you
in this case.

## Versioning

This is currently in beta. Be prepared for potentially large breaking changes
until `1.0.0` is released, after which I will adhere to semantic versioning.

However, I have successfully built applications using this library, so it is
somewhat battle-tested.

## To Do

- [ ] Fuzz testing
- [x] Normalize `NumericString`
- [x] To and from `NumericString` values for time types
  - This is for X.690 encoding, primarily
- [x] ~~Convertion to and from `std::time::Duration`~~ (No unambiguous way)
- [ ] I think you can replace `% 8` with a bit mask, which would be more performant
- [ ] `no-std` and other feature flags
- [ ] Documentation comments
- [ ] `README.md` documentation
- [ ] Debug assertions / debug logging?
