#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PkiPMIProtocolSpecifications
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PkiPMIProtocolSpecifications`.
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
use crate::AuthenticationFramework::*;
use crate::CertificateExtensions::*;
use crate::InformationFramework::*;
use crate::PkiPmiWrapper::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// AvlPduSet WRAPPED-PDU ::= {
///   certReq |
///   certRsp |
///   addAvlReq |
///   addAvlRsp |
///   replaceAvlReq |
///   replaceAvlRsp |
///   deleteAvlReq |
///   deleteAvlRsp |
///   rejectAVL |
///   certSubscribeReq |
///   certSubscribeRsp |
///   certUnsubscribeReq |
///   certUnsubscribeRsp |
///   certReplaceReq |
///   certReplaceRsp |
///   rejectCAsubscribe,
///   ... }
/// ```
///
///
pub fn AvlPduSet() -> Vec<WRAPPED_PDU> {
    Vec::from([
        certReq(),
        certRsp(),
        addAvlReq(),
        addAvlRsp(),
        replaceAvlReq(),
        replaceAvlRsp(),
        deleteAvlReq(),
        deleteAvlRsp(),
        rejectAVL(),
        certSubscribeReq(),
        certSubscribeRsp(),
        certUnsubscribeReq(),
        certUnsubscribeRsp(),
        certReplaceReq(),
        certReplaceRsp(),
        rejectCAsubscribe(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AVMPcommonComponents ::= SEQUENCE {
///   version    AVMPversion DEFAULT v1,
///   timeStamp  GeneralizedTime,
///   sequence   AVMPsequence,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct AVMPcommonComponents {
    pub version: OPTIONAL<AVMPversion>,
    pub timeStamp: GeneralizedTime,
    pub sequence: AVMPsequence,
    pub _unrecognized: Vec<X690Element>,
}
impl AVMPcommonComponents {
    pub fn new(
        version: OPTIONAL<AVMPversion>,
        timeStamp: GeneralizedTime,
        sequence: AVMPsequence,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AVMPcommonComponents {
            version,
            timeStamp,
            sequence,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> AVMPversion {
        AVMPversion_v1
    }
}
impl TryFrom<&X690Element> for AVMPcommonComponents {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AVMPcommonComponents(el)
    }
}

pub const _rctl1_components_for_AVMPcommonComponents: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStamp",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AVMPcommonComponents: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AVMPcommonComponents: &[ComponentSpec; 0] = &[];

pub fn _decode_AVMPcommonComponents(el: &X690Element) -> ASN1Result<AVMPcommonComponents> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AVMPcommonComponents")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AVMPcommonComponents,
        _eal_components_for_AVMPcommonComponents,
        _rctl2_components_for_AVMPcommonComponents,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AVMPversion> = None;
    let mut timeStamp_: OPTIONAL<GeneralizedTime> = None;
    let mut sequence_: OPTIONAL<AVMPsequence> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AVMPversion(_el)?),
            "timeStamp" => timeStamp_ = Some(BER.decode_generalized_time(_el)?),
            "sequence" => sequence_ = Some(_decode_AVMPsequence(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AVMPcommonComponents {
        version: version_,
        timeStamp: timeStamp_.unwrap(),
        sequence: sequence_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AVMPcommonComponents(value_: &AVMPcommonComponents) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.version {
        if *v_ != AVMPcommonComponents::_default_value_for_version() {
            components_.push(_encode_AVMPversion(&v_)?);
        }
    }
    components_.push(BER.encode_generalized_time(&value_.timeStamp)?);
    components_.push(_encode_AVMPsequence(&value_.sequence)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AVMPcommonComponents(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AVMPcommonComponents")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AVMPcommonComponents,
        _eal_components_for_AVMPcommonComponents,
        _rctl2_components_for_AVMPcommonComponents,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AVMPversion(_el)?,
            "timeStamp" => BER.validate_generalized_time(_el)?,
            "sequence" => _validate_AVMPsequence(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AVMPversion  ::=  ENUMERATED { v1(1), v2(2), v3(3), ... }
/// ```
pub type AVMPversion = ENUMERATED;

pub const AVMPversion_v1: AVMPversion = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMPversion_v2: AVMPversion = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMPversion_v3: AVMPversion = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_AVMPversion(el: &X690Element) -> ASN1Result<AVMPversion> {
    BER.decode_enumerated(&el)
}

pub fn _encode_AVMPversion(value_: &AVMPversion) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_AVMPversion(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AVMPsequence  ::=  INTEGER (1..MAX)
/// ```
pub type AVMPsequence = INTEGER;

pub fn _decode_AVMPsequence(el: &X690Element) -> ASN1Result<AVMPsequence> {
    BER.decode_integer(&el)
}

pub fn _encode_AVMPsequence(value_: &AVMPsequence) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_AVMPsequence(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certReq WRAPPED-PDU ::= {
///                 CertReq
///   IDENTIFIED BY id-certReq }
/// ```
///
///
pub fn certReq() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_certReq(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod certReq {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertReq; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertReq(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertReq(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertReq(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertReq ::= SEQUENCE {
///   COMPONENTS OF AVMPcommonComponents,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertReq {
    pub version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
    pub timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
    pub sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
    pub _unrecognized: Vec<X690Element>,
}
impl CertReq {
    pub fn new(
        version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
        timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
        sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertReq {
            version,
            timeStamp,
            sequence,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> AVMPversion {
        AVMPversion_v1
    }
}
impl TryFrom<&X690Element> for CertReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertReq(el)
    }
}

pub const _rctl1_components_for_CertReq: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStamp",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CertReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CertReq: &[ComponentSpec; 0] = &[];

pub fn _decode_CertReq(el: &X690Element) -> ASN1Result<CertReq> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReq,
        _eal_components_for_CertReq,
        _rctl2_components_for_CertReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AVMPversion> = None;
    let mut timeStamp_: OPTIONAL<GeneralizedTime> = None;
    let mut sequence_: OPTIONAL<AVMPsequence> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AVMPversion(_el)?),
            "timeStamp" => timeStamp_ = Some(BER.decode_generalized_time(_el)?),
            "sequence" => sequence_ = Some(_decode_AVMPsequence(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertReq {
        version: version_,
        timeStamp: timeStamp_.unwrap(),
        sequence: sequence_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertReq(value_: &CertReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.version {
        if *v_ != CertReq::_default_value_for_version() {
            components_.push(_encode_AVMPversion(&v_)?);
        }
    }
    components_.push(BER.encode_generalized_time(&value_.timeStamp)?);
    components_.push(_encode_AVMPsequence(&value_.sequence)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertReq,
        _eal_components_for_CertReq,
        _rctl2_components_for_CertReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AVMPversion(_el)?,
            "timeStamp" => BER.validate_generalized_time(_el)?,
            "sequence" => _validate_AVMPsequence(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certRsp WRAPPED-PDU ::= {
///                 CertRsp
///   IDENTIFIED BY id-certRsp }
/// ```
///
///
pub fn certRsp() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_certRsp(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod certRsp {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertRsp; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertRsp(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertRsp(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertRsp(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertRsp ::= SEQUENCE {
///   COMPONENTS OF AVMPcommonComponents,
///   result        CHOICE {
///     success       [0]  CertOK,
///     failure       [1]  CertErr,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertRsp {
    pub version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
    pub timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
    pub sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
    pub result: CertRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl CertRsp {
    pub fn new(
        version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
        timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
        sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
        result: CertRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertRsp {
            version,
            timeStamp,
            sequence,
            result,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> AVMPversion {
        AVMPversion_v1
    }
}
impl TryFrom<&X690Element> for CertRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertRsp(el)
    }
}

pub const _rctl1_components_for_CertRsp: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStamp",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertRsp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertRsp,
        _eal_components_for_CertRsp,
        _rctl2_components_for_CertRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AVMPversion> = None;
    let mut timeStamp_: OPTIONAL<GeneralizedTime> = None;
    let mut sequence_: OPTIONAL<AVMPsequence> = None;
    let mut result_: OPTIONAL<CertRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AVMPversion(_el)?),
            "timeStamp" => timeStamp_ = Some(BER.decode_generalized_time(_el)?),
            "sequence" => sequence_ = Some(_decode_AVMPsequence(_el)?),
            "result" => result_ = Some(_decode_CertRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertRsp {
        version: version_,
        timeStamp: timeStamp_.unwrap(),
        sequence: sequence_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertRsp(value_: &CertRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    if let Some(v_) = &value_.version {
        if *v_ != CertRsp::_default_value_for_version() {
            components_.push(_encode_AVMPversion(&v_)?);
        }
    }
    components_.push(BER.encode_generalized_time(&value_.timeStamp)?);
    components_.push(_encode_AVMPsequence(&value_.sequence)?);
    components_.push(_encode_CertRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertRsp(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertRsp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertRsp,
        _eal_components_for_CertRsp,
        _rctl2_components_for_CertRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AVMPversion(_el)?,
            "timeStamp" => BER.validate_generalized_time(_el)?,
            "sequence" => _validate_AVMPsequence(_el)?,
            "result" => _validate_CertRsp_result(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertOK ::= SEQUENCE {
///   dhCert  Certificate,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for CertOK {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertOK")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertOK,
        _eal_components_for_CertOK,
        _rctl2_components_for_CertOK,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut dhCert_: OPTIONAL<Certificate> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "dhCert" => dhCert_ = Some(_decode_Certificate(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertOK {
        dhCert: dhCert_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertOK(value_: &CertOK) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_Certificate(&value_.dhCert)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertOK(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertOK")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertOK,
        _eal_components_for_CertOK,
        _rctl2_components_for_CertOK,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "dhCert" => _validate_Certificate(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertErr ::= SEQUENCE {
///   notOK  CHOICE {
///     wrErr   [0]  PkiWaError,
///     avmpErr [1]  AVMP-error,
///     ... },
///   note   Notifications OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertErr {
    pub notOK: CertErr_notOK,
    pub note: OPTIONAL<Notifications>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertErr {
    pub fn new(
        notOK: CertErr_notOK,
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
impl TryFrom<&X690Element> for CertErr {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertErr(el)
    }
}

pub const _rctl1_components_for_CertErr: &[ComponentSpec; 2] = &[
    ComponentSpec::new("notOK", false, TagSelector::any, None, None),
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertErr")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertErr,
        _eal_components_for_CertErr,
        _rctl2_components_for_CertErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut notOK_: OPTIONAL<CertErr_notOK> = None;
    let mut note_: OPTIONAL<Notifications> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notOK" => notOK_ = Some(_decode_CertErr_notOK(_el)?),
            "note" => note_ = Some(_decode_Notifications(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertErr {
        notOK: notOK_.unwrap(),
        note: note_,
        _unrecognized,
    })
}

pub fn _encode_CertErr(value_: &CertErr) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_CertErr_notOK(&value_.notOK)?);
    if let Some(v_) = &value_.note {
        components_.push(_encode_Notifications(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CertErr(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CertErr")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CertErr,
        _eal_components_for_CertErr,
        _rctl2_components_for_CertErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notOK" => _validate_CertErr_notOK(_el)?,
            "note" => _validate_Notifications(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Notifications  ::=  SEQUENCE SIZE (1..MAX) OF Attribute {{SupportedAttributes}}
/// ```
pub type Notifications = Vec<Attribute>; // SequenceOfType

pub fn _decode_Notifications(el: &X690Element) -> ASN1Result<Notifications> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Notifications")),
    };
    let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Attribute(el)?);
    }
    Ok(items)
}

pub fn _encode_Notifications(value_: &Notifications) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Attribute(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Notifications(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Attribute(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Notifications")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// addAvlReq WRAPPED-PDU ::= {
///                 AddAvlReq
///   IDENTIFIED BY id-addAvlReq }
/// ```
///
///
pub fn addAvlReq() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_addAvlReq(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod addAvlReq {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AddAvlReq; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AddAvlReq(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AddAvlReq(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AddAvlReq(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AddAvlReq ::= SEQUENCE {
///   COMPONENTS OF AVMPcommonComponents,
///   certlist      CertAVL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct AddAvlReq {
    pub version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
    pub timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
    pub sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
    pub certlist: CertAVL,
    pub _unrecognized: Vec<X690Element>,
}
impl AddAvlReq {
    pub fn new(
        version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
        timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
        sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
        certlist: CertAVL,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AddAvlReq {
            version,
            timeStamp,
            sequence,
            certlist,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> AVMPversion {
        AVMPversion_v1
    }
}
impl TryFrom<&X690Element> for AddAvlReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlReq(el)
    }
}

pub const _rctl1_components_for_AddAvlReq: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStamp",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddAvlReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AddAvlReq,
        _eal_components_for_AddAvlReq,
        _rctl2_components_for_AddAvlReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AVMPversion> = None;
    let mut timeStamp_: OPTIONAL<GeneralizedTime> = None;
    let mut sequence_: OPTIONAL<AVMPsequence> = None;
    let mut certlist_: OPTIONAL<CertAVL> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AVMPversion(_el)?),
            "timeStamp" => timeStamp_ = Some(BER.decode_generalized_time(_el)?),
            "sequence" => sequence_ = Some(_decode_AVMPsequence(_el)?),
            "certlist" => certlist_ = Some(_decode_CertAVL(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AddAvlReq {
        version: version_,
        timeStamp: timeStamp_.unwrap(),
        sequence: sequence_.unwrap(),
        certlist: certlist_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AddAvlReq(value_: &AddAvlReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    if let Some(v_) = &value_.version {
        if *v_ != AddAvlReq::_default_value_for_version() {
            components_.push(_encode_AVMPversion(&v_)?);
        }
    }
    components_.push(BER.encode_generalized_time(&value_.timeStamp)?);
    components_.push(_encode_AVMPsequence(&value_.sequence)?);
    components_.push(_encode_CertAVL(&value_.certlist)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AddAvlReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddAvlReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AddAvlReq,
        _eal_components_for_AddAvlReq,
        _rctl2_components_for_AddAvlReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AVMPversion(_el)?,
            "timeStamp" => BER.validate_generalized_time(_el)?,
            "sequence" => _validate_AVMPsequence(_el)?,
            "certlist" => _validate_CertAVL(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// addAvlRsp WRAPPED-PDU ::= {
///                  AddAvlRsp
///   IDENTIFIED BY  id-addAvlRsp }
/// ```
///
///
pub fn addAvlRsp() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_addAvlRsp(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod addAvlRsp {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AddAvlRsp; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AddAvlRsp(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AddAvlRsp(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AddAvlRsp(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AddAvlRsp ::= SEQUENCE {
///   COMPONENTS OF AVMPcommonComponents,
///   result        CHOICE {
///     success       [0]  AddAvlOK,
///     failure       [1]  AddAvlErr,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct AddAvlRsp {
    pub version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
    pub timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
    pub sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
    pub result: AddAvlRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl AddAvlRsp {
    pub fn new(
        version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
        timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
        sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
        result: AddAvlRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AddAvlRsp {
            version,
            timeStamp,
            sequence,
            result,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> AVMPversion {
        AVMPversion_v1
    }
}
impl TryFrom<&X690Element> for AddAvlRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlRsp(el)
    }
}

pub const _rctl1_components_for_AddAvlRsp: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStamp",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddAvlRsp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AddAvlRsp,
        _eal_components_for_AddAvlRsp,
        _rctl2_components_for_AddAvlRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AVMPversion> = None;
    let mut timeStamp_: OPTIONAL<GeneralizedTime> = None;
    let mut sequence_: OPTIONAL<AVMPsequence> = None;
    let mut result_: OPTIONAL<AddAvlRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AVMPversion(_el)?),
            "timeStamp" => timeStamp_ = Some(BER.decode_generalized_time(_el)?),
            "sequence" => sequence_ = Some(_decode_AVMPsequence(_el)?),
            "result" => result_ = Some(_decode_AddAvlRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AddAvlRsp {
        version: version_,
        timeStamp: timeStamp_.unwrap(),
        sequence: sequence_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AddAvlRsp(value_: &AddAvlRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    if let Some(v_) = &value_.version {
        if *v_ != AddAvlRsp::_default_value_for_version() {
            components_.push(_encode_AVMPversion(&v_)?);
        }
    }
    components_.push(BER.encode_generalized_time(&value_.timeStamp)?);
    components_.push(_encode_AVMPsequence(&value_.sequence)?);
    components_.push(_encode_AddAvlRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AddAvlRsp(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddAvlRsp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AddAvlRsp,
        _eal_components_for_AddAvlRsp,
        _rctl2_components_for_AddAvlRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AVMPversion(_el)?,
            "timeStamp" => BER.validate_generalized_time(_el)?,
            "sequence" => _validate_AVMPsequence(_el)?,
            "result" => _validate_AddAvlRsp_result(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AddAvlOK ::= SEQUENCE {
///   ok     NULL,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for AddAvlOK {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddAvlOK")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AddAvlOK,
        _eal_components_for_AddAvlOK,
        _rctl2_components_for_AddAvlOK,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut ok_: OPTIONAL<NULL> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ok" => ok_ = Some(BER.decode_null(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AddAvlOK {
        ok: ok_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AddAvlOK(value_: &AddAvlOK) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(BER.encode_null(&value_.ok)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AddAvlOK(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddAvlOK")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AddAvlOK,
        _eal_components_for_AddAvlOK,
        _rctl2_components_for_AddAvlOK,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ok" => BER.validate_null(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AddAvlErr ::= SEQUENCE {
///   notOK  AVMP-error,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for AddAvlErr {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddAvlErr")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AddAvlErr,
        _eal_components_for_AddAvlErr,
        _rctl2_components_for_AddAvlErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut notOK_: OPTIONAL<AVMP_error> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notOK" => notOK_ = Some(_decode_AVMP_error(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AddAvlErr {
        notOK: notOK_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AddAvlErr(value_: &AddAvlErr) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_AVMP_error(&value_.notOK)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AddAvlErr(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AddAvlErr")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AddAvlErr,
        _eal_components_for_AddAvlErr,
        _rctl2_components_for_AddAvlErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notOK" => _validate_AVMP_error(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// replaceAvlReq WRAPPED-PDU ::= {
///                  ReplaceAvlReq
///   IDENTIFIED BY  id-replaceAvlReq }
/// ```
///
///
pub fn replaceAvlReq() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_replaceAvlReq(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod replaceAvlReq {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ReplaceAvlReq; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ReplaceAvlReq(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ReplaceAvlReq(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ReplaceAvlReq(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReplaceAvlReq ::= SEQUENCE {
///   COMPONENTS OF AVMPcommonComponents,
///   old           AvlSerialNumber OPTIONAL,
///   new           CertAVL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct ReplaceAvlReq {
    pub version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
    pub timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
    pub sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
    pub old: OPTIONAL<AvlSerialNumber>,
    pub new: CertAVL,
    pub _unrecognized: Vec<X690Element>,
}
impl ReplaceAvlReq {
    pub fn new(
        version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
        timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
        sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
        old: OPTIONAL<AvlSerialNumber>,
        new: CertAVL,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ReplaceAvlReq {
            version,
            timeStamp,
            sequence,
            old,
            new,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> AVMPversion {
        AVMPversion_v1
    }
}
impl TryFrom<&X690Element> for ReplaceAvlReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ReplaceAvlReq(el)
    }
}

pub const _rctl1_components_for_ReplaceAvlReq: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStamp",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ReplaceAvlReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ReplaceAvlReq,
        _eal_components_for_ReplaceAvlReq,
        _rctl2_components_for_ReplaceAvlReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AVMPversion> = None;
    let mut timeStamp_: OPTIONAL<GeneralizedTime> = None;
    let mut sequence_: OPTIONAL<AVMPsequence> = None;
    let mut old_: OPTIONAL<AvlSerialNumber> = None;
    let mut new_: OPTIONAL<CertAVL> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AVMPversion(_el)?),
            "timeStamp" => timeStamp_ = Some(BER.decode_generalized_time(_el)?),
            "sequence" => sequence_ = Some(_decode_AVMPsequence(_el)?),
            "old" => old_ = Some(_decode_AvlSerialNumber(_el)?),
            "new" => new_ = Some(_decode_CertAVL(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ReplaceAvlReq {
        version: version_,
        timeStamp: timeStamp_.unwrap(),
        sequence: sequence_.unwrap(),
        old: old_,
        new: new_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ReplaceAvlReq(value_: &ReplaceAvlReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    if let Some(v_) = &value_.version {
        if *v_ != ReplaceAvlReq::_default_value_for_version() {
            components_.push(_encode_AVMPversion(&v_)?);
        }
    }
    components_.push(BER.encode_generalized_time(&value_.timeStamp)?);
    components_.push(_encode_AVMPsequence(&value_.sequence)?);
    if let Some(v_) = &value_.old {
        components_.push(_encode_AvlSerialNumber(&v_)?);
    }
    components_.push(_encode_CertAVL(&value_.new)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ReplaceAvlReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ReplaceAvlReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ReplaceAvlReq,
        _eal_components_for_ReplaceAvlReq,
        _rctl2_components_for_ReplaceAvlReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AVMPversion(_el)?,
            "timeStamp" => BER.validate_generalized_time(_el)?,
            "sequence" => _validate_AVMPsequence(_el)?,
            "old" => _validate_AvlSerialNumber(_el)?,
            "new" => _validate_CertAVL(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// replaceAvlRsp WRAPPED-PDU ::= {
///                  ReplaceAvlRsp
///   IDENTIFIED BY  id-replaceAvlRsp }
/// ```
///
///
pub fn replaceAvlRsp() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_replaceAvlRsp(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod replaceAvlRsp {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ReplaceAvlRsp; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ReplaceAvlRsp(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ReplaceAvlRsp(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ReplaceAvlRsp(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReplaceAvlRsp ::= SEQUENCE {
///   COMPONENTS OF AVMPcommonComponents,
///   result        CHOICE {
///     success       [0]  RepAvlOK,
///     failure       [1]  RepAvlErr,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct ReplaceAvlRsp {
    pub version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
    pub timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
    pub sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
    pub result: ReplaceAvlRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl ReplaceAvlRsp {
    pub fn new(
        version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
        timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
        sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
        result: ReplaceAvlRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ReplaceAvlRsp {
            version,
            timeStamp,
            sequence,
            result,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> AVMPversion {
        AVMPversion_v1
    }
}
impl TryFrom<&X690Element> for ReplaceAvlRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ReplaceAvlRsp(el)
    }
}

pub const _rctl1_components_for_ReplaceAvlRsp: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStamp",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ReplaceAvlRsp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ReplaceAvlRsp,
        _eal_components_for_ReplaceAvlRsp,
        _rctl2_components_for_ReplaceAvlRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AVMPversion> = None;
    let mut timeStamp_: OPTIONAL<GeneralizedTime> = None;
    let mut sequence_: OPTIONAL<AVMPsequence> = None;
    let mut result_: OPTIONAL<ReplaceAvlRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AVMPversion(_el)?),
            "timeStamp" => timeStamp_ = Some(BER.decode_generalized_time(_el)?),
            "sequence" => sequence_ = Some(_decode_AVMPsequence(_el)?),
            "result" => result_ = Some(_decode_ReplaceAvlRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ReplaceAvlRsp {
        version: version_,
        timeStamp: timeStamp_.unwrap(),
        sequence: sequence_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ReplaceAvlRsp(value_: &ReplaceAvlRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    if let Some(v_) = &value_.version {
        if *v_ != ReplaceAvlRsp::_default_value_for_version() {
            components_.push(_encode_AVMPversion(&v_)?);
        }
    }
    components_.push(BER.encode_generalized_time(&value_.timeStamp)?);
    components_.push(_encode_AVMPsequence(&value_.sequence)?);
    components_.push(_encode_ReplaceAvlRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ReplaceAvlRsp(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ReplaceAvlRsp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ReplaceAvlRsp,
        _eal_components_for_ReplaceAvlRsp,
        _rctl2_components_for_ReplaceAvlRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AVMPversion(_el)?,
            "timeStamp" => BER.validate_generalized_time(_el)?,
            "sequence" => _validate_AVMPsequence(_el)?,
            "result" => _validate_ReplaceAvlRsp_result(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RepAvlOK ::= SEQUENCE {
///   ok     NULL,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for RepAvlOK {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RepAvlOK")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RepAvlOK,
        _eal_components_for_RepAvlOK,
        _rctl2_components_for_RepAvlOK,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut ok_: OPTIONAL<NULL> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ok" => ok_ = Some(BER.decode_null(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(RepAvlOK {
        ok: ok_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_RepAvlOK(value_: &RepAvlOK) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(BER.encode_null(&value_.ok)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_RepAvlOK(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RepAvlOK")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RepAvlOK,
        _eal_components_for_RepAvlOK,
        _rctl2_components_for_RepAvlOK,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ok" => BER.validate_null(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RepAvlErr ::= SEQUENCE {
///   notOK  AVMP-error,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for RepAvlErr {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RepAvlErr")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RepAvlErr,
        _eal_components_for_RepAvlErr,
        _rctl2_components_for_RepAvlErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut notOK_: OPTIONAL<AVMP_error> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notOK" => notOK_ = Some(_decode_AVMP_error(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(RepAvlErr {
        notOK: notOK_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_RepAvlErr(value_: &RepAvlErr) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_AVMP_error(&value_.notOK)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_RepAvlErr(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RepAvlErr")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RepAvlErr,
        _eal_components_for_RepAvlErr,
        _rctl2_components_for_RepAvlErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notOK" => _validate_AVMP_error(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// deleteAvlReq WRAPPED-PDU ::= {
///                  DeleteAvlReq
///   IDENTIFIED BY  id-deleteAvlReq }
/// ```
///
///
pub fn deleteAvlReq() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_deleteAvlReq(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod deleteAvlReq {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DeleteAvlReq; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DeleteAvlReq(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DeleteAvlReq(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DeleteAvlReq(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DeleteAvlReq ::= SEQUENCE {
///   COMPONENTS OF AVMPcommonComponents,
///   avl-Id        AvlSerialNumber OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct DeleteAvlReq {
    pub version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
    pub timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
    pub sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
    pub avl_Id: OPTIONAL<AvlSerialNumber>,
    pub _unrecognized: Vec<X690Element>,
}
impl DeleteAvlReq {
    pub fn new(
        version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
        timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
        sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
        avl_Id: OPTIONAL<AvlSerialNumber>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DeleteAvlReq {
            version,
            timeStamp,
            sequence,
            avl_Id,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> AVMPversion {
        AVMPversion_v1
    }
}
impl TryFrom<&X690Element> for DeleteAvlReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DeleteAvlReq(el)
    }
}

pub const _rctl1_components_for_DeleteAvlReq: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStamp",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DeleteAvlReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DeleteAvlReq,
        _eal_components_for_DeleteAvlReq,
        _rctl2_components_for_DeleteAvlReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AVMPversion> = None;
    let mut timeStamp_: OPTIONAL<GeneralizedTime> = None;
    let mut sequence_: OPTIONAL<AVMPsequence> = None;
    let mut avl_Id_: OPTIONAL<AvlSerialNumber> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AVMPversion(_el)?),
            "timeStamp" => timeStamp_ = Some(BER.decode_generalized_time(_el)?),
            "sequence" => sequence_ = Some(_decode_AVMPsequence(_el)?),
            "avl-Id" => avl_Id_ = Some(_decode_AvlSerialNumber(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DeleteAvlReq {
        version: version_,
        timeStamp: timeStamp_.unwrap(),
        sequence: sequence_.unwrap(),
        avl_Id: avl_Id_,
        _unrecognized,
    })
}

pub fn _encode_DeleteAvlReq(value_: &DeleteAvlReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    if let Some(v_) = &value_.version {
        if *v_ != DeleteAvlReq::_default_value_for_version() {
            components_.push(_encode_AVMPversion(&v_)?);
        }
    }
    components_.push(BER.encode_generalized_time(&value_.timeStamp)?);
    components_.push(_encode_AVMPsequence(&value_.sequence)?);
    if let Some(v_) = &value_.avl_Id {
        components_.push(_encode_AvlSerialNumber(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DeleteAvlReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DeleteAvlReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DeleteAvlReq,
        _eal_components_for_DeleteAvlReq,
        _rctl2_components_for_DeleteAvlReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AVMPversion(_el)?,
            "timeStamp" => BER.validate_generalized_time(_el)?,
            "sequence" => _validate_AVMPsequence(_el)?,
            "avl-Id" => _validate_AvlSerialNumber(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// deleteAvlRsp WRAPPED-PDU ::= {
///                  DeleteAvlRsp
///   IDENTIFIED BY  id-deleteAvlRsp }
/// ```
///
///
pub fn deleteAvlRsp() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_deleteAvlRsp(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod deleteAvlRsp {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DeleteAvlRsp; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DeleteAvlRsp(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DeleteAvlRsp(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DeleteAvlRsp(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DeleteAvlRsp ::= SEQUENCE {
///   COMPONENTS OF AVMPcommonComponents,
///   result        CHOICE {
///     success       [0]  DelAvlOK,
///     failure       [1]  DelAvlErr,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct DeleteAvlRsp {
    pub version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
    pub timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
    pub sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
    pub result: DeleteAvlRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl DeleteAvlRsp {
    pub fn new(
        version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
        timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
        sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
        result: DeleteAvlRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DeleteAvlRsp {
            version,
            timeStamp,
            sequence,
            result,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> AVMPversion {
        AVMPversion_v1
    }
}
impl TryFrom<&X690Element> for DeleteAvlRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DeleteAvlRsp(el)
    }
}

pub const _rctl1_components_for_DeleteAvlRsp: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStamp",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DeleteAvlRsp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DeleteAvlRsp,
        _eal_components_for_DeleteAvlRsp,
        _rctl2_components_for_DeleteAvlRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AVMPversion> = None;
    let mut timeStamp_: OPTIONAL<GeneralizedTime> = None;
    let mut sequence_: OPTIONAL<AVMPsequence> = None;
    let mut result_: OPTIONAL<DeleteAvlRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AVMPversion(_el)?),
            "timeStamp" => timeStamp_ = Some(BER.decode_generalized_time(_el)?),
            "sequence" => sequence_ = Some(_decode_AVMPsequence(_el)?),
            "result" => result_ = Some(_decode_DeleteAvlRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DeleteAvlRsp {
        version: version_,
        timeStamp: timeStamp_.unwrap(),
        sequence: sequence_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_DeleteAvlRsp(value_: &DeleteAvlRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    if let Some(v_) = &value_.version {
        if *v_ != DeleteAvlRsp::_default_value_for_version() {
            components_.push(_encode_AVMPversion(&v_)?);
        }
    }
    components_.push(BER.encode_generalized_time(&value_.timeStamp)?);
    components_.push(_encode_AVMPsequence(&value_.sequence)?);
    components_.push(_encode_DeleteAvlRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DeleteAvlRsp(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DeleteAvlRsp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DeleteAvlRsp,
        _eal_components_for_DeleteAvlRsp,
        _rctl2_components_for_DeleteAvlRsp,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AVMPversion(_el)?,
            "timeStamp" => BER.validate_generalized_time(_el)?,
            "sequence" => _validate_AVMPsequence(_el)?,
            "result" => _validate_DeleteAvlRsp_result(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DelAvlOK ::= SEQUENCE {
///   ok     NULL,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for DelAvlOK {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DelAvlOK")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DelAvlOK,
        _eal_components_for_DelAvlOK,
        _rctl2_components_for_DelAvlOK,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut ok_: OPTIONAL<NULL> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ok" => ok_ = Some(BER.decode_null(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DelAvlOK {
        ok: ok_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_DelAvlOK(value_: &DelAvlOK) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(BER.encode_null(&value_.ok)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DelAvlOK(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DelAvlOK")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DelAvlOK,
        _eal_components_for_DelAvlOK,
        _rctl2_components_for_DelAvlOK,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ok" => BER.validate_null(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DelAvlErr ::= SEQUENCE {
///   notOK  AVMP-error,
///   ... }
/// ```
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
impl TryFrom<&X690Element> for DelAvlErr {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DelAvlErr")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DelAvlErr,
        _eal_components_for_DelAvlErr,
        _rctl2_components_for_DelAvlErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut notOK_: OPTIONAL<AVMP_error> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notOK" => notOK_ = Some(_decode_AVMP_error(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DelAvlErr {
        notOK: notOK_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_DelAvlErr(value_: &DelAvlErr) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_AVMP_error(&value_.notOK)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DelAvlErr(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DelAvlErr")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DelAvlErr,
        _eal_components_for_DelAvlErr,
        _rctl2_components_for_DelAvlErr,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "notOK" => _validate_AVMP_error(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rejectAVL  WRAPPED-PDU ::= {
///                  RejectAVL
///   IDENTIFIED BY  id-rejectAVL }
/// ```
///
///
pub fn rejectAVL() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_rejectAVL(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod rejectAVL {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = RejectAVL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_RejectAVL(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_RejectAVL(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_RejectAVL(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RejectAVL ::= SEQUENCE {
///   COMPONENTS OF AVMPcommonComponents,
///   reason        AVMP-error,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct RejectAVL {
    pub version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
    pub timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
    pub sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
    pub reason: AVMP_error,
    pub _unrecognized: Vec<X690Element>,
}
impl RejectAVL {
    pub fn new(
        version: OPTIONAL<AVMPversion>, /* REPLICATED_COMPONENT */
        timeStamp: GeneralizedTime,     /* REPLICATED_COMPONENT */
        sequence: AVMPsequence,         /* REPLICATED_COMPONENT */
        reason: AVMP_error,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        RejectAVL {
            version,
            timeStamp,
            sequence,
            reason,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> AVMPversion {
        AVMPversion_v1
    }
}
impl TryFrom<&X690Element> for RejectAVL {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RejectAVL(el)
    }
}

pub const _rctl1_components_for_RejectAVL: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStamp",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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

pub const _rctl2_components_for_RejectAVL: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RejectAVL: &[ComponentSpec; 0] = &[];

pub fn _decode_RejectAVL(el: &X690Element) -> ASN1Result<RejectAVL> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RejectAVL")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RejectAVL,
        _eal_components_for_RejectAVL,
        _rctl2_components_for_RejectAVL,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<AVMPversion> = None;
    let mut timeStamp_: OPTIONAL<GeneralizedTime> = None;
    let mut sequence_: OPTIONAL<AVMPsequence> = None;
    let mut reason_: OPTIONAL<AVMP_error> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_AVMPversion(_el)?),
            "timeStamp" => timeStamp_ = Some(BER.decode_generalized_time(_el)?),
            "sequence" => sequence_ = Some(_decode_AVMPsequence(_el)?),
            "reason" => reason_ = Some(_decode_AVMP_error(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(RejectAVL {
        version: version_,
        timeStamp: timeStamp_.unwrap(),
        sequence: sequence_.unwrap(),
        reason: reason_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_RejectAVL(value_: &RejectAVL) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    if let Some(v_) = &value_.version {
        if *v_ != RejectAVL::_default_value_for_version() {
            components_.push(_encode_AVMPversion(&v_)?);
        }
    }
    components_.push(BER.encode_generalized_time(&value_.timeStamp)?);
    components_.push(_encode_AVMPsequence(&value_.sequence)?);
    components_.push(_encode_AVMP_error(&value_.reason)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_RejectAVL(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RejectAVL")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RejectAVL,
        _eal_components_for_RejectAVL,
        _rctl2_components_for_RejectAVL,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_AVMPversion(_el)?,
            "timeStamp" => BER.validate_generalized_time(_el)?,
            "sequence" => _validate_AVMPsequence(_el)?,
            "reason" => _validate_AVMP_error(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CASPcommonComponents ::= SEQUENCE {
///   version    CASPversion DEFAULT v1,
///   sequence   CASPsequence,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CASPcommonComponents {
    pub version: OPTIONAL<CASPversion>,
    pub sequence: CASPsequence,
    pub _unrecognized: Vec<X690Element>,
}
impl CASPcommonComponents {
    pub fn new(
        version: OPTIONAL<CASPversion>,
        sequence: CASPsequence,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CASPcommonComponents {
            version,
            sequence,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> CASPversion {
        CASPversion_v1
    }
}
impl TryFrom<&X690Element> for CASPcommonComponents {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CASPcommonComponents(el)
    }
}

pub const _rctl1_components_for_CASPcommonComponents: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CASPcommonComponents: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CASPcommonComponents: &[ComponentSpec; 0] = &[];

pub fn _decode_CASPcommonComponents(el: &X690Element) -> ASN1Result<CASPcommonComponents> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CASPcommonComponents")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CASPcommonComponents,
        _eal_components_for_CASPcommonComponents,
        _rctl2_components_for_CASPcommonComponents,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CASPversion> = None;
    let mut sequence_: OPTIONAL<CASPsequence> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CASPversion(_el)?),
            "sequence" => sequence_ = Some(_decode_CASPsequence(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CASPcommonComponents {
        version: version_,
        sequence: sequence_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CASPcommonComponents(value_: &CASPcommonComponents) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.version {
        if *v_ != CASPcommonComponents::_default_value_for_version() {
            components_.push(_encode_CASPversion(&v_)?);
        }
    }
    components_.push(_encode_CASPsequence(&value_.sequence)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_CASPcommonComponents(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CASPcommonComponents")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_CASPcommonComponents,
        _eal_components_for_CASPcommonComponents,
        _rctl2_components_for_CASPcommonComponents,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CASPversion(_el)?,
            "sequence" => _validate_CASPsequence(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CASPversion  ::=  ENUMERATED { v1(1), v2(2), v3(3), ... }
/// ```
pub type CASPversion = ENUMERATED;

pub const CASPversion_v1: CASPversion = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CASPversion_v2: CASPversion = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CASPversion_v3: CASPversion = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_CASPversion(el: &X690Element) -> ASN1Result<CASPversion> {
    BER.decode_enumerated(&el)
}

pub fn _encode_CASPversion(value_: &CASPversion) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_CASPversion(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CASPsequence  ::=  INTEGER (1..MAX)
/// ```
pub type CASPsequence = INTEGER;

pub fn _decode_CASPsequence(el: &X690Element) -> ASN1Result<CASPsequence> {
    BER.decode_integer(&el)
}

pub fn _encode_CASPsequence(value_: &CASPsequence) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_CASPsequence(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certSubscribeReq WRAPPED-PDU ::= {
///                  CertSubscribeReq
///   IDENTIFIED BY  id-certSubscribeReq }
/// ```
///
///
pub fn certSubscribeReq() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_certSubscribeReq(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod certSubscribeReq {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertSubscribeReq; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertSubscribeReq(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertSubscribeReq(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertSubscribeReq(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertSubscribeReq ::= SEQUENCE {
///   COMPONENTS OF CASPcommonComponents,
///   certs   SEQUENCE (SIZE (1..MAX)) OF SEQUENCE {
///     subject      Name,
///     serialNumber CertificateSerialNumber,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertSubscribeReq {
    pub version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
    pub sequence: CASPsequence,         /* REPLICATED_COMPONENT */
    pub certs: Vec<CertSubscribeReq_certs_Item>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertSubscribeReq {
    pub fn new(
        version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
        sequence: CASPsequence,         /* REPLICATED_COMPONENT */
        certs: Vec<CertSubscribeReq_certs_Item>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertSubscribeReq {
            version,
            sequence,
            certs,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> CASPversion {
        CASPversion_v1
    }
}
impl TryFrom<&X690Element> for CertSubscribeReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeReq(el)
    }
}

pub const _rctl1_components_for_CertSubscribeReq: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let mut version_: OPTIONAL<CASPversion> = None;
    let mut sequence_: OPTIONAL<CASPsequence> = None;
    let mut certs_: OPTIONAL<Vec<CertSubscribeReq_certs_Item>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CASPversion(_el)?),
            "sequence" => sequence_ = Some(_decode_CASPsequence(_el)?),
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
        version: version_,
        sequence: sequence_.unwrap(),
        certs: certs_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertSubscribeReq(value_: &CertSubscribeReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.version {
        if *v_ != CertSubscribeReq::_default_value_for_version() {
            components_.push(_encode_CASPversion(&v_)?);
        }
    }
    components_.push(_encode_CASPsequence(&value_.sequence)?);
    components_.push(
        |value_: &SEQUENCE_OF<CertSubscribeReq_certs_Item>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_CertSubscribeReq_certs_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.certs)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
            "version" => _validate_CASPversion(_el)?,
            "sequence" => _validate_CASPsequence(_el)?,
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
/// certSubscribeRsp WRAPPED-PDU ::= {
///                  CertSubscribeRsp
///   IDENTIFIED BY  id-certSubscribeRsp }
/// ```
///
///
pub fn certSubscribeRsp() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_certSubscribeRsp(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod certSubscribeRsp {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertSubscribeRsp; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertSubscribeRsp(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertSubscribeRsp(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertSubscribeRsp(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertSubscribeRsp ::= SEQUENCE {
///   COMPONENTS OF CASPcommonComponents,
///   result       CHOICE {
///     success       [0]  CertSubscribeOK,
///     failure       [1]  CertSubscribeErr,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertSubscribeRsp {
    pub version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
    pub sequence: CASPsequence,         /* REPLICATED_COMPONENT */
    pub result: CertSubscribeRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl CertSubscribeRsp {
    pub fn new(
        version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
        sequence: CASPsequence,         /* REPLICATED_COMPONENT */
        result: CertSubscribeRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertSubscribeRsp {
            version,
            sequence,
            result,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> CASPversion {
        CASPversion_v1
    }
}
impl TryFrom<&X690Element> for CertSubscribeRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertSubscribeRsp(el)
    }
}

pub const _rctl1_components_for_CertSubscribeRsp: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let mut version_: OPTIONAL<CASPversion> = None;
    let mut sequence_: OPTIONAL<CASPsequence> = None;
    let mut result_: OPTIONAL<CertSubscribeRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CASPversion(_el)?),
            "sequence" => sequence_ = Some(_decode_CASPsequence(_el)?),
            "result" => result_ = Some(_decode_CertSubscribeRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertSubscribeRsp {
        version: version_,
        sequence: sequence_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertSubscribeRsp(value_: &CertSubscribeRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.version {
        if *v_ != CertSubscribeRsp::_default_value_for_version() {
            components_.push(_encode_CASPversion(&v_)?);
        }
    }
    components_.push(_encode_CASPsequence(&value_.sequence)?);
    components_.push(_encode_CertSubscribeRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
            "version" => _validate_CASPversion(_el)?,
            "sequence" => _validate_CASPsequence(_el)?,
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
/// certUnsubscribeReq WRAPPED-PDU ::= {
///                  CertUnsubscribeReq
///   IDENTIFIED BY  id-certUnsubscribeReq }
/// ```
///
///
pub fn certUnsubscribeReq() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_certUnsubscribeReq(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod certUnsubscribeReq {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertUnsubscribeReq; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertUnsubscribeReq(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertUnsubscribeReq(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertUnsubscribeReq(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUnsubscribeReq ::= SEQUENCE {
///   COMPONENTS OF CASPcommonComponents,
///   certs  SEQUENCE (SIZE (1..MAX)) OF SEQUENCE {
///     subject      Name,
///     serialNumber CertificateSerialNumber,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertUnsubscribeReq {
    pub version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
    pub sequence: CASPsequence,         /* REPLICATED_COMPONENT */
    pub certs: Vec<CertUnsubscribeReq_certs_Item>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUnsubscribeReq {
    pub fn new(
        version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
        sequence: CASPsequence,         /* REPLICATED_COMPONENT */
        certs: Vec<CertUnsubscribeReq_certs_Item>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUnsubscribeReq {
            version,
            sequence,
            certs,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> CASPversion {
        CASPversion_v1
    }
}
impl TryFrom<&X690Element> for CertUnsubscribeReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeReq(el)
    }
}

pub const _rctl1_components_for_CertUnsubscribeReq: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let mut version_: OPTIONAL<CASPversion> = None;
    let mut sequence_: OPTIONAL<CASPsequence> = None;
    let mut certs_: OPTIONAL<Vec<CertUnsubscribeReq_certs_Item>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CASPversion(_el)?),
            "sequence" => sequence_ = Some(_decode_CASPsequence(_el)?),
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
        version: version_,
        sequence: sequence_.unwrap(),
        certs: certs_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUnsubscribeReq(value_: &CertUnsubscribeReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.version {
        if *v_ != CertUnsubscribeReq::_default_value_for_version() {
            components_.push(_encode_CASPversion(&v_)?);
        }
    }
    components_.push(_encode_CASPsequence(&value_.sequence)?);
    components_.push(
        |value_: &SEQUENCE_OF<CertUnsubscribeReq_certs_Item>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_CertUnsubscribeReq_certs_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.certs)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
            "version" => _validate_CASPversion(_el)?,
            "sequence" => _validate_CASPsequence(_el)?,
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
/// certUnsubscribeRsp WRAPPED-PDU ::= {
///                  CertUnsubscribeRsp
///   IDENTIFIED BY  id-certUnsubscribeRsp }
/// ```
///
///
pub fn certUnsubscribeRsp() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_certUnsubscribeRsp(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod certUnsubscribeRsp {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertUnsubscribeRsp; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertUnsubscribeRsp(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertUnsubscribeRsp(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertUnsubscribeRsp(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUnsubscribeRsp ::= SEQUENCE {
///   COMPONENTS OF CASPcommonComponents,
///   result       CHOICE {
///     success       [0]  CertUnsubscribeOK,
///     failure       [1]  CertUnsubscribeErr,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertUnsubscribeRsp {
    pub version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
    pub sequence: CASPsequence,         /* REPLICATED_COMPONENT */
    pub result: CertUnsubscribeRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUnsubscribeRsp {
    pub fn new(
        version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
        sequence: CASPsequence,         /* REPLICATED_COMPONENT */
        result: CertUnsubscribeRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUnsubscribeRsp {
            version,
            sequence,
            result,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> CASPversion {
        CASPversion_v1
    }
}
impl TryFrom<&X690Element> for CertUnsubscribeRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertUnsubscribeRsp(el)
    }
}

pub const _rctl1_components_for_CertUnsubscribeRsp: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let mut version_: OPTIONAL<CASPversion> = None;
    let mut sequence_: OPTIONAL<CASPsequence> = None;
    let mut result_: OPTIONAL<CertUnsubscribeRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CASPversion(_el)?),
            "sequence" => sequence_ = Some(_decode_CASPsequence(_el)?),
            "result" => result_ = Some(_decode_CertUnsubscribeRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUnsubscribeRsp {
        version: version_,
        sequence: sequence_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUnsubscribeRsp(value_: &CertUnsubscribeRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.version {
        if *v_ != CertUnsubscribeRsp::_default_value_for_version() {
            components_.push(_encode_CASPversion(&v_)?);
        }
    }
    components_.push(_encode_CASPsequence(&value_.sequence)?);
    components_.push(_encode_CertUnsubscribeRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
            "version" => _validate_CASPversion(_el)?,
            "sequence" => _validate_CASPsequence(_el)?,
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
/// certReplaceReq WRAPPED-PDU ::= {
///                  CertReplaceReq
///   IDENTIFIED BY  id-certReplaceReq }
/// ```
///
///
pub fn certReplaceReq() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_certReplaceReq(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod certReplaceReq {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertReplaceReq; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertReplaceReq(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertReplaceReq(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertReplaceReq(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertReplaceReq ::= SEQUENCE {
///   COMPONENTS OF CASPcommonComponents,
///   certs         SEQUENCE (SIZE (1..MAX)) OF SEQUENCE {
///     old           CertificateSerialNumber,
///     new           Certificate,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertReplaceReq {
    pub version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
    pub sequence: CASPsequence,         /* REPLICATED_COMPONENT */
    pub certs: Vec<CertReplaceReq_certs_Item>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertReplaceReq {
    pub fn new(
        version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
        sequence: CASPsequence,         /* REPLICATED_COMPONENT */
        certs: Vec<CertReplaceReq_certs_Item>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertReplaceReq {
            version,
            sequence,
            certs,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> CASPversion {
        CASPversion_v1
    }
}
impl TryFrom<&X690Element> for CertReplaceReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceReq(el)
    }
}

pub const _rctl1_components_for_CertReplaceReq: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let mut version_: OPTIONAL<CASPversion> = None;
    let mut sequence_: OPTIONAL<CASPsequence> = None;
    let mut certs_: OPTIONAL<Vec<CertReplaceReq_certs_Item>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CASPversion(_el)?),
            "sequence" => sequence_ = Some(_decode_CASPsequence(_el)?),
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
        version: version_,
        sequence: sequence_.unwrap(),
        certs: certs_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertReplaceReq(value_: &CertReplaceReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.version {
        if *v_ != CertReplaceReq::_default_value_for_version() {
            components_.push(_encode_CASPversion(&v_)?);
        }
    }
    components_.push(_encode_CASPsequence(&value_.sequence)?);
    components_.push(
        |value_: &SEQUENCE_OF<CertReplaceReq_certs_Item>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_CertReplaceReq_certs_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.certs)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
            "version" => _validate_CASPversion(_el)?,
            "sequence" => _validate_CASPsequence(_el)?,
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
/// certReplaceRsp WRAPPED-PDU ::= {
///                  CertReplaceRsp
///   IDENTIFIED BY  id-certReplaceRsp }
/// ```
///
///
pub fn certReplaceRsp() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_certReplaceRsp(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod certReplaceRsp {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertReplaceRsp; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertReplaceRsp(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertReplaceRsp(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertReplaceRsp(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertReplaceRsp ::= SEQUENCE {
///   COMPONENTS OF CASPcommonComponents,
///   result        CHOICE {
///     success       [0]  CertReplaceOK,
///     failure       [1]  CertReplaceErr,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertReplaceRsp {
    pub version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
    pub sequence: CASPsequence,         /* REPLICATED_COMPONENT */
    pub result: CertReplaceRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl CertReplaceRsp {
    pub fn new(
        version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
        sequence: CASPsequence,         /* REPLICATED_COMPONENT */
        result: CertReplaceRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertReplaceRsp {
            version,
            sequence,
            result,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> CASPversion {
        CASPversion_v1
    }
}
impl TryFrom<&X690Element> for CertReplaceRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceRsp(el)
    }
}

pub const _rctl1_components_for_CertReplaceRsp: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let mut version_: OPTIONAL<CASPversion> = None;
    let mut sequence_: OPTIONAL<CASPsequence> = None;
    let mut result_: OPTIONAL<CertReplaceRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CASPversion(_el)?),
            "sequence" => sequence_ = Some(_decode_CASPsequence(_el)?),
            "result" => result_ = Some(_decode_CertReplaceRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertReplaceRsp {
        version: version_,
        sequence: sequence_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertReplaceRsp(value_: &CertReplaceRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.version {
        if *v_ != CertReplaceRsp::_default_value_for_version() {
            components_.push(_encode_CASPversion(&v_)?);
        }
    }
    components_.push(_encode_CASPsequence(&value_.sequence)?);
    components_.push(_encode_CertReplaceRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
            "version" => _validate_CASPversion(_el)?,
            "sequence" => _validate_CASPsequence(_el)?,
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
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
///   code        CHOICE {
///     signedData     [0]  SignedData-error,
///     envelopedData  [1]  EnvelopedData-error,
///     casp           [2]  CASP-error,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertReplaceErr {
    pub code: CertReplaceErr_code,
    pub _unrecognized: Vec<X690Element>,
}
impl CertReplaceErr {
    pub fn new(code: CertReplaceErr_code, _unrecognized: Vec<X690Element>) -> Self {
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
    TagSelector::any,
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
    let mut code_: OPTIONAL<CertReplaceErr_code> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "code" => code_ = Some(_decode_CertReplaceErr_code(_el)?),
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
    components_.push(_encode_CertReplaceErr_code(&value_.code)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
            "code" => _validate_CertReplaceErr_code(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certUpdateReq WRAPPED-PDU ::= {
///                  CertUpdateReq
///   IDENTIFIED BY  id-certUpdateReq }
/// ```
///
///
pub fn certUpdateReq() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_certUpdateReq(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod certUpdateReq {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertUpdateReq; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertUpdateReq(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertUpdateReq(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertUpdateReq(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUpdateReq ::= SEQUENCE {
///   COMPONENTS OF CASPcommonComponents,
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
    pub version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
    pub sequence: CASPsequence,         /* REPLICATED_COMPONENT */
    pub certs: Vec<CertUpdateReq_certs_Item>,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUpdateReq {
    pub fn new(
        version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
        sequence: CASPsequence,         /* REPLICATED_COMPONENT */
        certs: Vec<CertUpdateReq_certs_Item>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUpdateReq {
            version,
            sequence,
            certs,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> CASPversion {
        CASPversion_v1
    }
}
impl TryFrom<&X690Element> for CertUpdateReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateReq(el)
    }
}

pub const _rctl1_components_for_CertUpdateReq: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let mut version_: OPTIONAL<CASPversion> = None;
    let mut sequence_: OPTIONAL<CASPsequence> = None;
    let mut certs_: OPTIONAL<Vec<CertUpdateReq_certs_Item>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CASPversion(_el)?),
            "sequence" => sequence_ = Some(_decode_CASPsequence(_el)?),
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
        version: version_,
        sequence: sequence_.unwrap(),
        certs: certs_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUpdateReq(value_: &CertUpdateReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.version {
        if *v_ != CertUpdateReq::_default_value_for_version() {
            components_.push(_encode_CASPversion(&v_)?);
        }
    }
    components_.push(_encode_CASPsequence(&value_.sequence)?);
    components_.push(
        |value_: &SEQUENCE_OF<CertUpdateReq_certs_Item>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_CertUpdateReq_certs_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.certs)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
            "version" => _validate_CASPversion(_el)?,
            "sequence" => _validate_CASPsequence(_el)?,
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
/// certUpdateRsp WRAPPED-PDU ::= {
///                  CertUpdateRsp
///   IDENTIFIED BY  id-certUpdateRsp }
/// ```
///
///
pub fn certUpdateRsp() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_certUpdateRsp(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod certUpdateRsp {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CertUpdateRsp; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CertUpdateRsp(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CertUpdateRsp(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CertUpdateRsp(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertUpdateRsp ::= SEQUENCE {
///   COMPONENTS OF CASPcommonComponents,
///   result        CHOICE {
///     success       [0]  CertUpdateOK,
///     failure       [1]  CertUpdateErr,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct CertUpdateRsp {
    pub version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
    pub sequence: CASPsequence,         /* REPLICATED_COMPONENT */
    pub result: CertUpdateRsp_result,
    pub _unrecognized: Vec<X690Element>,
}
impl CertUpdateRsp {
    pub fn new(
        version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
        sequence: CASPsequence,         /* REPLICATED_COMPONENT */
        result: CertUpdateRsp_result,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        CertUpdateRsp {
            version,
            sequence,
            result,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> CASPversion {
        CASPversion_v1
    }
}
impl TryFrom<&X690Element> for CertUpdateRsp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertUpdateRsp(el)
    }
}

pub const _rctl1_components_for_CertUpdateRsp: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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
    let mut version_: OPTIONAL<CASPversion> = None;
    let mut sequence_: OPTIONAL<CASPsequence> = None;
    let mut result_: OPTIONAL<CertUpdateRsp_result> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CASPversion(_el)?),
            "sequence" => sequence_ = Some(_decode_CASPsequence(_el)?),
            "result" => result_ = Some(_decode_CertUpdateRsp_result(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(CertUpdateRsp {
        version: version_,
        sequence: sequence_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_CertUpdateRsp(value_: &CertUpdateRsp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.version {
        if *v_ != CertUpdateRsp::_default_value_for_version() {
            components_.push(_encode_CASPversion(&v_)?);
        }
    }
    components_.push(_encode_CASPsequence(&value_.sequence)?);
    components_.push(_encode_CertUpdateRsp_result(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
            "version" => _validate_CASPversion(_el)?,
            "sequence" => _validate_CASPsequence(_el)?,
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
/// rejectCAsubscribe  WRAPPED-PDU ::= {
///                  RejectCAsubscribe
///   IDENTIFIED BY  id-rejectCAsubscribe }
/// ```
///
///
pub fn rejectCAsubscribe() -> WRAPPED_PDU {
    TYPE_IDENTIFIER {
        id: id_rejectCAsubscribe(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod rejectCAsubscribe {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = RejectCAsubscribe; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_RejectCAsubscribe(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_RejectCAsubscribe(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_RejectCAsubscribe(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RejectCAsubscribe ::= SEQUENCE {
///   COMPONENTS OF CASPcommonComponents,
///   reason        CASP-error,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct RejectCAsubscribe {
    pub version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
    pub sequence: CASPsequence,         /* REPLICATED_COMPONENT */
    pub reason: CASP_error,
    pub _unrecognized: Vec<X690Element>,
}
impl RejectCAsubscribe {
    pub fn new(
        version: OPTIONAL<CASPversion>, /* REPLICATED_COMPONENT */
        sequence: CASPsequence,         /* REPLICATED_COMPONENT */
        reason: CASP_error,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        RejectCAsubscribe {
            version,
            sequence,
            reason,
            _unrecognized,
        }
    }
    pub fn _default_value_for_version() -> CASPversion {
        CASPversion_v1
    }
}
impl TryFrom<&X690Element> for RejectCAsubscribe {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RejectCAsubscribe(el)
    }
}

pub const _rctl1_components_for_RejectCAsubscribe: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sequence",
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

pub const _rctl2_components_for_RejectCAsubscribe: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RejectCAsubscribe: &[ComponentSpec; 0] = &[];

pub fn _decode_RejectCAsubscribe(el: &X690Element) -> ASN1Result<RejectCAsubscribe> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RejectCAsubscribe")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RejectCAsubscribe,
        _eal_components_for_RejectCAsubscribe,
        _rctl2_components_for_RejectCAsubscribe,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<CASPversion> = None;
    let mut sequence_: OPTIONAL<CASPsequence> = None;
    let mut reason_: OPTIONAL<CASP_error> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_CASPversion(_el)?),
            "sequence" => sequence_ = Some(_decode_CASPsequence(_el)?),
            "reason" => reason_ = Some(_decode_CASP_error(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(RejectCAsubscribe {
        version: version_,
        sequence: sequence_.unwrap(),
        reason: reason_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_RejectCAsubscribe(value_: &RejectCAsubscribe) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.version {
        if *v_ != RejectCAsubscribe::_default_value_for_version() {
            components_.push(_encode_CASPversion(&v_)?);
        }
    }
    components_.push(_encode_CASPsequence(&value_.sequence)?);
    components_.push(_encode_CASP_error(&value_.reason)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_RejectCAsubscribe(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RejectCAsubscribe")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RejectCAsubscribe,
        _eal_components_for_RejectCAsubscribe,
        _rctl2_components_for_RejectCAsubscribe,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => _validate_CASPversion(_el)?,
            "sequence" => _validate_CASPsequence(_el)?,
            "reason" => _validate_CASP_error(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignedData-error  ::=  ENUMERATED {
///   noReason                           (0),
///   signedDataContectTypeExpected      (1),
///   wrongSignedDataVersion             (2),
///   missingContent                     (3),
///   missingContentComponent            (4),
///   invalidContentComponent            (5),
///   unsupportedHashAlgorithm           (6),
///   ... }
/// ```
pub type SignedData_error = ENUMERATED;

pub const SignedData_error_noReason: SignedData_error = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SignedData_error_signedDataContectTypeExpected: SignedData_error = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SignedData_error_wrongSignedDataVersion: SignedData_error = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SignedData_error_missingContent: SignedData_error = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SignedData_error_missingContentComponent: SignedData_error = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SignedData_error_invalidContentComponent: SignedData_error = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SignedData_error_unsupportedHashAlgorithm: SignedData_error = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_SignedData_error(el: &X690Element) -> ASN1Result<SignedData_error> {
    BER.decode_enumerated(&el)
}

pub fn _encode_SignedData_error(value_: &SignedData_error) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_SignedData_error(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EnvelopedData-error  ::=  ENUMERATED {
///   noReason                           (0),
///   ... }
/// ```
pub type EnvelopedData_error = ENUMERATED;

pub const EnvelopedData_error_noReason: EnvelopedData_error = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_EnvelopedData_error(el: &X690Element) -> ASN1Result<EnvelopedData_error> {
    BER.decode_enumerated(&el)
}

pub fn _encode_EnvelopedData_error(value_: &EnvelopedData_error) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_EnvelopedData_error(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AVMP-error  ::=  ENUMERATED {
///   noReason                           (0),
///   unknownAvlEntity                   (1),
///   unknownContentType                 (2),
///   unsupportedAVMPversion             (3),
///   missingContent                     (4),
///   missingContentComponent            (5),
///   invalidContentComponent            (6),
///   sequenceError                      (7),
///   protocolError                      (8),
///   invalidAvlSignature                (9),
///   duplicateAVL                       (10),
///   missingAvlComponent                (11),
///   invalidAvlVersion                  (12),
///   notAllowedForConstrainedAVLEntity  (13),
///   constrainedRequired                (14),
///   nonConstrainedRequired             (15),
///   unsupportedCriticalEntryExtension  (16),
///   unsupportedCriticalExtension       (17),
///   maxAVLsExceeded                    (18),
///   unknownCert                        (19),
///   unknownAVL                         (20),
///   unsupportedScopeRestriction        (21),
///   ... }
/// ```
pub type AVMP_error = ENUMERATED;

pub const AVMP_error_noReason: AVMP_error = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_unknownAvlEntity: AVMP_error = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_unknownContentType: AVMP_error = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_unsupportedAVMPversion: AVMP_error = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_missingContent: AVMP_error = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_missingContentComponent: AVMP_error = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_invalidContentComponent: AVMP_error = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_sequenceError: AVMP_error = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_protocolError: AVMP_error = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_invalidAvlSignature: AVMP_error = 9; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_duplicateAVL: AVMP_error = 10; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_missingAvlComponent: AVMP_error = 11; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_invalidAvlVersion: AVMP_error = 12; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_notAllowedForConstrainedAVLEntity: AVMP_error = 13; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_constrainedRequired: AVMP_error = 14; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_nonConstrainedRequired: AVMP_error = 15; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_unsupportedCriticalEntryExtension: AVMP_error = 16; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_unsupportedCriticalExtension: AVMP_error = 17; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_maxAVLsExceeded: AVMP_error = 18; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_unknownCert: AVMP_error = 19; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_unknownAVL: AVMP_error = 20; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AVMP_error_unsupportedScopeRestriction: AVMP_error = 21; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_AVMP_error(el: &X690Element) -> ASN1Result<AVMP_error> {
    BER.decode_enumerated(&el)
}

pub fn _encode_AVMP_error(value_: &AVMP_error) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_AVMP_error(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
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
/// id-signedData OBJECT IDENTIFIER ::= {iso(1) member-body(2)
/// us(840)rsadsi(113549) pkcs(1) pkcs7(7) 2}
/// ```
///
///
pub fn id_signedData() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 2,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-envelopedData OBJECT IDENTIFIER ::= {iso(1) member-body(2) us(840)
/// rsadsi(113549) pkcs(1) pkcs7(7) 3}
/// ```
///
///
pub fn id_envelopedData() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 3,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-certReq              OBJECT IDENTIFIER ::= {id-cmsct 0}
/// ```
///
///
pub fn id_certReq() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([0])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-certRsp              OBJECT IDENTIFIER ::= {id-cmsct 1}
/// ```
///
///
pub fn id_certRsp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-addAvlReq            OBJECT IDENTIFIER ::= {id-cmsct 2}
/// ```
///
///
pub fn id_addAvlReq() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-addAvlRsp            OBJECT IDENTIFIER ::= {id-cmsct 3}
/// ```
///
///
pub fn id_addAvlRsp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-replaceAvlReq        OBJECT IDENTIFIER ::= {id-cmsct 4}
/// ```
///
///
pub fn id_replaceAvlReq() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-replaceAvlRsp        OBJECT IDENTIFIER ::= {id-cmsct 5}
/// ```
///
///
pub fn id_replaceAvlRsp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-updateAvlReq         OBJECT IDENTIFIER ::= {id-cmsct 6}
/// ```
///
///
pub fn id_updateAvlReq() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-updateAvlRsp         OBJECT IDENTIFIER ::= {id-cmsct 7}
/// ```
///
///
pub fn id_updateAvlRsp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-deleteAvlReq         OBJECT IDENTIFIER ::= {id-cmsct 8}
/// ```
///
///
pub fn id_deleteAvlReq() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-deleteAvlRsp         OBJECT IDENTIFIER ::= {id-cmsct 9}
/// ```
///
///
pub fn id_deleteAvlRsp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rejectAVL            OBJECT IDENTIFIER ::= {id-cmsct 10}
/// ```
///
///
pub fn id_rejectAVL() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-certSubscribeReq     OBJECT IDENTIFIER ::= {id-cmsct 11}
/// ```
///
///
pub fn id_certSubscribeReq() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-certSubscribeRsp     OBJECT IDENTIFIER ::= {id-cmsct 12}
/// ```
///
///
pub fn id_certSubscribeRsp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-certUnsubscribeReq   OBJECT IDENTIFIER ::= {id-cmsct 13}
/// ```
///
///
pub fn id_certUnsubscribeReq() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-certUnsubscribeRsp   OBJECT IDENTIFIER ::= {id-cmsct 14}
/// ```
///
///
pub fn id_certUnsubscribeRsp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([14])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-certReplaceReq       OBJECT IDENTIFIER ::= {id-cmsct 15}
/// ```
///
///
pub fn id_certReplaceReq() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([15])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-certReplaceRsp       OBJECT IDENTIFIER ::= {id-cmsct 16}
/// ```
///
///
pub fn id_certReplaceRsp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([16])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-certUpdateReq        OBJECT IDENTIFIER ::= {id-cmsct 17}
/// ```
///
///
pub fn id_certUpdateReq() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([17])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-certUpdateRsp        OBJECT IDENTIFIER ::= {id-cmsct 18}
/// ```
///
///
pub fn id_certUpdateRsp() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([18])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rejectCAsubscribe    OBJECT IDENTIFIER ::= {id-cmsct 19}
/// ```
///
///
pub fn id_rejectCAsubscribe() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cmsct().0, Vec::<u32>::from([19])].concat()) // OID_GETTER
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
        (TagClass::CONTEXT, 0) => Ok(TBresponse::success(
            |el: &X690Element| -> ASN1Result<TBOK> { Ok(_decode_TBOK(&el.inner()?)?) }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(TBresponse::failure(
            |el: &X690Element| -> ASN1Result<TBerror> { Ok(_decode_TBerror(&el.inner()?)?) }(&el)?,
        )),
        _ => Ok(TBresponse::_unrecognized(el.clone())),
    }
}

pub fn _encode_TBresponse(value_: &TBresponse) -> ASN1Result<X690Element> {
    match value_ {
        TBresponse::success(v) => |v_1: &TBOK| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_TBOK(&v_1)?),
            ))
        }(&v),
        TBresponse::failure(v) => |v_1: &TBerror| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_TBerror(&v_1)?),
            ))
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
            Ok(_validate_TBOK(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_TBerror(&el.inner()?)?)
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
///   info                   UTF8String  OPTIONAL,
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
        TagSelector::tag((TagClass::UNIVERSAL, 12)),
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
            "levelOfAssurance" => {
                levelOfAssurance_ = Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                    Ok(BER.decode_integer(&el.inner()?)?)
                }(_el)?)
            }
            "confidenceLevel" => {
                confidenceLevel_ = Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                    Ok(BER.decode_integer(&el.inner()?)?)
                }(_el)?)
            }
            "validationTime" => {
                validationTime_ = Some(|el: &X690Element| -> ASN1Result<UTCTime> {
                    Ok(BER.decode_utc_time(&el.inner()?)?)
                }(_el)?)
            }
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
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&BER.encode_integer(&v_1)?),
        ))
    }(&value_.levelOfAssurance)?);
    components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(&BER.encode_integer(&v_1)?),
        ))
    }(&value_.confidenceLevel)?);
    components_.push(|v_1: &UTCTime| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 2),
            X690Value::from_explicit(&BER.encode_utc_time(&v_1)?),
        ))
    }(&value_.validationTime)?);
    if let Some(v_) = &value_.info {
        components_.push(BER.encode_utf8_string(&v_)?);
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
                Ok(BER.validate_integer(&el.inner()?)?)
            }(_el)?,
            "confidenceLevel" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "confidenceLevel",
                    ));
                }
                Ok(BER.validate_integer(&el.inner()?)?)
            }(_el)?,
            "validationTime" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "validationTime")
                    );
                }
                Ok(BER.validate_utc_time(&el.inner()?)?)
            }(_el)?,
            "info" => BER.validate_utf8_string(_el)?,
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
/// CertRsp-result ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertRsp_result {
    success(CertOK),
    failure(CertErr),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for CertRsp_result {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertRsp_result(el)
    }
}

pub fn _decode_CertRsp_result(el: &X690Element) -> ASN1Result<CertRsp_result> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CertRsp_result::success(
            |el: &X690Element| -> ASN1Result<CertOK> { Ok(_decode_CertOK(&el.inner()?)?) }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CertRsp_result::failure(
            |el: &X690Element| -> ASN1Result<CertErr> { Ok(_decode_CertErr(&el.inner()?)?) }(&el)?,
        )),
        _ => Ok(CertRsp_result::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertRsp_result(value_: &CertRsp_result) -> ASN1Result<X690Element> {
    match value_ {
        CertRsp_result::success(v) => |v_1: &CertOK| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_CertOK(&v_1)?),
            ))
        }(&v),
        CertRsp_result::failure(v) => |v_1: &CertErr| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_CertErr(&v_1)?),
            ))
        }(&v),
        CertRsp_result::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertRsp_result(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "success"));
            }
            Ok(_validate_CertOK(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_CertErr(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertErr-notOK ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertErr_notOK {
    wrErr(PkiWaError),
    avmpErr(AVMP_error),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for CertErr_notOK {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertErr_notOK(el)
    }
}

pub fn _decode_CertErr_notOK(el: &X690Element) -> ASN1Result<CertErr_notOK> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CertErr_notOK::wrErr(
            |el: &X690Element| -> ASN1Result<PkiWaError> { Ok(_decode_PkiWaError(&el.inner()?)?) }(
                &el,
            )?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CertErr_notOK::avmpErr(
            |el: &X690Element| -> ASN1Result<AVMP_error> { Ok(_decode_AVMP_error(&el.inner()?)?) }(
                &el,
            )?,
        )),
        _ => Ok(CertErr_notOK::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertErr_notOK(value_: &CertErr_notOK) -> ASN1Result<X690Element> {
    match value_ {
        CertErr_notOK::wrErr(v) => |v_1: &PkiWaError| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_PkiWaError(&v_1)?),
            ))
        }(&v),
        CertErr_notOK::avmpErr(v) => |v_1: &AVMP_error| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_AVMP_error(&v_1)?),
            ))
        }(&v),
        CertErr_notOK::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertErr_notOK(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "wrErr"));
            }
            Ok(_validate_PkiWaError(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "avmpErr"));
            }
            Ok(_validate_AVMP_error(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
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

impl TryFrom<&X690Element> for AddAvlRsp_result {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AddAvlRsp_result(el)
    }
}

pub fn _decode_AddAvlRsp_result(el: &X690Element) -> ASN1Result<AddAvlRsp_result> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(AddAvlRsp_result::success(
            |el: &X690Element| -> ASN1Result<AddAvlOK> { Ok(_decode_AddAvlOK(&el.inner()?)?) }(
                &el,
            )?,
        )),
        (TagClass::CONTEXT, 1) => Ok(AddAvlRsp_result::failure(
            |el: &X690Element| -> ASN1Result<AddAvlErr> { Ok(_decode_AddAvlErr(&el.inner()?)?) }(
                &el,
            )?,
        )),
        _ => Ok(AddAvlRsp_result::_unrecognized(el.clone())),
    }
}

pub fn _encode_AddAvlRsp_result(value_: &AddAvlRsp_result) -> ASN1Result<X690Element> {
    match value_ {
        AddAvlRsp_result::success(v) => |v_1: &AddAvlOK| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_AddAvlOK(&v_1)?),
            ))
        }(&v),
        AddAvlRsp_result::failure(v) => |v_1: &AddAvlErr| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_AddAvlErr(&v_1)?),
            ))
        }(&v),
        AddAvlRsp_result::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_AddAvlRsp_result(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "success"));
            }
            Ok(_validate_AddAvlOK(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_AddAvlErr(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
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

impl TryFrom<&X690Element> for ReplaceAvlRsp_result {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ReplaceAvlRsp_result(el)
    }
}

pub fn _decode_ReplaceAvlRsp_result(el: &X690Element) -> ASN1Result<ReplaceAvlRsp_result> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(ReplaceAvlRsp_result::success(
            |el: &X690Element| -> ASN1Result<RepAvlOK> { Ok(_decode_RepAvlOK(&el.inner()?)?) }(
                &el,
            )?,
        )),
        (TagClass::CONTEXT, 1) => Ok(ReplaceAvlRsp_result::failure(
            |el: &X690Element| -> ASN1Result<RepAvlErr> { Ok(_decode_RepAvlErr(&el.inner()?)?) }(
                &el,
            )?,
        )),
        _ => Ok(ReplaceAvlRsp_result::_unrecognized(el.clone())),
    }
}

pub fn _encode_ReplaceAvlRsp_result(value_: &ReplaceAvlRsp_result) -> ASN1Result<X690Element> {
    match value_ {
        ReplaceAvlRsp_result::success(v) => |v_1: &RepAvlOK| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_RepAvlOK(&v_1)?),
            ))
        }(&v),
        ReplaceAvlRsp_result::failure(v) => |v_1: &RepAvlErr| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_RepAvlErr(&v_1)?),
            ))
        }(&v),
        ReplaceAvlRsp_result::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_ReplaceAvlRsp_result(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "success"));
            }
            Ok(_validate_RepAvlOK(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_RepAvlErr(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
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

impl TryFrom<&X690Element> for DeleteAvlRsp_result {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DeleteAvlRsp_result(el)
    }
}

pub fn _decode_DeleteAvlRsp_result(el: &X690Element) -> ASN1Result<DeleteAvlRsp_result> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(DeleteAvlRsp_result::success(
            |el: &X690Element| -> ASN1Result<DelAvlOK> { Ok(_decode_DelAvlOK(&el.inner()?)?) }(
                &el,
            )?,
        )),
        (TagClass::CONTEXT, 1) => Ok(DeleteAvlRsp_result::failure(
            |el: &X690Element| -> ASN1Result<DelAvlErr> { Ok(_decode_DelAvlErr(&el.inner()?)?) }(
                &el,
            )?,
        )),
        _ => Ok(DeleteAvlRsp_result::_unrecognized(el.clone())),
    }
}

pub fn _encode_DeleteAvlRsp_result(value_: &DeleteAvlRsp_result) -> ASN1Result<X690Element> {
    match value_ {
        DeleteAvlRsp_result::success(v) => |v_1: &DelAvlOK| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_DelAvlOK(&v_1)?),
            ))
        }(&v),
        DeleteAvlRsp_result::failure(v) => |v_1: &DelAvlErr| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_DelAvlErr(&v_1)?),
            ))
        }(&v),
        DeleteAvlRsp_result::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_DeleteAvlRsp_result(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "success"));
            }
            Ok(_validate_DelAvlOK(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_DelAvlErr(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
        (TagClass::CONTEXT, 0) => Ok(CertSubscribeRsp_result::success(
            |el: &X690Element| -> ASN1Result<CertSubscribeOK> {
                Ok(_decode_CertSubscribeOK(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CertSubscribeRsp_result::failure(
            |el: &X690Element| -> ASN1Result<CertSubscribeErr> {
                Ok(_decode_CertSubscribeErr(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(CertSubscribeRsp_result::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertSubscribeRsp_result(
    value_: &CertSubscribeRsp_result,
) -> ASN1Result<X690Element> {
    match value_ {
        CertSubscribeRsp_result::success(v) => |v_1: &CertSubscribeOK| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_CertSubscribeOK(&v_1)?),
            ))
        }(&v),
        CertSubscribeRsp_result::failure(v) => {
            |v_1: &CertSubscribeErr| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_CertSubscribeErr(&v_1)?),
                ))
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
            Ok(_validate_CertSubscribeOK(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_CertSubscribeErr(&el.inner()?)?)
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
        (TagClass::CONTEXT, 0) => Ok(CertSubscribeOK_Item::ok(|el: &X690Element| -> ASN1Result<
            CertSubscribeOK_Item_ok,
        > {
            Ok(_decode_CertSubscribeOK_Item_ok(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(CertSubscribeOK_Item::not_ok(
            |el: &X690Element| -> ASN1Result<CertSubscribeOK_Item_not_ok> {
                Ok(_decode_CertSubscribeOK_Item_not_ok(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(CertSubscribeOK_Item::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertSubscribeOK_Item(value_: &CertSubscribeOK_Item) -> ASN1Result<X690Element> {
    match value_ {
        CertSubscribeOK_Item::ok(v) => |v_1: &CertSubscribeOK_Item_ok| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_CertSubscribeOK_Item_ok(&v_1)?),
            ))
        }(&v),
        CertSubscribeOK_Item::not_ok(v) => {
            |v_1: &CertSubscribeOK_Item_not_ok| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_CertSubscribeOK_Item_not_ok(&v_1)?),
                ))
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
            Ok(_validate_CertSubscribeOK_Item_ok(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "not-ok"));
            }
            Ok(_validate_CertSubscribeOK_Item_not_ok(&el.inner()?)?)
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
            |el: &X690Element| -> ASN1Result<CertUnsubscribeOK> {
                Ok(_decode_CertUnsubscribeOK(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CertUnsubscribeRsp_result::failure(
            |el: &X690Element| -> ASN1Result<CertUnsubscribeErr> {
                Ok(_decode_CertUnsubscribeErr(&el.inner()?)?)
            }(&el)?,
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
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&_encode_CertUnsubscribeOK(&v_1)?),
                ))
            }(&v)
        }
        CertUnsubscribeRsp_result::failure(v) => {
            |v_1: &CertUnsubscribeErr| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_CertUnsubscribeErr(&v_1)?),
                ))
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
            Ok(_validate_CertUnsubscribeOK(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_CertUnsubscribeErr(&el.inner()?)?)
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
            |el: &X690Element| -> ASN1Result<CertUnsubscribeOK_Item_ok> {
                Ok(_decode_CertUnsubscribeOK_Item_ok(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CertUnsubscribeOK_Item::not_ok(
            |el: &X690Element| -> ASN1Result<CertUnsubscribeOK_Item_not_ok> {
                Ok(_decode_CertUnsubscribeOK_Item_not_ok(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(CertUnsubscribeOK_Item::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertUnsubscribeOK_Item(value_: &CertUnsubscribeOK_Item) -> ASN1Result<X690Element> {
    match value_ {
        CertUnsubscribeOK_Item::ok(v) => {
            |v_1: &CertUnsubscribeOK_Item_ok| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&_encode_CertUnsubscribeOK_Item_ok(&v_1)?),
                ))
            }(&v)
        }
        CertUnsubscribeOK_Item::not_ok(v) => {
            |v_1: &CertUnsubscribeOK_Item_not_ok| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_CertUnsubscribeOK_Item_not_ok(&v_1)?),
                ))
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
            Ok(_validate_CertUnsubscribeOK_Item_ok(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "not-ok"));
            }
            Ok(_validate_CertUnsubscribeOK_Item_not_ok(&el.inner()?)?)
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
        (TagClass::CONTEXT, 0) => Ok(CertReplaceRsp_result::success(
            |el: &X690Element| -> ASN1Result<CertReplaceOK> {
                Ok(_decode_CertReplaceOK(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CertReplaceRsp_result::failure(
            |el: &X690Element| -> ASN1Result<CertReplaceErr> {
                Ok(_decode_CertReplaceErr(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(CertReplaceRsp_result::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertReplaceRsp_result(value_: &CertReplaceRsp_result) -> ASN1Result<X690Element> {
    match value_ {
        CertReplaceRsp_result::success(v) => |v_1: &CertReplaceOK| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_CertReplaceOK(&v_1)?),
            ))
        }(&v),
        CertReplaceRsp_result::failure(v) => |v_1: &CertReplaceErr| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_CertReplaceErr(&v_1)?),
            ))
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
            Ok(_validate_CertReplaceOK(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_CertReplaceErr(&el.inner()?)?)
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
        (TagClass::CONTEXT, 0) => Ok(CertReplaceOK_Item::ok(|el: &X690Element| -> ASN1Result<
            CertReplaceOK_Item_ok,
        > {
            Ok(_decode_CertReplaceOK_Item_ok(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(CertReplaceOK_Item::not_ok(
            |el: &X690Element| -> ASN1Result<CertReplaceOK_Item_not_ok> {
                Ok(_decode_CertReplaceOK_Item_not_ok(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(CertReplaceOK_Item::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertReplaceOK_Item(value_: &CertReplaceOK_Item) -> ASN1Result<X690Element> {
    match value_ {
        CertReplaceOK_Item::ok(v) => |v_1: &CertReplaceOK_Item_ok| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_CertReplaceOK_Item_ok(&v_1)?),
            ))
        }(&v),
        CertReplaceOK_Item::not_ok(v) => {
            |v_1: &CertReplaceOK_Item_not_ok| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_CertReplaceOK_Item_not_ok(&v_1)?),
                ))
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
            Ok(_validate_CertReplaceOK_Item_ok(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "not-ok"));
            }
            Ok(_validate_CertReplaceOK_Item_not_ok(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CertReplaceErr-code ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum CertReplaceErr_code {
    signedData(SignedData_error),
    envelopedData(EnvelopedData_error),
    casp(CASP_error),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for CertReplaceErr_code {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CertReplaceErr_code(el)
    }
}

pub fn _decode_CertReplaceErr_code(el: &X690Element) -> ASN1Result<CertReplaceErr_code> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CertReplaceErr_code::signedData(
            |el: &X690Element| -> ASN1Result<SignedData_error> {
                Ok(_decode_SignedData_error(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CertReplaceErr_code::envelopedData(
            |el: &X690Element| -> ASN1Result<EnvelopedData_error> {
                Ok(_decode_EnvelopedData_error(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(CertReplaceErr_code::casp(
            |el: &X690Element| -> ASN1Result<CASP_error> { Ok(_decode_CASP_error(&el.inner()?)?) }(
                &el,
            )?,
        )),
        _ => Ok(CertReplaceErr_code::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertReplaceErr_code(value_: &CertReplaceErr_code) -> ASN1Result<X690Element> {
    match value_ {
        CertReplaceErr_code::signedData(v) => |v_1: &SignedData_error| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_SignedData_error(&v_1)?),
            ))
        }(&v),
        CertReplaceErr_code::envelopedData(v) => {
            |v_1: &EnvelopedData_error| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_EnvelopedData_error(&v_1)?),
                ))
            }(&v)
        }
        CertReplaceErr_code::casp(v) => |v_1: &CASP_error| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(&_encode_CASP_error(&v_1)?),
            ))
        }(&v),
        CertReplaceErr_code::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CertReplaceErr_code(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "signedData"));
            }
            Ok(_validate_SignedData_error(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "envelopedData")
                );
            }
            Ok(_validate_EnvelopedData_error(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "casp"));
            }
            Ok(_validate_CASP_error(&el.inner()?)?)
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
        (TagClass::CONTEXT, 0) => Ok(CertUpdateRsp_result::success(
            |el: &X690Element| -> ASN1Result<CertUpdateOK> {
                Ok(_decode_CertUpdateOK(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CertUpdateRsp_result::failure(
            |el: &X690Element| -> ASN1Result<CertUpdateErr> {
                Ok(_decode_CertUpdateErr(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(CertUpdateRsp_result::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertUpdateRsp_result(value_: &CertUpdateRsp_result) -> ASN1Result<X690Element> {
    match value_ {
        CertUpdateRsp_result::success(v) => |v_1: &CertUpdateOK| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_CertUpdateOK(&v_1)?),
            ))
        }(&v),
        CertUpdateRsp_result::failure(v) => |v_1: &CertUpdateErr| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_CertUpdateErr(&v_1)?),
            ))
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
            Ok(_validate_CertUpdateOK(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "failure"));
            }
            Ok(_validate_CertUpdateErr(&el.inner()?)?)
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
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
        (TagClass::CONTEXT, 0) => Ok(CertUpdateOK_Item::ok(|el: &X690Element| -> ASN1Result<
            CertUpdateOK_Item_ok,
        > {
            Ok(_decode_CertUpdateOK_Item_ok(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(CertUpdateOK_Item::not_ok(|el: &X690Element| -> ASN1Result<
            CertUpdateOK_Item_not_ok,
        > {
            Ok(_decode_CertUpdateOK_Item_not_ok(&el.inner()?)?)
        }(&el)?)),
        _ => Ok(CertUpdateOK_Item::_unrecognized(el.clone())),
    }
}

pub fn _encode_CertUpdateOK_Item(value_: &CertUpdateOK_Item) -> ASN1Result<X690Element> {
    match value_ {
        CertUpdateOK_Item::ok(v) => |v_1: &CertUpdateOK_Item_ok| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_CertUpdateOK_Item_ok(&v_1)?),
            ))
        }(&v),
        CertUpdateOK_Item::not_ok(v) => {
            |v_1: &CertUpdateOK_Item_not_ok| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_CertUpdateOK_Item_not_ok(&v_1)?),
                ))
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
            Ok(_validate_CertUpdateOK_Item_ok(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "not-ok"));
            }
            Ok(_validate_CertUpdateOK_Item_not_ok(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
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
