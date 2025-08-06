//! # X.690 Encoding Rules Library
//!
//! This library provides comprehensive support for X.690 encoding rules, which define how ASN.1
//! (Abstract Syntax Notation One) data structures are encoded for transmission and storage.
//!
//! ## Overview
//!
//! X.690 defines several encoding rules:
//! - **BER (Basic Encoding Rules)**: The most flexible encoding, supporting both definite and indefinite lengths
//! - **CER (Canonical Encoding Rules)**: A restricted form of BER that produces canonical encodings
//! - **DER (Distinguished Encoding Rules)**: A restricted form of BER that produces unique encodings
//!
//! This library focuses on BER encoding and decoding, providing a complete implementation
//! of the X.690 specification.
//!
//! ## Key Features
//!
//! - Complete BER encoding and decoding support
//! - Support for all ASN.1 universal types
//! - Efficient memory management with zero-copy operations where possible
//! - Comprehensive error handling with detailed error information
//! - Support for both definite and indefinite length encoding
//! - Tag and length encoding/decoding utilities

pub mod ber;
#[cfg(feature = "der")]
pub mod der;
pub mod codec;
pub mod parsing;
pub(crate) mod utils;

pub use crate::ber::*;
pub use crate::codec::*;
pub use crate::parsing::*;
pub use crate::utils::primitive;

use crate::utils::likely;
use wildboar_asn1::error::{ASN1Error, ASN1ErrorCode, ASN1Result};
use wildboar_asn1::{
    ByteSlice, CharacterString, EmbeddedPDV, ExternalEncoding,
    ExternalIdentification, GeneralizedTime,
    PresentationContextSwitchingTypeIdentification, Tag, TagClass, TagNumber,
    UTCTime,
    UNIV_TAG_INTEGER,
    UNIV_TAG_OBJECT_IDENTIFIER, UNIV_TAG_OCTET_STRING,
    UNIV_TAG_OBJECT_DESCRIPTOR, BIT_STRING, BOOLEAN, DATE, DATE_TIME,
    DURATION_EQUIVALENT, EXTERNAL, INTEGER, OBJECT_IDENTIFIER,
    OCTET_STRING, REAL, RELATIVE_OID, TIME, TIME_OF_DAY,
};
use wildboar_asn1::{ENUMERATED, read_i64, DURATION, ComponentSpec, TagSelector};
use std::borrow::Cow;
use std::io::{Error, ErrorKind, Result, Write};
use std::mem::size_of;
use std::sync::Arc;
use bytes::{Bytes, BytesMut, BufMut};

/// Tag class bits for `UNIVERSAL` tags in X.690 encoding
pub const X690_TAG_CLASS_UNIVERSAL: u8 = 0b0000_0000;

/// Tag class bits for `APPLICATION` tags in X.690 encoding
pub const X690_TAG_CLASS_APPLICATION: u8 = 0b0100_0000;

/// Tag class bits for `CONTEXT` tags in X.690 encoding
pub const X690_TAG_CLASS_CONTEXT: u8 = 0b1000_0000;

/// Tag class bits for `PRIVATE` tags in X.690 encoding
pub const X690_TAG_CLASS_PRIVATE: u8 = 0b1100_0000;

/// Special `REAL` value constant for positive infinity
pub const X690_SPECIAL_REAL_PLUS_INFINITY: u8 = 0b0000_0000;

/// Special `REAL` value constant for negative infinity
pub const X690_SPECIAL_REAL_MINUS_INFINITY: u8 = 0b0000_0001;

/// Special `REAL` value constant for Not-a-Number (NaN)
pub const X690_SPECIAL_REAL_NOT_A_NUMBER: u8 = 0b0000_0010;

/// Special `REAL` value constant for negative zero
pub const X690_SPECIAL_REAL_MINUS_ZERO: u8 = 0b0000_0011;

/// Flag indicating a special `REAL` value
pub const X690_REAL_SPECIAL: u8 = 0b0100_0000;

/// Base encoding constant for base-10 `REAL` values
pub const X690_REAL_BASE10: u8 = 0b0000_0000;

/// Base encoding constant for binary `REAL` values
pub const X690_REAL_BINARY: u8 = 0b1000_0000;

/// Sign bit constant for positive `REAL` values
pub const X690_REAL_POSITIVE: u8 = 0b0000_0000;

/// Sign bit constant for negative `REAL` values
pub const X690_REAL_NEGATIVE: u8 = 0b0100_0000;

/// Bit mask for extracting the sign bit from `REAL` encoding
pub const X690_REAL_SIGN_MASK: u8 = 0b0100_0000;

/// Bit mask for extracting the base bits from `REAL` encoding
pub const X690_REAL_BASE_MASK: u8 = 0b0011_0000;

/// Base encoding constant for base-2 `REAL` values
pub const X690_REAL_BASE_2: u8 = 0b0000_0000;

/// Base encoding constant for base-8 `REAL` values
pub const X690_REAL_BASE_8: u8 = 0b0001_0000;

/// Base encoding constant for base-16 `REAL` values
pub const X690_REAL_BASE_16: u8 = 0b0010_0000;

/// Reserved base encoding constant for `REAL` values
pub const X690_REAL_BASE_RESERVED: u8 = 0b0011_0000;

/// Bit mask for extracting binary scaling factor from `REAL` encoding
pub const X690_REAL_BINARY_SCALING_MASK: u8 = 0b0000_1100;

/// Bit mask for extracting exponent format from `REAL` encoding
pub const X690_REAL_EXPONENT_FORMAT_MASK: u8 = 0b0000_0011;

/// Exponent format constant for 1-octet exponent
pub const X690_REAL_EXPONENT_FORMAT_1_OCTET: u8 = 0b0000_0000;

/// Exponent format constant for 2-octet exponent
pub const X690_REAL_EXPONENT_FORMAT_2_OCTET: u8 = 0b0000_0001;

/// Exponent format constant for 3-octet exponent
pub const X690_REAL_EXPONENT_FORMAT_3_OCTET: u8 = 0b0000_0010;

/// Exponent format constant for variable-length exponent
pub const X690_REAL_EXPONENT_FORMAT_VAR_OCTET: u8 = 0b0000_0011;

/// ISO 6093 NR1 format constant for `REAL` encoding
pub const X690_REAL_NR1: u8 = 1;

/// ISO 6093 NR2 format constant for `REAL` encoding
pub const X690_REAL_NR2: u8 = 2;

/// ISO 6093 NR3 format constant for `REAL` encoding
pub const X690_REAL_NR3: u8 = 3;

/// Represents the length of an X.690 encoded element
///
/// In X.690 encoding, lengths can be either definite (a specific number of octets)
/// or indefinite (marked with a special value and terminated by end-of-content markers).
#[derive(Clone, Debug, Hash, Copy, PartialEq, Eq)]
pub enum X690Length {
    /// Definite length with a specific number of octets
    Definite(usize),
    /// Indefinite length, terminated by end-of-content markers
    Indefinite,
}

/// Represents the value content of an X.690 encoded element
///
/// X.690 values can be stored in different forms depending on how they were created
/// and whether they need to be serialized for transmission.
#[derive(Clone, Debug, Hash)]
pub enum X690Value {
    /// A primitive value stored as raw bytes
    Primitive(Bytes),
    /// A constructed value containing child elements
    Constructed(Arc<Vec<X690Element>>),
    /// A value that has been serialized to bytes (for lazy decoding or faster encoding)
    Serialized(Bytes),
}

impl X690Value {

    /// Returns the length of the content octets in bytes
    ///
    /// For primitive values, this is the length of the raw bytes.
    /// For constructed values, this is the sum of all child element lengths.
    /// For serialized values, this decodes the serialized data to determine the length.
    pub fn len(&self) -> usize {
        match self {
            X690Value::Primitive(v) => v.len(),
            X690Value::Constructed(components) => {
                let mut sum: usize = 0;
                for component in components.iter() {
                    sum += component.len();
                }
                sum
            },
            X690Value::Serialized(v) => {
                match BER.decode_from_slice(&v) {
                    Ok((_, el)) => el.len(),
                    Err(_) => return 0,
                }
            }
        }
    }

    /// Creates a constructed value from a single explicit element
    ///
    /// This is used when an element needs to be wrapped in an explicit tag.
    #[inline]
    pub fn from_explicit(inner: X690Element) -> Self {
        X690Value::Constructed(Arc::new(Vec::from([ inner ])))
    }

    /// Returns the components of a constructed value
    ///
    /// For constructed values, returns the child elements.
    /// For serialized values, decodes the serialized data and returns the components.
    /// For primitive values, returns an error.
    pub fn components(&self) -> ASN1Result<Arc<Vec<X690Element>>> {
        match self {
            X690Value::Constructed(components) => Ok(components.clone()),
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                el.value.components()
            },
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }

}

/// Represents a complete X.690 encoded element with tag and value
///
/// An `X690Element` contains a tag that identifies the type and class of the element,
/// and a value that contains the actual data. The value can be primitive (raw bytes),
/// constructed (containing child elements), or serialized (encoded bytes).
#[derive(Clone, Debug, Hash)]
pub struct X690Element {
    /// The tag identifying the type and class of this element
    pub tag: Tag,
    /// The value content of this element
    pub value: X690Value,
}

impl X690Element {

    /// Creates a new `X690Element` with the specified tag and value
    #[inline]
    pub const fn new(tag: Tag, value: X690Value) -> X690Element {
        X690Element { tag, value }
    }

