## ITU-T Recommendation T.61 Teletex-to-Unicode

ITU-T Recommendation T.61 specifies the T.61 character set for use in an
ancient telecommunications service called "Teletex." For this reason, the
words "Teletex" and "T.61" are often used interchangeably when talking about
character encodings, including by the ITU-T themselves.

Teletex character encodings are scarcely used anymore, except in ASN.1
specifications, but even then, it appears in legacy ASN.1 protocols and even
there, most implementations just treat Teletex as ASCII. This is mostly
accurate, since the first 127 characters overlaps with ASCII, although some
characters that are defined in ASCII are not defined in T.61.

## This crate

This crate is `no_std` and was not written in any part by AI or LLMs. This
crate was fuzz-tested. It has no dependencies.

## Usage

To convert the bytes of a T.61 string into UTF-8, you should prefer to
iterate character-by-character, if possible, so as to avoid any heap
allocation, like so:

```rust
use teletex::teletex_to_utf8;
let input = b"Big\xA4Money\xA4";
for utf8char in teletex_to_utf8(input) {
    // ...utf8char is the UTF-8 equivalent
}
```

However, if you actually need an allocated string, you can do this:

```rust
let input = b"Big\xA4Money\xA4";
let output: String = teletex_to_utf8(input).collect();
assert_eq!(output.as_str(), "Big$Money$");
```
