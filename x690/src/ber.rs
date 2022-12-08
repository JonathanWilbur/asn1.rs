use asn1::construction::{ComponentSpec, TagSelector};
use asn1::error::{ASN1Error, ASN1ErrorCode, ASN1Result};
use asn1::types::{
    ASN1Value,
    // INSTANCE_OF,
    BMPString,
    ByteSlice,
    Bytes,
    ContextNegotiation,
    ExternalEncoding,
    ExternalIdentification,
    // EXTERNAL,
    // EMBEDDED_PDV,
    GeneralString,
    GeneralizedTime,
    GraphicString,
    IA5String,
    IdentificationSyntaxes,
    // SET_OF,
    NumericString,
    ObjectDescriptor,
    PresentationContextSwitchingTypeIdentification,
    PrintableString,
    T61String,
    Tag,
    TagClass,
    UTCOffset,
    // GeneralizedTime,
    UTCTime,
    UTF8String,
    UniversalString,
    VideotexString,
    VisibleString,
    ASN1_UNIVERSAL_TAG_NUMBER_BIT_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN,
    ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_DATE,
    ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME,
    ASN1_UNIVERSAL_TAG_NUMBER_DURATION,
    ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV,
    ASN1_UNIVERSAL_TAG_NUMBER_END_OF_CONTENT,
    ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED,
    ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL,
    ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
    ASN1_UNIVERSAL_TAG_NUMBER_NULL,
    ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR,
    ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
    ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI,
    ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_REAL,
    ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID,
    ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI,
    ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE,
    ASN1_UNIVERSAL_TAG_NUMBER_SET,
    ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_TIME,
    ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY,
    ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING,
    ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING,
    BIT_STRING,
    BOOLEAN,
    CHARACTER_STRING,
    DATE,
    DATE_TIME,
    DURATION,
    DURATION_EQUIVALENT,
    EMBEDDED_PDV,
    EXTERNAL,
    INTEGER,
    MAX_IA5_STRING_CHAR_CODE,
    OBJECT_IDENTIFIER,
    OCTET_STRING,
    // CHARACTER_STRING,
    // CharacterString,
    // RELATIVE_OID_IRI,
    OID_IRI,
    OPTIONAL,
    REAL,
    RELATIVE_OID,
    SEQUENCE,
    // SEQUENCE_OF,
    SET,
    TIME,
    TIME_OF_DAY,
};
use asn1::{
    InstanceOf, ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME, ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME,
    ENUMERATED, INSTANCE_OF, NULL,
};
use std::borrow::Borrow;
use std::cmp::min;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use crate::parsing::_parse_sequence;
use crate::{
    ber_cst, deconstruct, x690_write_bit_string_value, x690_write_bmp_string_value,
    x690_write_boolean_value, x690_write_character_string_value, x690_write_date_time_value,
    x690_write_date_value, x690_write_duration_value, x690_write_embedded_pdv_value,
    x690_write_external_value, x690_write_generalized_time_value, x690_write_integer_value,
    x690_write_object_descriptor_value, x690_write_object_identifier_value,
    x690_write_octet_string_value, x690_write_real_value, x690_write_relative_oid_value,
    x690_write_string_value, x690_write_time_of_day_value, x690_write_time_value,
    x690_write_universal_string_value, x690_write_utc_time_value, x690_write_utf8_string_value,
    X690Element, X690Encoding, X690_REAL_BASE10, X690_REAL_BASE_16, X690_REAL_BASE_2,
    X690_REAL_BASE_8, X690_REAL_BASE_MASK, X690_REAL_BINARY_SCALING_MASK,
    X690_REAL_EXPONENT_FORMAT_1_OCTET, X690_REAL_EXPONENT_FORMAT_2_OCTET,
    X690_REAL_EXPONENT_FORMAT_3_OCTET, X690_REAL_EXPONENT_FORMAT_MASK,
    X690_REAL_EXPONENT_FORMAT_VAR_OCTET, X690_REAL_NR1, X690_REAL_NR2, X690_REAL_NR3,
    X690_REAL_SPECIAL, X690_SPECIAL_REAL_MINUS_INFINITY, X690_SPECIAL_REAL_MINUS_ZERO,
    X690_SPECIAL_REAL_NOT_A_NUMBER, X690_SPECIAL_REAL_PLUS_INFINITY,
};
use asn1::bitstring::join_bit_strings;

// BIT STRING is constructed in a such a way that the octets of each subelement
// cannot simply be concatenated. As such, this function deconstructed a
// constructed BIT STRING to obtain a single BIT STRING.
pub fn deconstruct_bit_string(el: &X690Element) -> ASN1Result<BIT_STRING> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => Ok(ber_decode_bit_string_value(bytes.as_slice())?),
        X690Encoding::EXPLICIT(_) => {
            let mut err = ASN1Error::new(ASN1ErrorCode::invalid_construction);
            err.component_name = el.name.clone();
            err.tag = Some(Tag::new(el.tag_class, el.tag_number));
            err.constructed = Some(true);
            err.length = Some(el.len());
            return Err(err);
        }
        X690Encoding::AlreadyEncoded(bytes) => match ber_cst(&bytes) {
            Ok((_, cst)) => {
                return deconstruct_bit_string(&cst);
            }
            Err(e) => return Err(e),
        },
        X690Encoding::Constructed(children) => {
            let mut substituent_bit_strings: Vec<BIT_STRING> = Vec::new();
            for child in children {
                if child.tag_class != el.tag_class || child.tag_number != el.tag_number {
                    let mut err =
                        ASN1Error::new(ASN1ErrorCode::string_constructed_with_invalid_tagging);
                    err.component_name = el.name.clone();
                    err.tag = Some(Tag::new(el.tag_class, el.tag_number));
                    err.length = Some(el.len());
                    err.constructed = Some(true);
                    return Err(err);
                }
                match deconstruct_bit_string(&child) {
                    Ok(deconstructed_child) => {
                        substituent_bit_strings.push(deconstructed_child);
                    }
                    Err(e) => return Err(e),
                }
            }
            return Ok(join_bit_strings(&substituent_bit_strings.as_slice()));
        }
    }
}

pub fn ber_decode_boolean_value(value_bytes: ByteSlice) -> ASN1Result<BOOLEAN> {
    if value_bytes.len() != 1 {
        let err = ASN1Error::new(ASN1ErrorCode::x690_boolean_not_one_byte);
        return Err(err);
    }
    Ok(value_bytes[0] > 0)
}

pub fn ber_decode_integer_value(value_bytes: ByteSlice) -> ASN1Result<INTEGER> {
    let len = value_bytes.len();
    match len {
        1 => Ok(value_bytes[0] as i8 as INTEGER),
        2 => Ok(i16::from_be_bytes([value_bytes[0], value_bytes[1]]) as i64),
        3 => Ok(i32::from_be_bytes([
            if value_bytes[0] & 0b1000_0000 > 0 {
                0xFF
            } else {
                0x00
            },
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
        ]) as i64),
        4 => Ok(i32::from_be_bytes([
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
            value_bytes[3],
        ]) as i64),
        5..=8 => {
            let mut buf: [u8; 8] = [0; 8];
            buf[8 - len..].copy_from_slice(value_bytes);
            Ok(i64::from_be_bytes(buf))
        }
        _ => Err(ASN1Error::new(ASN1ErrorCode::value_too_big)),
    }
}

