#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # Lightweight-Directory-Access-Protocol-V3
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `Lightweight-Directory-Access-Protocol-V3`.
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPMessage ::= SEQUENCE {
///   messageID   MessageID,
///   protocolOp
///     CHOICE {bindRequest           BindRequest,
///             bindResponse          BindResponse,
///             unbindRequest         UnbindRequest,
///             searchRequest         SearchRequest,
///             searchResEntry        SearchResultEntry,
///             searchResDone         SearchResultDone,
///             searchResRef          SearchResultReference,
///             modifyRequest         ModifyRequest,
///             modifyResponse        ModifyResponse,
///             addRequest            AddRequest,
///             addResponse           AddResponse,
///             delRequest            DelRequest,
///             delResponse           DelResponse,
///             modDNRequest          ModifyDNRequest,
///             modDNResponse         ModifyDNResponse,
///             compareRequest        CompareRequest,
///             compareResponse       CompareResponse,
///             abandonRequest        AbandonRequest,
///             extendedReq           ExtendedRequest,
///             extendedResp          ExtendedResponse,
///             ...,
///             intermediateResponse  IntermediateResponse},
///   controls    [0]  Controls OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct LDAPMessage {
    pub messageID: MessageID,
    pub protocolOp: LDAPMessage_protocolOp,
    pub controls: OPTIONAL<Controls>,
    pub _unrecognized: Vec<X690Element>,
}
impl LDAPMessage {}
impl TryFrom<X690Element> for LDAPMessage {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_LDAPMessage(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for LDAPMessage {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_LDAPMessage(el)
    }
}

pub const _rctl1_components_for_LDAPMessage: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "messageID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("protocolOp", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "controls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_LDAPMessage: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_LDAPMessage: &[ComponentSpec; 0] = &[];

pub fn _decode_LDAPMessage(el: &X690Element) -> ASN1Result<LDAPMessage> {
    |el_: &X690Element| -> ASN1Result<LDAPMessage> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_LDAPMessage,
            _eal_components_for_LDAPMessage,
            _rctl2_components_for_LDAPMessage,
        )?;
        let messageID = _decode_MessageID(_components.get("messageID").unwrap())?;
        let protocolOp = _decode_LDAPMessage_protocolOp(_components.get("protocolOp").unwrap())?;
        let controls: OPTIONAL<Controls> = match _components.get("controls") {
            Some(c_) => Some(_decode_Controls(c_)?),
            _ => None,
        };
        Ok(LDAPMessage {
            messageID,
            protocolOp,
            controls,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_LDAPMessage(value_: &LDAPMessage) -> ASN1Result<X690Element> {
    |value_: &LDAPMessage| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_MessageID(&value_.messageID)?);
        components_.push(_encode_LDAPMessage_protocolOp(&value_.protocolOp)?);
        if let Some(v_) = &value_.controls {
            components_.push(|v_1: &Controls| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Controls(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v_)?);
        }
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
/// MessageID  ::=  INTEGER(0..maxInt)
/// ```
pub type MessageID = INTEGER;

pub fn _decode_MessageID(el: &X690Element) -> ASN1Result<MessageID> {
    ber_decode_integer(&el)
}

pub fn _encode_MessageID(value_: &MessageID) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// maxInt INTEGER ::= 2147483647
/// ```
///
///
pub const maxInt: i32 = 2147483647;

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPString  ::=  OCTET STRING
/// ```
pub type LDAPString = OCTET_STRING; // OctetStringType

pub fn _decode_LDAPString(el: &X690Element) -> ASN1Result<LDAPString> {
    ber_decode_octet_string(&el)
}

pub fn _encode_LDAPString(value_: &LDAPString) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPOID  ::=  OCTET STRING
/// ```
pub type LDAPOID = OCTET_STRING; // OctetStringType

pub fn _decode_LDAPOID(el: &X690Element) -> ASN1Result<LDAPOID> {
    ber_decode_octet_string(&el)
}

pub fn _encode_LDAPOID(value_: &LDAPOID) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPDN  ::=  LDAPString
/// ```
pub type LDAPDN = LDAPString; // DefinedType

pub fn _decode_LDAPDN(el: &X690Element) -> ASN1Result<LDAPDN> {
    _decode_LDAPString(&el)
}

pub fn _encode_LDAPDN(value_: &LDAPDN) -> ASN1Result<X690Element> {
    _encode_LDAPString(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RelativeLDAPDN  ::=
///   LDAPString
/// ```
pub type RelativeLDAPDN = LDAPString; // DefinedType

pub fn _decode_RelativeLDAPDN(el: &X690Element) -> ASN1Result<RelativeLDAPDN> {
    _decode_LDAPString(&el)
}

pub fn _encode_RelativeLDAPDN(value_: &RelativeLDAPDN) -> ASN1Result<X690Element> {
    _encode_LDAPString(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeDescription  ::=  LDAPString
/// ```
pub type AttributeDescription = LDAPString; // DefinedType

pub fn _decode_AttributeDescription(el: &X690Element) -> ASN1Result<AttributeDescription> {
    _decode_LDAPString(&el)
}

pub fn _encode_AttributeDescription(value_: &AttributeDescription) -> ASN1Result<X690Element> {
    _encode_LDAPString(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeValue  ::=  OCTET STRING
/// ```
pub type AttributeValue = OCTET_STRING; // OctetStringType

pub fn _decode_AttributeValue(el: &X690Element) -> ASN1Result<AttributeValue> {
    ber_decode_octet_string(&el)
}

pub fn _encode_AttributeValue(value_: &AttributeValue) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeValueAssertion ::= SEQUENCE {
///   attributeDesc   AttributeDescription,
///   assertionValue  AssertionValue
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeValueAssertion {
    pub attributeDesc: AttributeDescription,
    pub assertionValue: AssertionValue,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeValueAssertion {}
impl TryFrom<X690Element> for AttributeValueAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeValueAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeValueAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeValueAssertion(el)
    }
}

pub const _rctl1_components_for_AttributeValueAssertion: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "attributeDesc",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assertionValue",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeValueAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeValueAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeValueAssertion(el: &X690Element) -> ASN1Result<AttributeValueAssertion> {
    |el_: &X690Element| -> ASN1Result<AttributeValueAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeValueAssertion,
            _eal_components_for_AttributeValueAssertion,
            _rctl2_components_for_AttributeValueAssertion,
        )?;
        let attributeDesc =
            _decode_AttributeDescription(_components.get("attributeDesc").unwrap())?;
        let assertionValue = _decode_AssertionValue(_components.get("assertionValue").unwrap())?;
        Ok(AttributeValueAssertion {
            attributeDesc,
            assertionValue,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeValueAssertion(
    value_: &AttributeValueAssertion,
) -> ASN1Result<X690Element> {
    |value_: &AttributeValueAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AttributeDescription(&value_.attributeDesc)?);
        components_.push(_encode_AssertionValue(&value_.assertionValue)?);
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
/// AssertionValue  ::=  OCTET STRING
/// ```
pub type AssertionValue = OCTET_STRING; // OctetStringType

pub fn _decode_AssertionValue(el: &X690Element) -> ASN1Result<AssertionValue> {
    ber_decode_octet_string(&el)
}

pub fn _encode_AssertionValue(value_: &AssertionValue) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PartialAttribute ::= SEQUENCE {
///   type  AttributeDescription,
///   vals  SET OF value AttributeValue
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PartialAttribute {
    pub type_: AttributeDescription,
    pub vals: Vec<AttributeValue>,
    pub _unrecognized: Vec<X690Element>,
}
impl PartialAttribute {}
impl TryFrom<X690Element> for PartialAttribute {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PartialAttribute(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PartialAttribute {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PartialAttribute(el)
    }
}

pub const _rctl1_components_for_PartialAttribute: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "vals",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PartialAttribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PartialAttribute: &[ComponentSpec; 0] = &[];

pub fn _decode_PartialAttribute(el: &X690Element) -> ASN1Result<PartialAttribute> {
    |el_: &X690Element| -> ASN1Result<PartialAttribute> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PartialAttribute,
            _eal_components_for_PartialAttribute,
            _rctl2_components_for_PartialAttribute,
        )?;
        let type_ = _decode_AttributeDescription(_components.get("type").unwrap())?;
        let vals = |el: &X690Element| -> ASN1Result<SET_OF<AttributeValue>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SET_OF<AttributeValue> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_AttributeValue(el)?);
            }
            Ok(items)
        }(_components.get("vals").unwrap())?;
        Ok(PartialAttribute {
            type_,
            vals,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PartialAttribute(value_: &PartialAttribute) -> ASN1Result<X690Element> {
    |value_: &PartialAttribute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AttributeDescription(&value_.type_)?);
        components_.push(
            |value_: &SET_OF<AttributeValue>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_AttributeValue(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.vals)?,
        );
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
/// Attribute  ::=  PartialAttribute(WITH COMPONENTS {
///                                  ...,
///                                  vals  (SIZE (1..MAX))
///                                })
/// ```
pub type Attribute = PartialAttribute; // DefinedType

pub fn _decode_Attribute(el: &X690Element) -> ASN1Result<Attribute> {
    _decode_PartialAttribute(&el)
}

pub fn _encode_Attribute(value_: &Attribute) -> ASN1Result<X690Element> {
    _encode_PartialAttribute(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MatchingRuleId  ::=  LDAPString
/// ```
pub type MatchingRuleId = LDAPString; // DefinedType

pub fn _decode_MatchingRuleId(el: &X690Element) -> ASN1Result<MatchingRuleId> {
    _decode_LDAPString(&el)
}

pub fn _encode_MatchingRuleId(value_: &MatchingRuleId) -> ASN1Result<X690Element> {
    _encode_LDAPString(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPResult ::= SEQUENCE {
///   resultCode
///     ENUMERATED {success(0), operationsError(1), protocolError(2),
///                 timeLimitExceeded(3), sizeLimitExceeded(4), compareFalse(5),
///                 compareTrue(6), authMethodNotSupported(7),
///                 strongerAuthRequired(8),
///                 -- 9 reserved
///                 referral(10), adminLimitExceeded(11),
///                 unavailableCriticalExtension(12), confidentialityRequired(13),
///                 saslBindInProgress(14), noSuchAttribute(16),
///                 undefinedAttributeType(17), inappropriateMatching(18),
///                 constraintViolation(19), attributeOrValueExists(20),
///                 invalidAttributeSyntax(21),
///                 -- 22-31 unused
///                 noSuchObject(32), aliasProblem(33),
///                 invalidDNSyntax(34),
///                 -- 35 reserved for undefined isLeaf
///                 aliasDereferencingProblem(36),
///                 -- 37-47 unused
///                 inappropriateAuthentication(48), invalidCredentials(49),
///                 insufficientAccessRights(50), busy(51), unavailable(52),
///                 unwillingToPerform(53),
///                 loopDetect(54),
///                 -- 55-63 unused
///                 namingViolation(64), objectClassViolation(65),
///                 notAllowedOnNonLeaf(66), notAllowedOnRDN(67),
///                 entryAlreadyExists(68),
///                 objectClassModsProhibited(69),
///                 -- 70 reserved for CLDAP
///                 affectsMultipleDSAs(71),
///                 -- 72-79 unused
///                 other(80), ...
///                 },
///   matchedDN          LDAPDN,
///   diagnosticMessage  LDAPString,
///   referral           [3]  Referral OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct LDAPResult {
    pub resultCode: LDAPResult_resultCode,
    pub matchedDN: LDAPDN,
    pub diagnosticMessage: LDAPString,
    pub referral: OPTIONAL<Referral>,
    pub _unrecognized: Vec<X690Element>,
}
impl LDAPResult {}
impl TryFrom<X690Element> for LDAPResult {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_LDAPResult(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for LDAPResult {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_LDAPResult(el)
    }
}

pub const _rctl1_components_for_LDAPResult: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "resultCode",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "matchedDN",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "diagnosticMessage",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referral",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_LDAPResult: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_LDAPResult: &[ComponentSpec; 0] = &[];

pub fn _decode_LDAPResult(el: &X690Element) -> ASN1Result<LDAPResult> {
    |el_: &X690Element| -> ASN1Result<LDAPResult> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_LDAPResult,
            _eal_components_for_LDAPResult,
            _rctl2_components_for_LDAPResult,
        )?;
        let resultCode = _decode_LDAPResult_resultCode(_components.get("resultCode").unwrap())?;
        let matchedDN = _decode_LDAPDN(_components.get("matchedDN").unwrap())?;
        let diagnosticMessage = _decode_LDAPString(_components.get("diagnosticMessage").unwrap())?;
        let referral: OPTIONAL<Referral> = match _components.get("referral") {
            Some(c_) => Some(_decode_Referral(c_)?),
            _ => None,
        };
        Ok(LDAPResult {
            resultCode,
            matchedDN,
            diagnosticMessage,
            referral,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_LDAPResult(value_: &LDAPResult) -> ASN1Result<X690Element> {
    |value_: &LDAPResult| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(_encode_LDAPResult_resultCode(&value_.resultCode)?);
        components_.push(_encode_LDAPDN(&value_.matchedDN)?);
        components_.push(_encode_LDAPString(&value_.diagnosticMessage)?);
        if let Some(v_) = &value_.referral {
            components_.push(|v_1: &Referral| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Referral(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v_)?);
        }
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
/// Referral  ::=  SEQUENCE SIZE (1..MAX) OF uri URI
/// ```
pub type Referral = Vec<URI>; // SequenceOfType

pub fn _decode_Referral(el: &X690Element) -> ASN1Result<Referral> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<URI>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<URI> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_URI(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_Referral(value_: &Referral) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<URI>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_URI(&v)?);
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
/// URI  ::=  LDAPString
/// ```
pub type URI = LDAPString; // DefinedType

pub fn _decode_URI(el: &X690Element) -> ASN1Result<URI> {
    _decode_LDAPString(&el)
}

pub fn _encode_URI(value_: &URI) -> ASN1Result<X690Element> {
    _encode_LDAPString(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Controls  ::=  SEQUENCE OF control Control
/// ```
pub type Controls = Vec<Control>; // SequenceOfType

pub fn _decode_Controls(el: &X690Element) -> ASN1Result<Controls> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<Control>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<Control> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_Control(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_Controls(value_: &Controls) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<Control>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Control(&v)?);
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
/// Control ::= SEQUENCE {
///   controlType   LDAPOID,
///   criticality   BOOLEAN DEFAULT FALSE,
///   controlValue  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Control {
    pub controlType: LDAPOID,
    pub criticality: OPTIONAL<BOOLEAN>,
    pub controlValue: OPTIONAL<OCTET_STRING>,
    pub _unrecognized: Vec<X690Element>,
}
impl Control {
    pub fn _default_value_for_criticality() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for Control {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Control(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Control {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Control(el)
    }
}

pub const _rctl1_components_for_Control: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "controlType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criticality",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "controlValue",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Control: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Control: &[ComponentSpec; 0] = &[];

pub fn _decode_Control(el: &X690Element) -> ASN1Result<Control> {
    |el_: &X690Element| -> ASN1Result<Control> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Control,
            _eal_components_for_Control,
            _rctl2_components_for_Control,
        )?;
        let controlType = _decode_LDAPOID(_components.get("controlType").unwrap())?;
        let criticality: OPTIONAL<BOOLEAN> = match _components.get("criticality") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let controlValue: OPTIONAL<OCTET_STRING> = match _components.get("controlValue") {
            Some(c_) => Some(ber_decode_octet_string(c_)?),
            _ => None,
        };
        Ok(Control {
            controlType,
            criticality,
            controlValue,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Control(value_: &Control) -> ASN1Result<X690Element> {
    |value_: &Control| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_LDAPOID(&value_.controlType)?);
        if let Some(v_) = &value_.criticality {
            if *v_ != Control::_default_value_for_criticality() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        if let Some(v_) = &value_.controlValue {
            components_.push(ber_encode_octet_string(&v_)?);
        }
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
/// BindRequest ::= [APPLICATION 0]  SEQUENCE {
///   version         INTEGER(1..127),
///   name            LDAPDN,
///   authentication  AuthenticationChoice
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct BindRequest {
    pub version: INTEGER,
    pub name: LDAPDN,
    pub authentication: AuthenticationChoice,
    pub _unrecognized: Vec<X690Element>,
}
impl BindRequest {}
impl TryFrom<X690Element> for BindRequest {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_BindRequest(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for BindRequest {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_BindRequest(el)
    }
}

pub const _rctl1_components_for_BindRequest: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "version",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new("authentication", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_BindRequest: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_BindRequest: &[ComponentSpec; 0] = &[];

pub fn _decode_BindRequest(el: &X690Element) -> ASN1Result<BindRequest> {
    |el_: &X690Element| -> ASN1Result<BindRequest> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_BindRequest,
            _eal_components_for_BindRequest,
            _rctl2_components_for_BindRequest,
        )?;
        let version = ber_decode_integer(_components.get("version").unwrap())?;
        let name = _decode_LDAPDN(_components.get("name").unwrap())?;
        let authentication =
            _decode_AuthenticationChoice(_components.get("authentication").unwrap())?;
        Ok(BindRequest {
            version,
            name,
            authentication,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_BindRequest(value_: &BindRequest) -> ASN1Result<X690Element> {
    |v_1: &BindRequest| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &BindRequest| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(13);
            components_.push(ber_encode_integer(&value_.version)?);
            components_.push(_encode_LDAPDN(&value_.name)?);
            components_.push(_encode_AuthenticationChoice(&value_.authentication)?);
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 0;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthenticationChoice  ::=  CHOICE {
///   simple  [0]  OCTET STRING,
///   -- 1 and 2 reserved
///   sasl    [3]  SaslCredentials,
///   ...
/// }
/// ```
#[derive(Debug, Clone)]
pub enum AuthenticationChoice {
    simple(OCTET_STRING),
    sasl(SaslCredentials),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for AuthenticationChoice {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AuthenticationChoice(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AuthenticationChoice {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AuthenticationChoice(el)
    }
}

pub fn _decode_AuthenticationChoice(el: &X690Element) -> ASN1Result<AuthenticationChoice> {
    |el: &X690Element| -> ASN1Result<AuthenticationChoice> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => {
                Ok(AuthenticationChoice::simple(ber_decode_octet_string(&el)?))
            }
            (TagClass::CONTEXT, 3) => Ok(AuthenticationChoice::sasl(_decode_SaslCredentials(&el)?)),
            _ => Ok(AuthenticationChoice::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_AuthenticationChoice(value_: &AuthenticationChoice) -> ASN1Result<X690Element> {
    |value: &AuthenticationChoice| -> ASN1Result<X690Element> {
        match value {
            AuthenticationChoice::simple(v) => |v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_octet_string(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            AuthenticationChoice::sasl(v) => |v_1: &SaslCredentials| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SaslCredentials(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v),
            AuthenticationChoice::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SaslCredentials ::= SEQUENCE {
///   mechanism    LDAPString,
///   credentials  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SaslCredentials {
    pub mechanism: LDAPString,
    pub credentials: OPTIONAL<OCTET_STRING>,
    pub _unrecognized: Vec<X690Element>,
}
impl SaslCredentials {}
impl TryFrom<X690Element> for SaslCredentials {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SaslCredentials(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SaslCredentials {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SaslCredentials(el)
    }
}

pub const _rctl1_components_for_SaslCredentials: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "mechanism",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "credentials",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SaslCredentials: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SaslCredentials: &[ComponentSpec; 0] = &[];

pub fn _decode_SaslCredentials(el: &X690Element) -> ASN1Result<SaslCredentials> {
    |el_: &X690Element| -> ASN1Result<SaslCredentials> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SaslCredentials,
            _eal_components_for_SaslCredentials,
            _rctl2_components_for_SaslCredentials,
        )?;
        let mechanism = _decode_LDAPString(_components.get("mechanism").unwrap())?;
        let credentials: OPTIONAL<OCTET_STRING> = match _components.get("credentials") {
            Some(c_) => Some(ber_decode_octet_string(c_)?),
            _ => None,
        };
        Ok(SaslCredentials {
            mechanism,
            credentials,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SaslCredentials(value_: &SaslCredentials) -> ASN1Result<X690Element> {
    |value_: &SaslCredentials| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_LDAPString(&value_.mechanism)?);
        if let Some(v_) = &value_.credentials {
            components_.push(ber_encode_octet_string(&v_)?);
        }
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
/// BindResponse ::= [APPLICATION 1]  SEQUENCE {
///   COMPONENTS OF LDAPResult,
///   serverSaslCreds  [7]  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct BindResponse {
    pub resultCode: LDAPResult_resultCode, /* REPLICATED_COMPONENT */
    pub matchedDN: LDAPDN,                 /* REPLICATED_COMPONENT */
    pub diagnosticMessage: LDAPString,     /* REPLICATED_COMPONENT */
    pub referral: OPTIONAL<Referral>,      /* REPLICATED_COMPONENT */
    pub serverSaslCreds: OPTIONAL<OCTET_STRING>,
    pub _unrecognized: Vec<X690Element>,
}
impl BindResponse {}
impl TryFrom<X690Element> for BindResponse {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_BindResponse(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for BindResponse {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_BindResponse(el)
    }
}

pub const _rctl1_components_for_BindResponse: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "resultCode",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "matchedDN",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "diagnosticMessage",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referral",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "serverSaslCreds",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_BindResponse: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_BindResponse: &[ComponentSpec; 0] = &[];

pub fn _decode_BindResponse(el: &X690Element) -> ASN1Result<BindResponse> {
    |el_: &X690Element| -> ASN1Result<BindResponse> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_BindResponse,
            _eal_components_for_BindResponse,
            _rctl2_components_for_BindResponse,
        )?;
        let resultCode = _decode_LDAPResult_resultCode(_components.get("resultCode").unwrap())?;
        let matchedDN = _decode_LDAPDN(_components.get("matchedDN").unwrap())?;
        let diagnosticMessage = _decode_LDAPString(_components.get("diagnosticMessage").unwrap())?;
        let referral: OPTIONAL<Referral> = match _components.get("referral") {
            Some(c_) => Some(_decode_Referral(c_)?),
            _ => None,
        };
        let serverSaslCreds: OPTIONAL<OCTET_STRING> = match _components.get("serverSaslCreds") {
            Some(c_) => Some(ber_decode_octet_string(c_)?),
            _ => None,
        };
        Ok(BindResponse {
            resultCode,
            matchedDN,
            diagnosticMessage,
            referral,
            serverSaslCreds,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_BindResponse(value_: &BindResponse) -> ASN1Result<X690Element> {
    |v_1: &BindResponse| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &BindResponse| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(15);
            components_.push(_encode_LDAPResult_resultCode(&value_.resultCode)?);
            components_.push(_encode_LDAPDN(&value_.matchedDN)?);
            components_.push(_encode_LDAPString(&value_.diagnosticMessage)?);
            if let Some(v_) = &value_.referral {
                components_.push(|v_1: &Referral| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_Referral(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 3;
                    Ok(el_1)
                }(&v_)?);
            }
            if let Some(v_) = &value_.serverSaslCreds {
                components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_octet_string(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 7;
                    Ok(el_1)
                }(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 1;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnbindRequest  ::=  [APPLICATION 2]  NULL
/// ```
pub type UnbindRequest = NULL; // NullType

pub fn _decode_UnbindRequest(el: &X690Element) -> ASN1Result<UnbindRequest> {
    ber_decode_null(&el)?;
    Ok(())
}

pub fn _encode_UnbindRequest(value_: &UnbindRequest) -> ASN1Result<X690Element> {
    |v_1: &UnbindRequest| -> ASN1Result<X690Element> {
        let mut el_1 = ber_encode_null(&())?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 2;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchRequest ::= [APPLICATION 3]  SEQUENCE {
///   baseObject    LDAPDN,
///   scope
///     ENUMERATED {baseObject(0), singleLevel(1), wholeSubtree(2), ...
///                 },
///   derefAliases
///     ENUMERATED {neverDerefAliases(0), derefInSearching(1),
///                 derefFindingBaseObj(2), derefAlways(3)},
///   sizeLimit     INTEGER(0..maxInt),
///   timeLimit     INTEGER(0..maxInt),
///   typesOnly     BOOLEAN,
///   filter        Filter,
///   attributes    AttributeSelection
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SearchRequest {
    pub baseObject: LDAPDN,
    pub scope: SearchRequest_scope,
    pub derefAliases: SearchRequest_derefAliases,
    pub sizeLimit: INTEGER,
    pub timeLimit: INTEGER,
    pub typesOnly: BOOLEAN,
    pub filter: Filter,
    pub attributes: AttributeSelection,
    pub _unrecognized: Vec<X690Element>,
}
impl SearchRequest {}
impl TryFrom<X690Element> for SearchRequest {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SearchRequest(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SearchRequest {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SearchRequest(el)
    }
}

pub const _rctl1_components_for_SearchRequest: &[ComponentSpec; 8] = &[
    ComponentSpec::new(
        "baseObject",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "scope",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "derefAliases",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sizeLimit",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeLimit",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "typesOnly",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new("filter", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "attributes",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SearchRequest: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SearchRequest: &[ComponentSpec; 0] = &[];

pub fn _decode_SearchRequest(el: &X690Element) -> ASN1Result<SearchRequest> {
    |el_: &X690Element| -> ASN1Result<SearchRequest> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SearchRequest,
            _eal_components_for_SearchRequest,
            _rctl2_components_for_SearchRequest,
        )?;
        let baseObject = _decode_LDAPDN(_components.get("baseObject").unwrap())?;
        let scope = _decode_SearchRequest_scope(_components.get("scope").unwrap())?;
        let derefAliases =
            _decode_SearchRequest_derefAliases(_components.get("derefAliases").unwrap())?;
        let sizeLimit = ber_decode_integer(_components.get("sizeLimit").unwrap())?;
        let timeLimit = ber_decode_integer(_components.get("timeLimit").unwrap())?;
        let typesOnly = ber_decode_boolean(_components.get("typesOnly").unwrap())?;
        let filter = _decode_Filter(_components.get("filter").unwrap())?;
        let attributes = _decode_AttributeSelection(_components.get("attributes").unwrap())?;
        Ok(SearchRequest {
            baseObject,
            scope,
            derefAliases,
            sizeLimit,
            timeLimit,
            typesOnly,
            filter,
            attributes,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SearchRequest(value_: &SearchRequest) -> ASN1Result<X690Element> {
    |v_1: &SearchRequest| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &SearchRequest| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(18);
            components_.push(_encode_LDAPDN(&value_.baseObject)?);
            components_.push(_encode_SearchRequest_scope(&value_.scope)?);
            components_.push(_encode_SearchRequest_derefAliases(&value_.derefAliases)?);
            components_.push(ber_encode_integer(&value_.sizeLimit)?);
            components_.push(ber_encode_integer(&value_.timeLimit)?);
            components_.push(ber_encode_boolean(&value_.typesOnly)?);
            components_.push(_encode_Filter(&value_.filter)?);
            components_.push(_encode_AttributeSelection(&value_.attributes)?);
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 3;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeSelection  ::=  SEQUENCE OF selector LDAPString
/// ```
pub type AttributeSelection = Vec<LDAPString>; // SequenceOfType

pub fn _decode_AttributeSelection(el: &X690Element) -> ASN1Result<AttributeSelection> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<LDAPString>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<LDAPString> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_LDAPString(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AttributeSelection(value_: &AttributeSelection) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<LDAPString>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_LDAPString(&v)?);
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
/// Filter  ::=  CHOICE {
///   and              [0]  SET SIZE (1..MAX) OF filter Filter,
///   or               [1]  SET SIZE (1..MAX) OF filter Filter,
///   not              [2]  Filter,
///   equalityMatch    [3]  AttributeValueAssertion,
///   substrings       [4]  SubstringFilter,
///   greaterOrEqual   [5]  AttributeValueAssertion,
///   lessOrEqual      [6]  AttributeValueAssertion,
///   present          [7]  AttributeDescription,
///   approxMatch      [8]  AttributeValueAssertion,
///   extensibleMatch  [9]  MatchingRuleAssertion,
///   ...
/// }
/// ```
// TODO: CHECK_RECURSIVE_DEFINITION
#[derive(Debug, Clone)]
pub enum Filter {
    and(Vec<Box<Filter>>),
    or(Vec<Box<Filter>>),
    not(Box<Filter>),
    equalityMatch(AttributeValueAssertion),
    substrings(SubstringFilter),
    greaterOrEqual(AttributeValueAssertion),
    lessOrEqual(AttributeValueAssertion),
    present(AttributeDescription),
    approxMatch(AttributeValueAssertion),
    extensibleMatch(MatchingRuleAssertion),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for Filter {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Filter(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Filter {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Filter(el)
    }
}

pub fn _decode_Filter(el: &X690Element) -> ASN1Result<Filter> {
    |el: &X690Element| -> ASN1Result<Filter> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(Filter::and(
                |el: &X690Element| -> ASN1Result<SET_OF<Box<Filter>>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<Box<Filter>> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(Box::new(_decode_Filter(el)?));
                    }
                    Ok(items)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(Filter::or(
                |el: &X690Element| -> ASN1Result<SET_OF<Box<Filter>>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<Box<Filter>> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(Box::new(_decode_Filter(el)?));
                    }
                    Ok(items)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 2) => {
                Ok(Filter::not(|el: &X690Element| -> ASN1Result<Box<Filter>> {
                    Ok(Box::new(_decode_Filter(&el.inner()?)?))
                }(&el)?))
            }
            (TagClass::CONTEXT, 3) => {
                Ok(Filter::equalityMatch(_decode_AttributeValueAssertion(&el)?))
            }
            (TagClass::CONTEXT, 4) => Ok(Filter::substrings(_decode_SubstringFilter(&el)?)),
            (TagClass::CONTEXT, 5) => Ok(Filter::greaterOrEqual(_decode_AttributeValueAssertion(
                &el,
            )?)),
            (TagClass::CONTEXT, 6) => {
                Ok(Filter::lessOrEqual(_decode_AttributeValueAssertion(&el)?))
            }
            (TagClass::CONTEXT, 7) => Ok(Filter::present(_decode_AttributeDescription(&el)?)),
            (TagClass::CONTEXT, 8) => {
                Ok(Filter::approxMatch(_decode_AttributeValueAssertion(&el)?))
            }
            (TagClass::CONTEXT, 9) => {
                Ok(Filter::extensibleMatch(_decode_MatchingRuleAssertion(&el)?))
            }
            _ => Ok(Filter::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Filter(value_: &Filter) -> ASN1Result<X690Element> {
    |value: &Filter| -> ASN1Result<X690Element> {
        match value {
            Filter::and(v) => |v_1: &Vec<Box<Filter>>| -> ASN1Result<X690Element> {
                let mut el_1 = |value_: &SET_OF<Box<Filter>>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_Filter(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            Filter::or(v) => |v_1: &Vec<Box<Filter>>| -> ASN1Result<X690Element> {
                let mut el_1 = |value_: &SET_OF<Box<Filter>>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_Filter(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            Filter::not(v) => |v_1: &Box<Filter>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Filter(&v_1)?))),
                ))
            }(&v),
            Filter::equalityMatch(v) => {
                |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AttributeValueAssertion(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 3;
                    Ok(el_1)
                }(&v)
            }
            Filter::substrings(v) => |v_1: &SubstringFilter| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SubstringFilter(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 4;
                Ok(el_1)
            }(&v),
            Filter::greaterOrEqual(v) => {
                |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AttributeValueAssertion(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 5;
                    Ok(el_1)
                }(&v)
            }
            Filter::lessOrEqual(v) => |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeValueAssertion(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 6;
                Ok(el_1)
            }(&v),
            Filter::present(v) => |v_1: &AttributeDescription| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeDescription(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 7;
                Ok(el_1)
            }(&v),
            Filter::approxMatch(v) => |v_1: &AttributeValueAssertion| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeValueAssertion(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 8;
                Ok(el_1)
            }(&v),
            Filter::extensibleMatch(v) => {
                |v_1: &MatchingRuleAssertion| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_MatchingRuleAssertion(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 9;
                    Ok(el_1)
                }(&v)
            }
            Filter::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubstringFilter ::= SEQUENCE {
///   type        AttributeDescription,
///   substrings
///     SEQUENCE SIZE (1..MAX) OF substring
///       CHOICE {initial  [0]  AssertionValue, -- can occur at most once--
///               any      [1]  AssertionValue,
///               final    [2]  AssertionValue} -- can occur at most once
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SubstringFilter {
    pub type_: AttributeDescription,
    pub substrings: Vec<SubstringFilter_substrings_substring>,
    pub _unrecognized: Vec<X690Element>,
}
impl SubstringFilter {}
impl TryFrom<X690Element> for SubstringFilter {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SubstringFilter(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SubstringFilter {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SubstringFilter(el)
    }
}

pub const _rctl1_components_for_SubstringFilter: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "substrings",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SubstringFilter: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SubstringFilter: &[ComponentSpec; 0] = &[];

pub fn _decode_SubstringFilter(el: &X690Element) -> ASN1Result<SubstringFilter> {
    |el_: &X690Element| -> ASN1Result<SubstringFilter> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SubstringFilter,
            _eal_components_for_SubstringFilter,
            _rctl2_components_for_SubstringFilter,
        )?;
        let type_ = _decode_AttributeDescription(_components.get("type").unwrap())?;
        let substrings =
            |el: &X690Element| -> ASN1Result<SEQUENCE_OF<SubstringFilter_substrings_substring>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SEQUENCE_OF<SubstringFilter_substrings_substring> =
                    Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_SubstringFilter_substrings_substring(el)?);
                }
                Ok(items)
            }(_components.get("substrings").unwrap())?;
        Ok(SubstringFilter {
            type_,
            substrings,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SubstringFilter(value_: &SubstringFilter) -> ASN1Result<X690Element> {
    |value_: &SubstringFilter| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AttributeDescription(&value_.type_)?);
        components_.push(|value_: &SEQUENCE_OF<
            SubstringFilter_substrings_substring,
        >|
         -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_SubstringFilter_substrings_substring(&v)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                Arc::new(X690Encoding::Constructed(children)),
            ))
        }(&value_.substrings)?);
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
/// MatchingRuleAssertion ::= SEQUENCE {
///   matchingRule  [1]  MatchingRuleId OPTIONAL,
///   type          [2]  AttributeDescription OPTIONAL,
///   matchValue    [3]  AssertionValue,
///   dnAttributes  [4]  BOOLEAN DEFAULT FALSE
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct MatchingRuleAssertion {
    pub matchingRule: OPTIONAL<MatchingRuleId>,
    pub type_: OPTIONAL<AttributeDescription>,
    pub matchValue: AssertionValue,
    pub dnAttributes: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl MatchingRuleAssertion {
    pub fn _default_value_for_dnAttributes() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for MatchingRuleAssertion {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_MatchingRuleAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for MatchingRuleAssertion {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_MatchingRuleAssertion(el)
    }
}

pub const _rctl1_components_for_MatchingRuleAssertion: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "matchingRule",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "type",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "matchValue",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "dnAttributes",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_MatchingRuleAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MatchingRuleAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_MatchingRuleAssertion(el: &X690Element) -> ASN1Result<MatchingRuleAssertion> {
    |el_: &X690Element| -> ASN1Result<MatchingRuleAssertion> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_MatchingRuleAssertion,
            _eal_components_for_MatchingRuleAssertion,
            _rctl2_components_for_MatchingRuleAssertion,
        )?;
        let matchingRule: OPTIONAL<MatchingRuleId> = match _components.get("matchingRule") {
            Some(c_) => Some(_decode_MatchingRuleId(c_)?),
            _ => None,
        };
        let type_: OPTIONAL<AttributeDescription> = match _components.get("type") {
            Some(c_) => Some(_decode_AttributeDescription(c_)?),
            _ => None,
        };
        let matchValue = _decode_AssertionValue(_components.get("matchValue").unwrap())?;
        let dnAttributes: OPTIONAL<BOOLEAN> = match _components.get("dnAttributes") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        Ok(MatchingRuleAssertion {
            matchingRule,
            type_,
            matchValue,
            dnAttributes,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_MatchingRuleAssertion(value_: &MatchingRuleAssertion) -> ASN1Result<X690Element> {
    |value_: &MatchingRuleAssertion| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        if let Some(v_) = &value_.matchingRule {
            components_.push(|v_1: &MatchingRuleId| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_MatchingRuleId(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v_)?);
        }
        if let Some(v_) = &value_.type_ {
            components_.push(|v_1: &AttributeDescription| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeDescription(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v_)?);
        }
        components_.push(|v_1: &AssertionValue| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_AssertionValue(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 3;
            Ok(el_1)
        }(&value_.matchValue)?);
        if let Some(v_) = &value_.dnAttributes {
            if *v_ != MatchingRuleAssertion::_default_value_for_dnAttributes() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_boolean(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 4;
                    Ok(el_1)
                }(&v_)?);
            }
        }
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
/// SearchResultEntry ::= [APPLICATION 4]  SEQUENCE {
///   objectName  LDAPDN,
///   attributes  PartialAttributeList
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SearchResultEntry {
    pub objectName: LDAPDN,
    pub attributes: PartialAttributeList,
    pub _unrecognized: Vec<X690Element>,
}
impl SearchResultEntry {}
impl TryFrom<X690Element> for SearchResultEntry {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SearchResultEntry(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SearchResultEntry {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SearchResultEntry(el)
    }
}

pub const _rctl1_components_for_SearchResultEntry: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "objectName",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributes",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SearchResultEntry: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SearchResultEntry: &[ComponentSpec; 0] = &[];

pub fn _decode_SearchResultEntry(el: &X690Element) -> ASN1Result<SearchResultEntry> {
    |el_: &X690Element| -> ASN1Result<SearchResultEntry> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SearchResultEntry,
            _eal_components_for_SearchResultEntry,
            _rctl2_components_for_SearchResultEntry,
        )?;
        let objectName = _decode_LDAPDN(_components.get("objectName").unwrap())?;
        let attributes = _decode_PartialAttributeList(_components.get("attributes").unwrap())?;
        Ok(SearchResultEntry {
            objectName,
            attributes,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SearchResultEntry(value_: &SearchResultEntry) -> ASN1Result<X690Element> {
    |v_1: &SearchResultEntry| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &SearchResultEntry| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(12);
            components_.push(_encode_LDAPDN(&value_.objectName)?);
            components_.push(_encode_PartialAttributeList(&value_.attributes)?);
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 4;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PartialAttributeList  ::=  SEQUENCE OF partialAttribute PartialAttribute
/// ```
pub type PartialAttributeList = Vec<PartialAttribute>; // SequenceOfType

pub fn _decode_PartialAttributeList(el: &X690Element) -> ASN1Result<PartialAttributeList> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<PartialAttribute>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<PartialAttribute> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_PartialAttribute(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_PartialAttributeList(value_: &PartialAttributeList) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<PartialAttribute>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_PartialAttribute(&v)?);
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
/// SearchResultReference  ::=  [APPLICATION 19]  SEQUENCE SIZE (1..MAX) OF uri URI
/// ```
pub type SearchResultReference = Vec<URI>; // SequenceOfType

pub fn _decode_SearchResultReference(el: &X690Element) -> ASN1Result<SearchResultReference> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<URI>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<URI> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_URI(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_SearchResultReference(value_: &SearchResultReference) -> ASN1Result<X690Element> {
    |v_1: &SearchResultReference| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &SEQUENCE_OF<URI>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_URI(&v)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                Arc::new(X690Encoding::Constructed(children)),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 19;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchResultDone  ::=  [APPLICATION 5]  LDAPResult
/// ```
pub type SearchResultDone = LDAPResult; // DefinedType

pub fn _decode_SearchResultDone(el: &X690Element) -> ASN1Result<SearchResultDone> {
    _decode_LDAPResult(&el)
}

pub fn _encode_SearchResultDone(value_: &SearchResultDone) -> ASN1Result<X690Element> {
    |v_1: &SearchResultDone| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_LDAPResult(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 5;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyRequest ::= [APPLICATION 6]  SEQUENCE {
///   object   LDAPDN,
///   changes
///     SEQUENCE OF change
///       SEQUENCE {operation     ENUMERATED {add(0), delete(1), replace(2), ...
///                                           },
///                 modification  PartialAttribute}
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyRequest {
    pub object: LDAPDN,
    pub changes: Vec<ModifyRequest_changes_change>,
    pub _unrecognized: Vec<X690Element>,
}
impl ModifyRequest {}
impl TryFrom<X690Element> for ModifyRequest {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyRequest(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyRequest {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyRequest(el)
    }
}

pub const _rctl1_components_for_ModifyRequest: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "object",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "changes",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ModifyRequest: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ModifyRequest: &[ComponentSpec; 0] = &[];

pub fn _decode_ModifyRequest(el: &X690Element) -> ASN1Result<ModifyRequest> {
    |el_: &X690Element| -> ASN1Result<ModifyRequest> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ModifyRequest,
            _eal_components_for_ModifyRequest,
            _rctl2_components_for_ModifyRequest,
        )?;
        let object = _decode_LDAPDN(_components.get("object").unwrap())?;
        let changes = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ModifyRequest_changes_change>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<ModifyRequest_changes_change> =
                Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_ModifyRequest_changes_change(el)?);
            }
            Ok(items)
        }(_components.get("changes").unwrap())?;
        Ok(ModifyRequest {
            object,
            changes,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ModifyRequest(value_: &ModifyRequest) -> ASN1Result<X690Element> {
    |v_1: &ModifyRequest| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &ModifyRequest| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(12);
            components_.push(_encode_LDAPDN(&value_.object)?);
            components_.push(
                |value_: &SEQUENCE_OF<ModifyRequest_changes_change>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_ModifyRequest_changes_change(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&value_.changes)?,
            );
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 6;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyResponse  ::=  [APPLICATION 7]  LDAPResult
/// ```
pub type ModifyResponse = LDAPResult; // DefinedType

pub fn _decode_ModifyResponse(el: &X690Element) -> ASN1Result<ModifyResponse> {
    _decode_LDAPResult(&el)
}

pub fn _encode_ModifyResponse(value_: &ModifyResponse) -> ASN1Result<X690Element> {
    |v_1: &ModifyResponse| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_LDAPResult(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 7;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AddRequest ::= [APPLICATION 8]  SEQUENCE {
///   entry       LDAPDN,
///   attributes  AttributeList
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AddRequest {
    pub entry: LDAPDN,
    pub attributes: AttributeList,
    pub _unrecognized: Vec<X690Element>,
}
impl AddRequest {}
impl TryFrom<X690Element> for AddRequest {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AddRequest(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AddRequest {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AddRequest(el)
    }
}

pub const _rctl1_components_for_AddRequest: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "entry",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributes",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AddRequest: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AddRequest: &[ComponentSpec; 0] = &[];

pub fn _decode_AddRequest(el: &X690Element) -> ASN1Result<AddRequest> {
    |el_: &X690Element| -> ASN1Result<AddRequest> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AddRequest,
            _eal_components_for_AddRequest,
            _rctl2_components_for_AddRequest,
        )?;
        let entry = _decode_LDAPDN(_components.get("entry").unwrap())?;
        let attributes = _decode_AttributeList(_components.get("attributes").unwrap())?;
        Ok(AddRequest {
            entry,
            attributes,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AddRequest(value_: &AddRequest) -> ASN1Result<X690Element> {
    |v_1: &AddRequest| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &AddRequest| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(12);
            components_.push(_encode_LDAPDN(&value_.entry)?);
            components_.push(_encode_AttributeList(&value_.attributes)?);
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 8;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeList  ::=  SEQUENCE OF attribute Attribute
/// ```
pub type AttributeList = Vec<Attribute>; // SequenceOfType

pub fn _decode_AttributeList(el: &X690Element) -> ASN1Result<AttributeList> {
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

pub fn _encode_AttributeList(value_: &AttributeList) -> ASN1Result<X690Element> {
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
/// AddResponse  ::=  [APPLICATION 9]  LDAPResult
/// ```
pub type AddResponse = LDAPResult; // DefinedType

pub fn _decode_AddResponse(el: &X690Element) -> ASN1Result<AddResponse> {
    _decode_LDAPResult(&el)
}

pub fn _encode_AddResponse(value_: &AddResponse) -> ASN1Result<X690Element> {
    |v_1: &AddResponse| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_LDAPResult(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 9;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DelRequest  ::=  [APPLICATION 10]  LDAPDN
/// ```
pub type DelRequest = LDAPDN; // DefinedType

pub fn _decode_DelRequest(el: &X690Element) -> ASN1Result<DelRequest> {
    _decode_LDAPDN(&el)
}

pub fn _encode_DelRequest(value_: &DelRequest) -> ASN1Result<X690Element> {
    |v_1: &DelRequest| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_LDAPDN(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 10;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DelResponse  ::=  [APPLICATION 11]  LDAPResult
/// ```
pub type DelResponse = LDAPResult; // DefinedType

pub fn _decode_DelResponse(el: &X690Element) -> ASN1Result<DelResponse> {
    _decode_LDAPResult(&el)
}

pub fn _encode_DelResponse(value_: &DelResponse) -> ASN1Result<X690Element> {
    |v_1: &DelResponse| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_LDAPResult(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 11;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyDNRequest ::= [APPLICATION 12]  SEQUENCE {
///   entry         LDAPDN,
///   newrdn        RelativeLDAPDN,
///   deleteoldrdn  BOOLEAN,
///   newSuperior   [0]  LDAPDN OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyDNRequest {
    pub entry: LDAPDN,
    pub newrdn: RelativeLDAPDN,
    pub deleteoldrdn: BOOLEAN,
    pub newSuperior: OPTIONAL<LDAPDN>,
    pub _unrecognized: Vec<X690Element>,
}
impl ModifyDNRequest {}
impl TryFrom<X690Element> for ModifyDNRequest {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyDNRequest(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyDNRequest {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyDNRequest(el)
    }
}

pub const _rctl1_components_for_ModifyDNRequest: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "entry",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "newrdn",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "deleteoldrdn",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "newSuperior",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ModifyDNRequest: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ModifyDNRequest: &[ComponentSpec; 0] = &[];

pub fn _decode_ModifyDNRequest(el: &X690Element) -> ASN1Result<ModifyDNRequest> {
    |el_: &X690Element| -> ASN1Result<ModifyDNRequest> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ModifyDNRequest,
            _eal_components_for_ModifyDNRequest,
            _rctl2_components_for_ModifyDNRequest,
        )?;
        let entry = _decode_LDAPDN(_components.get("entry").unwrap())?;
        let newrdn = _decode_RelativeLDAPDN(_components.get("newrdn").unwrap())?;
        let deleteoldrdn = ber_decode_boolean(_components.get("deleteoldrdn").unwrap())?;
        let newSuperior: OPTIONAL<LDAPDN> = match _components.get("newSuperior") {
            Some(c_) => Some(_decode_LDAPDN(c_)?),
            _ => None,
        };
        Ok(ModifyDNRequest {
            entry,
            newrdn,
            deleteoldrdn,
            newSuperior,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ModifyDNRequest(value_: &ModifyDNRequest) -> ASN1Result<X690Element> {
    |v_1: &ModifyDNRequest| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &ModifyDNRequest| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(14);
            components_.push(_encode_LDAPDN(&value_.entry)?);
            components_.push(_encode_RelativeLDAPDN(&value_.newrdn)?);
            components_.push(ber_encode_boolean(&value_.deleteoldrdn)?);
            if let Some(v_) = &value_.newSuperior {
                components_.push(|v_1: &LDAPDN| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_LDAPDN(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 12;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyDNResponse  ::=  [APPLICATION 13]  LDAPResult
/// ```
pub type ModifyDNResponse = LDAPResult; // DefinedType

pub fn _decode_ModifyDNResponse(el: &X690Element) -> ASN1Result<ModifyDNResponse> {
    _decode_LDAPResult(&el)
}

pub fn _encode_ModifyDNResponse(value_: &ModifyDNResponse) -> ASN1Result<X690Element> {
    |v_1: &ModifyDNResponse| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_LDAPResult(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 13;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CompareRequest ::= [APPLICATION 14]  SEQUENCE {
///   entry  LDAPDN,
///   ava    AttributeValueAssertion
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct CompareRequest {
    pub entry: LDAPDN,
    pub ava: AttributeValueAssertion,
    pub _unrecognized: Vec<X690Element>,
}
impl CompareRequest {}
impl TryFrom<X690Element> for CompareRequest {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CompareRequest(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CompareRequest {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CompareRequest(el)
    }
}

pub const _rctl1_components_for_CompareRequest: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "entry",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "ava",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_CompareRequest: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_CompareRequest: &[ComponentSpec; 0] = &[];

pub fn _decode_CompareRequest(el: &X690Element) -> ASN1Result<CompareRequest> {
    |el_: &X690Element| -> ASN1Result<CompareRequest> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_CompareRequest,
            _eal_components_for_CompareRequest,
            _rctl2_components_for_CompareRequest,
        )?;
        let entry = _decode_LDAPDN(_components.get("entry").unwrap())?;
        let ava = _decode_AttributeValueAssertion(_components.get("ava").unwrap())?;
        Ok(CompareRequest {
            entry,
            ava,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_CompareRequest(value_: &CompareRequest) -> ASN1Result<X690Element> {
    |v_1: &CompareRequest| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &CompareRequest| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(12);
            components_.push(_encode_LDAPDN(&value_.entry)?);
            components_.push(_encode_AttributeValueAssertion(&value_.ava)?);
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 14;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CompareResponse  ::=  [APPLICATION 15]  LDAPResult
/// ```
pub type CompareResponse = LDAPResult; // DefinedType

pub fn _decode_CompareResponse(el: &X690Element) -> ASN1Result<CompareResponse> {
    _decode_LDAPResult(&el)
}

pub fn _encode_CompareResponse(value_: &CompareResponse) -> ASN1Result<X690Element> {
    |v_1: &CompareResponse| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_LDAPResult(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 15;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AbandonRequest  ::=  [APPLICATION 16]  MessageID
/// ```
pub type AbandonRequest = MessageID; // DefinedType

pub fn _decode_AbandonRequest(el: &X690Element) -> ASN1Result<AbandonRequest> {
    _decode_MessageID(&el)
}

pub fn _encode_AbandonRequest(value_: &AbandonRequest) -> ASN1Result<X690Element> {
    |v_1: &AbandonRequest| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_MessageID(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 16;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendedRequest ::= [APPLICATION 23]  SEQUENCE {
///   requestName   [0]  LDAPOID,
///   requestValue  [1]  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ExtendedRequest {
    pub requestName: LDAPOID,
    pub requestValue: OPTIONAL<OCTET_STRING>,
    pub _unrecognized: Vec<X690Element>,
}
impl ExtendedRequest {}
impl TryFrom<X690Element> for ExtendedRequest {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedRequest(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ExtendedRequest {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedRequest(el)
    }
}

pub const _rctl1_components_for_ExtendedRequest: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "requestName",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "requestValue",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ExtendedRequest: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ExtendedRequest: &[ComponentSpec; 0] = &[];

pub fn _decode_ExtendedRequest(el: &X690Element) -> ASN1Result<ExtendedRequest> {
    |el_: &X690Element| -> ASN1Result<ExtendedRequest> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ExtendedRequest,
            _eal_components_for_ExtendedRequest,
            _rctl2_components_for_ExtendedRequest,
        )?;
        let requestName = _decode_LDAPOID(_components.get("requestName").unwrap())?;
        let requestValue: OPTIONAL<OCTET_STRING> = match _components.get("requestValue") {
            Some(c_) => Some(ber_decode_octet_string(c_)?),
            _ => None,
        };
        Ok(ExtendedRequest {
            requestName,
            requestValue,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ExtendedRequest(value_: &ExtendedRequest) -> ASN1Result<X690Element> {
    |v_1: &ExtendedRequest| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &ExtendedRequest| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(12);
            components_.push(|v_1: &LDAPOID| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_LDAPOID(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&value_.requestName)?);
            if let Some(v_) = &value_.requestValue {
                components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_octet_string(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 23;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendedResponse ::= [APPLICATION 24]  SEQUENCE {
///   COMPONENTS OF LDAPResult,
///   responseName   [10]  LDAPOID OPTIONAL,
///   responseValue  [11]  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ExtendedResponse {
    pub resultCode: LDAPResult_resultCode, /* REPLICATED_COMPONENT */
    pub matchedDN: LDAPDN,                 /* REPLICATED_COMPONENT */
    pub diagnosticMessage: LDAPString,     /* REPLICATED_COMPONENT */
    pub referral: OPTIONAL<Referral>,      /* REPLICATED_COMPONENT */
    pub responseName: OPTIONAL<LDAPOID>,
    pub responseValue: OPTIONAL<OCTET_STRING>,
    pub _unrecognized: Vec<X690Element>,
}
impl ExtendedResponse {}
impl TryFrom<X690Element> for ExtendedResponse {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedResponse(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ExtendedResponse {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedResponse(el)
    }
}

pub const _rctl1_components_for_ExtendedResponse: &[ComponentSpec; 6] = &[
    ComponentSpec::new(
        "resultCode",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "matchedDN",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "diagnosticMessage",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "referral",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responseName",
        true,
        TagSelector::tag((TagClass::CONTEXT, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responseValue",
        true,
        TagSelector::tag((TagClass::CONTEXT, 11)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ExtendedResponse: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ExtendedResponse: &[ComponentSpec; 0] = &[];

pub fn _decode_ExtendedResponse(el: &X690Element) -> ASN1Result<ExtendedResponse> {
    |el_: &X690Element| -> ASN1Result<ExtendedResponse> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ExtendedResponse,
            _eal_components_for_ExtendedResponse,
            _rctl2_components_for_ExtendedResponse,
        )?;
        let resultCode = _decode_LDAPResult_resultCode(_components.get("resultCode").unwrap())?;
        let matchedDN = _decode_LDAPDN(_components.get("matchedDN").unwrap())?;
        let diagnosticMessage = _decode_LDAPString(_components.get("diagnosticMessage").unwrap())?;
        let referral: OPTIONAL<Referral> = match _components.get("referral") {
            Some(c_) => Some(_decode_Referral(c_)?),
            _ => None,
        };
        let responseName: OPTIONAL<LDAPOID> = match _components.get("responseName") {
            Some(c_) => Some(_decode_LDAPOID(c_)?),
            _ => None,
        };
        let responseValue: OPTIONAL<OCTET_STRING> = match _components.get("responseValue") {
            Some(c_) => Some(ber_decode_octet_string(c_)?),
            _ => None,
        };
        Ok(ExtendedResponse {
            resultCode,
            matchedDN,
            diagnosticMessage,
            referral,
            responseName,
            responseValue,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ExtendedResponse(value_: &ExtendedResponse) -> ASN1Result<X690Element> {
    |v_1: &ExtendedResponse| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &ExtendedResponse| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(16);
            components_.push(_encode_LDAPResult_resultCode(&value_.resultCode)?);
            components_.push(_encode_LDAPDN(&value_.matchedDN)?);
            components_.push(_encode_LDAPString(&value_.diagnosticMessage)?);
            if let Some(v_) = &value_.referral {
                components_.push(|v_1: &Referral| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_Referral(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 3;
                    Ok(el_1)
                }(&v_)?);
            }
            if let Some(v_) = &value_.responseName {
                components_.push(|v_1: &LDAPOID| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_LDAPOID(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 10;
                    Ok(el_1)
                }(&v_)?);
            }
            if let Some(v_) = &value_.responseValue {
                components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_octet_string(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 11;
                    Ok(el_1)
                }(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 24;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IntermediateResponse ::= [APPLICATION 25]  SEQUENCE {
///   responseName   [0]  LDAPOID OPTIONAL,
///   responseValue  [1]  OCTET STRING OPTIONAL
/// }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct IntermediateResponse {
    pub responseName: OPTIONAL<LDAPOID>,
    pub responseValue: OPTIONAL<OCTET_STRING>,
    pub _unrecognized: Vec<X690Element>,
}
impl IntermediateResponse {}
impl TryFrom<X690Element> for IntermediateResponse {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IntermediateResponse(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IntermediateResponse {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_IntermediateResponse(el)
    }
}

pub const _rctl1_components_for_IntermediateResponse: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "responseName",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responseValue",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IntermediateResponse: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IntermediateResponse: &[ComponentSpec; 0] = &[];

pub fn _decode_IntermediateResponse(el: &X690Element) -> ASN1Result<IntermediateResponse> {
    |el_: &X690Element| -> ASN1Result<IntermediateResponse> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IntermediateResponse,
            _eal_components_for_IntermediateResponse,
            _rctl2_components_for_IntermediateResponse,
        )?;
        let responseName: OPTIONAL<LDAPOID> = match _components.get("responseName") {
            Some(c_) => Some(_decode_LDAPOID(c_)?),
            _ => None,
        };
        let responseValue: OPTIONAL<OCTET_STRING> = match _components.get("responseValue") {
            Some(c_) => Some(ber_decode_octet_string(c_)?),
            _ => None,
        };
        Ok(IntermediateResponse {
            responseName,
            responseValue,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_IntermediateResponse(value_: &IntermediateResponse) -> ASN1Result<X690Element> {
    |v_1: &IntermediateResponse| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &IntermediateResponse| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(12);
            if let Some(v_) = &value_.responseName {
                components_.push(|v_1: &LDAPOID| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_LDAPOID(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v_)?);
            }
            if let Some(v_) = &value_.responseValue {
                components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_octet_string(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v_)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SET,
                Arc::new(X690Encoding::Constructed(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag_class = TagClass::APPLICATION;
        el_1.tag_number = 25;
        Ok(el_1)
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPMessage-protocolOp ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum LDAPMessage_protocolOp {
    bindRequest(BindRequest),
    bindResponse(BindResponse),
    unbindRequest(UnbindRequest),
    searchRequest(SearchRequest),
    searchResEntry(SearchResultEntry),
    searchResDone(SearchResultDone),
    searchResRef(SearchResultReference),
    modifyRequest(ModifyRequest),
    modifyResponse(ModifyResponse),
    addRequest(AddRequest),
    addResponse(AddResponse),
    delRequest(DelRequest),
    delResponse(DelResponse),
    modDNRequest(ModifyDNRequest),
    modDNResponse(ModifyDNResponse),
    compareRequest(CompareRequest),
    compareResponse(CompareResponse),
    abandonRequest(AbandonRequest),
    extendedReq(ExtendedRequest),
    extendedResp(ExtendedResponse),
    intermediateResponse(IntermediateResponse), /* CHOICE_ALT_EXT */
    _unrecognized(X690Element),                 /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for LDAPMessage_protocolOp {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_LDAPMessage_protocolOp(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for LDAPMessage_protocolOp {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_LDAPMessage_protocolOp(el)
    }
}

pub fn _decode_LDAPMessage_protocolOp(el: &X690Element) -> ASN1Result<LDAPMessage_protocolOp> {
    |el: &X690Element| -> ASN1Result<LDAPMessage_protocolOp> {
        match (el.tag_class, el.tag_number) {
            (TagClass::APPLICATION, 0) => Ok(LDAPMessage_protocolOp::bindRequest(
                _decode_BindRequest(&el)?,
            )),
            (TagClass::APPLICATION, 1) => Ok(LDAPMessage_protocolOp::bindResponse(
                _decode_BindResponse(&el)?,
            )),
            (TagClass::APPLICATION, 2) => Ok(LDAPMessage_protocolOp::unbindRequest(
                _decode_UnbindRequest(&el)?,
            )),
            (TagClass::APPLICATION, 3) => Ok(LDAPMessage_protocolOp::searchRequest(
                _decode_SearchRequest(&el)?,
            )),
            (TagClass::APPLICATION, 4) => Ok(LDAPMessage_protocolOp::searchResEntry(
                _decode_SearchResultEntry(&el)?,
            )),
            (TagClass::APPLICATION, 5) => Ok(LDAPMessage_protocolOp::searchResDone(
                _decode_SearchResultDone(&el)?,
            )),
            (TagClass::APPLICATION, 19) => Ok(LDAPMessage_protocolOp::searchResRef(
                _decode_SearchResultReference(&el)?,
            )),
            (TagClass::APPLICATION, 6) => Ok(LDAPMessage_protocolOp::modifyRequest(
                _decode_ModifyRequest(&el)?,
            )),
            (TagClass::APPLICATION, 7) => Ok(LDAPMessage_protocolOp::modifyResponse(
                _decode_ModifyResponse(&el)?,
            )),
            (TagClass::APPLICATION, 8) => {
                Ok(LDAPMessage_protocolOp::addRequest(_decode_AddRequest(&el)?))
            }
            (TagClass::APPLICATION, 9) => Ok(LDAPMessage_protocolOp::addResponse(
                _decode_AddResponse(&el)?,
            )),
            (TagClass::APPLICATION, 10) => {
                Ok(LDAPMessage_protocolOp::delRequest(_decode_DelRequest(&el)?))
            }
            (TagClass::APPLICATION, 11) => Ok(LDAPMessage_protocolOp::delResponse(
                _decode_DelResponse(&el)?,
            )),
            (TagClass::APPLICATION, 12) => Ok(LDAPMessage_protocolOp::modDNRequest(
                _decode_ModifyDNRequest(&el)?,
            )),
            (TagClass::APPLICATION, 13) => Ok(LDAPMessage_protocolOp::modDNResponse(
                _decode_ModifyDNResponse(&el)?,
            )),
            (TagClass::APPLICATION, 14) => Ok(LDAPMessage_protocolOp::compareRequest(
                _decode_CompareRequest(&el)?,
            )),
            (TagClass::APPLICATION, 15) => Ok(LDAPMessage_protocolOp::compareResponse(
                _decode_CompareResponse(&el)?,
            )),
            (TagClass::APPLICATION, 16) => Ok(LDAPMessage_protocolOp::abandonRequest(
                _decode_AbandonRequest(&el)?,
            )),
            (TagClass::APPLICATION, 23) => Ok(LDAPMessage_protocolOp::extendedReq(
                _decode_ExtendedRequest(&el)?,
            )),
            (TagClass::APPLICATION, 24) => Ok(LDAPMessage_protocolOp::extendedResp(
                _decode_ExtendedResponse(&el)?,
            )),
            (TagClass::APPLICATION, 25) => Ok(LDAPMessage_protocolOp::intermediateResponse(
                _decode_IntermediateResponse(&el)?,
            )),
            _ => Ok(LDAPMessage_protocolOp::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_LDAPMessage_protocolOp(value_: &LDAPMessage_protocolOp) -> ASN1Result<X690Element> {
    |value: &LDAPMessage_protocolOp| -> ASN1Result<X690Element> {
        match value {
            LDAPMessage_protocolOp::bindRequest(v) => _encode_BindRequest(&v),
            LDAPMessage_protocolOp::bindResponse(v) => _encode_BindResponse(&v),
            LDAPMessage_protocolOp::unbindRequest(v) => _encode_UnbindRequest(&v),
            LDAPMessage_protocolOp::searchRequest(v) => _encode_SearchRequest(&v),
            LDAPMessage_protocolOp::searchResEntry(v) => _encode_SearchResultEntry(&v),
            LDAPMessage_protocolOp::searchResDone(v) => _encode_SearchResultDone(&v),
            LDAPMessage_protocolOp::searchResRef(v) => _encode_SearchResultReference(&v),
            LDAPMessage_protocolOp::modifyRequest(v) => _encode_ModifyRequest(&v),
            LDAPMessage_protocolOp::modifyResponse(v) => _encode_ModifyResponse(&v),
            LDAPMessage_protocolOp::addRequest(v) => _encode_AddRequest(&v),
            LDAPMessage_protocolOp::addResponse(v) => _encode_AddResponse(&v),
            LDAPMessage_protocolOp::delRequest(v) => _encode_DelRequest(&v),
            LDAPMessage_protocolOp::delResponse(v) => _encode_DelResponse(&v),
            LDAPMessage_protocolOp::modDNRequest(v) => _encode_ModifyDNRequest(&v),
            LDAPMessage_protocolOp::modDNResponse(v) => _encode_ModifyDNResponse(&v),
            LDAPMessage_protocolOp::compareRequest(v) => _encode_CompareRequest(&v),
            LDAPMessage_protocolOp::compareResponse(v) => _encode_CompareResponse(&v),
            LDAPMessage_protocolOp::abandonRequest(v) => _encode_AbandonRequest(&v),
            LDAPMessage_protocolOp::extendedReq(v) => _encode_ExtendedRequest(&v),
            LDAPMessage_protocolOp::extendedResp(v) => _encode_ExtendedResponse(&v),
            LDAPMessage_protocolOp::intermediateResponse(v) => _encode_IntermediateResponse(&v),
            LDAPMessage_protocolOp::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LDAPResult-resultCode ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type LDAPResult_resultCode = ENUMERATED;

pub const LDAPResult_resultCode_success: LDAPResult_resultCode = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_operationsError: LDAPResult_resultCode = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_protocolError: LDAPResult_resultCode = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_timeLimitExceeded: LDAPResult_resultCode = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_sizeLimitExceeded: LDAPResult_resultCode = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_compareFalse: LDAPResult_resultCode = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_compareTrue: LDAPResult_resultCode = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_authMethodNotSupported: LDAPResult_resultCode = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_strongerAuthRequired: LDAPResult_resultCode = 8; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_referral: LDAPResult_resultCode = 10; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_adminLimitExceeded: LDAPResult_resultCode = 11; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_unavailableCriticalExtension: LDAPResult_resultCode = 12; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_confidentialityRequired: LDAPResult_resultCode = 13; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_saslBindInProgress: LDAPResult_resultCode = 14; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_noSuchAttribute: LDAPResult_resultCode = 16; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_undefinedAttributeType: LDAPResult_resultCode = 17; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_inappropriateMatching: LDAPResult_resultCode = 18; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_constraintViolation: LDAPResult_resultCode = 19; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_attributeOrValueExists: LDAPResult_resultCode = 20; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_invalidAttributeSyntax: LDAPResult_resultCode = 21; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_noSuchObject: LDAPResult_resultCode = 32; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_aliasProblem: LDAPResult_resultCode = 33; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_invalidDNSyntax: LDAPResult_resultCode = 34; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_aliasDereferencingProblem: LDAPResult_resultCode = 36; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_inappropriateAuthentication: LDAPResult_resultCode = 48; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_invalidCredentials: LDAPResult_resultCode = 49; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_insufficientAccessRights: LDAPResult_resultCode = 50; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_busy: LDAPResult_resultCode = 51; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_unavailable: LDAPResult_resultCode = 52; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_unwillingToPerform: LDAPResult_resultCode = 53; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_loopDetect: LDAPResult_resultCode = 54; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_namingViolation: LDAPResult_resultCode = 64; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_objectClassViolation: LDAPResult_resultCode = 65; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_notAllowedOnNonLeaf: LDAPResult_resultCode = 66; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_notAllowedOnRDN: LDAPResult_resultCode = 67; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_entryAlreadyExists: LDAPResult_resultCode = 68; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_objectClassModsProhibited: LDAPResult_resultCode = 69; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_affectsMultipleDSAs: LDAPResult_resultCode = 71; /* LONG_NAMED_ENUMERATED_VALUE */

pub const LDAPResult_resultCode_other: LDAPResult_resultCode = 80; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_LDAPResult_resultCode(el: &X690Element) -> ASN1Result<LDAPResult_resultCode> {
    ber_decode_enumerated(&el)
}

pub fn _encode_LDAPResult_resultCode(value_: &LDAPResult_resultCode) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchRequest-scope ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type SearchRequest_scope = ENUMERATED;

pub const SearchRequest_scope_baseObject: SearchRequest_scope = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchRequest_scope_singleLevel: SearchRequest_scope = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchRequest_scope_wholeSubtree: SearchRequest_scope = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_SearchRequest_scope(el: &X690Element) -> ASN1Result<SearchRequest_scope> {
    ber_decode_enumerated(&el)
}

pub fn _encode_SearchRequest_scope(value_: &SearchRequest_scope) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchRequest-derefAliases ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type SearchRequest_derefAliases = ENUMERATED;

pub const SearchRequest_derefAliases_neverDerefAliases: SearchRequest_derefAliases = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchRequest_derefAliases_derefInSearching: SearchRequest_derefAliases = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchRequest_derefAliases_derefFindingBaseObj: SearchRequest_derefAliases = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SearchRequest_derefAliases_derefAlways: SearchRequest_derefAliases = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_SearchRequest_derefAliases(
    el: &X690Element,
) -> ASN1Result<SearchRequest_derefAliases> {
    ber_decode_enumerated(&el)
}

pub fn _encode_SearchRequest_derefAliases(
    value_: &SearchRequest_derefAliases,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubstringFilter-substrings-substring ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum SubstringFilter_substrings_substring {
    initial(AssertionValue),
    any(AssertionValue),
    final_(AssertionValue),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for SubstringFilter_substrings_substring {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SubstringFilter_substrings_substring(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SubstringFilter_substrings_substring {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SubstringFilter_substrings_substring(el)
    }
}

pub fn _decode_SubstringFilter_substrings_substring(
    el: &X690Element,
) -> ASN1Result<SubstringFilter_substrings_substring> {
    |el: &X690Element| -> ASN1Result<SubstringFilter_substrings_substring> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(SubstringFilter_substrings_substring::initial(
                _decode_AssertionValue(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(SubstringFilter_substrings_substring::any(
                _decode_AssertionValue(&el)?,
            )),
            (TagClass::CONTEXT, 2) => Ok(SubstringFilter_substrings_substring::final_(
                _decode_AssertionValue(&el)?,
            )),
            _ => Ok(SubstringFilter_substrings_substring::_unrecognized(
                el.clone(),
            )),
        }
    }(&el)
}

pub fn _encode_SubstringFilter_substrings_substring(
    value_: &SubstringFilter_substrings_substring,
) -> ASN1Result<X690Element> {
    |value: &SubstringFilter_substrings_substring| -> ASN1Result<X690Element> {
        match value {
            SubstringFilter_substrings_substring::initial(v) => {
                |v_1: &AssertionValue| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AssertionValue(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            SubstringFilter_substrings_substring::any(v) => {
                |v_1: &AssertionValue| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AssertionValue(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            SubstringFilter_substrings_substring::final_(v) => {
                |v_1: &AssertionValue| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AssertionValue(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v)
            }
            SubstringFilter_substrings_substring::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyRequest-changes-change-operation ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type ModifyRequest_changes_change_operation = ENUMERATED;

pub const ModifyRequest_changes_change_operation_add: ModifyRequest_changes_change_operation = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ModifyRequest_changes_change_operation_delete: ModifyRequest_changes_change_operation = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ModifyRequest_changes_change_operation_replace: ModifyRequest_changes_change_operation =
    2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_ModifyRequest_changes_change_operation(
    el: &X690Element,
) -> ASN1Result<ModifyRequest_changes_change_operation> {
    ber_decode_enumerated(&el)
}

pub fn _encode_ModifyRequest_changes_change_operation(
    value_: &ModifyRequest_changes_change_operation,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ModifyRequest-changes-change ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ModifyRequest_changes_change {
    pub operation: ModifyRequest_changes_change_operation,
    pub modification: PartialAttribute,
    pub _unrecognized: Vec<X690Element>,
}
impl ModifyRequest_changes_change {}
impl TryFrom<X690Element> for ModifyRequest_changes_change {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyRequest_changes_change(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ModifyRequest_changes_change {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ModifyRequest_changes_change(el)
    }
}

pub const _rctl1_components_for_ModifyRequest_changes_change: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "operation",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "modification",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ModifyRequest_changes_change: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ModifyRequest_changes_change: &[ComponentSpec; 0] = &[];

pub fn _decode_ModifyRequest_changes_change(
    el: &X690Element,
) -> ASN1Result<ModifyRequest_changes_change> {
    |el_: &X690Element| -> ASN1Result<ModifyRequest_changes_change> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ModifyRequest_changes_change,
            _eal_components_for_ModifyRequest_changes_change,
            _rctl2_components_for_ModifyRequest_changes_change,
        )?;
        let operation =
            _decode_ModifyRequest_changes_change_operation(_components.get("operation").unwrap())?;
        let modification = _decode_PartialAttribute(_components.get("modification").unwrap())?;
        Ok(ModifyRequest_changes_change {
            operation,
            modification,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ModifyRequest_changes_change(
    value_: &ModifyRequest_changes_change,
) -> ASN1Result<X690Element> {
    |value_: &ModifyRequest_changes_change| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_ModifyRequest_changes_change_operation(
            &value_.operation,
        )?);
        components_.push(_encode_PartialAttribute(&value_.modification)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}
