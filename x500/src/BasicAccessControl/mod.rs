#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # BasicAccessControl
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `BasicAccessControl`.
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
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use wildboar_asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// accessControlScheme ATTRIBUTE ::= {
///   WITH SYNTAX             OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE  objectIdentifierMatch
///   SINGLE VALUE            TRUE
///   USAGE                   directoryOperation
///   ID                      id-aca-accessControlScheme }
/// ```
///
///
pub fn accessControlScheme() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                               /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),          /* OBJECT_FIELD_SETTING */
        id: id_aca_accessControlScheme(),                        /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod accessControlScheme {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = OBJECT_IDENTIFIER; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_object_identifier(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_object_identifier(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_object_identifier(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ACIItem ::= SEQUENCE {
///   identificationTag    UnboundedDirectoryString,
///   precedence           Precedence,
///   authenticationLevel  AuthenticationLevel,
///   itemOrUserFirst      CHOICE {
///     itemFirst       [0]  SEQUENCE {
///       protectedItems       ProtectedItems,
///       itemPermissions      SET OF ItemPermission,
///       ...},
///     userFirst       [1]  SEQUENCE {
///       userClasses          UserClasses,
///       userPermissions      SET OF UserPermission,
///       ...},
///     ...},
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct ACIItem {
    pub identificationTag: UnboundedDirectoryString,
    pub precedence: Precedence,
    pub authenticationLevel: AuthenticationLevel,
    pub itemOrUserFirst: ACIItem_itemOrUserFirst,
    pub _unrecognized: Vec<X690Element>,
}
impl ACIItem {
    pub fn new(
        identificationTag: UnboundedDirectoryString,
        precedence: Precedence,
        authenticationLevel: AuthenticationLevel,
        itemOrUserFirst: ACIItem_itemOrUserFirst,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ACIItem {
            identificationTag,
            precedence,
            authenticationLevel,
            itemOrUserFirst,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for ACIItem {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ACIItem(el)
    }
}

pub const _rctl1_components_for_ACIItem: &[ComponentSpec; 4] = &[
    ComponentSpec::new("identificationTag", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "precedence",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("authenticationLevel", false, TagSelector::any, None, None),
    ComponentSpec::new("itemOrUserFirst", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_ACIItem: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ACIItem: &[ComponentSpec; 0] = &[];

pub fn _decode_ACIItem(el: &X690Element) -> ASN1Result<ACIItem> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ACIItem")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ACIItem,
        _eal_components_for_ACIItem,
        _rctl2_components_for_ACIItem,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut identificationTag_: OPTIONAL<UnboundedDirectoryString> = None;
    let mut precedence_: OPTIONAL<Precedence> = None;
    let mut authenticationLevel_: OPTIONAL<AuthenticationLevel> = None;
    let mut itemOrUserFirst_: OPTIONAL<ACIItem_itemOrUserFirst> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "identificationTag" => {
                identificationTag_ = Some(_decode_UnboundedDirectoryString(_el)?)
            }
            "precedence" => precedence_ = Some(_decode_Precedence(_el)?),
            "authenticationLevel" => authenticationLevel_ = Some(_decode_AuthenticationLevel(_el)?),
            "itemOrUserFirst" => itemOrUserFirst_ = Some(_decode_ACIItem_itemOrUserFirst(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ACIItem {
        identificationTag: identificationTag_.unwrap(),
        precedence: precedence_.unwrap(),
        authenticationLevel: authenticationLevel_.unwrap(),
        itemOrUserFirst: itemOrUserFirst_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ACIItem(value_: &ACIItem) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    components_.push(_encode_UnboundedDirectoryString(&value_.identificationTag)?);
    components_.push(_encode_Precedence(&value_.precedence)?);
    components_.push(_encode_AuthenticationLevel(&value_.authenticationLevel)?);
    components_.push(_encode_ACIItem_itemOrUserFirst(&value_.itemOrUserFirst)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ACIItem(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ACIItem")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ACIItem,
        _eal_components_for_ACIItem,
        _rctl2_components_for_ACIItem,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "identificationTag" => _validate_UnboundedDirectoryString(_el)?,
            "precedence" => _validate_Precedence(_el)?,
            "authenticationLevel" => _validate_AuthenticationLevel(_el)?,
            "itemOrUserFirst" => _validate_ACIItem_itemOrUserFirst(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Precedence  ::=  INTEGER(0..255,...)
/// ```
pub type Precedence = INTEGER;

pub fn _decode_Precedence(el: &X690Element) -> ASN1Result<Precedence> {
    BER.decode_integer(&el)
}

pub fn _encode_Precedence(value_: &Precedence) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_Precedence(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ProtectedItems ::= SEQUENCE {
///   entry                          [0]  NULL OPTIONAL,
///   allUserAttributeTypes          [1]  NULL OPTIONAL,
///   attributeType                  [2]  SET SIZE (1..MAX) OF AttributeType
///                                         OPTIONAL,
///   allAttributeValues             [3]  SET SIZE (1..MAX) OF AttributeType
///                                         OPTIONAL,
///   allUserAttributeTypesAndValues [4]  NULL OPTIONAL,
///   attributeValue                 [5]  SET SIZE (1..MAX) OF AttributeTypeAndValue
///                                         OPTIONAL,
///   selfValue                      [6]  SET SIZE (1..MAX) OF AttributeType
///                                         OPTIONAL,
///   rangeOfValues                  [7]  Filter OPTIONAL,
///   maxValueCount                  [8]  SET SIZE (1..MAX) OF MaxValueCount
///                                         OPTIONAL,
///   maxImmSub                      [9]  INTEGER OPTIONAL,
///   restrictedBy                   [10] SET SIZE (1..MAX) OF RestrictedValue
///                                         OPTIONAL,
///   contexts                       [11] SET SIZE (1..MAX) OF ContextAssertion
///                                         OPTIONAL,
///   classes                        [12] Refinement OPTIONAL,
///   entryMethods                   [30]  SET OF OBJECT IDENTIFIER OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct ProtectedItems {
    pub entry: OPTIONAL<NULL>,
    pub allUserAttributeTypes: OPTIONAL<NULL>,
    pub attributeType: OPTIONAL<Vec<AttributeType>>,
    pub allAttributeValues: OPTIONAL<Vec<AttributeType>>,
    pub allUserAttributeTypesAndValues: OPTIONAL<NULL>,
    pub attributeValue: OPTIONAL<Vec<AttributeTypeAndValue>>,
    pub selfValue: OPTIONAL<Vec<AttributeType>>,
    pub rangeOfValues: OPTIONAL<Filter>,
    pub maxValueCount: OPTIONAL<Vec<MaxValueCount>>,
    pub maxImmSub: OPTIONAL<INTEGER>,
    pub restrictedBy: OPTIONAL<Vec<RestrictedValue>>,
    pub contexts: OPTIONAL<Vec<ContextAssertion>>,
    pub classes: OPTIONAL<Refinement>,
    pub entryMethods: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub _unrecognized: Vec<X690Element>,
}
impl ProtectedItems {
    pub fn new(
        entry: OPTIONAL<NULL>,
        allUserAttributeTypes: OPTIONAL<NULL>,
        attributeType: OPTIONAL<Vec<AttributeType>>,
        allAttributeValues: OPTIONAL<Vec<AttributeType>>,
        allUserAttributeTypesAndValues: OPTIONAL<NULL>,
        attributeValue: OPTIONAL<Vec<AttributeTypeAndValue>>,
        selfValue: OPTIONAL<Vec<AttributeType>>,
        rangeOfValues: OPTIONAL<Filter>,
        maxValueCount: OPTIONAL<Vec<MaxValueCount>>,
        maxImmSub: OPTIONAL<INTEGER>,
        restrictedBy: OPTIONAL<Vec<RestrictedValue>>,
        contexts: OPTIONAL<Vec<ContextAssertion>>,
        classes: OPTIONAL<Refinement>,
        entryMethods: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ProtectedItems {
            entry,
            allUserAttributeTypes,
            attributeType,
            allAttributeValues,
            allUserAttributeTypesAndValues,
            attributeValue,
            selfValue,
            rangeOfValues,
            maxValueCount,
            maxImmSub,
            restrictedBy,
            contexts,
            classes,
            entryMethods,
            _unrecognized,
        }
    }
}
impl Default for ProtectedItems {
    fn default() -> Self {
        ProtectedItems {
            entry: None,
            allUserAttributeTypes: None,
            attributeType: None,
            allAttributeValues: None,
            allUserAttributeTypesAndValues: None,
            attributeValue: None,
            selfValue: None,
            rangeOfValues: None,
            maxValueCount: None,
            maxImmSub: None,
            restrictedBy: None,
            contexts: None,
            classes: None,
            entryMethods: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<&X690Element> for ProtectedItems {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ProtectedItems(el)
    }
}

pub const _rctl1_components_for_ProtectedItems: &[ComponentSpec; 14] = &[
    ComponentSpec::new(
        "entry",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "allUserAttributeTypes",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeType",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "allAttributeValues",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "allUserAttributeTypesAndValues",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeValue",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "selfValue",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "rangeOfValues",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "maxValueCount",
        true,
        TagSelector::tag((TagClass::CONTEXT, 8)),
        None,
        None,
    ),
    ComponentSpec::new(
        "maxImmSub",
        true,
        TagSelector::tag((TagClass::CONTEXT, 9)),
        None,
        None,
    ),
    ComponentSpec::new(
        "restrictedBy",
        true,
        TagSelector::tag((TagClass::CONTEXT, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "contexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 11)),
        None,
        None,
    ),
    ComponentSpec::new(
        "classes",
        true,
        TagSelector::tag((TagClass::CONTEXT, 12)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryMethods",
        true,
        TagSelector::tag((TagClass::CONTEXT, 30)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ProtectedItems: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ProtectedItems: &[ComponentSpec; 0] = &[];

pub fn _decode_ProtectedItems(el: &X690Element) -> ASN1Result<ProtectedItems> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ProtectedItems"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ProtectedItems,
        _eal_components_for_ProtectedItems,
        _rctl2_components_for_ProtectedItems,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut entry_: OPTIONAL<NULL> = None;
    let mut allUserAttributeTypes_: OPTIONAL<NULL> = None;
    let mut attributeType_: OPTIONAL<Vec<AttributeType>> = None;
    let mut allAttributeValues_: OPTIONAL<Vec<AttributeType>> = None;
    let mut allUserAttributeTypesAndValues_: OPTIONAL<NULL> = None;
    let mut attributeValue_: OPTIONAL<Vec<AttributeTypeAndValue>> = None;
    let mut selfValue_: OPTIONAL<Vec<AttributeType>> = None;
    let mut rangeOfValues_: OPTIONAL<Filter> = None;
    let mut maxValueCount_: OPTIONAL<Vec<MaxValueCount>> = None;
    let mut maxImmSub_: OPTIONAL<INTEGER> = None;
    let mut restrictedBy_: OPTIONAL<Vec<RestrictedValue>> = None;
    let mut contexts_: OPTIONAL<Vec<ContextAssertion>> = None;
    let mut classes_: OPTIONAL<Refinement> = None;
    let mut entryMethods_: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "entry" => {
                entry_ = Some(|el: &X690Element| -> ASN1Result<NULL> {
                    Ok(BER.decode_null(&el.inner()?)?)
                }(_el)?)
            }
            "allUserAttributeTypes" => {
                allUserAttributeTypes_ = Some(|el: &X690Element| -> ASN1Result<NULL> {
                    Ok(BER.decode_null(&el.inner()?)?)
                }(_el)?)
            }
            "attributeType" => {
                attributeType_ = Some(|el: &X690Element| -> ASN1Result<Vec<AttributeType>> {
                    Ok(|el: &X690Element| -> ASN1Result<SET_OF<AttributeType>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "attributeType",
                                ))
                            }
                        };
                        let mut items: SET_OF<AttributeType> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_AttributeType(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "allAttributeValues" => {
                allAttributeValues_ = Some(|el: &X690Element| -> ASN1Result<Vec<AttributeType>> {
                    Ok(|el: &X690Element| -> ASN1Result<SET_OF<AttributeType>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "allAttributeValues",
                                ))
                            }
                        };
                        let mut items: SET_OF<AttributeType> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_AttributeType(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "allUserAttributeTypesAndValues" => {
                allUserAttributeTypesAndValues_ = Some(|el: &X690Element| -> ASN1Result<NULL> {
                    Ok(BER.decode_null(&el.inner()?)?)
                }(_el)?)
            }
            "attributeValue" => {
                attributeValue_ = Some(
                    |el: &X690Element| -> ASN1Result<Vec<AttributeTypeAndValue>> {
                        Ok(
                            |el: &X690Element| -> ASN1Result<SET_OF<AttributeTypeAndValue>> {
                                let elements = match &el.value {
                                    X690Value::Constructed(children) => children,
                                    _ => {
                                        return Err(el.to_asn1_err_named(
                                            ASN1ErrorCode::invalid_construction,
                                            "attributeValue",
                                        ))
                                    }
                                };
                                let mut items: SET_OF<AttributeTypeAndValue> =
                                    Vec::with_capacity(elements.len());
                                for el in elements.iter() {
                                    items.push(_decode_AttributeTypeAndValue(el)?);
                                }
                                Ok(items)
                            }(&el.inner()?)?,
                        )
                    }(_el)?,
                )
            }
            "selfValue" => {
                selfValue_ = Some(|el: &X690Element| -> ASN1Result<Vec<AttributeType>> {
                    Ok(|el: &X690Element| -> ASN1Result<SET_OF<AttributeType>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "selfValue",
                                ))
                            }
                        };
                        let mut items: SET_OF<AttributeType> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_AttributeType(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "rangeOfValues" => {
                rangeOfValues_ = Some(|el: &X690Element| -> ASN1Result<Filter> {
                    Ok(_decode_Filter(&el.inner()?)?)
                }(_el)?)
            }
            "maxValueCount" => {
                maxValueCount_ = Some(|el: &X690Element| -> ASN1Result<Vec<MaxValueCount>> {
                    Ok(|el: &X690Element| -> ASN1Result<SET_OF<MaxValueCount>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "maxValueCount",
                                ))
                            }
                        };
                        let mut items: SET_OF<MaxValueCount> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_MaxValueCount(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "maxImmSub" => {
                maxImmSub_ = Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                    Ok(BER.decode_integer(&el.inner()?)?)
                }(_el)?)
            }
            "restrictedBy" => {
                restrictedBy_ = Some(|el: &X690Element| -> ASN1Result<Vec<RestrictedValue>> {
                    Ok(|el: &X690Element| -> ASN1Result<SET_OF<RestrictedValue>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "restrictedBy",
                                ))
                            }
                        };
                        let mut items: SET_OF<RestrictedValue> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_RestrictedValue(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "contexts" => {
                contexts_ = Some(|el: &X690Element| -> ASN1Result<Vec<ContextAssertion>> {
                    Ok(|el: &X690Element| -> ASN1Result<SET_OF<ContextAssertion>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "contexts",
                                ))
                            }
                        };
                        let mut items: SET_OF<ContextAssertion> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_ContextAssertion(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "classes" => {
                classes_ = Some(|el: &X690Element| -> ASN1Result<Refinement> {
                    Ok(_decode_Refinement(&el.inner()?)?)
                }(_el)?)
            }
            "entryMethods" => {
                entryMethods_ = Some(|el: &X690Element| -> ASN1Result<Vec<OBJECT_IDENTIFIER>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "entryMethods",
                                    ))
                                }
                            };
                            let mut items: SET_OF<OBJECT_IDENTIFIER> =
                                Vec::with_capacity(elements.len());
                            for el in elements.iter() {
                                items.push(BER.decode_object_identifier(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ProtectedItems {
        entry: entry_,
        allUserAttributeTypes: allUserAttributeTypes_,
        attributeType: attributeType_,
        allAttributeValues: allAttributeValues_,
        allUserAttributeTypesAndValues: allUserAttributeTypesAndValues_,
        attributeValue: attributeValue_,
        selfValue: selfValue_,
        rangeOfValues: rangeOfValues_,
        maxValueCount: maxValueCount_,
        maxImmSub: maxImmSub_,
        restrictedBy: restrictedBy_,
        contexts: contexts_,
        classes: classes_,
        entryMethods: entryMethods_,
        _unrecognized,
    })
}

pub fn _encode_ProtectedItems(value_: &ProtectedItems) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(24);
    if let Some(v_) = &value_.entry {
        components_.push(|v_1: &NULL| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&BER.encode_null(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.allUserAttributeTypes {
        components_.push(|v_1: &NULL| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&BER.encode_null(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.attributeType {
        components_.push(|v_1: &Vec<AttributeType>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(
                    &|value_: &SET_OF<AttributeType>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AttributeType(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.allAttributeValues {
        components_.push(|v_1: &Vec<AttributeType>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(
                    &|value_: &SET_OF<AttributeType>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AttributeType(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.allUserAttributeTypesAndValues {
        components_.push(|v_1: &NULL| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::from_explicit(&BER.encode_null(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.attributeValue {
        components_.push(
            |v_1: &Vec<AttributeTypeAndValue>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 5),
                    X690Value::from_explicit(
                        &|value_: &SET_OF<AttributeTypeAndValue>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_AttributeTypeAndValue(&v)?);
                            }
                            Ok(X690Element::new(
                                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                                X690Value::Constructed(Arc::new(children)),
                            ))
                        }(&v_1)?,
                    ),
                ))
            }(&v_)?,
        );
    }
    if let Some(v_) = &value_.selfValue {
        components_.push(|v_1: &Vec<AttributeType>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 6),
                X690Value::from_explicit(
                    &|value_: &SET_OF<AttributeType>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_AttributeType(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.rangeOfValues {
        components_.push(|v_1: &Filter| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 7),
                X690Value::from_explicit(&_encode_Filter(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.maxValueCount {
        components_.push(|v_1: &Vec<MaxValueCount>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 8),
                X690Value::from_explicit(
                    &|value_: &SET_OF<MaxValueCount>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_MaxValueCount(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.maxImmSub {
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 9),
                X690Value::from_explicit(&BER.encode_integer(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.restrictedBy {
        components_.push(|v_1: &Vec<RestrictedValue>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 10),
                X690Value::from_explicit(
                    &|value_: &SET_OF<RestrictedValue>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_RestrictedValue(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.contexts {
        components_.push(|v_1: &Vec<ContextAssertion>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 11),
                X690Value::from_explicit(
                    &|value_: &SET_OF<ContextAssertion>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ContextAssertion(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.classes {
        components_.push(|v_1: &Refinement| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 12),
                X690Value::from_explicit(&_encode_Refinement(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.entryMethods {
        components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 30),
                X690Value::from_explicit(
                    &|value_: &SET_OF<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(BER.encode_object_identifier(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
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

pub fn _validate_ProtectedItems(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ProtectedItems"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ProtectedItems,
        _eal_components_for_ProtectedItems,
        _rctl2_components_for_ProtectedItems,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "entry" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "entry"));
                }
                Ok(BER.validate_null(&el.inner()?)?)
            }(_el)?,
            "allUserAttributeTypes" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "allUserAttributeTypes",
                    ));
                }
                Ok(BER.validate_null(&el.inner()?)?)
            }(_el)?,
            "attributeType" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributeType")
                    );
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
                            "attributeType",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "allAttributeValues" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "allAttributeValues",
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
                            "allAttributeValues",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "allUserAttributeTypesAndValues" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "allUserAttributeTypesAndValues",
                    ));
                }
                Ok(BER.validate_null(&el.inner()?)?)
            }(_el)?,
            "attributeValue" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributeValue")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_AttributeTypeAndValue(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "attributeValue",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "selfValue" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 6 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "selfValue")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_AttributeType(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "selfValue")
                        ),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "rangeOfValues" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 7 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "rangeOfValues")
                    );
                }
                Ok(_validate_Filter(&el.inner()?)?)
            }(_el)?,
            "maxValueCount" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 8 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "maxValueCount")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_MaxValueCount(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "maxValueCount",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "maxImmSub" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 9 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "maxImmSub")
                    );
                }
                Ok(BER.validate_integer(&el.inner()?)?)
            }(_el)?,
            "restrictedBy" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 10 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "restrictedBy")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_RestrictedValue(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "restrictedBy",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "contexts" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 11 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contexts")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_ContextAssertion(&sub)?;
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
            "classes" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 12 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "classes")
                    );
                }
                Ok(_validate_Refinement(&el.inner()?)?)
            }(_el)?,
            "entryMethods" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 30 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "entryMethods")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                BER.validate_object_identifier(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "entryMethods",
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
/// MaxValueCount ::= SEQUENCE {
///   type      AttributeType,
///   maxCount  INTEGER,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct MaxValueCount {
    pub type_: AttributeType,
    pub maxCount: INTEGER,
    pub _unrecognized: Vec<X690Element>,
}
impl MaxValueCount {
    pub fn new(type_: AttributeType, maxCount: INTEGER, _unrecognized: Vec<X690Element>) -> Self {
        MaxValueCount {
            type_,
            maxCount,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for MaxValueCount {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_MaxValueCount(el)
    }
}

pub const _rctl1_components_for_MaxValueCount: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "maxCount",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_MaxValueCount: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MaxValueCount: &[ComponentSpec; 0] = &[];

pub fn _decode_MaxValueCount(el: &X690Element) -> ASN1Result<MaxValueCount> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MaxValueCount")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MaxValueCount,
        _eal_components_for_MaxValueCount,
        _rctl2_components_for_MaxValueCount,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut type__: OPTIONAL<AttributeType> = None;
    let mut maxCount_: OPTIONAL<INTEGER> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => type__ = Some(_decode_AttributeType(_el)?),
            "maxCount" => maxCount_ = Some(BER.decode_integer(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(MaxValueCount {
        type_: type__.unwrap(),
        maxCount: maxCount_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_MaxValueCount(value_: &MaxValueCount) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_AttributeType(&value_.type_)?);
    components_.push(BER.encode_integer(&value_.maxCount)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_MaxValueCount(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MaxValueCount")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MaxValueCount,
        _eal_components_for_MaxValueCount,
        _rctl2_components_for_MaxValueCount,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => _validate_AttributeType(_el)?,
            "maxCount" => BER.validate_integer(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RestrictedValue ::= SEQUENCE {
///   type      AttributeType,
///   valuesIn  AttributeType,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct RestrictedValue {
    pub type_: AttributeType,
    pub valuesIn: AttributeType,
    pub _unrecognized: Vec<X690Element>,
}
impl RestrictedValue {
    pub fn new(
        type_: AttributeType,
        valuesIn: AttributeType,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        RestrictedValue {
            type_,
            valuesIn,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for RestrictedValue {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_RestrictedValue(el)
    }
}

pub const _rctl1_components_for_RestrictedValue: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "valuesIn",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_RestrictedValue: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_RestrictedValue: &[ComponentSpec; 0] = &[];

pub fn _decode_RestrictedValue(el: &X690Element) -> ASN1Result<RestrictedValue> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RestrictedValue"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RestrictedValue,
        _eal_components_for_RestrictedValue,
        _rctl2_components_for_RestrictedValue,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut type__: OPTIONAL<AttributeType> = None;
    let mut valuesIn_: OPTIONAL<AttributeType> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => type__ = Some(_decode_AttributeType(_el)?),
            "valuesIn" => valuesIn_ = Some(_decode_AttributeType(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(RestrictedValue {
        type_: type__.unwrap(),
        valuesIn: valuesIn_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_RestrictedValue(value_: &RestrictedValue) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_AttributeType(&value_.type_)?);
    components_.push(_encode_AttributeType(&value_.valuesIn)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_RestrictedValue(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RestrictedValue"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_RestrictedValue,
        _eal_components_for_RestrictedValue,
        _rctl2_components_for_RestrictedValue,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => _validate_AttributeType(_el)?,
            "valuesIn" => _validate_AttributeType(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UserClasses ::= SEQUENCE {
///   allUsers   [0]  NULL                                      OPTIONAL,
///   thisEntry  [1]  NULL                                      OPTIONAL,
///   name       [2]  SET SIZE (1..MAX) OF NameAndOptionalUID   OPTIONAL,
///   userGroup  [3]  SET SIZE (1..MAX) OF NameAndOptionalUID   OPTIONAL,
///                   -- dn component shall be the name of an
///                   -- entry of GroupOfUniqueNames
///   subtree    [4]  SET SIZE (1..MAX) OF SubtreeSpecification OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct UserClasses {
    pub allUsers: OPTIONAL<NULL>,
    pub thisEntry: OPTIONAL<NULL>,
    pub name: OPTIONAL<Vec<NameAndOptionalUID>>,
    pub userGroup: OPTIONAL<Vec<NameAndOptionalUID>>,
    pub subtree: OPTIONAL<Vec<SubtreeSpecification>>,
    pub _unrecognized: Vec<X690Element>,
}
impl UserClasses {
    pub fn new(
        allUsers: OPTIONAL<NULL>,
        thisEntry: OPTIONAL<NULL>,
        name: OPTIONAL<Vec<NameAndOptionalUID>>,
        userGroup: OPTIONAL<Vec<NameAndOptionalUID>>,
        subtree: OPTIONAL<Vec<SubtreeSpecification>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        UserClasses {
            allUsers,
            thisEntry,
            name,
            userGroup,
            subtree,
            _unrecognized,
        }
    }
}
impl Default for UserClasses {
    fn default() -> Self {
        UserClasses {
            allUsers: None,
            thisEntry: None,
            name: None,
            userGroup: None,
            subtree: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<&X690Element> for UserClasses {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UserClasses(el)
    }
}

pub const _rctl1_components_for_UserClasses: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "allUsers",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "thisEntry",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "userGroup",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subtree",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UserClasses: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UserClasses: &[ComponentSpec; 0] = &[];

pub fn _decode_UserClasses(el: &X690Element) -> ASN1Result<UserClasses> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserClasses")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UserClasses,
        _eal_components_for_UserClasses,
        _rctl2_components_for_UserClasses,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut allUsers_: OPTIONAL<NULL> = None;
    let mut thisEntry_: OPTIONAL<NULL> = None;
    let mut name_: OPTIONAL<Vec<NameAndOptionalUID>> = None;
    let mut userGroup_: OPTIONAL<Vec<NameAndOptionalUID>> = None;
    let mut subtree_: OPTIONAL<Vec<SubtreeSpecification>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "allUsers" => {
                allUsers_ = Some(|el: &X690Element| -> ASN1Result<NULL> {
                    Ok(BER.decode_null(&el.inner()?)?)
                }(_el)?)
            }
            "thisEntry" => {
                thisEntry_ = Some(|el: &X690Element| -> ASN1Result<NULL> {
                    Ok(BER.decode_null(&el.inner()?)?)
                }(_el)?)
            }
            "name" => {
                name_ = Some(|el: &X690Element| -> ASN1Result<Vec<NameAndOptionalUID>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SET_OF<NameAndOptionalUID>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "name",
                                    ))
                                }
                            };
                            let mut items: SET_OF<NameAndOptionalUID> =
                                Vec::with_capacity(elements.len());
                            for el in elements.iter() {
                                items.push(_decode_NameAndOptionalUID(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(_el)?)
            }
            "userGroup" => {
                userGroup_ = Some(|el: &X690Element| -> ASN1Result<Vec<NameAndOptionalUID>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SET_OF<NameAndOptionalUID>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "userGroup",
                                    ))
                                }
                            };
                            let mut items: SET_OF<NameAndOptionalUID> =
                                Vec::with_capacity(elements.len());
                            for el in elements.iter() {
                                items.push(_decode_NameAndOptionalUID(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(_el)?)
            }
            "subtree" => {
                subtree_ = Some(
                    |el: &X690Element| -> ASN1Result<Vec<SubtreeSpecification>> {
                        Ok(
                            |el: &X690Element| -> ASN1Result<SET_OF<SubtreeSpecification>> {
                                let elements = match &el.value {
                                    X690Value::Constructed(children) => children,
                                    _ => {
                                        return Err(el.to_asn1_err_named(
                                            ASN1ErrorCode::invalid_construction,
                                            "subtree",
                                        ))
                                    }
                                };
                                let mut items: SET_OF<SubtreeSpecification> =
                                    Vec::with_capacity(elements.len());
                                for el in elements.iter() {
                                    items.push(_decode_SubtreeSpecification(el)?);
                                }
                                Ok(items)
                            }(&el.inner()?)?,
                        )
                    }(_el)?,
                )
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(UserClasses {
        allUsers: allUsers_,
        thisEntry: thisEntry_,
        name: name_,
        userGroup: userGroup_,
        subtree: subtree_,
        _unrecognized,
    })
}

pub fn _encode_UserClasses(value_: &UserClasses) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    if let Some(v_) = &value_.allUsers {
        components_.push(|v_1: &NULL| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&BER.encode_null(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.thisEntry {
        components_.push(|v_1: &NULL| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&BER.encode_null(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.name {
        components_.push(|v_1: &Vec<NameAndOptionalUID>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(
                    &|value_: &SET_OF<NameAndOptionalUID>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_NameAndOptionalUID(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.userGroup {
        components_.push(|v_1: &Vec<NameAndOptionalUID>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(
                    &|value_: &SET_OF<NameAndOptionalUID>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_NameAndOptionalUID(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.subtree {
        components_.push(
            |v_1: &Vec<SubtreeSpecification>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 4),
                    X690Value::from_explicit(
                        &|value_: &SET_OF<SubtreeSpecification>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_SubtreeSpecification(&v)?);
                            }
                            Ok(X690Element::new(
                                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                                X690Value::Constructed(Arc::new(children)),
                            ))
                        }(&v_1)?,
                    ),
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

pub fn _validate_UserClasses(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserClasses")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UserClasses,
        _eal_components_for_UserClasses,
        _rctl2_components_for_UserClasses,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "allUsers" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "allUsers")
                    );
                }
                Ok(BER.validate_null(&el.inner()?)?)
            }(_el)?,
            "thisEntry" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "thisEntry")
                    );
                }
                Ok(BER.validate_null(&el.inner()?)?)
            }(_el)?,
            "name" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "name"));
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_NameAndOptionalUID(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "name")),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "userGroup" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "userGroup")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_NameAndOptionalUID(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "userGroup")
                        ),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "subtree" => {
                |el: &X690Element| -> ASN1Result<()> {
                    if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                        return Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "subtree")
                        );
                    }
                    Ok(|el: &X690Element| -> ASN1Result<()> {
                        match &el.value {
                            X690Value::Constructed(subs) => {
                                for sub in subs.iter() {
                                    _validate_SubtreeSpecification(&sub)?;
                                }
                                Ok(())
                            }
                            _ => Err(el
                                .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "subtree")),
                        }
                    }(&el.inner()?)?)
                }(_el)?
            }
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ItemPermission ::= SEQUENCE {
///   precedence        Precedence OPTIONAL,
///              -- defaults to precedence in ACIItem
///   userClasses       UserClasses,
///   grantsAndDenials  GrantsAndDenials,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct ItemPermission {
    pub precedence: OPTIONAL<Precedence>,
    pub userClasses: UserClasses,
    pub grantsAndDenials: GrantsAndDenials,
    pub _unrecognized: Vec<X690Element>,
}
impl ItemPermission {
    pub fn new(
        precedence: OPTIONAL<Precedence>,
        userClasses: UserClasses,
        grantsAndDenials: GrantsAndDenials,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ItemPermission {
            precedence,
            userClasses,
            grantsAndDenials,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for ItemPermission {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ItemPermission(el)
    }
}

pub const _rctl1_components_for_ItemPermission: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "precedence",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "userClasses",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "grantsAndDenials",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ItemPermission: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ItemPermission: &[ComponentSpec; 0] = &[];

pub fn _decode_ItemPermission(el: &X690Element) -> ASN1Result<ItemPermission> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ItemPermission"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ItemPermission,
        _eal_components_for_ItemPermission,
        _rctl2_components_for_ItemPermission,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut precedence_: OPTIONAL<Precedence> = None;
    let mut userClasses_: OPTIONAL<UserClasses> = None;
    let mut grantsAndDenials_: OPTIONAL<GrantsAndDenials> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "precedence" => precedence_ = Some(_decode_Precedence(_el)?),
            "userClasses" => userClasses_ = Some(_decode_UserClasses(_el)?),
            "grantsAndDenials" => grantsAndDenials_ = Some(_decode_GrantsAndDenials(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ItemPermission {
        precedence: precedence_,
        userClasses: userClasses_.unwrap(),
        grantsAndDenials: grantsAndDenials_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ItemPermission(value_: &ItemPermission) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.precedence {
        components_.push(_encode_Precedence(&v_)?);
    }
    components_.push(_encode_UserClasses(&value_.userClasses)?);
    components_.push(_encode_GrantsAndDenials(&value_.grantsAndDenials)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ItemPermission(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ItemPermission"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ItemPermission,
        _eal_components_for_ItemPermission,
        _rctl2_components_for_ItemPermission,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "precedence" => _validate_Precedence(_el)?,
            "userClasses" => _validate_UserClasses(_el)?,
            "grantsAndDenials" => _validate_GrantsAndDenials(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UserPermission ::= SEQUENCE {
///   precedence        Precedence OPTIONAL,
///              -- defaults to precedence in ACIItem
///   protectedItems    ProtectedItems,
///   grantsAndDenials  GrantsAndDenials,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct UserPermission {
    pub precedence: OPTIONAL<Precedence>,
    pub protectedItems: ProtectedItems,
    pub grantsAndDenials: GrantsAndDenials,
    pub _unrecognized: Vec<X690Element>,
}
impl UserPermission {
    pub fn new(
        precedence: OPTIONAL<Precedence>,
        protectedItems: ProtectedItems,
        grantsAndDenials: GrantsAndDenials,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        UserPermission {
            precedence,
            protectedItems,
            grantsAndDenials,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for UserPermission {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UserPermission(el)
    }
}

pub const _rctl1_components_for_UserPermission: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "precedence",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "protectedItems",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "grantsAndDenials",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UserPermission: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UserPermission: &[ComponentSpec; 0] = &[];

pub fn _decode_UserPermission(el: &X690Element) -> ASN1Result<UserPermission> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserPermission"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UserPermission,
        _eal_components_for_UserPermission,
        _rctl2_components_for_UserPermission,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut precedence_: OPTIONAL<Precedence> = None;
    let mut protectedItems_: OPTIONAL<ProtectedItems> = None;
    let mut grantsAndDenials_: OPTIONAL<GrantsAndDenials> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "precedence" => precedence_ = Some(_decode_Precedence(_el)?),
            "protectedItems" => protectedItems_ = Some(_decode_ProtectedItems(_el)?),
            "grantsAndDenials" => grantsAndDenials_ = Some(_decode_GrantsAndDenials(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(UserPermission {
        precedence: precedence_,
        protectedItems: protectedItems_.unwrap(),
        grantsAndDenials: grantsAndDenials_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_UserPermission(value_: &UserPermission) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.precedence {
        components_.push(_encode_Precedence(&v_)?);
    }
    components_.push(_encode_ProtectedItems(&value_.protectedItems)?);
    components_.push(_encode_GrantsAndDenials(&value_.grantsAndDenials)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_UserPermission(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserPermission"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UserPermission,
        _eal_components_for_UserPermission,
        _rctl2_components_for_UserPermission,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "precedence" => _validate_Precedence(_el)?,
            "protectedItems" => _validate_ProtectedItems(_el)?,
            "grantsAndDenials" => _validate_GrantsAndDenials(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthenticationLevel  ::=  CHOICE {
///   basicLevels     SEQUENCE {
///     level           ENUMERATED {none(0), simple(1), strong(2),...},
///     localQualifier  INTEGER OPTIONAL,
///     signed          BOOLEAN DEFAULT FALSE,
///     ...},
///   other           EXTERNAL,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum AuthenticationLevel {
    basicLevels(AuthenticationLevel_basicLevels),
    other(EXTERNAL),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for AuthenticationLevel {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AuthenticationLevel(el)
    }
}

pub fn _decode_AuthenticationLevel(el: &X690Element) -> ASN1Result<AuthenticationLevel> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(AuthenticationLevel::basicLevels(
            _decode_AuthenticationLevel_basicLevels(&el)?,
        )),
        (TagClass::UNIVERSAL, 8) => Ok(AuthenticationLevel::other(BER.decode_external(&el)?)),
        _ => Ok(AuthenticationLevel::_unrecognized(el.clone())),
    }
}

pub fn _encode_AuthenticationLevel(value_: &AuthenticationLevel) -> ASN1Result<X690Element> {
    match value_ {
        AuthenticationLevel::basicLevels(v) => _encode_AuthenticationLevel_basicLevels(&v),
        AuthenticationLevel::other(v) => BER.encode_external(&v),
        AuthenticationLevel::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_AuthenticationLevel(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_AuthenticationLevel_basicLevels(&el),
        (TagClass::UNIVERSAL, 8) => BER.validate_external(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// GrantsAndDenials  ::=  BIT STRING {
///   -- permissions that may be used in conjunction
///   -- with any component of ProtectedItems
///   grantAdd             (0),
///   denyAdd              (1),
///   grantDiscloseOnError (2),
///   denyDiscloseOnError  (3),
///   grantRead            (4),
///   denyRead             (5),
///   grantRemove          (6),
///   denyRemove           (7),
///   -- permissions that may be used only in conjunction
///   -- with the entry component
///   grantBrowse          (8),
///   denyBrowse           (9),
///   grantExport          (10),
///   denyExport           (11),
///   grantImport          (12),
///   denyImport           (13),
///   grantModify          (14),
///   denyModify           (15),
///   grantRename          (16),
///   denyRename           (17),
///   grantReturnDN        (18),
///   denyReturnDN         (19),
///   -- permissions that may be used in conjunction
///   -- with any component, except entry, of ProtectedItems
///   grantCompare         (20),
///   denyCompare          (21),
///   grantFilterMatch     (22),
///   denyFilterMatch      (23),
///   grantInvoke          (24),
///   denyInvoke           (25) }
/// ```
pub type GrantsAndDenials = BIT_STRING;

pub const GrantsAndDenials_grantAdd: BIT = 0; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyAdd: BIT = 1; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantDiscloseOnError: BIT = 2; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyDiscloseOnError: BIT = 3; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantRead: BIT = 4; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyRead: BIT = 5; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantRemove: BIT = 6; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyRemove: BIT = 7; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantBrowse: BIT = 8; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyBrowse: BIT = 9; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantExport: BIT = 10; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyExport: BIT = 11; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantImport: BIT = 12; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyImport: BIT = 13; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantModify: BIT = 14; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyModify: BIT = 15; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantRename: BIT = 16; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyRename: BIT = 17; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantReturnDN: BIT = 18; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyReturnDN: BIT = 19; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantCompare: BIT = 20; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyCompare: BIT = 21; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantFilterMatch: BIT = 22; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyFilterMatch: BIT = 23; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_grantInvoke: BIT = 24; /* LONG_NAMED_BIT */

pub const GrantsAndDenials_denyInvoke: BIT = 25; /* LONG_NAMED_BIT */

pub fn _decode_GrantsAndDenials(el: &X690Element) -> ASN1Result<GrantsAndDenials> {
    BER.decode_bit_string(&el)
}

pub fn _encode_GrantsAndDenials(value_: &GrantsAndDenials) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_GrantsAndDenials(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// prescriptiveACI ATTRIBUTE ::= {
///   WITH SYNTAX             ACIItem
///   EQUALITY MATCHING RULE  directoryStringFirstComponentMatch
///   USAGE                   directoryOperation
///   ID                      id-aca-prescriptiveACI }
/// ```
///
///
pub fn prescriptiveACI() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(directoryStringFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        id: id_aca_prescriptiveACI(),                   /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod prescriptiveACI {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ACIItem; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ACIItem(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ACIItem(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ACIItem(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// entryACI ATTRIBUTE ::= {
///   WITH SYNTAX             ACIItem
///   EQUALITY MATCHING RULE  directoryStringFirstComponentMatch
///   USAGE                   directoryOperation
///   ID                      id-aca-entryACI }
/// ```
///
///
pub fn entryACI() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(directoryStringFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        id: id_aca_entryACI(),                          /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod entryACI {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ACIItem; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ACIItem(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ACIItem(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ACIItem(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subentryACI ATTRIBUTE ::= {
///   WITH SYNTAX             ACIItem
///   EQUALITY MATCHING RULE  directoryStringFirstComponentMatch
///   USAGE                   directoryOperation
///   ID                      id-aca-subentryACI }
/// ```
///
///
pub fn subentryACI() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(directoryStringFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        id: id_aca_subentryACI(),                       /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod subentryACI {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ACIItem; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ACIItem(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ACIItem(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ACIItem(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aca-accessControlScheme     OBJECT IDENTIFIER ::= {id-aca 1}
/// ```
///
///
#[inline]
pub fn id_aca_accessControlScheme () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_aca(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aca-prescriptiveACI         OBJECT IDENTIFIER ::= {id-aca 4}
/// ```
///
///
#[inline]
pub fn id_aca_prescriptiveACI () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_aca(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aca-entryACI                OBJECT IDENTIFIER ::= {id-aca 5}
/// ```
///
///
#[inline]
pub fn id_aca_entryACI () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_aca(), 5).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aca-subentryACI             OBJECT IDENTIFIER ::= {id-aca 6}
/// ```
///
///
#[inline]
pub fn id_aca_subentryACI () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_aca(), 6).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// basicAccessControlScheme       OBJECT IDENTIFIER ::= {id-acScheme 1}
/// ```
///
///
#[inline]
pub fn basicAccessControlScheme () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_acScheme(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// simplifiedAccessControlScheme  OBJECT IDENTIFIER ::= {id-acScheme 2}
/// ```
///
///
#[inline]
pub fn simplifiedAccessControlScheme () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_acScheme(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rule-based-access-control      OBJECT IDENTIFIER ::= {id-acScheme 3}
/// ```
///
///
#[inline]
pub fn rule_based_access_control () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_acScheme(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rule-and-basic-access-control  OBJECT IDENTIFIER ::= {id-acScheme 4}
/// ```
///
///
#[inline]
pub fn rule_and_basic_access_control () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_acScheme(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rule-and-simple-access-control OBJECT IDENTIFIER ::= {id-acScheme 5}
/// ```
///
///
#[inline]
pub fn rule_and_simple_access_control () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_acScheme(), 5).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ACIItem-itemOrUserFirst-itemFirst ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct ACIItem_itemOrUserFirst_itemFirst {
    pub protectedItems: ProtectedItems,
    pub itemPermissions: Vec<ItemPermission>,
    pub _unrecognized: Vec<X690Element>,
}
impl ACIItem_itemOrUserFirst_itemFirst {
    pub fn new(
        protectedItems: ProtectedItems,
        itemPermissions: Vec<ItemPermission>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ACIItem_itemOrUserFirst_itemFirst {
            protectedItems,
            itemPermissions,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for ACIItem_itemOrUserFirst_itemFirst {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ACIItem_itemOrUserFirst_itemFirst(el)
    }
}

pub const _rctl1_components_for_ACIItem_itemOrUserFirst_itemFirst: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "protectedItems",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "itemPermissions",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ACIItem_itemOrUserFirst_itemFirst: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ACIItem_itemOrUserFirst_itemFirst: &[ComponentSpec; 0] = &[];

pub fn _decode_ACIItem_itemOrUserFirst_itemFirst(
    el: &X690Element,
) -> ASN1Result<ACIItem_itemOrUserFirst_itemFirst> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ACIItem-itemOrUserFirst-itemFirst",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ACIItem_itemOrUserFirst_itemFirst,
        _eal_components_for_ACIItem_itemOrUserFirst_itemFirst,
        _rctl2_components_for_ACIItem_itemOrUserFirst_itemFirst,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut protectedItems_: OPTIONAL<ProtectedItems> = None;
    let mut itemPermissions_: OPTIONAL<Vec<ItemPermission>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protectedItems" => protectedItems_ = Some(_decode_ProtectedItems(_el)?),
            "itemPermissions" => {
                itemPermissions_ = Some(|el: &X690Element| -> ASN1Result<SET_OF<ItemPermission>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "itemPermissions",
                            ))
                        }
                    };
                    let mut items: SET_OF<ItemPermission> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_ItemPermission(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ACIItem_itemOrUserFirst_itemFirst {
        protectedItems: protectedItems_.unwrap(),
        itemPermissions: itemPermissions_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ACIItem_itemOrUserFirst_itemFirst(
    value_: &ACIItem_itemOrUserFirst_itemFirst,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_ProtectedItems(&value_.protectedItems)?);
    components_.push(
        |value_: &SET_OF<ItemPermission>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_ItemPermission(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.itemPermissions)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ACIItem_itemOrUserFirst_itemFirst(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ACIItem-itemOrUserFirst-itemFirst",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ACIItem_itemOrUserFirst_itemFirst,
        _eal_components_for_ACIItem_itemOrUserFirst_itemFirst,
        _rctl2_components_for_ACIItem_itemOrUserFirst_itemFirst,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "protectedItems" => _validate_ProtectedItems(_el)?,
            "itemPermissions" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_ItemPermission(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el
                        .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "itemPermissions")),
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
/// ACIItem-itemOrUserFirst-userFirst ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct ACIItem_itemOrUserFirst_userFirst {
    pub userClasses: UserClasses,
    pub userPermissions: Vec<UserPermission>,
    pub _unrecognized: Vec<X690Element>,
}
impl ACIItem_itemOrUserFirst_userFirst {
    pub fn new(
        userClasses: UserClasses,
        userPermissions: Vec<UserPermission>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ACIItem_itemOrUserFirst_userFirst {
            userClasses,
            userPermissions,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for ACIItem_itemOrUserFirst_userFirst {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ACIItem_itemOrUserFirst_userFirst(el)
    }
}

pub const _rctl1_components_for_ACIItem_itemOrUserFirst_userFirst: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "userClasses",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "userPermissions",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ACIItem_itemOrUserFirst_userFirst: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ACIItem_itemOrUserFirst_userFirst: &[ComponentSpec; 0] = &[];

pub fn _decode_ACIItem_itemOrUserFirst_userFirst(
    el: &X690Element,
) -> ASN1Result<ACIItem_itemOrUserFirst_userFirst> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ACIItem-itemOrUserFirst-userFirst",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ACIItem_itemOrUserFirst_userFirst,
        _eal_components_for_ACIItem_itemOrUserFirst_userFirst,
        _rctl2_components_for_ACIItem_itemOrUserFirst_userFirst,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut userClasses_: OPTIONAL<UserClasses> = None;
    let mut userPermissions_: OPTIONAL<Vec<UserPermission>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "userClasses" => userClasses_ = Some(_decode_UserClasses(_el)?),
            "userPermissions" => {
                userPermissions_ = Some(|el: &X690Element| -> ASN1Result<SET_OF<UserPermission>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "userPermissions",
                            ))
                        }
                    };
                    let mut items: SET_OF<UserPermission> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_UserPermission(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ACIItem_itemOrUserFirst_userFirst {
        userClasses: userClasses_.unwrap(),
        userPermissions: userPermissions_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ACIItem_itemOrUserFirst_userFirst(
    value_: &ACIItem_itemOrUserFirst_userFirst,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_UserClasses(&value_.userClasses)?);
    components_.push(
        |value_: &SET_OF<UserPermission>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_UserPermission(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.userPermissions)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ACIItem_itemOrUserFirst_userFirst(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "ACIItem-itemOrUserFirst-userFirst",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ACIItem_itemOrUserFirst_userFirst,
        _eal_components_for_ACIItem_itemOrUserFirst_userFirst,
        _rctl2_components_for_ACIItem_itemOrUserFirst_userFirst,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "userClasses" => _validate_UserClasses(_el)?,
            "userPermissions" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_UserPermission(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el
                        .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "userPermissions")),
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
/// ACIItem-itemOrUserFirst ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum ACIItem_itemOrUserFirst {
    itemFirst(ACIItem_itemOrUserFirst_itemFirst),
    userFirst(ACIItem_itemOrUserFirst_userFirst),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for ACIItem_itemOrUserFirst {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ACIItem_itemOrUserFirst(el)
    }
}

pub fn _decode_ACIItem_itemOrUserFirst(el: &X690Element) -> ASN1Result<ACIItem_itemOrUserFirst> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(ACIItem_itemOrUserFirst::itemFirst(
            |el: &X690Element| -> ASN1Result<ACIItem_itemOrUserFirst_itemFirst> {
                Ok(_decode_ACIItem_itemOrUserFirst_itemFirst(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(ACIItem_itemOrUserFirst::userFirst(
            |el: &X690Element| -> ASN1Result<ACIItem_itemOrUserFirst_userFirst> {
                Ok(_decode_ACIItem_itemOrUserFirst_userFirst(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(ACIItem_itemOrUserFirst::_unrecognized(el.clone())),
    }
}

pub fn _encode_ACIItem_itemOrUserFirst(
    value_: &ACIItem_itemOrUserFirst,
) -> ASN1Result<X690Element> {
    match value_ {
        ACIItem_itemOrUserFirst::itemFirst(v) => {
            |v_1: &ACIItem_itemOrUserFirst_itemFirst| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&_encode_ACIItem_itemOrUserFirst_itemFirst(&v_1)?),
                ))
            }(&v)
        }
        ACIItem_itemOrUserFirst::userFirst(v) => {
            |v_1: &ACIItem_itemOrUserFirst_userFirst| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_ACIItem_itemOrUserFirst_userFirst(&v_1)?),
                ))
            }(&v)
        }
        ACIItem_itemOrUserFirst::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_ACIItem_itemOrUserFirst(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "itemFirst"));
            }
            Ok(_validate_ACIItem_itemOrUserFirst_itemFirst(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "userFirst"));
            }
            Ok(_validate_ACIItem_itemOrUserFirst_userFirst(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthenticationLevel-basicLevels-level ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type AuthenticationLevel_basicLevels_level = ENUMERATED;

pub const AuthenticationLevel_basicLevels_level_none: AuthenticationLevel_basicLevels_level = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AuthenticationLevel_basicLevels_level_simple: AuthenticationLevel_basicLevels_level = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AuthenticationLevel_basicLevels_level_strong: AuthenticationLevel_basicLevels_level = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_AuthenticationLevel_basicLevels_level(
    el: &X690Element,
) -> ASN1Result<AuthenticationLevel_basicLevels_level> {
    BER.decode_enumerated(&el)
}

pub fn _encode_AuthenticationLevel_basicLevels_level(
    value_: &AuthenticationLevel_basicLevels_level,
) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_AuthenticationLevel_basicLevels_level(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthenticationLevel-basicLevels ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct AuthenticationLevel_basicLevels {
    pub level: AuthenticationLevel_basicLevels_level,
    pub localQualifier: OPTIONAL<INTEGER>,
    pub signed: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl AuthenticationLevel_basicLevels {
    pub fn new(
        level: AuthenticationLevel_basicLevels_level,
        localQualifier: OPTIONAL<INTEGER>,
        signed: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AuthenticationLevel_basicLevels {
            level,
            localQualifier,
            signed,
            _unrecognized,
        }
    }
    pub fn _default_value_for_signed() -> BOOLEAN {
        false
    }
}
impl TryFrom<&X690Element> for AuthenticationLevel_basicLevels {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AuthenticationLevel_basicLevels(el)
    }
}

pub const _rctl1_components_for_AuthenticationLevel_basicLevels: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "level",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "localQualifier",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "signed",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AuthenticationLevel_basicLevels: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AuthenticationLevel_basicLevels: &[ComponentSpec; 0] = &[];

pub fn _decode_AuthenticationLevel_basicLevels(
    el: &X690Element,
) -> ASN1Result<AuthenticationLevel_basicLevels> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AuthenticationLevel-basicLevels",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AuthenticationLevel_basicLevels,
        _eal_components_for_AuthenticationLevel_basicLevels,
        _rctl2_components_for_AuthenticationLevel_basicLevels,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut level_: OPTIONAL<AuthenticationLevel_basicLevels_level> = None;
    let mut localQualifier_: OPTIONAL<INTEGER> = None;
    let mut signed_: OPTIONAL<BOOLEAN> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "level" => level_ = Some(_decode_AuthenticationLevel_basicLevels_level(_el)?),
            "localQualifier" => localQualifier_ = Some(BER.decode_integer(_el)?),
            "signed" => signed_ = Some(BER.decode_boolean(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AuthenticationLevel_basicLevels {
        level: level_.unwrap(),
        localQualifier: localQualifier_,
        signed: signed_,
        _unrecognized,
    })
}

pub fn _encode_AuthenticationLevel_basicLevels(
    value_: &AuthenticationLevel_basicLevels,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_AuthenticationLevel_basicLevels_level(
        &value_.level,
    )?);
    if let Some(v_) = &value_.localQualifier {
        components_.push(BER.encode_integer(&v_)?);
    }
    if let Some(v_) = &value_.signed {
        if *v_ != AuthenticationLevel_basicLevels::_default_value_for_signed() {
            components_.push(BER.encode_boolean(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AuthenticationLevel_basicLevels(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AuthenticationLevel-basicLevels",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AuthenticationLevel_basicLevels,
        _eal_components_for_AuthenticationLevel_basicLevels,
        _rctl2_components_for_AuthenticationLevel_basicLevels,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "level" => _validate_AuthenticationLevel_basicLevels_level(_el)?,
            "localQualifier" => BER.validate_integer(_el)?,
            "signed" => BER.validate_boolean(_el)?,
            _ => (),
        }
    }
    Ok(())
}
