#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # ServiceAdministration
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `ServiceAdministration`.
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
use crate::DirectoryAbstractService::*;
use crate::InformationFramework::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchRule ::= SEQUENCE {
///   COMPONENTS OF SearchRuleId,
///   serviceType           [1]  OBJECT IDENTIFIER                          OPTIONAL,
///   userClass             [2]  INTEGER                                    OPTIONAL,
///   inputAttributeTypes   [3]  SEQUENCE SIZE (0..MAX) OF RequestAttribute OPTIONAL,
///   attributeCombination  [4]  AttributeCombination                       DEFAULT and:{},
///   outputAttributeTypes  [5]  SEQUENCE SIZE (1..MAX) OF ResultAttribute  OPTIONAL,
///   defaultControls       [6]  ControlOptions                             OPTIONAL,
///   mandatoryControls     [7]  ControlOptions                             OPTIONAL,
///   searchRuleControls    [8]  ControlOptions                             OPTIONAL,
///   familyGrouping        [9]  FamilyGrouping                             OPTIONAL,
///   familyReturn          [10] FamilyReturn                               OPTIONAL,
///   relaxation            [11] RelaxationPolicy                           OPTIONAL,
///   additionalControl     [12] SEQUENCE SIZE (1..MAX) OF AttributeType    OPTIONAL,
///   allowedSubset         [13] AllowedSubset                              DEFAULT '111'B,
///   imposedSubset         [14] ImposedSubset                              OPTIONAL,
///   entryLimit            [15] EntryLimit                                 OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SearchRule {
    pub id: INTEGER,              /* REPLICATED_COMPONENT */
    pub dmdId: OBJECT_IDENTIFIER, /* REPLICATED_COMPONENT */
    pub serviceType: OPTIONAL<OBJECT_IDENTIFIER>,
    pub userClass: OPTIONAL<INTEGER>,
    pub inputAttributeTypes: OPTIONAL<Vec<RequestAttribute>>,
    pub attributeCombination: OPTIONAL<AttributeCombination>,
    pub outputAttributeTypes: OPTIONAL<Vec<ResultAttribute>>,
    pub defaultControls: OPTIONAL<ControlOptions>,
    pub mandatoryControls: OPTIONAL<ControlOptions>,
    pub searchRuleControls: OPTIONAL<ControlOptions>,
    pub familyGrouping: OPTIONAL<FamilyGrouping>,
    pub familyReturn: OPTIONAL<FamilyReturn>,
    pub relaxation: OPTIONAL<RelaxationPolicy>,
    pub additionalControl: OPTIONAL<Vec<AttributeType>>,
    pub allowedSubset: OPTIONAL<AllowedSubset>,
    pub imposedSubset: OPTIONAL<ImposedSubset>,
    pub entryLimit: OPTIONAL<EntryLimit>,
    pub _unrecognized: Vec<X690Element>,
}
impl SearchRule {
    pub fn new(
        id: INTEGER,              /* REPLICATED_COMPONENT */
        dmdId: OBJECT_IDENTIFIER, /* REPLICATED_COMPONENT */
        serviceType: OPTIONAL<OBJECT_IDENTIFIER>,
        userClass: OPTIONAL<INTEGER>,
        inputAttributeTypes: OPTIONAL<Vec<RequestAttribute>>,
        attributeCombination: OPTIONAL<AttributeCombination>,
        outputAttributeTypes: OPTIONAL<Vec<ResultAttribute>>,
        defaultControls: OPTIONAL<ControlOptions>,
        mandatoryControls: OPTIONAL<ControlOptions>,
        searchRuleControls: OPTIONAL<ControlOptions>,
        familyGrouping: OPTIONAL<FamilyGrouping>,
        familyReturn: OPTIONAL<FamilyReturn>,
        relaxation: OPTIONAL<RelaxationPolicy>,
        additionalControl: OPTIONAL<Vec<AttributeType>>,
        allowedSubset: OPTIONAL<AllowedSubset>,
        imposedSubset: OPTIONAL<ImposedSubset>,
        entryLimit: OPTIONAL<EntryLimit>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SearchRule {
            id,
            dmdId,
            serviceType,
            userClass,
            inputAttributeTypes,
            attributeCombination,
            outputAttributeTypes,
            defaultControls,
            mandatoryControls,
            searchRuleControls,
            familyGrouping,
            familyReturn,
            relaxation,
            additionalControl,
            allowedSubset,
            imposedSubset,
            entryLimit,
            _unrecognized,
        }
    }
    pub fn _default_value_for_attributeCombination() -> AttributeCombination {
        AttributeCombination::and(vec![])
    }
    pub fn _default_value_for_allowedSubset() -> AllowedSubset {
        BIT_STRING::from_bin("111")
    }
}
impl TryFrom<X690Element> for SearchRule {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SearchRule(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SearchRule {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SearchRule(el)
    }
}

pub const _rctl1_components_for_SearchRule: &[ComponentSpec; 17] = &[
    ComponentSpec::new(
        "id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "dmdId",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "serviceType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "userClass",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "inputAttributeTypes",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeCombination",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "outputAttributeTypes",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "defaultControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "mandatoryControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "searchRuleControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyGrouping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 9)),
        None,
        None,
    ),
    ComponentSpec::new(
        "familyReturn",
        true,
        TagSelector::tag((TagClass::CONTEXT, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "relaxation",
        true,
        TagSelector::tag((TagClass::CONTEXT, 11)),
        None,
        None,
    ),
    ComponentSpec::new(
        "additionalControl",
        true,
        TagSelector::tag((TagClass::CONTEXT, 12)),
        None,
        None,
    ),
    ComponentSpec::new(
        "allowedSubset",
        true,
        TagSelector::tag((TagClass::CONTEXT, 13)),
        None,
        None,
    ),
    ComponentSpec::new(
        "imposedSubset",
        true,
        TagSelector::tag((TagClass::CONTEXT, 14)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryLimit",
        true,
        TagSelector::tag((TagClass::CONTEXT, 15)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SearchRule: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SearchRule: &[ComponentSpec; 0] = &[];

pub fn _decode_SearchRule(el: &X690Element) -> ASN1Result<SearchRule> {
    |el_: &X690Element| -> ASN1Result<SearchRule> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SearchRule,
            _eal_components_for_SearchRule,
            _rctl2_components_for_SearchRule,
        )?;
        let id = ber_decode_integer(_components.get("id").unwrap())?;
        let dmdId = |el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
            Ok(ber_decode_object_identifier(&el.inner()?)?)
        }(_components.get("dmdId").unwrap())?;
        let serviceType: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("serviceType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(ber_decode_object_identifier(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let userClass: OPTIONAL<INTEGER> = match _components.get("userClass") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let inputAttributeTypes: OPTIONAL<Vec<RequestAttribute>> = match _components
            .get("inputAttributeTypes")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<RequestAttribute>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<RequestAttribute>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<RequestAttribute> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_RequestAttribute(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        let attributeCombination: OPTIONAL<AttributeCombination> =
            match _components.get("attributeCombination") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<AttributeCombination> {
                    Ok(_decode_AttributeCombination(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let outputAttributeTypes: OPTIONAL<Vec<ResultAttribute>> = match _components
            .get("outputAttributeTypes")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<ResultAttribute>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ResultAttribute>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<ResultAttribute> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_ResultAttribute(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        let defaultControls: OPTIONAL<ControlOptions> = match _components.get("defaultControls") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ControlOptions> {
                Ok(_decode_ControlOptions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let mandatoryControls: OPTIONAL<ControlOptions> = match _components.get("mandatoryControls")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ControlOptions> {
                Ok(_decode_ControlOptions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let searchRuleControls: OPTIONAL<ControlOptions> =
            match _components.get("searchRuleControls") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ControlOptions> {
                    Ok(_decode_ControlOptions(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let familyGrouping: OPTIONAL<FamilyGrouping> = match _components.get("familyGrouping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                Ok(_decode_FamilyGrouping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let familyReturn: OPTIONAL<FamilyReturn> = match _components.get("familyReturn") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<FamilyReturn> {
                Ok(_decode_FamilyReturn(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let relaxation: OPTIONAL<RelaxationPolicy> = match _components.get("relaxation") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<RelaxationPolicy> {
                Ok(_decode_RelaxationPolicy(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let additionalControl: OPTIONAL<Vec<AttributeType>> = match _components
            .get("additionalControl")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<AttributeType>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AttributeType>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<AttributeType> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_AttributeType(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        let allowedSubset: OPTIONAL<AllowedSubset> = match _components.get("allowedSubset") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<AllowedSubset> {
                Ok(_decode_AllowedSubset(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let imposedSubset: OPTIONAL<ImposedSubset> = match _components.get("imposedSubset") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<ImposedSubset> {
                Ok(_decode_ImposedSubset(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryLimit: OPTIONAL<EntryLimit> = match _components.get("entryLimit") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<EntryLimit> {
                Ok(_decode_EntryLimit(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(SearchRule {
            id,
            dmdId,
            serviceType,
            userClass,
            inputAttributeTypes,
            attributeCombination,
            outputAttributeTypes,
            defaultControls,
            mandatoryControls,
            searchRuleControls,
            familyGrouping,
            familyReturn,
            relaxation,
            additionalControl,
            allowedSubset,
            imposedSubset,
            entryLimit,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SearchRule(value_: &SearchRule) -> ASN1Result<X690Element> {
    |value_: &SearchRule| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(27);
        components_.push(ber_encode_integer(&value_.id)?);
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    ber_encode_object_identifier(&v_1)?,
                ))),
            ))
        }(&value_.dmdId)?);
        if let Some(v_) = &value_.serviceType {
            components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_object_identifier(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.userClass {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.inputAttributeTypes {
            components_.push(|v_1: &Vec<RequestAttribute>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        RequestAttribute,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_RequestAttribute(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.attributeCombination {
            if *v_ != SearchRule::_default_value_for_attributeCombination() {
                components_.push(|v_1: &AttributeCombination| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_AttributeCombination(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.outputAttributeTypes {
            components_.push(|v_1: &Vec<ResultAttribute>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        ResultAttribute,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ResultAttribute(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.defaultControls {
            components_.push(|v_1: &ControlOptions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    6,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ControlOptions(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.mandatoryControls {
            components_.push(|v_1: &ControlOptions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    7,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ControlOptions(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.searchRuleControls {
            components_.push(|v_1: &ControlOptions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    8,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ControlOptions(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyGrouping {
            components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    9,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyGrouping(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.familyReturn {
            components_.push(|v_1: &FamilyReturn| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    10,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_FamilyReturn(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.relaxation {
            components_.push(|v_1: &RelaxationPolicy| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    11,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_RelaxationPolicy(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.additionalControl {
            components_.push(|v_1: &Vec<AttributeType>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    12,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        AttributeType,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AttributeType(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.allowedSubset {
            if *v_ != SearchRule::_default_value_for_allowedSubset() {
                components_.push(|v_1: &AllowedSubset| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        13,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_AllowedSubset(
                            &v_1,
                        )?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.imposedSubset {
            components_.push(|v_1: &ImposedSubset| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    14,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ImposedSubset(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryLimit {
            components_.push(|v_1: &EntryLimit| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    15,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_EntryLimit(&v_1)?))),
                ))
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
/// SearchRuleId ::= SEQUENCE {
///   id          INTEGER,
///   dmdId  [0]  OBJECT IDENTIFIER }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SearchRuleId {
    pub id: INTEGER,
    pub dmdId: OBJECT_IDENTIFIER,
}
impl SearchRuleId {
    pub fn new(id: INTEGER, dmdId: OBJECT_IDENTIFIER) -> Self {
        SearchRuleId { id, dmdId }
    }
}
impl TryFrom<X690Element> for SearchRuleId {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SearchRuleId(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SearchRuleId {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SearchRuleId(el)
    }
}

pub const _rctl1_components_for_SearchRuleId: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "id",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "dmdId",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SearchRuleId: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SearchRuleId: &[ComponentSpec; 0] = &[];

pub fn _decode_SearchRuleId(el: &X690Element) -> ASN1Result<SearchRuleId> {
    |el_: &X690Element| -> ASN1Result<SearchRuleId> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SearchRuleId,
            _eal_components_for_SearchRuleId,
            _rctl2_components_for_SearchRuleId,
        )?;
        let id = ber_decode_integer(_components.get("id").unwrap())?;
        let dmdId = |el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
            Ok(ber_decode_object_identifier(&el.inner()?)?)
        }(_components.get("dmdId").unwrap())?;
        Ok(SearchRuleId { id, dmdId })
    }(&el)
}

pub fn _encode_SearchRuleId(value_: &SearchRuleId) -> ASN1Result<X690Element> {
    |value_: &SearchRuleId| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_integer(&value_.id)?);
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    ber_encode_object_identifier(&v_1)?,
                ))),
            ))
        }(&value_.dmdId)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AllowedSubset  ::=  BIT STRING {baseObject(0), oneLevel(1), wholeSubtree(2)}
/// ```
pub type AllowedSubset = BIT_STRING;

pub const AllowedSubset_baseObject: BIT = 0; /* LONG_NAMED_BIT */

pub const AllowedSubset_oneLevel: BIT = 1; /* LONG_NAMED_BIT */

pub const AllowedSubset_wholeSubtree: BIT = 2; /* LONG_NAMED_BIT */

pub fn _decode_AllowedSubset(el: &X690Element) -> ASN1Result<AllowedSubset> {
    ber_decode_bit_string(&el)
}

pub fn _encode_AllowedSubset(value_: &AllowedSubset) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ImposedSubset  ::=  ENUMERATED {baseObject(0), oneLevel(1), wholeSubtree(2),...}
/// ```
pub type ImposedSubset = ENUMERATED;

pub const ImposedSubset_baseObject: ImposedSubset = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ImposedSubset_oneLevel: ImposedSubset = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ImposedSubset_wholeSubtree: ImposedSubset = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_ImposedSubset(el: &X690Element) -> ASN1Result<ImposedSubset> {
    ber_decode_enumerated(&el)
}

pub fn _encode_ImposedSubset(value_: &ImposedSubset) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RequestAttribute ::= SEQUENCE {
///   attributeType            ATTRIBUTE.&id({SupportedAttributes}),
///   includeSubtypes     [0]  BOOLEAN DEFAULT FALSE,
///   selectedValues      [1]  SEQUENCE SIZE (0..MAX) OF ATTRIBUTE.&Type
///                            ({SupportedAttributes}{@attributeType}) OPTIONAL,
///   defaultValues       [2]  SEQUENCE SIZE (0..MAX) OF SEQUENCE {
///     entryType                OBJECT-CLASS.&id OPTIONAL,
///     values                   SEQUENCE OF ATTRIBUTE.&Type
///                              ({SupportedAttributes}{@attributeType}),
///                              ...} OPTIONAL,
///   contexts            [3]  SEQUENCE SIZE (0..MAX) OF ContextProfile OPTIONAL,
///   contextCombination  [4]  ContextCombination DEFAULT and:{},
///   matchingUse         [5]  SEQUENCE SIZE (1..MAX) OF MatchingUse OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RequestAttribute {
    pub attributeType: OBJECT_IDENTIFIER,
    pub includeSubtypes: OPTIONAL<BOOLEAN>,
    pub selectedValues: OPTIONAL<Vec<X690Element>>,
    pub defaultValues: OPTIONAL<Vec<RequestAttribute_defaultValues_Item>>,
    pub contexts: OPTIONAL<Vec<ContextProfile>>,
    pub contextCombination: OPTIONAL<ContextCombination>,
    pub matchingUse: OPTIONAL<Vec<MatchingUse>>,
    pub _unrecognized: Vec<X690Element>,
}
impl RequestAttribute {
    pub fn new(
        attributeType: OBJECT_IDENTIFIER,
        includeSubtypes: OPTIONAL<BOOLEAN>,
        selectedValues: OPTIONAL<Vec<X690Element>>,
        defaultValues: OPTIONAL<Vec<RequestAttribute_defaultValues_Item>>,
        contexts: OPTIONAL<Vec<ContextProfile>>,
        contextCombination: OPTIONAL<ContextCombination>,
        matchingUse: OPTIONAL<Vec<MatchingUse>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        RequestAttribute {
            attributeType,
            includeSubtypes,
            selectedValues,
            defaultValues,
            contexts,
            contextCombination,
            matchingUse,
            _unrecognized,
        }
    }
    pub fn _default_value_for_includeSubtypes() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_contextCombination() -> ContextCombination {
        ContextCombination::and(vec![])
    }
}
impl TryFrom<X690Element> for RequestAttribute {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RequestAttribute(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RequestAttribute {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RequestAttribute(el)
    }
}

pub const _rctl1_components_for_RequestAttribute: &[ComponentSpec; 7] = &[
    ComponentSpec::new(
        "attributeType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "includeSubtypes",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "selectedValues",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "defaultValues",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "contexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "contextCombination",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "matchingUse",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RequestAttribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RequestAttribute: &[ComponentSpec; 0] = &[];

pub fn _decode_RequestAttribute(el: &X690Element) -> ASN1Result<RequestAttribute> {
    |el_: &X690Element| -> ASN1Result<RequestAttribute> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RequestAttribute,
            _eal_components_for_RequestAttribute,
            _rctl2_components_for_RequestAttribute,
        )?;
        let attributeType =
            ber_decode_object_identifier(_components.get("attributeType").unwrap())?;
        let includeSubtypes: OPTIONAL<BOOLEAN> = match _components.get("includeSubtypes") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let selectedValues: OPTIONAL<Vec<X690Element>> = match _components.get("selectedValues") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<X690Element>> {
                Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<X690Element>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<X690Element> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(x690_identity(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let defaultValues: OPTIONAL<Vec<RequestAttribute_defaultValues_Item>> =
            match _components.get("defaultValues") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<
                    Vec<RequestAttribute_defaultValues_Item>,
                > {
                    Ok(|el: &X690Element| -> ASN1Result<
                        SEQUENCE_OF<RequestAttribute_defaultValues_Item>,
                    > {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<RequestAttribute_defaultValues_Item> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_RequestAttribute_defaultValues_Item(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let contexts: OPTIONAL<Vec<ContextProfile>> = match _components.get("contexts") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<ContextProfile>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ContextProfile>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<ContextProfile> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_ContextProfile(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        let contextCombination: OPTIONAL<ContextCombination> =
            match _components.get("contextCombination") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ContextCombination> {
                    Ok(_decode_ContextCombination(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let matchingUse: OPTIONAL<Vec<MatchingUse>> = match _components.get("matchingUse") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<MatchingUse>> {
                Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<MatchingUse>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<MatchingUse> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_MatchingUse(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(RequestAttribute {
            attributeType,
            includeSubtypes,
            selectedValues,
            defaultValues,
            contexts,
            contextCombination,
            matchingUse,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_RequestAttribute(value_: &RequestAttribute) -> ASN1Result<X690Element> {
    |value_: &RequestAttribute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(17);
        components_.push(ber_encode_object_identifier(&value_.attributeType)?);
        if let Some(v_) = &value_.includeSubtypes {
            if *v_ != RequestAttribute::_default_value_for_includeSubtypes() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.selectedValues {
            components_.push(|v_1: &Vec<X690Element>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        X690Element,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(x690_identity(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.defaultValues {
            components_.push(
                |v_1: &Vec<RequestAttribute_defaultValues_Item>| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(
                            Box::new(|value_: &SEQUENCE_OF<
                                RequestAttribute_defaultValues_Item,
                            >|
                             -> ASN1Result<X690Element> {
                                let mut children: Vec<X690Element> =
                                    Vec::with_capacity(value_.len());
                                for v in value_ {
                                    children.push(_encode_RequestAttribute_defaultValues_Item(&v)?);
                                }
                                Ok(X690Element::new(
                                    TagClass::UNIVERSAL,
                                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                                    Arc::new(X690Encoding::Constructed(children)),
                                ))
                            }(&v_1)?),
                        )),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.contexts {
            components_.push(|v_1: &Vec<ContextProfile>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        ContextProfile,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ContextProfile(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.contextCombination {
            if *v_ != RequestAttribute::_default_value_for_contextCombination() {
                components_.push(|v_1: &ContextCombination| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_ContextCombination(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.matchingUse {
            components_.push(|v_1: &Vec<MatchingUse>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        MatchingUse,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_MatchingUse(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
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
/// ContextProfile ::= SEQUENCE {
///   contextType   CONTEXT.&id({SupportedContexts}),
///   contextValue  SEQUENCE SIZE (1..MAX) OF CONTEXT.&Assertion
///                  ({SupportedContexts}{@contextType}) OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ContextProfile {
    pub contextType: OBJECT_IDENTIFIER,
    pub contextValue: OPTIONAL<Vec<X690Element>>,
    pub _unrecognized: Vec<X690Element>,
}
impl ContextProfile {
    pub fn new(
        contextType: OBJECT_IDENTIFIER,
        contextValue: OPTIONAL<Vec<X690Element>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ContextProfile {
            contextType,
            contextValue,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ContextProfile {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ContextProfile(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ContextProfile {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ContextProfile(el)
    }
}

pub const _rctl1_components_for_ContextProfile: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "contextType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "contextValue",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ContextProfile: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ContextProfile: &[ComponentSpec; 0] = &[];

pub fn _decode_ContextProfile(el: &X690Element) -> ASN1Result<ContextProfile> {
    |el_: &X690Element| -> ASN1Result<ContextProfile> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ContextProfile,
            _eal_components_for_ContextProfile,
            _rctl2_components_for_ContextProfile,
        )?;
        let contextType = ber_decode_object_identifier(_components.get("contextType").unwrap())?;
        let contextValue: OPTIONAL<Vec<X690Element>> = match _components.get("contextValue") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<X690Element>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SEQUENCE_OF<X690Element> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(x690_identity(el)?);
                }
                Ok(items)
            }(c_)?),
            _ => None,
        };
        Ok(ContextProfile {
            contextType,
            contextValue,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ContextProfile(value_: &ContextProfile) -> ASN1Result<X690Element> {
    |value_: &ContextProfile| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.contextType)?);
        if let Some(v_) = &value_.contextValue {
            components_.push(
                |value_: &SEQUENCE_OF<X690Element>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(x690_identity(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
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
/// ContextCombination  ::=  CHOICE {
///   context  [0]  CONTEXT.&id({SupportedContexts}),
///   and      [1]  SEQUENCE OF ContextCombination,
///   or       [2]  SEQUENCE OF ContextCombination,
///   not      [3]  ContextCombination,
///   ... }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ContextCombination {
    context(OBJECT_IDENTIFIER),
    and(Vec<Box<ContextCombination>>),
    or(Vec<Box<ContextCombination>>),
    not(Box<ContextCombination>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ContextCombination {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ContextCombination(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ContextCombination {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ContextCombination(el)
    }
}

pub fn _decode_ContextCombination(el: &X690Element) -> ASN1Result<ContextCombination> {
    |el: &X690Element| -> ASN1Result<ContextCombination> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(ContextCombination::context(
                ber_decode_object_identifier(&el)?,
            )),
            (TagClass::CONTEXT, 1) => {
                Ok(ContextCombination::and(|el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<Box<ContextCombination>>,
                > {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<Box<ContextCombination>> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(Box::new(_decode_ContextCombination(el)?));
                    }
                    Ok(items)
                }(&el)?))
            }
            (TagClass::CONTEXT, 2) => Ok(ContextCombination::or(|el: &X690Element| -> ASN1Result<
                SEQUENCE_OF<Box<ContextCombination>>,
            > {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SEQUENCE_OF<Box<ContextCombination>> =
                    Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(Box::new(_decode_ContextCombination(el)?));
                }
                Ok(items)
            }(&el)?)),
            (TagClass::CONTEXT, 3) => {
                Ok(ContextCombination::not(|el: &X690Element| -> ASN1Result<
                    Box<ContextCombination>,
                > {
                    Ok(Box::new(_decode_ContextCombination(&el.inner()?)?))
                }(&el)?))
            }
            _ => Ok(ContextCombination::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ContextCombination(value_: &ContextCombination) -> ASN1Result<X690Element> {
    |value: &ContextCombination| -> ASN1Result<X690Element> {
        match value {
            ContextCombination::context(v) => {
                |v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_object_identifier(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            ContextCombination::and(v) => {
                |v_1: &Vec<Box<ContextCombination>>| -> ASN1Result<X690Element> {
                    let mut el_1 =
                        |value_: &SEQUENCE_OF<Box<ContextCombination>>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_ContextCombination(&v)?);
                            }
                            Ok(X690Element::new(
                                TagClass::UNIVERSAL,
                                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                                Arc::new(X690Encoding::Constructed(children)),
                            ))
                        }(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            ContextCombination::or(v) => {
                |v_1: &Vec<Box<ContextCombination>>| -> ASN1Result<X690Element> {
                    let mut el_1 =
                        |value_: &SEQUENCE_OF<Box<ContextCombination>>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_ContextCombination(&v)?);
                            }
                            Ok(X690Element::new(
                                TagClass::UNIVERSAL,
                                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                                Arc::new(X690Encoding::Constructed(children)),
                            ))
                        }(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v)
            }
            ContextCombination::not(v) => |v_1: &ContextCombination| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_ContextCombination(&v_1)?,
                    ))),
                ))
            }(&v),
            ContextCombination::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MatchingUse ::= SEQUENCE {
///   restrictionType    MATCHING-RESTRICTION.&id({SupportedMatchingRestrictions}),
///   restrictionValue   MATCHING-RESTRICTION.&Restriction
///                         ({SupportedMatchingRestrictions}{@restrictionType}),
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct MatchingUse {
    pub restrictionType: OBJECT_IDENTIFIER,
    pub restrictionValue: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl MatchingUse {
    pub fn new(
        restrictionType: OBJECT_IDENTIFIER,
        restrictionValue: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        MatchingUse {
            restrictionType,
            restrictionValue,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for MatchingUse {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_MatchingUse(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for MatchingUse {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_MatchingUse(el)
    }
}

pub const _rctl1_components_for_MatchingUse: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "restrictionType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("restrictionValue", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_MatchingUse: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MatchingUse: &[ComponentSpec; 0] = &[];

pub fn _decode_MatchingUse(el: &X690Element) -> ASN1Result<MatchingUse> {
    |el_: &X690Element| -> ASN1Result<MatchingUse> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_MatchingUse,
            _eal_components_for_MatchingUse,
            _rctl2_components_for_MatchingUse,
        )?;
        let restrictionType =
            ber_decode_object_identifier(_components.get("restrictionType").unwrap())?;
        let restrictionValue = x690_identity(_components.get("restrictionValue").unwrap())?;
        Ok(MatchingUse {
            restrictionType,
            restrictionValue,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_MatchingUse(value_: &MatchingUse) -> ASN1Result<X690Element> {
    |value_: &MatchingUse| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.restrictionType)?);
        components_.push(x690_identity(&value_.restrictionValue)?);
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
/// SupportedMatchingRestrictions MATCHING-RESTRICTION ::= {...}
/// ```
///
///
pub fn SupportedMatchingRestrictions() -> Vec<MATCHING_RESTRICTION> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeCombination  ::=  CHOICE {
///   attribute  [0]  AttributeType,
///   and        [1]  SEQUENCE OF AttributeCombination,
///   or         [2]  SEQUENCE OF AttributeCombination,
///   not        [3]  AttributeCombination,
///   ... }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeCombination {
    attribute(AttributeType),
    and(Vec<Box<AttributeCombination>>),
    or(Vec<Box<AttributeCombination>>),
    not(Box<AttributeCombination>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for AttributeCombination {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCombination(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeCombination {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCombination(el)
    }
}

pub fn _decode_AttributeCombination(el: &X690Element) -> ASN1Result<AttributeCombination> {
    |el: &X690Element| -> ASN1Result<AttributeCombination> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => {
                Ok(AttributeCombination::attribute(_decode_AttributeType(&el)?))
            }
            (TagClass::CONTEXT, 1) => {
                Ok(AttributeCombination::and(|el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<Box<AttributeCombination>>,
                > {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<Box<AttributeCombination>> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(Box::new(_decode_AttributeCombination(el)?));
                    }
                    Ok(items)
                }(&el)?))
            }
            (TagClass::CONTEXT, 2) => {
                Ok(AttributeCombination::or(|el: &X690Element| -> ASN1Result<
                    SEQUENCE_OF<Box<AttributeCombination>>,
                > {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<Box<AttributeCombination>> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(Box::new(_decode_AttributeCombination(el)?));
                    }
                    Ok(items)
                }(&el)?))
            }
            (TagClass::CONTEXT, 3) => {
                Ok(AttributeCombination::not(|el: &X690Element| -> ASN1Result<
                    Box<AttributeCombination>,
                > {
                    Ok(Box::new(_decode_AttributeCombination(&el.inner()?)?))
                }(&el)?))
            }
            _ => Ok(AttributeCombination::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_AttributeCombination(value_: &AttributeCombination) -> ASN1Result<X690Element> {
    |value: &AttributeCombination| -> ASN1Result<X690Element> {
        match value {
            AttributeCombination::attribute(v) => {
                |v_1: &AttributeType| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_AttributeType(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            AttributeCombination::and(v) => {
                |v_1: &Vec<Box<AttributeCombination>>| -> ASN1Result<X690Element> {
                    let mut el_1 =
                        |value_: &SEQUENCE_OF<Box<AttributeCombination>>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_AttributeCombination(&v)?);
                            }
                            Ok(X690Element::new(
                                TagClass::UNIVERSAL,
                                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                                Arc::new(X690Encoding::Constructed(children)),
                            ))
                        }(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            AttributeCombination::or(v) => {
                |v_1: &Vec<Box<AttributeCombination>>| -> ASN1Result<X690Element> {
                    let mut el_1 =
                        |value_: &SEQUENCE_OF<Box<AttributeCombination>>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_AttributeCombination(&v)?);
                            }
                            Ok(X690Element::new(
                                TagClass::UNIVERSAL,
                                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                                Arc::new(X690Encoding::Constructed(children)),
                            ))
                        }(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v)
            }
            AttributeCombination::not(v) => {
                |v_1: &AttributeCombination| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_AttributeCombination(&v_1)?,
                        ))),
                    ))
                }(&v)
            }
            AttributeCombination::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ResultAttribute ::= SEQUENCE {
///   attributeType      ATTRIBUTE.&id({SupportedAttributes}),
///   outputValues       CHOICE {
///     selectedValues     SEQUENCE OF ATTRIBUTE.&Type
///                        ({SupportedAttributes}{@attributeType}),
///     matchedValuesOnly  NULL } OPTIONAL,
///   contexts      [0]  SEQUENCE SIZE (1..MAX) OF ContextProfile OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ResultAttribute {
    pub attributeType: OBJECT_IDENTIFIER,
    pub outputValues: OPTIONAL<ResultAttribute_outputValues>,
    pub contexts: OPTIONAL<Vec<ContextProfile>>,
    pub _unrecognized: Vec<X690Element>,
}
impl ResultAttribute {
    pub fn new(
        attributeType: OBJECT_IDENTIFIER,
        outputValues: OPTIONAL<ResultAttribute_outputValues>,
        contexts: OPTIONAL<Vec<ContextProfile>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ResultAttribute {
            attributeType,
            outputValues,
            contexts,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ResultAttribute {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ResultAttribute(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ResultAttribute {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ResultAttribute(el)
    }
}

pub const _rctl1_components_for_ResultAttribute: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "attributeType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "outputValues",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 16)),
            &TagSelector::tag((TagClass::UNIVERSAL, 5)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "contexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ResultAttribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ResultAttribute: &[ComponentSpec; 0] = &[];

pub fn _decode_ResultAttribute(el: &X690Element) -> ASN1Result<ResultAttribute> {
    |el_: &X690Element| -> ASN1Result<ResultAttribute> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ResultAttribute,
            _eal_components_for_ResultAttribute,
            _rctl2_components_for_ResultAttribute,
        )?;
        let attributeType =
            ber_decode_object_identifier(_components.get("attributeType").unwrap())?;
        let outputValues: OPTIONAL<ResultAttribute_outputValues> =
            match _components.get("outputValues") {
                Some(c_) => Some(_decode_ResultAttribute_outputValues(c_)?),
                _ => None,
            };
        let contexts: OPTIONAL<Vec<ContextProfile>> = match _components.get("contexts") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<ContextProfile>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ContextProfile>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<ContextProfile> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_ContextProfile(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        Ok(ResultAttribute {
            attributeType,
            outputValues,
            contexts,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ResultAttribute(value_: &ResultAttribute) -> ASN1Result<X690Element> {
    |value_: &ResultAttribute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(ber_encode_object_identifier(&value_.attributeType)?);
        if let Some(v_) = &value_.outputValues {
            components_.push(_encode_ResultAttribute_outputValues(&v_)?);
        }
        if let Some(v_) = &value_.contexts {
            components_.push(|v_1: &Vec<ContextProfile>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        ContextProfile,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ContextProfile(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
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
/// ControlOptions ::= SEQUENCE {
///   serviceControls   [0]  ServiceControlOptions DEFAULT {},
///   searchOptions     [1]  SearchControlOptions  DEFAULT {searchAliases},
///   hierarchyOptions  [2]  HierarchySelections   OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ControlOptions {
    pub serviceControls: OPTIONAL<ServiceControlOptions>,
    pub searchOptions: OPTIONAL<SearchControlOptions>,
    pub hierarchyOptions: OPTIONAL<HierarchySelections>,
    pub _unrecognized: Vec<X690Element>,
}
impl ControlOptions {
    pub fn new(
        serviceControls: OPTIONAL<ServiceControlOptions>,
        searchOptions: OPTIONAL<SearchControlOptions>,
        hierarchyOptions: OPTIONAL<HierarchySelections>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ControlOptions {
            serviceControls,
            searchOptions,
            hierarchyOptions,
            _unrecognized,
        }
    }
    pub fn _default_value_for_serviceControls() -> ServiceControlOptions {
        BIT_STRING::new()
    }
    pub fn _default_value_for_searchOptions() -> SearchControlOptions {
        BIT_STRING::with_bits_set(&[SearchControlOptions_searchAliases])
    }
}
impl Default for ControlOptions {
    fn default() -> Self {
        ControlOptions {
            serviceControls: None,
            searchOptions: None,
            hierarchyOptions: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for ControlOptions {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ControlOptions(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ControlOptions {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ControlOptions(el)
    }
}

pub const _rctl1_components_for_ControlOptions: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "serviceControls",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "searchOptions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "hierarchyOptions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ControlOptions: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ControlOptions: &[ComponentSpec; 0] = &[];

pub fn _decode_ControlOptions(el: &X690Element) -> ASN1Result<ControlOptions> {
    |el_: &X690Element| -> ASN1Result<ControlOptions> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ControlOptions,
            _eal_components_for_ControlOptions,
            _rctl2_components_for_ControlOptions,
        )?;
        let serviceControls: OPTIONAL<ServiceControlOptions> =
            match _components.get("serviceControls") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<ServiceControlOptions> {
                    Ok(_decode_ServiceControlOptions(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let searchOptions: OPTIONAL<SearchControlOptions> = match _components.get("searchOptions") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<SearchControlOptions> {
                Ok(_decode_SearchControlOptions(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let hierarchyOptions: OPTIONAL<HierarchySelections> =
            match _components.get("hierarchyOptions") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<HierarchySelections> {
                    Ok(_decode_HierarchySelections(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(ControlOptions {
            serviceControls,
            searchOptions,
            hierarchyOptions,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ControlOptions(value_: &ControlOptions) -> ASN1Result<X690Element> {
    |value_: &ControlOptions| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.serviceControls {
            if *v_ != ControlOptions::_default_value_for_serviceControls() {
                components_.push(|v_1: &ServiceControlOptions| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_ServiceControlOptions(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.searchOptions {
            if *v_ != ControlOptions::_default_value_for_searchOptions() {
                components_.push(|v_1: &SearchControlOptions| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_SearchControlOptions(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.hierarchyOptions {
            components_.push(|v_1: &HierarchySelections| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_HierarchySelections(&v_1)?,
                    ))),
                ))
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
/// EntryLimit ::= SEQUENCE {
///   default  INTEGER,
///   max      INTEGER,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EntryLimit {
    pub default: INTEGER,
    pub max: INTEGER,
    pub _unrecognized: Vec<X690Element>,
}
impl EntryLimit {
    pub fn new(default: INTEGER, max: INTEGER, _unrecognized: Vec<X690Element>) -> Self {
        EntryLimit {
            default,
            max,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for EntryLimit {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EntryLimit(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EntryLimit {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EntryLimit(el)
    }
}

pub const _rctl1_components_for_EntryLimit: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "default",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "max",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EntryLimit: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EntryLimit: &[ComponentSpec; 0] = &[];

pub fn _decode_EntryLimit(el: &X690Element) -> ASN1Result<EntryLimit> {
    |el_: &X690Element| -> ASN1Result<EntryLimit> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EntryLimit,
            _eal_components_for_EntryLimit,
            _rctl2_components_for_EntryLimit,
        )?;
        let default = ber_decode_integer(_components.get("default").unwrap())?;
        let max = ber_decode_integer(_components.get("max").unwrap())?;
        Ok(EntryLimit {
            default,
            max,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_EntryLimit(value_: &EntryLimit) -> ASN1Result<X690Element> {
    |value_: &EntryLimit| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_integer(&value_.default)?);
        components_.push(ber_encode_integer(&value_.max)?);
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
/// RelaxationPolicy ::= SEQUENCE {
///   basic        [0]  MRMapping DEFAULT {},
///   tightenings  [1]  SEQUENCE SIZE (1..MAX) OF MRMapping OPTIONAL,
///   relaxations  [2]  SEQUENCE SIZE (1..MAX) OF MRMapping OPTIONAL,
///   maximum      [3]  INTEGER OPTIONAL, -- mandatory if tightenings is present
///   minimum      [4]  INTEGER DEFAULT 1,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RelaxationPolicy {
    pub basic: OPTIONAL<MRMapping>,
    pub tightenings: OPTIONAL<Vec<MRMapping>>,
    pub relaxations: OPTIONAL<Vec<MRMapping>>,
    pub maximum: OPTIONAL<INTEGER>,
    pub minimum: OPTIONAL<INTEGER>,
    pub _unrecognized: Vec<X690Element>,
}
impl RelaxationPolicy {
    pub fn new(
        basic: OPTIONAL<MRMapping>,
        tightenings: OPTIONAL<Vec<MRMapping>>,
        relaxations: OPTIONAL<Vec<MRMapping>>,
        maximum: OPTIONAL<INTEGER>,
        minimum: OPTIONAL<INTEGER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        RelaxationPolicy {
            basic,
            tightenings,
            relaxations,
            maximum,
            minimum,
            _unrecognized,
        }
    }
    pub fn _default_value_for_basic() -> MRMapping {
        MRMapping {
            mapping: None,
            substitution: None,
            ..Default::default()
        }
    }
    pub fn _default_value_for_minimum() -> INTEGER {
        1
    }
}
impl Default for RelaxationPolicy {
    fn default() -> Self {
        RelaxationPolicy {
            basic: None,
            tightenings: None,
            relaxations: None,
            maximum: None,
            minimum: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for RelaxationPolicy {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RelaxationPolicy(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RelaxationPolicy {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RelaxationPolicy(el)
    }
}

pub const _rctl1_components_for_RelaxationPolicy: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "basic",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "tightenings",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "relaxations",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "maximum",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "minimum",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RelaxationPolicy: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RelaxationPolicy: &[ComponentSpec; 0] = &[];

pub fn _decode_RelaxationPolicy(el: &X690Element) -> ASN1Result<RelaxationPolicy> {
    |el_: &X690Element| -> ASN1Result<RelaxationPolicy> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RelaxationPolicy,
            _eal_components_for_RelaxationPolicy,
            _rctl2_components_for_RelaxationPolicy,
        )?;
        let basic: OPTIONAL<MRMapping> = match _components.get("basic") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<MRMapping> {
                Ok(_decode_MRMapping(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let tightenings: OPTIONAL<Vec<MRMapping>> = match _components.get("tightenings") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<MRMapping>> {
                Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<MRMapping>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<MRMapping> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_MRMapping(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let relaxations: OPTIONAL<Vec<MRMapping>> = match _components.get("relaxations") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<MRMapping>> {
                Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<MRMapping>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<MRMapping> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_MRMapping(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let maximum: OPTIONAL<INTEGER> = match _components.get("maximum") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let minimum: OPTIONAL<INTEGER> = match _components.get("minimum") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(RelaxationPolicy {
            basic,
            tightenings,
            relaxations,
            maximum,
            minimum,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_RelaxationPolicy(value_: &RelaxationPolicy) -> ASN1Result<X690Element> {
    |value_: &RelaxationPolicy| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        if let Some(v_) = &value_.basic {
            if *v_ != RelaxationPolicy::_default_value_for_basic() {
                components_.push(|v_1: &MRMapping| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_MRMapping(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.tightenings {
            components_.push(|v_1: &Vec<MRMapping>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        MRMapping,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_MRMapping(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.relaxations {
            components_.push(|v_1: &Vec<MRMapping>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        MRMapping,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_MRMapping(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.maximum {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.minimum {
            if *v_ != RelaxationPolicy::_default_value_for_minimum() {
                components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                    ))
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
/// MRMapping ::= SEQUENCE {
///   mapping       [0]  SEQUENCE SIZE (1..MAX) OF Mapping OPTIONAL,
///   substitution  [1]  SEQUENCE SIZE (1..MAX) OF MRSubstitution OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone, PartialEq)]
pub struct MRMapping {
    pub mapping: OPTIONAL<Vec<Mapping>>,
    pub substitution: OPTIONAL<Vec<MRSubstitution>>,
    pub _unrecognized: Vec<X690Element>,
}
impl MRMapping {
    pub fn new(
        mapping: OPTIONAL<Vec<Mapping>>,
        substitution: OPTIONAL<Vec<MRSubstitution>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        MRMapping {
            mapping,
            substitution,
            _unrecognized,
        }
    }
}
impl Default for MRMapping {
    fn default() -> Self {
        MRMapping {
            mapping: None,
            substitution: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for MRMapping {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_MRMapping(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for MRMapping {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_MRMapping(el)
    }
}

pub const _rctl1_components_for_MRMapping: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "mapping",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "substitution",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_MRMapping: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MRMapping: &[ComponentSpec; 0] = &[];

pub fn _decode_MRMapping(el: &X690Element) -> ASN1Result<MRMapping> {
    |el_: &X690Element| -> ASN1Result<MRMapping> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_MRMapping,
            _eal_components_for_MRMapping,
            _rctl2_components_for_MRMapping,
        )?;
        let mapping: OPTIONAL<Vec<Mapping>> = match _components.get("mapping") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<Mapping>> {
                Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<Mapping>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<Mapping> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_Mapping(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let substitution: OPTIONAL<Vec<MRSubstitution>> = match _components.get("substitution") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<MRSubstitution>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<MRSubstitution>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<MRSubstitution> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_MRSubstitution(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        Ok(MRMapping {
            mapping,
            substitution,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_MRMapping(value_: &MRMapping) -> ASN1Result<X690Element> {
    |value_: &MRMapping| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.mapping {
            components_.push(|v_1: &Vec<Mapping>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        Mapping,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_Mapping(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.substitution {
            components_.push(|v_1: &Vec<MRSubstitution>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SEQUENCE_OF<
                        MRSubstitution,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_MRSubstitution(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
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
/// Mapping ::= SEQUENCE {
///   mappingFunction  OBJECT IDENTIFIER (CONSTRAINED BY {-- shall be an--
///                      -- object identifier of a mapping-based matching algorithm -- }),
///   level            INTEGER DEFAULT 0,
///   ... }
/// ```
///
///
#[derive(Debug, Clone, PartialEq)]
pub struct Mapping {
    pub mappingFunction: OBJECT_IDENTIFIER,
    pub level: OPTIONAL<INTEGER>,
    pub _unrecognized: Vec<X690Element>,
}
impl Mapping {
    pub fn new(
        mappingFunction: OBJECT_IDENTIFIER,
        level: OPTIONAL<INTEGER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Mapping {
            mappingFunction,
            level,
            _unrecognized,
        }
    }
    pub fn _default_value_for_level() -> INTEGER {
        0
    }
}
impl TryFrom<X690Element> for Mapping {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Mapping(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Mapping {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Mapping(el)
    }
}

pub const _rctl1_components_for_Mapping: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "mappingFunction",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "level",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Mapping: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Mapping: &[ComponentSpec; 0] = &[];

pub fn _decode_Mapping(el: &X690Element) -> ASN1Result<Mapping> {
    |el_: &X690Element| -> ASN1Result<Mapping> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Mapping,
            _eal_components_for_Mapping,
            _rctl2_components_for_Mapping,
        )?;
        let mappingFunction =
            ber_decode_object_identifier(_components.get("mappingFunction").unwrap())?;
        let level: OPTIONAL<INTEGER> = match _components.get("level") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        Ok(Mapping {
            mappingFunction,
            level,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Mapping(value_: &Mapping) -> ASN1Result<X690Element> {
    |value_: &Mapping| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.mappingFunction)?);
        if let Some(v_) = &value_.level {
            if *v_ != Mapping::_default_value_for_level() {
                components_.push(ber_encode_integer(&v_)?);
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
/// MRSubstitution ::= SEQUENCE {
///   attribute             AttributeType,
///   oldMatchingRule  [0]  MATCHING-RULE.&id OPTIONAL,
///   newMatchingRule  [1]  MATCHING-RULE.&id OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone, PartialEq)]
pub struct MRSubstitution {
    pub attribute: AttributeType,
    pub oldMatchingRule: OPTIONAL<OBJECT_IDENTIFIER>,
    pub newMatchingRule: OPTIONAL<OBJECT_IDENTIFIER>,
    pub _unrecognized: Vec<X690Element>,
}
impl MRSubstitution {
    pub fn new(
        attribute: AttributeType,
        oldMatchingRule: OPTIONAL<OBJECT_IDENTIFIER>,
        newMatchingRule: OPTIONAL<OBJECT_IDENTIFIER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        MRSubstitution {
            attribute,
            oldMatchingRule,
            newMatchingRule,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for MRSubstitution {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_MRSubstitution(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for MRSubstitution {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_MRSubstitution(el)
    }
}

pub const _rctl1_components_for_MRSubstitution: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "attribute",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "oldMatchingRule",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "newMatchingRule",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_MRSubstitution: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MRSubstitution: &[ComponentSpec; 0] = &[];

pub fn _decode_MRSubstitution(el: &X690Element) -> ASN1Result<MRSubstitution> {
    |el_: &X690Element| -> ASN1Result<MRSubstitution> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_MRSubstitution,
            _eal_components_for_MRSubstitution,
            _rctl2_components_for_MRSubstitution,
        )?;
        let attribute = _decode_AttributeType(_components.get("attribute").unwrap())?;
        let oldMatchingRule: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("oldMatchingRule")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(ber_decode_object_identifier(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let newMatchingRule: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("newMatchingRule")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(ber_decode_object_identifier(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(MRSubstitution {
            attribute,
            oldMatchingRule,
            newMatchingRule,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_MRSubstitution(value_: &MRSubstitution) -> ASN1Result<X690Element> {
    |value_: &MRSubstitution| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_AttributeType(&value_.attribute)?);
        if let Some(v_) = &value_.oldMatchingRule {
            components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_object_identifier(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.newMatchingRule {
            components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_object_identifier(&v_1)?,
                    ))),
                ))
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
/// SEARCH-RULE ::= CLASS {
///   &dmdId                 OBJECT IDENTIFIER,
///   &serviceType           OBJECT IDENTIFIER               OPTIONAL,
///   &userClass             INTEGER                         OPTIONAL,
///   &InputAttributeTypes   REQUEST-ATTRIBUTE               OPTIONAL,
///   &combination           AttributeCombination            OPTIONAL,
///   &OutputAttributeTypes  RESULT-ATTRIBUTE                OPTIONAL,
///   &defaultControls       ControlOptions                  OPTIONAL,
///   &mandatoryControls     ControlOptions                  OPTIONAL,
///   &searchRuleControls    ControlOptions                  OPTIONAL,
///   &familyGrouping        FamilyGrouping                  OPTIONAL,
///   &familyReturn          FamilyReturn                    OPTIONAL,
///   &additionalControl     AttributeType                   OPTIONAL,
///   &relaxation            RelaxationPolicy                OPTIONAL,
///   &allowedSubset         AllowedSubset                   DEFAULT '111'B,
///   &imposedSubset         ImposedSubset                   OPTIONAL,
///   &entryLimit            EntryLimit                      OPTIONAL,
///   &id                    INTEGER                         UNIQUE }
/// WITH SYNTAX {
///   DMD ID                 &dmdId
///   [SERVICE-TYPE          &serviceType]
///   [USER-CLASS            &userClass]
///   [INPUT ATTRIBUTES      &InputAttributeTypes]
///   [COMBINATION           &combination]
///   [OUTPUT ATTRIBUTES     &OutputAttributeTypes]
///   [DEFAULT CONTROL       &defaultControls]
///   [MANDATORY CONTROL     &mandatoryControls]
///   [SEARCH-RULE CONTROL   &searchRuleControls]
///   [FAMILY-GROUPING       &familyGrouping]
///   [FAMILY-RETURN         &familyReturn]
///   [ADDITIONAL CONTROL    &additionalControl]
///   [RELAXATION            &relaxation]
///   [ALLOWED SUBSET        &allowedSubset]
///   [IMPOSED SUBSET        &imposedSubset]
///   [ENTRY LIMIT           &entryLimit]
///   ID                     &id }
/// ```
///
#[derive(Debug)]
pub struct SEARCH_RULE {
    pub dmdId: OBJECT_IDENTIFIER,
    pub serviceType: OPTIONAL<OBJECT_IDENTIFIER>,
    pub userClass: OPTIONAL<INTEGER>,
    pub InputAttributeTypes: OPTIONAL<Vec<REQUEST_ATTRIBUTE>>,
    pub combination: OPTIONAL<AttributeCombination>,
    pub OutputAttributeTypes: OPTIONAL<Vec<RESULT_ATTRIBUTE>>,
    pub defaultControls: OPTIONAL<ControlOptions>,
    pub mandatoryControls: OPTIONAL<ControlOptions>,
    pub searchRuleControls: OPTIONAL<ControlOptions>,
    pub familyGrouping: OPTIONAL<FamilyGrouping>,
    pub familyReturn: OPTIONAL<FamilyReturn>,
    pub additionalControl: OPTIONAL<AttributeType>,
    pub relaxation: OPTIONAL<RelaxationPolicy>,
    pub allowedSubset: OPTIONAL<AllowedSubset>,
    pub imposedSubset: OPTIONAL<ImposedSubset>,
    pub entryLimit: OPTIONAL<EntryLimit>,
    pub id: INTEGER,
}
impl SEARCH_RULE {}

/// This was manually extracted from the definition for `REQUEST-ATTRIBUTE`,
/// since the Wildboar ASN.1 compiler cannot compile embedded structs.
///
/// ### ASN.1 Definition:
///
/// ```asn1
/// REQUEST-ATTRIBUTE-DefaultValues-Item ::= SEQUENCE {
///     entryType              OBJECT-CLASS.&id            OPTIONAL,
///     values                 SEQUENCE OF ATTRIBUTE.&Type }
/// ```
///
#[derive(Debug, PartialEq)]
pub struct REQUEST_ATTRIBUTE_DefaultValues_Item {
    pub entryType: OBJECT_IDENTIFIER,
    pub values: SEQUENCE_OF<X690Element>,
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// REQUEST-ATTRIBUTE ::= CLASS {
///   &attributeType         ATTRIBUTE.&id,
///   &SelectedValues        ATTRIBUTE.&Type                 OPTIONAL,
///   &DefaultValues         SEQUENCE {
///     entryType              OBJECT-CLASS.&id            OPTIONAL,
///     values                 SEQUENCE OF ATTRIBUTE.&Type } OPTIONAL,
///   &contexts              SEQUENCE OF ContextProfile      OPTIONAL,
///   &contextCombination    ContextCombination              OPTIONAL,
///   &MatchingUse           MatchingUse                     OPTIONAL,
///   &includeSubtypes       BOOLEAN                         DEFAULT FALSE }
/// WITH SYNTAX {
///   ATTRIBUTE TYPE         &attributeType
///   [SELECTED VALUES       &SelectedValues]
///   [DEFAULT VALUES        &DefaultValues]
///   [CONTEXTS              &contexts]
///   [CONTEXT COMBINATION   &contextCombination]
///   [MATCHING USE          &MatchingUse]
///   [INCLUDE SUBTYPES      &includeSubtypes] }
/// ```
///
#[derive(Debug)]
pub struct REQUEST_ATTRIBUTE {
    pub attributeType: OBJECT_IDENTIFIER,
    pub SelectedValues: OPTIONAL<X690Element>,
    pub DefaultValues: OPTIONAL<Vec<REQUEST_ATTRIBUTE_DefaultValues_Item>>,
    pub contexts: OPTIONAL<Vec<ContextProfile>>,
    pub contextCombination: OPTIONAL<ContextCombination>,
    pub MatchingUse: OPTIONAL<MatchingUse>,
    pub includeSubtypes: OPTIONAL<BOOLEAN>,
}
impl REQUEST_ATTRIBUTE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RESULT-ATTRIBUTE ::= CLASS {
///   &attributeType         ATTRIBUTE.&id,
///   &outputValues          CHOICE {
///     selectedValues         SEQUENCE OF ATTRIBUTE.&Type,
///     matchedValuesOnly      NULL }                      OPTIONAL,
///   &contexts              ContextProfile                  OPTIONAL }
/// WITH SYNTAX {
///   ATTRIBUTE TYPE        &attributeType
///   [OUTPUT VALUES        &outputValues]
///   [CONTEXTS             &contexts] }
/// ```
///
#[derive(Debug)]
pub struct RESULT_ATTRIBUTE {
    pub attributeType: OBJECT_IDENTIFIER,
    // FIXME: &outputValues;
    pub contexts: OPTIONAL<ContextProfile>,
}
impl RESULT_ATTRIBUTE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MATCHING-RESTRICTION ::= CLASS {
///   &Restriction,
///   &Rules                MATCHING-RULE.&id,
///   &id                   OBJECT IDENTIFIER  UNIQUE }
/// WITH SYNTAX {
///   RESTRICTION           &Restriction
///   RULES                 &Rules
///   ID                    &id }
/// ```
///
#[derive(Debug)]
pub struct MATCHING_RESTRICTION {
    pub Rules: OBJECT_IDENTIFIER,
    pub id: OBJECT_IDENTIFIER,
}
impl MATCHING_RESTRICTION {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RequestAttribute-defaultValues-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct RequestAttribute_defaultValues_Item {
    pub entryType: OPTIONAL<OBJECT_IDENTIFIER>,
    pub values: Vec<X690Element>,
    pub _unrecognized: Vec<X690Element>,
}
impl RequestAttribute_defaultValues_Item {
    pub fn new(
        entryType: OPTIONAL<OBJECT_IDENTIFIER>,
        values: Vec<X690Element>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        RequestAttribute_defaultValues_Item {
            entryType,
            values,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for RequestAttribute_defaultValues_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RequestAttribute_defaultValues_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RequestAttribute_defaultValues_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_RequestAttribute_defaultValues_Item(el)
    }
}

pub const _rctl1_components_for_RequestAttribute_defaultValues_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "entryType",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "values",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RequestAttribute_defaultValues_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RequestAttribute_defaultValues_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_RequestAttribute_defaultValues_Item(
    el: &X690Element,
) -> ASN1Result<RequestAttribute_defaultValues_Item> {
    |el_: &X690Element| -> ASN1Result<RequestAttribute_defaultValues_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RequestAttribute_defaultValues_Item,
            _eal_components_for_RequestAttribute_defaultValues_Item,
            _rctl2_components_for_RequestAttribute_defaultValues_Item,
        )?;
        let entryType: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("entryType") {
            Some(c_) => Some(ber_decode_object_identifier(c_)?),
            _ => None,
        };
        let values = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<X690Element>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<X690Element> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(x690_identity(el)?);
            }
            Ok(items)
        }(_components.get("values").unwrap())?;
        Ok(RequestAttribute_defaultValues_Item {
            entryType,
            values,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_RequestAttribute_defaultValues_Item(
    value_: &RequestAttribute_defaultValues_Item,
) -> ASN1Result<X690Element> {
    |value_: &RequestAttribute_defaultValues_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.entryType {
            components_.push(ber_encode_object_identifier(&v_)?);
        }
        components_.push(
            |value_: &SEQUENCE_OF<X690Element>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(x690_identity(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.values)?,
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
/// ResultAttribute-outputValues ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum ResultAttribute_outputValues {
    selectedValues(Vec<X690Element>),
    matchedValuesOnly(NULL),
}

impl TryFrom<X690Element> for ResultAttribute_outputValues {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ResultAttribute_outputValues(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ResultAttribute_outputValues {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ResultAttribute_outputValues(el)
    }
}

pub fn _decode_ResultAttribute_outputValues(
    el: &X690Element,
) -> ASN1Result<ResultAttribute_outputValues> {
    |el: &X690Element| -> ASN1Result<ResultAttribute_outputValues> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(ResultAttribute_outputValues::selectedValues(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<X690Element>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SEQUENCE_OF<X690Element> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(x690_identity(el)?);
                    }
                    Ok(items)
                }(&el)?,
            )),
            (TagClass::UNIVERSAL, 5) => Ok(ResultAttribute_outputValues::matchedValuesOnly(
                ber_decode_null(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_ResultAttribute_outputValues(
    value_: &ResultAttribute_outputValues,
) -> ASN1Result<X690Element> {
    |value: &ResultAttribute_outputValues| -> ASN1Result<X690Element> {
        match value {
            ResultAttribute_outputValues::selectedValues(v) => {
                |value_: &SEQUENCE_OF<X690Element>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(x690_identity(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v)
            }
            ResultAttribute_outputValues::matchedValuesOnly(v) => ber_encode_null(&v),
        }
    }(&value_)
}
