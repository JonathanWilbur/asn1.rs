#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # LdapSystemSchema
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `LdapSystemSchema`.
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
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use asn1::*;
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// namingContexts ATTRIBUTE ::= {
///   WITH SYNTAX              DistinguishedName
///   USAGE                    dSAOperation
///   LDAP-SYNTAX              dn.&id
///   LDAP-NAME                {"namingContexts"}
///   ID                       id-lat-namingContexts }
/// ```
///
///
pub fn namingContexts() -> ATTRIBUTE {
    ATTRIBUTE {
        usage: Some(AttributeUsage_dSAOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(dn().id),                /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("namingContexts")])), /* OBJECT_FIELD_SETTING */
        id: id_lat_namingContexts(),              /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// altServer ATTRIBUTE ::= {
///   WITH SYNTAX              IA5String
///   USAGE                    dSAOperation
///   LDAP-SYNTAX              ia5String.&id
///   LDAP-NAME                {"altServer"}
///   ID                       id-lat-altServer }
/// ```
///
///
pub fn altServer() -> ATTRIBUTE {
    ATTRIBUTE {
        usage: Some(AttributeUsage_dSAOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(ia5String().id),         /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("altServer")])), /* OBJECT_FIELD_SETTING */
        id: id_lat_altServer(),                   /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// supportedExtension ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   USAGE                    dSAOperation
///   LDAP-SYNTAX              oid.&id
///   LDAP-NAME                {"supportedExtension"}
///   ID                       id-lat-supportedExtension }
/// ```
///
///
pub fn supportedExtension() -> ATTRIBUTE {
    ATTRIBUTE {
        usage: Some(AttributeUsage_dSAOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),               /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("supportedExtension")])), /* OBJECT_FIELD_SETTING */
        id: id_lat_supportedExtension(),          /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// supportedControl ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   USAGE                    dSAOperation
///   LDAP-SYNTAX              oid.&id
///   LDAP-NAME                {"supportedControl"}
///   ID                       id-lat-supportedControl }
/// ```
///
///
pub fn supportedControl() -> ATTRIBUTE {
    ATTRIBUTE {
        usage: Some(AttributeUsage_dSAOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),               /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("supportedControl")])), /* OBJECT_FIELD_SETTING */
        id: id_lat_supportedControl(),            /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// supportedSASLMechanisms ATTRIBUTE ::= {
///   WITH SYNTAX              DirectoryString{ub-saslMechanism}
///   USAGE                    dSAOperation
///   LDAP-SYNTAX              directoryString.&id
///   LDAP-NAME                {"supportedSASLMechanisms"}
///   ID                       id-lat-supportedSASLMechanisms }
/// ```
///
///
pub fn supportedSASLMechanisms() -> ATTRIBUTE {
    ATTRIBUTE {
        usage: Some(AttributeUsage_dSAOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(directoryString().id),   /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("supportedSASLMechanisms")])), /* OBJECT_FIELD_SETTING */
        id: id_lat_supportedSASLMechanisms(), /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// supportedLDAPVersion ATTRIBUTE ::= {
///   WITH SYNTAX              INTEGER
///   USAGE                    dSAOperation
///   LDAP-SYNTAX              integer.&id
///   LDAP-NAME                {"supportedLDAPVersion"}
///   ID                       id-lat-supportedLDAPVersion }
/// ```
///
///
pub fn supportedLDAPVersion() -> ATTRIBUTE {
    ATTRIBUTE {
        usage: Some(AttributeUsage_dSAOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(integer().id),           /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("supportedLDAPVersion")])), /* OBJECT_FIELD_SETTING */
        id: id_lat_supportedLDAPVersion(), /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// supportedFeatures ATTRIBUTE ::= {