    /// Returns the total length of this element in bytes when encoded
    ///
    /// This includes the tag bytes, length bytes, and value bytes.
    pub fn len(&self) -> usize {
        let tag_length: usize = get_written_x690_tag_length(self.tag.tag_number);
        let value_length = self.value.len();
        let length_length: usize = get_written_x690_length_length(value_length);
        let ret = tag_length + length_length + value_length;
        ret
    }

    /// Returns true if this element is constructed (contains child elements)
    ///
    /// For serialized values, this checks the constructed bit in the tag.
    /// For constructed values, this always returns true.
    /// For primitive values, this always returns false.
    #[inline]
    pub fn is_constructed (&self) -> bool {
        if let X690Value::Serialized(v) = &self.value {
            return v.get(0).is_some_and(|b| (*b & 0b0010_0000) == 0b0010_0000);
        }
        if let X690Value::Constructed(_) = self.value {
            true
        } else {
            false
        }
    }

    /// Returns the components of this element if it is constructed
    ///
    /// This is a convenience method that delegates to the value's [`X690Value::components`] method.
    #[inline]
    pub fn components (&self) -> ASN1Result<Arc<Vec<X690Element>>> {
        self.value.components()
    }

    /// Returns the inner element if this is an explicit wrapper
    ///
    /// For explicit tagged values, this returns the single child element.
    /// For other values, this returns an error.
    pub fn inner(&self) -> ASN1Result<X690Element> {
        match &self.value {
            X690Value::Constructed(components) => {
                if components.len() != 1 {
                    return Err(self.to_asn1_error(ASN1ErrorCode::invalid_construction));
                }
                Ok(components[0].clone())
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v)?;
                el.inner()
            },
            _ => Err(self.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    /// Returns the content octets of this element
    ///
    /// For primitive values, returns the raw bytes.
    /// For constructed values, serializes the child elements and returns the bytes.
    /// For serialized values, decodes and returns the content octets.
    pub fn content_octets <'a> (&'a self) -> ASN1Result<Cow<'a, [u8]>> {
        match &self.value {
            X690Value::Primitive(v) => Ok(Cow::Borrowed(&v)),
            X690Value::Constructed(_) => {
                let mut output = BytesMut::with_capacity(self.len()).writer();
                x690_write_value(&mut output, &self.value)?;
                Ok(Cow::Owned(output.into_inner().into()))
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(v).unwrap();
                match el.value {
                    X690Value::Primitive(inner) => Ok(Cow::Owned(inner.to_vec())),
                    X690Value::Constructed(_) => {
                        let mut output = BytesMut::with_capacity(el.len()).writer();
                        x690_write_value(&mut output, &el.value)?;
                        Ok(Cow::Owned(output.into_inner().into()))
                    },
                    _ => panic!("ASN.1 / X.690 decoding returned serialized value"),
                }
            }
        }
    }

    /// Creates an `ASN1Error` with information from this element
    ///
    /// This is useful for creating detailed error messages that include
    /// information about the element that caused the error.
    #[inline]
    pub fn to_asn1_error (&self, errcode: ASN1ErrorCode) -> ASN1Error {
        ASN1Error {
            error_code: errcode,
            component_name: None,
            tag: Some(Tag::new(self.tag.tag_class, self.tag.tag_number)),
            length: Some(self.len()),
            constructed: Some(self.is_constructed()),
            value_preview: None,
            bytes_read: None,
            values_read: None,
            err_source: None,
        }
    }

    /// Creates an `ASN1Error` with information from this element and a component name
    ///
    /// This is useful for creating detailed error messages that include
    /// information about the element and the specific component that caused the error.
    pub fn to_asn1_err_named (&self, errcode: ASN1ErrorCode, name: &str) -> ASN1Error {
        let mut e = self.to_asn1_error(errcode);
        e.component_name = Some(name.to_string());
        e
    }

    /// Returns `true` if this element is empty
    ///
    /// For primitive values, checks if the byte length is zero.
    /// For constructed values, checks if there are no child elements.
    /// For serialized values, checks if the serialized data is minimal (just tag and length).
    #[inline]
    pub fn is_empty (&self) -> bool {
        match &self.value {
            X690Value::Primitive(v) => v.len() == 0,
            X690Value::Constructed(components) => components.len() == 0,
            X690Value::Serialized(v) => v.len() <= 2,
        }
    }

}

impl From<i8> for X690Element {
    /// Converts an `i8` to an `X690Element` by encoding it as an `INTEGER`
    #[inline]
    fn from(value: i8) -> Self {
        BER.encode_i8(value).unwrap()
    }
}

impl From<i16> for X690Element {
    /// Converts an `i16` to an `X690Element` by encoding it as an `INTEGER`
    #[inline]
    fn from(value: i16) -> Self {
        BER.encode_i16(value).unwrap()
    }
}

impl From<i32> for X690Element {
    /// Converts an `i32` to an `X690Element` by encoding it as an `INTEGER`
    #[inline]
    fn from(value: i32) -> Self {
        BER.encode_i32(value).unwrap()
    }
}

impl From<i64> for X690Element {
    /// Converts an `i64` to an `X690Element` by encoding it as an `INTEGER`
    #[inline]
    fn from(value: i64) -> Self {
        BER.encode_i64(value).unwrap()
    }
}

impl From<u8> for X690Element {
    /// Converts a `u8` to an `X690Element` by encoding it as an `INTEGER`
    #[inline]
    fn from(value: u8) -> Self {
        BER.encode_u8(value).unwrap()
    }
}

impl From<u16> for X690Element {
    /// Converts a `u16` to an `X690Element` by encoding it as an `INTEGER`
    #[inline]
    fn from(value: u16) -> Self {
        BER.encode_u16(value).unwrap()
    }
}

impl From<u32> for X690Element {
    /// Converts a `u32` to an `X690Element` by encoding it as an `INTEGER`
    #[inline]
    fn from(value: u32) -> Self {
        BER.encode_u32(value).unwrap()
    }
}

impl From<u64> for X690Element {
    /// Converts a `u64` to an ``X690Element`` by encoding it as an `INTEGER`
    #[inline]
    fn from(value: u64) -> Self {
        BER.encode_u64(value).unwrap()
    }
}

impl From<OBJECT_IDENTIFIER> for X690Element {
    /// Converts an `OBJECT_IDENTIFIER` to an `X690Element` by encoding it
    #[inline]
    fn from(value: OBJECT_IDENTIFIER) -> Self {
        X690Element::from(&value)
    }
}

impl From<&OBJECT_IDENTIFIER> for X690Element {
    /// Converts a reference to an `OBJECT_IDENTIFIER` to an `X690Element` by encoding it
    #[inline]
    fn from(value: &OBJECT_IDENTIFIER) -> Self {
        BER.encode_object_identifier(value).unwrap()
    }
}

impl From<bool> for X690Element {
    /// Converts a bool to an X690Element by encoding it as a BOOLEAN
    #[inline]
    fn from(value: bool) -> Self {
        BER.encode_boolean(&value).unwrap()
    }
}

impl From<DATE> for X690Element {
    /// Converts a `DATE` to an `X690Element` by encoding it
    #[inline]
    fn from(value: DATE) -> Self {
        BER.encode_date(&value).unwrap()
    }
}

impl From<TIME_OF_DAY> for X690Element {
    /// Converts a `TIME_OF_DAY` to an `X690Element` by encoding it
    #[inline]
    fn from(value: TIME_OF_DAY) -> Self {
        BER.encode_time_of_day(&value).unwrap()
    }
}

impl From<DATE_TIME> for X690Element {
    /// Converts a `DATE_TIME` to an `X690Element` by encoding it
    #[inline]
    fn from(value: DATE_TIME) -> Self {
        BER.encode_date_time(&value).unwrap()
    }
}

impl From<TIME> for X690Element {
    /// Converts a `TIME` to an `X690Element` by encoding it
    #[inline]
    fn from(value: TIME) -> Self {
        BER.encode_time(&value).unwrap()
    }
}

impl From<DURATION> for X690Element {
    /// Converts a `DURATION` to an `X690Element` by encoding it
    #[inline]
    fn from(value: DURATION) -> Self {
        BER.encode_duration(&value).unwrap()
    }
}

impl TryInto<i8> for X690Element {
    type Error = ASN1Error;
    /// Attempts to decode an X690Element as an `i8` `INTEGER`
    #[inline]
    fn try_into(self) -> ASN1Result<i8> {
        BER.decode_i8(&self)
    }
}

impl TryInto<i16> for X690Element {
    type Error = ASN1Error;
    /// Attempts to decode an X690Element as an `i16` `INTEGER`
    #[inline]
    fn try_into(self) -> ASN1Result<i16> {
        BER.decode_i16(&self)
    }
}

impl TryInto<i32> for X690Element {
    type Error = ASN1Error;
    /// Attempts to decode an X690Element as an `i32` `INTEGER`
    #[inline]
    fn try_into(self) -> ASN1Result<i32> {
        BER.decode_i32(&self)
    }
}

impl TryInto<i64> for X690Element {
    type Error = ASN1Error;
    /// Attempts to decode an X690Element as an `i64` `INTEGER`
    #[inline]
    fn try_into(self) -> ASN1Result<i64> {
        BER.decode_i64(&self)
    }
}

