#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # OSIProtocolSpecification
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `OSIProtocolSpecification`.
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
use crate::CommonProtocolSpecification::*;
use crate::InformationFramework::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// OSI-PDU{APPLICATION-CONTEXT:protocol}  ::=  TYPE-IDENTIFIER.&Type (
///   OsiBind{{protocol}} |
///   OsiBindResult{{protocol}} |
///   OsiBindError{{protocol}} |
///   OsiOperation{{protocol.&Operations}} |
///   OsiUnbind |
///   PresentationAbort )
/// ```
pub type OSI_PDU = X690Element; // ObjectClassFieldType

pub fn _decode_OSI_PDU(el: &X690Element) -> ASN1Result<OSI_PDU> {
    x690_identity(&el)
}

pub fn _encode_OSI_PDU(value_: &OSI_PDU) -> ASN1Result<X690Element> {
    x690_identity(&value_)
}

pub fn _validate_OSI_PDU(el: &X690Element) -> ASN1Result<()> {
    BER.validate_any(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBind{APPLICATION-CONTEXT:Protocols} ::= SET {
///   mode-selector                  [0]  IMPLICIT SET {
///     mode-value                     [0]  IMPLICIT INTEGER(1)},
///   normal-mode-parameters         [2]  IMPLICIT SEQUENCE {
///     protocol-version               [0]  IMPLICIT BIT STRING {version-1(0)}
///                                           DEFAULT {version-1},
///     calling-presentation-selector  [1]  IMPLICIT Presentation-selector OPTIONAL,
///     called-presentation-selector   [2]  IMPLICIT Presentation-selector OPTIONAL,
///     presentation-context-definition-list
///                                    [4]  IMPLICIT Context-list,
///     user-data                           CHOICE {
///       fully-encoded-data  [APPLICATION 1] IMPLICIT SEQUENCE SIZE (1) OF SEQUENCE {
///         transfer-syntax-name              Transfer-syntax-name OPTIONAL,
///         presentation-context-identifier   Presentation-context-identifier,
///         presentation-data-values          CHOICE {
///           single-ASN1-type             [0]  ABSTRACT-SYNTAX.&Type
///                                              (AARQ-apdu{{Protocols}})}}}}}
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiBind {
    pub mode_selector: OsiBind_mode_selector,
    pub normal_mode_parameters: OsiBind_normal_mode_parameters,
}
impl OsiBind {
    pub fn new(
        mode_selector: OsiBind_mode_selector,
        normal_mode_parameters: OsiBind_normal_mode_parameters,
    ) -> Self {
        OsiBind {
            mode_selector,
            normal_mode_parameters,
        }
    }
}
impl TryFrom<&X690Element> for OsiBind {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBind(el)
    }
}

pub const _rctl1_components_for_OsiBind: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "mode-selector",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "normal-mode-parameters",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_OsiBind: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiBind: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiBind(el: &X690Element) -> ASN1Result<OsiBind> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBind")),
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_OsiBind,
        _eal_components_for_OsiBind,
        _rctl2_components_for_OsiBind,
        20,
    )?;
    let mode_selector_ = _decode_OsiBind_mode_selector(_components.get("mode-selector").unwrap())?;
    let normal_mode_parameters_ =
        _decode_OsiBind_normal_mode_parameters(_components.get("normal-mode-parameters").unwrap())?;
    Ok(OsiBind {
        mode_selector: mode_selector_,
        normal_mode_parameters: normal_mode_parameters_,
    })
}

pub fn _encode_OsiBind(value_: &OsiBind) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(|v_1: &OsiBind_mode_selector| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_OsiBind_mode_selector(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_.mode_selector)?);
    components_.push(
        |v_1: &OsiBind_normal_mode_parameters| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_OsiBind_normal_mode_parameters(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&value_.normal_mode_parameters)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiBind(el: &X690Element) -> ASN1Result<()> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBind")),
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_OsiBind,
        _eal_components_for_OsiBind,
        _rctl2_components_for_OsiBind,
        20,
    )?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "mode-selector"));
        }
        Ok(_validate_OsiBind_mode_selector(&el)?)
    }(_components.get("mode-selector").unwrap())?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "normal-mode-parameters",
            ));
        }
        Ok(_validate_OsiBind_normal_mode_parameters(&el)?)
    }(_components.get("normal-mode-parameters").unwrap())?;
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Presentation-selector  ::=  OCTET STRING(SIZE (1..4, ..., 5..MAX))
/// ```
pub type Presentation_selector = OCTET_STRING; // OctetStringType

pub fn _decode_Presentation_selector(el: &X690Element) -> ASN1Result<Presentation_selector> {
    BER.decode_octet_string(&el)
}

pub fn _encode_Presentation_selector(value_: &Presentation_selector) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_Presentation_selector(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Context-list  ::=  SEQUENCE SIZE (2) OF SEQUENCE {
///   presentation-context-identifier  Presentation-context-identifier,
///   abstract-syntax-name             Abstract-syntax-name,
///   transfer-syntax-name-list        SEQUENCE OF Transfer-syntax-name }
/// ```
pub type Context_list = Vec<Context_list_Item>; // SequenceOfType

pub fn _decode_Context_list(el: &X690Element) -> ASN1Result<Context_list> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context-list")),
    };
    let mut items: SEQUENCE_OF<Context_list_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Context_list_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_Context_list(value_: &Context_list) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Context_list_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Context_list(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Context_list_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context-list")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Presentation-context-identifier  ::=  INTEGER(1..127, ..., 128..MAX)
/// ```
pub type Presentation_context_identifier = INTEGER;

pub fn _decode_Presentation_context_identifier(
    el: &X690Element,
) -> ASN1Result<Presentation_context_identifier> {
    BER.decode_integer(&el)
}

pub fn _encode_Presentation_context_identifier(
    value_: &Presentation_context_identifier,
) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_Presentation_context_identifier(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Abstract-syntax-name  ::=  OBJECT IDENTIFIER
/// ```
pub type Abstract_syntax_name = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_Abstract_syntax_name(el: &X690Element) -> ASN1Result<Abstract_syntax_name> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_Abstract_syntax_name(value_: &Abstract_syntax_name) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_Abstract_syntax_name(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Transfer-syntax-name  ::=  OBJECT IDENTIFIER
/// ```
pub type Transfer_syntax_name = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_Transfer_syntax_name(el: &X690Element) -> ASN1Result<Transfer_syntax_name> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_Transfer_syntax_name(value_: &Transfer_syntax_name) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_Transfer_syntax_name(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AARQ-apdu{APPLICATION-CONTEXT:Protocols} ::= [APPLICATION 0] IMPLICIT SEQUENCE {
///   protocol-version                  [0] IMPLICIT BIT STRING {version1(0)}
///                                              DEFAULT {version1},
///   application-context-name          [1]  Application-context-name,
///   called-AP-title                   [2]  Name OPTIONAL,
///   called-AE-qualifier               [3]  RelativeDistinguishedName    OPTIONAL,
///   called-AP-invocation-identifier   [4]  AP-invocation-identifier     OPTIONAL,
///   called-AE-invocation-identifier   [5]  AE-invocation-identifier     OPTIONAL,
///   calling-AP-title                  [6]  Name                         OPTIONAL,
///   calling-AE-qualifier              [7]  RelativeDistinguishedName    OPTIONAL,
///   calling-AP-invocation-identifier  [8]  AP-invocation-identifier     OPTIONAL,
///   calling-AE-invocation-identifier  [9]  AE-invocation-identifier     OPTIONAL,
///   implementation-information        [29] IMPLICIT Implementation-data OPTIONAL,
///   user-information                  [30] IMPLICIT
///                                            Association-informationBind{{Protocols}}}
/// ```
///
#[derive(Debug, Clone)]
pub struct AARQ_apdu {
    pub protocol_version: OPTIONAL<AARQ_apdu_protocol_version>,
    pub application_context_name: Application_context_name,
    pub called_AP_title: OPTIONAL<Name>,
    pub called_AE_qualifier: OPTIONAL<RelativeDistinguishedName>,
    pub called_AP_invocation_identifier: OPTIONAL<AP_invocation_identifier>,
    pub called_AE_invocation_identifier: OPTIONAL<AE_invocation_identifier>,
    pub calling_AP_title: OPTIONAL<Name>,
    pub calling_AE_qualifier: OPTIONAL<RelativeDistinguishedName>,
    pub calling_AP_invocation_identifier: OPTIONAL<AP_invocation_identifier>,
    pub calling_AE_invocation_identifier: OPTIONAL<AE_invocation_identifier>,
    pub implementation_information: OPTIONAL<Implementation_data>,
    pub user_information: Association_informationBind,
}
impl AARQ_apdu {
    pub fn new(
        protocol_version: OPTIONAL<AARQ_apdu_protocol_version>,
        application_context_name: Application_context_name,
        called_AP_title: OPTIONAL<Name>,
        called_AE_qualifier: OPTIONAL<RelativeDistinguishedName>,
        called_AP_invocation_identifier: OPTIONAL<AP_invocation_identifier>,
        called_AE_invocation_identifier: OPTIONAL<AE_invocation_identifier>,
        calling_AP_title: OPTIONAL<Name>,
        calling_AE_qualifier: OPTIONAL<RelativeDistinguishedName>,
        calling_AP_invocation_identifier: OPTIONAL<AP_invocation_identifier>,
        calling_AE_invocation_identifier: OPTIONAL<AE_invocation_identifier>,
        implementation_information: OPTIONAL<Implementation_data>,
        user_information: Association_informationBind,
    ) -> Self {
        AARQ_apdu {
            protocol_version,
            application_context_name,
            called_AP_title,
            called_AE_qualifier,
            called_AP_invocation_identifier,
            called_AE_invocation_identifier,
            calling_AP_title,
            calling_AE_qualifier,
            calling_AP_invocation_identifier,
            calling_AE_invocation_identifier,
            implementation_information,
            user_information,
        }
    }
    pub fn _default_value_for_protocol_version() -> AARQ_apdu_protocol_version {
        BIT_STRING::with_bits_set(&[AARQ_apdu_protocol_version_version1])
    }
}
impl TryFrom<&X690Element> for AARQ_apdu {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AARQ_apdu(el)
    }
}

pub const _rctl1_components_for_AARQ_apdu: &[ComponentSpec; 12] = &[
    ComponentSpec::new(
        "protocol-version",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "application-context-name",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "called-AP-title",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "called-AE-qualifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "called-AP-invocation-identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "called-AE-invocation-identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "calling-AP-title",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "calling-AE-qualifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "calling-AP-invocation-identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
    ComponentSpec::new(
        "calling-AE-invocation-identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 9)),
        None,
        None,
    ),
    ComponentSpec::new(
        "implementation-information",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "user-information",
        false,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AARQ_apdu: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AARQ_apdu: &[ComponentSpec; 0] = &[];

pub fn _decode_AARQ_apdu(el: &X690Element) -> ASN1Result<AARQ_apdu> {
    |el: &X690Element| -> ASN1Result<AARQ_apdu> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AARQ-apdu")),
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_AARQ_apdu,
            _eal_components_for_AARQ_apdu,
            _rctl2_components_for_AARQ_apdu,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut protocol_version_: OPTIONAL<AARQ_apdu_protocol_version> = None;
        let mut application_context_name_: OPTIONAL<Application_context_name> = None;
        let mut called_AP_title_: OPTIONAL<Name> = None;
        let mut called_AE_qualifier_: OPTIONAL<RelativeDistinguishedName> = None;
        let mut called_AP_invocation_identifier_: OPTIONAL<AP_invocation_identifier> = None;
        let mut called_AE_invocation_identifier_: OPTIONAL<AE_invocation_identifier> = None;
        let mut calling_AP_title_: OPTIONAL<Name> = None;
        let mut calling_AE_qualifier_: OPTIONAL<RelativeDistinguishedName> = None;
        let mut calling_AP_invocation_identifier_: OPTIONAL<AP_invocation_identifier> = None;
        let mut calling_AE_invocation_identifier_: OPTIONAL<AE_invocation_identifier> = None;
        let mut implementation_information_: OPTIONAL<Implementation_data> = None;
        let mut user_information_: OPTIONAL<Association_informationBind> = None;
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "protocol-version" => {
                    protocol_version_ = Some(_decode_AARQ_apdu_protocol_version(_el)?)
                }
                "application-context-name" => {
                    application_context_name_ =
                        Some(|el: &X690Element| -> ASN1Result<Application_context_name> {
                            Ok(_decode_Application_context_name(&el.inner()?)?)
                        }(_el)?)
                }
                "called-AP-title" => {
                    called_AP_title_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                        Ok(_decode_Name(&el.inner()?)?)
                    }(_el)?)
                }
                "called-AE-qualifier" => {
                    called_AE_qualifier_ = Some(
                        |el: &X690Element| -> ASN1Result<RelativeDistinguishedName> {
                            Ok(_decode_RelativeDistinguishedName(&el.inner()?)?)
                        }(_el)?,
                    )
                }
                "called-AP-invocation-identifier" => {
                    called_AP_invocation_identifier_ =
                        Some(|el: &X690Element| -> ASN1Result<AP_invocation_identifier> {
                            Ok(_decode_AP_invocation_identifier(&el.inner()?)?)
                        }(_el)?)
                }
                "called-AE-invocation-identifier" => {
                    called_AE_invocation_identifier_ =
                        Some(|el: &X690Element| -> ASN1Result<AE_invocation_identifier> {
                            Ok(_decode_AE_invocation_identifier(&el.inner()?)?)
                        }(_el)?)
                }
                "calling-AP-title" => {
                    calling_AP_title_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                        Ok(_decode_Name(&el.inner()?)?)
                    }(_el)?)
                }
                "calling-AE-qualifier" => {
                    calling_AE_qualifier_ = Some(
                        |el: &X690Element| -> ASN1Result<RelativeDistinguishedName> {
                            Ok(_decode_RelativeDistinguishedName(&el.inner()?)?)
                        }(_el)?,
                    )
                }
                "calling-AP-invocation-identifier" => {
                    calling_AP_invocation_identifier_ =
                        Some(|el: &X690Element| -> ASN1Result<AP_invocation_identifier> {
                            Ok(_decode_AP_invocation_identifier(&el.inner()?)?)
                        }(_el)?)
                }
                "calling-AE-invocation-identifier" => {
                    calling_AE_invocation_identifier_ =
                        Some(|el: &X690Element| -> ASN1Result<AE_invocation_identifier> {
                            Ok(_decode_AE_invocation_identifier(&el.inner()?)?)
                        }(_el)?)
                }
                "implementation-information" => {
                    implementation_information_ = Some(_decode_Implementation_data(_el)?)
                }
                "user-information" => {
                    user_information_ = Some(_decode_Association_informationBind(_el)?)
                }
                _ => {
                    return Err(
                        _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AARQ-apdu")
                    )
                }
            }
        }
        Ok(AARQ_apdu {
            protocol_version: protocol_version_,
            application_context_name: application_context_name_.unwrap(),
            called_AP_title: called_AP_title_,
            called_AE_qualifier: called_AE_qualifier_,
            called_AP_invocation_identifier: called_AP_invocation_identifier_,
            called_AE_invocation_identifier: called_AE_invocation_identifier_,
            calling_AP_title: calling_AP_title_,
            calling_AE_qualifier: calling_AE_qualifier_,
            calling_AP_invocation_identifier: calling_AP_invocation_identifier_,
            calling_AE_invocation_identifier: calling_AE_invocation_identifier_,
            implementation_information: implementation_information_,
            user_information: user_information_.unwrap(),
        })
    }(&el)
}

pub fn _encode_AARQ_apdu(value_: &AARQ_apdu) -> ASN1Result<X690Element> {
    |v_1: &AARQ_apdu| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &AARQ_apdu| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(17);
            if let Some(v_) = &value_.protocol_version {
                if *v_ != AARQ_apdu::_default_value_for_protocol_version() {
                    components_.push(
                        |v_1: &AARQ_apdu_protocol_version| -> ASN1Result<X690Element> {
                            let mut el_1 = _encode_AARQ_apdu_protocol_version(&v_1)?;
                            el_1.tag.tag_class = TagClass::CONTEXT;
                            el_1.tag.tag_number = 0;
                            Ok(el_1)
                        }(&v_)?,
                    );
                }
            }
            components_.push(
                |v_1: &Application_context_name| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        Tag::new(TagClass::CONTEXT, 1),
                        X690Value::from_explicit(&_encode_Application_context_name(&v_1)?),
                    ))
                }(&value_.application_context_name)?,
            );
            if let Some(v_) = &value_.called_AP_title {
                components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        Tag::new(TagClass::CONTEXT, 2),
                        X690Value::from_explicit(&_encode_Name(&v_1)?),
                    ))
                }(&v_)?);
            }
            if let Some(v_) = &value_.called_AE_qualifier {
                components_.push(
                    |v_1: &RelativeDistinguishedName| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 3),
                            X690Value::from_explicit(&_encode_RelativeDistinguishedName(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.called_AP_invocation_identifier {
                components_.push(
                    |v_1: &AP_invocation_identifier| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 4),
                            X690Value::from_explicit(&_encode_AP_invocation_identifier(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.called_AE_invocation_identifier {
                components_.push(
                    |v_1: &AE_invocation_identifier| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 5),
                            X690Value::from_explicit(&_encode_AE_invocation_identifier(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.calling_AP_title {
                components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        Tag::new(TagClass::CONTEXT, 6),
                        X690Value::from_explicit(&_encode_Name(&v_1)?),
                    ))
                }(&v_)?);
            }
            if let Some(v_) = &value_.calling_AE_qualifier {
                components_.push(
                    |v_1: &RelativeDistinguishedName| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 7),
                            X690Value::from_explicit(&_encode_RelativeDistinguishedName(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.calling_AP_invocation_identifier {
                components_.push(
                    |v_1: &AP_invocation_identifier| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 8),
                            X690Value::from_explicit(&_encode_AP_invocation_identifier(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.calling_AE_invocation_identifier {
                components_.push(
                    |v_1: &AE_invocation_identifier| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 9),
                            X690Value::from_explicit(&_encode_AE_invocation_identifier(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.implementation_information {
                components_.push(|v_1: &Implementation_data| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_Implementation_data(&v_1)?;
                    el_1.tag.tag_class = TagClass::CONTEXT;
                    el_1.tag.tag_number = 29;
                    Ok(el_1)
                }(&v_)?);
            }
            components_.push(
                |v_1: &Association_informationBind| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_Association_informationBind(&v_1)?;
                    el_1.tag.tag_class = TagClass::CONTEXT;
                    el_1.tag.tag_number = 30;
                    Ok(el_1)
                }(&value_.user_information)?,
            );
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                X690Value::Constructed(Arc::new(components_)),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::APPLICATION;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_AARQ_apdu(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 0 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AARQ-apdu"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AARQ-apdu")
                    )
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_AARQ_apdu,
                _eal_components_for_AARQ_apdu,
                _rctl2_components_for_AARQ_apdu,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "protocol-version" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "protocol-version",
                            ));
                        }
                        Ok(_validate_AARQ_apdu_protocol_version(&el)?)
                    }(_el)?,
                    "application-context-name" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "application-context-name",
                            ));
                        }
                        Ok(_validate_Application_context_name(&el.inner()?)?)
                    }(_el)?,
                    "called-AP-title" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "called-AP-title",
                            ));
                        }
                        Ok(_validate_Name(&el.inner()?)?)
                    }(_el)?,
                    "called-AE-qualifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "called-AE-qualifier",
                            ));
                        }
                        Ok(_validate_RelativeDistinguishedName(&el.inner()?)?)
                    }(_el)?,
                    "called-AP-invocation-identifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "called-AP-invocation-identifier",
                            ));
                        }
                        Ok(_validate_AP_invocation_identifier(&el.inner()?)?)
                    }(_el)?,
                    "called-AE-invocation-identifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "called-AE-invocation-identifier",
                            ));
                        }
                        Ok(_validate_AE_invocation_identifier(&el.inner()?)?)
                    }(_el)?,
                    "calling-AP-title" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "calling-AP-title",
                            ));
                        }
                        Ok(_validate_Name(&el.inner()?)?)
                    }(_el)?,
                    "calling-AE-qualifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "calling-AE-qualifier",
                            ));
                        }
                        Ok(_validate_RelativeDistinguishedName(&el.inner()?)?)
                    }(_el)?,
                    "calling-AP-invocation-identifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 8 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "calling-AP-invocation-identifier",
                            ));
                        }
                        Ok(_validate_AP_invocation_identifier(&el.inner()?)?)
                    }(_el)?,
                    "calling-AE-invocation-identifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 9 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "calling-AE-invocation-identifier",
                            ));
                        }
                        Ok(_validate_AE_invocation_identifier(&el.inner()?)?)
                    }(_el)?,
                    "implementation-information" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 29 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "implementation-information",
                            ));
                        }
                        Ok(_validate_Implementation_data(&el)?)
                    }(_el)?,
                    "user-information" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 30 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "user-information",
                            ));
                        }
                        Ok(_validate_Association_informationBind(&el)?)
                    }(_el)?,
                    _ => {
                        return Err(
                            _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AARQ-apdu")
                        )
                    }
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Association-informationBind{APPLICATION-CONTEXT:Protocols}  ::=
///   SEQUENCE SIZE (1..MAX) OF
///     EXTERNAL
///       (WITH COMPONENTS {
///          identification         (WITH COMPONENTS {..., syntax ABSENT}),
///          data-value-descriptor  ABSENT,
///          data-value             (CONTAINING TheOsiBind{{Protocols}})})
/// ```
pub type Association_informationBind = Vec<EXTERNAL>; // SequenceOfType

pub fn _decode_Association_informationBind(
    el: &X690Element,
) -> ASN1Result<Association_informationBind> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "Association-informationBind",
            ))
        }
    };
    let mut items: SEQUENCE_OF<EXTERNAL> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(BER.decode_external(el)?);
    }
    Ok(items)
}

