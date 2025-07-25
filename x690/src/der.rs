//! This module implements the Basic Encoding Rules (DER) for X.690.
use wildboar_asn1::{
    ASN1Error, ASN1ErrorCode, ASN1Result, Tag
};
use crate::codec::{DistinguishedEncodingRules, X690Codec};
use std::io::{Write, Result};
use crate::{
    X690Element,
    X690Value,
    X690Length,
    x690_write_bit_string_value,
    x690_write_string_value,
    x690_write_tlv,
    X690_REAL_BASE_MASK,
    X690_REAL_BINARY_SCALING_MASK,
    X690_REAL_EXPONENT_FORMAT_MASK,
    X690_SPECIAL_REAL_NOT_A_NUMBER,
    X690_SPECIAL_REAL_PLUS_INFINITY,
    X690_SPECIAL_REAL_MINUS_INFINITY,
    X690_SPECIAL_REAL_MINUS_ZERO,
    x690_write_duration_value,
    x690_write_real_value,
    x690_write_object_descriptor_value,
    x690_write_utf8_string_value,
    x690_write_octet_string_value,
    x690_write_generalized_time_value,
    x690_write_utc_time_value,
    x690_write_universal_string_value,
    x690_write_bmp_string_value,
    x690_decode_tag,
};
use wildboar_asn1::{
    TagClass,
    ASN1Value,
    BMPString,
    ByteSlice,
    GeneralString,
    GeneralizedTime,
    GraphicString,
    IA5String,
    NumericString,
    ObjectDescriptor,
    PrintableString,
    T61String,
    UTCTime,
    UTF8String,
    UniversalString,
    VideotexString,
    VisibleString,
    BIT_STRING,
    BOOLEAN,
    DURATION,
    OCTET_STRING,
    REAL,
    SEQUENCE,
    SET,
    UNIV_TAG_BIT_STRING,
    UNIV_TAG_BMP_STRING,
    UNIV_TAG_BOOLEAN,
    UNIV_TAG_CHARACTER_STRING,
    UNIV_TAG_DATE,
    UNIV_TAG_DATE_TIME,
    UNIV_TAG_DURATION,
    UNIV_TAG_EMBEDDED_PDV,
    UNIV_TAG_END_OF_CONTENT,
    UNIV_TAG_ENUMERATED,
    UNIV_TAG_EXTERNAL,
    UNIV_TAG_GENERAL_STRING,
    UNIV_TAG_GRAPHIC_STRING,
    UNIV_TAG_IA5_STRING,
    UNIV_TAG_INTEGER,
    UNIV_TAG_NULL,
    UNIV_TAG_NUMERIC_STRING,
    UNIV_TAG_OBJECT_DESCRIPTOR,
    UNIV_TAG_OBJECT_IDENTIFIER,
    UNIV_TAG_OCTET_STRING,
    UNIV_TAG_OID_IRI,
    UNIV_TAG_PRINTABLE_STRING,
    UNIV_TAG_REAL,
    UNIV_TAG_RELATIVE_OID,
    UNIV_TAG_RELATIVE_OID_IRI,
    UNIV_TAG_SEQUENCE,
    UNIV_TAG_SET,
    UNIV_TAG_T61_STRING,
    UNIV_TAG_TIME,
    UNIV_TAG_TIME_OF_DAY,
    UNIV_TAG_UNIVERSAL_STRING,
    UNIV_TAG_UTF8_STRING,
    UNIV_TAG_VIDEOTEX_STRING,
    UNIV_TAG_VISIBLE_STRING,
    UNIV_TAG_UTC_TIME,
    UNIV_TAG_GENERALIZED_TIME,
};
use bytes::{Bytes, BytesMut, BufMut};
use std::mem::size_of;
use std::sync::Arc;
#[cfg(feature = "simdutf8")]
use simdutf8::basic::from_utf8;
use std::str::FromStr;
use crate::utils::{vec_u16_to_vec_u8, vec_u32_to_vec_u8, get_days_in_month, unlikely, x690_read_var_length_u64, primitive};

pub const DER: DistinguishedEncodingRules = DistinguishedEncodingRules::new();

/// Decode the length of an X.690-encoded element from a byte slice
///
/// This starts at the first byte.
pub const fn der_decode_length(bytes: ByteSlice) -> ASN1Result<(usize, X690Length)> {
    if bytes.len() == 0 {
        // Truncated.
        return Err(ASN1Error::new(ASN1ErrorCode::tlv_truncated));
    }
    if bytes[0] < 0b1000_0000 {
        // Equivalent to ((b[0] & 0b1000_0000) == 0)
        return Ok((1, X690Length::Definite(bytes[0] as usize)));
    }
    if bytes[0] == 0b1000_0000 {
        return Err(ASN1Error::new(ASN1ErrorCode::indefinite_length_prohibited));
    }
    // Otherwise, it is long definite form.
    let length_length = (bytes[0] & 0b0111_1111) as usize;
    if length_length > size_of::<usize>() {
        // Length too big.
        return Err(ASN1Error::new(ASN1ErrorCode::length_too_big));
    }
    if (bytes.len() - 1) < length_length {
        // Insufficient bytes to read the length.
        return Err(ASN1Error::new(ASN1ErrorCode::tlv_truncated));
    }
    let bytes_read = 1 + length_length;
    let len: usize = match length_length {
        1 => bytes[1] as usize,
        2 => u16::from_be_bytes([bytes[1], bytes[2]]) as usize,
        3 => u32::from_be_bytes([0, bytes[1], bytes[2], bytes[3]]) as usize,
        4 => u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as usize,
        5 => {
            u64::from_be_bytes([0, 0, 0, bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]]) as usize
        }
        6 => u64::from_be_bytes([
            0, 0, bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
        ]) as usize,
        7 => u64::from_be_bytes([
            0, bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]) as usize,
        8 => u64::from_be_bytes([
            bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8],
        ]) as usize,
        _ => 0, // This should never happen.
    };
    Ok((bytes_read, X690Length::Definite(len)))
}

impl X690Codec for DistinguishedEncodingRules {

    fn decode_from_slice(&self, bytes: wildboar_asn1::ByteSlice) -> ASN1Result<(usize, X690Element)> {
        let (len, tag, constructed) = x690_decode_tag(bytes)?;
        let mut bytes_read: usize = len;
        let value_length;
        let (len_len, len) = der_decode_length(&bytes[bytes_read..])?;
        bytes_read += len_len;
        value_length = len;
        match value_length {
            X690Length::Definite(len) => {
                if (bytes.len() - bytes_read) < len {
                    let mut err = ASN1Error::new(ASN1ErrorCode::tlv_truncated);
                    err.tag = Some(Tag::new(tag.tag_class, tag.tag_number));
                    err.length = Some(len);
                    return Err(err);
                }
                if !constructed {
                    let el = X690Element::new(
                        tag,
                        X690Value::Primitive(Bytes::copy_from_slice(&bytes[bytes_read..bytes_read + len])),
                    );
                    bytes_read += len;
                    return Ok((bytes_read, el));
                }
                /* This is a small optimization. The smallest an X.690-encoded ASN.1
                value can be is two bytes. Therefore, the most elements that could
                possibly be encoded is (value_length / 2). (>> 1 has the effect of
                dividing by two, but more efficiently. */
                let estimated_children_count = len >> 1;
                let mut children: Vec<X690Element> = Vec::with_capacity(estimated_children_count);
                // let mut value_bytes_read: usize = bytes_read;
                let end_of_tag_and_length = bytes_read;
                while bytes_read < (end_of_tag_and_length + len) {
                    let (el_len, el) = self.decode_from_slice(&bytes[bytes_read..])?;
                    if el_len == 0 {
                        break;
                    }
                    bytes_read += el_len;
                    children.push(el);
                }
                let el = X690Element::new(
                    tag,
                    X690Value::Constructed(Arc::new(children)),
                );
                Ok((bytes_read, el))
            },
            X690Length::Indefinite => Err(ASN1Error::new(ASN1ErrorCode::indefinite_length_prohibited)),
        }
    }

    fn decode_from_bytes(&self, bytes: Bytes) -> ASN1Result<(usize, X690Element)> {
        let (len, tag, constructed) = x690_decode_tag(&bytes)?;
        let mut bytes_read: usize = len;
        let value_length;
        let (len_len, len) = der_decode_length(&bytes[bytes_read..])?;
        bytes_read += len_len;
        value_length = len;
        match value_length {
            X690Length::Definite(len) => {
                if (bytes.len() - bytes_read) < len {
                    let mut err = ASN1Error::new(ASN1ErrorCode::tlv_truncated);
                    err.tag = Some(Tag::new(tag.tag_class, tag.tag_number));
                    err.length = Some(len);
                    return Err(err);
                }
                if !constructed {
                    let el = X690Element::new(
                        tag,
                        X690Value::Primitive(bytes.slice(bytes_read..bytes_read + len)),
                    );
                    bytes_read += len;
                    return Ok((bytes_read, el));
                }
                /* This is a small optimization. The smallest an X.690-encoded ASN.1
                value can be is two bytes. Therefore, the most elements that could
                possibly be encoded is (value_length / 2). (>> 1 has the effect of
                dividing by two, but more efficiently. */
                let estimated_children_count = len >> 1;
                let mut children: Vec<X690Element> = Vec::with_capacity(estimated_children_count);
                // let mut value_bytes_read: usize = bytes_read;
                let end_of_tag_and_length = bytes_read;
                while bytes_read < (end_of_tag_and_length + len) {
                    let (el_len, el) = self.decode_from_bytes(bytes.slice(bytes_read..))?;
                    if el_len == 0 {
                        break;
                    }
                    bytes_read += el_len;
                    children.push(el);
                }
                let el = X690Element::new(
                    tag,
                    X690Value::Constructed(Arc::new(children)),
                );
                Ok((bytes_read, el))
            },
            X690Length::Indefinite => Err(ASN1Error::new(ASN1ErrorCode::indefinite_length_prohibited)),
        }
    }

    #[inline]
    fn write<W>(&self, output: &mut W, el: &X690Element) -> Result<usize> where W: Write {
        x690_write_tlv(output, el)
    }