impl TryInto<i128> for X690Element {
    type Error = ASN1Error;
    /// Attempts to decode an X690Element as an `i128` `INTEGER`
    #[inline]
    fn try_into(self) -> ASN1Result<i128> {
        BER.decode_i128(&self)
    }
}

impl TryInto<u8> for X690Element {
    type Error = ASN1Error;
    /// Attempts to decode an X690Element as a `u8` `INTEGER`
    #[inline]
    fn try_into(self) -> ASN1Result<u8> {
        BER.decode_u8(&self)
    }
}

impl TryInto<u16> for X690Element {
    type Error = ASN1Error;
    /// Attempts to decode an X690Element as a `u16` `INTEGER`
    #[inline]
    fn try_into(self) -> ASN1Result<u16> {
        BER.decode_u16(&self)
    }
}

impl TryInto<u32> for X690Element {
    type Error = ASN1Error;
    /// Attempts to decode an X690Element as a `u32` `INTEGER`
    #[inline]
    fn try_into(self) -> ASN1Result<u32> {
        BER.decode_u32(&self)
    }
}

impl TryInto<u64> for X690Element {
    type Error = ASN1Error;
    /// Attempts to decode an X690Element as a `u64` `INTEGER`
    #[inline]
    fn try_into(self) -> ASN1Result<u64> {
        BER.decode_u64(&self)
    }
}

impl TryInto<u128> for X690Element {
    type Error = ASN1Error;
    /// Attempts to decode an X690Element as a `u128` `INTEGER`
    #[inline]
    fn try_into(self) -> ASN1Result<u128> {
        BER.decode_u128(&self)
    }
}

impl TryInto<BOOLEAN> for X690Element {
    type Error = ASN1Error;
    /// Attempts to decode an X690Element as a `BOOLEAN`
    #[inline]
    fn try_into(self) -> ASN1Result<BOOLEAN> {
        BER.decode_boolean(&self)
    }
}

impl PartialEq for X690Element {
    /// Compares two X690Elements for equality
    ///
    /// For serialized values, this decodes them first before comparison.
    /// Primitive values are compared by their raw bytes.
    /// Constructed values are compared by their child elements.
    fn eq(&self, other: &Self) -> bool {
        // Helper to decode if serialized, else return reference to self
        fn as_decoded<'a>(el: &'a X690Element) -> Cow<'a, X690Element> {
            match &el.value {
                X690Value::Serialized(bytes) => {
                    match BER.decode_from_slice(bytes) {
                        Ok((_, decoded)) => Cow::Owned(decoded),
                        Err(_) => Cow::Borrowed(el), // fallback: treat as not equal
                    }
                }
                _ => Cow::Borrowed(el),
            }
        }

        let left = as_decoded(self);
        let right = as_decoded(other);

        match (&left.value, &right.value) {
            (X690Value::Primitive(a), X690Value::Primitive(b)) => a == b,
            (X690Value::Constructed(a), X690Value::Constructed(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                a.iter().zip(b.iter()).all(|(x, y)| x == y)
            }
            (X690Value::Primitive(_), _) | (X690Value::Constructed(_), _) | (_, X690Value::Primitive(_)) | (_, X690Value::Constructed(_)) => false,
            // Should not reach here, as all Serialized are decoded above
            _ => false,
        }
    }
}

impl Eq for X690Element {}

/// Decodes an X.690 tag from a byte slice
///
/// Returns a tuple containing:
/// - The number of bytes read
/// - The decoded tag
/// - Whether the tag is constructed
///
/// This function handles both short and long tag formats as specified in X.690.
pub fn x690_decode_tag(bytes: ByteSlice) -> ASN1Result<(usize, Tag, bool)> {
    if bytes.len() == 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::tlv_truncated));
    }
    let mut bytes_read = 1;
    let tag_class = match (bytes[0] & 0b1100_0000) >> 6 {
        0 => TagClass::UNIVERSAL,
        1 => TagClass::APPLICATION,
        2 => TagClass::CONTEXT,
        3 => TagClass::PRIVATE,
        _ => panic!("Impossible tag class"),
    };
    let constructed = (bytes[0] & 0b0010_0000) > 0;
    let mut tag_number: TagNumber = 0;

    if (bytes[0] & 0b00011111) == 0b00011111 {
        // If it is a long tag...
        for byte in bytes[1..].iter() {
            let final_byte: bool = ((*byte) & 0b1000_0000) == 0;
            if (tag_number > 0) && !final_byte {
                // tag_number > 0 means we've already processed one byte.
                // Tag encoded on more than 14 bits / two bytes.
                return Err(ASN1Error::new(ASN1ErrorCode::tag_too_big));
            }
            let seven_bits = ((*byte) & 0b0111_1111) as u16;
            if !final_byte && (seven_bits == 0) {
                // You cannot encode a long tag with padding bytes.
                return Err(ASN1Error::new(ASN1ErrorCode::padding_in_tag_number));
            }
            tag_number <<= 7;
            tag_number += seven_bits;
            bytes_read += 1;
            if final_byte {
                break;
            }
        }
        if tag_number <= 30 {
            // This could have been encoded in short form.
            return Err(ASN1Error::new(ASN1ErrorCode::tag_number_could_have_used_short_form));
        }
    } else {
        tag_number = (bytes[0] & 0b00011111) as TagNumber;
    }

    let tag = Tag::new(tag_class, tag_number);
    Ok((bytes_read, tag, constructed))
}

/// Calculates the total length of tag and length bytes in an X.690 encoding
///
/// This function examines the first few bytes to determine how many bytes
/// are used for the tag and length encoding, without actually decoding the values.
pub fn get_x690_tag_and_length_length(bytes: ByteSlice) -> usize {
    if bytes.len() == 0 {
        return 0;
    }
    let mut len: usize = 1;
    if (bytes[0] & 0b00011111) == 0b00011111 {
        // If it is a long tag...
        for byte in bytes[1..].iter() {
            len += 1; // Even the byte without the continuation flag set should be counted.
            if ((*byte) & 0b1000_0000) == 0 {
                break;
            }
        }
    }
    if len >= bytes.len() {
        return len;
    }
    let length_byte_0 = bytes[len - 1];
    len += 1;
    if (length_byte_0 & 0b1000_0000) == 0 {
        // Short definite form or indefinite form.
        return len;
    }
    (length_byte_0 & 0b0111_1111) as usize
}

/// Calculates the number of bytes needed to encode a number in base-128 format
///
/// Base-128 encoding is used for long tag numbers and other variable-length `INTEGER`s
/// in X.690 encoding.
const fn base_128_len(num: u32) -> usize {
    if likely(num < 128) {
        return 1;
    }
    let mut l = 0;
    let mut i = num;
    while i > 0 {
        l += 1;
        i >>= 7;
    }
    return l;
}

/// Writes a number in base-128 format to a writer
///
/// Base-128 encoding uses 7 bits per byte with a continuation bit in the high bit.
/// This is used for encoding long tag numbers and other variable-length `INTEGER`s.
///
/// Returns the number of bytes written.
fn write_base_128<W>(output: &mut W, mut num: u32) -> Result<usize>
where
    W: Write,
{
    #[cfg(feature = "likely_stable")]
    if likely(num < 128) {
        return output.write(&[num as u8]);
    }

    // A u32 can take up to 5 bytes.
    let mut encoded: [u8; 5] = [0; 5];
    let mut byte_count: usize = 0;
    while num > 0b0111_1111 {
        encoded[byte_count] = (num & 0b0111_1111) as u8 | 0b1000_0000;
        byte_count += 1;
        num >>= 7;
    }
    encoded[byte_count] = num as u8;
    output.write(&encoded[0..byte_count+1])
}

/// Calculates the number of bytes needed to encode a tag number
///
/// Tag numbers less than 31 use the short form (1 byte).
/// Tag numbers 31 and above use the long form with base-128 encoding.
pub const fn get_written_x690_tag_length(tagnum: TagNumber) -> usize {
    if tagnum < 31 {
        // See ITU Rec. X.690 (2021), Section 8.1.2.4.
        return 1;
    }
    base_128_len(tagnum as u32) + 1
}

/// Calculates the number of bytes needed to encode a length value
///
/// Lengths 0-127 use the short form (1 byte).
/// Longer lengths use the long form with a length indicator byte followed by the length value.
pub const fn get_written_x690_length_length(len: usize) -> usize {
    if len <= 127 {
        // See ITU Rec. X.690 (2021), Section 8.1.3.3, "NOTE"
        return 1;
    }
    let octets_needed: usize = match len {
        0..=255 => 1,
        256..=65535 => 2,
        65536..=16777215 => 3,
        16777216..=4294967295 => 4,
        _ => return 5, // This is 4GB * 255. It's more than enough for anything.
    };
    octets_needed + 1
}

/// Writes an X.690 tag to a writer
///
/// This function handles both short and long tag formats as specified in X.690.
/// The tag includes the class, constructed bit, and tag number.
///
/// Returns the number of bytes written.
pub fn x690_write_tag<W>(
    output: &mut W,
    class: TagClass,
    constructed: bool,
    tagnum: TagNumber,
) -> Result<usize>
where
    W: Write,
{
    let k: u8 = match class {
        TagClass::UNIVERSAL => X690_TAG_CLASS_UNIVERSAL,
        TagClass::APPLICATION => X690_TAG_CLASS_APPLICATION,
        TagClass::CONTEXT => X690_TAG_CLASS_CONTEXT,
        TagClass::PRIVATE => X690_TAG_CLASS_PRIVATE,
    };
    if tagnum < 31 {
        // See ITU Rec. X.690 (2021), Section 8.1.2.4.
        return output.write(&[k
            | if constructed {
                0b0010_0000
            } else {
                0b0000_0000
            }
            | tagnum as u8]);
    } else {
        let first_byte_result = output.write(&[k
            | if constructed {
                0b0010_0000
            } else {
                0b0000_0000
            }
            | 0b0001_1111u8]);
        if let Err(e) = first_byte_result {
            return Err(e);
        }
        return write_base_128(output, tagnum.into());
    }
}

