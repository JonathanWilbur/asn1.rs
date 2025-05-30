#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # DirectoryOperationalBindingTypes
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `DirectoryOperationalBindingTypes`.
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
use crate::UsefulDefinitions::*;
use wildboar_asn1::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-op-binding-shadow                     OBJECT IDENTIFIER ::= {id-ob 1}
/// ```
///
///
#[inline]
pub fn id_op_binding_shadow () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ob(), 1).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-op-binding-hierarchical               OBJECT IDENTIFIER ::= {id-ob 2}
/// ```
///
///
#[inline]
pub fn id_op_binding_hierarchical () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ob(), 2).unwrap() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-op-binding-non-specific-hierarchical  OBJECT IDENTIFIER ::= {id-ob 3}
/// ```
///
///
#[inline]
pub fn id_op_binding_non_specific_hierarchical () -> OBJECT_IDENTIFIER {
	OBJECT_IDENTIFIER::from_prefix_and_arc(id_ob(), 3).unwrap() // OID_GETTER
}
