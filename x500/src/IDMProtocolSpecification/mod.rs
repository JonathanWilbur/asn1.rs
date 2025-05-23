#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # IDMProtocolSpecification
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `IDMProtocolSpecification`.
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
use crate::CommonProtocolSpecification::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// IDM-PDU{IDM-PROTOCOL:protocol}  ::=  CHOICE {
///   bind         [0]  IdmBind{{protocol}},
///   bindResult   [1]  IdmBindResult{{protocol}},
///   bindError    [2]  IdmBindError{{protocol}},
///   request      [3]  Request{{protocol.&Operations}},
///   result       [4]  IdmResult{{protocol.&Operations}},
///   error        [5]  Error{{protocol.&Operations}},
///   reject       [6]  IdmReject,
///   unbind       [7]  Unbind,
///   abort        [8]  Abort,
///   startTLS     [9]  StartTLS,
///   tLSResponse  [10] TLSResponse,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum IDM_PDU {
    bind(IdmBind),
    bindResult(IdmBindResult),
    bindError(IdmBindError),
    request(Request),
    result(IdmResult),
    error(Error),
    reject(IdmReject),
    unbind(Unbind),
    abort(Abort),
    startTLS(StartTLS),
    tLSResponse(TLSResponse),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for IDM_PDU {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_IDM_PDU(el)
    }
}

