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
use std::borrow::Borrow;
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
impl TryFrom<X690Element> for UserNotice {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UserNotice(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UserNotice {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<UserNotice> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UserNotice,
            _eal_components_for_UserNotice,
            _rctl2_components_for_UserNotice,
        )?;
        let noticeRef: OPTIONAL<NoticeReference> = match _components.get("noticeRef") {
            Some(c_) => Some(_decode_NoticeReference(c_)?),
            _ => None,
        };
        let explicitText: OPTIONAL<DisplayText> = match _components.get("explicitText") {
            Some(c_) => Some(_decode_DisplayText(c_)?),
            _ => None,
        };
        Ok(UserNotice {
            noticeRef,
            explicitText,
        })
    }(&el)
}

pub fn _encode_UserNotice(value_: &UserNotice) -> ASN1Result<X690Element> {
    |value_: &UserNotice| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.noticeRef {
            components_.push(_encode_NoticeReference(&v_)?);
        }
        if let Some(v_) = &value_.explicitText {
            components_.push(_encode_DisplayText(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
impl TryFrom<X690Element> for NoticeReference {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_NoticeReference(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for NoticeReference {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<NoticeReference> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_NoticeReference,
            _eal_components_for_NoticeReference,
            _rctl2_components_for_NoticeReference,
        )?;
        let organization = _decode_DisplayText(_components.get("organization").unwrap())?;
        let noticeNumbers = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<INTEGER>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<INTEGER> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(ber_decode_integer(el)?);
            }
            Ok(items)
        }(_components.get("noticeNumbers").unwrap())?;
        Ok(NoticeReference {
            organization,
            noticeNumbers,
        })
    }(&el)
}

pub fn _encode_NoticeReference(value_: &NoticeReference) -> ASN1Result<X690Element> {
    |value_: &NoticeReference| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_DisplayText(&value_.organization)?);
        components_.push(|value_: &SEQUENCE_OF<INTEGER>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(ber_encode_integer(&v)?);
            }
            Ok(X690Element::new(
                TagClass::UNIVERSAL,
                ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                Arc::new(X690Encoding::Constructed(children)),
            ))
        }(&value_.noticeNumbers)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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

impl TryFrom<X690Element> for DisplayText {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DisplayText(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DisplayText {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DisplayText(el)
    }
}

pub fn _decode_DisplayText(el: &X690Element) -> ASN1Result<DisplayText> {
    |el: &X690Element| -> ASN1Result<DisplayText> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 26) => {
                Ok(DisplayText::visibleString(ber_decode_visible_string(&el)?))
            }
            (TagClass::UNIVERSAL, 30) => Ok(DisplayText::bmpString(ber_decode_bmp_string(&el)?)),
            (TagClass::UNIVERSAL, 12) => Ok(DisplayText::utf8String(ber_decode_utf8_string(&el)?)),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_DisplayText(value_: &DisplayText) -> ASN1Result<X690Element> {
    |value: &DisplayText| -> ASN1Result<X690Element> {
        match value {
            DisplayText::visibleString(v) => ber_encode_visible_string(&v),
            DisplayText::bmpString(v) => ber_encode_bmp_string(&v),
            DisplayText::utf8String(v) => ber_encode_utf8_string(&v),
        }
    }(&value_)
}
