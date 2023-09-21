#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PKIXTSP
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PKIXTSP`.
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
use crate::PKIXCMP::*;
use asn1::*;
use cms::CryptographicMessageSyntax::{ContentInfo, _decode_ContentInfo, _encode_ContentInfo};
use std::borrow::Borrow;
use std::sync::Arc;
use x500::AuthenticationFramework::{
    AlgorithmIdentifier, Extensions, _decode_AlgorithmIdentifier, _decode_Extensions,
    _encode_AlgorithmIdentifier, _encode_Extensions,
};
use x500::CertificateExtensions::{GeneralName, _decode_GeneralName, _encode_GeneralName};
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ct-TSTInfo  OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs-9(9) smime(16) ct(1) 4}
/// ```
///
///
pub fn id_ct_TSTInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* ct */ 1, 4,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeStampReq ::= SEQUENCE  {
///     version                  INTEGER  { v1(1) },
///     messageImprint           MessageImprint,
///     --a hash algorithm OID and the hash value of the data to be
///     --time-stamped
///     reqPolicy                TSAPolicyId                OPTIONAL,
///     nonce                    INTEGER                    OPTIONAL,
///     certReq                  BOOLEAN                    DEFAULT FALSE,
///     extensions               [0] IMPLICIT Extensions    OPTIONAL  }
/// ```
///
#[derive(Debug, Clone)]
pub struct TimeStampReq {
    pub version: TimeStampReq_version,
    pub messageImprint: MessageImprint,
    pub reqPolicy: OPTIONAL<TSAPolicyId>,
    pub nonce: OPTIONAL<INTEGER>,
    pub certReq: OPTIONAL<BOOLEAN>,
    pub extensions: OPTIONAL<Extensions>,
}
impl TimeStampReq {
    pub fn new(
        version: TimeStampReq_version,
        messageImprint: MessageImprint,
        reqPolicy: OPTIONAL<TSAPolicyId>,
        nonce: OPTIONAL<INTEGER>,
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
    let mut version_: OPTIONAL<TimeStampReq_version> = None;
    let mut messageImprint_: OPTIONAL<MessageImprint> = None;
    let mut reqPolicy_: OPTIONAL<TSAPolicyId> = None;
    let mut nonce_: OPTIONAL<INTEGER> = None;
    let mut certReq_: OPTIONAL<BOOLEAN> = None;
    let mut extensions_: OPTIONAL<Extensions> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_TimeStampReq_version(_el)?),
            "messageImprint" => messageImprint_ = Some(_decode_MessageImprint(_el)?),
            "reqPolicy" => reqPolicy_ = Some(_decode_TSAPolicyId(_el)?),
            "nonce" => nonce_ = Some(BER.decode_integer(_el)?),
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
    components_.push(_encode_TimeStampReq_version(&value_.version)?);
    components_.push(_encode_MessageImprint(&value_.messageImprint)?);
    if let Some(v_) = &value_.reqPolicy {
        components_.push(_encode_TSAPolicyId(&v_)?);
    }
    if let Some(v_) = &value_.nonce {
        components_.push(BER.encode_integer(&v_)?);
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
            "version" => _validate_TimeStampReq_version(_el)?,
            "messageImprint" => _validate_MessageImprint(_el)?,
            "reqPolicy" => _validate_TSAPolicyId(_el)?,
            "nonce" => BER.validate_integer(_el)?,
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
/// MessageImprint ::= SEQUENCE  {
///     hashAlgorithm                AlgorithmIdentifier,
///     hashedMessage                OCTET STRING  }
/// ```
///
#[derive(Debug, Clone)]
pub struct MessageImprint {
    pub hashAlgorithm: AlgorithmIdentifier,
    pub hashedMessage: OCTET_STRING,
}
impl MessageImprint {
    pub fn new(hashAlgorithm: AlgorithmIdentifier, hashedMessage: OCTET_STRING) -> Self {
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
    let mut hashAlgorithm_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut hashedMessage_: OPTIONAL<OCTET_STRING> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hashAlgorithm" => hashAlgorithm_ = Some(_decode_AlgorithmIdentifier(_el)?),
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
    components_.push(_encode_AlgorithmIdentifier(&value_.hashAlgorithm)?);
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
            "hashAlgorithm" => _validate_AlgorithmIdentifier(_el)?,
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
/// TSAPolicyId  ::=  OBJECT IDENTIFIER
/// ```
pub type TSAPolicyId = OBJECT_IDENTIFIER; // ObjectIdentifierType

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
/// TimeStampResp ::= SEQUENCE  {
///     status                  PKIStatusInfo,
///     timeStampToken          TimeStampToken     OPTIONAL  }
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
///     status        PKIStatus,
///     statusString  PKIFreeText     OPTIONAL,
///     failInfo      PKIFailureInfo  OPTIONAL  }
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
///     granted                (0),
///     -- when the PKIStatus contains the value zero a TimeStampToken, as requested, is present.
///     grantedWithMods        (1),
///     -- when the PKIStatus contains the value one a TimeStampToken, with modifications, is present.
///     rejection              (2),
///     waiting                (3),
///     revocationWarning      (4),
///     -- this message contains a warning that a revocation is
///     -- imminent
///     revocationNotification (5)
///     -- notification that a revocation has occurred
/// }
/// ```
pub type PKIStatus = INTEGER;

pub const PKIStatus_granted: PKIStatus = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const PKIStatus_grantedWithMods: PKIStatus = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const PKIStatus_rejection: PKIStatus = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const PKIStatus_waiting: PKIStatus = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const PKIStatus_revocationWarning: PKIStatus = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const PKIStatus_revocationNotification: PKIStatus = 5; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_PKIStatus(el: &X690Element) -> ASN1Result<PKIStatus> {
    BER.decode_integer(&el)
}

pub fn _encode_PKIStatus(value_: &PKIStatus) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_PKIStatus(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PKIFailureInfo  ::=  BIT STRING {
///     badAlg               (0),
///     -- unrecognized or unsupported Algorithm Identifier
///     badRequest           (2),
///     -- transaction not permitted or supported
///     badDataFormat        (5),
///     -- the data submitted has the wrong format
///     timeNotAvailable    (14),
///     -- the TSA's time source is not available
///     unacceptedPolicy    (15),
///     -- the requested TSA policy is not supported by the TSA.
///     unacceptedExtension (16),
///     -- the requested extension is not supported by the TSA.
///     addInfoNotAvailable (17),
///     -- the additional information requested could not be understood
///     -- or is not available
///     systemFailure       (25)
///     -- the request cannot be handled due to system failure
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

pub const PKIFailureInfo_systemFailure: BIT = 25; /* LONG_NAMED_BIT */

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
/// TimeStampToken  ::=  ContentInfo
/// ```
pub type TimeStampToken = ContentInfo; // DefinedType

pub fn _decode_TimeStampToken(el: &X690Element) -> ASN1Result<TimeStampToken> {
    _decode_ContentInfo(&el)
}

pub fn _encode_TimeStampToken(value_: &TimeStampToken) -> ASN1Result<X690Element> {
    _encode_ContentInfo(&value_)
}

pub fn _validate_TimeStampToken(el: &X690Element) -> ASN1Result<()> {
    _validate_ContentInfo(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TSTInfo ::= SEQUENCE  {
///     version                      INTEGER  { v1(1) },
///     policy                       TSAPolicyId,
///     messageImprint               MessageImprint,
///     -- MUST have the same value as the similar field in
///     -- TimeStampReq
///     serialNumber                 INTEGER,
///     -- Time-Stamping users MUST be ready to accommodate integers
///     -- up to 160 bits.
///     genTime                      GeneralizedTime,
///     accuracy                     Accuracy                 OPTIONAL,
///     ordering                     BOOLEAN             DEFAULT FALSE,
///     nonce                        INTEGER                  OPTIONAL,
///     -- MUST be present if the similar field was present
///     -- in TimeStampReq.  In that case it MUST have the same value.
///     tsa                          [0] GeneralName          OPTIONAL,
///     extensions                   [1] IMPLICIT Extensions  OPTIONAL   }
/// ```
///
#[derive(Debug, Clone)]
pub struct TSTInfo {
    pub version: TSTInfo_version,
    pub policy: TSAPolicyId,
    pub messageImprint: MessageImprint,
    pub serialNumber: INTEGER,
    pub genTime: GeneralizedTime,
    pub accuracy: OPTIONAL<Accuracy>,
    pub ordering: OPTIONAL<BOOLEAN>,
    pub nonce: OPTIONAL<INTEGER>,
    pub tsa: OPTIONAL<GeneralName>,
    pub extensions: OPTIONAL<Extensions>,
}
impl TSTInfo {
    pub fn new(
        version: TSTInfo_version,
        policy: TSAPolicyId,
        messageImprint: MessageImprint,
        serialNumber: INTEGER,
        genTime: GeneralizedTime,
        accuracy: OPTIONAL<Accuracy>,
        ordering: OPTIONAL<BOOLEAN>,
        nonce: OPTIONAL<INTEGER>,
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
    let mut version_: OPTIONAL<TSTInfo_version> = None;
    let mut policy_: OPTIONAL<TSAPolicyId> = None;
    let mut messageImprint_: OPTIONAL<MessageImprint> = None;
    let mut serialNumber_: OPTIONAL<INTEGER> = None;
    let mut genTime_: OPTIONAL<GeneralizedTime> = None;
    let mut accuracy_: OPTIONAL<Accuracy> = None;
    let mut ordering_: OPTIONAL<BOOLEAN> = None;
    let mut nonce_: OPTIONAL<INTEGER> = None;
    let mut tsa_: OPTIONAL<GeneralName> = None;
    let mut extensions_: OPTIONAL<Extensions> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "version" => version_ = Some(_decode_TSTInfo_version(_el)?),
            "policy" => policy_ = Some(_decode_TSAPolicyId(_el)?),
            "messageImprint" => messageImprint_ = Some(_decode_MessageImprint(_el)?),
            "serialNumber" => serialNumber_ = Some(BER.decode_integer(_el)?),
            "genTime" => genTime_ = Some(BER.decode_generalized_time(_el)?),
            "accuracy" => accuracy_ = Some(_decode_Accuracy(_el)?),
            "ordering" => ordering_ = Some(BER.decode_boolean(_el)?),
            "nonce" => nonce_ = Some(BER.decode_integer(_el)?),
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
    components_.push(_encode_TSTInfo_version(&value_.version)?);
    components_.push(_encode_TSAPolicyId(&value_.policy)?);
    components_.push(_encode_MessageImprint(&value_.messageImprint)?);
    components_.push(BER.encode_integer(&value_.serialNumber)?);
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
        components_.push(BER.encode_integer(&v_)?);
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
            "version" => _validate_TSTInfo_version(_el)?,
            "policy" => _validate_TSAPolicyId(_el)?,
            "messageImprint" => _validate_MessageImprint(_el)?,
            "serialNumber" => BER.validate_integer(_el)?,
            "genTime" => BER.validate_generalized_time(_el)?,
            "accuracy" => _validate_Accuracy(_el)?,
            "ordering" => BER.validate_boolean(_el)?,
            "nonce" => BER.validate_integer(_el)?,
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
/// Accuracy ::= SEQUENCE {
///     seconds        INTEGER           OPTIONAL,
///     millis     [0] INTEGER  (1..999) OPTIONAL,
///     micros     [1] INTEGER  (1..999) OPTIONAL  }
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
/// TimeStampReq-version ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type TimeStampReq_version = INTEGER;

pub const TimeStampReq_version_v1: TimeStampReq_version = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_TimeStampReq_version(el: &X690Element) -> ASN1Result<TimeStampReq_version> {
    BER.decode_integer(&el)
}

pub fn _encode_TimeStampReq_version(value_: &TimeStampReq_version) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_TimeStampReq_version(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TSTInfo-version ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type TSTInfo_version = INTEGER;

pub const TSTInfo_version_v1: TSTInfo_version = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_TSTInfo_version(el: &X690Element) -> ASN1Result<TSTInfo_version> {
    BER.decode_integer(&el)
}

pub fn _encode_TSTInfo_version(value_: &TSTInfo_version) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_TSTInfo_version(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}
