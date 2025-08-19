# ISO Transport over TCP (ITOT) TPKT Decoding

Functionality for handling ISO Transport over TCP (ITOT) TPKT packets as
described (repeatedly) in:

- [IETF RFC 983](https://datatracker.ietf.org/doc/html/rfc983#section-6)
- [IETF RFC 1006](https://datatracker.ietf.org/doc/html/rfc1006#section-6)
- [IETF RFC 2126](https://datatracker.ietf.org/doc/html/rfc2126#section-4.3)

This is apparently also used in Signalling System No. 7 (SS7).

## This crate

This crate is `no_std` and does not require `alloc`. This crate was not written
in any part by AI or LLMs. It has no dependencies.

## Usage

Read TPKT frames from a buffer like so:

```rust
use tpkt3::TpktIter;
let buffer = [
    3u8, 0, 0, 5, 1, 2, 3, 4, 5,
    3u8, 1, 0, 4, 5, 4, 3, 2, 1,
];
let mut tpkts = TpktIter::new(buffer.as_slice());
for tpkt in tpkts {
  // ...presumably, decode the TPDU from tpkt.tpdu.
}
```

After iterating over your TPKTs, there may be some trailing bytes in the buffer
that have not been consumed. Use `TpktIter::bytes_unread()` to know how many
remain.
