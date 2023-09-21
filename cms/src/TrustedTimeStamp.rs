#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # TrustedTimeStamp
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `TrustedTimeStamp`.
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
use crate::CryptographicMessageSyntax::*;
use crate::TransientKey::*;
use asn1::*;
use std::sync::Arc;
use x500::AuthenticationFramework::*;
use x500::CertificateExtensions::*;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeStampReq ::= SEQUENCE {
/// version         Version,
/// messageImprint  MessageImprint,
/// reqPolicy       TSAPolicyId  OPTIONAL,
/// nonce           Nonce  OPTIONAL,
/// certReq         BOOLEAN  DEFAULT FALSE,
/// extensions      [0] Extensions  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct TimeStampReq {
    pub version: Version,
    pub messageImprint: MessageImprint,
    pub reqPolicy: OPTIONAL<TSAPolicyId>,
    pub nonce: OPTIONAL<Nonce>,
    pub certReq: OPTIONAL<BOOLEAN>,
    pub extensions: OPTIONAL<Extensions>,
}
impl TimeStampReq {
    pub fn new(
        version: Version,
        messageImprint: MessageImprint,
        reqPolicy: OPTIONAL<TSAPolicyId>,
        nonce: OPTIONAL<Nonce>,
        certReq: OPTIONAL<BOOLEAN>,
        extensions: OPTIONAL<Extensions>,
    ) -> Self {
        TimeStampReq {
            version,
            messageImprint,
            reqPolicy,
            nonce,
            certReq,
            extensions,
        }
    }
    pub fn _default_value_for_certReq() -> BOOLEAN {
        false
    }
}
impl TryFrom<&X690Element> for TimeStampReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimeStampReq(el)
    }
}

pub const _rctl1_components_for_TimeStampReq: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "messageImprint",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "reqPolicy",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nonce",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "certReq",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TimeStampReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TimeStampReq: &[ComponentSpec; 0] = &[];

pub fn _decode_TimeStampReq(el: &X690Element) -> ASN1Result<TimeStampReq> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeStampReq,
        _eal_components_for_TimeStampReq,
        _rctl2_components_for_TimeStampReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut messageImprint_: OPTIONAL<MessageImprint> = None;
    let mut reqPolicy_: OPTIONAL<TSAPolicyId> = None;
    let mut nonce_: OPTIONAL<Nonce> = None;
    let mut certReq_: OPTIONAL<BOOLEAN> = None;
    let mut extensions_: OPTIONAL<Extensions> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            "messageImprint" => messageImprint_ = Some(_decode_MessageImprint(_el)?),
            "reqPolicy" => reqPolicy_ = Some(_decode_TSAPolicyId(_el)?),
            "nonce" => nonce_ = Some(_decode_Nonce(_el)?),
            "certReq" => certReq_ = Some(BER.decode_boolean(_el)?),
            "extensions" => extensions_ = Some(_decode_Extensions(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampReq")
                )
            }
        }
    }
    Ok(TimeStampReq {
        version: version_.unwrap(),
        messageImprint: messageImprint_.unwrap(),
        reqPolicy: reqPolicy_,
        nonce: nonce_,
        certReq: certReq_,
        extensions: extensions_,
    })
}

pub fn _encode_TimeStampReq(value_: &TimeStampReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_Version(&value_.version)?);
    components_.push(_encode_MessageImprint(&value_.messageImprint)?);
    if let Some(v_) = &value_.reqPolicy {
        components_.push(_encode_TSAPolicyId(&v_)?);
    }
    if let Some(v_) = &value_.nonce {
        components_.push(_encode_Nonce(&v_)?);
    }
    if let Some(v_) = &value_.certReq {
        if *v_ != TimeStampReq::_default_value_for_certReq() {
            components_.push(BER.encode_boolean(&v_)?);
        }
    }
    if let Some(v_) = &value_.extensions {
        components_.push(|v_1: &Extensions| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Extensions(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_TimeStampReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeStampReq,
        _eal_components_for_TimeStampReq,
        _rctl2_components_for_TimeStampReq,
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
            "messageImprint" => _validate_MessageImprint(_el)?,
            "reqPolicy" => _validate_TSAPolicyId(_el)?,
            "nonce" => _validate_Nonce(_el)?,
            "certReq" => BER.validate_boolean(_el)?,
            "extensions" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "extensions")
                    );
                }
                Ok(_validate_Extensions(&el)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampReq")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageImprint ::= SEQUENCE {
/// hashAlgorithm  DigestAlgorithmIdentifier,
/// hashedMessage  OCTET STRING
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct MessageImprint {
    pub hashAlgorithm: DigestAlgorithmIdentifier,
    pub hashedMessage: OCTET_STRING,
}
impl MessageImprint {
    pub fn new(hashAlgorithm: DigestAlgorithmIdentifier, hashedMessage: OCTET_STRING) -> Self {
        MessageImprint {
            hashAlgorithm,
            hashedMessage,
        }
    }
}
impl TryFrom<&X690Element> for MessageImprint {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_MessageImprint(el)
    }
}

pub const _rctl1_components_for_MessageImprint: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "hashAlgorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "hashedMessage",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_MessageImprint: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MessageImprint: &[ComponentSpec; 0] = &[];

pub fn _decode_MessageImprint(el: &X690Element) -> ASN1Result<MessageImprint> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MessageImprint"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MessageImprint,
        _eal_components_for_MessageImprint,
        _rctl2_components_for_MessageImprint,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut hashAlgorithm_: OPTIONAL<DigestAlgorithmIdentifier> = None;
    let mut hashedMessage_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hashAlgorithm" => hashAlgorithm_ = Some(_decode_DigestAlgorithmIdentifier(_el)?),
            "hashedMessage" => hashedMessage_ = Some(BER.decode_octet_string(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MessageImprint")
                )
            }
        }
    }
    Ok(MessageImprint {
        hashAlgorithm: hashAlgorithm_.unwrap(),
        hashedMessage: hashedMessage_.unwrap(),
    })
}

pub fn _encode_MessageImprint(value_: &MessageImprint) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_DigestAlgorithmIdentifier(&value_.hashAlgorithm)?);
    components_.push(BER.encode_octet_string(&value_.hashedMessage)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_MessageImprint(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MessageImprint"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MessageImprint,
        _eal_components_for_MessageImprint,
        _rctl2_components_for_MessageImprint,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hashAlgorithm" => _validate_DigestAlgorithmIdentifier(_el)?,
            "hashedMessage" => BER.validate_octet_string(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MessageImprint")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageImprints  ::=  SEQUENCE SIZE(1..MAX) OF MessageImprint
/// ```
pub type MessageImprints = Vec<MessageImprint>; // SequenceOfType

pub fn _decode_MessageImprints(el: &X690Element) -> ASN1Result<MessageImprints> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MessageImprints"))
        }
    };
    let mut items: SEQUENCE_OF<MessageImprint> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_MessageImprint(el)?);
    }
    Ok(items)
}

pub fn _encode_MessageImprints(value_: &MessageImprints) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_MessageImprint(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_MessageImprints(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_MessageImprint(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MessageImprints")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TSAPolicyId  ::=  POLICY.&id({TSAPolicies})
/// ```
pub type TSAPolicyId = OBJECT_IDENTIFIER; // ObjectClassFieldType

pub fn _decode_TSAPolicyId(el: &X690Element) -> ASN1Result<TSAPolicyId> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_TSAPolicyId(value_: &TSAPolicyId) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_TSAPolicyId(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TSAPolicies POLICY ::= {
/// --
/// ... -- Any supported TSA policy --
/// }
/// ```
///
///
pub fn TSAPolicies() -> Vec<POLICY> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// POLICY ::= OIDS
/// ```
///
///
pub type POLICY = OIDS;

/// ### ASN.1 Definition:
///
/// ```asn1
/// Nonce  ::=  INTEGER
/// ```
pub type Nonce = INTEGER;

pub fn _decode_Nonce(el: &X690Element) -> ASN1Result<Nonce> {
    BER.decode_integer(&el)
}

pub fn _encode_Nonce(value_: &Nonce) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_Nonce(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeStampResp ::= SEQUENCE {
/// status          PKIStatusInfo,
/// timeStampToken  TimeStampToken  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct TimeStampResp {
    pub status: PKIStatusInfo,
    pub timeStampToken: OPTIONAL<TimeStampToken>,
}
impl TimeStampResp {
    pub fn new(status: PKIStatusInfo, timeStampToken: OPTIONAL<TimeStampToken>) -> Self {
        TimeStampResp {
            status,
            timeStampToken,
        }
    }
}
impl TryFrom<&X690Element> for TimeStampResp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimeStampResp(el)
    }
}

pub const _rctl1_components_for_TimeStampResp: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "status",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeStampToken",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TimeStampResp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TimeStampResp: &[ComponentSpec; 0] = &[];

pub fn _decode_TimeStampResp(el: &X690Element) -> ASN1Result<TimeStampResp> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampResp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeStampResp,
        _eal_components_for_TimeStampResp,
        _rctl2_components_for_TimeStampResp,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut status_: OPTIONAL<PKIStatusInfo> = None;
    let mut timeStampToken_: OPTIONAL<TimeStampToken> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => status_ = Some(_decode_PKIStatusInfo(_el)?),
            "timeStampToken" => timeStampToken_ = Some(_decode_TimeStampToken(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampResp")
                )
            }
        }
    }
    Ok(TimeStampResp {
        status: status_.unwrap(),
        timeStampToken: timeStampToken_,
    })
}

pub fn _encode_TimeStampResp(value_: &TimeStampResp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_PKIStatusInfo(&value_.status)?);
    if let Some(v_) = &value_.timeStampToken {
        components_.push(_encode_TimeStampToken(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_TimeStampResp(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampResp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeStampResp,
        _eal_components_for_TimeStampResp,
        _rctl2_components_for_TimeStampResp,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => _validate_PKIStatusInfo(_el)?,
            "timeStampToken" => _validate_TimeStampToken(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampResp")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PKIStatusInfo ::= SEQUENCE {
/// status        PKIStatus,
/// statusString  PKIFreeText  OPTIONAL,
/// failInfo      PKIFailureInfo  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct PKIStatusInfo {
    pub status: PKIStatus,
    pub statusString: OPTIONAL<PKIFreeText>,
    pub failInfo: OPTIONAL<PKIFailureInfo>,
}
impl PKIStatusInfo {
    pub fn new(
        status: PKIStatus,
        statusString: OPTIONAL<PKIFreeText>,
        failInfo: OPTIONAL<PKIFailureInfo>,
    ) -> Self {
        PKIStatusInfo {
            status,
            statusString,
            failInfo,
        }
    }
}
impl TryFrom<&X690Element> for PKIStatusInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PKIStatusInfo(el)
    }
}

pub const _rctl1_components_for_PKIStatusInfo: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "status",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "statusString",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "failInfo",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PKIStatusInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PKIStatusInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_PKIStatusInfo(el: &X690Element) -> ASN1Result<PKIStatusInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PKIStatusInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PKIStatusInfo,
        _eal_components_for_PKIStatusInfo,
        _rctl2_components_for_PKIStatusInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut status_: OPTIONAL<PKIStatus> = None;
    let mut statusString_: OPTIONAL<PKIFreeText> = None;
    let mut failInfo_: OPTIONAL<PKIFailureInfo> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => status_ = Some(_decode_PKIStatus(_el)?),
            "statusString" => statusString_ = Some(_decode_PKIFreeText(_el)?),
            "failInfo" => failInfo_ = Some(_decode_PKIFailureInfo(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PKIStatusInfo")
                )
            }
        }
    }
    Ok(PKIStatusInfo {
        status: status_.unwrap(),
        statusString: statusString_,
        failInfo: failInfo_,
    })
}