pub fn ber_decode_bit_string_value(value_bytes: ByteSlice) -> ASN1Result<BIT_STRING> {
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
    Ok(BIT_STRING {
        bytes: Vec::from(&value_bytes[1..]),
        trailing_bits,
    })
}

pub fn ber_decode_octet_string_value(value_bytes: ByteSlice) -> ASN1Result<OCTET_STRING> {
    Ok(Vec::from(value_bytes))
}

pub fn ber_decode_object_identifier_value(value_bytes: ByteSlice) -> ASN1Result<OBJECT_IDENTIFIER> {
    let len = value_bytes.len();
    if len < 1 {
        return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
    }
    let arc1 = (value_bytes[0] / 40) as u32;
    let arc2 = (value_bytes[0] % 40) as u32;
    let mut nodes: Vec<u32> = vec![arc1, arc2];
    let mut current_node: u32 = 0;
    for byte in value_bytes[1..].iter() {
        current_node <<= 7;
        current_node += (byte & 0b0111_1111) as u32;
        if (byte & 0b1000_0000) == 0 {
            nodes.push(current_node);
            current_node = 0;
        }
    }
    if current_node > 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    Ok(nodes)
}

pub fn ber_decode_relative_oid_value(value_bytes: ByteSlice) -> ASN1Result<RELATIVE_OID> {
    let len = value_bytes.len();
    let mut nodes: Vec<u32> = Vec::with_capacity(len);
    let mut current_node: u32 = 0;
    for byte in value_bytes[1..].iter() {
        current_node <<= 7;
        current_node += (byte & 0b0111_1111) as u32;
        if (byte & 0b1000_0000) == 0 {
            nodes.push(current_node);
            current_node = 0;
        }
    }
    if current_node > 0 {
        // Truncated.
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    Ok(nodes)
}

pub fn ber_read_var_length_u64(bytes: ByteSlice) -> u64 {
    match bytes.len() {
        0 => 0,
        1 => bytes[0] as u8 as u64,
        2 => u16::from_be_bytes([bytes[0], bytes[1]]) as u64,
        3 => u32::from_be_bytes([0x00, bytes[0], bytes[1], bytes[2]]) as u64,
        4 => u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64,
        5 => u64::from_be_bytes([
            0x00, 0x00, 0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4],
        ]),
        6 => u64::from_be_bytes([
            0x00, 0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5],
        ]),
        7 => u64::from_be_bytes([
            0x00, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
        ]),
        8 => u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]),
        _ => 0,
    }
}

pub fn ber_decode_real_value(value_bytes: ByteSlice) -> ASN1Result<REAL> {
    if value_bytes.len() == 0 {
        return Ok(0.000000);
    }
    match value_bytes[0] & 0b1100_0000 {
        X690_REAL_SPECIAL => match value_bytes[0] {
            X690_SPECIAL_REAL_PLUS_INFINITY => return Ok(f64::INFINITY),
            X690_SPECIAL_REAL_MINUS_INFINITY => return Ok(f64::NEG_INFINITY),
            X690_SPECIAL_REAL_NOT_A_NUMBER => return Ok(f64::NAN),
            X690_SPECIAL_REAL_MINUS_ZERO => return Ok(-0.000000),
            _ => return Err(ASN1Error::new(ASN1ErrorCode::urecognized_real_format)),
        },
        X690_REAL_BASE10 => {
            let str_ = match String::from_utf8(value_bytes[1..].to_vec()) {
                Ok(v) => String::from(v.trim_start()),
                _ => {
                    return Err(ASN1Error::new(
                        ASN1ErrorCode::base_10_real_string_decoding_error,
                    ))
                }
            };
            let f64_value = match f64::from_str(&str_) {
                Ok(f) => f,
                _ => {
                    return Err(ASN1Error::new(
                        ASN1ErrorCode::base_10_real_string_malformed(String::from(&str_[0..32])),
                    ))
                }
            };
            // FIXME: Wrong formatting.
            let format = value_bytes[0] & 0b0011_1111;
            return match format {
                X690_REAL_NR1 => Ok(f64_value),
                X690_REAL_NR2 => Ok(f64_value),
                X690_REAL_NR3 => Ok(f64_value),
                _ => {
                    return Err(ASN1Error::new(
                        ASN1ErrorCode::base_10_real_unrecognized_format(format),
                    ))
                }
            };
        }
        _ => {
            // Binary encoding
            let negative = (value_bytes[0] & 0b0100_0000) > 0;
            let base_byte = value_bytes[0] & X690_REAL_BASE_MASK;
            let base: u8 = match base_byte {
                X690_REAL_BASE_2 => 2,
                X690_REAL_BASE_8 => 8,
                X690_REAL_BASE_16 => 16,
                _ => {
                    return Err(ASN1Error::new(
                        ASN1ErrorCode::base_10_real_unrecognized_base(base_byte),
                    ))
                }
            };
            let scale: u8 = (value_bytes[0] & X690_REAL_BINARY_SCALING_MASK)
                .overflowing_shr(2)
                .0;
            let mantissa: u64;
            let exponent: i32;
            match value_bytes[0] & X690_REAL_EXPONENT_FORMAT_MASK {
                X690_REAL_EXPONENT_FORMAT_1_OCTET => {
                    if value_bytes.len() < 3 {
                        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                    }
                    exponent = value_bytes[1] as i8 as i32;
                    mantissa = ber_read_var_length_u64(&value_bytes[2..])
                }
                X690_REAL_EXPONENT_FORMAT_2_OCTET => {
                    if value_bytes.len() < 4 {
                        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                    }
                    if value_bytes.len() > 4 + 6 {
                        // Mantissa too big..
                        return Err(ASN1Error::new(ASN1ErrorCode::field_too_big));
                    }
                    exponent = i32::from_be_bytes([0, 0, value_bytes[1], value_bytes[2]]);
                    mantissa = ber_read_var_length_u64(&value_bytes[3..])
                }
                X690_REAL_EXPONENT_FORMAT_3_OCTET => {
                    if value_bytes.len() < 5 {
                        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                    }
                    if value_bytes.len() > 5 + 6 {
                        // Mantissa too big.
                        return Err(ASN1Error::new(ASN1ErrorCode::field_too_big));
                    }
                    exponent =
                        i32::from_be_bytes([0, value_bytes[1], value_bytes[2], value_bytes[3]]);
                    mantissa = ber_read_var_length_u64(&value_bytes[4..])
                }
                X690_REAL_EXPONENT_FORMAT_VAR_OCTET => {
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
                    // FIXME: How do you know the mantissa was not too big?
                    if exponent_len == 1 {
                        exponent = value_bytes[2] as i8 as i32;
                        mantissa = ber_read_var_length_u64(&value_bytes[3..]);
                    } else {
                        // The exponent must have length 2.
                        exponent = i32::from_be_bytes([0, 0, value_bytes[2], value_bytes[3]]);
                        mantissa = ber_read_var_length_u64(&value_bytes[4..]);
                    }
                }
                _ => return Err(ASN1Error::new(ASN1ErrorCode::nonsense)), // This should never happen.
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

pub fn ber_decode_numeric_string_value(value_bytes: ByteSlice) -> ASN1Result<NumericString> {
    for (i, byte) in value_bytes.iter().enumerate() {
        if !byte.is_ascii_digit() && *byte != 0x20 {
            return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                *byte as u32,
                i,
            )));
        }
    }
    unsafe { Ok(String::from_utf8_unchecked(value_bytes.to_vec())) }
}

pub fn ber_decode_printable_string_value(value_bytes: ByteSlice) -> ASN1Result<PrintableString> {
    for (i, byte) in value_bytes.iter().enumerate() {
        let b = *byte as char;
        if byte.is_ascii_alphanumeric()
            || (b >= '\x27' && b < '0' && b != '*') // '()+,-./ BUT NOT *
            || b == ' '
            || b == ':'
            || b == '='
            || b == '?'
        {
            continue;
        }
        return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
            *byte as u32,
            i,
        )));
    }
    unsafe { Ok(String::from_utf8_unchecked(value_bytes.to_vec())) }
}

