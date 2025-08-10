# PKI-Stub ASN.1 Data Structures and Functionality

## To Do

- [x] `AttributeValue::eq` should decode DNs
- [x] `Hash`
- [x] `PartialEq` / `Eq`
- [x] `Ord` for `Time`
- [x] `AttributeCertificate`
- [x] `TBSCertAVL`
- [ ] `into_iter()` cleanup
- [ ] Inlining
- [ ] Tests
  - [ ] Public Key Certificate Decoding
  - [ ] Attribute Certificate Decoding
- [ ] Documentation Comments
- [ ] Readme Documentation

## Examples

```bash
cargo run --example attr-cert
```

This will generate a minimal X.509 attribute certificate.
