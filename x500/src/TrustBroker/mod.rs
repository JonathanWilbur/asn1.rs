#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # TrustBroker
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `TrustBroker`.
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
/// TBprot  ::=  CHOICE {
///   initReq     [0] InitializationReq,
///   initAcc     [1] InitializationAcc,
///   initRej     [2] InitializationRej,
///   initAbt     [3] InitializationAbort,
///   tBrequest   [4] TBrequest,
///   tBresponse  [5] TBresponse,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum TBprot {
    initReq(InitializationReq),
    initAcc(InitializationAcc),
    initRej(InitializationRej),
    initAbt(InitializationAbort),
    tBrequest(TBrequest),
    tBresponse(TBresponse),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for TBprot {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TBprot(el)
    }
}

pub fn _decode_TBprot(el: &X690Element) -> ASN1Result<TBprot> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(TBprot::initReq(_decode_InitializationReq(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(TBprot::initAcc(_decode_InitializationAcc(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(TBprot::initRej(_decode_InitializationRej(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(TBprot::initAbt(_decode_InitializationAbort(&el)?)),
        (TagClass::CONTEXT, 4) => Ok(TBprot::tBrequest(
            |el: &X690Element| -> ASN1Result<TBrequest> { Ok(_decode_TBrequest(&el.inner()?)?) }(
                &el,
            )?,
        )),
        (TagClass::CONTEXT, 5) => Ok(TBprot::tBresponse(
            |el: &X690Element| -> ASN1Result<TBresponse> { Ok(_decode_TBresponse(&el.inner()?)?) }(
                &el,
            )?,
        )),
        _ => Ok(TBprot::_unrecognized(el.clone())),
    }
}

pub fn _encode_TBprot(value_: &TBprot) -> ASN1Result<X690Element> {
    match value_ {
        TBprot::initReq(v) => |v_1: &InitializationReq| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_InitializationReq(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        TBprot::initAcc(v) => |v_1: &InitializationAcc| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_InitializationAcc(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
        TBprot::initRej(v) => |v_1: &InitializationRej| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_InitializationRej(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v),
        TBprot::initAbt(v) => |v_1: &InitializationAbort| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_InitializationAbort(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v),
        TBprot::tBrequest(v) => |v_1: &TBrequest| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::from_explicit(&_encode_TBrequest(&v_1)?),
            ))
        }(&v),
        TBprot::tBresponse(v) => |v_1: &TBresponse| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 5),
                X690Value::from_explicit(&_encode_TBresponse(&v_1)?),
            ))
        }(&v),
        TBprot::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_TBprot(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "initReq"));
            }
            Ok(_validate_InitializationReq(&el)?)
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
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "tBrequest"));
            }
            Ok(_validate_TBrequest(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 5) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "tBresponse"));
            }
            Ok(_validate_TBresponse(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InitializationReq ::= SEQUENCE {
///   version    Version,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct InitializationReq {
    pub version: Version,
    pub _unrecognized: Vec<X690Element>,
}
impl InitializationReq {
    pub fn new(version: Version, _unrecognized: Vec<X690Element>) -> Self {
        InitializationReq {
            version,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for InitializationReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_InitializationReq(el)
    }
}

pub const _rctl1_components_for_InitializationReq: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "version",
    false,
    TagSelector::tag((TagClass::UNIVERSAL, 3)),
    None,
    None,
)];

pub const _rctl2_components_for_InitializationReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_InitializationReq: &[ComponentSpec; 0] = &[];

pub fn _decode_InitializationReq(el: &X690Element) -> ASN1Result<InitializationReq> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitializationReq")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InitializationReq,
        _eal_components_for_InitializationReq,
        _rctl2_components_for_InitializationReq,
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
    Ok(InitializationReq {
        version: version_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_InitializationReq(value_: &InitializationReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_Version(&value_.version)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_InitializationReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "InitializationReq")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_InitializationReq,
        _eal_components_for_InitializationReq,
        _rctl2_components_for_InitializationReq,
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
///     unsupportedVersions     (0),
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
/// TBrequest  ::=  CHOICE {
///   caCert      [0] PKCertIdentifier,
///   subjectCert [1] PKCertIdentifier,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum TBrequest {
    caCert(PKCertIdentifier),
    subjectCert(PKCertIdentifier),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for TBrequest {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TBrequest(el)
    }
}

pub fn _decode_TBrequest(el: &X690Element) -> ASN1Result<TBrequest> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(TBrequest::caCert(
            |el: &X690Element| -> ASN1Result<PKCertIdentifier> {
                Ok(_decode_PKCertIdentifier(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(TBrequest::subjectCert(|el: &X690Element| -> ASN1Result<
            PKCertIdentifier,
        > {
            Ok(_decode_PKCertIdentifier(&el.inner()?)?)
        }(&el)?)),
        _ => Ok(TBrequest::_unrecognized(el.clone())),
    }
}

pub fn _encode_TBrequest(value_: &TBrequest) -> ASN1Result<X690Element> {
    match value_ {
        TBrequest::caCert(v) => |v_1: &PKCertIdentifier| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_PKCertIdentifier(&v_1)?),
            ))
        }(&v),
        TBrequest::subjectCert(v) => |v_1: &PKCertIdentifier| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_PKCertIdentifier(&v_1)?),
            ))
        }(&v),
        TBrequest::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_TBrequest(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "caCert"));
            }
            Ok(_validate_PKCertIdentifier(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "subjectCert")
                );
            }
            Ok(_validate_PKCertIdentifier(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBresponse  ::=  CHOICE {
///   success [0]  TBOK,
///   failure [1]  TBerror,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum TBresponse {
    success(TBOK),
    failure(TBerror),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for TBresponse {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TBresponse(el)
    }
}

pub fn _decode_TBresponse(el: &X690Element) -> ASN1Result<TBresponse> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(TBresponse::success(_decode_TBOK(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(TBresponse::failure(_decode_TBerror(&el)?)),
        _ => Ok(TBresponse::_unrecognized(el.clone())),
    }
}

pub fn _encode_TBresponse(value_: &TBresponse) -> ASN1Result<X690Element> {
    match value_ {
        TBresponse::success(v) => |v_1: &TBOK| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_TBOK(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        TBresponse::failure(v) => |v_1: &TBerror| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_TBerror(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
        TBresponse::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_TBresponse(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "success"));
            }
            Ok(_validate_TBOK(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_TBerror(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBOK ::= SEQUENCE {
///   levelOfAssurance  [0]  INTEGER (0..100),
///   confidenceLevel   [1]  INTEGER (0..100),
///   validationTime    [2]  UTCTime,
///   info              [3]  UTF8String  OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct TBOK {
    pub levelOfAssurance: INTEGER,
    pub confidenceLevel: INTEGER,
    pub validationTime: UTCTime,
    pub info: OPTIONAL<UTF8String>,
    pub _unrecognized: Vec<X690Element>,
}
impl TBOK {
    pub fn new(
        levelOfAssurance: INTEGER,
        confidenceLevel: INTEGER,
        validationTime: UTCTime,
        info: OPTIONAL<UTF8String>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TBOK {
            levelOfAssurance,
            confidenceLevel,
            validationTime,
            info,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for TBOK {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TBOK(el)
    }
}

pub const _rctl1_components_for_TBOK: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "levelOfAssurance",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "confidenceLevel",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "validationTime",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "info",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TBOK: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TBOK: &[ComponentSpec; 0] = &[];

pub fn _decode_TBOK(el: &X690Element) -> ASN1Result<TBOK> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TBOK")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBOK,
        _eal_components_for_TBOK,
        _rctl2_components_for_TBOK,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut levelOfAssurance_: OPTIONAL<INTEGER> = None;
    let mut confidenceLevel_: OPTIONAL<INTEGER> = None;
    let mut validationTime_: OPTIONAL<UTCTime> = None;
    let mut info_: OPTIONAL<UTF8String> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "levelOfAssurance" => levelOfAssurance_ = Some(BER.decode_integer(_el)?),
            "confidenceLevel" => confidenceLevel_ = Some(BER.decode_integer(_el)?),
            "validationTime" => validationTime_ = Some(BER.decode_utc_time(_el)?),
            "info" => info_ = Some(BER.decode_utf8_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TBOK {
        levelOfAssurance: levelOfAssurance_.unwrap(),
        confidenceLevel: confidenceLevel_.unwrap(),
        validationTime: validationTime_.unwrap(),
        info: info_,
        _unrecognized,
    })
}

pub fn _encode_TBOK(value_: &TBOK) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
        let mut el_1 = BER.encode_integer(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_.levelOfAssurance)?);
    components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
        let mut el_1 = BER.encode_integer(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 1;
        Ok(el_1)
    }(&value_.confidenceLevel)?);
    components_.push(|v_1: &UTCTime| -> ASN1Result<X690Element> {
        let mut el_1 = BER.encode_utc_time(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 2;
        Ok(el_1)
    }(&value_.validationTime)?);
    if let Some(v_) = &value_.info {
        components_.push(|v_1: &UTF8String| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_utf8_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
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

pub fn _validate_TBOK(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TBOK")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBOK,
        _eal_components_for_TBOK,
        _rctl2_components_for_TBOK,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "levelOfAssurance" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "levelOfAssurance",
                    ));
                }
                Ok(BER.validate_integer(&el)?)
            }(_el)?,
            "confidenceLevel" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "confidenceLevel",
                    ));
                }
                Ok(BER.validate_integer(&el)?)
            }(_el)?,
            "validationTime" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "validationTime")
                    );
                }
                Ok(BER.validate_utc_time(&el)?)
            }(_el)?,
            "info" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "info"));
                }
                Ok(BER.validate_utf8_string(&el)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TBerror ::= SEQUENCE {
///   code        ENUMERATED {
///     caCertInvalid        (1),
///     unknownCert          (2),
///     unknownCertStatus    (3),
///     subjectCertRevoked   (4),
///     incorrectCert        (5),
///     contractExpired      (6),
///     pathValidationFailed (7),
///     timeOut              (8),
///     other                (99),
///     ... },
///   diagnostic  UTF8String OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct TBerror {
    pub code: TBerror_code,
    pub diagnostic: OPTIONAL<UTF8String>,
    pub _unrecognized: Vec<X690Element>,
}
impl TBerror {
    pub fn new(
        code: TBerror_code,
        diagnostic: OPTIONAL<UTF8String>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TBerror {
            code,
            diagnostic,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for TBerror {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TBerror(el)
    }
}

pub const _rctl1_components_for_TBerror: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "code",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "diagnostic",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 12)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TBerror: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TBerror: &[ComponentSpec; 0] = &[];

pub fn _decode_TBerror(el: &X690Element) -> ASN1Result<TBerror> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TBerror")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBerror,
        _eal_components_for_TBerror,
        _rctl2_components_for_TBerror,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut code_: OPTIONAL<TBerror_code> = None;
    let mut diagnostic_: OPTIONAL<UTF8String> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "code" => code_ = Some(_decode_TBerror_code(_el)?),
            "diagnostic" => diagnostic_ = Some(BER.decode_utf8_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TBerror {
        code: code_.unwrap(),
        diagnostic: diagnostic_,
        _unrecognized,
    })
}

pub fn _encode_TBerror(value_: &TBerror) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_TBerror_code(&value_.code)?);
    if let Some(v_) = &value_.diagnostic {
        components_.push(BER.encode_utf8_string(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_TBerror(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TBerror")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TBerror,
        _eal_components_for_TBerror,
        _rctl2_components_for_TBerror,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "code" => _validate_TBerror_code(_el)?,
            "diagnostic" => BER.validate_utf8_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InitializationRej-diag ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type InitializationRej_diag = ENUMERATED;

pub const InitializationRej_diag_unsupportedVersions: InitializationRej_diag = 0; /* LONG_NAMED_ENUMERATED_VALUE */

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
/// TBerror-code ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type TBerror_code = ENUMERATED;

pub const TBerror_code_caCertInvalid: TBerror_code = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const TBerror_code_unknownCert: TBerror_code = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const TBerror_code_unknownCertStatus: TBerror_code = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const TBerror_code_subjectCertRevoked: TBerror_code = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const TBerror_code_incorrectCert: TBerror_code = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const TBerror_code_contractExpired: TBerror_code = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const TBerror_code_pathValidationFailed: TBerror_code = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const TBerror_code_timeOut: TBerror_code = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub const TBerror_code_other: TBerror_code = 99; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_TBerror_code(el: &X690Element) -> ASN1Result<TBerror_code> {
    BER.decode_enumerated(&el)
}

pub fn _encode_TBerror_code(value_: &TBerror_code) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_TBerror_code(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}
