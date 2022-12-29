#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # UsefulDefinitions
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `UsefulDefinitions`.
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
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ID     ::=  OBJECT IDENTIFIER
/// ```
pub type ID = OBJECT_IDENTIFIER; // ObjectIdentifierType

pub fn _decode_ID(el: &X690Element) -> ASN1Result<ID> {
    ber_decode_object_identifier(&el)
}

pub fn _encode_ID(value_: &ID) -> ASN1Result<X690Element> {
    ber_encode_object_identifier(&value_)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ds ID ::= {joint-iso-itu-t ds(5)}
/// ```
///
///
pub fn ds() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([joint_iso_itu_t, /* ds */ 5])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id ID ::= {joint-iso-itu-t registration-procedures(17) module(1) directory-defs(2)}
/// ```
///
///
pub fn id() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        joint_iso_itu_t,
        /* registration-procedures */ 17,
        /* module */ 1,
        /* directory-defs */ 2,
    ]))
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// internet            ID ::= {iso(1) identified-organization(3) dod(6) internet(1)}
/// ```
///
///
pub fn internet() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* iso */ 1, /* identified-organization */ 3, /* dod */ 6,
        /* internet */ 1,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-dir            ID ::= {internet directory(1)}
/// ```
///
///
pub fn ldap_dir() -> ID {
    OBJECT_IDENTIFIER([internet().0, Vec::<u32>::from([/* directory */ 1])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// intSecurity         ID ::= {internet security(5)}
/// ```
///
///
pub fn intSecurity() -> ID {
    OBJECT_IDENTIFIER([internet().0, Vec::<u32>::from([/* security */ 5])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-enterprise     ID ::= {internet private(4) enterprise(1)}
/// ```
///
///
pub fn ldap_enterprise() -> ID {
    OBJECT_IDENTIFIER(
        [
            internet().0,
            Vec::<u32>::from([/* private */ 4, /* enterprise */ 1]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-x509           ID ::= {ldap-dir x509(15)}
/// ```
///
///
pub fn ldap_x509() -> ID {
    OBJECT_IDENTIFIER([ldap_dir().0, Vec::<u32>::from([/* x509 */ 15])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-openLDAP       ID ::= {ldap-enterprise openLDAP(4203) ldap(1)}
/// ```
///
///
pub fn ldap_openLDAP() -> ID {
    OBJECT_IDENTIFIER(
        [
            ldap_enterprise().0,
            Vec::<u32>::from([/* openLDAP */ 4203, /* ldap */ 1]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// openLDAP-attributes ID ::= {ldap-openLDAP attributeType(3)}
/// ```
///
///
pub fn openLDAP_attributes() -> ID {
    OBJECT_IDENTIFIER([ldap_openLDAP().0, Vec::<u32>::from([/* attributeType */ 3])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// openLDAP-controls   ID ::= {ldap-openLDAP controls(10)}
/// ```
///
///
pub fn openLDAP_controls() -> ID {
    OBJECT_IDENTIFIER([ldap_openLDAP().0, Vec::<u32>::from([/* controls */ 10])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-wall           ID ::= {ldap-enterprise wahl(1466)}
/// ```
///
///
pub fn ldap_wall() -> ID {
    OBJECT_IDENTIFIER([ldap_enterprise().0, Vec::<u32>::from([/* wahl */ 1466])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-dynExt         ID ::= {ldap-wall 101 119}
/// ```
///
///
pub fn ldap_dynExt() -> ID {
    OBJECT_IDENTIFIER([ldap_wall().0, Vec::<u32>::from([101, 119])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-attr           ID ::= {ldap-wall 101 120}
/// ```
///
///
pub fn ldap_attr() -> ID {
    OBJECT_IDENTIFIER([ldap_wall().0, Vec::<u32>::from([101, 120])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-match          ID ::= {ldap-wall 109 114}
/// ```
///
///
pub fn ldap_match() -> ID {
    OBJECT_IDENTIFIER([ldap_wall().0, Vec::<u32>::from([109, 114])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-syntax         ID ::= {ldap-wall 115 121 1}
/// ```
///
///
pub fn ldap_syntax() -> ID {
    OBJECT_IDENTIFIER([ldap_wall().0, Vec::<u32>::from([115, 121, 1])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cosine              ID ::= {itu-t(0) data(9) pss(2342) ucl(19200300) pilot(100)}
/// ```
///
///
pub fn cosine() -> ID {
    OBJECT_IDENTIFIER(Vec::<u32>::from([
        /* itu-t */ 0, /* data */ 9, /* pss */ 2342, /* ucl */ 19200300,
        /* pilot */ 100,
    ])) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cosineAttr          ID ::= {cosine pilotAttributeType(1)}
/// ```
///
///
pub fn cosineAttr() -> ID {
    OBJECT_IDENTIFIER([cosine().0, Vec::<u32>::from([/* pilotAttributeType */ 1])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// module                                   ID ::= {ds 1}
/// ```
///
///
pub fn module() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([1])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// serviceElement                           ID ::= {ds 2}
/// ```
///
///
pub fn serviceElement() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([2])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// applicationContext                       ID ::= {ds 3}
/// ```
///
///
pub fn applicationContext() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([3])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeType                            ID ::= {ds 4}
/// ```
///
///
pub fn attributeType() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([4])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeSyntaxVendor                    ID ::= {ds 5}
/// ```
///
///
pub fn attributeSyntaxVendor() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([5])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// objectClass                              ID ::= {ds 6}
/// ```
///
///
pub fn objectClass() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([6])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeSet                             ID ::= {ds 7}
/// ```
///
///
pub fn attributeSet() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([7])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// algorithm                                ID ::= {ds 8}
/// ```
///
///
pub fn algorithm() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([8])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// abstractSyntax                           ID ::= {ds 9}
/// ```
///
///
pub fn abstractSyntax() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([9])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// object                                   ID ::= {ds 10}
/// ```
///
///
pub fn object() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([10])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// port                                     ID ::= {ds 11}
/// ```
///
///
pub fn port() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([11])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsaOperationalAttribute                  ID ::= {ds 12}
/// ```
///
///
pub fn dsaOperationalAttribute() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([12])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// matchingRule                             ID ::= {ds 13}
/// ```
///
///
pub fn matchingRule() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// knowledgeMatchingRule                    ID ::= {ds 14}
/// ```
///
///
pub fn knowledgeMatchingRule() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([14])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nameForm                                 ID ::= {ds 15}
/// ```
///
///
pub fn nameForm() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([15])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// group                                    ID ::= {ds 16}
/// ```
///
///
pub fn group() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([16])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subentry                                 ID ::= {ds 17}
/// ```
///
///
pub fn subentry() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([17])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// operationalAttributeType                 ID ::= {ds 18}
/// ```
///
///
pub fn operationalAttributeType() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([18])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// operationalBinding                       ID ::= {ds 19}
/// ```
///
///
pub fn operationalBinding() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([19])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// schemaObjectClass                        ID ::= {ds 20}
/// ```
///
///
pub fn schemaObjectClass() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([20])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// schemaOperationalAttribute               ID ::= {ds 21}
/// ```
///
///
pub fn schemaOperationalAttribute() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([21])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// administrativeRoles                      ID ::= {ds 23}
/// ```
///
///
pub fn administrativeRoles() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([23])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// accessControlAttribute                   ID ::= {ds 24}
/// ```
///
///
pub fn accessControlAttribute() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([24])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rosObject                                ID ::= {ds 25}
/// ```
///
///
pub fn rosObject() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([25])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// contract                                 ID ::= {ds 26}
/// ```
///
///
pub fn contract() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([26])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// package                                  ID ::= {ds 27}
/// ```
///
///
pub fn package() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([27])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// accessControlSchemes                     ID ::= {ds 28}
/// ```
///
///
pub fn accessControlSchemes() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([28])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificateExtension                     ID ::= {ds 29}
/// ```
///
///
pub fn certificateExtension() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([29])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// managementObject                         ID ::= {ds 30}
/// ```
///
///
pub fn managementObject() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([30])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeValueContext                    ID ::= {ds 31}
/// ```
///
///
pub fn attributeValueContext() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([31])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// securityExchange                         ID ::= {ds 32}
/// ```
///
///
pub fn securityExchange() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([32])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// idmProtocol                              ID ::= {ds 33}
/// ```
///
///
pub fn idmProtocol() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([33])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// problem                                  ID ::= {ds 34}
/// ```
///
///
pub fn problem() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([34])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// notification                             ID ::= {ds 35}
/// ```
///
///
pub fn notification() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([35])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// matchingRestriction                      ID ::= {ds 36}
/// ```
///
///
pub fn matchingRestriction() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([36])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// controlAttributeType                     ID ::= {ds 37}
/// ```
///
///
pub fn controlAttributeType() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([37])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// keyPurposes                              ID ::= {ds 38}
/// ```
///
///
pub fn keyPurposes() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([38])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// passwordQuality                          ID ::= {ds 39}
/// ```
///
///
pub fn passwordQuality() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([39])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeSyntax                          ID ::= {ds 40}
/// ```
///
///
pub fn attributeSyntax() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([40])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// avRestriction                            ID ::= {ds 41}
/// ```
///
///
pub fn avRestriction() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([41])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cmsContentType                           ID ::= {ds 42}
/// ```
///
///
pub fn cmsContentType() -> ID {
    OBJECT_IDENTIFIER([ds().0, Vec::<u32>::from([42])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// usefulDefinitions                        ID ::= {module usefulDefinitions(0) 9}
/// ```
///
///
pub fn usefulDefinitions() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* usefulDefinitions */ 0, 9])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// informationFramework                     ID ::= {module informationFramework(1) 9}
/// ```
///
///
pub fn informationFramework() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* informationFramework */ 1, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryAbstractService                 ID ::= {module directoryAbstractService(2) 9}
/// ```
///
///
pub fn directoryAbstractService() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* directoryAbstractService */ 2, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// distributedOperations                    ID ::= {module distributedOperations(3) 9}
/// ```
///
///
pub fn distributedOperations() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* distributedOperations */ 3, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// protocolObjectIdentifiers                ID ::= {module protocolObjectIdentifiers(4) 9}
/// ```
///
///
pub fn protocolObjectIdentifiers() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* protocolObjectIdentifiers */ 4, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// selectedAttributeTypes                   ID ::= {module selectedAttributeTypes(5) 9}
/// ```
///
///
pub fn selectedAttributeTypes() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* selectedAttributeTypes */ 5, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// selectedObjectClasses                    ID ::= {module selectedObjectClasses(6) 9}
/// ```
///
///
pub fn selectedObjectClasses() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* selectedObjectClasses */ 6, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// authenticationFramework                  ID ::= {module authenticationFramework(7) 9}
/// ```
///
///
pub fn authenticationFramework() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* authenticationFramework */ 7, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// algorithmObjectIdentifiers               ID ::= {module algorithmObjectIdentifiers(8) 9}
/// ```
///
///
pub fn algorithmObjectIdentifiers() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* algorithmObjectIdentifiers */ 8, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryObjectIdentifiers               ID ::= {module directoryObjectIdentifiers(9) 9}
/// ```
///
///
pub fn directoryObjectIdentifiers() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* directoryObjectIdentifiers */ 9, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// upperBounds                              ID ::= {module upperBounds(10) 9}
/// ```
///
///
pub fn upperBounds() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* upperBounds */ 10, 9])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dap                                      ID ::= {module dap(11) 9}
/// ```
///
///
pub fn dap() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* dap */ 11, 9])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsp                                      ID ::= {module dsp(12) 9}
/// ```
///
///
pub fn dsp() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* dsp */ 12, 9])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// distributedDirectoryOIDs                 ID ::= {module distributedDirectoryOIDs(13) 9}
/// ```
///
///
pub fn distributedDirectoryOIDs() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* distributedDirectoryOIDs */ 13, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryShadowOIDs                      ID ::= {module directoryShadowOIDs(14) 9}
/// ```
///
///
pub fn directoryShadowOIDs() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* directoryShadowOIDs */ 14, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryShadowAbstractService           ID ::= {module directoryShadowAbstractService(15) 9}
/// ```
///
///
pub fn directoryShadowAbstractService() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* directoryShadowAbstractService */ 15, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// disp                                     ID ::= {module disp(16) 7}
/// ```
///
///
pub fn disp() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* disp */ 16, 7])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dop                                      ID ::= {module dop(17) 7}
/// ```
///
///
pub fn dop() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* dop */ 17, 7])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// opBindingManagement                      ID ::= {module opBindingManagement(18) 9}
/// ```
///
///
pub fn opBindingManagement() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* opBindingManagement */ 18, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// opBindingOIDs                            ID ::= {module opBindingOIDs(19) 9}
/// ```
///
///
pub fn opBindingOIDs() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* opBindingOIDs */ 19, 9])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hierarchicalOperationalBindings          ID ::= {module hierarchicalOperationalBindings(20) 9}
/// ```
///
///
pub fn hierarchicalOperationalBindings() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* hierarchicalOperationalBindings */ 20, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsaOperationalAttributeTypes             ID ::= {module dsaOperationalAttributeTypes(22) 9}
/// ```
///
///
pub fn dsaOperationalAttributeTypes() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* dsaOperationalAttributeTypes */ 22, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// schemaAdministration                     ID ::= {module schemaAdministration(23) 9}
/// ```
///
///
pub fn schemaAdministration() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* schemaAdministration */ 23, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// basicAccessControl                       ID ::= {module basicAccessControl(24) 9}
/// ```
///
///
pub fn basicAccessControl() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* basicAccessControl */ 24, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryOperationalBindingTypes         ID ::= {module directoryOperationalBindingTypes(25) 9}
/// ```
///
///
pub fn directoryOperationalBindingTypes() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* directoryOperationalBindingTypes */ 25, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificateExtensions                    ID ::= {module certificateExtensions(26) 9}
/// ```
///
///
pub fn certificateExtensions() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* certificateExtensions */ 26, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryManagement                      ID ::= {module directoryManagement(27) 9}
/// ```
///
///
pub fn directoryManagement() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* directoryManagement */ 27, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// enhancedSecurity                         ID ::= {module enhancedSecurity(28) 9}
/// ```
///
///
pub fn enhancedSecurity() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* enhancedSecurity */ 28, 9])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directorySecurityExchanges               ID ::= {module directorySecurityExchanges (29) 9}
/// ```
///
///
pub fn directorySecurityExchanges() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* directorySecurityExchanges */ 29, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// iDMProtocolSpecification                 ID ::= {module iDMProtocolSpecification(30) 9}
/// ```
///
///
pub fn iDMProtocolSpecification() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* iDMProtocolSpecification */ 30, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryIDMProtocols                    ID ::= {module directoryIDMProtocols(31) 9}
/// ```
///
///
pub fn directoryIDMProtocols() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* directoryIDMProtocols */ 31, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeCertificateDefinitions          ID ::= {module attributeCertificateDefinitions(32) 9}
/// ```
///
///
pub fn attributeCertificateDefinitions() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* attributeCertificateDefinitions */ 32, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// serviceAdministration                    ID ::= {module serviceAdministration(33) 9}
/// ```
///
///
pub fn serviceAdministration() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* serviceAdministration */ 33, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldapAttributes                           ID ::= {module ldapAttributes(34) 9}
/// ```
///
///
pub fn ldapAttributes() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* ldapAttributes */ 34, 9])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// commonProtocolSpecification              ID ::= {module commonProtocolSpecification(35) 9}
/// ```
///
///
pub fn commonProtocolSpecification() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* commonProtocolSpecification */ 35, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oSIProtocolSpecification                 ID ::= {module oSIProtocolSpecification(36) 9}
/// ```
///
///
pub fn oSIProtocolSpecification() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* oSIProtocolSpecification */ 36, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryOSIProtocols                    ID ::= {module directoryOSIProtocols(37) 9}
/// ```
///
///
pub fn directoryOSIProtocols() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* directoryOSIProtocols */ 37, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldapSystemSchema                         ID ::= {module ldapSystemSchema(38) 9}
/// ```
///
///
pub fn ldapSystemSchema() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* ldapSystemSchema */ 38, 9])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// passwordPolicy                           ID ::= {module passwordPolicy(39) 9}
/// ```
///
///
pub fn passwordPolicy() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* passwordPolicy */ 39, 9])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkiPmiExternalDataTypes                  ID ::= {module pkiPmiExternalDataTypes(40) 9}
/// ```
///
///
pub fn pkiPmiExternalDataTypes() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* pkiPmiExternalDataTypes */ 40, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extensionAttributes                      ID ::= {module extensionAttributes(41) 9}
/// ```
///
///
pub fn extensionAttributes() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* extensionAttributes */ 41, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkiPmiWrapper                            ID ::= {module pkiPmiWrapper(42) 9}
/// ```
///
///
pub fn pkiPmiWrapper() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* pkiPmiWrapper */ 42, 9])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// avlManagement                            ID ::= {module avlManagement(43) 9}
/// ```
///
///
pub fn avlManagement() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([/* avlManagement */ 43, 9])].concat())
    // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// trustBrokerProtocol                      ID ::= {module trustBrokerProtocol(44) 9}
/// ```
///
///
pub fn trustBrokerProtocol() -> ID {
    OBJECT_IDENTIFIER(
        [
            module().0,
            Vec::<u32>::from([/* trustBrokerProtocol */ 44, 9]),
        ]
        .concat(),
    ) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc                                    ID ::= objectClass
/// ```
///
///
pub fn id_oc() -> ID {
    objectClass() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at                                    ID ::= attributeType
/// ```
///
///
pub fn id_at() -> ID {
    attributeType() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as                                    ID ::= abstractSyntax
/// ```
///
///
pub fn id_as() -> ID {
    abstractSyntax() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr                                    ID ::= matchingRule
/// ```
///
///
pub fn id_mr() -> ID {
    matchingRule() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf                                    ID ::= nameForm
/// ```
///
///
pub fn id_nf() -> ID {
    nameForm() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sc                                    ID ::= subentry
/// ```
///
///
pub fn id_sc() -> ID {
    subentry() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa                                    ID ::= operationalAttributeType
/// ```
///
///
pub fn id_oa() -> ID {
    operationalAttributeType() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ob                                    ID ::= operationalBinding
/// ```
///
///
pub fn id_ob() -> ID {
    operationalBinding() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa                                   ID ::= dsaOperationalAttribute
/// ```
///
///
pub fn id_doa() -> ID {
    dsaOperationalAttribute() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kmr                                   ID ::= knowledgeMatchingRule
/// ```
///
///
pub fn id_kmr() -> ID {
    knowledgeMatchingRule() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soc                                   ID ::= schemaObjectClass
/// ```
///
///
pub fn id_soc() -> ID {
    schemaObjectClass() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa                                   ID ::= schemaOperationalAttribute
/// ```
///
///
pub fn id_soa() -> ID {
    schemaOperationalAttribute() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ar                                    ID ::= administrativeRoles
/// ```
///
///
pub fn id_ar() -> ID {
    administrativeRoles() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aca                                   ID ::= accessControlAttribute
/// ```
///
///
pub fn id_aca() -> ID {
    accessControlAttribute() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac                                    ID ::= applicationContext
/// ```
///
///
pub fn id_ac() -> ID {
    applicationContext() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rosObject                             ID ::= rosObject
/// ```
///
///
pub fn id_rosObject() -> ID {
    rosObject() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contract                              ID ::= contract
/// ```
///
///
pub fn id_contract() -> ID {
    contract() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package                               ID ::= package
/// ```
///
///
pub fn id_package() -> ID {
    package() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-acScheme                              ID ::= accessControlSchemes
/// ```
///
///
pub fn id_acScheme() -> ID {
    accessControlSchemes() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce                                    ID ::= certificateExtension
/// ```
///
///
pub fn id_ce() -> ID {
    certificateExtension() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mgt                                   ID ::= managementObject
/// ```
///
///
pub fn id_mgt() -> ID {
    managementObject() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc                                   ID ::= attributeValueContext
/// ```
///
///
pub fn id_avc() -> ID {
    attributeValueContext() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-se                                    ID ::= securityExchange
/// ```
///
///
pub fn id_se() -> ID {
    securityExchange() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-idm                                   ID ::= idmProtocol
/// ```
///
///
pub fn id_idm() -> ID {
    idmProtocol() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr                                    ID ::= problem
/// ```
///
///
pub fn id_pr() -> ID {
    problem() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not                                   ID ::= notification
/// ```
///
///
pub fn id_not() -> ID {
    notification() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mre                                   ID ::= matchingRestriction
/// ```
///
///
pub fn id_mre() -> ID {
    matchingRestriction() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cat                                   ID ::= controlAttributeType
/// ```
///
///
pub fn id_cat() -> ID {
    controlAttributeType() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kp                                    ID ::= keyPurposes
/// ```
///
///
pub fn id_kp() -> ID {
    keyPurposes() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pq                                    ID ::= passwordQuality
/// ```
///
///
pub fn id_pq() -> ID {
    passwordQuality() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ats                                   ID ::= attributeSyntax
/// ```
///
///
pub fn id_ats() -> ID {
    attributeSyntax() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx                                   ID ::= attributeSyntax
/// ```
///
///
pub fn id_asx() -> ID {
    attributeSyntax() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx                                   ID ::= ldap-syntax
/// ```
///
///
pub fn id_lsx() -> ID {
    ldap_syntax() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx                                   ID ::= ldap-x509
/// ```
///
///
pub fn id_ldx() -> ID {
    ldap_x509() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat                                   ID ::= ldap-attr
/// ```
///
///
pub fn id_lat() -> ID {
    ldap_attr() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lmr                                   ID ::= ldap-match
/// ```
///
///
pub fn id_lmr() -> ID {
    ldap_match() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oat                                   ID ::= openLDAP-attributes
/// ```
///
///
pub fn id_oat() -> ID {
    openLDAP_attributes() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-coat                                  ID ::= cosineAttr
/// ```
///
///
pub fn id_coat() -> ID {
    cosineAttr() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avr                                   ID ::= avRestriction
/// ```
///
///
pub fn id_avr() -> ID {
    avRestriction() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cmsct                                 ID ::= cmsContentType
/// ```
///
///
pub fn id_cmsct() -> ID {
    cmsContentType() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// distributedDirectoryObjectIdentifiers ID ::= {module 13}
/// ```
///
///
pub fn distributedDirectoryObjectIdentifiers() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([13])].concat()) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// operationalBindingOIDs                ID ::= {module 25}
/// ```
///
///
pub fn operationalBindingOIDs() -> ID {
    OBJECT_IDENTIFIER([module().0, Vec::<u32>::from([25])].concat()) // OID_GETTER
}
