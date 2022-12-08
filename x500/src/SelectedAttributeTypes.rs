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
use crate::AuthenticationFramework::*;
use crate::DirectoryAbstractService::*;
use crate::InformationFramework::*;
use crate::PasswordPolicy::*;
use crate::PkiPmiExternalDataTypes::*;
use crate::SchemaAdministration::*;
use crate::ServiceAdministration::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::borrow::Borrow;
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

impl TryFrom<X690Element> for UnboundedDirectoryString {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UnboundedDirectoryString(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UnboundedDirectoryString {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UnboundedDirectoryString(el)
    }
}

pub fn _decode_UnboundedDirectoryString(el: &X690Element) -> ASN1Result<UnboundedDirectoryString> {
    |el: &X690Element| -> ASN1Result<UnboundedDirectoryString> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 20) => Ok(UnboundedDirectoryString::teletexString(
                ber_decode_t61_string(&el)?,
            )),
            (TagClass::UNIVERSAL, 19) => Ok(UnboundedDirectoryString::printableString(
                ber_decode_printable_string(&el)?,
            )),
            (TagClass::UNIVERSAL, 30) => Ok(UnboundedDirectoryString::bmpString(
                ber_decode_bmp_string(&el)?,
            )),
            (TagClass::UNIVERSAL, 28) => Ok(UnboundedDirectoryString::universalString(
                ber_decode_universal_string(&el)?,
            )),
            (TagClass::UNIVERSAL, 12) => Ok(UnboundedDirectoryString::uTF8String(
                ber_decode_utf8_string(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_UnboundedDirectoryString(
    value_: &UnboundedDirectoryString,
) -> ASN1Result<X690Element> {
    |value: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
        match value {
            UnboundedDirectoryString::teletexString(v) => ber_encode_t61_string(&v),
            UnboundedDirectoryString::printableString(v) => ber_encode_printable_string(&v),
            UnboundedDirectoryString::bmpString(v) => ber_encode_bmp_string(&v),
            UnboundedDirectoryString::universalString(v) => ber_encode_universal_string(&v),
            UnboundedDirectoryString::uTF8String(v) => ber_encode_utf8_string(&v),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&value_)
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

impl TryFrom<X690Element> for DirectoryString {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryString(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DirectoryString {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DirectoryString(el)
    }
}

pub fn _decode_DirectoryString(el: &X690Element) -> ASN1Result<DirectoryString> {
    |el: &X690Element| -> ASN1Result<DirectoryString> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 20) => {
                Ok(DirectoryString::teletexString(ber_decode_t61_string(&el)?))
            }
            (TagClass::UNIVERSAL, 19) => Ok(DirectoryString::printableString(
                ber_decode_printable_string(&el)?,
            )),
            (TagClass::UNIVERSAL, 30) => {
                Ok(DirectoryString::bmpString(ber_decode_bmp_string(&el)?))
            }
            (TagClass::UNIVERSAL, 28) => Ok(DirectoryString::universalString(
                ber_decode_universal_string(&el)?,
            )),
            (TagClass::UNIVERSAL, 12) => {
                Ok(DirectoryString::uTF8String(ber_decode_utf8_string(&el)?))
            }
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_DirectoryString(value_: &DirectoryString) -> ASN1Result<X690Element> {
    |value: &DirectoryString| -> ASN1Result<X690Element> {
        match value {
            DirectoryString::teletexString(v) => ber_encode_t61_string(&v),
            DirectoryString::printableString(v) => ber_encode_printable_string(&v),
            DirectoryString::bmpString(v) => ber_encode_bmp_string(&v),
            DirectoryString::universalString(v) => ber_encode_universal_string(&v),
            DirectoryString::uTF8String(v) => ber_encode_utf8_string(&v),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// UniqueIdentifier  ::=  BIT STRING
/// ```
pub type UniqueIdentifier = BIT_STRING;

pub fn _decode_UniqueIdentifier(el: &X690Element) -> ASN1Result<UniqueIdentifier> {
    ber_decode_bit_string(&el)
}

pub fn _encode_UniqueIdentifier(value_: &UniqueIdentifier) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// UUIDPair ::= SEQUENCE {
///   issuerUUID   UUID,
///   subjectUUID  UUID,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UUIDPair {
    pub issuerUUID: UUID,
    pub subjectUUID: UUID,
    pub _unrecognized: Vec<X690Element>,
}
impl UUIDPair {
    fn new(issuerUUID: UUID, subjectUUID: UUID, _unrecognized: Vec<X690Element>) -> Self {
        UUIDPair {
            issuerUUID,
            subjectUUID,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for UUIDPair {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UUIDPair(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UUIDPair {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<UUIDPair> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UUIDPair,
            _eal_components_for_UUIDPair,
            _rctl2_components_for_UUIDPair,
        )?;
        let issuerUUID = _decode_UUID(_components.get("issuerUUID").unwrap())?;
        let subjectUUID = _decode_UUID(_components.get("subjectUUID").unwrap())?;
        Ok(UUIDPair {
            issuerUUID,
            subjectUUID,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_UUIDPair(value_: &UUIDPair) -> ASN1Result<X690Element> {
    |value_: &UUIDPair| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_UUID(&value_.issuerUUID)?);
        components_.push(_encode_UUID(&value_.subjectUUID)?);
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
/// UUID  ::=  OCTET STRING(SIZE (16))
/// ```
pub type UUID = OCTET_STRING; // OctetStringType

pub fn _decode_UUID(el: &X690Element) -> ASN1Result<UUID> {
    ber_decode_octet_string(&el)
}

pub fn _encode_UUID(value_: &UUID) -> ASN1Result<X690Element> {
    ber_encode_octet_string(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// URI  ::=  UTF8String
/// ```
pub type URI = UTF8String; // UTF8String

pub fn _decode_URI(el: &X690Element) -> ASN1Result<URI> {
    ber_decode_utf8_string(&el)
}

pub fn _encode_URI(value_: &URI) -> ASN1Result<X690Element> {
    ber_encode_utf8_string(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// DomainName  ::=  UTF8String (CONSTRAINED BY { -- Conforms to the format of a domain name. -- })
/// ```
pub type DomainName = UTF8String; // UTF8String

pub fn _decode_DomainName(el: &X690Element) -> ASN1Result<DomainName> {
    ber_decode_utf8_string(&el)
}

pub fn _encode_DomainName(value_: &DomainName) -> ASN1Result<X690Element> {
    ber_encode_utf8_string(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// IntEmail  ::=  UTF8String (CONSTRAINED BY { -- Conforms to the format of an (internationalized) email address. -- })
/// ```
pub type IntEmail = UTF8String; // UTF8String

pub fn _decode_IntEmail(el: &X690Element) -> ASN1Result<IntEmail> {
    ber_decode_utf8_string(&el)
}

pub fn _encode_IntEmail(value_: &IntEmail) -> ASN1Result<X690Element> {
    ber_encode_utf8_string(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// Jid  ::=  UTF8String (CONSTRAINED BY { /* Conforms to the format of a jabber identifier. */ })
/// ```
pub type Jid = UTF8String; // UTF8String

pub fn _decode_Jid(el: &X690Element) -> ASN1Result<Jid> {
    ber_decode_utf8_string(&el)
}

pub fn _encode_Jid(value_: &Jid) -> ASN1Result<X690Element> {
    ber_encode_utf8_string(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// CountryName  ::=  PrintableString(SIZE (2)) (CONSTRAINED BY { -- ISO 3166 alpha-2 codes only -- })
/// ```
pub type CountryName = PrintableString; // PrintableString

pub fn _decode_CountryName(el: &X690Element) -> ASN1Result<CountryName> {
    ber_decode_printable_string(&el)
}

pub fn _encode_CountryName(value_: &CountryName) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// CountryCode3c  ::=  PrintableString(SIZE (3)) (CONSTRAINED BY { -- ISO 3166 alpha-3 codes only -- })
/// ```
pub type CountryCode3c = PrintableString; // PrintableString

pub fn _decode_CountryCode3c(el: &X690Element) -> ASN1Result<CountryCode3c> {
    ber_decode_printable_string(&el)
}

pub fn _encode_CountryCode3c(value_: &CountryCode3c) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// CountryCode3n  ::=  NumericString(SIZE (3)) (CONSTRAINED BY { -- ISO 3166 numeric-3 codes only -- })
/// ```
pub type CountryCode3n = NumericString; // NumericString

pub fn _decode_CountryCode3n(el: &X690Element) -> ASN1Result<CountryCode3n> {
    ber_decode_numeric_string(&el)
}

pub fn _encode_CountryCode3n(value_: &CountryCode3n) -> ASN1Result<X690Element> {
    ber_encode_numeric_string(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// UtmCoordinates ::= SEQUENCE {
///   zone      PrintableString,
///   easting   NumericString,
///   northing  NumericString }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UtmCoordinates {
    pub zone: PrintableString,
    pub easting: NumericString,
    pub northing: NumericString,
}
impl UtmCoordinates {
    fn new(zone: PrintableString, easting: NumericString, northing: NumericString) -> Self {
        UtmCoordinates {
            zone,
            easting,
            northing,
        }
    }
}
impl TryFrom<X690Element> for UtmCoordinates {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UtmCoordinates(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UtmCoordinates {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<UtmCoordinates> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UtmCoordinates,
            _eal_components_for_UtmCoordinates,
            _rctl2_components_for_UtmCoordinates,
        )?;
        let zone = ber_decode_printable_string(_components.get("zone").unwrap())?;
        let easting = ber_decode_numeric_string(_components.get("easting").unwrap())?;
        let northing = ber_decode_numeric_string(_components.get("northing").unwrap())?;
        Ok(UtmCoordinates {
            zone,
            easting,
            northing,
        })
    }(&el)
}

pub fn _encode_UtmCoordinates(value_: &UtmCoordinates) -> ASN1Result<X690Element> {
    |value_: &UtmCoordinates| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(ber_encode_printable_string(&value_.zone)?);
        components_.push(ber_encode_numeric_string(&value_.easting)?);
        components_.push(ber_encode_numeric_string(&value_.northing)?);
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// Guide ::= SET {
///   objectClass  [0]  OBJECT-CLASS.&id OPTIONAL,
///   criteria     [1]  Criteria,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Guide {
    pub objectClass: OPTIONAL<OBJECT_IDENTIFIER>,
    pub criteria: Criteria,
    pub _unrecognized: Vec<X690Element>,
}
impl Guide {
    fn new(
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
impl TryFrom<X690Element> for Guide {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Guide(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Guide {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Guide> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_set(
            el_refs_.as_slice(),
            _rctl1_components_for_Guide,
            _eal_components_for_Guide,
            _rctl2_components_for_Guide,
            30,
        )?;
        let objectClass_: OPTIONAL<OBJECT_IDENTIFIER> = match _components.get("objectClass") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
                Ok(ber_decode_object_identifier(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let criteria =
            |el: &X690Element| -> ASN1Result<Criteria> { Ok(_decode_Criteria(&el.inner()?)?) }(
                _components.get("criteria").unwrap(),
            )?;
        Ok(Guide {
            objectClass: objectClass_,
            criteria,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Guide(value_: &Guide) -> ASN1Result<X690Element> {
    |value_: &Guide| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.objectClass {
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
        components_.push(|v_1: &Criteria| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Criteria(&v_1)?))),
            ))
        }(&value_.criteria)?);
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
/// Criteria  ::=  CHOICE {
///   type  [0]  CriteriaItem,
///   and   [1]  SET OF Criteria,
///   or    [2]  SET OF Criteria,
///   not   [3]  Criteria,
///   ... }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Criteria {
    type_(CriteriaItem),
    and(Vec<Box<Criteria>>),
    or(Vec<Box<Criteria>>),
    not(Box<Criteria>),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for Criteria {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Criteria(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Criteria {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Criteria(el)
    }
}

pub fn _decode_Criteria(el: &X690Element) -> ASN1Result<Criteria> {
    |el: &X690Element| -> ASN1Result<Criteria> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(Criteria::type_(
                |el: &X690Element| -> ASN1Result<CriteriaItem> {
                    Ok(_decode_CriteriaItem(&el.inner()?)?)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(Criteria::and(
                |el: &X690Element| -> ASN1Result<SET_OF<Box<Criteria>>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<Box<Criteria>> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(Box::new(_decode_Criteria(el)?));
                    }
                    Ok(items)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 2) => Ok(Criteria::or(
                |el: &X690Element| -> ASN1Result<SET_OF<Box<Criteria>>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<Box<Criteria>> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(Box::new(_decode_Criteria(el)?));
                    }
                    Ok(items)
                }(&el)?,
            )),
            (TagClass::CONTEXT, 3) => Ok(Criteria::not(
                |el: &X690Element| -> ASN1Result<Box<Criteria>> {
                    Ok(Box::new(_decode_Criteria(&el.inner()?)?))
                }(&el)?,
            )),
            _ => Ok(Criteria::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Criteria(value_: &Criteria) -> ASN1Result<X690Element> {
    |value: &Criteria| -> ASN1Result<X690Element> {
        match value {
            Criteria::type_(v) => |v_1: &CriteriaItem| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_CriteriaItem(
                        &v_1,
                    )?))),
                ))
            }(&v),
            Criteria::and(v) => |v_1: &Vec<Box<Criteria>>| -> ASN1Result<X690Element> {
                let mut el_1 = |value_: &SET_OF<Box<Criteria>>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_Criteria(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            Criteria::or(v) => |v_1: &Vec<Box<Criteria>>| -> ASN1Result<X690Element> {
                let mut el_1 = |value_: &SET_OF<Box<Criteria>>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_Criteria(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v),
            Criteria::not(v) => |v_1: &Criteria| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Criteria(&v_1)?))),
                ))
            }(&v),
            Criteria::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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
#[derive(Debug, Clone, PartialEq)]
pub enum CriteriaItem {
    equality(AttributeType),
    substrings(AttributeType),
    greaterOrEqual(AttributeType),
    lessOrEqual(AttributeType),
    approximateMatch(AttributeType),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for CriteriaItem {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_CriteriaItem(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for CriteriaItem {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_CriteriaItem(el)
    }
}

pub fn _decode_CriteriaItem(el: &X690Element) -> ASN1Result<CriteriaItem> {
    |el: &X690Element| -> ASN1Result<CriteriaItem> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(CriteriaItem::equality(_decode_AttributeType(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(CriteriaItem::substrings(_decode_AttributeType(&el)?)),
            (TagClass::CONTEXT, 2) => Ok(CriteriaItem::greaterOrEqual(_decode_AttributeType(&el)?)),
            (TagClass::CONTEXT, 3) => Ok(CriteriaItem::lessOrEqual(_decode_AttributeType(&el)?)),
            (TagClass::CONTEXT, 4) => {
                Ok(CriteriaItem::approximateMatch(_decode_AttributeType(&el)?))
            }
            _ => Ok(CriteriaItem::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_CriteriaItem(value_: &CriteriaItem) -> ASN1Result<X690Element> {
    |value: &CriteriaItem| -> ASN1Result<X690Element> {
        match value {
            CriteriaItem::equality(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeType(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            CriteriaItem::substrings(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeType(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            CriteriaItem::greaterOrEqual(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeType(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v),
            CriteriaItem::lessOrEqual(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeType(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 3;
                Ok(el_1)
            }(&v),
            CriteriaItem::approximateMatch(v) => |v_1: &AttributeType| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_AttributeType(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 4;
                Ok(el_1)
            }(&v),
            CriteriaItem::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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
///
#[derive(Debug, Clone)]
pub struct EnhancedGuide {
    pub objectClass: OBJECT_IDENTIFIER,
    pub criteria: Criteria,
    pub subset: OPTIONAL<EnhancedGuide_subset>,
    pub _unrecognized: Vec<X690Element>,
}
impl EnhancedGuide {
    fn new(
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
impl TryFrom<X690Element> for EnhancedGuide {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EnhancedGuide(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EnhancedGuide {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<EnhancedGuide> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EnhancedGuide,
            _eal_components_for_EnhancedGuide,
            _rctl2_components_for_EnhancedGuide,
        )?;
        let objectClass_ = |el: &X690Element| -> ASN1Result<OBJECT_IDENTIFIER> {
            Ok(ber_decode_object_identifier(&el.inner()?)?)
        }(_components.get("objectClass").unwrap())?;
        let criteria =
            |el: &X690Element| -> ASN1Result<Criteria> { Ok(_decode_Criteria(&el.inner()?)?) }(
                _components.get("criteria").unwrap(),
            )?;
        let subset: OPTIONAL<EnhancedGuide_subset> = match _components.get("subset") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<EnhancedGuide_subset> {
                Ok(_decode_EnhancedGuide_subset(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(EnhancedGuide {
            objectClass: objectClass_,
            criteria,
            subset,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_EnhancedGuide(value_: &EnhancedGuide) -> ASN1Result<X690Element> {
    |value_: &EnhancedGuide| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(|v_1: &OBJECT_IDENTIFIER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    ber_encode_object_identifier(&v_1)?,
                ))),
            ))
        }(&value_.objectClass)?);
        components_.push(|v_1: &Criteria| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                1,
                Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Criteria(&v_1)?))),
            ))
        }(&value_.criteria)?);
        if let Some(v_) = &value_.subset {
            if *v_ != EnhancedGuide::_default_value_for_subset() {
                components_.push(|v_1: &EnhancedGuide_subset| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_EnhancedGuide_subset(&v_1)?,
                        ))),
                    ))
                }(&v_)?);
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// PostalAddress  ::=  SEQUENCE SIZE (1..MAX) OF UnboundedDirectoryString
/// ```
pub type PostalAddress = Vec<UnboundedDirectoryString>; // SequenceOfType

pub fn _decode_PostalAddress(el: &X690Element) -> ASN1Result<PostalAddress> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<UnboundedDirectoryString>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<UnboundedDirectoryString> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_UnboundedDirectoryString(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_PostalAddress(value_: &PostalAddress) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_UnboundedDirectoryString(&v)?);
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// TelephoneNumber  ::=  PrintableString(SIZE (1..ub-telephone-number))
/// ```
pub type TelephoneNumber = PrintableString; // PrintableString

pub fn _decode_TelephoneNumber(el: &X690Element) -> ASN1Result<TelephoneNumber> {
    ber_decode_printable_string(&el)
}

pub fn _encode_TelephoneNumber(value_: &TelephoneNumber) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-telephone-number INTEGER ::= 32
/// ```
///
///
pub const ub_telephone_number: INTEGER = 32;

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
///
#[derive(Debug, Clone)]
pub struct TelexNumber {
    pub telexNumber: PrintableString,
    pub countryCode: PrintableString,
    pub answerback: PrintableString,
    pub _unrecognized: Vec<X690Element>,
}
impl TelexNumber {
    fn new(
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
impl TryFrom<X690Element> for TelexNumber {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TelexNumber(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TelexNumber {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<TelexNumber> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TelexNumber,
            _eal_components_for_TelexNumber,
            _rctl2_components_for_TelexNumber,
        )?;
        let telexNumber = ber_decode_printable_string(_components.get("telexNumber").unwrap())?;
        let countryCode = ber_decode_printable_string(_components.get("countryCode").unwrap())?;
        let answerback = ber_decode_printable_string(_components.get("answerback").unwrap())?;
        Ok(TelexNumber {
            telexNumber,
            countryCode,
            answerback,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TelexNumber(value_: &TelexNumber) -> ASN1Result<X690Element> {
    |value_: &TelexNumber| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(ber_encode_printable_string(&value_.telexNumber)?);
        components_.push(ber_encode_printable_string(&value_.countryCode)?);
        components_.push(ber_encode_printable_string(&value_.answerback)?);
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
/// ub-telex-number INTEGER ::= 14
/// ```
///
///
pub const ub_telex_number: INTEGER = 14;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-country-code INTEGER ::= 4
/// ```
///
///
pub const ub_country_code: INTEGER = 4;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-answerback   INTEGER ::= 8
/// ```
///
///
pub const ub_answerback: INTEGER = 8;

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

/// ### ASN.1 Definition:
///
/// ```asn1
/// FacsimileTelephoneNumber ::= SEQUENCE {
///   telephoneNumber  TelephoneNumber,
///   parameters       G3FacsimileNonBasicParameters OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct FacsimileTelephoneNumber {
    pub telephoneNumber: TelephoneNumber,
    pub parameters: OPTIONAL<G3FacsimileNonBasicParameters>,
    pub _unrecognized: Vec<X690Element>,
}
impl FacsimileTelephoneNumber {
    fn new(
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
impl TryFrom<X690Element> for FacsimileTelephoneNumber {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_FacsimileTelephoneNumber(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for FacsimileTelephoneNumber {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<FacsimileTelephoneNumber> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_FacsimileTelephoneNumber,
            _eal_components_for_FacsimileTelephoneNumber,
            _rctl2_components_for_FacsimileTelephoneNumber,
        )?;
        let telephoneNumber = _decode_TelephoneNumber(_components.get("telephoneNumber").unwrap())?;
        let parameters: OPTIONAL<G3FacsimileNonBasicParameters> =
            match _components.get("parameters") {
                Some(c_) => Some(_decode_G3FacsimileNonBasicParameters(c_)?),
                _ => None,
            };
        Ok(FacsimileTelephoneNumber {
            telephoneNumber,
            parameters,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_FacsimileTelephoneNumber(
    value_: &FacsimileTelephoneNumber,
) -> ASN1Result<X690Element> {
    |value_: &FacsimileTelephoneNumber| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_TelephoneNumber(&value_.telephoneNumber)?);
        if let Some(v_) = &value_.parameters {
            components_.push(_encode_G3FacsimileNonBasicParameters(&v_)?);
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// X121Address  ::=  NumericString(SIZE (1..ub-x121-address))
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
/// ub-x121-address INTEGER ::= 15
/// ```
///
///
pub const ub_x121_address: INTEGER = 15;

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

/// ### ASN.1 Definition:
///
/// ```asn1
/// InternationalISDNNumber  ::=
///   NumericString(SIZE (1..ub-international-isdn-number))
/// ```
pub type InternationalISDNNumber = NumericString; // NumericString

pub fn _decode_InternationalISDNNumber(el: &X690Element) -> ASN1Result<InternationalISDNNumber> {
    ber_decode_numeric_string(&el)
}

pub fn _encode_InternationalISDNNumber(
    value_: &InternationalISDNNumber,
) -> ASN1Result<X690Element> {
    ber_encode_numeric_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-international-isdn-number INTEGER ::= 16
/// ```
///
///
pub const ub_international_isdn_number: INTEGER = 16;

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

/// ### ASN.1 Definition:
///
/// ```asn1
/// DestinationIndicator  ::=  PrintableString(SIZE (1..MAX))
/// ```
pub type DestinationIndicator = PrintableString; // PrintableString

pub fn _decode_DestinationIndicator(el: &X690Element) -> ASN1Result<DestinationIndicator> {
    ber_decode_printable_string(&el)
}

pub fn _encode_DestinationIndicator(value_: &DestinationIndicator) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// CommunicationsService  ::=  OBJECT IDENTIFIER
/// ```
pub type CommunicationsService = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_CommunicationsService(el: &X690Element) -> ASN1Result<CommunicationsService> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_CommunicationsService(value_: &CommunicationsService) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// CommunicationsNetwork  ::=  OBJECT IDENTIFIER
/// ```
pub type CommunicationsNetwork = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_CommunicationsNetwork(el: &X690Element) -> ASN1Result<CommunicationsNetwork> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_CommunicationsNetwork(value_: &CommunicationsNetwork) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
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
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<PreferredDeliveryMethod_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<PreferredDeliveryMethod_Item> =
            Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_PreferredDeliveryMethod_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_PreferredDeliveryMethod(
    value_: &PreferredDeliveryMethod,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<PreferredDeliveryMethod_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_PreferredDeliveryMethod_Item(&v)?);
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
    fn new(
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
impl TryFrom<X690Element> for PresentationAddress {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PresentationAddress(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PresentationAddress {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<PresentationAddress> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PresentationAddress,
            _eal_components_for_PresentationAddress,
            _rctl2_components_for_PresentationAddress,
        )?;
        let pSelector: OPTIONAL<OCTET_STRING> = match _components.get("pSelector") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OCTET_STRING> {
                Ok(ber_decode_octet_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let sSelector: OPTIONAL<OCTET_STRING> = match _components.get("sSelector") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OCTET_STRING> {
                Ok(ber_decode_octet_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let tSelector: OPTIONAL<OCTET_STRING> = match _components.get("tSelector") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<OCTET_STRING> {
                Ok(ber_decode_octet_string(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let nAddresses = |el: &X690Element| -> ASN1Result<Vec<OCTET_STRING>> {
            Ok(|el: &X690Element| -> ASN1Result<SET_OF<OCTET_STRING>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<OCTET_STRING> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(ber_decode_octet_string(el)?);
                }
                Ok(items)
            }(&el.inner()?)?)
        }(_components.get("nAddresses").unwrap())?;
        Ok(PresentationAddress {
            pSelector,
            sSelector,
            tSelector,
            nAddresses,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_PresentationAddress(value_: &PresentationAddress) -> ASN1Result<X690Element> {
    |value_: &PresentationAddress| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(14);
        if let Some(v_) = &value_.pSelector {
            components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_octet_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.sSelector {
            components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_octet_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.tSelector {
            components_.push(|v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_octet_string(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        components_.push(|v_1: &Vec<OCTET_STRING>| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                3,
                Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                    OCTET_STRING,
                >|
                 -> ASN1Result<
                    X690Element,
                > {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(ber_encode_octet_string(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?))),
            ))
        }(&value_.nAddresses)?);
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// ProtocolInformation ::= SEQUENCE {
///   nAddress  OCTET STRING,
///   profiles  SET OF OBJECT IDENTIFIER }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ProtocolInformation {
    pub nAddress: OCTET_STRING,
    pub profiles: Vec<OBJECT_IDENTIFIER>,
}
impl ProtocolInformation {
    fn new(nAddress: OCTET_STRING, profiles: Vec<OBJECT_IDENTIFIER>) -> Self {
        ProtocolInformation { nAddress, profiles }
    }
}
impl TryFrom<X690Element> for ProtocolInformation {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ProtocolInformation(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ProtocolInformation {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<ProtocolInformation> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ProtocolInformation,
            _eal_components_for_ProtocolInformation,
            _rctl2_components_for_ProtocolInformation,
        )?;
        let nAddress = ber_decode_octet_string(_components.get("nAddress").unwrap())?;
        let profiles = |el: &X690Element| -> ASN1Result<SET_OF<OBJECT_IDENTIFIER>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SET_OF<OBJECT_IDENTIFIER> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(ber_decode_object_identifier(el)?);
            }
            Ok(items)
        }(_components.get("profiles").unwrap())?;
        Ok(ProtocolInformation { nAddress, profiles })
    }(&el)
}

pub fn _encode_ProtocolInformation(value_: &ProtocolInformation) -> ASN1Result<X690Element> {
    |value_: &ProtocolInformation| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_octet_string(&value_.nAddress)?);
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
            }(&value_.profiles)?,
        );
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// NameAndOptionalUID ::= SEQUENCE {
///   dn   DistinguishedName,
///   uid  UniqueIdentifier OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct NameAndOptionalUID {
    pub dn: DistinguishedName,
    pub uid: OPTIONAL<UniqueIdentifier>,
    pub _unrecognized: Vec<X690Element>,
}
impl NameAndOptionalUID {
    fn new(
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
impl TryFrom<X690Element> for NameAndOptionalUID {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_NameAndOptionalUID(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for NameAndOptionalUID {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<NameAndOptionalUID> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_NameAndOptionalUID,
            _eal_components_for_NameAndOptionalUID,
            _rctl2_components_for_NameAndOptionalUID,
        )?;
        let dn = _decode_DistinguishedName(_components.get("dn").unwrap())?;
        let uid: OPTIONAL<UniqueIdentifier> = match _components.get("uid") {
            Some(c_) => Some(_decode_UniqueIdentifier(c_)?),
            _ => None,
        };
        Ok(NameAndOptionalUID {
            dn,
            uid,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_NameAndOptionalUID(value_: &NameAndOptionalUID) -> ASN1Result<X690Element> {
    |value_: &NameAndOptionalUID| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(_encode_DistinguishedName(&value_.dn)?);
        if let Some(v_) = &value_.uid {
            components_.push(_encode_UniqueIdentifier(&v_)?);
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
///
#[derive(Debug, Clone)]
pub struct UiiFormat {
    pub baseObject: OPTIONAL<URI>,
    pub subset: OPTIONAL<UiiFormat_subset>,
    pub next: UiiFormat_next,
}
impl UiiFormat {
    fn new(
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
impl TryFrom<X690Element> for UiiFormat {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UiiFormat(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UiiFormat {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<UiiFormat> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UiiFormat,
            _eal_components_for_UiiFormat,
            _rctl2_components_for_UiiFormat,
        )?;
        let baseObject: OPTIONAL<URI> = match _components.get("baseObject") {
            Some(c_) => Some(_decode_URI(c_)?),
            _ => None,
        };
        let subset: OPTIONAL<UiiFormat_subset> = match _components.get("subset") {
            Some(c_) => Some(_decode_UiiFormat_subset(c_)?),
            _ => None,
        };
        let next = _decode_UiiFormat_next(_components.get("next").unwrap())?;
        Ok(UiiFormat {
            baseObject,
            subset,
            next,
        })
    }(&el)
}

pub fn _encode_UiiFormat(value_: &UiiFormat) -> ASN1Result<X690Element> {
    |value_: &UiiFormat| -> ASN1Result<X690Element> {
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
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(components_)),
        ))
    }(&value_)
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
// TODO: CHECK_RECURSIVE_DEFINITION
#[derive(Debug, Clone)]
pub enum UiiFilter {
    item(UiiItem),
    and(Vec<Box<UiiFilter>>),
    or(Vec<Box<UiiFilter>>),
    not(Box<UiiFilter>),
}

impl TryFrom<X690Element> for UiiFilter {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UiiFilter(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UiiFilter {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UiiFilter(el)
    }
}

pub fn _decode_UiiFilter(el: &X690Element) -> ASN1Result<UiiFilter> {
    |el: &X690Element| -> ASN1Result<UiiFilter> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(UiiFilter::item(_decode_UiiItem(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(UiiFilter::and(|el: &X690Element| -> ASN1Result<
                SET_OF<Box<UiiFilter>>,
            > {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<Box<UiiFilter>> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(Box::new(_decode_UiiFilter(el)?));
                }
                Ok(items)
            }(&el)?)),
            (TagClass::CONTEXT, 2) => Ok(UiiFilter::or(|el: &X690Element| -> ASN1Result<
                SET_OF<Box<UiiFilter>>,
            > {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<Box<UiiFilter>> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(Box::new(_decode_UiiFilter(el)?));
                }
                Ok(items)
            }(&el)?)),
            (TagClass::CONTEXT, 3) => Ok(UiiFilter::not(
                |el: &X690Element| -> ASN1Result<Box<UiiFilter>> {
                    Ok(Box::new(_decode_UiiFilter(&el.inner()?)?))
                }(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_UiiFilter(value_: &UiiFilter) -> ASN1Result<X690Element> {
    |value: &UiiFilter| -> ASN1Result<X690Element> {
        match value {
            UiiFilter::item(v) => |v_1: &UiiItem| -> ASN1Result<X690Element> {
                let mut el_1 = _encode_UiiItem(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            UiiFilter::and(v) => |v_1: &Vec<Box<UiiFilter>>| -> ASN1Result<X690Element> {
                let mut el_1 = |value_: &SET_OF<Box<UiiFilter>>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_UiiFilter(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            UiiFilter::or(v) => |v_1: &Vec<Box<UiiFilter>>| -> ASN1Result<X690Element> {
                let mut el_1 = |value_: &SET_OF<Box<UiiFilter>>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_UiiFilter(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 2;
                Ok(el_1)
            }(&v),
            UiiFilter::not(v) => |v_1: &UiiFilter| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_UiiFilter(&v_1)?))),
                ))
            }(&v),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UiiItem ::= SEQUENCE {
///   type   ATTRIBUTE.&id,
///   length INTEGER OPTIONAL }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct UiiItem {
    pub type_: OBJECT_IDENTIFIER,
    pub length: OPTIONAL<INTEGER>,
}
impl UiiItem {
    fn new(type_: OBJECT_IDENTIFIER, length: OPTIONAL<INTEGER>) -> Self {
        UiiItem { type_, length }
    }
}
impl TryFrom<X690Element> for UiiItem {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UiiItem(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UiiItem {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<UiiItem> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_UiiItem,
            _eal_components_for_UiiItem,
            _rctl2_components_for_UiiItem,
        )?;
        let type_ = ber_decode_object_identifier(_components.get("type").unwrap())?;
        let length: OPTIONAL<INTEGER> = match _components.get("length") {
            Some(c_) => Some(ber_decode_integer(c_)?),
            _ => None,
        };
        Ok(UiiItem { type_, length })
    }(&el)
}

pub fn _encode_UiiItem(value_: &UiiItem) -> ASN1Result<X690Element> {
    |value_: &UiiItem| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        components_.push(ber_encode_object_identifier(&value_.type_)?);
        if let Some(v_) = &value_.length {
            components_.push(ber_encode_integer(&v_)?);
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
///
#[derive(Debug, Clone)]
pub struct EpcFormat {
    pub fields: Vec<EpcFormat_fields_Item>,
    pub digitShift: OPTIONAL<INTEGER>,
    pub checkCalc: OPTIONAL<INTEGER>,
    pub urnPrefix: OPTIONAL<UTF8String>,
}
impl EpcFormat {
    fn new(
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
impl TryFrom<X690Element> for EpcFormat {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EpcFormat(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EpcFormat {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<EpcFormat> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EpcFormat,
            _eal_components_for_EpcFormat,
            _rctl2_components_for_EpcFormat,
        )?;
        let fields = |el: &X690Element| -> ASN1Result<SEQUENCE_OF<EpcFormat_fields_Item>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SEQUENCE_OF<EpcFormat_fields_Item> = Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_EpcFormat_fields_Item(el)?);
            }
            Ok(items)
        }(_components.get("fields").unwrap())?;
        let digitShift: OPTIONAL<INTEGER> = match _components.get("digitShift") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let checkCalc: OPTIONAL<INTEGER> = match _components.get("checkCalc") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let urnPrefix: OPTIONAL<UTF8String> = match _components.get("urnPrefix") {
            Some(c_) => Some(ber_decode_utf8_string(c_)?),
            _ => None,
        };
        Ok(EpcFormat {
            fields,
            digitShift,
            checkCalc,
            urnPrefix,
        })
    }(&el)
}

pub fn _encode_EpcFormat(value_: &EpcFormat) -> ASN1Result<X690Element> {
    |value_: &EpcFormat| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(9);
        components_.push(
            |value_: &SEQUENCE_OF<EpcFormat_fields_Item>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_EpcFormat_fields_Item(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.fields)?,
        );
        if let Some(v_) = &value_.digitShift {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.checkCalc {
            components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.urnPrefix {
            components_.push(ber_encode_utf8_string(&v_)?);
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// MultipleMatchingLocalities ::= SEQUENCE {
///   matchingRuleUsed  MATCHING-RULE.&id OPTIONAL,
///   attributeList     SEQUENCE OF AttributeValueAssertion,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct MultipleMatchingLocalities {
    pub matchingRuleUsed: OPTIONAL<OBJECT_IDENTIFIER>,
    pub attributeList: Vec<AttributeValueAssertion>,
    pub _unrecognized: Vec<X690Element>,
}
impl MultipleMatchingLocalities {
    fn new(
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
impl TryFrom<X690Element> for MultipleMatchingLocalities {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_MultipleMatchingLocalities(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for MultipleMatchingLocalities {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<MultipleMatchingLocalities> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_MultipleMatchingLocalities,
            _eal_components_for_MultipleMatchingLocalities,
            _rctl2_components_for_MultipleMatchingLocalities,
        )?;
        let matchingRuleUsed: OPTIONAL<OBJECT_IDENTIFIER> =
            match _components.get("matchingRuleUsed") {
                Some(c_) => Some(ber_decode_object_identifier(c_)?),
                _ => None,
            };
        let attributeList =
            |el: &X690Element| -> ASN1Result<SEQUENCE_OF<AttributeValueAssertion>> {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SEQUENCE_OF<AttributeValueAssertion> =
                    Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(_decode_AttributeValueAssertion(el)?);
                }
                Ok(items)
            }(_components.get("attributeList").unwrap())?;
        Ok(MultipleMatchingLocalities {
            matchingRuleUsed,
            attributeList,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_MultipleMatchingLocalities(
    value_: &MultipleMatchingLocalities,
) -> ASN1Result<X690Element> {
    |value_: &MultipleMatchingLocalities| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.matchingRuleUsed {
            components_.push(ber_encode_object_identifier(&v_)?);
        }
        components_.push(
            |value_: &SEQUENCE_OF<AttributeValueAssertion>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_AttributeValueAssertion(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.attributeList)?,
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// MRMappings  ::=  SEQUENCE OF MRMapping
/// ```
pub type MRMappings = Vec<MRMapping>; // SequenceOfType

pub fn _decode_MRMappings(el: &X690Element) -> ASN1Result<MRMappings> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<MRMapping>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<MRMapping> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_MRMapping(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_MRMappings(value_: &MRMappings) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<MRMapping>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_MRMapping(&v)?);
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
///
#[derive(Debug, Clone)]
pub struct PwdResponse {
    pub warning: OPTIONAL<PwdResponse_warning>,
    pub error: OPTIONAL<PwdResponse_error>,
}
impl PwdResponse {
    fn new(warning: OPTIONAL<PwdResponse_warning>, error: OPTIONAL<PwdResponse_error>) -> Self {
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
impl TryFrom<X690Element> for PwdResponse {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PwdResponse(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PwdResponse {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<PwdResponse> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_PwdResponse,
            _eal_components_for_PwdResponse,
            _rctl2_components_for_PwdResponse,
        )?;
        let warning: OPTIONAL<PwdResponse_warning> = match _components.get("warning") {
            Some(c_) => Some(_decode_PwdResponse_warning(c_)?),
            _ => None,
        };
        let error: OPTIONAL<PwdResponse_error> = match _components.get("error") {
            Some(c_) => Some(_decode_PwdResponse_error(c_)?),
            _ => None,
        };
        Ok(PwdResponse { warning, error })
    }(&el)
}

pub fn _encode_PwdResponse(value_: &PwdResponse) -> ASN1Result<X690Element> {
    |value_: &PwdResponse| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(7);
        if let Some(v_) = &value_.warning {
            components_.push(_encode_PwdResponse_warning(&v_)?);
        }
        if let Some(v_) = &value_.error {
            components_.push(_encode_PwdResponse_error(&v_)?);
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
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<SubstringAssertion_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<SubstringAssertion_Item> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_SubstringAssertion_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_SubstringAssertion(value_: &SubstringAssertion) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<SubstringAssertion_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_SubstringAssertion_Item(&v)?);
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// CaseIgnoreList  ::=  SEQUENCE OF UnboundedDirectoryString
/// ```
pub type CaseIgnoreList = Vec<UnboundedDirectoryString>; // SequenceOfType

pub fn _decode_CaseIgnoreList(el: &X690Element) -> ASN1Result<CaseIgnoreList> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<UnboundedDirectoryString>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<UnboundedDirectoryString> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_UnboundedDirectoryString(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_CaseIgnoreList(value_: &CaseIgnoreList) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<UnboundedDirectoryString>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_UnboundedDirectoryString(&v)?);
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
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<OctetSubstringAssertion_Item>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<OctetSubstringAssertion_Item> =
            Vec::with_capacity(elements.len());
        for el in elements {
            items.push(_decode_OctetSubstringAssertion_Item(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_OctetSubstringAssertion(
    value_: &OctetSubstringAssertion,
) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<OctetSubstringAssertion_Item>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(_encode_OctetSubstringAssertion_Item(&v)?);
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
    ber_decode_enumerated(&el)
}

pub fn _encode_SequenceMatchType(value_: &SequenceMatchType) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
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
    ber_decode_enumerated(&el)
}

pub fn _encode_WordMatchTypes(value_: &WordMatchTypes) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
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
    ber_decode_enumerated(&el)
}

pub fn _encode_CharacterMatchTypes(value_: &CharacterMatchTypes) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
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

pub fn _encode_ZonalSelect(value_: &ZonalSelect) -> ASN1Result<X690Element> {
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
    ber_decode_enumerated(&el)
}

pub fn _encode_ZonalResult(value_: &ZonalResult) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// LanguageContextSyntax  ::=  PrintableString(SIZE (2..3))
/// ```
pub type LanguageContextSyntax = PrintableString; // PrintableString

pub fn _decode_LanguageContextSyntax(el: &X690Element) -> ASN1Result<LanguageContextSyntax> {
    ber_decode_printable_string(&el)
}

pub fn _encode_LanguageContextSyntax(value_: &LanguageContextSyntax) -> ASN1Result<X690Element> {
    ber_encode_printable_string(&value_)
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
///
#[derive(Debug, Clone)]
pub struct TimeSpecification {
    pub time: TimeSpecification_time,
    pub notThisTime: OPTIONAL<BOOLEAN>,
    pub timeZone: OPTIONAL<TimeZone>,
    pub _unrecognized: Vec<X690Element>,
}
impl TimeSpecification {
    fn new(
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
impl TryFrom<X690Element> for TimeSpecification {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TimeSpecification(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TimeSpecification {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<TimeSpecification> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TimeSpecification,
            _eal_components_for_TimeSpecification,
            _rctl2_components_for_TimeSpecification,
        )?;
        let time = _decode_TimeSpecification_time(_components.get("time").unwrap())?;
        let notThisTime: OPTIONAL<BOOLEAN> = match _components.get("notThisTime") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        let timeZone: OPTIONAL<TimeZone> = match _components.get("timeZone") {
            Some(c_) => Some(_decode_TimeZone(c_)?),
            _ => None,
        };
        Ok(TimeSpecification {
            time,
            notThisTime,
            timeZone,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TimeSpecification(value_: &TimeSpecification) -> ASN1Result<X690Element> {
    |value_: &TimeSpecification| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(_encode_TimeSpecification_time(&value_.time)?);
        if let Some(v_) = &value_.notThisTime {
            if *v_ != TimeSpecification::_default_value_for_notThisTime() {
                components_.push(ber_encode_boolean(&v_)?);
            }
        }
        if let Some(v_) = &value_.timeZone {
            components_.push(_encode_TimeZone(&v_)?);
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
    fn new(
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
impl TryFrom<X690Element> for Period {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Period(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Period {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<Period> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_Period,
            _eal_components_for_Period,
            _rctl2_components_for_Period,
        )?;
        let timesOfDay: OPTIONAL<Vec<DayTimeBand>> = match _components.get("timesOfDay") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<DayTimeBand>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<DayTimeBand>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<DayTimeBand> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_DayTimeBand(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let days: OPTIONAL<Period_days> = match _components.get("days") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Period_days> {
                Ok(_decode_Period_days(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let weeks: OPTIONAL<Period_weeks> = match _components.get("weeks") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Period_weeks> {
                Ok(_decode_Period_weeks(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let months: OPTIONAL<Period_months> = match _components.get("months") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Period_months> {
                Ok(_decode_Period_months(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let years: OPTIONAL<Vec<INTEGER>> = match _components.get("years") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<Vec<INTEGER>> {
                Ok(|el: &X690Element| -> ASN1Result<SET_OF<INTEGER>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<INTEGER> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(ber_decode_integer(el)?);
                    }
                    Ok(items)
                }(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(Period {
            timesOfDay,
            days,
            weeks,
            months,
            years,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_Period(value_: &Period) -> ASN1Result<X690Element> {
    |value_: &Period| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(15);
        if let Some(v_) = &value_.timesOfDay {
            components_.push(|v_1: &Vec<DayTimeBand>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        DayTimeBand,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(_encode_DayTimeBand(&v)?);
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
        if let Some(v_) = &value_.days {
            components_.push(|v_1: &Period_days| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Period_days(&v_1)?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.weeks {
            components_.push(|v_1: &Period_weeks| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Period_weeks(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.months {
            components_.push(|v_1: &Period_months| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_Period_months(
                        &v_1,
                    )?))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.years {
            components_.push(|v_1: &Vec<INTEGER>| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(|value_: &SET_OF<
                        INTEGER,
                    >|
                     -> ASN1Result<
                        X690Element,
                    > {
                        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                        for v in value_ {
                            children.push(ber_encode_integer(&v)?);
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

impl TryFrom<X690Element> for XDayOf {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_XDayOf(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for XDayOf {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_XDayOf(el)
    }
}

pub fn _decode_XDayOf(el: &X690Element) -> ASN1Result<XDayOf> {
    |el: &X690Element| -> ASN1Result<XDayOf> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 1) => {
                Ok(XDayOf::first(|el: &X690Element| -> ASN1Result<NamedDay> {
                    Ok(_decode_NamedDay(&el.inner()?)?)
                }(&el)?))
            }
            (TagClass::CONTEXT, 2) => {
                Ok(XDayOf::second(|el: &X690Element| -> ASN1Result<NamedDay> {
                    Ok(_decode_NamedDay(&el.inner()?)?)
                }(&el)?))
            }
            (TagClass::CONTEXT, 3) => {
                Ok(XDayOf::third(|el: &X690Element| -> ASN1Result<NamedDay> {
                    Ok(_decode_NamedDay(&el.inner()?)?)
                }(&el)?))
            }
            (TagClass::CONTEXT, 4) => {
                Ok(XDayOf::fourth(|el: &X690Element| -> ASN1Result<NamedDay> {
                    Ok(_decode_NamedDay(&el.inner()?)?)
                }(&el)?))
            }
            (TagClass::CONTEXT, 5) => {
                Ok(XDayOf::fifth(|el: &X690Element| -> ASN1Result<NamedDay> {
                    Ok(_decode_NamedDay(&el.inner()?)?)
                }(&el)?))
            }
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_XDayOf(value_: &XDayOf) -> ASN1Result<X690Element> {
    |value: &XDayOf| -> ASN1Result<X690Element> {
        match value {
            XDayOf::first(v) => |v_1: &NamedDay| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_NamedDay(&v_1)?))),
                ))
            }(&v),
            XDayOf::second(v) => |v_1: &NamedDay| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    2,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_NamedDay(&v_1)?))),
                ))
            }(&v),
            XDayOf::third(v) => |v_1: &NamedDay| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    3,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_NamedDay(&v_1)?))),
                ))
            }(&v),
            XDayOf::fourth(v) => |v_1: &NamedDay| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    4,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_NamedDay(&v_1)?))),
                ))
            }(&v),
            XDayOf::fifth(v) => |v_1: &NamedDay| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    5,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_NamedDay(&v_1)?))),
                ))
            }(&v),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&value_)
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

impl TryFrom<X690Element> for NamedDay {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_NamedDay(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for NamedDay {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_NamedDay(el)
    }
}

pub fn _decode_NamedDay(el: &X690Element) -> ASN1Result<NamedDay> {
    |el: &X690Element| -> ASN1Result<NamedDay> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 10) => {
                Ok(NamedDay::intNamedDays(_decode_NamedDay_intNamedDays(&el)?))
            }
            (TagClass::UNIVERSAL, 3) => {
                Ok(NamedDay::bitNamedDays(_decode_NamedDay_bitNamedDays(&el)?))
            }
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_NamedDay(value_: &NamedDay) -> ASN1Result<X690Element> {
    |value: &NamedDay| -> ASN1Result<X690Element> {
        match value {
            NamedDay::intNamedDays(v) => _encode_NamedDay_intNamedDays(&v),
            NamedDay::bitNamedDays(v) => _encode_NamedDay_bitNamedDays(&v),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&value_)
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
///
#[derive(Debug, Clone)]
pub struct DayTimeBand {
    pub startDayTime: OPTIONAL<DayTime>,
    pub endDayTime: OPTIONAL<DayTime>,
    pub _unrecognized: Vec<X690Element>,
}
impl DayTimeBand {
    fn new(
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
impl TryFrom<X690Element> for DayTimeBand {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DayTimeBand(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DayTimeBand {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<DayTimeBand> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DayTimeBand,
            _eal_components_for_DayTimeBand,
            _rctl2_components_for_DayTimeBand,
        )?;
        let startDayTime: OPTIONAL<DayTime> = match _components.get("startDayTime") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DayTime> {
                Ok(_decode_DayTime(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let endDayTime: OPTIONAL<DayTime> = match _components.get("endDayTime") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<DayTime> {
                Ok(_decode_DayTime(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(DayTimeBand {
            startDayTime,
            endDayTime,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DayTimeBand(value_: &DayTimeBand) -> ASN1Result<X690Element> {
    |value_: &DayTimeBand| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.startDayTime {
            if *v_ != DayTimeBand::_default_value_for_startDayTime() {
                components_.push(|v_1: &DayTime| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DayTime(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.endDayTime {
            if *v_ != DayTimeBand::_default_value_for_endDayTime() {
                components_.push(|v_1: &DayTime| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(_encode_DayTime(&v_1)?))),
                    ))
                }(&v_)?);
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
///
#[derive(Debug, Clone, PartialEq)]
pub struct DayTime {
    pub hour: INTEGER,
    pub minute: OPTIONAL<INTEGER>,
    pub second: OPTIONAL<INTEGER>,
    pub _unrecognized: Vec<X690Element>,
}
impl DayTime {
    fn new(
        hour: INTEGER,
        minute: OPTIONAL<INTEGER>,
        second: OPTIONAL<INTEGER>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        DayTime {
            hour,
            minute,
            second,
            _unrecognized,
        }
    }
    pub fn _default_value_for_minute() -> INTEGER {
        0
    }
    pub fn _default_value_for_second() -> INTEGER {
        0
    }
}
impl TryFrom<X690Element> for DayTime {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_DayTime(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for DayTime {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_DayTime(el)
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
    |el_: &X690Element| -> ASN1Result<DayTime> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_DayTime,
            _eal_components_for_DayTime,
            _rctl2_components_for_DayTime,
        )?;
        let hour =
            |el: &X690Element| -> ASN1Result<INTEGER> { Ok(ber_decode_integer(&el.inner()?)?) }(
                _components.get("hour").unwrap(),
            )?;
        let minute: OPTIONAL<INTEGER> = match _components.get("minute") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let second: OPTIONAL<INTEGER> = match _components.get("second") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<INTEGER> {
                Ok(ber_decode_integer(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(DayTime {
            hour,
            minute,
            second,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_DayTime(value_: &DayTime) -> ASN1Result<X690Element> {
    |value_: &DayTime| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
            ))
        }(&value_.hour)?);
        if let Some(v_) = &value_.minute {
            if *v_ != DayTime::_default_value_for_minute() {
                components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.second {
            if *v_ != DayTime::_default_value_for_second() {
                components_.push(|v_1: &INTEGER| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_integer(&v_1)?))),
                    ))
                }(&v_)?);
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeZone  ::=  INTEGER(-12..12)
/// ```
pub type TimeZone = INTEGER;

pub fn _decode_TimeZone(el: &X690Element) -> ASN1Result<TimeZone> {
    ber_decode_integer(&el)
}

pub fn _encode_TimeZone(value_: &TimeZone) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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

impl TryFrom<X690Element> for TimeAssertion {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TimeAssertion(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TimeAssertion {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TimeAssertion(el)
    }
}

pub fn _decode_TimeAssertion(el: &X690Element) -> ASN1Result<TimeAssertion> {
    |el: &X690Element| -> ASN1Result<TimeAssertion> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(TimeAssertion::now(ber_decode_null(&el)?)),
            (TagClass::UNIVERSAL, 24) => Ok(TimeAssertion::at(ber_decode_generalized_time(&el)?)),
            (TagClass::UNIVERSAL, 16) => {
                Ok(TimeAssertion::between(_decode_TimeAssertion_between(&el)?))
            }
            _ => Ok(TimeAssertion::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_TimeAssertion(value_: &TimeAssertion) -> ASN1Result<X690Element> {
    |value: &TimeAssertion| -> ASN1Result<X690Element> {
        match value {
            TimeAssertion::now(v) => ber_encode_null(&v),
            TimeAssertion::at(v) => ber_encode_generalized_time(&v),
            TimeAssertion::between(v) => _encode_TimeAssertion_between(&v),
            TimeAssertion::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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

impl TryFrom<X690Element> for LocaleContextSyntax {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_LocaleContextSyntax(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for LocaleContextSyntax {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_LocaleContextSyntax(el)
    }
}

pub fn _decode_LocaleContextSyntax(el: &X690Element) -> ASN1Result<LocaleContextSyntax> {
    |el: &X690Element| -> ASN1Result<LocaleContextSyntax> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 6) => Ok(LocaleContextSyntax::localeID1(
                ber_decode_object_identifier(&el)?,
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
    }(&el)
}

pub fn _encode_LocaleContextSyntax(value_: &LocaleContextSyntax) -> ASN1Result<X690Element> {
    |value: &LocaleContextSyntax| -> ASN1Result<X690Element> {
        match value {
            LocaleContextSyntax::localeID1(v) => ber_encode_object_identifier(&v),
            LocaleContextSyntax::localeID2(v) => _encode_UnboundedDirectoryString(&v),
            LocaleContextSyntax::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// AttributeOptionList  ::=  SEQUENCE OF UTF8String
/// ```
pub type AttributeOptionList = Vec<UTF8String>; // SequenceOfType

pub fn _decode_AttributeOptionList(el: &X690Element) -> ASN1Result<AttributeOptionList> {
    |el: &X690Element| -> ASN1Result<SEQUENCE_OF<UTF8String>> {
        let elements = match el.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let mut items: SEQUENCE_OF<UTF8String> = Vec::with_capacity(elements.len());
        for el in elements {
            items.push(ber_decode_utf8_string(el)?);
        }
        Ok(items)
    }(&el)
}

pub fn _encode_AttributeOptionList(value_: &AttributeOptionList) -> ASN1Result<X690Element> {
    |value_: &SEQUENCE_OF<UTF8String>| -> ASN1Result<X690Element> {
        let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
        for v in value_ {
            children.push(ber_encode_utf8_string(&v)?);
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
/// id-at-knowledgeInformation                OBJECT IDENTIFIER ::= {id-at 2}
/// ```
///
///
pub fn id_at_knowledgeInformation() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-commonName                          OBJECT IDENTIFIER ::= {id-at 3}
/// ```
///
///
pub fn id_at_commonName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-surname                             OBJECT IDENTIFIER ::= {id-at 4}
/// ```
///
///
pub fn id_at_surname() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-serialNumber                        OBJECT IDENTIFIER ::= {id-at 5}
/// ```
///
///
pub fn id_at_serialNumber() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-countryName                         OBJECT IDENTIFIER ::= {id-at 6}
/// ```
///
///
pub fn id_at_countryName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-localityName                        OBJECT IDENTIFIER ::= {id-at 7}
/// ```
///
///
pub fn id_at_localityName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveLocalityName              OBJECT IDENTIFIER ::= {id-at 7 1}
/// ```
///
///
pub fn id_at_collectiveLocalityName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([7, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-stateOrProvinceName                 OBJECT IDENTIFIER ::= {id-at 8}
/// ```
///
///
pub fn id_at_stateOrProvinceName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveStateOrProvinceName       OBJECT IDENTIFIER ::= {id-at 8 1}
/// ```
///
///
pub fn id_at_collectiveStateOrProvinceName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([8, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-streetAddress                       OBJECT IDENTIFIER ::= {id-at 9}
/// ```
///
///
pub fn id_at_streetAddress() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveStreetAddress             OBJECT IDENTIFIER ::= {id-at 9 1}
/// ```
///
///
pub fn id_at_collectiveStreetAddress() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([9, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-organizationName                    OBJECT IDENTIFIER ::= {id-at 10}
/// ```
///
///
pub fn id_at_organizationName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveOrganizationName          OBJECT IDENTIFIER ::= {id-at 10 1}
/// ```
///
///
pub fn id_at_collectiveOrganizationName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([10, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-organizationalUnitName              OBJECT IDENTIFIER ::= {id-at 11}
/// ```
///
///
pub fn id_at_organizationalUnitName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveOrganizationalUnitName    OBJECT IDENTIFIER ::= {id-at 11 1}
/// ```
///
///
pub fn id_at_collectiveOrganizationalUnitName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([11, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-title                               OBJECT IDENTIFIER ::= {id-at 12}
/// ```
///
///
pub fn id_at_title() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([12])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-description                         OBJECT IDENTIFIER ::= {id-at 13}
/// ```
///
///
pub fn id_at_description() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([13])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-searchGuide                         OBJECT IDENTIFIER ::= {id-at 14}
/// ```
///
///
pub fn id_at_searchGuide() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([14])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-businessCategory                    OBJECT IDENTIFIER ::= {id-at 15}
/// ```
///
///
pub fn id_at_businessCategory() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([15])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-postalAddress                       OBJECT IDENTIFIER ::= {id-at 16}
/// ```
///
///
pub fn id_at_postalAddress() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([16])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectivePostalAddress             OBJECT IDENTIFIER ::= {id-at 16 1}
/// ```
///
///
pub fn id_at_collectivePostalAddress() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([16, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-postalCode                          OBJECT IDENTIFIER ::= {id-at 17}
/// ```
///
///
pub fn id_at_postalCode() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([17])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectivePostalCode                OBJECT IDENTIFIER ::= {id-at 17 1}
/// ```
///
///
pub fn id_at_collectivePostalCode() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([17, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-postOfficeBox                       OBJECT IDENTIFIER ::= {id-at 18}
/// ```
///
///
pub fn id_at_postOfficeBox() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([18])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectivePostOfficeBox             OBJECT IDENTIFIER ::= {id-at 18 1}
/// ```
///
///
pub fn id_at_collectivePostOfficeBox() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([18, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-physicalDeliveryOfficeName          OBJECT IDENTIFIER ::= {id-at 19}
/// ```
///
///
pub fn id_at_physicalDeliveryOfficeName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([19])].concat() // OID_GETTER
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
    [id_at(), Vec::<u32>::from([19, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-telephoneNumber                     OBJECT IDENTIFIER ::= {id-at 20}
/// ```
///
///
pub fn id_at_telephoneNumber() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([20])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveTelephoneNumber           OBJECT IDENTIFIER ::= {id-at 20 1}
/// ```
///
///
pub fn id_at_collectiveTelephoneNumber() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([20, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-telexNumber                         OBJECT IDENTIFIER ::= {id-at 21}
/// ```
///
///
pub fn id_at_telexNumber() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([21])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveTelexNumber               OBJECT IDENTIFIER ::= {id-at 21 1}
/// ```
///
///
pub fn id_at_collectiveTelexNumber() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([21, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-facsimileTelephoneNumber            OBJECT IDENTIFIER ::= {id-at 23}
/// ```
///
///
pub fn id_at_facsimileTelephoneNumber() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([23])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveFacsimileTelephoneNumber  OBJECT IDENTIFIER ::= {id-at 23 1}
/// ```
///
///
pub fn id_at_collectiveFacsimileTelephoneNumber() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([23, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-x121Address                         OBJECT IDENTIFIER ::= {id-at 24}
/// ```
///
///
pub fn id_at_x121Address() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([24])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-internationalISDNNumber             OBJECT IDENTIFIER ::= {id-at 25}
/// ```
///
///
pub fn id_at_internationalISDNNumber() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([25])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-collectiveInternationalISDNNumber   OBJECT IDENTIFIER ::= {id-at 25 1}
/// ```
///
///
pub fn id_at_collectiveInternationalISDNNumber() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([25, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-registeredAddress                   OBJECT IDENTIFIER ::= {id-at 26}
/// ```
///
///
pub fn id_at_registeredAddress() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([26])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-destinationIndicator                OBJECT IDENTIFIER ::= {id-at 27}
/// ```
///
///
pub fn id_at_destinationIndicator() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([27])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-preferredDeliveryMethod             OBJECT IDENTIFIER ::= {id-at 28}
/// ```
///
///
pub fn id_at_preferredDeliveryMethod() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([28])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-presentationAddress                 OBJECT IDENTIFIER ::= {id-at 29}
/// ```
///
///
pub fn id_at_presentationAddress() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([29])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-supportedApplicationContext         OBJECT IDENTIFIER ::= {id-at 30}
/// ```
///
///
pub fn id_at_supportedApplicationContext() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([30])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-member                              OBJECT IDENTIFIER ::= {id-at 31}
/// ```
///
///
pub fn id_at_member() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([31])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-owner                               OBJECT IDENTIFIER ::= {id-at 32}
/// ```
///
///
pub fn id_at_owner() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([32])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-roleOccupant                        OBJECT IDENTIFIER ::= {id-at 33}
/// ```
///
///
pub fn id_at_roleOccupant() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([33])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-seeAlso                             OBJECT IDENTIFIER ::= {id-at 34}
/// ```
///
///
pub fn id_at_seeAlso() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([34])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-name                                OBJECT IDENTIFIER ::= {id-at 41}
/// ```
///
///
pub fn id_at_name() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([41])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-givenName                           OBJECT IDENTIFIER ::= {id-at 42}
/// ```
///
///
pub fn id_at_givenName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([42])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-initials                            OBJECT IDENTIFIER ::= {id-at 43}
/// ```
///
///
pub fn id_at_initials() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([43])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-generationQualifier                 OBJECT IDENTIFIER ::= {id-at 44}
/// ```
///
///
pub fn id_at_generationQualifier() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([44])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uniqueIdentifier                    OBJECT IDENTIFIER ::= {id-at 45}
/// ```
///
///
pub fn id_at_uniqueIdentifier() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([45])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-dnQualifier                         OBJECT IDENTIFIER ::= {id-at 46}
/// ```
///
///
pub fn id_at_dnQualifier() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([46])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-enhancedSearchGuide                 OBJECT IDENTIFIER ::= {id-at 47}
/// ```
///
///
pub fn id_at_enhancedSearchGuide() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([47])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-protocolInformation                 OBJECT IDENTIFIER ::= {id-at 48}
/// ```
///
///
pub fn id_at_protocolInformation() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([48])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-distinguishedName                   OBJECT IDENTIFIER ::= {id-at 49}
/// ```
///
///
pub fn id_at_distinguishedName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([49])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uniqueMember                        OBJECT IDENTIFIER ::= {id-at 50}
/// ```
///
///
pub fn id_at_uniqueMember() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([50])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-houseIdentifier                     OBJECT IDENTIFIER ::= {id-at 51}
/// ```
///
///
pub fn id_at_houseIdentifier() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([51])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-dmdName                             OBJECT IDENTIFIER ::= {id-at 54}
/// ```
///
///
pub fn id_at_dmdName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([54])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-pseudonym                           OBJECT IDENTIFIER ::= {id-at 65}
/// ```
///
///
pub fn id_at_pseudonym() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([65])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-communicationsService               OBJECT IDENTIFIER ::= {id-at 66}
/// ```
///
///
pub fn id_at_communicationsService() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([66])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-communicationsNetwork               OBJECT IDENTIFIER ::= {id-at 67}
/// ```
///
///
pub fn id_at_communicationsNetwork() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([67])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uuidpair                            OBJECT IDENTIFIER ::= {id-at 77}
/// ```
///
///
pub fn id_at_uuidpair() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([77])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-tagOid                              OBJECT IDENTIFIER ::= {id-at 78}
/// ```
///
///
pub fn id_at_tagOid() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([78])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uiiFormat                           OBJECT IDENTIFIER ::= {id-at 79}
/// ```
///
///
pub fn id_at_uiiFormat() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([79])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uiiInUrn                            OBJECT IDENTIFIER ::= {id-at 80}
/// ```
///
///
pub fn id_at_uiiInUrn() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([80])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-contentUrl                          OBJECT IDENTIFIER ::= {id-at 81}
/// ```
///
///
pub fn id_at_contentUrl() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([81])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uri                                 OBJECT IDENTIFIER ::= {id-at 83}
/// ```
///
///
pub fn id_at_uri() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([83])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-urn                                 OBJECT IDENTIFIER ::= {id-at 86}
/// ```
///
///
pub fn id_at_urn() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([86])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-url                                 OBJECT IDENTIFIER ::= {id-at 87}
/// ```
///
///
pub fn id_at_url() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([87])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-utmCoordinates                      OBJECT IDENTIFIER ::= {id-at 88}
/// ```
///
///
pub fn id_at_utmCoordinates() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([88])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-urnC                                OBJECT IDENTIFIER ::= {id-at 89}
/// ```
///
///
pub fn id_at_urnC() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([89])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-uii                                 OBJECT IDENTIFIER ::= {id-at 90}
/// ```
///
///
pub fn id_at_uii() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([90])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-epc                                 OBJECT IDENTIFIER ::= {id-at 91}
/// ```
///
///
pub fn id_at_epc() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([91])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-tagAfi                              OBJECT IDENTIFIER ::= {id-at 92}
/// ```
///
///
pub fn id_at_tagAfi() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([92])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-epcFormat                           OBJECT IDENTIFIER ::= {id-at 93}
/// ```
///
///
pub fn id_at_epcFormat() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([93])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-epcInUrn                            OBJECT IDENTIFIER ::= {id-at 94}
/// ```
///
///
pub fn id_at_epcInUrn() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([94])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-ldapUrl                             OBJECT IDENTIFIER ::= {id-at 95}
/// ```
///
///
pub fn id_at_ldapUrl() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([95])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-tagLocation                         OBJECT IDENTIFIER ::= {id-at 96}
/// ```
///
///
pub fn id_at_tagLocation() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([96])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-organizationIdentifier              OBJECT IDENTIFIER ::= {id-at 97}
/// ```
///
///
pub fn id_at_organizationIdentifier() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([97])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-countryCode3c                       OBJECT IDENTIFIER ::= {id-at 98}
/// ```
///
///
pub fn id_at_countryCode3c() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([98])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-countryCode3n                       OBJECT IDENTIFIER ::= {id-at 99}
/// ```
///
///
pub fn id_at_countryCode3n() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([99])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-dnsName                             OBJECT IDENTIFIER ::= {id-at 100}
/// ```
///
///
pub fn id_at_dnsName() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([100])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-intEmail                            OBJECT IDENTIFIER ::= {id-at 104}
/// ```
///
///
pub fn id_at_intEmail() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([104])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-jid                                 OBJECT IDENTIFIER ::= {id-at 105}
/// ```
///
///
pub fn id_at_jid() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([105])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-objectIdentifier                    OBJECT IDENTIFIER ::= {id-at 106}
/// ```
///
///
pub fn id_at_objectIdentifier() -> OBJECT_IDENTIFIER {
    [id_at(), Vec::<u32>::from([106])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-utmCoords                          OBJECT IDENTIFIER ::= {id-asx 4}
/// ```
///
///
pub fn id_asx_utmCoords() -> OBJECT_IDENTIFIER {
    [id_asx(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-uiiForm                            OBJECT IDENTIFIER ::= {id-asx 5}
/// ```
///
///
pub fn id_asx_uiiForm() -> OBJECT_IDENTIFIER {
    [id_asx(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-epcForm                            OBJECT IDENTIFIER ::= {id-asx 6}
/// ```
///
///
pub fn id_asx_epcForm() -> OBJECT_IDENTIFIER {
    [id_asx(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-countryString3c                    OBJECT IDENTIFIER ::= {id-asx 7}
/// ```
///
///
pub fn id_asx_countryString3c() -> OBJECT_IDENTIFIER {
    [id_asx(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-countryString3n                    OBJECT IDENTIFIER ::= {id-asx 8}
/// ```
///
///
pub fn id_asx_countryString3n() -> OBJECT_IDENTIFIER {
    [id_asx(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-dnsString                          OBJECT IDENTIFIER ::= {id-asx 9}
/// ```
///
///
pub fn id_asx_dnsString() -> OBJECT_IDENTIFIER {
    [id_asx(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-intEmailString                     OBJECT IDENTIFIER ::= {id-asx 11}
/// ```
///
///
pub fn id_asx_intEmailString() -> OBJECT_IDENTIFIER {
    [id_asx(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-jidString                          OBJECT IDENTIFIER ::= {id-asx 12}
/// ```
///
///
pub fn id_asx_jidString() -> OBJECT_IDENTIFIER {
    [id_asx(), Vec::<u32>::from([12])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-attributeTypeDescription           OBJECT IDENTIFIER ::= {id-lsx 3}
/// ```
///
///
pub fn id_lsx_attributeTypeDescription() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-bitString                          OBJECT IDENTIFIER ::= {id-lsx 6}
/// ```
///
///
pub fn id_lsx_bitString() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-boolean                            OBJECT IDENTIFIER ::= {id-lsx 7}
/// ```
///
///
pub fn id_lsx_boolean() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-countryString                      OBJECT IDENTIFIER ::= {id-lsx 11}
/// ```
///
///
pub fn id_lsx_countryString() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-dn                                 OBJECT IDENTIFIER ::= {id-lsx 12}
/// ```
///
///
pub fn id_lsx_dn() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([12])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-deliveryMethod                     OBJECT IDENTIFIER ::= {id-lsx 14}
/// ```
///
///
pub fn id_lsx_deliveryMethod() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([14])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-directoryString                    OBJECT IDENTIFIER ::= {id-lsx 15}
/// ```
///
///
pub fn id_lsx_directoryString() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([15])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-dITContentRuleDescription          OBJECT IDENTIFIER ::= {id-lsx 16}
/// ```
///
///
pub fn id_lsx_dITContentRuleDescription() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([16])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-dITStructureRuleDescription        OBJECT IDENTIFIER ::= {id-lsx 17}
/// ```
///
///
pub fn id_lsx_dITStructureRuleDescription() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([17])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-enhancedGuide                      OBJECT IDENTIFIER ::= {id-lsx 21}
/// ```
///
///
pub fn id_lsx_enhancedGuide() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([21])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-facsimileTelephoneNr               OBJECT IDENTIFIER ::= {id-lsx 22}
/// ```
///
///
pub fn id_lsx_facsimileTelephoneNr() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([22])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-fax                                OBJECT IDENTIFIER ::= {id-lsx 23}
/// ```
///
///
pub fn id_lsx_fax() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([23])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-generalizedTime                    OBJECT IDENTIFIER ::= {id-lsx 24}
/// ```
///
///
pub fn id_lsx_generalizedTime() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([24])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-guide                              OBJECT IDENTIFIER ::= {id-lsx 25}
/// ```
///
///
pub fn id_lsx_guide() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([25])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-ia5String                          OBJECT IDENTIFIER ::= {id-lsx 26}
/// ```
///
///
pub fn id_lsx_ia5String() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([26])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-integer                            OBJECT IDENTIFIER ::= {id-lsx 27}
/// ```
///
///
pub fn id_lsx_integer() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([27])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-jpeg                               OBJECT IDENTIFIER ::= {id-lsx 28}
/// ```
///
///
pub fn id_lsx_jpeg() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([28])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-matchingRuleDescription            OBJECT IDENTIFIER ::= {id-lsx 30}
/// ```
///
///
pub fn id_lsx_matchingRuleDescription() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([30])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-matchingRuleUseDescription         OBJECT IDENTIFIER ::= {id-lsx 31}
/// ```
///
///
pub fn id_lsx_matchingRuleUseDescription() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([31])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-nameAndOptionalUID                 OBJECT IDENTIFIER ::= {id-lsx 34}
/// ```
///
///
pub fn id_lsx_nameAndOptionalUID() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([34])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-nameFormDescription                OBJECT IDENTIFIER ::= {id-lsx 35}
/// ```
///
///
pub fn id_lsx_nameFormDescription() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([35])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-numericString                      OBJECT IDENTIFIER ::= {id-lsx 36}
/// ```
///
///
pub fn id_lsx_numericString() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([36])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-objectClassDescription             OBJECT IDENTIFIER ::= {id-lsx 37}
/// ```
///
///
pub fn id_lsx_objectClassDescription() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([37])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-oid                                OBJECT IDENTIFIER ::= {id-lsx 38}
/// ```
///
///
pub fn id_lsx_oid() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([38])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-otherMailbox                       OBJECT IDENTIFIER ::= {id-lsx 39}
/// ```
///
///
pub fn id_lsx_otherMailbox() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([39])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-octetString                        OBJECT IDENTIFIER ::= {id-lsx 40}
/// ```
///
///
pub fn id_lsx_octetString() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([40])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-postalAddr                         OBJECT IDENTIFIER ::= {id-lsx 41}
/// ```
///
///
pub fn id_lsx_postalAddr() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([41])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-presentationAddr                   OBJECT IDENTIFIER ::= {id-lsx 43}
/// ```
///
///
pub fn id_lsx_presentationAddr() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([43])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-printableString                    OBJECT IDENTIFIER ::= {id-lsx 44}
/// ```
///
///
pub fn id_lsx_printableString() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([44])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-subtreeSpec                        OBJECT IDENTIFIER ::= {id-lsx 45}
/// ```
///
///
pub fn id_lsx_subtreeSpec() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([45])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-telephoneNr                        OBJECT IDENTIFIER ::= {id-lsx 50}
/// ```
///
///
pub fn id_lsx_telephoneNr() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([50])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-telexNr                            OBJECT IDENTIFIER ::= {id-lsx 52}
/// ```
///
///
pub fn id_lsx_telexNr() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([52])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-utcTime                            OBJECT IDENTIFIER ::= {id-lsx 53}
/// ```
///
///
pub fn id_lsx_utcTime() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([53])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-ldapSyntaxDescription              OBJECT IDENTIFIER ::= {id-lsx 54}
/// ```
///
///
pub fn id_lsx_ldapSyntaxDescription() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([54])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx-substringAssertion                 OBJECT IDENTIFIER ::= {id-lsx 58}
/// ```
///
///
pub fn id_lsx_substringAssertion() -> OBJECT_IDENTIFIER {
    [id_lsx(), Vec::<u32>::from([58])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oidC1                                  OBJECT IDENTIFIER ::= {id 0}
/// ```
///
///
pub fn id_oidC1() -> OBJECT_IDENTIFIER {
    [id(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oidC2                                  OBJECT IDENTIFIER ::= {id 1}
/// ```
///
///
pub fn id_oidC2() -> OBJECT_IDENTIFIER {
    [id(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oidC                                   OBJECT IDENTIFIER ::= {id 2}
/// ```
///
///
pub fn id_oidC() -> OBJECT_IDENTIFIER {
    [id(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cat-sequenceMatchType                  OBJECT IDENTIFIER ::= {id-cat 1}
/// ```
///
///
pub fn id_cat_sequenceMatchType() -> OBJECT_IDENTIFIER {
    [id_cat(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cat-wordMatchType                      OBJECT IDENTIFIER ::= {id-cat 2}
/// ```
///
///
pub fn id_cat_wordMatchType() -> OBJECT_IDENTIFIER {
    [id_cat(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cat-characterMatchTypes                OBJECT IDENTIFIER ::= {id-cat 3}
/// ```
///
///
pub fn id_cat_characterMatchTypes() -> OBJECT_IDENTIFIER {
    [id_cat(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cat-selectedContexts                   OBJECT IDENTIFIER ::= {id-cat 4}
/// ```
///
///
pub fn id_cat_selectedContexts() -> OBJECT_IDENTIFIER {
    [id_cat(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-dSAProblem                         OBJECT IDENTIFIER ::= {id-not 0}
/// ```
///
///
pub fn id_not_dSAProblem() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-searchServiceProblem               OBJECT IDENTIFIER ::= {id-not 1}
/// ```
///
///
pub fn id_not_searchServiceProblem() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-serviceType                        OBJECT IDENTIFIER ::= {id-not 2}
/// ```
///
///
pub fn id_not_serviceType() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-attributeTypeList                  OBJECT IDENTIFIER ::= {id-not 3}
/// ```
///
///
pub fn id_not_attributeTypeList() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-matchingRuleList                   OBJECT IDENTIFIER ::= {id-not 4}
/// ```
///
///
pub fn id_not_matchingRuleList() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-filterItem                         OBJECT IDENTIFIER ::= {id-not 5}
/// ```
///
///
pub fn id_not_filterItem() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-attributeCombinations              OBJECT IDENTIFIER ::= {id-not 6}
/// ```
///
///
pub fn id_not_attributeCombinations() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-contextTypeList                    OBJECT IDENTIFIER ::= {id-not 7}
/// ```
///
///
pub fn id_not_contextTypeList() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-contextList                        OBJECT IDENTIFIER ::= {id-not 8}
/// ```
///
///
pub fn id_not_contextList() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-contextCombinations                OBJECT IDENTIFIER ::= {id-not 9}
/// ```
///
///
pub fn id_not_contextCombinations() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-hierarchySelectList                OBJECT IDENTIFIER ::= {id-not 10}
/// ```
///
///
pub fn id_not_hierarchySelectList() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-searchControlOptionsList           OBJECT IDENTIFIER ::= {id-not 11}
/// ```
///
///
pub fn id_not_searchControlOptionsList() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-serviceControlOptionsList          OBJECT IDENTIFIER ::= {id-not 12}
/// ```
///
///
pub fn id_not_serviceControlOptionsList() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([12])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-multipleMatchingLocalities         OBJECT IDENTIFIER ::= {id-not 13}
/// ```
///
///
pub fn id_not_multipleMatchingLocalities() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([13])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-proposedRelaxation                 OBJECT IDENTIFIER ::= {id-not 14}
/// ```
///
///
pub fn id_not_proposedRelaxation() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([14])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-appliedRelaxation                  OBJECT IDENTIFIER ::= {id-not 15}
/// ```
///
///
pub fn id_not_appliedRelaxation() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([15])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-pwdResponse                        OBJECT IDENTIFIER ::= {id-not 16}
/// ```
///
///
pub fn id_not_pwdResponse() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([16])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not-ldapDiagnosticMsg                  OBJECT IDENTIFIER ::= {id-not 17}
/// ```
///
///
pub fn id_not_ldapDiagnosticMsg() -> OBJECT_IDENTIFIER {
    [id_not(), Vec::<u32>::from([17])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-targetDsaUnavailable                OBJECT IDENTIFIER ::= {id-pr 1}
/// ```
///
///
pub fn id_pr_targetDsaUnavailable() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-dataSourceUnavailable               OBJECT IDENTIFIER ::= {id-pr 2}
/// ```
///
///
pub fn id_pr_dataSourceUnavailable() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unidentifiedOperation               OBJECT IDENTIFIER ::= {id-pr 3}
/// ```
///
///
pub fn id_pr_unidentifiedOperation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unavailableOperation                OBJECT IDENTIFIER ::= {id-pr 4}
/// ```
///
///
pub fn id_pr_unavailableOperation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchAttributeViolation            OBJECT IDENTIFIER ::= {id-pr 5}
/// ```
///
///
pub fn id_pr_searchAttributeViolation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchAttributeCombinationViolation OBJECT IDENTIFIER ::= {id-pr 6}
/// ```
///
///
pub fn id_pr_searchAttributeCombinationViolation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchValueNotAllowed               OBJECT IDENTIFIER ::= {id-pr 7}
/// ```
///
///
pub fn id_pr_searchValueNotAllowed() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-missingSearchAttribute              OBJECT IDENTIFIER ::= {id-pr 8}
/// ```
///
///
pub fn id_pr_missingSearchAttribute() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchValueViolation                OBJECT IDENTIFIER ::= {id-pr 9}
/// ```
///
///
pub fn id_pr_searchValueViolation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-attributeNegationViolation          OBJECT IDENTIFIER ::= {id-pr 10}
/// ```
///
///
pub fn id_pr_attributeNegationViolation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchValueRequired                 OBJECT IDENTIFIER ::= {id-pr 11}
/// ```
///
///
pub fn id_pr_searchValueRequired() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-invalidSearchValue                  OBJECT IDENTIFIER ::= {id-pr 12}
/// ```
///
///
pub fn id_pr_invalidSearchValue() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([12])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchContextViolation              OBJECT IDENTIFIER ::= {id-pr 13}
/// ```
///
///
pub fn id_pr_searchContextViolation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([13])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchContextCombinationViolation   OBJECT IDENTIFIER ::= {id-pr 14}
/// ```
///
///
pub fn id_pr_searchContextCombinationViolation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([14])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-missingSearchContext                OBJECT IDENTIFIER ::= {id-pr 15}
/// ```
///
///
pub fn id_pr_missingSearchContext() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([15])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchContextValueViolation         OBJECT IDENTIFIER ::= {id-pr 16}
/// ```
///
///
pub fn id_pr_searchContextValueViolation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([16])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchContextValueRequired          OBJECT IDENTIFIER ::= {id-pr 17}
/// ```
///
///
pub fn id_pr_searchContextValueRequired() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([17])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-invalidContextSearchValue           OBJECT IDENTIFIER ::= {id-pr 18}
/// ```
///
///
pub fn id_pr_invalidContextSearchValue() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([18])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unsupportedMatchingRule             OBJECT IDENTIFIER ::= {id-pr 19}
/// ```
///
///
pub fn id_pr_unsupportedMatchingRule() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([19])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-attributeMatchingViolation          OBJECT IDENTIFIER ::= {id-pr 20}
/// ```
///
///
pub fn id_pr_attributeMatchingViolation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([20])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unsupportedMatchingUse              OBJECT IDENTIFIER ::= {id-pr 21}
/// ```
///
///
pub fn id_pr_unsupportedMatchingUse() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([21])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-matchingUseViolation                OBJECT IDENTIFIER ::= {id-pr 22}
/// ```
///
///
pub fn id_pr_matchingUseViolation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([22])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-hierarchySelectForbidden            OBJECT IDENTIFIER ::= {id-pr 23}
/// ```
///
///
pub fn id_pr_hierarchySelectForbidden() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([23])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-invalidHierarchySelect              OBJECT IDENTIFIER ::= {id-pr 24}
/// ```
///
///
pub fn id_pr_invalidHierarchySelect() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([24])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unavailableHierarchySelect          OBJECT IDENTIFIER ::= {id-pr 25}
/// ```
///
///
pub fn id_pr_unavailableHierarchySelect() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([25])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-invalidSearchControlOptions         OBJECT IDENTIFIER ::= {id-pr 26}
/// ```
///
///
pub fn id_pr_invalidSearchControlOptions() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([26])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-invalidServiceControlOptions        OBJECT IDENTIFIER ::= {id-pr 27}
/// ```
///
///
pub fn id_pr_invalidServiceControlOptions() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([27])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-searchSubsetViolation               OBJECT IDENTIFIER ::= {id-pr 28}
/// ```
///
///
pub fn id_pr_searchSubsetViolation() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([28])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unmatchedKeyAttributes              OBJECT IDENTIFIER ::= {id-pr 29}
/// ```
///
///
pub fn id_pr_unmatchedKeyAttributes() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([29])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-ambiguousKeyAttributes              OBJECT IDENTIFIER ::= {id-pr 30}
/// ```
///
///
pub fn id_pr_ambiguousKeyAttributes() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([30])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-unavailableRelaxationLevel          OBJECT IDENTIFIER ::= {id-pr 31}
/// ```
///
///
pub fn id_pr_unavailableRelaxationLevel() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([31])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-emptyHierarchySelection             OBJECT IDENTIFIER ::= {id-pr 32}
/// ```
///
///
pub fn id_pr_emptyHierarchySelection() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([32])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-administratorImposedLimit           OBJECT IDENTIFIER ::= {id-pr 33}
/// ```
///
///
pub fn id_pr_administratorImposedLimit() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([33])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-permanentRestriction                OBJECT IDENTIFIER ::= {id-pr 34}
/// ```
///
///
pub fn id_pr_permanentRestriction() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([34])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-temporaryRestriction                OBJECT IDENTIFIER ::= {id-pr 35}
/// ```
///
///
pub fn id_pr_temporaryRestriction() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([35])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr-relaxationNotSupported              OBJECT IDENTIFIER ::= {id-pr 36}
/// ```
///
///
pub fn id_pr_relaxationNotSupported() -> OBJECT_IDENTIFIER {
    [id_pr(), Vec::<u32>::from([36])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-coat-uid                               OBJECT IDENTIFIER ::= {id-coat 1}
/// ```
///
///
pub fn id_coat_uid() -> OBJECT_IDENTIFIER {
    [id_coat(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-coat-dc                                OBJECT IDENTIFIER ::= {id-coat 25}
/// ```
///
///
pub fn id_coat_dc() -> OBJECT_IDENTIFIER {
    [id_coat(), Vec::<u32>::from([25])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseIgnoreMatch                     OBJECT IDENTIFIER ::= {id-mr 2}
/// ```
///
///
pub fn id_mr_caseIgnoreMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseIgnoreOrderingMatch             OBJECT IDENTIFIER ::= {id-mr 3}
/// ```
///
///
pub fn id_mr_caseIgnoreOrderingMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseIgnoreSubstringsMatch           OBJECT IDENTIFIER ::= {id-mr 4}
/// ```
///
///
pub fn id_mr_caseIgnoreSubstringsMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([4])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseExactMatch                      OBJECT IDENTIFIER ::= {id-mr 5}
/// ```
///
///
pub fn id_mr_caseExactMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseExactOrderingMatch              OBJECT IDENTIFIER ::= {id-mr 6}
/// ```
///
///
pub fn id_mr_caseExactOrderingMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseExactSubstringsMatch            OBJECT IDENTIFIER ::= {id-mr 7}
/// ```
///
///
pub fn id_mr_caseExactSubstringsMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-numericStringMatch                  OBJECT IDENTIFIER ::= {id-mr 8}
/// ```
///
///
pub fn id_mr_numericStringMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([8])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-numericStringOrderingMatch          OBJECT IDENTIFIER ::= {id-mr 9}
/// ```
///
///
pub fn id_mr_numericStringOrderingMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([9])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-numericStringSubstringsMatch        OBJECT IDENTIFIER ::= {id-mr 10}
/// ```
///
///
pub fn id_mr_numericStringSubstringsMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([10])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseIgnoreListMatch                 OBJECT IDENTIFIER ::= {id-mr 11}
/// ```
///
///
pub fn id_mr_caseIgnoreListMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([11])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-caseIgnoreListSubstringsMatch       OBJECT IDENTIFIER ::= {id-mr 12}
/// ```
///
///
pub fn id_mr_caseIgnoreListSubstringsMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([12])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-booleanMatch                        OBJECT IDENTIFIER ::= {id-mr 13}
/// ```
///
///
pub fn id_mr_booleanMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([13])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-integerMatch                        OBJECT IDENTIFIER ::= {id-mr 14}
/// ```
///
///
pub fn id_mr_integerMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([14])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-integerOrderingMatch                OBJECT IDENTIFIER ::= {id-mr 15}
/// ```
///
///
pub fn id_mr_integerOrderingMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([15])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-bitStringMatch                      OBJECT IDENTIFIER ::= {id-mr 16}
/// ```
///
///
pub fn id_mr_bitStringMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([16])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-octetStringMatch                    OBJECT IDENTIFIER ::= {id-mr 17}
/// ```
///
///
pub fn id_mr_octetStringMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([17])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-octetStringOrderingMatch            OBJECT IDENTIFIER ::= {id-mr 18}
/// ```
///
///
pub fn id_mr_octetStringOrderingMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([18])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-octetStringSubstringsMatch          OBJECT IDENTIFIER ::= {id-mr 19}
/// ```
///
///
pub fn id_mr_octetStringSubstringsMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([19])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-telephoneNumberMatch                OBJECT IDENTIFIER ::= {id-mr 20}
/// ```
///
///
pub fn id_mr_telephoneNumberMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([20])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-telephoneNumberSubstringsMatch      OBJECT IDENTIFIER ::= {id-mr 21}
/// ```
///
///
pub fn id_mr_telephoneNumberSubstringsMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([21])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-presentationAddressMatch            OBJECT IDENTIFIER ::= {id-mr 22}
/// ```
///
///
pub fn id_mr_presentationAddressMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([22])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-uniqueMemberMatch                   OBJECT IDENTIFIER ::= {id-mr 23}
/// ```
///
///
pub fn id_mr_uniqueMemberMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([23])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-protocolInformationMatch            OBJECT IDENTIFIER ::= {id-mr 24}
/// ```
///
///
pub fn id_mr_protocolInformationMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([24])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-uTCTimeMatch                        OBJECT IDENTIFIER ::= {id-mr 25}
/// ```
///
///
pub fn id_mr_uTCTimeMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([25])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-uTCTimeOrderingMatch                OBJECT IDENTIFIER ::= {id-mr 26}
/// ```
///
///
pub fn id_mr_uTCTimeOrderingMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([26])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-generalizedTimeMatch                OBJECT IDENTIFIER ::= {id-mr 27}
/// ```
///
///
pub fn id_mr_generalizedTimeMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([27])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-generalizedTimeOrderingMatch        OBJECT IDENTIFIER ::= {id-mr 28}
/// ```
///
///
pub fn id_mr_generalizedTimeOrderingMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([28])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-integerFirstComponentMatch          OBJECT IDENTIFIER ::= {id-mr 29}
/// ```
///
///
pub fn id_mr_integerFirstComponentMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([29])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-objectIdentifierFirstComponentMatch OBJECT IDENTIFIER ::= {id-mr 30}
/// ```
///
///
pub fn id_mr_objectIdentifierFirstComponentMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([30])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-directoryStringFirstComponentMatch  OBJECT IDENTIFIER ::= {id-mr 31}
/// ```
///
///
pub fn id_mr_directoryStringFirstComponentMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([31])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-wordMatch                           OBJECT IDENTIFIER ::= {id-mr 32}
/// ```
///
///
pub fn id_mr_wordMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([32])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-keywordMatch                        OBJECT IDENTIFIER ::= {id-mr 33}
/// ```
///
///
pub fn id_mr_keywordMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([33])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-storedPrefixMatch                   OBJECT IDENTIFIER ::= {id-mr 41}
/// ```
///
///
pub fn id_mr_storedPrefixMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([41])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-systemProposedMatch                 OBJECT IDENTIFIER ::= {id-mr 47}
/// ```
///
///
pub fn id_mr_systemProposedMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([47])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-generalWordMatch                    OBJECT IDENTIFIER ::= {id-mr 48}
/// ```
///
///
pub fn id_mr_generalWordMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([48])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-approximateStringMatch              OBJECT IDENTIFIER ::= {id-mr 49}
/// ```
///
///
pub fn id_mr_approximateStringMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([49])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-ignoreIfAbsentMatch                 OBJECT IDENTIFIER ::= {id-mr 50}
/// ```
///
///
pub fn id_mr_ignoreIfAbsentMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([50])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-nullMatch                           OBJECT IDENTIFIER ::= {id-mr 51}
/// ```
///
///
pub fn id_mr_nullMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([51])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-zonalMatch                          OBJECT IDENTIFIER ::= {id-mr 52}
/// ```
///
///
pub fn id_mr_zonalMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([52])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-facsimileNumberMatch                OBJECT IDENTIFIER ::= {id-mr 63}
/// ```
///
///
pub fn id_mr_facsimileNumberMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([63])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-facsimileNumberSubstringsMatch      OBJECT IDENTIFIER ::= {id-mr 64}
/// ```
///
///
pub fn id_mr_facsimileNumberSubstringsMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([64])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-uuidpairmatch                       OBJECT IDENTIFIER ::= {id-mr 68}
/// ```
///
///
pub fn id_mr_uuidpairmatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([68])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-uriMatch                            OBJECT IDENTIFIER ::= {id-mr 70}
/// ```
///
///
pub fn id_mr_uriMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([70])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-dnsNameMatch                        OBJECT IDENTIFIER ::= {id-mr 74}
/// ```
///
///
pub fn id_mr_dnsNameMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([74])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-intEmailMatch                       OBJECT IDENTIFIER ::= {id-mr 75}
/// ```
///
///
pub fn id_mr_intEmailMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([75])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-jidMatch                            OBJECT IDENTIFIER ::= {id-mr 76}
/// ```
///
///
pub fn id_mr_jidMatch() -> OBJECT_IDENTIFIER {
    [id_mr(), Vec::<u32>::from([76])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lmr-caseExactIA5Match                  OBJECT IDENTIFIER ::= {id-lmr 1}
/// ```
///
///
pub fn id_lmr_caseExactIA5Match() -> OBJECT_IDENTIFIER {
    [id_lmr(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lmr-caseIgnoreIA5Match                 OBJECT IDENTIFIER ::= {id-lmr 2}
/// ```
///
///
pub fn id_lmr_caseIgnoreIA5Match() -> OBJECT_IDENTIFIER {
    [id_lmr(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lmr-caseIgnoreIA5SubstringsMatch       OBJECT IDENTIFIER ::= {id-lmr 3}
/// ```
///
///
pub fn id_lmr_caseIgnoreIA5SubstringsMatch() -> OBJECT_IDENTIFIER {
    [id_lmr(), Vec::<u32>::from([3])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc-language                           OBJECT IDENTIFIER ::= {id-avc 0}
/// ```
///
///
pub fn id_avc_language() -> OBJECT_IDENTIFIER {
    [id_avc(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc-temporal                           OBJECT IDENTIFIER ::= {id-avc 1}
/// ```
///
///
pub fn id_avc_temporal() -> OBJECT_IDENTIFIER {
    [id_avc(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc-locale                             OBJECT IDENTIFIER ::= {id-avc 2}
/// ```
///
///
pub fn id_avc_locale() -> OBJECT_IDENTIFIER {
    [id_avc(), Vec::<u32>::from([2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc-ldapAttributeOption                OBJECT IDENTIFIER ::= {id-avc 5}
/// ```
///
///
pub fn id_avc_ldapAttributeOption() -> OBJECT_IDENTIFIER {
    [id_avc(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EnhancedGuide-subset ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type EnhancedGuide_subset = INTEGER;

pub const EnhancedGuide_subset_baseObject: EnhancedGuide_subset = 0; /* LONG_NAMED_INTEGER_VALUE */

pub const EnhancedGuide_subset_oneLevel: EnhancedGuide_subset = 1; /* LONG_NAMED_INTEGER_VALUE */

pub const EnhancedGuide_subset_wholeSubtree: EnhancedGuide_subset = 2; /* LONG_NAMED_INTEGER_VALUE */

pub fn _decode_EnhancedGuide_subset(el: &X690Element) -> ASN1Result<EnhancedGuide_subset> {
    ber_decode_integer(&el)
}

pub fn _encode_EnhancedGuide_subset(value_: &EnhancedGuide_subset) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PreferredDeliveryMethod-Item ::= INTEGER { -- REMOVED_FROM_UNNESTING -- }
/// ```
pub type PreferredDeliveryMethod_Item = INTEGER;

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
    ber_decode_integer(&el)
}

pub fn _encode_PreferredDeliveryMethod_Item(
    value_: &PreferredDeliveryMethod_Item,
) -> ASN1Result<X690Element> {
    ber_encode_integer(&value_)
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
    ber_decode_enumerated(&el)
}

pub fn _encode_UiiFormat_subset(value_: &UiiFormat_subset) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
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

impl TryFrom<X690Element> for UiiFormat_next {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_UiiFormat_next(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for UiiFormat_next {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_UiiFormat_next(el)
    }
}

pub fn _decode_UiiFormat_next(el: &X690Element) -> ASN1Result<UiiFormat_next> {
    |el: &X690Element| -> ASN1Result<UiiFormat_next> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 2) => Ok(UiiFormat_next::length(ber_decode_integer(&el)?)),
            (TagClass::CONTEXT, 0) => Ok(UiiFormat_next::filter(_decode_UiiFilter(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(UiiFormat_next::filter(_decode_UiiFilter(&el)?)),
            (TagClass::CONTEXT, 2) => Ok(UiiFormat_next::filter(_decode_UiiFilter(&el)?)),
            (TagClass::CONTEXT, 3) => Ok(UiiFormat_next::filter(_decode_UiiFilter(&el)?)),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_UiiFormat_next(value_: &UiiFormat_next) -> ASN1Result<X690Element> {
    |value: &UiiFormat_next| -> ASN1Result<X690Element> {
        match value {
            UiiFormat_next::length(v) => ber_encode_integer(&v),
            UiiFormat_next::filter(v) => _encode_UiiFilter(&v),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&value_)
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

impl TryFrom<X690Element> for EpcFormat_fields_Item_charField {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EpcFormat_fields_Item_charField(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EpcFormat_fields_Item_charField {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_EpcFormat_fields_Item_charField(el)
    }
}

pub fn _decode_EpcFormat_fields_Item_charField(
    el: &X690Element,
) -> ASN1Result<EpcFormat_fields_Item_charField> {
    |el: &X690Element| -> ASN1Result<EpcFormat_fields_Item_charField> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(EpcFormat_fields_Item_charField::characters(
                ber_decode_integer(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(EpcFormat_fields_Item_charField::maxValue(
                ber_decode_integer(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_EpcFormat_fields_Item_charField(
    value_: &EpcFormat_fields_Item_charField,
) -> ASN1Result<X690Element> {
    |value: &EpcFormat_fields_Item_charField| -> ASN1Result<X690Element> {
        match value {
            EpcFormat_fields_Item_charField::characters(v) => {
                |v_1: &INTEGER| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_integer(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            EpcFormat_fields_Item_charField::maxValue(v) => {
                |v_1: &INTEGER| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_integer(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&value_)
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
    ber_decode_enumerated(&el)
}

pub fn _encode_EpcFormat_fields_Item_result(
    value_: &EpcFormat_fields_Item_result,
) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// EpcFormat-fields-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct EpcFormat_fields_Item {
    pub bits: INTEGER,
    pub charField: EpcFormat_fields_Item_charField,
    pub result: OPTIONAL<EpcFormat_fields_Item_result>,
}
impl EpcFormat_fields_Item {
    fn new(
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
impl TryFrom<X690Element> for EpcFormat_fields_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_EpcFormat_fields_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for EpcFormat_fields_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<EpcFormat_fields_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_EpcFormat_fields_Item,
            _eal_components_for_EpcFormat_fields_Item,
            _rctl2_components_for_EpcFormat_fields_Item,
        )?;
        let bits = ber_decode_integer(_components.get("bits").unwrap())?;
        let charField =
            _decode_EpcFormat_fields_Item_charField(_components.get("charField").unwrap())?;
        let result: OPTIONAL<EpcFormat_fields_Item_result> = match _components.get("result") {
            Some(c_) => Some(_decode_EpcFormat_fields_Item_result(c_)?),
            _ => None,
        };
        Ok(EpcFormat_fields_Item {
            bits,
            charField,
            result,
        })
    }(&el)
}

pub fn _encode_EpcFormat_fields_Item(value_: &EpcFormat_fields_Item) -> ASN1Result<X690Element> {
    |value_: &EpcFormat_fields_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(8);
        components_.push(ber_encode_integer(&value_.bits)?);
        components_.push(_encode_EpcFormat_fields_Item_charField(&value_.charField)?);
        if let Some(v_) = &value_.result {
            if *v_ != EpcFormat_fields_Item::_default_value_for_result() {
                components_.push(_encode_EpcFormat_fields_Item_result(&v_)?);
            }
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
/// PwdResponse-warning ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum PwdResponse_warning {
    timeleft(INTEGER),
    graceRemaining(INTEGER),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<X690Element> for PwdResponse_warning {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_PwdResponse_warning(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for PwdResponse_warning {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_PwdResponse_warning(el)
    }
}

pub fn _decode_PwdResponse_warning(el: &X690Element) -> ASN1Result<PwdResponse_warning> {
    |el: &X690Element| -> ASN1Result<PwdResponse_warning> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(PwdResponse_warning::timeleft(ber_decode_integer(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(PwdResponse_warning::graceRemaining(ber_decode_integer(
                &el,
            )?)),
            _ => Ok(PwdResponse_warning::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_PwdResponse_warning(value_: &PwdResponse_warning) -> ASN1Result<X690Element> {
    |value: &PwdResponse_warning| -> ASN1Result<X690Element> {
        match value {
            PwdResponse_warning::timeleft(v) => |v_1: &INTEGER| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_integer(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 0;
                Ok(el_1)
            }(&v),
            PwdResponse_warning::graceRemaining(v) => |v_1: &INTEGER| -> ASN1Result<X690Element> {
                let mut el_1 = ber_encode_integer(&v_1)?;
                el_1.tag_class = TagClass::CONTEXT;
                el_1.tag_number = 1;
                Ok(el_1)
            }(&v),
            PwdResponse_warning::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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
    ber_decode_enumerated(&el)
}

pub fn _encode_PwdResponse_error(value_: &PwdResponse_error) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
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

impl TryFrom<X690Element> for SubstringAssertion_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_SubstringAssertion_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for SubstringAssertion_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_SubstringAssertion_Item(el)
    }
}

pub fn _decode_SubstringAssertion_Item(el: &X690Element) -> ASN1Result<SubstringAssertion_Item> {
    |el: &X690Element| -> ASN1Result<SubstringAssertion_Item> {
        match (el.tag_class, el.tag_number) {
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
            (TagClass::UNIVERSAL, 16) => {
                Ok(SubstringAssertion_Item::control(_decode_Attribute(&el)?))
            }
            _ => Ok(SubstringAssertion_Item::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_SubstringAssertion_Item(
    value_: &SubstringAssertion_Item,
) -> ASN1Result<X690Element> {
    |value: &SubstringAssertion_Item| -> ASN1Result<X690Element> {
        match value {
            SubstringAssertion_Item::initial(v) => {
                |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_UnboundedDirectoryString(&v_1)?,
                        ))),
                    ))
                }(&v)
            }
            SubstringAssertion_Item::any(v) => {
                |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_UnboundedDirectoryString(&v_1)?,
                        ))),
                    ))
                }(&v)
            }
            SubstringAssertion_Item::final_(v) => {
                |v_1: &UnboundedDirectoryString| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        2,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(
                            _encode_UnboundedDirectoryString(&v_1)?,
                        ))),
                    ))
                }(&v)
            }
            SubstringAssertion_Item::control(v) => _encode_Attribute(&v),
            SubstringAssertion_Item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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

impl TryFrom<X690Element> for OctetSubstringAssertion_Item {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_OctetSubstringAssertion_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for OctetSubstringAssertion_Item {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_OctetSubstringAssertion_Item(el)
    }
}

pub fn _decode_OctetSubstringAssertion_Item(
    el: &X690Element,
) -> ASN1Result<OctetSubstringAssertion_Item> {
    |el: &X690Element| -> ASN1Result<OctetSubstringAssertion_Item> {
        match (el.tag_class, el.tag_number) {
            (TagClass::CONTEXT, 0) => Ok(OctetSubstringAssertion_Item::initial(
                ber_decode_octet_string(&el)?,
            )),
            (TagClass::CONTEXT, 1) => Ok(OctetSubstringAssertion_Item::any(
                ber_decode_octet_string(&el)?,
            )),
            (TagClass::CONTEXT, 2) => Ok(OctetSubstringAssertion_Item::final_(
                ber_decode_octet_string(&el)?,
            )),
            _ => Ok(OctetSubstringAssertion_Item::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_OctetSubstringAssertion_Item(
    value_: &OctetSubstringAssertion_Item,
) -> ASN1Result<X690Element> {
    |value: &OctetSubstringAssertion_Item| -> ASN1Result<X690Element> {
        match value {
            OctetSubstringAssertion_Item::initial(v) => {
                |v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_octet_string(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 0;
                    Ok(el_1)
                }(&v)
            }
            OctetSubstringAssertion_Item::any(v) => {
                |v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_octet_string(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 1;
                    Ok(el_1)
                }(&v)
            }
            OctetSubstringAssertion_Item::final_(v) => {
                |v_1: &OCTET_STRING| -> ASN1Result<X690Element> {
                    let mut el_1 = ber_encode_octet_string(&v_1)?;
                    el_1.tag_class = TagClass::CONTEXT;
                    el_1.tag_number = 2;
                    Ok(el_1)
                }(&v)
            }
            OctetSubstringAssertion_Item::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeSpecification-time-absolute ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TimeSpecification_time_absolute {
    pub startTime: OPTIONAL<GeneralizedTime>,
    pub endTime: OPTIONAL<GeneralizedTime>,
    pub _unrecognized: Vec<X690Element>,
}
impl TimeSpecification_time_absolute {
    fn new(
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
impl TryFrom<X690Element> for TimeSpecification_time_absolute {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TimeSpecification_time_absolute(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TimeSpecification_time_absolute {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<TimeSpecification_time_absolute> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TimeSpecification_time_absolute,
            _eal_components_for_TimeSpecification_time_absolute,
            _rctl2_components_for_TimeSpecification_time_absolute,
        )?;
        let startTime: OPTIONAL<GeneralizedTime> = match _components.get("startTime") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralizedTime> {
                Ok(ber_decode_generalized_time(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let endTime: OPTIONAL<GeneralizedTime> = match _components.get("endTime") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralizedTime> {
                Ok(ber_decode_generalized_time(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        Ok(TimeSpecification_time_absolute {
            startTime,
            endTime,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TimeSpecification_time_absolute(
    value_: &TimeSpecification_time_absolute,
) -> ASN1Result<X690Element> {
    |value_: &TimeSpecification_time_absolute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        if let Some(v_) = &value_.startTime {
            components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    0,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_generalized_time(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.endTime {
            components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_generalized_time(&v_1)?,
                    ))),
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
/// TimeSpecification-time ::= CHOICE { -- REMOVED_FROM_UNNESTING -- }
/// ```
#[derive(Debug, Clone)]
pub enum TimeSpecification_time {
    absolute(TimeSpecification_time_absolute),
    periodic(Vec<Period>),
}

impl TryFrom<X690Element> for TimeSpecification_time {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TimeSpecification_time(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TimeSpecification_time {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_TimeSpecification_time(el)
    }
}

pub fn _decode_TimeSpecification_time(el: &X690Element) -> ASN1Result<TimeSpecification_time> {
    |el: &X690Element| -> ASN1Result<TimeSpecification_time> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 16) => Ok(TimeSpecification_time::absolute(
                _decode_TimeSpecification_time_absolute(&el)?,
            )),
            (TagClass::UNIVERSAL, 17) => Ok(TimeSpecification_time::periodic(
                |el: &X690Element| -> ASN1Result<SET_OF<Period>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<Period> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(_decode_Period(el)?);
                    }
                    Ok(items)
                }(&el)?,
            )),
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&el)
}

pub fn _encode_TimeSpecification_time(value_: &TimeSpecification_time) -> ASN1Result<X690Element> {
    |value: &TimeSpecification_time| -> ASN1Result<X690Element> {
        match value {
            TimeSpecification_time::absolute(v) => _encode_TimeSpecification_time_absolute(&v),
            TimeSpecification_time::periodic(v) => {
                |value_: &SET_OF<Period>| -> ASN1Result<X690Element> {
                    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                    for v in value_ {
                        children.push(_encode_Period(&v)?);
                    }
                    Ok(X690Element::new(
                        TagClass::UNIVERSAL,
                        ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                        Arc::new(X690Encoding::Constructed(children)),
                    ))
                }(&v)
            }
            _ => {
                return Err(ASN1Error::new(
                    ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                ))
            }
        }
    }(&value_)
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
    ber_decode_bit_string(&el)
}

pub fn _encode_Period_days_bitDay(value_: &Period_days_bitDay) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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

impl TryFrom<X690Element> for Period_days {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Period_days(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Period_days {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Period_days(el)
    }
}

pub fn _decode_Period_days(el: &X690Element) -> ASN1Result<Period_days> {
    |el: &X690Element| -> ASN1Result<Period_days> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 17) => Ok(Period_days::intDay(|el: &X690Element| -> ASN1Result<
                SET_OF<INTEGER>,
            > {
                let elements = match el.value.borrow() {
                    X690Encoding::Constructed(children) => children,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                };
                let mut items: SET_OF<INTEGER> = Vec::with_capacity(elements.len());
                for el in elements {
                    items.push(ber_decode_integer(el)?);
                }
                Ok(items)
            }(&el)?)),
            (TagClass::UNIVERSAL, 3) => Ok(Period_days::bitDay(_decode_Period_days_bitDay(&el)?)),
            (TagClass::CONTEXT, 1) => Ok(Period_days::dayOf(_decode_XDayOf(&el)?)),
            (TagClass::CONTEXT, 2) => Ok(Period_days::dayOf(_decode_XDayOf(&el)?)),
            (TagClass::CONTEXT, 3) => Ok(Period_days::dayOf(_decode_XDayOf(&el)?)),
            (TagClass::CONTEXT, 4) => Ok(Period_days::dayOf(_decode_XDayOf(&el)?)),
            (TagClass::CONTEXT, 5) => Ok(Period_days::dayOf(_decode_XDayOf(&el)?)),
            _ => Ok(Period_days::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Period_days(value_: &Period_days) -> ASN1Result<X690Element> {
    |value: &Period_days| -> ASN1Result<X690Element> {
        match value {
            Period_days::intDay(v) => |value_: &SET_OF<INTEGER>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(ber_encode_integer(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&v),
            Period_days::bitDay(v) => _encode_Period_days_bitDay(&v),
            Period_days::dayOf(v) => _encode_XDayOf(&v),
            Period_days::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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
    ber_decode_bit_string(&el)
}

pub fn _encode_Period_weeks_bitWeek(value_: &Period_weeks_bitWeek) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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

impl TryFrom<X690Element> for Period_weeks {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Period_weeks(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Period_weeks {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Period_weeks(el)
    }
}

pub fn _decode_Period_weeks(el: &X690Element) -> ASN1Result<Period_weeks> {
    |el: &X690Element| -> ASN1Result<Period_weeks> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(Period_weeks::allWeeks(ber_decode_null(&el)?)),
            (TagClass::UNIVERSAL, 17) => Ok(Period_weeks::intWeek(
                |el: &X690Element| -> ASN1Result<SET_OF<INTEGER>> {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<INTEGER> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(ber_decode_integer(el)?);
                    }
                    Ok(items)
                }(&el)?,
            )),
            (TagClass::UNIVERSAL, 3) => {
                Ok(Period_weeks::bitWeek(_decode_Period_weeks_bitWeek(&el)?))
            }
            _ => Ok(Period_weeks::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Period_weeks(value_: &Period_weeks) -> ASN1Result<X690Element> {
    |value: &Period_weeks| -> ASN1Result<X690Element> {
        match value {
            Period_weeks::allWeeks(v) => ber_encode_null(&v),
            Period_weeks::intWeek(v) => |value_: &SET_OF<INTEGER>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(ber_encode_integer(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&v),
            Period_weeks::bitWeek(v) => _encode_Period_weeks_bitWeek(&v),
            Period_weeks::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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
    ber_decode_bit_string(&el)
}

pub fn _encode_Period_months_bitMonth(value_: &Period_months_bitMonth) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
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

impl TryFrom<X690Element> for Period_months {
    type Error = ASN1Error;

    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_Period_months(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for Period_months {
    type Error = ASN1Error;

    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_Period_months(el)
    }
}

pub fn _decode_Period_months(el: &X690Element) -> ASN1Result<Period_months> {
    |el: &X690Element| -> ASN1Result<Period_months> {
        match (el.tag_class, el.tag_number) {
            (TagClass::UNIVERSAL, 5) => Ok(Period_months::allMonths(ber_decode_null(&el)?)),
            (TagClass::UNIVERSAL, 17) => {
                Ok(Period_months::intMonth(|el: &X690Element| -> ASN1Result<
                    SET_OF<INTEGER>,
                > {
                    let elements = match el.value.borrow() {
                        X690Encoding::Constructed(children) => children,
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
                    };
                    let mut items: SET_OF<INTEGER> = Vec::with_capacity(elements.len());
                    for el in elements {
                        items.push(ber_decode_integer(el)?);
                    }
                    Ok(items)
                }(&el)?))
            }
            (TagClass::UNIVERSAL, 3) => Ok(Period_months::bitMonth(
                _decode_Period_months_bitMonth(&el)?,
            )),
            _ => Ok(Period_months::_unrecognized(el.clone())),
        }
    }(&el)
}

pub fn _encode_Period_months(value_: &Period_months) -> ASN1Result<X690Element> {
    |value: &Period_months| -> ASN1Result<X690Element> {
        match value {
            Period_months::allMonths(v) => ber_encode_null(&v),
            Period_months::intMonth(v) => |value_: &SET_OF<INTEGER>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(ber_encode_integer(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&v),
            Period_months::bitMonth(v) => _encode_Period_months_bitMonth(&v),
            Period_months::_unrecognized(el) => Ok(el.clone()),
        }
    }(&value_)
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
    ber_decode_enumerated(&el)
}

pub fn _encode_NamedDay_intNamedDays(value_: &NamedDay_intNamedDays) -> ASN1Result<X690Element> {
    ber_encode_enumerated(&value_)
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
    ber_decode_bit_string(&el)
}

pub fn _encode_NamedDay_bitNamedDays(value_: &NamedDay_bitNamedDays) -> ASN1Result<X690Element> {
    ber_encode_bit_string(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// TimeAssertion-between ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct TimeAssertion_between {
    pub startTime: GeneralizedTime,
    pub endTime: OPTIONAL<GeneralizedTime>,
    pub entirely: OPTIONAL<BOOLEAN>,
    pub _unrecognized: Vec<X690Element>,
}
impl TimeAssertion_between {
    fn new(
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
impl TryFrom<X690Element> for TimeAssertion_between {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_TimeAssertion_between(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for TimeAssertion_between {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
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
    |el_: &X690Element| -> ASN1Result<TimeAssertion_between> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_TimeAssertion_between,
            _eal_components_for_TimeAssertion_between,
            _rctl2_components_for_TimeAssertion_between,
        )?;
        let startTime = |el: &X690Element| -> ASN1Result<GeneralizedTime> {
            Ok(ber_decode_generalized_time(&el.inner()?)?)
        }(_components.get("startTime").unwrap())?;
        let endTime: OPTIONAL<GeneralizedTime> = match _components.get("endTime") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<GeneralizedTime> {
                Ok(ber_decode_generalized_time(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let entirely: OPTIONAL<BOOLEAN> = match _components.get("entirely") {
            Some(c_) => Some(ber_decode_boolean(c_)?),
            _ => None,
        };
        Ok(TimeAssertion_between {
            startTime,
            endTime,
            entirely,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_TimeAssertion_between(value_: &TimeAssertion_between) -> ASN1Result<X690Element> {
    |value_: &TimeAssertion_between| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                0,
                Arc::new(X690Encoding::EXPLICIT(Box::new(
                    ber_encode_generalized_time(&v_1)?,
                ))),
            ))
        }(&value_.startTime)?);
        if let Some(v_) = &value_.endTime {
            components_.push(|v_1: &GeneralizedTime| -> ASN1Result<X690Element> {
                Ok(X690Element::new(
                    TagClass::CONTEXT,
                    1,
                    Arc::new(X690Encoding::EXPLICIT(Box::new(
                        ber_encode_generalized_time(&v_1)?,
                    ))),
                ))
            }(&v_)?);
        }
        if let Some(v_) = &value_.entirely {
            if *v_ != TimeAssertion_between::_default_value_for_entirely() {
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
