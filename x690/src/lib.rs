pub mod ber;
pub mod codec;
pub mod parsing;
pub(crate) mod utils;
pub use crate::ber::*;
pub use crate::codec::*;
pub use crate::parsing::*;
pub use crate::utils::*;
use wildboar_asn1::error::{ASN1Error, ASN1ErrorCode, ASN1Result};
use wildboar_asn1::{
    ASN1Value, ByteSlice, CharacterString, EmbeddedPDV, ExternalEncoding,
    ExternalIdentification, GeneralizedTime, ObjectDescriptor,
    PresentationContextSwitchingTypeIdentification, Tag, TagClass, TagNumber, TaggedASN1Value,
    UTCTime, UniversalString, UNIV_TAG_BIT_STRING,
    UNIV_TAG_BMP_STRING, UNIV_TAG_BOOLEAN,
    UNIV_TAG_CHARACTER_STRING, UNIV_TAG_DATE,
    UNIV_TAG_DATE_TIME, UNIV_TAG_DURATION,
    UNIV_TAG_EMBEDDED_PDV, UNIV_TAG_END_OF_CONTENT,
    UNIV_TAG_ENUMERATED, UNIV_TAG_EXTERNAL,
    UNIV_TAG_GENERALIZED_TIME, UNIV_TAG_GENERAL_STRING,
    UNIV_TAG_GRAPHIC_STRING, UNIV_TAG_IA5_STRING,
    UNIV_TAG_INTEGER, UNIV_TAG_NULL,
    UNIV_TAG_NUMERIC_STRING, UNIV_TAG_OBJECT_DESCRIPTOR,
    UNIV_TAG_OBJECT_IDENTIFIER, UNIV_TAG_OCTET_STRING,
    UNIV_TAG_OID_IRI, UNIV_TAG_PRINTABLE_STRING,
    UNIV_TAG_REAL, UNIV_TAG_RELATIVE_OID,
    UNIV_TAG_RELATIVE_OID_IRI, UNIV_TAG_SEQUENCE,
    UNIV_TAG_SEQUENCE_OF, UNIV_TAG_SET,
    UNIV_TAG_SET_OF, UNIV_TAG_T61_STRING,
    UNIV_TAG_TIME, UNIV_TAG_TIME_OF_DAY,
    UNIV_TAG_UNIVERSAL_STRING, UNIV_TAG_UTC_TIME,
    UNIV_TAG_UTF8_STRING, UNIV_TAG_VIDEOTEX_STRING,
    UNIV_TAG_VISIBLE_STRING, BIT_STRING, BOOLEAN, DATE, DATE_TIME,
    DURATION_EQUIVALENT, EXTERNAL, INTEGER, OBJECT_IDENTIFIER,
    OCTET_STRING, REAL, RELATIVE_OID, TIME, TIME_OF_DAY,
};
use wildboar_asn1::{ENUMERATED, read_i64, DURATION, ComponentSpec, TagSelector};
use std::borrow::Cow;
use std::io::{Error, ErrorKind, Result, Write};
use std::mem::size_of;
use std::sync::Arc;
use smallvec::{smallvec, SmallVec};
use bytes::{Bytes, BytesMut, BufMut};

pub const X690_TAG_CLASS_UNIVERSAL: u8 = 0b0000_0000;
pub const X690_TAG_CLASS_APPLICATION: u8 = 0b0100_0000;
pub const X690_TAG_CLASS_CONTEXT: u8 = 0b1000_0000;
pub const X690_TAG_CLASS_PRIVATE: u8 = 0b1100_0000;
pub const X690_SPECIAL_REAL_PLUS_INFINITY: u8 = 0b0000_0000;
pub const X690_SPECIAL_REAL_MINUS_INFINITY: u8 = 0b0000_0001;
pub const X690_SPECIAL_REAL_NOT_A_NUMBER: u8 = 0b0000_0010;
pub const X690_SPECIAL_REAL_MINUS_ZERO: u8 = 0b0000_0011;
pub const X690_REAL_SPECIAL: u8 = 0b0100_0000;
pub const X690_REAL_BASE10: u8 = 0b0000_0000;
pub const X690_REAL_BINARY: u8 = 0b1000_0000;
pub const X690_REAL_POSITIVE: u8 = 0b0000_0000;
pub const X690_REAL_NEGATIVE: u8 = 0b0100_0000;
pub const X690_REAL_SIGN_MASK: u8 = 0b0100_0000;
pub const X690_REAL_BASE_MASK: u8 = 0b0011_0000;
pub const X690_REAL_BASE_2: u8 = 0b0000_0000;
pub const X690_REAL_BASE_8: u8 = 0b0001_0000;
pub const X690_REAL_BASE_16: u8 = 0b0010_0000;
pub const X690_REAL_BASE_RESERVED: u8 = 0b0011_0000;
pub const X690_REAL_BINARY_SCALING_MASK: u8 = 0b0000_1100;
pub const X690_REAL_EXPONENT_FORMAT_MASK: u8 = 0b0000_0011;
pub const X690_REAL_EXPONENT_FORMAT_1_OCTET: u8 = 0b0000_0000;
pub const X690_REAL_EXPONENT_FORMAT_2_OCTET: u8 = 0b0000_0001;
pub const X690_REAL_EXPONENT_FORMAT_3_OCTET: u8 = 0b0000_0010;
pub const X690_REAL_EXPONENT_FORMAT_VAR_OCTET: u8 = 0b0000_0011;
pub const X690_REAL_NR1: u8 = 1;
pub const X690_REAL_NR2: u8 = 2;
pub const X690_REAL_NR3: u8 = 3;
// const IEEE_754_DPFP_SIGN_MASK