/// Writes an X.690 length to a writer
///
/// This function handles both short and long length formats as specified in X.690.
/// Lengths 0-127 use the short form, longer lengths use the long form.
///
/// Returns the number of bytes written.
pub fn x690_write_length<W>(output: &mut W, length: usize) -> Result<usize>
where
    W: Write,
{
    if length <= 127 {
        // See ITU Rec. X.690 (2021), Section 8.1.3.3, "NOTE"
        return output.write(&[length as u8]);
    } else {
        // Calculate num of octets needed.
        // write 0b1000_0000 | octets needed
        let octets_needed: u8 = match length {
            0..=255 => 1,
            256..=65535 => 2,
            65536..=16777215 => 3,
            16777216..=4294967295 => 4,
            _ => return Err(Error::from(ErrorKind::Unsupported)),
        };
        let length_bytes = length.to_be_bytes();
        output.write(&[0b1000_0000 | octets_needed])?;
        output.write(&length_bytes[std::mem::size_of::<usize>()-octets_needed as usize..])
            .map(|n| n + 1)
    }
}

/// Writes a `BOOLEAN` value in X.690 format
///
/// `BOOLEAN` values are encoded as a single octet: 0xFF for true, 0x00 for false.
///
/// Returns the number of bytes written.
#[inline]
pub fn x690_write_boolean_value<W>(output: &mut W, value: &BOOLEAN) -> Result<usize>
where
    W: Write,
{
    if *value {
        return output.write(&[0xFF]);
    } else {
        return output.write(&[0x00]);
    }
}

/// Writes an `INTEGER` value in X.690 format
///
/// `INTEGER` values are written as raw bytes in big-endian format.
///
/// Returns the number of bytes written.
#[inline]
pub fn x690_write_integer_value<W>(output: &mut W, value: &INTEGER) -> Result<usize>
where
    W: Write,
{
    if value.len() == 0 {
        return Err(std::io::Error::from(ErrorKind::InvalidData));
    }
    if value.len() == 1 {
        return output.write(value);
    }
    if value[0] == 0x00 && (value[1] & 0b1000_0000) == 0 {
        return Err(std::io::Error::from(ErrorKind::InvalidData));
    }
    if value[0] == 0xFF && (value[1] & 0b1000_0000) > 0 {
        return Err(std::io::Error::from(ErrorKind::InvalidData));
    }
    output.write(value)
}

/// Writes an i64 value in X.690 `INTEGER` format
///
/// This function handles the encoding of i64 values as `INTEGER` types,
/// including proper handling of sign extension and padding.
///
/// Returns the number of bytes written.
pub fn x690_write_i64_value<W>(output: &mut W, value: i64) -> Result<usize>
where
    W: Write,
{
    let bytes: [u8; 8] = value.to_be_bytes();
    let padding_byte: u8 = if value >= 0 { 0x00 } else { 0xFF };
    let mut number_of_padding_bytes: usize = 0;
    for byte in bytes {
        if byte == padding_byte {
            number_of_padding_bytes += 1;
        } else {
            break;
        }
    }
    let mut bytes_written: usize = 0;
    if (number_of_padding_bytes == size_of::<i64>())
        || (value >= 0 && ((bytes[number_of_padding_bytes] & 0b1000_0000) > 0))
        || (value < 0 && ((bytes[number_of_padding_bytes] & 0b1000_0000) == 0)) {
        bytes_written += output.write(&[padding_byte])?;
    }
    bytes_written += output.write(&(bytes[number_of_padding_bytes..size_of::<i64>()]))?;
    Ok(bytes_written)
}

/// Writes an `ENUMERATED` value in X.690 format
///
/// `ENUMERATED` values are encoded the same as `INTEGER` values.
///
/// Returns the number of bytes written.
#[inline]
pub fn x690_write_enum_value<W>(output: &mut W, value: &ENUMERATED) -> Result<usize>
where
    W: Write,
{
    x690_write_i64_value(output, *value)
}

/// Writes a `BIT STRING` value in X.690 format
///
/// `BIT STRING` values include a trailing bits count byte followed by the actual bits.
///
/// Returns the number of bytes written.
pub fn x690_write_bit_string_value<W>(output: &mut W, value: &BIT_STRING) -> Result<usize>
where
    W: Write,
{
    let trailing_bits = value.get_trailing_bits_count();
    output.write(&[trailing_bits])?;
    if trailing_bits == 0 {
        let bytes_written = output.write(value.get_bytes_ref())?;
        return Ok(bytes_written + 1);
    }
    // Otherwise, we check if the trailing bits are set and fix that.
    let maybe_last_byte = value.get_bytes_ref().last();
    let der_violated;
    let bytes = value.get_bytes_ref();
    let correct_last_byte: u8;
    if let Some(last_byte) = maybe_last_byte {
        let trailing_bits_mask = !(0xFFu8 << trailing_bits);
        der_violated = (last_byte & trailing_bits_mask) > 0;
        correct_last_byte = last_byte & (0xFFu8 << trailing_bits);
    } else {
        return Err(std::io::Error::from(ErrorKind::InvalidData));
    }

    // No violation? Just write the whole thing.
    if likely(!der_violated) {
        let bytes_written = output.write(value.get_bytes_ref())?;
        return Ok(bytes_written + 1);
    }

    debug_assert!(maybe_last_byte.is_some());
    let mut bytes_written = output.write(&bytes[..bytes.len() - 1])?;
    bytes_written += output.write(&[ correct_last_byte ])?;
    Ok(bytes_written + 1)
}

/// Writes an `OCTET STRING` value in X.690 format
///
/// `OCTET STRING` values are written as raw bytes.
///
/// Returns the number of bytes written.
#[inline]
pub fn x690_write_octet_string_value<W>(output: &mut W, value: &OCTET_STRING) -> Result<usize>
where
    W: Write,
{
    output.write(value)
}

/// Writes an `OBJECT IDENTIFIER` value in X.690 format
///
/// `OBJECT IDENTIFIER` values are encoded using the X.690 encoding format.
///
/// Returns the number of bytes written.
#[inline]
pub fn x690_write_object_identifier_value<W>(
    output: &mut W,
    value: &OBJECT_IDENTIFIER,
) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_x690_slice())
}

/// Writes an `ObjectDescriptor` value in X.690 format
///
/// `ObjectDescriptor` values are written as UTF-8 encoded strings.
///
/// Returns the number of bytes written.
#[inline]
pub fn x690_write_object_descriptor_value<W>(
    output: &mut W,
    value: &str,
) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_bytes())
}

/// Encode the components of an `EXTERNAL` value as X.690-encoded elements
///
/// This function encodes the components of an `EXTERNAL` value as X.690-encoded elements.
/// It handles the encoding of the identification, data value descriptor, and data value.
///
/// # Arguments
/// * `value` - The `EXTERNAL` value to encode
///
/// # Returns
/// A vector of X.690-encoded elements representing the components of the `EXTERNAL` value
///
/// To be a complete `EXTERNAL` value, these MUST be contained within a "parent" element
/// having tag `[UNIVERSAL 8]` and it MUST be constructed.
pub fn x690_encode_external_components (value: &EXTERNAL) -> Result<Vec<X690Element>> {
    let mut inner_elements: Vec<X690Element> = Vec::with_capacity(4);
    match &value.identification {
        ExternalIdentification::syntax(oid) => {
            let mut bytes = BytesMut::new().writer();
            x690_write_object_identifier_value(&mut bytes, &oid)?;
            let element = X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OBJECT_IDENTIFIER),
                X690Value::Primitive(bytes.into_inner().into()),
            );
            inner_elements.push(element);
        }
        ExternalIdentification::presentation_context_id(pci) => {
            let mut bytes = BytesMut::new().writer();
            x690_write_integer_value(&mut bytes, pci)?;
            let element = X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
                X690Value::Primitive(bytes.into_inner().into()),
            );
            inner_elements.push(element);
        }
        ExternalIdentification::context_negotiation(cn) => {
            let mut direct_ref_bytes = BytesMut::new().writer();
            x690_write_object_identifier_value(&mut direct_ref_bytes, &cn.transfer_syntax)?;
            let direct_ref_element = X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OBJECT_IDENTIFIER),
                X690Value::Primitive(direct_ref_bytes.into_inner().into()),
            );
            inner_elements.push(direct_ref_element);
            let mut indirect_ref_bytes = BytesMut::new().writer();
            x690_write_integer_value(&mut indirect_ref_bytes, &cn.presentation_context_id)?;
            let indirect_ref_element = X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
                X690Value::Primitive(indirect_ref_bytes.into_inner().into()),
            );
            inner_elements.push(indirect_ref_element);
        }
    };
    match &value.data_value_descriptor {
        Some(dvd) => {
            let mut bytes = BytesMut::new().writer();
            x690_write_object_descriptor_value(&mut bytes, &dvd)?;
            let element = X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OBJECT_DESCRIPTOR),
                X690Value::Primitive(bytes.into_inner().into()),
            );
            inner_elements.push(element);
        }
        None => (),
    };
    let mut data_value_bytes = BytesMut::new().writer();
    match &value.data_value {
        ExternalEncoding::single_ASN1_type(t) => {
            let el = BER.encode_any(t)?;
            x690_write_tlv(&mut data_value_bytes, &el)?
        },
        ExternalEncoding::octet_aligned(o) => x690_write_octet_string_value(&mut data_value_bytes, o)?,
        ExternalEncoding::arbitrary(b) => x690_write_bit_string_value(&mut data_value_bytes, b)?,
    };
    let data_value_element = X690Element::new(
        Tag::new(TagClass::CONTEXT, 1),
        X690Value::Primitive(data_value_bytes.into_inner().into()),
    );
    inner_elements.push(data_value_element);
    Ok(inner_elements)
}

