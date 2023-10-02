#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # SelectedAttributeTypes
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `SelectedAttributeTypes`.
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
use crate::PkiPmiExternalDataTypes::*;
use crate::SchemaAdministration::*;
use crate::ServiceAdministration::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// UnboundedDirectoryString  ::=  CHOICE {
///   teletexString    TeletexString(SIZE (1..MAX)),
///   printableString  PrintableString(SIZE (1..MAX)),
///   bmpString        BMPString(SIZE (1..MAX)),
///   universalString  UniversalString(SIZE (1..MAX)),
///   uTF8String       UTF8String(SIZE (1..MAX)) }
/// ```
#[derive(Debug, Clone)]
pub enum UnboundedDirectoryString {
    teletexString(TeletexString),
    printableString(PrintableString),
    bmpString(BMPString),
    universalString(UniversalString),
    uTF8String(UTF8String),
}

impl TryFrom<&X690Element> for UnboundedDirectoryString {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UnboundedDirectoryString(el)
    }
}

pub fn _decode_UnboundedDirectoryString(el: &X690Element) -> ASN1Result<UnboundedDirectoryString> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 20) => Ok(UnboundedDirectoryString::teletexString(
            BER.decode_t61_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 19) => Ok(UnboundedDirectoryString::printableString(
            BER.decode_printable_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 30) => Ok(UnboundedDirectoryString::bmpString(
            BER.decode_bmp_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 28) => Ok(UnboundedDirectoryString::universalString(
            BER.decode_universal_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 12) => Ok(UnboundedDirectoryString::uTF8String(
            BER.decode_utf8_string(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "UnboundedDirectoryString",
            ))
        }
    }
}

pub fn _encode_UnboundedDirectoryString(
    value_: &UnboundedDirectoryString,
) -> ASN1Result<X690Element> {
    match value_ {
        UnboundedDirectoryString::teletexString(v) => BER.encode_t61_string(&v),
        UnboundedDirectoryString::printableString(v) => BER.encode_printable_string(&v),
        UnboundedDirectoryString::bmpString(v) => BER.encode_bmp_string(&v),
        UnboundedDirectoryString::universalString(v) => BER.encode_universal_string(&v),
        UnboundedDirectoryString::uTF8String(v) => BER.encode_utf8_string(&v),
    }
}

pub fn _validate_UnboundedDirectoryString(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 20) => BER.validate_t61_string(&el),
        (TagClass::UNIVERSAL, 19) => BER.validate_printable_string(&el),
        (TagClass::UNIVERSAL, 30) => BER.validate_bmp_string(&el),
        (TagClass::UNIVERSAL, 28) => BER.validate_universal_string(&el),
        (TagClass::UNIVERSAL, 12) => BER.validate_utf8_string(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "UnboundedDirectoryString",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DirectoryString{INTEGER:maxSize}  ::=  CHOICE {
///   teletexString    TeletexString(SIZE (1..maxSize,...)),
///   printableString  PrintableString(SIZE (1..maxSize,...)),
///   bmpString        BMPString(SIZE (1..maxSize,...)),
///   universalString  UniversalString(SIZE (1..maxSize,...)),
///   uTF8String       UTF8String(SIZE (1..maxSize,...)) }
/// ```
#[derive(Debug, Clone)]
pub enum DirectoryString {
    teletexString(TeletexString),
    printableString(PrintableString),
    bmpString(BMPString),
    universalString(UniversalString),
    uTF8String(UTF8String),
}

impl TryFrom<&X690Element> for DirectoryString {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryString(el)
    }
}

pub fn _decode_DirectoryString(el: &X690Element) -> ASN1Result<DirectoryString> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 20) => {
            Ok(DirectoryString::teletexString(BER.decode_t61_string(&el)?))
        }
        (TagClass::UNIVERSAL, 19) => Ok(DirectoryString::printableString(
            BER.decode_printable_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 30) => Ok(DirectoryString::bmpString(BER.decode_bmp_string(&el)?)),
        (TagClass::UNIVERSAL, 28) => Ok(DirectoryString::universalString(
            BER.decode_universal_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 12) => Ok(DirectoryString::uTF8String(BER.decode_utf8_string(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "DirectoryString",
            ))
        }
    }
}

pub fn _encode_DirectoryString(value_: &DirectoryString) -> ASN1Result<X690Element> {
    match value_ {
        DirectoryString::teletexString(v) => BER.encode_t61_string(&v),
        DirectoryString::printableString(v) => BER.encode_printable_string(&v),
        DirectoryString::bmpString(v) => BER.encode_bmp_string(&v),
        DirectoryString::universalString(v) => BER.encode_universal_string(&v),
        DirectoryString::uTF8String(v) => BER.encode_utf8_string(&v),
    }
}

pub fn _validate_DirectoryString(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 20) => BER.validate_t61_string(&el),
        (TagClass::UNIVERSAL, 19) => BER.validate_printable_string(&el),
        (TagClass::UNIVERSAL, 30) => BER.validate_bmp_string(&el),
        (TagClass::UNIVERSAL, 28) => BER.validate_universal_string(&el),
        (TagClass::UNIVERSAL, 12) => BER.validate_utf8_string(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "DirectoryString",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// knowledgeInformation ATTRIBUTE ::= {
///   WITH SYNTAX              UnboundedDirectoryString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   OBSOLETE                 TRUE
///   ID                       id-at-knowledgeInformation }
/// ```
///
///
pub fn knowledgeInformation() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        obsolete: Some(true),                              /* OBJECT_FIELD_SETTING */
        id: id_at_knowledgeInformation(),                  /* OBJECT_FIELD_SETTING */
        derivation: None,
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
    }
}

pub mod knowledgeInformation {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// name ATTRIBUTE ::= {
///   WITH SYNTAX              UnboundedDirectoryString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"name"}
///   ID                       id-at-name }
/// ```
///
///
pub fn name() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("name")])), /* OBJECT_FIELD_SETTING */
        id: id_at_name(),                                  /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod name {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// commonName ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              UnboundedDirectoryString
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"cn", "commonName"}
///   ID                       id-at-commonName }
/// ```
///
///
pub fn commonName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("cn"), String::from("commonName")])), /* OBJECT_FIELD_SETTING */
        id: id_at_commonName(), /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod commonName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// surname ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              UnboundedDirectoryString
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"sn"}
///   ID                       id-at-surname }
/// ```
///
///
pub fn surname() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("sn")])), /* OBJECT_FIELD_SETTING */
        id: id_at_surname(),                    /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod surname {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// givenName ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              UnboundedDirectoryString
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"givenName"}
///   ID                       id-at-givenName }
/// ```
///
///
pub fn givenName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("givenName")])), /* OBJECT_FIELD_SETTING */
        id: id_at_givenName(),                  /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod givenName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// initials ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              UnboundedDirectoryString
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"initials"}
///   ID                       id-at-initials }
/// ```
///
///
pub fn initials() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("initials")])), /* OBJECT_FIELD_SETTING */
        id: id_at_initials(),                   /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod initials {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// generationQualifier ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              UnboundedDirectoryString
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"generationQualifier"}
///   ID                       id-at-generationQualifier }
/// ```
///
///
pub fn generationQualifier() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("generationQualifier")])), /* OBJECT_FIELD_SETTING */
        id: id_at_generationQualifier(),        /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod generationQualifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uniqueIdentifier ATTRIBUTE ::= {
///   WITH SYNTAX              UniqueIdentifier
///   EQUALITY MATCHING RULE   bitStringMatch
///   LDAP-SYNTAX              bitString.&id
///   LDAP-NAME                {"x500UniqueIdentifier"}
///   ID                       id-at-uniqueIdentifier }
/// ```
///
///
pub fn uniqueIdentifier() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(bitStringMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(bitString().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("x500UniqueIdentifier")])), /* OBJECT_FIELD_SETTING */
        id: id_at_uniqueIdentifier(), /* OBJECT_FIELD_SETTING */
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

pub mod uniqueIdentifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UniqueIdentifier; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UniqueIdentifier(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UniqueIdentifier(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UniqueIdentifier(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniqueIdentifier  ::=  BIT STRING
/// ```
pub type UniqueIdentifier = BIT_STRING;

pub fn _decode_UniqueIdentifier(el: &X690Element) -> ASN1Result<UniqueIdentifier> {
    BER.decode_bit_string(&el)
}

pub fn _encode_UniqueIdentifier(value_: &UniqueIdentifier) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_UniqueIdentifier(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dnQualifier ATTRIBUTE ::= {
///   WITH SYNTAX              PrintableString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   ORDERING MATCHING RULE   caseIgnoreOrderingMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              printableString.&id
///   LDAP-NAME                {"dnQualifier"}
///   ID                       id-at-dnQualifier }
/// ```
///
///
pub fn dnQualifier() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(caseIgnoreOrderingMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(printableString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("dnQualifier")])), /* OBJECT_FIELD_SETTING */
        id: id_at_dnQualifier(),                           /* OBJECT_FIELD_SETTING */
        derivation: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod dnQualifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PrintableString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_printable_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_printable_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_printable_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// serialNumber ATTRIBUTE ::= {
///   WITH SYNTAX              PrintableString(SIZE (1..MAX))
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              printableString.&id
///   LDAP-NAME                {"serialNumber"}
///   ID                       id-at-serialNumber }
/// ```
///
///
pub fn serialNumber() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(printableString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("serialNumber")])), /* OBJECT_FIELD_SETTING */
        id: id_at_serialNumber(),                          /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod serialNumber {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PrintableString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_printable_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_printable_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_printable_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pseudonym ATTRIBUTE ::= {
///   SUBTYPE OF              name
///   WITH SYNTAX             UnboundedDirectoryString
///   ID                      id-at-pseudonym }
/// ```
///
///
pub fn pseudonym() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())), /* OBJECT_FIELD_SETTING */
        id: id_at_pseudonym(),              /* OBJECT_FIELD_SETTING */
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

pub mod pseudonym {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uUIDPair ATTRIBUTE ::= {
///   WITH SYNTAX             UUIDPair
///   EQUALITY MATCHING RULE  uUIDPairMatch
///   ID                      id-at-uuidpair }
/// ```
///
///
pub fn uUIDPair() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(uUIDPairMatch())), /* OBJECT_FIELD_SETTING */
        id: id_at_uuidpair(),                            /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod uUIDPair {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UUIDPair; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UUIDPair(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UUIDPair(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UUIDPair(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UUIDPair ::= SEQUENCE {
///   issuerUUID   UUID,
///   subjectUUID  UUID,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct UUIDPair {
    pub issuerUUID: UUID,
    pub subjectUUID: UUID,
    pub _unrecognized: Vec<X690Element>,
}
impl UUIDPair {
    pub fn new(issuerUUID: UUID, subjectUUID: UUID, _unrecognized: Vec<X690Element>) -> Self {
        UUIDPair {
            issuerUUID,
            subjectUUID,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for UUIDPair {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UUIDPair(el)
    }
}

pub const _rctl1_components_for_UUIDPair: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "issuerUUID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subjectUUID",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UUIDPair: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UUIDPair: &[ComponentSpec; 0] = &[];

pub fn _decode_UUIDPair(el: &X690Element) -> ASN1Result<UUIDPair> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UUIDPair")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UUIDPair,
        _eal_components_for_UUIDPair,
        _rctl2_components_for_UUIDPair,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut issuerUUID_: OPTIONAL<UUID> = None;
    let mut subjectUUID_: OPTIONAL<UUID> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuerUUID" => issuerUUID_ = Some(_decode_UUID(_el)?),
            "subjectUUID" => subjectUUID_ = Some(_decode_UUID(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(UUIDPair {
        issuerUUID: issuerUUID_.unwrap(),
        subjectUUID: subjectUUID_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_UUIDPair(value_: &UUIDPair) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_UUID(&value_.issuerUUID)?);
    components_.push(_encode_UUID(&value_.subjectUUID)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_UUIDPair(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UUIDPair")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UUIDPair,
        _eal_components_for_UUIDPair,
        _rctl2_components_for_UUIDPair,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "issuerUUID" => _validate_UUID(_el)?,
            "subjectUUID" => _validate_UUID(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UUID  ::=  OCTET STRING(SIZE (16))
/// ```
pub type UUID = OCTET_STRING; // OctetStringType

pub fn _decode_UUID(el: &X690Element) -> ASN1Result<UUID> {
    BER.decode_octet_string(&el)
}

pub fn _encode_UUID(value_: &UUID) -> ASN1Result<X690Element> {
    BER.encode_octet_string(&value_)
}

pub fn _validate_UUID(el: &X690Element) -> ASN1Result<()> {
    BER.validate_octet_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uri ATTRIBUTE ::= {
///   WITH SYNTAX              URI
///   EQUALITY MATCHING RULE   uriMatch
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"uri"}
///   ID                       id-at-uri }
/// ```
///
///
pub fn uri() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(uriMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),     /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("uri")])), /* OBJECT_FIELD_SETTING */
        id: id_at_uri(),                            /* OBJECT_FIELD_SETTING */
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

pub mod uri {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = URI; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_URI(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_URI(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_URI(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// URI  ::=  UTF8String
/// ```
pub type URI = UTF8String; // UTF8String

pub fn _decode_URI(el: &X690Element) -> ASN1Result<URI> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_URI(value_: &URI) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_URI(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// urn ATTRIBUTE ::= {
///   SUBTYPE OF               uri
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"urn"}
///   ID                       id-at-urn }
/// ```
///
///
pub fn urn() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(uri())),      /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("urn")])), /* OBJECT_FIELD_SETTING */
        id: id_at_urn(),                        /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod urn {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// url ATTRIBUTE ::= {
///   SUBTYPE OF               uri
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"url"}
///   ID                       id-at-url }
/// ```
///
///
pub fn url() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(uri())),      /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("url")])), /* OBJECT_FIELD_SETTING */
        id: id_at_url(),                        /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod url {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dnsName ATTRIBUTE ::= {
///   WITH SYNTAX             DomainName
///   EQUALITY MATCHING RULE  dnsNameMatch
///   LDAP-SYNTAX             dnsString.&id
///   LDAP-NAME               {"DNS name"}
///   ID                      id-at-dnsName }
/// ```
///
///
pub fn dnsName() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(dnsNameMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dnsString().id),               /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("DNS name")])), /* OBJECT_FIELD_SETTING */
        id: id_at_dnsName(),                            /* OBJECT_FIELD_SETTING */
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

pub mod dnsName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DomainName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DomainName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DomainName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DomainName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DomainName  ::=  UTF8String (CONSTRAINED BY { -- Conforms to the format of a domain name. -- })
/// ```
pub type DomainName = UTF8String; // UTF8String

pub fn _decode_DomainName(el: &X690Element) -> ASN1Result<DomainName> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_DomainName(value_: &DomainName) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_DomainName(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// intEmail ATTRIBUTE ::= {
///   WITH SYNTAX             IntEmail
///   EQUALITY MATCHING RULE  intEmailMatch
///   SINGLE VALUE            TRUE
///   LDAP-SYNTAX             intEmailString.&id
///   LDAP-NAME               {"Internationalized Email"}
///   ID                      id-at-intEmail }
/// ```
///
///
pub fn intEmail() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(intEmailMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                       /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(intEmailString().id),           /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Internationalized Email")])), /* OBJECT_FIELD_SETTING */
        id: id_at_intEmail(), /* OBJECT_FIELD_SETTING */
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

pub mod intEmail {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = IntEmail; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_IntEmail(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_IntEmail(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_IntEmail(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// IntEmail  ::=  UTF8String (CONSTRAINED BY { -- Conforms to the format of an (internationalized) email address. -- })
/// ```
pub type IntEmail = UTF8String; // UTF8String

pub fn _decode_IntEmail(el: &X690Element) -> ASN1Result<IntEmail> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_IntEmail(value_: &IntEmail) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_IntEmail(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// jid ATTRIBUTE ::= {
///   WITH SYNTAX             Jid
///   EQUALITY MATCHING RULE  jidMatch
///   SINGLE VALUE            TRUE
///   LDAP-SYNTAX             jidString.&id
///   LDAP-NAME               {"Jabber identifier"}
///   ID                      id-at-jid }
/// ```
///
///
pub fn jid() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(jidMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                  /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(jidString().id),           /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Jabber identifier")])), /* OBJECT_FIELD_SETTING */
        id: id_at_jid(),                            /* OBJECT_FIELD_SETTING */
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

pub mod jid {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Jid; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Jid(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Jid(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Jid(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Jid  ::=  UTF8String (CONSTRAINED BY { /* Conforms to the format of a jabber identifier. */ })
/// ```
pub type Jid = UTF8String; // UTF8String

pub fn _decode_Jid(el: &X690Element) -> ASN1Result<Jid> {
    BER.decode_utf8_string(&el)
}

pub fn _encode_Jid(value_: &Jid) -> ASN1Result<X690Element> {
    BER.encode_utf8_string(&value_)
}

pub fn _validate_Jid(el: &X690Element) -> ASN1Result<()> {
    BER.validate_utf8_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// objectIdentifier ATTRIBUTE ::= {
///   WITH SYNTAX             OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE  objectIdentifierMatch
///   SINGLE VALUE            TRUE
///   LDAP-SYNTAX             oid.&id
///   LDAP-NAME               {"Object Identifier"}
///   ID                      id-at-objectIdentifier }
/// ```
///
///
pub fn objectIdentifier() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                               /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),                              /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Object Identifier")])), /* OBJECT_FIELD_SETTING */
        id: id_at_objectIdentifier(),                            /* OBJECT_FIELD_SETTING */
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

pub mod objectIdentifier {
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
/// countryName ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              CountryName
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              countryString.&id
///   LDAP-NAME                {"c"}
///   ID                       id-at-countryName }
/// ```
///
///
pub fn countryName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),   /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),            /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(countryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c")])), /* OBJECT_FIELD_SETTING */
        id: id_at_countryName(),              /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod countryName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CountryName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CountryName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CountryName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CountryName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CountryName  ::=  PrintableString(SIZE (2)) (CONSTRAINED BY { -- ISO 3166 alpha-2 codes only -- })
/// ```
pub type CountryName = PrintableString; // PrintableString

pub fn _decode_CountryName(el: &X690Element) -> ASN1Result<CountryName> {
    BER.decode_printable_string(&el)
}

pub fn _encode_CountryName(value_: &CountryName) -> ASN1Result<X690Element> {
    BER.encode_printable_string(&value_)
}

pub fn _validate_CountryName(el: &X690Element) -> ASN1Result<()> {
    BER.validate_printable_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// countryCode3c ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              CountryCode3c
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              countryString3c.&id
///   LDAP-NAME                {"c3"}
///   ID                       id-at-countryCode3c }
/// ```
///
///
pub fn countryCode3c() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),              /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(countryString3c().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c3")])), /* OBJECT_FIELD_SETTING */
        id: id_at_countryCode3c(),              /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod countryCode3c {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CountryCode3c; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CountryCode3c(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CountryCode3c(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CountryCode3c(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CountryCode3c  ::=  PrintableString(SIZE (3)) (CONSTRAINED BY { -- ISO 3166 alpha-3 codes only -- })
/// ```
pub type CountryCode3c = PrintableString; // PrintableString

pub fn _decode_CountryCode3c(el: &X690Element) -> ASN1Result<CountryCode3c> {
    BER.decode_printable_string(&el)
}

pub fn _encode_CountryCode3c(value_: &CountryCode3c) -> ASN1Result<X690Element> {
    BER.encode_printable_string(&value_)
}

pub fn _validate_CountryCode3c(el: &X690Element) -> ASN1Result<()> {
    BER.validate_printable_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// countryCode3n ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              CountryCode3n
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              countryString3n.&id
///   LDAP-NAME                {"n3"}
///   ID                       id-at-countryCode3n }
/// ```
///
///
pub fn countryCode3n() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),              /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(countryString3n().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("n3")])), /* OBJECT_FIELD_SETTING */
        id: id_at_countryCode3n(),              /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod countryCode3n {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CountryCode3n; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CountryCode3n(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CountryCode3n(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CountryCode3n(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CountryCode3n  ::=  NumericString(SIZE (3)) (CONSTRAINED BY { -- ISO 3166 numeric-3 codes only -- })
/// ```
pub type CountryCode3n = NumericString; // NumericString

pub fn _decode_CountryCode3n(el: &X690Element) -> ASN1Result<CountryCode3n> {
    BER.decode_numeric_string(&el)
}

pub fn _encode_CountryCode3n(value_: &CountryCode3n) -> ASN1Result<X690Element> {
    BER.encode_numeric_string(&value_)
}

pub fn _validate_CountryCode3n(el: &X690Element) -> ASN1Result<()> {
    BER.validate_numeric_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// localityName ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              UnboundedDirectoryString
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"l"}
///   ID                       id-at-localityName }
/// ```
///
///
pub fn localityName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("l")])), /* OBJECT_FIELD_SETTING */
        id: id_at_localityName(),               /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod localityName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveLocalityName ATTRIBUTE ::= {
///   SUBTYPE OF              localityName
///   COLLECTIVE              TRUE
///   LDAP-SYNTAX             directoryString.&id
///   LDAP-NAME               {"c-l"}
///   ID                      id-at-collectiveLocalityName }
/// ```
///
///
pub fn collectiveLocalityName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(localityName())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                     /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),     /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-l")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectiveLocalityName(),         /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectiveLocalityName {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// stateOrProvinceName ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              UnboundedDirectoryString
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"st"}
///   ID                       id-at-stateOrProvinceName }
/// ```
///
///
pub fn stateOrProvinceName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("st")])), /* OBJECT_FIELD_SETTING */
        id: id_at_stateOrProvinceName(),        /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod stateOrProvinceName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveStateOrProvinceName ATTRIBUTE ::= {
///   SUBTYPE OF               stateOrProvinceName
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"c-st"}
///   ID                       id-at-collectiveStateOrProvinceName }
/// ```
///
///
pub fn collectiveStateOrProvinceName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(stateOrProvinceName())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                            /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-st")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectiveStateOrProvinceName(),         /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectiveStateOrProvinceName {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// streetAddress ATTRIBUTE ::= {
///   WITH SYNTAX              UnboundedDirectoryString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"street"}
///   ID                       id-at-streetAddress }
/// ```
///
///
pub fn streetAddress() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("street")])), /* OBJECT_FIELD_SETTING */
        id: id_at_streetAddress(),                         /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod streetAddress {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveStreetAddress ATTRIBUTE ::= {
///   SUBTYPE OF               streetAddress
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"c-street"}
///   ID                       id-at-collectiveStreetAddress }
/// ```
///
///
pub fn collectiveStreetAddress() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(streetAddress())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                      /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),      /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-street")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectiveStreetAddress(),         /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectiveStreetAddress {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// houseIdentifier ATTRIBUTE ::= {
///   WITH SYNTAX              UnboundedDirectoryString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"houseIdentifier"}
///   ID                       id-at-houseIdentifier }
/// ```
///
///
pub fn houseIdentifier() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("houseIdentifier")])), /* OBJECT_FIELD_SETTING */
        id: id_at_houseIdentifier(),                       /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod houseIdentifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// utmCoordinates  ATTRIBUTE ::= {
///   WITH SYNTAX              UtmCoordinates
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              utmCoords.&id
///   LDAP-NAME                {"utmCoordinates"}
///   ID                       id-at-utmCoordinates }
/// ```
///
///
pub fn utmCoordinates() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),        /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(utmCoords().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("utmCoordinates")])), /* OBJECT_FIELD_SETTING */
        id: id_at_utmCoordinates(),       /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
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

pub mod utmCoordinates {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UtmCoordinates; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UtmCoordinates(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UtmCoordinates(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UtmCoordinates(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UtmCoordinates ::= SEQUENCE {
///   zone      PrintableString,
///   easting   NumericString,
///   northing  NumericString }
/// ```
///
#[derive(Debug, Clone)]
pub struct UtmCoordinates {
    pub zone: PrintableString,
    pub easting: NumericString,
    pub northing: NumericString,
}
impl UtmCoordinates {
    pub fn new(zone: PrintableString, easting: NumericString, northing: NumericString) -> Self {
        UtmCoordinates {
            zone,
            easting,
            northing,
        }
    }
}
impl TryFrom<&X690Element> for UtmCoordinates {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UtmCoordinates(el)
    }
}

pub const _rctl1_components_for_UtmCoordinates: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "zone",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 19)),
        None,
        None,
    ),
    ComponentSpec::new(
        "easting",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 18)),
        None,
        None,
    ),
    ComponentSpec::new(
        "northing",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 18)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UtmCoordinates: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UtmCoordinates: &[ComponentSpec; 0] = &[];

pub fn _decode_UtmCoordinates(el: &X690Element) -> ASN1Result<UtmCoordinates> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UtmCoordinates"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UtmCoordinates,
        _eal_components_for_UtmCoordinates,
        _rctl2_components_for_UtmCoordinates,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut zone_: OPTIONAL<PrintableString> = None;
    let mut easting_: OPTIONAL<NumericString> = None;
    let mut northing_: OPTIONAL<NumericString> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "zone" => zone_ = Some(BER.decode_printable_string(_el)?),
            "easting" => easting_ = Some(BER.decode_numeric_string(_el)?),
            "northing" => northing_ = Some(BER.decode_numeric_string(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UtmCoordinates")
                )
            }
        }
    }
    Ok(UtmCoordinates {
        zone: zone_.unwrap(),
        easting: easting_.unwrap(),
        northing: northing_.unwrap(),
    })
}

pub fn _encode_UtmCoordinates(value_: &UtmCoordinates) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(BER.encode_printable_string(&value_.zone)?);
    components_.push(BER.encode_numeric_string(&value_.easting)?);
    components_.push(BER.encode_numeric_string(&value_.northing)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_UtmCoordinates(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UtmCoordinates"))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UtmCoordinates,
        _eal_components_for_UtmCoordinates,
        _rctl2_components_for_UtmCoordinates,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "zone" => BER.validate_printable_string(_el)?,
            "easting" => BER.validate_numeric_string(_el)?,
            "northing" => BER.validate_numeric_string(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UtmCoordinates")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// organizationName ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              UnboundedDirectoryString
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"o"}
///   ID                       id-at-organizationName }
/// ```
///
///
pub fn organizationName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("o")])), /* OBJECT_FIELD_SETTING */
        id: id_at_organizationName(),           /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod organizationName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveOrganizationName ATTRIBUTE ::= {
///   SUBTYPE OF               organizationName
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"c-o"}
///   ID                       id-at-collectiveOrganizationName }
/// ```
///
///
pub fn collectiveOrganizationName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(organizationName())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                         /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),         /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-o")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectiveOrganizationName(),         /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectiveOrganizationName {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// organizationalUnitName ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              UnboundedDirectoryString
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"ou"}
///   ID                       id-at-organizationalUnitName }
/// ```
///
///
pub fn organizationalUnitName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("ou")])), /* OBJECT_FIELD_SETTING */
        id: id_at_organizationalUnitName(),     /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod organizationalUnitName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveOrganizationalUnitName ATTRIBUTE ::= {
///   SUBTYPE OF               organizationalUnitName
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"c-ou"}
///   ID                       id-at-collectiveOrganizationalUnitName }
/// ```
///
///
pub fn collectiveOrganizationalUnitName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(organizationalUnitName())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                               /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),               /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-ou")])),    /* OBJECT_FIELD_SETTING */
        id: id_at_collectiveOrganizationalUnitName(),         /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectiveOrganizationalUnitName {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// title ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              UnboundedDirectoryString
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"title"}
///   ID                       id-at-title }
/// ```
///
///
pub fn title() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())),     /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("title")])), /* OBJECT_FIELD_SETTING */
        id: id_at_title(),                      /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod title {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// organizationIdentifier ATTRIBUTE ::= {
///   WITH SYNTAX              UnboundedDirectoryString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"organizationIdentifier"}
///   ID                       id-at-organizationIdentifier }
/// ```
///
///
pub fn organizationIdentifier() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                         /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("organizationIdentifier")])), /* OBJECT_FIELD_SETTING */
        id: id_at_organizationIdentifier(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod organizationIdentifier {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// description ATTRIBUTE ::= {
///   WITH SYNTAX              UnboundedDirectoryString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"description"}
///   ID                       id-at-description }
/// ```
///
///
pub fn description() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("description")])), /* OBJECT_FIELD_SETTING */
        id: id_at_description(),                           /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod description {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// searchGuide ATTRIBUTE ::= {
///   WITH SYNTAX              Guide
///   LDAP-SYNTAX              guide.&id
///   LDAP-NAME                {"searchGuide"}
///   ID                       id-at-searchGuide }
/// ```
///
///
pub fn searchGuide() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(guide().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("searchGuide")])), /* OBJECT_FIELD_SETTING */
        id: id_at_searchGuide(),      /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
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

