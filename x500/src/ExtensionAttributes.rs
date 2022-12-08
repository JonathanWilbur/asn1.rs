#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # ExtensionAttributes
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `ExtensionAttributes`.
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
use crate::AttributeCertificateDefinitions::*;
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
/// ExtensionAttribute ::= SEQUENCE {
///   type            ATTRIBUTE.&id,
///   value           SET SIZE (0..1) OF SEQUENCE {
///     mandatory  [0]  BOOLEAN DEFAULT FALSE,
///     critical   [1]  BOOLEAN DEFAULT FALSE,
///     ext        [2]  EXTENSION.&ExtnType,
///     ... },
///   ... }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ExtensionAttribute {
    pub type_: OBJECT_IDENTIFIER,
    pub value: Vec<ExtensionAttribute_value_Item>,
    pub _unrecognized: Vec<X690Element>,
}
impl ExtensionAttribute {
    fn new(
        type_: OBJECT_IDENTIFIER,
        value: Vec<ExtensionAttribute_value_Item>,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ExtensionAttribute {
            type_,
            value,
            _unrecognized,
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
        "type",
        false,
        TagSelector::tag((TagClass::UNIVERSAL, 6)),
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
        let type_ = ber_decode_object_identifier(_components.get("type").unwrap())?;
        let value = |el: &X690Element| -> ASN1Result<SET_OF<ExtensionAttribute_value_Item>> {
            let elements = match el.value.borrow() {
                X690Encoding::Constructed(children) => children,
                _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
            };
            let mut items: SET_OF<ExtensionAttribute_value_Item> =
                Vec::with_capacity(elements.len());
            for el in elements {
                items.push(_decode_ExtensionAttribute_value_Item(el)?);
            }
            Ok(items)
        }(_components.get("value").unwrap())?;
        Ok(ExtensionAttribute {
            type_,
            value,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ExtensionAttribute(value_: &ExtensionAttribute) -> ASN1Result<X690Element> {
    |value_: &ExtensionAttribute| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(12);
        components_.push(ber_encode_object_identifier(&value_.type_)?);
        components_.push(
            |value_: &SET_OF<ExtensionAttribute_value_Item>| -> ASN1Result<X690Element> {
                let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
                for v in value_ {
                    children.push(_encode_ExtensionAttribute_value_Item(&v)?);
                }
                Ok(X690Element::new(
                    TagClass::UNIVERSAL,
                    ASN1_UNIVERSAL_TAG_NUMBER_SET_OF,
                    Arc::new(X690Encoding::Constructed(children)),
                ))
            }(&value_.value)?,
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

// %FIXME%: COULD_NOT_COMPILE_ASSIGNMENT extensionSyntax PARAMETERIZATION_UNSUPPORTED

/// ### ASN.1 Definition:
///
/// ```asn1
/// a-authorityKeyIdentifier ATTRIBUTE ::= {
///   WITH SYNTAX       authorityKeyIdentifier.&ExtnType
///   LDAP-SYNTAX       id-asx-authorityKeyIdentifier
///   LDAP-NAME         {"Authority Key Identifier"}
///   ID                id-ce-a-authorityKeyIdentifier }
/// ```
///
///
pub fn a_authorityKeyIdentifier() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_authorityKeyIdentifier()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Authority Key Identifier")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_authorityKeyIdentifier(), /* OBJECT_FIELD_SETTING */
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
/// a-keyUsage ATTRIBUTE ::= {
///   WITH SYNTAX       keyUsage.&ExtnType
///   LDAP-SYNTAX       id-asx-keyUsage
///   LDAP-NAME         {"Key Usage"}
///   ID                id-ce-a-keyUsage }
/// ```
///
///
pub fn a_keyUsage() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_keyUsage()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Key Usage")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_keyUsage(),              /* OBJECT_FIELD_SETTING */
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
/// a-extKeyUsage ATTRIBUTE ::= {
///   WITH SYNTAX       extKeyUsage.&ExtnType
///   LDAP-SYNTAX       id-asx-extKeyUsage
///   LDAP-NAME         {"Extended Key Usage"}
///   ID                id-ce-a-extKeyUsage }
/// ```
///
///
pub fn a_extKeyUsage() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_extKeyUsage()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Extended Key Usage")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_extKeyUsage(),              /* OBJECT_FIELD_SETTING */
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
/// a-privateKeyUsagePeriod ATTRIBUTE ::= {
///   WITH SYNTAX       privateKeyUsagePeriod.&ExtnType
///   LDAP-SYNTAX       id-asx-privateKeyUsagePeriod
///   LDAP-NAME         {"Private Key Usage Period"}
///   ID                id-ce-a-privateKeyUsagePeriod }
/// ```
///
///
pub fn a_privateKeyUsagePeriod() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_privateKeyUsagePeriod()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Private Key Usage Period")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_privateKeyUsagePeriod(), /* OBJECT_FIELD_SETTING */
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
/// a-certificatePolicies ATTRIBUTE ::= {
///   WITH SYNTAX       certificatePolicies.&ExtnType
///   LDAP-SYNTAX       id-asx-certificatePolicies
///   LDAP-NAME         {"Certificate Policies"}
///   ID                id-ce-a-certificatePolicies }
/// ```
///
///
pub fn a_certificatePolicies() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_certificatePolicies()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Certificate Policies")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_certificatePolicies(), /* OBJECT_FIELD_SETTING */
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
/// a-policyMappings ATTRIBUTE ::= {
///   WITH SYNTAX       policyMappings.&ExtnType
///   LDAP-SYNTAX       id-asx-policyMappings
///   LDAP-NAME         {"Policy Mappings"}
///   ID                id-ce-a-policyMappings }
/// ```
///
///
pub fn a_policyMappings() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_policyMappings()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Policy Mappings")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_policyMappings(),              /* OBJECT_FIELD_SETTING */
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
/// a-authorizationValidation ATTRIBUTE ::= {
///   WITH SYNTAX       authorizationValidation.&ExtnType
///   LDAP-SYNTAX       id-asx-authorizationValidation
///   LDAP-NAME         {"Authorization Validation"}
///   ID                id-ce-a-authorizationValidation }
/// ```
///
///
pub fn a_authorizationValidation() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_authorizationValidation()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Authorization Validation")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_authorizationValidation(), /* OBJECT_FIELD_SETTING */
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
/// a-subjectAltName ATTRIBUTE ::= {
///   WITH SYNTAX       subjectAltName.&ExtnType
///   LDAP-SYNTAX       id-asx-subjectAltName
///   LDAP-NAME         {"Subject Alternative Name"}
///   ID                id-ce-a-subjectAltName }
/// ```
///
///
pub fn a_subjectAltName() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_subjectAltName()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Subject Alternative Name")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_subjectAltName(), /* OBJECT_FIELD_SETTING */
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
/// a-issuerAltName ATTRIBUTE ::= {
///   WITH SYNTAX       issuerAltName.&ExtnType
///   LDAP-SYNTAX       id-asx-issuerAltName
///   LDAP-NAME         {"Issuer Alternative Name"}
///   ID                id-ce-a-issuerAltName }
/// ```
///
///
pub fn a_issuerAltName() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_issuerAltName()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Issuer Alternative Name")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_issuerAltName(), /* OBJECT_FIELD_SETTING */
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
/// a-subjectDirectoryAttributes ATTRIBUTE ::= {
///   WITH SYNTAX       subjectDirectoryAttributes.&ExtnType
///   LDAP-SYNTAX       id-asx-subjectDirectoryAttributes
///   LDAP-NAME         {"Subject Directory Attributes"}
///   ID                id-ce-a-subjectDirectoryAttributes }
/// ```
///
///
pub fn a_subjectDirectoryAttributes() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_subjectDirectoryAttributes()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Subject Directory Attributes")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_subjectDirectoryAttributes(), /* OBJECT_FIELD_SETTING */
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
/// a-basicConstraints ATTRIBUTE ::= {
///   WITH SYNTAX       basicConstraints.&ExtnType
///   LDAP-SYNTAX       id-asx-basicConstraints
///   LDAP-NAME         {"Basic Constraints"}
///   ID                id-ce-a-basicConstraints }
/// ```
///
///
pub fn a_basicConstraints() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_basicConstraints()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Basic Constraints")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_basicConstraints(),              /* OBJECT_FIELD_SETTING */
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
/// a-nameConstraints ATTRIBUTE ::= {
///   WITH SYNTAX       policyConstraints.&ExtnType
///   LDAP-SYNTAX       id-asx-nameConstraints
///   LDAP-NAME         {"Name Constraints"}
///   ID                id-ce-a-nameConstraints }
/// ```
///
///
pub fn a_nameConstraints() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_nameConstraints()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Name Constraints")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_nameConstraints(),              /* OBJECT_FIELD_SETTING */
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
/// a-policyConstraints ATTRIBUTE ::= {
///   WITH SYNTAX       policyConstraints.&ExtnType
///   LDAP-SYNTAX       id-asx-policyConstraints
///   LDAP-NAME         {"Policy Constraints"}
///   ID                id-ce-a-policyConstraints }
/// ```
///
///
pub fn a_policyConstraints() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_policyConstraints()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Policy Constraints")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_policyConstraints(),              /* OBJECT_FIELD_SETTING */
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
/// a-cRLNumber ATTRIBUTE ::= {
///   WITH SYNTAX       cRLNumber.&ExtnType
///   LDAP-SYNTAX       id-asx-cRLNumber
///   LDAP-NAME         {"CRL Number"}
///   ID                id-ce-a-cRLNumber}
/// ```
///
///
pub fn a_cRLNumber() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_cRLNumber()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("CRL Number")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_cRLNumber(),              /* OBJECT_FIELD_SETTING */
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
/// a-statusReferrals ATTRIBUTE ::= {
///   WITH SYNTAX       statusReferrals.&ExtnType
///   LDAP-SYNTAX       id-asx-statusReferrals
///   LDAP-NAME         {"Status Referrals"}
///   ID                id-ce-a-statusReferrals}
/// ```
///
///
pub fn a_statusReferrals() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_statusReferrals()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Status Referrals")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_statusReferrals(),              /* OBJECT_FIELD_SETTING */
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
/// a-cRLStreamIdentifier ATTRIBUTE ::= {
///   WITH SYNTAX       cRLStreamIdentifier.&ExtnType
///   LDAP-SYNTAX       id-asx-cRLStreamIdentifier
///   LDAP-NAME         {"CRL stream identifier"}
///   ID                id-ce-a-cRLStreamIdentifier}
/// ```
///
///
pub fn a_cRLStreamIdentifier() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_cRLStreamIdentifier()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("CRL stream identifier")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_cRLStreamIdentifier(), /* OBJECT_FIELD_SETTING */
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
/// a-orderedList ATTRIBUTE ::= {
///   WITH SYNTAX       orderedList.&ExtnType
///   LDAP-SYNTAX       id-asx-orderedList
///   LDAP-NAME         {"Ordered list"}
///   ID                id-ce-a-orderedList}
/// ```
///
///
pub fn a_orderedList() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_orderedList()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Ordered list")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_orderedList(),              /* OBJECT_FIELD_SETTING */
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
/// a-deltaInfo ATTRIBUTE ::= {
///   WITH SYNTAX       deltaInfo.&ExtnType
///   LDAP-SYNTAX       id-asx-deltaInfo
///   LDAP-NAME         {"Delta information"}
///   ID                id-ce-a-deltaInfo}
/// ```
///
///
pub fn a_deltaInfo() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_deltaInfo()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Delta information")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_deltaInfo(),              /* OBJECT_FIELD_SETTING */
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
/// a-toBeRevoked ATTRIBUTE ::= {
///   WITH SYNTAX       toBeRevoked.&ExtnType
///   LDAP-SYNTAX       id-asx-toBeRevoked
///   LDAP-NAME         {"To be revoked"}
///   ID                id-ce-a-toBeRevoked}
/// ```
///
///
pub fn a_toBeRevoked() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_toBeRevoked()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("To be revoked")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_toBeRevoked(),              /* OBJECT_FIELD_SETTING */
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
/// a-revokedGroups ATTRIBUTE ::= {
///   WITH SYNTAX       revokedGroups.&ExtnType
///   LDAP-SYNTAX       id-asx-revokedGroups
///   LDAP-NAME         {"Revoked group of certificates"}
///   ID                id-ce-a-revokedGroups}
/// ```
///
///
pub fn a_revokedGroups() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_revokedGroups()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Revoked group of certificates")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_revokedGroups(), /* OBJECT_FIELD_SETTING */
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
/// a-expiredCertsOnCRL ATTRIBUTE ::= {
///   WITH SYNTAX       expiredCertsOnCRL.&ExtnType
///   LDAP-SYNTAX       id-asx-expiredCertsOnCRL
///   LDAP-NAME         {"Expired certificates on CRL"}
///   ID                id-ce-a-expiredCertsOnCRL}
/// ```
///
///
pub fn a_expiredCertsOnCRL() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_expiredCertsOnCRL()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Expired certificates on CRL")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_expiredCertsOnCRL(), /* OBJECT_FIELD_SETTING */
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
/// a-reasonCode ATTRIBUTE ::= {
///   WITH SYNTAX       reasonCode.&ExtnType
///   LDAP-SYNTAX       id-asx-reasonCode
///   LDAP-NAME         {"Reason code"}
///   ID                id-ce-a-reasonCode}
/// ```
///
///
pub fn a_reasonCode() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_reasonCode()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Reason code")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_reasonCode(),              /* OBJECT_FIELD_SETTING */
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
/// a-holdInstructionCode ATTRIBUTE ::= {
///   WITH SYNTAX       holdInstructionCode.&ExtnType
///   LDAP-SYNTAX       id-asx-holdInstructionCode
///   LDAP-NAME         {"Hold instruction code"}
///   ID                id-ce-a-holdInstructionCode}
/// ```
///
///
pub fn a_holdInstructionCode() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_holdInstructionCode()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Hold instruction code")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_holdInstructionCode(), /* OBJECT_FIELD_SETTING */
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
/// a-invalidityDate ATTRIBUTE ::= {
///   WITH SYNTAX       invalidityDate.&ExtnType
///   LDAP-SYNTAX       id-asx-invalidityDate
///   LDAP-NAME         {"Invalidity date"}
///   ID                id-ce-a-invalidityDate}
/// ```
///
///
pub fn a_invalidityDate() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_invalidityDate()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Invalidity date")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_invalidityDate(),              /* OBJECT_FIELD_SETTING */
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
/// a-cRLDistributionPoints ATTRIBUTE ::= {
///   WITH SYNTAX       cRLDistributionPoints.&ExtnType
///   LDAP-SYNTAX       id-asx-cRLDistributionPoints
///   LDAP-NAME         {"CRL distribution points"}
///   ID                id-ce-a-cRLDistributionPoints}
/// ```
///
///
pub fn a_cRLDistributionPoints() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_cRLDistributionPoints()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("CRL distribution points")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_cRLDistributionPoints(), /* OBJECT_FIELD_SETTING */
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
/// a-issuingDistributionPoint ATTRIBUTE ::= {
///   WITH SYNTAX       issuingDistributionPoint.&ExtnType
///   LDAP-SYNTAX       id-asx-issuingDistributionPoint
///   LDAP-NAME         {"Issuing distribution point"}
///   ID                id-ce-a-issuingDistributionPoint}
/// ```
///
///
pub fn a_issuingDistributionPoint() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_issuingDistributionPoint()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Issuing distribution point")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_issuingDistributionPoint(), /* OBJECT_FIELD_SETTING */
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
/// a-certificateIssuer ATTRIBUTE ::= {
///   WITH SYNTAX       certificateIssuer.&ExtnType
///   LDAP-SYNTAX       id-asx-certificateIssuer
///   LDAP-NAME         {"Certificate issuer"}
///   ID                id-ce-a-certificateIssuer}
/// ```
///
///
pub fn a_certificateIssuer() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_certificateIssuer()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Certificate issuer")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_certificateIssuer(),              /* OBJECT_FIELD_SETTING */
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
/// a-deltaCRLIndicator ATTRIBUTE ::= {
///   WITH SYNTAX       deltaCRLIndicator.&ExtnType
///   LDAP-SYNTAX       id-asx-deltaCRLIndicator
///   LDAP-NAME         {"Delta CRL indicator"}
///   ID                id-ce-a-deltaCRLIndicator}
/// ```
///
///
pub fn a_deltaCRLIndicator() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_deltaCRLIndicator()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Delta CRL indicator")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_deltaCRLIndicator(),              /* OBJECT_FIELD_SETTING */
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
/// a-baseUpdateTime ATTRIBUTE ::= {
///   WITH SYNTAX       baseUpdateTime.&ExtnType
///   LDAP-SYNTAX       id-asx-baseUpdateTime
///   LDAP-NAME         {"Base update time"}
///   ID                id-ce-a-baseUpdateTime}
/// ```
///
///
pub fn a_baseUpdateTime() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_baseUpdateTime()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Base update time")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_baseUpdateTime(),              /* OBJECT_FIELD_SETTING */
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
/// a-freshestCRL ATTRIBUTE ::= {
///   WITH SYNTAX       freshestCRL.&ExtnType
///   LDAP-SYNTAX       id-asx-freshestCRL
///   LDAP-NAME         {"Freshest CRL"}
///   ID                id-ce-a-freshestCRL}
/// ```
///
///
pub fn a_freshestCRL() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_freshestCRL()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Freshest CRL")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_freshestCRL(),              /* OBJECT_FIELD_SETTING */
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
/// a-timeSpecification ATTRIBUTE ::= {
///   WITH SYNTAX       timeSpecification.&ExtnType
///   LDAP-SYNTAX       id-asx-timeSpecification
///   LDAP-NAME         {"Time specification"}
///   ID                id-ce-a-timeSpecification}
/// ```
///
///
pub fn a_timeSpecification() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_timeSpecification()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Time specification")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_timeSpecification(),              /* OBJECT_FIELD_SETTING */
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
/// a-targetingInformation ATTRIBUTE ::= {
///   WITH SYNTAX       targetingInformation.&ExtnType
///   LDAP-SYNTAX       id-asx-targetingInformation
///   LDAP-NAME         {"Targeting information"}
///   ID                id-ce-a-targetingInformation}
/// ```
///
///
pub fn a_targetingInformation() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_targetingInformation()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Targeting information")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_targetingInformation(), /* OBJECT_FIELD_SETTING */
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
/// a-userNotice ATTRIBUTE ::= {
///   WITH SYNTAX       userNotice.&ExtnType
///   LDAP-SYNTAX       id-asx-userNotice
///   LDAP-NAME         {"User notice"}
///   ID                id-ce-a-userNotice}
/// ```
///
///
pub fn a_userNotice() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_userNotice()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("User notice")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_userNotice(),              /* OBJECT_FIELD_SETTING */
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
/// a-acceptablePrivilegePolicies ATTRIBUTE ::= {
///   WITH SYNTAX       acceptablePrivilegePolicies.&ExtnType
///   LDAP-SYNTAX       id-asx-acceptablePrivilegePolicies
///   LDAP-NAME         {"Acceptable Privilege Policies"}
///   ID                id-ce-a-acceptablePrivilegePolicies}
/// ```
///
///
pub fn a_acceptablePrivilegePolicies() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_acceptablePrivilegePolicies()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Acceptable Privilege Policies")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_acceptablePrivilegePolicies(), /* OBJECT_FIELD_SETTING */
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
/// a-singleUse ATTRIBUTE ::= {
///   WITH SYNTAX       singleUse.&ExtnType
///   LDAP-SYNTAX       id-asx-singleUse
///   LDAP-NAME         {"Single use"}
///   ID                id-ce-a-singleUse}
/// ```
///
///
pub fn a_singleUse() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_singleUse()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Single use")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_singleUse(),              /* OBJECT_FIELD_SETTING */
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
/// a-groupAC ATTRIBUTE ::= {
///   WITH SYNTAX       groupAC.&ExtnType
///   LDAP-SYNTAX       id-asx-groupAC
///   LDAP-NAME         {"Group attribute certificate"}
///   ID                id-ce-a-groupAC}
/// ```
///
///
pub fn a_groupAC() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_groupAC()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Group attribute certificate")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_groupAC(), /* OBJECT_FIELD_SETTING */
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
/// a-noRevAvail ATTRIBUTE ::= {
///   WITH SYNTAX       noRevAvail.&ExtnType
///   LDAP-SYNTAX       id-asx-noRevAvail
///   LDAP-NAME         {"No revocation information available"}
///   ID                id-ce-a-noRevAvail}
/// ```
///
///
pub fn a_noRevAvail() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_noRevAvail()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from(
            "No revocation information available",
        )])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_noRevAvail(),              /* OBJECT_FIELD_SETTING */
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
/// a-sOAIdentifier ATTRIBUTE ::= {
///   WITH SYNTAX       sOAIdentifier.&ExtnType
///   LDAP-SYNTAX       id-asx-sOAIdentifier
///   LDAP-NAME         {"SOA identifier"}
///   ID                id-ce-a-sOAIdentifier}
/// ```
///
///
pub fn a_sOAIdentifier() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_sOAIdentifier()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("SOA identifier")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_sOAIdentifier(),              /* OBJECT_FIELD_SETTING */
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
/// a-attributeDescriptor ATTRIBUTE ::= {
///   WITH SYNTAX       attributeDescriptor.&ExtnType
///   LDAP-SYNTAX       id-asx-attributeDescriptor
///   LDAP-NAME         {"Attribute descriptor"}
///   ID                id-ce-a-attributeDescriptor}
/// ```
///
///
pub fn a_attributeDescriptor() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_attributeDescriptor()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Attribute descriptor")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_attributeDescriptor(), /* OBJECT_FIELD_SETTING */
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
/// a-roleSpecCertIdentifier ATTRIBUTE ::= {
///   WITH SYNTAX       roleSpecCertIdentifier.&ExtnType
///   LDAP-SYNTAX       id-asx-roleSpecCertIdentifier
///   LDAP-NAME         {"Role specification certificate identifier"}
///   ID                id-ce-a-roleSpecCertIdentifier}
/// ```
///
///
pub fn a_roleSpecCertIdentifier() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_roleSpecCertIdentifier()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from(
            "Role specification certificate identifier",
        )])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_roleSpecCertIdentifier(),              /* OBJECT_FIELD_SETTING */
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
/// a-basicAttConstraints ATTRIBUTE ::= {
///   WITH SYNTAX       basicAttConstraints.&ExtnType
///   LDAP-SYNTAX       id-asx-basicAttConstraints
///   LDAP-NAME         {"Basic attribute constraints"}
///   ID                id-ce-a-basicAttConstraints}
/// ```
///
///
pub fn a_basicAttConstraints() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_basicAttConstraints()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Basic attribute constraints")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_basicAttConstraints(), /* OBJECT_FIELD_SETTING */
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
/// a-delegatedNameConstraints ATTRIBUTE ::= {
///   WITH SYNTAX       delegatedNameConstraints.&ExtnType
///   LDAP-SYNTAX       id-asx-delegatedNameConstraints
///   LDAP-NAME         {"Delegated name constraints"}
///   ID                id-ce-a-delegatedNameConstraints}
/// ```
///
///
pub fn a_delegatedNameConstraints() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_delegatedNameConstraints()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Delegated name constraints")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_delegatedNameConstraints(), /* OBJECT_FIELD_SETTING */
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
/// a-acceptableCertPolicies ATTRIBUTE ::= {
///   WITH SYNTAX       acceptableCertPolicies.&ExtnType
///   LDAP-SYNTAX       id-asx-acceptableCertPolicies
///   LDAP-NAME         {"Acceptable certificate policiesGroup attribute certificate"}
///   ID                id-ce-a-acceptableCertPolicies}
/// ```
///
///
pub fn a_acceptableCertPolicies() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_acceptableCertPolicies()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from(
            "Acceptable certificate policiesGroup attribute certificate",
        )])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_acceptableCertPolicies(),              /* OBJECT_FIELD_SETTING */
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
/// a-authorityAttributeIdentifier ATTRIBUTE ::= {
///   WITH SYNTAX       authorityAttributeIdentifier.&ExtnType
///   LDAP-SYNTAX       id-asx-authorityAttributeIdentifier
///   LDAP-NAME         {"Authority attribute identifier"}
///   ID                id-ce-a-authorityAttributeIdentifier}
/// ```
///
///
pub fn a_authorityAttributeIdentifier() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_authorityAttributeIdentifier()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Authority attribute identifier")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_authorityAttributeIdentifier(), /* OBJECT_FIELD_SETTING */
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
/// a-indirectIssuer ATTRIBUTE ::= {
///   WITH SYNTAX       indirectIssuer.&ExtnType
///   LDAP-SYNTAX       id-asx-indirectIssuer
///   LDAP-NAME         {"Indirect issuer"}
///   ID                id-ce-a-indirectIssuer}
/// ```
///
///
pub fn a_indirectIssuer() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_indirectIssuer()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Indirect issuer")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_indirectIssuer(),              /* OBJECT_FIELD_SETTING */
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
/// a-issuedOnBehalfOf ATTRIBUTE ::= {
///   WITH SYNTAX       issuedOnBehalfOf.&ExtnType
///   LDAP-SYNTAX       id-asx-issuedOnBehalfOf
///   LDAP-NAME         {"Issued on behalf of"}
///   ID                id-ce-a-issuedOnBehalfOf}
/// ```
///
///
pub fn a_issuedOnBehalfOf() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_issuedOnBehalfOf()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Issued on behalf of")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_issuedOnBehalfOf(),              /* OBJECT_FIELD_SETTING */
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
/// a-noAssertion ATTRIBUTE ::= {
///   WITH SYNTAX       noAssertion.&ExtnType
///   LDAP-SYNTAX       id-asx-noAssertion
///   LDAP-NAME         {"No assertion"}
///   ID                id-ce-a-noAssertion}
/// ```
///
///
pub fn a_noAssertion() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_noAssertion()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("No assertion")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_noAssertion(),              /* OBJECT_FIELD_SETTING */
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
/// a-allowedAttributeAssignments ATTRIBUTE ::= {
///   WITH SYNTAX       allowedAttributeAssignments.&ExtnType
///   LDAP-SYNTAX       id-asx-allowedAttributeAssignments
///   LDAP-NAME         {"Allowed attribute assignments"}
///   ID                id-ce-a-allowedAttributeAssignments}
/// ```
///
///
pub fn a_allowedAttributeAssignments() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_allowedAttributeAssignments()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Allowed attribute assignments")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_allowedAttributeAssignments(), /* OBJECT_FIELD_SETTING */
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
/// a-attributeMappings ATTRIBUTE ::= {
///   WITH SYNTAX       attributeMappings.&ExtnType
///   LDAP-SYNTAX       id-asx-attributeMappings
///   LDAP-NAME         {"Attribute mappings"}
///   ID                id-ce-a-attributeMappings}
/// ```
///
///
pub fn a_attributeMappings() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_attributeMappings()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Attribute mappings")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_attributeMappings(),              /* OBJECT_FIELD_SETTING */
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
/// a-holderNameConstraints ATTRIBUTE ::= {
///   WITH SYNTAX       holderNameConstraints.&ExtnType
///   LDAP-SYNTAX       id-asx-holderNameConstraints
///   LDAP-NAME         {"Holder name constraints"}
///   ID                id-ce-a-holderNameConstraints}
/// ```
///
///
pub fn a_holderNameConstraints() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_holderNameConstraints()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Holder name constraints")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_holderNameConstraints(), /* OBJECT_FIELD_SETTING */
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
/// a-aAissuingDistributionPoint ATTRIBUTE ::= {
///   WITH SYNTAX       aAissuingDistributionPoint.&ExtnType
///   LDAP-SYNTAX       id-asx-aAissuingDistributionPoint
///   LDAP-NAME         {"AA issuing distribution point"}
///   ID                id-ce-a-aAissuingDistributionPoint}
/// ```
///
///
pub fn a_aAissuingDistributionPoint() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_aAissuingDistributionPoint()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("AA issuing distribution point")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_aAissuingDistributionPoint(), /* OBJECT_FIELD_SETTING */
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
/// a-protRestrict ATTRIBUTE ::= {
///   WITH SYNTAX       protRestrict.&ExtnType
///   LDAP-SYNTAX       id-asx-protRestrict
///   LDAP-NAME         {"Protocol restriction"}
///   ID                id-ce-a-protRestrict}
/// ```
///
///
pub fn a_protRestrict() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_protRestrict()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Protocol restriction")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_protRestrict(), /* OBJECT_FIELD_SETTING */
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
/// a-subjectAltPublicKeyInfo ATTRIBUTE ::= {
///   WITH SYNTAX       subjectAltPublicKeyInfo.&ExtnType
///   LDAP-SYNTAX       id-asx-subjectAltPublicKeyInfo
///   LDAP-NAME         {"Subject alternative public key info"}
///   ID                id-ce-a-subjectAltPublicKeyInfo}
/// ```
///
///
pub fn a_subjectAltPublicKeyInfo() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_subjectAltPublicKeyInfo()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from(
            "Subject alternative public key info",
        )])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_subjectAltPublicKeyInfo(),              /* OBJECT_FIELD_SETTING */
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
/// a-altSignatureAlgorithm ATTRIBUTE ::= {
///   WITH SYNTAX       altSignatureAlgorithm.&ExtnType
///   LDAP-SYNTAX       id-asx-altSignatureAlgorithm
///   LDAP-NAME         {"Alternative signature algorithm"}
///   ID                id-ce-a-altSignatureAlgorithm}
/// ```
///
///
pub fn a_altSignatureAlgorithm() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_altSignatureAlgorithm()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Alternative signature algorithm")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_altSignatureAlgorithm(), /* OBJECT_FIELD_SETTING */
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
/// a-altSignatureValue ATTRIBUTE ::= {
///   WITH SYNTAX       altSignatureValue.&ExtnType
///   LDAP-SYNTAX       id-asx-altSignatureValue
///   LDAP-NAME         {"Alternative signature value"}
///   ID                id-ce-a-altSignatureValue}
/// ```
///
///
pub fn a_altSignatureValue() -> ATTRIBUTE {
    ATTRIBUTE {
        ldapSyntax: Some(id_asx_altSignatureValue()), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("Alternative signature value")])), /* OBJECT_FIELD_SETTING */
        id: id_ce_a_altSignatureValue(), /* OBJECT_FIELD_SETTING */
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
/// id-ce-a-subjectDirectoryAttributes         OBJECT IDENTIFIER ::= {id-ce 9 1}
/// ```
///
///
pub fn id_ce_a_subjectDirectoryAttributes() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([9, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-subjectKeyIdentifier               OBJECT IDENTIFIER ::= {id-ce 14 1}
/// ```
///
///
pub fn id_ce_a_subjectKeyIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([14, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-keyUsage                           OBJECT IDENTIFIER ::= {id-ce 15 1}
/// ```
///
///
pub fn id_ce_a_keyUsage() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([15, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-privateKeyUsagePeriod              OBJECT IDENTIFIER ::= {id-ce 16 1}
/// ```
///
///
pub fn id_ce_a_privateKeyUsagePeriod() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([16, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-subjectAltName                     OBJECT IDENTIFIER ::= {id-ce 17 1}
/// ```
///
///
pub fn id_ce_a_subjectAltName() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([17, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-issuerAltName                      OBJECT IDENTIFIER ::= {id-ce 18 1}
/// ```
///
///
pub fn id_ce_a_issuerAltName() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([18, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-basicConstraints                   OBJECT IDENTIFIER ::= {id-ce 19 1}
/// ```
///
///
pub fn id_ce_a_basicConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([19, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-cRLNumber                          OBJECT IDENTIFIER ::= {id-ce 20 1}
/// ```
///
///
pub fn id_ce_a_cRLNumber() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([20, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-reasonCode                         OBJECT IDENTIFIER ::= {id-ce 21 1}
/// ```
///
///
pub fn id_ce_a_reasonCode() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([21, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-holdInstructionCode                OBJECT IDENTIFIER ::= {id-ce 23 1}
/// ```
///
///
pub fn id_ce_a_holdInstructionCode() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([23, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-invalidityDate                     OBJECT IDENTIFIER ::= {id-ce 24 1}
/// ```
///
///
pub fn id_ce_a_invalidityDate() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([24, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-deltaCRLIndicator                  OBJECT IDENTIFIER ::= {id-ce 27 1}
/// ```
///
///
pub fn id_ce_a_deltaCRLIndicator() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([27, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-issuingDistributionPoint           OBJECT IDENTIFIER ::= {id-ce 28 1}
/// ```
///
///
pub fn id_ce_a_issuingDistributionPoint() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([28, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-certificateIssuer                  OBJECT IDENTIFIER ::= {id-ce 29 1}
/// ```
///
///
pub fn id_ce_a_certificateIssuer() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([29, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-nameConstraints                    OBJECT IDENTIFIER ::= {id-ce 30 1}
/// ```
///
///
pub fn id_ce_a_nameConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([30, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-cRLDistributionPoints              OBJECT IDENTIFIER ::= {id-ce 31 1}
/// ```
///
///
pub fn id_ce_a_cRLDistributionPoints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([31, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-certificatePolicies                OBJECT IDENTIFIER ::= {id-ce 32 1}
/// ```
///
///
pub fn id_ce_a_certificatePolicies() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([32, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-policyMappings                     OBJECT IDENTIFIER ::= {id-ce 33 1}
/// ```
///
///
pub fn id_ce_a_policyMappings() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([33, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-authorityKeyIdentifier             OBJECT IDENTIFIER ::= {id-ce 35 1}
/// ```
///
///
pub fn id_ce_a_authorityKeyIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([35, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-policyConstraints                  OBJECT IDENTIFIER ::= {id-ce 36 1}
/// ```
///
///
pub fn id_ce_a_policyConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([36, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-extKeyUsage                        OBJECT IDENTIFIER ::= {id-ce 37 1}
/// ```
///
///
pub fn id_ce_a_extKeyUsage() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([37, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-authorityAttributeIdentifier       OBJECT IDENTIFIER ::= {id-ce 38 1}
/// ```
///
///
pub fn id_ce_a_authorityAttributeIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([38, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-roleSpecCertIdentifier             OBJECT IDENTIFIER ::= {id-ce 39 1}
/// ```
///
///
pub fn id_ce_a_roleSpecCertIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([39, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-cRLStreamIdentifier                OBJECT IDENTIFIER ::= {id-ce 40 1}
/// ```
///
///
pub fn id_ce_a_cRLStreamIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([40, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-basicAttConstraints                OBJECT IDENTIFIER ::= {id-ce 41 1}
/// ```
///
///
pub fn id_ce_a_basicAttConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([41, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-delegatedNameConstraints           OBJECT IDENTIFIER ::= {id-ce 42 1}
/// ```
///
///
pub fn id_ce_a_delegatedNameConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([42, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-timeSpecification                  OBJECT IDENTIFIER ::= {id-ce 43 1}
/// ```
///
///
pub fn id_ce_a_timeSpecification() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([43, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-cRLScope                           OBJECT IDENTIFIER ::= {id-ce 44 1}
/// ```
///
///
pub fn id_ce_a_cRLScope() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([44, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-statusReferrals                    OBJECT IDENTIFIER ::= {id-ce 45 1}
/// ```
///
///
pub fn id_ce_a_statusReferrals() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([45, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-freshestCRL                        OBJECT IDENTIFIER ::= {id-ce 46 1}
/// ```
///
///
pub fn id_ce_a_freshestCRL() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([46, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-orderedList                        OBJECT IDENTIFIER ::= {id-ce 47 1}
/// ```
///
///
pub fn id_ce_a_orderedList() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([47, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-attributeDescriptor                OBJECT IDENTIFIER ::= {id-ce 48 1}
/// ```
///
///
pub fn id_ce_a_attributeDescriptor() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([48, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-userNotice                         OBJECT IDENTIFIER ::= {id-ce 49 1}
/// ```
///
///
pub fn id_ce_a_userNotice() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([49, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-sOAIdentifier                      OBJECT IDENTIFIER ::= {id-ce 50 1}
/// ```
///
///
pub fn id_ce_a_sOAIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([50, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-baseUpdateTime                     OBJECT IDENTIFIER ::= {id-ce 51 1}
/// ```
///
///
pub fn id_ce_a_baseUpdateTime() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([51, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-acceptableCertPolicies             OBJECT IDENTIFIER ::= {id-ce 52 1}
/// ```
///
///
pub fn id_ce_a_acceptableCertPolicies() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([52, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-deltaInfo                          OBJECT IDENTIFIER ::= {id-ce 53 1}
/// ```
///
///
pub fn id_ce_a_deltaInfo() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([53, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-inhibitAnyPolicy                   OBJECT IDENTIFIER ::= {id-ce 54 1}
/// ```
///
///
pub fn id_ce_a_inhibitAnyPolicy() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([54, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-targetingInformation               OBJECT IDENTIFIER ::= {id-ce 55 1}
/// ```
///
///
pub fn id_ce_a_targetingInformation() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([55, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-noRevAvail                         OBJECT IDENTIFIER ::= {id-ce 56 1}
/// ```
///
///
pub fn id_ce_a_noRevAvail() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([56, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-acceptablePrivilegePolicies        OBJECT IDENTIFIER ::= {id-ce 57 1}
/// ```
///
///
pub fn id_ce_a_acceptablePrivilegePolicies() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([57, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-toBeRevoked                        OBJECT IDENTIFIER ::= {id-ce 58 1}
/// ```
///
///
pub fn id_ce_a_toBeRevoked() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([58, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-revokedGroups                      OBJECT IDENTIFIER ::= {id-ce 59 1}
/// ```
///
///
pub fn id_ce_a_revokedGroups() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([59, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-expiredCertsOnCRL                  OBJECT IDENTIFIER ::= {id-ce 60 1}
/// ```
///
///
pub fn id_ce_a_expiredCertsOnCRL() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([60, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-indirectIssuer                     OBJECT IDENTIFIER ::= {id-ce 61 1}
/// ```
///
///
pub fn id_ce_a_indirectIssuer() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([61, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-noAssertion                        OBJECT IDENTIFIER ::= {id-ce 62 1}
/// ```
///
///
pub fn id_ce_a_noAssertion() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([62, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-aAissuingDistributionPoint         OBJECT IDENTIFIER ::= {id-ce 63 1}
/// ```
///
///
pub fn id_ce_a_aAissuingDistributionPoint() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([63, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-issuedOnBehalfOf                   OBJECT IDENTIFIER ::= {id-ce 64 1}
/// ```
///
///
pub fn id_ce_a_issuedOnBehalfOf() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([64, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-singleUse                          OBJECT IDENTIFIER ::= {id-ce 65 1}
/// ```
///
///
pub fn id_ce_a_singleUse() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([65, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-groupAC                            OBJECT IDENTIFIER ::= {id-ce 66 1}
/// ```
///
///
pub fn id_ce_a_groupAC() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([66, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-allowedAttributeAssignments        OBJECT IDENTIFIER ::= {id-ce 67 1}
/// ```
///
///
pub fn id_ce_a_allowedAttributeAssignments() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([67, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-attributeMappings                  OBJECT IDENTIFIER ::= {id-ce 68 1}
/// ```
///
///
pub fn id_ce_a_attributeMappings() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([68, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-holderNameConstraints              OBJECT IDENTIFIER ::= {id-ce 69 1}
/// ```
///
///
pub fn id_ce_a_holderNameConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([69, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-authorizationValidation            OBJECT IDENTIFIER ::= {id-ce 70 1}
/// ```
///
///
pub fn id_ce_a_authorizationValidation() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([70, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-protRestrict                       OBJECT IDENTIFIER ::= {id-ce 71 1}
/// ```
///
///
pub fn id_ce_a_protRestrict() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([71, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-subjectAltPublicKeyInfo            OBJECT IDENTIFIER ::= {id-ce 72 1}
/// ```
///
///
pub fn id_ce_a_subjectAltPublicKeyInfo() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([72, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-altSignatureAlgorithm              OBJECT IDENTIFIER ::= {id-ce 73 1}
/// ```
///
///
pub fn id_ce_a_altSignatureAlgorithm() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([73, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce-a-altSignatureValue                  OBJECT IDENTIFIER ::= {id-ce 74 1}
/// ```
///
///
pub fn id_ce_a_altSignatureValue() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([74, 1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-subjectDirectoryAttributes          OBJECT IDENTIFIER ::= {id-ce 9 2}
/// ```
///
///
pub fn id_asx_subjectDirectoryAttributes() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([9, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-subjectKeyIdentifier                OBJECT IDENTIFIER ::= {id-ce 14 2}
/// ```
///
///
pub fn id_asx_subjectKeyIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([14, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-keyUsage                            OBJECT IDENTIFIER ::= {id-ce 15 2}
/// ```
///
///
pub fn id_asx_keyUsage() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([15, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-privateKeyUsagePeriod               OBJECT IDENTIFIER ::= {id-ce 16 2}
/// ```
///
///
pub fn id_asx_privateKeyUsagePeriod() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([16, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-subjectAltName                      OBJECT IDENTIFIER ::= {id-ce 17 2}
/// ```
///
///
pub fn id_asx_subjectAltName() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([17, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-issuerAltName                       OBJECT IDENTIFIER ::= {id-ce 18 2}
/// ```
///
///
pub fn id_asx_issuerAltName() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([18, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-basicConstraints                    OBJECT IDENTIFIER ::= {id-ce 19 2}
/// ```
///
///
pub fn id_asx_basicConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([19, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-cRLNumber                           OBJECT IDENTIFIER ::= {id-ce 20 2}
/// ```
///
///
pub fn id_asx_cRLNumber() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([20, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-reasonCode                          OBJECT IDENTIFIER ::= {id-ce 21 2}
/// ```
///
///
pub fn id_asx_reasonCode() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([21, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-holdInstructionCode                 OBJECT IDENTIFIER ::= {id-ce 23 2}
/// ```
///
///
pub fn id_asx_holdInstructionCode() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([23, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-invalidityDate                      OBJECT IDENTIFIER ::= {id-ce 24 2}
/// ```
///
///
pub fn id_asx_invalidityDate() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([24, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-deltaCRLIndicator                   OBJECT IDENTIFIER ::= {id-ce 27 2}
/// ```
///
///
pub fn id_asx_deltaCRLIndicator() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([27, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-issuingDistributionPoint            OBJECT IDENTIFIER ::= {id-ce 28 2}
/// ```
///
///
pub fn id_asx_issuingDistributionPoint() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([28, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-certificateIssuer                   OBJECT IDENTIFIER ::= {id-ce 29 2}
/// ```
///
///
pub fn id_asx_certificateIssuer() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([29, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-nameConstraints                     OBJECT IDENTIFIER ::= {id-ce 30 2}
/// ```
///
///
pub fn id_asx_nameConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([30, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-cRLDistributionPoints               OBJECT IDENTIFIER ::= {id-ce 31 2}
/// ```
///
///
pub fn id_asx_cRLDistributionPoints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([31, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-certificatePolicies                 OBJECT IDENTIFIER ::= {id-ce 32 2}
/// ```
///
///
pub fn id_asx_certificatePolicies() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([32, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-policyMappings                      OBJECT IDENTIFIER ::= {id-ce 33 2}
/// ```
///
///
pub fn id_asx_policyMappings() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([33, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-authorityKeyIdentifier              OBJECT IDENTIFIER ::= {id-ce 35 2}
/// ```
///
///
pub fn id_asx_authorityKeyIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([35, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-policyConstraints                   OBJECT IDENTIFIER ::= {id-ce 36 2}
/// ```
///
///
pub fn id_asx_policyConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([36, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-extKeyUsage                         OBJECT IDENTIFIER ::= {id-ce 37 2}
/// ```
///
///
pub fn id_asx_extKeyUsage() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([37, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-authorityAttributeIdentifier        OBJECT IDENTIFIER ::= {id-ce 38 2}
/// ```
///
///
pub fn id_asx_authorityAttributeIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([38, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-roleSpecCertIdentifier              OBJECT IDENTIFIER ::= {id-ce 39 2}
/// ```
///
///
pub fn id_asx_roleSpecCertIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([39, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-cRLStreamIdentifier                 OBJECT IDENTIFIER ::= {id-ce 40 2}
/// ```
///
///
pub fn id_asx_cRLStreamIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([40, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-basicAttConstraints                 OBJECT IDENTIFIER ::= {id-ce 41 2}
/// ```
///
///
pub fn id_asx_basicAttConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([41, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-delegatedNameConstraints            OBJECT IDENTIFIER ::= {id-ce 42 2}
/// ```
///
///
pub fn id_asx_delegatedNameConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([42, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-timeSpecification                   OBJECT IDENTIFIER ::= {id-ce 43 2}
/// ```
///
///
pub fn id_asx_timeSpecification() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([43, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-cRLScope                            OBJECT IDENTIFIER ::= {id-ce 44 2}
/// ```
///
///
pub fn id_asx_cRLScope() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([44, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-statusReferrals                     OBJECT IDENTIFIER ::= {id-ce 45 2}
/// ```
///
///
pub fn id_asx_statusReferrals() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([45, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-freshestCRL                         OBJECT IDENTIFIER ::= {id-ce 46 2}
/// ```
///
///
pub fn id_asx_freshestCRL() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([46, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-orderedList                         OBJECT IDENTIFIER ::= {id-ce 47 2}
/// ```
///
///
pub fn id_asx_orderedList() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([47, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-attributeDescriptor                 OBJECT IDENTIFIER ::= {id-ce 48 2}
/// ```
///
///
pub fn id_asx_attributeDescriptor() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([48, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-userNotice                          OBJECT IDENTIFIER ::= {id-ce 49 2}
/// ```
///
///
pub fn id_asx_userNotice() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([49, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-sOAIdentifier                       OBJECT IDENTIFIER ::= {id-ce 50 2}
/// ```
///
///
pub fn id_asx_sOAIdentifier() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([50, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-baseUpdateTime                      OBJECT IDENTIFIER ::= {id-ce 51 2}
/// ```
///
///
pub fn id_asx_baseUpdateTime() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([51, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-acceptableCertPolicies              OBJECT IDENTIFIER ::= {id-ce 52 2}
/// ```
///
///
pub fn id_asx_acceptableCertPolicies() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([52, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-deltaInfo                           OBJECT IDENTIFIER ::= {id-ce 53 2}
/// ```
///
///
pub fn id_asx_deltaInfo() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([53, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-inhibitAnyPolicy                    OBJECT IDENTIFIER ::= {id-ce 54 2}
/// ```
///
///
pub fn id_asx_inhibitAnyPolicy() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([54, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-targetingInformation                OBJECT IDENTIFIER ::= {id-ce 55 2}
/// ```
///
///
pub fn id_asx_targetingInformation() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([55, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-noRevAvail                          OBJECT IDENTIFIER ::= {id-ce 56 2}
/// ```
///
///
pub fn id_asx_noRevAvail() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([56, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-acceptablePrivilegePolicies         OBJECT IDENTIFIER ::= {id-ce 57 2}
/// ```
///
///
pub fn id_asx_acceptablePrivilegePolicies() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([57, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-toBeRevoked                         OBJECT IDENTIFIER ::= {id-ce 58 2}
/// ```
///
///
pub fn id_asx_toBeRevoked() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([58, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-revokedGroups                       OBJECT IDENTIFIER ::= {id-ce 59 2}
/// ```
///
///
pub fn id_asx_revokedGroups() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([59, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-expiredCertsOnCRL                   OBJECT IDENTIFIER ::= {id-ce 60 2}
/// ```
///
///
pub fn id_asx_expiredCertsOnCRL() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([60, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-indirectIssuer                      OBJECT IDENTIFIER ::= {id-ce 61 2}
/// ```
///
///
pub fn id_asx_indirectIssuer() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([61, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-noAssertion                         OBJECT IDENTIFIER ::= {id-ce 62 2}
/// ```
///
///
pub fn id_asx_noAssertion() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([62, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-aAissuingDistributionPoint          OBJECT IDENTIFIER ::= {id-ce 63 2}
/// ```
///
///
pub fn id_asx_aAissuingDistributionPoint() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([63, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-issuedOnBehalfOf                    OBJECT IDENTIFIER ::= {id-ce 64 2}
/// ```
///
///
pub fn id_asx_issuedOnBehalfOf() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([64, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-singleUse                           OBJECT IDENTIFIER ::= {id-ce 65 2}
/// ```
///
///
pub fn id_asx_singleUse() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([65, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-groupAC                             OBJECT IDENTIFIER ::= {id-ce 66 2}
/// ```
///
///
pub fn id_asx_groupAC() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([66, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-allowedAttributeAssignments         OBJECT IDENTIFIER ::= {id-ce 67 2}
/// ```
///
///
pub fn id_asx_allowedAttributeAssignments() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([67, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-attributeMappings                   OBJECT IDENTIFIER ::= {id-ce 68 2}
/// ```
///
///
pub fn id_asx_attributeMappings() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([68, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-holderNameConstraints               OBJECT IDENTIFIER ::= {id-ce 69 2}
/// ```
///
///
pub fn id_asx_holderNameConstraints() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([69, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-authorizationValidation             OBJECT IDENTIFIER ::= {id-ce 70 2}
/// ```
///
///
pub fn id_asx_authorizationValidation() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([70, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-protRestrict                        OBJECT IDENTIFIER ::= {id-ce 71 2}
/// ```
///
///
pub fn id_asx_protRestrict() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([71, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-subjectAltPublicKeyInfo             OBJECT IDENTIFIER ::= {id-ce 72 2}
/// ```
///
///
pub fn id_asx_subjectAltPublicKeyInfo() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([72, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-altSignatureAlgorithm               OBJECT IDENTIFIER ::= {id-ce 73 2}
/// ```
///
///
pub fn id_asx_altSignatureAlgorithm() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([73, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx-altSignatureValue                   OBJECT IDENTIFIER ::= {id-ce 74 2}
/// ```
///
///
pub fn id_asx_altSignatureValue() -> OBJECT_IDENTIFIER {
    [id_ce(), Vec::<u32>::from([74, 2])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ExtensionAttribute-value-Item ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct ExtensionAttribute_value_Item {
    pub mandatory: OPTIONAL<BOOLEAN>,
    pub critical: OPTIONAL<BOOLEAN>,
    pub ext: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl ExtensionAttribute_value_Item {
    fn new(
        mandatory: OPTIONAL<BOOLEAN>,
        critical: OPTIONAL<BOOLEAN>,
        ext: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        ExtensionAttribute_value_Item {
            mandatory,
            critical,
            ext,
            _unrecognized,
        }
    }
    pub fn _default_value_for_mandatory() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_critical() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for ExtensionAttribute_value_Item {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_ExtensionAttribute_value_Item(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for ExtensionAttribute_value_Item {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_ExtensionAttribute_value_Item(el)
    }
}

pub const _rctl1_components_for_ExtensionAttribute_value_Item: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "mandatory",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "critical",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "ext",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_ExtensionAttribute_value_Item: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_ExtensionAttribute_value_Item: &[ComponentSpec; 0] = &[];

pub fn _decode_ExtensionAttribute_value_Item(
    el: &X690Element,
) -> ASN1Result<ExtensionAttribute_value_Item> {
    |el_: &X690Element| -> ASN1Result<ExtensionAttribute_value_Item> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_ExtensionAttribute_value_Item,
            _eal_components_for_ExtensionAttribute_value_Item,
            _rctl2_components_for_ExtensionAttribute_value_Item,
        )?;
        let mandatory: OPTIONAL<BOOLEAN> = match _components.get("mandatory") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let critical: OPTIONAL<BOOLEAN> = match _components.get("critical") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let ext =
            |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                _components.get("ext").unwrap(),
            )?;
        Ok(ExtensionAttribute_value_Item {
            mandatory,
            critical,
            ext,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_ExtensionAttribute_value_Item(
    value_: &ExtensionAttribute_value_Item,
) -> ASN1Result<X690Element> {
    |value_: &ExtensionAttribute_value_Item| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.mandatory {
            if *v_ != ExtensionAttribute_value_Item::_default_value_for_mandatory() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.critical {
            if *v_ != ExtensionAttribute_value_Item::_default_value_for_critical() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                2,
                Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
            ))
        }(&value_.ext)?);
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
/// extensionSyntax-Type ::= SEQUENCE { -- REMOVED_FROM_UNNESTING -- }
/// ```
///
///
#[derive(Debug, Clone)]
pub struct extensionSyntax_Type {
    pub mandatory: OPTIONAL<BOOLEAN>,
    pub critical: OPTIONAL<BOOLEAN>,
    pub ext: X690Element,
    pub _unrecognized: Vec<X690Element>,
}
impl extensionSyntax_Type {
    fn new(
        mandatory: OPTIONAL<BOOLEAN>,
        critical: OPTIONAL<BOOLEAN>,
        ext: X690Element,
        _unrecognized: Vec<X690Element>,
    ) -> Self {
        extensionSyntax_Type {
            mandatory,
            critical,
            ext,
            _unrecognized,
        }
    }
    pub fn _default_value_for_mandatory() -> BOOLEAN {
        false
    }
    pub fn _default_value_for_critical() -> BOOLEAN {
        false
    }
}
impl TryFrom<X690Element> for extensionSyntax_Type {
    type Error = ASN1Error;
    fn try_from(el: X690Element) -> Result<Self, Self::Error> {
        _decode_extensionSyntax_Type(&el)
    }
}
impl<'a> TryFrom<&'a X690Element> for extensionSyntax_Type {
    type Error = ASN1Error;
    fn try_from(el: &'a X690Element) -> Result<Self, Self::Error> {
        _decode_extensionSyntax_Type(el)
    }
}

pub const _rctl1_components_for_extensionSyntax_Type: &[ComponentSpec; 3] = &[
    ComponentSpec::new(
        "mandatory",
        true,
        TagSelector::tag((TagClass::CONTEXT, 0)),
        None,
        None,
    ),
    ComponentSpec::new(
        "critical",
        true,
        TagSelector::tag((TagClass::CONTEXT, 1)),
        None,
        None,
    ),
    ComponentSpec::new(
        "ext",
        false,
        TagSelector::tag((TagClass::CONTEXT, 2)),
        None,
        None,
    ),
];

pub const _rctl2_components_for_extensionSyntax_Type: &[ComponentSpec; 0] = &[];

pub const _eal_components_for_extensionSyntax_Type: &[ComponentSpec; 0] = &[];

pub fn _decode_extensionSyntax_Type(el: &X690Element) -> ASN1Result<extensionSyntax_Type> {
    |el_: &X690Element| -> ASN1Result<extensionSyntax_Type> {
        let elements = match el_.value.borrow() {
            X690Encoding::Constructed(children) => children,
            _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        };
        let el_refs_ = elements.iter().collect::<Vec<&X690Element>>();
        let (_components, _unrecognized) = _parse_sequence(
            el_refs_.as_slice(),
            _rctl1_components_for_extensionSyntax_Type,
            _eal_components_for_extensionSyntax_Type,
            _rctl2_components_for_extensionSyntax_Type,
        )?;
        let mandatory: OPTIONAL<BOOLEAN> = match _components.get("mandatory") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let critical: OPTIONAL<BOOLEAN> = match _components.get("critical") {
            Some(c_) => Some(|el: &X690Element| -> ASN1Result<BOOLEAN> {
                Ok(ber_decode_boolean(&el.inner()?)?)
            }(c_)?),
            _ => None,
        };
        let ext =
            |el: &X690Element| -> ASN1Result<X690Element> { Ok(x690_identity(&el.inner()?)?) }(
                _components.get("ext").unwrap(),
            )?;
        Ok(extensionSyntax_Type {
            mandatory,
            critical,
            ext,
            _unrecognized,
        })
    }(&el)
}

pub fn _encode_extensionSyntax_Type(value_: &extensionSyntax_Type) -> ASN1Result<X690Element> {
    |value_: &extensionSyntax_Type| -> ASN1Result<X690Element> {
        let mut components_: Vec<X690Element> = Vec::with_capacity(13);
        if let Some(v_) = &value_.mandatory {
            if *v_ != extensionSyntax_Type::_default_value_for_mandatory() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        0,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        if let Some(v_) = &value_.critical {
            if *v_ != extensionSyntax_Type::_default_value_for_critical() {
                components_.push(|v_1: &BOOLEAN| -> ASN1Result<X690Element> {
                    Ok(X690Element::new(
                        TagClass::CONTEXT,
                        1,
                        Arc::new(X690Encoding::EXPLICIT(Box::new(ber_encode_boolean(&v_1)?))),
                    ))
                }(&v_)?);
            }
        }
        components_.push(|v_1: &X690Element| -> ASN1Result<X690Element> {
            Ok(X690Element::new(
                TagClass::CONTEXT,
                2,
                Arc::new(X690Encoding::EXPLICIT(Box::new(x690_identity(&v_1)?))),
            ))
        }(&value_.ext)?);
        Ok(X690Element::new(
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_SET,
            Arc::new(X690Encoding::Constructed(
                [components_, value_._unrecognized.clone()].concat(),
            )),
        ))
    }(&value_)
}
