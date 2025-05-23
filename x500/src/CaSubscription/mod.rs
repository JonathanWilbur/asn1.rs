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
use crate::Wrapper::{Version, _decode_Version, _encode_Version, _validate_Version};
use asn1::*;
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

impl TryFrom<&X690Element> for CasubProt {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CasubProt(el)
    }
}

pub fn _decode_CasubProt(el: &X690Element) -> ASN1Result<CasubProt> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CasubProt::initReq(_decode_InitializationRec(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(CasubProt::initAcc(_decode_InitializationAcc(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(CasubProt::initRej(_decode_InitializationRej(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(CasubProt::initAbt(_decode_InitializationAbort(&el)?)),
        (TagClass::CONTEXT, 4) => Ok(CasubProt::certSubscribeReq(_decode_CertSubscribeReq(&el)?)),
        (TagClass::CONTEXT, 5) => Ok(CasubProt::certSubscribeRsp(_decode_CertSubscribeRsp(&el)?)),
        (TagClass::CONTEXT, 6) => Ok(CasubProt::certUnsubscribeReq(_decode_CertUnsubscribeReq(
            &el,
        )?)),
        (TagClass::CONTEXT, 7) => Ok(CasubProt::certUnsubscribeRsp(_decode_CertUnsubscribeRsp(
            &el,
        )?)),
        (TagClass::CONTEXT, 8) => Ok(CasubProt::certReplaceReq(_decode_CertReplaceReq(&el)?)),
        (TagClass::CONTEXT, 9) => Ok(CasubProt::certReplaceRsp(_decode_CertReplaceRsp(&el)?)),
        (TagClass::CONTEXT, 10) => Ok(CasubProt::certUpdateReq(_decode_CertUpdateReq(&el)?)),
        (TagClass::CONTEXT, 11) => Ok(CasubProt::certUpdateRsp(_decode_CertUpdateRsp(&el)?)),
        (TagClass::CONTEXT, 12) => Ok(CasubProt::cAsubscribeAbort(_decode_CAsubscribeAbort(&el)?)),
        _ => Ok(CasubProt::_unrecognized(el.clone())),
    }
}

pub fn _encode_CasubProt(value_: &CasubProt) -> ASN1Result<X690Element> {
    match value_ {
        CasubProt::initReq(v) => |v_1: &InitializationRec| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_InitializationRec(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        CasubProt::initAcc(v) => |v_1: &InitializationAcc| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_InitializationAcc(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
        CasubProt::initRej(v) => |v_1: &InitializationRej| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_InitializationRej(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v),
        CasubProt::initAbt(v) => |v_1: &InitializationAbort| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_InitializationAbort(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v),
        CasubProt::certSubscribeReq(v) => |v_1: &CertSubscribeReq| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertSubscribeReq(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 4;
            Ok(el_1)
        }(&v),
        CasubProt::certSubscribeRsp(v) => |v_1: &CertSubscribeRsp| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertSubscribeRsp(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 5;
            Ok(el_1)
        }(&v),
        CasubProt::certUnsubscribeReq(v) => |v_1: &CertUnsubscribeReq| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertUnsubscribeReq(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 6;
            Ok(el_1)
        }(&v),
        CasubProt::certUnsubscribeRsp(v) => |v_1: &CertUnsubscribeRsp| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertUnsubscribeRsp(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 7;
            Ok(el_1)
        }(&v),
        CasubProt::certReplaceReq(v) => |v_1: &CertReplaceReq| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertReplaceReq(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 8;
            Ok(el_1)
        }(&v),
        CasubProt::certReplaceRsp(v) => |v_1: &CertReplaceRsp| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertReplaceRsp(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 9;
            Ok(el_1)
        }(&v),
        CasubProt::certUpdateReq(v) => |v_1: &CertUpdateReq| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertUpdateReq(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 10;
            Ok(el_1)
        }(&v),
        CasubProt::certUpdateRsp(v) => |v_1: &CertUpdateRsp| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertUpdateRsp(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 11;
            Ok(el_1)
        }(&v),
        CasubProt::cAsubscribeAbort(v) => |v_1: &CAsubscribeAbort| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CAsubscribeAbort(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 12;
            Ok(el_1)
        }(&v),
        CasubProt::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CasubProt(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "initReq"));
            }
            Ok(_validate_InitializationRec(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "initAcc"));
            }
            Ok(_validate_InitializationAcc(&el)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "initRej"));
            }
            Ok(_validate_InitializationRej(&el)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "initAbt"));
            }
            Ok(_validate_InitializationAbort(&el)?)
        }(&el),
        (TagClass::CONTEXT, 4) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certSubscribeReq")
                );
            }
            Ok(_validate_CertSubscribeReq(&el)?)
        }(&el),
        (TagClass::CONTEXT, 5) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certSubscribeRsp")
                );
            }
            Ok(_validate_CertSubscribeRsp(&el)?)
        }(&el),
        (TagClass::CONTEXT, 6) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certUnsubscribeReq")
                );
            }
            Ok(_validate_CertUnsubscribeReq(&el)?)
        }(&el),
        (TagClass::CONTEXT, 7) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certUnsubscribeRsp")
                );
            }
            Ok(_validate_CertUnsubscribeRsp(&el)?)
        }(&el),
        (TagClass::CONTEXT, 8) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 8 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certReplaceReq")
                );
            }
            Ok(_validate_CertReplaceReq(&el)?)
        }(&el),
        (TagClass::CONTEXT, 9) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 9 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certReplaceRsp")
                );
            }
            Ok(_validate_CertReplaceRsp(&el)?)
        }(&el),
        (TagClass::CONTEXT, 10) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 10 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certUpdateReq")
                );
            }
            Ok(_validate_CertUpdateReq(&el)?)
        }(&el),
        (TagClass::CONTEXT, 11) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 11 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certUpdateRsp")
                );
            }
            Ok(_validate_CertUpdateRsp(&el)?)
        }(&el),
        (TagClass::CONTEXT, 12) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 12 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "cAsubscribeAbort")
                );
            }
            Ok(_validate_CAsubscribeAbort(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InitializationRec ::= SEQUENCE {
///   version    Version,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for InitializationRec {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitializationRec")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InitializationRec,
        _eal_components_for_InitializationRec,
        _rctl2_components_for_InitializationRec,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(InitializationRec {
        version: version_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_InitializationRec(value_: &InitializationRec) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_Version(&value_.version)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_InitializationRec(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitializationRec")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InitializationRec,
        _eal_components_for_InitializationRec,
        _rctl2_components_for_InitializationRec,
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
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InitializationAcc ::= SEQUENCE {
///   version    Version,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for InitializationAcc {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitializationAcc")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InitializationAcc,
        _eal_components_for_InitializationAcc,
        _rctl2_components_for_InitializationAcc,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(InitializationAcc {
        version: version_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_InitializationAcc(value_: &InitializationAcc) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_Version(&value_.version)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_InitializationAcc(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitializationAcc")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InitializationAcc,
        _eal_components_for_InitializationAcc,
        _rctl2_components_for_InitializationAcc,
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
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for InitializationRej {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitializationRej")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InitializationRej,
        _eal_components_for_InitializationRej,
        _rctl2_components_for_InitializationRej,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut diag_: OPTIONAL<InitializationRej_diag> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "diag" => diag_ = Some(_decode_InitializationRej_diag(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(InitializationRej {
        diag: diag_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_InitializationRej(value_: &InitializationRej) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_InitializationRej_diag(&value_.diag)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_InitializationRej(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitializationRej")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InitializationRej,
        _eal_components_for_InitializationRej,
        _rctl2_components_for_InitializationRej,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "diag" => _validate_InitializationRej_diag(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for InitializationAbort {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitializationAbort")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InitializationAbort,
        _eal_components_for_InitializationAbort,
        _rctl2_components_for_InitializationAbort,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut diag_: OPTIONAL<InitializationAbort_diag> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "diag" => diag_ = Some(_decode_InitializationAbort_diag(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(InitializationAbort {
        diag: diag_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_InitializationAbort(value_: &InitializationAbort) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_InitializationAbort_diag(&value_.diag)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_InitializationAbort(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitializationAbort")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InitializationAbort,
        _eal_components_for_InitializationAbort,
        _rctl2_components_for_InitializationAbort,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "diag" => _validate_InitializationAbort_diag(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for CertSubscribeReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertSubscribeReq")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeReq,
        _eal_components_for_CertSubscribeReq,
        _rctl2_components_for_CertSubscribeReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<InvokeID> = None;
    let mut certs_: OPTIONAL<Vec<CertSubscribeReq_certs_Item>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(_decode_InvokeID(_el)?),
            "certs" => {
                certs_ = Some(|el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<CertSubscribeReq_certs_Item>,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certs")
                            )
                        }
                    };
                    let mut items: SEQUENCE_OF<CertSubscribeReq_certs_Item> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_CertSubscribeReq_certs_Item(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertSubscribeReq {
        invokeID: invokeID_.unwrap(),
        certs: certs_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertSubscribeReq(value_: &CertSubscribeReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_InvokeID(&value_.invokeID)?);
    components_.push(
        |value_: &SEQUENCE_OF<CertSubscribeReq_certs_Item>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_CertSubscribeReq_certs_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.certs)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertSubscribeReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertSubscribeReq")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeReq,
        _eal_components_for_CertSubscribeReq,
        _rctl2_components_for_CertSubscribeReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => _validate_InvokeID(_el)?,
            "certs" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_CertSubscribeReq_certs_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certs")),
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
/// InvokeID  ::=  INTEGER (0..127)
/// ```
pub type InvokeID = INTEGER;

pub fn _decode_InvokeID(el: &X690Element) -> ASN1Result<InvokeID> {
    BER.decode_integer(&el)
}

pub fn _encode_InvokeID(value_: &InvokeID) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_InvokeID(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
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
impl TryFrom<&X690Element> for CertSubscribeRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertSubscribeRsp")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeRsp,
        _eal_components_for_CertSubscribeRsp,
        _rctl2_components_for_CertSubscribeRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<InvokeID> = None;
    let mut result_: OPTIONAL<CertSubscribeRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(_decode_InvokeID(_el)?),
            "result" => result_ = Some(_decode_CertSubscribeRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertSubscribeRsp {
        invokeID: invokeID_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertSubscribeRsp(value_: &CertSubscribeRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_InvokeID(&value_.invokeID)?);
    components_.push(_encode_CertSubscribeRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertSubscribeRsp(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertSubscribeRsp")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeRsp,
        _eal_components_for_CertSubscribeRsp,
        _rctl2_components_for_CertSubscribeRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => _validate_InvokeID(_el)?,
            "result" => _validate_CertSubscribeRsp_result(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertSubscribeOK"))
        }
    };
    let mut items: SEQUENCE_OF<CertSubscribeOK_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_CertSubscribeOK_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_CertSubscribeOK(value_: &CertSubscribeOK) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_CertSubscribeOK_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CertSubscribeOK(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_CertSubscribeOK_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertSubscribeOK")),
    }
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
    BER.decode_enumerated(&el)
}

pub fn _encode_CertStatus(value_: &CertStatus) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_CertStatus(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
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
    BER.decode_enumerated(&el)
}

pub fn _encode_CASP_CertStatusCode(value_: &CASP_CertStatusCode) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_CASP_CertStatusCode(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertSubscribeErr ::= SEQUENCE {
///   code       CASP-error,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for CertSubscribeErr {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertSubscribeErr")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeErr,
        _eal_components_for_CertSubscribeErr,
        _rctl2_components_for_CertSubscribeErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut code_: OPTIONAL<CASP_error> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "code" => code_ = Some(_decode_CASP_error(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertSubscribeErr {
        code: code_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertSubscribeErr(value_: &CertSubscribeErr) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_CASP_error(&value_.code)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertSubscribeErr(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertSubscribeErr")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeErr,
        _eal_components_for_CertSubscribeErr,
        _rctl2_components_for_CertSubscribeErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "code" => _validate_CASP_error(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for CertUnsubscribeReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUnsubscribeReq")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeReq,
        _eal_components_for_CertUnsubscribeReq,
        _rctl2_components_for_CertUnsubscribeReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<InvokeID> = None;
    let mut certs_: OPTIONAL<Vec<CertUnsubscribeReq_certs_Item>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(_decode_InvokeID(_el)?),
            "certs" => {
                certs_ = Some(|el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<CertUnsubscribeReq_certs_Item>,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certs")
                            )
                        }
                    };
                    let mut items: SEQUENCE_OF<CertUnsubscribeReq_certs_Item> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_CertUnsubscribeReq_certs_Item(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUnsubscribeReq {
        invokeID: invokeID_.unwrap(),
        certs: certs_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUnsubscribeReq(value_: &CertUnsubscribeReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_InvokeID(&value_.invokeID)?);
    components_.push(
        |value_: &SEQUENCE_OF<CertUnsubscribeReq_certs_Item>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_CertUnsubscribeReq_certs_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.certs)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUnsubscribeReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUnsubscribeReq")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeReq,
        _eal_components_for_CertUnsubscribeReq,
        _rctl2_components_for_CertUnsubscribeReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => _validate_InvokeID(_el)?,
            "certs" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_CertUnsubscribeReq_certs_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certs")),
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
/// CertUnsubscribeRsp ::= SEQUENCE {
///   invokeID     InvokeID,
///   result       CHOICE {
///     success       [0]  CertUnsubscribeOK,
///     failure       [1]  CertUnsubscribeErr,
///     ... },
///   ... }
/// ```
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
impl TryFrom<&X690Element> for CertUnsubscribeRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUnsubscribeRsp")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeRsp,
        _eal_components_for_CertUnsubscribeRsp,
        _rctl2_components_for_CertUnsubscribeRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<InvokeID> = None;
    let mut result_: OPTIONAL<CertUnsubscribeRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(_decode_InvokeID(_el)?),
            "result" => result_ = Some(_decode_CertUnsubscribeRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUnsubscribeRsp {
        invokeID: invokeID_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUnsubscribeRsp(value_: &CertUnsubscribeRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_InvokeID(&value_.invokeID)?);
    components_.push(_encode_CertUnsubscribeRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUnsubscribeRsp(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUnsubscribeRsp")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeRsp,
        _eal_components_for_CertUnsubscribeRsp,
        _rctl2_components_for_CertUnsubscribeRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => _validate_InvokeID(_el)?,
            "result" => _validate_CertUnsubscribeRsp_result(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUnsubscribeOK")
            )
        }
    };
    let mut items: SEQUENCE_OF<CertUnsubscribeOK_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_CertUnsubscribeOK_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_CertUnsubscribeOK(value_: &CertUnsubscribeOK) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_CertUnsubscribeOK_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CertUnsubscribeOK(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_CertUnsubscribeOK_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUnsubscribeOK")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUnsubscribeErr ::= SEQUENCE {
///   code         CASP-error,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for CertUnsubscribeErr {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUnsubscribeErr")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeErr,
        _eal_components_for_CertUnsubscribeErr,
        _rctl2_components_for_CertUnsubscribeErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut code_: OPTIONAL<CASP_error> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "code" => code_ = Some(_decode_CASP_error(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUnsubscribeErr {
        code: code_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUnsubscribeErr(value_: &CertUnsubscribeErr) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_CASP_error(&value_.code)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUnsubscribeErr(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUnsubscribeErr")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeErr,
        _eal_components_for_CertUnsubscribeErr,
        _rctl2_components_for_CertUnsubscribeErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "code" => _validate_CASP_error(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for CertReplaceReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReplaceReq"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceReq,
        _eal_components_for_CertReplaceReq,
        _rctl2_components_for_CertReplaceReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<InvokeID> = None;
    let mut certs_: OPTIONAL<Vec<CertReplaceReq_certs_Item>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(_decode_InvokeID(_el)?),
            "certs" => {
                certs_ = Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertReplaceReq_certs_Item>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "certs",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<CertReplaceReq_certs_Item> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_CertReplaceReq_certs_Item(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertReplaceReq {
        invokeID: invokeID_.unwrap(),
        certs: certs_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertReplaceReq(value_: &CertReplaceReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_InvokeID(&value_.invokeID)?);
    components_.push(
        |value_: &SEQUENCE_OF<CertReplaceReq_certs_Item>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_CertReplaceReq_certs_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.certs)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertReplaceReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReplaceReq"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceReq,
        _eal_components_for_CertReplaceReq,
        _rctl2_components_for_CertReplaceReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => _validate_InvokeID(_el)?,
            "certs" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_CertReplaceReq_certs_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certs")),
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
/// CertReplaceRsp ::= SEQUENCE {
///   invokeID     InvokeID,
///   result        CHOICE {
///     success       [0]  CertReplaceOK,
///     failure       [1]  CertReplaceErr,
///     ... },
///   ... }
/// ```
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
impl TryFrom<&X690Element> for CertReplaceRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReplaceRsp"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceRsp,
        _eal_components_for_CertReplaceRsp,
        _rctl2_components_for_CertReplaceRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<InvokeID> = None;
    let mut result_: OPTIONAL<CertReplaceRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(_decode_InvokeID(_el)?),
            "result" => result_ = Some(_decode_CertReplaceRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertReplaceRsp {
        invokeID: invokeID_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertReplaceRsp(value_: &CertReplaceRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_InvokeID(&value_.invokeID)?);
    components_.push(_encode_CertReplaceRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertReplaceRsp(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReplaceRsp"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceRsp,
        _eal_components_for_CertReplaceRsp,
        _rctl2_components_for_CertReplaceRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => _validate_InvokeID(_el)?,
            "result" => _validate_CertReplaceRsp_result(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReplaceOK")),
    };
    let mut items: SEQUENCE_OF<CertReplaceOK_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_CertReplaceOK_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_CertReplaceOK(value_: &CertReplaceOK) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_CertReplaceOK_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CertReplaceOK(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_CertReplaceOK_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReplaceOK")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertReplaceErr ::= SEQUENCE {
///   code          CASP-error,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for CertReplaceErr {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReplaceErr"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceErr,
        _eal_components_for_CertReplaceErr,
        _rctl2_components_for_CertReplaceErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut code_: OPTIONAL<CASP_error> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "code" => code_ = Some(_decode_CASP_error(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertReplaceErr {
        code: code_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertReplaceErr(value_: &CertReplaceErr) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_CASP_error(&value_.code)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertReplaceErr(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReplaceErr"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceErr,
        _eal_components_for_CertReplaceErr,
        _rctl2_components_for_CertReplaceErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "code" => _validate_CASP_error(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for CertUpdateReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUpdateReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateReq,
        _eal_components_for_CertUpdateReq,
        _rctl2_components_for_CertUpdateReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<InvokeID> = None;
    let mut certs_: OPTIONAL<Vec<CertUpdateReq_certs_Item>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(_decode_InvokeID(_el)?),
            "certs" => {
                certs_ = Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<CertUpdateReq_certs_Item>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "certs",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<CertUpdateReq_certs_Item> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_CertUpdateReq_certs_Item(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUpdateReq {
        invokeID: invokeID_.unwrap(),
        certs: certs_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUpdateReq(value_: &CertUpdateReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_InvokeID(&value_.invokeID)?);
    components_.push(
        |value_: &SEQUENCE_OF<CertUpdateReq_certs_Item>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_CertUpdateReq_certs_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.certs)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUpdateReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUpdateReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateReq,
        _eal_components_for_CertUpdateReq,
        _rctl2_components_for_CertUpdateReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => _validate_InvokeID(_el)?,
            "certs" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_CertUpdateReq_certs_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "certs")),
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
/// CertUpdateRsp ::= SEQUENCE {
///   invokeID      InvokeID,
///   result        CHOICE {
///     success       [0]  CertUpdateOK,
///     failure       [1]  CertUpdateErr,
///     ... },
///   ... }
/// ```
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
impl TryFrom<&X690Element> for CertUpdateRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUpdateRsp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateRsp,
        _eal_components_for_CertUpdateRsp,
        _rctl2_components_for_CertUpdateRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<InvokeID> = None;
    let mut result_: OPTIONAL<CertUpdateRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(_decode_InvokeID(_el)?),
            "result" => result_ = Some(_decode_CertUpdateRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUpdateRsp {
        invokeID: invokeID_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUpdateRsp(value_: &CertUpdateRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_InvokeID(&value_.invokeID)?);
    components_.push(_encode_CertUpdateRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUpdateRsp(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUpdateRsp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateRsp,
        _eal_components_for_CertUpdateRsp,
        _rctl2_components_for_CertUpdateRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => _validate_InvokeID(_el)?,
            "result" => _validate_CertUpdateRsp_result(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUpdateOK")),
    };
    let mut items: SEQUENCE_OF<CertUpdateOK_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_CertUpdateOK_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_CertUpdateOK(value_: &CertUpdateOK) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_CertUpdateOK_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CertUpdateOK(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_CertUpdateOK_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUpdateOK")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUpdateErr ::= SEQUENCE {
///   code          CASP-error,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for CertUpdateErr {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUpdateErr")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateErr,
        _eal_components_for_CertUpdateErr,
        _rctl2_components_for_CertUpdateErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut code_: OPTIONAL<CASP_error> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "code" => code_ = Some(_decode_CASP_error(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUpdateErr {
        code: code_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUpdateErr(value_: &CertUpdateErr) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_CASP_error(&value_.code)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUpdateErr(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUpdateErr")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateErr,
        _eal_components_for_CertUpdateErr,
        _rctl2_components_for_CertUpdateErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "code" => _validate_CASP_error(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for CAsubscribeAbort {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CAsubscribeAbort")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CAsubscribeAbort,
        _eal_components_for_CAsubscribeAbort,
        _rctl2_components_for_CAsubscribeAbort,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<InvokeID> = None;
    let mut reason_: OPTIONAL<CASP_error> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(_decode_InvokeID(_el)?),
            "reason" => reason_ = Some(_decode_CASP_error(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CAsubscribeAbort {
        invokeID: invokeID_.unwrap(),
        reason: reason_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CAsubscribeAbort(value_: &CAsubscribeAbort) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_InvokeID(&value_.invokeID)?);
    components_.push(_encode_CASP_error(&value_.reason)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CAsubscribeAbort(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CAsubscribeAbort")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CAsubscribeAbort,
        _eal_components_for_CAsubscribeAbort,
        _rctl2_components_for_CAsubscribeAbort,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => _validate_InvokeID(_el)?,
            "reason" => _validate_CASP_error(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    BER.decode_enumerated(&el)
}

pub fn _encode_CASP_error(value_: &CASP_error) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_CASP_error(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InitializationRej-diag ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type InitializationRej_diag = ENUMERATED;

pub const InitializationRej_diag_unsupportedVersion: InitializationRej_diag = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_InitializationRej_diag(el: &X690Element) -> ASN1Result<InitializationRej_diag> {
    BER.decode_enumerated(&el)
}

pub fn _encode_InitializationRej_diag(value_: &InitializationRej_diag) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_InitializationRej_diag(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
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
    BER.decode_enumerated(&el)
}

pub fn _encode_InitializationAbort_diag(
    value_: &InitializationAbort_diag,
) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_InitializationAbort_diag(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertSubscribeReq-certs-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertSubscribeReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertSubscribeReq-certs-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeReq_certs_Item,
        _eal_components_for_CertSubscribeReq_certs_Item,
        _rctl2_components_for_CertSubscribeReq_certs_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut subject_: OPTIONAL<Name> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subject" => subject_ = Some(_decode_Name(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertSubscribeReq_certs_Item {
        subject: subject_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertSubscribeReq_certs_Item(
    value_: &CertSubscribeReq_certs_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_Name(&value_.subject)?);
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertSubscribeReq_certs_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertSubscribeReq-certs-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeReq_certs_Item,
        _eal_components_for_CertSubscribeReq_certs_Item,
        _rctl2_components_for_CertSubscribeReq_certs_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subject" => _validate_Name(_el)?,
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for CertSubscribeRsp_result {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeRsp_result(el)
    }
}

pub fn _decode_CertSubscribeRsp_result(el: &X690Element) -> ASN1Result<CertSubscribeRsp_result> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CertSubscribeRsp_result::success(_decode_CertSubscribeOK(
            &el,
        )?)),
        (TagClass::CONTEXT, 1) => Ok(CertSubscribeRsp_result::failure(_decode_CertSubscribeErr(
            &el,
        )?)),
        _ => Ok(CertSubscribeRsp_result::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertSubscribeRsp_result(
    value_: &CertSubscribeRsp_result,
) -> ASN1Result<X690Element> {
    match value_ {
        CertSubscribeRsp_result::success(v) => |v_1: &CertSubscribeOK| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertSubscribeOK(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        CertSubscribeRsp_result::failure(v) => {
            |v_1: &CertSubscribeErr| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertSubscribeErr(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        CertSubscribeRsp_result::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertSubscribeRsp_result(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "success"));
            }
            Ok(_validate_CertSubscribeOK(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_CertSubscribeErr(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertSubscribeOK-Item-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertSubscribeOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertSubscribeOK-Item-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeOK_Item_ok,
        _eal_components_for_CertSubscribeOK_Item_ok,
        _rctl2_components_for_CertSubscribeOK_Item_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut cert_: OPTIONAL<Certificate> = None;
    let mut status_: OPTIONAL<CertStatus> = None;
    let mut revokeReason_: OPTIONAL<CRLReason> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "cert" => cert_ = Some(_decode_Certificate(_el)?),
            "status" => status_ = Some(_decode_CertStatus(_el)?),
            "revokeReason" => revokeReason_ = Some(_decode_CRLReason(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertSubscribeOK_Item_ok {
        cert: cert_.unwrap(),
        status: status_.unwrap(),
        revokeReason: revokeReason_,
        _unrecognized,
    })
}

pub fn _encode_CertSubscribeOK_Item_ok(
    value_: &CertSubscribeOK_Item_ok,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_Certificate(&value_.cert)?);
    components_.push(_encode_CertStatus(&value_.status)?);
    if let Some(v_) = &value_.revokeReason {
        components_.push(_encode_CRLReason(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertSubscribeOK_Item_ok(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertSubscribeOK-Item-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeOK_Item_ok,
        _eal_components_for_CertSubscribeOK_Item_ok,
        _rctl2_components_for_CertSubscribeOK_Item_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "cert" => _validate_Certificate(_el)?,
            "status" => _validate_CertStatus(_el)?,
            "revokeReason" => _validate_CRLReason(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertSubscribeOK-Item-not-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertSubscribeOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertSubscribeOK-Item-not-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeOK_Item_not_ok,
        _eal_components_for_CertSubscribeOK_Item_not_ok,
        _rctl2_components_for_CertSubscribeOK_Item_not_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut status_: OPTIONAL<CASP_CertStatusCode> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => status_ = Some(_decode_CASP_CertStatusCode(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertSubscribeOK_Item_not_ok {
        status: status_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertSubscribeOK_Item_not_ok(
    value_: &CertSubscribeOK_Item_not_ok,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_CASP_CertStatusCode(&value_.status)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertSubscribeOK_Item_not_ok(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertSubscribeOK-Item-not-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertSubscribeOK_Item_not_ok,
        _eal_components_for_CertSubscribeOK_Item_not_ok,
        _rctl2_components_for_CertSubscribeOK_Item_not_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => _validate_CASP_CertStatusCode(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for CertSubscribeOK_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeOK_Item(el)
    }
}

pub fn _decode_CertSubscribeOK_Item(el: &X690Element) -> ASN1Result<CertSubscribeOK_Item> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CertSubscribeOK_Item::ok(_decode_CertSubscribeOK_Item_ok(
            &el,
        )?)),
        (TagClass::CONTEXT, 1) => Ok(CertSubscribeOK_Item::not_ok(
            _decode_CertSubscribeOK_Item_not_ok(&el)?,
        )),
        _ => Ok(CertSubscribeOK_Item::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertSubscribeOK_Item(value_: &CertSubscribeOK_Item) -> ASN1Result<X690Element> {
    match value_ {
        CertSubscribeOK_Item::ok(v) => |v_1: &CertSubscribeOK_Item_ok| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertSubscribeOK_Item_ok(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        CertSubscribeOK_Item::not_ok(v) => {
            |v_1: &CertSubscribeOK_Item_not_ok| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertSubscribeOK_Item_not_ok(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        CertSubscribeOK_Item::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertSubscribeOK_Item(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ok"));
            }
            Ok(_validate_CertSubscribeOK_Item_ok(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "not-ok"));
            }
            Ok(_validate_CertSubscribeOK_Item_not_ok(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUnsubscribeReq-certs-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertUnsubscribeReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertUnsubscribeReq-certs-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeReq_certs_Item,
        _eal_components_for_CertUnsubscribeReq_certs_Item,
        _rctl2_components_for_CertUnsubscribeReq_certs_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut subject_: OPTIONAL<Name> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subject" => subject_ = Some(_decode_Name(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUnsubscribeReq_certs_Item {
        subject: subject_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUnsubscribeReq_certs_Item(
    value_: &CertUnsubscribeReq_certs_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_Name(&value_.subject)?);
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUnsubscribeReq_certs_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertUnsubscribeReq-certs-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeReq_certs_Item,
        _eal_components_for_CertUnsubscribeReq_certs_Item,
        _rctl2_components_for_CertUnsubscribeReq_certs_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subject" => _validate_Name(_el)?,
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for CertUnsubscribeRsp_result {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeRsp_result(el)
    }
}

pub fn _decode_CertUnsubscribeRsp_result(
    el: &X690Element,
) -> ASN1Result<CertUnsubscribeRsp_result> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CertUnsubscribeRsp_result::success(
            _decode_CertUnsubscribeOK(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CertUnsubscribeRsp_result::failure(
            _decode_CertUnsubscribeErr(&el)?,
        )),
        _ => Ok(CertUnsubscribeRsp_result::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertUnsubscribeRsp_result(
    value_: &CertUnsubscribeRsp_result,
) -> ASN1Result<X690Element> {
    match value_ {
        CertUnsubscribeRsp_result::success(v) => {
            |v_1: &CertUnsubscribeOK| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertUnsubscribeOK(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
        CertUnsubscribeRsp_result::failure(v) => {
            |v_1: &CertUnsubscribeErr| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertUnsubscribeErr(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        CertUnsubscribeRsp_result::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertUnsubscribeRsp_result(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "success"));
            }
            Ok(_validate_CertUnsubscribeOK(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_CertUnsubscribeErr(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUnsubscribeOK-Item-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertUnsubscribeOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertUnsubscribeOK-Item-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeOK_Item_ok,
        _eal_components_for_CertUnsubscribeOK_Item_ok,
        _rctl2_components_for_CertUnsubscribeOK_Item_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut subject_: OPTIONAL<Name> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subject" => subject_ = Some(_decode_Name(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUnsubscribeOK_Item_ok {
        subject: subject_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUnsubscribeOK_Item_ok(
    value_: &CertUnsubscribeOK_Item_ok,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_Name(&value_.subject)?);
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUnsubscribeOK_Item_ok(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertUnsubscribeOK-Item-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeOK_Item_ok,
        _eal_components_for_CertUnsubscribeOK_Item_ok,
        _rctl2_components_for_CertUnsubscribeOK_Item_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subject" => _validate_Name(_el)?,
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUnsubscribeOK-Item-not-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertUnsubscribeOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertUnsubscribeOK-Item-not-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeOK_Item_not_ok,
        _eal_components_for_CertUnsubscribeOK_Item_not_ok,
        _rctl2_components_for_CertUnsubscribeOK_Item_not_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut status_: OPTIONAL<CASP_CertStatusCode> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => status_ = Some(_decode_CASP_CertStatusCode(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUnsubscribeOK_Item_not_ok {
        status: status_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUnsubscribeOK_Item_not_ok(
    value_: &CertUnsubscribeOK_Item_not_ok,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_CASP_CertStatusCode(&value_.status)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUnsubscribeOK_Item_not_ok(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertUnsubscribeOK-Item-not-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUnsubscribeOK_Item_not_ok,
        _eal_components_for_CertUnsubscribeOK_Item_not_ok,
        _rctl2_components_for_CertUnsubscribeOK_Item_not_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => _validate_CASP_CertStatusCode(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for CertUnsubscribeOK_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeOK_Item(el)
    }
}

pub fn _decode_CertUnsubscribeOK_Item(el: &X690Element) -> ASN1Result<CertUnsubscribeOK_Item> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CertUnsubscribeOK_Item::ok(
            _decode_CertUnsubscribeOK_Item_ok(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CertUnsubscribeOK_Item::not_ok(
            _decode_CertUnsubscribeOK_Item_not_ok(&el)?,
        )),
        _ => Ok(CertUnsubscribeOK_Item::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertUnsubscribeOK_Item(value_: &CertUnsubscribeOK_Item) -> ASN1Result<X690Element> {
    match value_ {
        CertUnsubscribeOK_Item::ok(v) => {
            |v_1: &CertUnsubscribeOK_Item_ok| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertUnsubscribeOK_Item_ok(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
        CertUnsubscribeOK_Item::not_ok(v) => {
            |v_1: &CertUnsubscribeOK_Item_not_ok| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertUnsubscribeOK_Item_not_ok(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        CertUnsubscribeOK_Item::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertUnsubscribeOK_Item(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ok"));
            }
            Ok(_validate_CertUnsubscribeOK_Item_ok(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "not-ok"));
            }
            Ok(_validate_CertUnsubscribeOK_Item_not_ok(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertReplaceReq-certs-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertReplaceReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertReplaceReq-certs-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceReq_certs_Item,
        _eal_components_for_CertReplaceReq_certs_Item,
        _rctl2_components_for_CertReplaceReq_certs_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut old_: OPTIONAL<CertificateSerialNumber> = None;
    let mut new_: OPTIONAL<Certificate> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "old" => old_ = Some(_decode_CertificateSerialNumber(_el)?),
            "new" => new_ = Some(_decode_Certificate(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertReplaceReq_certs_Item {
        old: old_.unwrap(),
        new: new_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertReplaceReq_certs_Item(
    value_: &CertReplaceReq_certs_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_CertificateSerialNumber(&value_.old)?);
    components_.push(_encode_Certificate(&value_.new)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertReplaceReq_certs_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertReplaceReq-certs-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceReq_certs_Item,
        _eal_components_for_CertReplaceReq_certs_Item,
        _rctl2_components_for_CertReplaceReq_certs_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "old" => _validate_CertificateSerialNumber(_el)?,
            "new" => _validate_Certificate(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for CertReplaceRsp_result {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceRsp_result(el)
    }
}

pub fn _decode_CertReplaceRsp_result(el: &X690Element) -> ASN1Result<CertReplaceRsp_result> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CertReplaceRsp_result::success(_decode_CertReplaceOK(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(CertReplaceRsp_result::failure(_decode_CertReplaceErr(&el)?)),
        _ => Ok(CertReplaceRsp_result::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertReplaceRsp_result(value_: &CertReplaceRsp_result) -> ASN1Result<X690Element> {
    match value_ {
        CertReplaceRsp_result::success(v) => |v_1: &CertReplaceOK| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertReplaceOK(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        CertReplaceRsp_result::failure(v) => |v_1: &CertReplaceErr| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertReplaceErr(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
        CertReplaceRsp_result::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertReplaceRsp_result(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "success"));
            }
            Ok(_validate_CertReplaceOK(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_CertReplaceErr(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertReplaceOK-Item-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertReplaceOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReplaceOK-Item-ok")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceOK_Item_ok,
        _eal_components_for_CertReplaceOK_Item_ok,
        _rctl2_components_for_CertReplaceOK_Item_ok,
    )
    .into_iter();
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
    Ok(CertReplaceOK_Item_ok {
        issuer: issuer_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertReplaceOK_Item_ok(value_: &CertReplaceOK_Item_ok) -> ASN1Result<X690Element> {
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

pub fn _validate_CertReplaceOK_Item_ok(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReplaceOK-Item-ok")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceOK_Item_ok,
        _eal_components_for_CertReplaceOK_Item_ok,
        _rctl2_components_for_CertReplaceOK_Item_ok,
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
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertReplaceOK-Item-not-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertReplaceOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertReplaceOK-Item-not-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceOK_Item_not_ok,
        _eal_components_for_CertReplaceOK_Item_not_ok,
        _rctl2_components_for_CertReplaceOK_Item_not_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut status_: OPTIONAL<CASP_CertStatusCode> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => status_ = Some(_decode_CASP_CertStatusCode(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertReplaceOK_Item_not_ok {
        status: status_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertReplaceOK_Item_not_ok(
    value_: &CertReplaceOK_Item_not_ok,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_CASP_CertStatusCode(&value_.status)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertReplaceOK_Item_not_ok(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertReplaceOK-Item-not-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReplaceOK_Item_not_ok,
        _eal_components_for_CertReplaceOK_Item_not_ok,
        _rctl2_components_for_CertReplaceOK_Item_not_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => _validate_CASP_CertStatusCode(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for CertReplaceOK_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceOK_Item(el)
    }
}

pub fn _decode_CertReplaceOK_Item(el: &X690Element) -> ASN1Result<CertReplaceOK_Item> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CertReplaceOK_Item::ok(_decode_CertReplaceOK_Item_ok(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(CertReplaceOK_Item::not_ok(
            _decode_CertReplaceOK_Item_not_ok(&el)?,
        )),
        _ => Ok(CertReplaceOK_Item::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertReplaceOK_Item(value_: &CertReplaceOK_Item) -> ASN1Result<X690Element> {
    match value_ {
        CertReplaceOK_Item::ok(v) => |v_1: &CertReplaceOK_Item_ok| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertReplaceOK_Item_ok(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        CertReplaceOK_Item::not_ok(v) => {
            |v_1: &CertReplaceOK_Item_not_ok| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertReplaceOK_Item_not_ok(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        CertReplaceOK_Item::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertReplaceOK_Item(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ok"));
            }
            Ok(_validate_CertReplaceOK_Item_ok(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "not-ok"));
            }
            Ok(_validate_CertReplaceOK_Item_not_ok(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUpdateReq-certs-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertUpdateReq_certs_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertUpdateReq-certs-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateReq_certs_Item,
        _eal_components_for_CertUpdateReq_certs_Item,
        _rctl2_components_for_CertUpdateReq_certs_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut subject_: OPTIONAL<Name> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut certStatus_: OPTIONAL<CertStatus> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subject" => subject_ = Some(_decode_Name(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            "certStatus" => certStatus_ = Some(_decode_CertStatus(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUpdateReq_certs_Item {
        subject: subject_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
        certStatus: certStatus_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUpdateReq_certs_Item(
    value_: &CertUpdateReq_certs_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_Name(&value_.subject)?);
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    components_.push(_encode_CertStatus(&value_.certStatus)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUpdateReq_certs_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertUpdateReq-certs-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateReq_certs_Item,
        _eal_components_for_CertUpdateReq_certs_Item,
        _rctl2_components_for_CertUpdateReq_certs_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subject" => _validate_Name(_el)?,
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            "certStatus" => _validate_CertStatus(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for CertUpdateRsp_result {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateRsp_result(el)
    }
}

pub fn _decode_CertUpdateRsp_result(el: &X690Element) -> ASN1Result<CertUpdateRsp_result> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CertUpdateRsp_result::success(_decode_CertUpdateOK(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(CertUpdateRsp_result::failure(_decode_CertUpdateErr(&el)?)),
        _ => Ok(CertUpdateRsp_result::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertUpdateRsp_result(value_: &CertUpdateRsp_result) -> ASN1Result<X690Element> {
    match value_ {
        CertUpdateRsp_result::success(v) => |v_1: &CertUpdateOK| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertUpdateOK(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        CertUpdateRsp_result::failure(v) => |v_1: &CertUpdateErr| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertUpdateErr(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
        CertUpdateRsp_result::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertUpdateRsp_result(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "success"));
            }
            Ok(_validate_CertUpdateOK(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_CertUpdateErr(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUpdateOK-Item-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertUpdateOK_Item_ok {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUpdateOK-Item-ok")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateOK_Item_ok,
        _eal_components_for_CertUpdateOK_Item_ok,
        _rctl2_components_for_CertUpdateOK_Item_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut subject_: OPTIONAL<Name> = None;
    let mut serialNumber_: OPTIONAL<CertificateSerialNumber> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subject" => subject_ = Some(_decode_Name(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_CertificateSerialNumber(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUpdateOK_Item_ok {
        subject: subject_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUpdateOK_Item_ok(value_: &CertUpdateOK_Item_ok) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_Name(&value_.subject)?);
    components_.push(_encode_CertificateSerialNumber(&value_.serialNumber)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUpdateOK_Item_ok(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertUpdateOK-Item-ok")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateOK_Item_ok,
        _eal_components_for_CertUpdateOK_Item_ok,
        _rctl2_components_for_CertUpdateOK_Item_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "subject" => _validate_Name(_el)?,
            "serialNumber" => _validate_CertificateSerialNumber(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUpdateOK-Item-not-ok ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
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
impl TryFrom<&X690Element> for CertUpdateOK_Item_not_ok {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertUpdateOK-Item-not-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateOK_Item_not_ok,
        _eal_components_for_CertUpdateOK_Item_not_ok,
        _rctl2_components_for_CertUpdateOK_Item_not_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut status_: OPTIONAL<CASP_CertStatusCode> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => status_ = Some(_decode_CASP_CertStatusCode(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUpdateOK_Item_not_ok {
        status: status_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUpdateOK_Item_not_ok(
    value_: &CertUpdateOK_Item_not_ok,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_CASP_CertStatusCode(&value_.status)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertUpdateOK_Item_not_ok(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "CertUpdateOK-Item-not-ok",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertUpdateOK_Item_not_ok,
        _eal_components_for_CertUpdateOK_Item_not_ok,
        _rctl2_components_for_CertUpdateOK_Item_not_ok,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => _validate_CASP_CertStatusCode(_el)?,
            _ => (),
        }
    }
    Ok(())
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

impl TryFrom<&X690Element> for CertUpdateOK_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateOK_Item(el)
    }
}

pub fn _decode_CertUpdateOK_Item(el: &X690Element) -> ASN1Result<CertUpdateOK_Item> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CertUpdateOK_Item::ok(_decode_CertUpdateOK_Item_ok(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(CertUpdateOK_Item::not_ok(_decode_CertUpdateOK_Item_not_ok(
            &el,
        )?)),
        _ => Ok(CertUpdateOK_Item::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertUpdateOK_Item(value_: &CertUpdateOK_Item) -> ASN1Result<X690Element> {
    match value_ {
        CertUpdateOK_Item::ok(v) => |v_1: &CertUpdateOK_Item_ok| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_CertUpdateOK_Item_ok(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        CertUpdateOK_Item::not_ok(v) => {
            |v_1: &CertUpdateOK_Item_not_ok| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_CertUpdateOK_Item_not_ok(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
        CertUpdateOK_Item::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertUpdateOK_Item(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ok"));
            }
            Ok(_validate_CertUpdateOK_Item_ok(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "not-ok"));
            }
            Ok(_validate_CertUpdateOK_Item_not_ok(&el)?)
        }(&el),
        _ => Ok(()),
    }
}