pub fn _encode_Association_informationBind(
    value_: &Association_informationBind,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(BER.encode_external(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Association_informationBind(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                BER.validate_external(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "Association-informationBind",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Application-context-name  ::=  OBJECT IDENTIFIER
/// ```
pub type Application_context_name = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_Application_context_name(el: &X690Element) -> ASN1Result<Application_context_name> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_Application_context_name(
    value_: &Application_context_name,
) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_Application_context_name(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AP-invocation-identifier  ::=  INTEGER
/// ```
pub type AP_invocation_identifier = INTEGER;

pub fn _decode_AP_invocation_identifier(el: &X690Element) -> ASN1Result<AP_invocation_identifier> {
    BER.decode_integer(&el)
}

pub fn _encode_AP_invocation_identifier(
    value_: &AP_invocation_identifier,
) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_AP_invocation_identifier(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AE-invocation-identifier  ::=  INTEGER
/// ```
pub type AE_invocation_identifier = INTEGER;

pub fn _decode_AE_invocation_identifier(el: &X690Element) -> ASN1Result<AE_invocation_identifier> {
    BER.decode_integer(&el)
}

pub fn _encode_AE_invocation_identifier(
    value_: &AE_invocation_identifier,
) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_AE_invocation_identifier(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Implementation-data  ::=  GraphicString
/// ```
pub type Implementation_data = GraphicString; // GraphicString

pub fn _decode_Implementation_data(el: &X690Element) -> ASN1Result<Implementation_data> {
    BER.decode_graphic_string(&el)
}

pub fn _encode_Implementation_data(value_: &Implementation_data) -> ASN1Result<X690Element> {
    BER.encode_graphic_string(&value_)
}

pub fn _validate_Implementation_data(el: &X690Element) -> ASN1Result<()> {
    BER.validate_graphic_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TheOsiBind{APPLICATION-CONTEXT:Protocols}  ::=
///   [16]  APPLICATION-CONTEXT.&bind-operation.&ArgumentType({Protocols})
/// ```
pub type TheOsiBind = X690Element; // ObjectClassFieldType

pub fn _decode_TheOsiBind(el: &X690Element) -> ASN1Result<TheOsiBind> {
    |el: &X690Element| -> ASN1Result<TheOsiBind> { Ok(x690_identity(&el.inner()?)?) }(&el)
}

pub fn _encode_TheOsiBind(value_: &TheOsiBind) -> ASN1Result<X690Element> {
    |v_1: &TheOsiBind| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 16),
            X690Value::from_explicit(&x690_identity(&v_1)?),
        ))
    }(&value_)
}

pub fn _validate_TheOsiBind(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 16 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TheOsiBind"));
        }
        Ok(BER.validate_any(&el.inner()?)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindResult{APPLICATION-CONTEXT:Protocols} ::= SET {
///   mode-selector                    [0] IMPLICIT SET {mode-value  [0] IMPLICIT INTEGER(1)},
///   normal-mode-parameters           [2] IMPLICIT SEQUENCE {
///     protocol-version                 [0] IMPLICIT BIT STRING {version-1(0)}
///                                            DEFAULT {version-1},
///     responding-presentation-selector [3] IMPLICIT Presentation-selector OPTIONAL,
///     presentation-context-definition-result-list
///                                      [5] IMPLICIT SEQUENCE SIZE (2) OF SEQUENCE {
///       result                           [0] IMPLICIT Result(acceptance),
///       transfer-syntax-name             [1] IMPLICIT Transfer-syntax-name },
///     user-data                            CHOICE {
///       fully-encoded-data [APPLICATION 1] IMPLICIT SEQUENCE SIZE(1) OF SEQUENCE {
///         transfer-syntax-name               Transfer-syntax-name OPTIONAL,
///         presentation-context-identifier    Presentation-context-identifier,
///         presentation-data-values           CHOICE {
///           single-ASN1-type              [0]  ABSTRACT-SYNTAX.&Type(AARE-apdu{{Protocols}}
///   )}}}}}
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiBindResult {
    pub mode_selector: OsiBindResult_mode_selector,
    pub normal_mode_parameters: OsiBindResult_normal_mode_parameters,
}
impl OsiBindResult {
    pub fn new(
        mode_selector: OsiBindResult_mode_selector,
        normal_mode_parameters: OsiBindResult_normal_mode_parameters,
    ) -> Self {
        OsiBindResult {
            mode_selector,
            normal_mode_parameters,
        }
    }
}
impl TryFrom<&X690Element> for OsiBindResult {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBindResult(el)
    }
}

pub const _rctl1_components_for_OsiBindResult: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "mode-selector",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "normal-mode-parameters",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_OsiBindResult: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiBindResult: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiBindResult(el: &X690Element) -> ASN1Result<OsiBindResult> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBindResult")),
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_OsiBindResult,
        _eal_components_for_OsiBindResult,
        _rctl2_components_for_OsiBindResult,
        20,
    )?;
    let mode_selector_ =
        _decode_OsiBindResult_mode_selector(_components.get("mode-selector").unwrap())?;
    let normal_mode_parameters_ = _decode_OsiBindResult_normal_mode_parameters(
        _components.get("normal-mode-parameters").unwrap(),
    )?;
    Ok(OsiBindResult {
        mode_selector: mode_selector_,
        normal_mode_parameters: normal_mode_parameters_,
    })
}

pub fn _encode_OsiBindResult(value_: &OsiBindResult) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(
        |v_1: &OsiBindResult_mode_selector| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_OsiBindResult_mode_selector(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&value_.mode_selector)?,
    );
    components_.push(
        |v_1: &OsiBindResult_normal_mode_parameters| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_OsiBindResult_normal_mode_parameters(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&value_.normal_mode_parameters)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiBindResult(el: &X690Element) -> ASN1Result<()> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBindResult")),
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_OsiBindResult,
        _eal_components_for_OsiBindResult,
        _rctl2_components_for_OsiBindResult,
        20,
    )?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "mode-selector"));
        }
        Ok(_validate_OsiBindResult_mode_selector(&el)?)
    }(_components.get("mode-selector").unwrap())?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "normal-mode-parameters",
            ));
        }
        Ok(_validate_OsiBindResult_normal_mode_parameters(&el)?)
    }(_components.get("normal-mode-parameters").unwrap())?;
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Result  ::=  INTEGER {
///   acceptance         (0),
///   user-rejection     (1),
///   provider-rejection (2)}
/// ```
pub type Result_ = i8;

pub const Result__acceptance: Result_ = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const Result__user_rejection: Result_ = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Result__provider_rejection: Result_ = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Result_(el: &X690Element) -> ASN1Result<Result_> {
    BER.decode_i8(el)
}

pub fn _encode_Result_(value_: &Result_) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Result_(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AARE-apdu{APPLICATION-CONTEXT:Protocols} ::= [APPLICATION 1] IMPLICIT SEQUENCE {
///   protocol-version                     [0] IMPLICIT BIT STRING {version1(0)}
///                                              DEFAULT {version1},
///   application-context-name             [1]  Application-context-name,
///   result                               [2]  Associate-result(accepted),
///   result-source-diagnostic             [3]  Associate-source-diagnostic,
///   responding-AP-title                  [4]  Name                          OPTIONAL,
///   responding-AE-qualifier              [5]  RelativeDistinguishedName     OPTIONAL,
///   responding-AP-invocation-identifier  [6]  AP-invocation-identifier      OPTIONAL,
///   responding-AE-invocation-identifier  [7]  AE-invocation-identifier      OPTIONAL,
///   implementation-information           [29] IMPLICIT Implementation-data  OPTIONAL,
///   user-information                     [30] IMPLICIT
///                                         Association-informationBindRes{{Protocols}}}
/// ```
///
#[derive(Debug, Clone)]
pub struct AARE_apdu {
    pub protocol_version: OPTIONAL<AARE_apdu_protocol_version>,
    pub application_context_name: Application_context_name,
    pub result: Associate_result,
    pub result_source_diagnostic: Associate_source_diagnostic,
    pub responding_AP_title: OPTIONAL<Name>,
    pub responding_AE_qualifier: OPTIONAL<RelativeDistinguishedName>,
    pub responding_AP_invocation_identifier: OPTIONAL<AP_invocation_identifier>,
    pub responding_AE_invocation_identifier: OPTIONAL<AE_invocation_identifier>,
    pub implementation_information: OPTIONAL<Implementation_data>,
    pub user_information: Association_informationBindRes,
}
impl AARE_apdu {
    pub fn new(
        protocol_version: OPTIONAL<AARE_apdu_protocol_version>,
        application_context_name: Application_context_name,
        result: Associate_result,
        result_source_diagnostic: Associate_source_diagnostic,
        responding_AP_title: OPTIONAL<Name>,
        responding_AE_qualifier: OPTIONAL<RelativeDistinguishedName>,
        responding_AP_invocation_identifier: OPTIONAL<AP_invocation_identifier>,
        responding_AE_invocation_identifier: OPTIONAL<AE_invocation_identifier>,
        implementation_information: OPTIONAL<Implementation_data>,
        user_information: Association_informationBindRes,
    ) -> Self {
        AARE_apdu {
            protocol_version,
            application_context_name,
            result,
            result_source_diagnostic,
            responding_AP_title,
            responding_AE_qualifier,
            responding_AP_invocation_identifier,
            responding_AE_invocation_identifier,
            implementation_information,
            user_information,
        }
    }
    pub fn _default_value_for_protocol_version() -> AARE_apdu_protocol_version {
        BIT_STRING::with_bits_set(&[AARE_apdu_protocol_version_version1])
    }
}
impl TryFrom<&X690Element> for AARE_apdu {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AARE_apdu(el)
    }
}

pub const _rctl1_components_for_AARE_apdu: &[ComponentSpec; 10] = &[
    ComponentSpec::new(
        "protocol-version",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "application-context-name",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "result",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "result-source-diagnostic",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responding-AP-title",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responding-AE-qualifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responding-AP-invocation-identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responding-AE-invocation-identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "implementation-information",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "user-information",
        false,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AARE_apdu: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AARE_apdu: &[ComponentSpec; 0] = &[];

pub fn _decode_AARE_apdu(el: &X690Element) -> ASN1Result<AARE_apdu> {
    |el: &X690Element| -> ASN1Result<AARE_apdu> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AARE-apdu")),
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_AARE_apdu,
            _eal_components_for_AARE_apdu,
            _rctl2_components_for_AARE_apdu,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut protocol_version_: OPTIONAL<AARE_apdu_protocol_version> = None;
        let mut application_context_name_: OPTIONAL<Application_context_name> = None;
        let mut result_: OPTIONAL<Associate_result> = None;
        let mut result_source_diagnostic_: OPTIONAL<Associate_source_diagnostic> = None;
        let mut responding_AP_title_: OPTIONAL<Name> = None;
        let mut responding_AE_qualifier_: OPTIONAL<RelativeDistinguishedName> = None;
        let mut responding_AP_invocation_identifier_: OPTIONAL<AP_invocation_identifier> = None;
        let mut responding_AE_invocation_identifier_: OPTIONAL<AE_invocation_identifier> = None;
        let mut implementation_information_: OPTIONAL<Implementation_data> = None;
        let mut user_information_: OPTIONAL<Association_informationBindRes> = None;
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "protocol-version" => {
                    protocol_version_ = Some(_decode_AARE_apdu_protocol_version(_el)?)
                }
                "application-context-name" => {
                    application_context_name_ =
                        Some(|el: &X690Element| -> ASN1Result<Application_context_name> {
                            Ok(_decode_Application_context_name(&el.inner()?)?)
                        }(_el)?)
                }
                "result" => {
                    result_ = Some(|el: &X690Element| -> ASN1Result<Associate_result> {
                        Ok(_decode_Associate_result(&el.inner()?)?)
                    }(_el)?)
                }
                "result-source-diagnostic" => {
                    result_source_diagnostic_ = Some(|el: &X690Element| -> ASN1Result<
                        Associate_source_diagnostic,
                    > {
                        Ok(_decode_Associate_source_diagnostic(&el.inner()?)?)
                    }(_el)?)
                }
                "responding-AP-title" => {
                    responding_AP_title_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                        Ok(_decode_Name(&el.inner()?)?)
                    }(_el)?)
                }
                "responding-AE-qualifier" => {
                    responding_AE_qualifier_ = Some(
                        |el: &X690Element| -> ASN1Result<RelativeDistinguishedName> {
                            Ok(_decode_RelativeDistinguishedName(&el.inner()?)?)
                        }(_el)?,
                    )
                }
                "responding-AP-invocation-identifier" => {
                    responding_AP_invocation_identifier_ =
                        Some(|el: &X690Element| -> ASN1Result<AP_invocation_identifier> {
                            Ok(_decode_AP_invocation_identifier(&el.inner()?)?)
                        }(_el)?)
                }
                "responding-AE-invocation-identifier" => {
                    responding_AE_invocation_identifier_ =
                        Some(|el: &X690Element| -> ASN1Result<AE_invocation_identifier> {
                            Ok(_decode_AE_invocation_identifier(&el.inner()?)?)
                        }(_el)?)
                }
                "implementation-information" => {
                    implementation_information_ = Some(_decode_Implementation_data(_el)?)
                }
                "user-information" => {
                    user_information_ = Some(_decode_Association_informationBindRes(_el)?)
                }
                _ => {
                    return Err(
                        _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AARE-apdu")
                    )
                }
            }
        }
        Ok(AARE_apdu {
            protocol_version: protocol_version_,
            application_context_name: application_context_name_.unwrap(),
            result: result_.unwrap(),
            result_source_diagnostic: result_source_diagnostic_.unwrap(),
            responding_AP_title: responding_AP_title_,
            responding_AE_qualifier: responding_AE_qualifier_,
            responding_AP_invocation_identifier: responding_AP_invocation_identifier_,
            responding_AE_invocation_identifier: responding_AE_invocation_identifier_,
            implementation_information: implementation_information_,
            user_information: user_information_.unwrap(),
        })
    }(&el)
}

pub fn _encode_AARE_apdu(value_: &AARE_apdu) -> ASN1Result<X690Element> {
    |v_1: &AARE_apdu| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &AARE_apdu| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(15);
            if let Some(v_) = &value_.protocol_version {
                if *v_ != AARE_apdu::_default_value_for_protocol_version() {
                    components_.push(
                        |v_1: &AARE_apdu_protocol_version| -> ASN1Result<X690Element> {
                            let mut el_1 = _encode_AARE_apdu_protocol_version(&v_1)?;
                            el_1.tag.tag_class = TagClass::CONTEXT;
                            el_1.tag.tag_number = 0;
                            Ok(el_1)
                        }(&v_)?,
                    );
                }
            }
            components_.push(
                |v_1: &Application_context_name| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        Tag::new(TagClass::CONTEXT, 1),
                        X690Value::from_explicit(&_encode_Application_context_name(&v_1)?),
                    ))
                }(&value_.application_context_name)?,
            );
            components_.push(|v_1: &Associate_result| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(&_encode_Associate_result(&v_1)?),
                ))
            }(&value_.result)?);
            components_.push(
                |v_1: &Associate_source_diagnostic| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        Tag::new(TagClass::CONTEXT, 3),
                        X690Value::from_explicit(&_encode_Associate_source_diagnostic(&v_1)?),
                    ))
                }(&value_.result_source_diagnostic)?,
            );
            if let Some(v_) = &value_.responding_AP_title {
                components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        Tag::new(TagClass::CONTEXT, 4),
                        X690Value::from_explicit(&_encode_Name(&v_1)?),
                    ))
                }(&v_)?);
            }
            if let Some(v_) = &value_.responding_AE_qualifier {
                components_.push(
                    |v_1: &RelativeDistinguishedName| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 5),
                            X690Value::from_explicit(&_encode_RelativeDistinguishedName(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.responding_AP_invocation_identifier {
                components_.push(
                    |v_1: &AP_invocation_identifier| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 6),
                            X690Value::from_explicit(&_encode_AP_invocation_identifier(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.responding_AE_invocation_identifier {
                components_.push(
                    |v_1: &AE_invocation_identifier| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 7),
                            X690Value::from_explicit(&_encode_AE_invocation_identifier(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.implementation_information {
                components_.push(|v_1: &Implementation_data| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_Implementation_data(&v_1)?;
                    el_1.tag.tag_class = TagClass::CONTEXT;
                    el_1.tag.tag_number = 29;
                    Ok(el_1)
                }(&v_)?);
            }
            components_.push(
                |v_1: &Association_informationBindRes| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_Association_informationBindRes(&v_1)?;
                    el_1.tag.tag_class = TagClass::CONTEXT;
                    el_1.tag.tag_number = 30;
                    Ok(el_1)
                }(&value_.user_information)?,
            );
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                X690Value::Constructed(Arc::new(components_)),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::APPLICATION;
        el_1.tag.tag_number = 1;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_AARE_apdu(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 1 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AARE-apdu"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AARE-apdu")
                    )
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_AARE_apdu,
                _eal_components_for_AARE_apdu,
                _rctl2_components_for_AARE_apdu,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "protocol-version" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "protocol-version",
                            ));
                        }
                        Ok(_validate_AARE_apdu_protocol_version(&el)?)
                    }(_el)?,
                    "application-context-name" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "application-context-name",
                            ));
                        }
                        Ok(_validate_Application_context_name(&el.inner()?)?)
                    }(_el)?,
                    "result" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "result")
                            );
                        }
                        Ok(_validate_Associate_result(&el.inner()?)?)
                    }(_el)?,
                    "result-source-diagnostic" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "result-source-diagnostic",
                            ));
                        }
                        Ok(_validate_Associate_source_diagnostic(&el.inner()?)?)
                    }(_el)?,
                    "responding-AP-title" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "responding-AP-title",
                            ));
                        }
                        Ok(_validate_Name(&el.inner()?)?)
                    }(_el)?,
                    "responding-AE-qualifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "responding-AE-qualifier",
                            ));
                        }
                        Ok(_validate_RelativeDistinguishedName(&el.inner()?)?)
                    }(_el)?,
                    "responding-AP-invocation-identifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "responding-AP-invocation-identifier",
                            ));
                        }
                        Ok(_validate_AP_invocation_identifier(&el.inner()?)?)
                    }(_el)?,
                    "responding-AE-invocation-identifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "responding-AE-invocation-identifier",
                            ));
                        }
                        Ok(_validate_AE_invocation_identifier(&el.inner()?)?)
                    }(_el)?,
                    "implementation-information" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 29 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "implementation-information",
                            ));
                        }
                        Ok(_validate_Implementation_data(&el)?)
                    }(_el)?,
                    "user-information" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 30 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "user-information",
                            ));
                        }
                        Ok(_validate_Association_informationBindRes(&el)?)
                    }(_el)?,
                    _ => {
                        return Err(
                            _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AARE-apdu")
                        )
                    }
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Association-informationBindRes{APPLICATION-CONTEXT:Protocols}  ::=
///   SEQUENCE SIZE (1) OF
///     EXTERNAL (
///       WITH COMPONENTS {
///         identification         (WITH COMPONENTS {..., syntax ABSENT}),
///         data-value-descriptor  ABSENT,
///         data-value             (CONTAINING TheOsiBindRes{{Protocols}})})
/// ```
pub type Association_informationBindRes = Vec<EXTERNAL>; // SequenceOfType

pub fn _decode_Association_informationBindRes(
    el: &X690Element,
) -> ASN1Result<Association_informationBindRes> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "Association-informationBindRes",
            ))
        }
    };
    let mut items: SEQUENCE_OF<EXTERNAL> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(BER.decode_external(el)?);
    }
    Ok(items)
}

pub fn _encode_Association_informationBindRes(
    value_: &Association_informationBindRes,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(BER.encode_external(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Association_informationBindRes(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                BER.validate_external(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "Association-informationBindRes",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Associate-result  ::=  INTEGER {
///   accepted           (0),
///   rejected-permanent (1),
///   rejected-transient (2)}(0..2, ...)
/// ```
pub type Associate_result = i8;

pub const Associate_result_accepted: Associate_result = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_result_rejected_permanent: Associate_result = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_result_rejected_transient: Associate_result = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Associate_result(el: &X690Element) -> ASN1Result<Associate_result> {
    BER.decode_i8(el)
}

pub fn _encode_Associate_result(value_: &Associate_result) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Associate_result(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Associate-source-diagnostic  ::=  CHOICE {
///   acse-service-user     [1]  INTEGER {
///     null                                            (0),
///     no-reason-given                                 (1),
///     application-context-name-not-supported          (2),
///     calling-AP-title-not-recognized                 (3),
///     calling-AP-invocation-identifier-not-recognized (4),
///     calling-AE-qualifier-not-recognized             (5),
///     calling-AE-invocation-identifier-not-recognized (6),
///     called-AP-title-not-recognized                  (7),
///     called-AP-invocation-identifier-not-recognized  (8),
///     called-AE-qualifier-not-recognized              (9),
///     called-AE-invocation-identifier-not-recognized  (10)}(0..10, ...),
///   acse-service-provider [2]  INTEGER {
///     null                                            (0),
///     no-reason-given                                 (1),
///     no-common-acse-version                          (2)}(0..2, ...)}
/// ```
#[derive(Debug, Clone)]
pub enum Associate_source_diagnostic {
    acse_service_user(Associate_source_diagnostic_acse_service_user),
    acse_service_provider(Associate_source_diagnostic_acse_service_provider),
}

impl TryFrom<&X690Element> for Associate_source_diagnostic {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Associate_source_diagnostic(el)
    }
}

pub fn _decode_Associate_source_diagnostic(
    el: &X690Element,
) -> ASN1Result<Associate_source_diagnostic> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 1) => Ok(Associate_source_diagnostic::acse_service_user(
            |el: &X690Element| -> ASN1Result<Associate_source_diagnostic_acse_service_user> {
                Ok(_decode_Associate_source_diagnostic_acse_service_user(
                    &el.inner()?,
                )?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(Associate_source_diagnostic::acse_service_provider(
            |el: &X690Element| -> ASN1Result<Associate_source_diagnostic_acse_service_provider> {
                Ok(_decode_Associate_source_diagnostic_acse_service_provider(
                    &el.inner()?,
                )?)
            }(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Associate-source-diagnostic",
            ))
        }
    }
}

pub fn _encode_Associate_source_diagnostic(
    value_: &Associate_source_diagnostic,
) -> ASN1Result<X690Element> {
    match value_ {
        Associate_source_diagnostic::acse_service_user(v) => {
            |v_1: &Associate_source_diagnostic_acse_service_user| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(
                        &_encode_Associate_source_diagnostic_acse_service_user(&v_1)?,
                    ),
                ))
            }(&v)
        }
        Associate_source_diagnostic::acse_service_provider(v) => {
            |v_1: &Associate_source_diagnostic_acse_service_provider| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(
                        &_encode_Associate_source_diagnostic_acse_service_provider(&v_1)?,
                    ),
                ))
            }(&v)
        }
    }
}

pub fn _validate_Associate_source_diagnostic(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "acse-service-user")
                );
            }
            Ok(_validate_Associate_source_diagnostic_acse_service_user(
                &el.inner()?,
            )?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "acse-service-provider",
                ));
            }
            Ok(_validate_Associate_source_diagnostic_acse_service_provider(
                &el.inner()?,
            )?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "Associate-source-diagnostic",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TheOsiBindRes{APPLICATION-CONTEXT:Protocols}  ::=
///   [17]  APPLICATION-CONTEXT.&bind-operation.&ResultType({Protocols})
/// ```
pub type TheOsiBindRes = X690Element; // ObjectClassFieldType

pub fn _decode_TheOsiBindRes(el: &X690Element) -> ASN1Result<TheOsiBindRes> {
    |el: &X690Element| -> ASN1Result<TheOsiBindRes> { Ok(x690_identity(&el.inner()?)?) }(&el)
}

pub fn _encode_TheOsiBindRes(value_: &TheOsiBindRes) -> ASN1Result<X690Element> {
    |v_1: &TheOsiBindRes| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 17),
            X690Value::from_explicit(&x690_identity(&v_1)?),
        ))
    }(&value_)
}

pub fn _validate_TheOsiBindRes(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 17 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TheOsiBindRes"));
        }
        Ok(BER.validate_any(&el.inner()?)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindError{APPLICATION-CONTEXT:Protocols}  ::=  CHOICE {
///   normal-mode-parameters  SEQUENCE {
///     protocol-version               [0]  IMPLICIT BIT STRING {version-1(0)}
///                                           DEFAULT {version-1},
///     responding-presentation-selector
///                                    [3]  IMPLICIT Presentation-selector OPTIONAL,
///     presentation-context-definition-result-list
///                                    [5]  IMPLICIT Result-list OPTIONAL,
///     provider-reason                [10] IMPLICIT Provider-reason OPTIONAL,
///     user-data                           CHOICE {
///       fully-encoded-data  [APPLICATION 1] IMPLICIT SEQUENCE SIZE (1) OF SEQUENCE {
///         transfer-syntax-name                Transfer-syntax-name   OPTIONAL,
///         presentation-context-identifier     Presentation-context-identifier,
///         presentation-data-values            CHOICE {
///           single-ASN1-type               [0]
///                      ABSTRACT-SYNTAX.&Type(AAREerr-apdu{{Protocols}})}}} OPTIONAL}}
/// ```
#[derive(Debug, Clone)]
pub enum OsiBindError {
    normal_mode_parameters(OsiBindError_normal_mode_parameters),
}

impl TryFrom<&X690Element> for OsiBindError {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBindError(el)
    }
}

pub fn _decode_OsiBindError(el: &X690Element) -> ASN1Result<OsiBindError> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(OsiBindError::normal_mode_parameters(
            _decode_OsiBindError_normal_mode_parameters(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiBindError",
            ))
        }
    }
}

