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
use std::borrow::Borrow;
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

impl TryFrom<X690Element> for IDM_PDU {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IDM_PDU(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IDM_PDU {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_IDM_PDU(el)
    }
}

pub fn _decode_IDM_PDU(el: &X690Element) -> ASN1Result<IDM_PDU> {
    |el: &X690Element| -> ASN1Result<IDM_PDU> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(IDM_PDU::bind(_decode_IdmBind(&el.inner()?)?)),
            (TagClass::CONTEXT, 1) => Ok(IDM_PDU::bindResult(_decode_IdmBindResult(&el.inner()?)?)),
            (TagClass::CONTEXT, 2) => Ok(IDM_PDU::bindError(_decode_IdmBindError(&el.inner()?)?)),
            (TagClass::CONTEXT, 3) => Ok(IDM_PDU::request(_decode_Request(&el.inner()?)?)),
            (TagClass::CONTEXT, 4) => Ok(IDM_PDU::result(_decode_IdmResult(&el.inner()?)?)),
            (TagClass::CONTEXT, 5) => Ok(IDM_PDU::error(_decode_Error(&el.inner()?)?)),
            (TagClass::CONTEXT, 6) => Ok(IDM_PDU::reject(_decode_IdmReject(&el.inner()?)?)),
            (TagClass::CONTEXT, 7) => Ok(IDM_PDU::unbind(_decode_Unbind(&el.inner()?)?)),
            (TagClass::CONTEXT, 8) => Ok(IDM_PDU::abort(_decode_Abort(&el.inner()?)?)),
            (TagClass::CONTEXT, 9) => Ok(IDM_PDU::startTLS(_decode_StartTLS(&el.inner()?)?)),
            (TagClass::CONTEXT, 10) => Ok(IDM_PDU::tLSResponse(_decode_TLSResponse(&el.inner()?)?)),
            _ => Ok(IDM_PDU::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_IDM_PDU(value_: &IDM_PDU) -> ASN1Result<X690Element> {
    |value: &IDM_PDU| -> ASN1Result<X690Element> {
        match value {
            IDM_PDU::bind(v) => |v_1: &IdmBind| -> ASN1Result<X690Element> {
                let el_1 = _encode_IdmBind(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            IDM_PDU::bindResult(v) => |v_1: &IdmBindResult| -> ASN1Result<X690Element> {
                let el_1 = _encode_IdmBindResult(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            IDM_PDU::bindError(v) => |v_1: &IdmBindError| -> ASN1Result<X690Element> {
                let el_1 = _encode_IdmBindError(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            IDM_PDU::request(v) => |v_1: &Request| -> ASN1Result<X690Element> {
                let el_1 = _encode_Request(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            IDM_PDU::result(v) => |v_1: &IdmResult| -> ASN1Result<X690Element> {
                let el_1 = _encode_IdmResult(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            IDM_PDU::error(v) => |v_1: &Error| -> ASN1Result<X690Element> {
                let el_1 = _encode_Error(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            IDM_PDU::reject(v) => |v_1: &IdmReject| -> ASN1Result<X690Element> {
                let el_1 = _encode_IdmReject(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    6,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            IDM_PDU::unbind(v) => |v_1: &Unbind| -> ASN1Result<X690Element> {
                let el_1 = _encode_Unbind(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    7,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            IDM_PDU::abort(v) => |v_1: &Abort| -> ASN1Result<X690Element> {
                let el_1 = _encode_Abort(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    8,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            IDM_PDU::startTLS(v) => |v_1: &StartTLS| -> ASN1Result<X690Element> {
                let el_1 = _encode_StartTLS(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    9,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            IDM_PDU::tLSResponse(v) => |v_1: &TLSResponse| -> ASN1Result<X690Element> {
                let el_1 = _encode_TLSResponse(&v_1)?;
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    10,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                ))
            }(&v),
            IDM_PDU::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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
impl TryFrom<X690Element> for IdmBind {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IdmBind(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IdmBind {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<IdmBind> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IdmBind,
            _eal_components_for_IdmBind,
            _rctl2_components_for_IdmBind,
        )?;
        let protocolID = ber_decode_object_identifier(_components.get("protocolID").unwrap())?;
        let callingAETitle: OPTIONAL<GeneralName> = match _components.get("callingAETitle") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                Ok(_decode_GeneralName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let calledAETitle: OPTIONAL<GeneralName> = match _components.get("calledAETitle") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                Ok(_decode_GeneralName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let argument =
            |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                _components.get("argument").unwrap(),
            )?;
        Ok(IdmBind {
            protocolID,
            callingAETitle,
            calledAETitle,
            argument,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_IdmBind(value_: &IdmBind) -> ASN1Result<X690Element> {
    |value_: &IdmBind| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(ber_encode_object_identifier(&value_.protocolID)?);
        if let Some(v_) = &value_.callingAETitle {
            components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.calledAETitle {
            components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v_)?);
        }
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                2,
                Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
            ))
        }(&value_.argument)?);
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
/// IdmBindResult{IDM-PROTOCOL:Protocols} ::= SEQUENCE {
///   protocolID              IDM-PROTOCOL.&id({Protocols}),
///   respondingAETitle  [0]  GeneralName OPTIONAL,
///   result             [1]  IDM-PROTOCOL.&bind-operation.&ResultType
///                             ({Protocols}{@protocolID}),
///   ... }
/// ```
///
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
impl TryFrom<X690Element> for IdmBindResult {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IdmBindResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IdmBindResult {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<IdmBindResult> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IdmBindResult,
            _eal_components_for_IdmBindResult,
            _rctl2_components_for_IdmBindResult,
        )?;
        let protocolID = ber_decode_object_identifier(_components.get("protocolID").unwrap())?;
        let respondingAETitle: OPTIONAL<GeneralName> = match _components.get("respondingAETitle") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                Ok(_decode_GeneralName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let result =
            |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                _components.get("result").unwrap(),
            )?;
        Ok(IdmBindResult {
            protocolID,
            respondingAETitle,
            result,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_IdmBindResult(value_: &IdmBindResult) -> ASN1Result<X690Element> {
    |value_: &IdmBindResult| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(ber_encode_object_identifier(&value_.protocolID)?);
        if let Some(v_) = &value_.respondingAETitle {
            components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v_)?);
        }
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
            ))
        }(&value_.result)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for IdmBindError {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IdmBindError(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IdmBindError {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<IdmBindError> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IdmBindError,
            _eal_components_for_IdmBindError,
            _rctl2_components_for_IdmBindError,
        )?;
        let protocolID = ber_decode_object_identifier(_components.get("protocolID").unwrap())?;
        let respondingAETitle: OPTIONAL<GeneralName> = match _components.get("respondingAETitle") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                Ok(_decode_GeneralName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let aETitleError: OPTIONAL<IdmBindError_aETitleError> =
            match _components.get("aETitleError") {
                Some(c_) => Some(_decode_IdmBindError_aETitleError(c_)?),
                _ => None,
            };
        let error =
            |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                _components.get("error").unwrap(),
            )?;
        Ok(IdmBindError {
            protocolID,
            respondingAETitle,
            aETitleError,
            error,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_IdmBindError(value_: &IdmBindError) -> ASN1Result<X690Element> {
    |value_: &IdmBindError| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(ber_encode_object_identifier(&value_.protocolID)?);
        if let Some(v_) = &value_.respondingAETitle {
            components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.aETitleError {
            components_.push(_encode_IdmBindError_aETitleError(&v_)?);
        }
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
            ))
        }(&value_.error)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
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
///
#[derive(Debug, Clone)]
pub struct Request {
    pub invokeID: INTEGER,
    pub opcode: Code,
    pub argument: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl Request {
    pub fn new(
        invokeID: INTEGER,
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
impl TryFrom<X690Element> for Request {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Request(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Request {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Request> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Request,
            _eal_components_for_Request,
            _rctl2_components_for_Request,
        )?;
        let invokeID = ber_decode_integer(_components.get("invokeID").unwrap())?;
        let opcode = _decode_Code(_components.get("opcode").unwrap())?;
        let argument = x690_identity(_components.get("argument").unwrap())?;
        Ok(Request {
            invokeID,
            opcode,
            argument,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Request(value_: &Request) -> ASN1Result<X690Element> {
    |value_: &Request| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(ber_encode_integer(&value_.invokeID)?);
        components_.push(_encode_Code(&value_.opcode)?);
        components_.push(x690_identity(&value_.argument)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
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
///
#[derive(Debug, Clone)]
pub struct IdmResult {
    pub invokeID: INTEGER,
    pub opcode: Code,
    pub result: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl IdmResult {
    pub fn new(
        invokeID: INTEGER,
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
impl TryFrom<X690Element> for IdmResult {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IdmResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IdmResult {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<IdmResult> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IdmResult,
            _eal_components_for_IdmResult,
            _rctl2_components_for_IdmResult,
        )?;
        let invokeID = ber_decode_integer(_components.get("invokeID").unwrap())?;
        let opcode = _decode_Code(_components.get("opcode").unwrap())?;
        let result = x690_identity(_components.get("result").unwrap())?;
        Ok(IdmResult {
            invokeID,
            opcode,
            result,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_IdmResult(value_: &IdmResult) -> ASN1Result<X690Element> {
    |value_: &IdmResult| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(ber_encode_integer(&value_.invokeID)?);
        components_.push(_encode_Code(&value_.opcode)?);
        components_.push(x690_identity(&value_.result)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
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
///
#[derive(Debug, Clone)]
pub struct Error {
    pub invokeID: INTEGER,
    pub errcode: X690Element,
    pub error: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl Error {
    pub fn new(
        invokeID: INTEGER,
        errcode: X690Element,
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
impl TryFrom<X690Element> for Error {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Error(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Error {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Error> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Error,
            _eal_components_for_Error,
            _rctl2_components_for_Error,
        )?;
        let invokeID = ber_decode_integer(_components.get("invokeID").unwrap())?;
        let errcode = x690_identity(_components.get("errcode").unwrap())?;
        let error = x690_identity(_components.get("error").unwrap())?;
        Ok(Error {
            invokeID,
            errcode,
            error,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Error(value_: &Error) -> ASN1Result<X690Element> {
    |value_: &Error| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(ber_encode_integer(&value_.invokeID)?);
        components_.push(x690_identity(&value_.errcode)?);
        components_.push(x690_identity(&value_.error)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
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
///
#[derive(Debug, Clone)]
pub struct IdmReject {
    pub invokeID: INTEGER,
    pub reason: IdmReject_reason,
    pub _unrecognized: Vec<X690Element>,
}
impl IdmReject {
    pub fn new(
        invokeID: INTEGER,
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
impl TryFrom<X690Element> for IdmReject {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IdmReject(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IdmReject {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<IdmReject> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IdmReject,
            _eal_components_for_IdmReject,
            _rctl2_components_for_IdmReject,
        )?;
        let invokeID = ber_decode_integer(_components.get("invokeID").unwrap())?;
        let reason = _decode_IdmReject_reason(_components.get("reason").unwrap())?;
        Ok(IdmReject {
            invokeID,
            reason,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_IdmReject(value_: &IdmReject) -> ASN1Result<X690Element> {
    |value_: &IdmReject| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_integer(&value_.invokeID)?);
        components_.push(_encode_IdmReject_reason(&value_.reason)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Unbind  ::=  NULL
/// ```
pub type Unbind = NULL; // NullType

pub fn _decode_Unbind(el: &X690Element) -> ASN1Result<Unbind> {
    Ok(())
}

pub fn _encode_Unbind(value_: &Unbind) -> ASN1Result<X690Element> {
    ber_encode_null(&value_)
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
    ber_decode_enumerated(&el)
}

pub fn _encode_Abort(value_: &Abort) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// StartTLS  ::=  NULL
/// ```
pub type StartTLS = NULL; // NullType

pub fn _decode_StartTLS(el: &X690Element) -> ASN1Result<StartTLS> {
    Ok(())
}

pub fn _encode_StartTLS(value_: &StartTLS) -> ASN1Result<X690Element> {
    ber_encode_null(&value_)
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
    ber_decode_enumerated(&el)
}

pub fn _encode_TLSResponse(value_: &TLSResponse) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
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
    ber_decode_enumerated(&el)
}

pub fn _encode_IdmBindError_aETitleError(
    value_: &IdmBindError_aETitleError,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
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
    ber_decode_enumerated(&el)
}

pub fn _encode_IdmReject_reason(value_: &IdmReject_reason) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}
