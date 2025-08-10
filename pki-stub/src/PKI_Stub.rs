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
use wildboar_asn1::*;
use std::sync::Arc;
use x690::*;
use std::iter::{Iterator, FusedIterator, ExactSizeIterator};
use crate::utils::{gt_to_chrono, utctime_to_chrono};
use chrono::{DateTime, Utc};
use std::cmp::Ordering;

#[inline]
const fn base_256_len(value: usize) -> usize {
    if value < 128 {
        return 1;
    }
    if value <= u8::MAX as usize {
        return 2;
    }
    if value <= u16::MAX as usize {
        return 3;
    }
    if value <= (u32::MAX >> 8) as usize {
        return 4;
    }
    if value <= u32::MAX as usize {
        return 5;
    }
    if value <= (u64::MAX >> 24) as usize {
        return 6;
    }
    if value <= (u64::MAX >> 16) as usize {
        return 7;
    }
    if value <= (u64::MAX >> 8) as usize {
        return 8;
    }
    return 9;
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-wrprot            OBJECT IDENTIFIER ::= wrapperProtocolType
/// ```
///
///
#[inline]
pub fn id_wrprot () -> OBJECT_IDENTIFIER {
	wrapperProtocolType() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// wrapperProtocolType  OBJECT IDENTIFIER ::= {ds 43}
/// ```
///
///
#[inline]
pub fn wrapperProtocolType () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(ds(), 43).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ds                   OBJECT IDENTIFIER ::= {joint-iso-itu-t ds(5)}
/// ```
///
///
#[inline]
pub fn ds () -> OBJECT_IDENTIFIER {
	oid!(joint_iso_itu_t,/* ds */ 5) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo              OBJECT IDENTIFIER ::= algorithms
/// ```
///
///
#[inline]
pub fn id_algo () -> OBJECT_IDENTIFIER {
	algorithms() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// algorithms           OBJECT IDENTIFIER ::= {ds 44}
/// ```
///
///
#[inline]
pub fn algorithms () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 44 ].as_slice()) } // OID_GETTER
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
///   [DYN-PARMS     &DynParms]
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
impl TryFrom<&X690Element> for AlgorithmWithInvoke {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AlgorithmWithInvoke")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AlgorithmWithInvoke,
        _eal_components_for_AlgorithmWithInvoke,
        _rctl2_components_for_AlgorithmWithInvoke,
    )
    ;
    let mut _i: usize = 0;
    let mut algorithm_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut parameters_: OPTIONAL<X690Element> = None;
    let mut dynamParms_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => algorithm_ = Some(BER.decode_object_identifier(_el)?),
            "parameters" => {
                parameters_ = Some(|el: &X690Element| -> ASN1Result<X690Element> {
                    Ok(x690_identity(&el.inner()?)?)
                }(_el)?)
            }
            "dynamParms" => {
                dynamParms_ = Some(|el: &X690Element| -> ASN1Result<X690Element> {
                    Ok(x690_identity(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AlgorithmWithInvoke {
        algorithm: algorithm_.unwrap(),
        parameters: parameters_,
        dynamParms: dynamParms_,
        _unrecognized,
    })
}

pub fn _encode_AlgorithmWithInvoke(value_: &AlgorithmWithInvoke) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_object_identifier(&value_.algorithm)?);
    if let Some(v_) = &value_.parameters {
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(x690_identity(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.dynamParms {
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(x690_identity(&v_1)?),
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

pub fn _validate_AlgorithmWithInvoke(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AlgorithmWithInvoke")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AlgorithmWithInvoke,
        _eal_components_for_AlgorithmWithInvoke,
        _rctl2_components_for_AlgorithmWithInvoke,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => BER.validate_object_identifier(_el)?,
            "parameters" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "parameters")
                    );
                }
                Ok(BER.validate_any(&el.inner()?)?)
            }(_el)?,
            "dynamParms" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "dynamParms")
                    );
                }
                Ok(BER.validate_any(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    ;
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
    ;
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

pub fn _validate_AlgoInvoke(el: &X690Element) -> ASN1Result<()> {
    BER.validate_any(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HASH{ToBeHashed} ::= SEQUENCE {
///   algorithmIdentifier  AlgorithmIdentifier{{SupportedAlgorithms}},
///   hashValue            BIT STRING,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for HASH {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "HASH")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_HASH,
        _eal_components_for_HASH,
        _rctl2_components_for_HASH,
    )
    ;
    let mut _i: usize = 0;
    let mut algorithmIdentifier_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut hashValue_: OPTIONAL<BIT_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => algorithmIdentifier_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "hashValue" => hashValue_ = Some(BER.decode_bit_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(HASH {
        algorithmIdentifier: algorithmIdentifier_.unwrap(),
        hashValue: hashValue_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_HASH(value_: &HASH) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
    components_.push(BER.encode_bit_string(&value_.hashValue)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_HASH(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "HASH")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_HASH,
        _eal_components_for_HASH,
        _rctl2_components_for_HASH,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => _validate_AlgorithmIdentifier(_el)?,
            "hashValue" => BER.validate_bit_string(_el)?,
            _ => (),
        }
    }
    Ok(())
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
        id: oid!(1, 2, 840, 113549, 1, 11), /* OBJECT_FIELD_SETTING */
    }
}

pub mod sha224WithRSAEncryptionAlgorithm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
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
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SIGNED{ToBeSigned} ::= SEQUENCE {
///   toBeSigned              ToBeSigned,
///   algorithmIdentifier     AlgorithmIdentifier{{SupportedAlgorithms}},
///   signature               BIT STRING,
///   ...,
///   altAlgorithmIdentifier  AlgorithmIdentifier{{SupportedAlgorithms}} OPTIONAL,
///   altSignature            BIT STRING OPTIONAL
///   } (WITH COMPONENTS {..., altAlgorithmIdentifier PRESENT, altSignature PRESENT } |
///      WITH COMPONENTS {..., altAlgorithmIdentifier ABSENT,  altSignature ABSENT } )
/// ```
///
#[derive(Debug, Clone)]
pub struct SIGNED<ToBeSigned> {
    pub toBeSigned: ToBeSigned,
    pub algorithmIdentifier: AlgorithmIdentifier,
    pub signature: BIT_STRING,
    pub altAlgorithmIdentifier: OPTIONAL<AlgorithmIdentifier>,
    pub altSignature: OPTIONAL<BIT_STRING>,
    pub _unrecognized: Vec<X690Element>,
}
impl<ToBeSigned> SIGNED<ToBeSigned> {
    pub fn new(
        toBeSigned: ToBeSigned,
        algorithmIdentifier: AlgorithmIdentifier,
        signature: BIT_STRING,
        altAlgorithmIdentifier: OPTIONAL<AlgorithmIdentifier>,
        altSignature: OPTIONAL<BIT_STRING>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SIGNED {
            toBeSigned,
            algorithmIdentifier,
            signature,
            altAlgorithmIdentifier,
            altSignature,
            _unrecognized,
        }
    }
}

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

pub const _eal_components_for_SIGNED: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "altAlgorithmIdentifier",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "altSignature",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub fn _decode_SIGNED<ToBeSigned: 'static>(
    _decode_ToBeSigned: fn(&X690Element) -> ASN1Result<ToBeSigned>,
    el: &X690Element,
) -> ASN1Result<SIGNED<ToBeSigned>> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SIGNED")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SIGNED,
        _eal_components_for_SIGNED,
        _rctl2_components_for_SIGNED,
    )
    ;
    let mut _i: usize = 0;
    let mut toBeSigned_: OPTIONAL<ToBeSigned> = None;
    let mut algorithmIdentifier_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut signature_: OPTIONAL<BIT_STRING> = None;
    let mut altAlgorithmIdentifier_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut altSignature_: OPTIONAL<BIT_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "toBeSigned" => toBeSigned_ = Some(_decode_ToBeSigned(_el)?),
            "algorithmIdentifier" => algorithmIdentifier_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "signature" => signature_ = Some(BER.decode_bit_string(_el)?),
            "altAlgorithmIdentifier" => {
                altAlgorithmIdentifier_ = Some(_decode_AlgorithmIdentifier(_el)?)
            }
            "altSignature" => altSignature_ = Some(BER.decode_bit_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(SIGNED {
        toBeSigned: toBeSigned_.unwrap(),
        algorithmIdentifier: algorithmIdentifier_.unwrap(),
        signature: signature_.unwrap(),
        altAlgorithmIdentifier: altAlgorithmIdentifier_,
        altSignature: altSignature_,
        _unrecognized,
    })
}

pub fn _encode_SIGNED<ToBeSigned>(
    _encode_ToBeSigned: fn(&ToBeSigned) -> ASN1Result<X690Element>,
    value_: &SIGNED<ToBeSigned>,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    components_.push(_encode_ToBeSigned(&value_.toBeSigned)?);
    components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
    components_.push(BER.encode_bit_string(&value_.signature)?);
    if let Some(v_) = &value_.altAlgorithmIdentifier {
        components_.push(_encode_AlgorithmIdentifier(&v_)?);
    }
    if let Some(v_) = &value_.altSignature {
        components_.push(BER.encode_bit_string(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_SIGNED<ToBeSigned>(
    _validate_ToBeSigned: fn(&X690Element) -> ASN1Result<()>,
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SIGNED")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SIGNED,
        _eal_components_for_SIGNED,
        _rctl2_components_for_SIGNED,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "toBeSigned" => _validate_ToBeSigned(_el)?,
            "algorithmIdentifier" => _validate_AlgorithmIdentifier(_el)?,
            "signature" => BER.validate_bit_string(_el)?,
            "altAlgorithmIdentifier" => _validate_AlgorithmIdentifier(_el)?,
            "altSignature" => BER.validate_bit_string(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for FingerPrint {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "FingerPrint")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_FingerPrint,
        _eal_components_for_FingerPrint,
        _rctl2_components_for_FingerPrint,
    )
    ;
    let mut _i: usize = 0;
    let mut algorithmIdentifier_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut fingerprint_: OPTIONAL<BIT_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => algorithmIdentifier_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "fingerprint" => fingerprint_ = Some(BER.decode_bit_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(FingerPrint {
        algorithmIdentifier: algorithmIdentifier_.unwrap(),
        fingerprint: fingerprint_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_FingerPrint(value_: &FingerPrint) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
    components_.push(BER.encode_bit_string(&value_.fingerprint)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_FingerPrint(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "FingerPrint")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_FingerPrint,
        _eal_components_for_FingerPrint,
        _rctl2_components_for_FingerPrint,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => _validate_AlgorithmIdentifier(_el)?,
            "fingerprint" => BER.validate_bit_string(_el)?,
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
///   --[[2:  if present, version shall be v2 or v3
///   subjectUniqueIdentifier  [2] IMPLICIT UniqueIdentifier OPTIONAL--]]--,
///   --[[3:  if present, version shall be v2 or v3
///   extensions               [3]  Extensions OPTIONAL --]]
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
    ;
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut signature_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut validity_: OPTIONAL<Validity> = None;
    let mut subject_: OPTIONAL<Name> = None;
    let mut subjectPublicKeyInfo_: OPTIONAL<SubjectPublicKeyInfo> = None;
    let mut issuerUniqueIdentifier_: OPTIONAL<UniqueIdentifier> = None;
    let mut subjectUniqueIdentifier_: OPTIONAL<UniqueIdentifier> = None;
    let mut extensions_: OPTIONAL<Extensions> = None;
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
            "subjectUniqueIdentifier" => {
                subjectUniqueIdentifier_ = Some(_decode_UniqueIdentifier(_el)?)
            }
            "extensions" => {
                extensions_ = Some(|el: &X690Element| -> ASN1Result<Extensions> {
                    Ok(_decode_Extensions(&el.inner()?)?)
                }(_el)?)
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
        subjectUniqueIdentifier: subjectUniqueIdentifier_,
        extensions: extensions_,
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
                    X690Value::from_explicit(_encode_Version(&v_1)?),
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
    if let Some(v_) = &value_.subjectUniqueIdentifier {
        components_.push(|v_1: &UniqueIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_UniqueIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.extensions {
        components_.push(|v_1: &Extensions| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(_encode_Extensions(&v_1)?),
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
    ;
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
            "subjectUniqueIdentifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "subjectUniqueIdentifier",
                    ));
                }
                Ok(_validate_UniqueIdentifier(&el)?)
            }(_el)?,
            "extensions" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "extensions")
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
/// Version  ::=  INTEGER {v1(0), v2(1), v3(2)}
/// ```
pub type Version = i8;

pub const Version_v1: Version = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const Version_v2: Version = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Version_v3: Version = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Version(el: &X690Element) -> ASN1Result<Version> {
    BER.decode_i8(&el)
}

pub fn _encode_Version(value_: &Version) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Version(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(&el)
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
    ;
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
    ;
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
    ;
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
    ;
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

impl Time {

    // Intentionally not exported because we don't want chrono to become
    // a requirement of the public-facing API.
    pub(crate) fn into_chrono(&self) -> Result<DateTime<Utc>, ()> {
        match self {
            Time::utcTime(t) => utctime_to_chrono(t),
            Time::generalizedTime(t) => gt_to_chrono(t),
        }
    }
}

impl PartialEq for Time {

    /// `Eq` cannot be implemented for `Time` because two times that have the
    /// same error making them unable to be converted to chrono times will not
    /// equal each other, even if they are the same values.
    fn eq(&self, other: &Self) -> bool {
        match (self.into_chrono(), other.into_chrono()) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        }
    }

}

impl PartialOrd for Time {

    /// Only returns `None` if one or more of the times cannot be converted to a
    /// `DateTime` from the `chrono` crate. Because this is fallible, we cannot
    /// implement `Ord` for `Time`.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.into_chrono(), other.into_chrono()) {
            (Ok(a), Ok(b)) => Some(a.cmp(&b)),
            _ => None,
        }
    }

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
/// UniqueIdentifier  ::=  BIT STRING
/// ```
pub type UniqueIdentifier = BIT_STRING;

pub fn _decode_UniqueIdentifier(el: &X690Element) -> ASN1Result<UniqueIdentifier> {
    BER.decode_bit_string(&el)
}

pub fn _encode_UniqueIdentifier(value_: &UniqueIdentifier) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_UniqueIdentifier(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
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
    ;
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
    ;
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
pub fn der() -> OBJECT_IDENTIFIER {
    oid!(joint_iso_itu_t,
        /* asn1 */ 1,
        /* ber-derived */ 2,
        /* distinguished-encoding */ 1) // OID_GETTER
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
/// RDNSequence  ::=  SEQUENCE OF RelativeDistinguishedName
/// ```
pub type RDNSequence = Vec<RelativeDistinguishedName>; // SequenceOfType

pub fn _decode_RDNSequence(el: &X690Element) -> ASN1Result<RDNSequence> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RDNSequence")),
    };
    let mut items: SEQUENCE_OF<RelativeDistinguishedName> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_RelativeDistinguishedName(el)?);
    }
    Ok(items)
}

pub fn _encode_RDNSequence(value_: &RDNSequence) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_RelativeDistinguishedName(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_RDNSequence(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_RelativeDistinguishedName(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RDNSequence")),
    }
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "RelativeDistinguishedName",
            ))
        }
    };
    let mut items: SET_OF<AttributeTypeAndValue> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AttributeTypeAndValue(el)?);
    }
    Ok(items)
}

pub fn _encode_RelativeDistinguishedName(
    value_: &RelativeDistinguishedName,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AttributeTypeAndValue(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_RelativeDistinguishedName(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AttributeTypeAndValue(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "RelativeDistinguishedName",
        )),
    }
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

pub fn _validate_DistinguishedName(el: &X690Element) -> ASN1Result<()> {
    _validate_RDNSequence(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeTypeAndValue ::= SEQUENCE {
///   type                  ATTRIBUTE.&id({SupportedAttributes}),
///   value                 ATTRIBUTE.&Type({SupportedAttributes}{@type}),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct AttributeTypeAndValue {
    pub type_: OBJECT_IDENTIFIER,
    pub value: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeTypeAndValue {
    pub fn new(
        type_: OBJECT_IDENTIFIER,
        value: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeTypeAndValue {
            type_,
            value,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for AttributeTypeAndValue {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    ComponentSpec::new("value", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_AttributeTypeAndValue: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeTypeAndValue: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeTypeAndValue(el: &X690Element) -> ASN1Result<AttributeTypeAndValue> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeTypeAndValue")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeTypeAndValue,
        _eal_components_for_AttributeTypeAndValue,
        _rctl2_components_for_AttributeTypeAndValue,
    )
    ;
    let mut _i: usize = 0;
    let mut type__: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut value_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => type__ = Some(BER.decode_object_identifier(_el)?),
            "value" => value_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttributeTypeAndValue {
        type_: type__.unwrap(),
        value: value_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AttributeTypeAndValue(value_: &AttributeTypeAndValue) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.type_)?);
    components_.push(x690_identity(&value_.value)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AttributeTypeAndValue(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeTypeAndValue")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeTypeAndValue,
        _eal_components_for_AttributeTypeAndValue,
        _rctl2_components_for_AttributeTypeAndValue,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => BER.validate_object_identifier(_el)?,
            "value" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedAttributes ATTRIBUTE ::= {...}
/// ```
///
///
pub fn SupportedAttributes() -> Vec<ATTRIBUTE> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ATTRIBUTE ::= CLASS {
///   &derivation               ATTRIBUTE OPTIONAL,
///   &Type                     OPTIONAL, -- either &Type or &derivation required
///   &equality-match           MATCHING-RULE OPTIONAL,
///   &ordering-match           MATCHING-RULE OPTIONAL,
///   &substrings-match         MATCHING-RULE OPTIONAL,
///   &single-valued            BOOLEAN DEFAULT FALSE,
///   &collective               BOOLEAN DEFAULT FALSE,
///   &dummy                    BOOLEAN DEFAULT FALSE,
///   -- operational extensions
///   &no-user-modification     BOOLEAN DEFAULT FALSE,
///   &usage                    AttributeUsage DEFAULT userApplications,
///   &ldapSyntax               SYNTAX-NAME.&id OPTIONAL,
///   &ldapName                 SEQUENCE SIZE(1..MAX) OF UTF8String OPTIONAL,
///   &ldapDesc                 UTF8String OPTIONAL,
///   &obsolete                 BOOLEAN DEFAULT FALSE,
///   &id                       OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   [SUBTYPE OF               &derivation]
///   [WITH SYNTAX              &Type]
///   [EQUALITY MATCHING RULE   &equality-match]
///   [ORDERING MATCHING RULE   &ordering-match]
///   [SUBSTRINGS MATCHING RULE &substrings-match]
///   [SINGLE VALUE             &single-valued]
///   [COLLECTIVE               &collective]
///   [DUMMY                    &dummy]
///   [NO USER MODIFICATION     &no-user-modification]
///   [USAGE                    &usage]
///   [LDAP-SYNTAX              &ldapSyntax]
///   [LDAP-NAME                &ldapName]
///   [LDAP-DESC                &ldapDesc]
///   [OBSOLETE                 &obsolete]
///   ID                        &id }
/// ```
///
#[derive(Debug, Clone)]
pub struct ATTRIBUTE {
    pub derivation: OPTIONAL<Box<ATTRIBUTE>>,
    pub equality_match: OPTIONAL<Box<MATCHING_RULE>>,
    pub ordering_match: OPTIONAL<Box<MATCHING_RULE>>,
    pub substrings_match: OPTIONAL<Box<MATCHING_RULE>>,
    pub single_valued: OPTIONAL<BOOLEAN>,
    pub collective: OPTIONAL<BOOLEAN>,
    pub dummy: OPTIONAL<BOOLEAN>,
    pub no_user_modification: OPTIONAL<BOOLEAN>,
    pub usage: OPTIONAL<AttributeUsage>,
    pub ldapSyntax: OPTIONAL<OBJECT_IDENTIFIER>,
    pub ldapName: OPTIONAL<Vec<UTF8String>>,
    pub ldapDesc: OPTIONAL<UTF8String>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub id: OBJECT_IDENTIFIER,
}
impl ATTRIBUTE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Attribute {ATTRIBUTE:SupportedAttributes} ::= SEQUENCE {
///   type                ATTRIBUTE.&id({SupportedAttributes}),
///   values              SET SIZE (0..MAX) OF ATTRIBUTE.&Type({SupportedAttributes}{@type}),
///   valuesWithContext   SET SIZE (1..MAX) OF SEQUENCE {
///     value               ATTRIBUTE.&Type({SupportedAttributes}{@type}),
///     contextList         SET SIZE (1..MAX) OF Context,
///     ...} OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Attribute {
    pub type_: OBJECT_IDENTIFIER,
    pub values: Vec<X690Element>,
    pub valuesWithContext: OPTIONAL<Vec<Attribute_valuesWithContext_Item>>,
    pub _unrecognized: Vec<X690Element>,
}
impl Attribute {
    pub fn new(
        type_: OBJECT_IDENTIFIER,
        values: Vec<X690Element>,
        valuesWithContext: OPTIONAL<Vec<Attribute_valuesWithContext_Item>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Attribute {
            type_,
            values,
            valuesWithContext,
            _unrecognized,
        }
    }

    pub fn iter_values(&self) -> AttributeValuesIterator<'_> {
        AttributeValuesIterator {
            attr: &self,
            i: 0,
            j: 0,
        }
    }

    /// Accuracy does not matter too much in this function. Do not expect it.
    pub fn size_in_bytes(&self) -> usize {
        let values_len: usize = self.values
            .iter()
            .map(|v| v.len())
            .reduce(|acc, size| acc + size)
            .unwrap_or(0);
        let vwc_len: usize = self.valuesWithContext
            .as_ref()
            .map(|vwcs| vwcs
                .iter()
                .map(|v| v.size_in_bytes())
                .reduce(|acc, size| acc + size)
            )
            .flatten()
            .unwrap_or(0);
        let values_tag_len = 1 + base_256_len(values_len);
        let vwc_tag_len = if self.valuesWithContext.is_some() {
            1 + base_256_len(vwc_len)
        } else {
            0
        };
        let content_octets_len = 2 // Assume an OID fits on 127 bytes or less.
            + self.type_.as_x690_slice().len()
            + values_tag_len
            + values_len
            + vwc_tag_len
            + vwc_len;
        let tag_len = 1 + base_256_len(content_octets_len);
        tag_len + content_octets_len
    }
}

#[derive(Debug, Clone)]
pub struct AttributeValuesIterator<'a> {
    attr: &'a Attribute,
    i: usize,
    j: usize,
}

impl <'a> Iterator for AttributeValuesIterator<'a> {
    type Item = (&'a OBJECT_IDENTIFIER, &'a X690Element, Option<&'a [Context]>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.attr.values.get(self.i) {
            self.i += 1;
            return Some((&self.attr.type_, val, None));
        }
        if let Some(vwc) = self.attr.valuesWithContext.as_ref().map(|vwcs| vwcs.get(self.j)).flatten() {
            self.j += 1;
            return Some((&self.attr.type_, &vwc.value, Some(vwc.contextList.as_slice())));
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.attr.values.len()
            + self.attr.valuesWithContext
                .as_ref()
                .map(|vwcs| vwcs.len())
                .unwrap_or(0);
        (exact, Some(exact))
    }

}

impl FusedIterator for AttributeValuesIterator<'_> {}
impl ExactSizeIterator for AttributeValuesIterator<'_> {}

impl TryFrom<&X690Element> for Attribute {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Attribute(el)
    }
}

pub const _rctl1_components_for_Attribute: &[ComponentSpec; 3] = &[
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
    ComponentSpec::new(
        "valuesWithContext",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Attribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Attribute: &[ComponentSpec; 0] = &[];

pub fn _decode_Attribute(el: &X690Element) -> ASN1Result<Attribute> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Attribute")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Attribute,
        _eal_components_for_Attribute,
        _rctl2_components_for_Attribute,
    )
    ;
    let mut _i: usize = 0;
    let mut type__: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut values_: OPTIONAL<Vec<X690Element>> = None;
    let mut valuesWithContext_: OPTIONAL<Vec<Attribute_valuesWithContext_Item>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => type__ = Some(BER.decode_object_identifier(_el)?),
            "values" => {
                values_ = Some(|el: &X690Element| -> ASN1Result<SET_OF<X690Element>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "values")
                            )
                        }
                    };
                    let mut items: SET_OF<X690Element> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(x690_identity(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            "valuesWithContext" => {
                valuesWithContext_ = Some(|el: &X690Element| -> ASN1Result<
                    SET_OF<Attribute_valuesWithContext_Item>,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "valuesWithContext",
                            ))
                        }
                    };
                    let mut items: SET_OF<Attribute_valuesWithContext_Item> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_Attribute_valuesWithContext_Item(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Attribute {
        type_: type__.unwrap(),
        values: values_.unwrap(),
        valuesWithContext: valuesWithContext_,
        _unrecognized,
    })
}

pub fn _encode_Attribute(value_: &Attribute) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_object_identifier(&value_.type_)?);
    components_.push(|value_: &SET_OF<X690Element>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(x690_identity(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }(&value_.values)?);
    if let Some(v_) = &value_.valuesWithContext {
        components_.push(
            |value_: &SET_OF<Attribute_valuesWithContext_Item>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_Attribute_valuesWithContext_Item(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
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

pub fn _validate_Attribute(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Attribute")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Attribute,
        _eal_components_for_Attribute,
        _rctl2_components_for_Attribute,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => BER.validate_object_identifier(_el)?,
            "values" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            BER.validate_any(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "values")),
                }
            }(_el)?,
            "valuesWithContext" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_Attribute_valuesWithContext_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "valuesWithContext",
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
/// AttributeCertificate  ::=  SIGNED{TBSAttributeCertificate}
/// ```
pub type AttributeCertificate = SIGNED<TBSAttributeCertificate>; // DefinedType

pub fn _decode_AttributeCertificate(el: &X690Element) -> ASN1Result<AttributeCertificate> {
    _decode_SIGNED::<TBSAttributeCertificate>(_decode_TBSAttributeCertificate, el)
}

pub fn _encode_AttributeCertificate(value_: &AttributeCertificate) -> ASN1Result<X690Element> {
    _encode_SIGNED::<TBSAttributeCertificate>(_encode_TBSAttributeCertificate, value_)
}

pub fn _validate_AttributeCertificate(el: &X690Element) -> ASN1Result<()> {
    _validate_SIGNED::<TBSAttributeCertificate>(_validate_TBSAttributeCertificate, el)
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
impl TryFrom<&X690Element> for TBSAttributeCertificate {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "TBSAttributeCertificate",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSAttributeCertificate,
        _eal_components_for_TBSAttributeCertificate,
        _rctl2_components_for_TBSAttributeCertificate,
    )
    ;
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AttCertVersion> = None;
    let mut holder_: OPTIONAL<Holder> = None;
    let mut issuer_: OPTIONAL<AttCertIssuer> = None;
    let mut signature_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut attrCertValidityPeriod_: OPTIONAL<AttCertValidityPeriod> = None;
    let mut attributes_: OPTIONAL<Vec<Attribute>> = None;
    let mut issuerUniqueID_: OPTIONAL<UniqueIdentifier> = None;
    let mut extensions_: OPTIONAL<Extensions> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AttCertVersion(_el)?),
            "holder" => holder_ = Some(_decode_Holder(_el)?),
            "issuer" => issuer_ = Some(_decode_AttCertIssuer(_el)?),
            "signature" => signature_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            "attrCertValidityPeriod" => {
                attrCertValidityPeriod_ = Some(_decode_AttCertValidityPeriod(_el)?)
            }
            "attributes" => {
                attributes_ = Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<Attribute>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "attributes",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_Attribute(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            "issuerUniqueID" => issuerUniqueID_ = Some(_decode_UniqueIdentifier(_el)?),
            "extensions" => extensions_ = Some(_decode_Extensions(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TBSAttributeCertificate {
        version: version_.unwrap(),
        holder: holder_.unwrap(),
        issuer: issuer_.unwrap(),
        signature: signature_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
        attrCertValidityPeriod: attrCertValidityPeriod_.unwrap(),
        attributes: attributes_.unwrap(),
        issuerUniqueID: issuerUniqueID_,
        _unrecognized,
        extensions: extensions_,
    })
}

pub fn _encode_TBSAttributeCertificate(
    value_: &TBSAttributeCertificate,
) -> ASN1Result<X690Element> {
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
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
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
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_TBSAttributeCertificate(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "TBSAttributeCertificate",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBSAttributeCertificate,
        _eal_components_for_TBSAttributeCertificate,
        _rctl2_components_for_TBSAttributeCertificate,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AttCertVersion(_el)?,
            "holder" => _validate_Holder(_el)?,
            "issuer" => _validate_AttCertIssuer(_el)?,
            "signature" => _validate_AlgorithmIdentifier(_el)?,
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            "attrCertValidityPeriod" => _validate_AttCertValidityPeriod(_el)?,
            "attributes" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_Attribute(&sub)?;
                        }
                        Ok(())
                    }
                    _ => {
                        Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributes"))
                    }
                }
            }(_el)?,
            "issuerUniqueID" => _validate_UniqueIdentifier(_el)?,
            "extensions" => _validate_Extensions(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttCertVersion  ::=  INTEGER {v2(1)}
/// ```
pub type AttCertVersion = i8;

pub const AttCertVersion_v2: AttCertVersion = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_AttCertVersion(el: &X690Element) -> ASN1Result<AttCertVersion> {
    BER.decode_i8(&el)
}

pub fn _encode_AttCertVersion(value_: &AttCertVersion) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_AttCertVersion(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(&el)
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
impl TryFrom<&X690Element> for Holder {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Holder")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Holder,
        _eal_components_for_Holder,
        _rctl2_components_for_Holder,
    )
    ;
    let mut _i: usize = 0;
    let mut baseCertificateID_: OPTIONAL<IssuerSerial> = None;
    let mut entityName_: OPTIONAL<GeneralNames> = None;
    let mut objectDigestInfo_: OPTIONAL<ObjectDigestInfo> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "baseCertificateID" => {
                baseCertificateID_ = Some(|el: &X690Element| -> ASN1Result<IssuerSerial> {
                    Ok(_decode_IssuerSerial(&el.inner()?)?)
                }(_el)?)
            }
            "entityName" => {
                entityName_ = Some(|el: &X690Element| -> ASN1Result<GeneralNames> {
                    Ok(_decode_GeneralNames(&el.inner()?)?)
                }(_el)?)
            }
            "objectDigestInfo" => {
                objectDigestInfo_ = Some(|el: &X690Element| -> ASN1Result<ObjectDigestInfo> {
                    Ok(_decode_ObjectDigestInfo(&el.inner()?)?)
                }(_el)?)
            }
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Holder")),
        }
    }
    Ok(Holder {
        baseCertificateID: baseCertificateID_,
        entityName: entityName_,
        objectDigestInfo: objectDigestInfo_,
    })
}

pub fn _encode_Holder(value_: &Holder) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    if let Some(v_) = &value_.baseCertificateID {
        components_.push(|v_1: &IssuerSerial| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(_encode_IssuerSerial(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.entityName {
        components_.push(|v_1: &GeneralNames| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(_encode_GeneralNames(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.objectDigestInfo {
        components_.push(|v_1: &ObjectDigestInfo| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(_encode_ObjectDigestInfo(&v_1)?),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Holder(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Holder")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Holder,
        _eal_components_for_Holder,
        _rctl2_components_for_Holder,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "baseCertificateID" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "baseCertificateID",
                    ));
                }
                Ok(_validate_IssuerSerial(&el.inner()?)?)
            }(_el)?,
            "entityName" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "entityName")
                    );
                }
                Ok(_validate_GeneralNames(&el.inner()?)?)
            }(_el)?,
            "objectDigestInfo" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "objectDigestInfo",
                    ));
                }
                Ok(_validate_ObjectDigestInfo(&el.inner()?)?)
            }(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Holder")),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for IssuerSerial {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IssuerSerial")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IssuerSerial,
        _eal_components_for_IssuerSerial,
        _rctl2_components_for_IssuerSerial,
    )
    ;
    let mut _i: usize = 0;
    let mut issuer_: OPTIONAL<GeneralNames> = None;
    let mut serial_: OPTIONAL<CertificateSerialNumber> = None;
    let mut issuerUID_: OPTIONAL<UniqueIdentifier> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => issuer_ = Some(_decode_GeneralNames(_el)?),
            "serial" => serial_ = Some(_decode_CertificateSerialNumber(_el)?),
            "issuerUID" => issuerUID_ = Some(_decode_UniqueIdentifier(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(IssuerSerial {
        issuer: issuer_.unwrap(),
        serial: serial_.unwrap(),
        issuerUID: issuerUID_,
        _unrecognized,
    })
}

pub fn _encode_IssuerSerial(value_: &IssuerSerial) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_GeneralNames(&value_.issuer)?);
    components_.push(_encode_CertificateSerialNumber(&value_.serial)?);
    if let Some(v_) = &value_.issuerUID {
        components_.push(_encode_UniqueIdentifier(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_IssuerSerial(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IssuerSerial")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IssuerSerial,
        _eal_components_for_IssuerSerial,
        _rctl2_components_for_IssuerSerial,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => _validate_GeneralNames(_el)?,
            "serial" => _validate_CertificateSerialNumber(_el)?,
            "issuerUID" => _validate_UniqueIdentifier(_el)?,
            _ => (),
        }
    }
    Ok(())
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

    /// Returns `true` if the other digest applies to the same thing
    ///
    /// You can only compare two digests if they are comparable
    /// according to this function.
    pub fn is_comparable_with(&self, other: &Self) -> bool {
        self.digestedObjectType == other.digestedObjectType
            && self.otherObjectTypeID == other.otherObjectTypeID
            && self.digestAlgorithm == other.digestAlgorithm
    }
}
impl TryFrom<&X690Element> for ObjectDigestInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ObjectDigestInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ObjectDigestInfo,
        _eal_components_for_ObjectDigestInfo,
        _rctl2_components_for_ObjectDigestInfo,
    )
    ;
    let mut _i: usize = 0;
    let mut digestedObjectType_: OPTIONAL<ObjectDigestInfo_digestedObjectType> = None;
    let mut otherObjectTypeID_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut digestAlgorithm_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut objectDigest_: OPTIONAL<BIT_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "digestedObjectType" => {
                digestedObjectType_ = Some(_decode_ObjectDigestInfo_digestedObjectType(_el)?)
            }
            "otherObjectTypeID" => otherObjectTypeID_ = Some(BER.decode_object_identifier(_el)?),
            "digestAlgorithm" => digestAlgorithm_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "objectDigest" => objectDigest_ = Some(BER.decode_bit_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ObjectDigestInfo {
        digestedObjectType: digestedObjectType_.unwrap(),
        otherObjectTypeID: otherObjectTypeID_,
        digestAlgorithm: digestAlgorithm_.unwrap(),
        objectDigest: objectDigest_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ObjectDigestInfo(value_: &ObjectDigestInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(_encode_ObjectDigestInfo_digestedObjectType(
        &value_.digestedObjectType,
    )?);
    if let Some(v_) = &value_.otherObjectTypeID {
        components_.push(BER.encode_object_identifier(&v_)?);
    }
    components_.push(_encode_AlgorithmIdentifier(&value_.digestAlgorithm)?);
    components_.push(BER.encode_bit_string(&value_.objectDigest)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ObjectDigestInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ObjectDigestInfo")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ObjectDigestInfo,
        _eal_components_for_ObjectDigestInfo,
        _rctl2_components_for_ObjectDigestInfo,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "digestedObjectType" => _validate_ObjectDigestInfo_digestedObjectType(_el)?,
            "otherObjectTypeID" => BER.validate_object_identifier(_el)?,
            "digestAlgorithm" => _validate_AlgorithmIdentifier(_el)?,
            "objectDigest" => BER.validate_bit_string(_el)?,
            _ => (),
        }
    }
    Ok(())
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

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.issuerName.is_none()
        && self.baseCertificateID.is_none()
        && self.objectDigestInfo.is_none()
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
impl TryFrom<&X690Element> for AttCertIssuer {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    |el: &X690Element| -> ASN1Result<AttCertIssuer> {
        Ok(|el: &X690Element| -> ASN1Result<AttCertIssuer> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertIssuer")
                    )
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_AttCertIssuer,
                _eal_components_for_AttCertIssuer,
                _rctl2_components_for_AttCertIssuer,
            )
            ;
            let mut _i: usize = 0;
            let mut issuerName_: OPTIONAL<GeneralNames> = None;
            let mut baseCertificateID_: OPTIONAL<IssuerSerial> = None;
            let mut objectDigestInfo_: OPTIONAL<ObjectDigestInfo> = None;
            let mut _unrecognized: Vec<X690Element> = vec![];
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "issuerName" => issuerName_ = Some(_decode_GeneralNames(_el)?),
                    "baseCertificateID" => {
                        baseCertificateID_ = Some(|el: &X690Element| -> ASN1Result<IssuerSerial> {
                            Ok(_decode_IssuerSerial(&el.inner()?)?)
                        }(_el)?)
                    }
                    "objectDigestInfo" => {
                        objectDigestInfo_ =
                            Some(|el: &X690Element| -> ASN1Result<ObjectDigestInfo> {
                                Ok(_decode_ObjectDigestInfo(&el.inner()?)?)
                            }(_el)?)
                    }
                    _ => _unrecognized.push(_el.clone()),
                }
            }
            Ok(AttCertIssuer {
                issuerName: issuerName_,
                baseCertificateID: baseCertificateID_,
                objectDigestInfo: objectDigestInfo_,
                _unrecognized,
            })
        }(&el.inner()?)?)
    }(&el)
}

pub fn _encode_AttCertIssuer(value_: &AttCertIssuer) -> ASN1Result<X690Element> {
    |v_1: &AttCertIssuer| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(|value_: &AttCertIssuer| -> ASN1Result<X690Element> {
                let mut components_: Vec<X690Element> = Vec::with_capacity(13);
                if let Some(v_) = &value_.issuerName {
                    components_.push(_encode_GeneralNames(&v_)?);
                }
                if let Some(v_) = &value_.baseCertificateID {
                    components_.push(|v_1: &IssuerSerial| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 0),
                            X690Value::from_explicit(_encode_IssuerSerial(&v_1)?),
                        ))
                    }(&v_)?);
                }
                if let Some(v_) = &value_.objectDigestInfo {
                    components_.push(|v_1: &ObjectDigestInfo| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 1),
                            X690Value::from_explicit(_encode_ObjectDigestInfo(&v_1)?),
                        ))
                    }(&v_)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                    X690Value::Constructed(Arc::new(
                        [components_, value_._unrecognized.clone()].concat(),
                    )),
                ))
            }(&v_1)?),
        ))
    }(&value_)
}

