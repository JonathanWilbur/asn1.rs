#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # AuthenticationFramework
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `AuthenticationFramework`.
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
use crate::CertificateExtensions::*;
use crate::InformationFramework::*;
pub use crate::PKI_Stub::{
    FingerPrint, PKCertIdentifier, ScopeRestrictions, _decode_FingerPrint, _decode_HASH,
    _decode_PKCertIdentifier, _decode_SIGNED, _decode_ScopeRestrictions, _encode_FingerPrint,
    _encode_HASH, _encode_PKCertIdentifier, _encode_SIGNED, _encode_ScopeRestrictions,
    _validate_FingerPrint, _validate_HASH, _validate_PKCertIdentifier, _validate_SIGNED,
    _validate_ScopeRestrictions, HASH, SIGNED,
};
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use wildboar_asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// SIGNATURE ::= SEQUENCE {
///   algorithmIdentifier  AlgorithmIdentifier{{SupportedAlgorithms}},
///   signature            BIT STRING,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct SIGNATURE {
    pub algorithmIdentifier: AlgorithmIdentifier,
    pub signature: BIT_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl SIGNATURE {
    pub fn new(
        algorithmIdentifier: AlgorithmIdentifier,
        signature: BIT_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SIGNATURE {
            algorithmIdentifier,
            signature,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for SIGNATURE {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SIGNATURE(el)
    }
}

pub const _rctl1_components_for_SIGNATURE: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithmIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "signature",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SIGNATURE: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SIGNATURE: &[ComponentSpec; 0] = &[];

pub fn _decode_SIGNATURE(el: &X690Element) -> ASN1Result<SIGNATURE> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SIGNATURE")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SIGNATURE,
        _eal_components_for_SIGNATURE,
        _rctl2_components_for_SIGNATURE,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithmIdentifier_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut signature_: OPTIONAL<BIT_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => algorithmIdentifier_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "signature" => signature_ = Some(BER.decode_bit_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(SIGNATURE {
        algorithmIdentifier: algorithmIdentifier_.unwrap(),
        signature: signature_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_SIGNATURE(value_: &SIGNATURE) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
    components_.push(BER.encode_bit_string(&value_.signature)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_SIGNATURE(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SIGNATURE")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SIGNATURE,
        _eal_components_for_SIGNATURE,
        _rctl2_components_for_SIGNATURE,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => _validate_AlgorithmIdentifier(_el)?,
            "signature" => BER.validate_bit_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ENCRYPTED{ToBeEnciphered}  ::=  BIT STRING (CONSTRAINED BY {
///    -- shall be the result of applying an encipherment procedure
///    -- to the BER-encoded octets of a value of -- ToBeEnciphered } )
/// ```
pub type ENCRYPTED = BIT_STRING;

pub fn _decode_ENCRYPTED(el: &X690Element) -> ASN1Result<ENCRYPTED> {
    BER.decode_bit_string(&el)
}

pub fn _encode_ENCRYPTED(value_: &ENCRYPTED) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_ENCRYPTED(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ENCRYPTED-HASH{ToBeSigned}  ::=  BIT STRING (CONSTRAINED BY {
///   -- shall be the result of applying a hashing procedure to the DER-encoded (see 6.2)
///   -- octets of a value of -- ToBeSigned -- and then applying an encipherment procedure
///   -- to those octets -- } )
/// ```
pub type ENCRYPTED_HASH = BIT_STRING;

pub fn _decode_ENCRYPTED_HASH(el: &X690Element) -> ASN1Result<ENCRYPTED_HASH> {
    BER.decode_bit_string(&el)
}

pub fn _encode_ENCRYPTED_HASH(value_: &ENCRYPTED_HASH) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_ENCRYPTED_HASH(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ALGORITHM ::= CLASS {
///   &Type          OPTIONAL,
///   &id            OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   [PARMS         &Type]
///   IDENTIFIED BY  &id }
/// ```
///
#[derive(Debug)]
pub struct ALGORITHM {
    pub id: OBJECT_IDENTIFIER,
}
impl ALGORITHM {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AlgorithmIdentifier{ALGORITHM:SupportedAlgorithms} ::= SEQUENCE {
///   algorithm       ALGORITHM.&id({SupportedAlgorithms}),
///   parameters      ALGORITHM.&Type({SupportedAlgorithms}{@algorithm}) OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct AlgorithmIdentifier {
    pub algorithm: OBJECT_IDENTIFIER,
    pub parameters: OPTIONAL<X690Element>,
    pub _unrecognized: Vec<X690Element>,
}
impl AlgorithmIdentifier {
    pub fn new(
        algorithm: OBJECT_IDENTIFIER,
        parameters: OPTIONAL<X690Element>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AlgorithmIdentifier {
            algorithm,
            parameters,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for AlgorithmIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AlgorithmIdentifier(el)
    }
}

pub const _rctl1_components_for_AlgorithmIdentifier: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("parameters", true, TagSelector::any, None, None),
];

pub const _rctl2_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];

pub fn _decode_AlgorithmIdentifier(el: &X690Element) -> ASN1Result<AlgorithmIdentifier> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AlgorithmIdentifier")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AlgorithmIdentifier,
        _eal_components_for_AlgorithmIdentifier,
        _rctl2_components_for_AlgorithmIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithm_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut parameters_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => algorithm_ = Some(BER.decode_object_identifier(_el)?),
            "parameters" => parameters_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AlgorithmIdentifier {
        algorithm: algorithm_.unwrap(),
        parameters: parameters_,
        _unrecognized,
    })
}

pub fn _encode_AlgorithmIdentifier(value_: &AlgorithmIdentifier) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.algorithm)?);
    if let Some(v_) = &value_.parameters {
        components_.push(x690_identity(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AlgorithmIdentifier")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AlgorithmIdentifier,
        _eal_components_for_AlgorithmIdentifier,
        _rctl2_components_for_AlgorithmIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => BER.validate_object_identifier(_el)?,
            "parameters" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedAltAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedAltAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ecPublicKey ALGORITHM ::= {  -- copied IETF RFC 5480
///   PARMS       SupportedCurves
///   IDENTIFIED  BY id-ecPublicKey }
/// ```
///
///
pub fn ecPublicKey() -> ALGORITHM {
    ALGORITHM {
        id: id_ecPublicKey(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ecPublicKey {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SupportedCurves; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SupportedCurves(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SupportedCurves(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SupportedCurves(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ecPublicKey OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) ansi-X9-62(10045)
///                                       keyType(2) 1 }
/// ```
///
///
#[inline]
pub fn id_ecPublicKey () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* member-body */ 2,/* us */ 840,/* ansi-X9-62 */ 10045,/* keyType */ 2,1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedCurves OBJECT IDENTIFIER ::= {dummyCurv, ...}
/// ```
///
///
pub type SupportedCurves = OBJECT_IDENTIFIER; // VALUE_SET_TYPE

pub fn _decode_SupportedCurves(el: &X690Element) -> ASN1Result<SupportedCurves> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_SupportedCurves(value_: &SupportedCurves) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_SupportedCurves(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dummyCurv OBJECT IDENTIFIER ::= {2 5 5}
/// ```
///
///
#[inline]
pub fn dummyCurv () -> OBJECT_IDENTIFIER {
	oid!(2,5,5) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Certificate  ::=  SIGNED{TBSCertificate}
/// ```
pub type Certificate = SIGNED<TBSCertificate>; // DefinedType

pub fn _decode_Certificate(el: &X690Element) -> ASN1Result<Certificate> {
    _decode_SIGNED::<TBSCertificate>(_decode_TBSCertificate, el)
}

pub fn _encode_Certificate(value_: &Certificate) -> ASN1Result<X690Element> {
    _encode_SIGNED::<TBSCertificate>(_encode_TBSCertificate, value_)
}

pub fn _validate_Certificate(el: &X690Element) -> ASN1Result<()> {
    _validate_SIGNED::<TBSCertificate>(_validate_TBSCertificate, el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBSCertificate ::= SEQUENCE {
///   version                  [0]  Version DEFAULT v1,
///   serialNumber                  CertificateSerialNumber,
///   signature                     AlgorithmIdentifier{{SupportedAlgorithms}},
///   issuer                        Name,
///   validity                      Validity,
///   subject                       Name,
///   subjectPublicKeyInfo          SubjectPublicKeyInfo,
///   issuerUniqueIdentifier   [1] IMPLICIT UniqueIdentifier OPTIONAL,
///   ...,
///   [[2:  -- if present, version shall be v2 or v3
///   subjectUniqueIdentifier  [2] IMPLICIT UniqueIdentifier OPTIONAL]],
///   [[3:  -- if present, version shall be v2 or v3
///   extensions               [3]  Extensions OPTIONAL ]]
///   -- If present, version shall be v3]]
///  } (CONSTRAINED BY { -- shall be DER encoded -- } )
/// ```
///
#[derive(Debug, Clone)]
pub struct TBSCertificate {
    pub version: OPTIONAL<Version>,
    pub serialNumber: CertificateSerialNumber,
    pub signature: AlgorithmIdentifier,
    pub issuer: Name,
    pub validity: Validity,
    pub subject: Name,
    pub subjectPublicKeyInfo: SubjectPublicKeyInfo,
    pub issuerUniqueIdentifier: OPTIONAL<UniqueIdentifier>,
    pub _unrecognized: Vec<X690Element>,
}
impl TBSCertificate {
    pub fn new(
        version: OPTIONAL<Version>,
        serialNumber: CertificateSerialNumber,
        signature: AlgorithmIdentifier,
        issuer: Name,
        validity: Validity,
        subject: Name,
        subjectPublicKeyInfo: SubjectPublicKeyInfo,
        issuerUniqueIdentifier: OPTIONAL<UniqueIdentifier>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TBSCertificate {
            version,
            serialNumber,
            signature,
            issuer,
            validity,
            subject,
            subjectPublicKeyInfo,
            issuerUniqueIdentifier,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> Version {
        Version_v1
    }
}
impl TryFrom<&X690Element> for TBSCertificate {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TBSCertificate(el)
    }
}

pub const _rctl1_components_for_TBSCertificate: &[ComponentSpec; 8] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "signature",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new("issuer", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "validity",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new("subject", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "subjectPublicKeyInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuerUniqueIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TBSCertificate: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TBSCertificate: &[ComponentSpec; 0] = &[];

pub fn _decode_TBSCertificate(el: &X690Element) -> ASN1Result<TBSCertificate> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TBSCertificate"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSCertificate,
        _eal_components_for_TBSCertificate,
        _rctl2_components_for_TBSCertificate,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut signature_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut validity_: OPTIONAL<Validity> = None;
    let mut subject_: OPTIONAL<Name> = None;
    let mut subjectPublicKeyInfo_: OPTIONAL<SubjectPublicKeyInfo> = None;
    let mut issuerUniqueIdentifier_: OPTIONAL<UniqueIdentifier> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => {
                version_ = Some(|el: &X690Element| -> ASN1Result<Version> {
                    Ok(_decode_Version(&el.inner()?)?)
                }(_el)?)
            }
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            "signature" => signature_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "issuer" => issuer_ = Some(_decode_Name(_el)?),
            "validity" => validity_ = Some(_decode_Validity(_el)?),
            "subject" => subject_ = Some(_decode_Name(_el)?),
            "subjectPublicKeyInfo" => {
                subjectPublicKeyInfo_ = Some(_decode_SubjectPublicKeyInfo(_el)?)
            }
            "issuerUniqueIdentifier" => {
                issuerUniqueIdentifier_ = Some(_decode_UniqueIdentifier(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TBSCertificate {
        version: version_,
        serialNumber: serialNumber_.unwrap(),
        signature: signature_.unwrap(),
        issuer: issuer_.unwrap(),
        validity: validity_.unwrap(),
        subject: subject_.unwrap(),
        subjectPublicKeyInfo: subjectPublicKeyInfo_.unwrap(),
        issuerUniqueIdentifier: issuerUniqueIdentifier_,
        _unrecognized,
    })
}

pub fn _encode_TBSCertificate(value_: &TBSCertificate) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(20);
    if let Some(v_) = &value_.version {
        if *v_ != TBSCertificate::_default_value_for_version() {
            components_.push(|v_1: &Version| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&_encode_Version(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    components_.push(_encode_AlgorithmIdentifier(&value_.signature)?);
    components_.push(_encode_Name(&value_.issuer)?);
    components_.push(_encode_Validity(&value_.validity)?);
    components_.push(_encode_Name(&value_.subject)?);
    components_.push(_encode_SubjectPublicKeyInfo(&value_.subjectPublicKeyInfo)?);
    if let Some(v_) = &value_.issuerUniqueIdentifier {
        components_.push(|v_1: &UniqueIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_UniqueIdentifier(&v_1)?;
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

pub fn _validate_TBSCertificate(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TBSCertificate"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSCertificate,
        _eal_components_for_TBSCertificate,
        _rctl2_components_for_TBSCertificate,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "version")
                    );
                }
                Ok(_validate_Version(&el.inner()?)?)
            }(_el)?,
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            "signature" => _validate_AlgorithmIdentifier(_el)?,
            "issuer" => _validate_Name(_el)?,
            "validity" => _validate_Validity(_el)?,
            "subject" => _validate_Name(_el)?,
            "subjectPublicKeyInfo" => _validate_SubjectPublicKeyInfo(_el)?,
            "issuerUniqueIdentifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "issuerUniqueIdentifier",
                    ));
                }
                Ok(_validate_UniqueIdentifier(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Version  ::=  INTEGER {v1(0), v2(1), v3(2)}
/// ```
pub type Version = i8;

pub const Version_v1: i8 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const Version_v2: i8 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Version_v3: i8 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Version(el: &X690Element) -> ASN1Result<Version> {
    BER.decode_i8(el)
}

pub fn _encode_Version(value_: &Version) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Version(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateSerialNumber  ::=  INTEGER
/// ```
pub type CertificateSerialNumber = INTEGER;

pub fn _decode_CertificateSerialNumber(el: &X690Element) -> ASN1Result<CertificateSerialNumber> {
    BER.decode_integer(&el)
}

pub fn _encode_CertificateSerialNumber(
    value_: &CertificateSerialNumber,
) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_CertificateSerialNumber(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Validity ::= SEQUENCE {
///   notBefore  Time,
///   notAfter   Time,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Validity {
    pub notBefore: Time,
    pub notAfter: Time,
    pub _unrecognized: Vec<X690Element>,
}
impl Validity {
    pub fn new(notBefore: Time, notAfter: Time, _unrecognized: Vec<X690Element>) -> Self {
        Validity {
            notBefore,
            notAfter,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for Validity {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Validity(el)
    }
}

pub const _rctl1_components_for_Validity: &[ComponentSpec; 2] = &[
    ComponentSpec::new("notBefore", false, TagSelector::any, None, None),
    ComponentSpec::new("notAfter", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_Validity: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Validity: &[ComponentSpec; 0] = &[];

pub fn _decode_Validity(el: &X690Element) -> ASN1Result<Validity> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Validity")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Validity,
        _eal_components_for_Validity,
        _rctl2_components_for_Validity,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut notBefore_: OPTIONAL<Time> = None;
    let mut notAfter_: OPTIONAL<Time> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notBefore" => notBefore_ = Some(_decode_Time(_el)?),
            "notAfter" => notAfter_ = Some(_decode_Time(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Validity {
        notBefore: notBefore_.unwrap(),
        notAfter: notAfter_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_Validity(value_: &Validity) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_Time(&value_.notBefore)?);
    components_.push(_encode_Time(&value_.notAfter)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Validity(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Validity")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Validity,
        _eal_components_for_Validity,
        _rctl2_components_for_Validity,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notBefore" => _validate_Time(_el)?,
            "notAfter" => _validate_Time(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubjectPublicKeyInfo ::= SEQUENCE {
///   algorithm         AlgorithmIdentifier{{SupportedAlgorithms}},
///   subjectPublicKey  PublicKey,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct SubjectPublicKeyInfo {
    pub algorithm: AlgorithmIdentifier,
    pub subjectPublicKey: PublicKey,
    pub _unrecognized: Vec<X690Element>,
}
impl SubjectPublicKeyInfo {
    pub fn new(
        algorithm: AlgorithmIdentifier,
        subjectPublicKey: PublicKey,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SubjectPublicKeyInfo {
            algorithm,
            subjectPublicKey,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for SubjectPublicKeyInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SubjectPublicKeyInfo(el)
    }
}

pub const _rctl1_components_for_SubjectPublicKeyInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subjectPublicKey",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SubjectPublicKeyInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SubjectPublicKeyInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_SubjectPublicKeyInfo(el: &X690Element) -> ASN1Result<SubjectPublicKeyInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubjectPublicKeyInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SubjectPublicKeyInfo,
        _eal_components_for_SubjectPublicKeyInfo,
        _rctl2_components_for_SubjectPublicKeyInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithm_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut subjectPublicKey_: OPTIONAL<PublicKey> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => algorithm_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "subjectPublicKey" => subjectPublicKey_ = Some(_decode_PublicKey(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(SubjectPublicKeyInfo {
        algorithm: algorithm_.unwrap(),
        subjectPublicKey: subjectPublicKey_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_SubjectPublicKeyInfo(value_: &SubjectPublicKeyInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_AlgorithmIdentifier(&value_.algorithm)?);
    components_.push(_encode_PublicKey(&value_.subjectPublicKey)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_SubjectPublicKeyInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubjectPublicKeyInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SubjectPublicKeyInfo,
        _eal_components_for_SubjectPublicKeyInfo,
        _rctl2_components_for_SubjectPublicKeyInfo,
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
            "subjectPublicKey" => _validate_PublicKey(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PublicKey  ::=  BIT STRING
/// ```
pub type PublicKey = BIT_STRING;

pub fn _decode_PublicKey(el: &X690Element) -> ASN1Result<PublicKey> {
    BER.decode_bit_string(&el)
}

pub fn _encode_PublicKey(value_: &PublicKey) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_PublicKey(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Time  ::=  CHOICE {
///   utcTime          UTCTime,
///   generalizedTime  GeneralizedTime }
/// ```
#[derive(Debug, Clone)]
pub enum Time {
    utcTime(UTCTime),
    generalizedTime(GeneralizedTime),
}

impl TryFrom<&X690Element> for Time {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Time(el)
    }
}

pub fn _decode_Time(el: &X690Element) -> ASN1Result<Time> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 23) => Ok(Time::utcTime(BER.decode_utc_time(&el)?)),
        (TagClass::UNIVERSAL, 24) => Ok(Time::generalizedTime(BER.decode_generalized_time(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Time",
            ))
        }
    }
}

pub fn _encode_Time(value_: &Time) -> ASN1Result<X690Element> {
    match value_ {
        Time::utcTime(v) => BER.encode_utc_time(&v),
        Time::generalizedTime(v) => BER.encode_generalized_time(&v),
    }
}

pub fn _validate_Time(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 23) => BER.validate_utc_time(&el),
        (TagClass::UNIVERSAL, 24) => BER.validate_generalized_time(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Time",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Extensions  ::=  SEQUENCE SIZE (1..MAX) OF Extension
/// ```
pub type Extensions = Vec<Extension>; // SequenceOfType

pub fn _decode_Extensions(el: &X690Element) -> ASN1Result<Extensions> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Extensions")),
    };
    let mut items: SEQUENCE_OF<Extension> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Extension(el)?);
    }
    Ok(items)
}

pub fn _encode_Extensions(value_: &Extensions) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Extension(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Extensions(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Extension(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Extensions")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Extension ::= SEQUENCE {
///   extnId     EXTENSION.&id({ExtensionSet}),
///   critical   BOOLEAN DEFAULT FALSE,
///   extnValue  OCTET STRING
///     (CONTAINING EXTENSION.&ExtnType({ExtensionSet}{@extnId})
///        ENCODED BY der),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Extension {
    pub extnId: OBJECT_IDENTIFIER,
    pub critical: OPTIONAL<BOOLEAN>,
    pub extnValue: OCTET_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl Extension {
    pub fn new(
        extnId: OBJECT_IDENTIFIER,
        critical: OPTIONAL<BOOLEAN>,
        extnValue: OCTET_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Extension {
            extnId,
            critical,
            extnValue,
            _unrecognized,
        }
    }
    pub fn _default_value_for_critical() -> BOOLEAN {
        false
    }
}
impl TryFrom<&X690Element> for Extension {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Extension(el)
    }
}

pub const _rctl1_components_for_Extension: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "extnId",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "critical",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extnValue",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Extension: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Extension: &[ComponentSpec; 0] = &[];

pub fn _decode_Extension(el: &X690Element) -> ASN1Result<Extension> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Extension")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Extension,
        _eal_components_for_Extension,
        _rctl2_components_for_Extension,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut extnId_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut critical_: OPTIONAL<BOOLEAN> = None;
    let mut extnValue_: OPTIONAL<OCTET_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "extnId" => extnId_ = Some(BER.decode_object_identifier(_el)?),
            "critical" => critical_ = Some(BER.decode_boolean(_el)?),
            "extnValue" => extnValue_ = Some(BER.decode_octet_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Extension {
        extnId: extnId_.unwrap(),
        critical: critical_,
        extnValue: extnValue_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_Extension(value_: &Extension) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_object_identifier(&value_.extnId)?);
    if let Some(v_) = &value_.critical {
        if *v_ != Extension::_default_value_for_critical() {
            components_.push(BER.encode_boolean(&v_)?);
        }
    }
    components_.push(BER.encode_octet_string(&value_.extnValue)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Extension(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Extension")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Extension,
        _eal_components_for_Extension,
        _rctl2_components_for_Extension,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "extnId" => BER.validate_object_identifier(_el)?,
            "critical" => BER.validate_boolean(_el)?,
            "extnValue" => BER.validate_octet_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// der OBJECT IDENTIFIER ::= {joint-iso-itu-t asn1(1) ber-derived(2) distinguished-encoding(1)}
/// ```
///
///
#[inline]
pub fn der () -> OBJECT_IDENTIFIER {
	oid!(joint_iso_itu_t,/* asn1 */ 1,/* ber-derived */ 2,/* distinguished-encoding */ 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtensionSet EXTENSION ::= {...}
/// ```
///
///
pub fn ExtensionSet() -> Vec<EXTENSION> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EXTENSION ::= CLASS {
///   &id           OBJECT IDENTIFIER UNIQUE,
///   &ExtnType }
/// WITH SYNTAX {
///   SYNTAX        &ExtnType
///   IDENTIFIED BY &id }
/// ```
///
#[derive(Debug)]
pub struct EXTENSION {
    pub id: OBJECT_IDENTIFIER,
}
impl EXTENSION {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Certificates ::= SEQUENCE {
///   userCertificate    Certificate,
///   certificationPath  ForwardCertificationPath OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Certificates {
    pub userCertificate: Certificate,
    pub certificationPath: OPTIONAL<ForwardCertificationPath>,
    pub _unrecognized: Vec<X690Element>,
}
impl Certificates {
    pub fn new(
        userCertificate: Certificate,
        certificationPath: OPTIONAL<ForwardCertificationPath>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Certificates {
            userCertificate,
            certificationPath,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for Certificates {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Certificates(el)
    }
}

pub const _rctl1_components_for_Certificates: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "userCertificate",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certificationPath",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Certificates: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Certificates: &[ComponentSpec; 0] = &[];

pub fn _decode_Certificates(el: &X690Element) -> ASN1Result<Certificates> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Certificates")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Certificates,
        _eal_components_for_Certificates,
        _rctl2_components_for_Certificates,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut userCertificate_: OPTIONAL<Certificate> = None;
    let mut certificationPath_: OPTIONAL<ForwardCertificationPath> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "userCertificate" => userCertificate_ = Some(_decode_Certificate(_el)?),
            "certificationPath" => {
                certificationPath_ = Some(_decode_ForwardCertificationPath(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Certificates {
        userCertificate: userCertificate_.unwrap(),
        certificationPath: certificationPath_,
        _unrecognized,
    })
}

pub fn _encode_Certificates(value_: &Certificates) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_Certificate(&value_.userCertificate)?);
    if let Some(v_) = &value_.certificationPath {
        components_.push(_encode_ForwardCertificationPath(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Certificates(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Certificates")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Certificates,
        _eal_components_for_Certificates,
        _rctl2_components_for_Certificates,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "userCertificate" => _validate_Certificate(_el)?,
            "certificationPath" => _validate_ForwardCertificationPath(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ForwardCertificationPath  ::=  SEQUENCE SIZE (1..MAX) OF CrossCertificates
/// ```
pub type ForwardCertificationPath = Vec<CrossCertificates>; // SequenceOfType

pub fn _decode_ForwardCertificationPath(el: &X690Element) -> ASN1Result<ForwardCertificationPath> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ForwardCertificationPath",
            ))
        }
    };
    let mut items: SEQUENCE_OF<CrossCertificates> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_CrossCertificates(el)?);
    }
    Ok(items)
}

pub fn _encode_ForwardCertificationPath(
    value_: &ForwardCertificationPath,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_CrossCertificates(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_ForwardCertificationPath(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_CrossCertificates(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "ForwardCertificationPath",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CrossCertificates  ::=  SET SIZE (1..MAX) OF Certificate
/// ```
pub type CrossCertificates = Vec<Certificate>; // SetOfType

pub fn _decode_CrossCertificates(el: &X690Element) -> ASN1Result<CrossCertificates> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CrossCertificates")
            )
        }
    };
    let mut items: SET_OF<Certificate> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Certificate(el)?);
    }
    Ok(items)
}

pub fn _encode_CrossCertificates(value_: &CrossCertificates) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Certificate(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CrossCertificates(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Certificate(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CrossCertificates")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificationPath ::= SEQUENCE {
///   userCertificate    Certificate,
///   theCACertificates  SEQUENCE SIZE (1..MAX) OF CertificatePair OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertificationPath {
    pub userCertificate: Certificate,
    pub theCACertificates: OPTIONAL<Vec<CertificatePair>>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertificationPath {
    pub fn new(
        userCertificate: Certificate,
        theCACertificates: OPTIONAL<Vec<CertificatePair>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertificationPath {
            userCertificate,
            theCACertificates,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for CertificationPath {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertificationPath(el)
    }
}

pub const _rctl1_components_for_CertificationPath: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "userCertificate",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "theCACertificates",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificationPath: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertificationPath: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificationPath(el: &X690Element) -> ASN1Result<CertificationPath> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificationPath")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificationPath,
        _eal_components_for_CertificationPath,
        _rctl2_components_for_CertificationPath,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut userCertificate_: OPTIONAL<Certificate> = None;
    let mut theCACertificates_: OPTIONAL<Vec<CertificatePair>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "userCertificate" => userCertificate_ = Some(_decode_Certificate(_el)?),
            "theCACertificates" => {
                theCACertificates_ = Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertificatePair>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "theCACertificates",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<CertificatePair> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_CertificatePair(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertificationPath {
        userCertificate: userCertificate_.unwrap(),
        theCACertificates: theCACertificates_,
        _unrecognized,
    })
}

pub fn _encode_CertificationPath(value_: &CertificationPath) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_Certificate(&value_.userCertificate)?);
    if let Some(v_) = &value_.theCACertificates {
        components_.push(
            |value_: &SEQUENCE_OF<CertificatePair>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_CertificatePair(&v)?);
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

pub fn _validate_CertificationPath(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificationPath")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificationPath,
        _eal_components_for_CertificationPath,
        _rctl2_components_for_CertificationPath,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "userCertificate" => _validate_Certificate(_el)?,
            "theCACertificates" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_CertificatePair(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "theCACertificates",
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
/// PkiPath  ::=  SEQUENCE SIZE (1..MAX) OF Certificate
/// ```
pub type PkiPath = Vec<Certificate>; // SequenceOfType

pub fn _decode_PkiPath(el: &X690Element) -> ASN1Result<PkiPath> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PkiPath")),
    };
    let mut items: SEQUENCE_OF<Certificate> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Certificate(el)?);
    }
    Ok(items)
}

pub fn _encode_PkiPath(value_: &PkiPath) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Certificate(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_PkiPath(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Certificate(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PkiPath")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateList  ::=  SIGNED{CertificateListContent}
/// ```
pub type CertificateList = SIGNED<CertificateListContent>; // DefinedType

pub fn _decode_CertificateList(el: &X690Element) -> ASN1Result<CertificateList> {
    _decode_SIGNED::<CertificateListContent>(_decode_CertificateListContent, el)
}

pub fn _encode_CertificateList(value_: &CertificateList) -> ASN1Result<X690Element> {
    _encode_SIGNED::<CertificateListContent>(_encode_CertificateListContent, value_)
}

pub fn _validate_CertificateList(el: &X690Element) -> ASN1Result<()> {
    _validate_SIGNED::<CertificateListContent>(_validate_CertificateListContent, el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateListContent ::= SEQUENCE {
///   version              Version OPTIONAL,
///   -- if present, version shall be v2
///   signature            AlgorithmIdentifier{{SupportedAlgorithms}},
///   issuer               Name,
///   thisUpdate           Time,
///   nextUpdate           Time OPTIONAL,
///   revokedCertificates  SEQUENCE OF SEQUENCE {
///     serialNumber         CertificateSerialNumber,
///     revocationDate       Time,
///     crlEntryExtensions   Extensions OPTIONAL,
///     ...} OPTIONAL,
///   ...,
///   ...,
///   crlExtensions   [0]  Extensions OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertificateListContent {
    pub version: OPTIONAL<Version>,
    pub signature: AlgorithmIdentifier,
    pub issuer: Name,
    pub thisUpdate: Time,
    pub nextUpdate: OPTIONAL<Time>,
    pub revokedCertificates: OPTIONAL<Vec<CertificateListContent_revokedCertificates_Item>>,
    pub _unrecognized: Vec<X690Element>,
    pub crlExtensions: OPTIONAL<Extensions>,
}
impl CertificateListContent {
    pub fn new(
        version: OPTIONAL<Version>,
        signature: AlgorithmIdentifier,
        issuer: Name,
        thisUpdate: Time,
        nextUpdate: OPTIONAL<Time>,
        revokedCertificates: OPTIONAL<Vec<CertificateListContent_revokedCertificates_Item>>,
        _unrecognized: Vec<X690Element>,
        crlExtensions: OPTIONAL<Extensions>,
    ) -> Self {
        CertificateListContent {
            version,
            signature,
            issuer,
            thisUpdate,
            nextUpdate,
            revokedCertificates,
            crlExtensions,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for CertificateListContent {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateListContent(el)
    }
}

pub const _rctl1_components_for_CertificateListContent: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "signature",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new("issuer", false, TagSelector::any, None, None),
    ComponentSpec::new("thisUpdate", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "nextUpdate",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 23)),
            &TagSelector::tag((TagClass::UNIVERSAL, 24)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "revokedCertificates",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificateListContent: &[ComponentSpec; 1] =
    &[ComponentSpec::new(
        "crlExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    )];

pub const _eal_components_for_CertificateListContent: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificateListContent(el: &X690Element) -> ASN1Result<CertificateListContent> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateListContent",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateListContent,
        _eal_components_for_CertificateListContent,
        _rctl2_components_for_CertificateListContent,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut signature_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut thisUpdate_: OPTIONAL<Time> = None;
    let mut nextUpdate_: OPTIONAL<Time> = None;
    let mut revokedCertificates_: OPTIONAL<Vec<CertificateListContent_revokedCertificates_Item>> =
        None;
    let mut crlExtensions_: OPTIONAL<Extensions> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            "signature" => signature_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "issuer" => issuer_ = Some(_decode_Name(_el)?),
            "thisUpdate" => thisUpdate_ = Some(_decode_Time(_el)?),
            "nextUpdate" => nextUpdate_ = Some(_decode_Time(_el)?),
            "revokedCertificates" => {
                revokedCertificates_ = Some(|el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<CertificateListContent_revokedCertificates_Item>,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "revokedCertificates",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<CertificateListContent_revokedCertificates_Item> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_CertificateListContent_revokedCertificates_Item(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            "crlExtensions" => {
                crlExtensions_ = Some(|el: &X690Element| -> ASN1Result<Extensions> {
                    Ok(_decode_Extensions(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertificateListContent {
        version: version_,
        signature: signature_.unwrap(),
        issuer: issuer_.unwrap(),
        thisUpdate: thisUpdate_.unwrap(),
        nextUpdate: nextUpdate_,
        revokedCertificates: revokedCertificates_,
        _unrecognized,
        crlExtensions: crlExtensions_,
    })
}

pub fn _encode_CertificateListContent(value_: &CertificateListContent) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(17);
    if let Some(v_) = &value_.version {
        components_.push(_encode_Version(&v_)?);
    }
    components_.push(_encode_AlgorithmIdentifier(&value_.signature)?);
    components_.push(_encode_Name(&value_.issuer)?);
    components_.push(_encode_Time(&value_.thisUpdate)?);
    if let Some(v_) = &value_.nextUpdate {
        components_.push(_encode_Time(&v_)?);
    }
    if let Some(v_) = &value_.revokedCertificates {
        components_.push(|value_: &SEQUENCE_OF<
            CertificateListContent_revokedCertificates_Item,
        >|
         -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_CertificateListContent_revokedCertificates_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.crlExtensions {
        components_.push(|v_1: &Extensions| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_Extensions(&v_1)?),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertificateListContent(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateListContent",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateListContent,
        _eal_components_for_CertificateListContent,
        _rctl2_components_for_CertificateListContent,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_Version(_el)?,
            "signature" => _validate_AlgorithmIdentifier(_el)?,
            "issuer" => _validate_Name(_el)?,
            "thisUpdate" => _validate_Time(_el)?,
            "nextUpdate" => _validate_Time(_el)?,
            "revokedCertificates" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_CertificateListContent_revokedCertificates_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "revokedCertificates",
                    )),
                }
            }(_el)?,
            "crlExtensions" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "crlExtensions")
                    );
                }
                Ok(_validate_Extensions(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertAVL  ::=  SIGNED {TBSCertAVL}
/// ```
pub type CertAVL = SIGNED<TBSCertAVL>; // DefinedType

pub fn _decode_CertAVL(el: &X690Element) -> ASN1Result<CertAVL> {
    _decode_SIGNED::<TBSCertAVL>(_decode_TBSCertAVL, el)
}

pub fn _encode_CertAVL(value_: &CertAVL) -> ASN1Result<X690Element> {
    _encode_SIGNED::<TBSCertAVL>(_encode_TBSCertAVL, value_)
}

pub fn _validate_CertAVL(el: &X690Element) -> ASN1Result<()> {
    _validate_SIGNED::<TBSCertAVL>(_validate_TBSCertAVL, el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBSCertAVL ::= SEQUENCE {
///   version               [0]  IMPLICIT Version DEFAULT v1,
///   serialNumber               AvlSerialNumber OPTIONAL,
///   signature                  AlgorithmIdentifier {{SupportedAlgorithms}},
///   issuer                     Name,
///   constrained                BOOLEAN,
///   entries                    SEQUENCE (SIZE (1..MAX)) OF SEQUENCE {
///     idType                     CHOICE {
///       certIdentifier        [0]  PKCertIdentifier,
///       entityGroup                DistinguishedName, -- only for constrained = FALSE
///       ... },
///     scope                 [0]  IMPLICIT ScopeRestrictions OPTIONAL,
///     entryExtensions       [1]  IMPLICIT Extensions OPTIONAL,
///     ... },
///   ...,
///   ...,
///   avlExtensions              Extensions OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct TBSCertAVL {
    pub version: OPTIONAL<Version>,
    pub serialNumber: OPTIONAL<AvlSerialNumber>,
    pub signature: AlgorithmIdentifier,
    pub issuer: Name,
    pub constrained: BOOLEAN,
    pub entries: Vec<TBSCertAVL_entries_Item>,
    pub _unrecognized: Vec<X690Element>,
    pub avlExtensions: OPTIONAL<Extensions>,
}
impl TBSCertAVL {
    pub fn new(
        version: OPTIONAL<Version>,
        serialNumber: OPTIONAL<AvlSerialNumber>,
        signature: AlgorithmIdentifier,
        issuer: Name,
        constrained: BOOLEAN,
        entries: Vec<TBSCertAVL_entries_Item>,
        _unrecognized: Vec<X690Element>,
        avlExtensions: OPTIONAL<Extensions>,
    ) -> Self {
        TBSCertAVL {
            version,
            serialNumber,
            signature,
            issuer,
            constrained,
            entries,
            avlExtensions,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> Version {
        Version_v1
    }
}
impl TryFrom<&X690Element> for TBSCertAVL {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TBSCertAVL(el)
    }
}

pub const _rctl1_components_for_TBSCertAVL: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "serialNumber",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "signature",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new("issuer", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "constrained",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entries",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TBSCertAVL: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "avlExtensions",
    true,
    TagSelector::tag((TagClass::UNIVERSAL, 16)),
    None,
    None,
)];

pub const _eal_components_for_TBSCertAVL: &[ComponentSpec; 0] = &[];

pub fn _decode_TBSCertAVL(el: &X690Element) -> ASN1Result<TBSCertAVL> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TBSCertAVL")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSCertAVL,
        _eal_components_for_TBSCertAVL,
        _rctl2_components_for_TBSCertAVL,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut serialNumber_: OPTIONAL<AvlSerialNumber> = None;
    let mut signature_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut constrained_: OPTIONAL<BOOLEAN> = None;
    let mut entries_: OPTIONAL<Vec<TBSCertAVL_entries_Item>> = None;
    let mut avlExtensions_: OPTIONAL<Extensions> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_AvlSerialNumber(_el)?),
            "signature" => signature_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "issuer" => issuer_ = Some(_decode_Name(_el)?),
            "constrained" => constrained_ = Some(BER.decode_boolean(_el)?),
            "entries" => {
                entries_ = Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<TBSCertAVL_entries_Item>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "entries",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<TBSCertAVL_entries_Item> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_TBSCertAVL_entries_Item(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            "avlExtensions" => avlExtensions_ = Some(_decode_Extensions(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TBSCertAVL {
        version: version_,
        serialNumber: serialNumber_,
        signature: signature_.unwrap(),
        issuer: issuer_.unwrap(),
        constrained: constrained_.unwrap(),
        entries: entries_.unwrap(),
        _unrecognized,
        avlExtensions: avlExtensions_,
    })
}

pub fn _encode_TBSCertAVL(value_: &TBSCertAVL) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(17);
    if let Some(v_) = &value_.version {
        if *v_ != TBSCertAVL::_default_value_for_version() {
            components_.push(|v_1: &Version| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Version(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.serialNumber {
        components_.push(_encode_AvlSerialNumber(&v_)?);
    }
    components_.push(_encode_AlgorithmIdentifier(&value_.signature)?);
    components_.push(_encode_Name(&value_.issuer)?);
    components_.push(BER.encode_boolean(&value_.constrained)?);
    components_.push(
        |value_: &SEQUENCE_OF<TBSCertAVL_entries_Item>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_TBSCertAVL_entries_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.entries)?,
    );
    if let Some(v_) = &value_.avlExtensions {
        components_.push(_encode_Extensions(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_TBSCertAVL(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TBSCertAVL")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSCertAVL,
        _eal_components_for_TBSCertAVL,
        _rctl2_components_for_TBSCertAVL,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "version")
                    );
                }
                Ok(_validate_Version(&el)?)
            }(_el)?,
            "serialNumber" => _validate_AvlSerialNumber(_el)?,
            "signature" => _validate_AlgorithmIdentifier(_el)?,
            "issuer" => _validate_Name(_el)?,
            "constrained" => BER.validate_boolean(_el)?,
            "entries" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_TBSCertAVL_entries_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "entries")),
                }
            }(_el)?,
            "avlExtensions" => _validate_Extensions(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AvlSerialNumber  ::=  INTEGER (0..MAX)
/// ```
pub type AvlSerialNumber = INTEGER;

pub fn _decode_AvlSerialNumber(el: &X690Element) -> ASN1Result<AvlSerialNumber> {
    BER.decode_integer(&el)
}

pub fn _encode_AvlSerialNumber(value_: &AvlSerialNumber) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_AvlSerialNumber(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkiUser OBJECT-CLASS ::= {
///   SUBCLASS OF         {top}
///   KIND                auxiliary
///   MAY CONTAIN         {userCertificate}
///   LDAP-NAME           {"pkiUser"}
///   LDAP-DESC           "X.509 PKI User"
///   ID                  id-oc-pkiUser }
/// ```
///
///
pub fn pkiUser() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([userCertificate()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pkiUser")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 PKI User")), /* OBJECT_FIELD_SETTING */
        id: id_oc_pkiUser(),                    /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
}

pub mod pkiUser {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkiCA OBJECT-CLASS ::= {
///   SUBCLASS OF         {top}
///   KIND                auxiliary
///   MAY CONTAIN         {cACertificate |
///                        certificateRevocationList |
///                        eepkCertificateRevocationList |
///                        authorityRevocationList |
///                        crossCertificatePair}
///   LDAP-NAME           {"pkiCA"}
///   LDAP-DESC           "X.509 PKI Certificate Authority"
///   ID                  id-oc-pkiCA }
/// ```
///
///
pub fn pkiCA() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            cACertificate(),
            certificateRevocationList(),
            eepkCertificateRevocationList(),
            authorityRevocationList(),
            crossCertificatePair(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pkiCA")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 PKI Certificate Authority")), /* OBJECT_FIELD_SETTING */
        id: id_oc_pkiCA(),                      /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
}

pub mod pkiCA {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cRLDistributionPoint OBJECT-CLASS ::= {
///   SUBCLASS OF         {top}
///   KIND                structural
///   MUST CONTAIN        {commonName}
///   MAY CONTAIN         {certificateRevocationList |
///                        eepkCertificateRevocationList |
///                        authorityRevocationList |
///                        deltaRevocationList}
///   LDAP-NAME           {"cRLDistributionPoint"}
///   LDAP-DESC           "X.509 CRL distribution point"
///   ID                  id-oc-cRLDistributionPoint }
/// ```
///
///
pub fn cRLDistributionPoint() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([commonName()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            certificateRevocationList(),
            eepkCertificateRevocationList(),
            authorityRevocationList(),
            deltaRevocationList(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("cRLDistributionPoint")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 CRL distribution point")), /* OBJECT_FIELD_SETTING */
        id: id_oc_cRLDistributionPoint(),                             /* OBJECT_FIELD_SETTING */
    }
}

pub mod cRLDistributionPoint {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cRLDistPtNameForm NAME-FORM ::= {
///   NAMES               cRLDistributionPoint
///   WITH ATTRIBUTES     {commonName}
///   ID                  id-nf-cRLDistPtNameForm }
/// ```
///
///
pub fn cRLDistPtNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: cRLDistributionPoint(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([commonName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_cRLDistPtNameForm(),            /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod cRLDistPtNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// deltaCRL OBJECT-CLASS ::= {
///   SUBCLASS OF         {top}
///   KIND                auxiliary
///   MAY CONTAIN         {deltaRevocationList}
///   LDAP-NAME           {"deltaCRL"}
///   LDAP-DESC           "X.509 delta CRL"
///   ID                  id-oc-deltaCRL }
/// ```
///
///
pub fn deltaCRL() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([deltaRevocationList()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("deltaCRL")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 delta CRL")), /* OBJECT_FIELD_SETTING */
        id: id_oc_deltaCRL(),                   /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
}

pub mod deltaCRL {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cpCps OBJECT-CLASS ::= {
///   SUBCLASS OF         {top}
///   KIND                auxiliary
///   MAY CONTAIN         {certificatePolicy |
///                        certificationPracticeStmt}
///   LDAP-NAME           {"cpCps"}
///   LDAP-DESC           "Certificate Policy and Certification Practice Statement"
///   ID                  id-oc-cpCps }
/// ```
///
///
pub fn cpCps() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            certificatePolicy(),
            certificationPracticeStmt(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("cpCps")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from(
            "Certificate Policy and Certification Practice Statement",
        )), /* OBJECT_FIELD_SETTING */
        id: id_oc_cpCps(),                      /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
}

pub mod cpCps {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkiCertPath OBJECT-CLASS ::= {
///   SUBCLASS OF         {top}
///   KIND                auxiliary
///   MAY CONTAIN         {pkiPath}
///   LDAP-NAME           {"pkiCertPath"}
///   LDAP-DESC           "PKI Certification Path"
///   ID                  id-oc-pkiCertPath }
/// ```
///
///
pub fn pkiCertPath() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([pkiPath()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pkiCertPath")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("PKI Certification Path")), /* OBJECT_FIELD_SETTING */
        id: id_oc_pkiCertPath(),                /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
}

pub mod pkiCertPath {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// userCertificate ATTRIBUTE ::= {
///   WITH SYNTAX              Certificate
///   EQUALITY MATCHING RULE   certificateExactMatch
///   LDAP-SYNTAX              x509Certificate.&id
///   LDAP-NAME                {"userCertificate"}
///   LDAP-DESC                "X.509 user certificate"
///   ID                       id-at-userCertificate }
/// ```
///
///
pub fn userCertificate() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(certificateExactMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509Certificate().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("userCertificate")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 user certificate")),  /* OBJECT_FIELD_SETTING */
        id: id_at_userCertificate(),                             /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod userCertificate {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Certificate; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Certificate(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Certificate(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Certificate(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cACertificate ATTRIBUTE ::= {
///   WITH SYNTAX              Certificate
///   EQUALITY MATCHING RULE   certificateExactMatch
///   LDAP-SYNTAX              x509Certificate.&id
///   LDAP-NAME                {"cACertificate"}
///   LDAP-DESC                "X.509 CA certificate"
///   ID                       id-at-cAcertificate }
/// ```
///
///
pub fn cACertificate() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(certificateExactMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509Certificate().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("cACertificate")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 CA certificate")),    /* OBJECT_FIELD_SETTING */
        id: id_at_cAcertificate(),                               /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod cACertificate {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Certificate; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Certificate(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Certificate(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Certificate(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// crossCertificatePair ATTRIBUTE ::= {
///   WITH SYNTAX              CertificatePair
///   EQUALITY MATCHING RULE   certificatePairExactMatch
///   LDAP-SYNTAX              x509CertificatePair.&id
///   LDAP-NAME                {"crossCertificatePair"}
///   LDAP-DESC                "X.509 cross certificate pair"
///   ID                       id-at-crossCertificatePair }
/// ```
///
///
pub fn crossCertificatePair() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(certificatePairExactMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509CertificatePair().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("crossCertificatePair")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 cross certificate pair")), /* OBJECT_FIELD_SETTING */
        id: id_at_crossCertificatePair(),                             /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod crossCertificatePair {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificatePair; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificatePair(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificatePair(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificatePair(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificatePair ::= SEQUENCE {
///   issuedToThisCA  [0]  Certificate OPTIONAL,
///   issuedByThisCA  [1]  Certificate OPTIONAL,
///   ... }
///   (WITH COMPONENTS { ..., issuedToThisCA PRESENT} |
///    WITH COMPONENTS { ..., issuedByThisCA PRESENT})
/// ```
///
#[derive(Debug, Clone)]
pub struct CertificatePair {
    pub issuedToThisCA: OPTIONAL<Certificate>,
    pub issuedByThisCA: OPTIONAL<Certificate>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertificatePair {
    pub fn new(
        issuedToThisCA: OPTIONAL<Certificate>,
        issuedByThisCA: OPTIONAL<Certificate>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertificatePair {
            issuedToThisCA,
            issuedByThisCA,
            _unrecognized,
        }
    }
}
impl Default for CertificatePair {
    fn default() -> Self {
        CertificatePair {
            issuedToThisCA: None,
            issuedByThisCA: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<&X690Element> for CertificatePair {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertificatePair(el)
    }
}

pub const _rctl1_components_for_CertificatePair: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "issuedToThisCA",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuedByThisCA",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificatePair: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertificatePair: &[ComponentSpec; 0] = &[];

pub fn _decode_CertificatePair(el: &X690Element) -> ASN1Result<CertificatePair> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificatePair"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificatePair,
        _eal_components_for_CertificatePair,
        _rctl2_components_for_CertificatePair,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuedToThisCA_: OPTIONAL<Certificate> = None;
    let mut issuedByThisCA_: OPTIONAL<Certificate> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuedToThisCA" => {
                issuedToThisCA_ = Some(|el: &X690Element| -> ASN1Result<Certificate> {
                    Ok(_decode_Certificate(&el.inner()?)?)
                }(_el)?)
            }
            "issuedByThisCA" => {
                issuedByThisCA_ = Some(|el: &X690Element| -> ASN1Result<Certificate> {
                    Ok(_decode_Certificate(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertificatePair {
        issuedToThisCA: issuedToThisCA_,
        issuedByThisCA: issuedByThisCA_,
        _unrecognized,
    })
}

pub fn _encode_CertificatePair(value_: &CertificatePair) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.issuedToThisCA {
        components_.push(|v_1: &Certificate| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_Certificate(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.issuedByThisCA {
        components_.push(|v_1: &Certificate| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_Certificate(&v_1)?),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertificatePair(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertificatePair"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificatePair,
        _eal_components_for_CertificatePair,
        _rctl2_components_for_CertificatePair,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuedToThisCA" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "issuedToThisCA")
                    );
                }
                Ok(_validate_Certificate(&el.inner()?)?)
            }(_el)?,
            "issuedByThisCA" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "issuedByThisCA")
                    );
                }
                Ok(_validate_Certificate(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificateRevocationList ATTRIBUTE ::= {
///   WITH SYNTAX              CertificateList
///   EQUALITY MATCHING RULE   certificateListExactMatch
///   LDAP-SYNTAX              x509CertificateList.&id
///   LDAP-NAME                {"certificateRevocationList"}
///   LDAP-DESC                "X.509 certificate revocation list"
///   ID                       id-at-certificateRevocationList }
/// ```
///
///
pub fn certificateRevocationList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(certificateListExactMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509CertificateList().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("certificateRevocationList")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 certificate revocation list")), /* OBJECT_FIELD_SETTING */
        id: id_at_certificateRevocationList(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod certificateRevocationList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateList; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateList(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateList(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateList(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// eepkCertificateRevocationList ATTRIBUTE ::= {
///   WITH SYNTAX              CertificateList
///   EQUALITY MATCHING RULE   certificateListExactMatch
///   LDAP-SYNTAX              x509CertificateList.&id
///   LDAP-NAME                {"eepkCertificateRevocationList"}
///   LDAP-DESC                "X.509 EEPK certificate revocation list"
///   ID                       id-at-eepkCertificateRevocationList }
/// ```
///
///
pub fn eepkCertificateRevocationList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(certificateListExactMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509CertificateList().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("eepkCertificateRevocationList")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 EEPK certificate revocation list")), /* OBJECT_FIELD_SETTING */
        id: id_at_eepkCertificateRevocationList(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod eepkCertificateRevocationList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateList; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateList(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateList(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateList(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// authorityRevocationList ATTRIBUTE ::= {
///   WITH SYNTAX              CertificateList
///   EQUALITY MATCHING RULE   certificateListExactMatch
///   LDAP-SYNTAX              x509CertificateList.&id
///   LDAP-NAME                {"authorityRevocationList"}
///   LDAP-DESC                "X.509 authority revocation list"
///   ID                       id-at-authorityRevocationList }
/// ```
///
///
pub fn authorityRevocationList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(certificateListExactMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509CertificateList().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("authorityRevocationList")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 authority revocation list")), /* OBJECT_FIELD_SETTING */
        id: id_at_authorityRevocationList(),                             /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod authorityRevocationList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateList; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateList(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateList(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateList(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// deltaRevocationList ATTRIBUTE ::= {
///   WITH SYNTAX              CertificateList
///   EQUALITY MATCHING RULE   certificateListExactMatch
///   LDAP-SYNTAX              x509CertificateList.&id
///   LDAP-NAME                {"deltaRevocationList"}
///   LDAP-DESC                "X.509 delta revocation list"
///   ID                       id-at-deltaRevocationList }
/// ```
///
///
pub fn deltaRevocationList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(certificateListExactMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509CertificateList().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("deltaRevocationList")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 delta revocation list")), /* OBJECT_FIELD_SETTING */
        id: id_at_deltaRevocationList(),                             /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod deltaRevocationList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateList; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateList(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateList(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateList(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// supportedAlgorithms ATTRIBUTE ::= {
///   WITH SYNTAX              SupportedAlgorithm
///   EQUALITY MATCHING RULE   algorithmIdentifierMatch
///   LDAP-SYNTAX              x509SupportedAlgorithm.&id
///   LDAP-NAME                {"supportedAlgorithms"}
///   LDAP-DESC                "X.509 support algorithms"
///   ID                       id-at-supportedAlgorithms }
/// ```
///
///
pub fn supportedAlgorithms() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(algorithmIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509SupportedAlgorithm().id),              /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("supportedAlgorithms")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 support algorithms")),   /* OBJECT_FIELD_SETTING */
        id: id_at_supportedAlgorithms(),                            /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod supportedAlgorithms {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SupportedAlgorithm; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SupportedAlgorithm(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SupportedAlgorithm(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SupportedAlgorithm(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedAlgorithm ::= SEQUENCE {
///   algorithmIdentifier              AlgorithmIdentifier{{SupportedAlgorithms}},
///   intendedUsage               [0]  KeyUsage OPTIONAL,
///   intendedCertificatePolicies [1]  CertificatePoliciesSyntax OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct SupportedAlgorithm {
    pub algorithmIdentifier: AlgorithmIdentifier,
    pub intendedUsage: OPTIONAL<KeyUsage>,
    pub intendedCertificatePolicies: OPTIONAL<CertificatePoliciesSyntax>,
    pub _unrecognized: Vec<X690Element>,
}
impl SupportedAlgorithm {
    pub fn new(
        algorithmIdentifier: AlgorithmIdentifier,
        intendedUsage: OPTIONAL<KeyUsage>,
        intendedCertificatePolicies: OPTIONAL<CertificatePoliciesSyntax>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SupportedAlgorithm {
            algorithmIdentifier,
            intendedUsage,
            intendedCertificatePolicies,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for SupportedAlgorithm {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SupportedAlgorithm(el)
    }
}

pub const _rctl1_components_for_SupportedAlgorithm: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "algorithmIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "intendedUsage",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "intendedCertificatePolicies",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SupportedAlgorithm: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SupportedAlgorithm: &[ComponentSpec; 0] = &[];

pub fn _decode_SupportedAlgorithm(el: &X690Element) -> ASN1Result<SupportedAlgorithm> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SupportedAlgorithm")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SupportedAlgorithm,
        _eal_components_for_SupportedAlgorithm,
        _rctl2_components_for_SupportedAlgorithm,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithmIdentifier_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut intendedUsage_: OPTIONAL<KeyUsage> = None;
    let mut intendedCertificatePolicies_: OPTIONAL<CertificatePoliciesSyntax> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => algorithmIdentifier_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "intendedUsage" => {
                intendedUsage_ = Some(|el: &X690Element| -> ASN1Result<KeyUsage> {
                    Ok(_decode_KeyUsage(&el.inner()?)?)
                }(_el)?)
            }
            "intendedCertificatePolicies" => {
                intendedCertificatePolicies_ = Some(
                    |el: &X690Element| -> ASN1Result<CertificatePoliciesSyntax> {
                        Ok(_decode_CertificatePoliciesSyntax(&el.inner()?)?)
                    }(_el)?,
                )
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(SupportedAlgorithm {
        algorithmIdentifier: algorithmIdentifier_.unwrap(),
        intendedUsage: intendedUsage_,
        intendedCertificatePolicies: intendedCertificatePolicies_,
        _unrecognized,
    })
}

pub fn _encode_SupportedAlgorithm(value_: &SupportedAlgorithm) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
    if let Some(v_) = &value_.intendedUsage {
        components_.push(|v_1: &KeyUsage| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_KeyUsage(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.intendedCertificatePolicies {
        components_.push(
            |v_1: &CertificatePoliciesSyntax| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_CertificatePoliciesSyntax(&v_1)?),
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

pub fn _validate_SupportedAlgorithm(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SupportedAlgorithm")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SupportedAlgorithm,
        _eal_components_for_SupportedAlgorithm,
        _rctl2_components_for_SupportedAlgorithm,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => _validate_AlgorithmIdentifier(_el)?,
            "intendedUsage" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "intendedUsage")
                    );
                }
                Ok(_validate_KeyUsage(&el.inner()?)?)
            }(_el)?,
            "intendedCertificatePolicies" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "intendedCertificatePolicies",
                    ));
                }
                Ok(_validate_CertificatePoliciesSyntax(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificationPracticeStmt ATTRIBUTE ::= {
///   WITH SYNTAX  InfoSyntax
///   ID           id-at-certificationPracticeStmt }
/// ```
///
///
pub fn certificationPracticeStmt() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_at_certificationPracticeStmt(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod certificationPracticeStmt {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = InfoSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_InfoSyntax(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_InfoSyntax(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_InfoSyntax(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InfoSyntax  ::=  CHOICE {
///   content  UnboundedDirectoryString,
///   pointer  SEQUENCE {
///     name     GeneralNames,
///     hash     HASH{HashedPolicyInfo} OPTIONAL,
///     ... },
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum InfoSyntax {
    content(UnboundedDirectoryString),
    pointer(InfoSyntax_pointer),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for InfoSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_InfoSyntax(el)
    }
}

pub fn _decode_InfoSyntax(el: &X690Element) -> ASN1Result<InfoSyntax> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 20) => {
            Ok(InfoSyntax::content(_decode_UnboundedDirectoryString(&el)?))
        }
        (TagClass::UNIVERSAL, 19) => {
            Ok(InfoSyntax::content(_decode_UnboundedDirectoryString(&el)?))
        }
        (TagClass::UNIVERSAL, 30) => {
            Ok(InfoSyntax::content(_decode_UnboundedDirectoryString(&el)?))
        }
        (TagClass::UNIVERSAL, 28) => {
            Ok(InfoSyntax::content(_decode_UnboundedDirectoryString(&el)?))
        }
        (TagClass::UNIVERSAL, 12) => {
            Ok(InfoSyntax::content(_decode_UnboundedDirectoryString(&el)?))
        }
        (TagClass::UNIVERSAL, 16) => Ok(InfoSyntax::pointer(_decode_InfoSyntax_pointer(&el)?)),
        _ => Ok(InfoSyntax::_unrecognized(el.clone())),
    }
}

pub fn _encode_InfoSyntax(value_: &InfoSyntax) -> ASN1Result<X690Element> {
    match value_ {
        InfoSyntax::content(v) => _encode_UnboundedDirectoryString(&v),
        InfoSyntax::pointer(v) => _encode_InfoSyntax_pointer(&v),
        InfoSyntax::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_InfoSyntax(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 20) => _validate_UnboundedDirectoryString(&el),
        (TagClass::UNIVERSAL, 19) => _validate_UnboundedDirectoryString(&el),
        (TagClass::UNIVERSAL, 30) => _validate_UnboundedDirectoryString(&el),
        (TagClass::UNIVERSAL, 28) => _validate_UnboundedDirectoryString(&el),
        (TagClass::UNIVERSAL, 12) => _validate_UnboundedDirectoryString(&el),
        (TagClass::UNIVERSAL, 16) => _validate_InfoSyntax_pointer(&el),
        _ => Ok(()),
    }
}

pub type POLICY = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// HashedPolicyInfo  ::=  POLICY.&Type({Policies})
/// ```
pub type HashedPolicyInfo = X690Element; // ObjectClassFieldType

pub fn _decode_HashedPolicyInfo(el: &X690Element) -> ASN1Result<HashedPolicyInfo> {
    x690_identity(&el)
}

pub fn _encode_HashedPolicyInfo(value_: &HashedPolicyInfo) -> ASN1Result<X690Element> {
    x690_identity(&value_)
}

pub fn _validate_HashedPolicyInfo(el: &X690Element) -> ASN1Result<()> {
    BER.validate_any(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Policies POLICY ::= {...}
/// ```
///
///
pub fn Policies() -> Vec<POLICY> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificatePolicy ATTRIBUTE ::= {
///   WITH SYNTAX             PolicySyntax
///   EQUALITY MATCHING RULE  policyMatch
///   ID                      id-at-certificatePolicy }
/// ```
///
///
pub fn certificatePolicy() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(policyMatch())), /* OBJECT_FIELD_SETTING */
        id: id_at_certificatePolicy(),                 /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod certificatePolicy {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PolicySyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PolicySyntax(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PolicySyntax(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PolicySyntax(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PolicySyntax ::= SEQUENCE {
///   policyIdentifier  PolicyID,
///   policySyntax      InfoSyntax,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct PolicySyntax {
    pub policyIdentifier: PolicyID,
    pub policySyntax: InfoSyntax,
    pub _unrecognized: Vec<X690Element>,
}
impl PolicySyntax {
    pub fn new(
        policyIdentifier: PolicyID,
        policySyntax: InfoSyntax,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PolicySyntax {
            policyIdentifier,
            policySyntax,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for PolicySyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PolicySyntax(el)
    }
}

pub const _rctl1_components_for_PolicySyntax: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "policyIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("policySyntax", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_PolicySyntax: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PolicySyntax: &[ComponentSpec; 0] = &[];

pub fn _decode_PolicySyntax(el: &X690Element) -> ASN1Result<PolicySyntax> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PolicySyntax")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PolicySyntax,
        _eal_components_for_PolicySyntax,
        _rctl2_components_for_PolicySyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut policyIdentifier_: OPTIONAL<PolicyID> = None;
    let mut policySyntax_: OPTIONAL<InfoSyntax> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "policyIdentifier" => policyIdentifier_ = Some(_decode_PolicyID(_el)?),
            "policySyntax" => policySyntax_ = Some(_decode_InfoSyntax(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PolicySyntax {
        policyIdentifier: policyIdentifier_.unwrap(),
        policySyntax: policySyntax_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_PolicySyntax(value_: &PolicySyntax) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_PolicyID(&value_.policyIdentifier)?);
    components_.push(_encode_InfoSyntax(&value_.policySyntax)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_PolicySyntax(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PolicySyntax")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PolicySyntax,
        _eal_components_for_PolicySyntax,
        _rctl2_components_for_PolicySyntax,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "policyIdentifier" => _validate_PolicyID(_el)?,
            "policySyntax" => _validate_InfoSyntax(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PolicyID  ::=  CertPolicyId
/// ```
pub type PolicyID = CertPolicyId; // DefinedType

pub fn _decode_PolicyID(el: &X690Element) -> ASN1Result<PolicyID> {
    _decode_CertPolicyId(&el)
}

pub fn _encode_PolicyID(value_: &PolicyID) -> ASN1Result<X690Element> {
    _encode_CertPolicyId(&value_)
}

pub fn _validate_PolicyID(el: &X690Element) -> ASN1Result<()> {
    _validate_CertPolicyId(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkiPath ATTRIBUTE ::= {
///   WITH SYNTAX              PkiPath
///   EQUALITY MATCHING RULE   pkiPathMatch
///   ID                       id-at-pkiPath }
/// ```
///
///
pub fn pkiPath() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(pkiPathMatch())), /* OBJECT_FIELD_SETTING */
        id: id_at_pkiPath(),                            /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pkiPath {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PkiPath; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PkiPath(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PkiPath(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PkiPath(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// supportedPublicKeyAlgorithms ATTRIBUTE ::= {
///   WITH SYNTAX            SupportedPublicKeyAlgorithms
///   EQUALITY MATCHING RULE algorithmIdentifierMatch
///   LDAP-SYNTAX            x509SupportedPublicKeyAlgos.&id
///   LDAP-NAME              {"supportedPublicKeyAlgorithms"}
///   LDAP-DESC              "X.509 supported publiv key algorithms"
///   ID                     id-at-supportedPublicKeyAlgorithms }
/// ```
///
///
pub fn supportedPublicKeyAlgorithms() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(algorithmIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(x509SupportedPublicKeyAlgos().id),         /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("supportedPublicKeyAlgorithms")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 supported publiv key algorithms")), /* OBJECT_FIELD_SETTING */
        id: id_at_supportedPublicKeyAlgorithms(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod supportedPublicKeyAlgorithms {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SupportedPublicKeyAlgorithms; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SupportedPublicKeyAlgorithms(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SupportedPublicKeyAlgorithms(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SupportedPublicKeyAlgorithms(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedPublicKeyAlgorithms ::= SEQUENCE {
///   algorithmIdentifier      AlgorithmIdentifier{{SupportedPublicKeyAlgos}},
///   minKeySize               INTEGER,
///   extensions          [0]  SEQUENCE SIZE (1..MAX) OF OidOrAttr OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct SupportedPublicKeyAlgorithms {
    pub algorithmIdentifier: AlgorithmIdentifier,
    pub minKeySize: INTEGER,
    pub extensions: OPTIONAL<Vec<OidOrAttr>>,
    pub _unrecognized: Vec<X690Element>,
}
impl SupportedPublicKeyAlgorithms {
    pub fn new(
        algorithmIdentifier: AlgorithmIdentifier,
        minKeySize: INTEGER,
        extensions: OPTIONAL<Vec<OidOrAttr>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SupportedPublicKeyAlgorithms {
            algorithmIdentifier,
            minKeySize,
            extensions,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for SupportedPublicKeyAlgorithms {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SupportedPublicKeyAlgorithms(el)
    }
}

pub const _rctl1_components_for_SupportedPublicKeyAlgorithms: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "algorithmIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "minKeySize",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SupportedPublicKeyAlgorithms: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SupportedPublicKeyAlgorithms: &[ComponentSpec; 0] = &[];

pub fn _decode_SupportedPublicKeyAlgorithms(
    el: &X690Element,
) -> ASN1Result<SupportedPublicKeyAlgorithms> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "SupportedPublicKeyAlgorithms",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SupportedPublicKeyAlgorithms,
        _eal_components_for_SupportedPublicKeyAlgorithms,
        _rctl2_components_for_SupportedPublicKeyAlgorithms,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithmIdentifier_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut minKeySize_: OPTIONAL<INTEGER> = None;
    let mut extensions_: OPTIONAL<Vec<OidOrAttr>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => algorithmIdentifier_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "minKeySize" => minKeySize_ = Some(BER.decode_integer(_el)?),
            "extensions" => {
                extensions_ = Some(|el: &X690Element| -> ASN1Result<Vec<OidOrAttr>> {
                    Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<OidOrAttr>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "extensions",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<OidOrAttr> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_OidOrAttr(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(SupportedPublicKeyAlgorithms {
        algorithmIdentifier: algorithmIdentifier_.unwrap(),
        minKeySize: minKeySize_.unwrap(),
        extensions: extensions_,
        _unrecognized,
    })
}

pub fn _encode_SupportedPublicKeyAlgorithms(
    value_: &SupportedPublicKeyAlgorithms,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
    components_.push(BER.encode_integer(&value_.minKeySize)?);
    if let Some(v_) = &value_.extensions {
        components_.push(|v_1: &Vec<OidOrAttr>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<OidOrAttr>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_OidOrAttr(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_SupportedPublicKeyAlgorithms(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "SupportedPublicKeyAlgorithms",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SupportedPublicKeyAlgorithms,
        _eal_components_for_SupportedPublicKeyAlgorithms,
        _rctl2_components_for_SupportedPublicKeyAlgorithms,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => _validate_AlgorithmIdentifier(_el)?,
            "minKeySize" => BER.validate_integer(_el)?,
            "extensions" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "extensions")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_OidOrAttr(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "extensions")
                        ),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedPublicKeyAlgos ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedPublicKeyAlgos() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OidOrAttr  ::=  CHOICE {
///   oid       ATTRIBUTE.&id ({ ExtAttributes }),
///   attribute Attribute {{ ExtAttributes }},
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum OidOrAttr {
    oid(OBJECT_IDENTIFIER),
    attribute(Attribute),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for OidOrAttr {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OidOrAttr(el)
    }
}

pub fn _decode_OidOrAttr(el: &X690Element) -> ASN1Result<OidOrAttr> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 6) => Ok(OidOrAttr::oid(BER.decode_object_identifier(&el)?)),
        (TagClass::UNIVERSAL, 16) => Ok(OidOrAttr::attribute(_decode_Attribute(&el)?)),
        _ => Ok(OidOrAttr::_unrecognized(el.clone())),
    }
}

pub fn _encode_OidOrAttr(value_: &OidOrAttr) -> ASN1Result<X690Element> {
    match value_ {
        OidOrAttr::oid(v) => BER.encode_object_identifier(&v),
        OidOrAttr::attribute(v) => _encode_Attribute(&v),
        OidOrAttr::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_OidOrAttr(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 6) => BER.validate_object_identifier(&el),
        (TagClass::UNIVERSAL, 16) => _validate_Attribute(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtAttributes ATTRIBUTE ::= {...}
/// ```
///
///
pub fn ExtAttributes() -> Vec<ATTRIBUTE> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// userPassword ATTRIBUTE ::= {
///   WITH SYNTAX              OCTET STRING(SIZE (0..MAX))
///   EQUALITY MATCHING RULE   octetStringMatch
///   LDAP-SYNTAX              octetString.&id
///   LDAP-NAME                {"userPassword"}
///   ID                       id-at-userPassword }
/// ```
///
///
pub fn userPassword() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(octetStringMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(octetString().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("userPassword")])), /* OBJECT_FIELD_SETTING */
        id: id_at_userPassword(),                           /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod userPassword {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = OCTET_STRING; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_octet_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_octet_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_octet_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// x509SupportedPublicKeyAlgos SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 supported publiv key algorithms"
///   DIRECTORY SYNTAX  SupportedPublicKeyAlgorithms
///   ID                id-asx-x509SupportedPublicKeyAlgos }
/// ```
///
///
pub fn x509SupportedPublicKeyAlgos() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 supported publiv key algorithms"), /* OBJECT_FIELD_SETTING */
        id: id_asx_x509SupportedPublicKeyAlgos(),                        /* OBJECT_FIELD_SETTING */
    }
}

pub mod x509SupportedPublicKeyAlgos {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SupportedPublicKeyAlgorithms; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SupportedPublicKeyAlgorithms(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SupportedPublicKeyAlgorithms(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SupportedPublicKeyAlgorithms(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// x509Certificate SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 Certificate"
///   DIRECTORY SYNTAX  Certificate
///   ID                id-lsx-x509Certificate }
/// ```
///
///
pub fn x509Certificate() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 Certificate"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_x509Certificate(),                /* OBJECT_FIELD_SETTING */
    }
}

pub mod x509Certificate {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Certificate; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Certificate(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Certificate(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Certificate(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// x509CertificateList SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 Certificate List"
///   DIRECTORY SYNTAX  CertificateList
///   ID                id-lsx-x509CertificateList }
/// ```
///
///
pub fn x509CertificateList() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 Certificate List"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_x509CertificateList(),                 /* OBJECT_FIELD_SETTING */
    }
}

pub mod x509CertificateList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificateList; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificateList(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificateList(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificateList(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// x509CertificatePair SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 Certificate Pair"
///   DIRECTORY SYNTAX  CertificatePair
///   ID                id-lsx-x509CertificatePair }
/// ```
///
///
pub fn x509CertificatePair() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 Certificate Pair"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_x509CertificatePair(),                 /* OBJECT_FIELD_SETTING */
    }
}

pub mod x509CertificatePair {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertificatePair; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertificatePair(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertificatePair(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertificatePair(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// x509SupportedAlgorithm SYNTAX-NAME ::= {
///   LDAP-DESC         "X.509 Supported Algorithm"
///   DIRECTORY SYNTAX  SupportedAlgorithm
///   ID                id-lsx-x509SupportedAlgorithm }
/// ```
///
///
pub fn x509SupportedAlgorithm() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("X.509 Supported Algorithm"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_x509SupportedAlgorithm(),                 /* OBJECT_FIELD_SETTING */
    }
}

pub mod x509SupportedAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SupportedAlgorithm; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SupportedAlgorithm(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SupportedAlgorithm(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SupportedAlgorithm(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-cRLDistributionPoint          OBJECT IDENTIFIER ::= {id-oc 19}
/// ```
///
///
#[inline]
pub fn id_oc_cRLDistributionPoint () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oc(), 19).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pkiUser                       OBJECT IDENTIFIER ::= {id-oc 21}
/// ```
///
///
#[inline]
pub fn id_oc_pkiUser () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oc(), 21).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pkiCA                         OBJECT IDENTIFIER ::= {id-oc 22}
/// ```
///
///
#[inline]
pub fn id_oc_pkiCA () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oc(), 22).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-deltaCRL                      OBJECT IDENTIFIER ::= {id-oc 23}
/// ```
///
///
#[inline]
pub fn id_oc_deltaCRL () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oc(), 23).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-cpCps                         OBJECT IDENTIFIER ::= {id-oc 30}
/// ```
///
///
#[inline]
pub fn id_oc_cpCps () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oc(), 30).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pkiCertPath                   OBJECT IDENTIFIER ::= {id-oc 31}
/// ```
///
///
#[inline]
pub fn id_oc_pkiCertPath () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oc(), 31).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-cRLDistPtNameForm             OBJECT IDENTIFIER ::= {id-nf 14}
/// ```
///
///
#[inline]
pub fn id_nf_cRLDistPtNameForm () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_nf(), 14).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-userPassword                  OBJECT IDENTIFIER ::= {id-at 35}
/// ```
///
///
#[inline]
pub fn id_at_userPassword () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 35).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-userCertificate               OBJECT IDENTIFIER ::= {id-at 36}
/// ```
///
///
#[inline]
pub fn id_at_userCertificate () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 36).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-cAcertificate                 OBJECT IDENTIFIER ::= {id-at 37}
/// ```
///
///
#[inline]
pub fn id_at_cAcertificate () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 37).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-authorityRevocationList       OBJECT IDENTIFIER ::= {id-at 38}
/// ```
///
///
#[inline]
pub fn id_at_authorityRevocationList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 38).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-certificateRevocationList     OBJECT IDENTIFIER ::= {id-at 39}
/// ```
///
///
#[inline]
pub fn id_at_certificateRevocationList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 39).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-crossCertificatePair          OBJECT IDENTIFIER ::= {id-at 40}
/// ```
///
///
#[inline]
pub fn id_at_crossCertificatePair () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 40).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-supportedAlgorithms           OBJECT IDENTIFIER ::= {id-at 52}
/// ```
///
///
#[inline]
pub fn id_at_supportedAlgorithms () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 52).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-deltaRevocationList           OBJECT IDENTIFIER ::= {id-at 53}
/// ```
///
///
#[inline]
pub fn id_at_deltaRevocationList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 53).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-certificationPracticeStmt     OBJECT IDENTIFIER ::= {id-at 68}
/// ```
///
///
#[inline]
pub fn id_at_certificationPracticeStmt () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 68).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-certificatePolicy             OBJECT IDENTIFIER ::= {id-at 69}
/// ```
///
///
#[inline]
pub fn id_at_certificatePolicy () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 69).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-pkiPath                       OBJECT IDENTIFIER ::= {id-at 70}
/// ```
///
///
#[inline]
pub fn id_at_pkiPath () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 70).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-eepkCertificateRevocationList OBJECT IDENTIFIER ::= {id-at 101}
/// ```
///
///
#[inline]
pub fn id_at_eepkCertificateRevocationList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 101).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-supportedPublicKeyAlgorithms  OBJECT IDENTIFIER ::= {id-at 103}
/// ```
///
///
#[inline]
pub fn id_at_supportedPublicKeyAlgorithms () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 103).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-x509SupportedPublicKeyAlgos  OBJECT IDENTIFIER ::= {id-asx 10}
/// ```
///
///
#[inline]
pub fn id_asx_x509SupportedPublicKeyAlgos () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_asx(), 10).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-x509Certificate              OBJECT IDENTIFIER ::= {id-lsx 8}
/// ```
///
///
#[inline]
pub fn id_lsx_x509Certificate () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_lsx(), 8).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-x509CertificateList          OBJECT IDENTIFIER ::= {id-lsx 9}
/// ```
///
///
#[inline]
pub fn id_lsx_x509CertificateList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_lsx(), 9).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-x509CertificatePair          OBJECT IDENTIFIER ::= {id-lsx 10}
/// ```
///
///
#[inline]
pub fn id_lsx_x509CertificatePair () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_lsx(), 10).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-x509SupportedAlgorithm       OBJECT IDENTIFIER ::= {id-lsx 49}
/// ```
///
///
#[inline]
pub fn id_lsx_x509SupportedAlgorithm () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_lsx(), 49).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateListContent-revokedCertificates-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertificateListContent_revokedCertificates_Item {
    pub serialNumber: CertificateSerialNumber,
    pub revocationDate: Time,
    pub crlEntryExtensions: OPTIONAL<Extensions>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertificateListContent_revokedCertificates_Item {
    pub fn new(
        serialNumber: CertificateSerialNumber,
        revocationDate: Time,
        crlEntryExtensions: OPTIONAL<Extensions>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertificateListContent_revokedCertificates_Item {
            serialNumber,
            revocationDate,
            crlEntryExtensions,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for CertificateListContent_revokedCertificates_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateListContent_revokedCertificates_Item(el)
    }
}

pub const _rctl1_components_for_CertificateListContent_revokedCertificates_Item: &[ComponentSpec;
     3] = &[
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("revocationDate", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "crlEntryExtensions",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertificateListContent_revokedCertificates_Item: &[ComponentSpec;
     0] = &[];

pub const _eal_components_for_CertificateListContent_revokedCertificates_Item: &[ComponentSpec; 0] =
    &[];

pub fn _decode_CertificateListContent_revokedCertificates_Item(
    el: &X690Element,
) -> ASN1Result<CertificateListContent_revokedCertificates_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateListContent-revokedCertificates-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateListContent_revokedCertificates_Item,
        _eal_components_for_CertificateListContent_revokedCertificates_Item,
        _rctl2_components_for_CertificateListContent_revokedCertificates_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut revocationDate_: OPTIONAL<Time> = None;
    let mut crlEntryExtensions_: OPTIONAL<Extensions> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            "revocationDate" => revocationDate_ = Some(_decode_Time(_el)?),
            "crlEntryExtensions" => crlEntryExtensions_ = Some(_decode_Extensions(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertificateListContent_revokedCertificates_Item {
        serialNumber: serialNumber_.unwrap(),
        revocationDate: revocationDate_.unwrap(),
        crlEntryExtensions: crlEntryExtensions_,
        _unrecognized,
    })
}

pub fn _encode_CertificateListContent_revokedCertificates_Item(
    value_: &CertificateListContent_revokedCertificates_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    components_.push(_encode_Time(&value_.revocationDate)?);
    if let Some(v_) = &value_.crlEntryExtensions {
        components_.push(_encode_Extensions(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertificateListContent_revokedCertificates_Item(
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertificateListContent-revokedCertificates-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertificateListContent_revokedCertificates_Item,
        _eal_components_for_CertificateListContent_revokedCertificates_Item,
        _rctl2_components_for_CertificateListContent_revokedCertificates_Item,
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
            "revocationDate" => _validate_Time(_el)?,
            "crlEntryExtensions" => _validate_Extensions(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBSCertAVL-entries-Item-idType ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TBSCertAVL_entries_Item_idType {
    certIdentifier(PKCertIdentifier),
    entityGroup(DistinguishedName),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for TBSCertAVL_entries_Item_idType {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TBSCertAVL_entries_Item_idType(el)
    }
}

pub fn _decode_TBSCertAVL_entries_Item_idType(
    el: &X690Element,
) -> ASN1Result<TBSCertAVL_entries_Item_idType> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(TBSCertAVL_entries_Item_idType::certIdentifier(
            |el: &X690Element| -> ASN1Result<PKCertIdentifier> {
                Ok(_decode_PKCertIdentifier(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::UNIVERSAL, 16) => Ok(TBSCertAVL_entries_Item_idType::entityGroup(
            _decode_DistinguishedName(&el)?,
        )),
        _ => Ok(TBSCertAVL_entries_Item_idType::_unrecognized(el.clone())),
    }
}

pub fn _encode_TBSCertAVL_entries_Item_idType(
    value_: &TBSCertAVL_entries_Item_idType,
) -> ASN1Result<X690Element> {
    match value_ {
        TBSCertAVL_entries_Item_idType::certIdentifier(v) => {
            |v_1: &PKCertIdentifier| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&_encode_PKCertIdentifier(&v_1)?),
                ))
            }(&v)
        }
        TBSCertAVL_entries_Item_idType::entityGroup(v) => _encode_DistinguishedName(&v),
        TBSCertAVL_entries_Item_idType::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_TBSCertAVL_entries_Item_idType(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certIdentifier")
                );
            }
            Ok(_validate_PKCertIdentifier(&el.inner()?)?)
        }(&el),
        (TagClass::UNIVERSAL, 16) => _validate_DistinguishedName(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBSCertAVL-entries-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct TBSCertAVL_entries_Item {
    pub idType: TBSCertAVL_entries_Item_idType,
    pub scope: OPTIONAL<ScopeRestrictions>,
    pub entryExtensions: OPTIONAL<Extensions>,
    pub _unrecognized: Vec<X690Element>,
}
impl TBSCertAVL_entries_Item {
    pub fn new(
        idType: TBSCertAVL_entries_Item_idType,
        scope: OPTIONAL<ScopeRestrictions>,
        entryExtensions: OPTIONAL<Extensions>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TBSCertAVL_entries_Item {
            idType,
            scope,
            entryExtensions,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for TBSCertAVL_entries_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TBSCertAVL_entries_Item(el)
    }
}

pub const _rctl1_components_for_TBSCertAVL_entries_Item: &[ComponentSpec; 3] = &[
    ComponentSpec::new("idType", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "scope",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryExtensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TBSCertAVL_entries_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TBSCertAVL_entries_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_TBSCertAVL_entries_Item(el: &X690Element) -> ASN1Result<TBSCertAVL_entries_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "TBSCertAVL-entries-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSCertAVL_entries_Item,
        _eal_components_for_TBSCertAVL_entries_Item,
        _rctl2_components_for_TBSCertAVL_entries_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut idType_: OPTIONAL<TBSCertAVL_entries_Item_idType> = None;
    let mut scope_: OPTIONAL<ScopeRestrictions> = None;
    let mut entryExtensions_: OPTIONAL<Extensions> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "idType" => idType_ = Some(_decode_TBSCertAVL_entries_Item_idType(_el)?),
            "scope" => scope_ = Some(_decode_ScopeRestrictions(_el)?),
            "entryExtensions" => entryExtensions_ = Some(_decode_Extensions(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TBSCertAVL_entries_Item {
        idType: idType_.unwrap(),
        scope: scope_,
        entryExtensions: entryExtensions_,
        _unrecognized,
    })
}

pub fn _encode_TBSCertAVL_entries_Item(
    value_: &TBSCertAVL_entries_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_TBSCertAVL_entries_Item_idType(&value_.idType)?);
    if let Some(v_) = &value_.scope {
        components_.push(|v_1: &ScopeRestrictions| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ScopeRestrictions(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.entryExtensions {
        components_.push(|v_1: &Extensions| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Extensions(&v_1)?;
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

pub fn _validate_TBSCertAVL_entries_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "TBSCertAVL-entries-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSCertAVL_entries_Item,
        _eal_components_for_TBSCertAVL_entries_Item,
        _rctl2_components_for_TBSCertAVL_entries_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "idType" => _validate_TBSCertAVL_entries_Item_idType(_el)?,
            "scope" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "scope"));
                }
                Ok(_validate_ScopeRestrictions(&el)?)
            }(_el)?,
            "entryExtensions" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "entryExtensions",
                    ));
                }
                Ok(_validate_Extensions(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InfoSyntax-pointer ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct InfoSyntax_pointer {
    pub name: GeneralNames,
    pub hash: OPTIONAL<HASH>,
    pub _unrecognized: Vec<X690Element>,
}
impl InfoSyntax_pointer {
    pub fn new(name: GeneralNames, hash: OPTIONAL<HASH>, _unrecognized: Vec<X690Element>) -> Self {
        InfoSyntax_pointer {
            name,
            hash,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for InfoSyntax_pointer {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_InfoSyntax_pointer(el)
    }
}

pub const _rctl1_components_for_InfoSyntax_pointer: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "name",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "hash",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_InfoSyntax_pointer: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_InfoSyntax_pointer: &[ComponentSpec; 0] = &[];

pub fn _decode_InfoSyntax_pointer(el: &X690Element) -> ASN1Result<InfoSyntax_pointer> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InfoSyntax-pointer")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InfoSyntax_pointer,
        _eal_components_for_InfoSyntax_pointer,
        _rctl2_components_for_InfoSyntax_pointer,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut name_: OPTIONAL<GeneralNames> = None;
    let mut hash_: OPTIONAL<HASH> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "name" => name_ = Some(_decode_GeneralNames(_el)?),
            "hash" => hash_ = Some(_decode_HASH(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(InfoSyntax_pointer {
        name: name_.unwrap(),
        hash: hash_,
        _unrecognized,
    })
}

pub fn _encode_InfoSyntax_pointer(value_: &InfoSyntax_pointer) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_GeneralNames(&value_.name)?);
    if let Some(v_) = &value_.hash {
        components_.push(_encode_HASH(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_InfoSyntax_pointer(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InfoSyntax-pointer")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InfoSyntax_pointer,
        _eal_components_for_InfoSyntax_pointer,
        _rctl2_components_for_InfoSyntax_pointer,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "name" => _validate_GeneralNames(_el)?,
            "hash" => _validate_HASH(_el)?,
            _ => (),
        }
    }
    Ok(())
}