/// Write an `EXTERNAL` value as an X.690-encoded element
///
/// This function writes an `EXTERNAL` value as an X.690-encoded element.
/// It handles the encoding of the identification, data value descriptor,
/// and data value.
///
/// # Arguments
/// * `output` - The writable stream to write the `EXTERNAL` value to
///
/// # Returns
/// The number of bytes written to the writable stream
///
/// NOTE: This has to be encoded in a strange way that is detailed in ITU-T
/// Recommendation X.690, Section 8.18.
pub fn x690_write_external_value<W>(output: &mut W, value: &EXTERNAL) -> Result<usize>
where
    W: Write,
{
    let components = x690_encode_external_components(value)?;
    let mut bytes_written: usize = 0;
    for component in components {
        bytes_written += x690_write_tlv(output, &component)?;
    }
    Ok(bytes_written)
}

/// Write a `REAL` value as an X.690-encoded element
///
/// This function writes a `REAL` value as an X.690-encoded element.
/// It handles the encoding of the value according to ITU Recommendation
/// X.690, Section 8.5.
///
/// This adheres to the Basic Encoding Rules (BER) encoding of `REAL` values,
/// but not necessarily the Distinguished Encoding Rules (DER) encoding or
/// the Canonical Encoding Rules (CER).
///
/// # Arguments
/// * `output` - The writable stream to write the `REAL` value to
///
/// # Returns
/// The number of bytes written to the writable stream
///
pub fn x690_write_real_value<W>(output: &mut W, value: &REAL) -> Result<usize>
where
    W: Write,
{
    // This may seem like a floating precision problem, but this is how the
    // `num` crate does it:
    // https://github.com/rust-num/num-traits/blob/5397a1c27124af874e42d3d185f78d8ce01ecf69/src/identities.rs#L61
    let is_zero = *value == 0.0;
    // If the real value is the value plus zero, there shall be no contents octets in the encoding.
    if is_zero {
        return Ok(0);
    }
    // If the real value is the value minus zero, then it shall be encoded as specified in 8.5.9.
    if is_zero && value.is_sign_negative() {
        return output.write(&[X690_REAL_SPECIAL | X690_SPECIAL_REAL_MINUS_ZERO]);
    }

    if value.is_nan() {
        return output.write(&[X690_REAL_SPECIAL | X690_SPECIAL_REAL_NOT_A_NUMBER]);
    }

    if value.is_infinite() {
        if value.is_sign_negative() {
            return output.write(&[X690_REAL_SPECIAL | X690_SPECIAL_REAL_MINUS_INFINITY]);
        } else {
            return output.write(&[X690_REAL_SPECIAL | X690_SPECIAL_REAL_PLUS_INFINITY]);
        }
    }

    let sign_bit: u8 = if value.is_sign_negative() {
        X690_REAL_NEGATIVE
    } else {
        X690_REAL_POSITIVE
    };
    let base_bits: u8 = X690_REAL_BASE_2;
    let scaling_factor: u8 = 0;
    let bits = value.to_bits();
    let mantissa_mask = (1u64 << 52) - 1;
    let mantissa: u64 = bits & mantissa_mask;
    let biased_exp = ((bits >> 52) & 0x7FF) as u16;

    // For normal numbers, add the implicit leading 1
    let mut mantissa = if biased_exp != 0 { mantissa | (1u64 << 52) } else { mantissa };
    let mut exponent = if biased_exp != 0 { biased_exp as i16 - 1023 - 52 } else { -1023 - 51 };

    // Normalize - remove trailing zeros
    while mantissa > 0 && mantissa & 1 == 0 {
        mantissa >>= 1;
        exponent += 1;
    }

    let e_bytes = exponent.to_be_bytes();
    let mut bytes_written: usize = 0;
    if exponent > u8::MAX as i16 {
        let byte0: u8 = X690_REAL_BINARY
            | sign_bit
            | base_bits
            | scaling_factor
            | X690_REAL_EXPONENT_FORMAT_2_OCTET;
        bytes_written += output.write(&[byte0, e_bytes[0], e_bytes[1]])?;
    } else {
        let byte0: u8 = X690_REAL_BINARY
            | sign_bit
            | base_bits
            | scaling_factor
            | X690_REAL_EXPONENT_FORMAT_1_OCTET;
        bytes_written += output.write(&[byte0, e_bytes[1]])?;
    };

    return match x690_write_i64_value(output, mantissa as i64) {
        Err(e) => return Err(e),
        Ok(wrote) => Ok(wrote + bytes_written),
    };
}

/// Encode the `identification` field of a context-switching type
///
/// This function encodes the `identification` field of a context-switching type,
/// such as an `EXTERNAL` or `EMBEDDED PDV`, as an X.690-encoded element.
///
/// Returns the X.690-encoded element.
///
/// # Arguments
/// * `id` - The `PresentationContextSwitchingTypeIdentification` value to convert
///
/// # Returns
/// The X.690-encoded element.
///
pub fn x690_encode_context_switching_identification(
    id: &PresentationContextSwitchingTypeIdentification,
) -> Result<X690Element> {
    match id {
        PresentationContextSwitchingTypeIdentification::syntaxes(syntaxes) => {
            let mut abstract_value_bytes = BytesMut::new().writer();
            let mut transfer_value_bytes = BytesMut::new().writer();
            x690_write_object_identifier_value(
                &mut abstract_value_bytes,
                &syntaxes.r#abstract,
            )?;
            x690_write_object_identifier_value(&mut transfer_value_bytes, &syntaxes.transfer)?;
            let mut syntaxes_elements: Vec<X690Element> = Vec::with_capacity(2);
            syntaxes_elements.push(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::Primitive(abstract_value_bytes.into_inner().into()),
            ));
            syntaxes_elements.push(X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::Primitive(transfer_value_bytes.into_inner().into()),
            ));
            let element = X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::Constructed(Arc::new(syntaxes_elements)),
            );
            return Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::Constructed(Arc::new(Vec::from([ element ]))),
            ));
        }
        PresentationContextSwitchingTypeIdentification::syntax(oid) => {
            // We assume that, on average, each OID arc is encoded on two bytes.
            let mut bytes = BytesMut::with_capacity(oid.as_x690_slice().len()).writer();
            x690_write_object_identifier_value(&mut bytes, &oid)?;
            let element = X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::Primitive(bytes.into_inner().into()),
            );
            return Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::Constructed(Arc::new(Vec::from([ element ]))),
            ));
        }
        PresentationContextSwitchingTypeIdentification::presentation_context_id(pci) => {
            let mut bytes = BytesMut::with_capacity(pci.len()).writer();
            x690_write_integer_value(&mut bytes, pci)?;
            let element = X690Element::new(
                Tag::new(TagClass::CONTEXT, 2),
                X690Value::Primitive(bytes.into_inner().into()),
            );
            return Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::Constructed(Arc::new(Vec::from([ element ]))),
            ));
        }
        PresentationContextSwitchingTypeIdentification::context_negotiation(cn) => {
            let mut pci_bytes = BytesMut::new().writer();
            x690_write_integer_value(&mut pci_bytes, &cn.presentation_context_id)?;
            let pci_element = X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::Primitive(pci_bytes.into_inner().into()),
            );
            let mut transfer_syntax_bytes = BytesMut::new().writer();
            x690_write_object_identifier_value(
                &mut transfer_syntax_bytes,
                &cn.transfer_syntax,
            )?;
            let transfer_syntax_element = X690Element::new(
                Tag::new(TagClass::CONTEXT, 1),
                X690Value::Primitive(transfer_syntax_bytes.into_inner().into()),
            );
            let cn_elements: Vec<X690Element> = vec![pci_element, transfer_syntax_element];
            let element = X690Element::new(
                Tag::new(TagClass::CONTEXT, 3),
                X690Value::Constructed(Arc::new(cn_elements)),
            );
            return Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::Constructed(Arc::new(Vec::from([ element ]))),
            ));
        }
        PresentationContextSwitchingTypeIdentification::transfer_syntax(ts) => {
            let mut bytes = BytesMut::new().writer();
            x690_write_object_identifier_value(&mut bytes, &ts)?;
            let element = X690Element::new(
                Tag::new(TagClass::CONTEXT, 4),
                X690Value::Primitive(bytes.into_inner().into()),
            );
            return Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::Constructed(Arc::new(Vec::from([ element ]))),
            ));
        }
        PresentationContextSwitchingTypeIdentification::fixed => {
            let element = X690Element::new(
                Tag::new(TagClass::CONTEXT, 5),
                X690Value::Primitive(Bytes::new()),
            );
            return Ok(X690Element::new(
                Tag::new(TagClass::CONTEXT, 0),
                X690Value::Constructed(Arc::new(Vec::from([ element ]))),
            ));
        }
    }
}

