#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PKIX1Implicit93
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PKIX1Implicit93`.
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
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// UserNotice ::= SEQUENCE {
///   noticeRef     NoticeReference OPTIONAL,
///   explicitText  DisplayText OPTIONAL
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct UserNotice {
    pub noticeRef: OPTIONAL<NoticeReference>,
    pub explicitText: OPTIONAL<DisplayText>,
}
impl UserNotice {
    pub fn new(noticeRef: OPTIONAL<NoticeReference>, explicitText: OPTIONAL<DisplayText>) -> Self {
        UserNotice {
            noticeRef,
            explicitText,
        }
    }
}
impl Default for UserNotice {
    fn default() -> Self {
        UserNotice {
            noticeRef: None,
            explicitText: None,
        }
    }
}
impl TryFrom<&X690Element> for UserNotice {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UserNotice(el)
    }
}

pub const _rctl1_components_for_UserNotice: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "noticeRef",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "explicitText",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 26)),
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 12)),
        ]),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UserNotice: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UserNotice: &[ComponentSpec; 0] = &[];

pub fn _decode_UserNotice(el: &X690Element) -> ASN1Result<UserNotice> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserNotice")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UserNotice,
        _eal_components_for_UserNotice,
        _rctl2_components_for_UserNotice,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut noticeRef_: OPTIONAL<NoticeReference> = None;
    let mut explicitText_: OPTIONAL<DisplayText> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "noticeRef" => noticeRef_ = Some(_decode_NoticeReference(_el)?),
            "explicitText" => explicitText_ = Some(_decode_DisplayText(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserNotice"))
            }
        }
    }
    Ok(UserNotice {
        noticeRef: noticeRef_,
        explicitText: explicitText_,
    })
}

pub fn _encode_UserNotice(value_: &UserNotice) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    if let Some(v_) = &value_.noticeRef {
        components_.push(_encode_NoticeReference(&v_)?);
    }
    if let Some(v_) = &value_.explicitText {
        components_.push(_encode_DisplayText(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_UserNotice(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserNotice")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UserNotice,
        _eal_components_for_UserNotice,
        _rctl2_components_for_UserNotice,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "noticeRef" => _validate_NoticeReference(_el)?,
            "explicitText" => _validate_DisplayText(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserNotice"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NoticeReference ::= SEQUENCE {
///   organization   DisplayText,
///   noticeNumbers  SEQUENCE OF INTEGER
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct NoticeReference {
    pub organization: DisplayText,
    pub noticeNumbers: Vec<INTEGER>,
}
impl NoticeReference {
    pub fn new(organization: DisplayText, noticeNumbers: Vec<INTEGER>) -> Self {
        NoticeReference {
            organization,
            noticeNumbers,
        }
    }
}
impl TryFrom<&X690Element> for NoticeReference {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_NoticeReference(el)
    }
}

pub const _rctl1_components_for_NoticeReference: &[ComponentSpec; 2] = &[
    ComponentSpec::new("organization", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "noticeNumbers",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_NoticeReference: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_NoticeReference: &[ComponentSpec; 0] = &[];

pub fn _decode_NoticeReference(el: &X690Element) -> ASN1Result<NoticeReference> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NoticeReference"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_NoticeReference,
        _eal_components_for_NoticeReference,
        _rctl2_components_for_NoticeReference,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut organization_: OPTIONAL<DisplayText> = None;
    let mut noticeNumbers_: OPTIONAL<Vec<INTEGER>> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "organization" => organization_ = Some(_decode_DisplayText(_el)?),
            "noticeNumbers" => {
                noticeNumbers_ = Some(|el: &X690Element| -> ASN1Result<SEQUENCE_OF<INTEGER>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(el.to_asn1_err_named(
                                ASN1ErrorCode::invalid_construction,
                                "noticeNumbers",
                            ))
                        }
                    };
                    let mut items: SEQUENCE_OF<INTEGER> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(BER.decode_integer(el)?);
                    }
                    Ok(items)
                }(_el)?)
            }
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NoticeReference")
                )
            }
        }
    }
    Ok(NoticeReference {
        organization: organization_.unwrap(),
        noticeNumbers: noticeNumbers_.unwrap(),
    })
}

pub fn _encode_NoticeReference(value_: &NoticeReference) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(_encode_DisplayText(&value_.organization)?);
    components_.push(|value_: &SEQUENCE_OF<INTEGER>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(BER.encode_integer(&v)?);
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
            X690Value::Constructed(Arc::new(children)),
        ))
    }(&value_.noticeNumbers)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_NoticeReference(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NoticeReference"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_NoticeReference,
        _eal_components_for_NoticeReference,
        _rctl2_components_for_NoticeReference,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "organization" => _validate_DisplayText(_el)?,
            "noticeNumbers" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            BER.validate_integer(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "noticeNumbers")
                    ),
                }
            }(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NoticeReference")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DisplayText  ::=  CHOICE {
///   visibleString  VisibleString(SIZE (1..200)),
///   bmpString      BMPString(SIZE (1..200)),
///   utf8String     UTF8String(SIZE (1..200))
/// }
/// ```
#[derive(Debug, Clone)]
pub enum DisplayText {
    visibleString(VisibleString),
    bmpString(BMPString),
    utf8String(UTF8String),
}

impl TryFrom<&X690Element> for DisplayText {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DisplayText(el)
    }
}

pub fn _decode_DisplayText(el: &X690Element) -> ASN1Result<DisplayText> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 26) => {
            Ok(DisplayText::visibleString(BER.decode_visible_string(&el)?))
        }
        (TagClass::UNIVERSAL, 30) => Ok(DisplayText::bmpString(BER.decode_bmp_string(&el)?)),
        (TagClass::UNIVERSAL, 12) => Ok(DisplayText::utf8String(BER.decode_utf8_string(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "DisplayText",
            ))
        }
    }
}

pub fn _encode_DisplayText(value_: &DisplayText) -> ASN1Result<X690Element> {
    match value_ {
        DisplayText::visibleString(v) => BER.encode_visible_string(&v),
        DisplayText::bmpString(v) => BER.encode_bmp_string(&v),
        DisplayText::utf8String(v) => BER.encode_utf8_string(&v),
        _ => {
            let mut err =
                ASN1Error::new(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice);
            err.component_name = Some("DisplayText".to_string());
            Err(err)
        }
    }
}

pub fn _validate_DisplayText(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 26) => BER.validate_visible_string(&el),
        (TagClass::UNIVERSAL, 30) => BER.validate_bmp_string(&el),
        (TagClass::UNIVERSAL, 12) => BER.validate_utf8_string(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "DisplayText",
            ))
        }
    }
}