pub fn _encode_OsiBindError(value_: &OsiBindError) -> ASN1Result<X690Element> {
    match value_ {
        OsiBindError::normal_mode_parameters(v) => _encode_OsiBindError_normal_mode_parameters(&v),
    }
}

pub fn _validate_OsiBindError(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_OsiBindError_normal_mode_parameters(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiBindError",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Result-list  ::=
///   SEQUENCE SIZE (2) OF SEQUENCE {
///     result                [0] IMPLICIT Result,
///     transfer-syntax-name  [1] IMPLICIT Transfer-syntax-name   OPTIONAL,
///     provider-reason       [2] IMPLICIT INTEGER {
///       reason-not-specified                     (0),
///       abstract-syntax-not-supported            (1),
///       proposed-transfer-syntaxes-not-supported (2)} OPTIONAL}
/// ```
pub type Result_list = Vec<Result_list_Item>; // SequenceOfType

pub fn _decode_Result_list(el: &X690Element) -> ASN1Result<Result_list> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Result-list")),
    };
    let mut items: SEQUENCE_OF<Result_list_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Result_list_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_Result_list(value_: &Result_list) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Result_list_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Result_list(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Result_list_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Result-list")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Provider-reason  ::=  INTEGER {
///   reason-not-specified                (0),
///   temporary-congestion                (1),
///   local-limit-exceeded                (2),
///   called-presentation-address-unknown (3),
///   protocol-version-not-supported      (4),
///   default-context-not-supported       (5),
///   user-data-not-readable              (6),
///   no-PSAP-available                   (7)}
/// ```
pub type Provider_reason = i8;

pub const Provider_reason_reason_not_specified: Provider_reason = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const Provider_reason_temporary_congestion: Provider_reason = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Provider_reason_local_limit_exceeded: Provider_reason = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const Provider_reason_called_presentation_address_unknown: Provider_reason = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const Provider_reason_protocol_version_not_supported: Provider_reason = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const Provider_reason_default_context_not_supported: Provider_reason = 5; /* LONG_NAMED_INTEGER_VALUE */

pub const Provider_reason_user_data_not_readable: Provider_reason = 6; /* LONG_NAMED_INTEGER_VALUE */

pub const Provider_reason_no_PSAP_available: Provider_reason = 7; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Provider_reason(el: &X690Element) -> ASN1Result<Provider_reason> {
    BER.decode_i8(el)
}

pub fn _encode_Provider_reason(value_: &Provider_reason) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Provider_reason(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AAREerr-apdu{APPLICATION-CONTEXT:Protocols} ::= [APPLICATION 1] IMPLICIT SEQUENCE {
///   protocol-version                    [0]  IMPLICIT BIT STRING {version1(0)}
///                                              DEFAULT {version1},
///   application-context-name            [1]  Application-context-name,
///   result                              [2]  Associate-result
///                                              (rejected-permanent..rejected-transient),
///   result-source-diagnostic            [3]  Associate-source-diagnostic,
///   responding-AP-title                 [4]  Name OPTIONAL,
///   responding-AE-qualifier             [5]  RelativeDistinguishedName OPTIONAL,
///   responding-AP-invocation-identifier [6]  AP-invocation-identifier  OPTIONAL,
///   responding-AE-invocation-identifier [7]  AE-invocation-identifier  OPTIONAL,
///   implementation-information          [29] IMPLICIT Implementation-data OPTIONAL,
///   user-information                    [30] IMPLICIT
///                                 Association-informationBindErr{{Protocols}} OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct AAREerr_apdu {
    pub protocol_version: OPTIONAL<AAREerr_apdu_protocol_version>,
    pub application_context_name: Application_context_name,
    pub result: Associate_result,
    pub result_source_diagnostic: Associate_source_diagnostic,
    pub responding_AP_title: OPTIONAL<Name>,
    pub responding_AE_qualifier: OPTIONAL<RelativeDistinguishedName>,
    pub responding_AP_invocation_identifier: OPTIONAL<AP_invocation_identifier>,
    pub responding_AE_invocation_identifier: OPTIONAL<AE_invocation_identifier>,
    pub implementation_information: OPTIONAL<Implementation_data>,
    pub user_information: OPTIONAL<Association_informationBindErr>,
}
impl AAREerr_apdu {
    pub fn new(
        protocol_version: OPTIONAL<AAREerr_apdu_protocol_version>,
        application_context_name: Application_context_name,
        result: Associate_result,
        result_source_diagnostic: Associate_source_diagnostic,
        responding_AP_title: OPTIONAL<Name>,
        responding_AE_qualifier: OPTIONAL<RelativeDistinguishedName>,
        responding_AP_invocation_identifier: OPTIONAL<AP_invocation_identifier>,
        responding_AE_invocation_identifier: OPTIONAL<AE_invocation_identifier>,
        implementation_information: OPTIONAL<Implementation_data>,
        user_information: OPTIONAL<Association_informationBindErr>,
    ) -> Self {
        AAREerr_apdu {
            protocol_version,
            application_context_name,
            result,
            result_source_diagnostic,
            responding_AP_title,
            responding_AE_qualifier,
            responding_AP_invocation_identifier,
            responding_AE_invocation_identifier,
            implementation_information,
            user_information,
        }
    }
    pub fn _default_value_for_protocol_version() -> AAREerr_apdu_protocol_version {
        BIT_STRING::with_bits_set(&[AAREerr_apdu_protocol_version_version1])
    }
}
impl TryFrom<&X690Element> for AAREerr_apdu {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AAREerr_apdu(el)
    }
}

pub const _rctl1_components_for_AAREerr_apdu: &[ComponentSpec; 10] = &[
    ComponentSpec::new(
        "protocol-version",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "application-context-name",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "result",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "result-source-diagnostic",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responding-AP-title",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responding-AE-qualifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responding-AP-invocation-identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responding-AE-invocation-identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "implementation-information",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
    ComponentSpec::new(
        "user-information",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AAREerr_apdu: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AAREerr_apdu: &[ComponentSpec; 0] = &[];

pub fn _decode_AAREerr_apdu(el: &X690Element) -> ASN1Result<AAREerr_apdu> {
    |el: &X690Element| -> ASN1Result<AAREerr_apdu> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AAREerr-apdu")
                )
            }
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_AAREerr_apdu,
            _eal_components_for_AAREerr_apdu,
            _rctl2_components_for_AAREerr_apdu,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut protocol_version_: OPTIONAL<AAREerr_apdu_protocol_version> = None;
        let mut application_context_name_: OPTIONAL<Application_context_name> = None;
        let mut result_: OPTIONAL<Associate_result> = None;
        let mut result_source_diagnostic_: OPTIONAL<Associate_source_diagnostic> = None;
        let mut responding_AP_title_: OPTIONAL<Name> = None;
        let mut responding_AE_qualifier_: OPTIONAL<RelativeDistinguishedName> = None;
        let mut responding_AP_invocation_identifier_: OPTIONAL<AP_invocation_identifier> = None;
        let mut responding_AE_invocation_identifier_: OPTIONAL<AE_invocation_identifier> = None;
        let mut implementation_information_: OPTIONAL<Implementation_data> = None;
        let mut user_information_: OPTIONAL<Association_informationBindErr> = None;
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "protocol-version" => {
                    protocol_version_ = Some(_decode_AAREerr_apdu_protocol_version(_el)?)
                }
                "application-context-name" => {
                    application_context_name_ =
                        Some(|el: &X690Element| -> ASN1Result<Application_context_name> {
                            Ok(_decode_Application_context_name(&el.inner()?)?)
                        }(_el)?)
                }
                "result" => {
                    result_ = Some(|el: &X690Element| -> ASN1Result<Associate_result> {
                        Ok(_decode_Associate_result(&el.inner()?)?)
                    }(_el)?)
                }
                "result-source-diagnostic" => {
                    result_source_diagnostic_ = Some(|el: &X690Element| -> ASN1Result<
                        Associate_source_diagnostic,
                    > {
                        Ok(_decode_Associate_source_diagnostic(&el.inner()?)?)
                    }(_el)?)
                }
                "responding-AP-title" => {
                    responding_AP_title_ = Some(|el: &X690Element| -> ASN1Result<Name> {
                        Ok(_decode_Name(&el.inner()?)?)
                    }(_el)?)
                }
                "responding-AE-qualifier" => {
                    responding_AE_qualifier_ = Some(
                        |el: &X690Element| -> ASN1Result<RelativeDistinguishedName> {
                            Ok(_decode_RelativeDistinguishedName(&el.inner()?)?)
                        }(_el)?,
                    )
                }
                "responding-AP-invocation-identifier" => {
                    responding_AP_invocation_identifier_ =
                        Some(|el: &X690Element| -> ASN1Result<AP_invocation_identifier> {
                            Ok(_decode_AP_invocation_identifier(&el.inner()?)?)
                        }(_el)?)
                }
                "responding-AE-invocation-identifier" => {
                    responding_AE_invocation_identifier_ =
                        Some(|el: &X690Element| -> ASN1Result<AE_invocation_identifier> {
                            Ok(_decode_AE_invocation_identifier(&el.inner()?)?)
                        }(_el)?)
                }
                "implementation-information" => {
                    implementation_information_ = Some(_decode_Implementation_data(_el)?)
                }
                "user-information" => {
                    user_information_ = Some(_decode_Association_informationBindErr(_el)?)
                }
                _ => {
                    return Err(
                        _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AAREerr-apdu")
                    )
                }
            }
        }
        Ok(AAREerr_apdu {
            protocol_version: protocol_version_,
            application_context_name: application_context_name_.unwrap(),
            result: result_.unwrap(),
            result_source_diagnostic: result_source_diagnostic_.unwrap(),
            responding_AP_title: responding_AP_title_,
            responding_AE_qualifier: responding_AE_qualifier_,
            responding_AP_invocation_identifier: responding_AP_invocation_identifier_,
            responding_AE_invocation_identifier: responding_AE_invocation_identifier_,
            implementation_information: implementation_information_,
            user_information: user_information_,
        })
    }(&el)
}

pub fn _encode_AAREerr_apdu(value_: &AAREerr_apdu) -> ASN1Result<X690Element> {
    |v_1: &AAREerr_apdu| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &AAREerr_apdu| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(15);
            if let Some(v_) = &value_.protocol_version {
                if *v_ != AAREerr_apdu::_default_value_for_protocol_version() {
                    components_.push(
                        |v_1: &AAREerr_apdu_protocol_version| -> ASN1Result<X690Element> {
                            let mut el_1 = _encode_AAREerr_apdu_protocol_version(&v_1)?;
                            el_1.tag.tag_class = TagClass::CONTEXT;
                            el_1.tag.tag_number = 0;
                            Ok(el_1)
                        }(&v_)?,
                    );
                }
            }
            components_.push(
                |v_1: &Application_context_name| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        Tag::new(TagClass::CONTEXT, 1),
                        X690Value::from_explicit(&_encode_Application_context_name(&v_1)?),
                    ))
                }(&value_.application_context_name)?,
            );
            components_.push(|v_1: &Associate_result| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(&_encode_Associate_result(&v_1)?),
                ))
            }(&value_.result)?);
            components_.push(
                |v_1: &Associate_source_diagnostic| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        Tag::new(TagClass::CONTEXT, 3),
                        X690Value::from_explicit(&_encode_Associate_source_diagnostic(&v_1)?),
                    ))
                }(&value_.result_source_diagnostic)?,
            );
            if let Some(v_) = &value_.responding_AP_title {
                components_.push(|v_1: &Name| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        Tag::new(TagClass::CONTEXT, 4),
                        X690Value::from_explicit(&_encode_Name(&v_1)?),
                    ))
                }(&v_)?);
            }
            if let Some(v_) = &value_.responding_AE_qualifier {
                components_.push(
                    |v_1: &RelativeDistinguishedName| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 5),
                            X690Value::from_explicit(&_encode_RelativeDistinguishedName(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.responding_AP_invocation_identifier {
                components_.push(
                    |v_1: &AP_invocation_identifier| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 6),
                            X690Value::from_explicit(&_encode_AP_invocation_identifier(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.responding_AE_invocation_identifier {
                components_.push(
                    |v_1: &AE_invocation_identifier| -> ASN1Result<X690Element> {
                        Ok(X690Element::new(
                            Tag::new(TagClass::CONTEXT, 7),
                            X690Value::from_explicit(&_encode_AE_invocation_identifier(&v_1)?),
                        ))
                    }(&v_)?,
                );
            }
            if let Some(v_) = &value_.implementation_information {
                components_.push(|v_1: &Implementation_data| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_Implementation_data(&v_1)?;
                    el_1.tag.tag_class = TagClass::CONTEXT;
                    el_1.tag.tag_number = 29;
                    Ok(el_1)
                }(&v_)?);
            }
            if let Some(v_) = &value_.user_information {
                components_.push(
                    |v_1: &Association_informationBindErr| -> ASN1Result<X690Element> {
                        let mut el_1 = _encode_Association_informationBindErr(&v_1)?;
                        el_1.tag.tag_class = TagClass::CONTEXT;
                        el_1.tag.tag_number = 30;
                        Ok(el_1)
                    }(&v_)?,
                );
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                X690Value::Constructed(Arc::new(components_)),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::APPLICATION;
        el_1.tag.tag_number = 1;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_AAREerr_apdu(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 1 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AAREerr-apdu"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AAREerr-apdu")
                    )
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_AAREerr_apdu,
                _eal_components_for_AAREerr_apdu,
                _rctl2_components_for_AAREerr_apdu,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "protocol-version" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "protocol-version",
                            ));
                        }
                        Ok(_validate_AAREerr_apdu_protocol_version(&el)?)
                    }(_el)?,
                    "application-context-name" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "application-context-name",
                            ));
                        }
                        Ok(_validate_Application_context_name(&el.inner()?)?)
                    }(_el)?,
                    "result" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "result")
                            );
                        }
                        Ok(_validate_Associate_result(&el.inner()?)?)
                    }(_el)?,
                    "result-source-diagnostic" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "result-source-diagnostic",
                            ));
                        }
                        Ok(_validate_Associate_source_diagnostic(&el.inner()?)?)
                    }(_el)?,
                    "responding-AP-title" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "responding-AP-title",
                            ));
                        }
                        Ok(_validate_Name(&el.inner()?)?)
                    }(_el)?,
                    "responding-AE-qualifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "responding-AE-qualifier",
                            ));
                        }
                        Ok(_validate_RelativeDistinguishedName(&el.inner()?)?)
                    }(_el)?,
                    "responding-AP-invocation-identifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "responding-AP-invocation-identifier",
                            ));
                        }
                        Ok(_validate_AP_invocation_identifier(&el.inner()?)?)
                    }(_el)?,
                    "responding-AE-invocation-identifier" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "responding-AE-invocation-identifier",
                            ));
                        }
                        Ok(_validate_AE_invocation_identifier(&el.inner()?)?)
                    }(_el)?,
                    "implementation-information" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 29 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "implementation-information",
                            ));
                        }
                        Ok(_validate_Implementation_data(&el)?)
                    }(_el)?,
                    "user-information" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 30 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "user-information",
                            ));
                        }
                        Ok(_validate_Association_informationBindErr(&el)?)
                    }(_el)?,
                    _ => {
                        return Err(_el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "AAREerr-apdu",
                        ))
                    }
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Association-informationBindErr{APPLICATION-CONTEXT:Protocols}  ::=
///   SEQUENCE SIZE (1) OF
///     EXTERNAL (
///       WITH COMPONENTS {
///         identification         (WITH COMPONENTS {..., syntax ABSENT}),
///         data-value-descriptor  ABSENT,
///         data-value             (CONTAINING TheOsiBindErr{{Protocols}})})
/// ```
pub type Association_informationBindErr = Vec<EXTERNAL>; // SequenceOfType

pub fn _decode_Association_informationBindErr(
    el: &X690Element,
) -> ASN1Result<Association_informationBindErr> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "Association-informationBindErr",
            ))
        }
    };
    let mut items: SEQUENCE_OF<EXTERNAL> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(BER.decode_external(el)?);
    }
    Ok(items)
}

pub fn _encode_Association_informationBindErr(
    value_: &Association_informationBindErr,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(BER.encode_external(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Association_informationBindErr(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                BER.validate_external(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "Association-informationBindErr",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TheOsiBindErr{APPLICATION-CONTEXT:Protocols}  ::=
///   [18]  APPLICATION-CONTEXT.&bind-operation.&Errors.&ParameterType ({Protocols})
/// ```
pub type TheOsiBindErr = X690Element; // ObjectClassFieldType

pub fn _decode_TheOsiBindErr(el: &X690Element) -> ASN1Result<TheOsiBindErr> {
    |el: &X690Element| -> ASN1Result<TheOsiBindErr> { Ok(x690_identity(&el.inner()?)?) }(&el)
}

pub fn _encode_TheOsiBindErr(value_: &TheOsiBindErr) -> ASN1Result<X690Element> {
    |v_1: &TheOsiBindErr| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 18),
            X690Value::from_explicit(&x690_identity(&v_1)?),
        ))
    }(&value_)
}

pub fn _validate_TheOsiBindErr(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 18 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TheOsiBindErr"));
        }
        Ok(BER.validate_any(&el.inner()?)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiUnbind  ::=  CHOICE {
///   fully-encoded-data
///     [APPLICATION 1] IMPLICIT SEQUENCE SIZE (1) OF SEQUENCE {
///        presentation-context-identifier  Presentation-context-identifier,
///        presentation-data-values     CHOICE {
///          single-ASN1-type        [0]  ABSTRACT-SYNTAX.&Type(TheOsiUnbind)}}}
/// ```
#[derive(Debug, Clone)]
pub enum OsiUnbind {
    fully_encoded_data(Vec<OsiUnbind_fully_encoded_data_Item>),
}

impl TryFrom<&X690Element> for OsiUnbind {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiUnbind(el)
    }
}

pub fn _decode_OsiUnbind(el: &X690Element) -> ASN1Result<OsiUnbind> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => Ok(OsiUnbind::fully_encoded_data(
            |el: &X690Element| -> ASN1Result<SEQUENCE_OF<OsiUnbind_fully_encoded_data_Item>> {
                let elements = match &el.value {
                    X690Value::Constructed(children) => children,
                    _ => {
                        return Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "fully-encoded-data",
                        ))
                    }
                };
                let mut items: SEQUENCE_OF<OsiUnbind_fully_encoded_data_Item> =
                    Vec::with_capacity(elements.len());
                for el in elements.iter() {
                    items.push(_decode_OsiUnbind_fully_encoded_data_Item(el)?);
                }
                Ok(items)
            }(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiUnbind",
            ))
        }
    }
}

pub fn _encode_OsiUnbind(value_: &OsiUnbind) -> ASN1Result<X690Element> {
    match value_ {
        OsiUnbind::fully_encoded_data(v) => {
            |v_1: &Vec<OsiUnbind_fully_encoded_data_Item>| -> ASN1Result<X690Element> {
                let mut el_1 = |value_: &SEQUENCE_OF<OsiUnbind_fully_encoded_data_Item>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_OsiUnbind_fully_encoded_data_Item(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}(&v_1)?;
                el_1.tag.tag_class = TagClass::APPLICATION;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
    }
}

pub fn _validate_OsiUnbind(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fully-encoded-data")
                );
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_OsiUnbind_fully_encoded_data_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "fully-encoded-data",
                    )),
                }
            }(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiUnbind",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TheOsiUnbind ::= [APPLICATION 2] IMPLICIT SEQUENCE {
///   reason  [0] IMPLICIT Release-request-reason OPTIONAL}
/// ```
///
#[derive(Debug, Clone)]
pub struct TheOsiUnbind {
    pub reason: OPTIONAL<Release_request_reason>,
}
impl TheOsiUnbind {
    pub fn new(reason: OPTIONAL<Release_request_reason>) -> Self {
        TheOsiUnbind { reason }
    }
}
impl Default for TheOsiUnbind {
    fn default() -> Self {
        TheOsiUnbind { reason: None }
    }
}
impl TryFrom<&X690Element> for TheOsiUnbind {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TheOsiUnbind(el)
    }
}

pub const _rctl1_components_for_TheOsiUnbind: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "reason",
    true,
    TagSelector::tag((TagClass::CONTEXT, 0)),
    None,
    None,
)];

pub const _rctl2_components_for_TheOsiUnbind: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TheOsiUnbind: &[ComponentSpec; 0] = &[];

pub fn _decode_TheOsiUnbind(el: &X690Element) -> ASN1Result<TheOsiUnbind> {
    |el: &X690Element| -> ASN1Result<TheOsiUnbind> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TheOsiUnbind")
                )
            }
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_TheOsiUnbind,
            _eal_components_for_TheOsiUnbind,
            _rctl2_components_for_TheOsiUnbind,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut reason_: OPTIONAL<Release_request_reason> = None;
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "reason" => reason_ = Some(_decode_Release_request_reason(_el)?),
                _ => {
                    return Err(
                        _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TheOsiUnbind")
                    )
                }
            }
        }
        Ok(TheOsiUnbind { reason: reason_ })
    }(&el)
}

pub fn _encode_TheOsiUnbind(value_: &TheOsiUnbind) -> ASN1Result<X690Element> {
    |v_1: &TheOsiUnbind| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &TheOsiUnbind| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(6);
            if let Some(v_) = &value_.reason {
                components_.push(|v_1: &Release_request_reason| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_Release_request_reason(&v_1)?;
                    el_1.tag.tag_class = TagClass::CONTEXT;
                    el_1.tag.tag_number = 0;
                    Ok(el_1)
                }(&v_)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                X690Value::Constructed(Arc::new(components_)),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::APPLICATION;
        el_1.tag.tag_number = 2;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_TheOsiUnbind(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 2 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TheOsiUnbind"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TheOsiUnbind")
                    )
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_TheOsiUnbind,
                _eal_components_for_TheOsiUnbind,
                _rctl2_components_for_TheOsiUnbind,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "reason" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "reason")
                            );
                        }
                        Ok(_validate_Release_request_reason(&el)?)
                    }(_el)?,
                    _ => {
                        return Err(_el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "TheOsiUnbind",
                        ))
                    }
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Release-request-reason  ::=  INTEGER {normal(0)}
/// ```
pub type Release_request_reason = i8;

pub const Release_request_reason_normal: Release_request_reason = 0; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Release_request_reason(el: &X690Element) -> ASN1Result<Release_request_reason> {
    BER.decode_i8(el)
}

