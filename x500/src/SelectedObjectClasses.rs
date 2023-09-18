#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # SelectedObjectClasses
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `SelectedObjectClasses`.
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
use crate::PasswordPolicy::*;
use crate::SelectedAttributeTypes::*;
use crate::UsefulDefinitions::*;
use asn1::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// TelecommunicationAttributeSet ATTRIBUTE ::= {facsimileTelephoneNumber |
///    internationalISDNNumber |
///    telephoneNumber |
///    telexNumber |
///    preferredDeliveryMethod |
///    destinationIndicator |
///    registeredAddress |
///    x121Address}
/// ```
///
///
pub fn TelecommunicationAttributeSet() -> Vec<ATTRIBUTE> {
    Vec::from([
        facsimileTelephoneNumber(),
        internationalISDNNumber(),
        telephoneNumber(),
        telexNumber(),
        preferredDeliveryMethod(),
        destinationIndicator(),
        registeredAddress(),
        x121Address(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// PostalAttributeSet ATTRIBUTE ::= {physicalDeliveryOfficeName |
///    postalAddress |
///    postalCode |
///    postOfficeBox |
///    streetAddress}
/// ```
///
///
pub fn PostalAttributeSet() -> Vec<ATTRIBUTE> {
    Vec::from([
        physicalDeliveryOfficeName(),
        postalAddress(),
        postalCode(),
        postOfficeBox(),
        streetAddress(),
    ])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// LocaleAttributeSet ATTRIBUTE ::= {localityName |
///    stateOrProvinceName |
///    streetAddress}
/// ```
///
///
pub fn LocaleAttributeSet() -> Vec<ATTRIBUTE> {
    Vec::from([localityName(), stateOrProvinceName(), streetAddress()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// OrganizationalAttributeSet ATTRIBUTE ::= {description |
///    LocaleAttributeSet |
///    PostalAttributeSet |
///    TelecommunicationAttributeSet |
///    businessCategory |
///    seeAlso |
///    searchGuide |
///    userPassword}
/// ```
///
///
pub fn OrganizationalAttributeSet() -> Vec<ATTRIBUTE> {
    [
        LocaleAttributeSet().as_slice(),
        PostalAttributeSet().as_slice(),
        TelecommunicationAttributeSet().as_slice(),
        Vec::from([
            description(),
            businessCategory(),
            seeAlso(),
            searchGuide(),
            userPassword(),
        ])
        .as_slice(),
    ]
    .concat()
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// country OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {countryName}
///   MAY CONTAIN   {description |
///                  searchGuide}
///   LDAP-NAME     {"country"}  -- RFC 4519
///   ID            id-oc-country }
/// ```
///
///
pub fn country() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([countryName()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([description(), searchGuide()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("country")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_country(),                                  /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod country {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// locality OBJECT-CLASS ::= {
///   SUBCLASS OF  {top}
///   MAY CONTAIN  {description |
///                 searchGuide |
///                 LocaleAttributeSet |
///                 seeAlso}
///   LDAP-NAME    {"locality"}  -- RFC 4519
///   ID           id-oc-locality }
/// ```
///
///
pub fn locality() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(
            [
                LocaleAttributeSet().as_slice(),
                Vec::from([description(), searchGuide(), seeAlso()]).as_slice(),
            ]
            .concat(),
        ), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("locality")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_locality(),                   /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapDesc: None,
    }
}

pub mod locality {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// organization OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {organizationName}
///   MAY CONTAIN   {OrganizationalAttributeSet}
///   LDAP-NAME     {"organization"}  -- RFC 4519
///   ID            id-oc-organization }
/// ```
///
///
pub fn organization() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([organizationName()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(
            [
                OrganizationalAttributeSet().as_slice(),
                Vec::from([]).as_slice(),
            ]
            .concat(),
        ), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("organization")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_organization(),               /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod organization {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// organizationalUnit OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {organizationalUnitName}
///   MAY CONTAIN   {OrganizationalAttributeSet}
///   LDAP-NAME     {"organizationalUnit"}  -- RFC 4519
///   ID            id-oc-organizationalUnit }
/// ```
///
///
pub fn organizationalUnit() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([organizationalUnitName()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(
            [
                OrganizationalAttributeSet().as_slice(),
                Vec::from([]).as_slice(),
            ]
            .concat(),
        ), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("organizationalUnit")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_organizationalUnit(),         /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod organizationalUnit {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// person OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {commonName |
///                  surname}
///   MAY CONTAIN   {description |
///                  telephoneNumber |
///                  userPassword |
///                  seeAlso}
///   LDAP-NAME     {"person"}  -- RFC 4519
///   ID            id-oc-person }
/// ```
///
///
pub fn person() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([commonName(), surname()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            description(),
            telephoneNumber(),
            userPassword(),
            seeAlso(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("person")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_person(),                     /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod person {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// organizationalPerson OBJECT-CLASS ::= {
///   SUBCLASS OF  {person}
///   MAY CONTAIN  {LocaleAttributeSet |
///                 PostalAttributeSet |
///                 TelecommunicationAttributeSet |
///                 organizationalUnitName |
///                 title}
///   LDAP-NAME    {"organizationalPerson"}  -- RFC 4519
///   ID           id-oc-organizationalPerson }
/// ```
///
///
pub fn organizationalPerson() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([person()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(
            [
                LocaleAttributeSet().as_slice(),
                PostalAttributeSet().as_slice(),
                TelecommunicationAttributeSet().as_slice(),
                Vec::from([organizationalUnitName(), title()]).as_slice(),
            ]
            .concat(),
        ), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("organizationalPerson")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_organizationalPerson(), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapDesc: None,
    }
}

pub mod organizationalPerson {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// organizationalRole OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {commonName}
///   MAY CONTAIN   {description |
///                  LocaleAttributeSet |
///                  organizationalUnitName |
///                  PostalAttributeSet |
///                  preferredDeliveryMethod |
///                  roleOccupant |
///                  seeAlso |
///                  TelecommunicationAttributeSet}
///   LDAP-NAME      {"organizationalRole"}  -- RFC 4519
///   ID            id-oc-organizationalRole }
/// ```
///
///
pub fn organizationalRole() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([commonName()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(
            [
                LocaleAttributeSet().as_slice(),
                PostalAttributeSet().as_slice(),
                TelecommunicationAttributeSet().as_slice(),
                Vec::from([
                    description(),
                    organizationalUnitName(),
                    preferredDeliveryMethod(),
                    roleOccupant(),
                    seeAlso(),
                ])
                .as_slice(),
            ]
            .concat(),
        ), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("organizationalRole")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_organizationalRole(),         /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod organizationalRole {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// groupOfNames OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {commonName | member}
///   MAY CONTAIN   {description |
///                  organizationName |
///                  organizationalUnitName |
///                  owner |
///                  seeAlso |
///                  businessCategory}
///   LDAP-NAME     {"groupOfNames"}  -- RFC 4519
///   ID            id-oc-groupOfNames }
/// ```
///
///
pub fn groupOfNames() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([commonName(), member()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            description(),
            organizationName(),
            organizationalUnitName(),
            owner(),
            seeAlso(),
            businessCategory(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("groupOfNames")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_groupOfNames(),               /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod groupOfNames {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// groupOfUniqueNames OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {commonName |
///                  uniqueMember}
///   MAY CONTAIN   {description |
///                  organizationName |
///                  organizationalUnitName |
///                  owner |
///                  seeAlso |
///                  businessCategory}
///   LDAP-NAME     {"groupOfUniqueNames"}  -- RFC 4519
///   ID            id-oc-groupOfUniqueNames }
/// ```
///
///
pub fn groupOfUniqueNames() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([commonName(), uniqueMember()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            description(),
            organizationName(),
            organizationalUnitName(),
            owner(),
            seeAlso(),
            businessCategory(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("groupOfUniqueNames")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_groupOfUniqueNames(),                                  /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod groupOfUniqueNames {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// residentialPerson OBJECT-CLASS ::= {
///   SUBCLASS OF   {person}
///   MUST CONTAIN  {localityName}
///   MAY CONTAIN   {LocaleAttributeSet |
///                  PostalAttributeSet |
///                  preferredDeliveryMethod |
///                  TelecommunicationAttributeSet |
///                  businessCategory}
///   LDAP-NAME     {"residentialPerson"}  -- RFC 4519
///   ID            id-oc-residentialPerson }
/// ```
///
///
pub fn residentialPerson() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([person()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([localityName()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(
            [
                LocaleAttributeSet().as_slice(),
                PostalAttributeSet().as_slice(),
                TelecommunicationAttributeSet().as_slice(),
                Vec::from([preferredDeliveryMethod(), businessCategory()]).as_slice(),
            ]
            .concat(),
        ), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("residentialPerson")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_residentialPerson(),             /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod residentialPerson {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// applicationProcess OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {commonName}
///   MAY CONTAIN   {description |
///                  localityName |
///                  organizationalUnitName |
///                  seeAlso}
///   LDAP-NAME     {"applicationProcess"}   -- RFC 4519
///   ID            id-oc-applicationProcess }
/// ```
///
///
pub fn applicationProcess() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([commonName()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            description(),
            localityName(),
            organizationalUnitName(),
            seeAlso(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("applicationProcess")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_applicationProcess(),         /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod applicationProcess {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// applicationEntity OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {commonName |
///                  presentationAddress}
///   MAY CONTAIN   {description |
///                  localityName |
///                  organizationName |
///                  organizationalUnitName |
///                  seeAlso |
///                  supportedApplicationContext}
///   LDAP-NAME     {"applicationEntity"} -- RFC 2256
///   ID            id-oc-applicationEntity }
/// ```
///
///
pub fn applicationEntity() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([commonName(), presentationAddress()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            description(),
            localityName(),
            organizationName(),
            organizationalUnitName(),
            seeAlso(),
            supportedApplicationContext(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("applicationEntity")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_applicationEntity(),                                  /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod applicationEntity {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dSA OBJECT-CLASS ::= {
///   SUBCLASS OF   {applicationEntity}
///   MAY CONTAIN   {knowledgeInformation}
///   LDAP-NAME     {"dSA"} -- RFC 2256
///   ID            id-oc-dSA }
/// ```
///
///
pub fn dSA() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([applicationEntity()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([knowledgeInformation()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("dSA")])),     /* OBJECT_FIELD_SETTING */
        id: id_oc_dSA(),                                      /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapDesc: None,
    }
}

pub mod dSA {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// device OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {commonName}
///   MAY CONTAIN   {description |
///                  localityName |
///                  organizationName |
///                  organizationalUnitName |
///                  owner |
///                  seeAlso |
///                  serialNumber}
///   LDAP-NAME      {"device"}  -- RFC 4519
///   ID            id-oc-device }
/// ```
///
///
pub fn device() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([commonName()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            description(),
            localityName(),
            organizationName(),
            organizationalUnitName(),
            owner(),
            seeAlso(),
            serialNumber(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("device")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_device(),                     /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod device {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// strongAuthenticationUser OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   KIND          auxiliary
///   MUST CONTAIN  {userCertificate}
///   LDAP-NAME     {"strongAuthenticationUser"} -- RFC 4523
///   LDAP-DESC     {"X.521 strong authentication user"}
///   ID            id-oc-strongAuthenticationUser }
/// ```
///
///
pub fn strongAuthenticationUser() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([userCertificate()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("strongAuthenticationUser")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.521 strong authentication user")), /* OBJECT_FIELD_SETTING */
        id: id_oc_strongAuthenticationUser(),                             /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
    }
}

pub mod strongAuthenticationUser {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// userSecurityInformation OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   KIND          auxiliary
///   MAY CONTAIN   {supportedAlgorithms}
///   LDAP-NAME     {"userSecurityInformation"} -- RFC 4523
///   LDAP-DESC     {"X.521 user security information"}
///   ID            id-oc-userSecurityInformation }
/// ```
///
///
pub fn userSecurityInformation() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([supportedAlgorithms()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("userSecurityInformation")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.521 user security information")), /* OBJECT_FIELD_SETTING */
        id: id_oc_userSecurityInformation(),                             /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
}

pub mod userSecurityInformation {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// userPwdClass  OBJECT-CLASS ::= {
///   KIND          auxiliary
///   MAY CONTAIN   { userPwd }
///   ID            id-oc-userPwdClass }
/// ```
///
///
pub fn userPwdClass() -> OBJECT_CLASS {
    OBJECT_CLASS {
        kind: Some(ObjectClassKind_auxiliary), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([userPwd()])), /* OBJECT_FIELD_SETTING */
        id: id_oc_userPwdClass(),              /* OBJECT_FIELD_SETTING */
        Superclasses: None,
        MandatoryAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod userPwdClass {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificationAuthority OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   KIND          auxiliary
///   MUST CONTAIN  {cACertificate |
///                  certificateRevocationList |
///                  authorityRevocationList}
///   MAY CONTAIN   {crossCertificatePair}
///   LDAP-NAME     {"certificationAuthority"} -- RFC 4523
///   LDAP-DESC     {"X.509 certificate authority"}
///   ID            id-oc-certificationAuthority }
/// ```
///
///
pub fn certificationAuthority() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([
            cACertificate(),
            certificateRevocationList(),
            authorityRevocationList(),
        ])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([crossCertificatePair()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("certificationAuthority")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 certificate authority")), /* OBJECT_FIELD_SETTING */
        id: id_oc_certificationAuthority(),                          /* OBJECT_FIELD_SETTING */
    }
}

pub mod certificationAuthority {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificationAuthority-V2 OBJECT-CLASS ::= {
///   SUBCLASS OF   {certificationAuthority}
///   KIND          auxiliary
///   MAY CONTAIN   {deltaRevocationList}
///   LDAP-NAME     {"certificationAuthority-V2"}
///   LDAP-DESC     {"X.509 certificate authority, version 2"} -- RFC 4523
///   ID            id-oc-certificationAuthority-V2 }
/// ```
///
///
pub fn certificationAuthority_V2() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([certificationAuthority()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),                     /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([deltaRevocationList()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("certificationAuthority-V2")])), /* OBJECT_FIELD_SETTING */
        ldapDesc: Some(String::from("X.509 certificate authority, version 2")), /* OBJECT_FIELD_SETTING */
        id: id_oc_certificationAuthority_V2(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
    }
}

pub mod certificationAuthority_V2 {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dMD OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {dmdName}
///   MAY CONTAIN   {OrganizationalAttributeSet}
///   LDAP-NAME     {"dmd"} -- RFC 2256
///   ID            id-oc-dmd }
/// ```
///
///
pub fn dMD() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([dmdName()])), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(
            [
                OrganizationalAttributeSet().as_slice(),
                Vec::from([]).as_slice(),
            ]
            .concat(),
        ), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("dmd")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_dmd(),                        /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        ldapDesc: None,
    }
}

pub mod dMD {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidC1obj OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {oidC}
///   LDAP-NAME     {"oidC1obj"}
///   ID            id-oc-oidC1obj }
/// ```
///
///
pub fn oidC1obj() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([oidC()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("oidC1obj")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_oidC1obj(),                   /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapDesc: None,
    }
}

pub mod oidC1obj {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidC2obj OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {oidC}
///   LDAP-NAME     {"oidC2obj"}
///   ID            id-oc-oidC2obj }
/// ```
///
///
pub fn oidC2obj() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([oidC()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("oidC2obj")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_oidC2obj(),                   /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapDesc: None,
    }
}

pub mod oidC2obj {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidCobj OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  {oidC}
///   LDAP-NAME     {"oidCobj"}
///   ID            id-oc-oidCobj }
/// ```
///
///
pub fn oidCobj() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([oidC()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("oidCobj")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_oidCobj(),                    /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapDesc: None,
    }
}

pub mod oidCobj {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidRoot OBJECT-CLASS ::= {
///   SUBCLASS OF   {alias}
///   MUST CONTAIN  { oidC1 | oidC2 | oidC}
///   LDAP-NAME     {"oidRoot"}
///   ID            id-oidRoot }
/// ```
///
///
pub fn oidRoot() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([alias()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([oidC1(), oidC2(), oidC()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("oidRoot")])), /* OBJECT_FIELD_SETTING */
        id: id_oidRoot(),                         /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapDesc: None,
    }
}

pub mod oidRoot {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidArc OBJECT-CLASS ::= {
///   SUBCLASS OF   {alias}
///   MUST CONTAIN  {oidC}
///   LDAP-NAME     {"oidArc"}
///   ID            id-oidArc }
/// ```
///
///
pub fn oidArc() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([alias()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([oidC()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("oidArc")])), /* OBJECT_FIELD_SETTING */
        id: id_oidArc(),                          /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapDesc: None,
    }
}

pub mod oidArc {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// urnCobj OBJECT-CLASS ::= {
///   SUBCLASS OF   {top}
///   MUST CONTAIN  { urnC }
///   LDAP-NAME     {"urnCobj"}
///   ID            id-oc-urnCobj }
/// ```
///
///
pub fn urnCobj() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Some(Vec::from([urnC()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("urnCobj")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_urnCobj(),                    /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_structural), /* OBJECT_FIELD_SETTING DEFAULT_OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapDesc: None,
    }
}

pub mod urnCobj {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// isoTagInfo OBJECT-CLASS ::= {
///   SUBCLASS OF  { top }
///   KIND         auxiliary
///   MAY CONTAIN  { tagOid |
///                  tagAfi |
///                  uii |
///                  uiiInUrn |
///                  contentUrl |
///                  tagLocation }
///   LDAP-NAME    {"isoTagInfo"}
///   ID           id-oc-isoTagInfo }
/// ```
///
///
pub fn isoTagInfo() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([
            tagOid(),
            tagAfi(),
            uii(),
            uiiInUrn(),
            contentUrl(),
            tagLocation(),
        ])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("isoTagInfo")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_isoTagInfo(),                 /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapDesc: None,
    }
}

pub mod isoTagInfo {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// isoTagType  OBJECT-CLASS ::= {
///   SUBCLASS OF  { top }
///   KIND         auxiliary
///   MAY CONTAIN  { tagOid |
///                  tagAfi |
///                  uiiFormat }
///   LDAP-NAME    {"isoTagType"}
///   ID           id-oc-isoTagType }
/// ```
///
///
pub fn isoTagType() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([tagOid(), tagAfi(), uiiFormat()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("isoTagType")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_isoTagType(),                                  /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapDesc: None,
    }
}

pub mod isoTagType {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// epcTagInfoObj OBJECT-CLASS ::= {
///   SUBCLASS OF  { top }
///   KIND         auxiliary
///   MAY CONTAIN  { epc |
///                  epcInUrn |
///                  contentUrl |
///                  tagLocation }
///   LDAP-NAME    {"epcTagInfoObj"}
///   ID           id-oc-epcTagInfoObj }
/// ```
///
///
pub fn epcTagInfoObj() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([epc(), epcInUrn(), contentUrl(), tagLocation()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("epcTagInfoObj")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_epcTagInfoObj(),                                  /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapDesc: None,
    }
}

pub mod epcTagInfoObj {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// epcTagTypeObj OBJECT-CLASS ::= {
///   SUBCLASS OF  { top }
///   KIND         auxiliary
///   MAY CONTAIN  { uiiFormat }
///   LDAP-NAME    {"epcTagTypeObj"}
///   ID           id-oc-epcTagTypeObj }
/// ```
///
///
pub fn epcTagTypeObj() -> OBJECT_CLASS {
    OBJECT_CLASS {
        Superclasses: Some(Vec::from([top()])), /* OBJECT_FIELD_SETTING */
        kind: Some(ObjectClassKind_auxiliary),  /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([uiiFormat()])), /* OBJECT_FIELD_SETTING */
        ldapName: Some(Vec::from([String::from("epcTagTypeObj")])), /* OBJECT_FIELD_SETTING */
        id: id_oc_epcTagTypeObj(),              /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: None,
        ldapDesc: None,
    }
}

pub mod epcTagTypeObj {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// countryNameForm NAME-FORM ::= {
///   NAMES            country
///   WITH ATTRIBUTES  {countryName}
///   ID               id-nf-countryNameForm }
/// ```
///
///
pub fn countryNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: country(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([countryName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_countryNameForm(), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod countryNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// locNameForm NAME-FORM ::= {
///   NAMES            locality
///   WITH ATTRIBUTES  {localityName}
///   ID               id-nf-locNameForm }
/// ```
///
///
pub fn locNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: locality(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([localityName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_locNameForm(),      /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod locNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// sOPNameForm NAME-FORM ::= {
///   NAMES            locality
///   WITH ATTRIBUTES  {stateOrProvinceName}
///   ID               id-nf-sOPNameForm }
/// ```
///
///
pub fn sOPNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: locality(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([stateOrProvinceName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_sOPNameForm(),      /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod sOPNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// orgNameForm NAME-FORM ::= {
///   NAMES            organization
///   WITH ATTRIBUTES  {organizationName}
///   ID               id-nf-orgNameForm }
/// ```
///
///
pub fn orgNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: organization(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([organizationName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_orgNameForm(),          /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod orgNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// orgUnitNameForm NAME-FORM ::= {
///   NAMES            organizationalUnit
///   WITH ATTRIBUTES  {organizationalUnitName}
///   ID               id-nf-orgUnitNameForm }
/// ```
///
///
pub fn orgUnitNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: organizationalUnit(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([organizationalUnitName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_orgUnitNameForm(),            /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod orgUnitNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// personNameForm NAME-FORM ::= {
///   NAMES            person
///   WITH ATTRIBUTES  {commonName}
///   ID               id-nf-personNameForm }
/// ```
///
///
pub fn personNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: person(),                     /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([commonName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_personNameForm(),                     /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod personNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// orgPersonNameForm NAME-FORM ::= {
///   NAMES            organizationalPerson
///   WITH ATTRIBUTES  {commonName}
///   AND OPTIONALLY   {organizationalUnitName}
///   ID               id-nf-orgPersonNameForm }
/// ```
///
///
pub fn orgPersonNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: organizationalPerson(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([commonName()]), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([organizationalUnitName()])), /* OBJECT_FIELD_SETTING */
        id: id_nf_orgPersonNameForm(),            /* OBJECT_FIELD_SETTING */
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod orgPersonNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// orgRoleNameForm NAME-FORM ::= {
///   NAMES            organizationalRole
///   WITH ATTRIBUTES  {commonName}
///   ID               id-nf-orgRoleNameForm }
/// ```
///
///
pub fn orgRoleNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: organizationalRole(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([commonName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_orgRoleNameForm(),            /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod orgRoleNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// gONNameForm NAME-FORM ::= {
///   NAMES            groupOfNames
///   WITH ATTRIBUTES  {commonName}
///   ID               id-nf-gONNameForm }
/// ```
///
///
pub fn gONNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: groupOfNames(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([commonName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_gONNameForm(),          /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod gONNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// resPersonNameForm NAME-FORM ::= {
///   NAMES            residentialPerson
///   WITH ATTRIBUTES  {commonName}
///   AND OPTIONALLY   {streetAddress}
///   ID               id-nf-resPersonNameForm }
/// ```
///
///
pub fn resPersonNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: residentialPerson(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([commonName()]), /* OBJECT_FIELD_SETTING */
        OptionalAttributes: Some(Vec::from([streetAddress()])), /* OBJECT_FIELD_SETTING */
        id: id_nf_resPersonNameForm(),         /* OBJECT_FIELD_SETTING */
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod resPersonNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// applProcessNameForm NAME-FORM ::= {
///   NAMES            applicationProcess
///   WITH ATTRIBUTES  {commonName}
///   ID               id-nf-applProcessNameForm }
/// ```
///
///
pub fn applProcessNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: applicationProcess(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([commonName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_applProcessNameForm(),        /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod applProcessNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// applEntityNameForm NAME-FORM ::= {
///   NAMES            applicationEntity
///   WITH ATTRIBUTES  {commonName}
///   ID               id-nf-applEntityNameForm }
/// ```
///
///
pub fn applEntityNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: applicationEntity(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([commonName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_applEntityNameForm(),        /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod applEntityNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dSANameForm NAME-FORM ::= {
///   NAMES            dSA
///   WITH ATTRIBUTES  {commonName}
///   ID               id-nf-dSANameForm }
/// ```
///
///
pub fn dSANameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: dSA(),                        /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([commonName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_dSANameForm(),                        /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod dSANameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// deviceNameForm NAME-FORM ::= {
///   NAMES            device
///   WITH ATTRIBUTES  {commonName}
///   ID               id-nf-deviceNameForm }
/// ```
///
///
pub fn deviceNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: device(),                     /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([commonName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_deviceNameForm(),                     /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod deviceNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dMDNameForm NAME-FORM ::= {
///   NAMES            dMD
///   WITH ATTRIBUTES  {dmdName}
///   ID               id-nf-dMDNameForm }
/// ```
///
///
pub fn dMDNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: dMD(),                     /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([dmdName()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_dMDNameForm(),                     /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod dMDNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidC1NameForm NAME-FORM ::= {
///   NAMES            oidCobj
///   WITH ATTRIBUTES  {oidC}
///   ID               id-nf-oidC1NameForm }
/// ```
///
///
pub fn oidC1NameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: oidCobj(),              /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([oidC()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_oidC1NameForm(),                /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod oidC1NameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidC2NameForm NAME-FORM ::= {
///   NAMES            oidCobj
///   WITH ATTRIBUTES  {oidC}
///   ID               id-nf-oidC2NameForm }
/// ```
///
///
pub fn oidC2NameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: oidCobj(),              /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([oidC()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_oidC2NameForm(),                /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod oidC2NameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidCNameForm NAME-FORM ::= {
///   NAMES            oidCobj
///   WITH ATTRIBUTES  {oidC}
///   ID               id-nf-oidCNameForm }
/// ```
///
///
pub fn oidCNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: oidCobj(),              /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([oidC()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_oidCNameForm(),                 /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod oidCNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// urnCNameForm NAME-FORM ::= {
///   NAMES            urnCobj
///   WITH ATTRIBUTES  {urnC}
///   ID               id-nf-urnCNameForm }
/// ```
///
///
pub fn urnCNameForm() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: urnCobj(),              /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([urnC()]), /* OBJECT_FIELD_SETTING */
        id: id_nf_urnCNameForm(),                 /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod urnCNameForm {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidRootNf NAME-FORM ::= {
///   NAMES            oidRoot
///   WITH ATTRIBUTES  {oidC1 | oidC2 | oidC}
///   ID               id-oidRootNf }
/// ```
///
///
pub fn oidRootNf() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: oidRoot(), /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([oidC1(), oidC2(), oidC()]), /* OBJECT_FIELD_SETTING */
        id: id_oidRootNf(),          /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod oidRootNf {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oidArcNf NAME-FORM ::= {
///   NAMES            oidArc
///   WITH ATTRIBUTES  {oidC}
///   ID               id-oidArcNf }
/// ```
///
///
pub fn oidArcNf() -> NAME_FORM {
    NAME_FORM {
        namedObjectClass: oidArc(),               /* OBJECT_FIELD_SETTING */
        MandatoryAttributes: Vec::from([oidC()]), /* OBJECT_FIELD_SETTING */
        id: id_oidArcNf(),                        /* OBJECT_FIELD_SETTING */
        OptionalAttributes: None,
        ldapName: None,
        ldapDesc: None,
    }
}

pub mod oidArcNf {
    /* OBJECT_TYPES */
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-country                  OBJECT IDENTIFIER ::= {id-oc 2}
/// ```
///
///
pub fn id_oc_country() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-locality                 OBJECT IDENTIFIER ::= {id-oc 3}
/// ```
///
///
pub fn id_oc_locality() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-organization             OBJECT IDENTIFIER ::= {id-oc 4}
/// ```
///
///
pub fn id_oc_organization() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-organizationalUnit       OBJECT IDENTIFIER ::= {id-oc 5}
/// ```
///
///
pub fn id_oc_organizationalUnit() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-person                   OBJECT IDENTIFIER ::= {id-oc 6}
/// ```
///
///
pub fn id_oc_person() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-organizationalPerson     OBJECT IDENTIFIER ::= {id-oc 7}
/// ```
///
///
pub fn id_oc_organizationalPerson() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-organizationalRole       OBJECT IDENTIFIER ::= {id-oc 8}
/// ```
///
///
pub fn id_oc_organizationalRole() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-groupOfNames             OBJECT IDENTIFIER ::= {id-oc 9}
/// ```
///
///
pub fn id_oc_groupOfNames() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-residentialPerson        OBJECT IDENTIFIER ::= {id-oc 10}
/// ```
///
///
pub fn id_oc_residentialPerson() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-applicationProcess       OBJECT IDENTIFIER ::= {id-oc 11}
/// ```
///
///
pub fn id_oc_applicationProcess() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-applicationEntity        OBJECT IDENTIFIER ::= {id-oc 12}
/// ```
///
///
pub fn id_oc_applicationEntity() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-dSA                      OBJECT IDENTIFIER ::= {id-oc 13}
/// ```
///
///
pub fn id_oc_dSA() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-device                   OBJECT IDENTIFIER ::= {id-oc 14}
/// ```
///
///
pub fn id_oc_device() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([14])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-strongAuthenticationUser OBJECT IDENTIFIER ::= {id-oc 15}
/// ```
///
///
pub fn id_oc_strongAuthenticationUser() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([15])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-certificationAuthority   OBJECT IDENTIFIER ::= {id-oc 16}
/// ```
///
///
pub fn id_oc_certificationAuthority() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([16])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-certificationAuthority-V2
///                                OBJECT IDENTIFIER ::= {id-oc 16 2}
/// ```
///
///
pub fn id_oc_certificationAuthority_V2() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([16, 2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-groupOfUniqueNames       OBJECT IDENTIFIER ::= {id-oc 17}
/// ```
///
///
pub fn id_oc_groupOfUniqueNames() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([17])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-userSecurityInformation  OBJECT IDENTIFIER ::= {id-oc 18}
/// ```
///
///
pub fn id_oc_userSecurityInformation() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([18])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-dmd                      OBJECT IDENTIFIER ::= {id-oc 20}
/// ```
///
///
pub fn id_oc_dmd() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([20])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-oidC1obj                 OBJECT IDENTIFIER ::= {id-oc 35}
/// ```
///
///
pub fn id_oc_oidC1obj() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([35])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-oidC2obj                 OBJECT IDENTIFIER ::= {id-oc 36}
/// ```
///
///
pub fn id_oc_oidC2obj() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([36])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-oidCobj                  OBJECT IDENTIFIER ::= {id-oc 37}
/// ```
///
///
pub fn id_oc_oidCobj() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([37])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-isoTagInfo               OBJECT IDENTIFIER ::= {id-oc 38}
/// ```
///
///
pub fn id_oc_isoTagInfo() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([38])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-isoTagType               OBJECT IDENTIFIER ::= {id-oc 39}
/// ```
///
///
pub fn id_oc_isoTagType() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([39])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-userPwdClass             OBJECT IDENTIFIER ::= {id-oc 41}
/// ```
///
///
pub fn id_oc_userPwdClass() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([41])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-urnCobj                  OBJECT IDENTIFIER ::= {id-oc 42}
/// ```
///
///
pub fn id_oc_urnCobj() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([42])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-epcTagInfoObj            OBJECT IDENTIFIER ::= {id-oc 43}
/// ```
///
///
pub fn id_oc_epcTagInfoObj() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([43])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc-epcTagTypeObj            OBJECT IDENTIFIER ::= {id-oc 44}
/// ```
///
///
pub fn id_oc_epcTagTypeObj() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_oc().0, Vec::<u32>::from([44])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oidRoot                     OBJECT IDENTIFIER ::= {id 3}
/// ```
///
///
pub fn id_oidRoot() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oidArc                      OBJECT IDENTIFIER ::= {id 5}
/// ```
///
///
pub fn id_oidArc() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-countryNameForm          OBJECT IDENTIFIER ::= {id-nf 0}
/// ```
///
///
pub fn id_nf_countryNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([0])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-locNameForm              OBJECT IDENTIFIER ::= {id-nf 1}
/// ```
///
///
pub fn id_nf_locNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-sOPNameForm              OBJECT IDENTIFIER ::= {id-nf 2}
/// ```
///
///
pub fn id_nf_sOPNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-orgNameForm              OBJECT IDENTIFIER ::= {id-nf 3}
/// ```
///
///
pub fn id_nf_orgNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-orgUnitNameForm          OBJECT IDENTIFIER ::= {id-nf 4}
/// ```
///
///
pub fn id_nf_orgUnitNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-personNameForm           OBJECT IDENTIFIER ::= {id-nf 5}
/// ```
///
///
pub fn id_nf_personNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-orgPersonNameForm        OBJECT IDENTIFIER ::= {id-nf 6}
/// ```
///
///
pub fn id_nf_orgPersonNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-orgRoleNameForm          OBJECT IDENTIFIER ::= {id-nf 7}
/// ```
///
///
pub fn id_nf_orgRoleNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-gONNameForm              OBJECT IDENTIFIER ::= {id-nf 8}
/// ```
///
///
pub fn id_nf_gONNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-resPersonNameForm        OBJECT IDENTIFIER ::= {id-nf 9}
/// ```
///
///
pub fn id_nf_resPersonNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-applProcessNameForm      OBJECT IDENTIFIER ::= {id-nf 10}
/// ```
///
///
pub fn id_nf_applProcessNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-applEntityNameForm       OBJECT IDENTIFIER ::= {id-nf 11}
/// ```
///
///
pub fn id_nf_applEntityNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-dSANameForm              OBJECT IDENTIFIER ::= {id-nf 12}
/// ```
///
///
pub fn id_nf_dSANameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-deviceNameForm           OBJECT IDENTIFIER ::= {id-nf 13}
/// ```
///
///
pub fn id_nf_deviceNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-dMDNameForm              OBJECT IDENTIFIER ::= {id-nf 15}
/// ```
///
///
pub fn id_nf_dMDNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([15])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-oidC1NameForm            OBJECT IDENTIFIER ::= {id-nf 17}
/// ```
///
///
pub fn id_nf_oidC1NameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([17])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-oidC2NameForm            OBJECT IDENTIFIER ::= {id-nf 18}
/// ```
///
///
pub fn id_nf_oidC2NameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([18])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-oidCNameForm             OBJECT IDENTIFIER ::= {id-nf 19}
/// ```
///
///
pub fn id_nf_oidCNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([19])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf-urnCNameForm             OBJECT IDENTIFIER ::= {id-nf 20}
/// ```
///
///
pub fn id_nf_urnCNameForm() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id_nf().0, Vec::<u32>::from([20])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oidRootNf                   OBJECT IDENTIFIER ::= {id 4}
/// ```
///
///
pub fn id_oidRootNf() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oidArcNf                    OBJECT IDENTIFIER ::= {id 6}
/// ```
///
///
pub fn id_oidArcNf() -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER([id().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}
