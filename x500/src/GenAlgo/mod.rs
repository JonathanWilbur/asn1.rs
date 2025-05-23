#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # GenAlgo
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `GenAlgo`.
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
use crate::PKI_Stub::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-mca  OBJECT IDENTIFIER ::= {id-algo 1}
/// ```
///
///
#[inline]
pub fn id_algo_mca () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-ska  OBJECT IDENTIFIER ::= {id-algo 2}
/// ```
///
///
#[inline]
pub fn id_algo_ska () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-aead OBJECT IDENTIFIER ::= {id-algo 3}
/// ```
///
///
#[inline]
pub fn id_algo_aead () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-pka  OBJECT IDENTIFIER ::= {id-algo 4}
/// ```
///
///
#[inline]
pub fn id_algo_pka () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-ha   OBJECT IDENTIFIER ::= {id-algo 5}
/// ```
///
///
#[inline]
pub fn id_algo_ha () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo(), 5).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dsa  OBJECT IDENTIFIER ::= {id-algo 6}
/// ```
///
///
#[inline]
pub fn id_algo_dsa () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo(), 6).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-kea  OBJECT IDENTIFIER ::= {id-algo 7}
/// ```
///
///
#[inline]
pub fn id_algo_kea () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_algo(), 7).unwrap() // OID_GETTER
}


