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
use wildboar_asn1::*;
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

pub mod authorityKeyIdentifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AuthorityKeyIdentifier; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AuthorityKeyIdentifier(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AuthorityKeyIdentifier(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AuthorityKeyIdentifier(el)
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
impl TryFrom<&X690Element> for AuthorityKeyIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AuthorityKeyIdentifier",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AuthorityKeyIdentifier,
        _eal_components_for_AuthorityKeyIdentifier,
        _rctl2_components_for_AuthorityKeyIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut keyIdentifier_: OPTIONAL<KeyIdentifier> = None;
    let mut authorityCertIssuer_: OPTIONAL<GeneralNames> = None;
    let mut authorityCertSerialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "keyIdentifier" => keyIdentifier_ = Some(_decode_KeyIdentifier(_el)?),
            "authorityCertIssuer" => authorityCertIssuer_ = Some(_decode_GeneralNames(_el)?),
            "authorityCertSerialNumber" => {
                authorityCertSerialNumber_ = Some(_decode_CertificateSerialNumber(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AuthorityKeyIdentifier {
        keyIdentifier: keyIdentifier_,
        authorityCertIssuer: authorityCertIssuer_,
        authorityCertSerialNumber: authorityCertSerialNumber_,
        _unrecognized,
    })
}

pub fn _encode_AuthorityKeyIdentifier(value_: &AuthorityKeyIdentifier) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.keyIdentifier {
        components_.push(|v_1: &KeyIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_KeyIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.authorityCertIssuer {
        components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralNames(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.authorityCertSerialNumber {
        components_.push(|v_1: &CertificateSerialNumber| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertificateSerialNumber(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AuthorityKeyIdentifier(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AuthorityKeyIdentifier",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AuthorityKeyIdentifier,
        _eal_components_for_AuthorityKeyIdentifier,
        _rctl2_components_for_AuthorityKeyIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "keyIdentifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "keyIdentifier")
                    );
                }
                Ok(_validate_KeyIdentifier(&el)?)
            }(_el)?,
            "authorityCertIssuer" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "authorityCertIssuer",
                    ));
                }
                Ok(_validate_GeneralNames(&el)?)
            }(_el)?,
            "authorityCertSerialNumber" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "authorityCertSerialNumber",
                    ));
                }
                Ok(_validate_CertificateSerialNumber(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyIdentifier  ::=  OCTET STRING
/// ```
pub type KeyIdentifier = OCTET_STRING; // OctetStringType

pub fn _decode_KeyIdentifier(el: &X690Element) -> ASN1Result<KeyIdentifier> {
    BER.decode_octet_string(&el)
}

pub fn _encode_KeyIdentifier(value_: &KeyIdentifier) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_KeyIdentifier(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
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

pub mod subjectKeyIdentifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = SubjectKeyIdentifier; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_SubjectKeyIdentifier(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_SubjectKeyIdentifier(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_SubjectKeyIdentifier(el)
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

pub fn _validate_SubjectKeyIdentifier(el: &X690Element) -> ASN1Result<()> {
    _validate_KeyIdentifier(&el)
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

pub mod keyUsage {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = KeyUsage; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_KeyUsage(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_KeyUsage(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_KeyUsage(el)
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

// This is defined in IETF RFC 5280.
pub const KeyUsage_nonRepudiation: BIT = KeyUsage_contentCommitment; /* LONG_NAMED_BIT */

pub const KeyUsage_keyEncipherment: BIT = 2; /* LONG_NAMED_BIT */

pub const KeyUsage_dataEncipherment: BIT = 3; /* LONG_NAMED_BIT */

pub const KeyUsage_keyAgreement: BIT = 4; /* LONG_NAMED_BIT */

pub const KeyUsage_keyCertSign: BIT = 5; /* LONG_NAMED_BIT */

pub const KeyUsage_cRLSign: BIT = 6; /* LONG_NAMED_BIT */

pub const KeyUsage_encipherOnly: BIT = 7; /* LONG_NAMED_BIT */

pub const KeyUsage_decipherOnly: BIT = 8; /* LONG_NAMED_BIT */

pub fn _decode_KeyUsage(el: &X690Element) -> ASN1Result<KeyUsage> {
    BER.decode_bit_string(&el)
}

pub fn _encode_KeyUsage(value_: &KeyUsage) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_KeyUsage(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
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

pub mod extKeyUsage {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = Vec<KeyPurposeId>; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        let elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "&ExtnType")),
        };
        let mut items: SEQUENCE_OF<KeyPurposeId> = Vec::with_capacity(elements.len());
        for el in elements.iter() {
            items.push(_decode_KeyPurposeId(el)?);
        }
        Ok(items)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_KeyPurposeId(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    _validate_KeyPurposeId(&sub)?;
                }
                Ok(())
            }
            _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "&ExtnType")),
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// KeyPurposeId  ::=  OBJECT IDENTIFIER
/// ```
pub type KeyPurposeId = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_KeyPurposeId(el: &X690Element) -> ASN1Result<KeyPurposeId> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_KeyPurposeId(value_: &KeyPurposeId) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_KeyPurposeId(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
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

pub mod privateKeyUsagePeriod {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = PrivateKeyUsagePeriod; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_PrivateKeyUsagePeriod(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_PrivateKeyUsagePeriod(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_PrivateKeyUsagePeriod(el)
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
impl TryFrom<&X690Element> for PrivateKeyUsagePeriod {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PrivateKeyUsagePeriod")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PrivateKeyUsagePeriod,
        _eal_components_for_PrivateKeyUsagePeriod,
        _rctl2_components_for_PrivateKeyUsagePeriod,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut notBefore_: OPTIONAL<GeneralizedTime> = None;
    let mut notAfter_: OPTIONAL<GeneralizedTime> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notBefore" => notBefore_ = Some(BER.decode_generalized_time(_el)?),
            "notAfter" => notAfter_ = Some(BER.decode_generalized_time(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PrivateKeyUsagePeriod {
        notBefore: notBefore_,
        notAfter: notAfter_,
        _unrecognized,
    })
}

pub fn _encode_PrivateKeyUsagePeriod(value_: &PrivateKeyUsagePeriod) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.notBefore {
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_generalized_time(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.notAfter {
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_generalized_time(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_PrivateKeyUsagePeriod(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PrivateKeyUsagePeriod")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PrivateKeyUsagePeriod,
        _eal_components_for_PrivateKeyUsagePeriod,
        _rctl2_components_for_PrivateKeyUsagePeriod,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notBefore" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "notBefore")
                    );
                }
                Ok(BER.validate_generalized_time(&el)?)
            }(_el)?,
            "notAfter" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "notAfter")
                    );
                }
                Ok(BER.validate_generalized_time(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod certificatePolicies {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = CertificatePoliciesSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_CertificatePoliciesSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_CertificatePoliciesSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificatePoliciesSyntax(el)
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificatePoliciesSyntax",
            ))
        }
    };
    let mut items: SEQUENCE_OF<PolicyInformation> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_PolicyInformation(el)?);
    }
    Ok(items)
}

pub fn _encode_CertificatePoliciesSyntax(
    value_: &CertificatePoliciesSyntax,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_PolicyInformation(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CertificatePoliciesSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_PolicyInformation(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "CertificatePoliciesSyntax",
        )),
    }
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
impl TryFrom<&X690Element> for PolicyInformation {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PolicyInformation")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PolicyInformation,
        _eal_components_for_PolicyInformation,
        _rctl2_components_for_PolicyInformation,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut policyIdentifier_: OPTIONAL<CertPolicyId> = None;
    let mut policyQualifiers_: OPTIONAL<Vec<PolicyQualifierInfo>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "policyIdentifier" => policyIdentifier_ = Some(_decode_CertPolicyId(_el)?),
            "policyQualifiers" => {
                policyQualifiers_ = Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<PolicyQualifierInfo>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "policyQualifiers",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<PolicyQualifierInfo> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_PolicyQualifierInfo(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PolicyInformation {
        policyIdentifier: policyIdentifier_.unwrap(),
        policyQualifiers: policyQualifiers_,
        _unrecognized,
    })
}

pub fn _encode_PolicyInformation(value_: &PolicyInformation) -> ASN1Result<X690Element> {
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
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_)?,
        );
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_PolicyInformation(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PolicyInformation")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PolicyInformation,
        _eal_components_for_PolicyInformation,
        _rctl2_components_for_PolicyInformation,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "policyIdentifier" => _validate_CertPolicyId(_el)?,
            "policyQualifiers" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_PolicyQualifierInfo(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "policyQualifiers",
                    )),
                }
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertPolicyId  ::=  OBJECT IDENTIFIER
/// ```
pub type CertPolicyId = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_CertPolicyId(el: &X690Element) -> ASN1Result<CertPolicyId> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_CertPolicyId(value_: &CertPolicyId) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_CertPolicyId(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
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
impl TryFrom<&X690Element> for PolicyQualifierInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PolicyQualifierInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PolicyQualifierInfo,
        _eal_components_for_PolicyQualifierInfo,
        _rctl2_components_for_PolicyQualifierInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut policyQualifierId_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut qualifier_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "policyQualifierId" => policyQualifierId_ = Some(BER.decode_object_identifier(_el)?),
            "qualifier" => qualifier_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PolicyQualifierInfo {
        policyQualifierId: policyQualifierId_.unwrap(),
        qualifier: qualifier_,
        _unrecognized,
    })
}

pub fn _encode_PolicyQualifierInfo(value_: &PolicyQualifierInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.policyQualifierId)?);
    if let Some(v_) = &value_.qualifier {
        components_.push(x690_identity(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_PolicyQualifierInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PolicyQualifierInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PolicyQualifierInfo,
        _eal_components_for_PolicyQualifierInfo,
        _rctl2_components_for_PolicyQualifierInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "policyQualifierId" => BER.validate_object_identifier(_el)?,
            "qualifier" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedPolicyQualifiers CERT-POLICY-QUALIFIER ::= {...}
/// ```
///
///
pub fn SupportedPolicyQualifiers() -> Vec<CERT_POLICY_QUALIFIER> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// anyPolicy OBJECT IDENTIFIER ::= {id-ce-certificatePolicies 0}
/// ```
///
///
#[inline]
pub fn anyPolicy () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce_certificatePolicies(), 0).unwrap() // OID_GETTER
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

pub mod policyMappings {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = PolicyMappingsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_PolicyMappingsSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_PolicyMappingsSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_PolicyMappingsSyntax(el)
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PolicyMappingsSyntax")
            )
        }
    };
    let mut items: SEQUENCE_OF<PolicyMappingsSyntax_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_PolicyMappingsSyntax_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_PolicyMappingsSyntax(value_: &PolicyMappingsSyntax) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_PolicyMappingsSyntax_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_PolicyMappingsSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_PolicyMappingsSyntax_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PolicyMappingsSyntax")),
    }
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

pub mod authorizationValidation {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AvlId; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AvlId(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AvlId(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AvlId(el)
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
impl TryFrom<&X690Element> for AvlId {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AvlId")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AvlId,
        _eal_components_for_AvlId,
        _rctl2_components_for_AvlId,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut serialNumber_: OPTIONAL<AvlSerialNumber> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => issuer_ = Some(_decode_Name(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_AvlSerialNumber(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AvlId {
        issuer: issuer_.unwrap(),
        serialNumber: serialNumber_,
        _unrecognized,
    })
}

pub fn _encode_AvlId(value_: &AvlId) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_Name(&value_.issuer)?);
    if let Some(v_) = &value_.serialNumber {
        components_.push(_encode_AvlSerialNumber(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AvlId(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AvlId")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AvlId,
        _eal_components_for_AvlId,
        _rctl2_components_for_AvlId,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => _validate_Name(_el)?,
            "serialNumber" => _validate_AvlSerialNumber(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod subjectAltName {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = GeneralNames; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_GeneralNames(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_GeneralNames(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_GeneralNames(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// GeneralNames  ::=  SEQUENCE SIZE (1..MAX) OF GeneralName
/// ```
pub type GeneralNames = Vec<GeneralName>; // SequenceOfType

pub fn _decode_GeneralNames(el: &X690Element) -> ASN1Result<GeneralNames> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "GeneralNames")),
    };
    let mut items: SEQUENCE_OF<GeneralName> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_GeneralName(el)?);
    }
    Ok(items)
}

pub fn _encode_GeneralNames(value_: &GeneralNames) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_GeneralName(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_GeneralNames(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_GeneralName(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "GeneralNames")),
    }
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

impl TryFrom<&X690Element> for GeneralName {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_GeneralName(el)
    }
}

pub fn _decode_GeneralName(el: &X690Element) -> ASN1Result<GeneralName> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(GeneralName::otherName(BER.decode_instance_of(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(GeneralName::rfc822Name(BER.decode_ia5_string(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(GeneralName::dNSName(BER.decode_ia5_string(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(GeneralName::x400Address(_decode_ORAddress(&el)?)),
        (TagClass::CONTEXT, 4) => Ok(GeneralName::directoryName(
            |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(&el)?,
        )),
        (TagClass::CONTEXT, 5) => Ok(GeneralName::ediPartyName(_decode_EDIPartyName(&el)?)),
        (TagClass::CONTEXT, 6) => Ok(GeneralName::uniformResourceIdentifier(
            BER.decode_ia5_string(&el)?,
        )),
        (TagClass::CONTEXT, 7) => Ok(GeneralName::iPAddress(BER.decode_octet_string(&el)?)),
        (TagClass::CONTEXT, 8) => Ok(GeneralName::registeredID(
            BER.decode_object_identifier(&el)?,
        )),
        _ => Ok(GeneralName::_unrecognized(el.clone())),
    }
}

pub fn _encode_GeneralName(value_: &GeneralName) -> ASN1Result<X690Element> {
    match value_ {
        GeneralName::otherName(v) => |v_1: &INSTANCE_OF| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_instance_of(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        GeneralName::rfc822Name(v) => |v_1: &IA5String| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_ia5_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
        GeneralName::dNSName(v) => |v_1: &IA5String| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_ia5_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v),
        GeneralName::x400Address(v) => |v_1: &ORAddress| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ORAddress(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v),
        GeneralName::directoryName(v) => |v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::from_explicit(&_encode_Name(&v_1)?),
            ))
        }(&v),
        GeneralName::ediPartyName(v) => |v_1: &EDIPartyName| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_EDIPartyName(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 5;
            Ok(el_1)
        }(&v),
        GeneralName::uniformResourceIdentifier(v) => |v_1: &IA5String| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_ia5_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 6;
            Ok(el_1)
        }(&v),
        GeneralName::iPAddress(v) => |v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_octet_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 7;
            Ok(el_1)
        }(&v),
        GeneralName::registeredID(v) => |v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_object_identifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 8;
            Ok(el_1)
        }(&v),
        GeneralName::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_GeneralName(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "otherName"));
            }
            Ok(BER.validate_external(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "rfc822Name"));
            }
            Ok(BER.validate_ia5_string(&el)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "dNSName"));
            }
            Ok(BER.validate_ia5_string(&el)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "x400Address")
                );
            }
            Ok(_validate_ORAddress(&el)?)
        }(&el),
        (TagClass::CONTEXT, 4) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "directoryName")
                );
            }
            Ok(_validate_Name(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 5) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ediPartyName")
                );
            }
            Ok(_validate_EDIPartyName(&el)?)
        }(&el),
        (TagClass::CONTEXT, 6) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "uniformResourceIdentifier",
                ));
            }
            Ok(BER.validate_ia5_string(&el)?)
        }(&el),
        (TagClass::CONTEXT, 7) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "iPAddress"));
            }
            Ok(BER.validate_octet_string(&el)?)
        }(&el),
        (TagClass::CONTEXT, 8) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 8 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "registeredID")
                );
            }
            Ok(BER.validate_object_identifier(&el)?)
        }(&el),
        _ => Ok(()),
    }
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
impl TryFrom<&X690Element> for EDIPartyName {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EDIPartyName")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EDIPartyName,
        _eal_components_for_EDIPartyName,
        _rctl2_components_for_EDIPartyName,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut nameAssigner_: OPTIONAL<UnboundedDirectoryString> = None;
    let mut partyName_: OPTIONAL<UnboundedDirectoryString> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "nameAssigner" => {
                nameAssigner_ = Some(|el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
                    Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
                }(_el)?)
            }
            "partyName" => {
                partyName_ = Some(|el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
                    Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(EDIPartyName {
        nameAssigner: nameAssigner_,
        partyName: partyName_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_EDIPartyName(value_: &EDIPartyName) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.nameAssigner {
        components_.push(
            |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&_encode_UnboundedDirectoryString(&v_1)?),
                ))
            }(&v_)?,
        );
    }
    components_.push(
        |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_UnboundedDirectoryString(&v_1)?),
            ))
        }(&value_.partyName)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_EDIPartyName(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EDIPartyName")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EDIPartyName,
        _eal_components_for_EDIPartyName,
        _rctl2_components_for_EDIPartyName,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "nameAssigner" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "nameAssigner")
                    );
                }
                Ok(_validate_UnboundedDirectoryString(&el.inner()?)?)
            }(_el)?,
            "partyName" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "partyName")
                    );
                }
                Ok(_validate_UnboundedDirectoryString(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod issuerAltName {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = GeneralNames; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_GeneralNames(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_GeneralNames(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_GeneralNames(el)
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

pub mod subjectDirectoryAttributes {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AttributesSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AttributesSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AttributesSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributesSyntax(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributesSyntax  ::=  SEQUENCE SIZE (1..MAX) OF Attribute{{SupportedAttributes}}
/// ```
pub type AttributesSyntax = Vec<Attribute>; // SequenceOfType

pub fn _decode_AttributesSyntax(el: &X690Element) -> ASN1Result<AttributesSyntax> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributesSyntax")
            )
        }
    };
    let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Attribute(el)?);
    }
    Ok(items)
}

pub fn _encode_AttributesSyntax(value_: &AttributesSyntax) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Attribute(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_AttributesSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Attribute(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributesSyntax")),
    }
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

pub mod associatedInformation {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AttributesSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AttributesSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AttributesSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributesSyntax(el)
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

pub mod basicConstraints {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = BasicConstraintsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_BasicConstraintsSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_BasicConstraintsSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_BasicConstraintsSyntax(el)
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
impl TryFrom<&X690Element> for BasicConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "BasicConstraintsSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_BasicConstraintsSyntax,
        _eal_components_for_BasicConstraintsSyntax,
        _rctl2_components_for_BasicConstraintsSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut cA_: OPTIONAL<BOOLEAN> = None;
    let mut pathLenConstraint_: OPTIONAL<INTEGER> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "cA" => cA_ = Some(BER.decode_boolean(_el)?),
            "pathLenConstraint" => pathLenConstraint_ = Some(BER.decode_integer(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(BasicConstraintsSyntax {
        cA: cA_,
        pathLenConstraint: pathLenConstraint_,
        _unrecognized,
    })
}

pub fn _encode_BasicConstraintsSyntax(value_: &BasicConstraintsSyntax) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.cA {
        if *v_ != BasicConstraintsSyntax::_default_value_for_cA() {
            components_.push(BER.encode_boolean(&v_)?);
        }
    }
    if let Some(v_) = &value_.pathLenConstraint {
        components_.push(BER.encode_integer(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_BasicConstraintsSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "BasicConstraintsSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_BasicConstraintsSyntax,
        _eal_components_for_BasicConstraintsSyntax,
        _rctl2_components_for_BasicConstraintsSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "cA" => BER.validate_boolean(_el)?,
            "pathLenConstraint" => BER.validate_integer(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod nameConstraints {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = NameConstraintsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_NameConstraintsSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_NameConstraintsSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_NameConstraintsSyntax(el)
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
impl TryFrom<&X690Element> for NameConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NameConstraintsSyntax")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_NameConstraintsSyntax,
        _eal_components_for_NameConstraintsSyntax,
        _rctl2_components_for_NameConstraintsSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut permittedSubtrees_: OPTIONAL<GeneralSubtrees> = None;
    let mut excludedSubtrees_: OPTIONAL<GeneralSubtrees> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "permittedSubtrees" => permittedSubtrees_ = Some(_decode_GeneralSubtrees(_el)?),
            "excludedSubtrees" => excludedSubtrees_ = Some(_decode_GeneralSubtrees(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(NameConstraintsSyntax {
        permittedSubtrees: permittedSubtrees_,
        excludedSubtrees: excludedSubtrees_,
        _unrecognized,
    })
}

pub fn _encode_NameConstraintsSyntax(value_: &NameConstraintsSyntax) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.permittedSubtrees {
        components_.push(|v_1: &GeneralSubtrees| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralSubtrees(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.excludedSubtrees {
        components_.push(|v_1: &GeneralSubtrees| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralSubtrees(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_NameConstraintsSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NameConstraintsSyntax")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_NameConstraintsSyntax,
        _eal_components_for_NameConstraintsSyntax,
        _rctl2_components_for_NameConstraintsSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "permittedSubtrees" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "permittedSubtrees",
                    ));
                }
                Ok(_validate_GeneralSubtrees(&el)?)
            }(_el)?,
            "excludedSubtrees" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "excludedSubtrees",
                    ));
                }
                Ok(_validate_GeneralSubtrees(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// GeneralSubtrees  ::=  SEQUENCE SIZE (1..MAX) OF GeneralSubtree
/// ```
pub type GeneralSubtrees = Vec<GeneralSubtree>; // SequenceOfType

pub fn _decode_GeneralSubtrees(el: &X690Element) -> ASN1Result<GeneralSubtrees> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "GeneralSubtrees"))
        }
    };
    let mut items: SEQUENCE_OF<GeneralSubtree> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_GeneralSubtree(el)?);
    }
    Ok(items)
}

pub fn _encode_GeneralSubtrees(value_: &GeneralSubtrees) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_GeneralSubtree(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_GeneralSubtrees(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_GeneralSubtree(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "GeneralSubtrees")),
    }
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
        Vec::from([0]) // TODO: Make fixed int
    }
}
impl TryFrom<&X690Element> for GeneralSubtree {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "GeneralSubtree"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_GeneralSubtree,
        _eal_components_for_GeneralSubtree,
        _rctl2_components_for_GeneralSubtree,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut base_: OPTIONAL<GeneralName> = None;
    let mut minimum_: OPTIONAL<BaseDistance> = None;
    let mut maximum_: OPTIONAL<BaseDistance> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "base" => base_ = Some(_decode_GeneralName(_el)?),
            "minimum" => minimum_ = Some(_decode_BaseDistance(_el)?),
            "maximum" => maximum_ = Some(_decode_BaseDistance(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(GeneralSubtree {
        base: base_.unwrap(),
        minimum: minimum_,
        maximum: maximum_,
        _unrecognized,
    })
}

pub fn _encode_GeneralSubtree(value_: &GeneralSubtree) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_GeneralName(&value_.base)?);
    if let Some(v_) = &value_.minimum {
        if *v_ != GeneralSubtree::_default_value_for_minimum() {
            components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_BaseDistance(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.maximum {
        components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_BaseDistance(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_GeneralSubtree(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "GeneralSubtree"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_GeneralSubtree,
        _eal_components_for_GeneralSubtree,
        _rctl2_components_for_GeneralSubtree,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "base" => _validate_GeneralName(_el)?,
            "minimum" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "minimum")
                    );
                }
                Ok(_validate_BaseDistance(&el)?)
            }(_el)?,
            "maximum" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "maximum")
                    );
                }
                Ok(_validate_BaseDistance(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BaseDistance  ::=  INTEGER(0..MAX)
/// ```
pub type BaseDistance = INTEGER;

pub fn _decode_BaseDistance(el: &X690Element) -> ASN1Result<BaseDistance> {
    BER.decode_integer(&el)
}

pub fn _encode_BaseDistance(value_: &BaseDistance) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_BaseDistance(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
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

pub mod policyConstraints {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = PolicyConstraintsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_PolicyConstraintsSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_PolicyConstraintsSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_PolicyConstraintsSyntax(el)
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
impl TryFrom<&X690Element> for PolicyConstraintsSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "PolicyConstraintsSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PolicyConstraintsSyntax,
        _eal_components_for_PolicyConstraintsSyntax,
        _rctl2_components_for_PolicyConstraintsSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut requireExplicitPolicy_: OPTIONAL<SkipCerts> = None;
    let mut inhibitPolicyMapping_: OPTIONAL<SkipCerts> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "requireExplicitPolicy" => requireExplicitPolicy_ = Some(_decode_SkipCerts(_el)?),
            "inhibitPolicyMapping" => inhibitPolicyMapping_ = Some(_decode_SkipCerts(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PolicyConstraintsSyntax {
        requireExplicitPolicy: requireExplicitPolicy_,
        inhibitPolicyMapping: inhibitPolicyMapping_,
        _unrecognized,
    })
}

pub fn _encode_PolicyConstraintsSyntax(
    value_: &PolicyConstraintsSyntax,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.requireExplicitPolicy {
        components_.push(|v_1: &SkipCerts| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SkipCerts(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.inhibitPolicyMapping {
        components_.push(|v_1: &SkipCerts| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SkipCerts(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_PolicyConstraintsSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "PolicyConstraintsSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PolicyConstraintsSyntax,
        _eal_components_for_PolicyConstraintsSyntax,
        _rctl2_components_for_PolicyConstraintsSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "requireExplicitPolicy" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "requireExplicitPolicy",
                    ));
                }
                Ok(_validate_SkipCerts(&el)?)
            }(_el)?,
            "inhibitPolicyMapping" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "inhibitPolicyMapping",
                    ));
                }
                Ok(_validate_SkipCerts(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SkipCerts  ::=  INTEGER(0..MAX)
/// ```
pub type SkipCerts = INTEGER;

pub fn _decode_SkipCerts(el: &X690Element) -> ASN1Result<SkipCerts> {
    BER.decode_integer(&el)
}

pub fn _encode_SkipCerts(value_: &SkipCerts) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_SkipCerts(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
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

pub mod inhibitAnyPolicy {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = SkipCerts; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_SkipCerts(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_SkipCerts(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_SkipCerts(el)
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

pub mod cRLNumber {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = CRLNumber; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_CRLNumber(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_CRLNumber(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_CRLNumber(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CRLNumber  ::=  INTEGER(0..MAX)
/// ```
pub type CRLNumber = INTEGER;

pub fn _decode_CRLNumber(el: &X690Element) -> ASN1Result<CRLNumber> {
    BER.decode_integer(&el)
}

pub fn _encode_CRLNumber(value_: &CRLNumber) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_CRLNumber(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
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

pub mod crlScope {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = CRLScopeSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_CRLScopeSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_CRLScopeSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_CRLScopeSyntax(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CRLScopeSyntax  ::=  SEQUENCE SIZE (1..MAX) OF PerAuthorityScope
/// ```
pub type CRLScopeSyntax = Vec<PerAuthorityScope>; // SequenceOfType

pub fn _decode_CRLScopeSyntax(el: &X690Element) -> ASN1Result<CRLScopeSyntax> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CRLScopeSyntax"))
        }
    };
    let mut items: SEQUENCE_OF<PerAuthorityScope> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_PerAuthorityScope(el)?);
    }
    Ok(items)
}

pub fn _encode_CRLScopeSyntax(value_: &CRLScopeSyntax) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_PerAuthorityScope(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CRLScopeSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_PerAuthorityScope(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CRLScopeSyntax")),
    }
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
impl TryFrom<&X690Element> for PerAuthorityScope {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PerAuthorityScope")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PerAuthorityScope,
        _eal_components_for_PerAuthorityScope,
        _rctl2_components_for_PerAuthorityScope,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut authorityName_: OPTIONAL<GeneralName> = None;
    let mut distributionPoint_: OPTIONAL<DistributionPointName> = None;
    let mut onlyContains_: OPTIONAL<OnlyCertificateTypes> = None;
    let mut onlySomeReasons_: OPTIONAL<ReasonFlags> = None;
    let mut serialNumberRange_: OPTIONAL<NumberRange> = None;
    let mut subjectKeyIdRange_: OPTIONAL<NumberRange> = None;
    let mut nameSubtrees_: OPTIONAL<GeneralNames> = None;
    let mut baseRevocationInfo_: OPTIONAL<BaseRevocationInfo> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "authorityName" => {
                authorityName_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "distributionPoint" => {
                distributionPoint_ = Some(|el: &X690Element| -> ASN1Result<DistributionPointName> {
                    Ok(_decode_DistributionPointName(&el.inner()?)?)
                }(_el)?)
            }
            "onlyContains" => onlyContains_ = Some(_decode_OnlyCertificateTypes(_el)?),
            "onlySomeReasons" => onlySomeReasons_ = Some(_decode_ReasonFlags(_el)?),
            "serialNumberRange" => serialNumberRange_ = Some(_decode_NumberRange(_el)?),
            "subjectKeyIdRange" => subjectKeyIdRange_ = Some(_decode_NumberRange(_el)?),
            "nameSubtrees" => nameSubtrees_ = Some(_decode_GeneralNames(_el)?),
            "baseRevocationInfo" => baseRevocationInfo_ = Some(_decode_BaseRevocationInfo(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PerAuthorityScope {
        authorityName: authorityName_,
        distributionPoint: distributionPoint_,
        onlyContains: onlyContains_,
        onlySomeReasons: onlySomeReasons_,
        serialNumberRange: serialNumberRange_,
        subjectKeyIdRange: subjectKeyIdRange_,
        nameSubtrees: nameSubtrees_,
        baseRevocationInfo: baseRevocationInfo_,
        _unrecognized,
    })
}

pub fn _encode_PerAuthorityScope(value_: &PerAuthorityScope) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(18);
    if let Some(v_) = &value_.authorityName {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.distributionPoint {
        components_.push(|v_1: &DistributionPointName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_DistributionPointName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.onlyContains {
        components_.push(|v_1: &OnlyCertificateTypes| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_OnlyCertificateTypes(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.onlySomeReasons {
        components_.push(|v_1: &ReasonFlags| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ReasonFlags(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 4;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.serialNumberRange {
        components_.push(|v_1: &NumberRange| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_NumberRange(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 5;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.subjectKeyIdRange {
        components_.push(|v_1: &NumberRange| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_NumberRange(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 6;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.nameSubtrees {
        components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralNames(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 7;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.baseRevocationInfo {
        components_.push(|v_1: &BaseRevocationInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_BaseRevocationInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 9;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_PerAuthorityScope(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PerAuthorityScope")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PerAuthorityScope,
        _eal_components_for_PerAuthorityScope,
        _rctl2_components_for_PerAuthorityScope,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "authorityName" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "authorityName")
                    );
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "distributionPoint" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "distributionPoint",
                    ));
                }
                Ok(_validate_DistributionPointName(&el.inner()?)?)
            }(_el)?,
            "onlyContains" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "onlyContains")
                    );
                }
                Ok(_validate_OnlyCertificateTypes(&el)?)
            }(_el)?,
            "onlySomeReasons" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "onlySomeReasons",
                    ));
                }
                Ok(_validate_ReasonFlags(&el)?)
            }(_el)?,
            "serialNumberRange" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "serialNumberRange",
                    ));
                }
                Ok(_validate_NumberRange(&el)?)
            }(_el)?,
            "subjectKeyIdRange" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "subjectKeyIdRange",
                    ));
                }
                Ok(_validate_NumberRange(&el)?)
            }(_el)?,
            "nameSubtrees" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "nameSubtrees")
                    );
                }
                Ok(_validate_GeneralNames(&el)?)
            }(_el)?,
            "baseRevocationInfo" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 9 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "baseRevocationInfo",
                    ));
                }
                Ok(_validate_BaseRevocationInfo(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    BER.decode_bit_string(&el)
}

pub fn _encode_OnlyCertificateTypes(value_: &OnlyCertificateTypes) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_OnlyCertificateTypes(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
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
impl TryFrom<&X690Element> for NumberRange {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NumberRange")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_NumberRange,
        _eal_components_for_NumberRange,
        _rctl2_components_for_NumberRange,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut startingNumber_: OPTIONAL<INTEGER> = None;
    let mut endingNumber_: OPTIONAL<INTEGER> = None;
    let mut modulus_: OPTIONAL<INTEGER> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "startingNumber" => startingNumber_ = Some(BER.decode_integer(_el)?),
            "endingNumber" => endingNumber_ = Some(BER.decode_integer(_el)?),
            "modulus" => modulus_ = Some(BER.decode_integer(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(NumberRange {
        startingNumber: startingNumber_,
        endingNumber: endingNumber_,
        modulus: modulus_,
        _unrecognized,
    })
}

pub fn _encode_NumberRange(value_: &NumberRange) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.startingNumber {
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_integer(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.endingNumber {
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_integer(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.modulus {
        components_.push(BER.encode_integer(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_NumberRange(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NumberRange")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_NumberRange,
        _eal_components_for_NumberRange,
        _rctl2_components_for_NumberRange,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "startingNumber" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "startingNumber")
                    );
                }
                Ok(BER.validate_integer(&el)?)
            }(_el)?,
            "endingNumber" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "endingNumber")
                    );
                }
                Ok(BER.validate_integer(&el)?)
            }(_el)?,
            "modulus" => BER.validate_integer(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for BaseRevocationInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BaseRevocationInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_BaseRevocationInfo,
        _eal_components_for_BaseRevocationInfo,
        _rctl2_components_for_BaseRevocationInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut cRLStreamIdentifier_: OPTIONAL<CRLStreamIdentifier> = None;
    let mut cRLNumber_: OPTIONAL<CRLNumber> = None;
    let mut baseThisUpdate_: OPTIONAL<GeneralizedTime> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "cRLStreamIdentifier" => cRLStreamIdentifier_ = Some(_decode_CRLStreamIdentifier(_el)?),
            "cRLNumber" => cRLNumber_ = Some(_decode_CRLNumber(_el)?),
            "baseThisUpdate" => baseThisUpdate_ = Some(BER.decode_generalized_time(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(BaseRevocationInfo {
        cRLStreamIdentifier: cRLStreamIdentifier_,
        cRLNumber: cRLNumber_.unwrap(),
        baseThisUpdate: baseThisUpdate_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_BaseRevocationInfo(value_: &BaseRevocationInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.cRLStreamIdentifier {
        components_.push(|v_1: &CRLStreamIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CRLStreamIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(|v_1: &CRLNumber| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_CRLNumber(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 1;
        Ok(el_1)
    }(&value_.cRLNumber)?);
    components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
        let mut el_1 = BER.encode_generalized_time(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 2;
        Ok(el_1)
    }(&value_.baseThisUpdate)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_BaseRevocationInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BaseRevocationInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_BaseRevocationInfo,
        _eal_components_for_BaseRevocationInfo,
        _rctl2_components_for_BaseRevocationInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "cRLStreamIdentifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "cRLStreamIdentifier",
                    ));
                }
                Ok(_validate_CRLStreamIdentifier(&el)?)
            }(_el)?,
            "cRLNumber" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "cRLNumber")
                    );
                }
                Ok(_validate_CRLNumber(&el)?)
            }(_el)?,
            "baseThisUpdate" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "baseThisUpdate")
                    );
                }
                Ok(BER.validate_generalized_time(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod statusReferrals {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = StatusReferrals; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_StatusReferrals(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_StatusReferrals(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_StatusReferrals(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// StatusReferrals  ::=  SEQUENCE SIZE (1..MAX) OF StatusReferral
/// ```
pub type StatusReferrals = Vec<StatusReferral>; // SequenceOfType

pub fn _decode_StatusReferrals(el: &X690Element) -> ASN1Result<StatusReferrals> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "StatusReferrals"))
        }
    };
    let mut items: SEQUENCE_OF<StatusReferral> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_StatusReferral(el)?);
    }
    Ok(items)
}

pub fn _encode_StatusReferrals(value_: &StatusReferrals) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_StatusReferral(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_StatusReferrals(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_StatusReferral(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "StatusReferrals")),
    }
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

impl TryFrom<&X690Element> for StatusReferral {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_StatusReferral(el)
    }
}

pub fn _decode_StatusReferral(el: &X690Element) -> ASN1Result<StatusReferral> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(StatusReferral::cRLReferral(_decode_CRLReferral(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(StatusReferral::otherReferral(BER.decode_instance_of(&el)?)),
        _ => Ok(StatusReferral::_unrecognized(el.clone())),
    }
}

pub fn _encode_StatusReferral(value_: &StatusReferral) -> ASN1Result<X690Element> {
    match value_ {
        StatusReferral::cRLReferral(v) => |v_1: &CRLReferral| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CRLReferral(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        StatusReferral::otherReferral(v) => |v_1: &INSTANCE_OF| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_instance_of(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
        StatusReferral::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_StatusReferral(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "cRLReferral")
                );
            }
            Ok(_validate_CRLReferral(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "otherReferral")
                );
            }
            Ok(BER.validate_external(&el)?)
        }(&el),
        _ => Ok(()),
    }
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
impl TryFrom<&X690Element> for CRLReferral {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CRLReferral")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CRLReferral,
        _eal_components_for_CRLReferral,
        _rctl2_components_for_CRLReferral,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuer_: OPTIONAL<GeneralName> = None;
    let mut location_: OPTIONAL<GeneralName> = None;
    let mut deltaRefInfo_: OPTIONAL<DeltaRefInfo> = None;
    let mut cRLScope_: OPTIONAL<CRLScopeSyntax> = None;
    let mut lastUpdate_: OPTIONAL<GeneralizedTime> = None;
    let mut lastChangedCRL_: OPTIONAL<GeneralizedTime> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => {
                issuer_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "location" => {
                location_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "deltaRefInfo" => deltaRefInfo_ = Some(_decode_DeltaRefInfo(_el)?),
            "cRLScope" => cRLScope_ = Some(_decode_CRLScopeSyntax(_el)?),
            "lastUpdate" => lastUpdate_ = Some(BER.decode_generalized_time(_el)?),
            "lastChangedCRL" => lastChangedCRL_ = Some(BER.decode_generalized_time(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CRLReferral {
        issuer: issuer_,
        location: location_,
        deltaRefInfo: deltaRefInfo_,
        cRLScope: cRLScope_.unwrap(),
        lastUpdate: lastUpdate_,
        lastChangedCRL: lastChangedCRL_,
        _unrecognized,
    })
}

pub fn _encode_CRLReferral(value_: &CRLReferral) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(16);
    if let Some(v_) = &value_.issuer {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.location {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.deltaRefInfo {
        components_.push(|v_1: &DeltaRefInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_DeltaRefInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_CRLScopeSyntax(&value_.cRLScope)?);
    if let Some(v_) = &value_.lastUpdate {
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_generalized_time(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.lastChangedCRL {
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_generalized_time(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 4;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CRLReferral(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CRLReferral")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CRLReferral,
        _eal_components_for_CRLReferral,
        _rctl2_components_for_CRLReferral,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "issuer"));
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "location" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "location")
                    );
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "deltaRefInfo" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "deltaRefInfo")
                    );
                }
                Ok(_validate_DeltaRefInfo(&el)?)
            }(_el)?,
            "cRLScope" => _validate_CRLScopeSyntax(_el)?,
            "lastUpdate" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "lastUpdate")
                    );
                }
                Ok(BER.validate_generalized_time(&el)?)
            }(_el)?,
            "lastChangedCRL" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "lastChangedCRL")
                    );
                }
                Ok(BER.validate_generalized_time(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for DeltaRefInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DeltaRefInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DeltaRefInfo,
        _eal_components_for_DeltaRefInfo,
        _rctl2_components_for_DeltaRefInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut deltaLocation_: OPTIONAL<GeneralName> = None;
    let mut lastDelta_: OPTIONAL<GeneralizedTime> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "deltaLocation" => deltaLocation_ = Some(_decode_GeneralName(_el)?),
            "lastDelta" => lastDelta_ = Some(BER.decode_generalized_time(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DeltaRefInfo {
        deltaLocation: deltaLocation_.unwrap(),
        lastDelta: lastDelta_,
        _unrecognized,
    })
}

pub fn _encode_DeltaRefInfo(value_: &DeltaRefInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_GeneralName(&value_.deltaLocation)?);
    if let Some(v_) = &value_.lastDelta {
        components_.push(BER.encode_generalized_time(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DeltaRefInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DeltaRefInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DeltaRefInfo,
        _eal_components_for_DeltaRefInfo,
        _rctl2_components_for_DeltaRefInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "deltaLocation" => _validate_GeneralName(_el)?,
            "lastDelta" => BER.validate_generalized_time(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod cRLStreamIdentifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = CRLStreamIdentifier; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_CRLStreamIdentifier(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_CRLStreamIdentifier(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_CRLStreamIdentifier(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CRLStreamIdentifier  ::=  INTEGER (0..MAX)
/// ```
pub type CRLStreamIdentifier = INTEGER;

pub fn _decode_CRLStreamIdentifier(el: &X690Element) -> ASN1Result<CRLStreamIdentifier> {
    BER.decode_integer(&el)
}

pub fn _encode_CRLStreamIdentifier(value_: &CRLStreamIdentifier) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_CRLStreamIdentifier(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
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

pub mod orderedList {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = OrderedListSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_OrderedListSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_OrderedListSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_OrderedListSyntax(el)
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
    BER.decode_enumerated(&el)
}

pub fn _encode_OrderedListSyntax(value_: &OrderedListSyntax) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_OrderedListSyntax(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
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

pub mod deltaInfo {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = DeltaInformation; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_DeltaInformation(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_DeltaInformation(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_DeltaInformation(el)
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
impl TryFrom<&X690Element> for DeltaInformation {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DeltaInformation")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DeltaInformation,
        _eal_components_for_DeltaInformation,
        _rctl2_components_for_DeltaInformation,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut deltaLocation_: OPTIONAL<GeneralName> = None;
    let mut nextDelta_: OPTIONAL<GeneralizedTime> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "deltaLocation" => deltaLocation_ = Some(_decode_GeneralName(_el)?),
            "nextDelta" => nextDelta_ = Some(BER.decode_generalized_time(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DeltaInformation {
        deltaLocation: deltaLocation_.unwrap(),
        nextDelta: nextDelta_,
        _unrecognized,
    })
}

pub fn _encode_DeltaInformation(value_: &DeltaInformation) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_GeneralName(&value_.deltaLocation)?);
    if let Some(v_) = &value_.nextDelta {
        components_.push(BER.encode_generalized_time(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DeltaInformation(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DeltaInformation")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DeltaInformation,
        _eal_components_for_DeltaInformation,
        _rctl2_components_for_DeltaInformation,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "deltaLocation" => _validate_GeneralName(_el)?,
            "nextDelta" => BER.validate_generalized_time(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod toBeRevoked {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = ToBeRevokedSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_ToBeRevokedSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_ToBeRevokedSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_ToBeRevokedSyntax(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ToBeRevokedSyntax  ::=  SEQUENCE SIZE (1..MAX) OF ToBeRevokedGroup
/// ```
pub type ToBeRevokedSyntax = Vec<ToBeRevokedGroup>; // SequenceOfType

pub fn _decode_ToBeRevokedSyntax(el: &X690Element) -> ASN1Result<ToBeRevokedSyntax> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ToBeRevokedSyntax")
            )
        }
    };
    let mut items: SEQUENCE_OF<ToBeRevokedGroup> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_ToBeRevokedGroup(el)?);
    }
    Ok(items)
}

pub fn _encode_ToBeRevokedSyntax(value_: &ToBeRevokedSyntax) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_ToBeRevokedGroup(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_ToBeRevokedSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_ToBeRevokedGroup(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ToBeRevokedSyntax")),
    }
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
impl TryFrom<&X690Element> for ToBeRevokedGroup {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ToBeRevokedGroup")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ToBeRevokedGroup,
        _eal_components_for_ToBeRevokedGroup,
        _rctl2_components_for_ToBeRevokedGroup,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut certificateIssuer_: OPTIONAL<GeneralName> = None;
    let mut reasonInfo_: OPTIONAL<ReasonInfo> = None;
    let mut revocationTime_: OPTIONAL<GeneralizedTime> = None;
    let mut certificateGroup_: OPTIONAL<CertificateGroup> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "certificateIssuer" => {
                certificateIssuer_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "reasonInfo" => reasonInfo_ = Some(_decode_ReasonInfo(_el)?),
            "revocationTime" => revocationTime_ = Some(BER.decode_generalized_time(_el)?),
            "certificateGroup" => certificateGroup_ = Some(_decode_CertificateGroup(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ToBeRevokedGroup {
        certificateIssuer: certificateIssuer_,
        reasonInfo: reasonInfo_,
        revocationTime: revocationTime_.unwrap(),
        certificateGroup: certificateGroup_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ToBeRevokedGroup(value_: &ToBeRevokedGroup) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    if let Some(v_) = &value_.certificateIssuer {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.reasonInfo {
        components_.push(|v_1: &ReasonInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ReasonInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(BER.encode_generalized_time(&value_.revocationTime)?);
    components_.push(_encode_CertificateGroup(&value_.certificateGroup)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ToBeRevokedGroup(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ToBeRevokedGroup")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ToBeRevokedGroup,
        _eal_components_for_ToBeRevokedGroup,
        _rctl2_components_for_ToBeRevokedGroup,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "certificateIssuer" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "certificateIssuer",
                    ));
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "reasonInfo" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "reasonInfo")
                    );
                }
                Ok(_validate_ReasonInfo(&el)?)
            }(_el)?,
            "revocationTime" => BER.validate_generalized_time(_el)?,
            "certificateGroup" => _validate_CertificateGroup(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for ReasonInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ReasonInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ReasonInfo,
        _eal_components_for_ReasonInfo,
        _rctl2_components_for_ReasonInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut reasonCode_: OPTIONAL<CRLReason> = None;
    let mut holdInstructionCode_: OPTIONAL<HoldInstruction> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "reasonCode" => reasonCode_ = Some(_decode_CRLReason(_el)?),
            "holdInstructionCode" => holdInstructionCode_ = Some(_decode_HoldInstruction(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ReasonInfo {
        reasonCode: reasonCode_.unwrap(),
        holdInstructionCode: holdInstructionCode_,
        _unrecognized,
    })
}

pub fn _encode_ReasonInfo(value_: &ReasonInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_CRLReason(&value_.reasonCode)?);
    if let Some(v_) = &value_.holdInstructionCode {
        components_.push(_encode_HoldInstruction(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ReasonInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ReasonInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ReasonInfo,
        _eal_components_for_ReasonInfo,
        _rctl2_components_for_ReasonInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "reasonCode" => _validate_CRLReason(_el)?,
            "holdInstructionCode" => _validate_HoldInstruction(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for CertificateGroup {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateGroup(el)
    }
}

pub fn _decode_CertificateGroup(el: &X690Element) -> ASN1Result<CertificateGroup> {
    match (el.tag.tag_class, el.tag.tag_number) {
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
}

pub fn _encode_CertificateGroup(value_: &CertificateGroup) -> ASN1Result<X690Element> {
    match value_ {
        CertificateGroup::serialNumbers(v) => {
            |v_1: &CertificateSerialNumbers| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateSerialNumbers(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
        CertificateGroup::serialNumberRange(v) => {
            |v_1: &CertificateGroupNumberRange| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateGroupNumberRange(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        CertificateGroup::nameSubtree(v) => |v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v),
        CertificateGroup::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertificateGroup(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "serialNumbers")
                );
            }
            Ok(_validate_CertificateSerialNumbers(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "serialNumberRange")
                );
            }
            Ok(_validate_CertificateGroupNumberRange(&el)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "nameSubtree")
                );
            }
            Ok(_validate_GeneralName(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
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
impl TryFrom<&X690Element> for CertificateGroupNumberRange {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateGroupNumberRange",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateGroupNumberRange,
        _eal_components_for_CertificateGroupNumberRange,
        _rctl2_components_for_CertificateGroupNumberRange,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut startingNumber_: OPTIONAL<INTEGER> = None;
    let mut endingNumber_: OPTIONAL<INTEGER> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "startingNumber" => startingNumber_ = Some(BER.decode_integer(_el)?),
            "endingNumber" => endingNumber_ = Some(BER.decode_integer(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertificateGroupNumberRange {
        startingNumber: startingNumber_.unwrap(),
        endingNumber: endingNumber_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertificateGroupNumberRange(
    value_: &CertificateGroupNumberRange,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
        let mut el_1 = BER.encode_integer(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_.startingNumber)?);
    components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
        let mut el_1 = BER.encode_integer(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 1;
        Ok(el_1)
    }(&value_.endingNumber)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertificateGroupNumberRange(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateGroupNumberRange",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateGroupNumberRange,
        _eal_components_for_CertificateGroupNumberRange,
        _rctl2_components_for_CertificateGroupNumberRange,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "startingNumber" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "startingNumber")
                    );
                }
                Ok(BER.validate_integer(&el)?)
            }(_el)?,
            "endingNumber" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "endingNumber")
                    );
                }
                Ok(BER.validate_integer(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateSerialNumbers  ::=  SEQUENCE SIZE (1..MAX) OF CertificateSerialNumber
/// ```
pub type CertificateSerialNumbers = Vec<CertificateSerialNumber>; // SequenceOfType

pub fn _decode_CertificateSerialNumbers(el: &X690Element) -> ASN1Result<CertificateSerialNumbers> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateSerialNumbers",
            ))
        }
    };
    let mut items: SEQUENCE_OF<CertificateSerialNumber> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_CertificateSerialNumber(el)?);
    }
    Ok(items)
}

pub fn _encode_CertificateSerialNumbers(
    value_: &CertificateSerialNumbers,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_CertificateSerialNumber(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CertificateSerialNumbers(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_CertificateSerialNumber(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "CertificateSerialNumbers",
        )),
    }
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

pub mod revokedGroups {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = RevokedGroupsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_RevokedGroupsSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_RevokedGroupsSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_RevokedGroupsSyntax(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RevokedGroupsSyntax  ::=  SEQUENCE SIZE (1..MAX) OF RevokedGroup
/// ```
pub type RevokedGroupsSyntax = Vec<RevokedGroup>; // SequenceOfType

pub fn _decode_RevokedGroupsSyntax(el: &X690Element) -> ASN1Result<RevokedGroupsSyntax> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RevokedGroupsSyntax")
            )
        }
    };
    let mut items: SEQUENCE_OF<RevokedGroup> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_RevokedGroup(el)?);
    }
    Ok(items)
}

pub fn _encode_RevokedGroupsSyntax(value_: &RevokedGroupsSyntax) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_RevokedGroup(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_RevokedGroupsSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_RevokedGroup(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RevokedGroupsSyntax")),
    }
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
impl TryFrom<&X690Element> for RevokedGroup {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RevokedGroup")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RevokedGroup,
        _eal_components_for_RevokedGroup,
        _rctl2_components_for_RevokedGroup,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut certificateIssuer_: OPTIONAL<GeneralName> = None;
    let mut reasonInfo_: OPTIONAL<ReasonInfo> = None;
    let mut invalidityDate_: OPTIONAL<GeneralizedTime> = None;
    let mut revokedcertificateGroup_: OPTIONAL<RevokedCertificateGroup> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "certificateIssuer" => {
                certificateIssuer_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "reasonInfo" => reasonInfo_ = Some(_decode_ReasonInfo(_el)?),
            "invalidityDate" => invalidityDate_ = Some(BER.decode_generalized_time(_el)?),
            "revokedcertificateGroup" => {
                revokedcertificateGroup_ =
                    Some(|el: &X690Element| -> ASN1Result<RevokedCertificateGroup> {
                        Ok(_decode_RevokedCertificateGroup(&el.inner()?)?)
                    }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(RevokedGroup {
        certificateIssuer: certificateIssuer_,
        reasonInfo: reasonInfo_,
        invalidityDate: invalidityDate_,
        revokedcertificateGroup: revokedcertificateGroup_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_RevokedGroup(value_: &RevokedGroup) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    if let Some(v_) = &value_.certificateIssuer {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.reasonInfo {
        components_.push(|v_1: &ReasonInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ReasonInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.invalidityDate {
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_generalized_time(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(|v_1: &RevokedCertificateGroup| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 3),
            X690Value::from_explicit(&_encode_RevokedCertificateGroup(&v_1)?),
        ))
    }(&value_.revokedcertificateGroup)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_RevokedGroup(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RevokedGroup")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RevokedGroup,
        _eal_components_for_RevokedGroup,
        _rctl2_components_for_RevokedGroup,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "certificateIssuer" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "certificateIssuer",
                    ));
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "reasonInfo" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "reasonInfo")
                    );
                }
                Ok(_validate_ReasonInfo(&el)?)
            }(_el)?,
            "invalidityDate" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "invalidityDate")
                    );
                }
                Ok(BER.validate_generalized_time(&el)?)
            }(_el)?,
            "revokedcertificateGroup" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "revokedcertificateGroup",
                    ));
                }
                Ok(_validate_RevokedCertificateGroup(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for RevokedCertificateGroup {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RevokedCertificateGroup(el)
    }
}

pub fn _decode_RevokedCertificateGroup(el: &X690Element) -> ASN1Result<RevokedCertificateGroup> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(RevokedCertificateGroup::serialNumberRange(
            _decode_NumberRange(&el)?,
        )),
        (TagClass::CONTEXT, 0) => Ok(RevokedCertificateGroup::nameSubtree(_decode_GeneralName(
            &el,
        )?)),
        (TagClass::CONTEXT, 1) => Ok(RevokedCertificateGroup::nameSubtree(_decode_GeneralName(
            &el,
        )?)),
        (TagClass::CONTEXT, 2) => Ok(RevokedCertificateGroup::nameSubtree(_decode_GeneralName(
            &el,
        )?)),
        (TagClass::CONTEXT, 3) => Ok(RevokedCertificateGroup::nameSubtree(_decode_GeneralName(
            &el,
        )?)),
        (TagClass::CONTEXT, 4) => Ok(RevokedCertificateGroup::nameSubtree(_decode_GeneralName(
            &el,
        )?)),
        (TagClass::CONTEXT, 5) => Ok(RevokedCertificateGroup::nameSubtree(_decode_GeneralName(
            &el,
        )?)),
        (TagClass::CONTEXT, 6) => Ok(RevokedCertificateGroup::nameSubtree(_decode_GeneralName(
            &el,
        )?)),
        (TagClass::CONTEXT, 7) => Ok(RevokedCertificateGroup::nameSubtree(_decode_GeneralName(
            &el,
        )?)),
        (TagClass::CONTEXT, 8) => Ok(RevokedCertificateGroup::nameSubtree(_decode_GeneralName(
            &el,
        )?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "RevokedCertificateGroup",
            ))
        }
    }
}

pub fn _encode_RevokedCertificateGroup(
    value_: &RevokedCertificateGroup,
) -> ASN1Result<X690Element> {
    match value_ {
        RevokedCertificateGroup::serialNumberRange(v) => _encode_NumberRange(&v),
        RevokedCertificateGroup::nameSubtree(v) => _encode_GeneralName(&v),
    }
}

pub fn _validate_RevokedCertificateGroup(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_NumberRange(&el),
        (TagClass::CONTEXT, 0) => _validate_GeneralName(&el),
        (TagClass::CONTEXT, 1) => _validate_GeneralName(&el),
        (TagClass::CONTEXT, 2) => _validate_GeneralName(&el),
        (TagClass::CONTEXT, 3) => _validate_GeneralName(&el),
        (TagClass::CONTEXT, 4) => _validate_GeneralName(&el),
        (TagClass::CONTEXT, 5) => _validate_GeneralName(&el),
        (TagClass::CONTEXT, 6) => _validate_GeneralName(&el),
        (TagClass::CONTEXT, 7) => _validate_GeneralName(&el),
        (TagClass::CONTEXT, 8) => _validate_GeneralName(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "RevokedCertificateGroup",
            ))
        }
    }
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

pub mod expiredCertsOnCRL {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = ExpiredCertsOnCRL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_ExpiredCertsOnCRL(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_ExpiredCertsOnCRL(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_ExpiredCertsOnCRL(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExpiredCertsOnCRL  ::=  GeneralizedTime
/// ```
pub type ExpiredCertsOnCRL = GeneralizedTime; // GeneralizedTime

pub fn _decode_ExpiredCertsOnCRL(el: &X690Element) -> ASN1Result<ExpiredCertsOnCRL> {
    BER.decode_generalized_time(&el)
}

pub fn _encode_ExpiredCertsOnCRL(value_: &ExpiredCertsOnCRL) -> ASN1Result<X690Element> {
    BER.encode_generalized_time(&value_)
}

pub fn _validate_ExpiredCertsOnCRL(el: &X690Element) -> ASN1Result<()> {
    BER.validate_generalized_time(&el)
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

pub mod reasonCode {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = CRLReason; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_CRLReason(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_CRLReason(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_CRLReason(el)
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
    BER.decode_enumerated(&el)
}

pub fn _encode_CRLReason(value_: &CRLReason) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_CRLReason(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
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

pub mod holdInstructionCode {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = HoldInstruction; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_HoldInstruction(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_HoldInstruction(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_HoldInstruction(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HoldInstruction  ::=  OBJECT IDENTIFIER
/// ```
pub type HoldInstruction = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_HoldInstruction(el: &X690Element) -> ASN1Result<HoldInstruction> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_HoldInstruction(value_: &HoldInstruction) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_HoldInstruction(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
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

pub mod invalidityDate {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = GeneralizedTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        BER.decode_generalized_time(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        BER.encode_generalized_time(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_generalized_time(el)
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

pub mod cRLDistributionPoints {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = CRLDistPointsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_CRLDistPointsSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_CRLDistPointsSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_CRLDistPointsSyntax(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CRLDistPointsSyntax  ::=  SEQUENCE SIZE (1..MAX) OF DistributionPoint
/// ```
pub type CRLDistPointsSyntax = Vec<DistributionPoint>; // SequenceOfType

pub fn _decode_CRLDistPointsSyntax(el: &X690Element) -> ASN1Result<CRLDistPointsSyntax> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CRLDistPointsSyntax")
            )
        }
    };
    let mut items: SEQUENCE_OF<DistributionPoint> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_DistributionPoint(el)?);
    }
    Ok(items)
}

pub fn _encode_CRLDistPointsSyntax(value_: &CRLDistPointsSyntax) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_DistributionPoint(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CRLDistPointsSyntax(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_DistributionPoint(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CRLDistPointsSyntax")),
    }
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
impl TryFrom<&X690Element> for DistributionPoint {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DistributionPoint")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DistributionPoint,
        _eal_components_for_DistributionPoint,
        _rctl2_components_for_DistributionPoint,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut distributionPoint_: OPTIONAL<DistributionPointName> = None;
    let mut reasons_: OPTIONAL<ReasonFlags> = None;
    let mut cRLIssuer_: OPTIONAL<GeneralNames> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "distributionPoint" => {
                distributionPoint_ = Some(|el: &X690Element| -> ASN1Result<DistributionPointName> {
                    Ok(_decode_DistributionPointName(&el.inner()?)?)
                }(_el)?)
            }
            "reasons" => reasons_ = Some(_decode_ReasonFlags(_el)?),
            "cRLIssuer" => cRLIssuer_ = Some(_decode_GeneralNames(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DistributionPoint {
        distributionPoint: distributionPoint_,
        reasons: reasons_,
        cRLIssuer: cRLIssuer_,
        _unrecognized,
    })
}

pub fn _encode_DistributionPoint(value_: &DistributionPoint) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.distributionPoint {
        components_.push(|v_1: &DistributionPointName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_DistributionPointName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.reasons {
        components_.push(|v_1: &ReasonFlags| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ReasonFlags(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.cRLIssuer {
        components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralNames(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DistributionPoint(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DistributionPoint")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DistributionPoint,
        _eal_components_for_DistributionPoint,
        _rctl2_components_for_DistributionPoint,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "distributionPoint" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "distributionPoint",
                    ));
                }
                Ok(_validate_DistributionPointName(&el.inner()?)?)
            }(_el)?,
            "reasons" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "reasons")
                    );
                }
                Ok(_validate_ReasonFlags(&el)?)
            }(_el)?,
            "cRLIssuer" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "cRLIssuer")
                    );
                }
                Ok(_validate_GeneralNames(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for DistributionPointName {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DistributionPointName(el)
    }
}

pub fn _decode_DistributionPointName(el: &X690Element) -> ASN1Result<DistributionPointName> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(DistributionPointName::fullName(_decode_GeneralNames(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(DistributionPointName::nameRelativeToCRLIssuer(
            _decode_RelativeDistinguishedName(&el)?,
        )),
        _ => Ok(DistributionPointName::_unrecognized(el.clone())),
    }
}

pub fn _encode_DistributionPointName(value_: &DistributionPointName) -> ASN1Result<X690Element> {
    match value_ {
        DistributionPointName::fullName(v) => |v_1: &GeneralNames| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralNames(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        DistributionPointName::nameRelativeToCRLIssuer(v) => {
            |v_1: &RelativeDistinguishedName| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_RelativeDistinguishedName(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        DistributionPointName::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_DistributionPointName(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fullName"));
            }
            Ok(_validate_GeneralNames(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "nameRelativeToCRLIssuer",
                ));
            }
            Ok(_validate_RelativeDistinguishedName(&el)?)
        }(&el),
        _ => Ok(()),
    }
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
    BER.decode_bit_string(&el)
}

pub fn _encode_ReasonFlags(value_: &ReasonFlags) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_ReasonFlags(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
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

pub mod issuingDistributionPoint {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = IssuingDistPointSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_IssuingDistPointSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_IssuingDistPointSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_IssuingDistPointSyntax(el)
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
impl TryFrom<&X690Element> for IssuingDistPointSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "IssuingDistPointSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IssuingDistPointSyntax,
        _eal_components_for_IssuingDistPointSyntax,
        _rctl2_components_for_IssuingDistPointSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut distributionPoint_: OPTIONAL<DistributionPointName> = None;
    let mut onlyContainsUserPublicKeyCerts_: OPTIONAL<BOOLEAN> = None;
    let mut onlyContainsCACerts_: OPTIONAL<BOOLEAN> = None;
    let mut onlySomeReasons_: OPTIONAL<ReasonFlags> = None;
    let mut indirectCRL_: OPTIONAL<BOOLEAN> = None;
    let mut onlyContainsAttributeCerts_: OPTIONAL<BOOLEAN> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "distributionPoint" => {
                distributionPoint_ = Some(|el: &X690Element| -> ASN1Result<DistributionPointName> {
                    Ok(_decode_DistributionPointName(&el.inner()?)?)
                }(_el)?)
            }
            "onlyContainsUserPublicKeyCerts" => {
                onlyContainsUserPublicKeyCerts_ = Some(BER.decode_boolean(_el)?)
            }
            "onlyContainsCACerts" => onlyContainsCACerts_ = Some(BER.decode_boolean(_el)?),
            "onlySomeReasons" => onlySomeReasons_ = Some(_decode_ReasonFlags(_el)?),
            "indirectCRL" => indirectCRL_ = Some(BER.decode_boolean(_el)?),
            "onlyContainsAttributeCerts" => {
                onlyContainsAttributeCerts_ = Some(BER.decode_boolean(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(IssuingDistPointSyntax {
        distributionPoint: distributionPoint_,
        onlyContainsUserPublicKeyCerts: onlyContainsUserPublicKeyCerts_,
        onlyContainsCACerts: onlyContainsCACerts_,
        onlySomeReasons: onlySomeReasons_,
        indirectCRL: indirectCRL_,
        onlyContainsAttributeCerts: onlyContainsAttributeCerts_,
        _unrecognized,
    })
}

pub fn _encode_IssuingDistPointSyntax(value_: &IssuingDistPointSyntax) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(16);
    if let Some(v_) = &value_.distributionPoint {
        components_.push(|v_1: &DistributionPointName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_DistributionPointName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.onlyContainsUserPublicKeyCerts {
        if *v_ != IssuingDistPointSyntax::_default_value_for_onlyContainsUserPublicKeyCerts() {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                let mut el_1 = BER.encode_boolean(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.onlyContainsCACerts {
        if *v_ != IssuingDistPointSyntax::_default_value_for_onlyContainsCACerts() {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                let mut el_1 = BER.encode_boolean(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.onlySomeReasons {
        components_.push(|v_1: &ReasonFlags| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ReasonFlags(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.indirectCRL {
        if *v_ != IssuingDistPointSyntax::_default_value_for_indirectCRL() {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                let mut el_1 = BER.encode_boolean(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 4;
                Ok(el_1)
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.onlyContainsAttributeCerts {
        components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_boolean(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 5;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_IssuingDistPointSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "IssuingDistPointSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IssuingDistPointSyntax,
        _eal_components_for_IssuingDistPointSyntax,
        _rctl2_components_for_IssuingDistPointSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "distributionPoint" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "distributionPoint",
                    ));
                }
                Ok(_validate_DistributionPointName(&el.inner()?)?)
            }(_el)?,
            "onlyContainsUserPublicKeyCerts" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "onlyContainsUserPublicKeyCerts",
                    ));
                }
                Ok(BER.validate_boolean(&el)?)
            }(_el)?,
            "onlyContainsCACerts" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "onlyContainsCACerts",
                    ));
                }
                Ok(BER.validate_boolean(&el)?)
            }(_el)?,
            "onlySomeReasons" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "onlySomeReasons",
                    ));
                }
                Ok(_validate_ReasonFlags(&el)?)
            }(_el)?,
            "indirectCRL" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "indirectCRL")
                    );
                }
                Ok(BER.validate_boolean(&el)?)
            }(_el)?,
            "onlyContainsAttributeCerts" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "onlyContainsAttributeCerts",
                    ));
                }
                Ok(BER.validate_boolean(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod certificateIssuer {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = GeneralNames; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_GeneralNames(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_GeneralNames(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_GeneralNames(el)
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

pub mod deltaCRLIndicator {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = BaseCRLNumber; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_BaseCRLNumber(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_BaseCRLNumber(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_BaseCRLNumber(el)
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

pub fn _validate_BaseCRLNumber(el: &X690Element) -> ASN1Result<()> {
    _validate_CRLNumber(&el)
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

pub mod baseUpdateTime {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = GeneralizedTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        BER.decode_generalized_time(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        BER.encode_generalized_time(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_generalized_time(el)
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

pub mod freshestCRL {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = CRLDistPointsSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_CRLDistPointsSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_CRLDistPointsSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_CRLDistPointsSyntax(el)
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

pub mod protRestrict {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = ProtRestriction; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_ProtRestriction(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_ProtRestriction(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_ProtRestriction(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ProtRestriction  ::=  SEQUENCE (SIZE (1..MAX)) OF OBJECT IDENTIFIER
/// ```
pub type ProtRestriction = Vec<OBJECT_IDENTIFIER>; // SequenceOfType

pub fn _decode_ProtRestriction(el: &X690Element) -> ASN1Result<ProtRestriction> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ProtRestriction"))
        }
    };
    let mut items: SEQUENCE_OF<OBJECT_IDENTIFIER> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(BER.decode_object_identifier(el)?);
    }
    Ok(items)
}

pub fn _encode_ProtRestriction(value_: &ProtRestriction) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(BER.encode_object_identifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_ProtRestriction(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                BER.validate_object_identifier(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ProtRestriction")),
    }
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

pub mod subjectAltPublicKeyInfo {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = SubjectAltPublicKeyInfo; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_SubjectAltPublicKeyInfo(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_SubjectAltPublicKeyInfo(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_SubjectAltPublicKeyInfo(el)
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
impl TryFrom<&X690Element> for SubjectAltPublicKeyInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "SubjectAltPublicKeyInfo",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SubjectAltPublicKeyInfo,
        _eal_components_for_SubjectAltPublicKeyInfo,
        _rctl2_components_for_SubjectAltPublicKeyInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithm_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut subjectAltPublicKey_: OPTIONAL<BIT_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => algorithm_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "subjectAltPublicKey" => subjectAltPublicKey_ = Some(BER.decode_bit_string(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "SubjectAltPublicKeyInfo",
                ))
            }
        }
    }
    Ok(SubjectAltPublicKeyInfo {
        algorithm: algorithm_.unwrap(),
        subjectAltPublicKey: subjectAltPublicKey_.unwrap(),
    })
}

pub fn _encode_SubjectAltPublicKeyInfo(
    value_: &SubjectAltPublicKeyInfo,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_AlgorithmIdentifier(&value_.algorithm)?);
    components_.push(BER.encode_bit_string(&value_.subjectAltPublicKey)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SubjectAltPublicKeyInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "SubjectAltPublicKeyInfo",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SubjectAltPublicKeyInfo,
        _eal_components_for_SubjectAltPublicKeyInfo,
        _rctl2_components_for_SubjectAltPublicKeyInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => _validate_AlgorithmIdentifier(_el)?,
            "subjectAltPublicKey" => BER.validate_bit_string(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "SubjectAltPublicKeyInfo",
                ))
            }
        }
    }
    Ok(())
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

pub mod altSignatureAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AltSignatureAlgorithm; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AltSignatureAlgorithm(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AltSignatureAlgorithm(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AltSignatureAlgorithm(el)
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

pub fn _validate_AltSignatureAlgorithm(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
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

pub mod altSignatureValue {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AltSignatureValue; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AltSignatureValue(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AltSignatureValue(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AltSignatureValue(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AltSignatureValue  ::=  BIT STRING
/// ```
pub type AltSignatureValue = BIT_STRING;

pub fn _decode_AltSignatureValue(el: &X690Element) -> ASN1Result<AltSignatureValue> {
    BER.decode_bit_string(&el)
}

pub fn _encode_AltSignatureValue(value_: &AltSignatureValue) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_AltSignatureValue(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
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

pub mod aAissuingDistributionPoint {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = AAIssuingDistPointSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_AAIssuingDistPointSyntax(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_AAIssuingDistPointSyntax(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_AAIssuingDistPointSyntax(el)
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
impl TryFrom<&X690Element> for AAIssuingDistPointSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AAIssuingDistPointSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AAIssuingDistPointSyntax,
        _eal_components_for_AAIssuingDistPointSyntax,
        _rctl2_components_for_AAIssuingDistPointSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut distributionPoint_: OPTIONAL<DistributionPointName> = None;
    let mut onlySomeReasons_: OPTIONAL<ReasonFlags> = None;
    let mut indirectCRL_: OPTIONAL<BOOLEAN> = None;
    let mut containsUserAttributeCerts_: OPTIONAL<BOOLEAN> = None;
    let mut containsAACerts_: OPTIONAL<BOOLEAN> = None;
    let mut containsSOAPublicKeyCerts_: OPTIONAL<BOOLEAN> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "distributionPoint" => {
                distributionPoint_ = Some(|el: &X690Element| -> ASN1Result<DistributionPointName> {
                    Ok(_decode_DistributionPointName(&el.inner()?)?)
                }(_el)?)
            }
            "onlySomeReasons" => onlySomeReasons_ = Some(_decode_ReasonFlags(_el)?),
            "indirectCRL" => indirectCRL_ = Some(BER.decode_boolean(_el)?),
            "containsUserAttributeCerts" => {
                containsUserAttributeCerts_ = Some(BER.decode_boolean(_el)?)
            }
            "containsAACerts" => containsAACerts_ = Some(BER.decode_boolean(_el)?),
            "containsSOAPublicKeyCerts" => {
                containsSOAPublicKeyCerts_ = Some(BER.decode_boolean(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AAIssuingDistPointSyntax {
        distributionPoint: distributionPoint_,
        onlySomeReasons: onlySomeReasons_,
        indirectCRL: indirectCRL_,
        containsUserAttributeCerts: containsUserAttributeCerts_,
        containsAACerts: containsAACerts_,
        containsSOAPublicKeyCerts: containsSOAPublicKeyCerts_,
        _unrecognized,
    })
}

pub fn _encode_AAIssuingDistPointSyntax(
    value_: &AAIssuingDistPointSyntax,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(16);
    if let Some(v_) = &value_.distributionPoint {
        components_.push(|v_1: &DistributionPointName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_DistributionPointName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.onlySomeReasons {
        components_.push(|v_1: &ReasonFlags| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ReasonFlags(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.indirectCRL {
        if *v_ != AAIssuingDistPointSyntax::_default_value_for_indirectCRL() {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                let mut el_1 = BER.encode_boolean(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.containsUserAttributeCerts {
        if *v_ != AAIssuingDistPointSyntax::_default_value_for_containsUserAttributeCerts() {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                let mut el_1 = BER.encode_boolean(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.containsAACerts {
        if *v_ != AAIssuingDistPointSyntax::_default_value_for_containsAACerts() {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                let mut el_1 = BER.encode_boolean(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 4;
                Ok(el_1)
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.containsSOAPublicKeyCerts {
        if *v_ != AAIssuingDistPointSyntax::_default_value_for_containsSOAPublicKeyCerts() {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                let mut el_1 = BER.encode_boolean(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 5;
                Ok(el_1)
            }(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AAIssuingDistPointSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AAIssuingDistPointSyntax",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AAIssuingDistPointSyntax,
        _eal_components_for_AAIssuingDistPointSyntax,
        _rctl2_components_for_AAIssuingDistPointSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "distributionPoint" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "distributionPoint",
                    ));
                }
                Ok(_validate_DistributionPointName(&el.inner()?)?)
            }(_el)?,
            "onlySomeReasons" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "onlySomeReasons",
                    ));
                }
                Ok(_validate_ReasonFlags(&el)?)
            }(_el)?,
            "indirectCRL" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "indirectCRL")
                    );
                }
                Ok(BER.validate_boolean(&el)?)
            }(_el)?,
            "containsUserAttributeCerts" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "containsUserAttributeCerts",
                    ));
                }
                Ok(BER.validate_boolean(&el)?)
            }(_el)?,
            "containsAACerts" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "containsAACerts",
                    ));
                }
                Ok(BER.validate_boolean(&el)?)
            }(_el)?,
            "containsSOAPublicKeyCerts" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "containsSOAPublicKeyCerts",
                    ));
                }
                Ok(BER.validate_boolean(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod certificateExactMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = CertificateExactAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_CertificateExactAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_CertificateExactAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateExactAssertion(el)
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
impl TryFrom<&X690Element> for CertificateExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateExactAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateExactAssertion,
        _eal_components_for_CertificateExactAssertion,
        _rctl2_components_for_CertificateExactAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            "issuer" => issuer_ = Some(_decode_Name(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertificateExactAssertion {
        serialNumber: serialNumber_.unwrap(),
        issuer: issuer_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertificateExactAssertion(
    value_: &CertificateExactAssertion,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    components_.push(_encode_Name(&value_.issuer)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertificateExactAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateExactAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateExactAssertion,
        _eal_components_for_CertificateExactAssertion,
        _rctl2_components_for_CertificateExactAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            "issuer" => _validate_Name(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod certificateMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = CertificateAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_CertificateAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_CertificateAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateAssertion(el)
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
impl TryFrom<&X690Element> for CertificateAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificateAssertion")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateAssertion,
        _eal_components_for_CertificateAssertion,
        _rctl2_components_for_CertificateAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut subjectKeyIdentifier_: OPTIONAL<SubjectKeyIdentifier> = None;
    let mut authorityKeyIdentifier_: OPTIONAL<AuthorityKeyIdentifier> = None;
    let mut certificateValid_: OPTIONAL<Time> = None;
    let mut privateKeyValid_: OPTIONAL<GeneralizedTime> = None;
    let mut subjectPublicKeyAlgID_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut keyUsage_: OPTIONAL<KeyUsage> = None;
    let mut subjectAltName_: OPTIONAL<AltNameType> = None;
    let mut policy_: OPTIONAL<CertPolicySet> = None;
    let mut pathToName_: OPTIONAL<Name> = None;
    let mut subject_: OPTIONAL<Name> = None;
    let mut nameConstraints_: OPTIONAL<NameConstraintsSyntax> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            "issuer" => {
                issuer_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                    Ok(_decode_Name(&el.inner()?)?)
                }(_el)?)
            }
            "subjectKeyIdentifier" => {
                subjectKeyIdentifier_ = Some(_decode_SubjectKeyIdentifier(_el)?)
            }
            "authorityKeyIdentifier" => {
                authorityKeyIdentifier_ = Some(_decode_AuthorityKeyIdentifier(_el)?)
            }
            "certificateValid" => {
                certificateValid_ = Some(|el: &X690Element| -> ASN1Result<Time> {
                    Ok(_decode_Time(&el.inner()?)?)
                }(_el)?)
            }
            "privateKeyValid" => privateKeyValid_ = Some(BER.decode_generalized_time(_el)?),
            "subjectPublicKeyAlgID" => {
                subjectPublicKeyAlgID_ = Some(BER.decode_object_identifier(_el)?)
            }
            "keyUsage" => keyUsage_ = Some(_decode_KeyUsage(_el)?),
            "subjectAltName" => {
                subjectAltName_ = Some(|el: &X690Element| -> ASN1Result<AltNameType> {
                    Ok(_decode_AltNameType(&el.inner()?)?)
                }(_el)?)
            }
            "policy" => policy_ = Some(_decode_CertPolicySet(_el)?),
            "pathToName" => {
                pathToName_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                    Ok(_decode_Name(&el.inner()?)?)
                }(_el)?)
            }
            "subject" => {
                subject_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                    Ok(_decode_Name(&el.inner()?)?)
                }(_el)?)
            }
            "nameConstraints" => nameConstraints_ = Some(_decode_NameConstraintsSyntax(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertificateAssertion {
        serialNumber: serialNumber_,
        issuer: issuer_,
        subjectKeyIdentifier: subjectKeyIdentifier_,
        authorityKeyIdentifier: authorityKeyIdentifier_,
        certificateValid: certificateValid_,
        privateKeyValid: privateKeyValid_,
        subjectPublicKeyAlgID: subjectPublicKeyAlgID_,
        keyUsage: keyUsage_,
        subjectAltName: subjectAltName_,
        policy: policy_,
        pathToName: pathToName_,
        subject: subject_,
        nameConstraints: nameConstraints_,
        _unrecognized,
    })
}

pub fn _encode_CertificateAssertion(value_: &CertificateAssertion) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(23);
    if let Some(v_) = &value_.serialNumber {
        components_.push(|v_1: &CertificateSerialNumber| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertificateSerialNumber(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.issuer {
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_Name(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.subjectKeyIdentifier {
        components_.push(|v_1: &SubjectKeyIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SubjectKeyIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.authorityKeyIdentifier {
        components_.push(|v_1: &AuthorityKeyIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AuthorityKeyIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.certificateValid {
        components_.push(|v_1: &Time| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::from_explicit(&_encode_Time(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.privateKeyValid {
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_generalized_time(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 5;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.subjectPublicKeyAlgID {
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_object_identifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 6;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.keyUsage {
        components_.push(|v_1: &KeyUsage| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_KeyUsage(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 7;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.subjectAltName {
        components_.push(|v_1: &AltNameType| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 8),
                X690Value::from_explicit(&_encode_AltNameType(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.policy {
        components_.push(|v_1: &CertPolicySet| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertPolicySet(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 9;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.pathToName {
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 10),
                X690Value::from_explicit(&_encode_Name(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.subject {
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 11),
                X690Value::from_explicit(&_encode_Name(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.nameConstraints {
        components_.push(|v_1: &NameConstraintsSyntax| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_NameConstraintsSyntax(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 12;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertificateAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificateAssertion")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateAssertion,
        _eal_components_for_CertificateAssertion,
        _rctl2_components_for_CertificateAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serialNumber" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "serialNumber")
                    );
                }
                Ok(_validate_CertificateSerialNumber(&el)?)
            }(_el)?,
            "issuer" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "issuer"));
                }
                Ok(_validate_Name(&el.inner()?)?)
            }(_el)?,
            "subjectKeyIdentifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "subjectKeyIdentifier",
                    ));
                }
                Ok(_validate_SubjectKeyIdentifier(&el)?)
            }(_el)?,
            "authorityKeyIdentifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "authorityKeyIdentifier",
                    ));
                }
                Ok(_validate_AuthorityKeyIdentifier(&el)?)
            }(_el)?,
            "certificateValid" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "certificateValid",
                    ));
                }
                Ok(_validate_Time(&el.inner()?)?)
            }(_el)?,
            "privateKeyValid" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "privateKeyValid",
                    ));
                }
                Ok(BER.validate_generalized_time(&el)?)
            }(_el)?,
            "subjectPublicKeyAlgID" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "subjectPublicKeyAlgID",
                    ));
                }
                Ok(BER.validate_object_identifier(&el)?)
            }(_el)?,
            "keyUsage" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "keyUsage")
                    );
                }
                Ok(_validate_KeyUsage(&el)?)
            }(_el)?,
            "subjectAltName" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 8 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "subjectAltName")
                    );
                }
                Ok(_validate_AltNameType(&el.inner()?)?)
            }(_el)?,
            "policy" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 9 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "policy"));
                }
                Ok(_validate_CertPolicySet(&el)?)
            }(_el)?,
            "pathToName" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 10 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "pathToName")
                    );
                }
                Ok(_validate_Name(&el.inner()?)?)
            }(_el)?,
            "subject" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 11 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "subject")
                    );
                }
                Ok(_validate_Name(&el.inner()?)?)
            }(_el)?,
            "nameConstraints" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 12 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "nameConstraints",
                    ));
                }
                Ok(_validate_NameConstraintsSyntax(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for AltNameType {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AltNameType(el)
    }
}

pub fn _decode_AltNameType(el: &X690Element) -> ASN1Result<AltNameType> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 10) => Ok(AltNameType::builtinNameForm(
            _decode_AltNameType_builtinNameForm(&el)?,
        )),
        (TagClass::UNIVERSAL, 6) => Ok(AltNameType::otherNameForm(
            BER.decode_object_identifier(&el)?,
        )),
        _ => Ok(AltNameType::_unrecognized(el.clone())),
    }
}

pub fn _encode_AltNameType(value_: &AltNameType) -> ASN1Result<X690Element> {
    match value_ {
        AltNameType::builtinNameForm(v) => _encode_AltNameType_builtinNameForm(&v),
        AltNameType::otherNameForm(v) => BER.encode_object_identifier(&v),
        AltNameType::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_AltNameType(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 10) => _validate_AltNameType_builtinNameForm(&el),
        (TagClass::UNIVERSAL, 6) => BER.validate_object_identifier(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertPolicySet  ::=  SEQUENCE SIZE (1..MAX) OF CertPolicyId
/// ```
pub type CertPolicySet = Vec<CertPolicyId>; // SequenceOfType

pub fn _decode_CertPolicySet(el: &X690Element) -> ASN1Result<CertPolicySet> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertPolicySet")),
    };
    let mut items: SEQUENCE_OF<CertPolicyId> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_CertPolicyId(el)?);
    }
    Ok(items)
}

pub fn _encode_CertPolicySet(value_: &CertPolicySet) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_CertPolicyId(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CertPolicySet(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_CertPolicyId(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertPolicySet")),
    }
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

pub mod certificatePairExactMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = CertificatePairExactAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_CertificatePairExactAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_CertificatePairExactAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificatePairExactAssertion(el)
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
impl TryFrom<&X690Element> for CertificatePairExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificatePairExactAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificatePairExactAssertion,
        _eal_components_for_CertificatePairExactAssertion,
        _rctl2_components_for_CertificatePairExactAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuedToThisCAAssertion_: OPTIONAL<CertificateExactAssertion> = None;
    let mut issuedByThisCAAssertion_: OPTIONAL<CertificateExactAssertion> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuedToThisCAAssertion" => {
                issuedToThisCAAssertion_ = Some(_decode_CertificateExactAssertion(_el)?)
            }
            "issuedByThisCAAssertion" => {
                issuedByThisCAAssertion_ = Some(_decode_CertificateExactAssertion(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertificatePairExactAssertion {
        issuedToThisCAAssertion: issuedToThisCAAssertion_,
        issuedByThisCAAssertion: issuedByThisCAAssertion_,
        _unrecognized,
    })
}

pub fn _encode_CertificatePairExactAssertion(
    value_: &CertificatePairExactAssertion,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.issuedToThisCAAssertion {
        components_.push(
            |v_1: &CertificateExactAssertion| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateExactAssertion(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v_)?,
        );
    }
    if let Some(v_) = &value_.issuedByThisCAAssertion {
        components_.push(
            |v_1: &CertificateExactAssertion| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertificateExactAssertion(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v_)?,
        );
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertificatePairExactAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificatePairExactAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificatePairExactAssertion,
        _eal_components_for_CertificatePairExactAssertion,
        _rctl2_components_for_CertificatePairExactAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuedToThisCAAssertion" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "issuedToThisCAAssertion",
                    ));
                }
                Ok(_validate_CertificateExactAssertion(&el)?)
            }(_el)?,
            "issuedByThisCAAssertion" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "issuedByThisCAAssertion",
                    ));
                }
                Ok(_validate_CertificateExactAssertion(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod certificatePairMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = CertificatePairAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_CertificatePairAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_CertificatePairAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificatePairAssertion(el)
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
impl TryFrom<&X690Element> for CertificatePairAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificatePairAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificatePairAssertion,
        _eal_components_for_CertificatePairAssertion,
        _rctl2_components_for_CertificatePairAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuedToThisCAAssertion_: OPTIONAL<CertificateAssertion> = None;
    let mut issuedByThisCAAssertion_: OPTIONAL<CertificateAssertion> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuedToThisCAAssertion" => {
                issuedToThisCAAssertion_ = Some(_decode_CertificateAssertion(_el)?)
            }
            "issuedByThisCAAssertion" => {
                issuedByThisCAAssertion_ = Some(_decode_CertificateAssertion(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertificatePairAssertion {
        issuedToThisCAAssertion: issuedToThisCAAssertion_,
        issuedByThisCAAssertion: issuedByThisCAAssertion_,
        _unrecognized,
    })
}

pub fn _encode_CertificatePairAssertion(
    value_: &CertificatePairAssertion,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.issuedToThisCAAssertion {
        components_.push(|v_1: &CertificateAssertion| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertificateAssertion(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.issuedByThisCAAssertion {
        components_.push(|v_1: &CertificateAssertion| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertificateAssertion(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertificatePairAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificatePairAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificatePairAssertion,
        _eal_components_for_CertificatePairAssertion,
        _rctl2_components_for_CertificatePairAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuedToThisCAAssertion" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "issuedToThisCAAssertion",
                    ));
                }
                Ok(_validate_CertificateAssertion(&el)?)
            }(_el)?,
            "issuedByThisCAAssertion" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "issuedByThisCAAssertion",
                    ));
                }
                Ok(_validate_CertificateAssertion(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod certificateListExactMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = CertificateListExactAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_CertificateListExactAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_CertificateListExactAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateListExactAssertion(el)
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
impl TryFrom<&X690Element> for CertificateListExactAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateListExactAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateListExactAssertion,
        _eal_components_for_CertificateListExactAssertion,
        _rctl2_components_for_CertificateListExactAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut thisUpdate_: OPTIONAL<Time> = None;
    let mut distributionPoint_: OPTIONAL<DistributionPointName> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => issuer_ = Some(_decode_Name(_el)?),
            "thisUpdate" => thisUpdate_ = Some(_decode_Time(_el)?),
            "distributionPoint" => distributionPoint_ = Some(_decode_DistributionPointName(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "CertificateListExactAssertion",
                ))
            }
        }
    }
    Ok(CertificateListExactAssertion {
        issuer: issuer_.unwrap(),
        thisUpdate: thisUpdate_.unwrap(),
        distributionPoint: distributionPoint_,
    })
}

pub fn _encode_CertificateListExactAssertion(
    value_: &CertificateListExactAssertion,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_Name(&value_.issuer)?);
    components_.push(_encode_Time(&value_.thisUpdate)?);
    if let Some(v_) = &value_.distributionPoint {
        components_.push(_encode_DistributionPointName(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_CertificateListExactAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateListExactAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateListExactAssertion,
        _eal_components_for_CertificateListExactAssertion,
        _rctl2_components_for_CertificateListExactAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => _validate_Name(_el)?,
            "thisUpdate" => _validate_Time(_el)?,
            "distributionPoint" => _validate_DistributionPointName(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "CertificateListExactAssertion",
                ))
            }
        }
    }
    Ok(())
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

pub mod certificateListMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = CertificateListAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_CertificateListAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_CertificateListAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateListAssertion(el)
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
impl TryFrom<&X690Element> for CertificateListAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateListAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateListAssertion,
        _eal_components_for_CertificateListAssertion,
        _rctl2_components_for_CertificateListAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut minCRLNumber_: OPTIONAL<CRLNumber> = None;
    let mut maxCRLNumber_: OPTIONAL<CRLNumber> = None;
    let mut reasonFlags_: OPTIONAL<ReasonFlags> = None;
    let mut dateAndTime_: OPTIONAL<Time> = None;
    let mut distributionPoint_: OPTIONAL<DistributionPointName> = None;
    let mut authorityKeyIdentifier_: OPTIONAL<AuthorityKeyIdentifier> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => issuer_ = Some(_decode_Name(_el)?),
            "minCRLNumber" => minCRLNumber_ = Some(_decode_CRLNumber(_el)?),
            "maxCRLNumber" => maxCRLNumber_ = Some(_decode_CRLNumber(_el)?),
            "reasonFlags" => reasonFlags_ = Some(_decode_ReasonFlags(_el)?),
            "dateAndTime" => dateAndTime_ = Some(_decode_Time(_el)?),
            "distributionPoint" => {
                distributionPoint_ = Some(|el: &X690Element| -> ASN1Result<DistributionPointName> {
                    Ok(_decode_DistributionPointName(&el.inner()?)?)
                }(_el)?)
            }
            "authorityKeyIdentifier" => {
                authorityKeyIdentifier_ = Some(_decode_AuthorityKeyIdentifier(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertificateListAssertion {
        issuer: issuer_,
        minCRLNumber: minCRLNumber_,
        maxCRLNumber: maxCRLNumber_,
        reasonFlags: reasonFlags_,
        dateAndTime: dateAndTime_,
        distributionPoint: distributionPoint_,
        authorityKeyIdentifier: authorityKeyIdentifier_,
        _unrecognized,
    })
}

pub fn _encode_CertificateListAssertion(
    value_: &CertificateListAssertion,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(17);
    if let Some(v_) = &value_.issuer {
        components_.push(_encode_Name(&v_)?);
    }
    if let Some(v_) = &value_.minCRLNumber {
        components_.push(|v_1: &CRLNumber| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CRLNumber(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.maxCRLNumber {
        components_.push(|v_1: &CRLNumber| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CRLNumber(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
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
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(&_encode_DistributionPointName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.authorityKeyIdentifier {
        components_.push(|v_1: &AuthorityKeyIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AuthorityKeyIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertificateListAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateListAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateListAssertion,
        _eal_components_for_CertificateListAssertion,
        _rctl2_components_for_CertificateListAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => _validate_Name(_el)?,
            "minCRLNumber" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "minCRLNumber")
                    );
                }
                Ok(_validate_CRLNumber(&el)?)
            }(_el)?,
            "maxCRLNumber" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "maxCRLNumber")
                    );
                }
                Ok(_validate_CRLNumber(&el)?)
            }(_el)?,
            "reasonFlags" => _validate_ReasonFlags(_el)?,
            "dateAndTime" => _validate_Time(_el)?,
            "distributionPoint" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "distributionPoint",
                    ));
                }
                Ok(_validate_DistributionPointName(&el.inner()?)?)
            }(_el)?,
            "authorityKeyIdentifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "authorityKeyIdentifier",
                    ));
                }
                Ok(_validate_AuthorityKeyIdentifier(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod algorithmIdentifierMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = AlgorithmIdentifier; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_AlgorithmIdentifier(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_AlgorithmIdentifier(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_AlgorithmIdentifier(el)
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

pub mod policyMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = PolicyID; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_PolicyID(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_PolicyID(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_PolicyID(el)
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

pub mod pkiPathMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = PkiPathMatchSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_PkiPathMatchSyntax(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_PkiPathMatchSyntax(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_PkiPathMatchSyntax(el)
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
impl TryFrom<&X690Element> for PkiPathMatchSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PkiPathMatchSyntax")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PkiPathMatchSyntax,
        _eal_components_for_PkiPathMatchSyntax,
        _rctl2_components_for_PkiPathMatchSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut firstIssuer_: OPTIONAL<Name> = None;
    let mut lastSubject_: OPTIONAL<Name> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "firstIssuer" => firstIssuer_ = Some(_decode_Name(_el)?),
            "lastSubject" => lastSubject_ = Some(_decode_Name(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PkiPathMatchSyntax {
        firstIssuer: firstIssuer_.unwrap(),
        lastSubject: lastSubject_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_PkiPathMatchSyntax(value_: &PkiPathMatchSyntax) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_Name(&value_.firstIssuer)?);
    components_.push(_encode_Name(&value_.lastSubject)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_PkiPathMatchSyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PkiPathMatchSyntax")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PkiPathMatchSyntax,
        _eal_components_for_PkiPathMatchSyntax,
        _rctl2_components_for_PkiPathMatchSyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "firstIssuer" => _validate_Name(_el)?,
            "lastSubject" => _validate_Name(_el)?,
            _ => (),
        }
    }
    Ok(())
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

pub mod enhancedCertificateMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = EnhancedCertificateAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_EnhancedCertificateAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_EnhancedCertificateAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_EnhancedCertificateAssertion(el)
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
impl TryFrom<&X690Element> for EnhancedCertificateAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "EnhancedCertificateAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EnhancedCertificateAssertion,
        _eal_components_for_EnhancedCertificateAssertion,
        _rctl2_components_for_EnhancedCertificateAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut subjectKeyIdentifier_: OPTIONAL<SubjectKeyIdentifier> = None;
    let mut authorityKeyIdentifier_: OPTIONAL<AuthorityKeyIdentifier> = None;
    let mut certificateValid_: OPTIONAL<Time> = None;
    let mut privateKeyValid_: OPTIONAL<GeneralizedTime> = None;
    let mut subjectPublicKeyAlgID_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut keyUsage_: OPTIONAL<KeyUsage> = None;
    let mut subjectAltName_: OPTIONAL<AltName> = None;
    let mut policy_: OPTIONAL<CertPolicySet> = None;
    let mut pathToName_: OPTIONAL<GeneralNames> = None;
    let mut subject_: OPTIONAL<Name> = None;
    let mut nameConstraints_: OPTIONAL<NameConstraintsSyntax> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            "issuer" => {
                issuer_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                    Ok(_decode_Name(&el.inner()?)?)
                }(_el)?)
            }
            "subjectKeyIdentifier" => {
                subjectKeyIdentifier_ = Some(_decode_SubjectKeyIdentifier(_el)?)
            }
            "authorityKeyIdentifier" => {
                authorityKeyIdentifier_ = Some(_decode_AuthorityKeyIdentifier(_el)?)
            }
            "certificateValid" => {
                certificateValid_ = Some(|el: &X690Element| -> ASN1Result<Time> {
                    Ok(_decode_Time(&el.inner()?)?)
                }(_el)?)
            }
            "privateKeyValid" => privateKeyValid_ = Some(BER.decode_generalized_time(_el)?),
            "subjectPublicKeyAlgID" => {
                subjectPublicKeyAlgID_ = Some(BER.decode_object_identifier(_el)?)
            }
            "keyUsage" => keyUsage_ = Some(_decode_KeyUsage(_el)?),
            "subjectAltName" => subjectAltName_ = Some(_decode_AltName(_el)?),
            "policy" => policy_ = Some(_decode_CertPolicySet(_el)?),
            "pathToName" => pathToName_ = Some(_decode_GeneralNames(_el)?),
            "subject" => {
                subject_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                    Ok(_decode_Name(&el.inner()?)?)
                }(_el)?)
            }
            "nameConstraints" => nameConstraints_ = Some(_decode_NameConstraintsSyntax(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(EnhancedCertificateAssertion {
        serialNumber: serialNumber_,
        issuer: issuer_,
        subjectKeyIdentifier: subjectKeyIdentifier_,
        authorityKeyIdentifier: authorityKeyIdentifier_,
        certificateValid: certificateValid_,
        privateKeyValid: privateKeyValid_,
        subjectPublicKeyAlgID: subjectPublicKeyAlgID_,
        keyUsage: keyUsage_,
        subjectAltName: subjectAltName_,
        policy: policy_,
        pathToName: pathToName_,
        subject: subject_,
        nameConstraints: nameConstraints_,
        _unrecognized,
    })
}

pub fn _encode_EnhancedCertificateAssertion(
    value_: &EnhancedCertificateAssertion,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(23);
    if let Some(v_) = &value_.serialNumber {
        components_.push(|v_1: &CertificateSerialNumber| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertificateSerialNumber(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.issuer {
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_Name(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.subjectKeyIdentifier {
        components_.push(|v_1: &SubjectKeyIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_SubjectKeyIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.authorityKeyIdentifier {
        components_.push(|v_1: &AuthorityKeyIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AuthorityKeyIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.certificateValid {
        components_.push(|v_1: &Time| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::from_explicit(&_encode_Time(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.privateKeyValid {
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_generalized_time(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 5;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.subjectPublicKeyAlgID {
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_object_identifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 6;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.keyUsage {
        components_.push(|v_1: &KeyUsage| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_KeyUsage(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 7;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.subjectAltName {
        components_.push(|v_1: &AltName| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AltName(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 8;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.policy {
        components_.push(|v_1: &CertPolicySet| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertPolicySet(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 9;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.pathToName {
        components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralNames(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 10;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.subject {
        components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 11),
                X690Value::from_explicit(&_encode_Name(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.nameConstraints {
        components_.push(|v_1: &NameConstraintsSyntax| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_NameConstraintsSyntax(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 12;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_EnhancedCertificateAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "EnhancedCertificateAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EnhancedCertificateAssertion,
        _eal_components_for_EnhancedCertificateAssertion,
        _rctl2_components_for_EnhancedCertificateAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serialNumber" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "serialNumber")
                    );
                }
                Ok(_validate_CertificateSerialNumber(&el)?)
            }(_el)?,
            "issuer" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "issuer"));
                }
                Ok(_validate_Name(&el.inner()?)?)
            }(_el)?,
            "subjectKeyIdentifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "subjectKeyIdentifier",
                    ));
                }
                Ok(_validate_SubjectKeyIdentifier(&el)?)
            }(_el)?,
            "authorityKeyIdentifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "authorityKeyIdentifier",
                    ));
                }
                Ok(_validate_AuthorityKeyIdentifier(&el)?)
            }(_el)?,
            "certificateValid" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "certificateValid",
                    ));
                }
                Ok(_validate_Time(&el.inner()?)?)
            }(_el)?,
            "privateKeyValid" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "privateKeyValid",
                    ));
                }
                Ok(BER.validate_generalized_time(&el)?)
            }(_el)?,
            "subjectPublicKeyAlgID" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "subjectPublicKeyAlgID",
                    ));
                }
                Ok(BER.validate_object_identifier(&el)?)
            }(_el)?,
            "keyUsage" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "keyUsage")
                    );
                }
                Ok(_validate_KeyUsage(&el)?)
            }(_el)?,
            "subjectAltName" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 8 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "subjectAltName")
                    );
                }
                Ok(_validate_AltName(&el)?)
            }(_el)?,
            "policy" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 9 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "policy"));
                }
                Ok(_validate_CertPolicySet(&el)?)
            }(_el)?,
            "pathToName" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 10 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "pathToName")
                    );
                }
                Ok(_validate_GeneralNames(&el)?)
            }(_el)?,
            "subject" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 11 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "subject")
                    );
                }
                Ok(_validate_Name(&el.inner()?)?)
            }(_el)?,
            "nameConstraints" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 12 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "nameConstraints",
                    ));
                }
                Ok(_validate_NameConstraintsSyntax(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AltName ::= SEQUENCE {
///   altnameType   AltNameType,
///   altNameValue  GeneralName OPTIONAL }
/// ```
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
impl TryFrom<&X690Element> for AltName {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AltName")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AltName,
        _eal_components_for_AltName,
        _rctl2_components_for_AltName,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut altnameType_: OPTIONAL<AltNameType> = None;
    let mut altNameValue_: OPTIONAL<GeneralName> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "altnameType" => altnameType_ = Some(_decode_AltNameType(_el)?),
            "altNameValue" => altNameValue_ = Some(_decode_GeneralName(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AltName")),
        }
    }
    Ok(AltName {
        altnameType: altnameType_.unwrap(),
        altNameValue: altNameValue_,
    })
}

pub fn _encode_AltName(value_: &AltName) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_AltNameType(&value_.altnameType)?);
    if let Some(v_) = &value_.altNameValue {
        components_.push(_encode_GeneralName(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_AltName(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AltName")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AltName,
        _eal_components_for_AltName,
        _rctl2_components_for_AltName,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "altnameType" => _validate_AltNameType(_el)?,
            "altNameValue" => _validate_GeneralName(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AltName")),
        }
    }
    Ok(())
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

pub mod certExactAssertion {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateExactAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateExactAssertion(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateExactAssertion(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateExactAssertion(el)
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

pub mod certAssertion {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateAssertion(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateAssertion(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateAssertion(el)
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

pub mod certPairExactAssertion {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificatePairExactAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificatePairExactAssertion(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificatePairExactAssertion(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificatePairExactAssertion(el)
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

pub mod certPairAssertion {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificatePairAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificatePairAssertion(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificatePairAssertion(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificatePairAssertion(el)
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

pub mod certListExactAssertion {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateListExactAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateListExactAssertion(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateListExactAssertion(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateListExactAssertion(el)
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

pub mod certListAssertion {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateListAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateListAssertion(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateListAssertion(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateListAssertion(el)
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

pub mod algorithmIdentifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AlgorithmIdentifier; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AlgorithmIdentifier(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AlgorithmIdentifier(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AlgorithmIdentifier(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-subjectDirectoryAttributes         OBJECT IDENTIFIER ::= {id-ce 9}
/// ```
///
///
#[inline]
pub fn id_ce_subjectDirectoryAttributes () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 9).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-subjectKeyIdentifier               OBJECT IDENTIFIER ::= {id-ce 14}
/// ```
///
///
#[inline]
pub fn id_ce_subjectKeyIdentifier () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 14).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-keyUsage                           OBJECT IDENTIFIER ::= {id-ce 15}
/// ```
///
///
#[inline]
pub fn id_ce_keyUsage () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 15).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-privateKeyUsagePeriod              OBJECT IDENTIFIER ::= {id-ce 16}
/// ```
///
///
#[inline]
pub fn id_ce_privateKeyUsagePeriod () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 16).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-subjectAltName                     OBJECT IDENTIFIER ::= {id-ce 17}
/// ```
///
///
#[inline]
pub fn id_ce_subjectAltName () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 17).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-issuerAltName                      OBJECT IDENTIFIER ::= {id-ce 18}
/// ```
///
///
#[inline]
pub fn id_ce_issuerAltName () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 18).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-basicConstraints                   OBJECT IDENTIFIER ::= {id-ce 19}
/// ```
///
///
#[inline]
pub fn id_ce_basicConstraints () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 19).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-cRLNumber                          OBJECT IDENTIFIER ::= {id-ce 20}
/// ```
///
///
#[inline]
pub fn id_ce_cRLNumber () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 20).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-reasonCode                         OBJECT IDENTIFIER ::= {id-ce 21}
/// ```
///
///
#[inline]
pub fn id_ce_reasonCode () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 21).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-holdInstructionCode                OBJECT IDENTIFIER ::= {id-ce 23}
/// ```
///
///
#[inline]
pub fn id_ce_holdInstructionCode () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 23).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-invalidityDate                     OBJECT IDENTIFIER ::= {id-ce 24}
/// ```
///
///
#[inline]
pub fn id_ce_invalidityDate () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 24).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-deltaCRLIndicator                  OBJECT IDENTIFIER ::= {id-ce 27}
/// ```
///
///
#[inline]
pub fn id_ce_deltaCRLIndicator () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 27).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-issuingDistributionPoint           OBJECT IDENTIFIER ::= {id-ce 28}
/// ```
///
///
#[inline]
pub fn id_ce_issuingDistributionPoint () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 28).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-certificateIssuer                  OBJECT IDENTIFIER ::= {id-ce 29}
/// ```
///
///
#[inline]
pub fn id_ce_certificateIssuer () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 29).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-nameConstraints                    OBJECT IDENTIFIER ::= {id-ce 30}
/// ```
///
///
#[inline]
pub fn id_ce_nameConstraints () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 30).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-cRLDistributionPoints              OBJECT IDENTIFIER ::= {id-ce 31}
/// ```
///
///
#[inline]
pub fn id_ce_cRLDistributionPoints () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 31).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-certificatePolicies                OBJECT IDENTIFIER ::= {id-ce 32}
/// ```
///
///
#[inline]
pub fn id_ce_certificatePolicies () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 32).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-policyMappings                     OBJECT IDENTIFIER ::= {id-ce 33}
/// ```
///
///
#[inline]
pub fn id_ce_policyMappings () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 33).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-authorityKeyIdentifier             OBJECT IDENTIFIER ::= {id-ce 35}
/// ```
///
///
#[inline]
pub fn id_ce_authorityKeyIdentifier () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 35).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-policyConstraints                  OBJECT IDENTIFIER ::= {id-ce 36}
/// ```
///
///
#[inline]
pub fn id_ce_policyConstraints () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 36).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-extKeyUsage                        OBJECT IDENTIFIER ::= {id-ce 37}
/// ```
///
///
#[inline]
pub fn id_ce_extKeyUsage () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 37).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-cRLStreamIdentifier                OBJECT IDENTIFIER ::= {id-ce 40}
/// ```
///
///
#[inline]
pub fn id_ce_cRLStreamIdentifier () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 40).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-cRLScope                           OBJECT IDENTIFIER ::= {id-ce 44}
/// ```
///
///
#[inline]
pub fn id_ce_cRLScope () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 44).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-statusReferrals                    OBJECT IDENTIFIER ::= {id-ce 45}
/// ```
///
///
#[inline]
pub fn id_ce_statusReferrals () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 45).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-freshestCRL                        OBJECT IDENTIFIER ::= {id-ce 46}
/// ```
///
///
#[inline]
pub fn id_ce_freshestCRL () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 46).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-orderedList                        OBJECT IDENTIFIER ::= {id-ce 47}
/// ```
///
///
#[inline]
pub fn id_ce_orderedList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 47).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-baseUpdateTime                     OBJECT IDENTIFIER ::= {id-ce 51}
/// ```
///
///
#[inline]
pub fn id_ce_baseUpdateTime () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 51).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-deltaInfo                          OBJECT IDENTIFIER ::= {id-ce 53}
/// ```
///
///
#[inline]
pub fn id_ce_deltaInfo () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 53).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-inhibitAnyPolicy                   OBJECT IDENTIFIER ::= {id-ce 54}
/// ```
///
///
#[inline]
pub fn id_ce_inhibitAnyPolicy () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 54).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-toBeRevoked                        OBJECT IDENTIFIER ::= {id-ce 58}
/// ```
///
///
#[inline]
pub fn id_ce_toBeRevoked () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 58).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-revokedGroups                      OBJECT IDENTIFIER ::= {id-ce 59}
/// ```
///
///
#[inline]
pub fn id_ce_revokedGroups () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 59).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-expiredCertsOnCRL                  OBJECT IDENTIFIER ::= {id-ce 60}
/// ```
///
///
#[inline]
pub fn id_ce_expiredCertsOnCRL () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 60).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-aAissuingDistributionPoint         OBJECT IDENTIFIER ::= {id-ce 63}
/// ```
///
///
#[inline]
pub fn id_ce_aAissuingDistributionPoint () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 63).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-authorizationValidation            OBJECT IDENTIFIER ::= {id-ce 70}
/// ```
///
///
#[inline]
pub fn id_ce_authorizationValidation () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 70).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-protRestrict                       OBJECT IDENTIFIER ::= {id-ce 71}
/// ```
///
///
#[inline]
pub fn id_ce_protRestrict () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 71).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-subjectAltPublicKeyInfo            OBJECT IDENTIFIER ::= {id-ce 72}
/// ```
///
///
#[inline]
pub fn id_ce_subjectAltPublicKeyInfo () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 72).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-altSignatureAlgorithm              OBJECT IDENTIFIER ::= {id-ce 73}
/// ```
///
///
#[inline]
pub fn id_ce_altSignatureAlgorithm () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 73).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-altSignatureValue                  OBJECT IDENTIFIER ::= {id-ce 74}
/// ```
///
///
#[inline]
pub fn id_ce_altSignatureValue () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 74).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-associatedInformation              OBJECT IDENTIFIER ::= {id-ce 75}
/// ```
///
///
#[inline]
pub fn id_ce_associatedInformation () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ce(), 75).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificateExactMatch       OBJECT IDENTIFIER ::= {id-mr 34}
/// ```
///
///
#[inline]
pub fn id_mr_certificateExactMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 34).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificateMatch            OBJECT IDENTIFIER ::= {id-mr 35}
/// ```
///
///
#[inline]
pub fn id_mr_certificateMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 35).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificatePairExactMatch   OBJECT IDENTIFIER ::= {id-mr 36}
/// ```
///
///
#[inline]
pub fn id_mr_certificatePairExactMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 36).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificatePairMatch        OBJECT IDENTIFIER ::= {id-mr 37}
/// ```
///
///
#[inline]
pub fn id_mr_certificatePairMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 37).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificateListExactMatch   OBJECT IDENTIFIER ::= {id-mr 38}
/// ```
///
///
#[inline]
pub fn id_mr_certificateListExactMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 38).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-certificateListMatch        OBJECT IDENTIFIER ::= {id-mr 39}
/// ```
///
///
#[inline]
pub fn id_mr_certificateListMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 39).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-algorithmIdentifierMatch    OBJECT IDENTIFIER ::= {id-mr 40}
/// ```
///
///
#[inline]
pub fn id_mr_algorithmIdentifierMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 40).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-policyMatch                 OBJECT IDENTIFIER ::= {id-mr 60}
/// ```
///
///
#[inline]
pub fn id_mr_policyMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 60).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-pkiPathMatch                OBJECT IDENTIFIER ::= {id-mr 62}
/// ```
///
///
#[inline]
pub fn id_mr_pkiPathMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 62).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-enhancedCertificateMatch    OBJECT IDENTIFIER ::= {id-mr 65}
/// ```
///
///
#[inline]
pub fn id_mr_enhancedCertificateMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 65).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certExactAssertion         OBJECT IDENTIFIER ::= {id-ldx 1}
/// ```
///
///
#[inline]
pub fn id_ldx_certExactAssertion () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ldx(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certAssertion              OBJECT IDENTIFIER ::= {id-ldx 2}
/// ```
///
///
#[inline]
pub fn id_ldx_certAssertion () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ldx(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certPairExactAssertion     OBJECT IDENTIFIER ::= {id-ldx 3}
/// ```
///
///
#[inline]
pub fn id_ldx_certPairExactAssertion () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ldx(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certPairAssertion          OBJECT IDENTIFIER ::= {id-ldx 4}
/// ```
///
///
#[inline]
pub fn id_ldx_certPairAssertion () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ldx(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certListExactAssertion     OBJECT IDENTIFIER ::= {id-ldx 5}
/// ```
///
///
#[inline]
pub fn id_ldx_certListExactAssertion () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ldx(), 5).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-certListAssertion          OBJECT IDENTIFIER ::= {id-ldx 6}
/// ```
///
///
#[inline]
pub fn id_ldx_certListAssertion () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ldx(), 6).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx-algorithmIdentifier        OBJECT IDENTIFIER ::= {id-ldx 7}
/// ```
///
///
#[inline]
pub fn id_ldx_algorithmIdentifier () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ldx(), 7).unwrap() // OID_GETTER
}
/// ### ASN.1 Definition:
///
/// ```asn1
/// PolicyMappingsSyntax-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for PolicyMappingsSyntax_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "PolicyMappingsSyntax-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PolicyMappingsSyntax_Item,
        _eal_components_for_PolicyMappingsSyntax_Item,
        _rctl2_components_for_PolicyMappingsSyntax_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuerDomainPolicy_: OPTIONAL<CertPolicyId> = None;
    let mut subjectDomainPolicy_: OPTIONAL<CertPolicyId> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuerDomainPolicy" => issuerDomainPolicy_ = Some(_decode_CertPolicyId(_el)?),
            "subjectDomainPolicy" => subjectDomainPolicy_ = Some(_decode_CertPolicyId(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PolicyMappingsSyntax_Item {
        issuerDomainPolicy: issuerDomainPolicy_.unwrap(),
        subjectDomainPolicy: subjectDomainPolicy_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_PolicyMappingsSyntax_Item(
    value_: &PolicyMappingsSyntax_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_CertPolicyId(&value_.issuerDomainPolicy)?);
    components_.push(_encode_CertPolicyId(&value_.subjectDomainPolicy)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_PolicyMappingsSyntax_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "PolicyMappingsSyntax-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PolicyMappingsSyntax_Item,
        _eal_components_for_PolicyMappingsSyntax_Item,
        _rctl2_components_for_PolicyMappingsSyntax_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuerDomainPolicy" => _validate_CertPolicyId(_el)?,
            "subjectDomainPolicy" => _validate_CertPolicyId(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    BER.decode_enumerated(&el)
}

pub fn _encode_AltNameType_builtinNameForm(
    value_: &AltNameType_builtinNameForm,
) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_AltNameType_builtinNameForm(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}