pub fn _encode_PKIStatusInfo(value_: &PKIStatusInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_PKIStatus(&value_.status)?);
    if let Some(v_) = &value_.statusString {
        components_.push(_encode_PKIFreeText(&v_)?);
    }
    if let Some(v_) = &value_.failInfo {
        components_.push(_encode_PKIFailureInfo(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_PKIStatusInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PKIStatusInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PKIStatusInfo,
        _eal_components_for_PKIStatusInfo,
        _rctl2_components_for_PKIStatusInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "status" => _validate_PKIStatus(_el)?,
            "statusString" => _validate_PKIFreeText(_el)?,
            "failInfo" => _validate_PKIFailureInfo(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PKIStatusInfo")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PKIStatus  ::=  INTEGER {
/// granted                (0), -- request is completely granted
/// grantedWithMods        (1), -- modifications were needed, requester is
///                             -- responsible for asserting the differences
/// rejection              (2), -- request not fulfilled, the failure code
///                             -- provides additional information
/// waiting                (3), -- request not yet processed, requester
///                             -- receives a receipt that the
///                             -- request has been received
/// revocationWarning      (4), -- a revocation is imminent
/// revocationNotification (5)  -- a revocation has occurred
/// }
/// ```
pub type PKIStatus = i8;

pub const PKIStatus_granted: PKIStatus = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const PKIStatus_grantedWithMods: PKIStatus = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const PKIStatus_rejection: PKIStatus = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const PKIStatus_waiting: PKIStatus = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const PKIStatus_revocationWarning: PKIStatus = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const PKIStatus_revocationNotification: PKIStatus = 5; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_PKIStatus(el: &X690Element) -> ASN1Result<PKIStatus> {
    BER.decode_i8(el)
}

pub fn _encode_PKIStatus(value_: &PKIStatus) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_PKIStatus(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PKIFreeText  ::=  SEQUENCE SIZE(1..MAX) OF UTF8String
/// ```
pub type PKIFreeText = Vec<UTF8String>; // SequenceOfType

pub fn _decode_PKIFreeText(el: &X690Element) -> ASN1Result<PKIFreeText> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PKIFreeText")),
    };
    let mut items: SEQUENCE_OF<UTF8String> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(BER.decode_utf8_string(el)?);
    }
    Ok(items)
}

pub fn _encode_PKIFreeText(value_: &PKIFreeText) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(BER.encode_utf8_string(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_PKIFreeText(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                BER.validate_utf8_string(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PKIFreeText")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PKIFailureInfo  ::=  BIT STRING {
/// badAlg                 (0), -- unrecognized or unsupported algorithm
/// badRequest             (2), -- transaction not permitted or supported
/// badDataFormat          (5), -- data submitted has the wrong format
/// timeNotAvailable      (14), -- TSAs service is not available
/// unacceptedPolicy      (15), -- requested TSA policy is not supported
/// unacceptedExtension   (16), -- requested TSA extension is not supported
/// addInfoNotAvailable   (17), -- requested additional info not available
/// systemNotAvailable    (24), -- system is not available
/// systemFailure         (25), -- system failure
/// verificationFailure   (27)  -- verification of time stamp has failed
/// }
/// ```
pub type PKIFailureInfo = BIT_STRING;

pub const PKIFailureInfo_badAlg: BIT = 0; /* LONG_NAMED_BIT */

pub const PKIFailureInfo_badRequest: BIT = 2; /* LONG_NAMED_BIT */

pub const PKIFailureInfo_badDataFormat: BIT = 5; /* LONG_NAMED_BIT */

pub const PKIFailureInfo_timeNotAvailable: BIT = 14; /* LONG_NAMED_BIT */

pub const PKIFailureInfo_unacceptedPolicy: BIT = 15; /* LONG_NAMED_BIT */

pub const PKIFailureInfo_unacceptedExtension: BIT = 16; /* LONG_NAMED_BIT */

pub const PKIFailureInfo_addInfoNotAvailable: BIT = 17; /* LONG_NAMED_BIT */

pub const PKIFailureInfo_systemNotAvailable: BIT = 24; /* LONG_NAMED_BIT */

pub const PKIFailureInfo_systemFailure: BIT = 25; /* LONG_NAMED_BIT */

pub const PKIFailureInfo_verificationFailure: BIT = 27; /* LONG_NAMED_BIT */

pub fn _decode_PKIFailureInfo(el: &X690Element) -> ASN1Result<PKIFailureInfo> {
    BER.decode_bit_string(&el)
}

pub fn _encode_PKIFailureInfo(value_: &PKIFailureInfo) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_PKIFailureInfo(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeStampToken ::= SEQUENCE {
/// contentType  CONTENTS.&id({Contents}),
/// content      [0] EXPLICIT CONTENTS.&Type({Contents}{@contentType})
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct TimeStampToken {
    pub contentType: OBJECT_IDENTIFIER,
    pub content: X690Element,
}
impl TimeStampToken {
    pub fn new(contentType: OBJECT_IDENTIFIER, content: X690Element) -> Self {
        TimeStampToken {
            contentType,
            content,
        }
    }
}
impl TryFrom<&X690Element> for TimeStampToken {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimeStampToken(el)
    }
}

pub const _rctl1_components_for_TimeStampToken: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "contentType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "content",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TimeStampToken: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TimeStampToken: &[ComponentSpec; 0] = &[];

pub fn _decode_TimeStampToken(el: &X690Element) -> ASN1Result<TimeStampToken> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampToken"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeStampToken,
        _eal_components_for_TimeStampToken,
        _rctl2_components_for_TimeStampToken,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut contentType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut content_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contentType" => contentType_ = Some(BER.decode_object_identifier(_el)?),
            "content" => {
                content_ = Some(|el: &X690Element| -> ASN1Result<X690Element> {
                    Ok(x690_identity(&el.inner()?)?)
                }(_el)?)
            }
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampToken")
                )
            }
        }
    }
    Ok(TimeStampToken {
        contentType: contentType_.unwrap(),
        content: content_.unwrap(),
    })
}

pub fn _encode_TimeStampToken(value_: &TimeStampToken) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.contentType)?);
    components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&x690_identity(&v_1)?),
        ))
    }(&value_.content)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_TimeStampToken(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampToken"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeStampToken,
        _eal_components_for_TimeStampToken,
        _rctl2_components_for_TimeStampToken,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contentType" => BER.validate_object_identifier(_el)?,
            "content" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "content")
                    );
                }
                Ok(BER.validate_any(&el.inner()?)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeStampToken")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Contents CONTENTS ::= {
/// { SignedData            IDENTIFIED BY id-signedData } |
/// { AuthenticatedData     IDENTIFIED BY id-ct-authData } |
/// { DigestedData          IDENTIFIED BY id-digestedData } |
/// { TransientKeySignedTST IDENTIFIED BY transientKeySignedTST },
/// --
/// ...  -- Expect additional time-stamp encapsulations --
/// }
/// ```
///
///
pub fn Contents() -> Vec<CONTENTS> {
    [
        Contents_Union0_Intersection0_Element(),
        Contents_Union1_Intersection0_Element(),
        Contents_Union2_Intersection0_Element(),
        Contents_Union3_Intersection0_Element(),
    ]
    .into()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TSTInfo ::= SEQUENCE {
/// version         Version,
/// policy          TSAPolicyId,
/// messageImprint  MessageImprint,
/// serialNumber    SerialNumber,
/// genTime         GeneralizedTime,
/// accuracy        Accuracy  OPTIONAL,
/// ordering        BOOLEAN DEFAULT FALSE,
/// nonce           Nonce  OPTIONAL,
/// tsa             [0] EXPLICIT GeneralName  OPTIONAL,
/// extensions      [1] Extensions  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct TSTInfo {
    pub version: Version,
    pub policy: TSAPolicyId,
    pub messageImprint: MessageImprint,
    pub serialNumber: SerialNumber,
    pub genTime: GeneralizedTime,
    pub accuracy: OPTIONAL<Accuracy>,
    pub ordering: OPTIONAL<BOOLEAN>,
    pub nonce: OPTIONAL<Nonce>,
    pub tsa: OPTIONAL<GeneralName>,
    pub extensions: OPTIONAL<Extensions>,
}
impl TSTInfo {
    pub fn new(
        version: Version,
        policy: TSAPolicyId,
        messageImprint: MessageImprint,
        serialNumber: SerialNumber,
        genTime: GeneralizedTime,
        accuracy: OPTIONAL<Accuracy>,
        ordering: OPTIONAL<BOOLEAN>,
        nonce: OPTIONAL<Nonce>,
        tsa: OPTIONAL<GeneralName>,
        extensions: OPTIONAL<Extensions>,
    ) -> Self {
        TSTInfo {
            version,
            policy,
            messageImprint,
            serialNumber,
            genTime,
            accuracy,
            ordering,
            nonce,
            tsa,
            extensions,
        }
    }
    pub fn _default_value_for_ordering() -> BOOLEAN {
        false
    }
}
impl TryFrom<&X690Element> for TSTInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TSTInfo(el)
    }
}

pub const _rctl1_components_for_TSTInfo: &[ComponentSpec; 10] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "policy",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "messageImprint",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "serialNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "genTime",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "accuracy",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "ordering",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nonce",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "tsa",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TSTInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TSTInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_TSTInfo(el: &X690Element) -> ASN1Result<TSTInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TSTInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TSTInfo,
        _eal_components_for_TSTInfo,
        _rctl2_components_for_TSTInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut policy_: OPTIONAL<TSAPolicyId> = None;
    let mut messageImprint_: OPTIONAL<MessageImprint> = None;
    let mut serialNumber_: OPTIONAL<SerialNumber> = None;
    let mut genTime_: OPTIONAL<GeneralizedTime> = None;
    let mut accuracy_: OPTIONAL<Accuracy> = None;
    let mut ordering_: OPTIONAL<BOOLEAN> = None;
    let mut nonce_: OPTIONAL<Nonce> = None;
    let mut tsa_: OPTIONAL<GeneralName> = None;
    let mut extensions_: OPTIONAL<Extensions> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            "policy" => policy_ = Some(_decode_TSAPolicyId(_el)?),
            "messageImprint" => messageImprint_ = Some(_decode_MessageImprint(_el)?),
            "serialNumber" => serialNumber_ = Some(_decode_SerialNumber(_el)?),
            "genTime" => genTime_ = Some(BER.decode_generalized_time(_el)?),
            "accuracy" => accuracy_ = Some(_decode_Accuracy(_el)?),
            "ordering" => ordering_ = Some(BER.decode_boolean(_el)?),
            "nonce" => nonce_ = Some(_decode_Nonce(_el)?),
            "tsa" => {
                tsa_ = Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                    Ok(_decode_GeneralName(&el.inner()?)?)
                }(_el)?)
            }
            "extensions" => extensions_ = Some(_decode_Extensions(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TSTInfo")),
        }
    }
    Ok(TSTInfo {
        version: version_.unwrap(),
        policy: policy_.unwrap(),
        messageImprint: messageImprint_.unwrap(),
        serialNumber: serialNumber_.unwrap(),
        genTime: genTime_.unwrap(),
        accuracy: accuracy_,
        ordering: ordering_,
        nonce: nonce_,
        tsa: tsa_,
        extensions: extensions_,
    })
}

