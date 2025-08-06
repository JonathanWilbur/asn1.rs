# X.500

Implementation of the ASN.1 data structures and PDUs defined in the
X.500 series of recommendations published by the
[International Telecommunication Union's (ITU)](https://www.itu.int/en/Pages/default.aspx).

This library only implements the ASN.1 data structures and PDUs defined in these
specifications, as well as subset of the business logic, such as:

- Matching Rules
- Context Matching
- Stringifying
- Comparison
- Filter evaluation

The protocols whose PDUs and data structures are implemented in this library
are (inclusively):

- Directory Access Protocol (DAP) - Which is used for reading X.500 directories.
- Directory Information Shadowing Protocol (DISP) - Which is used for replicating X.500 directories.
- Directory System Protocol (DSP) - Which is used for X.500 directories to read other X.500 directories.
- Directory Operational Binding Management Protocol (DOP) - Which manages rules for how X.500 directories interact.
- Certificate Authority Subscription Protocol (CASP)
- Authorization Validation Management Protocol (AVMP)
- Trust Broker Protocol (TBP)

## Footnote

These libraries were generated entirely or in part by the
[ASN.1 Compilation Service](https://wildboarsoftware.com/asn1-compilation)
offered by [Wildboar Software](https://wildboarsoftware.com). The ASN.1
compiler itself is closed-source and proprietary, but some of the libraries
produced with it are released publicly under the
[MIT license](https://mit-license.org/).

If you would like to see additional ASN.1 libraries in Rust or other
programming languages, or if you have any other questions, please contact us at
[contact@wildboarsoftware.com](mailto:contact@wildboarsoftware.com).

## To Do

- [ ] **FIRST**: Dedupe everything in `PKI-Stub`!
- [x] `Refinement.matches_object_classes()`
- [ ] `Refinement.matches_anything()`
- [ ] `Filter.filter_items()` iterator (does need to return whether negated or not for `nullMatch`)
- [ ] `Filter.to_partly_normalized()`
  - Basically, just clean up `and:{}` and `or:{}` and push `not` downward
- [ ] `Filter.matches_anything()`
- [ ] Same for `Criteria`
- [ ] Same for `UiiFilter`
- [ ] Same for `ContextCombination`
- [ ] Replicate `Filter` iteration to `wildboar-ldap`
- [x] `Attribute.values()` iterator
- [ ] `ToString`
- [x] Search Result Iterator
- [x] List Result Iterator
- [ ] `OPTIONALLY_PROTECTED.is_signed()` and also `"_SEQ` too
- [ ] `Name` trait implementations
- [ ] Format
- [ ] Unit tests
- [ ] Collections
- [ ] `localeContext` matching
- [ ] `ldapAttributeOptionContext` matching
- [ ] `integerOrderingMatch`
- [ ] `UnboundedDirectoryString.case_ignore_match(other)`
- [ ] `UnboundedDirectoryString.case_exact_match(other)`
- [ ] `UnboundedDirectoryString.case_ignore_ordering_match(other)`
- [ ] `UnboundedDirectoryString.case_exact_ordering_match(other)`
- [ ] `UnboundedDirectoryString::join()`
- [ ] `UnboundedDirectoryString.chars()`
- [ ] `Display` for `InvokeId`
- [ ] `Display` for `Code`
- [ ] `acceptableCertPoliciesMatch`
- [ ] `dualStringMatch`
- [ ] `integerFirstComponentMatch`
- [ ] `directoryStringFirstComponentMatch`
- [ ] `objectIdentifierFirstComponentMatch`
- [ ] `policyMatch`
- [x] `PresentationAddress.eq`
- [ ] `ProtocolInformation.eq`
- [ ] `pwdEncAlgMatch`
- [ ] `sOAIdentifierMatch`
- [ ] `UUIDPair.eq`
- [ ] `octetStringOrderingMatch`
- [ ] `numericStringOrderingMatch`
- [ ] inlining
- [ ] `u32` instead of `INTEGER`
- [ ] `PresentationAddress` length checks
- [ ] `UUID` length checks
- [ ] Almost all substring matching rules
- [ ] Doc comments
- [ ] Readme examples
- [ ] `.into_iter()` waste
- [ ] `CommonArguments`, `CommonResults`, etc. traits
- [ ] `accessControlSchemesThat*` from Meerkat's `authz` folder
- [ ] Newtypes for these bit strings:
  - [ ] `DSEType`
  - [ ] `ServiceControlOptions`
  - [ ] `HierarchySelections`
  - [ ] `SearchControlOptions`
  - [ ] `criticalExtensions`
- [ ] `EntryModification`
  - [ ] `get_affected_attribute_types()`
- [ ] `EntryInformation`
  - [ ] `is_child()`
  - [ ] `is_parent()`
  - [ ] `is_subentry()`
  - [ ] `is_admin_point()`
  - [ ] `is_alias()`
  - [ ] `is_root()`
  - [ ] `is_glue()`
  - [ ] `is_cp()`
  - [ ] `is_entry()`
  - [ ] `is_subr()`
  - [ ] `is_nssr()`
  - [ ] `is_supr()`
  - [ ] `is_xr()`
  - [ ] `is_shadow()`
  - [ ] `is_imm_supr()`
  - [ ] `is_rhob()`
  - [ ] `is_dit_bridge()`
  - [ ] `is_ds_subentry()`
  - [ ] `is_family_member()`
  - [ ] `is_subordinate_alias()`
  - [ ] `is_writeable_copy()`
  - [ ] `is_hierarchy_member()`
  - [ ] `get_object_classes()`
  - [ ] `get_object_class_set()`
  - [ ] `get_dse_type()`
  - [ ] `get_aliased_entry_name()`
  - [ ] `get_admin_roles()`
  - [ ] `get_admin_roles_set()`
  - [ ] Object Classes Iterator
  - [ ] Admin Roles Iterator
  - [ ] Family member iterator
- [ ] AttributeCertificate
  - [ ] `is_platform_certificate()` (should be behind a feature flag)
  - [ ] `is_qualified_certificate()`
  - [ ] `is_role_specification_certificate()`
  - [ ] `is_attribute_descriptor_certificate()`
  - [ ] `get_role_name()`
- [ ] Certificate
  - [ ] `is_ca()`
  - [ ] `get_key_usage()`
  - [ ] `get_ext_key_usage()`
  - [ ] I think there's a _lot_ more...
- [ ] `DITStructureNode` (like `DITStructureRule`, but tree structure)
- [ ] `Display`
  - [ ] `OperationalBindingID`
  - [ ] `Validity`
  - [ ] `SearchRuleId`
  - [ ] `Extension`?
- [ ] `NamedDay.to_day_of_week()`
- [ ] Features
  - [ ] `MTSAbstractService`
  - [ ] `PkiPMIProtocolSpecifications.asn1` -> `PkiPmiWrapper` (depends on ACD)
  - [ ] `AlgorithmObjectIdentifiers`
  - [ ] You could turn `AttributeCertificateDefinitions` into a feature flag if you tried
  - [ ] `ExtensionAttributes`
  - [ ] `PkiPmiExternalDataTypes` could probably get de-coupled, then flagged
  - [ ] `SpkmGssTokens` with a little work
  - [ ] `HierarchicalOperationalBindings`
  - [ ] `ProtocolObjectIdentifiers`
  - [ ] `DirectoryManagement`
  - [ ] `UpperBounds`
  - [ ] `PKI-Stub`
  - [ ] A flag for all of PKI PMI Protocols
- [ ] You could do a lot more in this library if comparators took a function
      that would compare elements, but I would like to add this _after_ its
      been proven in the field.