pub mod searchGuide {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Guide; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Guide(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Guide(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Guide(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Guide ::= SET {
///   objectClass  [0]  OBJECT-CLASS.&id OPTIONAL,
///   criteria     [1]  Criteria,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Guide {
    pub objectClass: OPTIONAL<OBJECT_IDENTIFIER>,
    pub criteria: Criteria,
    pub _unrecognized: Vec<X690Element>,
}
impl Guide {
    pub fn new(
        objectClass_: OPTIONAL<OBJECT_IDENTIFIER>,
        criteria: Criteria,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Guide {
            objectClass: objectClass_,
            criteria,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for Guide {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Guide(el)
    }
}

pub const _rctl1_components_for_Guide: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "objectClass",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criteria",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Guide: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Guide: &[ComponentSpec; 0] = &[];

pub fn _decode_Guide(el: &X690Element) -> ASN1Result<Guide> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Guide")),
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_Guide,
        _eal_components_for_Guide,
        _rctl2_components_for_Guide,
        30,
    )?;
    let objectClass_: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("objectClass") {
        Some(c_) => Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
            Ok(BER.decode_object_identifier(&el.inner()?)?)
        }(c_)?),
        _ => None,
    };
    let criteria_ =
        |el: &X690Element| -> ASN1Result<Criteria> { Ok(_decode_Criteria(&el.inner()?)?) }(
            _components.get("criteria").unwrap(),
        )?;
    Ok(Guide {
        objectClass: objectClass_,
        criteria: criteria_,
        _unrecognized,
    })
}

pub fn _encode_Guide(value_: &Guide) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.objectClass {
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&BER.encode_object_identifier(&v_1)?),
            ))
        }(&v_)?);
    }
    components_.push(|v_1: &Criteria| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(&_encode_Criteria(&v_1)?),
        ))
    }(&value_.criteria)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Guide(el: &X690Element) -> ASN1Result<()> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Guide")),
    };
    let (_components, _unrecognized) = _parse_set(
        elements.as_slice(),
        _rctl1_components_for_Guide,
        _eal_components_for_Guide,
        _rctl2_components_for_Guide,
        30,
    )?;
    match _components.get("objectClass") {
        Some(c_) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "objectClass")
                );
            }
            Ok(BER.validate_object_identifier(&el.inner()?)?)
        }(c_)?,
        _ => (),
    };
    |el: &X690Element| -> ASN1Result<()> {
        if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "criteria"));
        }
        Ok(_validate_Criteria(&el.inner()?)?)
    }(_components.get("criteria").unwrap())?;
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Criteria  ::=  CHOICE {
///   type  [0]  CriteriaItem,
///   and   [1]  SET OF Criteria,
///   or    [2]  SET OF Criteria,
///   not   [3]  Criteria,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum Criteria {
    type_(CriteriaItem),
    and(Vec<Criteria>),
    or(Vec<Criteria>),
    not(Box<Criteria>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for Criteria {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Criteria(el)
    }
}

pub fn _decode_Criteria(el: &X690Element) -> ASN1Result<Criteria> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(Criteria::type_(
            |el: &X690Element| -> ASN1Result<CriteriaItem> {
                Ok(_decode_CriteriaItem(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(Criteria::and(
            |el: &X690Element| -> ASN1Result<Vec<Criteria>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<Criteria>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and")
                            )
                        }
                    };
                    let mut items: SET_OF<Criteria> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_Criteria(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(Criteria::or(
            |el: &X690Element| -> ASN1Result<Vec<Criteria>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<Criteria>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or")
                            )
                        }
                    };
                    let mut items: SET_OF<Criteria> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_Criteria(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 3) => Ok(Criteria::not(Box::new(
            |el: &X690Element| -> ASN1Result<Criteria> { Ok(_decode_Criteria(&el.inner()?)?) }(
                &el,
            )?,
        ))),
        _ => Ok(Criteria::_unrecognized(el.clone())),
    }
}

pub fn _encode_Criteria(value_: &Criteria) -> ASN1Result<X690Element> {
    match value_ {
        Criteria::type_(v) => |v_1: &CriteriaItem| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_CriteriaItem(&v_1)?),
            ))
        }(&v),
        Criteria::and(v) => |v_1: &Vec<Criteria>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&|value_: &SET_OF<Criteria>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_Criteria(&v)?);
                    }
                    Ok(X690Element::new(
                        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                        X690Value::Constructed(Arc::new(children)),
                    ))
                }(&v_1)?),
            ))
        }(&v),
        Criteria::or(v) => |v_1: &Vec<Criteria>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(&|value_: &SET_OF<Criteria>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_Criteria(&v)?);
                    }
                    Ok(X690Element::new(
                        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                        X690Value::Constructed(Arc::new(children)),
                    ))
                }(&v_1)?),
            ))
        }(&v),
        Criteria::not(v) => |v_1: &Criteria| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(&_encode_Criteria(&v_1)?),
            ))
        }(&v),
        Criteria::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_Criteria(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "type"));
            }
            Ok(_validate_CriteriaItem(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and"));
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_Criteria(&sub)?;
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
                            _validate_Criteria(&sub)?;
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
            Ok(_validate_Criteria(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CriteriaItem  ::=  CHOICE {
///   equality          [0]  AttributeType,
///   substrings        [1]  AttributeType,
///   greaterOrEqual    [2]  AttributeType,
///   lessOrEqual       [3]  AttributeType,
///   approximateMatch  [4]  AttributeType,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum CriteriaItem {
    equality(AttributeType),
    substrings(AttributeType),
    greaterOrEqual(AttributeType),
    lessOrEqual(AttributeType),
    approximateMatch(AttributeType),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for CriteriaItem {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_CriteriaItem(el)
    }
}

pub fn _decode_CriteriaItem(el: &X690Element) -> ASN1Result<CriteriaItem> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(CriteriaItem::equality(
            |el: &X690Element| -> ASN1Result<AttributeType> {
                Ok(_decode_AttributeType(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(CriteriaItem::substrings(
            |el: &X690Element| -> ASN1Result<AttributeType> {
                Ok(_decode_AttributeType(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(CriteriaItem::greaterOrEqual(
            |el: &X690Element| -> ASN1Result<AttributeType> {
                Ok(_decode_AttributeType(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 3) => Ok(CriteriaItem::lessOrEqual(|el: &X690Element| -> ASN1Result<
            AttributeType,
        > {
            Ok(_decode_AttributeType(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 4) => Ok(CriteriaItem::approximateMatch(
            |el: &X690Element| -> ASN1Result<AttributeType> {
                Ok(_decode_AttributeType(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(CriteriaItem::_unrecognized(el.clone())),
    }
}

pub fn _encode_CriteriaItem(value_: &CriteriaItem) -> ASN1Result<X690Element> {
    match value_ {
        CriteriaItem::equality(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_AttributeType(&v_1)?),
            ))
        }(&v),
        CriteriaItem::substrings(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_AttributeType(&v_1)?),
            ))
        }(&v),
        CriteriaItem::greaterOrEqual(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(&_encode_AttributeType(&v_1)?),
            ))
        }(&v),
        CriteriaItem::lessOrEqual(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(&_encode_AttributeType(&v_1)?),
            ))
        }(&v),
        CriteriaItem::approximateMatch(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::from_explicit(&_encode_AttributeType(&v_1)?),
            ))
        }(&v),
        CriteriaItem::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_CriteriaItem(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "equality"));
            }
            Ok(_validate_AttributeType(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "substrings"));
            }
            Ok(_validate_AttributeType(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "greaterOrEqual")
                );
            }
            Ok(_validate_AttributeType(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "lessOrEqual")
                );
            }
            Ok(_validate_AttributeType(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 4) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "approximateMatch")
                );
            }
            Ok(_validate_AttributeType(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// enhancedSearchGuide ATTRIBUTE ::= {
///   WITH SYNTAX              EnhancedGuide
///   LDAP-SYNTAX              enhancedGuide.&id
///   LDAP-NAME                {"enhancedSearchGuide"}
///   ID                       id-at-enhancedSearchGuide }
/// ```
///
///
pub fn enhancedSearchGuide() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(enhancedGuide().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("enhancedSearchGuide")])), /* OBJECT_FIELD_SETTING */
        id: id_at_enhancedSearchGuide(),      /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
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

pub mod enhancedSearchGuide {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = EnhancedGuide; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_EnhancedGuide(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_EnhancedGuide(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_EnhancedGuide(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EnhancedGuide ::= SEQUENCE {
///   objectClass  [0]  OBJECT-CLASS.&id,
///   criteria     [1]  Criteria,
///   subset       [2]  INTEGER {
///     baseObject   (0),
///     oneLevel     (1),
///     wholeSubtree (2)} DEFAULT oneLevel,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct EnhancedGuide {
    pub objectClass: OBJECT_IDENTIFIER,
    pub criteria: Criteria,
    pub subset: OPTIONAL<EnhancedGuide_subset>,
    pub _unrecognized: Vec<X690Element>,
}
impl EnhancedGuide {
    pub fn new(
        objectClass_: OBJECT_IDENTIFIER,
        criteria: Criteria,
        subset: OPTIONAL<EnhancedGuide_subset>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        EnhancedGuide {
            objectClass: objectClass_,
            criteria,
            subset,
            _unrecognized,
        }
    }
    pub fn _default_value_for_subset() -> EnhancedGuide_subset {
        EnhancedGuide_subset_oneLevel
    }
}
impl TryFrom<&X690Element> for EnhancedGuide {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_EnhancedGuide(el)
    }
}

pub const _rctl1_components_for_EnhancedGuide: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "objectClass",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "criteria",
        false,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subset",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EnhancedGuide: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EnhancedGuide: &[ComponentSpec; 0] = &[];

pub fn _decode_EnhancedGuide(el: &X690Element) -> ASN1Result<EnhancedGuide> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EnhancedGuide")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EnhancedGuide,
        _eal_components_for_EnhancedGuide,
        _rctl2_components_for_EnhancedGuide,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut objectClass_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut criteria_: OPTIONAL<Criteria> = None;
    let mut subset_: OPTIONAL<EnhancedGuide_subset> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "objectClass" => {
                objectClass_ = Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                    Ok(BER.decode_object_identifier(&el.inner()?)?)
                }(_el)?)
            }
            "criteria" => {
                criteria_ = Some(|el: &X690Element| -> ASN1Result<Criteria> {
                    Ok(_decode_Criteria(&el.inner()?)?)
                }(_el)?)
            }
            "subset" => {
                subset_ = Some(|el: &X690Element| -> ASN1Result<EnhancedGuide_subset> {
                    Ok(_decode_EnhancedGuide_subset(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(EnhancedGuide {
        objectClass: objectClass_.unwrap(),
        criteria: criteria_.unwrap(),
        subset: subset_,
        _unrecognized,
    })
}

pub fn _encode_EnhancedGuide(value_: &EnhancedGuide) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&BER.encode_object_identifier(&v_1)?),
        ))
    }(&value_.objectClass)?);
    components_.push(|v_1: &Criteria| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 1),
            X690Value::from_explicit(&_encode_Criteria(&v_1)?),
        ))
    }(&value_.criteria)?);
    if let Some(v_) = &value_.subset {
        if *v_ != EnhancedGuide::_default_value_for_subset() {
            components_.push(|v_1: &EnhancedGuide_subset| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(&_encode_EnhancedGuide_subset(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_EnhancedGuide(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EnhancedGuide")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EnhancedGuide,
        _eal_components_for_EnhancedGuide,
        _rctl2_components_for_EnhancedGuide,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "objectClass" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "objectClass")
                    );
                }
                Ok(BER.validate_object_identifier(&el.inner()?)?)
            }(_el)?,
            "criteria" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "criteria")
                    );
                }
                Ok(_validate_Criteria(&el.inner()?)?)
            }(_el)?,
            "subset" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "subset"));
                }
                Ok(_validate_EnhancedGuide_subset(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// businessCategory ATTRIBUTE ::= {
///   WITH SYNTAX              UnboundedDirectoryString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"businessCategory"}
///   ID                       id-at-businessCategory }
/// ```
///
///
pub fn businessCategory() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("businessCategory")])), /* OBJECT_FIELD_SETTING */
        id: id_at_businessCategory(),                      /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod businessCategory {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// postalAddress ATTRIBUTE ::= {
///   WITH SYNTAX              PostalAddress
///   EQUALITY MATCHING RULE   caseIgnoreListMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreListSubstringsMatch
///   LDAP-SYNTAX              postalAddr.&id
///   LDAP-NAME                {"postalAddress"}
///   ID                       id-at-postalAddress }
/// ```
///
///
pub fn postalAddress() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreListMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreListSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(postalAddr().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("postalAddress")])), /* OBJECT_FIELD_SETTING */
        id: id_at_postalAddress(),         /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod postalAddress {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PostalAddress; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PostalAddress(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PostalAddress(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PostalAddress(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PostalAddress  ::=  SEQUENCE SIZE (1..MAX) OF UnboundedDirectoryString
/// ```
pub type PostalAddress = Vec<UnboundedDirectoryString>; // SequenceOfType

pub fn _decode_PostalAddress(el: &X690Element) -> ASN1Result<PostalAddress> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PostalAddress")),
    };
    let mut items: SEQUENCE_OF<UnboundedDirectoryString> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_UnboundedDirectoryString(el)?);
    }
    Ok(items)
}