pub fn ber_decode_ia5_string_value(value_bytes: ByteSlice) -> ASN1Result<IA5String> {
    for (i, byte) in value_bytes.iter().enumerate() {
        if *byte > MAX_IA5_STRING_CHAR_CODE {
            return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                *byte as u32,
                i,
            )));
        }
    }
    unsafe { Ok(String::from_utf8_unchecked(value_bytes.to_vec())) }
}

pub fn ber_decode_utc_time_value(value_bytes: ByteSlice) -> ASN1Result<UTCTime> {
    let len = value_bytes.len();
    if len < 10 {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    if len > 17 {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    for byte in value_bytes[0..10].iter() {
        if !byte.is_ascii_digit() {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
    }
    let s = match String::from_utf8(value_bytes.to_vec()) {
        Ok(r) => r,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
    };
    let mut ret = UTCTime::new();
    ret.year = match u8::from_str(&s[0..2]) {
        Ok(u) => u,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_year)),
    };
    ret.month = match u8::from_str(&s[2..4]) {
        Ok(u) => {
            if u == 0 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
            }
            if u > 12 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
            }
            u
        }
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_month)),
    };
    ret.day = match u8::from_str(&s[4..6]) {
        Ok(u) => {
            if u == 0 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
            }
            if u > 31 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
            }
            u
        }
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_day)),
    };
    ret.hour = match u8::from_str(&s[6..8]) {
        Ok(u) => {
            if u > 23 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
            }
            u
        }
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour)),
    };
    ret.minute = match u8::from_str(&s[8..10]) {
        Ok(u) => {
            if u > 59 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
            }
            u
        }
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute)),
    };
    if (len > 12) && value_bytes[10].is_ascii_digit() {
        // Seconds component is present.
        if !value_bytes[11].is_ascii_digit() {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
        }
        ret.second = match u8::from_str(&s[10..12]) {
            Ok(u) => {
                if u > 59 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
                }
                Some(u)
            }
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_second)),
        };
    }
    if value_bytes[len - 1] as char != 'Z' {
        if (value_bytes[len - 5] as char != '+') && (value_bytes[len - 5] as char != '-') {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
        for byte in value_bytes[len - 4..len].iter() {
            if !byte.is_ascii_digit() {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
        }
        let offset_hour = match i8::from_str(&s[len - 4..len - 2]) {
            Ok(u) => {
                if u > 12 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
                }
                u
            }
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset)),
        };
        let offset_minute = match u8::from_str(&s[len - 2..len]) {
            Ok(u) => {
                if u > 59 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
                }
                u
            }
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset)),
        };
        ret.utc_offset = Some(UTCOffset {
            hour: if value_bytes[len - 5] as char == '-' {
                -1 * offset_hour
            } else {
                offset_hour
            },
            minute: offset_minute,
        });
    }
    Ok(ret)
}

pub fn ber_decode_generalized_time_value(value_bytes: ByteSlice) -> ASN1Result<GeneralizedTime> {
    let len = value_bytes.len();
    if len < 10 {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    // There is technically no limit on how big a GeneralizedTime can be, but
    // we have to set a reasonable limit here.
    if len > 32 {
        return Err(ASN1Error::new(ASN1ErrorCode::value_too_big));
    }
    for byte in value_bytes[0..10].iter() {
        if !byte.is_ascii_digit() {
            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
        }
    }
    let s = match String::from_utf8(value_bytes.to_vec()) {
        Ok(r) => r,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
    };
    let mut date = DATE::new();
    let mut ret = GeneralizedTime::new();
    date.year = match u16::from_str(&s[0..4]) {
        Ok(u) => u,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_year)),
    };
    date.month = match u8::from_str(&s[4..6]) {
        Ok(u) => {
            if u == 0 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
            }
            if u > 12 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
            }
            u
        }
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_month)),
    };
    date.day = match u8::from_str(&s[6..8]) {
        Ok(u) => {
            if u == 0 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
            }
            if u > 31 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
            }
            u
        }
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_day)),
    };
    ret.hour = match u8::from_str(&s[8..10]) {
        Ok(u) => {
            if u > 23 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
            }
            u
        }
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour)),
    };
    if (len > 12) && value_bytes[10].is_ascii_digit() {
        if !value_bytes[11].is_ascii_digit() {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
        }
        ret.minute = match u8::from_str(&s[10..12]) {
            Ok(u) => {
                if u > 59 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
                }
                Some(u)
            }
            Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute)),
        };
    }

    if let Some(_) = ret.minute {
        // Normal "if"s cannot be combined with "if let"s.
        if (len > 14) && value_bytes[12].is_ascii_digit() {
            // Seconds component is present.
            if !value_bytes[13].is_ascii_digit() {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
            }
            ret.second = match u8::from_str(&s[12..14]) {
                Ok(u) => {
                    if u > 59 {
                        return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
                    }
                    Some(u)
                }
                Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_second)),
            };
        }
    }

    if let Some(_) = ret.second {
        if (len >= 16) && ((value_bytes[14] as char == '.') || (value_bytes[14] as char == ',')) {
            // let frac_byte = value_bytes[15];
            let mut i = 15;
            while i < len && value_bytes[i].is_ascii_digit() {
                i += 1;
            }
            let end = min(i, 19); // We can only tolerate four digits of precision.
                                  // FIXME: Pad with zeroes or whatever to make units consistent.
            ret.fraction = match u16::from_str(&s[12..end]) {
                Ok(u) => {
                    if u > 9999 {
                        return Err(ASN1Error::new(ASN1ErrorCode::field_too_big));
                    }
                    Some(u)
                }
                Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_fraction_of_seconds)),
            };
        }
    }

    if value_bytes[len - 1] as char == 'Z' {
        // ret.utc = true; // This is the default.
        return Ok(ret); // UTCTime
    }

    if (value_bytes[len - 5] as char != '+') && (value_bytes[len - 5] as char != '-') {
        ret.utc = false;
        return Ok(ret); // Local Time
    }

    // For the rest of this function, we are parsing the UTC Offset.
    for byte in value_bytes[len - 4..len].iter() {
        if !byte.is_ascii_digit() {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
        }
    }
    let offset_hour = match i8::from_str(&s[len - 4..len - 2]) {
        Ok(u) => {
            if u > 12 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            u
        }
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset)),
    };
    let offset_minute = match u8::from_str(&s[len - 2..len]) {
        Ok(u) => {
            if u > 59 {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset));
            }
            u
        }
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::invalid_time_offset)),
    };
    ret.utc_offset = Some(UTCOffset {
        hour: if value_bytes[len - 5] as char == '-' {
            -1 * offset_hour
        } else {
            offset_hour
        },
        minute: offset_minute,
    });
    Ok(ret)
}