pub fn _encode_Release_request_reason(value_: &Release_request_reason) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Release_request_reason(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiUnbindResult  ::=  CHOICE {
///   fully-encoded-data  [APPLICATION 1] IMPLICIT SEQUENCE SIZE (1) OF SEQUENCE {
///     presentation-context-identifier     Presentation-context-identifier,
///     presentation-data-values            CHOICE {
///       single-ASN1-type               [0]  ABSTRACT-SYNTAX.&Type(TheOsiUnbindRes)}}}
/// ```
#[derive(Debug, Clone)]
pub enum OsiUnbindResult {
    fully_encoded_data(Vec<OsiUnbindResult_fully_encoded_data_Item>),
}

impl TryFrom<&X690Element> for OsiUnbindResult {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiUnbindResult(el)
    }
}

pub fn _decode_OsiUnbindResult(el: &X690Element) -> ASN1Result<OsiUnbindResult> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => Ok(
            OsiUnbindResult::fully_encoded_data(|el: &X690Element| -> ASN1Result<
                SEQUENCE_OF<OsiUnbindResult_fully_encoded_data_Item>,
            > {
                let elements = match &el.value {
                    X690Value::Constructed(children) => children,
                    _ => {
                        return Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "fully-encoded-data",
                        ))
                    }
                };
                let mut items: SEQUENCE_OF<OsiUnbindResult_fully_encoded_data_Item> =
                    Vec::with_capacity(elements.len());
                for el in elements.iter() {
                    items.push(_decode_OsiUnbindResult_fully_encoded_data_Item(el)?);
                }
                Ok(items)
            }(&el)?),
        ),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiUnbindResult",
            ))
        }
    }
}

pub fn _encode_OsiUnbindResult(value_: &OsiUnbindResult) -> ASN1Result<X690Element> {
    match value_ {
        OsiUnbindResult::fully_encoded_data(v) => {
            |v_1: &Vec<OsiUnbindResult_fully_encoded_data_Item>| -> ASN1Result<X690Element> {
                let mut el_1 = |value_: &SEQUENCE_OF<OsiUnbindResult_fully_encoded_data_Item>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_OsiUnbindResult_fully_encoded_data_Item(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}(&v_1)?;
                el_1.tag.tag_class = TagClass::APPLICATION;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
    }
}

pub fn _validate_OsiUnbindResult(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fully-encoded-data")
                );
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_OsiUnbindResult_fully_encoded_data_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "fully-encoded-data",
                    )),
                }
            }(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiUnbindResult",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TheOsiUnbindRes ::= [APPLICATION 3] IMPLICIT SEQUENCE {
///   reason  [0] IMPLICIT Release-response-reason OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct TheOsiUnbindRes {
    pub reason: OPTIONAL<Release_response_reason>,
}
impl TheOsiUnbindRes {
    pub fn new(reason: OPTIONAL<Release_response_reason>) -> Self {
        TheOsiUnbindRes { reason }
    }
}
impl Default for TheOsiUnbindRes {
    fn default() -> Self {
        TheOsiUnbindRes { reason: None }
    }
}
impl TryFrom<&X690Element> for TheOsiUnbindRes {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TheOsiUnbindRes(el)
    }
}

pub const _rctl1_components_for_TheOsiUnbindRes: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "reason",
    true,
    TagSelector::tag((TagClass::CONTEXT, 0)),
    None,
    None,
)];

pub const _rctl2_components_for_TheOsiUnbindRes: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TheOsiUnbindRes: &[ComponentSpec; 0] = &[];

pub fn _decode_TheOsiUnbindRes(el: &X690Element) -> ASN1Result<TheOsiUnbindRes> {
    |el: &X690Element| -> ASN1Result<TheOsiUnbindRes> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TheOsiUnbindRes")
                )
            }
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_TheOsiUnbindRes,
            _eal_components_for_TheOsiUnbindRes,
            _rctl2_components_for_TheOsiUnbindRes,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut reason_: OPTIONAL<Release_response_reason> = None;
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "reason" => reason_ = Some(_decode_Release_response_reason(_el)?),
                _ => {
                    return Err(_el
                        .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TheOsiUnbindRes"))
                }
            }
        }
        Ok(TheOsiUnbindRes { reason: reason_ })
    }(&el)
}

pub fn _encode_TheOsiUnbindRes(value_: &TheOsiUnbindRes) -> ASN1Result<X690Element> {
    |v_1: &TheOsiUnbindRes| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &TheOsiUnbindRes| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(6);
            if let Some(v_) = &value_.reason {
                components_.push(|v_1: &Release_response_reason| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_Release_response_reason(&v_1)?;
                    el_1.tag.tag_class = TagClass::CONTEXT;
                    el_1.tag.tag_number = 0;
                    Ok(el_1)
                }(&v_)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                X690Value::Constructed(Arc::new(components_)),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::APPLICATION;
        el_1.tag.tag_number = 3;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_TheOsiUnbindRes(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 3 {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TheOsiUnbindRes")
            );
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(el
                        .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TheOsiUnbindRes"))
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_TheOsiUnbindRes,
                _eal_components_for_TheOsiUnbindRes,
                _rctl2_components_for_TheOsiUnbindRes,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "reason" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "reason")
                            );
                        }
                        Ok(_validate_Release_response_reason(&el)?)
                    }(_el)?,
                    _ => {
                        return Err(_el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "TheOsiUnbindRes",
                        ))
                    }
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Release-response-reason  ::=  INTEGER {normal(0)}
/// ```
pub type Release_response_reason = i8;

pub const Release_response_reason_normal: Release_response_reason = 0; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Release_response_reason(el: &X690Element) -> ASN1Result<Release_response_reason> {
    BER.decode_i8(el)
}

pub fn _encode_Release_response_reason(
    value_: &Release_response_reason,
) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Release_response_reason(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiOperation{OPERATION:Operations}  ::=  CHOICE {
///   fully-encoded-data [APPLICATION 1] IMPLICIT SEQUENCE SIZE (1) OF SEQUENCE {
///     presentation-context-identifier    Presentation-context-identifier,
///     presentation-data-values           CHOICE {
///       single-ASN1-type              [0]
///                      ABSTRACT-SYNTAX.&Type(OsiDirectoryOperation {{Operations}})}}}
/// ```
#[derive(Debug, Clone)]
pub enum OsiOperation {
    fully_encoded_data(Vec<OsiOperation_fully_encoded_data_Item>),
}

impl TryFrom<&X690Element> for OsiOperation {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiOperation(el)
    }
}

pub fn _decode_OsiOperation(el: &X690Element) -> ASN1Result<OsiOperation> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => Ok(OsiOperation::fully_encoded_data(
            |el: &X690Element| -> ASN1Result<SEQUENCE_OF<OsiOperation_fully_encoded_data_Item>> {
                let elements = match &el.value {
                    X690Value::Constructed(children) => children,
                    _ => {
                        return Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "fully-encoded-data",
                        ))
                    }
                };
                let mut items: SEQUENCE_OF<OsiOperation_fully_encoded_data_Item> =
                    Vec::with_capacity(elements.len());
                for el in elements.iter() {
                    items.push(_decode_OsiOperation_fully_encoded_data_Item(el)?);
                }
                Ok(items)
            }(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiOperation",
            ))
        }
    }
}

pub fn _encode_OsiOperation(value_: &OsiOperation) -> ASN1Result<X690Element> {
    match value_ {
        OsiOperation::fully_encoded_data(v) => {
            |v_1: &Vec<OsiOperation_fully_encoded_data_Item>| -> ASN1Result<X690Element> {
                let mut el_1 = |value_: &SEQUENCE_OF<OsiOperation_fully_encoded_data_Item>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_OsiOperation_fully_encoded_data_Item(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}(&v_1)?;
                el_1.tag.tag_class = TagClass::APPLICATION;
                el_1.tag.tag_number = 1;
                Ok(el_1)
            }(&v)
        }
    }
}

pub fn _validate_OsiOperation(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fully-encoded-data")
                );
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_OsiOperation_fully_encoded_data_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "fully-encoded-data",
                    )),
                }
            }(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiOperation",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiDirectoryOperation{OPERATION:Operations}  ::=  CHOICE {
///   request  OsiReq{{Operations}},
///   result   OsiRes{{Operations}},
///   error    OsiErr{{Operations}},
///   reject   OsiRej}
/// ```
#[derive(Debug, Clone)]
pub enum OsiDirectoryOperation {
    request(OsiReq),
    result(OsiRes),
    error(OsiErr),
    reject(OsiRej),
}

impl TryFrom<&X690Element> for OsiDirectoryOperation {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiDirectoryOperation(el)
    }
}

pub fn _decode_OsiDirectoryOperation(el: &X690Element) -> ASN1Result<OsiDirectoryOperation> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 1) => Ok(OsiDirectoryOperation::request(_decode_OsiReq(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(OsiDirectoryOperation::result(_decode_OsiRes(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(OsiDirectoryOperation::error(_decode_OsiErr(&el)?)),
        (TagClass::CONTEXT, 4) => Ok(OsiDirectoryOperation::reject(_decode_OsiRej(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiDirectoryOperation",
            ))
        }
    }
}

pub fn _encode_OsiDirectoryOperation(value_: &OsiDirectoryOperation) -> ASN1Result<X690Element> {
    match value_ {
        OsiDirectoryOperation::request(v) => _encode_OsiReq(&v),
        OsiDirectoryOperation::result(v) => _encode_OsiRes(&v),
        OsiDirectoryOperation::error(v) => _encode_OsiErr(&v),
        OsiDirectoryOperation::reject(v) => _encode_OsiRej(&v),
    }
}

pub fn _validate_OsiDirectoryOperation(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 1) => _validate_OsiReq(&el),
        (TagClass::CONTEXT, 2) => _validate_OsiRes(&el),
        (TagClass::CONTEXT, 3) => _validate_OsiErr(&el),
        (TagClass::CONTEXT, 4) => _validate_OsiRej(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiDirectoryOperation",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiReq{OPERATION:Operations} ::= [1] IMPLICIT SEQUENCE {
///   invokeId  InvokeId,
///   opcode    OPERATION.&operationCode({Operations}),
///   argument  OPERATION.&ArgumentType({Operations}{@opcode}) }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiReq {
    pub invokeId: InvokeId,
    pub opcode: Code,
    pub argument: X690Element,
}
impl OsiReq {
    pub fn new(invokeId: InvokeId, opcode: Code, argument: X690Element) -> Self {
        OsiReq {
            invokeId,
            opcode,
            argument,
        }
    }
}
impl TryFrom<&X690Element> for OsiReq {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiReq(el)
    }
}

pub const _rctl1_components_for_OsiReq: &[ComponentSpec; 3] = &[
    ComponentSpec::new("invokeId", false, TagSelector::any, None, None),
    ComponentSpec::new("opcode", false, TagSelector::any, None, None),
    ComponentSpec::new("argument", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_OsiReq: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiReq: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiReq(el: &X690Element) -> ASN1Result<OsiReq> {
    |el: &X690Element| -> ASN1Result<OsiReq> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiReq")),
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_OsiReq,
            _eal_components_for_OsiReq,
            _rctl2_components_for_OsiReq,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut invokeId_: OPTIONAL<InvokeId> = None;
        let mut opcode_: OPTIONAL<Code> = None;
        let mut argument_: OPTIONAL<X690Element> = None;
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "invokeId" => invokeId_ = Some(_decode_InvokeId(_el)?),
                "opcode" => opcode_ = Some(_decode_Code(_el)?),
                "argument" => argument_ = Some(x690_identity(_el)?),
                _ => {
                    return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiReq"))
                }
            }
        }
        Ok(OsiReq {
            invokeId: invokeId_.unwrap(),
            opcode: opcode_.unwrap(),
            argument: argument_.unwrap(),
        })
    }(&el)
}

pub fn _encode_OsiReq(value_: &OsiReq) -> ASN1Result<X690Element> {
    |v_1: &OsiReq| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &OsiReq| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(8);
            components_.push(_encode_InvokeId(&value_.invokeId)?);
            components_.push(_encode_Code(&value_.opcode)?);
            components_.push(x690_identity(&value_.argument)?);
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                X690Value::Constructed(Arc::new(components_)),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 1;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_OsiReq(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiReq"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiReq"))
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_OsiReq,
                _eal_components_for_OsiReq,
                _rctl2_components_for_OsiReq,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "invokeId" => _validate_InvokeId(_el)?,
                    "opcode" => _validate_Code(_el)?,
                    "argument" => BER.validate_any(_el)?,
                    _ => {
                        return Err(
                            _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiReq")
                        )
                    }
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiRes{OPERATION:Operations} ::= [2] IMPLICIT SEQUENCE {
///   invokeId  InvokeId,
///   result    SEQUENCE {
///     opcode    OPERATION.&operationCode({Operations}),
///     result    OPERATION.&ResultType({Operations}{@.opcode}) }}
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiRes {
    pub invokeId: InvokeId,
    pub result: OsiRes_result,
}
impl OsiRes {
    pub fn new(invokeId: InvokeId, result: OsiRes_result) -> Self {
        OsiRes { invokeId, result }
    }
}
impl TryFrom<&X690Element> for OsiRes {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiRes(el)
    }
}

pub const _rctl1_components_for_OsiRes: &[ComponentSpec; 2] = &[
    ComponentSpec::new("invokeId", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "result",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_OsiRes: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiRes: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiRes(el: &X690Element) -> ASN1Result<OsiRes> {
    |el: &X690Element| -> ASN1Result<OsiRes> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRes")),
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_OsiRes,
            _eal_components_for_OsiRes,
            _rctl2_components_for_OsiRes,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut invokeId_: OPTIONAL<InvokeId> = None;
        let mut result_: OPTIONAL<OsiRes_result> = None;
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "invokeId" => invokeId_ = Some(_decode_InvokeId(_el)?),
                "result" => result_ = Some(_decode_OsiRes_result(_el)?),
                _ => {
                    return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRes"))
                }
            }
        }
        Ok(OsiRes {
            invokeId: invokeId_.unwrap(),
            result: result_.unwrap(),
        })
    }(&el)
}

pub fn _encode_OsiRes(value_: &OsiRes) -> ASN1Result<X690Element> {
    |v_1: &OsiRes| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &OsiRes| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(7);
            components_.push(_encode_InvokeId(&value_.invokeId)?);
            components_.push(_encode_OsiRes_result(&value_.result)?);
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                X690Value::Constructed(Arc::new(components_)),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 2;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_OsiRes(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRes"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRes"))
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_OsiRes,
                _eal_components_for_OsiRes,
                _rctl2_components_for_OsiRes,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "invokeId" => _validate_InvokeId(_el)?,
                    "result" => _validate_OsiRes_result(_el)?,
                    _ => {
                        return Err(
                            _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRes")
                        )
                    }
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiErr{OPERATION:Operations} ::= [3] IMPLICIT SEQUENCE {
///   invokeID  InvokeId,
///   errcode   OPERATION.&Errors.&errorCode({Operations}),
///   error     OPERATION.&Errors.&ParameterType({Operations}{@.errcode}) }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiErr {
    pub invokeID: InvokeId,
    pub errcode: X690Element,
    pub error: X690Element,
}
impl OsiErr {
    pub fn new(invokeID: InvokeId, errcode: X690Element, error: X690Element) -> Self {
        OsiErr {
            invokeID,
            errcode,
            error,
        }
    }
}
impl TryFrom<&X690Element> for OsiErr {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiErr(el)
    }
}

pub const _rctl1_components_for_OsiErr: &[ComponentSpec; 3] = &[
    ComponentSpec::new("invokeID", false, TagSelector::any, None, None),
    ComponentSpec::new("errcode", false, TagSelector::any, None, None),
    ComponentSpec::new("error", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_OsiErr: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiErr: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiErr(el: &X690Element) -> ASN1Result<OsiErr> {
    |el: &X690Element| -> ASN1Result<OsiErr> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiErr")),
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_OsiErr,
            _eal_components_for_OsiErr,
            _rctl2_components_for_OsiErr,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut invokeID_: OPTIONAL<InvokeId> = None;
        let mut errcode_: OPTIONAL<X690Element> = None;
        let mut error_: OPTIONAL<X690Element> = None;
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "invokeID" => invokeID_ = Some(_decode_InvokeId(_el)?),
                "errcode" => errcode_ = Some(x690_identity(_el)?),
                "error" => error_ = Some(x690_identity(_el)?),
                _ => {
                    return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiErr"))
                }
            }
        }
        Ok(OsiErr {
            invokeID: invokeID_.unwrap(),
            errcode: errcode_.unwrap(),
            error: error_.unwrap(),
        })
    }(&el)
}

