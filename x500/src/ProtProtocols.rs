#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # ProtProtocols
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `ProtProtocols`.
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
use crate::PKI_Stub::*;
use crate::Wrapper::*;
use asn1::*;

/// ### ASN.1 Definition:
///
/// ```asn1
/// avlProt WRAPPED-PROT ::= {
///                  AvlProt
///   IDENTIFIED BY  id-avlprot }
/// ```
///
///
pub fn avlProt() -> WRAPPED_PROT {
    TYPE_IDENTIFIER {
        id: id_avlprot(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// casubProt WRAPPED-PROT ::= {
///                  CasubProt
///   IDENTIFIED BY  id-casubprot }
/// ```
///
///
pub fn casubProt() -> WRAPPED_PROT {
    TYPE_IDENTIFIER {
        id: id_casubprot(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// tbprot WRAPPED-PROT ::= {
///                  TBprot
///   IDENTIFIED BY  id-tbprot }
/// ```
///
///
pub fn tbprot() -> WRAPPED_PROT {
    TYPE_IDENTIFIER {
        id: id_tbprot(), /* OBJECT_FIELD_SETTING */
    }
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// SupportedProtSet WRAPPED-PROT ::= {avlProt | casubProt | tbprot }
/// ```
///
///
pub fn SupportedProtSet() -> Vec<WRAPPED_PROT> {
    Vec::from([avlProt(), casubProt(), tbprot()])
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-avlprot          OBJECT IDENTIFIER ::= {id-wrprot 0}
/// ```
///
///
pub fn id_avlprot() -> OBJECT_IDENTIFIER {
    [id_wrprot(), Vec::<u32>::from([0])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-casubprot        OBJECT IDENTIFIER ::= {id-wrprot 1}
/// ```
///
///
pub fn id_casubprot() -> OBJECT_IDENTIFIER {
    [id_wrprot(), Vec::<u32>::from([1])].concat() // OID_GETTER
}

/// ### ASN.1 Definition:
///
/// ```asn1
/// id-tbprot           OBJECT IDENTIFIER ::= {id-wrprot 2}
/// ```
///
///
pub fn id_tbprot() -> OBJECT_IDENTIFIER {
    [id_wrprot(), Vec::<u32>::from([2])].concat() // OID_GETTER
}
