#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # SchemaAdministration
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `SchemaAdministration`.
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
use crate::InformationFramework::*;
use crate::LdapSystemSchema::*;
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// subschema OBJECT-CLASS ::= {
///   KIND        auxiliary
///   MAY CONTAIN { dITStructureRules |
///                 nameForms |
///                 dITContentRules |
///                 objectClasses |
///                 attributeTypes |
///                 friends |
///                 contextTypes |
///                 dITContextUse |
///                 matchingRules |
///                 matchingRuleUse |
///                 ldapSyntaxes }
///   LDAP-NAME   {"subschema"}
///   ID          id-soc-subschema }
/// ```
///
///
pub fn subschema() -> OBJECT_CLASS {
    OBJECT_CLASS {
        kind: Some(ObjectClassKind_auxiliary), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::<_>::from([
            dITStructureRules(),
            nameForms(),
            dITContentRules(),
            objectClasses(),
            attributeTypes(),
            friends(),
            contextTypes(),
            dITContextUse(),
            matchingRules(),
            matchingRuleUse(),
            ldapSyntaxes(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("subschema")])), /* OBJECT_FIELD_SETTING */
        id: id_soc_subschema(),                /* OBJECT_FIELD_SETTING */
        Superclasses: None,
        MandatoryAttributes: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dITStructureRules ATTRIBUTE ::= {
///   WITH SYNTAX              DITStructureRuleDescription
///   EQUALITY MATCHING RULE   integerFirstComponentMatch
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              dITStructureRuleDescription.&id
///   LDAP-NAME                {"dITStructureRules"}
///   ID                       id-soa-dITStructureRule }
/// ```
///
///
pub fn dITStructureRules() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),               /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dITStructureRuleDescription().id),           /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("dITStructureRules")])), /* OBJECT_FIELD_SETTING */
        id: id_soa_dITStructureRule(),                                /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// DITStructureRuleDescription ::= SEQUENCE {
///   COMPONENTS OF DITStructureRule,
///   name         [1]  SET SIZE (1..MAX) OF UnboundedDirectoryString OPTIONAL,
///   description       UnboundedDirectoryString OPTIONAL,
///   obsolete          BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DITStructureRuleDescription {
    pub ruleIdentifier: RuleIdentifier, /* REPLICATED_COMPONENT */
    pub nameForm: OBJECT_IDENTIFIER,    /* REPLICATED_COMPONENT */
    pub superiorStructureRules: OPTIONAL<Vec<RuleIdentifier>>, /* REPLICATED_COMPONENT */
    pub name: OPTIONAL<Vec<UnboundedDirectoryString>>,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl DITStructureRuleDescription {
    pub fn new(
        ruleIdentifier: RuleIdentifier, /* REPLICATED_COMPONENT */
        nameForm: OBJECT_IDENTIFIER,    /* REPLICATED_COMPONENT */
        superiorStructureRules: OPTIONAL<Vec<RuleIdentifier>>, /* REPLICATED_COMPONENT */
        name: OPTIONAL<Vec<UnboundedDirectoryString>>,
        description: OPTIONAL<UnboundedDirectoryString>,
        obsolete: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DITStructureRuleDescription {
            ruleIdentifier,
            nameForm,
            superiorStructureRules,
            name,
            description,
            obsolete,
            _unrecognized,
        }
    }
    pub fn _default_value_for_obsolete() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for DITStructureRuleDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DITStructureRuleDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DITStructureRuleDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DITStructureRuleDescription(el)
    }
}