/// ### ASN.1 Definition:
///
/// ```asn1
/// dhModpGr14Hkdf256Algo ALGORITHM ::= {
///   PARMS         Group14
///   DYN-PARMS     Payload14
///   IDENTIFIED BY id-algo-dhModpGr14Hkdf256Algo }
/// ```
///
///
pub fn dhModpGr14Hkdf256Algo() -> ALGORITHM {
    ALGORITHM {
        id: id_algo_dhModpGr14Hkdf256Algo(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod dhModpGr14Hkdf256Algo {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Group14; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Group14(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Group14(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Group14(el)
    }
    pub type DynParms = Payload14; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_DynParms(el: &X690Element) -> ASN1Result<DynParms> {
        _decode_Payload14(el)
    }
    pub fn _encode_DynParms(value_: &DynParms) -> ASN1Result<X690Element> {
        _encode_Payload14(value_)
    }
    pub fn _validate_DynParms(el: &X690Element) -> ASN1Result<()> {
        _validate_Payload14(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Group14  ::=  INTEGER (14)
/// ```
pub type Group14 = INTEGER;

pub fn _decode_Group14(el: &X690Element) -> ASN1Result<Group14> {
    BER.decode_integer(&el)
}

pub fn _encode_Group14(value_: &Group14) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_Group14(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Payload14 ::= SEQUENCE {
///   dhPublicKey OCTET STRING (SIZE (256)),
///   nonce       OCTET STRING (SIZE (32)),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Payload14 {
    pub dhPublicKey: OCTET_STRING,
    pub nonce: OCTET_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl Payload14 {
    pub fn new(
        dhPublicKey: OCTET_STRING,
        nonce: OCTET_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Payload14 {
            dhPublicKey,
            nonce,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for Payload14 {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Payload14(el)
    }
}

pub const _rctl1_components_for_Payload14: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "dhPublicKey",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nonce",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Payload14: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Payload14: &[ComponentSpec; 0] = &[];

pub fn _decode_Payload14(el: &X690Element) -> ASN1Result<Payload14> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Payload14")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Payload14,
        _eal_components_for_Payload14,
        _rctl2_components_for_Payload14,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut dhPublicKey_: OPTIONAL<OCTET_STRING> = None;
    let mut nonce_: OPTIONAL<OCTET_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "dhPublicKey" => dhPublicKey_ = Some(BER.decode_octet_string(_el)?),
            "nonce" => nonce_ = Some(BER.decode_octet_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Payload14 {
        dhPublicKey: dhPublicKey_.unwrap(),
        nonce: nonce_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_Payload14(value_: &Payload14) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_octet_string(&value_.dhPublicKey)?);
    components_.push(BER.encode_octet_string(&value_.nonce)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Payload14(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Payload14")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Payload14,
        _eal_components_for_Payload14,
        _rctl2_components_for_Payload14,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "dhPublicKey" => BER.validate_octet_string(_el)?,
            "nonce" => BER.validate_octet_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dhModpGr23Hkdf256Algo ALGORITHM ::= {
///   PARMS         Group23
///   DYN-PARMS     Payload23
///   IDENTIFIED BY id-algo-dhModpGr23Hkdf256Algo }
/// ```
///
///
pub fn dhModpGr23Hkdf256Algo() -> ALGORITHM {
    ALGORITHM {
        id: id_algo_dhModpGr23Hkdf256Algo(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod dhModpGr23Hkdf256Algo {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Group23; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Group23(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Group23(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Group23(el)
    }
    pub type DynParms = Payload23; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_DynParms(el: &X690Element) -> ASN1Result<DynParms> {
        _decode_Payload23(el)
    }
    pub fn _encode_DynParms(value_: &DynParms) -> ASN1Result<X690Element> {
        _encode_Payload23(value_)
    }
    pub fn _validate_DynParms(el: &X690Element) -> ASN1Result<()> {
        _validate_Payload23(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Group23  ::=  INTEGER (23)
/// ```
pub type Group23 = INTEGER;

pub fn _decode_Group23(el: &X690Element) -> ASN1Result<Group23> {
    BER.decode_integer(&el)
}

pub fn _encode_Group23(value_: &Group23) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_Group23(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Payload23 ::= SEQUENCE {
///   dhPublicKey OCTET STRING (SIZE (512)),
///   nonce       OCTET STRING (SIZE (32)),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Payload23 {
    pub dhPublicKey: OCTET_STRING,
    pub nonce: OCTET_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl Payload23 {
    pub fn new(
        dhPublicKey: OCTET_STRING,
        nonce: OCTET_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Payload23 {
            dhPublicKey,
            nonce,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for Payload23 {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Payload23(el)
    }
}

pub const _rctl1_components_for_Payload23: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "dhPublicKey",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nonce",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Payload23: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Payload23: &[ComponentSpec; 0] = &[];

pub fn _decode_Payload23(el: &X690Element) -> ASN1Result<Payload23> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Payload23")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Payload23,
        _eal_components_for_Payload23,
        _rctl2_components_for_Payload23,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut dhPublicKey_: OPTIONAL<OCTET_STRING> = None;
    let mut nonce_: OPTIONAL<OCTET_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "dhPublicKey" => dhPublicKey_ = Some(BER.decode_octet_string(_el)?),
            "nonce" => nonce_ = Some(BER.decode_octet_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Payload23 {
        dhPublicKey: dhPublicKey_.unwrap(),
        nonce: nonce_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_Payload23(value_: &Payload23) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_octet_string(&value_.dhPublicKey)?);
    components_.push(BER.encode_octet_string(&value_.nonce)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Payload23(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Payload23")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Payload23,
        _eal_components_for_Payload23,
        _rctl2_components_for_Payload23,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "dhPublicKey" => BER.validate_octet_string(_el)?,
            "nonce" => BER.validate_octet_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dhModpGr28Hkdf256Algo ALGORITHM ::= {
///   PARMS         Group28
///   DYN-PARMS     Payload28
///   IDENTIFIED BY id-algo-dhModpGr28Hkdf256Algo }
/// ```
///
///
pub fn dhModpGr28Hkdf256Algo() -> ALGORITHM {
    ALGORITHM {
        id: id_algo_dhModpGr28Hkdf256Algo(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod dhModpGr28Hkdf256Algo {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Group28; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Group28(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Group28(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Group28(el)
    }
    pub type DynParms = Payload28; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_DynParms(el: &X690Element) -> ASN1Result<DynParms> {
        _decode_Payload28(el)
    }
    pub fn _encode_DynParms(value_: &DynParms) -> ASN1Result<X690Element> {
        _encode_Payload28(value_)
    }
    pub fn _validate_DynParms(el: &X690Element) -> ASN1Result<()> {
        _validate_Payload28(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Group28  ::=  INTEGER (28)
/// ```
pub type Group28 = INTEGER;

pub fn _decode_Group28(el: &X690Element) -> ASN1Result<Group28> {
    BER.decode_integer(&el)
}

pub fn _encode_Group28(value_: &Group28) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_Group28(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Payload28 ::= SEQUENCE {
///   dhPublicKey OCTET STRING (SIZE (512)),
///   nonce       OCTET STRING (SIZE (32)),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Payload28 {
    pub dhPublicKey: OCTET_STRING,
    pub nonce: OCTET_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl Payload28 {
    pub fn new(
        dhPublicKey: OCTET_STRING,
        nonce: OCTET_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Payload28 {
            dhPublicKey,
            nonce,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for Payload28 {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Payload28(el)
    }
}

pub const _rctl1_components_for_Payload28: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "dhPublicKey",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nonce",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Payload28: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Payload28: &[ComponentSpec; 0] = &[];

pub fn _decode_Payload28(el: &X690Element) -> ASN1Result<Payload28> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Payload28")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Payload28,
        _eal_components_for_Payload28,
        _rctl2_components_for_Payload28,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut dhPublicKey_: OPTIONAL<OCTET_STRING> = None;
    let mut nonce_: OPTIONAL<OCTET_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "dhPublicKey" => dhPublicKey_ = Some(BER.decode_octet_string(_el)?),
            "nonce" => nonce_ = Some(BER.decode_octet_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Payload28 {
        dhPublicKey: dhPublicKey_.unwrap(),
        nonce: nonce_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_Payload28(value_: &Payload28) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_octet_string(&value_.dhPublicKey)?);
    components_.push(BER.encode_octet_string(&value_.nonce)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Payload28(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Payload28")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Payload28,
        _eal_components_for_Payload28,
        _rctl2_components_for_Payload28,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "dhPublicKey" => BER.validate_octet_string(_el)?,
            "nonce" => BER.validate_octet_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr14Hkdf256Algo       OBJECT IDENTIFIER ::= {id-algo-kea 1}
/// ```
///
///
#[inline]
pub fn id_algo_dhModpGr14Hkdf256Algo () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 44, 7, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr15Hkdf384Algo       OBJECT IDENTIFIER ::= {id-algo-kea 2}
/// ```
///
///
#[inline]
pub fn id_algo_dhModpGr15Hkdf384Algo () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 44, 7, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr16Hkdf512Algo       OBJECT IDENTIFIER ::= {id-algo-kea 3}
/// ```
///
///
#[inline]
pub fn id_algo_dhModpGr16Hkdf512Algo () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 44, 7, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr17Hkdf768Algo       OBJECT IDENTIFIER ::= {id-algo-kea 4}
/// ```
///
///
#[inline]
pub fn id_algo_dhModpGr17Hkdf768Algo () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 44, 7, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr18Hkdf1024Algo      OBJECT IDENTIFIER ::= {id-algo-kea 5}
/// ```
///
///
#[inline]
pub fn id_algo_dhModpGr18Hkdf1024Algo () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 44, 7, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr23Hkdf256Algo       OBJECT IDENTIFIER ::= {id-algo-kea 10}
/// ```
///
///
#[inline]
pub fn id_algo_dhModpGr23Hkdf256Algo () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 44, 7, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr28Hkdf256Algo       OBJECT IDENTIFIER ::= {id-algo-kea 15}
/// ```
///
///
#[inline]
pub fn id_algo_dhModpGr28Hkdf256Algo () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked([ 85, 44, 7, 15 ].as_slice()) } // OID_GETTER
}
