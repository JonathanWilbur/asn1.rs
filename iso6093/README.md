# ISO 6093 Real Number Parsing and Printing in Rust

[ISO 6093:1985](https://www.iso.org/standard/12285.html) is a standard for
representing real numbers textually. It defines three forms: NR1, NR2, and NR3.
These are used in ASN.1 encodings: specifically, in encoding `REAL` values using
the Basic Encoding Rules (BER), Canonical Encoding Rules (CER), or
Distinguished Encoding Rules (DER) as defined in
[ITU-T Recommendation X.690](https://www.itu.int/rec/T-REC-X.690/en), but they
could be used elsewhere.

This library implements functions for parsing and printing all three forms. It
is a `no-std`-compatible crate and has no dependencies.

## Overview

All formats defined in ISO 6093:1985 support leading SPACE characters and, if
the signed variants are used, a positive or negative sign. All formats support
using either the PERIOD or COMMA characters as the decimal mark, if a decimal
mark is used.

First Numeric Representation (NR1) is for representing integers only. This crate
stores integers on an `f64` just for consistency with the other variants.

Second Numeric Representation (NR2) is the explicit-point real number. The value
is simply printed using decimal digits, a decimal mark, and then the fractional
digits. This is also a good format to use when it is known that the number will
never be so large or small that it cannot be easily printed.

Third Numeric Representation (NR3) is "scientific notation."

## Usage

You can parse numerical values having a particular format via `parse_nr1`,
`parse_nr2` and `parse_nr3`.

```rust
assert_eq!(parse_nr1(s!(" 0004902")), Ok(4902.0));
assert_eq!(parse_nr1(s!("   +1234")), Ok(1234.0));
assert_eq!(parse_nr1(s!("    1234")), Ok(1234.0));
assert_eq!(parse_nr1(s!("  -56780")), Ok(-56780.0));
assert_eq!(parse_nr2(s!("  1237,0")), Ok(1237.0));
assert_eq!(parse_nr2(s!("+0.00001")), Ok(0.00001));
assert_eq!(parse_nr2(s!("-5,67800")), Ok(-5.67800));
assert_eq!(parse_nr3(s!("+5.6e+03")), Ok(5600.0));
assert_eq!(parse_nr3(s!("+0,3E-04")), Ok(0.00003));
assert_eq!(parse_nr3(s!(" 0,3e-04")), Ok(0.00003));
assert_eq!(parse_nr3(s!("-2,8E+00")), Ok(-2.8));
```

Or, if which representation (NR1, NR2, or NR3) is unknown, you can try to parse
any one of them using `parse_iso6093`:

```rust
assert_eq!(parse_iso6093(s!("0001234")), Ok(ISO6093RealNumber::NR1(1234.0)));
assert_eq!(parse_iso6093(s!("-05,6780")), Ok(ISO6093RealNumber::NR2(-5.6780)));
assert_eq!(parse_iso6093(s!("+0,3E-04")), Ok(ISO6093RealNumber::NR3(0.00003)));
```

You can also print values as such:

```rust
assert_eq!(ISO6093RealNumber::NR1(-123.0).to_string().as_str(), "-123");
assert_eq!(ISO6093RealNumber::NR2(0.00123).to_string().as_str(), "0.00123");
assert_eq!(ISO6093RealNumber::NR3(0.00123).to_string().as_str(), "1.23e-3");
```

`ISO6093RealNumber` implements `Display` and `Debug`, which can be used to
print it as well.

## No Alloc Usage

This library is `no-std` by default, but it is _not_ zero-allocation by
default. You can make it zero-allocation by turning off the default `alloc`
feature flag, but this causes the API to change in the following manner: the
functions `parse_nr2`, `parse_nr3`, and `parse_iso6093` all change
to take a `&mut str` instead of a `&str`. The reason for this is that, to parse
the numbers, the string has to be "normalized" to change commas into periods.
With no way to allocate a new string, the only other option is to modify the
original. These implementations do NOT change it back.

## To Do

- [ ] Format