pub fn _encode_OsiErr(value_: &OsiErr) -> ASN1Result<X690Element> {
    |v_1: &OsiErr| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &OsiErr| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(8);
            components_.push(_encode_InvokeId(&value_.invokeID)?);
            components_.push(x690_identity(&value_.errcode)?);
            components_.push(x690_identity(&value_.error)?);
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                X690Value::Constructed(Arc::new(components_)),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 3;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_OsiErr(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiErr"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiErr"))
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_OsiErr,
                _eal_components_for_OsiErr,
                _rctl2_components_for_OsiErr,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "invokeID" => _validate_InvokeId(_el)?,
                    "errcode" => BER.validate_any(_el)?,
                    "error" => BER.validate_any(_el)?,
                    _ => {
                        return Err(
                            _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiErr")
                        )
                    }
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiRej ::= [4] IMPLICIT SEQUENCE {
///   invokeId          InvokeId,
///   problem           CHOICE {
///     general      [0]  IMPLICIT GeneralProblem,
///     invoke       [1]  IMPLICIT InvokeProblem,
///     returnResult [2]  IMPLICIT ReturnResultProblem,
///     returnError  [3]  IMPLICIT ReturnErrorProblem,
///     ... },
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiRej {
    pub invokeId: InvokeId,
    pub problem: OsiRej_problem,
    pub _unrecognized: Vec<X690Element>,
}
impl OsiRej {
    pub fn new(
        invokeId: InvokeId,
        problem: OsiRej_problem,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        OsiRej {
            invokeId,
            problem,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for OsiRej {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiRej(el)
    }
}

pub const _rctl1_components_for_OsiRej: &[ComponentSpec; 2] = &[
    ComponentSpec::new("invokeId", false, TagSelector::any, None, None),
    ComponentSpec::new("problem", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_OsiRej: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiRej: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiRej(el: &X690Element) -> ASN1Result<OsiRej> {
    |el: &X690Element| -> ASN1Result<OsiRej> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRej")),
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_OsiRej,
            _eal_components_for_OsiRej,
            _rctl2_components_for_OsiRej,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut invokeId_: OPTIONAL<InvokeId> = None;
        let mut problem_: OPTIONAL<OsiRej_problem> = None;
        let mut _unrecognized: Vec<X690Element> = vec![];
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "invokeId" => invokeId_ = Some(_decode_InvokeId(_el)?),
                "problem" => problem_ = Some(_decode_OsiRej_problem(_el)?),
                _ => _unrecognized.push(_el.clone()),
            }
        }
        Ok(OsiRej {
            invokeId: invokeId_.unwrap(),
            problem: problem_.unwrap(),
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_OsiRej(value_: &OsiRej) -> ASN1Result<X690Element> {
    |v_1: &OsiRej| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &OsiRej| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(12);
            components_.push(_encode_InvokeId(&value_.invokeId)?);
            components_.push(_encode_OsiRej_problem(&value_.problem)?);
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                X690Value::Constructed(Arc::new(
                    [components_, value_._unrecognized.clone()].concat(),
                )),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 4;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_OsiRej(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRej"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRej"))
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_OsiRej,
                _eal_components_for_OsiRej,
                _rctl2_components_for_OsiRej,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "invokeId" => _validate_InvokeId(_el)?,
                    "problem" => _validate_OsiRej_problem(_el)?,
                    _ => (),
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// GeneralProblem  ::=  INTEGER {
///   unrecognizedPDU          (0),
///   mistypedPDU              (1),
///   badlyStructuredPDU       (2) }
/// ```
pub type GeneralProblem = i8;

pub const GeneralProblem_unrecognizedPDU: GeneralProblem = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const GeneralProblem_mistypedPDU: GeneralProblem = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const GeneralProblem_badlyStructuredPDU: GeneralProblem = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_GeneralProblem(el: &X690Element) -> ASN1Result<GeneralProblem> {
    BER.decode_i8(el)
}

pub fn _encode_GeneralProblem(value_: &GeneralProblem) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_GeneralProblem(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InvokeProblem  ::=  INTEGER {
///   duplicateInvocation      (0),
///   unrecognizedOperation    (1),
///   mistypedArgument         (2),
///   resourceLimitation       (3),
///   releaseInProgress        (4)}
/// ```
pub type InvokeProblem = i8;

pub const InvokeProblem_duplicateInvocation: InvokeProblem = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const InvokeProblem_unrecognizedOperation: InvokeProblem = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const InvokeProblem_mistypedArgument: InvokeProblem = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const InvokeProblem_resourceLimitation: InvokeProblem = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const InvokeProblem_releaseInProgress: InvokeProblem = 4; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_InvokeProblem(el: &X690Element) -> ASN1Result<InvokeProblem> {
    BER.decode_i8(el)
}

pub fn _encode_InvokeProblem(value_: &InvokeProblem) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_InvokeProblem(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReturnResultProblem  ::=  INTEGER {
///   unrecognizedInvocation   (0),
///   resultResponseUnexpected (1),
///   mistypedResult           (2)}
/// ```
pub type ReturnResultProblem = i8;

pub const ReturnResultProblem_unrecognizedInvocation: ReturnResultProblem = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const ReturnResultProblem_resultResponseUnexpected: ReturnResultProblem = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const ReturnResultProblem_mistypedResult: ReturnResultProblem = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ReturnResultProblem(el: &X690Element) -> ASN1Result<ReturnResultProblem> {
    BER.decode_i8(el)
}

pub fn _encode_ReturnResultProblem(value_: &ReturnResultProblem) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_ReturnResultProblem(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ReturnErrorProblem  ::=  INTEGER {
///   unrecognizedInvocation   (0),
///   errorResponseUnexpected  (1),
///   unrecognizedError        (2),
///   unexpectedError          (3),
///   mistypedParameter        (4)}
/// ```
pub type ReturnErrorProblem = i8;

pub const ReturnErrorProblem_unrecognizedInvocation: ReturnErrorProblem = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const ReturnErrorProblem_errorResponseUnexpected: ReturnErrorProblem = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const ReturnErrorProblem_unrecognizedError: ReturnErrorProblem = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const ReturnErrorProblem_unexpectedError: ReturnErrorProblem = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const ReturnErrorProblem_mistypedParameter: ReturnErrorProblem = 4; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ReturnErrorProblem(el: &X690Element) -> ASN1Result<ReturnErrorProblem> {
    BER.decode_i8(el)
}

pub fn _encode_ReturnErrorProblem(value_: &ReturnErrorProblem) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_ReturnErrorProblem(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PresentationAbort  ::=  CHOICE {
///   aru-ppdu  ARU-PPDU,
///   arp-ppdu  ARP-PPDU }
/// ```
#[derive(Debug, Clone)]
pub enum PresentationAbort {
    aru_ppdu(ARU_PPDU),
    arp_ppdu(ARP_PPDU),
}

impl TryFrom<&X690Element> for PresentationAbort {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PresentationAbort(el)
    }
}

pub fn _decode_PresentationAbort(el: &X690Element) -> ASN1Result<PresentationAbort> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(PresentationAbort::aru_ppdu(_decode_ARU_PPDU(&el)?)),
        (TagClass::UNIVERSAL, 16) => Ok(PresentationAbort::arp_ppdu(_decode_ARP_PPDU(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "PresentationAbort",
            ))
        }
    }
}

pub fn _encode_PresentationAbort(value_: &PresentationAbort) -> ASN1Result<X690Element> {
    match value_ {
        PresentationAbort::aru_ppdu(v) => _encode_ARU_PPDU(&v),
        PresentationAbort::arp_ppdu(v) => _encode_ARP_PPDU(&v),
    }
}

pub fn _validate_PresentationAbort(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => _validate_ARU_PPDU(&el),
        (TagClass::UNIVERSAL, 16) => _validate_ARP_PPDU(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "PresentationAbort",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ARU-PPDU  ::=  CHOICE {
///   normal-mode-parameters     [0] IMPLICIT SEQUENCE {
///     presentation-context-identifier-list
///                                     [0] IMPLICIT Presentation-context-identifier-list,
///     user-data                           CHOICE {
///       fully-encoded-data [APPLICATION 1]  IMPLICIT SEQUENCE SIZE(1..MAX) OF SEQUENCE {
///         presentation-context-identifier     Presentation-context-identifier,
///         presentation-data-values            CHOICE {
///           single-ASN1-type               [0]  ABSTRACT-SYNTAX.&Type(ABRT-apdu)}}}}}
/// ```
#[derive(Debug, Clone)]
pub enum ARU_PPDU {
    normal_mode_parameters(ARU_PPDU_normal_mode_parameters),
}

impl TryFrom<&X690Element> for ARU_PPDU {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ARU_PPDU(el)
    }
}

pub fn _decode_ARU_PPDU(el: &X690Element) -> ASN1Result<ARU_PPDU> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(ARU_PPDU::normal_mode_parameters(
            _decode_ARU_PPDU_normal_mode_parameters(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "ARU-PPDU",
            ))
        }
    }
}

pub fn _encode_ARU_PPDU(value_: &ARU_PPDU) -> ASN1Result<X690Element> {
    match value_ {
        ARU_PPDU::normal_mode_parameters(v) => {
            |v_1: &ARU_PPDU_normal_mode_parameters| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ARU_PPDU_normal_mode_parameters(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&v)
        }
    }
}

pub fn _validate_ARU_PPDU(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "normal-mode-parameters",
                ));
            }
            Ok(_validate_ARU_PPDU_normal_mode_parameters(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "ARU-PPDU",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Presentation-context-identifier-list  ::=  SEQUENCE SIZE (1) OF SEQUENCE {
///   presentation-context-identifier  Presentation-context-identifier,
///   transfer-syntax-name             Transfer-syntax-name}
/// ```
pub type Presentation_context_identifier_list = Vec<Presentation_context_identifier_list_Item>; // SequenceOfType

pub fn _decode_Presentation_context_identifier_list(
    el: &X690Element,
) -> ASN1Result<Presentation_context_identifier_list> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "Presentation-context-identifier-list",
            ))
        }
    };
    let mut items: SEQUENCE_OF<Presentation_context_identifier_list_Item> =
        Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_Presentation_context_identifier_list_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_Presentation_context_identifier_list(
    value_: &Presentation_context_identifier_list,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_Presentation_context_identifier_list_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_Presentation_context_identifier_list(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_Presentation_context_identifier_list_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "Presentation-context-identifier-list",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ABRT-apdu ::= [APPLICATION 4] IMPLICIT SEQUENCE {
///   abort-source  [0] IMPLICIT ABRT-source }
/// ```
///
#[derive(Debug, Clone)]
pub struct ABRT_apdu {
    pub abort_source: ABRT_source,
}
impl ABRT_apdu {
    pub fn new(abort_source: ABRT_source) -> Self {
        ABRT_apdu { abort_source }
    }
}
impl TryFrom<&X690Element> for ABRT_apdu {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ABRT_apdu(el)
    }
}

pub const _rctl1_components_for_ABRT_apdu: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "abort-source",
    false,
    TagSelector::tag((TagClass::CONTEXT, 0)),
    None,
    None,
)];

pub const _rctl2_components_for_ABRT_apdu: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ABRT_apdu: &[ComponentSpec; 0] = &[];

pub fn _decode_ABRT_apdu(el: &X690Element) -> ASN1Result<ABRT_apdu> {
    |el: &X690Element| -> ASN1Result<ABRT_apdu> {
        let _elements = match &el.value {
            X690Value::Constructed(children) => children,
            _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ABRT-apdu")),
        };
        let _seq_iter = X690StructureIterator::new(
            _elements.as_slice(),
            _rctl1_components_for_ABRT_apdu,
            _eal_components_for_ABRT_apdu,
            _rctl2_components_for_ABRT_apdu,
        )
        .into_iter();
        let mut _i: usize = 0;
        let mut abort_source_: OPTIONAL<ABRT_source> = None;
        for _fallible_component_name in _seq_iter {
            let _component_name = _fallible_component_name?;
            let _maybe_el = _elements.get(_i);
            _i += 1;
            let _el = _maybe_el.unwrap();
            match _component_name {
                "abort-source" => abort_source_ = Some(_decode_ABRT_source(_el)?),
                _ => {
                    return Err(
                        _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ABRT-apdu")
                    )
                }
            }
        }
        Ok(ABRT_apdu {
            abort_source: abort_source_.unwrap(),
        })
    }(&el)
}

pub fn _encode_ABRT_apdu(value_: &ABRT_apdu) -> ASN1Result<X690Element> {
    |v_1: &ABRT_apdu| -> ASN1Result<X690Element> {
        let mut el_1 = |value_: &ABRT_apdu| -> ASN1Result<X690Element> {
            let mut components_: Vec<X690Element> = Vec::with_capacity(6);
            components_.push(|v_1: &ABRT_source| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_ABRT_source(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 0;
                Ok(el_1)
            }(&value_.abort_source)?);
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                X690Value::Constructed(Arc::new(components_)),
            ))
        }(&v_1)?;
        el_1.tag.tag_class = TagClass::APPLICATION;
        el_1.tag.tag_number = 4;
        Ok(el_1)
    }(&value_)
}

pub fn _validate_ABRT_apdu(el: &X690Element) -> ASN1Result<()> {
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 4 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ABRT-apdu"));
        }
        Ok(|el: &X690Element| -> ASN1Result<()> {
            let _elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ABRT-apdu")
                    )
                }
            };
            let _seq_iter = X690StructureIterator::new(
                _elements.as_slice(),
                _rctl1_components_for_ABRT_apdu,
                _eal_components_for_ABRT_apdu,
                _rctl2_components_for_ABRT_apdu,
            )
            .into_iter();
            let mut _i: usize = 0;
            for _fallible_component_name in _seq_iter {
                let _component_name = _fallible_component_name?;
                let _maybe_el = _elements.get(_i);
                _i += 1;
                let _el = _maybe_el.unwrap();
                match _component_name {
                    "abort-source" => |el: &X690Element| -> ASN1Result<()> {
                        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "abort-source",
                            ));
                        }
                        Ok(_validate_ABRT_source(&el)?)
                    }(_el)?,
                    _ => {
                        return Err(
                            _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ABRT-apdu")
                        )
                    }
                }
            }
            Ok(())
        }(&el)?)
    }(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ABRT-source  ::=  INTEGER {
///   acse-service-user     (0),
///   acse-service-provider (1) }
/// ```
pub type ABRT_source = i8;

pub const ABRT_source_acse_service_user: ABRT_source = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const ABRT_source_acse_service_provider: ABRT_source = 1; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_ABRT_source(el: &X690Element) -> ASN1Result<ABRT_source> {
    BER.decode_i8(el)
}

pub fn _encode_ABRT_source(value_: &ABRT_source) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_ABRT_source(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ARP-PPDU ::= SEQUENCE {
///   provider-reason   [0] IMPLICIT Abort-reason OPTIONAL,
///   event-identifier  [1] IMPLICIT Event-identifier OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct ARP_PPDU {
    pub provider_reason: OPTIONAL<Abort_reason>,
    pub event_identifier: OPTIONAL<Event_identifier>,
}
impl ARP_PPDU {
    pub fn new(
        provider_reason: OPTIONAL<Abort_reason>,
        event_identifier: OPTIONAL<Event_identifier>,
    ) -> Self {
        ARP_PPDU {
            provider_reason,
            event_identifier,
        }
    }
}
impl Default for ARP_PPDU {
    fn default() -> Self {
        ARP_PPDU {
            provider_reason: None,
            event_identifier: None,
        }
    }
}
impl TryFrom<&X690Element> for ARP_PPDU {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ARP_PPDU(el)
    }
}

pub const _rctl1_components_for_ARP_PPDU: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "provider-reason",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "event-identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ARP_PPDU: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ARP_PPDU: &[ComponentSpec; 0] = &[];

pub fn _decode_ARP_PPDU(el: &X690Element) -> ASN1Result<ARP_PPDU> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ARP-PPDU")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ARP_PPDU,
        _eal_components_for_ARP_PPDU,
        _rctl2_components_for_ARP_PPDU,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut provider_reason_: OPTIONAL<Abort_reason> = None;
    let mut event_identifier_: OPTIONAL<Event_identifier> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "provider-reason" => provider_reason_ = Some(_decode_Abort_reason(_el)?),
            "event-identifier" => event_identifier_ = Some(_decode_Event_identifier(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ARP-PPDU")),
        }
    }
    Ok(ARP_PPDU {
        provider_reason: provider_reason_,
        event_identifier: event_identifier_,
    })
}

pub fn _encode_ARP_PPDU(value_: &ARP_PPDU) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    if let Some(v_) = &value_.provider_reason {
        components_.push(|v_1: &Abort_reason| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Abort_reason(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.event_identifier {
        components_.push(|v_1: &Event_identifier| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Event_identifier(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ARP_PPDU(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ARP-PPDU")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ARP_PPDU,
        _eal_components_for_ARP_PPDU,
        _rctl2_components_for_ARP_PPDU,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "provider-reason" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "provider-reason",
                    ));
                }
                Ok(_validate_Abort_reason(&el)?)
            }(_el)?,
            "event-identifier" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "event-identifier",
                    ));
                }
                Ok(_validate_Event_identifier(&el)?)
            }(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ARP-PPDU")),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Abort-reason  ::=  INTEGER {
///   reason-not-specified                 (0),
///   unrecognized-ppdu                    (1),
///   unexpected-ppdu                      (2),
///   unexpected-session-service-primitive (3),
///   unrecognized-ppdu-parameter          (4),
///   unexpected-ppdu-parameter            (5),
///   invalid-ppdu-parameter-value         (6)}
/// ```
pub type Abort_reason = i8;

pub const Abort_reason_reason_not_specified: Abort_reason = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const Abort_reason_unrecognized_ppdu: Abort_reason = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Abort_reason_unexpected_ppdu: Abort_reason = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const Abort_reason_unexpected_session_service_primitive: Abort_reason = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const Abort_reason_unrecognized_ppdu_parameter: Abort_reason = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const Abort_reason_unexpected_ppdu_parameter: Abort_reason = 5; /* LONG_NAMED_INTEGER_VALUE */

pub const Abort_reason_invalid_ppdu_parameter_value: Abort_reason = 6; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Abort_reason(el: &X690Element) -> ASN1Result<Abort_reason> {
    BER.decode_i8(el)
}

pub fn _encode_Abort_reason(value_: &Abort_reason) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Abort_reason(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Event-identifier  ::=  INTEGER {
///   cp-PPDU              (0),
///   cpa-PPDU             (1),
///   cpr-PPDU             (2),
///   aru-PPDU             (3),
///   arp-PPDU             (4),
///   td-PPDU              (7),
///   s-release-indication (14),
///   s-release-confirm    (15) }
/// ```
pub type Event_identifier = i8;

pub const Event_identifier_cp_PPDU: Event_identifier = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const Event_identifier_cpa_PPDU: Event_identifier = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Event_identifier_cpr_PPDU: Event_identifier = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const Event_identifier_aru_PPDU: Event_identifier = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const Event_identifier_arp_PPDU: Event_identifier = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const Event_identifier_td_PPDU: Event_identifier = 7; /* LONG_NAMED_INTEGER_VALUE */

pub const Event_identifier_s_release_indication: Event_identifier = 14; /* LONG_NAMED_INTEGER_VALUE */

pub const Event_identifier_s_release_confirm: Event_identifier = 15; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Event_identifier(el: &X690Element) -> ASN1Result<Event_identifier> {
    BER.decode_i8(el)
}

pub fn _encode_Event_identifier(value_: &Event_identifier) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Event_identifier(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBind-mode-selector ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiBind_mode_selector {
    pub mode_value: INTEGER,
}
impl OsiBind_mode_selector {
    pub fn new(mode_value: INTEGER) -> Self {
        OsiBind_mode_selector { mode_value }
    }
}
impl TryFrom<&X690Element> for OsiBind_mode_selector {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBind_mode_selector(el)
    }
}

pub const _rctl1_components_for_OsiBind_mode_selector: &[ComponentSpec; 1] = &[ComponentSpec::new(
    "mode-value",
    false,
    TagSelector::tag((TagClass::CONTEXT, 0)),
    None,
    None,
)];

pub const _rctl2_components_for_OsiBind_mode_selector: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiBind_mode_selector: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiBind_mode_selector(el: &X690Element) -> ASN1Result<OsiBind_mode_selector> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBind-mode-selector")
            )
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_OsiBind_mode_selector,
        _eal_components_for_OsiBind_mode_selector,
        _rctl2_components_for_OsiBind_mode_selector,
        10,
    )?;
    let mode_value_ = BER.decode_integer(_components.get("mode-value").unwrap())?;
    Ok(OsiBind_mode_selector {
        mode_value: mode_value_,
    })
}

pub fn _encode_OsiBind_mode_selector(value_: &OsiBind_mode_selector) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(6);
    components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
        let mut el_1 = BER.encode_integer(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_.mode_value)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiBind_mode_selector(el: &X690Element) -> ASN1Result<()> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBind-mode-selector")
            )
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_OsiBind_mode_selector,
        _eal_components_for_OsiBind_mode_selector,
        _rctl2_components_for_OsiBind_mode_selector,
        10,
    )?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "mode-value"));
        }
        Ok(BER.validate_integer(&el)?)
    }(_components.get("mode-value").unwrap())?;
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBind-normal-mode-parameters-protocol-version ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type OsiBind_normal_mode_parameters_protocol_version = BIT_STRING;

pub const OsiBind_normal_mode_parameters_protocol_version_version_1: BIT = 0; /* LONG_NAMED_BIT */

pub fn _decode_OsiBind_normal_mode_parameters_protocol_version(
    el: &X690Element,
) -> ASN1Result<OsiBind_normal_mode_parameters_protocol_version> {
    BER.decode_bit_string(&el)
}

pub fn _encode_OsiBind_normal_mode_parameters_protocol_version(
    value_: &OsiBind_normal_mode_parameters_protocol_version,
) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_OsiBind_normal_mode_parameters_protocol_version(
    el: &X690Element,
) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBind-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values {
    single_ASN1_type(X690Element),
}

impl TryFrom<&X690Element>
    for OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values
{
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(el)
    }
}

pub fn _decode_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<
    OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
> {
    match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => Ok(OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(|el: &X690Element| -> ASN1Result<X690Element> {
	Ok(x690_identity(&el.inner()?)?)
}(&el)?)),
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice, "OsiBind-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values")),
	}
}

pub fn _encode_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    value_: &OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
) -> ASN1Result<X690Element> {
    match value_ {
		OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(v) => |v_1: &X690Element| -> ASN1Result<X690Element> { Ok(X690Element::new(Tag::new(TagClass::CONTEXT, 0), X690Value::from_explicit(&x690_identity(&v_1)?))) }(&v),
	}
}

pub fn _validate_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "single-ASN1-type"));
	}
	Ok(BER.validate_any(&el.inner()?)?)
}(&el),
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice, "OsiBind-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values")),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBind-normal-mode-parameters-user-data-fully-encoded-data-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item {
    pub transfer_syntax_name: OPTIONAL<Transfer_syntax_name>,
    pub presentation_context_identifier: Presentation_context_identifier,
    pub presentation_data_values:
        OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
}
impl OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item {
    pub fn new(
        transfer_syntax_name: OPTIONAL<Transfer_syntax_name>,
        presentation_context_identifier: Presentation_context_identifier,
        presentation_data_values: OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
    ) -> Self {
        OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item {
            transfer_syntax_name,
            presentation_context_identifier,
            presentation_data_values,
        }
    }
}
impl TryFrom<&X690Element> for OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item(el)
    }
}

pub const _rctl1_components_for_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item:
    &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "transfer-syntax-name",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "presentation-context-identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "presentation-data-values",
        false,
        TagSelector::any,
        None,
        None,
    ),
];

pub const _rctl2_components_for_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item:
    &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item:
    &[ComponentSpec; 0] = &[];

pub fn _decode_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item(
    el: &X690Element,
) -> ASN1Result<OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBind-normal-mode-parameters-user-data-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _eal_components_for_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _rctl2_components_for_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut transfer_syntax_name_: OPTIONAL<Transfer_syntax_name> = None;
    let mut presentation_context_identifier_: OPTIONAL<Presentation_context_identifier> = None;
    let mut presentation_data_values_: OPTIONAL<
        OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
    > = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
			"transfer-syntax-name" => transfer_syntax_name_ = Some(_decode_Transfer_syntax_name(_el)?),
			"presentation-context-identifier" => presentation_context_identifier_ = Some(_decode_Presentation_context_identifier(_el)?),
			"presentation-data-values" => presentation_data_values_ = Some(_decode_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(_el)?),
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBind-normal-mode-parameters-user-data-fully-encoded-data-Item")),
		}
    }
    Ok(
        OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item {
            transfer_syntax_name: transfer_syntax_name_,
            presentation_context_identifier: presentation_context_identifier_.unwrap(),
            presentation_data_values: presentation_data_values_.unwrap(),
        },
    )
}

pub fn _encode_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item(
    value_: &OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    if let Some(v_) = &value_.transfer_syntax_name {
        components_.push(_encode_Transfer_syntax_name(&v_)?);
    }
    components_.push(_encode_Presentation_context_identifier(
        &value_.presentation_context_identifier,
    )?);
    components_.push(_encode_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(&value_.presentation_data_values)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item(
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBind-normal-mode-parameters-user-data-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _eal_components_for_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _rctl2_components_for_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
			"transfer-syntax-name" => _validate_Transfer_syntax_name(_el)?,
			"presentation-context-identifier" => _validate_Presentation_context_identifier(_el)?,
			"presentation-data-values" => _validate_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(_el)?,
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBind-normal-mode-parameters-user-data-fully-encoded-data-Item")),
		}
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBind-normal-mode-parameters-user-data ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum OsiBind_normal_mode_parameters_user_data {
    fully_encoded_data(Vec<OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item>),
}

impl TryFrom<&X690Element> for OsiBind_normal_mode_parameters_user_data {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBind_normal_mode_parameters_user_data(el)
    }
}

pub fn _decode_OsiBind_normal_mode_parameters_user_data(
    el: &X690Element,
) -> ASN1Result<OsiBind_normal_mode_parameters_user_data> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => Ok(
            OsiBind_normal_mode_parameters_user_data::fully_encoded_data(
                |el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item>,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "fully-encoded-data",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<
                        OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item,
                    > = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item(el)?);
                    }
                    Ok(items)
                }(&el)?,
            ),
        ),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiBind-normal-mode-parameters-user-data",
            ))
        }
    }
}

pub fn _encode_OsiBind_normal_mode_parameters_user_data(
    value_: &OsiBind_normal_mode_parameters_user_data,
) -> ASN1Result<X690Element> {
    match value_ {
        OsiBind_normal_mode_parameters_user_data::fully_encoded_data(v) => |v_1: &Vec<
            OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item,
        >|
         -> ASN1Result<
            X690Element,
        > {
            let mut el_1 = |value_: &SEQUENCE_OF<
                OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item,
            >|
             -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(
                        _encode_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item(
                            &v,
                        )?,
                    );
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_1)?;
            el_1.tag.tag_class = TagClass::APPLICATION;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
    }
}

pub fn _validate_OsiBind_normal_mode_parameters_user_data(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fully-encoded-data")
                );
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_OsiBind_normal_mode_parameters_user_data_fully_encoded_data_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "fully-encoded-data",
                    )),
                }
            }(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiBind-normal-mode-parameters-user-data",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBind-normal-mode-parameters ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiBind_normal_mode_parameters {
    pub protocol_version: OPTIONAL<OsiBind_normal_mode_parameters_protocol_version>,
    pub calling_presentation_selector: OPTIONAL<Presentation_selector>,
    pub called_presentation_selector: OPTIONAL<Presentation_selector>,
    pub presentation_context_definition_list: Context_list,
    pub user_data: OsiBind_normal_mode_parameters_user_data,
}
impl OsiBind_normal_mode_parameters {
    pub fn new(
        protocol_version: OPTIONAL<OsiBind_normal_mode_parameters_protocol_version>,
        calling_presentation_selector: OPTIONAL<Presentation_selector>,
        called_presentation_selector: OPTIONAL<Presentation_selector>,
        presentation_context_definition_list: Context_list,
        user_data: OsiBind_normal_mode_parameters_user_data,
    ) -> Self {
        OsiBind_normal_mode_parameters {
            protocol_version,
            calling_presentation_selector,
            called_presentation_selector,
            presentation_context_definition_list,
            user_data,
        }
    }
    pub fn _default_value_for_protocol_version() -> OsiBind_normal_mode_parameters_protocol_version
    {
        BIT_STRING::with_bits_set(&[OsiBind_normal_mode_parameters_protocol_version_version_1])
    }
}
impl TryFrom<&X690Element> for OsiBind_normal_mode_parameters {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBind_normal_mode_parameters(el)
    }
}

pub const _rctl1_components_for_OsiBind_normal_mode_parameters: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "protocol-version",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "calling-presentation-selector",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "called-presentation-selector",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "presentation-context-definition-list",
        false,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new("user-data", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_OsiBind_normal_mode_parameters: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiBind_normal_mode_parameters: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiBind_normal_mode_parameters(
    el: &X690Element,
) -> ASN1Result<OsiBind_normal_mode_parameters> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBind-normal-mode-parameters",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiBind_normal_mode_parameters,
        _eal_components_for_OsiBind_normal_mode_parameters,
        _rctl2_components_for_OsiBind_normal_mode_parameters,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut protocol_version_: OPTIONAL<OsiBind_normal_mode_parameters_protocol_version> = None;
    let mut calling_presentation_selector_: OPTIONAL<Presentation_selector> = None;
    let mut called_presentation_selector_: OPTIONAL<Presentation_selector> = None;
    let mut presentation_context_definition_list_: OPTIONAL<Context_list> = None;
    let mut user_data_: OPTIONAL<OsiBind_normal_mode_parameters_user_data> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protocol-version" => {
                protocol_version_ = Some(_decode_OsiBind_normal_mode_parameters_protocol_version(
                    _el,
                )?)
            }
            "calling-presentation-selector" => {
                calling_presentation_selector_ = Some(_decode_Presentation_selector(_el)?)
            }
            "called-presentation-selector" => {
                called_presentation_selector_ = Some(_decode_Presentation_selector(_el)?)
            }
            "presentation-context-definition-list" => {
                presentation_context_definition_list_ = Some(_decode_Context_list(_el)?)
            }
            "user-data" => {
                user_data_ = Some(_decode_OsiBind_normal_mode_parameters_user_data(_el)?)
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OsiBind-normal-mode-parameters",
                ))
            }
        }
    }
    Ok(OsiBind_normal_mode_parameters {
        protocol_version: protocol_version_,
        calling_presentation_selector: calling_presentation_selector_,
        called_presentation_selector: called_presentation_selector_,
        presentation_context_definition_list: presentation_context_definition_list_.unwrap(),
        user_data: user_data_.unwrap(),
    })
}