#[derive(Clone, Debug, Hash, Copy, PartialEq, Eq)]
pub enum X690Length {
    Definite(usize),
    Indefinite,
}

#[derive(Clone, Debug)]
pub struct X690ComponentIterator<'a>{
    pub el: &'a X690Element,
    pub i: usize,
}

impl <'a> Iterator for X690ComponentIterator<'a> {
    type Item = X690Element;

    fn next(&mut self) -> Option<Self::Item> {
        if let X690Value::Constructed(components) = &self.el.value {
            let i = self.i;
            self.i += 1;
            components.get(i).cloned()
        } else {
            None
        }
    }

}

#[derive(Clone, Debug, Hash)]
pub enum X690Value {
    Primitive(Bytes),
    Constructed(Arc<Vec<X690Element>>),
    // TODO: Serialized variant?
}

impl X690Value {

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
        }
    }

    pub fn from_explicit(inner: X690Element) -> Self {
        X690Value::Constructed(Arc::new(Vec::from([ inner ])))
    }

}

#[derive(Clone, Debug, Hash)]
pub struct X690Element {
    pub tag: Tag,
    pub value: X690Value,
}

impl X690Element {

    pub fn new(tag: Tag, value: X690Value) -> X690Element {
        X690Element { tag, value }
    }

    pub fn len(&self) -> usize {
        let tag_length: usize = get_written_x690_tag_length(self.tag.tag_number);
        let value_length = match &self.value {
            X690Value::Primitive(v) => v.len(),
            X690Value::Constructed(components) => {
                let mut sum: usize = 0;
                for component in components.iter() {
                    sum += component.len();
                }
                sum
            },
        };
        let length_length: usize = get_written_x690_length_length(value_length);
        let ret = tag_length + length_length + value_length;
        ret
    }

    pub fn is_constructed (&self) -> bool {
        if let X690Value::Constructed(_) = self.value {
            true
        } else {
            false
        }
    }

    pub fn components (&self) -> X690ComponentIterator<'_> {
        X690ComponentIterator {
            el: self,
            i: 0,
        }
    }

    pub fn inner(&self) -> ASN1Result<X690Element> {
        match &self.value {
            X690Value::Constructed(components) => {
                if components.len() != 1 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                }
                Ok(components[0].clone())
            },
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }

    pub fn content_octets <'a> (&'a self) -> ASN1Result<Cow<'a, [u8]>> {
        match &self.value {
            X690Value::Primitive(v) => Ok(Cow::Borrowed(&v)),
            X690Value::Constructed(_) => {
                let mut output = BytesMut::with_capacity(self.len()).writer();
                write_x690_encoding(&mut output, &self.value)?;
                Ok(Cow::Owned(output.into_inner().into()))
            },
        }
    }

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

    pub fn to_asn1_err_named (&self, errcode: ASN1ErrorCode, name: &str) -> ASN1Error {
        ASN1Error {
            error_code: errcode,
            component_name: Some(name.to_string()),
            tag: Some(Tag::new(self.tag.tag_class, self.tag.tag_number)),
            length: Some(self.len()),
            constructed: Some(self.is_constructed()),
            value_preview: None,
            bytes_read: None,
            values_read: None,
            err_source: None,
        }
    }

    pub fn is_empty (&self) -> bool {
        match &self.value {
            X690Value::Primitive(v) => v.len() == 0,
            X690Value::Constructed(components) => components.len() == 0,
        }
    }

}

impl From<i8> for X690Element {
    #[inline]
    fn from(value: i8) -> Self {
        BER.encode_i8(value).unwrap()
    }
}

impl From<i16> for X690Element {
    #[inline]
    fn from(value: i16) -> Self {
        BER.encode_i16(value).unwrap()
    }
}

impl From<i32> for X690Element {
    #[inline]
    fn from(value: i32) -> Self {
        BER.encode_i32(value).unwrap()
    }
}

impl From<i64> for X690Element {
    #[inline]
    fn from(value: i64) -> Self {
        BER.encode_i64(value).unwrap()
    }
}

impl From<u8> for X690Element {
    #[inline]
    fn from(value: u8) -> Self {
        BER.encode_u8(value).unwrap()
    }
}

impl From<u16> for X690Element {
    #[inline]
    fn from(value: u16) -> Self {
        BER.encode_u16(value).unwrap()
    }
}

impl From<u32> for X690Element {
    #[inline]
    fn from(value: u32) -> Self {
        BER.encode_u32(value).unwrap()
    }
}

impl From<u64> for X690Element {
    #[inline]
    fn from(value: u64) -> Self {
        BER.encode_u64(value).unwrap()
    }
}

impl From<OBJECT_IDENTIFIER> for X690Element {
    #[inline]
    fn from(value: OBJECT_IDENTIFIER) -> Self {
        X690Element::from(&value)
    }
}

