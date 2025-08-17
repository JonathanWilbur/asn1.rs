#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PKI-Stub
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PKI-Stub`.
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
pub mod PKI_Stub;
pub mod display;
pub mod eq;
pub mod hash;
pub mod oids;
pub mod othername;
pub mod parse;
pub mod pki;
pub mod unescape;
pub mod utils;
pub use crate::PKI_Stub::*;
pub use crate::oids::{common_attr_type_to_long_name, common_attr_type_to_short_name};
use std::str::FromStr;
use wildboar_asn1::utils::read_i64;
use wildboar_asn1::{
    ASN1Error, OBJECT_IDENTIFIER, TagClass, UNIV_TAG_BIT_STRING, UNIV_TAG_BMP_STRING,
    UNIV_TAG_BOOLEAN, UNIV_TAG_DATE, UNIV_TAG_DATE_TIME, UNIV_TAG_DURATION, UNIV_TAG_ENUMERATED,
    UNIV_TAG_GENERAL_STRING, UNIV_TAG_GENERALIZED_TIME, UNIV_TAG_GRAPHIC_STRING,
    UNIV_TAG_IA5_STRING, UNIV_TAG_INTEGER, UNIV_TAG_NULL, UNIV_TAG_NUMERIC_STRING,
    UNIV_TAG_OBJECT_DESCRIPTOR, UNIV_TAG_OBJECT_IDENTIFIER, UNIV_TAG_OCTET_STRING,
    UNIV_TAG_OID_IRI, UNIV_TAG_PRINTABLE_STRING, UNIV_TAG_REAL, UNIV_TAG_RELATIVE_OID,
    UNIV_TAG_RELATIVE_OID_IRI, UNIV_TAG_TIME, UNIV_TAG_TIME_OF_DAY, UNIV_TAG_UNIVERSAL_STRING,
    UNIV_TAG_UTC_TIME, UNIV_TAG_UTF8_STRING, UNIV_TAG_VISIBLE_STRING,
};
use x690::{BER, X690Codec, X690Element, x690_write_tlv};

pub fn unrecognized_value_to_string(value: &X690Element) -> String {
    let mut encoding: Vec<u8> = Vec::with_capacity(value.len() + 2);
    x690_write_tlv(&mut encoding, &value).unwrap_or_default();
    format!("#{}", hex::encode(&encoding))
}

pub trait DisplayX500AttributeType {
    fn attr_type_to_name(self: &Self, attr_type: &OBJECT_IDENTIFIER) -> Option<&str> {
        self.attr_type_to_long_name(attr_type).or(self
            .attr_type_to_short_name(attr_type)
            .or(self.attr_type_to_descriptor(attr_type)))
    }

    // Same as `attr_type_to_name`, but it prefers shorter names.
    fn attr_type_to_shortest_name(self: &Self, attr_type: &OBJECT_IDENTIFIER) -> Option<&str> {
        self.attr_type_to_short_name(attr_type).or(self
            .attr_type_to_long_name(attr_type)
            .or(self.attr_type_to_descriptor(attr_type)))
    }

    #[inline]
    fn attr_type_to_descriptor(self: &Self, _: &OBJECT_IDENTIFIER) -> Option<&str> {
        None
    }

    #[inline]
    fn attr_type_to_long_name(self: &Self, attr_type: &OBJECT_IDENTIFIER) -> Option<&str> {
        common_attr_type_to_long_name(attr_type)
    }

    #[inline]
    fn attr_type_to_short_name(self: &Self, attr_type: &OBJECT_IDENTIFIER) -> Option<&str> {
        common_attr_type_to_short_name(attr_type)
    }
}

pub struct DefaultX500ValueDisplayer;

impl DisplayX500AttributeType for DefaultX500ValueDisplayer {}
