# PKI-Stub ASN.1 Data Structures and Functionality

## To Do

- [ ] `Hash`
  - [ ] `AttributeTypeAndValue` - Requires stringprep
  - [ ] `Name`
  - [ ] `GeneralName`
  - [ ] `Certificate`
  - [ ] `AttributeCertificate`
  - [ ] `CertAVL`
  - [ ] `IssuerSerialNumber`
  - [ ] `PKCertIdentifier`
- [x] `PartialEq` / `Eq`
- [ ] `TBSCertificate` / `Certificate`
  - [ ] `.matches_issuer_serial()`
  - [ ] `.claims_to_be_issued_by_cert(cert)`
    - Pass in the whole cert so `subjectAltNames` can be checked for a match.
- [ ] `AttributeCertificate`
  - [ ] `.is_role_spec_cert()`
  - [ ] `.is_soa()`
  - [ ] `.claims_to_be_issued_by_cert(cert)`
- [ ] `TBSCertAVL`
  - [ ] `.claims_to_be_issued_by_cert(cert)`
- [ ] `into_iter()` cleanup
- [ ] Inlining
- [ ] Tests
  - [ ] Public Key Certificate Decoding
  - [ ] Attribute Certificate Decoding
- [ ] Documentation Comments
- [ ] Readme Documentation