impl From<&OBJECT_IDENTIFIER> for X690Element {
    #[inline]
    fn from(value: &OBJECT_IDENTIFIER) -> Self {
        BER.encode_object_identifier(value).unwrap()
    }
}

impl From<bool> for X690Element {
    #[inline]
    fn from(value: bool) -> Self {
        BER.encode_boolean(&value).unwrap()
    }
}

impl From<DATE> for X690Element {
    #[inline]
    fn from(value: DATE) -> Self {
        BER.encode_date(&value).unwrap()
    }
}

impl From<TIME_OF_DAY> for X690Element {
    #[inline]
    fn from(value: TIME_OF_DAY) -> Self {
        BER.encode_time_of_day(&value).unwrap()
    }
}

impl From<DATE_TIME> for X690Element {
    #[inline]
    fn from(value: DATE_TIME) -> Self {
        BER.encode_date_time(&value).unwrap()
    }
}

impl From<TIME> for X690Element {
    #[inline]
    fn from(value: TIME) -> Self {
        BER.encode_time(&value).unwrap()
    }
}

impl From<DURATION> for X690Element {
    #[inline]
    fn from(value: DURATION) -> Self {
        BER.encode_duration(&value).unwrap()
    }
}

impl TryInto<i8> for X690Element {
    type Error = ASN1Error;
    #[inline]
    fn try_into(self) -> ASN1Result<i8> {
        BER.decode_i8(&self)
    }
}

impl TryInto<i16> for X690Element {
    type Error = ASN1Error;
    #[inline]
    fn try_into(self) -> ASN1Result<i16> {
        BER.decode_i16(&self)
    }
}

impl TryInto<i32> for X690Element {
    type Error = ASN1Error;
    #[inline]
    fn try_into(self) -> ASN1Result<i32> {
        BER.decode_i32(&self)
    }
}

impl TryInto<i64> for X690Element {
    type Error = ASN1Error;
    #[inline]
    fn try_into(self) -> ASN1Result<i64> {
        BER.decode_i64(&self)
    }
}

impl TryInto<i128> for X690Element {
    type Error = ASN1Error;
    #[inline]
    fn try_into(self) -> ASN1Result<i128> {
        BER.decode_i128(&self)
    }
}

impl TryInto<u8> for X690Element {
    type Error = ASN1Error;
    #[inline]
    fn try_into(self) -> ASN1Result<u8> {
        BER.decode_u8(&self)
    }
}

impl TryInto<u16> for X690Element {
    type Error = ASN1Error;
    #[inline]
    fn try_into(self) -> ASN1Result<u16> {
        BER.decode_u16(&self)
    }
}

impl TryInto<u32> for X690Element {
    type Error = ASN1Error;
    #[inline]
    fn try_into(self) -> ASN1Result<u32> {
        BER.decode_u32(&self)
    }
}

impl TryInto<u64> for X690Element {
    type Error = ASN1Error;
    #[inline]
    fn try_into(self) -> ASN1Result<u64> {
        BER.decode_u64(&self)
    }
}

impl TryInto<u128> for X690Element {
    type Error = ASN1Error;
    #[inline]
    fn try_into(self) -> ASN1Result<u128> {
        BER.decode_u128(&self)
    }
}

impl TryInto<BOOLEAN> for X690Element {
    type Error = ASN1Error;
    #[inline]
    fn try_into(self) -> ASN1Result<BOOLEAN> {
        BER.decode_boolean(&self)
    }
}

// impl PartialEq for X690Element {
//     fn eq(&self, other: &Self) -> bool {
//         let Ok(thing1) = (self) else {
//             return false;
//         };
//         let Ok(thing2) = ber_decode_any(other) else {
//             return false;
//         };
//         thing1 == thing2
//     }
// }

pub fn get_x690_tag_and_length_length(bytes: ByteSlice) -> usize {
    if bytes.len() == 0 {
        return 0;
    }
    let mut len: usize = 1;
    if (bytes[0] & 0b00011111) == 0b00011111 {
        // If it is a long tag...
        for byte in bytes[1..].into_iter() {
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

fn base_128_len(num: u32) -> usize {
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

fn write_base_128<W>(output: &mut W, mut num: u32) -> Result<usize>
where
    W: Write,
{
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

// Just gets the theoretical length of the written tag.
pub fn get_written_x690_tag_length(tagnum: TagNumber) -> usize {
    if tagnum < 31 {
        // See ITU Rec. X.690 (2021), Section 8.1.2.4.
        return 1;
    }
    base_128_len(tagnum.into()) + 1
}

// Just gets the length of the length bytes for an element
pub fn get_written_x690_length_length(len: usize) -> usize {
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
    }
}

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

pub fn x690_write_integer_value<W>(output: &mut W, value: &INTEGER) -> Result<usize>
where
    W: Write,
{
    output.write(value)
}

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

pub fn x690_write_enum_value<W>(output: &mut W, value: &ENUMERATED) -> Result<usize>
where
    W: Write,
{
    x690_write_i64_value(output, *value)
}

pub fn x690_write_bit_string_value<W>(output: &mut W, value: &BIT_STRING) -> Result<usize>
where
    W: Write,
{
    output.write(&[value.get_trailing_bits_count()])?;
    let bytes_written = output.write(value.get_bytes_ref())?;
    Ok(bytes_written + 1)
}

pub fn x690_write_octet_string_value<W>(output: &mut W, value: &OCTET_STRING) -> Result<usize>
where
    W: Write,
{
    output.write(value)
}

pub fn x690_write_object_identifier_value<W>(
    output: &mut W,
    value: &OBJECT_IDENTIFIER,
) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_x690_slice())
}

pub fn x690_write_object_descriptor_value<W>(
    output: &mut W,
    value: &str,
) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_bytes())
}

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
        ExternalEncoding::single_ASN1_type(t) => ber_encode(&mut data_value_bytes, t)?,
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