pub fn ber_decode_graphic_string_value(value_bytes: ByteSlice) -> ASN1Result<GraphicString> {
    for (i, byte) in value_bytes.iter().enumerate() {
        if !byte.is_ascii_graphic() && (*byte as char != ' ') {
            return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                *byte as u32,
                i,
            )));
        }
    }
    unsafe { Ok(String::from_utf8_unchecked(value_bytes.to_vec())) }
}

pub fn ber_decode_visible_string_value(value_bytes: ByteSlice) -> ASN1Result<VisibleString> {
    for (i, byte) in value_bytes.iter().enumerate() {
        if !byte.is_ascii() || *byte == 0x7F {
            return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                *byte as u32,
                i,
            )));
        }
    }
    unsafe { Ok(String::from_utf8_unchecked(value_bytes.to_vec())) }
}

pub fn ber_decode_general_string_value(value_bytes: ByteSlice) -> ASN1Result<GeneralString> {
    for (i, byte) in value_bytes.iter().enumerate() {
        if !byte.is_ascii() {
            return Err(ASN1Error::new(ASN1ErrorCode::prohibited_character(
                *byte as u32,
                i,
            )));
        }
    }
    unsafe { Ok(String::from_utf8_unchecked(value_bytes.to_vec())) }
}

pub fn ber_decode_universal_string_value(value_bytes: ByteSlice) -> ASN1Result<UniversalString> {
    if (value_bytes.len() % 4) != 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    let char_length = value_bytes.len() / 4;
    let mut ret = String::with_capacity(char_length);
    for i in 0..char_length {
        let code_point = u32::from_be_bytes([
            value_bytes[(i * 4) + 0],
            value_bytes[(i * 4) + 1],
            value_bytes[(i * 4) + 2],
            value_bytes[(i * 4) + 3],
        ]);
        match char::from_u32(code_point) {
            Some(c) => ret.push(c),
            None => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
    }
    return Ok(ret);
}

pub fn ber_decode_bmp_string_value(value_bytes: ByteSlice) -> ASN1Result<BMPString> {
    if (value_bytes.len() % 2) != 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    let char_length = value_bytes.len() / 4;
    let mut ret = String::with_capacity(char_length);
    for i in 0..char_length {
        let code_point = u16::from_be_bytes([value_bytes[(i * 4) + 0], value_bytes[(i * 4) + 1]]);
        match char::from_u32(code_point as u32) {
            Some(c) => ret.push(c),
            None => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
        };
    }
    return Ok(ret);
}

pub fn ber_decode_date_value(value_bytes: ByteSlice) -> ASN1Result<DATE> {
    if value_bytes.len() != 10 {
        // "YYYY-MM-DD".len()
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    let str_ = match String::from_utf8(value_bytes.to_vec()) {
        Ok(s) => s,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
    };
    let year = match u16::from_str(&str_[0..4]) {
        Ok(x) => x,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
    };
    let month = match u8::from_str(&str_[5..7]) {
        Ok(x) => x,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
    };
    let day = match u8::from_str(&str_[8..]) {
        Ok(x) => x,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
    };
    if month > 12 || month == 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_month));
    }
    if day > 31 || day == 0 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_day));
    }
    return Ok(DATE { year, month, day });
}

pub fn ber_decode_time_of_day_value(value_bytes: ByteSlice) -> ASN1Result<TIME_OF_DAY> {
    if value_bytes.len() != 8 {
        // "HH:MM:SS".len()
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    let str_ = match String::from_utf8(value_bytes.to_vec()) {
        Ok(s) => s,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
    };
    let hour = match u8::from_str(&str_[0..2]) {
        Ok(x) => x,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
    };
    let minute = match u8::from_str(&str_[3..5]) {
        Ok(x) => x,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
    };
    let second = match u8::from_str(&str_[6..]) {
        Ok(x) => x,
        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
    };
    if hour > 23 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_hour));
    }
    if minute > 59 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_minute));
    }
    if second > 59 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_second));
    }
    return Ok(TIME_OF_DAY {
        hour,
        minute,
        second,
    });
}

