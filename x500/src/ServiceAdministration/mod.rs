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
use wildboar_asn1::*;
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
        /* COULD_NOT_COMPILE_DEFAULT_VALUE attributeCombination */
        AttributeCombination::and(vec![])
    }
    pub fn _default_value_for_allowedSubset() -> AllowedSubset {
        BIT_STRING::from_bin("111")
    }
}
impl TryFrom<&X690Element> for SearchRule {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchRule")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SearchRule,
        _eal_components_for_SearchRule,
        _rctl2_components_for_SearchRule,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut id_: OPTIONAL<INTEGER> = None;
    let mut dmdId_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut serviceType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut userClass_: OPTIONAL<INTEGER> = None;
    let mut inputAttributeTypes_: OPTIONAL<Vec<RequestAttribute>> = None;
    let mut attributeCombination_: OPTIONAL<AttributeCombination> = None;
    let mut outputAttributeTypes_: OPTIONAL<Vec<ResultAttribute>> = None;
    let mut defaultControls_: OPTIONAL<ControlOptions> = None;
    let mut mandatoryControls_: OPTIONAL<ControlOptions> = None;
    let mut searchRuleControls_: OPTIONAL<ControlOptions> = None;
    let mut familyGrouping_: OPTIONAL<FamilyGrouping> = None;
    let mut familyReturn_: OPTIONAL<FamilyReturn> = None;
    let mut relaxation_: OPTIONAL<RelaxationPolicy> = None;
    let mut additionalControl_: OPTIONAL<Vec<AttributeType>> = None;
    let mut allowedSubset_: OPTIONAL<AllowedSubset> = None;
    let mut imposedSubset_: OPTIONAL<ImposedSubset> = None;
    let mut entryLimit_: OPTIONAL<EntryLimit> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "id" => id_ = Some(BER.decode_integer(_el)?),
            "dmdId" => {
                dmdId_ = Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                    Ok(BER.decode_object_identifier(&el.inner()?)?)
                }(_el)?)
            }
            "serviceType" => {
                serviceType_ = Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                    Ok(BER.decode_object_identifier(&el.inner()?)?)
                }(_el)?)
            }
            "userClass" => {
                userClass_ = Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                    Ok(BER.decode_integer(&el.inner()?)?)
                }(_el)?)
            }
            "inputAttributeTypes" => {
                inputAttributeTypes_ =
                    Some(|el: &X690Element| -> ASN1Result<Vec<RequestAttribute>> {
                        Ok(
                            |el: &X690Element| -> ASN1Result<SEQUENCE_OF<RequestAttribute>> {
                                let elements = match &el.value {
                                    X690Value::Constructed(children) => children,
                                    _ => {
                                        return Err(el.to_asn1_err_named(
                                            ASN1ErrorCode::invalid_construction,
                                            "inputAttributeTypes",
                                        ))
                                    }
                                };
                                let mut items: SEQUENCE_OF<RequestAttribute> =
                                    Vec::with_capacity(elements.len());
                                for el in elements.iter() {
                                    items.push(_decode_RequestAttribute(el)?);
                                }
                                Ok(items)
                            }(&el.inner()?)?,
                        )
                    }(_el)?)
            }
            "attributeCombination" => {
                attributeCombination_ =
                    Some(|el: &X690Element| -> ASN1Result<AttributeCombination> {
                        Ok(_decode_AttributeCombination(&el.inner()?)?)
                    }(_el)?)
            }
            "outputAttributeTypes" => {
                outputAttributeTypes_ =
                    Some(|el: &X690Element| -> ASN1Result<Vec<ResultAttribute>> {
                        Ok(
                            |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ResultAttribute>> {
                                let elements = match &el.value {
                                    X690Value::Constructed(children) => children,
                                    _ => {
                                        return Err(el.to_asn1_err_named(
                                            ASN1ErrorCode::invalid_construction,
                                            "outputAttributeTypes",
                                        ))
                                    }
                                };
                                let mut items: SEQUENCE_OF<ResultAttribute> =
                                    Vec::with_capacity(elements.len());
                                for el in elements.iter() {
                                    items.push(_decode_ResultAttribute(el)?);
                                }
                                Ok(items)
                            }(&el.inner()?)?,
                        )
                    }(_el)?)
            }
            "defaultControls" => {
                defaultControls_ = Some(|el: &X690Element| -> ASN1Result<ControlOptions> {
                    Ok(_decode_ControlOptions(&el.inner()?)?)
                }(_el)?)
            }
            "mandatoryControls" => {
                mandatoryControls_ = Some(|el: &X690Element| -> ASN1Result<ControlOptions> {
                    Ok(_decode_ControlOptions(&el.inner()?)?)
                }(_el)?)
            }
            "searchRuleControls" => {
                searchRuleControls_ = Some(|el: &X690Element| -> ASN1Result<ControlOptions> {
                    Ok(_decode_ControlOptions(&el.inner()?)?)
                }(_el)?)
            }
            "familyGrouping" => {
                familyGrouping_ = Some(|el: &X690Element| -> ASN1Result<FamilyGrouping> {
                    Ok(_decode_FamilyGrouping(&el.inner()?)?)
                }(_el)?)
            }
            "familyReturn" => {
                familyReturn_ = Some(|el: &X690Element| -> ASN1Result<FamilyReturn> {
                    Ok(_decode_FamilyReturn(&el.inner()?)?)
                }(_el)?)
            }
            "relaxation" => {
                relaxation_ = Some(|el: &X690Element| -> ASN1Result<RelaxationPolicy> {
                    Ok(_decode_RelaxationPolicy(&el.inner()?)?)
                }(_el)?)
            }
            "additionalControl" => {
                additionalControl_ = Some(|el: &X690Element| -> ASN1Result<Vec<AttributeType>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AttributeType>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "additionalControl",
                                    ))
                                }
                            };
                            let mut items: SEQUENCE_OF<AttributeType> =
                                Vec::with_capacity(elements.len());
                            for el in elements.iter() {
                                items.push(_decode_AttributeType(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(_el)?)
            }
            "allowedSubset" => {
                allowedSubset_ = Some(|el: &X690Element| -> ASN1Result<AllowedSubset> {
                    Ok(_decode_AllowedSubset(&el.inner()?)?)
                }(_el)?)
            }
            "imposedSubset" => {
                imposedSubset_ = Some(|el: &X690Element| -> ASN1Result<ImposedSubset> {
                    Ok(_decode_ImposedSubset(&el.inner()?)?)
                }(_el)?)
            }
            "entryLimit" => {
                entryLimit_ = Some(|el: &X690Element| -> ASN1Result<EntryLimit> {
                    Ok(_decode_EntryLimit(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(SearchRule {
        id: id_.unwrap(),
        dmdId: dmdId_.unwrap(),
        serviceType: serviceType_,
        userClass: userClass_,
        inputAttributeTypes: inputAttributeTypes_,
        attributeCombination: attributeCombination_,
        outputAttributeTypes: outputAttributeTypes_,
        defaultControls: defaultControls_,
        mandatoryControls: mandatoryControls_,
        searchRuleControls: searchRuleControls_,
        familyGrouping: familyGrouping_,
        familyReturn: familyReturn_,
        relaxation: relaxation_,
        additionalControl: additionalControl_,
        allowedSubset: allowedSubset_,
        imposedSubset: imposedSubset_,
        entryLimit: entryLimit_,
        _unrecognized,
    })
}

pub fn _encode_SearchRule(value_: &SearchRule) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(27);
    components_.push(BER.encode_integer(&value_.id)?);
    components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(BER.encode_object_identifier(&v_1)?),
        ))
    }(&value_.dmdId)?);
    if let Some(v_) = &value_.serviceType {
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(BER.encode_object_identifier(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.userClass {
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(BER.encode_integer(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.inputAttributeTypes {
        components_.push(|v_1: &Vec<RequestAttribute>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<RequestAttribute>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_RequestAttribute(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.attributeCombination {
        if *v_ != SearchRule::_default_value_for_attributeCombination() {
            components_.push(|v_1: &AttributeCombination| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 4),
                    X690Value::from_explicit(_encode_AttributeCombination(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.outputAttributeTypes {
        components_.push(|v_1: &Vec<ResultAttribute>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 5),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<ResultAttribute>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ResultAttribute(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.defaultControls {
        components_.push(|v_1: &ControlOptions| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 6),
                X690Value::from_explicit(_encode_ControlOptions(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.mandatoryControls {
        components_.push(|v_1: &ControlOptions| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 7),
                X690Value::from_explicit(_encode_ControlOptions(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.searchRuleControls {
        components_.push(|v_1: &ControlOptions| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 8),
                X690Value::from_explicit(_encode_ControlOptions(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.familyGrouping {
        components_.push(|v_1: &FamilyGrouping| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 9),
                X690Value::from_explicit(_encode_FamilyGrouping(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.familyReturn {
        components_.push(|v_1: &FamilyReturn| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 10),
                X690Value::from_explicit(_encode_FamilyReturn(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.relaxation {
        components_.push(|v_1: &RelaxationPolicy| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 11),
                X690Value::from_explicit(_encode_RelaxationPolicy(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.additionalControl {
        components_.push(|v_1: &Vec<AttributeType>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 12),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<AttributeType>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AttributeType(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.allowedSubset {
        if *v_ != SearchRule::_default_value_for_allowedSubset() {
            components_.push(|v_1: &AllowedSubset| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 13),
                    X690Value::from_explicit(_encode_AllowedSubset(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.imposedSubset {
        components_.push(|v_1: &ImposedSubset| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 14),
                X690Value::from_explicit(_encode_ImposedSubset(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.entryLimit {
        components_.push(|v_1: &EntryLimit| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 15),
                X690Value::from_explicit(_encode_EntryLimit(&v_1)?),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_SearchRule(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchRule")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SearchRule,
        _eal_components_for_SearchRule,
        _rctl2_components_for_SearchRule,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "id" => BER.validate_integer(_el)?,
            "dmdId" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "dmdId"));
                }
                Ok(BER.validate_object_identifier(&el.inner()?)?)
            }(_el)?,
            "serviceType" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "serviceType")
                    );
                }
                Ok(BER.validate_object_identifier(&el.inner()?)?)
            }(_el)?,
            "userClass" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "userClass")
                    );
                }
                Ok(BER.validate_integer(&el.inner()?)?)
            }(_el)?,
            "inputAttributeTypes" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "inputAttributeTypes",
                    ));
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_RequestAttribute(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "inputAttributeTypes",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "attributeCombination" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "attributeCombination",
                    ));
                }
                Ok(_validate_AttributeCombination(&el.inner()?)?)
            }(_el)?,
            "outputAttributeTypes" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "outputAttributeTypes",
                    ));
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_ResultAttribute(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "outputAttributeTypes",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "defaultControls" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "defaultControls",
                    ));
                }
                Ok(_validate_ControlOptions(&el.inner()?)?)
            }(_el)?,
            "mandatoryControls" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "mandatoryControls",
                    ));
                }
                Ok(_validate_ControlOptions(&el.inner()?)?)
            }(_el)?,
            "searchRuleControls" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 8 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "searchRuleControls",
                    ));
                }
                Ok(_validate_ControlOptions(&el.inner()?)?)
            }(_el)?,
            "familyGrouping" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 9 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "familyGrouping")
                    );
                }
                Ok(_validate_FamilyGrouping(&el.inner()?)?)
            }(_el)?,
            "familyReturn" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 10 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "familyReturn")
                    );
                }
                Ok(_validate_FamilyReturn(&el.inner()?)?)
            }(_el)?,
            "relaxation" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 11 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "relaxation")
                    );
                }
                Ok(_validate_RelaxationPolicy(&el.inner()?)?)
            }(_el)?,
            "additionalControl" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 12 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "additionalControl",
                    ));
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_AttributeType(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "additionalControl",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "allowedSubset" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 13 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "allowedSubset")
                    );
                }
                Ok(_validate_AllowedSubset(&el.inner()?)?)
            }(_el)?,
            "imposedSubset" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 14 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "imposedSubset")
                    );
                }
                Ok(_validate_ImposedSubset(&el.inner()?)?)
            }(_el)?,
            "entryLimit" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 15 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "entryLimit")
                    );
                }
                Ok(_validate_EntryLimit(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchRuleId ::= SEQUENCE {
///   id          INTEGER,
///   dmdId  [0]  OBJECT IDENTIFIER }
/// ```
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
impl TryFrom<&X690Element> for SearchRuleId {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchRuleId")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SearchRuleId,
        _eal_components_for_SearchRuleId,
        _rctl2_components_for_SearchRuleId,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut id_: OPTIONAL<INTEGER> = None;
    let mut dmdId_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "id" => id_ = Some(BER.decode_integer(_el)?),
            "dmdId" => {
                dmdId_ = Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                    Ok(BER.decode_object_identifier(&el.inner()?)?)
                }(_el)?)
            }
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchRuleId")
                )
            }
        }
    }
    Ok(SearchRuleId {
        id: id_.unwrap(),
        dmdId: dmdId_.unwrap(),
    })
}

