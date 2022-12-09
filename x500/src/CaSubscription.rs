#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # CaSubscription
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `CaSubscription`.
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
use crate::Wrapper::{Version, _decode_Version, _encode_Version};
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// CasubProt  ::=  CHOICE {
///   initReq             [0]  InitializationRec,
///   initAcc             [1]  InitializationAcc,
///   initRej             [2]  InitializationRej,
///   initAbt             [3]  InitializationAbort,
///   certSubscribeReq    [4]  CertSubscribeReq,
///   certSubscribeRsp    [5]  CertSubscribeRsp,
///   certUnsubscribeReq  [6]  CertUnsubscribeReq,
///   certUnsubscribeRsp  [7]  CertUnsubscribeRsp,
///   certReplaceReq      [8]  CertReplaceReq,
///   certReplaceRsp      [9]  CertReplaceRsp,
///   certUpdateReq       [10] CertUpdateReq,
///   certUpdateRsp       [11] CertUpdateRsp,
///   cAsubscribeAbort    [12] CAsubscribeAbort,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum CasubProt {
    initReq(InitializationRec),
    initAcc(InitializationAcc),
    initRej(InitializationRej),
    initAbt(InitializationAbort),
    certSubscribeReq(CertSubscribeReq),
    certSubscribeRsp(CertSubscribeRsp),
    certUnsubscribeReq(CertUnsubscribeReq),
    certUnsubscribeRsp(CertUnsubscribeRsp),
    certReplaceReq(CertReplaceReq),
    certReplaceRsp(CertReplaceRsp),
    certUpdateReq(CertUpdateReq),
    certUpdateRsp(CertUpdateRsp),
    cAsubscribeAbort(CAsubscribeAbort),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CasubProt {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CasubProt(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CasubProt {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CasubProt(el)
    }
}

pub fn _decode_CasubProt(el: &X690Element) -> ASN1Result<CasubProt> {
    |el: &X690Element| -> ASN1Result<CasubProt> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(CasubProt::initReq(_decode_InitializationRec(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(CasubProt::initAcc(_decode_InitializationAcc(&el)?)),
            (TagClass::CONTEXT, 2) => Ok(CasubProt::initRej(_decode_InitializationRej(&el)?)),
            (TagClass::CONTEXT, 3) => Ok(CasubProt::initAbt(_decode_InitializationAbort(&el)?)),
            (TagClass::CONTEXT, 4) => {
                Ok(CasubProt::certSubscribeReq(_decode_CertSubscribeReq(&el)?))
            }
            (TagClass::CONTEXT, 5) => {
                Ok(CasubProt::certSubscribeRsp(_decode_CertSubscribeRsp(&el)?))
            }
            (TagClass::CONTEXT, 6) => Ok(CasubProt::certUnsubscribeReq(
                _decode_CertUnsubscribeReq(&el)?,
            )),
            (TagClass::CONTEXT, 7) => Ok(CasubProt::certUnsubscribeRsp(
                _decode_CertUnsubscribeRsp(&el)?,
            )),
            (TagClass::CONTEXT, 8) => Ok(CasubProt::certReplaceReq(_decode_CertReplaceReq(&el)?)),
            (TagClass::CONTEXT, 9) => Ok(CasubProt::certReplaceRsp(_decode_CertReplaceRsp(&el)?)),
            (TagClass::CONTEXT, 10) => Ok(CasubProt::certUpdateReq(_decode_CertUpdateReq(&el)?)),
            (TagClass::CONTEXT, 11) => Ok(CasubProt::certUpdateRsp(_decode_CertUpdateRsp(&el)?)),
            (TagClass::CONTEXT, 12) => {
                Ok(CasubProt::cAsubscribeAbort(_decode_CAsubscribeAbort(&el)?))
            }
            _ => Ok(CasubProt::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CasubProt(value_: &CasubProt) -> ASN1Result<X690Element> {
    |value: &CasubProt| -> ASN1Result<X690Element> {
        match value {
            CasubProt::initReq(v) => |v_1: &InitializationRec| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InitializationRec(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            CasubProt::initAcc(v) => |v_1: &InitializationAcc| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InitializationAcc(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            CasubProt::initRej(v) => |v_1: &InitializationRej| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InitializationRej(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v),
            CasubProt::initAbt(v) => |v_1: &InitializationAbort| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InitializationAbort(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v),
            CasubProt::certSubscribeReq(v) => |v_1: &CertSubscribeReq| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertSubscribeReq(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 4;
                Ok(el_1)
            }(&v),
            CasubProt::certSubscribeRsp(v) => |v_1: &CertSubscribeRsp| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertSubscribeRsp(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 5;
                Ok(el_1)
            }(&v),
            CasubProt::certUnsubscribeReq(v) => {
                |v_1: &CertUnsubscribeReq| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertUnsubscribeReq(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 6;
                    Ok(el_1)
                }(&v)
            }
            CasubProt::certUnsubscribeRsp(v) => {
                |v_1: &CertUnsubscribeRsp| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertUnsubscribeRsp(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 7;
                    Ok(el_1)
                }(&v)
            }
            CasubProt::certReplaceReq(v) => |v_1: &CertReplaceReq| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertReplaceReq(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 8;
                Ok(el_1)
            }(&v),
            CasubProt::certReplaceRsp(v) => |v_1: &CertReplaceRsp| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertReplaceRsp(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 9;
                Ok(el_1)
            }(&v),
            CasubProt::certUpdateReq(v) => |v_1: &CertUpdateReq| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertUpdateReq(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 10;
                Ok(el_1)
            }(&v),
            CasubProt::certUpdateRsp(v) => |v_1: &CertUpdateRsp| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertUpdateRsp(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 11;
                Ok(el_1)
            }(&v),
            CasubProt::cAsubscribeAbort(v) => |v_1: &CAsubscribeAbort| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CAsubscribeAbort(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 12;
                Ok(el_1)
            }(&v),
            CasubProt::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InitializationRec ::= SEQUENCE {
///   version    Version,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct InitializationRec {
    pub version: Version,
    pub _unrecognized: Vec<X690Element>,
}
impl InitializationRec {
    pub fn new(version: Version, _unrecognized: Vec<X690Element>) -> Self {
        InitializationRec {
            version,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for InitializationRec {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_InitializationRec(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for InitializationRec {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_InitializationRec(el)
    }
}

pub const _rctl1_components_for_InitializationRec: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "version",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 3)),
    None,
    None,
)];

pub const _rctl2_components_for_InitializationRec: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_InitializationRec: &[ComponentSpec; 0] = &[];

pub fn _decode_InitializationRec(el: &X690Element) -> ASN1Result<InitializationRec> {
    |el_: &X690Element| -> ASN1Result<InitializationRec> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_InitializationRec,
            _eal_components_for_InitializationRec,
            _rctl2_components_for_InitializationRec,
        )?;
        let version = _decode_Version(_components.get("version").unwrap())?;
        Ok(InitializationRec {
            version,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_InitializationRec(value_: &InitializationRec) -> ASN1Result<X690Element> {
    |value_: &InitializationRec| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_Version(&value_.version)?);
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
/// InitializationAcc ::= SEQUENCE {
///   version    Version,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct InitializationAcc {
    pub version: Version,
    pub _unrecognized: Vec<X690Element>,
}
impl InitializationAcc {
    pub fn new(version: Version, _unrecognized: Vec<X690Element>) -> Self {
        InitializationAcc {
            version,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for InitializationAcc {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_InitializationAcc(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for InitializationAcc {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_InitializationAcc(el)
    }
}

pub const _rctl1_components_for_InitializationAcc: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "version",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 3)),
    None,
    None,
)];

pub const _rctl2_components_for_InitializationAcc: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_InitializationAcc: &[ComponentSpec; 0] = &[];

pub fn _decode_InitializationAcc(el: &X690Element) -> ASN1Result<InitializationAcc> {
    |el_: &X690Element| -> ASN1Result<InitializationAcc> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_InitializationAcc,
            _eal_components_for_InitializationAcc,
            _rctl2_components_for_InitializationAcc,
        )?;
        let version = _decode_Version(_components.get("version").unwrap())?;
        Ok(InitializationAcc {
            version,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_InitializationAcc(value_: &InitializationAcc) -> ASN1Result<X690Element> {
    |value_: &InitializationAcc| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_Version(&value_.version)?);
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
/// InitializationRej ::= SEQUENCE {
///   diag        ENUMERATED {
///     unsupportedVersion     (0),
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct InitializationRej {
    pub diag: InitializationRej_diag,
    pub _unrecognized: Vec<X690Element>,
}
impl InitializationRej {
    pub fn new(diag: InitializationRej_diag, _unrecognized: Vec<X690Element>) -> Self {
        InitializationRej {
            diag,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for InitializationRej {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_InitializationRej(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for InitializationRej {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_InitializationRej(el)
    }
}

pub const _rctl1_components_for_InitializationRej: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "diag",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 10)),
    None,
    None,
)];

pub const _rctl2_components_for_InitializationRej: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_InitializationRej: &[ComponentSpec; 0] = &[];

pub fn _decode_InitializationRej(el: &X690Element) -> ASN1Result<InitializationRej> {
    |el_: &X690Element| -> ASN1Result<InitializationRej> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_InitializationRej,
            _eal_components_for_InitializationRej,
            _rctl2_components_for_InitializationRej,
        )?;
        let diag = _decode_InitializationRej_diag(_components.get("diag").unwrap())?;
        Ok(InitializationRej {
            diag,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_InitializationRej(value_: &InitializationRej) -> ASN1Result<X690Element> {
    |value_: &InitializationRej| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_InitializationRej_diag(&value_.diag)?);
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
/// InitializationAbort ::= SEQUENCE {
///   diag        ENUMERATED {
///     unsupportedVersion       (0),
///     onlySingleVersionAllowed (1),
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct InitializationAbort {
    pub diag: InitializationAbort_diag,
    pub _unrecognized: Vec<X690Element>,
}
impl InitializationAbort {
    pub fn new(diag: InitializationAbort_diag, _unrecognized: Vec<X690Element>) -> Self {
        InitializationAbort {
            diag,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for InitializationAbort {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_InitializationAbort(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for InitializationAbort {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_InitializationAbort(el)
    }
}

pub const _rctl1_components_for_InitializationAbort: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "diag",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 10)),
    None,
    None,
)];

pub const _rctl2_components_for_InitializationAbort: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_InitializationAbort: &[ComponentSpec; 0] = &[];

pub fn _decode_InitializationAbort(el: &X690Element) -> ASN1Result<InitializationAbort> {
    |el_: &X690Element| -> ASN1Result<InitializationAbort> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_InitializationAbort,
            _eal_components_for_InitializationAbort,
            _rctl2_components_for_InitializationAbort,
        )?;
        let diag = _decode_InitializationAbort_diag(_components.get("diag").unwrap())?;
        Ok(InitializationAbort {
            diag,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_InitializationAbort(value_: &InitializationAbort) -> ASN1Result<X690Element> {
    |value_: &InitializationAbort| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_InitializationAbort_diag(&value_.diag)?);
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
/// CertSubscribeReq ::= SEQUENCE {
///   invokeID     InvokeID,
///   certs        SEQUENCE (SIZE (1..MAX)) OF SEQUENCE {
///     subject      Name,
///     serialNumber CertificateSerialNumber,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertSubscribeReq {
    pub invokeID: InvokeID,
    pub certs: Vec<CertSubscribeReq_certs_Item>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertSubscribeReq {
    pub fn new(
        invokeID: InvokeID,
        certs: Vec<CertSubscribeReq_certs_Item>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertSubscribeReq {
            invokeID,
            certs,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertSubscribeReq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeReq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertSubscribeReq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeReq(el)
    }
}

pub const _rctl1_components_for_CertSubscribeReq: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certs",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertSubscribeReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertSubscribeReq: &[ComponentSpec; 0] = &[];

pub fn _decode_CertSubscribeReq(el: &X690Element) -> ASN1Result<CertSubscribeReq> {
    |el_: &X690Element| -> ASN1Result<CertSubscribeReq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertSubscribeReq,
            _eal_components_for_CertSubscribeReq,
            _rctl2_components_for_CertSubscribeReq,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let certs = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertSubscribeReq_certs_Item>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<CertSubscribeReq_certs_Item> =
                Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_CertSubscribeReq_certs_Item(el)?);
            }
            Ok(items)
        }(_components.get("certs").unwrap())?;
        Ok(CertSubscribeReq {
            invokeID,
            certs,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertSubscribeReq(value_: &CertSubscribeReq) -> ASN1Result<X690Element> {
    |value_: &CertSubscribeReq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(
            |value_: &SEQUENCE_OF<CertSubscribeReq_certs_Item>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_CertSubscribeReq_certs_Item(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.certs)?,
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
/// InvokeID  ::=  INTEGER (0..127)
/// ```
pub type InvokeID = INTEGER;

pub fn _decode_InvokeID(el: &X690Element) -> ASN1Result<InvokeID> {
    ber_decode_integer(&el)
}

pub fn _encode_InvokeID(value_: &InvokeID) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertSubscribeRsp ::= SEQUENCE {
///   invokeID     InvokeID,
///   result       CHOICE {
///     success       [0]  CertSubscribeOK,
///     failure       [1]  CertSubscribeErr,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertSubscribeRsp {
    pub invokeID: InvokeID,
    pub result: CertSubscribeRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl CertSubscribeRsp {
    pub fn new(
        invokeID: InvokeID,
        result: CertSubscribeRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertSubscribeRsp {
            invokeID,
            result,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertSubscribeRsp {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeRsp(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertSubscribeRsp {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeRsp(el)
    }
}

pub const _rctl1_components_for_CertSubscribeRsp: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("result", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_CertSubscribeRsp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertSubscribeRsp: &[ComponentSpec; 0] = &[];

pub fn _decode_CertSubscribeRsp(el: &X690Element) -> ASN1Result<CertSubscribeRsp> {
    |el_: &X690Element| -> ASN1Result<CertSubscribeRsp> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertSubscribeRsp,
            _eal_components_for_CertSubscribeRsp,
            _rctl2_components_for_CertSubscribeRsp,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let result = _decode_CertSubscribeRsp_result(_components.get("result").unwrap())?;
        Ok(CertSubscribeRsp {
            invokeID,
            result,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertSubscribeRsp(value_: &CertSubscribeRsp) -> ASN1Result<X690Element> {
    |value_: &CertSubscribeRsp| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(_encode_CertSubscribeRsp_result(&value_.result)?);
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
/// CertSubscribeOK  ::=  SEQUENCE (SIZE (1..MAX)) OF CHOICE {
///   ok       [0] SEQUENCE {
///     cert         Certificate,
///     status       CertStatus,
///     revokeReason CRLReason OPTIONAL,
///     ... },
///   not-ok   [1] SEQUENCE {
///     status       CASP-CertStatusCode,
///     ... },
///   ... }
/// ```
pub type CertSubscribeOK = Vec<CertSubscribeOK_Item>; // SequenceOfType

pub fn _decode_CertSubscribeOK(el: &X690Element) -> ASN1Result<CertSubscribeOK> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertSubscribeOK_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<CertSubscribeOK_Item> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_CertSubscribeOK_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CertSubscribeOK(value_: &CertSubscribeOK) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<CertSubscribeOK_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_CertSubscribeOK_Item(&v)?);
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
/// CertStatus  ::=  ENUMERATED {
///   good    (0),
///   revoked (1),
///   on-hold (2),
///   expired (3),
///   ... }
/// ```
pub type CertStatus = ENUMERATED;

pub const CertStatus_good: CertStatus = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CertStatus_revoked: CertStatus = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CertStatus_on_hold: CertStatus = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CertStatus_expired: CertStatus = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_CertStatus(el: &X690Element) -> ASN1Result<CertStatus> {
    ber_decode_enumerated(&el)
}

pub fn _encode_CertStatus(value_: &CertStatus) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CASP-CertStatusCode  ::=  ENUMERATED {
///   noReason       (1),
///   unknownCert    (2),
///   ... }
/// ```
pub type CASP_CertStatusCode = ENUMERATED;

pub const CASP_CertStatusCode_noReason: CASP_CertStatusCode = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CASP_CertStatusCode_unknownCert: CASP_CertStatusCode = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_CASP_CertStatusCode(el: &X690Element) -> ASN1Result<CASP_CertStatusCode> {
    ber_decode_enumerated(&el)
}

pub fn _encode_CASP_CertStatusCode(value_: &CASP_CertStatusCode) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertSubscribeErr ::= SEQUENCE {
///   code       CASP-error,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertSubscribeErr {
    pub code: CASP_error,
    pub _unrecognized: Vec<X690Element>,
}
impl CertSubscribeErr {
    pub fn new(code: CASP_error, _unrecognized: Vec<X690Element>) -> Self {
        CertSubscribeErr {
            code,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertSubscribeErr {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeErr(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertSubscribeErr {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeErr(el)
    }
}

pub const _rctl1_components_for_CertSubscribeErr: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "code",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 10)),
    None,
    None,
)];

pub const _rctl2_components_for_CertSubscribeErr: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertSubscribeErr: &[ComponentSpec; 0] = &[];

pub fn _decode_CertSubscribeErr(el: &X690Element) -> ASN1Result<CertSubscribeErr> {
    |el_: &X690Element| -> ASN1Result<CertSubscribeErr> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertSubscribeErr,
            _eal_components_for_CertSubscribeErr,
            _rctl2_components_for_CertSubscribeErr,
        )?;
        let code = _decode_CASP_error(_components.get("code").unwrap())?;
        Ok(CertSubscribeErr {
            code,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertSubscribeErr(value_: &CertSubscribeErr) -> ASN1Result<X690Element> {
    |value_: &CertSubscribeErr| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_CASP_error(&value_.code)?);
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
/// CertUnsubscribeReq ::= SEQUENCE {
///   invokeID     InvokeID,
///   certs  SEQUENCE (SIZE (1..MAX)) OF SEQUENCE {
///     subject      Name,
///     serialNumber CertificateSerialNumber,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUnsubscribeReq {
    pub invokeID: InvokeID,
    pub certs: Vec<CertUnsubscribeReq_certs_Item>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUnsubscribeReq {
    pub fn new(
        invokeID: InvokeID,
        certs: Vec<CertUnsubscribeReq_certs_Item>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUnsubscribeReq {
            invokeID,
            certs,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUnsubscribeReq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeReq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUnsubscribeReq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeReq(el)
    }
}

pub const _rctl1_components_for_CertUnsubscribeReq: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certs",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertUnsubscribeReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUnsubscribeReq: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUnsubscribeReq(el: &X690Element) -> ASN1Result<CertUnsubscribeReq> {
    |el_: &X690Element| -> ASN1Result<CertUnsubscribeReq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUnsubscribeReq,
            _eal_components_for_CertUnsubscribeReq,
            _rctl2_components_for_CertUnsubscribeReq,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let certs = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertUnsubscribeReq_certs_Item>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<CertUnsubscribeReq_certs_Item> =
                Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_CertUnsubscribeReq_certs_Item(el)?);
            }
            Ok(items)
        }(_components.get("certs").unwrap())?;
        Ok(CertUnsubscribeReq {
            invokeID,
            certs,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUnsubscribeReq(value_: &CertUnsubscribeReq) -> ASN1Result<X690Element> {
    |value_: &CertUnsubscribeReq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(
            |value_: &SEQUENCE_OF<CertUnsubscribeReq_certs_Item>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_CertUnsubscribeReq_certs_Item(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.certs)?,
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
/// CertUnsubscribeRsp ::= SEQUENCE {
///   invokeID     InvokeID,
///   result       CHOICE {
///     success       [0]  CertUnsubscribeOK,
///     failure       [1]  CertUnsubscribeErr,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUnsubscribeRsp {
    pub invokeID: InvokeID,
    pub result: CertUnsubscribeRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUnsubscribeRsp {
    pub fn new(
        invokeID: InvokeID,
        result: CertUnsubscribeRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUnsubscribeRsp {
            invokeID,
            result,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUnsubscribeRsp {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeRsp(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUnsubscribeRsp {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeRsp(el)
    }
}

pub const _rctl1_components_for_CertUnsubscribeRsp: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("result", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_CertUnsubscribeRsp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUnsubscribeRsp: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUnsubscribeRsp(el: &X690Element) -> ASN1Result<CertUnsubscribeRsp> {
    |el_: &X690Element| -> ASN1Result<CertUnsubscribeRsp> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUnsubscribeRsp,
            _eal_components_for_CertUnsubscribeRsp,
            _rctl2_components_for_CertUnsubscribeRsp,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let result = _decode_CertUnsubscribeRsp_result(_components.get("result").unwrap())?;
        Ok(CertUnsubscribeRsp {
            invokeID,
            result,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUnsubscribeRsp(value_: &CertUnsubscribeRsp) -> ASN1Result<X690Element> {
    |value_: &CertUnsubscribeRsp| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(_encode_CertUnsubscribeRsp_result(&value_.result)?);
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
/// CertUnsubscribeOK  ::=  SEQUENCE (SIZE (1..MAX)) OF CHOICE {
///   ok       [0] SEQUENCE {
///     subject      Name,
///     serialNumber CertificateSerialNumber,
///     ... },
///   not-ok   [1] SEQUENCE {
///     status       CASP-CertStatusCode,
///     ... },
///   ... }
/// ```
pub type CertUnsubscribeOK = Vec<CertUnsubscribeOK_Item>; // SequenceOfType

pub fn _decode_CertUnsubscribeOK(el: &X690Element) -> ASN1Result<CertUnsubscribeOK> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertUnsubscribeOK_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<CertUnsubscribeOK_Item> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_CertUnsubscribeOK_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CertUnsubscribeOK(value_: &CertUnsubscribeOK) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<CertUnsubscribeOK_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_CertUnsubscribeOK_Item(&v)?);
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
/// CertUnsubscribeErr ::= SEQUENCE {
///   code         CASP-error,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUnsubscribeErr {
    pub code: CASP_error,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUnsubscribeErr {
    pub fn new(code: CASP_error, _unrecognized: Vec<X690Element>) -> Self {
        CertUnsubscribeErr {
            code,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUnsubscribeErr {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeErr(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUnsubscribeErr {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeErr(el)
    }
}

pub const _rctl1_components_for_CertUnsubscribeErr: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "code",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 10)),
    None,
    None,
)];

pub const _rctl2_components_for_CertUnsubscribeErr: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUnsubscribeErr: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUnsubscribeErr(el: &X690Element) -> ASN1Result<CertUnsubscribeErr> {
    |el_: &X690Element| -> ASN1Result<CertUnsubscribeErr> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUnsubscribeErr,
            _eal_components_for_CertUnsubscribeErr,
            _rctl2_components_for_CertUnsubscribeErr,
        )?;
        let code = _decode_CASP_error(_components.get("code").unwrap())?;
        Ok(CertUnsubscribeErr {
            code,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUnsubscribeErr(value_: &CertUnsubscribeErr) -> ASN1Result<X690Element> {
    |value_: &CertUnsubscribeErr| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_CASP_error(&value_.code)?);
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
/// CertReplaceReq ::= SEQUENCE {
///   invokeID     InvokeID,
///   certs         SEQUENCE (SIZE (1..MAX)) OF SEQUENCE {
///     old           CertificateSerialNumber,
///     new           Certificate,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertReplaceReq {
    pub invokeID: InvokeID,
    pub certs: Vec<CertReplaceReq_certs_Item>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertReplaceReq {
    pub fn new(
        invokeID: InvokeID,
        certs: Vec<CertReplaceReq_certs_Item>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertReplaceReq {
            invokeID,
            certs,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertReplaceReq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceReq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertReplaceReq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceReq(el)
    }
}

pub const _rctl1_components_for_CertReplaceReq: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certs",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertReplaceReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertReplaceReq: &[ComponentSpec; 0] = &[];

pub fn _decode_CertReplaceReq(el: &X690Element) -> ASN1Result<CertReplaceReq> {
    |el_: &X690Element| -> ASN1Result<CertReplaceReq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertReplaceReq,
            _eal_components_for_CertReplaceReq,
            _rctl2_components_for_CertReplaceReq,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let certs = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertReplaceReq_certs_Item>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<CertReplaceReq_certs_Item> =
                Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_CertReplaceReq_certs_Item(el)?);
            }
            Ok(items)
        }(_components.get("certs").unwrap())?;
        Ok(CertReplaceReq {
            invokeID,
            certs,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertReplaceReq(value_: &CertReplaceReq) -> ASN1Result<X690Element> {
    |value_: &CertReplaceReq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(
            |value_: &SEQUENCE_OF<CertReplaceReq_certs_Item>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_CertReplaceReq_certs_Item(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.certs)?,
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
/// CertReplaceRsp ::= SEQUENCE {
///   invokeID     InvokeID,
///   result        CHOICE {
///     success       [0]  CertReplaceOK,
///     failure       [1]  CertReplaceErr,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertReplaceRsp {
    pub invokeID: InvokeID,
    pub result: CertReplaceRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl CertReplaceRsp {
    pub fn new(
        invokeID: InvokeID,
        result: CertReplaceRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertReplaceRsp {
            invokeID,
            result,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertReplaceRsp {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceRsp(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertReplaceRsp {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceRsp(el)
    }
}

pub const _rctl1_components_for_CertReplaceRsp: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("result", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_CertReplaceRsp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertReplaceRsp: &[ComponentSpec; 0] = &[];

pub fn _decode_CertReplaceRsp(el: &X690Element) -> ASN1Result<CertReplaceRsp> {
    |el_: &X690Element| -> ASN1Result<CertReplaceRsp> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertReplaceRsp,
            _eal_components_for_CertReplaceRsp,
            _rctl2_components_for_CertReplaceRsp,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let result = _decode_CertReplaceRsp_result(_components.get("result").unwrap())?;
        Ok(CertReplaceRsp {
            invokeID,
            result,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertReplaceRsp(value_: &CertReplaceRsp) -> ASN1Result<X690Element> {
    |value_: &CertReplaceRsp| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(_encode_CertReplaceRsp_result(&value_.result)?);
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
/// CertReplaceOK  ::=  SEQUENCE (SIZE (1..MAX)) OF CHOICE {
///   ok        [0] SEQUENCE {
///     issuer        Name,
///     serialNumber  CertificateSerialNumber,
///     ... },
///   not-ok    [1] SEQUENCE {
///     status        CASP-CertStatusCode,
///     ... },
///   ... }
/// ```
pub type CertReplaceOK = Vec<CertReplaceOK_Item>; // SequenceOfType

pub fn _decode_CertReplaceOK(el: &X690Element) -> ASN1Result<CertReplaceOK> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertReplaceOK_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<CertReplaceOK_Item> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_CertReplaceOK_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CertReplaceOK(value_: &CertReplaceOK) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<CertReplaceOK_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_CertReplaceOK_Item(&v)?);
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
/// CertReplaceErr ::= SEQUENCE {
///   code          CASP-error,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertReplaceErr {
    pub code: CASP_error,
    pub _unrecognized: Vec<X690Element>,
}
impl CertReplaceErr {
    pub fn new(code: CASP_error, _unrecognized: Vec<X690Element>) -> Self {
        CertReplaceErr {
            code,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertReplaceErr {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceErr(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertReplaceErr {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceErr(el)
    }
}

pub const _rctl1_components_for_CertReplaceErr: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "code",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 10)),
    None,
    None,
)];

pub const _rctl2_components_for_CertReplaceErr: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertReplaceErr: &[ComponentSpec; 0] = &[];

pub fn _decode_CertReplaceErr(el: &X690Element) -> ASN1Result<CertReplaceErr> {
    |el_: &X690Element| -> ASN1Result<CertReplaceErr> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertReplaceErr,
            _eal_components_for_CertReplaceErr,
            _rctl2_components_for_CertReplaceErr,
        )?;
        let code = _decode_CASP_error(_components.get("code").unwrap())?;
        Ok(CertReplaceErr {
            code,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertReplaceErr(value_: &CertReplaceErr) -> ASN1Result<X690Element> {
    |value_: &CertReplaceErr| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_CASP_error(&value_.code)?);
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
/// CertUpdateReq ::= SEQUENCE {
///   invokeID     InvokeID,
///   certs  SEQUENCE (SIZE (1..MAX)) OF SEQUENCE {
///     subject      Name,
///     serialNumber CertificateSerialNumber,
///     certStatus   CertStatus,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUpdateReq {
    pub invokeID: InvokeID,
    pub certs: Vec<CertUpdateReq_certs_Item>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUpdateReq {
    pub fn new(
        invokeID: InvokeID,
        certs: Vec<CertUpdateReq_certs_Item>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUpdateReq {
            invokeID,
            certs,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUpdateReq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateReq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUpdateReq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateReq(el)
    }
}

pub const _rctl1_components_for_CertUpdateReq: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certs",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertUpdateReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUpdateReq: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUpdateReq(el: &X690Element) -> ASN1Result<CertUpdateReq> {
    |el_: &X690Element| -> ASN1Result<CertUpdateReq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUpdateReq,
            _eal_components_for_CertUpdateReq,
            _rctl2_components_for_CertUpdateReq,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let certs = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertUpdateReq_certs_Item>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<CertUpdateReq_certs_Item> =
                Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_CertUpdateReq_certs_Item(el)?);
            }
            Ok(items)
        }(_components.get("certs").unwrap())?;
        Ok(CertUpdateReq {
            invokeID,
            certs,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUpdateReq(value_: &CertUpdateReq) -> ASN1Result<X690Element> {
    |value_: &CertUpdateReq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(
            |value_: &SEQUENCE_OF<CertUpdateReq_certs_Item>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_CertUpdateReq_certs_Item(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.certs)?,
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
/// CertUpdateRsp ::= SEQUENCE {
///   invokeID      InvokeID,
///   result        CHOICE {
///     success       [0]  CertUpdateOK,
///     failure       [1]  CertUpdateErr,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUpdateRsp {
    pub invokeID: InvokeID,
    pub result: CertUpdateRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUpdateRsp {
    pub fn new(
        invokeID: InvokeID,
        result: CertUpdateRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUpdateRsp {
            invokeID,
            result,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUpdateRsp {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateRsp(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUpdateRsp {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateRsp(el)
    }
}

pub const _rctl1_components_for_CertUpdateRsp: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("result", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_CertUpdateRsp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUpdateRsp: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUpdateRsp(el: &X690Element) -> ASN1Result<CertUpdateRsp> {
    |el_: &X690Element| -> ASN1Result<CertUpdateRsp> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUpdateRsp,
            _eal_components_for_CertUpdateRsp,
            _rctl2_components_for_CertUpdateRsp,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let result = _decode_CertUpdateRsp_result(_components.get("result").unwrap())?;
        Ok(CertUpdateRsp {
            invokeID,
            result,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUpdateRsp(value_: &CertUpdateRsp) -> ASN1Result<X690Element> {
    |value_: &CertUpdateRsp| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(_encode_CertUpdateRsp_result(&value_.result)?);
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
/// CertUpdateOK  ::=  SEQUENCE (SIZE (1..MAX)) OF CHOICE {
///   ok        [0] SEQUENCE {
///     subject       Name,
///     serialNumber  CertificateSerialNumber,
///     ... },
///   not-ok    [1] SEQUENCE {
///     status        CASP-CertStatusCode,
///     ... },
///   ... }
/// ```
pub type CertUpdateOK = Vec<CertUpdateOK_Item>; // SequenceOfType

pub fn _decode_CertUpdateOK(el: &X690Element) -> ASN1Result<CertUpdateOK> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertUpdateOK_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<CertUpdateOK_Item> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_CertUpdateOK_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CertUpdateOK(value_: &CertUpdateOK) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<CertUpdateOK_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_CertUpdateOK_Item(&v)?);
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
/// CertUpdateErr ::= SEQUENCE {
///   code          CASP-error,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUpdateErr {
    pub code: CASP_error,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUpdateErr {
    pub fn new(code: CASP_error, _unrecognized: Vec<X690Element>) -> Self {
        CertUpdateErr {
            code,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUpdateErr {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateErr(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUpdateErr {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateErr(el)
    }
}

pub const _rctl1_components_for_CertUpdateErr: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "code",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 10)),
    None,
    None,
)];

pub const _rctl2_components_for_CertUpdateErr: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUpdateErr: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUpdateErr(el: &X690Element) -> ASN1Result<CertUpdateErr> {
    |el_: &X690Element| -> ASN1Result<CertUpdateErr> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUpdateErr,
            _eal_components_for_CertUpdateErr,
            _rctl2_components_for_CertUpdateErr,
        )?;
        let code = _decode_CASP_error(_components.get("code").unwrap())?;
        Ok(CertUpdateErr {
            code,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUpdateErr(value_: &CertUpdateErr) -> ASN1Result<X690Element> {
    |value_: &CertUpdateErr| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_CASP_error(&value_.code)?);
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
/// CAsubscribeAbort ::= SEQUENCE {
///   invokeID     InvokeID,
///   reason       CASP-error,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CAsubscribeAbort {
    pub invokeID: InvokeID,
    pub reason: CASP_error,
    pub _unrecognized: Vec<X690Element>,
}
impl CAsubscribeAbort {
    pub fn new(invokeID: InvokeID, reason: CASP_error, _unrecognized: Vec<X690Element>) -> Self {
        CAsubscribeAbort {
            invokeID,
            reason,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CAsubscribeAbort {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CAsubscribeAbort(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CAsubscribeAbort {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CAsubscribeAbort(el)
    }
}

pub const _rctl1_components_for_CAsubscribeAbort: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "reason",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CAsubscribeAbort: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CAsubscribeAbort: &[ComponentSpec; 0] = &[];

pub fn _decode_CAsubscribeAbort(el: &X690Element) -> ASN1Result<CAsubscribeAbort> {
    |el_: &X690Element| -> ASN1Result<CAsubscribeAbort> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CAsubscribeAbort,
            _eal_components_for_CAsubscribeAbort,
            _rctl2_components_for_CAsubscribeAbort,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let reason = _decode_CASP_error(_components.get("reason").unwrap())?;
        Ok(CAsubscribeAbort {
            invokeID,
            reason,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CAsubscribeAbort(value_: &CAsubscribeAbort) -> ASN1Result<X690Element> {
    |value_: &CAsubscribeAbort| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(_encode_CASP_error(&value_.reason)?);
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
/// CASP-error  ::=  ENUMERATED {
///   noReason                      (0),
///   unknownContentType            (1),
///   unsupportedWLMPversion        (2),
///   missingContent                (3),
///   missingContentComponent       (4),
///   invalidContentComponent       (5),
///   sequenceError                 (6),
///   unknownSubject                (7),
///   unknownCert                   (8),
///   ... }
/// ```
pub type CASP_error = ENUMERATED;

pub const CASP_error_noReason: CASP_error = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CASP_error_unknownContentType: CASP_error = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CASP_error_unsupportedWLMPversion: CASP_error = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CASP_error_missingContent: CASP_error = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CASP_error_missingContentComponent: CASP_error = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CASP_error_invalidContentComponent: CASP_error = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CASP_error_sequenceError: CASP_error = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CASP_error_unknownSubject: CASP_error = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CASP_error_unknownCert: CASP_error = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_CASP_error(el: &X690Element) -> ASN1Result<CASP_error> {
    ber_decode_enumerated(&el)
}

pub fn _encode_CASP_error(value_: &CASP_error) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InitializationRej-diag ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type InitializationRej_diag = ENUMERATED;

pub const InitializationRej_diag_unsupportedVersion: InitializationRej_diag = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_InitializationRej_diag(el: &X690Element) -> ASN1Result<InitializationRej_diag> {
    ber_decode_enumerated(&el)
}

pub fn _encode_InitializationRej_diag(value_: &InitializationRej_diag) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InitializationAbort-diag ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type InitializationAbort_diag = ENUMERATED;

pub const InitializationAbort_diag_unsupportedVersion: InitializationAbort_diag = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const InitializationAbort_diag_onlySingleVersionAllowed: InitializationAbort_diag = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_InitializationAbort_diag(el: &X690Element) -> ASN1Result<InitializationAbort_diag> {
    ber_decode_enumerated(&el)
}

pub fn _encode_InitializationAbort_diag(
    value_: &InitializationAbort_diag,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertSubscribeReq-certs-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertSubscribeReq_certs_Item {
    pub subject: Name,
    pub serialNumber: CertificateSerialNumber,
    pub _unrecognized: Vec<X690Element>,
}
impl CertSubscribeReq_certs_Item {
    pub fn new(
        subject: Name,
        serialNumber: CertificateSerialNumber,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertSubscribeReq_certs_Item {
            subject,
            serialNumber,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertSubscribeReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeReq_certs_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertSubscribeReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeReq_certs_Item(el)
    }
}

pub const _rctl1_components_for_CertSubscribeReq_certs_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new("subject", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertSubscribeReq_certs_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertSubscribeReq_certs_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_CertSubscribeReq_certs_Item(
    el: &X690Element,
) -> ASN1Result<CertSubscribeReq_certs_Item> {
    |el_: &X690Element| -> ASN1Result<CertSubscribeReq_certs_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertSubscribeReq_certs_Item,
            _eal_components_for_CertSubscribeReq_certs_Item,
            _rctl2_components_for_CertSubscribeReq_certs_Item,
        )?;
        let subject = _decode_Name(_components.get("subject").unwrap())?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        Ok(CertSubscribeReq_certs_Item {
            subject,
            serialNumber,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertSubscribeReq_certs_Item(
    value_: &CertSubscribeReq_certs_Item,
) -> ASN1Result<X690Element> {
    |value_: &CertSubscribeReq_certs_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Name(&value_.subject)?);
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
/// CertSubscribeRsp-result ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertSubscribeRsp_result {
    success(CertSubscribeOK),
    failure(CertSubscribeErr),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CertSubscribeRsp_result {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeRsp_result(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertSubscribeRsp_result {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeRsp_result(el)
    }
}

pub fn _decode_CertSubscribeRsp_result(el: &X690Element) -> ASN1Result<CertSubscribeRsp_result> {
    |el: &X690Element| -> ASN1Result<CertSubscribeRsp_result> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(CertSubscribeRsp_result::success(
                _decode_CertSubscribeOK(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(CertSubscribeRsp_result::failure(
                _decode_CertSubscribeErr(&el)?,
            )),
            _ => Ok(CertSubscribeRsp_result::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CertSubscribeRsp_result(
    value_: &CertSubscribeRsp_result,
) -> ASN1Result<X690Element> {
    |value: &CertSubscribeRsp_result| -> ASN1Result<X690Element> {
        match value {
            CertSubscribeRsp_result::success(v) => {
                |v_1: &CertSubscribeOK| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertSubscribeOK(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            CertSubscribeRsp_result::failure(v) => {
                |v_1: &CertSubscribeErr| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertSubscribeErr(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            CertSubscribeRsp_result::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertSubscribeOK-Item-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertSubscribeOK_Item_ok {
    pub cert: Certificate,
    pub status: CertStatus,
    pub revokeReason: OPTIONAL<CRLReason>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertSubscribeOK_Item_ok {
    pub fn new(
        cert: Certificate,
        status: CertStatus,
        revokeReason: OPTIONAL<CRLReason>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertSubscribeOK_Item_ok {
            cert,
            status,
            revokeReason,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertSubscribeOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeOK_Item_ok(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertSubscribeOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeOK_Item_ok(el)
    }
}

pub const _rctl1_components_for_CertSubscribeOK_Item_ok: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "cert",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "status",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "revokeReason",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertSubscribeOK_Item_ok: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertSubscribeOK_Item_ok: &[ComponentSpec; 0] = &[];

pub fn _decode_CertSubscribeOK_Item_ok(el: &X690Element) -> ASN1Result<CertSubscribeOK_Item_ok> {
    |el_: &X690Element| -> ASN1Result<CertSubscribeOK_Item_ok> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertSubscribeOK_Item_ok,
            _eal_components_for_CertSubscribeOK_Item_ok,
            _rctl2_components_for_CertSubscribeOK_Item_ok,
        )?;
        let cert = _decode_Certificate(_components.get("cert").unwrap())?;
        let status = _decode_CertStatus(_components.get("status").unwrap())?;
        let revokeReason: OPTIONAL<CRLReason> = match _components.get("revokeReason") {
            Some(c_) => Some(_decode_CRLReason(c_)?),
            _ => None,
        };
        Ok(CertSubscribeOK_Item_ok {
            cert,
            status,
            revokeReason,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertSubscribeOK_Item_ok(
    value_: &CertSubscribeOK_Item_ok,
) -> ASN1Result<X690Element> {
    |value_: &CertSubscribeOK_Item_ok| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_Certificate(&value_.cert)?);
        components_.push(_encode_CertStatus(&value_.status)?);
        if let Some(v_) = &value_.revokeReason {
            components_.push(_encode_CRLReason(&v_)?);
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
/// CertSubscribeOK-Item-not-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertSubscribeOK_Item_not_ok {
    pub status: CASP_CertStatusCode,
    pub _unrecognized: Vec<X690Element>,
}
impl CertSubscribeOK_Item_not_ok {
    pub fn new(status: CASP_CertStatusCode, _unrecognized: Vec<X690Element>) -> Self {
        CertSubscribeOK_Item_not_ok {
            status,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertSubscribeOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeOK_Item_not_ok(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertSubscribeOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeOK_Item_not_ok(el)
    }
}

pub const _rctl1_components_for_CertSubscribeOK_Item_not_ok: &[ComponentSpec; 1] =
    &[ComponentSpec::new(
        "status",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    )];

pub const _rctl2_components_for_CertSubscribeOK_Item_not_ok: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertSubscribeOK_Item_not_ok: &[ComponentSpec; 0] = &[];

pub fn _decode_CertSubscribeOK_Item_not_ok(
    el: &X690Element,
) -> ASN1Result<CertSubscribeOK_Item_not_ok> {
    |el_: &X690Element| -> ASN1Result<CertSubscribeOK_Item_not_ok> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertSubscribeOK_Item_not_ok,
            _eal_components_for_CertSubscribeOK_Item_not_ok,
            _rctl2_components_for_CertSubscribeOK_Item_not_ok,
        )?;
        let status = _decode_CASP_CertStatusCode(_components.get("status").unwrap())?;
        Ok(CertSubscribeOK_Item_not_ok {
            status,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertSubscribeOK_Item_not_ok(
    value_: &CertSubscribeOK_Item_not_ok,
) -> ASN1Result<X690Element> {
    |value_: &CertSubscribeOK_Item_not_ok| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_CASP_CertStatusCode(&value_.status)?);
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
/// CertSubscribeOK-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertSubscribeOK_Item {
    ok(CertSubscribeOK_Item_ok),
    not_ok(CertSubscribeOK_Item_not_ok),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CertSubscribeOK_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeOK_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertSubscribeOK_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeOK_Item(el)
    }
}

pub fn _decode_CertSubscribeOK_Item(el: &X690Element) -> ASN1Result<CertSubscribeOK_Item> {
    |el: &X690Element| -> ASN1Result<CertSubscribeOK_Item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(CertSubscribeOK_Item::ok(
                _decode_CertSubscribeOK_Item_ok(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(CertSubscribeOK_Item::not_ok(
                _decode_CertSubscribeOK_Item_not_ok(&el)?,
            )),
            _ => Ok(CertSubscribeOK_Item::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CertSubscribeOK_Item(value_: &CertSubscribeOK_Item) -> ASN1Result<X690Element> {
    |value: &CertSubscribeOK_Item| -> ASN1Result<X690Element> {
        match value {
            CertSubscribeOK_Item::ok(v) => {
                |v_1: &CertSubscribeOK_Item_ok| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertSubscribeOK_Item_ok(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            CertSubscribeOK_Item::not_ok(v) => {
                |v_1: &CertSubscribeOK_Item_not_ok| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertSubscribeOK_Item_not_ok(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            CertSubscribeOK_Item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUnsubscribeReq-certs-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUnsubscribeReq_certs_Item {
    pub subject: Name,
    pub serialNumber: CertificateSerialNumber,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUnsubscribeReq_certs_Item {
    pub fn new(
        subject: Name,
        serialNumber: CertificateSerialNumber,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUnsubscribeReq_certs_Item {
            subject,
            serialNumber,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUnsubscribeReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeReq_certs_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUnsubscribeReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeReq_certs_Item(el)
    }
}

pub const _rctl1_components_for_CertUnsubscribeReq_certs_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new("subject", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertUnsubscribeReq_certs_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUnsubscribeReq_certs_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUnsubscribeReq_certs_Item(
    el: &X690Element,
) -> ASN1Result<CertUnsubscribeReq_certs_Item> {
    |el_: &X690Element| -> ASN1Result<CertUnsubscribeReq_certs_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUnsubscribeReq_certs_Item,
            _eal_components_for_CertUnsubscribeReq_certs_Item,
            _rctl2_components_for_CertUnsubscribeReq_certs_Item,
        )?;
        let subject = _decode_Name(_components.get("subject").unwrap())?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        Ok(CertUnsubscribeReq_certs_Item {
            subject,
            serialNumber,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUnsubscribeReq_certs_Item(
    value_: &CertUnsubscribeReq_certs_Item,
) -> ASN1Result<X690Element> {
    |value_: &CertUnsubscribeReq_certs_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Name(&value_.subject)?);
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
/// CertUnsubscribeRsp-result ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertUnsubscribeRsp_result {
    success(CertUnsubscribeOK),
    failure(CertUnsubscribeErr),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CertUnsubscribeRsp_result {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeRsp_result(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUnsubscribeRsp_result {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeRsp_result(el)
    }
}

pub fn _decode_CertUnsubscribeRsp_result(
    el: &X690Element,
) -> ASN1Result<CertUnsubscribeRsp_result> {
    |el: &X690Element| -> ASN1Result<CertUnsubscribeRsp_result> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(CertUnsubscribeRsp_result::success(
                _decode_CertUnsubscribeOK(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(CertUnsubscribeRsp_result::failure(
                _decode_CertUnsubscribeErr(&el)?,
            )),
            _ => Ok(CertUnsubscribeRsp_result::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CertUnsubscribeRsp_result(
    value_: &CertUnsubscribeRsp_result,
) -> ASN1Result<X690Element> {
    |value: &CertUnsubscribeRsp_result| -> ASN1Result<X690Element> {
        match value {
            CertUnsubscribeRsp_result::success(v) => {
                |v_1: &CertUnsubscribeOK| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertUnsubscribeOK(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            CertUnsubscribeRsp_result::failure(v) => {
                |v_1: &CertUnsubscribeErr| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertUnsubscribeErr(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            CertUnsubscribeRsp_result::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUnsubscribeOK-Item-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUnsubscribeOK_Item_ok {
    pub subject: Name,
    pub serialNumber: CertificateSerialNumber,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUnsubscribeOK_Item_ok {
    pub fn new(
        subject: Name,
        serialNumber: CertificateSerialNumber,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUnsubscribeOK_Item_ok {
            subject,
            serialNumber,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUnsubscribeOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeOK_Item_ok(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUnsubscribeOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeOK_Item_ok(el)
    }
}

pub const _rctl1_components_for_CertUnsubscribeOK_Item_ok: &[ComponentSpec; 2] = &[
    ComponentSpec::new("subject", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertUnsubscribeOK_Item_ok: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUnsubscribeOK_Item_ok: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUnsubscribeOK_Item_ok(
    el: &X690Element,
) -> ASN1Result<CertUnsubscribeOK_Item_ok> {
    |el_: &X690Element| -> ASN1Result<CertUnsubscribeOK_Item_ok> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUnsubscribeOK_Item_ok,
            _eal_components_for_CertUnsubscribeOK_Item_ok,
            _rctl2_components_for_CertUnsubscribeOK_Item_ok,
        )?;
        let subject = _decode_Name(_components.get("subject").unwrap())?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        Ok(CertUnsubscribeOK_Item_ok {
            subject,
            serialNumber,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUnsubscribeOK_Item_ok(
    value_: &CertUnsubscribeOK_Item_ok,
) -> ASN1Result<X690Element> {
    |value_: &CertUnsubscribeOK_Item_ok| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Name(&value_.subject)?);
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
/// CertUnsubscribeOK-Item-not-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUnsubscribeOK_Item_not_ok {
    pub status: CASP_CertStatusCode,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUnsubscribeOK_Item_not_ok {
    pub fn new(status: CASP_CertStatusCode, _unrecognized: Vec<X690Element>) -> Self {
        CertUnsubscribeOK_Item_not_ok {
            status,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUnsubscribeOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeOK_Item_not_ok(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUnsubscribeOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeOK_Item_not_ok(el)
    }
}

pub const _rctl1_components_for_CertUnsubscribeOK_Item_not_ok: &[ComponentSpec; 1] =
    &[ComponentSpec::new(
        "status",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    )];

pub const _rctl2_components_for_CertUnsubscribeOK_Item_not_ok: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUnsubscribeOK_Item_not_ok: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUnsubscribeOK_Item_not_ok(
    el: &X690Element,
) -> ASN1Result<CertUnsubscribeOK_Item_not_ok> {
    |el_: &X690Element| -> ASN1Result<CertUnsubscribeOK_Item_not_ok> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUnsubscribeOK_Item_not_ok,
            _eal_components_for_CertUnsubscribeOK_Item_not_ok,
            _rctl2_components_for_CertUnsubscribeOK_Item_not_ok,
        )?;
        let status = _decode_CASP_CertStatusCode(_components.get("status").unwrap())?;
        Ok(CertUnsubscribeOK_Item_not_ok {
            status,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUnsubscribeOK_Item_not_ok(
    value_: &CertUnsubscribeOK_Item_not_ok,
) -> ASN1Result<X690Element> {
    |value_: &CertUnsubscribeOK_Item_not_ok| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_CASP_CertStatusCode(&value_.status)?);
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
/// CertUnsubscribeOK-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertUnsubscribeOK_Item {
    ok(CertUnsubscribeOK_Item_ok),
    not_ok(CertUnsubscribeOK_Item_not_ok),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CertUnsubscribeOK_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeOK_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUnsubscribeOK_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeOK_Item(el)
    }
}

pub fn _decode_CertUnsubscribeOK_Item(el: &X690Element) -> ASN1Result<CertUnsubscribeOK_Item> {
    |el: &X690Element| -> ASN1Result<CertUnsubscribeOK_Item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(CertUnsubscribeOK_Item::ok(
                _decode_CertUnsubscribeOK_Item_ok(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(CertUnsubscribeOK_Item::not_ok(
                _decode_CertUnsubscribeOK_Item_not_ok(&el)?,
            )),
            _ => Ok(CertUnsubscribeOK_Item::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CertUnsubscribeOK_Item(value_: &CertUnsubscribeOK_Item) -> ASN1Result<X690Element> {
    |value: &CertUnsubscribeOK_Item| -> ASN1Result<X690Element> {
        match value {
            CertUnsubscribeOK_Item::ok(v) => {
                |v_1: &CertUnsubscribeOK_Item_ok| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertUnsubscribeOK_Item_ok(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            CertUnsubscribeOK_Item::not_ok(v) => {
                |v_1: &CertUnsubscribeOK_Item_not_ok| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertUnsubscribeOK_Item_not_ok(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            CertUnsubscribeOK_Item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertReplaceReq-certs-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertReplaceReq_certs_Item {
    pub old: CertificateSerialNumber,
    pub new: Certificate,
    pub _unrecognized: Vec<X690Element>,
}
impl CertReplaceReq_certs_Item {
    pub fn new(
        old: CertificateSerialNumber,
        new: Certificate,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertReplaceReq_certs_Item {
            old,
            new,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertReplaceReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceReq_certs_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertReplaceReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceReq_certs_Item(el)
    }
}

pub const _rctl1_components_for_CertReplaceReq_certs_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "old",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "new",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertReplaceReq_certs_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertReplaceReq_certs_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_CertReplaceReq_certs_Item(
    el: &X690Element,
) -> ASN1Result<CertReplaceReq_certs_Item> {
    |el_: &X690Element| -> ASN1Result<CertReplaceReq_certs_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertReplaceReq_certs_Item,
            _eal_components_for_CertReplaceReq_certs_Item,
            _rctl2_components_for_CertReplaceReq_certs_Item,
        )?;
        let old = _decode_CertificateSerialNumber(_components.get("old").unwrap())?;
        let new = _decode_Certificate(_components.get("new").unwrap())?;
        Ok(CertReplaceReq_certs_Item {
            old,
            new,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertReplaceReq_certs_Item(
    value_: &CertReplaceReq_certs_Item,
) -> ASN1Result<X690Element> {
    |value_: &CertReplaceReq_certs_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_CertificateSerialNumber(&value_.old)?);
        components_.push(_encode_Certificate(&value_.new)?);
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
/// CertReplaceRsp-result ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertReplaceRsp_result {
    success(CertReplaceOK),
    failure(CertReplaceErr),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CertReplaceRsp_result {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceRsp_result(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertReplaceRsp_result {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceRsp_result(el)
    }
}

pub fn _decode_CertReplaceRsp_result(el: &X690Element) -> ASN1Result<CertReplaceRsp_result> {
    |el: &X690Element| -> ASN1Result<CertReplaceRsp_result> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => {
                Ok(CertReplaceRsp_result::success(_decode_CertReplaceOK(&el)?))
            }
            (TagClass::CONTEXT, 1) => {
                Ok(CertReplaceRsp_result::failure(_decode_CertReplaceErr(&el)?))
            }
            _ => Ok(CertReplaceRsp_result::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CertReplaceRsp_result(value_: &CertReplaceRsp_result) -> ASN1Result<X690Element> {
    |value: &CertReplaceRsp_result| -> ASN1Result<X690Element> {
        match value {
            CertReplaceRsp_result::success(v) => |v_1: &CertReplaceOK| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertReplaceOK(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            CertReplaceRsp_result::failure(v) => {
                |v_1: &CertReplaceErr| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertReplaceErr(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            CertReplaceRsp_result::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertReplaceOK-Item-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertReplaceOK_Item_ok {
    pub issuer: Name,
    pub serialNumber: CertificateSerialNumber,
    pub _unrecognized: Vec<X690Element>,
}
impl CertReplaceOK_Item_ok {
    pub fn new(
        issuer: Name,
        serialNumber: CertificateSerialNumber,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertReplaceOK_Item_ok {
            issuer,
            serialNumber,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertReplaceOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceOK_Item_ok(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertReplaceOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceOK_Item_ok(el)
    }
}

pub const _rctl1_components_for_CertReplaceOK_Item_ok: &[ComponentSpec; 2] = &[
    ComponentSpec::new("issuer", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertReplaceOK_Item_ok: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertReplaceOK_Item_ok: &[ComponentSpec; 0] = &[];

pub fn _decode_CertReplaceOK_Item_ok(el: &X690Element) -> ASN1Result<CertReplaceOK_Item_ok> {
    |el_: &X690Element| -> ASN1Result<CertReplaceOK_Item_ok> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertReplaceOK_Item_ok,
            _eal_components_for_CertReplaceOK_Item_ok,
            _rctl2_components_for_CertReplaceOK_Item_ok,
        )?;
        let issuer = _decode_Name(_components.get("issuer").unwrap())?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        Ok(CertReplaceOK_Item_ok {
            issuer,
            serialNumber,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertReplaceOK_Item_ok(value_: &CertReplaceOK_Item_ok) -> ASN1Result<X690Element> {
    |value_: &CertReplaceOK_Item_ok| -> ASN1Result<X690Element> {
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
/// CertReplaceOK-Item-not-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertReplaceOK_Item_not_ok {
    pub status: CASP_CertStatusCode,
    pub _unrecognized: Vec<X690Element>,
}
impl CertReplaceOK_Item_not_ok {
    pub fn new(status: CASP_CertStatusCode, _unrecognized: Vec<X690Element>) -> Self {
        CertReplaceOK_Item_not_ok {
            status,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertReplaceOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceOK_Item_not_ok(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertReplaceOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceOK_Item_not_ok(el)
    }
}

pub const _rctl1_components_for_CertReplaceOK_Item_not_ok: &[ComponentSpec; 1] =
    &[ComponentSpec::new(
        "status",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    )];

pub const _rctl2_components_for_CertReplaceOK_Item_not_ok: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertReplaceOK_Item_not_ok: &[ComponentSpec; 0] = &[];

pub fn _decode_CertReplaceOK_Item_not_ok(
    el: &X690Element,
) -> ASN1Result<CertReplaceOK_Item_not_ok> {
    |el_: &X690Element| -> ASN1Result<CertReplaceOK_Item_not_ok> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertReplaceOK_Item_not_ok,
            _eal_components_for_CertReplaceOK_Item_not_ok,
            _rctl2_components_for_CertReplaceOK_Item_not_ok,
        )?;
        let status = _decode_CASP_CertStatusCode(_components.get("status").unwrap())?;
        Ok(CertReplaceOK_Item_not_ok {
            status,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertReplaceOK_Item_not_ok(
    value_: &CertReplaceOK_Item_not_ok,
) -> ASN1Result<X690Element> {
    |value_: &CertReplaceOK_Item_not_ok| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_CASP_CertStatusCode(&value_.status)?);
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
/// CertReplaceOK-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertReplaceOK_Item {
    ok(CertReplaceOK_Item_ok),
    not_ok(CertReplaceOK_Item_not_ok),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CertReplaceOK_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceOK_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertReplaceOK_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceOK_Item(el)
    }
}

pub fn _decode_CertReplaceOK_Item(el: &X690Element) -> ASN1Result<CertReplaceOK_Item> {
    |el: &X690Element| -> ASN1Result<CertReplaceOK_Item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => {
                Ok(CertReplaceOK_Item::ok(_decode_CertReplaceOK_Item_ok(&el)?))
            }
            (TagClass::CONTEXT, 1) => Ok(CertReplaceOK_Item::not_ok(
                _decode_CertReplaceOK_Item_not_ok(&el)?,
            )),
            _ => Ok(CertReplaceOK_Item::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CertReplaceOK_Item(value_: &CertReplaceOK_Item) -> ASN1Result<X690Element> {
    |value: &CertReplaceOK_Item| -> ASN1Result<X690Element> {
        match value {
            CertReplaceOK_Item::ok(v) => |v_1: &CertReplaceOK_Item_ok| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertReplaceOK_Item_ok(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            CertReplaceOK_Item::not_ok(v) => {
                |v_1: &CertReplaceOK_Item_not_ok| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertReplaceOK_Item_not_ok(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            CertReplaceOK_Item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUpdateReq-certs-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUpdateReq_certs_Item {
    pub subject: Name,
    pub serialNumber: CertificateSerialNumber,
    pub certStatus: CertStatus,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUpdateReq_certs_Item {
    pub fn new(
        subject: Name,
        serialNumber: CertificateSerialNumber,
        certStatus: CertStatus,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUpdateReq_certs_Item {
            subject,
            serialNumber,
            certStatus,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUpdateReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateReq_certs_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUpdateReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateReq_certs_Item(el)
    }
}

pub const _rctl1_components_for_CertUpdateReq_certs_Item: &[ComponentSpec; 3] = &[
    ComponentSpec::new("subject", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certStatus",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertUpdateReq_certs_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUpdateReq_certs_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUpdateReq_certs_Item(el: &X690Element) -> ASN1Result<CertUpdateReq_certs_Item> {
    |el_: &X690Element| -> ASN1Result<CertUpdateReq_certs_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUpdateReq_certs_Item,
            _eal_components_for_CertUpdateReq_certs_Item,
            _rctl2_components_for_CertUpdateReq_certs_Item,
        )?;
        let subject = _decode_Name(_components.get("subject").unwrap())?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        let certStatus = _decode_CertStatus(_components.get("certStatus").unwrap())?;
        Ok(CertUpdateReq_certs_Item {
            subject,
            serialNumber,
            certStatus,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUpdateReq_certs_Item(
    value_: &CertUpdateReq_certs_Item,
) -> ASN1Result<X690Element> {
    |value_: &CertUpdateReq_certs_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_Name(&value_.subject)?);
        components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
        components_.push(_encode_CertStatus(&value_.certStatus)?);
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
/// CertUpdateRsp-result ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertUpdateRsp_result {
    success(CertUpdateOK),
    failure(CertUpdateErr),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CertUpdateRsp_result {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateRsp_result(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUpdateRsp_result {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateRsp_result(el)
    }
}

pub fn _decode_CertUpdateRsp_result(el: &X690Element) -> ASN1Result<CertUpdateRsp_result> {
    |el: &X690Element| -> ASN1Result<CertUpdateRsp_result> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(CertUpdateRsp_result::success(_decode_CertUpdateOK(&el)?)),
            (TagClass::CONTEXT, 1) => {
                Ok(CertUpdateRsp_result::failure(_decode_CertUpdateErr(&el)?))
            }
            _ => Ok(CertUpdateRsp_result::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CertUpdateRsp_result(value_: &CertUpdateRsp_result) -> ASN1Result<X690Element> {
    |value: &CertUpdateRsp_result| -> ASN1Result<X690Element> {
        match value {
            CertUpdateRsp_result::success(v) => |v_1: &CertUpdateOK| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertUpdateOK(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            CertUpdateRsp_result::failure(v) => |v_1: &CertUpdateErr| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertUpdateErr(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            CertUpdateRsp_result::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUpdateOK-Item-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUpdateOK_Item_ok {
    pub subject: Name,
    pub serialNumber: CertificateSerialNumber,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUpdateOK_Item_ok {
    pub fn new(
        subject: Name,
        serialNumber: CertificateSerialNumber,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUpdateOK_Item_ok {
            subject,
            serialNumber,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUpdateOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateOK_Item_ok(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUpdateOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateOK_Item_ok(el)
    }
}

pub const _rctl1_components_for_CertUpdateOK_Item_ok: &[ComponentSpec; 2] = &[
    ComponentSpec::new("subject", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertUpdateOK_Item_ok: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUpdateOK_Item_ok: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUpdateOK_Item_ok(el: &X690Element) -> ASN1Result<CertUpdateOK_Item_ok> {
    |el_: &X690Element| -> ASN1Result<CertUpdateOK_Item_ok> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUpdateOK_Item_ok,
            _eal_components_for_CertUpdateOK_Item_ok,
            _rctl2_components_for_CertUpdateOK_Item_ok,
        )?;
        let subject = _decode_Name(_components.get("subject").unwrap())?;
        let serialNumber =
            _decode_CertificateSerialNumber(_components.get("serialNumber").unwrap())?;
        Ok(CertUpdateOK_Item_ok {
            subject,
            serialNumber,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUpdateOK_Item_ok(value_: &CertUpdateOK_Item_ok) -> ASN1Result<X690Element> {
    |value_: &CertUpdateOK_Item_ok| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Name(&value_.subject)?);
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
/// CertUpdateOK-Item-not-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertUpdateOK_Item_not_ok {
    pub status: CASP_CertStatusCode,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUpdateOK_Item_not_ok {
    pub fn new(status: CASP_CertStatusCode, _unrecognized: Vec<X690Element>) -> Self {
        CertUpdateOK_Item_not_ok {
            status,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertUpdateOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateOK_Item_not_ok(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUpdateOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateOK_Item_not_ok(el)
    }
}

pub const _rctl1_components_for_CertUpdateOK_Item_not_ok: &[ComponentSpec; 1] =
    &[ComponentSpec::new(
        "status",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    )];

pub const _rctl2_components_for_CertUpdateOK_Item_not_ok: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertUpdateOK_Item_not_ok: &[ComponentSpec; 0] = &[];

pub fn _decode_CertUpdateOK_Item_not_ok(el: &X690Element) -> ASN1Result<CertUpdateOK_Item_not_ok> {
    |el_: &X690Element| -> ASN1Result<CertUpdateOK_Item_not_ok> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertUpdateOK_Item_not_ok,
            _eal_components_for_CertUpdateOK_Item_not_ok,
            _rctl2_components_for_CertUpdateOK_Item_not_ok,
        )?;
        let status = _decode_CASP_CertStatusCode(_components.get("status").unwrap())?;
        Ok(CertUpdateOK_Item_not_ok {
            status,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertUpdateOK_Item_not_ok(
    value_: &CertUpdateOK_Item_not_ok,
) -> ASN1Result<X690Element> {
    |value_: &CertUpdateOK_Item_not_ok| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_CASP_CertStatusCode(&value_.status)?);
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
/// CertUpdateOK-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertUpdateOK_Item {
    ok(CertUpdateOK_Item_ok),
    not_ok(CertUpdateOK_Item_not_ok),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CertUpdateOK_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateOK_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertUpdateOK_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateOK_Item(el)
    }
}

pub fn _decode_CertUpdateOK_Item(el: &X690Element) -> ASN1Result<CertUpdateOK_Item> {
    |el: &X690Element| -> ASN1Result<CertUpdateOK_Item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(CertUpdateOK_Item::ok(_decode_CertUpdateOK_Item_ok(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(CertUpdateOK_Item::not_ok(
                _decode_CertUpdateOK_Item_not_ok(&el)?,
            )),
            _ => Ok(CertUpdateOK_Item::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CertUpdateOK_Item(value_: &CertUpdateOK_Item) -> ASN1Result<X690Element> {
    |value: &CertUpdateOK_Item| -> ASN1Result<X690Element> {
        match value {
            CertUpdateOK_Item::ok(v) => |v_1: &CertUpdateOK_Item_ok| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertUpdateOK_Item_ok(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            CertUpdateOK_Item::not_ok(v) => {
                |v_1: &CertUpdateOK_Item_not_ok| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_CertUpdateOK_Item_not_ok(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            CertUpdateOK_Item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}