    fn decode_boolean_value(&self, value_bytes: ByteSlice) -> ASN1Result<BOOLEAN> {
        if value_bytes.len() != 1 {
            let err = ASN1Error::new(ASN1ErrorCode::x690_boolean_not_one_byte);
            return Err(err);
        }
        if value_bytes[0] != 0 && value_bytes[0] != 0xFF {
            return Err(ASN1Error::new(ASN1ErrorCode::der_boolean_not_0x00_or_0xFF));
        }
        Ok(value_bytes[0] > 0)
    }

    fn decode_bit_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<BIT_STRING> {
        let len = value_bytes.len();
        if len < 1 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }

        let trailing_bits = value_bytes[0];
        if trailing_bits > 7 {
            return Err(ASN1Error::new(
                ASN1ErrorCode::x690_bit_string_remainder_gt_7,
            ));
        }
        if unlikely(len == 1) {
            return Ok(BIT_STRING::new());
        }
        if trailing_bits != 0 {
            let last_byte = *value_bytes.last().unwrap();
            let mask = 0b1111_1111u8 >> (8 - trailing_bits);
            if last_byte & mask != 0 {
                return Err(ASN1Error::new(ASN1ErrorCode::bit_string_trailing_bits_set));
            }
        }
        Ok(BIT_STRING::from_parts_borrowed(&value_bytes[1..], trailing_bits))
    }

    #[inline]
    fn decode_octet_string_value(&self, value_bytes: ByteSlice) -> ASN1Result<OCTET_STRING> {
        Ok(Vec::from(value_bytes))
    }

    fn decode_real_value(&self, value_bytes: ByteSlice) -> ASN1Result<REAL> {
        if value_bytes.len() == 0 {
            return Ok(0.000000);
        }
        match value_bytes[0] & 0b1100_0000 {
            crate::X690_REAL_SPECIAL => match value_bytes[0] & 0b0011_1111 {
                X690_SPECIAL_REAL_PLUS_INFINITY => return Ok(f64::INFINITY),
                X690_SPECIAL_REAL_MINUS_INFINITY => return Ok(f64::NEG_INFINITY),
                X690_SPECIAL_REAL_NOT_A_NUMBER => return Ok(f64::NAN),
                X690_SPECIAL_REAL_MINUS_ZERO => return Ok(-0.000000),
                _ => return Err(ASN1Error::new(ASN1ErrorCode::unrecognized_special_real)),
            },
            crate::X690_REAL_BASE10 => {
                let base10_format = value_bytes[0] & 0b0011_1111;
                if base10_format != crate::X690_REAL_NR3 {
                    return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_unrecognized_format(base10_format)));
                }
                if value_bytes.len() == 1 {
                    return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
                }
                if !value_bytes[1..].is_ascii() {
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                let s = unsafe {
                    std::str::from_utf8_unchecked(&value_bytes[1..])
                };
                for c in s.chars() {
                    if c == ' ' || c == 'e' || c == ',' {
                        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                    }
                }
                if s.starts_with('+') || s.starts_with('0') || s.contains("0E") {
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                if s.contains("+0") && !s.ends_with("+0") {
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                return iso6093::parse_nr3(&s)
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value));
            }
            _ => {
                // Binary encoding
                let negative = (value_bytes[0] & 0b0100_0000) > 0;
                let base_byte = value_bytes[0] & X690_REAL_BASE_MASK;
                let base: u8 = match base_byte {
                    crate::X690_REAL_BASE_2 => 2,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_unrecognized_base(base_byte))),
                };
                let scale: u8 = (value_bytes[0] & X690_REAL_BINARY_SCALING_MASK)
                    .overflowing_shr(2)
                    .0;
                if scale > 0 {
                    return Err(ASN1Error::new(ASN1ErrorCode::real_format_prohibited));
                }
                let mantissa: u64;
                let exponent: i32;
                match value_bytes[0] & X690_REAL_EXPONENT_FORMAT_MASK {
                    crate::X690_REAL_EXPONENT_FORMAT_1_OCTET => {
                        if value_bytes.len() < 3 {
                            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                        }
                        exponent = value_bytes[1] as i8 as i32;
                        mantissa = x690_read_var_length_u64(&value_bytes[2..])
                            .ok_or(ASN1Error::new(ASN1ErrorCode::value_too_big))?;
                    }
                    crate::X690_REAL_EXPONENT_FORMAT_2_OCTET => {
                        if value_bytes.len() < 4 {
                            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                        }
                        if value_bytes.len() > 4 + 6 {
                            // Mantissa too big..
                            return Err(ASN1Error::new(ASN1ErrorCode::field_too_big));
                        }
                        exponent = i32::from_be_bytes([0, 0, value_bytes[1], value_bytes[2]]);
                        if exponent > -128 && exponent < 128 {
                            // Could have been encoded using the 1-octet format.
                            return Err(ASN1Error::new(ASN1ErrorCode::real_format_prohibited));
                        }
                        mantissa = x690_read_var_length_u64(&value_bytes[3..])
                            .ok_or(ASN1Error::new(ASN1ErrorCode::value_too_big))?;
                    }
                    // Any of these variants are either (1) not encoding on the
                    // fewest bytes, or (2) encoding a value that is too large
                    // to decode into an IEEE 754 double.
                    crate::X690_REAL_EXPONENT_FORMAT_3_OCTET
                    | crate::X690_REAL_EXPONENT_FORMAT_VAR_OCTET => return Err(ASN1Error::new(ASN1ErrorCode::value_too_big)),
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::bin_real_unrecognized_exp_fmt))
                }
                if mantissa != 0 && ((mantissa % 2) == 0) {
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                let unsigned_value = (mantissa as f64)
                    * (2u8.pow(scale.into())) as f64
                    * ((base as f64).powi(exponent)) as f64;
                if negative {
                    return Ok(-1.0 * unsigned_value);
                } else {
                    return Ok(unsigned_value);
                }
            }
        }
    }

    #[inline]
    fn decode_utc_time_value(&self, value_bytes: ByteSlice) -> ASN1Result<UTCTime> {
        if value_bytes.len() != 13 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if !value_bytes.last().is_some_and(|b| *b == b'Z') {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        UTCTime::try_from(value_bytes)
    }

    #[inline]
    fn decode_generalized_time_value(&self, value_bytes: ByteSlice) -> ASN1Result<GeneralizedTime> {
        if value_bytes.len() < 15 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        if !value_bytes.last().is_some_and(|b| *b == b'Z') {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        if value_bytes.contains(&b',') {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        GeneralizedTime::try_from(value_bytes)
    }

    #[inline]
    fn decode_duration_value(&self, value_bytes: ByteSlice) -> ASN1Result<DURATION> {
        DURATION::try_from(value_bytes)
    }

    #[inline]
    fn decode_bit_string(&self, el: &X690Element) -> ASN1Result<BIT_STRING> {
        self.decode_bit_string_value(primitive(&el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_octet_string(&self, el: &X690Element) -> ASN1Result<OCTET_STRING> {
        Ok(primitive(el)?.into_owned())
            .map_err(|e: ASN1Error| el.to_asn1_error(e.error_code))
    }

    fn decode_sequence(&self, el: &X690Element) -> ASN1Result<SEQUENCE> {
        match &el.value {
            X690Value::Constructed(children) => {
                let mut ret: Vec<ASN1Value> = Vec::with_capacity(children.len());
                for child in children.iter() {
                    ret.push(self.decode_any(child)
                        .map_err(|e| child.to_asn1_error(e.error_code))?);
                }
                Ok(ret)
            },
            X690Value::Serialized(v) => {
                let (_, el) = DER.decode_from_slice(&v)?;
                self.decode_sequence(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    fn decode_set(&self, el: &X690Element) -> ASN1Result<SET> {
        match &el.value {
            X690Value::Constructed(children) => {
                let mut ret: Vec<ASN1Value> = Vec::with_capacity(children.len());
                for child in children.iter() {
                    ret.push(self.decode_any(child)
                        .map_err(|e| child.to_asn1_error(e.error_code))?);
                }
                Ok(ret)
            },
            X690Value::Serialized(v) => {
                let (_, el) = DER.decode_from_slice(&v)?;
                self.decode_set(&el)
            },
            _ => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
        }
    }

    #[inline]
    fn decode_object_descriptor(&self, el: &X690Element) -> ASN1Result<ObjectDescriptor> {
        self.decode_graphic_string_value(primitive(el)?.as_ref())
    }

    #[inline]
    fn decode_utf8_string(&self, el: &X690Element) -> ASN1Result<UTF8String> {
        #[cfg(not(feature = "simdutf8"))]
        return String::from_utf8(primitive(el)?.into_owned())
            .map_err(|e| el.to_asn1_error(ASN1ErrorCode::invalid_utf8(Some(e.utf8_error()))));
        #[cfg(feature = "simdutf8")]
        return from_utf8(primitive(el)?.as_ref())
            .map(|s| s.to_owned())
            .map_err(|_| el.to_asn1_error(ASN1ErrorCode::invalid_utf8(None)));
    }

    #[inline]
    fn decode_numeric_string(&self, el: &X690Element) -> ASN1Result<NumericString> {
        self.decode_numeric_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_printable_string(&self, el: &X690Element) -> ASN1Result<PrintableString> {
        self.decode_printable_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_t61_string(&self, el: &X690Element) -> ASN1Result<T61String> {
        Ok(primitive(el)?.into_owned())
            .map_err(|e: ASN1Error| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_videotex_string(&self, el: &X690Element) -> ASN1Result<VideotexString> {
        Ok(primitive(el)?.into_owned())
            .map_err(|e: ASN1Error| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_ia5_string(&self, el: &X690Element) -> ASN1Result<IA5String> {
        self.decode_ia5_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_utc_time(&self, el: &X690Element) -> ASN1Result<UTCTime> {
        self.decode_utc_time_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_generalized_time(&self, el: &X690Element) -> ASN1Result<GeneralizedTime> {
        self.decode_generalized_time_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_graphic_string(&self, el: &X690Element) -> ASN1Result<GraphicString> {
        self.decode_graphic_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_visible_string(&self, el: &X690Element) -> ASN1Result<VisibleString> {
        self.decode_visible_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_general_string(&self, el: &X690Element) -> ASN1Result<GeneralString> {
        self.decode_general_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_universal_string(&self, el: &X690Element) -> ASN1Result<UniversalString> {
        self.decode_universal_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn decode_bmp_string(&self, el: &X690Element) -> ASN1Result<BMPString> {
        self.decode_bmp_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    fn decode_any(&self, el: &X690Element) -> ASN1Result<ASN1Value> {
        if el.tag.tag_class != TagClass::UNIVERSAL {
            return match &el.value {
                X690Value::Primitive(bytes) => Ok(ASN1Value::UnknownBytes(Arc::new(bytes.to_vec()))),
                X690Value::Constructed(components) => {
                    let mut values: Vec<ASN1Value> = Vec::with_capacity(components.len());
                    for child in components.iter() {
                        values.push(self.decode_any(&child)
                            .map_err(|e| child.to_asn1_error(e.error_code))?);
                    }
                    return Ok(ASN1Value::SequenceValue(values));
                },
                X690Value::Serialized(v) => {
                    let (_, el) = DER.decode_from_slice(&v)?;
                    self.decode_any(&el)
                }
            };
        }

        let result = match el.tag.tag_number {
            UNIV_TAG_END_OF_CONTENT => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
            UNIV_TAG_BOOLEAN => Ok(ASN1Value::BooleanValue(self.decode_boolean(el)?)),
            UNIV_TAG_INTEGER => Ok(ASN1Value::IntegerValue(self.decode_integer(el)?)),
            UNIV_TAG_BIT_STRING => Ok(ASN1Value::BitStringValue(self.decode_bit_string(el)?)),
            UNIV_TAG_OCTET_STRING => Ok(ASN1Value::OctetStringValue(self.decode_octet_string(el)?)),
            UNIV_TAG_NULL => Ok(ASN1Value::NullValue),
            UNIV_TAG_OBJECT_IDENTIFIER => Ok(ASN1Value::ObjectIdentifierValue(self.decode_object_identifier(el)?)),
            UNIV_TAG_OBJECT_DESCRIPTOR => Ok(ASN1Value::ObjectDescriptor(self.decode_object_descriptor(el)?)),
            UNIV_TAG_EXTERNAL => Ok(ASN1Value::ExternalValue(self.decode_external(el)?)),
            UNIV_TAG_REAL => Ok(ASN1Value::RealValue(self.decode_real(el)?)),
            UNIV_TAG_ENUMERATED => Ok(ASN1Value::EnumeratedValue(self.decode_enumerated(el)?)),
            UNIV_TAG_EMBEDDED_PDV => Ok(ASN1Value::EmbeddedPDVValue(self.decode_embedded_pdv(el)?)),
            UNIV_TAG_UTF8_STRING => Ok(ASN1Value::UTF8String(self.decode_utf8_string(el)?)),
            UNIV_TAG_RELATIVE_OID => Ok(ASN1Value::RelativeOIDValue(self.decode_relative_oid(el)?)),
            UNIV_TAG_TIME => Ok(ASN1Value::TimeValue(self.decode_time(el)?)),
            // UNIV_TAG_RESERVED_15 => ()
            UNIV_TAG_SEQUENCE => Ok(ASN1Value::SequenceValue(self.decode_sequence(el)?)),
            // UNIV_TAG_SEQUENCE_OF => ()
            UNIV_TAG_SET => Ok(ASN1Value::SetValue(self.decode_set(el)?)),
            // UNIV_TAG_SET_OF => ()
            UNIV_TAG_NUMERIC_STRING => Ok(ASN1Value::NumericString(self.decode_numeric_string(el)?)),
            UNIV_TAG_PRINTABLE_STRING => Ok(ASN1Value::PrintableString(self.decode_printable_string(el)?)),
            UNIV_TAG_T61_STRING => Ok(ASN1Value::T61String(self.decode_t61_string(el)?)),
            UNIV_TAG_VIDEOTEX_STRING => Ok(ASN1Value::VideotexString(self.decode_videotex_string(el)?)),
            UNIV_TAG_IA5_STRING => Ok(ASN1Value::IA5String(self.decode_ia5_string(el)?)),
            UNIV_TAG_UTC_TIME => {
                Ok(ASN1Value::UTCTime(UTCTime::try_from(primitive(el)?.as_ref())?))
            },
            UNIV_TAG_GENERALIZED_TIME => {
                Ok(ASN1Value::GeneralizedTime(GeneralizedTime::try_from(primitive(el)?.as_ref())?))
            },
            UNIV_TAG_GRAPHIC_STRING => Ok(ASN1Value::GraphicString(self.decode_graphic_string(el)?)),
            UNIV_TAG_VISIBLE_STRING => Ok(ASN1Value::VisibleString(self.decode_visible_string(el)?)),
            UNIV_TAG_GENERAL_STRING => Ok(ASN1Value::GeneralString(self.decode_general_string(el)?)),
            UNIV_TAG_UNIVERSAL_STRING => Ok(ASN1Value::UniversalString(self.decode_universal_string(el)?)),
            UNIV_TAG_CHARACTER_STRING => Ok(ASN1Value::UnrestrictedCharacterStringValue(self.decode_character_string(el)?)),
            UNIV_TAG_BMP_STRING => Ok(ASN1Value::BMPString(self.decode_bmp_string(el)?)),
            UNIV_TAG_DATE => Ok(ASN1Value::DATE(self.decode_date(el)?)),
            UNIV_TAG_TIME_OF_DAY => Ok(ASN1Value::TIME_OF_DAY(self.decode_time_of_day(el)?)),
            UNIV_TAG_DATE_TIME => Ok(ASN1Value::DATE_TIME(self.decode_date_time(el)?)),
            UNIV_TAG_DURATION => Ok(ASN1Value::DURATION(self.decode_duration(el)?)),
            UNIV_TAG_OID_IRI => Ok(ASN1Value::IRIValue(self.decode_oid_iri(el)?)),
            UNIV_TAG_RELATIVE_OID_IRI => Ok(ASN1Value::RelativeIRIValue(self.decode_relative_oid_iri(el)?)),
            _ => Err(el.to_asn1_error(ASN1ErrorCode::unrecognized_universal_type)),
        };
        result.map_err(|e| el.to_asn1_error(e.error_code))
    }

    fn encode_any(&self, value: &ASN1Value) -> ASN1Result<X690Element> {
        match value {
            ASN1Value::BooleanValue(v) => self.encode_boolean(v),
            ASN1Value::IntegerValue(v) => self.encode_integer(v),
            ASN1Value::BitStringValue(v) => self.encode_bit_string(v),
            ASN1Value::OctetStringValue(v) => self.encode_octet_string(v),
            ASN1Value::NullValue => self.encode_null(&()),
            ASN1Value::ObjectIdentifierValue(v) => self.encode_object_identifier(v),
            ASN1Value::ObjectDescriptor(v) => self.encode_object_descriptor(v),
            ASN1Value::ExternalValue(v) => self.encode_external(v),
            ASN1Value::RealValue(v) => self.encode_real(v),
            ASN1Value::EnumeratedValue(v) => self.encode_enumerated(v),
            ASN1Value::EmbeddedPDVValue(v) => self.encode_embedded_pdv(v),
            ASN1Value::UTF8String(v) => self.encode_utf8_string(v),
            ASN1Value::RelativeOIDValue(v) => self.encode_relative_oid(v),
            ASN1Value::NumericString(v) => self.encode_numeric_string(v),
            ASN1Value::PrintableString(v) => self.encode_printable_string(v),
            ASN1Value::T61String(v) => self.encode_t61_string(v),
            ASN1Value::VideotexString(v) => self.encode_videotex_string(v),
            ASN1Value::IA5String(v) => self.encode_ia5_string(v),
            ASN1Value::UTCTime(v) => self.encode_utc_time(v),
            ASN1Value::GeneralizedTime(v) => self.encode_generalized_time(v),
            ASN1Value::GraphicString(v) => self.encode_graphic_string(v),
            ASN1Value::VisibleString(v) => self.encode_visible_string(v),
            ASN1Value::GeneralString(v) => self.encode_general_string(v),
            ASN1Value::UniversalString(v) => self.encode_universal_string(&v.0),
            ASN1Value::UnrestrictedCharacterStringValue(v) => {
                DER.encode_character_string(v)
            },
            ASN1Value::BMPString(v) => self.encode_bmp_string(&v.0),
            ASN1Value::IRIValue(v) => self.encode_oid_iri(v),
            ASN1Value::RelativeIRIValue(v) => self.encode_relative_oid_iri(v),
            ASN1Value::TimeValue(v) => self.encode_time(v),
            ASN1Value::UnknownBytes(_) => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
            ASN1Value::ChoiceValue(v) => DER.encode_any(v),
            ASN1Value::ISO646String(v) => self.encode_ia5_string(v),
            ASN1Value::TeletexString(v) => self.encode_t61_string(v),
            ASN1Value::DATE(v) => self.encode_date(v),
            ASN1Value::TIME_OF_DAY(v) => self.encode_time_of_day(v),
            ASN1Value::DATE_TIME(v) => self.encode_date_time(v),
            ASN1Value::DURATION(v) => self.encode_duration(v),
            ASN1Value::TaggedValue(v) => DER.encode_any(&v.value),
            ASN1Value::SequenceValue(v)
            | ASN1Value::SequenceOfValue(v) => {
                let mut components: Vec<X690Element> = Vec::with_capacity(v.len());
                for subv in v {
                    let el = DER.encode_any(&subv)?;
                    components.push(el);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
                    X690Value::Constructed(Arc::new(components)),
                ))
            },
            ASN1Value::SetValue(v)
            | ASN1Value::SetOfValue(v) => {
                let mut components: Vec<X690Element> = Vec::with_capacity(v.len());
                for subv in v {
                    let el = DER.encode_any(&subv)?;
                    components.push(el);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET),
                    X690Value::Constructed(Arc::new(components)),
                ))
            },
            ASN1Value::InstanceOfValue(v) => DER.encode_instance_of(&v),
        }
    }

    fn encode_bit_string(&self, value: &BIT_STRING) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len_in_bytes() + 1).writer();
        x690_write_bit_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_BIT_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    #[inline]
    fn encode_octet_string(&self, value: &OCTET_STRING) -> ASN1Result<X690Element> {
        // Slight optimization to skip all this.
        // let mut out: Bytes = Vec::with_capacity(value.len());
        // x690_write_octet_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OCTET_STRING),
            X690Value::Primitive(Bytes::copy_from_slice(value)),
        ))
    }

    #[inline]
    fn encode_real(&self, value: &REAL) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(32).writer(); // This should cover most values.
        x690_write_real_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_REAL),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_object_descriptor(&self, value: &str) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_object_descriptor_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OBJECT_DESCRIPTOR),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_utf8_string(&self, value: &str) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        let bytes_written = x690_write_utf8_string_value(&mut out, &value)?;
        // value.len() is in bytes, not characters, so this allocation should
        // be 100% accurate.
        debug_assert_eq!(bytes_written, value.len());
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UTF8_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_numeric_string(&self, value: &str) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_NUMERIC_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_printable_string(&self, value: &str) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_PRINTABLE_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_t61_string(&self, value: &T61String) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_octet_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_T61_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    // FIXME: Use &[u8]
    fn encode_videotex_string(&self, value: &VideotexString) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_octet_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_VIDEOTEX_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_ia5_string(&self, value: &str) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_IA5_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_utc_time(&self, value: &UTCTime) -> ASN1Result<X690Element> {
        if !value.utc_offset.is_zero() {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        let mut out = BytesMut::with_capacity(17).writer(); // This is the max length of a UTCTime.
        x690_write_utc_time_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UTC_TIME),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_generalized_time(&self, value: &GeneralizedTime) -> ASN1Result<X690Element> {
        // Ensure minutes and seconds are present, as required by DER.
        let mut v = value.clone();
        if let Some((min, maybe_sec)) = v.min_and_sec {
            if maybe_sec.is_none() {
                v.min_and_sec = Some((min, Some(0)))
            }
        } else {
            v.min_and_sec = Some((0, Some(0)));
        }

        // We just delete the fraction. Sorry.
        v.fraction = 0;
        v.flags = 0;

        let mut out = BytesMut::with_capacity(32).writer(); // There is no defined max length, but this is very generous capacity.
        x690_write_generalized_time_value(&mut out, &v)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_GENERALIZED_TIME),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_graphic_string(&self, value: &str) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_GRAPHIC_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_visible_string(&self, value: &str) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_VISIBLE_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_general_string(&self, value: &str) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len()).writer();
        x690_write_string_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_GENERAL_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    /// NOTE: This might not be faster on your system if it is little-endian.
    fn encode_owned_universal_string(&self, value: UniversalString) -> ASN1Result<X690Element> {
        let mut out = vec_u32_to_vec_u8(value.0);
        debug_assert_eq!(out.len() % 4, 0);
        if cfg!(target_endian = "little") {
            // Swap every quartet of bytes in `out` to convert from little-endian to big-endian
            for chunk in out.chunks_exact_mut(4) {
                chunk.swap(0, 3);
                chunk.swap(1, 2);
            }
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UNIVERSAL_STRING),
            X690Value::Primitive(out.into()),
        ))
    }

    /// NOTE: This might not be faster on your system if it is little-endian.
    fn encode_owned_bmp_string(&self, value: BMPString) -> ASN1Result<X690Element> {
        let mut out = vec_u16_to_vec_u8(value.0);
        debug_assert_eq!(out.len() % 2, 0);
        if cfg!(target_endian = "little") {
            // Swap every pair of bytes in `out` to convert from little-endian to big-endian
            for chunk in out.chunks_exact_mut(2) {
                chunk.swap(0, 1);
            }
        }
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_BMP_STRING),
            X690Value::Primitive(out.into()),
        ))
    }

    fn encode_universal_string<T: AsRef<[u32]>>(&self, value: &T) -> ASN1Result<X690Element> {
        let chars = value.as_ref();
        let mut out = BytesMut::with_capacity(chars.len() << 2).writer(); // Four bytes for every character
        x690_write_universal_string_value(&mut out, chars)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UNIVERSAL_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_bmp_string<T: AsRef<[u16]>>(&self, value: &T) -> ASN1Result<X690Element> {
        let chars = value.as_ref();
        let mut out = BytesMut::with_capacity(chars.len() << 1).writer(); // Two bytes for every character
        x690_write_bmp_string_value(&mut out, chars)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_BMP_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_duration(&self, value: &DURATION) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(16).writer(); // There is no guaranteed size, but 16 is a reasonable pre-allocation.
        x690_write_duration_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_DURATION),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    #[inline]
    fn validate_boolean_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() != 1 {
            return Err(ASN1Error::new(ASN1ErrorCode::x690_boolean_not_one_byte));
        }
        Ok(())
    }

    fn validate_bit_string_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() == 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        if content_octets[0] > 7 {
            return Err(ASN1Error::new(ASN1ErrorCode::x690_bit_string_remainder_gt_7));
        }
        if content_octets.len() == 1 && content_octets[0] > 7 {
            return Err(ASN1Error::new(ASN1ErrorCode::x690_bit_string_remainder_but_no_bits));
        }
        return Ok(());
    }

    fn validate_real_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() == 0 {
            return Ok(()); // +0.0
        }
        match content_octets[0] & 0b11000000 {
            crate::X690_REAL_SPECIAL => { // Special real formatting
                let special_real_value = content_octets[0] & 0b0000_0011;
                if (special_real_value != X690_SPECIAL_REAL_NOT_A_NUMBER)
                    && (special_real_value != X690_SPECIAL_REAL_PLUS_INFINITY)
                    && (special_real_value != X690_SPECIAL_REAL_MINUS_INFINITY)
                    && (special_real_value != X690_SPECIAL_REAL_MINUS_ZERO) {
                    return Err(ASN1Error::new(ASN1ErrorCode::unrecognized_special_real));
                }
            },
            crate::X690_REAL_BASE10 => { // Textual / Base-10 formatting
                let base10_format = content_octets[0] & 0b0011_1111;
                if base10_format != crate::X690_REAL_NR3 {
                    return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_unrecognized_format(base10_format)));
                }
                if content_octets.len() == 1 {
                    return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
                }
                if !content_octets[1..].is_ascii() {
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                let s = unsafe {
                    std::str::from_utf8_unchecked(&content_octets[1..])
                };
                for c in s.chars() {
                    if c == ' ' || c == 'e' || c == ',' {
                        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                    }
                }
                if s.starts_with('+') || s.starts_with('0') || s.contains("0E") {
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                if s.contains("+0") && !s.ends_with("+0") {
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                iso6093::parse_nr3(&s)
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::malformed_value))?;
            },
            _ => { // Binary formatting
                let base_byte = content_octets[0] & X690_REAL_BASE_MASK;
                match base_byte {
                    crate::X690_REAL_BASE_2 => 2,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_unrecognized_base(base_byte))),
                };
                let scale: u8 = (content_octets[0] & X690_REAL_BINARY_SCALING_MASK)
                    .overflowing_shr(2)
                    .0;
                if scale > 0 {
                    return Err(ASN1Error::new(ASN1ErrorCode::real_format_prohibited));
                }
                let mantissa: u64;
                match content_octets[0] & X690_REAL_EXPONENT_FORMAT_MASK {
                    crate::X690_REAL_EXPONENT_FORMAT_1_OCTET => {
                        if content_octets.len() < 3 {
                            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                        }
                        mantissa = x690_read_var_length_u64(&content_octets[2..])
                            .ok_or(ASN1Error::new(ASN1ErrorCode::value_too_big))?;
                    }
                    crate::X690_REAL_EXPONENT_FORMAT_2_OCTET => {
                        if content_octets.len() < 4 {
                            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                        }
                        if content_octets.len() > 4 + 6 { // Mantissa too big.
                            return Err(ASN1Error::new(ASN1ErrorCode::field_too_big));
                        }
                        let exponent = i32::from_be_bytes([0, 0, content_octets[1], content_octets[2]]);
                        if exponent > -128 && exponent < 128 { // Could have used the 1-octet format.
                            return Err(ASN1Error::new(ASN1ErrorCode::real_format_prohibited));
                        }
                        mantissa = x690_read_var_length_u64(&content_octets[3..])
                            .ok_or(ASN1Error::new(ASN1ErrorCode::value_too_big))?;
                    }
                    // Any of these variants are either (1) not encoding on the
                    // fewest bytes, or (2) encoding a value that is too large
                    // to decode into an IEEE 754 double.
                    crate::X690_REAL_EXPONENT_FORMAT_3_OCTET
                    | crate::X690_REAL_EXPONENT_FORMAT_VAR_OCTET => return Err(ASN1Error::new(ASN1ErrorCode::value_too_big)),
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::bin_real_unrecognized_exp_fmt))
                }
                if mantissa != 0 && ((mantissa % 2) == 0) {
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
            }
        };
        Ok(())
    }

    // 9604152030Z
    fn validate_utc_time_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() != 13 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if !content_octets.last().is_some_and(|b| *b == b'Z') {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        let maybe_bad_char = content_octets[0..12].iter().position(|b| !b.is_ascii_digit());
        if let Some(bad_char_index) = maybe_bad_char {
            let bad_char = content_octets[bad_char_index];
            return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                bad_char as u32,
                bad_char_index,
            )));
        }
        #[cfg(feature = "simdutf8")]
        let s = from_utf8(content_octets)
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_utf8(None)))?;
        #[cfg(not(feature = "simdutf8"))]
        let s = std::str::from_utf8(content_octets)
                .map_err(|e| ASN1Error::new(ASN1ErrorCode::invalid_utf8(Some(e))))?;
        let mut year = u16::from_str(&s[0..2])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
        if year > 75 { // I think this is specified in RFC 5280. I forgot where I saw it.
            year += 1900;
        } else {
            year += 2000;
        }
        // I confirmed in a unit test below that u8::from_str() will tolerate leading zeros.
        let month = u8::from_str(&s[2..4])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
        let day = u8::from_str(&s[4..6])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_day))?;
        let hour = u8::from_str(&s[6..8])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_hour))?;
        let minute = u8::from_str(&s[8..10])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
        let second = u8::from_str(&s[10..12])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_second))?;
        if month > 12 || month == 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
        }
        let max_day = get_days_in_month(year, month);
        if day == 0 || day > max_day {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
        }
        if hour > 23 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
        }
        if minute > 59 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
        }
        if second > 59 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
        }
        Ok(())
    }

    fn validate_generalized_time_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() < 15 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        #[cfg(feature = "simdutf8")]
        let s = from_utf8(content_octets)
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_utf8(None)))?;
        #[cfg(not(feature = "simdutf8"))]
        let s = std::str::from_utf8(content_octets)
            .map_err(|e| ASN1Error::new(ASN1ErrorCode::invalid_utf8(Some(e))))?;

        // Extract and validate date and time parts
        let year: u32 = s[..4].parse().map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_year))?;
        let month: u32 = s[4..6].parse().map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_month))?;
        let day: u32 = s[6..8].parse().map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_day))?;
        let hour: u32 = s[8..10].parse().map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_hour))?;
        let minute: u32 = s[10..12].parse().map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
        let second: u32 = s[12..14].parse().map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_second))?;

        if month == 0 || month > 12 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
        }

        if day == 0 || day > get_days_in_month(year as u16, month as u8) as u32 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
        }

        if hour >= 24 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
        }
        if minute >= 60 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
        }
        if second >= 60 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
        }

        // Check timezone or fractions of seconds
        match &s[14..] {
            "Z" => Ok(()),
            s if s.starts_with('.') => {
                let (fraction, tz) = s[1..].split_at(s[1..].find(|c: char| !c.is_numeric()).unwrap_or(0));
                if fraction.is_empty() || fraction.ends_with("0") {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_fraction_of_seconds));
                }
                match tz {
                    "Z" => Ok(()),
                    _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset)),
                }
            },
            _ => Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        }
    }

    // Before some tweaking, this was produced by ChatGPT.
    fn validate_duration_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() < 3 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        // The duration should NOT start with 'P' (ASCII 80)
        if content_octets[0] == b'P' {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        if content_octets.ends_with(b"T") {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }

        let mut idx = 0;
        let mut has_time_component = false;
        let mut encountered_fraction: bool = false;
        while idx < content_octets.len() {
            match content_octets[idx] {
                b'T' => {
                    has_time_component = true;
                    idx += 1;
                }
                b'Y' | b'M' | b'D' | b'H' | b'S' => {
                    // These bytes should be preceded by a number
                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                }
                _ if content_octets[idx].is_ascii_digit() => {
                    // Consume all consecutive digits
                    while idx < content_octets.len() && content_octets[idx].is_ascii_digit() {
                        idx += 1;
                    }

                    // After digits, expect one of the duration identifiers
                    if idx >= content_octets.len() {
                        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                    }

                    match content_octets[idx] {
                        b'Y' | b'M' if !has_time_component => idx += 1,
                        b'D' if !has_time_component => idx += 1,
                        b'H' | b'M' | b'S' if has_time_component => idx += 1,
                        b'.' => {
                            if encountered_fraction {
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                            idx += 1;
                            // Consume all consecutive digits
                            while idx < content_octets.len() && content_octets[idx].is_ascii_digit() {
                                idx += 1;
                            }
                            encountered_fraction = true;
                        }
                        _ => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
                    }
                }
                _ => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
            }
        }

        Ok(())
    }

    fn validate_bit_string(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_bit_string_value(&v)
                .map_err(|e| el.to_asn1_error(e.error_code)),
            X690Value::Constructed(_) => Err(el.to_asn1_error(ASN1ErrorCode::invalid_construction)),
            X690Value::Serialized(v) => {
                let (_, el) = DER.decode_from_slice(&v)?;
                self.validate_bit_string(&el)
                    .map_err(|e| el.to_asn1_error(e.error_code))
            }
        }
    }

    #[inline]
    fn validate_octet_string(&self, el: &X690Element) -> ASN1Result<()> {
        primitive(el)?;
        Ok(())
    }

    #[inline]
    fn validate_object_descriptor(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_graphic_string(el).map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_utf8_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_utf8_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_numeric_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_numeric_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_printable_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_printable_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_t61_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_t61_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_videotex_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_videotex_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_ia5_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_ia5_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_utc_time(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_utc_time_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_generalized_time(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_generalized_time_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_graphic_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_graphic_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_visible_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_visible_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_general_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_general_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_universal_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_universal_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

    #[inline]
    fn validate_bmp_string(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_bmp_string_value(primitive(el)?.as_ref())
            .map_err(|e| el.to_asn1_error(e.error_code))
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use super::X690Value;
    use crate::X690_TAG_CLASS_UNIVERSAL;
    use wildboar_asn1::{DATE, UTCTime, GeneralizedTime, DURATION, UTCOffset};

    #[test]
    fn test_der_decode_algorithm_identifier() {
        let encoded_data: Vec<u8> = vec![
            X690_TAG_CLASS_UNIVERSAL
            | 0b0010_0000 // Constructed
            | UNIV_TAG_SEQUENCE as u8,
            0x07,
            0x06,
            0x03,
            0x55,
            0x04,
            0x03,
            0x05,
            0x00,
        ];
        let (bytes_read, el) = match DER.decode_from_slice(encoded_data.as_slice()) {
            Err(_) => panic!("asdf"),
            Ok(result) => result,
        };
        assert_eq!(bytes_read, 9);
        assert_eq!(el.tag.tag_class, TagClass::UNIVERSAL);
        assert_eq!(el.tag.tag_number, UNIV_TAG_SEQUENCE);
        if let X690Value::Constructed(children) = &el.value {
            assert_eq!(children.len(), 2);
            assert_eq!(children[0].tag.tag_class, TagClass::UNIVERSAL);
            assert_eq!(
                children[0].tag.tag_number,
                UNIV_TAG_OBJECT_IDENTIFIER
            );
            assert_eq!(children[1].tag.tag_class, TagClass::UNIVERSAL);
            assert_eq!(children[1].tag.tag_number, UNIV_TAG_NULL);
            if let X690Value::Primitive(oid_bytes) = &children[0].value {
                let oid = match DER.decode_object_identifier_value(&oid_bytes) {
                    Err(_) => panic!("woriyjh"),
                    Ok(result) => result,
                };
                assert_eq!(oid.to_dot_delim_string(), "2.5.4.3");
                // let alg_id = AlgorithmIdentifier{
                //     algorithm: oid,
                //     parameters: Some(children[1]),
                // };
            } else {
                panic!("teuye");
            }
        } else {
            panic!("zxcv");
        }
    }

    #[test]
    fn test_der_decode_utc_time() {
        let time = "\x17\x0D991105223344Z";
        let value_bytes = Vec::from(time);
        let cst = match DER.decode_from_slice(&value_bytes) {
            Ok((_, el)) => el,
            Err(e) => panic!("{}", e),
        };
        if let X690Value::Primitive(bytes) = &cst.value {
            let utc_time = DER.decode_utc_time_value(&bytes);
            let decoded_value = match utc_time {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            };
            assert_eq!(decoded_value.year, 99);
            assert_eq!(decoded_value.month, 11);
            assert_eq!(decoded_value.day, 5);
            assert_eq!(decoded_value.hour, 22);
            assert_eq!(decoded_value.minute, 33);
            assert_eq!(decoded_value.second, 44);
            assert_eq!(decoded_value.utc_offset.hour, 0);
            assert_eq!(decoded_value.utc_offset.minute, 0);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_der_decode_length_short_form() {
        // Test short form (length < 128)
        let bytes = [0x05]; // Length 5
        let (bytes_read, length) = der_decode_length(&bytes).unwrap();
        assert_eq!(bytes_read, 1);
        assert_eq!(length, X690Length::Definite(5));
    }

    #[test]
    fn test_der_decode_length_indefinite() {
        let bytes = [0x80]; // Indefinite length
        assert!(der_decode_length(&bytes).is_err());
    }

    #[test]
    fn test_der_decode_length_long_form() {
        // Test long form with 1 byte length
        let bytes = [0x81, 0xFF]; // Length 255
        let (bytes_read, length) = der_decode_length(&bytes).unwrap();
        assert_eq!(bytes_read, 2);
        assert_eq!(length, X690Length::Definite(255));
    }

    #[test]
    fn test_der_decode_length_long_form_2_bytes() {
        // Test long form with 2 bytes length
        let bytes = [0x82, 0x01, 0x00]; // Length 256
        let (bytes_read, length) = der_decode_length(&bytes).unwrap();
        assert_eq!(bytes_read, 3);
        assert_eq!(length, X690Length::Definite(256));
    }

    #[test]
    fn test_der_decode_length_truncated() {
        // Test truncated length field
        let bytes: [u8; 0] = [];
        let result = der_decode_length(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_der_decode_boolean_value() {
        // Test true value
        let bytes = [0xFF];
        let result = DER.decode_boolean_value(&bytes).unwrap();
        assert_eq!(result, true);

        // Test false value
        let bytes = [0x00];
        let result = DER.decode_boolean_value(&bytes).unwrap();
        assert_eq!(result, false);

        // Test non-zero value (should be true)
        let bytes = [0x01];
        let result = DER.decode_boolean_value(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_der_decode_boolean_value_invalid() {
        // Test with wrong number of bytes
        let bytes: [u8; 0] = [];
        let result = DER.decode_boolean_value(&bytes);
        assert!(result.is_err());

        let bytes = [0x00, 0x01];
        let result = DER.decode_boolean_value(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_der_decode_bit_string_value() {
        // Test bit string with no trailing bits
        let bytes = [0x00, 0xFF, 0xFF];
        let result = DER.decode_bit_string_value(&bytes).unwrap();
        assert_eq!(result.get_trailing_bits_count(), 0);
        assert_eq!(result.get_bytes_ref(), &[0xFFu8, 0xFF]);

        // Test bit string with trailing bits
        let bytes = [0x03, 0xFF, 0xF8]; // 3 trailing bits
        let result = DER.decode_bit_string_value(&bytes).unwrap();
        assert_eq!(result.get_trailing_bits_count(), 3);
        assert_eq!(result.get_bytes_ref(), &[0xFFu8, 0xF8]);
    }

    #[test]
    fn test_der_decode_bit_string_value_invalid() {
        // Test empty bit string
        let bytes: [u8; 0] = [];
        let result = DER.decode_bit_string_value(&bytes);
        assert!(result.is_err());

        // Test invalid trailing bits (> 7)
        let bytes = [0x08, 0xFF];
        let result = DER.decode_bit_string_value(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_der_decode_octet_string_value() {
        let bytes = [0x01, 0x02, 0x03, 0x04];
        let result = DER.decode_octet_string_value(&bytes).unwrap();
        assert_eq!(result, vec![0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_der_decode_real_value_special() {
        // Test plus infinity
        let bytes = [0x40]; // Special real, plus infinity
        let result = DER.decode_real_value(&bytes).unwrap();
        assert!(result.is_infinite() && result > 0.0);

        // Test minus infinity
        let bytes = [0x41]; // Special real, minus infinity
        let result = DER.decode_real_value(&bytes).unwrap();
        assert!(result.is_infinite() && result < 0.0);

        // Test NaN
        let bytes = [0x42]; // Special real, NaN
        let result = DER.decode_real_value(&bytes).unwrap();
        assert!(result.is_nan());

        // Test minus zero
        let bytes = [0x43]; // Special real, minus zero
        let result = DER.decode_real_value(&bytes).unwrap();
        assert_eq!(result, -0.0);
    }

    #[test]
    fn test_der_decode_real_value_base10() {
        // Test NR1 format (integer)
        let bytes = [0x01, b'1', b'2', b'3']; // NR1 format, value "123"
        let result = DER.decode_real_value(&bytes);
        assert!(result.is_err());

        // Test NR2 format (decimal)
        let bytes = [0x02, b'1', b'2', b'3', b'.', b'4', b'5']; // NR2 format, value "123.45"
        let result = DER.decode_real_value(&bytes);
        assert!(result.is_err());

        // Test NR3 format (scientific notation)
        let bytes = [0x03, b'1', b'2', b'3', b'.', b'4', b'5', b'E', b'+', b'2']; // NR3 format, value "123.45E+2"
        let result = DER.decode_real_value(&bytes).unwrap();
        assert_eq!(result, 12345.0);
    }

    #[test]
    fn test_der_decode_real_value_binary() {
        // Test binary encoding with base 2
        let bytes = [0x80, 0x02, 0x01]; // Binary, base 2, exponent 2, mantissa 1
        let result = DER.decode_real_value(&bytes).unwrap();
        assert_eq!(result, 4.0); // 1 * 2^2 = 4
    }

    #[test]
    fn test_der_decode_real_value_empty() {
        // Test empty real value (should return 0.0)
        let bytes: [u8; 0] = [];
        let result = DER.decode_real_value(&bytes).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_der_decode_generalized_time_value() {
        let bytes = b"20231201120000Z";
        let result = DER.decode_generalized_time_value(bytes).unwrap();
        assert_eq!(result.date.year, 2023);
        assert_eq!(result.date.month, 12);
        assert_eq!(result.date.day, 1);
        assert_eq!(result.hour, 12);
        assert_eq!(result.min_and_sec, Some((0, Some(0))));
    }

    #[test]
    fn test_der_decode_duration_value() {
        let bytes = b"P1Y2M3DT4H5M6S";
        let result = DER.decode_duration_value(bytes).unwrap();
        assert_eq!(result.years,   1);
        assert_eq!(result.months,  2);
        assert_eq!(result.days,    3);
        assert_eq!(result.hours,   4);
        assert_eq!(result.minutes, 5);
        assert_eq!(result.seconds, 6);
    }

    #[test]
    fn test_der_encode_decode_boolean() {
        let value = true;
        let encoded = DER.encode_boolean(&value).unwrap();
        let decoded = DER.decode_boolean(&encoded).unwrap();
        assert_eq!(decoded, value);

        let value = false;
        let encoded = DER.encode_boolean(&value).unwrap();
        let decoded = DER.decode_boolean(&encoded).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_der_encode_decode_bit_string() {
        let value = BIT_STRING::from_parts_borrowed(&[0xFF, 0x00], 4);
        let encoded = DER.encode_bit_string(&value).unwrap();
        let decoded = DER.decode_bit_string(&encoded).unwrap();
        assert_eq!(decoded.get_bytes_ref(), value.get_bytes_ref());
        assert_eq!(decoded.get_trailing_bits_count(), value.get_trailing_bits_count());
    }

    #[test]
    fn test_der_encode_decode_octet_string() {
        let value = vec![0x01, 0x02, 0x03, 0x04];
        let encoded = DER.encode_octet_string(&value).unwrap();
        let decoded = DER.decode_octet_string(&encoded).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_der_encode_decode_real() {
        let value = 123.456;
        let encoded = DER.encode_real(&value).unwrap();
        let decoded = DER.decode_real(&encoded).unwrap();
        assert!((decoded - value).abs() < 0.001); // Allow small floating point differences

        let value = f64::INFINITY;
        let encoded = DER.encode_real(&value).unwrap();
        let decoded = DER.decode_real(&encoded).unwrap();
        assert!(decoded.is_infinite() && decoded > 0.0);

        let value = f64::NEG_INFINITY;
        let encoded = DER.encode_real(&value).unwrap();
        let decoded = DER.decode_real(&encoded).unwrap();
        assert!(decoded.is_infinite() && decoded < 0.0);
    }

    #[test]
    fn test_der_encode_decode_utf8_string() {
        let value = "Hello, 世界!";
        let encoded = DER.encode_utf8_string(value).unwrap();
        let decoded = DER.decode_utf8_string(&encoded).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_der_encode_decode_numeric_string() {
        let value = "1234567890";
        let encoded = DER.encode_numeric_string(value).unwrap();
        let decoded = DER.decode_numeric_string(&encoded).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_der_encode_decode_printable_string() {
        let value = "Hello World";
        let encoded = DER.encode_printable_string(value).unwrap();
        let decoded = DER.decode_printable_string(&encoded).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_der_encode_decode_ia5_string() {
        let value = "Hello World!";
        let encoded = DER.encode_ia5_string(value).unwrap();
        let decoded = DER.decode_ia5_string(&encoded).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_der_encode_decode_utc_time() {
        let value = UTCTime{
            year: 23,
            month: 12,
            day: 1,
            hour: 12,
            minute: 0,
            second: 0,
            utc_offset: UTCOffset{hour: 0, minute: 0},
        };
        let encoded = DER.encode_utc_time(&value).unwrap();
        let decoded = DER.decode_utc_time(&encoded).unwrap();
        assert_eq!(decoded.year, value.year);
        assert_eq!(decoded.month, value.month);
        assert_eq!(decoded.day, value.day);
        assert_eq!(decoded.hour, value.hour);
        assert_eq!(decoded.minute, value.minute);
        assert_eq!(decoded.second, value.second);
    }

    #[test]
    fn test_der_encode_decode_generalized_time_utc() {
        let value = GeneralizedTime{
            date: DATE{year: 2023, month: 12, day: 1},
            utc_offset: Some(UTCOffset::utc()),
            hour: 12,
            min_and_sec: Some((0, Some(0))),
            flags: 0,
            fraction: 0,
        };
        let encoded = DER.encode_generalized_time(&value).unwrap();
        let decoded = DER.decode_generalized_time(&encoded).unwrap();
        assert_eq!(decoded.date.year, value.date.year);
        assert_eq!(decoded.date.month, value.date.month);
        assert_eq!(decoded.date.day, value.date.day);
        assert_eq!(decoded.hour, value.hour);
        assert_eq!(decoded.min_and_sec, Some((0, Some(0))));
    }

    // This should fail because local time is not allowed.
    #[test]
    fn test_der_encode_decode_generalized_time_local() {
        let value = GeneralizedTime{
            date: DATE{year: 2023, month: 12, day: 1},
            utc_offset: None,
            hour: 12,
            min_and_sec: None,
            flags: 0,
            fraction: 0,
        };
        if let Ok(encoded) = DER.encode_generalized_time(&value) {
            let decoded = DER.decode_generalized_time(&encoded);
            assert!(decoded.is_err());
        }
    }

    #[test]
    fn test_der_encode_decode_universal_string() {
        let value = UniversalString(vec![0x0048, 0x0065, 0x006C, 0x006C, 0x006F]); // "Hello"
        let encoded = DER.encode_universal_string(&value.0).unwrap();
        let decoded = DER.decode_universal_string(&encoded).unwrap();
        assert_eq!(decoded.0, value.0);
    }

    #[test]
    fn test_der_encode_decode_bmp_string() {
        let value = BMPString(vec![0x0048, 0x0065, 0x006C, 0x006C, 0x006F]); // "Hello"
        let encoded = DER.encode_bmp_string(&value.0).unwrap();
        let decoded = DER.decode_bmp_string(&encoded).unwrap();
        assert_eq!(decoded.0, value.0);
    }

    #[test]
    fn test_der_encode_decode_duration() {
        let value = DURATION{
            years: 1,
            months: 2,
            days: 3,
            hours: 4,
            minutes: 5,
            seconds: 6,
            weeks: 0,
            fractional_part: None,
        };
        let encoded = DER.encode_duration(&value).unwrap();
        let decoded = DER.decode_duration(&encoded).unwrap();
        assert_eq!(decoded.years, value.years);
        assert_eq!(decoded.months, value.months);
        assert_eq!(decoded.days, value.days);
        assert_eq!(decoded.hours, value.hours);
        assert_eq!(decoded.minutes, value.minutes);
        assert_eq!(decoded.seconds, value.seconds);
    }

    #[test]
    fn test_der_validate_boolean_value() {
        // Valid boolean values
        assert!(DER.validate_boolean_value(&[0x00]).is_ok());
        assert!(DER.validate_boolean_value(&[0xFF]).is_ok());
        assert!(DER.validate_boolean_value(&[0x01]).is_ok());

        // Invalid boolean values
        assert!(DER.validate_boolean_value(&[]).is_err());
        assert!(DER.validate_boolean_value(&[0x00, 0x01]).is_err());
    }

    #[test]
    fn test_der_validate_bit_string_value() {
        // Valid bit string values
        assert!(DER.validate_bit_string_value(&[0x00, 0xFF]).is_ok());
        assert!(DER.validate_bit_string_value(&[0x03, 0xFF]).is_ok());

        // Invalid bit string values
        assert!(DER.validate_bit_string_value(&[]).is_err());
        assert!(DER.validate_bit_string_value(&[0x08]).is_err());
        assert!(DER.validate_bit_string_value(&[0x08, 0xFF]).is_err());
    }

    #[test]
    fn test_der_validate_utc_time_value() {
        // Valid UTC time values
        assert!(DER.validate_utc_time_value(b"991205223344Z").is_ok());

        // Invalid UTC time values
        assert!(DER.validate_utc_time_value(b"9912052233Z").is_err()); // Missing seconds
        assert!(DER.validate_utc_time_value(b"991205223344+0523").is_err()); // UTC Offset
        assert!(DER.validate_utc_time_value(b"").is_err());
        assert!(DER.validate_utc_time_value(b"991205223344").is_err()); // Missing timezone
        assert!(DER.validate_utc_time_value(b"991205223344+9999").is_err()); // Invalid offset
    }

    #[test]
    fn test_der_validate_generalized_time_value() {
        // Valid generalized time values
        assert!(DER.validate_generalized_time_value(b"20231201120000Z").is_ok());
        assert!(DER.validate_generalized_time_value(b"20231201120000.123Z").is_ok());

        // Invalid generalized time values
        assert!(DER.validate_generalized_time_value(b"20231201120000+0500").is_err()); // Offset time
        assert!(DER.validate_generalized_time_value(b"20231201120000").is_err()); // Local time
        assert!(DER.validate_generalized_time_value(b"").is_err()); // Empty
        assert!(DER.validate_generalized_time_value(b"20231301120000Z").is_err()); // Invalid month
    }

    #[test]
    fn test_der_validate_duration_value() {
        // Valid duration values
        assert!(DER.validate_duration_value(b"1Y2M3DT4H5M6S").is_ok());
        assert!(DER.validate_duration_value(b"1Y2M3D").is_ok());
        assert!(DER.validate_duration_value(b"T4H5M6S").is_ok());

        // Invalid duration values
        assert!(DER.validate_duration_value(b"").is_err());
        assert!(DER.validate_duration_value(b"P").is_err()); // No components
        assert!(DER.validate_duration_value(b"P1Y2M3DT").is_err()); // T without time components
    }

    #[test]
    fn test_der_decode_sequence() {
        // Create a simple sequence with two elements
        let seq_value = vec![
            ASN1Value::BooleanValue(true),
            ASN1Value::IntegerValue(vec![42]),
        ];
        let encoded = DER.encode_any(&ASN1Value::SequenceValue(seq_value.clone())).unwrap();
        let decoded = DER.decode_sequence(&encoded).unwrap();
        assert_eq!(decoded.len(), 2);

        if let ASN1Value::BooleanValue(val) = &decoded[0] {
            assert_eq!(*val, true);
        } else {
            panic!("Expected boolean value");
        }

        if let ASN1Value::IntegerValue(val) = &decoded[1] {
            assert_eq!(*val, vec![42]);
        } else {
            panic!("Expected integer value");
        }
    }

    #[test]
    fn test_der_decode_set() {
        // Create a simple set with two elements
        let set_value = vec![
            ASN1Value::BooleanValue(false),
            ASN1Value::IntegerValue(vec![123]),
        ];
        let encoded = DER.encode_any(&ASN1Value::SetValue(set_value.clone())).unwrap();
        let decoded = DER.decode_set(&encoded).unwrap();
        assert_eq!(decoded.len(), 2);

        // Note: Set order is not guaranteed, so we check both possible orders
        let has_bool = decoded.iter().any(|v| {
            if let ASN1Value::BooleanValue(val) = v {
                *val == false
            } else {
                false
            }
        });
        let has_int = decoded.iter().any(|v| {
            if let ASN1Value::IntegerValue(val) = v {
                *val == vec![123]
            } else {
                false
            }
        });
        assert!(has_bool);
        assert!(has_int);
    }

    #[test]
    fn test_der_decode_any() {
        // Test decoding various ASN.1 values
        let bool_value = ASN1Value::BooleanValue(true);
        let encoded = DER.encode_any(&bool_value).unwrap();
        let decoded = DER.decode_any(&encoded).unwrap();
        assert_eq!(decoded, bool_value);

        let int_value = ASN1Value::IntegerValue(vec![42]);
        let encoded = DER.encode_any(&int_value).unwrap();
        let decoded = DER.decode_any(&encoded).unwrap();
        assert_eq!(decoded, int_value);

        let string_value = ASN1Value::UTF8String("Hello".to_string());
        let encoded = DER.encode_any(&string_value).unwrap();
        let decoded = DER.decode_any(&encoded).unwrap();
        assert_eq!(decoded, string_value);
    }

    #[test]
    fn test_der_decode_from_bytes() {
        // Test decoding from Bytes
        let encoded_data = vec![
            X690_TAG_CLASS_UNIVERSAL | UNIV_TAG_BOOLEAN as u8,
            0x01,
            0xFF,
        ];
        let bytes = Bytes::from(encoded_data);
        let (bytes_read, el) = DER.decode_from_bytes(bytes).unwrap();
        assert_eq!(bytes_read, 3);
        assert_eq!(el.tag.tag_class, TagClass::UNIVERSAL);
        assert_eq!(el.tag.tag_number, UNIV_TAG_BOOLEAN);
    }

    #[test]
    fn test_der_indefinite_length() {
        // Test indefinite length encoding
        let encoded_data = vec![
            X690_TAG_CLASS_UNIVERSAL | 0b0010_0000 | UNIV_TAG_SEQUENCE as u8,
            0x80, // Indefinite length
            X690_TAG_CLASS_UNIVERSAL | UNIV_TAG_BOOLEAN as u8,
            0x01,
            0xFF,
            X690_TAG_CLASS_UNIVERSAL | UNIV_TAG_END_OF_CONTENT as u8,
            0x00,
        ];
        assert!(DER.decode_from_slice(&encoded_data).is_err());
    }

    #[test]
    fn test_der_constructed_bit_string() {
        let bit_string1 = BIT_STRING::from_parts_borrowed(&[0xFF], 0);
        let bit_string2 = BIT_STRING::from_parts_borrowed(&[0x00], 4);
        let encoded1 = DER.encode_bit_string(&bit_string1).unwrap();
        let encoded2 = DER.encode_bit_string(&bit_string2).unwrap();
        let constructed = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_BIT_STRING),
            X690Value::Constructed(Arc::new(vec![encoded1, encoded2])),
        );
        assert!(DER.decode_bit_string(&constructed).is_err());
    }

    #[test]
    fn test_der_validate_string_types() {
        // Test UTF8String validation
        let utf8_value = "Hello, 世界!";
        let encoded = DER.encode_utf8_string(utf8_value).unwrap();
        assert!(DER.validate_utf8_string(&encoded).is_ok());

        // Test NumericString validation
        let numeric_value = "1234567890";
        let encoded = DER.encode_numeric_string(numeric_value).unwrap();
        assert!(DER.validate_numeric_string(&encoded).is_ok());

        // Test PrintableString validation
        let printable_value = "Hello World";
        let encoded = DER.encode_printable_string(printable_value).unwrap();
        assert!(DER.validate_printable_string(&encoded).is_ok());

        // Test IA5String validation
        let ia5_value = "Hello World!";
        let encoded = DER.encode_ia5_string(ia5_value).unwrap();
        assert!(DER.validate_ia5_string(&encoded).is_ok());
    }

    #[test]
    fn test_der_validate_time_types() {
        let utc_value = UTCTime{
            year: 23,
            month: 12,
            day: 1,
            hour: 12,
            minute: 0,
            second: 0,
            utc_offset: UTCOffset{hour: 0, minute: 0},
        };
        let encoded = DER.encode_utc_time(&utc_value).unwrap();
        assert!(DER.validate_utc_time(&encoded).is_ok());
        let gen_value = GeneralizedTime{
            date: DATE{year: 2023, month: 12, day: 1},
            utc_offset: Some(UTCOffset::utc()),
            hour: 12,
            min_and_sec: Some((0, Some(0))),
            flags: 0,
            fraction: 0,
        };
        let encoded = DER.encode_generalized_time(&gen_value).unwrap();
        assert!(DER.validate_generalized_time(&encoded).is_ok());
    }

    #[test]
    fn test_der_validate_bit_string() {
        // Test primitive BIT STRING validation
        let bit_string = BIT_STRING::from_parts_borrowed(&[0xFF, 0x0F], 4);
        let encoded = DER.encode_bit_string(&bit_string).unwrap();
        assert!(DER.validate_bit_string(&encoded).is_ok());

        // Test constructed BIT STRING validation
        let bit_string1 = BIT_STRING::from_parts_borrowed(&[0xFF], 0);
        let bit_string2 = BIT_STRING::from_parts_borrowed(&[0x0F], 4);

        let encoded1 = DER.encode_bit_string(&bit_string1).unwrap();
        let encoded2 = DER.encode_bit_string(&bit_string2).unwrap();

        let constructed = X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_BIT_STRING),
            X690Value::Constructed(Arc::new(vec![encoded1, encoded2])),
        );

        assert!(DER.validate_bit_string(&constructed).is_err());
    }

    #[test]
    fn test_der_validate_octet_string() {
        let octet_string = vec![0x01, 0x02, 0x03, 0x04];
        let encoded = DER.encode_octet_string(&octet_string).unwrap();
        assert!(DER.validate_octet_string(&encoded).is_ok());
    }

    #[test]
    fn test_der_validate_object_descriptor() {
        let descriptor = "Test Object Descriptor";
        let encoded = DER.encode_object_descriptor(descriptor).unwrap();
        assert!(DER.validate_object_descriptor(&encoded).is_ok());
    }

    #[test]
    fn test_der_validate_universal_string() {
        let universal_string = UniversalString(vec![0x0048, 0x0065, 0x006C, 0x006C, 0x006F]);
        let encoded = DER.encode_universal_string(&universal_string.0).unwrap();
        assert!(DER.validate_universal_string(&encoded).is_ok());
    }

    #[test]
    fn test_der_validate_bmp_string() {
        let bmp_string = BMPString(vec![0x0048, 0x0065, 0x006C, 0x006C, 0x006F]);
        let encoded = DER.encode_bmp_string(&bmp_string.0).unwrap();
        assert!(DER.validate_bmp_string(&encoded).is_ok());
    }

    #[test]
    fn test_der_error_cases() {
        // Test truncated TLV
        let truncated = [0x01]; // Just tag, no length or value
        let result = DER.decode_from_slice(&truncated);
        assert!(result.is_err());

        // Test invalid length
        let invalid_length = [0x01, 0x82, 0x01]; // Length says 2 bytes but only 1 provided
        let result = DER.decode_from_slice(&invalid_length);
        assert!(result.is_err());

        // Test indefinite length with non-constructed tag
        let invalid_indefinite = [
            X690_TAG_CLASS_UNIVERSAL | UNIV_TAG_BOOLEAN as u8,
            0x80, // Indefinite length
        ];
        let result = DER.decode_from_slice(&invalid_indefinite);
        assert!(result.is_err());
    }

    #[test]
    fn test_der_length_edge_cases() {
        // Test maximum short form length
        let max_short = [0x7F];
        let (bytes_read, length) = der_decode_length(&max_short).unwrap();
        assert_eq!(bytes_read, 1);
        assert_eq!(length, X690Length::Definite(127));

        // Test minimum long form length
        let min_long = [0x81, 0x80];
        let (bytes_read, length) = der_decode_length(&min_long).unwrap();
        assert_eq!(bytes_read, 2);
        assert_eq!(length, X690Length::Definite(128));

        // Test large length values
        let large_length = [0x84, 0x01, 0x00, 0x00, 0x00]; // 16777216
        let (bytes_read, length) = der_decode_length(&large_length).unwrap();
        assert_eq!(bytes_read, 5);
        assert_eq!(length, X690Length::Definite(16777216));
    }

    #[test]
    fn test_der_real_value_edge_cases() {
        // Test zero value
        let zero_bytes: [u8; 0] = [];
        let result = DER.decode_real_value(&zero_bytes).unwrap();
        assert_eq!(result, 0.0);

        // Test negative zero
        let neg_zero_bytes = [0x43]; // Special real, minus zero
        let result = DER.decode_real_value(&neg_zero_bytes).unwrap();
        assert_eq!(result, -0.0);

        // Test very large numbers
        let large_bytes = [0x80, 0x0A, 0x01]; // Binary, base 2, exponent 10, mantissa 1
        let result = DER.decode_real_value(&large_bytes).unwrap();
        assert_eq!(result, 1024.0); // 1 * 2^10 = 1024
    }

    #[test]
    fn test_der_string_encoding_edge_cases() {
        // Test empty strings
        let empty_utf8 = "";
        let encoded = DER.encode_utf8_string(empty_utf8).unwrap();
        let decoded = DER.decode_utf8_string(&encoded).unwrap();
        assert_eq!(decoded, empty_utf8);

        // Test strings with special characters
        let special_chars = "Hello\n\t\r\0";
        let encoded = DER.encode_utf8_string(special_chars).unwrap();
        let decoded = DER.decode_utf8_string(&encoded).unwrap();
        assert_eq!(decoded, special_chars);

        // Test very long strings
        let long_string = "A".repeat(1000);
        let encoded = DER.encode_utf8_string(&long_string).unwrap();
        let decoded = DER.decode_utf8_string(&encoded).unwrap();
        assert_eq!(decoded, long_string);
    }

    #[test]
    fn test_der_sequence_encoding_edge_cases() {
        // Test empty sequence
        let empty_seq = vec![];
        let encoded = DER.encode_any(&ASN1Value::SequenceValue(empty_seq.clone())).unwrap();
        let decoded = DER.decode_sequence(&encoded).unwrap();
        assert_eq!(decoded.len(), 0);

        // Test sequence with many elements
        let many_elements: Vec<ASN1Value> = (0..100)
            .map(|i| ASN1Value::IntegerValue(vec![i as u8]))
            .collect();
        let encoded = DER.encode_any(&ASN1Value::SequenceValue(many_elements.clone())).unwrap();
        let decoded = DER.decode_sequence(&encoded).unwrap();
        assert_eq!(decoded.len(), 100);

        for (i, val) in decoded.iter().enumerate() {
            if let ASN1Value::IntegerValue(num) = val {
                assert_eq!(*num, vec![i as u8]);
            } else {
                panic!("Expected integer value");
            }
        }
    }

    #[test]
    fn test_der_nested_structures() {
        // Test nested sequences
        let inner_seq = vec![
            ASN1Value::BooleanValue(true),
            ASN1Value::IntegerValue(vec![42]),
        ];
        let outer_seq = vec![
            ASN1Value::UTF8String("Hello".to_string()),
            ASN1Value::SequenceValue(inner_seq),
        ];

        let encoded = DER.encode_any(&ASN1Value::SequenceValue(outer_seq)).unwrap();
        let decoded = DER.decode_sequence(&encoded).unwrap();

        assert_eq!(decoded.len(), 2);

        if let ASN1Value::UTF8String(s) = &decoded[0] {
            assert_eq!(s, "Hello");
        } else {
            panic!("Expected UTF8String");
        }

        if let ASN1Value::SequenceValue(inner) = &decoded[1] {
            assert_eq!(inner.len(), 2);
        } else {
            panic!("Expected SequenceValue");
        }
    }

    #[test]
    fn test_der_round_trip_complex() {
        // Test complex round-trip encoding/decoding
        let complex_value = ASN1Value::SequenceValue(vec![
            ASN1Value::BooleanValue(true),
            ASN1Value::IntegerValue(vec![42]),
            ASN1Value::UTF8String("Hello, 世界!".to_string()),
            ASN1Value::OctetStringValue(vec![0x01, 0x02, 0x03, 0x04]),
            ASN1Value::RealValue(3.14159),
            ASN1Value::SetValue(vec![
                ASN1Value::BooleanValue(false),
                ASN1Value::IntegerValue(vec![123]),
            ]),
        ]);

        let encoded = DER.encode_any(&complex_value).unwrap();
        let decoded = DER.decode_any(&encoded).unwrap();

        // Verify the structure matches
        if let ASN1Value::SequenceValue(seq) = &decoded {
            assert_eq!(seq.len(), 6);

            // Check boolean
            if let ASN1Value::BooleanValue(val) = &seq[0] {
                assert_eq!(*val, true);
            } else {
                panic!("Expected boolean");
            }

            // Check integer
            if let ASN1Value::IntegerValue(val) = &seq[1] {
                assert_eq!(*val, vec![42]);
            } else {
                panic!("Expected integer");
            }

            // Check string
            if let ASN1Value::UTF8String(val) = &seq[2] {
                assert_eq!(val, "Hello, 世界!");
            } else {
                panic!("Expected UTF8String");
            }

            // Check octet string
            if let ASN1Value::OctetStringValue(val) = &seq[3] {
                assert_eq!(val, &[0x01, 0x02, 0x03, 0x04]);
            } else {
                panic!("Expected OctetStringValue");
            }

            // Check real
            if let ASN1Value::RealValue(val) = &seq[4] {
                assert!((*val - 3.14159).abs() < 0.001);
            } else {
                panic!("Expected RealValue");
            }

            // Check set
            if let ASN1Value::SetValue(set) = &seq[5] {
                assert_eq!(set.len(), 2);
            } else {
                panic!("Expected SetValue");
            }
        } else {
            panic!("Expected SequenceValue");
        }
    }

    #[test]
    fn test_der_real_edge_cases() {
        let cases = vec![
            0.0,
            -0.0,
            (10.0 / 3.0), // Non-terminating decimal
            (-10.0 / 3.0), // Negative non-terminating decimal
            1.0,
            -1.0,
            2.718281828459045, // e
            2.302585092994046, // ln(10)
            0.6931471805599453, // ln(2)
            0.4342944819032518, // Log10(e)
            1.4426950408889634, // Log2(e)
            3.141592653589793, //
            0.7071067811865476, // sqrt(1/2)
            1.4142135623730951,
            (1.4142135623730951 / 2.0),
            1.618033988749895, // GOLDEN_RATIO
            0.57721,      // EULER_MASCHERONI_CONSTANT
            0.2614972128, // MEISSEL_MERTENS_CONSTANT
            0.2801694990, // BERNSTEINS_CONSTANT
            0.3036630028, // GAUSS_KUZMIN_WIRSING_CONSTANT
            0.3532363718, // HAFNER_SARNAK_MCCURLEY_CONSTANT
            0.5671432904, // OMEGA_CONSTANT
            0.6243299885, // GOLOMB_DICKMAN_CONSTANT
            0.6434105462, // CAHENS_CONSTANT
            0.6601618158, // TWIN_PRIME_CONSTANT
            0.6627434193, // LAPLACE_LIMIT
            0.70258,      // LANDAU_RAMANUJAN_CONSTANT
            0.8093940205, // ALLADI_GRINSTEAD_CONSTANT
            0.87058838,   // BRUNS_CONSTANT_FOR_PRIME_QUADRUPLETS
            0.9159655941, // CATALANS_CONSTANT
            1.0986858055, // LENGYELS_CONSTANT
            1.13198824,   // VISWANATHS_CONSTANT
            1.2020569,    // APERYS_CONSTANT
            1.30357,      // CONWAYS_CONSTANT
            1.3063778838, // MILLS_CONSTANT
            1.3247179572, // PLASTIC_CONSTANT
            1.4513692348, // RAMANUJAN_SOLDNER_CONSTANT
            1.4560749485, // BACKHOUSES_CONSTANT
            1.4670780794, // PORTERS_CONSTANT
            1.5396007178, // LIEBS_SQUARE_ICE_CONSTANT
            1.6066951524, // ERDOS_BORWEIN_CONSTANT
            1.7052111401, // NIVENS_CONSTANT
            1.9021605831, // BRUNS_CONSTANT_FOR_TWIN_PRIMES
            2.2955871493, // UNIVERSAL_PARABOLIC_CONSTANT
            2.5029078750, // FEIGENBAUM_CONSTANT_ALPHA
            2.5849817595, // SIERPINSKIS_CONSTANT
            2.6854520010, // KHINCHINS_CONSTANT
            2.8077702420, // FRANSEN_ROBINSON_CONSTANT
            3.2758229187, // LEVYS_CONSTANT
            3.3598856662, // RECIPROCAL_FIBONACCI_CONSTANT
            4.6692016091, // FEIGENBAUM_CONSTANT_DELTA
            1.2824271291,  // GLAISHER_KINKELIN_CONSTANT
            5.59e123, // Huge number
            5.59e-12, // Tiny number
            -5.59e123, // Huge negative number
            -5.59e-12, // Tiny negative number
        ];
        for case in cases {
            let encoded = DER.encode_real(&case).unwrap();
            let decoded = DER.decode_real(&encoded).unwrap();
            assert!((case - decoded).abs() < 0.001); // Little wiggle room for error.
        }
    }

    #[test]
    fn test_real_fuzz_bug_1() {
        let case = [ 2, 117 ];
        let _ = DER.validate_real_value(case.as_slice());
        let _ = DER.validate_real_value(case.as_slice());
        let _ = DER.decode_real_value(case.as_slice());
        let _ = DER.decode_real_value(case.as_slice());
    }

    #[test]
    fn test_utc_time_fuzz_bug_1() {
        let case: Vec<u8> = vec![23, 14, 49, 49, 49, 55, 55, 49, 49, 49, 49, 53, 149, 55, 48, 128, 48, 4, 0];
        if let Ok((_, el)) = DER.decode_from_slice(case.as_slice()) {
            let _ = DER.decode_any(&el);
        }
    }

}