pub const _rctl1_components_for_DITStructureRuleDescription: &[ComponentSpec; 6] = &[
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
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "obsolete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DITStructureRuleDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DITStructureRuleDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_DITStructureRuleDescription(
    el: &X690Element,
) -> ASN1Result<DITStructureRuleDescription> {
    |el_: &X690Element| -> ASN1Result<DITStructureRuleDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DITStructureRuleDescription,
            _eal_components_for_DITStructureRuleDescription,
            _rctl2_components_for_DITStructureRuleDescription,
        )?;
        let ruleIdentifier = _decode_RuleIdentifier(_components.get("ruleIdentifier").unwrap())?;
        let nameForm = ber_decode_object_identifier(_components.get("nameForm").unwrap())?;
        let superiorStructureRules: OPTIONAL<Vec<RuleIdentifier>> =
            match _components.get("superiorStructureRules") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SET_OF<RuleIdentifier>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<RuleIdentifier> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_RuleIdentifier(el)?);
                    }
                    Ok(items)
                }(c_)?),
                _ => None,
            };
        let name: OPTIONAL<Vec<UnboundedDirectoryString>> =
            match _components.get("name") {
                Some(c_) => {
                    Some(
                        |el: &X690Element| -> ASN1Result<Vec<UnboundedDirectoryString>> {
                            Ok(|el: &X690Element| -> ASN1Result<SET_OF<UnboundedDirectoryString>> {
	let elements = match el.value.borrow() {
		X690Encoding::Constructed(children) => children,
		_ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
	};
	let mut items: SET_OF<UnboundedDirectoryString> = Vec::with_capacity(elements.len());
	for el in elements {
		items.push(_decode_UnboundedDirectoryString(el)?);
	}
	Ok(items)
}(&el.inner()?)?)
                        }(c_)?,
                    )
                }
                _ => None,
            };
        let description: OPTIONAL<UnboundedDirectoryString> = match _components.get("description") {
            Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
            _ => None,
        };
        let obsolete: OPTIONAL<BOOLEAN> = match _components.get("obsolete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        Ok(DITStructureRuleDescription {
            ruleIdentifier,
            nameForm,
            superiorStructureRules,
            name,
            description,
            obsolete,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DITStructureRuleDescription(
    value_: &DITStructureRuleDescription,
) -> ASN1Result<X690Element> {
    |value_: &DITStructureRuleDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(16);
        components_.push(_encode_RuleIdentifier(&value_.ruleIdentifier)?);
        components_.push(ber_encode_object_identifier(&value_.nameForm)?);
        if let Some(v_) = &value_.superiorStructureRules {
            components_.push(
                |value_: &SET_OF<RuleIdentifier>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_RuleIdentifier(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.name {
            components_.push(|v_1: &Vec<UnboundedDirectoryString>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 1, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_UnboundedDirectoryString(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
        }
        if let Some(v_) = &value_.description {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        if let Some(v_) = &value_.obsolete {
            if *v_ != DITStructureRuleDescription::_default_value_for_obsolete() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dITContentRules ATTRIBUTE ::= {
///   WITH SYNTAX              DITContentRuleDescription
///   EQUALITY MATCHING RULE   objectIdentifierFirstComponentMatch
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              dITContentRuleDescription.&id
///   LDAP-NAME                {"dITContentRules"}
///   ID                       id-soa-dITContentRules }
/// ```
///
///
pub fn dITContentRules() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dITContentRuleDescription().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("dITContentRules")])), /* OBJECT_FIELD_SETTING */
        id: id_soa_dITContentRules(),                   /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// DITContentRuleDescription ::= SEQUENCE {
///   COMPONENTS OF DITContentRule,
///   name         [4]  SET SIZE (1..MAX) OF UnboundedDirectoryString OPTIONAL,
///   description       UnboundedDirectoryString OPTIONAL,
///   obsolete          BOOLEAN DEFAULT FALSE,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DITContentRuleDescription {
    pub structuralObjectClass: OBJECT_IDENTIFIER, /* REPLICATED_COMPONENT */
    pub auxiliaries: OPTIONAL<Vec<OBJECT_IDENTIFIER>>, /* REPLICATED_COMPONENT */
    pub mandatory: OPTIONAL<Vec<OBJECT_IDENTIFIER>>, /* REPLICATED_COMPONENT */
    pub optional: OPTIONAL<Vec<OBJECT_IDENTIFIER>>, /* REPLICATED_COMPONENT */
    pub precluded: OPTIONAL<Vec<OBJECT_IDENTIFIER>>, /* REPLICATED_COMPONENT */
    pub name: OPTIONAL<Vec<UnboundedDirectoryString>>,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl DITContentRuleDescription {
    pub fn new(
        structuralObjectClass: OBJECT_IDENTIFIER, /* REPLICATED_COMPONENT */
        auxiliaries: OPTIONAL<Vec<OBJECT_IDENTIFIER>>, /* REPLICATED_COMPONENT */
        mandatory: OPTIONAL<Vec<OBJECT_IDENTIFIER>>, /* REPLICATED_COMPONENT */
        optional: OPTIONAL<Vec<OBJECT_IDENTIFIER>>, /* REPLICATED_COMPONENT */
        precluded: OPTIONAL<Vec<OBJECT_IDENTIFIER>>, /* REPLICATED_COMPONENT */
        name: OPTIONAL<Vec<UnboundedDirectoryString>>,
        description: OPTIONAL<UnboundedDirectoryString>,
        obsolete: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DITContentRuleDescription {
            structuralObjectClass,
            auxiliaries,
            mandatory,
            optional,
            precluded,
            name,
            description,
            obsolete,
            _unrecognized,
        }
    }
    pub fn _default_value_for_obsolete() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for DITContentRuleDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DITContentRuleDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DITContentRuleDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DITContentRuleDescription(el)
    }
}

pub const _rctl1_components_for_DITContentRuleDescription: &[ComponentSpec; 8] = &[
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
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "obsolete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DITContentRuleDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DITContentRuleDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_DITContentRuleDescription(
    el: &X690Element,
) -> ASN1Result<DITContentRuleDescription> {
    |el_: &X690Element| -> ASN1Result<DITContentRuleDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DITContentRuleDescription,
            _eal_components_for_DITContentRuleDescription,
            _rctl2_components_for_DITContentRuleDescription,
        )?;
        let structuralObjectClass =
            ber_decode_object_identifier(_components.get("structuralObjectClass").unwrap())?;
        let auxiliaries: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components.get("auxiliaries") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<OBJECT_IDENTIFIER> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(ber_decode_object_identifier(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        let mandatory: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components.get("mandatory") {
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
        let optional: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components.get("optional") {
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
        let precluded: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components.get("precluded") {
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
        let name: OPTIONAL<Vec<UnboundedDirectoryString>> =
            match _components.get("name") {
                Some(c_) => {
                    Some(
                        |el: &X690Element| -> ASN1Result<Vec<UnboundedDirectoryString>> {
                            Ok(|el: &X690Element| -> ASN1Result<SET_OF<UnboundedDirectoryString>> {
	let elements = match el.value.borrow() {
		X690Encoding::Constructed(children) => children,
		_ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
	};
	let mut items: SET_OF<UnboundedDirectoryString> = Vec::with_capacity(elements.len());
	for el in elements {
		items.push(_decode_UnboundedDirectoryString(el)?);
	}
	Ok(items)
}(&el.inner()?)?)
                        }(c_)?,
                    )
                }
                _ => None,
            };
        let description: OPTIONAL<UnboundedDirectoryString> = match _components.get("description") {
            Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
            _ => None,
        };
        let obsolete: OPTIONAL<BOOLEAN> = match _components.get("obsolete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        Ok(DITContentRuleDescription {
            structuralObjectClass,
            auxiliaries,
            mandatory,
            optional,
            precluded,
            name,
            description,
            obsolete,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DITContentRuleDescription(
    value_: &DITContentRuleDescription,
) -> ASN1Result<X690Element> {
    |value_: &DITContentRuleDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(18);
        components_.push(ber_encode_object_identifier(&value_.structuralObjectClass)?);
        if let Some(v_) = &value_.auxiliaries {
            components_.push(
                |value_: &SET_OF<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(ber_encode_object_identifier(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.mandatory {
            components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
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
        if let Some(v_) = &value_.optional {
            components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
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
        if let Some(v_) = &value_.precluded {
            components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
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
        if let Some(v_) = &value_.name {
            components_.push(|v_1: &Vec<UnboundedDirectoryString>| -> ASN1Result<X690Element> { Ok(X690Element::new(TagClass::CONTEXT, 4, Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
	let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
	for v in value_ {
		children.push(_encode_UnboundedDirectoryString(&v)?);
	}
	Ok(X690Element::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF, Arc::new(X690Encoding::Constructed(children))))
}(&v_1)?))))) }(&v_)?);
        }
        if let Some(v_) = &value_.description {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        if let Some(v_) = &value_.obsolete {
            if *v_ != DITContentRuleDescription::_default_value_for_obsolete() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// matchingRules ATTRIBUTE ::= {
///   WITH SYNTAX              MatchingRuleDescription
///   EQUALITY MATCHING RULE   objectIdentifierFirstComponentMatch
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              matchingRuleDescription.&id
///   LDAP-NAME                {"matchingRules"}
///   ID                       id-soa-matchingRules }
/// ```
///
///
pub fn matchingRules() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(matchingRuleDescription().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("matchingRules")])), /* OBJECT_FIELD_SETTING */
        id: id_soa_matchingRules(),                     /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// MatchingRuleDescription ::= SEQUENCE {
///   identifier        MATCHING-RULE.&id,
///   name              SET SIZE (1..MAX) OF UnboundedDirectoryString OPTIONAL,
///   description       UnboundedDirectoryString OPTIONAL,
///   obsolete          BOOLEAN DEFAULT FALSE,
///   information  [0]  UnboundedDirectoryString OPTIONAL,
///                 -- describes the ASN.1 syntax
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct MatchingRuleDescription {
    pub identifier: OBJECT_IDENTIFIER,
    pub name: OPTIONAL<Vec<UnboundedDirectoryString>>,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub information: OPTIONAL<UnboundedDirectoryString>,
    pub _unrecognized: Vec<X690Element>,
}
impl MatchingRuleDescription {
    pub fn new(
        identifier: OBJECT_IDENTIFIER,
        name: OPTIONAL<Vec<UnboundedDirectoryString>>,
        description: OPTIONAL<UnboundedDirectoryString>,
        obsolete: OPTIONAL<BOOLEAN>,
        information: OPTIONAL<UnboundedDirectoryString>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        MatchingRuleDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        }
    }
    pub fn _default_value_for_obsolete() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for MatchingRuleDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_MatchingRuleDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for MatchingRuleDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_MatchingRuleDescription(el)
    }
}

pub const _rctl1_components_for_MatchingRuleDescription: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "obsolete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "information",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_MatchingRuleDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MatchingRuleDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_MatchingRuleDescription(el: &X690Element) -> ASN1Result<MatchingRuleDescription> {
    |el_: &X690Element| -> ASN1Result<MatchingRuleDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_MatchingRuleDescription,
            _eal_components_for_MatchingRuleDescription,
            _rctl2_components_for_MatchingRuleDescription,
        )?;
        let identifier = ber_decode_object_identifier(_components.get("identifier").unwrap())?;
        let name: OPTIONAL<Vec<UnboundedDirectoryString>> = match _components.get("name") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SET_OF<UnboundedDirectoryString>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<UnboundedDirectoryString> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_UnboundedDirectoryString(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        let description: OPTIONAL<UnboundedDirectoryString> = match _components.get("description") {
            Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
            _ => None,
        };
        let obsolete: OPTIONAL<BOOLEAN> = match _components.get("obsolete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let information: OPTIONAL<UnboundedDirectoryString> = match _components.get("information") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
                Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(MatchingRuleDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_MatchingRuleDescription(
    value_: &MatchingRuleDescription,
) -> ASN1Result<X690Element> {
    |value_: &MatchingRuleDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(ber_encode_object_identifier(&value_.identifier)?);
        if let Some(v_) = &value_.name {
            components_.push(
                |value_: &SET_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_UnboundedDirectoryString(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.description {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        if let Some(v_) = &value_.obsolete {
            if *v_ != MatchingRuleDescription::_default_value_for_obsolete() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        if let Some(v_) = &value_.information {
            components_.push(
                |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_UnboundedDirectoryString(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeTypes ATTRIBUTE ::= {
///   WITH SYNTAX              AttributeTypeDescription
///   EQUALITY MATCHING RULE   objectIdentifierFirstComponentMatch
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              attributeTypeDescription.&id
///   LDAP-NAME                {"attributeTypes"}
///   ID                       id-soa-attributeTypes }
/// ```
///
///
pub fn attributeTypes() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(attributeTypeDescription().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("attributeTypes")])), /* OBJECT_FIELD_SETTING */
        id: id_soa_attributeTypes(),                    /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeTypeDescription ::= SEQUENCE {
///   identifier        ATTRIBUTE.&id,
///   name              SET SIZE (1..MAX) OF UnboundedDirectoryString OPTIONAL,
///   description       UnboundedDirectoryString OPTIONAL,
///   obsolete          BOOLEAN DEFAULT FALSE,
///   information  [0]  AttributeTypeInformation,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeTypeDescription {
    pub identifier: OBJECT_IDENTIFIER,
    pub name: OPTIONAL<Vec<UnboundedDirectoryString>>,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub information: AttributeTypeInformation,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeTypeDescription {
    pub fn new(
        identifier: OBJECT_IDENTIFIER,
        name: OPTIONAL<Vec<UnboundedDirectoryString>>,
        description: OPTIONAL<UnboundedDirectoryString>,
        obsolete: OPTIONAL<BOOLEAN>,
        information: AttributeTypeInformation,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeTypeDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        }
    }
    pub fn _default_value_for_obsolete() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for AttributeTypeDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeTypeDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeTypeDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeTypeDescription(el)
    }
}

pub const _rctl1_components_for_AttributeTypeDescription: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "obsolete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "information",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeTypeDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeTypeDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeTypeDescription(el: &X690Element) -> ASN1Result<AttributeTypeDescription> {
    |el_: &X690Element| -> ASN1Result<AttributeTypeDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeTypeDescription,
            _eal_components_for_AttributeTypeDescription,
            _rctl2_components_for_AttributeTypeDescription,
        )?;
        let identifier = ber_decode_object_identifier(_components.get("identifier").unwrap())?;
        let name: OPTIONAL<Vec<UnboundedDirectoryString>> = match _components.get("name") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SET_OF<UnboundedDirectoryString>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<UnboundedDirectoryString> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_UnboundedDirectoryString(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        let description: OPTIONAL<UnboundedDirectoryString> = match _components.get("description") {
            Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
            _ => None,
        };
        let obsolete: OPTIONAL<BOOLEAN> = match _components.get("obsolete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let information = |el: &X690Element| -> ASN1Result<AttributeTypeInformation> {
            Ok(_decode_AttributeTypeInformation(&el.inner()?)?)
        }(_components.get("information").unwrap())?;
        Ok(AttributeTypeDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeTypeDescription(
    value_: &AttributeTypeDescription,
) -> ASN1Result<X690Element> {
    |value_: &AttributeTypeDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(ber_encode_object_identifier(&value_.identifier)?);
        if let Some(v_) = &value_.name {
            components_.push(
                |value_: &SET_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_UnboundedDirectoryString(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.description {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        if let Some(v_) = &value_.obsolete {
            if *v_ != AttributeTypeDescription::_default_value_for_obsolete() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        components_.push(
            |v_1: &AttributeTypeInformation| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_AttributeTypeInformation(&v_1)?,
                    ))),
                ))
            }(&value_.information)?,
        );
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeTypeInformation ::= SEQUENCE {
///   derivation       [0]  ATTRIBUTE.&id             OPTIONAL,
///   equalityMatch    [1]  MATCHING-RULE.&id         OPTIONAL,
///   orderingMatch    [2]  MATCHING-RULE.&id         OPTIONAL,
///   substringsMatch  [3]  MATCHING-RULE.&id         OPTIONAL,
///   attributeSyntax  [4]  UnboundedDirectoryString  OPTIONAL,
///   multi-valued     [5]  BOOLEAN                   DEFAULT TRUE,
///   collective       [6]  BOOLEAN                   DEFAULT FALSE,
///   userModifiable   [7]  BOOLEAN                   DEFAULT TRUE,
///   application           AttributeUsage            DEFAULT userApplications,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeTypeInformation {
    pub derivation: OPTIONAL<OBJECT_IDENTIFIER>,
    pub equalityMatch: OPTIONAL<OBJECT_IDENTIFIER>,
    pub orderingMatch: OPTIONAL<OBJECT_IDENTIFIER>,
    pub substringsMatch: OPTIONAL<OBJECT_IDENTIFIER>,
    pub attributeSyntax: OPTIONAL<UnboundedDirectoryString>,
    pub multi_valued: OPTIONAL<BOOLEAN>,
    pub collective: OPTIONAL<BOOLEAN>,
    pub userModifiable: OPTIONAL<BOOLEAN>,
    pub application: OPTIONAL<AttributeUsage>,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeTypeInformation {
    pub fn new(
        derivation: OPTIONAL<OBJECT_IDENTIFIER>,
        equalityMatch: OPTIONAL<OBJECT_IDENTIFIER>,
        orderingMatch: OPTIONAL<OBJECT_IDENTIFIER>,
        substringsMatch: OPTIONAL<OBJECT_IDENTIFIER>,
        attributeSyntax: OPTIONAL<UnboundedDirectoryString>,
        multi_valued: OPTIONAL<BOOLEAN>,
        collective: OPTIONAL<BOOLEAN>,
        userModifiable: OPTIONAL<BOOLEAN>,
        application: OPTIONAL<AttributeUsage>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeTypeInformation {
            derivation,
            equalityMatch,
            orderingMatch,
            substringsMatch,
            attributeSyntax,
            multi_valued,
            collective,
            userModifiable,
            application,
            _unrecognized,
        }
    }
    pub fn _default_value_for_multi_valued() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_collective() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_userModifiable() -> BOOLEAN {
        true
    }
    pub fn _default_value_for_application() -> AttributeUsage {
        AttributeUsage_userApplications
    }
}
impl Default for AttributeTypeInformation {
    fn default() -> Self {
        AttributeTypeInformation {
            derivation: None,
            equalityMatch: None,
            orderingMatch: None,
            substringsMatch: None,
            attributeSyntax: None,
            multi_valued: None,
            collective: None,
            userModifiable: None,
            application: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for AttributeTypeInformation {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeTypeInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeTypeInformation {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeTypeInformation(el)
    }
}

pub const _rctl1_components_for_AttributeTypeInformation: &[ComponentSpec; 9] = &[
    ComponentSpec::new(
        "derivation",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "equalityMatch",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "orderingMatch",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "substringsMatch",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeSyntax",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "multi-valued",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "collective",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "userModifiable",
        true,
        TagSelector::tag((TagClass::CONTEXT, 7)),
        None,
        None,
    ),
    ComponentSpec::new(
        "application",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeTypeInformation: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeTypeInformation: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeTypeInformation(el: &X690Element) -> ASN1Result<AttributeTypeInformation> {
    |el_: &X690Element| -> ASN1Result<AttributeTypeInformation> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeTypeInformation,
            _eal_components_for_AttributeTypeInformation,
            _rctl2_components_for_AttributeTypeInformation,
        )?;
        let derivation: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("derivation") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(ber_decode_object_identifier(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let equalityMatch: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("equalityMatch") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(ber_decode_object_identifier(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let orderingMatch: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("orderingMatch") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(ber_decode_object_identifier(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let substringsMatch: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("substringsMatch")
        {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(ber_decode_object_identifier(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let attributeSyntax: OPTIONAL<UnboundedDirectoryString> =
            match _components.get("attributeSyntax") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
                    Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let multi_valued: OPTIONAL<BOOLEAN> = match _components.get("multi-valued") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let collective: OPTIONAL<BOOLEAN> = match _components.get("collective") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let userModifiable: OPTIONAL<BOOLEAN> = match _components.get("userModifiable") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let application: OPTIONAL<AttributeUsage> = match _components.get("application") {
            Some(c_) => Some(_decode_AttributeUsage(c_)?),
            _ => None,
        };
        Ok(AttributeTypeInformation {
            derivation,
            equalityMatch,
            orderingMatch,
            substringsMatch,
            attributeSyntax,
            multi_valued,
            collective,
            userModifiable,
            application,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeTypeInformation(
    value_: &AttributeTypeInformation,
) -> ASN1Result<X690Element> {
    |value_: &AttributeTypeInformation| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(19);
        if let Some(v_) = &value_.derivation {
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
        if let Some(v_) = &value_.equalityMatch {
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
        if let Some(v_) = &value_.orderingMatch {
            components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_object_identifier(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.substringsMatch {
            components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_object_identifier(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.attributeSyntax {
            components_.push(
                |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        4,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_UnboundedDirectoryString(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.multi_valued {
            if *v_ != AttributeTypeInformation::_default_value_for_multi_valued() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        5,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.collective {
            if *v_ != AttributeTypeInformation::_default_value_for_collective() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        6,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.userModifiable {
            if *v_ != AttributeTypeInformation::_default_value_for_userModifiable() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        7,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.application {
            if *v_ != AttributeTypeInformation::_default_value_for_application() {
                components_.push(_encode_AttributeUsage(&v_)?);
            }
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// objectClasses ATTRIBUTE ::= {
///   WITH SYNTAX              ObjectClassDescription
///   EQUALITY MATCHING RULE   objectIdentifierFirstComponentMatch
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              objectClassDescription.&id
///   LDAP-NAME                {"objectClasses"}
///   ID                       id-soa-objectClasses }
/// ```
///
///
pub fn objectClasses() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(objectClassDescription().id),  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("objectClasses")])), /* OBJECT_FIELD_SETTING */
        id: id_soa_objectClasses(),                     /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// ObjectClassDescription ::= SEQUENCE {
///   identifier        OBJECT-CLASS.&id,
///   name              SET SIZE (1..MAX) OF UnboundedDirectoryString OPTIONAL,
///   description       UnboundedDirectoryString                      OPTIONAL,
///   obsolete          BOOLEAN                                       DEFAULT FALSE,
///   information  [0]  ObjectClassInformation,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ObjectClassDescription {
    pub identifier: OBJECT_IDENTIFIER,
    pub name: OPTIONAL<Vec<UnboundedDirectoryString>>,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub information: ObjectClassInformation,
    pub _unrecognized: Vec<X690Element>,
}
impl ObjectClassDescription {
    pub fn new(
        identifier: OBJECT_IDENTIFIER,
        name: OPTIONAL<Vec<UnboundedDirectoryString>>,
        description: OPTIONAL<UnboundedDirectoryString>,
        obsolete: OPTIONAL<BOOLEAN>,
        information: ObjectClassInformation,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ObjectClassDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        }
    }
    pub fn _default_value_for_obsolete() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for ObjectClassDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ObjectClassDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ObjectClassDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ObjectClassDescription(el)
    }
}

pub const _rctl1_components_for_ObjectClassDescription: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "obsolete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "information",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ObjectClassDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ObjectClassDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_ObjectClassDescription(el: &X690Element) -> ASN1Result<ObjectClassDescription> {
    |el_: &X690Element| -> ASN1Result<ObjectClassDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ObjectClassDescription,
            _eal_components_for_ObjectClassDescription,
            _rctl2_components_for_ObjectClassDescription,
        )?;
        let identifier = ber_decode_object_identifier(_components.get("identifier").unwrap())?;
        let name: OPTIONAL<Vec<UnboundedDirectoryString>> = match _components.get("name") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SET_OF<UnboundedDirectoryString>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<UnboundedDirectoryString> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_UnboundedDirectoryString(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        let description: OPTIONAL<UnboundedDirectoryString> = match _components.get("description") {
            Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
            _ => None,
        };
        let obsolete: OPTIONAL<BOOLEAN> = match _components.get("obsolete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let information = |el: &X690Element| -> ASN1Result<ObjectClassInformation> {
            Ok(_decode_ObjectClassInformation(&el.inner()?)?)
        }(_components.get("information").unwrap())?;
        Ok(ObjectClassDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ObjectClassDescription(value_: &ObjectClassDescription) -> ASN1Result<X690Element> {
    |value_: &ObjectClassDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(ber_encode_object_identifier(&value_.identifier)?);
        if let Some(v_) = &value_.name {
            components_.push(
                |value_: &SET_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_UnboundedDirectoryString(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.description {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        if let Some(v_) = &value_.obsolete {
            if *v_ != ObjectClassDescription::_default_value_for_obsolete() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        components_.push(|v_1: &ObjectClassInformation| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_ObjectClassInformation(&v_1)?,
                ))),
            ))
        }(&value_.information)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ObjectClassInformation ::= SEQUENCE {
///   subclassOf        SET SIZE (1..MAX) OF OBJECT-CLASS.&id OPTIONAL,
///   kind              ObjectClassKind                       DEFAULT structural,
///   mandatories  [3]  SET SIZE (1..MAX) OF ATTRIBUTE.&id    OPTIONAL,
///   optionals    [4]  SET SIZE (1..MAX) OF ATTRIBUTE.&id    OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ObjectClassInformation {
    pub subclassOf: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub kind: OPTIONAL<ObjectClassKind>,
    pub mandatories: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub optionals: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub _unrecognized: Vec<X690Element>,
}
impl ObjectClassInformation {
    pub fn new(
        subclassOf: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        kind: OPTIONAL<ObjectClassKind>,
        mandatories: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        optionals: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ObjectClassInformation {
            subclassOf,
            kind,
            mandatories,
            optionals,
            _unrecognized,
        }
    }
    pub fn _default_value_for_kind() -> ObjectClassKind {
        ObjectClassKind_structural
    }
}
impl Default for ObjectClassInformation {
    fn default() -> Self {
        ObjectClassInformation {
            subclassOf: None,
            kind: None,
            mandatories: None,
            optionals: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for ObjectClassInformation {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ObjectClassInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ObjectClassInformation {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ObjectClassInformation(el)
    }
}

pub const _rctl1_components_for_ObjectClassInformation: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "subclassOf",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "kind",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new(
        "mandatories",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "optionals",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ObjectClassInformation: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ObjectClassInformation: &[ComponentSpec; 0] = &[];

pub fn _decode_ObjectClassInformation(el: &X690Element) -> ASN1Result<ObjectClassInformation> {
    |el_: &X690Element| -> ASN1Result<ObjectClassInformation> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ObjectClassInformation,
            _eal_components_for_ObjectClassInformation,
            _rctl2_components_for_ObjectClassInformation,
        )?;
        let subclassOf: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components.get("subclassOf") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<OBJECT_IDENTIFIER> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(ber_decode_object_identifier(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        let kind: OPTIONAL<ObjectClassKind> = match _components.get("kind") {
            Some(c_) => Some(_decode_ObjectClassKind(c_)?),
            _ => None,
        };
        let mandatories: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components.get("mandatories") {
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
        let optionals: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components.get("optionals") {
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
        Ok(ObjectClassInformation {
            subclassOf,
            kind,
            mandatories,
            optionals,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ObjectClassInformation(value_: &ObjectClassInformation) -> ASN1Result<X690Element> {
    |value_: &ObjectClassInformation| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        if let Some(v_) = &value_.subclassOf {
            components_.push(
                |value_: &SET_OF<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(ber_encode_object_identifier(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.kind {
            if *v_ != ObjectClassInformation::_default_value_for_kind() {
                components_.push(_encode_ObjectClassKind(&v_)?);
            }
        }
        if let Some(v_) = &value_.mandatories {
            components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
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
        if let Some(v_) = &value_.optionals {
            components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
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
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nameForms ATTRIBUTE ::= {
///   WITH SYNTAX              NameFormDescription
///   EQUALITY MATCHING RULE   objectIdentifierFirstComponentMatch
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              nameFormDescription.&id
///   LDAP-NAME                {"nameForms"}
///   ID                       id-soa-nameForms }
/// ```
///
///
pub fn nameForms() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(nameFormDescription().id),     /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("nameForms")])), /* OBJECT_FIELD_SETTING */
        id: id_soa_nameForms(),                         /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// NameFormDescription ::= SEQUENCE {
///   identifier        NAME-FORM.&id,
///   name              SET SIZE (1..MAX) OF UnboundedDirectoryString OPTIONAL,
///   description       UnboundedDirectoryString                      OPTIONAL,
///   obsolete          BOOLEAN                                       DEFAULT FALSE,
///   information  [0]  NameFormInformation,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct NameFormDescription {
    pub identifier: OBJECT_IDENTIFIER,
    pub name: OPTIONAL<Vec<UnboundedDirectoryString>>,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub information: NameFormInformation,
    pub _unrecognized: Vec<X690Element>,
}
impl NameFormDescription {
    pub fn new(
        identifier: OBJECT_IDENTIFIER,
        name: OPTIONAL<Vec<UnboundedDirectoryString>>,
        description: OPTIONAL<UnboundedDirectoryString>,
        obsolete: OPTIONAL<BOOLEAN>,
        information: NameFormInformation,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        NameFormDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        }
    }
    pub fn _default_value_for_obsolete() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for NameFormDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_NameFormDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for NameFormDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_NameFormDescription(el)
    }
}

pub const _rctl1_components_for_NameFormDescription: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "obsolete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "information",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_NameFormDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_NameFormDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_NameFormDescription(el: &X690Element) -> ASN1Result<NameFormDescription> {
    |el_: &X690Element| -> ASN1Result<NameFormDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_NameFormDescription,
            _eal_components_for_NameFormDescription,
            _rctl2_components_for_NameFormDescription,
        )?;
        let identifier = ber_decode_object_identifier(_components.get("identifier").unwrap())?;
        let name: OPTIONAL<Vec<UnboundedDirectoryString>> = match _components.get("name") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SET_OF<UnboundedDirectoryString>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<UnboundedDirectoryString> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_UnboundedDirectoryString(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        let description: OPTIONAL<UnboundedDirectoryString> = match _components.get("description") {
            Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
            _ => None,
        };
        let obsolete: OPTIONAL<BOOLEAN> = match _components.get("obsolete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let information = |el: &X690Element| -> ASN1Result<NameFormInformation> {
            Ok(_decode_NameFormInformation(&el.inner()?)?)
        }(_components.get("information").unwrap())?;
        Ok(NameFormDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_NameFormDescription(value_: &NameFormDescription) -> ASN1Result<X690Element> {
    |value_: &NameFormDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(ber_encode_object_identifier(&value_.identifier)?);
        if let Some(v_) = &value_.name {
            components_.push(
                |value_: &SET_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_UnboundedDirectoryString(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.description {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        if let Some(v_) = &value_.obsolete {
            if *v_ != NameFormDescription::_default_value_for_obsolete() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        components_.push(|v_1: &NameFormInformation| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_NameFormInformation(&v_1)?,
                ))),
            ))
        }(&value_.information)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NameFormInformation ::= SEQUENCE {
///   subordinate        OBJECT-CLASS.&id,
///   namingMandatories  SET OF ATTRIBUTE.&id,
///   namingOptionals    SET SIZE (1..MAX) OF ATTRIBUTE.&id OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct NameFormInformation {
    pub subordinate: OBJECT_IDENTIFIER,
    pub namingMandatories: Vec<OBJECT_IDENTIFIER>,
    pub namingOptionals: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub _unrecognized: Vec<X690Element>,
}
impl NameFormInformation {
    pub fn new(
        subordinate: OBJECT_IDENTIFIER,
        namingMandatories: Vec<OBJECT_IDENTIFIER>,
        namingOptionals: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        NameFormInformation {
            subordinate,
            namingMandatories,
            namingOptionals,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for NameFormInformation {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_NameFormInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for NameFormInformation {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_NameFormInformation(el)
    }
}

pub const _rctl1_components_for_NameFormInformation: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "subordinate",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "namingMandatories",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "namingOptionals",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_NameFormInformation: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_NameFormInformation: &[ComponentSpec; 0] = &[];

pub fn _decode_NameFormInformation(el: &X690Element) -> ASN1Result<NameFormInformation> {
    |el_: &X690Element| -> ASN1Result<NameFormInformation> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_NameFormInformation,
            _eal_components_for_NameFormInformation,
            _rctl2_components_for_NameFormInformation,
        )?;
        let subordinate = ber_decode_object_identifier(_components.get("subordinate").unwrap())?;
        let namingMandatories = |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SET_OF<OBJECT_IDENTIFIER> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(ber_decode_object_identifier(el)?);
            }
            Ok(items)
        }(_components.get("namingMandatories").unwrap())?;
        let namingOptionals: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components
            .get("namingOptionals")
        {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<OBJECT_IDENTIFIER> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(ber_decode_object_identifier(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        Ok(NameFormInformation {
            subordinate,
            namingMandatories,
            namingOptionals,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_NameFormInformation(value_: &NameFormInformation) -> ASN1Result<X690Element> {
    |value_: &NameFormInformation| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(ber_encode_object_identifier(&value_.subordinate)?);
        components_.push(
            |value_: &SET_OF<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(ber_encode_object_identifier(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.namingMandatories)?,
        );
        if let Some(v_) = &value_.namingOptionals {
            components_.push(
                |value_: &SET_OF<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(ber_encode_object_identifier(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// matchingRuleUse ATTRIBUTE ::= {
///   WITH SYNTAX              MatchingRuleUseDescription
///   EQUALITY MATCHING RULE   objectIdentifierFirstComponentMatch
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              matchingRuleUseDescription.&id
///   LDAP-NAME                {"matchingRuleUse"}
///   ID                       id-soa-matchingRuleUse }
/// ```
///
///
pub fn matchingRuleUse() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(matchingRuleUseDescription().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("matchingRuleUse")])), /* OBJECT_FIELD_SETTING */
        id: id_soa_matchingRuleUse(),                   /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// MatchingRuleUseDescription ::= SEQUENCE {
///   identifier        MATCHING-RULE.&id,
///   name              SET SIZE (1..MAX) OF UnboundedDirectoryString OPTIONAL,
///   description       UnboundedDirectoryString                      OPTIONAL,
///   obsolete          BOOLEAN                                       DEFAULT FALSE,
///   information  [0]  SET OF ATTRIBUTE.&id,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct MatchingRuleUseDescription {
    pub identifier: OBJECT_IDENTIFIER,
    pub name: OPTIONAL<Vec<UnboundedDirectoryString>>,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub information: Vec<OBJECT_IDENTIFIER>,
    pub _unrecognized: Vec<X690Element>,
}
impl MatchingRuleUseDescription {
    pub fn new(
        identifier: OBJECT_IDENTIFIER,
        name: OPTIONAL<Vec<UnboundedDirectoryString>>,
        description: OPTIONAL<UnboundedDirectoryString>,
        obsolete: OPTIONAL<BOOLEAN>,
        information: Vec<OBJECT_IDENTIFIER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        MatchingRuleUseDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        }
    }
    pub fn _default_value_for_obsolete() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for MatchingRuleUseDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_MatchingRuleUseDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for MatchingRuleUseDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_MatchingRuleUseDescription(el)
    }
}

pub const _rctl1_components_for_MatchingRuleUseDescription: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "obsolete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "information",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_MatchingRuleUseDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MatchingRuleUseDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_MatchingRuleUseDescription(
    el: &X690Element,
) -> ASN1Result<MatchingRuleUseDescription> {
    |el_: &X690Element| -> ASN1Result<MatchingRuleUseDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_MatchingRuleUseDescription,
            _eal_components_for_MatchingRuleUseDescription,
            _rctl2_components_for_MatchingRuleUseDescription,
        )?;
        let identifier = ber_decode_object_identifier(_components.get("identifier").unwrap())?;
        let name: OPTIONAL<Vec<UnboundedDirectoryString>> = match _components.get("name") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SET_OF<UnboundedDirectoryString>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<UnboundedDirectoryString> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_UnboundedDirectoryString(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        let description: OPTIONAL<UnboundedDirectoryString> = match _components.get("description") {
            Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
            _ => None,
        };
        let obsolete: OPTIONAL<BOOLEAN> = match _components.get("obsolete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let information = |el: &X690Element| -> ASN1Result<Vec<OBJECT_IDENTIFIER>> {
            Ok(
                |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<OBJECT_IDENTIFIER> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(ber_decode_object_identifier(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )
        }(_components.get("information").unwrap())?;
        Ok(MatchingRuleUseDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_MatchingRuleUseDescription(
    value_: &MatchingRuleUseDescription,
) -> ASN1Result<X690Element> {
    |value_: &MatchingRuleUseDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(ber_encode_object_identifier(&value_.identifier)?);
        if let Some(v_) = &value_.name {
            components_.push(
                |value_: &SET_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_UnboundedDirectoryString(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.description {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        if let Some(v_) = &value_.obsolete {
            if *v_ != MatchingRuleUseDescription::_default_value_for_obsolete() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
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
                }(&v_1)?))),
            ))
        }(&value_.information)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// structuralObjectClass ATTRIBUTE ::= {
///   WITH SYNTAX             OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE  objectIdentifierMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   LDAP-SYNTAX             oid.&id
///   LDAP-NAME               {"structuralObjectClass"}
///   ID                      id-soa-structuralObjectClass }
/// ```
///
///
pub fn structuralObjectClass() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                               /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),                        /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),          /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),                              /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("structuralObjectClass")])), /* OBJECT_FIELD_SETTING */
        id: id_soa_structuralObjectClass(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// governingStructureRule ATTRIBUTE ::= {
///   WITH SYNTAX             INTEGER
///   EQUALITY MATCHING RULE  integerMatch
///   SINGLE VALUE            TRUE
///   NO USER MODIFICATION    TRUE
///   USAGE                   directoryOperation
///   LDAP-SYNTAX             integer.&id
///   LDAP-NAME               {"governingStructureRule"}
///   ID                      id-soa-governingStructureRule }
/// ```
///
///
pub fn governingStructureRule() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        no_user_modification: Some(true),               /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("governingStructureRule")])), /* OBJECT_FIELD_SETTING */
        id: id_soa_governingStructureRule(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// contextTypes ATTRIBUTE ::= {
///   WITH SYNTAX             ContextDescription
///   EQUALITY MATCHING RULE  objectIdentifierFirstComponentMatch
///   USAGE                   directoryOperation
///   ID                      id-soa-contextTypes }
/// ```
///
///
pub fn contextTypes() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        id: id_soa_contextTypes(),                      /* OBJECT_FIELD_SETTING */
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
/// ContextDescription ::= SEQUENCE {
///   identifier        CONTEXT.&id,
///   name              SET SIZE (1..MAX) OF UnboundedDirectoryString OPTIONAL,
///   description       UnboundedDirectoryString                      OPTIONAL,
///   obsolete          BOOLEAN                                       DEFAULT FALSE,
///   information  [0]  ContextInformation,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ContextDescription {
    pub identifier: OBJECT_IDENTIFIER,
    pub name: OPTIONAL<Vec<UnboundedDirectoryString>>,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub information: ContextInformation,
    pub _unrecognized: Vec<X690Element>,
}
impl ContextDescription {
    pub fn new(
        identifier: OBJECT_IDENTIFIER,
        name: OPTIONAL<Vec<UnboundedDirectoryString>>,
        description: OPTIONAL<UnboundedDirectoryString>,
        obsolete: OPTIONAL<BOOLEAN>,
        information: ContextInformation,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ContextDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        }
    }
    pub fn _default_value_for_obsolete() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for ContextDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ContextDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ContextDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ContextDescription(el)
    }
}

pub const _rctl1_components_for_ContextDescription: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "obsolete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "information",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ContextDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ContextDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_ContextDescription(el: &X690Element) -> ASN1Result<ContextDescription> {
    |el_: &X690Element| -> ASN1Result<ContextDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ContextDescription,
            _eal_components_for_ContextDescription,
            _rctl2_components_for_ContextDescription,
        )?;
        let identifier = ber_decode_object_identifier(_components.get("identifier").unwrap())?;
        let name: OPTIONAL<Vec<UnboundedDirectoryString>> = match _components.get("name") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SET_OF<UnboundedDirectoryString>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<UnboundedDirectoryString> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_UnboundedDirectoryString(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        let description: OPTIONAL<UnboundedDirectoryString> = match _components.get("description") {
            Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
            _ => None,
        };
        let obsolete: OPTIONAL<BOOLEAN> = match _components.get("obsolete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let information = |el: &X690Element| -> ASN1Result<ContextInformation> {
            Ok(_decode_ContextInformation(&el.inner()?)?)
        }(_components.get("information").unwrap())?;
        Ok(ContextDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ContextDescription(value_: &ContextDescription) -> ASN1Result<X690Element> {
    |value_: &ContextDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(ber_encode_object_identifier(&value_.identifier)?);
        if let Some(v_) = &value_.name {
            components_.push(
                |value_: &SET_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_UnboundedDirectoryString(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.description {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        if let Some(v_) = &value_.obsolete {
            if *v_ != ContextDescription::_default_value_for_obsolete() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        components_.push(|v_1: &ContextInformation| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_ContextInformation(&v_1)?,
                ))),
            ))
        }(&value_.information)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ContextInformation ::= SEQUENCE {
///   syntax           UnboundedDirectoryString,
///   assertionSyntax  UnboundedDirectoryString OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ContextInformation {
    pub syntax: UnboundedDirectoryString,
    pub assertionSyntax: OPTIONAL<UnboundedDirectoryString>,
    pub _unrecognized: Vec<X690Element>,
}
impl ContextInformation {
    pub fn new(
        syntax: UnboundedDirectoryString,
        assertionSyntax: OPTIONAL<UnboundedDirectoryString>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ContextInformation {
            syntax,
            assertionSyntax,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for ContextInformation {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ContextInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ContextInformation {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ContextInformation(el)
    }
}

pub const _rctl1_components_for_ContextInformation: &[ComponentSpec; 2] = &[
    ComponentSpec::new("syntax", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "assertionSyntax",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ContextInformation: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ContextInformation: &[ComponentSpec; 0] = &[];

pub fn _decode_ContextInformation(el: &X690Element) -> ASN1Result<ContextInformation> {
    |el_: &X690Element| -> ASN1Result<ContextInformation> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ContextInformation,
            _eal_components_for_ContextInformation,
            _rctl2_components_for_ContextInformation,
        )?;
        let syntax = _decode_UnboundedDirectoryString(_components.get("syntax").unwrap())?;
        let assertionSyntax: OPTIONAL<UnboundedDirectoryString> =
            match _components.get("assertionSyntax") {
                Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
                _ => None,
            };
        Ok(ContextInformation {
            syntax,
            assertionSyntax,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ContextInformation(value_: &ContextInformation) -> ASN1Result<X690Element> {
    |value_: &ContextInformation| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_UnboundedDirectoryString(&value_.syntax)?);
        if let Some(v_) = &value_.assertionSyntax {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dITContextUse ATTRIBUTE ::= {
///   WITH SYNTAX             DITContextUseDescription
///   EQUALITY MATCHING RULE  objectIdentifierFirstComponentMatch
///   USAGE                   directoryOperation
///   ID                      id-soa-dITContextUse }
/// ```
///
///
pub fn dITContextUse() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        id: id_soa_dITContextUse(),                     /* OBJECT_FIELD_SETTING */
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
/// DITContextUseDescription ::= SEQUENCE {
///   identifier        ATTRIBUTE.&id,
///   name              SET SIZE (1..MAX) OF UnboundedDirectoryString OPTIONAL,
///   description       UnboundedDirectoryString OPTIONAL,
///   obsolete          BOOLEAN DEFAULT FALSE,
///   information  [0]  DITContextUseInformation,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DITContextUseDescription {
    pub identifier: OBJECT_IDENTIFIER,
    pub name: OPTIONAL<Vec<UnboundedDirectoryString>>,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub information: DITContextUseInformation,
    pub _unrecognized: Vec<X690Element>,
}
impl DITContextUseDescription {
    pub fn new(
        identifier: OBJECT_IDENTIFIER,
        name: OPTIONAL<Vec<UnboundedDirectoryString>>,
        description: OPTIONAL<UnboundedDirectoryString>,
        obsolete: OPTIONAL<BOOLEAN>,
        information: DITContextUseInformation,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DITContextUseDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        }
    }
    pub fn _default_value_for_obsolete() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for DITContextUseDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DITContextUseDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DITContextUseDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DITContextUseDescription(el)
    }
}

pub const _rctl1_components_for_DITContextUseDescription: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "obsolete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "information",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DITContextUseDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DITContextUseDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_DITContextUseDescription(el: &X690Element) -> ASN1Result<DITContextUseDescription> {
    |el_: &X690Element| -> ASN1Result<DITContextUseDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DITContextUseDescription,
            _eal_components_for_DITContextUseDescription,
            _rctl2_components_for_DITContextUseDescription,
        )?;
        let identifier = ber_decode_object_identifier(_components.get("identifier").unwrap())?;
        let name: OPTIONAL<Vec<UnboundedDirectoryString>> = match _components.get("name") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SET_OF<UnboundedDirectoryString>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<UnboundedDirectoryString> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_UnboundedDirectoryString(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        let description: OPTIONAL<UnboundedDirectoryString> = match _components.get("description") {
            Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
            _ => None,
        };
        let obsolete: OPTIONAL<BOOLEAN> = match _components.get("obsolete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let information = |el: &X690Element| -> ASN1Result<DITContextUseInformation> {
            Ok(_decode_DITContextUseInformation(&el.inner()?)?)
        }(_components.get("information").unwrap())?;
        Ok(DITContextUseDescription {
            identifier,
            name,
            description,
            obsolete,
            information,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DITContextUseDescription(
    value_: &DITContextUseDescription,
) -> ASN1Result<X690Element> {
    |value_: &DITContextUseDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(ber_encode_object_identifier(&value_.identifier)?);
        if let Some(v_) = &value_.name {
            components_.push(
                |value_: &SET_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_UnboundedDirectoryString(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.description {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        if let Some(v_) = &value_.obsolete {
            if *v_ != DITContextUseDescription::_default_value_for_obsolete() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        components_.push(
            |v_1: &DITContextUseInformation| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_DITContextUseInformation(&v_1)?,
                    ))),
                ))
            }(&value_.information)?,
        );
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DITContextUseInformation ::= SEQUENCE {
///   mandatoryContexts  [1]  SET SIZE (1..MAX) OF CONTEXT.&id OPTIONAL,
///   optionalContexts   [2]  SET SIZE (1..MAX) OF CONTEXT.&id OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct DITContextUseInformation {
    pub mandatoryContexts: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub optionalContexts: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
    pub _unrecognized: Vec<X690Element>,
}
impl DITContextUseInformation {
    pub fn new(
        mandatoryContexts: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        optionalContexts: OPTIONAL<Vec<OBJECT_IDENTIFIER>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DITContextUseInformation {
            mandatoryContexts,
            optionalContexts,
            _unrecognized,
        }
    }
}
impl Default for DITContextUseInformation {
    fn default() -> Self {
        DITContextUseInformation {
            mandatoryContexts: None,
            optionalContexts: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for DITContextUseInformation {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DITContextUseInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DITContextUseInformation {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DITContextUseInformation(el)
    }
}

pub const _rctl1_components_for_DITContextUseInformation: &[ComponentSpec; 2] = &[
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

pub const _rctl2_components_for_DITContextUseInformation: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DITContextUseInformation: &[ComponentSpec; 0] = &[];

pub fn _decode_DITContextUseInformation(el: &X690Element) -> ASN1Result<DITContextUseInformation> {
    |el_: &X690Element| -> ASN1Result<DITContextUseInformation> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DITContextUseInformation,
            _eal_components_for_DITContextUseInformation,
            _rctl2_components_for_DITContextUseInformation,
        )?;
        let mandatoryContexts: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components
            .get("mandatoryContexts")
        {
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
        let optionalContexts: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = match _components
            .get("optionalContexts")
        {
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
        Ok(DITContextUseInformation {
            mandatoryContexts,
            optionalContexts,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DITContextUseInformation(
    value_: &DITContextUseInformation,
) -> ASN1Result<X690Element> {
    |value_: &DITContextUseInformation| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.mandatoryContexts {
            components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
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
        if let Some(v_) = &value_.optionalContexts {
            components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
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
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// friends ATTRIBUTE ::= {
///   WITH SYNTAX             FriendsDescription
///   EQUALITY MATCHING RULE  objectIdentifierFirstComponentMatch
///   USAGE                   directoryOperation
///   ID                      id-soa-friends }
/// ```
///
///
pub fn friends() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        id: id_soa_friends(),                           /* OBJECT_FIELD_SETTING */
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
/// FriendsDescription ::= SEQUENCE {
///   anchor            ATTRIBUTE.&id,
///   name              SET SIZE (1..MAX) OF UnboundedDirectoryString OPTIONAL,
///   description       UnboundedDirectoryString OPTIONAL,
///   obsolete          BOOLEAN DEFAULT FALSE,
///   friends      [0]  SET SIZE (1..MAX) OF ATTRIBUTE.&id,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct FriendsDescription {
    pub anchor: OBJECT_IDENTIFIER,
    pub name: OPTIONAL<Vec<UnboundedDirectoryString>>,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub obsolete: OPTIONAL<BOOLEAN>,
    pub friends: Vec<OBJECT_IDENTIFIER>,
    pub _unrecognized: Vec<X690Element>,
}
impl FriendsDescription {
    pub fn new(
        anchor: OBJECT_IDENTIFIER,
        name: OPTIONAL<Vec<UnboundedDirectoryString>>,
        description: OPTIONAL<UnboundedDirectoryString>,
        obsolete: OPTIONAL<BOOLEAN>,
        friends: Vec<OBJECT_IDENTIFIER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        FriendsDescription {
            anchor,
            name,
            description,
            obsolete,
            friends,
            _unrecognized,
        }
    }
    pub fn _default_value_for_obsolete() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for FriendsDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_FriendsDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for FriendsDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_FriendsDescription(el)
    }
}

pub const _rctl1_components_for_FriendsDescription: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "anchor",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "name",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "description",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 20)),
            &TagSelector::tag((TagClass::UNIVERSAL, 19)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "obsolete",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "friends",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_FriendsDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_FriendsDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_FriendsDescription(el: &X690Element) -> ASN1Result<FriendsDescription> {
    |el_: &X690Element| -> ASN1Result<FriendsDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_FriendsDescription,
            _eal_components_for_FriendsDescription,
            _rctl2_components_for_FriendsDescription,
        )?;
        let anchor = ber_decode_object_identifier(_components.get("anchor").unwrap())?;
        let name: OPTIONAL<Vec<UnboundedDirectoryString>> = match _components.get("name") {
            Some(c_) => Some(
                |el: &X690Element| -> ASN1Result<SET_OF<UnboundedDirectoryString>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<UnboundedDirectoryString> =
                        Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_UnboundedDirectoryString(el)?);
                    }
                    Ok(items)
                }(c_)?,
            ),
            _ => None,
        };
        let description: OPTIONAL<UnboundedDirectoryString> = match _components.get("description") {
            Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
            _ => None,
        };
        let obsolete: OPTIONAL<BOOLEAN> = match _components.get("obsolete") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let friends = |el: &X690Element| -> ASN1Result<Vec<OBJECT_IDENTIFIER>> {
            Ok(
                |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<OBJECT_IDENTIFIER> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(ber_decode_object_identifier(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?,
            )
        }(_components.get("friends").unwrap())?;
        Ok(FriendsDescription {
            anchor,
            name,
            description,
            obsolete,
            friends,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_FriendsDescription(value_: &FriendsDescription) -> ASN1Result<X690Element> {
    |value_: &FriendsDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        components_.push(ber_encode_object_identifier(&value_.anchor)?);
        if let Some(v_) = &value_.name {
            components_.push(
                |value_: &SET_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_UnboundedDirectoryString(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.description {
            components_.push(_encode_UnboundedDirectoryString(&v_)?);
        }
        if let Some(v_) = &value_.obsolete {
            if *v_ != FriendsDescription::_default_value_for_obsolete() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        components_.push(|v_1: &Vec<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
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
                }(&v_1)?))),
            ))
        }(&value_.friends)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soc-subschema OBJECT IDENTIFIER ::= {id-soc 1}
/// ```
///
///
pub fn id_soc_subschema() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soc().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-dITStructureRule       OBJECT IDENTIFIER ::= {id-soa 1}
/// ```
///
///
pub fn id_soa_dITStructureRule() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-dITContentRules        OBJECT IDENTIFIER ::= {id-soa 2}
/// ```
///
///
pub fn id_soa_dITContentRules() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-matchingRules          OBJECT IDENTIFIER ::= {id-soa 4}
/// ```
///
///
pub fn id_soa_matchingRules() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-attributeTypes         OBJECT IDENTIFIER ::= {id-soa 5}
/// ```
///
///
pub fn id_soa_attributeTypes() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-objectClasses          OBJECT IDENTIFIER ::= {id-soa 6}
/// ```
///
///
pub fn id_soa_objectClasses() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-nameForms              OBJECT IDENTIFIER ::= {id-soa 7}
/// ```
///
///
pub fn id_soa_nameForms() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-matchingRuleUse        OBJECT IDENTIFIER ::= {id-soa 8}
/// ```
///
///
pub fn id_soa_matchingRuleUse() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-structuralObjectClass  OBJECT IDENTIFIER ::= {id-soa 9}
/// ```
///
///
pub fn id_soa_structuralObjectClass() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-governingStructureRule OBJECT IDENTIFIER ::= {id-soa 10}
/// ```
///
///
pub fn id_soa_governingStructureRule() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-contextTypes           OBJECT IDENTIFIER ::= {id-soa 11}
/// ```
///
///
pub fn id_soa_contextTypes() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-dITContextUse          OBJECT IDENTIFIER ::= {id-soa 12}
/// ```
///
///
pub fn id_soa_dITContextUse() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-friends                OBJECT IDENTIFIER ::= {id-soa 13}
/// ```
///
///
pub fn id_soa_friends() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_soa().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}