// NOTE: This has to be encoded in a strange way that is detailed in ITU Recommendation X.690, Section 8.18.
pub fn x690_write_external_value<W>(output: &mut W, value: &EXTERNAL) -> Result<usize>
where
    W: Write,
{
    let components = x690_encode_external_components(value)?;
    let mut bytes_written: usize = 0;
    for component in components {
        bytes_written += write_x690_node(output, &component)?;
    }
    Ok(bytes_written)
}

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
        return output.write(&[X690_SPECIAL_REAL_MINUS_ZERO]);
    }

    if value.is_nan() {
        return output.write(&[X690_SPECIAL_REAL_NOT_A_NUMBER]);
    }

    if value.is_infinite() {
        if value.is_sign_negative() {
            return output.write(&[X690_SPECIAL_REAL_MINUS_INFINITY]);
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
    let bytes = value.to_be_bytes();
    let mut mantissa: u64 = u64::from_be_bytes([
        0x00,
        0x08 & bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
    ]);
    let mut exponent: u16 = u16::from_be_bytes([bytes[0] & 0x7F, bytes[1] & 0b1110_0000]) >> 5;
    while (mantissa > 0) && !((mantissa % 2) > 0) {
        mantissa = mantissa >> 1;
        exponent += 1;
    }
    let e_bytes = exponent.to_be_bytes();
    let mut bytes_written: usize = 0;
    if exponent > u8::MAX as u16 {
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

pub fn x690_context_switching_identification_to_cst(
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

pub fn x690_encode_embedded_pdv_components (value: &EmbeddedPDV) -> Result<Vec<X690Element>> {
    let id = x690_context_switching_identification_to_cst(&value.identification)?;
    let mut data_value_bytes = BytesMut::new().writer();
    x690_write_octet_string_value(&mut data_value_bytes, &value.data_value)?;
    let data_value_element = X690Element::new(
        Tag::new(TagClass::CONTEXT, 1),
        X690Value::Primitive(data_value_bytes.into_inner().into()),
    );
    Ok(vec![id, data_value_element])
}

pub fn x690_write_embedded_pdv_value<W>(output: &mut W, value: &EmbeddedPDV) -> Result<usize>
where
    W: Write,
{
    let components: Vec<X690Element> = x690_encode_embedded_pdv_components(value)?;
    let mut bytes_written: usize = 0;
    for component in components {
        bytes_written += write_x690_node(output, &component)?;
    }
    Ok(bytes_written)
}

pub fn x690_write_utf8_string_value<W>(output: &mut W, value: &str) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_bytes())
}

pub fn x690_write_relative_oid_value<W>(output: &mut W, value: &RELATIVE_OID) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_x690_slice())
}

pub fn x690_write_time_value<W>(output: &mut W, value: &TIME) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_bytes())
}

pub fn x690_write_utc_time_value<W>(output: &mut W, value: &UTCTime) -> Result<usize>
where
    W: Write,
{
    output.write(value.to_string().as_bytes())
}

pub fn x690_write_generalized_time_value<W>(
    output: &mut W,
    value: &GeneralizedTime,
) -> Result<usize>
where
    W: Write,
{
    output.write(value.to_string().as_bytes())
}

pub fn x690_write_universal_string_value<W>(
    output: &mut W,
    value: &str,
) -> Result<usize>
where
    W: Write,
{
    let bytes: Vec<u8> = value
        .chars()
        .map(|c| c as u32)
        .flat_map(|c| {
            [
                ((c & 0xFF000000) >> 24) as u8,
                ((c & 0x00FF0000) >> 16) as u8,
                ((c & 0x0000FF00) >> 8) as u8,
                (c & 0x000000FF) as u8,
            ]
        })
        .collect();
    output.write(bytes.as_slice())
}

pub fn x690_encode_character_string_components (value: &CharacterString) -> Result<Vec<X690Element>> {
    let id = x690_context_switching_identification_to_cst(&value.identification)?;
    let mut data_value_bytes = BytesMut::new().writer();
    x690_write_octet_string_value(&mut data_value_bytes, &value.string_value)?;
    let data_value_element = X690Element::new(
        Tag::new(TagClass::CONTEXT, 1),
        X690Value::Primitive(data_value_bytes.into_inner().into()),
    );
    Ok(vec![id, data_value_element])
}

// This is almost the same for EmbeddedPDV.
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
        bytes_written += write_x690_node(output, &component)?;
    }
    Ok(bytes_written)
}