pub fn _encode_TSTInfo(value_: &TSTInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    components_.push(_encode_Version(&value_.version)?);
    components_.push(_encode_TSAPolicyId(&value_.policy)?);
    components_.push(_encode_MessageImprint(&value_.messageImprint)?);
    components_.push(_encode_SerialNumber(&value_.serialNumber)?);
    components_.push(BER.encode_generalized_time(&value_.genTime)?);
    if let Some(v_) = &value_.accuracy {
        components_.push(_encode_Accuracy(&v_)?);
    }
    if let Some(v_) = &value_.ordering {
        if *v_ != TSTInfo::_default_value_for_ordering() {
            components_.push(BER.encode_boolean(&v_)?);
        }
    }
    if let Some(v_) = &value_.nonce {
        components_.push(_encode_Nonce(&v_)?);
    }
    if let Some(v_) = &value_.tsa {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_GeneralName(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.extensions {
        components_.push(|v_1: &Extensions| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Extensions(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_TSTInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TSTInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TSTInfo,
        _eal_components_for_TSTInfo,
        _rctl2_components_for_TSTInfo,
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
            "policy" => _validate_TSAPolicyId(_el)?,
            "messageImprint" => _validate_MessageImprint(_el)?,
            "serialNumber" => _validate_SerialNumber(_el)?,
            "genTime" => BER.validate_generalized_time(_el)?,
            "accuracy" => _validate_Accuracy(_el)?,
            "ordering" => BER.validate_boolean(_el)?,
            "nonce" => _validate_Nonce(_el)?,
            "tsa" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "tsa"));
                }
                Ok(_validate_GeneralName(&el.inner()?)?)
            }(_el)?,
            "extensions" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "extensions")
                    );
                }
                Ok(_validate_Extensions(&el)?)
            }(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TSTInfo")),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Version  ::=  INTEGER { v1(1) }
/// ```
pub type Version = i8;

pub const Version_v1: Version = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Version(el: &X690Element) -> ASN1Result<Version> {
    BER.decode_i8(el)
}

pub fn _encode_Version(value_: &Version) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Version(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SerialNumber  ::=  INTEGER
/// ```
pub type SerialNumber = INTEGER;

pub fn _decode_SerialNumber(el: &X690Element) -> ASN1Result<SerialNumber> {
    BER.decode_integer(&el)
}

pub fn _encode_SerialNumber(value_: &SerialNumber) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_SerialNumber(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Accuracy ::= SEQUENCE {
/// seconds  INTEGER OPTIONAL,
/// millis   [0] INTEGER(1..999)  OPTIONAL,
/// micros   [1] INTEGER(1..999)  OPTIONAL
/// } (ALL EXCEPT({ -- No components present -- }))
/// ```
///
#[derive(Debug, Clone)]
pub struct Accuracy {
    pub seconds: OPTIONAL<INTEGER>,
    pub millis: OPTIONAL<INTEGER>,
    pub micros: OPTIONAL<INTEGER>,
}
impl Accuracy {
    pub fn new(
        seconds: OPTIONAL<INTEGER>,
        millis: OPTIONAL<INTEGER>,
        micros: OPTIONAL<INTEGER>,
    ) -> Self {
        Accuracy {
            seconds,
            millis,
            micros,
        }
    }
}
impl Default for Accuracy {
    fn default() -> Self {
        Accuracy {
            seconds: None,
            millis: None,
            micros: None,
        }
    }
}
impl TryFrom<&X690Element> for Accuracy {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Accuracy(el)
    }
}

pub const _rctl1_components_for_Accuracy: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "seconds",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "millis",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "micros",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Accuracy: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Accuracy: &[ComponentSpec; 0] = &[];

pub fn _decode_Accuracy(el: &X690Element) -> ASN1Result<Accuracy> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Accuracy")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Accuracy,
        _eal_components_for_Accuracy,
        _rctl2_components_for_Accuracy,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut seconds_: OPTIONAL<INTEGER> = None;
    let mut millis_: OPTIONAL<INTEGER> = None;
    let mut micros_: OPTIONAL<INTEGER> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "seconds" => seconds_ = Some(BER.decode_integer(_el)?),
            "millis" => millis_ = Some(BER.decode_integer(_el)?),
            "micros" => micros_ = Some(BER.decode_integer(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Accuracy")),
        }
    }
    Ok(Accuracy {
        seconds: seconds_,
        millis: millis_,
        micros: micros_,
    })
}

pub fn _encode_Accuracy(value_: &Accuracy) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    if let Some(v_) = &value_.seconds {
        components_.push(BER.encode_integer(&v_)?);
    }
    if let Some(v_) = &value_.millis {
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_integer(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.micros {
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_integer(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Accuracy(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Accuracy")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Accuracy,
        _eal_components_for_Accuracy,
        _rctl2_components_for_Accuracy,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "seconds" => BER.validate_integer(_el)?,
            "millis" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "millis"));
                }
                Ok(BER.validate_integer(&el)?)
            }(_el)?,
            "micros" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "micros"));
                }
                Ok(BER.validate_integer(&el)?)
            }(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Accuracy")),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ETSTInfo  ::=  OCTET STRING (CONTAINING TSTInfo)
/// ```
pub type ETSTInfo = OCTET_STRING; // OctetStringType

pub fn _decode_ETSTInfo(el: &X690Element) -> ASN1Result<ETSTInfo> {
    BER.decode_octet_string(&el)
}

pub fn _encode_ETSTInfo(value_: &ETSTInfo) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_ETSTInfo(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ct-TSTInfo OID ::= { iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1)
/// pkcs-9(9) smime(16) ct(1) tstInfo(4) }
/// ```
///
///
pub fn id_ct_TSTInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* ct */ 1,
        /* tstInfo */ 4,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EContents CONTENTS ::= {
/// { ETSTInfo IDENTIFIED BY id-ct-TSTInfo },
/// --
/// ... -- Expect additional content types --
/// }
/// ```
///
///
pub fn EContents() -> Vec<CONTENTS> {
    [EContents_Union0_Intersection0_Element()].into()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EncapsulatedContentInfo ::= SEQUENCE {
/// eContentType  CONTENTS.&id({EContents}),
/// eContent      [0] EXPLICIT CONTENTS.&Type({EContents}{@eContentType})
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct EncapsulatedContentInfo {
    pub eContentType: OBJECT_IDENTIFIER,
    pub eContent: X690Element,
}
impl EncapsulatedContentInfo {
    pub fn new(eContentType: OBJECT_IDENTIFIER, eContent: X690Element) -> Self {
        EncapsulatedContentInfo {
            eContentType,
            eContent,
        }
    }
}
impl TryFrom<&X690Element> for EncapsulatedContentInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_EncapsulatedContentInfo(el)
    }
}

pub const _rctl1_components_for_EncapsulatedContentInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "eContentType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "eContent",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EncapsulatedContentInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EncapsulatedContentInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_EncapsulatedContentInfo(el: &X690Element) -> ASN1Result<EncapsulatedContentInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "EncapsulatedContentInfo",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncapsulatedContentInfo,
        _eal_components_for_EncapsulatedContentInfo,
        _rctl2_components_for_EncapsulatedContentInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut eContentType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut eContent_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "eContentType" => eContentType_ = Some(BER.decode_object_identifier(_el)?),
            "eContent" => {
                eContent_ = Some(|el: &X690Element| -> ASN1Result<X690Element> {
                    Ok(x690_identity(&el.inner()?)?)
                }(_el)?)
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "EncapsulatedContentInfo",
                ))
            }
        }
    }
    Ok(EncapsulatedContentInfo {
        eContentType: eContentType_.unwrap(),
        eContent: eContent_.unwrap(),
    })
}

pub fn _encode_EncapsulatedContentInfo(
    value_: &EncapsulatedContentInfo,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.eContentType)?);
    components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&x690_identity(&v_1)?),
        ))
    }(&value_.eContent)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_EncapsulatedContentInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "EncapsulatedContentInfo",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EncapsulatedContentInfo,
        _eal_components_for_EncapsulatedContentInfo,
        _rctl2_components_for_EncapsulatedContentInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "eContentType" => BER.validate_object_identifier(_el)?,
            "eContent" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "eContent")
                    );
                }
                Ok(BER.validate_any(&el.inner()?)?)
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "EncapsulatedContentInfo",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// VerifyReq ::= SEQUENCE {
/// version    Version,
/// tst        TimeStampToken,
/// requestID  RequestID  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct VerifyReq {
    pub version: Version,
    pub tst: TimeStampToken,
    pub requestID: OPTIONAL<RequestID>,
}
impl VerifyReq {
    pub fn new(version: Version, tst: TimeStampToken, requestID: OPTIONAL<RequestID>) -> Self {
        VerifyReq {
            version,
            tst,
            requestID,
        }
    }
}
impl TryFrom<&X690Element> for VerifyReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_VerifyReq(el)
    }
}

pub const _rctl1_components_for_VerifyReq: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "tst",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestID",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_VerifyReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_VerifyReq: &[ComponentSpec; 0] = &[];

pub fn _decode_VerifyReq(el: &X690Element) -> ASN1Result<VerifyReq> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "VerifyReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_VerifyReq,
        _eal_components_for_VerifyReq,
        _rctl2_components_for_VerifyReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut tst_: OPTIONAL<TimeStampToken> = None;
    let mut requestID_: OPTIONAL<RequestID> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            "tst" => tst_ = Some(_decode_TimeStampToken(_el)?),
            "requestID" => requestID_ = Some(_decode_RequestID(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "VerifyReq"))
            }
        }
    }
    Ok(VerifyReq {
        version: version_.unwrap(),
        tst: tst_.unwrap(),
        requestID: requestID_,
    })
}

