#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PKI-Stub
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PKI-Stub`.
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
pub use crate::AuthenticationFramework::{
    FingerPrint, ScopeRestrictions, _decode_FingerPrint, _decode_HASH, _decode_SIGNED,
    _decode_ScopeRestrictions, _encode_FingerPrint, _encode_HASH, _encode_SIGNED,
    _encode_ScopeRestrictions, HASH, SIGNED,
};
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-wrprot            OBJECT IDENTIFIER ::= wrapperProtocolType
/// ```
///
///
pub fn id_wrprot() -> OBJECT_IDENTIFIER {
    wrapperProtocolType() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// wrapperProtocolType  OBJECT IDENTIFIER ::= {ds 43}
/// ```
///
///
pub fn wrapperProtocolType() -> OBJECT_IDENTIFIER {
    [ds(), Vec::<u32>::from([43])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ds                   OBJECT IDENTIFIER ::= {joint-iso-itu-t ds(5)}
/// ```
///
///
pub fn ds() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([joint_iso_itu_t, /* ds */ 5]) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo              OBJECT IDENTIFIER ::= algorithms
/// ```
///
///
pub fn id_algo() -> OBJECT_IDENTIFIER {
    algorithms() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// algorithms           OBJECT IDENTIFIER ::= {ds 44}
/// ```
///
///
pub fn algorithms() -> OBJECT_IDENTIFIER {
    [ds(), Vec::<u32>::from([44])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ALGORITHM ::= CLASS {
///   &Type          OPTIONAL,
///   &DynParms      OPTIONAL,
///   &id            OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   [PARMS         &Type]
///   [DYN-PARMS     &DynParms ]
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
/// AlgorithmWithInvoke{ALGORITHM:SupportedAlgorithms} ::= SEQUENCE {
///   algorithm       ALGORITHM.&id({SupportedAlgorithms}),
///   parameters  [0] ALGORITHM.&Type({SupportedAlgorithms}{@algorithm}) OPTIONAL,
///   dynamParms  [1] ALGORITHM.&DynParms({SupportedAlgorithms}{@algorithm}) OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AlgorithmWithInvoke {
    pub algorithm: OBJECT_IDENTIFIER,
    pub parameters: OPTIONAL<X690Element>,
    pub dynamParms: OPTIONAL<X690Element>,
    pub _unrecognized: Vec<X690Element>,
}
impl AlgorithmWithInvoke {
    pub fn new(
        algorithm: OBJECT_IDENTIFIER,
        parameters: OPTIONAL<X690Element>,
        dynamParms: OPTIONAL<X690Element>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AlgorithmWithInvoke {
            algorithm,
            parameters,
            dynamParms,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AlgorithmWithInvoke {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AlgorithmWithInvoke(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AlgorithmWithInvoke {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AlgorithmWithInvoke(el)
    }
}

pub const _rctl1_components_for_AlgorithmWithInvoke: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "parameters",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "dynamParms",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AlgorithmWithInvoke: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AlgorithmWithInvoke: &[ComponentSpec; 0] = &[];

pub fn _decode_AlgorithmWithInvoke(el: &X690Element) -> ASN1Result<AlgorithmWithInvoke> {
    |el_: &X690Element| -> ASN1Result<AlgorithmWithInvoke> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AlgorithmWithInvoke,
            _eal_components_for_AlgorithmWithInvoke,
            _rctl2_components_for_AlgorithmWithInvoke,
        )?;
        let algorithm = ber_decode_object_identifier(_components.get("algorithm").unwrap())?;
        let parameters: OPTIONAL<X690Element> = match _components.get("parameters") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<X690Element> {
                Ok(x690_identity(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let dynamParms: OPTIONAL<X690Element> = match _components.get("dynamParms") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<X690Element> {
                Ok(x690_identity(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(AlgorithmWithInvoke {
            algorithm,
            parameters,
            dynamParms,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AlgorithmWithInvoke(value_: &AlgorithmWithInvoke) -> ASN1Result<X690Element> {
    |value_: &AlgorithmWithInvoke| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(ber_encode_object_identifier(&value_.algorithm)?);
        if let Some(v_) = &value_.parameters {
            components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.dynamParms {
            components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
                ))
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AlgorithmIdentifier{ALGORITHM:SupportedAlgorithms} ::= SEQUENCE {
///   algorithm       ALGORITHM.&id({SupportedAlgorithms}),
///   parameters      ALGORITHM.&Type({SupportedAlgorithms}{@algorithm}) OPTIONAL,
///   ... }
/// ```
///
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
impl TryFrom<X690Element> for AlgorithmIdentifier {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AlgorithmIdentifier(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AlgorithmIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<AlgorithmIdentifier> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AlgorithmIdentifier,
            _eal_components_for_AlgorithmIdentifier,
            _rctl2_components_for_AlgorithmIdentifier,
        )?;
        let algorithm = ber_decode_object_identifier(_components.get("algorithm").unwrap())?;
        let parameters: OPTIONAL<X690Element> = match _components.get("parameters") {
            Some(c_) => Some(x690_identity(c_)?),
            _ => None,
        };
        Ok(AlgorithmIdentifier {
            algorithm,
            parameters,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AlgorithmIdentifier(value_: &AlgorithmIdentifier) -> ASN1Result<X690Element> {
    |value_: &AlgorithmIdentifier| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.algorithm)?);
        if let Some(v_) = &value_.parameters {
            components_.push(x690_identity(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AlgoInvoke{ALGORITHM:SupportedAlgorithms}  ::=
///     ALGORITHM.&DynParms({SupportedAlgorithms})
/// ```
pub type AlgoInvoke = X690Element; // ObjectClassFieldType

pub fn _decode_AlgoInvoke(el: &X690Element) -> ASN1Result<AlgoInvoke> {
    x690_identity(&el)
}

pub fn _encode_AlgoInvoke(value_: &AlgoInvoke) -> ASN1Result<X690Element> {
    x690_identity(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sha224WithRSAEncryptionAlgorithm ALGORITHM ::= { -- IETF RFC 5754
///   PARMS         NULL
///   IDENTIFIED BY {1 2 840 113549 1 11} }
/// ```
///
///
pub fn sha224WithRSAEncryptionAlgorithm() -> ALGORITHM {
    ALGORITHM {
        id: Vec::<u32>::from([1, 2, 840, 113549, 1, 11]), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedAlgorithms() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PkiPath  ::=  SEQUENCE SIZE (1..MAX) OF Certificate
/// ```
pub type PkiPath = Vec<Certificate>; // SequenceOfType

pub fn _decode_PkiPath(el: &X690Element) -> ASN1Result<PkiPath> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<Certificate>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<Certificate> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_Certificate(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_PkiPath(value_: &PkiPath) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<Certificate>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Certificate(&v)?);
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
/// Certificate  ::=  SIGNED{TBSCertificate}
/// ```
pub type Certificate = SIGNED<TBSCertificate>; // DefinedType

pub fn _decode_Certificate(el: &X690Element) -> ASN1Result<Certificate> {
    _decode_SIGNED::<TBSCertificate>(_decode_TBSCertificate, &el)
}

pub fn _encode_Certificate(value_: &Certificate) -> ASN1Result<X690Element> {
    _encode_SIGNED::<TBSCertificate>(_encode_TBSCertificate, &value_)
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
///   --[[2:  if present, version shall be v2 or v3
///   subjectUniqueIdentifier  [2] IMPLICIT UniqueIdentifier OPTIONAL--]]--,
///   --[[3:  if present, version shall be v2 or v3
///   extensions               [3]  Extensions OPTIONAL --]]
///   -- If present, version shall be v3]]
///  } (CONSTRAINED BY { -- shall be DER encoded -- } )
/// ```
///
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
    pub subjectUniqueIdentifier: OPTIONAL<UniqueIdentifier>,
    pub extensions: OPTIONAL<Extensions>,
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
        subjectUniqueIdentifier: OPTIONAL<UniqueIdentifier>,
        extensions: OPTIONAL<Extensions>,
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
            subjectUniqueIdentifier,
            extensions,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> Version {
        Version_v1
    }
}
impl TryFrom<X690Element> for TBSCertificate {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TBSCertificate(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TBSCertificate {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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

pub const _eal_components_for_TBSCertificate: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "subjectUniqueIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub fn _decode_TBSCertificate(el: &X690Element) -> ASN1Result<TBSCertificate> {
    |el_: &X690Element| -> ASN1Result<TBSCertificate> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TBSCertificate,
            _eal_components_for_TBSCertificate,
            _rctl2_components_for_TBSCertificate,
        )?;
        let version: OPTIONAL<Version> = match _components.get("version") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Version> {
                Ok(_decode_Version(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        let signature = _decode_AlgorithmIdentifier(_components.get("signature").unwrap())?;
        let issuer = _decode_Name(_components.get("issuer").unwrap())?;
        let validity = _decode_Validity(_components.get("validity").unwrap())?;
        let subject = _decode_Name(_components.get("subject").unwrap())?;
        let subjectPublicKeyInfo =
            _decode_SubjectPublicKeyInfo(_components.get("subjectPublicKeyInfo").unwrap())?;
        let issuerUniqueIdentifier: OPTIONAL<UniqueIdentifier> =
            match _components.get("issuerUniqueIdentifier") {
                Some(c_) => Some(_decode_UniqueIdentifier(c_)?),
                _ => None,
            };
        let subjectUniqueIdentifier: OPTIONAL<UniqueIdentifier> =
            match _components.get("subjectUniqueIdentifier") {
                Some(c_) => Some(_decode_UniqueIdentifier(c_)?),
                _ => None,
            };
        let extensions: OPTIONAL<Extensions> = match _components.get("extensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Extensions> {
                Ok(_decode_Extensions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(TBSCertificate {
            version,
            serialNumber,
            signature,
            issuer,
            validity,
            subject,
            subjectPublicKeyInfo,
            issuerUniqueIdentifier,
            subjectUniqueIdentifier,
            extensions,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TBSCertificate(value_: &TBSCertificate) -> ASN1Result<X690Element> {
    |value_: &TBSCertificate| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(20);
        if let Some(v_) = &value_.version {
            if *v_ != TBSCertificate::_default_value_for_version() {
                components_.push(|v_1: &Version| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Version(&v_1)?))),
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
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.subjectUniqueIdentifier {
            components_.push(|v_1: &UniqueIdentifier| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_UniqueIdentifier(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.extensions {
            components_.push(|v_1: &Extensions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Extensions(&v_1)?))),
                ))
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Version  ::=  INTEGER {v1(0), v2(1), v3(2)}
/// ```
pub type Version = INTEGER;

pub const Version_v1: Version = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const Version_v2: Version = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Version_v3: Version = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Version(el: &X690Element) -> ASN1Result<Version> {
    ber_decode_integer(&el)
}

pub fn _encode_Version(value_: &Version) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateSerialNumber  ::=  INTEGER
/// ```
pub type CertificateSerialNumber = INTEGER;

pub fn _decode_CertificateSerialNumber(el: &X690Element) -> ASN1Result<CertificateSerialNumber> {
    ber_decode_integer(&el)
}

pub fn _encode_CertificateSerialNumber(
    value_: &CertificateSerialNumber,
) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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
impl TryFrom<X690Element> for Validity {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Validity(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Validity {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Validity> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Validity,
            _eal_components_for_Validity,
            _rctl2_components_for_Validity,
        )?;
        let notBefore = _decode_Time(_components.get("notBefore").unwrap())?;
        let notAfter = _decode_Time(_components.get("notAfter").unwrap())?;
        Ok(Validity {
            notBefore,
            notAfter,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Validity(value_: &Validity) -> ASN1Result<X690Element> {
    |value_: &Validity| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Time(&value_.notBefore)?);
        components_.push(_encode_Time(&value_.notAfter)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for SubjectPublicKeyInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SubjectPublicKeyInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SubjectPublicKeyInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SubjectPublicKeyInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SubjectPublicKeyInfo,
            _eal_components_for_SubjectPublicKeyInfo,
            _rctl2_components_for_SubjectPublicKeyInfo,
        )?;
        let algorithm = _decode_AlgorithmIdentifier(_components.get("algorithm").unwrap())?;
        let subjectPublicKey = _decode_PublicKey(_components.get("subjectPublicKey").unwrap())?;
        Ok(SubjectPublicKeyInfo {
            algorithm,
            subjectPublicKey,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SubjectPublicKeyInfo(value_: &SubjectPublicKeyInfo) -> ASN1Result<X690Element> {
    |value_: &SubjectPublicKeyInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AlgorithmIdentifier(&value_.algorithm)?);
        components_.push(_encode_PublicKey(&value_.subjectPublicKey)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PublicKey  ::=  BIT STRING
/// ```
pub type PublicKey = BIT_STRING;

pub fn _decode_PublicKey(el: &X690Element) -> ASN1Result<PublicKey> {
    ber_decode_bit_string(&el)
}

pub fn _encode_PublicKey(value_: &PublicKey) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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

impl TryFrom<X690Element> for Time {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Time(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Time {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Time(el)
    }
}

pub fn _decode_Time(el: &X690Element) -> ASN1Result<Time> {
    |el: &X690Element| -> ASN1Result<Time> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 23) => Ok(Time::utcTime(ber_decode_utc_time(&el)?)),
            (TagClass::UNIVERSAL, 24) => {
                Ok(Time::generalizedTime(ber_decode_generalized_time(&el)?))
            }
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_Time(value_: &Time) -> ASN1Result<X690Element> {
    |value: &Time| -> ASN1Result<X690Element> {
        match value {
            Time::utcTime(v) => ber_encode_utc_time(&v),
            Time::generalizedTime(v) => ber_encode_generalized_time(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniqueIdentifier  ::=  BIT STRING
/// ```
pub type UniqueIdentifier = BIT_STRING;

pub fn _decode_UniqueIdentifier(el: &X690Element) -> ASN1Result<UniqueIdentifier> {
    ber_decode_bit_string(&el)
}

pub fn _encode_UniqueIdentifier(value_: &UniqueIdentifier) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Extensions  ::=  SEQUENCE SIZE (1..MAX) OF Extension
/// ```
pub type Extensions = Vec<Extension>; // SequenceOfType

pub fn _decode_Extensions(el: &X690Element) -> ASN1Result<Extensions> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<Extension>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<Extension> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_Extension(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_Extensions(value_: &Extensions) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<Extension>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Extension(&v)?);
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
/// Extension ::= SEQUENCE {
///   extnId     EXTENSION.&id({ExtensionSet}),
///   critical   BOOLEAN DEFAULT FALSE,
///   extnValue  OCTET STRING
///     (CONTAINING EXTENSION.&ExtnType({ExtensionSet}{@extnId})
///        ENCODED BY der),
///   ... }
/// ```
///
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
impl TryFrom<X690Element> for Extension {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Extension(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Extension {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Extension> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Extension,
            _eal_components_for_Extension,
            _rctl2_components_for_Extension,
        )?;
        let extnId = ber_decode_object_identifier(_components.get("extnId").unwrap())?;
        let critical: OPTIONAL<BOOLEAN> = match _components.get("critical") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let extnValue = ber_decode_octet_string(_components.get("extnValue").unwrap())?;
        Ok(Extension {
            extnId,
            critical,
            extnValue,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Extension(value_: &Extension) -> ASN1Result<X690Element> {
    |value_: &Extension| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(ber_encode_object_identifier(&value_.extnId)?);
        if let Some(v_) = &value_.critical {
            if *v_ != Extension::_default_value_for_critical() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        components_.push(ber_encode_octet_string(&value_.extnValue)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// der OBJECT IDENTIFIER ::= {joint-iso-itu-t asn1(1) ber-derived(2) distinguished-encoding(1)}
/// ```
///
///
pub fn der() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([
        joint_iso_itu_t,
        /* asn1 */ 1,
        /* ber-derived */ 2,
        /* distinguished-encoding */ 1,
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtensionSet EXTENSION ::= {...}
/// ```
///
///
pub fn ExtensionSet() -> Vec<EXTENSION> {
    Vec::from([])
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
/// Name  ::=  CHOICE { -- only one possibility for now -- rdnSequence  RDNSequence }
/// ```
#[derive(Debug, Clone)]
pub enum Name {
    rdnSequence(RDNSequence),
}

impl TryFrom<X690Element> for Name {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Name(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Name {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Name(el)
    }
}

pub fn _decode_Name(el: &X690Element) -> ASN1Result<Name> {
    |el: &X690Element| -> ASN1Result<Name> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(Name::rdnSequence(_decode_RDNSequence(&el)?)),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_Name(value_: &Name) -> ASN1Result<X690Element> {
    |value: &Name| -> ASN1Result<X690Element> {
        match value {
            Name::rdnSequence(v) => _encode_RDNSequence(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RDNSequence  ::=  SEQUENCE OF RelativeDistinguishedName
/// ```
pub type RDNSequence = Vec<RelativeDistinguishedName>; // SequenceOfType

pub fn _decode_RDNSequence(el: &X690Element) -> ASN1Result<RDNSequence> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<RelativeDistinguishedName>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<RelativeDistinguishedName> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_RelativeDistinguishedName(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_RDNSequence(value_: &RDNSequence) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<RelativeDistinguishedName>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_RelativeDistinguishedName(&v)?);
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
/// RelativeDistinguishedName  ::=  SET SIZE (1..MAX) OF AttributeTypeAndValue
/// ```
pub type RelativeDistinguishedName = Vec<AttributeTypeAndValue>; // SetOfType

pub fn _decode_RelativeDistinguishedName(
    el: &X690Element,
) -> ASN1Result<RelativeDistinguishedName> {
    |el: &X690Element| -> ASN1Result<SET_OF<AttributeTypeAndValue>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<AttributeTypeAndValue> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AttributeTypeAndValue(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_RelativeDistinguishedName(
    value_: &RelativeDistinguishedName,
) -> ASN1Result<X690Element> {
    |value_: &SET_OF<AttributeTypeAndValue>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AttributeTypeAndValue(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DistinguishedName  ::=  RDNSequence
/// ```
pub type DistinguishedName = RDNSequence; // DefinedType

pub fn _decode_DistinguishedName(el: &X690Element) -> ASN1Result<DistinguishedName> {
    _decode_RDNSequence(&el)
}

pub fn _encode_DistinguishedName(value_: &DistinguishedName) -> ASN1Result<X690Element> {
    _encode_RDNSequence(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeTypeAndValue ::= SEQUENCE {
///   type                  ATTRIBUTE.&id, --({SupportedAttributes}),
///   value                 ATTRIBUTE.&type, --({SupportedAttributes}{@type}),
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeTypeAndValue {
    pub type_: OBJECT_IDENTIFIER,
    pub value: UTF8String,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeTypeAndValue {
    pub fn new(type_: OBJECT_IDENTIFIER, value: UTF8String, _unrecognized: Vec<X690Element>) -> Self {
        AttributeTypeAndValue {
            type_,
            value,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttributeTypeAndValue {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeTypeAndValue(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeTypeAndValue {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeTypeAndValue(el)
    }
}

pub const _rctl1_components_for_AttributeTypeAndValue: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "value",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 12)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeTypeAndValue: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeTypeAndValue: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeTypeAndValue(el: &X690Element) -> ASN1Result<AttributeTypeAndValue> {
    |el_: &X690Element| -> ASN1Result<AttributeTypeAndValue> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeTypeAndValue,
            _eal_components_for_AttributeTypeAndValue,
            _rctl2_components_for_AttributeTypeAndValue,
        )?;
        let type_ = ber_decode_object_identifier(_components.get("type").unwrap())?;
        let value = ber_decode_utf8_string(_components.get("value").unwrap())?;
        Ok(AttributeTypeAndValue {
            type_,
            value,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeTypeAndValue(value_: &AttributeTypeAndValue) -> ASN1Result<X690Element> {
    |value_: &AttributeTypeAndValue| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.type_)?);
        components_.push(ber_encode_utf8_string(&value_.value)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedAttributes ATTRIBUTE ::= {...}
/// ```
///
///
pub fn SupportedAttributes() -> Vec<ATTRIBUTE> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ATTRIBUTE ::= CLASS {
///   &type                     UTF8String,
///   &id                       OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   WITH SYNTAX               &type
///   ID                        &id }
/// ```
///
#[derive(Debug)]
pub struct ATTRIBUTE {
    pub type_: UTF8String,
    pub id: OBJECT_IDENTIFIER,
}
impl ATTRIBUTE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Attribute {ATTRIBUTE:SupportedAttributes} ::= SEQUENCE {
///   type                ATTRIBUTE.&id({SupportedAttributes}),
///   values              SET SIZE (0..MAX) OF ATTRIBUTE.&type({SupportedAttributes}{@type}),
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Attribute {
    pub type_: OBJECT_IDENTIFIER,
    pub values: Vec<UTF8String>,
    pub _unrecognized: Vec<X690Element>,
}
impl Attribute {
    pub fn new(
        type_: OBJECT_IDENTIFIER,
        values: Vec<UTF8String>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Attribute {
            type_,
            values,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for Attribute {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Attribute(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Attribute {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Attribute(el)
    }
}

pub const _rctl1_components_for_Attribute: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "values",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Attribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Attribute: &[ComponentSpec; 0] = &[];

pub fn _decode_Attribute(el: &X690Element) -> ASN1Result<Attribute> {
    |el_: &X690Element| -> ASN1Result<Attribute> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Attribute,
            _eal_components_for_Attribute,
            _rctl2_components_for_Attribute,
        )?;
        let type_ = ber_decode_object_identifier(_components.get("type").unwrap())?;
        let values = |el: &X690Element| -> ASN1Result<SET_OF<UTF8String>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SET_OF<UTF8String> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(ber_decode_utf8_string(el)?);
            }
            Ok(items)
        }(_components.get("values").unwrap())?;
        Ok(Attribute {
            type_,
            values,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Attribute(value_: &Attribute) -> ASN1Result<X690Element> {
    |value_: &Attribute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.type_)?);
        components_.push(|value_: &SET_OF<UTF8String>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(ber_encode_utf8_string(&v)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                Arc::new(X690Encoding::Constructed(children)),
            ))
        }(&value_.values)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeCertificate  ::=  SIGNED{TBSAttributeCertificate}
/// ```
pub type AttributeCertificate = SIGNED<TBSAttributeCertificate>; // DefinedType

pub fn _decode_AttributeCertificate(el: &X690Element) -> ASN1Result<AttributeCertificate> {
    _decode_SIGNED::<TBSAttributeCertificate>(_decode_TBSAttributeCertificate, &el)
}

pub fn _encode_AttributeCertificate(value_: &AttributeCertificate) -> ASN1Result<X690Element> {
    _encode_SIGNED::<TBSAttributeCertificate>(_encode_TBSAttributeCertificate, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBSAttributeCertificate ::= SEQUENCE {
///   version                 AttCertVersion, -- version is v2
///   holder                  Holder,
///   issuer                  AttCertIssuer,
///   signature               AlgorithmIdentifier{{SupportedAlgorithms}},
///   serialNumber            CertificateSerialNumber,
///   attrCertValidityPeriod  AttCertValidityPeriod,
///   attributes              SEQUENCE OF Attribute{{SupportedAttributes}},
///   issuerUniqueID          UniqueIdentifier OPTIONAL,
///   ...,
///   ...,
///   extensions              Extensions OPTIONAL
///  }  (CONSTRAINED BY { -- shall be DER encoded -- } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TBSAttributeCertificate {
    pub version: AttCertVersion,
    pub holder: Holder,
    pub issuer: AttCertIssuer,
    pub signature: AlgorithmIdentifier,
    pub serialNumber: CertificateSerialNumber,
    pub attrCertValidityPeriod: AttCertValidityPeriod,
    pub attributes: Vec<Attribute>,
    pub issuerUniqueID: OPTIONAL<UniqueIdentifier>,
    pub _unrecognized: Vec<X690Element>,
    pub extensions: OPTIONAL<Extensions>,
}
impl TBSAttributeCertificate {
    pub fn new(
        version: AttCertVersion,
        holder: Holder,
        issuer: AttCertIssuer,
        signature: AlgorithmIdentifier,
        serialNumber: CertificateSerialNumber,
        attrCertValidityPeriod: AttCertValidityPeriod,
        attributes: Vec<Attribute>,
        issuerUniqueID: OPTIONAL<UniqueIdentifier>,
        _unrecognized: Vec<X690Element>,
        extensions: OPTIONAL<Extensions>,
    ) -> Self {
        TBSAttributeCertificate {
            version,
            holder,
            issuer,
            signature,
            serialNumber,
            attrCertValidityPeriod,
            attributes,
            issuerUniqueID,
            extensions,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for TBSAttributeCertificate {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TBSAttributeCertificate(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TBSAttributeCertificate {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TBSAttributeCertificate(el)
    }
}

pub const _rctl1_components_for_TBSAttributeCertificate: &[ComponentSpec; 8] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "holder",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuer",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
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
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attrCertValidityPeriod",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributes",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuerUniqueID",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TBSAttributeCertificate: &[ComponentSpec; 1] =
    &[ComponentSpec::new(
        "extensions",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    )];

pub const _eal_components_for_TBSAttributeCertificate: &[ComponentSpec; 0] = &[];

pub fn _decode_TBSAttributeCertificate(el: &X690Element) -> ASN1Result<TBSAttributeCertificate> {
    |el_: &X690Element| -> ASN1Result<TBSAttributeCertificate> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TBSAttributeCertificate,
            _eal_components_for_TBSAttributeCertificate,
            _rctl2_components_for_TBSAttributeCertificate,
        )?;
        let version = _decode_AttCertVersion(_components.get("version").unwrap())?;
        let holder = _decode_Holder(_components.get("holder").unwrap())?;
        let issuer = _decode_AttCertIssuer(_components.get("issuer").unwrap())?;
        let signature = _decode_AlgorithmIdentifier(_components.get("signature").unwrap())?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        let attrCertValidityPeriod =
            _decode_AttCertValidityPeriod(_components.get("attrCertValidityPeriod").unwrap())?;
        let attributes = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<Attribute>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_Attribute(el)?);
            }
            Ok(items)
        }(_components.get("attributes").unwrap())?;
        let issuerUniqueID: OPTIONAL<UniqueIdentifier> = match _components.get("issuerUniqueID") {
            Some(c_) => Some(_decode_UniqueIdentifier(c_)?),
            _ => None,
        };
        let extensions: OPTIONAL<Extensions> = match _components.get("extensions") {
            Some(c_) => Some(_decode_Extensions(c_)?),
            _ => None,
        };
        Ok(TBSAttributeCertificate {
            version,
            holder,
            issuer,
            signature,
            serialNumber,
            attrCertValidityPeriod,
            attributes,
            issuerUniqueID,
            _unrecognized,
            extensions,
        })
    }(&el)
}

pub fn _encode_TBSAttributeCertificate(
    value_: &TBSAttributeCertificate,
) -> ASN1Result<X690Element> {
    |value_: &TBSAttributeCertificate| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(19);
        components_.push(_encode_AttCertVersion(&value_.version)?);
        components_.push(_encode_Holder(&value_.holder)?);
        components_.push(_encode_AttCertIssuer(&value_.issuer)?);
        components_.push(_encode_AlgorithmIdentifier(&value_.signature)?);
        components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
        components_.push(_encode_AttCertValidityPeriod(
            &value_.attrCertValidityPeriod,
        )?);
        components_.push(
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
            }(&value_.attributes)?,
        );
        if let Some(v_) = &value_.issuerUniqueID {
            components_.push(_encode_UniqueIdentifier(&v_)?);
        }
        if let Some(v_) = &value_.extensions {
            components_.push(_encode_Extensions(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttCertVersion  ::=  INTEGER {v2(1)}
/// ```
pub type AttCertVersion = INTEGER;

pub const AttCertVersion_v2: AttCertVersion = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_AttCertVersion(el: &X690Element) -> ASN1Result<AttCertVersion> {
    ber_decode_integer(&el)
}

pub fn _encode_AttCertVersion(value_: &AttCertVersion) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Holder ::= SEQUENCE {
///   baseCertificateID  [0]  IssuerSerial OPTIONAL,
///   entityName         [1]  GeneralNames OPTIONAL,
///   objectDigestInfo   [2]  ObjectDigestInfo OPTIONAL }
///   (WITH COMPONENTS {..., baseCertificateID  PRESENT } |
///    WITH COMPONENTS {..., entityName  PRESENT } |
///    WITH COMPONENTS {..., objectDigestInfo  PRESENT } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Holder {
    pub baseCertificateID: OPTIONAL<IssuerSerial>,
    pub entityName: OPTIONAL<GeneralNames>,
    pub objectDigestInfo: OPTIONAL<ObjectDigestInfo>,
}
impl Holder {
    pub fn new(
        baseCertificateID: OPTIONAL<IssuerSerial>,
        entityName: OPTIONAL<GeneralNames>,
        objectDigestInfo: OPTIONAL<ObjectDigestInfo>,
    ) -> Self {
        Holder {
            baseCertificateID,
            entityName,
            objectDigestInfo,
        }
    }
}
impl Default for Holder {
    fn default() -> Self {
        Holder {
            baseCertificateID: None,
            entityName: None,
            objectDigestInfo: None,
        }
    }
}
impl TryFrom<X690Element> for Holder {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Holder(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Holder {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Holder(el)
    }
}

pub const _rctl1_components_for_Holder: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "baseCertificateID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entityName",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "objectDigestInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Holder: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Holder: &[ComponentSpec; 0] = &[];

pub fn _decode_Holder(el: &X690Element) -> ASN1Result<Holder> {
    |el_: &X690Element| -> ASN1Result<Holder> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Holder,
            _eal_components_for_Holder,
            _rctl2_components_for_Holder,
        )?;
        let baseCertificateID: OPTIONAL<IssuerSerial> = match _components.get("baseCertificateID") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<IssuerSerial> {
                Ok(_decode_IssuerSerial(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entityName: OPTIONAL<GeneralNames> = match _components.get("entityName") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralNames> {
                Ok(_decode_GeneralNames(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let objectDigestInfo: OPTIONAL<ObjectDigestInfo> = match _components.get("objectDigestInfo")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ObjectDigestInfo> {
                Ok(_decode_ObjectDigestInfo(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(Holder {
            baseCertificateID,
            entityName,
            objectDigestInfo,
        })
    }(&el)
}

pub fn _encode_Holder(value_: &Holder) -> ASN1Result<X690Element> {
    |value_: &Holder| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        if let Some(v_) = &value_.baseCertificateID {
            components_.push(|v_1: &IssuerSerial| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_IssuerSerial(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entityName {
            components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralNames(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.objectDigestInfo {
            components_.push(|v_1: &ObjectDigestInfo| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ObjectDigestInfo(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IssuerSerial ::= SEQUENCE {
///   issuer     GeneralNames,
///   serial     CertificateSerialNumber,
///   issuerUID  UniqueIdentifier OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct IssuerSerial {
    pub issuer: GeneralNames,
    pub serial: CertificateSerialNumber,
    pub issuerUID: OPTIONAL<UniqueIdentifier>,
    pub _unrecognized: Vec<X690Element>,
}
impl IssuerSerial {
    pub fn new(
        issuer: GeneralNames,
        serial: CertificateSerialNumber,
        issuerUID: OPTIONAL<UniqueIdentifier>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        IssuerSerial {
            issuer,
            serial,
            issuerUID,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for IssuerSerial {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IssuerSerial(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IssuerSerial {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_IssuerSerial(el)
    }
}

pub const _rctl1_components_for_IssuerSerial: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "issuer",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "serial",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuerUID",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IssuerSerial: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IssuerSerial: &[ComponentSpec; 0] = &[];

pub fn _decode_IssuerSerial(el: &X690Element) -> ASN1Result<IssuerSerial> {
    |el_: &X690Element| -> ASN1Result<IssuerSerial> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IssuerSerial,
            _eal_components_for_IssuerSerial,
            _rctl2_components_for_IssuerSerial,
        )?;
        let issuer = _decode_GeneralNames(_components.get("issuer").unwrap())?;
        let serial = _decode_CertificateSerialNumber(_components.get("serial").unwrap())?;
        let issuerUID: OPTIONAL<UniqueIdentifier> = match _components.get("issuerUID") {
            Some(c_) => Some(_decode_UniqueIdentifier(c_)?),
            _ => None,
        };
        Ok(IssuerSerial {
            issuer,
            serial,
            issuerUID,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_IssuerSerial(value_: &IssuerSerial) -> ASN1Result<X690Element> {
    |value_: &IssuerSerial| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_GeneralNames(&value_.issuer)?);
        components_.push(_encode_CertificateSerialNumber(&value_.serial)?);
        if let Some(v_) = &value_.issuerUID {
            components_.push(_encode_UniqueIdentifier(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ObjectDigestInfo ::= SEQUENCE {
///   digestedObjectType   ENUMERATED {
///     publicKey        (0),
///     publicKeyCert    (1),
///     otherObjectTypes (2)},
///   otherObjectTypeID   OBJECT IDENTIFIER OPTIONAL,
///   digestAlgorithm     AlgorithmIdentifier{{SupportedAlgorithms}},
///   objectDigest        BIT STRING,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ObjectDigestInfo {
    pub digestedObjectType: ObjectDigestInfo_digestedObjectType,
    pub otherObjectTypeID: OPTIONAL<OBJECT_IDENTIFIER>,
    pub digestAlgorithm: AlgorithmIdentifier,
    pub objectDigest: BIT_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl ObjectDigestInfo {
    pub fn new(
        digestedObjectType: ObjectDigestInfo_digestedObjectType,
        otherObjectTypeID: OPTIONAL<OBJECT_IDENTIFIER>,
        digestAlgorithm: AlgorithmIdentifier,
        objectDigest: BIT_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ObjectDigestInfo {
            digestedObjectType,
            otherObjectTypeID,
            digestAlgorithm,
            objectDigest,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ObjectDigestInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ObjectDigestInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ObjectDigestInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ObjectDigestInfo(el)
    }
}

pub const _rctl1_components_for_ObjectDigestInfo: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "digestedObjectType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "otherObjectTypeID",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "digestAlgorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "objectDigest",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ObjectDigestInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ObjectDigestInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_ObjectDigestInfo(el: &X690Element) -> ASN1Result<ObjectDigestInfo> {
    |el_: &X690Element| -> ASN1Result<ObjectDigestInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ObjectDigestInfo,
            _eal_components_for_ObjectDigestInfo,
            _rctl2_components_for_ObjectDigestInfo,
        )?;
        let digestedObjectType = _decode_ObjectDigestInfo_digestedObjectType(
            _components.get("digestedObjectType").unwrap(),
        )?;
        let otherObjectTypeID: OPTIONAL<OBJECT_IDENTIFIER> =
            match _components.get("otherObjectTypeID") {
                Some(c_) => Some(ber_decode_object_identifier(c_)?),
                _ => None,
            };
        let digestAlgorithm =
            _decode_AlgorithmIdentifier(_components.get("digestAlgorithm").unwrap())?;
        let objectDigest = ber_decode_bit_string(_components.get("objectDigest").unwrap())?;
        Ok(ObjectDigestInfo {
            digestedObjectType,
            otherObjectTypeID,
            digestAlgorithm,
            objectDigest,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ObjectDigestInfo(value_: &ObjectDigestInfo) -> ASN1Result<X690Element> {
    |value_: &ObjectDigestInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(_encode_ObjectDigestInfo_digestedObjectType(
            &value_.digestedObjectType,
        )?);
        if let Some(v_) = &value_.otherObjectTypeID {
            components_.push(ber_encode_object_identifier(&v_)?);
        }
        components_.push(_encode_AlgorithmIdentifier(&value_.digestAlgorithm)?);
        components_.push(ber_encode_bit_string(&value_.objectDigest)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttCertIssuer ::= [0]  SEQUENCE {
///   issuerName              GeneralNames OPTIONAL,
///   baseCertificateID  [0]  IssuerSerial OPTIONAL,
///   objectDigestInfo   [1]  ObjectDigestInfo OPTIONAL,
///   ... }
///   (WITH COMPONENTS {..., issuerName  PRESENT } |
///    WITH COMPONENTS {..., baseCertificateID  PRESENT } |
///    WITH COMPONENTS {..., objectDigestInfo  PRESENT } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttCertIssuer {
    pub issuerName: OPTIONAL<GeneralNames>,
    pub baseCertificateID: OPTIONAL<IssuerSerial>,
    pub objectDigestInfo: OPTIONAL<ObjectDigestInfo>,
    pub _unrecognized: Vec<X690Element>,
}
impl AttCertIssuer {
    pub fn new(
        issuerName: OPTIONAL<GeneralNames>,
        baseCertificateID: OPTIONAL<IssuerSerial>,
        objectDigestInfo: OPTIONAL<ObjectDigestInfo>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttCertIssuer {
            issuerName,
            baseCertificateID,
            objectDigestInfo,
            _unrecognized,
        }
    }
}
impl Default for AttCertIssuer {
    fn default() -> Self {
        AttCertIssuer {
            issuerName: None,
            baseCertificateID: None,
            objectDigestInfo: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for AttCertIssuer {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttCertIssuer(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttCertIssuer {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttCertIssuer(el)
    }
}

pub const _rctl1_components_for_AttCertIssuer: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "issuerName",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "baseCertificateID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "objectDigestInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttCertIssuer: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttCertIssuer: &[ComponentSpec; 0] = &[];

pub fn _decode_AttCertIssuer(el: &X690Element) -> ASN1Result<AttCertIssuer> {
    |el_: &X690Element| -> ASN1Result<AttCertIssuer> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttCertIssuer,
            _eal_components_for_AttCertIssuer,
            _rctl2_components_for_AttCertIssuer,
        )?;
        let issuerName: OPTIONAL<GeneralNames> = match _components.get("issuerName") {
            Some(c_) => Some(_decode_GeneralNames(c_)?),
            _ => None,
        };
        let baseCertificateID: OPTIONAL<IssuerSerial> = match _components.get("baseCertificateID") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<IssuerSerial> {
                Ok(_decode_IssuerSerial(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let objectDigestInfo: OPTIONAL<ObjectDigestInfo> = match _components.get("objectDigestInfo")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ObjectDigestInfo> {
                Ok(_decode_ObjectDigestInfo(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(AttCertIssuer {
            issuerName,
            baseCertificateID,
            objectDigestInfo,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttCertIssuer(value_: &AttCertIssuer) -> ASN1Result<X690Element> {
    |v_1: &AttCertIssuer| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &AttCertIssuer| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(13);
            if let Some(v_) = &value_.issuerName {
                components_.push(_encode_GeneralNames(&v_)?);
            }
            if let Some(v_) = &value_.baseCertificateID {
                components_.push(|v_1: &IssuerSerial| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_IssuerSerial(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
            if let Some(v_) = &value_.objectDigestInfo {
                components_.push(|v_1: &ObjectDigestInfo| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ObjectDigestInfo(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::CONTEXT;
        el_1.tag_number = 0;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttCertValidityPeriod ::= SEQUENCE {
///   notBeforeTime  GeneralizedTime,
///   notAfterTime   GeneralizedTime,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttCertValidityPeriod {
    pub notBeforeTime: GeneralizedTime,
    pub notAfterTime: GeneralizedTime,
    pub _unrecognized: Vec<X690Element>,
}
impl AttCertValidityPeriod {
    pub fn new(
        notBeforeTime: GeneralizedTime,
        notAfterTime: GeneralizedTime,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttCertValidityPeriod {
            notBeforeTime,
            notAfterTime,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttCertValidityPeriod {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttCertValidityPeriod(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttCertValidityPeriod {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttCertValidityPeriod(el)
    }
}

pub const _rctl1_components_for_AttCertValidityPeriod: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "notBeforeTime",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "notAfterTime",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttCertValidityPeriod: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttCertValidityPeriod: &[ComponentSpec; 0] = &[];

pub fn _decode_AttCertValidityPeriod(el: &X690Element) -> ASN1Result<AttCertValidityPeriod> {
    |el_: &X690Element| -> ASN1Result<AttCertValidityPeriod> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttCertValidityPeriod,
            _eal_components_for_AttCertValidityPeriod,
            _rctl2_components_for_AttCertValidityPeriod,
        )?;
        let notBeforeTime = ber_decode_generalized_time(_components.get("notBeforeTime").unwrap())?;
        let notAfterTime = ber_decode_generalized_time(_components.get("notAfterTime").unwrap())?;
        Ok(AttCertValidityPeriod {
            notBeforeTime,
            notAfterTime,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttCertValidityPeriod(value_: &AttCertValidityPeriod) -> ASN1Result<X690Element> {
    |value_: &AttCertValidityPeriod| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_generalized_time(&value_.notBeforeTime)?);
        components_.push(ber_encode_generalized_time(&value_.notAfterTime)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
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
/// --x400Address                [3]  ORAddress,
///   directoryName              [4]  Name,
/// --ediPartyName               [5]  EDIPartyName,
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
    directoryName(Name),
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
            (TagClass::CONTEXT, 4) => Ok(GeneralName::directoryName(
                |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(&el)?,
            )),
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
            GeneralName::directoryName(v) => |v_1: &Name| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Name(&v_1)?))),
                ))
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
/// CertAVL  ::=  SIGNED {TBSCertAVL}
/// ```
pub type CertAVL = SIGNED<TBSCertAVL>; // DefinedType

pub fn _decode_CertAVL(el: &X690Element) -> ASN1Result<CertAVL> {
    _decode_SIGNED::<TBSCertAVL>(_decode_TBSCertAVL, &el)
}

pub fn _encode_CertAVL(value_: &CertAVL) -> ASN1Result<X690Element> {
    _encode_SIGNED::<TBSCertAVL>(_encode_TBSCertAVL, &value_)
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
impl TryFrom<X690Element> for TBSCertAVL {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TBSCertAVL(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TBSCertAVL {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<TBSCertAVL> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TBSCertAVL,
            _eal_components_for_TBSCertAVL,
            _rctl2_components_for_TBSCertAVL,
        )?;
        let version: OPTIONAL<Version> = match _components.get("version") {
            Some(c_) => Some(_decode_Version(c_)?),
            _ => None,
        };
        let serialNumber: OPTIONAL<AvlSerialNumber> = match _components.get("serialNumber") {
            Some(c_) => Some(_decode_AvlSerialNumber(c_)?),
            _ => None,
        };
        let signature = _decode_AlgorithmIdentifier(_components.get("signature").unwrap())?;
        let issuer = _decode_Name(_components.get("issuer").unwrap())?;
        let constrained = ber_decode_boolean(_components.get("constrained").unwrap())?;
        let entries = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<TBSCertAVL_entries_Item>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<TBSCertAVL_entries_Item> =
                Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_TBSCertAVL_entries_Item(el)?);
            }
            Ok(items)
        }(_components.get("entries").unwrap())?;
        let avlExtensions: OPTIONAL<Extensions> = match _components.get("avlExtensions") {
            Some(c_) => Some(_decode_Extensions(c_)?),
            _ => None,
        };
        Ok(TBSCertAVL {
            version,
            serialNumber,
            signature,
            issuer,
            constrained,
            entries,
            _unrecognized,
            avlExtensions,
        })
    }(&el)
}

pub fn _encode_TBSCertAVL(value_: &TBSCertAVL) -> ASN1Result<X690Element> {
    |value_: &TBSCertAVL| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        if let Some(v_) = &value_.version {
            if *v_ != TBSCertAVL::_default_value_for_version() {
                components_.push(|v_1: &Version| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_Version(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.serialNumber {
            components_.push(_encode_AvlSerialNumber(&v_)?);
        }
        components_.push(_encode_AlgorithmIdentifier(&value_.signature)?);
        components_.push(_encode_Name(&value_.issuer)?);
        components_.push(ber_encode_boolean(&value_.constrained)?);
        components_.push(
            |value_: &SEQUENCE_OF<TBSCertAVL_entries_Item>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_TBSCertAVL_entries_Item(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.entries)?,
        );
        if let Some(v_) = &value_.avlExtensions {
            components_.push(_encode_Extensions(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AvlSerialNumber  ::=  INTEGER (0..MAX)
/// ```
pub type AvlSerialNumber = INTEGER;

pub fn _decode_AvlSerialNumber(el: &X690Element) -> ASN1Result<AvlSerialNumber> {
    ber_decode_integer(&el)
}

pub fn _encode_AvlSerialNumber(value_: &AvlSerialNumber) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PKCertIdentifier  ::=  CHOICE {
///   issuerSerialNumber         IssuerSerialNumber,
///   fingerprintPKC        [0]  IMPLICIT FingerPrint {Certificate},
///   fingerprintPK         [1]  IMPLICIT FingerPrint {PublicKey},
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum PKCertIdentifier {
    issuerSerialNumber(IssuerSerialNumber),
    fingerprintPKC(FingerPrint),
    fingerprintPK(FingerPrint),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for PKCertIdentifier {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PKCertIdentifier(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PKCertIdentifier {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PKCertIdentifier(el)
    }
}

pub fn _decode_PKCertIdentifier(el: &X690Element) -> ASN1Result<PKCertIdentifier> {
    |el: &X690Element| -> ASN1Result<PKCertIdentifier> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(PKCertIdentifier::issuerSerialNumber(
                _decode_IssuerSerialNumber(&el)?,
            )),
            (TagClass::CONTEXT, 0) => {
                Ok(PKCertIdentifier::fingerprintPKC(_decode_FingerPrint(&el)?))
            }
            (TagClass::CONTEXT, 1) => {
                Ok(PKCertIdentifier::fingerprintPK(_decode_FingerPrint(&el)?))
            }
            _ => Ok(PKCertIdentifier::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_PKCertIdentifier(value_: &PKCertIdentifier) -> ASN1Result<X690Element> {
    |value: &PKCertIdentifier| -> ASN1Result<X690Element> {
        match value {
            PKCertIdentifier::issuerSerialNumber(v) => _encode_IssuerSerialNumber(&v),
            PKCertIdentifier::fingerprintPKC(v) => |v_1: &FingerPrint| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_FingerPrint(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            PKCertIdentifier::fingerprintPK(v) => |v_1: &FingerPrint| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_FingerPrint(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            PKCertIdentifier::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IssuerSerialNumber ::= SEQUENCE {
///   issuer        Name,
///   serialNumber  CertificateSerialNumber,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct IssuerSerialNumber {
    pub issuer: Name,
    pub serialNumber: CertificateSerialNumber,
    pub _unrecognized: Vec<X690Element>,
}
impl IssuerSerialNumber {
    pub fn new(
        issuer: Name,
        serialNumber: CertificateSerialNumber,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        IssuerSerialNumber {
            issuer,
            serialNumber,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for IssuerSerialNumber {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IssuerSerialNumber(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IssuerSerialNumber {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_IssuerSerialNumber(el)
    }
}

pub const _rctl1_components_for_IssuerSerialNumber: &[ComponentSpec; 2] = &[
    ComponentSpec::new("issuer", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IssuerSerialNumber: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IssuerSerialNumber: &[ComponentSpec; 0] = &[];

pub fn _decode_IssuerSerialNumber(el: &X690Element) -> ASN1Result<IssuerSerialNumber> {
    |el_: &X690Element| -> ASN1Result<IssuerSerialNumber> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IssuerSerialNumber,
            _eal_components_for_IssuerSerialNumber,
            _rctl2_components_for_IssuerSerialNumber,
        )?;
        let issuer = _decode_Name(_components.get("issuer").unwrap())?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        Ok(IssuerSerialNumber {
            issuer,
            serialNumber,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_IssuerSerialNumber(value_: &IssuerSerialNumber) -> ASN1Result<X690Element> {
    |value_: &IssuerSerialNumber| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Name(&value_.issuer)?);
        components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
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
/// ObjectDigestInfo-digestedObjectType ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type ObjectDigestInfo_digestedObjectType = ENUMERATED;

pub const ObjectDigestInfo_digestedObjectType_publicKey: ObjectDigestInfo_digestedObjectType = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ObjectDigestInfo_digestedObjectType_publicKeyCert: ObjectDigestInfo_digestedObjectType =
    1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ObjectDigestInfo_digestedObjectType_otherObjectTypes:
    ObjectDigestInfo_digestedObjectType = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_ObjectDigestInfo_digestedObjectType(
    el: &X690Element,
) -> ASN1Result<ObjectDigestInfo_digestedObjectType> {
    ber_decode_enumerated(&el)
}

pub fn _encode_ObjectDigestInfo_digestedObjectType(
    value_: &ObjectDigestInfo_digestedObjectType,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
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

impl TryFrom<X690Element> for TBSCertAVL_entries_Item_idType {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TBSCertAVL_entries_Item_idType(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TBSCertAVL_entries_Item_idType {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TBSCertAVL_entries_Item_idType(el)
    }
}

pub fn _decode_TBSCertAVL_entries_Item_idType(
    el: &X690Element,
) -> ASN1Result<TBSCertAVL_entries_Item_idType> {
    |el: &X690Element| -> ASN1Result<TBSCertAVL_entries_Item_idType> {
        match (el.tag_class, el.tag_number) {
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
    }(&el)
}

pub fn _encode_TBSCertAVL_entries_Item_idType(
    value_: &TBSCertAVL_entries_Item_idType,
) -> ASN1Result<X690Element> {
    |value: &TBSCertAVL_entries_Item_idType| -> ASN1Result<X690Element> {
        match value {
            TBSCertAVL_entries_Item_idType::certIdentifier(v) => {
                |v_1: &PKCertIdentifier| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_PKCertIdentifier(
                            &v_1,
                        )?))),
                    ))
                }(&v)
            }
            TBSCertAVL_entries_Item_idType::entityGroup(v) => _encode_DistinguishedName(&v),
            TBSCertAVL_entries_Item_idType::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBSCertAVL-entries-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
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
impl TryFrom<X690Element> for TBSCertAVL_entries_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TBSCertAVL_entries_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TBSCertAVL_entries_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<TBSCertAVL_entries_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TBSCertAVL_entries_Item,
            _eal_components_for_TBSCertAVL_entries_Item,
            _rctl2_components_for_TBSCertAVL_entries_Item,
        )?;
        let idType = _decode_TBSCertAVL_entries_Item_idType(_components.get("idType").unwrap())?;
        let scope: OPTIONAL<ScopeRestrictions> = match _components.get("scope") {
            Some(c_) => Some(_decode_ScopeRestrictions(c_)?),
            _ => None,
        };
        let entryExtensions: OPTIONAL<Extensions> = match _components.get("entryExtensions") {
            Some(c_) => Some(_decode_Extensions(c_)?),
            _ => None,
        };
        Ok(TBSCertAVL_entries_Item {
            idType,
            scope,
            entryExtensions,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TBSCertAVL_entries_Item(
    value_: &TBSCertAVL_entries_Item,
) -> ASN1Result<X690Element> {
    |value_: &TBSCertAVL_entries_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_TBSCertAVL_entries_Item_idType(&value_.idType)?);
        if let Some(v_) = &value_.scope {
            components_.push(|v_1: &ScopeRestrictions| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ScopeRestrictions(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryExtensions {
            components_.push(|v_1: &Extensions| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Extensions(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}
