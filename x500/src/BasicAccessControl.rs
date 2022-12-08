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
use asn1::*;
use std::borrow::Borrow;
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
impl TryFrom<X690Element> for ACIItem {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ACIItem(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ACIItem {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<ACIItem> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ACIItem,
            _eal_components_for_ACIItem,
            _rctl2_components_for_ACIItem,
        )?;
        let identificationTag =
            _decode_UnboundedDirectoryString(_components.get("identificationTag").unwrap())?;
        let precedence = _decode_Precedence(_components.get("precedence").unwrap())?;
        let authenticationLevel =
            _decode_AuthenticationLevel(_components.get("authenticationLevel").unwrap())?;
        let itemOrUserFirst =
            _decode_ACIItem_itemOrUserFirst(_components.get("itemOrUserFirst").unwrap())?;
        Ok(ACIItem {
            identificationTag,
            precedence,
            authenticationLevel,
            itemOrUserFirst,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ACIItem(value_: &ACIItem) -> ASN1Result<X690Element> {
    |value_: &ACIItem| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(_encode_UnboundedDirectoryString(&value_.identificationTag)?);
        components_.push(_encode_Precedence(&value_.precedence)?);
        components_.push(_encode_AuthenticationLevel(&value_.authenticationLevel)?);
        components_.push(_encode_ACIItem_itemOrUserFirst(&value_.itemOrUserFirst)?);
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
/// Precedence  ::=  INTEGER(0..255,...)
/// ```
pub type Precedence = INTEGER;

pub fn _decode_Precedence(el: &X690Element) -> ASN1Result<Precedence> {
    ber_decode_integer(&el)
}

pub fn _encode_Precedence(value_: &Precedence) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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
impl TryFrom<X690Element> for ProtectedItems {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ProtectedItems(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ProtectedItems {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<ProtectedItems> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ProtectedItems,
            _eal_components_for_ProtectedItems,
            _rctl2_components_for_ProtectedItems,
        )?;
        let entry: OPTIONAL<NULL> = match _components.get("entry") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<NULL> {
                ber_decode_null(&el.inner()?)?;
                Ok(())
            }(c_)?),
            _ => None,
        };
        let allUserAttributeTypes: OPTIONAL<NULL> = match _components.get("allUserAttributeTypes") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<NULL> {
                ber_decode_null(&el.inner()?)?;
                Ok(())
            }(c_)?),
            _ => None,
        };
        let attributeType: OPTIONAL<Vec<AttributeType>> = match _components.get("attributeType") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<AttributeType>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<AttributeType>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<AttributeType> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_AttributeType(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let allAttributeValues: OPTIONAL<Vec<AttributeType>> =
            match _components.get("allAttributeValues") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<AttributeType>> {
                    Ok(|el: &X690Element| -> ASN1Result<SET_OF<AttributeType>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SET_OF<AttributeType> = Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_AttributeType(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let allUserAttributeTypesAndValues: OPTIONAL<NULL> =
            match _components.get("allUserAttributeTypesAndValues") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<NULL> {
                    ber_decode_null(&el.inner()?)?;
                    Ok(())
                }(c_)?),
                _ => None,
            };
        let attributeValue: OPTIONAL<Vec<AttributeTypeAndValue>> = match _components
            .get("attributeValue")
        {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<Vec<AttributeTypeAndValue>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SET_OF<AttributeTypeAndValue>> {
                            let elements = match el.value.borrow() {
                                X690Encoding::Constructed(children) => children,
                                _ => {
                                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction))
                                }
                            };
                            let mut items: SET_OF<AttributeTypeAndValue> =
                                Vec::with_capacity(elements.len());
                            for el in elements {
                                items.push(_decode_AttributeTypeAndValue(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(c_)?,
            ),
            _ => None,
        };
        let selfValue: OPTIONAL<Vec<AttributeType>> = match _components.get("selfValue") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<AttributeType>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<AttributeType>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<AttributeType> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_AttributeType(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let rangeOfValues: OPTIONAL<Filter> = match _components.get("rangeOfValues") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Filter> {
                Ok(_decode_Filter(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let maxValueCount: OPTIONAL<Vec<MaxValueCount>> = match _components.get("maxValueCount") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<MaxValueCount>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<MaxValueCount>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<MaxValueCount> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_MaxValueCount(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let maxImmSub: OPTIONAL<INTEGER> = match _components.get("maxImmSub") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let restrictedBy: OPTIONAL<Vec<RestrictedValue>> = match _components.get("restrictedBy") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<RestrictedValue>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<RestrictedValue>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<RestrictedValue> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_RestrictedValue(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let contexts: OPTIONAL<Vec<ContextAssertion>> = match _components.get("contexts") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<ContextAssertion>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<ContextAssertion>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<ContextAssertion> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_ContextAssertion(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let classes: OPTIONAL<Refinement> = match _components.get("classes") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Refinement> {
                Ok(_decode_Refinement(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryMethods: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components.get("entryMethods") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<OBJECT_IDENTIFIER>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SET_OF<OBJECT_IDENTIFIER> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(ber_decode_object_identifier(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        Ok(ProtectedItems {
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
        })
    }(&el)
}

pub fn _encode_ProtectedItems(value_: &ProtectedItems) -> ASN1Result<X690Element> {
    |value_: &ProtectedItems| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(24);
        if let Some(v_) = &value_.entry {
            components_.push(|v_1: &NULL| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_null(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.allUserAttributeTypes {
            components_.push(|v_1: &NULL| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_null(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.attributeType {
            components_.push(|v_1: &Vec<AttributeType>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
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
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.allAttributeValues {
            components_.push(|v_1: &Vec<AttributeType>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
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
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.allUserAttributeTypesAndValues {
            components_.push(|v_1: &NULL| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_null(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.attributeValue {
            components_.push(
                |v_1: &Vec<AttributeTypeAndValue>| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        5,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            |value_: &SET_OF<AttributeTypeAndValue>| -> ASN1Result<X690Element> {
                                let mut children: Vec<X690Element> =
                                    Vec::with_capacity(value_.len());
                                for v in value_ {
                                    children.push(_encode_AttributeTypeAndValue(&v)?);
                                }
                                Ok(X690Element::new(
                                    TagClass::UNIVERSAL,
                                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                                    Arc::new(X690Encoding::Constructed(children)),
                                ))
                            }(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.selfValue {
            components_.push(|v_1: &Vec<AttributeType>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    6,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
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
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.rangeOfValues {
            components_.push(|v_1: &Filter| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    7,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Filter(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.maxValueCount {
            components_.push(|v_1: &Vec<MaxValueCount>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    8,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        MaxValueCount,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_MaxValueCount(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.maxImmSub {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    9,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.restrictedBy {
            components_.push(|v_1: &Vec<RestrictedValue>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    10,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        RestrictedValue,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_RestrictedValue(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.contexts {
            components_.push(|v_1: &Vec<ContextAssertion>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    11,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        ContextAssertion,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ContextAssertion(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.classes {
            components_.push(|v_1: &Refinement| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    12,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Refinement(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entryMethods {
            components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    30,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        OBJECT_IDENTIFIER,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(ber_encode_object_identifier(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
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
/// MaxValueCount ::= SEQUENCE {
///   type      AttributeType,
///   maxCount  INTEGER,
///   ... }
/// ```
///
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
impl TryFrom<X690Element> for MaxValueCount {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_MaxValueCount(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for MaxValueCount {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<MaxValueCount> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_MaxValueCount,
            _eal_components_for_MaxValueCount,
            _rctl2_components_for_MaxValueCount,
        )?;
        let type_ = _decode_AttributeType(_components.get("type").unwrap())?;
        let maxCount = ber_decode_integer(_components.get("maxCount").unwrap())?;
        Ok(MaxValueCount {
            type_,
            maxCount,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_MaxValueCount(value_: &MaxValueCount) -> ASN1Result<X690Element> {
    |value_: &MaxValueCount| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AttributeType(&value_.type_)?);
        components_.push(ber_encode_integer(&value_.maxCount)?);
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
/// RestrictedValue ::= SEQUENCE {
///   type      AttributeType,
///   valuesIn  AttributeType,
///   ... }
/// ```
///
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
impl TryFrom<X690Element> for RestrictedValue {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_RestrictedValue(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for RestrictedValue {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<RestrictedValue> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_RestrictedValue,
            _eal_components_for_RestrictedValue,
            _rctl2_components_for_RestrictedValue,
        )?;
        let type_ = _decode_AttributeType(_components.get("type").unwrap())?;
        let valuesIn = _decode_AttributeType(_components.get("valuesIn").unwrap())?;
        Ok(RestrictedValue {
            type_,
            valuesIn,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_RestrictedValue(value_: &RestrictedValue) -> ASN1Result<X690Element> {
    |value_: &RestrictedValue| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_AttributeType(&value_.type_)?);
        components_.push(_encode_AttributeType(&value_.valuesIn)?);
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
impl TryFrom<X690Element> for UserClasses {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UserClasses(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UserClasses {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<UserClasses> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UserClasses,
            _eal_components_for_UserClasses,
            _rctl2_components_for_UserClasses,
        )?;
        let allUsers: OPTIONAL<NULL> = match _components.get("allUsers") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<NULL> {
                ber_decode_null(&el.inner()?)?;
                Ok(())
            }(c_)?),
            _ => None,
        };
        let thisEntry: OPTIONAL<NULL> = match _components.get("thisEntry") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<NULL> {
                ber_decode_null(&el.inner()?)?;
                Ok(())
            }(c_)?),
            _ => None,
        };
        let name: OPTIONAL<Vec<NameAndOptionalUID>> = match _components.get("name") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<NameAndOptionalUID>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SET_OF<NameAndOptionalUID>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SET_OF<NameAndOptionalUID> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_NameAndOptionalUID(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        let userGroup: OPTIONAL<Vec<NameAndOptionalUID>> = match _components.get("userGroup") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<NameAndOptionalUID>> {
                Ok(
                    |el: &X690Element| -> ASN1Result<SET_OF<NameAndOptionalUID>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SET_OF<NameAndOptionalUID> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_NameAndOptionalUID(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?,
                )
            }(c_)?),
            _ => None,
        };
        let subtree: OPTIONAL<Vec<SubtreeSpecification>> = match _components.get("subtree") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<Vec<SubtreeSpecification>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SET_OF<SubtreeSpecification>> {
                            let elements = match el.value.borrow() {
                                X690Encoding::Constructed(children) => children,
                                _ => {
                                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction))
                                }
                            };
                            let mut items: SET_OF<SubtreeSpecification> =
                                Vec::with_capacity(elements.len());
                            for el in elements {
                                items.push(_decode_SubtreeSpecification(el)?);
                            }
                            Ok(items)
                        }(&el.inner()?)?,
                    )
                }(c_)?,
            ),
            _ => None,
        };
        Ok(UserClasses {
            allUsers,
            thisEntry,
            name,
            userGroup,
            subtree,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_UserClasses(value_: &UserClasses) -> ASN1Result<X690Element> {
    |value_: &UserClasses| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        if let Some(v_) = &value_.allUsers {
            components_.push(|v_1: &NULL| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_null(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.thisEntry {
            components_.push(|v_1: &NULL| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_null(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.name {
            components_.push(|v_1: &Vec<NameAndOptionalUID>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        NameAndOptionalUID,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_NameAndOptionalUID(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.userGroup {
            components_.push(|v_1: &Vec<NameAndOptionalUID>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        NameAndOptionalUID,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_NameAndOptionalUID(&v)?);
                        }
                        Ok(X690Element::new(
                            TagClass::UNIVERSAL,
                            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                            Arc::new(X690Encoding::Constructed(children)),
                        ))
                    }(
                        &v_1
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.subtree {
            components_.push(
                |v_1: &Vec<SubtreeSpecification>| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            |value_: &SET_OF<SubtreeSpecification>| -> ASN1Result<X690Element> {
                                let mut children: Vec<X690Element> =
                                    Vec::with_capacity(value_.len());
                                for v in value_ {
                                    children.push(_encode_SubtreeSpecification(&v)?);
                                }
                                Ok(X690Element::new(
                                    TagClass::UNIVERSAL,
                                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                                    Arc::new(X690Encoding::Constructed(children)),
                                ))
                            }(&v_1)?,
                        ))),
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
/// ItemPermission ::= SEQUENCE {
///   precedence        Precedence OPTIONAL,
///              -- defaults to precedence in ACIItem
///   userClasses       UserClasses,
///   grantsAndDenials  GrantsAndDenials,
///   ... }
/// ```
///
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
impl TryFrom<X690Element> for ItemPermission {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ItemPermission(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ItemPermission {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<ItemPermission> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ItemPermission,
            _eal_components_for_ItemPermission,
            _rctl2_components_for_ItemPermission,
        )?;
        let precedence: OPTIONAL<Precedence> = match _components.get("precedence") {
            Some(c_) => Some(_decode_Precedence(c_)?),
            _ => None,
        };
        let userClasses = _decode_UserClasses(_components.get("userClasses").unwrap())?;
        let grantsAndDenials =
            _decode_GrantsAndDenials(_components.get("grantsAndDenials").unwrap())?;
        Ok(ItemPermission {
            precedence,
            userClasses,
            grantsAndDenials,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ItemPermission(value_: &ItemPermission) -> ASN1Result<X690Element> {
    |value_: &ItemPermission| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.precedence {
            components_.push(_encode_Precedence(&v_)?);
        }
        components_.push(_encode_UserClasses(&value_.userClasses)?);
        components_.push(_encode_GrantsAndDenials(&value_.grantsAndDenials)?);
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
/// UserPermission ::= SEQUENCE {
///   precedence        Precedence OPTIONAL,
///              -- defaults to precedence in ACIItem
///   protectedItems    ProtectedItems,
///   grantsAndDenials  GrantsAndDenials,
///   ... }
/// ```
///
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
impl TryFrom<X690Element> for UserPermission {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UserPermission(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UserPermission {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<UserPermission> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UserPermission,
            _eal_components_for_UserPermission,
            _rctl2_components_for_UserPermission,
        )?;
        let precedence: OPTIONAL<Precedence> = match _components.get("precedence") {
            Some(c_) => Some(_decode_Precedence(c_)?),
            _ => None,
        };
        let protectedItems = _decode_ProtectedItems(_components.get("protectedItems").unwrap())?;
        let grantsAndDenials =
            _decode_GrantsAndDenials(_components.get("grantsAndDenials").unwrap())?;
        Ok(UserPermission {
            precedence,
            protectedItems,
            grantsAndDenials,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_UserPermission(value_: &UserPermission) -> ASN1Result<X690Element> {
    |value_: &UserPermission| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.precedence {
            components_.push(_encode_Precedence(&v_)?);
        }
        components_.push(_encode_ProtectedItems(&value_.protectedItems)?);
        components_.push(_encode_GrantsAndDenials(&value_.grantsAndDenials)?);
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

impl TryFrom<X690Element> for AuthenticationLevel {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AuthenticationLevel(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AuthenticationLevel {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AuthenticationLevel(el)
    }
}

pub fn _decode_AuthenticationLevel(el: &X690Element) -> ASN1Result<AuthenticationLevel> {
    |el: &X690Element| -> ASN1Result<AuthenticationLevel> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(AuthenticationLevel::basicLevels(
                _decode_AuthenticationLevel_basicLevels(&el)?,
            )),
            (TagClass::UNIVERSAL, 8) => Ok(AuthenticationLevel::other(ber_decode_external(&el)?)),
            _ => Ok(AuthenticationLevel::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_AuthenticationLevel(value_: &AuthenticationLevel) -> ASN1Result<X690Element> {
    |value: &AuthenticationLevel| -> ASN1Result<X690Element> {
        match value {
            AuthenticationLevel::basicLevels(v) => _encode_AuthenticationLevel_basicLevels(&v),
            AuthenticationLevel::other(v) => ber_encode_external(&v),
            AuthenticationLevel::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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
    ber_decode_bit_string(&el)
}

pub fn _encode_GrantsAndDenials(value_: &GrantsAndDenials) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aca-accessControlScheme     OBJECT IDENTIFIER ::= {id-aca 1}
/// ```
///
///
pub fn id_aca_accessControlScheme() -> OBJECT_IDENTIFIER {
    [id_aca(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aca-prescriptiveACI         OBJECT IDENTIFIER ::= {id-aca 4}
/// ```
///
///
pub fn id_aca_prescriptiveACI() -> OBJECT_IDENTIFIER {
    [id_aca(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aca-entryACI                OBJECT IDENTIFIER ::= {id-aca 5}
/// ```
///
///
pub fn id_aca_entryACI() -> OBJECT_IDENTIFIER {
    [id_aca(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aca-subentryACI             OBJECT IDENTIFIER ::= {id-aca 6}
/// ```
///
///
pub fn id_aca_subentryACI() -> OBJECT_IDENTIFIER {
    [id_aca(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// basicAccessControlScheme       OBJECT IDENTIFIER ::= {id-acScheme 1}
/// ```
///
///
pub fn basicAccessControlScheme() -> OBJECT_IDENTIFIER {
    [id_acScheme(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// simplifiedAccessControlScheme  OBJECT IDENTIFIER ::= {id-acScheme 2}
/// ```
///
///
pub fn simplifiedAccessControlScheme() -> OBJECT_IDENTIFIER {
    [id_acScheme(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rule-based-access-control      OBJECT IDENTIFIER ::= {id-acScheme 3}
/// ```
///
///
pub fn rule_based_access_control() -> OBJECT_IDENTIFIER {
    [id_acScheme(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rule-and-basic-access-control  OBJECT IDENTIFIER ::= {id-acScheme 4}
/// ```
///
///
pub fn rule_and_basic_access_control() -> OBJECT_IDENTIFIER {
    [id_acScheme(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rule-and-simple-access-control OBJECT IDENTIFIER ::= {id-acScheme 5}
/// ```
///
///
pub fn rule_and_simple_access_control() -> OBJECT_IDENTIFIER {
    [id_acScheme(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ACIItem-itemOrUserFirst-itemFirst ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
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
impl TryFrom<X690Element> for ACIItem_itemOrUserFirst_itemFirst {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ACIItem_itemOrUserFirst_itemFirst(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ACIItem_itemOrUserFirst_itemFirst {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<ACIItem_itemOrUserFirst_itemFirst> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ACIItem_itemOrUserFirst_itemFirst,
            _eal_components_for_ACIItem_itemOrUserFirst_itemFirst,
            _rctl2_components_for_ACIItem_itemOrUserFirst_itemFirst,
        )?;
        let protectedItems = _decode_ProtectedItems(_components.get("protectedItems").unwrap())?;
        let itemPermissions = |el: &X690Element| -> ASN1Result<SET_OF<ItemPermission>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SET_OF<ItemPermission> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_ItemPermission(el)?);
            }
            Ok(items)
        }(_components.get("itemPermissions").unwrap())?;
        Ok(ACIItem_itemOrUserFirst_itemFirst {
            protectedItems,
            itemPermissions,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ACIItem_itemOrUserFirst_itemFirst(
    value_: &ACIItem_itemOrUserFirst_itemFirst,
) -> ASN1Result<X690Element> {
    |value_: &ACIItem_itemOrUserFirst_itemFirst| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_ProtectedItems(&value_.protectedItems)?);
        components_.push(
            |value_: &SET_OF<ItemPermission>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_ItemPermission(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.itemPermissions)?,
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
/// ACIItem-itemOrUserFirst-userFirst ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
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
impl TryFrom<X690Element> for ACIItem_itemOrUserFirst_userFirst {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ACIItem_itemOrUserFirst_userFirst(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ACIItem_itemOrUserFirst_userFirst {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<ACIItem_itemOrUserFirst_userFirst> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ACIItem_itemOrUserFirst_userFirst,
            _eal_components_for_ACIItem_itemOrUserFirst_userFirst,
            _rctl2_components_for_ACIItem_itemOrUserFirst_userFirst,
        )?;
        let userClasses = _decode_UserClasses(_components.get("userClasses").unwrap())?;
        let userPermissions = |el: &X690Element| -> ASN1Result<SET_OF<UserPermission>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SET_OF<UserPermission> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_UserPermission(el)?);
            }
            Ok(items)
        }(_components.get("userPermissions").unwrap())?;
        Ok(ACIItem_itemOrUserFirst_userFirst {
            userClasses,
            userPermissions,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ACIItem_itemOrUserFirst_userFirst(
    value_: &ACIItem_itemOrUserFirst_userFirst,
) -> ASN1Result<X690Element> {
    |value_: &ACIItem_itemOrUserFirst_userFirst| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_UserClasses(&value_.userClasses)?);
        components_.push(
            |value_: &SET_OF<UserPermission>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_UserPermission(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.userPermissions)?,
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
/// ACIItem-itemOrUserFirst ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum ACIItem_itemOrUserFirst {
    itemFirst(ACIItem_itemOrUserFirst_itemFirst),
    userFirst(ACIItem_itemOrUserFirst_userFirst),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ACIItem_itemOrUserFirst {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ACIItem_itemOrUserFirst(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ACIItem_itemOrUserFirst {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ACIItem_itemOrUserFirst(el)
    }
}

pub fn _decode_ACIItem_itemOrUserFirst(el: &X690Element) -> ASN1Result<ACIItem_itemOrUserFirst> {
    |el: &X690Element| -> ASN1Result<ACIItem_itemOrUserFirst> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(ACIItem_itemOrUserFirst::itemFirst(
                _decode_ACIItem_itemOrUserFirst_itemFirst(&el.inner()?)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(ACIItem_itemOrUserFirst::userFirst(
                _decode_ACIItem_itemOrUserFirst_userFirst(&el.inner()?)?,
            )),
            _ => Ok(ACIItem_itemOrUserFirst::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ACIItem_itemOrUserFirst(
    value_: &ACIItem_itemOrUserFirst,
) -> ASN1Result<X690Element> {
    |value: &ACIItem_itemOrUserFirst| -> ASN1Result<X690Element> {
        match value {
            ACIItem_itemOrUserFirst::itemFirst(v) => {
                |v_1: &ACIItem_itemOrUserFirst_itemFirst| -> ASN1Result<X690Element> {
                    let el_1 = _encode_ACIItem_itemOrUserFirst_itemFirst(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            ACIItem_itemOrUserFirst::userFirst(v) => {
                |v_1: &ACIItem_itemOrUserFirst_userFirst| -> ASN1Result<X690Element> {
                    let el_1 = _encode_ACIItem_itemOrUserFirst_userFirst(&v_1)?;
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(el_1))),
                    ))
                }(&v)
            }
            ACIItem_itemOrUserFirst::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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
    ber_decode_enumerated(&el)
}

pub fn _encode_AuthenticationLevel_basicLevels_level(
    value_: &AuthenticationLevel_basicLevels_level,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthenticationLevel-basicLevels ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
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
impl TryFrom<X690Element> for AuthenticationLevel_basicLevels {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AuthenticationLevel_basicLevels(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AuthenticationLevel_basicLevels {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<AuthenticationLevel_basicLevels> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AuthenticationLevel_basicLevels,
            _eal_components_for_AuthenticationLevel_basicLevels,
            _rctl2_components_for_AuthenticationLevel_basicLevels,
        )?;
        let level =
            _decode_AuthenticationLevel_basicLevels_level(_components.get("level").unwrap())?;
        let localQualifier: OPTIONAL<INTEGER> = match _components.get("localQualifier") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        let signed: OPTIONAL<BOOLEAN> = match _components.get("signed") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        Ok(AuthenticationLevel_basicLevels {
            level,
            localQualifier,
            signed,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AuthenticationLevel_basicLevels(
    value_: &AuthenticationLevel_basicLevels,
) -> ASN1Result<X690Element> {
    |value_: &AuthenticationLevel_basicLevels| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_AuthenticationLevel_basicLevels_level(
            &value_.level,
        )?);
        if let Some(v_) = &value_.localQualifier {
            components_.push(ber_encode_integer(&v_)?);
        }
        if let Some(v_) = &value_.signed {
            if *v_ != AuthenticationLevel_basicLevels::_default_value_for_signed() {
                components_.push(ber_encode_boolean(&v_)?);
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
