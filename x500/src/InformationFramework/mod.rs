#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # InformationFramework
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `InformationFramework`.
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
use crate::SelectedAttributeTypes::*;
use crate::ServiceAdministration::*;
use crate::UsefulDefinitions::*;
use wildboar_asn1::*;
use std::sync::Arc;
use x690::*;
mod name;
pub use name::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// Attribute {ATTRIBUTE:SupportedAttributes} ::= SEQUENCE {
///   type                ATTRIBUTE.&id({SupportedAttributes}),
///   values              SET SIZE (0..MAX) OF ATTRIBUTE.&Type({SupportedAttributes}{@type}),
///   valuesWithContext   SET SIZE (1..MAX) OF SEQUENCE {
///     value               ATTRIBUTE.&Type({SupportedAttributes}{@type}),
///     contextList         SET SIZE (1..MAX) OF Context,
///     ...} OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Attribute {
    pub type_: OBJECT_IDENTIFIER,
    pub values: Vec<X690Element>,
    pub valuesWithContext: OPTIONAL<Vec<Attribute_valuesWithContext_Item>>,
    pub _unrecognized: Vec<X690Element>,
}
impl Attribute {
    pub fn new(
        type_: OBJECT_IDENTIFIER,
        values: Vec<X690Element>,
        valuesWithContext: OPTIONAL<Vec<Attribute_valuesWithContext_Item>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Attribute {
            type_,
            values,
            valuesWithContext,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for Attribute {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Attribute(el)
    }
}

pub const _rctl1_components_for_Attribute: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "values",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "valuesWithContext",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Attribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Attribute: &[ComponentSpec; 0] = &[];

pub fn _decode_Attribute(el: &X690Element) -> ASN1Result<Attribute> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Attribute")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Attribute,
        _eal_components_for_Attribute,
        _rctl2_components_for_Attribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut type__: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut values_: OPTIONAL<Vec<X690Element>> = None;
    let mut valuesWithContext_: OPTIONAL<Vec<Attribute_valuesWithContext_Item>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => type__ = Some(BER.decode_object_identifier(_el)?),
            "values" => {
                values_ = Some(|el: &X690Element| -> ASN1Result<SET_OF<X690Element>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "values")
                            )
                        }
                    };
                    let mut items: SET_OF<X690Element> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(x690_identity(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            "valuesWithContext" => {
                valuesWithContext_ = Some(|el: &X690Element| -> ASN1Result<
                    SET_OF<Attribute_valuesWithContext_Item>,
                > {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "valuesWithContext",
                            ))
                        }
                    };
                    let mut items: SET_OF<Attribute_valuesWithContext_Item> =
                        Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_Attribute_valuesWithContext_Item(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Attribute {
        type_: type__.unwrap(),
        values: values_.unwrap(),
        valuesWithContext: valuesWithContext_,
        _unrecognized,
    })
}

