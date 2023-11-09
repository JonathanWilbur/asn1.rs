# OSI Networking in Rust

This is a full implementation of OSI Networking in Rust. When completed, it will
support connection-mode and connectionless-mode, all X.224 transport classes,
all functional units of the session and presentation protocols, as well as
monitoring, logging, debugging, features. LPP, ITOT, TP4 over IP, and XOT will
be supported as protocol stacks, possibly in addition to a
Protocol-Buffers-based presentation service.

## To Do

- [ ] X.224 Transport Protocol
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
