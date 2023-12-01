# OSI Networking in Rust

This is a full implementation of OSI Networking in Rust. When completed, it will
support connection-mode and connectionless-mode, all X.224 transport classes,
all functional units of the session and presentation protocols, as well as
monitoring, logging, debugging, features. LPP, ITOT, TP4 over IP, and XOT will
be supported as protocol stacks, possibly in addition to a
Protocol-Buffers-based presentation service.

## To Do

- [ ] X.224 Transport Protocol
  - [ ] TPDU Encoding
    <!-- - [ ] `.write_nsdu_vectored(vs: &[&[u8]])` -->
    - [ ] `TPDU.write(w: W)`
      - This might actually work fine. The number/length of vectors in an output DT-TPDU can be known easily.
      - (Fixed part + Checksum + ED-TPDU-NR) is the first vector. User data is the second.
      - Heaplesslessly allocation a `[&[u8]; 2]` and call
      - I think user data `Cow<[u8]>` is the answer:
        - Fast slice-based parsing of inbound PDUs
        - Owned data for outbound PDUs, meaning that the user data can be sent to a tokio task for writing out.
      - As a hack the `DT` TPDU could also have an `spdu_header: Cow<[u8]>`,
        which makes `user_data` interpreted as the user data that follows the
        `DATA-TRANSFER` SPDU.
        - `simply_encoded_pdata: bool`
          - Makes `user_data` interpreted as `Simply-encoded-data`
        - `pdv_list: &[&[u8]]`
        - `fully_encoded_pdata: &[&[u8]]`
        - In fact, maybe you could even pass in PPDUs as `&[&[u8]]` (use indefinite encoding) (`(DT-SPDU, PPDU, PDVs, EOC)`)
    - [ ] `.write_nsdu_parts(bufs: &[&[u8]])`
    - [ ] `TPDU.to_bufs() -> &[&[u8]]`
  - [ ] Checksum Verification
  - [ ] State Tables / Protocol Machine
- [ ] X.225 Session Protocol
- [ ] X.226 Presentation Protocol
- [ ] X.227 Association Control Service Element (ACSE)
- [ ] X.882 Remote Operation Service Element (ROSE)
- [ ] ISO Transport over TCP (RFC 1006 / RFC 2126)
- [ ] Lightweight Presentation Protocol ([IETF RFC 1085](https://datatracker.ietf.org/doc/html/rfc1085))
- [ ] TP4 over IP
- [ ] X.25 over TCP (XOT) ([IETF RFC 1613](https://datatracker.ietf.org/doc/html/rfc1613))
- [ ] X.224 Transport over Connectionless Network
- [ ] Connectionless-Mode Stack
- [ ] X.224 Annex B Network Connection Management Subprotocol
- [ ] C ABI
- [ ] Congestion Detection
- [ ] DoS Prevention
- [ ] Logging
- [ ] Use the `parking_lot` crate for faster sync primitives
- [ ] An API for running an OSI initiator or responder that shares no resources
      and therefore has no locks or reference counting, etc.
      - This is useful for when each underlying TCP connection runs in its own
        completely isolated OSI environment rather than sharing a ref namespace.
        I think this categorically rules out multiplexing and possibly
        splitting, so it may be that only classes 0 and 1 are allowed in this case.

## Notes

- ISODE uses the process ID to determine the DST-REF of the responder, which
  seems to mean that it will be reused among connections, which seems to
  conflict with my implementation. The verbiage of X.224, Section 6.5.4,
  suggests that each ref must be unique, so I think this detail is incorrect.
  It also makes sense that each must be unique to support splitting.

## Network Service Implementation

I have established that there will be separate `OSINetworkConn` and `NSProvider`
traits. I got hung up because I tried to make `NSProvider` make sense for both,
but they are two logically different things: when trying to make a "layer" make
use of a particular connection, there was an unanswered "which?" and when trying
to make a network connection make another network connection (via `N-CONNECT`
request), there was an unanswered "how?". Now I believe that both should have
separate traits.

Further, the state tables for the transport layer should receive the whole
application association stack, not just N, T, and S. This is because:

1. You have to pass in the AA anyway so you can enqueue the creation of new
   network connections.
2. You cannot take a mutable reference to AA, then to S, which is one of its
   fields.

This is unique to this layer and should not be a problem for the upper layers,
because creating a new network connection

WAIT, are you sure? Actually, I think the AA will have to be passed into every
state table at every layer. How will S know where to get P?