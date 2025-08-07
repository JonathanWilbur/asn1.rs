# PKI-Stub ASN.1 Data Structures and Functionality

## To Do

- [x] `AttributeValue::eq` should decode DNs
- [x] `Hash`
- [x] `PartialEq` / `Eq`
- [ ] `TBSCertificate` / `Certificate`
  - [ ] `.matches_issuer_serial()`
  - [ ] `.iter_subject_names()`
  - [ ] `.iter_issuer_names()`
  - [ ] `.supports_key_usage()`
  - [ ] `.supports_ext_key_usage()`
  - [ ] `.iter_subject_directory_attributes()`
  - [ ] `.iter_associated_information()`
  - [ ] `.claims_to_be_issued_by_cert(cert)`
  - [ ] `.get_basic_constraints()`
  - [ ] `.get_private_key_usage_period()`
  - [ ] `.inhibit_any_policy_depth()`
  - [ ] `.was_valid_as_of()`
  - [ ] `.get_subject_key_id()`
    - Pass in the whole cert so `subjectAltNames` can be checked for a match.
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
