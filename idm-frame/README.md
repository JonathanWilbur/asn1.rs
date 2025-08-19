# IDM Frame Utilities

ITU-T Recommendation X.519 (2019) defines a protocol, Internet Directly-Mapped
(IDM) Protocol, which is used to provide the OSI ROSE service and all lower
layers of the OSI networking stack over TCP/IP, specifically for the purpose of
making the X.500 directory easily available over the Internet.

There are two versions of the IDM protocol defined. This crate supports both.
The second version supports negotiation of the transfer syntax, whereas the
first uses the Basic Encoding Rules (BER).

This crate does not handle decoding of the contained `IDM-PDU` ASN.1 data
structure: it only handles the frames.

## This crate

This crate is `no_std` and does not require `alloc`, except for running the
tests. This crate was not written in any part by AI or LLMs. This crate was
fuzz-tested. It has no dependencies. Extremely simple and light-weight.

## Usage

To read an IDM frame, simply create an `IdmFrameIter`, and iterate over it one
or more times.

```rust
let buffer: Vec<u8> = Vec::from([
    1, 1, 0, 0, 0, 5, 1, 2, 3, 4, 5,
].as_slice());
let mut frames = IdmFrameIter::new(buffer.as_slice());
let frame = frames.next()
  .expect("uh oh, no frame")
  .expect("uh oh, unrecognized / unsupported version");
assert_eq!(frame.version, 1);
assert_eq!(frame.final_, 1);
assert_eq!(frame.encoding, 0);
assert_eq!(frame.data, [1,2,3,4,5].as_slice());
```

This crate also provides two fairly trivial functions for helping you write
valid IDM frames: `write_idm_v1_frame_header` and `write_idm_v2_frame_header`.
These can be used like so:

```rust
let mut buf: [u8; IDMV2_FRAME_SIZE] = [0, 0, 0, 0, 0, 0, 0, 0];
write_idm_v2_frame_header(&mut buf, true, IDM_ENCODING_XER, 1024);
assert_eq!(buf, [2, 1, 0x10, 0, 0, 0, 4, 0]);
```

Cheers!