pub fn _encode_OsiBind_normal_mode_parameters(
    value_: &OsiBind_normal_mode_parameters,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(10);
    if let Some(v_) = &value_.protocol_version {
        if *v_ != OsiBind_normal_mode_parameters::_default_value_for_protocol_version() {
            components_.push(|v_1: &OsiBind_normal_mode_parameters_protocol_version| -> ASN1Result<X690Element> { let mut el_1 = _encode_OsiBind_normal_mode_parameters_protocol_version(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 0; Ok(el_1) }(&v_)?);
        }
    }
    if let Some(v_) = &value_.calling_presentation_selector {
        components_.push(|v_1: &Presentation_selector| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Presentation_selector(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.called_presentation_selector {
        components_.push(|v_1: &Presentation_selector| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Presentation_selector(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(|v_1: &Context_list| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_Context_list(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 4;
        Ok(el_1)
    }(&value_.presentation_context_definition_list)?);
    components_.push(_encode_OsiBind_normal_mode_parameters_user_data(
        &value_.user_data,
    )?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiBind_normal_mode_parameters(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBind-normal-mode-parameters",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiBind_normal_mode_parameters,
        _eal_components_for_OsiBind_normal_mode_parameters,
        _rctl2_components_for_OsiBind_normal_mode_parameters,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protocol-version" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "protocol-version",
                    ));
                }
                Ok(_validate_OsiBind_normal_mode_parameters_protocol_version(
                    &el,
                )?)
            }(_el)?,
            "calling-presentation-selector" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "calling-presentation-selector",
                    ));
                }
                Ok(_validate_Presentation_selector(&el)?)
            }(_el)?,
            "called-presentation-selector" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "called-presentation-selector",
                    ));
                }
                Ok(_validate_Presentation_selector(&el)?)
            }(_el)?,
            "presentation-context-definition-list" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "presentation-context-definition-list",
                    ));
                }
                Ok(_validate_Context_list(&el)?)
            }(_el)?,
            "user-data" => _validate_OsiBind_normal_mode_parameters_user_data(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OsiBind-normal-mode-parameters",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Context-list-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct Context_list_Item {
    pub presentation_context_identifier: Presentation_context_identifier,
    pub abstract_syntax_name: Abstract_syntax_name,
    pub transfer_syntax_name_list: Vec<Transfer_syntax_name>,
}
impl Context_list_Item {
    pub fn new(
        presentation_context_identifier: Presentation_context_identifier,
        abstract_syntax_name: Abstract_syntax_name,
        transfer_syntax_name_list: Vec<Transfer_syntax_name>,
    ) -> Self {
        Context_list_Item {
            presentation_context_identifier,
            abstract_syntax_name,
            transfer_syntax_name_list,
        }
    }
}
impl TryFrom<&X690Element> for Context_list_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Context_list_Item(el)
    }
}

pub const _rctl1_components_for_Context_list_Item: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "presentation-context-identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "abstract-syntax-name",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "transfer-syntax-name-list",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Context_list_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Context_list_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_Context_list_Item(el: &X690Element) -> ASN1Result<Context_list_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context-list-Item")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Context_list_Item,
        _eal_components_for_Context_list_Item,
        _rctl2_components_for_Context_list_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut presentation_context_identifier_: OPTIONAL<Presentation_context_identifier> = None;
    let mut abstract_syntax_name_: OPTIONAL<Abstract_syntax_name> = None;
    let mut transfer_syntax_name_list_: OPTIONAL<Vec<Transfer_syntax_name>> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier" => {
                presentation_context_identifier_ =
                    Some(_decode_Presentation_context_identifier(_el)?)
            }
            "abstract-syntax-name" => {
                abstract_syntax_name_ = Some(_decode_Abstract_syntax_name(_el)?)
            }
            "transfer-syntax-name-list" => {
                transfer_syntax_name_list_ = Some(|el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<Transfer_syntax_name>,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "transfer-syntax-name-list",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<Transfer_syntax_name> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_Transfer_syntax_name(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context-list-Item")
                )
            }
        }
    }
    Ok(Context_list_Item {
        presentation_context_identifier: presentation_context_identifier_.unwrap(),
        abstract_syntax_name: abstract_syntax_name_.unwrap(),
        transfer_syntax_name_list: transfer_syntax_name_list_.unwrap(),
    })
}

pub fn _encode_Context_list_Item(value_: &Context_list_Item) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(_encode_Presentation_context_identifier(
        &value_.presentation_context_identifier,
    )?);
    components_.push(_encode_Abstract_syntax_name(&value_.abstract_syntax_name)?);
    components_.push(
        |value_: &SEQUENCE_OF<Transfer_syntax_name>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_Transfer_syntax_name(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.transfer_syntax_name_list)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Context_list_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context-list-Item")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Context_list_Item,
        _eal_components_for_Context_list_Item,
        _rctl2_components_for_Context_list_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier" => _validate_Presentation_context_identifier(_el)?,
            "abstract-syntax-name" => _validate_Abstract_syntax_name(_el)?,
            "transfer-syntax-name-list" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_Transfer_syntax_name(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "transfer-syntax-name-list",
                    )),
                }
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context-list-Item")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AARQ-apdu-protocol-version ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type AARQ_apdu_protocol_version = BIT_STRING;

pub const AARQ_apdu_protocol_version_version1: BIT = 0; /* LONG_NAMED_BIT */

pub fn _decode_AARQ_apdu_protocol_version(
    el: &X690Element,
) -> ASN1Result<AARQ_apdu_protocol_version> {
    BER.decode_bit_string(&el)
}

pub fn _encode_AARQ_apdu_protocol_version(
    value_: &AARQ_apdu_protocol_version,
) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_AARQ_apdu_protocol_version(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindResult-mode-selector ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiBindResult_mode_selector {
    pub mode_value: INTEGER,
}
impl OsiBindResult_mode_selector {
    pub fn new(mode_value: INTEGER) -> Self {
        OsiBindResult_mode_selector { mode_value }
    }
}
impl TryFrom<&X690Element> for OsiBindResult_mode_selector {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBindResult_mode_selector(el)
    }
}

pub const _rctl1_components_for_OsiBindResult_mode_selector: &[ComponentSpec; 1] =
    &[ComponentSpec::new(
        "mode-value",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    )];

pub const _rctl2_components_for_OsiBindResult_mode_selector: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiBindResult_mode_selector: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiBindResult_mode_selector(
    el: &X690Element,
) -> ASN1Result<OsiBindResult_mode_selector> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBindResult-mode-selector",
            ))
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_OsiBindResult_mode_selector,
        _eal_components_for_OsiBindResult_mode_selector,
        _rctl2_components_for_OsiBindResult_mode_selector,
        10,
    )?;
    let mode_value_ = BER.decode_integer(_components.get("mode-value").unwrap())?;
    Ok(OsiBindResult_mode_selector {
        mode_value: mode_value_,
    })
}

pub fn _encode_OsiBindResult_mode_selector(
    value_: &OsiBindResult_mode_selector,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(6);
    components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
        let mut el_1 = BER.encode_integer(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_.mode_value)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiBindResult_mode_selector(el: &X690Element) -> ASN1Result<()> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBindResult-mode-selector",
            ))
        }
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_OsiBindResult_mode_selector,
        _eal_components_for_OsiBindResult_mode_selector,
        _rctl2_components_for_OsiBindResult_mode_selector,
        10,
    )?;
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "mode-value"));
        }
        Ok(BER.validate_integer(&el)?)
    }(_components.get("mode-value").unwrap())?;
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindResult-normal-mode-parameters-protocol-version ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type OsiBindResult_normal_mode_parameters_protocol_version = BIT_STRING;

pub const OsiBindResult_normal_mode_parameters_protocol_version_version_1: BIT = 0; /* LONG_NAMED_BIT */

pub fn _decode_OsiBindResult_normal_mode_parameters_protocol_version(
    el: &X690Element,
) -> ASN1Result<OsiBindResult_normal_mode_parameters_protocol_version> {
    BER.decode_bit_string(&el)
}

pub fn _encode_OsiBindResult_normal_mode_parameters_protocol_version(
    value_: &OsiBindResult_normal_mode_parameters_protocol_version,
) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_OsiBindResult_normal_mode_parameters_protocol_version(
    el: &X690Element,
) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindResult-normal-mode-parameters-presentation-context-definition-result-list-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item {
    pub result: Result_,
    pub transfer_syntax_name: Transfer_syntax_name,
}
impl OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item {
    pub fn new(result: Result_, transfer_syntax_name: Transfer_syntax_name) -> Self {
        OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item {
            result,
            transfer_syntax_name,
        }
    }
}
impl TryFrom<&X690Element>
    for OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item
{
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item(el)
    }
}

pub const _rctl1_components_for_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item: &[ComponentSpec; 2] = &[
	ComponentSpec::new("result", false, TagSelector::tag((TagClass::CONTEXT, 0)), None, None),
	ComponentSpec::new("transfer-syntax-name", false, TagSelector::tag((TagClass::CONTEXT, 1)), None, None)
];

pub const _rctl2_components_for_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item: &[ComponentSpec; 0] = &[

];

pub fn _decode_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item(
    el: &X690Element,
) -> ASN1Result<OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item>
{
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "OsiBindResult-normal-mode-parameters-presentation-context-definition-result-list-Item",
        )),
    };
    let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item,
		_eal_components_for_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item,
		_rctl2_components_for_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item,
	).into_iter();
    let mut _i: usize = 0;
    let mut result_: OPTIONAL<Result_> = None;
    let mut transfer_syntax_name_: OPTIONAL<Transfer_syntax_name> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
			"result" => result_ = Some(_decode_Result_(_el)?),
			"transfer-syntax-name" => transfer_syntax_name_ = Some(_decode_Transfer_syntax_name(_el)?),
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBindResult-normal-mode-parameters-presentation-context-definition-result-list-Item")),
		}
    }
    Ok(
        OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item {
            result: result_.unwrap(),
            transfer_syntax_name: transfer_syntax_name_.unwrap(),
        },
    )
}

pub fn _encode_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item(
    value_: &OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(|v_1: &Result_| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_Result_(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_.result)?);
    components_.push(|v_1: &Transfer_syntax_name| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_Transfer_syntax_name(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 1;
        Ok(el_1)
    }(&value_.transfer_syntax_name)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item(
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "OsiBindResult-normal-mode-parameters-presentation-context-definition-result-list-Item",
        )),
    };
    let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item,
		_eal_components_for_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item,
		_rctl2_components_for_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item,
	).into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
			"result" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "result"));
	}
	Ok(_validate_Result_(&el)?)
}(_el)?,
			"transfer-syntax-name" => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "transfer-syntax-name"));
	}
	Ok(_validate_Transfer_syntax_name(&el)?)
}(_el)?,
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBindResult-normal-mode-parameters-presentation-context-definition-result-list-Item")),
		}
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindResult-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values
{
    single_ASN1_type(X690Element),
}

impl TryFrom<&X690Element> for OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(el)
	}
}

pub fn _decode_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<
    OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
> {
    match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => Ok(OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(|el: &X690Element| -> ASN1Result<X690Element> {
	Ok(x690_identity(&el.inner()?)?)
}(&el)?)),
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice, "OsiBindResult-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values")),
	}
}

pub fn _encode_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    value_: &OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
) -> ASN1Result<X690Element> {
    match value_ {
		OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(v) => |v_1: &X690Element| -> ASN1Result<X690Element> { Ok(X690Element::new(Tag::new(TagClass::CONTEXT, 0), X690Value::from_explicit(&x690_identity(&v_1)?))) }(&v),
	}
}

pub fn _validate_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "single-ASN1-type"));
	}
	Ok(BER.validate_any(&el.inner()?)?)
}(&el),
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice, "OsiBindResult-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values")),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindResult-normal-mode-parameters-user-data-fully-encoded-data-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item {
	pub transfer_syntax_name: OPTIONAL<Transfer_syntax_name>,
	pub presentation_context_identifier: Presentation_context_identifier,
	pub presentation_data_values: OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values
}
impl OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item {
    pub fn new(
        transfer_syntax_name: OPTIONAL<Transfer_syntax_name>,
        presentation_context_identifier: Presentation_context_identifier,
        presentation_data_values: OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
    ) -> Self {
        OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item {
            transfer_syntax_name,
            presentation_context_identifier,
            presentation_data_values,
        }
    }
}
impl TryFrom<&X690Element>
    for OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item
{
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item(el)
    }
}

pub const _rctl1_components_for_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item: &[ComponentSpec; 3] = &[
	ComponentSpec::new("transfer-syntax-name", true, TagSelector::tag((TagClass::UNIVERSAL, 6)), None, None),
	ComponentSpec::new("presentation-context-identifier", false, TagSelector::tag((TagClass::UNIVERSAL, 2)), None, None),
	ComponentSpec::new("presentation-data-values", false, TagSelector::any, None, None)
];

pub const _rctl2_components_for_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item: &[ComponentSpec; 0] = &[

];

pub fn _decode_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item(
    el: &X690Element,
) -> ASN1Result<OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBindResult-normal-mode-parameters-user-data-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item,
		_eal_components_for_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item,
		_rctl2_components_for_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item,
	).into_iter();
    let mut _i: usize = 0;
    let mut transfer_syntax_name_: OPTIONAL<Transfer_syntax_name> = None;
    let mut presentation_context_identifier_: OPTIONAL<Presentation_context_identifier> = None;
    let mut presentation_data_values_: OPTIONAL<OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
			"transfer-syntax-name" => transfer_syntax_name_ = Some(_decode_Transfer_syntax_name(_el)?),
			"presentation-context-identifier" => presentation_context_identifier_ = Some(_decode_Presentation_context_identifier(_el)?),
			"presentation-data-values" => presentation_data_values_ = Some(_decode_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(_el)?),
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBindResult-normal-mode-parameters-user-data-fully-encoded-data-Item")),
		}
    }
    Ok(
        OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item {
            transfer_syntax_name: transfer_syntax_name_,
            presentation_context_identifier: presentation_context_identifier_.unwrap(),
            presentation_data_values: presentation_data_values_.unwrap(),
        },
    )
}

pub fn _encode_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item(
    value_: &OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    if let Some(v_) = &value_.transfer_syntax_name {
        components_.push(_encode_Transfer_syntax_name(&v_)?);
    }
    components_.push(_encode_Presentation_context_identifier(
        &value_.presentation_context_identifier,
    )?);
    components_.push(_encode_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(&value_.presentation_data_values)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item(
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBindResult-normal-mode-parameters-user-data-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
		_elements.as_slice(),
		_rctl1_components_for_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item,
		_eal_components_for_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item,
		_rctl2_components_for_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item,
	).into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
			"transfer-syntax-name" => _validate_Transfer_syntax_name(_el)?,
			"presentation-context-identifier" => _validate_Presentation_context_identifier(_el)?,
			"presentation-data-values" => _validate_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(_el)?,
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBindResult-normal-mode-parameters-user-data-fully-encoded-data-Item")),
		}
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindResult-normal-mode-parameters-user-data ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum OsiBindResult_normal_mode_parameters_user_data {
    fully_encoded_data(Vec<OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item>),
}

impl TryFrom<&X690Element> for OsiBindResult_normal_mode_parameters_user_data {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBindResult_normal_mode_parameters_user_data(el)
    }
}

pub fn _decode_OsiBindResult_normal_mode_parameters_user_data(
    el: &X690Element,
) -> ASN1Result<OsiBindResult_normal_mode_parameters_user_data> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => Ok(
            OsiBindResult_normal_mode_parameters_user_data::fully_encoded_data(
                |el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<
                        OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item,
                    >,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "fully-encoded-data",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<
                        OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item,
                    > = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item(el)?);
                    }
                    Ok(items)
                }(&el)?,
            ),
        ),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiBindResult-normal-mode-parameters-user-data",
            ))
        }
    }
}

pub fn _encode_OsiBindResult_normal_mode_parameters_user_data(
    value_: &OsiBindResult_normal_mode_parameters_user_data,
) -> ASN1Result<X690Element> {
    match value_ {
        OsiBindResult_normal_mode_parameters_user_data::fully_encoded_data(v) => |v_1: &Vec<
            OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item,
        >|
         -> ASN1Result<
            X690Element,
        > {
            let mut el_1 = |value_: &SEQUENCE_OF<
                OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item,
            >|
             -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_1)?;
            el_1.tag.tag_class = TagClass::APPLICATION;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
    }
}

pub fn _validate_OsiBindResult_normal_mode_parameters_user_data(
    el: &X690Element,
) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fully-encoded-data")
                );
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_OsiBindResult_normal_mode_parameters_user_data_fully_encoded_data_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "fully-encoded-data",
                    )),
                }
            }(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiBindResult-normal-mode-parameters-user-data",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindResult-normal-mode-parameters ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiBindResult_normal_mode_parameters {
    pub protocol_version: OPTIONAL<OsiBindResult_normal_mode_parameters_protocol_version>,
    pub responding_presentation_selector: OPTIONAL<Presentation_selector>,
    pub presentation_context_definition_result_list:
        Vec<OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item>,
    pub user_data: OsiBindResult_normal_mode_parameters_user_data,
}
impl OsiBindResult_normal_mode_parameters {
    pub fn new(
        protocol_version: OPTIONAL<OsiBindResult_normal_mode_parameters_protocol_version>,
        responding_presentation_selector: OPTIONAL<Presentation_selector>,
        presentation_context_definition_result_list: Vec<
            OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item,
        >,
        user_data: OsiBindResult_normal_mode_parameters_user_data,
    ) -> Self {
        OsiBindResult_normal_mode_parameters {
            protocol_version,
            responding_presentation_selector,
            presentation_context_definition_result_list,
            user_data,
        }
    }
    pub fn _default_value_for_protocol_version(
    ) -> OsiBindResult_normal_mode_parameters_protocol_version {
        BIT_STRING::with_bits_set(&[
            OsiBindResult_normal_mode_parameters_protocol_version_version_1,
        ])
    }
}
impl TryFrom<&X690Element> for OsiBindResult_normal_mode_parameters {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBindResult_normal_mode_parameters(el)
    }
}

pub const _rctl1_components_for_OsiBindResult_normal_mode_parameters: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "protocol-version",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responding-presentation-selector",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "presentation-context-definition-result-list",
        false,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new("user-data", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_OsiBindResult_normal_mode_parameters: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiBindResult_normal_mode_parameters: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiBindResult_normal_mode_parameters(
    el: &X690Element,
) -> ASN1Result<OsiBindResult_normal_mode_parameters> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBindResult-normal-mode-parameters",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiBindResult_normal_mode_parameters,
        _eal_components_for_OsiBindResult_normal_mode_parameters,
        _rctl2_components_for_OsiBindResult_normal_mode_parameters,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut protocol_version_: OPTIONAL<OsiBindResult_normal_mode_parameters_protocol_version> =
        None;
    let mut responding_presentation_selector_: OPTIONAL<Presentation_selector> = None;
    let mut presentation_context_definition_result_list_: OPTIONAL<
        Vec<OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item>,
    > = None;
    let mut user_data_: OPTIONAL<OsiBindResult_normal_mode_parameters_user_data> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
			"protocol-version" => protocol_version_ = Some(_decode_OsiBindResult_normal_mode_parameters_protocol_version(_el)?),
			"responding-presentation-selector" => responding_presentation_selector_ = Some(_decode_Presentation_selector(_el)?),
			"presentation-context-definition-result-list" => presentation_context_definition_result_list_ = Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item>> {	let elements = match &el.value {
		X690Value::Constructed(children) => children,
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "presentation-context-definition-result-list")),
	};
	let mut items: SEQUENCE_OF<OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item> = Vec::with_capacity(elements.len());
	for el in elements.iter() {
		items.push(_decode_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item(el)?);
	}
	Ok(items)
}(_el)?),
			"user-data" => user_data_ = Some(_decode_OsiBindResult_normal_mode_parameters_user_data(_el)?),
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBindResult-normal-mode-parameters")),
		}
    }
    Ok(OsiBindResult_normal_mode_parameters {
        protocol_version: protocol_version_,
        responding_presentation_selector: responding_presentation_selector_,
        presentation_context_definition_result_list: presentation_context_definition_result_list_
            .unwrap(),
        user_data: user_data_.unwrap(),
    })
}

pub fn _encode_OsiBindResult_normal_mode_parameters(
    value_: &OsiBindResult_normal_mode_parameters,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    if let Some(v_) = &value_.protocol_version {
        if *v_ != OsiBindResult_normal_mode_parameters::_default_value_for_protocol_version() {
            components_.push(|v_1: &OsiBindResult_normal_mode_parameters_protocol_version| -> ASN1Result<X690Element> { let mut el_1 = _encode_OsiBindResult_normal_mode_parameters_protocol_version(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 0; Ok(el_1) }(&v_)?);
        }
    }
    if let Some(v_) = &value_.responding_presentation_selector {
        components_.push(|v_1: &Presentation_selector| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Presentation_selector(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v_)?);
    }
    components_.push(|v_1: &Vec<OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item>| -> ASN1Result<X690Element> { let mut el_1 = |value_: &SEQUENCE_OF<OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item>| -> ASN1Result<X690Element> {	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item(&v)?);
	}
	Ok(X690Element::new(Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF), X690Value::Constructed(Arc::new(children))))
}(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 5; Ok(el_1) }(&value_.presentation_context_definition_result_list)?);
    components_.push(_encode_OsiBindResult_normal_mode_parameters_user_data(
        &value_.user_data,
    )?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiBindResult_normal_mode_parameters(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBindResult-normal-mode-parameters",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiBindResult_normal_mode_parameters,
        _eal_components_for_OsiBindResult_normal_mode_parameters,
        _rctl2_components_for_OsiBindResult_normal_mode_parameters,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protocol-version" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "protocol-version",
                    ));
                }
                Ok(_validate_OsiBindResult_normal_mode_parameters_protocol_version(&el)?)
            }(_el)?,
            "responding-presentation-selector" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "responding-presentation-selector",
                    ));
                }
                Ok(_validate_Presentation_selector(&el)?)
            }(_el)?,
            "presentation-context-definition-result-list" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "presentation-context-definition-result-list",
                    ));
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_OsiBindResult_normal_mode_parameters_presentation_context_definition_result_list_Item(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "presentation-context-definition-result-list",
                        )),
                    }
                }(&el)?)
            }(_el)?,
            "user-data" => _validate_OsiBindResult_normal_mode_parameters_user_data(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OsiBindResult-normal-mode-parameters",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AARE-apdu-protocol-version ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type AARE_apdu_protocol_version = BIT_STRING;

pub const AARE_apdu_protocol_version_version1: BIT = 0; /* LONG_NAMED_BIT */

pub fn _decode_AARE_apdu_protocol_version(
    el: &X690Element,
) -> ASN1Result<AARE_apdu_protocol_version> {
    BER.decode_bit_string(&el)
}

pub fn _encode_AARE_apdu_protocol_version(
    value_: &AARE_apdu_protocol_version,
) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_AARE_apdu_protocol_version(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Associate-source-diagnostic-acse-service-user ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type Associate_source_diagnostic_acse_service_user = i8;

pub const Associate_source_diagnostic_acse_service_user_null:
    Associate_source_diagnostic_acse_service_user = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_user_no_reason_given:
    Associate_source_diagnostic_acse_service_user = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_user_application_context_name_not_supported:
    Associate_source_diagnostic_acse_service_user = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_user_calling_AP_title_not_recognized:
    Associate_source_diagnostic_acse_service_user = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_user_calling_AP_invocation_identifier_not_recognized: Associate_source_diagnostic_acse_service_user = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_user_calling_AE_qualifier_not_recognized:
    Associate_source_diagnostic_acse_service_user = 5; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_user_calling_AE_invocation_identifier_not_recognized: Associate_source_diagnostic_acse_service_user = 6; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_user_called_AP_title_not_recognized:
    Associate_source_diagnostic_acse_service_user = 7; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_user_called_AP_invocation_identifier_not_recognized: Associate_source_diagnostic_acse_service_user = 8; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_user_called_AE_qualifier_not_recognized:
    Associate_source_diagnostic_acse_service_user = 9; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_user_called_AE_invocation_identifier_not_recognized: Associate_source_diagnostic_acse_service_user = 10; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Associate_source_diagnostic_acse_service_user(
    el: &X690Element,
) -> ASN1Result<Associate_source_diagnostic_acse_service_user> {
    BER.decode_i8(el)
}

pub fn _encode_Associate_source_diagnostic_acse_service_user(
    value_: &Associate_source_diagnostic_acse_service_user,
) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Associate_source_diagnostic_acse_service_user(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Associate-source-diagnostic-acse-service-provider ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type Associate_source_diagnostic_acse_service_provider = i8;

pub const Associate_source_diagnostic_acse_service_provider_null:
    Associate_source_diagnostic_acse_service_provider = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_provider_no_reason_given:
    Associate_source_diagnostic_acse_service_provider = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Associate_source_diagnostic_acse_service_provider_no_common_acse_version:
    Associate_source_diagnostic_acse_service_provider = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Associate_source_diagnostic_acse_service_provider(
    el: &X690Element,
) -> ASN1Result<Associate_source_diagnostic_acse_service_provider> {
    BER.decode_i8(el)
}

pub fn _encode_Associate_source_diagnostic_acse_service_provider(
    value_: &Associate_source_diagnostic_acse_service_provider,
) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Associate_source_diagnostic_acse_service_provider(
    el: &X690Element,
) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindError-normal-mode-parameters-protocol-version ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type OsiBindError_normal_mode_parameters_protocol_version = BIT_STRING;

pub const OsiBindError_normal_mode_parameters_protocol_version_version_1: BIT = 0; /* LONG_NAMED_BIT */

pub fn _decode_OsiBindError_normal_mode_parameters_protocol_version(
    el: &X690Element,
) -> ASN1Result<OsiBindError_normal_mode_parameters_protocol_version> {
    BER.decode_bit_string(&el)
}

pub fn _encode_OsiBindError_normal_mode_parameters_protocol_version(
    value_: &OsiBindError_normal_mode_parameters_protocol_version,
) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_OsiBindError_normal_mode_parameters_protocol_version(
    el: &X690Element,
) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindError-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values
{
    single_ASN1_type(X690Element),
}

impl TryFrom<&X690Element> for OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values {
	type Error = ASN1Error;
	fn try_from (el: &X690Element) -> Result<Self, Self::Error> {
		_decode_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(el)
	}
}

pub fn _decode_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<
    OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
> {
    match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => Ok(OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(|el: &X690Element| -> ASN1Result<X690Element> {
	Ok(x690_identity(&el.inner()?)?)
}(&el)?)),
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice, "OsiBindError-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values")),
	}
}

