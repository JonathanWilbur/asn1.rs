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
