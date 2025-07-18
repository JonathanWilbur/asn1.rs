//! This module implements the Basic Encoding Rules (BER) for X.690.
use wildboar_asn1::{
    join_bit_strings, ASN1Error, ASN1ErrorCode, ASN1Result, Tag
};
use crate::codec::{BasicEncodingRules, X690Codec};
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
    deconstruct,
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
use simdutf8::basic::from_utf8;
use std::str::FromStr;
use crate::utils::{vec_u16_to_vec_u8, vec_u32_to_vec_u8, get_days_in_month, x690_read_var_length_u64};

pub const BER: BasicEncodingRules = BasicEncodingRules::new();

/// Deconstruct a constructed `BIT STRING` to obtain a single `BIT STRING`
///
/// `BIT STRING` is constructed in a such a way that the octets of each subelement
/// cannot simply be concatenated. As such, this function deconstructed a
/// constructed `BIT STRING` to obtain a single `BIT STRING`.
pub fn ber_deconstruct_bit_string(el: &X690Element) -> ASN1Result<BIT_STRING> {
    match &el.value {
        X690Value::Primitive(bytes) => Ok(BER.decode_bit_string_value(bytes)?),
        X690Value::Constructed(children) => {
            let mut substituent_bit_strings: Vec<BIT_STRING> = Vec::new();
            for child in children.iter() {
                if child.tag.tag_class != el.tag.tag_class || child.tag.tag_number != el.tag.tag_number {
                    let mut err =
                        ASN1Error::new(ASN1ErrorCode::string_constructed_with_invalid_tagging);
                    err.tag = Some(Tag::new(el.tag.tag_class, el.tag.tag_number));
                    err.length = Some(el.len());
                    err.constructed = Some(true);
                    return Err(err);
                }
                let deconstructed_child = ber_deconstruct_bit_string(&child)?;
                substituent_bit_strings.push(deconstructed_child);
            }
            return Ok(join_bit_strings(&substituent_bit_strings.as_slice()));
        },
        X690Value::Serialized(v) => {
            let (_, el) = BER.decode_from_slice(&v).unwrap();
            ber_deconstruct_bit_string(&el)
        }
    }
}

// Non-terminal `BIT STRING`s in a constructed `BIT STRING` are not allowed to have
// a non-zero number of trailing bits.
fn validate_non_terminal_bit_strings (el: &X690Element) -> ASN1Result<()> {
    match &el.value {
        X690Value::Primitive(v) => {
            if v.len() == 0 {
                return Err(ASN1Error::new(ASN1ErrorCode::x690_bit_string_zero_bytes));
            }
            if v[0] != 0 {
                return Err(ASN1Error::new(ASN1ErrorCode::x690_bit_string_non_terminal_segment_with_trailing_bits));
            }
            Ok(())
        },
        X690Value::Constructed(subs) => {
            for sub in subs.iter() {
                validate_non_terminal_bit_strings(&sub)?;
            }
            Ok(())
        },
        X690Value::Serialized(v) => {
            let (_, el) = BER.decode_from_slice(&v).unwrap();
            validate_non_terminal_bit_strings(&el)
        }
    }
}

