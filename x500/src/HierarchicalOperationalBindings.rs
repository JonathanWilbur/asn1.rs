#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # HierarchicalOperationalBindings
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `HierarchicalOperationalBindings`.
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
use crate::DirectoryOperationalBindingTypes::*;
use crate::DistributedOperations::*;
use crate::InformationFramework::*;
use crate::OperationalBindingManagement::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// HierarchicalAgreement ::= SEQUENCE {
///   rdn                [0]  RelativeDistinguishedName,
///   immediateSuperior  [1]  DistinguishedName,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct HierarchicalAgreement {
    pub rdn: RelativeDistinguishedName,
    pub immediateSuperior: DistinguishedName,
    pub _unrecognized: Vec<X690Element>,
}
impl HierarchicalAgreement {
    pub fn new(
        rdn: RelativeDistinguishedName,
        immediateSuperior: DistinguishedName,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        HierarchicalAgreement {
            rdn,
            immediateSuperior,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for HierarchicalAgreement {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_HierarchicalAgreement(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for HierarchicalAgreement {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_HierarchicalAgreement(el)
    }
}

pub const _rctl1_components_for_HierarchicalAgreement: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "rdn",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "immediateSuperior",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_HierarchicalAgreement: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_HierarchicalAgreement: &[ComponentSpec; 0] = &[];

pub fn _decode_HierarchicalAgreement(el: &X690Element) -> ASN1Result<HierarchicalAgreement> {
    |el_: &X690Element| -> ASN1Result<HierarchicalAgreement> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_HierarchicalAgreement,
            _eal_components_for_HierarchicalAgreement,
            _rctl2_components_for_HierarchicalAgreement,
        )?;
        let rdn = |el: &X690Element| -> ASN1Result<RelativeDistinguishedName> {
            Ok(_decode_RelativeDistinguishedName(&el.inner()?)?)
        }(_components.get("rdn").unwrap())?;
        let immediateSuperior = |el: &X690Element| -> ASN1Result<DistinguishedName> {
            Ok(_decode_DistinguishedName(&el.inner()?)?)
        }(_components.get("immediateSuperior").unwrap())?;
        Ok(HierarchicalAgreement {
            rdn,
            immediateSuperior,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_HierarchicalAgreement(value_: &HierarchicalAgreement) -> ASN1Result<X690Element> {
    |value_: &HierarchicalAgreement| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(
            |v_1: &RelativeDistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_RelativeDistinguishedName(&v_1)?,
                    ))),
                ))
            }(&value_.rdn)?,
        );
        components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                    &v_1,
                )?))),
            ))
        }(&value_.immediateSuperior)?);
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
/// SuperiorToSubordinate ::= SEQUENCE {
///   contextPrefixInfo     [0]  DITcontext,
///   entryInfo             [1]  SET SIZE (1..MAX) OF
///                                Attribute{{SupportedAttributes}} OPTIONAL,
///   immediateSuperiorInfo [2]  SET SIZE (1..MAX) OF
///                                Attribute{{SupportedAttributes}} OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SuperiorToSubordinate {
    pub contextPrefixInfo: DITcontext,
    pub entryInfo: OPTIONAL<Vec<Attribute>>,
    pub immediateSuperiorInfo: OPTIONAL<Vec<Attribute>>,
    pub _unrecognized: Vec<X690Element>,
}
impl SuperiorToSubordinate {
    pub fn new(
        contextPrefixInfo: DITcontext,
        entryInfo: OPTIONAL<Vec<Attribute>>,
        immediateSuperiorInfo: OPTIONAL<Vec<Attribute>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SuperiorToSubordinate {
            contextPrefixInfo,
            entryInfo,
            immediateSuperiorInfo,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for SuperiorToSubordinate {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SuperiorToSubordinate(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SuperiorToSubordinate {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SuperiorToSubordinate(el)
    }
}

pub const _rctl1_components_for_SuperiorToSubordinate: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "contextPrefixInfo",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "immediateSuperiorInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SuperiorToSubordinate: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SuperiorToSubordinate: &[ComponentSpec; 0] = &[];

pub fn _decode_SuperiorToSubordinate(el: &X690Element) -> ASN1Result<SuperiorToSubordinate> {
    |el_: &X690Element| -> ASN1Result<SuperiorToSubordinate> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SuperiorToSubordinate,
            _eal_components_for_SuperiorToSubordinate,
            _rctl2_components_for_SuperiorToSubordinate,
        )?;
        let contextPrefixInfo =
            |el: &X690Element| -> ASN1Result<DITcontext> { Ok(_decode_DITcontext(&el.inner()?)?) }(
                _components.get("contextPrefixInfo").unwrap(),
            )?;
        let entryInfo: OPTIONAL<Vec<Attribute>> = match _components.get("entryInfo") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<Attribute>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<Attribute>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_Attribute(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let immediateSuperiorInfo: OPTIONAL<Vec<Attribute>> =
            match _components.get("immediateSuperiorInfo") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<Attribute>> {
                    Ok(|el: &X690Element| -> ASN1Result<SET_OF<Attribute>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(_decode_Attribute(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(SuperiorToSubordinate {
            contextPrefixInfo,
            entryInfo,
            immediateSuperiorInfo,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SuperiorToSubordinate(value_: &SuperiorToSubordinate) -> ASN1Result<X690Element> {
    |value_: &SuperiorToSubordinate| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(|v_1: &DITcontext| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DITcontext(&v_1)?))),
            ))
        }(&value_.contextPrefixInfo)?);
        if let Some(v_) = &value_.entryInfo {
            components_.push(|v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        Attribute,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_Attribute(&v)?);
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
        if let Some(v_) = &value_.immediateSuperiorInfo {
            components_.push(|v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        Attribute,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_Attribute(&v)?);
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
/// DITcontext  ::=  SEQUENCE OF Vertex
/// ```
pub type DITcontext = Vec<Vertex>; // SequenceOfType

pub fn _decode_DITcontext(el: &X690Element) -> ASN1Result<DITcontext> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<Vertex>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<Vertex> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_Vertex(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_DITcontext(value_: &DITcontext) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<Vertex>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Vertex(&v)?);
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
/// Vertex ::= SEQUENCE {
///   rdn           [0]  RelativeDistinguishedName,
///   admPointInfo  [1]  SET SIZE (1..MAX) OF Attribute{{SupportedAttributes}} OPTIONAL,
///   subentries    [2]  SET SIZE (1..MAX) OF SubentryInfo OPTIONAL,
///   accessPoints  [3]  MasterAndShadowAccessPoints OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Vertex {
    pub rdn: RelativeDistinguishedName,
    pub admPointInfo: OPTIONAL<Vec<Attribute>>,
    pub subentries: OPTIONAL<Vec<SubentryInfo>>,
    pub accessPoints: OPTIONAL<MasterAndShadowAccessPoints>,
    pub _unrecognized: Vec<X690Element>,
}
impl Vertex {
    pub fn new(
        rdn: RelativeDistinguishedName,
        admPointInfo: OPTIONAL<Vec<Attribute>>,
        subentries: OPTIONAL<Vec<SubentryInfo>>,
        accessPoints: OPTIONAL<MasterAndShadowAccessPoints>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Vertex {
            rdn,
            admPointInfo,
            subentries,
            accessPoints,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for Vertex {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Vertex(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Vertex {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Vertex(el)
    }
}

pub const _rctl1_components_for_Vertex: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "rdn",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "admPointInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subentries",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "accessPoints",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Vertex: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Vertex: &[ComponentSpec; 0] = &[];

pub fn _decode_Vertex(el: &X690Element) -> ASN1Result<Vertex> {
    |el_: &X690Element| -> ASN1Result<Vertex> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Vertex,
            _eal_components_for_Vertex,
            _rctl2_components_for_Vertex,
        )?;
        let rdn = |el: &X690Element| -> ASN1Result<RelativeDistinguishedName> {
            Ok(_decode_RelativeDistinguishedName(&el.inner()?)?)
        }(_components.get("rdn").unwrap())?;
        let admPointInfo: OPTIONAL<Vec<Attribute>> = match _components.get("admPointInfo") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<Attribute>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<Attribute>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_Attribute(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let subentries: OPTIONAL<Vec<SubentryInfo>> = match _components.get("subentries") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<SubentryInfo>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<SubentryInfo>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<SubentryInfo> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_SubentryInfo(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let accessPoints: OPTIONAL<MasterAndShadowAccessPoints> =
            match _components.get("accessPoints") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<MasterAndShadowAccessPoints> {
                        Ok(_decode_MasterAndShadowAccessPoints(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        Ok(Vertex {
            rdn,
            admPointInfo,
            subentries,
            accessPoints,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Vertex(value_: &Vertex) -> ASN1Result<X690Element> {
    |value_: &Vertex| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(
            |v_1: &RelativeDistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_RelativeDistinguishedName(&v_1)?,
                    ))),
                ))
            }(&value_.rdn)?,
        );
        if let Some(v_) = &value_.admPointInfo {
            components_.push(|v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        Attribute,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_Attribute(&v)?);
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
        if let Some(v_) = &value_.subentries {
            components_.push(|v_1: &Vec<SubentryInfo>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        SubentryInfo,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_SubentryInfo(&v)?);
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
        if let Some(v_) = &value_.accessPoints {
            components_.push(
                |v_1: &MasterAndShadowAccessPoints| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        3,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_MasterAndShadowAccessPoints(&v_1)?,
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
/// SubentryInfo ::= SEQUENCE {
///   rdn   [0]  RelativeDistinguishedName,
///   info  [1]  SET OF Attribute{{SupportedAttributes}},
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SubentryInfo {
    pub rdn: RelativeDistinguishedName,
    pub info: Vec<Attribute>,
    pub _unrecognized: Vec<X690Element>,
}
impl SubentryInfo {
    pub fn new(
        rdn: RelativeDistinguishedName,
        info: Vec<Attribute>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SubentryInfo {
            rdn,
            info,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for SubentryInfo {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SubentryInfo(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SubentryInfo {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SubentryInfo(el)
    }
}

pub const _rctl1_components_for_SubentryInfo: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "rdn",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "info",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SubentryInfo: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SubentryInfo: &[ComponentSpec; 0] = &[];

pub fn _decode_SubentryInfo(el: &X690Element) -> ASN1Result<SubentryInfo> {
    |el_: &X690Element| -> ASN1Result<SubentryInfo> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SubentryInfo,
            _eal_components_for_SubentryInfo,
            _rctl2_components_for_SubentryInfo,
        )?;
        let rdn = |el: &X690Element| -> ASN1Result<RelativeDistinguishedName> {
            Ok(_decode_RelativeDistinguishedName(&el.inner()?)?)
        }(_components.get("rdn").unwrap())?;
        let info = |el: &X690Element| -> ASN1Result<Vec<Attribute>> {
            Ok(|el: &X690Element| -> ASN1Result<SET_OF<Attribute>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_Attribute(el)?);
                }
                Ok(items)
            }(&el.inner()?)?)
        }(_components.get("info").unwrap())?;
        Ok(SubentryInfo {
            rdn,
            info,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SubentryInfo(value_: &SubentryInfo) -> ASN1Result<X690Element> {
    |value_: &SubentryInfo| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(
            |v_1: &RelativeDistinguishedName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_RelativeDistinguishedName(&v_1)?,
                    ))),
                ))
            }(&value_.rdn)?,
        );
        components_.push(|v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                    Attribute,
                >|
                 -> ASN1Result<
                    X690Element,
                > {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_Attribute(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?))),
            ))
        }(&value_.info)?);
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
/// SubordinateToSuperior ::= SEQUENCE {
///   accessPoints  [0]  MasterAndShadowAccessPoints OPTIONAL,
///   alias         [1]  BOOLEAN DEFAULT FALSE,
///   entryInfo     [2]  SET SIZE (1..MAX) OF Attribute{{SupportedAttributes}} OPTIONAL,
///   subentries    [3]  SET SIZE (1..MAX) OF SubentryInfo OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SubordinateToSuperior {
    pub accessPoints: OPTIONAL<MasterAndShadowAccessPoints>,
    pub alias: OPTIONAL<BOOLEAN>,
    pub entryInfo: OPTIONAL<Vec<Attribute>>,
    pub subentries: OPTIONAL<Vec<SubentryInfo>>,
    pub _unrecognized: Vec<X690Element>,
}
impl SubordinateToSuperior {
    pub fn new(
        accessPoints: OPTIONAL<MasterAndShadowAccessPoints>,
        alias: OPTIONAL<BOOLEAN>,
        entryInfo: OPTIONAL<Vec<Attribute>>,
        subentries: OPTIONAL<Vec<SubentryInfo>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SubordinateToSuperior {
            accessPoints,
            alias,
            entryInfo,
            subentries,
            _unrecognized,
        }
    }
    pub fn _default_value_for_alias() -> BOOLEAN {
        false
    }
}
impl Default for SubordinateToSuperior {
    fn default() -> Self {
        SubordinateToSuperior {
            accessPoints: None,
            alias: None,
            entryInfo: None,
            subentries: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for SubordinateToSuperior {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SubordinateToSuperior(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SubordinateToSuperior {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SubordinateToSuperior(el)
    }
}

pub const _rctl1_components_for_SubordinateToSuperior: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "accessPoints",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "alias",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entryInfo",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subentries",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SubordinateToSuperior: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SubordinateToSuperior: &[ComponentSpec; 0] = &[];

pub fn _decode_SubordinateToSuperior(el: &X690Element) -> ASN1Result<SubordinateToSuperior> {
    |el_: &X690Element| -> ASN1Result<SubordinateToSuperior> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SubordinateToSuperior,
            _eal_components_for_SubordinateToSuperior,
            _rctl2_components_for_SubordinateToSuperior,
        )?;
        let accessPoints: OPTIONAL<MasterAndShadowAccessPoints> =
            match _components.get("accessPoints") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<MasterAndShadowAccessPoints> {
                        Ok(_decode_MasterAndShadowAccessPoints(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        let alias: OPTIONAL<BOOLEAN> = match _components.get("alias") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entryInfo: OPTIONAL<Vec<Attribute>> = match _components.get("entryInfo") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<Attribute>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<Attribute>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<Attribute> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_Attribute(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let subentries: OPTIONAL<Vec<SubentryInfo>> = match _components.get("subentries") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<SubentryInfo>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<SubentryInfo>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<SubentryInfo> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_SubentryInfo(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(SubordinateToSuperior {
            accessPoints,
            alias,
            entryInfo,
            subentries,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SubordinateToSuperior(value_: &SubordinateToSuperior) -> ASN1Result<X690Element> {
    |value_: &SubordinateToSuperior| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        if let Some(v_) = &value_.accessPoints {
            components_.push(
                |v_1: &MasterAndShadowAccessPoints| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_MasterAndShadowAccessPoints(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.alias {
            if *v_ != SubordinateToSuperior::_default_value_for_alias() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.entryInfo {
            components_.push(|v_1: &Vec<Attribute>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        Attribute,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_Attribute(&v)?);
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
        if let Some(v_) = &value_.subentries {
            components_.push(|v_1: &Vec<SubentryInfo>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        SubentryInfo,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_SubentryInfo(&v)?);
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
/// SuperiorToSubordinateModification  ::=  SuperiorToSubordinate (
///   WITH COMPONENTS {..., entryInfo  ABSENT } )
/// ```
pub type SuperiorToSubordinateModification = SuperiorToSubordinate; // DefinedType

pub fn _decode_SuperiorToSubordinateModification(
    el: &X690Element,
) -> ASN1Result<SuperiorToSubordinateModification> {
    _decode_SuperiorToSubordinate(&el)
}

pub fn _encode_SuperiorToSubordinateModification(
    value_: &SuperiorToSubordinateModification,
) -> ASN1Result<X690Element> {
    _encode_SuperiorToSubordinate(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NonSpecificHierarchicalAgreement ::= SEQUENCE {
///   immediateSuperior  [1]  DistinguishedName,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct NonSpecificHierarchicalAgreement {
    pub immediateSuperior: DistinguishedName,
    pub _unrecognized: Vec<X690Element>,
}
impl NonSpecificHierarchicalAgreement {
    pub fn new(immediateSuperior: DistinguishedName, _unrecognized: Vec<X690Element>) -> Self {
        NonSpecificHierarchicalAgreement {
            immediateSuperior,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for NonSpecificHierarchicalAgreement {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_NonSpecificHierarchicalAgreement(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for NonSpecificHierarchicalAgreement {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_NonSpecificHierarchicalAgreement(el)
    }
}

pub const _rctl1_components_for_NonSpecificHierarchicalAgreement: &[ComponentSpec; 1] =
    &[ComponentSpec::new(
        "immediateSuperior",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    )];

pub const _rctl2_components_for_NonSpecificHierarchicalAgreement: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_NonSpecificHierarchicalAgreement: &[ComponentSpec; 0] = &[];

pub fn _decode_NonSpecificHierarchicalAgreement(
    el: &X690Element,
) -> ASN1Result<NonSpecificHierarchicalAgreement> {
    |el_: &X690Element| -> ASN1Result<NonSpecificHierarchicalAgreement> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_NonSpecificHierarchicalAgreement,
            _eal_components_for_NonSpecificHierarchicalAgreement,
            _rctl2_components_for_NonSpecificHierarchicalAgreement,
        )?;
        let immediateSuperior = |el: &X690Element| -> ASN1Result<DistinguishedName> {
            Ok(_decode_DistinguishedName(&el.inner()?)?)
        }(_components.get("immediateSuperior").unwrap())?;
        Ok(NonSpecificHierarchicalAgreement {
            immediateSuperior,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_NonSpecificHierarchicalAgreement(
    value_: &NonSpecificHierarchicalAgreement,
) -> ASN1Result<X690Element> {
    |value_: &NonSpecificHierarchicalAgreement| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(11);
        components_.push(|v_1: &DistinguishedName| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DistinguishedName(
                    &v_1,
                )?))),
            ))
        }(&value_.immediateSuperior)?);
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
/// NHOBSuperiorToSubordinate  ::=  SuperiorToSubordinate (
///   WITH COMPONENTS {..., entryInfo  ABSENT } )
/// ```
pub type NHOBSuperiorToSubordinate = SuperiorToSubordinate; // DefinedType

pub fn _decode_NHOBSuperiorToSubordinate(
    el: &X690Element,
) -> ASN1Result<NHOBSuperiorToSubordinate> {
    _decode_SuperiorToSubordinate(&el)
}

pub fn _encode_NHOBSuperiorToSubordinate(
    value_: &NHOBSuperiorToSubordinate,
) -> ASN1Result<X690Element> {
    _encode_SuperiorToSubordinate(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NHOBSubordinateToSuperior ::= SEQUENCE {
///   accessPoints  [0]  MasterAndShadowAccessPoints OPTIONAL,
///   subentries    [3]  SET SIZE (1..MAX) OF SubentryInfo OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct NHOBSubordinateToSuperior {
    pub accessPoints: OPTIONAL<MasterAndShadowAccessPoints>,
    pub subentries: OPTIONAL<Vec<SubentryInfo>>,
    pub _unrecognized: Vec<X690Element>,
}
impl NHOBSubordinateToSuperior {
    pub fn new(
        accessPoints: OPTIONAL<MasterAndShadowAccessPoints>,
        subentries: OPTIONAL<Vec<SubentryInfo>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        NHOBSubordinateToSuperior {
            accessPoints,
            subentries,
            _unrecognized,
        }
    }
}
impl Default for NHOBSubordinateToSuperior {
    fn default() -> Self {
        NHOBSubordinateToSuperior {
            accessPoints: None,
            subentries: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for NHOBSubordinateToSuperior {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_NHOBSubordinateToSuperior(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for NHOBSubordinateToSuperior {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_NHOBSubordinateToSuperior(el)
    }
}

pub const _rctl1_components_for_NHOBSubordinateToSuperior: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "accessPoints",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subentries",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_NHOBSubordinateToSuperior: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_NHOBSubordinateToSuperior: &[ComponentSpec; 0] = &[];

pub fn _decode_NHOBSubordinateToSuperior(
    el: &X690Element,
) -> ASN1Result<NHOBSubordinateToSuperior> {
    |el_: &X690Element| -> ASN1Result<NHOBSubordinateToSuperior> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_NHOBSubordinateToSuperior,
            _eal_components_for_NHOBSubordinateToSuperior,
            _rctl2_components_for_NHOBSubordinateToSuperior,
        )?;
        let accessPoints: OPTIONAL<MasterAndShadowAccessPoints> =
            match _components.get("accessPoints") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<MasterAndShadowAccessPoints> {
                        Ok(_decode_MasterAndShadowAccessPoints(&el.inner()?)?)
                    }(c_)?,
                ),
                _ => None,
            };
        let subentries: OPTIONAL<Vec<SubentryInfo>> = match _components.get("subentries") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<SubentryInfo>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<SubentryInfo>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<SubentryInfo> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_SubentryInfo(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(NHOBSubordinateToSuperior {
            accessPoints,
            subentries,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_NHOBSubordinateToSuperior(
    value_: &NHOBSubordinateToSuperior,
) -> ASN1Result<X690Element> {
    |value_: &NHOBSubordinateToSuperior| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.accessPoints {
            components_.push(
                |v_1: &MasterAndShadowAccessPoints| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_MasterAndShadowAccessPoints(&v_1)?,
                        ))),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.subentries {
            components_.push(|v_1: &Vec<SubentryInfo>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        SubentryInfo,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_SubentryInfo(&v)?);
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
/// hierarchicalOperationalBinding OPERATIONAL-BINDING ::= {
///   AGREEMENT             HierarchicalAgreement
///   APPLICATION CONTEXTS  {{directorySystemAC}}
///   ASYMMETRIC
///     ROLE-A { -- superior DSA
///       ESTABLISHMENT-INITIATOR  TRUE
///       ESTABLISHMENT-PARAMETER  SuperiorToSubordinate
///       MODIFICATION-INITIATOR   TRUE
///       MODIFICATION-PARAMETER   SuperiorToSubordinateModification
///       TERMINATION-INITIATOR    TRUE }
///     ROLE-B { -- subordinate DSA
///       ESTABLISHMENT-INITIATOR  TRUE
///       ESTABLISHMENT-PARAMETER  SubordinateToSuperior
///       MODIFICATION-INITIATOR   TRUE
///       MODIFICATION-PARAMETER   SubordinateToSuperior
///       TERMINATION-INITIATOR    TRUE }
///   ID                    id-op-binding-hierarchical }
/// ```
///
///
pub fn hierarchicalOperationalBinding() -> OPERATIONAL_BINDING {
    OPERATIONAL_BINDING {
        Cooperation: Vec::<_>::from(
            [ /* FIXME: COULD_NOT_COMPILE_DEFINED_SYNTAX_IN_OBJECT_SET */ ],
        ), /* OBJECT_FIELD_SETTING */
        roleA: Some(hierarchicalOperationalBinding_roleA()), /* OBJECT_FIELD_SETTING */
        roleB: Some(hierarchicalOperationalBinding_roleB()), /* OBJECT_FIELD_SETTING */
        id: id_op_binding_hierarchical(),                    /* OBJECT_FIELD_SETTING */
        both: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nonSpecificHierarchicalOperationalBinding OPERATIONAL-BINDING ::= {
///   AGREEMENT             NonSpecificHierarchicalAgreement
///   APPLICATION CONTEXTS  {{directorySystemAC}}
///   ASYMMETRIC
///     ROLE-A { -- superior DSA
///       ESTABLISHMENT-PARAMETER  NHOBSuperiorToSubordinate
///       MODIFICATION-INITIATOR   TRUE
///       MODIFICATION-PARAMETER   NHOBSuperiorToSubordinate
///       TERMINATION-INITIATOR    TRUE}
///     ROLE-B { -- subordinate DSA
///       ESTABLISHMENT-INITIATOR  TRUE
///       ESTABLISHMENT-PARAMETER  NHOBSubordinateToSuperior
///       MODIFICATION-INITIATOR   TRUE
///       MODIFICATION-PARAMETER   NHOBSubordinateToSuperior
///       TERMINATION-INITIATOR    TRUE}
///   ID                    id-op-binding-non-specific-hierarchical }
/// ```
///
///
pub fn nonSpecificHierarchicalOperationalBinding() -> OPERATIONAL_BINDING {
    OPERATIONAL_BINDING {
        Cooperation: Vec::<_>::from(
            [ /* FIXME: COULD_NOT_COMPILE_DEFINED_SYNTAX_IN_OBJECT_SET */ ],
        ), /* OBJECT_FIELD_SETTING */
        roleA: Some(nonSpecificHierarchicalOperationalBinding_roleA()), /* OBJECT_FIELD_SETTING */
        roleB: Some(nonSpecificHierarchicalOperationalBinding_roleB()), /* OBJECT_FIELD_SETTING */
        id: id_op_binding_non_specific_hierarchical(),                  /* OBJECT_FIELD_SETTING */
        both: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hierarchicalOperationalBinding-roleA ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn hierarchicalOperationalBinding_roleA() -> OP_BIND_ROLE {
    OP_BIND_ROLE {
        establish: Some(true), /* OBJECT_FIELD_SETTING */
        modify: Some(true),    /* OBJECT_FIELD_SETTING */
        terminate: Some(true), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hierarchicalOperationalBinding-roleB ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn hierarchicalOperationalBinding_roleB() -> OP_BIND_ROLE {
    OP_BIND_ROLE {
        establish: Some(true), /* OBJECT_FIELD_SETTING */
        modify: Some(true),    /* OBJECT_FIELD_SETTING */
        terminate: Some(true), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nonSpecificHierarchicalOperationalBinding-roleA ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn nonSpecificHierarchicalOperationalBinding_roleA() -> OP_BIND_ROLE {
    OP_BIND_ROLE {
        modify: Some(true),     /* OBJECT_FIELD_SETTING */
        terminate: Some(true),  /* OBJECT_FIELD_SETTING */
        establish: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nonSpecificHierarchicalOperationalBinding-roleB ::= {} -- REMOVED_FROM_UNNESTING --
/// ```
///
///
pub fn nonSpecificHierarchicalOperationalBinding_roleB() -> OP_BIND_ROLE {
    OP_BIND_ROLE {
        establish: Some(true), /* OBJECT_FIELD_SETTING */
        modify: Some(true),    /* OBJECT_FIELD_SETTING */
        terminate: Some(true), /* OBJECT_FIELD_SETTING */
    }
}