pub fn _encode_PostalAddress(value_: &PostalAddress) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_UnboundedDirectoryString(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_PostalAddress(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_UnboundedDirectoryString(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PostalAddress")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectivePostalAddress ATTRIBUTE ::= {
///   SUBTYPE OF               postalAddress
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              postalAddr.&id
///   LDAP-NAME                {"c-PostalAddress"}
///   ID                       id-at-collectivePostalAddress }
/// ```
///
///
pub fn collectivePostalAddress() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(postalAddress())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                      /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(postalAddr().id),           /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-PostalAddress")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectivePostalAddress(),         /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectivePostalAddress {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// postalCode ATTRIBUTE ::= {
///   WITH SYNTAX              UnboundedDirectoryString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"postalCode"}
///   ID                       id-at-postalCode }
/// ```
///
///
pub fn postalCode() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("postalCode")])), /* OBJECT_FIELD_SETTING */
        id: id_at_postalCode(),                            /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod postalCode {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectivePostalCode ATTRIBUTE ::= {
///   SUBTYPE OF               postalCode
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"c-PostalCode"}
///   ID                       id-at-collectivePostalCode }
/// ```
///
///
pub fn collectivePostalCode() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(postalCode())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                   /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),   /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-PostalCode")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectivePostalCode(),         /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectivePostalCode {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// postOfficeBox ATTRIBUTE ::= {
///   WITH SYNTAX              UnboundedDirectoryString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"postOfficeBox"}
///   ID                       id-at-postOfficeBox }
/// ```
///
///
pub fn postOfficeBox() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("postOfficeBox")])), /* OBJECT_FIELD_SETTING */
        id: id_at_postOfficeBox(),                         /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod postOfficeBox {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectivePostOfficeBox ATTRIBUTE ::= {
///   SUBTYPE OF               postOfficeBox
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"c-PostOfficeBox"}
///   ID                       id-at-collectivePostOfficeBox }
/// ```
///
///
pub fn collectivePostOfficeBox() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(postOfficeBox())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                      /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),      /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-PostOfficeBox")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectivePostOfficeBox(),         /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectivePostOfficeBox {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// physicalDeliveryOfficeName ATTRIBUTE ::= {
///   WITH SYNTAX              UnboundedDirectoryString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"physicalDeliveryOfficeName"}
///   ID                       id-at-physicalDeliveryOfficeName }
/// ```
///
///
pub fn physicalDeliveryOfficeName() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("physicalDeliveryOfficeName")])), /* OBJECT_FIELD_SETTING */
        id: id_at_physicalDeliveryOfficeName(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod physicalDeliveryOfficeName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectivePhysicalDeliveryOfficeName ATTRIBUTE ::= {
///   SUBTYPE OF               physicalDeliveryOfficeName
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"c-PhysicalDeliveryOfficeName"}
///   ID                       id-at-collectivePhysicalDeliveryOfficeName }
/// ```
///
///
pub fn collectivePhysicalDeliveryOfficeName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(physicalDeliveryOfficeName())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                                   /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),                   /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-PhysicalDeliveryOfficeName")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectivePhysicalDeliveryOfficeName(), /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectivePhysicalDeliveryOfficeName {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// telephoneNumber ATTRIBUTE ::= {
///   WITH SYNTAX              TelephoneNumber
///   EQUALITY MATCHING RULE   telephoneNumberMatch
///   SUBSTRINGS MATCHING RULE telephoneNumberSubstringsMatch
///   LDAP-SYNTAX              printableString.&id
///   LDAP-NAME                {"telephoneNumber"}
///   ID                       id-at-telephoneNumber }
/// ```
///
///
pub fn telephoneNumber() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(telephoneNumberMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(telephoneNumberSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(printableString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("telephoneNumber")])), /* OBJECT_FIELD_SETTING */
        id: id_at_telephoneNumber(),            /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod telephoneNumber {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = TelephoneNumber; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_TelephoneNumber(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_TelephoneNumber(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_TelephoneNumber(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TelephoneNumber  ::=  PrintableString(SIZE (1..ub-telephone-number))
/// ```
pub type TelephoneNumber = PrintableString; // PrintableString

pub fn _decode_TelephoneNumber(el: &X690Element) -> ASN1Result<TelephoneNumber> {
    BER.decode_printable_string(&el)
}

pub fn _encode_TelephoneNumber(value_: &TelephoneNumber) -> ASN1Result<X690Element> {
    BER.encode_printable_string(&value_)
}

pub fn _validate_TelephoneNumber(el: &X690Element) -> ASN1Result<()> {
    BER.validate_printable_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-telephone-number INTEGER ::= 32
/// ```
///
///
pub const ub_telephone_number: i64 = 32;

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveTelephoneNumber ATTRIBUTE ::= {
///   SUBTYPE OF               telephoneNumber
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              printableString.&id
///   LDAP-NAME                {"c-TelephoneNumber"}
///   ID                       id-at-collectiveTelephoneNumber }
/// ```
///
///
pub fn collectiveTelephoneNumber() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(telephoneNumber())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                        /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(printableString().id),        /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-TelephoneNumber")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectiveTelephoneNumber(),         /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectiveTelephoneNumber {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// telexNumber ATTRIBUTE ::= {
///   WITH SYNTAX              TelexNumber
///   LDAP-SYNTAX              telexNr.&id
///   LDAP-NAME                {"telexNumber"}
///   ID                       id-at-telexNumber }
/// ```
///
///
pub fn telexNumber() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(telexNr().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("telexNumber")])), /* OBJECT_FIELD_SETTING */
        id: id_at_telexNumber(),        /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
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

pub mod telexNumber {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = TelexNumber; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_TelexNumber(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_TelexNumber(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_TelexNumber(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TelexNumber ::= SEQUENCE {
///   telexNumber  PrintableString(SIZE (1..ub-telex-number)),
///   countryCode  PrintableString(SIZE (1..ub-country-code)),
///   answerback   PrintableString(SIZE (1..ub-answerback)),
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct TelexNumber {
    pub telexNumber: PrintableString,
    pub countryCode: PrintableString,
    pub answerback: PrintableString,
    pub _unrecognized: Vec<X690Element>,
}
impl TelexNumber {
    pub fn new(
        telexNumber: PrintableString,
        countryCode: PrintableString,
        answerback: PrintableString,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TelexNumber {
            telexNumber,
            countryCode,
            answerback,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for TelexNumber {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TelexNumber(el)
    }
}

pub const _rctl1_components_for_TelexNumber: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "telexNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 19)),
        None,
        None,
    ),
    ComponentSpec::new(
        "countryCode",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 19)),
        None,
        None,
    ),
    ComponentSpec::new(
        "answerback",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 19)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TelexNumber: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TelexNumber: &[ComponentSpec; 0] = &[];

pub fn _decode_TelexNumber(el: &X690Element) -> ASN1Result<TelexNumber> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TelexNumber")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TelexNumber,
        _eal_components_for_TelexNumber,
        _rctl2_components_for_TelexNumber,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut telexNumber_: OPTIONAL<PrintableString> = None;
    let mut countryCode_: OPTIONAL<PrintableString> = None;
    let mut answerback_: OPTIONAL<PrintableString> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "telexNumber" => telexNumber_ = Some(BER.decode_printable_string(_el)?),
            "countryCode" => countryCode_ = Some(BER.decode_printable_string(_el)?),
            "answerback" => answerback_ = Some(BER.decode_printable_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TelexNumber {
        telexNumber: telexNumber_.unwrap(),
        countryCode: countryCode_.unwrap(),
        answerback: answerback_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_TelexNumber(value_: &TelexNumber) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(BER.encode_printable_string(&value_.telexNumber)?);
    components_.push(BER.encode_printable_string(&value_.countryCode)?);
    components_.push(BER.encode_printable_string(&value_.answerback)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_TelexNumber(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TelexNumber")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TelexNumber,
        _eal_components_for_TelexNumber,
        _rctl2_components_for_TelexNumber,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "telexNumber" => BER.validate_printable_string(_el)?,
            "countryCode" => BER.validate_printable_string(_el)?,
            "answerback" => BER.validate_printable_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-telex-number INTEGER ::= 14
/// ```
///
///
pub const ub_telex_number: i64 = 14;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-country-code INTEGER ::= 4
/// ```
///
///
pub const ub_country_code: i64 = 4;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-answerback   INTEGER ::= 8
/// ```
///
///
pub const ub_answerback: i64 = 8;

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveTelexNumber ATTRIBUTE ::= {
///   SUBTYPE OF               telexNumber
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              telexNr.&id
///   LDAP-NAME                {"c-TelexNumber"}
///   ID                       id-at-collectiveTelexNumber }
/// ```
///
///
pub fn collectiveTelexNumber() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(telexNumber())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                    /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(telexNr().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-TelexNumber")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectiveTelexNumber(),         /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectiveTelexNumber {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// facsimileTelephoneNumber ATTRIBUTE ::= {
///   WITH SYNTAX              FacsimileTelephoneNumber
///   EQUALITY MATCHING RULE   facsimileNumberMatch
///   SUBSTRINGS MATCHING RULE facsimileNumberSubstringsMatch
///   LDAP-SYNTAX              facsimileTelephoneNr.&id
///   LDAP-NAME                {"facsimileTelephoneNumber"}
///   ID                       id-at-facsimileTelephoneNumber }
/// ```
///
///
pub fn facsimileTelephoneNumber() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(facsimileNumberMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(facsimileNumberSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(facsimileTelephoneNr().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("facsimileTelephoneNumber")])), /* OBJECT_FIELD_SETTING */
        id: id_at_facsimileTelephoneNumber(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod facsimileTelephoneNumber {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = FacsimileTelephoneNumber; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_FacsimileTelephoneNumber(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_FacsimileTelephoneNumber(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_FacsimileTelephoneNumber(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// FacsimileTelephoneNumber ::= SEQUENCE {
///   telephoneNumber  TelephoneNumber,
///   parameters       G3FacsimileNonBasicParameters OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct FacsimileTelephoneNumber {
    pub telephoneNumber: TelephoneNumber,
    pub parameters: OPTIONAL<G3FacsimileNonBasicParameters>,
    pub _unrecognized: Vec<X690Element>,
}
impl FacsimileTelephoneNumber {
    pub fn new(
        telephoneNumber: TelephoneNumber,
        parameters: OPTIONAL<G3FacsimileNonBasicParameters>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        FacsimileTelephoneNumber {
            telephoneNumber,
            parameters,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for FacsimileTelephoneNumber {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_FacsimileTelephoneNumber(el)
    }
}

pub const _rctl1_components_for_FacsimileTelephoneNumber: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "telephoneNumber",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 19)),
        None,
        None,
    ),
    ComponentSpec::new(
        "parameters",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_FacsimileTelephoneNumber: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_FacsimileTelephoneNumber: &[ComponentSpec; 0] = &[];

pub fn _decode_FacsimileTelephoneNumber(el: &X690Element) -> ASN1Result<FacsimileTelephoneNumber> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "FacsimileTelephoneNumber",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_FacsimileTelephoneNumber,
        _eal_components_for_FacsimileTelephoneNumber,
        _rctl2_components_for_FacsimileTelephoneNumber,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut telephoneNumber_: OPTIONAL<TelephoneNumber> = None;
    let mut parameters_: OPTIONAL<G3FacsimileNonBasicParameters> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "telephoneNumber" => telephoneNumber_ = Some(_decode_TelephoneNumber(_el)?),
            "parameters" => parameters_ = Some(_decode_G3FacsimileNonBasicParameters(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(FacsimileTelephoneNumber {
        telephoneNumber: telephoneNumber_.unwrap(),
        parameters: parameters_,
        _unrecognized,
    })
}

pub fn _encode_FacsimileTelephoneNumber(
    value_: &FacsimileTelephoneNumber,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_TelephoneNumber(&value_.telephoneNumber)?);
    if let Some(v_) = &value_.parameters {
        components_.push(_encode_G3FacsimileNonBasicParameters(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_FacsimileTelephoneNumber(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "FacsimileTelephoneNumber",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_FacsimileTelephoneNumber,
        _eal_components_for_FacsimileTelephoneNumber,
        _rctl2_components_for_FacsimileTelephoneNumber,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "telephoneNumber" => _validate_TelephoneNumber(_el)?,
            "parameters" => _validate_G3FacsimileNonBasicParameters(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveFacsimileTelephoneNumber ATTRIBUTE ::= {
///   SUBTYPE OF               facsimileTelephoneNumber
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              facsimileTelephoneNr.&id
///   LDAP-NAME                {"c-FacsimileTelephoneNumber"}
///   ID                       id-at-collectiveFacsimileTelephoneNumber }
/// ```
///
///
pub fn collectiveFacsimileTelephoneNumber() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(facsimileTelephoneNumber())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                                 /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(facsimileTelephoneNr().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-FacsimileTelephoneNumber")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectiveFacsimileTelephoneNumber(), /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectiveFacsimileTelephoneNumber {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// x121Address ATTRIBUTE ::= {
///   WITH SYNTAX              X121Address
///   EQUALITY MATCHING RULE   numericStringMatch
///   SUBSTRINGS MATCHING RULE numericStringSubstringsMatch
///   LDAP-SYNTAX              numericString.&id
///   LDAP-NAME                {"x121Address"}
///   ID                       id-at-x121Address }
/// ```
///
///
pub fn x121Address() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(numericStringMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(numericStringSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(numericString().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("x121Address")])), /* OBJECT_FIELD_SETTING */
        id: id_at_x121Address(),                              /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod x121Address {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = X121Address; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_X121Address(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_X121Address(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_X121Address(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// X121Address  ::=  NumericString(SIZE (1..ub-x121-address))
/// ```
pub type X121Address = NumericString; // NumericString

pub fn _decode_X121Address(el: &X690Element) -> ASN1Result<X121Address> {
    BER.decode_numeric_string(&el)
}

pub fn _encode_X121Address(value_: &X121Address) -> ASN1Result<X690Element> {
    BER.encode_numeric_string(&value_)
}

pub fn _validate_X121Address(el: &X690Element) -> ASN1Result<()> {
    BER.validate_numeric_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-x121-address INTEGER ::= 15
/// ```
///
///
pub const ub_x121_address: i64 = 15;

/// ### ASN.1 Definition:
///
/// ```asn1
/// internationalISDNNumber ATTRIBUTE ::= {
///   WITH SYNTAX              InternationalISDNNumber
///   EQUALITY MATCHING RULE   numericStringMatch
///   SUBSTRINGS MATCHING RULE numericStringSubstringsMatch
///   LDAP-SYNTAX              numericString.&id
///   LDAP-NAME                {"internationalISDNNumber"}
///   ID                       id-at-internationalISDNNumber }
/// ```
///
///
pub fn internationalISDNNumber() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(numericStringMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(numericStringSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(numericString().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("internationalISDNNumber")])), /* OBJECT_FIELD_SETTING */
        id: id_at_internationalISDNNumber(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod internationalISDNNumber {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = InternationalISDNNumber; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_InternationalISDNNumber(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_InternationalISDNNumber(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_InternationalISDNNumber(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// InternationalISDNNumber  ::=
///   NumericString(SIZE (1..ub-international-isdn-number))
/// ```
pub type InternationalISDNNumber = NumericString; // NumericString

pub fn _decode_InternationalISDNNumber(el: &X690Element) -> ASN1Result<InternationalISDNNumber> {
    BER.decode_numeric_string(&el)
}

pub fn _encode_InternationalISDNNumber(
    value_: &InternationalISDNNumber,
) -> ASN1Result<X690Element> {
    BER.encode_numeric_string(&value_)
}

pub fn _validate_InternationalISDNNumber(el: &X690Element) -> ASN1Result<()> {
    BER.validate_numeric_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-international-isdn-number INTEGER ::= 16
/// ```
///
///
pub const ub_international_isdn_number: i64 = 16;

/// ### ASN.1 Definition:
///
/// ```asn1
/// collectiveInternationalISDNNumber ATTRIBUTE ::= {
///   SUBTYPE OF               internationalISDNNumber
///   COLLECTIVE               TRUE
///   LDAP-SYNTAX              numericString.&id
///   LDAP-NAME                {"c-InternationalISDNNumber"}
///   ID                       id-at-collectiveInternationalISDNNumber }
/// ```
///
///
pub fn collectiveInternationalISDNNumber() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(internationalISDNNumber())), /* OBJECT_FIELD_SETTING */
        collective: Some(true),                                /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(numericString().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("c-InternationalISDNNumber")])), /* OBJECT_FIELD_SETTING */
        id: id_at_collectiveInternationalISDNNumber(), /* OBJECT_FIELD_SETTING */
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod collectiveInternationalISDNNumber {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// registeredAddress ATTRIBUTE ::= {
///   SUBTYPE OF               postalAddress
///   WITH SYNTAX              PostalAddress
///   LDAP-SYNTAX              postalAddr.&id
///   LDAP-NAME                {"registeredAddress"}
///   ID                       id-at-registeredAddress }
/// ```
///
///
pub fn registeredAddress() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(postalAddress())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(postalAddr().id),           /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("registeredAddress")])), /* OBJECT_FIELD_SETTING */
        id: id_at_registeredAddress(),               /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod registeredAddress {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PostalAddress; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PostalAddress(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PostalAddress(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PostalAddress(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// destinationIndicator ATTRIBUTE ::= {
///   WITH SYNTAX              DestinationIndicator
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              printableString.&id
///   LDAP-NAME                {"destinationIndicator"}
///   ID                       id-at-destinationIndicator }
/// ```
///
///
pub fn destinationIndicator() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(printableString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("destinationIndicator")])), /* OBJECT_FIELD_SETTING */
        id: id_at_destinationIndicator(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod destinationIndicator {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DestinationIndicator; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DestinationIndicator(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DestinationIndicator(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DestinationIndicator(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DestinationIndicator  ::=  PrintableString(SIZE (1..MAX))
/// ```
pub type DestinationIndicator = PrintableString; // PrintableString

pub fn _decode_DestinationIndicator(el: &X690Element) -> ASN1Result<DestinationIndicator> {
    BER.decode_printable_string(&el)
}

pub fn _encode_DestinationIndicator(value_: &DestinationIndicator) -> ASN1Result<X690Element> {
    BER.encode_printable_string(&value_)
}

pub fn _validate_DestinationIndicator(el: &X690Element) -> ASN1Result<()> {
    BER.validate_printable_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// communicationsService ATTRIBUTE ::= {
///   WITH SYNTAX              CommunicationsService
///   EQUALITY MATCHING RULE   objectIdentifierMatch
///   LDAP-SYNTAX              oid.&id
///   LDAP-NAME                {"communicationsService"}
///   ID                       id-at-communicationsService }
/// ```
///
///
pub fn communicationsService() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),                              /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("communicationsService")])), /* OBJECT_FIELD_SETTING */
        id: id_at_communicationsService(), /* OBJECT_FIELD_SETTING */
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

pub mod communicationsService {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CommunicationsService; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CommunicationsService(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CommunicationsService(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CommunicationsService(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CommunicationsService  ::=  OBJECT IDENTIFIER
/// ```
pub type CommunicationsService = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_CommunicationsService(el: &X690Element) -> ASN1Result<CommunicationsService> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_CommunicationsService(value_: &CommunicationsService) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_CommunicationsService(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// communicationsNetwork ATTRIBUTE ::= {
///   WITH SYNTAX              CommunicationsNetwork
///   EQUALITY MATCHING RULE   objectIdentifierMatch
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              oid.&id
///   LDAP-NAME                {"communicationsNetwork"}
///   ID                       id-at-communicationsNetwork }
/// ```
///
///
pub fn communicationsNetwork() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                               /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),                              /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("communicationsNetwork")])), /* OBJECT_FIELD_SETTING */
        id: id_at_communicationsNetwork(), /* OBJECT_FIELD_SETTING */
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

pub mod communicationsNetwork {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CommunicationsNetwork; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CommunicationsNetwork(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CommunicationsNetwork(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CommunicationsNetwork(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CommunicationsNetwork  ::=  OBJECT IDENTIFIER
/// ```
pub type CommunicationsNetwork = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_CommunicationsNetwork(el: &X690Element) -> ASN1Result<CommunicationsNetwork> {
    BER.decode_object_identifier(&el)
}

pub fn _encode_CommunicationsNetwork(value_: &CommunicationsNetwork) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_CommunicationsNetwork(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// preferredDeliveryMethod ATTRIBUTE ::= {
///   WITH SYNTAX              PreferredDeliveryMethod
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              deliveryMethod.&id
///   LDAP-NAME                {"preferredDeliveryMethod"}
///   ID                       id-at-preferredDeliveryMethod }
/// ```
///
///
pub fn preferredDeliveryMethod() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),             /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(deliveryMethod().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("preferredDeliveryMethod")])), /* OBJECT_FIELD_SETTING */
        id: id_at_preferredDeliveryMethod(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
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

pub mod preferredDeliveryMethod {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PreferredDeliveryMethod; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PreferredDeliveryMethod(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PreferredDeliveryMethod(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PreferredDeliveryMethod(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PreferredDeliveryMethod  ::=  SEQUENCE OF INTEGER {
///   any-delivery-method   (0),
///   mhs-delivery          (1),
///   physical-delivery     (2),
///   telex-delivery        (3),
///   teletex-delivery      (4),
///   g3-facsimile-delivery (5),
///   g4-facsimile-delivery (6),
///   ia5-terminal-delivery (7),
///   videotex-delivery     (8),
///   telephone-delivery    (9) }
/// ```
pub type PreferredDeliveryMethod = Vec<PreferredDeliveryMethod_Item>; // SequenceOfType

pub fn _decode_PreferredDeliveryMethod(el: &X690Element) -> ASN1Result<PreferredDeliveryMethod> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "PreferredDeliveryMethod",
            ))
        }
    };
    let mut items: SEQUENCE_OF<PreferredDeliveryMethod_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_PreferredDeliveryMethod_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_PreferredDeliveryMethod(
    value_: &PreferredDeliveryMethod,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_PreferredDeliveryMethod_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_PreferredDeliveryMethod(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_PreferredDeliveryMethod_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "PreferredDeliveryMethod",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// presentationAddress ATTRIBUTE ::= {
///   WITH SYNTAX              PresentationAddress
///   EQUALITY MATCHING RULE   presentationAddressMatch
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              presentationAddr.&id
///   LDAP-NAME                {"presentationAddress"}
///   ID                       id-at-presentationAddress }
/// ```
///
///
pub fn presentationAddress() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(presentationAddressMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                                  /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(presentationAddr().id),                    /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("presentationAddress")])), /* OBJECT_FIELD_SETTING */
        id: id_at_presentationAddress(),                            /* OBJECT_FIELD_SETTING */
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

pub mod presentationAddress {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PresentationAddress; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PresentationAddress(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PresentationAddress(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PresentationAddress(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PresentationAddress ::= SEQUENCE {
///   pSelector   [0]  OCTET STRING OPTIONAL,
///   sSelector   [1]  OCTET STRING OPTIONAL,
///   tSelector   [2]  OCTET STRING OPTIONAL,
///   nAddresses  [3]  SET SIZE (1..MAX) OF OCTET STRING,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct PresentationAddress {
    pub pSelector: OPTIONAL<OCTET_STRING>,
    pub sSelector: OPTIONAL<OCTET_STRING>,
    pub tSelector: OPTIONAL<OCTET_STRING>,
    pub nAddresses: Vec<OCTET_STRING>,
    pub _unrecognized: Vec<X690Element>,
}
impl PresentationAddress {
    pub fn new(
        pSelector: OPTIONAL<OCTET_STRING>,
        sSelector: OPTIONAL<OCTET_STRING>,
        tSelector: OPTIONAL<OCTET_STRING>,
        nAddresses: Vec<OCTET_STRING>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        PresentationAddress {
            pSelector,
            sSelector,
            tSelector,
            nAddresses,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for PresentationAddress {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PresentationAddress(el)
    }
}

pub const _rctl1_components_for_PresentationAddress: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "pSelector",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "sSelector",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "tSelector",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "nAddresses",
        false,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PresentationAddress: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PresentationAddress: &[ComponentSpec; 0] = &[];

pub fn _decode_PresentationAddress(el: &X690Element) -> ASN1Result<PresentationAddress> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PresentationAddress")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PresentationAddress,
        _eal_components_for_PresentationAddress,
        _rctl2_components_for_PresentationAddress,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut pSelector_: OPTIONAL<OCTET_STRING> = None;
    let mut sSelector_: OPTIONAL<OCTET_STRING> = None;
    let mut tSelector_: OPTIONAL<OCTET_STRING> = None;
    let mut nAddresses_: OPTIONAL<Vec<OCTET_STRING>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "pSelector" => {
                pSelector_ = Some(|el: &X690Element| -> ASN1Result<OCTET_STRING> {
                    Ok(BER.decode_octet_string(&el.inner()?)?)
                }(_el)?)
            }
            "sSelector" => {
                sSelector_ = Some(|el: &X690Element| -> ASN1Result<OCTET_STRING> {
                    Ok(BER.decode_octet_string(&el.inner()?)?)
                }(_el)?)
            }
            "tSelector" => {
                tSelector_ = Some(|el: &X690Element| -> ASN1Result<OCTET_STRING> {
                    Ok(BER.decode_octet_string(&el.inner()?)?)
                }(_el)?)
            }
            "nAddresses" => {
                nAddresses_ = Some(|el: &X690Element| -> ASN1Result<Vec<OCTET_STRING>> {
                    Ok(|el: &X690Element| -> ASN1Result<SET_OF<OCTET_STRING>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "nAddresses",
                                ))
                            }
                        };
                        let mut items: SET_OF<OCTET_STRING> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(BER.decode_octet_string(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(PresentationAddress {
        pSelector: pSelector_,
        sSelector: sSelector_,
        tSelector: tSelector_,
        nAddresses: nAddresses_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_PresentationAddress(value_: &PresentationAddress) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(14);
    if let Some(v_) = &value_.pSelector {
        components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&BER.encode_octet_string(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.sSelector {
        components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&BER.encode_octet_string(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.tSelector {
        components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(&BER.encode_octet_string(&v_1)?),
            ))
        }(&v_)?);
    }
    components_.push(|v_1: &Vec<OCTET_STRING>| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 3),
            X690Value::from_explicit(
                &|value_: &SET_OF<OCTET_STRING>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(BER.encode_octet_string(&v)?);
                    }
                    Ok(X690Element::new(
                        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                        X690Value::Constructed(Arc::new(children)),
                    ))
                }(&v_1)?,
            ),
        ))
    }(&value_.nAddresses)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_PresentationAddress(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PresentationAddress")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PresentationAddress,
        _eal_components_for_PresentationAddress,
        _rctl2_components_for_PresentationAddress,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "pSelector" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "pSelector")
                    );
                }
                Ok(BER.validate_octet_string(&el.inner()?)?)
            }(_el)?,
            "sSelector" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "sSelector")
                    );
                }
                Ok(BER.validate_octet_string(&el.inner()?)?)
            }(_el)?,
            "tSelector" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "tSelector")
                    );
                }
                Ok(BER.validate_octet_string(&el.inner()?)?)
            }(_el)?,
            "nAddresses" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "nAddresses")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                BER.validate_octet_string(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "nAddresses")
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
/// supportedApplicationContext ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE   objectIdentifierMatch
///   LDAP-SYNTAX              oid.&id
///   LDAP-NAME                {"supportedApplicationContext"}
///   ID                       id-at-supportedApplicationContext }
/// ```
///
///
pub fn supportedApplicationContext() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),                              /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("supportedApplicationContext")])), /* OBJECT_FIELD_SETTING */
        id: id_at_supportedApplicationContext(), /* OBJECT_FIELD_SETTING */
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

pub mod supportedApplicationContext {
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
/// protocolInformation ATTRIBUTE ::= {
///   WITH SYNTAX              ProtocolInformation
///   EQUALITY MATCHING RULE   protocolInformationMatch
///   ID                       id-at-protocolInformation }
/// ```
///
///
pub fn protocolInformation() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(protocolInformationMatch())), /* OBJECT_FIELD_SETTING */
        id: id_at_protocolInformation(),                            /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod protocolInformation {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ProtocolInformation; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ProtocolInformation(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ProtocolInformation(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ProtocolInformation(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ProtocolInformation ::= SEQUENCE {
///   nAddress  OCTET STRING,
///   profiles  SET OF OBJECT IDENTIFIER }
/// ```
///
#[derive(Debug, Clone)]
pub struct ProtocolInformation {
    pub nAddress: OCTET_STRING,
    pub profiles: Vec<OBJECT_IDENTIFIER>,
}
impl ProtocolInformation {
    pub fn new(nAddress: OCTET_STRING, profiles: Vec<OBJECT_IDENTIFIER>) -> Self {
        ProtocolInformation { nAddress, profiles }
    }
}
impl TryFrom<&X690Element> for ProtocolInformation {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_ProtocolInformation(el)
    }
}

pub const _rctl1_components_for_ProtocolInformation: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "nAddress",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
    ComponentSpec::new(
        "profiles",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 17)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ProtocolInformation: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ProtocolInformation: &[ComponentSpec; 0] = &[];

pub fn _decode_ProtocolInformation(el: &X690Element) -> ASN1Result<ProtocolInformation> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ProtocolInformation")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ProtocolInformation,
        _eal_components_for_ProtocolInformation,
        _rctl2_components_for_ProtocolInformation,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut nAddress_: OPTIONAL<OCTET_STRING> = None;
    let mut profiles_: OPTIONAL<Vec<OBJECT_IDENTIFIER>> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "nAddress" => nAddress_ = Some(BER.decode_octet_string(_el)?),
            "profiles" => {
                profiles_ = Some(
                    |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "profiles",
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
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ProtocolInformation"))
            }
        }
    }
    Ok(ProtocolInformation {
        nAddress: nAddress_.unwrap(),
        profiles: profiles_.unwrap(),
    })
}

pub fn _encode_ProtocolInformation(value_: &ProtocolInformation) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_octet_string(&value_.nAddress)?);
    components_.push(
        |value_: &SET_OF<OBJECT_IDENTIFIER>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(BER.encode_object_identifier(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.profiles)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_ProtocolInformation(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ProtocolInformation")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_ProtocolInformation,
        _eal_components_for_ProtocolInformation,
        _rctl2_components_for_ProtocolInformation,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "nAddress" => BER.validate_octet_string(_el)?,
            "profiles" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            BER.validate_object_identifier(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "profiles")),
                }
            }(_el)?,
            _ => {
                return Err(_el
                    .to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ProtocolInformation"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// distinguishedName ATTRIBUTE ::= {
///   WITH SYNTAX              DistinguishedName
///   EQUALITY MATCHING RULE   distinguishedNameMatch
///   LDAP-SYNTAX              dn.&id
///   LDAP-NAME                {"distinguishedName"}
///   ID                       id-at-distinguishedName }
/// ```
///
///
pub fn distinguishedName() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(distinguishedNameMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dn().id),                                /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("distinguishedName")])), /* OBJECT_FIELD_SETTING */
        id: id_at_distinguishedName(),                            /* OBJECT_FIELD_SETTING */
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

pub mod distinguishedName {
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
/// member ATTRIBUTE ::= {
///   SUBTYPE OF               distinguishedName
///   LDAP-SYNTAX              dn.&id
///   LDAP-NAME                {"member"}
///   ID                       id-at-member }
/// ```
///
///
pub fn member() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(distinguishedName())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dn().id),                       /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("member")])), /* OBJECT_FIELD_SETTING */
        id: id_at_member(),                              /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod member {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uniqueMember ATTRIBUTE ::= {
///   WITH SYNTAX              NameAndOptionalUID
///   EQUALITY MATCHING RULE   uniqueMemberMatch
///   LDAP-SYNTAX              nameAndOptionalUID.&id
///   LDAP-NAME                {"uniqueMember"}
///   ID                       id-at-uniqueMember }
/// ```
///
///
pub fn uniqueMember() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(uniqueMemberMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(nameAndOptionalUID().id),           /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("uniqueMember")])), /* OBJECT_FIELD_SETTING */
        id: id_at_uniqueMember(),                            /* OBJECT_FIELD_SETTING */
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

pub mod uniqueMember {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NameAndOptionalUID; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_NameAndOptionalUID(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_NameAndOptionalUID(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_NameAndOptionalUID(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NameAndOptionalUID ::= SEQUENCE {
///   dn   DistinguishedName,
///   uid  UniqueIdentifier OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct NameAndOptionalUID {
    pub dn: DistinguishedName,
    pub uid: OPTIONAL<UniqueIdentifier>,
    pub _unrecognized: Vec<X690Element>,
}
impl NameAndOptionalUID {
    pub fn new(
        dn: DistinguishedName,
        uid: OPTIONAL<UniqueIdentifier>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        NameAndOptionalUID {
            dn,
            uid,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for NameAndOptionalUID {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_NameAndOptionalUID(el)
    }
}

pub const _rctl1_components_for_NameAndOptionalUID: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "dn",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "uid",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 3)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_NameAndOptionalUID: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_NameAndOptionalUID: &[ComponentSpec; 0] = &[];

pub fn _decode_NameAndOptionalUID(el: &X690Element) -> ASN1Result<NameAndOptionalUID> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NameAndOptionalUID")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_NameAndOptionalUID,
        _eal_components_for_NameAndOptionalUID,
        _rctl2_components_for_NameAndOptionalUID,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut dn_: OPTIONAL<DistinguishedName> = None;
    let mut uid_: OPTIONAL<UniqueIdentifier> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "dn" => dn_ = Some(_decode_DistinguishedName(_el)?),
            "uid" => uid_ = Some(_decode_UniqueIdentifier(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(NameAndOptionalUID {
        dn: dn_.unwrap(),
        uid: uid_,
        _unrecognized,
    })
}

pub fn _encode_NameAndOptionalUID(value_: &NameAndOptionalUID) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_DistinguishedName(&value_.dn)?);
    if let Some(v_) = &value_.uid {
        components_.push(_encode_UniqueIdentifier(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_NameAndOptionalUID(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "NameAndOptionalUID")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_NameAndOptionalUID,
        _eal_components_for_NameAndOptionalUID,
        _rctl2_components_for_NameAndOptionalUID,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "dn" => _validate_DistinguishedName(_el)?,
            "uid" => _validate_UniqueIdentifier(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// owner ATTRIBUTE ::= {
///   SUBTYPE OF               distinguishedName
///   LDAP-SYNTAX              dn.&id
///   LDAP-NAME                {"owner"}
///   ID                       id-at-owner }
/// ```
///
///
pub fn owner() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(distinguishedName())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dn().id),                       /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("owner")])), /* OBJECT_FIELD_SETTING */
        id: id_at_owner(),                               /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod owner {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// roleOccupant ATTRIBUTE ::= {
///   SUBTYPE OF               distinguishedName
///   LDAP-SYNTAX              dn.&id
///   LDAP-NAME                {"roleOccupant"}
///   ID                       id-at-roleOccupant }
/// ```
///
///
pub fn roleOccupant() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(distinguishedName())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dn().id),                       /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("roleOccupant")])), /* OBJECT_FIELD_SETTING */
        id: id_at_roleOccupant(),                        /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod roleOccupant {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// seeAlso ATTRIBUTE ::= {
///   SUBTYPE OF               distinguishedName
///   LDAP-SYNTAX              dn.&id
///   LDAP-NAME                {"seeAlso"}
///   ID                       id-at-seeAlso }
/// ```
///
///
pub fn seeAlso() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(distinguishedName())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dn().id),                       /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("seeAlso")])), /* OBJECT_FIELD_SETTING */
        id: id_at_seeAlso(),                             /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod seeAlso {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dmdName ATTRIBUTE ::= {
///   SUBTYPE OF               name
///   WITH SYNTAX              UnboundedDirectoryString
///   ID                       id-at-dmdName }
/// ```
///
///
pub fn dmdName() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(name())), /* OBJECT_FIELD_SETTING */
        id: id_at_dmdName(),                /* OBJECT_FIELD_SETTING */
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

pub mod dmdName {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidC1 ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER
///   EQUALITY MATCHING RULE   integerMatch
///   SINGLE VALUE             TRUE
///   ID                       id-oidC1 }
/// ```
///
///
pub fn oidC1() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        id: id_oidC1(),                                 /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod oidC1 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = INTEGER; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_integer(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_integer(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_integer(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidC2 ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER
///   EQUALITY MATCHING RULE   integerMatch
///   SINGLE VALUE             TRUE
///   ID                       id-oidC2 }
/// ```
///
///
pub fn oidC2() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        id: id_oidC2(),                                 /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod oidC2 {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = INTEGER; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_integer(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_integer(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_integer(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidC ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER
///   EQUALITY MATCHING RULE   integerMatch
///   SINGLE VALUE             TRUE
///   ID                       id-oidC }
/// ```
///
///
pub fn oidC() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        id: id_oidC(),                                  /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod oidC {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = INTEGER; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_integer(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_integer(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_integer(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// urnC ATTRIBUTE ::= {
///   WITH SYNTAX              PrintableString
///   EQUALITY MATCHING RULE   caseExactMatch
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              printableString.&id
///   LDAP-NAME                {"urnC"}
///   ID                       id-at-urnC }
/// ```
///
///
pub fn urnC() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseExactMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                        /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(printableString().id),           /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("urnC")])), /* OBJECT_FIELD_SETTING */
        id: id_at_urnC(),                                 /* OBJECT_FIELD_SETTING */
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

pub mod urnC {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PrintableString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_printable_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_printable_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_printable_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tagOid ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE   objectIdentifierMatch
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              oid.&id
///   LDAP-NAME                {"tagOid"}
///   ID                       id-at-tagOid }
/// ```
///
///
pub fn tagOid() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                               /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),                              /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("tagOid")])),     /* OBJECT_FIELD_SETTING */
        id: id_at_tagOid(),                                      /* OBJECT_FIELD_SETTING */
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

pub mod tagOid {
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
/// uiiFormat ATTRIBUTE ::= {
///   WITH SYNTAX              UiiFormat
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              uiiForm.&id
///   LDAP-NAME                {"uiiFormat"}
///   ID                       id-at-uiiFormat }
/// ```
///
///
pub fn uiiFormat() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),      /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(uiiForm().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("uiiFormat")])), /* OBJECT_FIELD_SETTING */
        id: id_at_uiiFormat(),          /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
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

pub mod uiiFormat {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UiiFormat; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UiiFormat(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UiiFormat(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UiiFormat(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UiiFormat ::= SEQUENCE {
///   baseObject  URI  OPTIONAL,
///   subset      ENUMERATED {
///     baseObject   (0),
///     oneLevel     (1),
///     wholeSubtree (2) } DEFAULT baseObject,
///   next        CHOICE {
///     length      INTEGER,
///     filter      UiiFilter } }
/// ```
///
#[derive(Debug, Clone)]
pub struct UiiFormat {
    pub baseObject: OPTIONAL<URI>,
    pub subset: OPTIONAL<UiiFormat_subset>,
    pub next: UiiFormat_next,
}
impl UiiFormat {
    pub fn new(
        baseObject: OPTIONAL<URI>,
        subset: OPTIONAL<UiiFormat_subset>,
        next: UiiFormat_next,
    ) -> Self {
        UiiFormat {
            baseObject,
            subset,
            next,
        }
    }
    pub fn _default_value_for_subset() -> UiiFormat_subset {
        UiiFormat_subset_baseObject
    }
}
impl TryFrom<&X690Element> for UiiFormat {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UiiFormat(el)
    }
}

pub const _rctl1_components_for_UiiFormat: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "baseObject",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 12)),
        None,
        None,
    ),
    ComponentSpec::new(
        "subset",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
    ComponentSpec::new("next", false, TagSelector::any, None, None),
];

pub const _rctl2_components_for_UiiFormat: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UiiFormat: &[ComponentSpec; 0] = &[];

pub fn _decode_UiiFormat(el: &X690Element) -> ASN1Result<UiiFormat> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UiiFormat")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UiiFormat,
        _eal_components_for_UiiFormat,
        _rctl2_components_for_UiiFormat,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut baseObject_: OPTIONAL<URI> = None;
    let mut subset_: OPTIONAL<UiiFormat_subset> = None;
    let mut next_: OPTIONAL<UiiFormat_next> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "baseObject" => baseObject_ = Some(_decode_URI(_el)?),
            "subset" => subset_ = Some(_decode_UiiFormat_subset(_el)?),
            "next" => next_ = Some(_decode_UiiFormat_next(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UiiFormat"))
            }
        }
    }
    Ok(UiiFormat {
        baseObject: baseObject_,
        subset: subset_,
        next: next_.unwrap(),
    })
}

pub fn _encode_UiiFormat(value_: &UiiFormat) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    if let Some(v_) = &value_.baseObject {
        components_.push(_encode_URI(&v_)?);
    }
    if let Some(v_) = &value_.subset {
        if *v_ != UiiFormat::_default_value_for_subset() {
            components_.push(_encode_UiiFormat_subset(&v_)?);
        }
    }
    components_.push(_encode_UiiFormat_next(&value_.next)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_UiiFormat(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UiiFormat")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UiiFormat,
        _eal_components_for_UiiFormat,
        _rctl2_components_for_UiiFormat,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "baseObject" => _validate_URI(_el)?,
            "subset" => _validate_UiiFormat_subset(_el)?,
            "next" => _validate_UiiFormat_next(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UiiFormat"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UiiFilter  ::=  CHOICE {
///   item  [0]  UiiItem,
///   and   [1]  SET OF UiiFilter,
///   or    [2]  SET OF UiiFilter,
///   not   [3]  UiiFilter }
/// ```
#[derive(Debug, Clone)]
pub enum UiiFilter {
    item(UiiItem),
    and(Vec<UiiFilter>),
    or(Vec<UiiFilter>),
    not(Box<UiiFilter>),
}

impl TryFrom<&X690Element> for UiiFilter {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UiiFilter(el)
    }
}

pub fn _decode_UiiFilter(el: &X690Element) -> ASN1Result<UiiFilter> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(UiiFilter::item(|el: &X690Element| -> ASN1Result<UiiItem> {
            Ok(_decode_UiiItem(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(UiiFilter::and(
            |el: &X690Element| -> ASN1Result<Vec<UiiFilter>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<UiiFilter>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and")
                            )
                        }
                    };
                    let mut items: SET_OF<UiiFilter> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_UiiFilter(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(UiiFilter::or(
            |el: &X690Element| -> ASN1Result<Vec<UiiFilter>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<UiiFilter>> {
                    let elements = match &el.value {
                        X690Value::Constructed(children) => children,
                        _ => {
                            return Err(
                                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "or")
                            )
                        }
                    };
                    let mut items: SET_OF<UiiFilter> = Vec::with_capacity(elements.len());
                    for el in elements.iter() {
                        items.push(_decode_UiiFilter(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 3) => Ok(UiiFilter::not(Box::new(
            |el: &X690Element| -> ASN1Result<UiiFilter> { Ok(_decode_UiiFilter(&el.inner()?)?) }(
                &el,
            )?,
        ))),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "UiiFilter",
            ))
        }
    }
}

pub fn _encode_UiiFilter(value_: &UiiFilter) -> ASN1Result<X690Element> {
    match value_ {
        UiiFilter::item(v) => |v_1: &UiiItem| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&_encode_UiiItem(&v_1)?),
            ))
        }(&v),
        UiiFilter::and(v) => |v_1: &Vec<UiiFilter>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(
                    &|value_: &SET_OF<UiiFilter>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_UiiFilter(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v),
        UiiFilter::or(v) => |v_1: &Vec<UiiFilter>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(
                    &|value_: &SET_OF<UiiFilter>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_UiiFilter(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v),
        UiiFilter::not(v) => |v_1: &UiiFilter| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(&_encode_UiiFilter(&v_1)?),
            ))
        }(&v),
    }
}

pub fn _validate_UiiFilter(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "item"));
            }
            Ok(_validate_UiiItem(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "and"));
            }
            Ok(|el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_UiiFilter(&sub)?;
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
                            _validate_UiiFilter(&sub)?;
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
            Ok(_validate_UiiFilter(&el.inner()?)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "UiiFilter",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UiiItem ::= SEQUENCE {
///   type   ATTRIBUTE.&id,
///   length INTEGER OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct UiiItem {
    pub type_: OBJECT_IDENTIFIER,
    pub length: OPTIONAL<INTEGER>,
}
impl UiiItem {
    pub fn new(type_: OBJECT_IDENTIFIER, length: OPTIONAL<INTEGER>) -> Self {
        UiiItem { type_, length }
    }
}
impl TryFrom<&X690Element> for UiiItem {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UiiItem(el)
    }
}

pub const _rctl1_components_for_UiiItem: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "length",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UiiItem: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UiiItem: &[ComponentSpec; 0] = &[];

pub fn _decode_UiiItem(el: &X690Element) -> ASN1Result<UiiItem> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UiiItem")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UiiItem,
        _eal_components_for_UiiItem,
        _rctl2_components_for_UiiItem,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut type__: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut length_: OPTIONAL<INTEGER> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "type" => type__ = Some(BER.decode_object_identifier(_el)?),
            "length" => length_ = Some(BER.decode_integer(_el)?),
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UiiItem")),
        }
    }
    Ok(UiiItem {
        type_: type__.unwrap(),
        length: length_,
    })
}

pub fn _encode_UiiItem(value_: &UiiItem) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    components_.push(BER.encode_object_identifier(&value_.type_)?);
    if let Some(v_) = &value_.length {
        components_.push(BER.encode_integer(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_UiiItem(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UiiItem")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UiiItem,
        _eal_components_for_UiiItem,
        _rctl2_components_for_UiiItem,
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
            "length" => BER.validate_integer(_el)?,
            _ => return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UiiItem")),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uiiInUrn ATTRIBUTE ::= {
///   WITH SYNTAX              UTF8String
///   EQUALITY MATCHING RULE   caseExactMatch
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"uiiInUrn"}
///   ID                       id-at-uiiInUrn }
/// ```
///
///
pub fn uiiInUrn() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseExactMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                        /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),           /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("uiiInUrn")])), /* OBJECT_FIELD_SETTING */
        id: id_at_uiiInUrn(),                             /* OBJECT_FIELD_SETTING */
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

pub mod uiiInUrn {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UTF8String; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_utf8_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_utf8_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_utf8_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// contentUrl ATTRIBUTE ::= {
///   SUBTYPE OF               url
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"contentUrl"}
///   ID                       id-at-contentUrl }
/// ```
///
///
pub fn contentUrl() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(url())),      /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("contentUrl")])), /* OBJECT_FIELD_SETTING */
        id: id_at_contentUrl(),                 /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod contentUrl {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uii ATTRIBUTE ::= {
///   WITH SYNTAX              BIT STRING
///   EQUALITY MATCHING RULE   bitStringMatch
///   LDAP-SYNTAX              bitString.&id
///   LDAP-NAME                {"uii"}
///   ID                       id-at-uii }
/// ```
///
///
pub fn uii() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(bitStringMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(bitString().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("uii")])), /* OBJECT_FIELD_SETTING */
        id: id_at_uii(),                                  /* OBJECT_FIELD_SETTING */
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

pub mod uii {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = BIT_STRING; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_bit_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_bit_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_bit_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// epc ATTRIBUTE ::= {
///   WITH SYNTAX              BIT STRING
///   EQUALITY MATCHING RULE   bitStringMatch
///   LDAP-SYNTAX              bitString.&id
///   LDAP-NAME                {"epc"}
///   ID                       id-at-epc }
/// ```
///
///
pub fn epc() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(bitStringMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(bitString().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("epc")])), /* OBJECT_FIELD_SETTING */
        id: id_at_epc(),                                  /* OBJECT_FIELD_SETTING */
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

pub mod epc {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = BIT_STRING; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_bit_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_bit_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_bit_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tagAfi ATTRIBUTE ::= {
///   WITH SYNTAX              OCTET STRING
///   EQUALITY MATCHING RULE   octetStringMatch
///   LDAP-SYNTAX              octetString.&id
///   LDAP-NAME                {"tagAfi"}
///   ID                       id-at-tagAfi }
/// ```
///
///
pub fn tagAfi() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(octetStringMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(octetString().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("tagAfi")])), /* OBJECT_FIELD_SETTING */
        id: id_at_tagAfi(),                                 /* OBJECT_FIELD_SETTING */
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

pub mod tagAfi {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = OCTET_STRING; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_octet_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_octet_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_octet_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// epcFormat  ATTRIBUTE ::= {
///   WITH SYNTAX              EpcFormat
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              epcForm.&id
///   LDAP-NAME                {"epcFormat"}
///   ID                       id-at-epcFormat }
/// ```
///
///
pub fn epcFormat() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),      /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(epcForm().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("epcFormat")])), /* OBJECT_FIELD_SETTING */
        id: id_at_epcFormat(),          /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
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

pub mod epcFormat {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = EpcFormat; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_EpcFormat(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_EpcFormat(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_EpcFormat(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EpcFormat ::= SEQUENCE {
///   fields          SEQUENCE SIZE (1..MAX) OF SEQUENCE {
///     bits            INTEGER,
///     charField       CHOICE {
///       characters  [0] INTEGER,
///       maxValue    [1] INTEGER },
///     result          ENUMERATED {
///       numericPad     (0),
///       numeric        (1),
///       alpha7bits     (2) } DEFAULT numericPad },
///   digitShift  [0] INTEGER                        OPTIONAL,
///   checkCalc   [1] INTEGER                        OPTIONAL,
///   urnPrefix       UTF8String                     OPTIONAL }
/// ```
///
#[derive(Debug, Clone)]
pub struct EpcFormat {
    pub fields: Vec<EpcFormat_fields_Item>,
    pub digitShift: OPTIONAL<INTEGER>,
    pub checkCalc: OPTIONAL<INTEGER>,
    pub urnPrefix: OPTIONAL<UTF8String>,
}
impl EpcFormat {
    pub fn new(
        fields: Vec<EpcFormat_fields_Item>,
        digitShift: OPTIONAL<INTEGER>,
        checkCalc: OPTIONAL<INTEGER>,
        urnPrefix: OPTIONAL<UTF8String>,
    ) -> Self {
        EpcFormat {
            fields,
            digitShift,
            checkCalc,
            urnPrefix,
        }
    }
}
impl TryFrom<&X690Element> for EpcFormat {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_EpcFormat(el)
    }
}

pub const _rctl1_components_for_EpcFormat: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "fields",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "digitShift",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "checkCalc",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "urnPrefix",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 12)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EpcFormat: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EpcFormat: &[ComponentSpec; 0] = &[];

pub fn _decode_EpcFormat(el: &X690Element) -> ASN1Result<EpcFormat> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EpcFormat")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EpcFormat,
        _eal_components_for_EpcFormat,
        _rctl2_components_for_EpcFormat,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut fields_: OPTIONAL<Vec<EpcFormat_fields_Item>> = None;
    let mut digitShift_: OPTIONAL<INTEGER> = None;
    let mut checkCalc_: OPTIONAL<INTEGER> = None;
    let mut urnPrefix_: OPTIONAL<UTF8String> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "fields" => {
                fields_ = Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<EpcFormat_fields_Item>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "fields",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<EpcFormat_fields_Item> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_EpcFormat_fields_Item(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            "digitShift" => {
                digitShift_ = Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                    Ok(BER.decode_integer(&el.inner()?)?)
                }(_el)?)
            }
            "checkCalc" => {
                checkCalc_ = Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                    Ok(BER.decode_integer(&el.inner()?)?)
                }(_el)?)
            }
            "urnPrefix" => urnPrefix_ = Some(BER.decode_utf8_string(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EpcFormat"))
            }
        }
    }
    Ok(EpcFormat {
        fields: fields_.unwrap(),
        digitShift: digitShift_,
        checkCalc: checkCalc_,
        urnPrefix: urnPrefix_,
    })
}

pub fn _encode_EpcFormat(value_: &EpcFormat) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(9);
    components_.push(
        |value_: &SEQUENCE_OF<EpcFormat_fields_Item>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_EpcFormat_fields_Item(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.fields)?,
    );
    if let Some(v_) = &value_.digitShift {
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&BER.encode_integer(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.checkCalc {
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&BER.encode_integer(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.urnPrefix {
        components_.push(BER.encode_utf8_string(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_EpcFormat(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EpcFormat")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EpcFormat,
        _eal_components_for_EpcFormat,
        _rctl2_components_for_EpcFormat,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "fields" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_EpcFormat_fields_Item(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fields")),
                }
            }(_el)?,
            "digitShift" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "digitShift")
                    );
                }
                Ok(BER.validate_integer(&el.inner()?)?)
            }(_el)?,
            "checkCalc" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "checkCalc")
                    );
                }
                Ok(BER.validate_integer(&el.inner()?)?)
            }(_el)?,
            "urnPrefix" => BER.validate_utf8_string(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EpcFormat"))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// epcInUrn ATTRIBUTE ::= {
///   SUBTYPE OF               urn
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"epcInUrn"}
///   ID                       id-at-epcInUrn }
/// ```
///
///
pub fn epcInUrn() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(urn())),      /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),              /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("epcInUrn")])), /* OBJECT_FIELD_SETTING */
        id: id_at_epcInUrn(),                   /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod epcInUrn {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldapUrl ATTRIBUTE ::= {
///   SUBTYPE OF               url
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"ldapUrl"}
///   ID                       id-at-ldapUrl }
/// ```
///
///
pub fn ldapUrl() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(url())),      /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("ldapUrl")])), /* OBJECT_FIELD_SETTING */
        id: id_at_ldapUrl(),                    /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod ldapUrl {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tagLocation ATTRIBUTE ::= {
///   SUBTYPE OF               utmCoordinates
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              utmCoords.&id
///   LDAP-NAME                {"tagLocation"}
///   ID                       id-at-tagLocation }
/// ```
///
///
pub fn tagLocation() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(utmCoordinates())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                    /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(utmCoords().id),             /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("tagLocation")])), /* OBJECT_FIELD_SETTING */
        id: id_at_tagLocation(),                      /* OBJECT_FIELD_SETTING */
        equality_match: None,
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

pub mod tagLocation {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dSAProblem ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE   objectIdentifierMatch
///   ID                       id-not-dSAProblem }
/// ```
///
///
pub fn dSAProblem() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        id: id_not_dSAProblem(),                                 /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod dSAProblem {
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
/// searchServiceProblem ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE   objectIdentifierMatch
///   SINGLE VALUE             TRUE
///   ID                       id-not-searchServiceProblem }
/// ```
///
///
pub fn searchServiceProblem() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                               /* OBJECT_FIELD_SETTING */
        id: id_not_searchServiceProblem(),                       /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod searchServiceProblem {
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
/// serviceType ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE   objectIdentifierMatch
///   SINGLE VALUE             TRUE
///   ID                       id-not-serviceType }
/// ```
///
///
pub fn serviceType() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                               /* OBJECT_FIELD_SETTING */
        id: id_not_serviceType(),                                /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod serviceType {
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
/// attributeTypeList ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE   objectIdentifierMatch
///   ID                       id-not-attributeTypeList }
/// ```
///
///
pub fn attributeTypeList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        id: id_not_attributeTypeList(),                          /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod attributeTypeList {
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
/// matchingRuleList ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE   objectIdentifierMatch
///   ID                       id-not-matchingRuleList }
/// ```
///
///
pub fn matchingRuleList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        id: id_not_matchingRuleList(),                           /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod matchingRuleList {
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
/// filterItem ATTRIBUTE ::= {
///   WITH SYNTAX              FilterItem
///   ID                       id-not-filterItem }
/// ```
///
///
pub fn filterItem() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_not_filterItem(), /* OBJECT_FIELD_SETTING */
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

pub mod filterItem {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = FilterItem; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_FilterItem(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_FilterItem(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_FilterItem(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeCombinations ATTRIBUTE ::= {
///   WITH SYNTAX              AttributeCombination
///   ID                       id-not-attributeCombinations }
/// ```
///
///
pub fn attributeCombinations() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_not_attributeCombinations(), /* OBJECT_FIELD_SETTING */
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

pub mod attributeCombinations {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AttributeCombination; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AttributeCombination(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AttributeCombination(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeCombination(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// contextTypeList ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE   objectIdentifierMatch
///   ID                       id-not-contextTypeList }
/// ```
///
///
pub fn contextTypeList() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        id: id_not_contextTypeList(),                            /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod contextTypeList {
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
/// contextList ATTRIBUTE ::= {
///   WITH SYNTAX              ContextAssertion
///   ID                       id-not-contextList }
/// ```
///
///
pub fn contextList() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_not_contextList(), /* OBJECT_FIELD_SETTING */
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

pub mod contextList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ContextAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ContextAssertion(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ContextAssertion(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ContextAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// contextCombinations ATTRIBUTE ::= {
///   WITH SYNTAX              ContextCombination
///   ID                       id-not-contextCombinations }
/// ```
///
///
pub fn contextCombinations() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_not_contextCombinations(), /* OBJECT_FIELD_SETTING */
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

pub mod contextCombinations {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ContextCombination; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ContextCombination(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ContextCombination(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ContextCombination(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hierarchySelectList ATTRIBUTE ::= {
///   WITH SYNTAX              HierarchySelections
///   SINGLE VALUE             TRUE
///   ID                       id-not-hierarchySelectList }
/// ```
///
///
pub fn hierarchySelectList() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),        /* OBJECT_FIELD_SETTING */
        id: id_not_hierarchySelectList(), /* OBJECT_FIELD_SETTING */
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

pub mod hierarchySelectList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = HierarchySelections; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_HierarchySelections(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_HierarchySelections(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_HierarchySelections(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// searchControlOptionsList ATTRIBUTE ::= {
///   WITH SYNTAX              SearchControlOptions
///   SINGLE VALUE             TRUE
///   ID                       id-not-searchControlOptionsList }
/// ```
///
///
pub fn searchControlOptionsList() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),             /* OBJECT_FIELD_SETTING */
        id: id_not_searchControlOptionsList(), /* OBJECT_FIELD_SETTING */
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

pub mod searchControlOptionsList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SearchControlOptions; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SearchControlOptions(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SearchControlOptions(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SearchControlOptions(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// serviceControlOptionsList ATTRIBUTE ::= {
///   WITH SYNTAX              ServiceControlOptions
///   SINGLE VALUE             TRUE
///   ID                       id-not-serviceControlOptionsList }
/// ```
///
///
pub fn serviceControlOptionsList() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),              /* OBJECT_FIELD_SETTING */
        id: id_not_serviceControlOptionsList(), /* OBJECT_FIELD_SETTING */
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

pub mod serviceControlOptionsList {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ServiceControlOptions; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ServiceControlOptions(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ServiceControlOptions(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ServiceControlOptions(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// multipleMatchingLocalities ATTRIBUTE ::= {
///   WITH SYNTAX              MultipleMatchingLocalities
///   ID                       id-not-multipleMatchingLocalities }
/// ```
///
///
pub fn multipleMatchingLocalities() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_not_multipleMatchingLocalities(), /* OBJECT_FIELD_SETTING */
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

pub mod multipleMatchingLocalities {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MultipleMatchingLocalities; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MultipleMatchingLocalities(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MultipleMatchingLocalities(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MultipleMatchingLocalities(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MultipleMatchingLocalities ::= SEQUENCE {
///   matchingRuleUsed  MATCHING-RULE.&id OPTIONAL,
///   attributeList     SEQUENCE OF AttributeValueAssertion,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct MultipleMatchingLocalities {
    pub matchingRuleUsed: OPTIONAL<OBJECT_IDENTIFIER>,
    pub attributeList: Vec<AttributeValueAssertion>,
    pub _unrecognized: Vec<X690Element>,
}
impl MultipleMatchingLocalities {
    pub fn new(
        matchingRuleUsed: OPTIONAL<OBJECT_IDENTIFIER>,
        attributeList: Vec<AttributeValueAssertion>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        MultipleMatchingLocalities {
            matchingRuleUsed,
            attributeList,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for MultipleMatchingLocalities {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_MultipleMatchingLocalities(el)
    }
}

pub const _rctl1_components_for_MultipleMatchingLocalities: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "matchingRuleUsed",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
        None,
        None,
    ),
    ComponentSpec::new(
        "attributeList",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_MultipleMatchingLocalities: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_MultipleMatchingLocalities: &[ComponentSpec; 0] = &[];

pub fn _decode_MultipleMatchingLocalities(
    el: &X690Element,
) -> ASN1Result<MultipleMatchingLocalities> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "MultipleMatchingLocalities",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MultipleMatchingLocalities,
        _eal_components_for_MultipleMatchingLocalities,
        _rctl2_components_for_MultipleMatchingLocalities,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut matchingRuleUsed_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut attributeList_: OPTIONAL<Vec<AttributeValueAssertion>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "matchingRuleUsed" => matchingRuleUsed_ = Some(BER.decode_object_identifier(_el)?),
            "attributeList" => {
                attributeList_ = Some(
                    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AttributeValueAssertion>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "attributeList",
                                ))
                            }
                        };
                        let mut items: SEQUENCE_OF<AttributeValueAssertion> =
                            Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_AttributeValueAssertion(el)?);
                        }
                        Ok(items)
                    }(_el)?,
                )
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(MultipleMatchingLocalities {
        matchingRuleUsed: matchingRuleUsed_,
        attributeList: attributeList_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_MultipleMatchingLocalities(
    value_: &MultipleMatchingLocalities,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.matchingRuleUsed {
        components_.push(BER.encode_object_identifier(&v_)?);
    }
    components_.push(
        |value_: &SEQUENCE_OF<AttributeValueAssertion>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(_encode_AttributeValueAssertion(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&value_.attributeList)?,
    );
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_MultipleMatchingLocalities(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "MultipleMatchingLocalities",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_MultipleMatchingLocalities,
        _eal_components_for_MultipleMatchingLocalities,
        _rctl2_components_for_MultipleMatchingLocalities,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "matchingRuleUsed" => BER.validate_object_identifier(_el)?,
            "attributeList" => |el: &X690Element| -> ASN1Result<()> {
                match &el.value {
                    X690Value::Constructed(subs) => {
                        for sub in subs.iter() {
                            _validate_AttributeValueAssertion(&sub)?;
                        }
                        Ok(())
                    }
                    _ => Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "attributeList")
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
/// proposedRelaxation ATTRIBUTE ::= {
///   WITH SYNTAX              MRMappings
///   ID                       id-not-proposedRelaxation }
/// ```
///
///
pub fn proposedRelaxation() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_not_proposedRelaxation(), /* OBJECT_FIELD_SETTING */
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

pub mod proposedRelaxation {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MRMappings; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MRMappings(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MRMappings(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MRMappings(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// MRMappings  ::=  SEQUENCE OF MRMapping
/// ```
pub type MRMappings = Vec<MRMapping>; // SequenceOfType

pub fn _decode_MRMappings(el: &X690Element) -> ASN1Result<MRMappings> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MRMappings")),
    };
    let mut items: SEQUENCE_OF<MRMapping> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_MRMapping(el)?);
    }
    Ok(items)
}

pub fn _encode_MRMappings(value_: &MRMappings) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_MRMapping(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_MRMappings(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_MRMapping(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "MRMappings")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// appliedRelaxation ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   EQUALITY MATCHING RULE   objectIdentifierMatch
///   ID                       id-not-appliedRelaxation }
/// ```
///
///
pub fn appliedRelaxation() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierMatch())), /* OBJECT_FIELD_SETTING */
        id: id_not_appliedRelaxation(),                          /* OBJECT_FIELD_SETTING */
        derivation: None,
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

pub mod appliedRelaxation {
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
/// pwdResponseValue ATTRIBUTE ::= {
///   WITH SYNTAX              PwdResponse
///   ID                       id-not-pwdResponse }
/// ```
///
///
pub fn pwdResponseValue() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_not_pwdResponse(), /* OBJECT_FIELD_SETTING */
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

pub mod pwdResponseValue {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PwdResponse; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PwdResponse(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PwdResponse(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PwdResponse(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PwdResponse ::= SEQUENCE {
///   warning CHOICE {
///     timeleft        [0] INTEGER(0..MAX),
///     graceRemaining  [1] INTEGER(0..MAX),
///     ... } OPTIONAL,
///   error ENUMERATED {
///     passwordExpired  (0),
///     changeAfterReset (1),
///     ... } OPTIONAL}
/// ```
///
#[derive(Debug, Clone)]
pub struct PwdResponse {
    pub warning: OPTIONAL<PwdResponse_warning>,
    pub error: OPTIONAL<PwdResponse_error>,
}
impl PwdResponse {
    pub fn new(warning: OPTIONAL<PwdResponse_warning>, error: OPTIONAL<PwdResponse_error>) -> Self {
        PwdResponse { warning, error }
    }
}
impl Default for PwdResponse {
    fn default() -> Self {
        PwdResponse {
            warning: None,
            error: None,
        }
    }
}
impl TryFrom<&X690Element> for PwdResponse {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PwdResponse(el)
    }
}

pub const _rctl1_components_for_PwdResponse: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "warning",
        true,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
        ]),
        None,
        None,
    ),
    ComponentSpec::new(
        "error",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_PwdResponse: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_PwdResponse: &[ComponentSpec; 0] = &[];

pub fn _decode_PwdResponse(el: &X690Element) -> ASN1Result<PwdResponse> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PwdResponse")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PwdResponse,
        _eal_components_for_PwdResponse,
        _rctl2_components_for_PwdResponse,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut warning_: OPTIONAL<PwdResponse_warning> = None;
    let mut error_: OPTIONAL<PwdResponse_error> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "warning" => warning_ = Some(_decode_PwdResponse_warning(_el)?),
            "error" => error_ = Some(_decode_PwdResponse_error(_el)?),
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PwdResponse")
                )
            }
        }
    }
    Ok(PwdResponse {
        warning: warning_,
        error: error_,
    })
}

pub fn _encode_PwdResponse(value_: &PwdResponse) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(7);
    if let Some(v_) = &value_.warning {
        components_.push(_encode_PwdResponse_warning(&v_)?);
    }
    if let Some(v_) = &value_.error {
        components_.push(_encode_PwdResponse_error(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_PwdResponse(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PwdResponse")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_PwdResponse,
        _eal_components_for_PwdResponse,
        _rctl2_components_for_PwdResponse,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "warning" => _validate_PwdResponse_warning(_el)?,
            "error" => _validate_PwdResponse_error(_el)?,
            _ => {
                return Err(
                    _el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PwdResponse")
                )
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldapDiagnosticMsg ATTRIBUTE ::= {
///   WITH SYNTAX              UTF8String
///   SINGLE VALUE             TRUE
///   ID                       id-not-ldapDiagnosticMsg }
/// ```
///
///
pub fn ldapDiagnosticMsg() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),      /* OBJECT_FIELD_SETTING */
        id: id_not_ldapDiagnosticMsg(), /* OBJECT_FIELD_SETTING */
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

pub mod ldapDiagnosticMsg {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UTF8String; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_utf8_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_utf8_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_utf8_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uid ATTRIBUTE ::= {
///   WITH SYNTAX              UnboundedDirectoryString
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"uid"}
///   ID                       id-coat-uid }
/// ```
///
///
pub fn uid() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),            /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("uid")])),  /* OBJECT_FIELD_SETTING */
        id: id_coat_uid(),                                 /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod uid {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dc ATTRIBUTE ::= {
///   WITH SYNTAX              IA5String
///   EQUALITY MATCHING RULE   caseIgnoreMatch
///   SUBSTRINGS MATCHING RULE caseIgnoreSubstringsMatch
///   LDAP-SYNTAX              ia5String.&id
///   LDAP-NAME                {"dc"}
///   ID                       id-coat-dc }
/// ```
///
///
pub fn dc() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(caseIgnoreMatch())), /* OBJECT_FIELD_SETTING */
        substrings_match: Some(Box::new(caseIgnoreSubstringsMatch())), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(ia5String().id),                  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("dc")])),   /* OBJECT_FIELD_SETTING */
        id: id_coat_dc(),                                  /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        single_valued: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        collective: Some(false),    /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),         /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_userApplications), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod dc {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = IA5String; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_ia5_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_ia5_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_ia5_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// caseExactMatch MATCHING-RULE ::= {
///   SYNTAX       UnboundedDirectoryString
///   LDAP-SYNTAX  directoryString.&id
///   LDAP-NAME    {"caseExactMatch"}
///   ID           id-mr-caseExactMatch }
/// ```
///
///
pub fn caseExactMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("caseExactMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_caseExactMatch(),             /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod caseExactMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// caseIgnoreMatch MATCHING-RULE ::= {
///   SYNTAX       UnboundedDirectoryString
///   LDAP-SYNTAX  directoryString.&id
///   LDAP-NAME    {"caseIgnoreMatch"}
///   ID           id-mr-caseIgnoreMatch }
/// ```
///
///
pub fn caseIgnoreMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("caseIgnoreMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_caseIgnoreMatch(),            /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod caseIgnoreMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// caseExactOrderingMatch MATCHING-RULE ::= {
///   SYNTAX       UnboundedDirectoryString
///   LDAP-SYNTAX  directoryString.&id
///   LDAP-NAME    {"caseExactOrderingMatch"}
///   ID           id-mr-caseExactOrderingMatch }
/// ```
///
///
pub fn caseExactOrderingMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("caseExactOrderingMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_caseExactOrderingMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod caseExactOrderingMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// caseIgnoreOrderingMatch MATCHING-RULE ::= {
///   SYNTAX       UnboundedDirectoryString
///   LDAP-SYNTAX  directoryString.&id
///   LDAP-NAME    {"caseIgnoreOrderingMatch"}
///   ID           id-mr-caseIgnoreOrderingMatch }
/// ```
///
///
pub fn caseIgnoreOrderingMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("caseIgnoreOrderingMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_caseIgnoreOrderingMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod caseIgnoreOrderingMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// caseExactSubstringsMatch MATCHING-RULE ::= {
///   SYNTAX       SubstringAssertion -- only the PrintableString choice
///   LDAP-SYNTAX  substringAssertion.&id
///   LDAP-NAME    {"caseExactSubstringsMatch"}
///   ID           id-mr-caseExactSubstringsMatch }
/// ```
///
///
pub fn caseExactSubstringsMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(substringAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("caseExactSubstringsMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_caseExactSubstringsMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod caseExactSubstringsMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = SubstringAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_SubstringAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_SubstringAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_SubstringAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// caseIgnoreSubstringsMatch MATCHING-RULE ::= {
///   SYNTAX       SubstringAssertion
///   LDAP-SYNTAX  substringAssertion.&id
///   LDAP-NAME    {"caseIgnoreSubstringsMatch"}
///   ID           id-mr-caseIgnoreSubstringsMatch }
/// ```
///
///
pub fn caseIgnoreSubstringsMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(substringAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("caseIgnoreSubstringsMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_caseIgnoreSubstringsMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod caseIgnoreSubstringsMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = SubstringAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_SubstringAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_SubstringAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_SubstringAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubstringAssertion  ::=  SEQUENCE OF CHOICE {
///   initial  [0]  UnboundedDirectoryString,
///   any      [1]  UnboundedDirectoryString,
///   final    [2]  UnboundedDirectoryString,
///     -- at most one initial and one final component
///   control       Attribute{{SupportedAttributes}},
///     -- Used to specify interpretation of the following items
///   ... }
/// ```
pub type SubstringAssertion = Vec<SubstringAssertion_Item>; // SequenceOfType

pub fn _decode_SubstringAssertion(el: &X690Element) -> ASN1Result<SubstringAssertion> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubstringAssertion")
            )
        }
    };
    let mut items: SEQUENCE_OF<SubstringAssertion_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_SubstringAssertion_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_SubstringAssertion(value_: &SubstringAssertion) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_SubstringAssertion_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_SubstringAssertion(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_SubstringAssertion_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "SubstringAssertion")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// numericStringMatch MATCHING-RULE ::= {
///   SYNTAX       NumericString
///   LDAP-SYNTAX  numericString.&id
///   LDAP-NAME    {"numericStringMatch"}
///   ID           id-mr-numericStringMatch }
/// ```
///
///
pub fn numericStringMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(numericString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("numericStringMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_numericStringMatch(),       /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod numericStringMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = NumericString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_numeric_string(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_numeric_string(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_numeric_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// numericStringOrderingMatch MATCHING-RULE ::= {
///   SYNTAX       NumericString
///   LDAP-SYNTAX  numericString.&id
///   LDAP-NAME    {"numericStringOrderingMatch"}
///   ID           id-mr-numericStringOrderingMatch }
/// ```
///
///
pub fn numericStringOrderingMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(numericString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("numericStringOrderingMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_numericStringOrderingMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod numericStringOrderingMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = NumericString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_numeric_string(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_numeric_string(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_numeric_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// numericStringSubstringsMatch MATCHING-RULE ::= {
///   SYNTAX       SubstringAssertion
///   LDAP-SYNTAX  substringAssertion.&id
///   LDAP-NAME    {"numericStringSubstringsMatch"}
///   ID           id-mr-numericStringSubstringsMatch }
/// ```
///
///
pub fn numericStringSubstringsMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(substringAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("numericStringSubstringsMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_numericStringSubstringsMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod numericStringSubstringsMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = SubstringAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_SubstringAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_SubstringAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_SubstringAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// caseIgnoreListMatch MATCHING-RULE ::= {
///   SYNTAX       CaseIgnoreList
///   LDAP-SYNTAX  postalAddr.&id
///   LDAP-NAME    {"caseIgnoreListMatch"}
///   ID           id-mr-caseIgnoreListMatch }
/// ```
///
///
pub fn caseIgnoreListMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(postalAddr().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("caseIgnoreListMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_caseIgnoreListMatch(),   /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod caseIgnoreListMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = CaseIgnoreList; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_CaseIgnoreList(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_CaseIgnoreList(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_CaseIgnoreList(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CaseIgnoreList  ::=  SEQUENCE OF UnboundedDirectoryString
/// ```
pub type CaseIgnoreList = Vec<UnboundedDirectoryString>; // SequenceOfType

pub fn _decode_CaseIgnoreList(el: &X690Element) -> ASN1Result<CaseIgnoreList> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CaseIgnoreList"))
        }
    };
    let mut items: SEQUENCE_OF<UnboundedDirectoryString> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_UnboundedDirectoryString(el)?);
    }
    Ok(items)
}

pub fn _encode_CaseIgnoreList(value_: &CaseIgnoreList) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_UnboundedDirectoryString(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_CaseIgnoreList(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_UnboundedDirectoryString(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "CaseIgnoreList")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// caseIgnoreListSubstringsMatch MATCHING-RULE ::= {
///   SYNTAX       SubstringAssertion
///   LDAP-SYNTAX  substringAssertion.&id
///   LDAP-NAME    {"caseIgnoreListSubstringsMatch"}
///   ID           id-mr-caseIgnoreListSubstringsMatch }
/// ```
///
///
pub fn caseIgnoreListSubstringsMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(substringAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("caseIgnoreListSubstringsMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_caseIgnoreListSubstringsMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod caseIgnoreListSubstringsMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = SubstringAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_SubstringAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_SubstringAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_SubstringAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// storedPrefixMatch MATCHING-RULE ::= {
///   SYNTAX       UnboundedDirectoryString
///   ID           id-mr-storedPrefixMatch }
/// ```
///
///
pub fn storedPrefixMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_storedPrefixMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod storedPrefixMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// booleanMatch MATCHING-RULE ::= {
///   SYNTAX       BOOLEAN
///   LDAP-SYNTAX  bitString.&id
///   LDAP-NAME    {"booleanMatch"}
///   ID           id-mr-booleanMatch }
/// ```
///
///
pub fn booleanMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(bitString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("booleanMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_booleanMatch(),         /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod booleanMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = BOOLEAN; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_boolean(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_boolean(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_boolean(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// integerMatch MATCHING-RULE ::= {
///   SYNTAX       INTEGER
///   LDAP-SYNTAX  integer.&id
///   LDAP-NAME    {"integerMatch"}
///   ID           id-mr-integerMatch }
/// ```
///
///
pub fn integerMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(integer().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("integerMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_integerMatch(),       /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod integerMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = INTEGER; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_integer(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_integer(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_integer(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// integerOrderingMatch MATCHING-RULE ::= {
///   SYNTAX       INTEGER
///   LDAP-SYNTAX  integer.&id
///   LDAP-NAME    {"integerOrderingMatch"}
///   ID           id-mr-integerOrderingMatch }
/// ```
///
///
pub fn integerOrderingMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(integer().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("integerOrderingMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_integerOrderingMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod integerOrderingMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = INTEGER; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_integer(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_integer(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_integer(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// bitStringMatch MATCHING-RULE ::= {
///   SYNTAX       BIT STRING
///   LDAP-SYNTAX  bitString.&id
///   LDAP-NAME    {"bitStringMatch"}
///   ID           id-mr-bitStringMatch }
/// ```
///
///
pub fn bitStringMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(bitString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("bitStringMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_bitStringMatch(),       /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod bitStringMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = BIT_STRING; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_bit_string(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_bit_string(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_bit_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// octetStringMatch MATCHING-RULE ::= {
///   SYNTAX       OCTET STRING
///   LDAP-SYNTAX  octetString.&id
///   LDAP-NAME    {"octetStringMatch"}
///   ID           id-mr-octetStringMatch }
/// ```
///
///
pub fn octetStringMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(octetString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("octetStringMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_octetStringMatch(),       /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod octetStringMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = OCTET_STRING; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_octet_string(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_octet_string(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_octet_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// octetStringOrderingMatch MATCHING-RULE ::= {
///   SYNTAX       OCTET STRING
///   LDAP-SYNTAX  octetString.&id
///   LDAP-NAME    {"octetStringOrderingMatch"}
///   ID           id-mr-octetStringOrderingMatch }
/// ```
///
///
pub fn octetStringOrderingMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(octetString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("octetStringOrderingMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_octetStringOrderingMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod octetStringOrderingMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = OCTET_STRING; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_octet_string(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_octet_string(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_octet_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// octetStringSubstringsMatch MATCHING-RULE ::= {
///   SYNTAX  OctetSubstringAssertion
///   ID      id-mr-octetStringSubstringsMatch }
/// ```
///
///
pub fn octetStringSubstringsMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_octetStringSubstringsMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod octetStringSubstringsMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = OctetSubstringAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_OctetSubstringAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_OctetSubstringAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_OctetSubstringAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OctetSubstringAssertion  ::=  SEQUENCE OF CHOICE {
///   initial  [0]  OCTET STRING,
///   any      [1]  OCTET STRING,
///   final    [2]  OCTET STRING,
///   ... }
/// ```
pub type OctetSubstringAssertion = Vec<OctetSubstringAssertion_Item>; // SequenceOfType

pub fn _decode_OctetSubstringAssertion(el: &X690Element) -> ASN1Result<OctetSubstringAssertion> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "OctetSubstringAssertion",
            ))
        }
    };
    let mut items: SEQUENCE_OF<OctetSubstringAssertion_Item> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_OctetSubstringAssertion_Item(el)?);
    }
    Ok(items)
}

pub fn _encode_OctetSubstringAssertion(
    value_: &OctetSubstringAssertion,
) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_OctetSubstringAssertion_Item(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_OctetSubstringAssertion(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_OctetSubstringAssertion_Item(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(
            ASN1ErrorCode::invalid_construction,
            "OctetSubstringAssertion",
        )),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// telephoneNumberMatch MATCHING-RULE ::= {
///   SYNTAX       TelephoneNumber
///   LDAP-SYNTAX  telephoneNr.&id
///   LDAP-NAME    {"telephoneNumberMatch"}
///   ID           id-mr-telephoneNumberMatch }
/// ```
///
///
pub fn telephoneNumberMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(telephoneNr().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("telephoneNumberMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_telephoneNumberMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod telephoneNumberMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = TelephoneNumber; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_TelephoneNumber(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_TelephoneNumber(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_TelephoneNumber(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// telephoneNumberSubstringsMatch MATCHING-RULE ::= {
///   SYNTAX       SubstringAssertion
///   LDAP-SYNTAX  substringAssertion.&id
///   LDAP-NAME    {"telephoneNumberSubstringsMatch"}
///   ID           id-mr-telephoneNumberSubstringsMatch }
/// ```
///
///
pub fn telephoneNumberSubstringsMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(substringAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("telephoneNumberSubstringsMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_telephoneNumberSubstringsMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod telephoneNumberSubstringsMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = SubstringAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_SubstringAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_SubstringAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_SubstringAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// presentationAddressMatch MATCHING-RULE ::= {
///   SYNTAX       PresentationAddress
///   ID           id-mr-presentationAddressMatch }
/// ```
///
///
pub fn presentationAddressMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_presentationAddressMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod presentationAddressMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = PresentationAddress; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_PresentationAddress(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_PresentationAddress(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_PresentationAddress(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uniqueMemberMatch MATCHING-RULE ::= {
///   SYNTAX       NameAndOptionalUID
///   LDAP-SYNTAX  nameAndOptionalUID.&id
///   LDAP-NAME    {"uniqueMemberMatch"}
///   ID           id-mr-uniqueMemberMatch }
/// ```
///
///
pub fn uniqueMemberMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(nameAndOptionalUID().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("uniqueMemberMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_uniqueMemberMatch(),             /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod uniqueMemberMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = NameAndOptionalUID; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_NameAndOptionalUID(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_NameAndOptionalUID(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_NameAndOptionalUID(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// protocolInformationMatch MATCHING-RULE ::= {
///   SYNTAX       OCTET STRING
///   ID           id-mr-protocolInformationMatch }
/// ```
///
///
pub fn protocolInformationMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_protocolInformationMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod protocolInformationMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = OCTET_STRING; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_octet_string(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_octet_string(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_octet_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// facsimileNumberMatch MATCHING-RULE ::= {
///   SYNTAX       TelephoneNumber
///   ID           id-mr-facsimileNumberMatch }
/// ```
///
///
pub fn facsimileNumberMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_facsimileNumberMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod facsimileNumberMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = TelephoneNumber; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_TelephoneNumber(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_TelephoneNumber(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_TelephoneNumber(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// facsimileNumberSubstringsMatch MATCHING-RULE ::= {
///   SYNTAX       SubstringAssertion
///   ID           id-mr-facsimileNumberSubstringsMatch }
/// ```
///
///
pub fn facsimileNumberSubstringsMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_facsimileNumberSubstringsMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod facsimileNumberSubstringsMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = SubstringAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_SubstringAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_SubstringAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_SubstringAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uUIDPairMatch MATCHING-RULE ::= {
///   SYNTAX       UUIDPair
///   ID           id-mr-uuidpairmatch }
/// ```
///
///
pub fn uUIDPairMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_uuidpairmatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod uUIDPairMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UUIDPair; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_UUIDPair(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_UUIDPair(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_UUIDPair(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uTCTimeMatch MATCHING-RULE ::= {
///   SYNTAX       UTCTime
///   ID           id-mr-uTCTimeMatch }
/// ```
///
///
pub fn uTCTimeMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_uTCTimeMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod uTCTimeMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UTCTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_utc_time(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_utc_time(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_utc_time(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uTCTimeOrderingMatch MATCHING-RULE ::= {
///   SYNTAX       UTCTime
///   ID           id-mr-uTCTimeOrderingMatch }
/// ```
///
///
pub fn uTCTimeOrderingMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_uTCTimeOrderingMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod uTCTimeOrderingMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UTCTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_utc_time(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_utc_time(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_utc_time(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// generalizedTimeMatch MATCHING-RULE ::= {
///   SYNTAX       GeneralizedTime
///   -- as per 46.3 b) or c) of Rec. ITU-T X.680 | ISO/IEC 8824-1
///   LDAP-SYNTAX  generalizedTime.&id
///   LDAP-NAME    {"generalizedTimeMatch"}
///   ID           id-mr-generalizedTimeMatch }
/// ```
///
///
pub fn generalizedTimeMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(generalizedTime().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("generalizedTimeMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_generalizedTimeMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod generalizedTimeMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = GeneralizedTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_generalized_time(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_generalized_time(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_generalized_time(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// generalizedTimeOrderingMatch MATCHING-RULE ::= {
///   SYNTAX       GeneralizedTime
///   -- as per 46.3 b) or c) of Rec. ITU-T X.680 | ISO/IEC 8824-1
///   LDAP-SYNTAX  generalizedTime.&id
///   LDAP-NAME    {"generalizedTimeOrderingMatch"}
///   ID           id-mr-generalizedTimeOrderingMatch }
/// ```
///
///
pub fn generalizedTimeOrderingMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(generalizedTime().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("generalizedTimeOrderingMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_generalizedTimeOrderingMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod generalizedTimeOrderingMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = GeneralizedTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_generalized_time(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_generalized_time(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_generalized_time(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// systemProposedMatch MATCHING-RULE ::= {
///   ID  id-mr-systemProposedMatch }
/// ```
///
///
pub fn systemProposedMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_systemProposedMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod systemProposedMatch {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// integerFirstComponentMatch MATCHING-RULE ::= {
///   SYNTAX       INTEGER
///   LDAP-SYNTAX  integer.&id
///   LDAP-NAME    {"integerFirstComponentMatch"}
///   ID           id-mr-integerFirstComponentMatch }
/// ```
///
///
pub fn integerFirstComponentMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(integer().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("integerFirstComponentMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_integerFirstComponentMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod integerFirstComponentMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = INTEGER; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_integer(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_integer(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_integer(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// objectIdentifierFirstComponentMatch MATCHING-RULE ::= {
///   SYNTAX       OBJECT IDENTIFIER
///   LDAP-SYNTAX  oid.&id
///   LDAP-NAME    {"objectIdentifierFirstComponentMatch"}
///   ID           id-mr-objectIdentifierFirstComponentMatch }
/// ```
///
///
pub fn objectIdentifierFirstComponentMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(oid().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from(
            "objectIdentifierFirstComponentMatch",
        )])), /* OBJECT_FIELD_SETTING */
        id: id_mr_objectIdentifierFirstComponentMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod objectIdentifierFirstComponentMatch {
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
/// directoryStringFirstComponentMatch MATCHING-RULE ::= {
///   SYNTAX       UnboundedDirectoryString
///   LDAP-SYNTAX  directoryString.&id
///   LDAP-NAME    {"directoryStringFirstComponentMatch"}
///   ID           id-mr-directoryStringFirstComponentMatch }
/// ```
///
///
pub fn directoryStringFirstComponentMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from(
            "directoryStringFirstComponentMatch",
        )])), /* OBJECT_FIELD_SETTING */
        id: id_mr_directoryStringFirstComponentMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod directoryStringFirstComponentMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// wordMatch MATCHING-RULE ::= {
///   SYNTAX       UnboundedDirectoryString
///   LDAP-SYNTAX  directoryString.&id
///   LDAP-NAME    {"wordMatch"}
///   ID           id-mr-wordMatch }
/// ```
///
///
pub fn wordMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("wordMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_wordMatch(),                  /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod wordMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// keywordMatch MATCHING-RULE ::= {
///   SYNTAX       UnboundedDirectoryString
///   LDAP-SYNTAX  directoryString.&id
///   LDAP-NAME    {"keywordMatch"}
///   ID           id-mr-keywordMatch }
/// ```
///
///
pub fn keywordMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("keywordMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_keywordMatch(),               /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod keywordMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// generalWordMatch MATCHING-RULE ::= {
///   SYNTAX       SubstringAssertion
///   ID           id-mr-generalWordMatch }
/// ```
///
///
pub fn generalWordMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_generalWordMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod generalWordMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = SubstringAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_SubstringAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_SubstringAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_SubstringAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sequenceMatchType ATTRIBUTE ::= {
///   WITH SYNTAX   SequenceMatchType
///   SINGLE VALUE  TRUE
///   ID            id-cat-sequenceMatchType }
/// ```
///
///
pub fn sequenceMatchType() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),      /* OBJECT_FIELD_SETTING */
        id: id_cat_sequenceMatchType(), /* OBJECT_FIELD_SETTING */
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

pub mod sequenceMatchType {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SequenceMatchType; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SequenceMatchType(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SequenceMatchType(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SequenceMatchType(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SequenceMatchType  ::=  ENUMERATED {
///   sequenceExact                  (0),
///   sequenceDeletion               (1),
///   sequenceRestrictedDeletion     (2),
///   sequencePermutation            (3),
///   sequencePermutationAndDeletion (4),
///   sequenceProviderDefined        (5),
///   ... }
/// ```
pub type SequenceMatchType = ENUMERATED;

pub const SequenceMatchType_sequenceExact: SequenceMatchType = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SequenceMatchType_sequenceDeletion: SequenceMatchType = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SequenceMatchType_sequenceRestrictedDeletion: SequenceMatchType = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SequenceMatchType_sequencePermutation: SequenceMatchType = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SequenceMatchType_sequencePermutationAndDeletion: SequenceMatchType = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const SequenceMatchType_sequenceProviderDefined: SequenceMatchType = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_SequenceMatchType(el: &X690Element) -> ASN1Result<SequenceMatchType> {
    BER.decode_enumerated(&el)
}

pub fn _encode_SequenceMatchType(value_: &SequenceMatchType) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_SequenceMatchType(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// wordMatchTypes ATTRIBUTE ::= {
///   WITH SYNTAX   WordMatchTypes
///   SINGLE VALUE  TRUE
///   ID            id-cat-wordMatchType }
/// ```
///
///
pub fn wordMatchTypes() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),  /* OBJECT_FIELD_SETTING */
        id: id_cat_wordMatchType(), /* OBJECT_FIELD_SETTING */
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

pub mod wordMatchTypes {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = WordMatchTypes; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_WordMatchTypes(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_WordMatchTypes(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_WordMatchTypes(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// WordMatchTypes  ::=  ENUMERATED {
///   wordExact           (0),
///   wordTruncated       (1),
///   wordPhonetic        (2),
///   wordProviderDefined (3),
///   ... }
/// ```
pub type WordMatchTypes = ENUMERATED;

pub const WordMatchTypes_wordExact: WordMatchTypes = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WordMatchTypes_wordTruncated: WordMatchTypes = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WordMatchTypes_wordPhonetic: WordMatchTypes = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const WordMatchTypes_wordProviderDefined: WordMatchTypes = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_WordMatchTypes(el: &X690Element) -> ASN1Result<WordMatchTypes> {
    BER.decode_enumerated(&el)
}

pub fn _encode_WordMatchTypes(value_: &WordMatchTypes) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_WordMatchTypes(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// characterMatchTypes ATTRIBUTE ::= {
///   WITH SYNTAX   CharacterMatchTypes
///   SINGLE VALUE  TRUE
///   ID            id-cat-characterMatchTypes }
/// ```
///
///
pub fn characterMatchTypes() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),        /* OBJECT_FIELD_SETTING */
        id: id_cat_characterMatchTypes(), /* OBJECT_FIELD_SETTING */
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

pub mod characterMatchTypes {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CharacterMatchTypes; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CharacterMatchTypes(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CharacterMatchTypes(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CharacterMatchTypes(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// CharacterMatchTypes  ::=  ENUMERATED {
///   characterExact      (0),
///   characterCaseIgnore (1),
///   characterMapped     (2),
///   ... }
/// ```
pub type CharacterMatchTypes = ENUMERATED;

pub const CharacterMatchTypes_characterExact: CharacterMatchTypes = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CharacterMatchTypes_characterCaseIgnore: CharacterMatchTypes = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const CharacterMatchTypes_characterMapped: CharacterMatchTypes = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_CharacterMatchTypes(el: &X690Element) -> ASN1Result<CharacterMatchTypes> {
    BER.decode_enumerated(&el)
}

pub fn _encode_CharacterMatchTypes(value_: &CharacterMatchTypes) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_CharacterMatchTypes(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// selectedContexts ATTRIBUTE ::= {
///   WITH SYNTAX  ContextAssertion
///   ID           id-cat-selectedContexts }
/// ```
///
///
pub fn selectedContexts() -> ATTRIBUTE {
    ATTRIBUTE {
        id: id_cat_selectedContexts(), /* OBJECT_FIELD_SETTING */
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

pub mod selectedContexts {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ContextAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ContextAssertion(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ContextAssertion(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ContextAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// approximateStringMatch MATCHING-RULE ::= {
///   ID      id-mr-approximateStringMatch }
/// ```
///
///
pub fn approximateStringMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_approximateStringMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod approximateStringMatch {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ignoreIfAbsentMatch MATCHING-RULE ::= {
///   ID      id-mr-ignoreIfAbsentMatch }
/// ```
///
///
pub fn ignoreIfAbsentMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_ignoreIfAbsentMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod ignoreIfAbsentMatch {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nullMatch MATCHING-RULE ::= {
///   ID      id-mr-nullMatch }
/// ```
///
///
pub fn nullMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        id: id_mr_nullMatch(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod nullMatch {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ZONAL-MATCHING ::= MAPPING-BASED-MATCHING{ZonalSelect, TRUE, ZonalResult, zonalMatch.&id}
/// ```
///
///
pub type ZONAL_MATCHING = MAPPING_BASED_MATCHING<ZonalSelect, ZonalResult>;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ZonalSelect  ::=  SEQUENCE OF AttributeType
/// ```
pub type ZonalSelect = Vec<AttributeType>; // SequenceOfType

pub fn _decode_ZonalSelect(el: &X690Element) -> ASN1Result<ZonalSelect> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ZonalSelect")),
    };
    let mut items: SEQUENCE_OF<AttributeType> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(_decode_AttributeType(el)?);
    }
    Ok(items)
}

pub fn _encode_ZonalSelect(value_: &ZonalSelect) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(_encode_AttributeType(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_ZonalSelect(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                _validate_AttributeType(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "ZonalSelect")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ZonalResult  ::=  ENUMERATED {
///   cannot-select-mapping (0),
///   zero-mappings         (2),
///   multiple-mappings     (3),
///    ... }
/// ```
pub type ZonalResult = ENUMERATED;

pub const ZonalResult_cannot_select_mapping: ZonalResult = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ZonalResult_zero_mappings: ZonalResult = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const ZonalResult_multiple_mappings: ZonalResult = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_ZonalResult(el: &X690Element) -> ASN1Result<ZonalResult> {
    BER.decode_enumerated(&el)
}

pub fn _encode_ZonalResult(value_: &ZonalResult) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_ZonalResult(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// zonalMatch MATCHING-RULE ::= {
///   UNIQUE-MATCH-INDICATOR  multipleMatchingLocalities
///   ID                      id-mr-zonalMatch }
/// ```
///
///
pub fn zonalMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        uniqueMatchIndicator: Some(multipleMatchingLocalities()), /* OBJECT_FIELD_SETTING */
        id: id_mr_zonalMatch(),                                   /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        ldapSyntax: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod zonalMatch {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uriMatch MATCHING-RULE ::= {
///   SYNTAX       UTF8String
///   LDAP-SYNTAX  directoryString.&id
///   LDAP-NAME    {"uriMatch"}
///   ID           id-mr-uriMatch }
/// ```
///
///
pub fn uriMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("uriMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_uriMatch(),                   /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod uriMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UTF8String; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_utf8_string(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_utf8_string(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_utf8_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dnsNameMatch MATCHING-RULE ::= {
///   SYNTAX       DomainName
///   LDAP-SYNTAX  dnsString.&id
///   LDAP-NAME    {"dnsNameMatch"}
///   ID           id-mr-dnsNameMatch }
/// ```
///
///
pub fn dnsNameMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(dnsString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("dnsNameMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_dnsNameMatch(),         /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod dnsNameMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = DomainName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_DomainName(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_DomainName(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_DomainName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// intEmailMatch MATCHING-RULE ::= {
///   SYNTAX       IntEmail
///   LDAP-SYNTAX  dnsString.&id
///   LDAP-NAME    {"intEmailMatch"}
///   ID           id-mr-intEmailMatch }
/// ```
///
///
pub fn intEmailMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(dnsString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("intEmailMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_intEmailMatch(),        /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod intEmailMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = IntEmail; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_IntEmail(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_IntEmail(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_IntEmail(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// jidMatch MATCHING-RULE ::= {
///   SYNTAX       Jid
///   LDAP-SYNTAX  dnsString.&id
///   LDAP-NAME    {"jidMatch"}
///   ID           id-mr-jidMatch }
/// ```
///
///
pub fn jidMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(dnsString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("jidMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_jidMatch(),             /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod jidMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = Jid; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_Jid(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_Jid(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_Jid(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// caseExactIA5Match MATCHING-RULE ::= {
///   SYNTAX       IA5String
///   LDAP-SYNTAX  ia5String.&id
///   LDAP-NAME    {"caseExactIA5Match"}
///   ID           id-lmr-caseExactIA5Match }
/// ```
///
///
pub fn caseExactIA5Match() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(ia5String().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("caseExactIA5Match")])), /* OBJECT_FIELD_SETTING */
        id: id_lmr_caseExactIA5Match(),   /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod caseExactIA5Match {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = IA5String; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_ia5_string(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_ia5_string(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_ia5_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// caseIgnoreIA5Match MATCHING-RULE ::= {
///   SYNTAX       IA5String
///   LDAP-SYNTAX  ia5String.&id
///   LDAP-NAME    {"caseIgnoreIA5Match"}
///   ID           id-lmr-caseIgnoreIA5Match }
/// ```
///
///
pub fn caseIgnoreIA5Match() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(ia5String().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("caseIgnoreIA5Match")])), /* OBJECT_FIELD_SETTING */
        id: id_lmr_caseIgnoreIA5Match(),  /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod caseIgnoreIA5Match {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = IA5String; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        BER.decode_ia5_string(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        BER.encode_ia5_string(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        BER.validate_ia5_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// caseIgnoreIA5SubstringsMatch MATCHING-RULE ::= {
///   SYNTAX       SubstringAssertion
///   LDAP-SYNTAX  substringAssertion.&id
///   LDAP-NAME    {"caseIgnoreIA5SubstringsMatch"}
///   ID           id-lmr-caseIgnoreIA5Match }
/// ```
///
///
pub fn caseIgnoreIA5SubstringsMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(substringAssertion().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("caseIgnoreIA5SubstringsMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_lmr_caseIgnoreIA5Match(), /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod caseIgnoreIA5SubstringsMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = SubstringAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_SubstringAssertion(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_SubstringAssertion(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_SubstringAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// utmCoords SYNTAX-NAME ::= {
///   LDAP-DESC         "UTM Coordinates"
///   DIRECTORY SYNTAX  UtmCoordinates
///   ID                id-asx-utmCoords }
/// ```
///
///
pub fn utmCoords() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("UTM Coordinates"), /* OBJECT_FIELD_SETTING */
        id: id_asx_utmCoords(),                    /* OBJECT_FIELD_SETTING */
    }
}

pub mod utmCoords {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UtmCoordinates; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UtmCoordinates(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UtmCoordinates(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UtmCoordinates(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// uiiForm SYNTAX-NAME ::= {
///   LDAP-DESC         "UII Format"
///   DIRECTORY SYNTAX  UiiFormat
///   ID                id-asx-uiiForm }
/// ```
///
///
pub fn uiiForm() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("UII Format"), /* OBJECT_FIELD_SETTING */
        id: id_asx_uiiForm(),                 /* OBJECT_FIELD_SETTING */
    }
}

pub mod uiiForm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UiiFormat; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UiiFormat(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UiiFormat(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UiiFormat(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// epcForm SYNTAX-NAME ::= {
///   LDAP-DESC         "EPC Format"
///   DIRECTORY SYNTAX  EpcFormat
///   ID                id-asx-epcForm }
/// ```
///
///
pub fn epcForm() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("EPC Format"), /* OBJECT_FIELD_SETTING */
        id: id_asx_epcForm(),                 /* OBJECT_FIELD_SETTING */
    }
}

pub mod epcForm {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = EpcFormat; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_EpcFormat(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_EpcFormat(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_EpcFormat(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// countryString3c SYNTAX-NAME ::= {
///   LDAP-DESC         "Country String 3 characters"
///   DIRECTORY SYNTAX  CountryCode3c
///   ID                id-asx-countryString3c }
/// ```
///
///
pub fn countryString3c() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Country String 3 characters"), /* OBJECT_FIELD_SETTING */
        id: id_asx_countryString3c(),                          /* OBJECT_FIELD_SETTING */
    }
}

pub mod countryString3c {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CountryCode3c; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CountryCode3c(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CountryCode3c(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CountryCode3c(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// countryString3n SYNTAX-NAME ::= {
///   LDAP-DESC         "Country String 3 numeric characters"
///   DIRECTORY SYNTAX  CountryCode3n
///   ID                id-asx-countryString3n }
/// ```
///
///
pub fn countryString3n() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Country String 3 numeric characters"), /* OBJECT_FIELD_SETTING */
        id: id_asx_countryString3n(),                                  /* OBJECT_FIELD_SETTING */
    }
}

pub mod countryString3n {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CountryCode3n; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CountryCode3n(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CountryCode3n(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CountryCode3n(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dnsString SYNTAX-NAME ::= {
///   LDAP-DESC         "DNS Name String"
///   DIRECTORY SYNTAX  DomainName
///   ID                id-asx-dnsString }
/// ```
///
///
pub fn dnsString() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("DNS Name String"), /* OBJECT_FIELD_SETTING */
        id: id_asx_dnsString(),                    /* OBJECT_FIELD_SETTING */
    }
}

pub mod dnsString {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DomainName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DomainName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DomainName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DomainName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// intEmailString SYNTAX-NAME ::= {
///   LDAP-DESC         "Internationalized Email"
///   DIRECTORY SYNTAX  IntEmail
///   ID                id-asx-intEmailString }
/// ```
///
///
pub fn intEmailString() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Internationalized Email"), /* OBJECT_FIELD_SETTING */
        id: id_asx_intEmailString(),                       /* OBJECT_FIELD_SETTING */
    }
}

pub mod intEmailString {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = IntEmail; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_IntEmail(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_IntEmail(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_IntEmail(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// jidString SYNTAX-NAME ::= {
///   LDAP-DESC         "Jabber identifier"
///   DIRECTORY SYNTAX  Jid
///   ID                id-asx-jidString }
/// ```
///
///
pub fn jidString() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Jabber identifier"), /* OBJECT_FIELD_SETTING */
        id: id_asx_jidString(),                      /* OBJECT_FIELD_SETTING */
    }
}

pub mod jidString {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Jid; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Jid(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Jid(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Jid(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeTypeDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "Attribute Type Description"
///   DIRECTORY SYNTAX  AttributeTypeDescription
///   ID                id-lsx-attributeTypeDescription }
/// ```
///
///
pub fn attributeTypeDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Attribute Type Description"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_attributeTypeDescription(),                /* OBJECT_FIELD_SETTING */
    }
}

pub mod attributeTypeDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AttributeTypeDescription; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AttributeTypeDescription(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AttributeTypeDescription(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeTypeDescription(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// bitString SYNTAX-NAME ::= {
///   LDAP-DESC         "Bit String"
///   DIRECTORY SYNTAX  BIT STRING
///   ID                id-lsx-bitString }
/// ```
///
///
pub fn bitString() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Bit String"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_bitString(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod bitString {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = BIT_STRING; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_bit_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_bit_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_bit_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// boolean SYNTAX-NAME ::= {
///   LDAP-DESC         "Boolean"
///   DIRECTORY SYNTAX  BOOLEAN
///   ID                id-lsx-boolean }
/// ```
///
///
pub fn boolean() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Boolean"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_boolean(),              /* OBJECT_FIELD_SETTING */
    }
}

pub mod boolean {
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
/// countryString SYNTAX-NAME ::= {
///   LDAP-DESC         "Country String"
///   DIRECTORY SYNTAX  CountryName
///   ID                id-lsx-countryString }
/// ```
///
///
pub fn countryString() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Country String"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_countryString(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod countryString {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = CountryName; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_CountryName(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_CountryName(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_CountryName(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dn SYNTAX-NAME ::= {
///   LDAP-DESC         "DN"
///   DIRECTORY SYNTAX  DistinguishedName
///   ID                id-lsx-dn }
/// ```
///
///
pub fn dn() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("DN"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_dn(),              /* OBJECT_FIELD_SETTING */
    }
}

pub mod dn {
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
/// deliveryMethod SYNTAX-NAME ::= {
///   LDAP-DESC         "Delevery Method"
///   DIRECTORY SYNTAX  PreferredDeliveryMethod
///   ID                id-lsx-deliveryMethod }
/// ```
///
///
pub fn deliveryMethod() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Delevery Method"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_deliveryMethod(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod deliveryMethod {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PreferredDeliveryMethod; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PreferredDeliveryMethod(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PreferredDeliveryMethod(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PreferredDeliveryMethod(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryString SYNTAX-NAME ::= {
///   LDAP-DESC         "Directory String"
///   DIRECTORY SYNTAX  UnboundedDirectoryString
///   ID                id-lsx-directoryString }
/// ```
///
///
pub fn directoryString() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Directory String"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_directoryString(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod directoryString {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UnboundedDirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UnboundedDirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UnboundedDirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UnboundedDirectoryString(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dITContentRuleDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "DIT Content Rule Description"
///   DIRECTORY SYNTAX  DITContentRuleDescription
///   ID                id-lsx-dITContentRuleDescription }
/// ```
///
///
pub fn dITContentRuleDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("DIT Content Rule Description"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_dITContentRuleDescription(),                 /* OBJECT_FIELD_SETTING */
    }
}

pub mod dITContentRuleDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DITContentRuleDescription; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DITContentRuleDescription(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DITContentRuleDescription(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DITContentRuleDescription(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dITStructureRuleDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "DIT StructureRule Description"
///   DIRECTORY SYNTAX  DITStructureRuleDescription
///   ID                id-lsx-dITStructureRuleDescription }
/// ```
///
///
pub fn dITStructureRuleDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("DIT StructureRule Description"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_dITStructureRuleDescription(),                /* OBJECT_FIELD_SETTING */
    }
}

pub mod dITStructureRuleDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DITStructureRuleDescription; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DITStructureRuleDescription(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DITStructureRuleDescription(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DITStructureRuleDescription(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// enhancedGuide SYNTAX-NAME ::= {
///   LDAP-DESC         "Enhanced Guide"
///   DIRECTORY SYNTAX  EnhancedGuide
///   ID                id-lsx-enhancedGuide }
/// ```
///
///
pub fn enhancedGuide() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Enhanced Guide"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_enhancedGuide(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod enhancedGuide {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = EnhancedGuide; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_EnhancedGuide(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_EnhancedGuide(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_EnhancedGuide(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// facsimileTelephoneNr SYNTAX-NAME ::= {
///   LDAP-DESC         "Facsimile Telephone Number"
///   DIRECTORY SYNTAX  FacsimileTelephoneNumber
///   ID                id-lsx-facsimileTelephoneNr }
/// ```
///
///
pub fn facsimileTelephoneNr() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Facsimile Telephone Number"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_facsimileTelephoneNr(),                    /* OBJECT_FIELD_SETTING */
    }
}

pub mod facsimileTelephoneNr {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = FacsimileTelephoneNumber; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_FacsimileTelephoneNumber(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_FacsimileTelephoneNumber(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_FacsimileTelephoneNumber(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// fax SYNTAX-NAME ::= {
///   LDAP-DESC         "Fax"
///   DIRECTORY SYNTAX  NULL
///   ID                id-lsx-fax }
/// ```
///
///
pub fn fax() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Fax"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_fax(),              /* OBJECT_FIELD_SETTING */
    }
}

pub mod fax {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// generalizedTime SYNTAX-NAME ::= {
///   LDAP-DESC         "Generalized Time"
///   DIRECTORY SYNTAX  GeneralizedTime
///   ID                id-lsx-generalizedTime }
/// ```
///
///
pub fn generalizedTime() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Generalized Time"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_generalizedTime(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod generalizedTime {
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
/// guide SYNTAX-NAME ::= {
///   LDAP-DESC         "Guide"
///   DIRECTORY SYNTAX  Guide
///   ID                id-lsx-guide }
/// ```
///
///
pub fn guide() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Guide"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_guide(),              /* OBJECT_FIELD_SETTING */
    }
}

pub mod guide {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = Guide; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_Guide(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_Guide(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_Guide(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ia5String SYNTAX-NAME ::= {
///   LDAP-DESC         "IA5 String"
///   DIRECTORY SYNTAX  IA5String
///   ID                id-lsx-ia5String }
/// ```
///
///
pub fn ia5String() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("IA5 String"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_ia5String(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod ia5String {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = IA5String; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_ia5_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_ia5_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_ia5_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// integer SYNTAX-NAME ::= {
///   LDAP-DESC         "INTEGER"
///   DIRECTORY SYNTAX  INTEGER
///   ID                id-lsx-integer }
/// ```
///
///
pub fn integer() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("INTEGER"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_integer(),              /* OBJECT_FIELD_SETTING */
    }
}

pub mod integer {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = INTEGER; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_integer(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_integer(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_integer(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// jpeg SYNTAX-NAME ::= {
///   LDAP-DESC         "JPEG"
///   DIRECTORY SYNTAX  NULL
///   ID                id-lsx-jpeg }
/// ```
///
///
pub fn jpeg() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("JPEG"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_jpeg(),              /* OBJECT_FIELD_SETTING */
    }
}

pub mod jpeg {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// matchingRuleDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "Matching Rule Description"
///   DIRECTORY SYNTAX  MatchingRuleDescription
///   ID                id-lsx-matchingRuleDescription }
/// ```
///
///
pub fn matchingRuleDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Matching Rule Description"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_matchingRuleDescription(),                /* OBJECT_FIELD_SETTING */
    }
}

pub mod matchingRuleDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MatchingRuleDescription; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MatchingRuleDescription(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MatchingRuleDescription(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MatchingRuleDescription(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// matchingRuleUseDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "Matching Rule Use Description"
///   DIRECTORY SYNTAX  MatchingRuleUseDescription
///   ID                id-lsx-matchingRuleUseDescription }
/// ```
///
///
pub fn matchingRuleUseDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Matching Rule Use Description"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_matchingRuleUseDescription(),                 /* OBJECT_FIELD_SETTING */
    }
}

pub mod matchingRuleUseDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = MatchingRuleUseDescription; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_MatchingRuleUseDescription(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_MatchingRuleUseDescription(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_MatchingRuleUseDescription(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nameAndOptionalUID SYNTAX-NAME ::= {
///   LDAP-DESC         "Name And Optional UID"
///   DIRECTORY SYNTAX  NameAndOptionalUID
///   ID                id-lsx-nameAndOptionalUID }
/// ```
///
///
pub fn nameAndOptionalUID() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Name And Optional UID"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_nameAndOptionalUID(),                 /* OBJECT_FIELD_SETTING */
    }
}

pub mod nameAndOptionalUID {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NameAndOptionalUID; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_NameAndOptionalUID(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_NameAndOptionalUID(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_NameAndOptionalUID(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nameFormDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "Name Form Description"
///   DIRECTORY SYNTAX  NameFormDescription
///   ID                id-lsx-nameFormDescription }
/// ```
///
///
pub fn nameFormDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Name Form Description"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_nameFormDescription(),                /* OBJECT_FIELD_SETTING */
    }
}

pub mod nameFormDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NameFormDescription; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_NameFormDescription(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_NameFormDescription(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_NameFormDescription(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// numericString SYNTAX-NAME ::= {
///   LDAP-DESC         "Numeric String"
///   DIRECTORY SYNTAX  NumericString
///   ID                id-lsx-numericString }
/// ```
///
///
pub fn numericString() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Numeric String"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_numericString(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod numericString {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NumericString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_numeric_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_numeric_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_numeric_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// objectClassDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "Object Class Description"
///   DIRECTORY SYNTAX  ObjectClassDescription
///   ID                id-lsx-objectClassDescription }
/// ```
///
///
pub fn objectClassDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Object Class Description"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_objectClassDescription(),                /* OBJECT_FIELD_SETTING */
    }
}

pub mod objectClassDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = ObjectClassDescription; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_ObjectClassDescription(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_ObjectClassDescription(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_ObjectClassDescription(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oid SYNTAX-NAME ::= {
///   LDAP-DESC         "OID"
///   DIRECTORY SYNTAX  OBJECT IDENTIFIER
///   ID                id-lsx-oid }
/// ```
///
///
pub fn oid() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("OID"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_oid(),              /* OBJECT_FIELD_SETTING */
    }
}

pub mod oid {
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
/// otherMailbox SYNTAX-NAME ::= {
///   LDAP-DESC        "Other Mailbox"
///   DIRECTORY SYNTAX  NULL
///   ID                id-lsx-otherMailbox }
/// ```
///
///
pub fn otherMailbox() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Other Mailbox"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_otherMailbox(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod otherMailbox {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// octetString SYNTAX-NAME ::= {
///   LDAP-DESC         "Octet String"
///   DIRECTORY SYNTAX  OCTET STRING
///   ID                id-lsx-octetString }
/// ```
///
///
pub fn octetString() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Octet String"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_octetString(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod octetString {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = OCTET_STRING; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_octet_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_octet_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_octet_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// postalAddr SYNTAX-NAME ::= {
///   LDAP-DESC         "Postal Address"
///   DIRECTORY SYNTAX  PostalAddress
///   ID                id-lsx-postalAddr }
/// ```
///
///
pub fn postalAddr() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Postal Address"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_postalAddr(),                  /* OBJECT_FIELD_SETTING */
    }
}

pub mod postalAddr {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PostalAddress; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PostalAddress(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PostalAddress(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PostalAddress(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// presentationAddr SYNTAX-NAME ::= {
///   LDAP-DESC         "Presentation Address"
///   DIRECTORY SYNTAX  PresentationAddress
///   ID                id-lsx-presentationAddr }
/// ```
///
///
pub fn presentationAddr() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Presentation Address"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_presentationAddr(),                  /* OBJECT_FIELD_SETTING */
    }
}

pub mod presentationAddr {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PresentationAddress; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PresentationAddress(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PresentationAddress(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PresentationAddress(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// printableString SYNTAX-NAME ::= {
///   LDAP-DESC         "Printable String"
///   DIRECTORY SYNTAX  PrintableString
///   ID                id-lsx-printableString }
/// ```
///
///
pub fn printableString() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Printable String"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_printableString(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod printableString {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PrintableString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_printable_string(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_printable_string(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_printable_string(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subtreeSpec SYNTAX-NAME ::= {
///   LDAP-DESC         "SubtreeSpecification"
///   DIRECTORY SYNTAX  SubtreeSpecification
///   ID                id-lsx-subtreeSpec }
/// ```
///
///
pub fn subtreeSpec() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("SubtreeSpecification"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_subtreeSpec(),                       /* OBJECT_FIELD_SETTING */
    }
}

pub mod subtreeSpec {
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
/// telephoneNr SYNTAX-NAME ::= {
///   LDAP-DESC         "Telephone Number"
///   DIRECTORY SYNTAX  TelephoneNumber
///   ID                id-lsx-telephoneNr }
/// ```
///
///
pub fn telephoneNr() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Telephone Number"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_telephoneNr(),                   /* OBJECT_FIELD_SETTING */
    }
}

pub mod telephoneNr {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = TelephoneNumber; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_TelephoneNumber(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_TelephoneNumber(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_TelephoneNumber(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// telexNr SYNTAX-NAME ::= {
///   LDAP-DESC         "Telex Number"
///   DIRECTORY SYNTAX  TelexNumber
///   ID                id-lsx-telexNr }
/// ```
///
///
pub fn telexNr() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Telex Number"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_telexNr(),                   /* OBJECT_FIELD_SETTING */
    }
}

pub mod telexNr {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = TelexNumber; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_TelexNumber(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_TelexNumber(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_TelexNumber(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// utcTime SYNTAX-NAME ::= {
///   LDAP-DESC         "UTC Time"
///   DIRECTORY SYNTAX  UTCTime
///   ID                id-lsx-utcTime }
/// ```
///
///
pub fn utcTime() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("UTC Time"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_utcTime(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod utcTime {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UTCTime; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_utc_time(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_utc_time(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_utc_time(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldapSyntaxDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "LDAP Syntax Description"
///   DIRECTORY SYNTAX  NULL
///   ID                id-lsx-ldapSyntaxDescription }
/// ```
///
///
pub fn ldapSyntaxDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("LDAP Syntax Description"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_ldapSyntaxDescription(),                /* OBJECT_FIELD_SETTING */
    }
}

pub mod ldapSyntaxDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = NULL; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        BER.decode_null(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        BER.encode_null(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        BER.validate_null(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// substringAssertion SYNTAX-NAME ::= {
///   LDAP-DESC         "Substring Assertion"
///   DIRECTORY SYNTAX  SubstringAssertion
///   ID                id-lsx-substringAssertion }
/// ```
///
///
pub fn substringAssertion() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Substring Assertion"), /* OBJECT_FIELD_SETTING */
        id: id_lsx_substringAssertion(),               /* OBJECT_FIELD_SETTING */
    }
}

pub mod substringAssertion {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = SubstringAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_SubstringAssertion(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_SubstringAssertion(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_SubstringAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// languageContext CONTEXT ::= {
///   WITH SYNTAX  LanguageContextSyntax
///   ID           id-avc-language }
/// ```
///
///
pub fn languageContext() -> CONTEXT {
    CONTEXT {
        id: id_avc_language(),   /* OBJECT_FIELD_SETTING */
        absentMatch: Some(true), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod languageContext {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = LanguageContextSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_LanguageContextSyntax(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_LanguageContextSyntax(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_LanguageContextSyntax(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LanguageContextSyntax  ::=  PrintableString(SIZE (2..3))
/// ```
pub type LanguageContextSyntax = PrintableString; // PrintableString

pub fn _decode_LanguageContextSyntax(el: &X690Element) -> ASN1Result<LanguageContextSyntax> {
    BER.decode_printable_string(&el)
}

pub fn _encode_LanguageContextSyntax(value_: &LanguageContextSyntax) -> ASN1Result<X690Element> {
    BER.encode_printable_string(&value_)
}

pub fn _validate_LanguageContextSyntax(el: &X690Element) -> ASN1Result<()> {
    BER.validate_printable_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// temporalContext CONTEXT ::= {
///   WITH SYNTAX  TimeSpecification
///   ASSERTED AS  TimeAssertion
///   ID           id-avc-temporal }
/// ```
///
///
pub fn temporalContext() -> CONTEXT {
    CONTEXT {
        id: id_avc_temporal(),   /* OBJECT_FIELD_SETTING */
        absentMatch: Some(true), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod temporalContext {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = TimeSpecification; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_TimeSpecification(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_TimeSpecification(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_TimeSpecification(el)
    }
    pub type Assertion = TimeAssertion; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Assertion(el: &X690Element) -> ASN1Result<Assertion> {
        _decode_TimeAssertion(el)
    }
    pub fn _encode_Assertion(value_: &Assertion) -> ASN1Result<X690Element> {
        _encode_TimeAssertion(value_)
    }
    pub fn _validate_Assertion(el: &X690Element) -> ASN1Result<()> {
        _validate_TimeAssertion(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeSpecification ::= SEQUENCE {
///   time           CHOICE {
///     absolute       SEQUENCE {
///       startTime [0]  GeneralizedTime OPTIONAL,
///       endTime   [1]  GeneralizedTime OPTIONAL,
///       ... },
///     periodic      SET SIZE (1..MAX) OF Period},
///   notThisTime   BOOLEAN DEFAULT FALSE,
///   timeZone      TimeZone OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct TimeSpecification {
    pub time: TimeSpecification_time,
    pub notThisTime: OPTIONAL<BOOLEAN>,
    pub timeZone: OPTIONAL<TimeZone>,
    pub _unrecognized: Vec<X690Element>,
}
impl TimeSpecification {
    pub fn new(
        time: TimeSpecification_time,
        notThisTime: OPTIONAL<BOOLEAN>,
        timeZone: OPTIONAL<TimeZone>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TimeSpecification {
            time,
            notThisTime,
            timeZone,
            _unrecognized,
        }
    }
    pub fn _default_value_for_notThisTime() -> BOOLEAN {
        false
    }
}
impl TryFrom<&X690Element> for TimeSpecification {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimeSpecification(el)
    }
}

pub const _rctl1_components_for_TimeSpecification: &[ComponentSpec; 3] = &[
    ComponentSpec::new("time", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "notThisTime",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "timeZone",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TimeSpecification: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TimeSpecification: &[ComponentSpec; 0] = &[];

pub fn _decode_TimeSpecification(el: &X690Element) -> ASN1Result<TimeSpecification> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeSpecification")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeSpecification,
        _eal_components_for_TimeSpecification,
        _rctl2_components_for_TimeSpecification,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut time_: OPTIONAL<TimeSpecification_time> = None;
    let mut notThisTime_: OPTIONAL<BOOLEAN> = None;
    let mut timeZone_: OPTIONAL<TimeZone> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "time" => time_ = Some(_decode_TimeSpecification_time(_el)?),
            "notThisTime" => notThisTime_ = Some(BER.decode_boolean(_el)?),
            "timeZone" => timeZone_ = Some(_decode_TimeZone(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TimeSpecification {
        time: time_.unwrap(),
        notThisTime: notThisTime_,
        timeZone: timeZone_,
        _unrecognized,
    })
}

pub fn _encode_TimeSpecification(value_: &TimeSpecification) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(_encode_TimeSpecification_time(&value_.time)?);
    if let Some(v_) = &value_.notThisTime {
        if *v_ != TimeSpecification::_default_value_for_notThisTime() {
            components_.push(BER.encode_boolean(&v_)?);
        }
    }
    if let Some(v_) = &value_.timeZone {
        components_.push(_encode_TimeZone(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_TimeSpecification(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeSpecification")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeSpecification,
        _eal_components_for_TimeSpecification,
        _rctl2_components_for_TimeSpecification,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "time" => _validate_TimeSpecification_time(_el)?,
            "notThisTime" => BER.validate_boolean(_el)?,
            "timeZone" => _validate_TimeZone(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Period ::= SEQUENCE {
///   timesOfDay  [0]  SET SIZE (1..MAX) OF DayTimeBand OPTIONAL,
///   days        [1]  CHOICE {
///     intDay           SET OF INTEGER,
///     bitDay           BIT STRING {
///       sunday    (0),
///       monday    (1),
///       tuesday   (2),
///       wednesday (3),
///       thursday  (4),
///       friday    (5),
///       saturday  (6)},
///     dayOf            XDayOf,
///     ...} OPTIONAL,
///   weeks       [2]  CHOICE {
///     allWeeks         NULL,
///     intWeek          SET OF INTEGER,
///     bitWeek          BIT STRING {
///       week1     (0),
///       week2     (1),
///       week3     (2),
///       week4     (3),
///       week5     (4)},
///     ... } OPTIONAL,
///   months      [3]  CHOICE {
///     allMonths        NULL,
///     intMonth         SET OF INTEGER,
///     bitMonth         BIT STRING {
///       january   (0),
///       february  (1),
///       march     (2),
///       april     (3),
///       may       (4),
///       june      (5),
///       july      (6),
///       august    (7),
///       september (8),
///       october   (9),
///       november  (10),
///       december  (11)},
///     ...} OPTIONAL,
///   years       [4]  SET OF INTEGER(1000..MAX) OPTIONAL,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct Period {
    pub timesOfDay: OPTIONAL<Vec<DayTimeBand>>,
    pub days: OPTIONAL<Period_days>,
    pub weeks: OPTIONAL<Period_weeks>,
    pub months: OPTIONAL<Period_months>,
    pub years: OPTIONAL<Vec<INTEGER>>,
    pub _unrecognized: Vec<X690Element>,
}
impl Period {
    pub fn new(
        timesOfDay: OPTIONAL<Vec<DayTimeBand>>,
        days: OPTIONAL<Period_days>,
        weeks: OPTIONAL<Period_weeks>,
        months: OPTIONAL<Period_months>,
        years: OPTIONAL<Vec<INTEGER>>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        Period {
            timesOfDay,
            days,
            weeks,
            months,
            years,
            _unrecognized,
        }
    }
}
impl Default for Period {
    fn default() -> Self {
        Period {
            timesOfDay: None,
            days: None,
            weeks: None,
            months: None,
            years: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<&X690Element> for Period {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Period(el)
    }
}

pub const _rctl1_components_for_Period: &[ComponentSpec; 5] = &[
    ComponentSpec::new(
        "timesOfDay",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "days",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "weeks",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
    ComponentSpec::new(
        "months",
        true,
        TagSelector::tag((TagClass::CONTEXT, 3)),
        None,
        None,
    ),
    ComponentSpec::new(
        "years",
        true,
        TagSelector::tag((TagClass::CONTEXT, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_Period: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_Period: &[ComponentSpec; 0] = &[];

pub fn _decode_Period(el: &X690Element) -> ASN1Result<Period> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Period")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Period,
        _eal_components_for_Period,
        _rctl2_components_for_Period,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut timesOfDay_: OPTIONAL<Vec<DayTimeBand>> = None;
    let mut days_: OPTIONAL<Period_days> = None;
    let mut weeks_: OPTIONAL<Period_weeks> = None;
    let mut months_: OPTIONAL<Period_months> = None;
    let mut years_: OPTIONAL<Vec<INTEGER>> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "timesOfDay" => {
                timesOfDay_ = Some(|el: &X690Element| -> ASN1Result<Vec<DayTimeBand>> {
                    Ok(|el: &X690Element| -> ASN1Result<SET_OF<DayTimeBand>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "timesOfDay",
                                ))
                            }
                        };
                        let mut items: SET_OF<DayTimeBand> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(_decode_DayTimeBand(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            "days" => {
                days_ = Some(|el: &X690Element| -> ASN1Result<Period_days> {
                    Ok(_decode_Period_days(&el.inner()?)?)
                }(_el)?)
            }
            "weeks" => {
                weeks_ = Some(|el: &X690Element| -> ASN1Result<Period_weeks> {
                    Ok(_decode_Period_weeks(&el.inner()?)?)
                }(_el)?)
            }
            "months" => {
                months_ = Some(|el: &X690Element| -> ASN1Result<Period_months> {
                    Ok(_decode_Period_months(&el.inner()?)?)
                }(_el)?)
            }
            "years" => {
                years_ = Some(|el: &X690Element| -> ASN1Result<Vec<INTEGER>> {
                    Ok(|el: &X690Element| -> ASN1Result<SET_OF<INTEGER>> {
                        let elements = match &el.value {
                            X690Value::Constructed(children) => children,
                            _ => {
                                return Err(el.to_asn1_err_named(
                                    ASN1ErrorCode::invalid_construction,
                                    "years",
                                ))
                            }
                        };
                        let mut items: SET_OF<INTEGER> = Vec::with_capacity(elements.len());
                        for el in elements.iter() {
                            items.push(BER.decode_integer(el)?);
                        }
                        Ok(items)
                    }(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(Period {
        timesOfDay: timesOfDay_,
        days: days_,
        weeks: weeks_,
        months: months_,
        years: years_,
        _unrecognized,
    })
}

pub fn _encode_Period(value_: &Period) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(15);
    if let Some(v_) = &value_.timesOfDay {
        components_.push(|v_1: &Vec<DayTimeBand>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(
                    &|value_: &SET_OF<DayTimeBand>| -> ASN1Result<X690Element> {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_DayTimeBand(&v)?);
                        }
                        Ok(X690Element::new(
                            Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                            X690Value::Constructed(Arc::new(children)),
                        ))
                    }(&v_1)?,
                ),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.days {
        components_.push(|v_1: &Period_days| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_Period_days(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.weeks {
        components_.push(|v_1: &Period_weeks| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(&_encode_Period_weeks(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.months {
        components_.push(|v_1: &Period_months| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(&_encode_Period_months(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.years {
        components_.push(|v_1: &Vec<INTEGER>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::from_explicit(&|value_: &SET_OF<INTEGER>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(BER.encode_integer(&v)?);
                    }
                    Ok(X690Element::new(
                        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                        X690Value::Constructed(Arc::new(children)),
                    ))
                }(&v_1)?),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_Period(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "Period")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_Period,
        _eal_components_for_Period,
        _rctl2_components_for_Period,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "timesOfDay" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "timesOfDay")
                    );
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                _validate_DayTimeBand(&sub)?;
                            }
                            Ok(())
                        }
                        _ => Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "timesOfDay")
                        ),
                    }
                }(&el.inner()?)?)
            }(_el)?,
            "days" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "days"));
                }
                Ok(_validate_Period_days(&el.inner()?)?)
            }(_el)?,
            "weeks" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "weeks"));
                }
                Ok(_validate_Period_weeks(&el.inner()?)?)
            }(_el)?,
            "months" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "months"));
                }
                Ok(_validate_Period_months(&el.inner()?)?)
            }(_el)?,
            "years" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "years"));
                }
                Ok(|el: &X690Element| -> ASN1Result<()> {
                    match &el.value {
                        X690Value::Constructed(subs) => {
                            for sub in subs.iter() {
                                BER.validate_integer(&sub)?;
                            }
                            Ok(())
                        }
                        _ => {
                            Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "years"))
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
/// XDayOf  ::=  CHOICE {
///   first   [1]  NamedDay,
///   second  [2]  NamedDay,
///   third   [3]  NamedDay,
///   fourth  [4]  NamedDay,
///   fifth   [5]  NamedDay }
/// ```
#[derive(Debug, Clone)]
pub enum XDayOf {
    first(NamedDay),
    second(NamedDay),
    third(NamedDay),
    fourth(NamedDay),
    fifth(NamedDay),
}

impl TryFrom<&X690Element> for XDayOf {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_XDayOf(el)
    }
}

pub fn _decode_XDayOf(el: &X690Element) -> ASN1Result<XDayOf> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 1) => Ok(XDayOf::first(|el: &X690Element| -> ASN1Result<NamedDay> {
            Ok(_decode_NamedDay(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(XDayOf::second(|el: &X690Element| -> ASN1Result<NamedDay> {
            Ok(_decode_NamedDay(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(XDayOf::third(|el: &X690Element| -> ASN1Result<NamedDay> {
            Ok(_decode_NamedDay(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 4) => Ok(XDayOf::fourth(|el: &X690Element| -> ASN1Result<NamedDay> {
            Ok(_decode_NamedDay(&el.inner()?)?)
        }(&el)?)),
        (TagClass::CONTEXT, 5) => Ok(XDayOf::fifth(|el: &X690Element| -> ASN1Result<NamedDay> {
            Ok(_decode_NamedDay(&el.inner()?)?)
        }(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "XDayOf",
            ))
        }
    }
}

pub fn _encode_XDayOf(value_: &XDayOf) -> ASN1Result<X690Element> {
    match value_ {
        XDayOf::first(v) => |v_1: &NamedDay| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&_encode_NamedDay(&v_1)?),
            ))
        }(&v),
        XDayOf::second(v) => |v_1: &NamedDay| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(&_encode_NamedDay(&v_1)?),
            ))
        }(&v),
        XDayOf::third(v) => |v_1: &NamedDay| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::from_explicit(&_encode_NamedDay(&v_1)?),
            ))
        }(&v),
        XDayOf::fourth(v) => |v_1: &NamedDay| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::from_explicit(&_encode_NamedDay(&v_1)?),
            ))
        }(&v),
        XDayOf::fifth(v) => |v_1: &NamedDay| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 5),
                X690Value::from_explicit(&_encode_NamedDay(&v_1)?),
            ))
        }(&v),
    }
}

pub fn _validate_XDayOf(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "first"));
            }
            Ok(_validate_NamedDay(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "second"));
            }
            Ok(_validate_NamedDay(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 3) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 3 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "third"));
            }
            Ok(_validate_NamedDay(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 4) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 4 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fourth"));
            }
            Ok(_validate_NamedDay(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 5) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 5 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "fifth"));
            }
            Ok(_validate_NamedDay(&el.inner()?)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "XDayOf",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedDay  ::=  CHOICE {
///   intNamedDays ENUMERATED {
///     sunday      (1),
///     monday      (2),
///     tuesday     (3),
///     wednesday   (4),
///     thursday    (5),
///     friday      (6),
///     saturday    (7)},
///   bitNamedDays BIT STRING {
///     sunday      (0),
///     monday      (1),
///     tuesday     (2),
///     wednesday   (3),
///     thursday    (4),
///     friday      (5),
///     saturday    (6)} }
/// ```
#[derive(Debug, Clone)]
pub enum NamedDay {
    intNamedDays(NamedDay_intNamedDays),
    bitNamedDays(NamedDay_bitNamedDays),
}

impl TryFrom<&X690Element> for NamedDay {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_NamedDay(el)
    }
}

pub fn _decode_NamedDay(el: &X690Element) -> ASN1Result<NamedDay> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 10) => {
            Ok(NamedDay::intNamedDays(_decode_NamedDay_intNamedDays(&el)?))
        }
        (TagClass::UNIVERSAL, 3) => Ok(NamedDay::bitNamedDays(_decode_NamedDay_bitNamedDays(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "NamedDay",
            ))
        }
    }
}

pub fn _encode_NamedDay(value_: &NamedDay) -> ASN1Result<X690Element> {
    match value_ {
        NamedDay::intNamedDays(v) => _encode_NamedDay_intNamedDays(&v),
        NamedDay::bitNamedDays(v) => _encode_NamedDay_bitNamedDays(&v),
    }
}

pub fn _validate_NamedDay(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 10) => _validate_NamedDay_intNamedDays(&el),
        (TagClass::UNIVERSAL, 3) => _validate_NamedDay_bitNamedDays(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "NamedDay",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DayTimeBand ::= SEQUENCE {
///   startDayTime  [0]  DayTime DEFAULT {hour 0},
///   endDayTime    [1]  DayTime DEFAULT {hour 23, minute 59, second 59},
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct DayTimeBand {
    pub startDayTime: OPTIONAL<DayTime>,
    pub endDayTime: OPTIONAL<DayTime>,
    pub _unrecognized: Vec<X690Element>,
}
impl DayTimeBand {
    pub fn new(
        startDayTime: OPTIONAL<DayTime>,
        endDayTime: OPTIONAL<DayTime>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DayTimeBand {
            startDayTime,
            endDayTime,
            _unrecognized,
        }
    }
    pub fn _default_value_for_startDayTime() -> DayTime {
        DayTime {
            hour: 0,
            minute: None,
            second: None,
            _unrecognized: vec![],
        }
    }
    pub fn _default_value_for_endDayTime() -> DayTime {
        DayTime {
            hour: 23,
            minute: Some(59),
            second: Some(59),
            _unrecognized: vec![],
        }
    }
}
impl Default for DayTimeBand {
    fn default() -> Self {
        DayTimeBand {
            startDayTime: None,
            endDayTime: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<&X690Element> for DayTimeBand {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DayTimeBand(el)
    }
}

pub const _rctl1_components_for_DayTimeBand: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "startDayTime",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "endDayTime",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DayTimeBand: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DayTimeBand: &[ComponentSpec; 0] = &[];

pub fn _decode_DayTimeBand(el: &X690Element) -> ASN1Result<DayTimeBand> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DayTimeBand")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DayTimeBand,
        _eal_components_for_DayTimeBand,
        _rctl2_components_for_DayTimeBand,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut startDayTime_: OPTIONAL<DayTime> = None;
    let mut endDayTime_: OPTIONAL<DayTime> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "startDayTime" => {
                startDayTime_ = Some(|el: &X690Element| -> ASN1Result<DayTime> {
                    Ok(_decode_DayTime(&el.inner()?)?)
                }(_el)?)
            }
            "endDayTime" => {
                endDayTime_ = Some(|el: &X690Element| -> ASN1Result<DayTime> {
                    Ok(_decode_DayTime(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(DayTimeBand {
        startDayTime: startDayTime_,
        endDayTime: endDayTime_,
        _unrecognized,
    })
}

pub fn _encode_DayTimeBand(value_: &DayTimeBand) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.startDayTime {
        if *v_ != DayTimeBand::_default_value_for_startDayTime() {
            components_.push(|v_1: &DayTime| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&_encode_DayTime(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    if let Some(v_) = &value_.endDayTime {
        if *v_ != DayTimeBand::_default_value_for_endDayTime() {
            components_.push(|v_1: &DayTime| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_DayTime(&v_1)?),
                ))
            }(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DayTimeBand(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DayTimeBand")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DayTimeBand,
        _eal_components_for_DayTimeBand,
        _rctl2_components_for_DayTimeBand,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "startDayTime" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "startDayTime")
                    );
                }
                Ok(_validate_DayTime(&el.inner()?)?)
            }(_el)?,
            "endDayTime" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "endDayTime")
                    );
                }
                Ok(_validate_DayTime(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// DayTime ::= SEQUENCE {
///   hour    [0]  INTEGER(0..23),
///   minute  [1]  INTEGER(0..59) DEFAULT 0,
///   second  [2]  INTEGER(0..59) DEFAULT 0,
///   ... }
/// ```
///
#[derive(Debug, Clone)]
pub struct DayTime {
    pub hour: u8,
    pub minute: OPTIONAL<u8>,
    pub second: OPTIONAL<u8>,
    pub _unrecognized: Vec<X690Element>,
}
impl DayTime {
    pub fn new(
        hour: u8,
        minute: OPTIONAL<u8>,
        second: OPTIONAL<u8>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DayTime {
            hour,
            minute,
            second,
            _unrecognized,
        }
    }
    pub fn _default_value_for_minute() -> u8 {
        0
    }
    pub fn _default_value_for_second() -> u8 {
        0
    }
}
impl TryFrom<&X690Element> for DayTime {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_DayTime(el)
    }
}
impl PartialEq for DayTime {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour || self.minute == other.minute || self.second == other.second
    }
}

pub const _rctl1_components_for_DayTime: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "hour",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "minute",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "second",
        true,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_DayTime: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_DayTime: &[ComponentSpec; 0] = &[];

pub fn _decode_DayTime(el: &X690Element) -> ASN1Result<DayTime> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DayTime")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DayTime,
        _eal_components_for_DayTime,
        _rctl2_components_for_DayTime,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut hour_: OPTIONAL<u8> = None;
    let mut minute_: OPTIONAL<u8> = None;
    let mut second_: OPTIONAL<u8> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hour" => hour_ = Some(BER.decode_u8(&_el.inner()?)?),
            "minute" => minute_ = Some(BER.decode_u8(&_el.inner()?)?),
            "second" => second_ = Some(BER.decode_u8(&_el.inner()?)?),
            _ => _unrecognized.push(_el.clone()),
        };
    }
    Ok(DayTime {
        hour: hour_.unwrap(),
        minute: minute_,
        second: second_,
        _unrecognized,
    })
}

pub fn _encode_DayTime(value_: &DayTime) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(X690Element::new(
        Tag::new(TagClass::CONTEXT, 0),
        X690Value::from_explicit(&BER.encode_u8(value_.hour)?),
    ));
    if let Some(v_) = &value_.minute {
        if *v_ != DayTime::_default_value_for_minute() {
            components_.push(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&BER.encode_u8(*v_)?),
            ));
        }
    }
    if let Some(v_) = &value_.second {
        if *v_ != DayTime::_default_value_for_second() {
            components_.push(X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::from_explicit(&BER.encode_u8(*v_)?),
            ));
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_DayTime(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "DayTime")),
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_DayTime,
        _eal_components_for_DayTime,
        _rctl2_components_for_DayTime,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "hour" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "hour"));
                }
                Ok(BER.validate_u8(&el.inner()?)?)
            }(_el)?,
            "minute" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "minute"));
                }
                Ok(BER.validate_u8(&el.inner()?)?)
            }(_el)?,
            "second" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "second"));
                }
                Ok(BER.validate_u8(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeZone  ::=  INTEGER(-12..12)
/// ```
pub type TimeZone = INTEGER;

pub fn _decode_TimeZone(el: &X690Element) -> ASN1Result<TimeZone> {
    BER.decode_integer(&el)
}

pub fn _encode_TimeZone(value_: &TimeZone) -> ASN1Result<X690Element> {
    BER.encode_integer(&value_)
}

pub fn _validate_TimeZone(el: &X690Element) -> ASN1Result<()> {
    BER.validate_integer(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeAssertion  ::=  CHOICE {
///   now             NULL,
///   at              GeneralizedTime,
///   between         SEQUENCE {
///     startTime  [0]  GeneralizedTime,
///     endTime    [1]  GeneralizedTime OPTIONAL,
///     entirely        BOOLEAN DEFAULT FALSE,
///     ...},
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum TimeAssertion {
    now(NULL),
    at(GeneralizedTime),
    between(TimeAssertion_between),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for TimeAssertion {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimeAssertion(el)
    }
}

pub fn _decode_TimeAssertion(el: &X690Element) -> ASN1Result<TimeAssertion> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 5) => Ok(TimeAssertion::now(BER.decode_null(&el)?)),
        (TagClass::UNIVERSAL, 24) => Ok(TimeAssertion::at(BER.decode_generalized_time(&el)?)),
        (TagClass::UNIVERSAL, 16) => {
            Ok(TimeAssertion::between(_decode_TimeAssertion_between(&el)?))
        }
        _ => Ok(TimeAssertion::_unrecognized(el.clone())),
    }
}

pub fn _encode_TimeAssertion(value_: &TimeAssertion) -> ASN1Result<X690Element> {
    match value_ {
        TimeAssertion::now(v) => BER.encode_null(&v),
        TimeAssertion::at(v) => BER.encode_generalized_time(&v),
        TimeAssertion::between(v) => _encode_TimeAssertion_between(&v),
        TimeAssertion::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_TimeAssertion(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 5) => BER.validate_null(&el),
        (TagClass::UNIVERSAL, 24) => BER.validate_generalized_time(&el),
        (TagClass::UNIVERSAL, 16) => _validate_TimeAssertion_between(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// localeContext CONTEXT ::= {
///   WITH SYNTAX  LocaleContextSyntax
///   ID           id-avc-locale }
/// ```
///
///
pub fn localeContext() -> CONTEXT {
    CONTEXT {
        id: id_avc_locale(),     /* OBJECT_FIELD_SETTING */
        absentMatch: Some(true), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod localeContext {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = LocaleContextSyntax; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_LocaleContextSyntax(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_LocaleContextSyntax(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_LocaleContextSyntax(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LocaleContextSyntax  ::=  CHOICE {
///   localeID1  OBJECT IDENTIFIER,
///   localeID2  UnboundedDirectoryString,
///   ... }
/// ```
#[derive(Debug, Clone)]
pub enum LocaleContextSyntax {
    localeID1(OBJECT_IDENTIFIER),
    localeID2(UnboundedDirectoryString),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for LocaleContextSyntax {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_LocaleContextSyntax(el)
    }
}

pub fn _decode_LocaleContextSyntax(el: &X690Element) -> ASN1Result<LocaleContextSyntax> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 6) => Ok(LocaleContextSyntax::localeID1(
            BER.decode_object_identifier(&el)?,
        )),
        (TagClass::UNIVERSAL, 20) => Ok(LocaleContextSyntax::localeID2(
            _decode_UnboundedDirectoryString(&el)?,
        )),
        (TagClass::UNIVERSAL, 19) => Ok(LocaleContextSyntax::localeID2(
            _decode_UnboundedDirectoryString(&el)?,
        )),
        (TagClass::UNIVERSAL, 30) => Ok(LocaleContextSyntax::localeID2(
            _decode_UnboundedDirectoryString(&el)?,
        )),
        (TagClass::UNIVERSAL, 28) => Ok(LocaleContextSyntax::localeID2(
            _decode_UnboundedDirectoryString(&el)?,
        )),
        (TagClass::UNIVERSAL, 12) => Ok(LocaleContextSyntax::localeID2(
            _decode_UnboundedDirectoryString(&el)?,
        )),
        _ => Ok(LocaleContextSyntax::_unrecognized(el.clone())),
    }
}

pub fn _encode_LocaleContextSyntax(value_: &LocaleContextSyntax) -> ASN1Result<X690Element> {
    match value_ {
        LocaleContextSyntax::localeID1(v) => BER.encode_object_identifier(&v),
        LocaleContextSyntax::localeID2(v) => _encode_UnboundedDirectoryString(&v),
        LocaleContextSyntax::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_LocaleContextSyntax(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 6) => BER.validate_object_identifier(&el),
        (TagClass::UNIVERSAL, 20) => _validate_UnboundedDirectoryString(&el),
        (TagClass::UNIVERSAL, 19) => _validate_UnboundedDirectoryString(&el),
        (TagClass::UNIVERSAL, 30) => _validate_UnboundedDirectoryString(&el),
        (TagClass::UNIVERSAL, 28) => _validate_UnboundedDirectoryString(&el),
        (TagClass::UNIVERSAL, 12) => _validate_UnboundedDirectoryString(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldapAttributeOptionContext CONTEXT ::= {
///   WITH SYNTAX  AttributeOptionList
///   ASSERTED AS  AttributeOptionList
///   ABSENT-MATCH FALSE
///   ID           id-avc-ldapAttributeOption }
/// ```
///
///
pub fn ldapAttributeOptionContext() -> CONTEXT {
    CONTEXT {
        absentMatch: Some(false),         /* OBJECT_FIELD_SETTING */
        id: id_avc_ldapAttributeOption(), /* OBJECT_FIELD_SETTING */
    }
}

pub mod ldapAttributeOptionContext {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = AttributeOptionList; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_AttributeOptionList(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_AttributeOptionList(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeOptionList(el)
    }
    pub type Assertion = AttributeOptionList; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Assertion(el: &X690Element) -> ASN1Result<Assertion> {
        _decode_AttributeOptionList(el)
    }
    pub fn _encode_Assertion(value_: &Assertion) -> ASN1Result<X690Element> {
        _encode_AttributeOptionList(value_)
    }
    pub fn _validate_Assertion(el: &X690Element) -> ASN1Result<()> {
        _validate_AttributeOptionList(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeOptionList  ::=  SEQUENCE OF UTF8String
/// ```
pub type AttributeOptionList = Vec<UTF8String>; // SequenceOfType

pub fn _decode_AttributeOptionList(el: &X690Element) -> ASN1Result<AttributeOptionList> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeOptionList")
            )
        }
    };
    let mut items: SEQUENCE_OF<UTF8String> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(BER.decode_utf8_string(el)?);
    }
    Ok(items)
}

pub fn _encode_AttributeOptionList(value_: &AttributeOptionList) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(BER.encode_utf8_string(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_AttributeOptionList(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                BER.validate_utf8_string(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AttributeOptionList")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-knowledgeInformation                OBJECT IDENTIFIER ::= {id-at 2}
/// ```
///
///
pub fn id_at_knowledgeInformation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-commonName                          OBJECT IDENTIFIER ::= {id-at 3}
/// ```
///
///
pub fn id_at_commonName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-surname                             OBJECT IDENTIFIER ::= {id-at 4}
/// ```
///
///
pub fn id_at_surname() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-serialNumber                        OBJECT IDENTIFIER ::= {id-at 5}
/// ```
///
///
pub fn id_at_serialNumber() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-countryName                         OBJECT IDENTIFIER ::= {id-at 6}
/// ```
///
///
pub fn id_at_countryName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-localityName                        OBJECT IDENTIFIER ::= {id-at 7}
/// ```
///
///
pub fn id_at_localityName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveLocalityName              OBJECT IDENTIFIER ::= {id-at 7 1}
/// ```
///
///
pub fn id_at_collectiveLocalityName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([7, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-stateOrProvinceName                 OBJECT IDENTIFIER ::= {id-at 8}
/// ```
///
///
pub fn id_at_stateOrProvinceName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveStateOrProvinceName       OBJECT IDENTIFIER ::= {id-at 8 1}
/// ```
///
///
pub fn id_at_collectiveStateOrProvinceName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([8, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-streetAddress                       OBJECT IDENTIFIER ::= {id-at 9}
/// ```
///
///
pub fn id_at_streetAddress() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveStreetAddress             OBJECT IDENTIFIER ::= {id-at 9 1}
/// ```
///
///
pub fn id_at_collectiveStreetAddress() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([9, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-organizationName                    OBJECT IDENTIFIER ::= {id-at 10}
/// ```
///
///
pub fn id_at_organizationName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveOrganizationName          OBJECT IDENTIFIER ::= {id-at 10 1}
/// ```
///
///
pub fn id_at_collectiveOrganizationName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([10, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-organizationalUnitName              OBJECT IDENTIFIER ::= {id-at 11}
/// ```
///
///
pub fn id_at_organizationalUnitName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveOrganizationalUnitName    OBJECT IDENTIFIER ::= {id-at 11 1}
/// ```
///
///
pub fn id_at_collectiveOrganizationalUnitName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([11, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-title                               OBJECT IDENTIFIER ::= {id-at 12}
/// ```
///
///
pub fn id_at_title() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-description                         OBJECT IDENTIFIER ::= {id-at 13}
/// ```
///
///
pub fn id_at_description() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-searchGuide                         OBJECT IDENTIFIER ::= {id-at 14}
/// ```
///
///
pub fn id_at_searchGuide() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([14])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-businessCategory                    OBJECT IDENTIFIER ::= {id-at 15}
/// ```
///
///
pub fn id_at_businessCategory() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([15])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-postalAddress                       OBJECT IDENTIFIER ::= {id-at 16}
/// ```
///
///
pub fn id_at_postalAddress() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([16])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectivePostalAddress             OBJECT IDENTIFIER ::= {id-at 16 1}
/// ```
///
///
pub fn id_at_collectivePostalAddress() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([16, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-postalCode                          OBJECT IDENTIFIER ::= {id-at 17}
/// ```
///
///
pub fn id_at_postalCode() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([17])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectivePostalCode                OBJECT IDENTIFIER ::= {id-at 17 1}
/// ```
///
///
pub fn id_at_collectivePostalCode() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([17, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-postOfficeBox                       OBJECT IDENTIFIER ::= {id-at 18}
/// ```
///
///
pub fn id_at_postOfficeBox() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([18])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectivePostOfficeBox             OBJECT IDENTIFIER ::= {id-at 18 1}
/// ```
///
///
pub fn id_at_collectivePostOfficeBox() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([18, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-physicalDeliveryOfficeName          OBJECT IDENTIFIER ::= {id-at 19}
/// ```
///
///
pub fn id_at_physicalDeliveryOfficeName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([19])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectivePhysicalDeliveryOfficeName
///                                           OBJECT IDENTIFIER ::= {id-at 19 1}
/// ```
///
///
pub fn id_at_collectivePhysicalDeliveryOfficeName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([19, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-telephoneNumber                     OBJECT IDENTIFIER ::= {id-at 20}
/// ```
///
///
pub fn id_at_telephoneNumber() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([20])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveTelephoneNumber           OBJECT IDENTIFIER ::= {id-at 20 1}
/// ```
///
///
pub fn id_at_collectiveTelephoneNumber() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([20, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-telexNumber                         OBJECT IDENTIFIER ::= {id-at 21}
/// ```
///
///
pub fn id_at_telexNumber() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([21])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveTelexNumber               OBJECT IDENTIFIER ::= {id-at 21 1}
/// ```
///
///
pub fn id_at_collectiveTelexNumber() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([21, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-facsimileTelephoneNumber            OBJECT IDENTIFIER ::= {id-at 23}
/// ```
///
///
pub fn id_at_facsimileTelephoneNumber() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([23])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveFacsimileTelephoneNumber  OBJECT IDENTIFIER ::= {id-at 23 1}
/// ```
///
///
pub fn id_at_collectiveFacsimileTelephoneNumber() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([23, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-x121Address                         OBJECT IDENTIFIER ::= {id-at 24}
/// ```
///
///
pub fn id_at_x121Address() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([24])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-internationalISDNNumber             OBJECT IDENTIFIER ::= {id-at 25}
/// ```
///
///
pub fn id_at_internationalISDNNumber() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([25])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveInternationalISDNNumber   OBJECT IDENTIFIER ::= {id-at 25 1}
/// ```
///
///
pub fn id_at_collectiveInternationalISDNNumber() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([25, 1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-registeredAddress                   OBJECT IDENTIFIER ::= {id-at 26}
/// ```
///
///
pub fn id_at_registeredAddress() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([26])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-destinationIndicator                OBJECT IDENTIFIER ::= {id-at 27}
/// ```
///
///
pub fn id_at_destinationIndicator() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([27])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-preferredDeliveryMethod             OBJECT IDENTIFIER ::= {id-at 28}
/// ```
///
///
pub fn id_at_preferredDeliveryMethod() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([28])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-presentationAddress                 OBJECT IDENTIFIER ::= {id-at 29}
/// ```
///
///
pub fn id_at_presentationAddress() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([29])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-supportedApplicationContext         OBJECT IDENTIFIER ::= {id-at 30}
/// ```
///
///
pub fn id_at_supportedApplicationContext() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([30])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-member                              OBJECT IDENTIFIER ::= {id-at 31}
/// ```
///
///
pub fn id_at_member() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([31])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-owner                               OBJECT IDENTIFIER ::= {id-at 32}
/// ```
///
///
pub fn id_at_owner() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([32])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-roleOccupant                        OBJECT IDENTIFIER ::= {id-at 33}
/// ```
///
///
pub fn id_at_roleOccupant() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([33])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-seeAlso                             OBJECT IDENTIFIER ::= {id-at 34}
/// ```
///
///
pub fn id_at_seeAlso() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([34])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-name                                OBJECT IDENTIFIER ::= {id-at 41}
/// ```
///
///
pub fn id_at_name() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([41])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-givenName                           OBJECT IDENTIFIER ::= {id-at 42}
/// ```
///
///
pub fn id_at_givenName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([42])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-initials                            OBJECT IDENTIFIER ::= {id-at 43}
/// ```
///
///
pub fn id_at_initials() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([43])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-generationQualifier                 OBJECT IDENTIFIER ::= {id-at 44}
/// ```
///
///
pub fn id_at_generationQualifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([44])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uniqueIdentifier                    OBJECT IDENTIFIER ::= {id-at 45}
/// ```
///
///
pub fn id_at_uniqueIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([45])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-dnQualifier                         OBJECT IDENTIFIER ::= {id-at 46}
/// ```
///
///
pub fn id_at_dnQualifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([46])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-enhancedSearchGuide                 OBJECT IDENTIFIER ::= {id-at 47}
/// ```
///
///
pub fn id_at_enhancedSearchGuide() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([47])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-protocolInformation                 OBJECT IDENTIFIER ::= {id-at 48}
/// ```
///
///
pub fn id_at_protocolInformation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([48])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-distinguishedName                   OBJECT IDENTIFIER ::= {id-at 49}
/// ```
///
///
pub fn id_at_distinguishedName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([49])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uniqueMember                        OBJECT IDENTIFIER ::= {id-at 50}
/// ```
///
///
pub fn id_at_uniqueMember() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([50])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-houseIdentifier                     OBJECT IDENTIFIER ::= {id-at 51}
/// ```
///
///
pub fn id_at_houseIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([51])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-dmdName                             OBJECT IDENTIFIER ::= {id-at 54}
/// ```
///
///
pub fn id_at_dmdName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([54])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-pseudonym                           OBJECT IDENTIFIER ::= {id-at 65}
/// ```
///
///
pub fn id_at_pseudonym() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([65])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-communicationsService               OBJECT IDENTIFIER ::= {id-at 66}
/// ```
///
///
pub fn id_at_communicationsService() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([66])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-communicationsNetwork               OBJECT IDENTIFIER ::= {id-at 67}
/// ```
///
///
pub fn id_at_communicationsNetwork() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([67])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uuidpair                            OBJECT IDENTIFIER ::= {id-at 77}
/// ```
///
///
pub fn id_at_uuidpair() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([77])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-tagOid                              OBJECT IDENTIFIER ::= {id-at 78}
/// ```
///
///
pub fn id_at_tagOid() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([78])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uiiFormat                           OBJECT IDENTIFIER ::= {id-at 79}
/// ```
///
///
pub fn id_at_uiiFormat() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([79])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uiiInUrn                            OBJECT IDENTIFIER ::= {id-at 80}
/// ```
///
///
pub fn id_at_uiiInUrn() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([80])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-contentUrl                          OBJECT IDENTIFIER ::= {id-at 81}
/// ```
///
///
pub fn id_at_contentUrl() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([81])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uri                                 OBJECT IDENTIFIER ::= {id-at 83}
/// ```
///
///
pub fn id_at_uri() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([83])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-urn                                 OBJECT IDENTIFIER ::= {id-at 86}
/// ```
///
///
pub fn id_at_urn() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([86])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-url                                 OBJECT IDENTIFIER ::= {id-at 87}
/// ```
///
///
pub fn id_at_url() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([87])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-utmCoordinates                      OBJECT IDENTIFIER ::= {id-at 88}
/// ```
///
///
pub fn id_at_utmCoordinates() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([88])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-urnC                                OBJECT IDENTIFIER ::= {id-at 89}
/// ```
///
///
pub fn id_at_urnC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([89])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uii                                 OBJECT IDENTIFIER ::= {id-at 90}
/// ```
///
///
pub fn id_at_uii() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([90])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-epc                                 OBJECT IDENTIFIER ::= {id-at 91}
/// ```
///
///
pub fn id_at_epc() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([91])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-tagAfi                              OBJECT IDENTIFIER ::= {id-at 92}
/// ```
///
///
pub fn id_at_tagAfi() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([92])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-epcFormat                           OBJECT IDENTIFIER ::= {id-at 93}
/// ```
///
///
pub fn id_at_epcFormat() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([93])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-epcInUrn                            OBJECT IDENTIFIER ::= {id-at 94}
/// ```
///
///
pub fn id_at_epcInUrn() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([94])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-ldapUrl                             OBJECT IDENTIFIER ::= {id-at 95}
/// ```
///
///
pub fn id_at_ldapUrl() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([95])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-tagLocation                         OBJECT IDENTIFIER ::= {id-at 96}
/// ```
///
///
pub fn id_at_tagLocation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([96])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-organizationIdentifier              OBJECT IDENTIFIER ::= {id-at 97}
/// ```
///
///
pub fn id_at_organizationIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([97])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-countryCode3c                       OBJECT IDENTIFIER ::= {id-at 98}
/// ```
///
///
pub fn id_at_countryCode3c() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([98])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-countryCode3n                       OBJECT IDENTIFIER ::= {id-at 99}
/// ```
///
///
pub fn id_at_countryCode3n() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([99])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-dnsName                             OBJECT IDENTIFIER ::= {id-at 100}
/// ```
///
///
pub fn id_at_dnsName() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([100])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-intEmail                            OBJECT IDENTIFIER ::= {id-at 104}
/// ```
///
///
pub fn id_at_intEmail() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([104])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-jid                                 OBJECT IDENTIFIER ::= {id-at 105}
/// ```
///
///
pub fn id_at_jid() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([105])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-objectIdentifier                    OBJECT IDENTIFIER ::= {id-at 106}
/// ```
///
///
pub fn id_at_objectIdentifier() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_at().0, Vec::<u32>::from([106])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-utmCoords                          OBJECT IDENTIFIER ::= {id-asx 4}
/// ```
///
///
pub fn id_asx_utmCoords() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_asx().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-uiiForm                            OBJECT IDENTIFIER ::= {id-asx 5}
/// ```
///
///
pub fn id_asx_uiiForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_asx().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-epcForm                            OBJECT IDENTIFIER ::= {id-asx 6}
/// ```
///
///
pub fn id_asx_epcForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_asx().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-countryString3c                    OBJECT IDENTIFIER ::= {id-asx 7}
/// ```
///
///
pub fn id_asx_countryString3c() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_asx().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-countryString3n                    OBJECT IDENTIFIER ::= {id-asx 8}
/// ```
///
///
pub fn id_asx_countryString3n() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_asx().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-dnsString                          OBJECT IDENTIFIER ::= {id-asx 9}
/// ```
///
///
pub fn id_asx_dnsString() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_asx().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-intEmailString                     OBJECT IDENTIFIER ::= {id-asx 11}
/// ```
///
///
pub fn id_asx_intEmailString() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_asx().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-jidString                          OBJECT IDENTIFIER ::= {id-asx 12}
/// ```
///
///
pub fn id_asx_jidString() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_asx().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-attributeTypeDescription           OBJECT IDENTIFIER ::= {id-lsx 3}
/// ```
///
///
pub fn id_lsx_attributeTypeDescription() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-bitString                          OBJECT IDENTIFIER ::= {id-lsx 6}
/// ```
///
///
pub fn id_lsx_bitString() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-boolean                            OBJECT IDENTIFIER ::= {id-lsx 7}
/// ```
///
///
pub fn id_lsx_boolean() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-countryString                      OBJECT IDENTIFIER ::= {id-lsx 11}
/// ```
///
///
pub fn id_lsx_countryString() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-dn                                 OBJECT IDENTIFIER ::= {id-lsx 12}
/// ```
///
///
pub fn id_lsx_dn() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-deliveryMethod                     OBJECT IDENTIFIER ::= {id-lsx 14}
/// ```
///
///
pub fn id_lsx_deliveryMethod() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([14])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-directoryString                    OBJECT IDENTIFIER ::= {id-lsx 15}
/// ```
///
///
pub fn id_lsx_directoryString() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([15])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-dITContentRuleDescription          OBJECT IDENTIFIER ::= {id-lsx 16}
/// ```
///
///
pub fn id_lsx_dITContentRuleDescription() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([16])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-dITStructureRuleDescription        OBJECT IDENTIFIER ::= {id-lsx 17}
/// ```
///
///
pub fn id_lsx_dITStructureRuleDescription() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([17])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-enhancedGuide                      OBJECT IDENTIFIER ::= {id-lsx 21}
/// ```
///
///
pub fn id_lsx_enhancedGuide() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([21])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-facsimileTelephoneNr               OBJECT IDENTIFIER ::= {id-lsx 22}
/// ```
///
///
pub fn id_lsx_facsimileTelephoneNr() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([22])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-fax                                OBJECT IDENTIFIER ::= {id-lsx 23}
/// ```
///
///
pub fn id_lsx_fax() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([23])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-generalizedTime                    OBJECT IDENTIFIER ::= {id-lsx 24}
/// ```
///
///
pub fn id_lsx_generalizedTime() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([24])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-guide                              OBJECT IDENTIFIER ::= {id-lsx 25}
/// ```
///
///
pub fn id_lsx_guide() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([25])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-ia5String                          OBJECT IDENTIFIER ::= {id-lsx 26}
/// ```
///
///
pub fn id_lsx_ia5String() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([26])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-integer                            OBJECT IDENTIFIER ::= {id-lsx 27}
/// ```
///
///
pub fn id_lsx_integer() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([27])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-jpeg                               OBJECT IDENTIFIER ::= {id-lsx 28}
/// ```
///
///
pub fn id_lsx_jpeg() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([28])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-matchingRuleDescription            OBJECT IDENTIFIER ::= {id-lsx 30}
/// ```
///
///
pub fn id_lsx_matchingRuleDescription() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([30])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-matchingRuleUseDescription         OBJECT IDENTIFIER ::= {id-lsx 31}
/// ```
///
///
pub fn id_lsx_matchingRuleUseDescription() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([31])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-nameAndOptionalUID                 OBJECT IDENTIFIER ::= {id-lsx 34}
/// ```
///
///
pub fn id_lsx_nameAndOptionalUID() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([34])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-nameFormDescription                OBJECT IDENTIFIER ::= {id-lsx 35}
/// ```
///
///
pub fn id_lsx_nameFormDescription() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([35])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-numericString                      OBJECT IDENTIFIER ::= {id-lsx 36}
/// ```
///
///
pub fn id_lsx_numericString() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([36])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-objectClassDescription             OBJECT IDENTIFIER ::= {id-lsx 37}
/// ```
///
///
pub fn id_lsx_objectClassDescription() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([37])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-oid                                OBJECT IDENTIFIER ::= {id-lsx 38}
/// ```
///
///
pub fn id_lsx_oid() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([38])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-otherMailbox                       OBJECT IDENTIFIER ::= {id-lsx 39}
/// ```
///
///
pub fn id_lsx_otherMailbox() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([39])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-octetString                        OBJECT IDENTIFIER ::= {id-lsx 40}
/// ```
///
///
pub fn id_lsx_octetString() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([40])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-postalAddr                         OBJECT IDENTIFIER ::= {id-lsx 41}
/// ```
///
///
pub fn id_lsx_postalAddr() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([41])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-presentationAddr                   OBJECT IDENTIFIER ::= {id-lsx 43}
/// ```
///
///
pub fn id_lsx_presentationAddr() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([43])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-printableString                    OBJECT IDENTIFIER ::= {id-lsx 44}
/// ```
///
///
pub fn id_lsx_printableString() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([44])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-subtreeSpec                        OBJECT IDENTIFIER ::= {id-lsx 45}
/// ```
///
///
pub fn id_lsx_subtreeSpec() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([45])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-telephoneNr                        OBJECT IDENTIFIER ::= {id-lsx 50}
/// ```
///
///
pub fn id_lsx_telephoneNr() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([50])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-telexNr                            OBJECT IDENTIFIER ::= {id-lsx 52}
/// ```
///
///
pub fn id_lsx_telexNr() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([52])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-utcTime                            OBJECT IDENTIFIER ::= {id-lsx 53}
/// ```
///
///
pub fn id_lsx_utcTime() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([53])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-ldapSyntaxDescription              OBJECT IDENTIFIER ::= {id-lsx 54}
/// ```
///
///
pub fn id_lsx_ldapSyntaxDescription() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([54])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-substringAssertion                 OBJECT IDENTIFIER ::= {id-lsx 58}
/// ```
///
///
pub fn id_lsx_substringAssertion() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lsx().0, Vec::<u32>::from([58])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oidC1                                  OBJECT IDENTIFIER ::= {id 0}
/// ```
///
///
pub fn id_oidC1() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id().0, Vec::<u32>::from([0])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oidC2                                  OBJECT IDENTIFIER ::= {id 1}
/// ```
///
///
pub fn id_oidC2() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oidC                                   OBJECT IDENTIFIER ::= {id 2}
/// ```
///
///
pub fn id_oidC() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cat-sequenceMatchType                  OBJECT IDENTIFIER ::= {id-cat 1}
/// ```
///
///
pub fn id_cat_sequenceMatchType() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cat().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cat-wordMatchType                      OBJECT IDENTIFIER ::= {id-cat 2}
/// ```
///
///
pub fn id_cat_wordMatchType() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cat().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cat-characterMatchTypes                OBJECT IDENTIFIER ::= {id-cat 3}
/// ```
///
///
pub fn id_cat_characterMatchTypes() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cat().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cat-selectedContexts                   OBJECT IDENTIFIER ::= {id-cat 4}
/// ```
///
///
pub fn id_cat_selectedContexts() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_cat().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-dSAProblem                         OBJECT IDENTIFIER ::= {id-not 0}
/// ```
///
///
pub fn id_not_dSAProblem() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([0])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-searchServiceProblem               OBJECT IDENTIFIER ::= {id-not 1}
/// ```
///
///
pub fn id_not_searchServiceProblem() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-serviceType                        OBJECT IDENTIFIER ::= {id-not 2}
/// ```
///
///
pub fn id_not_serviceType() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-attributeTypeList                  OBJECT IDENTIFIER ::= {id-not 3}
/// ```
///
///
pub fn id_not_attributeTypeList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-matchingRuleList                   OBJECT IDENTIFIER ::= {id-not 4}
/// ```
///
///
pub fn id_not_matchingRuleList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-filterItem                         OBJECT IDENTIFIER ::= {id-not 5}
/// ```
///
///
pub fn id_not_filterItem() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-attributeCombinations              OBJECT IDENTIFIER ::= {id-not 6}
/// ```
///
///
pub fn id_not_attributeCombinations() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-contextTypeList                    OBJECT IDENTIFIER ::= {id-not 7}
/// ```
///
///
pub fn id_not_contextTypeList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-contextList                        OBJECT IDENTIFIER ::= {id-not 8}
/// ```
///
///
pub fn id_not_contextList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-contextCombinations                OBJECT IDENTIFIER ::= {id-not 9}
/// ```
///
///
pub fn id_not_contextCombinations() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-hierarchySelectList                OBJECT IDENTIFIER ::= {id-not 10}
/// ```
///
///
pub fn id_not_hierarchySelectList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-searchControlOptionsList           OBJECT IDENTIFIER ::= {id-not 11}
/// ```
///
///
pub fn id_not_searchControlOptionsList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-serviceControlOptionsList          OBJECT IDENTIFIER ::= {id-not 12}
/// ```
///
///
pub fn id_not_serviceControlOptionsList() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-multipleMatchingLocalities         OBJECT IDENTIFIER ::= {id-not 13}
/// ```
///
///
pub fn id_not_multipleMatchingLocalities() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-proposedRelaxation                 OBJECT IDENTIFIER ::= {id-not 14}
/// ```
///
///
pub fn id_not_proposedRelaxation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([14])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-appliedRelaxation                  OBJECT IDENTIFIER ::= {id-not 15}
/// ```
///
///
pub fn id_not_appliedRelaxation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([15])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-pwdResponse                        OBJECT IDENTIFIER ::= {id-not 16}
/// ```
///
///
pub fn id_not_pwdResponse() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([16])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-ldapDiagnosticMsg                  OBJECT IDENTIFIER ::= {id-not 17}
/// ```
///
///
pub fn id_not_ldapDiagnosticMsg() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_not().0, Vec::<u32>::from([17])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-targetDsaUnavailable                OBJECT IDENTIFIER ::= {id-pr 1}
/// ```
///
///
pub fn id_pr_targetDsaUnavailable() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-dataSourceUnavailable               OBJECT IDENTIFIER ::= {id-pr 2}
/// ```
///
///
pub fn id_pr_dataSourceUnavailable() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unidentifiedOperation               OBJECT IDENTIFIER ::= {id-pr 3}
/// ```
///
///
pub fn id_pr_unidentifiedOperation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unavailableOperation                OBJECT IDENTIFIER ::= {id-pr 4}
/// ```
///
///
pub fn id_pr_unavailableOperation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchAttributeViolation            OBJECT IDENTIFIER ::= {id-pr 5}
/// ```
///
///
pub fn id_pr_searchAttributeViolation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchAttributeCombinationViolation OBJECT IDENTIFIER ::= {id-pr 6}
/// ```
///
///
pub fn id_pr_searchAttributeCombinationViolation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchValueNotAllowed               OBJECT IDENTIFIER ::= {id-pr 7}
/// ```
///
///
pub fn id_pr_searchValueNotAllowed() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-missingSearchAttribute              OBJECT IDENTIFIER ::= {id-pr 8}
/// ```
///
///
pub fn id_pr_missingSearchAttribute() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchValueViolation                OBJECT IDENTIFIER ::= {id-pr 9}
/// ```
///
///
pub fn id_pr_searchValueViolation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-attributeNegationViolation          OBJECT IDENTIFIER ::= {id-pr 10}
/// ```
///
///
pub fn id_pr_attributeNegationViolation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchValueRequired                 OBJECT IDENTIFIER ::= {id-pr 11}
/// ```
///
///
pub fn id_pr_searchValueRequired() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-invalidSearchValue                  OBJECT IDENTIFIER ::= {id-pr 12}
/// ```
///
///
pub fn id_pr_invalidSearchValue() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchContextViolation              OBJECT IDENTIFIER ::= {id-pr 13}
/// ```
///
///
pub fn id_pr_searchContextViolation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchContextCombinationViolation   OBJECT IDENTIFIER ::= {id-pr 14}
/// ```
///
///
pub fn id_pr_searchContextCombinationViolation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([14])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-missingSearchContext                OBJECT IDENTIFIER ::= {id-pr 15}
/// ```
///
///
pub fn id_pr_missingSearchContext() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([15])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchContextValueViolation         OBJECT IDENTIFIER ::= {id-pr 16}
/// ```
///
///
pub fn id_pr_searchContextValueViolation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([16])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchContextValueRequired          OBJECT IDENTIFIER ::= {id-pr 17}
/// ```
///
///
pub fn id_pr_searchContextValueRequired() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([17])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-invalidContextSearchValue           OBJECT IDENTIFIER ::= {id-pr 18}
/// ```
///
///
pub fn id_pr_invalidContextSearchValue() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([18])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unsupportedMatchingRule             OBJECT IDENTIFIER ::= {id-pr 19}
/// ```
///
///
pub fn id_pr_unsupportedMatchingRule() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([19])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-attributeMatchingViolation          OBJECT IDENTIFIER ::= {id-pr 20}
/// ```
///
///
pub fn id_pr_attributeMatchingViolation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([20])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unsupportedMatchingUse              OBJECT IDENTIFIER ::= {id-pr 21}
/// ```
///
///
pub fn id_pr_unsupportedMatchingUse() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([21])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-matchingUseViolation                OBJECT IDENTIFIER ::= {id-pr 22}
/// ```
///
///
pub fn id_pr_matchingUseViolation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([22])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-hierarchySelectForbidden            OBJECT IDENTIFIER ::= {id-pr 23}
/// ```
///
///
pub fn id_pr_hierarchySelectForbidden() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([23])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-invalidHierarchySelect              OBJECT IDENTIFIER ::= {id-pr 24}
/// ```
///
///
pub fn id_pr_invalidHierarchySelect() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([24])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unavailableHierarchySelect          OBJECT IDENTIFIER ::= {id-pr 25}
/// ```
///
///
pub fn id_pr_unavailableHierarchySelect() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([25])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-invalidSearchControlOptions         OBJECT IDENTIFIER ::= {id-pr 26}
/// ```
///
///
pub fn id_pr_invalidSearchControlOptions() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([26])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-invalidServiceControlOptions        OBJECT IDENTIFIER ::= {id-pr 27}
/// ```
///
///
pub fn id_pr_invalidServiceControlOptions() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([27])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchSubsetViolation               OBJECT IDENTIFIER ::= {id-pr 28}
/// ```
///
///
pub fn id_pr_searchSubsetViolation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([28])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unmatchedKeyAttributes              OBJECT IDENTIFIER ::= {id-pr 29}
/// ```
///
///
pub fn id_pr_unmatchedKeyAttributes() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([29])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-ambiguousKeyAttributes              OBJECT IDENTIFIER ::= {id-pr 30}
/// ```
///
///
pub fn id_pr_ambiguousKeyAttributes() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([30])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unavailableRelaxationLevel          OBJECT IDENTIFIER ::= {id-pr 31}
/// ```
///
///
pub fn id_pr_unavailableRelaxationLevel() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([31])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-emptyHierarchySelection             OBJECT IDENTIFIER ::= {id-pr 32}
/// ```
///
///
pub fn id_pr_emptyHierarchySelection() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([32])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-administratorImposedLimit           OBJECT IDENTIFIER ::= {id-pr 33}
/// ```
///
///
pub fn id_pr_administratorImposedLimit() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([33])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-permanentRestriction                OBJECT IDENTIFIER ::= {id-pr 34}
/// ```
///
///
pub fn id_pr_permanentRestriction() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([34])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-temporaryRestriction                OBJECT IDENTIFIER ::= {id-pr 35}
/// ```
///
///
pub fn id_pr_temporaryRestriction() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([35])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-relaxationNotSupported              OBJECT IDENTIFIER ::= {id-pr 36}
/// ```
///
///
pub fn id_pr_relaxationNotSupported() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_pr().0, Vec::<u32>::from([36])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-coat-uid                               OBJECT IDENTIFIER ::= {id-coat 1}
/// ```
///
///
pub fn id_coat_uid() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_coat().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-coat-dc                                OBJECT IDENTIFIER ::= {id-coat 25}
/// ```
///
///
pub fn id_coat_dc() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_coat().0, Vec::<u32>::from([25])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseIgnoreMatch                     OBJECT IDENTIFIER ::= {id-mr 2}
/// ```
///
///
pub fn id_mr_caseIgnoreMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseIgnoreOrderingMatch             OBJECT IDENTIFIER ::= {id-mr 3}
/// ```
///
///
pub fn id_mr_caseIgnoreOrderingMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseIgnoreSubstringsMatch           OBJECT IDENTIFIER ::= {id-mr 4}
/// ```
///
///
pub fn id_mr_caseIgnoreSubstringsMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseExactMatch                      OBJECT IDENTIFIER ::= {id-mr 5}
/// ```
///
///
pub fn id_mr_caseExactMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseExactOrderingMatch              OBJECT IDENTIFIER ::= {id-mr 6}
/// ```
///
///
pub fn id_mr_caseExactOrderingMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseExactSubstringsMatch            OBJECT IDENTIFIER ::= {id-mr 7}
/// ```
///
///
pub fn id_mr_caseExactSubstringsMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-numericStringMatch                  OBJECT IDENTIFIER ::= {id-mr 8}
/// ```
///
///
pub fn id_mr_numericStringMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-numericStringOrderingMatch          OBJECT IDENTIFIER ::= {id-mr 9}
/// ```
///
///
pub fn id_mr_numericStringOrderingMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-numericStringSubstringsMatch        OBJECT IDENTIFIER ::= {id-mr 10}
/// ```
///
///
pub fn id_mr_numericStringSubstringsMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseIgnoreListMatch                 OBJECT IDENTIFIER ::= {id-mr 11}
/// ```
///
///
pub fn id_mr_caseIgnoreListMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseIgnoreListSubstringsMatch       OBJECT IDENTIFIER ::= {id-mr 12}
/// ```
///
///
pub fn id_mr_caseIgnoreListSubstringsMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-booleanMatch                        OBJECT IDENTIFIER ::= {id-mr 13}
/// ```
///
///
pub fn id_mr_booleanMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-integerMatch                        OBJECT IDENTIFIER ::= {id-mr 14}
/// ```
///
///
pub fn id_mr_integerMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([14])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-integerOrderingMatch                OBJECT IDENTIFIER ::= {id-mr 15}
/// ```
///
///
pub fn id_mr_integerOrderingMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([15])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-bitStringMatch                      OBJECT IDENTIFIER ::= {id-mr 16}
/// ```
///
///
pub fn id_mr_bitStringMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([16])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-octetStringMatch                    OBJECT IDENTIFIER ::= {id-mr 17}
/// ```
///
///
pub fn id_mr_octetStringMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([17])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-octetStringOrderingMatch            OBJECT IDENTIFIER ::= {id-mr 18}
/// ```
///
///
pub fn id_mr_octetStringOrderingMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([18])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-octetStringSubstringsMatch          OBJECT IDENTIFIER ::= {id-mr 19}
/// ```
///
///
pub fn id_mr_octetStringSubstringsMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([19])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-telephoneNumberMatch                OBJECT IDENTIFIER ::= {id-mr 20}
/// ```
///
///
pub fn id_mr_telephoneNumberMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([20])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-telephoneNumberSubstringsMatch      OBJECT IDENTIFIER ::= {id-mr 21}
/// ```
///
///
pub fn id_mr_telephoneNumberSubstringsMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([21])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-presentationAddressMatch            OBJECT IDENTIFIER ::= {id-mr 22}
/// ```
///
///
pub fn id_mr_presentationAddressMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([22])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-uniqueMemberMatch                   OBJECT IDENTIFIER ::= {id-mr 23}
/// ```
///
///
pub fn id_mr_uniqueMemberMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([23])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-protocolInformationMatch            OBJECT IDENTIFIER ::= {id-mr 24}
/// ```
///
///
pub fn id_mr_protocolInformationMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([24])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-uTCTimeMatch                        OBJECT IDENTIFIER ::= {id-mr 25}
/// ```
///
///
pub fn id_mr_uTCTimeMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([25])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-uTCTimeOrderingMatch                OBJECT IDENTIFIER ::= {id-mr 26}
/// ```
///
///
pub fn id_mr_uTCTimeOrderingMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([26])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-generalizedTimeMatch                OBJECT IDENTIFIER ::= {id-mr 27}
/// ```
///
///
pub fn id_mr_generalizedTimeMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([27])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-generalizedTimeOrderingMatch        OBJECT IDENTIFIER ::= {id-mr 28}
/// ```
///
///
pub fn id_mr_generalizedTimeOrderingMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([28])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-integerFirstComponentMatch          OBJECT IDENTIFIER ::= {id-mr 29}
/// ```
///
///
pub fn id_mr_integerFirstComponentMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([29])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-objectIdentifierFirstComponentMatch OBJECT IDENTIFIER ::= {id-mr 30}
/// ```
///
///
pub fn id_mr_objectIdentifierFirstComponentMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([30])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-directoryStringFirstComponentMatch  OBJECT IDENTIFIER ::= {id-mr 31}
/// ```
///
///
pub fn id_mr_directoryStringFirstComponentMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([31])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-wordMatch                           OBJECT IDENTIFIER ::= {id-mr 32}
/// ```
///
///
pub fn id_mr_wordMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([32])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-keywordMatch                        OBJECT IDENTIFIER ::= {id-mr 33}
/// ```
///
///
pub fn id_mr_keywordMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([33])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-storedPrefixMatch                   OBJECT IDENTIFIER ::= {id-mr 41}
/// ```
///
///
pub fn id_mr_storedPrefixMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([41])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-systemProposedMatch                 OBJECT IDENTIFIER ::= {id-mr 47}
/// ```
///
///
pub fn id_mr_systemProposedMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([47])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-generalWordMatch                    OBJECT IDENTIFIER ::= {id-mr 48}
/// ```
///
///
pub fn id_mr_generalWordMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([48])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-approximateStringMatch              OBJECT IDENTIFIER ::= {id-mr 49}
/// ```
///
///
pub fn id_mr_approximateStringMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([49])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-ignoreIfAbsentMatch                 OBJECT IDENTIFIER ::= {id-mr 50}
/// ```
///
///
pub fn id_mr_ignoreIfAbsentMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([50])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-nullMatch                           OBJECT IDENTIFIER ::= {id-mr 51}
/// ```
///
///
pub fn id_mr_nullMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([51])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-zonalMatch                          OBJECT IDENTIFIER ::= {id-mr 52}
/// ```
///
///
pub fn id_mr_zonalMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([52])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-facsimileNumberMatch                OBJECT IDENTIFIER ::= {id-mr 63}
/// ```
///
///
pub fn id_mr_facsimileNumberMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([63])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-facsimileNumberSubstringsMatch      OBJECT IDENTIFIER ::= {id-mr 64}
/// ```
///
///
pub fn id_mr_facsimileNumberSubstringsMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([64])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-uuidpairmatch                       OBJECT IDENTIFIER ::= {id-mr 68}
/// ```
///
///
pub fn id_mr_uuidpairmatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([68])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-uriMatch                            OBJECT IDENTIFIER ::= {id-mr 70}
/// ```
///
///
pub fn id_mr_uriMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([70])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-dnsNameMatch                        OBJECT IDENTIFIER ::= {id-mr 74}
/// ```
///
///
pub fn id_mr_dnsNameMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([74])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-intEmailMatch                       OBJECT IDENTIFIER ::= {id-mr 75}
/// ```
///
///
pub fn id_mr_intEmailMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([75])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-jidMatch                            OBJECT IDENTIFIER ::= {id-mr 76}
/// ```
///
///
pub fn id_mr_jidMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_mr().0, Vec::<u32>::from([76])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lmr-caseExactIA5Match                  OBJECT IDENTIFIER ::= {id-lmr 1}
/// ```
///
///
pub fn id_lmr_caseExactIA5Match() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lmr().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lmr-caseIgnoreIA5Match                 OBJECT IDENTIFIER ::= {id-lmr 2}
/// ```
///
///
pub fn id_lmr_caseIgnoreIA5Match() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lmr().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lmr-caseIgnoreIA5SubstringsMatch       OBJECT IDENTIFIER ::= {id-lmr 3}
/// ```
///
///
pub fn id_lmr_caseIgnoreIA5SubstringsMatch() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lmr().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc-language                           OBJECT IDENTIFIER ::= {id-avc 0}
/// ```
///
///
pub fn id_avc_language() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_avc().0, Vec::<u32>::from([0])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc-temporal                           OBJECT IDENTIFIER ::= {id-avc 1}
/// ```
///
///
pub fn id_avc_temporal() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_avc().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc-locale                             OBJECT IDENTIFIER ::= {id-avc 2}
/// ```
///
///
pub fn id_avc_locale() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_avc().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc-ldapAttributeOption                OBJECT IDENTIFIER ::= {id-avc 5}
/// ```
///
///
pub fn id_avc_ldapAttributeOption() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_avc().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EnhancedGuide-subset ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type EnhancedGuide_subset = i8;

pub const EnhancedGuide_subset_baseObject: EnhancedGuide_subset = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const EnhancedGuide_subset_oneLevel: EnhancedGuide_subset = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const EnhancedGuide_subset_wholeSubtree: EnhancedGuide_subset = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_EnhancedGuide_subset(el: &X690Element) -> ASN1Result<EnhancedGuide_subset> {
    BER.decode_i8(&el)
}

pub fn _encode_EnhancedGuide_subset(value_: &EnhancedGuide_subset) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_EnhancedGuide_subset(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PreferredDeliveryMethod-Item ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type PreferredDeliveryMethod_Item = i8;

pub const PreferredDeliveryMethod_Item_any_delivery_method: PreferredDeliveryMethod_Item = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const PreferredDeliveryMethod_Item_mhs_delivery: PreferredDeliveryMethod_Item = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const PreferredDeliveryMethod_Item_physical_delivery: PreferredDeliveryMethod_Item = 2; /* LONG_NAMED_INTEGER_VALUE */

pub const PreferredDeliveryMethod_Item_telex_delivery: PreferredDeliveryMethod_Item = 3; /* LONG_NAMED_INTEGER_VALUE */

pub const PreferredDeliveryMethod_Item_teletex_delivery: PreferredDeliveryMethod_Item = 4; /* LONG_NAMED_INTEGER_VALUE */

pub const PreferredDeliveryMethod_Item_g3_facsimile_delivery: PreferredDeliveryMethod_Item = 5; /* LONG_NAMED_INTEGER_VALUE */

pub const PreferredDeliveryMethod_Item_g4_facsimile_delivery: PreferredDeliveryMethod_Item = 6; /* LONG_NAMED_INTEGER_VALUE */

pub const PreferredDeliveryMethod_Item_ia5_terminal_delivery: PreferredDeliveryMethod_Item = 7; /* LONG_NAMED_INTEGER_VALUE */

pub const PreferredDeliveryMethod_Item_videotex_delivery: PreferredDeliveryMethod_Item = 8; /* LONG_NAMED_INTEGER_VALUE */

pub const PreferredDeliveryMethod_Item_telephone_delivery: PreferredDeliveryMethod_Item = 9; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_PreferredDeliveryMethod_Item(
    el: &X690Element,
) -> ASN1Result<PreferredDeliveryMethod_Item> {
    BER.decode_i8(&el)
}

pub fn _encode_PreferredDeliveryMethod_Item(
    value_: &PreferredDeliveryMethod_Item,
) -> ASN1Result<X690Element> {
    BER.encode_i8(*value_)
}

pub fn _validate_PreferredDeliveryMethod_Item(el: &X690Element) -> ASN1Result<()> {
    BER.validate_i8(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UiiFormat-subset ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type UiiFormat_subset = ENUMERATED;

pub const UiiFormat_subset_baseObject: UiiFormat_subset = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const UiiFormat_subset_oneLevel: UiiFormat_subset = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const UiiFormat_subset_wholeSubtree: UiiFormat_subset = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_UiiFormat_subset(el: &X690Element) -> ASN1Result<UiiFormat_subset> {
    BER.decode_enumerated(&el)
}

pub fn _encode_UiiFormat_subset(value_: &UiiFormat_subset) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_UiiFormat_subset(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UiiFormat-next ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum UiiFormat_next {
    length(INTEGER),
    filter(UiiFilter),
}

impl TryFrom<&X690Element> for UiiFormat_next {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UiiFormat_next(el)
    }
}

pub fn _decode_UiiFormat_next(el: &X690Element) -> ASN1Result<UiiFormat_next> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 2) => Ok(UiiFormat_next::length(BER.decode_integer(&el)?)),
        (TagClass::CONTEXT, 0) => Ok(UiiFormat_next::filter(_decode_UiiFilter(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(UiiFormat_next::filter(_decode_UiiFilter(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(UiiFormat_next::filter(_decode_UiiFilter(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(UiiFormat_next::filter(_decode_UiiFilter(&el)?)),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "UiiFormat-next",
            ))
        }
    }
}

pub fn _encode_UiiFormat_next(value_: &UiiFormat_next) -> ASN1Result<X690Element> {
    match value_ {
        UiiFormat_next::length(v) => BER.encode_integer(&v),
        UiiFormat_next::filter(v) => _encode_UiiFilter(&v),
    }
}

pub fn _validate_UiiFormat_next(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 2) => BER.validate_integer(&el),
        (TagClass::CONTEXT, 0) => _validate_UiiFilter(&el),
        (TagClass::CONTEXT, 1) => _validate_UiiFilter(&el),
        (TagClass::CONTEXT, 2) => _validate_UiiFilter(&el),
        (TagClass::CONTEXT, 3) => _validate_UiiFilter(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "UiiFormat-next",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EpcFormat-fields-Item-charField ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum EpcFormat_fields_Item_charField {
    characters(INTEGER),
    maxValue(INTEGER),
}

impl TryFrom<&X690Element> for EpcFormat_fields_Item_charField {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_EpcFormat_fields_Item_charField(el)
    }
}

pub fn _decode_EpcFormat_fields_Item_charField(
    el: &X690Element,
) -> ASN1Result<EpcFormat_fields_Item_charField> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(EpcFormat_fields_Item_charField::characters(
            |el: &X690Element| -> ASN1Result<INTEGER> { Ok(BER.decode_integer(&el.inner()?)?) }(
                &el,
            )?,
        )),
        (TagClass::CONTEXT, 1) => Ok(EpcFormat_fields_Item_charField::maxValue(
            |el: &X690Element| -> ASN1Result<INTEGER> { Ok(BER.decode_integer(&el.inner()?)?) }(
                &el,
            )?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "EpcFormat-fields-Item-charField",
            ))
        }
    }
}

pub fn _encode_EpcFormat_fields_Item_charField(
    value_: &EpcFormat_fields_Item_charField,
) -> ASN1Result<X690Element> {
    match value_ {
        EpcFormat_fields_Item_charField::characters(v) => {
            |v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&BER.encode_integer(&v_1)?),
                ))
            }(&v)
        }
        EpcFormat_fields_Item_charField::maxValue(v) => {
            |v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&BER.encode_integer(&v_1)?),
                ))
            }(&v)
        }
    }
}

pub fn _validate_EpcFormat_fields_Item_charField(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "characters"));
            }
            Ok(BER.validate_integer(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "maxValue"));
            }
            Ok(BER.validate_integer(&el.inner()?)?)
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "EpcFormat-fields-Item-charField",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EpcFormat-fields-Item-result ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type EpcFormat_fields_Item_result = ENUMERATED;

pub const EpcFormat_fields_Item_result_numericPad: EpcFormat_fields_Item_result = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const EpcFormat_fields_Item_result_numeric: EpcFormat_fields_Item_result = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const EpcFormat_fields_Item_result_alpha7bits: EpcFormat_fields_Item_result = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_EpcFormat_fields_Item_result(
    el: &X690Element,
) -> ASN1Result<EpcFormat_fields_Item_result> {
    BER.decode_enumerated(&el)
}

pub fn _encode_EpcFormat_fields_Item_result(
    value_: &EpcFormat_fields_Item_result,
) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_EpcFormat_fields_Item_result(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EpcFormat-fields-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct EpcFormat_fields_Item {
    pub bits: INTEGER,
    pub charField: EpcFormat_fields_Item_charField,
    pub result: OPTIONAL<EpcFormat_fields_Item_result>,
}
impl EpcFormat_fields_Item {
    pub fn new(
        bits: INTEGER,
        charField: EpcFormat_fields_Item_charField,
        result: OPTIONAL<EpcFormat_fields_Item_result>,
    ) -> Self {
        EpcFormat_fields_Item {
            bits,
            charField,
            result,
        }
    }
    pub fn _default_value_for_result() -> EpcFormat_fields_Item_result {
        EpcFormat_fields_Item_result_numericPad
    }
}
impl TryFrom<&X690Element> for EpcFormat_fields_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_EpcFormat_fields_Item(el)
    }
}

pub const _rctl1_components_for_EpcFormat_fields_Item: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "bits",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 2)),
        None,
        None,
    ),
    ComponentSpec::new("charField", false, TagSelector::any, None, None),
    ComponentSpec::new(
        "result",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 10)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_EpcFormat_fields_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_EpcFormat_fields_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_EpcFormat_fields_Item(el: &X690Element) -> ASN1Result<EpcFormat_fields_Item> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EpcFormat-fields-Item")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EpcFormat_fields_Item,
        _eal_components_for_EpcFormat_fields_Item,
        _rctl2_components_for_EpcFormat_fields_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut bits_: OPTIONAL<INTEGER> = None;
    let mut charField_: OPTIONAL<EpcFormat_fields_Item_charField> = None;
    let mut result_: OPTIONAL<EpcFormat_fields_Item_result> = None;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "bits" => bits_ = Some(BER.decode_integer(_el)?),
            "charField" => charField_ = Some(_decode_EpcFormat_fields_Item_charField(_el)?),
            "result" => result_ = Some(_decode_EpcFormat_fields_Item_result(_el)?),
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "EpcFormat-fields-Item",
                ))
            }
        }
    }
    Ok(EpcFormat_fields_Item {
        bits: bits_.unwrap(),
        charField: charField_.unwrap(),
        result: result_,
    })
}

pub fn _encode_EpcFormat_fields_Item(value_: &EpcFormat_fields_Item) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(8);
    components_.push(BER.encode_integer(&value_.bits)?);
    components_.push(_encode_EpcFormat_fields_Item_charField(&value_.charField)?);
    if let Some(v_) = &value_.result {
        if *v_ != EpcFormat_fields_Item::_default_value_for_result() {
            components_.push(_encode_EpcFormat_fields_Item_result(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(components_)),
    ))
}

pub fn _validate_EpcFormat_fields_Item(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "EpcFormat-fields-Item")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_EpcFormat_fields_Item,
        _eal_components_for_EpcFormat_fields_Item,
        _rctl2_components_for_EpcFormat_fields_Item,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "bits" => BER.validate_integer(_el)?,
            "charField" => _validate_EpcFormat_fields_Item_charField(_el)?,
            "result" => _validate_EpcFormat_fields_Item_result(_el)?,
            _ => {
                return Err(_el.to_asn1_err_named(
                    ASN1ErrorCode::invalid_construction,
                    "EpcFormat-fields-Item",
                ))
            }
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PwdResponse-warning ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum PwdResponse_warning {
    timeleft(INTEGER),
    graceRemaining(INTEGER),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for PwdResponse_warning {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_PwdResponse_warning(el)
    }
}

pub fn _decode_PwdResponse_warning(el: &X690Element) -> ASN1Result<PwdResponse_warning> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(PwdResponse_warning::timeleft(
            |el: &X690Element| -> ASN1Result<INTEGER> { Ok(BER.decode_integer(&el.inner()?)?) }(
                &el,
            )?,
        )),
        (TagClass::CONTEXT, 1) => Ok(PwdResponse_warning::graceRemaining(
            |el: &X690Element| -> ASN1Result<INTEGER> { Ok(BER.decode_integer(&el.inner()?)?) }(
                &el,
            )?,
        )),
        _ => Ok(PwdResponse_warning::_unrecognized(el.clone())),
    }
}

pub fn _encode_PwdResponse_warning(value_: &PwdResponse_warning) -> ASN1Result<X690Element> {
    match value_ {
        PwdResponse_warning::timeleft(v) => |v_1: &INTEGER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&BER.encode_integer(&v_1)?),
            ))
        }(&v),
        PwdResponse_warning::graceRemaining(v) => |v_1: &INTEGER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&BER.encode_integer(&v_1)?),
            ))
        }(&v),
        PwdResponse_warning::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_PwdResponse_warning(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "timeleft"));
            }
            Ok(BER.validate_integer(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(
                    el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "graceRemaining")
                );
            }
            Ok(BER.validate_integer(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PwdResponse-error ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type PwdResponse_error = ENUMERATED;

pub const PwdResponse_error_passwordExpired: PwdResponse_error = 0; /* LONG_NAMED_ENUMERATED_VALUE */

pub const PwdResponse_error_changeAfterReset: PwdResponse_error = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_PwdResponse_error(el: &X690Element) -> ASN1Result<PwdResponse_error> {
    BER.decode_enumerated(&el)
}

pub fn _encode_PwdResponse_error(value_: &PwdResponse_error) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_PwdResponse_error(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SubstringAssertion-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum SubstringAssertion_Item {
    initial(UnboundedDirectoryString),
    any(UnboundedDirectoryString),
    final_(UnboundedDirectoryString),
    control(Attribute),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for SubstringAssertion_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_SubstringAssertion_Item(el)
    }
}

pub fn _decode_SubstringAssertion_Item(el: &X690Element) -> ASN1Result<SubstringAssertion_Item> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(SubstringAssertion_Item::initial(
            |el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
                Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(SubstringAssertion_Item::any(
            |el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
                Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(SubstringAssertion_Item::final_(
            |el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
                Ok(_decode_UnboundedDirectoryString(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::UNIVERSAL, 16) => Ok(SubstringAssertion_Item::control(_decode_Attribute(&el)?)),
        _ => Ok(SubstringAssertion_Item::_unrecognized(el.clone())),
    }
}

pub fn _encode_SubstringAssertion_Item(
    value_: &SubstringAssertion_Item,
) -> ASN1Result<X690Element> {
    match value_ {
        SubstringAssertion_Item::initial(v) => {
            |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&_encode_UnboundedDirectoryString(&v_1)?),
                ))
            }(&v)
        }
        SubstringAssertion_Item::any(v) => {
            |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 1),
                    X690Value::from_explicit(&_encode_UnboundedDirectoryString(&v_1)?),
                ))
            }(&v)
        }
        SubstringAssertion_Item::final_(v) => {
            |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(&_encode_UnboundedDirectoryString(&v_1)?),
                ))
            }(&v)
        }
        SubstringAssertion_Item::control(v) => _encode_Attribute(&v),
        SubstringAssertion_Item::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_SubstringAssertion_Item(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "initial"));
            }
            Ok(_validate_UnboundedDirectoryString(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "any"));
            }
            Ok(_validate_UnboundedDirectoryString(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "final"));
            }
            Ok(_validate_UnboundedDirectoryString(&el.inner()?)?)
        }(&el),
        (TagClass::UNIVERSAL, 16) => _validate_Attribute(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OctetSubstringAssertion-Item ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum OctetSubstringAssertion_Item {
    initial(OCTET_STRING),
    any(OCTET_STRING),
    final_(OCTET_STRING),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for OctetSubstringAssertion_Item {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_OctetSubstringAssertion_Item(el)
    }
}

pub fn _decode_OctetSubstringAssertion_Item(
    el: &X690Element,
) -> ASN1Result<OctetSubstringAssertion_Item> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => Ok(OctetSubstringAssertion_Item::initial(
            |el: &X690Element| -> ASN1Result<OCTET_STRING> {
                Ok(BER.decode_octet_string(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 1) => Ok(OctetSubstringAssertion_Item::any(
            |el: &X690Element| -> ASN1Result<OCTET_STRING> {
                Ok(BER.decode_octet_string(&el.inner()?)?)
            }(&el)?,
        )),
        (TagClass::CONTEXT, 2) => Ok(OctetSubstringAssertion_Item::final_(
            |el: &X690Element| -> ASN1Result<OCTET_STRING> {
                Ok(BER.decode_octet_string(&el.inner()?)?)
            }(&el)?,
        )),
        _ => Ok(OctetSubstringAssertion_Item::_unrecognized(el.clone())),
    }
}

pub fn _encode_OctetSubstringAssertion_Item(
    value_: &OctetSubstringAssertion_Item,
) -> ASN1Result<X690Element> {
    match value_ {
        OctetSubstringAssertion_Item::initial(v) => {
            |v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 0),
                    X690Value::from_explicit(&BER.encode_octet_string(&v_1)?),
                ))
            }(&v)
        }
        OctetSubstringAssertion_Item::any(v) => |v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&BER.encode_octet_string(&v_1)?),
            ))
        }(&v),
        OctetSubstringAssertion_Item::final_(v) => {
            |v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    Tag::new(TagClass::CONTEXT, 2),
                    X690Value::from_explicit(&BER.encode_octet_string(&v_1)?),
                ))
            }(&v)
        }
        OctetSubstringAssertion_Item::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_OctetSubstringAssertion_Item(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::CONTEXT, 0) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "initial"));
            }
            Ok(BER.validate_octet_string(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 1) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "any"));
            }
            Ok(BER.validate_octet_string(&el.inner()?)?)
        }(&el),
        (TagClass::CONTEXT, 2) => |el: &X690Element| -> ASN1Result<()> {
            if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 2 {
                return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "final"));
            }
            Ok(BER.validate_octet_string(&el.inner()?)?)
        }(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeSpecification-time-absolute ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct TimeSpecification_time_absolute {
    pub startTime: OPTIONAL<GeneralizedTime>,
    pub endTime: OPTIONAL<GeneralizedTime>,
    pub _unrecognized: Vec<X690Element>,
}
impl TimeSpecification_time_absolute {
    pub fn new(
        startTime: OPTIONAL<GeneralizedTime>,
        endTime: OPTIONAL<GeneralizedTime>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TimeSpecification_time_absolute {
            startTime,
            endTime,
            _unrecognized,
        }
    }
}
impl Default for TimeSpecification_time_absolute {
    fn default() -> Self {
        TimeSpecification_time_absolute {
            startTime: None,
            endTime: None,
            _unrecognized: vec![],
        }
    }
}
impl TryFrom<&X690Element> for TimeSpecification_time_absolute {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimeSpecification_time_absolute(el)
    }
}

pub const _rctl1_components_for_TimeSpecification_time_absolute: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "startTime",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "endTime",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TimeSpecification_time_absolute: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TimeSpecification_time_absolute: &[ComponentSpec; 0] = &[];

pub fn _decode_TimeSpecification_time_absolute(
    el: &X690Element,
) -> ASN1Result<TimeSpecification_time_absolute> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "TimeSpecification-time-absolute",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeSpecification_time_absolute,
        _eal_components_for_TimeSpecification_time_absolute,
        _rctl2_components_for_TimeSpecification_time_absolute,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut startTime_: OPTIONAL<GeneralizedTime> = None;
    let mut endTime_: OPTIONAL<GeneralizedTime> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "startTime" => {
                startTime_ = Some(|el: &X690Element| -> ASN1Result<GeneralizedTime> {
                    Ok(BER.decode_generalized_time(&el.inner()?)?)
                }(_el)?)
            }
            "endTime" => {
                endTime_ = Some(|el: &X690Element| -> ASN1Result<GeneralizedTime> {
                    Ok(BER.decode_generalized_time(&el.inner()?)?)
                }(_el)?)
            }
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TimeSpecification_time_absolute {
        startTime: startTime_,
        endTime: endTime_,
        _unrecognized,
    })
}

pub fn _encode_TimeSpecification_time_absolute(
    value_: &TimeSpecification_time_absolute,
) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    if let Some(v_) = &value_.startTime {
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::from_explicit(&BER.encode_generalized_time(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.endTime {
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&BER.encode_generalized_time(&v_1)?),
            ))
        }(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_TimeSpecification_time_absolute(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::invalid_construction,
                "TimeSpecification-time-absolute",
            ))
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeSpecification_time_absolute,
        _eal_components_for_TimeSpecification_time_absolute,
        _rctl2_components_for_TimeSpecification_time_absolute,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "startTime" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "startTime")
                    );
                }
                Ok(BER.validate_generalized_time(&el.inner()?)?)
            }(_el)?,
            "endTime" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "endTime")
                    );
                }
                Ok(BER.validate_generalized_time(&el.inner()?)?)
            }(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeSpecification-time ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TimeSpecification_time {
    absolute(TimeSpecification_time_absolute),
    periodic(Vec<Period>),
}

impl TryFrom<&X690Element> for TimeSpecification_time {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimeSpecification_time(el)
    }
}

pub fn _decode_TimeSpecification_time(el: &X690Element) -> ASN1Result<TimeSpecification_time> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => Ok(TimeSpecification_time::absolute(
            _decode_TimeSpecification_time_absolute(&el)?,
        )),
        (TagClass::UNIVERSAL, 17) => Ok(TimeSpecification_time::periodic(
            |el: &X690Element| -> ASN1Result<SET_OF<Period>> {
                let elements = match &el.value {
                    X690Value::Constructed(children) => children,
                    _ => {
                        return Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "periodic")
                        )
                    }
                };
                let mut items: SET_OF<Period> = Vec::with_capacity(elements.len());
                for el in elements.iter() {
                    items.push(_decode_Period(el)?);
                }
                Ok(items)
            }(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "TimeSpecification-time",
            ))
        }
    }
}

pub fn _encode_TimeSpecification_time(value_: &TimeSpecification_time) -> ASN1Result<X690Element> {
    match value_ {
        TimeSpecification_time::absolute(v) => _encode_TimeSpecification_time_absolute(&v),
        TimeSpecification_time::periodic(v) => {
            |value_: &SET_OF<Period>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_Period(&v)?);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                    X690Value::Constructed(Arc::new(children)),
                ))
            }(&v)
        }
    }
}

pub fn _validate_TimeSpecification_time(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 16) => _validate_TimeSpecification_time_absolute(&el),
        (TagClass::UNIVERSAL, 17) => |el: &X690Element| -> ASN1Result<()> {
            match &el.value {
                X690Value::Constructed(subs) => {
                    for sub in subs.iter() {
                        _validate_Period(&sub)?;
                    }
                    Ok(())
                }
                _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "periodic")),
            }
        }(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "TimeSpecification-time",
            ))
        }
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Period-days-bitDay ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type Period_days_bitDay = BIT_STRING;

pub const Period_days_bitDay_sunday: BIT = 0; /* LONG_NAMED_BIT */

pub const Period_days_bitDay_monday: BIT = 1; /* LONG_NAMED_BIT */

pub const Period_days_bitDay_tuesday: BIT = 2; /* LONG_NAMED_BIT */

pub const Period_days_bitDay_wednesday: BIT = 3; /* LONG_NAMED_BIT */

pub const Period_days_bitDay_thursday: BIT = 4; /* LONG_NAMED_BIT */

pub const Period_days_bitDay_friday: BIT = 5; /* LONG_NAMED_BIT */

pub const Period_days_bitDay_saturday: BIT = 6; /* LONG_NAMED_BIT */

pub fn _decode_Period_days_bitDay(el: &X690Element) -> ASN1Result<Period_days_bitDay> {
    BER.decode_bit_string(&el)
}

pub fn _encode_Period_days_bitDay(value_: &Period_days_bitDay) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_Period_days_bitDay(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Period-days ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum Period_days {
    intDay(Vec<INTEGER>),
    bitDay(Period_days_bitDay),
    dayOf(XDayOf),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for Period_days {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Period_days(el)
    }
}

pub fn _decode_Period_days(el: &X690Element) -> ASN1Result<Period_days> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 17) => Ok(Period_days::intDay(
            |el: &X690Element| -> ASN1Result<SET_OF<INTEGER>> {
                let elements = match &el.value {
                    X690Value::Constructed(children) => children,
                    _ => {
                        return Err(
                            el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "intDay")
                        )
                    }
                };
                let mut items: SET_OF<INTEGER> = Vec::with_capacity(elements.len());
                for el in elements.iter() {
                    items.push(BER.decode_integer(el)?);
                }
                Ok(items)
            }(&el)?,
        )),
        (TagClass::UNIVERSAL, 3) => Ok(Period_days::bitDay(_decode_Period_days_bitDay(&el)?)),
        (TagClass::CONTEXT, 1) => Ok(Period_days::dayOf(_decode_XDayOf(&el)?)),
        (TagClass::CONTEXT, 2) => Ok(Period_days::dayOf(_decode_XDayOf(&el)?)),
        (TagClass::CONTEXT, 3) => Ok(Period_days::dayOf(_decode_XDayOf(&el)?)),
        (TagClass::CONTEXT, 4) => Ok(Period_days::dayOf(_decode_XDayOf(&el)?)),
        (TagClass::CONTEXT, 5) => Ok(Period_days::dayOf(_decode_XDayOf(&el)?)),
        _ => Ok(Period_days::_unrecognized(el.clone())),
    }
}

pub fn _encode_Period_days(value_: &Period_days) -> ASN1Result<X690Element> {
    match value_ {
        Period_days::intDay(v) => |value_: &SET_OF<INTEGER>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(BER.encode_integer(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&v),
        Period_days::bitDay(v) => _encode_Period_days_bitDay(&v),
        Period_days::dayOf(v) => _encode_XDayOf(&v),
        Period_days::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_Period_days(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 17) => |el: &X690Element| -> ASN1Result<()> {
            match &el.value {
                X690Value::Constructed(subs) => {
                    for sub in subs.iter() {
                        BER.validate_integer(&sub)?;
                    }
                    Ok(())
                }
                _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "intDay")),
            }
        }(&el),
        (TagClass::UNIVERSAL, 3) => _validate_Period_days_bitDay(&el),
        (TagClass::CONTEXT, 1) => _validate_XDayOf(&el),
        (TagClass::CONTEXT, 2) => _validate_XDayOf(&el),
        (TagClass::CONTEXT, 3) => _validate_XDayOf(&el),
        (TagClass::CONTEXT, 4) => _validate_XDayOf(&el),
        (TagClass::CONTEXT, 5) => _validate_XDayOf(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Period-weeks-bitWeek ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type Period_weeks_bitWeek = BIT_STRING;

pub const Period_weeks_bitWeek_week1: BIT = 0; /* LONG_NAMED_BIT */

pub const Period_weeks_bitWeek_week2: BIT = 1; /* LONG_NAMED_BIT */

pub const Period_weeks_bitWeek_week3: BIT = 2; /* LONG_NAMED_BIT */

pub const Period_weeks_bitWeek_week4: BIT = 3; /* LONG_NAMED_BIT */

pub const Period_weeks_bitWeek_week5: BIT = 4; /* LONG_NAMED_BIT */

pub fn _decode_Period_weeks_bitWeek(el: &X690Element) -> ASN1Result<Period_weeks_bitWeek> {
    BER.decode_bit_string(&el)
}

pub fn _encode_Period_weeks_bitWeek(value_: &Period_weeks_bitWeek) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_Period_weeks_bitWeek(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Period-weeks ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum Period_weeks {
    allWeeks(NULL),
    intWeek(Vec<INTEGER>),
    bitWeek(Period_weeks_bitWeek),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for Period_weeks {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Period_weeks(el)
    }
}

pub fn _decode_Period_weeks(el: &X690Element) -> ASN1Result<Period_weeks> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 5) => Ok(Period_weeks::allWeeks(BER.decode_null(&el)?)),
        (TagClass::UNIVERSAL, 17) => Ok(Period_weeks::intWeek(|el: &X690Element| -> ASN1Result<
            SET_OF<INTEGER>,
        > {
            let elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "intWeek"))
                }
            };
            let mut items: SET_OF<INTEGER> = Vec::with_capacity(elements.len());
            for el in elements.iter() {
                items.push(BER.decode_integer(el)?);
            }
            Ok(items)
        }(&el)?)),
        (TagClass::UNIVERSAL, 3) => Ok(Period_weeks::bitWeek(_decode_Period_weeks_bitWeek(&el)?)),
        _ => Ok(Period_weeks::_unrecognized(el.clone())),
    }
}

pub fn _encode_Period_weeks(value_: &Period_weeks) -> ASN1Result<X690Element> {
    match value_ {
        Period_weeks::allWeeks(v) => BER.encode_null(&v),
        Period_weeks::intWeek(v) => |value_: &SET_OF<INTEGER>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(BER.encode_integer(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&v),
        Period_weeks::bitWeek(v) => _encode_Period_weeks_bitWeek(&v),
        Period_weeks::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_Period_weeks(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 5) => BER.validate_null(&el),
        (TagClass::UNIVERSAL, 17) => |el: &X690Element| -> ASN1Result<()> {
            match &el.value {
                X690Value::Constructed(subs) => {
                    for sub in subs.iter() {
                        BER.validate_integer(&sub)?;
                    }
                    Ok(())
                }
                _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "intWeek")),
            }
        }(&el),
        (TagClass::UNIVERSAL, 3) => _validate_Period_weeks_bitWeek(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Period-months-bitMonth ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type Period_months_bitMonth = BIT_STRING;

pub const Period_months_bitMonth_january: BIT = 0; /* LONG_NAMED_BIT */

pub const Period_months_bitMonth_february: BIT = 1; /* LONG_NAMED_BIT */

pub const Period_months_bitMonth_march: BIT = 2; /* LONG_NAMED_BIT */

pub const Period_months_bitMonth_april: BIT = 3; /* LONG_NAMED_BIT */

pub const Period_months_bitMonth_may: BIT = 4; /* LONG_NAMED_BIT */

pub const Period_months_bitMonth_june: BIT = 5; /* LONG_NAMED_BIT */

pub const Period_months_bitMonth_july: BIT = 6; /* LONG_NAMED_BIT */

pub const Period_months_bitMonth_august: BIT = 7; /* LONG_NAMED_BIT */

pub const Period_months_bitMonth_september: BIT = 8; /* LONG_NAMED_BIT */

pub const Period_months_bitMonth_october: BIT = 9; /* LONG_NAMED_BIT */

pub const Period_months_bitMonth_november: BIT = 10; /* LONG_NAMED_BIT */

pub const Period_months_bitMonth_december: BIT = 11; /* LONG_NAMED_BIT */

pub fn _decode_Period_months_bitMonth(el: &X690Element) -> ASN1Result<Period_months_bitMonth> {
    BER.decode_bit_string(&el)
}

pub fn _encode_Period_months_bitMonth(value_: &Period_months_bitMonth) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_Period_months_bitMonth(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// Period-months ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum Period_months {
    allMonths(NULL),
    intMonth(Vec<INTEGER>),
    bitMonth(Period_months_bitMonth),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for Period_months {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_Period_months(el)
    }
}

pub fn _decode_Period_months(el: &X690Element) -> ASN1Result<Period_months> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 5) => Ok(Period_months::allMonths(BER.decode_null(&el)?)),
        (TagClass::UNIVERSAL, 17) => Ok(Period_months::intMonth(|el: &X690Element| -> ASN1Result<
            SET_OF<INTEGER>,
        > {
            let elements = match &el.value {
                X690Value::Constructed(children) => children,
                _ => {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "intMonth")
                    )
                }
            };
            let mut items: SET_OF<INTEGER> = Vec::with_capacity(elements.len());
            for el in elements.iter() {
                items.push(BER.decode_integer(el)?);
            }
            Ok(items)
        }(&el)?)),
        (TagClass::UNIVERSAL, 3) => Ok(Period_months::bitMonth(_decode_Period_months_bitMonth(
            &el,
        )?)),
        _ => Ok(Period_months::_unrecognized(el.clone())),
    }
}

pub fn _encode_Period_months(value_: &Period_months) -> ASN1Result<X690Element> {
    match value_ {
        Period_months::allMonths(v) => BER.encode_null(&v),
        Period_months::intMonth(v) => |value_: &SET_OF<INTEGER>| -> ASN1Result<X690Element> {
            let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
            for v in value_ {
                children.push(BER.encode_integer(&v)?);
            }
            Ok(X690Element::new(
                Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SET_OF),
                X690Value::Constructed(Arc::new(children)),
            ))
        }(&v),
        Period_months::bitMonth(v) => _encode_Period_months_bitMonth(&v),
        Period_months::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_Period_months(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 5) => BER.validate_null(&el),
        (TagClass::UNIVERSAL, 17) => |el: &X690Element| -> ASN1Result<()> {
            match &el.value {
                X690Value::Constructed(subs) => {
                    for sub in subs.iter() {
                        BER.validate_integer(&sub)?;
                    }
                    Ok(())
                }
                _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "intMonth")),
            }
        }(&el),
        (TagClass::UNIVERSAL, 3) => _validate_Period_months_bitMonth(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedDay-intNamedDays ::= ENUMERATED { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type NamedDay_intNamedDays = ENUMERATED;

pub const NamedDay_intNamedDays_sunday: NamedDay_intNamedDays = 1; /* LONG_NAMED_ENUMERATED_VALUE */

pub const NamedDay_intNamedDays_monday: NamedDay_intNamedDays = 2; /* LONG_NAMED_ENUMERATED_VALUE */

pub const NamedDay_intNamedDays_tuesday: NamedDay_intNamedDays = 3; /* LONG_NAMED_ENUMERATED_VALUE */

pub const NamedDay_intNamedDays_wednesday: NamedDay_intNamedDays = 4; /* LONG_NAMED_ENUMERATED_VALUE */

pub const NamedDay_intNamedDays_thursday: NamedDay_intNamedDays = 5; /* LONG_NAMED_ENUMERATED_VALUE */

pub const NamedDay_intNamedDays_friday: NamedDay_intNamedDays = 6; /* LONG_NAMED_ENUMERATED_VALUE */

pub const NamedDay_intNamedDays_saturday: NamedDay_intNamedDays = 7; /* LONG_NAMED_ENUMERATED_VALUE */

pub fn _decode_NamedDay_intNamedDays(el: &X690Element) -> ASN1Result<NamedDay_intNamedDays> {
    BER.decode_enumerated(&el)
}

pub fn _encode_NamedDay_intNamedDays(value_: &NamedDay_intNamedDays) -> ASN1Result<X690Element> {
    BER.encode_enumerated(&value_)
}

pub fn _validate_NamedDay_intNamedDays(el: &X690Element) -> ASN1Result<()> {
    BER.validate_enumerated(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// NamedDay-bitNamedDays ::= BIT STRING { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type NamedDay_bitNamedDays = BIT_STRING;

pub const NamedDay_bitNamedDays_sunday: BIT = 0; /* LONG_NAMED_BIT */

pub const NamedDay_bitNamedDays_monday: BIT = 1; /* LONG_NAMED_BIT */

pub const NamedDay_bitNamedDays_tuesday: BIT = 2; /* LONG_NAMED_BIT */

pub const NamedDay_bitNamedDays_wednesday: BIT = 3; /* LONG_NAMED_BIT */

pub const NamedDay_bitNamedDays_thursday: BIT = 4; /* LONG_NAMED_BIT */

pub const NamedDay_bitNamedDays_friday: BIT = 5; /* LONG_NAMED_BIT */

pub const NamedDay_bitNamedDays_saturday: BIT = 6; /* LONG_NAMED_BIT */

pub fn _decode_NamedDay_bitNamedDays(el: &X690Element) -> ASN1Result<NamedDay_bitNamedDays> {
    BER.decode_bit_string(&el)
}

pub fn _encode_NamedDay_bitNamedDays(value_: &NamedDay_bitNamedDays) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_NamedDay_bitNamedDays(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeAssertion-between ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct TimeAssertion_between {
    pub startTime: GeneralizedTime,
    pub endTime: OPTIONAL<GeneralizedTime>,
    pub entirely: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl TimeAssertion_between {
    pub fn new(
        startTime: GeneralizedTime,
        endTime: OPTIONAL<GeneralizedTime>,
        entirely: OPTIONAL<BOOLEAN>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        TimeAssertion_between {
            startTime,
            endTime,
            entirely,
            _unrecognized,
        }
    }
    pub fn _default_value_for_entirely() -> BOOLEAN {
        false
    }
}
impl TryFrom<&X690Element> for TimeAssertion_between {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_TimeAssertion_between(el)
    }
}

pub const _rctl1_components_for_TimeAssertion_between: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "startTime",
        false,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "endTime",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "entirely",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, 1)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_TimeAssertion_between: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_TimeAssertion_between: &[ComponentSpec; 0] = &[];

pub fn _decode_TimeAssertion_between(el: &X690Element) -> ASN1Result<TimeAssertion_between> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeAssertion-between")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeAssertion_between,
        _eal_components_for_TimeAssertion_between,
        _rctl2_components_for_TimeAssertion_between,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut startTime_: OPTIONAL<GeneralizedTime> = None;
    let mut endTime_: OPTIONAL<GeneralizedTime> = None;
    let mut entirely_: OPTIONAL<BOOLEAN> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "startTime" => {
                startTime_ = Some(|el: &X690Element| -> ASN1Result<GeneralizedTime> {
                    Ok(BER.decode_generalized_time(&el.inner()?)?)
                }(_el)?)
            }
            "endTime" => {
                endTime_ = Some(|el: &X690Element| -> ASN1Result<GeneralizedTime> {
                    Ok(BER.decode_generalized_time(&el.inner()?)?)
                }(_el)?)
            }
            "entirely" => entirely_ = Some(BER.decode_boolean(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(TimeAssertion_between {
        startTime: startTime_.unwrap(),
        endTime: endTime_,
        entirely: entirely_,
        _unrecognized,
    })
}

pub fn _encode_TimeAssertion_between(value_: &TimeAssertion_between) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(13);
    components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::CONTEXT, 0),
            X690Value::from_explicit(&BER.encode_generalized_time(&v_1)?),
        ))
    }(&value_.startTime)?);
    if let Some(v_) = &value_.endTime {
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::from_explicit(&BER.encode_generalized_time(&v_1)?),
            ))
        }(&v_)?);
    }
    if let Some(v_) = &value_.entirely {
        if *v_ != TimeAssertion_between::_default_value_for_entirely() {
            components_.push(BER.encode_boolean(&v_)?);
        }
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_TimeAssertion_between(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "TimeAssertion-between")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_TimeAssertion_between,
        _eal_components_for_TimeAssertion_between,
        _rctl2_components_for_TimeAssertion_between,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "startTime" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 0 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "startTime")
                    );
                }
                Ok(BER.validate_generalized_time(&el.inner()?)?)
            }(_el)?,
            "endTime" => |el: &X690Element| -> ASN1Result<()> {
                if el.tag.tag_class != TagClass::CONTEXT || el.tag.tag_number != 1 {
                    return Err(
                        el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "endTime")
                    );
                }
                Ok(BER.validate_generalized_time(&el.inner()?)?)
            }(_el)?,
            "entirely" => BER.validate_boolean(_el)?,
            _ => (),
        }
    }
    Ok(())
}
