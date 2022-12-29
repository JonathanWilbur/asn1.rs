#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # EnhancedSecurity
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `EnhancedSecurity`.
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
use crate::InformationFramework::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// OPTIONALLY-PROTECTED{Type}  ::=  CHOICE {
///   unsigned       Type,
///   signed         SIGNED{Type} }
/// ```
#[derive(Debug, Clone)]
pub enum OPTIONALLY_PROTECTED<Type> {
    unsigned(Type),
    signed(SIGNED<Type>),
}

pub fn _decode_OPTIONALLY_PROTECTED<Type: 'static>(
    _decode_Type: fn(&X690Element) -> ASN1Result<Type>,
    e: &X690Element,
) -> ASN1Result<OPTIONALLY_PROTECTED<Type>> {
    |el: &X690Element| -> ASN1Result<OPTIONALLY_PROTECTED<Type>> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(OPTIONALLY_PROTECTED::signed(_decode_SIGNED::<Type>(
                _decode_Type,
                &el,
            )?)),
            _ => Ok(OPTIONALLY_PROTECTED::unsigned(_decode_Type(&el)?)),
        }
    }(&e)
}

pub fn _encode_OPTIONALLY_PROTECTED<Type>(
    _encode_Type: fn(&Type) -> ASN1Result<X690Element>,
    v: &OPTIONALLY_PROTECTED<Type>,
) -> ASN1Result<X690Element> {
    |value: &OPTIONALLY_PROTECTED<Type>| -> ASN1Result<X690Element> {
        match value {
            OPTIONALLY_PROTECTED::unsigned(v) => _encode_Type(&v),
            OPTIONALLY_PROTECTED::signed(v) => _encode_SIGNED::<Type>(_encode_Type, &v),
        }
    }(&v)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OPTIONALLY-PROTECTED-SEQ{Type}  ::=  CHOICE {
///   unsigned       Type,
///   signed    [0]  SIGNED{Type} }
/// ```
#[derive(Debug, Clone)]
pub enum OPTIONALLY_PROTECTED_SEQ<Type> {
    unsigned(Type),
    signed(SIGNED<Type>),
}

pub fn _decode_OPTIONALLY_PROTECTED_SEQ<Type: 'static>(
    _decode_Type: fn(&X690Element) -> ASN1Result<Type>,
    e: &X690Element,
) -> ASN1Result<OPTIONALLY_PROTECTED_SEQ<Type>> {
    |el: &X690Element| -> ASN1Result<OPTIONALLY_PROTECTED_SEQ<Type>> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(OPTIONALLY_PROTECTED_SEQ::signed(_decode_SIGNED::<Type>(
                _decode_Type,
                &el,
            )?)),
            _ => Ok(OPTIONALLY_PROTECTED_SEQ::unsigned(_decode_Type(&el)?)),
        }
    }(&e)
}