pub fn _encode_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    value_: &OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
) -> ASN1Result<X690Element> {
    match value_ {
		OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(v) => |v_1: &X690Element| -> ASN1Result<X690Element> { Ok(X690Element::new(Tag::new(TagClass::CONTEXT, 0), X690Value::from_explicit(&x690_identity(&v_1)?))) }(&v),
	}
}

pub fn _validate_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "single-ASN1-type"));
	}
	Ok(BER.validate_any(&el.inner()?)?)
}(&el),
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice, "OsiBindError-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values")),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindError-normal-mode-parameters-user-data-fully-encoded-data-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item {
	pub transfer_syntax_name: OPTIONAL<Transfer_syntax_name>,
	pub presentation_context_identifier: Presentation_context_identifier,
	pub presentation_data_values: OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values
}
impl OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item {
    pub fn new(
        transfer_syntax_name: OPTIONAL<Transfer_syntax_name>,
        presentation_context_identifier: Presentation_context_identifier,
        presentation_data_values: OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
    ) -> Self {
        OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item {
            transfer_syntax_name,
            presentation_context_identifier,
            presentation_data_values,
        }
    }
}
impl TryFrom<&X690Element>
    for OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item
{
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item(el)
    }
}

pub const _rctl1_components_for_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item: &[ComponentSpec; 3] = &[
	ComponentSpec::new("transfer-syntax-name", true, TagSelector::tag((TagClass::UNIVERSAL, 6)), None, None),
	ComponentSpec::new("presentation-context-identifier", false, TagSelector::tag((TagClass::UNIVERSAL, 2)), None, None),
	ComponentSpec::new("presentation-data-values", false, TagSelector::any, None, None)
];

pub const _rctl2_components_for_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item: &[ComponentSpec; 0] = &[

];

pub fn _decode_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item(
    el: &X690Element,
) -> ASN1Result<OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBindError-normal-mode-parameters-user-data-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _eal_components_for_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _rctl2_components_for_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut transfer_syntax_name_: OPTIONAL<Transfer_syntax_name> = None;
    let mut presentation_context_identifier_: OPTIONAL<Presentation_context_identifier> = None;
    let mut presentation_data_values_: OPTIONAL<OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
			"transfer-syntax-name" => transfer_syntax_name_ = Some(_decode_Transfer_syntax_name(_el)?),
			"presentation-context-identifier" => presentation_context_identifier_ = Some(_decode_Presentation_context_identifier(_el)?),
			"presentation-data-values" => presentation_data_values_ = Some(_decode_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(_el)?),
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBindError-normal-mode-parameters-user-data-fully-encoded-data-Item")),
		}
    }
    Ok(
        OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item {
            transfer_syntax_name: transfer_syntax_name_,
            presentation_context_identifier: presentation_context_identifier_.unwrap(),
            presentation_data_values: presentation_data_values_.unwrap(),
        },
    )
}

pub fn _encode_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item(
    value_: &OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    if let Some(v_) = &value_.transfer_syntax_name {
        components_.push(_encode_Transfer_syntax_name(&v_)?);
    }
    components_.push(_encode_Presentation_context_identifier(
        &value_.presentation_context_identifier,
    )?);
    components_.push(_encode_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(&value_.presentation_data_values)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item(
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBindError-normal-mode-parameters-user-data-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _eal_components_for_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _rctl2_components_for_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
			"transfer-syntax-name" => _validate_Transfer_syntax_name(_el)?,
			"presentation-context-identifier" => _validate_Presentation_context_identifier(_el)?,
			"presentation-data-values" => _validate_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(_el)?,
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiBindError-normal-mode-parameters-user-data-fully-encoded-data-Item")),
		}
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindError-normal-mode-parameters-user-data ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum OsiBindError_normal_mode_parameters_user_data {
    fully_encoded_data(Vec<OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item>),
}

impl TryFrom<&X690Element> for OsiBindError_normal_mode_parameters_user_data {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBindError_normal_mode_parameters_user_data(el)
    }
}

pub fn _decode_OsiBindError_normal_mode_parameters_user_data(
    el: &X690Element,
) -> ASN1Result<OsiBindError_normal_mode_parameters_user_data> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => Ok(
            OsiBindError_normal_mode_parameters_user_data::fully_encoded_data(
                |el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<
                        OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item,
                    >,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "fully-encoded-data",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<
                        OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item,
                    > = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item(el)?);
                    }
                    Ok(items)
                }(&el)?,
            ),
        ),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiBindError-normal-mode-parameters-user-data",
            ))
        }
    }
}

pub fn _encode_OsiBindError_normal_mode_parameters_user_data(
    value_: &OsiBindError_normal_mode_parameters_user_data,
) -> ASN1Result<X690Element> {
    match value_ {
        OsiBindError_normal_mode_parameters_user_data::fully_encoded_data(v) => |v_1: &Vec<
            OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item,
        >|
         -> ASN1Result<
            X690Element,
        > {
            let mut el_1 = |value_: &SEQUENCE_OF<
                OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item,
            >|
             -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_1)?;
            el_1.tag.tag_class = TagClass::APPLICATION;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
    }
}

pub fn _validate_OsiBindError_normal_mode_parameters_user_data(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fully-encoded-data")
                );
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_OsiBindError_normal_mode_parameters_user_data_fully_encoded_data_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "fully-encoded-data",
                    )),
                }
            }(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiBindError-normal-mode-parameters-user-data",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiBindError-normal-mode-parameters ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiBindError_normal_mode_parameters {
    pub protocol_version: OPTIONAL<OsiBindError_normal_mode_parameters_protocol_version>,
    pub responding_presentation_selector: OPTIONAL<Presentation_selector>,
    pub presentation_context_definition_result_list: OPTIONAL<Result_list>,
    pub provider_reason: OPTIONAL<Provider_reason>,
    pub user_data: OPTIONAL<OsiBindError_normal_mode_parameters_user_data>,
}
impl OsiBindError_normal_mode_parameters {
    pub fn new(
        protocol_version: OPTIONAL<OsiBindError_normal_mode_parameters_protocol_version>,
        responding_presentation_selector: OPTIONAL<Presentation_selector>,
        presentation_context_definition_result_list: OPTIONAL<Result_list>,
        provider_reason: OPTIONAL<Provider_reason>,
        user_data: OPTIONAL<OsiBindError_normal_mode_parameters_user_data>,
    ) -> Self {
        OsiBindError_normal_mode_parameters {
            protocol_version,
            responding_presentation_selector,
            presentation_context_definition_result_list,
            provider_reason,
            user_data,
        }
    }
    pub fn _default_value_for_protocol_version(
    ) -> OsiBindError_normal_mode_parameters_protocol_version {
        BIT_STRING::with_bits_set(&[OsiBindError_normal_mode_parameters_protocol_version_version_1])
    }
}
impl Default for OsiBindError_normal_mode_parameters {
    fn default() -> Self {
        OsiBindError_normal_mode_parameters {
            protocol_version: None,
            responding_presentation_selector: None,
            presentation_context_definition_result_list: None,
            provider_reason: None,
            user_data: None,
        }
    }
}
impl TryFrom<&X690Element> for OsiBindError_normal_mode_parameters {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiBindError_normal_mode_parameters(el)
    }
}

pub const _rctl1_components_for_OsiBindError_normal_mode_parameters: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "protocol-version",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "responding-presentation-selector",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "presentation-context-definition-result-list",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "provider-reason",
        true,
        TagSelector::tag((TagClass::CONTEXT, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "user-data",
        true,
        TagSelector::or(&[&TagSelector::tag((TagClass::APPLICATION, 1))]),
        None,
        None,
    ),
];

pub const _rctl2_components_for_OsiBindError_normal_mode_parameters: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiBindError_normal_mode_parameters: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiBindError_normal_mode_parameters(
    el: &X690Element,
) -> ASN1Result<OsiBindError_normal_mode_parameters> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBindError-normal-mode-parameters",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiBindError_normal_mode_parameters,
        _eal_components_for_OsiBindError_normal_mode_parameters,
        _rctl2_components_for_OsiBindError_normal_mode_parameters,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut protocol_version_: OPTIONAL<OsiBindError_normal_mode_parameters_protocol_version> =
        None;
    let mut responding_presentation_selector_: OPTIONAL<Presentation_selector> = None;
    let mut presentation_context_definition_result_list_: OPTIONAL<Result_list> = None;
    let mut provider_reason_: OPTIONAL<Provider_reason> = None;
    let mut user_data_: OPTIONAL<OsiBindError_normal_mode_parameters_user_data> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protocol-version" => {
                protocol_version_ =
                    Some(_decode_OsiBindError_normal_mode_parameters_protocol_version(_el)?)
            }
            "responding-presentation-selector" => {
                responding_presentation_selector_ = Some(_decode_Presentation_selector(_el)?)
            }
            "presentation-context-definition-result-list" => {
                presentation_context_definition_result_list_ = Some(_decode_Result_list(_el)?)
            }
            "provider-reason" => provider_reason_ = Some(_decode_Provider_reason(_el)?),
            "user-data" => {
                user_data_ = Some(_decode_OsiBindError_normal_mode_parameters_user_data(_el)?)
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OsiBindError-normal-mode-parameters",
                ))
            }
        }
    }
    Ok(OsiBindError_normal_mode_parameters {
        protocol_version: protocol_version_,
        responding_presentation_selector: responding_presentation_selector_,
        presentation_context_definition_result_list: presentation_context_definition_result_list_,
        provider_reason: provider_reason_,
        user_data: user_data_,
    })
}

pub fn _encode_OsiBindError_normal_mode_parameters(
    value_: &OsiBindError_normal_mode_parameters,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(10);
    if let Some(v_) = &value_.protocol_version {
        if *v_ != OsiBindError_normal_mode_parameters::_default_value_for_protocol_version() {
            components_.push(|v_1: &OsiBindError_normal_mode_parameters_protocol_version| -> ASN1Result<X690Element> { let mut el_1 = _encode_OsiBindError_normal_mode_parameters_protocol_version(&v_1)?; el_1.tag.tag_class = TagClass::CONTEXT; el_1.tag.tag_number = 0; Ok(el_1) }(&v_)?);
        }
    }
    if let Some(v_) = &value_.responding_presentation_selector {
        components_.push(|v_1: &Presentation_selector| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Presentation_selector(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.presentation_context_definition_result_list {
        components_.push(|v_1: &Result_list| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Result_list(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 5;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.provider_reason {
        components_.push(|v_1: &Provider_reason| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Provider_reason(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 10;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.user_data {
        components_.push(_encode_OsiBindError_normal_mode_parameters_user_data(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiBindError_normal_mode_parameters(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiBindError-normal-mode-parameters",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiBindError_normal_mode_parameters,
        _eal_components_for_OsiBindError_normal_mode_parameters,
        _rctl2_components_for_OsiBindError_normal_mode_parameters,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protocol-version" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "protocol-version",
                    ));
                }
                Ok(_validate_OsiBindError_normal_mode_parameters_protocol_version(&el)?)
            }(_el)?,
            "responding-presentation-selector" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "responding-presentation-selector",
                    ));
                }
                Ok(_validate_Presentation_selector(&el)?)
            }(_el)?,
            "presentation-context-definition-result-list" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "presentation-context-definition-result-list",
                    ));
                }
                Ok(_validate_Result_list(&el)?)
            }(_el)?,
            "provider-reason" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 10 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "provider-reason",
                    ));
                }
                Ok(_validate_Provider_reason(&el)?)
            }(_el)?,
            "user-data" => _validate_OsiBindError_normal_mode_parameters_user_data(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OsiBindError-normal-mode-parameters",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Result-list-Item-provider-reason ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type Result_list_Item_provider_reason = i8;

pub const Result_list_Item_provider_reason_reason_not_specified: Result_list_Item_provider_reason =
    0; /* LONG_NAMED_INTEGER_VALUE */

pub const Result_list_Item_provider_reason_abstract_syntax_not_supported:
    Result_list_Item_provider_reason = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const Result_list_Item_provider_reason_proposed_transfer_syntaxes_not_supported:
    Result_list_Item_provider_reason = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_Result_list_Item_provider_reason(
    el: &X690Element,
) -> ASN1Result<Result_list_Item_provider_reason> {
    BER.decode_i8(el)
}

pub fn _encode_Result_list_Item_provider_reason(
    value_: &Result_list_Item_provider_reason,
) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_Result_list_Item_provider_reason(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Result-list-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct Result_list_Item {
    pub result: Result_,
    pub transfer_syntax_name: OPTIONAL<Transfer_syntax_name>,
    pub provider_reason: OPTIONAL<Result_list_Item_provider_reason>,
}
impl Result_list_Item {
    pub fn new(
        result: Result_,
        transfer_syntax_name: OPTIONAL<Transfer_syntax_name>,
        provider_reason: OPTIONAL<Result_list_Item_provider_reason>,
    ) -> Self {
        Result_list_Item {
            result,
            transfer_syntax_name,
            provider_reason,
        }
    }
}
impl TryFrom<&X690Element> for Result_list_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Result_list_Item(el)
    }
}

pub const _rctl1_components_for_Result_list_Item: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "result",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "transfer-syntax-name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "provider-reason",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Result_list_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Result_list_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_Result_list_Item(el: &X690Element) -> ASN1Result<Result_list_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Result-list-Item")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Result_list_Item,
        _eal_components_for_Result_list_Item,
        _rctl2_components_for_Result_list_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut result_: OPTIONAL<Result_> = None;
    let mut transfer_syntax_name_: OPTIONAL<Transfer_syntax_name> = None;
    let mut provider_reason_: OPTIONAL<Result_list_Item_provider_reason> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "result" => result_ = Some(_decode_Result_(_el)?),
            "transfer-syntax-name" => {
                transfer_syntax_name_ = Some(_decode_Transfer_syntax_name(_el)?)
            }
            "provider-reason" => {
                provider_reason_ = Some(_decode_Result_list_Item_provider_reason(_el)?)
            }
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Result-list-Item")
                )
            }
        }
    }
    Ok(Result_list_Item {
        result: result_.unwrap(),
        transfer_syntax_name: transfer_syntax_name_,
        provider_reason: provider_reason_,
    })
}

pub fn _encode_Result_list_Item(value_: &Result_list_Item) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(|v_1: &Result_| -> ASN1Result<X690Element> {
        let mut el_1 = _encode_Result_(&v_1)?;
        el_1.tag.tag_class = TagClass::CONTEXT;
        el_1.tag.tag_number = 0;
        Ok(el_1)
    }(&value_.result)?);
    if let Some(v_) = &value_.transfer_syntax_name {
        components_.push(|v_1: &Transfer_syntax_name| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Transfer_syntax_name(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v_)?);
    }
    if let Some(v_) = &value_.provider_reason {
        components_.push(
            |v_1: &Result_list_Item_provider_reason| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_Result_list_Item_provider_reason(&v_1)?;
                el_1.tag.tag_class = TagClass::CONTEXT;
                el_1.tag.tag_number = 2;
                Ok(el_1)
            }(&v_)?,
        );
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Result_list_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Result-list-Item")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Result_list_Item,
        _eal_components_for_Result_list_Item,
        _rctl2_components_for_Result_list_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "result" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "result"));
                }
                Ok(_validate_Result_(&el)?)
            }(_el)?,
            "transfer-syntax-name" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "transfer-syntax-name",
                    ));
                }
                Ok(_validate_Transfer_syntax_name(&el)?)
            }(_el)?,
            "provider-reason" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "provider-reason",
                    ));
                }
                Ok(_validate_Result_list_Item_provider_reason(&el)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Result-list-Item")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AAREerr-apdu-protocol-version ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type AAREerr_apdu_protocol_version = BIT_STRING;

pub const AAREerr_apdu_protocol_version_version1: BIT = 0; /* LONG_NAMED_BIT */

pub fn _decode_AAREerr_apdu_protocol_version(
    el: &X690Element,
) -> ASN1Result<AAREerr_apdu_protocol_version> {
    BER.decode_bit_string(&el)
}

pub fn _encode_AAREerr_apdu_protocol_version(
    value_: &AAREerr_apdu_protocol_version,
) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_AAREerr_apdu_protocol_version(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiUnbind-fully-encoded-data-Item-presentation-data-values ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum OsiUnbind_fully_encoded_data_Item_presentation_data_values {
    single_ASN1_type(X690Element),
}

impl TryFrom<&X690Element> for OsiUnbind_fully_encoded_data_Item_presentation_data_values {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiUnbind_fully_encoded_data_Item_presentation_data_values(el)
    }
}

pub fn _decode_OsiUnbind_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<OsiUnbind_fully_encoded_data_Item_presentation_data_values> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(
            OsiUnbind_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(
                |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                    &el,
                )?,
            ),
        ),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiUnbind-fully-encoded-data-Item-presentation-data-values",
            ))
        }
    }
}

pub fn _encode_OsiUnbind_fully_encoded_data_Item_presentation_data_values(
    value_: &OsiUnbind_fully_encoded_data_Item_presentation_data_values,
) -> ASN1Result<X690Element> {
    match value_ {
        OsiUnbind_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(v) => {
            |v_1: &X690Element| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&x690_identity(&v_1)?),
                ))
            }(&v)
        }
    }
}

pub fn _validate_OsiUnbind_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "single-ASN1-type")
                );
            }
            Ok(BER.validate_any(&el.inner()?)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiUnbind-fully-encoded-data-Item-presentation-data-values",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiUnbind-fully-encoded-data-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiUnbind_fully_encoded_data_Item {
    pub presentation_context_identifier: Presentation_context_identifier,
    pub presentation_data_values: OsiUnbind_fully_encoded_data_Item_presentation_data_values,
}
impl OsiUnbind_fully_encoded_data_Item {
    pub fn new(
        presentation_context_identifier: Presentation_context_identifier,
        presentation_data_values: OsiUnbind_fully_encoded_data_Item_presentation_data_values,
    ) -> Self {
        OsiUnbind_fully_encoded_data_Item {
            presentation_context_identifier,
            presentation_data_values,
        }
    }
}
impl TryFrom<&X690Element> for OsiUnbind_fully_encoded_data_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiUnbind_fully_encoded_data_Item(el)
    }
}

pub const _rctl1_components_for_OsiUnbind_fully_encoded_data_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "presentation-context-identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "presentation-data-values",
        false,
        TagSelector::any,
        None,
        None,
    ),
];

pub const _rctl2_components_for_OsiUnbind_fully_encoded_data_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiUnbind_fully_encoded_data_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiUnbind_fully_encoded_data_Item(
    el: &X690Element,
) -> ASN1Result<OsiUnbind_fully_encoded_data_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiUnbind-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiUnbind_fully_encoded_data_Item,
        _eal_components_for_OsiUnbind_fully_encoded_data_Item,
        _rctl2_components_for_OsiUnbind_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut presentation_context_identifier_: OPTIONAL<Presentation_context_identifier> = None;
    let mut presentation_data_values_: OPTIONAL<
        OsiUnbind_fully_encoded_data_Item_presentation_data_values,
    > = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier" => {
                presentation_context_identifier_ =
                    Some(_decode_Presentation_context_identifier(_el)?)
            }
            "presentation-data-values" => {
                presentation_data_values_ =
                    Some(_decode_OsiUnbind_fully_encoded_data_Item_presentation_data_values(_el)?)
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OsiUnbind-fully-encoded-data-Item",
                ))
            }
        }
    }
    Ok(OsiUnbind_fully_encoded_data_Item {
        presentation_context_identifier: presentation_context_identifier_.unwrap(),
        presentation_data_values: presentation_data_values_.unwrap(),
    })
}

pub fn _encode_OsiUnbind_fully_encoded_data_Item(
    value_: &OsiUnbind_fully_encoded_data_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_Presentation_context_identifier(
        &value_.presentation_context_identifier,
    )?);
    components_.push(
        _encode_OsiUnbind_fully_encoded_data_Item_presentation_data_values(
            &value_.presentation_data_values,
        )?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiUnbind_fully_encoded_data_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiUnbind-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiUnbind_fully_encoded_data_Item,
        _eal_components_for_OsiUnbind_fully_encoded_data_Item,
        _rctl2_components_for_OsiUnbind_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier" => _validate_Presentation_context_identifier(_el)?,
            "presentation-data-values" => {
                _validate_OsiUnbind_fully_encoded_data_Item_presentation_data_values(_el)?
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OsiUnbind-fully-encoded-data-Item",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiUnbindResult-fully-encoded-data-Item-presentation-data-values ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum OsiUnbindResult_fully_encoded_data_Item_presentation_data_values {
    single_ASN1_type(X690Element),
}

impl TryFrom<&X690Element> for OsiUnbindResult_fully_encoded_data_Item_presentation_data_values {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiUnbindResult_fully_encoded_data_Item_presentation_data_values(el)
    }
}