///   WITH SYNTAX              OBJECT IDENTIFIER
///   USAGE                    dSAOperation
///   LDAP-SYNTAX              oid.&id
///   LDAP-NAME                {"supportedFeatures"}
///   ID                       id-oat-supportedFeatures }
/// ```
///
///
pub fn supportedFeatures() -> ATTRIBUTE {
    ATTRIBUTE {
        usage: Some(AttributeUsage_dSAOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(oid().id),               /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("supportedFeatures")])), /* OBJECT_FIELD_SETTING */
        id: id_oat_supportedFeatures(),           /* OBJECT_FIELD_SETTING */
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

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldapSyntaxes ATTRIBUTE ::= {
///   WITH SYNTAX              LdapSyntaxDescription
///   EQUALITY MATCHING RULE   objectIdentifierFirstComponentMatch
///   USAGE                    directoryOperation
///   LDAP-SYNTAX              ldapSyntaxDescription.&id
///   LDAP-NAME                {"ldapSyntax"}
///   ID                       id-soa-ldapSyntaxes }
/// ```
///
///
pub fn ldapSyntaxes() -> ATTRIBUTE {
    ATTRIBUTE {
        equality_match: Some(Box::new(objectIdentifierFirstComponentMatch())), /* OBJECT_FIELD_SETTING */
        usage: Some(AttributeUsage_directoryOperation), /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(ldapSyntaxDescription().id),   /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("ldapSyntax")])), /* OBJECT_FIELD_SETTING */
        id: id_soa_ldapSyntaxes(),                      /* OBJECT_FIELD_SETTING */
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
/// LdapSyntaxDescription ::= SEQUENCE {
///   identifier               SYNTAX-NAME.&id,
///   description              UnboundedDirectoryString OPTIONAL,
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct LdapSyntaxDescription {
    pub identifier: OBJECT_IDENTIFIER,
    pub description: OPTIONAL<UnboundedDirectoryString>,
    pub _unrecognized: Vec<X690Element>,
}
impl LdapSyntaxDescription {
    pub fn new(
        identifier: OBJECT_IDENTIFIER,
        description: OPTIONAL<UnboundedDirectoryString>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        LdapSyntaxDescription {
            identifier,
            description,
            _unrecognized,
        }
    }
}
impl TryFrom<X690Element> for LdapSyntaxDescription {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_LdapSyntaxDescription(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for LdapSyntaxDescription {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_LdapSyntaxDescription(el)
    }
}

pub const _rctl1_components_for_LdapSyntaxDescription: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "identifier",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
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
];

pub const _rctl2_components_for_LdapSyntaxDescription: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_LdapSyntaxDescription: &[ComponentSpec; 0] = &[];

pub fn _decode_LdapSyntaxDescription(el: &X690Element) -> ASN1Result<LdapSyntaxDescription> {
    |el_: &X690Element| -> ASN1Result<LdapSyntaxDescription> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_LdapSyntaxDescription,
            _eal_components_for_LdapSyntaxDescription,
            _rctl2_components_for_LdapSyntaxDescription,
        )?;
        let identifier = ber_decode_object_identifier(_components.get("identifier").unwrap())?;
        let description: OPTIONAL<UnboundedDirectoryString> = match _components.get("description") {
            Some(c_) => Some(_decode_UnboundedDirectoryString(c_)?),
            _ => None,
        };
        Ok(LdapSyntaxDescription {
            identifier,
            description,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_LdapSyntaxDescription(value_: &LdapSyntaxDescription) -> ASN1Result<X690Element> {
    |value_: &LdapSyntaxDescription| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.identifier)?);
        if let Some(v_) = &value_.description {
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
/// id-lat-namingContexts                     OBJECT IDENTIFIER ::= {id-lat 5}
/// ```
///
///
pub fn id_lat_namingContexts() -> OBJECT_IDENTIFIER {
    [id_lat(), Vec::<u32>::from([5])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat-altServer                          OBJECT IDENTIFIER ::= {id-lat 6}
/// ```
///
///
pub fn id_lat_altServer() -> OBJECT_IDENTIFIER {
    [id_lat(), Vec::<u32>::from([6])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat-supportedExtension                 OBJECT IDENTIFIER ::= {id-lat 7}
/// ```
///
///
pub fn id_lat_supportedExtension() -> OBJECT_IDENTIFIER {
    [id_lat(), Vec::<u32>::from([7])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat-supportedControl                   OBJECT IDENTIFIER ::= {id-lat 13}
/// ```
///
///
pub fn id_lat_supportedControl() -> OBJECT_IDENTIFIER {
    [id_lat(), Vec::<u32>::from([13])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat-supportedSASLMechanisms            OBJECT IDENTIFIER ::= {id-lat 14}
/// ```
///
///
pub fn id_lat_supportedSASLMechanisms() -> OBJECT_IDENTIFIER {
    [id_lat(), Vec::<u32>::from([14])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat-supportedLDAPVersion               OBJECT IDENTIFIER ::= {id-lat 15}
/// ```
///
///
pub fn id_lat_supportedLDAPVersion() -> OBJECT_IDENTIFIER {
    [id_lat(), Vec::<u32>::from([15])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-ldapSyntaxes                       OBJECT IDENTIFIER ::= {id-lat 16}
/// ```
///
///
pub fn id_soa_ldapSyntaxes() -> OBJECT_IDENTIFIER {
    [id_lat(), Vec::<u32>::from([16])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oat-supportedFeatures                  OBJECT IDENTIFIER ::= {id-oat 5}
/// ```
///
///
pub fn id_oat_supportedFeatures() -> OBJECT_IDENTIFIER {
    [id_oat(), Vec::<u32>::from([5])].concat() // OID_GETTER
}