pub fn _encode_OPTIONALLY_PROTECTED_SEQ<Type>(
    _encode_Type: fn(&Type) -> ASN1Result<X690Element>,
    v: &OPTIONALLY_PROTECTED_SEQ<Type>,
) -> ASN1Result<X690Element> {
    |value: &OPTIONALLY_PROTECTED_SEQ<Type>| -> ASN1Result<X690Element> {
        match value {
            OPTIONALLY_PROTECTED_SEQ::unsigned(v) => _encode_Type(&v),
            OPTIONALLY_PROTECTED_SEQ::signed(v) => {
                |v_1: &SIGNED<Type>| -> ASN1Result<X690Element> {
                    let mut el_1 = _encode_SIGNED::<Type>(_encode_Type, &v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
        }
    }(&v)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeValueSecurityLabelContext CONTEXT ::= {
///   WITH SYNTAX    SignedSecurityLabel -- At most one security label context can
///                                      -- be assigned to an attribute value
///   ID             id-avc-attributeValueSecurityLabelContext }
/// ```
///
///
pub fn attributeValueSecurityLabelContext() -> CONTEXT {
    CONTEXT {
        id: id_avc_attributeValueSecurityLabelContext(), /* OBJECT_FIELD_SETTING */
        absentMatch: Some(true), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignedSecurityLabel  ::=  SIGNED{SignedSecurityLabelContent}
/// ```
pub type SignedSecurityLabel = SIGNED<SignedSecurityLabelContent>; // DefinedType

pub fn _decode_SignedSecurityLabel(el: &X690Element) -> ASN1Result<SignedSecurityLabel> {
    _decode_SIGNED::<SignedSecurityLabelContent>(_decode_SignedSecurityLabelContent, &el)
}

pub fn _encode_SignedSecurityLabel(value_: &SignedSecurityLabel) -> ASN1Result<X690Element> {
    _encode_SIGNED::<SignedSecurityLabelContent>(_encode_SignedSecurityLabelContent, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SignedSecurityLabelContent ::= SEQUENCE {
///   attHash        HASH{AttributeTypeAndValue},
///   issuer         Name OPTIONAL, -- name of labelling authority
///   keyIdentifier  KeyIdentifier OPTIONAL,
///   securityLabel  SecurityLabel,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SignedSecurityLabelContent {
    pub attHash: HASH,
    pub issuer: OPTIONAL<Name>,
    pub keyIdentifier: OPTIONAL<KeyIdentifier>,
    pub securityLabel: SecurityLabel,
    pub _unrecognized: Vec<X690Element>,
}
impl SignedSecurityLabelContent {
    pub fn new(
        attHash: HASH,
        issuer: OPTIONAL<Name>,
        keyIdentifier: OPTIONAL<KeyIdentifier>,
        securityLabel: SecurityLabel,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SignedSecurityLabelContent {
            attHash,
            issuer,
            keyIdentifier,
            securityLabel,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for SignedSecurityLabelContent {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SignedSecurityLabelContent(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SignedSecurityLabelContent {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SignedSecurityLabelContent(el)
    }
}

pub const _rctl1_components_for_SignedSecurityLabelContent: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "attHash",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "issuer",
        true,
        TagSelector::or(&[&TagSelector::tag((TagClass::UNIVERSAL, 16))]),
        None,
        None,
    ),
    ComponentSpec::new(
        "keyIdentifier",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityLabel",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SignedSecurityLabelContent: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SignedSecurityLabelContent: &[ComponentSpec; 0] = &[];

pub fn _decode_SignedSecurityLabelContent(
    el: &X690Element,
) -> ASN1Result<SignedSecurityLabelContent> {
    |el_: &X690Element| -> ASN1Result<SignedSecurityLabelContent> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SignedSecurityLabelContent,
            _eal_components_for_SignedSecurityLabelContent,
            _rctl2_components_for_SignedSecurityLabelContent,
        )?;
        let attHash = _decode_HASH(_components.get("attHash").unwrap())?;
        let issuer: OPTIONAL<Name> = match _components.get("issuer") {
            Some(c_) => Some(_decode_Name(c_)?),
            _ => None,
        };
        let keyIdentifier: OPTIONAL<KeyIdentifier> = match _components.get("keyIdentifier") {
            Some(c_) => Some(_decode_KeyIdentifier(c_)?),
            _ => None,
        };
        let securityLabel = _decode_SecurityLabel(_components.get("securityLabel").unwrap())?;
        Ok(SignedSecurityLabelContent {
            attHash,
            issuer,
            keyIdentifier,
            securityLabel,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SignedSecurityLabelContent(
    value_: &SignedSecurityLabelContent,
) -> ASN1Result<X690Element> {
    |value_: &SignedSecurityLabelContent| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        components_.push(_encode_HASH(&value_.attHash)?);
        if let Some(v_) = &value_.issuer {
            components_.push(_encode_Name(&v_)?);
        }
        if let Some(v_) = &value_.keyIdentifier {
            components_.push(_encode_KeyIdentifier(&v_)?);
        }
        components_.push(_encode_SecurityLabel(&value_.securityLabel)?);
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
/// SecurityLabel ::= SET {
///   security-policy-identifier  SecurityPolicyIdentifier OPTIONAL,
///   security-classification     SecurityClassification OPTIONAL,
///   privacy-mark                PrivacyMark OPTIONAL,
///   security-categories         SecurityCategories OPTIONAL,
///   ... }
///    (ALL EXCEPT ({ -- none, at least one component shall be present --}))
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SecurityLabel {
    pub security_policy_identifier: OPTIONAL<SecurityPolicyIdentifier>,
    pub security_classification: OPTIONAL<SecurityClassification>,
    pub privacy_mark: OPTIONAL<PrivacyMark>,
    pub security_categories: OPTIONAL<SecurityCategories>,
    pub _unrecognized: Vec<X690Element>,
}
impl SecurityLabel {
    pub fn new(
        security_policy_identifier: OPTIONAL<SecurityPolicyIdentifier>,
        security_classification: OPTIONAL<SecurityClassification>,
        privacy_mark: OPTIONAL<PrivacyMark>,
        security_categories: OPTIONAL<SecurityCategories>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SecurityLabel {
            security_policy_identifier,
            security_classification,
            privacy_mark,
            security_categories,
            _unrecognized,
        }
    }
}
impl Default for SecurityLabel {
    fn default() -> Self {
        SecurityLabel {
            security_policy_identifier: None,
            security_classification: None,
            privacy_mark: None,
            security_categories: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<X690Element> for SecurityLabel {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SecurityLabel(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SecurityLabel {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SecurityLabel(el)
    }
}

pub const _rctl1_components_for_SecurityLabel: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "security-policy-identifier",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "security-classification",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "privacy-mark",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 19)),
        None,
        None,
    ),
    ComponentSpec::new(
        "security-categories",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SecurityLabel: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SecurityLabel: &[ComponentSpec; 0] = &[];

pub fn _decode_SecurityLabel(el: &X690Element) -> ASN1Result<SecurityLabel> {
    |el_: &X690Element| -> ASN1Result<SecurityLabel> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_SecurityLabel,
            _eal_components_for_SecurityLabel,
            _rctl2_components_for_SecurityLabel,
            50,
        )?;
        let security_policy_identifier: OPTIONAL<SecurityPolicyIdentifier> =
            match _components.get("security-policy-identifier") {
                Some(c_) => Some(_decode_SecurityPolicyIdentifier(c_)?),
                _ => None,
            };
        let security_classification: OPTIONAL<SecurityClassification> =
            match _components.get("security-classification") {
                Some(c_) => Some(_decode_SecurityClassification(c_)?),
                _ => None,
            };
        let privacy_mark: OPTIONAL<PrivacyMark> = match _components.get("privacy-mark") {
            Some(c_) => Some(_decode_PrivacyMark(c_)?),
            _ => None,
        };
        let security_categories: OPTIONAL<SecurityCategories> =
            match _components.get("security-categories") {
                Some(c_) => Some(_decode_SecurityCategories(c_)?),
                _ => None,
            };
        Ok(SecurityLabel {
            security_policy_identifier,
            security_classification,
            privacy_mark,
            security_categories,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SecurityLabel(value_: &SecurityLabel) -> ASN1Result<X690Element> {
    |value_: &SecurityLabel| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        if let Some(v_) = &value_.security_policy_identifier {
            components_.push(_encode_SecurityPolicyIdentifier(&v_)?);
        }
        if let Some(v_) = &value_.security_classification {
            components_.push(_encode_SecurityClassification(&v_)?);
        }
        if let Some(v_) = &value_.privacy_mark {
            components_.push(_encode_PrivacyMark(&v_)?);
        }
        if let Some(v_) = &value_.security_categories {
            components_.push(_encode_SecurityCategories(&v_)?);
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
/// SecurityPolicyIdentifier  ::=  OBJECT IDENTIFIER
/// ```
pub type SecurityPolicyIdentifier = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_SecurityPolicyIdentifier(el: &X690Element) -> ASN1Result<SecurityPolicyIdentifier> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_SecurityPolicyIdentifier(
    value_: &SecurityPolicyIdentifier,
) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SecurityClassification  ::=  INTEGER {
///   unmarked      (0),
///   unclassified  (1),
///   restricted    (2),
///   confidential  (3),
///   secret        (4),
///   top-secret    (5)}
/// ```
pub type SecurityClassification = INTEGER;

pub const SecurityClassification_unmarked: i32 = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityClassification_unclassified: i32 = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityClassification_restricted: i32 = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityClassification_confidential: i32 = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityClassification_secret: i32 = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const SecurityClassification_top_secret: i32 = 5; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_SecurityClassification(el: &X690Element) -> ASN1Result<SecurityClassification> {
    ber_decode_integer(&el)
}

pub fn _encode_SecurityClassification(value_: &SecurityClassification) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PrivacyMark  ::=  PrintableString(SIZE (1..MAX))
/// ```
pub type PrivacyMark = PrintableString; // PrintableString

pub fn _decode_PrivacyMark(el: &X690Element) -> ASN1Result<PrivacyMark> {
    ber_decode_printable_string(&el)
}

pub fn _encode_PrivacyMark(value_: &PrivacyMark) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SecurityCategories  ::=  SET SIZE (1..MAX) OF SecurityCategory
/// ```
pub type SecurityCategories = Vec<SecurityCategory>; // SetOfType

pub fn _decode_SecurityCategories(el: &X690Element) -> ASN1Result<SecurityCategories> {
    |el: &X690Element| -> ASN1Result<SET_OF<SecurityCategory>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SET_OF<SecurityCategory> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_SecurityCategory(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_SecurityCategories(value_: &SecurityCategories) -> ASN1Result<X690Element> {
    |value_: &SET_OF<SecurityCategory>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_SecurityCategory(&v)?);
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
/// clearance ATTRIBUTE ::= {
///   WITH SYNTAX  Clearance
///   ID           id-at-clearance }
/// ```
///
///
pub fn clearance() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_at_clearance(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Clearance ::= SEQUENCE {
///   policyId            OBJECT IDENTIFIER,
///   classList           ClassList DEFAULT {unclassified},
///   securityCategories  SET SIZE (1..MAX) OF SecurityCategory OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Clearance {
    pub policyId: OBJECT_IDENTIFIER,
    pub classList: OPTIONAL<ClassList>,
    pub securityCategories: OPTIONAL<Vec<SecurityCategory>>,
    pub _unrecognized: Vec<X690Element>,
}
impl Clearance {
    pub fn new(
        policyId: OBJECT_IDENTIFIER,
        classList: OPTIONAL<ClassList>,
        securityCategories: OPTIONAL<Vec<SecurityCategory>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Clearance {
            policyId,
            classList,
            securityCategories,
            _unrecognized,
        }
    }
    pub fn _default_value_for_classList() -> ClassList {
        BIT_STRING::with_bits_set(&[ClassList_unclassified])
    }
}
impl TryFrom<X690Element> for Clearance {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Clearance(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Clearance {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Clearance(el)
    }
}

pub const _rctl1_components_for_Clearance: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "policyId",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "classList",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "securityCategories",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Clearance: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Clearance: &[ComponentSpec; 0] = &[];

pub fn _decode_Clearance(el: &X690Element) -> ASN1Result<Clearance> {
    |el_: &X690Element| -> ASN1Result<Clearance> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Clearance,
            _eal_components_for_Clearance,
            _rctl2_components_for_Clearance,
        )?;
        let policyId = ber_decode_object_identifier(_components.get("policyId").unwrap())?;
        let classList: OPTIONAL<ClassList> = match _components.get("classList") {
            Some(c_) => Some(_decode_ClassList(c_)?),
            _ => None,
        };
        let securityCategories: OPTIONAL<Vec<SecurityCategory>> =
            match _components.get("securityCategories") {
                Some(c_) => Some(|el: &X690Element| -> ASN1Result<SET_OF<SecurityCategory>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<SecurityCategory> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_SecurityCategory(el)?);
                    }
                    Ok(items)
                }(c_)?),
                _ => None,
            };
        Ok(Clearance {
            policyId,
            classList,
            securityCategories,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Clearance(value_: &Clearance) -> ASN1Result<X690Element> {
    |value_: &Clearance| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(ber_encode_object_identifier(&value_.policyId)?);
        if let Some(v_) = &value_.classList {
            if *v_ != Clearance::_default_value_for_classList() {
                components_.push(_encode_ClassList(&v_)?);
            }
        }
        if let Some(v_) = &value_.securityCategories {
            components_.push(
                |value_: &SET_OF<SecurityCategory>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_SecurityCategory(&v)?);
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
/// ClassList  ::=  BIT STRING {
///   unmarked      (0),
///   unclassified  (1),
///   restricted    (2),
///   confidential  (3),
///   secret        (4),
///   topSecret     (5)}
/// ```
pub type ClassList = BIT_STRING;

pub const ClassList_unmarked: BIT = 0; /* LONG_NAMED_BIT */

pub const ClassList_unclassified: BIT = 1; /* LONG_NAMED_BIT */

pub const ClassList_restricted: BIT = 2; /* LONG_NAMED_BIT */

pub const ClassList_confidential: BIT = 3; /* LONG_NAMED_BIT */

pub const ClassList_secret: BIT = 4; /* LONG_NAMED_BIT */

pub const ClassList_topSecret: BIT = 5; /* LONG_NAMED_BIT */

pub fn _decode_ClassList(el: &X690Element) -> ASN1Result<ClassList> {
    ber_decode_bit_string(&el)
}

pub fn _encode_ClassList(value_: &ClassList) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SecurityCategory ::= SEQUENCE {
///   type   [0]  SECURITY-CATEGORY.&id({SecurityCategoriesTable}),
///   value  [1]  EXPLICIT SECURITY-CATEGORY.&Type({SecurityCategoriesTable}{@type}),
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SecurityCategory {
    pub type_: OBJECT_IDENTIFIER,
    pub value: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl SecurityCategory {
    pub fn new(
        type_: OBJECT_IDENTIFIER,
        value: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        SecurityCategory {
            type_,
            value,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for SecurityCategory {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SecurityCategory(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SecurityCategory {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SecurityCategory(el)
    }
}

pub const _rctl1_components_for_SecurityCategory: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "value",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SecurityCategory: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SecurityCategory: &[ComponentSpec; 0] = &[];

pub fn _decode_SecurityCategory(el: &X690Element) -> ASN1Result<SecurityCategory> {
    |el_: &X690Element| -> ASN1Result<SecurityCategory> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SecurityCategory,
            _eal_components_for_SecurityCategory,
            _rctl2_components_for_SecurityCategory,
        )?;
        let type_ = ber_decode_object_identifier(_components.get("type").unwrap())?;
        let value =
            |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                _components.get("value").unwrap(),
            )?;
        Ok(SecurityCategory {
            type_,
            value,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_SecurityCategory(value_: &SecurityCategory) -> ASN1Result<X690Element> {
    |value_: &SecurityCategory| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            let mut el_1 = ber_encode_object_identifier(&v_1)?;
            el_1.tag_class = TagClass::CONTEXT;
            el_1.tag_number = 0;
            Ok(el_1)
        }(&value_.type_)?);
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
            ))
        }(&value_.value)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}

pub type SECURITY_CATEGORY = TYPE_IDENTIFIER;

/// ### ASN.1 Definition:
///
/// ```asn1
/// SecurityCategoriesTable SECURITY-CATEGORY ::= {...}
/// ```
///
///
pub fn SecurityCategoriesTable() -> Vec<SECURITY_CATEGORY> {
    Vec::from([])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeIntegrityInfo ATTRIBUTE ::= {
///   WITH SYNTAX   AttributeIntegrityInfo
///   SINGLE VALUE  TRUE
///   ID            id-at-attributeIntegrityInfo }
/// ```
///
///
pub fn attributeIntegrityInfo() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),          /* OBJECT_FIELD_SETTING */
        id: id_at_attributeIntegrityInfo(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeIntegrityInfo  ::=  SIGNED{AttributeIntegrityInfoContent}
/// ```
pub type AttributeIntegrityInfo = SIGNED<AttributeIntegrityInfoContent>; // DefinedType

pub fn _decode_AttributeIntegrityInfo(el: &X690Element) -> ASN1Result<AttributeIntegrityInfo> {
    _decode_SIGNED::<AttributeIntegrityInfoContent>(_decode_AttributeIntegrityInfoContent, &el)
}

pub fn _encode_AttributeIntegrityInfo(value_: &AttributeIntegrityInfo) -> ASN1Result<X690Element> {
    _encode_SIGNED::<AttributeIntegrityInfoContent>(_encode_AttributeIntegrityInfoContent, &value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeIntegrityInfoContent ::= SEQUENCE {
///   scope        Scope,           -- Identifies the attributes protected
///   signer       Signer OPTIONAL, -- Authority or data originators name
///   attribsHash  AttribsHash,     -- Hash value of protected attributes
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeIntegrityInfoContent {
    pub scope: Scope,
    pub signer: OPTIONAL<Signer>,
    pub attribsHash: AttribsHash,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeIntegrityInfoContent {
    pub fn new(
        scope: Scope,
        signer: OPTIONAL<Signer>,
        attribsHash: AttribsHash,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeIntegrityInfoContent {
            scope,
            signer,
            attribsHash,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttributeIntegrityInfoContent {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeIntegrityInfoContent(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeIntegrityInfoContent {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeIntegrityInfoContent(el)
    }
}

pub const _rctl1_components_for_AttributeIntegrityInfoContent: &[ComponentSpec; 3] = &[
    ComponentSpec::new("scope", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "signer",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "attribsHash",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeIntegrityInfoContent: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeIntegrityInfoContent: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeIntegrityInfoContent(
    el: &X690Element,
) -> ASN1Result<AttributeIntegrityInfoContent> {
    |el_: &X690Element| -> ASN1Result<AttributeIntegrityInfoContent> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeIntegrityInfoContent,
            _eal_components_for_AttributeIntegrityInfoContent,
            _rctl2_components_for_AttributeIntegrityInfoContent,
        )?;
        let scope = _decode_Scope(_components.get("scope").unwrap())?;
        let signer: OPTIONAL<Signer> = match _components.get("signer") {
            Some(c_) => Some(_decode_Signer(c_)?),
            _ => None,
        };
        let attribsHash = _decode_AttribsHash(_components.get("attribsHash").unwrap())?;
        Ok(AttributeIntegrityInfoContent {
            scope,
            signer,
            attribsHash,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeIntegrityInfoContent(
    value_: &AttributeIntegrityInfoContent,
) -> ASN1Result<X690Element> {
    |value_: &AttributeIntegrityInfoContent| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_Scope(&value_.scope)?);
        if let Some(v_) = &value_.signer {
            components_.push(_encode_Signer(&v_)?);
        }
        components_.push(_encode_AttribsHash(&value_.attribsHash)?);
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
/// Signer  ::=  CHOICE {
///   thisEntry   [0]  EXPLICIT ThisEntry,
///   thirdParty  [1]  SpecificallyIdentified,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum Signer {
    thisEntry(ThisEntry),
    thirdParty(SpecificallyIdentified),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for Signer {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Signer(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Signer {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Signer(el)
    }
}

pub fn _decode_Signer(el: &X690Element) -> ASN1Result<Signer> {
    |el: &X690Element| -> ASN1Result<Signer> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(Signer::thisEntry(
                |el: &X690Element| -> ASN1Result<ThisEntry> {
                    Ok(_decode_ThisEntry(&el.inner()?)?)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(Signer::thirdParty(_decode_SpecificallyIdentified(&el)?)),
            _ => Ok(Signer::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Signer(value_: &Signer) -> ASN1Result<X690Element> {
    |value: &Signer| -> ASN1Result<X690Element> {
        match value {
            Signer::thisEntry(v) => |v_1: &ThisEntry| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_ThisEntry(&v_1)?))),
                ))
            }(&v),
            Signer::thirdParty(v) => |v_1: &SpecificallyIdentified| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SpecificallyIdentified(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            Signer::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ThisEntry  ::=  CHOICE {
///   onlyOne   NULL,
///   specific  IssuerAndSerialNumber,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum ThisEntry {
    onlyOne(NULL),
    specific(IssuerAndSerialNumber),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for ThisEntry {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ThisEntry(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ThisEntry {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ThisEntry(el)
    }
}

pub fn _decode_ThisEntry(el: &X690Element) -> ASN1Result<ThisEntry> {
    |el: &X690Element| -> ASN1Result<ThisEntry> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(ThisEntry::onlyOne(ber_decode_null(&el)?)),
            (TagClass::UNIVERSAL, 16) => {
                Ok(ThisEntry::specific(_decode_IssuerAndSerialNumber(&el)?))
            }
            _ => Ok(ThisEntry::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_ThisEntry(value_: &ThisEntry) -> ASN1Result<X690Element> {
    |value: &ThisEntry| -> ASN1Result<X690Element> {
        match value {
            ThisEntry::onlyOne(v) => ber_encode_null(&v),
            ThisEntry::specific(v) => _encode_IssuerAndSerialNumber(&v),
            ThisEntry::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IssuerAndSerialNumber ::= SEQUENCE {
///   issuer  Name,
///   serial  CertificateSerialNumber,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct IssuerAndSerialNumber {
    pub issuer: Name,
    pub serial: CertificateSerialNumber,
    pub _unrecognized: Vec<X690Element>,
}
impl IssuerAndSerialNumber {
    pub fn new(
        issuer: Name,
        serial: CertificateSerialNumber,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        IssuerAndSerialNumber {
            issuer,
            serial,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for IssuerAndSerialNumber {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_IssuerAndSerialNumber(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for IssuerAndSerialNumber {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_IssuerAndSerialNumber(el)
    }
}

pub const _rctl1_components_for_IssuerAndSerialNumber: &[ComponentSpec; 2] = &[
    ComponentSpec::new("issuer", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "serial",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_IssuerAndSerialNumber: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_IssuerAndSerialNumber: &[ComponentSpec; 0] = &[];

pub fn _decode_IssuerAndSerialNumber(el: &X690Element) -> ASN1Result<IssuerAndSerialNumber> {
    |el_: &X690Element| -> ASN1Result<IssuerAndSerialNumber> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_IssuerAndSerialNumber,
            _eal_components_for_IssuerAndSerialNumber,
            _rctl2_components_for_IssuerAndSerialNumber,
        )?;
        let issuer = _decode_Name(_components.get("issuer").unwrap())?;
        let serial = _decode_CertificateSerialNumber(_components.get("serial").unwrap())?;
        Ok(IssuerAndSerialNumber {
            issuer,
            serial,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_IssuerAndSerialNumber(value_: &IssuerAndSerialNumber) -> ASN1Result<X690Element> {
    |value_: &IssuerAndSerialNumber| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_Name(&value_.issuer)?);
        components_.push(_encode_CertificateSerialNumber(&value_.serial)?);
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
/// SpecificallyIdentified ::= SEQUENCE {
///   name    GeneralName,
///   issuer  GeneralName OPTIONAL,
///   serial  CertificateSerialNumber OPTIONAL }
///   (WITH COMPONENTS { ..., issuer PRESENT, serial PRESENT } |
///   (WITH COMPONENTS { ..., issuer ABSENT, serial ABSENT }))
/// ```
///
///
#[derive(Debug, Clone)]
pub struct SpecificallyIdentified {
    pub name: GeneralName,
    pub issuer: OPTIONAL<GeneralName>,
    pub serial: OPTIONAL<CertificateSerialNumber>,
}
impl SpecificallyIdentified {
    pub fn new(
        name: GeneralName,
        issuer: OPTIONAL<GeneralName>,
        serial: OPTIONAL<CertificateSerialNumber>,
    ) -> Self {
        SpecificallyIdentified {
            name,
            issuer,
            serial,
        }
    }
}
impl TryFrom<X690Element> for SpecificallyIdentified {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SpecificallyIdentified(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SpecificallyIdentified {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SpecificallyIdentified(el)
    }
}

pub const _rctl1_components_for_SpecificallyIdentified: &[ComponentSpec; 3] = &[
    ComponentSpec::new("name", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "issuer",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
            &TagSelector::tag((TagClass::CONTEXT, 2)),
            &TagSelector::tag((TagClass::CONTEXT, 3)),
            &TagSelector::tag((TagClass::CONTEXT, 4)),
            &TagSelector::tag((TagClass::CONTEXT, 5)),
            &TagSelector::tag((TagClass::CONTEXT, 6)),
            &TagSelector::tag((TagClass::CONTEXT, 7)),
            &TagSelector::tag((TagClass::CONTEXT, 8)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "serial",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_SpecificallyIdentified: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_SpecificallyIdentified: &[ComponentSpec; 0] = &[];

pub fn _decode_SpecificallyIdentified(el: &X690Element) -> ASN1Result<SpecificallyIdentified> {
    |el_: &X690Element| -> ASN1Result<SpecificallyIdentified> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_SpecificallyIdentified,
            _eal_components_for_SpecificallyIdentified,
            _rctl2_components_for_SpecificallyIdentified,
        )?;
        let name = _decode_GeneralName(_components.get("name").unwrap())?;
        let issuer: OPTIONAL<GeneralName> = match _components.get("issuer") {
            Some(c_) => Some(_decode_GeneralName(c_)?),
            _ => None,
        };
        let serial: OPTIONAL<CertificateSerialNumber> = match _components.get("serial") {
            Some(c_) => Some(_decode_CertificateSerialNumber(c_)?),
            _ => None,
        };
        Ok(SpecificallyIdentified {
            name,
            issuer,
            serial,
        })
    }(&el)
}

pub fn _encode_SpecificallyIdentified(value_: &SpecificallyIdentified) -> ASN1Result<X690Element> {
    |value_: &SpecificallyIdentified| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(_encode_GeneralName(&value_.name)?);
        if let Some(v_) = &value_.issuer {
            components_.push(_encode_GeneralName(&v_)?);
        }
        if let Some(v_) = &value_.serial {
            components_.push(_encode_CertificateSerialNumber(&v_)?);
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
/// Scope  ::=  CHOICE {
///   wholeEntry     [0]  NULL, -- Signature protects all attribute values in this entry
///   selectedTypes  [1]  SelectedTypes,
///       -- Signature protects all attribute values of the selected attribute types
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum Scope {
    wholeEntry(NULL),
    selectedTypes(SelectedTypes),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for Scope {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Scope(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Scope {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Scope(el)
    }
}

pub fn _decode_Scope(el: &X690Element) -> ASN1Result<Scope> {
    |el: &X690Element| -> ASN1Result<Scope> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(Scope::wholeEntry(ber_decode_null(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(Scope::selectedTypes(_decode_SelectedTypes(&el)?)),
            _ => Ok(Scope::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Scope(value_: &Scope) -> ASN1Result<X690Element> {
    |value: &Scope| -> ASN1Result<X690Element> {
        match value {
            Scope::wholeEntry(v) => |v_1: &NULL| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_null(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            Scope::selectedTypes(v) => |v_1: &SelectedTypes| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_SelectedTypes(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            Scope::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SelectedTypes  ::=  SEQUENCE SIZE (1..MAX) OF AttributeType
/// ```
pub type SelectedTypes = Vec<AttributeType>; // SequenceOfType

pub fn _decode_SelectedTypes(el: &X690Element) -> ASN1Result<SelectedTypes> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AttributeType>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<AttributeType> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_AttributeType(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_SelectedTypes(value_: &SelectedTypes) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<AttributeType>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_AttributeType(&v)?);
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
/// AttribsHash  ::=  HASH{HashedAttributes}
/// ```
pub type AttribsHash = HASH; // DefinedType

pub fn _decode_AttribsHash(el: &X690Element) -> ASN1Result<AttribsHash> {
    _decode_HASH(&el)
}

pub fn _encode_AttribsHash(value_: &AttribsHash) -> ASN1Result<X690Element> {
    _encode_HASH(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// HashedAttributes  ::=  SEQUENCE SIZE (1..MAX) OF Attribute{{SupportedAttributes}}
/// ```
pub type HashedAttributes = Vec<Attribute>; // SequenceOfType

pub fn _decode_HashedAttributes(el: &X690Element) -> ASN1Result<HashedAttributes> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<Attribute>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<Attribute> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_Attribute(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_HashedAttributes(value_: &HashedAttributes) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<Attribute>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_Attribute(&v)?);
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
/// integrityInfo OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   KIND          auxiliary
///   MUST CONTAIN  {attributeIntegrityInfo}
///   ID            id-oc-integrityInfo }
/// ```
///
///
pub fn integrityInfo() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::<_>::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),       /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::<_>::from([attributeIntegrityInfo()])), /* OBJECT_FIELD_SETTING */
        id: id_oc_integrityInfo(), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeValueIntegrityInfoContext CONTEXT ::= {
///   WITH SYNTAX  AttributeValueIntegrityInfo
///   ID           id-avc-attributeValueIntegrityInfoContext }
/// ```
///
///
pub fn attributeValueIntegrityInfoContext() -> CONTEXT {
    CONTEXT {
        id: id_avc_attributeValueIntegrityInfoContext(), /* OBJECT_FIELD_SETTING */
        absentMatch: Some(true), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeValueIntegrityInfo  ::=  SIGNED{AttributeValueIntegrityInfoContent}
/// ```
pub type AttributeValueIntegrityInfo = SIGNED<AttributeValueIntegrityInfoContent>; // DefinedType

pub fn _decode_AttributeValueIntegrityInfo(
    el: &X690Element,
) -> ASN1Result<AttributeValueIntegrityInfo> {
    _decode_SIGNED::<AttributeValueIntegrityInfoContent>(
        _decode_AttributeValueIntegrityInfoContent,
        &el,
    )
}

pub fn _encode_AttributeValueIntegrityInfo(
    value_: &AttributeValueIntegrityInfo,
) -> ASN1Result<X690Element> {
    _encode_SIGNED::<AttributeValueIntegrityInfoContent>(
        _encode_AttributeValueIntegrityInfoContent,
        &value_,
    )
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeValueIntegrityInfoContent ::= SEQUENCE {
///   signer   Signer OPTIONAL, -- Authority or data originators name
///   aVIHash  AVIHash,         -- Hash value of protected attribute
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeValueIntegrityInfoContent {
    pub signer: OPTIONAL<Signer>,
    pub aVIHash: AVIHash,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeValueIntegrityInfoContent {
    pub fn new(
        signer: OPTIONAL<Signer>,
        aVIHash: AVIHash,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeValueIntegrityInfoContent {
            signer,
            aVIHash,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttributeValueIntegrityInfoContent {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeValueIntegrityInfoContent(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeValueIntegrityInfoContent {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeValueIntegrityInfoContent(el)
    }
}

pub const _rctl1_components_for_AttributeValueIntegrityInfoContent: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "signer",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "aVIHash",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeValueIntegrityInfoContent: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeValueIntegrityInfoContent: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeValueIntegrityInfoContent(
    el: &X690Element,
) -> ASN1Result<AttributeValueIntegrityInfoContent> {
    |el_: &X690Element| -> ASN1Result<AttributeValueIntegrityInfoContent> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeValueIntegrityInfoContent,
            _eal_components_for_AttributeValueIntegrityInfoContent,
            _rctl2_components_for_AttributeValueIntegrityInfoContent,
        )?;
        let signer: OPTIONAL<Signer> = match _components.get("signer") {
            Some(c_) => Some(_decode_Signer(c_)?),
            _ => None,
        };
        let aVIHash = _decode_AVIHash(_components.get("aVIHash").unwrap())?;
        Ok(AttributeValueIntegrityInfoContent {
            signer,
            aVIHash,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeValueIntegrityInfoContent(
    value_: &AttributeValueIntegrityInfoContent,
) -> ASN1Result<X690Element> {
    |value_: &AttributeValueIntegrityInfoContent| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.signer {
            components_.push(_encode_Signer(&v_)?);
        }
        components_.push(_encode_AVIHash(&value_.aVIHash)?);
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
/// AVIHash  ::=  HASH{AttributeTypeValueContexts}
/// ```
pub type AVIHash = HASH; // DefinedType

pub fn _decode_AVIHash(el: &X690Element) -> ASN1Result<AVIHash> {
    _decode_HASH(&el)
}

pub fn _encode_AVIHash(value_: &AVIHash) -> ASN1Result<X690Element> {
    _encode_HASH(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeTypeValueContexts ::= SEQUENCE {
///   type         ATTRIBUTE.&id({SupportedAttributes}),
///   value        ATTRIBUTE.&Type({SupportedAttributes}{@type}),
///   contextList  SET SIZE (1..MAX) OF Context OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct AttributeTypeValueContexts {
    pub type_: OBJECT_IDENTIFIER,
    pub value: X690Element,
    pub contextList: OPTIONAL<Vec<Context>>,
    pub _unrecognized: Vec<X690Element>,
}
impl AttributeTypeValueContexts {
    pub fn new(
        type_: OBJECT_IDENTIFIER,
        value: X690Element,
        contextList: OPTIONAL<Vec<Context>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        AttributeTypeValueContexts {
            type_,
            value,
            contextList,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for AttributeTypeValueContexts {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeTypeValueContexts(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for AttributeTypeValueContexts {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_AttributeTypeValueContexts(el)
    }
}

pub const _rctl1_components_for_AttributeTypeValueContexts: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new("value", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "contextList",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_AttributeTypeValueContexts: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_AttributeTypeValueContexts: &[ComponentSpec; 0] = &[];

pub fn _decode_AttributeTypeValueContexts(
    el: &X690Element,
) -> ASN1Result<AttributeTypeValueContexts> {
    |el_: &X690Element| -> ASN1Result<AttributeTypeValueContexts> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_AttributeTypeValueContexts,
            _eal_components_for_AttributeTypeValueContexts,
            _rctl2_components_for_AttributeTypeValueContexts,
        )?;
        let type_ = ber_decode_object_identifier(_components.get("type").unwrap())?;
        let value = x690_identity(_components.get("value").unwrap())?;
        let contextList: OPTIONAL<Vec<Context>> = match _components.get("contextList") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<SET_OF<Context>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<Context> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_Context(el)?);
                }
                Ok(items)
            }(c_)?),
            _ => None,
        };
        Ok(AttributeTypeValueContexts {
            type_,
            value,
            contextList,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_AttributeTypeValueContexts(
    value_: &AttributeTypeValueContexts,
) -> ASN1Result<X690Element> {
    |value_: &AttributeTypeValueContexts| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(ber_encode_object_identifier(&value_.type_)?);
        components_.push(x690_identity(&value_.value)?);
        if let Some(v_) = &value_.contextList {
            components_.push(|value_: &SET_OF<Context>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_Context(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
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
/// id-oc-integrityInfo OBJECT IDENTIFIER ::= {id-oc 40}
/// ```
///
///
pub fn id_oc_integrityInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([40])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-clearance                           OBJECT IDENTIFIER ::= {id-at 55}
/// ```
///
///
pub fn id_at_clearance() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([55])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-attributeIntegrityInfo              OBJECT IDENTIFIER ::= {id-at 57}
/// ```
///
///
pub fn id_at_attributeIntegrityInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([57])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc-attributeValueSecurityLabelContext OBJECT IDENTIFIER ::= {id-avc 3}
/// ```
///
///
pub fn id_avc_attributeValueSecurityLabelContext() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_avc().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc-attributeValueIntegrityInfoContext OBJECT IDENTIFIER ::= {id-avc 4}
/// ```
///
///
pub fn id_avc_attributeValueIntegrityInfoContext() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_avc().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}