pub fn _validate_AttCertIssuer(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertIssuer"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertIssuer")
                    )
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_AttCertIssuer,
                _eal_components_for_AttCertIssuer,
                _rctl2_components_for_AttCertIssuer,
            )
            ;
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "issuerName" => _validate_GeneralNames(_el)?,
                    "baseCertificateID" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "baseCertificateID",
                            ));
                        }
                        Ok(_validate_IssuerSerial(&el.inner()?)?)
                    }(_el)?,
                    "objectDigestInfo" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "objectDigestInfo",
                            ));
                        }
                        Ok(_validate_ObjectDigestInfo(&el.inner()?)?)
                    }(_el)?,
                    _ => (),
                }
            }
            Ok(())
        }(&el.inner()?)?)
    }(&el)
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
impl TryFrom<&X690Element> for AttCertValidityPeriod {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertValidityPeriod")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttCertValidityPeriod,
        _eal_components_for_AttCertValidityPeriod,
        _rctl2_components_for_AttCertValidityPeriod,
    )
    ;
    let mut _i: usize = 0;
    let mut notBeforeTime_: OPTIONAL<GeneralizedTime> = None;
    let mut notAfterTime_: OPTIONAL<GeneralizedTime> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notBeforeTime" => notBeforeTime_ = Some(BER.decode_generalized_time(_el)?),
            "notAfterTime" => notAfterTime_ = Some(BER.decode_generalized_time(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttCertValidityPeriod {
        notBeforeTime: notBeforeTime_.unwrap(),
        notAfterTime: notAfterTime_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AttCertValidityPeriod(value_: &AttCertValidityPeriod) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_generalized_time(&value_.notBeforeTime)?);
    components_.push(BER.encode_generalized_time(&value_.notAfterTime)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AttCertValidityPeriod(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttCertValidityPeriod")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttCertValidityPeriod,
        _eal_components_for_AttCertValidityPeriod,
        _rctl2_components_for_AttCertValidityPeriod,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notBeforeTime" => BER.validate_generalized_time(_el)?,
            "notAfterTime" => BER.validate_generalized_time(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    x400Address(X690Element),
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

// TODO: Streamline this decoding / encoding. These closures are unnecessary.
pub fn _decode_GeneralName(el: &X690Element) -> ASN1Result<GeneralName> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(GeneralName::otherName(
            |el: &X690Element| -> ASN1Result<INSTANCE_OF> {
                Ok(BER.decode_instance_of(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(GeneralName::rfc822Name(
            |el: &X690Element| -> ASN1Result<IA5String> {
                Ok(BER.decode_ia5_string(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(GeneralName::dNSName(
            |el: &X690Element| -> ASN1Result<IA5String> {
                Ok(BER.decode_ia5_string(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 3) => Ok(GeneralName::x400Address(el.inner()?)),
        (TagClass::CONTEXT, 4) => Ok(GeneralName::directoryName(
            |el: &X690Element| -> ASN1Result<Name> { Ok(_decode_Name(&el.inner()?)?) }(&el)?,
        )),
        (TagClass::CONTEXT, 5) => Ok(GeneralName::ediPartyName(
            _decode_EDIPartyName(&el.inner()?)?
        )),
        (TagClass::CONTEXT, 6) => Ok(GeneralName::uniformResourceIdentifier(
            |el: &X690Element| -> ASN1Result<IA5String> {
                Ok(BER.decode_ia5_string(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 7) => Ok(GeneralName::iPAddress(
            |el: &X690Element| -> ASN1Result<OCTET_STRING> {
                Ok(BER.decode_octet_string(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 8) => Ok(GeneralName::registeredID(|el: &X690Element| -> ASN1Result<
            OBJECT_IDENTIFIER,
        > {
            Ok(BER.decode_object_identifier(&el.inner()?)?)
        }(&el)?)),
        _ => Ok(GeneralName::_unrecognized(el.clone())),
    }
}

pub fn _encode_GeneralName(value_: &GeneralName) -> ASN1Result<X690Element> {
    match value_ {
        GeneralName::otherName(v) => |v_1: &INSTANCE_OF| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(BER.encode_instance_of(&v_1)?),
            ))
        }(&v),
        GeneralName::rfc822Name(v) => |v_1: &IA5String| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(BER.encode_ia5_string(&v_1)?),
            ))
        }(&v),
        GeneralName::dNSName(v) => |v_1: &IA5String| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(BER.encode_ia5_string(&v_1)?),
            ))
        }(&v),
        GeneralName::x400Address(v) => Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 4),
            X690Value::from_explicit(v.clone()),
        )),
        GeneralName::directoryName(v) => |v_1: &Name| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::from_explicit(_encode_Name(&v_1)?),
            ))
        }(&v),
        GeneralName::ediPartyName(v) => Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 5),
            X690Value::from_explicit(_encode_EDIPartyName(v)?),
        )),
        GeneralName::uniformResourceIdentifier(v) => |v_1: &IA5String| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 6),
                X690Value::from_explicit(BER.encode_ia5_string(&v_1)?),
            ))
        }(&v),
        GeneralName::iPAddress(v) => |v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 7),
                X690Value::from_explicit(BER.encode_octet_string(&v_1)?),
            ))
        }(&v),
        GeneralName::registeredID(v) => |v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 8),
                X690Value::from_explicit(BER.encode_object_identifier(&v_1)?),
            ))
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
            Ok(BER.validate_external(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "rfc822Name"));
            }
            Ok(BER.validate_ia5_string(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "dNSName"));
            }
            Ok(BER.validate_ia5_string(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 4) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "directoryName")
                );
            }
            Ok(_validate_Name(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 6) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "uniformResourceIdentifier",
                ));
            }
            Ok(BER.validate_ia5_string(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 7) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "iPAddress"));
            }
            Ok(BER.validate_octet_string(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 8) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 8 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "registeredID")
                );
            }
            Ok(BER.validate_object_identifier(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

pub type OTHER_NAME = TYPE_IDENTIFIER;

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
    ;
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
    ;
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

impl TryFrom<&X690Element> for PKCertIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PKCertIdentifier(el)
    }
}

