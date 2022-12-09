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
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-mca  OBJECT IDENTIFIER ::= {id-algo 1}
/// ```
///
///
pub fn id_algo_mca() -> OBJECT_IDENTIFIER {
    [id_algo(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-ska  OBJECT IDENTIFIER ::= {id-algo 2}
/// ```
///
///
pub fn id_algo_ska() -> OBJECT_IDENTIFIER {
    [id_algo(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-aead OBJECT IDENTIFIER ::= {id-algo 3}
/// ```
///
///
pub fn id_algo_aead() -> OBJECT_IDENTIFIER {
    [id_algo(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-pka  OBJECT IDENTIFIER ::= {id-algo 4}
/// ```
///
///
pub fn id_algo_pka() -> OBJECT_IDENTIFIER {
    [id_algo(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-ha   OBJECT IDENTIFIER ::= {id-algo 5}
/// ```
///
///
pub fn id_algo_ha() -> OBJECT_IDENTIFIER {
    [id_algo(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dsa  OBJECT IDENTIFIER ::= {id-algo 6}
/// ```
///
///
pub fn id_algo_dsa() -> OBJECT_IDENTIFIER {
    [id_algo(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-kea  OBJECT IDENTIFIER ::= {id-algo 7}
/// ```
///
///
pub fn id_algo_kea() -> OBJECT_IDENTIFIER {
    [id_algo(), Vec::<u32>::from([7])].concat() // OID_GETTER
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// Group14  ::=  INTEGER (14)
/// ```
pub type Group14 = INTEGER;

pub fn _decode_Group14(el: &X690Element) -> ASN1Result<Group14> {
    ber_decode_integer(&el)
}

pub fn _encode_Group14(value_: &Group14) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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
impl TryFrom<X690Element> for Payload14 {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Payload14(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Payload14 {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Payload14> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Payload14,
            _eal_components_for_Payload14,
            _rctl2_components_for_Payload14,
        )?;
        let dhPublicKey = ber_decode_octet_string(_components.get("dhPublicKey").unwrap())?;
        let nonce = ber_decode_octet_string(_components.get("nonce").unwrap())?;
        Ok(Payload14 {
            dhPublicKey,
            nonce,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Payload14(value_: &Payload14) -> ASN1Result<X690Element> {
    |value_: &Payload14| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_octet_string(&value_.dhPublicKey)?);
        components_.push(ber_encode_octet_string(&value_.nonce)?);
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// Group23  ::=  INTEGER (23)
/// ```
pub type Group23 = INTEGER;

pub fn _decode_Group23(el: &X690Element) -> ASN1Result<Group23> {
    ber_decode_integer(&el)
}

pub fn _encode_Group23(value_: &Group23) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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
impl TryFrom<X690Element> for Payload23 {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Payload23(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Payload23 {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Payload23> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Payload23,
            _eal_components_for_Payload23,
            _rctl2_components_for_Payload23,
        )?;
        let dhPublicKey = ber_decode_octet_string(_components.get("dhPublicKey").unwrap())?;
        let nonce = ber_decode_octet_string(_components.get("nonce").unwrap())?;
        Ok(Payload23 {
            dhPublicKey,
            nonce,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Payload23(value_: &Payload23) -> ASN1Result<X690Element> {
    |value_: &Payload23| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_octet_string(&value_.dhPublicKey)?);
        components_.push(ber_encode_octet_string(&value_.nonce)?);
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// Group28  ::=  INTEGER (28)
/// ```
pub type Group28 = INTEGER;

pub fn _decode_Group28(el: &X690Element) -> ASN1Result<Group28> {
    ber_decode_integer(&el)
}

pub fn _encode_Group28(value_: &Group28) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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
impl TryFrom<X690Element> for Payload28 {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Payload28(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Payload28 {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Payload28> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Payload28,
            _eal_components_for_Payload28,
            _rctl2_components_for_Payload28,
        )?;
        let dhPublicKey = ber_decode_octet_string(_components.get("dhPublicKey").unwrap())?;
        let nonce = ber_decode_octet_string(_components.get("nonce").unwrap())?;
        Ok(Payload28 {
            dhPublicKey,
            nonce,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Payload28(value_: &Payload28) -> ASN1Result<X690Element> {
    |value_: &Payload28| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_octet_string(&value_.dhPublicKey)?);
        components_.push(ber_encode_octet_string(&value_.nonce)?);
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
/// id-algo-dhModpGr14Hkdf256Algo       OBJECT IDENTIFIER ::= {id-algo-kea 1}
/// ```
///
///
pub fn id_algo_dhModpGr14Hkdf256Algo() -> OBJECT_IDENTIFIER {
    [id_algo_kea(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr15Hkdf384Algo       OBJECT IDENTIFIER ::= {id-algo-kea 2}
/// ```
///
///
pub fn id_algo_dhModpGr15Hkdf384Algo() -> OBJECT_IDENTIFIER {
    [id_algo_kea(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr16Hkdf512Algo       OBJECT IDENTIFIER ::= {id-algo-kea 3}
/// ```
///
///
pub fn id_algo_dhModpGr16Hkdf512Algo() -> OBJECT_IDENTIFIER {
    [id_algo_kea(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr17Hkdf768Algo       OBJECT IDENTIFIER ::= {id-algo-kea 4}
/// ```
///
///
pub fn id_algo_dhModpGr17Hkdf768Algo() -> OBJECT_IDENTIFIER {
    [id_algo_kea(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr18Hkdf1024Algo      OBJECT IDENTIFIER ::= {id-algo-kea 5}
/// ```
///
///
pub fn id_algo_dhModpGr18Hkdf1024Algo() -> OBJECT_IDENTIFIER {
    [id_algo_kea(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr23Hkdf256Algo       OBJECT IDENTIFIER ::= {id-algo-kea 10}
/// ```
///
///
pub fn id_algo_dhModpGr23Hkdf256Algo() -> OBJECT_IDENTIFIER {
    [id_algo_kea(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-algo-dhModpGr28Hkdf256Algo       OBJECT IDENTIFIER ::= {id-algo-kea 15}
/// ```
///
///
pub fn id_algo_dhModpGr28Hkdf256Algo() -> OBJECT_IDENTIFIER {
    [id_algo_kea(), Vec::<u32>::from([15])].concat() // OID_GETTER
}