pub fn _encode_SearchRuleId(value_: &SearchRuleId) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_integer(&value_.id)?);
    components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(BER.encode_object_identifier(&v_1)?),
        ))
    }(&value_.dmdId)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_SearchRuleId(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchRuleId")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SearchRuleId,
        _eal_components_for_SearchRuleId,
        _rctl2_components_for_SearchRuleId,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "id" => BER.validate_integer(_el)?,
            "dmdId" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "dmdId"));
                }
                Ok(BER.validate_object_identifier(&el.inner()?)?)
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchRuleId")
                )
            }
        }
    }
    Ok(())
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
    BER.decode_bit_string(&el)
}

pub fn _encode_AllowedSubset(value_: &AllowedSubset) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_AllowedSubset(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
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
    BER.decode_enumerated(&el)
}

pub fn _encode_ImposedSubset(value_: &ImposedSubset) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_ImposedSubset(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
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
        /* COULD_NOT_COMPILE_DEFAULT_VALUE contextCombination */
        ContextCombination::and(vec![])
    }
}
impl TryFrom<&X690Element> for RequestAttribute {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RequestAttribute")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RequestAttribute,
        _eal_components_for_RequestAttribute,
        _rctl2_components_for_RequestAttribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut attributeType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut includeSubtypes_: OPTIONAL<BOOLEAN> = None;
    let mut selectedValues_: OPTIONAL<Vec<X690Element>> = None;
    let mut defaultValues_: OPTIONAL<Vec<RequestAttribute_defaultValues_Item>> = None;
    let mut contexts_: OPTIONAL<Vec<ContextProfile>> = None;
    let mut contextCombination_: OPTIONAL<ContextCombination> = None;
    let mut matchingUse_: OPTIONAL<Vec<MatchingUse>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attributeType" => attributeType_ = Some(BER.decode_object_identifier(_el)?),
            "includeSubtypes" => {
                includeSubtypes_ = Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                    Ok(BER.decode_boolean(&el.inner()?)?)
                }(_el)?)
            }
            "selectedValues" => {
                selectedValues_ = Some(|el: &X690Element| -> ASN1Result<Vec<X690Element>> {
                    Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<X690Element>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "selectedValues",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<X690Element> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(x690_identity(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "defaultValues" => {
                defaultValues_ = Some(|el: &X690Element| -> ASN1Result<
                    Vec<RequestAttribute_defaultValues_Item>,
                > {
                    Ok(|el: &X690Element| -> ASN1Result<
                        SEQUENCE_OF<RequestAttribute_defaultValues_Item>,
                    > {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "defaultValues",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<RequestAttribute_defaultValues_Item> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_RequestAttribute_defaultValues_Item(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "contexts" => {
                contexts_ = Some(|el: &X690Element| -> ASN1Result<Vec<ContextProfile>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ContextProfile>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "contexts",
                                    ))
                                }
                            };
                            let mut items: SEQUENCE_OF<ContextProfile> =
                                Vec::with_capacity(elements.len());
                            for el in elements.iter() {
                                items.push(_decode_ContextProfile(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(_el)?)
            }
            "contextCombination" => {
                contextCombination_ = Some(|el: &X690Element| -> ASN1Result<ContextCombination> {
                    Ok(_decode_ContextCombination(&el.inner()?)?)
                }(_el)?)
            }
            "matchingUse" => {
                matchingUse_ = Some(|el: &X690Element| -> ASN1Result<Vec<MatchingUse>> {
                    Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<MatchingUse>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "matchingUse",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<MatchingUse> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_MatchingUse(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(RequestAttribute {
        attributeType: attributeType_.unwrap(),
        includeSubtypes: includeSubtypes_,
        selectedValues: selectedValues_,
        defaultValues: defaultValues_,
        contexts: contexts_,
        contextCombination: contextCombination_,
        matchingUse: matchingUse_,
        _unrecognized,
    })
}

pub fn _encode_RequestAttribute(value_: &RequestAttribute) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(17);
    components_.push(BER.encode_object_identifier(&value_.attributeType)?);
    if let Some(v_) = &value_.includeSubtypes {
        if *v_ != RequestAttribute::_default_value_for_includeSubtypes() {
            components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(BER.encode_boolean(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.selectedValues {
        components_.push(|v_1: &Vec<X690Element>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<X690Element>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(x690_identity(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.defaultValues {
        components_.push(
            |v_1: &Vec<RequestAttribute_defaultValues_Item>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(|value_: &SEQUENCE_OF<
                        RequestAttribute_defaultValues_Item,
                    >|
                     -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_RequestAttribute_defaultValues_Item(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?),
                ))
            }(&v_)?,
        );
    }
    if let Some(v_) = &value_.contexts {
        components_.push(|v_1: &Vec<ContextProfile>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<ContextProfile>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ContextProfile(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.contextCombination {
        if *v_ != RequestAttribute::_default_value_for_contextCombination() {
            components_.push(|v_1: &ContextCombination| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 4),
                    X690Value::from_explicit(_encode_ContextCombination(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.matchingUse {
        components_.push(|v_1: &Vec<MatchingUse>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 5),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<MatchingUse>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_MatchingUse(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_RequestAttribute(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RequestAttribute")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RequestAttribute,
        _eal_components_for_RequestAttribute,
        _rctl2_components_for_RequestAttribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attributeType" => BER.validate_object_identifier(_el)?,
            "includeSubtypes" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "includeSubtypes",
                    ));
                }
                Ok(BER.validate_boolean(&el.inner()?)?)
            }(_el)?,
            "selectedValues" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "selectedValues")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                BER.validate_any(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "selectedValues",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "defaultValues" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "defaultValues")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_RequestAttribute_defaultValues_Item(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "defaultValues",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "contexts" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contexts")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_ContextProfile(&sub)?;
                            }
                            Ok(())
                        }
                        _ => {
                            Err(el
                                .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contexts"))
                        }
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "contextCombination" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "contextCombination",
                    ));
                }
                Ok(_validate_ContextCombination(&el.inner()?)?)
            }(_el)?,
            "matchingUse" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "matchingUse")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_MatchingUse(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el
                            .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "matchingUse")),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for ContextProfile {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ContextProfile"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ContextProfile,
        _eal_components_for_ContextProfile,
        _rctl2_components_for_ContextProfile,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut contextType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut contextValue_: OPTIONAL<Vec<X690Element>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contextType" => contextType_ = Some(BER.decode_object_identifier(_el)?),
            "contextValue" => {
                contextValue_ = Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<X690Element>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "contextValue",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<X690Element> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(x690_identity(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ContextProfile {
        contextType: contextType_.unwrap(),
        contextValue: contextValue_,
        _unrecognized,
    })
}

pub fn _encode_ContextProfile(value_: &ContextProfile) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.contextType)?);
    if let Some(v_) = &value_.contextValue {
        components_.push(
            |value_: &SEQUENCE_OF<X690Element>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(x690_identity(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_)?,
        );
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ContextProfile(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ContextProfile"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ContextProfile,
        _eal_components_for_ContextProfile,
        _rctl2_components_for_ContextProfile,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contextType" => BER.validate_object_identifier(_el)?,
            "contextValue" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            BER.validate_any(&sub)?;
                        }
                        Ok(())
                    }
                    _ => {
                        Err(el
                            .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contextValue"))
                    }
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
/// ContextCombination  ::=  CHOICE {
///   context  [0]  CONTEXT.&id({SupportedContexts}),
///   and      [1]  SEQUENCE OF ContextCombination,
///   or       [2]  SEQUENCE OF ContextCombination,
///   not      [3]  ContextCombination,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum ContextCombination {
    context(OBJECT_IDENTIFIER),
    and(Vec<ContextCombination>),
    or(Vec<ContextCombination>),
    not(Box<ContextCombination>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for ContextCombination {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ContextCombination(el)
    }
}

impl PartialEq for ContextCombination {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ContextCombination::context(a) => match other {
                ContextCombination::context(b) => return a == b,
                _ => false,
            },
            ContextCombination::and(a) => {
                match other {
                    ContextCombination::and(b) => {
                        if a.len() != b.len() || a.len() > 20 {
                            return false;
                        }
                        // WARNING: This is O(n^2)
                        for x in a {
                            if b.iter().find(|y| *x == **y).is_none() {
                                return false;
                            }
                        }
                        return true;
                    }
                    _ => false,
                }
            }
            ContextCombination::or(a) => {
                match other {
                    ContextCombination::or(b) => {
                        if a.len() != b.len() || a.len() > 20 {
                            return false;
                        }
                        // WARNING: This is O(n^2)
                        for x in a {
                            if b.iter().find(|y| *x == **y).is_none() {
                                return false;
                            }
                        }
                        return true;
                    }
                    _ => false,
                }
            }
            ContextCombination::not(a) => match other {
                ContextCombination::not(b) => return a == b,
                _ => false,
            },
            _ => false,
        }
    }
}

pub fn _decode_ContextCombination(el: &X690Element) -> ASN1Result<ContextCombination> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(ContextCombination::context(
            |el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(BER.decode_object_identifier(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(ContextCombination::and(|el: &X690Element| -> ASN1Result<
            Vec<ContextCombination>,
        > {
            Ok(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ContextCombination>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and")
                            )
                        }
                    };
                    let mut items: SEQUENCE_OF<ContextCombination> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_ContextCombination(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )
        }(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(ContextCombination::or(|el: &X690Element| -> ASN1Result<
            Vec<ContextCombination>,
        > {
            Ok(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ContextCombination>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or")
                            )
                        }
                    };
                    let mut items: SEQUENCE_OF<ContextCombination> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_ContextCombination(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )
        }(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(ContextCombination::not(Box::new(
            |el: &X690Element| -> ASN1Result<ContextCombination> {
                Ok(_decode_ContextCombination(&el.inner()?)?)
            }(&el)?,
        ))),
        _ => Ok(ContextCombination::_unrecognized(el.clone())),
    }
}

pub fn _encode_ContextCombination(value_: &ContextCombination) -> ASN1Result<X690Element> {
    match value_ {
        ContextCombination::context(v) => |v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(BER.encode_object_identifier(&v_1)?),
            ))
        }(&v),
        ContextCombination::and(v) => |v_1: &Vec<ContextCombination>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<ContextCombination>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ContextCombination(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v),
        ContextCombination::or(v) => |v_1: &Vec<ContextCombination>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<ContextCombination>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ContextCombination(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v),
        ContextCombination::not(v) => |v_1: &ContextCombination| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(_encode_ContextCombination(&v_1)?),
            ))
        }(&v),
        ContextCombination::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_ContextCombination(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "context"));
            }
            Ok(BER.validate_object_identifier(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and"));
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_ContextCombination(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and")),
                }
            }(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or"));
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_ContextCombination(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or")),
                }
            }(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "not"));
            }
            Ok(_validate_ContextCombination(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
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
impl TryFrom<&X690Element> for MatchingUse {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MatchingUse")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MatchingUse,
        _eal_components_for_MatchingUse,
        _rctl2_components_for_MatchingUse,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut restrictionType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut restrictionValue_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "restrictionType" => restrictionType_ = Some(BER.decode_object_identifier(_el)?),
            "restrictionValue" => restrictionValue_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(MatchingUse {
        restrictionType: restrictionType_.unwrap(),
        restrictionValue: restrictionValue_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_MatchingUse(value_: &MatchingUse) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.restrictionType)?);
    components_.push(x690_identity(&value_.restrictionValue)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_MatchingUse(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MatchingUse")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MatchingUse,
        _eal_components_for_MatchingUse,
        _rctl2_components_for_MatchingUse,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "restrictionType" => BER.validate_object_identifier(_el)?,
            "restrictionValue" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedMatchingRestrictions MATCHING-RESTRICTION ::= {...}
/// ```
///
///
pub fn SupportedMatchingRestrictions() -> Vec<MATCHING_RESTRICTION> {
    Vec::new()
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
#[derive(Debug, Clone)]
pub enum AttributeCombination {
    attribute(AttributeType),
    and(Vec<AttributeCombination>),
    or(Vec<AttributeCombination>),
    not(Box<AttributeCombination>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for AttributeCombination {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeCombination(el)
    }
}

impl PartialEq for AttributeCombination {
    fn eq(&self, other: &Self) -> bool {
        match self {
            AttributeCombination::attribute(a) => match other {
                AttributeCombination::attribute(b) => return a == b,
                _ => false,
            },
            AttributeCombination::and(a) => {
                match other {
                    AttributeCombination::and(b) => {
                        if a.len() != b.len() || a.len() > 20 {
                            return false;
                        }
                        // WARNING: This is O(n^2)
                        for x in a {
                            if b.iter().find(|y| *x == **y).is_none() {
                                return false;
                            }
                        }
                        return true;
                    }
                    _ => false,
                }
            }
            AttributeCombination::or(a) => {
                match other {
                    AttributeCombination::or(b) => {
                        if a.len() != b.len() || a.len() > 20 {
                            return false;
                        }
                        // WARNING: This is O(n^2)
                        for x in a {
                            if b.iter().find(|y| *x == **y).is_none() {
                                return false;
                            }
                        }
                        return true;
                    }
                    _ => false,
                }
            }
            AttributeCombination::not(a) => match other {
                AttributeCombination::not(b) => return a == b,
                _ => false,
            },
            _ => false,
        }
    }
}

pub fn _decode_AttributeCombination(el: &X690Element) -> ASN1Result<AttributeCombination> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(AttributeCombination::attribute(
            |el: &X690Element| -> ASN1Result<AttributeType> {
                Ok(_decode_AttributeType(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(AttributeCombination::and(|el: &X690Element| -> ASN1Result<
            Vec<AttributeCombination>,
        > {
            Ok(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AttributeCombination>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and")
                            )
                        }
                    };
                    let mut items: SEQUENCE_OF<AttributeCombination> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_AttributeCombination(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )
        }(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(AttributeCombination::or(|el: &X690Element| -> ASN1Result<
            Vec<AttributeCombination>,
        > {
            Ok(
                |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AttributeCombination>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or")
                            )
                        }
                    };
                    let mut items: SEQUENCE_OF<AttributeCombination> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_AttributeCombination(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )
        }(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(AttributeCombination::not(Box::new(
            |el: &X690Element| -> ASN1Result<AttributeCombination> {
                Ok(_decode_AttributeCombination(&el.inner()?)?)
            }(&el)?,
        ))),
        _ => Ok(AttributeCombination::_unrecognized(el.clone())),
    }
}

pub fn _encode_AttributeCombination(value_: &AttributeCombination) -> ASN1Result<X690Element> {
    match value_ {
        AttributeCombination::attribute(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(_encode_AttributeType(&v_1)?),
            ))
        }(&v),
        AttributeCombination::and(v) => {
            |v_1: &Vec<AttributeCombination>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(
                        &|value_: &SEQUENCE_OF<AttributeCombination>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_AttributeCombination(&v)?);
                            }
                            Ok(X690Element::new(
                                Tag::new(
                                    TagClass::UNIVERSAL,
                                    UNIV_TAG_SEQUENCE_OF,
                                ),
                                X690Value::Constructed(Arc::new(children)),
                            ))
                        }(&v_1)?,
                    ),
                ))
            }(&v)
        }
        AttributeCombination::or(v) => {
            |v_1: &Vec<AttributeCombination>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(
                        &|value_: &SEQUENCE_OF<AttributeCombination>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_AttributeCombination(&v)?);
                            }
                            Ok(X690Element::new(
                                Tag::new(
                                    TagClass::UNIVERSAL,
                                    UNIV_TAG_SEQUENCE_OF,
                                ),
                                X690Value::Constructed(Arc::new(children)),
                            ))
                        }(&v_1)?,
                    ),
                ))
            }(&v)
        }
        AttributeCombination::not(v) => |v_1: &AttributeCombination| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(_encode_AttributeCombination(&v_1)?),
            ))
        }(&v),
        AttributeCombination::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_AttributeCombination(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attribute"));
            }
            Ok(_validate_AttributeType(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and"));
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_AttributeCombination(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and")),
                }
            }(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or"));
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_AttributeCombination(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or")),
                }
            }(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "not"));
            }
            Ok(_validate_AttributeCombination(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
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
impl TryFrom<&X690Element> for ResultAttribute {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ResultAttribute"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ResultAttribute,
        _eal_components_for_ResultAttribute,
        _rctl2_components_for_ResultAttribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut attributeType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut outputValues_: OPTIONAL<ResultAttribute_outputValues> = None;
    let mut contexts_: OPTIONAL<Vec<ContextProfile>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attributeType" => attributeType_ = Some(BER.decode_object_identifier(_el)?),
            "outputValues" => outputValues_ = Some(_decode_ResultAttribute_outputValues(_el)?),
            "contexts" => {
                contexts_ = Some(|el: &X690Element| -> ASN1Result<Vec<ContextProfile>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ContextProfile>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "contexts",
                                    ))
                                }
                            };
                            let mut items: SEQUENCE_OF<ContextProfile> =
                                Vec::with_capacity(elements.len());
                            for el in elements.iter() {
                                items.push(_decode_ContextProfile(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ResultAttribute {
        attributeType: attributeType_.unwrap(),
        outputValues: outputValues_,
        contexts: contexts_,
        _unrecognized,
    })
}

pub fn _encode_ResultAttribute(value_: &ResultAttribute) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_object_identifier(&value_.attributeType)?);
    if let Some(v_) = &value_.outputValues {
        components_.push(_encode_ResultAttribute_outputValues(&v_)?);
    }
    if let Some(v_) = &value_.contexts {
        components_.push(|v_1: &Vec<ContextProfile>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<ContextProfile>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ContextProfile(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ResultAttribute(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ResultAttribute"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ResultAttribute,
        _eal_components_for_ResultAttribute,
        _rctl2_components_for_ResultAttribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attributeType" => BER.validate_object_identifier(_el)?,
            "outputValues" => _validate_ResultAttribute_outputValues(_el)?,
            "contexts" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contexts")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_ContextProfile(&sub)?;
                            }
                            Ok(())
                        }
                        _ => {
                            Err(el
                                .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contexts"))
                        }
                    }
                }(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for ControlOptions {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ControlOptions"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ControlOptions,
        _eal_components_for_ControlOptions,
        _rctl2_components_for_ControlOptions,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut serviceControls_: OPTIONAL<ServiceControlOptions> = None;
    let mut searchOptions_: OPTIONAL<SearchControlOptions> = None;
    let mut hierarchyOptions_: OPTIONAL<HierarchySelections> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serviceControls" => {
                serviceControls_ = Some(|el: &X690Element| -> ASN1Result<ServiceControlOptions> {
                    Ok(_decode_ServiceControlOptions(&el.inner()?)?)
                }(_el)?)
            }
            "searchOptions" => {
                searchOptions_ = Some(|el: &X690Element| -> ASN1Result<SearchControlOptions> {
                    Ok(_decode_SearchControlOptions(&el.inner()?)?)
                }(_el)?)
            }
            "hierarchyOptions" => {
                hierarchyOptions_ = Some(|el: &X690Element| -> ASN1Result<HierarchySelections> {
                    Ok(_decode_HierarchySelections(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ControlOptions {
        serviceControls: serviceControls_,
        searchOptions: searchOptions_,
        hierarchyOptions: hierarchyOptions_,
        _unrecognized,
    })
}

pub fn _encode_ControlOptions(value_: &ControlOptions) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.serviceControls {
        if *v_ != ControlOptions::_default_value_for_serviceControls() {
            components_.push(|v_1: &ServiceControlOptions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(_encode_ServiceControlOptions(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.searchOptions {
        if *v_ != ControlOptions::_default_value_for_searchOptions() {
            components_.push(|v_1: &SearchControlOptions| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(_encode_SearchControlOptions(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.hierarchyOptions {
        components_.push(|v_1: &HierarchySelections| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(_encode_HierarchySelections(&v_1)?),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ControlOptions(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ControlOptions"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ControlOptions,
        _eal_components_for_ControlOptions,
        _rctl2_components_for_ControlOptions,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "serviceControls" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "serviceControls",
                    ));
                }
                Ok(_validate_ServiceControlOptions(&el.inner()?)?)
            }(_el)?,
            "searchOptions" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "searchOptions")
                    );
                }
                Ok(_validate_SearchControlOptions(&el.inner()?)?)
            }(_el)?,
            "hierarchyOptions" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "hierarchyOptions",
                    ));
                }
                Ok(_validate_HierarchySelections(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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
impl TryFrom<&X690Element> for EntryLimit {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EntryLimit")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EntryLimit,
        _eal_components_for_EntryLimit,
        _rctl2_components_for_EntryLimit,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut default_: OPTIONAL<INTEGER> = None;
    let mut max_: OPTIONAL<INTEGER> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "default" => default_ = Some(BER.decode_integer(_el)?),
            "max" => max_ = Some(BER.decode_integer(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(EntryLimit {
        default: default_.unwrap(),
        max: max_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_EntryLimit(value_: &EntryLimit) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_integer(&value_.default)?);
    components_.push(BER.encode_integer(&value_.max)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_EntryLimit(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EntryLimit")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EntryLimit,
        _eal_components_for_EntryLimit,
        _rctl2_components_for_EntryLimit,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "default" => BER.validate_integer(_el)?,
            "max" => BER.validate_integer(_el)?,
            _ => (),
        }
    }
    Ok(())
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
        Vec::from([1])
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
impl TryFrom<&X690Element> for RelaxationPolicy {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RelaxationPolicy")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RelaxationPolicy,
        _eal_components_for_RelaxationPolicy,
        _rctl2_components_for_RelaxationPolicy,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut basic_: OPTIONAL<MRMapping> = None;
    let mut tightenings_: OPTIONAL<Vec<MRMapping>> = None;
    let mut relaxations_: OPTIONAL<Vec<MRMapping>> = None;
    let mut maximum_: OPTIONAL<INTEGER> = None;
    let mut minimum_: OPTIONAL<INTEGER> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "basic" => {
                basic_ = Some(|el: &X690Element| -> ASN1Result<MRMapping> {
                    Ok(_decode_MRMapping(&el.inner()?)?)
                }(_el)?)
            }
            "tightenings" => {
                tightenings_ = Some(|el: &X690Element| -> ASN1Result<Vec<MRMapping>> {
                    Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<MRMapping>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "tightenings",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<MRMapping> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_MRMapping(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "relaxations" => {
                relaxations_ = Some(|el: &X690Element| -> ASN1Result<Vec<MRMapping>> {
                    Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<MRMapping>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "relaxations",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<MRMapping> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_MRMapping(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "maximum" => {
                maximum_ = Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                    Ok(BER.decode_integer(&el.inner()?)?)
                }(_el)?)
            }
            "minimum" => {
                minimum_ = Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                    Ok(BER.decode_integer(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(RelaxationPolicy {
        basic: basic_,
        tightenings: tightenings_,
        relaxations: relaxations_,
        maximum: maximum_,
        minimum: minimum_,
        _unrecognized,
    })
}

pub fn _encode_RelaxationPolicy(value_: &RelaxationPolicy) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    if let Some(v_) = &value_.basic {
        if v_.mapping.is_some() || v_.substitution.is_some() {
            components_.push(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(_encode_MRMapping(&v_)?),
            ));
        }
    }
    if let Some(v_) = &value_.tightenings {
        components_.push(|v_1: &Vec<MRMapping>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<MRMapping>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_MRMapping(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.relaxations {
        components_.push(|v_1: &Vec<MRMapping>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<MRMapping>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_MRMapping(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.maximum {
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(BER.encode_integer(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.minimum {
        if *v_ != RelaxationPolicy::_default_value_for_minimum() {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 4),
                    X690Value::from_explicit(BER.encode_integer(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_RelaxationPolicy(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RelaxationPolicy")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RelaxationPolicy,
        _eal_components_for_RelaxationPolicy,
        _rctl2_components_for_RelaxationPolicy,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "basic" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "basic"));
                }
                Ok(_validate_MRMapping(&el.inner()?)?)
            }(_el)?,
            "tightenings" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "tightenings")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_MRMapping(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el
                            .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "tightenings")),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "relaxations" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "relaxations")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_MRMapping(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el
                            .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "relaxations")),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "maximum" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "maximum")
                    );
                }
                Ok(BER.validate_integer(&el.inner()?)?)
            }(_el)?,
            "minimum" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "minimum")
                    );
                }
                Ok(BER.validate_integer(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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
#[derive(Debug, Clone)]
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
impl TryFrom<&X690Element> for MRMapping {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MRMapping")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MRMapping,
        _eal_components_for_MRMapping,
        _rctl2_components_for_MRMapping,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut mapping_: OPTIONAL<Vec<Mapping>> = None;
    let mut substitution_: OPTIONAL<Vec<MRSubstitution>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "mapping" => {
                mapping_ = Some(|el: &X690Element| -> ASN1Result<Vec<Mapping>> {
                    Ok(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<Mapping>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "mapping",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<Mapping> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_Mapping(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "substitution" => {
                substitution_ = Some(|el: &X690Element| -> ASN1Result<Vec<MRSubstitution>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SEQUENCE_OF<MRSubstitution>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "substitution",
                                    ))
                                }
                            };
                            let mut items: SEQUENCE_OF<MRSubstitution> =
                                Vec::with_capacity(elements.len());
                            for el in elements.iter() {
                                items.push(_decode_MRSubstitution(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(MRMapping {
        mapping: mapping_,
        substitution: substitution_,
        _unrecognized,
    })
}

pub fn _encode_MRMapping(value_: &MRMapping) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.mapping {
        components_.push(|v_1: &Vec<Mapping>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<Mapping>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_Mapping(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.substitution {
        components_.push(|v_1: &Vec<MRSubstitution>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(
                    &|value_: &SEQUENCE_OF<MRSubstitution>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_MRSubstitution(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_MRMapping(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MRMapping")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MRMapping,
        _eal_components_for_MRMapping,
        _rctl2_components_for_MRMapping,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "mapping" => {
                |el: &X690Element| -> ASN1Result<()> {
                    if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                        return Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "mapping")
                        );
                    }
                    Ok(|el: &X690Element| -> ASN1Result<()> {
                        match &el.value {
                            X690Value::Constructed(subs) => {
                                for sub in subs.iter() {
                                    _validate_Mapping(&sub)?;
                                }
                                Ok(())
                            }
                            _ => Err(el
                                .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "mapping")),
                        }
                    }(&el.inner()?)?)
                }(_el)?
            }
            "substitution" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "substitution")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_MRSubstitution(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "substitution",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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
#[derive(Debug, Clone)]
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
        Vec::from([0])
    }
}
impl TryFrom<&X690Element> for Mapping {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Mapping")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Mapping,
        _eal_components_for_Mapping,
        _rctl2_components_for_Mapping,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut mappingFunction_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut level_: OPTIONAL<INTEGER> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "mappingFunction" => mappingFunction_ = Some(BER.decode_object_identifier(_el)?),
            "level" => level_ = Some(BER.decode_integer(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Mapping {
        mappingFunction: mappingFunction_.unwrap(),
        level: level_,
        _unrecognized,
    })
}

pub fn _encode_Mapping(value_: &Mapping) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.mappingFunction)?);
    if let Some(v_) = &value_.level {
        if *v_ != Mapping::_default_value_for_level() {
            components_.push(BER.encode_integer(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Mapping(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Mapping")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Mapping,
        _eal_components_for_Mapping,
        _rctl2_components_for_Mapping,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "mappingFunction" => BER.validate_object_identifier(_el)?,
            "level" => BER.validate_integer(_el)?,
            _ => (),
        }
    }
    Ok(())
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
#[derive(Debug, Clone)]
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
impl TryFrom<&X690Element> for MRSubstitution {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MRSubstitution"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MRSubstitution,
        _eal_components_for_MRSubstitution,
        _rctl2_components_for_MRSubstitution,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut attribute_: OPTIONAL<AttributeType> = None;
    let mut oldMatchingRule_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut newMatchingRule_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attribute" => attribute_ = Some(_decode_AttributeType(_el)?),
            "oldMatchingRule" => {
                oldMatchingRule_ = Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                    Ok(BER.decode_object_identifier(&el.inner()?)?)
                }(_el)?)
            }
            "newMatchingRule" => {
                newMatchingRule_ = Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                    Ok(BER.decode_object_identifier(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(MRSubstitution {
        attribute: attribute_.unwrap(),
        oldMatchingRule: oldMatchingRule_,
        newMatchingRule: newMatchingRule_,
        _unrecognized,
    })
}

pub fn _encode_MRSubstitution(value_: &MRSubstitution) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_AttributeType(&value_.attribute)?);
    if let Some(v_) = &value_.oldMatchingRule {
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(BER.encode_object_identifier(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.newMatchingRule {
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(BER.encode_object_identifier(&v_1)?),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_MRSubstitution(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MRSubstitution"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MRSubstitution,
        _eal_components_for_MRSubstitution,
        _rctl2_components_for_MRSubstitution,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attribute" => _validate_AttributeType(_el)?,
            "oldMatchingRule" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "oldMatchingRule",
                    ));
                }
                Ok(BER.validate_object_identifier(&el.inner()?)?)
            }(_el)?,
            "newMatchingRule" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "newMatchingRule",
                    ));
                }
                Ok(BER.validate_object_identifier(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
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
    pub DefaultValues: OPTIONAL<RequestAttribute_defaultValues_Item>,
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
    pub outputValues: OPTIONAL<ResultAttribute_outputValues>,
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
impl TryFrom<&X690Element> for RequestAttribute_defaultValues_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "RequestAttribute-defaultValues-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RequestAttribute_defaultValues_Item,
        _eal_components_for_RequestAttribute_defaultValues_Item,
        _rctl2_components_for_RequestAttribute_defaultValues_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut entryType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut values_: OPTIONAL<Vec<X690Element>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "entryType" => entryType_ = Some(BER.decode_object_identifier(_el)?),
            "values" => {
                values_ = Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<X690Element>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "values")
                            )
                        }
                    };
                    let mut items: SEQUENCE_OF<X690Element> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(x690_identity(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(RequestAttribute_defaultValues_Item {
        entryType: entryType_,
        values: values_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_RequestAttribute_defaultValues_Item(
    value_: &RequestAttribute_defaultValues_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.entryType {
        components_.push(BER.encode_object_identifier(&v_)?);
    }
    components_.push(
        |value_: &SEQUENCE_OF<X690Element>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(x690_identity(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.values)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_RequestAttribute_defaultValues_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "RequestAttribute-defaultValues-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RequestAttribute_defaultValues_Item,
        _eal_components_for_RequestAttribute_defaultValues_Item,
        _rctl2_components_for_RequestAttribute_defaultValues_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "entryType" => BER.validate_object_identifier(_el)?,
            "values" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            BER.validate_any(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "values")),
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
/// ResultAttribute-outputValues ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum ResultAttribute_outputValues {
    selectedValues(Vec<X690Element>),
    matchedValuesOnly(NULL),
}

impl TryFrom<&X690Element> for ResultAttribute_outputValues {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ResultAttribute_outputValues(el)
    }
}

pub fn _decode_ResultAttribute_outputValues(
    el: &X690Element,
) -> ASN1Result<ResultAttribute_outputValues> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(ResultAttribute_outputValues::selectedValues(
            |el: &X690Element| -> ASN1Result<SEQUENCE_OF<X690Element>> {
                let elements = match &el.value {
                    X690Value::Constructed(children) => children,
                    _ => {
                        return Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "selectedValues",
                        ))
                    }
                };
                let mut items: SEQUENCE_OF<X690Element> = Vec::with_capacity(elements.len());
                for el in elements.iter() {
                    items.push(x690_identity(el)?);
                }
                Ok(items)
            }(&el)?,
        )),
        (TagClass::UNIVERSAL, 5) => Ok(ResultAttribute_outputValues::matchedValuesOnly(
            BER.decode_null(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "ResultAttribute-outputValues",
            ))
        }
    }
}

pub fn _encode_ResultAttribute_outputValues(
    value_: &ResultAttribute_outputValues,
) -> ASN1Result<X690Element> {
    match value_ {
        ResultAttribute_outputValues::selectedValues(v) => {
            |value_: &SEQUENCE_OF<X690Element>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(x690_identity(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v)
        }
        ResultAttribute_outputValues::matchedValuesOnly(v) => BER.encode_null(&v),
    }
}

pub fn _validate_ResultAttribute_outputValues(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => |el: &X690Element| -> ASN1Result<()> {
            match &el.value {
                X690Value::Constructed(subs) => {
                    for sub in subs.iter() {
                        BER.validate_any(&sub)?;
                    }
                    Ok(())
                }
                _ => {
                    Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "selectedValues"))
                }
            }
        }(&el),
        (TagClass::UNIVERSAL, 5) => BER.validate_null(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "ResultAttribute-outputValues",
            ))
        }
    }
}