pub fn ber_decode_date_time_value(value_bytes: ByteSlice) -> ASN1Result<DATE_TIME> {
    if value_bytes.len() != 19 {
        // "YYYY-MM-DDTHH:MM:SS".len()
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    let date = ber_decode_date_value(&value_bytes[0..10])?;
    let time = ber_decode_time_of_day_value(&value_bytes[11..19])?;
    return Ok(DATE_TIME { date, time });
}

const DURATION_COMPONENT_YEARS: u8 = 0b0000_0001;
const DURATION_COMPONENT_MONTHS: u8 = 0b0000_0010;
const DURATION_COMPONENT_WEEKS: u8 = 0b0000_0100;
const DURATION_COMPONENT_DAYS: u8 = 0b0000_1000;
const DURATION_COMPONENT_HOURS: u8 = 0b0001_0000;
const DURATION_COMPONENT_MINUTES: u8 = 0b0010_0000;
const DURATION_COMPONENT_SECONDS: u8 = 0b0100_0000;

pub fn ber_decode_duration_value(value_bytes: ByteSlice) -> ASN1Result<DURATION> {
    if value_bytes.len() < 3 {
        // The smallest duration string, e.g. P1Y
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    if value_bytes[0] as char != 'P' {
        return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
    }
    let mut ret = DURATION_EQUIVALENT::new();
    let mut start_of_last_digit = 0;
    let mut processing_time_components: bool = false;
    let mut index_of_period = 0; // 0 means NULL in this case.
    let mut encountered: u8 = 0;
    for i in 1..value_bytes.len() {
        let char_ = value_bytes[i];
        if !char_.is_ascii_digit() {
            if start_of_last_digit == i {
                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
            }
            match char_ as char {
                '.' => {
                    index_of_period = i;
                }
                'Y' | 'W' | 'M' | 'D' | 'H' | 'S' => {
                    if index_of_period > 0 {
                        if i != (value_bytes.len() - 1) {
                            // Extra data after the last permitted unit.
                            return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                        }
                    }
                    let end_index = if index_of_period > 0 {
                        index_of_period
                    } else {
                        i
                    };
                    let component_str = match String::from_utf8(
                        value_bytes[start_of_last_digit..end_index].to_vec(),
                    ) {
                        Ok(s) => s,
                        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
                    };
                    let component_value = match u32::from_str(&component_str) {
                        Ok(v) => v,
                        Err(_) => return Err(ASN1Error::new(ASN1ErrorCode::malformed_value)),
                    };
                    start_of_last_digit = i + 1;
                    match char_ as char {
                        'Y' => {
                            if processing_time_components {
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                            if encountered > 0 {
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                            encountered |= DURATION_COMPONENT_YEARS;
                            ret.years = component_value;
                        }
                        'M' => {
                            if processing_time_components {
                                if encountered > DURATION_COMPONENT_HOURS {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                encountered |= DURATION_COMPONENT_MINUTES;
                                ret.minutes = component_value;
                            } else {
                                if encountered > DURATION_COMPONENT_YEARS {
                                    return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                                }
                                encountered |= DURATION_COMPONENT_MONTHS;
                                ret.months = component_value;
                            }
                        }
                        'W' => {
                            if processing_time_components {
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                            if encountered > DURATION_COMPONENT_MONTHS {
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                            encountered |= DURATION_COMPONENT_WEEKS;
                            ret.weeks = component_value;
                        }
                        'D' => {
                            if processing_time_components {
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                            if encountered > DURATION_COMPONENT_WEEKS {
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                            encountered |= DURATION_COMPONENT_DAYS;
                            ret.days = component_value;
                        }
                        'H' => {
                            if !processing_time_components {
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                            if encountered > DURATION_COMPONENT_DAYS {
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                            encountered |= DURATION_COMPONENT_HOURS;
                            ret.hours = component_value;
                        }
                        'S' => {
                            if !processing_time_components {
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                            if encountered > DURATION_COMPONENT_MINUTES {
                                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
                            }
                            encountered |= DURATION_COMPONENT_SECONDS;
                            ret.seconds = component_value;
                        }
                        _ => panic!("Impossible code reached."),
                    };
                }
                'T' => {
                    processing_time_components = true;
                }
                _ => (),
            }
        }
    }
    Ok(ret)
}

pub fn ber_decode_boolean(el: &X690Element) -> ASN1Result<BOOLEAN> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_boolean_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_integer(el: &X690Element) -> ASN1Result<INTEGER> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_integer_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_enumerated(el: &X690Element) -> ASN1Result<ENUMERATED> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_integer_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_bit_string(el: &X690Element) -> ASN1Result<BIT_STRING> {
    deconstruct_bit_string(&el)
}

pub fn ber_decode_octet_string(el: &X690Element) -> ASN1Result<OCTET_STRING> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => Ok(bytes.clone()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_null(el: &X690Element) -> ASN1Result<()> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => {
            if bytes.len() != 0 {
                return Err(ASN1Error::new(ASN1ErrorCode::malformed_value));
            }
            Ok(())
        }
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_object_identifier(el: &X690Element) -> ASN1Result<OBJECT_IDENTIFIER> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_object_identifier_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
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
const _RCTL1_FOR_EXTERNAL: &[ComponentSpec; 4] = &[
    ComponentSpec::new(
        "direct-reference",
        true,
        TagSelector::tag((
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
        )),
        None,
        None,
    ),
    ComponentSpec::new(
        "indirect-reference",
        true,
        TagSelector::tag((TagClass::UNIVERSAL, ASN1_UNIVERSAL_TAG_NUMBER_INTEGER)),
        None,
        None,
    ),
    ComponentSpec::new(
        "data-value-descriptor",
        true,
        TagSelector::tag((
            TagClass::UNIVERSAL,
            ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR,
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
const _EAL_FOR_EXTERNAL: &[ComponentSpec; 0] = &[];
const _RCTL2_FOR_EXTERNAL: &[ComponentSpec; 0] = &[];

pub fn ber_decode_external(el: &X690Element) -> ASN1Result<EXTERNAL> {
    let elements = match el.value.borrow() {
        X690Encoding::Constructed(children) => children,
        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    };
    if elements.len() > 4 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
    }
    if elements.len() < 1 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
    }
    let el_refs = elements.iter().collect::<Vec<&X690Element>>();
    let (components, unrecognized) = _parse_sequence(
        el_refs.as_slice(),
        _RCTL1_FOR_EXTERNAL.as_slice(),
        _EAL_FOR_EXTERNAL.as_slice(),
        _RCTL2_FOR_EXTERNAL.as_slice(),
    )
    .unwrap();
    if unrecognized.len() > 0 {
        return Err(ASN1Error::new(
            ASN1ErrorCode::unrecognized_components_in_inextensible_type,
        ));
    }
    let dir_ref: OPTIONAL<OBJECT_IDENTIFIER> = match components.get("direct-reference") {
        Some(c) => match ber_decode_object_identifier(c) {
            Ok(v) => Some(v),
            Err(e) => return Err(e),
        },
        None => None,
    };
    let indir_ref: OPTIONAL<INTEGER> = match components.get("indirect-reference") {
        Some(c) => match ber_decode_integer(c) {
            Ok(v) => Some(v),
            Err(e) => return Err(e),
        },
        None => None,
    };
    let dvd: OPTIONAL<ObjectDescriptor> = match components.get("data-value-descriptor") {
        Some(c) => match ber_decode_object_descriptor(c) {
            Ok(v) => Some(v),
            Err(e) => return Err(e),
        },
        None => None,
    };
    let encoding: ExternalEncoding = match components.get("encoding") {
        Some(c) => {
            if c.tag_class != TagClass::CONTEXT {
                let mut err =
                    ASN1Error::new(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice);
                err.component_name = Some(String::from("encoding"));
                err.tag = Some(Tag::new(c.tag_class, c.tag_number));
                err.length = Some(c.len());
                err.constructed = Some(c.is_constructed());
                return Err(err);
            }
            match c.tag_number {
                0 => {
                    if let X690Encoding::EXPLICIT(inner) = c.value.borrow() {
                        let v = ber_decode_any(inner.deref())?;
                        ExternalEncoding::single_ASN1_type(Arc::new(v))
                    } else {
                        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                    }
                }
                1 => {
                    let v = ber_decode_octet_string(c)?;
                    ExternalEncoding::octet_aligned(v)
                }
                2 => {
                    let v = ber_decode_bit_string(c)?;
                    ExternalEncoding::arbitrary(v)
                }
                _ => {
                    return Err(ASN1Error::new(
                        ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                    ))
                }
            }
        }
        None => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    };
    let identification: ExternalIdentification;
    if dir_ref.is_some() && indir_ref.is_some() {
        identification = ExternalIdentification::context_negotiation(ContextNegotiation {
            transfer_syntax: dir_ref.unwrap(),
            presentation_context_id: indir_ref.unwrap(),
        });
    } else if dir_ref.is_some() {
        identification = ExternalIdentification::syntax(dir_ref.unwrap());
    } else if indir_ref.is_some() {
        identification = ExternalIdentification::presentation_context_id(indir_ref.unwrap());
    } else {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
    }
    Ok(EXTERNAL {
        identification,
        data_value_descriptor: dvd,
        data_value: encoding,
    })
}

pub fn ber_decode_instance_of(el: &X690Element) -> ASN1Result<InstanceOf> {
    let elements = match el.value.borrow() {
        X690Encoding::Constructed(children) => children,
        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    };
    if elements.len() != 2 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
    }
    if elements[0].tag_class != TagClass::UNIVERSAL
        || elements[0].tag_number != ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER
        || elements[1].tag_class != TagClass::CONTEXT
        || elements[1].tag_number != 0
    {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
    }
    let type_id: OBJECT_IDENTIFIER = ber_decode_object_identifier(&elements[0])?;
    let value = ber_decode_any(&elements[1].inner()?)?;

    Ok(InstanceOf {
        type_id,
        value: Arc::new(value),
    })
}

pub fn ber_decode_real(el: &X690Element) -> ASN1Result<REAL> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => match ber_decode_real_value(bytes.as_slice()) {
            Ok(decoded) => Ok(decoded),
            Err(e) => Err(e),
        },
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

/*
    An `EmbeddedPDV` is a constructed data type, defined in
    the [International Telecommunications Union](https://www.itu.int)'s
    [X.680](https://www.itu.int/rec/T-REC-X.680/en).
    The specification defines `EmbeddedPDV` as:

    `EmbeddedPDV ::= [UNIVERSAL 11] IMPLICIT SEQUENCE {
        identification CHOICE {
            syntaxes SEQUENCE {
                abstract OBJECT IDENTIFIER,
                transfer OBJECT IDENTIFIER },
            syntax OBJECT IDENTIFIER,
            presentation-context-id INTEGER,
            context-negotiation SEQUENCE {
                presentation-context-id INTEGER,
                transfer-syntax OBJECT IDENTIFIER },
            transfer-syntax OBJECT IDENTIFIER,
            fixed NULL },
        data-value-descriptor ObjectDescriptor OPTIONAL,
        data-value OCTET STRING }
    (WITH COMPONENTS { ... , data-value-descriptor ABSENT })`
*/
pub fn ber_decode_presentation_context_switching_type_id(
    el: &X690Element,
) -> ASN1Result<PresentationContextSwitchingTypeIdentification> {
    if el.tag_class != TagClass::CONTEXT {
        let mut err =
            ASN1Error::new(ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice);
        err.component_name = Some(String::from("identification"));
        err.tag = Some(Tag::new(el.tag_class, el.tag_number));
        err.length = Some(el.len());
        err.constructed = Some(el.is_constructed());
        return Err(err);
    }
    match el.tag_number {
        0 => {
            // syntaxes
            if let X690Encoding::Constructed(children) = el.value.borrow() {
                if children.len() != 2 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                }
                let r#abstract = ber_decode_object_identifier(&children[0])?;
                let transfer = ber_decode_object_identifier(&children[1])?;
                Ok(PresentationContextSwitchingTypeIdentification::syntaxes(
                    IdentificationSyntaxes {
                        r#abstract,
                        transfer,
                    },
                ))
            } else {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
            }
        }
        1 => {
            // syntax
            let v = ber_decode_object_identifier(el)?;
            Ok(PresentationContextSwitchingTypeIdentification::syntax(v))
        }
        2 => {
            // presentation-context-id
            let v = ber_decode_integer(el)?;
            Ok(PresentationContextSwitchingTypeIdentification::presentation_context_id(v))
        }
        3 => {
            // context-negotiation
            if let X690Encoding::Constructed(children) = el.value.borrow() {
                if children.len() != 2 {
                    return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
                }
                let presentation_context_id = ber_decode_integer(&children[0])?;
                let transfer_syntax = ber_decode_object_identifier(&children[1])?;
                Ok(
                    PresentationContextSwitchingTypeIdentification::context_negotiation(
                        ContextNegotiation {
                            presentation_context_id,
                            transfer_syntax,
                        },
                    ),
                )
            } else {
                return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
            }
        }
        4 => {
            // transfer-syntax
            let v = ber_decode_object_identifier(el)?;
            Ok(PresentationContextSwitchingTypeIdentification::transfer_syntax(v))
        }
        5 => {
            // fixed
            ber_decode_null(el)?;
            Ok(PresentationContextSwitchingTypeIdentification::fixed)
        }
        _ => Err(ASN1Error::new(
            ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
        )),
    }
}

pub fn ber_decode_embedded_pdv(el: &X690Element) -> ASN1Result<EMBEDDED_PDV> {
    let elements = match el.value.borrow() {
        X690Encoding::Constructed(children) => children,
        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    };
    if elements.len() != 2 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
    }
    let identification: PresentationContextSwitchingTypeIdentification;
    if let X690Encoding::EXPLICIT(inner) = elements[0].value.borrow() {
        identification = ber_decode_presentation_context_switching_type_id(&inner)?;
    } else {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
    }
    let data_value: OCTET_STRING = ber_decode_octet_string(&elements[1])?;
    Ok(EMBEDDED_PDV {
        identification,
        data_value,
    })
}

pub fn ber_decode_character_string(el: &X690Element) -> ASN1Result<CHARACTER_STRING> {
    let elements = match el.value.borrow() {
        X690Encoding::Constructed(children) => children,
        _ => return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    };
    if elements.len() != 2 {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
    }
    let identification: PresentationContextSwitchingTypeIdentification;
    if let X690Encoding::EXPLICIT(inner) = elements[0].value.borrow() {
        identification = ber_decode_presentation_context_switching_type_id(&inner)?;
    } else {
        return Err(ASN1Error::new(ASN1ErrorCode::invalid_construction));
    }
    let string_value: OCTET_STRING = ber_decode_octet_string(&elements[1])?;
    Ok(CHARACTER_STRING {
        identification,
        string_value,
    })
}

pub fn ber_decode_relative_oid(el: &X690Element) -> ASN1Result<RELATIVE_OID> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => match ber_decode_relative_oid_value(bytes.as_slice()) {
            Ok(decoded) => Ok(decoded),
            Err(e) => Err(e),
        },
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_sequence(el: &X690Element) -> ASN1Result<SEQUENCE> {
    match el.value.borrow() {
        X690Encoding::Constructed(components) => {
            let mut values: Vec<ASN1Value> = Vec::new();
            for component in components {
                match ber_decode_any(&component) {
                    Ok(v) => values.push(v),
                    Err(e) => return Err(e),
                }
            }
            return Ok(values);
        }
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_set(el: &X690Element) -> ASN1Result<SET> {
    match el.value.borrow() {
        X690Encoding::Constructed(components) => {
            let mut values: Vec<ASN1Value> = Vec::new();
            for component in components {
                match ber_decode_any(&component) {
                    Ok(v) => values.push(v),
                    Err(e) => return Err(e),
                }
            }
            return Ok(values);
        }
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_object_descriptor(el: &X690Element) -> ASN1Result<ObjectDescriptor> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_graphic_string_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_utf8_string(el: &X690Element) -> ASN1Result<UTF8String> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => match String::from_utf8(bytes.clone()) {
            Ok(x) => Ok(x),
            Err(_) => Err(ASN1Error::new(ASN1ErrorCode::invalid_utf8)),
        },
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_numeric_string(el: &X690Element) -> ASN1Result<NumericString> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_numeric_string_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_printable_string(el: &X690Element) -> ASN1Result<PrintableString> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_printable_string_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_t61_string(el: &X690Element) -> ASN1Result<T61String> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => Ok(bytes.clone()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_videotex_string(el: &X690Element) -> ASN1Result<VideotexString> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => Ok(bytes.clone()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_ia5_string(el: &X690Element) -> ASN1Result<IA5String> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_ia5_string_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_utc_time(el: &X690Element) -> ASN1Result<UTCTime> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_utc_time_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_generalized_time(el: &X690Element) -> ASN1Result<GeneralizedTime> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_generalized_time_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_graphic_string(el: &X690Element) -> ASN1Result<GraphicString> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_graphic_string_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_visible_string(el: &X690Element) -> ASN1Result<VisibleString> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_visible_string_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_general_string(el: &X690Element) -> ASN1Result<GeneralString> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_general_string_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_universal_string(el: &X690Element) -> ASN1Result<UniversalString> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_universal_string_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_bmp_string(el: &X690Element) -> ASN1Result<BMPString> {
    let deconstruction = deconstruct(el)?;
    match deconstruction.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_bmp_string_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
    }
}

pub fn ber_decode_date(el: &X690Element) -> ASN1Result<DATE> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_date_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_time_of_day(el: &X690Element) -> ASN1Result<TIME_OF_DAY> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_time_of_day_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_date_time(el: &X690Element) -> ASN1Result<DATE_TIME> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_date_time_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_duration(el: &X690Element) -> ASN1Result<DURATION> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => ber_decode_duration_value(bytes.as_slice()),
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_oid_iri(el: &X690Element) -> ASN1Result<OID_IRI> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => match String::from_utf8(bytes.clone()) {
            Ok(x) => Ok(x),
            Err(_) => Err(ASN1Error::new(ASN1ErrorCode::invalid_utf8)),
        },
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_relative_oid_iri(el: &X690Element) -> ASN1Result<OID_IRI> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => match String::from_utf8(bytes.clone()) {
            Ok(x) => Ok(x),
            Err(_) => Err(ASN1Error::new(ASN1ErrorCode::invalid_utf8)),
        },
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_time(el: &X690Element) -> ASN1Result<TIME> {
    match el.value.borrow() {
        X690Encoding::IMPLICIT(bytes) => match String::from_utf8(bytes.clone()) {
            Ok(x) => Ok(x),
            Err(_) => Err(ASN1Error::new(ASN1ErrorCode::invalid_utf8)),
        },
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_decode_any(el: &X690Element) -> ASN1Result<ASN1Value> {
    if let X690Encoding::AlreadyEncoded(encoding) = el.value.borrow() {
        match ber_cst(&encoding) {
            Ok((bytes_read, encoded_el)) => {
                if bytes_read != encoding.len() {
                    return Err(ASN1Error::new(ASN1ErrorCode::trailing_data));
                }
                return ber_decode_any(&encoded_el);
            }
            Err(e) => return Err(e),
        }
    }

    if el.tag_class != TagClass::UNIVERSAL {
        if let X690Encoding::EXPLICIT(inner) = el.value.borrow() {
            return ber_decode_any(&inner);
        } else if let X690Encoding::IMPLICIT(bytes) = el.value.borrow() {
            return Ok(ASN1Value::UnknownBytes(Arc::new(bytes.clone())));
        } else if let X690Encoding::Constructed(children) = el.value.borrow() {
            let mut values: Vec<ASN1Value> = Vec::with_capacity(children.len());
            for child in children {
                values.push(ber_decode_any(&child)?);
            }
            return Ok(ASN1Value::SequenceValue(values));
        } else {
            return Err(ASN1Error::new(ASN1ErrorCode::nonsense));
        }
    }

    match el.tag_number {
        ASN1_UNIVERSAL_TAG_NUMBER_END_OF_CONTENT => Err(ASN1Error::new(ASN1ErrorCode::nonsense)),
        ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN => match ber_decode_boolean(el) {
            Ok(v) => Ok(ASN1Value::BooleanValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_INTEGER => match ber_decode_integer(el) {
            Ok(v) => Ok(ASN1Value::IntegerValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_BIT_STRING => match ber_decode_bit_string(el) {
            Ok(v) => Ok(ASN1Value::BitStringValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING => match ber_decode_octet_string(el) {
            Ok(v) => Ok(ASN1Value::OctetStringValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_NULL => Ok(ASN1Value::NullValue),
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER => match ber_decode_object_identifier(el) {
            Ok(v) => Ok(ASN1Value::ObjectIdentifierValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR => match ber_decode_object_descriptor(el) {
            Ok(v) => Ok(ASN1Value::ObjectDescriptor(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL => match ber_decode_external(el) {
            Ok(v) => Ok(ASN1Value::ExternalValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_REAL => match ber_decode_real(el) {
            Ok(v) => Ok(ASN1Value::RealValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED => match ber_decode_integer(el) {
            Ok(v) => Ok(ASN1Value::EnumeratedValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV => match ber_decode_embedded_pdv(el) {
            Ok(v) => Ok(ASN1Value::EmbeddedPDVValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING => match ber_decode_utf8_string(el) {
            Ok(v) => Ok(ASN1Value::UTF8String(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID => match ber_decode_relative_oid(el) {
            Ok(v) => Ok(ASN1Value::RelativeOIDValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_TIME => match ber_decode_time(el) {
            Ok(v) => Ok(ASN1Value::TimeValue(v)),
            Err(e) => Err(e),
        },
        // ASN1_UNIVERSAL_TAG_NUMBER_RESERVED_15 => ()
        ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE => match ber_decode_sequence(el) {
            Ok(v) => Ok(ASN1Value::SequenceValue(v)),
            Err(e) => Err(e),
        },
        // ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE_OF => ()
        ASN1_UNIVERSAL_TAG_NUMBER_SET => match ber_decode_set(el) {
            Ok(v) => Ok(ASN1Value::SetValue(v)),
            Err(e) => Err(e),
        },
        // ASN1_UNIVERSAL_TAG_NUMBER_SET_OF => ()
        ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING => match ber_decode_numeric_string(el) {
            Ok(v) => Ok(ASN1Value::NumericString(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING => match ber_decode_printable_string(el) {
            Ok(v) => Ok(ASN1Value::PrintableString(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING => match ber_decode_t61_string(el) {
            Ok(v) => Ok(ASN1Value::T61String(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING => match ber_decode_videotex_string(el) {
            Ok(v) => Ok(ASN1Value::VideotexString(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING => match ber_decode_ia5_string(el) {
            Ok(v) => Ok(ASN1Value::IA5String(v)),
            Err(e) => Err(e),
        },
        // ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME => {
        //     match el.value {

        //     }
        // },
        // ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME => {
        //     match el.value {

        //     }
        // },
        ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING => match ber_decode_graphic_string(el) {
            Ok(v) => Ok(ASN1Value::GraphicString(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING => match ber_decode_visible_string(el) {
            Ok(v) => Ok(ASN1Value::VisibleString(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING => match ber_decode_general_string(el) {
            Ok(v) => Ok(ASN1Value::GeneralString(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING => match ber_decode_universal_string(el) {
            Ok(v) => Ok(ASN1Value::UniversalString(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING => match ber_decode_character_string(el) {
            Ok(v) => Ok(ASN1Value::UnrestrictedCharacterStringValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING => match ber_decode_bmp_string(el) {
            Ok(v) => Ok(ASN1Value::BMPString(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_DATE => match ber_decode_date(el) {
            Ok(v) => Ok(ASN1Value::DATE(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY => match ber_decode_time_of_day(el) {
            Ok(v) => Ok(ASN1Value::TIME_OF_DAY(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME => match ber_decode_date_time(el) {
            Ok(v) => Ok(ASN1Value::DATE_TIME(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_DURATION => match ber_decode_duration(el) {
            Ok(v) => Ok(ASN1Value::DURATION(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI => match ber_decode_oid_iri(el) {
            Ok(v) => Ok(ASN1Value::IRIValue(v)),
            Err(e) => Err(e),
        },
        ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI => match ber_decode_relative_oid_iri(el) {
            Ok(v) => Ok(ASN1Value::RelativeIRIValue(v)),
            Err(e) => Err(e),
        },
        _ => Err(ASN1Error::new(ASN1ErrorCode::invalid_construction)),
    }
}

pub fn ber_encode_boolean(value: &BOOLEAN) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_boolean_value(&mut out, value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_BOOLEAN,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_integer(value: &INTEGER) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_integer_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_INTEGER,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_enumerated(value: &ENUMERATED) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_integer_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_ENUMERATED,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_bit_string(value: &BIT_STRING) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_bit_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_BIT_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_octet_string(value: &OCTET_STRING) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_octet_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_OCTET_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_null(value: &NULL) -> ASN1Result<X690Element> {
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_NULL,
        Arc::new(X690Encoding::IMPLICIT(vec![])),
    ))
}

pub fn ber_encode_object_identifier(value: &OBJECT_IDENTIFIER) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_object_identifier_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_external(value: &EXTERNAL) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_external_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_instance_of(value: &INSTANCE_OF) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    let external = EXTERNAL {
        identification: ExternalIdentification::syntax(value.type_id.clone()),
        data_value_descriptor: None,
        data_value: ExternalEncoding::single_ASN1_type(value.value.clone()),
    };
    x690_write_external_value(&mut out, &external)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_EXTERNAL,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_real(value: &REAL) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_real_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_REAL,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_embedded_pdv(value: &EMBEDDED_PDV) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_embedded_pdv_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_EMBEDDED_PDV,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_character_string(value: &CHARACTER_STRING) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_character_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_CHARACTER_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_relative_oid(value: &RELATIVE_OID) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_relative_oid_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

// TODO: ?
// pub fn ber_encode_sequence (value: &SEQUENCE) -> ASN1Result<X690Element> {
//     let mut out: Bytes = Vec::new();
//     x690_write_(&mut out, &value)?;
//     Ok(X690Element::new(
//         TagClass::UNIVERSAL,
//         ASN1_UNIVERSAL_TAG_NUMBER_ASDF,
//         Arc::new(X690Encoding::IMPLICIT(out)),
//     ))
// }

// TODO: ?
// pub fn ber_encode_set (value: &SET) -> ASN1Result<X690Element> {
//     let mut out: Bytes = Vec::new();
//     x690_write_asdf(&mut out, &value)?;
//     Ok(X690Element::new(
//         TagClass::UNIVERSAL,
//         ASN1_UNIVERSAL_TAG_NUMBER_ASDF,
//         Arc::new(X690Encoding::IMPLICIT(out)),
//     ))
// }

pub fn ber_encode_object_descriptor(value: &ObjectDescriptor) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_object_descriptor_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_DESCRIPTOR,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_utf8_string(value: &UTF8String) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_utf8_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_UTF8_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_numeric_string(value: &NumericString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_NUMERIC_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_printable_string(value: &PrintableString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_PRINTABLE_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_t61_string(value: &T61String) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_octet_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_T61_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_videotex_string(value: &VideotexString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_octet_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_VIDEOTEX_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_ia5_string(value: &IA5String) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_IA5_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_utc_time(value: &UTCTime) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_utc_time_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_UTC_TIME,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_generalized_time(value: &GeneralizedTime) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_generalized_time_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_GENERALIZED_TIME,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_graphic_string(value: &GraphicString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_GRAPHIC_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_visible_string(value: &VisibleString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_VISIBLE_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_general_string(value: &GeneralString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_GENERAL_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_universal_string(value: &UniversalString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_universal_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_UNIVERSAL_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_bmp_string(value: &BMPString) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_bmp_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_BMP_STRING,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_date(value: &DATE) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_date_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_DATE,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_time_of_day(value: &TIME_OF_DAY) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_time_of_day_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_TIME_OF_DAY,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_date_time(value: &DATE_TIME) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_date_time_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_DATE_TIME,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_duration(value: &DURATION) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_duration_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_DURATION,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_oid_iri(value: &OID_IRI) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_OID_IRI,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_relative_oid_iri(value: &OID_IRI) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_string_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_RELATIVE_OID_IRI,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

pub fn ber_encode_time(value: &TIME) -> ASN1Result<X690Element> {
    let mut out: Bytes = Vec::new();
    x690_write_time_value(&mut out, &value)?;
    Ok(X690Element::new(
        TagClass::UNIVERSAL,
        ASN1_UNIVERSAL_TAG_NUMBER_TIME,
        Arc::new(X690Encoding::IMPLICIT(out)),
    ))
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::{ber_cst, X690Encoding};
    use crate::X690_TAG_CLASS_UNIVERSAL;

    // pub struct AlgorithmIdentifier {
    //     pub algorithm: OBJECT_IDENTIFIER,
    //     pub parameters: OPTIONAL<ASN1Value>,
    // }

    #[test]
    fn test_ber_decode_algorithm_identifier() {
        let encoded_data: Vec<u8> = vec![
            X690_TAG_CLASS_UNIVERSAL
            | 0b0010_0000 // Constructed
            | ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE as u8,
            0x07,
            0x06,
            0x03,
            0x55,
            0x04,
            0x03,
            0x05,
            0x00,
        ];
        let (bytes_read, el) = match ber_cst(encoded_data.as_slice()) {
            Err(_) => panic!("asdf"),
            Ok(result) => result,
        };
        assert_eq!(bytes_read, 9);
        assert_eq!(el.tag_class, TagClass::UNIVERSAL);
        assert_eq!(el.tag_number, ASN1_UNIVERSAL_TAG_NUMBER_SEQUENCE);
        if let X690Encoding::Constructed(children) = el.value.borrow() {
            assert_eq!(children.len(), 2);
            assert_eq!(children[0].tag_class, TagClass::UNIVERSAL);
            assert_eq!(
                children[0].tag_number,
                ASN1_UNIVERSAL_TAG_NUMBER_OBJECT_IDENTIFIER
            );
            assert_eq!(children[1].tag_class, TagClass::UNIVERSAL);
            assert_eq!(children[1].tag_number, ASN1_UNIVERSAL_TAG_NUMBER_NULL);
            if let X690Encoding::IMPLICIT(oid_bytes) = children[0].value.borrow() {
                let oid = match ber_decode_object_identifier_value(&oid_bytes) {
                    Err(_) => panic!("woriyjh"),
                    Ok(result) => result,
                };
                assert!(oid.starts_with(&[2, 5, 4, 3]));
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
        let cst = match ber_cst(value_bytes.as_slice()) {
            Ok((_, el)) => el,
            Err(e) => panic!("{}", e),
        };
        if let X690Encoding::IMPLICIT(bytes) = cst.value.borrow() {
            let utc_time = ber_decode_utc_time_value(&bytes);
            let decoded_value = match utc_time {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            };
            assert_eq!(decoded_value.year, 99);
            assert_eq!(decoded_value.month, 11);
            assert_eq!(decoded_value.day, 5);
            assert_eq!(decoded_value.hour, 22);
            assert_eq!(decoded_value.minute, 33);
            assert_eq!(decoded_value.second, Some(44));
            if let Some(offset) = decoded_value.utc_offset {
                assert_eq!(offset.hour, 5);
                assert_eq!(offset.minute, 23);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }
}
