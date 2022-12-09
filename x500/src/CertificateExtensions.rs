#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # CertificateExtensions
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `CertificateExtensions`.
//!
//! This compilation was produced by the
//! [Wildboar Software](https://wildboarsoftware.com/en)
//! [ASN.1 Compiler](https://wildboarsoftware.com/en/asn1-compilation).
//!
//! Types from the source ASN.1 module are generally available by their original
//! names, but with hyphens replaced by underscores. Encoders and decoders for
//! any given type are available as `_encode_TYPENAME()` and
//! `_decode_TYPENAME()`. Decoders are also available as implementations of
//! the `From<X690Element` and `From<&'a X690Element>` traits for some
//! types.
//!
use crate::AuthenticationFramework::*;
use crate::InformationFramework::*;
use crate::PkiPmiExternalDataTypes::*;
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// authorityKeyIdentifier EXTENSION ::= {
///   SYNTAX         AuthorityKeyIdentifier
///   IDENTIFIED BY  id-ce-authorityKeyIdentifier }
/// ```
///
///
pub fn authorityKeyIdentifier() -> EXTENSION {
    EXTENSION {
        id: id_ce_authorityKeyIdentifier(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthorityKeyIdentifier ::= SEQUENCE {
///   keyIdentifier              [0]  KeyIdentifier OPTIONAL,
///   authorityCertIssuer        [1]  GeneralNames OPTIONAL,
///   authorityCertSerialNumber  [2]  CertificateSerialNumber OPTIONAL,
///   ... }
///   (WITH COMPONENTS {..., authorityCertIssuer        PRESENT,
///                          authorityCertSerialNumber  PRESENT } |
///    WITH COMPONENTS {..., authorityCertIssuer        ABSENT,
///                          authorityCertSerialNumber  ABSENT } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AuthorityKeyIdentifier {
    pub keyIdentifier: OPTIONAL<KeyIdentifier>,
    pub authorityCertIssuer: OPTIONAL<GeneralNames>,
    pub authorityCertSerialNumber: OPTIONAL<CertificateSerialNumber>,
    pub _unrecognized: Vec<X690Element>,
}
impl AuthorityKeyIdentifier {
    pub fn new(
        keyIdentifier: OPTIONAL<KeyIdentifier>,
        authorityCertIssuer: OPTIONAL<GeneralNames>,
        authorityCertSerialNumber: OPTIONAL<CertificateSerialNumber>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AuthorityKeyIdentifier {
            keyIdentifier,
            authorityCertIssuer,
            authorityCertSerialNumber,
            _unrecognized,
        }
    }
}
impl Default for AuthorityKeyIdentifier {
    fn default() -> Self {
        AuthorityKeyIdentifier {
            keyIdentifier: None,
            authorityCertIssuer: None,
            authorityCertSerialNumber: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for AuthorityKeyIdentifier {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AuthorityKeyIdentifier(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AuthorityKeyIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AuthorityKeyIdentifier(el)
    }
}

pub const _rctl1_components_for_AuthorityKeyIdentifier: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "keyIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "authorityCertIssuer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "authorityCertSerialNumber",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AuthorityKeyIdentifier: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AuthorityKeyIdentifier: &[ComponentSpec; 0] = &[];

pub fn _decode_AuthorityKeyIdentifier(el: &X690Element) -> ASN1Result<AuthorityKeyIdentifier> {
    |el_: &X690Element| -> ASN1Result<AuthorityKeyIdentifier> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AuthorityKeyIdentifier,
            _eal_components_for_AuthorityKeyIdentifier,
            _rctl2_components_for_AuthorityKeyIdentifier,
        )?;
        let keyIdentifier: OPTIONAL<KeyIdentifier> = match _components.get("keyIdentifier") {
            Some(c_) => Some(_decode_KeyIdentifier(c_)?),
            _ => None,
        };
        let authorityCertIssuer: OPTIONAL<GeneralNames> =
            match _components.get("authorityCertIssuer") {
                Some(c_) => Some(_decode_GeneralNames(c_)?),
                _ => None,
            };
        let authorityCertSerialNumber: OPTIONAL<CertificateSerialNumber> =
            match _components.get("authorityCertSerialNumber") {
                Some(c_) => Some(_decode_CertificateSerialNumber(c_)?),
                _ => None,
            };
        Ok(AuthorityKeyIdentifier {
            keyIdentifier,
            authorityCertIssuer,
            authorityCertSerialNumber,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AuthorityKeyIdentifier(value_: &AuthorityKeyIdentifier) -> ASN1Result<X690Element> {
    |value_: &AuthorityKeyIdentifier| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.keyIdentifier {
            components_.push(|v_1: &KeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_KeyIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.authorityCertIssuer {
            components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralNames(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.authorityCertSerialNumber {
            components_.push(|v_1: &CertificateSerialNumber| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateSerialNumber(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyIdentifier  ::=  OCTET STRING
/// ```
pub type KeyIdentifier = OCTET_STRING; // OctetStringType

pub fn _decode_KeyIdentifier(el: &X690Element) -> ASN1Result<KeyIdentifier> {
    ber_decode_octet_string(&el)
}

pub fn _encode_KeyIdentifier(value_: &KeyIdentifier) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subjectKeyIdentifier EXTENSION ::= {
///   SYNTAX         SubjectKeyIdentifier
///   IDENTIFIED BY  id-ce-subjectKeyIdentifier }
/// ```
///
///
pub fn subjectKeyIdentifier() -> EXTENSION {
    EXTENSION {
        id: id_ce_subjectKeyIdentifier(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubjectKeyIdentifier  ::=  KeyIdentifier
/// ```
pub type SubjectKeyIdentifier = KeyIdentifier; // DefinedType

pub fn _decode_SubjectKeyIdentifier(el: &X690Element) -> ASN1Result<SubjectKeyIdentifier> {
    _decode_KeyIdentifier(&el)
}

pub fn _encode_SubjectKeyIdentifier(value_: &SubjectKeyIdentifier) -> ASN1Result<X690Element> {
    _encode_KeyIdentifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// keyUsage EXTENSION ::= {
///   SYNTAX         KeyUsage
///   IDENTIFIED BY  id-ce-keyUsage }
/// ```
///
///
pub fn keyUsage() -> EXTENSION {
    EXTENSION {
        id: id_ce_keyUsage(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyUsage  ::=  BIT STRING {
///   digitalSignature  (0),
///   contentCommitment (1),
///   keyEncipherment   (2),
///   dataEncipherment  (3),
///   keyAgreement      (4),
///   keyCertSign       (5),
///   cRLSign           (6),
///   encipherOnly      (7),
///   decipherOnly      (8) }
/// ```
pub type KeyUsage = BIT_STRING;

pub const KeyUsage_digitalSignature: BIT = 0; /* LONG_NAMED_BIT */

pub const KeyUsage_contentCommitment: BIT = 1; /* LONG_NAMED_BIT */

pub const KeyUsage_keyEncipherment: BIT = 2; /* LONG_NAMED_BIT */

pub const KeyUsage_dataEncipherment: BIT = 3; /* LONG_NAMED_BIT */

pub const KeyUsage_keyAgreement: BIT = 4; /* LONG_NAMED_BIT */

pub const KeyUsage_keyCertSign: BIT = 5; /* LONG_NAMED_BIT */

pub const KeyUsage_cRLSign: BIT = 6; /* LONG_NAMED_BIT */

pub const KeyUsage_encipherOnly: BIT = 7; /* LONG_NAMED_BIT */

pub const KeyUsage_decipherOnly: BIT = 8; /* LONG_NAMED_BIT */

pub fn _decode_KeyUsage(el: &X690Element) -> ASN1Result<KeyUsage> {
    ber_decode_bit_string(&el)
}

pub fn _encode_KeyUsage(value_: &KeyUsage) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extKeyUsage EXTENSION ::= {
///   SYNTAX         SEQUENCE SIZE (1..MAX) OF KeyPurposeId
///   IDENTIFIED BY  id-ce-extKeyUsage }
/// ```
///
///
pub fn extKeyUsage() -> EXTENSION {
    EXTENSION {
        id: id_ce_extKeyUsage(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyPurposeId  ::=  OBJECT IDENTIFIER
/// ```
pub type KeyPurposeId = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_KeyPurposeId(el: &X690Element) -> ASN1Result<KeyPurposeId> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_KeyPurposeId(value_: &KeyPurposeId) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// privateKeyUsagePeriod EXTENSION ::= {
///   SYNTAX         PrivateKeyUsagePeriod
///   IDENTIFIED BY  id-ce-privateKeyUsagePeriod }
/// ```
///
///
pub fn privateKeyUsagePeriod() -> EXTENSION {
    EXTENSION {
        id: id_ce_privateKeyUsagePeriod(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PrivateKeyUsagePeriod ::= SEQUENCE {
///   notBefore  [0]  GeneralizedTime OPTIONAL,
///   notAfter   [1]  GeneralizedTime OPTIONAL,
///   ... }
///   (WITH COMPONENTS {..., notBefore  PRESENT } |
///    WITH COMPONENTS {..., notAfter   PRESENT } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PrivateKeyUsagePeriod {
    pub notBefore: OPTIONAL<GeneralizedTime>,
    pub notAfter: OPTIONAL<GeneralizedTime>,
    pub _unrecognized: Vec<X690Element>,
}
impl PrivateKeyUsagePeriod {
    pub fn new(
        notBefore: OPTIONAL<GeneralizedTime>,
        notAfter: OPTIONAL<GeneralizedTime>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PrivateKeyUsagePeriod {
            notBefore,
            notAfter,
            _unrecognized,
        }
    }
}
impl Default for PrivateKeyUsagePeriod {
    fn default() -> Self {
        PrivateKeyUsagePeriod {
            notBefore: None,
            notAfter: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for PrivateKeyUsagePeriod {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PrivateKeyUsagePeriod(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PrivateKeyUsagePeriod {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PrivateKeyUsagePeriod(el)
    }
}

pub const _rctl1_components_for_PrivateKeyUsagePeriod: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "notBefore",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "notAfter",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PrivateKeyUsagePeriod: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PrivateKeyUsagePeriod: &[ComponentSpec; 0] = &[];

pub fn _decode_PrivateKeyUsagePeriod(el: &X690Element) -> ASN1Result<PrivateKeyUsagePeriod> {
    |el_: &X690Element| -> ASN1Result<PrivateKeyUsagePeriod> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PrivateKeyUsagePeriod,
            _eal_components_for_PrivateKeyUsagePeriod,
            _rctl2_components_for_PrivateKeyUsagePeriod,
        )?;
        let notBefore: OPTIONAL<GeneralizedTime> = match _components.get("notBefore") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        let notAfter: OPTIONAL<GeneralizedTime> = match _components.get("notAfter") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        Ok(PrivateKeyUsagePeriod {
            notBefore,
            notAfter,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PrivateKeyUsagePeriod(value_: &PrivateKeyUsagePeriod) -> ASN1Result<X690Element> {
    |value_: &PrivateKeyUsagePeriod| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.notBefore {
            components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_generalized_time(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.notAfter {
            components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_generalized_time(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificatePolicies EXTENSION ::= {
///   SYNTAX         CertificatePoliciesSyntax
///   IDENTIFIED BY  id-ce-certificatePolicies }
/// ```
///
///
pub fn certificatePolicies() -> EXTENSION {
    EXTENSION {
        id: id_ce_certificatePolicies(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificatePoliciesSyntax  ::=  SEQUENCE SIZE (1..MAX) OF PolicyInformation
/// ```
pub type CertificatePoliciesSyntax = Vec<PolicyInformation>; // SequenceOfType

pub fn _decode_CertificatePoliciesSyntax(
    el: &X690Element,
) -> ASN1Result<CertificatePoliciesSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<PolicyInformation>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<PolicyInformation> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_PolicyInformation(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CertificatePoliciesSyntax(
    value_: &CertificatePoliciesSyntax,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<PolicyInformation>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_PolicyInformation(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PolicyInformation ::= SEQUENCE {
///   policyIdentifier  CertPolicyId,
///   policyQualifiers  SEQUENCE SIZE (1..MAX) OF PolicyQualifierInfo OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PolicyInformation {
    pub policyIdentifier: CertPolicyId,
    pub policyQualifiers: OPTIONAL<Vec<PolicyQualifierInfo>>,
    pub _unrecognized: Vec<X690Element>,
}
impl PolicyInformation {
    pub fn new(
        policyIdentifier: CertPolicyId,
        policyQualifiers: OPTIONAL<Vec<PolicyQualifierInfo>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PolicyInformation {
            policyIdentifier,
            policyQualifiers,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for PolicyInformation {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PolicyInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PolicyInformation {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PolicyInformation(el)
    }
}

pub const _rctl1_components_for_PolicyInformation: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "policyIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "policyQualifiers",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PolicyInformation: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PolicyInformation: &[ComponentSpec; 0] = &[];

pub fn _decode_PolicyInformation(el: &X690Element) -> ASN1Result<PolicyInformation> {
    |el_: &X690Element| -> ASN1Result<PolicyInformation> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PolicyInformation,
            _eal_components_for_PolicyInformation,
            _rctl2_components_for_PolicyInformation,
        )?;
        let policyIdentifier = _decode_CertPolicyId(_components.get("policyIdentifier").unwrap())?;
        let policyQualifiers: OPTIONAL<Vec<PolicyQualifierInfo>> =
            match _components.get("policyQualifiers") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<PolicyQualifierInfo>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<PolicyQualifierInfo> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_PolicyQualifierInfo(el)?);
                        }
                        Ok(items)
                    }(c_)?,
                ),
                _ => None,
            };
        Ok(PolicyInformation {
            policyIdentifier,
            policyQualifiers,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PolicyInformation(value_: &PolicyInformation) -> ASN1Result<X690Element> {
    |value_: &PolicyInformation| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_CertPolicyId(&value_.policyIdentifier)?);
        if let Some(v_) = &value_.policyQualifiers {
            components_.push(
                |value_: &SEQUENCE_OF<PolicyQualifierInfo>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_PolicyQualifierInfo(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertPolicyId  ::=  OBJECT IDENTIFIER
/// ```
pub type CertPolicyId = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_CertPolicyId(el: &X690Element) -> ASN1Result<CertPolicyId> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_CertPolicyId(value_: &CertPolicyId) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PolicyQualifierInfo ::= SEQUENCE {
///   policyQualifierId  CERT-POLICY-QUALIFIER.&id({SupportedPolicyQualifiers}),
///   qualifier          CERT-POLICY-QUALIFIER.&Qualifier
///               ({SupportedPolicyQualifiers}{@policyQualifierId}) OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PolicyQualifierInfo {
    pub policyQualifierId: OBJECT_IDENTIFIER,
    pub qualifier: OPTIONAL<X690Element>,
    pub _unrecognized: Vec<X690Element>,
}
impl PolicyQualifierInfo {
    pub fn new(
        policyQualifierId: OBJECT_IDENTIFIER,
        qualifier: OPTIONAL<X690Element>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PolicyQualifierInfo {
            policyQualifierId,
            qualifier,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for PolicyQualifierInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PolicyQualifierInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PolicyQualifierInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PolicyQualifierInfo(el)
    }
}

pub const _rctl1_components_for_PolicyQualifierInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "policyQualifierId",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("qualifier", true, TagSelector::any, None, None),
];

pub const _rctl2_components_for_PolicyQualifierInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PolicyQualifierInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_PolicyQualifierInfo(el: &X690Element) -> ASN1Result<PolicyQualifierInfo> {
    |el_: &X690Element| -> ASN1Result<PolicyQualifierInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PolicyQualifierInfo,
            _eal_components_for_PolicyQualifierInfo,
            _rctl2_components_for_PolicyQualifierInfo,
        )?;
        let policyQualifierId =
            ber_decode_object_identifier(_components.get("policyQualifierId").unwrap())?;
        let qualifier: OPTIONAL<X690Element> = match _components.get("qualifier") {
            Some(c_) => Some(x690_identity(c_)?),
            _ => None,
        };
        Ok(PolicyQualifierInfo {
            policyQualifierId,
            qualifier,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PolicyQualifierInfo(value_: &PolicyQualifierInfo) -> ASN1Result<X690Element> {
    |value_: &PolicyQualifierInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.policyQualifierId)?);
        if let Some(v_) = &value_.qualifier {
            components_.push(x690_identity(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedPolicyQualifiers CERT-POLICY-QUALIFIER ::= {...}
/// ```
///
///
pub fn SupportedPolicyQualifiers() -> Vec<CERT_POLICY_QUALIFIER> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// anyPolicy OBJECT IDENTIFIER ::= {id-ce-certificatePolicies 0}
/// ```
///
///
pub fn anyPolicy() -> OBJECT_IDENTIFIER {
    [id_ce_certificatePolicies(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CERT-POLICY-QUALIFIER ::= CLASS {
///   &id                  OBJECT IDENTIFIER UNIQUE,
///   &Qualifier           OPTIONAL }
/// WITH SYNTAX {
///   POLICY-QUALIFIER-ID &id
///   [QUALIFIER-TYPE     &Qualifier] }
/// ```
///
#[derive(Debug)]
pub struct CERT_POLICY_QUALIFIER {
    pub id: OBJECT_IDENTIFIER,
}
impl CERT_POLICY_QUALIFIER {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// policyMappings EXTENSION ::= {
///   SYNTAX         PolicyMappingsSyntax
///   IDENTIFIED BY  id-ce-policyMappings }
/// ```
///
///
pub fn policyMappings() -> EXTENSION {
    EXTENSION {
        id: id_ce_policyMappings(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PolicyMappingsSyntax  ::=  SEQUENCE SIZE (1..MAX) OF SEQUENCE {
///   issuerDomainPolicy   CertPolicyId,
///   subjectDomainPolicy  CertPolicyId,
///   ... }
/// ```
pub type PolicyMappingsSyntax = Vec<PolicyMappingsSyntax_Item>; // SequenceOfType

pub fn _decode_PolicyMappingsSyntax(el: &X690Element) -> ASN1Result<PolicyMappingsSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<PolicyMappingsSyntax_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<PolicyMappingsSyntax_Item> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_PolicyMappingsSyntax_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_PolicyMappingsSyntax(value_: &PolicyMappingsSyntax) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<PolicyMappingsSyntax_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_PolicyMappingsSyntax_Item(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// authorizationValidation EXTENSION ::= {
///   SYNTAX         AvlId
///   IDENTIFIED BY  id-ce-authorizationValidation }
/// ```
///
///
pub fn authorizationValidation() -> EXTENSION {
    EXTENSION {
        id: id_ce_authorizationValidation(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AvlId ::= SEQUENCE {
///   issuer        Name,
///   serialNumber  AvlSerialNumber OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AvlId {
    pub issuer: Name,
    pub serialNumber: OPTIONAL<AvlSerialNumber>,
    pub _unrecognized: Vec<X690Element>,
}
impl AvlId {
    pub fn new(
        issuer: Name,
        serialNumber: OPTIONAL<AvlSerialNumber>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AvlId {
            issuer,
            serialNumber,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AvlId {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AvlId(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AvlId {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AvlId(el)
    }
}

pub const _rctl1_components_for_AvlId: &[ComponentSpec; 2] = &[
    ComponentSpec::new("issuer", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "serialNumber",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AvlId: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AvlId: &[ComponentSpec; 0] = &[];

pub fn _decode_AvlId(el: &X690Element) -> ASN1Result<AvlId> {
    |el_: &X690Element| -> ASN1Result<AvlId> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AvlId,
            _eal_components_for_AvlId,
            _rctl2_components_for_AvlId,
        )?;
        let issuer = _decode_Name(_components.get("issuer").unwrap())?;
        let serialNumber: OPTIONAL<AvlSerialNumber> = match _components.get("serialNumber") {
            Some(c_) => Some(_decode_AvlSerialNumber(c_)?),
            _ => None,
        };
        Ok(AvlId {
            issuer,
            serialNumber,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AvlId(value_: &AvlId) -> ASN1Result<X690Element> {
    |value_: &AvlId| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Name(&value_.issuer)?);
        if let Some(v_) = &value_.serialNumber {
            components_.push(_encode_AvlSerialNumber(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subjectAltName EXTENSION ::= {
///   SYNTAX         GeneralNames
///   IDENTIFIED BY  id-ce-subjectAltName }
/// ```
///
///
pub fn subjectAltName() -> EXTENSION {
    EXTENSION {
        id: id_ce_subjectAltName(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// GeneralNames  ::=  SEQUENCE SIZE (1..MAX) OF GeneralName
/// ```
pub type GeneralNames = Vec<GeneralName>; // SequenceOfType

pub fn _decode_GeneralNames(el: &X690Element) -> ASN1Result<GeneralNames> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<GeneralName>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<GeneralName> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_GeneralName(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_GeneralNames(value_: &GeneralNames) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<GeneralName>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_GeneralName(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// GeneralName  ::=  CHOICE {
///   otherName                  [0]  INSTANCE OF OTHER-NAME,
///   rfc822Name                 [1]  IA5String,
///   dNSName                    [2]  IA5String,
///   x400Address                [3]  ORAddress,
///   directoryName              [4]  Name,
///   ediPartyName               [5]  EDIPartyName,
///   uniformResourceIdentifier  [6]  IA5String,
///   iPAddress                  [7]  OCTET STRING,
///   registeredID               [8]  OBJECT IDENTIFIER,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum GeneralName {
    otherName(INSTANCE_OF),
    rfc822Name(IA5String),
    dNSName(IA5String),
    x400Address(ORAddress),
    directoryName(Name),
    ediPartyName(EDIPartyName),
    uniformResourceIdentifier(IA5String),
    iPAddress(OCTET_STRING),
    registeredID(OBJECT_IDENTIFIER),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for GeneralName {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_GeneralName(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for GeneralName {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_GeneralName(el)
    }
}

pub fn _decode_GeneralName(el: &X690Element) -> ASN1Result<GeneralName> {
    |el: &X690Element| -> ASN1Result<GeneralName> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(GeneralName::otherName(ber_decode_instance_of(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(GeneralName::rfc822Name(ber_decode_ia5_string(&el)?)),
            (TagClass::CONTEXT, 2) => Ok(GeneralName::dNSName(ber_decode_ia5_string(&el)?)),
            (TagClass::CONTEXT, 3) => Ok(GeneralName::x400Address(_decode_ORAddress(&el)?)),
            (TagClass::CONTEXT, 4) => Ok(GeneralName::directoryName(
                |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(&el)?,
            )),
            (TagClass::CONTEXT, 5) => Ok(GeneralName::ediPartyName(_decode_EDIPartyName(&el)?)),
            (TagClass::CONTEXT, 6) => Ok(GeneralName::uniformResourceIdentifier(
                ber_decode_ia5_string(&el)?,
            )),
            (TagClass::CONTEXT, 7) => Ok(GeneralName::iPAddress(ber_decode_octet_string(&el)?)),
            (TagClass::CONTEXT, 8) => Ok(GeneralName::registeredID(ber_decode_object_identifier(
                &el,
            )?)),
            _ => Ok(GeneralName::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_GeneralName(value_: &GeneralName) -> ASN1Result<X690Element> {
    |value: &GeneralName| -> ASN1Result<X690Element> {
        match value {
            GeneralName::otherName(v) => |v_1: &INSTANCE_OF| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_instance_of(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            GeneralName::rfc822Name(v) => |v_1: &IA5String| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_ia5_string(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            GeneralName::dNSName(v) => |v_1: &IA5String| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_ia5_string(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v),
            GeneralName::x400Address(v) => |v_1: &ORAddress| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ORAddress(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v),
            GeneralName::directoryName(v) => |v_1: &Name| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
                ))
            }(&v),
            GeneralName::ediPartyName(v) => |v_1: &EDIPartyName| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_EDIPartyName(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 5;
                Ok(el_1)
            }(&v),
            GeneralName::uniformResourceIdentifier(v) => {
                |v_1: &IA5String| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_ia5_string(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 6;
                    Ok(el_1)
                }(&v)
            }
            GeneralName::iPAddress(v) => |v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_octet_string(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 7;
                Ok(el_1)
            }(&v),
            GeneralName::registeredID(v) => |v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_object_identifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 8;
                Ok(el_1)
            }(&v),
            GeneralName::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

pub type OTHER_NAME = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// EDIPartyName ::= SEQUENCE {
///   nameAssigner  [0]  UnboundedDirectoryString OPTIONAL,
///   partyName     [1]  UnboundedDirectoryString,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EDIPartyName {
    pub nameAssigner: OPTIONAL<UnboundedDirectoryString>,
    pub partyName: UnboundedDirectoryString,
    pub _unrecognized: Vec<X690Element>,
}
impl EDIPartyName {
    pub fn new(
        nameAssigner: OPTIONAL<UnboundedDirectoryString>,
        partyName: UnboundedDirectoryString,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        EDIPartyName {
            nameAssigner,
            partyName,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for EDIPartyName {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EDIPartyName(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EDIPartyName {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EDIPartyName(el)
    }
}

pub const _rctl1_components_for_EDIPartyName: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "nameAssigner",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "partyName",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EDIPartyName: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EDIPartyName: &[ComponentSpec; 0] = &[];

pub fn _decode_EDIPartyName(el: &X690Element) -> ASN1Result<EDIPartyName> {
    |el_: &X690Element| -> ASN1Result<EDIPartyName> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EDIPartyName,
            _eal_components_for_EDIPartyName,
            _rctl2_components_for_EDIPartyName,
        )?;
        let nameAssigner: OPTIONAL<UnboundedDirectoryString> = match _components.get("nameAssigner")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
                Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let partyName = |el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
            Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
        }(_components.get("partyName").unwrap())?;
        Ok(EDIPartyName {
            nameAssigner,
            partyName,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_EDIPartyName(value_: &EDIPartyName) -> ASN1Result<X690Element> {
    |value_: &EDIPartyName| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.nameAssigner {
            components_.push(
                |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_UnboundedDirectoryString(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        components_.push(
            |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_UnboundedDirectoryString(&v_1)?,
                    ))),
                ))
            }(&value_.partyName)?,
        );
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// issuerAltName EXTENSION ::= {
///   SYNTAX         GeneralNames
///   IDENTIFIED BY  id-ce-issuerAltName }
/// ```
///
///
pub fn issuerAltName() -> EXTENSION {
    EXTENSION {
        id: id_ce_issuerAltName(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subjectDirectoryAttributes EXTENSION ::= {
///   SYNTAX         AttributesSyntax
///   IDENTIFIED BY  id-ce-subjectDirectoryAttributes }
/// ```
///
///
pub fn subjectDirectoryAttributes() -> EXTENSION {
    EXTENSION {
        id: id_ce_subjectDirectoryAttributes(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributesSyntax  ::=  SEQUENCE SIZE (1..MAX) OF Attribute{{SupportedAttributes}}
/// ```
pub type AttributesSyntax = Vec<Attribute>; // SequenceOfType

pub fn _decode_AttributesSyntax(el: &X690Element) -> ASN1Result<AttributesSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<Attribute>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_Attribute(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AttributesSyntax(value_: &AttributesSyntax) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<Attribute>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Attribute(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// associatedInformation EXTENSION ::= {
///   SYNTAX         AttributesSyntax
///   IDENTIFIED BY  id-ce-associatedInformation }
/// ```
///
///
pub fn associatedInformation() -> EXTENSION {
    EXTENSION {
        id: id_ce_associatedInformation(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// basicConstraints EXTENSION ::= {
///   SYNTAX         BasicConstraintsSyntax
///   IDENTIFIED BY  id-ce-basicConstraints }
/// ```
///
///
pub fn basicConstraints() -> EXTENSION {
    EXTENSION {
        id: id_ce_basicConstraints(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BasicConstraintsSyntax ::= SEQUENCE {
///   cA                 BOOLEAN DEFAULT FALSE,
///   pathLenConstraint  INTEGER(0..MAX) OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct BasicConstraintsSyntax {
    pub cA: OPTIONAL<BOOLEAN>,
    pub pathLenConstraint: OPTIONAL<INTEGER>,
    pub _unrecognized: Vec<X690Element>,
}
impl BasicConstraintsSyntax {
    pub fn new(
        cA: OPTIONAL<BOOLEAN>,
        pathLenConstraint: OPTIONAL<INTEGER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        BasicConstraintsSyntax {
            cA,
            pathLenConstraint,
            _unrecognized,
        }
    }
    pub fn _default_value_for_cA() -> BOOLEAN {
        false
    }
}
impl Default for BasicConstraintsSyntax {
    fn default() -> Self {
        BasicConstraintsSyntax {
            cA: None,
            pathLenConstraint: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for BasicConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_BasicConstraintsSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for BasicConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_BasicConstraintsSyntax(el)
    }
}

pub const _rctl1_components_for_BasicConstraintsSyntax: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "cA",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pathLenConstraint",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_BasicConstraintsSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_BasicConstraintsSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_BasicConstraintsSyntax(el: &X690Element) -> ASN1Result<BasicConstraintsSyntax> {
    |el_: &X690Element| -> ASN1Result<BasicConstraintsSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_BasicConstraintsSyntax,
            _eal_components_for_BasicConstraintsSyntax,
            _rctl2_components_for_BasicConstraintsSyntax,
        )?;
        let cA: OPTIONAL<BOOLEAN> = match _components.get("cA") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let pathLenConstraint: OPTIONAL<INTEGER> = match _components.get("pathLenConstraint") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        Ok(BasicConstraintsSyntax {
            cA,
            pathLenConstraint,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_BasicConstraintsSyntax(value_: &BasicConstraintsSyntax) -> ASN1Result<X690Element> {
    |value_: &BasicConstraintsSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.cA {
            if *v_ != BasicConstraintsSyntax::_default_value_for_cA() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        if let Some(v_) = &value_.pathLenConstraint {
            components_.push(ber_encode_integer(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nameConstraints EXTENSION ::= {
///   SYNTAX         NameConstraintsSyntax
///   IDENTIFIED BY  id-ce-nameConstraints }
/// ```
///
///
pub fn nameConstraints() -> EXTENSION {
    EXTENSION {
        id: id_ce_nameConstraints(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NameConstraintsSyntax ::= SEQUENCE {
///   permittedSubtrees  [0]  GeneralSubtrees OPTIONAL,
///   excludedSubtrees   [1]  GeneralSubtrees OPTIONAL,
///   ... }
///   (WITH COMPONENTS {..., permittedSubtrees  PRESENT } |
///    WITH COMPONENTS {..., excludedSubtrees   PRESENT } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct NameConstraintsSyntax {
    pub permittedSubtrees: OPTIONAL<GeneralSubtrees>,
    pub excludedSubtrees: OPTIONAL<GeneralSubtrees>,
    pub _unrecognized: Vec<X690Element>,
}
impl NameConstraintsSyntax {
    pub fn new(
        permittedSubtrees: OPTIONAL<GeneralSubtrees>,
        excludedSubtrees: OPTIONAL<GeneralSubtrees>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        NameConstraintsSyntax {
            permittedSubtrees,
            excludedSubtrees,
            _unrecognized,
        }
    }
}
impl Default for NameConstraintsSyntax {
    fn default() -> Self {
        NameConstraintsSyntax {
            permittedSubtrees: None,
            excludedSubtrees: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for NameConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_NameConstraintsSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for NameConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_NameConstraintsSyntax(el)
    }
}

pub const _rctl1_components_for_NameConstraintsSyntax: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "permittedSubtrees",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "excludedSubtrees",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_NameConstraintsSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_NameConstraintsSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_NameConstraintsSyntax(el: &X690Element) -> ASN1Result<NameConstraintsSyntax> {
    |el_: &X690Element| -> ASN1Result<NameConstraintsSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_NameConstraintsSyntax,
            _eal_components_for_NameConstraintsSyntax,
            _rctl2_components_for_NameConstraintsSyntax,
        )?;
        let permittedSubtrees: OPTIONAL<GeneralSubtrees> =
            match _components.get("permittedSubtrees") {
                Some(c_) => Some(_decode_GeneralSubtrees(c_)?),
                _ => None,
            };
        let excludedSubtrees: OPTIONAL<GeneralSubtrees> = match _components.get("excludedSubtrees")
        {
            Some(c_) => Some(_decode_GeneralSubtrees(c_)?),
            _ => None,
        };
        Ok(NameConstraintsSyntax {
            permittedSubtrees,
            excludedSubtrees,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_NameConstraintsSyntax(value_: &NameConstraintsSyntax) -> ASN1Result<X690Element> {
    |value_: &NameConstraintsSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.permittedSubtrees {
            components_.push(|v_1: &GeneralSubtrees| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralSubtrees(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.excludedSubtrees {
            components_.push(|v_1: &GeneralSubtrees| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralSubtrees(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// GeneralSubtrees  ::=  SEQUENCE SIZE (1..MAX) OF GeneralSubtree
/// ```
pub type GeneralSubtrees = Vec<GeneralSubtree>; // SequenceOfType

pub fn _decode_GeneralSubtrees(el: &X690Element) -> ASN1Result<GeneralSubtrees> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<GeneralSubtree>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<GeneralSubtree> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_GeneralSubtree(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_GeneralSubtrees(value_: &GeneralSubtrees) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<GeneralSubtree>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_GeneralSubtree(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// GeneralSubtree ::= SEQUENCE {
///   base          GeneralName,
///   minimum  [0]  BaseDistance DEFAULT 0,
///   maximum  [1]  BaseDistance OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct GeneralSubtree {
    pub base: GeneralName,
    pub minimum: OPTIONAL<BaseDistance>,
    pub maximum: OPTIONAL<BaseDistance>,
    pub _unrecognized: Vec<X690Element>,
}
impl GeneralSubtree {
    pub fn new(
        base: GeneralName,
        minimum: OPTIONAL<BaseDistance>,
        maximum: OPTIONAL<BaseDistance>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        GeneralSubtree {
            base,
            minimum,
            maximum,
            _unrecognized,
        }
    }
    pub fn _default_value_for_minimum() -> BaseDistance {
        0
    }
}
impl TryFrom<X690Element> for GeneralSubtree {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_GeneralSubtree(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for GeneralSubtree {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_GeneralSubtree(el)
    }
}

pub const _rctl1_components_for_GeneralSubtree: &[ComponentSpec; 3] = &[
    ComponentSpec::new("base", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "minimum",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "maximum",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_GeneralSubtree: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_GeneralSubtree: &[ComponentSpec; 0] = &[];

pub fn _decode_GeneralSubtree(el: &X690Element) -> ASN1Result<GeneralSubtree> {
    |el_: &X690Element| -> ASN1Result<GeneralSubtree> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_GeneralSubtree,
            _eal_components_for_GeneralSubtree,
            _rctl2_components_for_GeneralSubtree,
        )?;
        let base = _decode_GeneralName(_components.get("base").unwrap())?;
        let minimum: OPTIONAL<BaseDistance> = match _components.get("minimum") {
            Some(c_) => Some(_decode_BaseDistance(c_)?),
            _ => None,
        };
        let maximum: OPTIONAL<BaseDistance> = match _components.get("maximum") {
            Some(c_) => Some(_decode_BaseDistance(c_)?),
            _ => None,
        };
        Ok(GeneralSubtree {
            base,
            minimum,
            maximum,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_GeneralSubtree(value_: &GeneralSubtree) -> ASN1Result<X690Element> {
    |value_: &GeneralSubtree| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_GeneralName(&value_.base)?);
        if let Some(v_) = &value_.minimum {
            if *v_ != GeneralSubtree::_default_value_for_minimum() {
                components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_BaseDistance(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.maximum {
            components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_BaseDistance(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BaseDistance  ::=  INTEGER(0..MAX)
/// ```
pub type BaseDistance = INTEGER;

pub fn _decode_BaseDistance(el: &X690Element) -> ASN1Result<BaseDistance> {
    ber_decode_integer(&el)
}

pub fn _encode_BaseDistance(value_: &BaseDistance) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// policyConstraints EXTENSION ::= {
///   SYNTAX         PolicyConstraintsSyntax
///   IDENTIFIED BY  id-ce-policyConstraints }
/// ```
///
///
pub fn policyConstraints() -> EXTENSION {
    EXTENSION {
        id: id_ce_policyConstraints(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PolicyConstraintsSyntax ::= SEQUENCE {
///   requireExplicitPolicy  [0]  SkipCerts OPTIONAL,
///   inhibitPolicyMapping   [1]  SkipCerts OPTIONAL,
///   ... }
///   (WITH COMPONENTS {..., requireExplicitPolicy PRESENT } |
///    WITH COMPONENTS {..., inhibitPolicyMapping  PRESENT } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PolicyConstraintsSyntax {
    pub requireExplicitPolicy: OPTIONAL<SkipCerts>,
    pub inhibitPolicyMapping: OPTIONAL<SkipCerts>,
    pub _unrecognized: Vec<X690Element>,
}
impl PolicyConstraintsSyntax {
    pub fn new(
        requireExplicitPolicy: OPTIONAL<SkipCerts>,
        inhibitPolicyMapping: OPTIONAL<SkipCerts>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PolicyConstraintsSyntax {
            requireExplicitPolicy,
            inhibitPolicyMapping,
            _unrecognized,
        }
    }
}
impl Default for PolicyConstraintsSyntax {
    fn default() -> Self {
        PolicyConstraintsSyntax {
            requireExplicitPolicy: None,
            inhibitPolicyMapping: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for PolicyConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PolicyConstraintsSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PolicyConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PolicyConstraintsSyntax(el)
    }
}

pub const _rctl1_components_for_PolicyConstraintsSyntax: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "requireExplicitPolicy",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "inhibitPolicyMapping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PolicyConstraintsSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PolicyConstraintsSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_PolicyConstraintsSyntax(el: &X690Element) -> ASN1Result<PolicyConstraintsSyntax> {
    |el_: &X690Element| -> ASN1Result<PolicyConstraintsSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PolicyConstraintsSyntax,
            _eal_components_for_PolicyConstraintsSyntax,
            _rctl2_components_for_PolicyConstraintsSyntax,
        )?;
        let requireExplicitPolicy: OPTIONAL<SkipCerts> =
            match _components.get("requireExplicitPolicy") {
                Some(c_) => Some(_decode_SkipCerts(c_)?),
                _ => None,
            };
        let inhibitPolicyMapping: OPTIONAL<SkipCerts> =
            match _components.get("inhibitPolicyMapping") {
                Some(c_) => Some(_decode_SkipCerts(c_)?),
                _ => None,
            };
        Ok(PolicyConstraintsSyntax {
            requireExplicitPolicy,
            inhibitPolicyMapping,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PolicyConstraintsSyntax(
    value_: &PolicyConstraintsSyntax,
) -> ASN1Result<X690Element> {
    |value_: &PolicyConstraintsSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.requireExplicitPolicy {
            components_.push(|v_1: &SkipCerts| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SkipCerts(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.inhibitPolicyMapping {
            components_.push(|v_1: &SkipCerts| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SkipCerts(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SkipCerts  ::=  INTEGER(0..MAX)
/// ```
pub type SkipCerts = INTEGER;

pub fn _decode_SkipCerts(el: &X690Element) -> ASN1Result<SkipCerts> {
    ber_decode_integer(&el)
}

pub fn _encode_SkipCerts(value_: &SkipCerts) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// inhibitAnyPolicy EXTENSION ::= {
///   SYNTAX         SkipCerts
///   IDENTIFIED BY  id-ce-inhibitAnyPolicy }
/// ```
///
///
pub fn inhibitAnyPolicy() -> EXTENSION {
    EXTENSION {
        id: id_ce_inhibitAnyPolicy(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cRLNumber EXTENSION ::= {
///   SYNTAX         CRLNumber
///   IDENTIFIED BY  id-ce-cRLNumber }
/// ```
///
///
pub fn cRLNumber() -> EXTENSION {
    EXTENSION {
        id: id_ce_cRLNumber(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CRLNumber  ::=  INTEGER(0..MAX)
/// ```
pub type CRLNumber = INTEGER;

pub fn _decode_CRLNumber(el: &X690Element) -> ASN1Result<CRLNumber> {
    ber_decode_integer(&el)
}

pub fn _encode_CRLNumber(value_: &CRLNumber) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// crlScope EXTENSION ::= {
///   SYNTAX         CRLScopeSyntax
///   IDENTIFIED BY  id-ce-cRLScope }
/// ```
///
///
pub fn crlScope() -> EXTENSION {
    EXTENSION {
        id: id_ce_cRLScope(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CRLScopeSyntax  ::=  SEQUENCE SIZE (1..MAX) OF PerAuthorityScope
/// ```
pub type CRLScopeSyntax = Vec<PerAuthorityScope>; // SequenceOfType

pub fn _decode_CRLScopeSyntax(el: &X690Element) -> ASN1Result<CRLScopeSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<PerAuthorityScope>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<PerAuthorityScope> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_PerAuthorityScope(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CRLScopeSyntax(value_: &CRLScopeSyntax) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<PerAuthorityScope>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_PerAuthorityScope(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PerAuthorityScope ::= SEQUENCE {
///   authorityName       [0]  GeneralName OPTIONAL,
///   distributionPoint   [1]  DistributionPointName OPTIONAL,
///   onlyContains        [2]  OnlyCertificateTypes OPTIONAL,
///   onlySomeReasons     [4]  ReasonFlags OPTIONAL,
///   serialNumberRange   [5]  NumberRange OPTIONAL,
///   subjectKeyIdRange   [6]  NumberRange OPTIONAL,
///   nameSubtrees        [7]  GeneralNames OPTIONAL,
///   baseRevocationInfo  [9]  BaseRevocationInfo OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PerAuthorityScope {
    pub authorityName: OPTIONAL<GeneralName>,
    pub distributionPoint: OPTIONAL<DistributionPointName>,
    pub onlyContains: OPTIONAL<OnlyCertificateTypes>,
    pub onlySomeReasons: OPTIONAL<ReasonFlags>,
    pub serialNumberRange: OPTIONAL<NumberRange>,
    pub subjectKeyIdRange: OPTIONAL<NumberRange>,
    pub nameSubtrees: OPTIONAL<GeneralNames>,
    pub baseRevocationInfo: OPTIONAL<BaseRevocationInfo>,
    pub _unrecognized: Vec<X690Element>,
}
impl PerAuthorityScope {
    pub fn new(
        authorityName: OPTIONAL<GeneralName>,
        distributionPoint: OPTIONAL<DistributionPointName>,
        onlyContains: OPTIONAL<OnlyCertificateTypes>,
        onlySomeReasons: OPTIONAL<ReasonFlags>,
        serialNumberRange: OPTIONAL<NumberRange>,
        subjectKeyIdRange: OPTIONAL<NumberRange>,
        nameSubtrees: OPTIONAL<GeneralNames>,
        baseRevocationInfo: OPTIONAL<BaseRevocationInfo>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PerAuthorityScope {
            authorityName,
            distributionPoint,
            onlyContains,
            onlySomeReasons,
            serialNumberRange,
            subjectKeyIdRange,
            nameSubtrees,
            baseRevocationInfo,
            _unrecognized,
        }
    }
}
impl Default for PerAuthorityScope {
    fn default() -> Self {
        PerAuthorityScope {
            authorityName: None,
            distributionPoint: None,
            onlyContains: None,
            onlySomeReasons: None,
            serialNumberRange: None,
            subjectKeyIdRange: None,
            nameSubtrees: None,
            baseRevocationInfo: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for PerAuthorityScope {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PerAuthorityScope(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PerAuthorityScope {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PerAuthorityScope(el)
    }
}

pub const _rctl1_components_for_PerAuthorityScope: &[ComponentSpec; 8] = &[
    ComponentSpec::new(
        "authorityName",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "distributionPoint",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "onlyContains",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "onlySomeReasons",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "serialNumberRange",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subjectKeyIdRange",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameSubtrees",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "baseRevocationInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 9)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PerAuthorityScope: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PerAuthorityScope: &[ComponentSpec; 0] = &[];

pub fn _decode_PerAuthorityScope(el: &X690Element) -> ASN1Result<PerAuthorityScope> {
    |el_: &X690Element| -> ASN1Result<PerAuthorityScope> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PerAuthorityScope,
            _eal_components_for_PerAuthorityScope,
            _rctl2_components_for_PerAuthorityScope,
        )?;
        let authorityName: OPTIONAL<GeneralName> = match _components.get("authorityName") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                Ok(_decode_GeneralName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let distributionPoint: OPTIONAL<DistributionPointName> =
            match _components.get("distributionPoint") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistributionPointName> {
                    Ok(_decode_DistributionPointName(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let onlyContains: OPTIONAL<OnlyCertificateTypes> = match _components.get("onlyContains") {
            Some(c_) => Some(_decode_OnlyCertificateTypes(c_)?),
            _ => None,
        };
        let onlySomeReasons: OPTIONAL<ReasonFlags> = match _components.get("onlySomeReasons") {
            Some(c_) => Some(_decode_ReasonFlags(c_)?),
            _ => None,
        };
        let serialNumberRange: OPTIONAL<NumberRange> = match _components.get("serialNumberRange") {
            Some(c_) => Some(_decode_NumberRange(c_)?),
            _ => None,
        };
        let subjectKeyIdRange: OPTIONAL<NumberRange> = match _components.get("subjectKeyIdRange") {
            Some(c_) => Some(_decode_NumberRange(c_)?),
            _ => None,
        };
        let nameSubtrees: OPTIONAL<GeneralNames> = match _components.get("nameSubtrees") {
            Some(c_) => Some(_decode_GeneralNames(c_)?),
            _ => None,
        };
        let baseRevocationInfo: OPTIONAL<BaseRevocationInfo> =
            match _components.get("baseRevocationInfo") {
                Some(c_) => Some(_decode_BaseRevocationInfo(c_)?),
                _ => None,
            };
        Ok(PerAuthorityScope {
            authorityName,
            distributionPoint,
            onlyContains,
            onlySomeReasons,
            serialNumberRange,
            subjectKeyIdRange,
            nameSubtrees,
            baseRevocationInfo,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PerAuthorityScope(value_: &PerAuthorityScope) -> ASN1Result<X690Element> {
    |value_: &PerAuthorityScope| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(18);
        if let Some(v_) = &value_.authorityName {
            components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.distributionPoint {
            components_.push(|v_1: &DistributionPointName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_DistributionPointName(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.onlyContains {
            components_.push(|v_1: &OnlyCertificateTypes| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_OnlyCertificateTypes(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.onlySomeReasons {
            components_.push(|v_1: &ReasonFlags| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ReasonFlags(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 4;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.serialNumberRange {
            components_.push(|v_1: &NumberRange| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_NumberRange(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 5;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.subjectKeyIdRange {
            components_.push(|v_1: &NumberRange| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_NumberRange(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 6;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameSubtrees {
            components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralNames(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 7;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.baseRevocationInfo {
            components_.push(|v_1: &BaseRevocationInfo| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_BaseRevocationInfo(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 9;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OnlyCertificateTypes  ::=  BIT STRING {
///   user      (0),
///   authority (1),
///   attribute (2)}
/// ```
pub type OnlyCertificateTypes = BIT_STRING;

pub const OnlyCertificateTypes_user: BIT = 0; /* LONG_NAMED_BIT */

pub const OnlyCertificateTypes_authority: BIT = 1; /* LONG_NAMED_BIT */

pub const OnlyCertificateTypes_attribute: BIT = 2; /* LONG_NAMED_BIT */

pub fn _decode_OnlyCertificateTypes(el: &X690Element) -> ASN1Result<OnlyCertificateTypes> {
    ber_decode_bit_string(&el)
}

pub fn _encode_OnlyCertificateTypes(value_: &OnlyCertificateTypes) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NumberRange ::= SEQUENCE {
///   startingNumber  [0]  INTEGER OPTIONAL,
///   endingNumber    [1]  INTEGER OPTIONAL,
///   modulus              INTEGER OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct NumberRange {
    pub startingNumber: OPTIONAL<INTEGER>,
    pub endingNumber: OPTIONAL<INTEGER>,
    pub modulus: OPTIONAL<INTEGER>,
    pub _unrecognized: Vec<X690Element>,
}
impl NumberRange {
    pub fn new(
        startingNumber: OPTIONAL<INTEGER>,
        endingNumber: OPTIONAL<INTEGER>,
        modulus: OPTIONAL<INTEGER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        NumberRange {
            startingNumber,
            endingNumber,
            modulus,
            _unrecognized,
        }
    }
}
impl Default for NumberRange {
    fn default() -> Self {
        NumberRange {
            startingNumber: None,
            endingNumber: None,
            modulus: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for NumberRange {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_NumberRange(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for NumberRange {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_NumberRange(el)
    }
}

pub const _rctl1_components_for_NumberRange: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "startingNumber",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "endingNumber",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "modulus",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_NumberRange: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_NumberRange: &[ComponentSpec; 0] = &[];

pub fn _decode_NumberRange(el: &X690Element) -> ASN1Result<NumberRange> {
    |el_: &X690Element| -> ASN1Result<NumberRange> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_NumberRange,
            _eal_components_for_NumberRange,
            _rctl2_components_for_NumberRange,
        )?;
        let startingNumber: OPTIONAL<INTEGER> = match _components.get("startingNumber") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        let endingNumber: OPTIONAL<INTEGER> = match _components.get("endingNumber") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        let modulus: OPTIONAL<INTEGER> = match _components.get("modulus") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        Ok(NumberRange {
            startingNumber,
            endingNumber,
            modulus,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_NumberRange(value_: &NumberRange) -> ASN1Result<X690Element> {
    |value_: &NumberRange| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.startingNumber {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_integer(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.endingNumber {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_integer(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.modulus {
            components_.push(ber_encode_integer(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BaseRevocationInfo ::= SEQUENCE {
///   cRLStreamIdentifier  [0]  CRLStreamIdentifier OPTIONAL,
///   cRLNumber            [1]  CRLNumber,
///   baseThisUpdate       [2]  GeneralizedTime,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct BaseRevocationInfo {
    pub cRLStreamIdentifier: OPTIONAL<CRLStreamIdentifier>,
    pub cRLNumber: CRLNumber,
    pub baseThisUpdate: GeneralizedTime,
    pub _unrecognized: Vec<X690Element>,
}
impl BaseRevocationInfo {
    pub fn new(
        cRLStreamIdentifier: OPTIONAL<CRLStreamIdentifier>,
        cRLNumber: CRLNumber,
        baseThisUpdate: GeneralizedTime,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        BaseRevocationInfo {
            cRLStreamIdentifier,
            cRLNumber,
            baseThisUpdate,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for BaseRevocationInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_BaseRevocationInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for BaseRevocationInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_BaseRevocationInfo(el)
    }
}

pub const _rctl1_components_for_BaseRevocationInfo: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "cRLStreamIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "cRLNumber",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "baseThisUpdate",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_BaseRevocationInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_BaseRevocationInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_BaseRevocationInfo(el: &X690Element) -> ASN1Result<BaseRevocationInfo> {
    |el_: &X690Element| -> ASN1Result<BaseRevocationInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_BaseRevocationInfo,
            _eal_components_for_BaseRevocationInfo,
            _rctl2_components_for_BaseRevocationInfo,
        )?;
        let cRLStreamIdentifier: OPTIONAL<CRLStreamIdentifier> =
            match _components.get("cRLStreamIdentifier") {
                Some(c_) => Some(_decode_CRLStreamIdentifier(c_)?),
                _ => None,
            };
        let cRLNumber = _decode_CRLNumber(_components.get("cRLNumber").unwrap())?;
        let baseThisUpdate =
            ber_decode_generalized_time(_components.get("baseThisUpdate").unwrap())?;
        Ok(BaseRevocationInfo {
            cRLStreamIdentifier,
            cRLNumber,
            baseThisUpdate,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_BaseRevocationInfo(value_: &BaseRevocationInfo) -> ASN1Result<X690Element> {
    |value_: &BaseRevocationInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.cRLStreamIdentifier {
            components_.push(|v_1: &CRLStreamIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CRLStreamIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(|v_1: &CRLNumber| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CRLNumber(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 1;
            Ok(el_1)
        }(&value_.cRLNumber)?);
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            let mut el_1 = ber_encode_generalized_time(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 2;
            Ok(el_1)
        }(&value_.baseThisUpdate)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// statusReferrals EXTENSION ::= {
///   SYNTAX         StatusReferrals
///   IDENTIFIED BY  id-ce-statusReferrals }
/// ```
///
///
pub fn statusReferrals() -> EXTENSION {
    EXTENSION {
        id: id_ce_statusReferrals(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// StatusReferrals  ::=  SEQUENCE SIZE (1..MAX) OF StatusReferral
/// ```
pub type StatusReferrals = Vec<StatusReferral>; // SequenceOfType

pub fn _decode_StatusReferrals(el: &X690Element) -> ASN1Result<StatusReferrals> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<StatusReferral>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<StatusReferral> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_StatusReferral(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_StatusReferrals(value_: &StatusReferrals) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<StatusReferral>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_StatusReferral(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// StatusReferral  ::=  CHOICE {
///   cRLReferral    [0]  CRLReferral,
///   otherReferral  [1]  INSTANCE OF OTHER-REFERRAL,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum StatusReferral {
    cRLReferral(CRLReferral),
    otherReferral(INSTANCE_OF),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for StatusReferral {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_StatusReferral(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for StatusReferral {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_StatusReferral(el)
    }
}

pub fn _decode_StatusReferral(el: &X690Element) -> ASN1Result<StatusReferral> {
    |el: &X690Element| -> ASN1Result<StatusReferral> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(StatusReferral::cRLReferral(_decode_CRLReferral(&el)?)),
            (TagClass::CONTEXT, 1) => {
                Ok(StatusReferral::otherReferral(ber_decode_instance_of(&el)?))
            }
            _ => Ok(StatusReferral::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_StatusReferral(value_: &StatusReferral) -> ASN1Result<X690Element> {
    |value: &StatusReferral| -> ASN1Result<X690Element> {
        match value {
            StatusReferral::cRLReferral(v) => |v_1: &CRLReferral| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CRLReferral(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            StatusReferral::otherReferral(v) => |v_1: &INSTANCE_OF| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_instance_of(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            StatusReferral::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CRLReferral ::= SEQUENCE {
///   issuer          [0]  GeneralName OPTIONAL,
///   location        [1]  GeneralName OPTIONAL,
///   deltaRefInfo    [2]  DeltaRefInfo OPTIONAL,
///   cRLScope             CRLScopeSyntax,
///   lastUpdate      [3]  GeneralizedTime OPTIONAL,
///   lastChangedCRL  [4]  GeneralizedTime OPTIONAL,
///   ...
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CRLReferral {
    pub issuer: OPTIONAL<GeneralName>,
    pub location: OPTIONAL<GeneralName>,
    pub deltaRefInfo: OPTIONAL<DeltaRefInfo>,
    pub cRLScope: CRLScopeSyntax,
    pub lastUpdate: OPTIONAL<GeneralizedTime>,
    pub lastChangedCRL: OPTIONAL<GeneralizedTime>,
    pub _unrecognized: Vec<X690Element>,
}
impl CRLReferral {
    pub fn new(
        issuer: OPTIONAL<GeneralName>,
        location: OPTIONAL<GeneralName>,
        deltaRefInfo: OPTIONAL<DeltaRefInfo>,
        cRLScope: CRLScopeSyntax,
        lastUpdate: OPTIONAL<GeneralizedTime>,
        lastChangedCRL: OPTIONAL<GeneralizedTime>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CRLReferral {
            issuer,
            location,
            deltaRefInfo,
            cRLScope,
            lastUpdate,
            lastChangedCRL,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CRLReferral {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CRLReferral(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CRLReferral {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CRLReferral(el)
    }
}

pub const _rctl1_components_for_CRLReferral: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "issuer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "location",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "deltaRefInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "cRLScope",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "lastUpdate",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "lastChangedCRL",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CRLReferral: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CRLReferral: &[ComponentSpec; 0] = &[];

pub fn _decode_CRLReferral(el: &X690Element) -> ASN1Result<CRLReferral> {
    |el_: &X690Element| -> ASN1Result<CRLReferral> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CRLReferral,
            _eal_components_for_CRLReferral,
            _rctl2_components_for_CRLReferral,
        )?;
        let issuer: OPTIONAL<GeneralName> = match _components.get("issuer") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                Ok(_decode_GeneralName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let location: OPTIONAL<GeneralName> = match _components.get("location") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                Ok(_decode_GeneralName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let deltaRefInfo: OPTIONAL<DeltaRefInfo> = match _components.get("deltaRefInfo") {
            Some(c_) => Some(_decode_DeltaRefInfo(c_)?),
            _ => None,
        };
        let cRLScope = _decode_CRLScopeSyntax(_components.get("cRLScope").unwrap())?;
        let lastUpdate: OPTIONAL<GeneralizedTime> = match _components.get("lastUpdate") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        let lastChangedCRL: OPTIONAL<GeneralizedTime> = match _components.get("lastChangedCRL") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        Ok(CRLReferral {
            issuer,
            location,
            deltaRefInfo,
            cRLScope,
            lastUpdate,
            lastChangedCRL,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CRLReferral(value_: &CRLReferral) -> ASN1Result<X690Element> {
    |value_: &CRLReferral| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        if let Some(v_) = &value_.issuer {
            components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.location {
            components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.deltaRefInfo {
            components_.push(|v_1: &DeltaRefInfo| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_DeltaRefInfo(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(_encode_CRLScopeSyntax(&value_.cRLScope)?);
        if let Some(v_) = &value_.lastUpdate {
            components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_generalized_time(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.lastChangedCRL {
            components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_generalized_time(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 4;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DeltaRefInfo ::= SEQUENCE {
///   deltaLocation  GeneralName,
///   lastDelta      GeneralizedTime OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DeltaRefInfo {
    pub deltaLocation: GeneralName,
    pub lastDelta: OPTIONAL<GeneralizedTime>,
    pub _unrecognized: Vec<X690Element>,
}
impl DeltaRefInfo {
    pub fn new(
        deltaLocation: GeneralName,
        lastDelta: OPTIONAL<GeneralizedTime>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DeltaRefInfo {
            deltaLocation,
            lastDelta,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for DeltaRefInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DeltaRefInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DeltaRefInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DeltaRefInfo(el)
    }
}

pub const _rctl1_components_for_DeltaRefInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new("deltaLocation", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "lastDelta",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DeltaRefInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DeltaRefInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_DeltaRefInfo(el: &X690Element) -> ASN1Result<DeltaRefInfo> {
    |el_: &X690Element| -> ASN1Result<DeltaRefInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DeltaRefInfo,
            _eal_components_for_DeltaRefInfo,
            _rctl2_components_for_DeltaRefInfo,
        )?;
        let deltaLocation = _decode_GeneralName(_components.get("deltaLocation").unwrap())?;
        let lastDelta: OPTIONAL<GeneralizedTime> = match _components.get("lastDelta") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        Ok(DeltaRefInfo {
            deltaLocation,
            lastDelta,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DeltaRefInfo(value_: &DeltaRefInfo) -> ASN1Result<X690Element> {
    |value_: &DeltaRefInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_GeneralName(&value_.deltaLocation)?);
        if let Some(v_) = &value_.lastDelta {
            components_.push(ber_encode_generalized_time(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

pub type OTHER_REFERRAL = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// cRLStreamIdentifier EXTENSION ::= {
///   SYNTAX         CRLStreamIdentifier
///   IDENTIFIED BY  id-ce-cRLStreamIdentifier }
/// ```
///
///
pub fn cRLStreamIdentifier() -> EXTENSION {
    EXTENSION {
        id: id_ce_cRLStreamIdentifier(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CRLStreamIdentifier  ::=  INTEGER (0..MAX)
/// ```
pub type CRLStreamIdentifier = INTEGER;

pub fn _decode_CRLStreamIdentifier(el: &X690Element) -> ASN1Result<CRLStreamIdentifier> {
    ber_decode_integer(&el)
}

pub fn _encode_CRLStreamIdentifier(value_: &CRLStreamIdentifier) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// orderedList EXTENSION ::= {
///   SYNTAX         OrderedListSyntax
///   IDENTIFIED BY  id-ce-orderedList }
/// ```
///
///
pub fn orderedList() -> EXTENSION {
    EXTENSION {
        id: id_ce_orderedList(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OrderedListSyntax  ::=  ENUMERATED {
///   ascSerialNum (0),
///   ascRevDate   (1),
///   ...}
/// ```
pub type OrderedListSyntax = ENUMERATED;

pub const OrderedListSyntax_ascSerialNum: OrderedListSyntax = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const OrderedListSyntax_ascRevDate: OrderedListSyntax = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_OrderedListSyntax(el: &X690Element) -> ASN1Result<OrderedListSyntax> {
    ber_decode_enumerated(&el)
}

pub fn _encode_OrderedListSyntax(value_: &OrderedListSyntax) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// deltaInfo EXTENSION ::= {
///   SYNTAX         DeltaInformation
///   IDENTIFIED BY  id-ce-deltaInfo }
/// ```
///
///
pub fn deltaInfo() -> EXTENSION {
    EXTENSION {
        id: id_ce_deltaInfo(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DeltaInformation ::= SEQUENCE {
///   deltaLocation  GeneralName,
///   nextDelta      GeneralizedTime OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DeltaInformation {
    pub deltaLocation: GeneralName,
    pub nextDelta: OPTIONAL<GeneralizedTime>,
    pub _unrecognized: Vec<X690Element>,
}
impl DeltaInformation {
    pub fn new(
        deltaLocation: GeneralName,
        nextDelta: OPTIONAL<GeneralizedTime>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DeltaInformation {
            deltaLocation,
            nextDelta,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for DeltaInformation {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DeltaInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DeltaInformation {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DeltaInformation(el)
    }
}

pub const _rctl1_components_for_DeltaInformation: &[ComponentSpec; 2] = &[
    ComponentSpec::new("deltaLocation", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "nextDelta",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DeltaInformation: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DeltaInformation: &[ComponentSpec; 0] = &[];

pub fn _decode_DeltaInformation(el: &X690Element) -> ASN1Result<DeltaInformation> {
    |el_: &X690Element| -> ASN1Result<DeltaInformation> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DeltaInformation,
            _eal_components_for_DeltaInformation,
            _rctl2_components_for_DeltaInformation,
        )?;
        let deltaLocation = _decode_GeneralName(_components.get("deltaLocation").unwrap())?;
        let nextDelta: OPTIONAL<GeneralizedTime> = match _components.get("nextDelta") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        Ok(DeltaInformation {
            deltaLocation,
            nextDelta,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DeltaInformation(value_: &DeltaInformation) -> ASN1Result<X690Element> {
    |value_: &DeltaInformation| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_GeneralName(&value_.deltaLocation)?);
        if let Some(v_) = &value_.nextDelta {
            components_.push(ber_encode_generalized_time(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// toBeRevoked EXTENSION ::= {
///   SYNTAX         ToBeRevokedSyntax
///   IDENTIFIED BY  id-ce-toBeRevoked }
/// ```
///
///
pub fn toBeRevoked() -> EXTENSION {
    EXTENSION {
        id: id_ce_toBeRevoked(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ToBeRevokedSyntax  ::=  SEQUENCE SIZE (1..MAX) OF ToBeRevokedGroup
/// ```
pub type ToBeRevokedSyntax = Vec<ToBeRevokedGroup>; // SequenceOfType

pub fn _decode_ToBeRevokedSyntax(el: &X690Element) -> ASN1Result<ToBeRevokedSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ToBeRevokedGroup>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<ToBeRevokedGroup> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_ToBeRevokedGroup(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_ToBeRevokedSyntax(value_: &ToBeRevokedSyntax) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<ToBeRevokedGroup>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_ToBeRevokedGroup(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ToBeRevokedGroup ::= SEQUENCE {
///   certificateIssuer  [0]  GeneralName OPTIONAL,
///   reasonInfo         [1]  ReasonInfo OPTIONAL,
///   revocationTime          GeneralizedTime,
///   certificateGroup        CertificateGroup,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ToBeRevokedGroup {
    pub certificateIssuer: OPTIONAL<GeneralName>,
    pub reasonInfo: OPTIONAL<ReasonInfo>,
    pub revocationTime: GeneralizedTime,
    pub certificateGroup: CertificateGroup,
    pub _unrecognized: Vec<X690Element>,
}
impl ToBeRevokedGroup {
    pub fn new(
        certificateIssuer: OPTIONAL<GeneralName>,
        reasonInfo: OPTIONAL<ReasonInfo>,
        revocationTime: GeneralizedTime,
        certificateGroup: CertificateGroup,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ToBeRevokedGroup {
            certificateIssuer,
            reasonInfo,
            revocationTime,
            certificateGroup,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ToBeRevokedGroup {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ToBeRevokedGroup(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ToBeRevokedGroup {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ToBeRevokedGroup(el)
    }
}

pub const _rctl1_components_for_ToBeRevokedGroup: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "certificateIssuer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "reasonInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "revocationTime",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new("certificateGroup", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_ToBeRevokedGroup: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ToBeRevokedGroup: &[ComponentSpec; 0] = &[];

pub fn _decode_ToBeRevokedGroup(el: &X690Element) -> ASN1Result<ToBeRevokedGroup> {
    |el_: &X690Element| -> ASN1Result<ToBeRevokedGroup> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ToBeRevokedGroup,
            _eal_components_for_ToBeRevokedGroup,
            _rctl2_components_for_ToBeRevokedGroup,
        )?;
        let certificateIssuer: OPTIONAL<GeneralName> = match _components.get("certificateIssuer") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                Ok(_decode_GeneralName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let reasonInfo: OPTIONAL<ReasonInfo> = match _components.get("reasonInfo") {
            Some(c_) => Some(_decode_ReasonInfo(c_)?),
            _ => None,
        };
        let revocationTime =
            ber_decode_generalized_time(_components.get("revocationTime").unwrap())?;
        let certificateGroup =
            _decode_CertificateGroup(_components.get("certificateGroup").unwrap())?;
        Ok(ToBeRevokedGroup {
            certificateIssuer,
            reasonInfo,
            revocationTime,
            certificateGroup,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ToBeRevokedGroup(value_: &ToBeRevokedGroup) -> ASN1Result<X690Element> {
    |value_: &ToBeRevokedGroup| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        if let Some(v_) = &value_.certificateIssuer {
            components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.reasonInfo {
            components_.push(|v_1: &ReasonInfo| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ReasonInfo(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(ber_encode_generalized_time(&value_.revocationTime)?);
        components_.push(_encode_CertificateGroup(&value_.certificateGroup)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReasonInfo ::= SEQUENCE {
///   reasonCode           CRLReason,
///   holdInstructionCode  HoldInstruction OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ReasonInfo {
    pub reasonCode: CRLReason,
    pub holdInstructionCode: OPTIONAL<HoldInstruction>,
    pub _unrecognized: Vec<X690Element>,
}
impl ReasonInfo {
    pub fn new(
        reasonCode: CRLReason,
        holdInstructionCode: OPTIONAL<HoldInstruction>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ReasonInfo {
            reasonCode,
            holdInstructionCode,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ReasonInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ReasonInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ReasonInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ReasonInfo(el)
    }
}

pub const _rctl1_components_for_ReasonInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "reasonCode",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "holdInstructionCode",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ReasonInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ReasonInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_ReasonInfo(el: &X690Element) -> ASN1Result<ReasonInfo> {
    |el_: &X690Element| -> ASN1Result<ReasonInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ReasonInfo,
            _eal_components_for_ReasonInfo,
            _rctl2_components_for_ReasonInfo,
        )?;
        let reasonCode = _decode_CRLReason(_components.get("reasonCode").unwrap())?;
        let holdInstructionCode: OPTIONAL<HoldInstruction> =
            match _components.get("holdInstructionCode") {
                Some(c_) => Some(_decode_HoldInstruction(c_)?),
                _ => None,
            };
        Ok(ReasonInfo {
            reasonCode,
            holdInstructionCode,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ReasonInfo(value_: &ReasonInfo) -> ASN1Result<X690Element> {
    |value_: &ReasonInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_CRLReason(&value_.reasonCode)?);
        if let Some(v_) = &value_.holdInstructionCode {
            components_.push(_encode_HoldInstruction(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateGroup  ::=  CHOICE {
///   serialNumbers      [0]  CertificateSerialNumbers,
///   serialNumberRange  [1]  CertificateGroupNumberRange,
///   nameSubtree        [2]  GeneralName,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum CertificateGroup {
    serialNumbers(CertificateSerialNumbers),
    serialNumberRange(CertificateGroupNumberRange),
    nameSubtree(GeneralName),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CertificateGroup {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateGroup(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificateGroup {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateGroup(el)
    }
}

pub fn _decode_CertificateGroup(el: &X690Element) -> ASN1Result<CertificateGroup> {
    |el: &X690Element| -> ASN1Result<CertificateGroup> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(CertificateGroup::serialNumbers(
                _decode_CertificateSerialNumbers(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(CertificateGroup::serialNumberRange(
                _decode_CertificateGroupNumberRange(&el)?,
            )),
            (TagClass::CONTEXT, 2) => Ok(CertificateGroup::nameSubtree(
                |el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(&el)?,
            )),
            _ => Ok(CertificateGroup::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CertificateGroup(value_: &CertificateGroup) -> ASN1Result<X690Element> {
    |value: &CertificateGroup| -> ASN1Result<X690Element> {
        match value {
            CertificateGroup::serialNumbers(v) => {
                |v_1: &CertificateSerialNumbers| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertificateSerialNumbers(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            CertificateGroup::serialNumberRange(v) => {
                |v_1: &CertificateGroupNumberRange| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertificateGroupNumberRange(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            CertificateGroup::nameSubtree(v) => |v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v),
            CertificateGroup::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateGroupNumberRange ::= SEQUENCE {
///   startingNumber  [0]  INTEGER,
///   endingNumber    [1]  INTEGER,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertificateGroupNumberRange {
    pub startingNumber: INTEGER,
    pub endingNumber: INTEGER,
    pub _unrecognized: Vec<X690Element>,
}
impl CertificateGroupNumberRange {
    pub fn new(
        startingNumber: INTEGER,
        endingNumber: INTEGER,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertificateGroupNumberRange {
            startingNumber,
            endingNumber,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertificateGroupNumberRange {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateGroupNumberRange(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificateGroupNumberRange {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateGroupNumberRange(el)
    }
}

pub const _rctl1_components_for_CertificateGroupNumberRange: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "startingNumber",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "endingNumber",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificateGroupNumberRange: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertificateGroupNumberRange: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificateGroupNumberRange(
    el: &X690Element,
) -> ASN1Result<CertificateGroupNumberRange> {
    |el_: &X690Element| -> ASN1Result<CertificateGroupNumberRange> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificateGroupNumberRange,
            _eal_components_for_CertificateGroupNumberRange,
            _rctl2_components_for_CertificateGroupNumberRange,
        )?;
        let startingNumber = ber_decode_integer(_components.get("startingNumber").unwrap())?;
        let endingNumber = ber_decode_integer(_components.get("endingNumber").unwrap())?;
        Ok(CertificateGroupNumberRange {
            startingNumber,
            endingNumber,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertificateGroupNumberRange(
    value_: &CertificateGroupNumberRange,
) -> ASN1Result<X690Element> {
    |value_: &CertificateGroupNumberRange| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            let mut el_1 = ber_encode_integer(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 0;
            Ok(el_1)
        }(&value_.startingNumber)?);
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            let mut el_1 = ber_encode_integer(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 1;
            Ok(el_1)
        }(&value_.endingNumber)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateSerialNumbers  ::=  SEQUENCE SIZE (1..MAX) OF CertificateSerialNumber
/// ```
pub type CertificateSerialNumbers = Vec<CertificateSerialNumber>; // SequenceOfType

pub fn _decode_CertificateSerialNumbers(el: &X690Element) -> ASN1Result<CertificateSerialNumbers> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertificateSerialNumber>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<CertificateSerialNumber> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_CertificateSerialNumber(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CertificateSerialNumbers(
    value_: &CertificateSerialNumbers,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<CertificateSerialNumber>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_CertificateSerialNumber(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// revokedGroups EXTENSION ::= {
///   SYNTAX         RevokedGroupsSyntax
///   IDENTIFIED BY  id-ce-revokedGroups }
/// ```
///
///
pub fn revokedGroups() -> EXTENSION {
    EXTENSION {
        id: id_ce_revokedGroups(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RevokedGroupsSyntax  ::=  SEQUENCE SIZE (1..MAX) OF RevokedGroup
/// ```
pub type RevokedGroupsSyntax = Vec<RevokedGroup>; // SequenceOfType

pub fn _decode_RevokedGroupsSyntax(el: &X690Element) -> ASN1Result<RevokedGroupsSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<RevokedGroup>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<RevokedGroup> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_RevokedGroup(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_RevokedGroupsSyntax(value_: &RevokedGroupsSyntax) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<RevokedGroup>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_RevokedGroup(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RevokedGroup ::= SEQUENCE {
///   certificateIssuer        [0]  GeneralName OPTIONAL,
///   reasonInfo               [1]  ReasonInfo OPTIONAL,
///   invalidityDate           [2]  GeneralizedTime OPTIONAL,
///   revokedcertificateGroup  [3]  RevokedCertificateGroup,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RevokedGroup {
    pub certificateIssuer: OPTIONAL<GeneralName>,
    pub reasonInfo: OPTIONAL<ReasonInfo>,
    pub invalidityDate: OPTIONAL<GeneralizedTime>,
    pub revokedcertificateGroup: RevokedCertificateGroup,
    pub _unrecognized: Vec<X690Element>,
}
impl RevokedGroup {
    pub fn new(
        certificateIssuer: OPTIONAL<GeneralName>,
        reasonInfo: OPTIONAL<ReasonInfo>,
        invalidityDate: OPTIONAL<GeneralizedTime>,
        revokedcertificateGroup: RevokedCertificateGroup,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        RevokedGroup {
            certificateIssuer,
            reasonInfo,
            invalidityDate,
            revokedcertificateGroup,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for RevokedGroup {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RevokedGroup(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RevokedGroup {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RevokedGroup(el)
    }
}

pub const _rctl1_components_for_RevokedGroup: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "certificateIssuer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "reasonInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "invalidityDate",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "revokedcertificateGroup",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RevokedGroup: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RevokedGroup: &[ComponentSpec; 0] = &[];

pub fn _decode_RevokedGroup(el: &X690Element) -> ASN1Result<RevokedGroup> {
    |el_: &X690Element| -> ASN1Result<RevokedGroup> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RevokedGroup,
            _eal_components_for_RevokedGroup,
            _rctl2_components_for_RevokedGroup,
        )?;
        let certificateIssuer: OPTIONAL<GeneralName> = match _components.get("certificateIssuer") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                Ok(_decode_GeneralName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let reasonInfo: OPTIONAL<ReasonInfo> = match _components.get("reasonInfo") {
            Some(c_) => Some(_decode_ReasonInfo(c_)?),
            _ => None,
        };
        let invalidityDate: OPTIONAL<GeneralizedTime> = match _components.get("invalidityDate") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        let revokedcertificateGroup =
            |el: &X690Element| -> ASN1Result<RevokedCertificateGroup> {
                Ok(_decode_RevokedCertificateGroup(&el.inner()?)?)
            }(_components.get("revokedcertificateGroup").unwrap())?;
        Ok(RevokedGroup {
            certificateIssuer,
            reasonInfo,
            invalidityDate,
            revokedcertificateGroup,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_RevokedGroup(value_: &RevokedGroup) -> ASN1Result<X690Element> {
    |value_: &RevokedGroup| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        if let Some(v_) = &value_.certificateIssuer {
            components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.reasonInfo {
            components_.push(|v_1: &ReasonInfo| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ReasonInfo(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.invalidityDate {
            components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_generalized_time(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(|v_1: &RevokedCertificateGroup| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                3,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_RevokedCertificateGroup(&v_1)?,
                ))),
            ))
        }(&value_.revokedcertificateGroup)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RevokedCertificateGroup  ::=  CHOICE {
///   serialNumberRange  NumberRange,
///   nameSubtree        GeneralName }
/// ```
#[derive(Debug, Clone)]
pub enum RevokedCertificateGroup {
    serialNumberRange(NumberRange),
    nameSubtree(GeneralName),
}

impl TryFrom<X690Element> for RevokedCertificateGroup {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RevokedCertificateGroup(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RevokedCertificateGroup {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RevokedCertificateGroup(el)
    }
}

pub fn _decode_RevokedCertificateGroup(el: &X690Element) -> ASN1Result<RevokedCertificateGroup> {
    |el: &X690Element| -> ASN1Result<RevokedCertificateGroup> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(RevokedCertificateGroup::serialNumberRange(
                _decode_NumberRange(&el)?,
            )),
            (TagClass::CONTEXT, 0) => Ok(RevokedCertificateGroup::nameSubtree(
                _decode_GeneralName(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(RevokedCertificateGroup::nameSubtree(
                _decode_GeneralName(&el)?,
            )),
            (TagClass::CONTEXT, 2) => Ok(RevokedCertificateGroup::nameSubtree(
                _decode_GeneralName(&el)?,
            )),
            (TagClass::CONTEXT, 3) => Ok(RevokedCertificateGroup::nameSubtree(
                _decode_GeneralName(&el)?,
            )),
            (TagClass::CONTEXT, 4) => Ok(RevokedCertificateGroup::nameSubtree(
                _decode_GeneralName(&el)?,
            )),
            (TagClass::CONTEXT, 5) => Ok(RevokedCertificateGroup::nameSubtree(
                _decode_GeneralName(&el)?,
            )),
            (TagClass::CONTEXT, 6) => Ok(RevokedCertificateGroup::nameSubtree(
                _decode_GeneralName(&el)?,
            )),
            (TagClass::CONTEXT, 7) => Ok(RevokedCertificateGroup::nameSubtree(
                _decode_GeneralName(&el)?,
            )),
            (TagClass::CONTEXT, 8) => Ok(RevokedCertificateGroup::nameSubtree(
                _decode_GeneralName(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_RevokedCertificateGroup(
    value_: &RevokedCertificateGroup,
) -> ASN1Result<X690Element> {
    |value: &RevokedCertificateGroup| -> ASN1Result<X690Element> {
        match value {
            RevokedCertificateGroup::serialNumberRange(v) => _encode_NumberRange(&v),
            RevokedCertificateGroup::nameSubtree(v) => _encode_GeneralName(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// expiredCertsOnCRL EXTENSION ::= {
///   SYNTAX         ExpiredCertsOnCRL
///   IDENTIFIED BY  id-ce-expiredCertsOnCRL }
/// ```
///
///
pub fn expiredCertsOnCRL() -> EXTENSION {
    EXTENSION {
        id: id_ce_expiredCertsOnCRL(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExpiredCertsOnCRL  ::=  GeneralizedTime
/// ```
pub type ExpiredCertsOnCRL = GeneralizedTime; // GeneralizedTime

pub fn _decode_ExpiredCertsOnCRL(el: &X690Element) -> ASN1Result<ExpiredCertsOnCRL> {
    ber_decode_generalized_time(&el)
}

pub fn _encode_ExpiredCertsOnCRL(value_: &ExpiredCertsOnCRL) -> ASN1Result<X690Element> {
    ber_encode_generalized_time(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// reasonCode EXTENSION ::= {
///   SYNTAX         CRLReason
///   IDENTIFIED BY  id-ce-reasonCode }
/// ```
///
///
pub fn reasonCode() -> EXTENSION {
    EXTENSION {
        id: id_ce_reasonCode(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CRLReason  ::=  ENUMERATED {
///   unspecified          (0),
///   keyCompromise        (1),
///   cACompromise         (2),
///   affiliationChanged   (3),
///   superseded           (4),
///   cessationOfOperation (5),
///   certificateHold      (6),
///   removeFromCRL        (8),
///   privilegeWithdrawn   (9),
///   aACompromise         (10),
///   ...,
///   weakAlgorithmOrKey   (11) }
/// ```
pub type CRLReason = ENUMERATED;

pub const CRLReason_unspecified: CRLReason = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CRLReason_keyCompromise: CRLReason = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CRLReason_cACompromise: CRLReason = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CRLReason_affiliationChanged: CRLReason = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CRLReason_superseded: CRLReason = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CRLReason_cessationOfOperation: CRLReason = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CRLReason_certificateHold: CRLReason = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CRLReason_removeFromCRL: CRLReason = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CRLReason_privilegeWithdrawn: CRLReason = 9; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CRLReason_aACompromise: CRLReason = 10; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CRLReason_weakAlgorithmOrKey: CRLReason = 11; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_CRLReason(el: &X690Element) -> ASN1Result<CRLReason> {
    ber_decode_enumerated(&el)
}

pub fn _encode_CRLReason(value_: &CRLReason) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// holdInstructionCode EXTENSION ::= {
///   SYNTAX         HoldInstruction
///   IDENTIFIED BY  id-ce-holdInstructionCode }
/// ```
///
///
pub fn holdInstructionCode() -> EXTENSION {
    EXTENSION {
        id: id_ce_holdInstructionCode(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HoldInstruction  ::=  OBJECT IDENTIFIER
/// ```
pub type HoldInstruction = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_HoldInstruction(el: &X690Element) -> ASN1Result<HoldInstruction> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_HoldInstruction(value_: &HoldInstruction) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// invalidityDate EXTENSION ::= {
///   SYNTAX         GeneralizedTime
///   IDENTIFIED BY  id-ce-invalidityDate }
/// ```
///
///
pub fn invalidityDate() -> EXTENSION {
    EXTENSION {
        id: id_ce_invalidityDate(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cRLDistributionPoints EXTENSION ::= {
///   SYNTAX         CRLDistPointsSyntax
///   IDENTIFIED BY  id-ce-cRLDistributionPoints }
/// ```
///
///
pub fn cRLDistributionPoints() -> EXTENSION {
    EXTENSION {
        id: id_ce_cRLDistributionPoints(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CRLDistPointsSyntax  ::=  SEQUENCE SIZE (1..MAX) OF DistributionPoint
/// ```
pub type CRLDistPointsSyntax = Vec<DistributionPoint>; // SequenceOfType

pub fn _decode_CRLDistPointsSyntax(el: &X690Element) -> ASN1Result<CRLDistPointsSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<DistributionPoint>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<DistributionPoint> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_DistributionPoint(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CRLDistPointsSyntax(value_: &CRLDistPointsSyntax) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<DistributionPoint>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_DistributionPoint(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DistributionPoint ::= SEQUENCE {
///   distributionPoint  [0]  DistributionPointName OPTIONAL,
///   reasons            [1]  ReasonFlags OPTIONAL,
///   cRLIssuer          [2]  GeneralNames OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DistributionPoint {
    pub distributionPoint: OPTIONAL<DistributionPointName>,
    pub reasons: OPTIONAL<ReasonFlags>,
    pub cRLIssuer: OPTIONAL<GeneralNames>,
    pub _unrecognized: Vec<X690Element>,
}
impl DistributionPoint {
    pub fn new(
        distributionPoint: OPTIONAL<DistributionPointName>,
        reasons: OPTIONAL<ReasonFlags>,
        cRLIssuer: OPTIONAL<GeneralNames>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DistributionPoint {
            distributionPoint,
            reasons,
            cRLIssuer,
            _unrecognized,
        }
    }
}
impl Default for DistributionPoint {
    fn default() -> Self {
        DistributionPoint {
            distributionPoint: None,
            reasons: None,
            cRLIssuer: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for DistributionPoint {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DistributionPoint(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DistributionPoint {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DistributionPoint(el)
    }
}

pub const _rctl1_components_for_DistributionPoint: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "distributionPoint",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "reasons",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "cRLIssuer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DistributionPoint: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DistributionPoint: &[ComponentSpec; 0] = &[];

pub fn _decode_DistributionPoint(el: &X690Element) -> ASN1Result<DistributionPoint> {
    |el_: &X690Element| -> ASN1Result<DistributionPoint> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DistributionPoint,
            _eal_components_for_DistributionPoint,
            _rctl2_components_for_DistributionPoint,
        )?;
        let distributionPoint: OPTIONAL<DistributionPointName> =
            match _components.get("distributionPoint") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistributionPointName> {
                    Ok(_decode_DistributionPointName(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let reasons: OPTIONAL<ReasonFlags> = match _components.get("reasons") {
            Some(c_) => Some(_decode_ReasonFlags(c_)?),
            _ => None,
        };
        let cRLIssuer: OPTIONAL<GeneralNames> = match _components.get("cRLIssuer") {
            Some(c_) => Some(_decode_GeneralNames(c_)?),
            _ => None,
        };
        Ok(DistributionPoint {
            distributionPoint,
            reasons,
            cRLIssuer,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DistributionPoint(value_: &DistributionPoint) -> ASN1Result<X690Element> {
    |value_: &DistributionPoint| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.distributionPoint {
            components_.push(|v_1: &DistributionPointName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_DistributionPointName(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.reasons {
            components_.push(|v_1: &ReasonFlags| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ReasonFlags(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.cRLIssuer {
            components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralNames(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DistributionPointName  ::=  CHOICE {
///   fullName                 [0]  GeneralNames,
///   nameRelativeToCRLIssuer  [1]  RelativeDistinguishedName,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum DistributionPointName {
    fullName(GeneralNames),
    nameRelativeToCRLIssuer(RelativeDistinguishedName),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for DistributionPointName {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DistributionPointName(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DistributionPointName {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DistributionPointName(el)
    }
}

pub fn _decode_DistributionPointName(el: &X690Element) -> ASN1Result<DistributionPointName> {
    |el: &X690Element| -> ASN1Result<DistributionPointName> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => {
                Ok(DistributionPointName::fullName(_decode_GeneralNames(&el)?))
            }
            (TagClass::CONTEXT, 1) => Ok(DistributionPointName::nameRelativeToCRLIssuer(
                _decode_RelativeDistinguishedName(&el)?,
            )),
            _ => Ok(DistributionPointName::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_DistributionPointName(value_: &DistributionPointName) -> ASN1Result<X690Element> {
    |value: &DistributionPointName| -> ASN1Result<X690Element> {
        match value {
            DistributionPointName::fullName(v) => |v_1: &GeneralNames| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralNames(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            DistributionPointName::nameRelativeToCRLIssuer(v) => {
                |v_1: &RelativeDistinguishedName| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_RelativeDistinguishedName(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            DistributionPointName::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReasonFlags  ::=  BIT STRING {
///   unused                (0),
///   keyCompromise         (1),
///   cACompromise          (2),
///   affiliationChanged    (3),
///   superseded            (4),
///   cessationOfOperation  (5),
///   certificateHold       (6),
///   privilegeWithdrawn    (7),
///   aACompromise          (8),
///   weakAlgorithmOrKey    (9) }
/// ```
pub type ReasonFlags = BIT_STRING;

pub const ReasonFlags_unused: BIT = 0; /* LONG_NAMED_BIT */

pub const ReasonFlags_keyCompromise: BIT = 1; /* LONG_NAMED_BIT */

pub const ReasonFlags_cACompromise: BIT = 2; /* LONG_NAMED_BIT */

pub const ReasonFlags_affiliationChanged: BIT = 3; /* LONG_NAMED_BIT */

pub const ReasonFlags_superseded: BIT = 4; /* LONG_NAMED_BIT */

pub const ReasonFlags_cessationOfOperation: BIT = 5; /* LONG_NAMED_BIT */

pub const ReasonFlags_certificateHold: BIT = 6; /* LONG_NAMED_BIT */

pub const ReasonFlags_privilegeWithdrawn: BIT = 7; /* LONG_NAMED_BIT */

pub const ReasonFlags_aACompromise: BIT = 8; /* LONG_NAMED_BIT */

pub const ReasonFlags_weakAlgorithmOrKey: BIT = 9; /* LONG_NAMED_BIT */

pub fn _decode_ReasonFlags(el: &X690Element) -> ASN1Result<ReasonFlags> {
    ber_decode_bit_string(&el)
}

pub fn _encode_ReasonFlags(value_: &ReasonFlags) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// issuingDistributionPoint EXTENSION ::= {
///   SYNTAX         IssuingDistPointSyntax
///   IDENTIFIED BY  id-ce-issuingDistributionPoint }
/// ```
///
///
pub fn issuingDistributionPoint() -> EXTENSION {
    EXTENSION {
        id: id_ce_issuingDistributionPoint(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IssuingDistPointSyntax ::= SEQUENCE {
///   -- If onlyContainsUserPublicKeyCerts and onlyContainsCACerts are both FALSE,
///   -- the CRL covers both public-key certificate types
///   distributionPoint               [0]  DistributionPointName OPTIONAL,
///   onlyContainsUserPublicKeyCerts  [1]  BOOLEAN DEFAULT FALSE,
///   onlyContainsCACerts             [2]  BOOLEAN DEFAULT FALSE,
///   onlySomeReasons                 [3]  ReasonFlags OPTIONAL,
///   indirectCRL                     [4]  BOOLEAN DEFAULT FALSE,
///   onlyContainsAttributeCerts      [5]  BOOLEAN OPTIONAL, -- Use is strongly deprecated
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct IssuingDistPointSyntax {
    pub distributionPoint: OPTIONAL<DistributionPointName>,
    pub onlyContainsUserPublicKeyCerts: OPTIONAL<BOOLEAN>,
    pub onlyContainsCACerts: OPTIONAL<BOOLEAN>,
    pub onlySomeReasons: OPTIONAL<ReasonFlags>,
    pub indirectCRL: OPTIONAL<BOOLEAN>,
    pub onlyContainsAttributeCerts: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl IssuingDistPointSyntax {
    pub fn new(
        distributionPoint: OPTIONAL<DistributionPointName>,
        onlyContainsUserPublicKeyCerts: OPTIONAL<BOOLEAN>,
        onlyContainsCACerts: OPTIONAL<BOOLEAN>,
        onlySomeReasons: OPTIONAL<ReasonFlags>,
        indirectCRL: OPTIONAL<BOOLEAN>,
        onlyContainsAttributeCerts: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        IssuingDistPointSyntax {
            distributionPoint,
            onlyContainsUserPublicKeyCerts,
            onlyContainsCACerts,
            onlySomeReasons,
            indirectCRL,
            onlyContainsAttributeCerts,
            _unrecognized,
        }
    }
    pub fn _default_value_for_onlyContainsUserPublicKeyCerts() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_onlyContainsCACerts() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_indirectCRL() -> BOOLEAN {
        false
    }
}
impl Default for IssuingDistPointSyntax {
    fn default() -> Self {
        IssuingDistPointSyntax {
            distributionPoint: None,
            onlyContainsUserPublicKeyCerts: None,
            onlyContainsCACerts: None,
            onlySomeReasons: None,
            indirectCRL: None,
            onlyContainsAttributeCerts: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for IssuingDistPointSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IssuingDistPointSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IssuingDistPointSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_IssuingDistPointSyntax(el)
    }
}

pub const _rctl1_components_for_IssuingDistPointSyntax: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "distributionPoint",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "onlyContainsUserPublicKeyCerts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "onlyContainsCACerts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "onlySomeReasons",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "indirectCRL",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "onlyContainsAttributeCerts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IssuingDistPointSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IssuingDistPointSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_IssuingDistPointSyntax(el: &X690Element) -> ASN1Result<IssuingDistPointSyntax> {
    |el_: &X690Element| -> ASN1Result<IssuingDistPointSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IssuingDistPointSyntax,
            _eal_components_for_IssuingDistPointSyntax,
            _rctl2_components_for_IssuingDistPointSyntax,
        )?;
        let distributionPoint: OPTIONAL<DistributionPointName> =
            match _components.get("distributionPoint") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistributionPointName> {
                    Ok(_decode_DistributionPointName(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let onlyContainsUserPublicKeyCerts: OPTIONAL<BOOLEAN> =
            match _components.get("onlyContainsUserPublicKeyCerts") {
                Some(c_) => Some(ber_decode_boolean(c_)?),
                _ => None,
            };
        let onlyContainsCACerts: OPTIONAL<BOOLEAN> = match _components.get("onlyContainsCACerts") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let onlySomeReasons: OPTIONAL<ReasonFlags> = match _components.get("onlySomeReasons") {
            Some(c_) => Some(_decode_ReasonFlags(c_)?),
            _ => None,
        };
        let indirectCRL: OPTIONAL<BOOLEAN> = match _components.get("indirectCRL") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let onlyContainsAttributeCerts: OPTIONAL<BOOLEAN> =
            match _components.get("onlyContainsAttributeCerts") {
                Some(c_) => Some(ber_decode_boolean(c_)?),
                _ => None,
            };
        Ok(IssuingDistPointSyntax {
            distributionPoint,
            onlyContainsUserPublicKeyCerts,
            onlyContainsCACerts,
            onlySomeReasons,
            indirectCRL,
            onlyContainsAttributeCerts,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_IssuingDistPointSyntax(value_: &IssuingDistPointSyntax) -> ASN1Result<X690Element> {
    |value_: &IssuingDistPointSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        if let Some(v_) = &value_.distributionPoint {
            components_.push(|v_1: &DistributionPointName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_DistributionPointName(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.onlyContainsUserPublicKeyCerts {
            if *v_ != IssuingDistPointSyntax::_default_value_for_onlyContainsUserPublicKeyCerts() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.onlyContainsCACerts {
            if *v_ != IssuingDistPointSyntax::_default_value_for_onlyContainsCACerts() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.onlySomeReasons {
            components_.push(|v_1: &ReasonFlags| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ReasonFlags(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.indirectCRL {
            if *v_ != IssuingDistPointSyntax::_default_value_for_indirectCRL() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 4;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.onlyContainsAttributeCerts {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_boolean(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 5;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificateIssuer EXTENSION ::= {
///   SYNTAX         GeneralNames
///   IDENTIFIED BY  id-ce-certificateIssuer }
/// ```
///
///
pub fn certificateIssuer() -> EXTENSION {
    EXTENSION {
        id: id_ce_certificateIssuer(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// deltaCRLIndicator EXTENSION ::= {
///   SYNTAX         BaseCRLNumber
///   IDENTIFIED BY  id-ce-deltaCRLIndicator }
/// ```
///
///
pub fn deltaCRLIndicator() -> EXTENSION {
    EXTENSION {
        id: id_ce_deltaCRLIndicator(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BaseCRLNumber  ::=  CRLNumber
/// ```
pub type BaseCRLNumber = CRLNumber; // DefinedType

pub fn _decode_BaseCRLNumber(el: &X690Element) -> ASN1Result<BaseCRLNumber> {
    _decode_CRLNumber(&el)
}

pub fn _encode_BaseCRLNumber(value_: &BaseCRLNumber) -> ASN1Result<X690Element> {
    _encode_CRLNumber(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// baseUpdateTime EXTENSION ::= {
///   SYNTAX         GeneralizedTime
///   IDENTIFIED BY  id-ce-baseUpdateTime }
/// ```
///
///
pub fn baseUpdateTime() -> EXTENSION {
    EXTENSION {
        id: id_ce_baseUpdateTime(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// freshestCRL EXTENSION ::= {
///   SYNTAX         CRLDistPointsSyntax
///   IDENTIFIED BY  id-ce-freshestCRL }
/// ```
///
///
pub fn freshestCRL() -> EXTENSION {
    EXTENSION {
        id: id_ce_freshestCRL(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// protRestrict EXTENSION ::= {
///   SYNTAX        ProtRestriction
///   IDENTIFIED BY id-ce-protRestrict }
/// ```
///
///
pub fn protRestrict() -> EXTENSION {
    EXTENSION {
        id: id_ce_protRestrict(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ProtRestriction  ::=  SEQUENCE (SIZE (1..MAX)) OF OBJECT IDENTIFIER
/// ```
pub type ProtRestriction = Vec<OBJECT_IDENTIFIER>; // SequenceOfType

pub fn _decode_ProtRestriction(el: &X690Element) -> ASN1Result<ProtRestriction> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<OBJECT_IDENTIFIER>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<OBJECT_IDENTIFIER> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(ber_decode_object_identifier(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_ProtRestriction(value_: &ProtRestriction) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(ber_encode_object_identifier(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subjectAltPublicKeyInfo EXTENSION ::= {
///   SYNTAX           SubjectAltPublicKeyInfo
///   IDENTIFIED BY    id-ce-subjectAltPublicKeyInfo }
/// ```
///
///
pub fn subjectAltPublicKeyInfo() -> EXTENSION {
    EXTENSION {
        id: id_ce_subjectAltPublicKeyInfo(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubjectAltPublicKeyInfo ::= SEQUENCE {
///   algorithm              AlgorithmIdentifier{{SupportedAlgorithms}},
///   subjectAltPublicKey    BIT STRING }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SubjectAltPublicKeyInfo {
    pub algorithm: AlgorithmIdentifier,
    pub subjectAltPublicKey: BIT_STRING,
}
impl SubjectAltPublicKeyInfo {
    pub fn new(algorithm: AlgorithmIdentifier, subjectAltPublicKey: BIT_STRING) -> Self {
        SubjectAltPublicKeyInfo {
            algorithm,
            subjectAltPublicKey,
        }
    }
}
impl TryFrom<X690Element> for SubjectAltPublicKeyInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SubjectAltPublicKeyInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SubjectAltPublicKeyInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SubjectAltPublicKeyInfo(el)
    }
}

pub const _rctl1_components_for_SubjectAltPublicKeyInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subjectAltPublicKey",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SubjectAltPublicKeyInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SubjectAltPublicKeyInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_SubjectAltPublicKeyInfo(el: &X690Element) -> ASN1Result<SubjectAltPublicKeyInfo> {
    |el_: &X690Element| -> ASN1Result<SubjectAltPublicKeyInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SubjectAltPublicKeyInfo,
            _eal_components_for_SubjectAltPublicKeyInfo,
            _rctl2_components_for_SubjectAltPublicKeyInfo,
        )?;
        let algorithm = _decode_AlgorithmIdentifier(_components.get("algorithm").unwrap())?;
        let subjectAltPublicKey =
            ber_decode_bit_string(_components.get("subjectAltPublicKey").unwrap())?;
        Ok(SubjectAltPublicKeyInfo {
            algorithm,
            subjectAltPublicKey,
        })
    }(&el)
}

pub fn _encode_SubjectAltPublicKeyInfo(
    value_: &SubjectAltPublicKeyInfo,
) -> ASN1Result<X690Element> {
    |value_: &SubjectAltPublicKeyInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_AlgorithmIdentifier(&value_.algorithm)?);
        components_.push(ber_encode_bit_string(&value_.subjectAltPublicKey)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// altSignatureAlgorithm EXTENSION ::= {
///  SYNTAX           AltSignatureAlgorithm
///  IDENTIFIED BY    id-ce-altSignatureAlgorithm }
/// ```
///
///
pub fn altSignatureAlgorithm() -> EXTENSION {
    EXTENSION {
        id: id_ce_altSignatureAlgorithm(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AltSignatureAlgorithm  ::=  AlgorithmIdentifier{{SupportedAlgorithms}}
/// ```
pub type AltSignatureAlgorithm = AlgorithmIdentifier; // DefinedType

pub fn _decode_AltSignatureAlgorithm(el: &X690Element) -> ASN1Result<AltSignatureAlgorithm> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_AltSignatureAlgorithm(value_: &AltSignatureAlgorithm) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// altSignatureValue EXTENSION ::= {
///   SYNTAX           AltSignatureValue
///   IDENTIFIED BY    id-ce-altSignatureValue }
/// ```
///
///
pub fn altSignatureValue() -> EXTENSION {
    EXTENSION {
        id: id_ce_altSignatureValue(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AltSignatureValue  ::=  BIT STRING
/// ```
pub type AltSignatureValue = BIT_STRING;

pub fn _decode_AltSignatureValue(el: &X690Element) -> ASN1Result<AltSignatureValue> {
    ber_decode_bit_string(&el)
}

pub fn _encode_AltSignatureValue(value_: &AltSignatureValue) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// aAissuingDistributionPoint EXTENSION ::= {
///   SYNTAX         AAIssuingDistPointSyntax
///   IDENTIFIED BY  id-ce-aAissuingDistributionPoint }
/// ```
///
///
pub fn aAissuingDistributionPoint() -> EXTENSION {
    EXTENSION {
        id: id_ce_aAissuingDistributionPoint(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AAIssuingDistPointSyntax ::= SEQUENCE {
///   distributionPoint           [0]  DistributionPointName OPTIONAL,
///   onlySomeReasons             [1]  ReasonFlags OPTIONAL,
///   indirectCRL                 [2]  BOOLEAN DEFAULT FALSE,
///   containsUserAttributeCerts  [3]  BOOLEAN DEFAULT TRUE,
///   containsAACerts             [4]  BOOLEAN DEFAULT TRUE,
///   containsSOAPublicKeyCerts   [5]  BOOLEAN DEFAULT TRUE,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AAIssuingDistPointSyntax {
    pub distributionPoint: OPTIONAL<DistributionPointName>,
    pub onlySomeReasons: OPTIONAL<ReasonFlags>,
    pub indirectCRL: OPTIONAL<BOOLEAN>,
    pub containsUserAttributeCerts: OPTIONAL<BOOLEAN>,
    pub containsAACerts: OPTIONAL<BOOLEAN>,
    pub containsSOAPublicKeyCerts: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl AAIssuingDistPointSyntax {
    pub fn new(
        distributionPoint: OPTIONAL<DistributionPointName>,
        onlySomeReasons: OPTIONAL<ReasonFlags>,
        indirectCRL: OPTIONAL<BOOLEAN>,
        containsUserAttributeCerts: OPTIONAL<BOOLEAN>,
        containsAACerts: OPTIONAL<BOOLEAN>,
        containsSOAPublicKeyCerts: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AAIssuingDistPointSyntax {
            distributionPoint,
            onlySomeReasons,
            indirectCRL,
            containsUserAttributeCerts,
            containsAACerts,
            containsSOAPublicKeyCerts,
            _unrecognized,
        }
    }
    pub fn _default_value_for_indirectCRL() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_containsUserAttributeCerts() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_containsAACerts() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_containsSOAPublicKeyCerts() -> BOOLEAN {
        true
    }
}
impl Default for AAIssuingDistPointSyntax {
    fn default() -> Self {
        AAIssuingDistPointSyntax {
            distributionPoint: None,
            onlySomeReasons: None,
            indirectCRL: None,
            containsUserAttributeCerts: None,
            containsAACerts: None,
            containsSOAPublicKeyCerts: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for AAIssuingDistPointSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AAIssuingDistPointSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AAIssuingDistPointSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AAIssuingDistPointSyntax(el)
    }
}

pub const _rctl1_components_for_AAIssuingDistPointSyntax: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "distributionPoint",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "onlySomeReasons",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "indirectCRL",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "containsUserAttributeCerts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "containsAACerts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "containsSOAPublicKeyCerts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AAIssuingDistPointSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AAIssuingDistPointSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_AAIssuingDistPointSyntax(el: &X690Element) -> ASN1Result<AAIssuingDistPointSyntax> {
    |el_: &X690Element| -> ASN1Result<AAIssuingDistPointSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AAIssuingDistPointSyntax,
            _eal_components_for_AAIssuingDistPointSyntax,
            _rctl2_components_for_AAIssuingDistPointSyntax,
        )?;
        let distributionPoint: OPTIONAL<DistributionPointName> =
            match _components.get("distributionPoint") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistributionPointName> {
                    Ok(_decode_DistributionPointName(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let onlySomeReasons: OPTIONAL<ReasonFlags> = match _components.get("onlySomeReasons") {
            Some(c_) => Some(_decode_ReasonFlags(c_)?),
            _ => None,
        };
        let indirectCRL: OPTIONAL<BOOLEAN> = match _components.get("indirectCRL") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let containsUserAttributeCerts: OPTIONAL<BOOLEAN> =
            match _components.get("containsUserAttributeCerts") {
                Some(c_) => Some(ber_decode_boolean(c_)?),
                _ => None,
            };
        let containsAACerts: OPTIONAL<BOOLEAN> = match _components.get("containsAACerts") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let containsSOAPublicKeyCerts: OPTIONAL<BOOLEAN> =
            match _components.get("containsSOAPublicKeyCerts") {
                Some(c_) => Some(ber_decode_boolean(c_)?),
                _ => None,
            };
        Ok(AAIssuingDistPointSyntax {
            distributionPoint,
            onlySomeReasons,
            indirectCRL,
            containsUserAttributeCerts,
            containsAACerts,
            containsSOAPublicKeyCerts,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AAIssuingDistPointSyntax(
    value_: &AAIssuingDistPointSyntax,
) -> ASN1Result<X690Element> {
    |value_: &AAIssuingDistPointSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        if let Some(v_) = &value_.distributionPoint {
            components_.push(|v_1: &DistributionPointName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_DistributionPointName(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.onlySomeReasons {
            components_.push(|v_1: &ReasonFlags| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ReasonFlags(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.indirectCRL {
            if *v_ != AAIssuingDistPointSyntax::_default_value_for_indirectCRL() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.containsUserAttributeCerts {
            if *v_ != AAIssuingDistPointSyntax::_default_value_for_containsUserAttributeCerts() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 3;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.containsAACerts {
            if *v_ != AAIssuingDistPointSyntax::_default_value_for_containsAACerts() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 4;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.containsSOAPublicKeyCerts {
            if *v_ != AAIssuingDistPointSyntax::_default_value_for_containsSOAPublicKeyCerts() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 5;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificateExactMatch MATCHING-RULE ::= {
///   SYNTAX       CertificateExactAssertion
///   LDAP-SYNTAX  certExactAssertion.&id
///   LDAP-NAME    {"certificateExactMatch"}
///   LDAP-DESC    "X.509 Certificate Exact Match"
///   ID           id-mr-certificateExactMatch }
/// ```
///
///
pub fn certificateExactMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(certExactAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("certificateExactMatch")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 Certificate Exact Match")), /* OBJECT_FIELD_SETTING */
        id: id_mr_certificateExactMatch(),                             /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateExactAssertion ::= SEQUENCE {
///   serialNumber  CertificateSerialNumber,
///   issuer        Name,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertificateExactAssertion {
    pub serialNumber: CertificateSerialNumber,
    pub issuer: Name,
    pub _unrecognized: Vec<X690Element>,
}
impl CertificateExactAssertion {
    pub fn new(
        serialNumber: CertificateSerialNumber,
        issuer: Name,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertificateExactAssertion {
            serialNumber,
            issuer,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertificateExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateExactAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificateExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateExactAssertion(el)
    }
}

pub const _rctl1_components_for_CertificateExactAssertion: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("issuer", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_CertificateExactAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertificateExactAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificateExactAssertion(
    el: &X690Element,
) -> ASN1Result<CertificateExactAssertion> {
    |el_: &X690Element| -> ASN1Result<CertificateExactAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificateExactAssertion,
            _eal_components_for_CertificateExactAssertion,
            _rctl2_components_for_CertificateExactAssertion,
        )?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        let issuer = _decode_Name(_components.get("issuer").unwrap())?;
        Ok(CertificateExactAssertion {
            serialNumber,
            issuer,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertificateExactAssertion(
    value_: &CertificateExactAssertion,
) -> ASN1Result<X690Element> {
    |value_: &CertificateExactAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
        components_.push(_encode_Name(&value_.issuer)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificateMatch MATCHING-RULE ::= {
///   SYNTAX       CertificateAssertion
///   LDAP-SYNTAX  certAssertion.&id
///   LDAP-NAME    {"certificateMatch"}
///   LDAP-DESC    "X.509 Certificate Match"
///   ID           id-mr-certificateMatch }
/// ```
///
///
pub fn certificateMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(certAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("certificateMatch")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 Certificate Match")), /* OBJECT_FIELD_SETTING */
        id: id_mr_certificateMatch(),         /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateAssertion ::= SEQUENCE {
///   serialNumber            [0]  CertificateSerialNumber OPTIONAL,
///   issuer                  [1]  Name OPTIONAL,
///   subjectKeyIdentifier    [2]  SubjectKeyIdentifier OPTIONAL,
///   authorityKeyIdentifier  [3]  AuthorityKeyIdentifier OPTIONAL,
///   certificateValid        [4]  Time OPTIONAL,
///   privateKeyValid         [5]  GeneralizedTime OPTIONAL,
///   subjectPublicKeyAlgID   [6]  OBJECT IDENTIFIER OPTIONAL,
///   keyUsage                [7]  KeyUsage OPTIONAL,
///   subjectAltName          [8]  AltNameType OPTIONAL,
///   policy                  [9]  CertPolicySet OPTIONAL,
///   pathToName              [10] Name OPTIONAL,
///   subject                 [11] Name OPTIONAL,
///   nameConstraints         [12] NameConstraintsSyntax OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertificateAssertion {
    pub serialNumber: OPTIONAL<CertificateSerialNumber>,
    pub issuer: OPTIONAL<Name>,
    pub subjectKeyIdentifier: OPTIONAL<SubjectKeyIdentifier>,
    pub authorityKeyIdentifier: OPTIONAL<AuthorityKeyIdentifier>,
    pub certificateValid: OPTIONAL<Time>,
    pub privateKeyValid: OPTIONAL<GeneralizedTime>,
    pub subjectPublicKeyAlgID: OPTIONAL<OBJECT_IDENTIFIER>,
    pub keyUsage: OPTIONAL<KeyUsage>,
    pub subjectAltName: OPTIONAL<AltNameType>,
    pub policy: OPTIONAL<CertPolicySet>,
    pub pathToName: OPTIONAL<Name>,
    pub subject: OPTIONAL<Name>,
    pub nameConstraints: OPTIONAL<NameConstraintsSyntax>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertificateAssertion {
    pub fn new(
        serialNumber: OPTIONAL<CertificateSerialNumber>,
        issuer: OPTIONAL<Name>,
        subjectKeyIdentifier: OPTIONAL<SubjectKeyIdentifier>,
        authorityKeyIdentifier: OPTIONAL<AuthorityKeyIdentifier>,
        certificateValid: OPTIONAL<Time>,
        privateKeyValid: OPTIONAL<GeneralizedTime>,
        subjectPublicKeyAlgID: OPTIONAL<OBJECT_IDENTIFIER>,
        keyUsage: OPTIONAL<KeyUsage>,
        subjectAltName: OPTIONAL<AltNameType>,
        policy: OPTIONAL<CertPolicySet>,
        pathToName: OPTIONAL<Name>,
        subject: OPTIONAL<Name>,
        nameConstraints: OPTIONAL<NameConstraintsSyntax>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertificateAssertion {
            serialNumber,
            issuer,
            subjectKeyIdentifier,
            authorityKeyIdentifier,
            certificateValid,
            privateKeyValid,
            subjectPublicKeyAlgID,
            keyUsage,
            subjectAltName,
            policy,
            pathToName,
            subject,
            nameConstraints,
            _unrecognized,
        }
    }
}
impl Default for CertificateAssertion {
    fn default() -> Self {
        CertificateAssertion {
            serialNumber: None,
            issuer: None,
            subjectKeyIdentifier: None,
            authorityKeyIdentifier: None,
            certificateValid: None,
            privateKeyValid: None,
            subjectPublicKeyAlgID: None,
            keyUsage: None,
            subjectAltName: None,
            policy: None,
            pathToName: None,
            subject: None,
            nameConstraints: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for CertificateAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificateAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateAssertion(el)
    }
}

pub const _rctl1_components_for_CertificateAssertion: &[ComponentSpec; 13] = &[
    ComponentSpec::new(
        "serialNumber",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subjectKeyIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "authorityKeyIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certificateValid",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "privateKeyValid",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subjectPublicKeyAlgID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "keyUsage",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subjectAltName",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
    ComponentSpec::new(
        "policy",
        true,
        TagSelector::tag((TagClass::CONTEXT, 9)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pathToName",
        true,
        TagSelector::tag((TagClass::CONTEXT, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subject",
        true,
        TagSelector::tag((TagClass::CONTEXT, 11)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameConstraints",
        true,
        TagSelector::tag((TagClass::CONTEXT, 12)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificateAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertificateAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificateAssertion(el: &X690Element) -> ASN1Result<CertificateAssertion> {
    |el_: &X690Element| -> ASN1Result<CertificateAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificateAssertion,
            _eal_components_for_CertificateAssertion,
            _rctl2_components_for_CertificateAssertion,
        )?;
        let serialNumber: OPTIONAL<CertificateSerialNumber> = match _components.get("serialNumber")
        {
            Some(c_) => Some(_decode_CertificateSerialNumber(c_)?),
            _ => None,
        };
        let issuer: OPTIONAL<Name> = match _components.get("issuer") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Name> {
                Ok(_decode_Name(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let subjectKeyIdentifier: OPTIONAL<SubjectKeyIdentifier> =
            match _components.get("subjectKeyIdentifier") {
                Some(c_) => Some(_decode_SubjectKeyIdentifier(c_)?),
                _ => None,
            };
        let authorityKeyIdentifier: OPTIONAL<AuthorityKeyIdentifier> =
            match _components.get("authorityKeyIdentifier") {
                Some(c_) => Some(_decode_AuthorityKeyIdentifier(c_)?),
                _ => None,
            };
        let certificateValid: OPTIONAL<Time> = match _components.get("certificateValid") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Time> {
                Ok(_decode_Time(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let privateKeyValid: OPTIONAL<GeneralizedTime> = match _components.get("privateKeyValid") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        let subjectPublicKeyAlgID: OPTIONAL<OBJECT_IDENTIFIER> =
            match _components.get("subjectPublicKeyAlgID") {
                Some(c_) => Some(ber_decode_object_identifier(c_)?),
                _ => None,
            };
        let keyUsage: OPTIONAL<KeyUsage> = match _components.get("keyUsage") {
            Some(c_) => Some(_decode_KeyUsage(c_)?),
            _ => None,
        };
        let subjectAltName: OPTIONAL<AltNameType> = match _components.get("subjectAltName") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<AltNameType> {
                Ok(_decode_AltNameType(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let policy: OPTIONAL<CertPolicySet> = match _components.get("policy") {
            Some(c_) => Some(_decode_CertPolicySet(c_)?),
            _ => None,
        };
        let pathToName: OPTIONAL<Name> = match _components.get("pathToName") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Name> {
                Ok(_decode_Name(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let subject: OPTIONAL<Name> = match _components.get("subject") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Name> {
                Ok(_decode_Name(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameConstraints: OPTIONAL<NameConstraintsSyntax> =
            match _components.get("nameConstraints") {
                Some(c_) => Some(_decode_NameConstraintsSyntax(c_)?),
                _ => None,
            };
        Ok(CertificateAssertion {
            serialNumber,
            issuer,
            subjectKeyIdentifier,
            authorityKeyIdentifier,
            certificateValid,
            privateKeyValid,
            subjectPublicKeyAlgID,
            keyUsage,
            subjectAltName,
            policy,
            pathToName,
            subject,
            nameConstraints,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertificateAssertion(value_: &CertificateAssertion) -> ASN1Result<X690Element> {
    |value_: &CertificateAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(23);
        if let Some(v_) = &value_.serialNumber {
            components_.push(|v_1: &CertificateSerialNumber| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateSerialNumber(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.issuer {
            components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.subjectKeyIdentifier {
            components_.push(|v_1: &SubjectKeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SubjectKeyIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.authorityKeyIdentifier {
            components_.push(|v_1: &AuthorityKeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AuthorityKeyIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.certificateValid {
            components_.push(|v_1: &Time| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Time(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.privateKeyValid {
            components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_generalized_time(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 5;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.subjectPublicKeyAlgID {
            components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_object_identifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 6;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.keyUsage {
            components_.push(|v_1: &KeyUsage| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_KeyUsage(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 7;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.subjectAltName {
            components_.push(|v_1: &AltNameType| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    8,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AltNameType(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.policy {
            components_.push(|v_1: &CertPolicySet| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertPolicySet(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 9;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.pathToName {
            components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    10,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.subject {
            components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    11,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameConstraints {
            components_.push(|v_1: &NameConstraintsSyntax| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_NameConstraintsSyntax(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 12;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AltNameType  ::=  CHOICE {
///   builtinNameForm  ENUMERATED {
///     rfc822Name                (1),
///     dNSName                   (2),
///     x400Address               (3),
///     directoryName             (4),
///     ediPartyName              (5),
///     uniformResourceIdentifier (6),
///     iPAddress                 (7),
///     registeredId              (8),
///     ...},
///   otherNameForm    OBJECT IDENTIFIER,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum AltNameType {
    builtinNameForm(AltNameType_builtinNameForm),
    otherNameForm(OBJECT_IDENTIFIER),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for AltNameType {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AltNameType(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AltNameType {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AltNameType(el)
    }
}

pub fn _decode_AltNameType(el: &X690Element) -> ASN1Result<AltNameType> {
    |el: &X690Element| -> ASN1Result<AltNameType> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 10) => Ok(AltNameType::builtinNameForm(
                _decode_AltNameType_builtinNameForm(&el)?,
            )),
            (TagClass::UNIVERSAL, 6) => Ok(AltNameType::otherNameForm(
                ber_decode_object_identifier(&el)?,
            )),
            _ => Ok(AltNameType::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_AltNameType(value_: &AltNameType) -> ASN1Result<X690Element> {
    |value: &AltNameType| -> ASN1Result<X690Element> {
        match value {
            AltNameType::builtinNameForm(v) => _encode_AltNameType_builtinNameForm(&v),
            AltNameType::otherNameForm(v) => ber_encode_object_identifier(&v),
            AltNameType::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertPolicySet  ::=  SEQUENCE SIZE (1..MAX) OF CertPolicyId
/// ```
pub type CertPolicySet = Vec<CertPolicyId>; // SequenceOfType

pub fn _decode_CertPolicySet(el: &X690Element) -> ASN1Result<CertPolicySet> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertPolicyId>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<CertPolicyId> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_CertPolicyId(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CertPolicySet(value_: &CertPolicySet) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<CertPolicyId>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_CertPolicyId(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificatePairExactMatch MATCHING-RULE ::= {
///   SYNTAX       CertificatePairExactAssertion
///   LDAP-SYNTAX  certPairExactAssertion.&id
///   LDAP-NAME    {"certificatePairExactMatch"}
///   LDAP-DESC    "X.509 Certificate Pair Exact Match"
///   ID           id-mr-certificatePairExactMatch }
/// ```
///
///
pub fn certificatePairExactMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(certPairExactAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("certificatePairExactMatch")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 Certificate Pair Exact Match")), /* OBJECT_FIELD_SETTING */
        id: id_mr_certificatePairExactMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificatePairExactAssertion ::= SEQUENCE {
///   issuedToThisCAAssertion  [0]  CertificateExactAssertion OPTIONAL,
///   issuedByThisCAAssertion  [1]  CertificateExactAssertion OPTIONAL,
///   ... }
///   (WITH COMPONENTS { ..., issuedToThisCAAssertion  PRESENT } |
///    WITH COMPONENTS { ..., issuedByThisCAAssertion  PRESENT } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertificatePairExactAssertion {
    pub issuedToThisCAAssertion: OPTIONAL<CertificateExactAssertion>,
    pub issuedByThisCAAssertion: OPTIONAL<CertificateExactAssertion>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertificatePairExactAssertion {
    pub fn new(
        issuedToThisCAAssertion: OPTIONAL<CertificateExactAssertion>,
        issuedByThisCAAssertion: OPTIONAL<CertificateExactAssertion>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertificatePairExactAssertion {
            issuedToThisCAAssertion,
            issuedByThisCAAssertion,
            _unrecognized,
        }
    }
}
impl Default for CertificatePairExactAssertion {
    fn default() -> Self {
        CertificatePairExactAssertion {
            issuedToThisCAAssertion: None,
            issuedByThisCAAssertion: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for CertificatePairExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificatePairExactAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificatePairExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertificatePairExactAssertion(el)
    }
}

pub const _rctl1_components_for_CertificatePairExactAssertion: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "issuedToThisCAAssertion",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuedByThisCAAssertion",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificatePairExactAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertificatePairExactAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificatePairExactAssertion(
    el: &X690Element,
) -> ASN1Result<CertificatePairExactAssertion> {
    |el_: &X690Element| -> ASN1Result<CertificatePairExactAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificatePairExactAssertion,
            _eal_components_for_CertificatePairExactAssertion,
            _rctl2_components_for_CertificatePairExactAssertion,
        )?;
        let issuedToThisCAAssertion: OPTIONAL<CertificateExactAssertion> =
            match _components.get("issuedToThisCAAssertion") {
                Some(c_) => Some(_decode_CertificateExactAssertion(c_)?),
                _ => None,
            };
        let issuedByThisCAAssertion: OPTIONAL<CertificateExactAssertion> =
            match _components.get("issuedByThisCAAssertion") {
                Some(c_) => Some(_decode_CertificateExactAssertion(c_)?),
                _ => None,
            };
        Ok(CertificatePairExactAssertion {
            issuedToThisCAAssertion,
            issuedByThisCAAssertion,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertificatePairExactAssertion(
    value_: &CertificatePairExactAssertion,
) -> ASN1Result<X690Element> {
    |value_: &CertificatePairExactAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.issuedToThisCAAssertion {
            components_.push(
                |v_1: &CertificateExactAssertion| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertificateExactAssertion(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.issuedByThisCAAssertion {
            components_.push(
                |v_1: &CertificateExactAssertion| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertificateExactAssertion(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v_)?,
            );
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificatePairMatch MATCHING-RULE ::= {
///   SYNTAX       CertificatePairAssertion
///   LDAP-SYNTAX  certPairAssertion.&id
///   LDAP-NAME    {"certificatePairMatch"}
///   LDAP-DESC    "X.509 Certificate Pair Match"
///   ID           id-mr-certificatePairMatch }
/// ```
///
///
pub fn certificatePairMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(certPairAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("certificatePairMatch")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 Certificate Pair Match")), /* OBJECT_FIELD_SETTING */
        id: id_mr_certificatePairMatch(),                             /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificatePairAssertion ::= SEQUENCE {
///   issuedToThisCAAssertion  [0]  CertificateAssertion OPTIONAL,
///   issuedByThisCAAssertion  [1]  CertificateAssertion OPTIONAL,
///   ... }
///   (WITH COMPONENTS {..., issuedToThisCAAssertion  PRESENT } |
///    WITH COMPONENTS {..., issuedByThisCAAssertion  PRESENT } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertificatePairAssertion {
    pub issuedToThisCAAssertion: OPTIONAL<CertificateAssertion>,
    pub issuedByThisCAAssertion: OPTIONAL<CertificateAssertion>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertificatePairAssertion {
    pub fn new(
        issuedToThisCAAssertion: OPTIONAL<CertificateAssertion>,
        issuedByThisCAAssertion: OPTIONAL<CertificateAssertion>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertificatePairAssertion {
            issuedToThisCAAssertion,
            issuedByThisCAAssertion,
            _unrecognized,
        }
    }
}
impl Default for CertificatePairAssertion {
    fn default() -> Self {
        CertificatePairAssertion {
            issuedToThisCAAssertion: None,
            issuedByThisCAAssertion: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for CertificatePairAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificatePairAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificatePairAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertificatePairAssertion(el)
    }
}

pub const _rctl1_components_for_CertificatePairAssertion: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "issuedToThisCAAssertion",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuedByThisCAAssertion",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificatePairAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertificatePairAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificatePairAssertion(el: &X690Element) -> ASN1Result<CertificatePairAssertion> {
    |el_: &X690Element| -> ASN1Result<CertificatePairAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificatePairAssertion,
            _eal_components_for_CertificatePairAssertion,
            _rctl2_components_for_CertificatePairAssertion,
        )?;
        let issuedToThisCAAssertion: OPTIONAL<CertificateAssertion> =
            match _components.get("issuedToThisCAAssertion") {
                Some(c_) => Some(_decode_CertificateAssertion(c_)?),
                _ => None,
            };
        let issuedByThisCAAssertion: OPTIONAL<CertificateAssertion> =
            match _components.get("issuedByThisCAAssertion") {
                Some(c_) => Some(_decode_CertificateAssertion(c_)?),
                _ => None,
            };
        Ok(CertificatePairAssertion {
            issuedToThisCAAssertion,
            issuedByThisCAAssertion,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertificatePairAssertion(
    value_: &CertificatePairAssertion,
) -> ASN1Result<X690Element> {
    |value_: &CertificatePairAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.issuedToThisCAAssertion {
            components_.push(|v_1: &CertificateAssertion| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateAssertion(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.issuedByThisCAAssertion {
            components_.push(|v_1: &CertificateAssertion| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateAssertion(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificateListExactMatch MATCHING-RULE ::= {
///   SYNTAX       CertificateListExactAssertion
///   LDAP-SYNTAX  certListExactAssertion.&id
///   LDAP-NAME    {"certificateListExactMatch"}
///   LDAP-DESC    "X.509 Certificate List Exact Match"
///   ID           id-mr-certificateListExactMatch }
/// ```
///
///
pub fn certificateListExactMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(certListExactAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("certificateListExactMatch")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 Certificate List Exact Match")), /* OBJECT_FIELD_SETTING */
        id: id_mr_certificateListExactMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateListExactAssertion ::= SEQUENCE {
///   issuer             Name,
///   thisUpdate         Time,
///   distributionPoint  DistributionPointName OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertificateListExactAssertion {
    pub issuer: Name,
    pub thisUpdate: Time,
    pub distributionPoint: OPTIONAL<DistributionPointName>,
}
impl CertificateListExactAssertion {
    pub fn new(
        issuer: Name,
        thisUpdate: Time,
        distributionPoint: OPTIONAL<DistributionPointName>,
    ) -> Self {
        CertificateListExactAssertion {
            issuer,
            thisUpdate,
            distributionPoint,
        }
    }
}
impl TryFrom<X690Element> for CertificateListExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateListExactAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificateListExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateListExactAssertion(el)
    }
}

pub const _rctl1_components_for_CertificateListExactAssertion: &[ComponentSpec; 3] = &[
    ComponentSpec::new("issuer", false, TagSelector::any, None, None),
    ComponentSpec::new("thisUpdate", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "distributionPoint",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
        ]),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificateListExactAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertificateListExactAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificateListExactAssertion(
    el: &X690Element,
) -> ASN1Result<CertificateListExactAssertion> {
    |el_: &X690Element| -> ASN1Result<CertificateListExactAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificateListExactAssertion,
            _eal_components_for_CertificateListExactAssertion,
            _rctl2_components_for_CertificateListExactAssertion,
        )?;
        let issuer = _decode_Name(_components.get("issuer").unwrap())?;
        let thisUpdate = _decode_Time(_components.get("thisUpdate").unwrap())?;
        let distributionPoint: OPTIONAL<DistributionPointName> =
            match _components.get("distributionPoint") {
                Some(c_) => Some(_decode_DistributionPointName(c_)?),
                _ => None,
            };
        Ok(CertificateListExactAssertion {
            issuer,
            thisUpdate,
            distributionPoint,
        })
    }(&el)
}

pub fn _encode_CertificateListExactAssertion(
    value_: &CertificateListExactAssertion,
) -> ASN1Result<X690Element> {
    |value_: &CertificateListExactAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_Name(&value_.issuer)?);
        components_.push(_encode_Time(&value_.thisUpdate)?);
        if let Some(v_) = &value_.distributionPoint {
            components_.push(_encode_DistributionPointName(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificateListMatch MATCHING-RULE ::= {
///   SYNTAX  CertificateListAssertion
///   LDAP-SYNTAX  certListAssertion.&id
///   LDAP-NAME    {"certificateListMatch"}
///   LDAP-DESC    "X.509 Certificate List Match"
///   ID      id-mr-certificateListMatch }
/// ```
///
///
pub fn certificateListMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(certListAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("certificateListMatch")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 Certificate List Match")), /* OBJECT_FIELD_SETTING */
        id: id_mr_certificateListMatch(),                             /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateListAssertion ::= SEQUENCE {
///   issuer                       Name                   OPTIONAL,
///   minCRLNumber            [0]  CRLNumber              OPTIONAL,
///   maxCRLNumber            [1]  CRLNumber              OPTIONAL,
///   reasonFlags                  ReasonFlags            OPTIONAL,
///   dateAndTime                  Time                   OPTIONAL,
///   distributionPoint       [2]  DistributionPointName  OPTIONAL,
///   authorityKeyIdentifier  [3]  AuthorityKeyIdentifier OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertificateListAssertion {
    pub issuer: OPTIONAL<Name>,
    pub minCRLNumber: OPTIONAL<CRLNumber>,
    pub maxCRLNumber: OPTIONAL<CRLNumber>,
    pub reasonFlags: OPTIONAL<ReasonFlags>,
    pub dateAndTime: OPTIONAL<Time>,
    pub distributionPoint: OPTIONAL<DistributionPointName>,
    pub authorityKeyIdentifier: OPTIONAL<AuthorityKeyIdentifier>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertificateListAssertion {
    pub fn new(
        issuer: OPTIONAL<Name>,
        minCRLNumber: OPTIONAL<CRLNumber>,
        maxCRLNumber: OPTIONAL<CRLNumber>,
        reasonFlags: OPTIONAL<ReasonFlags>,
        dateAndTime: OPTIONAL<Time>,
        distributionPoint: OPTIONAL<DistributionPointName>,
        authorityKeyIdentifier: OPTIONAL<AuthorityKeyIdentifier>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertificateListAssertion {
            issuer,
            minCRLNumber,
            maxCRLNumber,
            reasonFlags,
            dateAndTime,
            distributionPoint,
            authorityKeyIdentifier,
            _unrecognized,
        }
    }
}
impl Default for CertificateListAssertion {
    fn default() -> Self {
        CertificateListAssertion {
            issuer: None,
            minCRLNumber: None,
            maxCRLNumber: None,
            reasonFlags: None,
            dateAndTime: None,
            distributionPoint: None,
            authorityKeyIdentifier: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for CertificateListAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateListAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificateListAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateListAssertion(el)
    }
}

pub const _rctl1_components_for_CertificateListAssertion: &[ComponentSpec; 7] = &[
    ComponentSpec::new(
        "issuer",
        true,
        TagSelector::or(&[&TagSelector::tag((TagClass::UNIVERSAL, 16))]),
        None,
        None,
    ),
    ComponentSpec::new(
        "minCRLNumber",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "maxCRLNumber",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "reasonFlags",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "dateAndTime",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 23)),
            &TagSelector::tag((TagClass::UNIVERSAL, 24)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "distributionPoint",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "authorityKeyIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificateListAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertificateListAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificateListAssertion(el: &X690Element) -> ASN1Result<CertificateListAssertion> {
    |el_: &X690Element| -> ASN1Result<CertificateListAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificateListAssertion,
            _eal_components_for_CertificateListAssertion,
            _rctl2_components_for_CertificateListAssertion,
        )?;
        let issuer: OPTIONAL<Name> = match _components.get("issuer") {
            Some(c_) => Some(_decode_Name(c_)?),
            _ => None,
        };
        let minCRLNumber: OPTIONAL<CRLNumber> = match _components.get("minCRLNumber") {
            Some(c_) => Some(_decode_CRLNumber(c_)?),
            _ => None,
        };
        let maxCRLNumber: OPTIONAL<CRLNumber> = match _components.get("maxCRLNumber") {
            Some(c_) => Some(_decode_CRLNumber(c_)?),
            _ => None,
        };
        let reasonFlags: OPTIONAL<ReasonFlags> = match _components.get("reasonFlags") {
            Some(c_) => Some(_decode_ReasonFlags(c_)?),
            _ => None,
        };
        let dateAndTime: OPTIONAL<Time> = match _components.get("dateAndTime") {
            Some(c_) => Some(_decode_Time(c_)?),
            _ => None,
        };
        let distributionPoint: OPTIONAL<DistributionPointName> =
            match _components.get("distributionPoint") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<DistributionPointName> {
                    Ok(_decode_DistributionPointName(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let authorityKeyIdentifier: OPTIONAL<AuthorityKeyIdentifier> =
            match _components.get("authorityKeyIdentifier") {
                Some(c_) => Some(_decode_AuthorityKeyIdentifier(c_)?),
                _ => None,
            };
        Ok(CertificateListAssertion {
            issuer,
            minCRLNumber,
            maxCRLNumber,
            reasonFlags,
            dateAndTime,
            distributionPoint,
            authorityKeyIdentifier,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertificateListAssertion(
    value_: &CertificateListAssertion,
) -> ASN1Result<X690Element> {
    |value_: &CertificateListAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        if let Some(v_) = &value_.issuer {
            components_.push(_encode_Name(&v_)?);
        }
        if let Some(v_) = &value_.minCRLNumber {
            components_.push(|v_1: &CRLNumber| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CRLNumber(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.maxCRLNumber {
            components_.push(|v_1: &CRLNumber| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CRLNumber(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.reasonFlags {
            components_.push(_encode_ReasonFlags(&v_)?);
        }
        if let Some(v_) = &value_.dateAndTime {
            components_.push(_encode_Time(&v_)?);
        }
        if let Some(v_) = &value_.distributionPoint {
            components_.push(|v_1: &DistributionPointName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_DistributionPointName(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.authorityKeyIdentifier {
            components_.push(|v_1: &AuthorityKeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AuthorityKeyIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// algorithmIdentifierMatch MATCHING-RULE ::= {
///   SYNTAX       AlgorithmIdentifier {{SupportedAlgorithms}}
///   LDAP-SYNTAX  algorithmIdentifier.&id
///   LDAP-NAME    {"algorithmIdentifierMatch"}
///   LDAP-DESC    "X.509 Algorithm Identifier Match"
///   ID           id-mr-algorithmIdentifierMatch }
/// ```
///
///
pub fn algorithmIdentifierMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(algorithmIdentifier().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("algorithmIdentifierMatch")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 Algorithm Identifier Match")), /* OBJECT_FIELD_SETTING */
        id: id_mr_algorithmIdentifierMatch(),                             /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// policyMatch MATCHING-RULE ::= {
///   SYNTAX  PolicyID
///   ID      id-mr-policyMatch }
/// ```
///
///
pub fn policyMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_policyMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkiPathMatch MATCHING-RULE ::= {
///   SYNTAX  PkiPathMatchSyntax
///   ID      id-mr-pkiPathMatch }
/// ```
///
///
pub fn pkiPathMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_pkiPathMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PkiPathMatchSyntax ::= SEQUENCE {
///   firstIssuer  Name,
///   lastSubject  Name,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PkiPathMatchSyntax {
    pub firstIssuer: Name,
    pub lastSubject: Name,
    pub _unrecognized: Vec<X690Element>,
}
impl PkiPathMatchSyntax {
    pub fn new(firstIssuer: Name, lastSubject: Name, _unrecognized: Vec<X690Element>) -> Self {
        PkiPathMatchSyntax {
            firstIssuer,
            lastSubject,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for PkiPathMatchSyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PkiPathMatchSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PkiPathMatchSyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PkiPathMatchSyntax(el)
    }
}

pub const _rctl1_components_for_PkiPathMatchSyntax: &[ComponentSpec; 2] = &[
    ComponentSpec::new("firstIssuer", false, TagSelector::any, None, None),
    ComponentSpec::new("lastSubject", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_PkiPathMatchSyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PkiPathMatchSyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_PkiPathMatchSyntax(el: &X690Element) -> ASN1Result<PkiPathMatchSyntax> {
    |el_: &X690Element| -> ASN1Result<PkiPathMatchSyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PkiPathMatchSyntax,
            _eal_components_for_PkiPathMatchSyntax,
            _rctl2_components_for_PkiPathMatchSyntax,
        )?;
        let firstIssuer = _decode_Name(_components.get("firstIssuer").unwrap())?;
        let lastSubject = _decode_Name(_components.get("lastSubject").unwrap())?;
        Ok(PkiPathMatchSyntax {
            firstIssuer,
            lastSubject,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PkiPathMatchSyntax(value_: &PkiPathMatchSyntax) -> ASN1Result<X690Element> {
    |value_: &PkiPathMatchSyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Name(&value_.firstIssuer)?);
        components_.push(_encode_Name(&value_.lastSubject)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// enhancedCertificateMatch MATCHING-RULE ::= {
///   SYNTAX  EnhancedCertificateAssertion
///   ID      id-mr-enhancedCertificateMatch }
/// ```
///
///
pub fn enhancedCertificateMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_enhancedCertificateMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EnhancedCertificateAssertion ::= SEQUENCE {
///   serialNumber            [0]  CertificateSerialNumber OPTIONAL,
///   issuer                  [1]  Name OPTIONAL,
///   subjectKeyIdentifier    [2]  SubjectKeyIdentifier OPTIONAL,
///   authorityKeyIdentifier  [3]  AuthorityKeyIdentifier OPTIONAL,
///   certificateValid        [4]  Time OPTIONAL,
///   privateKeyValid         [5]  GeneralizedTime OPTIONAL,
///   subjectPublicKeyAlgID   [6]  OBJECT IDENTIFIER OPTIONAL,
///   keyUsage                [7]  KeyUsage OPTIONAL,
///   subjectAltName          [8]  AltName OPTIONAL,
///   policy                  [9]  CertPolicySet OPTIONAL,
///   pathToName              [10] GeneralNames OPTIONAL,
///   subject                 [11] Name OPTIONAL,
///   nameConstraints         [12] NameConstraintsSyntax OPTIONAL,
///   ... }
///   (ALL EXCEPT ({ -- none; at least one component shall be present --}))
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EnhancedCertificateAssertion {
    pub serialNumber: OPTIONAL<CertificateSerialNumber>,
    pub issuer: OPTIONAL<Name>,
    pub subjectKeyIdentifier: OPTIONAL<SubjectKeyIdentifier>,
    pub authorityKeyIdentifier: OPTIONAL<AuthorityKeyIdentifier>,
    pub certificateValid: OPTIONAL<Time>,
    pub privateKeyValid: OPTIONAL<GeneralizedTime>,
    pub subjectPublicKeyAlgID: OPTIONAL<OBJECT_IDENTIFIER>,
    pub keyUsage: OPTIONAL<KeyUsage>,
    pub subjectAltName: OPTIONAL<AltName>,
    pub policy: OPTIONAL<CertPolicySet>,
    pub pathToName: OPTIONAL<GeneralNames>,
    pub subject: OPTIONAL<Name>,
    pub nameConstraints: OPTIONAL<NameConstraintsSyntax>,
    pub _unrecognized: Vec<X690Element>,
}
impl EnhancedCertificateAssertion {
    pub fn new(
        serialNumber: OPTIONAL<CertificateSerialNumber>,
        issuer: OPTIONAL<Name>,
        subjectKeyIdentifier: OPTIONAL<SubjectKeyIdentifier>,
        authorityKeyIdentifier: OPTIONAL<AuthorityKeyIdentifier>,
        certificateValid: OPTIONAL<Time>,
        privateKeyValid: OPTIONAL<GeneralizedTime>,
        subjectPublicKeyAlgID: OPTIONAL<OBJECT_IDENTIFIER>,
        keyUsage: OPTIONAL<KeyUsage>,
        subjectAltName: OPTIONAL<AltName>,
        policy: OPTIONAL<CertPolicySet>,
        pathToName: OPTIONAL<GeneralNames>,
        subject: OPTIONAL<Name>,
        nameConstraints: OPTIONAL<NameConstraintsSyntax>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        EnhancedCertificateAssertion {
            serialNumber,
            issuer,
            subjectKeyIdentifier,
            authorityKeyIdentifier,
            certificateValid,
            privateKeyValid,
            subjectPublicKeyAlgID,
            keyUsage,
            subjectAltName,
            policy,
            pathToName,
            subject,
            nameConstraints,
            _unrecognized,
        }
    }
}
impl Default for EnhancedCertificateAssertion {
    fn default() -> Self {
        EnhancedCertificateAssertion {
            serialNumber: None,
            issuer: None,
            subjectKeyIdentifier: None,
            authorityKeyIdentifier: None,
            certificateValid: None,
            privateKeyValid: None,
            subjectPublicKeyAlgID: None,
            keyUsage: None,
            subjectAltName: None,
            policy: None,
            pathToName: None,
            subject: None,
            nameConstraints: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for EnhancedCertificateAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EnhancedCertificateAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EnhancedCertificateAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EnhancedCertificateAssertion(el)
    }
}

pub const _rctl1_components_for_EnhancedCertificateAssertion: &[ComponentSpec; 13] = &[
    ComponentSpec::new(
        "serialNumber",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuer",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subjectKeyIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "authorityKeyIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certificateValid",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "privateKeyValid",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subjectPublicKeyAlgID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "keyUsage",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subjectAltName",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
    ComponentSpec::new(
        "policy",
        true,
        TagSelector::tag((TagClass::CONTEXT, 9)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pathToName",
        true,
        TagSelector::tag((TagClass::CONTEXT, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subject",
        true,
        TagSelector::tag((TagClass::CONTEXT, 11)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameConstraints",
        true,
        TagSelector::tag((TagClass::CONTEXT, 12)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EnhancedCertificateAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EnhancedCertificateAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_EnhancedCertificateAssertion(
    el: &X690Element,
) -> ASN1Result<EnhancedCertificateAssertion> {
    |el_: &X690Element| -> ASN1Result<EnhancedCertificateAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EnhancedCertificateAssertion,
            _eal_components_for_EnhancedCertificateAssertion,
            _rctl2_components_for_EnhancedCertificateAssertion,
        )?;
        let serialNumber: OPTIONAL<CertificateSerialNumber> = match _components.get("serialNumber")
        {
            Some(c_) => Some(_decode_CertificateSerialNumber(c_)?),
            _ => None,
        };
        let issuer: OPTIONAL<Name> = match _components.get("issuer") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Name> {
                Ok(_decode_Name(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let subjectKeyIdentifier: OPTIONAL<SubjectKeyIdentifier> =
            match _components.get("subjectKeyIdentifier") {
                Some(c_) => Some(_decode_SubjectKeyIdentifier(c_)?),
                _ => None,
            };
        let authorityKeyIdentifier: OPTIONAL<AuthorityKeyIdentifier> =
            match _components.get("authorityKeyIdentifier") {
                Some(c_) => Some(_decode_AuthorityKeyIdentifier(c_)?),
                _ => None,
            };
        let certificateValid: OPTIONAL<Time> = match _components.get("certificateValid") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Time> {
                Ok(_decode_Time(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let privateKeyValid: OPTIONAL<GeneralizedTime> = match _components.get("privateKeyValid") {
            Some(c_) => Some(ber_decode_generalized_time(c_)?),
            _ => None,
        };
        let subjectPublicKeyAlgID: OPTIONAL<OBJECT_IDENTIFIER> =
            match _components.get("subjectPublicKeyAlgID") {
                Some(c_) => Some(ber_decode_object_identifier(c_)?),
                _ => None,
            };
        let keyUsage: OPTIONAL<KeyUsage> = match _components.get("keyUsage") {
            Some(c_) => Some(_decode_KeyUsage(c_)?),
            _ => None,
        };
        let subjectAltName: OPTIONAL<AltName> = match _components.get("subjectAltName") {
            Some(c_) => Some(_decode_AltName(c_)?),
            _ => None,
        };
        let policy: OPTIONAL<CertPolicySet> = match _components.get("policy") {
            Some(c_) => Some(_decode_CertPolicySet(c_)?),
            _ => None,
        };
        let pathToName: OPTIONAL<GeneralNames> = match _components.get("pathToName") {
            Some(c_) => Some(_decode_GeneralNames(c_)?),
            _ => None,
        };
        let subject: OPTIONAL<Name> = match _components.get("subject") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Name> {
                Ok(_decode_Name(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nameConstraints: OPTIONAL<NameConstraintsSyntax> =
            match _components.get("nameConstraints") {
                Some(c_) => Some(_decode_NameConstraintsSyntax(c_)?),
                _ => None,
            };
        Ok(EnhancedCertificateAssertion {
            serialNumber,
            issuer,
            subjectKeyIdentifier,
            authorityKeyIdentifier,
            certificateValid,
            privateKeyValid,
            subjectPublicKeyAlgID,
            keyUsage,
            subjectAltName,
            policy,
            pathToName,
            subject,
            nameConstraints,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_EnhancedCertificateAssertion(
    value_: &EnhancedCertificateAssertion,
) -> ASN1Result<X690Element> {
    |value_: &EnhancedCertificateAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(23);
        if let Some(v_) = &value_.serialNumber {
            components_.push(|v_1: &CertificateSerialNumber| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateSerialNumber(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.issuer {
            components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.subjectKeyIdentifier {
            components_.push(|v_1: &SubjectKeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SubjectKeyIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.authorityKeyIdentifier {
            components_.push(|v_1: &AuthorityKeyIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AuthorityKeyIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.certificateValid {
            components_.push(|v_1: &Time| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Time(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.privateKeyValid {
            components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_generalized_time(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 5;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.subjectPublicKeyAlgID {
            components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_object_identifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 6;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.keyUsage {
            components_.push(|v_1: &KeyUsage| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_KeyUsage(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 7;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.subjectAltName {
            components_.push(|v_1: &AltName| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AltName(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 8;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.policy {
            components_.push(|v_1: &CertPolicySet| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertPolicySet(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 9;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.pathToName {
            components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_GeneralNames(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 10;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.subject {
            components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    11,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.nameConstraints {
            components_.push(|v_1: &NameConstraintsSyntax| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_NameConstraintsSyntax(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 12;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AltName ::= SEQUENCE {
///   altnameType   AltNameType,
///   altNameValue  GeneralName OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AltName {
    pub altnameType: AltNameType,
    pub altNameValue: OPTIONAL<GeneralName>,
}
impl AltName {
    pub fn new(altnameType: AltNameType, altNameValue: OPTIONAL<GeneralName>) -> Self {
        AltName {
            altnameType,
            altNameValue,
        }
    }
}
impl TryFrom<X690Element> for AltName {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AltName(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AltName {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AltName(el)
    }
}

pub const _rctl1_components_for_AltName: &[ComponentSpec; 2] = &[
    ComponentSpec::new("altnameType", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "altNameValue",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
            &TagSelector::tag((TagClass::CONTEXT, 2)),
            &TagSelector::tag((TagClass::CONTEXT, 3)),
            &TagSelector::tag((TagClass::CONTEXT, 4)),
            &TagSelector::tag((TagClass::CONTEXT, 5)),
            &TagSelector::tag((TagClass::CONTEXT, 6)),
            &TagSelector::tag((TagClass::CONTEXT, 7)),
            &TagSelector::tag((TagClass::CONTEXT, 8)),
        ]),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AltName: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AltName: &[ComponentSpec; 0] = &[];

pub fn _decode_AltName(el: &X690Element) -> ASN1Result<AltName> {
    |el_: &X690Element| -> ASN1Result<AltName> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AltName,
            _eal_components_for_AltName,
            _rctl2_components_for_AltName,
        )?;
        let altnameType = _decode_AltNameType(_components.get("altnameType").unwrap())?;
        let altNameValue: OPTIONAL<GeneralName> = match _components.get("altNameValue") {
            Some(c_) => Some(_decode_GeneralName(c_)?),
            _ => None,
        };
        Ok(AltName {
            altnameType,
            altNameValue,
        })
    }(&el)
}

pub fn _encode_AltName(value_: &AltName) -> ASN1Result<X690Element> {
    |value_: &AltName| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_AltNameType(&value_.altnameType)?);
        if let Some(v_) = &value_.altNameValue {
            components_.push(_encode_GeneralName(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certExactAssertion SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 Certificate Exact Assertion"
///   DIRECTORY SYNTAX  CertificateExactAssertion
///   ID                id-ldx-certExactAssertion }
/// ```
///
///
pub fn certExactAssertion() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 Certificate Exact Assertion"), /* OBJECT_FIELD_SETTING */
        id: id_ldx_certExactAssertion(),                             /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certAssertion SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 Certificate Assertion"
///   DIRECTORY SYNTAX  CertificateAssertion
///   ID                id-ldx-certAssertion }
/// ```
///
///
pub fn certAssertion() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 Certificate Assertion"), /* OBJECT_FIELD_SETTING */
        id: id_ldx_certAssertion(),                            /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certPairExactAssertion SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 Certificate Pair Exact Assertion"
///   DIRECTORY SYNTAX  CertificatePairExactAssertion
///   ID                id-ldx-certPairExactAssertion }
/// ```
///
///
pub fn certPairExactAssertion() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 Certificate Pair Exact Assertion"), /* OBJECT_FIELD_SETTING */
        id: id_ldx_certPairExactAssertion(),                              /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certPairAssertion SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 Certificate Pair Assertion"
///   DIRECTORY SYNTAX  CertificatePairAssertion
///   ID                id-ldx-certPairAssertion }
/// ```
///
///
pub fn certPairAssertion() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 Certificate Pair Assertion"), /* OBJECT_FIELD_SETTING */
        id: id_ldx_certPairAssertion(),                             /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certListExactAssertion SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 Certificate List Exact Assertion"
///   DIRECTORY SYNTAX  CertificateListExactAssertion
///   ID                id-ldx-certListExactAssertion }
/// ```
///
///
pub fn certListExactAssertion() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 Certificate List Exact Assertion"), /* OBJECT_FIELD_SETTING */
        id: id_ldx_certListExactAssertion(),                              /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certListAssertion SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 Certificate List Assertion"
///   DIRECTORY SYNTAX  CertificateListAssertion
///   ID                id-ldx-certListAssertion }
/// ```
///
///
pub fn certListAssertion() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 Certificate List Assertion"), /* OBJECT_FIELD_SETTING */
        id: id_ldx_certListAssertion(),                             /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// algorithmIdentifier SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 Algorithm Identifier"
///   DIRECTORY SYNTAX  AlgorithmIdentifier{{SupportedAlgorithms}}
///   ID                id-ldx-algorithmIdentifier }
/// ```
///
///
pub fn algorithmIdentifier() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 Algorithm Identifier"), /* OBJECT_FIELD_SETTING */
        id: id_ldx_algorithmIdentifier(),                     /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-subjectDirectoryAttributes         OBJECT IDENTIFIER ::= {id-ce 9}
/// ```
///
///
pub fn id_ce_subjectDirectoryAttributes() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-subjectKeyIdentifier               OBJECT IDENTIFIER ::= {id-ce 14}
/// ```
///
///
pub fn id_ce_subjectKeyIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([14])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-keyUsage                           OBJECT IDENTIFIER ::= {id-ce 15}
/// ```
///
///
pub fn id_ce_keyUsage() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([15])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-privateKeyUsagePeriod              OBJECT IDENTIFIER ::= {id-ce 16}
/// ```
///
///
pub fn id_ce_privateKeyUsagePeriod() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([16])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-subjectAltName                     OBJECT IDENTIFIER ::= {id-ce 17}
/// ```
///
///
pub fn id_ce_subjectAltName() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([17])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-issuerAltName                      OBJECT IDENTIFIER ::= {id-ce 18}
/// ```
///
///
pub fn id_ce_issuerAltName() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([18])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-basicConstraints                   OBJECT IDENTIFIER ::= {id-ce 19}
/// ```
///
///
pub fn id_ce_basicConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([19])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-cRLNumber                          OBJECT IDENTIFIER ::= {id-ce 20}
/// ```
///
///
pub fn id_ce_cRLNumber() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([20])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-reasonCode                         OBJECT IDENTIFIER ::= {id-ce 21}
/// ```
///
///
pub fn id_ce_reasonCode() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([21])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-holdInstructionCode                OBJECT IDENTIFIER ::= {id-ce 23}
/// ```
///
///
pub fn id_ce_holdInstructionCode() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([23])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-invalidityDate                     OBJECT IDENTIFIER ::= {id-ce 24}
/// ```
///
///
pub fn id_ce_invalidityDate() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([24])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-deltaCRLIndicator                  OBJECT IDENTIFIER ::= {id-ce 27}
/// ```
///
///
pub fn id_ce_deltaCRLIndicator() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([27])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-issuingDistributionPoint           OBJECT IDENTIFIER ::= {id-ce 28}
/// ```
///
///
pub fn id_ce_issuingDistributionPoint() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([28])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-certificateIssuer                  OBJECT IDENTIFIER ::= {id-ce 29}
/// ```
///
///
pub fn id_ce_certificateIssuer() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([29])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-nameConstraints                    OBJECT IDENTIFIER ::= {id-ce 30}
/// ```
///
///
pub fn id_ce_nameConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([30])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-cRLDistributionPoints              OBJECT IDENTIFIER ::= {id-ce 31}
/// ```
///
///
pub fn id_ce_cRLDistributionPoints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([31])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-certificatePolicies                OBJECT IDENTIFIER ::= {id-ce 32}
/// ```
///
///
pub fn id_ce_certificatePolicies() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([32])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-policyMappings                     OBJECT IDENTIFIER ::= {id-ce 33}
/// ```
///
///
pub fn id_ce_policyMappings() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([33])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-authorityKeyIdentifier             OBJECT IDENTIFIER ::= {id-ce 35}
/// ```
///
///
pub fn id_ce_authorityKeyIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([35])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-policyConstraints                  OBJECT IDENTIFIER ::= {id-ce 36}
/// ```
///
///
pub fn id_ce_policyConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([36])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-extKeyUsage                        OBJECT IDENTIFIER ::= {id-ce 37}
/// ```
///
///
pub fn id_ce_extKeyUsage() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([37])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-cRLStreamIdentifier                OBJECT IDENTIFIER ::= {id-ce 40}
/// ```
///
///
pub fn id_ce_cRLStreamIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([40])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-cRLScope                           OBJECT IDENTIFIER ::= {id-ce 44}
/// ```
///
///
pub fn id_ce_cRLScope() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([44])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-statusReferrals                    OBJECT IDENTIFIER ::= {id-ce 45}
/// ```
///
///
pub fn id_ce_statusReferrals() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([45])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-freshestCRL                        OBJECT IDENTIFIER ::= {id-ce 46}
/// ```
///
///
pub fn id_ce_freshestCRL() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([46])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-orderedList                        OBJECT IDENTIFIER ::= {id-ce 47}
/// ```
///
///
pub fn id_ce_orderedList() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([47])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-baseUpdateTime                     OBJECT IDENTIFIER ::= {id-ce 51}
/// ```
///
///
pub fn id_ce_baseUpdateTime() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([51])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-deltaInfo                          OBJECT IDENTIFIER ::= {id-ce 53}
/// ```
///
///
pub fn id_ce_deltaInfo() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([53])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-inhibitAnyPolicy                   OBJECT IDENTIFIER ::= {id-ce 54}
/// ```
///
///
pub fn id_ce_inhibitAnyPolicy() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([54])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-toBeRevoked                        OBJECT IDENTIFIER ::= {id-ce 58}
/// ```
///
///
pub fn id_ce_toBeRevoked() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([58])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-revokedGroups                      OBJECT IDENTIFIER ::= {id-ce 59}
/// ```
///
///
pub fn id_ce_revokedGroups() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([59])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-expiredCertsOnCRL                  OBJECT IDENTIFIER ::= {id-ce 60}
/// ```
///
///
pub fn id_ce_expiredCertsOnCRL() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([60])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-aAissuingDistributionPoint         OBJECT IDENTIFIER ::= {id-ce 63}
/// ```
///
///
pub fn id_ce_aAissuingDistributionPoint() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([63])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-authorizationValidation            OBJECT IDENTIFIER ::= {id-ce 70}
/// ```
///
///
pub fn id_ce_authorizationValidation() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([70])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-protRestrict                       OBJECT IDENTIFIER ::= {id-ce 71}
/// ```
///
///
pub fn id_ce_protRestrict() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([71])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-subjectAltPublicKeyInfo            OBJECT IDENTIFIER ::= {id-ce 72}
/// ```
///
///
pub fn id_ce_subjectAltPublicKeyInfo() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([72])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-altSignatureAlgorithm              OBJECT IDENTIFIER ::= {id-ce 73}
/// ```
///
///
pub fn id_ce_altSignatureAlgorithm() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([73])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-altSignatureValue                  OBJECT IDENTIFIER ::= {id-ce 74}
/// ```
///
///
pub fn id_ce_altSignatureValue() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([74])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-associatedInformation              OBJECT IDENTIFIER ::= {id-ce 75}
/// ```
///
///
pub fn id_ce_associatedInformation() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([75])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificateExactMatch       OBJECT IDENTIFIER ::= {id-mr 34}
/// ```
///
///
pub fn id_mr_certificateExactMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([34])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificateMatch            OBJECT IDENTIFIER ::= {id-mr 35}
/// ```
///
///
pub fn id_mr_certificateMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([35])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificatePairExactMatch   OBJECT IDENTIFIER ::= {id-mr 36}
/// ```
///
///
pub fn id_mr_certificatePairExactMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([36])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificatePairMatch        OBJECT IDENTIFIER ::= {id-mr 37}
/// ```
///
///
pub fn id_mr_certificatePairMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([37])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificateListExactMatch   OBJECT IDENTIFIER ::= {id-mr 38}
/// ```
///
///
pub fn id_mr_certificateListExactMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([38])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificateListMatch        OBJECT IDENTIFIER ::= {id-mr 39}
/// ```
///
///
pub fn id_mr_certificateListMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([39])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-algorithmIdentifierMatch    OBJECT IDENTIFIER ::= {id-mr 40}
/// ```
///
///
pub fn id_mr_algorithmIdentifierMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([40])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-policyMatch                 OBJECT IDENTIFIER ::= {id-mr 60}
/// ```
///
///
pub fn id_mr_policyMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([60])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-pkiPathMatch                OBJECT IDENTIFIER ::= {id-mr 62}
/// ```
///
///
pub fn id_mr_pkiPathMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([62])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-enhancedCertificateMatch    OBJECT IDENTIFIER ::= {id-mr 65}
/// ```
///
///
pub fn id_mr_enhancedCertificateMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([65])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certExactAssertion         OBJECT IDENTIFIER ::= {id-ldx 1}
/// ```
///
///
pub fn id_ldx_certExactAssertion() -> OBJECT_IDENTIFIER {
    [id_ldx(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certAssertion              OBJECT IDENTIFIER ::= {id-ldx 2}
/// ```
///
///
pub fn id_ldx_certAssertion() -> OBJECT_IDENTIFIER {
    [id_ldx(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certPairExactAssertion     OBJECT IDENTIFIER ::= {id-ldx 3}
/// ```
///
///
pub fn id_ldx_certPairExactAssertion() -> OBJECT_IDENTIFIER {
    [id_ldx(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certPairAssertion          OBJECT IDENTIFIER ::= {id-ldx 4}
/// ```
///
///
pub fn id_ldx_certPairAssertion() -> OBJECT_IDENTIFIER {
    [id_ldx(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certListExactAssertion     OBJECT IDENTIFIER ::= {id-ldx 5}
/// ```
///
///
pub fn id_ldx_certListExactAssertion() -> OBJECT_IDENTIFIER {
    [id_ldx(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certListAssertion          OBJECT IDENTIFIER ::= {id-ldx 6}
/// ```
///
///
pub fn id_ldx_certListAssertion() -> OBJECT_IDENTIFIER {
    [id_ldx(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-algorithmIdentifier        OBJECT IDENTIFIER ::= {id-ldx 7}
/// ```
///
///
pub fn id_ldx_algorithmIdentifier() -> OBJECT_IDENTIFIER {
    [id_ldx(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PolicyMappingsSyntax-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PolicyMappingsSyntax_Item {
    pub issuerDomainPolicy: CertPolicyId,
    pub subjectDomainPolicy: CertPolicyId,
    pub _unrecognized: Vec<X690Element>,
}
impl PolicyMappingsSyntax_Item {
    pub fn new(
        issuerDomainPolicy: CertPolicyId,
        subjectDomainPolicy: CertPolicyId,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PolicyMappingsSyntax_Item {
            issuerDomainPolicy,
            subjectDomainPolicy,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for PolicyMappingsSyntax_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PolicyMappingsSyntax_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PolicyMappingsSyntax_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PolicyMappingsSyntax_Item(el)
    }
}

pub const _rctl1_components_for_PolicyMappingsSyntax_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "issuerDomainPolicy",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subjectDomainPolicy",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PolicyMappingsSyntax_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PolicyMappingsSyntax_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_PolicyMappingsSyntax_Item(
    el: &X690Element,
) -> ASN1Result<PolicyMappingsSyntax_Item> {
    |el_: &X690Element| -> ASN1Result<PolicyMappingsSyntax_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PolicyMappingsSyntax_Item,
            _eal_components_for_PolicyMappingsSyntax_Item,
            _rctl2_components_for_PolicyMappingsSyntax_Item,
        )?;
        let issuerDomainPolicy =
            _decode_CertPolicyId(_components.get("issuerDomainPolicy").unwrap())?;
        let subjectDomainPolicy =
            _decode_CertPolicyId(_components.get("subjectDomainPolicy").unwrap())?;
        Ok(PolicyMappingsSyntax_Item {
            issuerDomainPolicy,
            subjectDomainPolicy,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PolicyMappingsSyntax_Item(
    value_: &PolicyMappingsSyntax_Item,
) -> ASN1Result<X690Element> {
    |value_: &PolicyMappingsSyntax_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_CertPolicyId(&value_.issuerDomainPolicy)?);
        components_.push(_encode_CertPolicyId(&value_.subjectDomainPolicy)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AltNameType-builtinNameForm ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type AltNameType_builtinNameForm = ENUMERATED;

pub const AltNameType_builtinNameForm_rfc822Name: AltNameType_builtinNameForm = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AltNameType_builtinNameForm_dNSName: AltNameType_builtinNameForm = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AltNameType_builtinNameForm_x400Address: AltNameType_builtinNameForm = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AltNameType_builtinNameForm_directoryName: AltNameType_builtinNameForm = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AltNameType_builtinNameForm_ediPartyName: AltNameType_builtinNameForm = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AltNameType_builtinNameForm_uniformResourceIdentifier: AltNameType_builtinNameForm = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AltNameType_builtinNameForm_iPAddress: AltNameType_builtinNameForm = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AltNameType_builtinNameForm_registeredId: AltNameType_builtinNameForm = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_AltNameType_builtinNameForm(
    el: &X690Element,
) -> ASN1Result<AltNameType_builtinNameForm> {
    ber_decode_enumerated(&el)
}

pub fn _encode_AltNameType_builtinNameForm(
    value_: &AltNameType_builtinNameForm,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}
