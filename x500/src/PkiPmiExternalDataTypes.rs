#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PkiPmiExternalDataTypes
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PkiPmiExternalDataTypes`.
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
use crate::AuthenticationFramework::*;
use crate::CertificateExtensions::*;
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// UserNotice ::= SEQUENCE {
///   noticeRef     NoticeReference OPTIONAL,
///   explicitText  DisplayText OPTIONAL }
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
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NoticeReference ::= SEQUENCE {
///   organization   DisplayText,
///   noticeNumbers  SEQUENCE OF INTEGER }
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
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
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
///   utf8String     UTF8String(SIZE (1..200)) }
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// authorityInfoAccess EXTENSION ::= {
///   SYNTAX         AuthorityInfoAccessSyntax
///   IDENTIFIED BY  id-pe-authorityInfoAccess }
/// ```
///
///
pub fn authorityInfoAccess() -> EXTENSION {
    EXTENSION {
        id: id_pe_authorityInfoAccess(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AuthorityInfoAccessSyntax  ::=  SEQUENCE SIZE (1..MAX) OF AccessDescription
/// ```
pub type AuthorityInfoAccessSyntax = Vec<AccessDescription>; // SequenceOfType

pub fn _decode_AuthorityInfoAccessSyntax(
    el: &X690Element,
) -> ASN1Result<AuthorityInfoAccessSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AccessDescription>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<AccessDescription> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AccessDescription(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AuthorityInfoAccessSyntax(
    value_: &AuthorityInfoAccessSyntax,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<AccessDescription>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AccessDescription(&v)?);
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
/// AccessDescription ::= SEQUENCE {
///   accessMethod    OBJECT IDENTIFIER,
///   accessLocation  GeneralName }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AccessDescription {
    pub accessMethod: OBJECT_IDENTIFIER,
    pub accessLocation: GeneralName,
}
impl AccessDescription {
    pub fn new(accessMethod: OBJECT_IDENTIFIER, accessLocation: GeneralName) -> Self {
        AccessDescription {
            accessMethod,
            accessLocation,
        }
    }
}
impl TryFrom<X690Element> for AccessDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AccessDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AccessDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AccessDescription(el)
    }
}

pub const _rctl1_components_for_AccessDescription: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "accessMethod",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("accessLocation", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_AccessDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AccessDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_AccessDescription(el: &X690Element) -> ASN1Result<AccessDescription> {
    |el_: &X690Element| -> ASN1Result<AccessDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AccessDescription,
            _eal_components_for_AccessDescription,
            _rctl2_components_for_AccessDescription,
        )?;
        let accessMethod = ber_decode_object_identifier(_components.get("accessMethod").unwrap())?;
        let accessLocation = _decode_GeneralName(_components.get("accessLocation").unwrap())?;
        Ok(AccessDescription {
            accessMethod,
            accessLocation,
        })
    }(&el)
}

