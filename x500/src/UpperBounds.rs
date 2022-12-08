#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # UpperBounds
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `UpperBounds`.
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
use std::borrow::Borrow;
use std::sync::Arc;
use x690::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-answerback                              INTEGER ::= 8
/// ```
///
///
pub const ub_answerback: INTEGER = 8;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-business-category                       INTEGER ::= 128
/// ```
///
///
pub const ub_business_category: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-common-name                             INTEGER ::= 64
/// ```
///
///
pub const ub_common_name: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-content                                 INTEGER ::= 32768
/// ```
///
///
pub const ub_content: INTEGER = 32768;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-country-code                            INTEGER ::= 4
/// ```
///
///
pub const ub_country_code: INTEGER = 4;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-description                             INTEGER ::= 1024
/// ```
///
///
pub const ub_description: INTEGER = 1024;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-destination-indicator                   INTEGER ::= 128
/// ```
///
///
pub const ub_destination_indicator: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-directory-string-first-component-match  INTEGER ::= 32768
/// ```
///
///
pub const ub_directory_string_first_component_match: INTEGER = 32768;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-domainLocalID                           INTEGER ::= 64
/// ```
///
///
pub const ub_domainLocalID: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-international-isdn-number               INTEGER ::= 16
/// ```
///
///
pub const ub_international_isdn_number: INTEGER = 16;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-knowledge-information                   INTEGER ::= 32768
/// ```
///
///
pub const ub_knowledge_information: INTEGER = 32768;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-labeledURI                              INTEGER ::= 32768
/// ```
///
///
pub const ub_labeledURI: INTEGER = 32768;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-localeContextSyntax                     INTEGER ::= 128
/// ```
///
///
pub const ub_localeContextSyntax: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-locality-name                           INTEGER ::= 128
/// ```
///
///
pub const ub_locality_name: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-match                                   INTEGER ::= 128
/// ```
///
///
pub const ub_match: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-name                                    INTEGER ::= 128
/// ```
///
///
pub const ub_name: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-organization-name                       INTEGER ::= 64
/// ```
///
///
pub const ub_organization_name: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-organizational-unit-name                INTEGER ::= 64
/// ```
///
///
pub const ub_organizational_unit_name: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-physical-office-name                    INTEGER ::= 128
/// ```
///
///
pub const ub_physical_office_name: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-post-office-box                         INTEGER ::= 40
/// ```
///
///
pub const ub_post_office_box: INTEGER = 40;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-postal-code                             INTEGER ::= 40
/// ```
///
///
pub const ub_postal_code: INTEGER = 40;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-postal-line                             INTEGER ::= 6
/// ```
///
///
pub const ub_postal_line: INTEGER = 6;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-postal-string                           INTEGER ::= 30
/// ```
///
///
pub const ub_postal_string: INTEGER = 30;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-privacy-mark-length                     INTEGER ::= 128
/// ```
///
///
pub const ub_privacy_mark_length: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-pseudonym                               INTEGER ::= 128
/// ```
///
///
pub const ub_pseudonym: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-saslMechanism                           INTEGER ::= 64
/// ```
///
///
pub const ub_saslMechanism: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-schema                                  INTEGER ::= 1024
/// ```
///
///
pub const ub_schema: INTEGER = 1024;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-search                                  INTEGER ::= 32768
/// ```
///
///
pub const ub_search: INTEGER = 32768;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-serial-number                           INTEGER ::= 64
/// ```
///
///
pub const ub_serial_number: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-state-name                              INTEGER ::= 128
/// ```
///
///
pub const ub_state_name: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-street-address                          INTEGER ::= 128
/// ```
///
///
pub const ub_street_address: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-surname                                 INTEGER ::= 64
/// ```
///
///
pub const ub_surname: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-tag                                     INTEGER ::= 64
/// ```
///
///
pub const ub_tag: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-telephone-number                        INTEGER ::= 32
/// ```
///
///
pub const ub_telephone_number: INTEGER = 32;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-teletex-terminal-id                     INTEGER ::= 1024
/// ```
///
///
pub const ub_teletex_terminal_id: INTEGER = 1024;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-telex-number                            INTEGER ::= 14
/// ```
///
///
pub const ub_telex_number: INTEGER = 14;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-title                                   INTEGER ::= 64
/// ```
///
///
pub const ub_title: INTEGER = 64;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-user-password                           INTEGER ::= 128
/// ```
///
///
pub const ub_user_password: INTEGER = 128;

/// ### ASN.1 Definition:
///
/// ```asn1
/// ub-x121-address                            INTEGER ::= 15
/// ```
///
///
pub const ub_x121_address: INTEGER = 15;