pub fn _decode_PKCertIdentifier(el: &X690Element) -> ASN1Result<PKCertIdentifier> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(PKCertIdentifier::issuerSerialNumber(
            _decode_IssuerSerialNumber(&el)?,
        )),
        (TagClass::CONTEXT, 0) => Ok(PKCertIdentifier::fingerprintPKC(_decode_FingerPrint(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(PKCertIdentifier::fingerprintPK(_decode_FingerPrint(&el)?)),
        _ => Ok(PKCertIdentifier::_unrecognized(el.clone())),
    }
}

pub fn _encode_PKCertIdentifier(value_: &PKCertIdentifier) -> ASN1Result<X690Element> {
    match value_ {
        PKCertIdentifier::issuerSerialNumber(v) => _encode_IssuerSerialNumber(&v),
        PKCertIdentifier::fingerprintPKC(v) => |v_1: &FingerPrint| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_FingerPrint(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        PKCertIdentifier::fingerprintPK(v) => |v_1: &FingerPrint| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_FingerPrint(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
        PKCertIdentifier::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_PKCertIdentifier(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_IssuerSerialNumber(&el),
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fingerprintPKC")
                );
            }
            Ok(_validate_FingerPrint(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fingerprintPK")
                );
            }
            Ok(_validate_FingerPrint(&el)?)
        }(&el),
        _ => Ok(()),
    }
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
impl TryFrom<&X690Element> for IssuerSerialNumber {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IssuerSerialNumber")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IssuerSerialNumber,
        _eal_components_for_IssuerSerialNumber,
        _rctl2_components_for_IssuerSerialNumber,
    )
    ;
    let mut _i: usize = 0;
    let mut issuer_: OPTIONAL<Name> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => issuer_ = Some(_decode_Name(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(IssuerSerialNumber {
        issuer: issuer_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_IssuerSerialNumber(value_: &IssuerSerialNumber) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_Name(&value_.issuer)?);
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_IssuerSerialNumber(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IssuerSerialNumber")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IssuerSerialNumber,
        _eal_components_for_IssuerSerialNumber,
        _rctl2_components_for_IssuerSerialNumber,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuer" => _validate_Name(_el)?,
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    BER.decode_enumerated(&el)
}

pub fn _encode_ObjectDigestInfo_digestedObjectType(
    value_: &ObjectDigestInfo_digestedObjectType,
) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_ObjectDigestInfo_digestedObjectType(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
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
                    X690Value::from_explicit(_encode_PKCertIdentifier(&v_1)?),
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
/// ScopeRestrictions  ::=  SEQUENCE OF ScopeRestriction
/// ```
pub type ScopeRestrictions = Vec<ScopeRestriction>; // SequenceOfType

pub fn _decode_ScopeRestrictions(el: &X690Element) -> ASN1Result<ScopeRestrictions> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ScopeRestrictions")
            )
        }
    };
    let mut items: SEQUENCE_OF<ScopeRestriction> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_ScopeRestriction(el)?);
    }
    Ok(items)
}

