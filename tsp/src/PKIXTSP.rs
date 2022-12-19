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
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;
use crate::PKIXCMP::*;
use cms::CryptographicMessageSyntax::{ContentInfo, _decode_ContentInfo, _encode_ContentInfo};
use x500::AuthenticationFramework::{
    AlgorithmIdentifier, Extensions, _decode_AlgorithmIdentifier, _decode_Extensions,
    _encode_AlgorithmIdentifier, _encode_Extensions,
};
use x500::CertificateExtensions::{GeneralName, _decode_GeneralName, _encode_GeneralName};

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ct-TSTInfo  OBJECT IDENTIFIER ::= { iso(1) member-body(2)
/// us(840) rsadsi(113549) pkcs(1) pkcs-9(9) smime(16) ct(1) 4}
/// ```
///
///
pub fn id_ct_TSTInfo() -> OBJECT_IDENTIFIER {
    Vec::<u32>::from([
        /* iso */ 1, /* member-body */ 2, /* us */ 840, /* rsadsi */ 113549,
        /* pkcs */ 1, /* pkcs-9 */ 9, /* smime */ 16, /* ct */ 1, 4,
    ]) // OID_GETTER
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
impl TryFrom<X690Element> for TimeStampReq {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TimeStampReq(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TimeStampReq {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<TimeStampReq> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TimeStampReq,
            _eal_components_for_TimeStampReq,
            _rctl2_components_for_TimeStampReq,
        )?;
        let version = _decode_TimeStampReq_version(_components.get("version").unwrap())?;
        let messageImprint = _decode_MessageImprint(_components.get("messageImprint").unwrap())?;
        let reqPolicy: OPTIONAL<TSAPolicyId> = match _components.get("reqPolicy") {
            Some(c_) => Some(_decode_TSAPolicyId(c_)?),
            _ => None,
        };
        let nonce: OPTIONAL<INTEGER> = match _components.get("nonce") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        let certReq: OPTIONAL<BOOLEAN> = match _components.get("certReq") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let extensions: OPTIONAL<Extensions> = match _components.get("extensions") {
            Some(c_) => Some(_decode_Extensions(c_)?),
            _ => None,
        };
        Ok(TimeStampReq {
            version,
            messageImprint,
            reqPolicy,
            nonce,
            certReq,
            extensions,
        })
    }(&el)
}

pub fn _encode_TimeStampReq(value_: &TimeStampReq) -> ASN1Result<X690Element> {
    |value_: &TimeStampReq| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(_encode_TimeStampReq_version(&value_.version)?);
        components_.push(_encode_MessageImprint(&value_.messageImprint)?);
        if let Some(v_) = &value_.reqPolicy {
            components_.push(_encode_TSAPolicyId(&v_)?);
        }
        if let Some(v_) = &value_.nonce {
            components_.push(ber_encode_integer(&v_)?);
        }
        if let Some(v_) = &value_.certReq {
            if *v_ != TimeStampReq::_default_value_for_certReq() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        if let Some(v_) = &value_.extensions {
            components_.push(|v_1: &Extensions| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Extensions(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MessageImprint ::= SEQUENCE  {
///     hashAlgorithm                AlgorithmIdentifier,
///     hashedMessage                OCTET STRING  }
/// ```
///
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
impl TryFrom<X690Element> for MessageImprint {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_MessageImprint(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for MessageImprint {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<MessageImprint> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_MessageImprint,
            _eal_components_for_MessageImprint,
            _rctl2_components_for_MessageImprint,
        )?;
        let hashAlgorithm = _decode_AlgorithmIdentifier(_components.get("hashAlgorithm").unwrap())?;
        let hashedMessage = ber_decode_octet_string(_components.get("hashedMessage").unwrap())?;
        Ok(MessageImprint {
            hashAlgorithm,
            hashedMessage,
        })
    }(&el)
}

pub fn _encode_MessageImprint(value_: &MessageImprint) -> ASN1Result<X690Element> {
    |value_: &MessageImprint| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_AlgorithmIdentifier(&value_.hashAlgorithm)?);
        components_.push(ber_encode_octet_string(&value_.hashedMessage)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TSAPolicyId  ::=  OBJECT IDENTIFIER
/// ```
pub type TSAPolicyId = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_TSAPolicyId(el: &X690Element) -> ASN1Result<TSAPolicyId> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_TSAPolicyId(value_: &TSAPolicyId) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeStampResp ::= SEQUENCE  {
///     status                  PKIStatusInfo,
///     timeStampToken          TimeStampToken     OPTIONAL  }
/// ```
///
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
impl TryFrom<X690Element> for TimeStampResp {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TimeStampResp(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TimeStampResp {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<TimeStampResp> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TimeStampResp,
            _eal_components_for_TimeStampResp,
            _rctl2_components_for_TimeStampResp,
        )?;
        let status = _decode_PKIStatusInfo(_components.get("status").unwrap())?;
        let timeStampToken: OPTIONAL<TimeStampToken> = match _components.get("timeStampToken") {
            Some(c_) => Some(_decode_TimeStampToken(c_)?),
            _ => None,
        };
        Ok(TimeStampResp {
            status,
            timeStampToken,
        })
    }(&el)
}

pub fn _encode_TimeStampResp(value_: &TimeStampResp) -> ASN1Result<X690Element> {
    |value_: &TimeStampResp| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_PKIStatusInfo(&value_.status)?);
        if let Some(v_) = &value_.timeStampToken {
            components_.push(_encode_TimeStampToken(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for PKIStatusInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PKIStatusInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PKIStatusInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<PKIStatusInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PKIStatusInfo,
            _eal_components_for_PKIStatusInfo,
            _rctl2_components_for_PKIStatusInfo,
        )?;
        let status = _decode_PKIStatus(_components.get("status").unwrap())?;
        let statusString: OPTIONAL<PKIFreeText> = match _components.get("statusString") {
            Some(c_) => Some(_decode_PKIFreeText(c_)?),
            _ => None,
        };
        let failInfo: OPTIONAL<PKIFailureInfo> = match _components.get("failInfo") {
            Some(c_) => Some(_decode_PKIFailureInfo(c_)?),
            _ => None,
        };
        Ok(PKIStatusInfo {
            status,
            statusString,
            failInfo,
        })
    }(&el)
}

pub fn _encode_PKIStatusInfo(value_: &PKIStatusInfo) -> ASN1Result<X690Element> {
    |value_: &PKIStatusInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_PKIStatus(&value_.status)?);
        if let Some(v_) = &value_.statusString {
            components_.push(_encode_PKIFreeText(&v_)?);
        }
        if let Some(v_) = &value_.failInfo {
            components_.push(_encode_PKIFailureInfo(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
    ber_decode_integer(&el)
}

pub fn _encode_PKIStatus(value_: &PKIStatus) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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
    ber_decode_bit_string(&el)
}

pub fn _encode_PKIFailureInfo(value_: &PKIFailureInfo) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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
impl TryFrom<X690Element> for TSTInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TSTInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TSTInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<TSTInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TSTInfo,
            _eal_components_for_TSTInfo,
            _rctl2_components_for_TSTInfo,
        )?;
        let version = _decode_TSTInfo_version(_components.get("version").unwrap())?;
        let policy = _decode_TSAPolicyId(_components.get("policy").unwrap())?;
        let messageImprint = _decode_MessageImprint(_components.get("messageImprint").unwrap())?;
        let serialNumber = ber_decode_integer(_components.get("serialNumber").unwrap())?;
        let genTime = ber_decode_generalized_time(_components.get("genTime").unwrap())?;
        let accuracy: OPTIONAL<Accuracy> = match _components.get("accuracy") {
            Some(c_) => Some(_decode_Accuracy(c_)?),
            _ => None,
        };
        let ordering: OPTIONAL<BOOLEAN> = match _components.get("ordering") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let nonce: OPTIONAL<INTEGER> = match _components.get("nonce") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        let tsa: OPTIONAL<GeneralName> = match _components.get("tsa") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralName> {
                Ok(_decode_GeneralName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let extensions: OPTIONAL<Extensions> = match _components.get("extensions") {
            Some(c_) => Some(_decode_Extensions(c_)?),
            _ => None,
        };
        Ok(TSTInfo {
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
        })
    }(&el)
}

pub fn _encode_TSTInfo(value_: &TSTInfo) -> ASN1Result<X690Element> {
    |value_: &TSTInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(_encode_TSTInfo_version(&value_.version)?);
        components_.push(_encode_TSAPolicyId(&value_.policy)?);
        components_.push(_encode_MessageImprint(&value_.messageImprint)?);
        components_.push(ber_encode_integer(&value_.serialNumber)?);
        components_.push(ber_encode_generalized_time(&value_.genTime)?);
        if let Some(v_) = &value_.accuracy {
            components_.push(_encode_Accuracy(&v_)?);
        }
        if let Some(v_) = &value_.ordering {
            if *v_ != TSTInfo::_default_value_for_ordering() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        if let Some(v_) = &value_.nonce {
            components_.push(ber_encode_integer(&v_)?);
        }
        if let Some(v_) = &value_.tsa {
            components_.push(|v_1: &GeneralName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_GeneralName(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.extensions {
            components_.push(|v_1: &Extensions| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Extensions(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for Accuracy {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Accuracy(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Accuracy {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Accuracy> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Accuracy,
            _eal_components_for_Accuracy,
            _rctl2_components_for_Accuracy,
        )?;
        let seconds: OPTIONAL<INTEGER> = match _components.get("seconds") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        let millis: OPTIONAL<INTEGER> = match _components.get("millis") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        let micros: OPTIONAL<INTEGER> = match _components.get("micros") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        Ok(Accuracy {
            seconds,
            millis,
            micros,
        })
    }(&el)
}

pub fn _encode_Accuracy(value_: &Accuracy) -> ASN1Result<X690Element> {
    |value_: &Accuracy| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        if let Some(v_) = &value_.seconds {
            components_.push(ber_encode_integer(&v_)?);
        }
        if let Some(v_) = &value_.millis {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_integer(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.micros {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_integer(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeStampReq-version ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type TimeStampReq_version = INTEGER;

pub const TimeStampReq_version_v1: TimeStampReq_version = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_TimeStampReq_version(el: &X690Element) -> ASN1Result<TimeStampReq_version> {
    ber_decode_integer(&el)
}

pub fn _encode_TimeStampReq_version(value_: &TimeStampReq_version) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TSTInfo-version ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type TSTInfo_version = INTEGER;

pub const TSTInfo_version_v1: TSTInfo_version = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_TSTInfo_version(el: &X690Element) -> ASN1Result<TSTInfo_version> {
    ber_decode_integer(&el)
}

pub fn _encode_TSTInfo_version(value_: &TSTInfo_version) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}
