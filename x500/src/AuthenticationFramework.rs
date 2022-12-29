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
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::borrow::Borrow;
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
impl TryFrom<X690Element> for SIGNATURE {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SIGNATURE(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SIGNATURE {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SIGNATURE> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SIGNATURE,
            _eal_components_for_SIGNATURE,
            _rctl2_components_for_SIGNATURE,
        )?;
        let algorithmIdentifier =
            _decode_AlgorithmIdentifier(_components.get("algorithmIdentifier").unwrap())?;
        let signature = ber_decode_bit_string(_components.get("signature").unwrap())?;
        Ok(SIGNATURE {
            algorithmIdentifier,
            signature,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SIGNATURE(value_: &SIGNATURE) -> ASN1Result<X690Element> {
    |value_: &SIGNATURE| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
        components_.push(ber_encode_bit_string(&value_.signature)?);
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
/// SIGNED{ToBeSigned} ::= SEQUENCE {
///   toBeSigned              ToBeSigned,
///   algorithmIdentifier     AlgorithmIdentifier{{SupportedAlgorithms}},
///   signature               BIT STRING,
///   ...,
/// [[4:
///   altAlgorithmIdentifier  AlgorithmIdentifier{{SupportedAltAlgorithms}} OPTIONAL,
///   altSignature            BIT STRING OPTIONAL]]
///   } (WITH COMPONENTS {..., altAlgorithmIdentifier PRESENT, altSignature PRESENT } |
///      WITH COMPONENTS {..., altAlgorithmIdentifier ABSENT,  altSignature ABSENT } )
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SIGNED<ToBeSigned> {
    pub toBeSigned: ToBeSigned,
    pub algorithmIdentifier: AlgorithmIdentifier,
    pub signature: BIT_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl<ToBeSigned> SIGNED<ToBeSigned> {
    pub fn new(
        toBeSigned: ToBeSigned,
        algorithmIdentifier: AlgorithmIdentifier,
        signature: BIT_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SIGNED {
            toBeSigned,
            algorithmIdentifier,
            signature,
            _unrecognized,
        }
    }
}
// impl <ToBeSigned> TryFrom<X690Element> for SIGNED<ToBeSigned> {
// 	type Error = ASN1Error;
// 	fn try_from (el: X690Element) -> Result<Self, Self::Error> {
// 		_decode_SIGNED(&el)
// 	}
// }
// impl<'a, ToBeSigned> TryFrom<&'a X690Element> for SIGNED<ToBeSigned> {
// 	type Error = ASN1Error;
// 	fn try_from (el: &'a X690Element) -> Result<Self, Self::Error> {
// 		_decode_SIGNED(el)
// 	}
// }

pub const _rctl1_components_for_SIGNED: &[ComponentSpec; 3] = &[
    ComponentSpec::new("toBeSigned", false, TagSelector::any, None, None),
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

pub const _rctl2_components_for_SIGNED: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SIGNED: &[ComponentSpec; 0] = &[];

pub fn _decode_SIGNED<ToBeSigned: 'static>(
    _decode_ToBeSigned: fn(&X690Element) -> ASN1Result<ToBeSigned>,
    el: &X690Element,
) -> ASN1Result<SIGNED<ToBeSigned>> {
    |el_: &X690Element| -> ASN1Result<SIGNED<ToBeSigned>> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SIGNED,
            _eal_components_for_SIGNED,
            _rctl2_components_for_SIGNED,
        )?;
        let toBeSigned = _decode_ToBeSigned(_components.get("toBeSigned").unwrap())?;
        let algorithmIdentifier =
            _decode_AlgorithmIdentifier(_components.get("algorithmIdentifier").unwrap())?;
        let signature = ber_decode_bit_string(_components.get("signature").unwrap())?;
        Ok(SIGNED {
            toBeSigned,
            algorithmIdentifier,
            signature,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SIGNED<ToBeSigned>(
    _encode_ToBeSigned: fn(&ToBeSigned) -> ASN1Result<X690Element>,
    value: &SIGNED<ToBeSigned>,
) -> ASN1Result<X690Element> {
    |value_: &SIGNED<ToBeSigned>| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(_encode_ToBeSigned(&value_.toBeSigned)?);
        components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
        components_.push(ber_encode_bit_string(&value_.signature)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HASH{ToBeHashed} ::= SEQUENCE {
///   algorithmIdentifier  AlgorithmIdentifier{{SupportedAlgorithms}},
///   hashValue            BIT STRING (CONSTRAINED BY {
///    -- shall be the result of applying a hashing procedure to the DER-encoded
///    -- octets of a value of -- ToBeHashed } ),
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct HASH {
    pub algorithmIdentifier: AlgorithmIdentifier,
    pub hashValue: BIT_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl HASH {
    pub fn new(
        algorithmIdentifier: AlgorithmIdentifier,
        hashValue: BIT_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        HASH {
            algorithmIdentifier,
            hashValue,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for HASH {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_HASH(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for HASH {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_HASH(el)
    }
}

pub const _rctl1_components_for_HASH: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithmIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "hashValue",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_HASH: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_HASH: &[ComponentSpec; 0] = &[];

pub fn _decode_HASH(el: &X690Element) -> ASN1Result<HASH> {
    |el_: &X690Element| -> ASN1Result<HASH> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_HASH,
            _eal_components_for_HASH,
            _rctl2_components_for_HASH,
        )?;
        let algorithmIdentifier =
            _decode_AlgorithmIdentifier(_components.get("algorithmIdentifier").unwrap())?;
        let hashValue = ber_decode_bit_string(_components.get("hashValue").unwrap())?;
        Ok(HASH {
            algorithmIdentifier,
            hashValue,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_HASH(value_: &HASH) -> ASN1Result<X690Element> {
    |value_: &HASH| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
        components_.push(ber_encode_bit_string(&value_.hashValue)?);
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
/// ENCRYPTED{ToBeEnciphered}  ::=  BIT STRING (CONSTRAINED BY {
///    -- shall be the result of applying an encipherment procedure
///    -- to the BER-encoded octets of a value of -- ToBeEnciphered } )
/// ```
pub type ENCRYPTED = BIT_STRING;

pub fn _decode_ENCRYPTED(el: &X690Element) -> ASN1Result<ENCRYPTED> {
    ber_decode_bit_string(&el)
}

pub fn _encode_ENCRYPTED(value_: &ENCRYPTED) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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
    ber_decode_bit_string(&el)
}

pub fn _encode_ENCRYPTED_HASH(value_: &ENCRYPTED_HASH) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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
/// SupportedAltAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedAltAlgorithms() -> Vec<ALGORITHM> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// FingerPrint {ToBeFingerprinted} ::= SEQUENCE {
///   algorithmIdentifier  AlgorithmIdentifier{{SupportedAlgorithms}},
///   fingerprint          BIT STRING,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct FingerPrint {
    pub algorithmIdentifier: AlgorithmIdentifier,
    pub fingerprint: BIT_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl FingerPrint {
    pub fn new(
        algorithmIdentifier: AlgorithmIdentifier,
        fingerprint: BIT_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        FingerPrint {
            algorithmIdentifier,
            fingerprint,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for FingerPrint {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_FingerPrint(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for FingerPrint {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_FingerPrint(el)
    }
}

pub const _rctl1_components_for_FingerPrint: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithmIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "fingerprint",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_FingerPrint: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_FingerPrint: &[ComponentSpec; 0] = &[];

pub fn _decode_FingerPrint(el: &X690Element) -> ASN1Result<FingerPrint> {
    |el_: &X690Element| -> ASN1Result<FingerPrint> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_FingerPrint,
            _eal_components_for_FingerPrint,
            _rctl2_components_for_FingerPrint,
        )?;
        let algorithmIdentifier =
            _decode_AlgorithmIdentifier(_components.get("algorithmIdentifier").unwrap())?;
        let fingerprint = ber_decode_bit_string(_components.get("fingerprint").unwrap())?;
        Ok(FingerPrint {
            algorithmIdentifier,
            fingerprint,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_FingerPrint(value_: &FingerPrint) -> ASN1Result<X690Element> {
    |value_: &FingerPrint| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
        components_.push(ber_encode_bit_string(&value_.fingerprint)?);
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ecPublicKey OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840) ansi-X9-62(10045)
///                                       keyType(2) 1 }
/// ```
///
///
pub fn id_ecPublicKey() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* ansi-X9-62 */ 10045,
        /* keyType */ 2, 1,
    ]) // OID_GETTER
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
    ber_decode_object_identifier(&el)
}

pub fn _encode_SupportedCurves(value_: &SupportedCurves) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dummyCurv OBJECT IDENTIFIER ::= {2 5 5}
/// ```
///
///
pub fn dummyCurv() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([2, 5, 5]) // OID_GETTER
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
///   [[2:  -- if present, version shall be v2 or v3
///   subjectUniqueIdentifier  [2] IMPLICIT UniqueIdentifier OPTIONAL]],
///   [[3:  -- if present, version shall be v2 or v3
///   extensions               [3]  Extensions OPTIONAL ]]
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
        vec![ Version_v1 as u8 ]
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

pub const _eal_components_for_TBSCertificate: &[ComponentSpec; 0] = &[];

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
        Ok(TBSCertificate {
            version,
            serialNumber,
            signature,
            issuer,
            validity,
            subject,
            subjectPublicKeyInfo,
            issuerUniqueIdentifier,
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
/// Version  ::=  INTEGER {v1(0), v2(1), v3(2)}
/// ```
pub type Version = INTEGER;

pub const Version_v1: i32 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const Version_v2: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Version_v3: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

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
    ]) // OID_GETTER
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
/// Certificates ::= SEQUENCE {
///   userCertificate    Certificate,
///   certificationPath  ForwardCertificationPath OPTIONAL,
///   ... }
/// ```
///
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
impl TryFrom<X690Element> for Certificates {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Certificates(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Certificates {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Certificates> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Certificates,
            _eal_components_for_Certificates,
            _rctl2_components_for_Certificates,
        )?;
        let userCertificate = _decode_Certificate(_components.get("userCertificate").unwrap())?;
        let certificationPath: OPTIONAL<ForwardCertificationPath> =
            match _components.get("certificationPath") {
                Some(c_) => Some(_decode_ForwardCertificationPath(c_)?),
                _ => None,
            };
        Ok(Certificates {
            userCertificate,
            certificationPath,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Certificates(value_: &Certificates) -> ASN1Result<X690Element> {
    |value_: &Certificates| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Certificate(&value_.userCertificate)?);
        if let Some(v_) = &value_.certificationPath {
            components_.push(_encode_ForwardCertificationPath(&v_)?);
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
/// ForwardCertificationPath  ::=  SEQUENCE SIZE (1..MAX) OF CrossCertificates
/// ```
pub type ForwardCertificationPath = Vec<CrossCertificates>; // SequenceOfType

pub fn _decode_ForwardCertificationPath(el: &X690Element) -> ASN1Result<ForwardCertificationPath> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CrossCertificates>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<CrossCertificates> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_CrossCertificates(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_ForwardCertificationPath(
    value_: &ForwardCertificationPath,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<CrossCertificates>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_CrossCertificates(&v)?);
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
/// CrossCertificates  ::=  SET SIZE (1..MAX) OF Certificate
/// ```
pub type CrossCertificates = Vec<Certificate>; // SetOfType

pub fn _decode_CrossCertificates(el: &X690Element) -> ASN1Result<CrossCertificates> {
    |el: &X690Element| -> ASN1Result<SET_OF<Certificate>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<Certificate> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_Certificate(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CrossCertificates(value_: &CrossCertificates) -> ASN1Result<X690Element> {
    |value_: &SET_OF<Certificate>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Certificate(&v)?);
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
/// CertificationPath ::= SEQUENCE {
///   userCertificate    Certificate,
///   theCACertificates  SEQUENCE SIZE (1..MAX) OF CertificatePair OPTIONAL,
///   ... }
/// ```
///
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
impl TryFrom<X690Element> for CertificationPath {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificationPath(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificationPath {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<CertificationPath> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificationPath,
            _eal_components_for_CertificationPath,
            _rctl2_components_for_CertificationPath,
        )?;
        let userCertificate = _decode_Certificate(_components.get("userCertificate").unwrap())?;
        let theCACertificates: OPTIONAL<Vec<CertificatePair>> =
            match _components.get("theCACertificates") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertificatePair>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<CertificatePair> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_CertificatePair(el)?);
                        }
                        Ok(items)
                    }(c_)?,
                ),
                _ => None,
            };
        Ok(CertificationPath {
            userCertificate,
            theCACertificates,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertificationPath(value_: &CertificationPath) -> ASN1Result<X690Element> {
    |value_: &CertificationPath| -> ASN1Result<X690Element> {
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
/// CertificateList  ::=  SIGNED{CertificateListContent}
/// ```
pub type CertificateList = SIGNED<CertificateListContent>; // DefinedType

pub fn _decode_CertificateList(el: &X690Element) -> ASN1Result<CertificateList> {
    _decode_SIGNED::<CertificateListContent>(_decode_CertificateListContent, &el)
}

pub fn _encode_CertificateList(value_: &CertificateList) -> ASN1Result<X690Element> {
    _encode_SIGNED::<CertificateListContent>(_encode_CertificateListContent, &value_)
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
impl TryFrom<X690Element> for CertificateListContent {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateListContent(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificateListContent {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<CertificateListContent> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificateListContent,
            _eal_components_for_CertificateListContent,
            _rctl2_components_for_CertificateListContent,
        )?;
        let version: OPTIONAL<Version> = match _components.get("version") {
            Some(c_) => Some(_decode_Version(c_)?),
            _ => None,
        };
        let signature = _decode_AlgorithmIdentifier(_components.get("signature").unwrap())?;
        let issuer = _decode_Name(_components.get("issuer").unwrap())?;
        let thisUpdate = _decode_Time(_components.get("thisUpdate").unwrap())?;
        let nextUpdate: OPTIONAL<Time> = match _components.get("nextUpdate") {
            Some(c_) => Some(_decode_Time(c_)?),
            _ => None,
        };
        let revokedCertificates: OPTIONAL<Vec<CertificateListContent_revokedCertificates_Item>> =
            match _components.get("revokedCertificates") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<CertificateListContent_revokedCertificates_Item>,
                > {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<CertificateListContent_revokedCertificates_Item> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_CertificateListContent_revokedCertificates_Item(el)?);
                    }
                    Ok(items)
                }(c_)?),
                _ => None,
            };
        let crlExtensions: OPTIONAL<Extensions> = match _components.get("crlExtensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Extensions> {
                Ok(_decode_Extensions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(CertificateListContent {
            version,
            signature,
            issuer,
            thisUpdate,
            nextUpdate,
            revokedCertificates,
            _unrecognized,
            crlExtensions,
        })
    }(&el)
}

pub fn _encode_CertificateListContent(value_: &CertificateListContent) -> ASN1Result<X690Element> {
    |value_: &CertificateListContent| -> ASN1Result<X690Element> {
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
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.crlExtensions {
            components_.push(|v_1: &Extensions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Extensions(&v_1)?))),
                ))
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
        vec![ Version_v1 as u8 ]
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
/// ScopeRestrictions  ::=  SEQUENCE OF ScopeRestriction
/// ```
pub type ScopeRestrictions = Vec<ScopeRestriction>; // SequenceOfType

pub fn _decode_ScopeRestrictions(el: &X690Element) -> ASN1Result<ScopeRestrictions> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ScopeRestriction>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<ScopeRestriction> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_ScopeRestriction(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_ScopeRestrictions(value_: &ScopeRestrictions) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<ScopeRestriction>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_ScopeRestriction(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

pub type SCOPE_RESTRICTION = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ScopeRestriction ::= SEQUENCE {
///   id            SCOPE-RESTRICTION.&id,
///   restriction   SCOPE-RESTRICTION.&Type,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ScopeRestriction {
    pub id: OBJECT_IDENTIFIER,
    pub restriction: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl ScopeRestriction {
    pub fn new(
        id: OBJECT_IDENTIFIER,
        restriction: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ScopeRestriction {
            id,
            restriction,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ScopeRestriction {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ScopeRestriction(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ScopeRestriction {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ScopeRestriction(el)
    }
}

pub const _rctl1_components_for_ScopeRestriction: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("restriction", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_ScopeRestriction: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ScopeRestriction: &[ComponentSpec; 0] = &[];

pub fn _decode_ScopeRestriction(el: &X690Element) -> ASN1Result<ScopeRestriction> {
    |el_: &X690Element| -> ASN1Result<ScopeRestriction> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ScopeRestriction,
            _eal_components_for_ScopeRestriction,
            _rctl2_components_for_ScopeRestriction,
        )?;
        let id = ber_decode_object_identifier(_components.get("id").unwrap())?;
        let restriction = x690_identity(_components.get("restriction").unwrap())?;
        Ok(ScopeRestriction {
            id,
            restriction,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ScopeRestriction(value_: &ScopeRestriction) -> ASN1Result<X690Element> {
    |value_: &ScopeRestriction| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.id)?);
        components_.push(x690_identity(&value_.restriction)?);
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
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([userCertificate()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pkiUser")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 PKI User")), /* OBJECT_FIELD_SETTING */
        id: id_oc_pkiUser(),                         /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
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
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([
            cACertificate(),
            certificateRevocationList(),
            eepkCertificateRevocationList(),
            authorityRevocationList(),
            crossCertificatePair(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pkiCA")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 PKI Certificate Authority")), /* OBJECT_FIELD_SETTING */
        id: id_oc_pkiCA(),                           /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
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
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural),      /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::<_>::from([commonName()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([
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
        MandatoryAttributes: Vec::<_>::from([commonName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_cRLDistPtNameForm(),            /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
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
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([deltaRevocationList()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("deltaCRL")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 delta CRL")),       /* OBJECT_FIELD_SETTING */
        id: id_oc_deltaCRL(),                                  /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
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
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([
            certificatePolicy(),
            certificationPracticeStmt(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("cpCps")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from(
            "Certificate Policy and Certification Practice Statement",
        )), /* OBJECT_FIELD_SETTING */
        id: id_oc_cpCps(),                           /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
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
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([pkiPath()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pkiCertPath")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("PKI Certification Path")), /* OBJECT_FIELD_SETTING */
        id: id_oc_pkiCertPath(),                     /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
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
impl TryFrom<X690Element> for CertificatePair {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificatePair(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificatePair {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<CertificatePair> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificatePair,
            _eal_components_for_CertificatePair,
            _rctl2_components_for_CertificatePair,
        )?;
        let issuedToThisCA: OPTIONAL<Certificate> = match _components.get("issuedToThisCA") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Certificate> {
                Ok(_decode_Certificate(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let issuedByThisCA: OPTIONAL<Certificate> = match _components.get("issuedByThisCA") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Certificate> {
                Ok(_decode_Certificate(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(CertificatePair {
            issuedToThisCA,
            issuedByThisCA,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertificatePair(value_: &CertificatePair) -> ASN1Result<X690Element> {
    |value_: &CertificatePair| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.issuedToThisCA {
            components_.push(|v_1: &Certificate| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Certificate(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.issuedByThisCA {
            components_.push(|v_1: &Certificate| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Certificate(&v_1)?))),
                ))
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
impl TryFrom<X690Element> for SupportedAlgorithm {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SupportedAlgorithm(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SupportedAlgorithm {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SupportedAlgorithm> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SupportedAlgorithm,
            _eal_components_for_SupportedAlgorithm,
            _rctl2_components_for_SupportedAlgorithm,
        )?;
        let algorithmIdentifier =
            _decode_AlgorithmIdentifier(_components.get("algorithmIdentifier").unwrap())?;
        let intendedUsage: OPTIONAL<KeyUsage> = match _components.get("intendedUsage") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<KeyUsage> {
                Ok(_decode_KeyUsage(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let intendedCertificatePolicies: OPTIONAL<CertificatePoliciesSyntax> =
            match _components.get("intendedCertificatePolicies") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<CertificatePoliciesSyntax> {
                        Ok(_decode_CertificatePoliciesSyntax(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        Ok(SupportedAlgorithm {
            algorithmIdentifier,
            intendedUsage,
            intendedCertificatePolicies,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SupportedAlgorithm(value_: &SupportedAlgorithm) -> ASN1Result<X690Element> {
    |value_: &SupportedAlgorithm| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
        if let Some(v_) = &value_.intendedUsage {
            components_.push(|v_1: &KeyUsage| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_KeyUsage(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.intendedCertificatePolicies {
            components_.push(
                |v_1: &CertificatePoliciesSyntax| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_CertificatePoliciesSyntax(&v_1)?,
                        ))),
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

impl TryFrom<X690Element> for InfoSyntax {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_InfoSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for InfoSyntax {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_InfoSyntax(el)
    }
}

pub fn _decode_InfoSyntax(el: &X690Element) -> ASN1Result<InfoSyntax> {
    |el: &X690Element| -> ASN1Result<InfoSyntax> {
        match (el.tag_class, el.tag_number) {
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
    }(&el)
}

pub fn _encode_InfoSyntax(value_: &InfoSyntax) -> ASN1Result<X690Element> {
    |value: &InfoSyntax| -> ASN1Result<X690Element> {
        match value {
            InfoSyntax::content(v) => _encode_UnboundedDirectoryString(&v),
            InfoSyntax::pointer(v) => _encode_InfoSyntax_pointer(&v),
            InfoSyntax::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// Policies POLICY ::= {...}
/// ```
///
///
pub fn Policies() -> Vec<POLICY> {
    Vec::from([])
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// PolicySyntax ::= SEQUENCE {
///   policyIdentifier  PolicyID,
///   policySyntax      InfoSyntax,
///   ... }
/// ```
///
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
impl TryFrom<X690Element> for PolicySyntax {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PolicySyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PolicySyntax {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<PolicySyntax> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PolicySyntax,
            _eal_components_for_PolicySyntax,
            _rctl2_components_for_PolicySyntax,
        )?;
        let policyIdentifier = _decode_PolicyID(_components.get("policyIdentifier").unwrap())?;
        let policySyntax = _decode_InfoSyntax(_components.get("policySyntax").unwrap())?;
        Ok(PolicySyntax {
            policyIdentifier,
            policySyntax,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PolicySyntax(value_: &PolicySyntax) -> ASN1Result<X690Element> {
    |value_: &PolicySyntax| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_PolicyID(&value_.policyIdentifier)?);
        components_.push(_encode_InfoSyntax(&value_.policySyntax)?);
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
/// PolicyID  ::=  CertPolicyId
/// ```
pub type PolicyID = CertPolicyId; // DefinedType

pub fn _decode_PolicyID(el: &X690Element) -> ASN1Result<PolicyID> {
    _decode_CertPolicyId(&el)
}

pub fn _encode_PolicyID(value_: &PolicyID) -> ASN1Result<X690Element> {
    _encode_CertPolicyId(&value_)
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
impl TryFrom<X690Element> for SupportedPublicKeyAlgorithms {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SupportedPublicKeyAlgorithms(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SupportedPublicKeyAlgorithms {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<SupportedPublicKeyAlgorithms> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SupportedPublicKeyAlgorithms,
            _eal_components_for_SupportedPublicKeyAlgorithms,
            _rctl2_components_for_SupportedPublicKeyAlgorithms,
        )?;
        let algorithmIdentifier =
            _decode_AlgorithmIdentifier(_components.get("algorithmIdentifier").unwrap())?;
        let minKeySize = ber_decode_integer(_components.get("minKeySize").unwrap())?;
        let extensions: OPTIONAL<Vec<OidOrAttr>> = match _components.get("extensions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<OidOrAttr>> {
                Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<OidOrAttr>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<OidOrAttr> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_OidOrAttr(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(SupportedPublicKeyAlgorithms {
            algorithmIdentifier,
            minKeySize,
            extensions,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SupportedPublicKeyAlgorithms(
    value_: &SupportedPublicKeyAlgorithms,
) -> ASN1Result<X690Element> {
    |value_: &SupportedPublicKeyAlgorithms| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
        components_.push(ber_encode_integer(&value_.minKeySize)?);
        if let Some(v_) = &value_.extensions {
            components_.push(|v_1: &Vec<OidOrAttr>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        OidOrAttr,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_OidOrAttr(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
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
/// SupportedPublicKeyAlgos ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedPublicKeyAlgos() -> Vec<ALGORITHM> {
    Vec::from([])
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

impl TryFrom<X690Element> for OidOrAttr {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_OidOrAttr(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for OidOrAttr {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_OidOrAttr(el)
    }
}

pub fn _decode_OidOrAttr(el: &X690Element) -> ASN1Result<OidOrAttr> {
    |el: &X690Element| -> ASN1Result<OidOrAttr> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 6) => Ok(OidOrAttr::oid(ber_decode_object_identifier(&el)?)),
            (TagClass::UNIVERSAL, 16) => Ok(OidOrAttr::attribute(_decode_Attribute(&el)?)),
            _ => Ok(OidOrAttr::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_OidOrAttr(value_: &OidOrAttr) -> ASN1Result<X690Element> {
    |value: &OidOrAttr| -> ASN1Result<X690Element> {
        match value {
            OidOrAttr::oid(v) => ber_encode_object_identifier(&v),
            OidOrAttr::attribute(v) => _encode_Attribute(&v),
            OidOrAttr::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtAttributes ATTRIBUTE ::= {...}
/// ```
///
///
pub fn ExtAttributes() -> Vec<ATTRIBUTE> {
    Vec::from([])
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-cRLDistributionPoint          OBJECT IDENTIFIER ::= {id-oc 19}
/// ```
///
///
pub fn id_oc_cRLDistributionPoint() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([19])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pkiUser                       OBJECT IDENTIFIER ::= {id-oc 21}
/// ```
///
///
pub fn id_oc_pkiUser() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([21])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pkiCA                         OBJECT IDENTIFIER ::= {id-oc 22}
/// ```
///
///
pub fn id_oc_pkiCA() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([22])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-deltaCRL                      OBJECT IDENTIFIER ::= {id-oc 23}
/// ```
///
///
pub fn id_oc_deltaCRL() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([23])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-cpCps                         OBJECT IDENTIFIER ::= {id-oc 30}
/// ```
///
///
pub fn id_oc_cpCps() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([30])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-pkiCertPath                   OBJECT IDENTIFIER ::= {id-oc 31}
/// ```
///
///
pub fn id_oc_pkiCertPath() -> OBJECT_IDENTIFIER {
    [id_oc(), Vec::<u32>::from([31])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-cRLDistPtNameForm             OBJECT IDENTIFIER ::= {id-nf 14}
/// ```
///
///
pub fn id_nf_cRLDistPtNameForm() -> OBJECT_IDENTIFIER {
    [id_nf(), Vec::<u32>::from([14])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-userPassword                  OBJECT IDENTIFIER ::= {id-at 35}
/// ```
///
///
pub fn id_at_userPassword() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([35])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-userCertificate               OBJECT IDENTIFIER ::= {id-at 36}
/// ```
///
///
pub fn id_at_userCertificate() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([36])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-cAcertificate                 OBJECT IDENTIFIER ::= {id-at 37}
/// ```
///
///
pub fn id_at_cAcertificate() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([37])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-authorityRevocationList       OBJECT IDENTIFIER ::= {id-at 38}
/// ```
///
///
pub fn id_at_authorityRevocationList() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([38])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-certificateRevocationList     OBJECT IDENTIFIER ::= {id-at 39}
/// ```
///
///
pub fn id_at_certificateRevocationList() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([39])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-crossCertificatePair          OBJECT IDENTIFIER ::= {id-at 40}
/// ```
///
///
pub fn id_at_crossCertificatePair() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([40])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-supportedAlgorithms           OBJECT IDENTIFIER ::= {id-at 52}
/// ```
///
///
pub fn id_at_supportedAlgorithms() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([52])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-deltaRevocationList           OBJECT IDENTIFIER ::= {id-at 53}
/// ```
///
///
pub fn id_at_deltaRevocationList() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([53])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-certificationPracticeStmt     OBJECT IDENTIFIER ::= {id-at 68}
/// ```
///
///
pub fn id_at_certificationPracticeStmt() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([68])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-certificatePolicy             OBJECT IDENTIFIER ::= {id-at 69}
/// ```
///
///
pub fn id_at_certificatePolicy() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([69])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-pkiPath                       OBJECT IDENTIFIER ::= {id-at 70}
/// ```
///
///
pub fn id_at_pkiPath() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([70])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-eepkCertificateRevocationList OBJECT IDENTIFIER ::= {id-at 101}
/// ```
///
///
pub fn id_at_eepkCertificateRevocationList() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([101])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-supportedPublicKeyAlgorithms  OBJECT IDENTIFIER ::= {id-at 103}
/// ```
///
///
pub fn id_at_supportedPublicKeyAlgorithms() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([103])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-x509SupportedPublicKeyAlgos  OBJECT IDENTIFIER ::= {id-asx 10}
/// ```
///
///
pub fn id_asx_x509SupportedPublicKeyAlgos() -> OBJECT_IDENTIFIER {
    [id_asx(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-x509Certificate              OBJECT IDENTIFIER ::= {id-lsx 8}
/// ```
///
///
pub fn id_lsx_x509Certificate() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-x509CertificateList          OBJECT IDENTIFIER ::= {id-lsx 9}
/// ```
///
///
pub fn id_lsx_x509CertificateList() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-x509CertificatePair          OBJECT IDENTIFIER ::= {id-lsx 10}
/// ```
///
///
pub fn id_lsx_x509CertificatePair() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-x509SupportedAlgorithm       OBJECT IDENTIFIER ::= {id-lsx 49}
/// ```
///
///
pub fn id_lsx_x509SupportedAlgorithm() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([49])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertificateListContent-revokedCertificates-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
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
impl TryFrom<X690Element> for CertificateListContent_revokedCertificates_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertificateListContent_revokedCertificates_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertificateListContent_revokedCertificates_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<CertificateListContent_revokedCertificates_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertificateListContent_revokedCertificates_Item,
            _eal_components_for_CertificateListContent_revokedCertificates_Item,
            _rctl2_components_for_CertificateListContent_revokedCertificates_Item,
        )?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        let revocationDate = _decode_Time(_components.get("revocationDate").unwrap())?;
        let crlEntryExtensions: OPTIONAL<Extensions> = match _components.get("crlEntryExtensions") {
            Some(c_) => Some(_decode_Extensions(c_)?),
            _ => None,
        };
        Ok(CertificateListContent_revokedCertificates_Item {
            serialNumber,
            revocationDate,
            crlEntryExtensions,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertificateListContent_revokedCertificates_Item(
    value_: &CertificateListContent_revokedCertificates_Item,
) -> ASN1Result<X690Element> {
    |value_: &CertificateListContent_revokedCertificates_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
        components_.push(_encode_Time(&value_.revocationDate)?);
        if let Some(v_) = &value_.crlEntryExtensions {
            components_.push(_encode_Extensions(&v_)?);
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
/// InfoSyntax-pointer ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
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
impl TryFrom<X690Element> for InfoSyntax_pointer {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_InfoSyntax_pointer(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for InfoSyntax_pointer {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<InfoSyntax_pointer> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_InfoSyntax_pointer,
            _eal_components_for_InfoSyntax_pointer,
            _rctl2_components_for_InfoSyntax_pointer,
        )?;
        let name = _decode_GeneralNames(_components.get("name").unwrap())?;
        let hash: OPTIONAL<HASH> = match _components.get("hash") {
            Some(c_) => Some(_decode_HASH(c_)?),
            _ => None,
        };
        Ok(InfoSyntax_pointer {
            name,
            hash,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_InfoSyntax_pointer(value_: &InfoSyntax_pointer) -> ASN1Result<X690Element> {
    |value_: &InfoSyntax_pointer| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_GeneralNames(&value_.name)?);
        if let Some(v_) = &value_.hash {
            components_.push(_encode_HASH(&v_)?);
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