pub fn _encode_VerifyReq(value_: &VerifyReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_Version(&value_.version)?);
    components_.push(_encode_TimeStampToken(&value_.tst)?);
    if let Some(v_) = &value_.requestID {
        components_.push(_encode_RequestID(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_VerifyReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "VerifyReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_VerifyReq,
        _eal_components_for_VerifyReq,
        _rctl2_components_for_VerifyReq,
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
            "tst" => _validate_TimeStampToken(_el)?,
            "requestID" => _validate_RequestID(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "VerifyReq"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// VerifyResp ::= SEQUENCE {
/// version    Version,
/// status     PKIStatusInfo,
/// tst        TimeStampToken,
/// requestID  RequestID  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct VerifyResp {
    pub version: Version,
    pub status: PKIStatusInfo,
    pub tst: TimeStampToken,
    pub requestID: OPTIONAL<RequestID>,
}
impl VerifyResp {
    pub fn new(
        version: Version,
        status: PKIStatusInfo,
        tst: TimeStampToken,
        requestID: OPTIONAL<RequestID>,
    ) -> Self {
        VerifyResp {
            version,
            status,
            tst,
            requestID,
        }
    }
}
impl TryFrom<&X690Element> for VerifyResp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_VerifyResp(el)
    }
}

pub const _rctl1_components_for_VerifyResp: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "status",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "tst",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestID",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_VerifyResp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_VerifyResp: &[ComponentSpec; 0] = &[];

pub fn _decode_VerifyResp(el: &X690Element) -> ASN1Result<VerifyResp> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "VerifyResp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_VerifyResp,
        _eal_components_for_VerifyResp,
        _rctl2_components_for_VerifyResp,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut status_: OPTIONAL<PKIStatusInfo> = None;
    let mut tst_: OPTIONAL<TimeStampToken> = None;
    let mut requestID_: OPTIONAL<RequestID> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            "status" => status_ = Some(_decode_PKIStatusInfo(_el)?),
            "tst" => tst_ = Some(_decode_TimeStampToken(_el)?),
            "requestID" => requestID_ = Some(_decode_RequestID(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "VerifyResp"))
            }
        }
    }
    Ok(VerifyResp {
        version: version_.unwrap(),
        status: status_.unwrap(),
        tst: tst_.unwrap(),
        requestID: requestID_,
    })
}

pub fn _encode_VerifyResp(value_: &VerifyResp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    components_.push(_encode_Version(&value_.version)?);
    components_.push(_encode_PKIStatusInfo(&value_.status)?);
    components_.push(_encode_TimeStampToken(&value_.tst)?);
    if let Some(v_) = &value_.requestID {
        components_.push(_encode_RequestID(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_VerifyResp(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "VerifyResp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_VerifyResp,
        _eal_components_for_VerifyResp,
        _rctl2_components_for_VerifyResp,
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
            "status" => _validate_PKIStatusInfo(_el)?,
            "tst" => _validate_TimeStampToken(_el)?,
            "requestID" => _validate_RequestID(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "VerifyResp"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendReq ::= SEQUENCE {
/// version    Version,
/// tst        TimeStampToken,
/// requestID  [0] OCTET STRING  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct ExtendReq {
    pub version: Version,
    pub tst: TimeStampToken,
    pub requestID: OPTIONAL<OCTET_STRING>,
}
impl ExtendReq {
    pub fn new(version: Version, tst: TimeStampToken, requestID: OPTIONAL<OCTET_STRING>) -> Self {
        ExtendReq {
            version,
            tst,
            requestID,
        }
    }
}
impl TryFrom<&X690Element> for ExtendReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendReq(el)
    }
}

pub const _rctl1_components_for_ExtendReq: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "tst",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ExtendReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ExtendReq: &[ComponentSpec; 0] = &[];

pub fn _decode_ExtendReq(el: &X690Element) -> ASN1Result<ExtendReq> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtendReq,
        _eal_components_for_ExtendReq,
        _rctl2_components_for_ExtendReq,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut tst_: OPTIONAL<TimeStampToken> = None;
    let mut requestID_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            "tst" => tst_ = Some(_decode_TimeStampToken(_el)?),
            "requestID" => requestID_ = Some(BER.decode_octet_string(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendReq"))
            }
        }
    }
    Ok(ExtendReq {
        version: version_.unwrap(),
        tst: tst_.unwrap(),
        requestID: requestID_,
    })
}

pub fn _encode_ExtendReq(value_: &ExtendReq) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_Version(&value_.version)?);
    components_.push(_encode_TimeStampToken(&value_.tst)?);
    if let Some(v_) = &value_.requestID {
        components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_octet_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ExtendReq(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendReq")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtendReq,
        _eal_components_for_ExtendReq,
        _rctl2_components_for_ExtendReq,
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
            "tst" => _validate_TimeStampToken(_el)?,
            "requestID" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "requestID")
                    );
                }
                Ok(BER.validate_octet_string(&el)?)
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendReq"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendResp ::= SEQUENCE {
///    version    Version,
///    status     PKIStatusInfo,
///    tst        TimeStampToken,
///    requestID  [0] OCTET STRING  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct ExtendResp {
    pub version: Version,
    pub status: PKIStatusInfo,
    pub tst: TimeStampToken,
    pub requestID: OPTIONAL<OCTET_STRING>,
}
impl ExtendResp {
    pub fn new(
        version: Version,
        status: PKIStatusInfo,
        tst: TimeStampToken,
        requestID: OPTIONAL<OCTET_STRING>,
    ) -> Self {
        ExtendResp {
            version,
            status,
            tst,
            requestID,
        }
    }
}
impl TryFrom<&X690Element> for ExtendResp {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendResp(el)
    }
}

pub const _rctl1_components_for_ExtendResp: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "status",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "tst",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestID",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ExtendResp: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ExtendResp: &[ComponentSpec; 0] = &[];

pub fn _decode_ExtendResp(el: &X690Element) -> ASN1Result<ExtendResp> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendResp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtendResp,
        _eal_components_for_ExtendResp,
        _rctl2_components_for_ExtendResp,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut status_: OPTIONAL<PKIStatusInfo> = None;
    let mut tst_: OPTIONAL<TimeStampToken> = None;
    let mut requestID_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            "status" => status_ = Some(_decode_PKIStatusInfo(_el)?),
            "tst" => tst_ = Some(_decode_TimeStampToken(_el)?),
            "requestID" => requestID_ = Some(BER.decode_octet_string(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendResp"))
            }
        }
    }
    Ok(ExtendResp {
        version: version_.unwrap(),
        status: status_.unwrap(),
        tst: tst_.unwrap(),
        requestID: requestID_,
    })
}

