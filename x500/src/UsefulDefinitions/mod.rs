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
    BER.decode_object_identifier(&el)
}

pub fn _encode_ID(value_: &ID) -> ASN1Result<X690Element> {
    BER.encode_object_identifier(&value_)
}

pub fn _validate_ID(el: &X690Element) -> ASN1Result<()> {
    BER.validate_object_identifier(&el)
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ds ID ::= {joint-iso-itu-t ds(5)}
/// ```
///
#[inline]
pub fn ds () -> OBJECT_IDENTIFIER {
	oid!(joint_iso_itu_t,/* ds */ 5) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id ID ::= {joint-iso-itu-t registration-procedures(17) module(1) directory-defs(2)}
/// ```
///
#[inline]
pub fn id () -> OBJECT_IDENTIFIER {
	oid!(joint_iso_itu_t,/* registration-procedures */ 17,/* module */ 1,/* directory-defs */ 2) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// internet            ID ::= {iso(1) identified-organization(3) dod(6) internet(1)}
/// ```
///
#[inline]
pub fn internet () -> OBJECT_IDENTIFIER {
	oid!(/* iso */ 1,/* identified-organization */ 3,/* dod */ 6,/* internet */ 1) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-dir            ID ::= {internet directory(1)}
/// ```
///
#[inline]
pub fn ldap_dir () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, /* directory */ 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// intSecurity         ID ::= {internet security(5)}
/// ```
///
#[inline]
pub fn intSecurity () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, /* security */ 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-enterprise     ID ::= {internet private(4) enterprise(1)}
/// ```
///
#[inline]
pub fn ldap_enterprise () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, /* private */ 4, /* enterprise */ 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-x509           ID ::= {ldap-dir x509(15)}
/// ```
///
#[inline]
pub fn ldap_x509 () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, 1, /* x509 */ 15 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-openLDAP       ID ::= {ldap-enterprise openLDAP(4203) ldap(1)}
/// ```
///
#[inline]
pub fn ldap_openLDAP () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, 4, 1, /* openLDAP */ 4203, /* ldap */ 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// openLDAP-attributes ID ::= {ldap-openLDAP attributeType(3)}
/// ```
///
#[inline]
pub fn openLDAP_attributes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, 4, 1, 0xa0, 107, 1, /* attributeType */ 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// openLDAP-controls   ID ::= {ldap-openLDAP controls(10)}
/// ```
///
#[inline]
pub fn openLDAP_controls () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, 4, 1, 0xa0, 107, 1, /* controls */ 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-wall           ID ::= {ldap-enterprise wahl(1466)}
/// ```
///
#[inline]
pub fn ldap_wall () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, 4, 1, /* wahl */ 1466 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-dynExt         ID ::= {ldap-wall 101 119}
/// ```
///
#[inline]
pub fn ldap_dynExt () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, 4, 1, 0x8b, 58, 101, 119 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-attr           ID ::= {ldap-wall 101 120}
/// ```
///
#[inline]
pub fn ldap_attr () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, 4, 1, 0x8b, 58, 101, 120 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-match          ID ::= {ldap-wall 109 114}
/// ```
///
#[inline]
pub fn ldap_match () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, 4, 1, 0x8b, 58, 109, 114 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldap-syntax         ID ::= {ldap-wall 115 121 1}
/// ```
///
#[inline]
pub fn ldap_syntax () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 43, 6, 1, 4, 1, 0x8b, 58, 115, 121, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cosine              ID ::= {itu-t(0) data(9) pss(2342) ucl(19200300) pilot(100)}
/// ```
///
#[inline]
pub fn cosine () -> OBJECT_IDENTIFIER {
	oid!(/* itu-t */ 0,/* data */ 9,/* pss */ 2342,/* ucl */ 19200300,/* pilot */ 100) // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cosineAttr          ID ::= {cosine pilotAttributeType(1)}
/// ```
///
#[inline]
pub fn cosineAttr () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 9, 0x92, 38, 0x89, 0x93, 0xf2, 44, 100, /* pilotAttributeType */ 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// module                                   ID ::= {ds 1}
/// ```
///
#[inline]
pub fn module () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// serviceElement                           ID ::= {ds 2}
/// ```
///
#[inline]
pub fn serviceElement () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 2 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// applicationContext                       ID ::= {ds 3}
/// ```
///
#[inline]
pub fn applicationContext () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 3 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeType                            ID ::= {ds 4}
/// ```
///
#[inline]
pub fn attributeType () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 4 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeSyntaxVendor                    ID ::= {ds 5}
/// ```
///
#[inline]
pub fn attributeSyntaxVendor () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 5 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// objectClass                              ID ::= {ds 6}
/// ```
///
#[inline]
pub fn objectClass () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 6 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeSet                             ID ::= {ds 7}
/// ```
///
#[inline]
pub fn attributeSet () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// algorithm                                ID ::= {ds 8}
/// ```
///
#[inline]
pub fn algorithm () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 8 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// abstractSyntax                           ID ::= {ds 9}
/// ```
///
#[inline]
pub fn abstractSyntax () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// object                                   ID ::= {ds 10}
/// ```
///
#[inline]
pub fn object () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 10 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// port                                     ID ::= {ds 11}
/// ```
///
#[inline]
pub fn port () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 11 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsaOperationalAttribute                  ID ::= {ds 12}
/// ```
///
#[inline]
pub fn dsaOperationalAttribute () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 12 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// matchingRule                             ID ::= {ds 13}
/// ```
///
#[inline]
pub fn matchingRule () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 13 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// knowledgeMatchingRule                    ID ::= {ds 14}
/// ```
///
#[inline]
pub fn knowledgeMatchingRule () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 14 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// nameForm                                 ID ::= {ds 15}
/// ```
///
#[inline]
pub fn nameForm () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 15 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// group                                    ID ::= {ds 16}
/// ```
///
#[inline]
pub fn group () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 16 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// subentry                                 ID ::= {ds 17}
/// ```
///
#[inline]
pub fn subentry () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 17 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// operationalAttributeType                 ID ::= {ds 18}
/// ```
///
#[inline]
pub fn operationalAttributeType () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 18 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// operationalBinding                       ID ::= {ds 19}
/// ```
///
#[inline]
pub fn operationalBinding () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 19 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// schemaObjectClass                        ID ::= {ds 20}
/// ```
///
#[inline]
pub fn schemaObjectClass () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 20 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// schemaOperationalAttribute               ID ::= {ds 21}
/// ```
///
#[inline]
pub fn schemaOperationalAttribute () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 21 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// administrativeRoles                      ID ::= {ds 23}
/// ```
///
#[inline]
pub fn administrativeRoles () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 23 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// accessControlAttribute                   ID ::= {ds 24}
/// ```
///
#[inline]
pub fn accessControlAttribute () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 24 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// rosObject                                ID ::= {ds 25}
/// ```
///
#[inline]
pub fn rosObject () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 25 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// contract                                 ID ::= {ds 26}
/// ```
///
#[inline]
pub fn contract () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 26 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// package                                  ID ::= {ds 27}
/// ```
///
#[inline]
pub fn package () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 27 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// accessControlSchemes                     ID ::= {ds 28}
/// ```
///
#[inline]
pub fn accessControlSchemes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 28 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificateExtension                     ID ::= {ds 29}
/// ```
///
#[inline]
pub fn certificateExtension () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 29 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// managementObject                         ID ::= {ds 30}
/// ```
///
#[inline]
pub fn managementObject () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 30 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeValueContext                    ID ::= {ds 31}
/// ```
///
#[inline]
pub fn attributeValueContext () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 31 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// securityExchange                         ID ::= {ds 32}
/// ```
///
#[inline]
pub fn securityExchange () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 32 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// idmProtocol                              ID ::= {ds 33}
/// ```
///
#[inline]
pub fn idmProtocol () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 33 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// problem                                  ID ::= {ds 34}
/// ```
///
#[inline]
pub fn problem () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 34 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// notification                             ID ::= {ds 35}
/// ```
///
#[inline]
pub fn notification () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 35 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// matchingRestriction                      ID ::= {ds 36}
/// ```
///
#[inline]
pub fn matchingRestriction () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 36 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// controlAttributeType                     ID ::= {ds 37}
/// ```
///
#[inline]
pub fn controlAttributeType () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 37 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// keyPurposes                              ID ::= {ds 38}
/// ```
///
#[inline]
pub fn keyPurposes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 38 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// passwordQuality                          ID ::= {ds 39}
/// ```
///
#[inline]
pub fn passwordQuality () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 39 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeSyntax                          ID ::= {ds 40}
/// ```
///
#[inline]
pub fn attributeSyntax () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 40 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// avRestriction                            ID ::= {ds 41}
/// ```
///
#[inline]
pub fn avRestriction () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 41 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// cmsContentType                           ID ::= {ds 42}
/// ```
///
#[inline]
pub fn cmsContentType () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 42 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// usefulDefinitions                        ID ::= {module usefulDefinitions(0) 9}
/// ```
///
#[inline]
pub fn usefulDefinitions () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* usefulDefinitions */ 0, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// informationFramework                     ID ::= {module informationFramework(1) 9}
/// ```
///
#[inline]
pub fn informationFramework () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* informationFramework */ 1, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryAbstractService                 ID ::= {module directoryAbstractService(2) 9}
/// ```
///
#[inline]
pub fn directoryAbstractService () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* directoryAbstractService */ 2, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// distributedOperations                    ID ::= {module distributedOperations(3) 9}
/// ```
///
#[inline]
pub fn distributedOperations () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* distributedOperations */ 3, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// protocolObjectIdentifiers                ID ::= {module protocolObjectIdentifiers(4) 9}
/// ```
///
#[inline]
pub fn protocolObjectIdentifiers () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* protocolObjectIdentifiers */ 4, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// selectedAttributeTypes                   ID ::= {module selectedAttributeTypes(5) 9}
/// ```
///
#[inline]
pub fn selectedAttributeTypes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* selectedAttributeTypes */ 5, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// selectedObjectClasses                    ID ::= {module selectedObjectClasses(6) 9}
/// ```
///
#[inline]
pub fn selectedObjectClasses () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* selectedObjectClasses */ 6, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// authenticationFramework                  ID ::= {module authenticationFramework(7) 9}
/// ```
///
#[inline]
pub fn authenticationFramework () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* authenticationFramework */ 7, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// algorithmObjectIdentifiers               ID ::= {module algorithmObjectIdentifiers(8) 9}
/// ```
///
#[inline]
pub fn algorithmObjectIdentifiers () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* algorithmObjectIdentifiers */ 8, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryObjectIdentifiers               ID ::= {module directoryObjectIdentifiers(9) 9}
/// ```
///
#[inline]
pub fn directoryObjectIdentifiers () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* directoryObjectIdentifiers */ 9, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// upperBounds                              ID ::= {module upperBounds(10) 9}
/// ```
///
#[inline]
pub fn upperBounds () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* upperBounds */ 10, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dap                                      ID ::= {module dap(11) 9}
/// ```
///
#[inline]
pub fn dap () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* dap */ 11, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsp                                      ID ::= {module dsp(12) 9}
/// ```
///
#[inline]
pub fn dsp () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* dsp */ 12, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// distributedDirectoryOIDs                 ID ::= {module distributedDirectoryOIDs(13) 9}
/// ```
///
#[inline]
pub fn distributedDirectoryOIDs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* distributedDirectoryOIDs */ 13, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryShadowOIDs                      ID ::= {module directoryShadowOIDs(14) 9}
/// ```
///
#[inline]
pub fn directoryShadowOIDs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* directoryShadowOIDs */ 14, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryShadowAbstractService           ID ::= {module directoryShadowAbstractService(15) 9}
/// ```
///
#[inline]
pub fn directoryShadowAbstractService () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* directoryShadowAbstractService */ 15, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// disp                                     ID ::= {module disp(16) 7}
/// ```
///
#[inline]
pub fn disp () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* disp */ 16, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dop                                      ID ::= {module dop(17) 7}
/// ```
///
#[inline]
pub fn dop () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* dop */ 17, 7 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// opBindingManagement                      ID ::= {module opBindingManagement(18) 9}
/// ```
///
#[inline]
pub fn opBindingManagement () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* opBindingManagement */ 18, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// opBindingOIDs                            ID ::= {module opBindingOIDs(19) 9}
/// ```
///
#[inline]
pub fn opBindingOIDs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* opBindingOIDs */ 19, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// hierarchicalOperationalBindings          ID ::= {module hierarchicalOperationalBindings(20) 9}
/// ```
///
#[inline]
pub fn hierarchicalOperationalBindings () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* hierarchicalOperationalBindings */ 20, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// dsaOperationalAttributeTypes             ID ::= {module dsaOperationalAttributeTypes(22) 9}
/// ```
///
#[inline]
pub fn dsaOperationalAttributeTypes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* dsaOperationalAttributeTypes */ 22, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// schemaAdministration                     ID ::= {module schemaAdministration(23) 9}
/// ```
///
#[inline]
pub fn schemaAdministration () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* schemaAdministration */ 23, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// basicAccessControl                       ID ::= {module basicAccessControl(24) 9}
/// ```
///
#[inline]
pub fn basicAccessControl () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* basicAccessControl */ 24, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryOperationalBindingTypes         ID ::= {module directoryOperationalBindingTypes(25) 9}
/// ```
///
#[inline]
pub fn directoryOperationalBindingTypes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* directoryOperationalBindingTypes */ 25, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// certificateExtensions                    ID ::= {module certificateExtensions(26) 9}
/// ```
///
#[inline]
pub fn certificateExtensions () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* certificateExtensions */ 26, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryManagement                      ID ::= {module directoryManagement(27) 9}
/// ```
///
#[inline]
pub fn directoryManagement () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* directoryManagement */ 27, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// enhancedSecurity                         ID ::= {module enhancedSecurity(28) 9}
/// ```
///
#[inline]
pub fn enhancedSecurity () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* enhancedSecurity */ 28, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directorySecurityExchanges               ID ::= {module directorySecurityExchanges (29) 9}
/// ```
///
#[inline]
pub fn directorySecurityExchanges () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* directorySecurityExchanges */ 29, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// iDMProtocolSpecification                 ID ::= {module iDMProtocolSpecification(30) 9}
/// ```
///
#[inline]
pub fn iDMProtocolSpecification () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* iDMProtocolSpecification */ 30, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryIDMProtocols                    ID ::= {module directoryIDMProtocols(31) 9}
/// ```
///
#[inline]
pub fn directoryIDMProtocols () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* directoryIDMProtocols */ 31, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// attributeCertificateDefinitions          ID ::= {module attributeCertificateDefinitions(32) 9}
/// ```
///
#[inline]
pub fn attributeCertificateDefinitions () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* attributeCertificateDefinitions */ 32, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// serviceAdministration                    ID ::= {module serviceAdministration(33) 9}
/// ```
///
#[inline]
pub fn serviceAdministration () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* serviceAdministration */ 33, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldapAttributes                           ID ::= {module ldapAttributes(34) 9}
/// ```
///
#[inline]
pub fn ldapAttributes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* ldapAttributes */ 34, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// commonProtocolSpecification              ID ::= {module commonProtocolSpecification(35) 9}
/// ```
///
#[inline]
pub fn commonProtocolSpecification () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* commonProtocolSpecification */ 35, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// oSIProtocolSpecification                 ID ::= {module oSIProtocolSpecification(36) 9}
/// ```
///
#[inline]
pub fn oSIProtocolSpecification () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* oSIProtocolSpecification */ 36, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// directoryOSIProtocols                    ID ::= {module directoryOSIProtocols(37) 9}
/// ```
///
#[inline]
pub fn directoryOSIProtocols () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* directoryOSIProtocols */ 37, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// ldapSystemSchema                         ID ::= {module ldapSystemSchema(38) 9}
/// ```
///
#[inline]
pub fn ldapSystemSchema () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* ldapSystemSchema */ 38, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// passwordPolicy                           ID ::= {module passwordPolicy(39) 9}
/// ```
///
#[inline]
pub fn passwordPolicy () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* passwordPolicy */ 39, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkiPmiExternalDataTypes                  ID ::= {module pkiPmiExternalDataTypes(40) 9}
/// ```
///
#[inline]
pub fn pkiPmiExternalDataTypes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* pkiPmiExternalDataTypes */ 40, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// extensionAttributes                      ID ::= {module extensionAttributes(41) 9}
/// ```
///
#[inline]
pub fn extensionAttributes () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* extensionAttributes */ 41, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// pkiPmiWrapper                            ID ::= {module pkiPmiWrapper(42) 9}
/// ```
///
#[inline]
pub fn pkiPmiWrapper () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* pkiPmiWrapper */ 42, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// avlManagement                            ID ::= {module avlManagement(43) 9}
/// ```
///
#[inline]
pub fn avlManagement () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* avlManagement */ 43, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// trustBrokerProtocol                      ID ::= {module trustBrokerProtocol(44) 9}
/// ```
///
#[inline]
pub fn trustBrokerProtocol () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, /* trustBrokerProtocol */ 44, 9 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oc                                    ID ::= objectClass
/// ```
///
#[inline]
pub fn id_oc() -> ID {
    objectClass() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-at                                    ID ::= attributeType
/// ```
///
#[inline]
pub fn id_at() -> ID {
    attributeType() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-as                                    ID ::= abstractSyntax
/// ```
///
#[inline]
pub fn id_as() -> ID {
    abstractSyntax() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mr                                    ID ::= matchingRule
/// ```
///
#[inline]
pub fn id_mr() -> ID {
    matchingRule() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-nf                                    ID ::= nameForm
/// ```
///
#[inline]
pub fn id_nf() -> ID {
    nameForm() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-sc                                    ID ::= subentry
/// ```
///
#[inline]
pub fn id_sc() -> ID {
    subentry() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oa                                    ID ::= operationalAttributeType
/// ```
///
#[inline]
pub fn id_oa() -> ID {
    operationalAttributeType() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ob                                    ID ::= operationalBinding
/// ```
///
#[inline]
pub fn id_ob() -> ID {
    operationalBinding() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-doa                                   ID ::= dsaOperationalAttribute
/// ```
///
#[inline]
pub fn id_doa() -> ID {
    dsaOperationalAttribute() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kmr                                   ID ::= knowledgeMatchingRule
/// ```
///
#[inline]
pub fn id_kmr() -> ID {
    knowledgeMatchingRule() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soc                                   ID ::= schemaObjectClass
/// ```
///
#[inline]
pub fn id_soc() -> ID {
    schemaObjectClass() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-soa                                   ID ::= schemaOperationalAttribute
/// ```
///
#[inline]
pub fn id_soa() -> ID {
    schemaOperationalAttribute() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ar                                    ID ::= administrativeRoles
/// ```
///
#[inline]
pub fn id_ar() -> ID {
    administrativeRoles() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-aca                                   ID ::= accessControlAttribute
/// ```
///
#[inline]
pub fn id_aca() -> ID {
    accessControlAttribute() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ac                                    ID ::= applicationContext
/// ```
///
#[inline]
pub fn id_ac() -> ID {
    applicationContext() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-rosObject                             ID ::= rosObject
/// ```
///
#[inline]
pub fn id_rosObject() -> ID {
    rosObject() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-contract                              ID ::= contract
/// ```
///
#[inline]
pub fn id_contract() -> ID {
    contract() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-package                               ID ::= package
/// ```
///
#[inline]
pub fn id_package() -> ID {
    package() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-acScheme                              ID ::= accessControlSchemes
/// ```
///
#[inline]
pub fn id_acScheme() -> ID {
    accessControlSchemes() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ce                                    ID ::= certificateExtension
/// ```
///
#[inline]
pub fn id_ce() -> ID {
    certificateExtension() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mgt                                   ID ::= managementObject
/// ```
///
#[inline]
pub fn id_mgt() -> ID {
    managementObject() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avc                                   ID ::= attributeValueContext
/// ```
///
#[inline]
pub fn id_avc() -> ID {
    attributeValueContext() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-se                                    ID ::= securityExchange
/// ```
///
#[inline]
pub fn id_se() -> ID {
    securityExchange() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-idm                                   ID ::= idmProtocol
/// ```
///
#[inline]
pub fn id_idm() -> ID {
    idmProtocol() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pr                                    ID ::= problem
/// ```
///
#[inline]
pub fn id_pr() -> ID {
    problem() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-not                                   ID ::= notification
/// ```
///
#[inline]
pub fn id_not() -> ID {
    notification() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-mre                                   ID ::= matchingRestriction
/// ```
///
#[inline]
pub fn id_mre() -> ID {
    matchingRestriction() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cat                                   ID ::= controlAttributeType
/// ```
///
#[inline]
pub fn id_cat() -> ID {
    controlAttributeType() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-kp                                    ID ::= keyPurposes
/// ```
///
#[inline]
pub fn id_kp() -> ID {
    keyPurposes() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-pq                                    ID ::= passwordQuality
/// ```
///
#[inline]
pub fn id_pq() -> ID {
    passwordQuality() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ats                                   ID ::= attributeSyntax
/// ```
///
#[inline]
pub fn id_ats() -> ID {
    attributeSyntax() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-asx                                   ID ::= attributeSyntax
/// ```
///
#[inline]
pub fn id_asx() -> ID {
    attributeSyntax() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lsx                                   ID ::= ldap-syntax
/// ```
///
#[inline]
pub fn id_lsx() -> ID {
    ldap_syntax() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-ldx                                   ID ::= ldap-x509
/// ```
///
#[inline]
pub fn id_ldx() -> ID {
    ldap_x509() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lat                                   ID ::= ldap-attr
/// ```
///
#[inline]
pub fn id_lat() -> ID {
    ldap_attr() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-lmr                                   ID ::= ldap-match
/// ```
///
#[inline]
pub fn id_lmr() -> ID {
    ldap_match() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-oat                                   ID ::= openLDAP-attributes
/// ```
///
#[inline]
pub fn id_oat() -> ID {
    openLDAP_attributes() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-coat                                  ID ::= cosineAttr
/// ```
///
#[inline]
pub fn id_coat() -> ID {
    cosineAttr() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avr                                   ID ::= avRestriction
/// ```
///
#[inline]
pub fn id_avr() -> ID {
    avRestriction() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-cmsct                                 ID ::= cmsContentType
/// ```
///
#[inline]
pub fn id_cmsct() -> ID {
    cmsContentType() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// distributedDirectoryObjectIdentifiers ID ::= {module 13}
/// ```
///
#[inline]
pub fn distributedDirectoryObjectIdentifiers () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, 13 ].as_slice()) } // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// operationalBindingOIDs                ID ::= {module 25}
/// ```
///
#[inline]
pub fn operationalBindingOIDs () -> OBJECT_IDENTIFIER {
	unsafe { OBJECT_IDENTIFIER::from_x690_slice_unchecked([ 85, 1, 25 ].as_slice()) } // OID_GETTER
}
