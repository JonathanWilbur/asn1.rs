#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # CryptoTools
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `CryptoTools`.
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
use crate::GenAlgo::*;
use crate::PKI_Stub::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// multipleSignaturesAlgo ALGORITHM ::= {
///   PARMS         MultipleSignaturesAlgo
///   IDENTIFIED BY id-algo-multipleSignaturesAlgo }
/// ```
///
///
pub fn multipleSignaturesAlgo() -> ALGORITHM {
    ALGORITHM {
        id: id_algo_multipleSignaturesAlgo(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod multipleSignaturesAlgo {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MultipleSignaturesAlgo; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MultipleSignaturesAlgo(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MultipleSignaturesAlgo(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MultipleSignaturesAlgo(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MultipleSignaturesAlgo  ::=  SEQUENCE SIZE (1..MAX) OF
///   algo  AlgorithmIdentifier{{SupportedSignatureAlgorithms}}
/// ```
pub type MultipleSignaturesAlgo = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_MultipleSignaturesAlgo(el: &X690Element) -> ASN1Result<MultipleSignaturesAlgo> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "MultipleSignaturesAlgo",
            ))
        }
    };
    let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AlgorithmIdentifier(el)?);
    }
    Ok(items)
}

pub fn _encode_MultipleSignaturesAlgo(value_: &MultipleSignaturesAlgo) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AlgorithmIdentifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_MultipleSignaturesAlgo(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AlgorithmIdentifier(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "MultipleSignaturesAlgo",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedSignatureAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedSignatureAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// multipleSymmetricKeyAlgo ALGORITHM ::= {
///   PARMS         MultipleSymmetricKeyAlgo
///   IDENTIFIED BY id-algo-multipleSymmetricKeyAlgo }
/// ```
///
///
pub fn multipleSymmetricKeyAlgo() -> ALGORITHM {
    ALGORITHM {
        id: id_algo_multipleSymmetricKeyAlgo(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod multipleSymmetricKeyAlgo {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MultipleSymmetricKeyAlgo; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MultipleSymmetricKeyAlgo(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MultipleSymmetricKeyAlgo(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MultipleSymmetricKeyAlgo(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MultipleSymmetricKeyAlgo  ::=  SEQUENCE SIZE (1..MAX) OF
///   algo  AlgorithmIdentifier{{SupportedSymmetricKeyAlgorithms}}
/// ```
pub type MultipleSymmetricKeyAlgo = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_MultipleSymmetricKeyAlgo(el: &X690Element) -> ASN1Result<MultipleSymmetricKeyAlgo> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "MultipleSymmetricKeyAlgo",
            ))
        }
    };
    let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AlgorithmIdentifier(el)?);
    }
    Ok(items)
}

pub fn _encode_MultipleSymmetricKeyAlgo(
    value_: &MultipleSymmetricKeyAlgo,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AlgorithmIdentifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_MultipleSymmetricKeyAlgo(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AlgorithmIdentifier(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "MultipleSymmetricKeyAlgo",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedSymmetricKeyAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedSymmetricKeyAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// multiplePublicKeyAlgo ALGORITHM ::= {
///   PARMS         MultiplePublicKeyAlgo
///   IDENTIFIED BY id-algo-multiplePublicKeyAlgo }
/// ```
///
///
pub fn multiplePublicKeyAlgo() -> ALGORITHM {
    ALGORITHM {
        id: id_algo_multiplePublicKeyAlgo(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod multiplePublicKeyAlgo {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MultiplePublicKeyAlgo; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MultiplePublicKeyAlgo(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MultiplePublicKeyAlgo(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MultiplePublicKeyAlgo(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MultiplePublicKeyAlgo  ::=  SEQUENCE SIZE (1..MAX) OF
///   algo  AlgorithmIdentifier{{SupportedPublicKeyAlgorithms}}
/// ```
pub type MultiplePublicKeyAlgo = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_MultiplePublicKeyAlgo(el: &X690Element) -> ASN1Result<MultiplePublicKeyAlgo> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MultiplePublicKeyAlgo")
            )
        }
    };
    let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AlgorithmIdentifier(el)?);
    }
    Ok(items)
}

pub fn _encode_MultiplePublicKeyAlgo(value_: &MultiplePublicKeyAlgo) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AlgorithmIdentifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_MultiplePublicKeyAlgo(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AlgorithmIdentifier(&sub)?;
            }
            Ok(())
        }
        _ => {
            Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MultiplePublicKeyAlgo"))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedPublicKeyAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedPublicKeyAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// multipleHashAlgo ALGORITHM ::= {
///   PARMS         MultipleHashAlgo
///   IDENTIFIED BY id-algo-multipleHashAlgo }
/// ```
///
///
pub fn multipleHashAlgo() -> ALGORITHM {
    ALGORITHM {
        id: id_algo_multipleHashAlgo(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod multipleHashAlgo {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MultipleHashAlgo; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MultipleHashAlgo(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MultipleHashAlgo(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MultipleHashAlgo(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MultipleHashAlgo  ::=  SEQUENCE SIZE (1..MAX) OF
///   algo  AlgorithmIdentifier{{SupportedHashAlgorithms}}
/// ```
pub type MultipleHashAlgo = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_MultipleHashAlgo(el: &X690Element) -> ASN1Result<MultipleHashAlgo> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MultipleHashAlgo")
            )
        }
    };
    let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AlgorithmIdentifier(el)?);
    }
    Ok(items)
}

pub fn _encode_MultipleHashAlgo(value_: &MultipleHashAlgo) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AlgorithmIdentifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_MultipleHashAlgo(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AlgorithmIdentifier(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MultipleHashAlgo")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedHashAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedHashAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// multipleAuthenEncryptAlgo ALGORITHM ::= {
///   PARMS         MultipleAuthenEncryptAlgo
///   IDENTIFIED BY id-algo-multipleAuthenEncryptAlgo }
/// ```
///
///
pub fn multipleAuthenEncryptAlgo() -> ALGORITHM {
    ALGORITHM {
        id: id_algo_multipleAuthenEncryptAlgo(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod multipleAuthenEncryptAlgo {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MultipleAuthenEncryptAlgo; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MultipleAuthenEncryptAlgo(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MultipleAuthenEncryptAlgo(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MultipleAuthenEncryptAlgo(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MultipleAuthenEncryptAlgo  ::=  SEQUENCE SIZE (1..MAX) OF
///   algo       AlgorithmIdentifier{{SupportedAuthenEncryptAlgorithms}}
/// ```
pub type MultipleAuthenEncryptAlgo = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_MultipleAuthenEncryptAlgo(
    el: &X690Element,
) -> ASN1Result<MultipleAuthenEncryptAlgo> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "MultipleAuthenEncryptAlgo",
            ))
        }
    };
    let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AlgorithmIdentifier(el)?);
    }
    Ok(items)
}

pub fn _encode_MultipleAuthenEncryptAlgo(
    value_: &MultipleAuthenEncryptAlgo,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AlgorithmIdentifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_MultipleAuthenEncryptAlgo(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AlgorithmIdentifier(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "MultipleAuthenEncryptAlgo",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedAuthenEncryptAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedAuthenEncryptAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// multipleIcvAlgo ALGORITHM ::= {
///   PARMS         MultipleIcvAlgo
///   IDENTIFIED BY id-algo-multipleIcvAlgo }
/// ```
///
///
pub fn multipleIcvAlgo() -> ALGORITHM {
    ALGORITHM {
        id: id_algo_multipleIcvAlgo(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod multipleIcvAlgo {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MultipleIcvAlgo; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MultipleIcvAlgo(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MultipleIcvAlgo(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MultipleIcvAlgo(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MultipleIcvAlgo  ::=  SEQUENCE SIZE (1..MAX) OF
///   algo  AlgorithmIdentifier{{SupportedIcvAlgorithms}}
/// ```
pub type MultipleIcvAlgo = Vec<AlgorithmIdentifier>; // SequenceOfType

pub fn _decode_MultipleIcvAlgo(el: &X690Element) -> ASN1Result<MultipleIcvAlgo> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MultipleIcvAlgo"))
        }
    };
    let mut items: SEQUENCE_OF<AlgorithmIdentifier> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AlgorithmIdentifier(el)?);
    }
    Ok(items)
}

pub fn _encode_MultipleIcvAlgo(value_: &MultipleIcvAlgo) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AlgorithmIdentifier(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_MultipleIcvAlgo(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AlgorithmIdentifier(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MultipleIcvAlgo")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedIcvAlgorithms ALGORITHM ::= {...}
/// ```
///
///
pub fn SupportedIcvAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MULTY-SIGNED{ToBeSigned} ::= SEQUENCE {
///   toBeSigned  ToBeSigned,
///   algorithm   ALGORITHM.&id({multipleSignaturesAlgo}),
///   parmeters     SEQUENCE SIZE (1..MAX) OF
///     sign          SEQUENCE {
///       algo          AlgorithmIdentifier{{SupportedSignatureAlgorithms}},
///       signature     BIT STRING,
///       ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct MULTY_SIGNED<ToBeSigned> {
    pub toBeSigned: ToBeSigned,
    pub algorithm: OBJECT_IDENTIFIER,
    pub parmeters: Vec<MULTY_SIGNED_parmeters_sign>,
    pub _unrecognized: Vec<X690Element>,
}
impl<ToBeSigned> MULTY_SIGNED<ToBeSigned> {
    pub fn new(
        toBeSigned: ToBeSigned,
        algorithm: OBJECT_IDENTIFIER,
        parmeters: Vec<MULTY_SIGNED_parmeters_sign>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        MULTY_SIGNED {
            toBeSigned,
            algorithm,
            parmeters,
            _unrecognized,
        }
    }
}

pub const _rctl1_components_for_MULTY_SIGNED: &[ComponentSpec; 3] = &[
    ComponentSpec::new("toBeSigned", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "parmeters",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_MULTY_SIGNED: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MULTY_SIGNED: &[ComponentSpec; 0] = &[];

pub fn _decode_MULTY_SIGNED<ToBeSigned>(
    _decode_ToBeSigned: fn(&X690Element) -> ASN1Result<ToBeSigned>,
    el: &X690Element,
) -> ASN1Result<MULTY_SIGNED<ToBeSigned>> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MULTY-SIGNED")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MULTY_SIGNED,
        _eal_components_for_MULTY_SIGNED,
        _rctl2_components_for_MULTY_SIGNED,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut toBeSigned_: OPTIONAL<ToBeSigned> = None;
    let mut algorithm_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut parmeters_: OPTIONAL<Vec<MULTY_SIGNED_parmeters_sign>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "toBeSigned" => toBeSigned_ = Some(_decode_ToBeSigned(_el)?),
            "algorithm" => algorithm_ = Some(BER.decode_object_identifier(_el)?),
            "parmeters" => {
                parmeters_ = Some(|el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<MULTY_SIGNED_parmeters_sign>,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "parmeters",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<MULTY_SIGNED_parmeters_sign> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_MULTY_SIGNED_parmeters_sign(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(MULTY_SIGNED {
        toBeSigned: toBeSigned_.unwrap(),
        algorithm: algorithm_.unwrap(),
        parmeters: parmeters_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_MULTY_SIGNED<ToBeSigned>(
    _encode_ToBeSigned: fn(&ToBeSigned) -> ASN1Result<X690Element>,
    value_: &MULTY_SIGNED<ToBeSigned>,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_ToBeSigned(&value_.toBeSigned)?);
    components_.push(BER.encode_object_identifier(&value_.algorithm)?);
    components_.push(
        |value_: &SEQUENCE_OF<MULTY_SIGNED_parmeters_sign>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_MULTY_SIGNED_parmeters_sign(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.parmeters)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_MULTY_SIGNED(
    _validate_ToBeSigned: fn(&X690Element) -> ASN1Result<()>,
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MULTY-SIGNED")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MULTY_SIGNED,
        _eal_components_for_MULTY_SIGNED,
        _rctl2_components_for_MULTY_SIGNED,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "toBeSigned" => _validate_ToBeSigned(_el)?,
            "algorithm" => BER.validate_object_identifier(_el)?,
            "parmeters" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_MULTY_SIGNED_parmeters_sign(&sub)?;
                        }
                        Ok(())
                    }
                    _ => {
                        Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "parmeters"))
                    }
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
/// Signed{ToBeSigned} ::= SEQUENCE {
///   toBeSigned   ToBeSigned,
///   signature    BIT STRING,
///   altSignature BIT STRING OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Signed<ToBeSigned> {
    pub toBeSigned: ToBeSigned,
    pub signature: BIT_STRING,
    pub altSignature: OPTIONAL<BIT_STRING>,
    pub _unrecognized: Vec<X690Element>,
}
impl<ToBeSigned> Signed<ToBeSigned> {
    pub fn new(
        toBeSigned: ToBeSigned,
        signature: BIT_STRING,
        altSignature: OPTIONAL<BIT_STRING>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Signed {
            toBeSigned,
            signature,
            altSignature,
            _unrecognized,
        }
    }
}

pub const _rctl1_components_for_Signed: &[ComponentSpec; 3] = &[
    ComponentSpec::new("toBeSigned", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "signature",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
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

pub const _rctl2_components_for_Signed: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Signed: &[ComponentSpec; 0] = &[];

pub fn _decode_Signed<ToBeSigned: 'static>(
    _decode_ToBeSigned: fn(&X690Element) -> ASN1Result<ToBeSigned>,
    el: &X690Element,
) -> ASN1Result<Signed<ToBeSigned>> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Signed")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Signed,
        _eal_components_for_Signed,
        _rctl2_components_for_Signed,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut toBeSigned_: OPTIONAL<ToBeSigned> = None;
    let mut signature_: OPTIONAL<BIT_STRING> = None;
    let mut altSignature_: OPTIONAL<BIT_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "toBeSigned" => toBeSigned_ = Some(_decode_ToBeSigned(_el)?),
            "signature" => signature_ = Some(BER.decode_bit_string(_el)?),
            "altSignature" => altSignature_ = Some(BER.decode_bit_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Signed {
        toBeSigned: toBeSigned_.unwrap(),
        signature: signature_.unwrap(),
        altSignature: altSignature_,
        _unrecognized,
    })
}

pub fn _encode_Signed<ToBeSigned>(
    _encode_ToBeSigned: fn(&ToBeSigned) -> ASN1Result<X690Element>,
    value_: &Signed<ToBeSigned>,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_ToBeSigned(&value_.toBeSigned)?);
    components_.push(BER.encode_bit_string(&value_.signature)?);
    if let Some(v_) = &value_.altSignature {
        components_.push(BER.encode_bit_string(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Signed<ToBeSigned>(
    _validate_ToBeSigned: fn(&X690Element) -> ASN1Result<()>,
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Signed")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Signed,
        _eal_components_for_Signed,
        _rctl2_components_for_Signed,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "toBeSigned" => _validate_ToBeSigned(_el)?,
            "signature" => BER.validate_bit_string(_el)?,
            "altSignature" => BER.validate_bit_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ICV-Total{ToBeProtected} ::= SEQUENCE {
///   toBeProtected               ToBeProtected,
///   algorithmIdentifier         AlgorithmWithInvoke{{SupportedIcvAlgorithms}},
///   icv                         BIT STRING,
///   altAlgorithmIdentifier  [0] AlgorithmWithInvoke{{SupportedIcvAlgorithms}} OPTIONAL,
///   altIcv                  [1] BIT STRING OPTIONAL,
///   ... }
///    (WITH COMPONENTS {..., altAlgorithmIdentifier PRESENT, altIcv PRESENT } |
///     WITH COMPONENTS {..., altAlgorithmIdentifier ABSENT,  altIcv ABSENT } )
/// ```
///
#[derive(Debug, Clone)]
pub struct ICV_Total<ToBeProtected> {
    pub toBeProtected: ToBeProtected,
    pub algorithmIdentifier: AlgorithmWithInvoke,
    pub icv: BIT_STRING,
    pub altAlgorithmIdentifier: OPTIONAL<AlgorithmWithInvoke>,
    pub altIcv: OPTIONAL<BIT_STRING>,
    pub _unrecognized: Vec<X690Element>,
}
impl<ToBeProtected> ICV_Total<ToBeProtected> {
    pub fn new(
        toBeProtected: ToBeProtected,
        algorithmIdentifier: AlgorithmWithInvoke,
        icv: BIT_STRING,
        altAlgorithmIdentifier: OPTIONAL<AlgorithmWithInvoke>,
        altIcv: OPTIONAL<BIT_STRING>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ICV_Total {
            toBeProtected,
            algorithmIdentifier,
            icv,
            altAlgorithmIdentifier,
            altIcv,
            _unrecognized,
        }
    }
}

pub const _rctl1_components_for_ICV_Total: &[ComponentSpec; 5] = &[
    ComponentSpec::new("toBeProtected", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "algorithmIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "icv",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "altAlgorithmIdentifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "altIcv",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ICV_Total: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ICV_Total: &[ComponentSpec; 0] = &[];

pub fn _decode_ICV_Total<ToBeProtected: 'static>(
    _decode_ToBeProtected: fn(&X690Element) -> ASN1Result<ToBeProtected>,
    el: &X690Element,
) -> ASN1Result<ICV_Total<ToBeProtected>> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ICV-Total")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ICV_Total,
        _eal_components_for_ICV_Total,
        _rctl2_components_for_ICV_Total,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut toBeProtected_: OPTIONAL<ToBeProtected> = None;
    let mut algorithmIdentifier_: OPTIONAL<AlgorithmWithInvoke> = None;
    let mut icv_: OPTIONAL<BIT_STRING> = None;
    let mut altAlgorithmIdentifier_: OPTIONAL<AlgorithmWithInvoke> = None;
    let mut altIcv_: OPTIONAL<BIT_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "toBeProtected" => toBeProtected_ = Some(_decode_ToBeProtected(_el)?),
            "algorithmIdentifier" => algorithmIdentifier_ = Some(_decode_AlgorithmWithInvoke(_el)?),
            "icv" => icv_ = Some(BER.decode_bit_string(_el)?),
            "altAlgorithmIdentifier" => {
                altAlgorithmIdentifier_ = Some(_decode_AlgorithmWithInvoke(_el)?)
            }
            "altIcv" => altIcv_ = Some(BER.decode_bit_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ICV_Total {
        toBeProtected: toBeProtected_.unwrap(),
        algorithmIdentifier: algorithmIdentifier_.unwrap(),
        icv: icv_.unwrap(),
        altAlgorithmIdentifier: altAlgorithmIdentifier_,
        altIcv: altIcv_,
        _unrecognized,
    })
}

pub fn _encode_ICV_Total<ToBeProtected>(
    _encode_ToBeProtected: fn(&ToBeProtected) -> ASN1Result<X690Element>,
    value_: &ICV_Total<ToBeProtected>,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    components_.push(_encode_ToBeProtected(&value_.toBeProtected)?);
    components_.push(_encode_AlgorithmWithInvoke(&value_.algorithmIdentifier)?);
    components_.push(BER.encode_bit_string(&value_.icv)?);
    if let Some(v_) = &value_.altAlgorithmIdentifier {
        components_.push(|v_1: &AlgorithmWithInvoke| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AlgorithmWithInvoke(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.altIcv {
        components_.push(|v_1: &BIT_STRING| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_bit_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ICV_Total<ToBeProtected>(
    _validate_ToBeProtected: fn(&X690Element) -> ASN1Result<()>,
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ICV-Total")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ICV_Total,
        _eal_components_for_ICV_Total,
        _rctl2_components_for_ICV_Total,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "toBeProtected" => _validate_ToBeProtected(_el)?,
            "algorithmIdentifier" => _validate_AlgorithmWithInvoke(_el)?,
            "icv" => BER.validate_bit_string(_el)?,
            "altAlgorithmIdentifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "altAlgorithmIdentifier",
                    ));
                }
                Ok(_validate_AlgorithmWithInvoke(&el)?)
            }(_el)?,
            "altIcv" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "altIcv"));
                }
                Ok(BER.validate_bit_string(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ICV-Invoke{ToBeProtected} ::= SEQUENCE {
///   toBeProtected      ToBeProtected,
///   dynParms       [0] AlgoInvoke{{SupportedIcvAlgorithms}} OPTIONAL,
///   icv                BIT STRING,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct ICV_Invoke<ToBeProtected> {
    pub toBeProtected: ToBeProtected,
    pub dynParms: OPTIONAL<AlgoInvoke>,
    pub icv: BIT_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl<ToBeProtected> ICV_Invoke<ToBeProtected> {
    pub fn new(
        toBeProtected: ToBeProtected,
        dynParms: OPTIONAL<AlgoInvoke>,
        icv: BIT_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ICV_Invoke {
            toBeProtected,
            dynParms,
            icv,
            _unrecognized,
        }
    }
}

pub const _rctl1_components_for_ICV_Invoke: &[ComponentSpec; 3] = &[
    ComponentSpec::new("toBeProtected", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "dynParms",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "icv",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ICV_Invoke: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ICV_Invoke: &[ComponentSpec; 0] = &[];

pub fn _decode_ICV_Invoke<ToBeProtected>(
    _decode_ToBeProtected: fn(&X690Element) -> ASN1Result<ToBeProtected>,
    el: &X690Element,
) -> ASN1Result<ICV_Invoke<ToBeProtected>> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ICV-Invoke")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ICV_Invoke,
        _eal_components_for_ICV_Invoke,
        _rctl2_components_for_ICV_Invoke,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut toBeProtected_: OPTIONAL<ToBeProtected> = None;
    let mut dynParms_: OPTIONAL<AlgoInvoke> = None;
    let mut icv_: OPTIONAL<BIT_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "toBeProtected" => toBeProtected_ = Some(_decode_ToBeProtected(_el)?),
            "dynParms" => dynParms_ = Some(_decode_AlgoInvoke(_el)?),
            "icv" => icv_ = Some(BER.decode_bit_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ICV_Invoke {
        toBeProtected: toBeProtected_.unwrap(),
        dynParms: dynParms_,
        icv: icv_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ICV_Invoke<ToBeProtected>(
    _encode_ToBeProtected: fn(&ToBeProtected) -> ASN1Result<X690Element>,
    value_: &ICV_Invoke<ToBeProtected>,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_ToBeProtected(&value_.toBeProtected)?);
    if let Some(v_) = &value_.dynParms {
        components_.push(|v_1: &AlgoInvoke| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AlgoInvoke(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(BER.encode_bit_string(&value_.icv)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ICV_Invoke<ToBeProtected>(
    _validate_ToBeProtected: fn(&X690Element) -> ASN1Result<()>,
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ICV-Invoke")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ICV_Invoke,
        _eal_components_for_ICV_Invoke,
        _rctl2_components_for_ICV_Invoke,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "toBeProtected" => _validate_ToBeProtected(_el)?,
            "dynParms" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "dynParms")
                    );
                }
                Ok(_validate_AlgoInvoke(&el)?)
            }(_el)?,
            "icv" => BER.validate_bit_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ENCIPHERED{ToBeEnciphered}  ::=  OCTET STRING (CONSTRAINED BY {
///    -- shall be the result of applying an encipherment procedure
///    -- to the BER-encoded octets of a value of -- ToBeEnciphered } )
/// ```
pub type ENCIPHERED = OCTET_STRING; // OctetStringType

pub fn _decode_ENCIPHERED(el: &X690Element) -> ASN1Result<ENCIPHERED> {
    BER.decode_octet_string(&el)
}

pub fn _encode_ENCIPHERED(value_: &ENCIPHERED) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_ENCIPHERED(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AUTHEN-ENCRYPT{ToBeAuth, ToBeEnciphered} ::= SEQUENCE {
///   aad  [0] ToBeAuth OPTIONAL,
///   encr [1] ToBeEnciphered,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct AUTHEN_ENCRYPT<ToBeAuth, ToBeEnciphered> {
    pub aad: OPTIONAL<ToBeAuth>,
    pub encr: ToBeEnciphered,
    pub _unrecognized: Vec<X690Element>,
}
impl<ToBeAuth, ToBeEnciphered> AUTHEN_ENCRYPT<ToBeAuth, ToBeEnciphered> {
    pub fn new(
        aad: OPTIONAL<ToBeAuth>,
        encr: ToBeEnciphered,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AUTHEN_ENCRYPT {
            aad,
            encr,
            _unrecognized,
        }
    }
}

pub const _rctl1_components_for_AUTHEN_ENCRYPT: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "aad",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encr",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AUTHEN_ENCRYPT: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AUTHEN_ENCRYPT: &[ComponentSpec; 0] = &[];

pub fn _decode_AUTHEN_ENCRYPT<ToBeAuth, ToBeEnciphered>(
    _decode_ToBeAuth: fn(&X690Element) -> ASN1Result<ToBeAuth>,
    _decode_ToBeEnciphered: fn(&X690Element) -> ASN1Result<ToBeEnciphered>,
    el: &X690Element,
) -> ASN1Result<AUTHEN_ENCRYPT<ToBeAuth, ToBeEnciphered>> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AUTHEN-ENCRYPT"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AUTHEN_ENCRYPT,
        _eal_components_for_AUTHEN_ENCRYPT,
        _rctl2_components_for_AUTHEN_ENCRYPT,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut aad_: OPTIONAL<ToBeAuth> = None;
    let mut encr_: OPTIONAL<ToBeEnciphered> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "aad" => aad_ = Some(_decode_ToBeAuth(_el)?),
            "encr" => encr_ = Some(_decode_ToBeEnciphered(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AUTHEN_ENCRYPT {
        aad: aad_,
        encr: encr_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AUTHEN_ENCRYPT<ToBeAuth, ToBeEnciphered>(
    _encode_ToBeAuth: fn(&ToBeAuth) -> ASN1Result<X690Element>,
    _encode_ToBeEnciphered: fn(&ToBeEnciphered) -> ASN1Result<X690Element>,
    value_: &AUTHEN_ENCRYPT<ToBeAuth, ToBeEnciphered>,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.aad {
        components_.push(|v_1: &ToBeAuth| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ToBeAuth(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(|v_1: &ToBeEnciphered| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_ToBeEnciphered(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 1;
        Ok(el_1)
    }(&value_.encr)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AUTHEN_ENCRYPT<ToBeAuth, ToBeEnciphered>(
    _validate_ToBeAuth: fn(&X690Element) -> ASN1Result<()>,
    _validate_ToBeEnciphered: fn(&X690Element) -> ASN1Result<()>,
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AUTHEN-ENCRYPT"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AUTHEN_ENCRYPT,
        _eal_components_for_AUTHEN_ENCRYPT,
        _rctl2_components_for_AUTHEN_ENCRYPT,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "aad" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "aad"));
                }
                Ok(_validate_ToBeAuth(&el)?)
            }(_el)?,
            "encr" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "encr"));
                }
                Ok(_validate_ToBeEnciphered(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-multipleSignaturesAlgo      OBJECT IDENTIFIER ::= {id-algo-mca 1}
/// ```
///
///
#[inline]
pub fn id_algo_multipleSignaturesAlgo () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo_mca(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-multipleSymmetricKeyAlgo    OBJECT IDENTIFIER ::= {id-algo-mca 2}
/// ```
///
///
#[inline]
pub fn id_algo_multipleSymmetricKeyAlgo () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo_mca(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-multiplePublicKeyAlgo       OBJECT IDENTIFIER ::= {id-algo-mca 3}
/// ```
///
///
#[inline]
pub fn id_algo_multiplePublicKeyAlgo () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo_mca(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-multipleHashAlgo            OBJECT IDENTIFIER ::= {id-algo-mca 4}
/// ```
///
///
#[inline]
pub fn id_algo_multipleHashAlgo () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo_mca(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-multipleAuthenEncryptAlgo   OBJECT IDENTIFIER ::= {id-algo-mca 5}
/// ```
///
///
#[inline]
pub fn id_algo_multipleAuthenEncryptAlgo () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo_mca(), 5).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-multipleIcvAlgo             OBJECT IDENTIFIER ::= {id-algo-mca 6}
/// ```
///
///
#[inline]
pub fn id_algo_multipleIcvAlgo () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo_mca(), 6).unwrap() // OID_GETTER
}
/// ### ASN.1 Definition:
///
/// ```asn1
/// MULTY-SIGNED-parmeters-sign ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct MULTY_SIGNED_parmeters_sign {
    pub algo: AlgorithmIdentifier,
    pub signature: BIT_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl MULTY_SIGNED_parmeters_sign {
    pub fn new(
        algo: AlgorithmIdentifier,
        signature: BIT_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        MULTY_SIGNED_parmeters_sign {
            algo,
            signature,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for MULTY_SIGNED_parmeters_sign {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_MULTY_SIGNED_parmeters_sign(el)
    }
}

pub const _rctl1_components_for_MULTY_SIGNED_parmeters_sign: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algo",
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

pub const _rctl2_components_for_MULTY_SIGNED_parmeters_sign: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MULTY_SIGNED_parmeters_sign: &[ComponentSpec; 0] = &[];

pub fn _decode_MULTY_SIGNED_parmeters_sign(
    el: &X690Element,
) -> ASN1Result<MULTY_SIGNED_parmeters_sign> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "MULTY-SIGNED-parmeters-sign",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MULTY_SIGNED_parmeters_sign,
        _eal_components_for_MULTY_SIGNED_parmeters_sign,
        _rctl2_components_for_MULTY_SIGNED_parmeters_sign,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algo_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut signature_: OPTIONAL<BIT_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algo" => algo_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "signature" => signature_ = Some(BER.decode_bit_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(MULTY_SIGNED_parmeters_sign {
        algo: algo_.unwrap(),
        signature: signature_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_MULTY_SIGNED_parmeters_sign(
    value_: &MULTY_SIGNED_parmeters_sign,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_AlgorithmIdentifier(&value_.algo)?);
    components_.push(BER.encode_bit_string(&value_.signature)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_MULTY_SIGNED_parmeters_sign(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "MULTY-SIGNED-parmeters-sign",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MULTY_SIGNED_parmeters_sign,
        _eal_components_for_MULTY_SIGNED_parmeters_sign,
        _rctl2_components_for_MULTY_SIGNED_parmeters_sign,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algo" => _validate_AlgorithmIdentifier(_el)?,
            "signature" => BER.validate_bit_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}
