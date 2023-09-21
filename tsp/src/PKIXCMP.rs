#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
//! # PKIXCMP
//!
//! Rust types, and their encoding and decoding to and from X.690 encodings
//! (such as the Basic Encoding Rules and Distinguished Encoding Rules), based
//! on the productions defined in the ASN.1 module `PKIXCMP`.
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
/// PKIFreeText  ::=  SEQUENCE SIZE (1..MAX) OF UTF8String
/// ```
pub type PKIFreeText = Vec<UTF8String>; // SequenceOfType

pub fn _decode_PKIFreeText(el: &X690Element) -> ASN1Result<PKIFreeText> {
    let elements = match &el.value {
        X690Encoding::Constructed(children) => children,
        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    };
    let mut items: SEQUENCE_OF<UTF8String> = Vec::with_capacity(elements.len());
    for el in elements {
        items.push(BER.decode_utf8_string(el)?);
    }
    Ok(items)
}

pub fn _encode_PKIFreeText(value_: &PKIFreeText) -> ASN1Result<X690Element> {
    let mut children: Vec<X690Element> = Vec::with_capacity(value_.len());
    for v in value_ {
        children.push(BER.encode_utf8_string(&v)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF),
        X690Value::Constructed(Arc::new(children)),
    ))
}

pub fn _validate_PKIFreeText(el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                BER.validate_utf8_string(&sub)?;
            }
            Ok(())
        }
        _ => Err(el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "PKIFreeText")),
    }
}