pub fn _encode_AccessDescription(value_: &AccessDescription) -> ASN1Result<X690Element> {
    |value_: &AccessDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_object_identifier(&value_.accessMethod)?);
        components_.push(_encode_GeneralName(&value_.accessLocation)?);
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
/// subjectInfoAccess EXTENSION ::= {
///   SYNTAX         SubjectInfoAccessSyntax
///   IDENTIFIED BY  id-pe-subjectInfoAccess }
/// ```
///
///
pub fn subjectInfoAccess() -> EXTENSION {
    EXTENSION {
        id: id_pe_subjectInfoAccess(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubjectInfoAccessSyntax  ::=  SEQUENCE SIZE (1..MAX) OF AccessDescription
/// ```
pub type SubjectInfoAccessSyntax = Vec<AccessDescription>; // SequenceOfType

pub fn _decode_SubjectInfoAccessSyntax(el: &X690Element) -> ASN1Result<SubjectInfoAccessSyntax> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AccessDescription>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<AccessDescription> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AccessDescription(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_SubjectInfoAccessSyntax(
    value_: &SubjectInfoAccessSyntax,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<AccessDescription>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AccessDescription(&v)?);
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
/// id-pkix                   OBJECT IDENTIFIER ::= { intSecurity mechanisms(5) pkix(7) }
/// ```
///
///
pub fn id_pkix() -> OBJECT_IDENTIFIER {
    [
        intSecurity(),
        Vec::<u32>::from([/* mechanisms */ 5, /* pkix */ 7]),
    ]
    .concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pe                     OBJECT IDENTIFIER ::= { id-pkix 1 }
/// ```
///
///
pub fn id_pe() -> OBJECT_IDENTIFIER {
    [id_pkix(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ad                     OBJECT IDENTIFIER ::= { id-pkix 48 }
/// ```
///
///
pub fn id_ad() -> OBJECT_IDENTIFIER {
    [id_pkix(), Vec::<u32>::from([48])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pe-authorityInfoAccess OBJECT IDENTIFIER ::= { id-pe 1 }
/// ```
///
///
pub fn id_pe_authorityInfoAccess() -> OBJECT_IDENTIFIER {
    [id_pe(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pe-subjectInfoAccess   OBJECT IDENTIFIER ::= { id-pe 11 }
/// ```
///
///
pub fn id_pe_subjectInfoAccess() -> OBJECT_IDENTIFIER {
    [id_pe(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ad-caIssuers           OBJECT IDENTIFIER ::= { id-ad 2 }
/// ```
///
///
pub fn id_ad_caIssuers() -> OBJECT_IDENTIFIER {
    [id_ad(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ad-ocsp                OBJECT IDENTIFIER ::= { id-ad 1 }
/// ```
///
///
pub fn id_ad_ocsp() -> OBJECT_IDENTIFIER {
    [id_ad(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// G3FacsimileNonBasicParameters  ::=  BIT STRING {
///   two-dimensional(8), -- As defined in ITU-T Recommendation T.30
///   fine-resolution(9),
///   unlimited-length(20), -- These bit values are chosen such that when
///   b4-length(21), -- encoded using ASN.1 Basic Encoding Rules
///   a3-width(22), -- the resulting octets have the same values
///   b4-width(23), -- as for T.30 encoding
///   t6-coding(25),
///   uncompressed(30), -- Trailing zero bits are not significant
///   width-middle-864-of-1728(37), -- It is recommended that implementations
///   width-middle-1216-of-1728(38), -- should not encode more than 32 bits unless
///   resolution-type(44), -- higher numbered bits are non-zero
///   resolution-400x400(45), resolution-300x300(46), resolution-8x15(47),
///   edi(49), dtm(50), bft(51), mixed-mode(58), character-mode(60),
///   twelve-bits(65), preferred-huffmann(66), full-colour(67), jpeg(68),
///   processable-mode-26(71)}
/// ```
pub type G3FacsimileNonBasicParameters = BIT_STRING;

pub const G3FacsimileNonBasicParameters_two_dimensional: BIT = 8; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_fine_resolution: BIT = 9; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_unlimited_length: BIT = 20; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_b4_length: BIT = 21; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_a3_width: BIT = 22; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_b4_width: BIT = 23; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_t6_coding: BIT = 25; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_uncompressed: BIT = 30; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_width_middle_864_of_1728: BIT = 37; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_width_middle_1216_of_1728: BIT = 38; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_resolution_type: BIT = 44; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_resolution_400x400: BIT = 45; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_resolution_300x300: BIT = 46; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_resolution_8x15: BIT = 47; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_edi: BIT = 49; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_dtm: BIT = 50; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_bft: BIT = 51; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_mixed_mode: BIT = 58; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_character_mode: BIT = 60; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_twelve_bits: BIT = 65; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_preferred_huffmann: BIT = 66; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_full_colour: BIT = 67; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_jpeg: BIT = 68; /* LONG_NAMED_BIT */

pub const G3FacsimileNonBasicParameters_processable_mode_26: BIT = 71; /* LONG_NAMED_BIT */

pub fn _decode_G3FacsimileNonBasicParameters(
    el: &X690Element,
) -> ASN1Result<G3FacsimileNonBasicParameters> {
    ber_decode_bit_string(&el)
}

pub fn _encode_G3FacsimileNonBasicParameters(
    value_: &G3FacsimileNonBasicParameters,
) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ORAddress ::= SEQUENCE {
///   built-in-standard-attributes        BuiltInStandardAttributes,
///   built-in-domain-defined-attributes  BuiltInDomainDefinedAttributes OPTIONAL,
///   -- see also teletex-domain-defined-attributes
///   extension-attributes                ExtensionAttributes OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ORAddress {
    pub built_in_standard_attributes: BuiltInStandardAttributes,
    pub built_in_domain_defined_attributes: OPTIONAL<BuiltInDomainDefinedAttributes>,
    pub extension_attributes: OPTIONAL<ExtensionAttributes>,
}
impl ORAddress {
    pub fn new(
        built_in_standard_attributes: BuiltInStandardAttributes,
        built_in_domain_defined_attributes: OPTIONAL<BuiltInDomainDefinedAttributes>,
        extension_attributes: OPTIONAL<ExtensionAttributes>,
    ) -> Self {
        ORAddress {
            built_in_standard_attributes,
            built_in_domain_defined_attributes,
            extension_attributes,
        }
    }
}
impl TryFrom<X690Element> for ORAddress {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ORAddress(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ORAddress {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ORAddress(el)
    }
}

pub const _rctl1_components_for_ORAddress: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "built-in-standard-attributes",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "built-in-domain-defined-attributes",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extension-attributes",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ORAddress: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ORAddress: &[ComponentSpec; 0] = &[];

pub fn _decode_ORAddress(el: &X690Element) -> ASN1Result<ORAddress> {
    |el_: &X690Element| -> ASN1Result<ORAddress> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ORAddress,
            _eal_components_for_ORAddress,
            _rctl2_components_for_ORAddress,
        )?;
        let built_in_standard_attributes = _decode_BuiltInStandardAttributes(
            _components.get("built-in-standard-attributes").unwrap(),
        )?;
        let built_in_domain_defined_attributes: OPTIONAL<BuiltInDomainDefinedAttributes> =
            match _components.get("built-in-domain-defined-attributes") {
                Some(c_) => Some(_decode_BuiltInDomainDefinedAttributes(c_)?),
                _ => None,
            };
        let extension_attributes: OPTIONAL<ExtensionAttributes> =
            match _components.get("extension-attributes") {
                Some(c_) => Some(_decode_ExtensionAttributes(c_)?),
                _ => None,
            };
        Ok(ORAddress {
            built_in_standard_attributes,
            built_in_domain_defined_attributes,
            extension_attributes,
        })
    }(&el)
}

pub fn _encode_ORAddress(value_: &ORAddress) -> ASN1Result<X690Element> {
    |value_: &ORAddress| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_BuiltInStandardAttributes(
            &value_.built_in_standard_attributes,
        )?);
        if let Some(v_) = &value_.built_in_domain_defined_attributes {
            components_.push(_encode_BuiltInDomainDefinedAttributes(&v_)?);
        }
        if let Some(v_) = &value_.extension_attributes {
            components_.push(_encode_ExtensionAttributes(&v_)?);
        }
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
/// BuiltInStandardAttributes ::= SEQUENCE {
///   country-name                CountryName OPTIONAL,
///   administration-domain-name  AdministrationDomainName OPTIONAL,
///   network-address             [0]  NetworkAddress OPTIONAL,
///   -- see also extended-network-address
///   terminal-identifier         [1]  TerminalIdentifier OPTIONAL,
///   private-domain-name         [2]  PrivateDomainName OPTIONAL,
///   organization-name           [3]  OrganizationName OPTIONAL,
///   -- see also teletex-organization-name
///   numeric-user-identifier     [4]  NumericUserIdentifier OPTIONAL,
///   personal-name               [5]  PersonalName OPTIONAL,
///   -- see also teletex-personal-name
///   organizational-unit-names   [6]  OrganizationalUnitNames OPTIONAL
///   -- see also teletex-organizational-unit-names --}
/// ```
///
///
#[derive(Debug, Clone)]
pub struct BuiltInStandardAttributes {
    pub country_name: OPTIONAL<CountryName>,
    pub administration_domain_name: OPTIONAL<AdministrationDomainName>,
    pub network_address: OPTIONAL<NetworkAddress>,
    pub terminal_identifier: OPTIONAL<TerminalIdentifier>,
    pub private_domain_name: OPTIONAL<PrivateDomainName>,
    pub organization_name: OPTIONAL<OrganizationName>,
    pub numeric_user_identifier: OPTIONAL<NumericUserIdentifier>,
    pub personal_name: OPTIONAL<PersonalName>,
    pub organizational_unit_names: OPTIONAL<OrganizationalUnitNames>,
}
impl BuiltInStandardAttributes {
    pub fn new(
        country_name: OPTIONAL<CountryName>,
        administration_domain_name: OPTIONAL<AdministrationDomainName>,
        network_address: OPTIONAL<NetworkAddress>,
        terminal_identifier: OPTIONAL<TerminalIdentifier>,
        private_domain_name: OPTIONAL<PrivateDomainName>,
        organization_name: OPTIONAL<OrganizationName>,
        numeric_user_identifier: OPTIONAL<NumericUserIdentifier>,
        personal_name: OPTIONAL<PersonalName>,
        organizational_unit_names: OPTIONAL<OrganizationalUnitNames>,
    ) -> Self {
        BuiltInStandardAttributes {
            country_name,
            administration_domain_name,
            network_address,
            terminal_identifier,
            private_domain_name,
            organization_name,
            numeric_user_identifier,
            personal_name,
            organizational_unit_names,
        }
    }
}
impl Default for BuiltInStandardAttributes {
    fn default() -> Self {
        BuiltInStandardAttributes {
            country_name: None,
            administration_domain_name: None,
            network_address: None,
            terminal_identifier: None,
            private_domain_name: None,
            organization_name: None,
            numeric_user_identifier: None,
            personal_name: None,
            organizational_unit_names: None,
        }
    }
}
impl TryFrom<X690Element> for BuiltInStandardAttributes {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_BuiltInStandardAttributes(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for BuiltInStandardAttributes {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_BuiltInStandardAttributes(el)
    }
}

pub const _rctl1_components_for_BuiltInStandardAttributes: &[ComponentSpec; 9] = &[
    ComponentSpec::new(
        "country-name",
        true,
        TagSelector::tag((TagClass::APPLICATION, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "administration-domain-name",
        true,
        TagSelector::tag((TagClass::APPLICATION, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "network-address",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "terminal-identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "private-domain-name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "organization-name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "numeric-user-identifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "personal-name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 5)),
        None,
        None,
    ),
    ComponentSpec::new(
        "organizational-unit-names",
        true,
        TagSelector::tag((TagClass::CONTEXT, 6)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_BuiltInStandardAttributes: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_BuiltInStandardAttributes: &[ComponentSpec; 0] = &[];

pub fn _decode_BuiltInStandardAttributes(
    el: &X690Element,
) -> ASN1Result<BuiltInStandardAttributes> {
    |el_: &X690Element| -> ASN1Result<BuiltInStandardAttributes> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_BuiltInStandardAttributes,
            _eal_components_for_BuiltInStandardAttributes,
            _rctl2_components_for_BuiltInStandardAttributes,
        )?;
        let country_name: OPTIONAL<CountryName> = match _components.get("country-name") {
            Some(c_) => Some(_decode_CountryName(c_)?),
            _ => None,
        };
        let administration_domain_name: OPTIONAL<AdministrationDomainName> =
            match _components.get("administration-domain-name") {
                Some(c_) => Some(_decode_AdministrationDomainName(c_)?),
                _ => None,
            };
        let network_address: OPTIONAL<NetworkAddress> = match _components.get("network-address") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<NetworkAddress> {
                Ok(_decode_NetworkAddress(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let terminal_identifier: OPTIONAL<TerminalIdentifier> =
            match _components.get("terminal-identifier") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<TerminalIdentifier> {
                    Ok(_decode_TerminalIdentifier(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let private_domain_name: OPTIONAL<PrivateDomainName> =
            match _components.get("private-domain-name") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<PrivateDomainName> {
                    Ok(_decode_PrivateDomainName(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let organization_name: OPTIONAL<OrganizationName> =
            match _components.get("organization-name") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OrganizationName> {
                    Ok(_decode_OrganizationName(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let numeric_user_identifier: OPTIONAL<NumericUserIdentifier> =
            match _components.get("numeric-user-identifier") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<NumericUserIdentifier> {
                    Ok(_decode_NumericUserIdentifier(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        let personal_name: OPTIONAL<PersonalName> = match _components.get("personal-name") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<PersonalName> {
                Ok(_decode_PersonalName(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let organizational_unit_names: OPTIONAL<OrganizationalUnitNames> =
            match _components.get("organizational-unit-names") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<OrganizationalUnitNames> {
                    Ok(_decode_OrganizationalUnitNames(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(BuiltInStandardAttributes {
            country_name,
            administration_domain_name,
            network_address,
            terminal_identifier,
            private_domain_name,
            organization_name,
            numeric_user_identifier,
            personal_name,
            organizational_unit_names,
        })
    }(&el)
}

pub fn _encode_BuiltInStandardAttributes(
    value_: &BuiltInStandardAttributes,
) -> ASN1Result<X690Element> {
    |value_: &BuiltInStandardAttributes| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        if let Some(v_) = &value_.country_name {
            components_.push(_encode_CountryName(&v_)?);
        }
        if let Some(v_) = &value_.administration_domain_name {
            components_.push(_encode_AdministrationDomainName(&v_)?);
        }
        if let Some(v_) = &value_.network_address {
            components_.push(|v_1: &NetworkAddress| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_NetworkAddress(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.terminal_identifier {
            components_.push(|v_1: &TerminalIdentifier| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_TerminalIdentifier(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.private_domain_name {
            components_.push(|v_1: &PrivateDomainName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_PrivateDomainName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.organization_name {
            components_.push(|v_1: &OrganizationName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_OrganizationName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.numeric_user_identifier {
            components_.push(|v_1: &NumericUserIdentifier| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_NumericUserIdentifier(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.personal_name {
            components_.push(|v_1: &PersonalName| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_PersonalName(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.organizational_unit_names {
            components_.push(|v_1: &OrganizationalUnitNames| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    6,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_OrganizationalUnitNames(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
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
/// CountryName  ::=  [APPLICATION 1]  CHOICE {
///   x121-dcc-code         NumericString(SIZE (ub-country-name-numeric-length)),
///   iso-3166-alpha2-code  PrintableString(SIZE (ub-country-name-alpha-length)) }
/// ```
#[derive(Debug, Clone)]
pub enum CountryName {
    x121_dcc_code(NumericString),
    iso_3166_alpha2_code(PrintableString),
}

impl TryFrom<X690Element> for CountryName {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CountryName(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CountryName {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CountryName(el)
    }
}

pub fn _decode_CountryName(el: &X690Element) -> ASN1Result<CountryName> {
    |el: &X690Element| -> ASN1Result<CountryName> {
        Ok(|el: &X690Element| -> ASN1Result<CountryName> {
            match (el.tag_class, el.tag_number) {
                (TagClass::UNIVERSAL, 18) => {
                    Ok(CountryName::x121_dcc_code(ber_decode_numeric_string(&el)?))
                }
                (TagClass::UNIVERSAL, 19) => Ok(CountryName::iso_3166_alpha2_code(
                    ber_decode_printable_string(&el)?,
                )),
                _ => {
                    return Err(ASN1Error::new(
                        ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                    ))
                }
            }
        }(&el.inner()?)?)
    }(&el)
}

pub fn _encode_CountryName(value_: &CountryName) -> ASN1Result<X690Element> {
    |v_1: &CountryName| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            TagClass::APPLICATION,
            1,
            Arc::new(X690Encoding::EXPLICIT(Box::new(
                |value: &CountryName| -> ASN1Result<X690Element> {
                    match value {
                        CountryName::x121_dcc_code(v) => ber_encode_numeric_string(&v),
                        CountryName::iso_3166_alpha2_code(v) => ber_encode_printable_string(&v),
                    }
                }(&v_1)?,
            ))),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AdministrationDomainName  ::=  [APPLICATION 2]  CHOICE {
///   numeric    NumericString(SIZE (0..ub-domain-name-length)),
///   printable  PrintableString(SIZE (0..ub-domain-name-length)) }
/// ```
#[derive(Debug, Clone)]
pub enum AdministrationDomainName {
    numeric(NumericString),
    printable(PrintableString),
}

impl TryFrom<X690Element> for AdministrationDomainName {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AdministrationDomainName(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AdministrationDomainName {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AdministrationDomainName(el)
    }
}

pub fn _decode_AdministrationDomainName(el: &X690Element) -> ASN1Result<AdministrationDomainName> {
    |el: &X690Element| -> ASN1Result<AdministrationDomainName> {
        Ok(|el: &X690Element| -> ASN1Result<AdministrationDomainName> {
            match (el.tag_class, el.tag_number) {
                (TagClass::UNIVERSAL, 18) => Ok(AdministrationDomainName::numeric(
                    ber_decode_numeric_string(&el)?,
                )),
                (TagClass::UNIVERSAL, 19) => Ok(AdministrationDomainName::printable(
                    ber_decode_printable_string(&el)?,
                )),
                _ => {
                    return Err(ASN1Error::new(
                        ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                    ))
                }
            }
        }(&el.inner()?)?)
    }(&el)
}

pub fn _encode_AdministrationDomainName(
    value_: &AdministrationDomainName,
) -> ASN1Result<X690Element> {
    |v_1: &AdministrationDomainName| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            TagClass::APPLICATION,
            2,
            Arc::new(X690Encoding::EXPLICIT(Box::new(
                |value: &AdministrationDomainName| -> ASN1Result<X690Element> {
                    match value {
                        AdministrationDomainName::numeric(v) => ber_encode_numeric_string(&v),
                        AdministrationDomainName::printable(v) => ber_encode_printable_string(&v),
                    }
                }(&v_1)?,
            ))),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NetworkAddress  ::=  X121Address
/// ```
pub type NetworkAddress = X121Address; // DefinedType

pub fn _decode_NetworkAddress(el: &X690Element) -> ASN1Result<NetworkAddress> {
    _decode_X121Address(&el)
}

pub fn _encode_NetworkAddress(value_: &NetworkAddress) -> ASN1Result<X690Element> {
    _encode_X121Address(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// X121Address  ::=  NumericString(SIZE (1..ub-x121-address-length))
/// ```
pub type X121Address = NumericString; // NumericString

pub fn _decode_X121Address(el: &X690Element) -> ASN1Result<X121Address> {
    ber_decode_numeric_string(&el)
}

pub fn _encode_X121Address(value_: &X121Address) -> ASN1Result<X690Element> {
    ber_encode_numeric_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TerminalIdentifier  ::=  PrintableString(SIZE (1..ub-terminal-id-length))
/// ```
pub type TerminalIdentifier = PrintableString; // PrintableString

pub fn _decode_TerminalIdentifier(el: &X690Element) -> ASN1Result<TerminalIdentifier> {
    ber_decode_printable_string(&el)
}

pub fn _encode_TerminalIdentifier(value_: &TerminalIdentifier) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PrivateDomainName  ::=  CHOICE {
///   numeric    NumericString(SIZE (1..ub-domain-name-length)),
///   printable  PrintableString(SIZE (1..ub-domain-name-length)) }
/// ```
#[derive(Debug, Clone)]
pub enum PrivateDomainName {
    numeric(NumericString),
    printable(PrintableString),
}

impl TryFrom<X690Element> for PrivateDomainName {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PrivateDomainName(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PrivateDomainName {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PrivateDomainName(el)
    }
}

pub fn _decode_PrivateDomainName(el: &X690Element) -> ASN1Result<PrivateDomainName> {
    |el: &X690Element| -> ASN1Result<PrivateDomainName> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 18) => {
                Ok(PrivateDomainName::numeric(ber_decode_numeric_string(&el)?))
            }
            (TagClass::UNIVERSAL, 19) => Ok(PrivateDomainName::printable(
                ber_decode_printable_string(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_PrivateDomainName(value_: &PrivateDomainName) -> ASN1Result<X690Element> {
    |value: &PrivateDomainName| -> ASN1Result<X690Element> {
        match value {
            PrivateDomainName::numeric(v) => ber_encode_numeric_string(&v),
            PrivateDomainName::printable(v) => ber_encode_printable_string(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OrganizationName  ::=  PrintableString(SIZE (1..ub-organization-name-length))
/// ```
pub type OrganizationName = PrintableString; // PrintableString

pub fn _decode_OrganizationName(el: &X690Element) -> ASN1Result<OrganizationName> {
    ber_decode_printable_string(&el)
}

pub fn _encode_OrganizationName(value_: &OrganizationName) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NumericUserIdentifier  ::=  NumericString(SIZE (1..ub-numeric-user-id-length))
/// ```
pub type NumericUserIdentifier = NumericString; // NumericString

pub fn _decode_NumericUserIdentifier(el: &X690Element) -> ASN1Result<NumericUserIdentifier> {
    ber_decode_numeric_string(&el)
}

pub fn _encode_NumericUserIdentifier(value_: &NumericUserIdentifier) -> ASN1Result<X690Element> {
    ber_encode_numeric_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PersonalName ::= SET {
///   surname               [0]  PrintableString(SIZE (1..ub-surname-length)),
///   given-name
///     [1]  PrintableString(SIZE (1..ub-given-name-length)) OPTIONAL,
///   initials
///     [2]  PrintableString(SIZE (1..ub-initials-length)) OPTIONAL,
///   generation-qualifier
///     [3]  PrintableString(SIZE (1..ub-generation-qualifier-length)) OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PersonalName {
    pub surname: PrintableString,
    pub given_name: OPTIONAL<PrintableString>,
    pub initials: OPTIONAL<PrintableString>,
    pub generation_qualifier: OPTIONAL<PrintableString>,
}
impl PersonalName {
    pub fn new(
        surname: PrintableString,
        given_name: OPTIONAL<PrintableString>,
        initials: OPTIONAL<PrintableString>,
        generation_qualifier: OPTIONAL<PrintableString>,
    ) -> Self {
        PersonalName {
            surname,
            given_name,
            initials,
            generation_qualifier,
        }
    }
}
impl TryFrom<X690Element> for PersonalName {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PersonalName(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PersonalName {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PersonalName(el)
    }
}

pub const _rctl1_components_for_PersonalName: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "surname",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "given-name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "initials",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "generation-qualifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PersonalName: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PersonalName: &[ComponentSpec; 0] = &[];

pub fn _decode_PersonalName(el: &X690Element) -> ASN1Result<PersonalName> {
    |el_: &X690Element| -> ASN1Result<PersonalName> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_PersonalName,
            _eal_components_for_PersonalName,
            _rctl2_components_for_PersonalName,
            40,
        )?;
        let surname = |el: &X690Element| -> ASN1Result<PrintableString> {
            Ok(ber_decode_printable_string(&el.inner()?)?)
        }(_components.get("surname").unwrap())?;
        let given_name: OPTIONAL<PrintableString> = match _components.get("given-name") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<PrintableString> {
                Ok(ber_decode_printable_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let initials: OPTIONAL<PrintableString> = match _components.get("initials") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<PrintableString> {
                Ok(ber_decode_printable_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let generation_qualifier: OPTIONAL<PrintableString> =
            match _components.get("generation-qualifier") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<PrintableString> {
                    Ok(ber_decode_printable_string(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(PersonalName {
            surname,
            given_name,
            initials,
            generation_qualifier,
        })
    }(&el)
}

pub fn _encode_PersonalName(value_: &PersonalName) -> ASN1Result<X690Element> {
    |value_: &PersonalName| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(9);
        components_.push(|v_1: &PrintableString| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    ber_encode_printable_string(&v_1)?,
                ))),
            ))
        }(&value_.surname)?);
        if let Some(v_) = &value_.given_name {
            components_.push(|v_1: &PrintableString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_printable_string(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.initials {
            components_.push(|v_1: &PrintableString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_printable_string(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.generation_qualifier {
            components_.push(|v_1: &PrintableString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_printable_string(&v_1)?,
                    ))),
                ))
            }(&v_)?);
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
/// OrganizationalUnitNames  ::=
///   SEQUENCE SIZE (1..ub-organizational-units) OF OrganizationalUnitName
/// ```
pub type OrganizationalUnitNames = Vec<OrganizationalUnitName>; // SequenceOfType

pub fn _decode_OrganizationalUnitNames(el: &X690Element) -> ASN1Result<OrganizationalUnitNames> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<OrganizationalUnitName>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<OrganizationalUnitName> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_OrganizationalUnitName(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_OrganizationalUnitNames(
    value_: &OrganizationalUnitNames,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<OrganizationalUnitName>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_OrganizationalUnitName(&v)?);
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
/// OrganizationalUnitName  ::=
///   PrintableString(SIZE (1..ub-organizational-unit-name-length))
/// ```
pub type OrganizationalUnitName = PrintableString; // PrintableString

pub fn _decode_OrganizationalUnitName(el: &X690Element) -> ASN1Result<OrganizationalUnitName> {
    ber_decode_printable_string(&el)
}

pub fn _encode_OrganizationalUnitName(value_: &OrganizationalUnitName) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// BuiltInDomainDefinedAttributes  ::=
///   SEQUENCE SIZE (1..ub-domain-defined-attributes) OF
///     BuiltInDomainDefinedAttribute
/// ```
pub type BuiltInDomainDefinedAttributes = Vec<BuiltInDomainDefinedAttribute>; // SequenceOfType

pub fn _decode_BuiltInDomainDefinedAttributes(
    el: &X690Element,
) -> ASN1Result<BuiltInDomainDefinedAttributes> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<BuiltInDomainDefinedAttribute>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<BuiltInDomainDefinedAttribute> =
            Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_BuiltInDomainDefinedAttribute(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_BuiltInDomainDefinedAttributes(
    value_: &BuiltInDomainDefinedAttributes,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<BuiltInDomainDefinedAttribute>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_BuiltInDomainDefinedAttribute(&v)?);
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
/// BuiltInDomainDefinedAttribute ::= SEQUENCE {
///   type   PrintableString(SIZE (1..ub-domain-defined-attribute-type-length)),
///   value  PrintableString(SIZE (1..ub-domain-defined-attribute-value-length)) }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct BuiltInDomainDefinedAttribute {
    pub type_: PrintableString,
    pub value: PrintableString,
}
impl BuiltInDomainDefinedAttribute {
    pub fn new(type_: PrintableString, value: PrintableString) -> Self {
        BuiltInDomainDefinedAttribute { type_, value }
    }
}
impl TryFrom<X690Element> for BuiltInDomainDefinedAttribute {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_BuiltInDomainDefinedAttribute(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for BuiltInDomainDefinedAttribute {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_BuiltInDomainDefinedAttribute(el)
    }
}

pub const _rctl1_components_for_BuiltInDomainDefinedAttribute: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 19)),
        None,
        None,
    ),
    ComponentSpec::new(
        "value",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 19)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_BuiltInDomainDefinedAttribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_BuiltInDomainDefinedAttribute: &[ComponentSpec; 0] = &[];

pub fn _decode_BuiltInDomainDefinedAttribute(
    el: &X690Element,
) -> ASN1Result<BuiltInDomainDefinedAttribute> {
    |el_: &X690Element| -> ASN1Result<BuiltInDomainDefinedAttribute> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_BuiltInDomainDefinedAttribute,
            _eal_components_for_BuiltInDomainDefinedAttribute,
            _rctl2_components_for_BuiltInDomainDefinedAttribute,
        )?;
        let type_ = ber_decode_printable_string(_components.get("type").unwrap())?;
        let value = ber_decode_printable_string(_components.get("value").unwrap())?;
        Ok(BuiltInDomainDefinedAttribute { type_, value })
    }(&el)
}

pub fn _encode_BuiltInDomainDefinedAttribute(
    value_: &BuiltInDomainDefinedAttribute,
) -> ASN1Result<X690Element> {
    |value_: &BuiltInDomainDefinedAttribute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_printable_string(&value_.type_)?);
        components_.push(ber_encode_printable_string(&value_.value)?);
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
/// ExtensionAttributes  ::=
///   SET SIZE (1..ub-extension-attributes) OF ExtensionAttribute
/// ```
pub type ExtensionAttributes = Vec<ExtensionAttribute>; // SetOfType

pub fn _decode_ExtensionAttributes(el: &X690Element) -> ASN1Result<ExtensionAttributes> {
    |el: &X690Element| -> ASN1Result<SET_OF<ExtensionAttribute>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<ExtensionAttribute> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_ExtensionAttribute(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_ExtensionAttributes(value_: &ExtensionAttributes) -> ASN1Result<X690Element> {
    |value_: &SET_OF<ExtensionAttribute>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_ExtensionAttribute(&v)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
            Arc::new(X690Encoding::Constructed(children)),
        ))
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtensionAttribute ::= SEQUENCE {
///   extension-attribute-type
///     [0]  EXTENSION-ATTRIBUTE.&id({ExtensionAttributeTable}),
///   extension-attribute-value
///     [1]  EXTENSION-ATTRIBUTE.&Type
///            ({ExtensionAttributeTable}{@extension-attribute-type}) }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ExtensionAttribute {
    pub extension_attribute_type: INTEGER,
    pub extension_attribute_value: X690Element,
}
impl ExtensionAttribute {
    pub fn new(extension_attribute_type: INTEGER, extension_attribute_value: X690Element) -> Self {
        ExtensionAttribute {
            extension_attribute_type,
            extension_attribute_value,
        }
    }
}
impl TryFrom<X690Element> for ExtensionAttribute {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ExtensionAttribute(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ExtensionAttribute {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ExtensionAttribute(el)
    }
}

pub const _rctl1_components_for_ExtensionAttribute: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "extension-attribute-type",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "extension-attribute-value",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ExtensionAttribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ExtensionAttribute: &[ComponentSpec; 0] = &[];

pub fn _decode_ExtensionAttribute(el: &X690Element) -> ASN1Result<ExtensionAttribute> {
    |el_: &X690Element| -> ASN1Result<ExtensionAttribute> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ExtensionAttribute,
            _eal_components_for_ExtensionAttribute,
            _rctl2_components_for_ExtensionAttribute,
        )?;
        let extension_attribute_type =
            |el: &X690Element| -> ASN1Result<INTEGER> { Ok(ber_decode_integer(&el.inner()?)?) }(
                _components.get("extension-attribute-type").unwrap(),
            )?;
        let extension_attribute_value =
            |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                _components.get("extension-attribute-value").unwrap(),
            )?;
        Ok(ExtensionAttribute {
            extension_attribute_type,
            extension_attribute_value,
        })
    }(&el)
}

pub fn _encode_ExtensionAttribute(value_: &ExtensionAttribute) -> ASN1Result<X690Element> {
    |value_: &ExtensionAttribute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
            ))
        }(&value_.extension_attribute_type)?);
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
            ))
        }(&value_.extension_attribute_value)?);
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
/// EXTENSION-ATTRIBUTE ::= CLASS {
///   &id           INTEGER(0..ub-extension-attributes) UNIQUE,
///   &Type }
/// WITH SYNTAX {
///                 &Type
///   IDENTIFIED BY &id }
/// ```
///
#[derive(Debug)]
pub struct EXTENSION_ATTRIBUTE {
    pub id: INTEGER,
}
impl EXTENSION_ATTRIBUTE {}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtensionAttributeTable EXTENSION-ATTRIBUTE ::= {common-name | teletex-common-name | universal-common-name |
///    teletex-organization-name | universal-organization-name |
///    teletex-personal-name | universal-personal-name |
///    teletex-organizational-unit-names | universal-organizational-unit-names |
///    teletex-domain-defined-attributes | universal-domain-defined-attributes |
///    pds-name | physical-delivery-country-name | postal-code |
///    physical-delivery-office-name | universal-physical-delivery-office-name |
///    physical-delivery-office-number | universal-physical-delivery-office-number
///    | extension-OR-address-components |
///    universal-extension-OR-address-components | physical-delivery-personal-name
///    | universal-physical-delivery-personal-name |
///    physical-delivery-organization-name |
///    universal-physical-delivery-organization-name |
///    extension-physical-delivery-address-components |
///    universal-extension-physical-delivery-address-components |
///    unformatted-postal-address | universal-unformatted-postal-address |
///    street-address | universal-street-address | post-office-box-address |
///    universal-post-office-box-address | poste-restante-address |
///    universal-poste-restante-address | unique-postal-name |
///    universal-unique-postal-name | local-postal-attributes |
///    universal-local-postal-attributes | extended-network-address | terminal-type }
/// ```
///
///
pub fn ExtensionAttributeTable() -> Vec<EXTENSION_ATTRIBUTE> {
    Vec::from([
        common_name(),
        teletex_common_name(),
        universal_common_name(),
        teletex_organization_name(),
        universal_organization_name(),
        teletex_personal_name(),
        universal_personal_name(),
        teletex_organizational_unit_names(),
        universal_organizational_unit_names(),
        teletex_domain_defined_attributes(),
        universal_domain_defined_attributes(),
        pds_name(),
        physical_delivery_country_name(),
        postal_code(),
        physical_delivery_office_name(),
        universal_physical_delivery_office_name(),
        physical_delivery_office_number(),
        universal_physical_delivery_office_number(),
        extension_OR_address_components(),
        universal_extension_OR_address_components(),
        physical_delivery_personal_name(),
        universal_physical_delivery_personal_name(),
        physical_delivery_organization_name(),
        universal_physical_delivery_organization_name(),
        extension_physical_delivery_address_components(),
        universal_extension_physical_delivery_address_components(),
        unformatted_postal_address(),
        universal_unformatted_postal_address(),
        street_address(),
        universal_street_address(),
        post_office_box_address(),
        universal_post_office_box_address(),
        poste_restante_address(),
        universal_poste_restante_address(),
        unique_postal_name(),
        universal_unique_postal_name(),
        local_postal_attributes(),
        universal_local_postal_attributes(),
        extended_network_address(),
        terminal_type(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// common-name EXTENSION-ATTRIBUTE ::= {
///                  CommonName
///   IDENTIFIED BY  1 }
/// ```
///
///
pub fn common_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 1, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CommonName  ::=  PrintableString(SIZE (1..ub-common-name-length))
/// ```
pub type CommonName = PrintableString; // PrintableString

pub fn _decode_CommonName(el: &X690Element) -> ASN1Result<CommonName> {
    ber_decode_printable_string(&el)
}

pub fn _encode_CommonName(value_: &CommonName) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// teletex-common-name EXTENSION-ATTRIBUTE ::= {
///                  TeletexCommonName
///   IDENTIFIED BY  2 }
/// ```
///
///
pub fn teletex_common_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 2, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TeletexCommonName  ::=  TeletexString(SIZE (1..ub-common-name-length))
/// ```
pub type TeletexCommonName = TeletexString; // TeletexString

pub fn _decode_TeletexCommonName(el: &X690Element) -> ASN1Result<TeletexCommonName> {
    ber_decode_t61_string(&el)
}

pub fn _encode_TeletexCommonName(value_: &TeletexCommonName) -> ASN1Result<X690Element> {
    ber_encode_t61_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-common-name EXTENSION-ATTRIBUTE ::= {
///                  UniversalCommonName
///   IDENTIFIED BY  24 }
/// ```
///
///
pub fn universal_common_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 24, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalCommonName  ::=  UniversalOrBMPString{ub-common-name-length}
/// ```
pub type UniversalCommonName = UniversalOrBMPString; // DefinedType

pub fn _decode_UniversalCommonName(el: &X690Element) -> ASN1Result<UniversalCommonName> {
    _decode_UniversalOrBMPString(&el)
}

pub fn _encode_UniversalCommonName(value_: &UniversalCommonName) -> ASN1Result<X690Element> {
    _encode_UniversalOrBMPString(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// teletex-organization-name EXTENSION-ATTRIBUTE ::= {
///                  TeletexOrganizationName
///   IDENTIFIED BY  3 }
/// ```
///
///
pub fn teletex_organization_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 3, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TeletexOrganizationName  ::=
///   TeletexString(SIZE (1..ub-organization-name-length))
/// ```
pub type TeletexOrganizationName = TeletexString; // TeletexString

pub fn _decode_TeletexOrganizationName(el: &X690Element) -> ASN1Result<TeletexOrganizationName> {
    ber_decode_t61_string(&el)
}

pub fn _encode_TeletexOrganizationName(
    value_: &TeletexOrganizationName,
) -> ASN1Result<X690Element> {
    ber_encode_t61_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-organization-name EXTENSION-ATTRIBUTE ::= {
///                  UniversalOrganizationName
///   IDENTIFIED BY  25 }
/// ```
///
///
pub fn universal_organization_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 25, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalOrganizationName  ::=  UniversalOrBMPString{ub-organization-name-length}
/// ```
pub type UniversalOrganizationName = UniversalOrBMPString; // DefinedType

pub fn _decode_UniversalOrganizationName(
    el: &X690Element,
) -> ASN1Result<UniversalOrganizationName> {
    _decode_UniversalOrBMPString(&el)
}

pub fn _encode_UniversalOrganizationName(
    value_: &UniversalOrganizationName,
) -> ASN1Result<X690Element> {
    _encode_UniversalOrBMPString(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// teletex-personal-name EXTENSION-ATTRIBUTE ::= {
///                  TeletexPersonalName
///   IDENTIFIED BY  4 }
/// ```
///
///
pub fn teletex_personal_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 4, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TeletexPersonalName ::= SET {
///   surname               [0]  TeletexString(SIZE (1..ub-surname-length)),
///   given-name
///     [1]  TeletexString(SIZE (1..ub-given-name-length)) OPTIONAL,
///   initials
///     [2]  TeletexString(SIZE (1..ub-initials-length)) OPTIONAL,
///   generation-qualifier
///     [3]  TeletexString(SIZE (1..ub-generation-qualifier-length)) OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TeletexPersonalName {
    pub surname: TeletexString,
    pub given_name: OPTIONAL<TeletexString>,
    pub initials: OPTIONAL<TeletexString>,
    pub generation_qualifier: OPTIONAL<TeletexString>,
}
impl TeletexPersonalName {
    pub fn new(
        surname: TeletexString,
        given_name: OPTIONAL<TeletexString>,
        initials: OPTIONAL<TeletexString>,
        generation_qualifier: OPTIONAL<TeletexString>,
    ) -> Self {
        TeletexPersonalName {
            surname,
            given_name,
            initials,
            generation_qualifier,
        }
    }
}
impl TryFrom<X690Element> for TeletexPersonalName {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TeletexPersonalName(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TeletexPersonalName {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TeletexPersonalName(el)
    }
}

pub const _rctl1_components_for_TeletexPersonalName: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "surname",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "given-name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "initials",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "generation-qualifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TeletexPersonalName: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TeletexPersonalName: &[ComponentSpec; 0] = &[];

pub fn _decode_TeletexPersonalName(el: &X690Element) -> ASN1Result<TeletexPersonalName> {
    |el_: &X690Element| -> ASN1Result<TeletexPersonalName> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_TeletexPersonalName,
            _eal_components_for_TeletexPersonalName,
            _rctl2_components_for_TeletexPersonalName,
            40,
        )?;
        let surname = |el: &X690Element| -> ASN1Result<TeletexString> {
            Ok(ber_decode_t61_string(&el.inner()?)?)
        }(_components.get("surname").unwrap())?;
        let given_name: OPTIONAL<TeletexString> = match _components.get("given-name") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<TeletexString> {
                Ok(ber_decode_t61_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let initials: OPTIONAL<TeletexString> = match _components.get("initials") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<TeletexString> {
                Ok(ber_decode_t61_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let generation_qualifier: OPTIONAL<TeletexString> =
            match _components.get("generation-qualifier") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<TeletexString> {
                    Ok(ber_decode_t61_string(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(TeletexPersonalName {
            surname,
            given_name,
            initials,
            generation_qualifier,
        })
    }(&el)
}

pub fn _encode_TeletexPersonalName(value_: &TeletexPersonalName) -> ASN1Result<X690Element> {
    |value_: &TeletexPersonalName| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(9);
        components_.push(|v_1: &TeletexString| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_t61_string(
                    &v_1,
                )?))),
            ))
        }(&value_.surname)?);
        if let Some(v_) = &value_.given_name {
            components_.push(|v_1: &TeletexString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_t61_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.initials {
            components_.push(|v_1: &TeletexString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_t61_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.generation_qualifier {
            components_.push(|v_1: &TeletexString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_t61_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
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
/// universal-personal-name EXTENSION-ATTRIBUTE ::= {
///                  UniversalPersonalName
///   IDENTIFIED BY  26 }
/// ```
///
///
pub fn universal_personal_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 26, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalPersonalName ::= SET {
///   surname
///     [0]  UniversalOrBMPString{ub-universal-surname-length},
///   -- If a language is specified within surname, then that language applies to each of the
///   -- following optional components unless the component specifies another language.
///   given-name
///     [1]  UniversalOrBMPString{ub-universal-given-name-length} OPTIONAL,
///   initials
///     [2]  UniversalOrBMPString{ub-universal-initials-length} OPTIONAL,
///   generation-qualifier
///     [3]  UniversalOrBMPString{ub-universal-generation-qualifier-length} OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UniversalPersonalName {
    pub surname: UniversalOrBMPString,
    pub given_name: OPTIONAL<UniversalOrBMPString>,
    pub initials: OPTIONAL<UniversalOrBMPString>,
    pub generation_qualifier: OPTIONAL<UniversalOrBMPString>,
}
impl UniversalPersonalName {
    pub fn new(
        surname: UniversalOrBMPString,
        given_name: OPTIONAL<UniversalOrBMPString>,
        initials: OPTIONAL<UniversalOrBMPString>,
        generation_qualifier: OPTIONAL<UniversalOrBMPString>,
    ) -> Self {
        UniversalPersonalName {
            surname,
            given_name,
            initials,
            generation_qualifier,
        }
    }
}
impl TryFrom<X690Element> for UniversalPersonalName {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UniversalPersonalName(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UniversalPersonalName {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UniversalPersonalName(el)
    }
}

pub const _rctl1_components_for_UniversalPersonalName: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "surname",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "given-name",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "initials",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "generation-qualifier",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UniversalPersonalName: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UniversalPersonalName: &[ComponentSpec; 0] = &[];

pub fn _decode_UniversalPersonalName(el: &X690Element) -> ASN1Result<UniversalPersonalName> {
    |el_: &X690Element| -> ASN1Result<UniversalPersonalName> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_UniversalPersonalName,
            _eal_components_for_UniversalPersonalName,
            _rctl2_components_for_UniversalPersonalName,
            40,
        )?;
        let surname = |el: &X690Element| -> ASN1Result<UniversalOrBMPString> {
            Ok(_decode_UniversalOrBMPString(&el.inner()?)?)
        }(_components.get("surname").unwrap())?;
        let given_name: OPTIONAL<UniversalOrBMPString> = match _components.get("given-name") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<UniversalOrBMPString> {
                Ok(_decode_UniversalOrBMPString(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let initials: OPTIONAL<UniversalOrBMPString> = match _components.get("initials") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<UniversalOrBMPString> {
                Ok(_decode_UniversalOrBMPString(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let generation_qualifier: OPTIONAL<UniversalOrBMPString> =
            match _components.get("generation-qualifier") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<UniversalOrBMPString> {
                    Ok(_decode_UniversalOrBMPString(&el.inner()?)?)
                }(c_)?),
                _ => None,
            };
        Ok(UniversalPersonalName {
            surname,
            given_name,
            initials,
            generation_qualifier,
        })
    }(&el)
}

pub fn _encode_UniversalPersonalName(value_: &UniversalPersonalName) -> ASN1Result<X690Element> {
    |value_: &UniversalPersonalName| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(9);
        components_.push(|v_1: &UniversalOrBMPString| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    _encode_UniversalOrBMPString(&v_1)?,
                ))),
            ))
        }(&value_.surname)?);
        if let Some(v_) = &value_.given_name {
            components_.push(|v_1: &UniversalOrBMPString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_UniversalOrBMPString(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.initials {
            components_.push(|v_1: &UniversalOrBMPString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_UniversalOrBMPString(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.generation_qualifier {
            components_.push(|v_1: &UniversalOrBMPString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        _encode_UniversalOrBMPString(&v_1)?,
                    ))),
                ))
            }(&v_)?);
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
/// teletex-organizational-unit-names EXTENSION-ATTRIBUTE ::= {
///                  TeletexOrganizationalUnitNames
///   IDENTIFIED BY  5 }
/// ```
///
///
pub fn teletex_organizational_unit_names() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 5, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TeletexOrganizationalUnitNames  ::=
///   SEQUENCE SIZE (1..ub-organizational-units) OF TeletexOrganizationalUnitName
/// ```
pub type TeletexOrganizationalUnitNames = Vec<TeletexOrganizationalUnitName>; // SequenceOfType

pub fn _decode_TeletexOrganizationalUnitNames(
    el: &X690Element,
) -> ASN1Result<TeletexOrganizationalUnitNames> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<TeletexOrganizationalUnitName>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<TeletexOrganizationalUnitName> =
            Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_TeletexOrganizationalUnitName(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_TeletexOrganizationalUnitNames(
    value_: &TeletexOrganizationalUnitNames,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<TeletexOrganizationalUnitName>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_TeletexOrganizationalUnitName(&v)?);
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
/// TeletexOrganizationalUnitName  ::=
///   TeletexString(SIZE (1..ub-organizational-unit-name-length))
/// ```
pub type TeletexOrganizationalUnitName = TeletexString; // TeletexString

pub fn _decode_TeletexOrganizationalUnitName(
    el: &X690Element,
) -> ASN1Result<TeletexOrganizationalUnitName> {
    ber_decode_t61_string(&el)
}

pub fn _encode_TeletexOrganizationalUnitName(
    value_: &TeletexOrganizationalUnitName,
) -> ASN1Result<X690Element> {
    ber_encode_t61_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-organizational-unit-names EXTENSION-ATTRIBUTE ::= {
///                  UniversalOrganizationalUnitNames
///   IDENTIFIED BY  27 }
/// ```
///
///
pub fn universal_organizational_unit_names() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 27, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalOrganizationalUnitNames  ::=
///   SEQUENCE SIZE (1..ub-organizational-units) OF UniversalOrganizationalUnitName
/// ```
pub type UniversalOrganizationalUnitNames = Vec<UniversalOrganizationalUnitName>; // SequenceOfType

pub fn _decode_UniversalOrganizationalUnitNames(
    el: &X690Element,
) -> ASN1Result<UniversalOrganizationalUnitNames> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<UniversalOrganizationalUnitName>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<UniversalOrganizationalUnitName> =
            Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_UniversalOrganizationalUnitName(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_UniversalOrganizationalUnitNames(
    value_: &UniversalOrganizationalUnitNames,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<UniversalOrganizationalUnitName>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_UniversalOrganizationalUnitName(&v)?);
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
/// UniversalOrganizationalUnitName  ::=
///   UniversalOrBMPString{ub-organizational-unit-name-length}
/// ```
pub type UniversalOrganizationalUnitName = UniversalOrBMPString; // DefinedType

pub fn _decode_UniversalOrganizationalUnitName(
    el: &X690Element,
) -> ASN1Result<UniversalOrganizationalUnitName> {
    _decode_UniversalOrBMPString(&el)
}

pub fn _encode_UniversalOrganizationalUnitName(
    value_: &UniversalOrganizationalUnitName,
) -> ASN1Result<X690Element> {
    _encode_UniversalOrBMPString(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalOrBMPString{INTEGER:ub-string-length} ::= SET {
///   character-encoding     CHOICE {
///     two-octets             BMPString(SIZE (1..ub-string-length)),
///     four-octets            UniversalString(SIZE (1..ub-string-length))},
///   iso-639-language-code  PrintableString(SIZE (2 | 5)) OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UniversalOrBMPString {
    pub character_encoding: UniversalOrBMPString_character_encoding,
    pub iso_639_language_code: OPTIONAL<PrintableString>,
}
impl UniversalOrBMPString {
    pub fn new(
        character_encoding: UniversalOrBMPString_character_encoding,
        iso_639_language_code: OPTIONAL<PrintableString>,
    ) -> Self {
        UniversalOrBMPString {
            character_encoding,
            iso_639_language_code,
        }
    }
}
impl TryFrom<X690Element> for UniversalOrBMPString {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UniversalOrBMPString(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UniversalOrBMPString {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UniversalOrBMPString(el)
    }
}

pub const _rctl1_components_for_UniversalOrBMPString: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "character-encoding",
        false,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::UNIVERSAL, 30)),
            &TagSelector::tag((TagClass::UNIVERSAL, 28)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "iso-639-language-code",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 19)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UniversalOrBMPString: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UniversalOrBMPString: &[ComponentSpec; 0] = &[];

pub fn _decode_UniversalOrBMPString(el: &X690Element) -> ASN1Result<UniversalOrBMPString> {
    |el_: &X690Element| -> ASN1Result<UniversalOrBMPString> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_UniversalOrBMPString,
            _eal_components_for_UniversalOrBMPString,
            _rctl2_components_for_UniversalOrBMPString,
            20,
        )?;
        let character_encoding = _decode_UniversalOrBMPString_character_encoding(
            _components.get("character-encoding").unwrap(),
        )?;
        let iso_639_language_code: OPTIONAL<PrintableString> =
            match _components.get("iso-639-language-code") {
                Some(c_) => Some(ber_decode_printable_string(c_)?),
                _ => None,
            };
        Ok(UniversalOrBMPString {
            character_encoding,
            iso_639_language_code,
        })
    }(&el)
}

pub fn _encode_UniversalOrBMPString(value_: &UniversalOrBMPString) -> ASN1Result<X690Element> {
    |value_: &UniversalOrBMPString| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_UniversalOrBMPString_character_encoding(
            &value_.character_encoding,
        )?);
        if let Some(v_) = &value_.iso_639_language_code {
            components_.push(ber_encode_printable_string(&v_)?);
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
/// pds-name EXTENSION-ATTRIBUTE ::= {
///                  PDSName
///   IDENTIFIED BY  7 }
/// ```
///
///
pub fn pds_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 7, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PDSName  ::=  PrintableString(SIZE (1..ub-pds-name-length))
/// ```
pub type PDSName = PrintableString; // PrintableString

pub fn _decode_PDSName(el: &X690Element) -> ASN1Result<PDSName> {
    ber_decode_printable_string(&el)
}

pub fn _encode_PDSName(value_: &PDSName) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// physical-delivery-country-name EXTENSION-ATTRIBUTE ::= {
///                  PhysicalDeliveryCountryName
///   IDENTIFIED BY  8 }
/// ```
///
///
pub fn physical_delivery_country_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 8, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PhysicalDeliveryCountryName  ::=  CHOICE {
///   x121-dcc-code         NumericString(SIZE (ub-country-name-numeric-length)),
///   iso-3166-alpha2-code  PrintableString(SIZE (ub-country-name-alpha-length)) }
/// ```
#[derive(Debug, Clone)]
pub enum PhysicalDeliveryCountryName {
    x121_dcc_code(NumericString),
    iso_3166_alpha2_code(PrintableString),
}

impl TryFrom<X690Element> for PhysicalDeliveryCountryName {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PhysicalDeliveryCountryName(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PhysicalDeliveryCountryName {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PhysicalDeliveryCountryName(el)
    }
}

pub fn _decode_PhysicalDeliveryCountryName(
    el: &X690Element,
) -> ASN1Result<PhysicalDeliveryCountryName> {
    |el: &X690Element| -> ASN1Result<PhysicalDeliveryCountryName> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 18) => Ok(PhysicalDeliveryCountryName::x121_dcc_code(
                ber_decode_numeric_string(&el)?,
            )),
            (TagClass::UNIVERSAL, 19) => Ok(PhysicalDeliveryCountryName::iso_3166_alpha2_code(
                ber_decode_printable_string(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_PhysicalDeliveryCountryName(
    value_: &PhysicalDeliveryCountryName,
) -> ASN1Result<X690Element> {
    |value: &PhysicalDeliveryCountryName| -> ASN1Result<X690Element> {
        match value {
            PhysicalDeliveryCountryName::x121_dcc_code(v) => ber_encode_numeric_string(&v),
            PhysicalDeliveryCountryName::iso_3166_alpha2_code(v) => ber_encode_printable_string(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// postal-code EXTENSION-ATTRIBUTE ::= {
///                  PostalCode
///   IDENTIFIED BY  9 }
/// ```
///
///
pub fn postal_code() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 9, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PostalCode  ::=  CHOICE {
///   numeric-code    NumericString(SIZE (1..ub-postal-code-length)),
///   printable-code  PrintableString(SIZE (1..ub-postal-code-length))
/// }
/// ```
#[derive(Debug, Clone)]
pub enum PostalCode {
    numeric_code(NumericString),
    printable_code(PrintableString),
}

impl TryFrom<X690Element> for PostalCode {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PostalCode(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PostalCode {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PostalCode(el)
    }
}

pub fn _decode_PostalCode(el: &X690Element) -> ASN1Result<PostalCode> {
    |el: &X690Element| -> ASN1Result<PostalCode> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 18) => {
                Ok(PostalCode::numeric_code(ber_decode_numeric_string(&el)?))
            }
            (TagClass::UNIVERSAL, 19) => Ok(PostalCode::printable_code(
                ber_decode_printable_string(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_PostalCode(value_: &PostalCode) -> ASN1Result<X690Element> {
    |value: &PostalCode| -> ASN1Result<X690Element> {
        match value {
            PostalCode::numeric_code(v) => ber_encode_numeric_string(&v),
            PostalCode::printable_code(v) => ber_encode_printable_string(&v),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// physical-delivery-office-name EXTENSION-ATTRIBUTE ::= {
///                  PhysicalDeliveryOfficeName
///   IDENTIFIED BY  10 }
/// ```
///
///
pub fn physical_delivery_office_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 10, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PhysicalDeliveryOfficeName  ::=  PDSParameter
/// ```
pub type PhysicalDeliveryOfficeName = PDSParameter; // DefinedType

pub fn _decode_PhysicalDeliveryOfficeName(
    el: &X690Element,
) -> ASN1Result<PhysicalDeliveryOfficeName> {
    _decode_PDSParameter(&el)
}

pub fn _encode_PhysicalDeliveryOfficeName(
    value_: &PhysicalDeliveryOfficeName,
) -> ASN1Result<X690Element> {
    _encode_PDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-physical-delivery-office-name EXTENSION-ATTRIBUTE ::= {
///                  UniversalPhysicalDeliveryOfficeName
///   IDENTIFIED BY  29 }
/// ```
///
///
pub fn universal_physical_delivery_office_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 29, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalPhysicalDeliveryOfficeName  ::=  UniversalPDSParameter
/// ```
pub type UniversalPhysicalDeliveryOfficeName = UniversalPDSParameter; // DefinedType

pub fn _decode_UniversalPhysicalDeliveryOfficeName(
    el: &X690Element,
) -> ASN1Result<UniversalPhysicalDeliveryOfficeName> {
    _decode_UniversalPDSParameter(&el)
}

pub fn _encode_UniversalPhysicalDeliveryOfficeName(
    value_: &UniversalPhysicalDeliveryOfficeName,
) -> ASN1Result<X690Element> {
    _encode_UniversalPDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// physical-delivery-office-number EXTENSION-ATTRIBUTE ::= {
///                  PhysicalDeliveryOfficeNumber
///   IDENTIFIED BY  11 }
/// ```
///
///
pub fn physical_delivery_office_number() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 11, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PhysicalDeliveryOfficeNumber  ::=  PDSParameter
/// ```
pub type PhysicalDeliveryOfficeNumber = PDSParameter; // DefinedType

pub fn _decode_PhysicalDeliveryOfficeNumber(
    el: &X690Element,
) -> ASN1Result<PhysicalDeliveryOfficeNumber> {
    _decode_PDSParameter(&el)
}

pub fn _encode_PhysicalDeliveryOfficeNumber(
    value_: &PhysicalDeliveryOfficeNumber,
) -> ASN1Result<X690Element> {
    _encode_PDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-physical-delivery-office-number EXTENSION-ATTRIBUTE ::= {
///                  UniversalPhysicalDeliveryOfficeNumber
///   IDENTIFIED BY  30 }
/// ```
///
///
pub fn universal_physical_delivery_office_number() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 30, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalPhysicalDeliveryOfficeNumber  ::=  UniversalPDSParameter
/// ```
pub type UniversalPhysicalDeliveryOfficeNumber = UniversalPDSParameter; // DefinedType

pub fn _decode_UniversalPhysicalDeliveryOfficeNumber(
    el: &X690Element,
) -> ASN1Result<UniversalPhysicalDeliveryOfficeNumber> {
    _decode_UniversalPDSParameter(&el)
}

pub fn _encode_UniversalPhysicalDeliveryOfficeNumber(
    value_: &UniversalPhysicalDeliveryOfficeNumber,
) -> ASN1Result<X690Element> {
    _encode_UniversalPDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extension-OR-address-components EXTENSION-ATTRIBUTE ::= {
///                  ExtensionORAddressComponents
///   IDENTIFIED BY  12 }
/// ```
///
///
pub fn extension_OR_address_components() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 12, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtensionORAddressComponents  ::=  PDSParameter
/// ```
pub type ExtensionORAddressComponents = PDSParameter; // DefinedType

pub fn _decode_ExtensionORAddressComponents(
    el: &X690Element,
) -> ASN1Result<ExtensionORAddressComponents> {
    _decode_PDSParameter(&el)
}

pub fn _encode_ExtensionORAddressComponents(
    value_: &ExtensionORAddressComponents,
) -> ASN1Result<X690Element> {
    _encode_PDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-extension-OR-address-components EXTENSION-ATTRIBUTE ::= {
///                  UniversalExtensionORAddressComponents
///   IDENTIFIED BY  31 }
/// ```
///
///
pub fn universal_extension_OR_address_components() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 31, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalExtensionORAddressComponents  ::=  UniversalPDSParameter
/// ```
pub type UniversalExtensionORAddressComponents = UniversalPDSParameter; // DefinedType

pub fn _decode_UniversalExtensionORAddressComponents(
    el: &X690Element,
) -> ASN1Result<UniversalExtensionORAddressComponents> {
    _decode_UniversalPDSParameter(&el)
}

pub fn _encode_UniversalExtensionORAddressComponents(
    value_: &UniversalExtensionORAddressComponents,
) -> ASN1Result<X690Element> {
    _encode_UniversalPDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// physical-delivery-personal-name EXTENSION-ATTRIBUTE ::= {
///                  PhysicalDeliveryPersonalName
///   IDENTIFIED BY  13 }
/// ```
///
///
pub fn physical_delivery_personal_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 13, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PhysicalDeliveryPersonalName  ::=  PDSParameter
/// ```
pub type PhysicalDeliveryPersonalName = PDSParameter; // DefinedType

pub fn _decode_PhysicalDeliveryPersonalName(
    el: &X690Element,
) -> ASN1Result<PhysicalDeliveryPersonalName> {
    _decode_PDSParameter(&el)
}

pub fn _encode_PhysicalDeliveryPersonalName(
    value_: &PhysicalDeliveryPersonalName,
) -> ASN1Result<X690Element> {
    _encode_PDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-physical-delivery-personal-name EXTENSION-ATTRIBUTE ::= {
///                  UniversalPhysicalDeliveryPersonalName
///   IDENTIFIED BY  32 }
/// ```
///
///
pub fn universal_physical_delivery_personal_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 32, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalPhysicalDeliveryPersonalName  ::=  UniversalPDSParameter
/// ```
pub type UniversalPhysicalDeliveryPersonalName = UniversalPDSParameter; // DefinedType

pub fn _decode_UniversalPhysicalDeliveryPersonalName(
    el: &X690Element,
) -> ASN1Result<UniversalPhysicalDeliveryPersonalName> {
    _decode_UniversalPDSParameter(&el)
}

pub fn _encode_UniversalPhysicalDeliveryPersonalName(
    value_: &UniversalPhysicalDeliveryPersonalName,
) -> ASN1Result<X690Element> {
    _encode_UniversalPDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// physical-delivery-organization-name EXTENSION-ATTRIBUTE ::= {
///                  PhysicalDeliveryOrganizationName
///   IDENTIFIED BY  14 }
/// ```
///
///
pub fn physical_delivery_organization_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 14, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PhysicalDeliveryOrganizationName  ::=  PDSParameter
/// ```
pub type PhysicalDeliveryOrganizationName = PDSParameter; // DefinedType

pub fn _decode_PhysicalDeliveryOrganizationName(
    el: &X690Element,
) -> ASN1Result<PhysicalDeliveryOrganizationName> {
    _decode_PDSParameter(&el)
}

pub fn _encode_PhysicalDeliveryOrganizationName(
    value_: &PhysicalDeliveryOrganizationName,
) -> ASN1Result<X690Element> {
    _encode_PDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-physical-delivery-organization-name EXTENSION-ATTRIBUTE ::= {
///                  UniversalPhysicalDeliveryOrganizationName
///   IDENTIFIED BY  33 }
/// ```
///
///
pub fn universal_physical_delivery_organization_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 33, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalPhysicalDeliveryOrganizationName  ::=  UniversalPDSParameter
/// ```
pub type UniversalPhysicalDeliveryOrganizationName = UniversalPDSParameter; // DefinedType

pub fn _decode_UniversalPhysicalDeliveryOrganizationName(
    el: &X690Element,
) -> ASN1Result<UniversalPhysicalDeliveryOrganizationName> {
    _decode_UniversalPDSParameter(&el)
}

pub fn _encode_UniversalPhysicalDeliveryOrganizationName(
    value_: &UniversalPhysicalDeliveryOrganizationName,
) -> ASN1Result<X690Element> {
    _encode_UniversalPDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extension-physical-delivery-address-components EXTENSION-ATTRIBUTE ::= {
///                  ExtensionPhysicalDeliveryAddressComponents
///   IDENTIFIED BY  15 }
/// ```
///
///
pub fn extension_physical_delivery_address_components() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 15, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtensionPhysicalDeliveryAddressComponents  ::=  PDSParameter
/// ```
pub type ExtensionPhysicalDeliveryAddressComponents = PDSParameter; // DefinedType

pub fn _decode_ExtensionPhysicalDeliveryAddressComponents(
    el: &X690Element,
) -> ASN1Result<ExtensionPhysicalDeliveryAddressComponents> {
    _decode_PDSParameter(&el)
}

pub fn _encode_ExtensionPhysicalDeliveryAddressComponents(
    value_: &ExtensionPhysicalDeliveryAddressComponents,
) -> ASN1Result<X690Element> {
    _encode_PDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-extension-physical-delivery-address-components EXTENSION-ATTRIBUTE ::= {
///                  UniversalExtensionPhysicalDeliveryAddressComponents
///   IDENTIFIED BY  34 }
/// ```
///
///
pub fn universal_extension_physical_delivery_address_components() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 34, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalExtensionPhysicalDeliveryAddressComponents  ::=  UniversalPDSParameter
/// ```
pub type UniversalExtensionPhysicalDeliveryAddressComponents = UniversalPDSParameter; // DefinedType

pub fn _decode_UniversalExtensionPhysicalDeliveryAddressComponents(
    el: &X690Element,
) -> ASN1Result<UniversalExtensionPhysicalDeliveryAddressComponents> {
    _decode_UniversalPDSParameter(&el)
}

pub fn _encode_UniversalExtensionPhysicalDeliveryAddressComponents(
    value_: &UniversalExtensionPhysicalDeliveryAddressComponents,
) -> ASN1Result<X690Element> {
    _encode_UniversalPDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// unformatted-postal-address EXTENSION-ATTRIBUTE ::= {
///                  UnformattedPostalAddress
///   IDENTIFIED BY  16 }
/// ```
///
///
pub fn unformatted_postal_address() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 16, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnformattedPostalAddress ::= SET {
///   printable-address SEQUENCE SIZE (1..ub-pds-physical-address-lines) OF
///     PrintableString (SIZE (1..ub-pds-parameter-length)) OPTIONAL,
///   teletex-string    TeletexString(SIZE (1..ub-unformatted-address-length)) OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UnformattedPostalAddress {
    pub printable_address: OPTIONAL<Vec<PrintableString>>,
    pub teletex_string: OPTIONAL<TeletexString>,
}
impl UnformattedPostalAddress {
    pub fn new(
        printable_address: OPTIONAL<Vec<PrintableString>>,
        teletex_string: OPTIONAL<TeletexString>,
    ) -> Self {
        UnformattedPostalAddress {
            printable_address,
            teletex_string,
        }
    }
}
impl Default for UnformattedPostalAddress {
    fn default() -> Self {
        UnformattedPostalAddress {
            printable_address: None,
            teletex_string: None,
        }
    }
}
impl TryFrom<X690Element> for UnformattedPostalAddress {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UnformattedPostalAddress(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UnformattedPostalAddress {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UnformattedPostalAddress(el)
    }
}

pub const _rctl1_components_for_UnformattedPostalAddress: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "printable-address",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "teletex-string",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 20)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UnformattedPostalAddress: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UnformattedPostalAddress: &[ComponentSpec; 0] = &[];

pub fn _decode_UnformattedPostalAddress(el: &X690Element) -> ASN1Result<UnformattedPostalAddress> {
    |el_: &X690Element| -> ASN1Result<UnformattedPostalAddress> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_UnformattedPostalAddress,
            _eal_components_for_UnformattedPostalAddress,
            _rctl2_components_for_UnformattedPostalAddress,
            20,
        )?;
        let printable_address: OPTIONAL<Vec<PrintableString>> =
            match _components.get("printable-address") {
                Some(c_) => Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<PrintableString>> {
                        let elements = match el.value.borrow() {
                            X690Encoding::Constructed(children) => children,
                            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                        };
                        let mut items: SEQUENCE_OF<PrintableString> =
                            Vec::with_capacity(elements.len());
                        for el in elements {
                            items.push(ber_decode_printable_string(el)?);
                        }
                        Ok(items)
                    }(c_)?,
                ),
                _ => None,
            };
        let teletex_string: OPTIONAL<TeletexString> = match _components.get("teletex-string") {
            Some(c_) => Some(ber_decode_t61_string(c_)?),
            _ => None,
        };
        Ok(UnformattedPostalAddress {
            printable_address,
            teletex_string,
        })
    }(&el)
}

pub fn _encode_UnformattedPostalAddress(
    value_: &UnformattedPostalAddress,
) -> ASN1Result<X690Element> {
    |value_: &UnformattedPostalAddress| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.printable_address {
            components_.push(
                |value_: &SEQUENCE_OF<PrintableString>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(ber_encode_printable_string(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_)?,
            );
        }
        if let Some(v_) = &value_.teletex_string {
            components_.push(ber_encode_t61_string(&v_)?);
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
/// universal-unformatted-postal-address EXTENSION-ATTRIBUTE ::= {
///                  UniversalUnformattedPostalAddress
///   IDENTIFIED BY  35 }
/// ```
///
///
pub fn universal_unformatted_postal_address() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 35, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalUnformattedPostalAddress  ::=
///   UniversalOrBMPString{ub-unformatted-address-length}
/// ```
pub type UniversalUnformattedPostalAddress = UniversalOrBMPString; // DefinedType

pub fn _decode_UniversalUnformattedPostalAddress(
    el: &X690Element,
) -> ASN1Result<UniversalUnformattedPostalAddress> {
    _decode_UniversalOrBMPString(&el)
}

pub fn _encode_UniversalUnformattedPostalAddress(
    value_: &UniversalUnformattedPostalAddress,
) -> ASN1Result<X690Element> {
    _encode_UniversalOrBMPString(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// street-address EXTENSION-ATTRIBUTE ::= {
///                  StreetAddress
///   IDENTIFIED BY  17 }
/// ```
///
///
pub fn street_address() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 17, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// StreetAddress  ::=  PDSParameter
/// ```
pub type StreetAddress = PDSParameter; // DefinedType

pub fn _decode_StreetAddress(el: &X690Element) -> ASN1Result<StreetAddress> {
    _decode_PDSParameter(&el)
}

pub fn _encode_StreetAddress(value_: &StreetAddress) -> ASN1Result<X690Element> {
    _encode_PDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-street-address EXTENSION-ATTRIBUTE ::= {
///                  UniversalStreetAddress
///   IDENTIFIED BY  36 }
/// ```
///
///
pub fn universal_street_address() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 36, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalStreetAddress  ::=  UniversalPDSParameter
/// ```
pub type UniversalStreetAddress = UniversalPDSParameter; // DefinedType

pub fn _decode_UniversalStreetAddress(el: &X690Element) -> ASN1Result<UniversalStreetAddress> {
    _decode_UniversalPDSParameter(&el)
}

pub fn _encode_UniversalStreetAddress(value_: &UniversalStreetAddress) -> ASN1Result<X690Element> {
    _encode_UniversalPDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// post-office-box-address EXTENSION-ATTRIBUTE ::= {
///                  PostOfficeBoxAddress
///   IDENTIFIED BY  18 }
/// ```
///
///
pub fn post_office_box_address() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 18, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PostOfficeBoxAddress  ::=  PDSParameter
/// ```
pub type PostOfficeBoxAddress = PDSParameter; // DefinedType

pub fn _decode_PostOfficeBoxAddress(el: &X690Element) -> ASN1Result<PostOfficeBoxAddress> {
    _decode_PDSParameter(&el)
}

pub fn _encode_PostOfficeBoxAddress(value_: &PostOfficeBoxAddress) -> ASN1Result<X690Element> {
    _encode_PDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-post-office-box-address EXTENSION-ATTRIBUTE ::= {
///                  UniversalPostOfficeBoxAddress
///   IDENTIFIED BY  37 }
/// ```
///
///
pub fn universal_post_office_box_address() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 37, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalPostOfficeBoxAddress  ::=  UniversalPDSParameter
/// ```
pub type UniversalPostOfficeBoxAddress = UniversalPDSParameter; // DefinedType

pub fn _decode_UniversalPostOfficeBoxAddress(
    el: &X690Element,
) -> ASN1Result<UniversalPostOfficeBoxAddress> {
    _decode_UniversalPDSParameter(&el)
}

pub fn _encode_UniversalPostOfficeBoxAddress(
    value_: &UniversalPostOfficeBoxAddress,
) -> ASN1Result<X690Element> {
    _encode_UniversalPDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// poste-restante-address EXTENSION-ATTRIBUTE ::= {
///                  PosteRestanteAddress
///   IDENTIFIED BY  19 }
/// ```
///
///
pub fn poste_restante_address() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 19, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PosteRestanteAddress  ::=  PDSParameter
/// ```
pub type PosteRestanteAddress = PDSParameter; // DefinedType

pub fn _decode_PosteRestanteAddress(el: &X690Element) -> ASN1Result<PosteRestanteAddress> {
    _decode_PDSParameter(&el)
}

pub fn _encode_PosteRestanteAddress(value_: &PosteRestanteAddress) -> ASN1Result<X690Element> {
    _encode_PDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-poste-restante-address EXTENSION-ATTRIBUTE ::= {
///                  UniversalPosteRestanteAddress
///   IDENTIFIED BY  38 }
/// ```
///
///
pub fn universal_poste_restante_address() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 38, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalPosteRestanteAddress  ::=  UniversalPDSParameter
/// ```
pub type UniversalPosteRestanteAddress = UniversalPDSParameter; // DefinedType

pub fn _decode_UniversalPosteRestanteAddress(
    el: &X690Element,
) -> ASN1Result<UniversalPosteRestanteAddress> {
    _decode_UniversalPDSParameter(&el)
}

pub fn _encode_UniversalPosteRestanteAddress(
    value_: &UniversalPosteRestanteAddress,
) -> ASN1Result<X690Element> {
    _encode_UniversalPDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// unique-postal-name EXTENSION-ATTRIBUTE ::= {
///                  UniquePostalName
///   IDENTIFIED BY  20 }
/// ```
///
///
pub fn unique_postal_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 20, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniquePostalName  ::=  PDSParameter
/// ```
pub type UniquePostalName = PDSParameter; // DefinedType

pub fn _decode_UniquePostalName(el: &X690Element) -> ASN1Result<UniquePostalName> {
    _decode_PDSParameter(&el)
}

pub fn _encode_UniquePostalName(value_: &UniquePostalName) -> ASN1Result<X690Element> {
    _encode_PDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-unique-postal-name EXTENSION-ATTRIBUTE ::= {
///                  UniversalUniquePostalName
///   IDENTIFIED BY  39 }
/// ```
///
///
pub fn universal_unique_postal_name() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 39, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalUniquePostalName  ::=  UniversalPDSParameter
/// ```
pub type UniversalUniquePostalName = UniversalPDSParameter; // DefinedType

pub fn _decode_UniversalUniquePostalName(
    el: &X690Element,
) -> ASN1Result<UniversalUniquePostalName> {
    _decode_UniversalPDSParameter(&el)
}

pub fn _encode_UniversalUniquePostalName(
    value_: &UniversalUniquePostalName,
) -> ASN1Result<X690Element> {
    _encode_UniversalPDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// local-postal-attributes EXTENSION-ATTRIBUTE ::= {
///                  LocalPostalAttributes
///   IDENTIFIED BY  21 }
/// ```
///
///
pub fn local_postal_attributes() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 21, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LocalPostalAttributes  ::=  PDSParameter
/// ```
pub type LocalPostalAttributes = PDSParameter; // DefinedType

pub fn _decode_LocalPostalAttributes(el: &X690Element) -> ASN1Result<LocalPostalAttributes> {
    _decode_PDSParameter(&el)
}

pub fn _encode_LocalPostalAttributes(value_: &LocalPostalAttributes) -> ASN1Result<X690Element> {
    _encode_PDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// universal-local-postal-attributes EXTENSION-ATTRIBUTE ::= {
///                  UniversalLocalPostalAttributes
///   IDENTIFIED BY  40 }
/// ```
///
///
pub fn universal_local_postal_attributes() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 40, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalLocalPostalAttributes  ::=  UniversalPDSParameter
/// ```
pub type UniversalLocalPostalAttributes = UniversalPDSParameter; // DefinedType

pub fn _decode_UniversalLocalPostalAttributes(
    el: &X690Element,
) -> ASN1Result<UniversalLocalPostalAttributes> {
    _decode_UniversalPDSParameter(&el)
}

pub fn _encode_UniversalLocalPostalAttributes(
    value_: &UniversalLocalPostalAttributes,
) -> ASN1Result<X690Element> {
    _encode_UniversalPDSParameter(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PDSParameter ::= SET {
///   printable-string  PrintableString(SIZE (1..ub-pds-parameter-length)) OPTIONAL,
///   teletex-string    TeletexString(SIZE (1..ub-pds-parameter-length)) OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct PDSParameter {
    pub printable_string: OPTIONAL<PrintableString>,
    pub teletex_string: OPTIONAL<TeletexString>,
}
impl PDSParameter {
    pub fn new(
        printable_string: OPTIONAL<PrintableString>,
        teletex_string: OPTIONAL<TeletexString>,
    ) -> Self {
        PDSParameter {
            printable_string,
            teletex_string,
        }
    }
}
impl Default for PDSParameter {
    fn default() -> Self {
        PDSParameter {
            printable_string: None,
            teletex_string: None,
        }
    }
}
impl TryFrom<X690Element> for PDSParameter {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PDSParameter(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PDSParameter {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PDSParameter(el)
    }
}

pub const _rctl1_components_for_PDSParameter: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "printable-string",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 19)),
        None,
        None,
    ),
    ComponentSpec::new(
        "teletex-string",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 20)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PDSParameter: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PDSParameter: &[ComponentSpec; 0] = &[];

pub fn _decode_PDSParameter(el: &X690Element) -> ASN1Result<PDSParameter> {
    |el_: &X690Element| -> ASN1Result<PDSParameter> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_PDSParameter,
            _eal_components_for_PDSParameter,
            _rctl2_components_for_PDSParameter,
            20,
        )?;
        let printable_string: OPTIONAL<PrintableString> = match _components.get("printable-string")
        {
            Some(c_) => Some(ber_decode_printable_string(c_)?),
            _ => None,
        };
        let teletex_string: OPTIONAL<TeletexString> = match _components.get("teletex-string") {
            Some(c_) => Some(ber_decode_t61_string(c_)?),
            _ => None,
        };
        Ok(PDSParameter {
            printable_string,
            teletex_string,
        })
    }(&el)
}

pub fn _encode_PDSParameter(value_: &PDSParameter) -> ASN1Result<X690Element> {
    |value_: &PDSParameter| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.printable_string {
            components_.push(ber_encode_printable_string(&v_)?);
        }
        if let Some(v_) = &value_.teletex_string {
            components_.push(ber_encode_t61_string(&v_)?);
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
/// UniversalPDSParameter  ::=  UniversalOrBMPString{ub-pds-parameter-length}
/// ```
pub type UniversalPDSParameter = UniversalOrBMPString; // DefinedType

pub fn _decode_UniversalPDSParameter(el: &X690Element) -> ASN1Result<UniversalPDSParameter> {
    _decode_UniversalOrBMPString(&el)
}

pub fn _encode_UniversalPDSParameter(value_: &UniversalPDSParameter) -> ASN1Result<X690Element> {
    _encode_UniversalOrBMPString(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extended-network-address EXTENSION-ATTRIBUTE ::= {
///                  ExtendedNetworkAddress
///   IDENTIFIED BY  22 }
/// ```
///
///
pub fn extended_network_address() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 22, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendedNetworkAddress  ::=  CHOICE {
///   e163-4-address    SEQUENCE {
///     number       [0]  NumericString(SIZE (1..ub-e163-4-number-length)),
///     sub-address  [1]  NumericString(SIZE (1..ub-e163-4-sub-address-length))
///                   OPTIONAL},
///   psap-address [0]  PresentationAddress }
/// ```
#[derive(Debug, Clone)]
pub enum ExtendedNetworkAddress {
    e163_4_address(ExtendedNetworkAddress_e163_4_address),
    psap_address(PresentationAddress),
}

impl TryFrom<X690Element> for ExtendedNetworkAddress {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedNetworkAddress(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ExtendedNetworkAddress {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedNetworkAddress(el)
    }
}

pub fn _decode_ExtendedNetworkAddress(el: &X690Element) -> ASN1Result<ExtendedNetworkAddress> {
    |el: &X690Element| -> ASN1Result<ExtendedNetworkAddress> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(ExtendedNetworkAddress::e163_4_address(
                _decode_ExtendedNetworkAddress_e163_4_address(&el)?,
            )),
            (TagClass::CONTEXT, 0) => Ok(ExtendedNetworkAddress::psap_address(
                _decode_PresentationAddress(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_ExtendedNetworkAddress(value_: &ExtendedNetworkAddress) -> ASN1Result<X690Element> {
    |value: &ExtendedNetworkAddress| -> ASN1Result<X690Element> {
        match value {
            ExtendedNetworkAddress::e163_4_address(v) => {
                _encode_ExtendedNetworkAddress_e163_4_address(&v)
            }
            ExtendedNetworkAddress::psap_address(v) => {
                |v_1: &PresentationAddress| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_PresentationAddress(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// terminal-type EXTENSION-ATTRIBUTE ::= {
///                  TerminalType
///   IDENTIFIED BY  23 }
/// ```
///
///
pub fn terminal_type() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 23, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TerminalType  ::=  INTEGER {
///   telex(3), teletex(4), g3-facsimile(5), g4-facsimile(6), ia5-terminal(7),
///   videotex(8)}(0..ub-integer-options)
/// ```
pub type TerminalType = INTEGER;

pub const TerminalType_telex: TerminalType = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const TerminalType_teletex: TerminalType = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const TerminalType_g3_facsimile: TerminalType = 5; /* LONG_NAMED_INTEGER_VALUE */

pub const TerminalType_g4_facsimile: TerminalType = 6; /* LONG_NAMED_INTEGER_VALUE */

pub const TerminalType_ia5_terminal: TerminalType = 7; /* LONG_NAMED_INTEGER_VALUE */

pub const TerminalType_videotex: TerminalType = 8; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_TerminalType(el: &X690Element) -> ASN1Result<TerminalType> {
    ber_decode_integer(&el)
}

pub fn _encode_TerminalType(value_: &TerminalType) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// teletex-domain-defined-attributes EXTENSION-ATTRIBUTE ::= {
///                  TeletexDomainDefinedAttributes
///   IDENTIFIED BY  6 }
/// ```
///
///
pub fn teletex_domain_defined_attributes() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 6, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TeletexDomainDefinedAttributes  ::=
///   SEQUENCE SIZE (1..ub-domain-defined-attributes) OF
///     TeletexDomainDefinedAttribute
/// ```
pub type TeletexDomainDefinedAttributes = Vec<TeletexDomainDefinedAttribute>; // SequenceOfType

pub fn _decode_TeletexDomainDefinedAttributes(
    el: &X690Element,
) -> ASN1Result<TeletexDomainDefinedAttributes> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<TeletexDomainDefinedAttribute>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<TeletexDomainDefinedAttribute> =
            Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_TeletexDomainDefinedAttribute(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_TeletexDomainDefinedAttributes(
    value_: &TeletexDomainDefinedAttributes,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<TeletexDomainDefinedAttribute>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_TeletexDomainDefinedAttribute(&v)?);
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
/// TeletexDomainDefinedAttribute ::= SEQUENCE {
///   type   TeletexString(SIZE (1..ub-domain-defined-attribute-type-length)),
///   value  TeletexString(SIZE (1..ub-domain-defined-attribute-value-length)) }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TeletexDomainDefinedAttribute {
    pub type_: TeletexString,
    pub value: TeletexString,
}
impl TeletexDomainDefinedAttribute {
    pub fn new(type_: TeletexString, value: TeletexString) -> Self {
        TeletexDomainDefinedAttribute { type_, value }
    }
}
impl TryFrom<X690Element> for TeletexDomainDefinedAttribute {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TeletexDomainDefinedAttribute(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TeletexDomainDefinedAttribute {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TeletexDomainDefinedAttribute(el)
    }
}

pub const _rctl1_components_for_TeletexDomainDefinedAttribute: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 20)),
        None,
        None,
    ),
    ComponentSpec::new(
        "value",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 20)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TeletexDomainDefinedAttribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TeletexDomainDefinedAttribute: &[ComponentSpec; 0] = &[];

pub fn _decode_TeletexDomainDefinedAttribute(
    el: &X690Element,
) -> ASN1Result<TeletexDomainDefinedAttribute> {
    |el_: &X690Element| -> ASN1Result<TeletexDomainDefinedAttribute> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TeletexDomainDefinedAttribute,
            _eal_components_for_TeletexDomainDefinedAttribute,
            _rctl2_components_for_TeletexDomainDefinedAttribute,
        )?;
        let type_ = ber_decode_t61_string(_components.get("type").unwrap())?;
        let value = ber_decode_t61_string(_components.get("value").unwrap())?;
        Ok(TeletexDomainDefinedAttribute { type_, value })
    }(&el)
}

pub fn _encode_TeletexDomainDefinedAttribute(
    value_: &TeletexDomainDefinedAttribute,
) -> ASN1Result<X690Element> {
    |value_: &TeletexDomainDefinedAttribute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_t61_string(&value_.type_)?);
        components_.push(ber_encode_t61_string(&value_.value)?);
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
/// universal-domain-defined-attributes EXTENSION-ATTRIBUTE ::= {
///                  UniversalDomainDefinedAttributes
///   IDENTIFIED BY  28 }
/// ```
///
///
pub fn universal_domain_defined_attributes() -> EXTENSION_ATTRIBUTE {
    EXTENSION_ATTRIBUTE {
        id: 28, /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalDomainDefinedAttributes  ::=
///   SEQUENCE SIZE (1..ub-domain-defined-attributes) OF
///     UniversalDomainDefinedAttribute
/// ```
pub type UniversalDomainDefinedAttributes = Vec<UniversalDomainDefinedAttribute>; // SequenceOfType

pub fn _decode_UniversalDomainDefinedAttributes(
    el: &X690Element,
) -> ASN1Result<UniversalDomainDefinedAttributes> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<UniversalDomainDefinedAttribute>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<UniversalDomainDefinedAttribute> =
            Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_UniversalDomainDefinedAttribute(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_UniversalDomainDefinedAttributes(
    value_: &UniversalDomainDefinedAttributes,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<UniversalDomainDefinedAttribute>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_UniversalDomainDefinedAttribute(&v)?);
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
/// UniversalDomainDefinedAttribute ::= SEQUENCE {
///   type   UniversalOrBMPString{ub-domain-defined-attribute-type-length},
///   value  UniversalOrBMPString{ub-domain-defined-attribute-value-length} }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UniversalDomainDefinedAttribute {
    pub type_: UniversalOrBMPString,
    pub value: UniversalOrBMPString,
}
impl UniversalDomainDefinedAttribute {
    pub fn new(type_: UniversalOrBMPString, value: UniversalOrBMPString) -> Self {
        UniversalDomainDefinedAttribute { type_, value }
    }
}
impl TryFrom<X690Element> for UniversalDomainDefinedAttribute {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UniversalDomainDefinedAttribute(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UniversalDomainDefinedAttribute {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UniversalDomainDefinedAttribute(el)
    }
}

pub const _rctl1_components_for_UniversalDomainDefinedAttribute: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
    ComponentSpec::new(
        "value",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UniversalDomainDefinedAttribute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UniversalDomainDefinedAttribute: &[ComponentSpec; 0] = &[];

pub fn _decode_UniversalDomainDefinedAttribute(
    el: &X690Element,
) -> ASN1Result<UniversalDomainDefinedAttribute> {
    |el_: &X690Element| -> ASN1Result<UniversalDomainDefinedAttribute> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UniversalDomainDefinedAttribute,
            _eal_components_for_UniversalDomainDefinedAttribute,
            _rctl2_components_for_UniversalDomainDefinedAttribute,
        )?;
        let type_ = _decode_UniversalOrBMPString(_components.get("type").unwrap())?;
        let value = _decode_UniversalOrBMPString(_components.get("value").unwrap())?;
        Ok(UniversalDomainDefinedAttribute { type_, value })
    }(&el)
}

pub fn _encode_UniversalDomainDefinedAttribute(
    value_: &UniversalDomainDefinedAttribute,
) -> ASN1Result<X690Element> {
    |value_: &UniversalDomainDefinedAttribute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(_encode_UniversalOrBMPString(&value_.type_)?);
        components_.push(_encode_UniversalOrBMPString(&value_.value)?);
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
/// ub-integer-options                        INTEGER ::= 256
/// ```
///
///
pub const ub_integer_options: INTEGER = 256;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-e163-4-number-length                   INTEGER ::= 15
/// ```
///
///
pub const ub_e163_4_number_length: INTEGER = 15;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-e163-4-sub-address-length              INTEGER ::= 40
/// ```
///
///
pub const ub_e163_4_sub_address_length: INTEGER = 40;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-unformatted-address-length             INTEGER ::= 180
/// ```
///
///
pub const ub_unformatted_address_length: INTEGER = 180;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-pds-parameter-length                   INTEGER ::= 30
/// ```
///
///
pub const ub_pds_parameter_length: INTEGER = 30;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-pds-physical-address-lines             INTEGER ::= 6
/// ```
///
///
pub const ub_pds_physical_address_lines: INTEGER = 6;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-postal-code-length                     INTEGER ::= 16
/// ```
///
///
pub const ub_postal_code_length: INTEGER = 16;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-pds-name-length                        INTEGER ::= 16
/// ```
///
///
pub const ub_pds_name_length: INTEGER = 16;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-universal-surname-length               INTEGER ::= 64
/// ```
///
///
pub const ub_universal_surname_length: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-universal-given-name-length            INTEGER ::= 40
/// ```
///
///
pub const ub_universal_given_name_length: INTEGER = 40;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-universal-initials-length              INTEGER ::= 16
/// ```
///
///
pub const ub_universal_initials_length: INTEGER = 16;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-universal-generation-qualifier-length  INTEGER ::= 16
/// ```
///
///
pub const ub_universal_generation_qualifier_length: INTEGER = 16;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-common-name-length                     INTEGER ::= 64
/// ```
///
///
pub const ub_common_name_length: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-extension-attributes                   INTEGER ::= 256
/// ```
///
///
pub const ub_extension_attributes: INTEGER = 256;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-domain-defined-attribute-type-length   INTEGER ::= 8
/// ```
///
///
pub const ub_domain_defined_attribute_type_length: INTEGER = 8;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-domain-defined-attribute-value-length  INTEGER ::= 128
/// ```
///
///
pub const ub_domain_defined_attribute_value_length: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-domain-defined-attributes              INTEGER ::= 4
/// ```
///
///
pub const ub_domain_defined_attributes: INTEGER = 4;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-organizational-unit-name-length        INTEGER ::= 32
/// ```
///
///
pub const ub_organizational_unit_name_length: INTEGER = 32;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-organizational-units                   INTEGER ::= 4
/// ```
///
///
pub const ub_organizational_units: INTEGER = 4;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-generation-qualifier-length            INTEGER ::= 3
/// ```
///
///
pub const ub_generation_qualifier_length: INTEGER = 3;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-initials-length                        INTEGER ::= 5
/// ```
///
///
pub const ub_initials_length: INTEGER = 5;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-given-name-length                      INTEGER ::= 16
/// ```
///
///
pub const ub_given_name_length: INTEGER = 16;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-surname-length                         INTEGER ::= 40
/// ```
///
///
pub const ub_surname_length: INTEGER = 40;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-numeric-user-id-length                 INTEGER ::= 32
/// ```
///
///
pub const ub_numeric_user_id_length: INTEGER = 32;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-organization-name-length               INTEGER ::= 64
/// ```
///
///
pub const ub_organization_name_length: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-terminal-id-length                     INTEGER ::= 24
/// ```
///
///
pub const ub_terminal_id_length: INTEGER = 24;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-x121-address-length                    INTEGER ::= 16
/// ```
///
///
pub const ub_x121_address_length: INTEGER = 16;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-domain-name-length                     INTEGER ::= 16
/// ```
///
///
pub const ub_domain_name_length: INTEGER = 16;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-country-name-alpha-length              INTEGER ::= 2
/// ```
///
///
pub const ub_country_name_alpha_length: INTEGER = 2;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-country-name-numeric-length            INTEGER ::= 3
/// ```
///
///
pub const ub_country_name_numeric_length: INTEGER = 3;

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniversalOrBMPString-character-encoding ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum UniversalOrBMPString_character_encoding {
    two_octets(BMPString),
    four_octets(UniversalString),
}

impl TryFrom<X690Element> for UniversalOrBMPString_character_encoding {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UniversalOrBMPString_character_encoding(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UniversalOrBMPString_character_encoding {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UniversalOrBMPString_character_encoding(el)
    }
}

pub fn _decode_UniversalOrBMPString_character_encoding(
    el: &X690Element,
) -> ASN1Result<UniversalOrBMPString_character_encoding> {
    |el: &X690Element| -> ASN1Result<UniversalOrBMPString_character_encoding> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 30) => Ok(UniversalOrBMPString_character_encoding::two_octets(
                ber_decode_bmp_string(&el)?,
            )),
            (TagClass::UNIVERSAL, 28) => Ok(UniversalOrBMPString_character_encoding::four_octets(
                ber_decode_universal_string(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_UniversalOrBMPString_character_encoding(
    value_: &UniversalOrBMPString_character_encoding,
) -> ASN1Result<X690Element> {
    |value: &UniversalOrBMPString_character_encoding| -> ASN1Result<X690Element> {
        match value {
            UniversalOrBMPString_character_encoding::two_octets(v) => ber_encode_bmp_string(&v),
            UniversalOrBMPString_character_encoding::four_octets(v) => {
                ber_encode_universal_string(&v)
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtendedNetworkAddress-e163-4-address ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ExtendedNetworkAddress_e163_4_address {
    pub number: NumericString,
    pub sub_address: OPTIONAL<NumericString>,
}
impl ExtendedNetworkAddress_e163_4_address {
    pub fn new(number: NumericString, sub_address: OPTIONAL<NumericString>) -> Self {
        ExtendedNetworkAddress_e163_4_address {
            number,
            sub_address,
        }
    }
}
impl TryFrom<X690Element> for ExtendedNetworkAddress_e163_4_address {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedNetworkAddress_e163_4_address(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ExtendedNetworkAddress_e163_4_address {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ExtendedNetworkAddress_e163_4_address(el)
    }
}

pub const _rctl1_components_for_ExtendedNetworkAddress_e163_4_address: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "number",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sub-address",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ExtendedNetworkAddress_e163_4_address: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ExtendedNetworkAddress_e163_4_address: &[ComponentSpec; 0] = &[];

pub fn _decode_ExtendedNetworkAddress_e163_4_address(
    el: &X690Element,
) -> ASN1Result<ExtendedNetworkAddress_e163_4_address> {
    |el_: &X690Element| -> ASN1Result<ExtendedNetworkAddress_e163_4_address> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ExtendedNetworkAddress_e163_4_address,
            _eal_components_for_ExtendedNetworkAddress_e163_4_address,
            _rctl2_components_for_ExtendedNetworkAddress_e163_4_address,
        )?;
        let number = |el: &X690Element| -> ASN1Result<NumericString> {
            Ok(ber_decode_numeric_string(&el.inner()?)?)
        }(_components.get("number").unwrap())?;
        let sub_address: OPTIONAL<NumericString> = match _components.get("sub-address") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<NumericString> {
                Ok(ber_decode_numeric_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(ExtendedNetworkAddress_e163_4_address {
            number,
            sub_address,
        })
    }(&el)
}

pub fn _encode_ExtendedNetworkAddress_e163_4_address(
    value_: &ExtendedNetworkAddress_e163_4_address,
) -> ASN1Result<X690Element> {
    |value_: &ExtendedNetworkAddress_e163_4_address| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(|v_1: &NumericString| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_numeric_string(
                    &v_1,
                )?))),
            ))
        }(&value_.number)?);
        if let Some(v_) = &value_.sub_address {
            components_.push(|v_1: &NumericString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_numeric_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
}