/// Decode the length of an X.690-encoded element from a byte slice
/// 
/// This starts at the first byte.
pub fn ber_decode_length(bytes: ByteSlice) -> ASN1Result<(usize, X690Length)> {
    if bytes.len() == 0 {
        // Truncated.
        return Err(ASN1Error::new(ASN1ErrorCode::tlv_truncated));
    }
    if bytes[0] < 0b1000_0000 {
        // Equivalent to ((b[0] & 0b1000_0000) == 0)
        return Ok((1, X690Length::Definite(bytes[0] as usize)));
    }
    if bytes[0] == 0b1000_0000 {
        return Ok((1, X690Length::Indefinite));
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

impl X690Codec for BasicEncodingRules {

    fn decode_from_slice(&self, bytes: wildboar_asn1::ByteSlice) -> ASN1Result<(usize, X690Element)> {
        let (len, tag, constructed) = x690_decode_tag(bytes)?;
        let mut bytes_read: usize = len;
        let value_length;
        let (len_len, len) = ber_decode_length(&bytes[bytes_read..])?;
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
            }
            X690Length::Indefinite => {
                if !constructed {
                    // Indefinite length must be constructed.
                    let mut err =
                        ASN1Error::new(ASN1ErrorCode::x690_indefinite_length_but_not_constructed);
                    err.tag = Some(Tag::new(tag.tag_class, tag.tag_number));
                    return Err(err);
                }
                /* We don't know how many child elements this element must have, but
                it is a good guess (optimization) to assume that there is at least
                one. */
                let mut children: Vec<X690Element> = Vec::with_capacity(1);
                let mut value_bytes_read: usize = 0;
                while value_bytes_read < bytes.len() {
                    let (el_len, el) =  self.decode_from_slice(&bytes[bytes_read + value_bytes_read..])?;
                    if el_len == 0 {
                        break;
                    }
                    value_bytes_read += el_len;
                    if el.tag.tag_class == TagClass::UNIVERSAL
                        && (el.tag.tag_number == UNIV_TAG_END_OF_CONTENT)
                    {
                        // We do NOT append the EOC element. It is treated like it does not exist.
                        break;
                    }
                    children.push(el);
                }
                bytes_read += value_bytes_read;
                let el = X690Element::new(
                    tag,
                    X690Value::Constructed(Arc::new(children)),
                );
                Ok((bytes_read, el))
            }
        }
    }

    fn decode_from_bytes(&self, bytes: Bytes) -> ASN1Result<(usize, X690Element)> {
        let (len, tag, constructed) = x690_decode_tag(&bytes)?;
        let mut bytes_read: usize = len;
        let value_length;
        let (len_len, len) = ber_decode_length(&bytes[bytes_read..])?;
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
            }
            X690Length::Indefinite => {
                if !constructed {
                    // Indefinite length must be constructed.
                    let mut err =
                        ASN1Error::new(ASN1ErrorCode::x690_indefinite_length_but_not_constructed);
                    err.tag = Some(Tag::new(tag.tag_class, tag.tag_number));
                    return Err(err);
                }
                /* We don't know how many child elements this element must have, but
                it is a good guess (optimization) to assume that there is at least
                one. */
                let mut children: Vec<X690Element> = Vec::with_capacity(1);
                let mut value_bytes_read: usize = 0;
                while value_bytes_read < bytes.len() {
                    let (el_len, el) = self.decode_from_bytes(bytes.slice(bytes_read + value_bytes_read..))?;
                    if el_len == 0 {
                        break;
                    }
                    value_bytes_read += el_len;
                    if el.tag.tag_class == TagClass::UNIVERSAL
                        && (el.tag.tag_number == UNIV_TAG_END_OF_CONTENT)
                    {
                        // We do NOT append the EOC element. It is treated like it does not exist.
                        break;
                    }
                    children.push(el);
                }
                bytes_read += value_bytes_read;
                let el = X690Element::new(
                    tag,
                    X690Value::Constructed(Arc::new(children)),
                );
                Ok((bytes_read, el))
            }
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
                let s = from_utf8(&value_bytes[1..])
                    .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_utf8(None)))?;
                let format = value_bytes[0] & 0b0011_1111;
                return match format {
                    crate::X690_REAL_NR1 => iso6093::parse_nr1(s)
                        .map_err(|_| ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(s.to_owned().into_bytes()))),
                    crate::X690_REAL_NR2 => iso6093::parse_nr2(s)
                        .map_err(|_| ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(s.to_owned().into_bytes()))),
                    crate::X690_REAL_NR3 => iso6093::parse_nr3(s)
                        .map_err(|_| ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(s.to_owned().into_bytes()))),
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_unrecognized_format(format))),
                };
            }
            _ => {
                // Binary encoding
                let negative = (value_bytes[0] & 0b0100_0000) > 0;
                let base_byte = value_bytes[0] & X690_REAL_BASE_MASK;
                let base: u8 = match base_byte {
                    crate::X690_REAL_BASE_2 => 2,
                    crate::X690_REAL_BASE_8 => 8,
                    crate::X690_REAL_BASE_16 => 16,
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_unrecognized_base(base_byte))),
                };
                let scale: u8 = (value_bytes[0] & X690_REAL_BINARY_SCALING_MASK)
                    .overflowing_shr(2)
                    .0;
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
                        mantissa = x690_read_var_length_u64(&value_bytes[3..])
                            .ok_or(ASN1Error::new(ASN1ErrorCode::value_too_big))?;
                    }
                    crate::X690_REAL_EXPONENT_FORMAT_3_OCTET => {
                        if value_bytes.len() < 5 {
                            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                        }
                        if value_bytes.len() > 5 + 6 {
                            // Mantissa too big.
                            return Err(ASN1Error::new(ASN1ErrorCode::field_too_big));
                        }
                        exponent =
                            i32::from_be_bytes([0, value_bytes[1], value_bytes[2], value_bytes[3]]);
                        mantissa = x690_read_var_length_u64(&value_bytes[4..])
                            .ok_or(ASN1Error::new(ASN1ErrorCode::value_too_big))?;
                    }
                    crate::X690_REAL_EXPONENT_FORMAT_VAR_OCTET => {
                        if value_bytes.len() < 3 {
                            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                        }
                        let exponent_len = value_bytes[1];
                        if exponent_len > 2 {
                            return Err(ASN1Error::new(ASN1ErrorCode::field_too_big));
                        }
                        if value_bytes.len() > (3 + exponent_len).into() {
                            // Mantissa too big.
                            return Err(ASN1Error::new(ASN1ErrorCode::field_too_big));
                        }
                        if exponent_len == 1 {
                            exponent = value_bytes[2] as i8 as i32;
                            mantissa = x690_read_var_length_u64(&value_bytes[3..])
                                .ok_or(ASN1Error::new(ASN1ErrorCode::value_too_big))?;
                        } else {
                            // The exponent must have length 2.
                            exponent = i32::from_be_bytes([0, 0, value_bytes[2], value_bytes[3]]);
                            mantissa = x690_read_var_length_u64(&value_bytes[4..])
                                .ok_or(ASN1Error::new(ASN1ErrorCode::value_too_big))?;
                        }
                    }
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::bin_real_unrecognized_exp_fmt))
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
        UTCTime::try_from(value_bytes)
    }

    #[inline]
    fn decode_generalized_time_value(&self, value_bytes: ByteSlice) -> ASN1Result<GeneralizedTime> {
        GeneralizedTime::try_from(value_bytes)
    }

    #[inline]
    fn decode_duration_value(&self, value_bytes: ByteSlice) -> ASN1Result<DURATION> {
        DURATION::try_from(value_bytes)
    }

    #[inline]
    fn decode_bit_string(&self, el: &X690Element) -> ASN1Result<BIT_STRING> {
        ber_deconstruct_bit_string(&el)
    }

    #[inline]
    fn decode_octet_string(&self, el: &X690Element) -> ASN1Result<OCTET_STRING> {
        Ok(deconstruct(el)?.into_owned())
    }

    fn decode_sequence(&self, el: &X690Element) -> ASN1Result<SEQUENCE> {
        match &el.value {
            X690Value::Constructed(children) => {
                let mut ret: Vec<ASN1Value> = Vec::with_capacity(children.len());
                for child in children.iter() {
                    ret.push(self.decode_any(child)?);
                }
                Ok(ret)
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v).unwrap();
                self.decode_sequence(&el)
            },
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }

    fn decode_set(&self, el: &X690Element) -> ASN1Result<SET> {
        match &el.value {
            X690Value::Constructed(children) => {
                let mut ret: Vec<ASN1Value> = Vec::with_capacity(children.len());
                for child in children.iter() {
                    ret.push(self.decode_any(child)?);
                }
                Ok(ret)
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v).unwrap();
                self.decode_set(&el)
            },
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
    }

    #[inline]
    fn decode_object_descriptor(&self, el: &X690Element) -> ASN1Result<ObjectDescriptor> {
        self.decode_graphic_string_value(deconstruct(el)?.as_ref())
    }

    #[inline]
    fn decode_utf8_string(&self, el: &X690Element) -> ASN1Result<UTF8String> {
        String::from_utf8(deconstruct(el)?.into_owned())
            .map_err(|e| ASN1Error::new(ASN1ErrorCode::invalid_utf8(Some(e.utf8_error()))))
    }

    #[inline]
    fn decode_numeric_string(&self, el: &X690Element) -> ASN1Result<NumericString> {
        self.decode_numeric_string_value(deconstruct(el)?.as_ref())
    }

    #[inline]
    fn decode_printable_string(&self, el: &X690Element) -> ASN1Result<PrintableString> {
        self.decode_printable_string_value(deconstruct(el)?.as_ref())
    }

    #[inline]
    fn decode_t61_string(&self, el: &X690Element) -> ASN1Result<T61String> {
        Ok(deconstruct(el)?.into_owned())
    }

    #[inline]
    fn decode_videotex_string(&self, el: &X690Element) -> ASN1Result<VideotexString> {
        Ok(deconstruct(el)?.into_owned())
    }

    #[inline]
    fn decode_ia5_string(&self, el: &X690Element) -> ASN1Result<IA5String> {
        self.decode_ia5_string_value(deconstruct(el)?.as_ref())
    }

    #[inline]
    fn decode_utc_time(&self, el: &X690Element) -> ASN1Result<UTCTime> {
        self.decode_utc_time_value(deconstruct(el)?.as_ref())
    }

    #[inline]
    fn decode_generalized_time(&self, el: &X690Element) -> ASN1Result<GeneralizedTime> {
        self.decode_generalized_time_value(deconstruct(el)?.as_ref())
    }

    #[inline]
    fn decode_graphic_string(&self, el: &X690Element) -> ASN1Result<GraphicString> {
        self.decode_graphic_string_value(deconstruct(el)?.as_ref())
    }

    #[inline]
    fn decode_visible_string(&self, el: &X690Element) -> ASN1Result<VisibleString> {
        self.decode_visible_string_value(deconstruct(el)?.as_ref())
    }

    #[inline]
    fn decode_general_string(&self, el: &X690Element) -> ASN1Result<GeneralString> {
        self.decode_general_string_value(deconstruct(el)?.as_ref())
    }

    #[inline]
    fn decode_universal_string(&self, el: &X690Element) -> ASN1Result<UniversalString> {
        self.decode_universal_string_value(deconstruct(el)?.as_ref())
    }

    #[inline]
    fn decode_bmp_string(&self, el: &X690Element) -> ASN1Result<BMPString> {
        self.decode_bmp_string_value(deconstruct(el)?.as_ref())
    }

    fn decode_any(&self, el: &X690Element) -> ASN1Result<ASN1Value> {
        if el.tag.tag_class != TagClass::UNIVERSAL {
            return match &el.value {
                X690Value::Primitive(bytes) => Ok(ASN1Value::UnknownBytes(Arc::new(bytes.to_vec()))),
                X690Value::Constructed(components) => {
                    let mut values: Vec<ASN1Value> = Vec::with_capacity(components.len());
                    for child in components.iter() {
                        values.push(self.decode_any(&child)?);
                    }
                    return Ok(ASN1Value::SequenceValue(values));
                },
                X690Value::Serialized(v) => {
                    let (_, el) = BER.decode_from_slice(&v).unwrap();
                    self.decode_any(&el)
                }
            };
        }

        match el.tag.tag_number {
            UNIV_TAG_END_OF_CONTENT => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
            UNIV_TAG_BOOLEAN => match self.decode_boolean(el) {
                Ok(v) => Ok(ASN1Value::BooleanValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_INTEGER => match self.decode_integer(el) {
                Ok(v) => Ok(ASN1Value::IntegerValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_BIT_STRING => match self.decode_bit_string(el) {
                Ok(v) => Ok(ASN1Value::BitStringValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_OCTET_STRING => match self.decode_octet_string(el) {
                Ok(v) => Ok(ASN1Value::OctetStringValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_NULL => Ok(ASN1Value::NullValue),
            UNIV_TAG_OBJECT_IDENTIFIER => match self.decode_object_identifier(el) {
                Ok(v) => Ok(ASN1Value::ObjectIdentifierValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_OBJECT_DESCRIPTOR => match self.decode_object_descriptor(el) {
                Ok(v) => Ok(ASN1Value::ObjectDescriptor(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_EXTERNAL => match self.decode_external(el) {
                Ok(v) => Ok(ASN1Value::ExternalValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_REAL => match self.decode_real(el) {
                Ok(v) => Ok(ASN1Value::RealValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_ENUMERATED => match self.decode_enumerated(el) {
                Ok(v) => Ok(ASN1Value::EnumeratedValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_EMBEDDED_PDV => match self.decode_embedded_pdv(el) {
                Ok(v) => Ok(ASN1Value::EmbeddedPDVValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_UTF8_STRING => match self.decode_utf8_string(el) {
                Ok(v) => Ok(ASN1Value::UTF8String(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_RELATIVE_OID => match self.decode_relative_oid(el) {
                Ok(v) => Ok(ASN1Value::RelativeOIDValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_TIME => match self.decode_time(el) {
                Ok(v) => Ok(ASN1Value::TimeValue(v)),
                Err(e) => Err(e),
            },
            // UNIV_TAG_RESERVED_15 => ()
            UNIV_TAG_SEQUENCE => match self.decode_sequence(el) {
                Ok(v) => Ok(ASN1Value::SequenceValue(v)),
                Err(e) => Err(e),
            },
            // UNIV_TAG_SEQUENCE_OF => ()
            UNIV_TAG_SET => match self.decode_set(el) {
                Ok(v) => Ok(ASN1Value::SetValue(v)),
                Err(e) => Err(e),
            },
            // UNIV_TAG_SET_OF => ()
            UNIV_TAG_NUMERIC_STRING => match self.decode_numeric_string(el) {
                Ok(v) => Ok(ASN1Value::NumericString(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_PRINTABLE_STRING => match self.decode_printable_string(el) {
                Ok(v) => Ok(ASN1Value::PrintableString(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_T61_STRING => match self.decode_t61_string(el) {
                Ok(v) => Ok(ASN1Value::T61String(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_VIDEOTEX_STRING => match self.decode_videotex_string(el) {
                Ok(v) => Ok(ASN1Value::VideotexString(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_IA5_STRING => match self.decode_ia5_string(el) {
                Ok(v) => Ok(ASN1Value::IA5String(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_UTC_TIME => {
                let content_octets = el.content_octets()?;
                Ok(ASN1Value::UTCTime(UTCTime::try_from(content_octets.as_ref())?))
            },
            UNIV_TAG_GENERALIZED_TIME => {
                let content_octets = el.content_octets()?;
                Ok(ASN1Value::GeneralizedTime(GeneralizedTime::try_from(content_octets.as_ref())?))
            },
            UNIV_TAG_GRAPHIC_STRING => match self.decode_graphic_string(el) {
                Ok(v) => Ok(ASN1Value::GraphicString(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_VISIBLE_STRING => match self.decode_visible_string(el) {
                Ok(v) => Ok(ASN1Value::VisibleString(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_GENERAL_STRING => match self.decode_general_string(el) {
                Ok(v) => Ok(ASN1Value::GeneralString(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_UNIVERSAL_STRING => match self.decode_universal_string(el) {
                Ok(v) => Ok(ASN1Value::UniversalString(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_CHARACTER_STRING => match self.decode_character_string(el) {
                Ok(v) => Ok(ASN1Value::UnrestrictedCharacterStringValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_BMP_STRING => match self.decode_bmp_string(el) {
                Ok(v) => Ok(ASN1Value::BMPString(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_DATE => match self.decode_date(el) {
                Ok(v) => Ok(ASN1Value::DATE(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_TIME_OF_DAY => match self.decode_time_of_day(el) {
                Ok(v) => Ok(ASN1Value::TIME_OF_DAY(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_DATE_TIME => match self.decode_date_time(el) {
                Ok(v) => Ok(ASN1Value::DATE_TIME(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_DURATION => match self.decode_duration(el) {
                Ok(v) => Ok(ASN1Value::DURATION(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_OID_IRI => match self.decode_oid_iri(el) {
                Ok(v) => Ok(ASN1Value::IRIValue(v)),
                Err(e) => Err(e),
            },
            UNIV_TAG_RELATIVE_OID_IRI => match self.decode_relative_oid_iri(el) {
                Ok(v) => Ok(ASN1Value::RelativeIRIValue(v)),
                Err(e) => Err(e),
            },
            _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
        }
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
                BER.encode_character_string(v)
            },
            ASN1Value::BMPString(v) => self.encode_bmp_string(&v.0),
            ASN1Value::IRIValue(v) => self.encode_oid_iri(v),
            ASN1Value::RelativeIRIValue(v) => self.encode_relative_oid_iri(v),
            ASN1Value::TimeValue(v) => self.encode_time(v),
            ASN1Value::UnknownBytes(_) => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
            ASN1Value::ChoiceValue(v) => BER.encode_any(v),
            ASN1Value::ISO646String(v) => self.encode_ia5_string(v),
            ASN1Value::TeletexString(v) => self.encode_t61_string(v),
            ASN1Value::DATE(v) => self.encode_date(v),
            ASN1Value::TIME_OF_DAY(v) => self.encode_time_of_day(v),
            ASN1Value::DATE_TIME(v) => self.encode_date_time(v),
            ASN1Value::DURATION(v) => self.encode_duration(v),
            ASN1Value::TaggedValue(v) => BER.encode_any(&v.value),
            ASN1Value::SequenceValue(v)
            | ASN1Value::SequenceOfValue(v) => {
                let mut components: Vec<X690Element> = Vec::with_capacity(v.len());
                for subv in v {
                    let el = BER.encode_any(&subv)?;
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
                    let el = BER.encode_any(&subv)?;
                    components.push(el);
                }
                Ok(X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SET),
                    X690Value::Constructed(Arc::new(components)),
                ))
            },
            ASN1Value::InstanceOfValue(v) => BER.encode_instance_of(&v),
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
        let mut out = BytesMut::with_capacity(17).writer(); // This is the max length of a UTCTime.
        x690_write_utc_time_value(&mut out, &value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UTC_TIME),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_generalized_time(&self, value: &GeneralizedTime) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(32).writer(); // There is no defined max length, but this is very generous capacity.
        x690_write_generalized_time_value(&mut out, &value)?;
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

    #[inline]
    fn encode_owned_object_descriptor(&self, value: ObjectDescriptor) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_OBJECT_DESCRIPTOR),
            X690Value::Primitive(Bytes::from(value)),
        ))
    }

    #[inline]
    fn encode_owned_utf8_string(&self, value: UTF8String) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UTF8_STRING),
            X690Value::Primitive(Bytes::from(value)),
        ))
    }

    #[inline]
    fn encode_owned_numeric_string(&self, value: NumericString) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_NUMERIC_STRING),
            X690Value::Primitive(Bytes::from(value)),
        ))
    }

    #[inline]
    fn encode_owned_printable_string(&self, value: PrintableString) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_PRINTABLE_STRING),
            X690Value::Primitive(Bytes::from(value)),
        ))
    }

    #[inline]
    fn encode_owned_t61_string(&self, value: T61String) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_T61_STRING),
            X690Value::Primitive(Bytes::from(value)),
        ))
    }

    #[inline]
    fn encode_owned_videotex_string(&self, value: VideotexString) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_VIDEOTEX_STRING),
            X690Value::Primitive(Bytes::from(value)),
        ))
    }

    #[inline]
    fn encode_owned_ia5_string(&self, value: String) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_IA5_STRING),
            X690Value::Primitive(Bytes::from(value)),
        ))
    }

    #[inline]
    fn encode_owned_graphic_string(&self, value: GraphicString) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_GRAPHIC_STRING),
            X690Value::Primitive(Bytes::from(value)),
        ))
    }

    #[inline]
    fn encode_owned_visible_string(&self, value: VisibleString) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_VISIBLE_STRING),
            X690Value::Primitive(Bytes::from(value)),
        ))
    }

    #[inline]
    fn encode_owned_general_string(&self, value: GeneralString) -> ASN1Result<X690Element> {
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_GENERAL_STRING),
            X690Value::Primitive(Bytes::from(value)),
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

    fn encode_universal_string(&self, value: &[u32]) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len() << 2).writer(); // Four bytes for every character
        x690_write_universal_string_value(&mut out, value)?;
        Ok(X690Element::new(
            Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UNIVERSAL_STRING),
            X690Value::Primitive(out.into_inner().into()),
        ))
    }

    fn encode_bmp_string(&self, value: &[u16]) -> ASN1Result<X690Element> {
        let mut out = BytesMut::with_capacity(value.len() << 1).writer(); // Two bytes for every character
        x690_write_bmp_string_value(&mut out, value)?;
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
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        match content_octets[0] & 0b11000000 {
            0b0100_0000 => { // Special real formatting
                let special_real_value = content_octets[0] & 0b0000_0011;
                if (special_real_value != X690_SPECIAL_REAL_NOT_A_NUMBER)
                    && (special_real_value != X690_SPECIAL_REAL_PLUS_INFINITY)
                    && (special_real_value != X690_SPECIAL_REAL_MINUS_INFINITY)
                    && (special_real_value != X690_SPECIAL_REAL_MINUS_ZERO) {
                    return Err(ASN1Error::new(ASN1ErrorCode::unrecognized_special_real));
                }
            },
            0b0000_0000 => { // Textual / Base-10 formatting
                let base10_format = content_octets[0] & 0b0011_1111;
                if content_octets.len() == 1 {
                    return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
                }
                let mut start: usize = 1;
                for char in &content_octets[1..] {
                    if *char != b' ' {
                        break;
                    }
                    start += 1;
                }
                let mut remaining_slice = &content_octets[start..];
                if remaining_slice.len() == 0 {
                    return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
                }
                if remaining_slice[0] == b'+' || remaining_slice[0] == b'-' {
                    start += 1;
                    remaining_slice = &content_octets[start..];
                }
                if remaining_slice.len() == 0 {
                    return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
                }
                match base10_format {
                    // Why yes, I did purchase a copy of ISO 6093 to figure out what these formats were.
                    1 => { // NR1, which matches /^ *(\+|-)?\d+$/
                        for byte in remaining_slice {
                            if !byte.is_ascii_digit() {
                                return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(remaining_slice.to_owned())));
                            }
                        }
                    },
                    2 => { // NR2, which matches /^ *(\+|-)?(?:\d+(\.|,)\d*)|(?:\d*(\.|,)\d+)$/
                        // Check that digits are encountered.
                        // Check that periods or commas are encountered.
                        // Check that the total number of periods or commas is 1 exactly.
                        let mut digits_encountered = false;
                        let mut period = false;
                        let mut comma = false;
                        for byte in remaining_slice {
                            let b = *byte;
                            if byte.is_ascii_digit() {
                                digits_encountered = true;
                            } else if b == b'.' {
                                if period || comma {
                                    return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(remaining_slice.to_owned())));
                                }
                                period = true;
                            } else if b == b',' {
                                if period || comma {
                                    return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(remaining_slice.to_owned())));
                                }
                                comma = true;
                            }
                        }
                        if !digits_encountered {
                            return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(remaining_slice.to_owned())));
                        }
                    },
                    3 => { // NR3, which matches /^ *(\+|-)?(?:\d+(\.|,)\d*)|(?:\d*(\.|,)\d+)(e|E)(\+|-)?\d+$/
                        let mut digits_encountered = false;
                        let mut period = false;
                        let mut comma = false;
                        let mut index_of_e = 0;
                        for (i, byte) in remaining_slice.iter().enumerate() {
                            let b = *byte;
                            if byte.is_ascii_digit() {
                                digits_encountered = true;
                            } else if b == b'.' {
                                if period || comma {
                                    return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(remaining_slice.to_owned())));
                                }
                                period = true;
                            } else if b == b',' {
                                if period || comma {
                                    return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(remaining_slice.to_owned())));
                                }
                                comma = true;
                            } else if b == b'e' || b == b'E' {
                                index_of_e = i;
                                break;
                            }
                        }
                        if !digits_encountered || index_of_e == 0 {
                            return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(remaining_slice.to_owned())));
                        }
                        remaining_slice = &content_octets[index_of_e+1..];
                        if remaining_slice.len() <= 1 {
                            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
                        }
                        if remaining_slice[0] != b'+' && remaining_slice[0] != b'-' {
                            return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(remaining_slice.to_owned())));
                        }
                        if !remaining_slice[1..].iter().all(|b| b.is_ascii_digit()) {
                            return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_string_malformed(remaining_slice.to_owned())));
                        }
                    },
                    _ => return Err(ASN1Error::new(ASN1ErrorCode::base_10_real_unrecognized_format(base10_format))),
                };
            },
            0b1000_0000 | 0b1100_0000 => { // Binary formatting
                let base = content_octets[0] & 0b0011_0000;
                if base == 0b0011_0000 {
                    return Err(ASN1Error::new(ASN1ErrorCode::bin_real_unrecognized_base));
                }
                let exp_encoding = content_octets[0] & 0b0000_0011;
                let exp_len: usize = match exp_encoding {
                    0b0000_0000 => 1,
                    0b0000_0001 => 2,
                    0b0000_0010 => 3,
                    0b0000_0011 => {
                        // One byte for prefix, one for exp length, at least one for exp length and at least one for the mantissa.
                        if content_octets.len() < 4 {
                            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
                        }
                        content_octets[1] as usize + 1
                    },
                    _ => panic!(),
                };
                if content_octets.len() < exp_len + 2 {
                    return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
                }
            },
            _ => panic!(),
        };
        Ok(())
    }

    // 9604152030Z
    fn validate_utc_time_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        if content_octets.len() > 17 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_big));
        }
        if content_octets.len() < 11 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        let maybe_bad_char = content_octets[0..10].iter().position(|b| b.is_ascii_digit());
        if let Some(bad_char_index) = maybe_bad_char {
            let bad_char = content_octets[bad_char_index];
            return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                bad_char as u32,
                bad_char_index,
            )));
        }
        let s = from_utf8(&content_octets[0..10])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_utf8(None)))?;
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
        let mut start_of_time_zone = 10;
        if content_octets[10].is_ascii_digit() {
            if content_octets.len() < 12 || !content_octets[11].is_ascii_digit() {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
            }
            let second = u8::from_str(&s[10..12])
                .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
            if second > 59 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
            }
            start_of_time_zone = 12;
        }
        if content_octets.len() < start_of_time_zone + 1 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        if content_octets[start_of_time_zone] == b'Z' {
            if content_octets.len() > start_of_time_zone + 1 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            return Ok(());
        }
        if content_octets.len() != start_of_time_zone + 5 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        if content_octets[start_of_time_zone] != b'+'
            && content_octets[start_of_time_zone] != b'-' {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }

        let maybe_bad_char = content_octets[start_of_time_zone + 1..].iter().position(|b| b.is_ascii_digit());
        if let Some(bad_char_index) = maybe_bad_char {
            let bad_char = content_octets[bad_char_index];
            return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                bad_char as u32,
                bad_char_index,
            )));
        }
        let offset_s = unsafe { std::str::from_utf8_unchecked(&content_octets[start_of_time_zone + 1..]) };
        let offset_hour = u8::from_str(&offset_s[6..8])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_hour))?;
        let offset_minute = u8::from_str(&offset_s[8..10])
            .map_err(|_| ASN1Error::new(ASN1ErrorCode::invalid_minute))?;
        if offset_hour > 23 || offset_minute > 59 {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        Ok(())
    }

    // Function produced by ChatGPT-4 before a few modifications by me.
    fn validate_generalized_time_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        let s = match from_utf8(content_octets) {
            Ok(v) => v,
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };

        // Check for basic length
        if s.len() < 15 {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }

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
            s if s.starts_with('.') || s.starts_with(',') => {
                let (fraction, tz) = s[1..].split_at(s.find(|c: char| !c.is_numeric()).unwrap_or(0));

                if fraction.is_empty() {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_fraction_of_seconds));
                }

                match tz {
                    "Z" => Ok(()),
                    tz if tz.starts_with('+') || tz.starts_with('-') => {
                        // Minutes in the offset are optional.
                        if (tz.len() == 5 || tz.len() == 3) && tz[1..].chars().all(|c| c.is_numeric()) {
                            Ok(())
                        } else {
                            Err(ASN1Error::new(ASN1ErrorCode::invalid_fraction_of_seconds))
                        }
                    }
                    _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset)),
                }
            }
            tz if tz.starts_with('+') || tz.starts_with('-') => {
                // Minutes in the offset are optional.
                if (tz.len() == 5 || tz.len() == 3)  && tz[1..].chars().all(|c| c.is_numeric()) {
                    Ok(())
                } else {
                    Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset))
                }
            }
            _ => Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        }
    }

    // Before some tweaking, this was produced by ChatGPT.
    fn validate_duration_value (&self, content_octets: ByteSlice) -> ASN1Result<()> {
        let mut idx = 0;

        // The duration should start with 'P' (ASCII 80)
        if idx >= content_octets.len() || content_octets[idx] != b'P' {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
        idx += 1;

        let mut has_time_component = false;

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
            X690Value::Primitive(v) => self.validate_bit_string_value(&v),
            X690Value::Constructed(subs) => {
                let subs_len = subs.len();
                if subs_len == 0 {
                    return Err(ASN1Error::new(ASN1ErrorCode::x690_bit_string_zero_bytes));
                }
                for sub in subs[0..subs_len - 1].iter() {
                    validate_non_terminal_bit_strings(&sub)?;
                }
                self.validate_bit_string(&subs[subs_len - 1])?;
                Ok(())
            }
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v).unwrap();
                self.validate_bit_string(&el)
            }
        }
    }

    // FIXME: I just realized: you still have to check that it is composed of OCTET STRING if it is constructed.
    #[inline]
    fn validate_octet_string(&self, _: &X690Element) -> ASN1Result<()> {
        Ok(())
    }

    fn validate_object_descriptor(&self, el: &X690Element) -> ASN1Result<()> {
        self.validate_graphic_string(el)
    }

    fn validate_utf8_string(&self, el: &X690Element) -> ASN1Result<()> {
        // The value has to be deconstructed because there is nothing
        // technically restricting the individual segments of a constructed
        // UTF8String from breaking on a multi-byte character.
        let content_octets = deconstruct(el)?;
        self.validate_utf8_string_value(content_octets.as_ref())
    }

    fn validate_numeric_string(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_numeric_string_value(&v),
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    self.validate_numeric_string(sub)?;
                }
                Ok(())
            }
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v).unwrap();
                self.validate_numeric_string(&el)
            }
        }
    }

    fn validate_printable_string(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_printable_string_value(&v),
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    self.validate_printable_string(sub)?;
                }
                Ok(())
            }
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v).unwrap();
                self.validate_printable_string(&el)
            }
        }
    }

    fn validate_t61_string(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_t61_string_value(&v),
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    self.validate_t61_string(sub)?;
                }
                Ok(())
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v).unwrap();
                self.validate_t61_string(&el)
            }
        }
    }

    fn validate_videotex_string(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_videotex_string_value(&v),
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    self.validate_videotex_string(sub)?;
                }
                Ok(())
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v).unwrap();
                self.validate_videotex_string(&el)
            }
        }
    }

    fn validate_ia5_string(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_ia5_string_value(&v),
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    self.validate_ia5_string(sub)?;
                }
                Ok(())
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v).unwrap();
                self.validate_ia5_string(&el)
            }
        }
    }

    fn validate_utc_time(&self, el: &X690Element) -> ASN1Result<()> {
        let content_octets = deconstruct(el)?;
        self.validate_utc_time_value(content_octets.as_ref())
    }

    fn validate_generalized_time(&self, el: &X690Element) -> ASN1Result<()> {
        let content_octets = deconstruct(el)?;
        self.validate_generalized_time_value(content_octets.as_ref())
    }

    fn validate_graphic_string(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_graphic_string_value(&v),
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    self.validate_graphic_string(sub)?;
                }
                Ok(())
            }
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v).unwrap();
                self.validate_graphic_string(&el)
            }
        }
    }

    fn validate_visible_string(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_visible_string_value(&v),
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    self.validate_visible_string(sub)?;
                }
                Ok(())
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v).unwrap();
                self.validate_visible_string(&el)
            }
        }
    }

    fn validate_general_string(&self, el: &X690Element) -> ASN1Result<()> {
        match &el.value {
            X690Value::Primitive(v) => self.validate_general_string_value(&v),
            X690Value::Constructed(subs) => {
                for sub in subs.iter() {
                    self.validate_general_string(sub)?;
                }
                Ok(())
            },
            X690Value::Serialized(v) => {
                let (_, el) = BER.decode_from_slice(&v).unwrap();
                self.validate_general_string(&el)
            }
        }
    }

    fn validate_universal_string(&self, el: &X690Element) -> ASN1Result<()> {
        // The value has to be deconstructed because there is nothing
        // technically restricting the individual segments of a constructed
        // UniversalString from breaking mid-character.
        let content_octets = deconstruct(el)?;
        self.validate_universal_string_value(content_octets.as_ref())
    }

    fn validate_bmp_string(&self, el: &X690Element) -> ASN1Result<()> {
        // The value has to be deconstructed because there is nothing
        // technically restricting the individual segments of a constructed
        // BMPString from breaking mid-character.
        let content_octets = deconstruct(el)?;
        self.validate_bmp_string_value(content_octets.as_ref())
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use super::X690Value;
    use crate::X690_TAG_CLASS_UNIVERSAL;

    #[test]
    fn test_ber_decode_algorithm_identifier() {
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
        let (bytes_read, el) = match BER.decode_from_slice(encoded_data.as_slice()) {
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
                let oid = match BER.decode_object_identifier_value(&oid_bytes) {
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
    fn test_ber_decode_utc_time() {
        let time = "\x17\x11991105223344+0523";
        let value_bytes = Vec::from(time);
        let cst = match BER.decode_from_slice(&value_bytes) {
            Ok((_, el)) => el,
            Err(e) => panic!("{}", e),
        };
        if let X690Value::Primitive(bytes) = &cst.value {
            let utc_time = BER.decode_utc_time_value(&bytes);
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
            assert_eq!(decoded_value.utc_offset.hour, 5);
            assert_eq!(decoded_value.utc_offset.minute, 23);
        } else {
            panic!();
        }
    }

    #[test]
    fn make_sure_u8_from_str_handles_leading_zeros () {
        let num = u8::from_str("05").unwrap();
        assert_eq!(num, 5);
    }
}