/// Encode the components of an `EMBEDDED PDV` value as X.690-encoded elements
///
/// This function encodes the components of an `EMBEDDED PDV` value as X.690-encoded elements.
/// It handles the encoding of the `identification` and `data-value`.
///
/// # Arguments
/// * `value` - The `EMBEDDED PDV` value to encode
///
/// To be a complete `EMBEDDED PDV` value, these MUST be contained within a "parent" element
/// having tag `[UNIVERSAL 11]` and it MUST be constructed.
pub fn x690_encode_embedded_pdv_components (value: &EmbeddedPDV) -> Result<Vec<X690Element>> {
    let id = x690_encode_context_switching_identification(&value.identification)?;
    let mut data_value_bytes = BytesMut::new().writer();
    x690_write_octet_string_value(&mut data_value_bytes, &value.data_value)?;
    let data_value_element = X690Element::new(
        Tag::new(TagClass::CONTEXT, 1),
        X690Value::Primitive(data_value_bytes.into_inner().into()),
    );
    Ok(vec![id, data_value_element])
}

/// Write an `EMBEDDED PDV` value as an X.690-encoded element
///
/// This function writes an `EMBEDDED PDV` value as an X.690-encoded element.
/// It handles the encoding of the `identification`, data value descriptor,
/// and `data-value`.
///
/// # Arguments
/// * `output` - The writable stream to write the `EMBEDDED PDV` value to
/// * `value` - The `EMBEDDED PDV` value to write
///
/// # Returns
/// The number of bytes written to the writable stream
///
pub fn x690_write_embedded_pdv_value<W>(output: &mut W, value: &EmbeddedPDV) -> Result<usize>
where
    W: Write,
{
    let components: Vec<X690Element> = x690_encode_embedded_pdv_components(value)?;
    let mut bytes_written: usize = 0;
    for component in components {
        bytes_written += x690_write_tlv(output, &component)?;
    }
    Ok(bytes_written)
}

/// Write a `UTF8String` value as an X.690-encoded element, returning the number of bytes written
#[inline]
pub fn x690_write_utf8_string_value<W>(output: &mut W, value: &str) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_bytes())
}

/// Write a `RELATIVE-OID` value as an X.690-encoded element, returning the number of bytes written
#[inline]
pub fn x690_write_relative_oid_value<W>(output: &mut W, value: &RELATIVE_OID) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_x690_slice())
}

/// Write a `TIME` value as an X.690-encoded element, returning the number of bytes written
#[inline]
pub fn x690_write_time_value<W>(output: &mut W, value: &TIME) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_bytes())
}

/// Write a `UTCTime` value as an X.690-encoded element, returning the number of bytes written
#[inline]
pub fn x690_write_utc_time_value<W>(output: &mut W, value: &UTCTime) -> Result<usize>
where
    W: Write,
{
    output.write(value.to_string().as_bytes())
}

/// Write a `GeneralizedTime` value as an X.690-encoded element, returning the number of bytes written
#[inline]
pub fn x690_write_generalized_time_value<W>(
    output: &mut W,
    value: &GeneralizedTime,
) -> Result<usize>
where
    W: Write,
{
    output.write(value.to_string().as_bytes())
}

/// Write a `UniversalString` value as an X.690-encoded element, returning the number of bytes written
#[inline]
pub fn x690_write_universal_string_value<W>(
    output: &mut W,
    value: &[u32],
) -> Result<usize>
where
    W: Write,
{
    for c in value {
        output.write(&c.to_be_bytes())?;
    }
    Ok(value.len() * 4)
}

/// Encode the components of a `CharacterString` value as X.690-encoded elements
///
/// This function encodes the components of a `CharacterString` value as X.690-encoded elements.
/// It handles the encoding of the `identification` and `string-value` fields.
///
/// # Arguments
/// * `value` - The `CharacterString` value to encode
///
/// # Returns
/// A vector of X.690-encoded elements representing the components of the `CharacterString` value
///
/// To be a complete `CharacterString` value, these MUST be contained within a "parent" element
/// having tag `[UNIVERSAL 29]` and it MUST be constructed.
pub fn x690_encode_character_string_components (value: &CharacterString) -> Result<Vec<X690Element>> {
    let id = x690_encode_context_switching_identification(&value.identification)?;
    let mut data_value_bytes = BytesMut::new().writer();
    x690_write_octet_string_value(&mut data_value_bytes, &value.string_value)?;
    let data_value_element = X690Element::new(
        Tag::new(TagClass::CONTEXT, 1),
        X690Value::Primitive(data_value_bytes.into_inner().into()),
    );
    Ok(vec![id, data_value_element])
}

/// Write a `CharacterString` value as an X.690-encoded element, returning the number of bytes written
///
/// This function writes a `CharacterString` value as an X.690-encoded element.
/// It handles the encoding of the `identification` and `string-value` fields.
///
/// # Arguments
/// * `output` - The writable stream to write the `CharacterString` value to
/// * `value` - The `CharacterString` value to write
///
/// # Returns
/// The number of bytes written to the writable stream
///
pub fn x690_write_character_string_value<W>(
    output: &mut W,
    value: &CharacterString,
) -> Result<usize>
where
    W: Write,
{
    let components: Vec<X690Element> = x690_encode_character_string_components(value)?;
    let mut bytes_written: usize = 0;
    for component in components {
        bytes_written += x690_write_tlv(output, &component)?;
    }
    Ok(bytes_written)
}

/// Write a `BMPString` value as an X.690-encoded element, returning the number of bytes written
pub fn x690_write_bmp_string_value<W>(output: &mut W, value: &[u16]) -> Result<usize>
where
    W: Write,
{
    for c in value {
        output.write(&c.to_be_bytes())?;
    }
    Ok(value.len() * 2)
}

/// Write a string value as an X.690-encoded element, returning the number of bytes written
#[inline]
pub fn x690_write_string_value<W>(output: &mut W, value: &str) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_bytes())
}

/// Write a `DATE` value as an X.690-encoded element, returning the number of bytes written
pub fn x690_write_date_value<W>(output: &mut W, value: &DATE) -> Result<usize>
where
    W: Write,
{
    if value.month > 12 || value.month == 0 || value.day > 31 || value.day == 0 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    output.write(value.to_num_string().as_bytes())
}

/// Write a `TIME-OF-DAY` value as an X.690-encoded element, returning the number of bytes written
pub fn x690_write_time_of_day_value<W>(output: &mut W, value: &TIME_OF_DAY) -> Result<usize>
where
    W: Write,
{
    if value.hour > 23 || value.minute > 59 || value.second > 59 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    output.write(value.to_num_string().as_bytes())
}

/// Write a `DATE-TIME` value as an X.690-encoded element, returning the number of bytes written
pub fn x690_write_date_time_value<W>(output: &mut W, value: &DATE_TIME) -> Result<usize>
where
    W: Write,
{
    output.write(value.to_num_string().as_bytes())
}

/// Write a `DURATION` value as an X.690-encoded element, returning the number of bytes written
pub fn x690_write_duration_value<W>(output: &mut W, value: &DURATION_EQUIVALENT) -> Result<usize>
where
    W: Write,
{
    output.write(&value.to_string().as_bytes()[1..]) // Skip the "P"
}

/// Write an X.690-encoded value to a writable stream, returning the number of bytes written
fn x690_write_value<W>(output: &mut W, encoding: &X690Value) -> Result<usize>
where
    W: Write,
{
    match encoding {
        X690Value::Primitive(v) => output.write(&v),
        X690Value::Constructed(components) => {
            let mut sum: usize = 0;
            for component in components.iter() {
                sum += x690_write_tlv(output, component)?;
            }
            Ok(sum)
        },
        X690Value::Serialized(v) => {
            let (_, el) = BER.decode_from_slice(&v)?;
            x690_write_value(output, &el.value)
        }
    }
}

/// Write an X.690-encoded element to a writable stream, returning the number of bytes written
pub fn x690_write_tlv<W>(output: &mut W, node: &X690Element) -> Result<usize>
where
    W: Write,
{
    if let X690Value::Serialized(serialized) = &node.value {
        return output.write(&serialized);
    }
    let mut bytes_written: usize = 0;
    bytes_written += x690_write_tag(output, node.tag.tag_class, node.is_constructed(), node.tag.tag_number)?;
    bytes_written += x690_write_length(output, node.value.len())?;
    bytes_written += x690_write_value(output, &node.value)?;
    Ok(bytes_written)
}