pub fn _encode_Attribute(value_: &Attribute) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_object_identifier(&value_.type_)?);
    components_.push(|value_: &SET_OF<X690Element>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(x690_identity(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }(&value_.values)?);
    if let Some(v_) = &value_.valuesWithContext {
        components_.push(
            |value_: &SET_OF<Attribute_valuesWithContext_Item>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_Attribute_valuesWithContext_Item(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
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

pub fn _validate_Attribute(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Attribute")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Attribute,
        _eal_components_for_Attribute,
        _rctl2_components_for_Attribute,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => BER.validate_object_identifier(_el)?,
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
            "valuesWithContext" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_Attribute_valuesWithContext_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "valuesWithContext",
                    )),
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
/// AttributeType  ::=  ATTRIBUTE.&id
/// ```
pub type AttributeType = OBJECT_IDENTIFIER; // ObjectClassFieldType

pub fn _decode_AttributeType(el: &X690Element) -> ASN1Result<AttributeType> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_AttributeType(value_: &AttributeType) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_AttributeType(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeValue  ::=  ATTRIBUTE.&Type
/// ```
pub type AttributeValue = X690Element; // ObjectClassFieldType

pub fn _decode_AttributeValue(el: &X690Element) -> ASN1Result<AttributeValue> {
    x690_identity(&el)
}

pub fn _encode_AttributeValue(value_: &AttributeValue) -> ASN1Result<X690Element> {
    x690_identity(&value_)
}

pub fn _validate_AttributeValue(el: &X690Element) -> ASN1Result<()> {
    BER.validate_any(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Context ::= SEQUENCE {
///   contextType    CONTEXT.&id({SupportedContexts}),
///   contextValues
///     SET SIZE (1..MAX) OF CONTEXT.&Type({SupportedContexts}{@contextType}),
///   fallback       BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Context {
    pub contextType: OBJECT_IDENTIFIER,
    pub contextValues: Vec<X690Element>,
    pub fallback: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl Context {
    pub fn new(
        contextType: OBJECT_IDENTIFIER,
        contextValues: Vec<X690Element>,
        fallback: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Context {
            contextType,
            contextValues,
            fallback,
            _unrecognized,
        }
    }
    pub fn _default_value_for_fallback() -> BOOLEAN {
        false
    }
}
impl TryFrom<&X690Element> for Context {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Context(el)
    }
}

pub const _rctl1_components_for_Context: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "contextType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "contextValues",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "fallback",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Context: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Context: &[ComponentSpec; 0] = &[];

pub fn _decode_Context(el: &X690Element) -> ASN1Result<Context> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Context,
        _eal_components_for_Context,
        _rctl2_components_for_Context,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut contextType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut contextValues_: OPTIONAL<Vec<X690Element>> = None;
    let mut fallback_: OPTIONAL<BOOLEAN> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contextType" => contextType_ = Some(BER.decode_object_identifier(_el)?),
            "contextValues" => {
                contextValues_ = Some(|el: &X690Element| -> ASN1Result<SET_OF<X690Element>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "contextValues",
                            ))
                        }
                    };
                    let mut items: SET_OF<X690Element> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(x690_identity(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            "fallback" => fallback_ = Some(BER.decode_boolean(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Context {
        contextType: contextType_.unwrap(),
        contextValues: contextValues_.unwrap(),
        fallback: fallback_,
        _unrecognized,
    })
}

pub fn _encode_Context(value_: &Context) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_object_identifier(&value_.contextType)?);
    components_.push(|value_: &SET_OF<X690Element>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(x690_identity(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }(&value_.contextValues)?);
    if let Some(v_) = &value_.fallback {
        if *v_ != Context::_default_value_for_fallback() {
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

pub fn _validate_Context(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Context")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Context,
        _eal_components_for_Context,
        _rctl2_components_for_Context,
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
            "contextValues" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            BER.validate_any(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contextValues")
                    ),
                }
            }(_el)?,
            "fallback" => BER.validate_boolean(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeValueAssertion ::= SEQUENCE {
///   type              ATTRIBUTE.&id({SupportedAttributes}),
///   assertion         ATTRIBUTE.&equality-match.&AssertionType
///                       ({SupportedAttributes}{@type}),
///   assertedContexts  CHOICE {
///     allContexts       [0]  NULL,
///     selectedContexts  [1]  SET SIZE (1..MAX) OF ContextAssertion } OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct AttributeValueAssertion {
    pub type_: OBJECT_IDENTIFIER,
    pub assertion: X690Element,
    pub assertedContexts: OPTIONAL<AttributeValueAssertion_assertedContexts>,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeValueAssertion {
    pub fn new(
        type_: OBJECT_IDENTIFIER,
        assertion: X690Element,
        assertedContexts: OPTIONAL<AttributeValueAssertion_assertedContexts>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeValueAssertion {
            type_,
            assertion,
            assertedContexts,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for AttributeValueAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeValueAssertion(el)
    }
}

pub const _rctl1_components_for_AttributeValueAssertion: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("assertion", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "assertedContexts",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
        ]),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeValueAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeValueAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeValueAssertion(el: &X690Element) -> ASN1Result<AttributeValueAssertion> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeValueAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeValueAssertion,
        _eal_components_for_AttributeValueAssertion,
        _rctl2_components_for_AttributeValueAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut type__: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut assertion_: OPTIONAL<X690Element> = None;
    let mut assertedContexts_: OPTIONAL<AttributeValueAssertion_assertedContexts> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => type__ = Some(BER.decode_object_identifier(_el)?),
            "assertion" => assertion_ = Some(x690_identity(_el)?),
            "assertedContexts" => {
                assertedContexts_ = Some(_decode_AttributeValueAssertion_assertedContexts(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttributeValueAssertion {
        type_: type__.unwrap(),
        assertion: assertion_.unwrap(),
        assertedContexts: assertedContexts_,
        _unrecognized,
    })
}

pub fn _encode_AttributeValueAssertion(
    value_: &AttributeValueAssertion,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_object_identifier(&value_.type_)?);
    components_.push(x690_identity(&value_.assertion)?);
    if let Some(v_) = &value_.assertedContexts {
        components_.push(_encode_AttributeValueAssertion_assertedContexts(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AttributeValueAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeValueAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeValueAssertion,
        _eal_components_for_AttributeValueAssertion,
        _rctl2_components_for_AttributeValueAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => BER.validate_object_identifier(_el)?,
            "assertion" => BER.validate_any(_el)?,
            "assertedContexts" => _validate_AttributeValueAssertion_assertedContexts(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContextAssertion ::= SEQUENCE {
///   contextType    CONTEXT.&id({SupportedContexts}),
///   contextValues  SET SIZE (1..MAX) OF
///       CONTEXT.&Assertion({SupportedContexts}{@contextType}),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct ContextAssertion {
    pub contextType: OBJECT_IDENTIFIER,
    pub contextValues: Vec<X690Element>,
    pub _unrecognized: Vec<X690Element>,
}
impl ContextAssertion {
    pub fn new(
        contextType: OBJECT_IDENTIFIER,
        contextValues: Vec<X690Element>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ContextAssertion {
            contextType,
            contextValues,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for ContextAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ContextAssertion(el)
    }
}

pub const _rctl1_components_for_ContextAssertion: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "contextType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "contextValues",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ContextAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ContextAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_ContextAssertion(el: &X690Element) -> ASN1Result<ContextAssertion> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ContextAssertion")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ContextAssertion,
        _eal_components_for_ContextAssertion,
        _rctl2_components_for_ContextAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut contextType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut contextValues_: OPTIONAL<Vec<X690Element>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "contextType" => contextType_ = Some(BER.decode_object_identifier(_el)?),
            "contextValues" => {
                contextValues_ = Some(|el: &X690Element| -> ASN1Result<SET_OF<X690Element>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "contextValues",
                            ))
                        }
                    };
                    let mut items: SET_OF<X690Element> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(x690_identity(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ContextAssertion {
        contextType: contextType_.unwrap(),
        contextValues: contextValues_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_ContextAssertion(value_: &ContextAssertion) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.contextType)?);
    components_.push(|value_: &SET_OF<X690Element>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(x690_identity(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }(&value_.contextValues)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_ContextAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ContextAssertion")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ContextAssertion,
        _eal_components_for_ContextAssertion,
        _rctl2_components_for_ContextAssertion,
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
            "contextValues" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            BER.validate_any(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contextValues")
                    ),
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
/// AttributeTypeAssertion ::= SEQUENCE {
///   type              ATTRIBUTE.&id({SupportedAttributes}),
///   assertedContexts  SEQUENCE SIZE (1..MAX) OF ContextAssertion OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct AttributeTypeAssertion {
    pub type_: OBJECT_IDENTIFIER,
    pub assertedContexts: OPTIONAL<Vec<ContextAssertion>>,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeTypeAssertion {
    pub fn new(
        type_: OBJECT_IDENTIFIER,
        assertedContexts: OPTIONAL<Vec<ContextAssertion>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeTypeAssertion {
            type_,
            assertedContexts,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for AttributeTypeAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeTypeAssertion(el)
    }
}

pub const _rctl1_components_for_AttributeTypeAssertion: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "assertedContexts",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeTypeAssertion: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeTypeAssertion: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeTypeAssertion(el: &X690Element) -> ASN1Result<AttributeTypeAssertion> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeTypeAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeTypeAssertion,
        _eal_components_for_AttributeTypeAssertion,
        _rctl2_components_for_AttributeTypeAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut type__: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut assertedContexts_: OPTIONAL<Vec<ContextAssertion>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => type__ = Some(BER.decode_object_identifier(_el)?),
            "assertedContexts" => {
                assertedContexts_ = Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<ContextAssertion>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "assertedContexts",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<ContextAssertion> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_ContextAssertion(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttributeTypeAssertion {
        type_: type__.unwrap(),
        assertedContexts: assertedContexts_,
        _unrecognized,
    })
}

pub fn _encode_AttributeTypeAssertion(value_: &AttributeTypeAssertion) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.type_)?);
    if let Some(v_) = &value_.assertedContexts {
        components_.push(
            |value_: &SEQUENCE_OF<ContextAssertion>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_ContextAssertion(&v)?);
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

pub fn _validate_AttributeTypeAssertion(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "AttributeTypeAssertion",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeTypeAssertion,
        _eal_components_for_AttributeTypeAssertion,
        _rctl2_components_for_AttributeTypeAssertion,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => BER.validate_object_identifier(_el)?,
            "assertedContexts" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_ContextAssertion(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "assertedContexts",
                    )),
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
/// SupportedAttributes ATTRIBUTE ::= {objectClass | aliasedEntryName, ...}
/// ```
///
///
pub fn SupportedAttributes() -> Vec<ATTRIBUTE> {
    Vec::from([objectClass(), aliasedEntryName()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedContexts CONTEXT ::= {...}
/// ```
///
///
pub fn SupportedContexts() -> Vec<CONTEXT> {
    Vec::new()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RDNSequence  ::=  SEQUENCE OF RelativeDistinguishedName
/// ```
pub type RDNSequence = Vec<RelativeDistinguishedName>; // SequenceOfType

pub fn _decode_RDNSequence(el: &X690Element) -> ASN1Result<RDNSequence> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RDNSequence")),
    };
    let mut items: SEQUENCE_OF<RelativeDistinguishedName> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_RelativeDistinguishedName(el)?);
    }
    Ok(items)
}

pub fn _encode_RDNSequence(value_: &RDNSequence) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_RelativeDistinguishedName(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_RDNSequence(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_RelativeDistinguishedName(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "RDNSequence")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DistinguishedName  ::=  RDNSequence
/// ```
pub type DistinguishedName = RDNSequence; // DefinedType

pub fn _decode_DistinguishedName(el: &X690Element) -> ASN1Result<DistinguishedName> {
    _decode_RDNSequence(&el)
}

pub fn _encode_DistinguishedName(value_: &DistinguishedName) -> ASN1Result<X690Element> {
    _encode_RDNSequence(&value_)
}

pub fn _validate_DistinguishedName(el: &X690Element) -> ASN1Result<()> {
    _validate_RDNSequence(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// RelativeDistinguishedName  ::=  SET SIZE (1..MAX) OF AttributeTypeAndValue
/// ```
pub type RelativeDistinguishedName = Vec<AttributeTypeAndValue>; // SetOfType

pub fn _decode_RelativeDistinguishedName(
    el: &X690Element,
) -> ASN1Result<RelativeDistinguishedName> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "RelativeDistinguishedName",
            ))
        }
    };
    let mut items: SET_OF<AttributeTypeAndValue> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AttributeTypeAndValue(el)?);
    }
    Ok(items)
}

pub fn _encode_RelativeDistinguishedName(
    value_: &RelativeDistinguishedName,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AttributeTypeAndValue(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_RelativeDistinguishedName(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AttributeTypeAndValue(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "RelativeDistinguishedName",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeTypeAndValue ::= SEQUENCE {
///   type                  ATTRIBUTE.&id({SupportedAttributes}),
///   value                 ATTRIBUTE.&Type({SupportedAttributes}{@type}),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct AttributeTypeAndValue {
    pub type_: OBJECT_IDENTIFIER,
    pub value: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeTypeAndValue {
    pub fn new(
        type_: OBJECT_IDENTIFIER,
        value: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeTypeAndValue {
            type_,
            value,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for AttributeTypeAndValue {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeTypeAndValue(el)
    }
}

pub const _rctl1_components_for_AttributeTypeAndValue: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("value", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_AttributeTypeAndValue: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeTypeAndValue: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeTypeAndValue(el: &X690Element) -> ASN1Result<AttributeTypeAndValue> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeTypeAndValue")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeTypeAndValue,
        _eal_components_for_AttributeTypeAndValue,
        _rctl2_components_for_AttributeTypeAndValue,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut type__: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut value_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => type__ = Some(BER.decode_object_identifier(_el)?),
            "value" => value_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(AttributeTypeAndValue {
        type_: type__.unwrap(),
        value: value_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_AttributeTypeAndValue(value_: &AttributeTypeAndValue) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.type_)?);
    components_.push(x690_identity(&value_.value)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_AttributeTypeAndValue(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeTypeAndValue")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AttributeTypeAndValue,
        _eal_components_for_AttributeTypeAndValue,
        _rctl2_components_for_AttributeTypeAndValue,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => BER.validate_object_identifier(_el)?,
            "value" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubtreeSpecification ::= SEQUENCE {
///   base                 [0]  LocalName DEFAULT {},
///   COMPONENTS OF             ChopSpecification,
///   specificationFilter  [4]  Refinement OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct SubtreeSpecification {
    pub base: OPTIONAL<LocalName>,
    pub specificExclusions: OPTIONAL<Vec<ChopSpecification_specificExclusions_Item>>, /* REPLICATED_COMPONENT */
    pub minimum: OPTIONAL<BaseDistance>, /* REPLICATED_COMPONENT */
    pub maximum: OPTIONAL<BaseDistance>, /* REPLICATED_COMPONENT */
    pub specificationFilter: OPTIONAL<Refinement>,
    pub _unrecognized: Vec<X690Element>,
}
impl SubtreeSpecification {
    pub fn new(
        base: OPTIONAL<LocalName>,
        specificExclusions: OPTIONAL<Vec<ChopSpecification_specificExclusions_Item>>, /* REPLICATED_COMPONENT */
        minimum: OPTIONAL<BaseDistance>, /* REPLICATED_COMPONENT */
        maximum: OPTIONAL<BaseDistance>, /* REPLICATED_COMPONENT */
        specificationFilter: OPTIONAL<Refinement>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SubtreeSpecification {
            base,
            specificExclusions,
            minimum,
            maximum,
            specificationFilter,
            _unrecognized,
        }
    }
    pub fn _default_value_for_base() -> LocalName {
        Vec::from([])
    }
    pub fn _default_value_for_minimum() -> BaseDistance {
        Vec::from([0]) // TODO: Make this an fixed int.
    }
}
impl Default for SubtreeSpecification {
    fn default() -> Self {
        SubtreeSpecification {
            base: None,
            specificExclusions: None,
            minimum: None,
            maximum: None,
            specificationFilter: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<&X690Element> for SubtreeSpecification {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SubtreeSpecification(el)
    }
}

pub const _rctl1_components_for_SubtreeSpecification: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "base",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "specificExclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "minimum",
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
        "specificationFilter",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SubtreeSpecification: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SubtreeSpecification: &[ComponentSpec; 0] = &[];

pub fn _decode_SubtreeSpecification(el: &X690Element) -> ASN1Result<SubtreeSpecification> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubtreeSpecification")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SubtreeSpecification,
        _eal_components_for_SubtreeSpecification,
        _rctl2_components_for_SubtreeSpecification,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut base_: OPTIONAL<LocalName> = None;
    let mut specificExclusions_: OPTIONAL<Vec<ChopSpecification_specificExclusions_Item>> = None;
    let mut minimum_: OPTIONAL<BaseDistance> = None;
    let mut maximum_: OPTIONAL<BaseDistance> = None;
    let mut specificationFilter_: OPTIONAL<Refinement> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "base" => {
                base_ = Some(|el: &X690Element| -> ASN1Result<LocalName> {
                    Ok(_decode_LocalName(&el.inner()?)?)
                }(_el)?)
            }
            "specificExclusions" => {
                specificExclusions_ = Some(|el: &X690Element| -> ASN1Result<
                    Vec<ChopSpecification_specificExclusions_Item>,
                > {
                    Ok(|el: &X690Element| -> ASN1Result<
                        SET_OF<ChopSpecification_specificExclusions_Item>,
                    > {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "specificExclusions",
                                ))
                            }
                        };
                        let mut items: SET_OF<ChopSpecification_specificExclusions_Item> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_ChopSpecification_specificExclusions_Item(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "minimum" => {
                minimum_ = Some(|el: &X690Element| -> ASN1Result<BaseDistance> {
                    Ok(_decode_BaseDistance(&el.inner()?)?)
                }(_el)?)
            }
            "maximum" => {
                maximum_ = Some(|el: &X690Element| -> ASN1Result<BaseDistance> {
                    Ok(_decode_BaseDistance(&el.inner()?)?)
                }(_el)?)
            }
            "specificationFilter" => {
                specificationFilter_ = Some(|el: &X690Element| -> ASN1Result<Refinement> {
                    Ok(_decode_Refinement(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(SubtreeSpecification {
        base: base_,
        specificExclusions: specificExclusions_,
        minimum: minimum_,
        maximum: maximum_,
        specificationFilter: specificationFilter_,
        _unrecognized,
    })
}

pub fn _encode_SubtreeSpecification(value_: &SubtreeSpecification) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    if let Some(v_) = &value_.base {
        if v_.len() > 0 {
            // Manually edited.
            components_.push(|v_1: &LocalName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(_encode_LocalName(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.specificExclusions {
        components_.push(
            |v_1: &Vec<ChopSpecification_specificExclusions_Item>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(|value_: &SET_OF<
                        ChopSpecification_specificExclusions_Item,
                    >|
                     -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ChopSpecification_specificExclusions_Item(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?),
                ))
            }(&v_)?,
        );
    }
    if let Some(v_) = &value_.minimum {
        if *v_ != SubtreeSpecification::_default_value_for_minimum() {
            components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(_encode_BaseDistance(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.maximum {
        components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(_encode_BaseDistance(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.specificationFilter {
        components_.push(|v_1: &Refinement| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::from_explicit(_encode_Refinement(&v_1)?),
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

pub fn _validate_SubtreeSpecification(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubtreeSpecification")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SubtreeSpecification,
        _eal_components_for_SubtreeSpecification,
        _rctl2_components_for_SubtreeSpecification,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "base" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "base"));
                }
                Ok(_validate_LocalName(&el.inner()?)?)
            }(_el)?,
            "specificExclusions" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "specificExclusions",
                    ));
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_ChopSpecification_specificExclusions_Item(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "specificExclusions",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "minimum" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "minimum")
                    );
                }
                Ok(_validate_BaseDistance(&el.inner()?)?)
            }(_el)?,
            "maximum" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "maximum")
                    );
                }
                Ok(_validate_BaseDistance(&el.inner()?)?)
            }(_el)?,
            "specificationFilter" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "specificationFilter",
                    ));
                }
                Ok(_validate_Refinement(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LocalName  ::=  RDNSequence
/// ```
pub type LocalName = RDNSequence; // DefinedType

pub fn _decode_LocalName(el: &X690Element) -> ASN1Result<LocalName> {
    _decode_RDNSequence(&el)
}

pub fn _encode_LocalName(value_: &LocalName) -> ASN1Result<X690Element> {
    _encode_RDNSequence(&value_)
}

pub fn _validate_LocalName(el: &X690Element) -> ASN1Result<()> {
    _validate_RDNSequence(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ChopSpecification-specificExclusions-Item  ::=  CHOICE {
///     chopBefore  [0]  LocalName,
///     chopAfter   [1]  LocalName,
///     ...}
/// ```
#[derive(Debug, Clone)]
pub enum ChopSpecification_specificExclusions_Item {
    chopBefore(LocalName),
    chopAfter(LocalName),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for ChopSpecification_specificExclusions_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ChopSpecification_specificExclusions_Item(el)
    }
}

pub fn _decode_ChopSpecification_specificExclusions_Item(
    el: &X690Element,
) -> ASN1Result<ChopSpecification_specificExclusions_Item> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(ChopSpecification_specificExclusions_Item::chopBefore(
            |el: &X690Element| -> ASN1Result<LocalName> { Ok(_decode_LocalName(&el.inner()?)?) }(
                &el,
            )?,
        )),
        (TagClass::CONTEXT, 1) => Ok(ChopSpecification_specificExclusions_Item::chopAfter(
            |el: &X690Element| -> ASN1Result<LocalName> { Ok(_decode_LocalName(&el.inner()?)?) }(
                &el,
            )?,
        )),
        _ => Ok(ChopSpecification_specificExclusions_Item::_unrecognized(
            el.clone(),
        )),
    }
}

pub fn _encode_ChopSpecification_specificExclusions_Item(
    value_: &ChopSpecification_specificExclusions_Item,
) -> ASN1Result<X690Element> {
    match value_ {
        ChopSpecification_specificExclusions_Item::chopBefore(v) => {
            |v_1: &LocalName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(_encode_LocalName(&v_1)?),
                ))
            }(&v)
        }
        ChopSpecification_specificExclusions_Item::chopAfter(v) => {
            |v_1: &LocalName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(_encode_LocalName(&v_1)?),
                ))
            }(&v)
        }
        ChopSpecification_specificExclusions_Item::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_ChopSpecification_specificExclusions_Item(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "chopBefore"));
            }
            Ok(_validate_LocalName(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "chopAfter"));
            }
            Ok(_validate_LocalName(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ChopSpecification ::= SEQUENCE {
///   specificExclusions    [1]  SET SIZE (1..MAX) OF ChopSpecification-specificExclusions-Item OPTIONAL,
///   minimum       [2]  BaseDistance DEFAULT 0,
///   maximum       [3]  BaseDistance OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct ChopSpecification {
    pub specificExclusions: OPTIONAL<Vec<ChopSpecification_specificExclusions_Item>>,
    pub minimum: OPTIONAL<BaseDistance>,
    pub maximum: OPTIONAL<BaseDistance>,
    pub _unrecognized: Vec<X690Element>,
}
impl ChopSpecification {
    pub fn new(
        specificExclusions: OPTIONAL<Vec<ChopSpecification_specificExclusions_Item>>,
        minimum: OPTIONAL<BaseDistance>,
        maximum: OPTIONAL<BaseDistance>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ChopSpecification {
            specificExclusions,
            minimum,
            maximum,
            _unrecognized,
        }
    }
    pub fn _default_value_for_minimum() -> BaseDistance {
        Vec::from([0]) // TODO: Make fixed int
    }
}
impl Default for ChopSpecification {
    fn default() -> Self {
        ChopSpecification {
            specificExclusions: None,
            minimum: None,
            maximum: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<&X690Element> for ChopSpecification {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ChopSpecification(el)
    }
}

pub const _rctl1_components_for_ChopSpecification: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "specificExclusions",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "minimum",
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
];

pub const _rctl2_components_for_ChopSpecification: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ChopSpecification: &[ComponentSpec; 0] = &[];

pub fn _decode_ChopSpecification(el: &X690Element) -> ASN1Result<ChopSpecification> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ChopSpecification")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ChopSpecification,
        _eal_components_for_ChopSpecification,
        _rctl2_components_for_ChopSpecification,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut specificExclusions_: OPTIONAL<Vec<ChopSpecification_specificExclusions_Item>> = None;
    let mut minimum_: OPTIONAL<BaseDistance> = None;
    let mut maximum_: OPTIONAL<BaseDistance> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "specificExclusions" => {
                specificExclusions_ = Some(|el: &X690Element| -> ASN1Result<
                    Vec<ChopSpecification_specificExclusions_Item>,
                > {
                    Ok(|el: &X690Element| -> ASN1Result<
                        SET_OF<ChopSpecification_specificExclusions_Item>,
                    > {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "specificExclusions",
                                ))
                            }
                        };
                        let mut items: SET_OF<ChopSpecification_specificExclusions_Item> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_ChopSpecification_specificExclusions_Item(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "minimum" => {
                minimum_ = Some(|el: &X690Element| -> ASN1Result<BaseDistance> {
                    Ok(_decode_BaseDistance(&el.inner()?)?)
                }(_el)?)
            }
            "maximum" => {
                maximum_ = Some(|el: &X690Element| -> ASN1Result<BaseDistance> {
                    Ok(_decode_BaseDistance(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(ChopSpecification {
        specificExclusions: specificExclusions_,
        minimum: minimum_,
        maximum: maximum_,
        _unrecognized,
    })
}

pub fn _encode_ChopSpecification(value_: &ChopSpecification) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    if let Some(v_) = &value_.specificExclusions {
        components_.push(
            |v_1: &Vec<ChopSpecification_specificExclusions_Item>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(|value_: &SET_OF<
                        ChopSpecification_specificExclusions_Item,
                    >|
                     -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_ChopSpecification_specificExclusions_Item(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?),
                ))
            }(&v_)?,
        );
    }
    if let Some(v_) = &value_.minimum {
        if *v_ != ChopSpecification::_default_value_for_minimum() {
            components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(_encode_BaseDistance(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.maximum {
        components_.push(|v_1: &BaseDistance| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(_encode_BaseDistance(&v_1)?),
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

pub fn _validate_ChopSpecification(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ChopSpecification")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ChopSpecification,
        _eal_components_for_ChopSpecification,
        _rctl2_components_for_ChopSpecification,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "specificExclusions" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "specificExclusions",
                    ));
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_ChopSpecification_specificExclusions_Item(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(
                            ASN1ErrorCode::invalid_construction,
                            "specificExclusions",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "minimum" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "minimum")
                    );
                }
                Ok(_validate_BaseDistance(&el.inner()?)?)
            }(_el)?,
            "maximum" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "maximum")
                    );
                }
                Ok(_validate_BaseDistance(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BaseDistance  ::=  INTEGER(0..MAX)
/// ```
pub type BaseDistance = INTEGER;

pub fn _decode_BaseDistance(el: &X690Element) -> ASN1Result<BaseDistance> {
    BER.decode_integer(&el)
}

pub fn _encode_BaseDistance(value_: &BaseDistance) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_BaseDistance(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Refinement  ::=  CHOICE {
///   item  [0]  OBJECT-CLASS.&id,
///   and   [1]  SET SIZE (1..MAX) OF Refinement,
///   or    [2]  SET SIZE (1..MAX) OF Refinement,
///   not   [3]  Refinement,
///   ... }
/// ```
// TODO: CHECK_RECURSIVE_DEFINITION
#[derive(Debug, Clone)]
pub enum Refinement {
    item(OBJECT_IDENTIFIER),
    and(Vec<Refinement>),
    or(Vec<Refinement>),
    not(Box<Refinement>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for Refinement {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Refinement(el)
    }
}

pub fn _decode_Refinement(el: &X690Element) -> ASN1Result<Refinement> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(Refinement::item(
            |el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(BER.decode_object_identifier(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(Refinement::and(
            |el: &X690Element| -> ASN1Result<Vec<Refinement>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<Refinement>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and")
                            )
                        }
                    };
                    let mut items: SET_OF<Refinement> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_Refinement(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(Refinement::or(
            |el: &X690Element| -> ASN1Result<Vec<Refinement>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<Refinement>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or")
                            )
                        }
                    };
                    let mut items: SET_OF<Refinement> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_Refinement(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 3) => Ok(Refinement::not(Box::new(
            |el: &X690Element| -> ASN1Result<Refinement> { Ok(_decode_Refinement(&el.inner()?)?) }(
                &el,
            )?,
        ))),
        _ => Ok(Refinement::_unrecognized(el.clone())),
    }
}

pub fn _encode_Refinement(value_: &Refinement) -> ASN1Result<X690Element> {
    match value_ {
        Refinement::item(v) => |v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(BER.encode_object_identifier(&v_1)?),
            ))
        }(&v),
        Refinement::and(v) => |v_1: &Vec<Refinement>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(
                    &|value_: &SET_OF<Refinement>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_Refinement(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v),
        Refinement::or(v) => |v_1: &Vec<Refinement>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(
                    &|value_: &SET_OF<Refinement>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_Refinement(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v),
        Refinement::not(v) => |v_1: &Refinement| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(_encode_Refinement(&v_1)?),
            ))
        }(&v),
        Refinement::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_Refinement(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "item"));
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
                            _validate_Refinement(&sub)?;
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
                            _validate_Refinement(&sub)?;
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
            Ok(_validate_Refinement(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OBJECT-CLASS ::= CLASS {
///   &Superclasses         OBJECT-CLASS OPTIONAL,
///   &kind                 ObjectClassKind DEFAULT structural,
///   &MandatoryAttributes  ATTRIBUTE OPTIONAL,
///   &OptionalAttributes   ATTRIBUTE OPTIONAL,
///   &ldapName             SEQUENCE SIZE(1..MAX) OF UTF8String OPTIONAL,
///   &ldapDesc             UTF8String OPTIONAL,
///   &id                   OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   [SUBCLASS OF          &Superclasses]
///   [KIND                 &kind]
///   [MUST CONTAIN         &MandatoryAttributes]
///   [MAY CONTAIN          &OptionalAttributes]
///   [LDAP-NAME            &ldapName]
///   [LDAP-DESC            &ldapDesc]
///   ID                    &id }
/// ```
///
#[derive(Debug)]
pub struct OBJECT_CLASS {
    pub Superclasses: OPTIONAL<Vec<OBJECT_CLASS>>,
    pub kind: OPTIONAL<ObjectClassKind>,
    pub MandatoryAttributes: OPTIONAL<Vec<ATTRIBUTE>>,
    pub OptionalAttributes: OPTIONAL<Vec<ATTRIBUTE>>,
    pub ldapName: OPTIONAL<Vec<UTF8String>>,
    pub ldapDesc: OPTIONAL<UTF8String>,
    pub id: OBJECT_IDENTIFIER,
}
impl OBJECT_CLASS {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ObjectClassKind  ::=  ENUMERATED {
///   abstract   (0),
///   structural (1),
///   auxiliary  (2)}
/// ```
pub type ObjectClassKind = ENUMERATED;

pub const ObjectClassKind_abstract_: ObjectClassKind = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ObjectClassKind_structural: ObjectClassKind = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ObjectClassKind_auxiliary: ObjectClassKind = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_ObjectClassKind(el: &X690Element) -> ASN1Result<ObjectClassKind> {
    BER.decode_enumerated(&el)
}

pub fn _encode_ObjectClassKind(value_: &ObjectClassKind) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_ObjectClassKind(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// top OBJECT-CLASS ::= {
///   KIND          abstract
///   MUST CONTAIN  {objectClass}
///   LDAP-NAME     {"top"}
///   ID            id-oc-top }
/// ```
///
///
pub fn top() -> OBJECT_CLASS {
    OBJECT_CLASS {
        kind: Some(ObjectClassKind_abstract_), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([objectClass()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("top")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_top(),                       /* OBJECT_FIELD_SETTING */
        Superclasses: None,
        OptionalAttributes: None,
        ldapDesc: None,
    }
}

pub mod top {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// alias OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {aliasedEntryName}
///   LDAP-NAME     {"alias"}
///   ID            id-oc-alias }
/// ```
///
///
pub fn alias() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([aliasedEntryName()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("alias")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_alias(),                      /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapDesc: None,
    }
}

pub mod alias {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// parent OBJECT-CLASS ::= {
///   KIND          abstract
///   ID            id-oc-parent }
/// ```
///
///
pub fn parent() -> OBJECT_CLASS {
    OBJECT_CLASS {
        kind: Some(ObjectClassKind_abstract_), /* OBJECT_FIELD_SETTING */
        id: id_oc_parent(),                    /* OBJECT_FIELD_SETTING */
        Superclasses: None,
        MandatoryAttributes: None,
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod parent {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// child OBJECT-CLASS ::= {
///   KIND          auxiliary
///   ID            id-oc-child }
/// ```
///
///
pub fn child() -> OBJECT_CLASS {
    OBJECT_CLASS {
        kind: Some(ObjectClassKind_auxiliary), /* OBJECT_FIELD_SETTING */
        id: id_oc_child(),                     /* OBJECT_FIELD_SETTING */
        Superclasses: None,
        MandatoryAttributes: None,
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod child {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ATTRIBUTE ::= CLASS {
///   &derivation               ATTRIBUTE OPTIONAL,
///   &Type                     OPTIONAL, -- either &Type or &derivation required
///   &equality-match           MATCHING-RULE OPTIONAL,
///   &ordering-match           MATCHING-RULE OPTIONAL,
///   &substrings-match         MATCHING-RULE OPTIONAL,
///   &single-valued            BOOLEAN DEFAULT FALSE,
///   &collective               BOOLEAN DEFAULT FALSE,
///   &dummy                    BOOLEAN DEFAULT FALSE,
///   -- operational extensions
///   &no-user-modification     BOOLEAN DEFAULT FALSE,
///   &usage                    AttributeUsage DEFAULT userApplications,
///   &ldapSyntax               SYNTAX-NAME.&id OPTIONAL,
///   &ldapName                 SEQUENCE SIZE(1..MAX) OF UTF8String OPTIONAL,
///   &ldapDesc                 UTF8String OPTIONAL,
///   &obsolete                 BOOLEAN DEFAULT FALSE,
///   &id                       OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   [SUBTYPE OF               &derivation]
///   [WITH SYNTAX              &Type]
///   [EQUALITY MATCHING RULE   &equality-match]
///   [ORDERING MATCHING RULE   &ordering-match]
///   [SUBSTRINGS MATCHING RULE &substrings-match]
///   [SINGLE VALUE             &single-valued]
///   [COLLECTIVE               &collective]
///   [DUMMY                    &dummy]
///   [NO USER MODIFICATION     &no-user-modification]
///   [USAGE                    &usage]
///   [LDAP-SYNTAX              &ldapSyntax]
///   [LDAP-NAME                &ldapName]
///   [LDAP-DESC                &ldapDesc]
///   [OBSOLETE                 &obsolete]
///   ID                        &id }
/// ```
///
#[derive(Debug, Clone)]
pub struct ATTRIBUTE {
    pub derivation: OPTIONAL<Box<ATTRIBUTE>>,
    pub equality_match: OPTIONAL<Box<MATCHING_RULE>>,
    pub ordering_match: OPTIONAL<Box<MATCHING_RULE>>,
    pub substrings_match: OPTIONAL<Box<MATCHING_RULE>>,
    pub single_valued: OPTIONAL<BOOLEAN>,
    pub collective: OPTIONAL<BOOLEAN>,
    pub dummy: OPTIONAL<BOOLEAN>,
    pub no_user_modification: OPTIONAL<BOOLEAN>,
    pub usage: OPTIONAL<AttributeUsage>,
    pub ldapSyntax: OPTIONAL<OBJECT_IDENTIFIER>,
    pub ldapName: OPTIONAL<Vec<UTF8String>>,
    pub ldapDesc: OPTIONAL<UTF8String>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub id: OBJECT_IDENTIFIER,
}
impl ATTRIBUTE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeUsage  ::=  ENUMERATED {
///   userApplications     (0),
///   directoryOperation   (1),
///   distributedOperation (2),
///   dSAOperation         (3),
///   ... }
/// ```
pub type AttributeUsage = ENUMERATED;

pub const AttributeUsage_userApplications: AttributeUsage = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AttributeUsage_directoryOperation: AttributeUsage = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AttributeUsage_distributedOperation: AttributeUsage = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const AttributeUsage_dSAOperation: AttributeUsage = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_AttributeUsage(el: &X690Element) -> ASN1Result<AttributeUsage> {
    BER.decode_enumerated(&el)
}

pub fn _encode_AttributeUsage(value_: &AttributeUsage) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_AttributeUsage(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// objectClass ATTRIBUTE ::= {
///   WITH SYNTAX             OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE  objectIdentifierMatch
///   LDAP-SYNTAX             oid.&id
///   LDAP-NAME               {"objectClass"}
///   ID                      id-at-objectClass }
/// ```
///
///
pub fn objectClass() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),                              /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("objectClass")])), /* OBJECT_FIELD_SETTING */
        id: id_at_objectClass(),                                 /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod objectClass {
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
/// aliasedEntryName ATTRIBUTE ::= {
///   WITH SYNTAX             DistinguishedName
///   EQUALITY MATCHING RULE  distinguishedNameMatch
///   SINGLE VALUE            TRUE
///   LDAP-SYNTAX             dn.&id
///   LDAP-NAME               {"aliasedObjectName"}
///   ID                      id-at-aliasedEntryName }
/// ```
///
///
pub fn aliasedEntryName() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                                /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dn().id),                                /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("aliasedObjectName")])), /* OBJECT_FIELD_SETTING */
        id: id_at_aliasedEntryName(),                             /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod aliasedEntryName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MATCHING-RULE ::= CLASS {
///   &ParentMatchingRules    MATCHING-RULE OPTIONAL,
///   &AssertionType          OPTIONAL,
///   &uniqueMatchIndicator   ATTRIBUTE OPTIONAL,
///   &ldapSyntax             SYNTAX-NAME.&id OPTIONAL,
///   &ldapName               SEQUENCE SIZE(1..MAX) OF UTF8String OPTIONAL,
///   &ldapDesc               UTF8String OPTIONAL,
///   &id                     OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   [PARENT                 &ParentMatchingRules]
///   [SYNTAX                 &AssertionType]
///   [UNIQUE-MATCH-INDICATOR &uniqueMatchIndicator]
///   [LDAP-SYNTAX            &ldapSyntax]
///   [LDAP-NAME              &ldapName]
///   [LDAP-DESC              &ldapDesc]
///   ID                      &id }
/// ```
///
#[derive(Debug, Clone)]
pub struct MATCHING_RULE {
    pub ParentMatchingRules: OPTIONAL<Vec<MATCHING_RULE>>,
    pub uniqueMatchIndicator: OPTIONAL<ATTRIBUTE>,
    pub ldapSyntax: OPTIONAL<OBJECT_IDENTIFIER>,
    pub ldapName: OPTIONAL<Vec<UTF8String>>,
    pub ldapDesc: OPTIONAL<UTF8String>,
    pub id: OBJECT_IDENTIFIER,
}
impl MATCHING_RULE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// objectIdentifierMatch MATCHING-RULE ::= {
///   SYNTAX       OBJECT IDENTIFIER
///   LDAP-SYNTAX  oid.&id
///   LDAP-NAME    {"objectIdentifierMatch"}
///   ID           id-mr-objectIdentifierMatch }
/// ```
///
///
pub fn objectIdentifierMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(oid().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("objectIdentifierMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_objectIdentifierMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod objectIdentifierMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = OBJECT_IDENTIFIER; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_object_identifier(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_object_identifier(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_object_identifier(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// distinguishedNameMatch MATCHING-RULE ::= {
///   SYNTAX       DistinguishedName
///   LDAP-SYNTAX  dn.&id
///   LDAP-NAME    {"distinguishedNameMatch"}
///   ID           id-mr-distinguishedNameMatch }
/// ```
///
///
pub fn distinguishedNameMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(dn().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("distinguishedNameMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_distinguishedNameMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod distinguishedNameMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MAPPING-BASED-MATCHING
///   {SelectedBy, BOOLEAN:combinable, MappingResult, OBJECT IDENTIFIER:matchingRule} ::= CLASS {
///   &selectBy             SelectedBy OPTIONAL,
///   &ApplicableTo         ATTRIBUTE,
///   &subtypesIncluded     BOOLEAN DEFAULT TRUE,
///   &combinable           BOOLEAN(combinable),
///   &mappingResults       MappingResult OPTIONAL,
///   &userControl          BOOLEAN DEFAULT FALSE,
///   &exclusive            BOOLEAN DEFAULT TRUE,
///   &matching-rule        MATCHING-RULE.&id(matchingRule),
///   &id                   OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   [SELECT BY            &selectBy]
///   APPLICABLE TO         &ApplicableTo
///   [SUBTYPES INCLUDED    &subtypesIncluded]
///   COMBINABLE            &combinable
///   [MAPPING RESULTS      &mappingResults]
///   [USER CONTROL         &userControl]
///   [EXCLUSIVE            &exclusive]
///   MATCHING RULE         &matching-rule
///   ID                    &id }
/// ```
///
#[derive(Debug)]
pub struct MAPPING_BASED_MATCHING<
    SelectedBy,    /* OBJECT_CLASS_ASSIGNMENT_PARAMETER */
    MappingResult, /* OBJECT_CLASS_ASSIGNMENT_PARAMETER */
> {
    pub selectBy: OPTIONAL<SelectedBy>,
    pub ApplicableTo: Vec<ATTRIBUTE>,
    pub subtypesIncluded: OPTIONAL<BOOLEAN>,
    pub combinable: BOOLEAN,
    pub mappingResults: OPTIONAL<MappingResult>,
    pub userControl: OPTIONAL<BOOLEAN>,
    pub exclusive: OPTIONAL<BOOLEAN>,
    pub matching_rule: OBJECT_IDENTIFIER,
    pub id: OBJECT_IDENTIFIER,
}
impl<SelectedBy, MappingResult>
    MAPPING_BASED_MATCHING<
        SelectedBy,    /* OBJECT_CLASS_ASSIGNMENT_PARAMETER */
        MappingResult, /* OBJECT_CLASS_ASSIGNMENT_PARAMETER */
    >
{
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NAME-FORM ::= CLASS {
///   &namedObjectClass     OBJECT-CLASS,
///   &MandatoryAttributes  ATTRIBUTE,
///   &OptionalAttributes   ATTRIBUTE OPTIONAL,
///   &ldapName             SEQUENCE SIZE(1..MAX) OF UTF8String OPTIONAL,
///   &ldapDesc             UTF8String OPTIONAL,
///   &id                   OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   NAMES                 &namedObjectClass
///   WITH ATTRIBUTES       &MandatoryAttributes
///   [AND OPTIONALLY       &OptionalAttributes]
///   [LDAP-NAME            &ldapName]
///   [LDAP-DESC            &ldapDesc]
///   ID                    &id }
/// ```
///
#[derive(Debug)]
pub struct NAME_FORM {
    pub namedObjectClass: OBJECT_CLASS,
    pub MandatoryAttributes: Vec<ATTRIBUTE>,
    pub OptionalAttributes: OPTIONAL<Vec<ATTRIBUTE>>,
    pub ldapName: OPTIONAL<Vec<UTF8String>>,
    pub ldapDesc: OPTIONAL<UTF8String>,
    pub id: OBJECT_IDENTIFIER,
}
impl NAME_FORM {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DITStructureRule ::= SEQUENCE {
///   ruleIdentifier          RuleIdentifier,
///                  -- shall be unique within the scope of the subschema
///   nameForm                NAME-FORM.&id,
///   superiorStructureRules  SET SIZE (1..MAX) OF RuleIdentifier OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct DITStructureRule {
    pub ruleIdentifier: RuleIdentifier,
    pub nameForm: OBJECT_IDENTIFIER,
    pub superiorStructureRules: OPTIONAL<Vec<RuleIdentifier>>,
    pub _unrecognized: Vec<X690Element>,
}
impl DITStructureRule {
    pub fn new(
        ruleIdentifier: RuleIdentifier,
        nameForm: OBJECT_IDENTIFIER,
        superiorStructureRules: OPTIONAL<Vec<RuleIdentifier>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DITStructureRule {
            ruleIdentifier,
            nameForm,
            superiorStructureRules,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for DITStructureRule {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DITStructureRule(el)
    }
}

pub const _rctl1_components_for_DITStructureRule: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "ruleIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nameForm",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "superiorStructureRules",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DITStructureRule: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DITStructureRule: &[ComponentSpec; 0] = &[];

pub fn _decode_DITStructureRule(el: &X690Element) -> ASN1Result<DITStructureRule> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DITStructureRule")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DITStructureRule,
        _eal_components_for_DITStructureRule,
        _rctl2_components_for_DITStructureRule,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut ruleIdentifier_: OPTIONAL<RuleIdentifier> = None;
    let mut nameForm_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut superiorStructureRules_: OPTIONAL<Vec<RuleIdentifier>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ruleIdentifier" => ruleIdentifier_ = Some(_decode_RuleIdentifier(_el)?),
            "nameForm" => nameForm_ = Some(BER.decode_object_identifier(_el)?),
            "superiorStructureRules" => {
                superiorStructureRules_ =
                    Some(|el: &X690Element| -> ASN1Result<SET_OF<RuleIdentifier>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "superiorStructureRules",
                                ))
                            }
                        };
                        let mut items: SET_OF<RuleIdentifier> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_RuleIdentifier(el)?);
                        }
                        Ok(items)
                    }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DITStructureRule {
        ruleIdentifier: ruleIdentifier_.unwrap(),
        nameForm: nameForm_.unwrap(),
        superiorStructureRules: superiorStructureRules_,
        _unrecognized,
    })
}

pub fn _encode_DITStructureRule(value_: &DITStructureRule) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_RuleIdentifier(&value_.ruleIdentifier)?);
    components_.push(BER.encode_object_identifier(&value_.nameForm)?);
    if let Some(v_) = &value_.superiorStructureRules {
        components_.push(
            |value_: &SET_OF<RuleIdentifier>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_RuleIdentifier(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
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

pub fn _validate_DITStructureRule(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DITStructureRule")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DITStructureRule,
        _eal_components_for_DITStructureRule,
        _rctl2_components_for_DITStructureRule,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "ruleIdentifier" => _validate_RuleIdentifier(_el)?,
            "nameForm" => BER.validate_object_identifier(_el)?,
            "superiorStructureRules" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_RuleIdentifier(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "superiorStructureRules",
                    )),
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
/// RuleIdentifier  ::=  INTEGER
/// ```
pub type RuleIdentifier = INTEGER;

pub fn _decode_RuleIdentifier(el: &X690Element) -> ASN1Result<RuleIdentifier> {
    BER.decode_integer(&el)
}

pub fn _encode_RuleIdentifier(value_: &RuleIdentifier) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_RuleIdentifier(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// STRUCTURE-RULE ::= CLASS {
///   &nameForm               NAME-FORM,
///   &SuperiorStructureRules STRUCTURE-RULE.&id OPTIONAL,
///   &id                     RuleIdentifier }
/// WITH SYNTAX {
///   NAME FORM               &nameForm
///   [SUPERIOR RULES         &SuperiorStructureRules]
///   ID                      &id }
/// ```
///
#[derive(Debug)]
pub struct STRUCTURE_RULE {
    pub nameForm: NAME_FORM,
    pub SuperiorStructureRules: OPTIONAL<RuleIdentifier>,
    pub id: RuleIdentifier,
}
impl STRUCTURE_RULE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DITContentRule ::= SEQUENCE {
///   structuralObjectClass       OBJECT-CLASS.&id,
///   auxiliaries                 SET SIZE (1..MAX) OF OBJECT-CLASS.&id OPTIONAL,
///   mandatory              [1]  SET SIZE (1..MAX) OF ATTRIBUTE.&id    OPTIONAL,
///   optional               [2]  SET SIZE (1..MAX) OF ATTRIBUTE.&id    OPTIONAL,
///   precluded              [3]  SET SIZE (1..MAX) OF ATTRIBUTE.&id    OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct DITContentRule {
    pub structuralObjectClass: OBJECT_IDENTIFIER,
    pub auxiliaries: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub mandatory: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub optional: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub precluded: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub _unrecognized: Vec<X690Element>,
}
impl DITContentRule {
    pub fn new(
        structuralObjectClass: OBJECT_IDENTIFIER,
        auxiliaries: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        mandatory: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        optional: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        precluded: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DITContentRule {
            structuralObjectClass,
            auxiliaries,
            mandatory,
            optional,
            precluded,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for DITContentRule {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DITContentRule(el)
    }
}

pub const _rctl1_components_for_DITContentRule: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "structuralObjectClass",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "auxiliaries",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "mandatory",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "optional",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "precluded",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DITContentRule: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DITContentRule: &[ComponentSpec; 0] = &[];

pub fn _decode_DITContentRule(el: &X690Element) -> ASN1Result<DITContentRule> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DITContentRule"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DITContentRule,
        _eal_components_for_DITContentRule,
        _rctl2_components_for_DITContentRule,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut structuralObjectClass_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut auxiliaries_: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = None;
    let mut mandatory_: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = None;
    let mut optional_: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = None;
    let mut precluded_: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "structuralObjectClass" => {
                structuralObjectClass_ = Some(BER.decode_object_identifier(_el)?)
            }
            "auxiliaries" => {
                auxiliaries_ = Some(
                    |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "auxiliaries",
                                ))
                            }
                        };
                        let mut items: SET_OF<OBJECT_IDENTIFIER> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(BER.decode_object_identifier(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            "mandatory" => {
                mandatory_ = Some(|el: &X690Element| -> ASN1Result<Vec<OBJECT_IDENTIFIER>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "mandatory",
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
            "optional" => {
                optional_ = Some(|el: &X690Element| -> ASN1Result<Vec<OBJECT_IDENTIFIER>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "optional",
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
            "precluded" => {
                precluded_ = Some(|el: &X690Element| -> ASN1Result<Vec<OBJECT_IDENTIFIER>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "precluded",
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
    Ok(DITContentRule {
        structuralObjectClass: structuralObjectClass_.unwrap(),
        auxiliaries: auxiliaries_,
        mandatory: mandatory_,
        optional: optional_,
        precluded: precluded_,
        _unrecognized,
    })
}

pub fn _encode_DITContentRule(value_: &DITContentRule) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    components_.push(BER.encode_object_identifier(&value_.structuralObjectClass)?);
    if let Some(v_) = &value_.auxiliaries {
        components_.push(
            |value_: &SET_OF<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(BER.encode_object_identifier(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v_)?,
        );
    }
    if let Some(v_) = &value_.mandatory {
        components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
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
    if let Some(v_) = &value_.optional {
        components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
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
    if let Some(v_) = &value_.precluded {
        components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
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

pub fn _validate_DITContentRule(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DITContentRule"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DITContentRule,
        _eal_components_for_DITContentRule,
        _rctl2_components_for_DITContentRule,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "structuralObjectClass" => BER.validate_object_identifier(_el)?,
            "auxiliaries" => {
                |el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                BER.validate_object_identifier(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el
                            .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "auxiliaries")),
                    }
                }(_el)?
            }
            "mandatory" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "mandatory")
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
                        _ => Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "mandatory")
                        ),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "optional" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "optional")
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
                        _ => {
                            Err(el
                                .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "optional"))
                        }
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "precluded" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "precluded")
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
                        _ => Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "precluded")
                        ),
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
/// CONTENT-RULE ::= CLASS {
///   &structuralClass          OBJECT-CLASS.&id UNIQUE,
///   &Auxiliaries              OBJECT-CLASS OPTIONAL,
///   &Mandatory                ATTRIBUTE OPTIONAL,
///   &Optional                 ATTRIBUTE OPTIONAL,
///   &Precluded                ATTRIBUTE OPTIONAL }
/// WITH SYNTAX {
///   STRUCTURAL OBJECT-CLASS   &structuralClass
///   [AUXILIARY OBJECT-CLASSES &Auxiliaries]
///   [MUST CONTAIN             &Mandatory]
///   [MAY CONTAIN              &Optional]
///   [MUST-NOT CONTAIN         &Precluded] }
/// ```
///
#[derive(Debug)]
pub struct CONTENT_RULE {
    pub structuralClass: OBJECT_IDENTIFIER,
    pub Auxiliaries: OPTIONAL<Vec<OBJECT_CLASS>>,
    pub Mandatory: OPTIONAL<Vec<ATTRIBUTE>>,
    pub Optional: OPTIONAL<Vec<ATTRIBUTE>>,
    pub Precluded: OPTIONAL<Vec<ATTRIBUTE>>,
}
impl CONTENT_RULE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CONTEXT ::= CLASS {
///   &Type,
///   &defaultValue   &Type OPTIONAL,
///   &Assertion      OPTIONAL,
///   &absentMatch    BOOLEAN DEFAULT TRUE,
///   &id             OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   WITH SYNTAX     &Type
///   [DEFAULT-VALUE  &defaultValue]
///   [ASSERTED AS    &Assertion]
///   [ABSENT-MATCH   &absentMatch]
///   ID              &id }
/// ```
///
#[derive(Debug)]
pub struct CONTEXT {
    pub absentMatch: OPTIONAL<BOOLEAN>,
    pub id: OBJECT_IDENTIFIER,
}
impl CONTEXT {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DITContextUse ::= SEQUENCE {
///   attributeType           ATTRIBUTE.&id,
///   mandatoryContexts  [1]  SET SIZE (1..MAX) OF CONTEXT.&id OPTIONAL,
///   optionalContexts   [2]  SET SIZE (1..MAX) OF CONTEXT.&id OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct DITContextUse {
    pub attributeType: OBJECT_IDENTIFIER,
    pub mandatoryContexts: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub optionalContexts: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub _unrecognized: Vec<X690Element>,
}
impl DITContextUse {
    pub fn new(
        attributeType: OBJECT_IDENTIFIER,
        mandatoryContexts: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        optionalContexts: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DITContextUse {
            attributeType,
            mandatoryContexts,
            optionalContexts,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for DITContextUse {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DITContextUse(el)
    }
}

pub const _rctl1_components_for_DITContextUse: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "attributeType",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "mandatoryContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "optionalContexts",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DITContextUse: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DITContextUse: &[ComponentSpec; 0] = &[];

pub fn _decode_DITContextUse(el: &X690Element) -> ASN1Result<DITContextUse> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DITContextUse")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DITContextUse,
        _eal_components_for_DITContextUse,
        _rctl2_components_for_DITContextUse,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut attributeType_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut mandatoryContexts_: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = None;
    let mut optionalContexts_: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "attributeType" => attributeType_ = Some(BER.decode_object_identifier(_el)?),
            "mandatoryContexts" => {
                mandatoryContexts_ =
                    Some(|el: &X690Element| -> ASN1Result<Vec<OBJECT_IDENTIFIER>> {
                        Ok(
                            |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                                let elements = match &el.value {
                                    X690Value::Constructed(children) => children,
                                    _ => {
                                        return Err(el.to_asn1_err_named(
                                            ASN1ErrorCode::invalid_construction,
                                            "mandatoryContexts",
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
            "optionalContexts" => {
                optionalContexts_ = Some(|el: &X690Element| -> ASN1Result<Vec<OBJECT_IDENTIFIER>> {
                    Ok(
                        |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                            let elements = match &el.value {
                                X690Value::Constructed(children) => children,
                                _ => {
                                    return Err(el.to_asn1_err_named(
                                        ASN1ErrorCode::invalid_construction,
                                        "optionalContexts",
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
    Ok(DITContextUse {
        attributeType: attributeType_.unwrap(),
        mandatoryContexts: mandatoryContexts_,
        optionalContexts: optionalContexts_,
        _unrecognized,
    })
}

pub fn _encode_DITContextUse(value_: &DITContextUse) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_object_identifier(&value_.attributeType)?);
    if let Some(v_) = &value_.mandatoryContexts {
        components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
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
    if let Some(v_) = &value_.optionalContexts {
        components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
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

pub fn _validate_DITContextUse(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DITContextUse")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DITContextUse,
        _eal_components_for_DITContextUse,
        _rctl2_components_for_DITContextUse,
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
            "mandatoryContexts" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "mandatoryContexts",
                    ));
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
                            "mandatoryContexts",
                        )),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "optionalContexts" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "optionalContexts",
                    ));
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
                            "optionalContexts",
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
/// DIT-CONTEXT-USE-RULE ::= CLASS {
///   &attributeType      ATTRIBUTE.&id UNIQUE,
///   &Mandatory          CONTEXT OPTIONAL,
///   &Optional           CONTEXT OPTIONAL}
/// WITH SYNTAX {
///   ATTRIBUTE TYPE      &attributeType
///   [MANDATORY CONTEXTS &Mandatory]
///   [OPTIONAL CONTEXTS  &Optional] }
/// ```
///
#[derive(Debug)]
pub struct DIT_CONTEXT_USE_RULE {
    pub attributeType: OBJECT_IDENTIFIER,
    pub Mandatory: OPTIONAL<Vec<CONTEXT>>,
    pub Optional: OPTIONAL<Vec<CONTEXT>>,
}
impl DIT_CONTEXT_USE_RULE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// FRIENDS ::= CLASS {
///   &anchor   ATTRIBUTE.&id UNIQUE,
///   &Friends  ATTRIBUTE }
/// WITH SYNTAX {
///   ANCHOR    &anchor
///   FRIENDS   &Friends }
/// ```
///
#[derive(Debug)]
pub struct FRIENDS {
    pub anchor: OBJECT_IDENTIFIER,
    pub Friends: Vec<ATTRIBUTE>,
}
impl FRIENDS {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SYNTAX-NAME ::= CLASS {
///   &ldapDesc               UTF8String,
///   &Type                   OPTIONAL,
///   &id                     OBJECT IDENTIFIER UNIQUE }
/// WITH SYNTAX {
///   LDAP-DESC               &ldapDesc
///   [DIRECTORY SYNTAX       &Type]
///   ID                      &id }
/// ```
///
#[derive(Debug)]
pub struct SYNTAX_NAME {
    pub ldapDesc: UTF8String,
    pub id: OBJECT_IDENTIFIER,
}
impl SYNTAX_NAME {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subentry OBJECT-CLASS ::= {
///   SUBCLASS OF      {top}
///   KIND             structural
///   MUST CONTAIN     {commonName |
///                     subtreeSpecification}
///   LDAP-NAME        {"subentry"}
///   ID               id-sc-subentry }
/// ```
///
///
pub fn subentry() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([commonName(), subtreeSpecification()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("subentry")])), /* OBJECT_FIELD_SETTING */
        id: id_sc_subentry(),                                  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapDesc: None,
    }
}

pub mod subentry {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subentryNameForm NAME-FORM ::= {
///   NAMES            subentry
///   WITH ATTRIBUTES  {commonName}
///   ID               id-nf-subentryNameForm }
/// ```
///
///
pub fn subentryNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: subentry(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([commonName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_subentryNameForm(), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod subentryNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subtreeSpecification ATTRIBUTE ::= {
///   WITH SYNTAX             SubtreeSpecification
///   USAGE                   directoryOperation
///   LDAP-SYNTAX             subtreeSpec.&id
///   LDAP-NAME               {"subtreeSpecification"}
///   ID                      id-oa-subtreeSpecification }
/// ```
///
///
pub fn subtreeSpecification() -> ATTRIBUTE {
    ATTRIBUTE {
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(subtreeSpec().id),             /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("subtreeSpecification")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_subtreeSpecification(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod subtreeSpecification {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SubtreeSpecification; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SubtreeSpecification(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SubtreeSpecification(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SubtreeSpecification(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// administrativeRole ATTRIBUTE ::= {
///   WITH SYNTAX             OBJECT-CLASS.&id
///   EQUALITY MATCHING RULE  objectIdentifierMatch
///   USAGE                   directoryOperation
///   LDAP-SYNTAX             oid.&id
///   LDAP-NAME               {"administrativeRole"}
///   ID                      id-oa-administrativeRole }
/// ```
///
///
pub fn administrativeRole() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),          /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),                              /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("administrativeRole")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_administrativeRole(),                          /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod administrativeRole {
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
/// createTimestamp ATTRIBUTE ::= {
///   WITH SYNTAX             GeneralizedTime
///   -- as per 46.3 b) or c) of Rec. ITU-T X.680 | ISO/IEC 8824-1
///   EQUALITY MATCHING RULE  generalizedTimeMatch
///   ORDERING MATCHING RULE  generalizedTimeOrderingMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   LDAP-SYNTAX             generalizedTime.&id
///   LDAP-NAME               {"createTimestamp"}
///   ID                      id-oa-createTimestamp }
/// ```
///
///
pub fn createTimestamp() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(generalizedTimeMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(generalizedTimeOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                              /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                       /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),         /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(generalizedTime().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("createTimestamp")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_createTimestamp(),                            /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod createTimestamp {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = GeneralizedTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_generalized_time(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_generalized_time(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_generalized_time(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// modifyTimestamp ATTRIBUTE ::= {
///   WITH SYNTAX             GeneralizedTime
///   -- as per 46.3 b) or c) of Rec. ITU-T X.680 | ISO/IEC 8824-1
///   EQUALITY MATCHING RULE  generalizedTimeMatch
///   ORDERING MATCHING RULE  generalizedTimeOrderingMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   LDAP-SYNTAX             generalizedTime.&id
///   LDAP-NAME               {"modifyTimestamp"}
///   ID                      id-oa-modifyTimestamp }
/// ```
///
///
pub fn modifyTimestamp() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(generalizedTimeMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(generalizedTimeOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                              /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                       /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),         /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(generalizedTime().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("modifyTimestamp")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_modifyTimestamp(),                            /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod modifyTimestamp {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = GeneralizedTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_generalized_time(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_generalized_time(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_generalized_time(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subschemaTimestamp ATTRIBUTE ::= {
///   WITH SYNTAX             GeneralizedTime
///   -- as per 46.3 b) or c) of Rec. ITU-T X.680 | ISO/IEC 8824-1
///   EQUALITY MATCHING RULE  generalizedTimeMatch
///   ORDERING MATCHING RULE  generalizedTimeOrderingMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   ID                      id-oa-subschemaTimestamp }
/// ```
///
///
pub fn subschemaTimestamp() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(generalizedTimeMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(generalizedTimeOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                              /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                       /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),         /* OBJECT_FIELD_SETTING */
        id: id_oa_subschemaTimestamp(),                         /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod subschemaTimestamp {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = GeneralizedTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_generalized_time(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_generalized_time(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_generalized_time(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// creatorsName ATTRIBUTE ::= {
///   WITH SYNTAX             DistinguishedName
///   EQUALITY MATCHING RULE  distinguishedNameMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   LDAP-SYNTAX             dn.&id
///   LDAP-NAME               {"creatorsName"}
///   ID                      id-oa-creatorsName }
/// ```
///
///
pub fn creatorsName() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                                /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                         /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),           /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dn().id),                                /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("creatorsName")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_creatorsName(),                                 /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod creatorsName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// modifiersName ATTRIBUTE ::= {
///   WITH SYNTAX             DistinguishedName
///   EQUALITY MATCHING RULE  distinguishedNameMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   LDAP-SYNTAX             dn.&id
///   LDAP-NAME               {"modifiersName"}
///   ID                      id-oa-modifiersName }
/// ```
///
///
pub fn modifiersName() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                                /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                         /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),           /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dn().id),                                /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("modifiersName")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_modifiersName(),                                /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod modifiersName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subschemaSubentryList ATTRIBUTE ::= {
///   WITH SYNTAX             DistinguishedName
///   EQUALITY MATCHING RULE  distinguishedNameMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   LDAP-SYNTAX             dn.&id
///   LDAP-NAME               {"subschemaSubentry"}
///   ID                      id-oa-subschemaSubentryList }
/// ```
///
///
pub fn subschemaSubentryList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                                /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                         /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),           /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dn().id),                                /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("subschemaSubentry")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_subschemaSubentryList(),                        /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod subschemaSubentryList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// accessControlSubentryList ATTRIBUTE ::= {
///   WITH SYNTAX             DistinguishedName
///   EQUALITY MATCHING RULE  distinguishedNameMatch
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   ID                      id-oa-accessControlSubentryList }
/// ```
///
///
pub fn accessControlSubentryList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                         /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),           /* OBJECT_FIELD_SETTING */
        id: id_oa_accessControlSubentryList(),                    /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod accessControlSubentryList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveAttributeSubentryList ATTRIBUTE ::= {
///   WITH SYNTAX             DistinguishedName
///   EQUALITY MATCHING RULE  distinguishedNameMatch
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   ID                      id-oa-collectiveAttributeSubentryList }
/// ```
///
///
pub fn collectiveAttributeSubentryList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                         /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),           /* OBJECT_FIELD_SETTING */
        id: id_oa_collectiveAttributeSubentryList(),              /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectiveAttributeSubentryList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// contextDefaultSubentryList ATTRIBUTE ::= {
///   WITH SYNTAX             DistinguishedName
///   EQUALITY MATCHING RULE  distinguishedNameMatch
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   ID                      id-oa-contextDefaultSubentryList }
/// ```
///
///
pub fn contextDefaultSubentryList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                         /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),           /* OBJECT_FIELD_SETTING */
        id: id_oa_contextDefaultSubentryList(),                   /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod contextDefaultSubentryList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// serviceAdminSubentryList ATTRIBUTE ::= {
///   WITH SYNTAX             DistinguishedName
///   EQUALITY MATCHING RULE  distinguishedNameMatch
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   ID                      id-oa-serviceAdminSubentryList }
/// ```
///
///
pub fn serviceAdminSubentryList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                         /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),           /* OBJECT_FIELD_SETTING */
        id: id_oa_serviceAdminSubentryList(),                     /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod serviceAdminSubentryList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdAdminSubentryList  ATTRIBUTE ::= {
///   WITH SYNTAX             DistinguishedName
///   EQUALITY MATCHING RULE  distinguishedNameMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   LDAP-SYNTAX             dn.&id
///   LDAP-NAME               {"pwdAdminSubentryList"}
///   ID                      id-oa-pwdAdminSubentryList }
/// ```
///
///
pub fn pwdAdminSubentryList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                                /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                         /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),           /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dn().id),                                /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdAdminSubentryList")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdAdminSubentryList(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdAdminSubentryList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hasSubordinates ATTRIBUTE ::= {
///   WITH SYNTAX             BOOLEAN
///   EQUALITY MATCHING RULE  booleanMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   ID                      id-oa-hasSubordinates }
/// ```
///
///
pub fn hasSubordinates() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(booleanMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),               /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        id: id_oa_hasSubordinates(),                    /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod hasSubordinates {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = BOOLEAN; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_boolean(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_boolean(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_boolean(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// accessControlSubentry OBJECT-CLASS ::= {
///   KIND          auxiliary
///   ID            id-sc-accessControlSubentry }
/// ```
///
///
pub fn accessControlSubentry() -> OBJECT_CLASS {
    OBJECT_CLASS {
        kind: Some(ObjectClassKind_auxiliary), /* OBJECT_FIELD_SETTING */
        id: id_sc_accessControlSubentry(),     /* OBJECT_FIELD_SETTING */
        Superclasses: None,
        MandatoryAttributes: None,
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod accessControlSubentry {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveAttributeSubentry OBJECT-CLASS ::= {
///   KIND          auxiliary
///   ID            id-sc-collectiveAttributeSubentry }
/// ```
///
///
pub fn collectiveAttributeSubentry() -> OBJECT_CLASS {
    OBJECT_CLASS {
        kind: Some(ObjectClassKind_auxiliary), /* OBJECT_FIELD_SETTING */
        id: id_sc_collectiveAttributeSubentry(), /* OBJECT_FIELD_SETTING */
        Superclasses: None,
        MandatoryAttributes: None,
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod collectiveAttributeSubentry {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveExclusions ATTRIBUTE ::= {
///   WITH SYNTAX             OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE  objectIdentifierMatch
///   USAGE                   directoryOperation
///   ID                      id-oa-collectiveExclusions }
/// ```
///
///
pub fn collectiveExclusions() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),          /* OBJECT_FIELD_SETTING */
        id: id_oa_collectiveExclusions(),                        /* OBJECT_FIELD_SETTING */
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

pub mod collectiveExclusions {
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
/// contextAssertionSubentry OBJECT-CLASS ::= {
///   KIND          auxiliary
///   MUST CONTAIN  {contextAssertionDefaults}
///   ID            id-sc-contextAssertionSubentry }
/// ```
///
///
pub fn contextAssertionSubentry() -> OBJECT_CLASS {
    OBJECT_CLASS {
        kind: Some(ObjectClassKind_auxiliary), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([contextAssertionDefaults()])), /* OBJECT_FIELD_SETTING */
        id: id_sc_contextAssertionSubentry(), /* OBJECT_FIELD_SETTING */
        Superclasses: None,
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod contextAssertionSubentry {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// contextAssertionDefaults ATTRIBUTE ::= {
///   WITH SYNTAX             TypeAndContextAssertion
///   EQUALITY MATCHING RULE  objectIdentifierFirstComponentMatch
///   USAGE                   directoryOperation
///   ID                      id-oa-contextAssertionDefault }
/// ```
///
///
pub fn contextAssertionDefaults() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        id: id_oa_contextAssertionDefault(),            /* OBJECT_FIELD_SETTING */
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

pub mod contextAssertionDefaults {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = TypeAndContextAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_TypeAndContextAssertion(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_TypeAndContextAssertion(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_TypeAndContextAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// serviceAdminSubentry OBJECT-CLASS ::= {
///   KIND          auxiliary
///   MUST CONTAIN  {searchRules}
///   ID            id-sc-serviceAdminSubentry }
/// ```
///
///
pub fn serviceAdminSubentry() -> OBJECT_CLASS {
    OBJECT_CLASS {
        kind: Some(ObjectClassKind_auxiliary), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([searchRules()])), /* OBJECT_FIELD_SETTING */
        id: id_sc_serviceAdminSubentry(),      /* OBJECT_FIELD_SETTING */
        Superclasses: None,
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod serviceAdminSubentry {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// searchRules ATTRIBUTE ::= {
///   WITH SYNTAX             SearchRuleDescription
///   EQUALITY MATCHING RULE  integerFirstComponentMatch
///   USAGE                   directoryOperation
///   ID                      id-oa-searchRules }
/// ```
///
///
pub fn searchRules() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),               /* OBJECT_FIELD_SETTING */
        id: id_oa_searchRules(),                                      /* OBJECT_FIELD_SETTING */
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

pub mod searchRules {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SearchRuleDescription; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SearchRuleDescription(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SearchRuleDescription(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SearchRuleDescription(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SearchRuleDescription ::= SEQUENCE {
///   COMPONENTS OF      SearchRule,
///   name         [28]  SET SIZE (1..MAX) OF UnboundedDirectoryString OPTIONAL,
///   description  [29]  UnboundedDirectoryString OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct SearchRuleDescription {
    pub id: INTEGER,                                          /* REPLICATED_COMPONENT */
    pub dmdId: OBJECT_IDENTIFIER,                             /* REPLICATED_COMPONENT */
    pub serviceType: OPTIONAL<OBJECT_IDENTIFIER>,             /* REPLICATED_COMPONENT */
    pub userClass: OPTIONAL<INTEGER>,                         /* REPLICATED_COMPONENT */
    pub inputAttributeTypes: OPTIONAL<Vec<RequestAttribute>>, /* REPLICATED_COMPONENT */
    pub attributeCombination: OPTIONAL<AttributeCombination>, /* REPLICATED_COMPONENT */
    pub outputAttributeTypes: OPTIONAL<Vec<ResultAttribute>>, /* REPLICATED_COMPONENT */
    pub defaultControls: OPTIONAL<ControlOptions>,            /* REPLICATED_COMPONENT */
    pub mandatoryControls: OPTIONAL<ControlOptions>,          /* REPLICATED_COMPONENT */
    pub searchRuleControls: OPTIONAL<ControlOptions>,         /* REPLICATED_COMPONENT */
    pub familyGrouping: OPTIONAL<FamilyGrouping>,             /* REPLICATED_COMPONENT */
    pub familyReturn: OPTIONAL<FamilyReturn>,                 /* REPLICATED_COMPONENT */
    pub relaxation: OPTIONAL<RelaxationPolicy>,               /* REPLICATED_COMPONENT */
    pub additionalControl: OPTIONAL<Vec<AttributeType>>,      /* REPLICATED_COMPONENT */
    pub allowedSubset: OPTIONAL<AllowedSubset>,               /* REPLICATED_COMPONENT */
    pub imposedSubset: OPTIONAL<ImposedSubset>,               /* REPLICATED_COMPONENT */
    pub entryLimit: OPTIONAL<EntryLimit>,                     /* REPLICATED_COMPONENT */
    pub name: OPTIONAL<Vec<UnboundedDirectoryString>>,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub _unrecognized: Vec<X690Element>,
}
impl SearchRuleDescription {
    pub fn new(
        id: INTEGER,                                          /* REPLICATED_COMPONENT */
        dmdId: OBJECT_IDENTIFIER,                             /* REPLICATED_COMPONENT */
        serviceType: OPTIONAL<OBJECT_IDENTIFIER>,             /* REPLICATED_COMPONENT */
        userClass: OPTIONAL<INTEGER>,                         /* REPLICATED_COMPONENT */
        inputAttributeTypes: OPTIONAL<Vec<RequestAttribute>>, /* REPLICATED_COMPONENT */
        attributeCombination: OPTIONAL<AttributeCombination>, /* REPLICATED_COMPONENT */
        outputAttributeTypes: OPTIONAL<Vec<ResultAttribute>>, /* REPLICATED_COMPONENT */
        defaultControls: OPTIONAL<ControlOptions>,            /* REPLICATED_COMPONENT */
        mandatoryControls: OPTIONAL<ControlOptions>,          /* REPLICATED_COMPONENT */
        searchRuleControls: OPTIONAL<ControlOptions>,         /* REPLICATED_COMPONENT */
        familyGrouping: OPTIONAL<FamilyGrouping>,             /* REPLICATED_COMPONENT */
        familyReturn: OPTIONAL<FamilyReturn>,                 /* REPLICATED_COMPONENT */
        relaxation: OPTIONAL<RelaxationPolicy>,               /* REPLICATED_COMPONENT */
        additionalControl: OPTIONAL<Vec<AttributeType>>,      /* REPLICATED_COMPONENT */
        allowedSubset: OPTIONAL<AllowedSubset>,               /* REPLICATED_COMPONENT */
        imposedSubset: OPTIONAL<ImposedSubset>,               /* REPLICATED_COMPONENT */
        entryLimit: OPTIONAL<EntryLimit>,                     /* REPLICATED_COMPONENT */
        name: OPTIONAL<Vec<UnboundedDirectoryString>>,
        description: OPTIONAL<UnboundedDirectoryString>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SearchRuleDescription {
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
            name,
            description,
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
impl TryFrom<&X690Element> for SearchRuleDescription {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SearchRuleDescription(el)
    }
}

pub const _rctl1_components_for_SearchRuleDescription: &[ComponentSpec; 19] = &[
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
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 28)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::tag((TagClass::CONTEXT, 29)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SearchRuleDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SearchRuleDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_SearchRuleDescription(el: &X690Element) -> ASN1Result<SearchRuleDescription> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchRuleDescription")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SearchRuleDescription,
        _eal_components_for_SearchRuleDescription,
        _rctl2_components_for_SearchRuleDescription,
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
    let mut name_: OPTIONAL<Vec<UnboundedDirectoryString>> = None;
    let mut description_: OPTIONAL<UnboundedDirectoryString> = None;
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
            "name" => {
                name_ = Some(
                    |el: &X690Element| -> ASN1Result<Vec<UnboundedDirectoryString>> {
                        Ok(
                            |el: &X690Element| -> ASN1Result<SET_OF<UnboundedDirectoryString>> {
                                let elements = match &el.value {
                                    X690Value::Constructed(children) => children,
                                    _ => {
                                        return Err(el.to_asn1_err_named(
                                            ASN1ErrorCode::invalid_construction,
                                            "name",
                                        ))
                                    }
                                };
                                let mut items: SET_OF<UnboundedDirectoryString> =
                                    Vec::with_capacity(elements.len());
                                for el in elements.iter() {
                                    items.push(_decode_UnboundedDirectoryString(el)?);
                                }
                                Ok(items)
                            }(&el.inner()?)?,
                        )
                    }(_el)?,
                )
            }
            "description" => {
                description_ = Some(|el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
                    Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(SearchRuleDescription {
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
        name: name_,
        description: description_,
        _unrecognized,
    })
}

pub fn _encode_SearchRuleDescription(value_: &SearchRuleDescription) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(29);
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
        if *v_ != SearchRuleDescription::_default_value_for_attributeCombination() {
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
        if *v_ != SearchRuleDescription::_default_value_for_allowedSubset() {
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
    if let Some(v_) = &value_.name {
        components_.push(
            |v_1: &Vec<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 28),
                    X690Value::from_explicit(
                        &|value_: &SET_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
                            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                            for v in value_ {
                                children.push(_encode_UnboundedDirectoryString(&v)?);
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
    if let Some(v_) = &value_.description {
        components_.push(
            |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 29),
                    X690Value::from_explicit(_encode_UnboundedDirectoryString(&v_1)?),
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

pub fn _validate_SearchRuleDescription(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SearchRuleDescription")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_SearchRuleDescription,
        _eal_components_for_SearchRuleDescription,
        _rctl2_components_for_SearchRuleDescription,
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
            "name" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 28 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "name"));
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_UnboundedDirectoryString(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "name")),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "description" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 29 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "description")
                    );
                }
                Ok(_validate_UnboundedDirectoryString(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdAdminSubentry OBJECT-CLASS ::= {
///   KIND           auxiliary
///   MUST CONTAIN   { pwdAttribute }
///   LDAP-NAME      {"pwdAdminSubentry"}
///   ID             id-sc-pwdAdminSubentry }
/// ```
///
///
pub fn pwdAdminSubentry() -> OBJECT_CLASS {
    OBJECT_CLASS {
        kind: Some(ObjectClassKind_auxiliary), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([pwdAttribute()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdAdminSubentry")])), /* OBJECT_FIELD_SETTING */
        id: id_sc_pwdAdminSubentry(),          /* OBJECT_FIELD_SETTING */
        Superclasses: None,
        OptionalAttributes: None,
        ldapDesc: None,
    }
}

pub mod pwdAdminSubentry {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdAttribute ATTRIBUTE ::= {
///   WITH SYNTAX             ATTRIBUTE.&id
///   EQUALITY MATCHING RULE  objectIdentifierMatch
///   SINGLE VALUE            TRUE
///   LDAP-SYNTAX             oid.&id
///   LDAP-NAME               {"pwdAttribute"}
///   ID                      id-at-pwdAttribute }
/// ```
///
///
pub fn pwdAttribute() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                               /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),                              /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdAttribute")])), /* OBJECT_FIELD_SETTING */
        id: id_at_pwdAttribute(),                                /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdAttribute {
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
/// pwdHistory{ATTRIBUTE:passwordAttribute,MATCHING-RULE:historyMatch,OBJECT IDENTIFIER:id}
/// ATTRIBUTE ::= {
///   WITH SYNTAX             PwdHistory{passwordAttribute}
///   EQUALITY MATCHING RULE  historyMatch
///   USAGE                   directoryOperation
///   ID                      id}
/// ```
pub fn pwdHistory(
    passwordAttribute: ATTRIBUTE,
    historyMatch: MATCHING_RULE,
    id: OBJECT_IDENTIFIER,
) -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: None,
        equality_match: Some(Box::new(historyMatch)),
        ordering_match: None,
        substrings_match: None,
        single_valued: None,
        collective: None,
        dummy: None,
        no_user_modification: None,
        usage: Some(AttributeUsage_directoryOperation),
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: None,
        id,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PwdHistory{ATTRIBUTE:passwordAttribute} ::= SEQUENCE {
///   time       GeneralizedTime,
///   password   passwordAttribute.&Type,
///   ...}
/// ```
///
#[derive(Debug, Clone)]
pub struct PwdHistory {
    pub time: GeneralizedTime,
    pub password: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl PwdHistory {
    pub fn new(
        time: GeneralizedTime,
        password: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PwdHistory {
            time,
            password,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for PwdHistory {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PwdHistory(el)
    }
}

pub const _rctl1_components_for_PwdHistory: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "time",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 24)),
        None,
        None,
    ),
    ComponentSpec::new("password", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_PwdHistory: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PwdHistory: &[ComponentSpec; 0] = &[];

pub fn _decode_PwdHistory(el: &X690Element) -> ASN1Result<PwdHistory> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PwdHistory")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PwdHistory,
        _eal_components_for_PwdHistory,
        _rctl2_components_for_PwdHistory,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut time_: OPTIONAL<GeneralizedTime> = None;
    let mut password_: OPTIONAL<X690Element> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "time" => time_ = Some(BER.decode_generalized_time(_el)?),
            "password" => password_ = Some(x690_identity(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PwdHistory {
        time: time_.unwrap(),
        password: password_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_PwdHistory(value_: &PwdHistory) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_generalized_time(&value_.time)?);
    components_.push(x690_identity(&value_.password)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_PwdHistory(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PwdHistory")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PwdHistory,
        _eal_components_for_PwdHistory,
        _rctl2_components_for_PwdHistory,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "time" => BER.validate_generalized_time(_el)?,
            "password" => BER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdRecentlyExpired{ATTRIBUTE:passwordAttribute,OBJECT IDENTIFIER:id} ATTRIBUTE ::= {
/// 	WITH SYNTAX             passwordAttribute.&Type
/// 	EQUALITY MATCHING RULE  passwordAttribute.&equality-match
/// 	SINGLE VALUE            TRUE
/// 	USAGE                   directoryOperation
/// 	ID                      id}
/// ```
///
pub fn pwdRecentlyExpired(passwordAttribute: ATTRIBUTE, id: OBJECT_IDENTIFIER) -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: None,
        equality_match: passwordAttribute.equality_match,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(true),
        collective: None,
        dummy: None,
        no_user_modification: None,
        usage: Some(AttributeUsage_directoryOperation),
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: None,
        id,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdHistoryMatch{ATTRIBUTE:passwordAttribute,OBJECT IDENTIFIER:id}
/// MATCHING-RULE ::= {
///   SYNTAX     passwordAttribute.&Type
///   ID         id}
/// ```
///
pub fn pwdHistoryMatch(id: OBJECT_IDENTIFIER) -> MATCHING_RULE {
    MATCHING_RULE {
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        id,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hierarchyLevel ATTRIBUTE ::= {
///   WITH SYNTAX             HierarchyLevel
///   EQUALITY MATCHING RULE  integerMatch
///   ORDERING MATCHING RULE  integerOrderingMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   ID                      id-oa-hierarchyLevel }
/// ```
///
///
pub fn hierarchyLevel() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),               /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        id: id_oa_hierarchyLevel(),                     /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod hierarchyLevel {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = HierarchyLevel; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_HierarchyLevel(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_HierarchyLevel(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_HierarchyLevel(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HierarchyLevel  ::=  INTEGER
/// ```
pub type HierarchyLevel = INTEGER;

pub fn _decode_HierarchyLevel(el: &X690Element) -> ASN1Result<HierarchyLevel> {
    BER.decode_integer(&el)
}

pub fn _encode_HierarchyLevel(value_: &HierarchyLevel) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_HierarchyLevel(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hierarchyBelow ATTRIBUTE ::= {
///   WITH SYNTAX             HierarchyBelow
///   EQUALITY MATCHING RULE  booleanMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   ID                      id-oa-hierarchyBelow }
/// ```
///
///
pub fn hierarchyBelow() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(booleanMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),               /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        id: id_oa_hierarchyBelow(),                     /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod hierarchyBelow {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = HierarchyBelow; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_HierarchyBelow(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_HierarchyBelow(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_HierarchyBelow(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HierarchyBelow  ::=  BOOLEAN
/// ```
pub type HierarchyBelow = BOOLEAN; // BooleanType

pub fn _decode_HierarchyBelow(el: &X690Element) -> ASN1Result<HierarchyBelow> {
    BER.decode_boolean(&el)
}

pub fn _encode_HierarchyBelow(value_: &HierarchyBelow) -> ASN1Result<X690Element> {
    BER.encode_boolean(&value_)
}

pub fn _validate_HierarchyBelow(el: &X690Element) -> ASN1Result<()> {
    BER.validate_boolean(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hierarchyParent ATTRIBUTE ::= {
///   WITH SYNTAX             DistinguishedName
///   EQUALITY MATCHING RULE  distinguishedNameMatch
///   SINGLE VALUE            TRUE
///   USAGE                   directoryOperation
///   ID                      id-oa-hierarchyParent }
/// ```
///
///
pub fn hierarchyParent() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                                /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),           /* OBJECT_FIELD_SETTING */
        id: id_oa_hierarchyParent(),                              /* OBJECT_FIELD_SETTING */
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

pub mod hierarchyParent {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hierarchyTop ATTRIBUTE ::= {
///   WITH SYNTAX             DistinguishedName
///   EQUALITY MATCHING RULE  distinguishedNameMatch
///   SINGLE VALUE            TRUE
///   USAGE                   directoryOperation
///   ID                      id-oa-hierarchyTop }
/// ```
///
///
pub fn hierarchyTop() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                                /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),           /* OBJECT_FIELD_SETTING */
        id: id_oa_hierarchyTop(),                                 /* OBJECT_FIELD_SETTING */
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

pub mod hierarchyTop {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DistinguishedName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DistinguishedName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DistinguishedName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DistinguishedName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-top                              OBJECT IDENTIFIER ::= {id-oc 0}
/// ```
///
///
#[inline]
pub fn id_oc_top () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oc(), 0).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-alias                            OBJECT IDENTIFIER ::= {id-oc 1}
/// ```
///
///
#[inline]
pub fn id_oc_alias () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oc(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-parent                           OBJECT IDENTIFIER ::= {id-oc 28}
/// ```
///
///
#[inline]
pub fn id_oc_parent () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oc(), 28).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-child                            OBJECT IDENTIFIER ::= {id-oc 29}
/// ```
///
///
#[inline]
pub fn id_oc_child () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oc(), 29).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-objectClass                      OBJECT IDENTIFIER ::= {id-at 0}
/// ```
///
///
#[inline]
pub fn id_at_objectClass () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 0).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-aliasedEntryName                 OBJECT IDENTIFIER ::= {id-at 1}
/// ```
///
///
#[inline]
pub fn id_at_aliasedEntryName () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-pwdAttribute                     OBJECT IDENTIFIER ::= {id-at 84}
/// ```
///
///
#[inline]
pub fn id_at_pwdAttribute () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 84).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-objectIdentifierMatch            OBJECT IDENTIFIER ::= {id-mr 0}
/// ```
///
///
#[inline]
pub fn id_mr_objectIdentifierMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 0).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-distinguishedNameMatch           OBJECT IDENTIFIER ::= {id-mr 1}
/// ```
///
///
#[inline]
pub fn id_mr_distinguishedNameMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-excludeAllCollectiveAttributes   OBJECT IDENTIFIER ::= {id-oa 0}
/// ```
///
///
#[inline]
pub fn id_oa_excludeAllCollectiveAttributes () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 0).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-createTimestamp                  OBJECT IDENTIFIER ::= {id-oa 1}
/// ```
///
///
#[inline]
pub fn id_oa_createTimestamp () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-modifyTimestamp                  OBJECT IDENTIFIER ::= {id-oa 2}
/// ```
///
///
#[inline]
pub fn id_oa_modifyTimestamp () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-creatorsName                     OBJECT IDENTIFIER ::= {id-oa 3}
/// ```
///
///
#[inline]
pub fn id_oa_creatorsName () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-modifiersName                    OBJECT IDENTIFIER ::= {id-oa 4}
/// ```
///
///
#[inline]
pub fn id_oa_modifiersName () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-administrativeRole               OBJECT IDENTIFIER ::= {id-oa 5}
/// ```
///
///
#[inline]
pub fn id_oa_administrativeRole () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 5).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-subtreeSpecification             OBJECT IDENTIFIER ::= {id-oa 6}
/// ```
///
///
#[inline]
pub fn id_oa_subtreeSpecification () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 6).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-collectiveExclusions             OBJECT IDENTIFIER ::= {id-oa 7}
/// ```
///
///
#[inline]
pub fn id_oa_collectiveExclusions () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 7).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-subschemaTimestamp               OBJECT IDENTIFIER ::= {id-oa 8}
/// ```
///
///
#[inline]
pub fn id_oa_subschemaTimestamp () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 8).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-hasSubordinates                  OBJECT IDENTIFIER ::= {id-oa 9}
/// ```
///
///
#[inline]
pub fn id_oa_hasSubordinates () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 9).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-subschemaSubentryList            OBJECT IDENTIFIER ::= {id-oa 10}
/// ```
///
///
#[inline]
pub fn id_oa_subschemaSubentryList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 10).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-accessControlSubentryList        OBJECT IDENTIFIER ::= {id-oa 11}
/// ```
///
///
#[inline]
pub fn id_oa_accessControlSubentryList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 11).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-collectiveAttributeSubentryList  OBJECT IDENTIFIER ::= {id-oa 12}
/// ```
///
///
#[inline]
pub fn id_oa_collectiveAttributeSubentryList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 12).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-contextDefaultSubentryList       OBJECT IDENTIFIER ::= {id-oa 13}
/// ```
///
///
#[inline]
pub fn id_oa_contextDefaultSubentryList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 13).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-contextAssertionDefault          OBJECT IDENTIFIER ::= {id-oa 14}
/// ```
///
///
#[inline]
pub fn id_oa_contextAssertionDefault () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 14).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-serviceAdminSubentryList         OBJECT IDENTIFIER ::= {id-oa 15}
/// ```
///
///
#[inline]
pub fn id_oa_serviceAdminSubentryList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 15).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-searchRules                      OBJECT IDENTIFIER ::= {id-oa 16}
/// ```
///
///
#[inline]
pub fn id_oa_searchRules () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 16).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-hierarchyLevel                   OBJECT IDENTIFIER ::= {id-oa 17}
/// ```
///
///
#[inline]
pub fn id_oa_hierarchyLevel () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 17).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-hierarchyBelow                   OBJECT IDENTIFIER ::= {id-oa 18}
/// ```
///
///
#[inline]
pub fn id_oa_hierarchyBelow () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 18).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-hierarchyParent                  OBJECT IDENTIFIER ::= {id-oa 19}
/// ```
///
///
#[inline]
pub fn id_oa_hierarchyParent () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 19).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-hierarchyTop                     OBJECT IDENTIFIER ::= {id-oa 20}
/// ```
///
///
#[inline]
pub fn id_oa_hierarchyTop () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 20).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdAdminSubentryList             OBJECT IDENTIFIER ::= {id-oa 21}
/// ```
///
///
#[inline]
pub fn id_oa_pwdAdminSubentryList () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 21).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-allAttributeTypes                OBJECT IDENTIFIER ::= {id-oa 48}
/// ```
///
///
#[inline]
pub fn id_oa_allAttributeTypes () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 48).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sc-subentry                         OBJECT IDENTIFIER ::= {id-sc 0}
/// ```
///
///
#[inline]
pub fn id_sc_subentry () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_sc(), 0).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sc-accessControlSubentry            OBJECT IDENTIFIER ::= {id-sc 1}
/// ```
///
///
#[inline]
pub fn id_sc_accessControlSubentry () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_sc(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sc-collectiveAttributeSubentry      OBJECT IDENTIFIER ::= {id-sc 2}
/// ```
///
///
#[inline]
pub fn id_sc_collectiveAttributeSubentry () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_sc(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sc-contextAssertionSubentry         OBJECT IDENTIFIER ::= {id-sc 3}
/// ```
///
///
#[inline]
pub fn id_sc_contextAssertionSubentry () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_sc(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sc-serviceAdminSubentry             OBJECT IDENTIFIER ::= {id-sc 4}
/// ```
///
///
#[inline]
pub fn id_sc_serviceAdminSubentry () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_sc(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sc-pwdAdminSubentry                 OBJECT IDENTIFIER ::= {id-sc 5}
/// ```
///
///
#[inline]
pub fn id_sc_pwdAdminSubentry () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_sc(), 5).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-subentryNameForm                 OBJECT IDENTIFIER ::= {id-nf 16}
/// ```
///
///
#[inline]
pub fn id_nf_subentryNameForm () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_nf(), 16).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ar-autonomousArea                   OBJECT IDENTIFIER ::= {id-ar 1}
/// ```
///
///
#[inline]
pub fn id_ar_autonomousArea () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ar(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ar-accessControlSpecificArea        OBJECT IDENTIFIER ::= {id-ar 2}
/// ```
///
///
#[inline]
pub fn id_ar_accessControlSpecificArea () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ar(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ar-accessControlInnerArea           OBJECT IDENTIFIER ::= {id-ar 3}
/// ```
///
///
#[inline]
pub fn id_ar_accessControlInnerArea () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ar(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ar-subschemaAdminSpecificArea       OBJECT IDENTIFIER ::= {id-ar 4}
/// ```
///
///
#[inline]
pub fn id_ar_subschemaAdminSpecificArea () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ar(), 4).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ar-collectiveAttributeSpecificArea  OBJECT IDENTIFIER ::= {id-ar 5}
/// ```
///
///
#[inline]
pub fn id_ar_collectiveAttributeSpecificArea () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ar(), 5).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ar-collectiveAttributeInnerArea     OBJECT IDENTIFIER ::= {id-ar 6}
/// ```
///
///
#[inline]
pub fn id_ar_collectiveAttributeInnerArea () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ar(), 6).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ar-contextDefaultSpecificArea       OBJECT IDENTIFIER ::= {id-ar 7}
/// ```
///
///
#[inline]
pub fn id_ar_contextDefaultSpecificArea () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ar(), 7).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ar-serviceSpecificArea              OBJECT IDENTIFIER ::= {id-ar 8}
/// ```
///
///
#[inline]
pub fn id_ar_serviceSpecificArea () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ar(), 8).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ar-pwdAdminSpecificArea             OBJECT IDENTIFIER ::= {id-ar 9}
/// ```
///
///
#[inline]
pub fn id_ar_pwdAdminSpecificArea () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ar(), 9).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Attribute-valuesWithContext-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct Attribute_valuesWithContext_Item {
    pub value: X690Element,
    pub contextList: Vec<Context>,
    pub _unrecognized: Vec<X690Element>,
}
impl Attribute_valuesWithContext_Item {
    pub fn new(
        value: X690Element,
        contextList: Vec<Context>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Attribute_valuesWithContext_Item {
            value,
            contextList,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for Attribute_valuesWithContext_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Attribute_valuesWithContext_Item(el)
    }
}

pub const _rctl1_components_for_Attribute_valuesWithContext_Item: &[ComponentSpec; 2] = &[
    ComponentSpec::new("value", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "contextList",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Attribute_valuesWithContext_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Attribute_valuesWithContext_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_Attribute_valuesWithContext_Item(
    el: &X690Element,
) -> ASN1Result<Attribute_valuesWithContext_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "Attribute-valuesWithContext-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Attribute_valuesWithContext_Item,
        _eal_components_for_Attribute_valuesWithContext_Item,
        _rctl2_components_for_Attribute_valuesWithContext_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut value_: OPTIONAL<X690Element> = None;
    let mut contextList_: OPTIONAL<Vec<Context>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "value" => value_ = Some(x690_identity(_el)?),
            "contextList" => {
                contextList_ = Some(|el: &X690Element| -> ASN1Result<SET_OF<Context>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "contextList",
                            ))
                        }
                    };
                    let mut items: SET_OF<Context> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_Context(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Attribute_valuesWithContext_Item {
        value: value_.unwrap(),
        contextList: contextList_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_Attribute_valuesWithContext_Item(
    value_: &Attribute_valuesWithContext_Item,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(x690_identity(&value_.value)?);
    components_.push(|value_: &SET_OF<Context>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Context(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }(&value_.contextList)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Attribute_valuesWithContext_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "Attribute-valuesWithContext-Item",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Attribute_valuesWithContext_Item,
        _eal_components_for_Attribute_valuesWithContext_Item,
        _rctl2_components_for_Attribute_valuesWithContext_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "value" => BER.validate_any(_el)?,
            "contextList" => {
                |el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_Context(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(el
                            .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "contextList")),
                    }
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
/// AttributeValueAssertion-assertedContexts ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum AttributeValueAssertion_assertedContexts {
    allContexts(NULL),
    selectedContexts(Vec<ContextAssertion>),
}

impl TryFrom<&X690Element> for AttributeValueAssertion_assertedContexts {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeValueAssertion_assertedContexts(el)
    }
}

pub fn _decode_AttributeValueAssertion_assertedContexts(
    el: &X690Element,
) -> ASN1Result<AttributeValueAssertion_assertedContexts> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(AttributeValueAssertion_assertedContexts::allContexts(
            |el: &X690Element| -> ASN1Result<NULL> { Ok(BER.decode_null(&el.inner()?)?) }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(AttributeValueAssertion_assertedContexts::selectedContexts(
            |el: &X690Element| -> ASN1Result<Vec<ContextAssertion>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<ContextAssertion>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "selectedContexts",
                            ))
                        }
                    };
                    let mut items: SET_OF<ContextAssertion> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_ContextAssertion(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "AttributeValueAssertion-assertedContexts",
            ))
        }
    }
}

pub fn _encode_AttributeValueAssertion_assertedContexts(
    value_: &AttributeValueAssertion_assertedContexts,
) -> ASN1Result<X690Element> {
    match value_ {
        AttributeValueAssertion_assertedContexts::allContexts(v) => {
            |v_1: &NULL| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(BER.encode_null(&v_1)?),
                ))
            }(&v)
        }
        AttributeValueAssertion_assertedContexts::selectedContexts(v) => {
            |v_1: &Vec<ContextAssertion>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
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
            }(&v)
        }
    }
}

pub fn _validate_AttributeValueAssertion_assertedContexts(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "allContexts")
                );
            }
            Ok(BER.validate_null(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "selectedContexts")
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
                    _ => Err(el.to_asn1_err_named(
                        ASN1ErrorCode::invalid_construction,
                        "selectedContexts",
                    )),
                }
            }(&el.inner()?)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "AttributeValueAssertion-assertedContexts",
            ))
        }
    }
}