pub fn _decode_OsiUnbindResult_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<OsiUnbindResult_fully_encoded_data_Item_presentation_data_values> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(
            OsiUnbindResult_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(
                |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                    &el,
                )?,
            ),
        ),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiUnbindResult-fully-encoded-data-Item-presentation-data-values",
            ))
        }
    }
}

pub fn _encode_OsiUnbindResult_fully_encoded_data_Item_presentation_data_values(
    value_: &OsiUnbindResult_fully_encoded_data_Item_presentation_data_values,
) -> ASN1Result<X690Element> {
    match value_ {
        OsiUnbindResult_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(v) => {
            |v_1: &X690Element| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&x690_identity(&v_1)?),
                ))
            }(&v)
        }
    }
}

pub fn _validate_OsiUnbindResult_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "single-ASN1-type")
                );
            }
            Ok(BER.validate_any(&el.inner()?)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiUnbindResult-fully-encoded-data-Item-presentation-data-values",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiUnbindResult-fully-encoded-data-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiUnbindResult_fully_encoded_data_Item {
    pub presentation_context_identifier: Presentation_context_identifier,
    pub presentation_data_values: OsiUnbindResult_fully_encoded_data_Item_presentation_data_values,
}
impl OsiUnbindResult_fully_encoded_data_Item {
    pub fn new(
        presentation_context_identifier: Presentation_context_identifier,
        presentation_data_values: OsiUnbindResult_fully_encoded_data_Item_presentation_data_values,
    ) -> Self {
        OsiUnbindResult_fully_encoded_data_Item {
            presentation_context_identifier,
            presentation_data_values,
        }
    }
}
impl TryFrom<&X690Element> for OsiUnbindResult_fully_encoded_data_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiUnbindResult_fully_encoded_data_Item(el)
    }
}

pub const _rctl1_components_for_OsiUnbindResult_fully_encoded_data_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "presentation-context-identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "presentation-data-values",
        false,
        TagSelector::any,
        None,
        None,
    ),
];

pub const _rctl2_components_for_OsiUnbindResult_fully_encoded_data_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiUnbindResult_fully_encoded_data_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiUnbindResult_fully_encoded_data_Item(
    el: &X690Element,
) -> ASN1Result<OsiUnbindResult_fully_encoded_data_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiUnbindResult-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiUnbindResult_fully_encoded_data_Item,
        _eal_components_for_OsiUnbindResult_fully_encoded_data_Item,
        _rctl2_components_for_OsiUnbindResult_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut presentation_context_identifier_: OPTIONAL<Presentation_context_identifier> = None;
    let mut presentation_data_values_: OPTIONAL<
        OsiUnbindResult_fully_encoded_data_Item_presentation_data_values,
    > = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier" => {
                presentation_context_identifier_ =
                    Some(_decode_Presentation_context_identifier(_el)?)
            }
            "presentation-data-values" => {
                presentation_data_values_ = Some(
                    _decode_OsiUnbindResult_fully_encoded_data_Item_presentation_data_values(_el)?,
                )
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OsiUnbindResult-fully-encoded-data-Item",
                ))
            }
        }
    }
    Ok(OsiUnbindResult_fully_encoded_data_Item {
        presentation_context_identifier: presentation_context_identifier_.unwrap(),
        presentation_data_values: presentation_data_values_.unwrap(),
    })
}

pub fn _encode_OsiUnbindResult_fully_encoded_data_Item(
    value_: &OsiUnbindResult_fully_encoded_data_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_Presentation_context_identifier(
        &value_.presentation_context_identifier,
    )?);
    components_.push(
        _encode_OsiUnbindResult_fully_encoded_data_Item_presentation_data_values(
            &value_.presentation_data_values,
        )?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiUnbindResult_fully_encoded_data_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiUnbindResult-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiUnbindResult_fully_encoded_data_Item,
        _eal_components_for_OsiUnbindResult_fully_encoded_data_Item,
        _rctl2_components_for_OsiUnbindResult_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier" => _validate_Presentation_context_identifier(_el)?,
            "presentation-data-values" => {
                _validate_OsiUnbindResult_fully_encoded_data_Item_presentation_data_values(_el)?
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OsiUnbindResult-fully-encoded-data-Item",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiOperation-fully-encoded-data-Item-presentation-data-values ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum OsiOperation_fully_encoded_data_Item_presentation_data_values {
    single_ASN1_type(X690Element),
}

impl TryFrom<&X690Element> for OsiOperation_fully_encoded_data_Item_presentation_data_values {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiOperation_fully_encoded_data_Item_presentation_data_values(el)
    }
}

pub fn _decode_OsiOperation_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<OsiOperation_fully_encoded_data_Item_presentation_data_values> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(
            OsiOperation_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(
                |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                    &el,
                )?,
            ),
        ),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiOperation-fully-encoded-data-Item-presentation-data-values",
            ))
        }
    }
}

pub fn _encode_OsiOperation_fully_encoded_data_Item_presentation_data_values(
    value_: &OsiOperation_fully_encoded_data_Item_presentation_data_values,
) -> ASN1Result<X690Element> {
    match value_ {
        OsiOperation_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(v) => {
            |v_1: &X690Element| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&x690_identity(&v_1)?),
                ))
            }(&v)
        }
    }
}

pub fn _validate_OsiOperation_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "single-ASN1-type")
                );
            }
            Ok(BER.validate_any(&el.inner()?)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "OsiOperation-fully-encoded-data-Item-presentation-data-values",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiOperation-fully-encoded-data-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiOperation_fully_encoded_data_Item {
    pub presentation_context_identifier: Presentation_context_identifier,
    pub presentation_data_values: OsiOperation_fully_encoded_data_Item_presentation_data_values,
}
impl OsiOperation_fully_encoded_data_Item {
    pub fn new(
        presentation_context_identifier: Presentation_context_identifier,
        presentation_data_values: OsiOperation_fully_encoded_data_Item_presentation_data_values,
    ) -> Self {
        OsiOperation_fully_encoded_data_Item {
            presentation_context_identifier,
            presentation_data_values,
        }
    }
}
impl TryFrom<&X690Element> for OsiOperation_fully_encoded_data_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiOperation_fully_encoded_data_Item(el)
    }
}

pub const _rctl1_components_for_OsiOperation_fully_encoded_data_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "presentation-context-identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "presentation-data-values",
        false,
        TagSelector::any,
        None,
        None,
    ),
];

pub const _rctl2_components_for_OsiOperation_fully_encoded_data_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiOperation_fully_encoded_data_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiOperation_fully_encoded_data_Item(
    el: &X690Element,
) -> ASN1Result<OsiOperation_fully_encoded_data_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiOperation-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiOperation_fully_encoded_data_Item,
        _eal_components_for_OsiOperation_fully_encoded_data_Item,
        _rctl2_components_for_OsiOperation_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut presentation_context_identifier_: OPTIONAL<Presentation_context_identifier> = None;
    let mut presentation_data_values_: OPTIONAL<
        OsiOperation_fully_encoded_data_Item_presentation_data_values,
    > = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier" => {
                presentation_context_identifier_ =
                    Some(_decode_Presentation_context_identifier(_el)?)
            }
            "presentation-data-values" => {
                presentation_data_values_ = Some(
                    _decode_OsiOperation_fully_encoded_data_Item_presentation_data_values(_el)?,
                )
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OsiOperation-fully-encoded-data-Item",
                ))
            }
        }
    }
    Ok(OsiOperation_fully_encoded_data_Item {
        presentation_context_identifier: presentation_context_identifier_.unwrap(),
        presentation_data_values: presentation_data_values_.unwrap(),
    })
}

pub fn _encode_OsiOperation_fully_encoded_data_Item(
    value_: &OsiOperation_fully_encoded_data_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_Presentation_context_identifier(
        &value_.presentation_context_identifier,
    )?);
    components_.push(
        _encode_OsiOperation_fully_encoded_data_Item_presentation_data_values(
            &value_.presentation_data_values,
        )?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiOperation_fully_encoded_data_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OsiOperation-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiOperation_fully_encoded_data_Item,
        _eal_components_for_OsiOperation_fully_encoded_data_Item,
        _rctl2_components_for_OsiOperation_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier" => _validate_Presentation_context_identifier(_el)?,
            "presentation-data-values" => {
                _validate_OsiOperation_fully_encoded_data_Item_presentation_data_values(_el)?
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "OsiOperation-fully-encoded-data-Item",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiRes-result ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct OsiRes_result {
    pub opcode: Code,
    pub result: X690Element,
}
impl OsiRes_result {
    pub fn new(opcode: Code, result: X690Element) -> Self {
        OsiRes_result { opcode, result }
    }
}
impl TryFrom<&X690Element> for OsiRes_result {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiRes_result(el)
    }
}

pub const _rctl1_components_for_OsiRes_result: &[ComponentSpec; 2] = &[
    ComponentSpec::new("opcode", false, TagSelector::any, None, None),
    ComponentSpec::new("result", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_OsiRes_result: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_OsiRes_result: &[ComponentSpec; 0] = &[];

pub fn _decode_OsiRes_result(el: &X690Element) -> ASN1Result<OsiRes_result> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRes-result")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiRes_result,
        _eal_components_for_OsiRes_result,
        _rctl2_components_for_OsiRes_result,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut opcode_: OPTIONAL<Code> = None;
    let mut result_: OPTIONAL<X690Element> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "opcode" => opcode_ = Some(_decode_Code(_el)?),
            "result" => result_ = Some(x690_identity(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRes-result")
                )
            }
        }
    }
    Ok(OsiRes_result {
        opcode: opcode_.unwrap(),
        result: result_.unwrap(),
    })
}

pub fn _encode_OsiRes_result(value_: &OsiRes_result) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_Code(&value_.opcode)?);
    components_.push(x690_identity(&value_.result)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_OsiRes_result(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRes-result")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_OsiRes_result,
        _eal_components_for_OsiRes_result,
        _rctl2_components_for_OsiRes_result,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "opcode" => _validate_Code(_el)?,
            "result" => BER.validate_any(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "OsiRes-result")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OsiRej-problem ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum OsiRej_problem {
    general(GeneralProblem),
    invoke(InvokeProblem),
    returnResult(ReturnResultProblem),
    returnError(ReturnErrorProblem),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for OsiRej_problem {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OsiRej_problem(el)
    }
}

pub fn _decode_OsiRej_problem(el: &X690Element) -> ASN1Result<OsiRej_problem> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(OsiRej_problem::general(_decode_GeneralProblem(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(OsiRej_problem::invoke(_decode_InvokeProblem(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(OsiRej_problem::returnResult(_decode_ReturnResultProblem(
            &el,
        )?)),
        (TagClass::CONTEXT, 3) => Ok(OsiRej_problem::returnError(_decode_ReturnErrorProblem(
            &el,
        )?)),
        _ => Ok(OsiRej_problem::_unrecognized(el.clone())),
    }
}

pub fn _encode_OsiRej_problem(value_: &OsiRej_problem) -> ASN1Result<X690Element> {
    match value_ {
        OsiRej_problem::general(v) => |v_1: &GeneralProblem| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_GeneralProblem(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&v),
        OsiRej_problem::invoke(v) => |v_1: &InvokeProblem| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_InvokeProblem(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
        OsiRej_problem::returnResult(v) => |v_1: &ReturnResultProblem| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ReturnResultProblem(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 2;
            Ok(el_1)
        }(&v),
        OsiRej_problem::returnError(v) => |v_1: &ReturnErrorProblem| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_ReturnErrorProblem(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 3;
            Ok(el_1)
        }(&v),
        OsiRej_problem::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_OsiRej_problem(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "general"));
            }
            Ok(_validate_GeneralProblem(&el)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "invoke"));
            }
            Ok(_validate_InvokeProblem(&el)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "returnResult")
                );
            }
            Ok(_validate_ReturnResultProblem(&el)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "returnError")
                );
            }
            Ok(_validate_ReturnErrorProblem(&el)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ARU-PPDU-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values
{
    single_ASN1_type(X690Element),
}

impl TryFrom<&X690Element>
    for ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values
{
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(el)
    }
}

pub fn _decode_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<
    ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
> {
    match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => Ok(ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(|el: &X690Element| -> ASN1Result<X690Element> {
	Ok(x690_identity(&el.inner()?)?)
}(&el)?)),
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice, "ARU-PPDU-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values")),
	}
}

pub fn _encode_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    value_: &ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
) -> ASN1Result<X690Element> {
    match value_ {
		ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values::single_ASN1_type(v) => |v_1: &X690Element| -> ASN1Result<X690Element> { Ok(X690Element::new(Tag::new(TagClass::CONTEXT, 0), X690Value::from_explicit(&x690_identity(&v_1)?))) }(&v),
	}
}

pub fn _validate_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(
    el: &X690Element,
) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
		(TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
	if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
		return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "single-ASN1-type"));
	}
	Ok(BER.validate_any(&el.inner()?)?)
}(&el),
		_ => return Err(el.to_asn1_err_named(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice, "ARU-PPDU-normal-mode-parameters-user-data-fully-encoded-data-Item-presentation-data-values")),
	}
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ARU-PPDU-normal-mode-parameters-user-data-fully-encoded-data-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item {
    pub presentation_context_identifier: Presentation_context_identifier,
    pub presentation_data_values:
        ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
}
impl ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item {
    pub fn new(
        presentation_context_identifier: Presentation_context_identifier,
        presentation_data_values: ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
    ) -> Self {
        ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item {
            presentation_context_identifier,
            presentation_data_values,
        }
    }
}
impl TryFrom<&X690Element> for ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item(el)
    }
}

pub const _rctl1_components_for_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item: &[ComponentSpec; 2] = &[
	ComponentSpec::new("presentation-context-identifier", false, TagSelector::tag((TagClass::UNIVERSAL, 2)), None, None),
	ComponentSpec::new("presentation-data-values", false, TagSelector::any, None, None)
];

pub const _rctl2_components_for_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item: &[ComponentSpec; 0] = &[

];

pub const _eal_components_for_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item:
    &[ComponentSpec; 0] = &[];

pub fn _decode_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item(
    el: &X690Element,
) -> ASN1Result<ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ARU-PPDU-normal-mode-parameters-user-data-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _eal_components_for_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _rctl2_components_for_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut presentation_context_identifier_: OPTIONAL<Presentation_context_identifier> = None;
    let mut presentation_data_values_: OPTIONAL<
        ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values,
    > = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
			"presentation-context-identifier" => presentation_context_identifier_ = Some(_decode_Presentation_context_identifier(_el)?),
			"presentation-data-values" => presentation_data_values_ = Some(_decode_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(_el)?),
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ARU-PPDU-normal-mode-parameters-user-data-fully-encoded-data-Item")),
		}
    }
    Ok(
        ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item {
            presentation_context_identifier: presentation_context_identifier_.unwrap(),
            presentation_data_values: presentation_data_values_.unwrap(),
        },
    )
}

pub fn _encode_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item(
    value_: &ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_Presentation_context_identifier(
        &value_.presentation_context_identifier,
    )?);
    components_.push(_encode_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(&value_.presentation_data_values)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item(
    el: &X690Element,
) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ARU-PPDU-normal-mode-parameters-user-data-fully-encoded-data-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _eal_components_for_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item,
        _rctl2_components_for_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
			"presentation-context-identifier" => _validate_Presentation_context_identifier(_el)?,
			"presentation-data-values" => _validate_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item_presentation_data_values(_el)?,
			_ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ARU-PPDU-normal-mode-parameters-user-data-fully-encoded-data-Item")),
		}
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ARU-PPDU-normal-mode-parameters-user-data ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum ARU_PPDU_normal_mode_parameters_user_data {
    fully_encoded_data(Vec<ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item>),
}

impl TryFrom<&X690Element> for ARU_PPDU_normal_mode_parameters_user_data {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ARU_PPDU_normal_mode_parameters_user_data(el)
    }
}

pub fn _decode_ARU_PPDU_normal_mode_parameters_user_data(
    el: &X690Element,
) -> ASN1Result<ARU_PPDU_normal_mode_parameters_user_data> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => Ok(
            ARU_PPDU_normal_mode_parameters_user_data::fully_encoded_data(
                |el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item>,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "fully-encoded-data",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<
                        ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item,
                    > = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item(el)?);
                    }
                    Ok(items)
                }(&el)?,
            ),
        ),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "ARU-PPDU-normal-mode-parameters-user-data",
            ))
        }
    }
}

pub fn _encode_ARU_PPDU_normal_mode_parameters_user_data(
    value_: &ARU_PPDU_normal_mode_parameters_user_data,
) -> ASN1Result<X690Element> {
    match value_ {
        ARU_PPDU_normal_mode_parameters_user_data::fully_encoded_data(v) => |v_1: &Vec<
            ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item,
        >|
         -> ASN1Result<
            X690Element,
        > {
            let mut el_1 = |value_: &SEQUENCE_OF<
                ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item,
            >|
             -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(
                        _encode_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item(
                            &v,
                        )?,
                    );
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_1)?;
            el_1.tag.tag_class = TagClass::APPLICATION;
            el_1.tag.tag_number = 1;
            Ok(el_1)
        }(&v),
    }
}

pub fn _validate_ARU_PPDU_normal_mode_parameters_user_data(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::APPLICATION, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::APPLICATION || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fully-encoded-data")
                );
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_ARU_PPDU_normal_mode_parameters_user_data_fully_encoded_data_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "fully-encoded-data",
                    )),
                }
            }(&el)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "ARU-PPDU-normal-mode-parameters-user-data",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ARU-PPDU-normal-mode-parameters ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct ARU_PPDU_normal_mode_parameters {
    pub presentation_context_identifier_list: Presentation_context_identifier_list,
    pub user_data: ARU_PPDU_normal_mode_parameters_user_data,
}
impl ARU_PPDU_normal_mode_parameters {
    pub fn new(
        presentation_context_identifier_list: Presentation_context_identifier_list,
        user_data: ARU_PPDU_normal_mode_parameters_user_data,
    ) -> Self {
        ARU_PPDU_normal_mode_parameters {
            presentation_context_identifier_list,
            user_data,
        }
    }
}
impl TryFrom<&X690Element> for ARU_PPDU_normal_mode_parameters {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ARU_PPDU_normal_mode_parameters(el)
    }
}

pub const _rctl1_components_for_ARU_PPDU_normal_mode_parameters: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "presentation-context-identifier-list",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new("user-data", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_ARU_PPDU_normal_mode_parameters: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ARU_PPDU_normal_mode_parameters: &[ComponentSpec; 0] = &[];

pub fn _decode_ARU_PPDU_normal_mode_parameters(
    el: &X690Element,
) -> ASN1Result<ARU_PPDU_normal_mode_parameters> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ARU-PPDU-normal-mode-parameters",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ARU_PPDU_normal_mode_parameters,
        _eal_components_for_ARU_PPDU_normal_mode_parameters,
        _rctl2_components_for_ARU_PPDU_normal_mode_parameters,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut presentation_context_identifier_list_: OPTIONAL<Presentation_context_identifier_list> =
        None;
    let mut user_data_: OPTIONAL<ARU_PPDU_normal_mode_parameters_user_data> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier-list" => {
                presentation_context_identifier_list_ =
                    Some(_decode_Presentation_context_identifier_list(_el)?)
            }
            "user-data" => {
                user_data_ = Some(_decode_ARU_PPDU_normal_mode_parameters_user_data(_el)?)
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "ARU-PPDU-normal-mode-parameters",
                ))
            }
        }
    }
    Ok(ARU_PPDU_normal_mode_parameters {
        presentation_context_identifier_list: presentation_context_identifier_list_.unwrap(),
        user_data: user_data_.unwrap(),
    })
}

pub fn _encode_ARU_PPDU_normal_mode_parameters(
    value_: &ARU_PPDU_normal_mode_parameters,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(
        |v_1: &Presentation_context_identifier_list| -> ASN1Result<X690Element> {
            let mut el_1 = _encode_Presentation_context_identifier_list(&v_1)?;
            el_1.tag.tag_class = TagClass::CONTEXT;
            el_1.tag.tag_number = 0;
            Ok(el_1)
        }(&value_.presentation_context_identifier_list)?,
    );
    components_.push(_encode_ARU_PPDU_normal_mode_parameters_user_data(
        &value_.user_data,
    )?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ARU_PPDU_normal_mode_parameters(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ARU-PPDU-normal-mode-parameters",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ARU_PPDU_normal_mode_parameters,
        _eal_components_for_ARU_PPDU_normal_mode_parameters,
        _rctl2_components_for_ARU_PPDU_normal_mode_parameters,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier-list" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "presentation-context-identifier-list",
                    ));
                }
                Ok(_validate_Presentation_context_identifier_list(&el)?)
            }(_el)?,
            "user-data" => _validate_ARU_PPDU_normal_mode_parameters_user_data(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "ARU-PPDU-normal-mode-parameters",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Presentation-context-identifier-list-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct Presentation_context_identifier_list_Item {
    pub presentation_context_identifier: Presentation_context_identifier,
    pub transfer_syntax_name: Transfer_syntax_name,
}
impl Presentation_context_identifier_list_Item {
    pub fn new(
        presentation_context_identifier: Presentation_context_identifier,
        transfer_syntax_name: Transfer_syntax_name,
    ) -> Self {
        Presentation_context_identifier_list_Item {
            presentation_context_identifier,
            transfer_syntax_name,
        }
    }
}
impl TryFrom<&X690Element> for Presentation_context_identifier_list_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Presentation_context_identifier_list_Item(el)
    }
}

pub const _rctl1_components_for_Presentation_context_identifier_list_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "presentation-context-identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "transfer-syntax-name",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Presentation_context_identifier_list_Item: &[ComponentSpec; 0] =
    &[];

pub const _eal_components_for_Presentation_context_identifier_list_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_Presentation_context_identifier_list_Item(
    el: &X690Element,
) -> ASN1Result<Presentation_context_identifier_list_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "Presentation-context-identifier-list-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Presentation_context_identifier_list_Item,
        _eal_components_for_Presentation_context_identifier_list_Item,
        _rctl2_components_for_Presentation_context_identifier_list_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut presentation_context_identifier_: OPTIONAL<Presentation_context_identifier> = None;
    let mut transfer_syntax_name_: OPTIONAL<Transfer_syntax_name> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier" => {
                presentation_context_identifier_ =
                    Some(_decode_Presentation_context_identifier(_el)?)
            }
            "transfer-syntax-name" => {
                transfer_syntax_name_ = Some(_decode_Transfer_syntax_name(_el)?)
            }
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "Presentation-context-identifier-list-Item",
                ))
            }
        }
    }
    Ok(Presentation_context_identifier_list_Item {
        presentation_context_identifier: presentation_context_identifier_.unwrap(),
        transfer_syntax_name: transfer_syntax_name_.unwrap(),
    })
}

pub fn _encode_Presentation_context_identifier_list_Item(
    value_: &Presentation_context_identifier_list_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_Presentation_context_identifier(
        &value_.presentation_context_identifier,
    )?);
    components_.push(_encode_Transfer_syntax_name(&value_.transfer_syntax_name)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_Presentation_context_identifier_list_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "Presentation-context-identifier-list-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Presentation_context_identifier_list_Item,
        _eal_components_for_Presentation_context_identifier_list_Item,
        _rctl2_components_for_Presentation_context_identifier_list_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "presentation-context-identifier" => _validate_Presentation_context_identifier(_el)?,
            "transfer-syntax-name" => _validate_Transfer_syntax_name(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "Presentation-context-identifier-list-Item",
                ))
            }
        }
    }
    Ok(())
}