/// Deconstruct an X.690-encoded element that could be primitively-constructed
///
/// The X.690 encoding rules allow for some universal types to be either
/// primitively-constructed or constructed. However, for the purposes of
/// validation or decoding, we may want to "deconstruct" such constructed
/// values to a single primitive value.
///
/// One such example is the `GeneralizedTime`. While it may be constructed,
/// it might be difficult to implement parsing and validation when it is
/// split across multiple X.690 tag-length-value (TLV) elements.
///
/// If the element is already primitively constructed, this just returns a
/// reference to it, so no copying overhead is incurred.
pub fn deconstruct<'a>(el: &'a X690Element) -> ASN1Result<Cow<'a, [u8]>> {
    match &el.value {
        X690Value::Primitive(bytes) => Ok(Cow::Borrowed(bytes)),
        X690Value::Constructed(children) => {
            let mut deconstructed_value = BytesMut::new();
            for child in children.iter() {
                /* Just to be clear, this is 100% intentional. In ITU X.690, it says that the substrings of a string
                type are to have OCTET STRING tags and it even has examples where it confirms this visually. */
                if child.tag.tag_class != TagClass::UNIVERSAL
                    || child.tag.tag_number != UNIV_TAG_OCTET_STRING
                {
                    let mut err =
                        ASN1Error::new(ASN1ErrorCode::string_constructed_with_invalid_tagging);
                    err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
                    err.length = Some(el.len());
                    err.constructed = Some(true);
                    return Err(err);
                }
                let deconstructed_child = deconstruct(&child)?;
                deconstructed_value.put(deconstructed_child.as_ref());
            }
            Ok(Cow::Owned(Vec::<u8>::from(deconstructed_value)))
        },
        X690Value::Serialized(v) => {
            let (_, el) = BER.decode_from_slice(&v)?;
            Ok(Cow::Owned(deconstruct(&el)?.into_owned()))
        }
    }
}

/// Read a `BOOLEAN` value from an X.690-encoded element's content octets
pub const fn x690_read_boolean_value(value_bytes: ByteSlice) -> ASN1Result<BOOLEAN> {
    if value_bytes.len() != 1 {
        let err = ASN1Error::new(ASN1ErrorCode::x690_boolean_not_one_byte);
        return Err(err);
    }
    Ok(value_bytes[0] > 0)
}

/// Read an `INTEGER` value from an X.690-encoded element's content octets
#[inline]
pub fn x690_read_integer_value(value_bytes: ByteSlice) -> ASN1Result<INTEGER> {
    // Intentionally not validating this. Most integers are small and correct.
    // If they have padding, its obvious how to handle that.
    Ok(Vec::from(value_bytes))
}

/// Read an `i64` value from an X.690-encoded element's content octets
#[inline]
pub fn x690_read_i64_value(value_bytes: ByteSlice) -> ASN1Result<i64> {
    match read_i64(value_bytes) {
        Some(v) => Ok(v),
        None => Err(ASN1Error::new(ASN1ErrorCode::value_too_big)),
    }
}

/// Read an `ENUMERATED` value from an X.690-encoded element's content octets
#[inline]
pub fn x690_read_enum_value(value_bytes: ByteSlice) -> ASN1Result<ENUMERATED> {
    x690_read_i64_value(value_bytes)
}

/// Read an `OBJECT IDENTIFIER` value from an X.690-encoded element's content octets
#[inline]
pub fn x690_read_object_identifier_value(value_bytes: ByteSlice) -> ASN1Result<OBJECT_IDENTIFIER> {
    OBJECT_IDENTIFIER::from_x690_encoding_slice(value_bytes)
}

/// Read a `RELATIVE-OID` value from an X.690-encoded element's content octets
#[inline]
pub fn x690_read_relative_oid_value(value_bytes: ByteSlice) -> ASN1Result<RELATIVE_OID> {
    RELATIVE_OID::from_x690_encoding_slice(value_bytes)
}

/// Read a `DATE` value from an X.690-encoded element's content octets
#[inline]
pub fn x690_read_date_value(value_bytes: ByteSlice) -> ASN1Result<DATE> {
    DATE::try_from(value_bytes)
}

/// Read a `TIME-OF-DAY` value from an X.690-encoded element's content octets
#[inline]
pub fn x690_read_time_of_day_value(value_bytes: ByteSlice) -> ASN1Result<TIME_OF_DAY> {
    TIME_OF_DAY::try_from(value_bytes)
}

/// Read a `DATE-TIME` value from an X.690-encoded element's content octets
#[inline]
pub fn x690_read_date_time_value(value_bytes: ByteSlice) -> ASN1Result<DATE_TIME> {
    DATE_TIME::try_from(value_bytes)
}

/// Read a `DURATION` value from an X.690-encoded element's content octets
#[inline]
pub fn x690_read_duration_value(value_bytes: ByteSlice) -> ASN1Result<DURATION> {
    DURATION::try_from(value_bytes)
}

/// A trait for relating an X.690-encoded element to something
pub trait RelateTLV {

    /// Relate something to an X.690-encoded tag-length-value (TLV) element
    fn relate_tlv (&mut self, el: &X690Element);
}

impl RelateTLV for ASN1Error {
    fn relate_tlv (&mut self, el: &X690Element) {
        self.tag = Some(el.tag);
        self.constructed = Some(el.is_constructed());
        self.length = Some(el.len());
    }
}

/// The Root Component Type List (RCTL) #1 for the X.690-specific encoding of
/// an `EXTERNAL` value as described in ITU Recommendation X.690, Section 8.18.
///
/// For reference, the full ASN.1 for this is:
///
/// ```asn1
/// [UNIVERSAL 8] IMPLICIT SEQUENCE {
///     direct-reference        OBJECT IDENTIFIER OPTIONAL,
///     indirect-reference      INTEGER OPTIONAL,
///     data-value-descriptor   ObjectDescriptor OPTIONAL,
///     encoding CHOICE {
///         single-ASN1-type    [0] ABSTRACT-SYNTAX.&Type,
///         octet-aligned       [1] IMPLICIT OCTET STRING,
///         arbitrary           [2] IMPLICIT BIT STRING } }
/// ```
pub const _RCTL1_FOR_EXTERNAL: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "direct-reference",
        true,
        TagSelector::tag((
            TagClass::UNIVERSAL,
            UNIV_TAG_OBJECT_IDENTIFIER,
        )),
        None,
        None,
    ),
    ComponentSpec::new(
        "indirect-reference",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, UNIV_TAG_INTEGER)),
        None,
        None,
    ),
    ComponentSpec::new(
        "data-value-descriptor",
        true,
        TagSelector::tag((
            TagClass::UNIVERSAL,
            UNIV_TAG_OBJECT_DESCRIPTOR,
        )),
        None,
        None,
    ),
    ComponentSpec::new(
        "encoding",
        false,
        TagSelector::or(&[
            &TagSelector::tag((TagClass::CONTEXT, 0)),
            &TagSelector::tag((TagClass::CONTEXT, 1)),
            &TagSelector::tag((TagClass::CONTEXT, 2)),
        ]),
        None,
        None,
    ),
];

/// The Extended Attribute List (EAL) for the X.690-specific encoding of
/// an `EXTERNAL` value as described in ITU Recommendation X.690, Section 8.18.
/// It is empty, so this is basically just a formality.
pub const _EAL_FOR_EXTERNAL: &[ComponentSpec; 0] = &[];

/// The Root Component Type List (RCTL) #2 for the X.690-specific encoding of
/// an `EXTERNAL` value as described in ITU Recommendation X.690, Section 8.18.
/// It is empty, so this is basically just a formality.
pub const _RCTL2_FOR_EXTERNAL: &[ComponentSpec; 0] = &[];

#[cfg(test)]
mod tests {

    use super::*;
    use std::sync::Arc;
    use wildboar_asn1::{
        Tag,
        TagClass,
        UNIV_TAG_OCTET_STRING,
        UNIV_TAG_BOOLEAN,
        UNIV_TAG_SEQUENCE,
    };
    use bytes::Bytes;

    #[test]
    fn test_x690_write_boolean_value() {
        let mut output = BytesMut::new().writer();
        crate::x690_write_boolean_value(&mut output, &true).unwrap();
        crate::x690_write_boolean_value(&mut output, &false).unwrap();
        let output: Bytes = output.into_inner().into();
        assert_eq!(output.len(), 2);
        assert!(output.starts_with(&[0xFF, 0x00]));
    }

    #[test]
    fn test_x690_write_integer_value() {
        let mut output = BytesMut::new();
        let mut i = 0;
        for value in -128i8..127i8 {
            let mut out = output.writer();
            crate::x690_write_enum_value(&mut out, &i64::from(value)).unwrap();
            output = out.into_inner();
            assert_eq!(output[i] as i8, value);
            i += 1;
        }
        assert_eq!(output.len(), 255);
    }

    #[test]
    fn test_x690_write_octet_string_value() {
        let mut output = BytesMut::new().writer();
        let bytes: Vec<u8> = vec![1, 3, 5, 7, 9];
        crate::x690_write_octet_string_value(&mut output, &bytes).unwrap();
        let output: Bytes = output.into_inner().into();
        assert_eq!(output.len(), 5);
        assert!(output.starts_with(&[1, 3, 5, 7, 9]));
    }

    #[test]
    fn test_x690_write_object_identifier_value() {
        let mut output = BytesMut::new().writer();
        let oid = wildboar_asn1::OBJECT_IDENTIFIER::try_from(vec![2u32, 5, 4, 3]).unwrap();
        crate::x690_write_object_identifier_value(&mut output, &oid).unwrap();
        let output: Bytes = output.into_inner().into();
        assert_eq!(output.len(), 3);
        assert!(output.starts_with(&[0x55, 0x04, 0x03]));
    }

    #[test]
    fn test_x690_write_object_descriptor_value() {
        let mut output = BytesMut::new().writer();
        let value = String::from("commonName");
        crate::x690_write_object_descriptor_value(&mut output, &value).unwrap();
        let output: Bytes = output.into_inner().into();
        assert_eq!(output.len(), value.len());
        assert_eq!(
            String::from_utf8(output.into()).unwrap(),
            String::from("commonName")
        );
    }

