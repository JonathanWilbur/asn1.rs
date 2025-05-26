# ISO 6093 Real Number Parsing and Printing in Rust

[ISO 6093:1985](https://www.iso.org/standard/12285.html) is a standard for
representing real numbers textually. It defines three forms: NR1, NR2, and NR3.
These are used in ASN.1 encodings: specifically, in encoding `REAL` values using
the Basic Encoding Rules (BER), Canonical Encoding Rules (CER), or
Distinguished Encoding Rules (DER) as defined in
[ITU-T Recommendation X.690](https://www.itu.int/rec/T-REC-X.690/en).

This library implements functions for parsing and printing all three forms. It
is a `no-std`-compatible crate.
