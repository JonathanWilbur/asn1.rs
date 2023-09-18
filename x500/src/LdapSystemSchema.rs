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

pub mod namingContexts {
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

pub mod altServer {
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

pub mod supportedExtension {
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

pub mod supportedControl {
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

pub mod supportedSASLMechanisms {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = DirectoryString; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_DirectoryString(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_DirectoryString(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_DirectoryString(el)
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

pub mod supportedLDAPVersion {
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

pub mod supportedFeatures {
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
        usage: Some(AttributeUsage_directoryOperation),              /* OBJECT_FIELD_SETTING */
        ldapSyntax: Some(ldapSyntaxDescription().id),                /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("ldapSyntax")])),     /* OBJECT_FIELD_SETTING */
        id: id_soa_ldapSyntaxes(),                                   /* OBJECT_FIELD_SETTING */
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

pub mod ldapSyntaxes {
    /* OBJECT_TYPES */
    use super::*;
    pub type Type = LdapSyntaxDescription; /* OBJECT_FIELD_SETTING OBJECT_TYPE_FIELD_SETTING */
    pub fn _decode_Type(el: &X690Element) -> ASN1Result<Type> {
        _decode_LdapSyntaxDescription(el)
    }
    pub fn _encode_Type(value_: &Type) -> ASN1Result<X690Element> {
        _encode_LdapSyntaxDescription(value_)
    }
    pub fn _validate_Type(el: &X690Element) -> ASN1Result<()> {
        _validate_LdapSyntaxDescription(el)
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
impl TryFrom<&X690Element> for LdapSyntaxDescription {
    type Error = ASN1Error;
    fn try_from(el: &X690Element) -> Result<Self, Self::Error> {
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
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "LdapSyntaxDescription")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_LdapSyntaxDescription,
        _eal_components_for_LdapSyntaxDescription,
        _rctl2_components_for_LdapSyntaxDescription,
    )
    .into_iter();
    let mut _i: usize = 0;
    let mut identifier_: OPTIONAL<OBJECT_IDENTIFIER> = None;
    let mut description_: OPTIONAL<UnboundedDirectoryString> = None;
    let mut _unrecognized: Vec<X690Element> = vec![];
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "identifier" => identifier_ = Some(BER.decode_object_identifier(_el)?),
            "description" => description_ = Some(_decode_UnboundedDirectoryString(_el)?),
            _ => _unrecognized.push(_el.clone()),
        }
    }
    Ok(LdapSyntaxDescription {
        identifier: identifier_.unwrap(),
        description: description_,
        _unrecognized,
    })
}

pub fn _encode_LdapSyntaxDescription(value_: &LdapSyntaxDescription) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(BER.encode_object_identifier(&value_.identifier)?);
    if let Some(v_) = &value_.description {
        components_.push(_encode_UnboundedDirectoryString(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn _validate_LdapSyntaxDescription(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "LdapSyntaxDescription")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_LdapSyntaxDescription,
        _eal_components_for_LdapSyntaxDescription,
        _rctl2_components_for_LdapSyntaxDescription,
    )
    .into_iter();
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "identifier" => BER.validate_object_identifier(_el)?,
            "description" => _validate_UnboundedDirectoryString(_el)?,
            _ => (),
        }
    }
    Ok(())
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat-namingContexts                     OBJECT IDENTIFIER ::= {id-lat 5}
/// ```
///
///
pub fn id_lat_namingContexts() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lat().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat-altServer                          OBJECT IDENTIFIER ::= {id-lat 6}
/// ```
///
///
pub fn id_lat_altServer() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lat().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat-supportedExtension                 OBJECT IDENTIFIER ::= {id-lat 7}
/// ```
///
///
pub fn id_lat_supportedExtension() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lat().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat-supportedControl                   OBJECT IDENTIFIER ::= {id-lat 13}
/// ```
///
///
pub fn id_lat_supportedControl() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lat().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat-supportedSASLMechanisms            OBJECT IDENTIFIER ::= {id-lat 14}
/// ```
///
///
pub fn id_lat_supportedSASLMechanisms() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lat().0, Vec::<u32>::from([14])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat-supportedLDAPVersion               OBJECT IDENTIFIER ::= {id-lat 15}
/// ```
///
///
pub fn id_lat_supportedLDAPVersion() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lat().0, Vec::<u32>::from([15])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa-ldapSyntaxes                       OBJECT IDENTIFIER ::= {id-lat 16}
/// ```
///
///
pub fn id_soa_ldapSyntaxes() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_lat().0, Vec::<u32>::from([16])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oat-supportedFeatures                  OBJECT IDENTIFIER ::= {id-oat 5}
/// ```
///
///
pub fn id_oat_supportedFeatures() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oat().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}