    #[test]
    fn test_x690_write_real_value() {
        let output = BytesMut::new();
        let value = 1.2345;
        crate::x690_write_real_value(&mut output.writer(), &value).unwrap();
    }

    #[test]
    fn test_constructed_encoding() {
        let asn1_data = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
            crate::X690Value::Constructed(Arc::new(vec![
                X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_BOOLEAN),
                    crate::X690Value::Primitive(Bytes::copy_from_slice(&[ 0xFF ])),
                ),
                X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER),
                    crate::X690Value::Primitive(Bytes::copy_from_slice(&[ 0x01, 0x03 ])),
                ),
            ])),
        );
        let mut output = Vec::new();
        match x690_write_tlv(&mut output, &asn1_data) {
            Ok(bytes_written) => {
                assert_eq!(bytes_written, 9);
            }
            Err(e) => panic!("{}", e),
        }
        assert!(output.starts_with(&[
            X690_TAG_CLASS_UNIVERSAL
            | 0b0010_0000 // Constructed
            | UNIV_TAG_SEQUENCE as u8,
            0x07,
            0x01,
            0x01,
            0xFF,
            0x02,
            0x02,
            0x01,
            0x03,
        ]));
    }

    #[test]
    fn test_ber_decode_definite_short() {
        let encoded_data: Vec<u8> = vec![
            X690_TAG_CLASS_UNIVERSAL
            | 0b0010_0000 // Constructed
            | UNIV_TAG_SEQUENCE as u8,
            0x06,
            0x01,
            0x01,
            0xFF,
            0x02,
            0x01,
            0x7F,
        ];
        match BER.decode_from_slice(encoded_data.as_slice()) {
            Ok((bytes_read, el)) => {
                assert_eq!(bytes_read, 8);
                assert_eq!(el.tag.tag_class, TagClass::UNIVERSAL);
                assert_eq!(el.tag.tag_number, UNIV_TAG_SEQUENCE);
                if let X690Value::Constructed(children) = el.value {
                    assert_eq!(children.len(), 2);
                    assert_eq!(children[0].tag.tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[1].tag.tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[0].tag.tag_number, UNIV_TAG_BOOLEAN);
                    assert_eq!(children[1].tag.tag_number, UNIV_TAG_INTEGER);
                } else {
                    panic!("Decoded non-constructed.");
                }
            }
            Err(e) => panic!("{}", e),
        };
    }

    #[test]
    fn test_ber_decode_indefinite() {
        let encoded_data: Vec<u8> = vec![
            X690_TAG_CLASS_UNIVERSAL
            | 0b0010_0000 // Constructed
            | UNIV_TAG_SEQUENCE as u8,
            0x80, // Indefinite length
            0x01,
            0x01,
            0xFF,
            0x02,
            0x01,
            0x7F,
            0x00, // End of content
            0x00,
        ];
        match BER.decode_from_slice(encoded_data.as_slice()) {
            Ok((bytes_read, el)) => {
                assert_eq!(bytes_read, 10);
                assert_eq!(el.tag.tag_class, TagClass::UNIVERSAL);
                assert_eq!(el.tag.tag_number, UNIV_TAG_SEQUENCE);
                if let X690Value::Constructed(children) = el.value {
                    assert_eq!(children.len(), 2);
                    assert_eq!(children[0].tag.tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[1].tag.tag_class, TagClass::UNIVERSAL);
                    assert_eq!(children[0].tag.tag_number, UNIV_TAG_BOOLEAN);
                    assert_eq!(children[1].tag.tag_number, UNIV_TAG_INTEGER);
                } else {
                    panic!("Decoded non-constructed.");
                }
            }
            Err(e) => panic!("{}", e),
        };
    }

    #[test]
    fn test_deconstruct_primitive() {
        // Test deconstructing a primitive value
        let bytes = Bytes::copy_from_slice(&[0x01, 0x02, 0x03, 0x04]);
        let element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(bytes.clone()),
        );

        let result = deconstruct(&element).unwrap();
        assert_eq!(result.as_ref(), &[0x01, 0x02, 0x03, 0x04]);
        // Should be borrowed since it's a primitive
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_deconstruct_constructed_valid() {
        // Test deconstructing a constructed value with valid OCTET STRING children
        let child1 = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(&[0x01, 0x02])),
        );
        let child2 = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(&[0x03, 0x04])),
        );

        let element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Constructed(Arc::new(vec![child1, child2])),
        );

        let result = deconstruct(&element).unwrap();
        assert_eq!(result.as_ref(), &[0x01, 0x02, 0x03, 0x04]);
        // Should be owned since it's constructed
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn test_deconstruct_constructed_invalid_tag_class() {
        // Test deconstructing a constructed value with invalid tag class
        let child = X690Element::new(
            Tag::new(TagClass::APPLICATION, UNIV_TAG_OCTET_STRING), // Wrong tag class
            X690Value::Primitive(Bytes::copy_from_slice(&[0x01, 0x02])),
        );

        let element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Constructed(Arc::new(vec![child])),
        );

        let result = deconstruct(&element);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error_code, ASN1ErrorCode::string_constructed_with_invalid_tagging);
    }

    #[test]
    fn test_deconstruct_constructed_invalid_tag_number() {
        // Test deconstructing a constructed value with invalid tag number
        let child = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER), // Wrong tag number
            X690Value::Primitive(Bytes::copy_from_slice(&[0x01, 0x02])),
        );

        let element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Constructed(Arc::new(vec![child])),
        );

        let result = deconstruct(&element);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error_code, ASN1ErrorCode::string_constructed_with_invalid_tagging);
    }

    #[test]
    fn test_deconstruct_constructed_nested() {
        // Test deconstructing a constructed value with nested constructed children
        let grandchild1 = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(&[0x01, 0x02])),
        );
        let grandchild2 = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(&[0x03, 0x04])),
        );

        let child = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Constructed(Arc::new(vec![grandchild1, grandchild2])),
        );

        let element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Constructed(Arc::new(vec![child])),
        );

        let result = deconstruct(&element).unwrap();
        assert_eq!(result.as_ref(), &[0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_deconstruct_constructed_empty() {
        // Test deconstructing a constructed value with no children
        let element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Constructed(Arc::new(vec![])),
        );

        let result = deconstruct(&element).unwrap();
        let empty: [u8; 0] = [];
        assert_eq!(result.as_ref(), &empty);
    }

    #[test]
    fn test_deconstruct_serialized() {
        // Test deconstructing a serialized value
        let inner_bytes = Bytes::copy_from_slice(&[0x01, 0x02, 0x03, 0x04]);
        let inner_element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(inner_bytes),
        );

        // Create a serialized version by encoding the inner element
        let mut serialized = Vec::new();
        x690_write_tlv(&mut serialized, &inner_element).unwrap();

        let element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Serialized(Bytes::copy_from_slice(&serialized)),
        );

        let result = deconstruct(&element).unwrap();
        assert_eq!(result.as_ref(), &[0x01, 0x02, 0x03, 0x04]);
        // Should be owned since it's serialized
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn test_deconstruct_serialized_constructed() {
        // Test deconstructing a serialized constructed value
        let child1 = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(&[0x01, 0x02])),
        );
        let child2 = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(&[0x03, 0x04])),
        );

        let inner_element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Constructed(Arc::new(vec![child1, child2])),
        );

        // Create a serialized version by encoding the inner element
        let mut serialized = Vec::new();
        x690_write_tlv(&mut serialized, &inner_element).unwrap();

        let element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Serialized(Bytes::copy_from_slice(&serialized)),
        );

        let result = deconstruct(&element).unwrap();
        assert_eq!(result.as_ref(), &[0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_deconstruct_mixed_constructed() {
        // Test deconstructing a constructed value with mixed primitive and constructed children
        let primitive_child = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(&[0x01, 0x02])),
        );

        let grandchild1 = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(&[0x03, 0x04])),
        );
        let grandchild2 = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(&[0x05, 0x06])),
        );

        let constructed_child = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Constructed(Arc::new(vec![grandchild1, grandchild2])),
        );

        let element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Constructed(Arc::new(vec![primitive_child, constructed_child])),
        );

        let result = deconstruct(&element).unwrap();
        assert_eq!(result.as_ref(), &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    fn test_deconstruct_large_data() {
        // Test deconstructing with larger data to ensure performance
        let mut children = Vec::new();
        let mut expected = Vec::new();

        for i in 0..100 {
            let data = vec![i as u8, (i + 1) as u8, (i + 2) as u8];
            expected.extend_from_slice(&data);

            let child = X690Element::new(
                Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
                X690Value::Primitive(Bytes::copy_from_slice(&data)),
            );
            children.push(child);
        }

        let element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Constructed(Arc::new(children)),
        );

        let result = deconstruct(&element).unwrap();
        assert_eq!(result.as_ref(), &expected);
    }

    #[test]
    fn test_deconstruct_serialized_invalid_data() {
        // Test deconstructing a serialized value with invalid data
        let element = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Serialized(Bytes::copy_from_slice(&[0x01, 0x02, 0x03])), // Invalid BER
        );

        let result = deconstruct(&element);
        assert!(result.is_err());
    }

    #[test]
    fn test_element_equality_1() {
        let element1 = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(&[0x01, 0x02])),
        );
        let element2 = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(&[0x01, 0x02])),
        );
        assert_eq!(element1, element2);
    }

}