pub fn _encode_ExtendResp(value_: &ExtendResp) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    components_.push(_encode_Version(&value_.version)?);
    components_.push(_encode_PKIStatusInfo(&value_.status)?);
    components_.push(_encode_TimeStampToken(&value_.tst)?);
    if let Some(v_) = &value_.requestID {
        components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_octet_string(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ExtendResp(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendResp")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ExtendResp,
        _eal_components_for_ExtendResp,
        _rctl2_components_for_ExtendResp,
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
            "status" => _validate_PKIStatusInfo(_el)?,
            "tst" => _validate_TimeStampToken(_el)?,
            "requestID" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "requestID")
                    );
                }
                Ok(BER.validate_octet_string(&el)?)
            }(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtendResp"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RequestID  ::=  OCTET STRING (SIZE(1..MAX))
/// ```
pub type RequestID = OCTET_STRING; // OctetStringType

pub fn _decode_RequestID(el: &X690Element) -> ASN1Result<RequestID> {
    BER.decode_octet_string(&el)
}

pub fn _encode_RequestID(value_: &RequestID) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_RequestID(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Extension{EXTENSION:ExtensionSet} ::= SEQUENCE {
/// extnId     EXTENSION.&id({ExtensionSet}),
/// critical   BOOLEAN DEFAULT FALSE,
/// extnValue  OCTET STRING
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Extension {
    pub extnId: X690Element, /* COULD_NOT_RESOLVE_OBJECT_CLASS_DEF */
    pub critical: OPTIONAL<BOOLEAN>,
    pub extnValue: OCTET_STRING,
}
impl Extension {
    pub fn new(
        extnId: X690Element, /* COULD_NOT_RESOLVE_OBJECT_CLASS_DEF */
        critical: OPTIONAL<BOOLEAN>,
        extnValue: OCTET_STRING,
    ) -> Self {
        Extension {
            extnId,
            critical,
            extnValue,
        }
    }
    pub fn _default_value_for_critical() -> BOOLEAN {
        false
    }
}
impl TryFrom<&X690Element> for Extension {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Extension(el)
    }
}

pub const _rctl1_components_for_Extension: &[ComponentSpec; 3] = &[
    ComponentSpec::new("extnId", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "critical",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extnValue",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Extension: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Extension: &[ComponentSpec; 0] = &[];

pub fn _decode_Extension(el: &X690Element) -> ASN1Result<Extension> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Extension")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Extension,
        _eal_components_for_Extension,
        _rctl2_components_for_Extension,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut extnId_: OPTIONAL<X690Element /* COULD_NOT_RESOLVE_OBJECT_CLASS_DEF */> = None;
    let mut critical_: OPTIONAL<BOOLEAN> = None;
    let mut extnValue_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "extnId" => {
                extnId_ = Some(x690_identity /* COULD_NOT_RESOLVE_OBJECT_CLASS_DEF */(_el)?)
            }
            "critical" => critical_ = Some(BER.decode_boolean(_el)?),
            "extnValue" => extnValue_ = Some(BER.decode_octet_string(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Extension"))
            }
        }
    }
    Ok(Extension {
        extnId: extnId_.unwrap(),
        critical: critical_,
        extnValue: extnValue_.unwrap(),
    })
}

pub fn _encode_Extension(value_: &Extension) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(x690_identity /* COULD_NOT_RESOLVE_OBJECT_CLASS_DEF */(&value_.extnId)?);
    if let Some(v_) = &value_.critical {
        if *v_ != Extension::_default_value_for_critical() {
            components_.push(BER.encode_boolean(&v_)?);
        }
    }
    components_.push(BER.encode_octet_string(&value_.extnValue)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Extension(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Extension")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Extension,
        _eal_components_for_Extension,
        _rctl2_components_for_Extension,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "extnId" => BER.validate_any /* COULD_NOT_RESOLVE_OBJECT_CLASS_DEF */(_el)?,
            "critical" => BER.validate_boolean(_el)?,
            "extnValue" => BER.validate_octet_string(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Extension"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Extensions  ::=  SEQUENCE OF Extension{{TSExtensions}}
/// ```
pub type Extensions = Vec<Extension>; // SequenceOfType

pub fn _decode_Extensions(el: &X690Element) -> ASN1Result<Extensions> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Extensions")),
    };
    let mut items: SEQUENCE_OF<Extension> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Extension(el)?);
    }
    Ok(items)
}

pub fn _encode_Extensions(value_: &Extensions) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Extension(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Extensions(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Extension(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Extensions")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TSExtensions EXTENSION ::= {
/// extHash    |
/// extMethod  |
/// extRenewal,
/// --
/// ... -- Expect additional extensions --
/// }
/// ```
///
///
pub fn TSExtensions() -> Vec<EXTENSION> {
    Vec::from([extHash(), extMethod(), extRenewal()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extHash EXTENSION ::= { SYNTAX ExtHash IDENTIFIED BY tsp-ext-hash }
/// ```
///
pub fn extHash() -> EXTENSION {
    EXTENSION {
        id: tsp_ext_hash(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod extHash {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = ExtHash; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_ExtHash(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_ExtHash(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_ExtHash(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extMethod EXTENSION ::= { SYNTAX ExtMethod IDENTIFIED BY tsp-ext-meth }
/// ```
///
pub fn extMethod() -> EXTENSION {
    EXTENSION {
        id: tsp_ext_meth(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod extMethod {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = ExtMethod; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_ExtMethod(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_ExtMethod(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_ExtMethod(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extRenewal EXTENSION ::= { SYNTAX ExtRenewal IDENTIFIED BY tsp-ext-renewal }
/// ```
///
pub fn extRenewal() -> EXTENSION {
    EXTENSION {
        id: tsp_ext_renewal(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod extRenewal {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = ExtRenewal; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_ExtRenewal(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_ExtRenewal(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_ExtRenewal(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtHash  ::=  SEQUENCE SIZE(1..MAX) OF MessageImprint
/// ```
pub type ExtHash = Vec<MessageImprint>; // SequenceOfType

pub fn _decode_ExtHash(el: &X690Element) -> ASN1Result<ExtHash> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtHash")),
    };
    let mut items: SEQUENCE_OF<MessageImprint> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_MessageImprint(el)?);
    }
    Ok(items)
}

pub fn _encode_ExtHash(value_: &ExtHash) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_MessageImprint(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_ExtHash(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_MessageImprint(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtHash")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtMethod  ::=  SEQUENCE SIZE(1..MAX) OF Method
/// ```
pub type ExtMethod = Vec<Method>; // SequenceOfType

pub fn _decode_ExtMethod(el: &X690Element) -> ASN1Result<ExtMethod> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtMethod")),
    };
    let mut items: SEQUENCE_OF<Method> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Method(el)?);
    }
    Ok(items)
}

pub fn _encode_ExtMethod(value_: &ExtMethod) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Method(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_ExtMethod(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Method(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtMethod")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Method  ::=  METHOD.&id({Methods})
/// ```
pub type Method = OBJECT_IDENTIFIER; // ObjectClassFieldType

pub fn _decode_Method(el: &X690Element) -> ASN1Result<Method> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_Method(value_: &Method) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_Method(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Methods METHOD ::= {
/// { OID tsp-itm-ds      } |
/// { OID tsp-itm-mac     } |
/// { OID tsp-req-link    } |
/// { OID tsp-req-link-ds } |
/// { OID tsp-req-tk      },
/// --
/// ... -- Any time stamping method --
/// }
/// ```
///
///
pub fn Methods() -> Vec<METHOD> {
    [
        Methods_Union0_Intersection0_Element(),
        Methods_Union1_Intersection0_Element(),
        Methods_Union2_Intersection0_Element(),
        Methods_Union3_Intersection0_Element(),
        Methods_Union4_Intersection0_Element(),
    ]
    .into()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtRenewal  ::=  TimeStampToken
/// ```
pub type ExtRenewal = TimeStampToken; // DefinedType

pub fn _decode_ExtRenewal(el: &X690Element) -> ASN1Result<ExtRenewal> {
    _decode_TimeStampToken(&el)
}

pub fn _encode_ExtRenewal(value_: &ExtRenewal) -> ASN1Result<X690Element> {
    _encode_TimeStampToken(&value_)
}

pub fn _validate_ExtRenewal(el: &X690Element) -> ASN1Result<()> {
    _validate_TimeStampToken(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-ext-renewal OID ::= {
/// iso(1) standard(0) time-stamp(18014) extensions(1) renewal(3) }
/// ```
///
///
pub fn tsp_ext_renewal() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014,
        /* extensions */ 1, /* renewal */ 3,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-ext-hash OID ::= {
/// iso(1) standard(0) time-stamp(18014) extensions(1) hash(1)}
/// ```
///
///
pub fn tsp_ext_hash() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014,
        /* extensions */ 1, /* hash */ 1,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-ext-meth OID ::= {
/// iso(1) standard(0) time-stamp(18014) extensions(1) meth(2) }
/// ```
///
///
pub fn tsp_ext_meth() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014,
        /* extensions */ 1, /* meth */ 2,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-itm-ds OID ::= {
/// iso(1) standard(0) time-stamp(18014) itm(2) ds(1)}
/// ```
///
///
pub fn tsp_itm_ds() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014, /* itm */ 2,
        /* ds */ 1,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-itm-mac OID ::= {
/// iso(1) standard(0) time-stamp(18014) itm(2) mac(2)}
/// ```
///
///
pub fn tsp_itm_mac() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014, /* itm */ 2,
        /* mac */ 2,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-req-link OID ::= {
/// iso(1) standard(0) time-stamp(18014) lt(3) link(1)}
/// ```
///
///
pub fn tsp_req_link() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014, /* lt */ 3,
        /* link */ 1,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-req-link-ds OID ::= {
/// iso(1) standard(0) time-stamp(18014) lt(3) link-ds(2)}
/// ```
///
///
pub fn tsp_req_link_ds() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014, /* lt */ 3,
        /* link-ds */ 2,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-signedData OID ::= {
/// iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs7(7) 2 }
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
/// id-ct-authData OID ::= {
/// iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1)
/// pkcs-9(9) smime(16) ct(1) authData(2) }
/// ```
///
///
pub fn id_ct_authData() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* ct */ 1,
        /* authData */ 2,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-digestedData OID ::= {
/// iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) pkcs7(7) 5 }
/// ```
///
///
pub fn id_digestedData() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs7 */ 7, 5,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-digestedData OID ::= {
/// iso(1) standard(0) time-stamp(18014) lt(3) digestedData(8) }
/// ```
///
///
pub fn tsp_digestedData() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014, /* lt */ 3,
        /* digestedData */ 8,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-signedData OID ::= {
/// iso(1) standard(0) time-stamp(18014) lt(3) signedData(9) }
/// ```
///
///
pub fn tsp_signedData() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014, /* lt */ 3,
        /* signedData */ 9,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DDVersion95  ::=  INTEGER { version2(2) } (version2, ...)
/// ```
pub type DDVersion95 = i8;

pub const DDVersion95_version2: DDVersion95 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_DDVersion95(el: &X690Element) -> ASN1Result<DDVersion95> {
    BER.decode_i8(el)
}

pub fn _encode_DDVersion95(value_: &DDVersion95) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_DDVersion95(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BindingInfo ::= SEQUENCE {
/// version      Version,
/// msgImprints  MessageImprints,
/// aggregate    [0] Chains  OPTIONAL,
/// links        Links,
/// publish      [1] Chains  OPTIONAL,
/// extensions   [2] BindingInfoExtensions  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct BindingInfo {
    pub version: Version,
    pub msgImprints: MessageImprints,
    pub aggregate: OPTIONAL<Chains>,
    pub links: Links,
    pub publish: OPTIONAL<Chains>,
    pub extensions: OPTIONAL<BindingInfoExtensions>,
}
impl BindingInfo {
    pub fn new(
        version: Version,
        msgImprints: MessageImprints,
        aggregate: OPTIONAL<Chains>,
        links: Links,
        publish: OPTIONAL<Chains>,
        extensions: OPTIONAL<BindingInfoExtensions>,
    ) -> Self {
        BindingInfo {
            version,
            msgImprints,
            aggregate,
            links,
            publish,
            extensions,
        }
    }
}
impl TryFrom<&X690Element> for BindingInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_BindingInfo(el)
    }
}

pub const _rctl1_components_for_BindingInfo: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "msgImprints",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "aggregate",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "links",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "publish",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extensions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_BindingInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_BindingInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_BindingInfo(el: &X690Element) -> ASN1Result<BindingInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindingInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_BindingInfo,
        _eal_components_for_BindingInfo,
        _rctl2_components_for_BindingInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut msgImprints_: OPTIONAL<MessageImprints> = None;
    let mut aggregate_: OPTIONAL<Chains> = None;
    let mut links_: OPTIONAL<Links> = None;
    let mut publish_: OPTIONAL<Chains> = None;
    let mut extensions_: OPTIONAL<BindingInfoExtensions> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            "msgImprints" => msgImprints_ = Some(_decode_MessageImprints(_el)?),
            "aggregate" => aggregate_ = Some(_decode_Chains(_el)?),
            "links" => links_ = Some(_decode_Links(_el)?),
            "publish" => publish_ = Some(_decode_Chains(_el)?),
            "extensions" => extensions_ = Some(_decode_BindingInfoExtensions(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindingInfo")
                )
            }
        }
    }
    Ok(BindingInfo {
        version: version_.unwrap(),
        msgImprints: msgImprints_.unwrap(),
        aggregate: aggregate_,
        links: links_.unwrap(),
        publish: publish_,
        extensions: extensions_,
    })
}

pub fn _encode_BindingInfo(value_: &BindingInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(11);
    components_.push(_encode_Version(&value_.version)?);
    components_.push(_encode_MessageImprints(&value_.msgImprints)?);
    if let Some(v_) = &value_.aggregate {
        components_.push(|v_1: &Chains| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Chains(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_Links(&value_.links)?);
    if let Some(v_) = &value_.publish {
        components_.push(|v_1: &Chains| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Chains(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.extensions {
        components_.push(|v_1: &BindingInfoExtensions| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_BindingInfoExtensions(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_BindingInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindingInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_BindingInfo,
        _eal_components_for_BindingInfo,
        _rctl2_components_for_BindingInfo,
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
            "msgImprints" => _validate_MessageImprints(_el)?,
            "aggregate" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "aggregate")
                    );
                }
                Ok(_validate_Chains(&el)?)
            }(_el)?,
            "links" => _validate_Links(_el)?,
            "publish" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "publish")
                    );
                }
                Ok(_validate_Chains(&el)?)
            }(_el)?,
            "extensions" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "extensions")
                    );
                }
                Ok(_validate_BindingInfoExtensions(&el)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindingInfo")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Chains  ::=  SEQUENCE SIZE(1..MAX) OF Chain
/// ```
pub type Chains = Vec<Chain>; // SequenceOfType

pub fn _decode_Chains(el: &X690Element) -> ASN1Result<Chains> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Chains")),
    };
    let mut items: SEQUENCE_OF<Chain> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Chain(el)?);
    }
    Ok(items)
}

pub fn _encode_Chains(value_: &Chains) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Chain(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Chains(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Chain(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Chains")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Chain ::= SEQUENCE {
/// algorithm  ChainAlgorithmIdentifier,
/// links      Links
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Chain {
    pub algorithm: ChainAlgorithmIdentifier,
    pub links: Links,
}
impl Chain {
    pub fn new(algorithm: ChainAlgorithmIdentifier, links: Links) -> Self {
        Chain { algorithm, links }
    }
}
impl TryFrom<&X690Element> for Chain {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Chain(el)
    }
}

pub const _rctl1_components_for_Chain: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "links",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Chain: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Chain: &[ComponentSpec; 0] = &[];

pub fn _decode_Chain(el: &X690Element) -> ASN1Result<Chain> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Chain")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Chain,
        _eal_components_for_Chain,
        _rctl2_components_for_Chain,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithm_: OPTIONAL<ChainAlgorithmIdentifier> = None;
    let mut links_: OPTIONAL<Links> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => algorithm_ = Some(_decode_ChainAlgorithmIdentifier(_el)?),
            "links" => links_ = Some(_decode_Links(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Chain")),
        }
    }
    Ok(Chain {
        algorithm: algorithm_.unwrap(),
        links: links_.unwrap(),
    })
}

pub fn _encode_Chain(value_: &Chain) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_ChainAlgorithmIdentifier(&value_.algorithm)?);
    components_.push(_encode_Links(&value_.links)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Chain(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Chain")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Chain,
        _eal_components_for_Chain,
        _rctl2_components_for_Chain,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => _validate_ChainAlgorithmIdentifier(_el)?,
            "links" => _validate_Links(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Chain")),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ChainAlgorithmIdentifier  ::=  AlgorithmIdentifier {{ ChainAlgorithms }}
/// ```
pub type ChainAlgorithmIdentifier = AlgorithmIdentifier; // DefinedType

pub fn _decode_ChainAlgorithmIdentifier(el: &X690Element) -> ASN1Result<ChainAlgorithmIdentifier> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_ChainAlgorithmIdentifier(
    value_: &ChainAlgorithmIdentifier,
) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_ChainAlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ChainAlgorithms ALGORITHM ::= {
/// --
/// ... -- Expect additional chain algorithms --
/// }
/// ```
///
///
pub fn ChainAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Links  ::=  SEQUENCE SIZE(1..MAX) OF Link
/// ```
pub type Links = Vec<Link>; // SequenceOfType

pub fn _decode_Links(el: &X690Element) -> ASN1Result<Links> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Links")),
    };
    let mut items: SEQUENCE_OF<Link> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Link(el)?);
    }
    Ok(items)
}

pub fn _encode_Links(value_: &Links) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Link(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Links(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Link(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Links")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Link ::= SEQUENCE {
/// algorithm   [0] LinkAlgorithmIdentifier  OPTIONAL,
/// identifier  [1] INTEGER  OPTIONAL,
/// members      Nodes
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Link {
    pub algorithm: OPTIONAL<LinkAlgorithmIdentifier>,
    pub identifier: OPTIONAL<INTEGER>,
    pub members: Nodes,
}
impl Link {
    pub fn new(
        algorithm: OPTIONAL<LinkAlgorithmIdentifier>,
        identifier: OPTIONAL<INTEGER>,
        members: Nodes,
    ) -> Self {
        Link {
            algorithm,
            identifier,
            members,
        }
    }
}
impl TryFrom<&X690Element> for Link {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Link(el)
    }
}

pub const _rctl1_components_for_Link: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "algorithm",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "members",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Link: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Link: &[ComponentSpec; 0] = &[];

pub fn _decode_Link(el: &X690Element) -> ASN1Result<Link> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Link")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Link,
        _eal_components_for_Link,
        _rctl2_components_for_Link,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithm_: OPTIONAL<LinkAlgorithmIdentifier> = None;
    let mut identifier_: OPTIONAL<INTEGER> = None;
    let mut members_: OPTIONAL<Nodes> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => algorithm_ = Some(_decode_LinkAlgorithmIdentifier(_el)?),
            "identifier" => identifier_ = Some(BER.decode_integer(_el)?),
            "members" => members_ = Some(_decode_Nodes(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Link")),
        }
    }
    Ok(Link {
        algorithm: algorithm_,
        identifier: identifier_,
        members: members_.unwrap(),
    })
}

pub fn _encode_Link(value_: &Link) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    if let Some(v_) = &value_.algorithm {
        components_.push(|v_1: &LinkAlgorithmIdentifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_LinkAlgorithmIdentifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.identifier {
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_integer(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_Nodes(&value_.members)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Link(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Link")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Link,
        _eal_components_for_Link,
        _rctl2_components_for_Link,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "algorithm")
                    );
                }
                Ok(_validate_LinkAlgorithmIdentifier(&el)?)
            }(_el)?,
            "identifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "identifier")
                    );
                }
                Ok(BER.validate_integer(&el)?)
            }(_el)?,
            "members" => _validate_Nodes(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Link")),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LinkAlgorithmIdentifier  ::=  AlgorithmIdentifier {{ LinkAlgorithms }}
/// ```
pub type LinkAlgorithmIdentifier = AlgorithmIdentifier; // DefinedType

pub fn _decode_LinkAlgorithmIdentifier(el: &X690Element) -> ASN1Result<LinkAlgorithmIdentifier> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_LinkAlgorithmIdentifier(
    value_: &LinkAlgorithmIdentifier,
) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_LinkAlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LinkAlgorithms ALGORITHM ::= {
/// --
/// ... -- Expect additional link algorithms --
/// }
/// ```
///
///
pub fn LinkAlgorithms() -> Vec<ALGORITHM> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Nodes  ::=  SEQUENCE SIZE(1..MAX) OF Node
/// ```
pub type Nodes = Vec<Node>; // SequenceOfType

pub fn _decode_Nodes(el: &X690Element) -> ASN1Result<Nodes> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Nodes")),
    };
    let mut items: SEQUENCE_OF<Node> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Node(el)?);
    }
    Ok(items)
}

pub fn _encode_Nodes(value_: &Nodes) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Node(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Nodes(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Node(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Nodes")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Node  ::=  CHOICE {
/// imprints   [0] Imprints,
/// reference  [1] INTEGER
/// }
/// ```
#[derive(Debug, Clone)]
pub enum Node {
    imprints(Imprints),
    reference(INTEGER),
}

impl TryFrom<&X690Element> for Node {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Node(el)
    }
}

pub fn _decode_Node(el: &X690Element) -> ASN1Result<Node> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(Node::imprints(_decode_Imprints(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(Node::reference(BER.decode_integer(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Node",
            ))
        }
    }
}

pub fn _encode_Node(value_: &Node) -> ASN1Result<X690Element> {
    match value_ {
        Node::imprints(v) => |v_1: &Imprints| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Imprints(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        Node::reference(v) => |v_1: &INTEGER| -> ASN1Result<X690Element> {
            let mut el_1 = BER.encode_integer(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
    }
}

pub fn _validate_Node(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "imprints"));
            }
            Ok(_validate_Imprints(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "reference"));
            }
            Ok(BER.validate_integer(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Node",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Imprints  ::=   SEQUENCE SIZE(1..MAX) OF Imprint
/// ```
pub type Imprints = Vec<Imprint>; // SequenceOfType

pub fn _decode_Imprints(el: &X690Element) -> ASN1Result<Imprints> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Imprints")),
    };
    let mut items: SEQUENCE_OF<Imprint> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Imprint(el)?);
    }
    Ok(items)
}

pub fn _encode_Imprints(value_: &Imprints) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Imprint(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Imprints(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Imprint(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Imprints")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Imprint  ::=  OCTET STRING
/// ```
pub type Imprint = OCTET_STRING; // OctetStringType

pub fn _decode_Imprint(el: &X690Element) -> ASN1Result<Imprint> {
    BER.decode_octet_string(&el)
}

pub fn _encode_Imprint(value_: &Imprint) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_Imprint(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BindingInfoExtensions  ::=  SEQUENCE OF Extension{{BIExtensions}}
/// ```
pub type BindingInfoExtensions = Vec<Extension>; // SequenceOfType

pub fn _decode_BindingInfoExtensions(el: &X690Element) -> ASN1Result<BindingInfoExtensions> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindingInfoExtensions")
            )
        }
    };
    let mut items: SEQUENCE_OF<Extension> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Extension(el)?);
    }
    Ok(items)
}

pub fn _encode_BindingInfoExtensions(value_: &BindingInfoExtensions) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Extension(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_BindingInfoExtensions(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Extension(&sub)?;
            }
            Ok(())
        }
        _ => {
            Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "BindingInfoExtensions"))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BIExtensions EXTENSION ::= {
/// extName        |
/// extTime        |
/// extPublication,
/// --
/// ... -- Expect additional extensions --
/// }
/// ```
///
///
pub fn BIExtensions() -> Vec<EXTENSION> {
    Vec::from([extName(), extTime(), extPublication()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extName EXTENSION ::= { SYNTAX ExtName IDENTIFIED BY tsp-ext-name }
/// ```
///
pub fn extName() -> EXTENSION {
    EXTENSION {
        id: tsp_ext_name(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod extName {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = ExtName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_ExtName(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_ExtName(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_ExtName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extTime EXTENSION ::= { SYNTAX ExtTime IDENTIFIED BY tsp-ext-time }
/// ```
///
pub fn extTime() -> EXTENSION {
    EXTENSION {
        id: tsp_ext_time(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod extTime {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = ExtTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_ExtTime(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_ExtTime(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_ExtTime(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extPublication EXTENSION ::= { SYNTAX ExtPublication IDENTIFIED BY tsp-ext-publication }
/// ```
///
pub fn extPublication() -> EXTENSION {
    EXTENSION {
        id: tsp_ext_publication(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod extPublication {
    /* OBJECT_TYPES */
    use super::*;
    pub type ExtnType = ExtPublication; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_ExtnType(el: &X690Element) -> ASN1Result<ExtnType> {
        _decode_ExtPublication(el)
    }
    pub fn _encode_ExtnType(value_: &ExtnType) -> ASN1Result<X690Element> {
        _encode_ExtPublication(value_)
    }
    pub fn _validate_ExtnType(el: &X690Element) -> ASN1Result<()> {
        _validate_ExtPublication(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtName  ::=  GeneralName
/// ```
pub type ExtName = GeneralName; // DefinedType

pub fn _decode_ExtName(el: &X690Element) -> ASN1Result<ExtName> {
    _decode_GeneralName(&el)
}

pub fn _encode_ExtName(value_: &ExtName) -> ASN1Result<X690Element> {
    _encode_GeneralName(&value_)
}

pub fn _validate_ExtName(el: &X690Element) -> ASN1Result<()> {
    _validate_GeneralName(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-ext-name OID ::= {
/// iso(1) standard(0) time-stamp(18014) lt(3) name(5) }
/// ```
///
///
pub fn tsp_ext_name() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014, /* lt */ 3,
        /* name */ 5,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtTime  ::=  GeneralizedTime
/// ```
pub type ExtTime = GeneralizedTime; // GeneralizedTime

pub fn _decode_ExtTime(el: &X690Element) -> ASN1Result<ExtTime> {
    BER.decode_generalized_time(&el)
}

pub fn _encode_ExtTime(value_: &ExtTime) -> ASN1Result<X690Element> {
    BER.encode_generalized_time(&value_)
}

pub fn _validate_ExtTime(el: &X690Element) -> ASN1Result<()> {
    BER.validate_generalized_time(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-ext-time OID ::= {
/// iso(1) standard(0) time-stamp(18014) lt(3) time (6) }
/// ```
///
///
pub fn tsp_ext_time() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014, /* lt */ 3,
        /* time */ 6,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtPublication  ::=  SEQUENCE SIZE (1..MAX) OF PublicationInfo
/// ```
pub type ExtPublication = Vec<PublicationInfo>; // SequenceOfType

pub fn _decode_ExtPublication(el: &X690Element) -> ASN1Result<ExtPublication> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtPublication"))
        }
    };
    let mut items: SEQUENCE_OF<PublicationInfo> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_PublicationInfo(el)?);
    }
    Ok(items)
}

pub fn _encode_ExtPublication(value_: &ExtPublication) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_PublicationInfo(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_ExtPublication(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_PublicationInfo(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ExtPublication")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tsp-ext-publication OID ::= {
/// iso(1) standard(0) time-stamp(18014) lt(3) publication (7) }
/// ```
///
///
pub fn tsp_ext_publication() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* standard */ 0, /* time-stamp */ 18014, /* lt */ 3,
        /* publication */ 7,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PublicationInfo ::= SEQUENCE {
/// pubTime   GeneralizedTime OPTIONAL,
/// pubId     [0] GeneralName OPTIONAL,
/// pubChains [1] Chains      OPTIONAL,
/// sourceId  [2] GeneralName OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct PublicationInfo {
    pub pubTime: OPTIONAL<GeneralizedTime>,
    pub pubId: OPTIONAL<GeneralName>,
    pub pubChains: OPTIONAL<Chains>,
    pub sourceId: OPTIONAL<GeneralName>,
}
impl PublicationInfo {
    pub fn new(
        pubTime: OPTIONAL<GeneralizedTime>,
        pubId: OPTIONAL<GeneralName>,
        pubChains: OPTIONAL<Chains>,
        sourceId: OPTIONAL<GeneralName>,
    ) -> Self {
        PublicationInfo {
            pubTime,
            pubId,
            pubChains,
            sourceId,
        }
    }
}
impl Default for PublicationInfo {
    fn default() -> Self {
        PublicationInfo {
            pubTime: None,
            pubId: None,
            pubChains: None,
            sourceId: None,
        }
    }
}
impl TryFrom<&X690Element> for PublicationInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PublicationInfo(el)
    }
}

pub const _rctl1_components_for_PublicationInfo: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "pubTime",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pubId",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "pubChains",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sourceId",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PublicationInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PublicationInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_PublicationInfo(el: &X690Element) -> ASN1Result<PublicationInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PublicationInfo"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PublicationInfo,
        _eal_components_for_PublicationInfo,
        _rctl2_components_for_PublicationInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut pubTime_: OPTIONAL<GeneralizedTime> = None;
    let mut pubId_: OPTIONAL<GeneralName> = None;
    let mut pubChains_: OPTIONAL<Chains> = None;
    let mut sourceId_: OPTIONAL<GeneralName> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "pubTime" => pubTime_ = Some(BER.decode_generalized_time(_el)?),
            "pubId" => pubId_ = Some(_decode_GeneralName(_el)?),
            "pubChains" => pubChains_ = Some(_decode_Chains(_el)?),
            "sourceId" => sourceId_ = Some(_decode_GeneralName(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PublicationInfo")
                )
            }
        }
    }
    Ok(PublicationInfo {
        pubTime: pubTime_,
        pubId: pubId_,
        pubChains: pubChains_,
        sourceId: sourceId_,
    })
}

pub fn _encode_PublicationInfo(value_: &PublicationInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    if let Some(v_) = &value_.pubTime {
        components_.push(BER.encode_generalized_time(&v_)?);
    }
    if let Some(v_) = &value_.pubId {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralName(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.pubChains {
        components_.push(|v_1: &Chains| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Chains(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.sourceId {
        components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralName(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_PublicationInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PublicationInfo"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PublicationInfo,
        _eal_components_for_PublicationInfo,
        _rctl2_components_for_PublicationInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "pubTime" => BER.validate_generalized_time(_el)?,
            "pubId" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "pubId"));
                }
                Ok(_validate_GeneralName(&el)?)
            }(_el)?,
            "pubChains" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "pubChains")
                    );
                }
                Ok(_validate_Chains(&el)?)
            }(_el)?,
            "sourceId" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "sourceId")
                    );
                }
                Ok(_validate_GeneralName(&el)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PublicationInfo")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-merkle-chain OID ::= {
/// iso(1) identified-organization(3) tc68(133) country(16) x9(840)
/// x9Standards(9) x9-95(95) ids(1) merkle-chain(1) }
/// ```
///
///
pub fn id_merkle_chain () -> OID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* identified-organization */ 3, /* tc68 */ 133,
        /* country */ 16, /* x9 */ 840, /* x9Standards */ 9, /* x9-95 */ 95,
        /* ids */ 1, /* merkle-chain */ 1,
    ]))
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// merkle-chain ALGORITHM ::= {
/// OID id-merkle-chain PARMS MerkleChainParms
/// }
/// ```
///
///
pub fn merkle_chain() -> ALGORITHM {
    ALGORITHM {
        id: id_merkle_chain(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod merkle_chain {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MerkleChainParms; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MerkleChainParms(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MerkleChainParms(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MerkleChainParms(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MerkleChainParms  ::=  SEQUENCE SIZE(1..MAX) OF HashFunction
/// ```
pub type MerkleChainParms = Vec<HashFunction>; // SequenceOfType

pub fn _decode_MerkleChainParms(el: &X690Element) -> ASN1Result<MerkleChainParms> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MerkleChainParms")
            )
        }
    };
    let mut items: SEQUENCE_OF<HashFunction> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_HashFunction(el)?);
    }
    Ok(items)
}

pub fn _encode_MerkleChainParms(value_: &MerkleChainParms) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_HashFunction(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_MerkleChainParms(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_HashFunction(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MerkleChainParms")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HashFunction  ::=  DigestAlgorithmIdentifier
/// ```
pub type HashFunction = DigestAlgorithmIdentifier; // DefinedType

pub fn _decode_HashFunction(el: &X690Element) -> ASN1Result<HashFunction> {
    _decode_DigestAlgorithmIdentifier(&el)
}

pub fn _encode_HashFunction(value_: &HashFunction) -> ASN1Result<X690Element> {
    _encode_DigestAlgorithmIdentifier(&value_)
}

pub fn _validate_HashFunction(el: &X690Element) -> ASN1Result<()> {
    _validate_DigestAlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeCalibrationReport ::= SEQUENCE {
/// version        Version,
/// tseInfo        EntityInfo,
/// tsaInfo        EntityInfo,
/// dutInfo        [0] EntityInfo  OPTIONAL,
/// timingMetrics  TimingMetrics
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct TimeCalibrationReport {
    pub version: Version,
    pub tseInfo: EntityInfo,
    pub tsaInfo: EntityInfo,
    pub dutInfo: OPTIONAL<EntityInfo>,
    pub timingMetrics: TimingMetrics,
}
impl TimeCalibrationReport {
    pub fn new(
        version: Version,
        tseInfo: EntityInfo,
        tsaInfo: EntityInfo,
        dutInfo: OPTIONAL<EntityInfo>,
        timingMetrics: TimingMetrics,
    ) -> Self {
        TimeCalibrationReport {
            version,
            tseInfo,
            tsaInfo,
            dutInfo,
            timingMetrics,
        }
    }
}
impl TryFrom<&X690Element> for TimeCalibrationReport {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimeCalibrationReport(el)
    }
}

pub const _rctl1_components_for_TimeCalibrationReport: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "tseInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "tsaInfo",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "dutInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timingMetrics",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TimeCalibrationReport: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TimeCalibrationReport: &[ComponentSpec; 0] = &[];

pub fn _decode_TimeCalibrationReport(el: &X690Element) -> ASN1Result<TimeCalibrationReport> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeCalibrationReport")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeCalibrationReport,
        _eal_components_for_TimeCalibrationReport,
        _rctl2_components_for_TimeCalibrationReport,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut version_: OPTIONAL<Version> = None;
    let mut tseInfo_: OPTIONAL<EntityInfo> = None;
    let mut tsaInfo_: OPTIONAL<EntityInfo> = None;
    let mut dutInfo_: OPTIONAL<EntityInfo> = None;
    let mut timingMetrics_: OPTIONAL<TimingMetrics> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_Version(_el)?),
            "tseInfo" => tseInfo_ = Some(_decode_EntityInfo(_el)?),
            "tsaInfo" => tsaInfo_ = Some(_decode_EntityInfo(_el)?),
            "dutInfo" => dutInfo_ = Some(_decode_EntityInfo(_el)?),
            "timingMetrics" => timingMetrics_ = Some(_decode_TimingMetrics(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "TimeCalibrationReport",
                ))
            }
        }
    }
    Ok(TimeCalibrationReport {
        version: version_.unwrap(),
        tseInfo: tseInfo_.unwrap(),
        tsaInfo: tsaInfo_.unwrap(),
        dutInfo: dutInfo_,
        timingMetrics: timingMetrics_.unwrap(),
    })
}

pub fn _encode_TimeCalibrationReport(value_: &TimeCalibrationReport) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(10);
    components_.push(_encode_Version(&value_.version)?);
    components_.push(_encode_EntityInfo(&value_.tseInfo)?);
    components_.push(_encode_EntityInfo(&value_.tsaInfo)?);
    if let Some(v_) = &value_.dutInfo {
        components_.push(|v_1: &EntityInfo| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_EntityInfo(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(_encode_TimingMetrics(&value_.timingMetrics)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_TimeCalibrationReport(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeCalibrationReport")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeCalibrationReport,
        _eal_components_for_TimeCalibrationReport,
        _rctl2_components_for_TimeCalibrationReport,
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
            "tseInfo" => _validate_EntityInfo(_el)?,
            "tsaInfo" => _validate_EntityInfo(_el)?,
            "dutInfo" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "dutInfo")
                    );
                }
                Ok(_validate_EntityInfo(&el)?)
            }(_el)?,
            "timingMetrics" => _validate_TimingMetrics(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "TimeCalibrationReport",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EntityInfo ::= SEQUENCE {
/// entityName    UTF8String  OPTIONAL,
/// entityID      OBJECT IDENTIFIER  OPTIONAL,
/// entityOption  OCTET STRING  OPTIONAL
/// } (ALL EXCEPT ({ -- None; at least one component shall be present -- }))
/// ```
///
#[derive(Debug, Clone)]
pub struct EntityInfo {
    pub entityName: OPTIONAL<UTF8String>,
    pub entityID: OPTIONAL<OBJECT_IDENTIFIER>,
    pub entityOption: OPTIONAL<OCTET_STRING>,
}
impl EntityInfo {
    pub fn new(
        entityName: OPTIONAL<UTF8String>,
        entityID: OPTIONAL<OBJECT_IDENTIFIER>,
        entityOption: OPTIONAL<OCTET_STRING>,
    ) -> Self {
        EntityInfo {
            entityName,
            entityID,
            entityOption,
        }
    }
}
impl Default for EntityInfo {
    fn default() -> Self {
        EntityInfo {
            entityName: None,
            entityID: None,
            entityOption: None,
        }
    }
}
impl TryFrom<&X690Element> for EntityInfo {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_EntityInfo(el)
    }
}

pub const _rctl1_components_for_EntityInfo: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "entityName",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 12)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entityID",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entityOption",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EntityInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EntityInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_EntityInfo(el: &X690Element) -> ASN1Result<EntityInfo> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EntityInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EntityInfo,
        _eal_components_for_EntityInfo,
        _rctl2_components_for_EntityInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut entityName_: OPTIONAL<UTF8String> = None;
    let mut entityID_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut entityOption_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "entityName" => entityName_ = Some(BER.decode_utf8_string(_el)?),
            "entityID" => entityID_ = Some(BER.decode_object_identifier(_el)?),
            "entityOption" => entityOption_ = Some(BER.decode_octet_string(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EntityInfo"))
            }
        }
    }
    Ok(EntityInfo {
        entityName: entityName_,
        entityID: entityID_,
        entityOption: entityOption_,
    })
}

pub fn _encode_EntityInfo(value_: &EntityInfo) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    if let Some(v_) = &value_.entityName {
        components_.push(BER.encode_utf8_string(&v_)?);
    }
    if let Some(v_) = &value_.entityID {
        components_.push(BER.encode_object_identifier(&v_)?);
    }
    if let Some(v_) = &value_.entityOption {
        components_.push(BER.encode_octet_string(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_EntityInfo(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EntityInfo")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EntityInfo,
        _eal_components_for_EntityInfo,
        _rctl2_components_for_EntityInfo,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "entityName" => BER.validate_utf8_string(_el)?,
            "entityID" => BER.validate_object_identifier(_el)?,
            "entityOption" => BER.validate_octet_string(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EntityInfo"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimingMetrics ::= SEQUENCE {
/// ntpTime      GeneralizedTime, -- Time at which certification took place
/// offset       Accuracy,        -- Current lower clock offset
/// delay        Accuracy,        -- Path propagation delay
/// leapSecond   LeapSecond  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct TimingMetrics {
    pub ntpTime: GeneralizedTime,
    pub offset: Accuracy,
    pub delay: Accuracy,
    pub leapSecond: OPTIONAL<LeapSecond>,
}
impl TimingMetrics {
    pub fn new(
        ntpTime: GeneralizedTime,
        offset: Accuracy,
        delay: Accuracy,
        leapSecond: OPTIONAL<LeapSecond>,
    ) -> Self {
        TimingMetrics {
            ntpTime,
            offset,
            delay,
            leapSecond,
        }
    }
}
impl TryFrom<&X690Element> for TimingMetrics {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimingMetrics(el)
    }
}

pub const _rctl1_components_for_TimingMetrics: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "ntpTime",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "offset",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "delay",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "leapSecond",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TimingMetrics: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TimingMetrics: &[ComponentSpec; 0] = &[];

pub fn _decode_TimingMetrics(el: &X690Element) -> ASN1Result<TimingMetrics> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimingMetrics")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimingMetrics,
        _eal_components_for_TimingMetrics,
        _rctl2_components_for_TimingMetrics,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut ntpTime_: OPTIONAL<GeneralizedTime> = None;
    let mut offset_: OPTIONAL<Accuracy> = None;
    let mut delay_: OPTIONAL<Accuracy> = None;
    let mut leapSecond_: OPTIONAL<LeapSecond> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ntpTime" => ntpTime_ = Some(BER.decode_generalized_time(_el)?),
            "offset" => offset_ = Some(_decode_Accuracy(_el)?),
            "delay" => delay_ = Some(_decode_Accuracy(_el)?),
            "leapSecond" => leapSecond_ = Some(_decode_LeapSecond(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimingMetrics")
                )
            }
        }
    }
    Ok(TimingMetrics {
        ntpTime: ntpTime_.unwrap(),
        offset: offset_.unwrap(),
        delay: delay_.unwrap(),
        leapSecond: leapSecond_,
    })
}

pub fn _encode_TimingMetrics(value_: &TimingMetrics) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    components_.push(BER.encode_generalized_time(&value_.ntpTime)?);
    components_.push(_encode_Accuracy(&value_.offset)?);
    components_.push(_encode_Accuracy(&value_.delay)?);
    if let Some(v_) = &value_.leapSecond {
        components_.push(_encode_LeapSecond(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_TimingMetrics(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimingMetrics")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimingMetrics,
        _eal_components_for_TimingMetrics,
        _rctl2_components_for_TimingMetrics,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ntpTime" => BER.validate_generalized_time(_el)?,
            "offset" => _validate_Accuracy(_el)?,
            "delay" => _validate_Accuracy(_el)?,
            "leapSecond" => _validate_LeapSecond(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimingMetrics")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LeapSecond ::= SEQUENCE {
/// leapDay  GeneralizedTime,
/// action   INTEGER(0..1)  -- 1: last minute has 61 seconds --
///                         -- 0: last minute has 59 seconds --
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct LeapSecond {
    pub leapDay: GeneralizedTime,
    pub action: INTEGER,
}
impl LeapSecond {
    pub fn new(leapDay: GeneralizedTime, action: INTEGER) -> Self {
        LeapSecond { leapDay, action }
    }
}
impl TryFrom<&X690Element> for LeapSecond {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_LeapSecond(el)
    }
}

pub const _rctl1_components_for_LeapSecond: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "leapDay",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new(
        "action",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_LeapSecond: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_LeapSecond: &[ComponentSpec; 0] = &[];

pub fn _decode_LeapSecond(el: &X690Element) -> ASN1Result<LeapSecond> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "LeapSecond")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_LeapSecond,
        _eal_components_for_LeapSecond,
        _rctl2_components_for_LeapSecond,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut leapDay_: OPTIONAL<GeneralizedTime> = None;
    let mut action_: OPTIONAL<INTEGER> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "leapDay" => leapDay_ = Some(BER.decode_generalized_time(_el)?),
            "action" => action_ = Some(BER.decode_integer(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "LeapSecond"))
            }
        }
    }
    Ok(LeapSecond {
        leapDay: leapDay_.unwrap(),
        action: action_.unwrap(),
    })
}

pub fn _encode_LeapSecond(value_: &LeapSecond) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_generalized_time(&value_.leapDay)?);
    components_.push(BER.encode_integer(&value_.action)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_LeapSecond(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "LeapSecond")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_LeapSecond,
        _eal_components_for_LeapSecond,
        _rctl2_components_for_LeapSecond,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "leapDay" => BER.validate_generalized_time(_el)?,
            "action" => BER.validate_integer(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "LeapSecond"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OID  ::=  OBJECT IDENTIFIER
/// ```
pub type OID = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_OID(el: &X690Element) -> ASN1Result<OID> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_OID(value_: &OID) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_OID(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OIDS ::= CLASS {
/// &id OBJECT IDENTIFIER  UNIQUE
/// }
/// WITH SYNTAX { OID &id }
/// ```
///
#[derive(Debug)]
pub struct OIDS {
    pub id: OBJECT_IDENTIFIER,
}
impl OIDS {}

pub type CONTENTS = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// AlgorithmIdentifier { ALGORITHM:IOSet } ::= SEQUENCE {
/// algorithm   ALGORITHM.&id({IOSet}),
/// parameters  ALGORITHM.&Type({IOSet}{@algorithm})  OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct AlgorithmIdentifier {
    pub algorithm: OBJECT_IDENTIFIER,
    pub parameters: OPTIONAL<X690Element>,
}
impl AlgorithmIdentifier {
    pub fn new(algorithm: OBJECT_IDENTIFIER, parameters: OPTIONAL<X690Element>) -> Self {
        AlgorithmIdentifier {
            algorithm,
            parameters,
        }
    }
}
impl TryFrom<&X690Element> for AlgorithmIdentifier {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AlgorithmIdentifier(el)
    }
}

pub const _rctl1_components_for_AlgorithmIdentifier: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("parameters", true, TagSelector::any, None, None),
];

pub const _rctl2_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];

pub fn _decode_AlgorithmIdentifier(el: &X690Element) -> ASN1Result<AlgorithmIdentifier> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AlgorithmIdentifier")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AlgorithmIdentifier,
        _eal_components_for_AlgorithmIdentifier,
        _rctl2_components_for_AlgorithmIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithm_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut parameters_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => algorithm_ = Some(BER.decode_object_identifier(_el)?),
            "parameters" => parameters_ = Some(x690_identity(_el)?),
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AlgorithmIdentifier"))
            }
        }
    }
    Ok(AlgorithmIdentifier {
        algorithm: algorithm_.unwrap(),
        parameters: parameters_,
    })
}

pub fn _encode_AlgorithmIdentifier(value_: &AlgorithmIdentifier) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.algorithm)?);
    if let Some(v_) = &value_.parameters {
        components_.push(x690_identity(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_AlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AlgorithmIdentifier")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AlgorithmIdentifier,
        _eal_components_for_AlgorithmIdentifier,
        _rctl2_components_for_AlgorithmIdentifier,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => BER.validate_object_identifier(_el)?,
            "parameters" => BER.validate_any(_el)?,
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AlgorithmIdentifier"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ALGORITHM ::= CLASS {
/// &id    OBJECT IDENTIFIER  UNIQUE,
/// &Type  OPTIONAL
/// }
/// WITH SYNTAX { OID &id [PARMS &Type] }
/// ```
///
#[derive(Debug)]
pub struct ALGORITHM {
    pub id: OBJECT_IDENTIFIER,
}
impl ALGORITHM {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// METHOD ::= CLASS {
/// &id OBJECT IDENTIFIER  UNIQUE
/// }
/// WITH SYNTAX { OID &id }
/// ```
///
#[derive(Debug)]
pub struct METHOD {
    pub id: OBJECT_IDENTIFIER,
}
impl METHOD {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Contents-Union0-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn Contents_Union0_Intersection0_Element() -> CONTENTS {
    TYPE_IDENTIFIER {
        id: id_signedData(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod Contents_Union0_Intersection0_Element {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SignedData; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SignedData(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SignedData(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SignedData(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Contents-Union1-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn Contents_Union1_Intersection0_Element() -> CONTENTS {
    TYPE_IDENTIFIER {
        id: id_ct_authData(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod Contents_Union1_Intersection0_Element {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AuthenticatedData; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AuthenticatedData(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AuthenticatedData(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AuthenticatedData(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Contents-Union2-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn Contents_Union2_Intersection0_Element() -> CONTENTS {
    TYPE_IDENTIFIER {
        id: id_digestedData(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod Contents_Union2_Intersection0_Element {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DigestedData; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DigestedData(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DigestedData(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DigestedData(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Contents-Union3-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn Contents_Union3_Intersection0_Element() -> CONTENTS {
    TYPE_IDENTIFIER {
        id: transientKeySignedTST(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod Contents_Union3_Intersection0_Element {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = TransientKeySignedTST; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_TransientKeySignedTST(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_TransientKeySignedTST(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_TransientKeySignedTST(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EContents-Union0-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn EContents_Union0_Intersection0_Element() -> CONTENTS {
    TYPE_IDENTIFIER {
        id: id_ct_TSTInfo(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod EContents_Union0_Intersection0_Element {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ETSTInfo; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ETSTInfo(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ETSTInfo(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ETSTInfo(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Methods-Union0-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn Methods_Union0_Intersection0_Element() -> METHOD {
    METHOD {
        id: tsp_itm_ds(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod Methods_Union0_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Methods-Union1-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn Methods_Union1_Intersection0_Element() -> METHOD {
    METHOD {
        id: tsp_itm_mac(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod Methods_Union1_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Methods-Union2-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn Methods_Union2_Intersection0_Element() -> METHOD {
    METHOD {
        id: tsp_req_link(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod Methods_Union2_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Methods-Union3-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn Methods_Union3_Intersection0_Element() -> METHOD {
    METHOD {
        id: tsp_req_link_ds(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod Methods_Union3_Intersection0_Element {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Methods-Union4-Intersection0-Element ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn Methods_Union4_Intersection0_Element() -> METHOD {
    METHOD {
        id: tsp_req_tk(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod Methods_Union4_Intersection0_Element {
    /* OBJECT_TYPES */
}