pub fn _decode_IDM_PDU(el: &X690Element) -> ASN1Result<IDM_PDU> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(IDM_PDU::bind(|el: &X690Element| -> ASN1Result<IdmBind> {
            Ok(_decode_IdmBind(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(IDM_PDU::bindResult(
            |el: &X690Element| -> ASN1Result<IdmBindResult> {
                Ok(_decode_IdmBindResult(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(IDM_PDU::bindError(
            |el: &X690Element| -> ASN1Result<IdmBindError> {
                Ok(_decode_IdmBindError(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 3) => Ok(IDM_PDU::request(
            |el: &X690Element| -> ASN1Result<Request> { Ok(_decode_Request(&el.inner()?)?) }(&el)?,
        )),
        (TagClass::CONTEXT, 4) => Ok(IDM_PDU::result(
            |el: &X690Element| -> ASN1Result<IdmResult> { Ok(_decode_IdmResult(&el.inner()?)?) }(
                &el,
            )?,
        )),
        (TagClass::CONTEXT, 5) => Ok(IDM_PDU::error(|el: &X690Element| -> ASN1Result<Error> {
            Ok(_decode_Error(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 6) => Ok(IDM_PDU::reject(
            |el: &X690Element| -> ASN1Result<IdmReject> { Ok(_decode_IdmReject(&el.inner()?)?) }(
                &el,
            )?,
        )),
        (TagClass::CONTEXT, 7) => Ok(IDM_PDU::unbind(|el: &X690Element| -> ASN1Result<Unbind> {
            Ok(_decode_Unbind(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 8) => Ok(IDM_PDU::abort(|el: &X690Element| -> ASN1Result<Abort> {
            Ok(_decode_Abort(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 9) => Ok(IDM_PDU::startTLS(
            |el: &X690Element| -> ASN1Result<StartTLS> { Ok(_decode_StartTLS(&el.inner()?)?) }(
                &el,
            )?,
        )),
        (TagClass::CONTEXT, 10) => Ok(IDM_PDU::tLSResponse(
            |el: &X690Element| -> ASN1Result<TLSResponse> {
                Ok(_decode_TLSResponse(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(IDM_PDU::_unrecognized(el.clone())),
    }
}

pub fn _encode_IDM_PDU(value_: &IDM_PDU) -> ASN1Result<X690Element> {
    match value_ {
        IDM_PDU::bind(v) => |v_1: &IdmBind| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_IdmBind(&v_1)?),
            ))
        }(&v),
        IDM_PDU::bindResult(v) => |v_1: &IdmBindResult| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_IdmBindResult(&v_1)?),
            ))
        }(&v),
        IDM_PDU::bindError(v) => |v_1: &IdmBindError| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(&_encode_IdmBindError(&v_1)?),
            ))
        }(&v),
        IDM_PDU::request(v) => |v_1: &Request| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(&_encode_Request(&v_1)?),
            ))
        }(&v),
        IDM_PDU::result(v) => |v_1: &IdmResult| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::from_explicit(&_encode_IdmResult(&v_1)?),
            ))
        }(&v),
        IDM_PDU::error(v) => |v_1: &Error| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 5),
                X690Value::from_explicit(&_encode_Error(&v_1)?),
            ))
        }(&v),
        IDM_PDU::reject(v) => |v_1: &IdmReject| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 6),
                X690Value::from_explicit(&_encode_IdmReject(&v_1)?),
            ))
        }(&v),
        IDM_PDU::unbind(v) => |v_1: &Unbind| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 7),
                X690Value::from_explicit(&_encode_Unbind(&v_1)?),
            ))
        }(&v),
        IDM_PDU::abort(v) => |v_1: &Abort| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 8),
                X690Value::from_explicit(&_encode_Abort(&v_1)?),
            ))
        }(&v),
        IDM_PDU::startTLS(v) => |v_1: &StartTLS| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 9),
                X690Value::from_explicit(&_encode_StartTLS(&v_1)?),
            ))
        }(&v),
        IDM_PDU::tLSResponse(v) => |v_1: &TLSResponse| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 10),
                X690Value::from_explicit(&_encode_TLSResponse(&v_1)?),
            ))
        }(&v),
        IDM_PDU::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_IDM_PDU(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "bind"));
            }
            Ok(_validate_IdmBind(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "bindResult"));
            }
            Ok(_validate_IdmBindResult(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "bindError"));
            }
            Ok(_validate_IdmBindError(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "request"));
            }
            Ok(_validate_Request(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 4) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "result"));
            }
            Ok(_validate_IdmResult(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 5) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "error"));
            }
            Ok(_validate_Error(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 6) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "reject"));
            }
            Ok(_validate_IdmReject(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 7) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "unbind"));
            }
            Ok(_validate_Unbind(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 8) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 8 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "abort"));
            }
            Ok(_validate_Abort(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 9) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 9 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "startTLS"));
            }
            Ok(_validate_StartTLS(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 10) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 10 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "tLSResponse")
                );
            }
            Ok(_validate_TLSResponse(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IdmBind{IDM-PROTOCOL:Protocols} ::= SEQUENCE {
///   protocolID           IDM-PROTOCOL.&id({Protocols}),
///   callingAETitle  [0]  GeneralName OPTIONAL,
///   calledAETitle   [1]  GeneralName OPTIONAL,
///   argument        [2]  IDM-PROTOCOL.&bind-operation.&ArgumentType
///                          ({Protocols}{@protocolID}),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct IdmBind {
    pub protocolID: OBJECT_IDENTIFIER,
    pub callingAETitle: OPTIONAL<GeneralName>,
    pub calledAETitle: OPTIONAL<GeneralName>,
    pub argument: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl IdmBind {
    pub fn new(
        protocolID: OBJECT_IDENTIFIER,
        callingAETitle: OPTIONAL<GeneralName>,
        calledAETitle: OPTIONAL<GeneralName>,
        argument: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        IdmBind {
            protocolID,
            callingAETitle,
            calledAETitle,
            argument,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for IdmBind {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_IdmBind(el)
    }
}

pub const _rctl1_components_for_IdmBind: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "protocolID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "callingAETitle",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "calledAETitle",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "argument",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IdmBind: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IdmBind: &[ComponentSpec; 0] = &[];

pub fn _decode_IdmBind(el: &X690Element) -> ASN1Result<IdmBind> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdmBind")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdmBind,
        _eal_components_for_IdmBind,
        _rctl2_components_for_IdmBind,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut protocolID_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut callingAETitle_: OPTIONAL<GeneralName> = None;
    let mut calledAETitle_: OPTIONAL<GeneralName> = None;
    let mut argument_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protocolID" => protocolID_ = Some(BER.decode_object_identifier(_el)?),
            "callingAETitle" => {
                callingAETitle_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "calledAETitle" => {
                calledAETitle_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "argument" => {
                argument_ = Some(|el: &X690Element| -> ASN1Result<X690Element> {
                    Ok(x690_identity(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(IdmBind {
        protocolID: protocolID_.unwrap(),
        callingAETitle: callingAETitle_,
        calledAETitle: calledAETitle_,
        argument: argument_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_IdmBind(value_: &IdmBind) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(BER.encode_object_identifier(&value_.protocolID)?);
    if let Some(v_) = &value_.callingAETitle {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.calledAETitle {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v_)?);
    }
    components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 2),
            X690Value::from_explicit(&x690_identity(&v_1)?),
        ))
    }(&value_.argument)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_IdmBind(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdmBind")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdmBind,
        _eal_components_for_IdmBind,
        _rctl2_components_for_IdmBind,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protocolID" => BER.validate_object_identifier(_el)?,
            "callingAETitle" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "callingAETitle")
                    );
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "calledAETitle" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "calledAETitle")
                    );
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "argument" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "argument")
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
/// IdmBindResult{IDM-PROTOCOL:Protocols} ::= SEQUENCE {
///   protocolID              IDM-PROTOCOL.&id({Protocols}),
///   respondingAETitle  [0]  GeneralName OPTIONAL,
///   result             [1]  IDM-PROTOCOL.&bind-operation.&ResultType
///                             ({Protocols}{@protocolID}),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct IdmBindResult {
    pub protocolID: OBJECT_IDENTIFIER,
    pub respondingAETitle: OPTIONAL<GeneralName>,
    pub result: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl IdmBindResult {
    pub fn new(
        protocolID: OBJECT_IDENTIFIER,
        respondingAETitle: OPTIONAL<GeneralName>,
        result: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        IdmBindResult {
            protocolID,
            respondingAETitle,
            result,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for IdmBindResult {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_IdmBindResult(el)
    }
}

pub const _rctl1_components_for_IdmBindResult: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "protocolID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "respondingAETitle",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "result",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IdmBindResult: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IdmBindResult: &[ComponentSpec; 0] = &[];

pub fn _decode_IdmBindResult(el: &X690Element) -> ASN1Result<IdmBindResult> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdmBindResult")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdmBindResult,
        _eal_components_for_IdmBindResult,
        _rctl2_components_for_IdmBindResult,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut protocolID_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut respondingAETitle_: OPTIONAL<GeneralName> = None;
    let mut result_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protocolID" => protocolID_ = Some(BER.decode_object_identifier(_el)?),
            "respondingAETitle" => {
                respondingAETitle_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "result" => {
                result_ = Some(|el: &X690Element| -> ASN1Result<X690Element> {
                    Ok(x690_identity(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(IdmBindResult {
        protocolID: protocolID_.unwrap(),
        respondingAETitle: respondingAETitle_,
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_IdmBindResult(value_: &IdmBindResult) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_object_identifier(&value_.protocolID)?);
    if let Some(v_) = &value_.respondingAETitle {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v_)?);
    }
    components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(&x690_identity(&v_1)?),
        ))
    }(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_IdmBindResult(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdmBindResult")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdmBindResult,
        _eal_components_for_IdmBindResult,
        _rctl2_components_for_IdmBindResult,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protocolID" => BER.validate_object_identifier(_el)?,
            "respondingAETitle" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "respondingAETitle",
                    ));
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "result" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "result"));
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
/// IdmBindError{IDM-PROTOCOL:Protocols} ::= SEQUENCE {
///   protocolID              IDM-PROTOCOL.&id({Protocols}),
/// --errcode                 IDM-PROTOCOL.&bind-operation.&Errors.&errorCode OPTIONAL
///   respondingAETitle  [0]  GeneralName OPTIONAL,
///   aETitleError            ENUMERATED {
///     callingAETitleNotAccepted  (0),
///     calledAETitleNotRecognized (1),
///     ...} OPTIONAL,
///   error              [1]  IDM-PROTOCOL.&bind-operation.&Errors.&ParameterType
///                             ({Protocols}{@protocolID}),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct IdmBindError {
    pub protocolID: OBJECT_IDENTIFIER,
    pub respondingAETitle: OPTIONAL<GeneralName>,
    pub aETitleError: OPTIONAL<IdmBindError_aETitleError>,
    pub error: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl IdmBindError {
    pub fn new(
        protocolID: OBJECT_IDENTIFIER,
        respondingAETitle: OPTIONAL<GeneralName>,
        aETitleError: OPTIONAL<IdmBindError_aETitleError>,
        error: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        IdmBindError {
            protocolID,
            respondingAETitle,
            aETitleError,
            error,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for IdmBindError {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_IdmBindError(el)
    }
}

pub const _rctl1_components_for_IdmBindError: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "protocolID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "respondingAETitle",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aETitleError",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "error",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IdmBindError: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IdmBindError: &[ComponentSpec; 0] = &[];

pub fn _decode_IdmBindError(el: &X690Element) -> ASN1Result<IdmBindError> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdmBindError")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdmBindError,
        _eal_components_for_IdmBindError,
        _rctl2_components_for_IdmBindError,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut protocolID_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut respondingAETitle_: OPTIONAL<GeneralName> = None;
    let mut aETitleError_: OPTIONAL<IdmBindError_aETitleError> = None;
    let mut error_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protocolID" => protocolID_ = Some(BER.decode_object_identifier(_el)?),
            "respondingAETitle" => {
                respondingAETitle_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "aETitleError" => aETitleError_ = Some(_decode_IdmBindError_aETitleError(_el)?),
            "error" => {
                error_ = Some(|el: &X690Element| -> ASN1Result<X690Element> {
                    Ok(x690_identity(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(IdmBindError {
        protocolID: protocolID_.unwrap(),
        respondingAETitle: respondingAETitle_,
        aETitleError: aETitleError_,
        error: error_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_IdmBindError(value_: &IdmBindError) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(BER.encode_object_identifier(&value_.protocolID)?);
    if let Some(v_) = &value_.respondingAETitle {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.aETitleError {
        components_.push(_encode_IdmBindError_aETitleError(&v_)?);
    }
    components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(&x690_identity(&v_1)?),
        ))
    }(&value_.error)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_IdmBindError(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdmBindError")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdmBindError,
        _eal_components_for_IdmBindError,
        _rctl2_components_for_IdmBindError,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protocolID" => BER.validate_object_identifier(_el)?,
            "respondingAETitle" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "respondingAETitle",
                    ));
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "aETitleError" => _validate_IdmBindError_aETitleError(_el)?,
            "error" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "error"));
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
/// Request{OPERATION:Operations} ::= SEQUENCE {
///   invokeID  INTEGER,
///   opcode    OPERATION.&operationCode({Operations}),
///   argument  OPERATION.&ArgumentType({Operations}{@opcode}),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Request {
    pub invokeID: i64,
    pub opcode: Code,
    pub argument: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl Request {
    pub fn new(
        invokeID: i64,
        opcode: Code,
        argument: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Request {
            invokeID,
            opcode,
            argument,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for Request {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Request(el)
    }
}

pub const _rctl1_components_for_Request: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("opcode", false, TagSelector::any, None, None),
    ComponentSpec::new("argument", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_Request: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Request: &[ComponentSpec; 0] = &[];

pub fn _decode_Request(el: &X690Element) -> ASN1Result<Request> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Request")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Request,
        _eal_components_for_Request,
        _rctl2_components_for_Request,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<i64> = None;
    let mut opcode_: OPTIONAL<Code> = None;
    let mut argument_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(BER.decode_i64(_el)?),
            "opcode" => opcode_ = Some(_decode_Code(_el)?),
            "argument" => argument_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Request {
        invokeID: invokeID_.unwrap(),
        opcode: opcode_.unwrap(),
        argument: argument_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_Request(value_: &Request) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_i64(value_.invokeID)?);
    components_.push(_encode_Code(&value_.opcode)?);
    components_.push(x690_identity(&value_.argument)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Request(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Request")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Request,
        _eal_components_for_Request,
        _rctl2_components_for_Request,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => BER.validate_i64(_el)?,
            "opcode" => _validate_Code(_el)?,
            "argument" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IdmResult{OPERATION:Operations} ::= SEQUENCE {
///   invokeID  INTEGER,
///   opcode    OPERATION.&operationCode({Operations}),
///   result    OPERATION.&ResultType({Operations}{@opcode}),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct IdmResult {
    pub invokeID: i64,
    pub opcode: Code,
    pub result: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl IdmResult {
    pub fn new(
        invokeID: i64,
        opcode: Code,
        result: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        IdmResult {
            invokeID,
            opcode,
            result,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for IdmResult {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_IdmResult(el)
    }
}

pub const _rctl1_components_for_IdmResult: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("opcode", false, TagSelector::any, None, None),
    ComponentSpec::new("result", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_IdmResult: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IdmResult: &[ComponentSpec; 0] = &[];

pub fn _decode_IdmResult(el: &X690Element) -> ASN1Result<IdmResult> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdmResult")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdmResult,
        _eal_components_for_IdmResult,
        _rctl2_components_for_IdmResult,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<i64> = None;
    let mut opcode_: OPTIONAL<Code> = None;
    let mut result_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(BER.decode_i64(_el)?),
            "opcode" => opcode_ = Some(_decode_Code(_el)?),
            "result" => result_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(IdmResult {
        invokeID: invokeID_.unwrap(),
        opcode: opcode_.unwrap(),
        result: result_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_IdmResult(value_: &IdmResult) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_i64(value_.invokeID)?);
    components_.push(_encode_Code(&value_.opcode)?);
    components_.push(x690_identity(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_IdmResult(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdmResult")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdmResult,
        _eal_components_for_IdmResult,
        _rctl2_components_for_IdmResult,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => BER.validate_i64(_el)?,
            "opcode" => _validate_Code(_el)?,
            "result" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Error{OPERATION:Operations} ::= SEQUENCE {
///   invokeID  INTEGER,
///   errcode   OPERATION.&Errors.&errorCode({Operations}),
///   error     OPERATION.&Errors.&ParameterType({Operations}{@errcode}),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Error {
    pub invokeID: i64,
    pub errcode: Code,
    pub error: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl Error {
    pub fn new(
        invokeID: i64,
        errcode: Code,
        error: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Error {
            invokeID,
            errcode,
            error,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for Error {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Error(el)
    }
}

pub const _rctl1_components_for_Error: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "invokeID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("errcode", false, TagSelector::any, None, None),
    ComponentSpec::new("error", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_Error: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Error: &[ComponentSpec; 0] = &[];

pub fn _decode_Error(el: &X690Element) -> ASN1Result<Error> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Error")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Error,
        _eal_components_for_Error,
        _rctl2_components_for_Error,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<i64> = None;
    let mut errcode_: OPTIONAL<Code> = None;
    let mut error_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(BER.decode_i64(_el)?),
            "errcode" => errcode_ = Some(_decode_Code(_el)?),
            "error" => error_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Error {
        invokeID: invokeID_.unwrap(),
        errcode: errcode_.unwrap(),
        error: error_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_Error(value_: &Error) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_i64(value_.invokeID)?);
    components_.push(_encode_Code(&value_.errcode)?);
    components_.push(x690_identity(&value_.error)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Error(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Error")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Error,
        _eal_components_for_Error,
        _rctl2_components_for_Error,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => BER.validate_i64(_el)?,
            "errcode" => _validate_Code(_el)?,
            "error" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IdmReject ::= SEQUENCE {
///   invokeID  INTEGER,
///   reason    ENUMERATED {
///     mistypedPDU                 (0),
///     duplicateInvokeIDRequest    (1),
///     unsupportedOperationRequest (2),
///     unknownOperationRequest     (3),
///     mistypedArgumentRequest     (4),
///     resourceLimitationRequest   (5),
///     unknownInvokeIDResult       (6),
///     mistypedResultRequest       (7),
///     unknownInvokeIDError        (8),
///     unknownError                (9),
///     mistypedParameterError      (10),
///     unsupportedIdmVersion       (11),
///     unsuitableIdmVersion        (12),
///     invalidIdmVersion           (13),
///     ...},
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct IdmReject {
    pub invokeID: i64,
    pub reason: IdmReject_reason,
    pub _unrecognized: Vec<X690Element>,
}
impl IdmReject {
    pub fn new(
        invokeID: i64,
        reason: IdmReject_reason,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        IdmReject {
            invokeID,
            reason,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for IdmReject {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_IdmReject(el)
    }
}

pub const _rctl1_components_for_IdmReject: &[ComponentSpec; 2] = &[
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

pub const _rctl2_components_for_IdmReject: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IdmReject: &[ComponentSpec; 0] = &[];

pub fn _decode_IdmReject(el: &X690Element) -> ASN1Result<IdmReject> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdmReject")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdmReject,
        _eal_components_for_IdmReject,
        _rctl2_components_for_IdmReject,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut invokeID_: OPTIONAL<i64> = None;
    let mut reason_: OPTIONAL<IdmReject_reason> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => invokeID_ = Some(BER.decode_i64(_el)?),
            "reason" => reason_ = Some(_decode_IdmReject_reason(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(IdmReject {
        invokeID: invokeID_.unwrap(),
        reason: reason_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_IdmReject(value_: &IdmReject) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_i64(value_.invokeID)?);
    components_.push(_encode_IdmReject_reason(&value_.reason)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_IdmReject(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "IdmReject")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_IdmReject,
        _eal_components_for_IdmReject,
        _rctl2_components_for_IdmReject,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "invokeID" => BER.validate_i64(_el)?,
            "reason" => _validate_IdmReject_reason(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Unbind  ::=  NULL
/// ```
pub type Unbind = NULL; // NullType

pub fn _decode_Unbind(el: &X690Element) -> ASN1Result<Unbind> {
    BER.decode_null(&el)
}

pub fn _encode_Unbind(value_: &Unbind) -> ASN1Result<X690Element> {
    BER.encode_null(&value_)
}

pub fn _validate_Unbind(el: &X690Element) -> ASN1Result<()> {
    BER.validate_null(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Abort  ::=  ENUMERATED {
///   mistypedPDU         (0),
///   unboundRequest      (1),
///   invalidPDU          (2),
///   resourceLimitation  (3),
///   connectionFailed    (4),
///   invalidProtocol     (5),
///   reasonNotSpecified  (6),
///   ...}
/// ```
pub type Abort = ENUMERATED;

pub const Abort_mistypedPDU: Abort = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const Abort_unboundRequest: Abort = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const Abort_invalidPDU: Abort = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const Abort_resourceLimitation: Abort = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const Abort_connectionFailed: Abort = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const Abort_invalidProtocol: Abort = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const Abort_reasonNotSpecified: Abort = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_Abort(el: &X690Element) -> ASN1Result<Abort> {
    BER.decode_enumerated(&el)
}

pub fn _encode_Abort(value_: &Abort) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_Abort(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// StartTLS  ::=  NULL
/// ```
pub type StartTLS = NULL; // NullType

pub fn _decode_StartTLS(el: &X690Element) -> ASN1Result<StartTLS> {
    BER.decode_null(&el)
}

pub fn _encode_StartTLS(value_: &StartTLS) -> ASN1Result<X690Element> {
    BER.encode_null(&value_)
}

pub fn _validate_StartTLS(el: &X690Element) -> ASN1Result<()> {
    BER.validate_null(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TLSResponse  ::=  ENUMERATED {
///   success         (0),
///   operationsError (1),
///   protocolError   (2),
///   unavailable     (3),
///   ...}
/// ```
pub type TLSResponse = ENUMERATED;

pub const TLSResponse_success: TLSResponse = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const TLSResponse_operationsError: TLSResponse = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const TLSResponse_protocolError: TLSResponse = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const TLSResponse_unavailable: TLSResponse = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_TLSResponse(el: &X690Element) -> ASN1Result<TLSResponse> {
    BER.decode_enumerated(&el)
}

pub fn _encode_TLSResponse(value_: &TLSResponse) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_TLSResponse(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IDM-PROTOCOL ::= CLASS {
///   &bind-operation  OPERATION,
///   &Operations      OPERATION,
///   &id              OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   BIND-OPERATION   &bind-operation
///   OPERATIONS       &Operations
///   ID               &id }
/// ```
///
#[derive(Debug)]
pub struct IDM_PROTOCOL {
    pub bind_operation: OPERATION,
    pub Operations: Vec<OPERATION>,
    pub id: OBJECT_IDENTIFIER,
}
impl IDM_PROTOCOL {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IdmBindError-aETitleError ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type IdmBindError_aETitleError = ENUMERATED;

pub const IdmBindError_aETitleError_callingAETitleNotAccepted: IdmBindError_aETitleError = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmBindError_aETitleError_calledAETitleNotRecognized: IdmBindError_aETitleError = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_IdmBindError_aETitleError(
    el: &X690Element,
) -> ASN1Result<IdmBindError_aETitleError> {
    BER.decode_enumerated(&el)
}

pub fn _encode_IdmBindError_aETitleError(
    value_: &IdmBindError_aETitleError,
) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_IdmBindError_aETitleError(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IdmReject-reason ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type IdmReject_reason = ENUMERATED;

pub const IdmReject_reason_mistypedPDU: IdmReject_reason = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_duplicateInvokeIDRequest: IdmReject_reason = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_unsupportedOperationRequest: IdmReject_reason = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_unknownOperationRequest: IdmReject_reason = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_mistypedArgumentRequest: IdmReject_reason = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_resourceLimitationRequest: IdmReject_reason = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_unknownInvokeIDResult: IdmReject_reason = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_mistypedResultRequest: IdmReject_reason = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_unknownInvokeIDError: IdmReject_reason = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_unknownError: IdmReject_reason = 9; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_mistypedParameterError: IdmReject_reason = 10; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_unsupportedIdmVersion: IdmReject_reason = 11; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_unsuitableIdmVersion: IdmReject_reason = 12; /* LONG_NAMED_ENUMERATED_VALUE */

pub const IdmReject_reason_invalidIdmVersion: IdmReject_reason = 13; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_IdmReject_reason(el: &X690Element) -> ASN1Result<IdmReject_reason> {
    BER.decode_enumerated(&el)
}

pub fn _encode_IdmReject_reason(value_: &IdmReject_reason) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_IdmReject_reason(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}
