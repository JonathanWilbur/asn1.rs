#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # AVL-management
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `AVL-management`.
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
/// AvlProt  ::=  CHOICE {
///   initReq       [0]  InitializationRec,
///   initAcc       [1]  InitializationAcc,
///   initRej       [2]  InitializationRej,
///   initAbt       [3]  InitializationAbort,
///   certReq       [4]  CertReq,
///   certRsp       [5]  CertRsp,
///   addAvlReq     [6]  AddAvlReq,
///   addAvlRsp     [7]  AddAvlRsp,
///   replaceAvlReq [8]  ReplaceAvlReq,
///   replaceAvlRsp [9]  ReplaceAvlRsp,
///   deleteAvlReq  [10] DeleteAvlReq,
///   deleteAvlRsp  [11] DeleteAvlRsp,
///   abortAVL      [12] AbortAVL,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum AvlProt {
    initReq(InitializationRec),
    initAcc(InitializationAcc),
    initRej(InitializationRej),
    initAbt(InitializationAbort),
    certReq(CertReq),
    certRsp(CertRsp),
    addAvlReq(AddAvlReq),
    addAvlRsp(AddAvlRsp),
    replaceAvlReq(ReplaceAvlReq),
    replaceAvlRsp(ReplaceAvlRsp),
    deleteAvlReq(DeleteAvlReq),
    deleteAvlRsp(DeleteAvlRsp),
    abortAVL(AbortAVL),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for AvlProt {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AvlProt(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AvlProt {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AvlProt(el)
    }
}

pub fn _decode_AvlProt(el: &X690Element) -> ASN1Result<AvlProt> {
    |el: &X690Element| -> ASN1Result<AvlProt> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(AvlProt::initReq(_decode_InitializationRec(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(AvlProt::initAcc(_decode_InitializationAcc(&el)?)),
            (TagClass::CONTEXT, 2) => Ok(AvlProt::initRej(_decode_InitializationRej(&el)?)),
            (TagClass::CONTEXT, 3) => Ok(AvlProt::initAbt(_decode_InitializationAbort(&el)?)),
            (TagClass::CONTEXT, 4) => Ok(AvlProt::certReq(_decode_CertReq(&el)?)),
            (TagClass::CONTEXT, 5) => Ok(AvlProt::certRsp(_decode_CertRsp(&el)?)),
            (TagClass::CONTEXT, 6) => Ok(AvlProt::addAvlReq(_decode_AddAvlReq(&el)?)),
            (TagClass::CONTEXT, 7) => Ok(AvlProt::addAvlRsp(_decode_AddAvlRsp(&el)?)),
            (TagClass::CONTEXT, 8) => Ok(AvlProt::replaceAvlReq(_decode_ReplaceAvlReq(&el)?)),
            (TagClass::CONTEXT, 9) => Ok(AvlProt::replaceAvlRsp(_decode_ReplaceAvlRsp(&el)?)),
            (TagClass::CONTEXT, 10) => Ok(AvlProt::deleteAvlReq(_decode_DeleteAvlReq(&el)?)),
            (TagClass::CONTEXT, 11) => Ok(AvlProt::deleteAvlRsp(_decode_DeleteAvlRsp(&el)?)),
            (TagClass::CONTEXT, 12) => Ok(AvlProt::abortAVL(_decode_AbortAVL(&el)?)),
            _ => Ok(AvlProt::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_AvlProt(value_: &AvlProt) -> ASN1Result<X690Element> {
    |value: &AvlProt| -> ASN1Result<X690Element> {
        match value {
            AvlProt::initReq(v) => |v_1: &InitializationRec| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InitializationRec(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            AvlProt::initAcc(v) => |v_1: &InitializationAcc| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InitializationAcc(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            AvlProt::initRej(v) => |v_1: &InitializationRej| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InitializationRej(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v),
            AvlProt::initAbt(v) => |v_1: &InitializationAbort| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_InitializationAbort(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v),
            AvlProt::certReq(v) => |v_1: &CertReq| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertReq(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 4;
                Ok(el_1)
            }(&v),
            AvlProt::certRsp(v) => |v_1: &CertRsp| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertRsp(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 5;
                Ok(el_1)
            }(&v),
            AvlProt::addAvlReq(v) => |v_1: &AddAvlReq| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AddAvlReq(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 6;
                Ok(el_1)
            }(&v),
            AvlProt::addAvlRsp(v) => |v_1: &AddAvlRsp| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AddAvlRsp(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 7;
                Ok(el_1)
            }(&v),
            AvlProt::replaceAvlReq(v) => |v_1: &ReplaceAvlReq| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ReplaceAvlReq(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 8;
                Ok(el_1)
            }(&v),
            AvlProt::replaceAvlRsp(v) => |v_1: &ReplaceAvlRsp| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ReplaceAvlRsp(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 9;
                Ok(el_1)
            }(&v),
            AvlProt::deleteAvlReq(v) => |v_1: &DeleteAvlReq| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_DeleteAvlReq(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 10;
                Ok(el_1)
            }(&v),
            AvlProt::deleteAvlRsp(v) => |v_1: &DeleteAvlRsp| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_DeleteAvlRsp(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 11;
                Ok(el_1)
            }(&v),
            AvlProt::abortAVL(v) => |v_1: &AbortAVL| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AbortAVL(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 12;
                Ok(el_1)
            }(&v),
            AvlProt::_unrecognized(el) => Ok(el.clone()),
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
/// CertReq ::= SEQUENCE {
///   invokeID  InvokeID,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertReq {
    pub invokeID: InvokeID,
    pub _unrecognized: Vec<X690Element>,
}
impl CertReq {
    pub fn new(invokeID: InvokeID, _unrecognized: Vec<X690Element>) -> Self {
        CertReq {
            invokeID,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertReq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertReq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertReq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertReq(el)
    }
}

pub const _rctl1_components_for_CertReq: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "invokeID",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 2)),
    None,
    None,
)];

pub const _rctl2_components_for_CertReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertReq: &[ComponentSpec; 0] = &[];

pub fn _decode_CertReq(el: &X690Element) -> ASN1Result<CertReq> {
    |el_: &X690Element| -> ASN1Result<CertReq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertReq,
            _eal_components_for_CertReq,
            _rctl2_components_for_CertReq,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        Ok(CertReq {
            invokeID,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertReq(value_: &CertReq) -> ASN1Result<X690Element> {
    |value_: &CertReq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
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
/// CertRsp ::= SEQUENCE {
///   invokeID     InvokeID,
///   result       CHOICE {
///     success [0]  CertOK,
///     failure [1]  CertErr,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertRsp {
    pub invokeID: InvokeID,
    pub result: CertRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl CertRsp {
    pub fn new(
        invokeID: InvokeID,
        result: CertRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertRsp {
            invokeID,
            result,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertRsp {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertRsp(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertRsp {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertRsp(el)
    }
}

pub const _rctl1_components_for_CertRsp: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("result", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_CertRsp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertRsp: &[ComponentSpec; 0] = &[];

pub fn _decode_CertRsp(el: &X690Element) -> ASN1Result<CertRsp> {
    |el_: &X690Element| -> ASN1Result<CertRsp> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertRsp,
            _eal_components_for_CertRsp,
            _rctl2_components_for_CertRsp,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let result = _decode_CertRsp_result(_components.get("result").unwrap())?;
        Ok(CertRsp {
            invokeID,
            result,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertRsp(value_: &CertRsp) -> ASN1Result<X690Element> {
    |value_: &CertRsp| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(_encode_CertRsp_result(&value_.result)?);
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
/// CertOK ::= SEQUENCE {
///   dhCert  Certificate,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertOK {
    pub dhCert: Certificate,
    pub _unrecognized: Vec<X690Element>,
}
impl CertOK {
    pub fn new(dhCert: Certificate, _unrecognized: Vec<X690Element>) -> Self {
        CertOK {
            dhCert,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertOK {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertOK(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertOK {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertOK(el)
    }
}

pub const _rctl1_components_for_CertOK: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "dhCert",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 16)),
    None,
    None,
)];

pub const _rctl2_components_for_CertOK: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertOK: &[ComponentSpec; 0] = &[];

pub fn _decode_CertOK(el: &X690Element) -> ASN1Result<CertOK> {
    |el_: &X690Element| -> ASN1Result<CertOK> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertOK,
            _eal_components_for_CertOK,
            _rctl2_components_for_CertOK,
        )?;
        let dhCert = _decode_Certificate(_components.get("dhCert").unwrap())?;
        Ok(CertOK {
            dhCert,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertOK(value_: &CertOK) -> ASN1Result<X690Element> {
    |value_: &CertOK| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_Certificate(&value_.dhCert)?);
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
/// CertErr ::= SEQUENCE {
///   notOK  AVMP-error,
///   note   Notifications OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CertErr {
    pub notOK: AVMP_error,
    pub note: OPTIONAL<Notifications>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertErr {
    pub fn new(
        notOK: AVMP_error,
        note: OPTIONAL<Notifications>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertErr {
            notOK,
            note,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for CertErr {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertErr(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertErr {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertErr(el)
    }
}

pub const _rctl1_components_for_CertErr: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "notOK",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "note",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertErr: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertErr: &[ComponentSpec; 0] = &[];

pub fn _decode_CertErr(el: &X690Element) -> ASN1Result<CertErr> {
    |el_: &X690Element| -> ASN1Result<CertErr> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CertErr,
            _eal_components_for_CertErr,
            _rctl2_components_for_CertErr,
        )?;
        let notOK = _decode_AVMP_error(_components.get("notOK").unwrap())?;
        let note: OPTIONAL<Notifications> = match _components.get("note") {
            Some(c_) => Some(_decode_Notifications(c_)?),
            _ => None,
        };
        Ok(CertErr {
            notOK,
            note,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CertErr(value_: &CertErr) -> ASN1Result<X690Element> {
    |value_: &CertErr| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AVMP_error(&value_.notOK)?);
        if let Some(v_) = &value_.note {
            components_.push(_encode_Notifications(&v_)?);
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
/// Notifications  ::=  SEQUENCE SIZE (1..MAX) OF Attribute {{SupportedAttributes}}
/// ```
pub type Notifications = Vec<Attribute>; // SequenceOfType

pub fn _decode_Notifications(el: &X690Element) -> ASN1Result<Notifications> {
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

pub fn _encode_Notifications(value_: &Notifications) -> ASN1Result<X690Element> {
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
/// AddAvlReq ::= SEQUENCE {
///   invokeID     InvokeID,
///   certlist     CertAVL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AddAvlReq {
    pub invokeID: InvokeID,
    pub certlist: CertAVL,
    pub _unrecognized: Vec<X690Element>,
}
impl AddAvlReq {
    pub fn new(invokeID: InvokeID, certlist: CertAVL, _unrecognized: Vec<X690Element>) -> Self {
        AddAvlReq {
            invokeID,
            certlist,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AddAvlReq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlReq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AddAvlReq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlReq(el)
    }
}

pub const _rctl1_components_for_AddAvlReq: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certlist",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AddAvlReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AddAvlReq: &[ComponentSpec; 0] = &[];

pub fn _decode_AddAvlReq(el: &X690Element) -> ASN1Result<AddAvlReq> {
    |el_: &X690Element| -> ASN1Result<AddAvlReq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AddAvlReq,
            _eal_components_for_AddAvlReq,
            _rctl2_components_for_AddAvlReq,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let certlist = _decode_CertAVL(_components.get("certlist").unwrap())?;
        Ok(AddAvlReq {
            invokeID,
            certlist,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AddAvlReq(value_: &AddAvlReq) -> ASN1Result<X690Element> {
    |value_: &AddAvlReq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(_encode_CertAVL(&value_.certlist)?);
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
/// AddAvlRsp ::= SEQUENCE {
///   invokeID     InvokeID,
///   result       CHOICE {
///     success [0]  AddAvlOK,
///     failure [1]  AddAvlErr,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AddAvlRsp {
    pub invokeID: InvokeID,
    pub result: AddAvlRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl AddAvlRsp {
    pub fn new(
        invokeID: InvokeID,
        result: AddAvlRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AddAvlRsp {
            invokeID,
            result,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AddAvlRsp {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlRsp(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AddAvlRsp {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlRsp(el)
    }
}

pub const _rctl1_components_for_AddAvlRsp: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("result", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_AddAvlRsp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AddAvlRsp: &[ComponentSpec; 0] = &[];

pub fn _decode_AddAvlRsp(el: &X690Element) -> ASN1Result<AddAvlRsp> {
    |el_: &X690Element| -> ASN1Result<AddAvlRsp> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AddAvlRsp,
            _eal_components_for_AddAvlRsp,
            _rctl2_components_for_AddAvlRsp,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let result = _decode_AddAvlRsp_result(_components.get("result").unwrap())?;
        Ok(AddAvlRsp {
            invokeID,
            result,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AddAvlRsp(value_: &AddAvlRsp) -> ASN1Result<X690Element> {
    |value_: &AddAvlRsp| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(_encode_AddAvlRsp_result(&value_.result)?);
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
/// AddAvlOK ::= SEQUENCE {
///   ok     NULL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AddAvlOK {
    pub ok: NULL,
    pub _unrecognized: Vec<X690Element>,
}
impl AddAvlOK {
    pub fn new(ok: NULL, _unrecognized: Vec<X690Element>) -> Self {
        AddAvlOK { ok, _unrecognized }
    }
}
impl TryFrom<X690Element> for AddAvlOK {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlOK(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AddAvlOK {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlOK(el)
    }
}

pub const _rctl1_components_for_AddAvlOK: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "ok",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 5)),
    None,
    None,
)];

pub const _rctl2_components_for_AddAvlOK: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AddAvlOK: &[ComponentSpec; 0] = &[];

pub fn _decode_AddAvlOK(el: &X690Element) -> ASN1Result<AddAvlOK> {
    |el_: &X690Element| -> ASN1Result<AddAvlOK> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AddAvlOK,
            _eal_components_for_AddAvlOK,
            _rctl2_components_for_AddAvlOK,
        )?;
        let ok = ber_decode_null(_components.get("ok").unwrap())?;
        Ok(AddAvlOK {
            ok: (),
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AddAvlOK(value_: &AddAvlOK) -> ASN1Result<X690Element> {
    |value_: &AddAvlOK| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(ber_encode_null(&value_.ok)?);
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
/// AddAvlErr ::= SEQUENCE {
///   notOK  AVMP-error,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AddAvlErr {
    pub notOK: AVMP_error,
    pub _unrecognized: Vec<X690Element>,
}
impl AddAvlErr {
    pub fn new(notOK: AVMP_error, _unrecognized: Vec<X690Element>) -> Self {
        AddAvlErr {
            notOK,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AddAvlErr {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlErr(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AddAvlErr {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlErr(el)
    }
}

pub const _rctl1_components_for_AddAvlErr: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "notOK",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 10)),
    None,
    None,
)];

pub const _rctl2_components_for_AddAvlErr: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AddAvlErr: &[ComponentSpec; 0] = &[];

pub fn _decode_AddAvlErr(el: &X690Element) -> ASN1Result<AddAvlErr> {
    |el_: &X690Element| -> ASN1Result<AddAvlErr> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AddAvlErr,
            _eal_components_for_AddAvlErr,
            _rctl2_components_for_AddAvlErr,
        )?;
        let notOK = _decode_AVMP_error(_components.get("notOK").unwrap())?;
        Ok(AddAvlErr {
            notOK,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AddAvlErr(value_: &AddAvlErr) -> ASN1Result<X690Element> {
    |value_: &AddAvlErr| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_AVMP_error(&value_.notOK)?);
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
/// ReplaceAvlReq ::= SEQUENCE {
///   invokeID     InvokeID,
///   old          AvlSerialNumber OPTIONAL,
///   new          CertAVL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ReplaceAvlReq {
    pub invokeID: InvokeID,
    pub old: OPTIONAL<AvlSerialNumber>,
    pub new: CertAVL,
    pub _unrecognized: Vec<X690Element>,
}
impl ReplaceAvlReq {
    pub fn new(
        invokeID: InvokeID,
        old: OPTIONAL<AvlSerialNumber>,
        new: CertAVL,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ReplaceAvlReq {
            invokeID,
            old,
            new,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ReplaceAvlReq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ReplaceAvlReq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ReplaceAvlReq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ReplaceAvlReq(el)
    }
}

pub const _rctl1_components_for_ReplaceAvlReq: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "old",
        true,
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

pub const _rctl2_components_for_ReplaceAvlReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ReplaceAvlReq: &[ComponentSpec; 0] = &[];

pub fn _decode_ReplaceAvlReq(el: &X690Element) -> ASN1Result<ReplaceAvlReq> {
    |el_: &X690Element| -> ASN1Result<ReplaceAvlReq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ReplaceAvlReq,
            _eal_components_for_ReplaceAvlReq,
            _rctl2_components_for_ReplaceAvlReq,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let old: OPTIONAL<AvlSerialNumber> = match _components.get("old") {
            Some(c_) => Some(_decode_AvlSerialNumber(c_)?),
            _ => None,
        };
        let new = _decode_CertAVL(_components.get("new").unwrap())?;
        Ok(ReplaceAvlReq {
            invokeID,
            old,
            new,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ReplaceAvlReq(value_: &ReplaceAvlReq) -> ASN1Result<X690Element> {
    |value_: &ReplaceAvlReq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        if let Some(v_) = &value_.old {
            components_.push(_encode_AvlSerialNumber(&v_)?);
        }
        components_.push(_encode_CertAVL(&value_.new)?);
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
/// ReplaceAvlRsp ::= SEQUENCE {
///   invokeID     InvokeID,
///   result       CHOICE {
///     success [0]  RepAvlOK,
///     failure [1]  RepAvlErr,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ReplaceAvlRsp {
    pub invokeID: InvokeID,
    pub result: ReplaceAvlRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl ReplaceAvlRsp {
    pub fn new(
        invokeID: InvokeID,
        result: ReplaceAvlRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ReplaceAvlRsp {
            invokeID,
            result,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ReplaceAvlRsp {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ReplaceAvlRsp(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ReplaceAvlRsp {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ReplaceAvlRsp(el)
    }
}

pub const _rctl1_components_for_ReplaceAvlRsp: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("result", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_ReplaceAvlRsp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ReplaceAvlRsp: &[ComponentSpec; 0] = &[];

pub fn _decode_ReplaceAvlRsp(el: &X690Element) -> ASN1Result<ReplaceAvlRsp> {
    |el_: &X690Element| -> ASN1Result<ReplaceAvlRsp> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ReplaceAvlRsp,
            _eal_components_for_ReplaceAvlRsp,
            _rctl2_components_for_ReplaceAvlRsp,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let result = _decode_ReplaceAvlRsp_result(_components.get("result").unwrap())?;
        Ok(ReplaceAvlRsp {
            invokeID,
            result,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ReplaceAvlRsp(value_: &ReplaceAvlRsp) -> ASN1Result<X690Element> {
    |value_: &ReplaceAvlRsp| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(_encode_ReplaceAvlRsp_result(&value_.result)?);
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
/// RepAvlOK ::= SEQUENCE {
///   ok     NULL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RepAvlOK {
    pub ok: NULL,
    pub _unrecognized: Vec<X690Element>,
}
impl RepAvlOK {
    pub fn new(ok: NULL, _unrecognized: Vec<X690Element>) -> Self {
        RepAvlOK { ok, _unrecognized }
    }
}
impl TryFrom<X690Element> for RepAvlOK {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RepAvlOK(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RepAvlOK {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RepAvlOK(el)
    }
}

pub const _rctl1_components_for_RepAvlOK: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "ok",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 5)),
    None,
    None,
)];

pub const _rctl2_components_for_RepAvlOK: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RepAvlOK: &[ComponentSpec; 0] = &[];

pub fn _decode_RepAvlOK(el: &X690Element) -> ASN1Result<RepAvlOK> {
    |el_: &X690Element| -> ASN1Result<RepAvlOK> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RepAvlOK,
            _eal_components_for_RepAvlOK,
            _rctl2_components_for_RepAvlOK,
        )?;
        let ok = ber_decode_null(_components.get("ok").unwrap())?;
        Ok(RepAvlOK {
            ok: (),
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_RepAvlOK(value_: &RepAvlOK) -> ASN1Result<X690Element> {
    |value_: &RepAvlOK| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(ber_encode_null(&value_.ok)?);
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
/// RepAvlErr ::= SEQUENCE {
///   notOK  AVMP-error,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RepAvlErr {
    pub notOK: AVMP_error,
    pub _unrecognized: Vec<X690Element>,
}
impl RepAvlErr {
    pub fn new(notOK: AVMP_error, _unrecognized: Vec<X690Element>) -> Self {
        RepAvlErr {
            notOK,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for RepAvlErr {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RepAvlErr(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RepAvlErr {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RepAvlErr(el)
    }
}

pub const _rctl1_components_for_RepAvlErr: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "notOK",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 10)),
    None,
    None,
)];

pub const _rctl2_components_for_RepAvlErr: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RepAvlErr: &[ComponentSpec; 0] = &[];

pub fn _decode_RepAvlErr(el: &X690Element) -> ASN1Result<RepAvlErr> {
    |el_: &X690Element| -> ASN1Result<RepAvlErr> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RepAvlErr,
            _eal_components_for_RepAvlErr,
            _rctl2_components_for_RepAvlErr,
        )?;
        let notOK = _decode_AVMP_error(_components.get("notOK").unwrap())?;
        Ok(RepAvlErr {
            notOK,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_RepAvlErr(value_: &RepAvlErr) -> ASN1Result<X690Element> {
    |value_: &RepAvlErr| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_AVMP_error(&value_.notOK)?);
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
/// DeleteAvlReq ::= SEQUENCE {
///   invokeID     InvokeID,
///   avl-Id       AvlSerialNumber OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DeleteAvlReq {
    pub invokeID: InvokeID,
    pub avl_Id: OPTIONAL<AvlSerialNumber>,
    pub _unrecognized: Vec<X690Element>,
}
impl DeleteAvlReq {
    pub fn new(
        invokeID: InvokeID,
        avl_Id: OPTIONAL<AvlSerialNumber>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DeleteAvlReq {
            invokeID,
            avl_Id,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for DeleteAvlReq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DeleteAvlReq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DeleteAvlReq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DeleteAvlReq(el)
    }
}

pub const _rctl1_components_for_DeleteAvlReq: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "avl-Id",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DeleteAvlReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DeleteAvlReq: &[ComponentSpec; 0] = &[];

pub fn _decode_DeleteAvlReq(el: &X690Element) -> ASN1Result<DeleteAvlReq> {
    |el_: &X690Element| -> ASN1Result<DeleteAvlReq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DeleteAvlReq,
            _eal_components_for_DeleteAvlReq,
            _rctl2_components_for_DeleteAvlReq,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let avl_Id: OPTIONAL<AvlSerialNumber> = match _components.get("avl-Id") {
            Some(c_) => Some(_decode_AvlSerialNumber(c_)?),
            _ => None,
        };
        Ok(DeleteAvlReq {
            invokeID,
            avl_Id,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DeleteAvlReq(value_: &DeleteAvlReq) -> ASN1Result<X690Element> {
    |value_: &DeleteAvlReq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        if let Some(v_) = &value_.avl_Id {
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
/// DeleteAvlRsp ::= SEQUENCE {
///   invokeID     InvokeID,
///   result       CHOICE {
///     success [0]  DelAvlOK,
///     failure [1]  DelAvlErr,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DeleteAvlRsp {
    pub invokeID: InvokeID,
    pub result: DeleteAvlRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl DeleteAvlRsp {
    pub fn new(
        invokeID: InvokeID,
        result: DeleteAvlRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DeleteAvlRsp {
            invokeID,
            result,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for DeleteAvlRsp {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DeleteAvlRsp(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DeleteAvlRsp {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DeleteAvlRsp(el)
    }
}

pub const _rctl1_components_for_DeleteAvlRsp: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("result", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_DeleteAvlRsp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DeleteAvlRsp: &[ComponentSpec; 0] = &[];

pub fn _decode_DeleteAvlRsp(el: &X690Element) -> ASN1Result<DeleteAvlRsp> {
    |el_: &X690Element| -> ASN1Result<DeleteAvlRsp> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DeleteAvlRsp,
            _eal_components_for_DeleteAvlRsp,
            _rctl2_components_for_DeleteAvlRsp,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let result = _decode_DeleteAvlRsp_result(_components.get("result").unwrap())?;
        Ok(DeleteAvlRsp {
            invokeID,
            result,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DeleteAvlRsp(value_: &DeleteAvlRsp) -> ASN1Result<X690Element> {
    |value_: &DeleteAvlRsp| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(_encode_DeleteAvlRsp_result(&value_.result)?);
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
/// DelAvlOK ::= SEQUENCE {
///   ok     NULL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DelAvlOK {
    pub ok: NULL,
    pub _unrecognized: Vec<X690Element>,
}
impl DelAvlOK {
    pub fn new(ok: NULL, _unrecognized: Vec<X690Element>) -> Self {
        DelAvlOK { ok, _unrecognized }
    }
}
impl TryFrom<X690Element> for DelAvlOK {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DelAvlOK(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DelAvlOK {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DelAvlOK(el)
    }
}

pub const _rctl1_components_for_DelAvlOK: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "ok",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 5)),
    None,
    None,
)];

pub const _rctl2_components_for_DelAvlOK: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DelAvlOK: &[ComponentSpec; 0] = &[];

pub fn _decode_DelAvlOK(el: &X690Element) -> ASN1Result<DelAvlOK> {
    |el_: &X690Element| -> ASN1Result<DelAvlOK> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DelAvlOK,
            _eal_components_for_DelAvlOK,
            _rctl2_components_for_DelAvlOK,
        )?;
        let ok = ber_decode_null(_components.get("ok").unwrap())?;
        Ok(DelAvlOK {
            ok: (),
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DelAvlOK(value_: &DelAvlOK) -> ASN1Result<X690Element> {
    |value_: &DelAvlOK| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(ber_encode_null(&value_.ok)?);
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
/// DelAvlErr ::= SEQUENCE {
///   notOK  AVMP-error,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DelAvlErr {
    pub notOK: AVMP_error,
    pub _unrecognized: Vec<X690Element>,
}
impl DelAvlErr {
    pub fn new(notOK: AVMP_error, _unrecognized: Vec<X690Element>) -> Self {
        DelAvlErr {
            notOK,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for DelAvlErr {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DelAvlErr(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DelAvlErr {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DelAvlErr(el)
    }
}

pub const _rctl1_components_for_DelAvlErr: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "notOK",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 10)),
    None,
    None,
)];

pub const _rctl2_components_for_DelAvlErr: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DelAvlErr: &[ComponentSpec; 0] = &[];

pub fn _decode_DelAvlErr(el: &X690Element) -> ASN1Result<DelAvlErr> {
    |el_: &X690Element| -> ASN1Result<DelAvlErr> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DelAvlErr,
            _eal_components_for_DelAvlErr,
            _rctl2_components_for_DelAvlErr,
        )?;
        let notOK = _decode_AVMP_error(_components.get("notOK").unwrap())?;
        Ok(DelAvlErr {
            notOK,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DelAvlErr(value_: &DelAvlErr) -> ASN1Result<X690Element> {
    |value_: &DelAvlErr| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_AVMP_error(&value_.notOK)?);
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
/// AbortAVL ::= SEQUENCE {
///   invokeID     InvokeID,
///   reason       AVMP-error,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AbortAVL {
    pub invokeID: InvokeID,
    pub reason: AVMP_error,
    pub _unrecognized: Vec<X690Element>,
}
impl AbortAVL {
    pub fn new(invokeID: InvokeID, reason: AVMP_error, _unrecognized: Vec<X690Element>) -> Self {
        AbortAVL {
            invokeID,
            reason,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AbortAVL {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AbortAVL(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AbortAVL {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AbortAVL(el)
    }
}

pub const _rctl1_components_for_AbortAVL: &[ComponentSpec; 2] = &[
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

pub const _rctl2_components_for_AbortAVL: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AbortAVL: &[ComponentSpec; 0] = &[];

pub fn _decode_AbortAVL(el: &X690Element) -> ASN1Result<AbortAVL> {
    |el_: &X690Element| -> ASN1Result<AbortAVL> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AbortAVL,
            _eal_components_for_AbortAVL,
            _rctl2_components_for_AbortAVL,
        )?;
        let invokeID = _decode_InvokeID(_components.get("invokeID").unwrap())?;
        let reason = _decode_AVMP_error(_components.get("reason").unwrap())?;
        Ok(AbortAVL {
            invokeID,
            reason,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AbortAVL(value_: &AbortAVL) -> ASN1Result<X690Element> {
    |value_: &AbortAVL| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_InvokeID(&value_.invokeID)?);
        components_.push(_encode_AVMP_error(&value_.reason)?);
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
/// AVMP-error  ::=  ENUMERATED {
///   noReason                           (0),
///   protocolError                      (1),
///   duplicateAVL                       (2),
///   missingAvlComponent                (3),
///   invalidAvlVersion                  (4),
///   notAllowedForConstrainedAVLEntity  (5),
///   constrainedRequired                (6),
///   nonConstrainedRequired             (7),
///   unsupportedCriticalEntryExtenssion (8),
///   unsupportedCriticalExtenssion      (9),
///   maxAVLsExceeded                    (10),
///   unknownAVL                         (11),
///   ... }
/// ```
pub type AVMP_error = ENUMERATED;

pub const AVMP_error_noReason: AVMP_error = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_protocolError: AVMP_error = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_duplicateAVL: AVMP_error = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_missingAvlComponent: AVMP_error = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_invalidAvlVersion: AVMP_error = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_notAllowedForConstrainedAVLEntity: AVMP_error = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_constrainedRequired: AVMP_error = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_nonConstrainedRequired: AVMP_error = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_unsupportedCriticalEntryExtenssion: AVMP_error = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_unsupportedCriticalExtenssion: AVMP_error = 9; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_maxAVLsExceeded: AVMP_error = 10; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_unknownAVL: AVMP_error = 11; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_AVMP_error(el: &X690Element) -> ASN1Result<AVMP_error> {
    ber_decode_enumerated(&el)
}

pub fn _encode_AVMP_error(value_: &AVMP_error) -> ASN1Result<X690Element> {
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
/// CertRsp-result ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertRsp_result {
    success(CertOK),
    failure(CertErr),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CertRsp_result {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CertRsp_result(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CertRsp_result {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CertRsp_result(el)
    }
}

pub fn _decode_CertRsp_result(el: &X690Element) -> ASN1Result<CertRsp_result> {
    |el: &X690Element| -> ASN1Result<CertRsp_result> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(CertRsp_result::success(_decode_CertOK(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(CertRsp_result::failure(_decode_CertErr(&el)?)),
            _ => Ok(CertRsp_result::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CertRsp_result(value_: &CertRsp_result) -> ASN1Result<X690Element> {
    |value: &CertRsp_result| -> ASN1Result<X690Element> {
        match value {
            CertRsp_result::success(v) => |v_1: &CertOK| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertOK(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            CertRsp_result::failure(v) => |v_1: &CertErr| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertErr(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            CertRsp_result::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AddAvlRsp-result ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum AddAvlRsp_result {
    success(AddAvlOK),
    failure(AddAvlErr),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for AddAvlRsp_result {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlRsp_result(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AddAvlRsp_result {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlRsp_result(el)
    }
}

pub fn _decode_AddAvlRsp_result(el: &X690Element) -> ASN1Result<AddAvlRsp_result> {
    |el: &X690Element| -> ASN1Result<AddAvlRsp_result> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(AddAvlRsp_result::success(_decode_AddAvlOK(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(AddAvlRsp_result::failure(_decode_AddAvlErr(&el)?)),
            _ => Ok(AddAvlRsp_result::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_AddAvlRsp_result(value_: &AddAvlRsp_result) -> ASN1Result<X690Element> {
    |value: &AddAvlRsp_result| -> ASN1Result<X690Element> {
        match value {
            AddAvlRsp_result::success(v) => |v_1: &AddAvlOK| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AddAvlOK(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            AddAvlRsp_result::failure(v) => |v_1: &AddAvlErr| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AddAvlErr(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            AddAvlRsp_result::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReplaceAvlRsp-result ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum ReplaceAvlRsp_result {
    success(RepAvlOK),
    failure(RepAvlErr),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ReplaceAvlRsp_result {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ReplaceAvlRsp_result(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ReplaceAvlRsp_result {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ReplaceAvlRsp_result(el)
    }
}

pub fn _decode_ReplaceAvlRsp_result(el: &X690Element) -> ASN1Result<ReplaceAvlRsp_result> {
    |el: &X690Element| -> ASN1Result<ReplaceAvlRsp_result> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(ReplaceAvlRsp_result::success(_decode_RepAvlOK(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(ReplaceAvlRsp_result::failure(_decode_RepAvlErr(&el)?)),
            _ => Ok(ReplaceAvlRsp_result::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ReplaceAvlRsp_result(value_: &ReplaceAvlRsp_result) -> ASN1Result<X690Element> {
    |value: &ReplaceAvlRsp_result| -> ASN1Result<X690Element> {
        match value {
            ReplaceAvlRsp_result::success(v) => |v_1: &RepAvlOK| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_RepAvlOK(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            ReplaceAvlRsp_result::failure(v) => |v_1: &RepAvlErr| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_RepAvlErr(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            ReplaceAvlRsp_result::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DeleteAvlRsp-result ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum DeleteAvlRsp_result {
    success(DelAvlOK),
    failure(DelAvlErr),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for DeleteAvlRsp_result {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DeleteAvlRsp_result(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DeleteAvlRsp_result {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DeleteAvlRsp_result(el)
    }
}

pub fn _decode_DeleteAvlRsp_result(el: &X690Element) -> ASN1Result<DeleteAvlRsp_result> {
    |el: &X690Element| -> ASN1Result<DeleteAvlRsp_result> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(DeleteAvlRsp_result::success(_decode_DelAvlOK(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(DeleteAvlRsp_result::failure(_decode_DelAvlErr(&el)?)),
            _ => Ok(DeleteAvlRsp_result::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_DeleteAvlRsp_result(value_: &DeleteAvlRsp_result) -> ASN1Result<X690Element> {
    |value: &DeleteAvlRsp_result| -> ASN1Result<X690Element> {
        match value {
            DeleteAvlRsp_result::success(v) => |v_1: &DelAvlOK| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_DelAvlOK(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            DeleteAvlRsp_result::failure(v) => |v_1: &DelAvlErr| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_DelAvlErr(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            DeleteAvlRsp_result::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}
