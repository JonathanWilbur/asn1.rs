#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PasswordPolicy
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PasswordPolicy`.
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
use crate::InformationFramework::*;
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// userPwd	ATTRIBUTE ::= {
///   WITH SYNTAX              UserPwd
///   EQUALITY MATCHING RULE   userPwdMatch
///   SINGLE VALUE             TRUE
///   LDAP-SYNTAX              userPwdDescription.&id
///   LDAP-NAME                {"userPwd"}
///   ID                       id-at-userPwd }
/// ```
///
///
pub fn userPwd() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(userPwdMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(userPwdDescription().id),      /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("userPwd")])), /* OBJECT_FIELD_SETTING */
        id: id_at_userPwd(),                            /* OBJECT_FIELD_SETTING */
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

pub mod userPwd {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UserPwd; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UserPwd(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UserPwd(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UserPwd(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UserPwd  ::=  CHOICE {
///   clear                 UTF8String,
///   encrypted             SEQUENCE {
///     algorithmIdentifier   AlgorithmIdentifier{{SupportedAlgorithms}},
///     encryptedString       OCTET STRING,
///     ...},
///   ...}
/// ```
#[derive(Debug, Clone)]
pub enum UserPwd {
    clear(UTF8String),
    encrypted(UserPwd_encrypted),
    _unrecognized(X690Element), /* CHOICE_ALT_UNRECOGNIZED_EXT */
}

impl TryFrom<&X690Element> for UserPwd {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UserPwd(el)
    }
}

pub fn _decode_UserPwd(el: &X690Element) -> ASN1Result<UserPwd> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 12) => Ok(UserPwd::clear(BER.decode_utf8_string(&el)?)),
        (TagClass::UNIVERSAL, 16) => Ok(UserPwd::encrypted(_decode_UserPwd_encrypted(&el)?)),
        _ => Ok(UserPwd::_unrecognized(el.clone())),
    }
}

pub fn _encode_UserPwd(value_: &UserPwd) -> ASN1Result<X690Element> {
    match value_ {
        UserPwd::clear(v) => BER.encode_utf8_string(&v),
        UserPwd::encrypted(v) => _encode_UserPwd_encrypted(&v),
        UserPwd::_unrecognized(el) => Ok(el.clone()),
    }
}

pub fn _validate_UserPwd(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 12) => BER.validate_utf8_string(&el),
        (TagClass::UNIVERSAL, 16) => _validate_UserPwd_encrypted(&el),
        _ => Ok(()),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdStartTime ATTRIBUTE ::= {
///   WITH SYNTAX              GeneralizedTime
///   EQUALITY MATCHING RULE   generalizedTimeMatch
///   ORDERING MATCHING RULE   generalizedTimeOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              generalizedTime.&id
///   LDAP-NAME                {"pwdStartTime"}
///   ID                       id-oa-pwdStartTime }
/// ```
///
///
pub fn pwdStartTime() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(generalizedTimeMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(generalizedTimeOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                              /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),         /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(generalizedTime().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdStartTime")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdStartTime(),                               /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdStartTime {
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
/// pwdExpiryTime ATTRIBUTE ::= {
///   WITH SYNTAX              GeneralizedTime
///   EQUALITY MATCHING RULE   generalizedTimeMatch
///   ORDERING MATCHING RULE   generalizedTimeOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              generalizedTime.&id
///   LDAP-NAME                {"pwdExpiryTime"}
///   ID                       id-oa-pwdExpiryTime }
/// ```
///
///
pub fn pwdExpiryTime() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(generalizedTimeMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(generalizedTimeOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                              /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),         /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(generalizedTime().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdExpiryTime")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdExpiryTime(),                              /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdExpiryTime {
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
/// pwdEndTime ATTRIBUTE ::= {
///   WITH SYNTAX              GeneralizedTime
///   EQUALITY MATCHING RULE   generalizedTimeMatch
///   ORDERING MATCHING RULE   generalizedTimeOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              generalizedTime.&id
///   LDAP-NAME                {"pwdEndTime"}
///   ID                       id-oa-pwdEndTime }
/// ```
///
///
pub fn pwdEndTime() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(generalizedTimeMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(generalizedTimeOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                              /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),         /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(generalizedTime().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdEndTime")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdEndTime(),                                 /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdEndTime {
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
/// pwdFails ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (0..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    dSAOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdFails"}
///   ID                       id-oa-pwdFails }
/// ```
///
///
pub fn pwdFails() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_dSAOperation),       /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdFails")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdFails(),                           /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdFails {
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
/// pwdFailureTime ATTRIBUTE ::= {
///   WITH SYNTAX              GeneralizedTime
///   EQUALITY MATCHING RULE   generalizedTimeMatch
///   ORDERING MATCHING RULE   generalizedTimeOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    dSAOperation
///   LDAP-SYNTAX              generalizedTime.&id
///   LDAP-NAME                {"pwdFailureTime"}
///   ID                       id-oa-pwdFailureTime }
/// ```
///
///
pub fn pwdFailureTime() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(generalizedTimeMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(generalizedTimeOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                              /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_dSAOperation),               /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(generalizedTime().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdFailureTime")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdFailureTime(),                             /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdFailureTime {
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
/// pwdGracesUsed ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (0..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    dSAOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdGracesUsed"}
///   ID                       id-oa-pwdGracesUsed }
/// ```
///
///
pub fn pwdGracesUsed() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_dSAOperation),       /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdGracesUsed")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdGracesUsed(),                      /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdGracesUsed {
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
/// userPwdHistory ATTRIBUTE ::= pwdHistory{userPwd,userPwdHistoryMatch,id-oa-userPwdHistory}
/// ```
///
///
pub fn userPwdHistory() -> ATTRIBUTE {
    pwdHistory(userPwd(), userPwdHistoryMatch(), id_oa_userPwdHistory())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// userPwdRecentlyExpired ATTRIBUTE ::= pwdRecentlyExpired{userPwd,id-oa-userPwdRecentlyExpired}
/// ```
///
///
pub fn userPwdRecentlyExpired() -> ATTRIBUTE {
    pwdRecentlyExpired(userPwd(), id_oa_userPwdRecentlyExpired())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdModifyEntryAllowed ATTRIBUTE ::= {
///   WITH SYNTAX              BOOLEAN
///   EQUALITY MATCHING RULE   booleanMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              boolean.&id
///   LDAP-NAME                {"pwdModifyEntryAllowed"}
///   ID                       id-oa-pwdModifyEntryAllowed }
/// ```
///
///
pub fn pwdModifyEntryAllowed() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(booleanMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(boolean().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdModifyEntryAllowed")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdModifyEntryAllowed(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdModifyEntryAllowed {
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
/// pwdChangeAllowed ATTRIBUTE ::= {
///   WITH SYNTAX              BOOLEAN
///   EQUALITY MATCHING RULE   booleanMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              boolean.&id
///   LDAP-NAME                {"pwdChangeAllowed"}
///   ID                       id-oa-pwdChangeAllowed }
/// ```
///
///
pub fn pwdChangeAllowed() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(booleanMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(boolean().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdChangeAllowed")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdChangeAllowed(),                   /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdChangeAllowed {
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
/// pwdMaxAge ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (1 .. MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdMaxAge"}
///   ID                       id-oa-pwdMaxAge }
/// ```
///
///
pub fn pwdMaxAge() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdMaxAge")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdMaxAge(),                          /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdMaxAge {
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
/// pwdExpiryAge ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (1 .. MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdExpiryAge"}
///   ID                       id-oa-pwdExpiryAge }
/// ```
///
///
pub fn pwdExpiryAge() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdExpiryAge")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdExpiryAge(),                       /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdExpiryAge {
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
/// pwdMinLength ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (0..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdMinLength"}
///   ID                       id-oa-pwdMinLength }
/// ```
///
///
pub fn pwdMinLength() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdMinLength")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdMinLength(),                       /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdMinLength {
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
/// pwdVocabulary ATTRIBUTE ::= {
///   WITH SYNTAX              PwdVocabulary
///   EQUALITY MATCHING RULE   bitStringMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              pwdVocabularyDescription.&id
///   LDAP-NAME                {"pwdVocabulary"}
///   ID                       id-oa-pwdVocabulary }
/// ```
///
///
pub fn pwdVocabulary() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(bitStringMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                        /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),   /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(pwdVocabularyDescription().id),  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdVocabulary")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdVocabulary(),                        /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdVocabulary {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PwdVocabulary; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PwdVocabulary(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PwdVocabulary(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PwdVocabulary(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PwdVocabulary  ::=   BIT STRING {
///     noDictionaryWords   (0),
///     noPersonNames       (1),
///     noGeographicalNames (2) }
/// ```
pub type PwdVocabulary = BIT_STRING;

pub const PwdVocabulary_noDictionaryWords: BIT = 0; /* LONG_NAMED_BIT */

pub const PwdVocabulary_noPersonNames: BIT = 1; /* LONG_NAMED_BIT */

pub const PwdVocabulary_noGeographicalNames: BIT = 2; /* LONG_NAMED_BIT */

pub fn _decode_PwdVocabulary(el: &X690Element) -> ASN1Result<PwdVocabulary> {
    BER.decode_bit_string(&el)
}

pub fn _encode_PwdVocabulary(value_: &PwdVocabulary) -> ASN1Result<X690Element> {
    BER.encode_bit_string(&value_)
}

pub fn _validate_PwdVocabulary(el: &X690Element) -> ASN1Result<()> {
    BER.validate_bit_string(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdAlphabet ATTRIBUTE ::= {
///   WITH SYNTAX              PwdAlphabet
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              pwdAlphabetDescription.&id
///   LDAP-NAME                {"pwdAlphabet"}
///   ID                       id-oa-pwdAlphabet }
/// ```
///
///
pub fn pwdAlphabet() -> ATTRIBUTE {
    ATTRIBUTE {
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(pwdAlphabetDescription().id),  /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdAlphabet")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdAlphabet(),                        /* OBJECT_FIELD_SETTING */
        derivation: None,
        equality_match: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdAlphabet {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PwdAlphabet; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PwdAlphabet(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PwdAlphabet(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PwdAlphabet(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PwdAlphabet  ::=  SEQUENCE OF UTF8String
/// ```
pub type PwdAlphabet = Vec<UTF8String>; // SequenceOfType

pub fn _decode_PwdAlphabet(el: &X690Element) -> ASN1Result<PwdAlphabet> {
    let elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => return Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PwdAlphabet")),
    };
    let mut items: SEQUENCE_OF<UTF8String> = Vec::with_capacity(elements.len());
    for el in elements.iter() {
        items.push(BER.decode_utf8_string(el)?);
    }
    Ok(items)
}

pub fn _encode_PwdAlphabet(value_: &PwdAlphabet) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(BER.encode_utf8_string(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_PwdAlphabet(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                BER.validate_utf8_string(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PwdAlphabet")),
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdDictionaries ATTRIBUTE ::= {
///   SUBTYPE OF               uri
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"pwdDictionaries"}
///   ID                       id-oa-pwdDictionaries }
/// ```
///
///
pub fn pwdDictionaries() -> ATTRIBUTE {
    ATTRIBUTE {
        derivation: Some(Box::new(uri())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdDictionaries")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdDictionaries(),       /* OBJECT_FIELD_SETTING */
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

pub mod pwdDictionaries {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdExpiryWarning ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (1..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdExpiryWarning"}
///   ID                       id-oa-pwdExpiryWarning }
/// ```
///
///
pub fn pwdExpiryWarning() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdExpiryWarning")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdExpiryWarning(),                   /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdExpiryWarning {
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
/// pwdGraces ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (0..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdGraces"}
///   ID                       id-oa-pwdGraces }
/// ```
///
///
pub fn pwdGraces() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdGraces")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdGraces(),                          /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdGraces {
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
/// pwdFailureDuration ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (0..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdFailureDuration"}
///   ID                       id-oa-pwdFailureDuration }
/// ```
///
///
pub fn pwdFailureDuration() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdFailureDuration")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdFailureDuration(),                 /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdFailureDuration {
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
/// pwdLockoutDuration ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (0..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdLockoutDuration"}
///   ID                       id-oa-pwdLockoutDuration }
/// ```
///
///
pub fn pwdLockoutDuration() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdLockoutDuration")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdLockoutDuration(),                 /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdLockoutDuration {
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
/// pwdMaxFailures ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (1..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdMaxFailures"}
///   ID                       id-oa-pwdMaxFailures }
/// ```
///
///
pub fn pwdMaxFailures() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdMaxFailures")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdMaxFailures(),                     /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdMaxFailures {
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
/// pwdMaxTimeInHistory ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (1..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdMaxTimeInHistory"}
///   ID                       id-oa-pwdMaxTimeInHistory }
/// ```
///
///
pub fn pwdMaxTimeInHistory() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdMaxTimeInHistory")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdMaxTimeInHistory(),                /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdMaxTimeInHistory {
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
/// pwdMinTimeInHistory ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (0..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdMinTimeInHistory"}
///   ID                       id-oa-pwdMinTimeInHistory }
/// ```
///
///
pub fn pwdMinTimeInHistory() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdMinTimeInHistory")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdMinTimeInHistory(),                /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdMinTimeInHistory {
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
/// pwdHistorySlots ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (2..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdHistorySlots"}
///   ID                       id-oa-pwdHistorySlots }
/// ```
///
///
pub fn pwdHistorySlots() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdHistorySlots")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdHistorySlots(),                    /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdHistorySlots {
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
/// pwdRecentlyExpiredDuration ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER (0..MAX)
///   EQUALITY MATCHING RULE   integerMatch
///   ORDERING MATCHING RULE   integerOrderingMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"pwdRecentlyExpiredDuration"}
///   ID                       id-oa-pwdRecentlyExpiredDuration }
/// ```
///
///
pub fn pwdRecentlyExpiredDuration() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(integerMatch())), /* OBJECT_FIELD_SETTING */
        ordering_match: Some(Box::new(integerOrderingMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                      /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),                 /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdRecentlyExpiredDuration")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdRecentlyExpiredDuration(), /* OBJECT_FIELD_SETTING */
        derivation: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdRecentlyExpiredDuration {
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
/// pwdEncAlg ATTRIBUTE ::= {
///   WITH SYNTAX              PwdEncAlg
///   EQUALITY MATCHING RULE   pwdEncAlgMatch
///   SINGLE VALUE             TRUE
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              pwdEncAlgDescription.&id
///   LDAP-NAME                {"pwdEncAlg"}
///   ID                       id-oa-pwdEncAlg }
/// ```
///
///
pub fn pwdEncAlg() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(pwdEncAlgMatch())), /* OBJECT_FIELD_SETTING */
        single_valued: Some(true),                        /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation),   /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(pwdEncAlgDescription().id),      /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdEncAlg")])), /* OBJECT_FIELD_SETTING */
        id: id_oa_pwdEncAlg(),                            /* OBJECT_FIELD_SETTING */
        derivation: None,
        ordering_match: None,
        substrings_match: None,
        collective: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        dummy: Some(false),      /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        no_user_modification: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
        obsolete: Some(false), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
    }
}

pub mod pwdEncAlg {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PwdEncAlg; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PwdEncAlg(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PwdEncAlg(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PwdEncAlg(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PwdEncAlg  ::=  AlgorithmIdentifier{{SupportedAlgorithms}}
/// ```
pub type PwdEncAlg = AlgorithmIdentifier; // DefinedType

pub fn _decode_PwdEncAlg(el: &X690Element) -> ASN1Result<PwdEncAlg> {
    _decode_AlgorithmIdentifier(&el)
}

pub fn _encode_PwdEncAlg(value_: &PwdEncAlg) -> ASN1Result<X690Element> {
    _encode_AlgorithmIdentifier(&value_)
}

pub fn _validate_PwdEncAlg(el: &X690Element) -> ASN1Result<()> {
    _validate_AlgorithmIdentifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// userPwdMatch MATCHING-RULE ::= {
///   SYNTAX       UserPwd
///   LDAP-SYNTAX  userPwdDescription.&id
///   LDAP-NAME    {"userPwdMatch"}
///   ID           id-mr-userPwdMatch }
/// ```
///
///
pub fn userPwdMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(userPwdDescription().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("userPwdMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_userPwdMatch(),                  /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod userPwdMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = UserPwd; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_UserPwd(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_UserPwd(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_UserPwd(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdEncAlgMatch MATCHING-RULE ::= {
///   SYNTAX       PwdEncAlg
///   LDAP-SYNTAX  pwdEncAlgDescription.&id
///   LDAP-NAME    {"pwdEncAlgMatch"}
///   ID           id-mr-pwdEncAlgMatch }
/// ```
///
///
pub fn pwdEncAlgMatch() -> MATCHING_RULE {
    MATCHING_RULE {
        ldapSyntax: Some(pwdEncAlgDescription().id), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("pwdEncAlgMatch")])), /* OBJECT_FIELD_SETTING */
        id: id_mr_pwdEncAlgMatch(),                  /* OBJECT_FIELD_SETTING */
        ParentMatchingRules: None,
        uniqueMatchIndicator: None,
        ldapDesc: None,
    }
}

pub mod pwdEncAlgMatch {
    /* OBJECT_TYPES */
    use super::*;
    pub type AssertionType = PwdEncAlg; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_AssertionType(el: &X690Element) -> ASN1Result<AssertionType> {
        _decode_PwdEncAlg(el)
    }
    pub fn _encode_AssertionType(value_: &AssertionType) -> ASN1Result<X690Element> {
        _encode_PwdEncAlg(value_)
    }
    pub fn _validate_AssertionType(el: &X690Element) -> ASN1Result<()> {
        _validate_PwdEncAlg(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// userPwdHistoryMatch MATCHING-RULE ::= pwdHistoryMatch{userPwd,id-mr-userPwdHistoryMatch}
/// ```
///
///
pub fn userPwdHistoryMatch() -> MATCHING_RULE {
    pwdHistoryMatch(id_mr_userPwdHistoryMatch())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// userPwdDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "User Password Description"
///   DIRECTORY SYNTAX  UserPwd
///   ID                id-asx-userPwdDescription }
/// ```
///
///
pub fn userPwdDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("User Password Description"), /* OBJECT_FIELD_SETTING */
        id: id_asx_userPwdDescription(),                     /* OBJECT_FIELD_SETTING */
    }
}

pub mod userPwdDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = UserPwd; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_UserPwd(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_UserPwd(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_UserPwd(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdVocabularyDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "Password Vocabulary Description"
///   DIRECTORY SYNTAX  PwdVocabulary
///   ID                id-asx-pwdVocabularyDescription }
/// ```
///
///
pub fn pwdVocabularyDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Password Vocabulary Description"), /* OBJECT_FIELD_SETTING */
        id: id_asx_pwdVocabularyDescription(),                     /* OBJECT_FIELD_SETTING */
    }
}

pub mod pwdVocabularyDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PwdVocabulary; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PwdVocabulary(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PwdVocabulary(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PwdVocabulary(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdAlphabetDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "Password Alphabet Description"
///   DIRECTORY SYNTAX  PwdAlphabet
///   ID                id-asx-pwdAlphabetDescription }
/// ```
///
///
pub fn pwdAlphabetDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Password Alphabet Description"), /* OBJECT_FIELD_SETTING */
        id: id_asx_pwdAlphabetDescription(),                     /* OBJECT_FIELD_SETTING */
    }
}

pub mod pwdAlphabetDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PwdAlphabet; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PwdAlphabet(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PwdAlphabet(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PwdAlphabet(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pwdEncAlgDescription SYNTAX-NAME ::= {
///   LDAP-DESC         "Password Alphabet Description"
///   DIRECTORY SYNTAX  PwdEncAlg
///   ID                id-asx-pwdEncAlgDescription }
/// ```
///
///
pub fn pwdEncAlgDescription() -> SYNTAX_NAME {
    SYNTAX_NAME {
        ldapDesc: String::from("Password Alphabet Description"), /* OBJECT_FIELD_SETTING */
        id: id_asx_pwdEncAlgDescription(),                       /* OBJECT_FIELD_SETTING */
    }
}

pub mod pwdEncAlgDescription {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = PwdEncAlg; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_PwdEncAlg(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_PwdEncAlg(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_PwdEncAlg(el)
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at-userPwd                    OBJECT IDENTIFIER ::= {id-at 85}
/// ```
///
///
#[inline]
pub fn id_at_userPwd () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_at(), 85).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdStartTime               OBJECT IDENTIFIER ::= {id-oa 22}
/// ```
///
///
#[inline]
pub fn id_oa_pwdStartTime () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 22).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdExpiryTime              OBJECT IDENTIFIER ::= {id-oa 23}
/// ```
///
///
#[inline]
pub fn id_oa_pwdExpiryTime () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 23).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdEndTime                 OBJECT IDENTIFIER ::= {id-oa 24}
/// ```
///
///
#[inline]
pub fn id_oa_pwdEndTime () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 24).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdFails                   OBJECT IDENTIFIER ::= {id-oa 25}
/// ```
///
///
#[inline]
pub fn id_oa_pwdFails () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 25).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdFailureTime             OBJECT IDENTIFIER ::= {id-oa 26}
/// ```
///
///
#[inline]
pub fn id_oa_pwdFailureTime () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 26).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdGracesUsed              OBJECT IDENTIFIER ::= {id-oa 27}
/// ```
///
///
#[inline]
pub fn id_oa_pwdGracesUsed () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 27).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-userPwdHistory             OBJECT IDENTIFIER ::= {id-oa 28}
/// ```
///
///
#[inline]
pub fn id_oa_userPwdHistory () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 28).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-userPwdRecentlyExpired     OBJECT IDENTIFIER ::= {id-oa 29}
/// ```
///
///
#[inline]
pub fn id_oa_userPwdRecentlyExpired () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 29).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdModifyEntryAllowed      OBJECT IDENTIFIER ::= {id-oa 30}
/// ```
///
///
#[inline]
pub fn id_oa_pwdModifyEntryAllowed () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 30).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdChangeAllowed           OBJECT IDENTIFIER ::= {id-oa 31}
/// ```
///
///
#[inline]
pub fn id_oa_pwdChangeAllowed () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 31).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdMaxAge                  OBJECT IDENTIFIER ::= {id-oa 32}
/// ```
///
///
#[inline]
pub fn id_oa_pwdMaxAge () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 32).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdExpiryAge               OBJECT IDENTIFIER ::= {id-oa 33}
/// ```
///
///
#[inline]
pub fn id_oa_pwdExpiryAge () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 33).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdMinLength               OBJECT IDENTIFIER ::= {id-oa 34}
/// ```
///
///
#[inline]
pub fn id_oa_pwdMinLength () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 34).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdVocabulary              OBJECT IDENTIFIER ::= {id-oa 35}
/// ```
///
///
#[inline]
pub fn id_oa_pwdVocabulary () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 35).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdAlphabet                OBJECT IDENTIFIER ::= {id-oa 36}
/// ```
///
///
#[inline]
pub fn id_oa_pwdAlphabet () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 36).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdDictionaries            OBJECT IDENTIFIER ::= {id-oa 37}
/// ```
///
///
#[inline]
pub fn id_oa_pwdDictionaries () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 37).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdExpiryWarning           OBJECT IDENTIFIER ::= {id-oa 38}
/// ```
///
///
#[inline]
pub fn id_oa_pwdExpiryWarning () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 38).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdGraces                  OBJECT IDENTIFIER ::= {id-oa 39}
/// ```
///
///
#[inline]
pub fn id_oa_pwdGraces () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 39).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdFailureDuration         OBJECT IDENTIFIER ::= {id-oa 40}
/// ```
///
///
#[inline]
pub fn id_oa_pwdFailureDuration () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 40).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdLockoutDuration         OBJECT IDENTIFIER ::= {id-oa 41}
/// ```
///
///
#[inline]
pub fn id_oa_pwdLockoutDuration () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 41).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdMaxFailures             OBJECT IDENTIFIER ::= {id-oa 42}
/// ```
///
///
#[inline]
pub fn id_oa_pwdMaxFailures () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 42).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdMaxTimeInHistory        OBJECT IDENTIFIER ::= {id-oa 43}
/// ```
///
///
#[inline]
pub fn id_oa_pwdMaxTimeInHistory () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 43).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdMinTimeInHistory        OBJECT IDENTIFIER ::= {id-oa 44}
/// ```
///
///
#[inline]
pub fn id_oa_pwdMinTimeInHistory () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 44).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdHistorySlots            OBJECT IDENTIFIER ::= {id-oa 45}
/// ```
///
///
#[inline]
pub fn id_oa_pwdHistorySlots () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 45).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdRecentlyExpiredDuration OBJECT IDENTIFIER ::= {id-oa 46}
/// ```
///
///
#[inline]
pub fn id_oa_pwdRecentlyExpiredDuration () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 46).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa-pwdEncAlg                  OBJECT IDENTIFIER ::= {id-oa 47}
/// ```
///
///
#[inline]
pub fn id_oa_pwdEncAlg () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_oa(), 47).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-userPwdMatch               OBJECT IDENTIFIER ::= {id-mr 71}
/// ```
///
///
#[inline]
pub fn id_mr_userPwdMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 71).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-userPwdHistoryMatch        OBJECT IDENTIFIER ::= {id-mr 72}
/// ```
///
///
#[inline]
pub fn id_mr_userPwdHistoryMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 72).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr-pwdEncAlgMatch             OBJECT IDENTIFIER ::= {id-mr 73}
/// ```
///
///
#[inline]
pub fn id_mr_pwdEncAlgMatch () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_mr(), 73).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-userPwdDescription        OBJECT IDENTIFIER ::= {id-asx 0}
/// ```
///
///
#[inline]
pub fn id_asx_userPwdDescription () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_asx(), 0).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-pwdVocabularyDescription  OBJECT IDENTIFIER ::= {id-asx 1}
/// ```
///
///
#[inline]
pub fn id_asx_pwdVocabularyDescription () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_asx(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-pwdAlphabetDescription    OBJECT IDENTIFIER ::= {id-asx 2}
/// ```
///
///
#[inline]
pub fn id_asx_pwdAlphabetDescription () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_asx(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-pwdEncAlgDescription      OBJECT IDENTIFIER ::= {id-asx 3}
/// ```
///
///
#[inline]
pub fn id_asx_pwdEncAlgDescription () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_asx(), 3).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// UserPwd-encrypted ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
#[derive(Debug, Clone)]
pub struct UserPwd_encrypted {
    pub algorithmIdentifier: AlgorithmIdentifier,
    pub encryptedString: OCTET_STRING,
    pub _unrecognized: Vec<X690Element>,
}
impl UserPwd_encrypted {
    pub fn new(
        algorithmIdentifier: AlgorithmIdentifier,
        encryptedString: OCTET_STRING,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        UserPwd_encrypted {
            algorithmIdentifier,
            encryptedString,
            _unrecognized,
        }
    }
}
impl TryFrom<&X690Element> for UserPwd_encrypted {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
        _decode_UserPwd_encrypted(el)
    }
}

pub const _rctl1_components_for_UserPwd_encrypted: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithmIdentifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 16)),
        None,
        None,
    ),
    ComponentSpec::new(
        "encryptedString",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 4)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_UserPwd_encrypted: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_UserPwd_encrypted: &[ComponentSpec; 0] = &[];

pub fn _decode_UserPwd_encrypted(el: &X690Element) -> ASN1Result<UserPwd_encrypted> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserPwd-encrypted")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UserPwd_encrypted,
        _eal_components_for_UserPwd_encrypted,
        _rctl2_components_for_UserPwd_encrypted,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut algorithmIdentifier_: OPTIONAL<AlgorithmIdentifier> = None;
    let mut encryptedString_: OPTIONAL<OCTET_STRING> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => algorithmIdentifier_ = Some(_decode_AlgorithmIdentifier(_el)?),
            "encryptedString" => encryptedString_ = Some(BER.decode_octet_string(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(UserPwd_encrypted {
        algorithmIdentifier: algorithmIdentifier_.unwrap(),
        encryptedString: encryptedString_.unwrap(),
        _unrecognized,
    })
}

pub fn _encode_UserPwd_encrypted(value_: &UserPwd_encrypted) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(_encode_AlgorithmIdentifier(&value_.algorithmIdentifier)?);
    components_.push(BER.encode_octet_string(&value_.encryptedString)?);
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_UserPwd_encrypted(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "UserPwd-encrypted")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_UserPwd_encrypted,
        _eal_components_for_UserPwd_encrypted,
        _rctl2_components_for_UserPwd_encrypted,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithmIdentifier" => _validate_AlgorithmIdentifier(_el)?,
            "encryptedString" => BER.validate_octet_string(_el)?,
            _ => (),
        }
    }
    Ok(())
}
