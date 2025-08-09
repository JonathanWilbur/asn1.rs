# PKI-Stub ASN.1 Data Structures and Functionality

## To Do

- [x] `AttributeValue::eq` should decode DNs
- [x] `Hash`
- [x] `PartialEq` / `Eq`
- [ ] `Ord` for `Time`
- [ ] `AttributeCertificate`
  - [ ] `.claims_to_be_issued_by_cert(cert)`
  - [ ] `.is_role_spec_cert()`
  - [ ] `.is_soa()`
  - [ ] `.is_single_use()`
  - [ ] `.is_group_ac()`
  - [ ] `.has_no_rev_avail()`
  - [ ] `.can_assert()` (no `noAssertion` extension)
  - [ ] `.is_indirect_issuer()`
  - [ ] `.get_basic_constraints()`
  - [ ] `.iter_acceptable_cert_policies()`
- [ ] `TBSCertAVL`
  - [ ] `.claims_to_be_issued_by_cert(cert)`
- [ ] `into_iter()` cleanup
- [ ] `GeneralName`
  - [ ] To `AltNameType`
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