pub fn _encode_ScopeRestrictions(value_: &ScopeRestrictions) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_ScopeRestriction(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_ScopeRestrictions(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_ScopeRestriction(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ScopeRestrictions")),
    }
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
impl TryFrom<&X690Element> for ScopeRestriction {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ScopeRestriction")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ScopeRestriction,
        _eal_components_for_ScopeRestriction,
        _rctl2_components_for_ScopeRestriction,
    )
    ;
    let mut _i: usize = 0;
    let mut id_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut restriction_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "id" => id_ = Some(BER.decode_object_identifier(_el)?),
            "restriction" => restriction_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ScopeRestriction {
        id: id_.unwrap(),
        restriction: restriction_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ScopeRestriction(value_: &ScopeRestriction) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.id)?);
    components_.push(x690_identity(&value_.restriction)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ScopeRestriction(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ScopeRestriction")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ScopeRestriction,
        _eal_components_for_ScopeRestriction,
        _rctl2_components_for_ScopeRestriction,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "id" => BER.validate_object_identifier(_el)?,
            "restriction" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    ;
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
    ;
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
/// Context ::= SEQUENCE {
///   contextType    CONTEXT.&id({SupportedContexts}),
///   contextValues
///     SET SIZE (1..MAX) OF CONTEXT.&Type({SupportedContexts}{@contextType}),
///   fallback       BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Context {
    pub contextType: OBJECT_IDENTIFIER,
    pub contextValues: Vec<X690Element>,
    pub fallback: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl Context {
    pub fn new(
        contextType: OBJECT_IDENTIFIER,
        contextValues: Vec<X690Element>,
        fallback: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Context {
            contextType,
            contextValues,
            fallback,
            _unrecognized,
        }
    }
    pub fn _default_value_for_fallback() -> BOOLEAN {
        false
    }

    pub fn size_in_bytes(&self) -> usize {
        let cvlen: usize = self.contextValues
            .iter()
            .map(|cv| cv.len())
            .reduce(|acc, len| acc + len)
            .unwrap_or(0);
        let cvlen = 1 + base_256_len(cvlen);
        let components_len: usize = self.contextType.as_x690_slice().len()
            + if self.fallback.is_some() { 3 } else { 0 }
            + cvlen // The tag and length of the contextValues sequence
            + self.contextValues
                .iter()
                .map(|cv| cv.len())
                .reduce(|acc, len| acc + len)
                .unwrap_or(0);
        let tag_len_len = 1 + base_256_len(components_len);
        tag_len_len + components_len
    }
}
impl TryFrom<&X690Element> for Context {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Context(el)
    }
}

pub const _rctl1_components_for_Context: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "contextType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "contextValues",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "fallback",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Context: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Context: &[ComponentSpec; 0] = &[];

pub fn _decode_Context(el: &X690Element) -> ASN1Result<Context> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Context,
        _eal_components_for_Context,
        _rctl2_components_for_Context,
    )
    ;
    let mut _i: usize = 0;
    let mut contextType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut contextValues_: OPTIONAL<Vec<X690Element>> = None;
    let mut fallback_: OPTIONAL<BOOLEAN> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contextType" => contextType_ = Some(BER.decode_object_identifier(_el)?),
            "contextValues" => {
                contextValues_ = Some(|el: &X690Element| -> ASN1Result<SET_OF<X690Element>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "contextValues",
                            ))
                        }
                    };
                    let mut items: SET_OF<X690Element> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(x690_identity(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            "fallback" => fallback_ = Some(BER.decode_boolean(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Context {
        contextType: contextType_.unwrap(),
        contextValues: contextValues_.unwrap(),
        fallback: fallback_,
        _unrecognized,
    })
}

pub fn _encode_Context(value_: &Context) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_object_identifier(&value_.contextType)?);
    components_.push(|value_: &SET_OF<X690Element>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(x690_identity(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }(&value_.contextValues)?);
    if let Some(v_) = &value_.fallback {
        if *v_ != Context::_default_value_for_fallback() {
            components_.push(BER.encode_boolean(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Context(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Context,
        _eal_components_for_Context,
        _rctl2_components_for_Context,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contextType" => BER.validate_object_identifier(_el)?,
            "contextValues" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            BER.validate_any(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contextValues")
                    ),
                }
            }(_el)?,
            "fallback" => BER.validate_boolean(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Attribute-valuesWithContext-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct Attribute_valuesWithContext_Item {
    pub value: X690Element,
    pub contextList: Vec<Context>,
    pub _unrecognized: Vec<X690Element>,
}
impl Attribute_valuesWithContext_Item {
    pub fn new(
        value: X690Element,
        contextList: Vec<Context>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Attribute_valuesWithContext_Item {
            value,
            contextList,
            _unrecognized,
        }
    }

    pub fn size_in_bytes(&self) -> usize {
        let cl_len = self.contextList
            .iter()
            .map(|c| c.size_in_bytes())
            .reduce(|acc, size| acc + size)
            .unwrap_or(0);

        let cl_tag_len = 1 + base_256_len(cl_len);
        let content_octets_len = self.value.len() + cl_tag_len + cl_len;
        let tag_len = 1 + base_256_len(content_octets_len);
        tag_len + content_octets_len
    }
}
impl TryFrom<&X690Element> for Attribute_valuesWithContext_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Attribute_valuesWithContext_Item(el)
    }
}

pub const _rctl1_components_for_Attribute_valuesWithContext_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new("value", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "contextList",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Attribute_valuesWithContext_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Attribute_valuesWithContext_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_Attribute_valuesWithContext_Item(
    el: &X690Element,
) -> ASN1Result<Attribute_valuesWithContext_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "Attribute-valuesWithContext-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Attribute_valuesWithContext_Item,
        _eal_components_for_Attribute_valuesWithContext_Item,
        _rctl2_components_for_Attribute_valuesWithContext_Item,
    )
    ;
    let mut _i: usize = 0;
    let mut value_: OPTIONAL<X690Element> = None;
    let mut contextList_: OPTIONAL<Vec<Context>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "value" => value_ = Some(x690_identity(_el)?),
            "contextList" => {
                contextList_ = Some(|el: &X690Element| -> ASN1Result<SET_OF<Context>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "contextList",
                            ))
                        }
                    };
                    let mut items: SET_OF<Context> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_Context(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Attribute_valuesWithContext_Item {
        value: value_.unwrap(),
        contextList: contextList_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_Attribute_valuesWithContext_Item(
    value_: &Attribute_valuesWithContext_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(x690_identity(&value_.value)?);
    components_.push(|value_: &SET_OF<Context>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Context(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }(&value_.contextList)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Attribute_valuesWithContext_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "Attribute-valuesWithContext-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Attribute_valuesWithContext_Item,
        _eal_components_for_Attribute_valuesWithContext_Item,
        _rctl2_components_for_Attribute_valuesWithContext_Item,
    )
    ;
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "value" => BER.validate_any(_el)?,
            "contextList" => {
                |el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_Context(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el
                            .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contextList")),
                    }
                }(_el)?
            }
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeUsage  ::=  ENUMERATED {
///   userApplications     (0),
///   directoryOperation   (1),
///   distributedOperation (2),
///   dSAOperation         (3),
///   ... }
/// ```
pub type AttributeUsage = ENUMERATED;

pub const AttributeUsage_userApplications: AttributeUsage = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AttributeUsage_directoryOperation: AttributeUsage = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AttributeUsage_distributedOperation: AttributeUsage = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AttributeUsage_dSAOperation: AttributeUsage = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_AttributeUsage(el: &X690Element) -> ASN1Result<AttributeUsage> {
    BER.decode_enumerated(&el)
}

pub fn _encode_AttributeUsage(value_: &AttributeUsage) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_AttributeUsage(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MATCHING-RULE ::= CLASS {
///   &ParentMatchingRules    MATCHING-RULE OPTIONAL,
///   &AssertionType          OPTIONAL,
///   &uniqueMatchIndicator   ATTRIBUTE OPTIONAL,
///   &ldapSyntax             SYNTAX-NAME.&id OPTIONAL,
///   &ldapName               SEQUENCE SIZE(1..MAX) OF UTF8String OPTIONAL,
///   &ldapDesc               UTF8String OPTIONAL,
///   &id                     OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   [PARENT                 &ParentMatchingRules]
///   [SYNTAX                 &AssertionType]
///   [UNIQUE-MATCH-INDICATOR &uniqueMatchIndicator]
///   [LDAP-SYNTAX            &ldapSyntax]
///   [LDAP-NAME              &ldapName]
///   [LDAP-DESC              &ldapDesc]
///   ID                      &id }
/// ```
///
#[derive(Debug, Clone)]
pub struct MATCHING_RULE {
    pub ParentMatchingRules: OPTIONAL<Vec<MATCHING_RULE>>,
    pub uniqueMatchIndicator: OPTIONAL<ATTRIBUTE>,
    pub ldapSyntax: OPTIONAL<OBJECT_IDENTIFIER>,
    pub ldapName: OPTIONAL<Vec<UTF8String>>,
    pub ldapDesc: OPTIONAL<UTF8String>,
    pub id: OBJECT_IDENTIFIER,
}
impl MATCHING_RULE {}

/// X.500 Directory Name
///
/// In later versions of the X.500 specifications, the `dnsName` and `oid`
/// variants were introduced. These are not widely supported, however, so be
/// careful when using them.
///
/// ### ASN.1 Definition:
///
/// ```asn1
/// Name  ::=  CHOICE {
///   rdnSequence   RDNSequence,
///   dnsName       DomainName,
///   oid           OBJECT IDENTIFIER }
/// ```
#[derive(Debug, Clone)]
pub enum Name {
    rdnSequence(RDNSequence),
    dnsName(DomainName),
    oid(OBJECT_IDENTIFIER),
}

impl TryFrom<&X690Element> for Name {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Name(el)
    }
}

pub fn _decode_Name(el: &X690Element) -> ASN1Result<Name> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(Name::rdnSequence(_decode_RDNSequence(&el)?)),
        (TagClass::UNIVERSAL, 12) => Ok(Name::dnsName(_decode_DomainName(&el)?)),
        (TagClass::UNIVERSAL, 6) => Ok(Name::oid(BER.decode_object_identifier(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Name",
            ))
        }
    }
}

pub fn _encode_Name(value_: &Name) -> ASN1Result<X690Element> {
    match value_ {
        Name::rdnSequence(v) => _encode_RDNSequence(&v),
        Name::dnsName(v) => _encode_DomainName(&v),
        Name::oid(v) => BER.encode_object_identifier(&v),
    }
}

pub fn _validate_Name(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_RDNSequence(&el),
        (TagClass::UNIVERSAL, 12) => _validate_DomainName(&el),
        (TagClass::UNIVERSAL, 6) => BER.validate_object_identifier(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Name",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DomainName  ::=  UTF8String (CONSTRAINED BY {
/// -- Conforms to the format of an (internationalized) domain name. -- })
/// ```
pub type DomainName = UTF8String; // UTF8String

pub fn _decode_DomainName(el: &X690Element) -> ASN1Result<DomainName> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_DomainName(value_: &DomainName) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_DomainName(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
}

impl TryFrom<Name> for RDNSequence {
    type Error = ASN1Error;

    fn try_from(value: Name) -> Result<Self, Self::Error> {
        match value {
            Name::rdnSequence(rdns) => Ok(rdns),
            Name::dnsName(fqdn) => {
                let number_of_labels = fqdn.matches(".").count();
                let mut rdns: RDNSequence = Vec::with_capacity(number_of_labels);
                for label in fqdn.split(".") {
                    if label.chars().all(|c| c.is_ascii()) {
                        rdns.push(
                            vec![
                                AttributeTypeAndValue::new(
                                    oid!(0,9,2342,19200300,100,1,25), // dc
                                    BER.encode_ia5_string(label).unwrap(),
                                    vec![],
                                )
                            ]
                        );
                    }
                    if label.starts_with("xn--") {
                        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                    }
                    let punycoded_label = punycode::encode(label)
                        .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
                    let new_label = ["xn--",&punycoded_label].concat();
                    rdns.push(
                        vec![
                            AttributeTypeAndValue::new(
                                oid!(0,9,2342,19200300,100,1,25), // dc
                                BER.encode_ia5_string(&new_label).unwrap(),
                                vec![],
                            )
                        ]
                    );
                }
                Ok(rdns)
            },
            Name::oid(oid) =>  {
                let mut rdns: RDNSequence = Vec::with_capacity(oid.len());
                for arc in oid.arcs() {
                    rdns.push(vec![
                        AttributeTypeAndValue::new(
                            oid!(2,17,1,2,2), // oidC
                            BER.encode_u128(arc).unwrap(),
                            vec![],
                        )
                    ]);
                }
                Ok(rdns)
            },
        }
    }

}

impl From<RDNSequence> for Name {

    fn from(value: RDNSequence) -> Self {
        Name::rdnSequence(value)
    }

}

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
    pub nameAssigner: OPTIONAL<X690Element>,
    pub partyName: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl EDIPartyName {
    pub fn new(
        nameAssigner: OPTIONAL<X690Element>,
        partyName: X690Element,
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
    ;
    let mut _i: usize = 0;
    let mut nameAssigner_: OPTIONAL<X690Element> = None;
    let mut partyName_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "nameAssigner" => {
                nameAssigner_ = Some(_el.inner()?);
            }
            "partyName" => {
                partyName_ = Some(_el.inner()?);
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
            X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(v_.clone()),
            )
        );
    }
    components_.push(
        X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(value_.partyName.clone()),
        )
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
    ;
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
                Ok(BER.validate_any(&el.inner()?)?)
            }(_el)?,
            "partyName" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "partyName")
                    );
                }
                Ok(BER.validate_any(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::{Attribute, Attribute_valuesWithContext_Item};
    use wildboar_asn1::{Tag, TagClass, oid, UNIV_TAG_PRINTABLE_STRING};
    use x690::{X690Element, X690Value};
    use bytes::Bytes;

    fn get_attr_value(i: usize) -> X690Element {
        X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_PRINTABLE_STRING),
            X690Value::Primitive(Bytes::from(i.to_string()))
        )
    }


    #[test]
    fn test_attr_values_iter() {
        let attr = Attribute::new(
            oid!(2,5,4,3),
            vec![
                get_attr_value(0),
                get_attr_value(1),
                get_attr_value(2),
            ],
            Some(vec![
                Attribute_valuesWithContext_Item::new(
                    get_attr_value(3),
                    vec![],
                    vec![],
                ),
            ]),
            vec![],
        );
        assert_eq!(attr.iter_values().size_hint(), (4, Some(4)));
        assert_eq!(attr.iter_values().len(), 4);
        let v: Vec<_> = attr.iter_values().collect();
        assert_eq!(v.len(), 4);
    }

}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeValue ::= ATTRIBUTE.&Type
/// ```
pub struct AttributeValue(pub X690Element); // ObjectClassFieldType

pub fn _decode_AttributeValue(el: &X690Element) -> ASN1Result<AttributeValue> {
    Ok(AttributeValue(el.clone()))
}

pub fn _encode_AttributeValue(value_: &AttributeValue) -> ASN1Result<X690Element> {
    Ok(value_.0.clone())
}

pub fn _validate_AttributeValue(el: &X690Element) -> ASN1Result<()> {
    Ok(())
}