pub fn x690_write_bmp_string_value<W>(output: &mut W, value: &str) -> Result<usize>
where
    W: Write,
{
    let bytes: Vec<u8> = value
        .chars()
        .map(|c| c as u16)
        .flat_map(|c| {
            [
                // ((c & 0xFF000000) >> 24) as u8,
                // ((c & 0x00FF0000) >> 16) as u8,
                ((c & 0x0000FF00) >> 8) as u8,
                (c & 0x000000FF) as u8,
            ]
        })
        .collect();
    output.write(bytes.as_slice())
}

// This does not do any validation.
#[inline]
pub fn x690_write_string_value<W>(output: &mut W, value: &str) -> Result<usize>
where
    W: Write,
{
    output.write(value.as_bytes())
}

pub fn x690_write_date_value<W>(output: &mut W, value: &DATE) -> Result<usize>
where
    W: Write,
{
    if value.month > 12 || value.month == 0 || value.day > 31 || value.day == 0 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    output.write(value.to_string().as_bytes())
}

pub fn x690_write_time_of_day_value<W>(output: &mut W, value: &TIME_OF_DAY) -> Result<usize>
where
    W: Write,
{
    if value.hour > 23 || value.minute > 59 || value.second > 59 {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    output.write(value.to_string().as_bytes())
}

pub fn x690_write_date_time_value<W>(output: &mut W, value: &DATE_TIME) -> Result<usize>
where
    W: Write,
{
    output.write(value.to_string().as_bytes())
}

pub fn x690_write_duration_value<W>(output: &mut W, value: &DURATION_EQUIVALENT) -> Result<usize>
where
    W: Write,
{
    output.write(value.to_string().as_bytes())
}

pub fn create_x690_cst_node(value: &ASN1Value) -> Result<X690Element> {
    let mut tag_class: TagClass = TagClass::UNIVERSAL;
    let mut tag_number: TagNumber = 0;
    let encoded_value: X690Value;
    match value {
        ASN1Value::UnknownBytes(v) => {
            encoded_value = X690Value::Primitive(Bytes::copy_from_slice(v));
        }
        ASN1Value::TaggedValue(v) => {
            tag_class = v.tag.tag_class;
            tag_number = v.tag.tag_number;
            if v.explicit {
                let root = create_x690_cst_node(&v.value)?;
                encoded_value = X690Value::from_explicit(root);
            } else {
                let root = create_x690_cst_node(&v.value)?;
                encoded_value = root.value;
            }
        }
        ASN1Value::BooleanValue(v) => {
            tag_number = UNIV_TAG_BOOLEAN;
            encoded_value = X690Value::Primitive(Bytes::copy_from_slice(&[ if *v { 0xFF } else { 0x00 } ]));
        }
        ASN1Value::IntegerValue(v) => {
            tag_number = UNIV_TAG_INTEGER;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_integer_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::BitStringValue(v) => {
            tag_number = UNIV_TAG_BIT_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len_in_bytes() + 1).writer();
            x690_write_bit_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::OctetStringValue(v) => {
            tag_number = UNIV_TAG_OCTET_STRING;
            encoded_value = X690Value::Primitive(Bytes::copy_from_slice(v));
        }
        ASN1Value::NullValue => {
            tag_number = UNIV_TAG_NULL;
            encoded_value = X690Value::Primitive(Bytes::new());
        }
        ASN1Value::ObjectIdentifierValue(v) => {
            tag_number = UNIV_TAG_OBJECT_IDENTIFIER;
            let mut value_bytes = BytesMut::with_capacity(v.as_x690_slice().len()).writer();
            x690_write_object_identifier_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::ExternalValue(v) => {
            tag_number = UNIV_TAG_EXTERNAL;
            encoded_value = X690Value::Constructed(Arc::new(x690_encode_external_components(v)?));
        }
        ASN1Value::RealValue(v) => {
            tag_number = UNIV_TAG_REAL;
            let mut value_bytes = BytesMut::new().writer();
            x690_write_real_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::EnumeratedValue(v) => {
            tag_number = UNIV_TAG_ENUMERATED;
            let mut value_bytes = BytesMut::with_capacity(8).writer();
            x690_write_enum_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::EmbeddedPDVValue(v) => {
            tag_number = UNIV_TAG_EMBEDDED_PDV;
            encoded_value = X690Value::Constructed(Arc::new(x690_encode_embedded_pdv_components(v)?));
        }
        ASN1Value::UTF8String(v) => {
            tag_number = UNIV_TAG_UTF8_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_utf8_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::RelativeOIDValue(v) => {
            tag_number = UNIV_TAG_RELATIVE_OID;
            let mut value_bytes = BytesMut::with_capacity(v.as_x690_slice().len()).writer();
            x690_write_relative_oid_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::TimeValue(v) => {
            tag_number = UNIV_TAG_TIME;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_time_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::SequenceValue(v) => {
            tag_number = UNIV_TAG_SEQUENCE;
            let mut inner_values: Vec<X690Element> = Vec::with_capacity(v.len());
            for inner_value in v {
                let inner_node = create_x690_cst_node(inner_value)?;
                inner_values.push(inner_node);
            }
            encoded_value = X690Value::Constructed(Arc::new(inner_values));
        }
        ASN1Value::SequenceOfValue(v) => {
            tag_number = UNIV_TAG_SEQUENCE_OF;
            let mut inner_values: Vec<X690Element> = Vec::with_capacity(v.len());
            for inner_value in v {
                let inner_node = create_x690_cst_node(inner_value)?;
                inner_values.push(inner_node);
            }
            encoded_value = X690Value::Constructed(Arc::new(inner_values));
        }
        ASN1Value::SetValue(v) => {
            tag_number = UNIV_TAG_SET;
            let mut inner_values: Vec<X690Element> = Vec::with_capacity(v.len());
            for inner_value in v {
                let inner_node = create_x690_cst_node(inner_value)?;
                inner_values.push(inner_node);
            }
            encoded_value = X690Value::Constructed(Arc::new(inner_values));
        }
        ASN1Value::SetOfValue(v) => {
            tag_number = UNIV_TAG_SET_OF;
            let mut inner_values: Vec<X690Element> = Vec::with_capacity(v.len());
            for inner_value in v {
                let inner_node = create_x690_cst_node(inner_value)?;
                inner_values.push(inner_node);
            }
            encoded_value = X690Value::Constructed(Arc::new(inner_values));
        }
        ASN1Value::UTCTime(v) => {
            tag_number = UNIV_TAG_UTC_TIME;
            let mut value_bytes = BytesMut::with_capacity(17).writer(); // This is the max length of a UTCTime.
            x690_write_utc_time_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::GeneralizedTime(v) => {
            tag_number = UNIV_TAG_GENERALIZED_TIME;
            let mut value_bytes = BytesMut::with_capacity(32).writer(); // This should cover most values.
            x690_write_generalized_time_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::UniversalString(v) => {
            tag_number = UNIV_TAG_UNIVERSAL_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len() << 2).writer();
            x690_write_universal_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::UnrestrictedCharacterStringValue(v) => {
            tag_number = UNIV_TAG_CHARACTER_STRING;
            encoded_value = X690Value::Constructed(Arc::new(x690_encode_character_string_components(v)?));
        }
        ASN1Value::BMPString(v) => {
            tag_number = UNIV_TAG_BMP_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len() << 1).writer();
            x690_write_bmp_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::InstanceOfValue(v) => {
            tag_number = UNIV_TAG_EXTERNAL;
            let type_id = ASN1Value::ObjectIdentifierValue(v.type_id.clone());
            let val = TaggedASN1Value {
                tag: Tag::new(TagClass::CONTEXT, 0),
                explicit: true,
                value: v.value.clone(),
            };
            let value = ASN1Value::TaggedValue(val);
            let type_id_element = create_x690_cst_node(&type_id)?;
            let value_element = create_x690_cst_node(&value)?;
            encoded_value = X690Value::Constructed(Arc::new(vec![
                type_id_element,
                value_element,
            ]));
        }
        ASN1Value::IRIValue(v) => {
            tag_number = UNIV_TAG_OID_IRI;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::RelativeIRIValue(v) => {
            tag_number = UNIV_TAG_RELATIVE_OID_IRI;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::GeneralString(v) => {
            if !v.is_ascii() {
                // GeneralString must be below or at 0x7F.
                return Err(Error::from(ErrorKind::InvalidData));
            }
            tag_number = UNIV_TAG_GENERAL_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::IA5String(v) => {
            for c in v.chars() {
                if c > 127 as char {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = UNIV_TAG_IA5_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::GraphicString(v) => {
            for c in v.chars() {
                if !c.is_ascii_graphic() && (c != ' ') {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = UNIV_TAG_GRAPHIC_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::ISO646String(v) => {
            if !v.is_ascii() {
                // VisibleString / ISO646String must be below 0x7F.
                return Err(Error::from(ErrorKind::InvalidData));
            }
            for c in v.chars() {
                if c == '\x7F' {
                    // DELETE not allowed.
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = UNIV_TAG_VISIBLE_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::VisibleString(v) => {
            if !v.is_ascii() {
                // VisibleString / ISO646String must be below 0x7F.
                return Err(Error::from(ErrorKind::InvalidData));
            }
            for c in v.chars() {
                if c == '\x7F' {
                    // DELETE not allowed.
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = UNIV_TAG_VISIBLE_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::NumericString(v) => {
            for c in v.chars() {
                if !c.is_ascii_digit() && c != ' ' {
                    // SPACE is allowed in NumericString.
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = UNIV_TAG_NUMERIC_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::PrintableString(v) => {
            for c in v.chars() {
                if c.is_ascii_alphanumeric()
                    || (c >= '\x27' && c < '0' && c != '*') // '()+,-./ BUT NOT *
                    || c == ' '
                    || c == ':'
                    || c == '='
                    || c == '?'
                {
                    continue;
                }
                return Err(Error::from(ErrorKind::InvalidData));
            }
            tag_number = UNIV_TAG_PRINTABLE_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::TeletexString(v) => {
            tag_number = UNIV_TAG_T61_STRING;
            encoded_value = X690Value::Primitive(Bytes::copy_from_slice(v));
        }
        ASN1Value::T61String(v) => {
            tag_number = UNIV_TAG_T61_STRING;
            encoded_value = X690Value::Primitive(Bytes::copy_from_slice(v));
        }
        ASN1Value::VideotexString(v) => {
            tag_number = UNIV_TAG_VIDEOTEX_STRING;
            encoded_value = X690Value::Primitive(Bytes::copy_from_slice(v));
        }
        ASN1Value::DATE(v) => {
            tag_number = UNIV_TAG_DATE;
            let mut value_bytes = BytesMut::with_capacity(10).writer();
            x690_write_date_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::TIME_OF_DAY(v) => {
            tag_number = UNIV_TAG_TIME_OF_DAY;
            let mut value_bytes = BytesMut::with_capacity(8).writer();
            x690_write_time_of_day_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::DATE_TIME(v) => {
            tag_number = UNIV_TAG_DATE_TIME;
            let mut value_bytes = BytesMut::with_capacity(19).writer(); // 1951-10-14T15:30:00
            x690_write_date_time_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::DURATION(v) => {
            tag_number = UNIV_TAG_DURATION;
            // There is no guaranteed size, but 16 is a reasonable pre-allocation.
            let mut value_bytes = BytesMut::with_capacity(16).writer();
            x690_write_duration_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::ObjectDescriptor(v) => {
            for c in v.chars() {
                if !c.is_ascii_graphic() && (c != ' ') {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
            tag_number = UNIV_TAG_GRAPHIC_STRING;
            let mut value_bytes = BytesMut::with_capacity(v.len()).writer();
            x690_write_string_value(&mut value_bytes, v)?;
            encoded_value = X690Value::Primitive(value_bytes.into_inner().into());
        }
        ASN1Value::ChoiceValue(v) => {
            return create_x690_cst_node(v);
        }
    };

    Ok(X690Element::new(Tag::new(tag_class, tag_number), encoded_value))
}

fn write_x690_encoding<W>(output: &mut W, encoding: &X690Value) -> Result<usize>
where
    W: Write,
{
    match encoding {
        X690Value::Primitive(v) => output.write(&v),
        X690Value::Constructed(components) => {
            let mut sum: usize = 0;
            for component in components.iter() {
                sum += write_x690_node(output, component)?;
            }
            Ok(sum)
        },
    }
}

pub fn write_x690_node<W>(output: &mut W, node: &X690Element) -> Result<usize>
where
    W: Write,
{
    let mut bytes_written: usize = 0;
    bytes_written += x690_write_tag(output, node.tag.tag_class, node.is_constructed(), node.tag.tag_number)?;
    bytes_written += x690_write_length(output, node.value.len())?;
    bytes_written += write_x690_encoding(output, &node.value)?;
    Ok(bytes_written)
}

pub fn ber_encode<W>(output: &mut W, value: &ASN1Value) -> Result<usize>
where
    W: Write,
{
    let root = create_x690_cst_node(value)?;
    write_x690_node(output, &root)
}

// TODO: This needs testing.
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
        }
    }
}

pub fn x690_read_boolean_value(value_bytes: ByteSlice) -> ASN1Result<BOOLEAN> {
    if value_bytes.len() != 1 {
        let err = ASN1Error::new(ASN1ErrorCode::x690_boolean_not_one_byte);
        return Err(err);
    }
    Ok(value_bytes[0] > 0)
}

pub fn x690_read_integer_value(value_bytes: ByteSlice) -> ASN1Result<INTEGER> {
    // Intentionally not validating this. Most integers are small and correct.
    // If they have padding, its obvious how to handle that.
    Ok(Vec::from(value_bytes))
}

pub fn x690_read_i64_value(value_bytes: ByteSlice) -> ASN1Result<i64> {
    match read_i64(value_bytes) {
        Some(v) => Ok(v),
        None => Err(ASN1Error::new(ASN1ErrorCode::value_too_big)),
    }
}

pub fn x690_read_enum_value(value_bytes: ByteSlice) -> ASN1Result<ENUMERATED> {
    x690_read_i64_value(value_bytes)
}

pub fn x690_read_object_identifier_value(value_bytes: ByteSlice) -> ASN1Result<OBJECT_IDENTIFIER> {
    OBJECT_IDENTIFIER::from_x690_encoding_slice(value_bytes)
}

pub fn x690_read_relative_oid_value(value_bytes: ByteSlice) -> ASN1Result<RELATIVE_OID> {
    RELATIVE_OID::from_x690_encoding_slice(value_bytes)
}

pub fn x690_read_date_value(value_bytes: ByteSlice) -> ASN1Result<DATE> {
    DATE::try_from(value_bytes)
}

pub fn x690_read_time_of_day_value(value_bytes: ByteSlice) -> ASN1Result<TIME_OF_DAY> {
    TIME_OF_DAY::try_from(value_bytes)
}

pub fn x690_read_date_time_value(value_bytes: ByteSlice) -> ASN1Result<DATE_TIME> {
    DATE_TIME::try_from(value_bytes)
}

pub fn x690_read_duration_value(value_bytes: ByteSlice) -> ASN1Result<DURATION> {
    DURATION::try_from(value_bytes)
}


pub trait RelateTLV {
    fn relatve_tlv (&mut self, el: &X690Element);
}

impl RelateTLV for ASN1Error {
    fn relatve_tlv (&mut self, el: &X690Element) {
        self.tag = Some(el.tag);
        self.constructed = Some(el.is_constructed());
        self.length = Some(el.len());
    }
}

// [UNIVERSAL 8] IMPLICIT SEQUENCE {
//     direct-reference        OBJECT IDENTIFIER OPTIONAL,
//     indirect-reference      INTEGER OPTIONAL,
//     data-value-descriptor   ObjectDescriptor OPTIONAL,
//     encoding CHOICE {
//         single-ASN1-type    [0] ABSTRACT-SYNTAX.&Type,
//         octet-aligned       [1] IMPLICIT OCTET STRING,
//         arbitrary           [2] IMPLICIT BIT STRING } }
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
pub const _EAL_FOR_EXTERNAL: &[ComponentSpec; 0] = &[];
pub const _RCTL2_FOR_EXTERNAL: &[ComponentSpec; 0] = &[];

#[cfg(test)]
mod tests {

    use super::*;
    use std::sync::Arc;

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

    // #[test]
    // fn test_x690_write_bit_string_value () {
    //     use bitvec::prelude::*;
    //     let mut output = BytesMut::new();
    //     let mut bits = bitvec![usize, Lsb0; 0, 1, 0, 0, 1];
    //     crate::x690_write_bit_string_value(&mut output, &mut bits).unwrap();
    //     // assert_eq!(output.len(), 2);
    //     assert_eq!(output[0], 3);
    //     assert_eq!(output[1], 0b0100_1000);
    // }

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

    // #[test]
    // fn test_ber_encode() {
    //     let mut output = BytesMut::new().writer();
    //     let val = TaggedASN1Value {
    //         tag: Tag::new(TagClass::APPLICATION, 5),
    //         explicit: true,
    //         value: Arc::new(ASN1Value::BooleanValue(true)),
    //     };
    //     let value: ASN1Value = ASN1Value::TaggedValue(val);
    //     let result = crate::ber_encode(&mut output, &value).unwrap();
    //     let output: Bytes = output.into_inner().into();
    //     assert_eq!(result, output.len());
    //     assert_eq!(result, 5);
    // }

    // #[test]
    // fn test_ber_encode_deep_tagging_1() {
    //     let inner_val = TaggedASN1Value {
    //         tag: Tag::new(TagClass::CONTEXT, 7),
    //         explicit: true,
    //         value: Arc::from(ASN1Value::BooleanValue(false)),
    //     };
    //     let outer_val = TaggedASN1Value {
    //         tag: Tag::new(TagClass::APPLICATION, 5),
    //         explicit: true,
    //         value: Arc::from(ASN1Value::TaggedValue(inner_val)),
    //     };
    //     let mut output = BytesMut::new().writer();
    //     let value: ASN1Value = ASN1Value::TaggedValue(outer_val);
    //     let result = crate::ber_encode(&mut output, &value).unwrap();
    //     let output: Bytes = output.into_inner().into();
    //     assert_eq!(result, output.len());
    //     assert_eq!(result, 7);
    //     assert!(output.starts_with(&[
    //         X690_TAG_CLASS_APPLICATION
    //         | 0b0010_0000 // Constructed
    //         | 5,
    //         0x05, // Length = 5
    //         X690_TAG_CLASS_CONTEXT
    //         | 0b0010_0000 // Constructed
    //         | 7,
    //         0x03, // Length = 3
    //         UNIV_TAG_BOOLEAN as u8,
    //         0x01, // Length = 1
    //         0x00, // FALSE
    //     ]));
    // }

    // #[test]
    // fn test_ber_encode_deep_tagging_2() {
    //     let mut output = BytesMut::new().writer();
    //     let inner_val = TaggedASN1Value {
    //         tag: Tag::new(TagClass::CONTEXT, 7),
    //         explicit: false,
    //         value: Arc::new(ASN1Value::BooleanValue(false)),
    //     };
    //     let outer_val = TaggedASN1Value {
    //         tag: Tag::new(TagClass::APPLICATION, 5),
    //         explicit: false,
    //         value: Arc::new(ASN1Value::TaggedValue(inner_val)),
    //     };
    //     let value: ASN1Value = ASN1Value::TaggedValue(outer_val);
    //     let result = crate::ber_encode(&mut output, &value).unwrap();
    //     let output: Bytes = output.into_inner().into();
    //     assert_eq!(result, output.len());
    //     assert_eq!(result, 3);
    //     assert!(output.starts_with(&[
    //         X690_TAG_CLASS_APPLICATION
    //         | 0b0000_0000 // Primitive
    //         | 5,
    //         0x01, // Length = 5
    //         0x00, // FALSE
    //     ]));
    // }

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
        match write_x690_node(&mut output, &asn1_data) {
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

    // #[test]
    // fn test_ber_encode_2() {
    //     let asn1_data = ASN1Value::SequenceValue(vec![
    //         ASN1Value::BooleanValue(true),
    //         ASN1Value::IntegerValue(vec![127]),
    //     ]);
    //     let mut output = Vec::new();
    //     match ber_encode(&mut output, &asn1_data) {
    //         Ok(bytes_written) => {
    //             assert_eq!(bytes_written, 8);
    //         }
    //         Err(e) => panic!("{}", e),
    //     }
    //     assert!(output.starts_with(&[
    //         X690_TAG_CLASS_UNIVERSAL
    //         | 0b0010_0000 // Constructed
    //         | UNIV_TAG_SEQUENCE as u8,
    //         0x06,
    //         0x01,
    //         0x01,
    //         0xFF,
    //         0x02,
    //         0x01,
    //         0x7F,
    //     ]));
    // }

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
}

// TODO: Tests to verify that my error handling from nested matches actually works.
